// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `grade`: an answer to a problem, checked as `check` and `interact`
//! check it, its kind read from its shape.

use crate::argument_parsing::{AnswerFormat, GradeArgs, InteractArgs, SequentInput, Tree};
use crate::check::{self, Document};
use crate::interact::{self, Input};
use crate::limit::Deadline;
use crate::prove::on_large_stack;
use crate::settings::{Given, time_limit};
use crate::show::Show;
use crate::{Status, io};
use anyhow::{Result, bail};
use clap::{Args, FromArgMatches};
use deduxit::linear::Mode;
use deduxit::ordinary::{self, Formulas, Logic, Node, NodeId};
use deduxit::{ErrorKind, Settings, modal};
use std::time::Instant;

/// The wire level of `grade`'s JSON: level 1 holds a verdict, the kind of
/// answer read, a message and the line of a script that failed.
const GRADED_LEVEL: u32 = 1;

/// The logic a problem is in, with its sequent read in it.
enum Problem {
    /// Linear logic in a mode.
    Linear(Box<deduxit::linear::Sequent>, Mode),
    /// An ordinary logic.
    Ordinary(Box<ordinary::Sequent>, Logic),
    /// A modal system.
    Modal(Box<modal::Sequent>, modal::System),
    /// A calculus given as data, which a session reads the sequent in.
    Calculus,
}

/// A verdict on an answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
enum Verdict {
    /// The answer proves or refutes the problem's sequent, checked.
    Correct,
    /// The answer does not read, is about another sequent, or fails its
    /// check.
    Incorrect,
    /// A limit stopped the check.
    Unknown,
}

/// What `grade` writes: its verdict and why.
#[derive(Debug, serde::Serialize)]
struct Graded {
    /// The wire level, [`GRADED_LEVEL`].
    version: u32,
    /// The verdict.
    verdict: Verdict,
    /// The kind of answer read: `proof`, `refutation`, `outcome`,
    /// `countermodel`, `derivation`, `tableau`, `script` or `none`.
    answer: &'static str,
    /// Why, in a sentence: the check's own verdict line where one ran.
    message: String,
    /// For a script, its line, from 1, that failed first.
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
}

impl Graded {
    /// A verdict on an answer of kind `answer`.
    fn new(verdict: Verdict, answer: &'static str, message: impl Into<String>) -> Self {
        Self {
            version: GRADED_LEVEL,
            verdict,
            answer,
            message: message.into(),
            line: None,
        }
    }

    /// An answer of kind `answer` that is wrong for `why`.
    fn incorrect(answer: &'static str, why: impl Into<String>) -> Self {
        Self::new(Verdict::Incorrect, answer, why)
    }
}

