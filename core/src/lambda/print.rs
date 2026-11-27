// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The one printer of terms: a table of spellings per target (Unicode,
//! ASCII, and one per export), and a walk on a stack of its own that
//! writes a term or its typing judgement. Before it writes anything, it
//! bounds the text's length and refuses a term past the output bound.
//!
//! A node that several others name is written at every place (the
//! printed term is the tree the DAG unfolds to), so the text can be
//! longer than the derivation by any factor. The bound is computed over
//! the DAG, each node once.

use super::{Calculus, Node, Notation, Options, Term, TermId, Type, TypeId, TypeVarId, VarId};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::sequents::notation::Atoms;
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
    /// `Λ`, the type abstraction of System F.
    type_lambda: &'static str,
    /// `∀` before its variable.
    forall: &'static str,
    /// What a type application's type is written between.
    type_argument: (&'static str, &'static str),
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
    /// How a variable's name is written, but the digits at its end:
    /// LaTeX and Typst write a name of several letters as they write an
    /// atom's, set apart from the letters around it.
    variable: Atoms,
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
    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    Marked(&'static str, &'static str),
}

/// The Unicode spelling, which `Display` writes, and the command by
/// default.
pub(crate) const UNICODE: Spelling = Spelling {
    lambda: "λ",
    type_lambda: "Λ",
    forall: "∀",
    type_argument: ("[", "]"),
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
    variable: Atoms::Plain,
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
    type_lambda: "/\\",
    forall: "forall ",
    type_argument: ("[", "]"),
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
    variable: Atoms::Plain,
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
    type_lambda: r"\Lambda ",
    forall: r"\forall ",
    type_argument: ("[", "]"),
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
    variable: Atoms::Latex,
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
    type_lambda: "Lambda ",
    forall: "forall ",
    type_argument: ("[", "]"),
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
    variable: Atoms::Typst,
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
/// drawing's formulas have them, and an index and a subscript lowered
/// between the drawing's marks, since the math fonts have no subscript
/// digits.
#[cfg(feature = "svg")]
pub(crate) const SVG: Spelling = Spelling {
    atom: Atoms::Italic,
    subscript: ("\u{4}", "\u{5}"),
    index: Index::Marked("\u{4}", "\u{5}"),
    ..UNICODE
};

/// Writes a variable's name: what stands before the digits at its end
/// as `letters` writes it, and those digits as an index. A name of
/// digits alone has none.
pub(crate) fn write_name(
    out: &mut impl Write,
    name: &str,
    index: Index,
    letters: Atoms,
) -> std::fmt::Result {
    let stem = name.trim_end_matches(|c: char| c.is_ascii_digit());
    let (stem, digits) = if stem.is_empty() {
        (name, "")
    } else {
        name.split_at(stem.len())
    };
    letters.write(out, stem)?;
    if digits.is_empty() {
        return Ok(());
    }
    match index {
        Index::Digits => out.write_str(digits),
        #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
        Index::Marked(open, close) => write!(out, "{open}{digits}{close}"),
        Index::Subscript => digits.chars().try_for_each(|digit| {
            let d = digit.to_digit(10).unwrap_or(0);
            out.write_char(char::from_u32(0x2080 + d).unwrap_or(digit))
        }),
    }
}

/// Writes a type of an arena in a calculus's connectives, in brackets if
/// it is binary or a `∀` and `bracket` is set, every such operand in
/// brackets, with a stack of its own.
///
/// A `∀` is written with the name it keeps, or, where that name is free
/// in its body (an atom's, a type variable's, or that of a `∀` around
/// whose variable the body names), with the least index that makes it
/// none of them (`X₁`): the text then reads back as the type, whatever
/// names a substitution brought in.
#[expect(
    clippy::too_many_arguments,
    reason = "the arenas a type names and how to spell it"
)]
pub(crate) fn write_type(
    out: &mut impl Write,
    types: &[Type],
    atoms: &[Box<str>],
    type_variables: &[Box<str>],
    spelling: &Spelling,
    connectives: &Connectives,
    ty: TypeId,
    bracket: bool,
) -> std::fmt::Result {
    /// A step of the type's walk.
    enum Step {
        /// A type, bracketed if binary and the flag says so.
        Type(TypeId, bool),
        /// Text.
        Text(&'static str),
        /// The end of a `∀`'s body: its name leaves the names in scope.
        Leave,
    }
    let c = connectives;
    let atom = spelling.atom;
    let name = |out: &mut _, name: &str| write_name(out, name, spelling.index, atom);
    // The printed names of the `∀`s around, the innermost last.
    let mut scope: Vec<Box<str>> = Vec::new();
    // What each type of the root's names freely: computed at the first
    // `∀`.
    let mut free: Option<HashMap<TypeId, Free<'_>>> = None;
    let mut stack = vec![Step::Type(ty, bracket)];
    while let Some(step) = stack.pop() {
        let (t, bracket) = match step {
            Step::Text(s) => {
                out.write_str(s)?;
                continue;
            }
            Step::Leave => {
                scope.pop();
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
        match types[t.index()] {
            Type::Atom { atom: a } => atom.write(out, &atoms[a as usize])?,
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
            Type::Variable { variable } => name(out, &type_variables[variable.index()])?,
            Type::Bound { index } => {
                // A closed type's index names a `∀` around it.
                let at = scope.len().checked_sub(index as usize + 1);
                match at.and_then(|at| scope.get(at)) {
                    Some(bound) => name(out, bound)?,
                    None => out.write_str("?")?,
                }
            }
            Type::Forall { variable, body } => {
                let free = free.get_or_insert_with(|| free_in(types, atoms, type_variables, ty));
                let inner = &free[&body];
                // `Bound { index: k }` with `k ≥ 1` in the body names the
                // `∀` `k - 1` places out from this one.
                let outer = |candidate: &str| {
                    inner.escapes.iter().any(|&k| {
                        k >= 1
                            && scope
                                .len()
                                .checked_sub(k as usize)
                                .and_then(|at| scope.get(at))
                                .is_some_and(|name| &**name == candidate)
                    })
                };
                let chosen = fresh(&type_variables[variable.index()], |candidate| {
                    inner.names.contains(&candidate) || outer(candidate)
                });
                if bracket {
                    out.write_str("(")?;
                    stack.push(Step::Text(")"));
                }
                out.write_str(spelling.forall)?;
                name(out, &chosen)?;
                out.write_str(spelling.dot)?;
                scope.push(chosen);
                stack.push(Step::Leave);
                stack.push(Step::Type(body, false));
            }
        }
    }
    Ok(())
}

/// What a type names freely: the names of its atoms and type variables,
/// and the bound indices that point past it, as counted at its top.
struct Free<'a> {
    /// The names, sorted, each once.
    names: Vec<&'a str>,
    /// The indices, sorted, each once.
    escapes: Vec<u32>,
}

/// Returns what every type that `root` reaches names freely, each type
/// of its DAG visited once, its operands before it.
fn free_in<'a>(
    types: &[Type],
    atoms: &'a [Box<str>],
    type_variables: &'a [Box<str>],
    root: TypeId,
) -> HashMap<TypeId, Free<'a>> {
    let mut made: HashMap<TypeId, Free<'a>> = HashMap::default();
    let mut stack = vec![(root, false)];
    while let Some((t, left)) = stack.pop() {
        if made.contains_key(&t) {
            continue;
        }
        let ty = types[t.index()];
        let mut operands: Vec<TypeId> = Vec::new();
        match ty {
            Type::Tensor { left, right }
            | Type::With { left, right }
            | Type::Plus { left, right } => {
                operands.extend([left, right]);
            }
            Type::Lollipop { from, to } => operands.extend([from, to]),
            Type::Bang { of } | Type::Forall { body: of, .. } => operands.push(of),
            Type::Atom { .. }
            | Type::One
            | Type::Top
            | Type::Zero
            | Type::Bound { .. }
            | Type::Variable { .. } => {}
        }
        if !left {
            stack.push((t, true));
            stack.extend(operands.iter().map(|&o| (o, false)));
            continue;
        }
        let mut names: Vec<&'a str> = Vec::new();
        let mut escapes: Vec<u32> = Vec::new();
        match ty {
            Type::Atom { atom } => names.push(&atoms[atom as usize]),
            Type::Variable { variable } => names.push(&type_variables[variable.index()]),
            Type::Bound { index } => escapes.push(index),
            Type::Forall { body, .. } => {
                let inner = &made[&body];
                names.extend_from_slice(&inner.names);
                escapes.extend(inner.escapes.iter().filter_map(|k| k.checked_sub(1)));
            }
            Type::One
            | Type::Top
            | Type::Zero
            | Type::Tensor { .. }
            | Type::With { .. }
            | Type::Plus { .. }
            | Type::Lollipop { .. }
            | Type::Bang { .. } => {
                for o in &operands {
                    names.extend_from_slice(&made[o].names);
                    escapes.extend_from_slice(&made[o].escapes);
                }
            }
        }
        names.sort_unstable();
        names.dedup();
        escapes.sort_unstable();
        escapes.dedup();
        made.insert(t, Free { names, escapes });
    }
    made
}

