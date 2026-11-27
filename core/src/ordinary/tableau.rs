// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Signed analytic tableaux of classical logic (Smullyan's, with the
//! signs `T` and `F`).
//!
//! A tableau for `Γ ⊢ Δ` starts from every formula of `Γ` signed `T`
//! (true) and every formula of `Δ` signed `F` (false), and expands one
//! formula at a time: a conjunction made true adds both conjuncts to its
//! branch, a disjunction made true splits the branch in two. A branch
//! closes where a formula is both true and false, `⊥` true or `⊤` false.
//! The sequent is valid where every branch closes.
//!
//! A closed tableau is a derivation of LK upside down: a branch is the
//! sequent of a goal, `T` its left side and `F` its right, and an
//! expansion is the inference of LK on that formula. So
//! [`Tableau::of`] reads any derivation of LK as a tableau, and the
//! method [`Method::Tableau`](super::Method::Tableau) builds the tableau
//! step by step in the goal state ([`Interactive`]), which checks every
//! step by the rules of LK; a closed tableau's derivation is then checked
//! whole, and an open branch's assignment is checked as a countermodel
//! ([`Countermodel::found`](super::Countermodel)).

use super::{Derivation, Formulas, Interactive, Logic, Node, NodeId, Rule, Sequent, Side, Step};
use crate::Error;
use crate::kripke::{Model, World};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::proofs::{GoalId, Structural, ViewOptions};
use std::fmt::{self, Display, Formatter, Write};

/// The sign of a formula on a tableau's branch: true or false.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sign {
    /// `T`: the formula is true on the branch, a hypothesis of the goal.
    True,
    /// `F`: the formula is false on the branch, a formula right of the
    /// goal's `⊢`.
    False,
}

impl Sign {
    /// Returns the sign of a formula on a side of a sequent: true on the
    /// left of `⊢`, false on the right.
    pub const fn of(side: Side) -> Self {
        match side {
            Side::Left => Self::True,
            Side::Right => Self::False,
        }
    }

    /// Returns the sign's letter, `T` or `F`.
    pub const fn letter(self) -> &'static str {
        match self {
            Self::True => "T",
            Self::False => "F",
        }
    }
}

/// A signed formula: a line of a tableau.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Signed {
    /// Whether the formula is true or false on the branch.
    pub sign: Sign,
    /// The formula, in the tableau's arena.
    pub formula: NodeId,
}

impl Signed {
    /// Returns the formula with its sign.
    pub const fn new(sign: Sign, formula: NodeId) -> Self {
        Self { sign, formula }
    }
}

/// How a branch of a tableau ends.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum End {
    /// Closed: one formula true at one line and false at another.
    Closed {
        /// The line where the formula is true.
        true_line: u32,
        /// The line where it is false.
        false_line: u32,
    },
    /// Closed: `⊥` true at a line.
    Falsity {
        /// The line.
        line: u32,
    },
    /// Closed: `⊤` false at a line.
    Truth {
        /// The line.
        line: u32,
    },
    /// Open and finished: every formula left to expand on the branch is
    /// an atom, `⊤` true or `⊥` false, and none closes it. The atoms true
    /// on it make every true formula of the branch true and every false
    /// one false.
    Open,
    /// Open and not finished: a formula on the branch is still to be
    /// expanded (a proof in progress).
    Unexpanded,
}

/// A node of a tableau: the lines one expansion adds to a branch, and
/// what comes after them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableauNode {
    /// The number of the node's first line.
    first: u32,
    /// The lines, numbered from `first` on.
    lines: Vec<Signed>,
    /// The line the node's children expand, and the rule of LK they
    /// expand it by.
    expands: Option<(u32, Rule)>,
    /// The children: one for a rule with one premise, two for a split.
    children: Vec<u32>,
    /// How the branch ends, for a node without children.
    end: Option<End>,
}

impl TableauNode {
    /// Returns the number of the node's first line. Lines are numbered
    /// from 0 in the order the tableau is written, each node before its
    /// children.
    pub fn first_line(&self) -> u32 {
        self.first
    }

    /// Returns the lines the node adds to its branch: the sequent's for
    /// the root, the new formulas of a premise for the others.
    pub fn lines(&self) -> &[Signed] {
        &self.lines
    }

    /// Returns the line the node's children expand and the rule of LK they
    /// expand it by; `None` for a node without children.
    pub fn expands(&self) -> Option<(u32, Rule)> {
        self.expands
    }

    /// Returns the node's children, by their index in
    /// [`Tableau::nodes`]: one after a rule with one premise, two after a
    /// split.
    pub fn children(&self) -> &[u32] {
        &self.children
    }

    /// Returns how the branch ends, for a node without children.
    pub fn end(&self) -> Option<End> {
        self.end
    }
}

