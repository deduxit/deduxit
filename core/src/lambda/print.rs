// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The one printer of terms: a table of spellings per target, the
//! Unicode and ASCII texts and one per export, and a walk with a stack of
//! its own that writes a term, or its typing judgement, after an upper
//! bound of its length has passed the output bound.
//!
//! A node several others name is written at each place (the printed term
//! is the tree the DAG unfolds to), so the text can be longer than the
//! derivation by any factor; the bound is computed over the DAG, each
//! node once, before anything is written.

use super::{Calculus, Node, Notation, Options, Term, TermId, Type, TypeId, VarId};
use crate::Error;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::sequents::notation::Atoms;
use std::fmt::Write;

/// The spellings of a type's connectives in one calculus.
#[derive(Debug)]
pub(crate) struct Connectives {
    /// `⊗`, between spaces.
    tensor: &'static str,
    /// `&` or `∧`, between spaces.
    with: &'static str,
    /// `⊕` or `∨`, between spaces.
    plus: &'static str,
    /// `⊸` or `→`, between spaces.
    lollipop: &'static str,
    /// `!`, before its operand.
    bang: &'static str,
    /// `1`
    one: &'static str,
    /// `⊤`
    top: &'static str,
    /// `0` or `⊥`
    zero: &'static str,
}

/// How a target writes terms: every symbol and keyword, a variable's
/// index, and the types' connectives in each calculus.
#[derive(Debug)]
pub(crate) struct Spelling {
    /// `λ`
    lambda: &'static str,
    /// What stands between a binder's variable and its body.
    dot: &'static str,
    /// What stands between a variable and its type in a binder.
    colon: &'static str,
    /// What stands between a variable and its type in a judgement.
    has_type: &'static str,
    /// `⊢`, between spaces.
    turnstile: &'static str,
    /// `⊗`, between spaces.
    tensor: &'static str,
    /// `∗`
    unit: &'static str,
    /// `⟨⟩`, the simple calculus's unit.
    empty_pair: &'static str,
    /// `⟨`
    open_pair: &'static str,
    /// `⟩`
    close_pair: &'static str,
    /// `→` of a case's branch, between spaces.
    arrow: &'static str,
    /// `‖` between a case's branches, between spaces.
    bar: &'static str,
    /// What stands between two words of a term.
    space: &'static str,
    /// What stands between a function and its argument.
    apply: &'static str,
    /// What a keyword is written between.
    keyword: (&'static str, &'static str),
    /// What a subscript (`false`'s type) is written between.
    subscript: (&'static str, &'static str),
    /// How a variable's index is written.
    index: Index,
    /// How an atom's name is written.
    atom: Atoms,
    /// The connectives of the linear calculi.
    linear: Connectives,
    /// The connectives of the simple calculus.
    simple: Connectives,
}

/// How a target writes a variable's index after its letter.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Index {
    /// As subscript digits, `x₁`.
    Subscript,
    /// As digits, `x1`.
    Digits,
    /// Between the marks given, `x_{1}`.
    #[cfg(any(feature = "latex", feature = "typst"))]
    Marked(&'static str, &'static str),
}

/// The Unicode spelling, which the command and `Display`-like texts
/// write.
pub(crate) const UNICODE: Spelling = Spelling {
    lambda: "λ",
    dot: ". ",
    colon: ":",
    has_type: " : ",
    turnstile: " ⊢ ",
    tensor: " ⊗ ",
    unit: "∗",
    empty_pair: "⟨⟩",
    open_pair: "⟨",
    close_pair: "⟩",
    arrow: " → ",
    bar: " ‖ ",
    space: " ",
    apply: " ",
    keyword: ("", ""),
    subscript: ("_", ""),
    index: Index::Subscript,
    atom: Atoms::Plain,
    linear: Connectives {
        tensor: " ⊗ ",
        with: " & ",
        plus: " ⊕ ",
        lollipop: " ⊸ ",
        bang: "!",
        one: "1",
        top: "⊤",
        zero: "0",
    },
    simple: SIMPLE_UNICODE,
};

