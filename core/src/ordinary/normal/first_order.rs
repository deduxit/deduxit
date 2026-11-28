// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The first-order rewriting: the quantifier laws, Skolem's and Herbrand's
//! terms, and the fresh names they take.

use super::encode::negated_right;
use super::laws::{Budget, Passes, Run, Stage};
use super::{Form, Made, Rule};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
#[cfg(feature = "sat")]
use crate::limits::{Limits, Progress};
use crate::ordinary::{Formulas, Node, NodeId, Sequent};

/// Returns whether `id` mentions the variable of the binder right around
/// it, de Bruijn index 0 read outside it.
fn mentions_binder(formulas: &Formulas, id: NodeId) -> bool {
    formulas.free_variables(id).first() == Some(&0)
}

/// Returns the quantifier law of `stage` that applies at the root of `id`,
/// if one does: the first-order part of [`law`](super::laws::law)'s table.
pub(super) fn quantifier_law(stage: Stage, formulas: &Formulas, id: NodeId) -> Option<Rule> {
    let node = formulas.node(id);
    let kind = |k: NodeId| match formulas.node(k) {
        Node::Forall(_) => Some(true),
        Node::Exists(_) => Some(false),
        Node::Atom(_)
        | Node::True
        | Node::False
        | Node::Not(_)
        | Node::And(..)
        | Node::Or(..)
        | Node::Implies(..)
        | Node::Iff(..) => None,
    };
    match stage {
        Stage::Outward { universal } => match node {
            Node::Not(a) if !universal => match kind(a)? {
                true => Some(Rule::NotForall),
                false => Some(Rule::NotExists),
            },
            Node::And(a, b) | Node::Or(a, b) => {
                let and = matches!(node, Node::And(..));
                // The left operand first, a `∃` only where it may move.
                [a, b]
                    .into_iter()
                    .filter_map(kind)
                    .find(|&forall| forall || !universal)
                    .map(|forall| match (forall, and) {
                        (true, true) => Rule::ForallAnd,
                        (false, true) => Rule::ExistsAnd,
                        (true, false) => Rule::ForallOr,
                        (false, false) => Rule::ExistsOr,
                    })
            }
            Node::Implies(a, b) if !universal => match (kind(a), kind(b)) {
                (Some(true), _) => Some(Rule::ForallAntecedent),
                (Some(false), _) => Some(Rule::ExistsAntecedent),
                (None, Some(true)) => Some(Rule::ForallConsequent),
                (None, Some(false)) => Some(Rule::ExistsConsequent),
                (None, None) => None,
            },
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Forall(_)
            | Node::Exists(_) => None,
        },
        Stage::Inward => match node {
            Node::Forall(a) | Node::Exists(a) => {
                let forall = matches!(node, Node::Forall(_));
                if !mentions_binder(formulas, a) {
                    return Some(Rule::Vacuous);
                }
                match (forall, formulas.node(a)) {
                    (true, Node::And(..)) => Some(Rule::ForallDistribute),
                    (false, Node::Or(..)) => Some(Rule::ExistsDistribute),
                    (true, Node::Or(c, d)) | (false, Node::And(c, d))
                        if !mentions_binder(formulas, c) || !mentions_binder(formulas, d) =>
                    {
                        Some(if forall {
                            Rule::ForallOr
                        } else {
                            Rule::ExistsAnd
                        })
                    }
                    _ => None,
                }
            }
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
        },
        Stage::Split => match node {
            Node::Forall(a) if !mentions_binder(formulas, a) => Some(Rule::Vacuous),
            Node::Forall(a) if matches!(formulas.node(a), Node::And(..)) => {
                Some(Rule::ForallDistribute)
            }
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Forall(_)
            | Node::Exists(_) => None,
        },
        Stage::Strong { exists } => match node {
            Node::Exists(_) if exists => Some(Rule::Skolemize),
            Node::Forall(_) if !exists => Some(Rule::Herbrandize),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Forall(_)
            | Node::Exists(_) => None,
        },
        Stage::Iffs => match node {
            Node::Iff(..) => Some(Rule::EquivalenceAsImplications),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Forall(_)
            | Node::Exists(_) => None,
        },
        Stage::Arrows { .. } | Stage::Negations | Stage::Constants | Stage::Distribute { .. } => {
            None
        }
    }
}

