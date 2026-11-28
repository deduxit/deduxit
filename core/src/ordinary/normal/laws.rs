// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The rewriting by laws: the stages of each form, the laws that fit a
//! formula at a stage and their application, the passes that find the
//! leftmost outermost place, the bound on the sequents made, and the run
//! of the stages.

use super::first_order::{apply_quantifier, quantifier_law, skolem_term};
use super::{Form, Made, Rule, Step};
use crate::Error;
use crate::first_order::FoTermId;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::{Formulas, Node, NodeId, Sequent};

/// A phase of the rewriting into an equivalent form: the laws it applies.
#[derive(Clone, Copy, Debug)]
pub(super) enum Stage {
    /// `→` and `↔` rewritten, `↔` by cases where `cases` is set.
    Arrows {
        /// Whether `↔` becomes a disjunction of cases, for the
        /// disjunctive form.
        cases: bool,
    },
    /// Negations moved inwards to the atoms.
    Negations,
    /// `⊤` and `⊥` dropped from conjunctions and disjunctions.
    Constants,
    /// `∨` distributed over `∧` (`conjunctive`) or `∧` over `∨`.
    Distribute {
        /// Whether the result is a conjunction of clauses.
        conjunctive: bool,
    },
    /// `↔` split into two implications, on the way to prenex form.
    Iffs,
    /// Quantifiers moved out to the front: of every kind, or `∀` alone
    /// over `∧` and `∨` where `universal` is set (the clausal form, which
    /// has no `∃` left).
    Outward {
        /// Whether only `∀` moves.
        universal: bool,
    },
    /// Quantifiers moved in as far as they go.
    Inward,
    /// `∀` distributed over `∧` and vacuous `∀`s left out, so that each
    /// clause has its own prefix.
    Split,
    /// The strong quantifiers to be replaced by their terms: `∃` where
    /// `exists` is set, else `∀`.
    Strong {
        /// Whether the quantifier is `∃`.
        exists: bool,
    },
}

/// Returns the phases of an equivalent form, in order.
pub(super) const fn stages(form: Form) -> &'static [Stage] {
    match form {
        Form::Nnf => &[Stage::Arrows { cases: false }, Stage::Negations],
        Form::Cnf => &[
            Stage::Arrows { cases: false },
            Stage::Negations,
            Stage::Constants,
            Stage::Distribute { conjunctive: true },
        ],
        Form::Dnf => &[
            Stage::Arrows { cases: true },
            Stage::Negations,
            Stage::Constants,
            Stage::Distribute { conjunctive: false },
        ],
        Form::Prenex => &[Stage::Iffs, Stage::Outward { universal: false }],
        Form::Miniscope => &[
            Stage::Arrows { cases: false },
            Stage::Negations,
            Stage::Inward,
        ],
        Form::Tseitin | Form::PlaistedGreenbaum | Form::Skolem | Form::Herbrand | Form::Clausal => {
            &[]
        }
    }
}

/// Returns whether a node is `⊤`, `⊥`, `∧` or `∨`, by the kind asked.
pub(super) const fn is(node: Node, kind: Kind) -> bool {
    matches!(
        (kind, node),
        (Kind::True, Node::True)
            | (Kind::False, Node::False)
            | (Kind::And, Node::And(..))
            | (Kind::Or, Node::Or(..))
    )
}

/// The kinds of node a law's table asks about.
#[derive(Clone, Copy)]
pub(super) enum Kind {
    /// `⊤`
    True,
    /// `⊥`
    False,
    /// `∧`
    And,
    /// `∨`
    Or,
}

