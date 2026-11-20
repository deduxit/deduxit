// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A proof as a program: an intuitionistic proof read back as a
//! lambda-term.
//!
//! Every intuitionistic proof is also a program, whose types are the
//! proof's formulas and whose inputs are its hypotheses (the
//! Curry-Howard correspondence). This module gives that program as a
//! [`Term`]:
//! - a two-sided proof of intuitionistic linear logic reads back as a term
//!   of a linear lambda calculus, in which every variable is used once;
//! - a derivation of LJ reads back as a term of the simply typed lambda
//!   calculus.
//!
//! # Get a term
//!
//! Prove a sequent in an intuitionistic mode, then call
//! [`Derivation::term`](crate::linear::Derivation::term) on its
//! derivation:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::{Limits, lambda};
//! use deduxit::linear::{Mode, Options, Sequent, Verdict, prove};
//!
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! let outcome = prove(&sequent, Mode::INTUITIONISTIC, &Options::default())?;
//! let Verdict::Proved(proof) = &outcome.verdict else {
//!     panic!("the sequent is provable");
//! };
//! let term = proof.derivation()?.term(&Limits::default(), |_| false)?;
//! assert_eq!(term.to_string(), "f x");
//!
//! let mut judgement = String::new();
//! let options = lambda::Options::default();
//! term.write_judgement(&mut judgement, &options, &Limits::default(), |_| false)?;
//! assert_eq!(judgement, "x : A, f : A ⊸ B ⊢ f x : B");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! The judgement reads: given `x` of type `A` and `f` of type `A ⊸ B`,
//! the term `f x` has type `B`. [`Options`] picks the symbols, Unicode or
//! ASCII, and whether every `λ` shows its variable's type.
//! [`ordinary::Derivation::term`](crate::ordinary::Derivation::term)
//! reads a derivation of LJ back the same way. The LaTeX, Typst and SVG
//! exports write the judgement under a derivation when their `lambda`
//! option is set ([`export`](crate::export)).
//!
//! The term is built from a derivation the checker accepted, one
//! construction per rule, and the proof stays the certificate.
//!
//! # Write and check a term
//!
//! [`Term::parse`] reads a term from text, and [`Term::check`] checks
//! that it is well typed in its [`Calculus`]:
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::Limits;
//! use deduxit::lambda::{Calculus, Term, Type};
//!
//! let limits = Limits::default();
//! let term = Term::parse(r"\f:a -> b. \x:a. f x", Calculus::Simple, &limits)?;
//! let typing = term.check(&limits, |_| false)?;
//!
//! // The whole term is a function, from `a -> b`.
//! let Type::Lollipop { from, .. } = typing.types()[typing.ty(term.root()).index()] else {
//!     panic!("a function");
//! };
//! assert!(matches!(typing.types()[from.index()], Type::Lollipop { .. }));
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! The terms are typed in Church's style: every `λ` says its variable's
//! type, so a term has one type at most, and nothing is inferred. A
//! term read back from a proof is checked the same way, and so is one
//! read from JSON ([`wire::upgrade`](crate::wire::upgrade), with the
//! feature `serialize`).
//!
//! # What has no term
//!
//! A proof of classical linear logic or a derivation of LK, a proof with
//! Mix, of the cyclic or the Lambek calculus, with quantifiers, or with
//! open goals. Each is refused with [`Error::NoTerm`](crate::Error::NoTerm),
//! whose [`NoTerm`] says why.
//!
//! # The calculi
//!
//! A proof of intuitionistic linear logic reads back as a term of the
//! linear lambda calculus of Benton, Bierman, de Paiva and Hyland, "A
//! term calculus for intuitionistic linear logic" (TLCA 1993, LNCS 664).
//! deduxit takes it in the sequent calculus form, with the additives and
//! their units, that G. M. Bierman gives in *On Intuitionistic Linear
//! Logic* (PhD thesis, Cambridge, 1993, published as technical report
//! UCAM-CL-TR-346, 1994), chapter 3, figure 3.2. Each rule has its term:
//!
//! | rule | term |
//! |---|---|
//! | axiom | `x` |
//! | `⊸R`, `⊸L` | `λx:A. M`; `N[x := f M]` |
//! | `⊗R`, `⊗L` | `M ⊗ N`; `let z be x ⊗ y in M` |
//! | `1R`, `1L` | `∗`; `let z be ∗ in M` |
//! | `&R`, `&L₁`, `&L₂` | `⟨M, N⟩`; `M[x := fst(z)]`, `M[x := snd(z)]` |
//! | `⊕R₁`, `⊕R₂`, `⊕L` | `inl(M)`, `inr(M)`; `case z of inl(x) → M ‖ inr(y) → N` |
//! | `⊤R`, `0L` | `true(x⃗)`; `false_A(x⃗; z)` |
//! | `!R`, `!L` | `promote y⃗ for x⃗ in M`; `M[x := derelict(z)]` |
//! | `!w`, `!c` | `discard z in M`; `copy z as x, y in M` |
//! | cut | `(λx:A. N) M` |
//!
//! A left rule substitutes, as in the thesis (`M[x := …]`), so the term
//! of a cut-free proof is in normal form. The thesis's cut is the
//! substitution `N[x := M]` itself; deduxit writes it as the redex
//! `(λx:A. N) M`, whose reduction is that substitution. So cut
//! elimination is reduction, up to the commuting conversions that move a
//! `let` or a `case` past a cut.
//!
//! A proof of affine linear logic reads back to the same terms, with
//! weakening left implicit: a variable may go unused
//! ([`Calculus::Affine`]).
//!
//! A derivation of LJ reads back as a term of the simply typed lambda
//! calculus of Girard, Lafont and Taylor, *Proofs and Types* (1989),
//! sections 3.1.2 and 10.6. Their `π₁`, `π₂`, `ι₁`, `ι₂`, `δ` and `ε` are
//! written `fst`, `snd`, `inl`, `inr`, `case` and `abort`, and the unit,
//! which the book does not name, is `⟨⟩`. `¬A` is the type `A → ⊥`, and
//! `A ↔ B` the pair of both implications.
//!
//! The cut's redex, the affine reading, `⟨⟩`, the ASCII notation and the
//! subscript that gives `inl` and `inr` their type are deduxit's own;
//! every other form is the sources'.
//!
//! # The term's data
//!
//! A [`Term`] holds its own atoms, types and variables, and its nodes in
//! an arena in which every node comes after the nodes it names. A walk
//! over it needs no recursion, and its JSON form nests no deeper than one
//! node.

