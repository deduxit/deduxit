// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A derivation in a calculus given as data, laid out as a tree for the
//! exports.

use super::derivation::Kind;
use super::{Calculus, Derivation, NodeId, NodeRef, Step};
use crate::hash::HashMap;
use crate::linear::InfId;
use crate::linear::proofs::Labels;
use crate::linear::proofs::style::{Drawn, LITERAL, RAISED, superscript};
use crate::{Error, Limits};
use std::fmt::Write;

/// What the estimate counts per inference of the tree, in bytes: its
/// line, its label and the layout's numbers.
const INFERENCE: u64 = 256;

/// What the estimate counts per member of an inference's sequent, in
/// bytes: a short formula's text.
const MEMBER: u64 = 64;

/// A derivation in a calculus given as data, with the calculus that
/// writes its formulas, as the exports draw it: a proof tree, each node
/// under the rule that concludes it.
///
/// Every export takes it as it takes a derivation of LK:
/// `export::svg::to_string(&drawing, &style)`. A node that is the premise
/// of several nodes is drawn once for each, and an open goal as the style
/// draws one.
///
/// Natural deduction is drawn as textbooks draw it: a hypothesis is a leaf
/// that is its formula alone, with no line over it. The hypotheses a rule
/// discharges are bracketed and marked with a number, `[A]¹`, and the rule
/// that discharges them carries the same number, `→I¹`; a rule that
/// discharges in several premises carries each premise's number, `∨E¹˒²`.
/// The numbers count the discharges from the leaves up. A hypothesis no
/// rule discharges, an assumption, stands without brackets.
///
/// The inferences are numbered from the leaves, each after its premises,
/// premises in order: the SVG's `i<n>`. Where the nodes are stored in that
/// order already, every node under the root and the premise of one node
/// at most, inference `n` is node `n`: a session's derivation
/// ([`Interactive::derivation`](super::Interactive::derivation)) is stored
/// so.
///
/// Needs the cargo feature `serialize`.
#[derive(Clone, Debug)]
pub struct Drawing<'a> {
    /// The calculus whose print forms write the formulas.
    calculus: &'a Calculus,
    /// The derivation drawn.
    derivation: &'a Derivation,
    /// The node each inference draws, premises before the inferences
    /// they are premises of.
    nodes: Vec<NodeId>,
    /// Per inference, where its premises end in `premises`.
    ends: Vec<u32>,
    /// The premises of every inference.
    premises: Vec<InfId>,
    /// The label of each rule in the label markup: the derivation's rule
    /// names in their order, then a hypothesis's (none: it is drawn
    /// alone), then each rule with the marks of what it discharges.
    labels: Vec<String>,
    /// Per inference, its rule among `labels`.
    rules: Vec<u32>,
    /// Per scope that a rule discharges, its mark.
    marks: HashMap<u32, u32>,
}

