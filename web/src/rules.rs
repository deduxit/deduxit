// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of a tab that builds a derivation in a calculus given as
//! data: a rule file the page holds, one of the built-in calculi or the
//! visitor's own.
//!
//! The session document is the library's session in a calculus,
//! `{"version", "calculus", "terms", "scopes", "nodes", "history"}`,
//! which holds the rule file itself. The page keeps it, saves it and
//! sends it with every call; reading it checks every step taken again.
//! A member of a goal is named by its number across the goal's zones,
//! the first zone's members first. Every verdict is "valid in the
//! calculus given": nothing says the calculus's rules are sound.

use crate::failure::Failure;
use crate::tab;
use deduxit::export::{latex, svg, typst};
use deduxit::rules::{self, Apply, Calculus, Drawing, GoalId, Interactive, Undone};
use deduxit::{Progress, Settings, wire};
use serde::{Deserialize, Serialize};

/// The stop of every call. The page cancels a call by stopping its
/// worker, so only the bounds of the settings end a call early.
const fn never(_: Progress) -> bool {
    false
}

/// The built-in calculi by name, each its rule file: the command's.
const BUILT_IN: [(&str, &str); 13] = [
    ("LK", rules::LK),
    ("LJ", rules::LJ),
    ("LM", rules::LM),
    ("LL", rules::LL),
    ("tableau", rules::TABLEAU),
    ("NJ", rules::NJ),
    ("NK", rules::NK),
    ("NM", rules::NM),
    ("NK-DNE", rules::NK_DNE),
    ("NK-LEM", rules::NK_LEM),
    ("NJ-defined", rules::NJ_DEFINED),
    ("NK-defined", rules::NK_DEFINED),
    ("NM-defined", rules::NM_DEFINED),
];

/// The names of the built-in calculi, which [`calculus`] gives.
pub const NAMES: [&str; 13] = [
    "LK",
    "LJ",
    "LM",
    "LL",
    "tableau",
    "NJ",
    "NK",
    "NM",
    "NK-DNE",
    "NK-LEM",
    "NJ-defined",
    "NK-defined",
    "NM-defined",
];

/// Returns the rule file of a built-in calculus by its name ([`NAMES`]:
/// the sequent calculi `LK`, `LJ`, `LM`, `LL`, the `tableau`, and natural
/// deduction's `NJ`, `NK`, `NM` and their variants): the JSON text a
/// visitor starts their own calculus from.
///
/// # Errors
///
/// `unknown_name` for a name that is none of them.
pub fn calculus(name: &str) -> Result<String, Failure> {
    BUILT_IN
        .iter()
        .find(|(known, _)| *known == name)
        .map(|&(_, text)| text.to_owned())
        .ok_or_else(|| {
            Failure::refused(
                "unknown_name",
                "malformed",
                "the built-in calculi are LK, LJ, LM, LL, tableau, NJ, NK, NM, NK-DNE, NK-LEM, NJ-defined, NK-defined and NM-defined",
            )
        })
}

/// What the page shows of a rule file: its name, description, zones and
/// rules.
#[derive(Serialize)]
struct Described<'a> {
    /// The calculus's name.
    name: &'a str,
    /// Its description.
    description: &'a str,
    /// Its zones in order, each its name and the mark written before it.
    zones: Vec<Zone<'a>>,
    /// Its rules' names, in the file's order.
    rules: Vec<&'a str>,
}

/// A zone of a calculus's sequents.
#[derive(Serialize)]
struct Zone<'a> {
    /// Its name.
    name: &'a str,
    /// The mark written before it, such as `⊢`, if any.
    mark: Option<&'a str>,
}

impl<'a> Described<'a> {
    /// Returns what the page shows of `calculus`.
    fn of(calculus: &'a Calculus) -> Self {
        Self {
            name: calculus.name(),
            description: calculus.description(),
            zones: calculus
                .zones()
                .zip(calculus.marks())
                .map(|(name, mark)| Zone { name, mark })
                .collect(),
            rules: calculus.rules().collect(),
        }
    }
}