/// Returns `name` if `taken` refuses it not, else its letters with the
/// least index that `taken` refuses not: `X`, then `X1`, `X2`, ….
pub(crate) fn fresh(name: &str, taken: impl Fn(&str) -> bool) -> Box<str> {
    if !taken(name) {
        return name.into();
    }
    let stem = name.trim_end_matches(|c: char| c.is_ascii_digit());
    let stem = if stem.is_empty() { name } else { stem };
    (1u64..)
        .map(|index| format!("{stem}{index}"))
        .find(|candidate| !taken(candidate))
        .map_or_else(|| name.into(), Into::into)
}

impl Spelling {
    /// Returns the type connectives of a calculus.
    pub(crate) const fn connectives(&self, calculus: Calculus) -> &Connectives {
        match calculus {
            Calculus::Linear
            | Calculus::Affine
            | Calculus::SystemFLinear
            | Calculus::SystemFAffine => &self.linear,
            Calculus::Simple
            | Calculus::Untyped
            | Calculus::UntypedLinear
            | Calculus::UntypedAffine
            | Calculus::SystemF => &self.simple,
        }
    }
}

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
    /// An argument: a binder, a tensor or an application is bracketed.
    Argument,
}

/// Returns whether a node in a place is written in brackets.
const fn bracketed(node: &Node, place: Place) -> bool {
    match node {
        Node::Lambda { .. }
        | Node::TypeLambda { .. }
        | Node::LetTensor { .. }
        | Node::LetUnit { .. }
        | Node::Case { .. }
        | Node::Promote { .. }
        | Node::Discard { .. }
        | Node::Copy { .. } => !matches!(place, Place::Free),
        Node::Tensor { .. } => !matches!(place, Place::Free),
        Node::Apply { .. } | Node::TypeApply { .. } => matches!(place, Place::Argument),
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
    /// A type variable's name.
    TypeVar(TypeVarId),
    /// A type, bracketed if binary and the flag says so.
    Type(TypeId, bool),
    /// The start of the marked subterm's text.
    MarkStart,
    /// The end of the marked subterm's text.
    Marked,
}

/// A writer that counts the bytes written through it.
struct Counted<'w, W: Write> {
    /// The writer.
    out: &'w mut W,
    /// The bytes written.
    bytes: usize,
}

