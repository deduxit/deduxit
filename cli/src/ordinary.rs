// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Ordinary logic on the command line: a formula or sequent of classical,
//! intuitionistic or minimal logic read under `--logic`, translated into
//! linear logic, decided there, and its proof read back as LK or LJ.

use crate::argument_parsing::{Format, InputFormat, ProveArgs, Tree};
use crate::io::{self, Most, admit, bound, refused};
use crate::limit::{Deadline, Notice};
use crate::prove::{
    Ended, NOTICE_AFTER, Stop, advice, notice_line, on_large_stack, stopped, unknown,
};
use crate::show::{Show, Shown, every_steps, lambda_json, lambda_line, not_built};
use crate::{Status, catch_interrupt, interrupted};
use anyhow::{Result, anyhow, bail};
use deduxit::linear::proofs::{Sides, Structural};
use deduxit::linear::search::{Outcome, Verdict};
use deduxit::ordinary::{self, Derivation, Image, Logic, Method, Translation, ljt};
use deduxit::{
    Error, ErrorKind, Limits, Refusal, Settings,
    linear::{Forest, Proof, ViewOptions},
};
use std::fmt::Write;
use std::time::{Duration, Instant};

/// Returns the ordinary sequent `text` holds in `format`: text in the
/// syntax of ordinary logic (a parse error points into `text`), or a TPTP
/// problem, refused at the first subformula past `most` occurrences.
///
/// # Errors
///
/// Text that is no ordinary sequent, with a caret under the place, no
/// TPTP problem, or one past `most`.
pub(crate) fn sequent_in(
    text: &str,
    format: InputFormat,
    most: crate::io::Most,
) -> Result<ordinary::Sequent> {
    let within = bound(most);
    match format {
        // The library's error says that the text is no TPTP problem.
        InputFormat::Tptp => Ok(ordinary::read_tptp(text, &within)
            .map_err(|e| refused(e, most))?
            .sequent),
        InputFormat::Text | InputFormat::Auto | InputFormat::Lines => {
            ordinary::Sequent::parse_within(text, &within).map_err(|e| match e {
                Error::Parse(_) => crate::parse_error(text, e),
                e => refused(e, most),
            })
        }
        _ => bail!(
            "--logic reads text (one sequent, or one per line with --input-format lines) or TPTP problems"
        ),
    }
}

/// Returns the image of an ordinary sequent in the logic, under the
/// translation, that `options` give, refused past `most` occurrences.
///
/// # Errors
///
/// The translation's error: one that does not decide the logic, more
/// than one formula right of `⊢` in intuitionistic or minimal logic, or
/// an image past `most`.
pub(crate) fn image(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    most: crate::io::Most,
) -> Result<Image> {
    ordinary::translate(sequent, options.logic, options.translation(), &bound(most))
        .map_err(|e| refused(e, most))
}

/// Returns the image of an ordinary sequent under the translation
/// `options` choose, with the image's forest, refused past
/// `limits.occurrences`.
///
/// # Errors
///
/// As [`image`], and an image past the bound.
pub(crate) fn translated(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    limits: &Limits,
) -> Result<(Forest, Image)> {
    let most = Most::of(limits);
    let image = image(options, sequent, most)?;
    let sequent = admit(image.sequent().clone(), most)?;
    Ok((Forest::from_owned(sequent, &bound(most))?, image))
}

/// Refuses `--translation` and `--linear`, which name the embedding,
/// beside a choice of LJT.
///
/// # Errors
///
/// Either flag beside `--method ljt`, or beside a settings file's
/// `ordinary.method` of `ljt`.
pub(crate) fn method_given(args: &ProveArgs, options: &ordinary::Options) -> Result<()> {
    if options.method == Some(Method::Ljt) && (args.logic.translation.is_some() || args.linear) {
        bail!(
            "--translation and --linear name the embedding, and LJT decides without one: leave \
             them out, or give --method embedding"
        );
    }
    Ok(())
}

