// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `check --tableau`: a signed tableau written by hand, read as text and
//! reviewed step by step.

use crate::argument_parsing::{CheckArgs, Format};
use crate::io;
use crate::limit::Deadline;
use crate::show::{Show, Shown};
use crate::{Status, interrupted};
use anyhow::{Result, bail};
use deduxit::Error;
use deduxit::ordinary::{self, Standing, WrittenTableau};

/// Runs `check --tableau`: reads the tableau's text, reviews it against
/// the built-in rule file of the tableau and, where `--sequent` is given,
/// against that sequent, and prints what the review found or its JSON.
/// The status is 0 for a tableau that is right and finished, closed or
/// open, 1 for one with a wrong step or not finished, and 3 where a limit
/// stopped the review.
///
/// # Errors
///
/// A text that is no tableau, a sequent that cannot be read, a format
/// other than text or JSON, and flags that name another kind of file.
pub(crate) fn check(
    args: &CheckArgs,
    (settings, show): (&deduxit::Settings, &Show),
    deadline: &Deadline,
) -> Result<Status> {
    if args.mode.given() {
        bail!("a tableau is of classical logic, which has no mode: leave out the mode flags");
    }
    if !matches!(show.format, Format::Text | Format::Json) {
        bail!("check --tableau writes text or json");
    }
    let limits = settings.limits;
    let text = io::read(args.source().as_deref(), "tableau", limits.memory_bytes)?;
    let mut out = io::Output::open(args.output.output.as_deref(), false)?;
    let (status, keep) = review(
        &text,
        args.sequent.as_deref(),
        (settings, show),
        deadline,
        &mut out,
    )?;
    if keep {
        out.finish()?;
    }
    Ok(status)
}

/// Reviews a tableau's text against the built-in rule file of the
/// tableau and, where `expected` is given, against that sequent, and
/// writes what the review found or its JSON into `out`. Returns the
/// status, as [`check`] says, and whether the output is to be finished.
///
/// # Errors
///
/// A text that is no tableau, and a sequent that cannot be read.
pub(crate) fn review(
    text: &str,
    expected: Option<&str>,
    (settings, show): (&deduxit::Settings, &Show),
    deadline: &Deadline,
    out: &mut io::Output,
) -> Result<(Status, bool)> {
    let limits = settings.limits;
    if text.trim().is_empty() {
        bail!("the tableau is empty: write its first lines, the sequent's formulas signed T and F");
    }
    let tableau = WrittenTableau::parse_within(text, &limits).map_err(|error| match error {
        Error::Parse(_) => crate::parse_error_in(text, error, "the tableau"),
        error => error.into(),
    })?;
    let expected = expected
        .map(|given| {
            ordinary::Sequent::parse_within(given, &limits).map_err(|error| match error {
                Error::Parse(_) => crate::parse_error(given, error),
                error => error.into(),
            })
        })
        .transpose()?;
    let calculus = crate::calculus::named("tableau".as_ref(), &limits)?;
    let over = || interrupted() || deadline.passed() || io::closed();
    let review = match tableau.review(&calculus, expected.as_ref(), &limits, |_| over()) {
        Ok(review) => review,
        Err(error) if error.is_refusal() => {
            let line = format!(
                "unknown: the tableau is not reviewed: {}",
                advice(&error, deadline)
            );
            if show.format == Format::Json {
                eprintln!("{line}");
                return Ok((Status::Unknown, false));
            }
            show.open(out, Some(&line))?;
            let keep = show.close(out, Shown::Nothing, Some(&line))?;
            return Ok((Status::Unknown, keep));
        }
        Err(error) => return Err(error.into()),
    };
    let status = match review.standing {
        Standing::Closed | Standing::Open => Status::Yes,
        _ => Status::No,
    };
    if show.format == Format::Json {
        serde_json::to_writer(out.stream(), &review)?;
        return Ok((status, true));
    }
    let mut line = format!(
        "{} tableau of {}: {}",
        review.standing.word(),
        review.sequent,
        review.message
    );
    match review.standing {
        Standing::Closed => line.push_str(", so the sequent is valid in classical logic"),
        Standing::Open => {
            line.push_str(", so the sequent is not valid in classical logic");
            if let Some(countermodel) = &review.countermodel {
                line.push_str(&format!(": a checked countermodel\n{}", countermodel.model));
            }
        }
        _ => {}
    }
    show.open(out, Some(&line))?;
    let keep = show.close(out, Shown::Nothing, Some(&line))?;
    Ok((status, keep))
}

/// Returns a refusal's message with the flag that raises its bound.
fn advice(error: &Error, deadline: &Deadline) -> String {
    match error.setting() {
        Some("limits.memory_bytes") => format!("{error}; raise it with --memory-limit"),
        Some("limits.work") => format!("{error}; raise it with --work-limit"),
        Some("limits.occurrences") => format!("{error}; raise it with --occurrence-limit"),
        _ if deadline.passed() => {
            "the time limit was reached; --timeout DURATION gives the check longer".to_owned()
        }
        _ => error.to_string(),
    }
}