/// The ordinary connectives in Unicode.
const SIMPLE_UNICODE: Connectives = Connectives {
    tensor: " ⊗ ",
    with: " ∧ ",
    plus: " ∨ ",
    lollipop: " → ",
    bang: "!",
    one: "1",
    top: "⊤",
    zero: "⊥",
};

/// The ASCII spelling, the types as the parsers read them.
pub(crate) const ASCII: Spelling = Spelling {
    lambda: "\\",
    dot: ". ",
    colon: ":",
    has_type: " : ",
    turnstile: " |- ",
    tensor: " * ",
    unit: "()",
    empty_pair: "<>",
    open_pair: "<",
    close_pair: ">",
    arrow: " -> ",
    bar: " | ",
    space: " ",
    apply: " ",
    keyword: ("", ""),
    subscript: ("_", ""),
    index: Index::Digits,
    atom: Atoms::Plain,
    linear: Connectives {
        tensor: " * ",
        with: " & ",
        plus: " + ",
        lollipop: " -o ",
        bang: "!",
        one: "1",
        top: "top",
        zero: "0",
    },
    simple: Connectives {
        tensor: " * ",
        with: " /\\ ",
        plus: " \\/ ",
        lollipop: " -> ",
        bang: "!",
        one: "1",
        top: "true",
        zero: "false",
    },
};

/// The LaTeX spelling, in math mode, with the connectives of `cmll` and
/// `amssymb` as the formulas have them.
#[cfg(feature = "latex")]
pub(crate) const LATEX: Spelling = Spelling {
    lambda: r"\lambda ",
    dot: r".\,",
    colon: r"{:}",
    has_type: r" : ",
    turnstile: r" \vdash ",
    tensor: r" \otimes ",
    unit: r"\ast",
    empty_pair: r"\langle\rangle",
    open_pair: r"\langle ",
    close_pair: r"\rangle",
    arrow: r" \to ",
    bar: r" \parallel ",
    space: r"\ ",
    apply: r"\ ",
    keyword: (r"\mathsf{", "}"),
    subscript: ("_{", "}"),
    index: Index::Marked("_{", "}"),
    atom: Atoms::Latex,
    linear: Connectives {
        tensor: r" \otimes ",
        with: r" \with ",
        plus: r" \oplus ",
        lollipop: r" \multimap ",
        bang: r"\oc ",
        one: r"\mathbf{1}",
        top: r"\top",
        zero: "0",
    },
    simple: Connectives {
        tensor: r" \otimes ",
        with: r" \land ",
        plus: r" \lor ",
        lollipop: r" \to ",
        bang: r"\oc ",
        one: r"\mathbf{1}",
        top: r"\top",
        zero: r"\bot",
    },
};

/// The Typst spelling, in math mode: the symbols as their Unicode
/// characters, a keyword as a string.
#[cfg(feature = "typst")]
pub(crate) const TYPST: Spelling = Spelling {
    lambda: "lambda ",
    dot: ". ",
    colon: ":",
    has_type: " : ",
    turnstile: " ⊢ ",
    tensor: " ⊗ ",
    unit: "∗",
    empty_pair: "⟨⟩",
    open_pair: "⟨",
    close_pair: "⟩",
    arrow: " → ",
    bar: " ‖ ",
    space: " space ",
    apply: " space ",
    keyword: ("\"", "\""),
    subscript: ("_(", ")"),
    index: Index::Marked("_", ""),
    atom: Atoms::Typst,
    linear: Connectives {
        tensor: " ⊗ ",
        with: " \\& ",
        plus: " ⊕ ",
        lollipop: " ⊸ ",
        bang: "!",
        one: "bold(1)",
        top: "⊤",
        zero: "0",
    },
    simple: SIMPLE_UNICODE,
};

/// The SVG spelling: Unicode text, atoms in mathematical italic as the
/// drawing's formulas have them.
#[cfg(feature = "svg")]
pub(crate) const SVG: Spelling = Spelling {
    atom: Atoms::Italic,
    ..UNICODE
};

