// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A derivation in a calculus given as data, laid out as a Fitch proof:
//! numbered lines, each subproof's lines together under its hypotheses.

use super::derivation::Kind;
use super::{Calculus, Derivation, NodeId, Step};
use crate::hash::{HashMap, HashSet};
use crate::limits::Refusal;
use crate::{Error, Limits};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::{self, Display, Formatter, Write};

/// What the estimate counts per line, in bytes: its number, its
/// justification and the layout's numbers.
const LINE: u64 = 256;

/// What the estimate counts per member of a line's sequent, in bytes: a
/// short formula's text.
const MEMBER: u64 = 64;

/// The choices of a Fitch proof's text, whatever format writes it.
///
/// Start from `FitchOptions::default()` and change the fields you want,
/// by the `with_*` builders or by assignment. With serde a JSON object
/// with some of the fields reads as the defaults with those changed.
///
/// Needs the cargo feature `serialize`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FitchOptions {
    /// What a hypothesis's line gives as its justification: nothing by
    /// default, as Fitch and the LaTeX package `fitch` write it; some
    /// textbooks write `Hyp` or `PR`.
    pub hypothesis: String,
    /// What an open line, a goal of a proof in progress, gives as its
    /// justification: `?` by default.
    pub open: String,
}

impl Default for FitchOptions {
    /// No justification beside a hypothesis, and `?` beside an open line.
    fn default() -> Self {
        Self {
            hypothesis: String::new(),
            open: "?".to_owned(),
        }
    }
}

impl FitchOptions {
    /// Returns the options with another [`hypothesis`](Self::hypothesis).
    #[must_use]
    pub fn with_hypothesis(self, hypothesis: impl Into<String>) -> Self {
        Self {
            hypothesis: hypothesis.into(),
            ..self
        }
    }

    /// Returns the options with another [`open`](Self::open).
    #[must_use]
    pub fn with_open(self, open: impl Into<String>) -> Self {
        Self {
            open: open.into(),
            ..self
        }
    }
}

/// A line cited by another: one line, or a subproof by its first and its
/// last line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Cite {
    /// The line of this number.
    Line(u32),
    /// The subproof from the first number to the second.
    Range(u32, u32),
}

/// A line of a Fitch proof.
#[derive(Clone, Debug)]
pub(crate) struct Line {
    /// The node it shows.
    pub(crate) node: NodeId,
    /// How many subproofs it lies in: 0 in the main proof.
    pub(crate) depth: u32,
    /// The lines and subproofs it is justified by, in its rule's order.
    pub(crate) cites: Vec<Cite>,
}

/// A proof or subproof: its lines, by index, and its hypotheses first.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Block {
    /// How many subproofs it lies in: 0 for the main proof.
    pub(crate) depth: u32,
    /// Its first line.
    pub(crate) first: u32,
    /// How many of its lines, the first ones, are its hypotheses.
    pub(crate) hypotheses: u32,
    /// Its last line.
    pub(crate) last: u32,
}

/// A derivation in a calculus given as data, with the calculus that
/// writes its formulas, as a Fitch proof: numbered lines, each a formula
/// and its justification, the rule and the lines it follows from. A
/// subproof's lines stand together under its hypotheses, which a bar
/// closes, and a line beside them runs from its first line to its last.
/// A rule that discharges cites the subproof by its first and last line,
/// `→I 2–4`; the subproof's last line is the premise it discharges in.
///
/// Every export takes it as it takes a derivation:
/// `export::svg::to_string(&fitch, &style)`, `latex` (the package `fitch`)
/// and `typst` (the package `fitchcraft`); `Display` writes it as text.
///
/// The lines need not be in the derivation's order: each subproof's lines
/// are put together, its hypotheses first and its conclusion last, every
/// line after the lines it cites, and otherwise in the derivation's
/// order. A derivation where no such order exists, a subproof whose
/// conclusion a line inside it uses, or one that two rules conclude
/// differently, is refused ([`Error::NotFitch`]): it can still be drawn
/// as a tree ([`Drawing`](super::Drawing)). A node used by several lines is
/// one line, cited by each.
///
/// Needs the cargo feature `serialize`.
#[derive(Clone, Debug)]
pub struct Fitch<'a> {
    /// The calculus whose print forms write the formulas.
    calculus: &'a Calculus,
    /// The derivation.
    derivation: &'a Derivation,
    /// The choices of the text.
    options: FitchOptions,
    /// The lines in order.
    lines: Vec<Line>,
    /// The main proof first, then every subproof in the order it opens.
    blocks: Vec<Block>,
}

