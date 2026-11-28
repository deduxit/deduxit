// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of an ordinary tab, over JSON text as the linear calls are.
//! The page proves a sequent of classical, intuitionistic or minimal
//! logic step by step, by the rules of the sequent calculi LK (classical)
//! or LJ (intuitionistic and minimal).
//!
//! The session document is the library's session of ordinary sequents,
//! `{"version", "logic", "sequent", "inferences", "history"}`. The page
//! keeps it and sends it with every call. A formula of a goal is named by
//! its side of `⊢` and its position there.

use crate::Failure;
use crate::calls::{Automatic, Judged, inverting};
use crate::tab;
use deduxit::export::{latex, rocq, svg, typst};
use deduxit::ordinary::interpolant::{self, Partition};
use deduxit::ordinary::negative;
use deduxit::ordinary::{
    self, Applicable, Candidate, Context, Interactive, Logic, Method, Needs, NodeId, Rule, Sequent,
    Side, Status, Step, WrittenTableau,
};
use deduxit::ordinary::{normal, resolution};
use deduxit::{Settings, linear::GoalId, wire};
use serde::{Deserialize, Serialize};

/// An open goal as the page shows it.
#[derive(Serialize)]
struct OpenGoal {
    /// The goal's id, which every call on it takes.
    goal: GoalId,
    /// The formulas left of `⊢`, in the order of their positions, in the
    /// parser's Unicode syntax.
    left: Vec<String>,
    /// The formulas right of `⊢`, likewise.
    right: Vec<String>,
}

/// What the page shows of a session.
#[derive(Serialize)]
struct View {
    /// The host the stage draws the session into, as the protocol names
    /// it (`describe`): `tree` for a derivation.
    layout: &'static str,
    /// The session's logic.
    logic: Logic,
    /// Its calculus, `LK` or `LJ`.
    calculus: &'static str,
    /// Whether no goal is open.
    complete: bool,
    /// The steps taken.
    steps: usize,
    /// The open goals.
    goals: Vec<OpenGoal>,
    /// For each inference `i<n>` of the drawing, at index `n`, its goal id
    /// in the session. The drawing shows the structural rules as the
    /// settings' `view.structural` says.
    inferences: Vec<GoalId>,
    /// The goals the session's steps closed by a rule invertible in its
    /// logic (`Interactive::invertible_steps`), in order: what the page's
    /// undo goes past with `interactive.invertible` on.
    invertible: Vec<GoalId>,
    /// Every closed quantifier step with the term it took, as taken
    /// (`Interactive::opened_term`), so that a replay gives each witness
    /// again: left out for a proof without one.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    opened: Vec<OpenedWith>,
}

/// A quantifier step of a session and the term it took.
#[derive(Serialize)]
struct OpenedWith {
    /// The step's inference.
    inference: GoalId,
    /// The term as text, which the session's `witness` reads: an
    /// eigenvariable by its name, a witness given; `null` for a witness
    /// left open.
    term: Option<String>,
}

/// Returns a session's closed quantifier steps with their terms, in the
/// order of their inferences.
fn opened_with(state: &Interactive) -> Vec<OpenedWith> {
    // The explicit drawing shows every inference of the session.
    let mut goals = state.derivation_ids(deduxit::linear::proofs::Structural::Explicit);
    goals.sort_unstable();
    goals
        .into_iter()
        .filter_map(|goal| {
            let term = state.opened_term(goal)?;
            // An open witness is written with a `$`, which no term holds.
            let term = (!term.starts_with('$')).then_some(term);
            Some(OpenedWith {
                inference: goal,
                term,
            })
        })
        .collect()
}

impl View {
    /// Returns the view of a session under the settings' view.
    fn of(state: &Interactive, settings: &Settings) -> Self {
        let formulas = state.formulas();
        let texts = |ids: &[NodeId]| -> Vec<String> {
            ids.iter()
                .map(|&id| formulas.formula(id).to_string())
                .collect()
        };
        let goals = state
            .goals()
            .filter_map(|goal| {
                let (left, right) = state.goal(goal).ok()?;
                Some(OpenGoal {
                    goal,
                    left: texts(left),
                    right: texts(right),
                })
            })
            .collect();
        Self {
            layout: "tree",
            logic: state.logic(),
            calculus: state.logic().calculus(),
            complete: state.is_complete(),
            steps: state.steps(),
            goals,
            inferences: state.derivation_ids(settings.view.structural),
            invertible: state.invertible_steps(),
            opened: opened_with(state),
        }
    }
}

/// A session after a call that changed it, with its view and what the
/// call says beyond it.
#[derive(Serialize)]
struct Changed<'a, T: Serialize> {
    /// The session, which the page keeps for the next call.
    session: &'a Interactive,
    /// What the page shows of it.
    view: View,
    /// What the call says beyond the session.
    #[serde(flatten)]
    said: T,
}

/// Returns a session's document with its view and what a call said.
fn changed(
    state: &Interactive,
    settings: &Settings,
    said: impl Serialize,
) -> Result<String, Failure> {
    written(&Changed {
        session: state,
        view: View::of(state, settings),
        said,
    })
}

/// Starts a proof of a sequent of ordinary logic, and returns the session
/// with its view, `{"session", "view"}`.
///
/// The sequent is typed in the syntax of ordinary formulas, or is the
/// ordinary sequent's JSON. The logic is `classical`, `intuitionistic` or
/// `minimal`. An intuitionistic or minimal sequent with nothing right of
/// `⊢` is read as `Γ ⊢ ⊥`.
///
/// # Errors
///
/// A sequent that cannot be read or is past `limits.occurrences`, an
/// unknown logic, and more than one formula right of `⊢` in
/// intuitionistic or minimal logic.
pub fn start(sequent: &str, logic: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let logic: Logic = logic.parse().map_err(Failure::from)?;
    let sequent = sequent_of(sequent, &settings)?;
    let mut state =
        Interactive::within(&sequent, logic, &settings.limits).map_err(Failure::from)?;
    let mut automatic = Automatic::default();
    if settings.interactive.invertible {
        automatic.add(state.invert_all(&inverting(&settings)));
    }
    changed(&state, &settings, automatic)
}

