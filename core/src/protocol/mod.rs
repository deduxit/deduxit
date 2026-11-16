// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What each suite offers, as data: the kinds of artifacts it makes, the
//! operations from artifacts to artifacts with their parameters, the
//! settings each operation reads and what it promises of its output, and
//! the functions that read, summarize, run and check them over JSON. A
//! front end lists the operations a value takes, runs one, and shows its
//! check beside the result, without code of its own per suite. Needs the
//! cargo feature `protocol` (on by default).
//!
//! # Values
//!
//! A value is JSON text:
//! - a value of a kind whose form is one of the wire forms ([`wire`]) is
//!   that form's document, with its `version`;
//! - a **statement**, a sequent with the calculus it is claimed in, is the
//!   sequent's document with one key more: `logic` in ordinary logic,
//!   `mode` in linear logic. The sequent's reader ignores it, so it raises
//!   no level. `text`, where present, is the sequent as typed;
//! - the other kinds (a sequent of a calculus given as data, an
//!   interpolant, the drawings and texts) are objects of this module's
//!   own. The manual's schema of the wire forms describes every one.
//!
//! # Runs
//!
//! [`run`] runs an operation on its inputs and gives its output;
//! [`check`] makes the check of what the operation promises, which may run
//! other operations (a translation's check decides both sequents). The
//! library reads no clock: a run's output is a function of its inputs, its
//! parameters and the settings the operation [`reads`](Operation::reads),
//! when the search runs on one thread (`search.jobs` 1), so a caller may
//! keep it for those.
//!
//! # Examples
//!
//! ```
//! use deduxit::{Settings, protocol};
//!
//! let sequent = r#"{"version": 8, "formulas": [{"V": 0}, {"¬": 0}, {"∨": [0, 1]}],
//!     "atoms": ["a"], "left": [], "right": [2], "logic": "classical"}"#;
//! let settings = Settings::default().deterministic();
//! let translated = protocol::run(
//!     "ordinary.translate",
//!     &[sequent],
//!     r#"{"translation": "glivenko"}"#,
//!     &settings,
//!     |_| false,
//! )?;
//! assert_eq!(protocol::summary("ordinary.sequent", &translated.value, &settings.limits)?,
//!     "⊢ ¬¬(a ∨ ¬a)");
//! let check = protocol::check(
//!     "ordinary.translate",
//!     &[sequent],
//!     r#"{"translation": "glivenko"}"#,
//!     &translated.value,
//!     &settings,
//!     |_| false,
//! )?;
//! assert_eq!(check.status, protocol::Status::Held);
//! # Ok::<(), deduxit::Error>(())
//! ```

mod linear;
mod ordinary;
mod rules;
mod shared;

#[cfg(test)]
pub(crate) use ordinary::write_statement as ordinary_statement;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{Error, Limits, Progress, Settings, wire};

/// What a value is to a user: what a front end does with it.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// A sequent with the calculus it is claimed in.
    Statement,
    /// A calculus, given as a rule file.
    Definition,
    /// A session of step-by-step proving: the user's steps.
    Work,
    /// What a run found: an outcome, a proof, a derivation, a net.
    Evidence,
    /// A drawing or a text of something else, made again when asked for.
    Rendering,
}

/// How an operation's output relates to its inputs once made.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    /// A function of the inputs: made again when an input changes.
    Derived,
    /// Starts from the inputs and then holds the user's steps: never made
    /// again without being asked.
    Work,
    /// Starts as a copy and is the user's from then on: the operation only
    /// says where it came from.
    Copy,
}

/// What running something may cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Cost {
    /// About linear in the input: it meets no bound but the sizes.
    Instant,
    /// A search, which may end at a bound of the settings.
    Search,
}

/// A kind of value.
#[non_exhaustive]
#[derive(Debug, Serialize)]
pub struct Kind {
    /// Its name: `ordinary.sequent`, `drawing`.
    pub name: &'static str,
    /// What it is to a user.
    pub role: Role,
    /// The entry of the manual's schema of the wire forms that its value
    /// matches.
    pub form: &'static str,
    /// The lowest wire level of its document; `None` for a value of this
    /// module's own, which is no document.
    pub level: Option<u32>,
    /// Whether a value of it is worth storing: `false` for a rendering,
    /// which is quick to make again.
    pub stored: bool,
    /// What it is, in plain words.
    pub label: &'static str,
}

