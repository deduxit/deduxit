// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The search for ground instances: the clauses of the clausal form, the
//! universe of ground terms deepened level by level, and the
//! propositional decision of the instances found.

use super::{Instance, MOST_CUT, Statistics};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId, Symbol};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::search::Reason;
use crate::ordinary::derivation::ArenaSize;
use crate::ordinary::{Formulas, Logic, Node, NodeId, Sequent, Verdict, normal, sat};

/// A clause of the clausal form: its hypothesis, how many `∀` stand
/// before its matrix, and the matrix.
#[derive(Clone, Copy, Debug)]
pub(super) struct Clause {
    /// The variables its prefix binds.
    variables: usize,
    /// The matrix under the prefix.
    matrix: NodeId,
}

/// Returns the clauses of the clausal form's hypotheses `left`.
pub(super) fn clauses_of(formulas: &Formulas, left: &[NodeId]) -> Vec<Clause> {
    left.iter()
        .map(|&root| {
            let (mut variables, mut matrix) = (0, root);
            while let Node::Forall(body) = formulas.node(matrix) {
                variables += 1;
                matrix = body;
            }
            Clause { variables, matrix }
        })
        .collect()
}

/// The terms the instances are made of: the constants and function
/// symbols of the clauses, and the closed terms made so far by depth.
#[derive(Clone, Debug)]
pub(super) struct Universe {
    /// The function symbols with arguments, with their arities.
    pub(super) functions: Vec<(Symbol, usize)>,
    /// The closed terms by depth: the constants first.
    pub(super) levels: Vec<Vec<FoTermId>>,
}

impl Universe {
    /// Returns the universe of `clauses`: their constants, or a new one
    /// where they have none.
    pub(super) fn of(formulas: &mut Formulas, clauses: &[Clause]) -> Result<Self, Error> {
        let tables = formulas.first_order();
        let mut constants: Vec<FoTermId> = Vec::new();
        let mut functions: Vec<(Symbol, usize)> = Vec::new();
        // In the order the walk meets them, which numbers the universe's
        // levels and so the instances and the solver's atoms.
        let matrices = clauses.iter().map(|c| c.matrix);
        for t in formulas.terms_in(matrices, std::iter::empty()) {
            if let FoTerm::App(f, args) = tables.terms[t.index()] {
                let list = tables.list(args);
                if list.is_empty() {
                    constants.push(t);
                } else if !functions.iter().any(|&(g, _)| g == f) {
                    functions.push((f, list.len()));
                }
            }
        }
        constants.sort_unstable();
        functions.sort_unstable();
        // A clause with variables needs a term; a propositional sequent's
        // arena stays without tables.
        if constants.is_empty() && clauses.iter().any(|c| c.variables > 0) {
            constants.push(fresh_constant(formulas)?);
        }
        Ok(Self {
            functions,
            levels: vec![constants],
        })
    }

    /// Makes the terms of depth `depth`, each function applied to terms
    /// shallower whose deepest is one less, within `limit` terms in all.
    fn deepen(
        &mut self,
        formulas: &mut Formulas,
        limit: Option<u64>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let below: Vec<FoTermId> = self.levels.iter().flatten().copied().collect();
        let last: HashSet<FoTermId> = self.levels.last().into_iter().flatten().copied().collect();
        let mut made = Vec::new();
        for &(f, arity) in &self.functions {
            // Every tuple over `below`, in the order of a counter.
            let mut at = vec![0usize; arity];
            loop {
                let args: Vec<FoTermId> = at.iter().map(|&k| below[k]).collect();
                if args.iter().any(|a| last.contains(a)) {
                    let list = formulas.interner().list(&args);
                    made.push(formulas.interner().term(FoTerm::App(f, list)));
                    let all = below.len() + made.len();
                    asked(stop, all as u64)?;
                    if let Some(limit) = limit
                        && all as u64 > limit
                    {
                        return Err(Error::Refused(Refusal::Terms {
                            terms: all as u64,
                            limit,
                        }));
                    }
                }
                let Some(k) = (0..arity).rev().find(|&k| at[k] + 1 < below.len()) else {
                    break;
                };
                at[k] += 1;
                at[k + 1..].fill(0);
            }
        }
        self.levels.push(made);
        Ok(())
    }
}

