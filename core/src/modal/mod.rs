// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Propositional modal logic: formulas with `□` ("necessarily") and `◇`
//! ("possibly"), and the normal systems from K to S5.
//!
//! A [`Sequent`] `Γ ⊢ Δ` is read at one world of a Kripke model
//! ([`kripke`](crate::kripke)): it holds there when some formula of `Δ`
//! is true or some formula of `Γ` is false. `□A` is true at a world when
//! `A` is true at every world it sees, `◇A` when `A` is true at one of
//! them. A sequent is valid in a [`System`] when it holds at every world
//! of every model whose frame meets the system's [`Condition`]s.
//!
//! [`decide()`] says whether a sequent is valid in a system. A valid
//! sequent comes with a derivation in the system's labelled sequent
//! calculus ([`calculus()`]), which the checker of calculi given as data
//! ([`rules`](crate::rules)) accepted; a sequent that is not valid comes
//! with a Kripke model that refutes it, which [`Model::truth`] accepted.
//! [`Model::truth`] and [`Model::frame_fault`] also check a model you
//! give. Deciding needs the cargo feature `serialize`, as calculi given
//! as data do.
//!
//! [`Model::truth`]: crate::kripke::Model::truth
//! [`Model::frame_fault`]: crate::kripke::Model::frame_fault
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::modal::{Condition, Sequent, System};
//!
//! let sequent: Sequent = "[]p -> [][]p".parse()?;
//! assert_eq!(sequent.to_string(), "⊢ □p → □□p");
//! // The axiom 4, valid on the transitive frames.
//! assert!(System::S4.has(Condition::Transitive));
//! # Ok::<(), deduxit::Error>(())
//! ```

// A new variant must be decided at every arm, never fall into an
// existing one.
#![deny(clippy::wildcard_enum_match_arm)]

/// The labelled calculus of each system, as a calculus given as data.
#[cfg(feature = "serialize")]
mod calculus;
/// Deciding a sequent in a system.
#[cfg(feature = "serialize")]
mod decide;
/// The check of a Kripke model against a sequent and a system.
mod model;
#[cfg(feature = "parse")]
mod parse;
/// The labelled tableau that decides a sequent.
#[cfg(feature = "serialize")]
mod search;
#[cfg(all(test, feature = "parse", feature = "serialize"))]
mod search_tests;
mod system;

#[cfg(feature = "serialize")]
pub use calculus::calculus;
#[cfg(feature = "serialize")]
pub use decide::{Evidence, Options, Outcome, Statistics, Verdict, decide, decide_within};
pub use model::{CHECKED, Countermodel, Frame, Truth};
pub use system::{Condition, System};

use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Refusal, Space};
use crate::linear::sequents::{Stop, Stops};
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

/// The index of a formula in a [`Formulas`] arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(u32);

impl NodeId {
    /// Returns the index as a `usize`, for indexing the arena.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// A node of a modal formula, which refers to its subformulas by arena
/// index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    /// An atom, by its index among the arena's atom names.
    Atom(u32),
    /// `⊤`, true.
    True,
    /// `⊥`, false.
    False,
    /// `¬A`
    Not(NodeId),
    /// `A ∧ B`
    And(NodeId, NodeId),
    /// `A ∨ B`
    Or(NodeId, NodeId),
    /// `A → B`
    Implies(NodeId, NodeId),
    /// `A ↔ B`
    Iff(NodeId, NodeId),
    /// `□A`: `A` at every world this one sees.
    Box(NodeId),
    /// `◇A`: `A` at some world this one sees.
    Diamond(NodeId),
}

/// A node stays a tag and at most two arena indices, since the arena and
/// every table over it grow with its size.
const _: () = assert!(size_of::<Node>() == 12);

impl Node {
    /// Returns the subformulas, in the order they are written.
    pub const fn operands(self) -> (Option<NodeId>, Option<NodeId>) {
        match self {
            Self::Atom(_) | Self::True | Self::False => (None, None),
            Self::Not(a) | Self::Box(a) | Self::Diamond(a) => (Some(a), None),
            Self::And(a, b) | Self::Or(a, b) | Self::Implies(a, b) | Self::Iff(a, b) => {
                (Some(a), Some(b))
            }
        }
    }

    /// Returns whether the node is `□` or `◇`.
    pub const fn is_modal(self) -> bool {
        matches!(self, Self::Box(_) | Self::Diamond(_))
    }
}