/// A part of a proof or subproof as the ordering moves it: a line, or a
/// subproof inside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Unit {
    /// The line of this node.
    Node(u32),
    /// The subproof of this scope.
    Block(u32),
}

impl<'a> Fitch<'a> {
    /// Returns the derivation laid out as a Fitch proof, its formulas
    /// written by `calculus` and its text chosen by `options`.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidDerivation`] for a derivation without nodes, or
    /// whose sequents have other zones than the calculus's;
    /// [`Error::NotFitch`] for one no order writes as a Fitch proof, with
    /// the reason; [`Refusal::Output`] when its text is estimated past
    /// `limits.derivation_bytes`, and [`Refusal::Work`] past `limits.work`.
    pub fn new(
        calculus: &'a Calculus,
        derivation: &'a Derivation,
        options: &FitchOptions,
        limits: &Limits,
    ) -> Result<Self, Error> {
        let invalid = |reason: String| Error::InvalidDerivation {
            calculus: calculus.name().into(),
            reason,
        };
        if derivation.zones() != calculus.zones.len() {
            return Err(invalid(format!(
                "its sequents have {} zones, the calculus's {}",
                derivation.zones(),
                calculus.zones.len()
            )));
        }
        let Some(root) = derivation.root() else {
            return Err(invalid("it has no node".to_owned()));
        };
        let refused = |reason: String| Error::NotFitch { reason };
        // The text's size, before anything is laid out.
        let mut estimate = 0u64;
        for node in derivation.nodes() {
            let members = (0..derivation.zones()).map(|z| node.zone(z).len() as u64);
            estimate = estimate
                .saturating_add(LINE)
                .saturating_add(MEMBER.saturating_mul(members.sum()));
        }
        if let Some(limit) = limits.derivation_bytes
            && estimate > limit
        {
            return Err(Error::Refused(Refusal::Output {
                what: "Fitch proof",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            }));
        }
        let count = derivation.len();
        let parents = derivation.scope_parents();
        let id = |node: u32| derivation.at(node as usize).id();
        let scope = |node: u32| derivation.at(node as usize).scope().index() as u32;
        // A scope's depth; a parent is made before its children.
        let mut depth = vec![0u32; parents.len()];
        for s in 1..parents.len() {
            depth[s] = depth[parents[s] as usize] + 1;
        }
        let mut climb = Climb {
            parents,
            depth: &depth,
            work: 0,
            limit: limits.work,
        };
        let root_index = root.id().index() as u32;
        if scope(root_index) != 0 {
            return Err(refused(
                "the last node lies inside a subproof, not in the main proof".to_owned(),
            ));
        }

        // Each scope's parts, what each part waits for, and the line each
        // subproof concludes.
        let mut parts: Vec<Vec<Unit>> = vec![Vec::new(); parents.len()];
        let mut seen: HashSet<Unit> = HashSet::default();
        for node in 0..count as u32 {
            let mut level = scope(node);
            let mut part = Unit::Node(node);
            // The node, and every subproof around it up to the main proof,
            // each a part of the one around it.
            loop {
                if !seen.insert(part) {
                    break;
                }
                parts[level as usize].push(part);
                if level == 0 {
                    break;
                }
                climb.tick()?;
                part = Unit::Block(level);
                level = parents[level as usize];
            }
        }
        let mut waits: HashMap<Unit, Vec<Unit>> = HashMap::default();
        let mut owed: HashMap<Unit, u32> = HashMap::default();
        let mut concludes: Vec<Option<u32>> = vec![None; parents.len()];
        for node in 0..count as u32 {
            let at = scope(node);
            for &premise in derivation.at(node as usize).premises() {
                let premise = premise.index() as u32;
                let there = scope(premise);
                // A premise in the scope or one around it, or a subproof
                // directly inside one of those, which the node discharges.
                let (level, from) = if climb.inside(at, there)?.is_some() || at == there {
                    (there, Unit::Node(premise))
                } else if there != 0
                    && (parents[there as usize] == at
                        || climb.inside(at, parents[there as usize])?.is_some())
                {
                    match concludes[there as usize] {
                        Some(other) if other != premise => {
                            return Err(refused(format!(
                                "two rules take the subproof of node {} with different \
                                 conclusions, nodes {} and {}",
                                first_node(derivation, there),
                                other,
                                premise
                            )));
                        }
                        _ => concludes[there as usize] = Some(premise),
                    }
                    (parents[there as usize], Unit::Block(there))
                } else {
                    return Err(refused(format!(
                        "node {node} cites node {premise}, which lies in a subproof out of \
                         its reach"
                    )));
                };
                let Some(to) = climb.unit(scope(node), node, level)? else {
                    return Err(refused(format!(
                        "node {node} cites node {premise}, which lies in a subproof out of \
                         its reach"
                    )));
                };
                if to != from {
                    let list = waits.entry(from).or_default();
                    if !list.contains(&to) {
                        list.push(to);
                        *owed.entry(to).or_default() += 1;
                    }
                }
            }
        }

        // Each scope's order: its hypotheses first, its conclusion last,
        // every part after what it waits for, and otherwise by the
        // derivation's order.
        let key = |part: Unit| match part {
            Unit::Node(node) => node,
            Unit::Block(s) => first_node(derivation, s),
        };
        let hypothesis = |part: Unit| matches!(part, Unit::Node(n) if derivation.nodes[n as usize].kind == Kind::Hypothesis);
        let mut orders: Vec<Vec<Unit>> = vec![Vec::new(); parents.len()];
        for (level, list) in parts.iter().enumerate() {
            if list.is_empty() {
                continue;
            }
            let last = if level == 0 {
                Some(Unit::Node(root_index))
            } else {
                concludes[level].map(Unit::Node)
            };
            let mut ready: BinaryHeap<Reverse<(bool, u32, Unit)>> = BinaryHeap::new();
            for &part in list {
                if owed.get(&part).copied().unwrap_or(0) == 0 && Some(part) != last {
                    ready.push(Reverse((!hypothesis(part), key(part), part)));
                }
            }
            let order = &mut orders[level];
            while let Some(Reverse((_, _, part))) = ready.pop() {
                climb.tick()?;
                order.push(part);
                for &next in waits.get(&part).map_or(&[][..], Vec::as_slice) {
                    let left = owed.entry(next).or_default();
                    *left -= 1;
                    if *left == 0 && Some(next) != last {
                        ready.push(Reverse((!hypothesis(next), key(next), next)));
                    }
                }
            }
            if let Some(last) = last
                && owed.get(&last).copied().unwrap_or(0) == 0
                && !order.contains(&last)
            {
                order.push(last);
            }
            if order.len() != list.len() {
                let what = if level == 0 {
                    "the main proof".to_owned()
                } else {
                    format!(
                        "the subproof of node {}",
                        first_node(derivation, level as u32)
                    )
                };
                return Err(refused(format!(
                    "no order of {what} puts every line after the lines it cites and the \
                     line it concludes last: a line inside it uses its conclusion, or it \
                     uses a subproof that uses it"
                )));
            }
            // Hypotheses after another line cannot open a subproof.
            let opening = order.iter().take_while(|&&part| hypothesis(part)).count();
            if order.iter().skip(opening).any(|&part| hypothesis(part)) {
                return Err(refused(format!(
                    "a hypothesis of {} cannot stand among its first lines",
                    if level == 0 {
                        "the main proof".to_owned()
                    } else {
                        format!(
                            "the subproof of node {}",
                            first_node(derivation, level as u32)
                        )
                    }
                )));
            }
        }

        // The lines, each subproof's in place, by a stack of its own.
        let mut lines: Vec<Line> = Vec::with_capacity(count);
        let mut blocks: Vec<Block> = Vec::new();
        let mut number = vec![0u32; count];
        let mut spans = vec![(0u32, 0u32); parents.len()];
        let mut stack: Vec<(u32, usize, usize)> = vec![(0, 0, 0)];
        blocks.push(Block {
            depth: 0,
            first: 0,
            hypotheses: 0,
            last: 0,
        });
        while let Some(&mut (level, ref mut at, block)) = stack.last_mut() {
            let order = &orders[level as usize];
            let Some(&part) = order.get(*at) else {
                let last = lines.len() as u32 - 1;
                blocks[block].last = last;
                spans[level as usize] = (blocks[block].first, last);
                stack.pop();
                continue;
            };
            *at += 1;
            match part {
                Unit::Node(node) => {
                    if hypothesis(part) {
                        blocks[block].hypotheses += 1;
                    }
                    number[node as usize] = lines.len() as u32 + 1;
                    lines.push(Line {
                        node: id(node),
                        depth: depth[level as usize],
                        cites: Vec::new(),
                    });
                }
                Unit::Block(inner) => {
                    blocks.push(Block {
                        depth: depth[inner as usize],
                        first: lines.len() as u32,
                        hypotheses: 0,
                        last: lines.len() as u32,
                    });
                    stack.push((inner, 0, blocks.len() - 1));
                }
            }
        }
        for line in &mut lines {
            let at = scope(line.node.index() as u32);
            for &premise in derivation.node(line.node).premises() {
                let there = scope(premise.index() as u32);
                let cite = if there == at || climb.inside(at, there)?.is_some() {
                    Cite::Line(number[premise.index()])
                } else {
                    let (first, last) = spans[there as usize];
                    Cite::Range(first + 1, last + 1)
                };
                line.cites.push(cite);
            }
        }
        Ok(Self {
            calculus,
            derivation,
            options: options.clone(),
            lines,
            blocks,
        })
    }