/// Runs `grade`: reads the problem's sequent in its logic and the answer,
/// tells the answer's kind by its shape, checks it, and writes the
/// verdict. The status is 0 for correct, 1 for incorrect, 3 for unknown.
///
/// # Errors
///
/// A problem whose sequent cannot be read, flags that do not go together,
/// and an output that cannot be written: the answer's faults are its
/// verdict.
pub(crate) fn grade(args: &GradeArgs, given: &Given) -> Result<Status> {
    let start = Instant::now();
    let settings = args.settings(given)?;
    let deadline = Deadline::start(time_limit(&settings), start)?;
    let limits = settings.limits;
    if (args.logic.is_some() || args.system.is_some() || args.calculus.is_some())
        && args.mode.given()
    {
        bail!(
            "--logic, --system and --calculus name the problem's logic: leave out the mode flags"
        );
    }
    if args.input.sequent.is_none() && args.input.file.is_empty() {
        bail!("give the problem's sequent as the argument or with --file");
    }
    let from_stdin = args.input.file.iter().any(|f| f.as_os_str() == "-");
    if from_stdin && args.answer.is_none() {
        bail!("the sequent and the answer cannot both come from standard input: give --answer");
    }
    // A sequent from standard input is read once, as text in the format
    // the flags name, so that a session reads it again as its argument; a
    // file is read where it is needed, in the format its name says.
    let input = if from_stdin {
        SequentInput {
            sequent: Some(
                args.input
                    .text_of(args.logic.is_some(), limits.memory_bytes)?,
            ),
            file: Vec::new(),
            ..args.input.clone()
        }
    } else {
        args.input.clone()
    };
    // The problem, and its sequent as text for the checks that read it so:
    // a model's and a derivation's in a calculus given as data.
    let (problem, text) = if let Some(logic) = args.logic {
        let sequent = input.ordinary(&limits)?;
        let text = sequent.to_string();
        (Problem::Ordinary(Box::new(sequent), logic.into()), text)
    } else if let Some(system) = args.system {
        let sequent = input.modal(&limits)?;
        let text = sequent.to_string();
        (Problem::Modal(Box::new(sequent), system), text)
    } else if args.calculus.is_some() {
        // The session reads the sequent in the calculus, and refuses it there.
        let Some(text) = input.sequent.clone() else {
            bail!("with --calculus, give the sequent to derive as the argument");
        };
        (Problem::Calculus, text)
    } else {
        let mode = input.mode(args.mode.mode());
        mode.check()?;
        let sequent = input.sequent(&limits, &settings.input, mode)?;
        let text = sequent.to_string();
        (Problem::Linear(Box::new(sequent), mode), text)
    };
    let answer = io::read(args.answer.as_deref(), "answer", limits.memory_bytes)?;
    let graded = graded(
        args,
        &settings,
        (&problem, &text, &input),
        &answer,
        &deadline,
    )?;
    let status = match graded.verdict {
        Verdict::Correct => Status::Yes,
        Verdict::Incorrect => Status::No,
        Verdict::Unknown => Status::Unknown,
    };
    let mut out = io::Output::open(None, false)?;
    match args.format {
        AnswerFormat::Json => serde_json::to_writer(out.stream(), &graded)?,
        AnswerFormat::Text => {
            use std::io::Write as _;
            let verdict = match graded.verdict {
                Verdict::Correct => "correct",
                Verdict::Incorrect => "incorrect",
                Verdict::Unknown => "unknown",
            };
            let at = graded
                .line
                .map(|line| format!(" (line {line})"))
                .unwrap_or_default();
            write!(out.stream(), "{verdict}: {}{at}", graded.message)?;
        }
    }
    out.finish()?;
    Ok(status)
}

/// Grades the answer's text against the problem: a JSON document, a
/// tableau or a session's script, told by its shape.
///
/// # Errors
///
/// A calculus that cannot be read, and an output that cannot be made.
fn graded(
    args: &GradeArgs,
    settings: &Settings,
    (problem, text, input): (&Problem, &str, &SequentInput),
    answer: &str,
    deadline: &Deadline,
) -> Result<Graded> {
    let content = answer
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with("```"));
    let Some(first) = content else {
        return Ok(Graded::incorrect("none", "the answer is empty"));
    };
    if first.starts_with('{') {
        return document(args, settings, problem, text, answer, deadline);
    }
    if first.starts_with(|c: char| c.is_ascii_digit()) {
        return tableau(settings, problem, text, answer, deadline);
    }
    script(args, settings, problem, input, answer)
}

/// The settings a check runs under, and the verdict line it writes, kept
/// in memory.
fn shown(settings: &Settings) -> Show {
    let mut show = Show::session(
        crate::argument_parsing::Format::Text,
        settings.view,
        settings.limits,
        settings.styles.clone(),
    );
    show.tree = Tree::Never;
    show.abbreviate = Some(crate::argument_parsing::ABBREVIATE as usize);
    show
}

/// Runs a check that writes its verdict line into an output, kept in
/// memory, and returns its status and that line.
///
/// # Errors
///
/// The check's own errors.
fn kept(
    check: impl FnOnce(&mut io::Output) -> Result<(Status, bool)> + Send,
    stack: usize,
) -> Result<(Status, String)> {
    let held = io::Held::default();
    let sink = io::Sink::Held(held.clone());
    let status = on_large_stack(stack, move || {
        let mut out = sink.open(false)?;
        let (status, _) = check(&mut out)?;
        out.finish()?;
        anyhow::Ok(status)
    })??;
    Ok((status, first_line(&held.text())))
}

/// Grades the verdict of a check that holds where the answer is right.
fn verdict_of(answer: &'static str, (status, line): (Status, String)) -> Graded {
    let line = line
        .strip_prefix("unknown: ")
        .map_or(line.clone(), str::to_owned);
    let verdict = match status {
        Status::Yes => Verdict::Correct,
        Status::Unknown => Verdict::Unknown,
        Status::No | Status::Error => Verdict::Incorrect,
    };
    Graded::new(verdict, answer, line)
}

