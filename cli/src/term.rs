// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `term`: a lambda-term checked, reduced and typed at its principal
//! type, a term of a type found, and a type's terms listed and counted.

use crate::argument_parsing::{
    AnswerFormat, EnumerateArgs, InhabitArgs, TermCheckArgs, TermCommand, TermInferArgs,
    TermReduceArgs,
};
use crate::limit::Deadline;
use crate::prove::{Stop, on_large_stack, stopped};
use crate::settings::{Given, time_limit};
use crate::{Status, catch_interrupt, interrupted, io};
use anyhow::Result;
use deduxit::lambda::{
    self, Calculus, Count, End, InhabitOptions, Inhabitation, Notation, ReduceOptions, Term,
};
use deduxit::{Error, Limits, Refusal};
use std::fmt::Write;
use std::time::Instant;

/// Runs a `term` subcommand.
///
/// # Errors
///
/// Those of the subcommand.
pub(crate) fn run(command: &TermCommand, given: &Given) -> Result<Status> {
    match command {
        TermCommand::Check(args) => check(args, given),
        TermCommand::Reduce(args) => reduce(args, given),
        TermCommand::Infer(args) => infer(args, given),
        TermCommand::Inhabit(args) => inhabit(args, given),
        TermCommand::Enumerate(args) => enumerate(args, given),
    }
}

/// Runs `term check`: reads a term, checks it, and prints the judgement
/// with every type, or why the term is ill typed. Returns yes for a well
/// typed term, no for an ill typed one, unknown where the time, a bound or
/// Ctrl-C ended the check.
///
/// # Errors
///
/// An input that cannot be read or parsed, a limit, an output that
/// cannot be written.
fn check(args: &TermCheckArgs, given: &Given) -> Result<Status> {
    catch_interrupt();
    let settings = args.settings(given)?;
    let limits = settings.limits;
    let deadline = Deadline::start(time_limit(&settings), Instant::now())?;
    let text = term_text(args.term.as_ref(), args.file.as_deref(), &limits)?;
    let calculus = Calculus::from(args.calculus);
    let words = Words::of(calculus);
    // A term past the bounds on reading stays an error; its check is
    // what the time and the bounds can end.
    let checked = match Term::parse(&text, calculus, &limits) {
        Ok(term) => term
            .check(&limits, |_| interrupted() || deadline.passed())
            .map(|typing| (term, typing)),
        Err(error @ Error::IllTyped { .. }) => Err(error),
        Err(error) => return Err(error.into()),
    };
    let (term, typing) = match checked {
        Ok(checked) => checked,
        Err(error @ Error::IllTyped { .. }) => {
            return refuse(
                &error,
                calculus,
                words.no,
                args.format,
                args.output.as_deref(),
            );
        }
        Err(error) => return refused(error, &deadline, args.format, args),
    };
    let judgement = judgement(&term, &settings.lambda, &limits)?;
    let out = match args.format {
        AnswerFormat::Text => format!("{}\n{judgement}", words.yes),
        AnswerFormat::Json => serde_json::to_string(&WellTyped {
            term: &term,
            deduxit: env!("CARGO_PKG_VERSION"),
            verdict: &words.yes.replace(' ', "_"),
            typing: calculus.is_typed().then_some(&typing),
            judgement: &judgement,
        })?,
    };
    io::write(args.output.as_deref(), &out)?;
    Ok(Status::Yes)
}

/// The words of `term check`'s verdicts in a calculus.
struct Words {
    /// The verdict on a term the calculus accepts.
    yes: &'static str,
    /// The verdict on a term it refuses.
    no: &'static str,
}

impl Words {
    /// Returns the words of a calculus: a typed term is well typed, an
    /// untyped one well formed.
    const fn of(calculus: Calculus) -> Self {
        if calculus.is_typed() {
            Self {
                yes: "well typed",
                no: "ill typed",
            }
        } else {
            Self {
                yes: "well formed",
                no: "ill formed",
            }
        }
    }
}