// A new variant of `Node` or `Type` must be decided at every match, not
// fall into an existing arm.
#![deny(clippy::wildcard_enum_match_arm)]

mod check;
mod draw;
mod inhabit;
pub(crate) mod linear;
#[cfg(feature = "parse")]
mod parse;
pub(crate) mod print;
#[cfg(feature = "serialize")]
pub(crate) mod read;
pub(crate) mod simple;
#[cfg(all(test, feature = "parse"))]
mod tests;

pub use check::Typing;
pub use draw::Drawing;
#[cfg(feature = "protocol")]
pub(crate) use inhabit::has_type;
pub use inhabit::{Count, InhabitOptions, Inhabitants, Inhabitation, enumerate, inhabit};

use crate::hash::HashMap;
use crate::limits::Refusal;

/// The index of a node in a [`Term`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct TermId(u32);

impl TermId {
    /// Returns the index as a `usize`, for indexing the nodes.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The index of a variable in a [`Term`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct VarId(u32);

impl VarId {
    /// Returns the index as a `usize`, for indexing the variables.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The index of a type in a [`Term`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct TypeId(u32);

impl TypeId {
    /// Returns the index as a `usize`, for indexing the types.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The calculus a term belongs to, which says how often it may use a
/// variable.
///
/// # JSON
///
/// With the feature `serialize`, `"linear"`, `"affine"` or `"simple"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Calculus {
    /// The linear lambda calculus: every variable is used exactly once.
    /// `discard`, `copy`, `true` and `false` each count as a use, and the
    /// two components of a pair, like the two branches of a `case`, use
    /// the same variables.
    Linear,
    /// The affine lambda calculus: as the linear one, but a variable may
    /// also go unused. A proof that weakens a formula without `!` reads
    /// back to it.
    Affine,
    /// The simply typed lambda calculus: a variable may be used any number
    /// of times.
    Simple,
}

impl Calculus {
    /// Every calculus, in the order of their names.
    pub const ALL: &'static [Self] = &[Self::Linear, Self::Affine, Self::Simple];

