// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Proofs read back as lambda-terms: a proof of intuitionistic linear
//! logic as a term of the linear lambda calculus of Benton, Bierman, de
//! Paiva and Hyland, a derivation of LJ as a term of the simply typed
//! lambda calculus with products, sums, a unit and an empty type.
//!
//! The term is computed from a derivation the checker has accepted
//! ([`Derivation::term`](crate::Derivation::term),
//! [`ordinary::Derivation::term`](crate::ordinary::Derivation::term)),
//! one construction per rule, and is printed, never trusted. The linear
//! calculus is the sequent calculus term assignment of G. M. Bierman,
//! *On Intuitionistic Linear Logic* (PhD thesis, Cambridge, 1993,
//! published as technical report UCAM-CL-TR-346, 1994), chapter 3,
//! figure 3.2, which completes Benton, Bierman, de Paiva and Hyland, "A
//! term calculus for intuitionistic linear logic" (TLCA 1993, LNCS 664)
//! with the additives and their units:
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
//! A left rule substitutes (the thesis's `M[x := …]`), so that a
//! cut-free proof reads back as a term in normal form, and a cut is a
//! redex, whose reduction is the thesis's substitution: cut elimination
//! is reduction. A proof of affine linear logic reads back to the same
//! terms with weakening left implicit: a variable may go unused
//! ([`Calculus::Affine`]). An LJ derivation reads back to the simply
//! typed calculus of Girard, Lafont and Taylor, *Proofs and Types* (1989),
//! sections 3.1.2 and 10.6, with their `π₁`, `π₂`, `ι₁`, `ι₂`, `δ` and `ε`
//! written `fst`, `snd`, `inl`, `inr`, `case` and `abort`, and `⟨⟩` for
//! the unit; `¬A` is `A → ⊥` and `A ↔ B` the pair of both implications.
//!
//! A term is self-contained: its own atoms, types and variables, its
//! nodes in an arena whose operands precede them, so that no walk over it
//! needs recursion and its JSON form nests no deeper than one node. What
//! has no term is refused with its reason ([`NoTerm`]): a proof of
//! classical linear logic, with Mix, of the cyclic or the Lambek calculus,
//! with quantifiers, or with open goals, and a derivation of LK.

// A variant a later step adds must not fall into an existing arm.
#![deny(clippy::wildcard_enum_match_arm)]

pub(crate) mod linear;
pub(crate) mod print;
pub(crate) mod simple;
#[cfg(test)]
mod tests;

use crate::hash::HashMap;
use crate::limits::Refusal;

/// The index of a node in a [`Term`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize), serde(transparent))]
pub struct TermId(u32);

impl TermId {
    /// Returns the index as a `usize`, for indexing the nodes.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The index of a variable in a [`Term`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize), serde(transparent))]
pub struct VarId(u32);

impl VarId {
    /// Returns the index as a `usize`, for indexing the variables.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The index of a type in a [`Term`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize), serde(transparent))]
pub struct TypeId(u32);

impl TypeId {
    /// Returns the index as a `usize`, for indexing the types.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The calculus a term belongs to.
///
/// # JSON
///
/// With the feature `serialize`, `"linear"`, `"affine"` or `"simple"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Calculus {
    /// The linear lambda calculus: every variable used exactly once, but
    /// those under `discard`, `copy`, `true` and `false`.
    Linear,
    /// The affine lambda calculus: the linear one, a variable used at most
    /// once, which is what a proof that weakens a formula of no `!` reads
    /// back to.
    Affine,
    /// The simply typed lambda calculus, any variable used any number of
    /// times.
    Simple,
}

/// A type of a term: in the linear calculi a formula of intuitionistic
/// linear logic, in the simple one an ordinary formula, `With` its
/// conjunction `∧`, `Plus` its disjunction `∨`, `Lollipop` its
/// implication `→`, `Top` true and `Zero` false. Its operands precede it
/// in the term's types, and an equal type is the same id.
///
/// # JSON
///
/// With the feature `serialize`, an object tagged by `kind` in snake
/// case, its operands by their index: `{"kind": "lollipop", "from": 0,
/// "to": 1}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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

/// A variable of a term: its type and the name it is printed with, a
/// letter and an index (none for 0).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Variable {
    /// The type.
    #[cfg_attr(feature = "serialize", serde(rename = "type"))]
    ty: TypeId,
    /// The letter of the name.
    letter: char,
    /// The index after the letter, none for 0.
    index: u32,
}

impl Variable {
    /// Returns the variable's type.
    pub const fn ty(&self) -> TypeId {
        self.ty
    }

    /// Returns the letter of the variable's name.
    pub const fn letter(&self) -> char {
        self.letter
    }