/// Returns a term's text: the argument, else the file, else standard
/// input, read within the memory limit.
///
/// # Errors
///
/// A file or standard input that cannot be read.
fn term_text(
    term: Option<&String>,
    file: Option<&std::path::Path>,
    limits: &Limits,
) -> Result<String> {
    Ok(match term {
        Some(term) => term.clone(),
        None => io::read(
            Some(file.unwrap_or_else(|| std::path::Path::new("-"))),
            "term",
            limits.memory_bytes,
        )?,
    })
}

/// Writes the answer on a term that `verdict` refuses, with the reason,
/// and returns no.
///
/// # Errors
///
/// An output that cannot be written.
fn refuse(
    error: &Error,
    calculus: Calculus,
    verdict: &str,
    format: AnswerFormat,
    output: Option<&std::path::Path>,
) -> Result<Status> {
    let out = match format {
        AnswerFormat::Text => format!("{verdict}: {}", reason(error)),
        AnswerFormat::Json => serde_json::to_string(&IllTyped {
            version: level(calculus),
            deduxit: env!("CARGO_PKG_VERSION"),
            verdict: &verdict.replace(' ', "_"),
            error,
        })?,
    };
    io::write(output, &out)?;
    Ok(Status::No)
}

/// Returns the wire level of a term of `calculus`, which the records of
/// `term` carry: 23 for a typed term, 30 for an untyped one, 31 for one of
/// System F.
const fn level(calculus: Calculus) -> u32 {
    if calculus.is_polymorphic() {
        31
    } else if calculus.is_typed() {
        23
    } else {
        30
    }
}

/// `term check`'s record of a well-typed term: the term's document, its
/// keys first, so that the record reads back as the term.
#[derive(serde::Serialize)]
struct WellTyped<'a> {
    /// The term, its level first.
    #[serde(flatten)]
    term: &'a Term,
    /// The crate's version, which wrote the record.
    deduxit: &'static str,
    /// `well_typed`, `well_formed` or `principal_type`.
    verdict: &'a str,
    /// The type of every node, none for an untyped term.
    #[serde(skip_serializing_if = "Option::is_none")]
    typing: Option<&'a lambda::Typing>,
    /// The judgement with every type.
    judgement: &'a str,
}

/// `term check`'s record of an ill-typed term.
#[derive(serde::Serialize)]
struct IllTyped<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the record.
    deduxit: &'static str,
    /// `ill_typed`, `ill_formed` or `no_simple_type`.
    verdict: &'a str,
    /// Why, as the library writes an error.
    error: &'a Error,
}

/// Returns an ill-typed term's reason, without the error's prefix.
fn reason(error: &Error) -> String {
    match error {
        Error::IllTyped { reason, .. } => reason.clone(),
        other => other.to_string(),
    }
}

/// Returns a term's judgement with every type shown, so that it reads
/// back.
///
/// # Errors
///
/// A term past `limits.derivation_bytes`.
fn judgement(term: &Term, options: &lambda::Options, limits: &Limits) -> Result<String> {
    let mut text = String::new();
    term.write_judgement(&mut text, &options.with_types(true), limits, |_| false)?;
    Ok(text)
}