impl Notation {
    /// Returns the spelling of the notation.
    pub(crate) const fn spelling(self) -> &'static Spelling {
        match self {
            Self::Unicode => &UNICODE,
            Self::Ascii => &ASCII,
        }
    }
}

/// Where a subterm stands, which decides whether it needs brackets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    /// Alone, or delimited by keywords or brackets: anything goes.
    Free,
    /// An operand of `⊗`, or a case's first branch: a binder or a tensor
    /// is bracketed.
    Operand,
    /// A function applied: a binder or a tensor is bracketed.
    Function,
    /// An argument: anything but an atomic term is bracketed.
    Argument,
}

/// Returns whether a node in a place is written in brackets.
const fn bracketed(node: &Node, place: Place) -> bool {
    match node {
        Node::Lambda { .. }
        | Node::LetTensor { .. }
        | Node::LetUnit { .. }
        | Node::Case { .. }
        | Node::Promote { .. }
        | Node::Discard { .. }
        | Node::Copy { .. } => !matches!(place, Place::Free),
        Node::Tensor { .. } => !matches!(place, Place::Free),
        Node::Apply { .. } => matches!(place, Place::Argument),
        Node::Var { .. }
        | Node::Unit
        | Node::Pair { .. }
        | Node::First { .. }
        | Node::Second { .. }
        | Node::Left { .. }
        | Node::Right { .. }
        | Node::True { .. }
        | Node::False { .. }
        | Node::Derelict { .. } => false,
    }
}

/// What the walk writes next.
#[derive(Clone, Copy, Debug)]
enum Piece {
    /// Text.
    Text(&'static str),
    /// A keyword.
    Keyword(&'static str),
    /// A subterm in its place.
    Term(TermId, Place),
    /// A variable's name.
    Var(VarId),
    /// A type, bracketed if binary and the flag says so.
    Type(TypeId, bool),
}

/// Writes terms of one term in one spelling.
struct Printer<'t> {
    /// The term.
    term: &'t Term,
    /// The spelling.
    spelling: &'t Spelling,
    /// Whether binders carry their types.
    types: bool,
}

impl Printer<'_> {
    /// Returns the type connectives of the term's calculus.
    const fn connectives(&self) -> &Connectives {
        match self.term.calculus {
            Calculus::Linear | Calculus::Affine => &self.spelling.linear,
            Calculus::Simple => &self.spelling.simple,
        }
    }

    /// Writes a variable's name.
    fn var(&self, out: &mut impl Write, v: VarId) -> std::fmt::Result {
        let variable = &self.term.variables[v.index()];
        out.write_char(variable.letter)?;
        if variable.index == 0 {
            return Ok(());
        }
        match self.spelling.index {
            Index::Digits => write!(out, "{}", variable.index),
            #[cfg(any(feature = "latex", feature = "typst"))]
            Index::Marked(open, close) => write!(out, "{open}{}{close}", variable.index),
            Index::Subscript => {
                for digit in variable.index.to_string().chars() {
                    let d = digit.to_digit(10).unwrap_or(0);
                    out.write_char(char::from_u32(0x2080 + d).unwrap_or(digit))?;
                }
                Ok(())
            }
        }
    }