/// A signed analytic tableau of classical logic.
///
/// Its nodes are in the order they are written, each before its
/// children; node 0 is the root, which holds the sequent. Its lines are
/// numbered from 0 in the same order. Every formula is a node of the
/// tableau's arena ([`formulas`](Self::formulas)), which holds the
/// sequent's formulas and those of the derivation it was read from.
///
/// A branch holds every line from the root down to a leaf. An expansion
/// adds what its premise has beyond its conclusion, counted as a
/// multiset: the results of expanding the formula, written even where the
/// branch has them already, as a textbook writes them. A contraction and
/// a weakening add nothing.
///
/// With the feature `serialize` a tableau is written as `{"version": 14,
/// "sequent": {…}, "nodes": [{"lines": [["T", 2], ["F", 4]], "expands":
/// [0, "∧L"], "children": [1]}, {"lines": […], "end": {"kind":
/// "closed", "true_line": 3, "false_line": 5}}, …]}`, the sequent in
/// the ordinary sequent's form with every formula a line names.
#[derive(Clone, Debug)]
pub struct Tableau {
    /// The formulas the lines name.
    formulas: Formulas,
    /// The sequent's formulas left of `⊢`.
    left: Vec<NodeId>,
    /// The sequent's formulas right of `⊢`.
    right: Vec<NodeId>,
    /// The nodes, each before its children.
    nodes: Vec<TableauNode>,
}

/// Where a formula of a sequent stands on its branch: a line written
/// above, or the `k`th line of the node being made.
#[derive(Clone, Copy, Debug)]
enum Place {
    /// A line above, by its number.
    Line(u32),
    /// The node's own line `k`.
    New(usize),
}

/// A node of the walk over a derivation, waiting to be written: the
/// inference that ends its branch's segment, its parent, its lines, and
/// the place of every formula of that inference's conclusion.
struct Pending {
    /// The inference whose conclusion the node's branch has reached.
    inference: usize,
    /// The parent node, `None` for the root.
    parent: Option<u32>,
    /// The lines the node adds.
    lines: Vec<Signed>,
    /// The place of each formula of the conclusion, its hypotheses
    /// first.
    places: Vec<Place>,
}

/// Returns the line of each formula of a premise, its hypotheses first,
/// and the lines the premise adds: a formula the conclusion has keeps
/// its line, the principal formula (`principal`, a position in the
/// conclusion's list) excepted; a copy a contraction makes takes its
/// original's line; every other formula is a new line.
fn placed(
    conclusion: &[Signed],
    lines: &[u32],
    principal: Option<usize>,
    premise: &[Signed],
    structural: bool,
) -> (Vec<Place>, Vec<Signed>) {
    let mut free: Vec<bool> = (0..conclusion.len())
        .map(|k| Some(k) != principal)
        .collect();
    let mut added = Vec::new();
    let places = premise
        .iter()
        .map(|line| {
            let same = |k: &usize| conclusion[*k] == *line;
            if let Some(k) = (0..conclusion.len()).find(|k| free[*k] && same(k)) {
                free[k] = false;
                Place::Line(lines[k])
            } else if let (true, Some(k)) = (structural, (0..conclusion.len()).find(same)) {
                Place::Line(lines[k])
            } else {
                added.push(*line);
                Place::New(added.len() - 1)
            }
        })
        .collect();
    (places, added)
}