/// Grades an error met while an answer was read or checked: a limit's
/// refusal is unknown, any other fault the answer's.
fn fault(answer: &'static str, error: &anyhow::Error) -> Graded {
    let limit = error
        .downcast_ref::<deduxit::Error>()
        .is_some_and(|e| matches!(e.kind(), ErrorKind::Limit | ErrorKind::Stopped));
    let verdict = if limit {
        Verdict::Unknown
    } else {
        Verdict::Incorrect
    };
    Graded::new(verdict, answer, format!("{error:#}"))
}

/// Grades a JSON document: read as `check` reads one, about the problem's
/// sequent in its logic, then checked.
///
/// # Errors
///
/// An output that cannot be made.
fn document(
    args: &GradeArgs,
    settings: &Settings,
    problem: &Problem,
    text: &str,
    answer: &str,
    deadline: &Deadline,
) -> Result<Graded> {
    let limits = settings.limits;
    let read = match check::read_document(answer, &limits) {
        Ok(read) => read,
        Err(error) => return Ok(fault("none", &error.context("the answer does not read"))),
    };
    let stack = limits.stack_bytes();
    let show = shown(settings);
    let graded = match (read, problem) {
        (Document::Proof(proof), Problem::Linear(sequent, mode)) => {
            if proof.goal().is_some() {
                return Ok(Graded::incorrect(
                    "proof",
                    "the proof is of a goal, not of the sequent",
                ));
            }
            let stated = proof.mode().unwrap_or(*mode);
            if stated != *mode || !same(proof.sequent(), sequent) {
                return Ok(Graded::incorrect(
                    "proof",
                    format!(
                        "the proof is of {} in {stated}, and the problem is {sequent} in {mode}",
                        proof.sequent()
                    ),
                ));
            }
            let mode = *mode;
            kept(
                move |out| check::check_into(&proof, mode, &show, true, deadline, out),
                stack,
            )
            .map(|kept| verdict_of("proof", kept))
        }
        (Document::Disproof(disproof), Problem::Linear(sequent, mode)) => {
            if disproof.mode() != *mode || !same(disproof.sequent(), sequent) {
                return Ok(Graded::incorrect(
                    "refutation",
                    format!(
                        "the refutation is of {} in {}, and the problem is {sequent} in {mode}",
                        disproof.sequent(),
                        disproof.mode()
                    ),
                ));
            }
            kept(
                move |out| check::check_disproof(*disproof, &show, deadline, out),
                stack,
            )
            .map(|kept| verdict_of("refutation", kept))
        }
        (Document::Ordinary(outcome), Problem::Ordinary(sequent, logic)) => {
            if outcome.logic != *logic {
                return Ok(Graded::incorrect(
                    "outcome",
                    format!(
                        "the outcome is of {} logic, and the problem of {logic} logic",
                        outcome.logic
                    ),
                ));
            }
            if let ordinary::Verdict::Valid(evidence) = &outcome.verdict
                && let Ok(derivation) = evidence.derivation()
                && !proves_in(derivation.logic(), *logic)
            {
                return Ok(Graded::incorrect(
                    "outcome",
                    format!(
                        "the outcome's derivation is of {} logic, which proves nothing of {logic} \
                         logic",
                        derivation.logic()
                    ),
                ));
            }
            let about = match (&outcome.verdict, &outcome.countermodel) {
                (ordinary::Verdict::Valid(evidence), _) => evidence.derivation().ok().map(|d| {
                    let (left, right) = d.sequent();
                    (texts(d.formulas(), left), texts(d.formulas(), right))
                }),
                (ordinary::Verdict::NotValid, Some(countermodel)) => {
                    Some(sides(&countermodel.sequent))
                }
                _ => None,
            };
            match about {
                Some(about) if !same_sides(&sides(sequent), &about, *logic) => {
                    return Ok(Graded::incorrect(
                        "outcome",
                        format!("the outcome is not of the problem's sequent, {sequent}"),
                    ));
                }
                Some(_) => {}
                None => {
                    return Ok(Graded::new(
                        Verdict::Unknown,
                        "outcome",
                        "the outcome holds no derivation or countermodel to check",
                    ));
                }
            }
            let (settings, flags) = (settings.clone(), args.mode.clone());
            let held = io::Held::default();
            let sink = io::Sink::Held(held.clone());
            check::check_ordinary(*outcome, &flags, &settings, show, true, deadline, sink)
                .map(|status| verdict_of("outcome", (status, first_line(&held.text()))))
        }
        (Document::Derivation(derivation), Problem::Ordinary(sequent, logic)) => {
            if !proves_in(derivation.logic(), *logic) {
                return Ok(Graded::incorrect(
                    "derivation",
                    format!(
                        "the derivation is of {} logic, which proves nothing of {logic} logic",
                        derivation.logic()
                    ),
                ));
            }
            let (left, right) = derivation.sequent();
            let about = (
                texts(derivation.formulas(), left),
                texts(derivation.formulas(), right),
            );
            if !same_sides(&sides(sequent), &about, *logic) {
                return Ok(Graded::incorrect(
                    "derivation",
                    format!("the derivation is not of the problem's sequent, {sequent}"),
                ));
            }
            kept(
                move |out| check::check_derivation(&derivation, None, &show, true, deadline, out),
                stack,
            )
            .map(|kept| verdict_of("derivation", kept))
        }
        (Document::Model(model), Problem::Ordinary(_, logic)) => {
            let (text, logic, settings) = (text.to_owned(), *logic, settings.clone());
            kept(
                move |out| {
                    check::check_model(
                        &model,
                        (&text, logic),
                        None,
                        (&settings, &show),
                        deadline,
                        out,
                    )
                },
                stack,
            )
            .map(refuted)
        }
        (Document::Model(model), Problem::Modal(_, system)) => {
            let (text, system, settings) = (text.to_owned(), *system, settings.clone());
            kept(
                move |out| {
                    crate::modal::check_model(
                        &model,
                        (&text, system),
                        None,
                        (&settings, &show),
                        deadline,
                        out,
                    )
                },
                stack,
            )
            .map(refuted)
        }
        (Document::Modal(outcome), Problem::Modal(sequent, system)) => {
            if outcome.system != *system || outcome.sequent.to_string() != sequent.to_string() {
                return Ok(Graded::incorrect(
                    "outcome",
                    format!(
                        "the outcome is of {} in {}, and the problem is {sequent} in {system}",
                        outcome.sequent, outcome.system
                    ),
                ));
            }
            kept(
                move |out| crate::modal::check_outcome(&outcome, &show, deadline, out),
                stack,
            )
            .map(|kept| verdict_of("outcome", kept))
        }
        (Document::Rules(derivation), Problem::Calculus) => {
            return rules(args, settings, text, *derivation, show, deadline);
        }
        (Document::Unchecked(line), _) => {
            return Ok(Graded::new(Verdict::Unknown, "outcome", line));
        }
        (read, _) => {
            return Ok(Graded::incorrect(
                kind(&read),
                format!(
                    "{} answers no problem of {}",
                    what(&read),
                    logic_of(problem)
                ),
            ));
        }
    };
    Ok(graded.unwrap_or_else(|error| fault("none", &error)))
}

