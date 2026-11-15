// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The drawing of a Kripke model: each world a box with the atoms it
//! forces, its number beside it, and an arrow up to each world directly
//! above it. The model's world, the one a countermodel refutes the
//! sequent at, has its box in the highlight colour.
//!
//! The worlds stand in rows: a world with no arrow into it in the bottom
//! row, and every other one in the row above the highest world with an
//! arrow to it, so that every arrow points up. Within a row the worlds
//! are ordered by where the worlds below them stand, which keeps most
//! arrows from crossing. An arrow past a row bows out to the right.

use super::font::{DEPTH, HEIGHT};
use super::net::arrowhead;
use super::{Run, Style, document, escaped, italic, run, text, within};
use crate::Error;
use crate::kripke::Model;
use crate::limits::{Limits, Phase, Refusal};
use std::fmt::Write;

/// The bytes a world's drawing is estimated at, besides its atoms: its
/// box, its number and its sentence of the description.
const WORLD: u64 = 600;

/// The bytes an atom's drawing is estimated at, per byte of its name:
/// an italic letter takes four bytes, in the box and again in the
/// description.
const ATOM: u64 = 12;

/// The bytes an arrow's drawing is estimated at: its line, its head and
/// its words in the description.
const ARROW: u64 = 300;

/// The space between a box's text and its outline, sideways.
const PAD: i64 = 300;

/// The space between a box's text and its outline, up and down.
const PAD_UP: i64 = 200;

/// The least width of a box's text, so that a world that forces nothing
/// still has a box to point at.
const LEAST: i64 = 600;

/// The space between a world's number and its box.
const NUMBER_GAP: i64 = 150;

/// A world laid out.
struct Placed {
    /// The left end of its number.
    x: i64,
    /// The top of its box.
    top: i64,
    /// Its number, at the label size.
    number: Run,
    /// What it forces, at the size of formula text.
    forced: Run,
    /// The width of its box.
    width: i64,
}

impl Placed {
    /// Returns the left end of the box.
    const fn left(&self) -> i64 {
        self.x + self.number.width + NUMBER_GAP
    }

    /// Returns the middle of the box, across.
    const fn middle(&self) -> i64 {
        self.left() + self.width / 2
    }

    /// Returns the width of the world's number and box together.
    const fn span(&self) -> i64 {
        self.number.width + NUMBER_GAP + self.width
    }
}

