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

mod error;
mod expand;
#[cfg(feature = "serialize")]
mod replay;
mod witness;

use super::derivation::{check_inference, same};
use super::instance::Resolution;
use super::transfer::{self, Parameters};
use super::translate::Extra;
use super::{Derivation, Formulas, Inference, Logic, NodeId, Outcome, Rule, Sequent, Side};
use super::{Evidence, Translation, Verdict, bound, decide_within};
use crate::Error;
use crate::first_order::{Eigen, FoTerm, FoTermId};
use crate::hash::HashMap;
use crate::limits::{Limits, Progress, Refusal};
use crate::linear::proofs::{GoalId, InfId, Structural, ViewOptions};
use crate::linear::search;
pub use error::{BindingFault, StepError};
use expand::premise;
#[cfg(feature = "serialize")]
pub(crate) use replay::Parts;
use witness::{Witnesses, arena_terms, implications_from, unify_axiom};

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
struct Carry {
    /// The constants of that arena that stand for the goal's
    /// eigenvariables and open witnesses, by name, each with the state's
    /// term.
    constants: HashMap<String, FoTermId>,
    /// The names of that arena's symbols and atoms: any other name of the
    /// derivation's is its own, renamed apart from the proof's names.
    names: crate::hash::HashSet<String>,
}

/// Gives a valid first-order outcome whose evidence is no derivation (the
/// clausal proof of grounding's instances) the derivation of LK that
/// Herbrand's theorem builds from ground instances of the sequent, which
/// [`herbrand`](super::herbrand::herbrand) checks; where it builds none
/// within `limits` and `stop`, the outcome stays as it is.
///
/// # Errors
///
/// [`Error::ReadBack`] where the derivation built fails its check, a
/// defect.
#[cfg(feature = "sat")]
fn with_derivation(
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
    match super::herbrand::herbrand(sequent, &super::herbrand::Options::default(), limits, stop) {
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
            let options = super::Options::default()
                .with_logic(self.logic)
                .with_translation(translation)
                .with_method(Some(super::Method::Embedding));
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
        let mut options = super::Options::default()
            .with_logic(self.logic)
            .with_translation(translation);
        if !sequent.formulas.is_first_order() {
            options = options.with_method(Some(super::Method::Embedding));
        }
        // A goal with `=` is decided with the equality axioms; where it
        // lacks them, a derivation is of the goal with them, no derivation
        // of the goal.
        let graftable = !sequent.formulas.has_equality()
            || super::equality::with_axioms(&sequent, limits)?.is_none();
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
        let copied = super::transfer::copy(
            formulas,
            other,
            None,
            &super::transfer::Parameters::Constants,
        )?;
        Ok((0..other.len())
            .map(|k| copied.node(NodeId(k as u32)))
            .collect())
    }

    /// Grafts a derivation of the open goal onto it as one step, its root
    /// in the goal's place and the rest appended in reverse, so that every
    /// premise keeps a larger index than its conclusion; refused past
    /// `limits.derivation_bytes` by the size estimate's measure.
    fn graft(
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

    /// Returns the derivation so far with the open goals as leaves of
    /// [`Rule::Open`], under the default view and limits.
    ///
    /// # Errors
    ///
    /// As [`derivation_within`](Self::derivation_within) does.
    pub fn derivation(&self) -> Result<Derivation, Error> {
        self.derivation_within(&ViewOptions::default(), &Limits::default(), |_| false)
    }

    /// Returns the derivation so far, with the open goals as leaves of
    /// [`Rule::Open`] and the structural rules drawn as `view.structural`
    /// says. Its inferences are numbered premises before conclusions, so
    /// their ids are not this state's; [`derivation_ids`](Self::derivation_ids)
    /// maps them. The derivation is refused past `limits.derivation_bytes`
    /// by its estimate, before anything is built, and when `stop` fires.
    ///
    /// # Errors
    ///
    /// [`Refusal::Output`] past `limits.derivation_bytes`, and
    /// [`Refusal::Stopped`] when `stop` fired.
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

/// Returns the bytes the inferences are estimated at by the measure of
/// [`Size::bytes`]: per inference [`Size::BYTES_PER_INFERENCE`] and
/// [`Size::BYTES_PER_CHARACTER`] for every character of its sequent
/// written, each formula with two for its separator, every binary
/// subformula in brackets; saturating.
fn estimate(formulas: &Formulas, inferences: &[Inference]) -> u64 {
    let widths = super::derivation::widths(formulas);
    inferences.iter().fold(0u64, |sum, inference| {
        let bytes = super::derivation::inference_bytes(&widths, &inference.left, &inference.right);
        sum.saturating_add(bytes)
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