/// A parameter of an operation: a key of its JSON object of parameters.
#[non_exhaustive]
#[derive(Debug, Serialize)]
pub struct Parameter {
    /// The key.
    pub name: &'static str,
    /// The names it takes, where it is one of a list; empty where the
    /// description says what it takes.
    pub values: &'static [&'static str],
    /// The value taken where the key is left out, `None` where the
    /// description says what is done then.
    pub default: Option<&'static str>,
    /// What it chooses.
    pub description: &'static str,
}

/// What an operation promises of its output, and what checking it costs.
#[non_exhaustive]
#[derive(Debug, Serialize)]
pub struct Promise {
    /// Its name: `same_verdict`, `decides`.
    pub name: &'static str,
    /// What the check costs, `None` where deduxit has nothing to check
    /// (the promise holds by construction, or another program checks it).
    pub check: Option<Cost>,
    /// What it claims.
    pub description: &'static str,
}

/// An operation: what it takes, what it makes, and what it reads.
#[non_exhaustive]
#[derive(Debug, Serialize)]
pub struct Operation {
    /// Its name: `ordinary.translate`.
    pub name: &'static str,
    /// For each input in order, the kinds it takes.
    pub from: &'static [&'static [&'static str]],
    /// The kind of its output.
    pub to: &'static str,
    /// How its output relates to its inputs.
    pub class: Class,
    /// What a run costs.
    pub cost: Cost,
    /// Its parameters.
    pub parameters: &'static [Parameter],
    /// The settings keys its output depends on, besides its inputs and
    /// parameters. A key that names an object (`search`) stands for every
    /// key inside it. The time limits are no part of it: a stop ends a run
    /// without an output.
    pub reads: &'static [&'static str],
    /// What it promises of its output.
    pub promise: &'static Promise,
    /// Its name in plain words: "Glivenko's translation".
    pub label: &'static str,
}

/// An operation to run, with its inputs, parameters and settings.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Call<'a> {
    /// The operation.
    pub operation: &'static Operation,
    /// Its inputs' values, in order.
    pub inputs: &'a [&'a str],
    /// Its parameters as a JSON object; `""` for every default.
    pub parameters: &'a str,
    /// The settings it runs under.
    pub settings: &'a Settings,
}

impl<'a> Call<'a> {
    /// Returns the call of `operation` on `inputs`.
    pub const fn new(
        operation: &'static Operation,
        inputs: &'a [&'a str],
        parameters: &'a str,
        settings: &'a Settings,
    ) -> Self {
        Self {
            operation,
            inputs,
            parameters,
            settings,
        }
    }

    /// Returns input `index`.
    fn input(&self, index: usize) -> &'a str {
        self.inputs.get(index).copied().unwrap_or_default()
    }
}

/// The bound that ended a run before it decided, as the settings name it.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Limit {
    /// The library's name for it: `work_limit`, `copy_bound`.
    pub reason: &'static str,
    /// The settings key that lifts it, `None` where none does (an index
    /// space that is full).
    pub setting: Option<&'static str>,
}

/// What a run made.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Produced {
    /// The output's value.
    pub value: String,
    /// The bound that ended the run before it decided, where one did. The
    /// output then says "unknown".
    pub limit: Option<Limit>,
}

impl Produced {
    /// Returns an output that no bound ended.
    const fn done(value: String) -> Self {
        Self { value, limit: None }
    }
}

/// How a check ended.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Made, and it holds.
    Held,
    /// Made, and it does not hold: a fault of deduxit.
    Failed,
    /// A part could not be decided within the settings' bounds.
    Open,
    /// It held when the output was made, and the output holds nothing to
    /// check it by again (a clausal proof's counts).
    Recorded,
    /// Another program checks it (Rocq checks a certificate).
    Outside,
    /// There is nothing to check.
    None,
}

/// A verdict, in the words every suite shares.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    /// Valid, or proved.
    Valid,
    /// Not valid, or unprovable.
    NotValid,
    /// Not decided within the bounds.
    Unknown,
}

/// A part of a check.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct Part {
    /// What it is: `sequent`, `translation`, `atoms`, `first`.
    pub name: String,
    /// Whether it holds.
    pub status: Status,
    /// The verdict it rests on, where it rests on one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<Verdict>,
}

/// The check of what an operation promised of its output.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct Check {
    /// The promise checked.
    pub promise: String,
    /// The result.
    pub status: Status,
    /// Its parts, where it has several.
    pub parts: Vec<Part>,
}

