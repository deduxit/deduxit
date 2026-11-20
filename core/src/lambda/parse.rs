// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The reader of a term's text: what the printer writes with types
//! shown, in Unicode or ASCII, with or without the judgement around it.
//!
//! The reader types as it reads. A pattern's variables take their types
//! from the term the pattern takes apart, so each subterm's type is
//! known when the subterm closes, by the checker's own rule for one
//! node. Nothing recurses: what is open waits on a stack.

use super::check::{Fault, Names, describe, rule};
use super::{
    Builder, Calculus, Node, SUBSCRIPTED, Term, TermId, Type, TypeId, VarId, names_variable,
};
use crate::Error;
use crate::errors::ParseError;
use crate::hash::HashMap;
use crate::limits::{Limits, Refusal};
use crate::name;

/// The keywords that end a subterm.
const ENDS: [&str; 5] = ["be", "in", "of", "for", "as"];

/// A subterm read: its node, its type, and where its text lies.
#[derive(Clone, Copy, Debug)]
struct Value {
    /// The node.
    id: TermId,
    /// The node's type.
    ty: TypeId,
    /// The byte offset its text starts at.
    start: usize,
    /// The byte offset its text ends before.
    end: usize,
}

/// A keyword applied to one term in brackets.
#[derive(Clone, Copy, Debug)]
enum Call {
    /// `fst(M)`
    First,
    /// `snd(M)`
    Second,
    /// `derelict(M)`
    Derelict,
    /// `inl_S(M)`
    Left(TypeId),
    /// `inr_S(M)`
    Right(TypeId),
    /// `abort_A(M)`
    Abort(TypeId),
}

/// A construct that waits for a subterm, with where its text starts.
#[derive(Debug)]
enum Frame {
    /// A run of applications and of `⊗`: the left operand of a `⊗` and
    /// the application read so far.
    Run {
        /// The left operand of the `⊗` being read.
        left: Option<Value>,
        /// The function applied so far.
        function: Option<Value>,
    },
    /// `(M)`
    Bracket(usize),
    /// `⟨M, N⟩` before its comma.
    PairLeft(usize),
    /// `⟨M, N⟩` after its comma.
    PairRight(usize, Value),
    /// A keyword applied to one term.
    Call(usize, Call),
    /// `true(M, …)`, or `false_A(M, …; N)` before its `;`, with the
    /// terms read and `false`'s type.
    List(usize, Vec<TermId>, Option<TypeId>),
    /// `false_A(M, …; N)` after its `;`.
    Last(usize, Vec<TermId>, TypeId),
    /// `λx:A. M`
    Lambda(usize, VarId),
    /// `let M be … in N` before `be`.
    LetTerm(usize),
    /// `let M be x ⊗ y in N` or `let M be ∗ in N` after `in`.
    LetBody(usize, Value, Option<(VarId, VarId)>),
    /// `case M of …` before `of`.
    CaseTerm(usize),
    /// `case M of inl(x) → N ‖ inr(y) → P` in `N`, with `y`'s type.
    CaseLeft(usize, Value, VarId, TypeId),
    /// The same in `P`.
    CaseRight(usize, Value, VarId, Value, VarId),
    /// `promote M, … for …` before `for`.
    PromoteTerms(usize, Vec<Value>),
    /// `promote M⃗ for x⃗ in N` after `in`.
    PromoteBody(usize, Vec<TermId>, Vec<VarId>),
    /// `discard M in N` before `in`.
    DiscardTerm(usize),
    /// `discard M in N` after `in`.
    DiscardBody(usize, Value),
    /// `copy M as x, y in N` before `as`.
    CopyTerm(usize),
    /// `copy M as x, y in N` after `in`.
    CopyBody(usize, Value, VarId, VarId),
}

/// A binary connective of a type, as written.
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
    /// `-o` or `⊸`
    Lollipop,
    /// `+` or `⊕`
    Plus,
    /// `&`
    With,
    /// `*` or `⊗`
    Tensor,
}

impl Binary {
    /// Returns how tightly the connective binds, as the sequents'
    /// parsers have it.
    const fn strength(self) -> u8 {
        match self {
            Self::Iff | Self::Lollipop => 1,
            Self::Implies | Self::Plus => 2,
            Self::Or | Self::With => 3,
            Self::And => 4,
            Self::Tensor => 5,
        }
    }

    /// Returns whether an operand between this connective and `next` is
    /// this one's: it binds tighter, or as tight and groups to the left.
    const fn takes_before(self, next: Self) -> bool {
        self.strength() > next.strength()
            || (self.strength() == next.strength()
                && !matches!(self, Self::Iff | Self::Implies | Self::Lollipop))
    }
}

/// What waits for the type to its right.
#[derive(Clone, Copy, Debug)]
enum Pending {
    /// `(`
    Open,
    /// `~` or `¬`
    Not,
    /// `!`
    Bang,
    /// A binary connective with its left operand.
    Binary(Binary, TypeId),
}

