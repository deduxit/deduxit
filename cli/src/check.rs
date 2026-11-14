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
    linear::{CheckError, ClassicalSequent, Disproof, Forest, Mode, Proof, Reading},
    ordinary,
};
use std::time::Instant;

/// Runs `check`: reads a proof, or a disproof, checks it in the mode the
/// file states, or the flags give where it states none, under the time
/// limit and the bounds, and prints the verdict and the proof's
/// derivation, or the verdict as JSON. An outcome of ordinary logic, or a
/// derivation of LK or LJ, is checked by the rules of its calculus, with
/// the image's proof or refutation that the embedding found.
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
                mode: Some(mode),
                logic: None,
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
        Document::Ordinary(outcome) => {
            return check_ordinary(
                *outcome, &args.mode, &settings, show, quiet, &deadline, path,
            );
        }
        Document::Derivation(d) => {
            return on_large_stack(settings.limits.stack_bytes(), move || {
                let mut out = io::Output::open(path.as_deref(), show.format.is_binary())?;
                let (status, keep) = check_derivation(&d, None, &show, quiet, &deadline, &mut out)?;
                if keep {
                    out.finish()?;
                }
                anyhow::Ok(status)
            })?;
        }
        Document::Unchecked(line) => {
            let mut out = io::Output::open(path.as_deref(), show.format.is_binary())?;
            if unchecked_into(&line, None, &show, &mut out)? {
                out.finish()?;
            }
            return Ok(Status::Unknown);
        }
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
/// outcome holds too, or an outcome or a derivation of ordinary logic.
enum Document {
    /// A proof.
    Proof(Box<Proof>),
    /// A disproof.
    Disproof(Box<Disproof>),
    /// The outcome of deciding an ordinary sequent.
    Ordinary(Box<ordinary::Outcome>),
    /// A derivation of LK or LJ.
    Derivation(Box<ordinary::Derivation>),
    /// A document that holds nothing to check, and the line that says
    /// why: an ordinary outcome valid by a clausal proof, kept as its
    /// counts.
    Unchecked(String),
}

