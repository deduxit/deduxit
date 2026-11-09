// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Proofs: the proof terms the engines return, the checker that tests a
//! term against its sequent, and the derivations that show a proof to
//! people and to the exporters.
//!
//! A [`Proof`] is an array of [`Node`]s, one per rule application, over
//! the [`Forest`] of the sequent it proves. A node names the rule, the
//! occurrence the rule acts on and the nodes that prove its premises.
//!
//! The rules are those of the dyadic sequent calculus that every engine
//! searches in. It has the standard rules, but handles the `?` formulas
//! through an unrestricted zone `Θ`: a `?` step moves a formula into `Θ`,
//! and a copy uses a formula of `Θ` without consuming it. Weakening (affine
//! mode), Mix, the cut and the quantifier rules complete the set.
//!
//! A node does not record the sequent it proves. The checker computes it
//! from the premises, and the derivation shows it.

pub mod check;
pub mod cut;
pub mod derivation;
/// First-order proofs written by hand, and their tests.
#[cfg(all(test, feature = "parse"))]
mod first_order;
/// Text rendering of derivations.
pub(crate) mod fmt;
#[cfg(feature = "interactive")]
pub mod interactive;
/// Multisets of occurrence ids.
mod multiset;
/// A second, independent checker that the tests compare the checker with.
#[cfg(all(test, feature = "parse"))]
mod oracle;
/// The rules of the standard calculus, one-sided and named two-sided.
mod rule;
pub mod size;
pub mod style;

use crate::linear::occurrences::{Instances, instances};
pub use check::{CheckError, Dyadic, Fault, Invalid, Refused};
pub use derivation::{
    ClassicalRule, ClassicalSides, Compact, Derivation, InfId, Inference, Sides, Structural,
    TwoSided, ViewOptions,
};
pub use fmt::TextOptions;
#[cfg(feature = "interactive")]
pub use interactive::{CutFormula, GoalId, Interactive, Step, StepError};
pub use rule::{Division, Named, Rule};
pub use size::Size;
pub use style::{Labels, OpenGoal};

use crate::Error;
use crate::limits::{Limits, Progress};
use crate::linear::fragment::Mode;
use crate::linear::occurrences::{Forest, Member, OccId, Reading};
use crate::linear::sequents::{Eigen, FoTermId, Sequent};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A number of bytes, printed in the largest binary unit that divides it
/// exactly: `1 GiB`, `512 MiB`, `1500 B`.
pub(crate) struct Bytes(pub(crate) u64);

impl std::fmt::Display for Bytes {
    /// Writes the number with its unit.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let units = [(30, "GiB"), (20, "MiB"), (10, "KiB")];
        match units
            .into_iter()
            .find(|&(shift, _)| self.0 != 0 && self.0.is_multiple_of(1 << shift))
        {
            Some((shift, unit)) => write!(f, "{} {unit}", self.0 >> shift),
            None => write!(f, "{} B", self.0),
        }
    }
}

/// The index of a node in a proof's array of nodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(u32);

