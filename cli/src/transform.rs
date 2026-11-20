// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `seq translate`, `seq normalize` and `interpolate`: transformations of
//! ordinary sequents, each with its claim checked by the library before it
//! is printed.

use crate::argument_parsing::{
    AnswerFormat, FormArg, InterpolateArgs, NormalizeArgs, TransformArgs, TranslateArgs,
};
use crate::limit::Deadline;
use crate::prove::{Stop, on_large_stack, stopped};
use crate::settings::{Given, time_limit};
use crate::{Status, catch_interrupt, interrupted, io};
use anyhow::{Result, bail};
use deduxit::linear::search::{Jobs, Reason};
use deduxit::ordinary::interpolant::{self, Partition};
use deduxit::ordinary::negative::{self, Agreement};
use deduxit::ordinary::normal::{self, NormalForm, Rule};
use deduxit::ordinary::{self, Evidence, Formulas, Logic, NodeId, Outcome, Verdict};
use deduxit::{Error, Refusal, Settings};
use std::fmt::Write;
use std::time::Instant;

/// Runs `seq translate`: translates the sequent, decides it and its
/// translation, and prints both verdicts. Returns the exit status: yes
/// where both are valid, no where neither is.
///
/// # Errors
///
/// An input that cannot be read, a target the translation does not
/// reach, verdicts that differ (a defect), an output that cannot be
/// written.
pub(crate) fn translate(args: &TranslateArgs, given: &Given) -> Result<Status> {
    let (settings, sequent, deadline) = start(&args.common, given)?;
    let options = negative::Options::default()
        .with_translation(args.by.into())
        .with_target(args.to.map(Into::into));
    let compared = on_large_stack(settings.limits.stack_bytes(), || {
        negative::compare(
            &sequent,
            &options,
            &settings.search,
            &settings.limits,
            |_| interrupted() || deadline.passed(),
        )
    })?;
    let compared = match compared {
        Ok(compared) => compared,
        Err(Error::Refused(Refusal::Stopped { .. })) => {
            return unknown(&args.common, &Stop::why(stopped(&deadline)));
        }
        Err(error) => return Err(error.into()),
    };
    let status = match compared.agreement {
        Agreement::Valid => Status::Yes,
        Agreement::NotValid => Status::No,
        _ => Status::Unknown,
    };
    let text = match args.common.format {
        AnswerFormat::Json => serde_json::to_string(&compared)?,
        AnswerFormat::Text => {
            let target = compared.decided.logic;
            let title = compared.translation.title();
            let mut text = format!("{title} into {target} logic: {}\n", compared.translated);
            for outcome in [&compared.classical, &compared.decided] {
                writeln!(
                    text,
                    "{} logic: {}",
                    outcome.logic,
                    verdict(outcome, &deadline)
                )?;
                model(&mut text, outcome);
            }
            text.push_str(match compared.agreement {
                Agreement::Valid | Agreement::NotValid => "The verdicts agree, as ",
                _ => "A search stopped, so it is not known whether the verdicts agree, as ",
            });
            write!(text, "{title} claims.")?;
            text
        }
    };
    io::write(args.common.output.as_deref(), &text)?;
    Ok(status)
}

/// Runs `seq normalize`: rewrites the sequent into the normal form step by
/// step, checks the form's promise and prints the steps and the check.
/// Returns the exit status: yes where the check passed, unknown where a
/// limit stopped it.
///
/// # Errors
///
/// An input that cannot be read, a form past its bound, a check that
/// fails (a defect), an output that cannot be written.
pub(crate) fn normalize(args: &NormalizeArgs, given: &Given) -> Result<Status> {
    let settings = args.settings(given)?;
    let (settings, sequent, deadline) = start_with(&args.common, settings)?;
    let options = normal::Options::default().with_form(args.to.into());
    let made = on_large_stack(settings.limits.stack_bytes(), || {
        normal::normalize(
            &sequent,
            &options,
            &settings.search,
            &settings.limits,
            |_| interrupted() || deadline.passed(),
        )
    })?;
    let made = match made {
        Ok(made) => made,
        Err(Error::Refused(Refusal::Stopped { .. })) => {
            return unknown(&args.common, &Stop::why(stopped(&deadline)));
        }
        Err(error @ Error::Refused(Refusal::Form { .. })) => {
            if matches!(args.to, FormArg::Cnf | FormArg::Dnf) {
                bail!(
                    "{error}; raise the limit with --form-limit, or take --to tseitin, whose \
                     clauses grow with the sequent"
                )
            }
            bail!("{error}; raise the limit with --form-limit")
        }
        Err(error) => return Err(error.into()),
    };
    let status = if made.is_checked() {
        Status::Yes
    } else {
        Status::Unknown
    };
    let text = match args.common.format {
        AnswerFormat::Json => serde_json::to_string(&made)?,
        AnswerFormat::Text => steps_text(&made, &deadline)?,
    };
    io::write(args.common.output.as_deref(), &text)?;
    Ok(status)
}

