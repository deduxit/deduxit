// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `resolve`: a set of clauses refuted by resolution or DPLL, every
//! refutation checked; and `check --dimacs` of an outside solver's proof
//! and `check` of a refutation in JSON.

use crate::argument_parsing::{
    AnswerFormat, CheckArgs, ClausesArg, ProofFormatArg, ResolutionArg, ResolveArgs,
};
use crate::limit::Deadline;
use crate::prove::{Stop, on_large_stack, stopped};
use crate::settings::{Given, time_limit};
use crate::{Status, catch_interrupt, interrupted, io};
use anyhow::{Context, Result, bail};
use deduxit::linear::search::Jobs;
use deduxit::ordinary::normal::{self, Form};
use deduxit::ordinary::resolution::{
    self, Clauses, Event, Method, Options, ProofFormat, Refutation, Verdict,
};
use deduxit::{Error, Refusal, Settings};
use std::fmt::Write;
use std::io::Read;
use std::path::Path;
use std::time::Instant;

/// Runs `resolve`: reads the clauses, from DIMACS or a sequent's negation,
/// decides them, and prints the clauses and the refutation or the model.
/// Returns the exit status: yes for unsatisfiable clauses, no for
/// satisfiable ones.
///
/// # Errors
///
/// An input that cannot be read, a rewriting into clauses or a search
/// whose answer fails its check (a defect), an output that cannot be
/// written.
pub(crate) fn resolve(args: &ResolveArgs, given: &Given) -> Result<Status> {
    catch_interrupt();
    let mut settings = args.common.settings(given)?;
    if settings.search.jobs == Jobs::Auto {
        settings.search = settings.search.with_jobs(1);
    }
    let limits = settings.limits;
    let deadline = Deadline::start(time_limit(&settings), Instant::now())?;
    let stop = |_| interrupted() || deadline.passed();
    // The clauses, and for a sequent the form that made them.
    let (clauses, made) = match &args.dimacs {
        Some(path) => {
            let text = io::read(Some(path), "DIMACS file", limits.memory_bytes)?;
            (Clauses::from_dimacs(&text, &limits)?, None)
        }
        None => {
            let sequent = args.common.input.ordinary(&limits)?;
            if sequent.formulas().is_first_order() {
                return first_order(args, &sequent, &limits, &deadline);
            }
            let form = match args.clauses {
                ClausesArg::Cnf => Form::Cnf,
                ClausesArg::Tseitin => Form::Tseitin,
                ClausesArg::PlaistedGreenbaum => Form::PlaistedGreenbaum,
            };
            let found = on_large_stack(limits.stack_bytes(), || {
                resolution::clauses_of(&sequent, form, &settings.search, &limits, stop)
            })?;
            match found {
                Ok((clauses, made)) => (clauses, Some(made)),
                Err(Error::Refused(Refusal::Stopped { .. })) => {
                    return unknown(args, &Stop::why(stopped(&deadline)));
                }
                Err(error @ Error::Refused(Refusal::Form { .. })) => {
                    bail!(
                        "{error}; raise limits.form_occurrences in a --settings file, or take --clauses tseitin"
                    )
                }
                Err(error) => return Err(error.into()),
            }
        }
    };
    let options = Options::default()
        .with_method(match args.method {
            ResolutionArg::Saturation => Method::Saturation,
            ResolutionArg::Dpll => Method::Dpll,
        })
        .with_clauses(args.clause_limit.0)
        .with_steps(args.step_limit.0);
    let answer = on_large_stack(limits.stack_bytes(), || {
        resolution::decide(&clauses, &options, &limits, stop)
    })?;
    let answer = match answer {
        Ok(answer) => answer,
        Err(Error::Refused(Refusal::Stopped { .. })) => {
            return unknown(args, &Stop::why(stopped(&deadline)));
        }
        Err(Error::Refused(refusal)) => {
            let flag = match refusal.setting() {
                Some("limits.memory_bytes") => "; raise it with --memory-limit",
                Some("limits.work") => "; raise it with --work-limit",
                _ => "",
            };
            return unknown(args, &format!("{refusal}{flag}"));
        }
        Err(error) => return Err(error.into()),
    };
    let status = match answer.verdict {
        Verdict::Refuted(_) => Status::Yes,
        Verdict::Satisfiable { .. } => Status::No,
        _ => Status::Unknown,
    };
    let text = match args.common.format {
        AnswerFormat::Json => serde_json::to_string(&answer)?,
        AnswerFormat::Text => answer_text(&answer, made.as_ref(), &deadline)?,
    };
    io::write(args.common.output.as_deref(), &text)?;
    Ok(status)
}

