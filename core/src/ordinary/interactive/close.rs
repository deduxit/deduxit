// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A goal closed or hinted by the search: decided as a sequent of its own,
//! and the derivation found grafted in its place or read for its first
//! steps.

use super::derivation::estimate;
use super::expand::premise;
use super::witness::{arena_terms, implications_from};
use super::{Closed, Hint, Interactive, Shape, Step, terms_full};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::HashMap;
use crate::limits::{Limits, Progress, Refusal};
use crate::linear::proofs::{GoalId, InfId};
use crate::linear::search;
#[cfg(feature = "sat")]
use crate::ordinary::Logic;
use crate::ordinary::derivation::{check_inference, same};
use crate::ordinary::transfer::{self, Parameters};
use crate::ordinary::{Derivation, Formulas, Inference, NodeId, Outcome, Rule, Sequent, Side};
use crate::ordinary::{Evidence, Translation, Verdict, decide_within};

/// What deciding a goal found, and how to carry its derivation back.
struct Decided {
    /// The outcome. At first order its derivation, its countermodels and
    /// their sequents are over an arena of the goal alone.
    outcome: Outcome,
    /// At first order, how a derivation over that arena is carried back.
    constants: Option<Carry>,
    /// Whether a derivation in the outcome is one of the goal: not where
    /// it holds the equality axioms the goal lacks.
    graftable: bool,
}

/// How a derivation over the arena of a goal alone is carried back into
/// the proof's arena.
pub(super) struct Carry {
    /// The constants of that arena that stand for the goal's
    /// eigenvariables and open witnesses, by name, each with the state's
    /// term.
    pub(super) constants: HashMap<String, FoTermId>,
    /// The names of that arena's symbols and atoms: any other name of the
    /// derivation's is its own, renamed apart from the proof's names.
    pub(super) names: crate::hash::HashSet<String>,
}

