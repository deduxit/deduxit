// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The clausal encodings: Tseitin's and Plaisted and Greenbaum's, a new
//! atom for every compound subformula with the clauses that define it.

use super::first_order::fresh_prefix;
use super::laws::{Budget, Passes};
use super::{Form, Made, Rule, Step, disjunction};
use crate::Error;
use crate::ordinary::polarity::{FALSE, TRUE, polarities};
use crate::ordinary::{Formulas, Node, NodeId, Sequent};
use crate::hash::HashMap;

/// Returns whether `id` is a literal of a clause: an atom, a constant or
/// the negation of one.
fn is_literal(formulas: &Formulas, id: NodeId) -> bool {
    match formulas.node(id) {
        Node::Atom(_) | Node::True | Node::False => true,
        Node::Not(a) => matches!(formulas.node(a), Node::Atom(_) | Node::True | Node::False),
        Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => false,
        Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
    }
}

/// Returns the negation of the literal `id`, without a double negation.
fn negated(formulas: &mut Formulas, id: NodeId) -> Result<NodeId, Error> {
    match formulas.node(id) {
        Node::Not(a) => Ok(a),
        Node::True => formulas.add(Node::False),
        Node::False => formulas.add(Node::True),
        Node::Atom(_) | Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {
            formulas.add(Node::Not(id))
        }
        Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
    }
}

/// Moves the formulas right of `sequent`'s `⊢` left, each negated in
/// `formulas` by a `Negate` step of its own in `steps`, and returns the left
/// side with them.
pub(super) fn negated_right(
    sequent: &Sequent,
    formulas: &mut Formulas,
    steps: &mut Vec<Step>,
    budget: &mut Budget<'_>,
) -> Result<Vec<NodeId>, Error> {
    let mut left = sequent.left().to_vec();
    for (k, &d) in sequent.right().iter().enumerate() {
        let nodes = formulas.len();
        let after = formulas.add(Node::Not(d))?;
        left.push(after);
        let step = Step {
            rule: Rule::Negate,
            before: d,
            after,
            left: left.clone(),
            right: sequent.right()[k + 1..].to_vec(),
            added: Vec::new(),
            term: None,
        };
        budget.hold(&step, formulas.len() - nodes)?;
        steps.push(step);
    }
    Ok(left)
}

/// Encodes the sequent's negation as clauses: the formulas right of `⊢`
/// moved left negated, then every compound subformula named by a new atom,
/// the innermost first, with the clauses of its definition.
pub(super) fn encoded(
    sequent: &Sequent,
    form: Form,
    budget: &mut Budget<'_>,
) -> Result<Made, Error> {
    let mut formulas = sequent.formulas().clone();
    let mut steps = Vec::new();
    let mut passes = Passes::default();
    let mut left = negated_right(sequent, &mut formulas, &mut steps, budget)?;
    let size = passes.reach(&formulas, &left);
    budget.pass(passes.order.len(), size, steps.len())?;
    let order = passes.order.clone();
    // The classical encoder's rule: the left side true.
    let polarity = polarities(&formulas, &left, &[]);
    // A propositional arena has no symbols, so only the atoms are
    // avoided.
    let prefix = fresh_prefix(&formulas, "t");
    // The clauses so far, and their occurrences.
    let mut clauses: Vec<NodeId> = Vec::new();
    let mut clauses_size = 0u64;
    // What each compound node of the input is named.
    let mut name: HashMap<NodeId, NodeId> = HashMap::default();
    let mut count = 0u64;
    for &x in &order {
        if is_literal(&formulas, x) {
            continue;
        }
        let nodes = formulas.len();
        let current = |k: NodeId| name.get(&k).copied().unwrap_or(k);
        let node = formulas.node(x);
        let before = formulas.add(match node {
            Node::Atom(_) | Node::True | Node::False => node,
            Node::Not(a) => Node::Not(current(a)),
            Node::And(a, b) => Node::And(current(a), current(b)),
            Node::Or(a, b) => Node::Or(current(a), current(b)),
            Node::Implies(a, b) => Node::Implies(current(a), current(b)),
            Node::Iff(a, b) => Node::Iff(current(a), current(b)),
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        })?;
        // A negation of a named subformula is a literal already.
        if is_literal(&formulas, before) {
            name.insert(x, before);
            continue;
        }
        count += 1;
        let t = formulas.atom(&format!("{prefix}{count}"))?;
        let p = polarity[x.index()];
        let (forward, backward) = match form {
            Form::PlaistedGreenbaum => (p & TRUE != 0, p & FALSE != 0),
            Form::Nnf
            | Form::Cnf
            | Form::Dnf
            | Form::Tseitin
            | Form::Prenex
            | Form::Miniscope
            | Form::Skolem
            | Form::Herbrand
            | Form::Clausal => (true, true),
        };
        let added = definition(&mut formulas, before, t, forward, backward)?;
        name.insert(x, t);
        passes.reach(&formulas, &left);
        let reached = passes.order.len();
        left = passes.substitute(&mut formulas, &left, before, t)?;
        for &c in &added {
            clauses_size = clauses_size.saturating_add(passes.reach(&formulas, &[c]));
        }
        let size = passes.reach(&formulas, &left);
        clauses.extend_from_slice(&added);
        let step = Step {
            rule: Rule::Define,
            before,
            after: t,
            left: left.clone(),
            right: Vec::new(),
            added,
            term: None,
        };
        budget.pass(reached, size.saturating_add(clauses_size), steps.len())?;
        budget.hold(&step, formulas.len() - nodes)?;
        steps.push(step);
    }
    // The formulas of the input are literals now, unit clauses. A negated
    // constant is rewritten and `⊤`, which holds anyway, is left out, each
    // a step, so that the last step's sequent is the result.
    for (rule, constant, value) in [
        (Rule::NotTrue, Node::True, Node::False),
        (Rule::NotFalse, Node::False, Node::True),
    ] {
        let Some(before) = formulas
            .find(constant)
            .and_then(|c| formulas.find(Node::Not(c)))
            .filter(|before| left.contains(before))
        else {
            continue;
        };
        let nodes = formulas.len();
        let after = formulas.add(value)?;
        for id in &mut left {
            if *id == before {
                *id = after;
            }
        }
        let step = Step {
            rule,
            before,
            after,
            left: left.clone(),
            right: Vec::new(),
            added: Vec::new(),
            term: None,
        };
        budget.hold(&step, formulas.len() - nodes)?;
        steps.push(step);
    }
    if let Some(truth) = formulas.find(Node::True).filter(|t| left.contains(t)) {
        left.retain(|&id| id != truth);
        let step = Step {
            rule: Rule::TrueHypothesis,
            before: truth,
            after: truth,
            left: left.clone(),
            right: Vec::new(),
            added: Vec::new(),
            term: None,
        };
        budget.hold(&step, 0)?;
        steps.push(step);
    }
    left.extend_from_slice(&clauses);
    Ok((formulas, left, Vec::new(), steps))
}

