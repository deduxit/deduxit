// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use crate::argument_parsing::{CheckArgs, Format, ModeArgs};
use crate::io;
use crate::limit::Deadline;
use crate::prove::{Stop, on_large_stack, stopped};
use crate::settings::{Given, time_limit};
use crate::show::{
    Show, Shown, abbreviated, derivation, disproof_into, every_steps, lambda_json, lambda_line,
    linear_term, net_into, net_json, nets_exist,
};
use crate::{Status, exit_on_interrupt, interrupted};
use anyhow::{Context, Result, anyhow, bail};
use deduxit::{
    Error, ErrorKind, Limits,
    linear::{CheckError, Disproof, Forest, Mode, Proof, Reading},
};
use std::time::Instant;

/// Runs `check`: reads a proof, or a disproof, checks it in the mode the
/// file states, or the flags give where it states none, under the time
/// limit and the bounds, and prints the verdict and the proof's
/// derivation, or the verdict as JSON.
///
/// # Errors
///
/// A mode no calculus has, a file that is no proof in JSON, a proof of a
/// goal, a proof or disproof of another mode than the flags name, or an
/// output that cannot be made; an invalid proof is a verdict, and a check
/// given up at a limit is unknown, neither an error.
pub(crate) fn check(args: &CheckArgs, given: &Given) -> Result<Status> {
    let start = Instant::now();
    let settings = args.settings(given)?;
    let deadline = Deadline::start(time_limit(&settings), start)?;
    let mode = args.mode.mode();
    mode.check()?;
    let quiet = args.output.quiet;
    let show = Show::new(&args.output, &settings)?;
    let path = args.output.output.clone();
    exit_on_interrupt();
    // The file is read and parsed within the time limit, as `prove` reads
    // its sequent: a large proof file takes seconds to parse.
    let (source, limits) = (args.source(), settings.limits);
    let loaded = deadline.within(move || {
        let text = io::read(source.as_deref(), "proof", limits.memory_bytes)?;
        read_document(&text, &limits)
    })?;
    let Some(document) = loaded else {
        let limit = deadline.limit().expect("only a limit passes");
        let line = format!(
            "unknown: the time limit of {limit:?} was reached while the proof was read; \
             --timeout DURATION gives the check longer"
        );
        let mut out = io::Output::open(path.as_deref(), show.format.is_binary())?;
        let keep = if show.format == Format::Json {
            let checked = Checked {
                version: CHECKED_LEVEL,
                verdict: "unknown",
                mode,
                error: None,
                message: Some(line),
                lambda: None,
            };
            serde_json::to_writer(out.stream(), &checked)?;
            true
        } else {
            // An unknown is written whatever --no-verdict says, as the
            // check's own is.
            show.open(&mut out, Some(&line))?;
            show.close(&mut out, Shown::Nothing, Some(&line))?
        };
        if keep {
            out.finish()?;
        }
        return Ok(Status::Unknown);
    };
    let proof = match document? {
        Document::Proof(proof) => *proof,
        Document::Disproof(disproof) => {
            // A refutation is a claim about the mode it was found in.
            if args.mode.given() && mode != disproof.mode() {
                bail!(
                    "the refutation is of {} and the flags ask for {mode}: pass the flags it was \
                     found with, or none",
                    disproof.mode()
                );
            }
            let mut out = io::Output::open(path.as_deref(), show.format.is_binary())?;
            let (status, keep) = check_disproof(*disproof, &show, &deadline, &mut out)?;
            if keep {
                out.finish()?;
            }
            return Ok(status);
        }
    };
    if proof.goal().is_some() {
        return Err(Error::GoalProof.into());
    }
    let mode = proof_mode(&proof, &args.mode)?;
    on_large_stack(settings.limits.stack_bytes(), move || {
        let mut out = io::Output::open(path.as_deref(), show.format.is_binary())?;
        let (status, keep) = check_into(&proof, mode, &show, quiet, &deadline, &mut out)?;
        if keep {
            out.finish()?;
        }
        anyhow::Ok(status)
    })?
}

