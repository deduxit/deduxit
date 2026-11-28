// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Step-by-step proving in LK and LJ. A proof in progress is a derivation
//! of an ordinary sequent, built from its conclusion up by the rules of
//! [`Rule`], with the open goals as leaves. A step names a goal, a formula
//! in it by its side and position, and a rule. The step checks the rule
//! against the formula and the logic, checks every inference it makes by
//! the rules the derivation's checker applies, and returns the goals it
//! opens. The search can close a goal ([`decide_within`]): a propositional
//! goal through the embedding into linear logic, whose derivations have no
//! cut, a first-order one by the first-order methods, and a derivation
//! found is checked inference by inference before it enters. A finished
//! proof is a [`Derivation`] that [`Derivation::check`] validates, so this
//! module adds nothing to what must be trusted.
//!
//! Needs the cargo feature `interactive` (on by default).

mod close;
mod derivation;
mod error;
mod expand;
#[cfg(feature = "serialize")]
mod replay;
mod witness;

use super::bound;
use super::instance::Resolution;
use super::translate::Extra;
use super::{Formulas, Inference, Logic, NodeId, Outcome, Rule, Sequent, Side};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::limits::{Limits, Refusal};
use crate::linear::proofs::{GoalId, InfId};
pub use error::{BindingFault, StepError};
#[cfg(feature = "serialize")]
pub(crate) use replay::Parts;
use witness::{Witnesses, arena_terms, unify_axiom};

/// How a step of a rule with two premises gives them the goal's context,
/// the formulas the rule does not act on.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Context {
    /// Both premises take the whole context, as LJ's `∧R` does.
    #[default]
    Shared,
    /// The formulas at these positions go to the first premise, the rest
    /// to the second, as LJ's `→L` splits them.
    #[non_exhaustive]
    Split {
        /// The positions left of `⊢` going to the first premise.
        left: Vec<usize>,
        /// The positions right of `⊢` going to the first premise.
        right: Vec<usize>,
    },
}

/// A step of a proof in progress: the rule applied to the formula at a
/// position of a side of a goal, and for a rule with two premises how
/// they take the context.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// The side of the formula.
    pub side: Side,
    /// The position of the formula on its side.
    pub position: usize,
    /// The rule.
    pub rule: Rule,
    /// How two premises take the context.
    pub context: Context,
    /// The witness of `∀L` or `∃R`, a term of the proof's formulas
    /// ([`Interactive::witness`] reads one from text). `None` leaves the
    /// witness of `∀L` or `∃R` open, as `$1`, `$2`, …, for a later axiom to
    /// fill; every other rule takes none.
    pub witness: Option<FoTermId>,
}

impl Step {
    /// Returns the step of `rule` on the formula at `position` of `side`,
    /// its premises sharing the context.
    pub const fn new(side: Side, position: usize, rule: Rule) -> Self {
        Self {
            side,
            position,
            rule,
            context: Context::Shared,
            witness: None,
        }
    }

    /// Returns the step with the witness given, for `∀L` or `∃R`.
    #[must_use]
    pub fn with_witness(self, witness: FoTermId) -> Self {
        Self {
            witness: Some(witness),
            ..self
        }
    }

    /// Returns the step with the formulas at the positions given going to
    /// the first premise, left and right of `⊢`, and the rest to the
    /// second.
    #[must_use]
    pub fn split(self, left: &[usize], right: &[usize]) -> Self {
        Self {
            context: Context::Split {
                left: left.to_vec(),
                right: right.to_vec(),
            },
            ..self
        }
    }
}

/// A rule that can act on a formula of a goal, and what a step of it
/// needs beyond the formula.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Applicable {
    /// The rule.
    pub rule: Rule,
    /// What the step needs.
    pub needs: Needs,
}

/// What a step of a rule needs beyond the formula.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Needs {
    /// Nothing.
    Nothing,
    /// A context for its two premises: shared unless the step gives a
    /// split.
    Context,
    /// A witness term for the bound variable, or none to leave it open:
    /// `∀L` and `∃R`.
    Witness,
}