/// Decides an ordinary sequent by LJT and writes what `prove` writes for
/// it: the verdict line, the checked derivation of LJ, its lambda-term
/// and the statistics, or the outcome as JSON. Returns `None`, having
/// written nothing, where the logic's default method gives way to the
/// embedding: LJT ended at a bound other than the time limit or Ctrl-C,
/// or a bound refused the derivation of the proof it found.
///
/// # Errors
///
/// `--net`, which draws a linear proof; a bound that refused the
/// derivation of a proof under `--method ljt`; an output that cannot be
/// written.
pub(crate) fn by_ljt(
    args: &ProveArgs,
    settings: &Settings,
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    deadline: &Deadline,
) -> Result<Option<Status>> {
    if args.output.net {
        bail!(
            "--net draws the proof net of a linear proof, and LJT makes none: add --method embedding"
        );
    }
    let limits = settings.limits;
    let format = args.output.format();
    let show = Show::new(&args.output, settings)?;
    // The logic's default, which the embedding takes over from.
    let gives_way = options.method.is_none();
    catch_interrupt();
    on_large_stack(limits.stack_bytes(), || {
        let start = Instant::now();
        let notice = Notice::start(NOTICE_AFTER, notice_line(deadline.limit(), false));
        let halt = || interrupted() || deadline.passed();
        let decided = ljt::decide(sequent, options.logic, &limits, |_| halt());
        drop(notice);
        let ended = Ended {
            stop: stopped(deadline),
            elapsed: start.elapsed(),
        };
        if gives_way && ordinary::gives_way(&decided) {
            return Ok(None);
        }
        let outcome = match decided {
            Ok(outcome) => ordinary::Outcome::from(outcome),
            Err(Error::Refused(Refusal::Stopped { .. })) => {
                let line = format!(
                    "unknown ({} logic by LJT): a proof was found, but {} before its derivation \
                     was checked",
                    options.logic,
                    Stop::why(ended.stop)
                );
                match (format, &args.output.output) {
                    (Format::Text, None) => io::write(None, &format!("{line}\n"))?,
                    _ => eprintln!("{line}"),
                }
                return Ok(Some(Status::Unknown));
            }
            Err(error @ Error::Refused(_)) => bail!(
                "LJT gave no verdict: {error}; raise the limit (--derivation-limit, \
                 --memory-limit, --work-limit), or decide through the embedding with --method \
                 embedding"
            ),
            Err(error) => return Err(error.into()),
        };
        let status = match &outcome.verdict {
            ordinary::Verdict::Valid(_) => Status::Yes,
            ordinary::Verdict::NotValid => Status::No,
            _ => Status::Unknown,
        };
        let over = || interrupted() || deadline.passed() || crate::io::closed();
        let mut out = io::Output::open(args.output.output.as_deref(), format.is_binary())?;
        if format == Format::Json {
            match &outcome.verdict {
                ordinary::Verdict::Valid(evidence) if show.lambda.is_some() => {
                    let term = evidence
                        .derivation()
                        .and_then(|d| d.term(&show.limits, |_| over()));
                    let mut value = serde_json::to_value(&outcome)?;
                    if let Some(object) = value.as_object_mut() {
                        object.insert("lambda".into(), lambda_json(term, &show, over)?);
                    }
                    serde_json::to_writer(out.stream(), &value)?;
                }
                _ => serde_json::to_writer(out.stream(), &outcome)?,
            }
            out.finish()?;
            return Ok(Some(status));
        }
        let line = show.verdict.then(|| ljt_verdict_line(&outcome, &ended));
        let prefix = show.open(&mut out, line.as_deref())?;
        let why = || Stop::why(stopped(deadline));
        let shown = match (&outcome.verdict, args.output.quiet) {
            (ordinary::Verdict::Valid(_), false) if show.tree == Tree::Never => Shown::Nothing,
            (ordinary::Verdict::Valid(_), false) if over() => {
                Shown::LeftOut(format!("the derivation is not written: {}", why()))
            }
            (ordinary::Verdict::Valid(evidence), false) => match evidence.derivation() {
                Ok(d) => crate::show::shown(
                    shown(d.clone(), &show),
                    show.fit(),
                    &show,
                    every_steps(&over),
                    why,
                    prefix.as_deref(),
                    &mut out,
                )?,
                // A clausal proof, which the verdict line names.
                Err(error) => Shown::LeftOut(error.to_string()),
            },
            _ => Shown::Nothing,
        };
        let drawn = matches!(shown, Shown::Written | Shown::Rendered(_));
        let keep = show.close(&mut out, shown, line.as_deref())?;
        if let (ordinary::Verdict::Valid(evidence), Format::Text, Some(_)) =
            (&outcome.verdict, format, show.lambda)
        {
            let term = evidence
                .derivation()
                .and_then(|d| d.term(&show.limits, |_| over()));
            show.after(&mut out, &lambda_line(term, &show, over), drawn)?;
        }
        if args.stats {
            show.after(&mut out, &ljt_statistics(&outcome, ended.elapsed), drawn)?;
        }
        if keep {
            out.finish()?;
        }
        anyhow::Ok(Some(status))
    })?
}