    /// The names, as the JSON form and the command write them.
    pub const NAMES: &'static [&'static str] = &["linear", "affine", "simple"];

    /// Returns the calculus's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Affine => "affine",
            Self::Simple => "simple",
        }
    }
}

impl std::str::FromStr for Calculus {
    type Err = crate::Error;

    /// Reads a calculus by its name.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .copied()
            .find(|c| c.name() == s)
            .ok_or_else(|| crate::Error::UnknownName {
                what: "lambda calculus",
                name: s.into(),
                known: Self::NAMES,
            })
    }
}

/// A type of a term.
///
/// In the linear calculi a type is a formula of intuitionistic linear
/// logic. In the simple calculus it is an ordinary formula: `With` is the
/// conjunction `∧`, `Plus` the disjunction `∨`, `Lollipop` the
/// implication `→`, `Top` true and `Zero` false (`⊥`).
///
/// A type names its operands by their [`TypeId`], and they come before it
/// in [`Term::types`]. Two equal types are one id, so a type's text can
/// be much longer than the list of types.
///
/// # JSON
///
/// With the feature `serialize`, an object tagged by `kind` in snake
/// case, its operands by their index: `{"kind": "lollipop", "from": 0,
/// "to": 1}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(tag = "kind", rename_all = "snake_case"))]
pub enum Type {
    /// An atom, by its index among the term's atoms.
    Atom {
        /// The atom's index.
        atom: u32,
    },
    /// `1`
    One,
    /// `⊤`, or true.
    Top,
    /// `0`, or false.
    Zero,
    /// `A ⊗ B`
    Tensor {
        /// `A`
        left: TypeId,
        /// `B`
        right: TypeId,
    },
    /// `A & B`, or `A ∧ B`.
    With {
        /// `A`
        left: TypeId,
        /// `B`
        right: TypeId,
    },
    /// `A ⊕ B`, or `A ∨ B`.
    Plus {
        /// `A`
        left: TypeId,
        /// `B`
        right: TypeId,
    },
    /// `A ⊸ B`, or `A → B`.
    Lollipop {
        /// `A`
        from: TypeId,
        /// `B`
        to: TypeId,
    },
    /// `!A`
    Bang {
        /// `A`
        of: TypeId,
    },
}

/// A variable of a term: its type and the name it is printed with.
///
/// A term names a variable by its [`VarId`], so two variables may have
/// one name. The name is an identifier, and digits at its end are
/// printed as an index: `x1` is `x₁` in Unicode.
///
/// A term read back from a proof names its variables itself: a variable
/// of a function type takes `f`, `g`, `h`, any other `x`, `y`, `z`, `u`,
/// `v`, `w`, in the order the variables are made; once the letters run
/// out, they start again with the next index (`x1`).
///
/// # JSON
///
/// With the feature `serialize`, `{"type": 0, "name": "x"}`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Variable {
    /// The type.
    #[cfg_attr(feature = "serialize", serde(rename = "type"))]
    ty: TypeId,
    /// The name.
    name: Box<str>,
}

impl Variable {
    /// Returns the variable's type.
    pub const fn ty(&self) -> TypeId {
        self.ty
    }

