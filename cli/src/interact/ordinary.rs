// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `interact --logic`: a session that proves a sequent of classical,
//! intuitionistic or minimal logic step by step by the rules of LK or LJ,
//! over the library's goal state of ordinary sequents. Its commands are
//! the linear session's where they mean the same; a formula is named by
//! its side and its position there, `L0` or `R1`, as the library's
//! refusals name it.

use super::{
    Commands, Input, Ran, Saved, load, no_standard_input, proof_target, proof_written, run,
    show_target, so_far, written,
};
use crate::argument_parsing::{Format, InteractArgs};
use crate::io::{Most, refused};
use crate::limit::{Deadline, Notice};
use crate::prove::{Ended, NOTICE_AFTER, notice_line, on_large_stack, stopped};
use crate::settings::Threads;
use crate::show::{Show, Shown, bytes_text};
use crate::{catch_interrupt, clear_interrupt, interrupted, io, style};
use anyhow::{Context, Result, bail};
use deduxit::export::{Styles, latex, svg, typst};
use deduxit::linear::search::Options;
use deduxit::ordinary::{self, Interactive, Logic, Needs, Rule, Side, Step, Translation};
use deduxit::{
    Error, Limits, Refusal, Settings,
    linear::{GoalId, ViewOptions},
};
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
                    both take the whole context; ∀L and ∃R (forallL, existsR) take a witness
                    term in place of F…, or none to leave it open as $1, $2, … for an
                    axiom to fill, and ∀R and ∃L name a fresh eigenvariable
undo                retract the last step, with the invertible steps that followed it
invertible on|off   apply every invertible rule by itself, at once to every open goal and
                    then to the goals each step opens (LJ's ->L, ~L, orR1 and orR2 are
                    not invertible); undo takes back a step with the ones that followed it
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
/// runs the commands of `input` until `quit` or their end, on a thread
/// whose stack fits the search's recursion limit, and returns how it
/// ended.
///
/// # Errors
///
/// A mode flag beside the logic, a sequent that cannot
/// be read or has more than one formula right of `⊢` in intuitionistic or
/// minimal logic, or a terminal on standard input; a command's error is
/// its output and the session goes on.
pub(super) fn interact(
    args: &InteractArgs,
    settings: Settings,
    saved: Option<Interactive>,
    input: Input,
) -> Result<Ran> {
    let mode = &args.mode;
    if mode.intuitionistic || mode.affine || mode.mix {
        bail!("the logic's translation decides the mode: leave out -i, -a and --mix");
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
        invertible: false,
        groups: Vec::new(),
    };
    let invertible = settings.interactive.invertible;
    on_large_stack(limits.stack_bytes(), move || {
        let first = if invertible {
            session.invert_all()
        } else {
            None
        };
        run(
            &mut session,
            first.map(|taken| format!("applied {taken}")),
            input,
        )
    })?
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
    /// Whether every invertible rule is applied to the goals each step
    /// opens.
    invertible: bool,
    /// The steps `undo` retracts together, each a step with the invertible
    /// steps after it or the invertible steps on every goal, by the
    /// session's count of steps before and after them; a step outside
    /// them is retracted alone.
    groups: Vec<(usize, usize)>,
}

