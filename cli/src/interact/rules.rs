// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `interact --calculus`: a session of step-by-step proving in a calculus
//! given as data, over `deduxit::rules::Interactive`.

use super::{Commands, Saved, load, run};
use crate::Status;
use crate::argument_parsing::InteractArgs;
use crate::io;
use anyhow::{Context, Result, bail};
use deduxit::Settings;
use deduxit::rules::{Apply, Derive, GoalId, Interactive, LineId, Undone};
use std::path::Path;

/// The session's commands, as `help` lists them.
const HELP: &str = "\
goals               the open goals, each member numbered across the zones: 0, 1, …,
                    with the lines in force, numbered 1, 2, …
rules G             the rules that apply to goal G, with the members they act on and what
                    each needs given
apply G RULE [N…] [NAME=TEXT…]
                    apply a rule backwards to goal G, acting on members N…; NAME=TEXT
                    gives what the goal leaves open: a formula for a variable, members
                    separated by commas for a context (in quotes if TEXT has blanks)
derive G RULE L… [NAME=TEXT…]
                    apply a rule forwards from lines L… in force at goal G, which adds
                    its conclusion as a new line there; NAME=TEXT gives what the lines
                    leave open
assume G [L]        close goal G by line L in force, or by the line in force equal to it
undo                retract the last step
show                the derivation so far, a line per node
proof [FILE]        check the finished derivation and print it, or write it to FILE as JSON
save FILE           write the session as JSON
load FILE           resume a session written by save
help                this list
quit                end the session";

/// A session in a calculus given as data, and the bounds it keeps to.
struct Session {
    /// The state.
    state: Interactive,
    /// The bounds of a file read.
    limits: deduxit::Limits,
}

/// Runs `interact --calculus` on the sequent and assumptions the
/// arguments give, or on the session `saved` holds, until `quit` or the
/// end of standard input.
///
/// # Errors
///
/// A calculus or a sequent that cannot be read, or an output that cannot
/// be written.
pub(super) fn interact(
    args: &InteractArgs,
    settings: Settings,
    saved: Option<Interactive>,
) -> Result<Status> {
    if args.mode.given() || args.logic.logic.is_some() {
        bail!(
            "a session in a calculus given as data has its calculus's rules: leave out the mode flags and --logic"
        );
    }
    let limits = settings.limits;
    let state = match saved {
        Some(state) => state,
        None => {
            let path = args
                .calculus
                .as_deref()
                .context("--calculus names the calculus")?;
            let calculus = crate::calculus::named(path, &limits)?;
            let Some(goal) = &args.input.sequent else {
                bail!("give the sequent to derive as the argument, or a session with --state");
            };
            let assumptions: Vec<&str> = args.assume.iter().map(String::as_str).collect();
            Interactive::within(calculus, goal, &assumptions, &limits)?
        }
    };
    let mut session = Session { state, limits };
    run(&mut session, None)
}

impl Session {
    /// Returns a goal's line: its members numbered across the zones, and
    /// the hypotheses in force where there are any.
    fn goal_line(&self, goal: GoalId) -> String {
        let calculus = self.state.calculus();
        let terms = self.state.terms();
        let mut text = format!("goal {}:", goal.index());
        let mut number = 0usize;
        let mut written = false;
        for (zone, (members, mark)) in self
            .state
            .goal(goal)
            .iter()
            .zip(calculus.marks())
            .enumerate()
        {
            if let Some(mark) = mark {
                text.push(' ');
                text.push_str(mark);
            } else if zone > 0 && written {
                text.push_str(" ;");
            }
            for (place, &member) in members.iter().enumerate() {
                let separator = if place == 0 { " " } else { ", " };
                text.push_str(&format!(
                    "{separator}{number}: {}",
                    calculus.show(terms, member)
                ));
                number += 1;
                written = true;
            }
        }
        let lines = self.state.lines(goal);
        if !lines.is_empty() {
            let shown: Vec<String> = lines
                .iter()
                .map(|(line, sequent)| {
                    format!(
                        "{}. {}",
                        line.number(),
                        crate::calculus::sequent_of(calculus, terms, sequent)
                    )
                })
                .collect();
            text.push_str(&format!("\n    lines: {}", shown.join("; ")));
        }
        text
    }