    /// Writes a type, in brackets if it is binary and `bracket` is set,
    /// every binary operand in brackets, with a stack of its own.
    fn ty(&self, out: &mut impl Write, ty: TypeId, bracket: bool) -> std::fmt::Result {
        /// A step of the type's walk.
        enum Step {
            /// A type, bracketed if binary and the flag says so.
            Type(TypeId, bool),
            /// Text.
            Text(&'static str),
        }
        let c = self.connectives();
        let mut stack = vec![Step::Type(ty, bracket)];
        while let Some(step) = stack.pop() {
            let (t, bracket) = match step {
                Step::Text(s) => {
                    out.write_str(s)?;
                    continue;
                }
                Step::Type(t, bracket) => (t, bracket),
            };
            let binary = |stack: &mut Vec<Step>, l: TypeId, op: &'static str, r: TypeId| {
                if bracket {
                    stack.push(Step::Text(")"));
                }
                stack.push(Step::Type(r, true));
                stack.push(Step::Text(op));
                stack.push(Step::Type(l, true));
                if bracket {
                    stack.push(Step::Text("("));
                }
            };
            match self.term.types[t.index()] {
                Type::Atom { atom } => self
                    .spelling
                    .atom
                    .write(out, &self.term.atoms[atom as usize])?,
                Type::One => out.write_str(c.one)?,
                Type::Top => out.write_str(c.top)?,
                Type::Zero => out.write_str(c.zero)?,
                Type::Bang { of } => {
                    out.write_str(c.bang)?;
                    stack.push(Step::Type(of, true));
                }
                Type::Tensor { left, right } => binary(&mut stack, left, c.tensor, right),
                Type::With { left, right } => binary(&mut stack, left, c.with, right),
                Type::Plus { left, right } => binary(&mut stack, left, c.plus, right),
                Type::Lollipop { from, to } => binary(&mut stack, from, c.lollipop, to),
            }
        }
        Ok(())
    }