    /// Returns the variable's name, an index as digits at its end.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A node of a term.
///
/// A node names its subterms by their [`TermId`], and they come before it
/// in [`Term::nodes`]. A variable a node binds is in scope in its body,
/// and for `case` each branch's variable in that branch.
///
/// The enum is exhaustive, as the calculus is fixed: a `match` names every
/// node.
///
/// # JSON
///
/// With the feature `serialize`, an object tagged by `kind` in snake
/// case with the fields below: `{"kind": "lambda", "variable": 0,
/// "body": 1}`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(tag = "kind", rename_all = "snake_case"))]
pub enum Node {
    /// A variable.
    Var {
        /// The variable.
        variable: VarId,
    },
    /// `λx:A. M`, `A` the variable's type.
    Lambda {
        /// `x`
        variable: VarId,
        /// `M`
        body: TermId,
    },
    /// `M N`
    Apply {
        /// `M`
        function: TermId,
        /// `N`
        argument: TermId,
    },
    /// `M ⊗ N`
    Tensor {
        /// `M`
        left: TermId,
        /// `N`
        right: TermId,
    },
    /// `let M be x ⊗ y in N`
    LetTensor {
        /// `M`
        term: TermId,
        /// `x`
        left: VarId,
        /// `y`
        right: VarId,
        /// `N`
        body: TermId,
    },
    /// `∗`, of type `1`.
    Unit,
    /// `let M be ∗ in N`
    LetUnit {
        /// `M`
        term: TermId,
        /// `N`
        body: TermId,
    },
    /// `⟨M, N⟩`
    Pair {
        /// `M`
        left: TermId,
        /// `N`
        right: TermId,
    },
    /// `fst(M)`
    First {
        /// `M`
        term: TermId,
    },
    /// `snd(M)`
    Second {
        /// `M`
        term: TermId,
    },
    /// `inl(M)`, of the sum type given.
    Left {
        /// `M`
        term: TermId,
        /// The type `A ⊕ B` of the whole.
        sum: TypeId,
    },
    /// `inr(M)`, of the sum type given.
    Right {
        /// `M`
        term: TermId,
        /// The type `A ⊕ B` of the whole.
        sum: TypeId,
    },
    /// `case M of inl(x) → N ‖ inr(y) → P`
    Case {
        /// `M`
        term: TermId,
        /// `x`
        left: VarId,
        /// `N`
        left_body: TermId,
        /// `y`
        right: VarId,
        /// `P`
        right_body: TermId,
    },
    /// `true(M⃗)`, of type `⊤`, which uses up the terms `M⃗`. In the simple
    /// calculus it is the unit `⟨⟩`, and `M⃗` is empty.
    True {
        /// `M⃗`
        context: Box<[TermId]>,
    },
    /// `false_A(M⃗; N)`, of type `A`, with `N` of type `0`, which uses up
    /// the terms `M⃗`. In the simple calculus it is `abort(N)`, and `M⃗` is
    /// empty.
    False {
        /// `M⃗`
        context: Box<[TermId]>,
        /// `N`
        term: TermId,
        /// `A`
        goal: TypeId,
    },
    /// `promote M⃗ for x⃗ in N`
    Promote {
        /// `M⃗`, as many as `x⃗`.
        terms: Box<[TermId]>,
        /// `x⃗`
        variables: Box<[VarId]>,
        /// `N`
        body: TermId,
    },
    /// `derelict(M)`
    Derelict {
        /// `M`
        term: TermId,
    },
    /// `discard M in N`
    Discard {
        /// `M`
        term: TermId,
        /// `N`
        body: TermId,
    },
    /// `copy M as x, y in N`
    Copy {
        /// `M`
        term: TermId,
        /// `x`
        left: VarId,
        /// `y`
        right: VarId,
        /// `N`
        body: TermId,
    },
}

/// A lambda-term with its typing judgement: the variables it may use
/// freely, and its type.
///
/// [`Derivation::term`](crate::linear::Derivation::term) and
/// [`ordinary::Derivation::term`](crate::ordinary::Derivation::term) read
/// one back from a proof, and [`parse`](Self::parse) reads one from text.
/// [`check`](Self::check) says whether it is well typed. Print it with
/// `Display` (`term.to_string()`), [`write`](Self::write) or
/// [`write_judgement`](Self::write_judgement); the module docs have
/// examples.
///
/// A term holds:
/// - the variables of its hypotheses, in the order the sequent writes
///   them ([`context`](Self::context));
/// - its type ([`goal`](Self::goal));
/// - its nodes, each after the nodes it names, and the node that is the
///   whole term ([`root`](Self::root)).
///
/// Several nodes may name one node: a left rule's term goes to both
/// components of a pair, for example. The nodes are a DAG, and the
/// printed term writes a shared node at every place, so the text can be
/// much longer than the list of nodes.
///
/// # JSON
///
/// With the feature `serialize`, a document at wire level 23:
/// `{"version": 23, "calculus": "linear", "atoms": ["a"], "types":
/// [{"kind": "atom", "atom": 0}], "variables": [{"type": 0, "name":
/// "x"}], "context": [0], "goal": 0, "nodes": [{"kind": "var",
/// "variable": 0}], "root": 0}`.
/// [`wire::upgrade`](crate::wire::upgrade) reads one and checks its
/// shape: every index names what it should, and a type or a node comes
/// after what it names. [`check`](Self::check) then says whether the
/// term is well typed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term {
    /// The calculus.
    calculus: Calculus,
    /// The atoms' names.
    atoms: Vec<Box<str>>,
    /// The types, operands first.
    types: Vec<Type>,
    /// The variables.
    variables: Vec<Variable>,
    /// The variables of the hypotheses, in the order written.
    context: Vec<VarId>,
    /// The term's type.
    goal: TypeId,
    /// The nodes, operands first.
    nodes: Vec<Node>,
    /// The whole term.
    root: TermId,
}

