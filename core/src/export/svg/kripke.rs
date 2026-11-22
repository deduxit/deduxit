// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The drawing of a Kripke model: each world a box with the atoms it
//! forces, its number beside it, and an arrow for each of its arrows. The
//! model's world, the one a countermodel refutes the sequent at, has its
//! box in the highlight colour.
//!
//! Where the arrows are an order's steps (intuitionistic logic), the
//! worlds stand in rows: a world with no arrow into it in the bottom row,
//! and every other one in the row above the highest world with an arrow
//! to it, so that every arrow points up. Where they are a relation (modal
//! logic), with loops and cycles, the model's world stands in the top row
//! and every other world in the row of its distance from it along the
//! arrows; an arrow points down, up or across, a loop is a curl on the
//! box's right, and two worlds that see each other have two arrows side
//! by side. Within a row the worlds are ordered by where the worlds
//! before them stand, which keeps most arrows from crossing. An arrow
//! past a row bows out to the right.

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

/// How a model's arrows are read and drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Arrows {
    /// An order's steps, which go round nowhere: every arrow points up to
    /// a world above.
    Order,
    /// A relation, any world to any, itself included: the worlds each
    /// world sees.
    Relation,
}

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

    /// Returns the right end of the box.
    const fn right(&self) -> i64 {
        self.left() + self.width
    }

    /// Returns the width of the world's number and box together.
    const fn span(&self) -> i64 {
        self.number.width + NUMBER_GAP + self.width
    }
}

/// What a world's box shows, and what the description says of it, where
/// they are not a propositional world's atoms: a first-order world's
/// domain, values and facts.
pub(super) struct Contents {
    /// The box's text, in the drawing's markup.
    pub(super) markup: String,
    /// The description's words after `World N`.
    pub(super) words: String,
}

