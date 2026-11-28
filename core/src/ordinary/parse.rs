// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A precedence parser for ordinary formulas and sequents, in two
//! dialects: this crate's syntax (documented on [`Sequent`]) and TPTP's
//! `fof` and `cnf`, the syntax of the ILTP library.
//! It keeps its own stack, so a formula of any depth is read without
//! recursion.

use super::{Formulas, Node, NodeId, Sequent};
use crate::Error;
use crate::arena::parse::{AFTER_FORMULA, End, IN_BRACKETS, Reads, Text, count_within, sides};
use crate::errors::ParseError;
use crate::first_order::FoTermId;
use crate::limits::Limits;
use crate::linear::lltp::{Clauses, Status, clauses};

/// A binary connective as it is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Binary {
    /// `<->`, `↔` or TPTP's `<=>`
    Iff,
    /// TPTP's `<~>`, the negated equivalence.
    Xor,
    /// `->`, `→` or TPTP's `=>`
    Implies,
    /// TPTP's `<=`, the implication from right to left.
    Reverse,
    /// `\/`, `∨` or TPTP's `|`
    Or,
    /// TPTP's `~|`
    Nor,
    /// `/\`, `∧` or TPTP's `&`
    And,
    /// TPTP's `~&`
    Nand,
}

impl Binary {
    /// Returns how tightly the connective binds.
    const fn strength(self) -> u8 {
        match self {
            Self::Iff | Self::Xor => 1,
            Self::Implies | Self::Reverse => 2,
            Self::Or | Self::Nor => 3,
            Self::And | Self::Nand => 4,
        }
    }

    /// Returns whether an operand between this connective and `next` is
    /// this one's: it binds tighter, or as tight and associates to the
    /// left, as the disjunctions and conjunctions do.
    const fn takes_before(self, next: Self) -> bool {
        self.strength() > next.strength()
            || (self.strength() == next.strength() && self.strength() >= 3)
    }
}

/// What waits for the operand to its right.
#[derive(Clone, Copy, Debug)]
enum Pending {
    /// `(`
    Open,
    /// `~` or `¬`
    Not,
    /// A binary connective with its left operand.
    Binary(Binary, NodeId),
    /// A binder, `∀` where the flag is set, else `∃`; its variable's name
    /// is the innermost of `Parser::scope`.
    Binder(bool),
}

/// The dialect read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dialect {
    /// This crate's syntax, a whole sequent.
    Native,
    /// One formula of TPTP's `fof`.
    Tptp,
}

/// A text in the reading.
struct Parser<'a> {
    /// The text and the place in it.
    text: Text<'a>,
    /// The dialect.
    dialect: Dialect,
    /// The formulas read so far.
    formulas: &'a mut Formulas,
    /// What waits for its right operand, the innermost last.
    pending: Vec<Pending>,
    /// How many parentheses are open.
    open: usize,
    /// The most formulas the arena may hold: the caller's bound on
    /// occurrences, since every formula of the arena is at least one.
    limit: Option<u64>,
    /// The names of the variables of the binders waiting for their body,
    /// the innermost last, in NFC.
    scope: Scope,
    /// The names a TPTP problem's reading renames, none on its first try.
    renames: Option<&'a Renames>,
}

/// The names of the variables of the binders waiting for their body, the
/// innermost last, with the places each name stands at, so that a term's
/// name is found in one lookup however many binders are open (a `cnf`
/// clause binds each of its variables).
#[derive(Debug, Default)]
struct Scope {
    /// The names, the innermost last.
    names: Vec<String>,
    /// Per name, its places in `names`, the innermost last.
    places: crate::hash::HashMap<String, Vec<usize>>,
}

impl Scope {
    /// Opens a binder of the variable `name`.
    fn push(&mut self, name: String) {
        self.places
            .entry(name.clone())
            .or_default()
            .push(self.names.len());
        self.names.push(name);
    }

    /// Closes the innermost binder and returns its variable's name.
    fn pop(&mut self) -> Option<String> {
        let name = self.names.pop()?;
        if let Some(places) = self.places.get_mut(&name) {
            places.pop();
        }
        Some(name)
    }

    /// Returns the de Bruijn index of the innermost open binder of `name`:
    /// how many binders lie between it and the term.
    fn find(&self, name: &str) -> Option<u32> {
        let at = *self.places.get(name)?.last()?;
        Some((self.names.len() - 1 - at) as u32)
    }
}

/// The names a TPTP problem's reading renames, so that the sequent reads
/// back in this crate's syntax: a name that is a keyword of it (`par`,
/// `true`), and a function symbol that shares a predicate's name.
#[derive(Debug, Default)]
struct Renames {
    /// A keyword's new name, wherever it stands.
    keywords: crate::hash::HashMap<String, String>,
    /// A function symbol's new name, in term position alone.
    functions: crate::hash::HashMap<String, String>,
}

impl Renames {
    /// Returns a name that no word of `code` is and no renaming gives yet:
    /// `base`, else `base` with `_` after it as often as that takes.
    #[cold]
    #[inline(never)]
    fn fresh(&self, code: &str, base: String) -> String {
        let taken = |name: &str| {
            self.keywords
                .values()
                .chain(self.functions.values())
                .any(|n| n == name)
                || words(code).any(|word| word == name)
        };
        let mut name = base;
        while taken(&name) {
            name.push('_');
        }
        name
    }
}

/// Returns the identifiers of `code` in order, each in NFC, as the arena
/// keys names, a word after `$` (TPTP's `$true`) included.
fn words(code: &str) -> impl Iterator<Item = std::borrow::Cow<'_, str>> {
    code.split(|c: char| !unicode_ident::is_xid_continue(c))
        .filter(|word| !word.is_empty())
        .map(crate::name::normalized)
}