impl NodeId {
    /// Wraps a raw index.
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the raw index.
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Returns the index as a `usize`, for indexing the nodes.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Which subformula of a binary connective a rule picks.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Branch {
    /// The left subformula.
    Left,
    /// The right subformula.
    Right,
}

/// One rule application in a proof: the rule, the member it acts on, and
/// the nodes that prove its premises, which come before it in the proof.
///
/// The rules are those of the dyadic calculus. Below, `⊢ Θ ; Γ` is a
/// sequent with the unrestricted zone `Θ` and the linear zone `Γ`. `Θ` is
/// empty until a [`Quest`](Self::Quest) puts a formula in it.
///
/// The enum is exhaustive, so adding a rule is a breaking change. A new
/// rule needs its tag, an arm in the checker, a rule in the derivation
/// view, and a constructor in every mirror of the type
/// ([`NAMES`](Self::NAMES)).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    /// The axiom `⊢ Θ ; a, ~a`, on two occurrences of one atom with
    /// opposite signs, in either order.
    Ax(Member, Member),
    /// `⊢ Θ ; Γ, Δ, A ⊗ B` from `⊢ Θ ; Γ, A` and `⊢ Θ ; Δ, B`, on the
    /// occurrence of `A ⊗ B`. The first premise proves the left
    /// subformula, the second the right one.
    Tensor(Member, NodeId, NodeId),
    /// `⊢ Θ ; Γ, A ⅋ B` from `⊢ Θ ; Γ, A, B`.
    Par(Member, NodeId),
    /// `⊢ Θ ; 1`.
    One(Member),
    /// `⊢ Θ ; Γ, ⊥` from `⊢ Θ ; Γ`.
    Bot(Member, NodeId),
    /// `⊢ Θ ; Γ, A & B` from `⊢ Θ ; Γ, A` and `⊢ Θ ; Γ, B`, in that order.
    With(Member, NodeId, NodeId),
    /// `⊢ Θ ; Γ, A ⊕ B` from `⊢ Θ ; Γ, A` or from `⊢ Θ ; Γ, B`, as the
    /// [`Branch`] says.
    Plus(Member, Branch, NodeId),
    /// `⊢ Θ ; Γ, ⊤` for any `Γ`.
    Top(Member),
    /// Promotion: `⊢ Θ ; !A` from `⊢ Θ ; A`. The linear zone must be empty.
    Bang(Member, NodeId),
    /// `⊢ Θ ; Γ, ?A` from `⊢ Θ, A ; Γ`: the subformula `A` moves into the
    /// unrestricted zone.
    Quest(Member, NodeId),
    /// A copy: `⊢ Θ, A ; Γ` from `⊢ Θ, A ; Γ, A`, on the occurrence of `A`,
    /// which stays in `Θ`. It is the only rule that can repeat an
    /// occurrence in a branch, which is why the linear zone is a multiset.
    Copy(Member, NodeId),
    /// Weakening: `⊢ Θ ; Γ, A` from `⊢ Θ ; Γ`. Allowed in affine mode, and
    /// in every mode when `A` is a `?` formula. That case is the standard
    /// `?w`, which the dyadic calculus also gets as a
    /// [`Quest`](Self::Quest) whose formula goes unused.
    Weaken(Member, NodeId),
    /// Mix, when the mode allows it: `⊢ Θ ; Γ, Δ` from `⊢ Θ ; Γ` and
    /// `⊢ Θ ; Δ`.
    Mix(NodeId, NodeId),
    /// The cut: `⊢ Θ ; Γ, Δ` from `⊢ Θ ; Γ, A` and `⊢ Θ ; Δ, A⊥`.
    ///
    /// The member is an occurrence in a cut formula `A` of the sequent
    /// ([`Sequent::cuts`](crate::linear::Sequent::cuts)). The second
    /// premise consumes its dual in `A⊥`
    /// ([`Forest::dual`](crate::linear::Forest::dual)). An ordered mode
    /// refuses the cut. In intuitionistic mode, `Γ ⊢ A` and `Δ, A ⊢ C`
    /// give `Γ, Δ ⊢ C`, whichever premise has the cut formula as its goal.
    Cut(Member, NodeId, NodeId),
    /// `⊢ Θ ; Γ, ∀x.A` from `⊢ Θ ; Γ, A[a/x]`, on the occurrence of
    /// `∀x.A`, with the eigenvariable `a`.
    ///
    /// The premise consumes the body's instance under the frame of `∀x.A`
    /// with `a` added; the proof's instance table ([`Instances`]) numbers
    /// it. `a` must not occur in the conclusion. The checker ensures this
    /// with a condition on the whole proof: one such node introduces each
    /// eigenvariable, and only witnesses above that node contain it.
    Forall(Member, Eigen, NodeId),
    /// `⊢ Θ ; Γ, ∃x.A` from `⊢ Θ ; Γ, A[t/x]`, on the occurrence of
    /// `∃x.A`, with the witness `t`.
    ///
    /// `t` is a closed term of the sequent or of the proof's instance
    /// table. The premise consumes the body's instance under the frame of
    /// `∃x.A` with `t` added.
    Exists(Member, FoTermId, NodeId),
}

/// A node is a tag, a member and at most two node indices: 16 bytes.
const _: () = assert!(size_of::<Node>() == 16);

/// How many nodes [`Proof::build`] passes over between two calls of its
/// stop: some tenths of a millisecond of work.
const BUILD_POLL: usize = 1 << 16;

