// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Modal logic under `--system`: `prove` decides a modal sequent in a
//! system, and `check` checks a modal outcome, or a Kripke model given
//! against a modal sequent.

use crate::argument_parsing::{Format, ProveArgs, Tree};
use crate::check::{CHECKED_LEVEL, Checked};
use crate::io::{self, Most, bound, refused};
use crate::limit::{Deadline, Notice};
use crate::prove::{Ended, NOTICE_AFTER, Stop, advice, notice_line, on_large_stack, stopped};
use crate::show::{Prefixed, Rendered, Show, Shown, render};
use crate::{Status, catch_interrupt, interrupted};
use anyhow::{Result, bail};
use deduxit::export::svg;
use deduxit::kripke::Model;
use deduxit::limits::Refusal;
use deduxit::modal::{self, Countermodel, Evidence, Outcome, Sequent, System, Verdict};
use deduxit::{Error, Settings};
use std::fmt::Write as _;
use std::io::Write as _;
use std::time::Instant;

/// Reads a modal sequent from text in the modal syntax, or from its JSON
/// form, within `most`.
///
/// # Errors
///
/// Text that is no modal sequent, a format modal logic does not read, or
/// a sequent past the limits.
pub(crate) fn sequent_in(
    text: &str,
    format: crate::argument_parsing::InputFormat,
    most: Most,
) -> Result<Sequent> {
    use crate::argument_parsing::InputFormat;
    let within = bound(most);
    match format {
        InputFormat::Text | InputFormat::Auto => {
            Sequent::parse_within(text, &within).map_err(|e| match e {
                Error::Parse(_) => crate::parse_error(text, e),
                e => refused(e, most),
            })
        }
        InputFormat::Json => {
            let mut document = serde_json::Deserializer::from_str(text);
            let sequent = deduxit::wire::upgrade::<Sequent, _>(&mut document, &within)
                .map_err(|e| refused(e, most))?;
            document
                .end()
                .map_err(|_| anyhow::anyhow!("not a modal sequent in JSON: more follows it"))?;
            Ok(sequent)
        }
        _ => bail!("--system reads one sequent as text or as JSON"),
    }
}

/// Refuses the flags of the other suites beside `--system`.
///
/// # Errors
///
/// A mode flag, `--method`, `--translation`, `--linear`, `--net` or
/// `--lambda` beside `--system`.
pub(crate) fn flags(args: &ProveArgs) -> Result<()> {
    if args.mode.given() {
        bail!("--system decides modal logic, which has no mode: leave out the mode flags");
    }
    if args.method.is_some() || args.linear || args.logic.translation.is_some() {
        bail!(
            "--method, --translation and --linear choose how ordinary logic is decided: leave \
             them out beside --system"
        );
    }
    if args.output.net || args.output.lambda {
        bail!(
            "a modal derivation has no proof net and no lambda-term: leave out --net and --lambda"
        );
    }
    Ok(())
}

