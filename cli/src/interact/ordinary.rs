// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `interact --logic`: a session that proves a sequent of classical,
//! intuitionistic or minimal logic step by step by the rules of LK or LJ,
//! over the library's goal state of ordinary sequents. Its commands are
//! the linear session's where they mean the same; a formula is named by
//! its side and its position there, `L0` or `R1`, as the library's
//! refusals name it.

use super::{
    Commands, Saved, load, no_standard_input, proof_target, proof_written, run, show_target,
    so_far, written,
};
use crate::argument_parsing::{Format, InteractArgs};
use crate::io::{Most, refused};
use crate::limit::{Deadline, Notice};
use crate::prove::{Ended, NOTICE_AFTER, notice_line, on_large_stack, stopped};
use crate::settings::Threads;
use crate::show::{Show, Shown, bytes_text};
use crate::{Status, catch_interrupt, clear_interrupt, interrupted, io, style};
use anyhow::{Context, Result, bail};
use deduxit::export::{Styles, latex, svg, typst};
use deduxit::ordinary::{Interactive, Needs, Rule, Side, Step, Translation};
use deduxit::search::Options;
use deduxit::{Error, GoalId, Limits, Refusal, Settings, ViewOptions};
use std::path::Path;
use std::time::{Duration, Instant};

/// The commands, as `help` prints them.
const HELP: &str = "\
goals               the open goals, each formula with its side and position: L0, L1, …
                    left of ⊢ and R0, R1, … right of it
rules G F           the rules that act on formula F of goal G, such as R0
apply G F RULE [F…] apply a rule, by its name or an ASCII spelling (andR, orR1, ->L or
                    impL, ~R or notR, iffL, falseL, WL, CR); the formulas F… go to the first
                    premise of a rule with two, the others to the second, which otherwise
                    both take the whole context
undo                retract the last step
close [G]           let the search decide goal G, or every open goal, and graft its
                    derivation
show [--FORMAT] [FILE]
                    the derivation so far, open goals included, printed or written to FILE:
                    as text, or --latex, --typst, --svg, --png or --pdf (these two need a
                    FILE); with a FILE and no format, the one its extension names
proof [--FORMAT] [FILE]
                    check the finished derivation and print it or write it to FILE: as
                    text, or --json, --latex, --typst, --svg, --png, --pdf, or --rocq for
                    a certificate (--png and --pdf need a FILE); with a FILE and no
                    format, the one its extension names, else JSON