/// The formulas of modal sequents, as an arena in which every subformula
/// precedes the formulas built from it and equal formulas are one node,
/// so that two formulas are equal exactly when their ids are.
#[derive(Clone, Debug, Default)]
pub struct Formulas {
    /// The nodes, each after its subformulas.
    nodes: Vec<Node>,
    /// The atom names, by the index `Node::Atom` holds.
    atoms: Vec<String>,
    /// The index of every atom name.
    names: HashMap<String, u32>,
    /// The id of every node, for sharing.
    ids: HashMap<Node, NodeId>,
}

impl Formulas {
    /// The most nodes an arena holds: the ids are `u32`.
    pub const MOST: usize = u32::MAX as usize;

    /// Returns the node with the id given, which must belong to this arena.
    ///
    /// # Panics
    ///
    /// If the id is past the arena's end, as an id of another arena may
    /// be.
    pub fn node(&self, id: NodeId) -> Node {
        self.nodes[id.index()]
    }

    /// Returns the nodes, each after its subformulas.
    #[cfg(feature = "serialize")]
    pub(crate) fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Returns the number of nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the arena has no node.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the name of the atom with the index given.
    pub fn atom_name(&self, atom: u32) -> &str {
        &self.atoms[atom as usize]
    }

    /// Returns the atom names, in the order of their indices.
    pub fn atom_names(&self) -> &[String] {
        &self.atoms
    }