impl Node {
    /// The variants' names, in declaration order: one constructor each for
    /// a mirror of the type in another language to match.
    pub const NAMES: &'static [&'static str] = &[
        "Ax", "Tensor", "Par", "One", "Bot", "With", "Plus", "Top", "Bang", "Quest", "Copy",
        "Weaken", "Mix", "Cut", "Forall", "Exists",
    ];

    /// The rules' tags in a proof's JSON form, which are also their
    /// [`name`](Self::name)s: one per variant, and two for `Plus`, whose
    /// tag holds its branch.
    pub const TAGS: &'static [&'static str] = &[
        "ax", "⊗", "⅋", "1", "⊥", "&", "⊕₁", "⊕₂", "⊤", "!", "?", "copy", "wk", "mix", "cut", "∀",
        "∃",
    ];

    /// Returns the member the rule acts on: the first literal of an axiom,
    /// and `None` for Mix and the cut.
    pub const fn principal(self) -> Option<Member> {
        use Node::*;
        match self {
            Ax(o, _)
            | Tensor(o, ..)
            | Par(o, _)
            | One(o)
            | Bot(o, _)
            | With(o, ..)
            | Plus(o, ..)
            | Top(o)
            | Bang(o, _)
            | Quest(o, _)
            | Copy(o, _)
            | Weaken(o, _)
            | Forall(o, ..)
            | Exists(o, ..) => Some(o),
            Mix(..) | Cut(..) => None,
        }
    }

    /// Returns the members the node names: two for an axiom, none for Mix,
    /// and one otherwise. The cut names its occurrence in the cut formula.
    pub fn members(self) -> impl Iterator<Item = Member> {
        use Node::*;
        let second = match self {
            Ax(_, b) | Cut(b, ..) => Some(b),
            Tensor(..) | Par(..) | One(_) | Bot(..) | With(..) | Plus(..) | Top(_) | Bang(..)
            | Quest(..) | Copy(..) | Weaken(..) | Mix(..) | Forall(..) | Exists(..) => None,
        };
        self.principal().into_iter().chain(second)
    }

    /// Returns the node with every member it names passed through
    /// `rename`, or the first error `rename` returns.
    pub(crate) fn renamed<E>(
        self,
        rename: impl Fn(Member) -> Result<Member, E>,
    ) -> Result<Self, E> {
        use Node::*;
        Ok(match self {
            Ax(a, b) => Ax(rename(a)?, rename(b)?),
            Tensor(o, l, r) => Tensor(rename(o)?, l, r),
            Par(o, p) => Par(rename(o)?, p),
            One(o) => One(rename(o)?),
            Bot(o, p) => Bot(rename(o)?, p),
            With(o, l, r) => With(rename(o)?, l, r),
            Plus(o, b, p) => Plus(rename(o)?, b, p),
            Top(o) => Top(rename(o)?),
            Bang(o, p) => Bang(rename(o)?, p),
            Quest(o, p) => Quest(rename(o)?, p),
            Copy(o, p) => Copy(rename(o)?, p),
            Weaken(o, p) => Weaken(rename(o)?, p),
            Mix(l, r) => Mix(l, r),
            Cut(a, l, r) => Cut(rename(a)?, l, r),
            Forall(o, a, p) => Forall(rename(o)?, a, p),
            Exists(o, w, p) => Exists(rename(o)?, w, p),
        })
    }

    /// Returns the nodes that prove the premises, in the rule's order: none
    /// for an axiom, `1` or `⊤`, two for `⊗`, `&`, Mix and the cut, one
    /// otherwise.
    pub fn premises(self) -> impl Iterator<Item = NodeId> {
        use Node::*;
        let (first, second) = match self {
            Ax(..) | One(_) | Top(_) => (None, None),
            Tensor(_, l, r) | With(_, l, r) | Mix(l, r) | Cut(_, l, r) => (Some(l), Some(r)),
            Par(_, p)
            | Bot(_, p)
            | Plus(_, _, p)
            | Bang(_, p)
            | Quest(_, p)
            | Copy(_, p)
            | Weaken(_, p)
            | Forall(_, _, p)
            | Exists(_, _, p) => (Some(p), None),
        };
        first.into_iter().chain(second)
    }

    /// Returns the rule's name: `ax`, `⊗`, `⅋`, `1`, `⊥`, `&`, `⊕₁`, `⊕₂`,
    /// `⊤`, `!`, `?`, `copy`, `wk`, `mix`, `cut`, `∀` or `∃`.
    pub const fn name(self) -> &'static str {
        use Node::*;
        match self {
            Ax(..) => "ax",
            Tensor(..) => "⊗",
            Par(..) => "⅋",
            One(_) => "1",
            Bot(..) => "⊥",
            With(..) => "&",
            Plus(_, Branch::Left, _) => "⊕₁",
            Plus(_, Branch::Right, _) => "⊕₂",
            Top(_) => "⊤",
            Bang(..) => "!",
            Quest(..) => "?",
            Copy(..) => "copy",
            Weaken(..) => "wk",
            Mix(..) => "mix",
            Cut(..) => "cut",
            Forall(..) => "∀",
            Exists(..) => "∃",
        }
    }
}

impl Display for Node {
    /// Writes the rule's name, the members after `on` and the premise
    /// nodes after `from`: `⊗ on 1 from 0, 2`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())?;
        let mut occurrences = self.members().map(Member::get);
        if let Some(o) = occurrences.next() {
            write!(f, " on {o}")?;
            for o in occurrences {
                write!(f, ", {o}")?;
            }
        }
        let mut premises = self.premises().map(|p| p.get());
        if let Some(p) = premises.next() {
            write!(f, " from {p}")?;
            for p in premises {
                write!(f, ", {p}")?;
            }
        }
        Ok(())
    }
}

