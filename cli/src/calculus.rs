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
use crate::show::{Prefixed, Rendered, Show, Shown, note, render};
use crate::{Status, interrupted};
use anyhow::{Context, Result, bail};
use deduxit::export::{Drawable, latex, svg, typst};
use deduxit::linear::proofs::Layout;
use deduxit::rules::{
    self, Calculus, Derivation, Drawing, Fitch, NodeRef, ScopeId, Step, TermId, Terms,
};
use deduxit::{Error, Limits};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::Path;

/// The calculi this crate ships, by the name a derivation names and the
/// flag takes, any case.
const BUILT_IN: [(&str, &str); 13] = [
    ("LK", rules::LK),
    ("LJ", rules::LJ),
    ("LM", rules::LM),
    ("LL", rules::LL),
    ("tableau", rules::TABLEAU),
    ("NJ", rules::NJ),
    ("NK", rules::NK),
    ("NM", rules::NM),
    ("NK-DNE", rules::NK_DNE),
    ("NK-LEM", rules::NK_LEM),
    ("NJ-defined", rules::NJ_DEFINED),
    ("NK-defined", rules::NK_DEFINED),
    ("NM-defined", rules::NM_DEFINED),
];

/// The bytes a line of the node list takes beyond its sequent: the
/// number, the indent, the rule and its premises, for the list's bound.
const LINE_BYTES: u64 = 48;

/// Returns the labelled calculus of a modal system its name names:
/// `G3` and the system's name, any case (`G3S4`, `g3kd45`).
fn modal(name: &str, limits: &Limits) -> Option<Result<Calculus>> {
    let system = name
        .get(..2)
        .filter(|g3| g3.eq_ignore_ascii_case("G3"))
        .and_then(|_| name[2..].parse().ok())?;
    Some(deduxit::modal::calculus(system, limits).map_err(Into::into))
}

/// Returns the calculus `--calculus` names, a built-in one by name or a
/// rule file, or with no flag the built-in calculus the derivation
/// names (a modal system's `G3…` among them).
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
    if flag.is_none()
        && let Some(calculus) = modal(derivation.calculus(), limits)
    {
        return calculus;
    }
    let (text, what) = match flag {
        Some(path) => return named(path, limits),
        None => match built_in(derivation.calculus()) {
            Some(text) => (text.to_owned(), "the built-in calculus".to_owned()),
            None => bail!(
                "the derivation is in the calculus {:?}, which is not built in: give its rule \
                 file with --calculus FILE",
                derivation.calculus()
            ),
        },
    };
    read(&text, &what, limits)
}

/// Returns the calculus `--calculus` names: a built-in one by name, in
/// any case, or a rule file.
///
/// # Errors
///
/// A file that cannot be read or is no calculus.
pub(crate) fn named(path: &Path, limits: &Limits) -> Result<Calculus> {
    if let Some(calculus) = path.to_str().and_then(|name| modal(name, limits)) {
        return calculus;
    }
    let built_in = path.to_str().and_then(|name| {
        BUILT_IN
            .iter()
            .find(|(known, _)| known.eq_ignore_ascii_case(name))
            .map(|&(_, text)| text)
    });
    match built_in {
        Some(text) => read(text, "the built-in calculus", limits),
        None => read(
            &io::read_configuration(path, "rule file")?,
            &format!("the rule file {}", path.display()),
            limits,
        ),
    }
}

/// Reads a calculus from `text`, a rule file, `what` naming it in an
/// error.
fn read(text: &str, what: &str, limits: &Limits) -> Result<Calculus> {
    let mut document = serde_json::Deserializer::from_str(text);
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
    let drawn = matches!(show.format, Format::Latex | Format::Typst | Format::Svg);
    if !(drawn || matches!(show.format, Format::Text | Format::Json))
        || show.net
        || show.lambda.is_some()
    {
        bail!(
            "a derivation in a calculus given as data is shown as text, json, latex, typst or \
             svg: give one of those to --format, and leave out --net and --lambda"
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
            let sequent = sequent_text(derivation, calculus, root, bound(show, &show.limits));
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
            system: None,
            error: result.as_ref().err().cloned(),
            message: result.is_err().then_some(line),
            lambda: None,
            failing: None,
        };
        serde_json::to_writer(out.stream(), &checked)?;
        return Ok((status, true));
    }
    if drawn {
        return drawing(
            derivation,
            calculus,
            show,
            quiet,
            &line,
            status,
            result.is_ok(),
            &over,
            out,
        );
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
    if show.view.layout == Layout::Fitch {
        match Fitch::new(calculus, derivation, &show.styles.fitch, &show.limits) {
            Ok(fitch) => {
                for row in fitch.to_string().lines() {
                    lines.write(out, row)?;
                }
            }
            Err(error) => lines.write(out, &left_out(&error))?,
        }
        return Ok((status, true));
    }
    // The list is bounded as a drawing is: a line per node, each sequent
    // within --abbreviate.
    let width = bound(show, &show.limits) as u64;
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
        lines.write(
            out,
            &node_line(derivation, calculus, node, bound(show, &show.limits)),
        )?;
    }
    Ok((status, lines.written))
}

