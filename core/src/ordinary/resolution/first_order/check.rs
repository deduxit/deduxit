// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The check of a refutation: every step made again from its parents,
//! ending at the empty clause. It reads the clauses' inference function,
//! which is trusted with it.

use super::{Clause, Clauses, Refutation};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::derivation::ArenaSize;

impl Refutation {
    /// Checks the refutation step by step: from the given clauses alone,
    /// each inference's unifier is computed again and its clause made again,
    /// both equal to what it records, each parent made before it, and the
    /// last clause is `□`. An atom is held to the bound on terms the
    /// clauses were made under; there is no bound on work or memory, which
    /// [`check_within`](Self::check_within) takes.
    ///
    /// # Errors
    ///
    /// [`Error::ClaimFails`] for a step that fails, saying which;
    /// [`Refusal::Terms`] for an atom past the clauses' bound on terms;
    /// [`Refusal::Stopped`] when `stop` fires, asked once per step.
    pub fn check(&self, stop: impl FnMut(Progress) -> bool) -> Result<(), Error> {
        let limits = Limits::default()
            .with_terms(self.clauses.most_terms)
            .with_memory_bytes(None)
            .with_work(None);
        self.check_within(&limits, stop)
    }

    /// Checks the refutation as [`check`](Self::check) does, within
    /// `limits`: `limits.terms` bounds an atom, `limits.work` the steps
    /// and the literals they make, and `limits.memory_bytes` the copy of
    /// the clauses' arena the steps are made in again and what they add
    /// to it.
    ///
    /// # Errors
    ///
    /// [`Error::ClaimFails`] for a step that fails, saying which;
    /// [`Refusal::Terms`], [`Refusal::Work`] and [`Refusal::Memory`] past
    /// those bounds; [`Refusal::Stopped`] when `stop` fires, asked once
    /// per step.
    pub fn check_within(
        &self,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let fails = |reason: String| Error::ClaimFails {
            claim: "the first-order refutation".to_owned(),
            reason,
        };
        let memory = limits.memory_bytes.unwrap_or(u64::MAX);
        let most_work = limits.work.unwrap_or(u64::MAX);
        // The copy of the arena, charged before it is made.
        let mut held = ArenaSize::of(&self.clauses.formulas, &ArenaSize::default());
        let within_memory = |held: &ArenaSize| {
            if held.bytes > memory {
                return Err(Error::Refused(Refusal::Memory {
                    phase: Phase::Check,
                    limit_bytes: memory,
                    needed_bytes: Some(held.bytes),
                }));
            }
            Ok(())
        };
        within_memory(&held)?;
        let mut again = Clauses {
            formulas: self.clauses.formulas.clone(),
            items: self.clauses.items[..self.clauses.given].to_vec(),
            given: self.clauses.given,
            most_terms: limits.terms,
            sizes: HashMap::default(),
        };
        let mut work = 0u64;
        for (n, inference) in self.inferences.iter().enumerate() {
            if stop(Progress::new(Phase::Check, 1, n as u64 + 1)) {
                return Err(Error::Refused(Refusal::stopped(Phase::Check)));
            }
            let made = again
                .made(inference.step)
                .map_err(|e| {
                    if let Error::Refused(_) = e {
                        e
                    } else {
                        fails(format!("step {}: {e}", n + 1))
                    }
                })?
                .map_err(|why| fails(format!("step {}: {why}", n + 1)))?;
            // A step, and each literal it makes.
            work = work.saturating_add(1 + made.clause.literals.len() as u64);
            if work > most_work {
                return Err(Error::Refused(Refusal::Work { limit: most_work }));
            }
            held = ArenaSize::of(&again.formulas, &held);
            within_memory(&held)?;
            if made.bindings != inference.bindings {
                return Err(fails(format!(
                    "step {}: its unifier is not the most general one",
                    n + 1
                )));
            }
            let recorded = self
                .clauses
                .items
                .get(inference.clause)
                .ok_or_else(|| fails(format!("step {}: no clause made", n + 1)))?;
            if inference.clause != again.items.len() || made.clause != *recorded {
                return Err(fails(format!(
                    "step {}: its clause is not the one the step makes",
                    n + 1
                )));
            }
            again.items.push(made.clause);
        }
        // A given `□` is a refutation of no step.
        let ends = if self.inferences.is_empty() {
            again.items.iter().any(Clause::is_empty)
        } else {
            again.items.last().is_some_and(Clause::is_empty)
        };
        if !ends {
            return Err(fails("it does not end at □".to_owned()));
        }
        Ok(())
    }
}