/// Decides a first-order sequent's clauses by first-order resolution and
/// prints its answer. Returns the exit status: yes for a refutation,
/// unknown otherwise (no model is built).
///
/// # Errors
///
/// `--method dpll` or `--clauses` other than cnf, which resolution at
/// first order does not take; the clausal form's refusals; a refutation
/// that fails its check (a defect); an output that cannot be written.
fn first_order(
    args: &ResolveArgs,
    sequent: &deduxit::ordinary::Sequent,
    limits: &deduxit::Limits,
    deadline: &Deadline,
) -> Result<Status> {
    use deduxit::ordinary::resolution::first_order::{self, Answer};
    if args.method == ResolutionArg::Dpll {
        bail!(
            "--method dpll decides propositional clauses; a first-order sequent takes saturation"
        );
    }
    if args.clauses != ClausesArg::Cnf {
        bail!("--clauses names a propositional form; a first-order sequent takes its clausal form");
    }
    let equality = match args.equality {
        crate::argument_parsing::EqualityArg::Paramodulation => {
            deduxit::ordinary::resolution::Equality::Paramodulation
        }
        crate::argument_parsing::EqualityArg::Axioms => {
            deduxit::ordinary::resolution::Equality::Axioms
        }
    };
    let options = Options::default()
        .with_clauses(args.clause_limit.0)
        .with_steps(args.step_limit.0)
        .with_equality(equality);
    let stop = |_| interrupted() || deadline.passed();
    let found = on_large_stack(limits.stack_bytes(), || {
        first_order::resolve(sequent, &options, limits, stop)
    })?;
    let found = match found {
        Ok(found) => found,
        Err(Error::Refused(Refusal::Stopped { .. })) => {
            return unknown(args, &Stop::why(stopped(deadline)));
        }
        Err(error @ Error::Refused(Refusal::Form { .. })) => {
            bail!("{error}; raise limits.form_occurrences in a --settings file")
        }
        Err(Error::Refused(refusal)) => {
            let flag = match refusal.setting() {
                Some("limits.memory_bytes") => "; raise it with --memory-limit",
                Some("limits.work") => "; raise it with --work-limit",
                _ => "",
            };
            return unknown(args, &format!("{refusal}{flag}"));
        }
        Err(error) => return Err(error.into()),
    };
    let status = match found.answer {
        Answer::Refuted { .. } => Status::Yes,
        _ => Status::Unknown,
    };
    let text = match args.common.format {
        AnswerFormat::Json => serde_json::to_string(&found)?,
        AnswerFormat::Text => first_order_text(&found, deadline)?,
    };
    io::write(args.common.output.as_deref(), &text)?;
    Ok(status)
}