impl<W: Write> Write for Counted<'_, W> {
    /// Writes `s` and counts its bytes.
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.bytes += s.len();
        self.out.write_str(s)
    }
}

/// Writes terms of one term in one spelling.
struct Printer<'t> {
    /// The term.
    term: &'t Term,
    /// The spelling.
    spelling: &'t Spelling,
    /// Whether `λ`, `false` and `abort` show types.
    types: bool,
    /// An upper bound of the length of every type written, in brackets.
    type_lengths: Vec<u64>,
}

impl<'t> Printer<'t> {
    /// Returns the printer of a term in a spelling, with the bounds of its
    /// types' lengths computed over the arena of types, each type once:
    /// an equal operand is one id, so a type's text can be longer than
    /// the arena by any factor (an equivalence nested `k` deep writes its
    /// innermost operand `2ᵏ` times), and no text is built to measure it.
    fn new(term: &'t Term, spelling: &'t Spelling, types: bool) -> Self {
        let mut printer = Self {
            term,
            spelling,
            types,
            type_lengths: Vec::with_capacity(term.types.len()),
        };
        let c = printer.connectives();
        // A type variable's name, written with an index that renaming may
        // have added: every character at most four bytes, the index's
        // digits at most twenty, each as a mark around it at most.
        let longest = term
            .type_variables
            .iter()
            .map(|name| name.len() as u64)
            .max()
            .unwrap_or(0);
        let named = longest.saturating_mul(4).saturating_add(120);
        let mut lengths: Vec<u64> = Vec::with_capacity(term.types.len());
        for ty in &term.types {
            let of = |t: TypeId| lengths[t.index()];
            let binary = |l: TypeId, op: &str, r: TypeId| {
                of(l)
                    .saturating_add(of(r))
                    .saturating_add(op.len() as u64 + 2)
            };
            let length = match *ty {
                Type::Atom { atom } => {
                    let mut name = String::new();
                    // An atom's name is the input's, as long as the input
                    // allowed it.
                    let written = spelling.atom.write(&mut name, &term.atoms[atom as usize]);
                    written.map_or(u64::MAX, |()| name.len() as u64)
                }
                Type::One => c.one.len() as u64,
                Type::Top => c.top.len() as u64,
                Type::Zero => c.zero.len() as u64,
                Type::Bang { of: a } => of(a).saturating_add(c.bang.len() as u64 + 2),
                Type::Tensor { left, right } => binary(left, c.tensor, right),
                Type::With { left, right } => binary(left, c.with, right),
                Type::Plus { left, right } => binary(left, c.plus, right),
                Type::Lollipop { from, to } => binary(from, c.lollipop, to),
                Type::Forall { body, .. } => of(body)
                    .saturating_add(named)
                    .saturating_add((spelling.forall.len() + spelling.dot.len() + 2) as u64),
                Type::Bound { .. } | Type::Variable { .. } => named,
            };
            lengths.push(length);
        }
        printer.type_lengths = lengths;
        printer
    }