impl<'a> Reads<'a> for Parser<'a> {
    type Id = NodeId;

    fn text(&mut self) -> &mut Text<'a> {
        &mut self.text
    }

    fn formula(&mut self) -> Result<(NodeId, End), Error> {
        loop {
            // An operand, after any prefix operators and parentheses.
            let mut operand = loop {
                if self.text.eat("(") {
                    self.open += 1;
                    self.pending.push(Pending::Open);
                    // A parenthesis makes no formula, but no more than the
                    // bound may wait, so that a run of them cannot pile up.
                    count_within(self.open, self.limit)?;
                } else if self.negation() {
                    self.wait(Pending::Not)?;
                } else if self.starts_binder() && self.binder()? {
                    // The binder waits for its body.
                } else {
                    break self.constant_or_atom()?;
                };
            };
            // The operators after it.
            loop {
                let at = self.text.at;
                let binary = match self.dialect {
                    Dialect::Native => [
                        ("<->", Binary::Iff),
                        ("↔", Binary::Iff),
                        ("->", Binary::Implies),
                        ("→", Binary::Implies),
                        ("\\/", Binary::Or),
                        ("∨", Binary::Or),
                        ("/\\", Binary::And),
                        ("∧", Binary::And),
                    ]
                    .into_iter()
                    .find(|(token, _)| self.text.eat(token)),
                    Dialect::Tptp => [
                        ("<=>", Binary::Iff),
                        ("<~>", Binary::Xor),
                        ("=>", Binary::Implies),
                        ("<=", Binary::Reverse),
                        ("~|", Binary::Nor),
                        ("|", Binary::Or),
                        ("~&", Binary::Nand),
                        ("&", Binary::And),
                    ]
                    .into_iter()
                    .find(|(token, _)| self.text.eat(token)),
                };
                if let Some((_, binary)) = binary {
                    let left = self.reduce(operand, Some(binary))?;
                    self.wait(Pending::Binary(binary, left))?;
                    break;
                }
                if self.open > 0 && self.text.eat(")") {
                    operand = self.reduce(operand, None)?;
                    self.pending.pop();
                    self.open -= 1;
                    continue;
                }
                let end = if self.open > 0 {
                    None
                } else if self.dialect == Dialect::Tptp {
                    self.text.rest().is_empty().then_some(End::Stop)
                } else {
                    self.text.end()
                };
                let Some(end) = end else {
                    let expected: &[&str] = if self.open > 0 {
                        IN_BRACKETS
                    } else if self.dialect == Dialect::Tptp {
                        &["a connective", "the end"]
                    } else {
                        AFTER_FORMULA
                    };
                    let at = self.text.at.max(at);
                    return Err(ParseError::new(self.text.input, at, expected).into());
                };
                return Ok((self.reduce(operand, None)?, end));
            }
        }
    }
}

impl<'a> Parser<'a> {
    /// Consumes a negation if one starts here: `~` or `¬`, and in TPTP a
    /// `~` that does not start `~|` or `~&`.
    fn negation(&mut self) -> bool {
        match self.dialect {
            Dialect::Native => self.text.eat("~") || self.text.eat("¬"),
            Dialect::Tptp => {
                let rest = self.text.rest();
                !rest.starts_with("~|") && !rest.starts_with("~&") && self.text.eat("~")
            }
        }
    }

    /// Notes an operator that waits for its right operand, and fails once
    /// more operators wait than the limit allows. Each will make a formula
    /// of its own, since all lie on one branch of the formula; without the
    /// bound a run of `~` or of `->` would pile up without end.
    fn wait(&mut self, operator: Pending) -> Result<(), Error> {
        self.pending.push(operator);
        count_within(self.pending.len() - self.open, self.limit)
    }

    /// Fails once the arena holds more formulas than the limit allows.
    fn within(&self, id: NodeId) -> Result<NodeId, Error> {
        count_within(self.formulas.len(), self.limit).map(|()| id)
    }

    /// Returns the id of `node`, adding it to the arena within the limit.
    fn add(&mut self, node: Node) -> Result<NodeId, Error> {
        let id = self.formulas.add(node)?;
        self.within(id)
    }

    /// Reads a constant or an atom.
    fn constant_or_atom(&mut self) -> Result<NodeId, Error> {
        let at = self.text.at;
        let native = self.dialect == Dialect::Native;
        if native && self.text.eat("⊤") {
            return self.add(Node::True);
        }
        if native && self.text.eat("⊥") {
            return self.add(Node::False);
        }
        if !native && self.text.eat("$true") {
            return self.add(Node::True);
        }
        if !native && self.text.eat("$false") {
            return self.add(Node::False);
        }
        match self.text.identifier() {
            Some("true") if native => self.add(Node::True),
            Some("false") if native => self.add(Node::False),
            Some(name) if self.text.input.as_bytes().get(self.text.at) == Some(&b'(') => {
                self.predicate(name)
            }
            Some(name) if self.equality_follows() => self.equality_after(name, at),
            Some(name) => {
                let id = match self.formulas.atom(name) {
                    Ok(id) => id,
                    Err(error) => self.renamed_atom(name, error)?,
                };
                self.within(id)
            }
            None => Err(self.text.unexpected(at, &["a formula"])),
        }
    }

