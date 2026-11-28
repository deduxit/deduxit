// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The layout of a proof structure with boxes. It is the layout of a
//! structure without boxes (`net.rs`), with every `!`-box drawn as a
//! rectangle around its contents:
//! - the literals stand side by side along the top, under the axiom
//!   links' arcs;
//! - every vertex is one row below the lowest of what it depends on;
//! - the conclusions are at the bottom.
//!
//! The premises of a `?` vertex are its copies. A copy may lie in a box
//! that its `?` vertex is outside of. Its edge then leaves the box through
//! a door on the box's bottom border, and goes on as a wire to the door of
//! the box around or to the `?` vertex itself.
//!
//! A box is laid out as one piece above its `!` vertex. The `!`'s premise
//! stands where it stands in the `!` formula's text. Beside it stand the
//! copies in the box whose `?` vertex is outside it: those of a `?` that
//! comes earlier in the formula trees to its left, the others to its
//! right. The rectangle encloses them and every box nested in them. Its
//! bottom border runs through the `!` vertex's joint (the principal door)
//! and through an auxiliary door for every `?` vertex outside with a copy
//! inside. The wires from those copies meet at that door and leave the
//! box, straight to the door of the box around it or to the `?` vertex.
//!
//! - A `?` vertex stands at least a row below every door its wires leave.
//! - A link over a box rises above the box's top.
//! - The copies of a `?` vertex with several at its own depth stand on
//!   the row right above it.
//! - Wires and the premise edges of a `?` vertex are switched as a `⅋`'s
//!   premises are, and drawn like them.
//! - A weakening (a `?` vertex without copies) stands alone. The literal
//!   it hangs on is in the description.
//! - A door's wire may cross other edges and formulas, where the trees do
//!   not let it go around.
//!
//! Under [`WhyNot::Rules`] a `?` vertex is drawn as nodes of rules
//! instead: a dereliction `?d` under each copy, binary contractions `?c`
//! in a column over its formula, or a weakening `?w`. A box then has a
//! door per copy, and a wire crosses its border with no symbol.
//!
//! A vertex's formula, or a compact vertex's circle, has the id `o<n>`
//! when it is the only vertex of occurrence `n`, and `v<k>` (its vertex
//! number) when its occurrence has several. A link has `l<x>-<y>`, the
//! vertices it joins, and a box's rectangle `b<k>`, its `!` vertex.

use super::font::{AXIS, DEPTH, HEIGHT};
use super::net::{
    ARROW, BINARY, CONNECTIVE, Count, DOCUMENT, GROUP, LINK, PAR_MIDDLE, PIECE, ROOT, arrow_along,
    height, towards,
};
use super::{
    Conclusions, NOTATION, NetDrawing, NetGroups, Premises, RAISED_BOT, Style, WhyNot, cut_bar,
    escaped, lay, link_arc, net_conclusion, net_document, net_formula, net_links, net_text,
    net_verdict, run, text,
};
use crate::export::svg::Marks;
use crate::hash::HashMap;
use crate::linear::nets::{MellStructure, NetError, VertexId};
use crate::linear::occurrences::{OccId, Reading, Side};
use crate::linear::sequents::Kind;
use std::fmt::Write;

/// No vertex: the box outside every box, or a root's parent.
const NONE: u32 = u32::MAX;

/// The bytes of a box's rectangle besides its doors' gaps.
const BOX: u64 = 120;

/// The bytes of a door: its symbol or circle, its gap in the border and
/// the wire that leaves it.
const DOOR: u64 = 400;

/// The bytes of a wire from a copy into a door.
const WIRE: u64 = 80;

/// The bytes of a node of the rules: its label, its circle and the edge
/// into it.
const RULE: u64 = 200;

/// The auxiliary doors of a structure, as pairs of a box and a key. The
/// key is a `?` vertex outside the box with a copy inside, or under
/// [`WhyNot::Rules`] that copy itself, since that look gives each copy a
/// door of its own.
struct Doors {
    /// The pairs (`!` vertex, `?` vertex or copy), sorted.
    pairs: Vec<(u32, u32)>,
}

impl Doors {
    /// Returns the doors of a structure, one per `?` vertex and box, or
    /// with `apart` one per copy and box.
    ///
    /// A copy in another box than its `?` vertex gets a door on every box
    /// from its own outwards, up to the box that holds the `?` vertex. The
    /// walk outwards stops at a door already found, so no chain of boxes
    /// is walked twice for one key.
    fn of(net: &MellStructure, apart: bool) -> Self {
        let mut found: crate::hash::HashSet<(u32, u32)> = crate::hash::HashSet::default();
        for c in net.vertices() {
            let Some(q) = net.parent(c).filter(|&q| kind(net, q) == Kind::Quest) else {
                continue;
            };
            let home = raw(net.inside(q));
            let key = if apart { c.get() } else { q.get() };
            let mut b = raw(net.inside(c));
            while b != home && found.insert((b, key)) {
                b = raw(net.inside(VertexId::new(b)));
            }
        }
        let mut pairs: Vec<(u32, u32)> = found.into_iter().collect();
        pairs.sort_unstable();
        Self { pairs }
    }

    /// Returns the index of the door of box `b` for `key`, a `?` vertex
    /// or a copy.
    fn index(&self, b: u32, key: u32) -> usize {
        self.pairs
            .binary_search(&(b, key))
            .expect("a door on the chain of a copy")
    }
}

/// Where a wire ends under [`WhyNot::Rules`]: at a node of the rules or
/// at a formula.
#[derive(Clone, Copy)]
struct End {
    /// The node's centre, or the point of the formula it goes into.
    at: (i64, i64),
    /// How far from `at` it stops: the node's radius, or none.
    into: i64,
    /// The height of its leg when it comes down into `at` from above, or
    /// none when it comes in from the side.
    above: Option<i64>,
}

/// Returns the kind of a vertex's occurrence.
fn kind(net: &MellStructure, v: VertexId) -> Kind {
    net.forest().kind(net.occurrence(v).expect("a vertex"))
}

/// Returns a vertex as its raw index, `NONE` for none.
fn raw(v: Option<VertexId>) -> u32 {
    v.map_or(NONE, VertexId::get)
}

/// Returns an estimate of the bytes of a structure's drawing, made without
/// drawing it.
///
/// The estimate is at least what the drawing has without its
/// description, which [`described_bytes`] counts. `read` says the
/// structure is drawn as its essential net. The sums saturate, so no
/// structure is estimated small.
pub(super) fn estimate(net: &MellStructure, style: &Style, read: bool) -> u64 {
    let forest = net.forest();
    let sequent = forest.sequent();
    let formulas = style.net == NetDrawing::Formulas;
    let rules = style.net_look.why_not == WhyNot::Rules;
    let bottom = style.net_look.conclusions == Conclusions::Bottom;
    // Drawn as an essential net, a colour on every piece of text and
    // label, and an arrowhead or a dotted line on every edge.
    let colour = match read {
        true => escaped(&style.input).len() as u64 + 8,
        false => 0,
    };
    let arrow = if read { ARROW + colour } else { 0 };
    let mut bytes = DOCUMENT;
    // Per occurrence, the bytes of its formula's text, at four bytes a
    // letter, and how many negated literals it has, each a raised `⊥` of
    // two more pieces. Then, for the title, its symbol or its name.
    let (mut chars, mut duals) = (vec![0u64; forest.len()], vec![0u64; forest.len()]);
    for o in forest.ids().rev() {
        let i = o.index();
        let below: Vec<OccId> = forest.children(o).collect();
        (chars[i], duals[i]) = match (below.as_slice(), forest.atom(o)) {
            ([], atom) => {
                let name = atom.map_or(0, |a| sequent.atom_name(a).len() as u64);
                // The reading writes a literal as its atom.
                let dual = u64::from(!read && forest.kind(o) == Kind::DualAtom);
                (name.saturating_mul(4).saturating_add(3), dual)
            }
            (children, _) => children.iter().fold((BINARY, 0u64), |(c, d), &k| {
                (
                    c.saturating_add(chars[k.index()]),
                    d.saturating_add(duals[k.index()]),
                )
            }),
        };
        bytes = bytes.saturating_add(match forest.atom(o) {
            Some(atom) => (sequent.atom_name(atom).len() as u64).saturating_add(3),
            None => 9,
        });
    }
    for v in net.vertices() {
        let o = net.occurrence(v).expect("a vertex");
        let root = net.parent(v).is_none();
        if formulas || forest.is_literal(o) || root {
            let pieces = duals[o.index()].saturating_mul(2).saturating_add(1);
            bytes = bytes
                .saturating_add(chars[o.index()])
                .saturating_add(pieces.saturating_mul(PIECE + colour))
                .saturating_add(GROUP + colour);
        }
        // Its symbol, its circle or joint, the edges into it, its wire into
        // a door, and a box's rectangle. Drawn as an essential net, an
        // arrowhead on the edge between it and its parent, or on its wire.
        bytes = bytes.saturating_add(CONNECTIVE + WIRE + arrow);
        if kind(net, v) == Kind::Bang {
            bytes = bytes.saturating_add(BOX);
        }
        if root {
            bytes = bytes.saturating_add(ROOT + arrow);
            // And the edge down from a raised `!` to the bottom row.
            if bottom && kind(net, v) == Kind::Bang {
                bytes = bytes.saturating_add(ROOT);
            }
        }
        // Under the rules, a copy's dereliction and the contraction it
        // goes into, and a `?` vertex's weakening.
        if rules {
            let copy = net.parent(v).is_some_and(|q| kind(net, q) == Kind::Quest);
            if copy {
                bytes = bytes.saturating_add(2 * (RULE + arrow));
            }
            if kind(net, v) == Kind::Quest {
                bytes = bytes.saturating_add(RULE + arrow);
            }
        }
    }
    let doors = Doors::of(net, rules).pairs.len() as u64;
    let links = net.links().len() as u64;
    bytes = bytes
        .saturating_add(doors.saturating_mul(DOOR + arrow))
        .saturating_add(links.saturating_mul(LINK + arrow));
    if read {
        // The drawing's two more groups, each an element with its colour,
        // its stroke's width of up to ten digits and its end.
        let colours = escaped(&style.input).len() + escaped(&style.dominator).len();
        bytes = bytes.saturating_add(2 * 70 + colours as u64);
    }
    bytes
}

/// Returns the bytes of a structure's description as [`describe`]
/// writes it, escaped, or some number past `most`. The drawing's
/// estimate leaves it out: it needs the verdict, which the criterion
/// gives only after the estimate has refused what is too large.
pub(super) fn described_bytes(
    net: &MellStructure,
    verdict: &Result<(), NetError>,
    reading: Option<&Reading<'_>>,
    most: u64,
) -> u64 {
    let mut count = Count::new(most);
    let _ = describe(net, verdict, reading, &mut count);
    count.bytes()
}