/// Returns the text of a first-order sequent's clauses decided: the
/// verdict, the clauses numbered from 1, and the refutation's inferences,
/// each with its parents, the predicate it resolves on and its unifier.
///
/// # Errors
///
/// None in practice: writing into a string does not fail.
fn first_order_text(
    found: &deduxit::ordinary::resolution::first_order::FirstOrder,
    deadline: &Deadline,
) -> Result<String> {
    use deduxit::ordinary::resolution::first_order::{Answer, Step};
    let mut text = String::new();
    match &found.answer {
        Answer::Refuted {
            refutation,
            derivation,
            not_read_back,
        } => {
            let n = refutation.inferences().len();
            let steps = format!("a checked refutation of {n} inference{}", plural(n));
            let back = match (derivation, not_read_back) {
                (Some(d), _) => {
                    let k = d.inferences().len();
                    format!(
                        ", read back as a checked derivation of LK ({k} inference{})",
                        plural(k)
                    )
                }
                (None, Some(why)) => format!(
                    "; no derivation of LK was read back ({why}), so the verdict rests on the \
                     refutation and the clausal form's Skolem symbols"
                ),
                (None, None) => String::new(),
            };
            writeln!(
                text,
                "valid in classical logic: the clauses of its negation are unsatisfiable, {steps}{back}"
            )?;
        }
        Answer::Saturated => writeln!(
            text,
            "unknown: every inference was made and none gave □, so the clauses have a model, \
             which resolution does not build; prove --logic classical refutes the sequent where it \
             can"
        )?,
        Answer::Unknown(bound) => {
            let why = match bound {
                resolution::Bound::Stopped => Stop::why(stopped(deadline)),
                resolution::Bound::Clauses { .. } => {
                    format!("{bound}; raise it with --clause-limit")
                }
                resolution::Bound::Steps { .. } => format!("{bound}; raise it with --step-limit"),
                _ => bound.to_string(),
            };
            writeln!(text, "unknown: {why}")?;
        }
        _ => writeln!(text, "decided")?,
    }
    writeln!(
        text,
        "clauses of its negation by clausal normal form, each clause's variables X1, X2, …:"
    )?;
    let given = found.clauses.clauses().len();
    let made = match &found.answer {
        Answer::Refuted { refutation, .. } => refutation.inferences().len(),
        _ => 0,
    };
    let width = (given + made).to_string().len();
    for k in 0..given {
        writeln!(text, "{:>width$}. {}", k + 1, found.clauses.text(k))?;
    }
    if let Answer::Refuted { refutation, .. } = &found.answer {
        writeln!(
            text,
            "resolution, the first parent's variables X1, …, the second's Y1, …:"
        )?;
        let set = refutation.clauses();
        for inference in refutation.inferences() {
            let from = match inference.step {
                Step::Resolution { first, second, .. } => {
                    format!("from {}, {}", first + 1, second + 1)
                }
                Step::Factor { clause, .. } => format!("factor of {}", clause + 1),
                Step::Paramodulation { from, into, .. } => {
                    // The equation's clause first: its variables are X1, ….
                    format!("{} rewrites {}", from + 1, into + 1)
                }
                _ => String::new(),
            };
            let unifier = set.bindings_text(inference);
            let with = if unifier.is_empty() {
                String::new()
            } else {
                format!(" with {}", unifier.join(", "))
            };
            writeln!(
                text,
                "{:>width$}. {}  {from}{with}",
                inference.clause + 1,
                set.text(inference.clause)
            )?;
        }
    }
    Ok(text.trim_end().to_owned())
}

/// Writes the line of an answer a stop ended, as `transform` does, and
/// returns unknown.
///
/// # Errors
///
/// An output that cannot be written.
fn unknown(args: &ResolveArgs, why: &str) -> Result<Status> {
    let line = format!("unknown: {why}");
    match args.common.format {
        AnswerFormat::Text => io::write(args.common.output.as_deref(), &line)?,
        AnswerFormat::Json => eprintln!("{line}"),
    }
    Ok(Status::Unknown)
}