/// Decides a modal sequent in `system` and writes the verdict line, then
/// the derivation's nodes or the countermodel's worlds, or the outcome as
/// JSON; returns the exit status.
///
/// # Errors
///
/// An input that cannot be read, a bound that refuses the evidence, or an
/// output that cannot be written.
pub(crate) fn prove(
    args: &ProveArgs,
    settings: &Settings,
    system: System,
    deadline: &Deadline,
) -> Result<Status> {
    flags(args)?;
    let limits = settings.limits;
    let input = args.input.clone();
    let read = deadline.within(move || input.modal(&limits))?;
    let Some(sequent) = read else {
        let limit = deadline.limit().expect("only a limit passes");
        return crate::prove::unread_into(args, &crate::prove::unread(limit));
    };
    let sequent = sequent?;
    let format = args.output.format();
    crate::prove::output_can_be_made(args, format)?;
    let show = Show::new(&args.output, settings)?;
    catch_interrupt();
    let options = modal::Options::default().with_system(system);
    let start = Instant::now();
    let decided = on_large_stack(limits.stack_bytes(), || {
        let notice = Notice::start(NOTICE_AFTER, notice_line(deadline.limit(), false));
        let halt = || interrupted() || deadline.passed();
        let decided = modal::decide_within(&sequent, &options, &limits, |_| halt());
        drop(notice);
        decided
    })?;
    let ended = Ended {
        stop: stopped(deadline),
        elapsed: start.elapsed(),
    };
    let outcome = match decided {
        Ok(outcome) => outcome,
        Err(Error::Refused(Refusal::Stopped { .. })) => {
            let line = format!(
                "unknown in {system}: the search ended, but {} before its answer was checked",
                Stop::why(ended.stop)
            );
            match (format, &args.output.output) {
                (Format::Text, None) => io::write(None, &format!("{line}\n"))?,
                _ => eprintln!("{line}"),
            }
            return Ok(Status::Unknown);
        }
        Err(error @ Error::Refused(_)) => bail!(
            "the evidence of the verdict was refused: {error}; raise the limit \
             (--derivation-limit, --memory-limit, --work-limit)"
        ),
        Err(error) => return Err(error.into()),
    };
    let status = match outcome.verdict {
        Verdict::Valid(_) => Status::Yes,
        Verdict::NotValid => Status::No,
        _ => Status::Unknown,
    };
    let mut out = io::Output::open(args.output.output.as_deref(), format.is_binary())?;
    if format == Format::Json {
        serde_json::to_writer(out.stream(), &outcome)?;
        out.finish()?;
        return Ok(status);
    }
    let line = verdict_line(&outcome, &ended);
    let quiet = args.output.quiet;
    let prefix = show.open(&mut out, show.verdict.then_some(line.as_str()))?;
    let halt = || interrupted() || deadline.passed();
    let shown = if quiet || show.tree == Tree::Never {
        Shown::Nothing
    } else {
        evidence(&outcome, &show, prefix.as_deref(), &mut out, &halt)?
    };
    let drawn = matches!(shown, Shown::Written | Shown::Rendered(_));
    let keep = show.close(&mut out, shown, show.verdict.then_some(line.as_str()))?;
    if args.stats {
        let s = outcome.statistics;
        let lines = format!(
            "worlds made: {}\nsplits: {}\nworlds blocked: {}\nunits of work: {}\ninferences: \
             {}\ntime: {:.2?}",
            s.worlds, s.splits, s.blocked, s.work, s.inferences, ended.elapsed
        );
        show.after(&mut out, &lines, drawn)?;
    }
    if keep {
        out.finish()?;
    }
    Ok(status)
}

/// Returns the verdict line of a modal outcome.
fn verdict_line(outcome: &Outcome, ended: &Ended) -> String {
    let system = outcome.system;
    let sequent = &outcome.sequent;
    match &outcome.verdict {
        Verdict::Valid(Evidence::Derivation(derivation)) => format!(
            "valid in {system}: {sequent}, a checked derivation of {} in {}",
            counted(derivation.len(), "inference", "inferences"),
            derivation.calculus()
        ),
        Verdict::Valid(_) => format!("valid in {system}: {sequent}"),
        Verdict::NotValid => match &outcome.countermodel {
            Some(c) => format!(
                "not valid in {system}: a checked countermodel of {} refutes {sequent} at \
                 world {}",
                counted(c.model.worlds.len(), "world", "worlds"),
                c.model.world
            ),
            None => format!(
                "not valid in {system}: {sequent}; the countermodel is too large to check, so \
                 it is not shown"
            ),
        },
        Verdict::Unknown(reason) => {
            format!("unknown in {system}: {}", advice(*reason, "", ended))
        }
        _ => format!("unknown in {system}"),
    }
}

/// Writes the evidence of an outcome as `show` asks: the derivation (its
/// nodes as text, drawn in LaTeX, Typst, SVG, PNG or PDF), or the
/// countermodel (its worlds as text, drawn in SVG, PNG or PDF).
fn evidence(
    outcome: &Outcome,
    show: &Show,
    prefix: Option<&str>,
    out: &mut io::Output,
    halt: &dyn Fn() -> bool,
) -> Result<Shown> {
    match (&outcome.verdict, &outcome.countermodel) {
        (Verdict::Valid(Evidence::Derivation(derivation)), _) => {
            let calculus = modal::calculus(outcome.system, &show.limits)?;
            crate::calculus::shown(derivation, &calculus, show, prefix, out, halt)
        }
        (Verdict::NotValid, Some(countermodel)) => {
            countermodel_shown(countermodel, show, prefix, out, halt)
        }
        _ => Ok(Shown::Nothing),
    }
}

