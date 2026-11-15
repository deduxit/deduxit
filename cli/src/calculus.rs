// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! `check` of a derivation in a calculus given as data: the rule file read
//! from `--calculus` or named by the derivation, the check, the verdict
//! and the derivation's nodes as a list.

use crate::argument_parsing::{Format, Tree};
use crate::check::{CHECKED_LEVEL, Checked};
use crate::io;
use crate::limit::Deadline;
use crate::prove::{Stop, stopped};
use crate::show::Show;
use crate::{Status, interrupted};
use anyhow::{Context, Result, bail};
use deduxit::rules::{self, Calculus, Derivation, NodeRef, ScopeId, Step};
use deduxit::{Error, Limits};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::Path;

/// The calculi this crate ships, by the name a derivation names and the
/// flag takes, any case.
const BUILT_IN: [(&str, &str); 5] = [
    ("LK", rules::LK),
    ("LJ", rules::LJ),
    ("LM", rules::LM),
    ("LL", rules::LL),
    ("tableau", rules::TABLEAU),
];

/// The bytes a line of the node list takes beyond its sequent: the
/// number, the indent, the rule and its premises, for the list's bound.
const LINE_BYTES: u64 = 48;

/// Returns the calculus `--calculus` names, a built-in one by name or a
/// rule file, or with no flag the built-in calculus the derivation
/// names.
///
/// # Errors
///
/// A file that cannot be read or is no calculus, or a derivation that
/// names a calculus this crate does not ship and no flag.
pub(crate) fn calculus(
    flag: Option<&Path>,
    derivation: &Derivation,
    limits: &Limits,
) -> Result<Calculus> {
    let built_in = |name: &str| {
        BUILT_IN
            .iter()
            .find(|(known, _)| known.eq_ignore_ascii_case(name))
            .map(|&(_, text)| text)
    };
    let (text, what) = match flag {
        Some(path) => match path.to_str().and_then(built_in) {
            Some(text) => (text.to_owned(), "the built-in calculus".to_owned()),
            None => (
                io::read_configuration(path, "rule file")?,
                format!("the rule file {}", path.display()),
            ),
        },
        None => match built_in(derivation.calculus()) {
            Some(text) => (text.to_owned(), "the built-in calculus".to_owned()),
            None => bail!(
                "the derivation is in the calculus {:?}, which is not built in: give its rule \
                 file with --calculus FILE",
                derivation.calculus()
            ),
        },
    };
    let mut document = serde_json::Deserializer::from_str(&text);
    let calculus: Calculus = deduxit::wire::upgrade(&mut document, limits)
        .with_context(|| format!("cannot read {what}"))?;
    document
        .end()
        .with_context(|| format!("{what} is no calculus in JSON: more follows it"))?;
    Ok(calculus)
}