/// Returns the clauses that tie the new atom `t` to `before`, a compound
/// formula over literals: those of `t → before` where `forward` is set,
/// of `before → t` where `backward` is. A clause with `⊤` is left out and
/// `⊥` is dropped from a clause, the empty clause being `⊥`.
fn definition(
    formulas: &mut Formulas,
    before: NodeId,
    t: NodeId,
    forward: bool,
    backward: bool,
) -> Result<Vec<NodeId>, Error> {
    let nt = formulas.add(Node::Not(t))?;
    let lit = |f: &mut Formulas, k: NodeId, positive: bool| {
        if positive { Ok(k) } else { negated(f, k) }
    };
    // Each clause as literals: (operand, sign), with the atom's literal
    // first.
    let mut lists: Vec<Vec<NodeId>> = Vec::new();
    match formulas.node(before) {
        Node::Atom(_) | Node::True | Node::False => {
            unreachable!("a literal is named by no atom")
        }
        Node::Not(a) => {
            if forward {
                lists.push(vec![nt, lit(formulas, a, false)?]);
            }
            if backward {
                lists.push(vec![t, a]);
            }
        }
        Node::And(a, b) => {
            if forward {
                lists.push(vec![nt, a]);
                lists.push(vec![nt, b]);
            }
            if backward {
                lists.push(vec![t, lit(formulas, a, false)?, lit(formulas, b, false)?]);
            }
        }
        Node::Or(a, b) => {
            if forward {
                lists.push(vec![nt, a, b]);
            }
            if backward {
                lists.push(vec![t, lit(formulas, a, false)?]);
                lists.push(vec![t, lit(formulas, b, false)?]);
            }
        }
        Node::Implies(a, b) => {
            if forward {
                lists.push(vec![nt, lit(formulas, a, false)?, b]);
            }
            if backward {
                lists.push(vec![t, a]);
                lists.push(vec![t, lit(formulas, b, false)?]);
            }
        }
        Node::Iff(a, b) => {
            if forward {
                lists.push(vec![nt, lit(formulas, a, false)?, b]);
                lists.push(vec![nt, a, lit(formulas, b, false)?]);
            }
            if backward {
                lists.push(vec![t, a, b]);
                lists.push(vec![t, lit(formulas, a, false)?, lit(formulas, b, false)?]);
            }
        }
        Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
    }
    let mut clauses = Vec::with_capacity(lists.len());
    for list in lists {
        if list.iter().any(|&k| constant(formulas, k) == Some(true)) {
            continue;
        }
        let kept: Vec<NodeId> = list
            .into_iter()
            .filter(|&k| constant(formulas, k).is_none())
            .collect();
        clauses.push(disjunction(formulas, &kept)?);
    }
    Ok(clauses)
}

/// Returns the value of a literal that is a constant or a negated one:
/// `⊤` and `¬⊥` true, `⊥` and `¬⊤` false; `None` for any other formula.
fn constant(formulas: &Formulas, id: NodeId) -> Option<bool> {
    match formulas.node(id) {
        Node::True => Some(true),
        Node::False => Some(false),
        Node::Not(a) => match formulas.node(a) {
            Node::True => Some(false),
            Node::False => Some(true),
            Node::Atom(_)
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        },
        Node::Atom(_) | Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => None,
        Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
    }
}