/// Returns what the page shows of a session: the `view` that every call
/// that changes the session answers too.
///
/// # Errors
///
/// As the session's reader.
pub fn view(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    written(&View::of(&state, &settings))
}

/// A rule a formula takes, as the page lists it.
#[derive(Serialize)]
struct Offered {
    /// The rule's name.
    rule: Rule,
    /// Whether it has two premises. They share the goal's other formulas
    /// unless a step splits them.
    split: bool,
    /// Whether a step of it takes a witness term (`∀L`, `∃R`); written only
    /// then.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    witness: bool,
}

impl From<Applicable> for Offered {
    /// The rule and whether a step of it can take a split.
    fn from(applicable: Applicable) -> Self {
        Self {
            rule: applicable.rule,
            split: matches!(applicable.needs, Needs::Context),
            witness: matches!(applicable.needs, Needs::Witness),
        }
    }
}

/// Returns the rules that act on the formula at a position of a side
/// (`left` or `right`) of a goal, by its connective and the logic:
/// `[{"rule", "split"}]`. [`apply`] may still refuse a rule listed, with
/// the reason: for the step's split, or in LJ for what its premises would
/// hold right of `⊢`.
///
/// # Errors
///
/// An unknown side, and the step error for a goal that is not open or a
/// position outside its side.
pub fn rules(
    session: &str,
    goal: u32,
    side: &str,
    position: u32,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let rules = state
        .rules(GoalId::new(goal), side_of(side)?, position as usize)
        .map_err(|e| Failure::from(deduxit::Error::from(e)))?;
    written(&rules.into_iter().map(Offered::from).collect::<Vec<_>>())
}

/// Returns every rule of the logic that acts on the formula at a position
/// of a side of a goal, each with whether a step of it is legal on the
/// goal as it stands (`Interactive::candidates`): `[{"rule", "status"}]`,
/// `status` `applies`, `needs_input` with `needs` (`context` for two
/// premises, which share the goal's other formulas unless a step splits
/// them; `witness` for `∀L` and `∃R`) where the step with a shared context
/// and a witness left open is legal, or `refused` with `refusal`, the
/// error [`apply`] throws for that step. Whether the goals a step opens
/// can be closed is not judged.
///
/// # Errors
///
/// An unknown side, and the step error for a goal that is not open or a
/// position outside its side.
pub fn candidates(
    session: &str,
    goal: u32,
    side: &str,
    position: u32,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let candidates = state
        .candidates(GoalId::new(goal), side_of(side)?, position as usize)
        .map_err(|e| Failure::from(deduxit::Error::from(e)))?;
    let judged: Vec<Judged<Rule, &str>> = candidates
        .into_iter()
        .map(|Candidate { rule, status, .. }| match status {
            Status::NeedsInput { needs, .. } => Judged::needs(
                rule,
                match needs {
                    Needs::Context => "context",
                    Needs::Witness => "witness",
                    _ => "input",
                },
            ),
            Status::Refused(error) => Judged::refused(rule, &deduxit::Error::from(error)),
            _ => Judged::applies(rule),
        })
        .collect();
    written(&judged)
}

/// A step as the page sends it: `{"side", "position", "rule", "split",
/// "witness"}`.
///
/// - `rule` is the rule's name, such as `"→L"`.
/// - `witness`, for `∀L` and `∃R`, is the witness's text, which the
///   session reads (`Interactive::witness`: the sequent's function symbols
///   and the eigenvariables in the goal's scope by their printed names).
/// - `split`, for a rule with two premises, is the positions on each side
///   that go to the first premise: `{"left": [..], "right": [..]}`. It is
///   left out when both premises share the goal's other formulas.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepForm {
    /// The side of the formula the rule acts on.
    pub side: Side,
    /// Its position on that side.
    pub position: usize,
    /// The rule.
    pub rule: Rule,
    /// The positions going to the first premise, or none for premises that
    /// share the goal's other formulas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<SplitForm>,
    /// The witness's text, for `∀L` and `∃R`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub witness: Option<String>,
}

/// The formulas a split sends to the first premise of a rule with two,
/// by their positions on each side; the others go to the second.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SplitForm {
    /// The positions left of `⊢`.
    #[serde(default)]
    pub left: Vec<usize>,
    /// The positions right of `⊢`.
    #[serde(default)]
    pub right: Vec<usize>,
}

impl StepForm {
    /// Returns the library's step.
    fn step(&self) -> Step {
        let step = Step::new(self.side, self.position, self.rule);
        match &self.split {
            Some(split) => step.split(&split.left, &split.right),
            None => step,
        }
    }

    /// Returns the form of a library's step on `state`, its witness as
    /// text the session reads.
    fn of(step: &Step, state: &Interactive) -> Self {
        let split = match &step.context {
            Context::Split { left, right, .. } => Some(SplitForm {
                left: left.clone(),
                right: right.clone(),
            }),
            _ => None,
        };
        Self {
            side: step.side,
            position: step.position,
            rule: step.rule,
            split,
            witness: step.witness.and_then(|t| state.term_text(t)),
        }
    }
}

/// What a step opened.
#[derive(Serialize)]
struct Opened {
    /// The goals the step opened, none when it closed its goal. With
    /// `interactive.invertible` on, the goals the invertible steps after it
    /// left open, from left to right.
    opened: Vec<GoalId>,
    /// The invertible steps taken after it.
    #[serde(flatten)]
    automatic: Automatic,
}