/// Writes a structure's description, which a screen reader reads in
/// place of the drawing, as short sentences: the sequent it concludes,
/// each cut link, each axiom link (by the smaller of its ends), each box
/// by its `!` vertex, the box `b<k>` of the drawing, then each `?`
/// vertex's copies or the link its weakening hangs on, and `verdict`. A
/// formula is named by its vertex, as the net's text form names it, and
/// written in the title's notation.
pub(super) fn describe(
    net: &MellStructure,
    verdict: &Result<(), NetError>,
    reading: Option<&Reading<'_>>,
    out: &mut impl Write,
) -> std::fmt::Result {
    let forest = net.forest();
    let occ = |v: VertexId| net.occurrence(v).expect("a vertex of the structure");
    let end = |v: VertexId| (v.get(), occ(v));
    let formula =
        |out: &mut _, v: VertexId| net_formula(out, "formula", v.get(), forest, reading, occ(v));
    net_conclusion(out, forest, reading)?;
    net_links(out, net.cuts(), net.links(), end, forest, reading)?;
    for b in net.boxes() {
        write!(out, "\nBox {} holds ", b.get())?;
        if let Some(premise) = net.premises(b).next() {
            formula(out, premise)?;
            out.write_str(", the premise of ")?;
        }
        formula(out, b)?;
        out.write_char('.')?;
        if let Some(outer) = net.inside(b) {
            write!(out, " It lies in box {}.", outer.get())?;
        }
    }
    for q in net
        .vertices()
        .filter(|&q| forest.kind(occ(q)) == Kind::Quest)
    {
        out.write_char('\n')?;
        net_formula(out, "Formula", q.get(), forest, reading, occ(q))?;
        let copies = net.premises(q).count();
        match copies {
            0 => {
                out.write_str(", has no copy")?;
                if let Some(l) = net.hangs_on(q) {
                    out.write_str(" and hangs on the axiom link of ")?;
                    formula(out, l)?;
                }
            }
            1 => out.write_str(", has 1 copy: ")?,
            n => write!(out, ", has {n} copies: ")?,
        }
        for (k, c) in net.premises(q).enumerate() {
            if k > 0 {
                out.write_str("; ")?;
            }
            formula(out, c)?;
            if let Some(d) = net.inside(c).filter(|&d| Some(d) != net.inside(q)) {
                write!(out, ", in box {}", d.get())?;
            }
        }
        out.write_char('.')?;
    }
    match verdict {
        Ok(()) => net_verdict(out, net.criterion(), None),
        Err(error) => {
            let described = error.describe(net);
            let described = match reading {
                Some(_) => described.in_mode(crate::linear::Mode::INTUITIONISTIC),
                None => described,
            };
            net_verdict(out, net.criterion(), Some(&described))
        }
    }
}

/// A structure with boxes as the drawing reads its vertices: their
/// occurrences and kinds, their formulas as the look writes them, and
/// their ids.
struct View<'a> {
    /// The structure.
    net: &'a MellStructure,
    /// The style.
    style: &'a Style,
    /// The reading of the sequent, for an essential net.
    reading: Option<&'a Reading<'a>>,
    /// Whether the vertices are written as their formulas.
    formulas: bool,
    /// Whether a `?` vertex is drawn as nodes of the rules.
    rules: bool,
    /// How many vertices each occurrence has, for the ids.
    instances: Vec<u32>,
}

impl<'a> View<'a> {
    /// Returns the view of a structure under a style and a reading.
    fn of(net: &'a MellStructure, style: &'a Style, reading: Option<&'a Reading<'a>>) -> Self {
        let mut view = Self {
            net,
            style,
            reading,
            formulas: style.net == NetDrawing::Formulas,
            rules: style.net_look.why_not == WhyNot::Rules,
            instances: vec![0u32; net.forest().len()],
        };
        for v in 0..net.len() as u32 {
            let o = view.occ(v).index();
            view.instances[o] += 1;
        }
        view
    }

    /// Returns the advance of a character.
    fn advance(&self, c: char) -> i64 {
        i64::from(self.style.font.advances.advance(c))
    }

    /// Returns the width of a text.
    fn width(&self, text: &str) -> i64 {
        lay(text, 1000, &self.style.font, |_| {})
    }

    /// Returns a vertex's occurrence.
    fn occ(&self, v: u32) -> OccId {
        self.net.occurrence(VertexId::new(v)).expect("a vertex")
    }

    /// Returns a vertex's kind.
    fn kind_of(&self, v: u32) -> Kind {
        self.net.forest().kind(self.occ(v))
    }

    /// Returns a vertex's parent, or `NONE`.
    fn parent(&self, v: u32) -> u32 {
        raw(self.net.parent(VertexId::new(v)))
    }

    /// Returns the `!` vertex of a vertex's innermost box, or `NONE`.
    fn inside(&self, v: u32) -> u32 {
        raw(self.net.inside(VertexId::new(v)))
    }

    /// Returns whether a vertex is a conclusion outside every box.
    fn root(&self, v: u32) -> bool {
        self.parent(v) == NONE && self.inside(v) == NONE
    }

    /// Returns whether a vertex's formula is written.
    fn written(&self, v: u32) -> bool {
        self.formulas || self.net.forest().is_literal(self.occ(v)) || self.root(v)
    }

    /// Returns whether a vertex is a `⊗` or a `⅋`.
    fn binary(&self, v: u32) -> bool {
        matches!(self.kind_of(v), Kind::Tensor | Kind::Par)
    }

    /// Writes a vertex's formula as the drawing writes it: one-sided, or as
    /// the reading reads it, which writes a literal as its atom.
    fn write(&self, out: &mut String, v: u32, brackets: bool) {
        net_text(out, self.net.forest(), self.reading, self.occ(v), brackets);
    }

    /// Returns whether a vertex is in input position under the reading.
    fn input(&self, v: u32) -> bool {
        self.reading
            .is_some_and(|r| r.position(self.occ(v)) == Side::Input)
    }

    /// Returns the fill attribute of an input vertex's text, or nothing.
    fn fill(&self, v: u32) -> String {
        match self.input(v) {
            true => format!(r#" fill="{}""#, escaped(&self.style.input)),
            false => String::new(),
        }
    }

    /// Returns the symbol at a vertex's joint, door or circle, and the
    /// height of its middle above its baseline. Under the reading, `A⊥ ⅋ B`
    /// in output position and `A ⊗ B⊥` in input position are `⊸`, and a `?`
    /// is the hypothesis `!A`.
    fn symbol_of(&self, v: u32) -> (&'static str, i64) {
        match self.kind_of(v) {
            Kind::Par | Kind::Tensor if self.reading.is_some() => {
                match (self.kind_of(v) == Kind::Par) != self.input(v) {
                    true => (NOTATION.lollipop, AXIS),
                    false => (NOTATION.tensor, AXIS),
                }
            }
            Kind::Par => (NOTATION.par, PAR_MIDDLE),
            Kind::Tensor => (NOTATION.tensor, AXIS),
            Kind::Quest if self.reading.is_some() => (NOTATION.bang, PAR_MIDDLE),
            Kind::Bang => (NOTATION.bang, PAR_MIDDLE),
            Kind::Quest => (NOTATION.quest, PAR_MIDDLE),
            Kind::Atom
            | Kind::DualAtom
            | Kind::One
            | Kind::Bot
            | Kind::Top
            | Kind::Zero
            | Kind::With
            | Kind::Plus
            | Kind::Forall
            | Kind::Exists => {
                unreachable!("a literal has no symbol, a MELL net no unit, additive or binder")
            }
        }
    }

    /// Returns whether a copy lies in another box than its `?` vertex.
    fn boxed(&self, c: u32) -> bool {
        let q = self.parent(c);
        q != NONE && self.kind_of(q) == Kind::Quest && self.inside(c) != self.inside(q)
    }

    /// Returns a vertex's id: `o<n>` for the only vertex of its occurrence,
    /// else `v<k>`.
    fn id(&self, v: u32) -> String {
        match self.instances[self.occ(v).index()] {
            1 => format!("o{}", self.occ(v).get()),
            _ => format!("v{v}"),
        }
    }

    /// Returns the key of a boxed copy's doors: its `?` vertex, or under
    /// the rules the copy itself.
    fn key_of(&self, c: u32) -> u32 {
        if self.rules { c } else { self.parent(c) }
    }

    /// Returns the `?` vertex of a door's key.
    fn owner(&self, key: u32) -> u32 {
        if self.rules { self.parent(key) } else { key }
    }
}

/// The group strings the drawing writes into, which the document joins.
#[derive(Default)]
struct Groups {
    /// The edges, the circles and the boxes.
    lines: String,
    /// The switched premises and the wires, dashed.
    pars: String,
    /// The axiom links.
    links: String,
    /// What the highlight colour marks: the switching cycle and a step.
    marked: String,
    /// The formulas and the labels.
    texts: String,
    /// Under the reading, what is in input position.
    input_lines: String,
    /// Under the reading, the edges into the antecedents, dotted.
    dominator_lines: String,
}

impl Groups {
    /// Returns, apart, the group of the rules' nodes and edges (the input
    /// group when `input`, else the lines), the highlight, the texts and
    /// the dashed group.
    fn rules(&mut self, input: bool) -> (&mut String, &mut String, &mut String, &mut String) {
        let lines = if input {
            &mut self.input_lines
        } else {
            &mut self.lines
        };
        (lines, &mut self.marked, &mut self.texts, &mut self.pars)
    }
}

/// The column of a `?` vertex's contractions under [`WhyNot::Rules`].
struct Comb {
    /// The column's place, the `?` vertex's.
    column: i64,
    /// The row of the first nodes.
    first: u32,
    /// How many premises the `?` vertex has.
    k: usize,
    /// The `?` vertex's last node.
    last: (i64, i64),
}

/// A structure with boxes laid out: the layout tree, the doors, the rows,
/// every node's place and the drawing's heights, which the emitting
/// passes read.
struct Layout<'a> {
    /// The structure's vertices as the drawing reads them.
    view: View<'a>,
    /// What a step of cut elimination acted on.
    step: &'a Marks,
    /// How many vertices the structure has; node `n + b` is the box of
    /// `!` vertex `b`.
    n: usize,
    /// The cut links, by their ends.
    raw_cuts: Vec<(u32, u32)>,
    /// The formulas at the bottom: the roots, then the two ends of each
    /// cut link outside every box, side by side. The ends of a cut link
    /// inside a box stand in the box, beside its premise.
    tops: Vec<u32>,
    /// Whether switched edges are drawn dashed.
    dashed: bool,
    /// The gap between neighbouring subtrees.
    gap: i64,
    /// A node's radius.
    radius: i64,
    /// The margin around the drawing.
    margin: i64,
    /// The width of a stroke.
    stroke: i64,
    /// The size of a label.
    label_size: i64,
    /// The length of an arrowhead.
    arrow: i64,
    /// The dash array of a dashed edge.
    dash: String,
    /// The label of a dereliction.
    dereliction: String,
    /// The label of a contraction.
    contraction: String,
    /// The label of a weakening.
    weakening: String,
    /// Each vertex's place in the formula trees' pre-order.
    pre: Vec<u32>,
    /// The layout tree's edges, as (parent, key, child), sorted.
    edges: Vec<(u32, u32, u32)>,
    /// Where each node's edges start in `edges`.
    start: Vec<usize>,
    /// The auxiliary doors.
    doors: Doors,
    /// Per `?` vertex, the boxes whose doors its wires leave last.
    last: HashMap<u32, Vec<u32>>,
    /// Under the rules, each `?` vertex's premises.
    inputs: Vec<u32>,
    /// Under the rules, each `?` vertex's rows of nodes.
    levels: Vec<u32>,
    /// Under the rules, the rows each `?` vertex's nodes take above it.
    extra: Vec<u32>,
    /// The bottom row.
    bottom: u32,
    /// Whether each `!` vertex has auxiliary doors.
    has_doors: Vec<bool>,
    /// Each vertex's row.
    row: Vec<u32>,
    /// Each vertex's anchor, from the left of its text.
    from_left: Vec<i64>,
    /// Each node's subtree's extent around its anchor.
    extent: Vec<(i64, i64)>,
    /// Each node's offset from its parent's anchor.
    offset: Vec<i64>,
    /// The layout tree's nodes in post-order.
    post: Vec<usize>,
    /// Each node's anchor.
    x: Vec<i64>,
    /// The right side of the last formula at the bottom.
    right: i64,
    /// Each box's depth.
    nesting: Vec<u32>,
    /// Each box's lid, how far above the top its rectangle reaches.
    lid: Vec<i64>,
    /// Each link's rise over the boxes it spans.
    lift: HashMap<(u32, u32), i64>,
    /// The highest arc outside every box.
    highest: i64,
    /// The part of `line_height` that the legs of wires cannot use.
    room_beside: i64,
    /// The line height, raised for the wires.
    line_height: i64,
    /// The top row's baseline.
    baseline: i64,
    /// The top of the top row's text.
    top: i64,
    /// The distance between two rows with formulas.
    pitch: i64,
    /// The lowest row.
    rows: u32,
    /// Where a compact conclusion's hanging edge ends.
    end: i64,
    /// A compact conclusion's baseline.
    under: i64,
    /// The font's deepest descender.
    depth: i64,
    /// Each door's point.
    point: Vec<(i64, i64)>,
    /// A switching cycle, as the pairs of vertices its edges join, sorted.
    cycle: Vec<(u32, u32)>,
}