/// Returns the mode a proof is checked in: the one its file states, or
/// the flags' where it states none.
///
/// # Errors
///
/// A file that states another mode than the flags name.
pub(crate) fn proof_mode(proof: &Proof, flags: &ModeArgs) -> Result<Mode> {
    let asked = flags.mode();
    match proof.mode() {
        Some(stated) if flags.given() && stated != asked => bail!(
            "the proof is of {stated} and the flags ask for {asked}: pass the flags it was found \
             with, or none"
        ),
        Some(stated) => Ok(stated),
        None => Ok(asked),
    }
}

/// What `check` reads: a proof, or a disproof, which an unprovable
/// outcome holds too.
enum Document {
    /// A proof.
    Proof(Box<Proof>),
    /// A disproof.
    Disproof(Box<Disproof>),
}

/// Reads a proof file, or failing that a disproof, which an unprovable
/// outcome's keys make; its sequent within `limits.occurrences`.
///
/// # Errors
///
/// A document that is neither, or past the limit.
fn read_document(text: &str, limits: &Limits) -> Result<Document> {
    /// The key of a disproof that a proof lacks; serde skips the rest
    /// without holding it.
    #[derive(serde::Deserialize)]
    struct Keys {
        /// The refutation.
        refutation: Option<serde::de::IgnoredAny>,
    }
    let error = match read_proof(text, limits) {
        Ok(proof) => return Ok(Document::Proof(Box::new(proof))),
        Err(error) => error,
    };
    let refuted = serde_json::from_str::<Keys>(text).is_ok_and(|keys| keys.refutation.is_some());
    if !refuted {
        return Err(error);
    }
    let mut document = serde_json::Deserializer::from_str(text);
    let disproof = deduxit::wire::upgrade::<Disproof, _>(&mut document, limits)
        .map_err(|error| limit_advice(error).unwrap_or_else(|error| error.into()))?;
    document
        .end()
        .context("not a disproof in JSON: more follows it")?;
    Ok(Document::Disproof(Box::new(disproof)))
}

/// Returns a reader's refusal at a limit with the flag that raises it,
/// or the error back where it is none.
fn limit_advice(error: Error) -> std::result::Result<anyhow::Error, Error> {
    // Only the occurrence and term bounds have flags: the width of the
    // representation's indices is no setting.
    match error.setting() {
        Some("limits.occurrences") => Ok(anyhow!("{error}; raise it with --occurrence-limit")),
        Some("limits.terms") => Ok(anyhow!("{error}; raise it with --term-limit")),
        _ if error.kind() == ErrorKind::Limit => Ok(error.into()),
        _ => Err(error),
    }
}

/// Checks a disproof's refutation in the mode it states, within the
/// time limit, and writes the verdict, as text with the refutation's
/// phrase or as `check`'s JSON, and with `--format rocq` the certificate
/// of a refutation that holds; returns the verdict's status (valid 0,
/// invalid 1, unknown 3 for a refutation that rests on the search, the
/// time limit or a limit of the sequent) and whether the output is to be
/// finished.
fn check_disproof(
    disproof: Disproof,
    show: &Show,
    deadline: &Deadline,
    out: &mut io::Output,
) -> Result<(Status, bool)> {
    let mode = disproof.mode();
    // A saturated set's check has no stop of its own.
    let checked = disproof.clone();
    let result = deadline.within(move || checked.check())?;
    let status = match &result {
        Some(Ok(())) => Status::Yes,
        Some(Err(error)) if error.kind() == ErrorKind::Invalid => Status::No,
        Some(Err(_)) | None => Status::Unknown,
    };
    let sequent = shown_sequent(disproof.sequent(), mode, show);
    let line = match &result {
        Some(Ok(())) => format!("valid refutation of {sequent} ({mode}): {disproof}"),
        Some(Err(error)) if status == Status::No => {
            format!("invalid refutation of {sequent} ({mode}): {error}")
        }
        Some(Err(error)) => {
            format!("unknown: the refutation of {sequent} ({mode}) is not checked: {error}")
        }
        None => format!(
            "unknown: the refutation of {sequent} ({mode}) is not checked: the time limit of {:?} \
             was reached; --timeout DURATION gives the check longer",
            deadline.limit().expect("only a limit passes")
        ),
    };
    if show.format == Format::Json {
        let error = match &result {
            Some(Err(error)) => Some(error.clone()),
            _ => None,
        };
        let checked = Checked {
            version: CHECKED_LEVEL,
            verdict: match status {
                Status::Yes => "valid",
                Status::No => "invalid",
                _ => "unknown",
            },
            mode,
            message: (status != Status::Yes).then_some(line),
            error,
            lambda: None,
        };
        serde_json::to_writer(out.stream(), &checked)?;
        return Ok((status, true));
    }
    let line = (show.verdict || status != Status::Yes).then_some(line);
    let prefix = show.open(out, line.as_deref())?;
    let shown = if status == Status::Yes && show.format == Format::Rocq {
        disproof_into(&disproof, None, show, prefix.as_deref(), out)
    } else {
        Shown::Nothing
    };
    Ok((status, show.close(out, shown, line.as_deref())?))
}