/// Returns the law of `stage` that applies at the root of `id`, if one
/// does. The table of the rewriting: [`apply`] makes what each law gives.
pub(super) fn law(stage: Stage, formulas: &Formulas, id: NodeId) -> Option<Rule> {
    let node = formulas.node(id);
    let either =
        |a: NodeId, b: NodeId, kind| is(formulas.node(a), kind) || is(formulas.node(b), kind);
    match stage {
        Stage::Arrows { cases } => match node {
            Node::Implies(..) => Some(Rule::Implication),
            Node::Iff(..) if cases => Some(Rule::EquivalenceAsCases),
            Node::Iff(..) => Some(Rule::Equivalence),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..) => None,
            Node::Forall(_) | Node::Exists(_) => None,
        },
        Stage::Negations => match node {
            Node::Not(a) => match formulas.node(a) {
                Node::Not(_) => Some(Rule::DoubleNegation),
                Node::And(..) => Some(Rule::DeMorganAnd),
                Node::Or(..) => Some(Rule::DeMorganOr),
                Node::True => Some(Rule::NotTrue),
                Node::False => Some(Rule::NotFalse),
                Node::Forall(_) => Some(Rule::NotForall),
                Node::Exists(_) => Some(Rule::NotExists),
                // No arrow is left after the first phase.
                Node::Atom(_) | Node::Implies(..) | Node::Iff(..) => None,
            },
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => None,
        },
        Stage::Constants => match node {
            Node::And(a, b) if either(a, b, Kind::False) => Some(Rule::AndFalse),
            Node::And(a, b) if either(a, b, Kind::True) => Some(Rule::AndTrue),
            Node::Or(a, b) if either(a, b, Kind::True) => Some(Rule::OrTrue),
            Node::Or(a, b) if either(a, b, Kind::False) => Some(Rule::OrFalse),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => None,
        },
        Stage::Distribute { conjunctive } => match node {
            Node::Or(a, b) if conjunctive && either(a, b, Kind::And) => Some(Rule::DistributeOr),
            Node::And(a, b) if !conjunctive && either(a, b, Kind::Or) => Some(Rule::DistributeAnd),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => None,
        },
        Stage::Iffs
        | Stage::Outward { .. }
        | Stage::Inward
        | Stage::Split
        | Stage::Strong { .. } => quantifier_law(stage, formulas, id),
    }
}