impl Term {
    /// Returns the calculus the term belongs to.
    pub const fn calculus(&self) -> Calculus {
        self.calculus
    }

    /// Returns the names of the atoms the types name.
    pub fn atoms(&self) -> &[Box<str>] {
        &self.atoms
    }

    /// Returns the types, each after its operands.
    pub fn types(&self) -> &[Type] {
        &self.types
    }

    /// Returns the variables.
    pub fn variables(&self) -> &[Variable] {
        &self.variables
    }

    /// Returns the variables of the hypotheses, in the order the sequent
    /// writes them.
    pub fn context(&self) -> &[VarId] {
        &self.context
    }

    /// Returns the term's type: the sequent's goal, or `⊥` for an LJ
    /// sequent with nothing right of `⊢`.
    pub const fn goal(&self) -> TypeId {
        self.goal
    }

    /// Returns the nodes, each after the nodes it names.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Returns the node at `id`, an id of this term.
    ///
    /// # Panics
    ///
    /// If `id` is past this term's nodes, as an id of another term may be.
    pub fn node(&self, id: TermId) -> &Node {
        &self.nodes[id.index()]
    }

    /// Returns the whole term.
    pub const fn root(&self) -> TermId {
        self.root
    }
}

/// How a term is printed: its symbols, and whether `λ` shows types.
///
/// Every front end that shows a term takes this one value
/// ([`Settings::lambda`](crate::Settings::lambda)). Start from
/// `Options::default()` and change what differs:
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::{Limits, lambda::{Notation, Options}};
/// use deduxit::linear::{Mode, Options as SearchOptions, Sequent, Verdict, prove};
///
/// let sequent: Sequent = "|- A -o A".parse()?;
/// let outcome = prove(&sequent, Mode::INTUITIONISTIC, &SearchOptions::default())?;
/// let Verdict::Proved(proof) = &outcome.verdict else {
///     panic!("the sequent is provable");
/// };
/// let term = proof.derivation()?.term(&Limits::default(), |_| false)?;
///
/// let options = Options::default()
///     .with_notation(Notation::Ascii)
///     .with_types(true);
/// let mut text = String::new();
/// term.write_judgement(&mut text, &options, &Limits::default(), |_| false)?;
/// assert_eq!(text, r"|- \x:A. x : A -o A");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # JSON
///
/// With the feature `serialize`, `{"notation": "unicode", "types":
/// false}`; a key left out keeps its default, and an unknown one is
/// refused.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The symbols, Unicode by default ([`DEFAULT_NOTATION`](Self::DEFAULT_NOTATION)).
    pub notation: Notation,
    /// Whether every `λ` shows its variable's type (`λx:A. M`), and
    /// `inl`, `inr`, `false` and `abort` the type they give (`false_A`).
    /// Off by default ([`DEFAULT_TYPES`](Self::DEFAULT_TYPES)), since the
    /// sequent beside the term shows the types. A term written with it
    /// reads back ([`Term::parse`]).
    pub types: bool,
}

impl Options {
    /// The symbols by default.
    pub const DEFAULT_NOTATION: Notation = Notation::Unicode;

    /// Whether `λ` shows types by default.
    pub const DEFAULT_TYPES: bool = false;

    /// Returns the options with another [`notation`](Self::notation).
    #[must_use]
    pub const fn with_notation(self, notation: Notation) -> Self {
        Self { notation, ..self }
    }

    /// Returns the options with another [`types`](Self::types).
    #[must_use]
    pub const fn with_types(self, types: bool) -> Self {
        Self { types, ..self }
    }
}

impl Default for Options {
    /// Returns Unicode symbols, without types on `λ`.
    fn default() -> Self {
        Self {
            notation: Self::DEFAULT_NOTATION,
            types: Self::DEFAULT_TYPES,
        }
    }
}