/// Returns a sequent as `check` names it in a verdict: two-sided in an
/// intuitionistic mode where it has the reading, one-sided otherwise.
fn shown_sequent(sequent: &deduxit::linear::Sequent, mode: Mode, show: &Show) -> String {
    let formulas = sequent.roots().len();
    match Forest::new(sequent)
        .ok()
        .filter(|_| mode.is_intuitionistic())
        .and_then(|forest| {
            Reading::of_mode(&forest, mode)
                .ok()
                .map(|reading| abbreviated(&reading, show.abbreviate, formulas))
        }) {
        Some(two_sided) => two_sided,
        None => abbreviated(sequent, show.abbreviate, formulas),
    }
}

/// The wire level of `check`'s JSON: level 1 holds a verdict, a mode and
/// an error whole.
pub(crate) const CHECKED_LEVEL: u32 = 1;

/// What `check --format json` writes.
#[derive(serde::Serialize)]
pub(crate) struct Checked {
    /// The wire level.
    pub(crate) version: u32,
    /// `valid`, `invalid` or `unknown`.
    pub(crate) verdict: &'static str,
    /// The mode checked in.
    pub(crate) mode: Mode,
    /// Why the proof is not valid, as the library writes its errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<Error>,
    /// The error's message with formulas for its ids, or why the check
    /// was not made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) message: Option<String>,
    /// The proof's lambda-term, where `--lambda` asks for it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) lambda: Option<serde_json::Value>,
}

/// Reads a proof file, its sequent within `limits.occurrences`; the
/// outcome of a sequent that was not proved is refused with what it is.
///
/// # Errors
///
/// A document that is no proof, or past the limit.
pub(crate) fn read_proof(text: &str, limits: &Limits) -> Result<Proof> {
    let mut document = serde_json::Deserializer::from_str(text);
    match deduxit::wire::upgrade::<Proof, _>(&mut document, limits).map_err(limit_advice) {
        Ok(proof) => {
            document
                .end()
                .context("not a proof in JSON: more follows it")?;
            Ok(proof)
        }
        Err(Ok(refused)) => Err(refused),
        Err(Err(error)) => {
            // An outcome without a proof is a file the user may well hand
            // over; say what it is rather than which key it lacks.
            /// The one key of an outcome looked for; serde skips the
            /// rest without holding it.
            #[derive(serde::Deserialize)]
            struct Outcome {
                /// The outcome's verdict.
                verdict: Option<String>,
            }
            let verdict = serde_json::from_str::<Outcome>(text)
                .ok()
                .and_then(|outcome| outcome.verdict);
            if let Some(verdict) = verdict.filter(|v| v != "proved") {
                bail!(
                    "the file is the outcome of a sequent the search found {verdict}: it holds no proof to check"
                );
            }
            Err(error.into())
        }
    }
}