/// Returns the text of a normal form: the form, a line per step with the
/// sequent after it, and a line per claim checked.
///
/// # Errors
///
/// None in practice: writing into a string does not fail.
fn steps_text(made: &NormalForm, deadline: &Deadline) -> Result<String> {
    let mut title = made.form.title().to_owned();
    if let Some(first) = title.get_mut(..1) {
        first.make_ascii_uppercase();
    }
    let mut text = format!("{title}: {}\n", made.result);
    let formulas = made.result.formulas();
    let mut added: Vec<NodeId> = Vec::new();
    for (n, step) in made.steps.iter().enumerate() {
        let (before, after) = (formulas.formula(step.before), formulas.formula(step.after));
        let n = n + 1;
        match step.rule {
            Rule::Negate => writeln!(text, "{n}. {before} moves left of ⊢ as {after}")?,
            Rule::Define => {
                let clauses: Vec<String> = step
                    .added
                    .iter()
                    .map(|&c| formulas.formula(c).to_string())
                    .collect();
                writeln!(text, "{n}. {after} names {before}: {}", clauses.join(", "))?;
            }
            _ => {
                writeln!(text, "{n}. {}, at {before}:", step.rule.law())?;
                writeln!(
                    text,
                    "   {}",
                    sides(formulas, &step.left, &added, &step.right)
                )?;
            }
        }
        added.extend_from_slice(&step.added);
    }
    for check in &made.checks {
        writeln!(
            text,
            "{}: {}",
            check.claim.description(),
            verdict(&check.outcome, deadline)
        )?;
    }
    Ok(text.trim_end().to_owned())
}