impl Commands for Session {
    fn command(&mut self, words: &[&str]) -> Result<String> {
        let (command, rest) = words.split_first().map_or(("", &[][..]), |(c, r)| (*c, r));
        let goal = |i: usize| -> Result<GoalId> {
            let word = rest.get(i).context("which goal? see `goals`")?;
            // Rust's parse error says nothing a user can act on.
            let id: u32 = word.parse().map_err(|_| {
                anyhow::anyhow!(
                    "{word:?} is not a goal: a goal is a number, and `goals` lists them"
                )
            })?;
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
                            Needs::Witness => format!("{} (with a witness or none)", r.rule),
                            _ => r.rule.to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join("  ")
                }
            }
            "apply" => {
                let (goal, (side, position)) = (goal(0)?, formula(1)?);
                let word = rest.get(2).context("which rule? see `rules`")?;
                let rule: Rule = word.parse().map_err(|error| match error {
                    Error::UnknownName { .. } => anyhow::anyhow!(
                        "{}",
                        super::unknown_rule(
                            word,
                            &ordinary_rules(self.state.logic(), binders(&self.state)),
                            ASCII
                        )
                    ),
                    error => error.into(),
                })?;
                let mut step = Step::new(side, position, rule);
                if matches!(rule, Rule::ForallLeft | Rule::ExistsRight) && rest.len() > 3 {
                    step = step.with_witness(self.state.witness(goal, &rest[3..].join(" "))?);
                } else if rest.len() > 3 {
                    let (mut left, mut right) = (Vec::new(), Vec::new());
                    for word in &rest[3..] {
                        match address(word)? {
                            (Side::Left, p) => left.push(p),
                            (_, p) => right.push(p),
                        }
                    }
                    step = step.split(&left, &right);
                }
                let before = self.state.steps();
                let bound = self.state.bindings().len();
                let opened = self.state.apply(goal, &step)?;
                let filled: Vec<String> = self.state.bindings()[bound..]
                    .iter()
                    .map(|(witness, term)| format!("filled {witness} with {term}\n"))
                    .collect();
                format!("{}{}", filled.concat(), self.stepped(before, opened)?)
            }
            "undo" => {
                let steps = self.state.steps();
                let back = match self.groups.last() {
                    Some(&(before, after)) if after == steps => {
                        self.groups.pop();
                        steps - before
                    }
                    _ => 1,
                };
                let mut reopened = None;
                for _ in 0..back {
                    reopened = self.state.undo().or(reopened);
                }
                match reopened {
                    Some(goal) if back > 1 => {
                        format!("retracted {back} steps\nreopened {}", self.goal_line(goal))
                    }
                    Some(goal) => format!("reopened {}", self.goal_line(goal)),
                    None => "nothing to undo".to_owned(),
                }
            }
            "invertible" => match rest {
                ["on"] => {
                    let then = self.invert_all();
                    let goals = self.goals();
                    match then {
                        Some(then) => format!("invertible rules on; then {then}\n{goals}"),
                        None => format!("invertible rules on\n{goals}"),
                    }
                }
                ["off"] => {
                    self.invertible = false;
                    "invertible rules off".to_owned()
                }
                _ => bail!("invertible on|off: say which"),
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
                    Saved::Linear(_) | Saved::Rules(_) => bail!(
                        "{path} holds a session of linear logic or of a calculus given as \
                         data; `load` resumes one of this sequent, and `deduxit interact \
                         --state {path}` starts that one"
                    ),
                };
                // The exit status answers for the session's sequent and
                // logic, so a file of another question is refused, as
                // `--state` beside a sequent is.
                if sequent_text(&state) != sequent_text(&self.state) {
                    bail!(
                        "{path} holds a session of another sequent; `load` resumes one of this \
                         sequent, and `deduxit interact --state {path}` starts that one"
                    );
                }
                if state.logic() != self.state.logic() {
                    bail!(
                        "{path} holds a session in {} logic, this one is in {} logic; \
                         `deduxit interact --state {path}` starts that one",
                        state.logic(),
                        self.state.logic()
                    );
                }
                self.state = state;
                // A loaded session's steps are undone one at a time.
                self.groups.clear();
                self.goals()
            }
            "cut" => bail!(
                "`cut` is a command of a session of linear logic; a session of LK or LJ has \
                 the rules of the calculus alone, without cut"
            ),
            _ => bail!("unknown command {command:?}; `help` lists them"),
        })
    }

    fn finished(&self) -> bool {
        self.state.finished(&self.limits, |_| false).is_ok()
    }
}

impl Session {
    /// Turns the invertible rules on and applies them to every open goal,
    /// as one step for `undo`, and says how many steps that took.
    fn invert_all(&mut self) -> Option<String> {
        self.invertible = true;
        let before = self.state.steps();
        let inverted = self.state.invert_all(&self.limits);
        if inverted.steps > 0 {
            self.groups.push((before, self.state.steps()));
        }
        super::inverted_line(inverted.steps, inverted.finished.as_ref().err())
    }