/// Reads a rule file, its JSON or a built-in calculus's name, and returns
/// what the page shows of it: `{"name", "description", "zones": [{"name",
/// "mark"}], "rules"}`. The page calls it to check a rule file the
/// visitor wrote before it starts a session in it.
///
/// # Errors
///
/// `invalid_calculus` with the reason for a rule file the library
/// refuses, and the refusals of the settings' bounds.
pub fn read(calculus: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let calculus = calculus_of(calculus, &settings)?;
    written(&Described::of(&calculus))
}

/// An open goal as the page shows it.
#[derive(Serialize)]
struct OpenGoal {
    /// The goal's id, which every call on it takes.
    goal: GoalId,
    /// The goal as text, in the calculus's own syntax.
    sequent: String,
    /// The members of each zone as text, in the order of their numbers.
    zones: Vec<Vec<String>>,
    /// The hypotheses in force at the goal as text, the innermost first,
    /// the session's assumptions last: what `assume` closes it by.
    hypotheses: Vec<String>,
}

/// What the page shows of a session.
#[derive(Serialize)]
struct View<'a> {
    /// The calculus.
    calculus: Described<'a>,
    /// The sequent derived, the session's first goal, as text.
    sequent: String,
    /// The session's assumptions as text, in order.
    assumptions: Vec<String>,
    /// Whether no goal is open.
    finished: bool,
    /// The steps taken, which `undo` takes back.
    steps: usize,
    /// The open goals.
    goals: Vec<OpenGoal>,
    /// For each inference `i<n>` of the drawing, at index `n`, its node in
    /// the session: an open goal's id where the inference is one.
    inferences: Vec<GoalId>,
}

impl<'a> View<'a> {
    /// Returns the view of a session.
    fn of(state: &'a Interactive) -> Result<Self, Failure> {
        let (calculus, terms) = (state.calculus(), state.terms());
        let goals = state
            .goals()
            .into_iter()
            .map(|goal| OpenGoal {
                goal,
                sequent: calculus.show_zones(terms, state.goal(goal)).to_string(),
                zones: state
                    .goal(goal)
                    .iter()
                    .map(|zone| {
                        zone.iter()
                            .map(|&m| calculus.show(terms, m).to_string())
                            .collect()
                    })
                    .collect(),
                hypotheses: state
                    .hypotheses(goal)
                    .into_iter()
                    .map(|h| calculus.show_zones(terms, h).to_string())
                    .collect(),
            })
            .collect();
        Ok(Self {
            calculus: Described::of(calculus),
            sequent: calculus
                .show_zones(terms, state.goal(GoalId::new(0)))
                .to_string(),
            assumptions: state
                .assumptions()
                .iter()
                .map(|a| calculus.show_zones(terms, a).to_string())
                .collect(),
            finished: state.is_finished(),
            steps: state.steps(),
            goals,
            inferences: state.derivation_goals().map_err(Failure::from)?,
        })
    }
}

/// A session after a call that changed it, with its view and what the
/// call says beyond it.
#[derive(Serialize)]
struct Changed<'a, T: Serialize> {
    /// The session, which the page keeps for the next call.
    session: &'a Interactive,
    /// What the page shows of it.
    view: View<'a>,
    /// What the call says beyond the session.
    #[serde(flatten)]
    said: T,
}

/// Returns a session's document with its view and what a call said.
fn changed(state: &Interactive, said: impl Serialize) -> Result<String, Failure> {
    written(&Changed {
        session: state,
        view: View::of(state)?,
        said,
    })
}