/// A proof of a sequent: an array of rule applications ([`Node`]s) over
/// the sequent's [`Forest`], which the proof owns.
///
/// - Every node refers only to nodes before it.
/// - The last node is the root.
/// - The root reaches every node.
/// - A subproof that two nodes share is stored once.
///
/// A search engine returns a proof. Whether it proves its sequent is a
/// separate question, which [`check`](Self::check) answers.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::linear::{Forest, Member, Mode, Node, NodeId, Proof, Sequent};
///
/// // ⊢ ~A, A ⊗ ~B, B, whose members are 0: ~A, 1: A ⊗ ~B, 2: A,
/// // 3: ~B, 4: B.
/// let sequent: Sequent = "A, A -o B |- B".parse()?;
/// let forest = Forest::new(&sequent)?;
/// let (o, n) = (Member::new, NodeId::new);
/// let nodes = vec![
///     Node::Ax(o(0), o(2)),
///     Node::Ax(o(3), o(4)),
///     Node::Tensor(o(1), n(0), n(1)),
/// ];
/// let proof = Proof::new(forest, nodes, n(2))?;
/// proof.check(Mode::CLASSICAL)?;
/// assert_eq!(
///     proof.derivation()?.to_string(),
///     "─────── ax   ─────── ax\n\
///      ⊢ ~A, A      ⊢ ~B, B\n\
///      ──────────────────── ⊗\n\
///     \x20 ⊢ ~A, A ⊗ ~B, B"
/// );
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # JSON
///
/// With the feature `serialize`, a proof is `{"version": 1, "sequent": …,
/// "nodes": […], "mode": …, "goal": […]}` ([`wire`](crate::wire)):
///
/// - `sequent` is the sequent in [`Sequent`]'s form, nested, without its
///   own `version`;
/// - `mode` and `goal` are the mode the proof is meant for and the goal it
///   concludes, where the proof records them;
/// - `nodes` lists the nodes, premises before conclusions and the root
///   last.
///
/// A node is an object with one key, the rule's tag:
///
/// - `{"ax": [x, y]}` on two literals;
/// - `{"1": o}` and `{"⊤": o}` on one occurrence;
/// - `{"⊗": [o, l, r]}` and `{"&": [o, l, r]}` with two premises, and
///   `{"cut": [a, l, r]}` with `a` the occurrence in the cut formula;
/// - `{"mix": [l, r]}`;
/// - `{"⅋": [o, p]}` with one premise, and the same for `⊥`, `⊕₁`, `⊕₂`,
///   `!`, `?`, `copy` and `wk`.
///
/// An occurrence is an id of the sequent's [`Forest`], and a premise is a
/// node's index. A proof of `A, B ⊢ A ⊗ B` is
/// `[{"ax": [0, 3]}, {"ax": [1, 4]}, {"⊗": [2, 0, 1]}]`.
///
/// A proof of a first-order sequent adds the nodes `{"∀": [m, a, p]}` and
/// `{"∃": [m, w, p]}` and the key `instances` ([`wire`](crate::wire)).
///
/// Reading rebuilds the forest within the limits and checks the indices
/// and their order. It does not check the proof: call [`Proof::check`]
/// in the mode you want. The command's `check` reads this form, and the
/// output of `prove --format json` contains it.
#[derive(Clone, Debug)]
pub struct Proof {
    /// The forest of the sequent the proof is of.
    forest: Forest,
    /// The rule instances, premises before conclusions, the root last.
    nodes: Box<[Node]>,
    /// The members the root concludes, where they are not the sequent's
    /// root formulas.
    goal: Option<Box<[Member]>>,
    /// The mode the proof is meant for: a claim, which only a check tests.
    mode: Option<Mode>,
    /// What the members past the forest's length stand for, canonical:
    /// `None` for a proof whose sequent has no binder.
    instances: Option<Box<Instances>>,
}

impl Proof {
    /// Builds a proof from a list of nodes and its root. It keeps the nodes
    /// the root reaches, in their order, and drops the rest. It does not
    /// check the proof.
    ///
    /// # Errors
    ///
    /// - [`Error::IndexOutOfBounds`] for a root outside `nodes`, or a
    ///   reached node that names an occurrence outside the forest;
    /// - [`Error::NotTopological`] for a premise that does not come before
    ///   its node;
    /// - [`Refusal::Index`](crate::Refusal::Index) when the root reaches
    ///   more nodes than a proof holds (2³² − 1).
    pub fn new(forest: Forest, nodes: Vec<Node>, root: NodeId) -> Result<Self, Error> {
        if forest.has_binders() {
            let instances = Instances::new(&forest);
            return Self::with_instances(forest, instances, nodes, root);
        }
        Self::build(forest, nodes, root, &mut || false)
    }

    /// Builds a proof as [`new`](Self::new) does, for a first-order
    /// sequent. The members past the forest's length are those
    /// `instances` numbers: occurrences under binders, with the terms the
    /// binders were opened with.
    ///
    /// The table is checked against the forest and made canonical, so that
    /// equal instances are one member. The nodes' members and witnesses
    /// are renumbered into it; the instances they name stay the same. The
    /// table also gets every member a check derives from one a node names:
    /// a subformula's instance, a binder's body under its eigenvariable or
    /// witness, a cut's dual.
    ///
    /// # Errors
    ///
    /// Those of [`new`](Self::new), where a member is in range when the
    /// table numbers it, and a witness when it is a term of the sequent or
    /// of the table. [`Error::Json`] (form `instance table`) for a table
    /// made for another sequent or not well formed, or for a witness with
    /// a loose bound variable.
    pub fn with_instances(
        forest: Forest,
        instances: Instances,
        nodes: Vec<Node>,
        root: NodeId,
    ) -> Result<Self, Error> {
        let members = (forest.len() as u64).saturating_add(instances.len() as u64);
        let mut proof = Self::reach(forest, nodes, root, members, &mut || false)?;
        let mut nodes = std::mem::take(&mut proof.nodes).into_vec();
        let table = instances::canonical(&proof.forest, &instances, &mut nodes)?;
        proof.nodes = nodes.into_boxed_slice();
        proof.instances = (proof.forest.has_binders() || table != Instances::new(&proof.forest))
            .then(|| Box::new(table));
        Ok(proof)
    }

    /// Returns the proof's instance table: `None` for a proof whose
    /// sequent has no binder.
    pub fn instances(&self) -> Option<&Instances> {
        self.instances.as_deref()
    }

    /// Builds a proof as [`new`](Self::new) does, in place in `nodes`.
    /// Both passes call `stopped` every [`BUILD_POLL`] nodes, since an
    /// engine's proof can have hundreds of millions of nodes.
    ///
    /// # Errors
    ///
    /// Those of `new`, and [`Refusal::Stopped`](crate::Refusal::Stopped) in
    /// the search phase once `stopped` returns true.
    pub(crate) fn build(
        forest: Forest,
        nodes: Vec<Node>,
        root: NodeId,
        stopped: &mut dyn FnMut() -> bool,
    ) -> Result<Self, Error> {
        let members = forest.len() as u64;
        Self::reach(forest, nodes, root, members, stopped)
    }