    /// Returns the printer of a term in a spelling, without the bounds of
    /// its types' lengths: for writing alone, not for an estimate.
    const fn unmeasured(term: &'t Term, spelling: &'t Spelling, types: bool) -> Self {
        Self {
            term,
            spelling,
            types,
            type_lengths: Vec::new(),
        }
    }

    /// Returns the type connectives of the term's calculus.
    const fn connectives(&self) -> &Connectives {
        self.spelling.connectives(self.term.calculus)
    }

    /// Writes a variable's name.
    fn var(&self, out: &mut impl Write, v: VarId) -> std::fmt::Result {
        write_name(
            out,
            &self.term.variables[v.index()].name,
            self.spelling.index,
            self.spelling.variable,
        )
    }

    /// Writes a type variable's name.
    fn type_var(&self, out: &mut impl Write, v: TypeVarId) -> std::fmt::Result {
        write_name(
            out,
            &self.term.type_variables[v.index()],
            self.spelling.index,
            self.spelling.atom,
        )
    }

    /// Writes a type, in brackets if it is binary and `bracket` is set,
    /// every binary operand in brackets.
    fn ty(&self, out: &mut impl Write, ty: TypeId, bracket: bool) -> std::fmt::Result {
        write_type(
            out,
            &self.term.types,
            &self.term.atoms,
            &self.term.type_variables,
            self.spelling,
            self.connectives(),
            ty,
            bracket,
        )
    }