    /// Returns what a step that opened `opened` prints, the session's
    /// steps having been `before` steps: with the invertible rules on,
    /// after applying them to the goals it opened, as one step for `undo`.
    fn stepped(&mut self, before: usize, opened: Vec<GoalId>) -> Result<String> {
        if !self.invertible {
            return Ok(self.opened(&opened));
        }
        let (mut steps, mut open, mut refused) = (0, Vec::new(), None);
        for goal in opened {
            let inverted = self.state.invert(goal, &self.limits)?;
            steps += inverted.steps;
            open.extend(inverted.open);
            refused = refused.or(inverted.finished.err());
        }
        self.groups.push((before, self.state.steps()));
        let then = super::inverted_line(steps, refused.as_ref());
        let then = then.map(|t| format!("then {t}\n")).unwrap_or_default();
        Ok(format!("{then}{}", self.opened(&open)))
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
        let ordinary::Search::Embedding {
            translation,
            linear,
        } = &outcome.search
        else {
            unreachable!("a session closes a goal through the embedding")
        };
        let line = crate::ordinary::verdict_line(linear, outcome.logic, *translation, &ended);
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

/// The ASCII spellings of the rules of LK and LJ that have one, each
/// beside the rule's name: the words, since the command's word splitter
/// takes `\` as an escape, and `->` and `~` as the parser reads them.
pub(super) const ASCII: &[(&str, &str)] = &[
    ("∧L", "andL"),
    ("∧L₁", "andL1"),
    ("∧L₂", "andL2"),
    ("∧R", "andR"),
    ("∨L", "orL"),
    ("∨R", "orR"),
    ("∨R₁", "orR1"),
    ("∨R₂", "orR2"),
    ("→L", "->L"),
    ("→R", "->R"),
    ("¬L", "~L"),
    ("¬R", "~R"),
    ("↔L", "iffL"),
    ("↔L₁", "iffL1"),
    ("↔L₂", "iffL2"),
    ("↔R", "iffR"),
    ("⊥L", "falseL"),
    ("⊥R", "falseR"),
    ("⊤L", "trueL"),
    ("⊤R", "trueR"),
    ("∀L", "forallL"),
    ("∀R", "forallR"),
    ("∃L", "existsL"),
    ("∃R", "existsR"),
];

/// Returns the names of the rules a step takes in `logic`: every rule of
/// LK but a cut, and in LJ and minimal logic none that would leave other
/// than one formula right of `⊢` (nor `⊥L` in minimal logic); the
/// quantifier rules only where the sequent has `binders`.
pub(super) fn ordinary_rules(logic: Logic, binders: bool) -> Vec<&'static str> {
    const LK: [&str; 29] = [
        "ax", "WL", "WR", "CL", "CR", "∧L", "∧L₁", "∧L₂", "∧R", "∨L", "∨R", "∨R₁", "∨R₂", "→L",
        "→R", "¬L", "¬R", "↔L", "↔L₁", "↔L₂", "↔R", "⊥L", "⊥R", "⊤L", "⊤R", "∀L", "∀R", "∃L", "∃R",
    ];
    LK.into_iter()
        .filter(|&name| match name {
            "WR" | "CR" | "⊥R" | "∨R" => logic == Logic::Classical,
            "⊥L" => logic != Logic::Minimal,
            "∀L" | "∀R" | "∃L" | "∃R" => binders,
            _ => true,
        })
        .collect()
}

/// Returns whether the session's formulas have a quantifier.
fn binders(state: &Interactive) -> bool {
    let formulas = state.formulas();
    let (left, right) = state.sequent();
    let mut stack: Vec<_> = left.iter().chain(right).copied().collect();
    while let Some(k) = stack.pop() {
        let node = formulas.node(k);
        if matches!(node, ordinary::Node::Forall(_) | ordinary::Node::Exists(_)) {
            return true;
        }
        let (a, b) = node.operands();
        stack.extend(a.into_iter().chain(b));
    }
    false
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