impl Tableau {
    /// Returns the tableau of a derivation of LK: the derivation upside
    /// down, its structural rules left out. Each inference on a formula
    /// is an expansion of that formula's line, each axiom and `⊥L` and
    /// `⊤R` a closed branch, each open goal an open branch. The stop is
    /// asked once per inference.
    ///
    /// # Errors
    ///
    /// [`Error::Logic`] for a derivation of LJ: a signed tableau is
    /// classical. [`Error::ReadBack`] where an inference does not have
    /// the shape of its rule (a structural rule without a premise, a cut,
    /// an axiom without a formula on both sides), which a derivation that
    /// [`Derivation::check`] accepts never has. [`Refusal::Stopped`] when
    /// `stop` fired.
    pub fn of(
        derivation: &Derivation,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        derivation.propositional(super::Method::Tableau.title())?;
        if derivation.logic() != Logic::Classical {
            return Err(Error::Logic {
                logic: derivation.logic(),
                method: super::Method::Tableau,
            });
        }
        let fail = |reason: String| Error::ReadBack {
            calculus: "LK",
            reason,
        };
        let inferences = derivation.inferences();
        let (left, right) = derivation.sequent();
        let root: Vec<Signed> = signed(left, right).collect();
        let mut nodes: Vec<TableauNode> = Vec::new();
        let mut next_line = 0u32;
        let mut visited = 0u64;
        let mut pending = vec![Pending {
            inference: inferences.len() - 1,
            parent: None,
            places: (0..root.len()).map(Place::New).collect(),
            lines: root,
        }];
        while let Some(node) = pending.pop() {
            let index = u32::try_from(nodes.len()).map_err(|_| too_many(nodes.len()))?;
            let first = next_line;
            let count = u32::try_from(node.lines.len()).map_err(|_| too_many(node.lines.len()))?;
            next_line = next_line
                .checked_add(count)
                .ok_or_else(|| too_many(next_line as usize))?;
            if let Some(parent) = node.parent {
                nodes[parent as usize].children.push(index);
            }
            // The line of each formula of the inference's conclusion.
            let mut lines: Vec<u32> = node
                .places
                .iter()
                .map(|place| match *place {
                    Place::Line(line) => line,
                    // Below `count`, which a `u32` holds.
                    Place::New(k) => first + k as u32,
                })
                .collect();
            // Past the structural rules, to the inference that acts on a
            // formula or ends the branch, the lines carried along.
            let mut at = node.inference;
            loop {
                visited += 1;
                if stop(Progress::new(Phase::View, 1, visited)) {
                    return Err(Error::Refused(Refusal::stopped(Phase::View)));
                }
                let inference = &inferences[at];
                match inference.rule() {
                    Rule::WeakenLeft
                    | Rule::WeakenRight
                    | Rule::ContractLeft
                    | Rule::ContractRight => {
                        let premise = inference
                            .premises()
                            .first()
                            .ok_or_else(|| fail(format!("inference {at} has no premise")))?
                            .index();
                        let conclusion: Vec<Signed> =
                            signed(inference.left(), inference.right()).collect();
                        let above = &inferences[premise];
                        let (places, _) = placed(
                            &conclusion,
                            &lines,
                            None,
                            &signed(above.left(), above.right()).collect::<Vec<_>>(),
                            true,
                        );
                        lines = places
                            .iter()
                            .map(|place| match *place {
                                Place::Line(line) => Ok(line),
                                Place::New(_) => Err(fail(format!(
                                    "the structural inference {at} adds a formula"
                                ))),
                            })
                            .collect::<Result<_, _>>()?;
                        at = premise;
                    }
                    Rule::Axiom
                    | Rule::AndLeft
                    | Rule::AndLeft1
                    | Rule::AndLeft2
                    | Rule::AndRight
                    | Rule::OrLeft
                    | Rule::OrRight
                    | Rule::OrRight1
                    | Rule::OrRight2
                    | Rule::ImpliesLeft
                    | Rule::ImpliesRight
                    | Rule::NotLeft
                    | Rule::NotRight
                    | Rule::IffLeft
                    | Rule::IffLeft1
                    | Rule::IffLeft2
                    | Rule::IffRight
                    | Rule::FalseLeft
                    | Rule::FalseRight
                    | Rule::TrueLeft
                    | Rule::TrueRight
                    | Rule::Cut
                    | Rule::Open => break,
                    Rule::ForallLeft | Rule::ForallRight | Rule::ExistsLeft | Rule::ExistsRight => {
                        unreachable!("{}", super::REFUSED)
                    }
                }
            }
            let inference = &inferences[at];
            let conclusion: Vec<Signed> = signed(inference.left(), inference.right()).collect();
            let hypotheses = inference.left().len();
            let mut made = TableauNode {
                first,
                lines: node.lines,
                expands: None,
                children: Vec::new(),
                end: None,
            };
            match inference.rule() {
                Rule::Axiom => {
                    let pair = inference.left().iter().enumerate().find_map(|(i, x)| {
                        let j = inference.right().iter().position(|y| y == x)?;
                        Some((lines[i], lines[hypotheses + j]))
                    });
                    let (true_line, false_line) = pair
                        .ok_or_else(|| fail(format!("axiom {at} has no formula on both sides")))?;
                    made.end = Some(End::Closed {
                        true_line,
                        false_line,
                    });
                }
                Rule::FalseLeft | Rule::TrueRight => {
                    let falsity = inference.rule() == Rule::FalseLeft;
                    let constant = |x: &NodeId| match derivation.formulas().node(*x) {
                        Node::False => falsity,
                        Node::True => !falsity,
                        Node::Atom(_)
                        | Node::Not(_)
                        | Node::And(..)
                        | Node::Or(..)
                        | Node::Implies(..)
                        | Node::Iff(..) => false,
                        Node::Forall(_) | Node::Exists(_) => {
                            unreachable!("{}", crate::ordinary::REFUSED)
                        }
                    };
                    let line = if falsity {
                        inference.left().iter().position(constant).map(|i| lines[i])
                    } else {
                        inference
                            .right()
                            .iter()
                            .position(constant)
                            .map(|j| lines[hypotheses + j])
                    }
                    .ok_or_else(|| fail(format!("inference {at} closes on no line")))?;
                    made.end = Some(if falsity {
                        End::Falsity { line }
                    } else {
                        End::Truth { line }
                    });
                }
                Rule::Open => {
                    made.end = Some(if finished(derivation.formulas(), inference) {
                        End::Open
                    } else {
                        End::Unexpanded
                    });
                }
                Rule::Cut
                | Rule::WeakenLeft
                | Rule::WeakenRight
                | Rule::ContractLeft
                | Rule::ContractRight => {
                    return Err(fail(format!("inference {at} is no step of a tableau")));
                }
                Rule::ForallLeft | Rule::ForallRight | Rule::ExistsLeft | Rule::ExistsRight => {
                    unreachable!("{}", super::REFUSED)
                }
                rule @ (Rule::AndLeft
                | Rule::AndLeft1
                | Rule::AndLeft2
                | Rule::AndRight
                | Rule::OrLeft
                | Rule::OrRight
                | Rule::OrRight1
                | Rule::OrRight2
                | Rule::ImpliesLeft
                | Rule::ImpliesRight
                | Rule::NotLeft
                | Rule::NotRight
                | Rule::IffLeft
                | Rule::IffLeft1
                | Rule::IffLeft2
                | Rule::IffRight
                | Rule::FalseRight
                | Rule::TrueLeft) => {
                    let (side, position) = inference
                        .principal()
                        .ok_or_else(|| fail(format!("inference {at} has no principal formula")))?;
                    let principal = match side {
                        Side::Left => position,
                        Side::Right => hypotheses + position,
                    };
                    let line = *lines
                        .get(principal)
                        .ok_or_else(|| fail(format!("inference {at} names no formula")))?;
                    made.expands = Some((line, rule));
                    // The first premise is written first, so it waits last.
                    for &premise in inference.premises().iter().rev() {
                        let above = &inferences[premise.index()];
                        let (places, lines_added) = placed(
                            &conclusion,
                            &lines,
                            Some(principal),
                            &signed(above.left(), above.right()).collect::<Vec<_>>(),
                            false,
                        );
                        pending.push(Pending {
                            inference: premise.index(),
                            parent: Some(index),
                            lines: lines_added,
                            places,
                        });
                    }
                }
            }
            nodes.push(made);
        }
        Ok(Self {
            formulas: derivation.formulas().clone(),
            left: left.to_vec(),
            right: right.to_vec(),
            nodes,
        })
    }

