// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The memo and the loop check of a first-order search. The memo is the
//! table of sequents already decided; the loop check prunes a sequent
//! equal to one of its ancestors on the branch. Both key a stable
//! sequent (one with no rule left that applies without a choice) by its
//! instance: its members with their terms. `framed.rs` writes the keys.
//!
//! Only failures are recorded. A proof's members name spans and
//! metavariables (the variables a later unification binds) that an undo
//! of a rule around it takes back, so a proof kept for later would name
//! what is gone.
//!
//! A failure is a fact about its sequent when three things hold:
//! - its search ran with no answer excluded, since an excluded answer
//!   may be all that failed it;
//! - it rested on no prune of an ancestor (the propositional engine
//!   lets such a failure wait; here it is dropped);
//! - it bound only what the sequent's own terms reach: its metavariables
//!   and the ones its search made.
//!
//! A complete failure then holds for every sequent with its canonical
//! key, which names the variables up to renaming. A failure cut by a
//! bound holds only for the instance itself, at its own budget. The
//! propositional engine keeps such failures under the sequent's own key
//! too, so that the deepening of the copy bound ends.

use super::memo::{Entry, Failure, Key, Zones};
use super::zone::{Linear as _, Zone};
use super::{Cuts, Found, NO_DEPENDENCY, Run, Unrestricted};
use crate::linear::occurrences::OccId;
use crate::linear::search::Reason;

impl<Z: Zone> Run<'_, Z> {
    /// Decides a first-order stable sequent with the memo and the loop
    /// check. Returns `None` where the sequent has no keys.
    ///
    /// A complete failure recorded under its canonical key answers it, and
    /// so does a failure cut by a bound under its own key. A repeat of an
    /// ancestor is pruned. Otherwise the sequent is decided, and a failure
    /// is recorded as the module doc says.
    pub(super) fn prove_keyed(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        budget: u32,
    ) -> Option<Result<(Found, bool), Reason>> {
        let mut own = self.take_words();
        let mut canonical = self.take_words();
        let forest = self.problem.forest;
        let keyed = self
            .zone
            .keys(forest, &theta.set, gamma, &mut own, &mut canonical);
        // The keys live while the sequent's subtree is searched, one pair
        // per stable sequent on the branch. So they are charged to the
        // account before the search, and writing them counts as work.
        own.settle(&mut self.scratch);
        canonical.settle(&mut self.scratch);
        let decided = match self.poll_terms() {
            Err(reason) => Some(Err(reason)),
            Ok(()) => keyed.then(|| self.prove_by_keys(theta, gamma, (&own, &canonical), budget)),
        };
        self.give_words(own);
        self.give_words(canonical);
        decided
    }

    /// [`Self::prove_keyed`] with the keys written.
    fn prove_by_keys(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        (own, canonical): (&[OccId], &[OccId]),
        budget: u32,
    ) -> Result<(Found, bool), Reason> {
        // The unrestricted zone is written among the members' words, so
        // its id is unused.
        let own = Zones {
            theta: 0,
            members: own,
        };
        let canonical = Zones {
            theta: 0,
            members: canonical,
        };
        let (own_hash, canonical_hash) = (own.hash(), canonical.hash());
        if let Some(failure) = self.memo.refuted(canonical, canonical_hash) {
            return Ok((Found::NOTHING, failure == Failure::Hereditary));
        }
        // Only failures cut by a bound lie under the own key.
        let cut = self.memo.get(own, own_hash, budget).is_some();
        if self.problem.rules.stack
            && let Some(depth) = self.repeated(own, own_hash)
        {
            // A depth of the branch fits a `u32`: the recursion limit
            // bounds it.
            return Ok((Found::failed(Cuts::repeat(depth as u32)), false));
        }
        if cut {
            return Ok((Found::failed(Cuts::BUDGET), false));
        }
        if self.problem.rules.stack {
            self.push_stack(own, own_hash);
        }
        let nodes = self.nodes.mark();
        let zone = self.zone.mark();
        let mut members = self.take_list();
        gamma.list_into(&mut members);
        let result = self.decide(theta, gamma, &members, budget);
        self.give_list(members);
        let own_depth = self.pop_stack();
        let (found, hereditary) = result?;
        let Found::Failed(cuts) = found else {
            return Ok((found, false));
        };
        self.nodes.release(nodes);
        self.zone.undo(zone);
        // A prune that rested on this very sequent is settled here.
        let dependency = settled(cuts.dependency, own_depth);
        if dependency == NO_DEPENDENCY && !self.zone.excluding() {
            if cuts.exhausted {
                self.remember(own, own_hash, Entry::Failed(Failure::Exhausted(budget)))?;
            } else {
                self.remember(
                    canonical,
                    canonical_hash,
                    Entry::Failed(failure_of(hereditary)),
                )?;
            }
        }
        let cuts = Cuts {
            exhausted: cuts.exhausted,
            dependency,
        };
        Ok((Found::failed(cuts), hereditary))
    }

    /// Pushes a stable sequent on the branch stack, into the entry's own
    /// buffer, which grows to the longest linear zone it held.
    pub(super) fn push_stack(&mut self, key: Zones<'_>, hash: u64) {
        if self.stack_len == self.stack.len() {
            self.scratch
                .charge(size_of::<Key>() + size_of::<u64>() + size_of::<usize>());
            self.stack.push(Key::default());
            self.hashes.push(0);
            self.marks.push(0);
        }
        let pushed = &mut self.stack[self.stack_len];
        pushed.assign(key);
        pushed.members.settle(&mut self.scratch);
        self.hashes[self.stack_len] = hash;
        self.marks[self.stack_len] = self.waiting.len();
        self.stack_len += 1;
    }

    /// Pops the stable sequent [`push_stack`](Self::push_stack) pushed
    /// last, where the rules keep a stack, and returns its depth, which a
    /// failure that rests on it names; `NO_DEPENDENCY` without a stack.
    pub(super) fn pop_stack(&mut self) -> u32 {
        if self.problem.rules.stack {
            self.stack_len -= 1;
            (self.above() + self.stack_len) as u32
        } else {
            NO_DEPENDENCY
        }
    }
}

/// Returns what a failure decided below a stable sequent at `own_depth`
/// rests on: its dependency, or none where that is the sequent itself or
/// one pushed after it, which the sequent's failure settles.
pub(super) const fn settled(dependency: u32, own_depth: u32) -> u32 {
    if dependency >= own_depth {
        NO_DEPENDENCY
    } else {
        dependency
    }
}

/// Returns the failure of a sequent decided without the budget's cut:
/// hereditary where `hereditary` says so, else complete.
pub(super) const fn failure_of(hereditary: bool) -> Failure {
    if hereditary {
        Failure::Hereditary
    } else {
        Failure::Complete
    }
}