    /// Returns how many lines it has.
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Returns whether it has no line, which a derivation's Fitch proof
    /// never is.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Returns the node each line shows, in the lines' order: line `n`,
    /// counted from 1, shows the `n - 1`th.
    pub fn nodes(&self) -> impl ExactSizeIterator<Item = NodeId> + '_ {
        self.lines.iter().map(|line| line.node)
    }

    /// Returns the lines.
    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    pub(crate) fn lines(&self) -> &[Line] {
        &self.lines
    }

    /// Returns the main proof, then every subproof in the order it opens.
    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    pub(crate) fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Returns how a line is justified: its rule's label in the label
    /// markup, `None` for a hypothesis and an open line.
    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    pub(crate) fn rule(&self, line: &Line) -> Option<String> {
        match self.derivation.node(line.node).step() {
            Step::Rule(name) => Some(super::draw::markup(name)),
            Step::Hypothesis | Step::Open => None,
        }
    }

    /// Returns what a line that cites no rule gives as its justification:
    /// the options' text for a hypothesis or an open line, empty for a
    /// rule's.
    pub(crate) fn note(&self, line: &Line) -> &str {
        match self.derivation.node(line.node).step() {
            Step::Rule(_) => "",
            Step::Hypothesis => &self.options.hypothesis,
            Step::Open => &self.options.open,
        }
    }

    /// Returns whether a line is a hypothesis.
    #[cfg(any(feature = "latex", feature = "svg"))]
    pub(crate) fn is_hypothesis(&self, line: &Line) -> bool {
        self.derivation.node(line.node).step() == Step::Hypothesis
    }

    /// Writes a line's citations with `range` between a subproof's first
    /// and last line: `2–4, 5`.
    pub(crate) fn write_cites(line: &Line, range: &str, out: &mut impl Write) -> fmt::Result {
        for (k, cite) in line.cites.iter().enumerate() {
            if k > 0 {
                out.write_str(", ")?;
            }
            match *cite {
                Cite::Line(n) => write!(out, "{n}")?,
                Cite::Range(a, b) => write!(out, "{a}{range}{b}")?,
            }
        }
        Ok(())
    }

    /// Writes a line's formula in a target's notation.
    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    pub(crate) fn formula(
        &self,
        line: &Line,
        notation: &crate::linear::sequents::notation::Notation,
        out: &mut String,
    ) {
        let spelling = super::print::Spelling::of(notation, false, false);
        self.calculus
            .write_sequent_in(self.derivation, line.node, spelling, out);
    }

    /// Writes a line's formula as text.
    pub(crate) fn plain_formula(&self, line: &Line, out: &mut impl Write) -> fmt::Result {
        write!(
            out,
            "{}",
            self.calculus.show_sequent(self.derivation, line.node)
        )
    }

    /// Writes the name of a line's rule as the rule file names it;
    /// nothing for a hypothesis and an open line.
    #[cfg(feature = "svg")]
    pub(crate) fn plain_rule(&self, line: &Line, out: &mut impl Write) -> fmt::Result {
        match self.derivation.node(line.node).step() {
            Step::Rule(name) => out.write_str(name),
            Step::Hypothesis | Step::Open => Ok(()),
        }
    }

    /// Writes a line's justification as text: its rule and citations, or
    /// the options' text for a hypothesis or an open line.
    pub(crate) fn plain_justification(&self, line: &Line, out: &mut impl Write) -> fmt::Result {
        match self.derivation.node(line.node).step() {
            Step::Rule(name) => {
                out.write_str(name)?;
                if !line.cites.is_empty() {
                    out.write_char(' ')?;
                    Self::write_cites(line, "–", out)?;
                }
                Ok(())
            }
            Step::Hypothesis | Step::Open => out.write_str(self.note(line)),
        }
    }
}