    /// Returns the pieces of one node, in the order written.
    fn pieces(&self, node: &Node) -> Vec<Piece> {
        use Piece::{Keyword, Term as T, Text, Var};
        let s = self.spelling;
        let linear = self.term.calculus != Calculus::Simple;
        let typed = |v: VarId| {
            if self.types {
                vec![
                    Var(v),
                    Text(s.colon),
                    Piece::Type(self.term.variables[v.index()].ty, true),
                ]
            } else {
                vec![Var(v)]
            }
        };
        let list = |terms: &[TermId]| {
            let mut pieces = Vec::new();
            for (i, &t) in terms.iter().enumerate() {
                if i > 0 {
                    pieces.push(Text(", "));
                }
                pieces.push(T(t, Place::Free));
            }
            pieces
        };
        let call = |keyword: &'static str, term: TermId| {
            vec![Keyword(keyword), Text("("), T(term, Place::Free), Text(")")]
        };
        match node {
            Node::Var { variable } => vec![Var(*variable)],
            Node::Lambda { variable, body } => {
                let mut pieces = vec![Text(s.lambda)];
                pieces.extend(typed(*variable));
                pieces.extend([Text(s.dot), T(*body, Place::Free)]);
                pieces
            }
            Node::Apply { function, argument } => vec![
                T(*function, Place::Function),
                Text(s.apply),
                T(*argument, Place::Argument),
            ],
            Node::Tensor { left, right } => vec![
                T(*left, Place::Operand),
                Text(s.tensor),
                T(*right, Place::Operand),
            ],
            Node::LetTensor {
                term,
                left,
                right,
                body,
            } => vec![
                Keyword("let"),
                Text(s.space),
                T(*term, Place::Free),
                Text(s.space),
                Keyword("be"),
                Text(s.space),
                Var(*left),
                Text(s.tensor),
                Var(*right),
                Text(s.space),
                Keyword("in"),
                Text(s.space),
                T(*body, Place::Free),
            ],
            Node::Unit => vec![Text(s.unit)],
            Node::LetUnit { term, body } => vec![
                Keyword("let"),
                Text(s.space),
                T(*term, Place::Free),
                Text(s.space),
                Keyword("be"),
                Text(s.space),
                Text(s.unit),
                Text(s.space),
                Keyword("in"),
                Text(s.space),
                T(*body, Place::Free),
            ],
            Node::Pair { left, right } => vec![
                Text(s.open_pair),
                T(*left, Place::Free),
                Text(", "),
                T(*right, Place::Free),
                Text(s.close_pair),
            ],
            Node::First { term } => call("fst", *term),
            Node::Second { term } => call("snd", *term),
            Node::Left { term, .. } => call("inl", *term),
            Node::Right { term, .. } => call("inr", *term),
            Node::Case {
                term,
                left,
                left_body,
                right,
                right_body,
            } => vec![
                Keyword("case"),
                Text(s.space),
                T(*term, Place::Free),
                Text(s.space),
                Keyword("of"),
                Text(s.space),
                Keyword("inl"),
                Text("("),
                Var(*left),
                Text(")"),
                Text(s.arrow),
                T(*left_body, Place::Operand),
                Text(s.bar),
                Keyword("inr"),
                Text("("),
                Var(*right),
                Text(")"),
                Text(s.arrow),
                T(*right_body, Place::Free),
            ],
            Node::True { .. } if !linear => vec![Text(s.empty_pair)],
            Node::True { context } => {
                let mut pieces = vec![Keyword("true"), Text("(")];
                pieces.extend(list(context));
                pieces.push(Text(")"));
                pieces
            }
            Node::False { term, goal, .. } if !linear => {
                let mut pieces = vec![Keyword("abort")];
                if self.types {
                    pieces.extend([
                        Text(s.subscript.0),
                        Piece::Type(*goal, false),
                        Text(s.subscript.1),
                    ]);
                }
                pieces.extend([Text("("), T(*term, Place::Free), Text(")")]);
                pieces
            }
            Node::False {
                context,
                term,
                goal,
            } => {
                let mut pieces = vec![Keyword("false")];
                if self.types {
                    pieces.extend([
                        Text(s.subscript.0),
                        Piece::Type(*goal, false),
                        Text(s.subscript.1),
                    ]);
                }
                pieces.push(Text("("));
                pieces.extend(list(context));
                pieces.extend([Text("; "), T(*term, Place::Free), Text(")")]);
                pieces
            }
            Node::Promote {
                terms,
                variables,
                body,
            } => {
                let mut pieces = vec![Keyword("promote")];
                for (i, &t) in terms.iter().enumerate() {
                    pieces.push(Text(if i == 0 { s.space } else { ", " }));
                    pieces.push(T(t, Place::Free));
                }
                pieces.extend([Text(s.space), Keyword("for")]);
                for (i, &v) in variables.iter().enumerate() {
                    pieces.push(Text(if i == 0 { s.space } else { ", " }));
                    pieces.push(Var(v));
                }
                pieces.extend([
                    Text(s.space),
                    Keyword("in"),
                    Text(s.space),
                    T(*body, Place::Free),
                ]);
                pieces
            }
            Node::Derelict { term } => call("derelict", *term),
            Node::Discard { term, body } => vec![
                Keyword("discard"),
                Text(s.space),
                T(*term, Place::Free),
                Text(s.space),
                Keyword("in"),
                Text(s.space),
                T(*body, Place::Free),
            ],
            Node::Copy {
                term,
                left,
                right,
                body,
            } => vec![
                Keyword("copy"),
                Text(s.space),
                T(*term, Place::Free),
                Text(s.space),
                Keyword("as"),
                Text(s.space),
                Var(*left),
                Text(", "),
                Var(*right),
                Text(s.space),
                Keyword("in"),
                Text(s.space),
                T(*body, Place::Free),
            ],
        }
    }

    /// Returns the length of a piece that is no subterm, written.
    fn length(&self, piece: Piece) -> u64 {
        let mut text = String::new();
        let written = match piece {
            Piece::Text(t) => return t.len() as u64,
            Piece::Keyword(k) => {
                return (self.spelling.keyword.0.len() + k.len() + self.spelling.keyword.1.len())
                    as u64;
            }
            Piece::Term(..) => return 0,
            Piece::Var(v) => self.var(&mut text, v),
            Piece::Type(t, bracket) => self.ty(&mut text, t, bracket),
        };
        written.map_or(u64::MAX, |()| text.len() as u64)
    }

    /// Returns an upper bound of the length of every node written, each
    /// node's own pieces and two brackets added to its operands', over
    /// the DAG in its order, every sum saturating.
    fn lengths(&self) -> Vec<u64> {
        let mut lengths: Vec<u64> = Vec::with_capacity(self.term.nodes.len());
        for node in &self.term.nodes {
            let mut sum = 2u64;
            for piece in self.pieces(node) {
                let add = match piece {
                    Piece::Term(t, _) => lengths[t.index()],
                    other @ (Piece::Text(_)
                    | Piece::Keyword(_)
                    | Piece::Var(_)
                    | Piece::Type(..)) => self.length(other),
                };
                sum = sum.saturating_add(add);
            }
            lengths.push(sum);
        }
        lengths
    }

    /// Writes the term, or its judgement, into `out`.
    fn write(
        &self,
        out: &mut impl Write,
        judgement: bool,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let term = self.term;
        let s = self.spelling;
        let mut estimate = self.lengths()[term.root.index()];
        // The pieces in the order written, then reversed into the stack.
        let mut written = Vec::new();
        if judgement {
            for (i, &v) in term.context.iter().enumerate() {
                if i > 0 {
                    written.push(Piece::Text(", "));
                }
                written.extend([
                    Piece::Var(v),
                    Piece::Text(s.has_type),
                    Piece::Type(term.variables[v.index()].ty, false),
                ]);
            }
            let turnstile = if term.context.is_empty() {
                s.turnstile.trim_start()
            } else {
                s.turnstile
            };
            written.push(Piece::Text(turnstile));
        }
        written.push(Piece::Term(term.root, Place::Free));
        if judgement {
            written.extend([Piece::Text(s.has_type), Piece::Type(term.goal, false)]);
        }
        for piece in &written {
            if !matches!(piece, Piece::Term(..)) {
                estimate = estimate.saturating_add(self.length(*piece));
            }
        }
        let mut stack: Vec<Piece> = written.into_iter().rev().collect();
        if let Some(limit) = limits.derivation_bytes
            && estimate > limit
        {
            return Err(Error::Refused(Refusal::Output {
                what: "term",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            }));
        }
        let mut stop = crate::limits::counting(stop, Phase::Write);
        while let Some(piece) = stack.pop() {
            match piece {
                Piece::Text(t) => out.write_str(t)?,
                Piece::Keyword(k) => {
                    out.write_str(s.keyword.0)?;
                    out.write_str(k)?;
                    out.write_str(s.keyword.1)?;
                }
                Piece::Var(v) => self.var(out, v)?,
                Piece::Type(t, bracket) => self.ty(out, t, bracket)?,
                Piece::Term(t, place) => {
                    if stop() {
                        return Err(Error::Refused(Refusal::Stopped {
                            phase: Phase::Write,
                        }));
                    }
                    let node = &term.nodes[t.index()];
                    let bracket = bracketed(node, place);
                    if bracket {
                        stack.push(Piece::Text(")"));
                    }
                    stack.extend(self.pieces(node).into_iter().rev());
                    if bracket {
                        stack.push(Piece::Text("("));
                    }
                }
            }
        }
        Ok(())
    }
}