/// Returns the text of a decided clause set: the verdict, how a sequent's
/// clauses were made, the clauses numbered from 1, DPLL's steps, and the
/// refutation or the model.
///
/// # Errors
///
/// None in practice: writing into a string does not fail.
fn answer_text(
    answer: &resolution::Resolution,
    made: Option<&normal::NormalForm>,
    deadline: &Deadline,
) -> Result<String> {
    let set = &answer.clauses;
    let mut text = String::new();
    let of_sequent = made.is_some();
    match &answer.verdict {
        Verdict::Refuted(r) => {
            let n = r.resolvents().len();
            let steps = format!("a checked refutation of {n} resolution{}", plural(n));
            if of_sequent {
                writeln!(
                    text,
                    "valid in classical logic: the clauses of its negation are unsatisfiable, {steps}"
                )?;
            } else {
                writeln!(text, "unsatisfiable: {steps}")?;
            }
        }
        Verdict::Satisfiable { true_atoms } => {
            let atoms: Vec<&str> = true_atoms
                .iter()
                .map(|&a| set.atoms()[a as usize].as_str())
                .collect();
            let model = if atoms.is_empty() {
                "every atom false".to_owned()
            } else {
                format!("{} true, every other atom false", atoms.join(", "))
            };
            if of_sequent {
                writeln!(
                    text,
                    "not valid in classical logic: a checked model of the clauses of its negation, {model}"
                )?;
            } else {
                writeln!(text, "satisfiable: a checked model, {model}")?;
            }
        }
        Verdict::Unknown(bound) => {
            let why = match bound {
                resolution::Bound::Stopped => Stop::why(stopped(deadline)),
                resolution::Bound::Clauses { .. } => {
                    format!("{bound}; raise it with --clause-limit")
                }
                resolution::Bound::Steps { .. } => format!("{bound}; raise it with --step-limit"),
                _ => bound.to_string(),
            };
            writeln!(text, "unknown: {why}")?;
        }
        _ => writeln!(text, "decided")?,
    }
    if let Some(made) = made {
        let checked = if made.is_checked() {
            "checked"
        } else {
            "not checked within the limits"
        };
        writeln!(
            text,
            "clauses of its negation by {}, {checked}:",
            made.form.title()
        )?;
    } else {
        writeln!(text, "clauses:")?;
    }
    let width = (set.clauses().len() + resolvents(answer)).to_string().len();
    for (k, clause) in set.clauses().iter().enumerate() {
        writeln!(text, "{:>width$}. {}", k + 1, set.text(clause))?;
    }
    if !answer.trace.is_empty() {
        writeln!(text, "DPLL:")?;
        // The decisions open, by which a step is indented.
        let mut open = 0;
        for event in &answer.trace {
            if let Event::Decide { level, .. } | Event::Backtrack { level, .. } = *event {
                open = level;
            }
            writeln!(text, "{}", event_line(set, event, open))?;
        }
    }
    if let Verdict::Refuted(r) = &answer.verdict
        && !r.resolvents().is_empty()
    {
        writeln!(
            text,
            "{}:",
            if answer.trace.is_empty() {
                "resolution"
            } else {
                "resolution read off the steps"
            }
        )?;
        refutation_lines(&mut text, r, width)?;
    }
    Ok(text.trim_end().to_owned())
}

/// Returns how many resolvents an answer holds.
fn resolvents(answer: &resolution::Resolution) -> usize {
    match &answer.verdict {
        Verdict::Refuted(r) => r.resolvents().len(),
        _ => 0,
    }
}

/// Returns "s" for a count other than one.
const fn plural(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}

/// Writes a refutation's resolvents, numbered on from its clauses, from
/// 1, each with the clauses and the atom it comes from.
///
/// # Errors
///
/// None in practice: writing into a string does not fail.
fn refutation_lines(text: &mut String, r: &Refutation, width: usize) -> Result<()> {
    let set = r.clauses();
    let given = set.clauses().len();
    for (k, step) in r.resolvents().iter().enumerate() {
        writeln!(
            text,
            "{:>width$}. {}  from {}, {} on {}",
            given + k + 1,
            set.text(&step.clause),
            step.first + 1,
            step.second + 1,
            set.atoms()[step.atom as usize]
        )?;
    }
    Ok(())
}

/// Returns the line of a step of DPLL: a decision and a backtrack at the
/// depth of the decisions before them, every other step under the
/// `open` decisions.
fn event_line(set: &Clauses, event: &Event, open: u32) -> String {
    match *event {
        Event::Decide { literal, level } => {
            format!(
                "{}decide {} (level {level})",
                indent(level - 1),
                set.literal(literal)
            )
        }
        Event::Backtrack { literal, level } => format!(
            "{}backtrack: {} (level {level})",
            indent(level - 1),
            set.literal(literal)
        ),
        Event::Unit { literal, clause } => format!(
            "{}unit {} from clause {}",
            indent(open),
            set.literal(literal),
            clause + 1
        ),
        Event::Pure { literal } => format!("{}pure {}", indent(open), set.literal(literal)),
        Event::Conflict { clause } => {
            format!("{}conflict at clause {}", indent(open), clause + 1)
        }
        _ => "a step".to_owned(),
    }
}

/// Returns two spaces per level.
fn indent(level: u32) -> String {
    "  ".repeat(level as usize)
}