    /// Returns the formulas the lines name.
    pub fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    /// Returns the sequent the tableau is for, its two sides.
    pub fn sequent(&self) -> (&[NodeId], &[NodeId]) {
        (&self.left, &self.right)
    }

    /// Returns the nodes, each before its children; node 0 is the root.
    pub fn nodes(&self) -> &[TableauNode] {
        &self.nodes
    }

    /// Returns how many lines the tableau has.
    pub fn line_count(&self) -> u32 {
        self.nodes
            .last()
            .map_or(0, |node| node.first + node.lines.len() as u32)
    }

    /// Returns the line numbered `line`.
    ///
    /// # Panics
    ///
    /// For a number past the tableau's lines.
    pub fn line(&self, line: u32) -> Signed {
        let at = self.nodes.partition_point(|node| node.first <= line) - 1;
        let node = &self.nodes[at];
        node.lines[(line - node.first) as usize]
    }

    /// Returns whether every branch is closed.
    pub fn is_closed(&self) -> bool {
        self.nodes.iter().all(|node| {
            !node.children.is_empty()
                || matches!(
                    node.end,
                    Some(End::Closed { .. } | End::Falsity { .. } | End::Truth { .. })
                )
        })
    }

    /// Writes the tableau as text, a line per signed formula: its number
    /// (from 1, as a reader counts), its sign and formula, and the rule
    /// and line it comes from; a split starts each branch with `├─` or
    /// `└─`, and a branch ends with `× i, j` (closed by lines `i` and
    /// `j`), `× i` (by `⊥` true or `⊤` false), `open` or `not expanded`.
    ///
    /// # Errors
    ///
    /// The writer's.
    pub fn write_text(&self, out: &mut impl Write) -> fmt::Result {
        // Each row's text before its justification, and the justification.
        let mut rows: Vec<(String, String)> = Vec::new();
        // A node, the prefix of its lines, and the marker of its first.
        let mut stack: Vec<(usize, String, &str)> = vec![(0, String::new(), "")];
        while let Some((index, prefix, marker)) = stack.pop() {
            let node = &self.nodes[index];
            let from = self.parent_rule(index);
            for (k, line) in node.lines.iter().enumerate() {
                let lead = if k == 0 { marker } else { blank(marker) };
                let mut text = format!(
                    "{prefix}{lead}{}  {} ",
                    node.first as usize + k + 1,
                    line.sign.letter()
                );
                write!(text, "{}", self.formulas.formula(line.formula))?;
                let just =
                    from.map_or(String::new(), |(line, rule)| format!("{rule} {}", line + 1));
                rows.push((text, just));
            }
            let inner = format!("{prefix}{}", blank(marker));
            if let Some(end) = node.end {
                rows.push((format!("{inner}{}", ending(end)), String::new()));
            }
            match node.children.as_slice() {
                [only] => stack.push((*only as usize, inner, "")),
                children => {
                    for (k, &child) in children.iter().enumerate().rev() {
                        let last = k + 1 == children.len();
                        stack.push((
                            child as usize,
                            inner.clone(),
                            if last { "└─ " } else { "├─ " },
                        ));
                    }
                }
            }
        }
        let width = rows
            .iter()
            .map(|(text, _)| text.chars().count())
            .max()
            .unwrap_or(0);
        for (n, (text, just)) in rows.iter().enumerate() {
            if n > 0 {
                writeln!(out)?;
            }
            if just.is_empty() {
                out.write_str(text)?;
            } else {
                let pad = width - text.chars().count();
                write!(out, "{text}{:pad$}  {just}", "")?;
            }
        }
        Ok(())
    }

