// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The choice among the alternatives of a stable sequent, or of a `⊕` in
//! focus, when the sequent has quantifiers (the first-order zone).
//!
//! A stable sequent is one with no rule left that can be applied without
//! a choice. The choice logs which alternative it tries, so that the
//! search of a premise can later be resumed at it (`choices.rs`).

use super::choices::Site;
use super::zone::Zone;
use super::{Alternative, Cuts, Found, Run, Step, Unrestricted};

impl<Z: Zone> Run<'_, Z> {
    /// [`Self::choose_here`] in a first-order zone, logging its choice.
    pub(super) fn choose_framed<'m>(
        &mut self,
        theta: &Unrestricted<Z>,
        gamma: &Z::Linear,
        alternatives: impl Iterator<Item = Alternative<'m, Z>>,
        budget: u32,
    ) -> Step {
        let choice = self.zone.choices().enter(Site::Choose);
        let mut cuts = choice.cuts;
        let mut rest = None;
        let mut found = None;
        if choice.start != u32::MAX {
            for (i, alternative) in alternatives.enumerate() {
                // An alternative past the log's numbers is cut, never
                // refuted.
                let Some(i) = u32::try_from(i).ok().filter(|&i| i < u32::MAX) else {
                    cuts = cuts.and(Cuts::BUDGET);
                    break;
                };
                if i < choice.start {
                    continue;
                }
                self.zone.choices().take(&choice, i, cuts);
                match self.alternative(theta, gamma, alternative, &mut rest, budget)? {
                    Found::Failed(failed) => cuts = cuts.and(failed),
                    proved => {
                        found = Some(proved);
                        break;
                    }
                }
            }
        }
        if let Some(rest) = rest {
            self.give_context(rest);
        }
        Ok(found.unwrap_or_else(|| {
            self.zone.choices().fail(&choice, cuts);
            Found::failed(cuts)
        }))
    }
}
