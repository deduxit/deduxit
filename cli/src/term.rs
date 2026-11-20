// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `term`: a lambda-term checked, a term of a type found, and a type's
//! terms listed and counted.

use crate::argument_parsing::{
    AnswerFormat, CalculusArg, EnumerateArgs, InhabitArgs, TermCheckArgs, TermCommand,
};
use crate::limit::Deadline;
use crate::prove::{Stop, on_large_stack, stopped};
use crate::settings::{Given, time_limit};
use crate::{Status, catch_interrupt, interrupted, io};
use anyhow::Result;
use deduxit::lambda::{self, Calculus, Count, InhabitOptions, Inhabitation, Term};
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
        TermCommand::Check(args) => check(args),
        TermCommand::Inhabit(args) => inhabit(args, given),
        TermCommand::Enumerate(args) => enumerate(args, given),
    }
}

/// Runs `term check`: reads a term, checks it, and prints the judgement
/// with every type, or why the term is ill typed. Returns yes for a well
/// typed term, no for an ill typed one.
///
/// # Errors
///
/// An input that cannot be read or parsed, a limit, an output that
/// cannot be written.
fn check(args: &TermCheckArgs) -> Result<Status> {
    let settings = args.settings()?;
    let limits = settings.limits;
    let text = match (&args.term, &args.file) {
        (Some(term), _) => term.clone(),
        (None, file) => io::read(
            Some(file.as_deref().unwrap_or_else(|| std::path::Path::new("-"))),
            "term",
            limits.memory_bytes,
        )?,
    };
    let calculus = match args.calculus {
        CalculusArg::Simple => Calculus::Simple,
        CalculusArg::Linear => Calculus::Linear,
        CalculusArg::Affine => Calculus::Affine,
    };
    let checked = Term::parse(&text, calculus, &limits)
        .and_then(|term| term.check(&limits, |_| false).map(|typing| (term, typing)));
    let (term, typing) = match checked {
        Ok(checked) => checked,
        Err(error @ Error::IllTyped { .. }) => {
            let out = match args.format {
                AnswerFormat::Text => format!("ill typed: {}", reason(&error)),
                AnswerFormat::Json => serde_json::to_string(&IllTyped {
                    version: TERM_LEVEL,
                    deduxit: env!("CARGO_PKG_VERSION"),
                    verdict: "ill_typed",
                    error: &error,
                })?,
            };
            io::write(args.output.as_deref(), &out)?;
            return Ok(Status::No);
        }
        Err(error) => return Err(error.into()),
    };
    let judgement = judgement(&term, &settings.lambda, &limits)?;
    let out = match args.format {
        AnswerFormat::Text => format!("well typed\n{judgement}"),
        AnswerFormat::Json => serde_json::to_string(&WellTyped {
            term: &term,
            deduxit: env!("CARGO_PKG_VERSION"),
            verdict: "well_typed",
            typing: &typing,
            judgement: &judgement,
        })?,
    };
    io::write(args.output.as_deref(), &out)?;
    Ok(Status::Yes)
}

/// The wire level of a lambda-term's form, which `term check`'s records
/// carry.
const TERM_LEVEL: u32 = 23;

/// `term check`'s record of a well-typed term: the term's document, its
/// keys first, so that the record reads back as the term.
#[derive(serde::Serialize)]
struct WellTyped<'a> {
    /// The term, its level first.
    #[serde(flatten)]
    term: &'a Term,
    /// The crate's version, which wrote the record.
    deduxit: &'static str,
    /// `well_typed`.
    verdict: &'static str,
    /// The type of every node.
    typing: &'a lambda::Typing,
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
    /// `ill_typed`.
    verdict: &'static str,
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
                "at least {count} terms within depth {} (raise --depth to search deeper)",
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