    /// Returns the id of `node`, adding it unless the arena has it.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a subformula that is no node of
    /// this arena (an id of another one) or an atom that is not one of its
    /// names, and [`Refusal::Index`] when the arena is full.
    #[cfg_attr(
        target_pointer_width = "32",
        expect(
            clippy::absurd_extreme_comparisons,
            reason = "`MOST` is `usize::MAX` on 32 bits, where the test still holds"
        )
    )]
    pub fn add(&mut self, node: Node) -> Result<NodeId, Error> {
        if let Some(&id) = self.ids.get(&node) {
            return Ok(id);
        }
        let (a, b) = node.operands();
        for id in [a, b].into_iter().flatten() {
            self.holds(id)?;
        }
        if let Node::Atom(atom) = node
            && atom as usize >= self.atoms.len()
        {
            return Err(Error::IndexOutOfBounds {
                space: Space::Atom,
                index: atom as usize,
                len: self.atoms.len(),
            });
        }
        if self.nodes.len() >= Self::MOST {
            return Err(Error::Refused(Refusal::Index {
                what: Space::Formula,
                count: Self::MOST as u64 + 1,
                most: Self::MOST as u64,
            }));
        }
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        self.ids.insert(node, id);
        Ok(id)
    }

    /// Fails unless `id` is a node of this arena.
    fn holds(&self, id: NodeId) -> Result<(), Error> {
        if id.index() < self.nodes.len() {
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds {
                space: Space::Formula,
                index: id.index(),
                len: self.nodes.len(),
            })
        }
    }

    /// Returns the atom named `name`, adding it unless the arena has it.
    /// The name is kept in Unicode normal form NFC, so one name is one atom
    /// however its accents are encoded. It must be an identifier of the
    /// text syntax and no keyword, as an atom of ordinary logic, so that a
    /// formula without `□` and `◇` is written the same in both.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier, a keyword
    /// (`true`, `false`, and the linear syntax's `par`, `top` and `bot`)
    /// or a word reserved for a later version of the syntax (`forall`,
    /// `exists`), and [`Refusal::Index`] when the arena is full.
    pub fn atom(&mut self, name: &str) -> Result<NodeId, Error> {
        let name = &*crate::linear::sequents::name::normalized(name);
        let index = match self.names.get(name) {
            Some(&index) => index,
            None => {
                crate::linear::sequents::name::check_ordinary(name)?;
                let index = self.atoms.len() as u32;
                self.atoms.push(name.to_owned());
                self.names.insert(name.to_owned(), index);
                index
            }
        };
        self.add(Node::Atom(index))
    }

    /// Returns the formula at `id` as a value that prints it.
    pub fn formula(&self, id: NodeId) -> Formula<'_> {
        Formula { formulas: self, id }
    }

    /// Writes the formula at `id` with the Unicode symbols, every binary
    /// subformula in brackets.
    fn write<W: Write>(&self, out: &mut W, id: NodeId) -> FmtResult {
        for visit in Stops::new(id, false, |k| self.node(k).operands()) {
            match visit {
                Stop::Enter(k, nested) => match self.node(k) {
                    Node::Atom(a) => out.write_str(self.atom_name(a))?,
                    Node::True => out.write_char('⊤')?,
                    Node::False => out.write_char('⊥')?,
                    Node::Not(_) => out.write_char('¬')?,
                    Node::Box(_) => out.write_char('□')?,
                    Node::Diamond(_) => out.write_char('◇')?,
                    Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {
                        if nested {
                            out.write_char('(')?;
                        }
                    }
                },
                Stop::Between(k) => out.write_str(match self.node(k) {
                    Node::And(..) => " ∧ ",
                    Node::Or(..) => " ∨ ",
                    Node::Implies(..) => " → ",
                    Node::Atom(_)
                    | Node::True
                    | Node::False
                    | Node::Not(_)
                    | Node::Box(_)
                    | Node::Diamond(_)
                    | Node::Iff(..) => " ↔ ",
                })?,
                Stop::Exit(k, nested) => {
                    if nested && self.node(k).operands().1.is_some() {
                        out.write_char(')')?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// A formula of an arena, as a value that prints it with the Unicode
/// symbols, every binary subformula in brackets.
#[derive(Clone, Copy, Debug)]
pub struct Formula<'a> {
    /// The arena.
    formulas: &'a Formulas,
    /// The formula.
    id: NodeId,
}

impl Display for Formula<'_> {
    /// Writes the formula: `□(a → b) → ◇¬c`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.formulas.write(f, self.id)
    }
}

/// A sequent of propositional modal logic, `Γ ⊢ Δ`: the hypotheses `Γ`
/// left of the turnstile and the formulas `Δ` right of it. It holds at a
/// world of a Kripke model when some formula of `Δ` is true there or some
/// formula of `Γ` is false, and is valid in a [`System`] when it holds at
/// every world of every model of the system. So `Γ ⊢ Δ` is valid exactly
/// when the one formula `⋀Γ → ⋁Δ` is.
///
/// # Syntax
///
/// With the cargo feature `parse`, `"…".parse::<modal::Sequent>()` reads
/// a sequent written `formulas |- formulas` (or `⊢`), each side a list
/// separated by commas and possibly empty; a text without a turnstile is
/// one formula to prove. From the loosest binding to the tightest, a
/// formula is built with `<->`/`↔` (to the right), `->`/`→` (to the
/// right), `\/`/`∨`, `/\`/`∧` (these to the left) and the prefixes
/// `~`/`¬`, `[]`/`□` and `<>`/`◇`, over the constants `true`/`⊤` and
/// `false`/`⊥`, atoms and parentheses. A prefix takes the formula right
/// after it: `[]p -> q` is `(□p) → q`. An atom is a Unicode identifier
/// other than a keyword: `true`, `false`, the linear syntax's `par`,
/// `top` and `bot`, and the reserved `forall` and `exists`. The symbols
/// of linear logic (`&`, `|`, `*`, `-o`, …) are not read here.
///
/// It is the syntax of [`ordinary::Sequent`](crate::ordinary::Sequent)
/// with the two modal prefixes. `Display` writes the Unicode symbols
/// with every nested binary formula in brackets, which this syntax reads
/// back.
///
/// # JSON
///
/// With the cargo feature `serialize` a sequent is `{"version": 20,
/// "formulas": [{"V": 0}, {"□": 0}, {"→": [1, 0]}], "atoms": ["p"],
/// "left": [], "right": [2]}` for `⊢ □p → p` ([`wire`](crate::wire)):
/// the formulas in the arena's order, every operand before the formula
/// built from it, as `{"V": atom}`, `"⊤"`, `"⊥"`, `{"¬": a}`, `{"∧": [a,
/// b]}`, `{"∨": [a, b]}`, `{"→": [a, b]}`, `{"↔": [a, b]}`, `{"□": a}`
/// and `{"◇": a}`, and the two sides as indices into them. A sequent
/// whose arena holds neither `□` nor `◇` is written at level 8, as the
/// same sequent of ordinary logic is, and a sequent of ordinary logic
/// reads as a modal one.
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// let sequent: deduxit::modal::Sequent = "[](p -> q), <>p |- <>q".parse()?;
/// assert_eq!(sequent.to_string(), "□(p → q), ◇p ⊢ ◇q");
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Debug)]
pub struct Sequent {
    /// The formulas.
    formulas: Formulas,
    /// The hypotheses, in the order written.
    left: Vec<NodeId>,
    /// The formulas right of the turnstile, in the order written.
    right: Vec<NodeId>,
}

