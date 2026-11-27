// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! How both sequentializations put the proof together. A stage looks at a
//! sub-net and either proves it by an axiom or leaves steps: the sub-nets
//! that prove it and the rule that joins them. The steps wait on a stack
//! of their own, taken in the order a recursion would make its calls, so
//! that a derivation of any height costs the caller's stack nothing.

use crate::linear::occurrences::{Member, OccId};
use crate::linear::proofs::{Node, NodeId};

/// A rule a stage opened on a conclusion, applied below what proves the
/// rest.
#[derive(Clone, Copy)]
pub(super) enum Open {
    /// A `⅋` rule.
    Par(OccId),
    /// A `?` rule.
    Quest(OccId),
    /// A copy.
    Copy(OccId),
}

/// A rule that a stage leaves to apply to the sub-nets proved after it.
pub(super) enum Rule {
    /// Join the last two sub-nets proved with Mix.
    Mix,
    /// Apply this `⊗` to the last two sub-nets proved, which hold its left
    /// and its right premise.
    Tensor(OccId),
    /// Cut on this occurrence of a cut formula's tree the last two
    /// sub-nets proved, which hold it and its dual.
    Cut(OccId),
    /// Apply this promotion to the last sub-net proved.
    Bang(OccId),
    /// Apply these rules to the last sub-net proved, the last first.
    Opened(Vec<Open>),
}

/// A step of a sequentialization, with `G` what names a sub-net to prove.
enum Step<G> {
    /// Prove this sub-net.
    Prove(G),
    /// Apply this rule.
    Apply(Rule),
}

/// The proof made so far, the steps left and the sub-proofs waiting.
pub(super) struct Assembly<G> {
    /// The proof's arena so far.
    pub(super) nodes: Vec<Node>,
    /// What is yet to do, the next step last.
    steps: Vec<Step<G>>,
    /// The nodes concluding the sub-nets proved and not yet used as a
    /// premise, the last proved last.
    proved: Vec<NodeId>,
}

impl<G> Assembly<G> {
    /// Returns an empty assembly with room for `nodes` nodes.
    pub(super) fn with_capacity(nodes: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(nodes),
            steps: Vec::new(),
            proved: Vec::new(),
        }
    }

    /// Leaves a sub-net to prove, after the steps left later.
    pub(super) fn prove(&mut self, goal: G) {
        self.steps.push(Step::Prove(goal));
    }

    /// Leaves a rule to apply, after the steps left later.
    pub(super) fn apply(&mut self, rule: Rule) {
        self.steps.push(Step::Apply(rule));
    }

    /// Proves the sub-net of the stage by this node, which has no premise.
    pub(super) fn proved_by(&mut self, node: Node) {
        let id = self.push(node);
        self.proved.push(id);
    }

    /// Adds a node and returns its id.
    fn push(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        NodeId::new(self.nodes.len() as u32 - 1)
    }

    /// Takes the node concluding the last sub-net proved.
    fn premise(&mut self) -> NodeId {
        self.proved
            .pop()
            .expect("a sub-net proved for every premise")
    }

    /// Applies a rule to the sub-nets proved, and returns the node it made.
    fn join(&mut self, rule: Rule) -> NodeId {
        match rule {
            Rule::Mix => {
                let other = self.premise();
                let node = self.premise();
                self.push(Node::Mix(node, other))
            }
            Rule::Tensor(t) => {
                let right = self.premise();
                let left = self.premise();
                self.push(Node::Tensor(Member::from(t), left, right))
            }
            Rule::Cut(a) => {
                let right = self.premise();
                let left = self.premise();
                self.push(Node::Cut(Member::from(a), left, right))
            }
            Rule::Bang(b) => {
                let premise = self.premise();
                self.push(Node::Bang(Member::from(b), premise))
            }
            Rule::Opened(rules) => {
                let mut node = self.premise();
                for &rule in rules.iter().rev() {
                    node = self.push(match rule {
                        Open::Par(o) => Node::Par(Member::from(o), node),
                        Open::Quest(o) => Node::Quest(Member::from(o), node),
                        Open::Copy(o) => Node::Copy(Member::from(o), node),
                    });
                }
                node
            }
        }
    }
}

/// The parts of a sub-net that falls apart, by the label of the search
/// tree that reached each conclusion. Sorting the labels keeps a stage of
/// many parts near linear, where a search of the labels per conclusion and
/// a filter per part were quadratic.
pub(super) struct Parts {
    /// The labels, sorted, one per part: a part's index is its label's.
    labels: Vec<u32>,
    /// The parts' indices in the order of their first conclusion.
    order: Vec<usize>,
}

impl Parts {
    /// Returns the parts of the conclusions whose labels come in order.
    pub(super) fn of(labels: impl Iterator<Item = u32>) -> Self {
        let mut first: Vec<(u32, usize)> = labels.enumerate().map(|(i, l)| (l, i)).collect();
        first.sort_unstable();
        first.dedup_by_key(|&mut (label, _)| label);
        let labels: Vec<u32> = first.iter().map(|&(label, _)| label).collect();
        first.sort_unstable_by_key(|&(_, i)| i);
        let order = first
            .iter()
            .map(|&(label, _)| labels.binary_search(&label).expect("every label is listed"))
            .collect();
        Self { labels, order }
    }

    /// Returns the number of parts.
    pub(super) fn len(&self) -> usize {
        self.labels.len()
    }

    /// Returns the index of the part a label names.
    pub(super) fn index(&self, label: u32) -> usize {
        self.labels
            .binary_search(&label)
            .expect("every label is listed")
    }

    /// Leaves the steps that prove the parts, `goals` by index: the first
    /// part, then each other one and the Mix that joins it to those before.
    pub(super) fn leave<G: Default>(&self, assembly: &mut Assembly<G>, mut goals: Vec<G>) {
        // The steps are taken from the last.
        for (i, &j) in self.order.iter().enumerate().rev() {
            if i > 0 {
                assembly.apply(Rule::Mix);
            }
            assembly.prove(std::mem::take(&mut goals[j]));
        }
    }
}

/// A sequentialization: its stage, over an assembly of its own.
pub(super) trait Assemble {
    /// What names a sub-net to prove.
    type Goal;

    /// Returns the assembly.
    fn assembly(&mut self) -> &mut Assembly<Self::Goal>;

    /// Runs one stage on a sub-net: proves it by an axiom, or leaves the
    /// steps that prove it.
    fn stage(&mut self, goal: Self::Goal);

    /// Proves the sub-net `goal` and returns the node concluding it,
    /// asking `stop` before each stage; `None` when it fired.
    fn run(&mut self, goal: Self::Goal, stop: &mut dyn FnMut(u64) -> bool) -> Option<NodeId> {
        self.assembly().prove(goal);
        while let Some(step) = self.assembly().steps.pop() {
            match step {
                Step::Prove(goal) => {
                    if stop(1) {
                        return None;
                    }
                    self.stage(goal);
                }
                Step::Apply(rule) => {
                    let assembly = self.assembly();
                    let node = assembly.join(rule);
                    assembly.proved.push(node);
                }
            }
        }
        Some(self.assembly().premise())
    }
}