/// A rule that acts on a formula of a goal, and whether a step of it is
/// legal on the goal as it stands, as [`Interactive::candidates`] lists
/// them.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    /// The rule.
    pub rule: Rule,
    /// Whether a step of it is legal.
    pub status: Status,
}

/// Whether a step of a candidate rule is legal on the goal as it stands.
/// Whether the goals it opens can be closed is not judged.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// The step applies as it is.
    Applies,
    /// The step takes input the caller chooses, and the default is legal:
    /// a context for two premises (shared, or a split), a witness for
    /// `∀L` and `∃R` (or none, to leave it open).
    #[non_exhaustive]
    NeedsInput {
        /// What it takes.
        needs: Needs,
    },
    /// The step is not legal. The refusal is
    /// [`apply`](Interactive::apply)'s, of the step with the default
    /// input: a shared context, a witness left open.
    Refused(StepError),
}

/// What [`Interactive::close`] did: the outcome of deciding the goal, and
/// whether its derivation, if any, was grafted onto the goal.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Closed {
    /// The outcome of deciding the goal as a sequent of its own; a
    /// derivation in it is the goal's, kept when its graft was refused.
    pub outcome: Outcome,
    /// `Ok` when the derivation was grafted or there was none; the
    /// refusal of the graft by its size otherwise, the goal then still
    /// open.
    pub grafted: Result<(), Refusal>,
}

/// What [`Interactive::hint`] found: the outcome of deciding the goal,
/// and the first steps of the derivation found, as steps that
/// [`Interactive::apply`] takes in turn.
///
/// Needs the cargo feature `interactive` (on by default).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Hint {
    /// The outcome of deciding the goal as a sequent of its own.
    pub outcome: Outcome,
    /// The first steps of the derivation found: the weakenings and
    /// contractions at its root, then the first rule that is neither, which
    /// ends the list. The first step acts on the goal asked about, each
    /// later one on the goal the step before it opened. Each step names its
    /// formula and, for two premises, how they take the context. Empty
    /// unless the goal is valid. The steps come from one derivation so that
    /// following them makes progress: a contraction hinted alone could be
    /// answered, at the next hint, by a weakening of the copy, and the two
    /// would alternate for ever.
    pub steps: Vec<Step>,
}

/// What a rule makes of its principal formula in each premise: the
/// formulas that take its place on its side and those that join the
/// other side.
enum Shape {
    /// No premise.
    Leaf,
    /// One premise.
    Unary(Vec<NodeId>, Vec<NodeId>),
    /// Two premises, which take the context as a step says.
    Binary([(Vec<NodeId>, Vec<NodeId>); 2]),
    /// One premise holding an instance of the quantified formula's body,
    /// built by the step from its witness or eigenvariable.
    Instance,
}