impl Sequent {
    /// Returns the sequent `left ⊢ right` over the formulas given.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for an id of the two lists that is no
    /// node of `formulas`.
    pub fn new(formulas: Formulas, left: Vec<NodeId>, right: Vec<NodeId>) -> Result<Self, Error> {
        for &id in left.iter().chain(&right) {
            formulas.holds(id)?;
        }
        Ok(Self {
            formulas,
            left,
            right,
        })
    }

    /// Returns the arena of the formulas.
    pub fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    /// Returns the formulas left of the turnstile, in the order written.
    pub fn left(&self) -> &[NodeId] {
        &self.left
    }

    /// Returns the formulas right of the turnstile, in the order written.
    pub fn right(&self) -> &[NodeId] {
        &self.right
    }
}

impl Display for Sequent {
    /// Writes `Γ ⊢ Δ` with the Unicode symbols.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for (i, &id) in self.left.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            self.formulas.write(f, id)?;
        }
        if !self.left.is_empty() {
            f.write_char(' ')?;
        }
        f.write_char('⊢')?;
        for (i, &id) in self.right.iter().enumerate() {
            f.write_str(if i == 0 { " " } else { ", " })?;
            self.formulas.write(f, id)?;
        }
        Ok(())
    }
}

/// Returns the number of subformula occurrences the formulas unfold to,
/// saturating.
#[cfg(any(feature = "parse", feature = "serialize"))]
fn occurrences<'a>(formulas: &Formulas, roots: impl Iterator<Item = &'a NodeId>) -> u64 {
    // Every subformula precedes the formulas built from it.
    let mut size: Vec<u64> = Vec::with_capacity(formulas.len());
    for node in &formulas.nodes {
        let (a, b) = node.operands();
        let of = |k: Option<NodeId>| k.map_or(0, |k| size[k.index()]);
        size.push(1u64.saturating_add(of(a)).saturating_add(of(b)));
    }
    roots.fold(0, |sum, id| sum.saturating_add(size[id.index()]))
}

/// Checks that a sequent over the formulas unfolds to at most
/// `limits.occurrences` subformula occurrences.
#[cfg(any(feature = "parse", feature = "serialize"))]
pub(crate) fn bound(
    formulas: &Formulas,
    left: &[NodeId],
    right: &[NodeId],
    limits: &crate::Limits,
) -> Result<(), Error> {
    let occurrences = occurrences(formulas, left.iter().chain(right));
    match limits.occurrences {
        Some(limit) if occurrences > limit => {
            Err(Error::Refused(Refusal::Occurrences { occurrences, limit }))
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An arena shares equal formulas, refuses a subformula of another
    /// arena and an atom outside its names, and a sequent refuses an id
    /// outside its arena.
    #[test]
    fn the_arena_shares_and_refuses_foreign_ids() {
        let mut other = Formulas::default();
        let (a, b) = (other.atom("a").unwrap(), other.atom("b").unwrap());
        let and = other.add(Node::And(a, b)).unwrap();
        let mut formulas = Formulas::default();
        let x = formulas.atom("x").unwrap();
        let boxed = formulas.add(Node::Box(x)).unwrap();
        assert_eq!(formulas.add(Node::Box(x)).unwrap(), boxed);
        assert!(formulas.node(boxed).is_modal() && !formulas.node(x).is_modal());
        let refused = |result: Result<_, Error>, space| matches!(result, Err(Error::IndexOutOfBounds { space: s, .. }) if s == space);
        assert!(refused(formulas.add(Node::Box(and)), Space::Formula));
        assert!(refused(formulas.add(Node::Atom(5)), Space::Atom));
        let sequent = Sequent::new(formulas.clone(), vec![x], vec![and]);
        assert!(refused(sequent.map(|_| x), Space::Formula));
        let possibly = formulas.add(Node::Diamond(x)).unwrap();
        assert_ne!(possibly, boxed);
        let sequent = Sequent::new(formulas, vec![x], vec![boxed, possibly]).unwrap();
        assert_eq!(sequent.to_string(), "x ⊢ □x, ◇x");
        for name in ["true", "top", "forall", "a b"] {
            assert!(
                matches!(Formulas::default().atom(name), Err(Error::AtomName { .. })),
                "{name}"
            );
        }
    }
}