    /// Builds a proof as [`build`](Self::build) does, where a member is in
    /// range below `members`.
    fn reach(
        forest: Forest,
        mut nodes: Vec<Node>,
        root: NodeId,
        members: u64,
        stopped: &mut dyn FnMut() -> bool,
    ) -> Result<Self, Error> {
        /// The index of a node the root does not reach.
        const UNREACHED: u32 = u32::MAX;
        let stop = || {
            Error::Refused(crate::limits::Refusal::Stopped {
                phase: crate::limits::Phase::Search,
            })
        };
        if root.index() >= nodes.len() {
            return Err(Error::IndexOutOfBounds {
                space: crate::limits::Space::Node,
                index: root.index(),
                len: nodes.len(),
            });
        }
        // A premise comes before its conclusion, so one pass from the root
        // down visits every reached node after a node that reaches it. A
        // reached node is marked 0 until the second pass gives it its new
        // index. The root's index is below a vector's length, so adding one
        // cannot overflow a `usize`.
        let mut index = vec![UNREACHED; root.index() + 1];
        index[root.index()] = 0;
        for i in (0..=root.index()).rev() {
            if i % BUILD_POLL == 0 && stopped() {
                return Err(stop());
            }
            if index[i] == UNREACHED {
                continue;
            }
            for o in nodes[i].members() {
                if u64::from(o.get()) >= members {
                    return Err(Error::IndexOutOfBounds {
                        space: crate::limits::Space::Occurrence,
                        index: o.index(),
                        len: usize::try_from(members).unwrap_or(usize::MAX),
                    });
                }
            }
            for p in nodes[i].premises() {
                if p.index() >= i {
                    return Err(Error::NotTopological {
                        space: crate::limits::Space::Node,
                        index: p.index(),
                        parent: i,
                    });
                }
                index[p.index()] = 0;
            }
        }
        // The number of nodes must fit a `u32`: the checker counts a node's
        // readers in one. A root index of `u32::MAX` alone would allow 2³²
        // nodes, one too many.
        let reached = index.iter().filter(|&&i| i != UNREACHED).count();
        if u32::try_from(reached).is_err() {
            return Err(Error::Refused(crate::limits::Refusal::Index {
                what: crate::limits::Space::Node,
                count: reached as u64,
                most: u64::from(u32::MAX),
            }));
        }
        // The kept nodes move down in their order. A node is written at or
        // below the index it is read from, after the premises it names, so
        // nothing is overwritten before it is read.
        let mut kept = 0;
        for i in 0..=root.index() {
            if i % BUILD_POLL == 0 && stopped() {
                return Err(stop());
            }
            if index[i] == UNREACHED {
                continue;
            }
            // Fewer than `reached` nodes are kept so far, so this fits a
            // `u32`.
            index[i] = kept as u32;
            nodes[kept] = nodes[i].map_premises(|p| NodeId::new(index[p.index()]));
            kept += 1;
        }
        nodes.truncate(kept);
        Ok(Self {
            forest,
            nodes: nodes.into_boxed_slice(),
            goal: None,
            mode: None,
            instances: None,
        })
    }

    /// Builds a proof of `goal` as [`new`](Self::new) builds a proof of the
    /// sequent. `goal` is a list of members of the forest, read as the
    /// sequent of those subformulas, and the root concludes it. A goal that
    /// is the roots in any order is the sequent itself, and the proof then
    /// records no goal.
    ///
    /// # Errors
    ///
    /// As [`new`](Self::new) does, and [`Error::IndexOutOfBounds`] for a
    /// member of the goal outside the forest.
    pub fn new_of_goal(
        forest: Forest,
        goal: &[Member],
        nodes: Vec<Node>,
        root: NodeId,
    ) -> Result<Self, Error> {
        if let Some(m) = goal.iter().find(|m| m.occurrence(&forest).is_none()) {
            return Err(Error::IndexOutOfBounds {
                space: crate::limits::Space::Member,
                index: m.index(),
                len: forest.len(),
            });
        }
        // The roots in any order are the sequent itself, as `prove_goal`
        // reads them too.
        let occurrences: Vec<OccId> = goal.iter().map(|m| m.occ()).collect();
        let roots = forest.is_roots(&occurrences);
        let mut proof = Self::new(forest, nodes, root)?;
        proof.goal = (!roots).then(|| goal.into());
        Ok(proof)
    }

    /// Returns the proof with the mode it is meant for recorded.
    #[must_use]
    pub fn with_mode(self, mode: Mode) -> Self {
        Self {
            mode: Some(mode),
            ..self
        }
    }

    /// Returns the members the root concludes, or `None` for a proof of
    /// the whole sequent.
    pub fn goal(&self) -> Option<&[Member]> {
        self.goal.as_deref()
    }

    /// Returns the mode the proof is meant for, as the search that found
    /// it records it. This is a claim only: [`check`](Self::check) tests
    /// the proof in the mode you pass it.
    pub fn mode(&self) -> Option<Mode> {
        self.mode
    }

    /// Returns the occurrence a member of the proof stands for.
    pub fn occurrence(&self, member: Member) -> OccId {
        match self.instances.as_deref().and_then(|t| t.instance(member)) {
            Some((o, _)) => o,
            None => member.occ(),
        }
    }