/// Grades a derivation in the calculus `--calculus` names: its root must
/// be the problem's sequent, its assumptions among the problem's.
///
/// # Errors
///
/// A calculus that cannot be read.
fn rules(
    args: &GradeArgs,
    settings: &Settings,
    text: &str,
    derivation: deduxit::rules::Derivation,
    show: Show,
    deadline: &Deadline,
) -> Result<Graded> {
    let limits = settings.limits;
    let calculus = match crate::calculus::calculus(args.calculus.as_deref(), &derivation, &limits) {
        Ok(calculus) => calculus,
        Err(error) => return Ok(fault("derivation", &error)),
    };
    let assumptions: Vec<&str> = args.assume.iter().map(String::as_str).collect();
    let start = deduxit::rules::Interactive::within(calculus.clone(), text, &assumptions, &limits)?;
    let start = start.derivation()?;
    let shown_at = |d: &deduxit::rules::Derivation, node| {
        calculus.show_sequent(d, node).limit(usize::MAX).to_string()
    };
    let (Some(root), Some(goal)) = (derivation.root(), start.root()) else {
        return Ok(Graded::incorrect("derivation", "the derivation is empty"));
    };
    let (root, goal) = (
        shown_at(&derivation, root.id()),
        shown_at(&start, goal.id()),
    );
    if root != goal {
        return Ok(Graded::incorrect(
            "derivation",
            format!("the derivation is of {root}, and the problem is {goal}"),
        ));
    }
    // Each assumption as the calculus writes it: the goal of a session on it.
    let mut allowed = Vec::new();
    for assumed in &assumptions {
        let alone = deduxit::rules::Interactive::within(calculus.clone(), assumed, &[], &limits)?
            .derivation()?;
        if let Some(root) = alone.root() {
            allowed.push(shown_at(&alone, root.id()));
        }
    }
    if let Some(extra) = derivation
        .assumptions()
        .map(|node| shown_at(&derivation, node.id()))
        .find(|assumed| !allowed.contains(assumed))
    {
        return Ok(Graded::incorrect(
            "derivation",
            format!("the derivation assumes {extra}, which the problem does not give"),
        ));
    }
    let stack = limits.stack_bytes();
    Ok(kept(
        move |out| crate::calculus::check_rules(&derivation, &calculus, &show, true, deadline, out),
        stack,
    )
    .map_or_else(
        |error| fault("derivation", &error),
        |kept| verdict_of("derivation", kept),
    ))
}