/// Asks the stop once in 1 024 calls, `count` the things made so far, and
/// refuses when it fires.
fn asked(stop: &mut dyn FnMut(Progress) -> bool, count: u64) -> Result<(), Error> {
    if count.is_multiple_of(1024) && stop(Progress::new(Phase::Search, 1024, count)) {
        return Err(Error::Refused(Refusal::stopped(Phase::Search)));
    }
    Ok(())
}

/// Returns a constant of a new name, `c1` or with `_` after `c` until no
/// name of the arena is `c` and digits.
pub(super) fn fresh_constant(formulas: &mut Formulas) -> Result<FoTermId, Error> {
    let name = format!("{}1", normal::fresh_prefix(formulas, "c"));
    let symbol = formulas
        .interner()
        .symbol(&name, 0, false)
        .map_err(|fault| Error::Symbol { name, fault })?;
    let list = formulas.interner().list(&[]);
    Ok(formulas.interner().term(FoTerm::App(symbol, list)))
}

/// The search over the levels of the universe.
pub(super) struct Search<'l> {
    /// The clausal form's arena, which the instances extend.
    pub(super) formulas: Formulas,
    /// The clauses.
    pub(super) clauses: Vec<Clause>,
    /// The universe so far.
    pub(super) universe: Universe,
    /// The bounds.
    pub(super) limits: &'l Limits,
    /// What the search cost so far.
    pub(super) statistics: Statistics,
    /// The arena's size when its growth was last charged, with the bytes
    /// charged for it.
    pub(super) charged: ArenaSize,
    /// The bytes the last level's instances hold beside the arena.
    pub(super) held: u64,
}

impl Search<'_> {
    /// Returns the bounds a later phase works within: the memory bound
    /// less what the instances and the arena hold, which stay alive while
    /// the solver and the read-back run.
    pub(super) fn rest(&self) -> Limits {
        let used = self.held.saturating_add(self.charged.bytes);
        (*self.limits).with_memory_bytes(self.limits.memory_bytes.map(|m| m.saturating_sub(used)))
    }
}