/// A walk up the scopes, its steps counted against the work bound.
struct Climb<'d> {
    /// The parent of each scope, the main proof's its own.
    parents: &'d [u32],
    /// The depth of each scope, 0 for the main proof.
    depth: &'d [u32],
    /// The steps taken.
    work: u64,
    /// The most steps, [`Limits::work`].
    limit: Option<u64>,
}

impl Climb<'_> {
    /// Counts a step, and refuses past the bound.
    fn tick(&mut self) -> Result<(), Error> {
        self.work = self.work.saturating_add(1);
        match self.limit {
            Some(limit) if self.work > limit => Err(Error::Refused(Refusal::Work { limit })),
            _ => Ok(()),
        }
    }

    /// Returns the scope directly inside `level` that holds `inner`, or
    /// `None` where `level` does not lie around `inner`.
    fn inside(&mut self, mut inner: u32, level: u32) -> Result<Option<u32>, Error> {
        let below = self.depth[level as usize] + 1;
        while self.depth[inner as usize] > below {
            self.tick()?;
            inner = self.parents[inner as usize];
        }
        Ok(
            (self.depth[inner as usize] == below && self.parents[inner as usize] == level)
                .then_some(inner),
        )
    }

    /// Returns the part of the proof at `level` that holds the node in
    /// the scope `at`: the node's line, or the subproof around it.
    fn unit(&mut self, at: u32, node: u32, level: u32) -> Result<Option<Unit>, Error> {
        if at == level {
            return Ok(Some(Unit::Node(node)));
        }
        Ok(self.inside(at, level)?.map(Unit::Block))
    }
}