impl Check {
    /// Returns a check with no parts.
    fn whole(promise: &'static Promise, status: Status) -> Self {
        Self {
            promise: promise.name.to_owned(),
            status,
            parts: Vec::new(),
        }
    }

    /// Returns the check whose parts are `parts`: failed if one failed,
    /// open if one is open, else held.
    fn of(promise: &'static Promise, parts: Vec<Part>) -> Self {
        let status = if parts.iter().any(|p| p.status == Status::Failed) {
            Status::Failed
        } else if parts.iter().any(|p| p.status == Status::Open) {
            Status::Open
        } else {
            Status::Held
        };
        Self {
            promise: promise.name.to_owned(),
            status,
            parts,
        }
    }
}

/// Runs an operation through the caller, for a check that needs another
/// run: its name, inputs and parameters, answering its output's value.
pub type Ask<'a> = dyn FnMut(&str, &[&str], &str) -> Result<String, Error> + 'a;

/// What a suite offers. Each suite of the crate implements it, and a
/// suite of another crate may; [`suites`] lists this build's.
pub trait Suite: Sync {
    /// Its name, the prefix of its kinds and operations: `ordinary`.
    fn name(&self) -> &'static str;

    /// Its name in plain words.
    fn label(&self) -> &'static str;

    /// Its kinds.
    fn kinds(&self) -> &'static [Kind];

    /// Its operations.
    fn operations(&self) -> &'static [Operation];

    /// Checks that `value` is a value of `kind`, one of [`kinds`](Self::kinds),
    /// within `limits`.
    ///
    /// # Errors
    ///
    /// The error of the kind's reader.
    fn read(&self, kind: &Kind, value: &str, limits: &Limits) -> Result<(), Error>;

    /// Returns one line that names a value of `kind` in a list: a sequent's
    /// text, a verdict.
    ///
    /// # Errors
    ///
    /// As [`read`](Self::read).
    fn summary(&self, kind: &Kind, value: &str, limits: &Limits) -> Result<String, Error>;

    /// Runs one of [`operations`](Self::operations).
    ///
    /// # Errors
    ///
    /// An input of another kind, parameters the operation does not take,
    /// and the operation's own refusals.
    fn run(
        &self,
        call: &Call<'_>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Produced, Error>;

    /// Makes the check of what `call`'s operation promised of `output`,
    /// running through `ask` any other operation it needs.
    ///
    /// # Errors
    ///
    /// As [`run`](Self::run), and the errors of `ask`.
    fn check(
        &self,
        call: &Call<'_>,
        output: &str,
        ask: &mut Ask<'_>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Check, Error>;
}

/// The suites of this build.
pub fn suites() -> &'static [&'static dyn Suite] {
    &SUITES
}

/// The suites of this build, in the order a front end shows them.
static SUITES: [&dyn Suite; 3] = [&linear::LINEAR, &ordinary::ORDINARY, &rules::RULES];

/// The kinds no suite owns: the renderings every suite makes.
pub const fn shared() -> &'static [Kind] {
    &shared::KINDS
}

/// Every operation name of this build, in the suites' order.
const OPERATIONS: &[&str] = &[
    "linear.prove",
    "linear.start",
    "linear.goal",
    "linear.proof",
    "linear.net",
    "linear.eliminate",
    "linear.term",
    #[cfg(feature = "svg")]
    "linear.draw",
    "linear.export",
    #[cfg(feature = "rocq")]
    "linear.certificate",
    "linear.duplicate",
    "ordinary.decide",
    "ordinary.start",
    "ordinary.goal",
    "ordinary.derivation",
    "ordinary.translate",
    "ordinary.interpolate",
    "ordinary.half",
    "ordinary.embed",
    "ordinary.term",
    #[cfg(feature = "svg")]
    "ordinary.draw",
    "ordinary.export",
    #[cfg(feature = "rocq")]
    "ordinary.certificate",
    "ordinary.duplicate",
    "rules.start",
    "rules.derivation",
    "rules.from_ordinary",
    #[cfg(feature = "svg")]
    "rules.draw",
    #[cfg(any(feature = "latex", feature = "typst"))]
    "rules.export",
    "rules.duplicate",
];

/// Every kind name of this build: the suites' in order, then the shared.
const KINDS: &[&str] = &[
    "linear.sequent",
    "linear.session",
    "linear.outcome",
    "linear.proof",
    "linear.net",
    "ordinary.sequent",
    "ordinary.session",
    "ordinary.outcome",
    "ordinary.derivation",
    "ordinary.interpolant",
    "rules.calculus",
    "rules.sequent",
    "rules.session",
    "rules.derivation",
    "drawing",
    "text",
    "certificate",
    "term",
];