impl Search<'_> {
    /// Returns every instance of every clause over the terms up to depth
    /// `depth`, within `limits.occurrences` atoms in all.
    pub(super) fn instances(
        &mut self,
        depth: u32,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Vec<Instance>, Error> {
        while self.universe.levels.len() <= depth as usize {
            self.universe
                .deepen(&mut self.formulas, self.limits.terms, &mut *stop)?;
        }
        let terms: Vec<FoTermId> = self.universe.levels.iter().flatten().copied().collect();
        self.statistics.terms = terms.len() as u64;
        let mut instances = Vec::new();
        let mut size = 0u64;
        // What the instances hold, beside the arena they grow: an
        // instance's entry and its terms.
        let mut held = 0u64;
        for (k, clause) in self.clauses.iter().enumerate() {
            let mut at = vec![0usize; clause.variables];
            loop {
                let chosen: Vec<FoTermId> = at.iter().map(|&i| terms[i]).collect();
                // The frame is the innermost binder's first.
                let frame: Vec<FoTermId> = chosen.iter().rev().copied().collect();
                let formula = self.formulas.instantiate_frame(clause.matrix, &frame)?;
                size = size.saturating_add(1);
                asked(stop, size)?;
                if let Some(limit) = self.limits.occurrences
                    && size > limit
                {
                    return Err(Error::Refused(Refusal::Occurrences {
                        occurrences: size,
                        limit,
                    }));
                }
                held = held.saturating_add(
                    (size_of::<Instance>() + 2 * chosen.len() * size_of::<FoTermId>()) as u64,
                );
                self.charged = ArenaSize::of(&self.formulas, &self.charged);
                if let Some(limit) = self.limits.memory_bytes
                    && held.saturating_add(self.charged.bytes) > limit
                {
                    return Err(Error::Refused(Refusal::Memory {
                        phase: Phase::Search,
                        limit_bytes: limit,
                        needed_bytes: None,
                    }));
                }
                instances.push(Instance {
                    clause: k,
                    terms: chosen,
                    formula,
                });
                let Some(i) = (0..clause.variables)
                    .rev()
                    .find(|&i| at[i] + 1 < terms.len())
                else {
                    break;
                };
                at[i] += 1;
                at[i + 1..].fill(0);
            }
        }
        self.statistics.instances = instances.len() as u64;
        self.held = held;
        Ok(instances)
    }

    /// Decides `instances` by the SAT solver, their ground atoms read as
    /// propositional atoms.
    pub(super) fn decide(
        &mut self,
        instances: &[Instance],
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<sat::Outcome, Error> {
        let sequent = propositional(&self.formulas, instances)?;
        self.statistics.calls += 1;
        // The solver's share is the rest less the copy it reads, which
        // lives while it runs.
        let copy = ArenaSize::of(sequent.formulas(), &ArenaSize::default()).bytes;
        let rest = self.rest();
        let share = rest.with_memory_bytes(rest.memory_bytes.map(|m| m.saturating_sub(copy)));
        let given = self.limits.memory_bytes;
        // A refusal names the bound the caller gave, not this share.
        match sat::decide(&sequent, Logic::Classical, &share, &mut *stop) {
            Ok(mut outcome) => {
                if let (Verdict::Unknown(Reason::MemoryLimit { limit_bytes }), Some(given)) =
                    (&mut outcome.verdict, given)
                {
                    *limit_bytes = given;
                }
                Ok(outcome)
            }
            Err(Error::Refused(Refusal::Memory {
                phase,
                needed_bytes,
                limit_bytes,
            })) => Err(Error::Refused(Refusal::Memory {
                phase,
                needed_bytes,
                limit_bytes: given.unwrap_or(limit_bytes),
            })),
            Err(error) => Err(error),
        }
    }

    /// Returns the instances the solver still refutes when every other is
    /// dropped that it refutes without, one at a time, with the outcome
    /// on those; a set larger than [`MOST_CUT`] is kept whole.
    pub(super) fn cut(
        &mut self,
        mut instances: Vec<Instance>,
        mut decided: sat::Outcome,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(Vec<Instance>, sat::Outcome), Error> {
        if instances.len() > MOST_CUT {
            return Ok((instances, decided));
        }
        let mut k = 0;
        while k < instances.len() {
            let mut fewer = instances.clone();
            fewer.remove(k);
            let tried = match self.decide(&fewer, &mut *stop) {
                Ok(tried) => tried,
                // A bound ends the cut, not the search.
                Err(error) => {
                    crate::ordinary::decide::ended(error)?;
                    break;
                }
            };
            if matches!(tried.verdict, Verdict::Valid(_)) {
                instances = fewer;
                decided = tried;
            } else {
                k += 1;
            }
        }
        Ok((instances, decided))
    }
}

/// Returns `instances` as a propositional sequent `C₁σ₁, …, Cₙσₙ ⊢`, each
/// ground atom a propositional atom of its own.
fn propositional(formulas: &Formulas, instances: &[Instance]) -> Result<Sequent, Error> {
    let mut out = Formulas::default();
    let mut made: HashMap<NodeId, NodeId> = HashMap::default();
    let mut left = Vec::with_capacity(instances.len());
    for instance in instances {
        // Operands first, each node once.
        let mut stack = vec![(instance.formula, false)];
        while let Some((n, ready)) = stack.pop() {
            if made.contains_key(&n) {
                continue;
            }
            let node = formulas.node(n);
            if !ready {
                stack.push((n, true));
                let (a, b) = node.operands();
                stack.extend([a, b].into_iter().flatten().map(|k| (k, false)));
                continue;
            }
            let of = |k: NodeId| made[&k];
            let image = match node {
                // The solver reads atoms alone: a number names each.
                Node::Atom(_) => out.atom(&format!("g{}", out.atom_names().len() + 1))?,
                Node::True => out.add(Node::True)?,
                Node::False => out.add(Node::False)?,
                Node::Not(a) => out.add(Node::Not(of(a)))?,
                Node::And(a, b) => out.add(Node::And(of(a), of(b)))?,
                Node::Or(a, b) => out.add(Node::Or(of(a), of(b)))?,
                Node::Implies(a, b) => out.add(Node::Implies(of(a), of(b)))?,
                Node::Iff(a, b) => out.add(Node::Iff(of(a), of(b)))?,
                Node::Forall(_) | Node::Exists(_) => unreachable!("an instance is ground"),
            };
            made.insert(n, image);
        }
        left.push(made[&instance.formula]);
    }
    Sequent::new(out, left, Vec::new())
}