/// Grades the check of a model against the problem: the answer is right
/// where the sequent fails in the model.
fn refuted((status, line): (Status, String)) -> Graded {
    let verdict = match status {
        Status::No => Verdict::Correct,
        Status::Unknown => Verdict::Unknown,
        Status::Yes | Status::Error => Verdict::Incorrect,
    };
    let line = if status == Status::Yes {
        format!("the model does not refute the sequent: {line}")
    } else {
        line
    };
    Graded::new(verdict, "countermodel", line)
}

/// Grades a signed tableau, which answers a problem of classical logic.
///
/// # Errors
///
/// An output that cannot be made.
fn tableau(
    settings: &Settings,
    problem: &Problem,
    text: &str,
    answer: &str,
    deadline: &Deadline,
) -> Result<Graded> {
    if !matches!(problem, Problem::Ordinary(_, Logic::Classical)) {
        return Ok(Graded::incorrect(
            "tableau",
            format!(
                "a signed tableau answers a problem of classical logic, and this is one of {}",
                logic_of(problem)
            ),
        ));
    }
    let (show, settings, text, answer) = (
        shown(settings),
        settings.clone(),
        text.to_owned(),
        answer.to_owned(),
    );
    let stack = settings.limits.stack_bytes();
    Ok(kept(
        move |out| crate::tableau::review(&answer, Some(&text), (&settings, &show), deadline, out),
        stack,
    )
    .map_or_else(
        |error| fault("tableau", &error),
        |kept| verdict_of("tableau", kept),
    ))
}

/// Grades a script of `interact`'s commands: run on the problem's sequent,
/// it must end with the proof finished.
///
/// # Errors
///
/// A session that cannot start on the problem's sequent.
fn script(
    args: &GradeArgs,
    settings: &Settings,
    problem: &Problem,
    input: &SequentInput,
    answer: &str,
) -> Result<Graded> {
    if let Problem::Modal(..) = problem {
        return Ok(Graded::incorrect(
            "script",
            "modal logic has no session: answer with a derivation (prove's outcome) or a \
             countermodel",
        ));
    }
    let interact = session_args(args, input)?;
    let ran = interact::session(
        &interact,
        settings.clone(),
        Input::Script(answer.to_owned()),
    )?;
    let commands = crate::eliminate::counted(ran.commands, "command", "commands");
    let mut graded = if ran.finished {
        Graded::new(
            Verdict::Correct,
            "script",
            format!(
                "the script's {commands} {} the proof, which checks",
                if ran.commands == 1 {
                    "finishes"
                } else {
                    "finish"
                }
            ),
        )
    } else {
        Graded::incorrect(
            "script",
            format!(
                "the script's {commands} {} the proof unfinished",
                if ran.commands == 1 { "leaves" } else { "leave" }
            ),
        )
    };
    if let Some((line, why)) = ran.failed {
        graded
            .message
            .push_str(&format!("; the first that failed: {why}"));
        graded.line = Some(line);
    }
    Ok(graded)
}