/// Applies a step ([`StepForm`]) to an open goal and returns the session
/// with `opened`, the goals the step opened.
///
/// # Errors
///
/// A step that is no such JSON. The step error when the rule does not act
/// on the formula, the logic lacks it, or its split does not fit.
pub fn apply(session: &str, goal: u32, step: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let step: StepForm = serde_json::from_str(step).map_err(Failure::json)?;
    let goal = GoalId::new(goal);
    let mut library = step.step();
    if let Some(text) = &step.witness {
        let witness = state
            .witness(goal, text)
            .map_err(|e| Failure::from(deduxit::Error::from(e)))?;
        library = library.with_witness(witness);
    }
    let mut opened = state
        .apply(goal, &library)
        .map_err(|e| Failure::from(deduxit::Error::from(e)))?;
    let mut automatic = Automatic::default();
    if settings.interactive.invertible {
        let limits = inverting(&settings);
        for goal in std::mem::take(&mut opened) {
            let inverted = state
                .invert(goal, &limits)
                .map_err(|e| Failure::from(deduxit::Error::from(e)))?;
            opened.extend(automatic.add(inverted));
        }
    }
    changed(&state, &settings, Opened { opened, automatic })
}

/// Applies every invertible rule to every open goal
/// (`Interactive::invert_all`), whatever the settings say, and returns
/// the session with `inverted`, how many steps that took: what the page
/// calls when `interactive.invertible` is turned on during a session.
/// LK's rules of the connectives are invertible; LJ's `→L`, `¬L` and
/// `∨R₁`, `∨R₂` are not.
///
/// # Errors
///
/// As the session's reader.
pub fn invert_all(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let mut automatic = Automatic::default();
    automatic.add(state.invert_all(&inverting(&settings)));
    changed(&state, &settings, automatic)
}

/// What an undo reopened.
#[derive(Serialize)]
struct Reopened {
    /// The goal the undo reopened, `null` when no step was left.
    reopened: Option<GoalId>,
}

/// Takes back the session's last step: a rule applied, or the whole
/// derivation the search grafted onto a goal. Returns the session with
/// `reopened`, the goal, or `null` when no step was left.
///
/// # Errors
///
/// As the session's reader.
pub fn undo(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let reopened = state.undo();
    changed(&state, &settings, Reopened { reopened })
}

/// What closing a goal did.
#[derive(Serialize)]
struct Closing {
    /// The outcome of deciding the goal as a sequent of its own, as
    /// `prove --logic … --format json` writes one. Its derivation is the
    /// goal's.
    outcome: ordinary::Outcome,
    /// `null` when the derivation was grafted or there was none.
    /// Otherwise the refusal of the graft by its size, and the goal stays
    /// open.
    refused: Option<serde_json::Value>,
}

/// Decides a goal within the settings' bounds, and grafts its derivation
/// onto the goal when the goal is valid. Answers the session with
/// `outcome` and `refused`. A goal not valid or undecided stays open, and
/// the outcome says why.
///
/// A propositional goal is decided through its translation into linear
/// logic, the one the settings' `ordinary.translation` names, or the
/// logic's default when that is `null`. A goal of a first-order session
/// is decided by the first-order methods, a proof search and then a
/// search for a finite countermodel, its eigenvariables constants there:
/// a derivation found is checked before it is grafted, and a checked
/// structure refutes the goal.
///
/// # Errors
///
/// A goal that is not open, a translation that does not decide the
/// session's logic, and the refusals of the search's bounds.
pub fn close(session: &str, goal: u32, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let closed = state
        .close(
            GoalId::new(goal),
            settings.ordinary.translation,
            &settings.search,
            &settings.limits,
            crate::unstopped(),
        )
        .map_err(Failure::from)?;
    let refused = closed
        .grafted
        .err()
        .and_then(|refusal| serde_json::to_value(deduxit::Error::Refused(refusal)).ok());
    let mut outcome = closed.outcome;
    with_countermodel(&mut outcome, &state, GoalId::new(goal), &settings);
    changed(&state, &settings, Closing { outcome, refused })
}

/// Gives a "not valid" outcome without a countermodel the SAT solver's
/// model of the goal, as the command's default search finds one: the
/// session decides a goal through the embedding, which refutes the
/// goal's image in linear logic and has no Kripke model in intuitionistic
/// and minimal logic. Where the solver gives none within the settings'
/// limits, the outcome stays as it is.
fn with_countermodel(
    outcome: &mut ordinary::Outcome,
    state: &Interactive,
    goal: GoalId,
    settings: &Settings,
) {
    if !matches!(outcome.verdict, ordinary::Verdict::NotValid)
        || outcome.countermodel.is_some()
        || state.formulas().is_first_order()
    {
        return;
    }
    let Ok((left, right)) = state.goal(goal) else {
        return;
    };
    let Ok(sequent) = Sequent::new(state.formulas().clone(), left.to_vec(), right.to_vec()) else {
        return;
    };
    if let Ok(found) = ordinary::sat::decide(
        &sequent,
        state.logic(),
        &settings.limits,
        crate::unstopped(),
    ) && matches!(found.verdict, ordinary::Verdict::NotValid)
    {
        outcome.countermodel = found.model;
    }
}

/// What a hint found.
#[derive(Serialize)]
struct Hint {
    /// The outcome of deciding the goal.
    outcome: ordinary::Outcome,
    /// The first steps of the derivation found, as [`apply`] takes them in
    /// turn: the weakenings and contractions at its root, then the first
    /// rule that is neither, which ends the list. The first step is on the
    /// goal asked for, and each later one on the goal the step before it
    /// opened. Empty unless the goal is valid.
    steps: Vec<StepForm>,
}