/// Starts a derivation of `goal` in a calculus, from the hypotheses
/// `assumptions`, and returns the session with its view, `{"session",
/// "view"}`.
///
/// The calculus is a rule file's JSON or a built-in calculus's name. The
/// goal is a sequent in the calculus's own syntax, as the view writes
/// one; `assumptions` is a JSON array of such sequents, or empty for
/// none.
///
/// # Errors
///
/// `invalid_calculus` for a rule file the library refuses, `parse` for a
/// text that is no sequent of the calculus, `step_refused` for a sequent
/// the calculus's zones do not take, and the refusals of the settings'
/// bounds.
pub fn start(
    calculus: &str,
    goal: &str,
    assumptions: &str,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let calculus = calculus_of(calculus, &settings)?;
    let assumptions: Vec<String> = if assumptions.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(assumptions).map_err(Failure::json)?
    };
    let assumptions: Vec<&str> = assumptions.iter().map(String::as_str).collect();
    let state = Interactive::within(calculus, goal, &assumptions, &settings.limits)
        .map_err(Failure::from)?;
    changed(&state, serde_json::Map::new())
}

/// Reads a saved session, checking every step in it again, and returns
/// what the page shows of it: the `view` that every call that changes
/// the session answers too.
///
/// # Errors
///
/// As the session's reader: a document of another level or form, and
/// `invalid_derivation` or `step_refused` for a step the calculus does
/// not allow.
pub fn view(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    written(&View::of(&state)?)
}

/// A step the page may take, as [`rules`](self::rules) lists it.
#[derive(Serialize)]
struct Offered {
    /// The rule's name.
    rule: String,
    /// The members it acts on, by their numbers.
    at: Vec<u32>,
    /// What the step needs given by name, empty when it can be taken as
    /// it is.
    needs: Vec<String>,
}

/// Returns the steps that apply to an open goal: `[{"rule", "at",
/// "needs"}]`, a rule once for each choice of the members it acts on.
/// A step with `needs` empty can be taken as it is; one with names needs a
/// value given for each (a formula only a premise holds, how a context
/// splits).
///
/// # Errors
///
/// `step_refused` for a goal that is not open, and the refusals of the
/// settings' bounds.
pub fn rules(session: &str, goal: u32, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let offered: Vec<Offered> = state
        .rules(GoalId::new(goal))
        .map_err(Failure::from)?
        .into_iter()
        .map(|a| Offered {
            rule: a.rule,
            at: a.at,
            needs: a.needs,
        })
        .collect();
    written(&offered)
}

/// A step as the page sends it: `{"rule", "at", "given"}`.
///
/// - `rule` is the rule's name.
/// - `at` lists the members of the goal it acts on, by their numbers
///   across the goal's zones, one for each place the rule asks for.
/// - `given` maps what the goal leaves open, a variable or a context of
///   the rule by name, to its text: a formula for a variable, members
///   separated by `,` for a context.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepForm {
    /// The rule.
    pub rule: String,
    /// The members it acts on.
    #[serde(default)]
    pub at: Vec<u32>,
    /// The values given, by name.
    #[serde(default)]
    pub given: std::collections::BTreeMap<String, String>,
}

/// What a step opened.
#[derive(Serialize)]
struct Opened {
    /// The goals the step opened, none when it closed its goal.
    opened: Vec<GoalId>,
}

/// Applies a step ([`StepForm`]) backwards to an open goal and returns
/// the session with `opened`, the goals the step opened. The calculus's
/// checker checks the step before the session takes it.
///
/// # Errors
///
/// A step that is no such JSON; `step_needs` with the names of what the
/// step needs given; `step_refused` with the reason for a step the
/// calculus does not allow; `parse` for a given text that is no term.
pub fn apply(session: &str, goal: u32, step: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let step: StepForm = serde_json::from_str(step).map_err(Failure::json)?;
    let given: Vec<(&str, &str)> = step
        .given
        .iter()
        .map(|(name, text)| (name.as_str(), text.as_str()))
        .collect();
    let opened = state
        .apply(
            GoalId::new(goal),
            &Apply::new(&step.rule).at(&step.at).given(&given),
        )
        .map_err(Failure::from)?;
    changed(&state, Opened { opened })
}

