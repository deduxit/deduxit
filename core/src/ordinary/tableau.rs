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
use crate::linear::proofs::{Structural, ViewOptions};
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
    /// Open and finished: every formula on the branch is an atom, `⊤`
    /// true or `⊥` false, and none closes it. The atoms true on it make
    /// every true formula of the branch true and every false one false.
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

/// A node of the walk over a derivation, waiting to be written: the
/// inference that ends its branch's segment, its parent, its lines, and
/// how long the branch was above it.
struct Pending {
    /// The inference whose conclusion the node's branch has reached.
    inference: usize,
    /// The parent node, `None` for the root.
    parent: Option<u32>,
    /// The lines the node adds.
    lines: Vec<Signed>,
    /// The branch's lines above the node.
    above: usize,
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
    /// classical. [`Error::ReadBack`] where an inference does not follow
    /// from the formulas on its branch, which a derivation that
    /// [`Derivation::check`] accepts never has. [`Refusal::Stopped`] when
    /// `stop` fired.
    pub fn of(
        derivation: &Derivation,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
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
        // The branch's lines from the root down, with their numbers.
        let mut branch: Vec<(Signed, u32)> = Vec::new();
        let mut next_line = 0u32;
        let mut visited = 0u64;
        let mut pending = vec![Pending {
            inference: inferences.len() - 1,
            parent: None,
            lines: root,
            above: 0,
        }];
        while let Some(node) = pending.pop() {
            branch.truncate(node.above);
            let index = u32::try_from(nodes.len()).map_err(|_| too_many(nodes.len()))?;
            let first = next_line;
            for &line in &node.lines {
                branch.push((line, next_line));
                next_line = next_line
                    .checked_add(1)
                    .ok_or_else(|| too_many(next_line as usize))?;
            }
            if let Some(parent) = node.parent {
                nodes[parent as usize].children.push(index);
            }
            // Past the structural rules, to the inference that acts on a
            // formula or ends the branch.
            let mut at = node.inference;
            loop {
                visited += 1;
                if stop(Progress::new(Phase::View, 1, visited)) {
                    return Err(Error::Refused(Refusal::Stopped { phase: Phase::View }));
                }
                let inference = &inferences[at];
                match inference.rule() {
                    Rule::WeakenLeft
                    | Rule::WeakenRight
                    | Rule::ContractLeft
                    | Rule::ContractRight => {
                        at = inference.premises()[0].index();
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
                }
            }
            let inference = &inferences[at];
            let line_of = |signed: Signed| {
                branch
                    .iter()
                    .rev()
                    .find(|(line, _)| *line == signed)
                    .map(|&(_, number)| number)
            };
            let mut made = TableauNode {
                first,
                lines: node.lines,
                expands: None,
                children: Vec::new(),
                end: None,
            };
            match inference.rule() {
                Rule::Axiom => {
                    let both = inference
                        .left()
                        .iter()
                        .find(|x| inference.right().contains(x))
                        .ok_or_else(|| fail(format!("axiom {at} has no formula on both sides")))?;
                    let true_line = line_of(Signed::new(Sign::True, *both));
                    let false_line = line_of(Signed::new(Sign::False, *both));
                    let (Some(true_line), Some(false_line)) = (true_line, false_line) else {
                        return Err(fail(format!(
                            "axiom {at} closes on a formula off its branch"
                        )));
                    };
                    made.end = Some(End::Closed {
                        true_line,
                        false_line,
                    });
                }
                Rule::FalseLeft | Rule::TrueRight => {
                    let (sign, list) = if inference.rule() == Rule::FalseLeft {
                        (Sign::True, inference.left())
                    } else {
                        (Sign::False, inference.right())
                    };
                    let constant = |x: &&NodeId| match derivation.formulas().node(**x) {
                        Node::False => sign == Sign::True,
                        Node::True => sign == Sign::False,
                        Node::Atom(_)
                        | Node::Not(_)
                        | Node::And(..)
                        | Node::Or(..)
                        | Node::Implies(..)
                        | Node::Iff(..) => false,
                    };
                    let line = list
                        .iter()
                        .find(constant)
                        .and_then(|&x| line_of(Signed::new(sign, x)))
                        .ok_or_else(|| fail(format!("inference {at} closes on no line")))?;
                    made.end = Some(if sign == Sign::True {
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
                    let principal = *(match side {
                        Side::Left => inference.left(),
                        Side::Right => inference.right(),
                    })
                    .get(position)
                    .ok_or_else(|| fail(format!("inference {at} names no formula")))?;
                    let line =
                        line_of(Signed::new(Sign::of(side), principal)).ok_or_else(|| {
                            fail(format!("inference {at} expands a formula off its branch"))
                        })?;
                    made.expands = Some((line, rule));
                    // The first premise is written first, so it waits last.
                    for &premise in inference.premises().iter().rev() {
                        // What the premise has beyond the conclusion,
                        // counted as a multiset: the expansion's results,
                        // written even where the branch has them already.
                        let mut concluded: Vec<Signed> =
                            signed(inference.left(), inference.right()).collect();
                        let lines = signed(
                            inferences[premise.index()].left(),
                            inferences[premise.index()].right(),
                        )
                        .filter(|line| match concluded.iter().position(|c| c == line) {
                            Some(k) => {
                                concluded.swap_remove(k);
                                false
                            }
                            None => true,
                        })
                        .collect();
                        pending.push(Pending {
                            inference: premise.index(),
                            parent: Some(index),
                            lines,
                            above: branch.len(),
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
    };
    let two = |side: Side, x: NodeId| match (side, formulas.node(x)) {
        (Side::Left, Node::Or(..)) => Some(Rule::OrLeft),
        (Side::Left, Node::Implies(..)) => Some(Rule::ImpliesLeft),
        (Side::Right, Node::And(..)) => Some(Rule::AndRight),
        (Side::Right, Node::Iff(..)) => Some(Rule::IffRight),
        (_, Node::Atom(_) | Node::True | Node::False | Node::Not(_))
        | (Side::Left, Node::And(..) | Node::Iff(..))
        | (Side::Right, Node::Or(..) | Node::Implies(..)) => None,
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

/// Decides a sequent of classical logic by a systematic tableau, built in
/// the goal state so that every step is checked by the rules of LK: valid
/// with the checked derivation of LK the closed tableau is, or not valid
/// with the first open branch's assignment as a checked countermodel. The
/// stop is asked once per step, and each step is a unit of
/// `limits.work`.
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
    let mut open = Vec::new();
    let mut goals: Vec<_> = state.goals().collect();
    let mut steps = 0u64;
    while let Some(goal) = goals.pop() {
        steps += 1;
        if stop(Progress::new(Phase::Search, 1, steps)) {
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::Search,
            }));
        }
        if let Some(limit) = limits.work
            && steps > limit
        {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        let (left, right) = state.goal(goal)?;
        match next_step(state.formulas(), left, right) {
            Some(step) => {
                let opened = state.apply(goal, &step)?;
                // The first premise is expanded first, so it waits last.
                goals.extend(opened.into_iter().rev());
            }
            None => open.push(goal),
        }
    }
    let Some(&first) = open.first() else {
        let derivation = state.finished(limits, &mut stop)?;
        let tableau = Tableau::of(&derivation, &mut stop)?;
        return Ok(super::Outcome {
            verdict: super::Verdict::Valid(super::Evidence::Derivation(Box::new(derivation))),
            logic,
            search: super::Search::Tableau(Box::new(tableau)),
            countermodel: None,
        });
    };
    let view = ViewOptions::default().with_structural(Structural::Explicit);
    let derivation = state.derivation_within(&view, limits, &mut stop)?;
    let tableau = Tableau::of(&derivation, &mut stop)?;
    // The atoms true on the open branch, which falsify its sequent and so
    // the sequent at its root.
    let formulas = state.formulas();
    let (left, _) = state.goal(first)?;
    let atoms = left
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
        })
        .collect();
    let model = Model::new(vec![World::new(atoms, false, Vec::new())], 0);
    let countermodel =
        super::Countermodel::found(sequent.clone(), model, logic, limits, &mut stop)?;
    Ok(super::Outcome {
        verdict: super::Verdict::NotValid,
        logic,
        search: super::Search::Tableau(Box::new(tableau)),
        countermodel,
    })
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::{Evidence, Logic, Options, Sequent, Verdict, decide};
    use super::{End, Tableau};

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
    }
}