impl<'a> Layout<'a> {
    /// Returns the layout of a structure under a style and a reading, with
    /// the switching cycle of `verdict` and what `step` acted on to mark.
    fn of(
        net: &'a MellStructure,
        style: &'a Style,
        verdict: &Result<(), NetError>,
        reading: Option<&'a Reading<'a>>,
        step: &'a Marks,
    ) -> Self {
        let forest = net.forest();
        let raw_cuts: Vec<(u32, u32)> = net
            .cuts()
            .iter()
            .map(|&(x, y)| (x.get(), y.get()))
            .collect();
        let mut tops: Vec<u32> = (0..forest.roots().len() as u32).collect();
        tops.extend(
            raw_cuts
                .iter()
                .filter(|&&(x, _)| net.inside(VertexId::new(x)).is_none())
                .flat_map(|&(x, y)| [x, y]),
        );
        let view = View::of(net, style, reading);
        let rules = view.rules;
        // An essential net tells its edges apart by their polarity, so it
        // draws no switched edge dashed.
        let dashed = reading.is_none() && style.net_look.premises == Premises::Dashed;
        let (radius, stroke) = (i64::from(style.node_radius), i64::from(style.stroke_width));
        // Under the reading the rules are those of a hypothesis `!A`.
        let why_not = if reading.is_some() {
            NOTATION.bang
        } else {
            NOTATION.quest
        };
        let mut layout = Self {
            view,
            step,
            n: net.len(),
            raw_cuts,
            tops,
            dashed,
            gap: i64::from(style.literal_gap),
            radius,
            margin: i64::from(style.margin),
            stroke,
            label_size: i64::from(style.label_size),
            arrow: radius / 2,
            dash: format!("{} {}", 3 * stroke, 3 * stroke),
            dereliction: format!("{why_not}d"),
            contraction: format!("{why_not}c"),
            weakening: format!("{why_not}w"),
            pre: Vec::new(),
            edges: Vec::new(),
            start: Vec::new(),
            doors: Doors::of(net, rules),
            last: HashMap::default(),
            inputs: Vec::new(),
            levels: Vec::new(),
            extra: Vec::new(),
            bottom: 0,
            has_doors: Vec::new(),
            row: Vec::new(),
            from_left: Vec::new(),
            extent: Vec::new(),
            offset: Vec::new(),
            post: Vec::new(),
            x: Vec::new(),
            right: 0,
            nesting: Vec::new(),
            lid: Vec::new(),
            lift: HashMap::default(),
            highest: 0,
            room_beside: 0,
            line_height: i64::from(style.line_height),
            baseline: 0,
            top: 0,
            pitch: 0,
            rows: 0,
            end: 0,
            under: 0,
            depth: style.font.advances.depth(),
            point: Vec::new(),
            cycle: Vec::new(),
        };
        layout.build_tree();
        layout.count_levels();
        layout.place_rows();
        layout.measure_extents();
        layout.place_anchors();
        layout.raise_links();
        layout.raise_line_height();
        layout.fix_heights();
        layout.place_doors();
        layout.find_cycle(verdict);
        layout
    }

    /// Builds the layout tree, by each node's children:
    /// - a vertex: its premises;
    /// - a `!` vertex: its box, node `n + b`;
    /// - a `?` vertex: its copies at its own depth;
    /// - a box: its `!`'s premise and its boxed copies (copies in the box
    ///   of a `?` outside it), ordered by where their `?` vertex comes in
    ///   the formula trees' pre-order against the `!` vertex.
    fn build_tree(&mut self) {
        let (net, n) = (self.view.net, self.n);
        let mut pre = vec![0u32; n];
        let ends = self.raw_cuts.iter().flat_map(|&(x, y)| [x, y]);
        let roots = net.forest().roots().len() as u32;
        let mut walk: Vec<u32> = (0..roots).chain(ends).rev().collect();
        let mut time = 0;
        while let Some(v) = walk.pop() {
            pre[v as usize] = time;
            time += 1;
            let premises: Vec<u32> = net.premises(VertexId::new(v)).map(VertexId::get).collect();
            walk.extend(premises.into_iter().rev());
        }
        let nodes = 2 * n;
        let mut edges: Vec<(u32, u32, u32)> = Vec::with_capacity(nodes);
        // A cut link inside a box stands in it after everything else there.
        for (i, &(x, y)) in self.raw_cuts.iter().enumerate() {
            let b = self.view.inside(x);
            if b != NONE {
                let key = n as u32 + 2 * i as u32;
                edges.push((n as u32 + b, key, x));
                edges.push((n as u32 + b, key + 1, y));
            }
        }
        for v in 0..n as u32 {
            if self.view.kind_of(v) == Kind::Bang {
                edges.push((v, 0, n as u32 + v));
                let premise = net.premises(VertexId::new(v)).next().expect("a premise");
                edges.push((n as u32 + v, pre[v as usize], premise.get()));
                continue;
            }
            for (k, c) in net.premises(VertexId::new(v)).enumerate() {
                if self.view.boxed(c.get()) {
                    let unit = n as u32 + self.view.inside(c.get());
                    edges.push((unit, pre[v as usize], c.get()));
                } else {
                    edges.push((v, k as u32, c.get()));
                }
            }
        }
        edges.sort_unstable();
        let mut start = vec![0usize; nodes + 1];
        for &(node, _, _) in &edges {
            start[node as usize + 1] += 1;
        }
        for i in 0..nodes {
            start[i + 1] += start[i];
        }
        (self.pre, self.edges, self.start) = (pre, edges, start);
    }

    /// Returns a node's children in the layout tree.
    fn kids(&self, node: usize) -> impl Iterator<Item = u32> {
        self.edges[self.start[node]..self.start[node + 1]]
            .iter()
            .map(|e| e.2)
    }

    /// Returns the node of `!` vertex `b`'s box.
    fn unit(&self, b: u32) -> usize {
        self.n + b as usize
    }

    /// Returns the door of the box around that door `d`'s wire goes to,
    /// or none when it goes to its `?` vertex.
    fn target(&self, d: usize) -> Option<usize> {
        let (b, key) = self.doors.pairs[d];
        let outer = self.view.inside(b);
        (outer != self.view.inside(self.view.owner(key))).then(|| self.doors.index(outer, key))
    }

    /// Returns whether `q` is a `?` vertex whose one premise is a wire,
    /// which is its conclusion.
    fn bare(&self, q: u32) -> bool {
        self.view.rules && self.view.kind_of(q) == Kind::Quest && self.levels[q as usize] == 0
    }

    /// Finds per `?` vertex the boxes whose doors its wires leave last,
    /// which it must stand below, and under the rules, per `?` vertex:
    /// - `inputs`, its premises: the copies at its depth and the wires;
    /// - `levels`, its nodes' rows: one for the derelictions of the copies
    ///   at its depth (with wires alone, a landing row the first wire comes
    ///   down through), one per contraction (each premise past the first),
    ///   or one for a weakening; a lone wire takes none;
    /// - `extra`, the rows they take above it: all of them when its formula
    ///   is written under the last, all but the last when the last is its
    ///   circle.
    fn count_levels(&mut self) {
        let (n, view) = (self.n, &self.view);
        let mut last: HashMap<u32, Vec<u32>> = HashMap::default();
        for (d, &(b, key)) in self.doors.pairs.iter().enumerate() {
            if self.target(d).is_none() {
                last.entry(view.owner(key)).or_default().push(b);
            }
        }
        let (mut inputs, mut levels, mut extra) = (vec![0u32; n], vec![0u32; n], vec![0u32; n]);
        if view.rules {
            for c in 0..n as u32 {
                if view.parent(c) != NONE && view.kind_of(view.parent(c)) == Kind::Quest {
                    inputs[view.parent(c) as usize] += 1;
                }
            }
            for q in (0..n as u32).filter(|&q| view.kind_of(q) == Kind::Quest) {
                let (k, own) = (inputs[q as usize], self.kids(q as usize).next().is_some());
                levels[q as usize] =
                    u32::from(own || k >= 2) + k.saturating_sub(1) + u32::from(k == 0);
                extra[q as usize] = match view.formulas {
                    true => levels[q as usize],
                    false => levels[q as usize].saturating_sub(1),
                };
            }
        }
        (self.last, self.inputs, self.levels, self.extra) = (last, inputs, levels, extra);
    }

    /// Returns what a vertex's row depends on: its layout children, a `!`
    /// vertex's box contents, the boxes its wires leave last.
    fn depends(&self, v: u32) -> Vec<u32> {
        let mut on: Vec<u32> = if self.view.kind_of(v) == Kind::Bang {
            self.kids(self.unit(v)).collect()
        } else {
            self.kids(v as usize).collect()
        };
        if let Some(boxes) = self.last.get(&v) {
            on.extend(boxes);
        }
        on
    }