/// Returns the first node of a scope, by which a reason names a subproof.
fn first_node(derivation: &Derivation, scope: u32) -> u32 {
    derivation
        .nodes()
        .find(|node| node.scope().index() as u32 == scope)
        .map_or(0, |node| node.id().index() as u32)
}

impl Display for Fitch<'_> {
    /// Writes the proof as text, a line each: its number, a `│` for the
    /// main proof and each subproof it lies in, its formula and its
    /// justification, and a `├──` under a proof's or subproof's last
    /// hypothesis.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let width = self.lines.len().to_string().len();
        // Each line's text before its justification, to align them.
        let mut heads = Vec::with_capacity(self.lines.len());
        for (index, line) in self.lines.iter().enumerate() {
            let mut head = format!("{:>width$} ", index + 1);
            for _ in 0..=line.depth {
                head.push_str("│ ");
            }
            self.plain_formula(line, &mut head)?;
            heads.push(head);
        }
        let column = heads.iter().map(|h| h.chars().count()).max().unwrap_or(0);
        for (index, line) in self.lines.iter().enumerate() {
            let head = &heads[index];
            let mut justification = String::new();
            self.plain_justification(line, &mut justification)?;
            if justification.is_empty() {
                f.write_str(head)?;
            } else {
                let pad = column - head.chars().count();
                write!(f, "{head}{:pad$}   {justification}", "")?;
            }
            f.write_char('\n')?;
            // A bar under the last hypothesis of every block it closes.
            for block in &self.blocks {
                if block.hypotheses > 0 && block.first + block.hypotheses - 1 == index as u32 {
                    write!(f, "{:width$} ", "")?;
                    for _ in 0..block.depth {
                        f.write_str("│ ")?;
                    }
                    f.write_str("├───\n")?;
                }
            }
        }
        Ok(())
    }
}