    /// Reads an identifier if one starts here, out of line, for the
    /// first-order paths, so that the operand loop's copy is the only one
    /// inlined there.
    #[cold]
    #[inline(never)]
    fn word(&mut self) -> Option<&'a str> {
        self.text.identifier()
    }

    /// Returns whether a binder may start here: the first character is
    /// that of `forall`, `exists`, `∀`, `∃` or TPTP's `!` and `?`. It
    /// reads nothing, so a propositional text pays one comparison; the
    /// white space before it is skipped already.
    fn starts_binder(&self) -> bool {
        let first = self.text.input.as_bytes().get(self.text.at).copied();
        match self.dialect {
            Dialect::Native => matches!(first, Some(b'f' | b'e' | 0xE2)),
            Dialect::Tptp => matches!(first, Some(b'!' | b'?')),
        }
    }

    /// Reads a binder's quantifier and variables if one starts here, and
    /// returns whether it did: `forall x y.` or `∀x.` (and `exists`, `∃`)
    /// natively, `! [X, Y] :` or `? [X] :` in TPTP. Each variable waits
    /// for the body as a binder of its own, the last the innermost.
    #[cold]
    #[inline(never)]
    fn binder(&mut self) -> Result<bool, Error> {
        let start = self.text.at;
        let forall = match self.dialect {
            Dialect::Native => {
                if self.text.eat("∀") {
                    true
                } else if self.text.eat("∃") {
                    false
                } else {
                    match self.word() {
                        Some("forall") => true,
                        Some("exists") => false,
                        _ => {
                            self.text.at = start;
                            return Ok(false);
                        }
                    }
                }
            }
            Dialect::Tptp => {
                let forall = self.text.eat("!");
                if !forall && !self.text.eat("?") {
                    return Ok(false);
                }
                if !self.text.eat("[") {
                    let at = self.text.next_at();
                    return Err(self.text.unexpected(at, &["`[`"]));
                }
                forall
            }
        };
        let mut count = 0;
        loop {
            let at = self.text.next_at();
            let done = match self.dialect {
                Dialect::Native => self.text.eat("."),
                Dialect::Tptp => self.text.eat("]"),
            };
            if done {
                if count == 0 {
                    return Err(self.text.unexpected(at, &["a variable"]));
                }
                if self.dialect == Dialect::Tptp && !self.text.eat(":") {
                    let at = self.text.next_at();
                    return Err(self.text.unexpected(at, &["`:`"]));
                }
                return Ok(true);
            }
            if count > 0 && self.dialect == Dialect::Tptp && !self.text.eat(",") {
                return Err(self.text.unexpected(at, &["`,`", "`]`"]));
            }
            let at = self.text.next_at();
            let Some(name) = self.word() else {
                let expected: &[&str] = match self.dialect {
                    Dialect::Native if count > 0 => &["a variable", "`.`"],
                    Dialect::Native | Dialect::Tptp => &["a variable"],
                };
                return Err(self.text.unexpected(at, expected));
            };
            let name = crate::name::normalized(name).into_owned();
            if crate::name::check_ordinary(&name).is_err() {
                return Err(self.text.unexpected(at, &["a variable"]));
            }
            self.scope.push(name);
            self.wait(Pending::Binder(forall))?;
            count += 1;
        }
    }

    /// Returns the atom of a keyword's new name where the reading renames
    /// it, else `error`, which the arena gave for `name`.
    #[cold]
    #[inline(never)]
    fn renamed_atom(&mut self, name: &str, error: Error) -> Result<NodeId, Error> {
        let name = crate::name::normalized(name);
        match self.renames.and_then(|r| r.keywords.get(&*name)) {
            Some(new) => self.formulas.atom(new),
            None => Err(error),
        }
    }

    /// Returns the name a symbol is read under, in NFC: a keyword's new
    /// name, and in term position (`function`) a function symbol's, which
    /// a keyword's new name may have too (`par` beside `q(par)`).
    fn symbol_name(&self, name: &str, function: bool) -> String {
        let name = crate::name::normalized(name);
        let Some(renames) = self.renames else {
            return name.into_owned();
        };
        let base = renames.keywords.get(&*name).map_or(&*name, String::as_str);
        let renamed = if function {
            renames.functions.get(base).map_or(base, String::as_str)
        } else {
            base
        };
        renamed.to_owned()
    }

    /// Reads the arguments of the predicate `name`, whose `(` follows,
    /// and returns its atom. A term is a variable of a binder waiting for
    /// its body, the innermost of that name, or a function symbol applied
    /// to terms, a constant without brackets. It reads on a stack of its
    /// own, so a term of any depth costs no recursion. In TPTP a name
    /// that starts upper-case is a variable, and one that no binder binds
    /// is refused.
    #[cold]
    #[inline(never)]
    fn predicate(&mut self, name: &'a str) -> Result<NodeId, Error> {
        let (symbol, args) = self.application(name)?;
        // `f(a) = b`: the application was a term.
        if self.equality_follows() {
            let symbol = self.symbol_name(symbol, true);
            let left = self.formulas.function(&symbol, &args)?;
            return self.equality(left);
        }
        let symbol = self.symbol_name(symbol, false);
        let id = self.formulas.predicate(&symbol, &args)?;
        self.within(id)
    }

    /// Reads the arguments of `name`, whose `(` follows, and returns the
    /// name with its arguments, every inner application made a term. It
    /// reads on a stack of its own, so a term of any depth costs no
    /// recursion.
    fn application(&mut self, name: &'a str) -> Result<(&'a str, Vec<FoTermId>), Error> {
        // The applications open, each its symbol's name and the arguments
        // read so far.
        let mut open: Vec<(&'a str, Vec<FoTermId>)> = Vec::new();
        let mut name = name;
        loop {
            // A `(` follows `name`: its arguments start.
            self.text.at += 1;
            open.push((name, Vec::new()));
            loop {
                let here = self.text.next_at();
                let Some(word) = self.word() else {
                    return Err(self.text.unexpected(here, &["a term"]));
                };
                if self.text.input[self.text.at..].starts_with('(') {
                    name = word;
                    break;
                }
                let term = self.bare_term(word, here)?;
                // The term may end its own application and those around.
                let mut term = Some(term);
                loop {
                    if let Some(term) = term.take() {
                        open.last_mut()
                            .expect("an application is open")
                            .1
                            .push(term);
                    }
                    if self.text.eat(",") {
                        break;
                    }
                    let close = self.text.next_at();
                    if !self.text.eat(")") {
                        return Err(self.text.unexpected(close, &["`,`", "`)`"]));
                    }
                    let (symbol, args) = open.pop().expect("an application is open");
                    if open.is_empty() {
                        return Ok((symbol, args));
                    }
                    let symbol = self.symbol_name(symbol, true);
                    term = Some(self.formulas.function(&symbol, &args)?);
                }
            }
        }
    }

    /// Returns the atom `name = t` or its negation, `name` the word read at
    /// `here` and the sign next.
    #[cold]
    #[inline(never)]
    fn equality_after(&mut self, name: &str, here: usize) -> Result<NodeId, Error> {
        let left = self.bare_term(name, here)?;
        self.equality(left)
    }

    /// Returns the term a word without arguments is, read at `here`: the
    /// variable of the innermost binder of that name waiting for its body,
    /// else a constant. In TPTP an upper-case word is a variable, and one
    /// that no binder binds is refused.
    #[cold]
    #[inline(never)]
    fn bare_term(&mut self, word: &str, here: usize) -> Result<FoTermId, Error> {
        let written = here..here + word.len();
        let word = crate::name::normalized(word);
        match self.scope.find(&word) {
            Some(index) => Ok(self.formulas.variable(index)),
            None if self.dialect == Dialect::Tptp && word.starts_with(char::is_uppercase) => {
                // TPTP's own rule (its language description, "Annotated
                // Formulae"): outside CNF every variable is bound by a
                // quantification whose scope holds it.
                let error = ParseError::spanning(
                    self.text.input,
                    written,
                    &["a bound variable", "a constant"],
                );
                Err(Error::from(error.with_hint(
                    "an upper-case word is a variable in TPTP, and no quantifier around it \
                     binds it: each variable of a fof formula stands inside its quantifier's scope",
                )))
            }
            None => {
                let word = self.symbol_name(&word, true);
                self.formulas.function(&word, &[])
            }
        }
    }

    /// Returns whether `=`, `!=` or natively `≠` follows, skipping white
    /// space: the operand just read is then a term. A propositional text
    /// pays one compare per atom; TPTP's `=>` is no equality.
    fn equality_follows(&self) -> bool {
        // An inline scan, not `rest`: a call per atom cost the `read-tptp`
        // journey 2 %.
        let bytes = self.text.input.as_bytes();
        let mut at = self.text.at;
        while bytes.get(at).is_some_and(u8::is_ascii_whitespace) {
            at += 1;
        }
        matches!(bytes.get(at), Some(b'=' | b'!' | 0xE2)) && self.equality_sign(at)
    }

    /// Returns whether an equality's sign starts at byte `at`.
    #[cold]
    #[inline(never)]
    fn equality_sign(&self, at: usize) -> bool {
        let rest = &self.text.input[at..];
        (rest.starts_with('=') && !(self.dialect == Dialect::Tptp && rest.starts_with("=>")))
            || rest.starts_with("!=")
            || (self.dialect == Dialect::Native && rest.starts_with('≠'))
    }

    /// Reads the sign that follows the term `left` and the term after it,
    /// and returns the atom `left = right`, or its negation for `!=` and
    /// `≠`.
    #[cold]
    #[inline(never)]
    fn equality(&mut self, left: FoTermId) -> Result<NodeId, Error> {
        let negated = self.text.eat("!=") || self.text.eat("≠");
        if !negated {
            self.text.eat("=");
        }
        let here = self.text.next_at();
        let Some(word) = self.word() else {
            return Err(self.text.unexpected(here, &["a term"]));
        };
        let right = if self.text.input[self.text.at..].starts_with('(') {
            let (symbol, args) = self.application(word)?;
            let symbol = self.symbol_name(symbol, true);
            self.formulas.function(&symbol, &args)?
        } else {
            self.bare_term(word, here)?
        };
        let equal = self.formulas.equality(left, right)?;
        let equal = self.within(equal)?;
        if negated {
            self.add(Node::Not(equal))
        } else {
            Ok(equal)
        }
    }

    /// Makes the binder waiting innermost, `∀` where `forall` is set, of
    /// the body `operand`, and returns it.
    #[cold]
    #[inline(never)]
    fn close_binder(&mut self, forall: bool, operand: NodeId) -> Result<NodeId, Error> {
        self.pending.pop();
        let name = self.scope.pop().expect("a name per binder");
        let id = self.formulas.binder(forall, operand, &name)?;
        self.within(id)
    }

    /// Gives `operand` to the pending operators that take it before
    /// `next` does, or to all of them up to the innermost open parenthesis
    /// for `None`, and returns the formula they make of it.
    fn reduce(&mut self, mut operand: NodeId, next: Option<Binary>) -> Result<NodeId, Error> {
        while let Some(&top) = self.pending.last() {
            let node = match top {
                Pending::Open => break,
                Pending::Binder(forall) => {
                    // A native binder reaches as far right as it can: a
                    // connective after its body is inside it. TPTP's binds
                    // the unit that follows it.
                    if next.is_some() && self.dialect == Dialect::Native {
                        break;
                    }
                    operand = self.close_binder(forall, operand)?;
                    continue;
                }
                Pending::Not => Node::Not(operand),
                Pending::Binary(binary, left) => {
                    if next.is_some_and(|next| !binary.takes_before(next)) {
                        break;
                    }
                    match binary {
                        Binary::Iff => Node::Iff(left, operand),
                        Binary::Xor => Node::Not(self.add(Node::Iff(left, operand))?),
                        Binary::Implies => Node::Implies(left, operand),
                        Binary::Reverse => Node::Implies(operand, left),
                        Binary::Or => Node::Or(left, operand),
                        Binary::Nor => Node::Not(self.add(Node::Or(left, operand))?),
                        Binary::And => Node::And(left, operand),
                        Binary::Nand => Node::Not(self.add(Node::And(left, operand))?),
                    }
                }
            };
            self.pending.pop();
            operand = self.add(node)?;
        }
        Ok(operand)
    }
}

