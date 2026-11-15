// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Terms and sequents written as text, by the print forms a calculus
//! gives its connectives.

use super::calculus::{Associativity, Calculus, Fixity};
use super::derivation::{Derivation, NodeId};
use super::terms::{Term, TermId, Terms};
use std::fmt::{self, Display, Formatter, Write};

/// How a connective is written: the text, its place among the
/// arguments, and how tightly it binds.
#[derive(Clone, Copy)]
struct Form<'a> {
    /// What is written for it.
    symbol: &'a str,
    /// Where it stands among its arguments; a function's form for an
    /// arity its fixity does not fit.
    fixity: Fixity,
    /// How tightly it binds: higher binds tighter.
    precedence: u32,
    /// How a chain of it reads without brackets.
    associativity: Associativity,
}

/// A term or a sequent to be written, at most [`Shown::LIMIT`] characters
/// unless [`limit`](Shown::limit) says otherwise; past the bound it ends
/// with `…`.
///
/// A term is a graph with shared parts, so its text may be far longer
/// than its arena. The bound keeps the text, and the time spent on it,
/// within what a reader can take in.
#[derive(Clone, Copy, Debug)]
pub struct Shown<'a> {
    /// The calculus whose print forms are used.
    calculus: &'a Calculus,
    /// The terms.
    terms: &'a Terms,
    /// What is written.
    what: What<'a>,
    /// The most characters written.
    limit: usize,
}

/// What a [`Shown`] writes.
#[derive(Clone, Copy, Debug)]
enum What<'a> {
    /// One term.
    Term(TermId),
    /// The sequent of a node.
    Sequent(&'a Derivation, NodeId),
}

impl<'a> Shown<'a> {
    /// The most characters written by default.
    pub const LIMIT: usize = 4096;

    /// Returns the same text bounded by `limit` characters.
    #[must_use]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Returns how the calculus writes the connective named `name`; a
    /// name it does not know is written as a function.
    fn form(&self, name: &'a str, arity: usize) -> Form<'a> {
        let declared = self
            .calculus
            .connective_ids
            .get(name)
            .map(|&index| &self.calculus.form().connectives[index as usize]);
        let symbol = declared.and_then(|c| c.symbol.as_deref()).unwrap_or(name);
        let fixity = match (declared.and_then(|c| c.fixity), arity) {
            (Some(Fixity::Prefix) | None, 1) => Fixity::Prefix,
            (Some(Fixity::Postfix), 1) => Fixity::Postfix,
            (Some(Fixity::Infix) | None, 2) => Fixity::Infix,
            _ => Fixity::Function,
        };
        let precedence = declared.and_then(|c| c.precedence).unwrap_or(match fixity {
            Fixity::Infix => 0,
            Fixity::Prefix | Fixity::Postfix | Fixity::Function => u32::MAX,
        });
        let associativity = declared
            .and_then(|c| c.associativity)
            .unwrap_or(Associativity::None);
        Form {
            symbol,
            fixity,
            precedence,
            associativity,
        }
    }

    /// Returns how the term at `id` is written, if it is a connective
    /// applied to arguments; `None` for an atom and a constant, which need
    /// no brackets.
    fn compound(&self, id: TermId) -> Option<Form<'a>> {
        match self.terms.term(id) {
            Term::Apply {
                connective,
                arguments,
            } if !arguments.is_empty() => Some(self.form(connective, arguments.len())),
            Term::Apply { .. } | Term::Atom { .. } => None,
        }
    }

    /// Returns whether the term at `id`, an argument at `place` of a
    /// connective written `parent`, needs brackets.
    fn bracketed(&self, id: TermId, parent: Form<'_>, place: usize) -> bool {
        let Some(child) = self.compound(id) else {
            return false;
        };
        match (parent.fixity, child.fixity) {
            (Fixity::Function, _) | (_, Fixity::Function) => false,
            (Fixity::Infix, Fixity::Infix) => {
                let side = match parent.associativity {
                    Associativity::Left => place == 0,
                    Associativity::Right => place == 1,
                    Associativity::None => false,
                };
                child.precedence < parent.precedence
                    || (child.precedence == parent.precedence && !side)
            }
            (Fixity::Infix, Fixity::Prefix | Fixity::Postfix) => {
                child.precedence < parent.precedence
            }
            // A prefix operand of a prefix connective, or a postfix of a
            // postfix, reads one way only.
            (Fixity::Prefix, Fixity::Prefix) | (Fixity::Postfix, Fixity::Postfix) => false,
            (Fixity::Prefix | Fixity::Postfix, Fixity::Infix) => true,
            (Fixity::Prefix, Fixity::Postfix) | (Fixity::Postfix, Fixity::Prefix) => {
                child.precedence <= parent.precedence
            }
        }
    }