/// Decides a goal as [`close`] does and returns the outcome with the
/// first steps of the derivation found, `{"outcome", "steps"}`. The
/// session stays as it was. The page applies the steps in turn. A
/// contraction comes with the rule that uses both copies, so that
/// following the hints proves the goal.
///
/// # Errors
///
/// As [`close`].
pub fn hint(session: &str, goal: u32, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let hint = state
        .hint(
            GoalId::new(goal),
            settings.ordinary.translation,
            &settings.search,
            &settings.limits,
            crate::unstopped(),
        )
        .map_err(Failure::from)?;
    let mut outcome = hint.outcome;
    with_countermodel(&mut outcome, &state, GoalId::new(goal), &settings);
    written(&Hint {
        steps: hint
            .steps
            .iter()
            .map(|step| StepForm::of(step, &state))
            .collect(),
        outcome,
    })
}

/// What the check of a Kripke model found.
#[derive(Serialize)]
struct Checked {
    /// Whether the sequent holds at the world asked, or at every world.
    holds: bool,
    /// Where it does not: a world at or above the one asked that forces
    /// each formula left of `⊢` and not the one right of it, and what it
    /// forces of every subformula, each before the formulas made of it.
    failing: Option<Failing>,
}

/// A world where a sequent fails, and what it forces.
#[derive(Serialize)]
struct Failing {
    /// The world, by index.
    world: u32,
    /// Each subformula of the sequent and whether the world forces it.
    forcing: Vec<Forced>,
}

/// Whether a world forces a formula.
#[derive(Serialize)]
struct Forced {
    /// The formula, as the page prints formulas.
    formula: String,
    /// Whether the world forces it.
    forced: bool,
}

/// Checks a Kripke model, in its JSON form (`{"worlds": [{"atoms",
/// "falsity", "arrows"}], "world"}`), against a sequent of `logic`: that
/// it is a model of the logic, then whether the sequent holds at `world`,
/// or at every world when `world` is `None`. Answers `{"holds",
/// "failing"}`, `failing` the world where the sequent fails, with what it
/// forces of every subformula, or `null`.
///
/// # Errors
///
/// A model or a sequent that cannot be read, an unknown logic, a world
/// the model does not have, and a model that is not one of the logic
/// (code `invalid_model`, its reason).
pub fn model(
    model: &str,
    sequent: &str,
    logic: &str,
    world: Option<u32>,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let logic: Logic = logic.parse().map_err(Failure::from)?;
    let sequent = sequent_of(sequent, &settings)?;
    let mut document = serde_json::Deserializer::from_str(model);
    let model: deduxit::kripke::Model =
        wire::upgrade(&mut document, &settings.limits).map_err(Failure::from)?;
    if let Some(w) = world {
        model.check_world(w).map_err(Failure::from)?;
    }
    let forcing = model
        .forcing(&sequent, logic, &settings.limits, crate::unstopped())
        .map_err(Failure::from)?;
    let failing = match world {
        Some(w) => forcing.failing(w),
        None => forcing.failing_anywhere(),
    };
    let formulas = sequent.formulas();
    let failing = failing.map(|w| Failing {
        world: w,
        forcing: forcing
            .at(w)
            .map(|(id, forced)| Forced {
                formula: formulas.formula(id).to_string(),
                forced,
            })
            .collect(),
    });
    written(&Checked {
        holds: failing.is_none(),
        failing,
    })
}

/// What a finite structure makes of a first-order sequent.
#[derive(Serialize)]
struct Evaluated {
    /// Whether the structure refutes the sequent in its logic, by
    /// `FiniteCountermodel::check`.
    refutes: bool,
    /// The check's reason where it does not: the first formula the
    /// structure gets wrong.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    /// Each formula left of `⊢` and whether the structure makes it true.
    left: Vec<Valued>,
    /// Each formula right of `⊢`, likewise.
    right: Vec<Valued>,
}

/// A formula of a sequent and its value in a structure.
#[derive(Serialize)]
struct Valued {
    /// The formula, as the page prints formulas.
    formula: String,
    /// Whether the structure makes it true.
    #[serde(rename = "true")]
    holds: bool,
}

/// Checks a finite structure, in its JSON form (`{"size", "constants",
/// "functions", "predicates"}`), against a first-order sequent of `logic`.
/// Answers `{"refutes", "reason", "left", "right"}`: whether the structure
/// refutes the sequent, the check's reason where it does not, and each
/// formula with whether it is true there (`{"formula", "true"}`), each
/// evaluated as a sequent of that formula alone.
///
/// # Errors
///
/// A structure or a sequent that cannot be read, an unknown logic, a
/// structure that cannot interpret the sequent (code
/// `invalid_structure`, its reason), and the evaluation past the
/// settings' `limits.work` or `limits.memory_bytes`.
pub fn structure(
    structure: &str,
    sequent: &str,
    logic: &str,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let logic: Logic = logic.parse().map_err(Failure::from)?;
    let sequent = sequent_of(sequent, &settings)?;
    let mut document = serde_json::Deserializer::from_str(structure);
    let structure: ordinary::Structure =
        serde::Deserialize::deserialize(&mut document).map_err(Failure::json)?;
    document.end().map_err(Failure::json)?;
    // Each formula as the one hypothesis of a sequent: the structure
    // refutes `A ⊢` exactly where it makes `A` true.
    let true_in = |formula: NodeId| -> Result<Valued, Failure> {
        let alone = Sequent::new(sequent.formulas().clone(), vec![formula], Vec::new())
            .map_err(Failure::from)?;
        let evaluation = structure
            .refutes(&alone, &settings.limits, crate::unstopped())
            .map_err(Failure::from)?;
        Ok(Valued {
            formula: sequent.formulas().formula(formula).to_string(),
            holds: evaluation == ordinary::Evaluation::Refutes,
        })
    };
    let left = sequent
        .left()
        .iter()
        .map(|&f| true_in(f))
        .collect::<Result<_, _>>()?;
    let right = sequent
        .right()
        .iter()
        .map(|&f| true_in(f))
        .collect::<Result<_, _>>()?;
    // Every formula was interpreted above, so a refusal of the check is
    // the structure not refuting the sequent.
    let checked = ordinary::FiniteCountermodel::new(sequent, structure).check(
        logic,
        &settings.limits,
        crate::unstopped(),
    );
    let reason = match checked {
        Ok(()) => None,
        Err(error @ deduxit::Error::Structure { .. }) => Some(error.to_string()),
        Err(error) => return Err(Failure::from(error)),
    };
    written(&Evaluated {
        refutes: reason.is_none(),
        reason,
        left,
        right,
    })
}