/// The symbols a term is printed with.
///
/// # JSON
///
/// With the feature `serialize`, `"unicode"` or `"ascii"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Notation {
    /// The calculus's own symbols: `λx. M`, `M ⊗ N`, `⟨M, N⟩`, `∗`, and
    /// an index as a subscript (`x₁`).
    #[default]
    Unicode,
    /// ASCII alone: `\x. M`, `M * N`, `<M, N>`, `()`, `x1`, and the
    /// types as the parsers read them (`-o`, `->`).
    Ascii,
}

impl Notation {
    /// Every notation, in the order of their names.
    pub const ALL: &'static [Self] = &[Self::Unicode, Self::Ascii];

    /// The names, as the JSON form and the command write them.
    pub const NAMES: &'static [&'static str] = &["unicode", "ascii"];

    /// Returns the notation's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Unicode => "unicode",
            Self::Ascii => "ascii",
        }
    }
}

impl std::str::FromStr for Notation {
    type Err = crate::Error;

    /// Reads a notation by its name.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .copied()
            .find(|n| n.name() == s)
            .ok_or_else(|| crate::Error::UnknownName {
                what: "term notation",
                name: s.into(),
                known: Self::NAMES,
            })
    }
}

/// Why a proof or a derivation has no lambda-term: the `reason` of
/// [`Error::NoTerm`](crate::Error::NoTerm).
///
/// # JSON
///
/// With the feature `serialize`, its name in snake case, as the error's
/// `reason`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "snake_case"))]
pub enum NoTerm {
    /// A one-sided proof of classical linear logic, or a derivation of
    /// LK. A sequent with several conclusions is no type.
    Classical,
    /// A proof with Mix, which no intuitionistic calculus has.
    Mix,
    /// A proof of the cyclic or the Lambek calculus. Its term would be one
    /// of an ordered calculus, which deduxit does not write.
    Ordered,
    /// A proof with quantifiers over first-order terms, which are no
    /// quantifiers over types.
    Quantified,
    /// A proof in progress, with open goals.
    Open,
    /// A derivation in a calculus given as data, whose rules have no
    /// terms.
    Calculus,
}

impl std::fmt::Display for NoTerm {
    /// Says why there is no term.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Classical => {
                "a proof of classical logic may have several conclusions, which no lambda-term types; a term needs an intuitionistic proof"
            }
            Self::Mix => "Mix has no lambda-term",
            Self::Ordered => {
                "the cyclic and the Lambek calculus have the terms of an ordered calculus, not these"
            }
            Self::Quantified => {
                "quantifiers over first-order terms are not quantifiers over types"
            }
            Self::Open => "the proof has open goals",
            Self::Calculus => "the rules of a calculus given as data have no lambda-terms",
        })
    }
}

/// The words that no variable is named by: the keywords of the terms'
/// syntax.
#[cfg(any(feature = "parse", feature = "serialize"))]
const KEYWORDS: [&str; 18] = [
    "let", "be", "in", "case", "of", "inl", "inr", "fst", "snd", "true", "false", "abort",
    "promote", "for", "derelict", "discard", "copy", "as",
];

/// The keywords that take the type they give as a subscript, `inl_A`:
/// a word that starts with one and `_` is no variable's name either.
#[cfg(any(feature = "parse", feature = "serialize"))]
const SUBSCRIPTED: [&str; 4] = ["inl", "inr", "abort", "false"];

/// Returns whether `word` can name a variable, so that the printed term
/// reads back: an identifier that is no keyword, starts with no `λ` and
/// is no keyword with a subscript.
#[cfg(any(feature = "parse", feature = "serialize"))]
fn names_variable(word: &str) -> bool {
    crate::name::is_identifier(word)
        && !KEYWORDS.contains(&word)
        && !word.starts_with('λ')
        && !SUBSCRIPTED
            .iter()
            .any(|k| word.strip_prefix(k).is_some_and(|w| w.starts_with('_')))
}

/// An arena of types in which two equal types are one id, so that types
/// are compared as numbers.
#[derive(Clone, Debug, Default)]
pub(crate) struct Types {
    /// The types, operands first.
    list: Vec<Type>,
    /// The id of every type.
    ids: HashMap<Type, TypeId>,
}

