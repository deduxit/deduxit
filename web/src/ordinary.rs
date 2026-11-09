// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of an ordinary tab: a sequent of classical, intuitionistic
//! or minimal logic proved step by step by the rules of LK or LJ, over
//! JSON text as the linear calls are. The session document is the
//! library's goal state of ordinary sequents (`{"version", "logic",
//! "sequent", "inferences", "history"}`), which the page keeps and sends
//! with every call; a formula of a goal is named by its side and its
//! position there.

use crate::failure::Failure;
use crate::tab;
use deduxit::export::{latex, rocq, svg, typst};
use deduxit::ordinary::{
    self, Applicable, Context, Interactive, Logic, Needs, NodeId, Rule, Sequent, Side, Step,
};
use deduxit::{GoalId, Progress, Settings, wire};
use serde::{Deserialize, Serialize};

/// The stop of every call: the page cancels by stopping the worker, so
/// only the bounds of the settings end a call early.
const fn never(_: Progress) -> bool {
    false
}

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
    /// For each inference `n` of the drawing (`i<n>`), with the structural
    /// rules as the settings' `view.structural` draws them, the goal it is
    /// in the session.
    inferences: Vec<GoalId>,
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
            logic: state.logic(),
            calculus: state.logic().calculus(),
            complete: state.is_complete(),
            steps: state.steps(),
            goals,
            inferences: state.derivation_ids(settings.view.structural),
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

/// Starts a proof of a sequent of ordinary logic, typed in the syntax of
/// ordinary formulas or as the ordinary sequent's JSON, in a logic by its
/// name (`classical`, `intuitionistic`, `minimal`), and returns the
/// session with its view; an intuitionistic or minimal sequent with
/// nothing right of `⊢` as `Γ ⊢ ⊥`.
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
    let state = Interactive::within(&sequent, logic, &settings.limits).map_err(Failure::from)?;
    changed(&state, &settings, serde_json::Map::new())
}

/// Returns a session's view.
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
    /// Whether it has two premises, which share the goal's context unless
    /// a step splits it.
    split: bool,
}

impl From<Applicable> for Offered {
    /// The rule and whether a step of it takes a split.
    fn from(applicable: Applicable) -> Self {
        Self {
            rule: applicable.rule,
            split: matches!(applicable.needs, Needs::Context),
        }
    }
}

/// Returns the rules that act on the formula at a position of a side
/// (`left` or `right`) of a goal, by its connective and the logic:
/// `[{"rule", "split"}]`. A rule listed may still be refused by the
/// step's split or, in LJ, by what its premises would hold right of `⊢`,
/// with the reason.
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

/// A step as the page sends it: `{"side", "position", "rule", "split"}`,
/// the rule by its name (`"→L"`), `split` for a rule with two premises
/// the positions on each side that go to the first premise,
/// `{"left": [..], "right": [..]}`, and left out for premises that share
/// the goal's context.
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
    /// share the context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<SplitForm>,
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

    /// Returns the form of a library's step.
    fn of(step: &Step) -> Self {
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
        }
    }
}

/// What a step opened.
#[derive(Serialize)]
struct Opened {
    /// The goals the step opened, none when it closed its goal.
    opened: Vec<GoalId>,
}

/// Applies a step (as [`StepForm`]) to an open goal and returns the
/// session with the goals it opened.
///
/// # Errors
///
/// A step that is no such JSON, and the step error when the rule does
/// not act on the formula, the logic lacks it, or its split does not fit.
pub fn apply(session: &str, goal: u32, step: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let step: StepForm = serde_json::from_str(step).map_err(Failure::json)?;
    let opened = state
        .apply(GoalId::new(goal), &step.step())
        .map_err(|e| Failure::from(deduxit::Error::from(e)))?;
    changed(&state, &settings, Opened { opened })
}

/// What an undo reopened.
#[derive(Serialize)]
struct Reopened {
    /// The goal the undo reopened, `null` when no step was left.
    reopened: Option<GoalId>,
}