/// Decides a sequent of classical logic by its truth table and answers
/// the outcome: `{"version": 14, "verdict", "logic", "search": {"method":
/// "table", "table": {"sequent", "rows", "falsifying"}}, "countermodel"}`,
/// a row a `T` or `F` per formula of the sequent's form, from every atom
/// true to every atom false; `countermodel` the first falsifying row,
/// checked, for `not_valid`.
///
/// # Errors
///
/// A sequent that cannot be read, and more atoms than the settings'
/// `limits.table_atoms` (code `too_many_atoms`).
pub fn table(sequent: &str, settings: &str) -> Result<String, Failure> {
    decided(sequent, Method::Table, settings)
}

/// Decides a sequent of classical logic by a signed tableau and answers
/// the outcome: `{"version": 14, "verdict", "logic", "search": {"method":
/// "tableau", "tableau": {"sequent", "nodes"}}, "derivation" |
/// "countermodel"}`, the nodes each before its children, each with its
/// `lines` (`["T", formula]`), and `expands` and `children`, or `end`; a
/// closed tableau's derivation of LK and an open branch's countermodel,
/// both checked.
///
/// # Errors
///
/// A sequent that cannot be read, and a refusal of a bound of the
/// settings.
pub fn tableau(sequent: &str, settings: &str) -> Result<String, Failure> {
    decided(sequent, Method::Tableau, settings)
}

/// Reviews a signed tableau of classical logic written by hand, as text,
/// step by step against the built-in rule file `tableau` and, where
/// `sequent` is not `""`, against that sequent (text or JSON); answers the
/// review: `{"version": 14, "standing", "sequent", "text_line", "line",
/// "message", "countermodel"}`, `standing` being `closed`, `open`,
/// `unfinished` or `wrong`, `text_line` the line of the text (from 1) that
/// `message` is about, `line` the tableau's own number for it, and
/// `countermodel` the checked model of an open tableau.
///
/// # Errors
///
/// A text that is no tableau, a failure of code `parse` placed in the
/// text with a hint; a sequent that cannot be read; the settings' bounds.
pub fn tableau_review(text: &str, sequent: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let tableau = WrittenTableau::parse_within(text, &settings.limits).map_err(Failure::from)?;
    let expected = if sequent.trim().is_empty() {
        None
    } else {
        Some(sequent_of(sequent, &settings)?)
    };
    let mut json = serde_json::Deserializer::from_str(deduxit::rules::TABLEAU);
    let calculus: deduxit::rules::Calculus =
        wire::upgrade(&mut json, &settings.limits).map_err(Failure::from)?;
    let review = tableau
        .review(
            &calculus,
            expected.as_ref(),
            &settings.limits,
            crate::unstopped(),
        )
        .map_err(Failure::from)?;
    written(&review)
}

/// Translates a sequent of classical logic by a negative translation,
/// decides it in classical logic and its translation in the target logic,
/// and answers the comparison: `{"version": 16, "translation", "target",
/// "agreement", "sequent", "translated", "classical", "decided"}`, both
/// outcomes with their evidence, `agreement` being `valid`, `not_valid`
/// or `unknown`. `options` is `{"translation": "kolmogorov", "target":
/// "minimal"}`, every key optional (the translation `kolmogorov`, the
/// target the translation's own), or `""` for the defaults.
///
/// # Errors
///
/// A sequent or options that cannot be read, a target the translation
/// does not reach (code `negative_target`), and verdicts that differ
/// (code `claim_fails`, a defect).
pub fn translate(sequent: &str, options: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let sequent = sequent_of(sequent, &settings)?;
    let options: negative::Options = if options.trim().is_empty() {
        negative::Options::default()
    } else {
        serde_json::from_str(options).map_err(Failure::json)?
    };
    let search = deduxit::linear::search::Options::default();
    let compared = negative::compare(
        &sequent,
        &options,
        &search,
        &settings.limits,
        crate::unstopped(),
    )
    .map_err(Failure::from)?;
    written(&compared)
}

/// Decides a sequent of `logic` and answers its interpolant for
/// `partition`: `{"version": 16, "logic", "partition", "outcome",
/// "interpolant": {"text", "first": {"sequent", "outcome"}, "second":
/// {…}}}`, the interpolant absent for a sequent that is not valid or not
/// decided. `partition` is `{"left": [0], "right": []}`, the positions of
/// the first half's formulas on each side, or `""` for every hypothesis
/// first. Both halves are decided again and the interpolant's atoms
/// compared before it is answered.
///
/// # Errors
///
/// A sequent, logic or partition that cannot be read, a partition that is
/// not one of the sequent (code `partition`), and an interpolant that
/// fails its check (code `claim_fails`, a defect).
pub fn interpolate(
    sequent: &str,
    logic: &str,
    partition: &str,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let logic: Logic = logic.parse().map_err(Failure::from)?;
    let sequent = sequent_of(sequent, &settings)?;
    let partition = if partition.trim().is_empty() {
        Partition::hypotheses(&sequent)
    } else {
        serde_json::from_str(partition).map_err(Failure::json)?
    };
    let options = deduxit::ordinary::Options::default().with_logic(logic);
    let search = deduxit::linear::search::Options::default();
    let found = interpolant::interpolate(
        &sequent,
        &options,
        &partition,
        &search,
        &settings.limits,
        crate::unstopped(),
    )
    .map_err(Failure::from)?;
    written(&found)
}