/// A text in the reading.
struct Parser<'a> {
    /// The text.
    input: &'a str,
    /// The byte offset of the next character.
    at: usize,
    /// The calculus the term is read in.
    calculus: Calculus,
    /// The term so far.
    builder: Builder,
    /// The type of every node of the builder.
    of: Vec<TypeId>,
    /// The variables in scope by name, each name's innermost last, with
    /// the node that uses it.
    scope: HashMap<Box<str>, Vec<(VarId, TermId)>>,
    /// The names of the variables in scope, the innermost last.
    bound: Vec<Box<str>>,
    /// What waits for a subterm, the innermost last.
    frames: Vec<Frame>,
    /// How many nodes, waiting constructs and type operators were made.
    made: u64,
    /// The most of them, the caller's bound on occurrences.
    limit: Option<u64>,
}

impl<'a> Parser<'a> {
    /// Returns the error for the character at byte `at`, where
    /// `expected` could have stood.
    fn unexpected(&self, at: usize, expected: &'static [&'static str]) -> Error {
        ParseError::new(self.input, at, expected).into()
    }

    /// Returns the error for a fault of the text from `start` to `end`.
    fn ill_typed(&self, fault: Fault, start: usize, end: usize) -> Error {
        let names = Names {
            types: &self.builder.types.list,
            atoms: &self.builder.atoms,
            variables: &self.builder.variables,
            calculus: self.calculus,
        };
        let offset = |at: usize| u32::try_from(at).unwrap_or(u32::MAX);
        Error::IllTyped {
            reason: describe(fault, names),
            node: None,
            span: Some(offset(start)..offset(end)),
        }
    }

    /// Counts one more thing made, and refuses past the bound.
    fn count(&mut self) -> Result<(), Error> {
        self.made += 1;
        match self.limit {
            Some(limit) if self.made > limit => Err(Error::Refused(Refusal::Occurrences {
                occurrences: limit.saturating_add(1),
                limit,
            })),
            _ => Ok(()),
        }
    }

    /// Skips white space and returns the rest of the text.
    fn rest(&mut self) -> &'a str {
        let rest = &self.input[self.at..];
        let trimmed = rest.trim_start();
        self.at += rest.len() - trimmed.len();
        trimmed
    }

    /// Skips white space, then `token` if the text goes on with it, and
    /// returns whether it did.
    fn eat(&mut self, token: &str) -> bool {
        let found = self.rest().starts_with(token);
        if found {
            self.at += token.len();
        }
        found
    }

    /// Skips one of `tokens` if the text goes on with it.
    fn eat_any(&mut self, tokens: &[&str]) -> bool {
        tokens.iter().any(|token| self.eat(token))
    }

    /// Skips `tokens`' first that the text goes on with, or fails with
    /// `expected`.
    fn expect(&mut self, tokens: &[&str], expected: &'static [&'static str]) -> Result<(), Error> {
        if self.eat_any(tokens) {
            Ok(())
        } else {
            Err(self.unexpected(self.at, expected))
        }
    }

    /// Returns the word the text goes on with, after white space: an
    /// identifier's characters. It is not skipped.
    fn word(&mut self) -> Option<&'a str> {
        let rest = self.rest();
        if !rest.chars().next().is_some_and(name::starts_identifier) {
            return None;
        }
        let end = rest
            .char_indices()
            .find(|&(_, c)| !unicode_ident::is_xid_continue(c))
            .map_or(rest.len(), |(i, _)| i);
        Some(&rest[..end])
    }

    /// Skips the keyword `keyword` if the text goes on with that word,
    /// and returns whether it did.
    fn keyword(&mut self, keyword: &str) -> bool {
        let found = self.word() == Some(keyword);
        if found {
            self.at += keyword.len();
        }
        found
    }

    /// Skips the keyword `keyword`, or fails with `expected`.
    fn expect_keyword(
        &mut self,
        keyword: &str,
        expected: &'static [&'static str],
    ) -> Result<(), Error> {
        if self.keyword(keyword) {
            Ok(())
        } else {
            Err(self.unexpected(self.at, expected))
        }
    }

    /// Reads a variable's name: an identifier that is no keyword, and
    /// the subscript digits after it, which join the name as digits.
    fn name(&mut self) -> Result<Box<str>, Error> {
        const EXPECTED: &[&str] = &["a variable"];
        let word = self
            .word()
            .ok_or_else(|| self.unexpected(self.at, EXPECTED))?;
        let reserved = !names_variable(word);
        if reserved {
            let span = self.at..self.at + word.len();
            return Err(ParseError::spanning(self.input, span, EXPECTED).into());
        }
        self.at += word.len();
        let mut name = name::normalized(word).into_owned();
        let rest = &self.input[self.at..];
        for c in rest.chars() {
            let Some(digit) = (c as u32).checked_sub(0x2080).filter(|&d| d < 10) else {
                break;
            };
            name.push(char::from_digit(digit, 10).unwrap_or('0'));
            self.at += c.len_utf8();
        }
        Ok(name.into())
    }

    /// Makes a variable of a type and a name, with the node that uses it.
    fn variable(&mut self, ty: TypeId, name: &str) -> Result<(VarId, TermId), Error> {
        self.count()?;
        let made = self.builder.named_variable(ty, name);
        self.of.push(ty);
        Ok(made)
    }

    /// Makes a variable and puts it in scope.
    fn bind(&mut self, ty: TypeId, name: Box<str>) -> Result<VarId, Error> {
        let (variable, node) = self.variable(ty, &name)?;
        self.scope
            .entry(name.clone())
            .or_default()
            .push((variable, node));
        self.bound.push(name);
        Ok(variable)
    }

    /// Makes a node whose text lies from `start` to the place read, or
    /// says why it has no type.
    fn make(&mut self, node: Node, start: usize) -> Result<Value, Error> {
        self.count()?;
        let (variables, of) = (&self.builder.variables, &self.of);
        let ty = rule(
            &node,
            self.calculus,
            &mut self.builder.types,
            |v| variables[v.index()].ty(),
            |t| of[t.index()],
        )
        .map_err(|fault| self.ill_typed(fault, start, self.at))?;
        let id = self.builder.node(node);
        self.of.push(ty);
        Ok(Value {
            id,
            ty,
            start,
            end: self.at,
        })
    }

    /// Makes a construct wait for a subterm.
    fn wait(&mut self, frame: Frame) -> Result<(), Error> {
        self.count()?;
        self.frames.push(frame);
        Ok(())
    }

    /// Makes a construct wait for a subterm that is a run of its own.
    fn open(&mut self, frame: Frame) -> Result<(), Error> {
        self.wait(frame)?;
        self.wait(Frame::Run {
            left: None,
            function: None,
        })
    }

    /// Refuses, in the simple calculus, a construct of the linear ones
    /// whose text starts at `start`.
    fn linear_only(&self, what: &'static str, start: usize) -> Result<(), Error> {
        if self.calculus == Calculus::Simple {
            Err(self.ill_typed(Fault::Foreign { what }, start, self.at))
        } else {
            Ok(())
        }
    }

    /// Returns the fault of a term whose type does not fit its place.
    fn misshapen(&self, of: &'static str, needs: &'static str, term: Value) -> Error {
        let found = term.ty;
        self.ill_typed(Fault::Shape { of, needs, found }, term.start, term.end)
    }

    /// Returns whether the text goes on with a subterm.
    fn starts_operand(&mut self) -> bool {
        match self.word() {
            Some(word) => !ENDS.contains(&word),
            None => self.rest().starts_with(['(', '∗', '⟨', '<', '\\']),
        }
    }

    /// Reads the start of a subterm: a whole one where it is a variable
    /// or a constant, and otherwise what must be read first, which is
    /// left waiting.
    #[expect(clippy::too_many_lines, reason = "one arm per form of the syntax")]
    fn operand(&mut self) -> Result<Option<Value>, Error> {
        const TERM: &[&str] = &["a term"];
        const OPEN: &[&str] = &["`(`"];
        let start = {
            self.rest();
            self.at
        };
        let Some(word) = self.word() else {
            if self.eat_any(&["()", "∗"]) {
                self.linear_only("the unit `∗`", start)?;
                return self.make(Node::Unit, start).map(Some);
            }
            if self.eat_any(&["⟨⟩", "<>"]) {
                let context = Box::default();
                return self.make(Node::True { context }, start).map(Some);
            }
            if self.eat("(") {
                self.open(Frame::Bracket(start))?;
            } else if self.eat_any(&["⟨", "<"]) {
                self.open(Frame::PairLeft(start))?;
            } else if self.eat("\\") {
                self.lambda(start)?;
            } else {
                return Err(self.unexpected(self.at, TERM));
            }
            return Ok(None);
        };
        if word.starts_with('λ') {
            self.at += 'λ'.len_utf8();
            self.lambda(start)?;
            return Ok(None);
        }
        let subscripted = SUBSCRIPTED.iter().find_map(|&k| {
            let rest = word.strip_prefix(k)?;
            rest.strip_prefix('_').map(|ty| (k, ty))
        });
        if let Some((keyword, written)) = subscripted {
            self.at += keyword.len() + 1;
            let ty = if written.is_empty() {
                self.ty(true)?
            } else {
                let ty = self.constant_or_atom(written, self.at)?;
                self.at += written.len();
                ty
            };
            self.expect(&["("], OPEN)?;
            match keyword {
                "inl" => self.open(Frame::Call(start, Call::Left(ty)))?,
                "inr" => self.open(Frame::Call(start, Call::Right(ty)))?,
                "abort" => self.open(Frame::Call(start, Call::Abort(ty)))?,
                _ if self.eat(";") => self.open(Frame::Last(start, Vec::new(), ty))?,
                _ => self.open(Frame::List(start, Vec::new(), Some(ty)))?,
            }
            return Ok(None);
        }
        match word {
            "inl" | "inr" | "abort" | "false" => {
                let error = ParseError::new(
                    self.input,
                    self.at + word.len(),
                    &["`_` and the type the term has"],
                );
                return Err(error
                    .with_hint("write the type as a subscript, as in inl_(a ∨ b)(x) or abort_a(x)")
                    .into());
            }
            "fst" | "snd" | "derelict" => {
                self.at += word.len();
                self.expect(&["("], OPEN)?;
                let call = match word {
                    "fst" => Call::First,
                    "snd" => Call::Second,
                    _ => {
                        self.linear_only("a `derelict`", start)?;
                        Call::Derelict
                    }
                };
                self.open(Frame::Call(start, call))?;
            }
            "true" => {
                self.at += word.len();
                self.expect(&["("], OPEN)?;
                if self.eat(")") {
                    let context = Box::default();
                    return self.make(Node::True { context }, start).map(Some);
                }
                self.open(Frame::List(start, Vec::new(), None))?;
            }
            "let" => {
                self.at += word.len();
                self.linear_only("a `let`", start)?;
                self.open(Frame::LetTerm(start))?;
            }
            "case" => {
                self.at += word.len();
                self.open(Frame::CaseTerm(start))?;
            }
            "promote" => {
                self.at += word.len();
                self.linear_only("a `promote`", start)?;
                if self.keyword("for") {
                    self.promoted(start, Vec::new())?;
                } else {
                    self.open(Frame::PromoteTerms(start, Vec::new()))?;
                }
            }
            "discard" => {
                self.at += word.len();
                self.linear_only("a `discard`", start)?;
                self.open(Frame::DiscardTerm(start))?;
            }
            "copy" => {
                self.at += word.len();
                self.linear_only("a `copy`", start)?;
                self.open(Frame::CopyTerm(start))?;
            }
            _ if ENDS.contains(&word) => return Err(self.unexpected(self.at, TERM)),
            _ => {
                let name = self.name()?;
                let Some(&(variable, id)) = self.scope.get(&name).and_then(|known| known.last())
                else {
                    return Err(ParseError::spanning(
                        self.input,
                        start..self.at,
                        &["a variable that a binder or the context declares"],
                    )
                    .into());
                };
                let ty = self.builder.variables[variable.index()].ty();
                let end = self.at;
                return Ok(Some(Value { id, ty, start, end }));
            }
        }
        Ok(None)
    }

    /// Reads `x:A.` after a `λ` whose text starts at `start`, and leaves
    /// the binder waiting for its body.
    fn lambda(&mut self, start: usize) -> Result<(), Error> {
        let name = self.name()?;
        if !self.eat(":") {
            return Err(
                ParseError::new(self.input, self.at, &["`:` and the variable's type"])
                    .with_hint("write the variable's type, as in λx:a. x; types are not inferred")
                    .into(),
            );
        }
        let ty = self.ty(false)?;
        self.expect(&["."], &["a connective", "`.`"])?;
        let variable = self.bind(ty, name)?;
        self.open(Frame::Lambda(start, variable))
    }

    /// Reads `x, … in` after a `promote`'s `for`, and leaves the
    /// `promote` waiting for its body.
    fn promoted(&mut self, start: usize, terms: Vec<Value>) -> Result<(), Error> {
        const IN: &[&str] = &["`,`", "`in`"];
        let mut names = Vec::new();
        if !self.keyword("in") {
            loop {
                self.count()?;
                names.push(self.name()?);
                if !self.eat(",") {
                    self.expect_keyword("in", IN)?;
                    break;
                }
            }
        }
        if names.len() != terms.len() {
            let fault = Fault::Promoted {
                terms: terms.len(),
                variables: names.len(),
            };
            return Err(self.ill_typed(fault, start, self.at));
        }
        for &term in &terms {
            if !matches!(self.builder.types.get(term.ty), Type::Bang { .. }) {
                return Err(self.misshapen("a term a `promote` takes", "a type !A", term));
            }
        }
        let mut variables = Vec::with_capacity(names.len());
        for (name, term) in names.into_iter().zip(&terms) {
            variables.push(self.bind(term.ty, name)?);
        }
        let terms = terms.iter().map(|term| term.id).collect();
        self.open(Frame::PromoteBody(start, terms, variables))
    }

    /// Reads `inl(x) →` or `inr(x) →` of a case, and returns the name.
    fn branch(
        &mut self,
        keyword: &str,
        expected: &'static [&'static str],
    ) -> Result<Box<str>, Error> {
        self.expect_keyword(keyword, expected)?;
        self.expect(&["("], &["`(`"])?;
        let name = self.name()?;
        self.expect(&[")"], &["`)`"])?;
        self.expect(&["→", "->"], &["`→`"])?;
        Ok(name)
    }

    /// Hands a subterm read to the construct that waited for it. Returns
    /// the construct's own term where the subterm ended it.
    #[expect(clippy::too_many_lines, reason = "one arm per form of the syntax")]
    fn deliver(&mut self, frame: Frame, value: Value) -> Result<Option<Value>, Error> {
        const CLOSE: &[&str] = &["`)`"];
        const IN: &[&str] = &["`in`"];
        let unscoped = |parser: &mut Self, variables: usize| {
            for _ in 0..variables {
                let name = parser.bound.pop().expect("a binder's variable is in scope");
                if let Some(known) = parser.scope.get_mut(&name) {
                    known.pop();
                }
            }
        };
        match frame {
            Frame::Run { left, function } => {
                let applied = match function {
                    Some(function) => {
                        let node = Node::Apply {
                            function: function.id,
                            argument: value.id,
                        };
                        self.make(node, function.start)?
                    }
                    None => value,
                };
                if self.starts_operand() {
                    let function = Some(applied);
                    self.frames.push(Frame::Run { left, function });
                    return Ok(None);
                }
                let whole = match left {
                    Some(left) => {
                        self.linear_only("a pair `M ⊗ N`", left.start)?;
                        let node = Node::Tensor {
                            left: left.id,
                            right: applied.id,
                        };
                        self.make(node, left.start)?
                    }
                    None => applied,
                };
                if self.eat_any(&["⊗", "*"]) {
                    self.frames.push(Frame::Run {
                        left: Some(whole),
                        function: None,
                    });
                    return Ok(None);
                }
                Ok(Some(whole))
            }
            Frame::Bracket(start) => {
                self.expect(&[")"], CLOSE)?;
                let end = self.at;
                Ok(Some(Value {
                    start,
                    end,
                    ..value
                }))
            }
            Frame::PairLeft(start) => {
                self.expect(&[","], &["`,`"])?;
                self.open(Frame::PairRight(start, value))?;
                Ok(None)
            }
            Frame::PairRight(start, left) => {
                self.expect(&["⟩", ">"], &["`⟩`"])?;
                let node = Node::Pair {
                    left: left.id,
                    right: value.id,
                };
                self.make(node, start).map(Some)
            }
            Frame::Call(start, call) => {
                self.expect(&[")"], CLOSE)?;
                let term = value.id;
                let node = match call {
                    Call::First => Node::First { term },
                    Call::Second => Node::Second { term },
                    Call::Derelict => Node::Derelict { term },
                    Call::Left(sum) => Node::Left { term, sum },
                    Call::Right(sum) => Node::Right { term, sum },
                    Call::Abort(goal) => Node::False {
                        context: Box::default(),
                        term,
                        goal,
                    },
                };
                self.make(node, start).map(Some)
            }
            Frame::List(start, mut terms, goal) => {
                terms.push(value.id);
                if self.eat(",") {
                    self.open(Frame::List(start, terms, goal))?;
                    return Ok(None);
                }
                match goal {
                    Some(goal) => {
                        self.expect(&[";"], &["`,`", "`;`"])?;
                        self.open(Frame::Last(start, terms, goal))?;
                        Ok(None)
                    }
                    None => {
                        self.expect(&[")"], &["`,`", "`)`"])?;
                        let context = terms.into();
                        self.make(Node::True { context }, start).map(Some)
                    }
                }
            }
            Frame::Last(start, terms, goal) => {
                self.expect(&[")"], CLOSE)?;
                let node = Node::False {
                    context: terms.into(),
                    term: value.id,
                    goal,
                };
                self.make(node, start).map(Some)
            }
            Frame::Lambda(start, variable) => {
                unscoped(self, 1);
                let body = value.id;
                self.make(Node::Lambda { variable, body }, start).map(Some)
            }
            Frame::LetTerm(start) => {
                self.expect_keyword("be", &["`be`"])?;
                if self.eat_any(&["()", "∗"]) {
                    self.expect_keyword("in", IN)?;
                    self.open(Frame::LetBody(start, value, None))?;
                    return Ok(None);
                }
                let left = self.name()?;
                self.expect(&["⊗", "*"], &["`⊗`"])?;
                let right = self.name()?;
                self.expect_keyword("in", IN)?;
                let Type::Tensor { left: a, right: b } = self.builder.types.get(value.ty) else {
                    return Err(self.misshapen(
                        "the term a `let` takes apart",
                        "a pair type A ⊗ B",
                        value,
                    ));
                };
                let pattern = (self.bind(a, left)?, self.bind(b, right)?);
                self.open(Frame::LetBody(start, value, Some(pattern)))?;
                Ok(None)
            }
            Frame::LetBody(start, term, pattern) => {
                let (term, body) = (term.id, value.id);
                let node = match pattern {
                    Some((left, right)) => {
                        unscoped(self, 2);
                        Node::LetTensor {
                            term,
                            left,
                            right,
                            body,
                        }
                    }
                    None => Node::LetUnit { term, body },
                };
                self.make(node, start).map(Some)
            }
            Frame::CaseTerm(start) => {
                self.expect_keyword("of", &["`of`"])?;
                let name = self.branch("inl", &["`inl`"])?;
                let Type::Plus { left, right } = self.builder.types.get(value.ty) else {
                    return Err(self.misshapen(
                        "the term a `case` takes apart",
                        "a sum type",
                        value,
                    ));
                };
                let variable = self.bind(left, name)?;
                self.open(Frame::CaseLeft(start, value, variable, right))?;
                Ok(None)
            }
            Frame::CaseLeft(start, term, left, right) => {
                if !self.eat("‖") && (self.rest().starts_with("|-") || !self.eat("|")) {
                    return Err(self.unexpected(self.at, &["`‖`"]));
                }
                let name = self.branch("inr", &["`inr`"])?;
                unscoped(self, 1);
                let right = self.bind(right, name)?;
                self.open(Frame::CaseRight(start, term, left, value, right))?;
                Ok(None)
            }
            Frame::CaseRight(start, term, left, left_body, right) => {
                unscoped(self, 1);
                let node = Node::Case {
                    term: term.id,
                    left,
                    left_body: left_body.id,
                    right,
                    right_body: value.id,
                };
                self.make(node, start).map(Some)
            }
            Frame::PromoteTerms(start, mut terms) => {
                terms.push(value);
                if self.eat(",") {
                    self.open(Frame::PromoteTerms(start, terms))?;
                } else {
                    self.expect_keyword("for", &["`,`", "`for`"])?;
                    self.promoted(start, terms)?;
                }
                Ok(None)
            }
            Frame::PromoteBody(start, terms, variables) => {
                unscoped(self, variables.len());
                let node = Node::Promote {
                    terms: terms.into(),
                    variables: variables.into(),
                    body: value.id,
                };
                self.make(node, start).map(Some)
            }
            Frame::DiscardTerm(start) => {
                self.expect_keyword("in", IN)?;
                self.open(Frame::DiscardBody(start, value))?;
                Ok(None)
            }
            Frame::DiscardBody(start, term) => {
                let node = Node::Discard {
                    term: term.id,
                    body: value.id,
                };
                self.make(node, start).map(Some)
            }
            Frame::CopyTerm(start) => {
                self.expect_keyword("as", &["`as`"])?;
                let left = self.name()?;
                self.expect(&[","], &["`,`"])?;
                let right = self.name()?;
                self.expect_keyword("in", IN)?;
                if !matches!(self.builder.types.get(value.ty), Type::Bang { .. }) {
                    return Err(self.misshapen("the term a `copy` takes", "a type !A", value));
                }
                let (left, right) = (self.bind(value.ty, left)?, self.bind(value.ty, right)?);
                self.open(Frame::CopyBody(start, value, left, right))?;
                Ok(None)
            }
            Frame::CopyBody(start, term, left, right) => {
                unscoped(self, 2);
                let node = Node::Copy {
                    term: term.id,
                    left,
                    right,
                    body: value.id,
                };
                self.make(node, start).map(Some)
            }
        }
    }

    /// Reads a term.
    fn term(&mut self) -> Result<Value, Error> {
        self.wait(Frame::Run {
            left: None,
            function: None,
        })?;
        let mut value = None;
        loop {
            value = match value {
                None => self.operand()?,
                Some(value) => {
                    let Some(frame) = self.frames.pop() else {
                        return Ok(value);
                    };
                    self.deliver(frame, value)?
                }
            };
        }
    }

    /// Returns the type that `word` names where a type stands at byte
    /// `at`: a constant of the calculus, or an atom.
    fn constant_or_atom(&mut self, word: &str, at: usize) -> Result<TypeId, Error> {
        let simple = self.calculus == Calculus::Simple;
        let ty = match word {
            "true" if simple => Type::Top,
            "false" if simple => Type::Zero,
            "top" if !simple => Type::Top,
            "1" if !simple => Type::One,
            "0" if !simple => Type::Zero,
            _ => {
                let known = if simple {
                    name::check_ordinary(word)
                } else {
                    name::check(word)
                };
                if known.is_err() {
                    let span = at..at + word.len();
                    return Err(ParseError::spanning(self.input, span, &["a type"]).into());
                }
                let atom = self.builder.atom(&name::normalized(word));
                Type::Atom { atom }
            }
        };
        self.count()?;
        Ok(self.builder.ty(ty))
    }

    /// Reads a type's constant or atom.
    fn primary(&mut self) -> Result<TypeId, Error> {
        let simple = self.calculus == Calculus::Simple;
        if let Some(word) = self.word() {
            let at = self.at;
            self.at += word.len();
            return self.constant_or_atom(word, at);
        }
        let constant = if self.eat("⊤") {
            Type::Top
        } else if simple && self.eat("⊥") {
            Type::Zero
        } else if !simple && self.eat("1") {
            Type::One
        } else if !simple && self.eat("0") {
            Type::Zero
        } else {
            return Err(self.unexpected(self.at, &["a type"]));
        };
        self.count()?;
        Ok(self.builder.ty(constant))
    }

    /// Returns the type a binary connective makes of its operands.
    fn binary(&mut self, binary: Binary, left: TypeId, right: TypeId) -> TypeId {
        let ty = match binary {
            Binary::Iff => {
                let there = self.builder.ty(Type::Lollipop {
                    from: left,
                    to: right,
                });
                let back = self.builder.ty(Type::Lollipop {
                    from: right,
                    to: left,
                });
                Type::With {
                    left: there,
                    right: back,
                }
            }
            Binary::Implies | Binary::Lollipop => Type::Lollipop {
                from: left,
                to: right,
            },
            Binary::Or | Binary::Plus => Type::Plus { left, right },
            Binary::And | Binary::With => Type::With { left, right },
            Binary::Tensor => Type::Tensor { left, right },
        };
        self.builder.ty(ty)
    }

    /// Reads a type, up to the first character that cannot go on with
    /// it. With `prefix`, it reads no binary connective outside
    /// brackets: the type after a subscript's `_`.
    fn ty(&mut self, prefix: bool) -> Result<TypeId, Error> {
        let simple = self.calculus == Calculus::Simple;
        let binaries: &[(&str, Binary)] = if simple {
            &[
                ("<->", Binary::Iff),
                ("↔", Binary::Iff),
                ("->", Binary::Implies),
                ("→", Binary::Implies),
                ("\\/", Binary::Or),
                ("∨", Binary::Or),
                ("/\\", Binary::And),
                ("∧", Binary::And),
            ]
        } else {
            &[
                ("-o", Binary::Lollipop),
                ("⊸", Binary::Lollipop),
                ("*", Binary::Tensor),
                ("⊗", Binary::Tensor),
                ("&", Binary::With),
                ("+", Binary::Plus),
                ("⊕", Binary::Plus),
            ]
        };
        let mut pending: Vec<Pending> = Vec::new();
        let mut open = 0usize;
        loop {
            // An operand, after its prefix operators and brackets.
            let mut operand = loop {
                self.count()?;
                if self.eat("(") {
                    open += 1;
                    pending.push(Pending::Open);
                } else if simple && self.eat_any(&["~", "¬"]) {
                    pending.push(Pending::Not);
                } else if !simple && self.eat("!") {
                    pending.push(Pending::Bang);
                } else {
                    break self.primary()?;
                }
            };
            // What follows it.
            loop {
                while let Some(&(Pending::Not | Pending::Bang)) = pending.last() {
                    operand = match pending.pop() {
                        Some(Pending::Not) => {
                            let to = self.builder.ty(Type::Zero);
                            self.builder.ty(Type::Lollipop { from: operand, to })
                        }
                        _ => self.builder.ty(Type::Bang { of: operand }),
                    };
                }
                if prefix && open == 0 {
                    return Ok(operand);
                }
                let next = binaries
                    .iter()
                    .find(|(token, _)| self.eat(token))
                    .map(|&(_, binary)| binary);
                let closes = next.is_none() && open > 0 && self.eat(")");
                // The connectives that take the operand before what
                // follows: those that bind tighter than the next one, or
                // all up to the bracket that closes.
                while let Some(&Pending::Binary(binary, left)) = pending.last() {
                    if next.is_some_and(|next| !binary.takes_before(next)) {
                        break;
                    }
                    pending.pop();
                    operand = self.binary(binary, left, operand);
                }
                if let Some(next) = next {
                    pending.push(Pending::Binary(next, operand));
                    break;
                }
                if closes {
                    pending.pop();
                    open -= 1;
                    continue;
                }
                if open > 0 {
                    return Err(self.unexpected(self.at, &["a connective", "`)`"]));
                }
                return Ok(operand);
            }
        }
    }

    /// Reads a judgement, a term with its type, or a term alone, and
    /// returns the term.
    fn judgement(mut self) -> Result<Term, Error> {
        const TURNSTILE: &[&str] = &["⊢", "|-"];
        let mut context = Vec::new();
        if TURNSTILE.iter().any(|token| self.input.contains(token)) && !self.eat_any(TURNSTILE) {
            loop {
                let name = self.name()?;
                self.expect(&[":"], &["`:`"])?;
                let ty = self.ty(false)?;
                context.push(self.bind(ty, name)?);
                if !self.eat(",") {
                    self.expect(TURNSTILE, &["a connective", "`,`", "`⊢`"])?;
                    break;
                }
            }
        }
        let root = self.term()?;
        let goal = if self.eat(":") {
            let start = self.at;
            let goal = self.ty(false)?;
            if goal != root.ty {
                let found = root.ty;
                return Err(self.ill_typed(Fault::Goal { found, goal }, start, self.at));
            }
            goal
        } else {
            root.ty
        };
        if !self.rest().is_empty() {
            return Err(self.unexpected(self.at, &["the end"]));
        }
        Ok(self.builder.finish(self.calculus, context, goal, root.id))
    }
}