/// Returns what `rule` makes of the formula `id`, at whose root [`law`]
/// found it, added to the arena.
pub(super) fn apply(rule: Rule, formulas: &mut Formulas, id: NodeId) -> Result<NodeId, Error> {
    if !matches!(
        rule,
        Rule::Implication
            | Rule::Equivalence
            | Rule::EquivalenceAsCases
            | Rule::DoubleNegation
            | Rule::DeMorganAnd
            | Rule::DeMorganOr
            | Rule::NotTrue
            | Rule::NotFalse
            | Rule::AndTrue
            | Rule::AndFalse
            | Rule::OrTrue
            | Rule::OrFalse
            | Rule::DistributeOr
            | Rule::DistributeAnd
    ) {
        return apply_quantifier(rule, formulas, id);
    }
    let node = formulas.node(id);
    let not = |f: &mut Formulas, a| f.add(Node::Not(a));
    let (a, b) = node.operands();
    let (a, b) = (
        a.unwrap_or_else(|| unreachable!("every law rewrites a compound formula")),
        b.unwrap_or(a.unwrap_or_else(|| unreachable!("checked above"))),
    );
    let inner = |f: &Formulas, k: NodeId| f.node(k).operands();
    match rule {
        Rule::Implication => {
            let na = not(formulas, a)?;
            formulas.add(Node::Or(na, b))
        }
        Rule::Equivalence => {
            let (na, nb) = (not(formulas, a)?, not(formulas, b)?);
            let one = formulas.add(Node::Or(na, b))?;
            let two = formulas.add(Node::Or(a, nb))?;
            formulas.add(Node::And(one, two))
        }
        Rule::EquivalenceAsCases => {
            let (na, nb) = (not(formulas, a)?, not(formulas, b)?);
            let one = formulas.add(Node::And(a, b))?;
            let two = formulas.add(Node::And(na, nb))?;
            formulas.add(Node::Or(one, two))
        }
        Rule::DoubleNegation => Ok(inner(formulas, a)
            .0
            .unwrap_or_else(|| unreachable!("a double negation"))),
        Rule::DeMorganAnd | Rule::DeMorganOr => {
            let (Some(c), Some(d)) = inner(formulas, a) else {
                unreachable!("a negated binary formula")
            };
            let (nc, nd) = (not(formulas, c)?, not(formulas, d)?);
            formulas.add(if rule == Rule::DeMorganAnd {
                Node::Or(nc, nd)
            } else {
                Node::And(nc, nd)
            })
        }
        Rule::NotTrue | Rule::AndFalse => formulas.add(Node::False),
        Rule::NotFalse | Rule::OrTrue => formulas.add(Node::True),
        // The operand that is not the constant.
        Rule::AndTrue => Ok(if is(formulas.node(a), Kind::True) {
            b
        } else {
            a
        }),
        Rule::OrFalse => Ok(if is(formulas.node(a), Kind::False) {
            b
        } else {
            a
        }),
        Rule::DistributeOr | Rule::DistributeAnd => {
            let (outer, kind): (fn(NodeId, NodeId) -> Node, Kind) = if rule == Rule::DistributeOr {
                (Node::And, Kind::And)
            } else {
                (Node::Or, Kind::Or)
            };
            let inner_of = if rule == Rule::DistributeOr {
                Node::Or
            } else {
                Node::And
            };
            // `A ∨ (B ∧ C)`, else `(B ∧ C) ∨ A`, which keeps the order.
            if is(formulas.node(b), kind) {
                let (Some(c), Some(d)) = inner(formulas, b) else {
                    unreachable!("a binary operand")
                };
                let one = formulas.add(inner_of(a, c))?;
                let two = formulas.add(inner_of(a, d))?;
                formulas.add(outer(one, two))
            } else {
                let (Some(c), Some(d)) = inner(formulas, a) else {
                    unreachable!("a binary operand")
                };
                let one = formulas.add(inner_of(c, b))?;
                let two = formulas.add(inner_of(d, b))?;
                formulas.add(outer(one, two))
            }
        }
        Rule::Negate
        | Rule::Define
        | Rule::TrueHypothesis
        | Rule::NotForall
        | Rule::NotExists
        | Rule::ForallAnd
        | Rule::ExistsAnd
        | Rule::ForallOr
        | Rule::ExistsOr
        | Rule::ForallAntecedent
        | Rule::ExistsAntecedent
        | Rule::ForallConsequent
        | Rule::ExistsConsequent
        | Rule::EquivalenceAsImplications
        | Rule::ForallDistribute
        | Rule::ExistsDistribute
        | Rule::Vacuous
        | Rule::Skolemize
        | Rule::Herbrandize
        | Rule::SplitConjunction => unreachable!("taken above or no law of an equivalent form"),
    }
}

/// The scratch of the passes over the formulas a sequent reaches, kept
/// between steps: arrays indexed by node, valid where `seen` holds the
/// current pass's stamp, so that a pass costs what it reaches and not
/// the whole arena.
#[derive(Default)]
pub(super) struct Passes {
    /// The pass that last reached each node.
    seen: Vec<u32>,
    /// The current pass.
    stamp: u32,
    /// The nodes reached, in the arena's order: every operand before the
    /// formulas built from it.
    pub(super) order: Vec<NodeId>,
    /// The stack of the walk.
    stack: Vec<NodeId>,
    /// Per node: whether a law applies at or below it.
    found: Vec<bool>,
    /// Per node: the occurrences it unfolds to, saturating.
    size: Vec<u64>,
    /// Per node: what it becomes in a substitution.
    image: Vec<NodeId>,
}