impl<'a> Drawing<'a> {
    /// Returns the derivation laid out as a tree from its last node, its
    /// formulas written by `calculus`.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidDerivation`] for a derivation without nodes, or
    /// whose sequents have other zones than the calculus's;
    /// [`Refusal::Output`](crate::Refusal::Output) when the tree, a node
    /// shared by several drawn once for each, is estimated past
    /// `limits.derivation_bytes`, and [`Refusal::Work`](crate::Refusal::Work)
    /// when telling the rules' discharges takes more than `limits.work`
    /// steps up the scopes.
    pub fn new(
        calculus: &'a Calculus,
        derivation: &'a Derivation,
        limits: &Limits,
    ) -> Result<Self, Error> {
        let root = drawn_root(calculus, derivation)?;
        let refused = |estimate, limit| {
            Error::Refused(crate::limits::Refusal::Output {
                what: "drawing",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            })
        };

        // A walk from the root that makes each inference after its
        // premises', with a stack of its own.
        enum Visit {
            /// A node met on the way up.
            Enter(NodeId),
            /// A node whose premises' inferences are made.
            Exit(NodeId),
        }
        let (mut nodes, mut ends, mut premises) = (Vec::new(), Vec::new(), Vec::new());
        let mut made: Vec<InfId> = Vec::new();
        let mut stack = vec![Visit::Enter(root.id())];
        let mut estimate = 0u64;
        while let Some(visit) = stack.pop() {
            match visit {
                Visit::Enter(node) => {
                    let at = derivation.node(node);
                    let members = (0..derivation.zones()).map(|z| at.zone(z).len() as u64);
                    estimate = estimate
                        .saturating_add(INFERENCE)
                        .saturating_add(MEMBER.saturating_mul(members.sum()));
                    if let Some(limit) = limits.derivation_bytes
                        && estimate > limit
                    {
                        return Err(refused(estimate, limit));
                    }
                    stack.push(Visit::Exit(node));
                    stack.extend(at.premises().iter().rev().map(|&p| Visit::Enter(p)));
                }
                Visit::Exit(node) => {
                    let count = derivation.node(node).premises().len();
                    premises.extend(made.drain(made.len() - count..));
                    let id = InfId::new(
                        u32::try_from(nodes.len())
                            .map_err(|_| refused(estimate, u64::from(u32::MAX)))?,
                    );
                    nodes.push(node);
                    ends.push(
                        u32::try_from(premises.len())
                            .map_err(|_| refused(estimate, u64::from(u32::MAX)))?,
                    );
                    made.push(id);
                }
            }
        }
        let mut labels: Vec<String> = derivation
            .rules
            .iter()
            .map(|name| markup(name))
            .chain(std::iter::once(String::new()))
            .collect();
        // The discharges numbered from the leaves up, then each rule that
        // discharges as a rule of its own, labelled with its marks.
        let hypothesis = derivation.rules.len() as u32;
        let scope = |node: NodeId| derivation.node(node).scope().index() as u32;
        let mut marks: HashMap<u32, u32> = HashMap::default();
        let mut marked: HashMap<(u32, Vec<u32>), u32> = HashMap::default();
        let mut rules = Vec::with_capacity(nodes.len());
        let parents = derivation.scope_parents();
        let depth = super::fitch::depths(parents);
        let mut climb = super::fitch::Climb {
            parents,
            depth: &depth,
            work: 0,
            limit: limits.work,
        };
        for &node in &nodes {
            let rule = match derivation.nodes[node.index()].kind {
                Kind::Rule(rule) => rule,
                Kind::Hypothesis => hypothesis,
                // Past every label: `open()` names it.
                Kind::Open => u32::MAX,
            };
            let at = derivation.node(node);
            let mut own = Vec::new();
            let outer = scope(node);
            for &premise in at.premises() {
                // A premise in the node's scope or one around it is cited;
                // one in a scope inside those is the subproof the node
                // discharges.
                let inner = scope(premise);
                if inner == outer || climb.inside(outer, inner)?.is_some() {
                    continue;
                }
                let next = marks.len() as u32 + 1;
                own.push(*marks.entry(inner).or_insert(next));
            }
            if own.is_empty() || rule == u32::MAX {
                rules.push(rule);
                continue;
            }
            let index = *marked.entry((rule, own.clone())).or_insert_with(|| {
                let numbers: Vec<String> = own.iter().map(u32::to_string).collect();
                labels.push(format!(
                    "{}{RAISED}{{{}}}",
                    labels[rule as usize],
                    numbers.join(",")
                ));
                labels.len() as u32 - 1
            });
            rules.push(index);
        }
        let open = labels.len() as u32;
        for rule in &mut rules {
            if *rule == u32::MAX {
                *rule = open;
            }
        }
        Ok(Self {
            calculus,
            derivation,
            nodes,
            ends,
            premises,
            labels,
            rules,
            marks,
        })
    }

    /// Returns the node an inference draws.
    fn node(&self, id: InfId) -> NodeId {
        self.nodes[id.index()]
    }