/// Returns a Kripke model drawn as an SVG document titled `title`, whose
/// description reads each world and ends with `last`, a sentence on the
/// model's world. The model is taken as it is: read as an order, its
/// arrows must go round nowhere, which the caller's check of the model
/// ensures. With `contents`, each world's box shows its entry's markup in
/// place of its atoms, and the description starts with the header given
/// and says each entry's words.
///
/// # Errors
///
/// - [`Refusal::Output`] for a drawing estimated past
///   `limits.derivation_bytes`, before anything is laid out.
/// - [`Refusal::Stopped`] where `stop` asked, which is once per world.
/// - [`Error::InvalidOption`] for a style [`Style::check`] refuses.
#[expect(
    clippy::too_many_arguments,
    reason = "the model, what its boxes show, its reading, its words, and how it is drawn and bounded"
)]
pub(super) fn draw(
    model: &Model,
    contents: Option<(&str, &[Contents])>,
    arrows: Arrows,
    title: &str,
    last: &str,
    style: &Style,
    limits: &Limits,
    mut stop: impl FnMut() -> bool,
) -> Result<String, Error> {
    style.check()?;
    let worlds = &model.worlds;
    within(limits, || {
        worlds
            .iter()
            .fold(
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
            .saturating_add(contents.map_or(0, |(header, contents)| {
                contents.iter().fold(header.len() as u64, |sum, c| {
                    sum.saturating_add(
                        (c.markup.len() as u64 + c.words.len() as u64).saturating_mul(ATOM),
                    )
                })
            }))
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
    let (row, order) = match arrows {
        Arrows::Order => order_rows(&above, &mut || halt(&mut stop))?,
        Arrows::Relation => distance_rows(&above, model.world as usize, &mut || halt(&mut stop))?,
    };
    let rows = row.iter().max().map_or(0, |&r| r + 1);
    // Within a row, by the mean place of the worlds below, then by number.
    let mut below: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (w, targets) in above.iter().enumerate() {
        for &v in targets {
            if row[w] < row[v] {
                below[v].push(w);
            }
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
            if let Some((_, contents)) = contents {
                forced.push_str(&contents[w].markup);
            }
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
    // A relation's arrows within a row curve above or below it, a loop
    // curls a box's height out to its right.
    let relation = arrows == Arrows::Relation;
    let room = if relation {
        i64::from(style.line_height)
    } else {
        0
    };
    let curl = if relation { box_height } else { 0 };
    let height =
        rows as i64 * box_height + (rows as i64 - 1).max(0) * (rise - box_height) + 2 * room;
    for (r, members) in members.iter().enumerate() {
        let mut x = margin + (width - widths[r]) / 2;
        let top = if relation {
            margin + room + r as i64 * rise
        } else {
            margin + (rows - 1 - r) as i64 * rise
        };
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
        .any(|(w, targets)| targets.iter().any(|&v| row[v].abs_diff(row[w]) > 1));
    let size = (
        width + 2 * margin + if skips { reach } else { 0 } + curl,
        height + 2 * margin,
    );
    let description = style
        .description
        .then(|| describe(model, contents, arrows, &above, last));
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
                let head = i64::from(style.node_radius) / 2;
                if relation {
                    relation_arrow(
                        out,
                        (w, from),
                        (v, to),
                        &above,
                        &row,
                        (box_height, room, reach),
                        head,
                    );
                    continue;
                }
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
                    head,
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
/// world on what it forces and the worlds its arrows go to, then `last`.
fn describe(
    model: &Model,
    contents: Option<(&str, &[Contents])>,
    arrows: Arrows,
    above: &[Vec<usize>],
    last: &str,
) -> String {
    let n = model.worlds.len();
    if let Some((header, contents)) = contents {
        let mut out = header.to_owned();
        for (w, entry) in contents.iter().enumerate() {
            write!(out, "\nWorld {w}").unwrap();
            if w == model.world as usize {
                out.push_str(", the model's world,");
            }
            write!(out, " {}", entry.words).unwrap();
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
        return out;
    }
    let mut out = format!(
        "A Kripke model of {n} world{}. {}",
        if n == 1 { "" } else { "s" },
        match arrows {
            Arrows::Order => {
                "An arrow points from a world to a world above it, which forces everything \
                 the world below forces."
            }
            Arrows::Relation => "An arrow points from a world to a world it sees.",
        }
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
        let named = |v: usize| {
            if v == w {
                "itself".to_owned()
            } else {
                format!("world {v}")
            }
        };
        match (arrows, above[w].split_last()) {
            (_, None) => out.push('.'),
            (Arrows::Order, Some((only, []))) => {
                write!(out, "; world {only} is above it.").unwrap()
            }
            (Arrows::Order, Some((last, rest))) => {
                let rest: Vec<String> = rest.iter().map(usize::to_string).collect();
                write!(out, "; worlds {} and {last} are above it.", rest.join(", ")).unwrap();
            }
            (Arrows::Relation, Some((&only, []))) => {
                write!(out, "; it sees {}.", named(only)).unwrap()
            }
            (Arrows::Relation, Some((&last, rest))) => {
                let rest: Vec<String> = rest.iter().map(|&v| named(v)).collect();
                write!(out, "; it sees {} and {}.", rest.join(", "), named(last)).unwrap();
            }
        }
    }
    out.push('\n');
    out.push_str(last);
    out
}

/// Returns each world's row and the worlds in the order the rows were
/// found, by Kahn's order over an order's steps: a world after every
/// world below it, one row above the highest of them.
fn order_rows(
    above: &[Vec<usize>],
    halt: &mut dyn FnMut() -> Result<(), Error>,
) -> Result<(Vec<usize>, Vec<usize>), Error> {
    let n = above.len();
    let mut pending = vec![0usize; n];
    for targets in above {
        for &v in targets {
            pending[v] += 1;
        }
    }
    let mut ready: Vec<usize> = (0..n).filter(|&w| pending[w] == 0).rev().collect();
    let mut row = vec![0usize; n];
    let mut order = Vec::with_capacity(n);
    while let Some(w) = ready.pop() {
        halt()?;
        order.push(w);
        for &v in &above[w] {
            row[v] = row[v].max(row[w] + 1);
            pending[v] -= 1;
            if pending[v] == 0 {
                ready.push(v);
            }
        }
    }
    Ok((row, order))
}

/// Returns each world's row and the worlds in the order found, for a
/// relation: the model's world in row 0, every world it reaches in the
/// row of its distance along the arrows (breadth first), and every world
/// it does not reach after them, from row 0 again, in its number's order.
fn distance_rows(
    above: &[Vec<usize>],
    first: usize,
    halt: &mut dyn FnMut() -> Result<(), Error>,
) -> Result<(Vec<usize>, Vec<usize>), Error> {
    let n = above.len();
    let mut row = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);
    for start in std::iter::once(first).chain(0..n) {
        if row[start] != usize::MAX {
            continue;
        }
        row[start] = 0;
        let mut queue = std::collections::VecDeque::from([start]);
        while let Some(w) = queue.pop_front() {
            halt()?;
            order.push(w);
            for &v in &above[w] {
                if row[v] == usize::MAX {
                    row[v] = row[w] + 1;
                    queue.push_back(v);
                }
            }
        }
    }
    Ok((row, order))
}

/// Writes an arrow of a relation from world `w` to world `v`: a curl on
/// the box's right for a loop; within a row a curve above the row (to a
/// world further right) or below it (to the left); between rows a line
/// from one box's edge to the other's, set aside where the two worlds see
/// each other, bowing out by `reach` where it passes a row. `sizes` are
/// the box's height, the room above and below a row, and the reach.
fn relation_arrow(
    out: &mut String,
    (w, from): (usize, &Placed),
    (v, to): (usize, &Placed),
    above: &[Vec<usize>],
    row: &[usize],
    (box_height, room, reach): (i64, i64, i64),
    head: i64,
) {
    let path =
        |out: &mut String, start: (i64, i64), control: Option<(i64, i64)>, tip: (i64, i64)| {
            match control {
                Some(c) => writeln!(
                    out,
                    r#"<path d="M{} {}Q{} {} {} {}"/>"#,
                    start.0, start.1, c.0, c.1, tip.0, tip.1
                ),
                None => writeln!(
                    out,
                    r#"<path d="M{} {}L{} {}"/>"#,
                    start.0, start.1, tip.0, tip.1
                ),
            }
            .unwrap();
            let towards = control.unwrap_or(start);
            arrowhead(
                out,
                tip,
                ((tip.0 - towards.0) as f64, (tip.1 - towards.1) as f64),
                head,
            );
        };
    if w == v {
        // A curl out of the box's right side and back into it.
        let (x, y) = (from.right(), from.top + box_height / 2);
        let d = box_height / 3;
        writeln!(
            out,
            r#"<path d="M{x} {}C{} {} {} {} {x} {}"/>"#,
            y - d,
            x + box_height,
            y - 2 * d,
            x + box_height,
            y + 2 * d,
            y + d
        )
        .unwrap();
        arrowhead(out, (x, y + d), (-(box_height as f64), -(d as f64)), head);
        return;
    }
    if row[w] == row[v] {
        let ((x0, y0), (x1, y1), bend) = if from.middle() < to.middle() {
            ((from.middle(), from.top), (to.middle(), to.top), -2 * room)
        } else {
            (
                (from.middle(), from.top + box_height),
                (to.middle(), to.top + box_height),
                2 * room,
            )
        };
        path(
            out,
            (x0, y0),
            Some((i64::midpoint(x0, x1), y0 + bend)),
            (x1, y1),
        );
        return;
    }
    // Two worlds that see each other: each arrow a sixth of the narrower
    // box to the side of its direction.
    let aside = if above[v].binary_search(&w).is_ok() {
        let d = from.width.min(to.width) / 6;
        if row[w] < row[v] { -d } else { d }
    } else {
        0
    };
    let (start, tip) = if row[w] < row[v] {
        (
            (from.middle() + aside, from.top + box_height),
            (to.middle() + aside, to.top),
        )
    } else {
        (
            (from.middle() + aside, from.top),
            (to.middle() + aside, to.top + box_height),
        )
    };
    let control = (row[w].abs_diff(row[v]) > 1).then(|| {
        (
            2 * (start.0.max(tip.0) + reach) - (start.0 + tip.0) / 2,
            i64::midpoint(start.1, tip.1),
        )
    });
    path(out, start, control, tip);
}