impl std::str::FromStr for Sequent {
    type Err = Error;

    /// Parses a sequent of ordinary logic, or one formula to prove, in
    /// the syntax [`Sequent`] documents, within the default limits.
    fn from_str(s: &str) -> Result<Self, Error> {
        Self::parse_within(s, &Limits::default())
    }
}

impl Sequent {
    /// Parses a sequent of ordinary logic, or one formula to prove, in the
    /// syntax [`Sequent`] documents. It refuses the text as soon as its
    /// distinct subformulas, its formulas, the operators waiting for an
    /// operand or the open parentheses pass `limits.occurrences`, before
    /// anything of that size is built: each formula is at least one
    /// occurrence of its image.
    ///
    /// Needs the cargo feature `parse` (on by default).
    ///
    /// # Errors
    ///
    /// [`Error::Parse`] for text that is no sequent, [`Error::AtomName`]
    /// for an atom named by a keyword, and [`Refusal::Occurrences`](crate::Refusal::Occurrences) past
    /// the bound.
    pub fn parse_within(s: &str, limits: &Limits) -> Result<Self, Error> {
        let mut formulas = Formulas::default();
        let mut parser = Parser {
            text: Text { input: s, at: 0 },
            dialect: Dialect::Native,
            formulas: &mut formulas,
            pending: Vec::new(),
            open: 0,
            limit: limits.occurrences,
            scope: Scope::default(),
            renames: None,
        };
        let (left, right) = sides(&mut parser, limits.occurrences)?;
        Sequent::new(formulas, left, right)
    }
}