/// Runs `term reduce`: reads a term, reduces it step by step, and prints
/// each term with its redex and how the reduction ended. Returns yes at
/// the normal form and at a bound of steps given, no for a term its
/// calculus refuses, unknown at any other end.
///
/// # Errors
///
/// An input that cannot be read or parsed, an output that cannot be
/// written, a defect the check of a step found.
fn reduce(args: &TermReduceArgs, given: &Given) -> Result<Status> {
    catch_interrupt();
    let settings = args.settings(given)?;
    let limits = settings.limits;
    let deadline = Deadline::start(time_limit(&settings), Instant::now())?;
    let text = term_text(args.term.as_ref(), args.file.as_deref(), &limits)?;
    let calculus = Calculus::from(args.calculus);
    let term = match Term::parse(&text, calculus, &limits) {
        Ok(term) => term,
        Err(error @ Error::IllTyped { .. }) => {
            let no = Words::of(calculus).no;
            return refuse(&error, calculus, no, args.format, args.output.as_deref());
        }
        Err(error) => return Err(error.into()),
    };
    let options = ReduceOptions::default()
        .with_strategy(args.strategy.into())
        .with_steps(args.steps.0)
        .with_nodes(args.node_limit.0);
    let print = settings.lambda;
    let beta = match print.notation {
        Notation::Ascii => "beta",
        _ => "β",
    };
    let mut lines = String::new();
    let mut failed = None;
    let reduced = on_large_stack(limits.stack_bytes(), || {
        term.reduce(
            &options,
            &limits,
            |_| interrupted() || deadline.passed(),
            |before, step| {
                let mut text = String::new();
                match before.write_marked(&mut text, step.redex, &print, &limits, |_| false) {
                    Ok(span) => {
                        let _ = writeln!(lines, "{text}\n  {beta}: {}", &text[span]);
                    }
                    Err(error) => failed = failed.take().or(Some(error)),
                }
            },
        )
    })?;
    let reduced = match reduced {
        Ok(reduced) => reduced,
        Err(error @ Error::IllTyped { .. }) => {
            let no = Words::of(calculus).no;
            return refuse(&error, calculus, no, args.format, args.output.as_deref());
        }
        Err(error) => return refused(error, &deadline, args.format, args),
    };
    if let Some(error) = failed {
        return Err(error.into());
    }
    let mut reached = String::new();
    reduced
        .term
        .write(&mut reached, &print, &limits, |_| false)?;
    let steps = match reduced.steps.len() {
        1 => "1 step".to_owned(),
        n => format!("{n} steps"),
    };
    let (status, end) = match &reduced.end {
        End::NormalForm => (Status::Yes, format!("normal form after {steps}")),
        End::Steps { limit, .. } => (
            if given.has("steps") {
                Status::Yes
            } else {
                Status::Unknown
            },
            format!(
                "not finished after {steps}: the bound of {} was reached; --steps allows more",
                if *limit == 1 {
                    "1 step".to_owned()
                } else {
                    format!("{limit} steps")
                }
            ),
        ),
        End::Nodes { limit, .. } => (
            Status::Unknown,
            format!(
                "not finished after {steps}: the next step would make a term of more than \
                 {limit} nodes; --node-limit allows more"
            ),
        ),
        End::Refused(refusal) => (
            Status::Unknown,
            format!(
                "not finished after {steps}: {}",
                refused_why(refusal, &deadline)
            ),
        ),
        _ => (Status::Unknown, format!("not finished after {steps}")),
    };
    let out = match args.format {
        AnswerFormat::Text => format!("{lines}{reached}\n{end}"),
        AnswerFormat::Json => serde_json::to_string(&Reduced {
            reduction: &reduced,
            deduxit: env!("CARGO_PKG_VERSION"),
            text: &reached,
        })?,
    };
    io::write(args.output.as_deref(), &out)?;
    Ok(status)
}

/// Says why a bound or the stop ended a call, with the flag that lifts
/// the bound: the time limit or Ctrl-C for the stop.
fn refused_why(refusal: &Refusal, deadline: &Deadline) -> String {
    let flag = match refusal.setting() {
        Some("limits.memory_bytes") => "; raise it with --memory-limit",
        Some("limits.work") => "; raise it with --work-limit",
        _ => "",
    };
    if refusal.is_stop() {
        Stop::why(stopped(deadline))
    } else {
        format!("{refusal}{flag}")
    }
}

/// `term reduce`'s record: the library's document, then the writer and
/// the term reached as text.
#[derive(serde::Serialize)]
struct Reduced<'a> {
    /// The reduction.
    #[serde(flatten)]
    reduction: &'a lambda::Reduction,
    /// The crate's version, which wrote the record.
    deduxit: &'static str,
    /// The term reached.
    text: &'a str,
}

