// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The views of a proof in progress: its derivation with the open goals,
//! the goal each drawn inference stands for, and the finished derivation
//! checked.

use super::{Interactive, terms_full};
use crate::Error;
use crate::first_order::{Eigen, FoTerm};
use crate::limits::{Limits, Progress};
use crate::linear::proofs::{GoalId, InfId, Structural, ViewOptions};
use crate::ordinary::{Derivation, Formulas, Inference};

/// Returns the bytes the inferences are estimated at by the measure of
/// [`Size::bytes`](crate::linear::proofs::Size::bytes): per inference [`Size::BYTES_PER_INFERENCE`](crate::linear::proofs::Size::BYTES_PER_INFERENCE) and
/// [`Size::BYTES_PER_CHARACTER`](crate::linear::proofs::Size::BYTES_PER_CHARACTER) for every character of its sequent
/// written, each formula with two for its separator, every binary
/// subformula in brackets; saturating.
pub(super) fn estimate(formulas: &Formulas, inferences: &[Inference]) -> u64 {
    let widths = crate::ordinary::derivation::widths(formulas);
    inferences.iter().fold(0u64, |sum, inference| {
        let bytes = crate::ordinary::derivation::inference_bytes(
            &widths,
            &inference.left,
            &inference.right,
        );
        sum.saturating_add(bytes)
    })
}

impl Interactive {
    /// Returns the derivation so far with the open goals as leaves of
    /// [`Rule::Open`](crate::ordinary::Rule::Open), under the default view and limits.
    ///
    /// # Errors
    ///
    /// As [`derivation_within`](Self::derivation_within) does.
    pub fn derivation(&self) -> Result<Derivation, Error> {
        self.derivation_within(&ViewOptions::default(), &Limits::default(), |_| false)
    }