/// Retracts the session's last step: a rule applied, or a goal the
/// search closed, whole.
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
    /// `prove --logic --format json` writes one, its derivation the
    /// goal's.
    outcome: ordinary::Outcome,
    /// `null` when the derivation was grafted or there was none, otherwise
    /// the refusal of the graft by its size, the goal then still open.
    refused: Option<serde_json::Value>,
}

/// Decides a goal through the translation of the settings'
/// `ordinary.translation` (the logic's default when `null`) within the
/// settings' bounds, and grafts its derivation onto the goal when it is
/// valid; a goal not valid or undecided stays open, the outcome saying
/// why.
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
            never,
        )
        .map_err(Failure::from)?;
    let refused = closed
        .grafted
        .err()
        .and_then(|refusal| serde_json::to_value(deduxit::Error::Refused(refusal)).ok());
    changed(
        &state,
        &settings,
        Closing {
            outcome: closed.outcome,
            refused,
        },
    )
}

/// What a hint found.
#[derive(Serialize)]
struct Hint {
    /// The outcome of deciding the goal.
    outcome: ordinary::Outcome,
    /// The first steps of the derivation found, as [`apply`] takes them
    /// in turn, each on the goal the one before opened, the first on the
    /// goal asked for: the weakenings and contractions at its root and the
    /// first rule that is not one, which ends the list. Empty unless the
    /// goal is valid.
    steps: Vec<StepForm>,
}

/// Decides a goal as [`close`] does, the session left as it was, and
/// returns the outcome with the first steps of the derivation found,
/// which the page applies in turn: a contraction comes with the rule that
/// uses both copies, so that following the hints proves the goal.
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
            never,
        )
        .map_err(Failure::from)?;
    written(&Hint {
        steps: hint.steps.iter().map(StepForm::of).collect(),
        outcome: hint.outcome,
    })
}

/// Returns the lambda-term of a session's derivation
/// ([`ordinary::Derivation::term`]): a finished derivation of LJ as a term
/// of the simply typed lambda calculus, answered as the linear
/// [`calls::term`](crate::calls::term) answers, the term's JSON with its
/// `text` and `judgement`, or `{"error": …}` where it has none (`no_term`:
/// a derivation of LK, or one with open goals) or the settings' bounds
/// refuse it.
///
/// # Errors
///
/// A session the reader refuses.
pub fn term(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let term = state
        .derivation_within(&settings.view, &settings.limits, never)
        .and_then(|derivation| derivation.term(&settings.limits, never));
    crate::calls::termed(term, &settings)
}

/// Returns a session's derivation, open goals as bare leaves, as an SVG
/// drawing in the settings' style (`styles.svg`) with its structural
/// rules as `view.structural` draws them, with an id on every conclusion
/// (`i<n>`, the view's `inferences` naming its goal).
///
/// # Errors
///
/// As [`export`].
pub fn draw(session: &str, settings: &str) -> Result<String, Failure> {
    export(session, "svg", settings)
}

/// Returns a session's derivation in a format: `text`, `latex`, `typst`
/// or `svg`, open goals as bare leaves and the structural rules as the
/// settings' `view.structural` draws them, or `rocq`, the certificate of
/// the finished derivation over `Prop`, checked first, every structural
/// rule in it. PNG and PDF are the SVG drawing rendered by `render`.
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
        let derivation = state.finished(limits, never).map_err(Failure::from)?;
        rocq::write(&derivation, &styles.rocq, &mut out, never).map_err(Failure::from)?;
        return Ok(out);
    }
    let derivation = state
        .derivation_within(&settings.view, limits, never)
        .map_err(Failure::from)?;
    match format {
        "text" => derivation.write_text(&styles.text, &mut out, never),
        "latex" => latex::write(&derivation, &styles.latex, &mut out, never),
        "typst" => typst::write(&derivation, &styles.typst, &mut out, never),
        "svg" => svg::write(&derivation, &styles.svg, &mut out, never),
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
