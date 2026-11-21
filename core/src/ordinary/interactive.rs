// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Step-by-step proving in LK and LJ. A proof in progress is a derivation
//! of an ordinary sequent, built from its conclusion up by the rules of
//! [`Rule`], with the open goals as leaves. A step names a goal, a formula
//! in it by its side and position, and a rule. The step checks the rule
//! against the formula and the logic, checks every inference it makes by
//! the rules the derivation's checker applies, and returns the goals it
//! opens. The search can close a goal through the embedding into linear
//! logic ([`decide_within`]), whose derivations have no cut. A finished
//! proof is a [`Derivation`] that [`Derivation::check`] validates, so this
//! module adds nothing to what must be trusted.
//!
//! Needs the cargo feature `interactive` (on by default).

use super::derivation::{check_inference, same};
use super::instance::Resolution;
use super::translate::Extra;
use super::{Derivation, Formulas, Inference, Logic, Node, NodeId, Outcome, Rule, Sequent, Side};
use super::{Translation, Verdict, bound, decide_within};
use crate::Error;
use crate::first_order::{Eigen, FoTerm, FoTermId};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::proofs::{GoalId, InfId, Size, Structural, ViewOptions};
use crate::linear::search;
use crate::unify::{Clash, Unifier};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Why a step does not apply to a goal as asked. Positions are the ones
/// the step gave, counted from 0 on the side of the goal it named.
///
/// Needs the cargo feature `interactive` (on by default).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum StepError {
    /// No open goal has this id.
    #[non_exhaustive]
    NoGoal {
        /// The id asked for.
        goal: GoalId,
    },
    /// The side has no formula at the position; it has `len` formulas.
    #[non_exhaustive]
    NoFormula {
        /// The side asked for.
        side: Side,
        /// The position asked for.
        position: usize,
        /// How many formulas the side has.
        len: usize,
    },
    /// The rule does not act on the formula: the formula has another
    /// connective, or the rule is for the other side of `⊢`. Cut and the
    /// mark of an open goal act on no formula.
    #[non_exhaustive]
    Rule {
        /// The rule asked for.
        rule: Rule,
        /// The side of the formula.
        side: Side,
        /// The position of the formula.
        position: usize,
    },
    /// The logic does not offer the rule: ex falso (`⊥L`) in minimal
    /// logic, and in intuitionistic and minimal logic a rule whose premise
    /// would have two formulas right of `⊢` or none, since every goal there
    /// has exactly one.
    #[non_exhaustive]
    Logic {
        /// The rule asked for.
        rule: Rule,
        /// The logic of the proof.
        logic: Logic,
    },
    /// The axiom needs the formula on the other side of `⊢` too.
    #[non_exhaustive]
    NoPartner {
        /// The side of the formula.
        side: Side,
        /// The position of the formula.
        position: usize,
    },
    /// In intuitionistic or minimal logic, a premise would have this many
    /// formulas right of `⊢` instead of one.
    #[non_exhaustive]
    Succedents {
        /// How many formulas would stand right of `⊢`.
        count: usize,
    },
    /// A position of the split is out of range, repeated, or the formula
    /// the rule acts on.
    #[non_exhaustive]
    Split {
        /// The side of the position.
        side: Side,
        /// The offending position.
        position: usize,
    },
    /// A split was given to a rule with fewer than two premises.
    #[non_exhaustive]
    NoSplit {
        /// The rule asked for.
        rule: Rule,
    },
    /// The weakenings below an axiom, or the contractions below a shared
    /// rule, are estimated past the bound on the derivation's bytes that
    /// the proof was started or read under.
    #[non_exhaustive]
    Size {
        /// The estimate of the run, in bytes.
        estimate_bytes: u64,
        /// The bound, `Limits::derivation_bytes`.
        limit_bytes: u64,
    },
    /// A witness was given to a rule that takes none: only `∀L` and `∃R`
    /// take one, and the session names the eigenvariable of `∀R` and `∃L`.
    #[non_exhaustive]
    NoWitness {
        /// The rule asked for.
        rule: Rule,
    },
    /// The witness of `∀L` or `∃R` is not a term of the goal: a term of
    /// the sequent's function symbols and of the eigenvariables in the
    /// goal's scope.
    #[non_exhaustive]
    Witness {
        /// Why, in words.
        reason: String,
    },
    /// The axiom's formula unifies with a formula on the other side of `⊢`
    /// only by giving an open witness a term it may not take: an
    /// eigenvariable that a `∀R` or `∃L` introduced above the step that
    /// left the witness open, or on another branch.
    #[non_exhaustive]
    Binding {
        /// Why, in words.
        reason: String,
    },
    /// The step would make the proof's terms unfold to more symbols
    /// written out than the bound on terms (`Limits::terms`) the proof
    /// was started or read under: an instance, or the terms an axiom
    /// fills open witnesses with. A document of such a proof would be
    /// refused when read.
    #[non_exhaustive]
    Terms {
        /// The terms written out, as a document's reader counts them, or
        /// a number past the bound.
        terms: u64,
        /// The bound.
        limit: u64,
    },
}

/// Returns `left of ⊢` or `right of ⊢`.
const fn of(side: Side) -> &'static str {
    match side {
        Side::Left => "left of ⊢",
        Side::Right => "right of ⊢",
    }
}

/// Returns what a rule acts on, as a phrase: `a conjunction right of ⊢`.
const fn target(rule: Rule) -> &'static str {
    use Rule::*;
    match rule {
        Axiom => "a formula that stands on both sides of ⊢",
        WeakenLeft | ContractLeft => "any formula left of ⊢",
        WeakenRight | ContractRight => "any formula right of ⊢",
        AndLeft | AndLeft1 | AndLeft2 => "a conjunction left of ⊢",
        AndRight => "a conjunction right of ⊢",
        OrLeft => "a disjunction left of ⊢",
        OrRight | OrRight1 | OrRight2 => "a disjunction right of ⊢",
        ImpliesLeft => "an implication left of ⊢",
        ImpliesRight => "an implication right of ⊢",
        NotLeft => "a negation left of ⊢",
        NotRight => "a negation right of ⊢",
        IffLeft | IffLeft1 | IffLeft2 => "an equivalence left of ⊢",
        IffRight => "an equivalence right of ⊢",
        FalseLeft => "⊥ left of ⊢",
        FalseRight => "⊥ right of ⊢",
        TrueLeft => "⊤ left of ⊢",
        TrueRight => "⊤ right of ⊢",
        Cut => "no formula of a goal, so no step takes it",
        Open => "no formula, since it only marks an open goal",
        ForallLeft => "a universal formula left of ⊢",
        ForallRight => "a universal formula right of ⊢",
        ExistsLeft => "an existential formula left of ⊢",
        ExistsRight => "an existential formula right of ⊢",
    }
}

impl Display for StepError {
    /// Writes the refusal as a sentence that names the rule, side and
    /// position, and says what the step needs.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            StepError::NoGoal { goal } => write!(f, "there is no open goal {}", goal.get()),
            StepError::NoFormula {
                side,
                position,
                len,
            } => write!(
                f,
                "the goal has no formula {position} {}: it has {} there, and positions \
                 start at 0",
                of(*side),
                crate::errors::counted(*len, "formula", "formulas")
            ),
            StepError::Rule {
                rule,
                side,
                position,
            } => write!(
                f,
                "the rule {rule} does not act on formula {position} {}: it acts on {}",
                of(*side),
                target(*rule)
            ),
            StepError::Logic { rule, logic } => {
                let why = match (rule, logic) {
                    (Rule::FalseLeft, Logic::Minimal) => "minimal logic has no ex falso",
                    (Rule::OrRight, _) => {
                        "its premise would have both disjuncts right of ⊢, and every goal of LJ \
                         has exactly one formula there: use ∨R₁ or ∨R₂ to keep one disjunct"
                    }
                    (Rule::ContractRight, _) => {
                        "its premise would have two formulas right of ⊢, and every goal of LJ \
                         has exactly one"
                    }
                    _ => {
                        "its premise would have no formula right of ⊢, and every goal of LJ has \
                         exactly one"
                    }
                };
                write!(
                    f,
                    "the rule {rule} is not available in {logic} logic: {why}"
                )
            }
            StepError::NoPartner { side, position } => write!(
                f,
                "the axiom needs formula {position} {} to stand {} too",
                of(*side),
                of(side.flip())
            ),
            StepError::Succedents { count } => write!(
                f,
                "a premise would have {} right of ⊢, and every goal of LJ has exactly one: \
                 share the context, or split it so that each premise keeps one formula right \
                 of ⊢",
                crate::errors::counted(*count, "formula", "formulas")
            ),
            StepError::Split { side, position } => write!(
                f,
                "the split's position {position} {} is out of range, named twice, or the \
                 formula the rule acts on: name each other formula of the goal at most once",
                of(*side)
            ),
            StepError::Size {
                estimate_bytes,
                limit_bytes,
            } => write!(
                f,
                "the weakenings or contractions this step makes are estimated at \
                 {estimate_bytes} bytes, over the derivation bound of {limit_bytes} bytes the \
                 proof was started or read with: start it with a higher bound"
            ),
            StepError::NoSplit { rule } => {
                write!(
                    f,
                    "the rule {rule} has fewer than two premises and takes no split"
                )
            }
            StepError::NoWitness { rule } => write!(
                f,
                "the rule {rule} takes no witness: only ∀L and ∃R take one, and the session \
                 names the eigenvariable of ∀R and ∃L"
            ),
            StepError::Witness { reason } => write!(f, "the witness is not a term here: {reason}"),
            StepError::Binding { reason } => {
                write!(f, "the axiom cannot fill the open witnesses: {reason}")
            }
            StepError::Terms { terms, limit } => write!(
                f,
                "the step would make the proof's terms unfold to {terms} symbols written out, \
                 more than the bound of {limit} on terms the proof was started or read with: \
                 start it with a higher bound"
            ),
        }
    }
}

impl std::error::Error for StepError {}

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

/// An inference's sides and term as the step that made it left them.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Made {
    /// The formulas left of `⊢`.
    left: Vec<NodeId>,
    /// The formulas right of `⊢`.
    right: Vec<NodeId>,
    /// The term of a quantifier rule.
    term: Option<FoTermId>,
}

impl Made {
    /// Returns the sides and term of `inference`.
    fn of(inference: &Inference) -> Self {
        Self {
            left: inference.left.clone(),
            right: inference.right.clone(),
            term: inference.term,
        }
    }

    /// Returns whether `inference` has these sides and this term.
    fn shows(&self, inference: &Inference) -> bool {
        self.left == inference.left && self.right == inference.right && self.term == inference.term
    }
}

/// The open witnesses of a proof in progress: the witnesses of `∀L` and
/// `∃R` steps taken without one, which an axiom fills by unification.
///
/// The inferences show every binding in force: an inference that holds a
/// bound witness holds its term in its place, so the checker's rules read
/// them as they read any inference. What an inference was made as, before
/// a binding changed it, is kept beside, so that an undo shows it again.
#[derive(Clone, Debug, Default)]
struct Witnesses {
    /// The open witnesses by number, their levels and bindings, and the
    /// trail of bindings by step. A witness's level is one more than the
    /// largest eigenvariable in its step's scope, so a binding takes no
    /// eigenvariable made after it.
    unifier: Unifier,
    /// The step that made each open witness, by number.
    made: Vec<u32>,
    /// Per inference, its sides and term as made, where a binding changed
    /// them since; as long as the inferences.
    raw: Vec<Option<Made>>,
    /// Per open witness, the inferences registered as showing it: every
    /// inference that showed it since it was made and still stands, so
    /// every one that shows it now, and every one a binding of it changed.
    /// A binding is shown, and taken back, through these alone.
    holders: Vec<crate::hash::HashSet<u32>>,
    /// Per inference, the open witnesses it is registered under; as long
    /// as the inferences.
    registered: Vec<Vec<u32>>,
}

impl Witnesses {
    /// Returns no witnesses for a state of `len` inferences.
    fn new(len: usize) -> Self {
        Self {
            raw: vec![None; len],
            registered: vec![Vec::new(); len],
            ..Self::default()
        }
    }

    /// Makes an unbound open witness of term `term` at `level`, made by the
    /// step numbered `step`.
    fn push(&mut self, term: FoTermId, level: u32, step: u32) {
        self.unifier.push_meta(term, level);
        self.made.push(step);
        self.holders.push(crate::hash::HashSet::default());
    }

    /// Registers the inference `i` under each of the open witnesses
    /// `metas` that it is not registered under yet.
    fn register(&mut self, i: usize, metas: &[u32]) {
        // Fewer inferences than a `u32` counts.
        let id = i as u32;
        for &k in metas {
            if let Some(holders) = self.holders.get_mut(k as usize)
                && holders.insert(id)
            {
                self.registered[i].push(k);
            }
        }
    }

    /// Forgets the registrations of the inferences from `len` on, which an
    /// undo took away.
    fn forget_from(&mut self, len: usize) {
        for (i, metas) in self.registered.iter().enumerate().skip(len) {
            for &k in metas {
                if let Some(holders) = self.holders.get_mut(k as usize) {
                    // Fewer inferences than a `u32` counts.
                    holders.remove(&(i as u32));
                }
            }
        }
        self.registered.truncate(len);
        self.raw.truncate(len);
    }

    /// Returns the inferences registered under any of the open witnesses
    /// `metas`, ascending.
    fn holding(&self, metas: impl Iterator<Item = u32>) -> Vec<usize> {
        let mut found: Vec<usize> = metas
            .filter_map(|k| self.holders.get(k as usize))
            .flatten()
            .map(|&i| i as usize)
            .collect();
        found.sort_unstable();
        found.dedup();
        found
    }
}

/// Why an axiom's formulas do not unify as a session may bind them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unbound {
    /// The formulas differ in a connective, a predicate or a symbol, or a
    /// witness would occur in its own term.
    Differ,
    /// A witness would take this eigenvariable, made after it or on
    /// another branch.
    Eigen(Eigen),
    /// The proof's terms would unfold to this many symbols written out,
    /// past the bound on terms.
    Large(u64),
}

/// Unifies the formulas `left` and `right` of an axiom in `formulas`,
/// binding open witnesses under `step`. A binding takes no eigenvariable at
/// or above its witness's level and none that `in_scope` refuses, and no
/// term that unfolds to more than `most` symbols. With `arena`, the terms
/// of the arena once the bindings are shown must unfold to at most `most`
/// symbols too, by the measure a document's reader takes
/// ([`arena_terms`]): the session asks it, so that it writes no document
/// its reader refuses; the reader, whose document that measure has
/// bounded already, does not, since its arena is the last step's. On a
/// refusal nothing is left bound.
fn unify_axiom(
    formulas: &Formulas,
    unifier: &mut Unifier,
    (left, right): (NodeId, NodeId),
    step: u32,
    in_scope: &impl Fn(FoTermId) -> bool,
    most: u64,
    arena: bool,
) -> Result<(), Unbound> {
    let pairs = formulas.term_pairs(left, right).ok_or(Unbound::Differ)?;
    let terms = formulas.first_order();
    let was: Vec<bool> = unifier.metas().iter().map(|m| m.bound.is_some()).collect();
    let mark = unifier.trail_len();
    match unifier.unify(terms, pairs, step, &|_| false) {
        Ok(()) => {}
        Err(Clash::Eigen(e)) => return Err(Unbound::Eigen(e)),
        Err(Clash::Terms) => return Err(Unbound::Differ),
    }
    // Each new binding's term written out, its bindings followed: shared
    // terms could stand for one exponentially long.
    let mut sizes: crate::hash::HashMap<FoTermId, u64> = crate::hash::HashMap::default();
    let new: Vec<FoTermId> = unifier
        .metas()
        .iter()
        .zip(&was)
        .filter(|&(_, &was)| !was)
        .filter_map(|(m, _)| m.bound)
        .collect();
    let mut largest = 0;
    for &t in &new {
        largest = largest.max(unfolded(terms, unifier, t, &mut sizes));
    }
    if largest > most {
        unifier.undo_to(mark);
        return Err(Unbound::Large(largest));
    }
    let mut total = if arena {
        arena_terms(terms).max(largest)
    } else {
        0
    };
    // Each atom that holds a witness bound now makes at most one atom more,
    // its arguments resolved: what showing the bindings builds.
    let fresh: Vec<bool> = unifier
        .metas()
        .iter()
        .zip(&was)
        .map(|(m, &was)| !was && m.bound.is_some())
        .collect();
    let mut holds = vec![false; terms.terms.len()];
    for (n, term) in terms.terms.iter().enumerate() {
        holds[n] = match *term {
            FoTerm::Meta(k) => fresh.get(k as usize).copied().unwrap_or(false),
            FoTerm::App(_, args) => terms.list(args).iter().any(|a| holds[a.index()]),
            FoTerm::Bound(_) | FoTerm::Eigen(_) => false,
        };
    }
    for &(_, args) in terms.atoms.iter().filter(|_| arena) {
        let list = terms.list(args);
        if list.iter().any(|a| holds[a.index()]) {
            for &a in list {
                total = total.saturating_add(unfolded(terms, unifier, a, &mut sizes));
            }
        }
    }
    if total > most {
        unifier.undo_to(mark);
        return Err(Unbound::Large(total));
    }
    // Every eigenvariable a new binding names, its bindings followed.
    let mut seen = crate::hash::HashSet::default();
    let mut stack = new;
    while let Some(u) = stack.pop() {
        let u = unifier.resolve(terms, u);
        if !seen.insert(u) {
            continue;
        }
        match terms.terms[u.index()] {
            FoTerm::App(_, args) => stack.extend_from_slice(terms.list(args)),
            FoTerm::Eigen(e) if !in_scope(u) => {
                unifier.undo_to(mark);
                return Err(Unbound::Eigen(e));
            }
            FoTerm::Eigen(_) | FoTerm::Meta(_) | FoTerm::Bound(_) => {}
        }
    }
    Ok(())
}

/// Returns the measure of an arena's terms that a document's reader holds
/// to `limits.terms`: the terms the atoms' arguments unfold to, summed over
/// the atoms, or the largest term where it is larger.
fn arena_terms(terms: &crate::first_order::FirstOrder<NodeId>) -> u64 {
    let unfolded = terms.unfolded();
    let largest = unfolded.iter().copied().max().unwrap_or(0);
    terms
        .atoms
        .iter()
        .fold(0u64, |sum, &(_, args)| {
            terms
                .list(args)
                .iter()
                .fold(sum, |sum, t| sum.saturating_add(unfolded[t.index()]))
        })
        .max(largest)
}

