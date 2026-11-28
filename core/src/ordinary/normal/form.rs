// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The test of a result's form: whether a sequent is in the form a
//! normal form promises.

use super::laws::{Kind, Stage, is, law};
use super::{Form, Rule};
use crate::ordinary::{Formulas, Node, NodeId, Sequent};

/// Returns whether every formula of `result` is in `form`. A literal is
/// an atom or a negated atom; a constant stands only alone.
pub(super) fn in_form(result: &Sequent, form: Form) -> bool {
    let formulas = result.formulas();
    let literal = |id: NodeId| match formulas.node(id) {
        Node::Atom(_) => true,
        Node::Not(a) => matches!(formulas.node(a), Node::Atom(_)),
        Node::True
        | Node::False
        | Node::And(..)
        | Node::Or(..)
        | Node::Implies(..)
        | Node::Iff(..)
        | Node::Forall(_)
        | Node::Exists(_) => false,
    };
    // Whether a formula is built by `join` over parts that `part` accepts,
    // walked with a stack of its own.
    let built = |root: NodeId, join: Kind, part: &dyn Fn(NodeId) -> bool| {
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            let node = formulas.node(id);
            if is(node, join) {
                let (a, b) = node.operands();
                stack.extend([a, b].into_iter().flatten());
            } else if !part(id) {
                return false;
            }
        }
        true
    };
    let clause = |id: NodeId| built(id, Kind::Or, &literal);
    let cube = |id: NodeId| built(id, Kind::And, &literal);
    let alone = |id: NodeId| matches!(formulas.node(id), Node::True | Node::False);
    let roots = result.left().iter().chain(result.right());
    match form {
        Form::Nnf => roots.copied().all(|id| nnf(formulas, id)),
        Form::Cnf => roots
            .copied()
            .all(|id| alone(id) || built(id, Kind::And, &clause)),
        Form::Dnf => roots
            .copied()
            .all(|id| alone(id) || built(id, Kind::Or, &cube)),
        Form::Tseitin | Form::PlaistedGreenbaum => {
            result.right().is_empty()
                && result
                    .left()
                    .iter()
                    .all(|&id| matches!(formulas.node(id), Node::False) || clause(id))
        }
        Form::Prenex => roots
            .copied()
            .all(|id| !has_binder(formulas, prefixed(formulas, id, true))),
        Form::Miniscope => roots.copied().all(|id| {
            nnf(formulas, id) && !reaches(formulas, id, &|k| law(Stage::Inward, formulas, k))
        }),
        Form::Skolem | Form::Herbrand => {
            let strong = |exists: bool| move |k: NodeId| law(Stage::Strong { exists }, formulas, k);
            (form == Form::Herbrand || result.right().is_empty())
                && roots.clone().copied().all(|id| nnf(formulas, id))
                && result
                    .left()
                    .iter()
                    .all(|&id| !reaches(formulas, id, &strong(true)))
                && result
                    .right()
                    .iter()
                    .all(|&id| !reaches(formulas, id, &strong(false)))
        }
        Form::Clausal => {
            result.right().is_empty()
                && result.left().iter().all(|&id| {
                    let matrix = prefixed(formulas, id, false);
                    matches!(formulas.node(matrix), Node::False) || clause(matrix)
                })
        }
    }
}

/// Returns the formula under the quantifiers in front of `id`: every
/// quantifier where `both` is set, else `∀` alone.
fn prefixed(formulas: &Formulas, mut id: NodeId, both: bool) -> NodeId {
    loop {
        match formulas.node(id) {
            Node::Forall(a) => id = a,
            Node::Exists(a) if both => id = a,
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Exists(_) => return id,
        }
    }
}

/// Returns whether `id` holds a quantifier.
fn has_binder(formulas: &Formulas, id: NodeId) -> bool {
    reaches(formulas, id, &|k| {
        matches!(formulas.node(k), Node::Forall(_) | Node::Exists(_)).then_some(Rule::Vacuous)
    })
}

/// Returns whether some subformula of `id` is one `found` answers for,
/// walked with a stack of its own.
fn reaches(formulas: &Formulas, id: NodeId, found: &dyn Fn(NodeId) -> Option<Rule>) -> bool {
    let mut seen = crate::hash::HashSet::default();
    let mut stack = vec![id];
    while let Some(k) = stack.pop() {
        if !seen.insert(k) {
            continue;
        }
        if found(k).is_some() {
            return true;
        }
        let (a, b) = formulas.node(k).operands();
        stack.extend([a, b].into_iter().flatten());
    }
    false
}

/// Returns whether the formula `id` has no `→` or `↔` and a `¬` before
/// atoms alone, walked with a stack of its own.
fn nnf(formulas: &Formulas, id: NodeId) -> bool {
    let mut stack = vec![id];
    while let Some(id) = stack.pop() {
        match formulas.node(id) {
            Node::Atom(_) | Node::True | Node::False => {}
            Node::Not(a) => {
                if !matches!(formulas.node(a), Node::Atom(_)) {
                    return false;
                }
            }
            Node::And(a, b) | Node::Or(a, b) => stack.extend([a, b]),
            Node::Forall(a) | Node::Exists(a) => stack.push(a),
            Node::Implies(..) | Node::Iff(..) => return false,
        }
    }
    true
}
