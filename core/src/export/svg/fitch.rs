// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Fitch proofs as numbered lines. Each line's number stands right-aligned
//! at the left, then a vertical line for the main proof and one for every
//! subproof the line lies in, the formula, and in a column of its own the
//! justification: the rule and the lines it cites. A short bar closes a
//! proof's or subproof's hypotheses, under the last of them.

use super::font::{DEPTH, HEIGHT};
use super::{Escaping, NOTATION, PLAIN, Style, backdrop, escaped, head, label, run, text};
use crate::Error;
use crate::export::notation::flush;
use crate::rules::Fitch;
use crate::rules::fitch::Cite;
use std::fmt::Write;

/// Writes a Fitch proof as an SVG document into `out`, titled with its
/// conclusion in plain text, and asks `stop` after each line.
pub(super) fn draw(
    fitch: &Fitch<'_>,
    style: &Style,
    out: &mut impl Write,
    mut stop: impl FnMut() -> bool,
) -> Result<(), Error> {
    let font = &style.font;
    let pitch = i64::from(style.line_height);
    let label_size = i64::from(style.label_size);
    let (margin, stroke) = (i64::from(style.margin), i64::from(style.stroke_width));
    // The proofs' lines stand half a gap apart, and the formula half of
    // that right of its innermost line.
    let indent = i64::from(style.premise_gap) / 2;
    let lines = fitch.lines();
    let blocks = fitch.blocks();

    // Each line's number, formula and justification, laid out once to
    // measure; they are laid out again when written.
    let number = |n: usize| run(&n.to_string(), label_size, font);
    let mut formula = String::new();
    let mut lay_formula = |line| {
        formula.clear();
        fitch.formula(line, &NOTATION, &mut formula);
        run(&formula, 1000, font)
    };
    let justification = |line| {
        let mut markup = String::new();
        match fitch.rule(line) {
            Some(rule) => {
                markup.push_str(&label(&rule));
                if !line.cites.is_empty() {
                    markup.push(' ');
                    // A citation's text is no markup.
                    let _ = Fitch::write_cites(line, "–", &mut markup);
                }
            }
            None => markup.push_str(fitch.note(line)),
        }
        run(&markup, label_size, font)
    };
    let numbers = (1..=lines.len())
        .map(|n| number(n).width)
        .max()
        .unwrap_or(0);
    let bar = |depth: u32| margin + numbers + indent + i64::from(depth) * indent;
    let start = |depth: u32| bar(depth) + indent / 2;
    let mut widths = Vec::with_capacity(lines.len());
    for line in lines {
        widths.push(lay_formula(line).width);
        if stop() {
            return Err(stopped());
        }
    }
    let column = lines
        .iter()
        .zip(&widths)
        .map(|(line, width)| start(line.depth) + width)
        .max()
        .unwrap_or(0)
        + i64::from(style.premise_gap);
    let notes = lines
        .iter()
        .map(|line| justification(line).width)
        .max()
        .unwrap_or(0);
    let baseline = |index: u32| margin + HEIGHT + i64::from(index) * pitch;
    let last = lines.len() as u32 - 1;
    let size = (column + notes + margin, baseline(last) + DEPTH + margin);

    let mut title = String::new();
    if let Some(conclusion) = lines.last() {
        fitch.formula(conclusion, &PLAIN, &mut title);
    }
    head(out, style, &title, size)?;
    if style.description {
        out.write_str("<desc>")?;
        describe(fitch, &mut Escaping(out))?;
        out.write_str("</desc>\n")?;
    }
    backdrop(out, style, size)?;

    // The proofs' lines and the bars under their hypotheses.
    writeln!(
        out,
        "<g fill=\"none\" stroke=\"{}\" stroke-width=\"{stroke}\">",
        escaped(&style.line)
    )?;
    let mut buffer = String::new();
    for block in blocks {
        let x = bar(block.depth);
        let top = baseline(block.first) - HEIGHT;
        let bottom = baseline(block.last) + DEPTH;
        writeln!(buffer, r#"<path d="M{x} {top}V{bottom}"/>"#)?;
        if block.hypotheses > 0 {
            let first = block.first as usize;
            let reach = widths[first..first + block.hypotheses as usize]
                .iter()
                .max()
                .copied()
                .unwrap_or(0);
            let y = baseline(block.first + block.hypotheses - 1) + DEPTH;
            writeln!(
                buffer,
                r#"<path d="M{x} {y}H{}"/>"#,
                start(block.depth) + reach
            )?;
        }
        flush(out, &mut buffer, &mut stop)?;
    }
    out.write_str("</g>\n")?;

    // The numbers, formulas and justifications.
    let size_attribute = format!(r#" font-size="{label_size}""#);
    for (index, line) in lines.iter().enumerate() {
        let y = baseline(index as u32);
        let n = number(index + 1);
        text(
            &mut buffer,
            margin + numbers - n.width,
            y,
            &n,
            &size_attribute,
            None,
        );
        let laid = lay_formula(line);
        let id = format!(r#" id="l{}""#, index + 1);
        text(&mut buffer, start(line.depth), y, &laid, &id, None);
        let note = justification(line);
        if !note.pieces.is_empty() {
            text(&mut buffer, column, y, &note, &size_attribute, None);
        }
        flush(out, &mut buffer, &mut stop)?;
    }
    out.write_str("</svg>")?;
    Ok(())
}

/// The refusal of a drawing its stop ended.
fn stopped() -> Error {
    Error::Refused(crate::limits::Refusal::Stopped {
        phase: crate::limits::Phase::Write,
    })
}

/// Writes what a screen reader reads of a Fitch proof: a sentence per
/// line, its number, its formula in plain text, and what justifies it.
fn describe(fitch: &Fitch<'_>, out: &mut impl Write) -> std::fmt::Result {
    let mut formula = String::new();
    for (index, line) in fitch.lines().iter().enumerate() {
        if index > 0 {
            out.write_char('\n')?;
        }
        formula.clear();
        fitch.formula(line, &PLAIN, &mut formula);
        write!(out, "Line {}, {formula}: ", index + 1)?;
        let hypothesis = fitch.is_hypothesis(line);
        match fitch.rule(line) {
            _ if hypothesis && line.depth == 0 => out.write_str("an assumption")?,
            _ if hypothesis && line.opens > 0 => {
                out.write_str("a hypothesis, which opens a subproof")?;
            }
            _ if hypothesis => out.write_str("a hypothesis")?,
            None if line.opens > 0 => out.write_str("open, the first line of a subproof")?,
            None => out.write_str("open")?,
            Some(_) => {
                out.write_str("by ")?;
                let mut name = String::new();
                fitch.plain_rule(line, &mut name)?;
                out.write_str(&name)?;
                for (k, cite) in line.cites.iter().enumerate() {
                    out.write_str(match k {
                        0 => " from ",
                        _ if k + 1 == line.cites.len() => " and ",
                        _ => ", ",
                    })?;
                    match *cite {
                        Cite::Line(n) => write!(out, "line {n}")?,
                        Cite::Range(a, b) => {
                            write!(out, "the subproof of lines {a} to {b}")?;
                        }
                    }
                }
            }
        }
        out.write_char('.')?;
    }
    Ok(())
}