/// Returns how many symbols the term `t` has written out, its bound
/// witnesses replaced by their terms, saturating; `sizes` keeps what was
/// counted. It walks on a stack of its own, each shared term once.
fn unfolded(
    terms: &crate::first_order::FirstOrder<NodeId>,
    unifier: &Unifier,
    t: FoTermId,
    sizes: &mut crate::hash::HashMap<FoTermId, u64>,
) -> u64 {
    let mut stack = vec![(t, false)];
    while let Some((u, done)) = stack.pop() {
        if sizes.contains_key(&u) {
            continue;
        }
        // A witness an undone step made stays in the arena's atoms with a
        // number past the live ones: it is a leaf.
        let r = match terms.terms[u.index()] {
            FoTerm::Meta(k) if k as usize >= unifier.metas().len() => u,
            FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                unifier.resolve(terms, u)
            }
        };
        let parts: &[FoTermId] = match terms.terms[r.index()] {
            FoTerm::App(_, args) => terms.list(args),
            FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => &[],
        };
        if done {
            let size = parts.iter().fold(1u64, |sum, p| {
                sum.saturating_add(sizes.get(p).copied().unwrap_or(0))
            });
            sizes.insert(u, size);
        } else {
            stack.push((u, true));
            stack.extend(parts.iter().map(|&p| (p, false)));
        }
    }
    sizes.get(&t).copied().unwrap_or(1)
}

