// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What the classical connection read-back and Herbrand's construction
//! share of building a derivation of LK branch by branch: a branch, the
//! rule applied to it, the frames of the construction's stack, and the
//! closing of a branch by `⊤R`, `⊥L` or an axiom with its weakenings.
//! Each keeps its own oracle (which rule comes next, with which witness)
//! and its own charge of the memory bound.

use super::{Formulas, Node, NodeId, Rule, Side};
use crate::Error;
use crate::first_order::FoTermId;
use crate::limits::Progress;
use crate::linear::proofs::InfId;

/// An item of a branch: a formula on its side.
pub(super) trait Placed {
    /// Returns whether it stands left of `⊢`.
    fn left(&self) -> bool;
    /// Returns its formula.
    fn node(&self) -> NodeId;
}

/// A branch: its formulas and the eigenvariables (or the Skolem terms
/// that stand for them) it introduced.
#[derive(Clone, Debug)]
pub(super) struct Branch<I> {
    /// The formulas, hypotheses and goals in the order they came.
    pub(super) items: Vec<I>,
    /// The eigenvariables of `∀R` and `∃L` below it.
    pub(super) introduced: Vec<FoTermId>,
    /// The eigenvariables of Skolem terms a witness below it holds though
    /// the branch never introduced them: constants here, which no `∀R` or
    /// `∃L` above may introduce.
    pub(super) constants: Vec<FoTermId>,
}

impl<I> Default for Branch<I> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            introduced: Vec::new(),
            constants: Vec::new(),
        }
    }
}

/// A rule applied to a branch.
pub(super) struct Step<I> {
    /// The rule.
    pub(super) rule: Rule,
    /// The principal formula.
    pub(super) principal: (Side, usize),
    /// The term of a quantifier rule.
    pub(super) term: Option<FoTermId>,
    /// The premises.
    pub(super) premises: Vec<Branch<I>>,
}

/// A step of the construction.
pub(super) enum Frame<I> {
    /// Derive the branch.
    Expand(Branch<I>),
    /// Make the inference whose premises are the last results.
    Emit {
        /// Its hypotheses.
        left: Vec<NodeId>,
        /// Its goals.
        right: Vec<NodeId>,
        /// Its rule.
        rule: Rule,
        /// Its principal formula.
        principal: Option<(Side, usize)>,
        /// Its term.
        term: Option<FoTermId>,
        /// How many premises.
        premises: usize,
    },
}

/// Returns the hypotheses and the goals of a branch, in order, each vector
/// exactly as long as its side: every inference keeps both.
pub(super) fn sides<I: Placed>(branch: &Branch<I>) -> (Vec<NodeId>, Vec<NodeId>) {
    let n = branch.items.iter().filter(|i| i.left()).count();
    let mut left = Vec::with_capacity(n);
    let mut right = Vec::with_capacity(branch.items.len() - n);
    for item in &branch.items {
        if item.left() {
            left.push(item.node());
        } else {
            right.push(item.node());
        }
    }
    (left, right)
}

/// Returns the position of item `at` among the items of its side.
pub(super) fn position<I: Placed>(branch: &Branch<I>, at: usize) -> (Side, usize) {
    let left = branch.items[at].left();
    let before = branch.items[..at]
        .iter()
        .filter(|i| i.left() == left)
        .count();
    (if left { Side::Left } else { Side::Right }, before)
}

/// Returns the branch without item `at` and with `added`.
pub(super) fn replaced<I: Clone>(branch: &Branch<I>, at: usize, added: Vec<I>) -> Branch<I> {
    let mut premise = branch.clone();
    premise.items.remove(at);
    premise.items.extend(added);
    premise
}

/// Returns whether a rule of two premises on item `at` would close one of
/// them at once: an operand that stands on the other side, or `⊤` going
/// right or `⊥` going left.
pub(super) fn closes_a_premise<I: Placed>(
    formulas: &Formulas,
    branch: &Branch<I>,
    at: usize,
) -> bool {
    let item = &branch.items[at];
    let operands = match (formulas.node(item.node()), item.left()) {
        (Node::And(a, b), false) => [(a, false), (b, false)],
        (Node::Or(a, b), true) => [(a, true), (b, true)],
        (Node::Implies(a, b), true) => [(a, false), (b, true)],
        _ => return false,
    };
    operands.iter().any(|&(node, left)| {
        let shape = formulas.node(node);
        (left && shape == Node::False)
            || (!left && shape == Node::True)
            || branch
                .items
                .iter()
                .any(|other| other.left() != left && other.node() == node)
    })
}

/// A construction that adds inferences, each within its own bounds.
pub(super) trait Emits {
    /// Returns the arena of the formulas.
    fn formulas(&self) -> &Formulas;

    /// Adds an inference and returns its id, within the construction's
    /// bounds.
    #[expect(
        clippy::too_many_arguments,
        reason = "an inference's parts, as the checker reads them"
    )]
    fn emit(
        &mut self,
        left: Vec<NodeId>,
        right: Vec<NodeId>,
        rule: Rule,
        principal: Option<(Side, usize)>,
        term: Option<FoTermId>,
        premises: Vec<InfId>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<InfId, Error>;
}

/// Closes `branch` by `⊤R`, `⊥L` or an axiom with weakenings, the first
/// that applies, and returns the inference; `None` where none does.
pub(super) fn close<E: Emits, I: Placed>(
    emits: &mut E,
    branch: &Branch<I>,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Option<InfId>, Error> {
    let (left, right) = sides(branch);
    if let Some(at) = right
        .iter()
        .position(|&k| emits.formulas().node(k) == Node::True)
    {
        return emits
            .emit(
                left,
                right,
                Rule::TrueRight,
                Some((Side::Right, at)),
                None,
                Vec::new(),
                stop,
            )
            .map(Some);
    }
    if let Some(at) = left
        .iter()
        .position(|&k| emits.formulas().node(k) == Node::False)
    {
        return emits
            .emit(
                left,
                right,
                Rule::FalseLeft,
                Some((Side::Left, at)),
                None,
                Vec::new(),
                stop,
            )
            .map(Some);
    }
    let Some(&shared) = left.iter().find(|k| right.contains(k)) else {
        return Ok(None);
    };
    // `A ⊢ A`, then every other formula weakened in, in order.
    let mut id = emits.emit(
        vec![shared],
        vec![shared],
        Rule::Axiom,
        None,
        None,
        Vec::new(),
        stop,
    )?;
    let (mut l, mut r) = (vec![shared], vec![shared]);
    let mut skipped = (false, false);
    for item in &branch.items {
        let side = if item.left() {
            &mut skipped.0
        } else {
            &mut skipped.1
        };
        if item.node() == shared && !*side {
            *side = true;
            continue;
        }
        let (rule, principal) = if item.left() {
            l.push(item.node());
            (Rule::WeakenLeft, (Side::Left, l.len() - 1))
        } else {
            r.push(item.node());
            (Rule::WeakenRight, (Side::Right, r.len() - 1))
        };
        id = emits.emit(
            l.clone(),
            r.clone(),
            rule,
            Some(principal),
            None,
            vec![id],
            stop,
        )?;
    }
    Ok(Some(id))
}