/// Runs `check --dimacs`: reads the clauses and an outside solver's
/// proof, and checks the proof. Returns yes for a proof that holds, no for
/// one that does not, unknown where a limit ended the check.
///
/// # Errors
///
/// A file that cannot be read, or is no DIMACS or no proof of the format.
pub(crate) fn check_outside(
    args: &CheckArgs,
    settings: &Settings,
    deadline: &Deadline,
) -> Result<Status> {
    let limits = settings.limits;
    let Some(dimacs) = &args.dimacs else {
        bail!("check --dimacs needs the clauses' file");
    };
    let clauses_text = io::read(Some(dimacs), "DIMACS file", limits.memory_bytes)?;
    let clauses = Clauses::from_dimacs(&clauses_text, &limits)
        .with_context(|| format!("cannot read {}", dimacs.display()))?;
    let source = args.source();
    let proof = bytes(source.as_deref(), limits.memory_bytes)?;
    let format = match args.proof_format {
        Some(ProofFormatArg::Lrat) => ProofFormat::Lrat,
        Some(ProofFormatArg::Drat) => ProofFormat::Drat,
        None if source
            .as_deref()
            .and_then(Path::extension)
            .is_some_and(|e| e == "lrat") =>
        {
            ProofFormat::Lrat
        }
        None => ProofFormat::Drat,
    };
    let checked = resolution::check_proof(&clauses, &proof, format, &limits, |_| {
        interrupted() || deadline.passed()
    });
    let (status, line) = match checked {
        Ok(proof) => (
            Status::Yes,
            format!(
                "valid: the proof refutes the {} clause{}, {} step{} checked",
                clauses.clauses().len(),
                plural(clauses.clauses().len()),
                proof.steps(),
                plural(usize::try_from(proof.steps()).unwrap_or(usize::MAX))
            ),
        ),
        Err(error @ (Error::InvalidRefutation { .. } | Error::ClauseFile { .. })) => {
            if let Error::ClauseFile { .. } = error {
                return Err(anyhow::Error::from(error).context("cannot read the proof"));
            }
            (Status::No, format!("invalid: {error}"))
        }
        Err(Error::Refused(refusal)) => (
            Status::Unknown,
            format!("unknown: the proof is not checked: {refusal}"),
        ),
        Err(error) => return Err(error.into()),
    };
    io::write(args.output.output.as_deref(), &line)?;
    Ok(status)
}

/// Checks a refutation read from JSON. Returns yes where it holds, no
/// where it does not, unknown where a limit ended the check.
///
/// # Errors
///
/// An output that cannot be written.
pub(crate) fn check_refutation(
    refutation: &Refutation,
    settings: &Settings,
    path: Option<&Path>,
    deadline: &Deadline,
) -> Result<Status> {
    let checked = refutation.check(&settings.limits, |_| interrupted() || deadline.passed());
    let n = refutation.resolvents().len();
    let (status, line) = match checked {
        Ok(()) => (
            Status::Yes,
            format!(
                "valid: a refutation of {} clauses by {n} resolution{}",
                refutation.clauses().clauses().len(),
                plural(n)
            ),
        ),
        Err(error @ Error::InvalidRefutation { .. }) => (Status::No, format!("invalid: {error}")),
        Err(Error::Refused(refusal)) => (
            Status::Unknown,
            format!("unknown: the refutation is not checked: {refusal}"),
        ),
        Err(error) => return Err(error.into()),
    };
    io::write(path, &line)?;
    Ok(status)
}

/// Reads a file, or standard input for `-` or none, as bytes within the
/// memory limit.
///
/// # Errors
///
/// A file that cannot be read, or one longer than the limit.
fn bytes(path: Option<&Path>, memory: Option<u64>) -> Result<Vec<u8>> {
    let most = memory.unwrap_or(u64::MAX);
    let mut out = Vec::new();
    let read = match path {
        Some(path) if path != Path::new("-") => std::fs::File::open(path)
            .with_context(|| format!("cannot read {}", path.display()))?
            .take(most.saturating_add(1))
            .read_to_end(&mut out),
        _ => std::io::stdin()
            .lock()
            .take(most.saturating_add(1))
            .read_to_end(&mut out),
    };
    read.context("cannot read the proof")?;
    if out.len() as u64 > most {
        bail!(
            "the proof is longer than the memory limit of {most} bytes; raise it with \
             --memory-limit SIZE"
        );
    }
    Ok(out)
}