/// Returns a Kripke model drawn as an SVG document titled `title`, whose
/// description reads each world and ends with `last`, a sentence on the
/// model's world. The model is taken as it is: its arrows must go round
/// nowhere, which the caller's check of the model ensures.
///
/// # Errors
///
/// - [`Refusal::Output`] for a drawing estimated past
///   `limits.derivation_bytes`, before anything is laid out.
/// - [`Refusal::Stopped`] where `stop` asked, which is once per world.
/// - [`Error::InvalidOption`] for a style [`Style::check`] refuses.
pub(super) fn draw(
    model: &Model,
    title: &str,
    last: &str,
    style: &Style,
    limits: &Limits,
    mut stop: impl FnMut() -> bool,
) -> Result<String, Error> {
    style.check()?;
    let worlds = &model.worlds;
    within(limits, || {
        worlds.iter().fold(
            (title.len() as u64)
                .saturating_add(last.len() as u64)
                .saturating_mul(2)
                .saturating_add(1000),
            |sum, world| {
                let atoms = world.atoms.iter().fold(0u64, |sum, a| {
                    sum.saturating_add((a.len() as u64 + 2).saturating_mul(ATOM))
                });
                sum.saturating_add(WORLD)
                    .saturating_add(atoms)
                    .saturating_add((world.arrows.len() as u64).saturating_mul(ARROW))
            },
        )
    })?;
    let halt = |stop: &mut dyn FnMut() -> bool| -> Result<(), Error> {
        if stop() {
            Err(Refusal::Stopped {
                phase: Phase::Write,
            }
            .into())
        } else {
            Ok(())
        }
    };
    let n = worlds.len();
    // Each world's arrows once, in order.
    let above: Vec<Vec<usize>> = worlds
        .iter()
        .map(|world| {
            let mut targets: Vec<usize> = world.arrows.iter().map(|&v| v as usize).collect();
            targets.sort_unstable();
            targets.dedup();
            targets
        })
        .collect();
    // The rows, by Kahn's order: a world after every world below it.
    let mut pending = vec![0usize; n];
    for targets in &above {
        for &v in targets {
            pending[v] += 1;
        }
    }
    let mut ready: Vec<usize> = (0..n).filter(|&w| pending[w] == 0).rev().collect();
    let mut row = vec![0usize; n];
    let mut order = Vec::with_capacity(n);
    while let Some(w) = ready.pop() {
        halt(&mut stop)?;
        order.push(w);
        for &v in &above[w] {
            row[v] = row[v].max(row[w] + 1);
            pending[v] -= 1;
            if pending[v] == 0 {
                ready.push(v);
            }
        }
    }
    let rows = row.iter().max().map_or(0, |&r| r + 1);
    // Within a row, by the mean place of the worlds below, then by number.
    let mut below: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (w, targets) in above.iter().enumerate() {
        for &v in targets {
            below[v].push(w);
        }
    }
    let mut members: Vec<Vec<usize>> = Vec::with_capacity(rows);
    let mut place = vec![0i64; n];
    for r in 0..rows {
        let mut keyed: Vec<(i64, usize)> = order
            .iter()
            .filter(|&&w| row[w] == r)
            .map(|&w| {
                let from = &below[w];
                let key = if from.is_empty() {
                    0
                } else {
                    from.iter().map(|&u| place[u]).sum::<i64>() * 1000 / from.len() as i64
                };
                (key, w)
            })
            .collect();
        keyed.sort_unstable();
        for (k, &(_, w)) in keyed.iter().enumerate() {
            place[w] = k as i64;
        }
        members.push(keyed.into_iter().map(|(_, w)| w).collect());
    }
    // Each world's runs and widths.
    let label = i64::from(style.label_size);
    let mut placed: Vec<Placed> = worlds
        .iter()
        .enumerate()
        .map(|(w, world)| {
            let mut forced = String::new();
            for (k, atom) in world.atoms.iter().enumerate() {
                if k > 0 {
                    forced.push_str(", ");
                }
                italic(&mut forced, atom).expect("a string takes the text");
            }
            if world.falsity {
                if !forced.is_empty() {
                    forced.push_str(", ");
                }
                forced.push('⊥');
            }
            let forced = run(&forced, 1000, &style.font);
            let width = forced.width.max(LEAST) + 2 * PAD;
            Placed {
                x: 0,
                top: 0,
                number: run(&format!("w\u{4}{w}\u{5}"), label, &style.font),
                forced,
                width,
            }
        })
        .collect();
    let margin = i64::from(style.margin);
    let gap = i64::from(style.premise_gap);
    let box_height = HEIGHT + DEPTH + 2 * PAD_UP;
    let rise = box_height + 2 * i64::from(style.line_height);
    let widths: Vec<i64> = members
        .iter()
        .map(|members| {
            members.iter().map(|&w| placed[w].span()).sum::<i64>()
                + gap * (members.len() as i64 - 1).max(0)
        })
        .collect();
    let width = widths.iter().copied().max().unwrap_or(0);
    let height = rows as i64 * box_height + (rows as i64 - 1).max(0) * (rise - box_height);
    for (r, members) in members.iter().enumerate() {
        let mut x = margin + (width - widths[r]) / 2;
        let top = margin + (rows - 1 - r) as i64 * rise;
        for &w in members {
            placed[w].x = x;
            placed[w].top = top;
            x += placed[w].span() + gap;
        }
    }
    // An arrow past a row bows out to the right by `reach`, so that it
    // does not run through the boxes between and is told from the arrows
    // of the rows it passes.
    let reach = placed.iter().map(|p| p.width).max().unwrap_or(0) / 2 + gap / 2;
    let skips = above
        .iter()
        .enumerate()
        .any(|(w, targets)| targets.iter().any(|&v| row[v] > row[w] + 1));
    let size = (
        width + 2 * margin + if skips { reach } else { 0 },
        height + 2 * margin,
    );
    let description = style.description.then(|| describe(model, &above, last));
    let stroke = i64::from(style.stroke_width);
    let mut drawn = Ok(());
    let svg = document(style, title, description.as_deref(), size, |out| {
        writeln!(
            out,
            r#"<g fill="none" stroke="{}" stroke-width="{stroke}">"#,
            escaped(&style.line)
        )
        .unwrap();
        for (w, targets) in above.iter().enumerate() {
            if drawn.is_ok() {
                drawn = halt(&mut stop);
            }
            let from = &placed[w];
            for &v in targets {
                let to = &placed[v];
                let start = (from.middle(), from.top);
                let tip = (to.middle(), to.top + box_height);
                // The curve's middle stands `reach` right of its ends.
                let towards = if row[v] > row[w] + 1 {
                    let control = (
                        2 * (start.0.max(tip.0) + reach) - (start.0 + tip.0) / 2,
                        i64::midpoint(start.1, tip.1),
                    );
                    writeln!(
                        out,
                        r#"<path d="M{} {}Q{} {} {} {}"/>"#,
                        start.0, start.1, control.0, control.1, tip.0, tip.1
                    )
                    .unwrap();
                    control
                } else {
                    writeln!(
                        out,
                        r#"<path d="M{} {}L{} {}"/>"#,
                        start.0, start.1, tip.0, tip.1
                    )
                    .unwrap();
                    start
                };
                arrowhead(
                    out,
                    tip,
                    ((tip.0 - towards.0) as f64, (tip.1 - towards.1) as f64),
                    i64::from(style.node_radius) / 2,
                );
            }
        }
        for (w, world) in placed.iter().enumerate() {
            let marked = w == model.world as usize;
            let colour = if marked {
                format!(
                    r#" stroke="{}" stroke-width="{}""#,
                    escaped(&style.highlight),
                    2 * stroke
                )
            } else {
                String::new()
            };
            writeln!(
                out,
                r#"<rect id="w{w}" x="{}" y="{}" width="{}" height="{box_height}" rx="{PAD_UP}"{colour}/>"#,
                world.left(),
                world.top,
                world.width,
            )
            .unwrap();
        }
        out.push_str("</g>\n");
        for world in &placed {
            let baseline = world.top + PAD_UP + HEIGHT;
            text(out, world.x, baseline, &world.number, "", None);
            let x = world.left() + (world.width - world.forced.width) / 2;
            text(out, x, baseline, &world.forced, "", None);
        }
    });
    drawn?;
    Ok(svg)
}