/// Writes a countermodel as `show` asks: its worlds as text, or drawn in
/// SVG, PNG or PDF; LaTeX, Typst and Rocq leave it out with the reason.
fn countermodel_shown(
    countermodel: &Countermodel,
    show: &Show,
    prefix: Option<&str>,
    out: &mut io::Output,
    halt: &dyn Fn() -> bool,
) -> Result<Shown> {
    match show.format {
        Format::Text => {
            let mut writer = Prefixed { out, prefix };
            write!(writer, "{}", worlds_text(&countermodel.model, None))?;
            Ok(Shown::Written)
        }
        Format::Svg | Format::Png | Format::Pdf => {
            let drawing =
                match svg::modal_countermodel(countermodel, &show.styles.svg, &show.limits, |_| {
                    halt()
                }) {
                    Ok(drawing) => drawing,
                    Err(error) => {
                        return Ok(Shown::LeftOut(format!(
                            "the countermodel is not drawn: {error}"
                        )));
                    }
                };
            if show.format == Format::Svg {
                write!(out.stream(), "{}{drawing}", prefix.unwrap_or(""))?;
                return Ok(Shown::Written);
            }
            Ok(
                match render(
                    drawing,
                    show.format,
                    &show.styles,
                    show.limits.memory_bytes,
                    halt,
                )? {
                    Rendered::Bytes(bytes) => Shown::Rendered(bytes),
                    Rendered::Refused(line) => Shown::LeftOut(line),
                    Rendered::Stopped => {
                        Shown::LeftOut("the countermodel is not drawn: it was stopped".to_owned())
                    }
                },
            )
        }
        _ => Ok(Shown::LeftOut(
            "the countermodel is not drawn: LaTeX, Typst and Rocq have no drawing of a Kripke \
             model; --format svg, png or pdf draws it, and the text output lists its worlds"
                .to_owned(),
        )),
    }
}

/// Returns a modal model's worlds as lines: each world's number, what it
/// forces and the worlds it sees, `marked` (or the model's world) named.
fn worlds_text(model: &Model, marked: Option<u32>) -> String {
    let mut text = String::new();
    let marked = marked.unwrap_or(model.world);
    for (n, world) in model.worlds.iter().enumerate() {
        if n > 0 {
            text.push('\n');
        }
        write!(text, "world {n}").expect("a string takes any text");
        if n == marked as usize {
            text.push_str(" (where the sequent fails)");
        }
        if world.atoms.is_empty() {
            text.push_str(": forces nothing");
        } else {
            write!(text, ": forces {}", world.atoms.join(", ")).expect("a string takes any text");
        }
        let mut seen: Vec<u32> = world.arrows.clone();
        seen.sort_unstable();
        seen.dedup();
        if seen.is_empty() {
            text.push_str("; sees no world");
        } else {
            let seen: Vec<String> = seen.iter().map(u32::to_string).collect();
            write!(text, "; sees {}", seen.join(", ")).expect("a string takes any text");
        }
    }
    text
}

/// Checks a modal outcome read from a file: its evidence against its
/// sequent and system ([`Outcome::check`]), and writes the verdict, or the
/// verdict as JSON; returns the status (the verdict holds 0, it fails 1,
/// nothing to check or a bound 3) and whether the output is to be
/// finished.
///
/// # Errors
///
/// An output that cannot be written.
pub(crate) fn check_outcome(
    outcome: &Outcome,
    show: &Show,
    deadline: &Deadline,
    out: &mut io::Output,
) -> Result<(Status, bool)> {
    let over = || interrupted() || deadline.passed();
    let system = outcome.system;
    let sequent = &outcome.sequent;
    let evidence = !matches!(outcome.verdict, Verdict::Unknown(_))
        && (matches!(outcome.verdict, Verdict::Valid(_)) || outcome.countermodel.is_some());
    let result = outcome.check(&show.limits, |_| over());
    let (status, line) = match (&result, &outcome.verdict) {
        (Err(error), _) if error.is_refusal() => (
            Status::Unknown,
            format!("unknown: the outcome's evidence is not checked: {error}"),
        ),
        (Err(error), _) => (
            Status::No,
            format!("invalid: the outcome's evidence does not hold: {error}"),
        ),
        (Ok(()), _) if !evidence => (
            Status::Unknown,
            format!("unknown: the outcome of {sequent} in {system} holds no evidence to check"),
        ),
        (Ok(()), Verdict::Valid(_)) => (
            Status::Yes,
            format!("valid in {system}: {sequent}, its derivation checked"),
        ),
        (Ok(()), _) => (
            Status::Yes,
            format!("not valid in {system}: {sequent}, its countermodel checked"),
        ),
    };
    if show.format == Format::Json {
        let checked = Checked {
            version: CHECKED_LEVEL,
            verdict: match status {
                Status::Yes => "valid",
                Status::Unknown => "unknown",
                _ => "invalid",
            },
            mode: None,
            logic: None,
            calculus: None,
            system: Some(system),
            error: result.as_ref().err().cloned(),
            message: (status != Status::Yes).then_some(line),
            lambda: None,
            failing: None,
        };
        serde_json::to_writer(out.stream(), &checked)?;
        return Ok((status, true));
    }
    if show.format != Format::Text {
        bail!("a modal outcome is checked as text or json: give one of those to --format");
    }
    if show.verdict || status != Status::Yes {
        out.stream().write_all(line.as_bytes())?;
        return Ok((status, true));
    }
    Ok((status, false))
}

