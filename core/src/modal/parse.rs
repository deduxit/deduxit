// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A precedence parser for modal formulas and sequents, in the syntax
//! documented on [`Sequent`]. It keeps its own stack, so a formula of any
//! depth is read without recursion.

use super::{Formulas, Node, NodeId, Sequent};
use crate::Error;
use crate::errors::ParseError;
use crate::limits::{Limits, Refusal};

/// A binary connective.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Binary {
    /// `<->` or `↔`
    Iff,
    /// `->` or `→`
    Implies,
    /// `\/` or `∨`
    Or,
    /// `/\` or `∧`
    And,
}

impl Binary {
    /// The connectives as they are written, a spelling before every
    /// spelling it is a prefix of.
    const SPELLINGS: [(&'static str, Self); 8] = [
        ("<->", Self::Iff),
        ("↔", Self::Iff),
        ("->", Self::Implies),
        ("→", Self::Implies),
        ("\\/", Self::Or),
        ("∨", Self::Or),
        ("/\\", Self::And),
        ("∧", Self::And),
    ];

    /// Returns how tightly the connective binds.
    const fn strength(self) -> u8 {
        match self {
            Self::Iff => 1,
            Self::Implies => 2,
            Self::Or => 3,
            Self::And => 4,
        }
    }

    /// Returns whether an operand between this connective and `next` is
    /// this one's: it binds tighter, or as tight and associates to the
    /// left, as the disjunctions and conjunctions do.
    const fn takes_before(self, next: Self) -> bool {
        self.strength() > next.strength()
            || (self.strength() == next.strength() && self.strength() >= 3)
    }

    /// Returns the node of the connective over two formulas.
    const fn node(self, left: NodeId, right: NodeId) -> Node {
        match self {
            Self::Iff => Node::Iff(left, right),
            Self::Implies => Node::Implies(left, right),
            Self::Or => Node::Or(left, right),
            Self::And => Node::And(left, right),
        }
    }
}

/// A prefix operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Prefix {
    /// `~` or `¬`
    Not,
    /// `[]` or `□`
    Box,
    /// `<>` or `◇`
    Diamond,
}

impl Prefix {
    /// The operators as they are written.
    const SPELLINGS: [(&'static str, Self); 6] = [
        ("~", Self::Not),
        ("¬", Self::Not),
        ("[]", Self::Box),
        ("□", Self::Box),
        ("<>", Self::Diamond),
        ("◇", Self::Diamond),
    ];

    /// Returns the node of the operator over a formula.
    const fn node(self, operand: NodeId) -> Node {
        match self {
            Self::Not => Node::Not(operand),
            Self::Box => Node::Box(operand),
            Self::Diamond => Node::Diamond(operand),
        }
    }
}

/// What waits for the operand to its right.
#[derive(Clone, Copy, Debug)]
enum Pending {
    /// `(`
    Open,
    /// A prefix operator.
    Prefix(Prefix),
    /// A binary connective with its left operand.
    Binary(Binary, NodeId),
}

/// A text in the reading.
struct Parser<'a> {
    /// The text.
    input: &'a str,
    /// The byte offset of the next character.
    at: usize,
    /// The formulas read so far.
    formulas: &'a mut Formulas,
    /// What waits for its right operand, the innermost last.
    pending: Vec<Pending>,
    /// How many parentheses are open.
    open: usize,
    /// The most formulas the arena may hold: the caller's bound on
    /// occurrences, since every formula of the arena is at least one.
    limit: Option<u64>,
}

/// What a formula's list ended with.
enum End {
    /// A comma: another formula of the same side follows.
    Comma,
    /// The turnstile.
    Turnstile,
    /// The end of the text.
    Stop,
}