/// Returns the line that says why a derivation's drawing or Fitch proof
/// is left out.
fn left_out(error: &Error) -> String {
    let flag = match error.setting() {
        Some("limits.derivation_bytes") => "; raise --derivation-limit",
        Some("limits.work") => "; raise --work-limit",
        _ => "",
    };
    format!("the derivation is left out: {error}{flag}")
}

/// Writes a checked derivation in a drawn format (latex, typst, svg), as
/// a tree or a Fitch proof by `view.layout`: the verdict as the format's
/// comment, then the derivation. Where there is nothing to draw (the
/// derivation is invalid, or `-q` or `--tree never` asks for none) or it
/// is left out, the verdict and the reason go to standard error and the
/// output holds nothing. `over` says when the time limit or Ctrl-C ends
/// the drawing. Returns the status and whether anything was written.
#[expect(
    clippy::too_many_arguments,
    reason = "the check's parts, each needed once"
)]
fn drawing(
    derivation: &Derivation,
    calculus: &Calculus,
    show: &Show,
    quiet: bool,
    verdict: &str,
    status: Status,
    valid: bool,
    over: &dyn Fn() -> bool,
    out: &mut io::Output,
) -> Result<(Status, bool)> {
    let say = |line: &str| {
        if show.verdict || !valid {
            eprintln!("{line}");
        }
    };
    if !valid || quiet || show.tree == Tree::Never {
        say(verdict);
        return Ok((status, false));
    }
    let write = |drawable: Drawable<'_>| -> Result<String, Error> {
        let mut text = String::new();
        let styles = &show.styles;
        match show.format {
            Format::Latex => latex::write(drawable, &styles.latex, &mut text, |_| over())?,
            Format::Typst => typst::write(drawable, &styles.typst, &mut text, |_| over())?,
            _ => svg::write(drawable, &styles.svg, &mut text, |_| over())?,
        }
        Ok(text)
    };
    let made = match show.view.layout {
        Layout::Fitch => Fitch::new(calculus, derivation, &show.styles.fitch, &show.limits)
            .and_then(|fitch| write(Drawable::from(&fitch))),
        _ => Drawing::new(calculus, derivation, &show.limits)
            .and_then(|tree| write(Drawable::from(&tree))),
    };
    match made {
        Ok(text) => {
            if show.verdict {
                writeln!(out.stream(), "{}", note(show.format, verdict))?;
            }
            out.stream().write_all(text.as_bytes())?;
            Ok((status, true))
        }
        Err(error) => {
            say(verdict);
            eprintln!("{}", left_out(&error));
            Ok((status, false))
        }
    }
}