/// Returns an operation by name, with its suite.
///
/// # Errors
///
/// [`Error::UnknownName`] for a name no suite of this build has.
pub fn operation(name: &str) -> Result<(&'static dyn Suite, &'static Operation), Error> {
    suites()
        .iter()
        .find_map(|suite| {
            suite
                .operations()
                .iter()
                .find(|operation| operation.name == name)
                .map(|operation| (*suite, operation))
        })
        .ok_or_else(|| Error::UnknownName {
            what: "operation",
            name: name.into(),
            known: OPERATIONS,
        })
}

/// Returns a kind by name, with its suite (`None` for a shared kind).
///
/// # Errors
///
/// [`Error::UnknownName`] for a name this build does not have.
pub fn kind(name: &str) -> Result<(Option<&'static dyn Suite>, &'static Kind), Error> {
    if let Some(kind) = shared::KINDS.iter().find(|kind| kind.name == name) {
        return Ok((None, kind));
    }
    suites()
        .iter()
        .find_map(|suite| {
            suite
                .kinds()
                .iter()
                .find(|kind| kind.name == name)
                .map(|kind| (Some(*suite), kind))
        })
        .ok_or_else(|| Error::UnknownName {
            what: "kind",
            name: name.into(),
            known: KINDS,
        })
}

/// Checks that `value` is a value of the kind named `kind`, within
/// `limits`.
///
/// # Errors
///
/// An unknown kind, and the error of the kind's reader.
pub fn read(kind: &str, value: &str, limits: &Limits) -> Result<(), Error> {
    match self::kind(kind)? {
        (Some(suite), kind) => suite.read(kind, value, limits),
        (None, kind) => shared::read(kind, value),
    }
}

/// Returns one line that names a value of the kind named `kind`.
///
/// # Errors
///
/// As [`read`].
pub fn summary(kind: &str, value: &str, limits: &Limits) -> Result<String, Error> {
    match self::kind(kind)? {
        (Some(suite), kind) => suite.summary(kind, value, limits),
        (None, kind) => shared::summary(kind, value),
    }
}