    /// Returns the pieces of one node, in the order written.
    fn pieces(&self, node: &Node) -> Vec<Piece> {
        use Piece::{Keyword, Term as T, Text, Var};
        let s = self.spelling;
        let linear = !self.term.calculus.is_ordinary();
        let typed = |v: VarId| match self.term.variables[v.index()].ty {
            Some(ty) if self.types => vec![Var(v), Text(s.colon), Piece::Type(ty, true)],
            _ => vec![Var(v)],
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
        // A keyword with the type it gives as a subscript, where types
        // are shown. A spelling that closes its subscript with nothing
        // brackets a type that is no atom or constant, so that the text
        // reads back.
        let subscripted = |keyword: &'static str, ty: TypeId| {
            let mut pieces = vec![Keyword(keyword)];
            if self.types {
                pieces.extend([
                    Text(s.subscript.0),
                    Piece::Type(ty, s.subscript.1.is_empty()),
                    Text(s.subscript.1),
                ]);
            }
            pieces
        };
        let injection = |keyword: &'static str, term: TermId, sum: TypeId| {
            let mut pieces = subscripted(keyword, sum);
            pieces.extend([Text("("), T(term, Place::Free), Text(")")]);
            pieces
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
            Node::TypeLambda { variable, body } => vec![
                Text(s.type_lambda),
                Piece::TypeVar(*variable),
                Text(s.dot),
                T(*body, Place::Free),
            ],
            Node::TypeApply { term, argument } => vec![
                T(*term, Place::Function),
                Text(s.apply),
                Text(s.type_argument.0),
                Piece::Type(*argument, false),
                Text(s.type_argument.1),
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
            Node::Left { term, sum } => injection("inl", *term, *sum),
            Node::Right { term, sum } => injection("inr", *term, *sum),
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
            Node::False { term, goal, .. } if !linear => injection("abort", *term, *goal),
            Node::False {
                context,
                term,
                goal,
            } => {
                let mut pieces = subscripted("false", *goal);
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
            Piece::Term(..) | Piece::MarkStart | Piece::Marked => return 0,
            Piece::Var(v) => self.var(&mut text, v),
            Piece::TypeVar(v) => self.type_var(&mut text, v),
            Piece::Type(t, _) => return self.type_lengths[t.index()],
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
                    | Piece::TypeVar(_)
                    | Piece::Type(..)
                    | Piece::MarkStart
                    | Piece::Marked) => self.length(other),
                };
                sum = sum.saturating_add(add);
            }
            lengths.push(sum);
        }
        lengths
    }

    /// Writes the term, or its judgement, into `out`, and returns where
    /// the text of the subterm at `mark` first stands in it.
    fn write(
        &self,
        out: &mut impl Write,
        judgement: bool,
        mark: Option<TermId>,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Option<std::ops::Range<usize>>, Error> {
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
                written.push(Piece::Var(v));
                if let Some(ty) = term.variables[v.index()].ty {
                    written.extend([Piece::Text(s.has_type), Piece::Type(ty, false)]);
                }
            }
            let turnstile = if term.context.is_empty() {
                s.turnstile.trim_start()
            } else {
                s.turnstile
            };
            written.push(Piece::Text(turnstile));
        }
        written.push(Piece::Term(term.root, Place::Free));
        if judgement && let Some(goal) = term.goal {
            written.extend([Piece::Text(s.has_type), Piece::Type(goal, false)]);
        }
        for piece in &written {
            if !matches!(piece, Piece::Term(..)) {
                estimate = estimate.saturating_add(self.length(*piece));
            }
        }
        let stack: Vec<Piece> = written.into_iter().rev().collect();
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
        self.emit(out, stack, mark, &mut stop)
    }

    /// Writes the pieces of `stack`, the last first, unfolding every
    /// subterm in place, and asks `stop` before each. Returns where the
    /// text of the subterm at `mark` first stands, in bytes from where
    /// the writing started.
    fn emit(
        &self,
        out: &mut impl Write,
        mut stack: Vec<Piece>,
        mut mark: Option<TermId>,
        stop: &mut dyn FnMut() -> bool,
    ) -> Result<Option<std::ops::Range<usize>>, Error> {
        let (term, s) = (self.term, self.spelling);
        let out = &mut Counted { out, bytes: 0 };
        let mut start = 0;
        let mut marked = None;
        while let Some(piece) = stack.pop() {
            match piece {
                Piece::Text(t) => out.write_str(t)?,
                Piece::Keyword(k) => {
                    out.write_str(s.keyword.0)?;
                    out.write_str(k)?;
                    out.write_str(s.keyword.1)?;
                }
                Piece::Var(v) => self.var(out, v)?,
                Piece::TypeVar(v) => self.type_var(out, v)?,
                Piece::Type(t, bracket) => self.ty(out, t, bracket)?,
                Piece::MarkStart => start = out.bytes,
                Piece::Marked => marked = Some(start..out.bytes),
                Piece::Term(t, place) => {
                    let marking = mark == Some(t);
                    if marking {
                        mark = None;
                    }
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
                    // The mark holds the subterm's text inside its brackets.
                    if marking {
                        stack.push(Piece::Marked);
                    }
                    stack.extend(self.pieces(node).into_iter().rev());
                    if marking {
                        stack.push(Piece::MarkStart);
                    }
                    if bracket {
                        stack.push(Piece::Text("("));
                    }
                }
            }
        }
        Ok(marked)
    }
}

/// Returns upper bounds of the text of every node and of every type of a
/// term in a spelling, each node's or type's once over the arenas, every
/// sum saturating: the lengths an estimate of a drawing adds up.
pub(crate) fn lengths(term: &Term, spelling: &Spelling, types: bool) -> (Vec<u64>, Vec<u64>) {
    let printer = Printer::new(term, spelling, types);
    let nodes = printer.lengths();
    (nodes, printer.type_lengths)
}

/// How a judgement of a subterm is written besides its spelling: the
/// marks around each of its parts, what aligns its turnstile, and
/// whether `λ` shows its variable's type.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Framing {
    /// Whether `λ`, `inl`, `inr`, `false` and `abort` show types.
    pub(crate) types: bool,
    /// What stands before and after each declaration of the context and
    /// the typed term, empty for nothing.
    pub(crate) marks: (&'static str, &'static str),
    /// What stands before the turnstile, empty for nothing.
    pub(crate) align: &'static str,
}