    /// Returns every vertex's layer: one below the lowest of what it
    /// depends on, in a walk that finishes what a vertex depends on first.
    fn layers(&self) -> Vec<u32> {
        let (n, view) = (self.n, &self.view);
        let mut layer = vec![u32::MAX; n];
        let mut stack: Vec<(u32, bool)> = (0..n as u32).rev().map(|v| (v, false)).collect();
        while let Some((v, ready)) = stack.pop() {
            if layer[v as usize] != u32::MAX {
                continue;
            }
            let on = self.depends(v);
            if ready || on.iter().all(|&w| layer[w as usize] != u32::MAX) {
                // Under the rules, a copy in a box has its dereliction on the
                // row below it, inside the box.
                let below = |w: u32| {
                    u32::from(view.rules && view.kind_of(v) == Kind::Bang && view.boxed(w))
                };
                let lowest = on.iter().map(|&w| layer[w as usize] + below(w)).max();
                layer[v as usize] = match (lowest, view.net.forest().is_literal(view.occ(v))) {
                    (_, true) => 0,
                    (Some(l), false) => l + 1 + self.extra[v as usize],
                    (None, false) => 1 + self.extra[v as usize],
                };
            } else {
                stack.push((v, true));
                stack.extend(on.into_iter().map(|w| (w, false)));
            }
        }
        layer
    }

    /// Places the rows: every vertex one below the lowest of what it
    /// depends on (its layout children, a box's contents, the boxes its
    /// wires leave).
    ///
    /// The conclusions go to the bottom row. The copies of a `?` vertex
    /// with several at its own depth go down to the row above it (under
    /// the rules every copy at its depth, to the row above its nodes), so
    /// that their edges into it pass below every formula and box of the
    /// others. A `!` vertex with auxiliary doors stays at its row, above
    /// the `?` vertices its wires go to.
    fn place_rows(&mut self) {
        let layer = self.layers();
        let view = &self.view;
        let bottom = self
            .tops
            .iter()
            .map(|&r| layer[r as usize])
            .max()
            .unwrap_or(0);
        let has_doors: Vec<bool> = {
            let mut has = vec![false; self.n];
            for &(b, _) in &self.doors.pairs {
                has[b as usize] = true;
            }
            has
        };
        let mut row = layer;
        let mut order: Vec<u32> = (0..self.n as u32).collect();
        order.sort_unstable_by_key(|&v| self.pre[v as usize]);
        for &v in &order {
            let fixed = view.kind_of(v) == Kind::Bang && has_doors[v as usize];
            let p = view.parent(v);
            if fixed {
                continue;
            }
            if view.root(v) {
                row[v as usize] = bottom;
            } else if p != NONE
                && view.kind_of(p) == Kind::Quest
                && !view.boxed(v)
                && (view.rules || self.kids(p as usize).nth(1).is_some())
            {
                row[v as usize] = row[v as usize].max(row[p as usize] - 1 - self.extra[p as usize]);
            }
        }
        (self.bottom, self.has_doors, self.row) = (bottom, has_doors, row);
    }

    /// Returns the layout tree's nodes in post-order, from the formulas at
    /// the bottom.
    fn post_order(&self) -> Vec<usize> {
        let mut post: Vec<usize> = Vec::with_capacity(2 * self.n);
        let mut walk: Vec<(usize, bool)> = self
            .tops
            .iter()
            .rev()
            .map(|&r| (r as usize, false))
            .collect();
        while let Some((node, done)) = walk.pop() {
            if done {
                post.push(node);
                continue;
            }
            walk.push((node, true));
            walk.extend(
                self.kids(node)
                    .map(|k| (k as usize, false))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev(),
            );
        }
        post
    }

    /// Measures, bottom-up over the layout tree, the extent of every
    /// subtree around its anchor and each child's offset from its parent's
    /// anchor; a box (node `n + b`) is anchored at its `!`'s premise.
    fn measure_extents(&mut self) {
        let (n, nodes) = (self.n, 2 * self.n);
        let mut formula = String::new();
        self.from_left = vec![0i64; n];
        self.extent = vec![(0i64, 0i64); nodes];
        self.offset = vec![0i64; nodes];
        let post = self.post_order();
        for &node in &post {
            let children: Vec<u32> = self.kids(node).collect();
            if node >= n {
                self.box_extent(node, &children);
            } else {
                self.vertex_extent(node, &children, &mut formula);
            }
        }
        self.post = post;
    }

    /// Lays children side by side `gap` apart, the `fixed`-th at offset
    /// `at`, and returns the extent they cover.
    fn side_by_side(&mut self, children: &[u32], fixed: usize, at: i64) -> (i64, i64) {
        let (gap, offset, extent) = (self.gap, &mut self.offset, &self.extent);
        let e = |c: u32| extent[c as usize];
        offset[children[fixed] as usize] = at;
        let mut cursor = at + e(children[fixed]).0;
        for &c in children[..fixed].iter().rev() {
            offset[c as usize] = cursor - gap - e(c).1;
            cursor = offset[c as usize] + e(c).0;
        }
        let left = cursor;
        let mut cursor = at + e(children[fixed]).1;
        for &c in &children[fixed + 1..] {
            offset[c as usize] = cursor + gap - e(c).0;
            cursor = offset[c as usize] + e(c).1;
        }
        (left, cursor)
    }

    /// Lays out a box: its premise at its anchor, the copies beside it.
    fn box_extent(&mut self, node: usize, children: &[u32]) {
        let b = (node - self.n) as u32;
        let premise = self
            .view
            .net
            .premises(VertexId::new(b))
            .next()
            .expect("a premise")
            .get();
        let fixed = children
            .iter()
            .position(|&c| c == premise)
            .expect("the premise");
        let covered = self.side_by_side(children, fixed, 0);
        self.extent[node] = (covered.0 - self.radius, covered.1 + self.radius);
    }

    /// Returns the advance of a bracket on `side` of vertex `c` when it is
    /// binary, else 0.
    fn bracket(&self, c: u32, side: char) -> i64 {
        if self.view.binary(c) {
            self.view.advance(side)
        } else {
            0
        }
    }

    /// Lays out a vertex's subtree: its own text or circle, then its
    /// children at their offsets. `formula` is a buffer for its text.
    fn vertex_extent(&mut self, node: usize, children: &[u32], formula: &mut String) {
        let v = node as u32;
        formula.clear();
        let text_width = if self.view.written(v) {
            self.view.write(formula, v, false);
            self.view.width(formula)
        } else {
            0
        };
        if self.view.net.forest().is_literal(self.view.occ(v)) {
            let atom = if self.view.kind_of(v) == Kind::DualAtom {
                self.view.width(formula.trim_end_matches(RAISED_BOT))
            } else {
                text_width
            };
            self.from_left[node] = atom / 2;
            self.extent[node] = (-(atom / 2), text_width - atom / 2);
            return;
        }
        let own = match self.view.kind_of(v) {
            Kind::Tensor | Kind::Par => self.binary_extent(node, children, text_width, formula),
            Kind::Bang | Kind::Quest => self.exponential_extent(node, children, text_width),
            Kind::Atom
            | Kind::DualAtom
            | Kind::One
            | Kind::Bot
            | Kind::Top
            | Kind::Zero
            | Kind::With
            | Kind::Plus
            | Kind::Forall
            | Kind::Exists => {
                unreachable!("a literal is handled above and a MELL net has no units or additives")
            }
        };
        self.extent[node] = children.iter().fold(own, |(lo, hi), &c| {
            (
                lo.min(self.offset[c as usize] + self.extent[c as usize].0),
                hi.max(self.offset[c as usize] + self.extent[c as usize].1),
            )
        });
    }

    /// Lays out the two premises of a `⊗` or `⅋` vertex around it and
    /// returns its own extent. `formula` is a buffer for its text.
    fn binary_extent(
        &mut self,
        node: usize,
        children: &[u32],
        text_width: i64,
        formula: &mut String,
    ) -> (i64, i64) {
        let (v, gap, radius) = (node as u32, self.gap, self.radius);
        let (l, r) = (children[0] as usize, children[1] as usize);
        let (left, right) = (self.extent[l], self.extent[r]);
        if self.view.formulas {
            formula.clear();
            self.view.write(formula, l as u32, true);
            formula.push(' ');
            let half = self
                .view
                .symbol_of(v)
                .0
                .chars()
                .map(|c| self.view.advance(c))
                .sum::<i64>()
                / 2;
            self.from_left[node] = self.view.width(formula) + half;
            let space = self.view.advance(' ');
            self.offset[l] = -(half + space + self.bracket(l as u32, ')') + left.1);
            self.offset[r] = half + space + self.bracket(r as u32, '(') - right.0;
            (-self.from_left[node], text_width - self.from_left[node])
        } else {
            // The circle stands in the gap between the two subtrees'
            // extents, as a joint does. Halfway between the anchors, a box
            // of one subtree could reach over the circle, and the edge to
            // the other premise would rise through the box.
            self.offset[l] = -(gap / 2) - left.1;
            self.offset[r] = gap - gap / 2 - right.0;
            self.from_left[node] = text_width / 2;
            (
                (-radius).min(-self.from_left[node]),
                radius.max(text_width - self.from_left[node]),
            )
        }
    }

    /// Lays out the box of a `!` vertex or the copies of a `?` vertex
    /// after its symbol, and returns its own extent.
    fn exponential_extent(&mut self, node: usize, children: &[u32], text_width: i64) -> (i64, i64) {
        let (v, radius) = (node as u32, self.radius);
        let bang = self.view.kind_of(v) == Kind::Bang;
        let symbol = self
            .view
            .symbol_of(v)
            .0
            .chars()
            .map(|c| self.view.advance(c))
            .sum::<i64>();
        // The subformula as it stands after the symbol: the premise, or the
        // first copy.
        let first = if bang {
            self.view
                .net
                .premises(VertexId::new(v))
                .next()
                .map(VertexId::get)
        } else {
            children.first().copied()
        };
        if self.view.formulas {
            self.from_left[node] = symbol / 2;
            if let Some(f) = first {
                let at = symbol + self.bracket(f, '(') + self.from_left[f as usize]
                    - self.from_left[node];
                if bang {
                    let u = self.unit(v);
                    self.offset[u] = at;
                } else {
                    self.side_by_side(children, 0, at);
                }
            }
        } else {
            self.from_left[node] = text_width / 2;
            if !children.is_empty() {
                self.side_by_side(children, 0, 0);
                // The circle halfway between the first and the last anchor.
                let middle = self.offset[*children.last().unwrap() as usize] / 2;
                for &c in children {
                    self.offset[c as usize] -= middle;
                }
            }
        }
        if bang {
            // The box reaches out to its `!` door.
            let u = self.unit(v);
            let joint = -self.offset[u];
            self.extent[u] = (
                self.extent[u].0.min(joint - 2 * radius),
                self.extent[u].1.max(joint + 2 * radius),
            );
        }
        (
            (-radius).min(-self.from_left[node]),
            radius.max(text_width - self.from_left[node]),
        )
    }