/// Runs `term infer`: reads an untyped term and prints it typed at its
/// principal type, or why it has no simple type. Returns yes for a term
/// with a type, no for one without, unknown where the time, a bound or
/// Ctrl-C ended the inference.
///
/// # Errors
///
/// An input that cannot be read or parsed, a typed calculus, a limit, an
/// output that cannot be written.
fn infer(args: &TermInferArgs, given: &Given) -> Result<Status> {
    catch_interrupt();
    let settings = args.settings(given)?;
    let limits = settings.limits;
    let deadline = Deadline::start(time_limit(&settings), Instant::now())?;
    let stop = |_| interrupted() || deadline.passed();
    let text = term_text(args.term.as_ref(), args.file.as_deref(), &limits)?;
    let calculus = Calculus::from(args.calculus);
    let term = Term::parse(&text, calculus, &limits)?;
    // A term its calculus refuses is ill formed; only then is a failure
    // of the inference that the term has no simple type.
    let typed = match term
        .check(&limits, stop)
        .map_err(|error| (error, Words::of(calculus).no))
        .and_then(|_| {
            term.principal_type(&limits, stop)
                .map_err(|error| (error, "no simple type"))
        }) {
        Ok(typed) => typed,
        Err((error @ Error::IllTyped { .. }, no)) => {
            return refuse(&error, calculus, no, args.format, args.output.as_deref());
        }
        Err((error, _)) => return refused(error, &deadline, args.format, args),
    };
    let typing = match typed.check(&limits, stop) {
        Ok(typing) => typing,
        Err(error) => return refused(error, &deadline, args.format, args),
    };
    let judgement = judgement(&typed, &settings.lambda, &limits)?;
    let out = match args.format {
        AnswerFormat::Text => format!("principal type\n{judgement}"),
        AnswerFormat::Json => serde_json::to_string(&WellTyped {
            term: &typed,
            deduxit: env!("CARGO_PKG_VERSION"),
            verdict: "principal_type",
            typing: Some(&typing),
            judgement: &judgement,
        })?,
    };
    io::write(args.output.as_deref(), &out)?;
    Ok(Status::Yes)
}

/// Runs `term inhabit`: decides whether the type has a term, and prints
/// it. Returns yes for a type with a term, no for one without, unknown
/// when the time or a limit ran out.
///
/// # Errors
///
/// An input that cannot be read, an engine's error, an output that
/// cannot be written.
fn inhabit(args: &InhabitArgs, given: &Given) -> Result<Status> {
    catch_interrupt();
    let settings = args.common.settings(given)?;
    let limits = settings.limits;
    let deadline = Deadline::start(time_limit(&settings), Instant::now())?;
    let stop = |_| interrupted() || deadline.passed();
    let sequent = args.common.input.ordinary(&limits)?;
    let answer = on_large_stack(limits.stack_bytes(), || {
        lambda::inhabit(&sequent, &limits, stop)
    })?;
    let answer = match answer {
        Ok(answer) => answer,
        Err(error) => return refused(error, &deadline, args.common.format, args),
    };
    let (status, text) = match &answer {
        Inhabitation::Inhabited(term) => (
            Status::Yes,
            format!("inhabited\n{}", judgement(term, &settings.lambda, &limits)?),
        ),
        Inhabitation::Empty { countermodel, .. } => {
            let mut text = "empty: the formula is not valid in intuitionistic logic".to_owned();
            if let Some(countermodel) = countermodel {
                text.push_str("\ncountermodel:");
                for line in countermodel.model.to_string().lines() {
                    write!(text, "\n  {line}")?;
                }
            }
            (Status::No, text)
        }
        Inhabitation::Unknown(reason) => (Status::Unknown, format!("unknown: {reason}")),
        _ => (Status::Unknown, "unknown".to_owned()),
    };
    let out = match args.common.format {
        AnswerFormat::Json => serde_json::to_string(&answer)?,
        AnswerFormat::Text => text,
    };
    io::write(args.common.output.as_deref(), &out)?;
    Ok(status)
}