    /// Returns the line and rule of the expansion that made node `index`,
    /// `None` for the root.
    fn parent_rule(&self, index: usize) -> Option<(u32, Rule)> {
        let child = u32::try_from(index).ok()?;
        self.nodes[..index]
            .iter()
            .rev()
            .find(|node| node.children.contains(&child))
            .and_then(|node| node.expands)
    }

    /// Returns a tableau made of its parts, for the reader, which checks
    /// them.
    #[cfg(feature = "serialize")]
    pub(crate) fn from_parts(
        formulas: Formulas,
        (left, right): (Vec<NodeId>, Vec<NodeId>),
        nodes: Vec<TableauNode>,
    ) -> Self {
        Self {
            formulas,
            left,
            right,
            nodes,
        }
    }
}

impl TableauNode {
    /// Returns a node made of its parts, for the reader.
    #[cfg(feature = "serialize")]
    pub(crate) fn from_parts(
        first: u32,
        lines: Vec<Signed>,
        (expands, children, end): (Option<(u32, Rule)>, Vec<u32>, Option<End>),
    ) -> Self {
        Self {
            first,
            lines,
            expands,
            children,
            end,
        }
    }
}

impl Display for Tableau {
    /// Writes the tableau as [`write_text`](Self::write_text) does.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_text(f)
    }
}

/// Returns what stands under a branch's marker on the lines after its
/// first: the rule down to the next branch after `├─`, spaces after `└─`.
fn blank(marker: &str) -> &'static str {
    match marker {
        "" => "",
        "├─ " => "│  ",
        _ => "   ",
    }
}

/// Returns how a branch's end is written.
fn ending(end: End) -> String {
    match end {
        End::Closed {
            true_line,
            false_line,
        } => format!("× {}, {}", true_line + 1, false_line + 1),
        End::Falsity { line } | End::Truth { line } => format!("× {}", line + 1),
        End::Open => "open".to_owned(),
        End::Unexpanded => "not expanded".to_owned(),
    }
}

/// The bytes the goal state holds per goal a step opens, besides its
/// formulas' ids: the inference, its two lists, its premises and its
/// entry in the history.
const GOAL_BYTES: u64 = 160;

/// Returns the bytes an axiom adds to the goal state when it closes a goal
/// of `formulas` formulas: one weakening per formula but the two it keeps,
/// each holding what is left of the goal, and the axiom itself, `formulas
/// − 1` inferences of `formulas · (formulas − 1) / 2` ids in all. Charged
/// when the search meets the axiom, so that the search refuses before the
/// state holds them: uncharged, 2 048 closed branches of 1 200 formulas
/// each took 6 GB under a bound of 256 MiB.
fn axiom_bytes(formulas: usize) -> u64 {
    let n = formulas as u64;
    let below = n.saturating_sub(1);
    below
        .saturating_mul(GOAL_BYTES)
        .saturating_add(4u64.saturating_mul(n).saturating_mul(below))
}

/// Returns the refusal of a tableau of more nodes or lines than a `u32`
/// numbers.
fn too_many(count: usize) -> Error {
    Error::Refused(Refusal::Index {
        what: crate::limits::Space::Inference,
        count: count as u64,
        most: u64::from(u32::MAX),
    })
}

/// Returns a sequent's formulas signed, the left side's true and the
/// right side's false.
fn signed<'a>(left: &'a [NodeId], right: &'a [NodeId]) -> impl Iterator<Item = Signed> + 'a {
    left.iter()
        .map(|&x| Signed::new(Sign::True, x))
        .chain(right.iter().map(|&x| Signed::new(Sign::False, x)))
}

/// Returns whether an open goal is finished: nothing on it is left to
/// expand and nothing closes it.
fn finished(formulas: &Formulas, goal: &super::Inference) -> bool {
    next_step(formulas, goal.left(), goal.right()).is_none()
}