/// Returns the verdict line of an ordinary sequent decided by LJT or the
/// SAT solver: the verdict, the logic and the method, for a sequent valid
/// by a clausal proof that proof, and for an undecided sequent why.
pub(crate) fn ljt_verdict_line(outcome: &ordinary::Outcome, ended: &Ended) -> String {
    let method = match outcome.method() {
        Method::Sat => "the SAT solver",
        _ => "LJT",
    };
    let context = format!("{} logic by {method}", outcome.logic);
    match &outcome.verdict {
        ordinary::Verdict::Valid(ordinary::Evidence::Clausal(proof)) => {
            format!("valid ({context}): {proof}")
        }
        ordinary::Verdict::Valid(_) => format!("valid ({context})"),
        ordinary::Verdict::NotValid => {
            format!("not valid ({context}): the search was exhaustive")
        }
        ordinary::Verdict::Unknown(reason) => {
            format!("unknown ({context}): {}", advice(*reason, "", ended))
        }
        _ => format!("unknown ({context})"),
    }
}

/// Returns the lines `--stats` prints for LJT: its counters, a line each,
/// and the time.
pub(crate) fn ljt_statistics(outcome: &ordinary::Outcome, elapsed: Duration) -> String {
    let mut lines = String::new();
    if let ordinary::Search::Ljt(s) = &outcome.search {
        for (label, value) in [
            ("sequents expanded", s.sequents),
            ("memo hits", s.memo_hits),
            ("memo entries", s.memo_entries),
            ("refuted by an earlier failure", s.subsumed),
        ] {
            writeln!(lines, "{label}: {value}").expect("a string takes it");
        }
    }
    write!(lines, "time: {elapsed:.2?}").expect("a string takes it");
    lines
}

/// Returns the first line of the text output for an ordinary sequent
/// decided in `logic` through its image under `translation`: the verdict,
/// the logic and the translation, the engine, and for an undecided
/// sequent why.
pub(crate) fn verdict_line(
    outcome: &Outcome,
    logic: Logic,
    translation: Translation,
    ended: &Ended,
) -> String {
    let context = format!(
        "{logic} logic by the {translation} translation into {}, {} engine",
        translation.target(),
        outcome.engine
    );
    match &outcome.verdict {
        Verdict::Proved(_) => format!("valid ({context})"),
        Verdict::Unprovable(refutation) => {
            format!("not valid ({context}): the translated sequent is unprovable: {refutation}")
        }
        Verdict::Unknown(reason) => {
            format!("unknown ({context}): {}", unknown(*reason, outcome, ended))
        }
    }
}

/// Makes the derivation of LK or LJ a proof of the image reads back as
/// and writes it in the format `show` names, as
/// [`derivation`](crate::show::derivation) does for a linear one: within
/// the view's bounds, the time limit and Ctrl-C, fitted to a terminal.
/// The read-back is checked before it is written; one that fails the
/// check is an error, a defect to report.
pub(crate) fn derivation(
    image: &Image,
    proof: &Proof,
    show: &Show,
    mut halt: impl FnMut() -> bool,
    why: impl Fn() -> String,
    prefix: Option<&str>,
    out: &mut impl Write,
) -> Result<Shown> {
    if show.tree == Tree::Never {
        return Ok(Shown::Nothing);
    }
    let stopped = || Shown::LeftOut(format!("the derivation is not written: {}", why()));
    let d = match image.read_back(proof, &show.limits, |_| halt()) {
        Ok(d) => d,
        // The stop, whether the unfolding or the checker's pass in it
        // noticed it first.
        Err(error) if error.kind() == ErrorKind::Stopped => return Ok(stopped()),
        Err(error) if error.is_refusal() => {
            let sides = if image.mode().is_intuitionistic() {
                Sides::Two
            } else {
                Sides::One
            };
            let view = ViewOptions::default().with_sides(sides);
            let size = || {
                proof
                    .derivation_size(&view, &Limits::default(), |_| false)
                    .ok()
            };
            return Ok(Shown::LeftOut(not_built(&error, size)));
        }
        Err(error @ Error::ReadBack { .. }) => return Err(error.into()),
        Err(error) => return Err(anyhow!(error).context("the proof cannot be unfolded")),
    };
    match d.check(&show.limits, |_| halt()) {
        Err(Error::Refused(Refusal::Stopped { .. })) => return Ok(stopped()),
        checked => checked?,
    }
    crate::show::shown(shown(d, show), show.fit(), show, halt, why, prefix, out)
}