    /// Places the conclusions side by side, `literal_gap` apart, and
    /// top-down every anchor from its parent's.
    fn place_anchors(&mut self) {
        let mut x = vec![0i64; 2 * self.n];
        let mut right = self.margin - self.gap;
        for &r in &self.tops {
            let r = r as usize;
            x[r] = right + self.gap - self.extent[r].0;
            right = x[r] + self.extent[r].1;
        }
        for &node in self.post.iter().rev() {
            for c in self.kids(node) {
                x[c as usize] = x[node] + self.offset[c as usize];
            }
        }
        (self.x, self.right) = (x, right);
    }

    /// Returns a box's rectangle, left and right.
    fn sides(&self, b: u32) -> (i64, i64) {
        let u = self.unit(b);
        (self.x[u] + self.extent[u].0, self.x[u] + self.extent[u].1)
    }

    /// Returns a link's arc: its left and right ends, half its width and
    /// its height.
    fn arc(&self, (p, q): (u32, u32)) -> (i64, i64, i64, i64) {
        let x = &self.x;
        let (a, b) = (
            x[p as usize].min(x[q as usize]),
            x[p as usize].max(x[q as usize]),
        );
        let rx = (b - a) / 2;
        (a, b, rx, height(rx, self.view.style))
    }

    /// Finds each box's depth, its chain of boxes walked up only to a box
    /// whose depth is known, so that every box is walked once, and returns
    /// the boxes innermost first.
    fn find_nesting(&mut self) -> Vec<u32> {
        let mut nesting = vec![u32::MAX; self.n];
        let mut boxes: Vec<u32> = self.view.net.boxes().map(VertexId::get).collect();
        let mut chain = Vec::new();
        for &b in &boxes {
            let mut c = b;
            while c != NONE && nesting[c as usize] == u32::MAX {
                chain.push(c);
                c = self.view.inside(c);
            }
            let below = if c == NONE {
                0
            } else {
                nesting[c as usize] + 1
            };
            for (d, &c) in (below..).zip(chain.iter().rev()) {
                nesting[c as usize] = d;
            }
            chain.clear();
        }
        boxes.sort_unstable_by_key(|&b| std::cmp::Reverse(nesting[b as usize]));
        self.nesting = nesting;
        boxes
    }

    /// Finds each box's lid and each link's rise, so that no arc enters a
    /// box it does not belong to. A box's lid is the highest arc in it plus
    /// `node_radius`. A link rises by the highest lid among the boxes of
    /// its depth that it spans. The depths go innermost first.
    fn raise_links(&mut self) {
        let boxes = self.find_nesting();
        // Per depth (a box's `!` vertex, or `NONE`), its boxes by their left
        // side, and its links.
        let mut children_of: HashMap<u32, Vec<u32>> = HashMap::default();
        for &b in &boxes {
            children_of.entry(self.view.inside(b)).or_default().push(b);
        }
        for list in children_of.values_mut() {
            list.sort_unstable_by_key(|&b| self.sides(b).0);
        }
        let mut links_of: HashMap<u32, Vec<(u32, u32)>> = HashMap::default();
        for &(p, q) in self.view.net.links() {
            links_of
                .entry(self.view.inside(p.get()))
                .or_default()
                .push((p.get(), q.get()));
        }
        let mut lid = vec![0i64; self.n];
        let mut lift: HashMap<(u32, u32), i64> = HashMap::default();
        let mut depths: Vec<u32> = boxes;
        depths.push(NONE);
        let mut highest = 0;
        for &depth in &depths {
            let inner = children_of.get(&depth).map_or(&[][..], Vec::as_slice);
            let lids: Vec<i64> = inner.iter().map(|&c| lid[c as usize]).collect();
            let mut tallest = lids.iter().copied().max().unwrap_or(0);
            let table = runs(&lids);
            for &(p, q) in links_of.get(&depth).map_or(&[][..], Vec::as_slice) {
                let (a, b, _, ry) = self.arc((p, q));
                // The boxes of this depth under the link. They lie apart, so
                // they are a run of the list, whose highest lid the sparse
                // table gives.
                let from = inner.partition_point(|&c| self.sides(c).0 < a);
                let to = inner.partition_point(|&c| self.sides(c).1 <= b);
                let raised = most(&table, from, to);
                lift.insert((p, q), raised);
                tallest = tallest.max(raised + ry);
            }
            if depth == NONE {
                highest = tallest;
            } else {
                lid[depth as usize] = tallest + self.radius;
            }
        }
        (self.lid, self.lift, self.highest) = (lid, lift, highest);
    }

    /// Raises `line_height` so that the most wires running level in one
    /// band stand half the radius apart at least, and none reads as part
    /// of another. Each wire is counted at the row it runs above:
    /// - by default, the row of its `?` vertex, or of the box whose door it
    ///   goes into;
    /// - under the rules, the row of the node it comes into; for the first
    ///   wire of a `?` without copies at its depth, its landing row; for a
    ///   lone wire, its formula's row.
    ///
    /// Under the rules a door-to-door wire is not counted: a door there has
    /// one wire, which goes straight down to the next.
    fn raise_line_height(&mut self) {
        let (view, radius) = (&self.view, self.radius);
        // The part of `line_height` that the legs of wires cannot use. The
        // room for legs above a band is `line_height` less this:
        // - by default with formulas, `2.5·radius` and a stroke: a band is
        //   twice the radius above a joint, and a joint is 1.5 radii above its
        //   text's top, so the legs stay below the text of the row above;
        // - under the rules with formulas, so that the room is the pitch less
        //   the circles above, `pitch − 3·radius`;
        // - compact, the circles above, `3·radius`.
        let room_beside = match (view.formulas, view.rules) {
            (true, false) => 5 * radius / 2 + self.stroke,
            (true, true) => 3 * radius - (radius + HEIGHT + DEPTH),
            (false, _) => 3 * radius,
        };
        let mut level: HashMap<u32, i64> = HashMap::default();
        for (d, &(_, key)) in self.doors.pairs.iter().enumerate() {
            let above = match self.target(d) {
                Some(outer) => self.row[self.doors.pairs[outer].0 as usize],
                None => self.row[view.owner(key) as usize],
            };
            if !view.rules {
                *level.entry(above).or_default() += 1;
            }
        }
        for q in (0..self.n as u32).filter(|&q| view.rules && view.kind_of(q) == Kind::Quest) {
            let (k, own) = (
                self.inputs[q as usize],
                self.kids(q as usize).count() as u32,
            );
            let row = self.row[q as usize];
            let first = row + u32::from(!view.formulas) - self.levels[q as usize];
            let last = row - u32::from(view.formulas);
            for i in own..k {
                let band = match (k, i) {
                    (1, _) => row,
                    (_, 0) => first,
                    _ if i.max(1) + 1 == k => last,
                    _ => first + i.max(1),
                };
                *level.entry(band).or_default() += 1;
            }
        }
        let most = level.values().copied().max().unwrap_or(0);
        self.room_beside = room_beside;
        self.line_height = self.line_height.max(most * (radius / 2) + room_beside);
    }

    /// Fixes the heights of the rows from the highest arc and the line
    /// height.
    fn fix_heights(&mut self) {
        self.baseline = self.margin + self.stroke + self.highest + HEIGHT;
        self.top = self.baseline - HEIGHT;
        self.pitch = self.line_height + self.radius + HEIGHT + DEPTH;
        self.rows = self.row.iter().copied().max().unwrap_or(0);
        self.end =
            self.baseline - AXIS + i64::from(self.rows) * self.line_height + self.line_height / 2;
        self.under = self.end + HEIGHT;
    }

    /// Returns whether a vertex is a `!` conclusion with auxiliary doors
    /// written on the bottom row, under an edge down from its door.
    fn lowered(&self, v: u32) -> bool {
        self.view.style.net_look.conclusions == Conclusions::Bottom
            && self.view.root(v)
            && self.view.kind_of(v) == Kind::Bang
            && self.has_doors[v as usize]
    }

    /// Returns the baseline of a vertex's text.
    fn text_y(&self, v: u32) -> i64 {
        match (self.view.formulas, self.view.root(v)) {
            (true, _) if self.lowered(v) => self.baseline + i64::from(self.bottom) * self.pitch,
            (true, _) => self.baseline + i64::from(self.row[v as usize]) * self.pitch,
            (false, true) => self.under,
            (false, false) => self.baseline,
        }
    }

    /// Returns a vertex's joint, or a compact vertex's centre.
    fn node(&self, v: u32) -> (i64, i64) {
        let (row, radius) = (i64::from(self.row[v as usize]), self.radius);
        match self.view.formulas {
            true => (
                self.x[v as usize],
                self.baseline + row * self.pitch - HEIGHT - radius - radius / 2,
            ),
            false => (
                self.x[v as usize],
                self.baseline - AXIS + row * self.line_height,
            ),
        }
    }

    /// Returns the centre of a node of the rules on row `r`: where the
    /// joints of the row stand, or a compact circle of the row.
    fn centre_at(&self, r: u32) -> i64 {
        let radius = self.radius;
        match self.view.formulas {
            true => self.baseline + i64::from(r) * self.pitch - HEIGHT - radius - radius / 2,
            false => self.baseline - AXIS + i64::from(r) * self.line_height,
        }
    }

    /// Returns where an edge leaves a vertex's written formula, under it.
    fn foot(&self, v: u32) -> (i64, i64) {
        (self.x[v as usize], self.text_y(v) + self.depth)
    }

    /// Returns where an edge from a vertex below ends at `v`: a written
    /// formula's foot, or a compact circle's rim towards `to`.
    fn end_at(&self, v: u32, to: (i64, i64)) -> (i64, i64) {
        match self.view.formulas || self.view.net.forest().is_literal(self.view.occ(v)) {
            true => self.foot(v),
            false if self.bare(v) => self.node(v),
            false => towards(self.node(v), to, self.radius),
        }
    }

    /// Places the doors' points, innermost first: each on its box's bottom
    /// border, at the mean of the places of what feeds it.
    fn place_doors(&mut self) {
        let doors = self.doors.pairs.len();
        let mut feeders: Vec<Vec<i64>> = vec![Vec::new(); doors];
        let mut point = vec![(0i64, 0i64); doors];
        let mut by_depth: Vec<usize> = (0..doors).collect();
        by_depth.sort_unstable_by_key(|&d| {
            std::cmp::Reverse(self.nesting[self.doors.pairs[d].0 as usize])
        });
        for c in 0..self.n as u32 {
            if self.view.boxed(c) {
                let d = self.doors.index(self.view.inside(c), self.view.key_of(c));
                feeders[d].push(self.x[c as usize]);
            }
        }
        for &d in &by_depth {
            let (b, _) = self.doors.pairs[d];
            let xs = &feeders[d];
            let at = xs.iter().sum::<i64>() / xs.len().max(1) as i64;
            point[d] = (at, self.node(b).1);
            if let Some(outer) = self.target(d) {
                feeders[outer].push(at);
            }
        }
        self.point = point;
    }