/// Rewrites a sequent of classical logic into a normal form step by step
/// and checks the form's promise, answering the normal form: `{"version":
/// 21, "form", "promise", "checked", "sequent", "result", "steps":
/// [{"rule", "law", "before", "after", "left", "right", "added"}, …],
/// "checks"}`, the steps' numbers indices of `result.formulas`, the
/// sequent after a step its `left` with the clauses `added` by it and by
/// every step before it. `options` is `{"form": "cnf"}` (`nnf`, `cnf`, `dnf`, `tseitin`,
/// `plaisted-greenbaum`), or `""` for conjunctive normal form.
///
/// # Errors
///
/// A sequent or options that cannot be read, a form past
/// `limits.form_occurrences` (code `form_too_large`), and a promise found
/// false (code `claim_fails`, a defect).
pub fn normalize(sequent: &str, options: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let sequent = sequent_of(sequent, &settings)?;
    let options: normal::Options = if options.trim().is_empty() {
        normal::Options::default()
    } else {
        serde_json::from_str(options).map_err(Failure::json)?
    };
    let search = deduxit::linear::search::Options::default();
    let made = normal::normalize(
        &sequent,
        &options,
        &search,
        &settings.limits,
        crate::unstopped(),
    )
    .map_err(Failure::from)?;
    written(&made)
}

/// How [`resolve`] makes and decides the clauses.
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Resolving {
    /// The form that turns the sequent's negation into clauses.
    form: normal::Form,
    /// The method.
    method: resolution::Method,
    /// The bound on clauses, `null` for none.
    clauses: Option<u64>,
    /// The bound on steps, `null` for none.
    steps: Option<u64>,
}

impl Default for Resolving {
    /// Conjunctive normal form, saturation, the library's bounds.
    fn default() -> Self {
        let options = resolution::Options::default();
        Self {
            form: normal::Form::Cnf,
            method: options.method,
            clauses: options.clauses,
            steps: options.steps,
        }
    }
}

/// Turns a sequent of classical logic's negation into clauses and decides
/// them, answering the clause set decided: `{"version": 22, "method",
/// "verdict", "atoms", "clauses", "resolvents", "true_atoms", "bound",
/// "trace", "statistics"}`, `verdict` `unsatisfiable` (the sequent is
/// valid) with the checked refutation's `resolvents` (each `{"first",
/// "second", "atom", "clause"}`: the clauses by number from 0, the atom
/// from 1), `satisfiable` with a checked model's `true_atoms`, or
/// `unknown` with the `bound`; `trace` DPLL's steps. `options` is
/// `{"form": "cnf", "method": "saturation", "clauses": 2000, "steps":
/// 100000}`, every key optional, or `""` for those defaults.
///
/// # Errors
///
/// A sequent or options that cannot be read, a form past
/// `limits.form_occurrences`, and an answer or a rewriting that fails its
/// check (code `claim_fails`, a defect).
pub fn resolve(sequent: &str, options: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let sequent = sequent_of(sequent, &settings)?;
    let options: Resolving = if options.trim().is_empty() {
        Resolving::default()
    } else {
        serde_json::from_str(options).map_err(Failure::json)?
    };
    let search = deduxit::linear::search::Options::default();
    let (clauses, _) = resolution::clauses_of(
        &sequent,
        options.form,
        &search,
        &settings.limits,
        crate::unstopped(),
    )
    .map_err(Failure::from)?;
    let decide = resolution::Options::default()
        .with_method(options.method)
        .with_clauses(options.clauses)
        .with_steps(options.steps);
    let answer = resolution::decide(&clauses, &decide, &settings.limits, crate::unstopped())
        .map_err(Failure::from)?;
    written(&answer)
}

/// Starts a refutation by hand: the clauses of a classical sequent's
/// negation by `form` (`cnf`, `tseitin` or `plaisted-greenbaum`; `""` for
/// `cnf`), as a refutation with no resolvent yet: `{"version": 22,
/// "atoms", "clauses", "resolvents": []}`.
///
/// # Errors
///
/// A sequent or form that cannot be read, a form that gives no clauses
/// (code `invalid_option`), and a rewriting that fails its check.
pub fn clauses(sequent: &str, form: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let sequent = sequent_of(sequent, &settings)?;
    let form: normal::Form = if form.trim().is_empty() {
        normal::Form::Cnf
    } else {
        form.parse().map_err(Failure::from)?
    };
    let search = deduxit::linear::search::Options::default();
    let (clauses, _) = resolution::clauses_of(
        &sequent,
        form,
        &search,
        &settings.limits,
        crate::unstopped(),
    )
    .map_err(Failure::from)?;
    written(&resolution::Refutation::new(clauses))
}

/// Reads a refutation the page holds, checking its numbers.
fn refutation_of(refutation: &str, settings: &Settings) -> Result<resolution::Refutation, Failure> {
    let mut json = serde_json::Deserializer::from_str(refutation);
    wire::upgrade(&mut json, &settings.limits).map_err(Failure::from)
}