/// Closes an open goal by a hypothesis in force, and returns the session.
///
/// # Errors
///
/// `step_refused` for a goal that no hypothesis in force is, or whose
/// closing the rule that discharges the hypothesis refuses.
pub fn assume(session: &str, goal: u32, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    state.assume(GoalId::new(goal)).map_err(Failure::from)?;
    changed(&state, serde_json::Map::new())
}

/// What an undo reopened.
#[derive(Serialize)]
struct Reopened {
    /// The goal the undo reopened, `null` when no step was left or the
    /// step taken back was a forward one, which reopens no goal.
    reopened: Option<GoalId>,
}

/// Takes back the session's last step and returns the session with
/// `reopened`, the goal, or `null` when no step was left or a forward
/// step was taken back.
///
/// # Errors
///
/// As the session's reader.
pub fn undo(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let reopened = match state.undo() {
        Some(Undone::Reopened(goal)) => Some(goal),
        _ => None,
    };
    changed(&state, Reopened { reopened })
}

/// The verdict on a finished derivation.
#[derive(Serialize)]
struct Checked<'a> {
    /// The calculus the derivation is valid in.
    calculus: &'a str,
    /// What the verdict says: always "valid in the calculus given".
    verdict: &'static str,
    /// The derivation, as `check --calculus` reads one.
    derivation: rules::Derivation,
}

/// Checks a finished session's whole derivation by the calculus's
/// checker and returns `{"calculus", "verdict", "derivation"}`, the
/// verdict "valid in the calculus given" and the derivation's document.
///
/// # Errors
///
/// `open_goals` with their count while a goal is open,
/// `invalid_derivation` should the check refuse it, and the refusals of
/// the settings' bounds.
pub fn check(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let derivation = state.finished(never).map_err(Failure::from)?;
    written(&Checked {
        calculus: state.calculus().name(),
        verdict: "valid in the calculus given",
        derivation,
    })
}

/// Returns a session's derivation as an SVG drawing in the settings'
/// style (`styles.svg`), open goals as the style draws them. Every
/// inference has the id `i<n>`, and the view's `inferences` names its
/// node in the session. It is [`export`] in `svg`.
///
/// # Errors
///
/// As [`export`].
pub fn draw(session: &str, settings: &str) -> Result<String, Failure> {
    export(session, "svg", settings)
}

/// Returns a session's derivation in a format, `latex`, `typst` or
/// `svg`, open goals as the settings' style draws them. [`check`] gives
/// a finished derivation's document.
///
/// # Errors
///
/// An unknown format, a drawing estimated past `limits.derivation_bytes`,
/// and what the format refuses.
pub fn export(session: &str, format: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let derivation = state.derivation().map_err(Failure::from)?;
    let drawing =
        Drawing::new(state.calculus(), &derivation, &settings.limits).map_err(Failure::from)?;
    let styles = &settings.styles;
    let mut out = String::new();
    match format {
        "latex" => latex::write(&drawing, &styles.latex, &mut out, never),
        "typst" => typst::write(&drawing, &styles.typst, &mut out, never),
        "svg" => svg::write(&drawing, &styles.svg, &mut out, never),
        _ => {
            return Err(Failure::refused(
                "unknown_name",
                "malformed",
                "the formats are latex, typst and svg",
            ));
        }
    }
    .map_err(Failure::from)?;
    Ok(out)
}

/// Reads a rule file, its JSON or a built-in calculus's name.
fn calculus_of(input: &str, settings: &Settings) -> Result<Calculus, Failure> {
    let text = BUILT_IN
        .iter()
        .find(|(name, _)| *name == input)
        .map_or(input, |&(_, text)| text);
    let mut json = serde_json::Deserializer::from_str(text);
    let calculus = wire::upgrade(&mut json, &settings.limits).map_err(Failure::from)?;
    json.end().map_err(Failure::json)?;
    Ok(calculus)
}

/// Reads a session document in a calculus given as data.
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