/// Gives a valid first-order outcome whose evidence is no derivation (the
/// clausal proof of grounding's instances) the derivation of LK that
/// Herbrand's theorem builds from ground instances of the sequent, which
/// [`herbrand`](crate::ordinary::herbrand::herbrand) checks; where it builds none
/// within `limits` and `stop`, the outcome stays as it is.
///
/// # Errors
///
/// [`Error::ReadBack`] where the derivation built fails its check, a
/// defect.
#[cfg(feature = "sat")]
pub(super) fn with_derivation(
    outcome: &mut Outcome,
    sequent: &Sequent,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<(), Error> {
    let Verdict::Valid(evidence) = &outcome.verdict else {
        return Ok(());
    };
    // Herbrand's derivation is one of LK, and grounding is valid in
    // classical logic alone.
    if evidence.derivation().is_ok()
        || !sequent.formulas.is_first_order()
        || outcome.logic != Logic::Classical
    {
        return Ok(());
    }
    match crate::ordinary::herbrand::herbrand(
        sequent,
        &crate::ordinary::herbrand::Options::default(),
        limits,
        stop,
    ) {
        Ok(found) => {
            if let Verdict::Valid(evidence @ Evidence::Derivation(_)) = found.verdict {
                outcome.verdict = Verdict::Valid(evidence);
            }
            Ok(())
        }
        Err(error @ Error::ReadBack { .. }) => Err(error),
        // A bound or a sequent outside its reach leaves the clausal proof.
        Err(_) => Ok(()),
    }
}

/// Returns whether `outcome` rests on an open witness taken as a
/// constant, one of `metas` by name: a countermodel, which shows only
/// that the witness as a constant gives no proof, and a derivation whose
/// quantifier rule takes a term holding one, which no proof in progress
/// names.
fn takes_open_witness(outcome: &Outcome, metas: &crate::hash::HashSet<&str>) -> bool {
    match &outcome.verdict {
        Verdict::NotValid => true,
        Verdict::Valid(Evidence::Derivation(found)) => {
            let tables = found.formulas().first_order();
            let mut seen = crate::hash::HashSet::default();
            let mut stack: Vec<FoTermId> =
                found.inferences().iter().filter_map(|i| i.term).collect();
            while let Some(t) = stack.pop() {
                if !seen.insert(t) {
                    continue;
                }
                if let FoTerm::App(symbol, args) = tables.terms[t.index()] {
                    if metas.contains(tables.symbols[symbol.index()].0.as_str()) {
                        return true;
                    }
                    stack.extend_from_slice(tables.list(args));
                }
            }
            false
        }
        Verdict::Valid(_) | Verdict::Unknown(_) => false,
    }
}

/// Fits the root `top` of a derivation grafted onto the goal `goal`: the
/// goal keeps the order of its sides, and the root's principal is found
/// there by its formula, the sides being the same multisets.
///
/// # Errors
///
/// [`Error::GoalMismatch`] where the sides are other multisets.
fn fit_root(top: &mut Inference, goal: &Inference) -> Result<(), Error> {
    if !same(&top.left, &goal.left) || !same(&top.right, &goal.right) {
        return Err(Error::GoalMismatch);
    }
    top.principal = top.principal.map(|(side, position)| {
        let id = top.side(side)[position];
        let list = goal.side(side);
        (side, list.iter().position(|&y| y == id).unwrap_or(position))
    });
    top.left.clone_from(&goal.left);
    top.right.clone_from(&goal.right);
    Ok(())
}

impl Interactive {
    /// Decides the open goal as a sequent of its own in the state's logic
    /// and returns the outcome, with the search options, within `limits`
    /// and until `stop` fires. A propositional goal goes through the
    /// embedding by `translation` (`None` for the logic's default). A
    /// first-order goal is decided as [`decide_within`] decides a sequent
    /// with no method given: a search for a proof, then one for a finite
    /// countermodel, the embedding by `translation` where one is given;
    /// an eigenvariable or an open witness of the goal is a fresh constant
    /// there. When the goal is valid, the derivation found is grafted onto
    /// it as one step, which [`undo`](Self::undo) takes back whole, each of
    /// its inferences checked first. A goal the search refutes or cannot
    /// decide stays open, and the outcome says why ([`Verdict::NotValid`],
    /// or [`Verdict::Unknown`] with its reason). A goal that holds an open
    /// witness is never refuted: a countermodel there shows only that
    /// the witness as a constant gives no proof, and the outcome is
    /// unknown ([`Reason::Incomplete`](crate::linear::search::Reason::Incomplete)).
    ///
    /// A derivation larger than `limits.derivation_bytes` by the size
    /// estimate is not grafted: the goal stays open, [`Closed::grafted`]
    /// says why, and the outcome keeps the derivation. A valid goal whose
    /// evidence is no derivation stays open too, and so does a goal with
    /// `=` that lacks some of the equality axioms: it is decided with them
    /// as hypotheses, so its derivation is not the goal's.
    ///
    /// # Errors
    ///
    /// [`Error::OrdinaryStep`] for a goal that is not open, the errors of
    /// [`decide_within`] (a translation that does not decide the logic, the
    /// refusal of a bound or the stop), the goal then still open,
    /// [`Error::GoalMismatch`] if the derivation found concludes a sequent
    /// other than the goal, and
    /// [`Error::ReadBack`] if one of its inferences fails the check, a
    /// defect.
    pub fn close(
        &mut self,
        goal: GoalId,
        translation: Option<Translation>,
        search: &search::Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Closed, Error> {
        let Decided {
            outcome,
            constants,
            graftable,
        } = self.decide(goal, translation, search, limits, stop)?;
        let grafted = match &outcome.verdict {
            Verdict::Valid(Evidence::Derivation(found)) if graftable => {
                match self.graft(goal, found, constants.as_ref(), limits) {
                    Ok(()) => Ok(()),
                    Err(Error::Refused(refusal)) => Err(refusal),
                    Err(error) => return Err(error),
                }
            }
            Verdict::Valid(_) | Verdict::NotValid | Verdict::Unknown(_) => Ok(()),
        };
        Ok(Closed { outcome, grafted })
    }

    /// Decides the open goal as [`close`](Self::close) does and returns
    /// the outcome with the first steps of the derivation found
    /// ([`Hint::steps`]), which [`apply`](Self::apply) takes in turn. The
    /// state does not change.
    ///
    /// # Errors
    ///
    /// As [`close`](Self::close) does.
    pub fn hint(
        &self,
        goal: GoalId,
        translation: Option<Translation>,
        search: &search::Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Hint, Error> {
        let Decided {
            outcome,
            constants,
            graftable,
        } = self.decide(goal, translation, search, limits, stop)?;
        self.open(goal)?;
        let steps = match &outcome.verdict {
            Verdict::Valid(Evidence::Derivation(found)) if graftable => {
                self.first_steps(goal, found, constants.as_ref())
            }
            Verdict::Valid(_) | Verdict::NotValid | Verdict::Unknown(_) => Vec::new(),
        };
        Ok(Hint { outcome, steps })
    }

    /// Decides the open goal as a sequent of its own, as
    /// [`close`](Self::close) says.
    fn decide(
        &self,
        goal: GoalId,
        translation: Option<Translation>,
        search: &search::Options,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Decided, Error> {
        let goal = self.open(goal)?;
        if !self.formulas.is_first_order() {
            let sequent = Sequent {
                formulas: self.formulas.clone(),
                left: goal.left.clone(),
                right: goal.right.clone(),
            };
            // The embedding's derivations have no cut, which no step takes.
            let options = crate::ordinary::Options::default()
                .with_logic(self.logic)
                .with_translation(translation)
                .with_method(Some(crate::ordinary::Method::Embedding));
            return Ok(Decided {
                outcome: decide_within(&sequent, &options, search, limits, stop)?,
                constants: None,
                graftable: true,
            });
        }
        // The goal alone, in an arena of its own: no eigenvariable or open
        // witness of the proof reaches the search, and none of another
        // branch can be a witness of the derivation found.
        let roots: Vec<NodeId> = goal.left.iter().chain(&goal.right).copied().collect();
        let mut formulas = Formulas::default();
        let copied = transfer::copy(
            &mut formulas,
            &self.formulas,
            Some(&roots),
            &Parameters::Constants,
        )?;
        let sequent = Sequent {
            left: goal.left.iter().map(|&n| copied.node(n)).collect(),
            right: goal.right.iter().map(|&n| copied.node(n)).collect(),
            formulas,
        };
        let mut options = crate::ordinary::Options::default()
            .with_logic(self.logic)
            .with_translation(translation);
        if !sequent.formulas.is_first_order() {
            options = options.with_method(Some(crate::ordinary::Method::Embedding));
        }
        // A goal with `=` is decided with the equality axioms; where it
        // lacks them, a derivation is of the goal with them, no derivation
        // of the goal.
        let graftable = !sequent.formulas.has_equality()
            || crate::ordinary::equality::with_axioms(&sequent, limits)?.is_none();
        let mut outcome = decide_within(&sequent, &options, search, limits, &mut stop)?;
        #[cfg(feature = "sat")]
        with_derivation(&mut outcome, &sequent, limits, &mut stop)?;
        if copied.metas {
            let metas: crate::hash::HashSet<&str> = copied
                .constants
                .iter()
                .filter(|&(_, &t)| {
                    matches!(
                        self.formulas.first_order().terms[t.index()],
                        FoTerm::Meta(_)
                    )
                })
                .map(|(name, _)| name.as_str())
                .collect();
            if takes_open_witness(&outcome, &metas) {
                outcome.verdict = Verdict::Unknown(crate::linear::search::Reason::Incomplete);
                outcome.countermodel = None;
                outcome.structure = None;
                outcome.kripke = None;
            }
        }
        Ok(Decided {
            outcome,
            constants: Some(Carry {
                constants: copied.constants,
                names: copied.names,
            }),
            graftable,
        })
    }

    /// Returns the id in `formulas` of every formula of `other`, adding to
    /// `formulas` those it lacks.
    fn ids_of(formulas: &mut Formulas, other: &Formulas) -> Result<Vec<NodeId>, Error> {
        let copied = crate::ordinary::transfer::copy(
            formulas,
            other,
            None,
            &crate::ordinary::transfer::Parameters::Constants,
        )?;
        Ok((0..other.len())
            .map(|k| copied.node(NodeId(k as u32)))
            .collect())
    }

    /// Grafts a derivation of the open goal onto it as one step, its root
    /// in the goal's place and the rest appended in reverse, so that every
    /// premise keeps a larger index than its conclusion; refused past
    /// `limits.derivation_bytes` by the size estimate's measure.
    pub(super) fn graft(
        &mut self,
        goal: GoalId,
        found: &Derivation,
        constants: Option<&Carry>,
        limits: &Limits,
    ) -> Result<(), Error> {
        let estimate = estimate(found.formulas(), found.inferences());
        if let Some(limit) = limits.derivation_bytes
            && estimate > limit
        {
            return Err(Error::Refused(Refusal::Output {
                what: "derivation",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            }));
        }
        if let Some(constants) = constants {
            return self.graft_first_order(goal, found, constants);
        }
        let ids = Self::ids_of(&mut self.formulas, found.formulas())?;
        let map = |list: &[NodeId]| list.iter().map(|k| ids[k.index()]).collect::<Vec<_>>();
        let at = InfId::new(goal.get());
        let renumber =
            crate::goal_tree::grafted(at, self.inferences.len(), found.inferences().len() - 1);
        let mut grafted: Vec<Inference> = found
            .inferences()
            .iter()
            .map(|inference| Inference {
                term: None,
                left: map(&inference.left),
                right: map(&inference.right),
                rule: inference.rule,
                principal: inference.principal,
                premises: inference.premises.iter().map(|&p| renumber(p)).collect(),
            })
            .collect();
        let mut top = grafted.pop().expect("a derivation has a root");
        // The goal keeps the order of its sides; the root's principal is
        // found there by its formula, the sides being the same multisets.
        fit_root(&mut top, &self.inferences[at.index()])?;
        self.inferences[at.index()] = top;
        self.inferences.extend(grafted.into_iter().rev());
        self.history.push(at);
        Ok(())
    }

    /// Grafts a derivation of the first-order goal, over an arena of its
    /// own whose constants `constants` stand for the goal's parameters, as
    /// [`graft`](Self::graft) does, refused past the bound on terms the
    /// state was started under: its formulas and terms copied into the
    /// state's arena, the parameters back in their constants' places and
    /// its eigenvariables numbered past every one the proof holds, so that
    /// none meets a parameter or a later step's. Every inference is checked
    /// in the state's arena before the state changes.
    fn graft_first_order(
        &mut self,
        goal: GoalId,
        found: &Derivation,
        constants: &Carry,
    ) -> Result<(), Error> {
        let mut formulas = self.formulas.clone();
        let copied = self.carried(&mut formulas, found, constants)?;
        // The search held its own arena to the bound, which lacks the
        // atoms of the proof's other branches: a document of the proof
        // with the graft would be refused when read.
        let terms = arena_terms(formulas.first_order());
        if terms > self.terms {
            return Err(Error::Refused(Refusal::Terms {
                terms,
                limit: self.terms,
            }));
        }
        let map = |list: &[NodeId]| list.iter().map(|&k| copied.node(k)).collect::<Vec<_>>();
        // The root first, then the rest in reverse, so that every premise
        // comes after its conclusion; indices local to the step, as
        // `record` takes them.
        let n = found.inferences().len();
        // Fewer inferences than a `u32` counts.
        let local = |p: InfId| InfId::new((n - 1 - p.index()) as u32);
        let mut made: Vec<Inference> = found
            .inferences()
            .iter()
            .rev()
            .map(|inference| Inference {
                term: inference
                    .term
                    .map(|t| copied.terms[t.index()].expect("a term of the derivation")),
                left: map(&inference.left),
                right: map(&inference.right),
                rule: inference.rule,
                principal: inference.principal,
                premises: inference.premises.iter().map(|&p| local(p)).collect(),
            })
            .collect();
        fit_root(&mut made[0], &self.inferences[goal.index()])?;
        for inference in &made {
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|p| &made[p.index()])
                .collect();
            check_inference(&formulas, self.logic, inference, &premises).map_err(|reason| {
                Error::ReadBack {
                    calculus: self.logic.calculus(),
                    reason: format!("the derivation found, carried into the proof: {reason}"),
                }
            })?;
        }
        self.formulas = formulas;
        self.record(goal, made);
        Ok(())
    }

    /// Copies the formulas and terms of `found`, a derivation over an arena
    /// whose constants `constants` stand for this state's parameters, into
    /// `formulas`, an arena that extends the state's: each such constant
    /// back to its parameter, each eigenvariable numbered past every one
    /// the proof holds, the implications of every equivalence added.
    fn carried(
        &self,
        formulas: &mut Formulas,
        found: &Derivation,
        constants: &Carry,
    ) -> Result<transfer::Copied, Error> {
        let offset = self.next_eigenvariable().ok_or_else(terms_full)?;
        let before = formulas.len();
        let copied = transfer::copy(
            formulas,
            found.formulas(),
            None,
            &Parameters::Back {
                constants: &constants.constants,
                known: &constants.names,
                offset,
            },
        )?;
        implications_from(formulas, before)?;
        Ok(copied)
    }

    /// Returns the steps of a derivation of the open goal from its root
    /// up to and including the first rule that is not structural, each
    /// applied in a copy of the state to find the goal the next one acts
    /// on. Each turn goes one premise up the derivation, so there are no
    /// more turns than it has inferences; a step the copy refuses (a
    /// bound) ends the list there. A first-order derivation is over an
    /// arena of its own whose constants `constants` stand for the goal's
    /// parameters, carried into the copy's arena first.
    fn first_steps(
        &self,
        goal: GoalId,
        found: &Derivation,
        constants: Option<&Carry>,
    ) -> Vec<Step> {
        let mut state = self.clone();
        let carried = match constants {
            None => {
                Self::ids_of(&mut state.formulas, found.formulas()).map(|ids| (ids, Vec::new()))
            }
            Some(constants) => {
                let mut formulas = state.formulas.clone();
                self.carried(&mut formulas, found, constants).map(|copied| {
                    state.formulas = formulas;
                    let ids = copied
                        .nodes
                        .iter()
                        .map(|n| n.expect("every node"))
                        .collect();
                    (ids, copied.terms)
                })
            }
        };
        let Ok((ids, terms)) = carried else {
            return Vec::new();
        };
        // A witness is given only as a term of this state's arena, whose
        // ids the copy keeps: one the derivation brought in has no id here.
        let known = self.formulas.first_order().terms.len();
        let terms: Vec<Option<FoTermId>> = terms
            .into_iter()
            .map(|t| t.filter(|t| t.index() < known))
            .collect();
        let (mut goal, mut at) = (goal, found.root());
        let mut steps = Vec::new();
        while let Some(step) = state
            .open(goal)
            .ok()
            .and_then(|inference| state.step_at(goal, inference, found, at, &ids, &terms))
        {
            steps.push(step.clone());
            if !step.rule.is_structural() {
                break;
            }
            match state.apply(goal, &step).as_deref() {
                Ok(&[next]) => goal = next,
                _ => break,
            }
            at = found.inference(at).premises[0];
        }
        steps
    }

    /// Returns the rule of inference `at` of a derivation of the open goal
    /// `id` (whose conclusion is the goal `goal` as multisets) as a step
    /// on the goal, its formula found by what it is and, for two premises,
    /// the context as the derivation gave it to them; `ids` and `terms`
    /// are the state's ids of the derivation's nodes and terms. A witness
    /// of `∀L` or `∃R` is given where the goal holds it, else left open:
    /// one the search made up is no term of the proof.
    fn step_at(
        &self,
        id: GoalId,
        goal: &Inference,
        found: &Derivation,
        at: InfId,
        ids: &[NodeId],
        terms: &[Option<FoTermId>],
    ) -> Option<Step> {
        let map = |list: &[NodeId]| list.iter().map(|k| ids[k.index()]).collect::<Vec<_>>();
        let root = found.inference(at);
        let Some((side, at)) = root.principal else {
            // An axiom concludes exactly the goal `A ⊢ A`.
            return Some(Step::new(Side::Right, 0, root.rule));
        };
        let x = ids[root.side(side)[at].index()];
        let position = goal.side(side).iter().position(|&y| y == x)?;
        let mut step = Step::new(side, position, root.rule);
        if matches!(root.rule, Rule::ForallLeft | Rule::ExistsRight)
            && let Some(t) = root
                .term
                .and_then(|t| terms.get(t.index()).copied().flatten())
            && self.held(id, t).is_ok()
        {
            step = step.with_witness(t);
        }
        let Ok(Shape::Binary([first, _])) = self.shape(root.rule, side, position, x) else {
            return Some(step);
        };
        let given = found.inference(root.premises[0]);
        let (left, right) = (map(&given.left), map(&given.right));
        let shared = premise(goal, side, position, &first.0, &first.1, |_, _| true);
        if same(&shared.left, &left) && same(&shared.right, &right) {
            return Some(step);
        }
        // What the first premise holds beyond the rule's own formulas,
        // each matched to a position of the goal's context.
        let mut rest = (left, right);
        let (same_side, other_side) = match side {
            Side::Left => (&mut rest.0, &mut rest.1),
            Side::Right => (&mut rest.1, &mut rest.0),
        };
        for (list, added) in [(same_side, &first.0), (other_side, &first.1)] {
            for y in added {
                let i = list.iter().position(|z| z == y)?;
                list.swap_remove(i);
            }
        }
        let mut positions = (Vec::new(), Vec::new());
        for (s, held, out) in [
            (Side::Left, &mut rest.0, &mut positions.0),
            (Side::Right, &mut rest.1, &mut positions.1),
        ] {
            for (i, y) in goal.side(s).iter().enumerate() {
                if s == side && i == position {
                    continue;
                }
                if let Some(k) = held.iter().position(|z| z == y) {
                    held.swap_remove(k);
                    out.push(i);
                }
            }
        }
        Some(step.split(&positions.0, &positions.1))
    }
}