/// A proof in progress of an ordinary sequent in LK or LJ. It starts with
/// the sequent as its one open goal. [`apply`](Self::apply) applies a rule
/// to a formula of a goal, named by side and position;
/// [`close`](Self::close) lets the search prove a goal;
/// [`undo`](Self::undo) takes back the last step. Once no goal is open,
/// [`finished`](Self::finished) returns the derivation, checked.
///
/// In intuitionistic and minimal logic every goal has exactly one formula
/// right of `⊢`. A sequent with none starts as `Γ ⊢ ⊥`, the sequent the
/// search decides. The rules whose premise would have none or two (`WR`,
/// `CR`, `⊥R`, and `∨R` with both disjuncts) are not offered, and `¬R`
/// and `¬L` take the form of `→R` and `→L` for `A → ⊥`.
///
/// # Examples
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// use deduxit::Limits;
/// use deduxit::ordinary::{Interactive, Logic, Rule, Sequent, Side, Step};
///
/// let sequent: Sequent = "a /\\ b |- b /\\ a".parse()?;
/// let mut state = Interactive::new(&sequent, Logic::Intuitionistic)?;
/// let root = state.goals().next().unwrap();
/// let [goal] = state.apply(root, &Step::new(Side::Left, 0, Rule::AndLeft))?[..] else {
///     unreachable!()
/// };
/// let goals = state.apply(goal, &Step::new(Side::Right, 0, Rule::AndRight))?;
/// for goal in goals {
///     state.apply(goal, &Step::new(Side::Right, 0, Rule::Axiom))?;
/// }
/// let derivation = state.finished(&Limits::default(), |_| false)?;
/// assert_eq!(derivation.inference(derivation.root()).rule(), Rule::AndLeft);
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # JSON
///
/// With the feature `serialize`, a proof in progress is written as
/// `{"version": 8, "logic": "intuitionistic", "sequent": {…},
/// "inferences": […], "history": […]}` ([`wire`](crate::wire)).
/// `sequent` has the form of an ordinary [`Sequent`]. `inferences` lists
/// the inferences in the state's own order, the conclusion first, each
/// `{"left": [ids], "right": [ids], "rule": name, "principal": ["left",
/// position], "premises": [indices]}`, a quantifier step with its
/// `"term"` as a derivation's has it; an open goal has its two sides
/// alone. `history` lists, in order, the inference each step closed.
///
/// A first-order proof's open witnesses are terms `{"M": k}` of the
/// sequent's `fo` (`$1` for `k = 0`), and `"bindings": [[k, term,
/// step]]` lists those an axiom filled, in the order filled: the
/// witness, the index of its term in `fo`, and the step whose axiom
/// filled it; the key is left out where there is none. The inferences
/// are written as their steps made them, before a filling changed them,
/// and the reader fills them again step by step.
///
/// Reading the form back checks every inference by the checker's rules,
/// so a state read from JSON is as sound as one built through
/// [`apply`](Self::apply).
///
/// Needs the cargo feature `interactive` (on by default).
#[derive(Clone, Debug)]
pub struct Interactive {
    /// The logic.
    logic: Logic,
    /// The formulas, with the nodes the rules need: false, and the two
    /// implications of every equivalence.
    formulas: Formulas,
    /// The node of false.
    falsity: NodeId,
    /// The inferences, top-down: the root first, then the inferences each
    /// step made, in order; an open goal has [`Rule::Open`].
    inferences: Vec<Inference>,
    /// The goals closed so far, in order, for undo.
    history: Vec<InfId>,
    /// The bound on the bytes of a step's run of structural rules, the
    /// `derivation_bytes` the state was started or read under.
    limit: Option<u64>,
    /// The open witnesses, `None` while no step leaves one open.
    witnesses: Option<Box<Witnesses>>,
    /// The bound on the symbols of a term an open witness stands for, the
    /// `terms` the state was started or read under.
    terms: u64,
}

/// Returns an open goal with the sides given.
const fn open(left: Vec<NodeId>, right: Vec<NodeId>) -> Inference {
    Inference {
        term: None,
        left,
        right,
        rule: Rule::Open,
        principal: None,
        premises: Vec::new(),
    }
}

impl Interactive {
    /// Starts a proof of the sequent in the logic, the whole sequent its
    /// one open goal. In intuitionistic and minimal logic a sequent with
    /// nothing right of `⊢` starts as `Γ ⊢ ⊥`.
    ///
    /// # Errors
    ///
    /// [`Error::Succedents`] for an intuitionistic or minimal sequent with
    /// more than one formula right of `⊢`, and [`Refusal::Occurrences`]
    /// past the default bound on subformula occurrences.
    pub fn new(sequent: &Sequent, logic: Logic) -> Result<Self, Error> {
        Self::within(sequent, logic, &Limits::default())
    }