    /// Lists goals just opened, or says that the step closed its goal.
    fn opened(&self, goals: &[GoalId]) -> String {
        if goals.is_empty() {
            return if self.state.is_finished() {
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

    /// Returns the open goals, a line each, or that none is left.
    fn goals(&self) -> String {
        let goals = self.state.goals();
        if goals.is_empty() {
            return "no goal is open: `proof` checks the proof".to_owned();
        }
        goals
            .into_iter()
            .map(|goal| self.goal_line(goal))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Returns the derivation so far, a line per node.
    fn show(&self) -> Result<String> {
        let derivation = self.state.derivation()?;
        Ok(crate::calculus::lines(&derivation, self.state.calculus()))
    }
}

/// Returns the line a word names.
fn line(word: &str) -> Result<LineId> {
    let number: u32 = word.parse().with_context(|| {
        format!("{word:?} names no line: give its number, as a goal's `lines` shows it")
    })?;
    Ok(LineId::new(number))
}

/// Returns the goal a word names.
fn goal(word: Option<&&str>) -> Result<GoalId> {
    let word = word.context("which goal? see `goals`")?;
    let index: u32 = word
        .parse()
        .with_context(|| format!("{word:?} names no goal: give its number, as `goals` shows it"))?;
    Ok(deduxit::rules::GoalId::new(index))
}

impl Commands for Session {
    fn command(&mut self, words: &[&str]) -> Result<String> {
        let (command, rest) = words.split_first().map_or(("", &[][..]), |(c, r)| (*c, r));
        Ok(match command {
            "" => String::new(),
            "help" => HELP.to_owned(),
            "goals" => self.goals(),
            "rules" => {
                let rules = self.state.rules(goal(rest.first())?)?;
                if rules.is_empty() {
                    "no rule applies to the goal".to_owned()
                } else {
                    rules
                        .iter()
                        .map(|r| {
                            let mut line = r.rule.clone();
                            if !r.at.is_empty() {
                                let at: Vec<String> = r.at.iter().map(u32::to_string).collect();
                                line.push_str(&format!(" at {}", at.join(" ")));
                            }
                            if !r.needs.is_empty() {
                                line.push_str(&format!(", needs {}", r.needs.join(", ")));
                            }
                            line
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                }
            }
            "apply" => {
                let target = goal(rest.first())?;
                let rule = rest.get(1).context("which rule? see `rules`")?;
                let mut at = Vec::new();
                let mut given = Vec::new();
                for word in &rest[2..] {
                    if let Some((name, text)) = word.split_once('=') {
                        given.push((name, text));
                    } else {
                        at.push(word.parse::<u32>().with_context(|| {
                            format!("{word:?} is neither a member's number nor NAME=TEXT")
                        })?);
                    }
                }
                let opened = self
                    .state
                    .apply(target, &Apply::new(rule).at(&at).given(&given))?;
                self.opened(&opened)
            }
            "derive" => {
                let target = goal(rest.first())?;
                let rule = rest.get(1).context("which rule? see `rules` and `help`")?;
                let mut lines = Vec::new();
                let mut given = Vec::new();
                for word in &rest[2..] {
                    if let Some((name, text)) = word.split_once('=') {
                        given.push((name, text));
                    } else {
                        lines.push(line(word)?);
                    }
                }
                let derived = self
                    .state
                    .derive(target, &Derive::new(rule).lines(&lines).given(&given))?;
                let calculus = self.state.calculus();
                let sequent = self
                    .state
                    .lines(target)
                    .into_iter()
                    .find(|(l, _)| *l == derived)
                    .map(|(_, s)| crate::calculus::sequent_of(calculus, self.state.terms(), s))
                    .unwrap_or_default();
                format!("line {}: {sequent}", derived.number())
            }
            "assume" => {
                let target = goal(rest.first())?;
                match rest.get(1) {
                    Some(word) => self.state.cite(target, line(word)?)?,
                    None => self.state.assume(target)?,
                }
                self.opened(&[])
            }
            "undo" => match self.state.undo() {
                Some(Undone::Reopened(goal)) => format!("reopened {}", self.goal_line(goal)),
                Some(Undone::Removed(line)) => format!("removed line {}", line.number()),
                Some(_) => "undone".to_owned(),
                None => "nothing to undo".to_owned(),
            },
            "show" => self.show()?,
            "proof" => {
                let derivation = self.state.finished(|_| crate::interrupted())?;
                let calculus = self.state.calculus();
                let root = derivation.root().expect("a finished derivation has a root");
                let line = format!(
                    "valid in the calculus given ({}): {}",
                    calculus.name(),
                    crate::calculus::sequent_of(
                        calculus,
                        derivation.terms(),
                        &zones(&derivation, root)
                    )
                );
                match rest {
                    [] => format!("{line}\n{}", crate::calculus::lines(&derivation, calculus)),
                    [path] => {
                        io::write(Some(Path::new(path)), &serde_json::to_string(&derivation)?)?;
                        format!("{line}; written to {path}")
                    }
                    _ => bail!("proof writes one file, in quotes if its name has blanks"),
                }
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
                let Saved::Rules(state) = load(Path::new(path), &self.limits)? else {
                    bail!(
                        "{path} holds a session of another kind: `deduxit interact --state {path}` starts it"
                    );
                };
                // The exit status answers for the session's own question.
                if statement(&state) != statement(&self.state) {
                    bail!(
                        "{path} holds a session of another calculus, sequent or assumptions; \
                         `deduxit interact --state {path}` starts it"
                    );
                }
                self.state = *state;
                format!("session loaded from {path}\n{}", self.goals())
            }
            other => bail!("unknown command {other:?}; `help` lists the commands"),
        })
    }

    fn finished(&self) -> bool {
        self.state.is_finished() && self.state.finished(|_| false).is_ok()
    }
}

/// Returns what a session derives, as text: its calculus, its sequent and
/// its assumptions.
fn statement(state: &Interactive) -> String {
    let calculus = state.calculus();
    let terms = state.terms();
    let mut text = format!(
        "{}: {}",
        calculus.name(),
        crate::calculus::sequent_of(calculus, terms, state.goal(GoalId::new(0)))
    );
    for hypothesis in state.assumptions() {
        text.push_str(" | ");
        text.push_str(&crate::calculus::sequent_of(calculus, terms, hypothesis));
    }
    text
}

/// Returns the members of each zone of a node.
fn zones(
    derivation: &deduxit::rules::Derivation,
    node: deduxit::rules::NodeRef<'_>,
) -> Vec<Vec<deduxit::rules::TermId>> {
    (0..derivation.zones())
        .map(|zone| node.zone(zone).to_vec())
        .collect()
}
