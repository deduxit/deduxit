// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Terms and sequents written as text, by the print forms a calculus
//! gives its connectives.

use super::calculus::{Associativity, Calculus, Fixity};
use super::derivation::{Derivation, NodeId};
use super::terms::{Term, TermId, Terms};
use crate::linear::sequents::notation::Notation;
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
/// unless [`limit`](Shown::limit) says otherwise, and `…` after them where
/// the text goes on.
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
    /// A sequent given as each zone's members.
    Zones(&'a [Vec<TermId>]),
}

impl<'a> Shown<'a> {
    /// The most characters written by default.
    pub const LIMIT: usize = 4096;

    /// Returns the same text cut after `limit` characters, `…` then
    /// marking the cut.
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

    /// Writes the term at `id` through `out` as `spelling` spells it,
    /// with a stack of its own.
    fn write_term<W: Write + ?Sized>(
        &self,
        out: &mut Bounded<'_, W>,
        spelling: Spelling<'_>,
        id: TermId,
    ) -> fmt::Result {
        /// What is left to write.
        enum Item<'a> {
            /// A term, and whether it is bracketed.
            Term(TermId, bool),
            /// A piece of text.
            Text(&'a str),
            /// A connective's symbol.
            Symbol(&'a str),
        }
        let mut stack = vec![Item::Term(id, false)];
        while let Some(item) = stack.pop() {
            if out.full() {
                // In a target's notation a cut text still closes what it
                // opened, or the output would not compile.
                if out.whole {
                    let open = stack
                        .iter()
                        .filter(|i| matches!(i, Item::Text(")")))
                        .count();
                    let closing = if matches!(item, Item::Text(")")) {
                        open + 1
                    } else {
                        open
                    };
                    for _ in 0..closing {
                        out.out.write_str(")")?;
                    }
                }
                break;
            }
            let (id, bracketed) = match item {
                Item::Text(text) => {
                    out.write_str(text)?;
                    continue;
                }
                Item::Symbol(symbol) => {
                    spelling.symbol(out, symbol)?;
                    continue;
                }
                Item::Term(id, bracketed) => (id, bracketed),
            };
            // A bracket is closed only where it was opened: a cut text
            // closes those still waiting.
            if bracketed {
                out.write_str("(")?;
                if !out.full() {
                    stack.push(Item::Text(")"));
                }
            }
            let (connective, arguments) = match self.terms.term(id) {
                Term::Atom { name, .. } => {
                    spelling.atom(out, name)?;
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
                _ if arguments.is_empty() => spelling.symbol(out, form.symbol)?,
                Fixity::Prefix => {
                    spelling.symbol(out, form.symbol)?;
                    stack.push(argument(0));
                    if spaced(form.symbol.chars().next_back()) {
                        stack.push(Item::Text(" "));
                    }
                }
                Fixity::Postfix => {
                    stack.push(Item::Symbol(form.symbol));
                    if spaced(form.symbol.chars().next()) {
                        stack.push(Item::Text(" "));
                    }
                    stack.push(argument(0));
                }
                Fixity::Infix => {
                    stack.push(argument(1));
                    stack.push(Item::Text(" "));
                    stack.push(Item::Symbol(form.symbol));
                    stack.push(Item::Text(" "));
                    stack.push(argument(0));
                }
                Fixity::Function => {
                    spelling.symbol(out, form.symbol)?;
                    out.write_str(spelling.call())?;
                    if !out.full() {
                        stack.push(Item::Text(")"));
                    }
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
        self.write_in(f, Spelling::PLAIN)
    }
}

impl<'a> Shown<'a> {
    /// Writes the term or the sequent into `out` as `spelling` spells
    /// it, at most the bound's characters, then a mark of the cut where
    /// there is one.
    fn write_in<W: Write + ?Sized>(&self, out: &mut W, spelling: Spelling<'_>) -> fmt::Result {
        let mut out = Bounded {
            out,
            left: self.limit,
            cut: false,
            whole: spelling.notation.is_some(),
        };
        match self.what {
            What::Term(id) => self.write_term(&mut out, spelling, id)?,
            What::Sequent(derivation, node) => {
                let node = derivation.node(node);
                self.write_zones(&mut out, spelling, |index| node.zone(index))?;
            }
            What::Zones(zones) => {
                self.write_zones(&mut out, spelling, |index| &zones[index][..])?;
            }
        }
        if out.cut {
            out.out.write_str(spelling.cut())?;
        }
        Ok(())
    }

    /// Writes a sequent whose zone at each index `zone` gives.
    fn write_zones<W: Write + ?Sized>(
        &self,
        out: &mut Bounded<'_, W>,
        spelling: Spelling<'_>,
        zone: impl Fn(usize) -> &'a [TermId],
    ) -> fmt::Result {
        let mut written = false;
        for (index, form) in self.calculus.form().sequent.iter().enumerate() {
            let members = zone(index);
            let zone = form;
            {
                if let Some(mark) = &zone.mark {
                    if written {
                        out.write_str(" ")?;
                    }
                    if spelling.aligned && mark == "⊢" {
                        spelling.align(out)?;
                    }
                    spelling.symbol(out, mark)?;
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
                    let (open, close) = spelling.marks();
                    let opened = !out.full();
                    out.write_str(open)?;
                    self.write_term(out, spelling, member)?;
                    // A member's mark is closed whatever the bound cut.
                    if opened {
                        out.out.write_str(close)?;
                    }
                    written = true;
                }
            }
        }
        Ok(())
    }
}

/// A writer that takes at most a number of characters and then nothing.
struct Bounded<'a, W: Write + ?Sized> {
    /// Where the text goes.
    out: &'a mut W,
    /// The characters it still takes.
    left: usize,
    /// Whether some text was not taken.
    cut: bool,
    /// Whether each piece is taken whole or not at all, so that no
    /// command of a target's notation is cut in two.
    whole: bool,
}

impl<W: Write + ?Sized> Bounded<'_, W> {
    /// Returns whether some text was not taken, after which nothing is.
    const fn full(&self) -> bool {
        self.cut
    }
}

impl<W: Write + ?Sized> Write for Bounded<'_, W> {
    /// Writes as much of `text` as the bound takes.
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if self.cut {
            return Ok(());
        }
        if self.whole {
            if self.left == 0 {
                self.cut = true;
                return Ok(());
            }
            self.left = self.left.saturating_sub(text.chars().count());
            return self.out.write_str(text);
        }
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

    /// Returns a sequent of `terms`, each zone's members, as text, as
    /// [`show_sequent`](Self::show_sequent) writes a node's.
    ///
    /// # Panics
    ///
    /// For a sequent with fewer zones than the calculus, when the text is
    /// written.
    pub const fn show_zones<'a>(&'a self, terms: &'a Terms, zones: &'a [Vec<TermId>]) -> Shown<'a> {
        Shown {
            calculus: self,
            terms,
            what: What::Zones(zones),
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

/// How a term is spelt: as the calculus writes it, or in an export's
/// notation, where the connectives that notation knows take its symbols
/// and every other symbol is set upright.
#[derive(Clone, Copy)]
pub(crate) struct Spelling<'n> {
    /// The export's notation, or `None` for the calculus's own text.
    notation: Option<&'n Notation>,
    /// Whether each member of a sequent is put between the marks the SVG
    /// layout groups a formula by.
    marks: bool,
    /// Whether the turnstile is preceded by the notation's alignment
    /// mark.
    aligned: bool,
}

impl<'n> Spelling<'n> {
    /// The calculus's own text.
    const PLAIN: Self = Self {
        notation: None,
        marks: false,
        aligned: false,
    };

    /// Returns the spelling of `notation`, with `marks` around members
    /// and the turnstile `aligned` as [`Notation::sequent`] has them.
    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    pub(crate) const fn of(notation: &'n Notation, aligned: bool, marks: bool) -> Self {
        Self {
            notation: Some(notation),
            marks,
            aligned,
        }
    }

    /// Writes an atom's name, in a notation as one piece, so that a bound
    /// never cuts it in two.
    fn atom<W: Write + ?Sized>(self, out: &mut W, name: &str) -> fmt::Result {
        match self.notation {
            None => out.write_str(name),
            Some(notation) => {
                let mut piece = String::new();
                notation.atom.write(&mut piece, name)?;
                out.write_str(&piece)
            }
        }
    }

    /// Writes a connective's symbol or a zone's mark: the notation's own
    /// symbol where it has one, else the text set upright.
    fn symbol<W: Write + ?Sized>(self, out: &mut W, symbol: &str) -> fmt::Result {
        let Some(notation) = self.notation else {
            return out.write_str(symbol);
        };
        let known = match symbol {
            "⊗" => notation.tensor,
            "⅋" => notation.par,
            "&" => notation.with,
            "⊕" => notation.plus,
            "⊸" => notation.lollipop,
            "\\" => notation.under,
            "/" => notation.over,
            "!" => notation.bang,
            "?" => notation.quest,
            "1" => notation.one,
            "0" => notation.zero,
            "⊤" => notation.top,
            "⊥" => notation.bot,
            "¬" => notation.ordinary.not,
            "∧" => notation.ordinary.and,
            "∨" => notation.ordinary.or,
            "→" => notation.ordinary.implies,
            "↔" => notation.ordinary.iff,
            "⊢" => notation.turnstile,
            _ => {
                let mut piece = String::new();
                notation.atom.upright(&mut piece, symbol)?;
                return out.write_str(&piece);
            }
        };
        out.write_str(known)
    }

    /// Writes the alignment mark before a turnstile.
    fn align<W: Write + ?Sized>(self, out: &mut W) -> fmt::Result {
        self.notation.map_or(Ok(()), |n| out.write_str(n.align))
    }

    /// Returns what opens a function's arguments.
    fn call(self) -> &'static str {
        self.notation.map_or("(", |n| n.atom.call())
    }

    /// Returns what marks text cut by the bound.
    fn cut(self) -> &'static str {
        self.notation.map_or("…", |n| n.atom.cut())
    }

    /// Returns what goes before and after each member of a sequent.
    const fn marks(self) -> (&'static str, &'static str) {
        if self.marks {
            ("\u{2}", "\u{3}")
        } else {
            ("", "")
        }
    }
}

impl Calculus {
    /// Writes the sequent of `node` into `out` as `spelling` spells it,
    /// at most [`Shown::LIMIT`] characters of it.
    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    pub(crate) fn write_sequent_in(
        &self,
        derivation: &Derivation,
        node: NodeId,
        spelling: Spelling<'_>,
        out: &mut String,
    ) {
        // Writing into a `String` does not fail.
        let _ = self.show_sequent(derivation, node).write_in(out, spelling);
    }
}