    /// Starts a proof as [`new`](Self::new) does, of a sequent that
    /// unfolds to at most `limits.occurrences` subformula occurrences and
    /// whose terms unfold to at most `limits.terms` symbols, as a
    /// document's reader counts them; the steps are held to the latter
    /// too.
    ///
    /// # Errors
    ///
    /// As [`new`](Self::new) does, the bound being `limits.occurrences`,
    /// and [`Refusal::Terms`] past `limits.terms`.
    pub fn within(sequent: &Sequent, logic: Logic, limits: &Limits) -> Result<Self, Error> {
        if logic != Logic::Classical && sequent.right.len() > 1 {
            return Err(Error::Succedents {
                count: sequent.right.len(),
            });
        }
        let mut formulas = sequent.formulas.clone();
        bound(&formulas, &sequent.left, &sequent.right, limits)?;
        // A document of the proof would be refused when read.
        if let Some(limit) = limits.terms
            && formulas.is_first_order()
        {
            let terms = arena_terms(formulas.first_order());
            if terms > limit {
                return Err(Error::Refused(Refusal::Terms { terms, limit }));
            }
        }
        let falsity = Extra::add(&mut formulas)?.falsity;
        let mut right = sequent.right.clone();
        if logic != Logic::Classical && right.is_empty() {
            right.push(falsity);
        }
        Ok(Self {
            logic,
            formulas,
            falsity,
            inferences: vec![open(sequent.left.clone(), right)],
            history: Vec::new(),
            limit: limits.derivation_bytes,
            witnesses: None,
            terms: limits.terms.unwrap_or(u64::MAX),
        })
    }

    /// Returns the logic.
    pub fn logic(&self) -> Logic {
        self.logic
    }

    /// Returns the formulas the goals and inferences name.
    pub fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    /// Returns the sequent being proved, its two sides.
    pub fn sequent(&self) -> (&[NodeId], &[NodeId]) {
        let root = &self.inferences[0];
        (&root.left, &root.right)
    }