/// Writes the line of an answer that a stop or a limit ended, and returns
/// unknown; returns any other error.
///
/// # Errors
///
/// The error given, where it is no refusal; an output that cannot be
/// written.
fn refused<T: HasOutput>(
    error: Error,
    deadline: &Deadline,
    format: AnswerFormat,
    args: &T,
) -> Result<Status> {
    let why = match error {
        Error::Refused(Refusal::Stopped { .. }) => Stop::why(stopped(deadline)),
        Error::Refused(refusal) => {
            let flag = match refusal.setting() {
                Some("limits.memory_bytes") => "; raise it with --memory-limit",
                Some("limits.work") => "; raise it with --work-limit",
                _ => "",
            };
            format!("{refusal}{flag}")
        }
        error => return Err(error.into()),
    };
    let line = format!("unknown: {why}");
    match format {
        AnswerFormat::Text => io::write(args.output(), &line)?,
        AnswerFormat::Json => eprintln!("{line}"),
    }
    Ok(Status::Unknown)
}

/// A command's arguments that name where its answer goes.
trait HasOutput {
    /// The file the answer goes to, or `None` for standard output.
    fn output(&self) -> Option<&std::path::Path>;
}

impl HasOutput for InhabitArgs {
    fn output(&self) -> Option<&std::path::Path> {
        self.common.output.as_deref()
    }
}

impl HasOutput for EnumerateArgs {
    fn output(&self) -> Option<&std::path::Path> {
        self.common.output.as_deref()
    }
}

impl HasOutput for TermCheckArgs {
    fn output(&self) -> Option<&std::path::Path> {
        self.output.as_deref()
    }
}

impl HasOutput for TermReduceArgs {
    fn output(&self) -> Option<&std::path::Path> {
        self.output.as_deref()
    }
}

impl HasOutput for TermInferArgs {
    fn output(&self) -> Option<&std::path::Path> {
        self.output.as_deref()
    }
}

/// Runs `term enumerate`: lists the type's terms and counts them. Returns
/// yes for a type with a term, no for one without, unknown when the
/// search could not tell.
///
/// # Errors
///
/// An input that cannot be read, a type with `∨` or `⊥`, an engine's
/// error, an output that cannot be written.
fn enumerate(args: &EnumerateArgs, given: &Given) -> Result<Status> {
    catch_interrupt();
    let settings = args.common.settings(given)?;
    let limits = settings.limits;
    let deadline = Deadline::start(time_limit(&settings), Instant::now())?;
    let stop = |_| interrupted() || deadline.passed();
    let sequent = args.common.input.ordinary(&limits)?;
    let options = InhabitOptions::default()
        .with_depth(args.depth)
        .with_max_terms(args.max_terms);
    let found = on_large_stack(limits.stack_bytes(), || {
        lambda::enumerate(&sequent, &options, &limits, stop)
    })?;
    let found = match found {
        Ok(found) => found,
        Err(error) => return refused(error, &deadline, args.common.format, args),
    };
    let listed = found.terms.len();
    let (status, head) = match found.count {
        Count::Exactly { count: 0, .. } => (Status::No, "no term".to_owned()),
        Count::Exactly { count: 1, .. } => (Status::Yes, "exactly one term".to_owned()),
        Count::Exactly { count, .. } if count as usize == listed => {
            (Status::Yes, format!("exactly {count} terms"))
        }
        Count::Exactly { count, .. } => (
            Status::Yes,
            format!(
                "exactly {count} terms, the first {listed} listed (raise --max-terms for more)"
            ),
        ),
        Count::Infinite => (
            Status::Yes,
            format!(
                "infinitely many terms, the first {listed} within depth {}",
                args.depth
            ),
        ),
        Count::AtLeast { count: 0, .. } => (
            Status::Unknown,
            format!(
                "no term found within depth {} (raise --depth to search deeper)",
                args.depth
            ),
        ),
        Count::AtLeast { count, .. } => (
            Status::Yes,
            format!(
                "at least {count} terms: --depth {} or a limit cut the search",
                args.depth
            ),
        ),
        _ => (Status::Unknown, "unknown".to_owned()),
    };
    let out = match args.common.format {
        AnswerFormat::Json => serde_json::to_string(&found)?,
        AnswerFormat::Text => {
            let mut text = head;
            let options = settings.lambda.with_types(true);
            for term in &found.terms {
                text.push('\n');
                term.write(&mut text, &options, &limits, |_| false)?;
            }
            text
        }
    };
    io::write(args.common.output.as_deref(), &out)?;
    Ok(status)
}