/// Takes a step of a refutation by hand: resolves the clauses numbered
/// `first` and `second` (from 0) on the atom numbered `atom` (from 1, as
/// the document numbers atoms), one holding the atom and the other its
/// negation, and answers `{"refutation": {…}, "added": n, "finished":
/// bool}`: the refutation with the resolvent added as clause `n`, and
/// whether the empty clause is derived. The refutation read is checked
/// again first, so a document changed by hand is refused.
///
/// # Errors
///
/// A refutation that cannot be read or does not check
/// (`invalid_clause_refutation`), a number that is no integer, and a step
/// the clauses do not allow (code `step_refused`).
pub fn resolve_step(
    refutation: &str,
    first: f64,
    second: f64,
    atom: f64,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut refutation = refutation_of(refutation, &settings)?;
    refutation
        .check_steps(&settings.limits, crate::unstopped())
        .map_err(Failure::from)?;
    let first = crate::calls::index(first, "first")?;
    let second = crate::calls::index(second, "second")?;
    let atom = crate::calls::index(atom, "atom")?;
    // Atoms are counted from 1; a 0 names no atom, which `resolve` refuses.
    let atom = atom.checked_sub(1).unwrap_or(u32::MAX);
    let added = refutation
        .resolve(first as usize, second as usize, atom)
        .map_err(Failure::from)?;
    let answer = serde_json::json!({
        "refutation": refutation,
        "added": added,
        "finished": refutation.is_finished(),
    });
    Ok(answer.to_string())
}

/// Checks a refutation the page holds and answers `{"finished": bool}`:
/// whether the empty clause is derived. Every step is checked, finished
/// or not.
///
/// # Errors
///
/// A refutation that cannot be read, and one whose step does not hold
/// (code `invalid_clause_refutation`).
pub fn refutation_check(refutation: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let refutation = refutation_of(refutation, &settings)?;
    refutation
        .check_steps(&settings.limits, crate::unstopped())
        .map_err(Failure::from)?;
    Ok(serde_json::json!({"finished": refutation.is_finished()}).to_string())
}

/// Decides a sequent of classical logic by `method` and answers the
/// outcome as JSON.
fn decided(sequent: &str, method: Method, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let sequent = sequent_of(sequent, &settings)?;
    let options = deduxit::ordinary::Options::default()
        .with_logic(Logic::Classical)
        .with_method(Some(method));
    let search = deduxit::linear::search::Options::default();
    let outcome = deduxit::ordinary::decide_within(
        &sequent,
        &options,
        &search,
        &settings.limits,
        crate::unstopped(),
    )
    .map_err(Failure::from)?;
    written(&outcome)
}

/// Returns the lambda-term of a session's derivation
/// ([`ordinary::Derivation::term`]). A finished derivation of LJ gives a
/// term of the simply typed lambda calculus.
///
/// The answer is as the linear [`calls::term`](crate::calls::term)
/// answers: the term's JSON with its `text` and `judgement`. Where there
/// is no term, it is `{"error": …}`: `no_term` for a derivation of LK or
/// one with open goals, or the refusal of a bound of the settings.
///
/// # Errors
///
/// A session the reader refuses.
pub fn term(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let term = state
        .derivation_within(&settings.view, &settings.limits, crate::unstopped())
        .and_then(|derivation| derivation.term(&settings.limits, crate::unstopped()));
    crate::calls::termed(term, &settings)
}

/// Returns a session's derivation, open goals as bare leaves, as an SVG
/// drawing in the settings' style (`styles.svg`). It shows the structural
/// rules as `view.structural` says. Every inference has the id `i<n>`,
/// and the view's `inferences` names its goal. It is [`export`] in `svg`.
///
/// # Errors
///
/// As [`export`].
pub fn draw(session: &str, settings: &str) -> Result<String, Failure> {
    export(session, "svg", settings)
}

/// Returns a session's derivation in a format:
///
/// - `text`, `latex`, `typst` or `svg`: open goals as bare leaves, and
///   the structural rules as the settings' `view.structural` says;
/// - `rocq`: the certificate of the finished derivation over `Prop`,
///   checked first, with every structural rule in it.
///
/// For PNG and PDF, the renderers' module (`deduxit-web-render`) renders
/// the SVG drawing.
///
/// # Errors
///
/// An unknown format, a derivation past `limits.derivation_bytes`,
/// `open_goals` for a certificate of an unfinished proof, and what the
/// format refuses.
pub fn export(session: &str, format: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let (limits, styles) = (&settings.limits, &settings.styles);
    let mut out = String::new();
    if format == "rocq" {
        let derivation = state
            .finished(limits, crate::unstopped())
            .map_err(Failure::from)?;
        rocq::write(&derivation, &styles.rocq, &mut out, crate::unstopped())
            .map_err(Failure::from)?;
        return Ok(out);
    }
    let derivation = state
        .derivation_within(&settings.view, limits, crate::unstopped())
        .map_err(Failure::from)?;
    match format {
        "text" => derivation.write_text(&styles.text, &mut out, crate::unstopped()),
        "latex" => latex::write(&derivation, &styles.latex, &mut out, crate::unstopped()),
        "typst" => typst::write(&derivation, &styles.typst, &mut out, crate::unstopped()),
        "svg" => svg::write(&derivation, &styles.svg, &mut out, crate::unstopped()),
        _ => {
            return Err(Failure::refused(
                "unknown_name",
                "malformed",
                "the formats are text, latex, typst, svg and rocq",
            ));
        }
    }
    .map_err(Failure::from)?;
    Ok(out)
}

/// Reads an ordinary outcome (as [`close`] and [`hint`] answer it, or
/// as `prove --logic --format json` writes it) and returns its
/// countermodel and logic.
fn countermodel_of(
    outcome: &str,
    settings: &Settings,
) -> Result<(ordinary::Countermodel, Logic), Failure> {
    let mut json = serde_json::Deserializer::from_str(outcome);
    let outcome: ordinary::Outcome =
        wire::upgrade(&mut json, &settings.limits).map_err(Failure::from)?;
    match outcome.countermodel {
        Some(countermodel) => Ok((countermodel, outcome.logic)),
        None => Err(Failure::refused(
            "no_countermodel",
            "unsupported",
            "the outcome holds no countermodel: only a sequent found not valid has one, and a \
             model too large to check is left out",
        )),
    }
}