/// Returns what the quantifier law `rule` makes of `id`, at whose root
/// [`law`](super::laws::law) found it: a binder moved out of a binary formula (the other
/// operand lifted under it), moved in (the operand that does not mention
/// its variable lowered out of it), or a negation moved past it.
pub(super) fn apply_quantifier(
    rule: Rule,
    formulas: &mut Formulas,
    id: NodeId,
) -> Result<NodeId, Error> {
    let node = formulas.node(id);
    let binder = |forall: bool, body: NodeId| {
        if forall {
            Node::Forall(body)
        } else {
            Node::Exists(body)
        }
    };
    let body_of = |f: &Formulas, q: NodeId| match f.node(q) {
        Node::Forall(a) => (true, a),
        Node::Exists(a) => (false, a),
        Node::Atom(_)
        | Node::True
        | Node::False
        | Node::Not(_)
        | Node::And(..)
        | Node::Or(..)
        | Node::Implies(..)
        | Node::Iff(..) => unreachable!("the law found a quantifier there"),
    };
    match (rule, node) {
        (Rule::EquivalenceAsImplications, Node::Iff(a, b)) => {
            let one = formulas.add(Node::Implies(a, b))?;
            let two = formulas.add(Node::Implies(b, a))?;
            formulas.add(Node::And(one, two))
        }
        (Rule::NotForall | Rule::NotExists, Node::Not(q)) => {
            let (forall, body) = body_of(formulas, q);
            let negated = formulas.add(Node::Not(body))?;
            formulas.add_like(binder(!forall, negated), q)
        }
        // Out of a junction: the quantified operand's binder goes around
        // both, the other operand lifted under it.
        (
            Rule::ForallAnd | Rule::ExistsAnd | Rule::ForallOr | Rule::ExistsOr,
            Node::And(a, b) | Node::Or(a, b),
        ) => {
            let forall = matches!(rule, Rule::ForallAnd | Rule::ForallOr);
            let first = matches!(formulas.node(a), Node::Forall(_) if forall)
                || matches!(formulas.node(a), Node::Exists(_) if !forall);
            let (q, other) = if first { (a, b) } else { (b, a) };
            let (_, body) = body_of(formulas, q);
            let lifted = formulas.lifted(other, 1)?;
            let (x, y) = if first {
                (body, lifted)
            } else {
                (lifted, body)
            };
            let joined = formulas.add(if matches!(node, Node::And(..)) {
                Node::And(x, y)
            } else {
                Node::Or(x, y)
            })?;
            formulas.add_like(binder(forall, joined), q)
        }
        // Out of an implication: out of its antecedent the quantifier
        // turns, out of its consequent it stays.
        (Rule::ForallAntecedent | Rule::ExistsAntecedent, Node::Implies(a, b)) => {
            let (forall, body) = body_of(formulas, a);
            let lifted = formulas.lifted(b, 1)?;
            let implies = formulas.add(Node::Implies(body, lifted))?;
            formulas.add_like(binder(!forall, implies), a)
        }
        (Rule::ForallConsequent | Rule::ExistsConsequent, Node::Implies(a, b)) => {
            let (forall, body) = body_of(formulas, b);
            let lifted = formulas.lifted(a, 1)?;
            let implies = formulas.add(Node::Implies(lifted, body))?;
            formulas.add_like(binder(forall, implies), b)
        }
        // In: a binder over a formula that does not mention its variable
        // goes, over a conjunction (`∀`) or disjunction (`∃`) it splits,
        // and past an operand that does not mention it the operand goes
        // out.
        (_, Node::Forall(body) | Node::Exists(body)) => {
            let forall = matches!(node, Node::Forall(_));
            let lowered = |f: &mut Formulas, k: NodeId| {
                f.lowered(k)?
                    .ok_or_else(|| unreachable!("the law checked the variable is not free"))
            };
            match (rule, formulas.node(body)) {
                (Rule::Vacuous, _) => lowered(formulas, body),
                (Rule::ForallDistribute, Node::And(c, d))
                | (Rule::ExistsDistribute, Node::Or(c, d)) => {
                    let one = formulas.add_like(binder(forall, c), id)?;
                    let two = formulas.add_like(binder(forall, d), id)?;
                    formulas.add(if forall {
                        Node::And(one, two)
                    } else {
                        Node::Or(one, two)
                    })
                }
                (Rule::ForallOr, Node::Or(c, d)) | (Rule::ExistsAnd, Node::And(c, d)) => {
                    let join = |x: NodeId, y: NodeId| {
                        if forall {
                            Node::Or(x, y)
                        } else {
                            Node::And(x, y)
                        }
                    };
                    if mentions_binder(formulas, d) {
                        let out = lowered(formulas, c)?;
                        let kept = formulas.add_like(binder(forall, d), id)?;
                        formulas.add(join(out, kept))
                    } else {
                        let kept = formulas.add_like(binder(forall, c), id)?;
                        let out = lowered(formulas, d)?;
                        formulas.add(join(kept, out))
                    }
                }
                _ => unreachable!("the law found this shape"),
            }
        }
        _ => unreachable!("the law found this shape"),
    }
}

