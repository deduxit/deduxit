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
//! the first zone's members first, and a line by its number from 1, as
//! the session numbers its hypotheses and the lines a forward step
//! derived. Every verdict is "valid in the calculus given": nothing says
//! the calculus's rules are sound.

use crate::failure::Failure;
use crate::tab;
use deduxit::export::Drawable;
use deduxit::export::{latex, svg, typst};
use deduxit::linear::proofs::Layout;
use deduxit::rules::{
    self, Apply, Calculus, Derive, Drawing, Fitch, GoalId, Interactive, LineId, Undone,
};
use deduxit::{Limits, Progress, Settings, wire};
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
    /// The lines in force at the goal, in the order of their numbers:
    /// what a forward step cites and `cite` closes the goal by.
    lines: Vec<LineView>,
}

/// A line in force at a goal.
#[derive(Serialize)]
struct LineView {
    /// Its number, from 1, which never changes while the line stands:
    /// what the calls take.
    line: u32,
    /// Its number in the session's Fitch proof, from 1, which the drawing
    /// shows and a later step may change; `null` where no Fitch proof
    /// writes the session or the bounds refuse it.
    fitch: Option<u32>,
    /// The line as text.
    sequent: String,
}

/// What the page shows of a session.
#[derive(Serialize)]
struct View<'a> {
    /// The host the stage draws the session into, as the protocol names
    /// it (`describe`): `tree` for a derivation, `list` for a Fitch proof (`view.layout` `fitch`).
    layout: &'static str,
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
    /// For each inference `i<n>` of the tree's drawing, at index `n`, its
    /// node in the session: an open goal's id where the inference is one.
    /// A line cited twice is drawn under each parent, so its node stands
    /// twice. Empty when the settings' bounds refuse the drawing.
    inferences: Vec<GoalId>,
}

impl<'a> View<'a> {
    /// Returns the view of a session, its drawing's inferences within
    /// `limits`.
    fn of(state: &'a Interactive, settings: &Settings) -> Result<Self, Failure> {
        let limits = &settings.limits;
        let (calculus, terms) = (state.calculus(), state.terms());
        let fitch = fitch_numbers(state, limits);
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
                lines: state
                    .lines(goal)
                    .into_iter()
                    .map(|(line, sequent)| LineView {
                        line: line.number(),
                        fitch: fitch
                            .as_ref()
                            .and_then(|numbers| numbers.get(line.number() as usize - 1).copied()),
                        sequent: calculus.show_zones(terms, sequent).to_string(),
                    })
                    .collect(),
            })
            .collect();
        let derivation = state.derivation().map_err(Failure::from)?;
        let nodes = state.derivation_goals().map_err(Failure::from)?;
        let inferences = Drawing::new(calculus, &derivation, limits).map_or_else(
            |_| Vec::new(),
            |drawing| drawing.nodes().map(|node| nodes[node.index()]).collect(),
        );
        Ok(Self {
            layout: if settings.view.layout == Layout::Fitch {
                "list"
            } else {
                "tree"
            },
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
            inferences,
        })
    }
}