/// Writes a checked derivation as `show` asks, after `prefix`: its nodes
/// as a list in text, drawn as a tree in LaTeX, Typst and SVG, rendered
/// in PNG and PDF; Rocq leaves it out with the reason, as does a bound.
///
/// # Errors
///
/// An output that cannot be written.
pub(crate) fn shown(
    derivation: &Derivation,
    calculus: &Calculus,
    show: &Show,
    prefix: Option<&str>,
    out: &mut io::Output,
    halt: &dyn Fn() -> bool,
) -> Result<Shown> {
    let write = |drawable: Drawable<'_>, format: Format| -> Result<String, Error> {
        let mut text = String::new();
        let styles = &show.styles;
        match format {
            Format::Latex => latex::write(drawable, &styles.latex, &mut text, |_| halt())?,
            Format::Typst => typst::write(drawable, &styles.typst, &mut text, |_| halt())?,
            _ => svg::write(drawable, &styles.svg, &mut text, |_| halt())?,
        }
        Ok(text)
    };
    match show.format {
        Format::Text => {
            let width = bound(show, &show.limits) as u64;
            let estimate =
                (derivation.len() as u64).saturating_mul(width.saturating_add(LINE_BYTES));
            if let Some(limit) = show.limits.derivation_bytes
                && estimate > limit
            {
                return Ok(Shown::LeftOut(format!(
                    "the list of the derivation's {} is left out: it may take {estimate} \
                     bytes, past --derivation-limit; --format json writes it",
                    counted(derivation.len(), "node", "nodes")
                )));
            }
            let mut writer = Prefixed { out, prefix };
            for (n, node) in derivation.nodes().enumerate() {
                if halt() {
                    return Ok(Shown::Cut("the list is cut short".to_owned()));
                }
                if n > 0 {
                    writer.write_str("\n")?;
                }
                writer.write_str(&node_line(
                    derivation,
                    calculus,
                    node,
                    bound(show, &show.limits),
                ))?;
            }
            Ok(Shown::Written)
        }
        Format::Latex | Format::Typst | Format::Svg | Format::Png | Format::Pdf => {
            let drawn = Drawing::new(calculus, derivation, &show.limits).and_then(|tree| {
                let format = if show.format.is_binary() {
                    Format::Svg
                } else {
                    show.format
                };
                write(Drawable::from(&tree), format)
            });
            let text = match drawn {
                Ok(text) => text,
                Err(error) => return Ok(Shown::LeftOut(left_out(&error))),
            };
            if !show.format.is_binary() {
                write!(out.stream(), "{}{text}", prefix.unwrap_or(""))?;
                return Ok(Shown::Written);
            }
            Ok(
                match render(
                    text,
                    show.format,
                    &show.styles,
                    show.limits.memory_bytes,
                    halt,
                )? {
                    Rendered::Bytes(bytes) => Shown::Rendered(bytes),
                    Rendered::Refused(line) => Shown::LeftOut(line),
                    Rendered::Stopped => {
                        Shown::LeftOut("the derivation is not drawn: it was stopped".to_owned())
                    }
                },
            )
        }
        _ => Ok(Shown::LeftOut(
            "no certificate: a derivation in a calculus given as data has no Rocq certificate"
                .to_owned(),
        )),
    }
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
    limit: usize,
) -> String {
    let mut depth = 0usize;
    let mut scope = node.scope();
    while let Some(parent) = derivation.parent(scope) {
        depth += 1;
        scope = parent;
    }
    debug_assert!(scope == ScopeId::ROOT, "every scope lies in the root scope");
    let mut line = format!("{:>3}. {}", node.id().index(), "  ".repeat(depth));
    line.push_str(&sequent_text(derivation, calculus, node, limit));
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
    limit: usize,
) -> String {
    let text = calculus
        .show_sequent(derivation, node.id())
        .limit(limit)
        .to_string();
    if text.is_empty() {
        "the empty sequent".to_owned()
    } else {
        text
    }
}

/// Returns the most characters a sequent is written with: `--abbreviate`,
/// or with `none` the derivation's bound. A term shares its parts, so a
/// sequent of a few bytes may unfold far past any bound; without one its
/// text would take as long as its length, past the time limit.
fn bound(show: &Show, limits: &Limits) -> usize {
    show.abbreviate.unwrap_or_else(|| {
        let bytes = limits
            .derivation_bytes
            .unwrap_or(Limits::DEFAULT_DERIVATION_BYTES);
        usize::try_from(bytes).unwrap_or(usize::MAX)
    })
}

/// The most characters of a sequent a session's line writes.
const SESSION_LINE: usize = 200;

/// Returns a sequent given as each zone's members as text, within a
/// session's bound, or `the empty sequent`.
pub(crate) fn sequent_of(calculus: &Calculus, terms: &Terms, zones: &[Vec<TermId>]) -> String {
    let text = calculus
        .show_zones(terms, zones)
        .limit(SESSION_LINE)
        .to_string();
    if text.is_empty() {
        "the empty sequent".to_owned()
    } else {
        text
    }
}

/// Returns a derivation's nodes, a line each, as `check` lists them,
/// each sequent within a session's bound.
pub(crate) fn lines(derivation: &Derivation, calculus: &Calculus) -> String {
    derivation
        .nodes()
        .map(|node| node_line(derivation, calculus, node, SESSION_LINE))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns `n` with the noun that counts it.
fn counted(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}