/// Returns the term of the new function symbol `name` over the variables
/// `free` (outside indices, ascending), the outermost first: `name(x̄)`, a
/// constant where `free` is empty.
pub(super) fn skolem_term(
    formulas: &mut Formulas,
    name: &str,
    free: &[u32],
) -> Result<FoTermId, Error> {
    let symbol = formulas
        .interner()
        .symbol(name, free.len() as u32, false)
        .map_err(|fault| Error::Symbol {
            name: name.to_owned(),
            fault,
        })?;
    let args: Vec<FoTermId> = free
        .iter()
        .rev()
        .map(|&i| formulas.interner().term(FoTerm::Bound(i)))
        .collect();
    let list = formulas.interner().list(&args);
    Ok(formulas.interner().term(FoTerm::App(symbol, list)))
}

/// Returns `node`, a formula left of `⊢`, in negation normal form as
/// Skolem's and the clausal form make it before their strong quantifiers
/// go: the same laws in the same order, so that a closed instance of a
/// formula they rewrote meets their step's quantifier again.
///
/// # Errors
///
/// [`Refusal::Form`](crate::Refusal::Form),
/// [`Refusal::Work`](crate::Refusal::Work) and
/// [`Refusal::Memory`](crate::Refusal::Memory) past the bounds of `limits`;
/// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
#[cfg(feature = "sat")]
pub(in crate::ordinary) fn negation_normal(
    formulas: &mut Formulas,
    node: NodeId,
    limits: &Limits,
) -> Result<NodeId, Error> {
    let mut never = |_: Progress| false;
    let mut budget = Budget {
        limits,
        work: 0,
        held: 0,
        stop: &mut never,
    };
    let mut roots = vec![node];
    let (mut steps, mut passes) = (Vec::new(), Passes::default());
    let mut run = Run {
        formulas,
        roots: &mut roots,
        split: 1,
        steps: &mut steps,
        passes: &mut passes,
        budget: &mut budget,
    };
    run.stages(&[Stage::Arrows { cases: false }, Stage::Negations])?;
    Ok(roots[0])
}

/// Returns the prefix of the new function symbols' names: `base`, or
/// `base` with as many `_` after it as make no symbol or atom of the arena
/// the prefix and digits.
pub(in crate::ordinary) fn fresh_prefix(formulas: &Formulas, base: &str) -> String {
    let mut prefix = String::from(base);
    let taken = |prefix: &str| {
        let numbered = |name: &str| {
            name.strip_prefix(prefix)
                .is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|c| c.is_ascii_digit()))
        };
        formulas.atom_names().iter().any(|name| numbered(name))
            || formulas
                .first_order()
                .symbols
                .iter()
                .any(|(name, _)| numbered(name))
    };
    while taken(&prefix) {
        prefix.push('_');
    }
    prefix
}

/// Makes Skolem's, Herbrand's or the clausal normal form: Skolem's and the
/// clausal form move the formulas right of `⊢` left negated, then every
/// formula goes to negation normal form and each strong quantifier is
/// replaced by its term; the clausal form then moves the `∀`s left to the
/// front, distributes, gives each clause its own prefix and splits the
/// conjunctions into hypotheses.
pub(super) fn skolemized(
    sequent: &Sequent,
    form: Form,
    budget: &mut Budget<'_>,
) -> Result<Made, Error> {
    let mut formulas = sequent.formulas().clone();
    let mut left = sequent.left().to_vec();
    let mut right = sequent.right().to_vec();
    let mut steps = Vec::new();
    let refutation = form != Form::Herbrand;
    if refutation {
        left = negated_right(sequent, &mut formulas, &mut steps, budget)?;
        right.clear();
    }
    let split = left.len();
    let mut roots: Vec<NodeId> = left.iter().chain(&right).copied().collect();
    let mut passes = Passes::default();
    let prefix = fresh_prefix(&formulas, "sk");
    let mut count = 0u64;
    let mut run = Run {
        formulas: &mut formulas,
        roots: &mut roots,
        split,
        steps: &mut steps,
        passes: &mut passes,
        budget,
    };
    run.stages(&[Stage::Arrows { cases: false }, Stage::Negations])?;
    let rule = if refutation {
        Rule::Skolemize
    } else {
        Rule::Herbrandize
    };
    let all = run.roots.len();
    run.strong(0..split, true, rule, &prefix, &mut count)?;
    run.strong(split..all, false, rule, &prefix, &mut count)?;
    if form == Form::Clausal {
        run.stages(&[
            Stage::Outward { universal: true },
            Stage::Constants,
            Stage::Distribute { conjunctive: true },
            Stage::Split,
        ])?;
        run.hypotheses()?;
    }
    let split = run.split;
    let (left, right) = (roots[..split].to_vec(), roots[split..].to_vec());
    Ok((formulas, left, right, steps))
}