    /// Returns the formula a member of the proof stands for, as text.
    ///
    /// A proof that records an intuitionistic mode writes it as the
    /// sequent's intuitionistic reading does, which costs a pass over the
    /// forest on each call. Any other proof writes it one-sided. An
    /// instance has the terms of its frame in place of its loose bound
    /// variables. Each term is cut after a few thousand characters, since
    /// a term with shared subterms can unfold past any memory.
    ///
    /// # Panics
    ///
    /// For a member the proof does not number.
    pub fn formula(&self, member: Member) -> impl Display + '_ {
        let reading = self
            .mode
            .filter(|mode| mode.is_intuitionistic())
            .and_then(|mode| Reading::of_mode(&self.forest, mode.with_empty_antecedents()).ok());
        if let Some(table) = self.instances.as_deref()
            && member.index() >= self.forest.len()
        {
            let framing = instances::Framing::new(&self.forest.sequent().fo, table);
            let mut text = String::new();
            crate::linear::sequents::notation::TEXT
                .instance(
                    &mut text,
                    &self.forest,
                    reading.as_ref(),
                    &framing,
                    member,
                    Some(crate::errors::describe::TERM),
                )
                .expect("writing into a string cannot fail");
            return text;
        }
        match reading {
            Some(reading) => reading.formula(member.occ()).to_string(),
            None => self.forest.formula(member.occ()).to_string(),
        }
    }

    /// Returns the proof with `goal` recorded as what its root concludes.
    pub(crate) fn concluding(self, goal: &[OccId]) -> Self {
        Self {
            goal: Some(goal.iter().copied().map(Member::from).collect()),
            ..self
        }
    }

    /// Returns the proof carried over to `goal`, members of `forest` whose
    /// formulas are the proof's conclusions in any order.
    ///
    /// Each conclusion is matched to a member of the goal with the same
    /// formula: the same connectives, units and literals, with atoms
    /// compared by name. Every occurrence below the conclusion is renamed
    /// to the occurrence in the same place below that member. Where several
    /// members have the same formula, any of them will do. The result has
    /// the same rules as the proof and concludes the goal. It is not
    /// checked: call [`check`](Self::check) as for any proof.
    ///
    /// # Errors
    ///
    /// - [`Error::SequentMismatch`] when the conclusions and the goal differ
    ///   by more than their order, with the formulas only one of them has;
    /// - [`Error::IndexOutOfBounds`] for a member of the goal outside
    ///   `forest`, or a member a node names outside every conclusion.
    pub fn onto(&self, forest: Forest, goal: &[Member]) -> Result<Self, Error> {
        if let Some(m) = goal.iter().find(|m| m.occurrence(&forest).is_none()) {
            return Err(Error::IndexOutOfBounds {
                space: crate::limits::Space::Member,
                index: m.index(),
                len: forest.len(),
            });
        }
        let from = &self.forest;
        let name = |f: &Forest, o: OccId| f.atom(o).map(|a| f.sequent().atom_name(a).to_owned());
        let alike = |o: OccId, t: OccId| {
            from.size(o) == forest.size(t)
                && from.subtree(o).zip(forest.subtree(t)).all(|(a, b)| {
                    from.kind(a) == forest.kind(b) && name(from, a) == name(&forest, b)
                })
        };
        // Each conclusion takes the first free goal member alike to it, and
        // each occurrence below it the occurrence in the same place there.
        // The members are grouped by their formula's text, so a conclusion
        // is compared only with members written the same way: comparing
        // with every member would cost n² on a goal of n formulas. `alike`
        // still decides the match.
        let mut free: crate::hash::HashMap<String, std::collections::VecDeque<usize>> =
            crate::hash::HashMap::default();
        for (i, m) in goal.iter().enumerate() {
            free.entry(forest.formula(m.occ()).to_string())
                .or_default()
                .push_back(i);
        }
        let mut taken = vec![false; goal.len()];
        let mut place: Vec<Option<OccId>> = vec![None; from.len()];
        let mut only_proof = Vec::new();
        for c in self.conclusion() {
            let written = from.formula(c).to_string();
            let found = free.get_mut(&written).and_then(|bucket| {
                let at = bucket.iter().position(|&i| alike(c, goal[i].occ()))?;
                bucket.remove(at)
            });
            match found {
                Some(i) => {
                    taken[i] = true;
                    for (o, t) in from.subtree(c).zip(forest.subtree(goal[i].occ())) {
                        place[o.index()] = Some(t);
                    }
                }
                None => only_proof.push(c),
            }
        }
        let only_goal: Vec<OccId> = goal
            .iter()
            .zip(&taken)
            .filter(|&(_, &t)| !t)
            .map(|(m, _)| m.occ())
            .collect();
        if !only_proof.is_empty() || !only_goal.is_empty() {
            // Write the formulas as the proof's mode reads each forest:
            // intuitionistic where the mode is and the forest has a reading.
            let written = |f: &Forest, os: Vec<OccId>| -> Vec<String> {
                let reading = self
                    .mode
                    .filter(|mode| mode.is_intuitionistic())
                    .and_then(|mode| Reading::of_mode(f, mode.with_empty_antecedents()).ok());
                os.into_iter()
                    .map(|o| match &reading {
                        Some(reading) => reading.formula(o).to_string(),
                        None => f.formula(o).to_string(),
                    })
                    .collect()
            };
            return Err(Error::SequentMismatch {
                proof: written(from, only_proof),
                goal: written(&forest, only_goal),
            });
        }
        let rename = |m: Member| {
            place
                .get(m.index())
                .copied()
                .flatten()
                .map(Member::from)
                .ok_or(Error::IndexOutOfBounds {
                    space: crate::limits::Space::Member,
                    index: m.index(),
                    len: from.len(),
                })
        };
        let nodes = self
            .nodes
            .iter()
            .map(|node| node.renamed(rename))
            .collect::<Result<Vec<_>, _>>()?;
        let mut carried = Self::new_of_goal(forest, goal, nodes, self.root())?;
        carried.mode = self.mode;
        Ok(carried)
    }

    /// Returns the occurrences the root concludes: the goal's, or else the
    /// sequent's roots.
    pub(crate) fn conclusion(&self) -> Vec<OccId> {
        match &self.goal {
            Some(goal) => goal.iter().map(|m| m.occ()).collect(),
            None => self.forest.roots().to_vec(),
        }
    }

    /// Returns the forest of the sequent the proof is of.
    pub fn forest(&self) -> &Forest {
        &self.forest
    }

    /// Returns the sequent the proof is of.
    pub fn sequent(&self) -> &Sequent {
        self.forest.sequent()
    }

    /// Returns whether the proof has no cut.
    pub fn is_cut_free(&self) -> bool {
        !self.nodes.iter().any(|node| matches!(node, Node::Cut(..)))
    }

    /// Returns every node, premises before conclusions and the root last.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Returns the node at `id`.
    ///
    /// # Panics
    ///
    /// For an id past this proof's last node.
    pub fn node(&self, id: NodeId) -> Node {
        self.nodes[id.index()]
    }

    /// Returns every node id in order.
    pub fn ids(&self) -> impl DoubleEndedIterator<Item = NodeId> + ExactSizeIterator {
        // A proof has at least one node and fewer than 2³².
        (0..self.nodes.len() as u32).map(NodeId::new)
    }

    /// Returns the root: the last node, which concludes the sequent or the
    /// goal.
    pub fn root(&self) -> NodeId {
        // A proof has at least one node and fewer than 2³².
        NodeId::new(self.nodes.len() as u32 - 1)
    }

    /// Checks that the proof proves its sequent with the rules `mode`
    /// allows, within the default [`Limits`].
    ///
    /// The check passes when:
    ///
    /// - every node applies its rule correctly to what its premises derive;
    /// - the mode allows every rule used;
    /// - the root derives exactly the sequent's formulas, or the goal's,
    ///   with nothing left in the unrestricted zone;
    /// - in intuitionistic mode, the sequent has an intuitionistic reading
    ///   and every sequent of the proof has one formula right of `⊢`.
    ///
    /// # Errors
    ///
    /// - [`CheckError::Invalid`] naming the first node that fails, in
    ///   order, with what it needed;
    /// - [`CheckError::Refused`] when the check would pass the default
    ///   memory bound. A refusal is no verdict: the proof is then neither
    ///   valid nor invalid.
    pub fn check(&self, mode: Mode) -> Result<(), CheckError> {
        check::check(self, mode)
    }

    /// Checks the proof as [`check`](Self::check) does, within `limits`,
    /// and stops when `stop` returns true.
    ///
    /// - Memory: the check holds at most `limits.memory_bytes`. It counts
    ///   what it holds beyond the proof and its forest: a count and a
    ///   pointer for every node (twelve bytes on a 64-bit target, eight on
    ///   a 32-bit one), and every sequent it keeps for a later node, at the
    ///   size of its tables. A sequent that several nodes read counts once
    ///   for each of them.
    /// - Work: the check does at most `limits.work` units. A unit is a
    ///   node, or an entry of a premise's sequent that its rule handles. A
    ///   node with a large sequent that many others read costs each of them
    ///   that sequent, so a short file can make the work quadratic in its
    ///   length.
    /// - `stop` is called every 4 096 nodes or 65 536 units, whichever
    ///   comes first.
    ///
    /// A check that a bound or `stop` ends returns
    /// [`CheckError::Refused`]: the proof is then neither valid nor
    /// invalid.
    ///
    /// # Errors
    ///
    /// Those of [`check`](Self::check), with [`CheckError::Refused`] also
    /// past `limits.work` and when `stop` returned true.
    pub fn check_within(
        &self,
        mode: Mode,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), CheckError> {
        check::check_within(self, mode, limits, &mut stop)
    }

    /// Unfolds the proof into the [`Derivation`] of the standard sequent
    /// calculus it stands for, within the default [`Limits`]. A proof meant
    /// for intuitionistic mode unfolds two-sided.
    /// [`derivation_within`](Self::derivation_within) takes other options.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "parse", doc = "```")]
    #[cfg_attr(not(feature = "parse"), doc = "```ignore")]
    /// use deduxit::linear::{IllSequent, Mode, Options, Verdict, prove};
    ///
    /// let ill: IllSequent = "A, A -o B |- B".parse()?;
    /// let outcome = prove(ill.sequent(), Mode::INTUITIONISTIC, &Options::default())?;
    /// let Verdict::Proved(proof) = &outcome.verdict else {
    ///     panic!("provable");
    /// };
    /// assert_eq!(
    ///     proof.derivation()?.to_string(),
    ///     "───── ax   ───── ax\n\
    ///      A ⊢ A      B ⊢ B\n\
    ///      ──────────────── ⊸L\n\
    ///     \x20 A, A ⊸ B ⊢ B"
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// - [`Error::Check`] when the term is no proof in the mode the view
    ///   reads it in;
    /// - [`Error::NotIntuitionistic`] for a two-sided view of a sequent
    ///   without an intuitionistic reading;
    /// - [`Error::Refused`] for a derivation past the default bounds.
    pub fn derivation(&self) -> Result<Derivation<'_>, Error> {
        self.derivation_within(&ViewOptions::default(), &Limits::default(), |_| false)
    }

    /// Unfolds the proof as [`derivation`](Self::derivation) does, shown as
    /// `view` says, within `limits.derivation_bytes` and
    /// `limits.memory_bytes`. It stops when `stop` returns true. `stop` is
    /// called as [`check_within`](Self::check_within) calls it while the
    /// proof is checked, then once per node unfolded.
    ///
    /// # Errors
    ///
    /// Those of [`derivation`](Self::derivation), within `limits`, and
    /// [`Refusal::Stopped`](crate::Refusal::Stopped) when `stop` returned
    /// true.
    pub fn derivation_within(
        &self,
        view: &ViewOptions,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Derivation<'_>, Error> {
        if view.two_sided(self) {
            Derivation::two_sided(self, view, limits, stop)
        } else {
            Derivation::new(self, view, limits, stop)
        }
    }

    /// Returns the two-sided derivation within the default limits, for
    /// tests.
    #[cfg(all(test, feature = "parse"))]
    pub(crate) fn two_sided(&self) -> Result<Derivation<'_>, Error> {
        let view = ViewOptions::default().with_sides(Sides::Two);
        self.derivation_within(&view, &Limits::default(), |_| false)
    }
}