/// Returns a checked derivation of LK or LJ with its structural rules as
/// the view shows them, every one explicit in a certificate, which names
/// each formula it weakens.
pub(crate) fn shown(d: Derivation, show: &Show) -> Derivation {
    match (show.format, show.view.structural) {
        (Format::Rocq, _) | (_, Structural::Explicit) => d,
        (_, structural) => d.shown(structural).into_owned(),
    }
}

/// Returns the lambda-term of the LJ derivation read back from the proof
/// of an image, within the show's limits, or why it has none (a
/// derivation of LK).
pub(crate) fn term(
    image: &Image,
    proof: &deduxit::linear::Proof,
    show: &Show,
    halt: &dyn Fn() -> bool,
) -> Result<deduxit::lambda::Term, deduxit::Error> {
    let derivation = image.read_back(proof, &show.limits, |_| halt())?;
    derivation.term(&show.limits, |_| halt())
}

#[cfg(test)]
mod tests {
    use super::derivation;
    use crate::argument_parsing::{Cli, Command};
    use crate::settings::Given;
    use crate::show::{Show, Shown};
    use clap::Parser;
    use deduxit::linear::search::{Options, Verdict};
    use deduxit::ordinary::{self, Logic, Translation};

    /// A read-back the stop ends, in the unfolding or in the checker's
    /// pass inside it, is left out with the stop's reason.
    #[test]
    fn a_stopped_read_back_says_why() {
        let line = ["deduxit", "check", "--format", "latex", "--output", "x.tex"];
        let Command::Check(check) = Cli::try_parse_from(line).unwrap().command else {
            unreachable!("a check command")
        };
        let settings = check.settings(&Given::default()).unwrap();
        let show = Show::new(&check.output, &settings).unwrap();
        // Past the nodes the checker passes between two polls of its stop.
        let atoms: Vec<String> = (0..300).map(|i| format!("a{i}")).collect();
        let text = format!("{} |- {}", atoms.join(", "), atoms.join(" /\\ "));
        let sequent: ordinary::Sequent = text.parse().unwrap();
        let image = ordinary::translate(
            &sequent,
            Logic::Classical,
            Translation::Affine,
            &deduxit::Limits::default(),
        )
        .unwrap();
        let outcome =
            deduxit::linear::prove(image.sequent(), image.mode(), &Options::default()).unwrap();
        let Verdict::Proved(proof) = outcome.verdict else {
            panic!("the image is provable")
        };
        let mut out = String::new();
        let shown = derivation(
            &image,
            &proof,
            &show,
            || true,
            || "the time limit of 1s was reached".to_owned(),
            None,
            &mut out,
        )
        .unwrap();
        let Shown::LeftOut(line) = shown else {
            panic!("the derivation is left out")
        };
        assert_eq!(
            line,
            "the derivation is not written: the time limit of 1s was reached"
        );
    }

    /// A sequent valid by a clausal proof says so on its verdict line, by
    /// the SAT solver, and has no derivation to draw: the refusal names
    /// the method that gives one.
    #[test]
    fn a_clausal_proof_is_named_on_the_verdict_line() {
        let sequent: ordinary::Sequent = "|- ((a -> b) -> a) -> a".parse().unwrap();
        let outcome = ordinary::sat::decide(
            &sequent,
            Logic::Classical,
            &deduxit::Limits::default(),
            |_| false,
        )
        .unwrap();
        let outcome = ordinary::Outcome::from(outcome);
        let ended = super::Ended {
            stop: None,
            elapsed: std::time::Duration::ZERO,
        };
        let line = super::ljt_verdict_line(&outcome, &ended);
        assert!(
            line.starts_with(
                "valid (classical logic by the SAT solver): a checked clausal proof of "
            ),
            "{line}"
        );
        let ordinary::Verdict::Valid(evidence) = &outcome.verdict else {
            panic!("valid")
        };
        let refused = evidence.derivation().unwrap_err().to_string();
        assert!(refused.contains("the method embedding"), "{refused}");
    }
}