    /// Returns the node each inference draws, in the inferences' order:
    /// the SVG's `i<n>` draws the `n`th. A node shared by several is drawn
    /// once for each, so it may stand more than once.
    pub fn nodes(&self) -> impl ExactSizeIterator<Item = NodeId> + '_ {
        self.nodes.iter().copied()
    }

    /// Returns the mark of an inference that is a hypothesis some rule
    /// discharges, or `None`.
    fn mark(&self, id: InfId) -> Option<u32> {
        let node = self.derivation.node(self.node(id));
        if node.step() != Step::Hypothesis {
            return None;
        }
        self.marks.get(&(node.scope().index() as u32)).copied()
    }
}

/// Returns a rule's name in the label markup: a subscript digit lowered,
/// as the drawings set the labels of LK, and `_` and `*`, which the
/// markup reads as a subscript and a star, kept as text.
pub(super) fn markup(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        match c {
            '₀'..='₉' => {
                out.push('_');
                out.extend(char::from_digit(u32::from(c) - u32::from('₀'), 10));
            }
            '_' | '*' => {
                out.push(LITERAL);
                out.push(c);
            }
            c => out.push(c),
        }
    }
    out
}

impl Drawn for Drawing<'_> {
    fn rules(&self) -> usize {
        self.labels.len() + 1
    }

    fn open(&self) -> Option<usize> {
        Some(self.labels.len())
    }

    fn markup<'a>(&'a self, rule: usize, labels: &'a Labels) -> Option<&'a str> {
        match labels {
            Labels::Off => None,
            Labels::Upright | Labels::Subscript | Labels::Table(_) => {
                self.labels.get(rule).map(String::as_str)
            }
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn root(&self) -> InfId {
        // `new` makes the root's inference last, and at least one.
        InfId::new(u32::try_from(self.nodes.len() - 1).unwrap_or(u32::MAX))
    }

    fn premises(&self, id: InfId) -> &[InfId] {
        let start = match id.index() {
            0 => 0,
            index => self.ends[index - 1] as usize,
        };
        &self.premises[start..self.ends[id.index()] as usize]
    }

    fn rule(&self, id: InfId) -> usize {
        self.rules[id.index()] as usize
    }

    fn bare(&self) -> Option<usize> {
        Some(self.derivation.rules.len())
    }

    fn name(&self, id: InfId) -> &str {
        match self.derivation.node(self.node(id)).step() {
            Step::Rule(name) => name,
            Step::Hypothesis => "hypothesis",
            Step::Open => "open",
        }
    }

    fn times(&self, _: InfId) -> u32 {
        1
    }

    fn write_sequent(&self, out: &mut impl Write, id: InfId) -> std::fmt::Result {
        let shown = self.calculus.show_sequent(self.derivation, self.node(id));
        match self.mark(id) {
            None => write!(out, "{shown}"),
            Some(mark) => {
                write!(out, "[{shown}]")?;
                mark.to_string()
                    .chars()
                    .try_for_each(|c| out.write_char(superscript(c)))
            }
        }
    }

    #[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
    fn sequent(
        &self,
        out: &mut String,
        notation: &crate::linear::sequents::notation::Notation,
        id: InfId,
        aligned: bool,
        marks: bool,
    ) {
        let spelling = super::print::Spelling::of(notation, aligned, marks);
        let mark = self.mark(id);
        if mark.is_some() {
            out.push('[');
        }
        self.calculus
            .write_sequent_in(self.derivation, self.node(id), spelling, out);
        if let Some(mark) = mark {
            out.push(']');
            // Writing into a `String` does not fail.
            let _ = notation.atom.superscript(out, &mark.to_string());
        }
    }

    #[cfg(feature = "svg")]
    fn positions(&self, id: InfId) -> Vec<usize> {
        let node = self.derivation.node(self.node(id));
        let members = (0..self.derivation.zones()).map(|z| node.zone(z).len());
        (0..members.sum()).collect()
    }
}

/// Returns the root of a derivation that a layout of `calculus` draws:
/// its sequents have the calculus's zones, and it has a node.
///
/// # Errors
///
/// [`Error::InvalidDerivation`] for other zones or no node.
pub(super) fn drawn_root<'d>(
    calculus: &Calculus,
    derivation: &'d Derivation,
) -> Result<NodeRef<'d>, Error> {
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
    derivation
        .root()
        .ok_or_else(|| invalid("it has no node".to_owned()))
}