/// Checks a derivation against a calculus given as data and writes the
/// verdict, then the derivation's nodes as a list, or the verdict as
/// JSON; returns the verdict's status (valid 0, invalid 1, unknown 3) and
/// whether the output is to be finished.
///
/// # Errors
///
/// A format other than text and JSON, or an output that cannot be
/// written.
pub(crate) fn check_rules(
    derivation: &Derivation,
    calculus: &Calculus,
    show: &Show,
    quiet: bool,
    deadline: &Deadline,
    out: &mut io::Output,
) -> Result<(Status, bool)> {
    if !matches!(show.format, Format::Text | Format::Json) || show.net || show.lambda.is_some() {
        bail!(
            "a derivation in a calculus given as data is shown as text or JSON: leave out \
             --format, --net and --lambda"
        );
    }
    let over = || interrupted() || deadline.passed() || io::closed();
    let result = derivation.check(calculus, &show.limits, |_| over());
    let status = match &result {
        Ok(()) => Status::Yes,
        Err(error) if error.is_refusal() => Status::Unknown,
        Err(_) => Status::No,
    };
    let name = calculus.name();
    let line = match &result {
        Ok(()) => {
            // A root exists: the check refuses a derivation without.
            let root = derivation.root().expect("a checked derivation has a root");
            let sequent = sequent_text(derivation, calculus, root, show);
            let nodes = counted(derivation.len(), "node", "nodes");
            match derivation.assumptions().count() {
                0 => format!(
                    "valid in the calculus given ({name}): {sequent}, a derivation of {nodes}"
                ),
                n => format!(
                    "valid in the calculus given ({name}): {sequent}, a derivation of {nodes} \
                     from {}",
                    counted(n, "assumption", "assumptions")
                ),
            }
        }
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
                "unknown: the derivation in the calculus {name} is not checked: {error}{advice}"
            )
        }
        Err(Error::InvalidDerivation { reason, .. }) => {
            format!("invalid in the calculus given ({name}): {reason}")
        }
        Err(error) => format!("invalid in the calculus given ({name}): {error}"),
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
            calculus: Some(name.to_owned()),
            error: result.as_ref().err().cloned(),
            message: result.is_err().then_some(line),
            lambda: None,
            failing: None,
        };
        serde_json::to_writer(out.stream(), &checked)?;
        return Ok((status, true));
    }
    // Lines are written apart, the last without its newline, which
    // finishing the output adds.
    let mut lines = Lines { written: false };
    if show.verdict || result.is_err() {
        lines.write(out, &line)?;
    }
    if result.is_err() || quiet || show.tree == Tree::Never {
        return Ok((status, lines.written));
    }
    // The list is bounded as a drawing is: a line per node, each sequent
    // within --abbreviate.
    let width = show.abbreviate.map_or(u64::MAX, |a| a as u64);
    let estimate = (derivation.len() as u64).saturating_mul(width.saturating_add(LINE_BYTES));
    if let Some(limit) = show.limits.derivation_bytes
        && estimate > limit
    {
        let left_out = format!(
            "the list of the derivation's {} is left out: it may take {estimate} bytes, past \
             --derivation-limit; --format json writes the verdict alone",
            counted(derivation.len(), "node", "nodes")
        );
        lines.write(out, &left_out)?;
        return Ok((status, true));
    }
    for node in derivation.nodes() {
        if over() {
            let cut = format!("(the list is cut short: {})", Stop::why(stopped(deadline)));
            lines.write(out, &cut)?;
            break;
        }
        lines.write(out, &node_line(derivation, calculus, node, show))?;
    }
    Ok((status, lines.written))
}

/// Lines written to an output, each but the first after a newline.
struct Lines {
    /// Whether a line was written.
    written: bool,
}

impl Lines {
    /// Writes `line`, after a newline unless it is the first.
    fn write(&mut self, out: &mut io::Output, line: &str) -> Result<()> {
        if std::mem::replace(&mut self.written, true) {
            out.stream().write_all(b"\n")?;
        }
        out.stream().write_all(line.as_bytes())?;
        Ok(())
    }
}

/// Returns a node's line: its number, indented by the depth of its
/// scope, its sequent, and what justifies it.
fn node_line(
    derivation: &Derivation,
    calculus: &Calculus,
    node: NodeRef<'_>,
    show: &Show,
) -> String {
    let mut depth = 0usize;
    let mut scope = node.scope();
    while let Some(parent) = derivation.parent(scope) {
        depth += 1;
        scope = parent;
    }
    debug_assert!(scope == ScopeId::ROOT, "every scope lies in the root scope");
    let mut line = format!("{:>3}. {}", node.id().index(), "  ".repeat(depth));
    line.push_str(&sequent_text(derivation, calculus, node, show));
    match node.step() {
        Step::Rule(rule) => {
            write!(line, "    {rule}").expect("a string takes any text");
            for (place, premise) in node.premises().iter().enumerate() {
                let separator = if place == 0 { " " } else { ", " };
                write!(line, "{separator}{}", premise.index()).expect("a string takes any text");
            }
        }
        Step::Hypothesis => line.push_str("    hypothesis"),
        Step::Open => line.push_str("    open"),
    }
    line
}

/// Returns a node's sequent as text within `--abbreviate`, or `the empty
/// sequent` for one that writes nothing.
fn sequent_text(
    derivation: &Derivation,
    calculus: &Calculus,
    node: NodeRef<'_>,
    show: &Show,
) -> String {
    let text = calculus
        .show_sequent(derivation, node.id())
        .limit(show.abbreviate.unwrap_or(usize::MAX))
        .to_string();
    if text.is_empty() {
        "the empty sequent".to_owned()
    } else {
        text
    }
}

/// Returns `n` with the noun that counts it.
fn counted(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}