impl Passes {
    /// Collects the nodes `roots` reach into `order`, in the arena's order,
    /// and their unfolded sizes, and returns the sequent's size.
    pub(super) fn reach(&mut self, formulas: &Formulas, roots: &[NodeId]) -> u64 {
        let n = formulas.len();
        self.seen.resize(n, 0);
        self.found.resize(n, false);
        self.size.resize(n, 0);
        self.image.resize(n, NodeId(0));
        // A pass per step: `u32` passes would need more steps than the
        // occurrence bound and the work allow.
        self.stamp = self.stamp.wrapping_add(1);
        if self.stamp == 0 {
            self.seen.fill(0);
            self.stamp = 1;
        }
        self.order.clear();
        self.stack.clear();
        self.stack.extend_from_slice(roots);
        while let Some(id) = self.stack.pop() {
            if self.seen[id.index()] == self.stamp {
                continue;
            }
            self.seen[id.index()] = self.stamp;
            self.order.push(id);
            let (a, b) = formulas.node(id).operands();
            self.stack.extend([a, b].into_iter().flatten());
        }
        self.order.sort_unstable();
        for &id in &self.order {
            let (a, b) = formulas.node(id).operands();
            let of = |k: Option<NodeId>| k.map_or(0, |k| self.size[k.index()]);
            self.size[id.index()] = 1u64.saturating_add(of(a)).saturating_add(of(b));
        }
        roots
            .iter()
            .fold(0u64, |sum, r| sum.saturating_add(self.size[r.index()]))
    }

    /// Returns the leftmost outermost place among `roots`, whose nodes
    /// [`reach`](Self::reach) collected, where a law of `stage` applies,
    /// and the law.
    fn redex(
        &mut self,
        stage: Stage,
        formulas: &Formulas,
        roots: &[NodeId],
    ) -> Option<(NodeId, Rule)> {
        for &id in &self.order {
            let (a, b) = formulas.node(id).operands();
            let below = [a, b].into_iter().flatten().any(|k| self.found[k.index()]);
            self.found[id.index()] = below || law(stage, formulas, id).is_some();
        }
        let root = *roots.iter().find(|r| self.found[r.index()])?;
        let mut at = root;
        loop {
            if let Some(rule) = law(stage, formulas, at) {
                return Some((at, rule));
            }
            let (a, b) = formulas.node(at).operands();
            at = [a, b]
                .into_iter()
                .flatten()
                .find(|k| self.found[k.index()])
                .unwrap_or_else(|| unreachable!("a law applies below"));
        }
    }

    /// Returns `roots` with every occurrence of `before`, among the nodes
    /// [`reach`](Self::reach) collected, replaced by `after`.
    pub(super) fn substitute(
        &mut self,
        formulas: &mut Formulas,
        roots: &[NodeId],
        before: NodeId,
        after: NodeId,
    ) -> Result<Vec<NodeId>, Error> {
        for k in 0..self.order.len() {
            let id = self.order[k];
            let new = if id == before {
                after
            } else {
                let node = formulas.node(id);
                let map = |x: NodeId| self.image[x.index()];
                let mapped = match node {
                    Node::Atom(_) | Node::True | Node::False => node,
                    Node::Not(a) => Node::Not(map(a)),
                    Node::And(a, b) => Node::And(map(a), map(b)),
                    Node::Or(a, b) => Node::Or(map(a), map(b)),
                    Node::Implies(a, b) => Node::Implies(map(a), map(b)),
                    Node::Iff(a, b) => Node::Iff(map(a), map(b)),
                    Node::Forall(a) => Node::Forall(map(a)),
                    Node::Exists(a) => Node::Exists(map(a)),
                };
                if mapped == node {
                    id
                } else {
                    formulas.add_like(mapped, id)?
                }
            };
            self.image[id.index()] = new;
        }
        Ok(roots.iter().map(|r| self.image[r.index()]).collect())
    }
}

/// What a rewriting has done and may still do: its bounds.
pub(super) struct Budget<'a> {
    /// The bounds.
    pub(super) limits: &'a Limits,
    /// The work done: the nodes every pass reached.
    pub(super) work: u64,
    /// The bytes the steps and the arena hold, estimated.
    pub(super) held: u64,
    /// The stop.
    pub(super) stop: &'a mut dyn FnMut(Progress) -> bool,
}