/// Reads the one formula that `text[span]` holds into `formulas`, within
/// `limits.occurrences`, and returns its id. An error names its place in
/// the whole text, up to the formula's end.
///
/// # Errors
///
/// [`Error::Parse`] where the span holds no formula or more than one,
/// [`Error::AtomName`] for an atom named by a keyword, and
/// [`Refusal::Occurrences`](crate::Refusal::Occurrences) past the bound.
#[cfg(all(feature = "interactive", feature = "serialize"))]
pub(super) fn formula_in(
    text: &str,
    span: std::ops::Range<usize>,
    formulas: &mut Formulas,
    limits: &Limits,
) -> Result<NodeId, Error> {
    let input = &text[..span.end];
    let mut parser = Parser {
        text: Text {
            input,
            at: span.start,
        },
        dialect: Dialect::Native,
        formulas,
        pending: Vec::new(),
        open: 0,
        limit: limits.occurrences,
        scope: Scope::default(),
        renames: None,
    };
    let at = parser.text.at;
    let (formula, end) = match parser.formula() {
        Ok(read) => read,
        // The parser read the text up to the formula's end, so an error
        // there says "the end of input"; it is placed in the whole text
        // instead, at the character after the formula.
        Err(Error::Parse(error)) if error.found.is_none() && span.end < text.len() => {
            let placed = ParseError::new(text, span.end, error.expected);
            return Err(match error.hint {
                Some(hint) => placed.with_hint(hint),
                None => placed,
            }
            .into());
        }
        Err(error) => return Err(error),
    };
    match end {
        End::Stop => Ok(formula),
        End::Comma | End::Turnstile => {
            let read = &input[at..parser.text.at];
            let token = if read.ends_with("|-") {
                2
            } else if read.ends_with('⊢') {
                '⊢'.len_utf8()
            } else {
                1
            };
            let span = parser.text.at - token..parser.text.at;
            Err(
                ParseError::spanning(input, span, &["a connective", "the end of the formula"])
                    .into(),
            )
        }
    }
}

/// A problem of ordinary logic read from a TPTP file.
///
/// Needs the cargo feature `parse` (on by default).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Problem {
    /// The axioms left of `⊢`, the conjecture right of it.
    pub sequent: Sequent,
    /// The provability the file's header claims: its `Status (intuit.)`
    /// line, else its plain `Status` line.
    pub status: Option<Status>,
}

/// Reads a problem of TPTP's `fof` syntax, as the ILTP library writes
/// them: every clause with the role `axiom`, `hypothesis`, `lemma` or
/// `theorem` is a hypothesis, the one `conjecture` the formula right of
/// `⊢`. A file without a conjecture asks that its hypotheses be refuted:
/// nothing stands right of `⊢`. A formula
/// is built over `$true`, `$false` and atoms with these connectives, from
/// the tightest binding to the loosest: `~`, `! [X, …] :` and `? [X, …] :`;
/// `&` and `~&`; `|` and `~|`; `=>` and `<=`; `<=>` and `<~>`. TPTP asks
/// for parentheses wherever this order matters, and binds `~` as tightly
/// as here. A quantifier binds the unit after it: `! [X] : p(X) & q` is
/// `(∀X. p(X)) ∧ q`. An atom may be a predicate applied to terms, a term
/// a variable (upper-case, bound by a quantifier) or a function symbol
/// applied to terms (lower-case). `s = t` and `s != t` say that two terms
/// are equal and that they are not. A clause `cnf(name, role, clause).`
/// is the universal closure of its disjunction of literals over its
/// upper-case variables, and its role may be `negated_conjecture`, a
/// hypothesis like the others. The file's statuses are read as
/// [`lltp::read`](crate::linear::lltp::read) reads them.
///
/// A name TPTP allows and this crate's syntax keeps for itself is renamed
/// so that the sequent reads back: a keyword (`par`, `true`) gets `_`
/// after it, and a function symbol named like a predicate `_f`, each with
/// more `_` until it is no word of the file.
///
/// Every formula is read within `limits.occurrences`, as
/// [`Sequent::parse_within`] reads them.
///
/// Needs the cargo feature `parse` (on by default).
///
/// # Errors
///
/// [`Error::Tptp`] for a file that is not a sequence of such clauses,
/// [`Error::SeveralConjectures`] for a file with more than one conjecture
/// (TPTP asks for each to be proved, which right of `⊢` would read as
/// their disjunction), [`Error::Parse`] for a formula that is not one,
/// [`Error::AtomName`] for an atom named by a keyword of the ordinary
/// syntax, such as `true`, [`Error::Symbol`] for a name used with two
/// arities or as a predicate and a function symbol, and
/// [`Refusal::Occurrences`](crate::Refusal::Occurrences) past the bound.
pub fn read_tptp(text: &str, limits: &Limits) -> Result<Problem, Error> {
    let clauses = clauses(text, |message| Error::Tptp { message }, true)?;
    // A name TPTP allows and this syntax refuses is renamed, and the file
    // read again: a keyword, or a name both a predicate and a function
    // symbol. Each try renames one name more, so the tries end; a file
    // that needs none is read once, as before.
    let mut renames = Renames::default();
    loop {
        match read_clauses(&clauses, text, limits, &renames) {
            Err(Error::AtomName { name })
                if crate::name::is_identifier(&name) && !renames.keywords.contains_key(&name) =>
            {
                let new = renames.fresh(clauses.code(), format!("{name}_"));
                renames.keywords.insert(name, new);
            }
            Err(Error::Symbol {
                name,
                fault: crate::first_order::SymbolFault::Role,
            }) if !renames.functions.contains_key(&name) => {
                let new = renames.fresh(clauses.code(), format!("{name}_f"));
                renames.functions.insert(name, new);
            }
            read => return read,
        }
    }
}