    /// Returns the index the name carries after its letter, 0 for none.
    pub const fn index(&self) -> u32 {
        self.index
    }
}

/// A node of a term, which names its subterms by their index in the
/// term's nodes, each before it. The variables a node binds scope over
/// its body. The type is closed, as the calculus is: a quantifier would be
/// a variant of its own, with a type variable among the types.
///
/// # JSON
///
/// With the feature `serialize`, an object tagged by `kind` in snake
/// case with the fields below: `{"kind": "lambda", "variable": 0,
/// "body": 1}`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
    /// `true(M⃗)`, of type `⊤`, which consumes the terms given; in the
    /// simple calculus `⟨⟩`, with none.
    True {
        /// `M⃗`
        context: Box<[TermId]>,
    },
    /// `false_A(M⃗; N)`, of type `A`, `N` of type `0`, which consumes the
    /// terms given; in the simple calculus `abort(N)`, with none.
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

/// A lambda-term read back from a derivation, with its typing: the
/// variables of its hypotheses in the order the sequent writes them
/// (`context`), its type (`goal`), and its nodes, each after the nodes it
/// names, `root` the whole term. A node may be named by several others
/// (a term a left rule substituted into both components of a pair), so
/// the nodes form a DAG that the printed term unfolds.
///
/// # JSON
///
/// With the feature `serialize`, written only: `{"calculus": "linear",
/// "atoms": ["a"], "types": [{"kind": "atom", "atom": 0}], "variables":
/// [{"type": 0, "letter": "x", "index": 0}], "context": [0], "goal": 0,
/// "nodes": [{"kind": "var", "variable": 0}], "root": 0}`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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

    /// Returns the node at `id`, which must belong to this term.
    ///
    /// # Panics
    ///
    /// For an id of another term, past its end.
    pub fn node(&self, id: TermId) -> &Node {
        &self.nodes[id.index()]
    }

    /// Returns the whole term.
    pub const fn root(&self) -> TermId {
        self.root
    }
}

/// How a term is printed: the one options value of every front end that
/// shows one.
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
    /// Whether every binder carries its variable's type (`λx:A. M`) and
    /// `false` its goal (`false_A`), as the calculus writes them; off by
    /// default ([`DEFAULT_TYPES`](Self::DEFAULT_TYPES)), since the
    /// sequent shows the types.
    pub types: bool,
}

impl Options {
    /// The symbols by default.
    pub const DEFAULT_NOTATION: Notation = Notation::Unicode;

    /// Whether binders carry types by default.
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
    /// Returns Unicode symbols without types on binders.
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
    /// The calculus's own: `λx. M`, `M ⊗ N`, `⟨M, N⟩`, `∗`, an index as
    /// a subscript (`x₁`).
    #[default]
    Unicode,
    /// ASCII alone: `\x. M`, `M * N`, `<M, N>`, `()`, `x1`, and the
    /// types as the parser reads them (`-o`, `->`).
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

/// Why a proof or a derivation has no lambda-term.
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
    /// LK: no lambda-term, since a sequent with several conclusions is no
    /// type.
    Classical,
    /// A proof with Mix, which no intuitionistic calculus has.
    Mix,
    /// A proof of the cyclic or the Lambek calculus, whose terms are
    /// those of an ordered calculus of their own.
    Ordered,
    /// A proof with quantifiers over first-order terms, which are no
    /// quantifiers over types.
    Quantified,
    /// A proof in progress, with open goals.
    Open,
}

impl std::fmt::Display for NoTerm {
    /// Says why there is no term.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Classical => {
                "a proof of classical logic has several conclusions, which no lambda-term types"
            }
            Self::Mix => "Mix has no lambda-term",
            Self::Ordered => {
                "the cyclic and the Lambek calculus have the terms of an ordered calculus, not these"
            }
            Self::Quantified => {
                "quantifiers over first-order terms are no quantifiers over types"
            }
            Self::Open => "the proof has open goals",
        })
    }
}

/// What the read-backs share while they build a term: the arena of
/// types, hash-consed so that an equal type is one id, the variables
/// with the names they get, and the nodes.
#[derive(Debug)]
struct Builder {
    /// The atoms' names and their indices.
    atoms: Vec<Box<str>>,
    /// The index of every atom's name.
    atom_ids: HashMap<Box<str>, u32>,
    /// The types.
    types: Vec<Type>,
    /// The id of every type.
    type_ids: HashMap<Type, TypeId>,
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
            types: Vec::new(),
            type_ids: HashMap::default(),
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
        if let Some(&id) = self.type_ids.get(&ty) {
            return id;
        }
        // One type per occurrence or ordinary formula at most, each fewer
        // than 2³²; the simple calculus adds at most three per formula.
        let id = TypeId(self.types.len() as u32);
        self.types.push(ty);
        self.type_ids.insert(ty, id);
        id
    }

    /// Returns a new variable of type `ty` and the node that uses it,
    /// named by the next letter for a function or for any other type.
    fn variable(&mut self, ty: TypeId) -> (VarId, TermId) {
        let class = usize::from(!matches!(self.types[ty.index()], Type::Lollipop { .. }));
        let letters = LETTERS[class];
        let n = self.named[class];
        self.named[class] = n.saturating_add(1);
        let letters_len = letters.len() as u32;
        // Fewer variables than inferences, each of which binds two at most.
        let id = VarId(self.variables.len() as u32);
        self.variables.push(Variable {
            ty,
            letter: letters[(n % letters_len) as usize],
            index: n / letters_len,
        });
        (id, self.node(Node::Var { variable: id }))
    }

    /// Adds a node and returns its id.
    fn node(&mut self, node: Node) -> TermId {
        // Fewer nodes than twice the inferences, plus the variables.
        let id = TermId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }

    /// Returns the term rooted at `root`, its nodes those `root` reaches
    /// in their order: a node a rule made and the rules above left unused
    /// (a weakened variable's) goes.
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
            types: self.types,
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
        phase: crate::limits::Phase::View,
    })
}