/// Returns the arguments of a session on the problem: `interact`'s
/// defaults with the problem's sequent, logic and calculus.
///
/// # Errors
///
/// None in practice: the defaults parse.
fn session_args(args: &GradeArgs, input: &SequentInput) -> Result<InteractArgs> {
    let matches = InteractArgs::augment_args(clap::Command::new("grade"))
        .no_binary_name(true)
        .try_get_matches_from(std::iter::empty::<&str>())?;
    let mut interact = InteractArgs::from_arg_matches(&matches)?;
    interact.input = input.clone();
    interact.mode = args.mode.clone();
    interact.logic.logic = args.logic;
    interact.calculus.clone_from(&args.calculus);
    interact.assume.clone_from(&args.assume);
    Ok(interact)
}

/// Returns whether a derivation of logic `derivation` proves its sequent
/// in logic `problem`: minimal logic's derivations are intuitionistic
/// ones, and intuitionistic logic's classical ones.
fn proves_in(derivation: Logic, problem: Logic) -> bool {
    derivation == problem
        || matches!(
            (derivation, problem),
            (Logic::Minimal, Logic::Intuitionistic | Logic::Classical)
                | (Logic::Intuitionistic, Logic::Classical)
        )
}

/// Returns the first line of a check's output, its verdict line.
fn first_line(text: &str) -> String {
    text.trim().lines().next().unwrap_or_default().to_owned()
}

/// Returns whether two linear sequents are written alike.
fn same(a: &deduxit::linear::Sequent, b: &deduxit::linear::Sequent) -> bool {
    a.to_string() == b.to_string()
}

/// Returns the formulas of `ids` as text, sorted: a side of a sequent read
/// as a multiset.
fn texts(formulas: &Formulas, ids: &[NodeId]) -> Vec<String> {
    let mut texts: Vec<String> = ids
        .iter()
        .map(|&id| formulas.formula(id).to_string())
        .collect();
    texts.sort_unstable();
    texts
}

/// Returns an ordinary sequent's sides as text, as [`texts`] does.
fn sides(sequent: &ordinary::Sequent) -> (Vec<String>, Vec<String>) {
    (
        texts(sequent.formulas(), sequent.left()),
        texts(sequent.formulas(), sequent.right()),
    )
}

/// Returns whether an answer's sides are the problem's, read as
/// multisets; outside classical logic a problem with nothing right of `⊢`
/// is `Γ ⊢ ⊥`, as the logic decides it.
fn same_sides(
    problem: &(Vec<String>, Vec<String>),
    answer: &(Vec<String>, Vec<String>),
    logic: Logic,
) -> bool {
    if problem.0 != answer.0 {
        return false;
    }
    if problem.1 == answer.1 {
        return true;
    }
    let falsum = falsum();
    logic != Logic::Classical && problem.1.is_empty() && answer.1 == [falsum]
}

/// Returns `⊥` as an ordinary formula is printed.
fn falsum() -> String {
    let mut formulas = Formulas::default();
    formulas
        .add(Node::False)
        .map_or_else(|_| "⊥".to_owned(), |id| formulas.formula(id).to_string())
}

/// Returns the kind of answer a document is, as `grade`'s JSON names it.
const fn kind(document: &Document) -> &'static str {
    match document {
        Document::Proof(_) => "proof",
        Document::Disproof(_) => "refutation",
        Document::Ordinary(_) | Document::Modal(_) | Document::Unchecked(_) => "outcome",
        Document::Derivation(_) | Document::Rules(_) => "derivation",
        Document::Model(_) => "countermodel",
    }
}

/// Returns what a document is, in words.
const fn what(document: &Document) -> &'static str {
    match document {
        Document::Proof(_) => "a proof of linear logic",
        Document::Disproof(_) => "a refutation in linear logic",
        Document::Ordinary(_) => "an outcome of ordinary logic",
        Document::Derivation(_) => "a derivation of LK or LJ",
        Document::Unchecked(_) => "an outcome",
        Document::Model(_) => "a Kripke model",
        Document::Rules(_) => "a derivation in a calculus given as data",
        Document::Modal(_) => "an outcome of modal logic",
    }
}

/// Returns the problem's logic, in words.
fn logic_of(problem: &Problem) -> String {
    match problem {
        Problem::Linear(_, mode) => format!("linear logic ({mode})"),
        Problem::Ordinary(_, logic) => format!("{logic} logic"),
        Problem::Modal(_, system) => format!("the modal system {system}"),
        Problem::Calculus => "a calculus given as data".to_owned(),
    }
}