/// The bytes a node of the arena is charged: the node, its id and its
/// entry in the arena's table of ids.
const NODE_BYTES: u64 = 48;

/// The bytes a step is charged beside its ids.
const STEP_BYTES: u64 = size_of::<Step>() as u64;

impl Budget<'_> {
    /// Charges a pass over `reached` nodes and the sequent's `size`, asks
    /// the stop, and refuses past a bound.
    pub(super) fn pass(&mut self, reached: usize, size: u64, steps: usize) -> Result<(), Error> {
        let reached = reached as u64;
        self.work = self.work.saturating_add(reached);
        if let Some(limit) = self.limits.form_occurrences
            && size > limit
        {
            return Err(Error::Refused(Refusal::Form {
                occurrences: size,
                limit,
            }));
        }
        if let Some(limit) = self.limits.work
            && self.work > limit
        {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        if (self.stop)(Progress::new(Phase::Rewrite, reached, steps as u64)) {
            return Err(Error::Refused(Refusal::stopped(Phase::Rewrite)));
        }
        Ok(())
    }

    /// Charges a step and the arena's growth, and refuses past the memory
    /// bound.
    pub(super) fn hold(&mut self, step: &Step, nodes: usize) -> Result<(), Error> {
        let ids = step.left.len() + step.right.len() + step.added.len();
        self.held = self
            .held
            .saturating_add(STEP_BYTES + 4 * ids as u64)
            .saturating_add(NODE_BYTES * nodes as u64);
        match self.limits.memory_bytes {
            Some(limit_bytes) if self.held > limit_bytes => Err(Error::Refused(Refusal::Memory {
                phase: Phase::Rewrite,
                limit_bytes,
                needed_bytes: None,
            })),
            _ => Ok(()),
        }
    }
}

/// Rewrites into an equivalent form, phase by phase, the leftmost
/// outermost place a law applies first.
pub(super) fn equivalent(
    sequent: &Sequent,
    form: Form,
    budget: &mut Budget<'_>,
) -> Result<Made, Error> {
    let mut formulas = sequent.formulas().clone();
    let split = sequent.left().len();
    let mut roots: Vec<NodeId> = sequent
        .left()
        .iter()
        .chain(sequent.right())
        .copied()
        .collect();
    let mut passes = Passes::default();
    let mut steps = Vec::new();
    let mut run = Run {
        formulas: &mut formulas,
        roots: &mut roots,
        split,
        steps: &mut steps,
        passes: &mut passes,
        budget,
    };
    run.stages(stages(form))?;
    let (left, right) = (roots[..split].to_vec(), roots[split..].to_vec());
    Ok((formulas, left, right, steps))
}

/// A rewriting under way: the arena, the formulas of the input as they
/// stand (the first `split` left of `⊢`), the steps so far, the passes'
/// scratch and the bounds.
pub(super) struct Run<'r, 'b> {
    /// The arena, which grows with every step.
    pub(super) formulas: &'r mut Formulas,
    /// The input's formulas as they stand.
    pub(super) roots: &'r mut Vec<NodeId>,
    /// How many of them stand left of `⊢`.
    pub(super) split: usize,
    /// The steps so far.
    pub(super) steps: &'r mut Vec<Step>,
    /// The passes' scratch.
    pub(super) passes: &'r mut Passes,
    /// The bounds.
    pub(super) budget: &'r mut Budget<'b>,
}

impl Run<'_, '_> {
    /// Runs the phases `stages`, each until no law of it applies, the
    /// leftmost outermost place first.
    pub(super) fn stages(&mut self, stages: &[Stage]) -> Result<(), Error> {
        for &stage in stages {
            loop {
                let size = self.passes.reach(self.formulas, self.roots);
                self.budget
                    .pass(self.passes.order.len(), size, self.steps.len())?;
                let Some((before, rule)) = self.passes.redex(stage, self.formulas, self.roots)
                else {
                    break;
                };
                let nodes = self.formulas.len();
                let after = apply(rule, self.formulas, before)?;
                *self.roots = self
                    .passes
                    .substitute(self.formulas, self.roots, before, after)?;
                self.record(rule, before, after, None, nodes)?;
            }
        }
        Ok(())
    }