impl Types {
    /// Returns the arena of a term's types, which holds no type twice,
    /// with room for `more`, so that adding them moves nothing.
    fn of(types: &[Type], more: usize) -> Self {
        let room = types.len().saturating_add(more);
        let mut list = Vec::with_capacity(room);
        list.extend_from_slice(types);
        let mut ids = HashMap::with_capacity_and_hasher(room, Default::default());
        // A term's types are fewer than 2³²: its ids are `u32`.
        ids.extend(
            types
                .iter()
                .enumerate()
                .map(|(i, &ty)| (ty, TypeId(i as u32))),
        );
        Self { list, ids }
    }

    /// Returns the id of a type, added if new.
    pub(crate) fn add(&mut self, ty: Type) -> TypeId {
        if let Some(&id) = self.ids.get(&ty) {
            return id;
        }
        // A read-back adds one type per occurrence or ordinary formula at
        // most, three in the simple calculus; a reader counts what it
        // adds against its limit, which a `u32` holds; the checker adds
        // one type per node of a term whose nodes a `u32` numbers, to
        // types a `u32` numbers. A sum past 2³² would need both tables
        // near their ends, which the reader's limit keeps them from.
        let id = TypeId(self.list.len() as u32);
        self.list.push(ty);
        self.ids.insert(ty, id);
        id
    }

    /// Returns the type at `id`.
    pub(crate) fn get(&self, id: TypeId) -> Type {
        self.list[id.index()]
    }
}

/// What the read-backs and the readers fill while they build a term: the
/// atoms, the types (two equal types are one id), the variables with
/// their names, and the nodes.
#[derive(Clone, Debug)]
struct Builder {
    /// The atoms' names and their indices.
    atoms: Vec<Box<str>>,
    /// The index of every atom's name.
    atom_ids: HashMap<Box<str>, u32>,
    /// The types.
    types: Types,
    /// The variables.
    variables: Vec<Variable>,
    /// How many variables of functions and of other types were named.
    named: [u32; 2],
    /// The nodes.
    nodes: Vec<Node>,
}

/// The letters of a function's variables and of others', taken in turn.
const LETTERS: [&[char]; 2] = [&['f', 'g', 'h'], &['x', 'y', 'z', 'u', 'v', 'w']];

impl Builder {
    /// Returns an empty builder.
    fn new() -> Self {
        Self {
            atoms: Vec::new(),
            atom_ids: HashMap::default(),
            types: Types::default(),
            variables: Vec::new(),
            named: [0; 2],
            nodes: Vec::new(),
        }
    }

    /// Returns the index of the atom named `name`, added if new.
    fn atom(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.atom_ids.get(name) {
            return id;
        }
        // Fewer atoms than occurrences, which are fewer than 2³².
        let id = self.atoms.len() as u32;
        self.atoms.push(name.into());
        self.atom_ids.insert(name.into(), id);
        id
    }

    /// Returns the id of a type, added if new.
    fn ty(&mut self, ty: Type) -> TypeId {
        self.types.add(ty)
    }

    /// Returns a new variable of type `ty` and the node that uses it,
    /// named by the next letter for a function or for any other type.
    fn variable(&mut self, ty: TypeId) -> (VarId, TermId) {
        let class = usize::from(!matches!(self.types.get(ty), Type::Lollipop { .. }));
        let letters = LETTERS[class];
        let n = self.named[class];
        self.named[class] = n.saturating_add(1);
        let letters_len = letters.len() as u32;
        let letter = letters[(n % letters_len) as usize];
        let name = match n / letters_len {
            0 => letter.to_string(),
            index => format!("{letter}{index}"),
        };
        self.named_variable(ty, &name)
    }

    /// Returns a new variable of type `ty` named `name`, and the node
    /// that uses it.
    fn named_variable(&mut self, ty: TypeId, name: &str) -> (VarId, TermId) {
        // A read-back makes a variable per hypothesis of a sequent of
        // the derivation, at most two for one (a contraction's copies),
        // so this truncates only for a derivation of more than 2³¹
        // hypotheses; a reader counts its variables against its limit.
        let id = VarId(self.variables.len() as u32);
        self.variables.push(Variable {
            ty,
            name: name.into(),
        });
        (id, self.node(Node::Var { variable: id }))
    }

    /// Adds a node and returns its id.
    fn node(&mut self, node: Node) -> TermId {
        // At most two nodes per inference, one per formula a compact run
        // discards or copies, and one per variable (`linear::read` and
        // `simple::read` count them), so far fewer than 2³² for a
        // derivation held in memory.
        let id = TermId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }

    /// Returns the term whose whole is `root`, with only the nodes `root`
    /// reaches, in their order. A node that no rule above used, such as a
    /// weakened variable's, is dropped.
    fn finish(self, calculus: Calculus, context: Vec<VarId>, goal: TypeId, root: TermId) -> Term {
        let mut reached = vec![false; self.nodes.len()];
        reached[root.index()] = true;
        for i in (0..self.nodes.len()).rev() {
            if reached[i] {
                operands(&self.nodes[i], |t| reached[t.index()] = true);
            }
        }
        let mut renumbered = vec![TermId(0); self.nodes.len()];
        let mut nodes = Vec::with_capacity(reached.iter().filter(|&&r| r).count());
        for (i, mut node) in self.nodes.into_iter().enumerate() {
            if reached[i] {
                renumber(&mut node, |t| renumbered[t.index()]);
                renumbered[i] = TermId(nodes.len() as u32);
                nodes.push(node);
            }
        }
        Term {
            calculus,
            atoms: self.atoms,
            types: self.types.list,
            variables: self.variables,
            context,
            goal,
            root: renumbered[root.index()],
            nodes,
        }
    }
}

/// Hands every subterm a node names to `f`, in the order written.
pub(crate) fn operands(node: &Node, mut f: impl FnMut(TermId)) {
    match node {
        Node::Var { .. } | Node::Unit => {}
        Node::Lambda { body, .. } => f(*body),
        Node::Apply { function, argument } => {
            f(*function);
            f(*argument);
        }
        Node::Tensor { left, right } | Node::Pair { left, right } => {
            f(*left);
            f(*right);
        }
        Node::LetTensor { term, body, .. }
        | Node::LetUnit { term, body }
        | Node::Discard { term, body }
        | Node::Copy { term, body, .. } => {
            f(*term);
            f(*body);
        }
        Node::First { term }
        | Node::Second { term }
        | Node::Left { term, .. }
        | Node::Right { term, .. }
        | Node::Derelict { term } => f(*term),
        Node::Case {
            term,
            left_body,
            right_body,
            ..
        } => {
            f(*term);
            f(*left_body);
            f(*right_body);
        }
        Node::True { context } => context.iter().copied().for_each(f),
        Node::False { context, term, .. } => {
            context.iter().copied().for_each(&mut f);
            f(*term);
        }
        Node::Promote { terms, body, .. } => {
            terms.iter().copied().for_each(&mut f);
            f(*body);
        }
    }
}

/// Replaces every subterm a node names by what `f` maps it to.
fn renumber(node: &mut Node, f: impl Fn(TermId) -> TermId) {
    match node {
        Node::Var { .. } | Node::Unit => {}
        Node::Lambda { body, .. } => *body = f(*body),
        Node::Apply { function, argument } => {
            *function = f(*function);
            *argument = f(*argument);
        }
        Node::Tensor { left, right } | Node::Pair { left, right } => {
            *left = f(*left);
            *right = f(*right);
        }
        Node::LetTensor { term, body, .. }
        | Node::LetUnit { term, body }
        | Node::Discard { term, body }
        | Node::Copy { term, body, .. } => {
            *term = f(*term);
            *body = f(*body);
        }
        Node::First { term }
        | Node::Second { term }
        | Node::Left { term, .. }
        | Node::Right { term, .. }
        | Node::Derelict { term } => *term = f(*term),
        Node::Case {
            term,
            left_body,
            right_body,
            ..
        } => {
            *term = f(*term);
            *left_body = f(*left_body);
            *right_body = f(*right_body);
        }
        Node::True { context } => context.iter_mut().for_each(|t| *t = f(*t)),
        Node::False { context, term, .. } => {
            context.iter_mut().for_each(|t| *t = f(*t));
            *term = f(*term);
        }
        Node::Promote { terms, body, .. } => {
            terms.iter_mut().for_each(|t| *t = f(*t));
            *body = f(*body);
        }
    }
}

/// Returns the refusal of a read-back that `stop` ended.
const fn stopped() -> crate::Error {
    crate::Error::Refused(Refusal::Stopped {
        phase: crate::limits::Phase::ReadBack,
    })
}