/// Returns the step a systematic tableau takes next on a goal: a closing
/// rule where one applies, else the first formula of one premise (the
/// left side first), else the first of two premises; `None` where every
/// formula is an atom, `⊤` true or `⊥` false and none closes the goal.
fn next_step(formulas: &Formulas, left: &[NodeId], right: &[NodeId]) -> Option<Step> {
    if let Some(i) = left.iter().position(|x| right.contains(x)) {
        return Some(Step::new(Side::Left, i, Rule::Axiom));
    }
    if let Some(i) = left.iter().position(|&x| formulas.node(x) == Node::False) {
        return Some(Step::new(Side::Left, i, Rule::FalseLeft));
    }
    if let Some(i) = right.iter().position(|&x| formulas.node(x) == Node::True) {
        return Some(Step::new(Side::Right, i, Rule::TrueRight));
    }
    // One premise (Smullyan's α) before two (β).
    let one = |side: Side, x: NodeId| match (side, formulas.node(x)) {
        (Side::Left, Node::And(..)) => Some(Rule::AndLeft),
        (Side::Left, Node::Not(_)) => Some(Rule::NotLeft),
        (Side::Left, Node::Iff(..)) => Some(Rule::IffLeft),
        (Side::Right, Node::Or(..)) => Some(Rule::OrRight),
        (Side::Right, Node::Implies(..)) => Some(Rule::ImpliesRight),
        (Side::Right, Node::Not(_)) => Some(Rule::NotRight),
        (_, Node::Atom(_) | Node::True | Node::False)
        | (Side::Left, Node::Or(..) | Node::Implies(..))
        | (Side::Right, Node::And(..) | Node::Iff(..)) => None,
        (_, Node::Forall(_) | Node::Exists(_)) => unreachable!("{}", crate::ordinary::REFUSED),
    };
    let two = |side: Side, x: NodeId| match (side, formulas.node(x)) {
        (Side::Left, Node::Or(..)) => Some(Rule::OrLeft),
        (Side::Left, Node::Implies(..)) => Some(Rule::ImpliesLeft),
        (Side::Right, Node::And(..)) => Some(Rule::AndRight),
        (Side::Right, Node::Iff(..)) => Some(Rule::IffRight),
        (_, Node::Atom(_) | Node::True | Node::False | Node::Not(_))
        | (Side::Left, Node::And(..) | Node::Iff(..))
        | (Side::Right, Node::Or(..) | Node::Implies(..)) => None,
        (_, Node::Forall(_) | Node::Exists(_)) => unreachable!("{}", crate::ordinary::REFUSED),
    };
    let rules: [&dyn Fn(Side, NodeId) -> Option<Rule>; 2] = [&one, &two];
    for rule_of in rules {
        for (side, list) in [(Side::Left, left), (Side::Right, right)] {
            for (i, &x) in list.iter().enumerate() {
                if let Some(rule) = rule_of(side, x) {
                    return Some(Step::new(side, i, rule));
                }
            }
        }
    }
    None
}

/// The goals and steps of a tableau's search: the goals left open, the
/// closing step of every branch that closes, and the steps taken.
type Expanded = (Vec<GoalId>, Vec<(GoalId, Step)>, u64);

/// Expands every goal of the state until each is finished or closes, and
/// returns the open goals and the closing step of each closed one, not
/// taken: the caller closes them. The stop is asked once per step, each
/// step a unit of `limits.work`, and every goal a step opens, and every
/// axiom's weakenings to come, are charged to `limits.memory_bytes`.
fn expand(
    state: &mut Interactive,
    limits: &Limits,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<Expanded, Error> {
    let mut open = Vec::new();
    let mut closing = Vec::new();
    let mut goals: Vec<_> = state.goals().collect();
    let mut steps = 0u64;
    // What the goal state holds, counted per goal it opens, and what the
    // closing steps will add.
    let mut held = 0u64;
    while let Some(goal) = goals.pop() {
        steps += 1;
        if stop(Progress::new(Phase::Search, 1, steps)) {
            return Err(Error::Refused(Refusal::stopped(Phase::Search)));
        }
        if let Some(limit) = limits.work
            && steps > limit
        {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        let (left, right) = state.goal(goal)?;
        match next_step(state.formulas(), left, right) {
            Some(step) if matches!(step.rule, Rule::Axiom | Rule::FalseLeft | Rule::TrueRight) => {
                if step.rule == Rule::Axiom {
                    held = held.saturating_add(axiom_bytes(left.len() + right.len()));
                }
                closing.push((goal, step));
            }
            Some(step) => {
                let opened = state.apply(goal, &step)?;
                for &new in &opened {
                    let (left, right) = state.goal(new)?;
                    held = held.saturating_add(GOAL_BYTES + 8 * (left.len() + right.len()) as u64);
                }
                // The first premise is expanded first, so it waits last.
                goals.extend(opened.into_iter().rev());
            }
            None => open.push(goal),
        }
        if let Some(limit) = limits.memory_bytes
            && held > limit
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: limit,
                needed_bytes: None,
            }));
        }
    }
    Ok((open, closing, steps))
}