    /// Returns the open goals, in the order they were opened.
    pub fn goals(&self) -> impl Iterator<Item = GoalId> + '_ {
        self.inferences
            .iter()
            .enumerate()
            .filter(|(_, inference)| inference.rule == Rule::Open)
            // Fewer inferences than a `u32` counts: each was made by a
            // step, read from a document, or grafted from a derivation.
            .map(|(i, _)| GoalId::new(i as u32))
    }

    /// Returns the two sides of an open goal.
    ///
    /// # Errors
    ///
    /// [`StepError::NoGoal`] if the id is no open goal.
    pub fn goal(&self, id: GoalId) -> Result<(&[NodeId], &[NodeId]), StepError> {
        let goal = self.open(id)?;
        Ok((&goal.left, &goal.right))
    }

    /// Returns the open goal with the id, or the refusal.
    fn open(&self, id: GoalId) -> Result<&Inference, StepError> {
        self.inferences
            .get(id.index())
            .filter(|inference| inference.rule == Rule::Open)
            .ok_or(StepError::NoGoal { goal: id })
    }

    /// Returns whether no goal is open.
    pub fn is_complete(&self) -> bool {
        self.goals().next().is_none()
    }

    /// Returns how many steps were taken: rules applied and goals closed
    /// by the search, less those undone.
    pub fn steps(&self) -> usize {
        self.history.len()
    }

    /// Returns the goals that the session's steps closed by a rule
    /// invertible in its logic ([`is_invertible`](Self::is_invertible)),
    /// in the order of the goals: an axiom (its weakenings with it), or a
    /// rule of a connective whose premises each kept every other formula
    /// of the goal, a shared context. A step is judged by what it made,
    /// whoever took it: a graft whose last rule is such a rule is listed
    /// too.
    pub fn invertible_steps(&self) -> Vec<GoalId> {
        let steps: crate::hash::HashSet<InfId> = self.history.iter().copied().collect();
        let mut found: Vec<GoalId> = self
            .history
            .iter()
            .filter(|&&id| self.closed_invertibly(id, &steps))
            .map(|id| GoalId::new(id.get()))
            .collect();
        found.sort_unstable();
        found
    }

    /// Returns whether the step that closed inference `id` applied an
    /// invertible rule; `steps` are the inferences the steps closed.
    fn closed_invertibly(&self, id: InfId, steps: &crate::hash::HashSet<InfId>) -> bool {
        let goal = &self.inferences[id.index()];
        // An axiom's step is its weakenings, each with one premise that
        // no other step closed, then the axiom.
        let mut at = goal;
        while matches!(at.rule, Rule::WeakenLeft | Rule::WeakenRight)
            && let [premise] = at.premises[..]
            && !steps.contains(&premise)
        {
            at = &self.inferences[premise.index()];
        }
        if at.rule == Rule::Axiom {
            // An axiom that filled open witnesses chose their terms.
            let step = self.history.iter().position(|&h| h == id);
            return !self.witnesses.as_deref().is_some_and(|w| {
                step.is_some_and(|s| {
                    let s = s as u32;
                    w.unifier.trail_before(s + 1) > w.unifier.trail_before(s)
                })
            });
        }
        let Some((side, position)) = goal.principal else {
            return false;
        };
        let size = |i: &Inference| i.left.len() + i.right.len();
        let shared = goal.premises.len() < 2
            || goal
                .premises
                .iter()
                .all(|p| size(&self.inferences[p.index()]) >= size(goal));
        shared
            && self.is_invertible(
                (&goal.left, &goal.right),
                &Step::new(side, position, goal.rule),
            )
    }

    /// Returns the width of every formula of the state, as the
    /// derivation's estimate counts it. No step adds a formula.
    pub(super) fn widths(&self) -> Vec<u64> {
        super::derivation::widths(&self.formulas)
    }

    /// Returns the bytes the derivation's estimate gives the inferences
    /// from `from` on and the goal `goal`'s, under the state's `widths`:
    /// what a step on `goal` added, measured after it.
    pub(super) fn added_bytes(&self, widths: &[u64], goal: GoalId, from: usize) -> u64 {
        let bytes = |i: &Inference| super::derivation::inference_bytes(widths, &i.left, &i.right);
        let own = bytes(&self.inferences[goal.index()]);
        self.inferences[from..]
            .iter()
            .fold(own, |sum, i| sum.saturating_add(bytes(i)))
    }

    /// Returns how many inferences the state holds.
    pub(super) fn inference_count(&self) -> usize {
        self.inferences.len()
    }

    /// Returns the rules the logic offers for the formula at `position` of
    /// `side` of the open goal: the axiom where the formula stands on the
    /// other side too, the rules of its connective on its side, and
    /// weakening and contraction. [`apply`](Self::apply) still decides
    /// whether a split fits and, in LJ, whether each premise keeps one
    /// formula right of `⊢`.
    ///
    /// # Errors
    ///
    /// [`StepError::NoGoal`] for a goal that is not open, and
    /// [`StepError::NoFormula`] for a position outside its side.
    pub fn rules(
        &self,
        goal: GoalId,
        side: Side,
        position: usize,
    ) -> Result<Vec<Applicable>, StepError> {
        let id = goal;
        let goal = self.open(id)?;
        let x = formula_at(goal, side, position)?;
        let mut rules = Vec::new();
        if goal.side(side.flip()).contains(&x) || self.unifies(id, side, x) {
            rules.push(Applicable {
                rule: Rule::Axiom,
                needs: Needs::Nothing,
            });
        }
        for rule in Rule::ALL {
            if let Ok(shape) = self.shape(rule, side, position, x) {
                rules.push(Applicable {
                    rule,
                    needs: needs_of(rule, &shape),
                });
            }
        }
        Ok(rules)
    }

    /// Returns every rule of the logic that acts on the formula at
    /// `position` of `side` of the open goal, each with whether a step of
    /// it is legal on the goal as it stands: it applies, it takes a
    /// context or a witness and the default is legal, or it is refused
    /// with [`apply`](Self::apply)'s reason.
    ///
    /// The rules are the axiom, which acts on any formula, then those of
    /// the formula's connective on its side and weakening and contraction,
    /// in the order of [`Rule`]'s variants; a rule the logic does not have
    /// (`WR` in LJ, say) is no candidate. Each costs at most one `apply`:
    /// a quantifier step and the axiom of a proof with open witnesses are
    /// taken on a copy and taken back, every other step is checked without
    /// being taken. The proof does not change.
    ///
    /// # Errors
    ///
    /// As [`rules`](Self::rules): [`StepError::NoGoal`] for a goal that is
    /// not open, and [`StepError::NoFormula`] for a position outside its
    /// side.
    pub fn candidates(
        &self,
        goal: GoalId,
        side: Side,
        position: usize,
    ) -> Result<Vec<Candidate>, StepError> {
        let x = formula_at(self.open(goal)?, side, position)?;
        let mut rules = vec![(Rule::Axiom, Needs::Nothing)];
        for rule in Rule::ALL {
            if let Ok(shape) = self.shape(rule, side, position, x) {
                rules.push((rule, needs_of(rule, &shape)));
            }
        }
        // A step that builds instances or binds witnesses changes the
        // formulas, so it is taken on a copy and taken back.
        let mut trial: Option<Self> = None;
        Ok(rules
            .into_iter()
            .map(|(rule, needs)| {
                let step = Step::new(side, position, rule);
                let builds = matches!(
                    rule,
                    Rule::ForallLeft | Rule::ForallRight | Rule::ExistsLeft | Rule::ExistsRight
                ) || (rule == Rule::Axiom && self.witnesses.is_some());
                let judged = if builds {
                    let trial = trial.get_or_insert_with(|| self.clone());
                    trial.apply(goal, &step).map(|_| {
                        trial.undo();
                    })
                } else {
                    self.expand(goal, &step).map(drop)
                };
                let status = match (judged, needs) {
                    (Ok(()), Needs::Nothing) => Status::Applies,
                    (Ok(()), needs) => Status::NeedsInput { needs },
                    (Err(error), _) => Status::Refused(error),
                };
                Candidate { rule, status }
            })
            .collect())
    }

    /// Returns whether the formula `x` on `side` of the open goal `id`
    /// unifies with a formula on the other side by a binding of open
    /// witnesses that the axiom would make. Nothing is bound.
    fn unifies(&self, id: GoalId, side: Side, x: NodeId) -> bool {
        let (Some(w), Ok(goal)) = (self.witnesses.as_deref(), self.open(id)) else {
            return false;
        };
        let scope = self.scope(InfId::new(id.get()));
        goal.side(side.flip()).iter().any(|&y| {
            let pair = match side {
                Side::Left => (x, y),
                Side::Right => (y, x),
            };
            let mut unifier = w.unifier.clone();
            let number = self.history.len() as u32;
            let in_scope = |e| scope.contains(&e);
            unify_axiom(
                &self.formulas,
                &mut unifier,
                pair,
                number,
                &in_scope,
                self.terms,
                true,
            )
            .is_ok()
        })
    }

    /// Returns the open witnesses an axiom has bound, in the order they
    /// were left open: each by its name, `$1` for the first, with the term
    /// it stands for now as text, written as the proof prints its terms.
    pub fn bindings(&self) -> Vec<(String, String)> {
        let Some(w) = self.witnesses.as_deref() else {
            return Vec::new();
        };
        let terms = &self.formulas.first_order().terms;
        let mut found = Vec::new();
        for (inference, raw) in self.inferences.iter().zip(&w.raw) {
            let made = raw.as_ref().map_or(inference.term, |m| m.term);
            let (Some(made), Some(shown)) = (made, inference.term) else {
                continue;
            };
            if let FoTerm::Meta(k) = terms[made.index()]
                && w.unifier.metas()[k as usize].bound.is_some()
            {
                found.push((
                    k,
                    crate::linear::occurrences::instances::meta_name(k),
                    self.formulas.term_text(shown),
                ));
            }
        }
        found.sort_unstable_by_key(|&(k, ..)| k);
        found
            .into_iter()
            .map(|(_, name, term)| (name, term))
            .collect()
    }

    /// Returns the term a closed quantifier step on `goal` took, as text,
    /// or `None` for a goal no quantifier step closed.
    ///
    /// The term is written as the step took it: the eigenvariable of `∀R`
    /// or `∃L` by the name the proof prints it with, the witness of `∀L`
    /// or `∃R` that the step was given, or `$1`, `$2`, … for a witness
    /// left open, whatever an axiom filled it with later.
    /// [`witness`](Self::witness) reads a given witness back, so a front
    /// end that keeps only the steps takes each one again: the document
    /// records a term by its place in the proof's formulas, which another
    /// session numbers otherwise.
    pub fn opened_term(&self, goal: GoalId) -> Option<String> {
        let inference = self.inferences.get(goal.index())?;
        if !matches!(
            inference.rule,
            Rule::ForallLeft | Rule::ForallRight | Rule::ExistsLeft | Rule::ExistsRight
        ) {
            return None;
        }
        let made = self
            .witnesses
            .as_deref()
            .and_then(|w| w.raw.get(goal.index())?.as_ref())
            .map_or(inference.term, |made| made.term)?;
        Some(self.formulas.term_text(made))
    }

    /// Returns the term `term` of the proof's formulas as text that
    /// [`witness`](Self::witness) reads back, an eigenvariable by the name
    /// the proof prints it with, or `None` for an id outside them: what a
    /// front end shows of a step's witness, such as one of
    /// [`Hint::steps`].
    pub fn term_text(&self, term: FoTermId) -> Option<String> {
        (term.index() < self.formulas.first_order().terms.len())
            .then(|| self.formulas.term_text(term))
    }
}