/// Writes the judgement `Γ ⊢ M : A` of the subterm at `node` of a term,
/// `Γ` the variables of `context` with their types and `A` the type
/// `goal` of the term's arena. The text is not bounded here: the caller
/// estimated it.
pub(crate) fn write_judgement_at(
    out: &mut impl Write,
    term: &Term,
    spelling: &Spelling,
    context: &[VarId],
    node: TermId,
    goal: TypeId,
    framing: Framing,
) -> std::fmt::Result {
    let printer = Printer::unmeasured(term, spelling, framing.types);
    let (open, close) = framing.marks;
    let mut written = Vec::with_capacity(context.len() * 6 + 8);
    for (i, &v) in context.iter().enumerate() {
        if i > 0 {
            written.push(Piece::Text(", "));
        }
        written.extend([
            Piece::Text(open),
            Piece::Var(v),
            Piece::Text(spelling.has_type),
            Piece::Type(term.variables[v.index()].typed(), false),
            Piece::Text(close),
        ]);
    }
    // The alignment stands right before the turnstile, after the space
    // that separates it from the context.
    let turnstile = spelling.turnstile.trim_start();
    if !context.is_empty() {
        written.push(Piece::Text(
            &spelling.turnstile[..spelling.turnstile.len() - turnstile.len()],
        ));
    }
    written.extend([Piece::Text(framing.align), Piece::Text(turnstile)]);
    written.extend([
        Piece::Text(open),
        Piece::Term(node, Place::Free),
        Piece::Text(spelling.has_type),
        Piece::Type(goal, false),
        Piece::Text(close),
    ]);
    written.reverse();
    printer
        .emit(out, written, None, &mut || false)
        .map(drop)
        .map_err(|_| std::fmt::Error)
}

impl Term {
    /// Writes the term into `out` as `options` say, asking `stop` once per
    /// node written.
    ///
    /// Before it writes anything, it bounds the text's length from above
    /// and refuses a term past `limits.derivation_bytes`. The bound
    /// matters: a node that several others name is written at every
    /// place, so a small term can stand for a long text. For a term in a
    /// `String` without limits or a stop, use `Display` (`to_string()`).
    ///
    /// # Errors
    ///
    /// - [`Refusal::Output`] for a term estimated past the bound, before
    ///   anything is written.
    /// - [`Refusal::Stopped`] when `stop` fired.
    /// - [`Error::WriteFailed`] when `out` refused the text.
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

    /// Writes the term's typing judgement into `out`, as
    /// [`write`](Self::write) writes the term: the hypotheses' variables
    /// with their types, then the term and its type, as in `x : A, f : A ⊸
    /// B ⊢ f x : B`. With no hypotheses, the judgement starts with `⊢`.
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
        Printer::new(self, spelling, types)
            .write(out, judgement, None, limits, stop)
            .map(drop)
    }

    /// Writes the term into `out` as [`write`](Self::write) does, and
    /// returns where the text of the subterm at `node` stands in what it
    /// wrote: its byte range inside any brackets around it, at the first
    /// place the text writes it, or `0..0` for a node the text does not
    /// write. A front end marks a redex so
    /// ([`Strategy::pick`](super::Strategy::pick)).
    ///
    /// # Errors
    ///
    /// As [`write`](Self::write).
    pub fn write_marked(
        &self,
        out: &mut impl Write,
        node: TermId,
        options: &Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<std::ops::Range<usize>, Error> {
        let marked = Printer::new(self, options.notation.spelling(), options.types).write(
            out,
            false,
            Some(node),
            limits,
            stop,
        )?;
        Ok(marked.unwrap_or(0..0))
    }
}

impl std::fmt::Display for Term {
    /// Writes the term under the default [`Options`] within the default
    /// [`Limits`], as [`write`](Term::write) does. It fails only when the
    /// formatter does, so `to_string()` never panics.
    ///
    /// A term whose text would pass `Limits::derivation_bytes` is not
    /// written: the text is then the refusal's message between angle
    /// brackets. [`write`](Term::write) with larger limits writes such a
    /// term.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.write(f, &Options::default(), &Limits::default(), |_| false) {
            Ok(()) => Ok(()),
            // Refused before anything is written.
            Err(Error::Refused(refusal)) => write!(f, "<{refusal}>"),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