/// Decides a sequent of classical logic by a systematic tableau, built in
/// the goal state so that every step is checked by the rules of LK: valid
/// with the checked derivation of LK the closed tableau is, or not valid
/// with the first open branch's assignment as a checked countermodel. The
/// stop is asked once per step and once per branch closed, and each step
/// is a unit of `limits.work`.
pub(super) fn decide(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<super::Outcome, Error> {
    if logic != Logic::Classical {
        return Err(Error::Logic {
            logic,
            method: super::Method::Tableau,
        });
    }
    let mut state = Interactive::within(sequent, logic, limits)?;
    let (open, closing, steps) = expand(&mut state, limits, &mut stop)?;
    // The branches close only now: the axiom weakens every other formula
    // of its goal away, one inference holding the rest of the goal each, so
    // closing them as the search met them grew the state with the square
    // of every goal, and freeing it once the stop fired took half a second
    // (4.4 million inferences at 2 s).
    for (goal, step) in closing {
        if stop(Progress::new(Phase::Search, 1, steps)) {
            return Err(Error::Refused(Refusal::stopped(Phase::Search)));
        }
        state.apply(goal, &step)?;
    }
    let Some(&first) = open.first() else {
        let derivation = state.finished(limits, &mut stop)?;
        let tableau = Tableau::of(&derivation, &mut stop)?;
        return Ok(super::Outcome {
            verdict: super::Verdict::Valid(super::Evidence::Derivation(Box::new(derivation))),
            logic,
            search: super::Search::Tableau(Box::new(tableau)),
            countermodel: None,
            structure: None,
            kripke: None,
        });
    };
    let view = ViewOptions::default().with_structural(Structural::Explicit);
    let derivation = state.derivation_within(&view, limits, &mut stop)?;
    let tableau = Tableau::of(&derivation, &mut stop)?;
    // The atoms true on the open branch, which falsify its sequent and so
    // the sequent at its root.
    let formulas = state.formulas();
    let (left, _) = state.goal(first)?;
    let mut atoms: Vec<String> = left
        .iter()
        .filter_map(|&x| match formulas.node(x) {
            Node::Atom(a) => Some(formulas.atom_names()[a as usize].clone()),
            Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..) => None,
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        })
        .collect();
    atoms.sort_unstable();
    atoms.dedup();
    let model = Model::new(vec![World::new(atoms, false, Vec::new())], 0);
    // The stop that ends the model's check leaves the sequent unknown, the
    // tableau kept.
    let (verdict, countermodel) =
        match super::Countermodel::found(sequent.clone(), model, logic, limits, &mut stop) {
            Ok(countermodel) => (super::Verdict::NotValid, countermodel),
            Err(error) => (super::Verdict::Unknown(super::decide::ended(error)?), None),
        };
    Ok(super::Outcome {
        verdict,
        logic,
        search: super::Search::Tableau(Box::new(tableau)),
        countermodel,
        structure: None,
        kripke: None,
    })
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::{Evidence, Logic, Options, Sequent, Verdict, decide};
    use super::{End, Tableau};
    use crate::Limits;
    #[cfg(feature = "serialize")]
    use {super::super::Interactive, super::expand};

    /// A search stopped part way holds only the expansions it made: no
    /// branch is closed before the search ends, so the axiom's weakenings,
    /// one inference per formula of its goal, are never left to free.
    #[test]
    #[cfg(feature = "serialize")]
    fn a_stopped_search_holds_no_closed_branch() {
        // Forty atoms on each side, so every goal is large, and eight
        // branches after three splits, each closed by an axiom on `e` or `f`.
        let atoms = |x: char| {
            (0..40)
                .map(|i| format!("{x}{i}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let text = format!(
            "{}, (a \\/ b) /\\ (c \\/ d) /\\ (e \\/ f) |- {}, e, f",
            atoms('p'),
            atoms('q')
        );
        let sequent: Sequent = text.parse().unwrap();
        let limits = Limits::default();
        let mut state = Interactive::within(&sequent, Logic::Classical, &limits).unwrap();
        let mut asked = 0u64;
        let stopped = expand(&mut state, &limits, &mut |_| {
            asked += 1;
            asked > 6
        });
        assert!(stopped.is_err());
        // Six steps, each one inference and at most two premises.
        assert!(
            state.parts().2.len() <= 1 + 6 * 2,
            "{}",
            state.parts().2.len()
        );
        // Unstopped, every branch waits to close, and closing them all
        // gives the same verdict as the method.
        let mut state = Interactive::within(&sequent, Logic::Classical, &limits).unwrap();
        let (open, closing, _) = expand(&mut state, &limits, &mut |_| false).unwrap();
        assert_eq!(open, []);
        assert_eq!(closing.len(), 8);
    }

    /// The weakenings of the axioms that close the branches are charged
    /// while the search runs: 128 branches of 128 formulas are charged
    /// some 11 MB once closed (they hold 6 to 7 MB) where the search itself
    /// holds 0.3 MB, so a bound of 1 MiB refuses the search (before, it
    /// answered valid and held the weakenings uncounted) and one of 64 MiB
    /// lets it finish.
    #[test]
    fn the_axioms_weakenings_count_against_the_memory_bound() {
        use super::super::{Method, decide_within};
        use crate::limits::Refusal;
        let atoms = |x: char| {
            (0..60)
                .map(|i| format!("{x}{i}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let splits = (0..6)
            .map(|j| format!("(x{j} \\/ y{j})"))
            .collect::<Vec<_>>()
            .join(", ");
        let text = format!("{}, {splits}, (z \\/ z) |- {}, z", atoms('p'), atoms('q'));
        let sequent: Sequent = text.parse().unwrap();
        let options = Options::default()
            .with_logic(Logic::Classical)
            .with_method(Some(Method::Tableau));
        let decided = |bytes: u64| {
            let limits = Limits::default().with_memory_bytes(Some(bytes));
            decide_within(&sequent, &options, &Default::default(), &limits, |_| false)
        };
        assert!(matches!(
            decided(1 << 20),
            Err(crate::Error::Refused(Refusal::Memory { .. }))
        ));
        assert!(matches!(
            decided(64 << 20).unwrap().verdict,
            Verdict::Valid(Evidence::Derivation(_))
        ));
    }

    /// A valid sequent's tableau closes every branch and is the checked
    /// derivation of LK upside down; one that is not valid has an open
    /// branch, whose assignment is the checked countermodel; the text
    /// numbers the lines and says why each branch ends.
    #[test]
    fn a_tableau_closes_or_gives_a_countermodel() {
        let options = Options::default()
            .with_logic(Logic::Classical)
            .with_method(Some(super::super::Method::Tableau));
        let sequent: Sequent = "a /\\ b |- b /\\ a".parse().unwrap();
        let outcome = decide(&sequent, &options).unwrap();
        let Verdict::Valid(Evidence::Derivation(derivation)) = &outcome.verdict else {
            panic!("valid: {:?}", outcome.verdict);
        };
        let super::super::Search::Tableau(tableau) = &outcome.search else {
            panic!("a tableau");
        };
        assert!(tableau.is_closed());
        assert_eq!(
            Tableau::of(derivation, |_| false).unwrap().nodes(),
            tableau.nodes()
        );
        assert_eq!(
            tableau.to_string(),
            "1  T a ∧ b\n\
             2  F b ∧ a\n\
             3  T a      ∧L 1\n\
             4  T b      ∧L 1\n\
             ├─ 5  F b   ∧R 2\n\
             │  × 4, 5\n\
             └─ 6  F a   ∧R 2\n\
             \u{20}  × 3, 6"
        );

        let sequent: Sequent = "a \\/ b |- a".parse().unwrap();
        let outcome = decide(&sequent, &options).unwrap();
        assert!(matches!(outcome.verdict, Verdict::NotValid));
        let super::super::Search::Tableau(tableau) = &outcome.search else {
            panic!("a tableau");
        };
        assert!(!tableau.is_closed());
        assert!(tableau.nodes().iter().any(|n| n.end() == Some(End::Open)));
        let countermodel = outcome.countermodel.unwrap();
        assert_eq!(countermodel.model.worlds[0].atoms, ["b"]);

        let intuitionistic = options.with_logic(Logic::Intuitionistic);
        assert!(decide(&sequent, &intuitionistic).is_err());

        // A formula twice: each expansion cites its own line.
        let twice: Sequent = "a /\\ b, a /\\ b |- c".parse().unwrap();
        let outcome = decide(&twice, &options).unwrap();
        let super::super::Search::Tableau(tableau) = &outcome.search else {
            panic!("a tableau");
        };
        let cited: Vec<u32> = tableau
            .nodes()
            .iter()
            .filter_map(|n| n.expands().map(|(line, _)| line))
            .collect();
        assert_eq!(cited, [0, 1]);
        let atoms = &outcome.countermodel.unwrap().model.worlds[0].atoms;
        assert_eq!(atoms, &["a", "b"]);

        // The goal state is held within the memory bound.
        let wide: Sequent = "a \\/ b, c \\/ d, e \\/ f |- g".parse().unwrap();
        let small = crate::Limits::default().with_memory_bytes(Some(256));
        let refused = super::super::decide_within(
            &wide,
            &options,
            &crate::linear::search::Options::default(),
            &small,
            |_| false,
        );
        assert!(
            matches!(
                refused,
                Err(crate::Error::Refused(crate::Refusal::Memory { .. }))
            ),
            "{refused:?}"
        );
    }
}