/// Runs the operation named `operation` on `inputs` with `parameters`
/// (a JSON object, `""` for every default) under `settings`, `stop` asked
/// as the operation goes.
///
/// # Errors
///
/// An unknown operation, a number of inputs other than it takes, an input
/// of another kind, parameters it does not take, and its own refusals.
pub fn run(
    operation: &str,
    inputs: &[&str],
    parameters: &str,
    settings: &Settings,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Produced, Error> {
    let (suite, operation) = self::operation(operation)?;
    let call = Call::new(operation, inputs, parameters, settings);
    arity(&call)?;
    suite.run(&call, &mut stop)
}

/// Makes the check of what the operation named `operation` promised of
/// `output`, its output on `inputs` with `parameters`. The operations the
/// check needs are run directly, under the same settings.
///
/// # Errors
///
/// As [`run`].
pub fn check(
    operation: &str,
    inputs: &[&str],
    parameters: &str,
    output: &str,
    settings: &Settings,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Check, Error> {
    let (suite, operation) = self::operation(operation)?;
    let call = Call::new(operation, inputs, parameters, settings);
    arity(&call)?;
    // The check's own work and the runs it asks for take turns, so one
    // stop serves both.
    let stop = std::cell::RefCell::new(&mut stop);
    let mut ask = |name: &str, inputs: &[&str], parameters: &str| {
        run(name, inputs, parameters, settings, |progress| {
            (stop.borrow_mut())(progress)
        })
        .map(|produced| produced.value)
    };
    suite.check(&call, output, &mut ask, &mut |progress| {
        (stop.borrow_mut())(progress)
    })
}

/// The error of an input of a kind the operation does not take.
fn kinds(call: &Call<'_>) -> Error {
    Error::Json {
        form: "input",
        message: format!(
            "{} takes {}",
            call.operation.name,
            call.operation
                .from
                .first()
                .map_or(String::new(), |kinds| kinds.join(" or "))
        ),
    }
}

/// Refuses an input of a kind other than the operation's first input
/// takes.
fn takes(call: &Call<'_>, kind: &str) -> Result<(), Error> {
    if call
        .operation
        .from
        .first()
        .is_some_and(|kinds| kinds.contains(&kind))
    {
        Ok(())
    } else {
        Err(kinds(call))
    }
}

/// Returns the statement of a sequent typed in a suite's syntax, the value
/// a workspace's sequent holds: `suite` is `ordinary` with `parameter` a
/// logic, or `linear` with `parameter` a mode, whose intuitionistic modes
/// read the sequent two-sided in the syntax of intuitionistic linear logic.
/// The text is kept beside the sequent as typed.
///
/// # Errors
///
/// An unknown suite, logic or mode, the parser's error, and a sequent past
/// `limits.occurrences`.
pub fn statement(
    suite: &str,
    text: &str,
    parameter: &str,
    limits: &Limits,
) -> Result<String, Error> {
    match suite {
        "ordinary" => {
            let logic: crate::ordinary::Logic = parameter.parse()?;
            let sequent = crate::ordinary::Sequent::parse_within(text, limits)?;
            ordinary::typed_statement(&sequent, logic, Some(text))
        }
        "linear" => {
            let mode: crate::linear::Mode = parameter.parse()?;
            let sequent = if mode.is_intuitionistic() && !mode.is_ordered() {
                crate::linear::IllSequent::parse_within(text, limits)?.into_sequent()
            } else if mode.is_ordered() {
                crate::linear::Sequent::parse_ordered(text, mode.order(), limits)?
            } else {
                crate::linear::Sequent::parse_within(text, limits)?
            };
            linear::typed_statement(&sequent, mode, Some(text))
        }
        _ => Err(Error::UnknownName {
            what: "suite",
            name: suite.into(),
            known: &["linear", "ordinary"],
        }),
    }
}

/// Refuses a call with another number of inputs than its operation takes.
fn arity(call: &Call<'_>) -> Result<(), Error> {
    let takes = call.operation.from.len();
    if call.inputs.len() == takes {
        Ok(())
    } else {
        Err(Error::InvalidOption {
            key: "inputs",
            message: format!(
                "{} takes {takes} input{}, not {}",
                call.operation.name,
                if takes == 1 { "" } else { "s" },
                call.inputs.len()
            ),
        })
    }
}

/// The catalogue of this build: every suite with its kinds and operations,
/// and the shared kinds. Its JSON is what a front end builds its menus
/// from.
#[derive(Debug, Serialize)]
pub struct Catalogue {
    /// The build's version.
    deduxit: &'static str,
    /// The highest wire level the build reads and writes.
    level: u32,
    /// The suites.
    suites: Vec<Described>,
    /// The shared kinds.
    shared: &'static [Kind],
}

/// A suite as the catalogue writes it.
#[derive(Debug, Serialize)]
struct Described {
    /// Its name.
    suite: &'static str,
    /// Its name in plain words.
    label: &'static str,
    /// Its kinds.
    kinds: &'static [Kind],
    /// Its operations.
    operations: &'static [Operation],
}

/// Returns the catalogue of this build.
pub fn catalogue() -> Catalogue {
    Catalogue {
        deduxit: env!("CARGO_PKG_VERSION"),
        level: wire::LEVEL,
        suites: suites()
            .iter()
            .map(|suite| Described {
                suite: suite.name(),
                label: suite.label(),
                kinds: suite.kinds(),
                operations: suite.operations(),
            })
            .collect(),
        shared: &shared::KINDS,
    }
}

/// Reads a document of the form `T` within `limits`, and nothing after it.
fn document<T: wire::Readable>(text: &str, limits: &Limits) -> Result<T, Error> {
    let mut json = serde_json::Deserializer::from_str(text);
    let value = wire::upgrade(&mut json, limits)?;
    json.end().map_err(|error| Error::Json {
        form: T::FORM,
        message: error.to_string(),
    })?;
    Ok(value)
}

/// Reads an object of this module's own, `form` naming it in an error.
fn object<T: DeserializeOwned>(text: &str, form: &'static str) -> Result<T, Error> {
    serde_json::from_str(text).map_err(|error| Error::Json {
        form,
        message: error.to_string(),
    })
}

/// Reads an operation's parameters: `""` is every default.
fn parameters<T: DeserializeOwned + Default>(call: &Call<'_>) -> Result<T, Error> {
    if call.parameters.trim().is_empty() {
        return Ok(T::default());
    }
    serde_json::from_str(call.parameters).map_err(|error| Error::InvalidOption {
        key: "parameters",
        message: format!("{}: {error}", call.operation.name),
    })
}

/// Writes a value as JSON. A value of the crate always writes.
fn written(value: &impl Serialize) -> Result<String, Error> {
    serde_json::to_string(value).map_err(|error| Error::Json {
        form: "value",
        message: error.to_string(),
    })
}

/// The keys every outcome writes, read without the outcome's own reader,
/// which refuses one valid by a clausal proof.
#[derive(serde::Deserialize)]
struct Verdicted {
    /// The verdict's name.
    verdict: String,
}

/// Returns the verdict an outcome's value writes, in the shared words.
fn verdict_of(outcome: &str) -> Result<Verdict, Error> {
    let read: Verdicted = object(outcome, "outcome")?;
    Ok(match read.verdict.as_str() {
        "valid" | "proved" => Verdict::Valid,
        "not_valid" | "unprovable" => Verdict::NotValid,
        _ => Verdict::Unknown,
    })
}

/// Returns the part of a check that rests on `verdict`, which should be
/// `expected`: held if it is, open if it is unknown, failed otherwise.
fn part(name: &'static str, verdict: Verdict, expected: Verdict) -> Part {
    let status = match verdict {
        Verdict::Unknown => Status::Open,
        found if found == expected => Status::Held,
        _ => Status::Failed,
    };
    Part {
        name: name.to_owned(),
        status,
        verdict: Some(verdict),
    }
}

/// Returns the check that two verdicts agree: held if they do, failed if
/// they differ, open if either is unknown.
fn same_verdict(
    promise: &'static Promise,
    names: [&'static str; 2],
    verdicts: [Verdict; 2],
) -> Check {
    // A side not decided leaves the check open: the decided side is no
    // fault while the other has no verdict to compare.
    let open = verdicts.contains(&Verdict::Unknown);
    let agree = verdicts[0] == verdicts[1];
    let status = |verdict| match verdict {
        Verdict::Unknown => Status::Open,
        _ if agree || open => Status::Held,
        _ => Status::Failed,
    };
    Check::of(
        promise,
        vec![
            Part {
                name: names[0].to_owned(),
                status: status(verdicts[0]),
                verdict: Some(verdicts[0]),
            },
            Part {
                name: names[1].to_owned(),
                status: status(verdicts[1]),
                verdict: Some(verdicts[1]),
            },
        ],
    )
}

/// The promises the suites share.
mod promises {
    use super::{Cost, Promise};

    /// The output is read off the input.
    pub(super) static EXACT: Promise = Promise {
        name: "exact",
        check: None,
        description: "The output is read off the input: a goal, a half, a start or a copy.",
    };

    /// A verdict has its evidence.
    pub(super) static DECIDES: Promise = Promise {
        name: "decides",
        check: Some(Cost::Instant),
        description: "A verdict has its evidence, which deduxit's checker accepts: a proof or derivation, a refutation or a countermodel.",
    };

    /// A proof proves its sequent.
    pub(super) static PROVES: Promise = Promise {
        name: "proves",
        check: Some(Cost::Instant),
        description: "The proof, derivation or net proves its sequent, which deduxit's checker accepts.",
    };

    /// Two sequents are valid together or not.
    pub(super) static SAME_VERDICT: Promise = Promise {
        name: "same_verdict",
        check: Some(Cost::Search),
        description: "The input and the output are valid together or not: both are decided and the verdicts compared.",
    };

    /// An interpolant's three conditions.
    pub(super) static INTERPOLATES: Promise = Promise {
        name: "interpolates",
        check: Some(Cost::Search),
        description: "The interpolant's atoms occur in both parts, and both halves are valid.",
    };

    /// A drawing or a text shows its input.
    pub(super) static RENDERS: Promise = Promise {
        name: "renders",
        check: None,
        description: "The drawing or text shows its input.",
    };

    /// A proof assistant accepts the certificate.
    #[cfg(feature = "rocq")]
    pub(super) static CERTIFIES: Promise = Promise {
        name: "certifies",
        check: None,
        description: "Rocq accepts the certificate; deduxit checked what it writes, and Rocq checks the file.",
    };
}

/// The settings keys every operation that reads a sequent reads: the
/// readers count a document's formulas against them.
const SIZES: [&str; 2] = ["limits.occurrences", "limits.terms"];

/// The settings keys a session's start reads: the readers' sizes, and the
/// bounds its first state is held to.
const START: [&str; 4] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
];

/// The settings keys a search reads: every bound, the search's options,
/// and the derivation a valid sequent is read back to.
const SEARCH: [&str; 8] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "limits.recursion_depth",
    "limits.table_atoms",
    "search",
];

#[cfg(test)]
mod tests;