    /// Finds the switching cycle of `verdict`, as the pairs of vertices
    /// its edges join; a wire's edge joins its `?` vertex and the copy, or
    /// the box, it comes from.
    fn find_cycle(&mut self, verdict: &Result<(), NetError>) {
        let mut witness = verdict.as_ref().err();
        while let Some(NetError::InBox { error, .. }) = witness {
            witness = Some(error);
        }
        if let Some(NetError::SwitchingCycle { cycle: vertices }) = witness {
            let next = vertices.iter().cycle().skip(1);
            let mut cycle: Vec<(u32, u32)> = vertices
                .iter()
                .zip(next)
                .map(|(&a, &b)| (a.get().min(b.get()), a.get().max(b.get())))
                .collect();
            cycle.sort_unstable();
            self.cycle = cycle;
        }
    }

    /// Returns whether the edge between `a` and `b` is marked: on the
    /// switching cycle or acted on by the step.
    fn marks(&self, a: u32, b: u32) -> bool {
        self.cycle.binary_search(&(a.min(b), a.max(b))).is_ok() || self.step.has(a, b)
    }

    /// Returns whether the edge below `?` vertex `q` is marked.
    fn marked_below(&self, q: u32) -> bool {
        self.view.parent(q) != NONE && self.marks(self.view.parent(q), q)
    }

    /// Returns the radius a door's wire stops at: none under the rules,
    /// whose doors have no symbol.
    fn door_radius(&self) -> i64 {
        if self.view.rules { 0 } else { self.radius }
    }

    /// Writes an edge of an essential net from `from` to `to`, its
    /// arrowhead halfway along it pointing at `to`.
    fn directed(&self, out: &mut String, from: (i64, i64), to: (i64, i64)) {
        writeln!(
            out,
            r#"<path d="M{} {}L{} {}"/>"#,
            from.0, from.1, to.0, to.1
        )
        .unwrap();
        arrow_along(out, from, to, self.arrow);
    }