    /// Records a step that replaced `before` by `after`, the arena `nodes`
    /// long before it.
    fn record(
        &mut self,
        rule: Rule,
        before: NodeId,
        after: NodeId,
        term: Option<FoTermId>,
        nodes: usize,
    ) -> Result<(), Error> {
        let step = Step {
            rule,
            before,
            after,
            left: self.roots[..self.split].to_vec(),
            right: self.roots[self.split..].to_vec(),
            added: Vec::new(),
            term,
        };
        self.budget.hold(&step, self.formulas.len() - nodes)?;
        self.steps.push(step);
        Ok(())
    }

    /// Splits each conjunction left of `⊢` into its two parts and leaves out
    /// `⊤` there, each a step, so that the clauses stand as hypotheses one
    /// by one.
    pub(super) fn hypotheses(&mut self) -> Result<(), Error> {
        let at = |run: &Self, wanted: &dyn Fn(Node) -> bool| {
            run.roots[..run.split]
                .iter()
                .position(|&r| wanted(run.formulas.node(r)))
        };
        while let Some(k) = at(self, &|node| matches!(node, Node::And(..))) {
            self.budget.pass(0, 0, self.steps.len())?;
            let conjunction = self.roots[k];
            let (Some(a), Some(b)) = self.formulas.node(conjunction).operands() else {
                unreachable!("a conjunction has two operands")
            };
            self.roots.splice(k..=k, [a, b]);
            self.split += 1;
            let nodes = self.formulas.len();
            self.record(
                Rule::SplitConjunction,
                conjunction,
                conjunction,
                None,
                nodes,
            )?;
        }
        while let Some(k) = at(self, &|node| matches!(node, Node::True)) {
            let truth = self.roots.remove(k);
            self.split -= 1;
            let nodes = self.formulas.len();
            self.record(Rule::TrueHypothesis, truth, truth, None, nodes)?;
        }
        Ok(())
    }

    /// Replaces each strong quantifier of the formulas in `range` (in
    /// negation normal form, where every quantifier stands positive: `∃`
    /// where `exists` is set, else `∀`), the outermost first, by its term:
    /// a new function symbol named by `prefix` and a number past `count`,
    /// applied to the variables free in the quantified formula. Equal
    /// quantified formulas are one node and share their symbol.
    pub(super) fn strong(
        &mut self,
        range: std::ops::Range<usize>,
        exists: bool,
        rule: Rule,
        prefix: &str,
        count: &mut u64,
    ) -> Result<(), Error> {
        loop {
            let size = self.passes.reach(self.formulas, self.roots);
            self.budget
                .pass(self.passes.order.len(), size, self.steps.len())?;
            let side = self.roots[range.clone()].to_vec();
            self.passes.reach(self.formulas, &side);
            let Some((before, _)) =
                self.passes
                    .redex(Stage::Strong { exists }, self.formulas, &side)
            else {
                return Ok(());
            };
            let nodes = self.formulas.len();
            let (Node::Forall(body) | Node::Exists(body)) = self.formulas.node(before) else {
                unreachable!("the stage finds quantifiers")
            };
            *count += 1;
            let free = self.formulas.free_variables(before);
            let term = skolem_term(self.formulas, &format!("{prefix}{count}"), &free)?;
            if let Some(limit) = self.budget.limits.terms
                && free.len() as u64 + 1 > limit
            {
                return Err(Error::Refused(Refusal::Terms {
                    terms: free.len() as u64 + 1,
                    limit,
                }));
            }
            let after = self.formulas.substituted(body, term)?;
            let changed = self
                .passes
                .substitute(self.formulas, &side, before, after)?;
            self.roots.splice(range.clone(), changed);
            self.record(rule, before, after, Some(term), nodes)?;
        }
    }
}