/// Adds both implications of every equivalence of the arena from node
/// `from` on, which `shape` and the checker look up: a binding or an
/// instance can build an equivalence the arena lacked. An implication
/// added is no equivalence, so one pass suffices.
fn implications_from(formulas: &mut Formulas, from: usize) -> Result<(), Error> {
    let mut n = from;
    while n < formulas.len() {
        // As many nodes as the arena's ids number.
        if let Node::Iff(a, b) = formulas.node(NodeId(n as u32)) {
            formulas.add(Node::Implies(a, b))?;
            formulas.add(Node::Implies(b, a))?;
        }
        n += 1;
    }
    Ok(())
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
    /// unfolds to at most `limits.occurrences` subformula occurrences.
    ///
    /// # Errors
    ///
    /// As [`new`](Self::new) does, the bound being `limits.occurrences`.
    pub fn within(sequent: &Sequent, logic: Logic, limits: &Limits) -> Result<Self, Error> {
        if logic != Logic::Classical && sequent.right.len() > 1 {
            return Err(Error::Succedents {
                count: sequent.right.len(),
            });
        }
        let mut formulas = sequent.formulas.clone();
        bound(&formulas, &sequent.left, &sequent.right, limits)?;
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
                    needs: match shape {
                        Shape::Binary(_) => Needs::Context,
                        Shape::Instance if matches!(rule, Rule::ForallLeft | Rule::ExistsRight) => {
                            Needs::Witness
                        }
                        Shape::Leaf | Shape::Unary(..) | Shape::Instance => Needs::Nothing,
                    },
                });
            }
        }
        Ok(rules)
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

    /// Returns what `rule` makes of the formula `x` at `position` of
    /// `side`, or why it does not act on it in this logic. The axiom and
    /// an open goal are no shape.
    fn shape(
        &self,
        rule: Rule,
        side: Side,
        position: usize,
        x: NodeId,
    ) -> Result<Shape, StepError> {
        use Rule::*;
        use Side::{Left, Right};
        let node = self.formulas.node(x);
        let fits = match rule {
            Axiom | Cut | Open => false,
            ForallLeft => side == Left && matches!(node, Node::Forall(_)),
            ForallRight => side == Right && matches!(node, Node::Forall(_)),
            ExistsLeft => side == Left && matches!(node, Node::Exists(_)),
            ExistsRight => side == Right && matches!(node, Node::Exists(_)),
            WeakenLeft | ContractLeft => side == Left,
            WeakenRight | ContractRight => side == Right,
            AndLeft | AndLeft1 | AndLeft2 => side == Left && matches!(node, Node::And(..)),
            AndRight => side == Right && matches!(node, Node::And(..)),
            OrLeft => side == Left && matches!(node, Node::Or(..)),
            OrRight | OrRight1 | OrRight2 => side == Right && matches!(node, Node::Or(..)),
            ImpliesLeft => side == Left && matches!(node, Node::Implies(..)),
            ImpliesRight => side == Right && matches!(node, Node::Implies(..)),
            NotLeft => side == Left && matches!(node, Node::Not(_)),
            NotRight => side == Right && matches!(node, Node::Not(_)),
            IffLeft | IffLeft1 | IffLeft2 => side == Left && matches!(node, Node::Iff(..)),
            IffRight => side == Right && matches!(node, Node::Iff(..)),
            FalseLeft => side == Left && node == Node::False,
            FalseRight => side == Right && node == Node::False,
            TrueLeft => side == Left && node == Node::True,
            TrueRight => side == Right && node == Node::True,
        };
        if !fits {
            return Err(StepError::Rule {
                rule,
                side,
                position,
            });
        }
        let classical = self.logic == Logic::Classical;
        let refused = match rule {
            WeakenRight | ContractRight | OrRight | FalseRight => !classical,
            FalseLeft => self.logic == Logic::Minimal,
            Axiom | WeakenLeft | ContractLeft | AndLeft | AndLeft1 | AndLeft2 | AndRight
            | OrLeft | OrRight1 | OrRight2 | ImpliesLeft | ImpliesRight | NotLeft | NotRight
            | IffLeft | IffLeft1 | IffLeft2 | IffRight | TrueLeft | TrueRight | Cut | Open
            | ForallLeft | ForallRight | ExistsLeft | ExistsRight => false,
        };
        if refused {
            return Err(StepError::Logic {
                rule,
                logic: self.logic,
            });
        }
        let ids = &self.formulas.ids;
        // Equivalences have their implications in the arena, which the
        // state added at its start.
        let iff = |a, b| (ids[&Node::Implies(a, b)], ids[&Node::Implies(b, a)]);
        let falsity = self.falsity;
        Ok(match (rule, node) {
            (ForallLeft | ForallRight | ExistsLeft | ExistsRight, _) => Shape::Instance,
            (WeakenLeft | WeakenRight | TrueLeft | FalseRight, _) => Shape::Unary(vec![], vec![]),
            (ContractLeft | ContractRight, _) => Shape::Unary(vec![x, x], vec![]),
            (TrueRight | FalseLeft, _) => Shape::Leaf,
            (AndLeft, Node::And(a, b)) | (OrRight, Node::Or(a, b)) => {
                Shape::Unary(vec![a, b], vec![])
            }
            (AndLeft1, Node::And(a, _)) | (OrRight1, Node::Or(a, _)) => {
                Shape::Unary(vec![a], vec![])
            }
            (AndLeft2, Node::And(_, b)) | (OrRight2, Node::Or(_, b)) => {
                Shape::Unary(vec![b], vec![])
            }
            (ImpliesRight, Node::Implies(a, b)) => Shape::Unary(vec![b], vec![a]),
            (NotRight, Node::Not(a)) if classical => Shape::Unary(vec![], vec![a]),
            (NotRight, Node::Not(a)) => Shape::Unary(vec![falsity], vec![a]),
            (NotLeft, Node::Not(a)) if classical => Shape::Unary(vec![], vec![a]),
            (NotLeft, Node::Not(a)) => Shape::Binary([(vec![], vec![a]), (vec![falsity], vec![])]),
            (IffLeft, Node::Iff(a, b)) => {
                let (forward, backward) = iff(a, b);
                Shape::Unary(vec![forward, backward], vec![])
            }
            (IffLeft1, Node::Iff(a, b)) => Shape::Unary(vec![iff(a, b).0], vec![]),
            (IffLeft2, Node::Iff(a, b)) => Shape::Unary(vec![iff(a, b).1], vec![]),
            (AndRight, Node::And(a, b)) | (OrLeft, Node::Or(a, b)) => {
                Shape::Binary([(vec![a], vec![]), (vec![b], vec![])])
            }
            (ImpliesLeft, Node::Implies(a, b)) => {
                Shape::Binary([(vec![], vec![a]), (vec![b], vec![])])
            }
            (IffRight, Node::Iff(a, b)) => {
                let (forward, backward) = iff(a, b);
                Shape::Binary([(vec![forward], vec![]), (vec![backward], vec![])])
            }
            // Every other pair failed `fits`.
            _ => unreachable!("{rule} fits a formula it has no shape for"),
        })
    }

    /// Applies a step to the open goal and returns the goals it opens, in
    /// the rule's order of premises; a rule without premises opens none.
    ///
    /// The axiom closes a goal whose formula stands on both sides of `⊢`,
    /// weakening every other formula away first. The derivation shows
    /// these weakenings unless its structural rules are drawn admissible
    /// ([`Structural`]). A rule with two premises gives each the whole
    /// context unless the step splits it ([`Step::split`]). In
    /// intuitionistic and minimal logic, `→L` and `¬L` have a first premise
    /// with its own formula right of `⊢`. With a shared context they give
    /// every hypothesis to both premises and the goal's formula right of
    /// `⊢` to the second; the hypotheses are contracted first, which an
    /// admissible drawing leaves out.
    ///
    /// Every inference the step makes is checked by the rules of the
    /// derivation's checker before the state changes. A refused step
    /// changes nothing.
    ///
    /// # Errors
    ///
    /// A [`StepError`] that names what the rule needed: an open goal, a
    /// formula at the position, a rule that acts on it in the logic, the
    /// formula on both sides for the axiom, a split that fits, and in
    /// intuitionistic and minimal logic one formula right of `⊢` in every
    /// premise.
    pub fn apply(&mut self, goal: GoalId, step: &Step) -> Result<Vec<GoalId>, StepError> {
        if matches!(
            step.rule,
            Rule::ForallLeft | Rule::ForallRight | Rule::ExistsLeft | Rule::ExistsRight
        ) {
            let made = self.instance_step(goal, step)?;
            return Ok(self.record(goal, made));
        }
        if step.witness.is_some() {
            return Err(StepError::NoWitness { rule: step.rule });
        }
        if step.rule == Rule::Axiom
            && self.witnesses.is_some()
            && let Some(opened) = self.unifying_axiom(goal, step)?
        {
            return Ok(opened);
        }
        let made = self.expand(goal, step)?;
        Ok(self.record(goal, made))
    }

    /// Closes the goal by the axiom on the formula the step names and the
    /// first formula on the other side of `⊢` that unifies with it, where
    /// none is equal to it: the open witnesses take the unifier's terms,
    /// every inference that holds them shows those terms, and the axiom's
    /// weakenings and `A ⊢ A` are made as [`axiom`] makes them. Returns the
    /// goals opened, none, or `None` where a formula on the other side is
    /// equal to it, which needs no binding.
    fn unifying_axiom(
        &mut self,
        id: GoalId,
        step: &Step,
    ) -> Result<Option<Vec<GoalId>>, StepError> {
        let (side, position) = (step.side, step.position);
        let goal = self.open(id)?;
        let x = formula_at(goal, side, position)?;
        if step.context != Context::Shared || goal.side(side.flip()).contains(&x) {
            return Ok(None);
        }
        let len = goal.left.len() + goal.right.len();
        self.run_within(len, len)?;
        let goal = goal.clone();
        let scope = self.scope(InfId::new(id.get()));
        // The step's number, under which its bindings are trailed. Fewer
        // steps than inferences, which a `u32` counts.
        let number = self.history.len() as u32;
        let Some(w) = self.witnesses.as_deref_mut() else {
            return Ok(None);
        };
        let mark = w.unifier.trail_len();
        let was: Vec<bool> = w
            .unifier
            .metas()
            .iter()
            .map(|m| m.bound.is_some())
            .collect();
        let mut refused = Unbound::Differ;
        let mut partner = None;
        for (at, &y) in goal.side(side.flip()).iter().enumerate() {
            let pair = match side {
                Side::Left => (x, y),
                Side::Right => (y, x),
            };
            let in_scope = |e| scope.contains(&e);
            match unify_axiom(
                &self.formulas,
                &mut w.unifier,
                pair,
                number,
                &in_scope,
                self.terms,
                true,
            ) {
                Ok(()) => {
                    partner = Some(at);
                    break;
                }
                Err(Unbound::Differ) => {}
                Err(why) if refused == Unbound::Differ => refused = why,
                Err(_) => {}
            }
        }
        let Some(partner) = partner else {
            return Err(match refused {
                Unbound::Eigen(e) => {
                    let name = self.eigen_name(e);
                    StepError::Binding {
                        reason: format!(
                            "an open witness would take the eigenvariable {name}, which a ∀R or \
                             ∃L introduced after the witness was left open or on another branch"
                        ),
                    }
                }
                Unbound::Large(size) => StepError::Terms {
                    terms: size,
                    limit: self.terms,
                },
                Unbound::Differ => StepError::NoPartner { side, position },
            });
        };
        let made = axiom(&goal, side, position, partner);
        let fresh: Vec<u32> = self.witnesses.as_deref().map_or_else(Vec::new, |w| {
            (0..w.unifier.next_meta())
                .filter(|&k| w.unifier.metas()[k as usize].bound.is_some() && !was[k as usize])
                .collect()
        });
        match self.bind(&made, &fresh) {
            Ok((shown, raw)) => {
                let opened = self.record_made(id, shown, raw);
                Ok(Some(opened))
            }
            Err(error) => {
                if let Some(w) = self.witnesses.as_deref_mut() {
                    w.unifier.undo_to(mark);
                }
                Err(error)
            }
        }
    }

    /// Shows the bindings the unifier holds now in every inference that
    /// shows one of the open witnesses `fresh`, which the step bound, and
    /// returns the inferences `made` as they show them, with what each was
    /// made as where that differs. Only the inferences registered under
    /// `fresh` can change. Every inference that changes is checked by the
    /// checker's rules first; one that a binding would break (an
    /// eigenvariable brought into its rule's conclusion) refuses the
    /// binding and changes nothing.
    fn bind(
        &mut self,
        made: &[Inference],
        fresh: &[u32],
    ) -> Result<(Vec<Inference>, Vec<Option<Made>>), StepError> {
        let Some(w) = self.witnesses.as_deref_mut() else {
            return Ok((made.to_vec(), vec![None; made.len()]));
        };
        let bindings = w.unifier.metas().iter().map(|m| m.bound).collect();
        let mut resolution = Resolution::new(bindings);
        let full = |error: Error| StepError::Binding {
            reason: error.to_string(),
        };
        let built = self.formulas.len();
        let mut changed = crate::hash::HashMap::default();
        for i in w.holding(fresh.iter().copied()) {
            let inference = &self.inferences[i];
            let shown = shown(&mut resolution, &mut self.formulas, inference).map_err(full)?;
            if shown != *inference {
                changed.insert(i, shown);
            }
        }
        let shown_made = made
            .iter()
            .map(|i| shown(&mut resolution, &mut self.formulas, i))
            .collect::<Result<Vec<_>, _>>()
            .map_err(full)?;
        implications_from(&mut self.formulas, built).map_err(full)?;
        let at = |i: InfId| {
            changed
                .get(&i.index())
                .unwrap_or(&self.inferences[i.index()])
        };
        for inference in changed.values().filter(|i| i.rule != Rule::Open) {
            let premises: Vec<&Inference> = inference.premises.iter().map(|&p| at(p)).collect();
            if check_inference(&self.formulas, self.logic, inference, &premises).is_err() {
                return Err(StepError::Binding {
                    reason: "an inference that holds the witnesses would break its rule once \
                             they are filled: an eigenvariable would stand in the conclusion of \
                             the ∀R or ∃L that introduced it"
                        .to_owned(),
                });
            }
        }
        let raw = made
            .iter()
            .zip(&shown_made)
            .map(|(m, s)| (m != s).then(|| Made::of(m)))
            .collect();
        let shown_made = self.checked(shown_made);
        let w = self
            .witnesses
            .as_deref_mut()
            .expect("the witnesses are there");
        for (i, inference) in changed {
            w.raw[i].get_or_insert_with(|| Made::of(&self.inferences[i]));
            let metas = self.formulas.metas_in(
                inference.left.iter().chain(&inference.right).copied(),
                inference.term.into_iter(),
            );
            w.register(i, &metas);
            let at = &mut self.inferences[i];
            at.left = inference.left;
            at.right = inference.right;
            at.term = inference.term;
        }
        Ok((shown_made, raw))
    }

    /// Returns the name an eigenvariable is printed with.
    fn eigen_name(&self, e: Eigen) -> String {
        self.formulas
            .eigen_names()
            .into_iter()
            .find(|&(k, _)| k == e.get())
            .map_or_else(|| format!("#{}", e.get()), |(_, name)| name)
    }

    /// Returns the inferences a step of a quantifier rule makes: the goal's
    /// own, with the step's witness or a fresh eigenvariable as its term,
    /// and its premise, open, which holds the instance of the quantified
    /// formula's body in its place (beside it for `∀L`, and for `∃R` in
    /// LK). The instance is built in the arena where it is new.
    fn instance_step(&mut self, id: GoalId, step: &Step) -> Result<Vec<Inference>, StepError> {
        let (side, position, rule) = (step.side, step.position, step.rule);
        let x = formula_at(self.open(id)?, side, position)?;
        self.shape(rule, side, position, x)?;
        if step.context != Context::Shared {
            return Err(StepError::NoSplit { rule });
        }
        let (Node::Forall(body) | Node::Exists(body)) = self.formulas.node(x) else {
            unreachable!("{rule} fits a quantified formula alone")
        };
        let eigenvariable = matches!(rule, Rule::ForallRight | Rule::ExistsLeft);
        // `∀L` keeps its principal, and `∃R` where the right side has room.
        let keep = !eigenvariable && (rule == Rule::ForallLeft || self.logic == Logic::Classical);
        // An open witness's number and level, made once the instance is.
        let mut open = None;
        let term = match step.witness {
            Some(_) if eigenvariable => return Err(StepError::NoWitness { rule }),
            None if eigenvariable => self.fresh_eigenvariable()?,
            Some(t) => self.held(id, t)?,
            None => {
                let (term, level) = self.open_witness(id)?;
                open = Some(level);
                term
            }
        };
        self.instance_within(body, term)?;
        let instance =
            self.formulas
                .instantiate(body, term)
                .map_err(|error| StepError::Witness {
                    reason: error.to_string(),
                })?;
        // An instance may hold an equivalence the arena did not have, whose
        // implications `shape` looks up.
        Extra::add(&mut self.formulas).map_err(|error| StepError::Witness {
            reason: error.to_string(),
        })?;
        if let Some(level) = open {
            let (len, number) = (self.inferences.len(), self.history.len() as u32);
            let w = self
                .witnesses
                .get_or_insert_with(|| Box::new(Witnesses::new(len)));
            w.push(term, level, number);
        }
        let goal = self.open(id)?;
        let same = if keep {
            vec![instance, x]
        } else {
            vec![instance]
        };
        let made = vec![
            Inference {
                term: Some(term),
                left: goal.left.clone(),
                right: goal.right.clone(),
                rule,
                principal: Some((side, position)),
                premises: vec![InfId::new(1)],
            },
            premise(goal, side, position, &same, &[], |_, _| true),
        ];
        Ok(self.checked(made))
    }

    /// Refuses the instance of `body` by `term` where the arena's terms
    /// would pass the state's bound on terms once it is built, by the
    /// measure a document's reader takes ([`arena_terms`]): each atom of
    /// `body` that holds a bound variable makes at most one atom more,
    /// every bound variable in it written as `term`. So a session writes no
    /// document its reader refuses. Nothing is built.
    fn instance_within(&self, body: NodeId, term: FoTermId) -> Result<(), StepError> {
        let tables = self.formulas.first_order();
        let unfolded = tables.unfolded();
        let witness = unfolded.get(term.index()).copied().unwrap_or(1);
        // Each term written out with its bound variables as the witness;
        // a term comes after its arguments.
        let mut sizes = vec![0u64; tables.terms.len()];
        for (n, t) in tables.terms.iter().enumerate() {
            sizes[n] = match *t {
                FoTerm::Bound(_) => witness,
                FoTerm::Eigen(_) | FoTerm::Meta(_) => 1,
                FoTerm::App(_, args) => tables
                    .list(args)
                    .iter()
                    .fold(1u64, |sum, a| sum.saturating_add(sizes[a.index()])),
            };
        }
        let loose = tables.loose();
        let mut added = 0u64;
        let mut seen = crate::hash::HashSet::default();
        let mut stack = vec![body];
        while let Some(n) = stack.pop() {
            if !seen.insert(n) {
                continue;
            }
            match self.formulas.node(n) {
                Node::Atom(a) => {
                    let Some(&(_, args)) = tables.atoms.get(a as usize) else {
                        continue;
                    };
                    let list = tables.list(args);
                    if list.iter().any(|t| loose[t.index()] > 0) {
                        for t in list {
                            added = added.saturating_add(sizes[t.index()]);
                        }
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => stack.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    stack.push(a);
                    stack.push(b);
                }
            }
        }
        let total = arena_terms(tables).max(witness).saturating_add(added);
        if total > self.terms {
            return Err(StepError::Terms {
                terms: total,
                limit: self.terms,
            });
        }
        Ok(())
    }

    /// Returns the term of the next open witness, `$1` for the first, and
    /// its level for a step on the goal `id`: one more than the largest
    /// eigenvariable in the goal's scope, so that its binding takes none
    /// made after it.
    fn open_witness(&mut self, id: GoalId) -> Result<(FoTermId, u32), StepError> {
        let fault = |reason: &str| StepError::Witness {
            reason: reason.to_owned(),
        };
        let terms = &self.formulas.first_order().terms;
        let level = self
            .scope(InfId::new(id.get()))
            .iter()
            .filter_map(|t| match terms[t.index()] {
                FoTerm::Eigen(e) => Some(e.get()),
                FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
            })
            .max()
            .map_or(Some(0), |e| e.checked_add(1))
            .ok_or_else(|| fault("an eigenvariable of the goal has the last number"))?;
        let k = self.witnesses.as_ref().map_or(0, |w| w.unifier.next_meta());
        if k == u32::MAX {
            return Err(fault(
                "the proof has as many open witnesses as it can number",
            ));
        }
        Ok((self.formulas.interner().term(FoTerm::Meta(k)), level))
    }

    /// Returns the term of an eigenvariable that no inference of the
    /// proof holds: one numbered past every eigenvariable its inferences'
    /// formulas and terms hold, so that taking back a step and taking it
    /// again names the same one.
    fn fresh_eigenvariable(&mut self) -> Result<FoTermId, StepError> {
        let roots = self
            .inferences
            .iter()
            .flat_map(|i| i.left.iter().chain(&i.right));
        let terms = self.inferences.iter().filter_map(|i| i.term);
        let next = self
            .formulas
            .largest_eigenvariable(roots.copied(), terms)
            .map_or(Some(0), |k| k.checked_add(1))
            .ok_or_else(|| StepError::Witness {
                reason: "the proof holds an eigenvariable of the last number, so none is fresh"
                    .to_owned(),
            })?;
        Ok(self
            .formulas
            .interner()
            .term(FoTerm::Eigen(Eigen::new(next))))
    }

    /// Returns the witness `t` for a step on the goal `id` where it is a
    /// term of the goal: a term of the arena without a bound variable or
    /// an open witness, whose eigenvariables are in the goal's scope
    /// ([`scope`](Self::scope)).
    fn held(&self, id: GoalId, t: FoTermId) -> Result<FoTermId, StepError> {
        let fault = |reason: &str| StepError::Witness {
            reason: reason.to_owned(),
        };
        let terms = &self.formulas.first_order().terms;
        if t.index() >= terms.len() {
            return Err(fault("it is no term of the proof"));
        }
        if !self.formulas.is_closed(t) || self.formulas.holds_meta(t) {
            return Err(fault("it holds a variable"));
        }
        let scope = self.scope(InfId::new(id.get()));
        let mut seen = crate::hash::HashSet::default();
        let mut stack = vec![t];
        while let Some(u) = stack.pop() {
            if !seen.insert(u) {
                continue;
            }
            match terms[u.index()] {
                FoTerm::App(_, args) => {
                    stack.extend_from_slice(self.formulas.first_order().list(args))
                }
                FoTerm::Eigen(_) if !scope.contains(&u) => {
                    return Err(fault(
                        "it holds an eigenvariable that no ∀R or ∃L below the goal introduces",
                    ));
                }
                FoTerm::Eigen(_) | FoTerm::Bound(_) | FoTerm::Meta(_) => {}
            }
        }
        Ok(t)
    }

    /// Returns the eigenvariables in the scope of the inference `id`: those
    /// the `∀R` and `∃L` inferences below it introduce, on its path from
    /// the root.
    fn scope(&self, id: InfId) -> Vec<FoTermId> {
        let mut parent = vec![None; self.inferences.len()];
        for (n, inference) in self.inferences.iter().enumerate() {
            for p in &inference.premises {
                parent[p.index()] = Some(n);
            }
        }
        let mut scope = Vec::new();
        let mut at = parent[id.index()];
        while let Some(n) = at {
            scope.extend(self.inferences[n].eigenvariable());
            at = parent[n];
        }
        scope
    }

    /// Reads a witness for a step on the goal `goal` from text: a term of
    /// the sequent's function symbols, written `f(t, …)` or by its name
    /// for a constant, and of the eigenvariables in the goal's scope by
    /// the names the proof prints them with (`a`, `b`, …, past the
    /// sequent's symbols). Returns its term, made in the proof's formulas
    /// where it is new, for [`Step::with_witness`].
    ///
    /// # Errors
    ///
    /// [`StepError::NoGoal`] for a goal that is not open, and
    /// [`StepError::Witness`] for text that is not such a term: an unknown
    /// name, a predicate, a symbol given the wrong number of arguments, an
    /// eigenvariable outside the goal's scope, or a stray bracket or comma.
    pub fn witness(&mut self, goal: GoalId, text: &str) -> Result<FoTermId, StepError> {
        self.open(goal)?;
        let fault = |reason: String| StepError::Witness { reason };
        let eigens = self.formulas.eigen_names();
        let scope = self.scope(InfId::new(goal.get()));
        // The tokens: names, brackets and commas.
        let mut tokens = Vec::new();
        let mut name = String::new();
        for c in text.chars().chain([' ']) {
            if c.is_whitespace() || matches!(c, '(' | ')' | ',') {
                if !name.is_empty() {
                    tokens.push(std::mem::take(&mut name));
                }
                if !c.is_whitespace() {
                    tokens.push(c.to_string());
                }
            } else {
                name.push(c);
            }
        }
        // A term of text shares nothing, so it is as long as its names; one
        // past the bound on terms is refused before any of it is made.
        let names = tokens
            .iter()
            .filter(|t| !matches!(t.as_str(), "(" | ")" | ","))
            .count() as u64;
        if names > self.terms {
            return Err(fault(format!(
                "it has {names} symbols, more than the bound of {} on terms",
                self.terms
            )));
        }
        // The open applications wait on a stack with their arguments.
        let mut open: Vec<(String, Vec<FoTermId>)> = Vec::new();
        let mut done: Option<FoTermId> = None;
        let mut tokens = tokens.into_iter().peekable();
        while let Some(token) = tokens.next() {
            match token.as_str() {
                "(" if done.is_none() => {
                    return Err(fault("a bracket follows no function symbol".into()));
                }
                "(" | ")" | "," if done.is_none() => {
                    return Err(fault(format!("a term is missing before {token:?}")));
                }
                "," | ")" => {
                    let (Some((_, args)), Some(arg)) = (open.last_mut(), done.take()) else {
                        return Err(fault(format!("{token:?} closes no application")));
                    };
                    args.push(arg);
                    if token == ")"
                        && let Some((f, args)) = open.pop()
                    {
                        done = Some(self.applied(&f, &args).map_err(fault)?);
                    }
                }
                _ if done.is_some() => {
                    return Err(fault(format!("{token:?} follows a term without a comma")));
                }
                word => {
                    let applied = tokens.peek().is_some_and(|t| t == "(");
                    if let Some(&(k, _)) = eigens.iter().find(|(_, n)| n == word) {
                        if applied {
                            return Err(fault(format!(
                                "the eigenvariable {word} takes no arguments"
                            )));
                        }
                        let term = self.formulas.interner().term(FoTerm::Eigen(Eigen::new(k)));
                        if !scope.contains(&term) {
                            return Err(fault(format!(
                                "the eigenvariable {word} is not in the goal's scope: no ∀R or \
                                 ∃L below the goal introduces it"
                            )));
                        }
                        done = Some(term);
                    } else if applied {
                        tokens.next();
                        open.push((word.to_owned(), Vec::new()));
                    } else {
                        done = Some(self.applied(word, &[]).map_err(fault)?);
                    }
                }
            }
        }
        if !open.is_empty() {
            return Err(fault("a bracket is not closed".into()));
        }
        done.ok_or_else(|| fault("there is no term".into()))
    }

    /// Returns the term of the sequent's function symbol `name` applied to
    /// `args`, or why there is none.
    fn applied(&mut self, name: &str, args: &[FoTermId]) -> Result<FoTermId, String> {
        let tables = self.formulas.first_order();
        let Some(f) = tables.symbols.iter().position(|(s, _)| s == name) else {
            return Err(format!("{name} is not a symbol of the sequent"));
        };
        if tables.atoms.iter().any(|&(p, _)| p.index() == f) {
            return Err(format!("{name} is a predicate, not a term"));
        }
        let arity = tables.symbols[f].1 as usize;
        if args.len() != arity {
            return Err(format!(
                "{name} takes {arity} arguments, not {}",
                args.len()
            ));
        }
        let symbol = crate::first_order::Symbol::new(f as u32);
        let interner = self.formulas.interner();
        let args = interner.list(args);
        Ok(interner.term(FoTerm::App(symbol, args)))
    }

    /// Returns the inferences a step makes, the goal's own first with its
    /// rule, the others after it, each premise of one of them indexing
    /// this list; the premises left open have [`Rule::Open`].
    fn expand(&self, id: GoalId, step: &Step) -> Result<Vec<Inference>, StepError> {
        let goal = self.open(id)?;
        let (side, position, rule) = (step.side, step.position, step.rule);
        let x = formula_at(goal, side, position)?;
        let made = if rule == Rule::Axiom {
            if step.context != Context::Shared {
                return Err(StepError::NoSplit { rule });
            }
            let partner = goal
                .side(side.flip())
                .iter()
                .position(|&y| y == x)
                .ok_or(StepError::NoPartner { side, position })?;
            // One weakening per formula, each holding what is left of the
            // sequent.
            let len = goal.left.len() + goal.right.len();
            self.run_within(len, len)?;
            axiom(goal, side, position, partner)
        } else {
            let shape = self.shape(rule, side, position, x)?;
            if step.context != Context::Shared && !matches!(shape, Shape::Binary(_)) {
                return Err(StepError::NoSplit { rule });
            }
            // LJ's `→L` and `¬L` give their first premise a formula right
            // of `⊢` of its own: shared, the hypotheses go to both premises
            // through contractions below, the goal's formula right of `⊢`
            // to the second.
            if self.logic != Logic::Classical
                && step.context == Context::Shared
                && let Shape::Binary([first, second]) = &shape
                && side == Side::Left
                && !first.1.is_empty()
            {
                // A contraction per hypothesis, each sequent longer by one.
                let len = 2 * goal.left.len() + goal.right.len() + first.1.len();
                self.run_within(goal.left.len() + 2, len)?;
                return Ok(self.checked(contracted(goal, position, rule, first, second)));
            }
            let premises = match &shape {
                Shape::Instance => unreachable!("apply takes the quantifier rules apart"),
                Shape::Leaf => vec![],
                Shape::Unary(same, other) => {
                    vec![premise(goal, side, position, same, other, |_, _| true)]
                }
                Shape::Binary([first, second]) => {
                    let to_first = self.split(goal, side, position, &step.context)?;
                    let goes = |s: Side, i: usize| match &to_first {
                        None => true,
                        Some((left, right)) => match s {
                            Side::Left => left[i],
                            Side::Right => right[i],
                        },
                    };
                    let stays = |s: Side, i: usize| to_first.is_none() || !goes(s, i);
                    vec![
                        premise(goal, side, position, &first.0, &first.1, goes),
                        premise(goal, side, position, &second.0, &second.1, stays),
                    ]
                }
            };
            if self.logic != Logic::Classical
                && let Some(p) = premises.iter().find(|p| p.right.len() != 1)
            {
                return Err(StepError::Succedents {
                    count: p.right.len(),
                });
            }
            let mut made = vec![Inference {
                term: None,
                left: goal.left.clone(),
                right: goal.right.clone(),
                rule,
                principal: Some((side, position)),
                premises: (1..=premises.len()).map(|k| InfId::new(k as u32)).collect(),
            }];
            made.extend(premises);
            made
        };
        Ok(self.checked(made))
    }

    /// Refuses a run of `inferences` inferences, each sequent at most
    /// `len` formulas, past the state's bound. It uses the size estimate's
    /// measure, counting every formula as three characters (at least one,
    /// and two for its separator). An axiom's weakenings and a shared
    /// rule's contractions grow with the square of the goal, so they are
    /// checked before they are made.
    fn run_within(&self, inferences: usize, len: usize) -> Result<(), StepError> {
        let (inferences, len) = (inferences as u64, len as u64);
        let bytes = inferences.saturating_mul(
            Size::BYTES_PER_INFERENCE.saturating_add(
                len.saturating_mul(3)
                    .saturating_mul(Size::BYTES_PER_CHARACTER),
            ),
        );
        match self.limit {
            Some(limit) if bytes > limit => Err(StepError::Size {
                estimate_bytes: bytes,
                limit_bytes: limit,
            }),
            _ => Ok(()),
        }
    }

    /// Returns the inferences a step made, each checked by the checker's
    /// own rules, inference by inference: what a step makes is what a
    /// finished derivation's check reads.
    fn checked(&self, made: Vec<Inference>) -> Vec<Inference> {
        for inference in made.iter().filter(|i| i.rule != Rule::Open) {
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|p| &made[p.index()])
                .collect();
            if check_inference(&self.formulas, self.logic, inference, &premises).is_err() {
                unreachable!("a step made an inference the checker refuses: {inference:?}");
            }
        }
        made
    }

    /// Returns, for a split, which positions of each side go to the first
    /// premise; `None` for a shared context.
    #[expect(
        clippy::type_complexity,
        reason = "one flag per position of each side, used once"
    )]
    fn split(
        &self,
        goal: &Inference,
        side: Side,
        position: usize,
        context: &Context,
    ) -> Result<Option<(Vec<bool>, Vec<bool>)>, StepError> {
        let Context::Split { left, right } = context else {
            return Ok(None);
        };
        let mut flags = (vec![false; goal.left.len()], vec![false; goal.right.len()]);
        for (s, positions) in [(Side::Left, left), (Side::Right, right)] {
            let list = match s {
                Side::Left => &mut flags.0,
                Side::Right => &mut flags.1,
            };
            for &p in positions {
                let principal = s == side && p == position;
                match list.get_mut(p) {
                    Some(flag) if !*flag && !principal => *flag = true,
                    _ => {
                        return Err(StepError::Split {
                            side: s,
                            position: p,
                        });
                    }
                }
            }
        }
        Ok(Some(flags))
    }

    /// Fills the goal with the first of the inferences made and appends
    /// the others, every premise after its conclusion, records the step
    /// and returns the goals it opened.
    fn record(&mut self, goal: GoalId, made: Vec<Inference>) -> Vec<GoalId> {
        let len = made.len();
        self.record_made(goal, made, vec![None; len])
    }

    /// Records a step as [`record`](Self::record) does, `raw` saying for
    /// each inference made what it was made as where a binding of the step
    /// changed it.
    fn record_made(
        &mut self,
        goal: GoalId,
        mut made: Vec<Inference>,
        raw: Vec<Option<Made>>,
    ) -> Vec<GoalId> {
        if let Some(w) = self.witnesses.as_deref_mut() {
            let mut raw = raw.into_iter();
            // The goal keeps what its sides were made as; its rule's term
            // is the step's.
            let first = raw.next().flatten();
            if let Some(kept) = &mut w.raw[goal.index()] {
                kept.term = first.map_or(made.first().and_then(|i| i.term), |m| m.term);
            }
            w.raw.extend(raw);
        }
        let base = self.inferences.len();
        // Inference `k` of the list is the goal for `k = 0`, else the one
        // appended at `base + k - 1`; fewer inferences than a `u32` counts.
        let global = |k: InfId| match k.index() {
            0 => InfId::new(goal.get()),
            k => InfId::new((base + k - 1) as u32),
        };
        for inference in &mut made {
            for p in &mut inference.premises {
                *p = global(*p);
            }
        }
        let opened = made
            .iter()
            .enumerate()
            .filter(|(_, i)| i.rule == Rule::Open)
            .map(|(k, _)| GoalId::new(global(InfId::new(k as u32)).get()))
            .collect();
        let mut made = made.into_iter();
        if let Some(first) = made.next() {
            self.inferences[goal.index()] = first;
        }
        self.inferences.extend(made);
        self.history.push(InfId::new(goal.get()));
        if let Some(w) = self.witnesses.as_deref_mut() {
            w.registered.resize(self.inferences.len(), Vec::new());
            for i in std::iter::once(goal.index()).chain(base..self.inferences.len()) {
                let inference = &self.inferences[i];
                let metas = self.formulas.metas_in(
                    inference.left.iter().chain(&inference.right).copied(),
                    inference.term.into_iter(),
                );
                w.register(i, &metas);
            }
        }
        opened
    }

    /// Retracts the last step, a rule applied or a goal closed by the
    /// search, and returns the goal it reopened, or `None` when no step is
    /// left.
    pub fn undo(&mut self) -> Option<GoalId> {
        let goal = self.history.pop()?;
        // The step's inferences are the suffix of the arena.
        if let Some(first) = self.subtree(goal).into_iter().min() {
            self.inferences.truncate(first.index());
        }
        let inference = &mut self.inferences[goal.index()];
        inference.rule = Rule::Open;
        inference.principal = None;
        inference.premises.clear();
        inference.term = None;
        if let Some(w) = self.witnesses.as_deref_mut() {
            w.forget_from(self.inferences.len());
            if let Some(kept) = &mut w.raw[goal.index()] {
                kept.term = None;
            }
            // The step's number, under which its bindings were trailed.
            let number = self.history.len() as u32;
            let was: Vec<bool> = w
                .unifier
                .metas()
                .iter()
                .map(|m| m.bound.is_some())
                .collect();
            w.unifier.undo(number);
            while w.made.last().is_some_and(|&s| s >= number) {
                w.made.pop();
            }
            w.unifier.truncate_metas(w.made.len());
            w.holders.truncate(w.made.len());
            // The witnesses the step had bound, and the inferences that
            // showed them before.
            let unbound: Vec<u32> = (0..w.unifier.next_meta())
                .filter(|&k| was[k as usize] && w.unifier.metas()[k as usize].bound.is_none())
                .collect();
            let changed = w.holding(unbound.into_iter());
            if !changed.is_empty() {
                self.reshow(&changed);
            }
            if self.witnesses.as_ref().is_some_and(|w| w.made.is_empty()) {
                self.witnesses = None;
            }
        }
        Some(GoalId::new(goal.get()))
    }

    /// Shows the bindings in force, from what it was made as, in every
    /// inference of `changed` that a binding changed.
    fn reshow(&mut self, changed: &[usize]) {
        let Some(w) = self.witnesses.as_deref_mut() else {
            return;
        };
        let bindings = w.unifier.metas().iter().map(|m| m.bound).collect();
        let mut resolution = Resolution::new(bindings);
        for &i in changed {
            let (Some(inference), Some(raw)) = (self.inferences.get_mut(i), w.raw.get_mut(i))
            else {
                continue;
            };
            let Some(made) = raw else { continue };
            let mut again = |list: &[NodeId]| {
                list.iter()
                    .map(|&n| resolution.node(&mut self.formulas, n))
                    .collect::<Result<Vec<_>, _>>()
                    .expect("a binding taken back shows nodes the arena held before")
            };
            inference.left = again(&made.left);
            inference.right = again(&made.right);
            inference.term = made.term.map(|t| resolution.term(&mut self.formulas, t));
            if made.shows(inference) {
                *raw = None;
            }
        }
    }

    /// Returns the inferences above `id`, excluding it, in no particular
    /// order.
    fn subtree(&self, id: InfId) -> Vec<InfId> {
        crate::goal_tree::subtree(|i| &self.inferences[i.index()].premises[..], id)
    }

    /// Decides the open goal as a sequent of its own in the state's logic
    /// and returns the outcome. The search goes through the embedding by
    /// `translation` (`None` for the logic's default), with the search
    /// options, within `limits` and until `stop` fires. When the goal is
    /// valid, the derivation found is grafted onto it as one step, which
    /// [`undo`](Self::undo) takes back whole. A goal the search refutes or
    /// cannot decide stays open, and the outcome says why
    /// ([`Verdict::NotValid`], or [`Verdict::Unknown`] with its reason).
    ///
    /// A derivation larger than `limits.derivation_bytes` by the size
    /// estimate is not grafted: the goal stays open, [`Closed::grafted`]
    /// says why, and the outcome keeps the derivation.
    ///
    /// # Errors
    ///
    /// [`Error::OrdinaryStep`] for a goal that is not open, the errors of
    /// [`decide_within`] (a translation that does not decide the logic, the
    /// refusal of a bound or the stop), the goal then still open, and
    /// [`Error::GoalMismatch`] if the derivation found concludes a sequent
    /// other than the goal.
    pub fn close(
        &mut self,
        goal: GoalId,
        translation: Option<Translation>,
        search: &search::Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Closed, Error> {
        let outcome = self.decide(goal, translation, search, limits, stop)?;
        let grafted = match &outcome.verdict {
            Verdict::Valid(evidence) => match self.graft(goal, evidence.derivation()?, limits) {
                Ok(()) => Ok(()),
                Err(Error::Refused(refusal)) => Err(refusal),
                Err(error) => return Err(error),
            },
            Verdict::NotValid | Verdict::Unknown(_) => Ok(()),
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
        let outcome = self.decide(goal, translation, search, limits, stop)?;
        self.open(goal)?;
        let steps = match &outcome.verdict {
            Verdict::Valid(evidence) => self.first_steps(goal, evidence.derivation()?),
            Verdict::NotValid | Verdict::Unknown(_) => Vec::new(),
        };
        Ok(Hint { outcome, steps })
    }

    /// Decides the open goal as a sequent of its own.
    fn decide(
        &self,
        goal: GoalId,
        translation: Option<Translation>,
        search: &search::Options,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Outcome, Error> {
        let goal = self.open(goal)?;
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
        decide_within(&sequent, &options, search, limits, stop)
    }

    /// Returns the id in `formulas` of every formula of `other`, adding to
    /// `formulas` those it lacks.
    fn ids_of(formulas: &mut Formulas, other: &Formulas) -> Result<Vec<NodeId>, Error> {
        let mut ids: Vec<NodeId> = Vec::with_capacity(other.len());
        // Every subformula precedes the formulas built from it.
        for &node in &other.nodes {
            let at = |k: NodeId| ids[k.index()];
            let id = match node {
                Node::Atom(a) => formulas.atom(other.atom_name(a))?,
                Node::True | Node::False => formulas.add(node)?,
                Node::Not(a) => formulas.add(Node::Not(at(a)))?,
                Node::And(a, b) => formulas.add(Node::And(at(a), at(b)))?,
                Node::Or(a, b) => formulas.add(Node::Or(at(a), at(b)))?,
                Node::Implies(a, b) => formulas.add(Node::Implies(at(a), at(b)))?,
                Node::Iff(a, b) => formulas.add(Node::Iff(at(a), at(b)))?,
                Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            };
            ids.push(id);
        }
        Ok(ids)
    }

    /// Grafts a derivation of the open goal onto it as one step, its root
    /// in the goal's place and the rest appended in reverse, so that every
    /// premise keeps a larger index than its conclusion; refused past
    /// `limits.derivation_bytes` by the size estimate's measure.
    fn graft(&mut self, goal: GoalId, found: &Derivation, limits: &Limits) -> Result<(), Error> {
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
        let goal_inference = &self.inferences[at.index()];
        if !same(&top.left, &goal_inference.left) || !same(&top.right, &goal_inference.right) {
            return Err(Error::GoalMismatch);
        }
        top.principal = top.principal.map(|(side, position)| {
            let id = top.side(side)[position];
            let list = goal_inference.side(side);
            (side, list.iter().position(|&y| y == id).unwrap_or(position))
        });
        top.left.clone_from(&goal_inference.left);
        top.right.clone_from(&goal_inference.right);
        self.inferences[at.index()] = top;
        self.inferences.extend(grafted.into_iter().rev());
        self.history.push(at);
        Ok(())
    }

    /// Returns the steps of a derivation of the open goal from its root
    /// up to and including the first rule that is not structural, each
    /// applied in a copy of the state to find the goal the next one acts
    /// on. Each turn goes one premise up the derivation, so there are no
    /// more turns than it has inferences; a step the copy refuses (a
    /// bound) ends the list there.
    fn first_steps(&self, goal: GoalId, found: &Derivation) -> Vec<Step> {
        let mut state = self.clone();
        let (mut goal, mut at) = (goal, found.root());
        let mut steps = Vec::new();
        while let Some(step) = state
            .open(goal)
            .ok()
            .and_then(|inference| state.step_at(inference, found, at))
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
    /// (whose conclusion is the goal as multisets) as a step on the goal,
    /// its formula found by what it is and, for two premises, the context
    /// as the derivation gave it to them.
    fn step_at(&self, goal: &Inference, found: &Derivation, at: InfId) -> Option<Step> {
        let mut formulas = self.formulas.clone();
        let ids = Self::ids_of(&mut formulas, found.formulas()).ok()?;
        let map = |list: &[NodeId]| list.iter().map(|k| ids[k.index()]).collect::<Vec<_>>();
        let root = found.inference(at);
        let Some((side, at)) = root.principal else {
            // An axiom concludes exactly the goal `A ⊢ A`.
            return Some(Step::new(Side::Right, 0, root.rule));
        };
        let x = ids[root.side(side)[at].index()];
        let position = goal.side(side).iter().position(|&y| y == x)?;
        let step = Step::new(side, position, root.rule);
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
        if stop(Progress::new(Phase::View, 0, 0)) {
            return Err(Error::Refused(Refusal::Stopped { phase: Phase::View }));
        }
        let order = self.postorder();
        let mut new_id = vec![InfId::new(u32::MAX); self.inferences.len()];
        for (i, id) in order.iter().enumerate() {
            // As many as the state's inferences, which a `u32` counts.
            new_id[id.index()] = InfId::new(i as u32);
        }
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
        let full = || {
            Error::Refused(Refusal::Index {
                what: crate::limits::Space::Term,
                count: u64::from(u32::MAX) + 1,
                most: u64::from(u32::MAX),
            })
        };
        let base = terms
            .iter()
            .filter_map(|t| match *t {
                FoTerm::Eigen(e) => Some(e.get()),
                FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
            })
            .max()
            .map_or(Some(0), |e| e.checked_add(1))
            .ok_or_else(full)?;
        // A fresh eigenvariable is no term of the arena, so no two terms
        // become equal.
        for term in &mut formulas.interner().tables.terms {
            if let FoTerm::Meta(k) = *term {
                let number = base.checked_add(k).ok_or_else(full)?;
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
                let mut new_id = vec![InfId::new(0); self.inferences.len()];
                for (i, id) in order.iter().enumerate() {
                    new_id[id.index()] = InfId::new(i as u32);
                }
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
        derivation.check(limits, stop)?;
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

/// Returns the open premise that keeps the goal's context formulas for
/// which `keep` holds, in their order, the principal formula's place on
/// its side taken by `same` and `other` joining the other side's end.
fn premise(
    goal: &Inference,
    side: Side,
    position: usize,
    same: &[NodeId],
    other: &[NodeId],
    keep: impl Fn(Side, usize) -> bool,
) -> Inference {
    let mut sides = [Vec::new(), Vec::new()];
    for (k, s) in [Side::Left, Side::Right].into_iter().enumerate() {
        for (i, &y) in goal.side(s).iter().enumerate() {
            if s == side && i == position {
                sides[k].extend_from_slice(same);
            } else if keep(s, i) {
                sides[k].push(y);
            }
        }
    }
    let [mut left, mut right] = sides;
    match side {
        Side::Left => right.extend_from_slice(other),
        Side::Right => left.extend_from_slice(other),
    }
    open(left, right)
}

/// Returns the inferences of a rule with two premises on the hypothesis
/// at `position` whose first premise has its own formula right of `⊢`
/// (LJ's `→L` and `¬L`), the context shared as the checker can read it:
/// a contraction of every other hypothesis, each copy right after its
/// formula, then the rule with the formulas' first copies going to the
/// first premise and the second copies and the goal's formulas right of
/// `⊢` to the second.
fn contracted(
    goal: &Inference,
    position: usize,
    rule: Rule,
    first: &(Vec<NodeId>, Vec<NodeId>),
    second: &(Vec<NodeId>, Vec<NodeId>),
) -> Vec<Inference> {
    let mut made = Vec::with_capacity(goal.left.len() + 2);
    let mut left = goal.left.clone();
    // Each original hypothesis `i` stands at `i + k` once `k` copies were
    // put before it; `copies[j]` marks the copies of the final sequent.
    let mut copies = vec![false; left.len()];
    let mut k = 0;
    for i in 0..goal.left.len() {
        if i == position {
            continue;
        }
        let at = i + k;
        // Each inference's premise is the next one made; fewer than a
        // `u32` counts, as the goal's formulas are.
        made.push(Inference {
            term: None,
            left: left.clone(),
            right: goal.right.clone(),
            rule: Rule::ContractLeft,
            principal: Some((Side::Left, at)),
            premises: vec![InfId::new(made.len() as u32 + 1)],
        });
        left.insert(at + 1, left[at]);
        copies.insert(at + 1, true);
        k += 1;
    }
    // The rule's formula is an original, never copied, in its place.
    let principal = (0..left.len())
        .filter(|&j| !copies[j])
        .nth(position)
        .expect("the rule's formula is a hypothesis of the goal");
    let contracted = open(left, goal.right.clone());
    let n = made.len() as u32;
    made.push(Inference {
        term: None,
        left: contracted.left.clone(),
        right: contracted.right.clone(),
        rule,
        principal: Some((Side::Left, principal)),
        premises: vec![InfId::new(n + 1), InfId::new(n + 2)],
    });
    let to_first = |s: Side, j: usize| s == Side::Left && !copies[j];
    let to_second = |s: Side, j: usize| s == Side::Right || copies[j];
    made.push(premise(
        &contracted,
        Side::Left,
        principal,
        &first.0,
        &first.1,
        to_first,
    ));
    made.push(premise(
        &contracted,
        Side::Left,
        principal,
        &second.0,
        &second.1,
        to_second,
    ));
    made
}

/// Returns the inferences that close a goal by the axiom on the formula
/// at `position` of `side` and its partner at `partner` on the other side:
/// a weakening of every other formula, the last formula left of `⊢` first,
/// then right of it, and the axiom above them.
fn axiom(goal: &Inference, side: Side, position: usize, partner: usize) -> Vec<Inference> {
    let (mut keep_left, mut keep_right) = match side {
        Side::Left => (position, partner),
        Side::Right => (partner, position),
    };
    let (mut left, mut right) = (goal.left.clone(), goal.right.clone());
    let mut made = Vec::with_capacity(left.len() + right.len() - 1);
    // The formulas left of `⊢` from the last, then those right of it.
    let mut weaken =
        |rule: Rule, s: Side, at: usize, left: &mut Vec<NodeId>, right: &mut Vec<NodeId>| {
            // Each inference's premise is the next one made; fewer than a
            // `u32` counts, as the goal's formulas are.
            let next = InfId::new(made.len() as u32 + 1);
            made.push(Inference {
                term: None,
                left: left.clone(),
                right: right.clone(),
                rule,
                principal: Some((s, at)),
                premises: vec![next],
            });
            match s {
                Side::Left => left.remove(at),
                Side::Right => right.remove(at),
            };
        };
    for at in (0..left.len()).rev() {
        if at != keep_left {
            weaken(Rule::WeakenLeft, Side::Left, at, &mut left, &mut right);
            keep_left -= usize::from(at < keep_left);
        }
    }
    for at in (0..right.len()).rev() {
        if at != keep_right {
            weaken(Rule::WeakenRight, Side::Right, at, &mut left, &mut right);
            keep_right -= usize::from(at < keep_right);
        }
    }
    made.push(Inference {
        term: None,
        left,
        right,
        rule: Rule::Axiom,
        principal: None,
        premises: Vec::new(),
    });
    made
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

/// What a state holds, as its JSON form writes and reads it.
#[cfg(feature = "serialize")]
pub(crate) struct Parts {
    /// The logic.
    pub(crate) logic: Logic,
    /// The formulas.
    pub(crate) formulas: Formulas,
    /// The inferences, the root first.
    pub(crate) inferences: Vec<Inference>,
    /// The steps, each the inference it closed.
    pub(crate) history: Vec<InfId>,
    /// The bindings of open witnesses in force, in the order made: the
    /// witness, its term, the step whose axiom bound it.
    pub(crate) bindings: Vec<(u32, FoTermId, u32)>,
}

#[cfg(feature = "serialize")]
impl Interactive {
    /// Returns what the state holds, for its JSON form: the logic, the
    /// formulas, the inferences as their steps made them, before any
    /// binding of an open witness changed them, the history and the
    /// bindings in force, each the witness, its term and the step.
    #[expect(clippy::type_complexity, reason = "the parts of one form, used once")]
    pub(crate) fn parts(
        &self,
    ) -> (
        Logic,
        &Formulas,
        std::borrow::Cow<'_, [Inference]>,
        &[InfId],
        Vec<(u32, FoTermId, u32)>,
    ) {
        let Some(w) = self.witnesses.as_deref() else {
            return (
                self.logic,
                &self.formulas,
                std::borrow::Cow::Borrowed(&self.inferences),
                &self.history,
                Vec::new(),
            );
        };
        let mut inferences = self.inferences.clone();
        for (inference, raw) in inferences.iter_mut().zip(&w.raw) {
            if let Some(made) = raw {
                inference.left.clone_from(&made.left);
                inference.right.clone_from(&made.right);
                inference.term = made.term;
            }
        }
        (
            self.logic,
            &self.formulas,
            std::borrow::Cow::Owned(inferences),
            &self.history,
            w.unifier.bindings().collect(),
        )
    }

    /// Returns the state the parts describe, checked as a state built
    /// through [`apply`](Self::apply) is: the root first, every other
    /// inference the premise of exactly one before it, an open goal with
    /// neither rule nor premises and in intuitionistic and minimal logic
    /// one formula right of `⊢`, every other inference passing the
    /// checker's rules with its premises, the sequent within
    /// `limits.occurrences`, and the history distinct closed inferences
    /// each of whose steps added the suffix of the arena it then had.
    pub(crate) fn from_parts(parts: Parts, limits: &Limits) -> Result<Self, Error> {
        let Parts {
            logic,
            mut formulas,
            mut inferences,
            history,
            bindings,
        } = parts;
        let inconsistent = |reason| Error::InconsistentSession { reason };
        let root = inferences
            .first()
            .ok_or(inconsistent("the session has no inference"))?;
        bound(&formulas, &root.left, &root.right, limits)?;
        // Open witnesses make a session whose inferences are read as their
        // steps made them, each checked under the bindings in force then.
        if formulas.is_first_order()
            && formulas.holds_open_witness(root.left.iter().chain(&root.right).copied())
        {
            return Err(inconsistent("the sequent holds an open witness"));
        }
        let open = !bindings.is_empty()
            || inferences
                .iter()
                .any(|i| i.term.is_some_and(|t| formulas.holds_meta(t)))
            || formulas.is_first_order()
                && formulas.holds_open_witness(
                    inferences
                        .iter()
                        .flat_map(|i| i.left.iter().chain(&i.right))
                        .copied(),
                );
        let falsity = Extra::add(&mut formulas)?.falsity;
        let len = inferences.len();
        let mut parents = vec![0u8; len];
        for (n, inference) in inferences.iter().enumerate() {
            for id in inference.left.iter().chain(&inference.right) {
                formulas.holds(*id)?;
            }
            if let Some((side, at)) = inference.principal
                && at >= inference.side(side).len()
            {
                return Err(inconsistent(
                    "a principal position lies outside its sequent",
                ));
            }
            for p in &inference.premises {
                if p.index() <= n || p.index() >= len {
                    return Err(inconsistent("a premise does not follow its conclusion"));
                }
                parents[p.index()] = parents[p.index()].saturating_add(1);
            }
            if inference.rule == Rule::Open {
                let right = inference.right.len();
                if inference.principal.is_some()
                    || !inference.premises.is_empty()
                    || inference.term.is_some()
                {
                    return Err(inconsistent(
                        "an open goal has a principal, premises or a term",
                    ));
                }
                if logic != Logic::Classical && right != 1 {
                    return Err(inconsistent(
                        "an open goal of LJ must have exactly one formula right of ⊢",
                    ));
                }
                continue;
            }
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|p| &inferences[p.index()])
                .collect();
            if !open && check_inference(&formulas, logic, inference, &premises).is_err() {
                return Err(inconsistent("an inference breaks the rule it names"));
            }
        }
        if parents[0] != 0 || parents[1..].iter().any(|&n| n != 1) {
            return Err(inconsistent(
                "an inference is not the premise of exactly one",
            ));
        }
        // The history from the last step back: each step's goal closed,
        // named once, and what it added the suffix of the arena it left.
        let added = crate::goal_tree::added(
            &history,
            len,
            |i| &inferences[i.index()].premises[..],
            |i| inferences[i.index()].rule != Rule::Open,
        )
        .map_err(|fault| {
            inconsistent(match fault {
                crate::goal_tree::Fault::NotClosed => {
                    "a step names no inference it could have closed"
                }
                crate::goal_tree::Fault::NotSuffix => "a step did not add the suffix of the arena",
            })
        })?;
        // Its undo reopens each step's inference as a goal, which in LJ has
        // one formula right of `⊢`.
        if logic != Logic::Classical
            && history
                .iter()
                .any(|g| inferences[g.index()].right.len() != 1)
        {
            return Err(inconsistent(
                "a step of LJ closed a goal that has no formula or several right of ⊢",
            ));
        }
        let witnesses = if open {
            let read = Replay {
                logic,
                inferences: &inferences,
                history: &history,
                added: &added,
                most: limits.terms.unwrap_or(u64::MAX),
            };
            let (w, shown) = read.run(&mut formulas, &bindings)?;
            inferences = shown;
            Some(Box::new(w))
        } else {
            None
        };
        Ok(Self {
            logic,
            formulas,
            falsity,
            inferences,
            history,
            limit: limits.derivation_bytes,
            witnesses,
            terms: limits.terms.unwrap_or(u64::MAX),
        })
    }
}

/// A session document with open witnesses as its reader replays it: the
/// inferences as their steps made them, the steps and what each added.
#[cfg(feature = "serialize")]
struct Replay<'a> {
    /// The logic.
    logic: Logic,
    /// The inferences as made, the root first.
    inferences: &'a [Inference],
    /// The steps, each the inference it closed.
    history: &'a [InfId],
    /// The inferences each step added, a range per step.
    added: &'a [(usize, usize)],
    /// The bound on the symbols of a term an open witness stands for.
    most: u64,
}

#[cfg(feature = "serialize")]
impl Replay<'_> {
    /// Replays the steps in order, as a session took them: each step's
    /// open witness made at the level its scope gives, each axiom that
    /// unifies its formulas binding them as the session binds them, and
    /// every inference a step closed checked by the checker's rules under
    /// the bindings in force after that step, holding no witness made
    /// later. Then every inference is checked again under the bindings in
    /// force at the end, which can only bring eigenvariables in, and the
    /// bindings found must be the document's. Returns the witnesses and
    /// the inferences as they show the bindings.
    fn run(
        &self,
        formulas: &mut Formulas,
        bindings: &[(u32, FoTermId, u32)],
    ) -> Result<(Witnesses, Vec<Inference>), Error> {
        let bad = |reason| Error::InconsistentSession { reason };
        let inferences = self.inferences;
        let len = inferences.len();
        // The step that closed each inference: its goal, or one it added
        // that no later step closed.
        let mut closer: Vec<Option<usize>> = vec![None; len];
        for (s, (g, &(start, end))) in self.history.iter().zip(self.added).enumerate() {
            closer[start..end].fill(Some(s));
            closer[g.index()] = Some(s);
        }
        // Per step, the inferences it closed; those no step closed (a reader
        // without witnesses takes them) come first, before any witness.
        let mut by_step: Vec<Vec<usize>> = vec![Vec::new(); self.history.len() + 1];
        for (i, inference) in inferences.iter().enumerate() {
            if inference.rule != Rule::Open {
                by_step[closer[i].map_or(0, |s| s + 1)].push(i);
            }
        }
        let tree = Tree::of(formulas, inferences);
        let mut w = Witnesses::new(len);
        for (at, closed) in by_step.iter().enumerate() {
            let Some(s) = at.checked_sub(1) else {
                if formulas
                    .largest_meta(
                        closed.iter().flat_map(|&i| {
                            let inference = &inferences[i];
                            inference.left.iter().chain(&inference.right).copied()
                        }),
                        closed.iter().filter_map(|&i| inferences[i].term),
                    )
                    .is_some()
                {
                    return Err(bad("an inference no step closed holds an open witness"));
                }
                self.check(formulas, &w.unifier, closed, false)?;
                continue;
            };
            // Fewer steps than inferences, which a `u32` counts.
            let number = s as u32;
            for &i in closed {
                let inference = &inferences[i];
                let Some(t) = inference.term else { continue };
                if !formulas.holds_meta(t) {
                    continue;
                }
                let opens = matches!(inference.rule, Rule::ForallLeft | Rule::ExistsRight)
                    && formulas.first_order().terms[t.index()]
                        == FoTerm::Meta(w.unifier.next_meta());
                if !opens {
                    return Err(bad(
                        "an open witness is not the witness of the one ∀L or ∃R that left it open",
                    ));
                }
                let level = tree.level[i].ok_or(bad("an eigenvariable of the last number"))?;
                w.push(t, level, number);
            }
            // No inference of the step, nor a premise's sequent, holds a
            // witness made later. A premise's term is its own step's.
            let made = w.unifier.next_meta();
            let sequents = closed.iter().flat_map(|&i| {
                let inference = &inferences[i];
                let premises = inference.premises.iter().map(|p| &inferences[p.index()]);
                std::iter::once(inference).chain(premises)
            });
            let roots = sequents.flat_map(|i| i.left.iter().chain(&i.right).copied());
            let terms = closed.iter().filter_map(|&i| inferences[i].term);
            if formulas
                .largest_meta(roots, terms)
                .is_some_and(|k| k >= made)
            {
                return Err(bad(
                    "an inference holds an open witness no step left open before",
                ));
            }
            if let Some((left, right)) = self.axiom(s, &closer) {
                // The formulas as the session showed them when it took the
                // step: those it unified.
                let built = formulas.len();
                let now = w.unifier.metas().iter().map(|m| m.bound).collect();
                let mut resolution = Resolution::new(now);
                let pair = (
                    resolution.node(formulas, left)?,
                    resolution.node(formulas, right)?,
                );
                implications_from(formulas, built)?;
                let g = self.history[s].index();
                let in_scope = |e| tree.in_scope(e, g);
                match unify_axiom(
                    formulas,
                    &mut w.unifier,
                    pair,
                    number,
                    &in_scope,
                    self.most,
                    false,
                ) {
                    Ok(()) => {}
                    Err(Unbound::Large(size)) => {
                        return Err(Error::Refused(Refusal::Terms {
                            terms: size,
                            limit: self.most,
                        }));
                    }
                    Err(Unbound::Differ | Unbound::Eigen(_)) => {
                        return Err(bad(
                            "an axiom's formulas do not unify as the session's would",
                        ));
                    }
                }
            }
            self.check(formulas, &w.unifier, closed, false)?;
        }
        if !w.unifier.bindings().eq(bindings.iter().copied()) {
            return Err(bad("the bindings are not those the axioms make"));
        }
        let all: Vec<usize> = (0..len)
            .filter(|&i| inferences[i].rule != Rule::Open)
            .collect();
        let shown = self.check(formulas, &w.unifier, &all, true)?;
        let mut shown = shown.unwrap_or_else(|| inferences.to_vec());
        for ((inference, raw), made) in shown.iter_mut().zip(&mut w.raw).zip(inferences) {
            if inference.left != made.left
                || inference.right != made.right
                || inference.term != made.term
            {
                *raw = Some(Made::of(made));
            }
        }
        // Each inference under the witnesses it shows and those its form as
        // made reaches through the bindings: an undo of any binding in force
        // changes no other.
        let reach = reached(formulas, &w.unifier);
        for (i, (inference, made)) in shown.iter().zip(inferences).enumerate() {
            let mut metas = formulas.metas_in(
                made.left.iter().chain(&made.right).copied(),
                made.term.into_iter(),
            );
            metas = metas
                .iter()
                .flat_map(|&k| reach.get(k as usize).into_iter().flatten().copied())
                .chain(formulas.metas_in(
                    inference.left.iter().chain(&inference.right).copied(),
                    inference.term.into_iter(),
                ))
                .collect();
            metas.sort_unstable();
            metas.dedup();
            w.register(i, &metas);
        }
        Ok((w, shown))
    }

    /// Returns the formulas left and right of `⊢` of the axiom of step `s`,
    /// as made, where the step is an axiom's (its goal's run of
    /// weakenings, each closed by the step, ends in the axiom) and the two
    /// differ; `closer` is the step that closed each inference.
    fn axiom(&self, s: usize, closer: &[Option<usize>]) -> Option<(NodeId, NodeId)> {
        let mut at = &self.inferences[self.history[s].index()];
        while matches!(at.rule, Rule::WeakenLeft | Rule::WeakenRight)
            && let [p] = at.premises[..]
            && closer[p.index()] == Some(s)
            && self.inferences[p.index()].rule != Rule::Open
        {
            at = &self.inferences[p.index()];
        }
        match (at.rule, &at.left[..], &at.right[..]) {
            (Rule::Axiom, &[l], &[r]) if l != r => Some((l, r)),
            _ => None,
        }
    }

    /// Checks the inferences `closed` by the checker's rules as they show
    /// the bindings `unifier` holds, with their premises likewise. With
    /// `all`, returns every inference as it shows them, `None` where
    /// nothing is bound or `all` is not asked.
    fn check(
        &self,
        formulas: &mut Formulas,
        unifier: &Unifier,
        closed: &[usize],
        all: bool,
    ) -> Result<Option<Vec<Inference>>, Error> {
        let bindings: Vec<Option<FoTermId>> = unifier.metas().iter().map(|m| m.bound).collect();
        let bound = bindings.iter().any(Option::is_some);
        let built = formulas.len();
        let mut resolution = Resolution::new(bindings);
        let mut cache: crate::hash::HashMap<usize, Inference> = crate::hash::HashMap::default();
        let mut get = |i: usize, formulas: &mut Formulas| -> Result<Inference, Error> {
            if !bound {
                return Ok(self.inferences[i].clone());
            }
            if let Some(found) = cache.get(&i) {
                return Ok(found.clone());
            }
            let made = shown(&mut resolution, formulas, &self.inferences[i])?;
            cache.insert(i, made.clone());
            Ok(made)
        };
        let mut checked = Vec::with_capacity(closed.len());
        for &i in closed {
            let inference = get(i, formulas)?;
            let premises = inference
                .premises
                .iter()
                .map(|p| get(p.index(), formulas))
                .collect::<Result<Vec<_>, _>>()?;
            checked.push((inference, premises));
        }
        let every = if all && bound {
            Some(
                (0..self.inferences.len())
                    .map(|i| get(i, formulas))
                    .collect::<Result<Vec<_>, _>>()?,
            )
        } else {
            None
        };
        implications_from(formulas, built)?;
        for (inference, premises) in &checked {
            let premises: Vec<&Inference> = premises.iter().collect();
            if check_inference(formulas, self.logic, inference, &premises).is_err() {
                return Err(Error::InconsistentSession {
                    reason: "an inference breaks the rule it names under the bindings in force",
                });
            }
        }
        Ok(every)
    }
}

/// Returns, per open witness, the witnesses it reaches through the
/// bindings `unifier` holds, itself included, ascending: the open witnesses
/// of its term, of theirs, and so on. The bindings hold no cycle. It walks
/// on a stack of its own, each witness once.
#[cfg(feature = "serialize")]
fn reached(formulas: &Formulas, unifier: &Unifier) -> Vec<Vec<u32>> {
    let metas = unifier.metas();
    let mut reach: Vec<Option<Vec<u32>>> = vec![None; metas.len()];
    for start in 0..metas.len() {
        let mut stack = vec![(start, false)];
        while let Some((k, done)) = stack.pop() {
            if reach[k].is_some() {
                continue;
            }
            let next: Vec<u32> = match metas[k].bound {
                Some(t) => formulas.metas_in(std::iter::empty(), std::iter::once(t)),
                None => Vec::new(),
            };
            if done {
                let mut all = vec![k as u32];
                for &j in &next {
                    all.extend(
                        reach
                            .get(j as usize)
                            .and_then(Option::as_ref)
                            .into_iter()
                            .flatten(),
                    );
                }
                all.sort_unstable();
                all.dedup();
                reach[k] = Some(all);
            } else {
                stack.push((k, true));
                stack.extend(
                    next.iter()
                        .map(|&j| j as usize)
                        .filter(|&j| j < metas.len())
                        .map(|j| (j, false)),
                );
            }
        }
    }
    reach.into_iter().map(Option::unwrap_or_default).collect()
}

/// What the reader of a session asks of its tree of inferences: each
/// inference's place in a walk, the eigenvariables the inferences
/// introduce, and the level an open witness made there takes.
#[cfg(feature = "serialize")]
struct Tree {
    /// When a walk of the tree from the root enters each inference.
    enter: Vec<u32>,
    /// When it leaves it, after every inference above it.
    exit: Vec<u32>,
    /// The inferences that introduce each eigenvariable, by its term.
    introduced: crate::hash::HashMap<FoTermId, Vec<usize>>,
    /// Per inference, one more than the largest eigenvariable a `∀R` or
    /// `∃L` below it introduces, `Some(0)` for none and `None` past the
    /// last number.
    level: Vec<Option<u32>>,
}

#[cfg(feature = "serialize")]
impl Tree {
    /// Returns the tree of `inferences`, every premise after its
    /// conclusion and each the premise of one inference.
    fn of(formulas: &Formulas, inferences: &[Inference]) -> Self {
        let len = inferences.len();
        let mut introduced: crate::hash::HashMap<FoTermId, Vec<usize>> =
            crate::hash::HashMap::default();
        let mut level = vec![Some(0); len];
        for (i, inference) in inferences.iter().enumerate() {
            let above = match inference.eigenvariable() {
                Some(t) => {
                    introduced.entry(t).or_default().push(i);
                    match formulas.first_order().terms[t.index()] {
                        FoTerm::Eigen(e) => {
                            level[i].zip(e.get().checked_add(1)).map(|(a, b)| a.max(b))
                        }
                        FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => level[i],
                    }
                }
                None => level[i],
            };
            for p in &inference.premises {
                level[p.index()] = above;
            }
        }
        let (mut enter, mut exit) = (vec![0; len], vec![0; len]);
        let mut clock = 0u32;
        let mut stack = vec![(0usize, false)];
        while let Some((i, done)) = stack.pop() {
            clock += 1;
            if done {
                exit[i] = clock;
            } else {
                enter[i] = clock;
                stack.push((i, true));
                stack.extend(inferences[i].premises.iter().map(|p| (p.index(), false)));
            }
        }
        Self {
            enter,
            exit,
            introduced,
            level,
        }
    }

    /// Returns whether a `∀R` or `∃L` below inference `g` introduces the
    /// eigenvariable `e`.
    fn in_scope(&self, e: FoTermId, g: usize) -> bool {
        self.introduced.get(&e).is_some_and(|by| {
            by.iter()
                .any(|&b| self.enter[b] < self.enter[g] && self.exit[g] < self.exit[b])
        })
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::linear::search::generate::Rng;
    use Side::{Left, Right};

    /// Starts a proof of `text` in `logic` and returns it with its goal.
    fn start(text: &str, logic: Logic) -> (Interactive, GoalId) {
        let sequent: Sequent = text.parse().unwrap();
        let state = Interactive::new(&sequent, logic).unwrap();
        let goal = state.goals().next().unwrap();
        (state, goal)
    }

    /// Returns the position on `side` of the goal of the formula printed
    /// as `text`.
    fn at(state: &Interactive, goal: GoalId, side: Side, text: &str) -> usize {
        let (left, right) = state.goal(goal).unwrap();
        let list = if side == Left { left } else { right };
        list.iter()
            .position(|&k| state.formulas().formula(k).to_string() == text)
            .unwrap_or_else(|| panic!("no {text} {side:?} in goal {}", goal.get()))
    }

    /// Applies `rule` to the formula printed as `text` on `side`, the
    /// formulas printed as `first` going to the first premise if any, and
    /// returns the goals opened.
    fn step(
        state: &mut Interactive,
        goal: GoalId,
        (side, text): (Side, &str),
        rule: Rule,
        first: Option<(&[&str], &[&str])>,
    ) -> Vec<GoalId> {
        let mut step = Step::new(side, at(state, goal, side, text), rule);
        if let Some((left, right)) = first {
            let left: Vec<usize> = left.iter().map(|t| at(state, goal, Left, t)).collect();
            let right: Vec<usize> = right.iter().map(|t| at(state, goal, Right, t)).collect();
            step = step.split(&left, &right);
        }
        state
            .apply(goal, &step)
            .unwrap_or_else(|e| panic!("{rule} on {text}: {e}"))
    }

    /// Closes the goal by the axiom on the formula printed as `text` right
    /// of `⊢`.
    fn axiom(state: &mut Interactive, goal: GoalId, text: &str) {
        assert_eq!(step(state, goal, (Right, text), Rule::Axiom, None), []);
    }

    /// A state finished through the rules is a derivation the checker
    /// accepts, in each logic: the axiom with a context, a shared and a
    /// split context, the negations as implications into ⊥ in LJ and as
    /// themselves in LK, an equivalence, and contraction.
    #[test]
    fn a_finished_state_checks() {
        let finished = |s: &Interactive| {
            assert!(s.is_complete());
            s.finished(&Limits::default(), |_| false).unwrap()
        };
        for logic in [Logic::Intuitionistic, Logic::Minimal] {
            let (mut s, g) = start("a /\\ b |- b /\\ a", logic);
            let [g] = step(&mut s, g, (Left, "a ∧ b"), Rule::AndLeft, None)[..] else {
                panic!()
            };
            for (g, text) in step(&mut s, g, (Right, "b ∧ a"), Rule::AndRight, None)
                .into_iter()
                .zip(["b", "a"])
            {
                axiom(&mut s, g, text);
            }
            finished(&s);
            let (mut s, g) = start("a -> b, a |- b", logic);
            let first: &[&str] = &["a"];
            let [p, q] = step(
                &mut s,
                g,
                (Left, "a → b"),
                Rule::ImpliesLeft,
                Some((first, &[])),
            )[..] else {
                panic!()
            };
            axiom(&mut s, p, "a");
            axiom(&mut s, q, "b");
            finished(&s);
            // a ⊢ ¬¬a: ¬R gives a, ¬a ⊢ ⊥, and ¬L the premises a ⊢ a and
            // ⊥ ⊢ ⊥, an axiom in minimal logic too.
            let (mut s, g) = start("a |- ~~a", logic);
            let [g] = step(&mut s, g, (Right, "¬¬a"), Rule::NotRight, None)[..] else {
                panic!()
            };
            let [p, q] = step(&mut s, g, (Left, "¬a"), Rule::NotLeft, Some((first, &[])))[..]
            else {
                panic!()
            };
            axiom(&mut s, p, "a");
            axiom(&mut s, q, "⊥");
            let derivation = finished(&s);
            assert_eq!(derivation.sequent().1.len(), 1);
        }
        // Classically: the excluded middle by ¬R as LK has it, Peirce's law
        // with a shared context, and an equivalence.
        let k = Logic::Classical;
        let (mut s, g) = start("|- a \\/ ~a", k);
        let [g] = step(&mut s, g, (Right, "a ∨ ¬a"), Rule::OrRight, None)[..] else {
            panic!()
        };
        let [g] = step(&mut s, g, (Right, "¬a"), Rule::NotRight, None)[..] else {
            panic!()
        };
        assert_eq!(s.goal(g).unwrap().0.len(), 1);
        axiom(&mut s, g, "a");
        finished(&s);
        let (mut s, g) = start("|- ((a -> b) -> a) -> a", k);
        let [g] = step(
            &mut s,
            g,
            (Right, "((a → b) → a) → a"),
            Rule::ImpliesRight,
            None,
        )[..] else {
            panic!()
        };
        let [p, q] = step(&mut s, g, (Left, "(a → b) → a"), Rule::ImpliesLeft, None)[..] else {
            panic!()
        };
        let [p] = step(&mut s, p, (Right, "a → b"), Rule::ImpliesRight, None)[..] else {
            panic!()
        };
        axiom(&mut s, p, "a");
        axiom(&mut s, q, "a");
        finished(&s);
        let (mut s, g) = start("a <-> b, b |- a", k);
        let [g] = step(&mut s, g, (Left, "a ↔ b"), Rule::IffLeft2, None)[..] else {
            panic!()
        };
        let [g] = step(&mut s, g, (Left, "b"), Rule::ContractLeft, None)[..] else {
            panic!()
        };
        let [p, q] = step(
            &mut s,
            g,
            (Left, "b → a"),
            Rule::ImpliesLeft,
            Some((&["b"], &[])),
        )[..] else {
            panic!()
        };
        axiom(&mut s, p, "b");
        axiom(&mut s, q, "a");
        finished(&s);
    }

    /// The same derivation drawn with its structural rules explicit and
    /// admissible: the weakenings below an axiom and the contraction below
    /// the rule that uses both copies are left out, their conclusions the
    /// rule's.
    #[test]
    fn structural_rules_explicit_or_admissible() {
        let (mut s, g) = start("a -> b, a, c |- b", Logic::Intuitionistic);
        let [g] = step(&mut s, g, (Left, "a"), Rule::ContractLeft, None)[..] else {
            panic!()
        };
        let first: &[&str] = &["a"];
        let [p, q] = step(
            &mut s,
            g,
            (Left, "a → b"),
            Rule::ImpliesLeft,
            Some((first, &[])),
        )[..] else {
            panic!()
        };
        axiom(&mut s, p, "a");
        axiom(&mut s, q, "b");
        let view = |structural| {
            let view = ViewOptions::default().with_structural(structural);
            let derivation = s
                .derivation_within(&view, &Limits::default(), |_| false)
                .unwrap();
            (derivation.to_string(), s.derivation_ids(structural))
        };
        let (explicit, ids) = view(Structural::Explicit);
        assert_eq!(
            explicit,
            "             ───── ax\n\
            \x20            b ⊢ b\n\
            \x20           ──────── WL\n\
            \x20           b, a ⊢ b\n\
             ───── ax   ─────────── WL\n\
             a ⊢ a      b, a, c ⊢ b\n\
             ────────────────────── →L\n\
            \x20 a → b, a, a, c ⊢ b\n\
            \x20 ────────────────── CL\n\
            \x20  a → b, a, c ⊢ b"
        );
        assert_eq!(ids.len(), 6);
        let (admissible, ids) = view(Structural::Admissible);
        assert_eq!(
            admissible,
            "───── ax   ─────────── ax\n\
             a ⊢ a      b, a, c ⊢ b\n\
             ────────────────────── →L\n\
            \x20  a → b, a, c ⊢ b"
        );
        // The root shown is the →L inference, number 3 of the state.
        assert_eq!(ids.len(), 3);
        assert_eq!(s.inferences[ids[2].index()].rule, Rule::ImpliesLeft);
        // An open goal keeps the weakening above which it stands.
        let (mut s, g) = start("a, c |- a", Logic::Intuitionistic);
        step(&mut s, g, (Left, "c"), Rule::WeakenLeft, None);
        let view = ViewOptions::default().with_structural(Structural::Admissible);
        let shown = s
            .derivation_within(&view, &Limits::default(), |_| false)
            .unwrap();
        assert_eq!(shown.to_string(), " a ⊢ a\n──────── WL\na, c ⊢ a");
    }

    /// LJ's →L shares its context by default: both premises keep every
    /// hypothesis, the goal right of ⊢ goes to the second, the
    /// contractions the checker needs are made below and left out of the
    /// admissible drawing, and the finished derivation checks.
    #[test]
    fn a_shared_implication_left_checks() {
        for logic in [Logic::Intuitionistic, Logic::Minimal] {
            let (mut s, g) = start("c, a -> b, a |- b", logic);
            let [p, q] = step(&mut s, g, (Left, "a → b"), Rule::ImpliesLeft, None)[..] else {
                panic!()
            };
            let text = |s: &Interactive, g| {
                let (left, right) = s.goal(g).unwrap();
                let side = |list: &[NodeId]| -> Vec<String> {
                    list.iter()
                        .map(|&k| s.formulas().formula(k).to_string())
                        .collect()
                };
                (side(left), side(right))
            };
            assert_eq!(
                text(&s, p),
                (vec!["c".into(), "a".into()], vec!["a".into()])
            );
            assert_eq!(
                text(&s, q),
                (vec!["c".into(), "b".into(), "a".into()], vec!["b".into()])
            );
            axiom(&mut s, p, "a");
            axiom(&mut s, q, "b");
            let derivation = s.finished(&Limits::default(), |_| false).unwrap();
            let contractions = derivation
                .inferences()
                .iter()
                .filter(|i| i.rule() == Rule::ContractLeft)
                .count();
            assert_eq!(contractions, 2);
            let shown = s.derivation().unwrap().to_string();
            assert_eq!(
                shown,
                "──────── ax   ─────────── ax\n\
                 c, a ⊢ a      c, b, a ⊢ b\n\
                 ───────────────────────── →L\n\
                \x20    c, a → b, a ⊢ b"
            );
            assert_eq!(s.undo(), Some(q));
        }
    }

    /// Undo takes back the last step, a rule applied or a goal closed by
    /// the search, and the state is the one before it.
    #[test]
    fn undo_restores() {
        let (mut s, g) = start("a /\\ b, c |- b /\\ a", Logic::Intuitionistic);
        let parts = |s: &Interactive| (s.inferences.clone(), s.history.clone());
        let before = parts(&s);
        let [h] = step(&mut s, g, (Left, "a ∧ b"), Rule::AndLeft, None)[..] else {
            panic!()
        };
        let middle = parts(&s);
        let goals = step(&mut s, h, (Right, "b ∧ a"), Rule::AndRight, None);
        axiom(&mut s, goals[0], "b");
        let closed = s
            .close(
                goals[1],
                None,
                &search::Options::default(),
                &Limits::default(),
                |_| false,
            )
            .unwrap();
        assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)) && closed.grafted.is_ok());
        assert!(s.is_complete());
        assert_eq!(s.steps(), 4);
        for _ in 0..3 {
            s.undo();
        }
        assert_eq!(parts(&s), middle);
        assert_eq!(s.undo(), Some(g));
        assert_eq!(parts(&s), before);
        assert_eq!(s.undo(), None);
    }

    /// A rule refused on a goal it does not fit says why, and changes
    /// nothing.
    #[test]
    fn a_rule_that_does_not_fit_is_refused_with_why() {
        let (mut s, g) = start("a /\\ b, a -> c |- c \\/ a", Logic::Intuitionistic);
        let before = (s.inferences.clone(), s.history.clone());
        let refused = |s: &mut Interactive, step: Step| s.apply(g, &step).unwrap_err();
        let error = refused(&mut s, Step::new(Left, 0, Rule::AndRight));
        assert_eq!(
            error,
            StepError::Rule {
                rule: Rule::AndRight,
                side: Left,
                position: 0
            }
        );
        assert_eq!(
            error.to_string(),
            "the rule ∧R does not act on formula 0 left of ⊢: it acts on a conjunction right of ⊢"
        );
        let error = refused(&mut s, Step::new(Right, 0, Rule::OrRight));
        assert_eq!(
            error.to_string(),
            "the rule ∨R is not available in intuitionistic logic: its premise would have both \
             disjuncts right of ⊢, and every goal of LJ has exactly one formula there: use ∨R₁ \
             or ∨R₂ to keep one disjunct"
        );
        // →L sending c ∨ a to the first premise leaves it beside a there.
        let error = refused(
            &mut s,
            Step::new(Left, 1, Rule::ImpliesLeft).split(&[], &[0]),
        );
        assert_eq!(error, StepError::Succedents { count: 2 });
        assert!(
            error
                .to_string()
                .starts_with("a premise would have 2 formulas right of ⊢")
        );
        assert_eq!(
            refused(&mut s, Step::new(Left, 0, Rule::Axiom)).to_string(),
            "the axiom needs formula 0 left of ⊢ to stand right of ⊢ too"
        );
        assert_eq!(
            refused(
                &mut s,
                Step::new(Left, 1, Rule::ImpliesLeft).split(&[1], &[])
            ),
            StepError::Split {
                side: Left,
                position: 1
            }
        );
        assert_eq!(
            refused(&mut s, Step::new(Left, 0, Rule::AndLeft).split(&[1], &[])),
            StepError::NoSplit {
                rule: Rule::AndLeft
            }
        );
        assert_eq!(
            refused(&mut s, Step::new(Right, 3, Rule::OrRight1)).to_string(),
            "the goal has no formula 3 right of ⊢: it has 1 formula there, and positions start at 0"
        );
        assert_eq!(
            s.apply(GoalId::new(9), &Step::new(Left, 0, Rule::AndLeft)),
            Err(StepError::NoGoal {
                goal: GoalId::new(9)
            })
        );
        assert_eq!((s.inferences.clone(), s.history.clone()), before);
        let (mut s, g) = start("false |- a", Logic::Minimal);
        assert_eq!(
            s.apply(g, &Step::new(Left, 0, Rule::FalseLeft))
                .unwrap_err()
                .to_string(),
            "the rule ⊥L is not available in minimal logic: minimal logic has no ex falso"
        );
        let rules = |s: &Interactive, side, position| -> Vec<Rule> {
            s.rules(g, side, position)
                .unwrap()
                .into_iter()
                .map(|a| a.rule)
                .collect()
        };
        assert_eq!(rules(&s, Left, 0), [Rule::WeakenLeft, Rule::ContractLeft]);
        let (s, g) = start("a -> b, a |- b", Logic::Intuitionistic);
        let applicable = s.rules(g, Left, 0).unwrap();
        assert_eq!(
            applicable,
            [
                Applicable {
                    rule: Rule::WeakenLeft,
                    needs: Needs::Nothing
                },
                Applicable {
                    rule: Rule::ContractLeft,
                    needs: Needs::Nothing
                },
                Applicable {
                    rule: Rule::ImpliesLeft,
                    needs: Needs::Context
                },
            ]
        );
        let (s, g) = start("a -> b, a |- a", Logic::Intuitionistic);
        assert_eq!(s.rules(g, Left, 1).unwrap()[0].rule, Rule::Axiom);
    }

    /// The search closes a goal it proves and leaves one it cannot decide
    /// open, saying why; a hint is the first step of the derivation found,
    /// which applies to the goal.
    #[test]
    fn close_and_hint() {
        let options = search::Options::default();
        let limits = Limits::default();
        let (mut s, g) = start("a -> b, b -> c |- a -> c", Logic::Intuitionistic);
        let hint = s.hint(g, None, &options, &limits, |_| false).unwrap();
        let [step] = &hint.steps[..] else {
            panic!("{:?}", hint.steps)
        };
        assert_eq!((step.side, step.rule), (Right, Rule::ImpliesRight));
        let [g] = s.apply(g, step).unwrap()[..] else {
            panic!()
        };
        // The hint of a split →L names the split the derivation made.
        let hint = s.hint(g, None, &options, &limits, |_| false).unwrap();
        let [step] = &hint.steps[..] else {
            panic!("{:?}", hint.steps)
        };
        assert_eq!(step.rule, Rule::ImpliesLeft);
        assert!(matches!(step.context, Context::Split { .. }));
        s.apply(g, step).unwrap();
        for goal in s.goals().collect::<Vec<_>>() {
            let closed = s.close(goal, None, &options, &limits, |_| false).unwrap();
            assert!(matches!(closed.outcome.verdict, Verdict::Valid(_)));
        }
        s.finished(&limits, |_| false).unwrap();
        // ¬¬(a ∨ ¬a) uses its hypothesis twice: a search allowed one copy
        // of each `!` cannot decide it.
        let (mut s, g) = start("|- ~~(a \\/ ~a)", Logic::Intuitionistic);
        let one = options.clone().with_copies(Some(1));
        let closed = s.close(g, None, &one, &limits, |_| false).unwrap();
        let Verdict::Unknown(reason) = closed.outcome.verdict else {
            panic!("{:?}", closed.outcome.verdict)
        };
        assert_eq!(reason, search::Reason::CopyBound { copies: 1 });
        assert_eq!(s.goals().collect::<Vec<_>>(), [g]);
        assert_eq!(s.hint(g, None, &one, &limits, |_| false).unwrap().steps, []);
        // Peirce's law is not intuitionistic: the goal stays open.
        let (mut s, g) = start("|- ((a -> b) -> a) -> a", Logic::Intuitionistic);
        let closed = s.close(g, None, &options, &limits, |_| false).unwrap();
        assert!(matches!(closed.outcome.verdict, Verdict::NotValid));
        assert_eq!(s.steps(), 0);
    }

    /// Following the hints proves a sequent whose derivation contracts a
    /// hypothesis at its root: the contraction comes with the rule that
    /// uses both copies. Hinted alone, it could be answered by a weakening
    /// of the copy, and the two would alternate for ever.
    #[test]
    fn following_the_hints_proves_a_sequent() {
        // Every sequent is valid, so a search that deepens its copy bound
        // ends.
        let options = search::Options::default().with_copies(None);
        let limits = Limits::default();
        for (text, logic) in [
            (
                "a /\\ (b \\/ c) |- (a /\\ b) \\/ (a /\\ c)",
                Logic::Intuitionistic,
            ),
            ("|- ~~(a \\/ ~a)", Logic::Intuitionistic),
            ("|- ~~(a \\/ ~a)", Logic::Minimal),
            ("(a -> b) -> a, a -> b |- b", Logic::Intuitionistic),
        ] {
            let (mut s, _) = start(text, logic);
            // Each round takes at least one rule that is not structural.
            for _ in 0..50 {
                let Some(goal) = s.goals().next() else { break };
                let hint = s.hint(goal, None, &options, &limits, |_| false).unwrap();
                let mut goal = goal;
                for step in &hint.steps {
                    let opened = s.apply(goal, step).unwrap();
                    if let [next, ..] = opened[..] {
                        goal = next;
                    }
                }
                assert!(!hint.steps.is_empty(), "{text}");
            }
            assert!(s.is_complete(), "{text}");
            s.finished(&limits, |_| false).unwrap();
        }
    }

    /// The sequent's size, the derivation's estimate and the stop bound
    /// what a state makes.
    #[test]
    fn the_bounds_hold() {
        let sequent: Sequent = "a /\\ b |- b /\\ a".parse().unwrap();
        // Six occurrences: a ∧ b, a, b, and b ∧ a, b, a.
        let at = |n| Limits::default().with_occurrences(Some(n));
        assert!(Interactive::within(&sequent, Logic::Classical, &at(6)).is_ok());
        assert!(matches!(
            Interactive::within(&sequent, Logic::Classical, &at(5)),
            Err(Error::Refused(Refusal::Occurrences {
                occurrences: 6,
                limit: 5
            }))
        ));
        let s = Interactive::new(&sequent, Logic::Classical).unwrap();
        let view = ViewOptions::default();
        let small = Limits::default().with_derivation_bytes(Some(100));
        assert!(matches!(
            s.derivation_within(&view, &small, |_| false),
            Err(Error::Refused(Refusal::Output { .. }))
        ));
        assert!(matches!(
            s.derivation_within(&view, &Limits::default(), |_| true),
            Err(Error::Refused(Refusal::Stopped { .. }))
        ));
        let mut s = s;
        let g = s.goals().next().unwrap();
        let stopped = s
            .close(
                g,
                None,
                &search::Options::default(),
                &Limits::default(),
                |_| true,
            )
            .unwrap();
        assert!(matches!(
            stopped.outcome.verdict,
            Verdict::Unknown(search::Reason::Stopped)
        ));
        assert_eq!(s.goals().count(), 1);
        assert!(matches!(
            s.finished(&Limits::default(), |_| false),
            Err(Error::OpenGoals { count: 1 })
        ));
        // A shared →L over 200 hypotheses makes 199 contractions of about
        // 400 formulas each: past a bound of a megabyte, refused before
        // anything is made.
        let hypotheses: Vec<String> = (0..200).map(|i| format!("p{i}")).collect();
        let text = format!("a -> b, {}, a |- b", hypotheses.join(", "));
        let sequent: Sequent = text.parse().unwrap();
        let small = Limits::default().with_derivation_bytes(Some(1 << 20));
        let mut s = Interactive::within(&sequent, Logic::Intuitionistic, &small).unwrap();
        let g = s.goals().next().unwrap();
        let refused = s.apply(g, &Step::new(Left, 0, Rule::ImpliesLeft));
        assert!(matches!(
            refused,
            Err(StepError::Size {
                limit_bytes: 1_048_576,
                ..
            })
        ));
        assert_eq!(s.steps(), 0);
        let mut s = Interactive::new(&sequent, Logic::Intuitionistic).unwrap();
        assert_eq!(
            s.apply(g, &Step::new(Left, 0, Rule::ImpliesLeft))
                .map(|g| g.len()),
            Ok(2)
        );
    }

    /// On generated sequents in every logic, random steps with random
    /// splits and undos never make an inference the checker refuses, and
    /// every state whose open goals the search then proves finishes as a
    /// derivation the checker accepts; written as JSON, each state reads
    /// back as itself.
    #[test]
    fn random_steps_finish_as_checked_derivations() {
        const SEQUENTS: usize = 60;
        let search = search::Options::default().with_copies(Some(4));
        let limits = Limits::default().with_work(Some(20_000));
        let mut rng = Rng::new(49);
        let mut finished = 0;
        for _ in 0..SEQUENTS {
            let sequent = super::super::tests::generated_sequent(&mut rng);
            for &logic in Logic::ALL {
                let Ok(mut s) = Interactive::new(&sequent, logic) else {
                    continue;
                };
                for _ in 0..12 {
                    if rng.one_in(6) {
                        s.undo();
                        continue;
                    }
                    let goals: Vec<GoalId> = s.goals().collect();
                    let Some(&goal) = goals.get(rng.below(goals.len().max(1))) else {
                        break;
                    };
                    let (left, right) = s.goal(goal).unwrap();
                    let (nl, nr) = (left.len(), right.len());
                    if nl + nr == 0 {
                        break;
                    }
                    let k = rng.below(nl + nr);
                    let (side, position) = if k < nl { (Left, k) } else { (Right, k - nl) };
                    let rules = s.rules(goal, side, position).unwrap();
                    let Some(applicable) = rules.get(rng.below(rules.len().max(1))) else {
                        continue;
                    };
                    let mut step = Step::new(side, position, applicable.rule);
                    if applicable.needs == Needs::Context && rng.one_in(2) {
                        let pick = |n: usize, rng: &mut Rng, s: Side| -> Vec<usize> {
                            (0..n)
                                .filter(|&i| !(s == side && i == position) && rng.one_in(2))
                                .collect()
                        };
                        let first = (pick(nl, &mut rng, Left), pick(nr, &mut rng, Right));
                        step = step.split(&first.0, &first.1);
                    }
                    let _ = s.apply(goal, &step);
                }
                #[cfg(feature = "serialize")]
                {
                    let json = serde_json::to_string(&s).unwrap();
                    let mut document = serde_json::Deserializer::from_str(&json);
                    let read: Interactive = crate::wire::upgrade(&mut document, &limits)
                        .unwrap_or_else(|e| panic!("{sequent} in {logic}: {e}\n{json}"));
                    assert_eq!(serde_json::to_string(&read).unwrap(), json);
                }
                let goals: Vec<GoalId> = s.goals().collect();
                let proved = goals.into_iter().all(|goal| {
                    s.close(goal, None, &search, &limits, |_| false)
                        .is_ok_and(|c| matches!(c.outcome.verdict, Verdict::Valid(_)))
                });
                if proved {
                    s.finished(&Limits::default(), |_| false)
                        .unwrap_or_else(|e| panic!("{sequent} in {logic}: {e}"));
                    finished += 1;
                }
            }
        }
        // Random steps often weaken a formula the goal needs: a quarter
        // of the states finishing keeps the check from being vacuous.
        assert!(finished >= SEQUENTS / 4, "{finished} states finished");
    }

    /// A document that no sequence of steps writes is refused: an
    /// inference that breaks its rule, a step that names no inference it
    /// could have closed, and a derivation that is no tree.
    #[cfg(feature = "serialize")]
    #[test]
    fn a_forged_document_is_refused() {
        let read = |json: &str| {
            let mut document = serde_json::Deserializer::from_str(json);
            crate::wire::upgrade::<Interactive, _>(&mut document, &Limits::default()).map(|_| ())
        };
        let session = r#"{"version":8,"logic":"intuitionistic","sequent":{"formulas":[{"V":0},{"V":1},{"→":[0,1]},"⊥"],"atoms":["a","b"],"left":[2,0],"right":[1]},"inferences":[{"left":[2,0],"right":[1],"rule":"→L","principal":["left",0],"premises":[1,2]},{"left":[0],"right":[0],"rule":"ax"},{"left":[1],"right":[1]}],"history":[0,1]}"#;
        assert_eq!(read(session), Ok(()));
        let inconsistent =
            |json: &str| matches!(read(json), Err(Error::InconsistentSession { .. }));
        assert!(inconsistent(
            &session.replace(r#""rule":"ax""#, r#""rule":"⊤R""#)
        ));
        assert!(inconsistent(&session.replace(
            r#""left":[0],"right":[0]"#,
            r#""left":[1],"right":[1]"#
        )));
        assert!(inconsistent(
            &session.replace(r#""history":[0,1]"#, r#""history":[1,0]"#)
        ));
        let derivation = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0}],"atoms":["a"],"left":[0],"right":[0]},"inferences":[{"left":[0],"right":[0],"rule":"ax"},{"left":[0],"right":[0],"rule":"ax"}]}"#;
        let derivation_of = |json: &str| {
            let mut document = serde_json::Deserializer::from_str(json);
            crate::wire::upgrade::<Derivation, _>(&mut document, &Limits::default())
        };
        assert!(matches!(derivation_of(derivation), Err(Error::Json { .. })));
        // An open goal must be a leaf: drawn bare with a premise above it,
        // it would underflow the layout's height.
        let open = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0},{"V":1}],"atoms":["a","b"],"left":[1],"right":[1]},"inferences":[{"left":[1],"right":[1],"rule":"ax"},{"left":[1],"right":[1],"rule":"open","premises":[0]}]}"#;
        assert!(matches!(derivation_of(open), Err(Error::Json { .. })));
        // A step of LJ that closed `a, ¬a ⊢`: undone, it would reopen a
        // goal without a formula right of ⊢, whose search's derivation
        // concludes `a, ¬a ⊢ ⊥`.
        let empty = r#"{"version":8,"logic":"intuitionistic","sequent":{"formulas":[{"V":0},{"V":1},{"¬":0}],"atoms":["a","b"],"left":[0,2],"right":[1]},"inferences":[{"left":[0,2],"right":[1],"rule":"WR","principal":["right",0],"premises":[1]},{"left":[0,2],"right":[],"rule":"¬L","principal":["left",1],"premises":[2]},{"left":[0],"right":[0],"rule":"ax"}],"history":[0,1,2]}"#;
        assert!(inconsistent(empty));
        // LK's ¬R has its own form only, which its certificate reads.
        let arrow = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0},"⊥",{"¬":0}],"atoms":["a"],"left":[1],"right":[2]},"inferences":[{"left":[1],"right":[1],"rule":"ax"},{"left":[1,0],"right":[1],"rule":"WL","principal":["left",1],"premises":[0]},{"left":[1],"right":[2],"rule":"¬R","principal":["right",0],"premises":[1]}]}"#;
        let arrow = derivation_of(arrow).unwrap();
        assert!(matches!(
            arrow.check(&Limits::default(), |_| false),
            Err(Error::ReadBack { .. })
        ));
        #[cfg(feature = "rocq")]
        {
            let mut out = String::new();
            let options = crate::export::rocq::Options::default();
            let written = crate::export::rocq::write(&arrow, &options, &mut out, |_| false);
            assert!(written.is_err());
        }
        // A run whose conclusion lacks the principal of the rule above is
        // drawn, not left out.
        let lacking = r#"{"version":8,"logic":"classical","sequent":{"formulas":[{"V":0},{"V":1},"⊤"],"atoms":["a","b"],"left":[],"right":[1,1]},"inferences":[{"left":[],"right":[0,2],"rule":"⊤R","principal":["right",1]},{"left":[],"right":[1],"rule":"WR","principal":["right",0],"premises":[0]},{"left":[],"right":[1,1],"rule":"WR","principal":["right",0],"premises":[1]}]}"#;
        let lacking = derivation_of(lacking).unwrap();
        assert_eq!(lacking.shown(Structural::Admissible).inferences().len(), 2);
    }

    /// Applies the quantifier rule `rule` to the formula printed as `text`
    /// on `side`, with the witness read from `witness` if any, and returns
    /// the goals opened.
    fn quantifier(
        state: &mut Interactive,
        goal: GoalId,
        (side, text): (Side, &str),
        rule: Rule,
        witness: Option<&str>,
    ) -> Result<Vec<GoalId>, StepError> {
        let mut step = Step::new(side, at(state, goal, side, text), rule);
        if let Some(witness) = witness {
            step = step.with_witness(state.witness(goal, witness)?);
        }
        state.apply(goal, &step)
    }

    /// The quantifier rules take a typed witness, held to the goal's
    /// scope, and name a fresh eigenvariable, the same one again after an
    /// undo; the finished proof checks and its JSON reads back.
    #[test]
    fn the_quantifier_rules_take_witnesses_and_eigenvariables() {
        let (mut s, g) = start(
            "forall x. p(x) -> q(x), p(c) |- exists y. q(y)",
            Logic::Intuitionistic,
        );
        let [g] =
            quantifier(&mut s, g, (Right, "∃y. q(y)"), Rule::ExistsRight, Some("c")).unwrap()[..]
        else {
            panic!()
        };
        let [g] = quantifier(
            &mut s,
            g,
            (Left, "∀x. (p(x) → q(x))"),
            Rule::ForallLeft,
            Some("c"),
        )
        .unwrap()[..] else {
            panic!()
        };
        let [first, second] = step(&mut s, g, (Left, "p(c) → q(c)"), Rule::ImpliesLeft, None)[..]
        else {
            panic!()
        };
        axiom(&mut s, first, "p(c)");
        axiom(&mut s, second, "q(c)");
        s.finished(&Limits::default(), |_| false).unwrap();
        for (witness, why) in [
            ("d", "not a symbol"),
            ("p", "predicate"),
            ("c(c)", "arguments"),
            ("", "no term"),
        ] {
            let (mut s, g) = start("forall x. p(x) |- p(c)", Logic::Classical);
            let error = quantifier(
                &mut s,
                g,
                (Left, "∀x. p(x)"),
                Rule::ForallLeft,
                Some(witness),
            )
            .unwrap_err();
            assert!(error.to_string().contains(why), "{witness}: {error}");
        }
        // `∃L` names `a`, the first name no symbol has; undone and taken
        // The right side's `∃y. p(y)` is the hypotheses' formula, which
        // prints with the name written first.
        // again, it names `a` again. The other branch cannot use it.
        let (mut s, g) = start(
            "(exists x. p(x)) \\/ (exists x. p(x)) |- exists y. p(y)",
            Logic::Classical,
        );
        let [g, other] = step(
            &mut s,
            g,
            (Left, "(∃x. p(x)) ∨ (∃x. p(x))"),
            Rule::OrLeft,
            None,
        )[..] else {
            panic!()
        };
        let made = |s: &mut Interactive| {
            let [h] = quantifier(s, g, (Left, "∃x. p(x)"), Rule::ExistsLeft, None).unwrap()[..]
            else {
                panic!()
            };
            at(s, h, Left, "p(a)");
            h
        };
        made(&mut s);
        s.undo();
        let h = made(&mut s);
        let error = quantifier(
            &mut s,
            other,
            (Right, "∃x. p(x)"),
            Rule::ExistsRight,
            Some("a"),
        )
        .unwrap_err();
        assert!(
            error.to_string().contains("not in the goal's scope"),
            "{error}"
        );
        let [h] =
            quantifier(&mut s, h, (Right, "∃x. p(x)"), Rule::ExistsRight, Some("a")).unwrap()[..]
        else {
            panic!()
        };
        axiom(&mut s, h, "p(a)");
        let [o] =
            quantifier(&mut s, other, (Left, "∃x. p(x)"), Rule::ExistsLeft, None).unwrap()[..]
        else {
            panic!()
        };
        let [o] =
            quantifier(&mut s, o, (Right, "∃x. p(x)"), Rule::ExistsRight, Some("b")).unwrap()[..]
        else {
            panic!()
        };
        axiom(&mut s, o, "p(b)");
        let derivation = s.finished(&Limits::default(), |_| false).unwrap();
        assert!(
            derivation
                .inferences()
                .iter()
                .any(|i| i.rule() == Rule::ExistsLeft)
        );
        let (mut open, g) = start("forall x. p(x) |- p(c)", Logic::Classical);
        let [g] = quantifier(&mut open, g, (Left, "∀x. p(x)"), Rule::ForallLeft, None).unwrap()[..]
        else {
            panic!()
        };
        at(&open, g, Left, "p($1)");
        #[cfg(feature = "serialize")]
        {
            let json = serde_json::to_string(&s).unwrap();
            let mut read: Interactive = serde_json::from_str(&json).unwrap();
            assert_eq!(serde_json::to_string(&read).unwrap(), json);
            read.undo();
            read.undo();
            assert!(!read.is_complete());
        }
    }

    /// `∀R` and `∃L` are invertible, `∀L` and `∃R` are not.
    #[test]
    fn the_eigenvariable_rules_are_invertible() {
        let (mut s, _) = start(
            "exists x. p(x) |- forall y. (q -> q)",
            Logic::Intuitionistic,
        );
        s.invert_all(&Limits::default());
        assert!(s.is_complete(), "{:?}", s.goals().collect::<Vec<_>>());
        s.finished(&Limits::default(), |_| false).unwrap();
        let (mut s, _) = start("forall x. p(x) |- exists y. p(y)", Logic::Classical);
        s.invert_all(&Limits::default());
        assert_eq!(s.steps(), 0);
    }

    /// An instance holding an equivalence the arena lacked gets its
    /// implications, so the rules on it are offered and taken.
    #[test]
    fn an_instance_s_equivalence_has_its_implications() {
        let (mut s, g) = start("forall x. (p(x) <-> q(x)), p(c) |- q(c)", Logic::Classical);
        let [g] = quantifier(
            &mut s,
            g,
            (Left, "∀x. (p(x) ↔ q(x))"),
            Rule::ForallLeft,
            Some("c"),
        )
        .unwrap()[..] else {
            panic!()
        };
        let at = at(&s, g, Left, "p(c) ↔ q(c)");
        assert!(
            s.rules(g, Left, at)
                .unwrap()
                .iter()
                .any(|r| r.rule == Rule::IffLeft)
        );
        let (mut s, _) = start("|- forall x. (p(x) <-> p(x))", Logic::Intuitionistic);
        s.invert_all(&Limits::default());
        assert!(s.is_complete());
    }

    /// A session document is held to what a step makes: closed formulas,
    /// no open witness; and the drawn inferences' goals follow the
    /// admissible view, which keeps a weakening that brings an
    /// eigenvariable into its rule's conclusion.
    #[cfg(feature = "serialize")]
    #[test]
    fn a_first_order_session_document_is_held_to_its_steps() {
        let session = |fo: &str| {
            format!(
                r#"{{"version":25,"logic":"classical","sequent":{{"formulas":[{{"V":0}}],"atoms":[{{"P":[0,[0]]}}],"symbols":[["p",1]],"fo":[{fo}],"left":[0],"right":[0]}},"inferences":[{{"left":[0],"right":[0]}}],"history":[]}}"#
            )
        };
        assert!(serde_json::from_str::<Interactive>(&session(r#"{"B":0}"#)).is_err());
        assert!(serde_json::from_str::<Interactive>(&session(r#"{"M":0}"#)).is_err());
        let view = r#"{"version":25,"logic":"classical","sequent":{"formulas":[{"V":0},{"∀":0},{"V":1},{"V":2}],"atoms":[{"P":[0,[0]]},{"P":[1,[1]]},{"P":[0,[1]]}],"symbols":[["p",1],["q",1]],"fo":[{"B":0},{"E":0}],"binders":["x"],"left":[2,1],"right":[1]},"inferences":[{"left":[2,1],"right":[1],"rule":"WL","principal":["left",0],"premises":[1]},{"left":[1],"right":[1],"rule":"∀R","principal":["right",0],"premises":[2],"term":1},{"left":[1],"right":[3],"rule":"∀L","principal":["left",0],"premises":[3],"term":1},{"left":[3,1],"right":[3],"rule":"WL","principal":["left",1],"premises":[4]},{"left":[3],"right":[3],"rule":"ax"}],"history":[0,1,2,3]}"#;
        let state: Interactive = serde_json::from_str(view).unwrap();
        let shown = state
            .derivation_within(&ViewOptions::default(), &Limits::default(), |_| false)
            .unwrap();
        assert_eq!(
            state.derivation_ids(Structural::Admissible).len(),
            shown.inferences().len()
        );
        // An eigenvariable where no atom is reads back with its term.
        let (mut s, g) = start("|- forall x. true", Logic::Classical);
        let [g] = quantifier(&mut s, g, (Right, "∀x. ⊤"), Rule::ForallRight, None).unwrap()[..]
        else {
            panic!()
        };
        step(&mut s, g, (Right, "⊤"), Rule::TrueRight, None);
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(
            serde_json::to_string(&serde_json::from_str::<Interactive>(&json).unwrap()).unwrap(),
            json
        );
        let derivation = s.finished(&Limits::default(), |_| false).unwrap();
        let read: Derivation =
            serde_json::from_str(&serde_json::to_string(&derivation).unwrap()).unwrap();
        read.check(&Limits::default(), |_| false).unwrap();
    }

    /// Applies the axiom to the formula printed as `text` on `side`.
    fn closes(
        state: &mut Interactive,
        goal: GoalId,
        (side, text): (Side, &str),
    ) -> Result<Vec<GoalId>, StepError> {
        let position = at(state, goal, side, text);
        state.apply(goal, &Step::new(side, position, Rule::Axiom))
    }

    /// `∀x. p(x) ⊢ ∃x. p(x)` with no constant proves in LK, LJ and LM: two
    /// open witnesses, one bound to the other by the axiom and the other
    /// a fresh eigenvariable in the finished derivation, which checks. The
    /// binding shows in every inference, an undo takes it back, and a
    /// session document reads back as itself before and after it.
    #[test]
    fn an_axiom_fills_the_open_witnesses() {
        for logic in [Logic::Classical, Logic::Intuitionistic, Logic::Minimal] {
            let (mut s, g) = start("forall x. p(x) |- exists x. p(x)", logic);
            let [g] =
                quantifier(&mut s, g, (Right, "∃x. p(x)"), Rule::ExistsRight, None).unwrap()[..]
            else {
                panic!()
            };
            let [g] =
                quantifier(&mut s, g, (Left, "∀x. p(x)"), Rule::ForallLeft, None).unwrap()[..]
            else {
                panic!()
            };
            assert!(
                s.rules(g, Left, at(&s, g, Left, "p($2)"))
                    .unwrap()
                    .iter()
                    .any(|r| r.rule == Rule::Axiom)
            );
            #[cfg(feature = "serialize")]
            let before = serde_json::to_string(&s).unwrap();
            assert_eq!(closes(&mut s, g, (Left, "p($2)")).unwrap(), []);
            assert_eq!(s.bindings(), [("$2".to_owned(), "$1".to_owned())]);
            let derivation = s.finished(&Limits::default(), |_| false).unwrap();
            // The witness left open is an eigenvariable no rule introduces,
            // and the derivation's document holds no open witness.
            assert!(derivation.inferences().iter().all(|i| {
                i.term()
                    .is_none_or(|t| derivation.formulas().is_eigenvariable(t))
            }));
            #[cfg(feature = "serialize")]
            {
                let json = serde_json::to_string(&derivation).unwrap();
                let read: Derivation = serde_json::from_str(&json).unwrap();
                read.check(&Limits::default(), |_| false).unwrap();
            }
            #[cfg(feature = "serialize")]
            {
                let after = serde_json::to_string(&s).unwrap();
                assert!(after.contains(r#""bindings":[[1,"#), "{after}");
                let mut read: Interactive = serde_json::from_str(&after).unwrap();
                assert_eq!(serde_json::to_string(&read).unwrap(), after);
                read.undo();
                assert_eq!(serde_json::to_string(&read).unwrap(), before);
                let read: Interactive = serde_json::from_str(&before).unwrap();
                assert_eq!(serde_json::to_string(&read).unwrap(), before);
            }
            s.undo();
            assert_eq!(s.bindings(), []);
            at(&s, g, Left, "p($2)");
            s.undo();
            s.undo();
            assert!(s.witnesses.is_none());
        }
    }

    /// An open witness takes no eigenvariable made after it: in
    /// `∀x. ∃y. r(x, y) ⊢ ∃y. ∀x. r(x, y)` the axiom would have to give
    /// `$1`, left open first, the eigenvariable of the later `∃L`.
    #[test]
    fn an_open_witness_takes_no_later_eigenvariable() {
        let (mut s, g) = start(
            "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)",
            Logic::Classical,
        );
        let [g] = quantifier(
            &mut s,
            g,
            (Right, "∃y. ∀x. r(x, y)"),
            Rule::ExistsRight,
            None,
        )
        .unwrap()[..] else {
            panic!()
        };
        let [g] =
            quantifier(&mut s, g, (Right, "∀x. r(x, $1)"), Rule::ForallRight, None).unwrap()[..]
        else {
            panic!()
        };
        let [g] =
            quantifier(&mut s, g, (Left, "∀x. ∃y. r(x, y)"), Rule::ForallLeft, None).unwrap()[..]
        else {
            panic!()
        };
        let [g] =
            quantifier(&mut s, g, (Left, "∃y. r($2, y)"), Rule::ExistsLeft, None).unwrap()[..]
        else {
            panic!()
        };
        let error = closes(&mut s, g, (Left, "r($2, b)")).unwrap_err();
        assert!(matches!(error, StepError::Binding { .. }), "{error}");
        assert!(error.to_string().contains("eigenvariable b"), "{error}");
        assert_eq!(s.bindings(), []);
        at(&s, g, Left, "r($2, b)");
    }

    /// A session document with open witnesses is read as its steps made it:
    /// bindings that are not the axiom's, an axiom whose formulas do not
    /// unify, and an open witness in the sequent are refused.
    #[cfg(feature = "serialize")]
    #[test]
    fn a_session_document_is_held_to_its_bindings() {
        let (mut s, g) = start("forall x. p(x) |- exists x. p(x)", Logic::Classical);
        let [g] = quantifier(&mut s, g, (Right, "∃x. p(x)"), Rule::ExistsRight, None).unwrap()[..]
        else {
            panic!()
        };
        let [g] = quantifier(&mut s, g, (Left, "∀x. p(x)"), Rule::ForallLeft, None).unwrap()[..]
        else {
            panic!()
        };
        closes(&mut s, g, (Left, "p($2)")).unwrap();
        let json = serde_json::to_string(&s).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let refused = |change: &dyn Fn(&mut serde_json::Value)| {
            let mut forged = value.clone();
            change(&mut forged);
            serde_json::from_value::<Interactive>(forged).is_err()
        };
        // No binding, or one the axiom does not make.
        assert!(refused(&|v| {
            v.as_object_mut().unwrap().remove("bindings");
        }));
        assert!(refused(&|v| v["bindings"][0][2] = 0.into()));
        // The axiom's formulas, made as `p($2) ⊢ p($1)`, made equal.
        let ax = value["inferences"]
            .as_array()
            .unwrap()
            .iter()
            .position(|i| i["rule"] == "ax")
            .unwrap();
        assert!(refused(&|v| {
            let left = v["inferences"][ax]["left"].clone();
            v["inferences"][ax]["right"] = left;
        }));
    }

    /// Applies `rule` at `position` of `side` with no witness or split and
    /// returns the goals opened.
    fn at_position(
        state: &mut Interactive,
        goal: GoalId,
        (side, position): (Side, usize),
        rule: Rule,
    ) -> Result<Vec<GoalId>, StepError> {
        state.apply(goal, &Step::new(side, position, rule))
    }

    /// Returns whether the state's JSON reads back as itself.
    #[cfg(feature = "serialize")]
    fn reads_back(state: &Interactive) -> bool {
        let json = serde_json::to_string(state).unwrap();
        let read: Interactive =
            serde_json::from_str(&json).unwrap_or_else(|e| panic!("{e}: {json}"));
        serde_json::to_string(&read).unwrap() == json
    }

    /// A binding that builds an equivalence the arena lacked builds its
    /// implications too, which the rules on it and the checker look up:
    /// the rules list it, the finished derivation checks, and a sound
    /// binding through it is taken.
    #[test]
    fn a_binding_builds_an_equivalence_s_implications() {
        let one = |state: &mut Interactive, goal, at, rule| {
            let opened = at_position(state, goal, at, rule).unwrap();
            assert_eq!(opened.len(), 1, "{rule}");
            opened[0]
        };
        let (mut s, g) = start(
            "forall x. ((p(x) <-> q) /\\ s(x)) |- s(a) /\\ u",
            Logic::Classical,
        );
        let g = one(&mut s, g, (Left, 0), Rule::ForallLeft);
        let g = one(&mut s, g, (Left, 0), Rule::AndLeft);
        let [first, second] = at_position(&mut s, g, (Right, 0), Rule::AndRight).unwrap()[..]
        else {
            panic!()
        };
        closes(&mut s, first, (Left, "s($1)")).unwrap();
        assert!(
            s.rules(second, Left, 0)
                .unwrap()
                .iter()
                .any(|r| r.rule == Rule::IffLeft1)
        );
        one(&mut s, second, (Left, 0), Rule::IffLeft1);
        let (mut s, g) = start("forall x. (q <-> r(x)), q |- q", Logic::Classical);
        let g = one(&mut s, g, (Left, 0), Rule::ForallLeft);
        let g = one(&mut s, g, (Left, 0), Rule::IffLeft1);
        closes(&mut s, g, (Left, "q")).unwrap();
        s.finished(&Limits::default(), |_| false).unwrap();
        let (mut s, g) = start("forall x. (p(x) <-> q), p(a) |- q", Logic::Classical);
        let g = one(&mut s, g, (Left, 0), Rule::ForallLeft);
        let g = one(&mut s, g, (Left, 0), Rule::IffLeft1);
        let [first, second] = at_position(&mut s, g, (Left, 0), Rule::ImpliesLeft).unwrap()[..]
        else {
            panic!()
        };
        closes(&mut s, first, (Right, "p($1)")).unwrap();
        closes(&mut s, second, (Left, "q")).unwrap();
        s.finished(&Limits::default(), |_| false).unwrap();
        #[cfg(feature = "serialize")]
        assert!(reads_back(&s));
    }

    /// A session's own document reads back as itself where the axiom
    /// follows a weakening the user took, and where the goal the axiom
    /// closes was changed by an earlier binding.
    #[cfg(feature = "serialize")]
    #[test]
    fn a_session_s_own_bindings_read_back() {
        let (mut s, g) = start("p(a), q |- exists x. p(x)", Logic::Intuitionistic);
        let [g] = at_position(&mut s, g, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
            panic!()
        };
        let [g] = at_position(&mut s, g, (Left, 1), Rule::WeakenLeft).unwrap()[..] else {
            panic!()
        };
        closes(&mut s, g, (Left, "p(a)")).unwrap();
        assert!(reads_back(&s));
        let (mut s, g) = start(
            "r(c) |- exists x. (r(x) /\\ exists u. (q(u) -> q(g(x))))",
            Logic::Intuitionistic,
        );
        let [g] = at_position(&mut s, g, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
            panic!()
        };
        let split = Step::new(Right, 0, Rule::AndRight).split(&[0], &[]);
        let [first, second] = s.apply(g, &split).unwrap()[..] else {
            panic!()
        };
        let [g] = at_position(&mut s, second, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
            panic!()
        };
        let [g] = at_position(&mut s, g, (Right, 0), Rule::ImpliesRight).unwrap()[..] else {
            panic!()
        };
        closes(&mut s, first, (Left, "r(c)")).unwrap();
        closes(&mut s, g, (Left, "q($2)")).unwrap();
        s.finished(&Limits::default(), |_| false).unwrap();
        assert!(reads_back(&s));
    }

    /// The reader refuses an open witness that no step left open, or one
    /// held by an inference no step closed, and does not panic on it.
    #[cfg(feature = "serialize")]
    #[test]
    fn a_witness_no_step_left_open_is_refused() {
        let forged = r#"{"version":25,"logic":"intuitionistic","sequent":{"formulas":[{"V":0},{"V":1},{"∃":1},"⊥",{"V":2}],"atoms":[{"P":[1,[0]]},{"P":[1,[1]]},{"P":[1,[2]]}],"symbols":[["a",0],["p",1]],"fo":[{"F":[0,[]]},{"B":0},{"M":0}],"binders":["x"],"left":[0],"right":[2]},"inferences":[{"left":[0],"right":[2],"rule":"∃R","principal":["right",0],"premises":[1],"term":2},{"left":[0],"right":[4],"rule":"ax"}],"history":[1],"bindings":[[0,0,0]]}"#;
        assert!(serde_json::from_str::<Interactive>(forged).is_err());
        let (mut s, g) = start("p(a) |- exists x. p(x)", Logic::Intuitionistic);
        at_position(&mut s, g, (Right, 0), Rule::ExistsRight).unwrap();
        let mut value = serde_json::to_value(&s).unwrap();
        value["history"] = serde_json::json!([]);
        assert!(serde_json::from_value::<Interactive>(value).is_err());
        // An inference no step closed, which the reader takes, stays
        // readable once a witness is left open after it.
        let (mut s, g) = start(
            "exists x. forall y. s |- forall y. exists x. s",
            Logic::Intuitionistic,
        );
        let [g] = at_position(&mut s, g, (Right, 0), Rule::ForallRight).unwrap()[..] else {
            panic!()
        };
        let mut value = serde_json::to_value(&s).unwrap();
        value["history"] = serde_json::json!([]);
        let mut read: Interactive = serde_json::from_value(value).unwrap();
        at_position(&mut read, g, (Right, 0), Rule::ExistsRight).unwrap();
        assert!(reads_back(&read));
    }

    /// A binding whose term would unfold past the bound on terms is
    /// refused, the state as it was.
    #[test]
    fn a_binding_is_held_to_the_bound_on_terms() {
        let sequent: Sequent = "forall x y. (p(x) /\\ (p(f(y, y)) -> q)) |- q"
            .parse()
            .unwrap();
        let limits = Limits {
            terms: Some(8),
            ..Limits::default()
        };
        let mut s = Interactive::within(&sequent, Logic::Classical, &limits).unwrap();
        let mut g = GoalId::new(0);
        for rule in [Rule::ForallLeft, Rule::ForallLeft, Rule::AndLeft] {
            g = at_position(&mut s, g, (Left, 0), rule).unwrap()[0];
        }
        let [first, _] = at_position(&mut s, g, (Left, 1), Rule::ImpliesLeft).unwrap()[..] else {
            panic!()
        };
        let error = closes(&mut s, first, (Left, "p($1)")).unwrap_err();
        assert!(
            matches!(
                error,
                StepError::Terms {
                    terms: 11,
                    limit: 8
                }
            ),
            "{error}"
        );
        assert_eq!(s.bindings(), []);
    }

    /// A witness an undone step made stays in the arena's atoms, numbered
    /// past the live witnesses; the axiom's measure of the arena passes it
    /// by.
    #[test]
    fn an_undone_witness_left_in_the_arena_is_passed_by() {
        let (mut s, g) = start(
            "forall x. forall y. r(x, y) |- forall y. r(a, y)",
            Logic::Classical,
        );
        let [g] = at_position(&mut s, g, (Left, 0), Rule::ForallLeft).unwrap()[..] else {
            panic!()
        };
        at_position(&mut s, g, (Left, 0), Rule::ForallLeft).unwrap();
        s.undo();
        s.rules(g, Left, 0).unwrap();
        assert!(closes(&mut s, g, (Left, "∀y. r($1, y)")).is_ok());
    }

    /// A session held to a bound on terms writes documents its reader
    /// takes under the same bound, the steps after a filling growing the
    /// arena.
    #[cfg(feature = "serialize")]
    #[test]
    fn a_session_under_a_bound_on_terms_reads_back_under_it() {
        let sequent: Sequent = "forall x. p(x) |- (exists x. p(x)) /\\ (exists x. p(x))"
            .parse()
            .unwrap();
        let limits = Limits {
            terms: Some(5),
            ..Limits::default()
        };
        let mut s = Interactive::within(&sequent, Logic::Classical, &limits).unwrap();
        let [first, second] =
            at_position(&mut s, GoalId::new(0), (Right, 0), Rule::AndRight).unwrap()[..]
        else {
            panic!()
        };
        let [g] = at_position(&mut s, first, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
            panic!()
        };
        let [g] = at_position(&mut s, g, (Left, 0), Rule::ForallLeft).unwrap()[..] else {
            panic!()
        };
        closes(&mut s, g, (Left, "p($2)")).unwrap();
        let [g] = at_position(&mut s, second, (Right, 0), Rule::ExistsRight).unwrap()[..] else {
            panic!()
        };
        at_position(&mut s, g, (Left, 0), Rule::ForallLeft).unwrap();
        let json = serde_json::to_string(&s).unwrap();
        let mut reader = serde_json::Deserializer::from_str(&json);
        let read: Interactive = crate::wire::upgrade(&mut reader, &limits).unwrap();
        assert_eq!(serde_json::to_string(&read).unwrap(), json);
    }

    /// Returns where the state is not what a walk over every inference
    /// would make it: an inference that does not show the bindings in force
    /// over what it was made as, a form as made kept where nothing changed
    /// it, or a closed inference the checker's rules refuse.
    fn walked_differs(state: &Interactive) -> Option<String> {
        let mut formulas = state.formulas.clone();
        let w = state.witnesses.as_deref();
        let bindings = w.map_or_else(Vec::new, |w| {
            w.unifier.metas().iter().map(|m| m.bound).collect()
        });
        let mut resolution = Resolution::new(bindings);
        for (i, inference) in state.inferences.iter().enumerate() {
            let raw = w.and_then(|w| w.raw[i].as_ref());
            let mut made = inference.clone();
            if let Some(raw) = raw {
                made.left.clone_from(&raw.left);
                made.right.clone_from(&raw.right);
                made.term = raw.term;
            }
            let expected = shown(&mut resolution, &mut formulas, &made).unwrap();
            if expected != *inference {
                return Some(format!("inference {i} does not show the bindings"));
            }
            if raw.is_some_and(|raw| raw.shows(inference)) {
                return Some(format!("inference {i} keeps a form as made that shows"));
            }
            if inference.rule != Rule::Open {
                let premises: Vec<&Inference> = inference
                    .premises
                    .iter()
                    .map(|p| &state.inferences[p.index()])
                    .collect();
                if let Err(why) =
                    check_inference(&state.formulas, state.logic, inference, &premises)
                {
                    return Some(format!("inference {i}: {why}"));
                }
            }
        }
        None
    }

    /// Random first-order sessions with open witnesses, fillings, undos and
    /// documents read back show, after every step, what a walk over every
    /// inference would show, and every closed inference checks.
    #[test]
    fn a_binding_shown_through_its_holders_is_the_walk_s() {
        const SEQUENTS: [&str; 8] = [
            "forall x. p(x) |- exists x. p(x)",
            "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)",
            "forall x. (p(x) -> q(x)), forall x. p(x) |- forall x. q(x)",
            "exists x. (p(x) /\\ q(x)) |- (exists x. p(x)) /\\ (exists x. q(x))",
            "forall x. forall y. r(x, y) |- forall y. forall x. r(x, y)",
            "forall x. p(f(x)) |- (exists x. p(x)) /\\ (exists x. p(x))",
            "forall x. (p(x) <-> q), p(a) |- q",
            "r(c) |- exists x. (r(x) /\\ exists u. (q(u) -> q(g(x))))",
        ];
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut next = |n: usize| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed % n as u64) as usize
        };
        let mut bound = 0;
        for round in 0..90 {
            let logic = [Logic::Classical, Logic::Intuitionistic, Logic::Minimal][round % 3];
            let (mut s, _) = start(SEQUENTS[round % SEQUENTS.len()], logic);
            for _ in 0..40 {
                let goals: Vec<GoalId> = s.goals().collect();
                match next(10) {
                    0 | 1 => {
                        s.undo();
                    }
                    #[cfg(feature = "serialize")]
                    2 => {
                        let json = serde_json::to_string(&s).unwrap();
                        s = serde_json::from_str(&json).unwrap_or_else(|e| panic!("{e}: {json}"));
                    }
                    _ if !goals.is_empty() => {
                        let g = goals[next(goals.len())];
                        let (left, right) = s.goal(g).unwrap();
                        let (side, len) = if next(2) == 0 {
                            (Left, left.len())
                        } else {
                            (Right, right.len())
                        };
                        if len == 0 {
                            continue;
                        }
                        let position = next(len);
                        let rules = s.rules(g, side, position).unwrap();
                        if rules.is_empty() {
                            continue;
                        }
                        let rule = rules[next(rules.len())].rule;
                        let before = s.bindings().len();
                        if s.apply(g, &Step::new(side, position, rule)).is_ok()
                            && s.bindings().len() > before
                        {
                            bound += 1;
                        }
                    }
                    _ => {}
                }
                if let Some(why) = walked_differs(&s) {
                    panic!("round {round}: {why}");
                }
            }
        }
        assert!(bound > 20, "only {bound} fillings");
    }

    /// A typed witness and every instance are held to the bound on terms
    /// by the measure the reader takes, so a session under a small bound
    /// writes documents its reader takes under it.
    #[cfg(feature = "serialize")]
    #[test]
    fn an_instance_is_held_to_the_bound_on_terms() {
        let within = |text: &str, terms| {
            let sequent: Sequent = text.parse().unwrap();
            let limits = Limits {
                terms: Some(terms),
                ..Limits::default()
            };
            (
                Interactive::within(&sequent, Logic::Classical, &limits).unwrap(),
                limits,
            )
        };
        let (mut s, _) = within("forall x. p(x, x) |- q", 4);
        let error = s.witness(GoalId::new(0), "f(f(f(f(a))))").unwrap_err();
        assert!(error.to_string().contains("5 symbols"), "{error}");
        let (mut s, _) = within("forall x. p(x, x), r(f(a)) |- q", 5);
        let witness = s.witness(GoalId::new(0), "f(a)").unwrap();
        let step = Step::new(Left, 0, Rule::ForallLeft).with_witness(witness);
        let error = s.apply(GoalId::new(0), &step).unwrap_err();
        assert!(
            matches!(error, StepError::Terms { limit: 5, .. }),
            "{error}"
        );
        // The review's session: whatever steps the bound leaves, the
        // document reads back under it.
        for terms in 3..8 {
            let (mut s, limits) = within(
                "forall x. p(x) |- (exists x. p(x)) /\\ (exists x. p(x))",
                terms,
            );
            let steps = [
                (0, Right, Rule::AndRight),
                (1, Right, Rule::ExistsRight),
                (3, Left, Rule::ForallLeft),
                (2, Right, Rule::ExistsRight),
                (5, Left, Rule::ForallLeft),
            ];
            for (goal, side, rule) in steps {
                let _ = s.apply(GoalId::new(goal), &Step::new(side, 0, rule));
            }
            let json = serde_json::to_string(&s).unwrap();
            let mut reader = serde_json::Deserializer::from_str(&json);
            let read: Result<Interactive, _> = crate::wire::upgrade(&mut reader, &limits);
            assert!(read.is_ok(), "terms {terms}: {:?}", read.err());
        }
    }
}