lambda              the finished derivation's simply typed lambda-term, as its typing
                    judgement (prove's --lambda); LK and an open goal say why it has none
save FILE           write the session as JSON
load FILE           resume a session written by save
help                this list
quit                end the session";

/// Runs `interact` on a sequent of an ordinary logic (`--logic`), or on
/// the session of LK or LJ that `saved` holds: starts the session, then
/// reads commands from standard input until `quit` or the end of input,
/// on a thread whose stack fits the search's recursion limit.
///
/// # Errors
///
/// A mode flag or `--invertible` beside the logic, a sequent that cannot
/// be read or has more than one formula right of `⊢` in intuitionistic or
/// minimal logic, or a terminal on standard input; a command's error is
/// its output and the session goes on.
pub(super) fn interact(
    args: &InteractArgs,
    settings: Settings,
    saved: Option<Interactive>,
) -> Result<Status> {
    let mode = &args.mode;
    if mode.intuitionistic || mode.affine || mode.mix {
        bail!("the logic's translation decides the mode: leave out -i, -a and --mix");
    }
    if args.invertible {
        bail!("a session of LK or LJ applies no rule by itself: leave out --invertible");
    }
    let limits = settings.limits;
    let state = match (saved, args.logic.logic) {
        (Some(state), _) => state,
        (None, Some(logic)) => {
            no_standard_input(args)?;
            let sequent = args.input.ordinary(&limits)?;
            Interactive::within(&sequent, logic.into(), &limits)
                .map_err(|e| refused(e, Most::of(&limits)))?
        }
        (None, None) => unreachable!("a session of LK or LJ is saved or has a logic"),
    };
    // The library's `close` has no race of one thread against a pool: a
    // close searches on one thread unless `--jobs` asks for a count from
    // the start, never on every core at once.
    let threads = Threads::of(&settings);
    let jobs = if threads.alone.is_some() {
        1
    } else {
        threads.jobs
    };
    catch_interrupt();
    let mut session = Session {
        styles: style::read(&settings.styles, &args.style, Some(&args.rocq), None, false)?,
        lambda: settings.lambda,
        state,
        translation: args
            .logic
            .translation
            .map(Into::into)
            .or(settings.ordinary.translation),
        deepens: settings.search.copies.is_none(),
        timeout: crate::settings::time_limit(&settings),
        options: settings.search.with_jobs(jobs),
        view: settings.view,
        limits,
    };
    on_large_stack(limits.stack_bytes(), move || run(&mut session, None))?
}

/// A running session of LK or LJ: the state, the search settings of
/// `close` and the styles of the outputs.
struct Session {
    /// How `show` and `proof` write each format.
    styles: Styles,
    /// How `lambda` prints the proof's lambda-term.
    lambda: deduxit::lambda::Options,
    /// The proof in progress.
    state: Interactive,
    /// The translation a `close` decides a goal by, `None` for the
    /// logic's default.
    translation: Option<Translation>,
    /// The search settings of `close`.
    options: Options,
    /// How `show` and `proof` draw the structural rules.
    view: ViewOptions,
    /// The bounds of a `close`'s search, of the derivation it grafts and
    /// of the derivations drawn.
    limits: Limits,
    /// How long a `close` may take.
    timeout: Option<Duration>,
    /// Whether a `close` deepens the copy bound without a bound.
    deepens: bool,
}

impl Commands for Session {
    fn command(&mut self, words: &[&str]) -> Result<String> {
        let (command, rest) = words.split_first().map_or(("", &[][..]), |(c, r)| (*c, r));
        let goal = |i: usize| -> Result<GoalId> {
            let word = rest.get(i).context("which goal? see `goals`")?;
            let id: u32 = word
                .parse()
                .with_context(|| format!("{word:?} is not a goal"))?;
            Ok(GoalId::new(id))
        };
        let formula = |i: usize| -> Result<(Side, usize)> {
            address(rest.get(i).context("which formula? see `goals`")?)
        };
        Ok(match command {
            "" => String::new(),
            "help" => HELP.to_owned(),
            "goals" => self.goals(),
            "rules" => {
                let (side, position) = formula(1)?;
                let rules = self.state.rules(goal(0)?, side, position)?;
                if rules.is_empty() {
                    "no rule acts on it".to_owned()
                } else {
                    rules
                        .iter()
                        .map(|r| match r.needs {
                            Needs::Context => format!("{} (with a split or none)", r.rule),
                            _ => r.rule.to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join("  ")
                }
            }
            "apply" => {
                let (goal, (side, position)) = (goal(0)?, formula(1)?);
                let rule: Rule = rest.get(2).context("which rule? see `rules`")?.parse()?;
                let mut step = Step::new(side, position, rule);
                if rest.len() > 3 {
                    let (mut left, mut right) = (Vec::new(), Vec::new());
                    for word in &rest[3..] {
                        match address(word)? {
                            (Side::Left, p) => left.push(p),
                            (_, p) => right.push(p),
                        }
                    }
                    step = step.split(&left, &right);
                }
                let opened = self.state.apply(goal, &step)?;
                self.opened(&opened)
            }
            "undo" => match self.state.undo() {
                Some(goal) => format!("reopened {}", self.goal_line(goal)),
                None => "nothing to undo".to_owned(),
            },
            "close" => {
                let goals = match rest.first() {
                    Some(_) => vec![goal(0)?],
                    None => self.state.goals().collect(),
                };
                let mut lines = Vec::new();
                for goal in goals {
                    lines.push(format!("goal {}: {}", goal.get(), self.close(goal)?));
                }
                if self.state.is_complete() {
                    lines.push("no goal is open: `proof` checks the proof".to_owned());
                }
                lines.join("\n")
            }
            "show" => {
                let (format, path) = show_target(rest)?;
                let d = self
                    .state
                    .derivation_within(&self.view, &self.limits, |_| false)?;
                let styles = &self.styles;
                let text = match format {
                    Format::Latex => {
                        written(|out| latex::write(&d, &styles.latex, out, |_| false))?
                    }
                    Format::Typst => {
                        written(|out| typst::write(&d, &styles.typst, out, |_| false))?
                    }
                    Format::Svg | Format::Png | Format::Pdf => {
                        written(|out| svg::write(&d, &styles.svg, out, |_| false))?
                    }
                    _ => written(|out| d.write_text(&styles.text, out, |_| false))?,
                };
                so_far(text, format, path, styles, &self.limits)?
            }
            "proof" => {
                let (format, path) = proof_target(rest)?;
                // A Ctrl-C of an earlier `close` must not stop it.
                clear_interrupt();
                let d = self.state.finished(&self.limits, |_| interrupted())?;
                let logic = self.state.logic();
                let mut text = String::new();
                if path.is_none() {
                    text = format!("valid proof in {} ({logic} logic)", logic.calculus());
                }
                if format == Format::Json {
                    let separator = if text.is_empty() { "" } else { "\n" };
                    text = format!("{text}{separator}{}", serde_json::to_string(&d)?);
                    return proof_written(Shown::Written, text, path);
                }
                let mut show = Show::session(format, self.view, self.limits, self.styles.clone());
                show.verdict = path.is_none();
                let prefix = (!text.is_empty()).then_some("\n");
                let shown = crate::show::shown(
                    crate::ordinary::shown(d, &show),
                    None,
                    &show,
                    interrupted,
                    || "interrupted".to_owned(),
                    prefix,
                    &mut text,
                )?;
                proof_written(shown, text, path)?
            }
            "lambda" => {
                let d = self
                    .state
                    .derivation_within(&self.view, &self.limits, |_| false)?;
                super::lambda_line(&d.term(&self.limits, |_| false), &self.lambda, &self.limits)
            }
            "save" => {
                let [path] = rest else {
                    bail!("save where? give one file, in quotes if its name has blanks");
                };
                io::write(Some(Path::new(path)), &serde_json::to_string(&self.state)?)?;
                format!("session written to {path}")
            }
            "load" => {
                let [path] = rest else {
                    bail!("load what? give one file, in quotes if its name has blanks");
                };
                let state = match load(Path::new(path), &self.limits)? {
                    Saved::Ordinary(state) => *state,
                    Saved::Linear(_) => bail!(
                        "{path} holds a session of linear logic; `load` resumes one of this \
                         sequent, and `linlog interact --state {path}` starts that one"
                    ),
                };
                // The exit status answers for the session's sequent and
                // logic, so a file of another question is refused, as
                // `--state` beside a sequent is.
                if sequent_text(&state) != sequent_text(&self.state) {
                    bail!(
                        "{path} holds a session of another sequent; `load` resumes one of this \
                         sequent, and `linlog interact --state {path}` starts that one"
                    );
                }
                if state.logic() != self.state.logic() {
                    bail!(
                        "{path} holds a session in {} logic, this one is in {} logic; \
                         `linlog interact --state {path}` starts that one",
                        state.logic(),
                        self.state.logic()
                    );
                }
                self.state = state;
                self.goals()
            }
            "cut" | "invertible" => bail!(
                "`{command}` is a command of a session of linear logic; a session of LK or LJ \
                 has the rules of the calculus alone, without cut, and applies none by itself"
            ),
            _ => bail!("unknown command {command:?}; `help` lists them"),
        })
    }

    fn finished(&self) -> bool {
        self.state.finished(&self.limits, |_| false).is_ok()
    }
}

impl Session {
    /// Decides a goal through the session's translation, stopped by the
    /// time limit or Ctrl-C, grafts its derivation if it is valid, and
    /// returns the verdict line.
    fn close(&mut self, goal: GoalId) -> Result<String> {
        clear_interrupt();
        let start = Instant::now();
        let deadline = Deadline::start(self.timeout, start)?;
        let notice = Notice::start(NOTICE_AFTER, notice_line(self.timeout, self.deepens));
        let closed = self
            .state
            .close(goal, self.translation, &self.options, &self.limits, |_| {
                interrupted() || deadline.passed()
            });
        drop(notice);
        let ended = Ended {
            stop: stopped(&deadline),
            elapsed: start.elapsed(),
        };
        let closed = match closed {
            Ok(closed) => closed,
            Err(Error::Refused(Refusal::Stopped { .. })) => bail!(
                "the goal was decided, but its derivation was not read back before the time \
                 limit or the interrupt; the goal stays open"
            ),
            Err(error) => return Err(error.into()),
        };
        let outcome = &closed.outcome;
        let line = crate::ordinary::verdict_line(
            &outcome.linear,
            outcome.logic,
            outcome.translation,
            &ended,
        );
        match closed.grafted {
            Ok(()) => Ok(line),
            Err(Refusal::Output {
                estimate_bytes,
                limit_bytes,
                ..
            }) => bail!(
                "the goal is valid, but the derivation to graft is too large: it is estimated at \
                 {}, over the limit of {}; the goal stays open (--derivation-limit raises the \
                 limit)",
                bytes_text(estimate_bytes),
                bytes_text(limit_bytes)
            ),
            Err(refusal) => {
                bail!("the goal is valid, but its derivation is not grafted: {refusal}")
            }
        }
    }

    /// Lists the open goals, or says that none is.
    fn goals(&self) -> String {
        let goals: Vec<GoalId> = self.state.goals().collect();
        if goals.is_empty() {
            return "no goal is open: `proof` checks the proof".to_owned();
        }
        goals
            .iter()
            .map(|&g| self.goal_line(g))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Lists goals just opened, or says that the step closed its goal.
    fn opened(&self, goals: &[GoalId]) -> String {
        if goals.is_empty() {
            return if self.state.is_complete() {
                "closed; no goal is open: `proof` checks the proof".to_owned()
            } else {
                "closed".to_owned()
            };
        }
        goals
            .iter()
            .map(|&g| format!("opened {}", self.goal_line(g)))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Returns `goal G: ` and the goal's sequent with every formula's side
    /// and position: `goal 1: L0: a, L1: b ⊢ R0: a ∧ b`.
    fn goal_line(&self, goal: GoalId) -> String {
        let (left, right) = self.state.goal(goal).unwrap_or_default();
        let formulas = self.state.formulas();
        let side = |letter: char, ids: &[deduxit::ordinary::NodeId]| {
            ids.iter()
                .enumerate()
                .map(|(i, &id)| format!(" {letter}{i}: {}", formulas.formula(id)))
                .collect::<Vec<_>>()
                .join(",")
        };
        format!(
            "goal {}:{} ⊢{}",
            goal.get(),
            side('L', left),
            side('R', right)
        )
    }
}

/// Returns the side and position a word names: `L` or `R` (either case)
/// and a position, as `goals` writes them.
///
/// # Errors
///
/// A word of another form.
fn address(word: &str) -> Result<(Side, usize)> {
    let mut chars = word.chars();
    let side = match chars.next() {
        Some('L' | 'l') => Side::Left,
        Some('R' | 'r') => Side::Right,
        _ => bail!("{}", not_a_formula(word)),
    };
    let position = chars
        .as_str()
        .parse()
        .with_context(|| not_a_formula(word))?;
    Ok((side, position))
}

/// Says that a word names no formula, and how one is named.
fn not_a_formula(word: &str) -> String {
    format!(
        "{word:?} is not a formula: L0, L1, … name those left of ⊢ and R0, R1, … those right of \
         it, as `goals` shows them"
    )
}

/// Returns the sequent a session proves as text, which compares two
/// sessions whose formulas are numbered apart.
fn sequent_text(state: &Interactive) -> String {
    let (left, right) = state.sequent();
    let formulas = state.formulas();
    let side = |ids: &[deduxit::ordinary::NodeId]| {
        ids.iter()
            .map(|&id| formulas.formula(id).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!("{} ⊢ {}", side(left), side(right))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A formula is named by its side's letter, in either case, and its
    /// position; any other word is refused with how one is named.
    #[test]
    fn a_formula_is_named_by_its_side_and_position() {
        assert_eq!(address("L0").unwrap(), (Side::Left, 0));
        assert_eq!(address("r12").unwrap(), (Side::Right, 12));
        for refused in ["0", "L", "X1", "R-1", ""] {
            assert!(address(refused).is_err(), "{refused}");
        }
    }
}