impl Term {
    /// Reads a term of `calculus` from its text: what
    /// [`write`](Self::write) and [`write_judgement`](Self::write_judgement)
    /// write with [`Options::types`](super::Options::types) set, in
    /// Unicode or ASCII.
    ///
    /// # The syntax
    ///
    /// The text is a term, a term with its type (`M : A`), or a
    /// judgement with the free variables' types before `⊢`
    /// (`x : A, y : B ⊢ M : C`).
    ///
    /// | term | Unicode | ASCII |
    /// |---|---|---|
    /// | a function | `λx:A. M` | `\x:A. M` |
    /// | an application | `M N` | `M N` |
    /// | a pair, its parts | `⟨M, N⟩`, `fst(M)`, `snd(M)` | `<M, N>` |
    /// | a choice | `inl_(A ∨ B)(M)`, `inr_(A ∨ B)(M)` | `inl_(A \/ B)(M)` |
    /// | its use | `case M of inl(x) → N ‖ inr(y) → P` | `… -> N \| inr(y) -> P` |
    /// | the unit of the simple calculus | `⟨⟩` | `<>` |
    /// | from a term of type `⊥` | `abort_A(M)` | `abort_A(M)` |
    ///
    /// Application groups to the left, and a `λ`'s body runs as far to
    /// the right as it can. Every `λ` says its variable's type, and
    /// `inl`, `inr` and `abort` the type they give, since no type is
    /// inferred. A type after `_` is an atom or a constant, with `!`
    /// before it or not, or a type in brackets.
    ///
    /// The linear and the affine calculus have, besides, `M ⊗ N` (`M *
    /// N`), `let M be x ⊗ y in N`, `∗` (`()`), `let M be ∗ in N`,
    /// `true(M, …)`, `false_A(M, …; N)`, `promote M, … for x, … in N`,
    /// `derelict(M)`, `discard M in N` and `copy M as x, y in N`.
    ///
    /// A type is written as a formula of the calculus's logic: in the
    /// simple calculus as
    /// [`ordinary::Sequent`](crate::ordinary::Sequent) reads one, with
    /// `¬A` for `A → ⊥` and `A ↔ B` for `(A → B) ∧ (B → A)`; in the
    /// other two with `⊸` (`-o`), `⊗` (`*`), `&`, `⊕` (`+`), `!`, `1`,
    /// `0` and `⊤` (`top`).
    ///
    /// A variable's name is an identifier that is no keyword of the
    /// syntax; digits at its end are an index, which may be written as
    /// subscripts (`x₁` is `x1`). A name stands for its innermost
    /// binder's variable.
    ///
    /// # What is checked
    ///
    /// The types fit: the text's term has a type by the rules, and a
    /// type written after it is that type. How often a variable is used
    /// is not checked here: [`check`](Self::check) does both.
    ///
    #[cfg_attr(feature = "parse", doc = "```")]
    #[cfg_attr(not(feature = "parse"), doc = "```ignore")]
    /// use deduxit::Limits;
    /// use deduxit::lambda::{Calculus, Options, Term};
    ///
    /// let limits = Limits::default();
    /// let term = Term::parse("x : a, f : a → b ⊢ f x", Calculus::Simple, &limits)?;
    ///
    /// let mut judgement = String::new();
    /// term.write_judgement(&mut judgement, &Options::default(), &limits, |_| false)?;
    /// assert_eq!(judgement, "x : a, f : a → b ⊢ f x : b");
    /// # Ok::<(), deduxit::Error>(())
    /// ```
    ///
    /// Needs the cargo feature `parse` (on by default).
    ///
    /// # Errors
    ///
    /// - [`Error::Parse`] for text that is no term, with the place.
    /// - [`Error::IllTyped`] for a term whose types do not fit, with the
    ///   reason and the place.
    /// - [`Refusal::Occurrences`] for a text of more nodes, open
    ///   constructs and type connectives than `limits.occurrences`.
    pub fn parse(text: &str, calculus: Calculus, limits: &Limits) -> Result<Self, Error> {
        Parser {
            input: text,
            at: 0,
            calculus,
            builder: Builder::new(),
            of: Vec::new(),
            scope: HashMap::default(),
            bound: Vec::new(),
            frames: Vec::new(),
            made: 0,
            limit: limits.occurrences,
        }
        .judgement()
    }
}