/// Checks a Kripke model given against a modal sequent in `system`: the
/// frame conditions first, then whether the sequent holds at `world` or
/// at every world; writes the verdict and, where it fails, the failing
/// world's values; returns the status (holds 0, fails 1) and whether the
/// output is to be finished.
///
/// # Errors
///
/// A model that is none of the system (exit 2, the condition named), a
/// sequent that cannot be read, or an output that cannot be written.
pub(crate) fn check_model(
    model: &Model,
    (sequent, system): (&str, System),
    world: Option<u32>,
    (settings, show): (&Settings, &Show),
    deadline: &Deadline,
    out: &mut io::Output,
) -> Result<(Status, bool)> {
    let limits = settings.limits;
    let sequent = sequent_in(
        sequent,
        crate::argument_parsing::InputFormat::Text,
        Most::of(&limits),
    )?;
    if let Some(w) = world {
        model.check_world(w)?;
    }
    let over = || interrupted() || deadline.passed();
    if let Some(frame) = model.frame_fault(system, &limits, |_| over())? {
        bail!("the model is no model of {system}: {frame}");
    }
    let truth = model.truth(&sequent, system, &limits, |_| over())?;
    let failing = match world {
        Some(w) => (!truth.sequent_holds(w)).then_some(w),
        None => truth.failing(),
    };
    let at = world.map_or_else(|| "at every world".to_owned(), |w| format!("at world {w}"));
    let line = match failing {
        None => format!("holds in {system} {at}: {sequent}"),
        Some(w) => format!(
            "fails in {system} at world {w}: {sequent}; every formula left of ⊢ is true there \
             and every formula right of it false"
        ),
    };
    let status = if failing.is_some() {
        Status::No
    } else {
        Status::Yes
    };
    if show.format == Format::Json {
        let failing = failing.map(|w| {
            let formulas = sequent.formulas();
            let forcing: Vec<serde_json::Value> = truth
                .at(w)
                .map(|(id, value)| {
                    serde_json::json!({"formula": formulas.formula(id).to_string(), "forced": value})
                })
                .collect();
            serde_json::json!({"world": w, "forcing": forcing})
        });
        let checked = Checked {
            version: CHECKED_LEVEL,
            verdict: if failing.is_some() {
                "invalid"
            } else {
                "valid"
            },
            mode: None,
            logic: None,
            calculus: None,
            system: Some(system),
            error: None,
            message: Some(line),
            lambda: None,
            failing,
        };
        serde_json::to_writer(out.stream(), &checked)?;
        return Ok((status, true));
    }
    if show.format != Format::Text {
        bail!("a model is checked as text or json: give one of those to --format");
    }
    let mut text = line;
    if let Some(w) = failing {
        let formulas = sequent.formulas();
        for (id, value) in truth.at(w) {
            let node = formulas.node(id);
            if matches!(
                node,
                modal::Node::Atom(_) | modal::Node::True | modal::Node::False
            ) {
                continue;
            }
            write!(
                text,
                "\n  {} is {}",
                formulas.formula(id),
                if value { "true" } else { "false" }
            )
            .expect("a string takes any text");
            if let Some(v) = truth.witness(&sequent, w, id) {
                write!(text, ": world {v} shows it").expect("a string takes any text");
            }
        }
        text.push('\n');
        text.push_str(&worlds_text(model, Some(w)));
    }
    out.stream().write_all(text.as_bytes())?;
    Ok((status, true))
}

/// Returns `n` with the noun that counts it.
fn counted(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}