/// Checks the proof, writes the verdict and the derivation into `out`,
/// and returns the verdict's status and whether the output is to be
/// finished ([`Show::close`]); the time limit and a closed standard
/// output stop the check and the derivation after it (Ctrl-C ends the
/// command: `check` installs `exit_on_interrupt`).
fn check_into(
    proof: &Proof,
    mode: Mode,
    show: &Show,
    quiet: bool,
    deadline: &Deadline,
    out: &mut io::Output,
) -> Result<(Status, bool)> {
    let format = show.format;
    let over = || interrupted() || deadline.passed() || io::closed();
    let result = proof.check_within(mode, &show.limits, |_| over());
    let status = match &result {
        Ok(()) => Status::Yes,
        Err(CheckError::Refused(_)) => Status::Unknown,
        Err(_) => Status::No,
    };
    let report = |e: &deduxit::linear::CheckError| {
        e.describe(proof)
            .in_mode(mode)
            .abbreviated(show.abbreviate)
            .to_string()
    };
    if format == Format::Json {
        if show.net && result.is_ok() && !quiet {
            nets_exist(proof.sequent(), mode)?;
            let why = || Stop::why(stopped(deadline));
            if net_json(None, proof, mode, show, &over, &why, out)? {
                return Ok((status, true));
            }
        }
        let (error, message) = match &result {
            Ok(()) => (None, None),
            Err(e) => (Some(Error::from(e.clone())), Some(report(e))),
        };
        let lambda = match (&result, show.lambda) {
            (Ok(()), Some(_)) => {
                let term = linear_term(proof, mode, show, over);
                Some(lambda_json(term, show, over)?)
            }
            _ => None,
        };
        let checked = Checked {
            version: CHECKED_LEVEL,
            verdict: match status {
                Status::Yes => "valid",
                Status::Unknown => "unknown",
                _ => "invalid",
            },
            mode,
            error,
            message,
            lambda,
        };
        serde_json::to_writer(out.stream(), &checked)?;
        return Ok((status, true));
    }
    // A sequent with no intuitionistic reading is an invalid proof in
    // intuitionistic mode, printed one-sided.
    let sequent = shown_sequent(proof.sequent(), mode, show);
    let line = match &result {
        Ok(()) => format!("valid proof of {sequent} ({mode})"),
        Err(e @ CheckError::Refused(refused)) => {
            let advice = match refused.refusal.setting() {
                Some("limits.memory_bytes") => "; raise it with --memory-limit SIZE".to_owned(),
                Some("limits.work") => "; raise it with --work-limit N".to_owned(),
                _ => match stopped(deadline) {
                    Some(Stop::Timeout(t)) => format!(
                        ": the time limit of {t:?} was reached; --timeout DURATION gives the \
                         check longer"
                    ),
                    _ => String::new(),
                },
            };
            format!("unknown: the proof of {sequent} ({mode}) is not checked: {e}{advice}")
        }
        Err(e) => format!("invalid proof of {sequent} ({mode}): {}", report(e)),
    };
    let line = (show.verdict || result.is_err()).then_some(line);
    let prefix = show.open(out, line.as_deref())?;
    // The other formats write the term into their drawing.
    let lambda = |out: &mut io::Output, drawn: bool| match (show.lambda, format) {
        (Some(_), Format::Text) => {
            let term = linear_term(proof, mode, show, over);
            show.after(out, &lambda_line(term, show, over), drawn)
        }
        _ => Ok(()),
    };
    if result.is_err() || quiet {
        let keep = show.close(out, Shown::Nothing, line.as_deref())?;
        if result.is_ok() {
            lambda(out, false)?;
        }
        return Ok((status, keep));
    }
    let why = || Stop::why(stopped(deadline));
    if show.net {
        nets_exist(proof.sequent(), mode)?;
        let shown = net_into(None, proof, mode, show, &over, &why, prefix.as_deref(), out)?;
        return Ok((status, show.close(out, shown, line.as_deref())?));
    }
    let halt = every_steps(&over);
    let shown = derivation(proof, mode, None, show, halt, why, prefix.as_deref(), out)?;
    let drawn = matches!(shown, Shown::Written | Shown::Rendered(_));
    let keep = show.close(out, shown, line.as_deref())?;
    lambda(out, drawn)?;
    Ok((status, keep))
}