impl Term {
    /// Writes the term into `out` as `options` say, after an upper bound
    /// of its length has passed `limits.derivation_bytes`, asking `stop`
    /// once per node written.
    ///
    /// # Errors
    ///
    /// [`Refusal::Output`] for a term estimated past the bound, before
    /// anything is written; [`Refusal::Stopped`] when `stop` fired;
    /// [`Error::WriteFailed`] when `out` refused the text.
    pub fn write(
        &self,
        out: &mut impl Write,
        options: &Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        self.write_in(
            options.notation.spelling(),
            options.types,
            false,
            out,
            limits,
            stop,
        )
    }

    /// Writes the term's typing judgement into `out`, `x : A, y : B ⊢ M :
    /// C`, the hypotheses' variables with their types, then the term and
    /// its type, as [`write`](Self::write) writes the term.
    ///
    /// # Errors
    ///
    /// As [`write`](Self::write).
    pub fn write_judgement(
        &self,
        out: &mut impl Write,
        options: &Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        self.write_in(
            options.notation.spelling(),
            options.types,
            true,
            out,
            limits,
            stop,
        )
    }

    /// Writes the term, or its judgement, in a spelling.
    pub(crate) fn write_in(
        &self,
        spelling: &Spelling,
        types: bool,
        judgement: bool,
        out: &mut impl Write,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        Printer {
            term: self,
            spelling,
            types,
        }
        .write(out, judgement, limits, stop)
    }
}