/// Returns the description of a model: its size, then a sentence per
/// world on what it forces and the worlds directly above it, then `last`.
fn describe(model: &Model, above: &[Vec<usize>], last: &str) -> String {
    let n = model.worlds.len();
    let mut out = format!(
        "A Kripke model of {n} world{}. An arrow points from a world to a world above it, \
         which forces everything the world below forces.",
        if n == 1 { "" } else { "s" }
    );
    for (w, world) in model.worlds.iter().enumerate() {
        write!(out, "\nWorld {w}").unwrap();
        if w == model.world as usize {
            out.push_str(", the model's world,");
        }
        let mut forced: Vec<&str> = world.atoms.iter().map(String::as_str).collect();
        if world.falsity {
            forced.push("⊥");
        }
        match forced.split_last() {
            None => out.push_str(" forces nothing"),
            Some((only, [])) => write!(out, " forces {only}").unwrap(),
            Some((last, rest)) => write!(out, " forces {} and {last}", rest.join(", ")).unwrap(),
        }
        match above[w].split_last() {
            None => out.push('.'),
            Some((only, [])) => write!(out, "; world {only} is above it.").unwrap(),
            Some((last, rest)) => {
                let rest: Vec<String> = rest.iter().map(usize::to_string).collect();
                write!(out, "; worlds {} and {last} are above it.", rest.join(", ")).unwrap();
            }
        }
    }
    out.push('\n');
    out.push_str(last);
    out
}