    /// Writes the term at `id` through `out`, with a stack of its own.
    fn write_term(&self, out: &mut Bounded<'_, '_>, id: TermId) -> fmt::Result {
        /// What is left to write.
        enum Item<'a> {
            /// A term, and whether it is bracketed.
            Term(TermId, bool),
            /// A piece of text.
            Text(&'a str),
        }
        let mut stack = vec![Item::Term(id, false)];
        while let Some(item) = stack.pop() {
            if out.full() {
                break;
            }
            let (id, bracketed) = match item {
                Item::Text(text) => {
                    out.write_str(text)?;
                    continue;
                }
                Item::Term(id, bracketed) => (id, bracketed),
            };
            if bracketed {
                out.write_str("(")?;
                stack.push(Item::Text(")"));
            }
            let (connective, arguments) = match self.terms.term(id) {
                Term::Atom { name, .. } => {
                    out.write_str(name)?;
                    continue;
                }
                Term::Apply {
                    connective,
                    arguments,
                } => (connective, arguments),
            };
            let form = self.form(connective, arguments.len());
            let argument = |place: usize| {
                let id = arguments[place];
                Item::Term(id, self.bracketed(id, form, place))
            };
            // A word as a symbol keeps a space from its operand.
            let spaced = |c: Option<char>| c.is_some_and(char::is_alphanumeric);
            match form.fixity {
                _ if arguments.is_empty() => out.write_str(form.symbol)?,
                Fixity::Prefix => {
                    out.write_str(form.symbol)?;
                    stack.push(argument(0));
                    if spaced(form.symbol.chars().next_back()) {
                        stack.push(Item::Text(" "));
                    }
                }
                Fixity::Postfix => {
                    stack.push(Item::Text(form.symbol));
                    if spaced(form.symbol.chars().next()) {
                        stack.push(Item::Text(" "));
                    }
                    stack.push(argument(0));
                }
                Fixity::Infix => {
                    stack.push(argument(1));
                    stack.push(Item::Text(" "));
                    stack.push(Item::Text(form.symbol));
                    stack.push(Item::Text(" "));
                    stack.push(argument(0));
                }
                Fixity::Function => {
                    out.write_str(form.symbol)?;
                    out.write_str("(")?;
                    stack.push(Item::Text(")"));
                    for place in (0..arguments.len()).rev() {
                        stack.push(Item::Term(arguments[place], false));
                        if place > 0 {
                            stack.push(Item::Text(", "));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Display for Shown<'_> {
    /// Writes the term or the sequent, at most the bound's characters.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = Bounded {
            out: f,
            left: self.limit,
            cut: false,
        };
        match self.what {
            What::Term(id) => self.write_term(&mut out, id)?,
            What::Sequent(derivation, node) => {
                let node = derivation.node(node);
                let mut written = false;
                for (index, zone) in self.calculus.form().sequent.iter().enumerate() {
                    let members = node.zone(index);
                    if let Some(mark) = &zone.mark {
                        if written {
                            out.write_str(" ")?;
                        }
                        out.write_str(mark)?;
                        if !members.is_empty() {
                            out.write_str(" ")?;
                        }
                        written = true;
                    } else if written && !members.is_empty() {
                        out.write_str("; ")?;
                    }
                    for (place, &member) in members.iter().enumerate() {
                        if place > 0 {
                            out.write_str(", ")?;
                        }
                        self.write_term(&mut out, member)?;
                        written = true;
                    }
                }
            }
        }
        if out.cut {
            out.out.write_str("…")?;
        }
        Ok(())
    }
}

/// A writer that takes at most a number of characters and then nothing.
struct Bounded<'a, 'b> {
    /// Where the text goes.
    out: &'a mut Formatter<'b>,
    /// The characters it still takes.
    left: usize,
    /// Whether some text was not taken.
    cut: bool,
}

impl Bounded<'_, '_> {
    /// Returns whether some text was not taken, after which nothing is.
    const fn full(&self) -> bool {
        self.cut
    }
}

impl Write for Bounded<'_, '_> {
    /// Writes as much of `text` as the bound takes.
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = match text.char_indices().nth(self.left) {
            Some((at, _)) => {
                self.cut = true;
                at
            }
            None => text.len(),
        };
        // At most `left` characters lie before `end`.
        self.left -= text[..end].chars().count();
        self.out.write_str(&text[..end])
    }
}

impl Calculus {
    /// Returns the term at `id` of `terms` as text, written by the print
    /// forms of the calculus's connectives: a constant as its symbol, a
    /// unary connective before its operand, a binary one between its
    /// two, others as `f(a, b, c)`, with brackets where the precedences
    /// ask for them. A connective the calculus does not declare is
    /// written as a function.
    ///
    /// # Panics
    ///
    /// For an id that is no term of `terms`, when the text is written.
    pub const fn show<'a>(&'a self, terms: &'a Terms, id: TermId) -> Shown<'a> {
        Shown {
            calculus: self,
            terms,
            what: What::Term(id),
            limit: Shown::LIMIT,
        }
    }

    /// Returns the sequent of `node` as text: its zones in order, each
    /// after its mark (`⊢`), members separated by `,` and two zones without
    /// a mark between them by `;`.
    ///
    /// # Panics
    ///
    /// For a node of another derivation, or a derivation with other zones
    /// than the calculus's, when the text is written.
    pub const fn show_sequent<'a>(&'a self, derivation: &'a Derivation, node: NodeId) -> Shown<'a> {
        Shown {
            calculus: self,
            terms: derivation.terms(),
            what: What::Sequent(derivation, node),
            limit: Shown::LIMIT,
        }
    }
}