/// Returns the text of a TPTP file's bytes, for [`read_tptp`]. A TPTP
/// formula is ASCII, but some files write an author's name in Latin-1 in
/// a comment, as 107 of the ILTP library's first-order problems do: a
/// byte that is no UTF-8 is read as U+FFFD inside a `%` comment, which
/// the reader skips, and is refused anywhere else. Text that is UTF-8 is
/// borrowed as it is.
///
/// A caller that inlines a file's `include('NAME').` lines decodes each
/// file with this before it joins them; the library reads no files.
///
/// Needs the cargo feature `parse` (on by default).
///
/// ```
/// use deduxit::Limits;
/// use deduxit::ordinary;
///
/// // `é` in Latin-1, in a comment.
/// let bytes = b"% by Jos\xe9\nfof(c, conjecture, p => p).\n";
/// let text = ordinary::tptp_text(bytes)?;
/// let problem = ordinary::read_tptp(&text, &Limits::default())?;
/// assert_eq!(problem.sequent.to_string(), "\u{22a2} p \u{2192} p");
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # Errors
///
/// [`Error::Tptp`] for a byte that is no UTF-8 outside a comment, naming
/// its line.
pub fn tptp_text(bytes: &[u8]) -> Result<std::borrow::Cow<'_, str>, Error> {
    if let Ok(text) = std::str::from_utf8(bytes) {
        return Ok(std::borrow::Cow::Borrowed(text));
    }
    // Where the text is: in a formula, a comment, or a quoted name or
    // string, whose `%` opens no comment.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum At {
        Code,
        Comment,
        Quoted(char),
    }
    let mut at = At::Code;
    let mut line = 1usize;
    let mut escaped = false;
    let mut out = String::with_capacity(bytes.len());
    for chunk in bytes.utf8_chunks() {
        for c in chunk.valid().chars() {
            out.push(c);
            match at {
                At::Code => match c {
                    '%' => at = At::Comment,
                    '\'' | '"' => at = At::Quoted(c),
                    _ => {}
                },
                At::Comment => {
                    if c == '\n' {
                        at = At::Code;
                    }
                }
                At::Quoted(quote) => {
                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == quote {
                        at = At::Code;
                    }
                }
            }
            if c == '\n' {
                line += 1;
            }
        }
        if !chunk.invalid().is_empty() {
            if at != At::Comment {
                return Err(Error::Tptp {
                    message: format!(
                        "line {line} holds a byte that is not UTF-8 outside a comment"
                    ),
                });
            }
            out.push(char::REPLACEMENT_CHARACTER);
        }
    }
    Ok(std::borrow::Cow::Owned(out))
}

/// Reads the clauses of a TPTP file, `text`, with the names `renames`
/// gives, as [`read_tptp`] describes.
fn read_clauses(
    clauses: &Clauses,
    text: &str,
    limits: &Limits,
    renames: &Renames,
) -> Result<Problem, Error> {
    let mut formulas = Formulas::default();
    let mut roots = 0;
    let mut read = |formula: &str, clausal: bool| {
        roots += 1;
        count_within(roots, limits.occurrences)?;
        // A parse error's place in the file, which the formula is a part of.
        let (start, len) = (
            formula.as_ptr() as usize - clauses.code().as_ptr() as usize,
            formula.len(),
        );
        // A `cnf` clause is the universal closure of its disjunction, read
        // as `! [X, …] : (clause)`; `prefix` is that text's length before
        // the clause.
        let closed;
        let (input, prefix) = match clausal.then(|| closure(formula)).flatten() {
            Some((text, prefix)) => {
                closed = text;
                (closed.as_str(), prefix)
            }
            None => (formula, 0),
        };
        let mut parser = Parser {
            text: Text { input, at: 0 },
            dialect: Dialect::Tptp,
            formulas: &mut formulas,
            pending: Vec::new(),
            open: 0,
            limit: limits.occurrences,
            scope: Scope::default(),
            renames: Some(renames),
        };
        parser
            .formula()
            .map(|(formula, _)| formula)
            .map_err(|error| {
                if let Error::Parse(error) = error {
                    let at = |at: usize| {
                        let at = at.saturating_sub(prefix);
                        (start + at.min(len), at >= len)
                    };
                    Error::Parse(Box::new(clauses.located(text, &error, at)))
                } else {
                    error
                }
            })
    };
    let left: Vec<_> = clauses
        .hypotheses()
        .map(|(formula, clausal)| read(formula, clausal))
        .collect::<Result<_, _>>()?;
    // Without a conjecture TPTP asks for the hypotheses' refutation; a
    // file of no formula at all, empty or cut short, is no problem.
    let right = match clauses.conjecture() {
        Some(conjecture) => vec![read(conjecture, false)?],
        None if left.is_empty() => {
            return Err(Error::Tptp {
                message: "no formula".to_owned(),
            });
        }
        None => Vec::new(),
    };
    Ok(Problem {
        sequent: Sequent::new(formulas, left, right)?,
        status: clauses.status,
    })
}