impl Node {
    /// Returns the node with every premise index passed through `f`.
    pub(crate) fn map_premises(self, mut f: impl FnMut(NodeId) -> NodeId) -> Self {
        use Node::*;
        match self {
            Tensor(o, l, r) => Tensor(o, f(l), f(r)),
            Par(o, p) => Par(o, f(p)),
            Bot(o, p) => Bot(o, f(p)),
            With(o, l, r) => With(o, f(l), f(r)),
            Plus(o, s, p) => Plus(o, s, f(p)),
            Bang(o, p) => Bang(o, f(p)),
            Quest(o, p) => Quest(o, f(p)),
            Copy(o, p) => Copy(o, f(p)),
            Weaken(o, p) => Weaken(o, f(p)),
            Mix(l, r) => Mix(f(l), f(r)),
            Cut(o, l, r) => Cut(o, f(l), f(r)),
            Forall(o, a, p) => Forall(o, a, f(p)),
            Exists(o, w, p) => Exists(o, w, f(p)),
            leaf @ (Ax(..) | One(_) | Top(_)) => leaf,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The name lists follow the variants: `NAMES` has one name per
    /// variant in order, and `TAGS` every rule's name in order, with `Plus`
    /// twice.
    #[test]
    fn name_lists_follow_the_variants() {
        let (m, n) = (Member::new(0), NodeId::new(0));
        let nodes = [
            Node::Ax(m, m),
            Node::Tensor(m, n, n),
            Node::Par(m, n),
            Node::One(m),
            Node::Bot(m, n),
            Node::With(m, n, n),
            Node::Plus(m, Branch::Left, n),
            Node::Top(m),
            Node::Bang(m, n),
            Node::Quest(m, n),
            Node::Copy(m, n),
            Node::Weaken(m, n),
            Node::Mix(n, n),
            Node::Cut(m, n, n),
            Node::Forall(m, Eigen::new(0), n),
            Node::Exists(m, FoTermId::new(0), n),
        ];
        let names: Vec<String> = nodes
            .iter()
            .map(|node| format!("{node:?}").split('(').next().unwrap().to_owned())
            .collect();
        assert_eq!(names, Node::NAMES);
        let mut tags: Vec<&str> = nodes.iter().map(|node| node.name()).collect();
        tags.insert(7, Node::Plus(m, Branch::Right, n).name());
        assert_eq!(tags, Node::TAGS);
    }

    /// A goal must be a sequent of the mode it is checked in: in
    /// intuitionistic mode it needs one formula right of `⊢`, as every
    /// sequent of its derivation does. A goal that is the roots in another
    /// order is the sequent itself.
    #[cfg(feature = "parse")]
    #[test]
    fn a_goal_is_a_sequent_of_its_mode() {
        // `0 ⊢ a`: the hypothesis `0`, one-sided `⊤`, is occurrence 0.
        let sequent: Sequent = "0 |- a".parse().unwrap();
        let top = Member::new(0);
        let forest = Forest::new(&sequent).unwrap();
        let proof = Proof::new_of_goal(forest, &[top], vec![Node::Top(top)], NodeId::new(0));
        let proof = proof.unwrap();
        assert_eq!(proof.check(Mode::CLASSICAL), Ok(()));
        let Err(CheckError::Invalid(invalid)) = proof.check(Mode::INTUITIONISTIC) else {
            panic!("a goal without an output is no intuitionistic sequent");
        };
        assert_eq!(invalid.fault, Fault::Succedents { count: 0 });
        let sequent: Sequent = "A |- A".parse().unwrap();
        let (a, b) = (Member::new(0), Member::new(1));
        let forest = Forest::new(&sequent).unwrap();
        let swapped = Proof::new_of_goal(forest, &[b, a], vec![Node::Ax(a, b)], NodeId::new(0));
        assert_eq!(swapped.unwrap().goal(), None);
    }
}