/// Returns the sequent `left, added ⊢ right` over `formulas` as text.
fn sides(formulas: &Formulas, left: &[NodeId], added: &[NodeId], right: &[NodeId]) -> String {
    let list = |ids: &mut dyn Iterator<Item = &NodeId>| {
        ids.map(|&id| formulas.formula(id).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let (l, r) = (list(&mut left.iter().chain(added)), list(&mut right.iter()));
    match (l.is_empty(), r.is_empty()) {
        (true, _) => format!("⊢ {r}").trim_end().to_owned(),
        (false, true) => format!("{l} ⊢"),
        (false, false) => format!("{l} ⊢ {r}"),
    }
}

/// Runs `interpolate`: decides the sequent, reads its interpolant off the
/// derivation and prints it with both halves checked. Returns the exit
/// status: yes for a checked interpolant, no for a sequent that is not
/// valid.
///
/// # Errors
///
/// An input that cannot be read, a partition that is not one of the
/// sequent, an interpolant that fails its check (a defect), an output that
/// cannot be written.
pub(crate) fn interpolate(args: &InterpolateArgs, given: &Given) -> Result<Status> {
    let (settings, sequent, deadline) = start(&args.common, given)?;
    let logic: Logic = args.logic.into();
    let partition = match &args.first {
        None => Partition::hypotheses(&sequent),
        Some(first) => Partition::default()
            .with_left(first.iter().filter(|p| !p.right).map(|p| p.at).collect())
            .with_right(first.iter().filter(|p| p.right).map(|p| p.at).collect()),
    };
    let options = ordinary::Options::default().with_logic(logic);
    let found = on_large_stack(settings.limits.stack_bytes(), || {
        interpolant::interpolate(
            &sequent,
            &options,
            &partition,
            &settings.search,
            &settings.limits,
            |_| interrupted() || deadline.passed(),
        )
    })?;
    let found = match found {
        Ok(found) => found,
        Err(Error::Refused(Refusal::Stopped { .. })) => {
            return unknown(&args.common, &Stop::why(stopped(&deadline)));
        }
        Err(error) => return Err(error.into()),
    };
    let status = match (&found.outcome.verdict, &found.interpolant) {
        (_, Some(i)) if i.is_checked() => Status::Yes,
        (Verdict::NotValid, _) => Status::No,
        _ => Status::Unknown,
    };
    let text = match args.common.format {
        AnswerFormat::Json => serde_json::to_string(&found)?,
        AnswerFormat::Text => {
            let mut text = String::new();
            match &found.interpolant {
                Some(i) => {
                    writeln!(text, "interpolant: {}", i.formula())?;
                    for (half, outcome) in
                        [(&i.first, &i.first_outcome), (&i.second, &i.second_outcome)]
                    {
                        writeln!(text, "{half}: {}", verdict(outcome, &deadline))?;
                    }
                }
                None => {
                    writeln!(
                        text,
                        "{logic} logic: {}",
                        verdict(&found.outcome, &deadline)
                    )?;
                    model(&mut text, &found.outcome);
                    text.push_str(match found.outcome.verdict {
                        Verdict::NotValid => "A sequent that is not valid has no interpolant.",
                        Verdict::Valid(_) => {
                            "No derivation without cuts was found within the limits, so no \
                             interpolant was read off; raise --timeout or --work-limit."
                        }
                        _ => "No interpolant is read off a sequent the search did not decide.",
                    });
                }
            }
            text.trim_end().to_owned()
        }
    };
    io::write(args.common.output.as_deref(), &text)?;
    Ok(status)
}

/// Reads the settings and the sequent, and starts the time limit.
///
/// # Errors
///
/// A settings file or an input that cannot be read.
fn start(args: &TransformArgs, given: &Given) -> Result<(Settings, ordinary::Sequent, Deadline)> {
    start_with(args, args.settings(given)?)
}

/// Reads the sequent under the settings given, and starts the time limit.
///
/// # Errors
///
/// An input that cannot be read.
fn start_with(
    args: &TransformArgs,
    mut settings: Settings,
) -> Result<(Settings, ordinary::Sequent, Deadline)> {
    catch_interrupt();
    // These commands run the library's searches, which take every thread
    // from the start under `auto`; one thread is enough for a sequent a
    // person writes, and gives the same derivation every time.
    if settings.search.jobs == Jobs::Auto {
        settings.search = settings.search.with_jobs(1);
    }
    let sequent = args.input.ordinary(&settings.limits)?;
    let deadline = Deadline::start(time_limit(&settings), Instant::now())?;
    Ok((settings, sequent, deadline))
}

/// Writes the line of an answer a stop ended, as text or on standard
/// error beside JSON, which has no form for it, and returns unknown.
///
/// # Errors
///
/// An output that cannot be written.
fn unknown(args: &TransformArgs, why: &str) -> Result<Status> {
    let line = format!("unknown: {why}");
    match args.format {
        AnswerFormat::Text => io::write(args.output.as_deref(), &line)?,
        AnswerFormat::Json => eprintln!("{line}"),
    }
    Ok(Status::Unknown)
}

/// Returns an outcome's verdict and what shows it: `valid, a checked
/// derivation of LK (5 inferences)`.
fn verdict(outcome: &Outcome, deadline: &Deadline) -> String {
    match &outcome.verdict {
        Verdict::Valid(Evidence::Derivation(d)) => {
            let n = d.inferences().len();
            let calculus = d.logic().calculus();
            format!(
                "valid, a checked derivation of {calculus} ({n} inference{})",
                if n == 1 { "" } else { "s" }
            )
        }
        Verdict::Valid(Evidence::Clausal(_)) => "valid, a checked clausal proof".to_owned(),
        Verdict::Valid(Evidence::Table) => "valid, by its truth table".to_owned(),
        Verdict::Valid(_) => "valid, the checked proof of its image in linear logic".to_owned(),
        Verdict::NotValid if outcome.countermodel.is_some() => {
            "not valid, a checked Kripke model refutes it:".to_owned()
        }
        Verdict::NotValid => "not valid".to_owned(),
        Verdict::Unknown(Reason::Stopped) => format!("unknown, {}", Stop::why(stopped(deadline))),
        Verdict::Unknown(reason) => format!("unknown, {reason}"),
        _ => "decided".to_owned(),
    }
}

/// Writes the countermodel of an outcome that has one, indented under
/// its verdict.
fn model(text: &mut String, outcome: &Outcome) {
    if let Some(countermodel) = &outcome.countermodel {
        for line in countermodel.model.to_string().lines() {
            text.push_str("  ");
            text.push_str(line);
            text.push('\n');
        }
    }
}