    /// Returns the derivation so far, with the open goals as leaves of
    /// [`Rule::Open`](crate::ordinary::Rule::Open) and the structural rules drawn as `view.structural`
    /// says. Its inferences are numbered premises before conclusions, so
    /// their ids are not this state's; [`derivation_ids`](Self::derivation_ids)
    /// maps them. The derivation is refused past `limits.derivation_bytes`
    /// by its estimate, before anything is built, and when `stop` fires.
    ///
    /// # Errors
    ///
    /// [`Refusal::Output`](crate::Refusal::Output) past `limits.derivation_bytes`, and
    /// [`Refusal::Stopped`](crate::Refusal::Stopped) when `stop` fired.
    pub fn derivation_within(
        &self,
        view: &ViewOptions,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Derivation, Error> {
        let estimate = estimate(&self.formulas, &self.inferences);
        crate::goal_tree::may_build(estimate, limits, &mut stop)?;
        let order = self.postorder();
        let new_id = crate::goal_tree::places(&order, self.inferences.len());
        let inferences = order
            .iter()
            .map(|id| {
                let inference = &self.inferences[id.index()];
                Inference {
                    premises: inference
                        .premises
                        .iter()
                        .map(|p| new_id[p.index()])
                        .collect(),
                    ..inference.clone()
                }
            })
            .collect();
        let (left, right) = self.sequent();
        let (formulas, inferences) = self.finishing(inferences)?;
        let derivation = Derivation::from_parts(
            self.logic,
            formulas,
            (left.to_vec(), right.to_vec()),
            inferences,
        );
        Ok(match view.structural {
            Structural::Explicit => derivation,
            Structural::Admissible => derivation.admissible().0,
        })
    }

    /// Returns the formulas and `inferences`, the state's in another
    /// order, where no goal is open: with every open witness of the arena
    /// made an eigenvariable that no inference holds, numbered past every
    /// one the arena holds. No `∀R` or `∃L` introduces it, so the checker
    /// takes it as a constant, the domain's element a witness no axiom
    /// bound stands for; a bound witness the inferences no longer hold,
    /// and one an undone step left in the arena, go the same way, so the
    /// derivation's arena holds no open witness, which its reader refuses.
    /// The terms are changed in place, so every formula keeps its id.
    /// While a goal is open the formulas stay as they are, the witnesses
    /// printed `$1`, `$2`, ….
    fn finishing(&self, inferences: Vec<Inference>) -> Result<(Formulas, Vec<Inference>), Error> {
        let mut formulas = self.formulas.clone();
        let terms = &formulas.first_order().terms;
        if !self.is_complete() || !terms.iter().any(|t| matches!(t, FoTerm::Meta(_))) {
            return Ok((formulas, inferences));
        }
        let base = terms
            .iter()
            .filter_map(|t| match *t {
                FoTerm::Eigen(e) => Some(e.get()),
                FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
            })
            .max()
            .map_or(Some(0), |e| e.checked_add(1))
            .ok_or_else(terms_full)?;
        // A fresh eigenvariable is no term of the arena, so no two terms
        // become equal.
        for term in &mut formulas.interner().tables.terms {
            if let FoTerm::Meta(k) = *term {
                let number = base.checked_add(k).ok_or_else(terms_full)?;
                *term = FoTerm::Eigen(Eigen::new(number));
            }
        }
        formulas.rekey_atoms();
        Ok((formulas, inferences))
    }

    /// Returns, for every inference of the derivation that
    /// [`derivation_within`](Self::derivation_within) gives under the
    /// structural rules drawn as `structural` says, the id of the
    /// inference of this state it shows. It turns an inference of a
    /// drawing into the goal that [`apply`](Self::apply) and
    /// [`close`](Self::close) take.
    pub fn derivation_ids(&self, structural: Structural) -> Vec<GoalId> {
        let order = self.postorder();
        let goal = |id: InfId| GoalId::new(id.get());
        match structural {
            Structural::Explicit => order.into_iter().map(goal).collect(),
            Structural::Admissible => {
                // The shape alone decides what is shown: the derivation's
                // inferences without their sequents.
                let new_id = crate::goal_tree::places(&order, self.inferences.len());
                // At first order the formulas decide too: a weakening that
                // brings an eigenvariable into its rule's conclusion stays.
                let first_order = self.formulas.is_first_order();
                let shape: Vec<Inference> = order
                    .iter()
                    .map(|id| {
                        let inference = &self.inferences[id.index()];
                        let premises = inference
                            .premises
                            .iter()
                            .map(|p| new_id[p.index()])
                            .collect();
                        if first_order {
                            Inference {
                                premises,
                                ..inference.clone()
                            }
                        } else {
                            Inference {
                                term: None,
                                left: Vec::new(),
                                right: Vec::new(),
                                rule: inference.rule,
                                principal: None,
                                premises,
                            }
                        }
                    })
                    .collect();
                let formulas = if first_order {
                    self.formulas.clone()
                } else {
                    Formulas::default()
                };
                let skeleton =
                    Derivation::from_parts(self.logic, formulas, (Vec::new(), Vec::new()), shape);
                let (_, origin) = skeleton.admissible();
                origin.into_iter().map(|i| goal(order[i.index()])).collect()
            }
        }
    }

    /// Returns the arena's inferences in postorder, premises before
    /// conclusions, the order of [`derivation`](Self::derivation).
    fn postorder(&self) -> Vec<InfId> {
        crate::goal_tree::postorder(self.inferences.len(), |i| {
            &self.inferences[i.index()].premises[..]
        })
    }

    /// Returns the derivation of a finished proof, every structural rule
    /// explicit, checked by [`Derivation::check`] within `limits` and
    /// until `stop` fires. The check never refuses what this state built;
    /// it runs so that the checker, not this module, vouches for the
    /// result.
    ///
    /// # Errors
    ///
    /// [`Error::OpenGoals`] while goals are open, the refusals of
    /// [`derivation_within`](Self::derivation_within), and the errors of
    /// [`Derivation::check`], its rejection among them.
    pub fn finished(
        &self,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Derivation, Error> {
        let open = self.goals().count();
        if open > 0 {
            return Err(Error::OpenGoals { count: open });
        }
        let view = ViewOptions::default().with_structural(Structural::Explicit);
        let derivation = self.derivation_within(&view, limits, &mut stop)?;
        derivation
            .check(limits, stop)
            .map_err(|e| derivation.own_fault(e))?;
        Ok(derivation)
    }
}