/// Returns `inference` with every bound open witness replaced by its term,
/// as `resolution` replaces them.
fn shown(
    resolution: &mut Resolution,
    formulas: &mut Formulas,
    inference: &Inference,
) -> Result<Inference, Error> {
    let mut side = |list: &[NodeId]| {
        list.iter()
            .map(|&n| resolution.node(formulas, n))
            .collect::<Result<Vec<_>, _>>()
    };
    let (left, right) = (side(&inference.left)?, side(&inference.right)?);
    Ok(Inference {
        left,
        right,
        term: inference.term.map(|t| resolution.term(formulas, t)),
        ..inference.clone()
    })
}

/// Returns the formula at a position of a side of a goal, or the
/// refusal.
fn formula_at(goal: &Inference, side: Side, position: usize) -> Result<NodeId, StepError> {
    let list = goal.side(side);
    list.get(position).copied().ok_or(StepError::NoFormula {
        side,
        position,
        len: list.len(),
    })
}

/// Returns what a step by `rule` of `shape` needs given: a split of the
/// context for a rule of two premises, a witness for `∀L` and `∃R`.
fn needs_of(rule: Rule, shape: &Shape) -> Needs {
    match shape {
        Shape::Binary(_) => Needs::Context,
        Shape::Instance if matches!(rule, Rule::ForallLeft | Rule::ExistsRight) => Needs::Witness,
        Shape::Leaf | Shape::Unary(..) | Shape::Instance => Needs::Nothing,
    }
}

/// Returns the refusal of an arena whose terms would pass `u32::MAX`.
fn terms_full() -> Error {
    Error::Refused(Refusal::Index {
        what: crate::limits::Space::Term,
        count: u64::from(u32::MAX) + 1,
        most: u64::from(u32::MAX),
    })
}

#[cfg(all(test, feature = "parse"))]
mod tests;