    /// Writes an edge from `from` to `to`, dashed when `dashed`.
    fn segment(&self, out: &mut String, from: (i64, i64), to: (i64, i64), dashed: bool) {
        write!(out, r#"<path d="M{} {}L{} {}""#, from.0, from.1, to.0, to.1).unwrap();
        if dashed {
            write!(out, r#" stroke-dasharray="{}""#, self.dash).unwrap();
        }
        out.push_str("/>\n");
    }

    /// Writes the symbol `sign` of vertex `v` centred at `centre`, its
    /// middle `middle` thousandths above its baseline.
    fn label(&self, texts: &mut String, centre: (i64, i64), (sign, middle): (&str, i64), v: u32) {
        let label_size = self.label_size;
        let label = run(sign, label_size, &self.view.style.font);
        text(
            texts,
            centre.0 - label.width / 2,
            centre.1 + middle * label_size / 1000,
            &label,
            &format!(r#" font-size="{label_size}"{}"#, self.view.fill(v)),
            None,
        );
    }

    /// Writes a circle at `centre`, with its id when given.
    fn circle(&self, out: &mut String, centre: (i64, i64), id: Option<&str>) {
        let id = id.map_or(String::new(), |id| format!(r#" id="{id}""#));
        writeln!(
            out,
            r#"<circle{id} cx="{}" cy="{}" r="{}"/>"#,
            centre.0, centre.1, self.radius
        )
        .unwrap();
    }

    /// Writes a node of the rules: its label at `centre`, narrowed to fit
    /// inside the node's radius, in a circle when compact and `circled`.
    /// The last node of a `?` vertex has its own circle, with the vertex's
    /// id. Drawn as an essential net, a node is in the input colour, as a
    /// hypothesis's rules are.
    fn rule_node(
        &self,
        lines: &mut String,
        texts: &mut String,
        centre: (i64, i64),
        sign: &str,
        circled: bool,
    ) {
        let (style, radius, stroke, label_size) =
            (self.view.style, self.radius, self.stroke, self.label_size);
        let colour = match self.view.reading {
            Some(_) => format!(r#" fill="{}""#, escaped(&style.input)),
            None => String::new(),
        };
        if circled && !self.view.formulas {
            self.circle(lines, centre, None);
        }
        // The room is never negative, so a label wider than the room
        // has a positive width to divide by.
        let room = (2 * radius - 4 * stroke).max(0);
        let full = run(sign, label_size, &style.font).width;
        let size = if full > room {
            label_size * room / full
        } else {
            label_size
        };
        let label = run(sign, size, &style.font);
        text(
            texts,
            centre.0 - label.width / 2,
            centre.1 + PAR_MIDDLE * size / 1000,
            &label,
            &format!(r#" font-size="{size}"{colour}"#),
            None,
        );
    }

    /// Writes an edge of the rules from `a` down to `b`; under the reading,
    /// its arrow points up, and every edge of the rules leads up, from the
    /// `?` vertex's formula into its copies.
    fn rule_edge(&self, out: &mut String, a: (i64, i64), b: (i64, i64), dashing: bool) {
        match self.view.reading.is_some() {
            true => self.directed(out, b, a),
            false => self.segment(out, a, b, dashing),
        }
    }

    /// Writes every vertex: its formula when written, and but for a
    /// literal its circle when compact, its symbol and the edges into it.
    fn vertices(&self, g: &mut Groups) {
        let view = &self.view;
        for v in 0..self.n as u32 {
            if view.written(v) {
                self.vertex_text(v, &mut g.texts);
            }
            // Under the rules a `?` vertex is its nodes, drawn below.
            if view.net.forest().is_literal(view.occ(v))
                || (view.rules && view.kind_of(v) == Kind::Quest)
            {
                continue;
            }
            let centre = self.node(v);
            if !view.formulas {
                let group = if view.input(v) {
                    &mut g.input_lines
                } else {
                    &mut g.lines
                };
                self.circle(group, centre, Some(&view.id(v)));
            }
            self.label(&mut g.texts, centre, view.symbol_of(v), v);
            self.premise_edges(v, centre, g);
            self.hanging_edge(v, centre, g);
        }
    }

    /// Writes a vertex's formula.
    fn vertex_text(&self, v: u32, texts: &mut String) {
        let view = &self.view;
        let named = view.formulas || view.net.forest().is_literal(view.occ(v));
        let mut attributes = if named {
            format!(r#" id="{}""#, view.id(v))
        } else {
            String::new()
        };
        attributes.push_str(&view.fill(v));
        let mut formula = String::new();
        view.write(&mut formula, v, false);
        let line = run(&formula, 1000, &view.style.font);
        text(
            texts,
            self.x[v as usize] - self.from_left[v as usize],
            self.text_y(v),
            &line,
            &attributes,
            None,
        );
    }

    /// Writes the edges from vertex `v`'s joint or circle at `centre` into
    /// its premises.
    fn premise_edges(&self, v: u32, centre: (i64, i64), g: &mut Groups) {
        let (view, stroke) = (&self.view, self.stroke);
        let switched = matches!(view.kind_of(v), Kind::Par | Kind::Quest) && self.dashed;
        let children: Vec<u32> = if view.kind_of(v) == Kind::Bang {
            view.net
                .premises(VertexId::new(v))
                .map(VertexId::get)
                .collect()
        } else {
            self.kids(v as usize).collect()
        };
        for c in children {
            let to = self.end_at(c, centre);
            let from = towards(centre, to, self.radius);
            if view.reading.is_some() {
                // Output premises lead down into their connective, input
                // connectives up into their input premises (a `?` into its
                // copies), and nothing into the antecedent of an output
                // implication.
                let (upward, antecedent) = (view.input(c), view.input(c) && !view.input(v));
                let out = match (self.marks(v, c), antecedent, upward) {
                    (true, _, _) => &mut g.marked,
                    (false, true, _) => &mut g.dominator_lines,
                    (false, false, true) => &mut g.input_lines,
                    (false, false, false) => &mut g.lines,
                };
                if antecedent {
                    writeln!(
                        out,
                        r#"<path d="M{} {}L{} {}" stroke-dasharray="{stroke} {}"/>"#,
                        from.0,
                        from.1,
                        to.0,
                        to.1,
                        2 * stroke
                    )
                    .unwrap();
                } else if upward {
                    self.directed(out, from, to);
                } else {
                    self.directed(out, to, from);
                }
                continue;
            }
            let out = match (self.marks(v, c), switched) {
                (true, _) => &mut g.marked,
                (false, true) => &mut g.pars,
                (false, false) => &mut g.lines,
            };
            self.segment(out, from, to, switched && self.marks(v, c));
        }
    }

    /// Writes the edge down from a conclusion's joint or circle at
    /// `centre`, where it has one.
    fn hanging_edge(&self, v: u32, centre: (i64, i64), g: &mut Groups) {
        let (view, radius) = (&self.view, self.radius);
        // A lowered `!` conclusion's edge, down from its door to its text.
        // A `!` vertex is in output position, so under the reading this is
        // the goal's edge, and its arrow points down.
        if view.formulas && self.lowered(v) {
            let (start, foot) = (
                (centre.0, centre.1 + radius),
                (centre.0, self.text_y(v) - HEIGHT),
            );
            match view.reading {
                Some(_) => self.directed(&mut g.lines, start, foot),
                None => self.segment(&mut g.lines, start, foot, false),
            }
        }
        // A compact conclusion's edge hangs from its circle to its text.
        if view.root(v) && !view.formulas {
            let end = self.end;
            let (start, foot) = ((centre.0, centre.1 + radius), (centre.0, end));
            match (view.reading, view.input(v)) {
                (Some(_), true) => self.directed(&mut g.input_lines, foot, start),
                (Some(_), false) => self.directed(&mut g.lines, start, foot),
                (None, _) => {
                    writeln!(g.lines, r#"<path d="M{} {}V{end}"/>"#, start.0, start.1).unwrap();
                }
            }
        }
    }

    /// Writes, under the rules, every `?` vertex's nodes, and returns
    /// where each of its wires ends, by door (the node, how far from it,
    /// and whether from above):
    /// - a dereliction under each copy at its depth, on the first of its
    ///   rows;
    /// - the contractions in a column over its formula, one a row, joining
    ///   the copies first and then the wires, each nearest the column
    ///   first;
    /// - the last node over its formula, or its circle when compact;
    /// - a weakening alone, or a lone wire straight into its formula.
    ///
    /// A wire's leg runs level between two rows, as the default's wires do.
    /// Level with a row of nodes, it would pass through other `?` vertices'
    /// nodes on that row. A side premise comes into its contraction along
    /// twice the radius above it. The first wire, with wires alone, comes
    /// along twice the radius above the landing row and down the column. So
    /// the two premises of the first contraction never share a leg.
    fn rule_nodes(&self, g: &mut Groups) -> HashMap<usize, End> {
        let mut entry: HashMap<usize, End> = HashMap::default();
        for q in
            (0..self.n as u32).filter(|&q| self.view.rules && self.view.kind_of(q) == Kind::Quest)
        {
            self.quest_nodes(q, g, &mut entry);
        }
        entry
    }

    /// Returns the doors whose wires go to `?` vertex `q` itself.
    fn wires_into(&self, q: u32) -> impl Iterator<Item = (usize, &(u32, u32))> {
        self.doors
            .pairs
            .iter()
            .enumerate()
            .filter(move |&(d, &(_, key))| self.view.owner(key) == q && self.target(d).is_none())
    }

    /// Writes a compact conclusion's edge under the rules, from `from` down
    /// to the text of `?` vertex `q`, into the highlight when `down`.
    fn hanging(&self, q: u32, down: bool, lines: &mut String, marked: &mut String, from: i64) {
        let target = if down { marked } else { lines };
        let (column, end) = (self.x[q as usize], self.end);
        match self.view.reading.is_some() {
            true => self.directed(target, (column, end), (column, from)),
            false => writeln!(target, r#"<path d="M{column} {from}V{end}"/>"#).unwrap(),
        }
    }

    /// Writes the nodes of `?` vertex `q` under the rules, and records in
    /// `entry` where its wires end.
    fn quest_nodes(&self, q: u32, g: &mut Groups, entry: &mut HashMap<usize, End>) {
        let (view, radius) = (&self.view, self.radius);
        let down = self.marked_below(q);
        let (rule_lines, marked, texts, _) = g.rules(view.reading.is_some());
        if self.bare(q) {
            let into = match view.formulas {
                true => (self.x[q as usize], self.text_y(q) - HEIGHT),
                false => self.node(q),
            };
            for (d, _) in self.wires_into(q) {
                let above = Some(self.node(q).1 - 2 * radius);
                entry.insert(
                    d,
                    End {
                        at: into,
                        into: 0,
                        above,
                    },
                );
            }
            if !view.formulas && view.root(q) {
                self.hanging(q, down, rule_lines, marked, into.1);
            }
            return;
        }
        let last = match view.formulas {
            true => (self.x[q as usize], self.centre_at(self.row[q as usize] - 1)),
            false => self.node(q),
        };
        if view.formulas {
            let into = (last.0, self.text_y(q) - HEIGHT);
            let target = if down { &mut *marked } else { &mut *rule_lines };
            self.rule_edge(target, (last.0, last.1 + radius), into, false);
        } else {
            writeln!(
                rule_lines,
                r#"<circle id="{}" cx="{}" cy="{}" r="{radius}"/>"#,
                view.id(q),
                last.0,
                last.1
            )
            .unwrap();
            if view.root(q) {
                self.hanging(q, down, rule_lines, marked, last.1 + radius);
            }
        }
        if self.inputs[q as usize] == 0 {
            self.rule_node(rule_lines, texts, last, &self.weakening, false);
            return;
        }
        self.contractions(q, last, g, entry);
    }

    /// Writes the derelictions and contractions of `?` vertex `q` under the
    /// rules, its last node at `last`, and records in `entry` where its
    /// wires end.
    fn contractions(
        &self,
        q: u32,
        last: (i64, i64),
        g: &mut Groups,
        entry: &mut HashMap<usize, End>,
    ) {
        let (view, radius, dashed) = (&self.view, self.radius, self.dashed);
        // The premises by their distance from the column, each with its
        // place and whether the cycle runs through it.
        let mut premises: Vec<(i64, Option<u32>, Option<usize>, bool)> = self
            .kids(q as usize)
            .map(|c| (self.x[c as usize], Some(c), None, self.marks(q, c)))
            .chain(
                self.wires_into(q)
                    .map(|(d, &(b, _))| (self.point[d].0, None, Some(d), self.marks(q, b))),
            )
            .collect();
        let column = self.x[q as usize];
        premises.sort_unstable_by_key(|&(at, copy, ..)| (copy.is_none(), (at - column).abs(), at));
        let first = self.row[q as usize] + u32::from(!view.formulas) - self.levels[q as usize];
        let k = self.inputs[q as usize] as usize;
        let comb = Comb {
            column,
            first,
            k,
            last,
        };
        // The contractions the marks run through: from the first marked
        // premise's to the last's, or down to the formula when the edge
        // below the `?` vertex is marked.
        let on: Vec<usize> = premises
            .iter()
            .enumerate()
            .filter(|p| p.1.3)
            .map(|(i, _)| i.max(1))
            .collect();
        let from_on = on.iter().copied().min().unwrap_or(k - 1);
        let to_on = if self.marked_below(q) {
            k - 1
        } else {
            on.iter().copied().max().unwrap_or(0)
        };
        let (rule_lines, marked, texts, pars) = g.rules(view.reading.is_some());
        for (i, &(_, copy, wire, mark)) in premises.iter().enumerate() {
            let into = self.contraction_at(&comb, i.max(1));
            if let Some(d) = wire {
                let above = (i == 0).then(|| self.centre_at(first) - 2 * radius);
                entry.insert(
                    d,
                    End {
                        at: into,
                        into: radius,
                        above,
                    },
                );
                continue;
            }
            let c = copy.expect("a copy or a wire");
            let at = match k {
                1 => last,
                _ => (self.x[c as usize], self.centre_at(first)),
            };
            self.rule_node(&mut *rule_lines, &mut *texts, at, &self.dereliction, k > 1);
            let from = self.end_at(c, at);
            let target = if mark { &mut *marked } else { &mut *rule_lines };
            self.rule_edge(target, from, towards(at, from, radius), false);
            if k > 1 {
                let target = match (mark, dashed) {
                    (true, _) => &mut *marked,
                    (false, true) => &mut *pars,
                    (false, false) => &mut *rule_lines,
                };
                self.rule_edge(
                    target,
                    towards(at, into, radius),
                    towards(into, at, radius),
                    mark && dashed,
                );
            }
        }
        self.contraction_column(&comb, from_on, to_on, g);
    }

    /// Returns where contraction `j` of a column stands. Counted from 1, it
    /// takes premise `j` and what is above it in the column; contraction 1
    /// takes premises 0 and 1. The last contraction is the `?` vertex's
    /// last node.
    fn contraction_at(&self, comb: &Comb, j: usize) -> (i64, i64) {
        match j + 1 == comb.k {
            true => comb.last,
            false => (comb.column, self.centre_at(comb.first + j as u32)),
        }
    }

    /// Writes the contractions of a column, one a row, and the edges
    /// between them, marked from contraction `from_on` to `to_on`.
    fn contraction_column(&self, comb: &Comb, from_on: usize, to_on: usize, g: &mut Groups) {
        let (radius, dashed, k) = (self.radius, self.dashed, comb.k);
        let (rule_lines, marked, texts, pars) = g.rules(self.view.reading.is_some());
        for j in 1..k {
            let at = self.contraction_at(comb, j);
            self.rule_node(
                &mut *rule_lines,
                &mut *texts,
                at,
                &self.contraction,
                j + 1 < k,
            );
            if j + 1 < k {
                let below = self.contraction_at(comb, j + 1);
                let mark = from_on <= j && j < to_on;
                let target = match (mark, dashed) {
                    (true, _) => &mut *marked,
                    (false, true) => &mut *pars,
                    (false, false) => &mut *rule_lines,
                };
                self.rule_edge(
                    target,
                    (comb.column, at.1 + radius),
                    (comb.column, below.1 - radius),
                    mark && dashed,
                );
            }
        }
    }

    /// Writes the wire from each boxed copy into its door. Under the rules
    /// a copy's wire starts at its dereliction, a row below it in its box,
    /// and runs through its doors, which have no symbol of their own.
    fn wires(&self, g: &mut Groups) {
        let (view, radius) = (&self.view, self.radius);
        let door_radius = self.door_radius();
        for c in 0..self.n as u32 {
            if !view.boxed(c) {
                continue;
            }
            let q = view.parent(c);
            let d = self.doors.index(view.inside(c), view.key_of(c));
            let to = self.point[d];
            let switched = self.dashed && !self.bare(q);
            let (plain, marked, texts, pars) = g.rules(view.reading.is_some());
            let from = if view.rules {
                let at = (self.x[c as usize], self.centre_at(self.row[c as usize] + 1));
                let from = self.end_at(c, at);
                let target = if self.marks(q, c) {
                    &mut *marked
                } else {
                    &mut *plain
                };
                self.rule_edge(target, from, towards(at, from, radius), false);
                self.rule_node(&mut *plain, texts, at, &self.dereliction, true);
                towards(at, to, radius)
            } else {
                self.end_at(c, to)
            };
            let target = match (self.marks(q, c), switched) {
                (true, _) => marked,
                (false, true) => pars,
                (false, false) => plain,
            };
            self.rule_edge(
                target,
                from,
                towards(to, from, door_radius),
                self.marks(q, c) && switched,
            );
        }
    }

    /// Returns the legs of the wires from the doors, by door. The legs in
    /// one band are staggered so that no two share one, whether they go to
    /// different `?` vertices or to the doors of a box around. Going left,
    /// the wire whose door is further left runs higher; going right, the
    /// one further right. So where two spans overlap, one wire's way down
    /// does not cross the other's leg. The legs stand at most the radius
    /// apart, within the room below what stands on the row above. `entry`
    /// is where each wire ends under the rules.
    fn legs(&self, entry: &HashMap<usize, End>) -> HashMap<usize, i64> {
        let (view, radius) = (&self.view, self.radius);
        let mut legs: HashMap<usize, i64> = HashMap::default();
        let mut bands: HashMap<i64, Vec<(usize, i64, i64)>> = HashMap::default();
        for d in (0..self.doors.pairs.len()).filter(|&d| !view.rules || self.target(d).is_none()) {
            let (to, band) = match self.target(d) {
                Some(outer) => (self.point[outer], self.point[outer].1 - 2 * radius),
                None if view.rules => {
                    let End { at: to, above, .. } = entry[&d];
                    (to, above.unwrap_or(to.1 - 2 * radius))
                }
                None => {
                    let to = self.node(view.owner(self.doors.pairs[d].1));
                    (to, to.1 - 2 * radius)
                }
            };
            bands
                .entry(band)
                .or_default()
                .push((d, self.point[d].0, to.0));
        }
        let room = match (view.formulas, view.rules) {
            (true, true) => self.pitch - 3 * radius,
            _ => self.line_height - self.room_beside,
        };
        for (band, mut wires) in bands {
            wires.sort_unstable_by_key(|&(_, from, to)| match from > to {
                true => (false, from),
                false => (true, -from),
            });
            let step = radius.min(room / wires.len() as i64);
            for (i, &(d, ..)) in wires.iter().enumerate() {
                legs.insert(d, band - (wires.len() - 1 - i) as i64 * step);
            }
        }
        legs
    }

    /// Writes the wire from each door to the door of the box around it or
    /// to its `?` vertex, and by default the door's circle when compact
    /// and its symbol. `entry` is where each wire ends under the rules,
    /// `legs` the legs of the wires.
    fn door_wires(&self, g: &mut Groups, entry: &HashMap<usize, End>, legs: &HashMap<usize, i64>) {
        let (view, radius) = (&self.view, self.radius);
        let door_radius = self.door_radius();
        for (d, &(b, key)) in self.doors.pairs.iter().enumerate() {
            let q = view.owner(key);
            let from = self.point[d];
            let (to, into, above) = match self.target(d) {
                Some(outer) => (self.point[outer], door_radius, None),
                None if view.rules => {
                    let End {
                        at: to,
                        into,
                        above,
                    } = entry[&d];
                    (to, into, Some(above))
                }
                None => (self.node(q), radius, None),
            };
            let mark = self.marks(q, b);
            let switched = self.dashed && !self.bare(q);
            let out = match (mark, switched, view.reading.is_some()) {
                (true, _, _) => &mut g.marked,
                (false, true, _) => &mut g.pars,
                (false, false, true) => &mut g.input_lines,
                (false, false, false) => &mut g.lines,
            };
            let dashing = mark && switched;
            if let Some(Some(_)) = above {
                self.rule_wire(out, from, (to, into), legs[&d], dashing);
                continue;
            }
            let leg = legs.get(&d).copied().unwrap_or(to.1 - 2 * radius);
            self.door_wire(out, from, (to, into), leg, dashing);
            if view.rules {
                continue;
            }
            if !view.formulas {
                let group = if view.reading.is_some() {
                    &mut g.input_lines
                } else {
                    &mut g.lines
                };
                self.circle(group, from, None);
            }
            self.label(&mut g.texts, from, view.symbol_of(q), q);
        }
    }

    /// Writes a wire of the rules from the door at `from`: down from the
    /// door, along its leg to the column and down into a node of the rules
    /// or a formula at `to`, stopping `into` from it; under the reading led
    /// up, the arrowhead on its way down from the door.
    fn rule_wire(
        &self,
        out: &mut String,
        from: (i64, i64),
        (to, into): ((i64, i64), i64),
        leg: i64,
        dashing: bool,
    ) {
        write!(out, r#"<path d="M{} {}"#, from.0, from.1).unwrap();
        if from.0 != to.0 {
            write!(out, "V{leg}H{}", to.0).unwrap();
        }
        write!(out, "V{}", to.1 - into).unwrap();
        out.push('"');
        if dashing {
            write!(out, r#" stroke-dasharray="{}""#, self.dash).unwrap();
        }
        out.push_str("/>\n");
        if self.view.reading.is_some() {
            let down = if from.0 == to.0 { to.1 - into } else { leg };
            arrow_along(out, (from.0, down), from, self.arrow);
        }
    }

    /// Writes a door's wire from the door at `from`: straight down from the
    /// door, along its `leg`, twice the radius above the row of what it
    /// goes to, and into it at `to`, stopping `into` from it, from the side
    /// it comes from, so that it crosses no joint or circle on the way.
    fn door_wire(
        &self,
        out: &mut String,
        from: (i64, i64),
        (to, into): ((i64, i64), i64),
        leg: i64,
        dashing: bool,
    ) {
        let (radius, door_radius) = (self.radius, self.door_radius());
        if (from.0 - to.0).abs() <= 2 * radius || leg <= from.1 + radius {
            let (start, end) = (towards(from, to, door_radius), towards(to, from, into));
            match self.view.reading {
                Some(_) => self.directed(out, end, start),
                None => self.segment(out, start, end, dashing),
            }
            return;
        }
        let side = (from.0 - to.0).signum() * 2 * radius;
        let corner = (to.0 + side, leg);
        let end = towards(to, corner, into);
        write!(
            out,
            r#"<path d="M{} {}V{leg}H{}L{} {}""#,
            from.0,
            from.1 + door_radius,
            corner.0,
            end.0,
            end.1
        )
        .unwrap();
        if dashing {
            write!(out, r#" stroke-dasharray="{}""#, self.dash).unwrap();
        }
        out.push_str("/>\n");
        // Under the reading led up, the arrowhead on the longer of its
        // way down from the door and its way along.
        if self.view.reading.is_some() {
            let start = (from.0, from.1 + door_radius);
            match leg - start.1 >= (corner.0 - from.0).abs() {
                true => arrow_along(out, (from.0, leg), start, self.arrow),
                false => arrow_along(out, (corner.0, leg), (from.0, leg), self.arrow),
            }
        }
    }

    /// Writes each box's rectangle, its bottom border broken at its doors.
    fn boxes(&self, g: &mut Groups) {
        let radius = self.radius;
        let pairs = &self.doors.pairs;
        for b in self.view.net.boxes().map(VertexId::get) {
            let (left, right) = self.sides(b);
            let bottom = self.node(b).1;
            let ceiling = self.top - self.lid[b as usize];
            let mut gaps: Vec<i64> = vec![self.x[b as usize]];
            // Under the rules a wire crosses the border, which has no door.
            if !self.view.rules {
                gaps.extend(
                    pairs[pairs.partition_point(|&(c, _)| c < b)..]
                        .iter()
                        .take_while(|&&(c, _)| c == b)
                        .map(|&(c, q)| self.point[self.doors.index(c, q)].0),
                );
            }
            gaps.sort_unstable();
            let mut at = gaps[0] + radius;
            let out = match self.step.boxes.contains(&b) {
                true => &mut g.marked,
                false => &mut g.lines,
            };
            write!(out, r#"<path id="b{b}" d="M{at} {bottom}"#).unwrap();
            for &door in &gaps[1..] {
                if door - radius > at {
                    write!(out, "H{}", door - radius).unwrap();
                }
                at = at.max(door + radius);
                write!(out, "M{at} {bottom}").unwrap();
            }
            writeln!(
                out,
                r#"H{right}V{ceiling}H{left}V{bottom}H{}"/>"#,
                gaps[0] - radius
            )
            .unwrap();
        }
    }

    /// Writes each axiom link's arc, raised over the boxes it spans.
    fn links(&self, g: &mut Groups) {
        for &(p, q) in self.view.net.links() {
            let (p, q) = (p.get(), q.get());
            let (a, b, rx, ry) = self.arc((p, q));
            let rise = self.lift.get(&(p, q)).copied().unwrap_or(0);
            let out = if self.marks(p, q) {
                &mut g.marked
            } else {
                &mut g.links
            };
            let (left, right) = if self.x[p as usize] < self.x[q as usize] {
                (p, q)
            } else {
                (q, p)
            };
            let down = (self.text_y(left) - HEIGHT, self.text_y(right) - HEIGHT);
            // From the input literal to the output one, at the arc's top.
            let head = self
                .view
                .reading
                .is_some()
                .then(|| (left == if self.view.input(p) { q } else { p }, self.arrow));
            link_arc(
                out,
                (p.min(q), p.max(q)),
                (a, b, rx, ry),
                down,
                self.top - rise,
                head,
            );
        }
    }

    /// Writes each cut link as the literature draws one: a bar under its
    /// two formulas, joined to them from below.
    fn cut_bars(&self, g: &mut Groups) {
        let (view, radius) = (&self.view, self.radius);
        let cut_depth = 2 * radius;
        for &(a, d) in &self.raw_cuts {
            let below = |v: u32| match view.formulas
                || view.net.forest().is_literal(view.occ(v))
                || view.root(v)
            {
                true => self.foot(v).1,
                false => self.node(v).1 + radius,
            };
            let low = below(a).max(below(d)) + cut_depth;
            let out = match self.step.cut == Some((a, d)) {
                true => &mut g.marked,
                false => &mut g.lines,
            };
            cut_bar(
                out,
                (a, d),
                (self.x[a as usize], below(a)),
                (self.x[d as usize], below(d)),
                low,
            );
        }
    }

    /// Returns the drawing's width and height.
    fn size(&self) -> (i64, i64) {
        let width = if self.tops.is_empty() {
            2 * self.margin
        } else {
            self.right + self.margin
        };
        // Room under the bottom row for the bars of the cut links there.
        let cut_depth = 2 * self.radius;
        let cuts = if self.tops.len() > self.view.net.forest().roots().len() {
            self.depth.max(DEPTH) + cut_depth + self.stroke
        } else {
            DEPTH
        };
        let height = match self.view.formulas {
            true => self.baseline + i64::from(self.rows) * self.pitch + cuts + self.margin,
            false => self.under + cuts + self.margin,
        };
        (width, height)
    }
}

/// Returns a proof structure with boxes as an SVG document, titled with
/// its sequent in plain text.
///
/// `verdict` is the structure's [`MellStructure::is_correct`], whose
/// switching cycle is drawn in the highlight colour. `step` marks what a
/// step of cut elimination acted on, in the same colour.
///
/// With the `reading` of its sequent, the structure is drawn as its
/// essential net:
/// - the reading's formulas and connectives;
/// - the input occurrences and everything into them in the input colour;
/// - every edge, wire and link an arrow from the hypotheses toward the
///   goal;
/// - the edge into an implication's antecedent dotted.
pub(super) fn draw(
    net: &MellStructure,
    style: &Style,
    verdict: &Result<(), NetError>,
    reading: Option<&Reading<'_>>,
    step: &Marks,
) -> String {
    let layout = Layout::of(net, style, verdict, reading, step);
    let mut g = Groups::default();
    layout.vertices(&mut g);
    let entry = layout.rule_nodes(&mut g);
    layout.wires(&mut g);
    let legs = layout.legs(&entry);
    layout.door_wires(&mut g, &entry, &legs);
    layout.boxes(&mut g);
    layout.links(&mut g);
    layout.cut_bars(&mut g);
    let description = style.description.then(|| {
        let mut text = String::new();
        describe(net, verdict, reading, &mut text).unwrap();
        text
    });
    let groups = NetGroups {
        lines: &g.lines,
        inputs: &g.input_lines,
        dominators: &g.dominator_lines,
        pars: &g.pars,
        links: &g.links,
        marked: &g.marked,
        texts: &g.texts,
    };
    net_document(
        style,
        net.forest().sequent(),
        reading,
        layout.stroke,
        layout.dashed,
        &groups,
        description.as_deref(),
        layout.size(),
    )
}

/// Returns a sparse table of `values`: `table[k][i]` is the largest of
/// `values[i..i + 2^k]`.
fn runs(values: &[i64]) -> Vec<Vec<i64>> {
    let mut table = vec![values.to_vec()];
    let mut span = 1;
    while 2 * span <= values.len() {
        let last = &table[table.len() - 1];
        let next = (0..=values.len() - 2 * span)
            .map(|i| last[i].max(last[i + span]))
            .collect();
        table.push(next);
        span *= 2;
    }
    table
}

/// Returns the largest of `values[from..to]` from its table of [`runs`],
/// or 0 for an empty range.
fn most(table: &[Vec<i64>], from: usize, to: usize) -> i64 {
    if from >= to {
        return 0;
    }
    let k = (to - from).ilog2() as usize;
    table[k][from].max(table[k][to - (1 << k)])
}