/// Returns the universal closure of the `cnf` clause `clause` as TPTP text,
/// `! [X, …] : (clause)` with its variables (its words that start
/// upper-case) in the order they first occur, and the length of the text
/// before the clause; `None` for a clause without variables, which reads
/// as it is.
#[cold]
#[inline(never)]
fn closure(clause: &str) -> Option<(String, usize)> {
    let mut variables: Vec<std::borrow::Cow<'_, str>> = Vec::new();
    let mut seen = crate::hash::HashSet::default();
    for word in words(clause) {
        if word.starts_with(|c: char| c.is_ascii_uppercase()) && seen.insert(word.clone()) {
            variables.push(word);
        }
    }
    if variables.is_empty() {
        return None;
    }
    let mut text = format!("! [{}] : (", variables.join(", "));
    let prefix = text.len();
    text.push_str(clause);
    text.push(')');
    Some((text, prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both spellings of every connective, the precedences and the
    /// associativities read as documented; a text without a turnstile is
    /// a formula to prove, and the linear connectives are refused.
    #[test]
    fn reads_the_syntax() {
        let read = |text: &str| text.parse::<Sequent>().unwrap().to_string();
        assert_eq!(
            read("a /\\ b \\/ ~c -> d <-> e"),
            "⊢ (((a ∧ b) ∨ ¬c) → d) ↔ e"
        );
        assert_eq!(
            read("a ∧ b ∨ ¬c → d ↔ e"),
            read("a /\\ b \\/ ~c -> d <-> e")
        );
        assert_eq!(read("a -> b -> c"), "⊢ a → (b → c)");
        assert_eq!(read("a /\\ b /\\ c"), "⊢ (a ∧ b) ∧ c");
        assert_eq!(read("true, ⊥ |- false, ⊤"), "⊤, ⊥ ⊢ ⊥, ⊤");
        assert_eq!(read("|-"), "⊢");
        assert_eq!(read("a |-"), "a ⊢");
        assert_eq!(read("~~(a)"), "⊢ ¬¬a");
        for bad in [
            "a & b",
            "a | b",
            "a -o b",
            "a |- b |- c",
            "a ⊢ b ⊢ c",
            "(a",
            "a)",
            "a b",
            "",
        ] {
            assert!(bad.parse::<Sequent>().is_err(), "{bad:?}");
        }
        // A second turnstile is one token, found where it starts.
        for (bad, column, found) in [("a |- b |- c", 8, "|-"), ("a ⊢ b ⊢ c", 7, "⊢")] {
            let Err(Error::Parse(e)) = bad.parse::<Sequent>() else {
                panic!("{bad:?}");
            };
            assert_eq!(
                (e.column, e.found.as_deref()),
                (column, Some(found)),
                "{bad:?}"
            );
        }
        // Inside a parenthesis only a connective or its end can follow;
        // outside, in TPTP, the end.
        let Err(Error::Parse(e)) = "(a".parse::<Sequent>() else {
            panic!("(a");
        };
        assert_eq!(e.expected, ["a connective", "`)`"]);
        let Err(Error::Parse(e)) = read_tptp("fof(c, conjecture, a b).", &Limits::default()) else {
            panic!("a b");
        };
        assert_eq!(e.expected, ["a connective", "the end"]);
    }

    /// A binder reaches as far right as it can, takes several variables,
    /// and binds the innermost variable of its name; predicates take
    /// nested terms; what is printed reads back as itself.
    #[test]
    fn reads_binders_and_terms() {
        let read = |text: &str| text.parse::<Sequent>().unwrap().to_string();
        assert_eq!(
            read("forall x. p(x) -> q(x), p(a) |- q(a)"),
            "∀x. (p(x) → q(x)), p(a) ⊢ q(a)"
        );
        assert_eq!(
            read("∀x y. r(x, f(y, a))"),
            read("forall x. forall y. r(x, f(y, a))")
        );
        assert_eq!(read("(exists x. p(x)) /\\ q"), "⊢ (∃x. p(x)) ∧ q");
        assert_eq!(read("~ exists x. p(x) /\\ q"), "⊢ ¬(∃x. (p(x) ∧ q))");
        assert_eq!(read("a -> forall x. p(x)"), "⊢ a → (∀x. p(x))");
        // The inner `x` shadows the outer one.
        assert_eq!(read("forall x. exists x. p(x)"), "⊢ ∀x. ∃x_1. p(x_1)");
        // Bound names do not matter: one formula is one node.
        let sequent: Sequent = "forall x. p(x) |- forall y. p(y)".parse().unwrap();
        assert_eq!(sequent.left(), sequent.right());
        // A propositional atom beside a predicate is its nullary predicate.
        assert_eq!(read("p(a), b |- b"), "p(a), b ⊢ b");
        for text in [
            "forall x. p(x) -> q(x), p(a) |- q(a)",
            "∀x y. r(x, f(y, a))",
            "forall x. exists x. p(x)",
            "exists x. p(x, x), forall y. (p(y, y) <-> ~q) |- ⊥",
        ] {
            let printed = read(text);
            assert_eq!(read(&printed), printed, "{text}");
        }
        for bad in [
            "forall . p",
            "forall x p(x)",
            "forall forall. p",
            "p()",
            "p(a,)",
            "p(a",
            "p(a) /\\ p",
            "p(a) /\\ p(a, a)",
            "p(a) /\\ a",
            "p(f) /\\ p(f(a))",
            "p(p(a))",
        ] {
            assert!(bad.parse::<Sequent>().is_err(), "{bad:?}");
        }
    }

    /// In TPTP a quantifier binds the unit after it, and an upper-case
    /// word that no quantifier binds is refused.
    #[test]
    fn reads_tptp_quantifiers() {
        let read = |formula: &str| {
            let text = format!("fof(c, conjecture, {formula}).");
            read_tptp(&text, &Limits::default()).map(|p| p.sequent.to_string())
        };
        assert_eq!(read("! [X] : p(X) & q").unwrap(), "⊢ (∀X. p(X)) ∧ q");
        assert_eq!(
            read("? [X, Y] : (r(X, f(Y)) => r(a, Y))").unwrap(),
            "⊢ ∃X. ∃Y. (r(X, f(Y)) → r(a, Y))"
        );
        assert!(read("p(X)").is_err());
        assert!(read("! [] : p").is_err());
        assert!(read("! X : p(X)").is_err());
        // A variable used past its quantifier's scope, as ILTP's GEJ056+1,
        // GEJ056+2, GEJ057+1 and GEJ057+2 write their conjectures: the
        // whole word is named, and why it is refused.
        let free = read("(! [Ab] : p(Ab)) => q(Ab)").unwrap_err().to_string();
        assert!(
            free.contains("unexpected \"Ab\" at line 1, character 42")
                && free.contains("no quantifier around it binds it"),
            "{free}"
        );
        // Equality between terms, `!=` its negation; `=>` stays the
        // implication.
        assert_eq!(
            read("! [X] : (f(X) = X => ~ a != g(b, X))").unwrap(),
            "⊢ ∀X. (f(X) = X → ¬¬a = g(b, X))"
        );
        assert!(read("a = ").is_err());
        // An unmatched bracket is refused.
        assert!(read("p ] q").is_err());
    }

    /// A `cnf` clause is the universal closure of its disjunction, a
    /// negated conjecture one more hypothesis; a `cnf` conjecture is
    /// refused, and an error in a clause names its place in the file. The
    /// native syntax reads `=`, `!=` and `≠` and prints `=` back.
    #[test]
    fn reads_cnf_and_equality() {
        let text = "cnf(a, axiom, p(X) | ~ q(X, Y) | X = f(Y)).\n\
                    cnf(n, negated_conjecture, ~ p(a)).\n\
                    fof(b, axiom, q(a, b)).\n";
        let problem = read_tptp(text, &Limits::default()).unwrap();
        assert_eq!(
            problem.sequent.to_string(),
            "∀X. ∀Y. ((p(X) ∨ ¬q(X, Y)) ∨ X = f(Y)), ¬p(a), q(a, b) ⊢"
        );
        assert!(read_tptp("cnf(c, conjecture, p).", &Limits::default()).is_err());
        let Err(Error::Parse(e)) = read_tptp("cnf(a, axiom, p(X) | | q).", &Limits::default())
        else {
            panic!("an empty literal");
        };
        assert_eq!((e.line, e.column), (1, 22));

        let sequent: Sequent = "a = b, b != c |- f(a) ≠ c, forall x. x = x"
            .parse()
            .unwrap();
        assert_eq!(sequent.to_string(), "a = b, ¬b = c ⊢ ¬f(a) = c, ∀x. x = x");
        let again: Sequent = sequent.to_string().parse().unwrap();
        assert_eq!(again.to_string(), sequent.to_string());
        assert!(sequent.formulas().has_equality());
    }

    /// TPTP's connectives, its constants, the clauses' roles and the
    /// status, with `~` as tight as its operand.
    #[test]
    fn reads_tptp() {
        let text = "% Status (intuit.) : Non-Theorem\n\
                    fof(a, axiom, ~ p => (q <= r)).\n\
                    fof(b, hypothesis, (p <~> q) & (p ~| $true) & (q ~& $false)).\n\
                    fof(c, conjecture, ~ ~ p <=> p).\n";
        let problem = read_tptp(text, &Limits::default()).unwrap();
        assert_eq!(problem.status, Some(Status::NonTheorem));
        assert_eq!(
            problem.sequent.to_string(),
            "¬p → (r → q), (¬(p ↔ q) ∧ ¬(p ∨ ⊤)) ∧ ¬(q ∧ ⊥) ⊢ ¬¬p ↔ p"
        );
        assert!(matches!(
            read_tptp("fof(c, axiom, p) & q.", &Limits::default()),
            Err(Error::Tptp { .. })
        ));
        assert!(read_tptp("fof(c, conjecture, p -> q).", &Limits::default()).is_err());
        // A formula that does not parse is placed in the file, here on its
        // second line where the clause closes too early.
        let place = |text: &str| match read_tptp(text, &Limits::default()) {
            Err(Error::Parse(e)) => (e.line, e.column, e.found.clone()),
            other => panic!("{other:?}"),
        };
        assert_eq!(
            place("fof(a, axiom, p).\nfof(b, axiom, p & ).\nfof(c, conjecture, p)."),
            (2, 19, Some(")".to_owned()))
        );
        // A file without a conjecture asks for its hypotheses' refutation,
        // and a lemma is a hypothesis.
        let refuted =
            read_tptp("fof(a, axiom, p).\nfof(b, lemma, ~p).", &Limits::default()).unwrap();
        assert_eq!(refuted.sequent.to_string(), "p, ¬p ⊢");
        assert!(read_tptp("% nothing\n", &Limits::default()).is_err());
        // An intuitionistic status that claims nothing is not replaced by
        // the classical one.
        let open = "% Status : Theorem\n% Status (intuit.) : Open\nfof(c, conjecture, p).";
        assert_eq!(read_tptp(open, &Limits::default()).unwrap().status, None);
        // A name this syntax keeps for itself, and a function symbol that
        // shares a predicate's name, are renamed, fresh against every word
        // of the file, so the sequent reads back in this syntax.
        for (text, read) in [
            ("fof(c, conjecture, par | ~ par).", "⊢ par_ ∨ ¬par_"),
            (
                "fof(a, axiom, par_).\nfof(c, conjecture, p(par)).",
                "par_ ⊢ p(par__)",
            ),
            ("fof(a, axiom, q(f)).\nfof(c, conjecture, f).", "q(f_f) ⊢ f"),
            (
                "fof(a, axiom, f).\nfof(c, conjecture, q(g(f))).",
                "f ⊢ q(g(f_f))",
            ),
            ("fof(c, conjecture, ! [X] : true(X)).", "⊢ ∀X. true_(X)"),
            ("fof(c, conjecture, par | q(par)).", "⊢ par_ ∨ q(par__f)"),
            // `é` written composed and decomposed is one name, so the new
            // name of its function is fresh against both spellings.
            (
                "fof(c, conjecture, q(\u{e9}(a)) | \u{e9}(a) | q(e\u{301}_f(a))).",
                "⊢ (q(é_f_(a)) ∨ é(a)) ∨ q(é_f(a))",
            ),
        ] {
            let problem = read_tptp(text, &Limits::default()).unwrap();
            let printed = problem.sequent.to_string();
            assert_eq!(printed, read, "{text}");
            printed.replace('∀', "forall ").parse::<Sequent>().unwrap();
        }
        // TPTP's constants are `$true` and `$false`; a plain `true` would
        // name an atom that this syntax reads as the constant, so it is
        // renamed too.
        assert_eq!(
            read_tptp("fof(c, conjecture, true | p).", &Limits::default())
                .unwrap()
                .sequent
                .to_string(),
            "⊢ true_ ∨ p"
        );
    }

    /// A second conjecture is refused by name: right of `⊢` the two would
    /// be their disjunction, and `p ⊢ p, q` is valid where `p ⊢ q` is not.
    #[test]
    fn refuses_several_conjectures() {
        let text = "fof(a, axiom, p). fof(c1, conjecture, p). fof(c2, conjecture, q).";
        assert!(matches!(
            read_tptp(text, &Limits::default()),
            Err(Error::SeveralConjectures { second }) if second == "c2"
        ));
    }
}