/// Returns, at index `n - 1`, the number of the session's line `n` in its
/// Fitch proof, or `None` where no Fitch proof writes the session within
/// `limits`.
fn fitch_numbers(state: &Interactive, limits: &Limits) -> Option<Vec<u32>> {
    let (derivation, lines) = state.fitch_derivation().ok()?;
    let fitch = Fitch::new(
        state.calculus(),
        &derivation,
        &rules::FitchOptions::default(),
        limits,
    )
    .ok()?;
    let mut numbers = vec![0; derivation.len()];
    for (number, node) in (1..).zip(fitch.nodes()) {
        numbers[node.index()] = number;
    }
    // Every line is drawn, an equal hypothesis as the first of its scope.
    lines
        .iter()
        .map(|node| Some(numbers[node.index()]).filter(|&number| number > 0))
        .collect()
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
fn changed(
    state: &Interactive,
    settings: &Settings,
    said: impl Serialize,
) -> Result<String, Failure> {
    written(&Changed {
        session: state,
        view: View::of(state, settings)?,
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
    changed(&state, &settings, serde_json::Map::new())
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
    written(&View::of(&state, &settings)?)
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
    changed(&state, &settings, Opened { opened })
}

/// A forward step as the page sends it: `{"rule", "lines", "given"}`.
///
/// - `rule` is the rule's name.
/// - `lines` lists the lines in force that the rule's premises are, by
///   their numbers, in the rule's order.
/// - `given` maps what the lines leave open, a variable of the rule by
///   name, to its text: a formula only the conclusion holds.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeriveForm {
    /// The rule.
    pub rule: String,
    /// The lines cited.
    #[serde(default)]
    pub lines: Vec<u32>,
    /// The values given, by name.
    #[serde(default)]
    pub given: std::collections::BTreeMap<String, String>,
}

/// What a forward step derived.
#[derive(Serialize)]
struct Derived {
    /// The number of the line the step derived.
    line: u32,
}

/// Applies a rule forwards ([`DeriveForm`]) at an open goal: from lines
/// in force there to a new line in the goal's subproof, which the goal
/// and every goal inside the subproof may cite. Returns the session with
/// `line`, the new line's number. A rule that discharges hypotheses is
/// refused: it applies backwards, which opens its subproof.
///
/// # Errors
///
/// A step that is no such JSON; `step_refused` with the reason for a line
/// not in force, a rule the lines do not match, or one that discharges;
/// `step_needs` with the names of what the step needs given; `parse` for
/// a given text that is no term.
pub fn derive(session: &str, goal: u32, step: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let step: DeriveForm = serde_json::from_str(step).map_err(Failure::json)?;
    let lines: Vec<LineId> = step.lines.iter().copied().map(LineId::new).collect();
    let given: Vec<(&str, &str)> = step
        .given
        .iter()
        .map(|(name, text)| (name.as_str(), text.as_str()))
        .collect();
    let line = state
        .derive(
            GoalId::new(goal),
            &Derive::new(&step.rule).lines(&lines).given(&given),
        )
        .map_err(Failure::from)?;
    changed(
        &state,
        &settings,
        Derived {
            line: line.number(),
        },
    )
}

/// Closes an open goal by a line in force at it, by the line's number
/// from 1, and returns the session.
///
/// # Errors
///
/// `step_refused` for a goal that is not open, a line not in force at
/// it, and a line that is not the goal.
pub fn cite(session: &str, goal: u32, line: u32, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    state
        .cite(GoalId::new(goal), LineId::new(line))
        .map_err(Failure::from)?;
    changed(&state, &settings, serde_json::Map::new())
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
    changed(&state, &settings, serde_json::Map::new())
}

/// What an undo took back.
#[derive(Serialize)]
struct Reopened {
    /// The goal the undo reopened, `null` when no step was left or the
    /// step taken back was a forward one, which reopens no goal.
    reopened: Option<GoalId>,
    /// The number of the line a forward step taken back had derived, or
    /// `null`.
    removed: Option<u32>,
}

/// Takes back the session's last step and returns the session with
/// `reopened`, the goal a backward step or a closing reopens, and
/// `removed`, the line a forward step had derived; both `null` when no
/// step was left.
///
/// # Errors
///
/// As the session's reader.
pub fn undo(session: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let mut state = session_of(session, &settings)?;
    let (reopened, removed) = match state.undo() {
        Some(Undone::Reopened(goal)) => (Some(goal), None),
        Some(Undone::Removed(line)) => (None, Some(line.number())),
        _ => (None, None),
    };
    changed(&state, &settings, Reopened { reopened, removed })
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
/// style (`styles.svg`), open goals as the style draws them. In a tree
/// every inference has the id `i<n>`, and the view's `inferences` names
/// its node in the session; in a Fitch proof (`view.layout` `fitch`)
/// every line's formula has the id `l<n>`, `n` its number. It is
/// [`export`] in `svg`.
///
/// # Errors
///
/// As [`export`].
pub fn draw(session: &str, settings: &str) -> Result<String, Failure> {
    export(session, "svg", settings)
}

/// Returns a session's derivation in a format, `latex`, `typst` or
/// `svg`, open goals as the settings' style draws them: a tree, or with
/// `view.layout` `fitch` a Fitch proof, its numbered lines and
/// subproofs, the text `styles.fitch` sets. [`check`] gives a finished
/// derivation's document.
///
/// # Errors
///
/// An unknown format, a drawing estimated past `limits.derivation_bytes`,
/// `not_fitch` with the reason for a derivation no Fitch proof writes
/// (the tree draws it), and what the format refuses.
pub fn export(session: &str, format: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let state = session_of(session, &settings)?;
    let styles = &settings.styles;
    let write = |drawable: Drawable<'_>| -> Result<String, Failure> {
        let mut out = String::new();
        match format {
            "latex" => latex::write(drawable, &styles.latex, &mut out, never),
            "typst" => typst::write(drawable, &styles.typst, &mut out, never),
            "svg" => svg::write(drawable, &styles.svg, &mut out, never),
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
    };
    let calculus = state.calculus();
    if settings.view.layout == Layout::Fitch {
        // Every subproof opens with its hypotheses, those no step cites yet too.
        let (derivation, _) = state.fitch_derivation().map_err(Failure::from)?;
        let fitch = Fitch::new(calculus, &derivation, &styles.fitch, &settings.limits)
            .map_err(Failure::from)?;
        write(Drawable::from(&fitch))
    } else {
        let derivation = state.derivation().map_err(Failure::from)?;
        let tree = Drawing::new(calculus, &derivation, &settings.limits).map_err(Failure::from)?;
        write(Drawable::from(&tree))
    }
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