/// Reads a proof file, or failing that a disproof, which an unprovable
/// outcome's keys make; its sequent within `limits.occurrences`.
///
/// # Errors
///
/// A document that is neither, or past the limit.
fn read_document(text: &str, limits: &Limits) -> Result<Document> {
    /// The keys that tell the documents apart; serde skips the rest
    /// without holding it.
    #[derive(serde::Deserialize)]
    struct Keys {
        /// The refutation, which a disproof has and a proof lacks.
        refutation: Option<serde::de::IgnoredAny>,
        /// The logic of an ordinary outcome or derivation.
        logic: Option<String>,
        /// The verdict of an outcome.
        verdict: Option<serde::de::IgnoredAny>,
        /// The counts of a clausal proof, in an ordinary outcome.
        clausal_proof: Option<serde::de::IgnoredAny>,
    }
    let keys = serde_json::from_str::<Keys>(text).ok();
    if let Some(Keys {
        logic: Some(logic),
        verdict,
        clausal_proof,
        ..
    }) = &keys
    {
        if clausal_proof.is_some() {
            return Ok(Document::Unchecked(format!(
                "unknown: the file is the outcome of a sequent of {logic} logic valid by a clausal \
                 proof, which it keeps as its counts alone: it holds no proof to check"
            )));
        }
        let mut document = serde_json::Deserializer::from_str(text);
        let read = if verdict.is_some() {
            deduxit::wire::upgrade::<ordinary::Outcome, _>(&mut document, limits)
                .map(|outcome| Document::Ordinary(Box::new(outcome)))
        } else {
            deduxit::wire::upgrade::<ordinary::Derivation, _>(&mut document, limits)
                .map(|d| Document::Derivation(Box::new(d)))
        };
        let read =
            read.map_err(|error| limit_advice(error).unwrap_or_else(|error| error.into()))?;
        document
            .end()
            .context("not an outcome in JSON: more follows it")?;
        return Ok(read);
    }
    let error = match read_proof(text, limits) {
        Ok(proof) => return Ok(Document::Proof(Box::new(proof))),
        Err(error) => error,
    };
    let refuted = keys.is_some_and(|keys| keys.refutation.is_some());
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
    // A classical sequent written with hypotheses is shown so, as the
    // disproof's phrase speaks of its sides.
    let two_sided = !mode.is_intuitionistic()
        && disproof.sequent().antecedents().is_some_and(|n| n > 0)
        && disproof.goal().is_none();
    let sequent = match two_sided
        .then(|| ClassicalSequent::read_one_sided(disproof.sequent().clone(), &Limits::default()))
    {
        Some(Ok(written)) => {
            abbreviated(&written, show.abbreviate, disproof.sequent().roots().len())
        }
        _ => shown_sequent(disproof.sequent(), mode, show),
    };
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
            mode: Some(mode),
            logic: None,
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

/// Checks the outcome of deciding an ordinary sequent and writes the
/// verdict: a derivation by the rules of its calculus, with the image's
/// proof where the embedding found one; the image's proof alone where no
/// derivation was built; the image's refutation. A sequent found not
/// valid by LJT or the SAT solver holds nothing to check: unknown.
///
/// # Errors
///
/// An outcome of a sequent found unknown, which holds no proof; a mode
/// flag beside it; an output that cannot be made.
fn check_ordinary(
    outcome: ordinary::Outcome,
    flags: &ModeArgs,
    settings: &deduxit::Settings,
    show: Show,
    quiet: bool,
    deadline: &Deadline,
    path: Option<std::path::PathBuf>,
) -> Result<Status> {
    if flags.given() {
        bail!(
            "the file is an outcome of {} logic, which is checked by its own rules: leave out \
             the mode flags",
            outcome.logic
        );
    }
    let logic = outcome.logic;
    on_large_stack(settings.limits.stack_bytes(), move || {
        let image = match &outcome.search {
            ordinary::Search::Embedding { linear, .. } => Some(&**linear),
            _ => None,
        };
        let mut out = io::Output::open(path.as_deref(), show.format.is_binary())?;
        let (status, keep) = match (&outcome.verdict, image) {
            (ordinary::Verdict::Valid(ordinary::Evidence::Derivation(d)), image) => {
                let proof = image.and_then(|linear| match &linear.verdict {
                    deduxit::linear::Verdict::Proved(proof) => Some((&**proof, linear.mode)),
                    _ => None,
                });
                check_derivation(d, proof, &show, quiet, deadline, &mut out)?
            }
            (ordinary::Verdict::Valid(_), Some(linear)) => match &linear.verdict {
                deduxit::linear::Verdict::Proved(proof) => {
                    let (proof, mode) = (&**proof, linear.mode);
                    check_into(proof, mode, &show, quiet, deadline, &mut out)?
                }
                _ => bail!("the outcome's image holds no proof"),
            },
            (ordinary::Verdict::NotValid, Some(linear)) => match &linear.verdict {
                deduxit::linear::Verdict::Unprovable(disproof) => {
                    check_disproof((**disproof).clone(), &show, deadline, &mut out)?
                }
                _ => bail!("the outcome's image holds no refutation"),
            },
            (ordinary::Verdict::NotValid, None) => {
                let by = match outcome.method() {
                    ordinary::Method::Sat => "the SAT solver",
                    _ => "LJT",
                };
                let line = format!(
                    "unknown: the file is the outcome of a sequent of {logic} logic found not valid by \
                     {by}, and holds no refutation to check"
                );
                (
                    Status::Unknown,
                    unchecked_into(&line, Some(logic), &show, &mut out)?,
                )
            }
            _ => bail!(
                "the file is the outcome of a sequent of {logic} logic whose search found no verdict: \
                 it holds no proof to check"
            ),
        };
        if keep {
            out.finish()?;
        }
        anyhow::Ok(status)
    })?
}

/// Writes the line of a document that holds nothing to check, as text or
/// as `check`'s JSON, whatever `--no-verdict` says, and returns whether
/// the output is to be finished.
fn unchecked_into(
    line: &str,
    logic: Option<ordinary::Logic>,
    show: &Show,
    out: &mut io::Output,
) -> Result<bool> {
    if show.format == Format::Json {
        let checked = Checked {
            version: CHECKED_LEVEL,
            verdict: "unknown",
            mode: None,
            logic,
            error: None,
            message: Some(line.to_owned()),
            lambda: None,
        };
        serde_json::to_writer(out.stream(), &checked)?;
        return Ok(true);
    }
    show.open(out, Some(line))?;
    show.close(out, Shown::Nothing, Some(line))
}

/// Checks a derivation of LK or LJ by the rules of its calculus, and the
/// image's `proof` in its mode where the embedding found one, within the
/// time limit and the bounds, and writes the verdict and the derivation,
/// or the verdict as JSON; returns the verdict's status and whether the
/// output is to be finished.
///
/// # Errors
///
/// An output that cannot be written.
fn check_derivation(
    d: &ordinary::Derivation,
    proof: Option<(&Proof, Mode)>,
    show: &Show,
    quiet: bool,
    deadline: &Deadline,
    out: &mut io::Output,
) -> Result<(Status, bool)> {
    let over = || interrupted() || deadline.passed() || io::closed();
    let mut result = d.check(&show.limits, |_| over());
    if let (Ok(()), Some((proof, mode))) = (&result, proof)
        && let Err(error) = proof.check_within(mode, &show.limits, |_| over())
    {
        result = Err(Error::from(error));
    }
    let status = match &result {
        Ok(()) => Status::Yes,
        Err(error) if error.is_refusal() => Status::Unknown,
        Err(_) => Status::No,
    };
    let logic = d.logic();
    let calculus = logic.calculus();
    let (left, right) = d.sequent();
    let side = |ids: &[ordinary::NodeId]| {
        ids.iter()
            .map(|&id| d.formulas().formula(id).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let sequent = abbreviated(
        &match left {
            [] => format!("⊢ {}", side(right)),
            _ => format!("{} ⊢ {}", side(left), side(right)),
        },
        show.abbreviate,
        left.len() + right.len(),
    );
    let line = match &result {
        Ok(()) => format!("valid derivation in {calculus} of {sequent} ({logic} logic)"),
        Err(error) if status == Status::Unknown => {
            let advice = match error.setting() {
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
            format!(
                "unknown: the derivation in {calculus} of {sequent} ({logic} logic) is not \
                 checked: {error}{advice}"
            )
        }
        Err(error) => {
            format!("invalid derivation in {calculus} of {sequent} ({logic} logic): {error}")
        }
    };
    if show.format == Format::Json {
        let lambda = match (&result, show.lambda) {
            (Ok(()), Some(_)) => {
                let term = d.term(&show.limits, |_| over());
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
            mode: None,
            logic: Some(logic),
            error: result.as_ref().err().cloned(),
            message: result.is_err().then_some(line),
            lambda,
        };
        serde_json::to_writer(out.stream(), &checked)?;
        return Ok((status, true));
    }
    let line = (show.verdict || result.is_err()).then_some(line);
    let prefix = show.open(out, line.as_deref())?;
    if result.is_err() || quiet {
        return Ok((status, show.close(out, Shown::Nothing, line.as_deref())?));
    }
    let why = || Stop::why(stopped(deadline));
    let shown = crate::show::shown(
        crate::ordinary::shown(d.clone(), show),
        show.fit(),
        show,
        every_steps(&over),
        why,
        prefix.as_deref(),
        out,
    )?;
    let drawn = matches!(shown, Shown::Written | Shown::Rendered(_));
    let keep = show.close(out, shown, line.as_deref())?;
    if let (Some(_), Format::Text) = (show.lambda, show.format) {
        let term = d.term(&show.limits, |_| over());
        show.after(out, &lambda_line(term, show, over), drawn)?;
    }
    Ok((status, keep))
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
    /// The mode checked in, for linear logic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mode: Option<Mode>,
    /// The logic checked in, for ordinary logic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) logic: Option<ordinary::Logic>,
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
            /// The keys of an outcome looked for; serde skips the rest
            /// without holding it.
            #[derive(serde::Deserialize)]
            struct Outcome {
                /// The outcome's verdict.
                verdict: Option<String>,
                /// The logic of an ordinary outcome or derivation.
                logic: Option<String>,
            }
            let outcome = serde_json::from_str::<Outcome>(text).ok();
            if let Some(Outcome {
                verdict,
                logic: Some(logic),
            }) = &outcome
            {
                let what = if verdict.is_some() {
                    "an outcome"
                } else {
                    "a derivation"
                };
                bail!(
                    "the file is {what} of {logic} logic, which `check` reads, and this command \
                     reads proofs of linear logic: `prove --logic {logic} --linear --format json` \
                     writes the proof of its image"
                );
            }
            let verdict = outcome.and_then(|outcome| outcome.verdict);
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
            mode: Some(mode),
            logic: None,
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