/// Draws the countermodel of an ordinary outcome as SVG: each world a
/// box with the atoms it forces, an arrow up to each world directly
/// above it, and the model's world, where the sequent fails, outlined.
/// The model is checked first.
///
/// # Errors
///
/// An outcome that cannot be read, `no_countermodel` for one without a
/// countermodel, `invalid_model` for a model that does not refute its
/// sequent, and a drawing past `limits.derivation_bytes`.
pub fn model_drawing(outcome: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let (countermodel, logic) = countermodel_of(outcome, &settings)?;
    svg::countermodel(
        &countermodel,
        logic,
        &settings.styles.svg,
        &settings.limits,
        crate::unstopped(),
    )
    .map_err(Failure::from)
}

/// Writes the Rocq certificate of the countermodel of an ordinary
/// outcome, in the settings' `styles.rocq`: in intuitionistic and minimal
/// logic a lemma of deduxit's Rocq library that LJ or LM does not derive
/// the sequent, proved from the model; in classical logic the
/// certificate over `Prop` of the model's one world. The model is
/// checked first.
///
/// # Errors
///
/// As [`model_drawing`], the text past `limits.derivation_bytes`.
pub fn model_certificate(outcome: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let (countermodel, logic) = countermodel_of(outcome, &settings)?;
    let mut out = String::new();
    rocq::write_countermodel(
        &countermodel,
        logic,
        &settings.styles.rocq,
        &settings.limits,
        &mut out,
        crate::unstopped(),
    )
    .map_err(Failure::from)?;
    Ok(out)
}

/// What [`read`] found: the sequent as JSON and as text.
#[derive(Serialize)]
struct Read {
    /// `sequent`, as the linear `read` names what it read.
    kind: &'static str,
    /// The sequent as JSON.
    sequent: Sequent,
    /// The sequent in the parser's Unicode syntax.
    text: String,
    /// The logic it is read for.
    logic: Logic,
}

/// Reads a sequent of ordinary logic, typed or as JSON, for `logic`, and
/// answers `{"kind": "sequent", "sequent", "text", "logic"}`: what the
/// page shows before the proof starts. A sequent with more than one
/// formula right of `⊢` is refused outside classical logic, as `start`
/// refuses it.
///
/// # Errors
///
/// A parse error with its place, an unknown logic, and `succedents`.
pub fn read(input: &str, logic: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let logic: Logic = logic.parse().map_err(Failure::from)?;
    let sequent = sequent_of(input, &settings)?;
    // A session refuses the sequent where the logic does: two formulas
    // right of ⊢ outside classical logic.
    Interactive::within(&sequent, logic, &settings.limits).map_err(Failure::from)?;
    written(&Read {
        kind: "sequent",
        text: sequent.to_string(),
        sequent,
        logic,
    })
}

/// Returns the Rocq certificate of an ordinary outcome: of its checked
/// derivation where the sequent is valid, of its countermodel where it is
/// not. The certificate is over `Prop` and needs no library.
///
/// # Errors
///
/// An outcome that cannot be read (one valid by a clausal proof keeps no
/// proof to certify), `no_certificate` for an outcome with neither a
/// derivation nor a countermodel, and the certificate's own refusals.
pub fn certificate(outcome: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut json = serde_json::Deserializer::from_str(outcome);
    let outcome: ordinary::Outcome =
        wire::upgrade(&mut json, &settings.limits).map_err(Failure::from)?;
    json.end().map_err(Failure::json)?;
    let mut out = String::new();
    match (&outcome.verdict, &outcome.countermodel) {
        (ordinary::Verdict::Valid(evidence), _) => {
            let derivation = evidence.derivation().map_err(Failure::from)?;
            rocq::write(
                derivation,
                &settings.styles.rocq,
                &mut out,
                crate::unstopped(),
            )
        }
        (_, Some(countermodel)) => rocq::write_countermodel(
            countermodel,
            outcome.logic,
            &settings.styles.rocq,
            &settings.limits,
            &mut out,
            crate::unstopped(),
        ),
        _ => {
            return Err(Failure::refused(
                "no_certificate",
                "unsupported",
                "the outcome holds neither a derivation nor a countermodel to certify",
            ));
        }
    }
    .map_err(Failure::from)?;
    Ok(out)
}

/// Reads a side by its name, `left` or `right`.
fn side_of(side: &str) -> Result<Side, Failure> {
    match side {
        "left" => Ok(Side::Left),
        "right" => Ok(Side::Right),
        _ => Err(Failure::refused(
            "json",
            "malformed",
            &format!("a side is \"left\" or \"right\", not {side:?}"),
        )),
    }
}

/// Reads an ordinary sequent, typed or as JSON.
fn sequent_of(input: &str, settings: &Settings) -> Result<Sequent, Failure> {
    if input.trim_start().starts_with('{') {
        let mut json = serde_json::Deserializer::from_str(input);
        let sequent = wire::upgrade(&mut json, &settings.limits).map_err(Failure::from)?;
        json.end().map_err(Failure::json)?;
        Ok(sequent)
    } else {
        Sequent::parse_within(input, &settings.limits).map_err(Failure::from)
    }
}

/// Reads a session document of LK or LJ.
fn session_of(input: &str, settings: &Settings) -> Result<Interactive, Failure> {
    let mut json = serde_json::Deserializer::from_str(input);
    let state = wire::upgrade(&mut json, &settings.limits).map_err(Failure::from)?;
    json.end().map_err(Failure::json)?;
    Ok(state)
}

/// Writes a value as JSON.
fn written(value: &impl Serialize) -> Result<String, Failure> {
    serde_json::to_string(value).map_err(Failure::json)
}