impl<'a> Parser<'a> {
    /// Returns the error for the character at byte `at`, where `expected`
    /// could have stood.
    fn unexpected(&self, at: usize, expected: &'static [&'static str]) -> Error {
        Error::from(ParseError::new(self.input, at, expected))
    }

    /// Skips white space and returns the rest of the text.
    fn rest(&mut self) -> &'a str {
        let rest = &self.input[self.at..];
        self.at += rest.len() - rest.trim_start().len();
        &self.input[self.at..]
    }

    /// Consumes `token` if the text goes on with it.
    fn eat(&mut self, token: &str) -> bool {
        let found = self.rest().starts_with(token);
        if found {
            self.at += token.len();
        }
        found
    }

    /// Reads an identifier if one starts here.
    fn identifier(&mut self) -> Option<&'a str> {
        let rest = self.rest();
        let first = rest.chars().next()?;
        if !crate::linear::sequents::name::starts_identifier(first) {
            return None;
        }
        let end = rest
            .find(|c| !unicode_ident::is_xid_continue(c))
            .unwrap_or(rest.len());
        self.at += end;
        Some(&rest[..end])
    }

    /// Reads one formula up to a comma, the turnstile or the end of the
    /// text outside every parenthesis, which it says.
    fn formula(&mut self) -> Result<(NodeId, End), Error> {
        loop {
            // An operand, after any prefix operators and parentheses.
            let mut operand = loop {
                if self.eat("(") {
                    self.open += 1;
                    self.pending.push(Pending::Open);
                    // A parenthesis makes no formula, but no more than the
                    // bound may wait, so that a run of them cannot pile up.
                    count_within(self.open, self.limit)?;
                } else if let Some((_, prefix)) = Prefix::SPELLINGS
                    .into_iter()
                    .find(|(token, _)| self.eat(token))
                {
                    self.wait(Pending::Prefix(prefix))?;
                } else {
                    break self.constant_or_atom()?;
                }
            };
            // The operators after it.
            loop {
                let at = self.at;
                if let Some((_, binary)) = Binary::SPELLINGS
                    .into_iter()
                    .find(|(token, _)| self.eat(token))
                {
                    let left = self.reduce(operand, Some(binary))?;
                    self.wait(Pending::Binary(binary, left))?;
                    break;
                }
                if self.open > 0 && self.eat(")") {
                    operand = self.reduce(operand, None)?;
                    self.pending.pop();
                    self.open -= 1;
                    continue;
                }
                let end = if self.open > 0 {
                    None
                } else if self.eat(",") {
                    Some(End::Comma)
                } else if self.eat("|-") || self.eat("⊢") {
                    Some(End::Turnstile)
                } else if self.rest().is_empty() {
                    Some(End::Stop)
                } else {
                    None
                };
                let Some(end) = end else {
                    let expected: &[&str] = if self.open > 0 {
                        &["a connective", "`)`"]
                    } else {
                        &["a connective", "`,`", "`|-`", "the end"]
                    };
                    return Err(self.unexpected(self.at.max(at), expected));
                };
                return Ok((self.reduce(operand, None)?, end));
            }
        }
    }

    /// Notes an operator that waits for its right operand, and fails once
    /// more operators wait than the limit allows. Each will make a formula
    /// of its own, since all lie on one branch of the formula; without the
    /// bound a run of `[]` or of `->` would pile up without end.
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
        let at = self.at;
        if self.eat("⊤") {
            return self.add(Node::True);
        }
        if self.eat("⊥") {
            return self.add(Node::False);
        }
        match self.identifier() {
            Some("true") => self.add(Node::True),
            Some("false") => self.add(Node::False),
            Some(name) => {
                let id = self.formulas.atom(name)?;
                self.within(id)
            }
            None => Err(self.unexpected(at, &["a formula"])),
        }
    }

    /// Gives `operand` to the pending operators that take it before
    /// `next` does, or to all of them up to the innermost open parenthesis
    /// for `None`, and returns the formula they make of it.
    fn reduce(&mut self, mut operand: NodeId, next: Option<Binary>) -> Result<NodeId, Error> {
        while let Some(&top) = self.pending.last() {
            let node = match top {
                Pending::Open => break,
                Pending::Prefix(prefix) => prefix.node(operand),
                Pending::Binary(binary, left) => {
                    if next.is_some_and(|next| !binary.takes_before(next)) {
                        break;
                    }
                    binary.node(left, operand)
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

    /// Parses a sequent of modal logic, or one formula to prove, in the
    /// syntax [`Sequent`] documents, within the default limits.
    fn from_str(s: &str) -> Result<Self, Error> {
        Self::parse_within(s, &Limits::default())
    }
}

impl Sequent {
    /// Parses a sequent of modal logic, or one formula to prove, in the
    /// syntax [`Sequent`] documents. It refuses the text as soon as its
    /// distinct subformulas, its formulas, the operators waiting for an
    /// operand or the open parentheses pass `limits.occurrences`, before
    /// anything of that size is built, and a sequent that unfolds to more
    /// subformula occurrences than that.
    ///
    /// Needs the cargo feature `parse` (on by default).
    ///
    /// # Errors
    ///
    /// [`Error::Parse`] for text that is no sequent, [`Error::AtomName`]
    /// for an atom named by a keyword, and [`Refusal::Occurrences`] past
    /// the bound.
    pub fn parse_within(s: &str, limits: &Limits) -> Result<Self, Error> {
        let mut formulas = Formulas::default();
        let mut parser = Parser {
            input: s,
            at: 0,
            formulas: &mut formulas,
            pending: Vec::new(),
            open: 0,
            limit: limits.occurrences,
        };
        let (mut left, mut right) = (Vec::new(), Vec::new());
        let mut turnstile = false;
        // An empty side: the turnstile first, or nothing after it.
        if parser.eat("|-") || parser.eat("⊢") {
            turnstile = true;
        }
        if !(turnstile && parser.rest().is_empty()) {
            loop {
                let (formula, end) = parser.formula()?;
                if turnstile { &mut right } else { &mut left }.push(formula);
                count_within(left.len() + right.len(), limits.occurrences)?;
                match end {
                    End::Comma => {}
                    End::Turnstile if !turnstile => {
                        turnstile = true;
                        if parser.rest().is_empty() {
                            break;
                        }
                    }
                    End::Turnstile => {
                        // The second turnstile, `|-` or `⊢`, as one token.
                        let read = &parser.input[..parser.at];
                        let start = read.len() - if read.ends_with("|-") { 2 } else { "⊢".len() };
                        let span = start..parser.at;
                        let expected = &["a connective", "`,`", "the end"];
                        return Err(ParseError::spanning(parser.input, span, expected).into());
                    }
                    End::Stop => break,
                }
            }
        }
        if !turnstile {
            // One formula, or a list, to prove.
            std::mem::swap(&mut left, &mut right);
        }
        super::bound(&formulas, &left, &right, limits)?;
        Sequent::new(formulas, left, right)
    }
}

/// Fails once `count` formulas, of the arena or of a sequent, pass
/// `limit`: each is at least one subformula occurrence, however much of
/// the arena a formula of the sequent shares with the others.
fn count_within(count: usize, limit: Option<u64>) -> Result<(), Error> {
    match limit {
        Some(limit) if count as u64 > limit => Err(Error::Refused(Refusal::Occurrences {
            occurrences: limit.saturating_add(1),
            limit,
        })),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both spellings of every connective and prefix, the precedences and
    /// the associativities read as documented, what is written reads back
    /// as itself, and what is no sequent is refused where it goes wrong.
    #[test]
    fn reads_the_syntax() {
        let read = |text: &str| text.parse::<Sequent>().unwrap().to_string();
        assert_eq!(
            read("[]a /\\ <>b \\/ ~c -> d <-> e"),
            "⊢ (((□a ∧ ◇b) ∨ ¬c) → d) ↔ e"
        );
        assert_eq!(
            read("□a ∧ ◇b ∨ ¬c → d ↔ e"),
            read("[]a /\\ <>b \\/ ~c -> d <-> e")
        );
        assert_eq!(read("[]<>~[](a -> b)"), "⊢ □◇¬□(a → b)");
        assert_eq!(read("[] a -> [] [] a"), "⊢ □a → □□a");
        assert_eq!(read("a <-> <>a"), "⊢ a ↔ ◇a");
        assert_eq!(read("[]true, ⊥ |- <>false, ⊤"), "□⊤, ⊥ ⊢ ◇⊥, ⊤");
        assert_eq!(read("|-"), "⊢");
        assert_eq!(read("[]a |-"), "□a ⊢");
        for text in ["□(p → q) ⊢ □p → □q", "◇(a ∨ b), ¬◇a ⊢ ◇b, □(a ↔ (b ∧ c))"]
        {
            assert_eq!(read(text), text);
        }
        for bad in [
            "a & b", "a | b", "a -o b", "!a", "?a", "[a]b", "<a>b", "a [] b", "a <> b", "[]", "<>",
            "(a", "a)", "a b", "",
        ] {
            assert!(bad.parse::<Sequent>().is_err(), "{bad:?}");
        }
        let Err(Error::Parse(e)) = "[]a |- b |- c".parse::<Sequent>() else {
            panic!("a second turnstile");
        };
        assert_eq!((e.column, e.found.as_deref()), (10, Some("|-")));
        let Err(Error::Parse(e)) = "[](a".parse::<Sequent>() else {
            panic!("an open parenthesis");
        };
        assert_eq!(e.expected, ["a connective", "`)`"]);
    }

    /// The reader keeps to the occurrence bound: the arena's formulas, a
    /// sequent's formulas, what a shared formula unfolds to, and the
    /// operators and parentheses that wait.
    #[test]
    fn keeps_to_the_occurrence_bound() {
        let bound = |n| Limits::default().with_occurrences(Some(n));
        let refused = |text: &str, n| {
            matches!(
                Sequent::parse_within(text, &bound(n)),
                Err(Error::Refused(Refusal::Occurrences { .. }))
            )
        };
        // Four formulas: a, □a, ◇□a and the implication, which unfolds to
        // six occurrences, since □a stands twice.
        assert!(Sequent::parse_within("[]a -> <>[]a", &bound(6)).is_ok());
        assert!(refused("[]a -> <>[]a", 5));
        let repeated = ["[]a"; 3].join(", ");
        assert!(Sequent::parse_within(&repeated, &bound(6)).is_ok());
        assert!(refused(&repeated, 5));
        for run in ["[]".repeat(100), "a -> ".repeat(100), "(".repeat(100)] {
            assert!(refused(&format!("{run}a b"), 10), "{run}");
        }
    }
}
