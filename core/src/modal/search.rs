// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The search: one labelled tableau for every system, read upside down as
//! backward proof search in the system's labelled calculus
//! ([`calculus`](crate::modal::calculus())).
//!
//! A branch holds worlds, each with the formulas true and false there,
//! and the relation between them, kept closed under the system's frame
//! rules. Rules apply in this order: a branch closes on a formula true
//! and false at one world, `⊥` true or `⊤` false; the rules of one
//! premise and `□` true or `◇` false along every edge, to a fixpoint;
//! then a rule of two premises splits the branch, the oldest world
//! first; then a `□` false or a `◇` true without a witness makes a new
//! world, and in a serial system without reflexivity a world with a `□`
//! true or a `◇` false of its own and no edge makes one. A branch where
//! nothing applies is open: its worlds are a countermodel.
//!
//! The search ends without a loop check where the modal depth falls from
//! a world to the worlds it makes (K, D, T, KB, KDB, B), and in the
//! euclidean systems, whose worlds but the first form one cluster that
//! shares its witnesses. In K4, KD4 and S4 a world makes no world while
//! an earlier world that no world blocks holds all it holds, the modal
//! formulas true at every world that sees it counted: Garg, Genovese and
//! Negri's blocking ("Countermodels from sequent calculi in multi-modal
//! logics", LICS 2012), by any earlier world where they take an ancestor,
//! as anywhere blocking does in description logics without inverse
//! roles. Whatever the argument, `limits.work` bounds the search.
//!
//! Each step is one inference of the calculus on the branch's labelled
//! sequent. A closed branch is a derivation, built from the steps it
//! needs only: a step whose additions nothing above it uses is left out,
//! and a split whose first premise closes without its addition is not
//! searched further, so that independent parts of a branch do not
//! multiply.
//!
//! The search is not trusted for "valid": the derivation passes the
//! generic checker before the outcome is given. Its countermodel passes
//! the model check too, unless it is too large to check, where "not
//! valid" rests on the arguments above.

use super::calculus::Rule;
use super::{Condition, Node, NodeId, Sequent, System};
use crate::hash::HashSet;
use crate::kripke::{Model, World as ModelWorld};
use crate::limits::{Limits, Phase, Progress};
use crate::linear::search::Reason;
use crate::linear::search::memory::Account;
use std::collections::VecDeque;

/// The units of work between two asks of the stop.
pub(super) const POLL: u64 = 1 << 8;

/// What a branch holds: a formula at a world, true (left of `⊢`) or
/// false (right of it), or an edge of the relation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) enum Member {
    /// `world : node`, left of `⊢` or right of it.
    Formula {
        /// The world.
        world: u32,
        /// The formula, an index of the sequent's arena.
        node: u32,
        /// Whether it stands right of `⊢`: false there.
        right: bool,
    },
    /// `x R y`.
    Edge(u32, u32),
}

/// Returns the member of a formula at a world on a side.
pub(super) const fn formula(world: u32, node: NodeId, right: bool) -> Member {
    Member::Formula {
        world,
        node: node.0,
        right,
    }
}

/// A step of the search: one inference of the calculus.
#[derive(Clone, Debug)]
pub(super) struct Step {
    /// The rule.
    pub(super) rule: Rule,
    /// The members of the conclusion the rule writes out, in the order a
    /// node gives their positions: the left zone's, then the right
    /// zone's, each in the rule's order.
    pub(super) written: Vec<Member>,
    /// Per premise, the members it holds beyond the conclusion.
    pub(super) premises: Vec<Vec<Member>>,
    /// The member of the first premise a node gives by position: the new
    /// edge of a rule that makes a world, and `Ref`'s edge.
    pub(super) located: Option<Member>,
    /// Per premise, those of its members that were new on the branch.
    new: Vec<Vec<Member>>,
}

/// A node of a closed tableau: a step and the nodes of its premises.
#[derive(Clone, Debug)]
pub(super) struct ProofNode {
    /// The step, an index of the search's steps.
    pub(super) step: u32,
    /// The premises' nodes, indices of the search's proof nodes.
    pub(super) premises: Vec<u32>,
}

/// A world of a branch.
#[derive(Clone, Debug)]
struct World {
    /// A bit per formula of the arena: true here.
    truth: Vec<u64>,
    /// A bit per formula of the arena: false here.
    falsity: Vec<u64>,
    /// The worlds it sees, sorted.
    sees: Vec<u32>,
    /// The worlds that see it, sorted.
    seen_by: Vec<u32>,
    /// What it holds for blocking, once asked: its formulas true and
    /// false, with every `□` true and `◇` false at a world that sees it.
    signature: Option<(Vec<u64>, Vec<u64>)>,
    /// Whether an earlier world blocks it, once asked.
    blocked: Blocked,
}

/// Whether a world is blocked, as far as the search asked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Blocked {
    /// Not asked yet.
    Unknown,
    /// No earlier world blocks it.
    No,
    /// This earlier world blocks it.
    By(u32),
}

/// One branch of the tableau.
#[derive(Clone, Debug)]
struct Branch {
    /// The worlds, in the order made.
    worlds: Vec<World>,
    /// The members added and not yet acted on, oldest first.
    agenda: VecDeque<Member>,
    /// The formulas of a rule of two premises, not yet split on.
    betas: Vec<(u32, NodeId, bool)>,
    /// The `□` false and `◇` true, not yet given a witness.
    demands: Vec<(u32, NodeId, bool)>,
    /// The steps since the last split, in order.
    segment: Vec<u32>,
}

/// A split waiting for its premises.
#[derive(Debug)]
struct Frame {
    /// The split's step.
    step: u32,
    /// The steps of the branch before the split.
    segment: Vec<u32>,
    /// The branch at the split, for the second premise.
    second: Option<Branch>,
    /// The first premise's proof and the members it needs, once closed.
    first: Option<Closed>,
    /// The bytes the frame holds.
    bytes: usize,
}

/// A closed part of the tableau: its proof node and the members of its
/// conclusion that the proof uses.
type Closed = (u32, HashSet<Member>);

/// What saturating a branch led to.
enum Saturated {
    /// It closed by this step.
    Closed(u32),
    /// A rule of two premises applies: this step.
    Split(u32),
    /// Nothing applies.
    Open,
}

/// What the search found.
pub(super) enum Found {
    /// Every branch closed: the proof's root node.
    Closed(u32),
    /// A branch is open: its worlds as a model, not yet checked.
    Open(Model),
}

/// What the search counted.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Counts {
    /// The worlds made, on every branch.
    pub(super) worlds: u64,
    /// The splits.
    pub(super) splits: u64,
    /// The tests that found a world blocked.
    pub(super) blocked: u64,
    /// The units of work.
    pub(super) work: u64,
}

/// The search of one sequent in one system.
pub(super) struct Search<'a> {
    /// The sequent.
    sequent: &'a Sequent,
    /// The words of a world's bit sets.
    words: usize,
    /// A bit per formula: whether it is a `□`.
    boxes: Vec<u64>,
    /// A bit per formula: whether it is a `◇`.
    diamonds: Vec<u64>,
    /// Whether the relation is reflexive.
    reflexive: bool,
    /// Whether it is serial without being reflexive: `Ser` makes worlds.
    serial: bool,
    /// Whether it is symmetric.
    symmetric: bool,
    /// Whether it is transitive.
    transitive: bool,
    /// Whether it is euclidean.
    euclidean: bool,
    /// Whether a world may be blocked: transitive, neither symmetric nor
    /// euclidean.
    blocking: bool,
    /// The steps, on every branch.
    pub(super) steps: Vec<Step>,
    /// The nodes of the closed parts.
    pub(super) proofs: Vec<ProofNode>,
    /// The memory the search holds.
    account: Account,
    /// The bound on the units of work.
    work: u64,
    /// The units of work at the last ask of the stop.
    asked: u64,
    /// Units of work done by a scan since the last tick, counted at the
    /// next.
    owed: u64,
    /// Whether the steps or the proof nodes passed what a `u32` indexes:
    /// only a search without a memory bound gets there (a step holds 128
    /// bytes and more), and the next tick refuses it.
    full: bool,
    /// What the search counted.
    pub(super) counts: Counts,
}

/// The bytes of a step besides its members.
const STEP_BYTES: usize = 128;

/// The bytes of a member in a vector or a set.
const MEMBER_BYTES: usize = 32;

/// Returns whether a bit is set.
fn bit(set: &[u64], i: usize) -> bool {
    set[i / 64] >> (i % 64) & 1 == 1
}

impl<'a> Search<'a> {
    /// Returns a search of `sequent` in `system` within `limits`.
    pub(super) fn new(sequent: &'a Sequent, system: System, limits: &Limits) -> Self {
        let width = sequent.formulas().len();
        let words = width.div_ceil(64);
        let mut boxes = vec![0u64; words];
        let mut diamonds = vec![0u64; words];
        for i in 0..width {
            match sequent.formulas().node(NodeId(i as u32)) {
                Node::Box(_) => boxes[i / 64] |= 1 << (i % 64),
                Node::Diamond(_) => diamonds[i / 64] |= 1 << (i % 64),
                Node::Atom(_)
                | Node::True
                | Node::False
                | Node::Not(_)
                | Node::And(..)
                | Node::Or(..)
                | Node::Implies(..)
                | Node::Iff(..) => {}
            }
        }
        let has = |c| system.has(c);
        Self {
            sequent,
            words,
            boxes,
            diamonds,
            reflexive: has(Condition::Reflexive),
            serial: has(Condition::Serial) && !has(Condition::Reflexive),
            symmetric: has(Condition::Symmetric),
            transitive: has(Condition::Transitive),
            euclidean: has(Condition::Euclidean),
            blocking: has(Condition::Transitive)
                && !has(Condition::Symmetric)
                && !has(Condition::Euclidean),
            steps: Vec::new(),
            proofs: Vec::new(),
            account: Account::holding(limits.memory_bytes, 0),
            work: limits.work.unwrap_or(u64::MAX),
            asked: 0,
            owed: 0,
            full: false,
            counts: Counts::default(),
        }
    }

    /// Counts `units` of work and those owed, refuses past the bound,
    /// asks the stop every [`POLL`] units and refuses past the memory
    /// bound and past the indices.
    fn tick(&mut self, units: u64, stop: &mut impl FnMut(Progress) -> bool) -> Result<(), Reason> {
        if self.full {
            return Err(Reason::IndexLimit);
        }
        let units = units.saturating_add(std::mem::take(&mut self.owed));
        let done = self.counts.work.saturating_add(units);
        self.counts.work = done;
        if done > self.work {
            return Err(Reason::WorkLimit { limit: self.work });
        }
        // `asked` is at most `done`, so the difference does not wrap.
        if done - self.asked >= POLL {
            let since = done - self.asked;
            self.asked = done;
            if stop(Progress::new(Phase::Search, since, done)) {
                return Err(Reason::Stopped);
            }
        }
        if self.account.over() {
            return Err(Reason::MemoryLimit {
                limit_bytes: self.account.limit(),
            });
        }
        Ok(())
    }

    /// Returns the bytes a branch holds, saturating.
    fn bytes(&self, branch: &Branch) -> usize {
        // Two bit sets, and two more for a signature.
        let world = self.words.saturating_mul(32).saturating_add(80);
        let edges: usize = branch.worlds.iter().map(|w| w.sees.len()).sum();
        branch
            .worlds
            .len()
            .saturating_mul(world)
            .saturating_add(edges.saturating_mul(8))
            .saturating_add(branch.agenda.len().saturating_mul(MEMBER_BYTES))
            .saturating_add((branch.betas.len() + branch.demands.len()).saturating_mul(12))
            .saturating_add(branch.segment.len().saturating_mul(4))
    }

    /// Returns a `u32` index of a vector of this length, noting where it
    /// passes what a `u32` holds.
    fn index(&mut self, len: usize) -> u32 {
        u32::try_from(len).unwrap_or_else(|_| {
            self.full = true;
            u32::MAX
        })
    }

    /// Runs the search to its end.
    pub(super) fn run(&mut self, mut stop: impl FnMut(Progress) -> bool) -> Result<Found, Reason> {
        let stop = &mut stop;
        let mut branch = Branch {
            worlds: Vec::new(),
            agenda: VecDeque::new(),
            betas: Vec::new(),
            demands: Vec::new(),
            segment: Vec::new(),
        };
        self.world(&mut branch)?;
        for (&id, right) in self
            .sequent
            .left()
            .iter()
            .map(|id| (id, false))
            .chain(self.sequent.right().iter().map(|id| (id, true)))
        {
            Self::add(&mut branch, formula(0, id, right));
        }
        if self.reflexive {
            self.linear(
                &mut branch,
                Rule::Reflexive,
                Vec::new(),
                vec![Member::Edge(0, 0)],
                true,
            );
        }
        let mut frames: Vec<Frame> = Vec::new();
        loop {
            match self.saturate(&mut branch, stop)? {
                Saturated::Open => return Ok(Found::Open(self.model(&mut branch, stop)?)),
                Saturated::Split(step) => {
                    self.counts.splits += 1;
                    // The second premise starts a segment of its own.
                    let segment = std::mem::take(&mut branch.segment);
                    let second = branch.clone();
                    let bytes = self.bytes(&second);
                    self.account.charge(bytes);
                    self.premise(&mut branch, step, 0);
                    frames.push(Frame {
                        step,
                        segment,
                        second: Some(second),
                        first: None,
                        bytes,
                    });
                }
                Saturated::Closed(step) => {
                    let segment = std::mem::take(&mut branch.segment);
                    let mut closed = self.leaf(step);
                    closed = self.fold(&segment, closed);
                    loop {
                        let Some(mut frame) = frames.pop() else {
                            return Ok(Found::Closed(closed.0));
                        };
                        let step = frame.step;
                        match frame.first.take() {
                            None if !self.uses(step, 0, &closed.1) => {
                                // The first premise's proof does not use
                                // the split: it is a proof of the
                                // conclusion, and the second premise
                                // needs no search.
                                self.account.release(frame.bytes);
                                closed = self.fold(&frame.segment, closed);
                            }
                            None => {
                                let mut second = frame
                                    .second
                                    .take()
                                    .unwrap_or_else(|| unreachable!("a split keeps its branch"));
                                self.premise(&mut second, step, 1);
                                frame.first = Some(closed);
                                frames.push(frame);
                                branch = second;
                                break;
                            }
                            Some(first) => {
                                self.account.release(frame.bytes);
                                if self.uses(step, 1, &closed.1) {
                                    closed = self.join(step, first, closed);
                                }
                                closed = self.fold(&frame.segment, closed);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Returns whether the members the premise `premise` of `step` added
    /// to its branch are among `needed`.
    fn uses(&self, step: u32, premise: usize, needed: &HashSet<Member>) -> bool {
        self.steps[step as usize].new[premise]
            .iter()
            .any(|m| needed.contains(m))
    }

    /// Returns the closed part of a step that closes a branch.
    fn leaf(&mut self, step: u32) -> Closed {
        let needed = self.steps[step as usize].written.iter().copied().collect();
        (self.node(step, Vec::new()), needed)
    }

    /// Adds a proof node and returns its index.
    fn node(&mut self, step: u32, premises: Vec<u32>) -> u32 {
        self.account
            .charge(16 + 4 * premises.len() + std::mem::size_of::<ProofNode>());
        let index = self.index(self.proofs.len());
        self.proofs.push(ProofNode { step, premises });
        index
    }

    /// Returns the closed part of a split from its two premises'.
    fn join(&mut self, step: u32, first: Closed, second: Closed) -> Closed {
        let record = &self.steps[step as usize];
        let mut needed = first.1;
        for m in &record.premises[0] {
            needed.remove(m);
        }
        let mut more = second.1;
        for m in &record.premises[1] {
            more.remove(m);
        }
        needed.extend(more);
        needed.extend(record.written.iter().copied());
        (self.node(step, vec![first.0, second.0]), needed)
    }

    /// Returns the closed part of the conclusion of the steps of a
    /// segment, from the closed part of its last premise: a step whose
    /// new members the proof above does not use is left out.
    fn fold(&mut self, segment: &[u32], mut closed: Closed) -> Closed {
        for &step in segment.iter().rev() {
            if !self.uses(step, 0, &closed.1) {
                continue;
            }
            let record = &self.steps[step as usize];
            let (premise, written) = (record.premises[0].clone(), record.written.clone());
            for m in &premise {
                closed.1.remove(m);
            }
            closed.1.extend(written);
            closed.0 = self.node(step, vec![closed.0]);
        }
        closed
    }

    /// Adds a member to the branch, unless it holds it, and returns
    /// whether it was new.
    fn add(branch: &mut Branch, member: Member) -> bool {
        let new = match member {
            Member::Formula { world, node, right } => {
                let w = &mut branch.worlds[world as usize];
                let set = if right { &mut w.falsity } else { &mut w.truth };
                let i = node as usize;
                let new = !bit(set, i);
                set[i / 64] |= 1 << (i % 64);
                new
            }
            Member::Edge(x, y) => {
                let sees = &mut branch.worlds[x as usize].sees;
                match sees.binary_search(&y) {
                    Ok(_) => false,
                    Err(at) => {
                        sees.insert(at, y);
                        let seen_by = &mut branch.worlds[y as usize].seen_by;
                        let at = seen_by.binary_search(&x).unwrap_or_else(|at| at);
                        seen_by.insert(at, x);
                        true
                    }
                }
            }
        };
        if new {
            branch.agenda.push_back(member);
        }
        new
    }

    /// Returns whether the branch holds a member.
    fn holds(branch: &Branch, member: Member) -> bool {
        match member {
            Member::Formula { world, node, right } => {
                let w = &branch.worlds[world as usize];
                bit(if right { &w.falsity } else { &w.truth }, node as usize)
            }
            Member::Edge(x, y) => branch.worlds[x as usize].sees.binary_search(&y).is_ok(),
        }
    }

    /// Records a step of one premise and applies it, where some of its
    /// members are new; `located` says that a node gives the first of
    /// them by position.
    fn linear(
        &mut self,
        branch: &mut Branch,
        rule: Rule,
        written: Vec<Member>,
        premise: Vec<Member>,
        located: bool,
    ) {
        if premise.iter().all(|&m| Self::holds(branch, m)) {
            return;
        }
        let located = located.then(|| premise[0]);
        let step = self.record(Step {
            rule,
            written,
            premises: vec![premise],
            located,
            new: Vec::new(),
        });
        self.premise(branch, step, 0);
        branch.segment.push(step);
    }

    /// Adds a step to the arena and returns its index.
    fn record(&mut self, step: Step) -> u32 {
        let members: usize = step.written.len() + step.premises.iter().map(Vec::len).sum::<usize>();
        self.account.charge(STEP_BYTES + 2 * MEMBER_BYTES * members);
        let index = self.index(self.steps.len());
        self.steps.push(step);
        index
    }

    /// Adds a premise's members to the branch and records which were new.
    fn premise(&mut self, branch: &mut Branch, step: u32, premise: usize) {
        let members = self.steps[step as usize].premises[premise].clone();
        let new: Vec<Member> = members
            .into_iter()
            .filter(|&m| Self::add(branch, m))
            .collect();
        let record = &mut self.steps[step as usize];
        if record.new.len() <= premise {
            record.new.resize(premise + 1, Vec::new());
        }
        record.new[premise] = new;
    }

    /// Adds a world and returns it.
    fn world(&mut self, branch: &mut Branch) -> Result<u32, Reason> {
        let index = u32::try_from(branch.worlds.len()).map_err(|_| Reason::IndexLimit)?;
        self.account.charge(2 * self.words * 8 + 64);
        branch.worlds.push(World {
            truth: vec![0; self.words],
            falsity: vec![0; self.words],
            sees: Vec::new(),
            seen_by: Vec::new(),
            signature: None,
            blocked: Blocked::Unknown,
        });
        self.counts.worlds += 1;
        Ok(index)
    }

    /// Applies every rule of one premise and every propagation to a
    /// fixpoint, then returns a split or a new world, or that the branch
    /// closed or is open.
    fn saturate(
        &mut self,
        branch: &mut Branch,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<Saturated, Reason> {
        loop {
            while let Some(member) = branch.agenda.pop_front() {
                self.tick(1, stop)?;
                if let Some(step) = self.act(branch, member) {
                    return Ok(Saturated::Closed(step));
                }
            }
            if let Some(step) = self.split(branch, stop)? {
                return Ok(Saturated::Split(step));
            }
            if !self.witness(branch, stop)? {
                return Ok(Saturated::Open);
            }
        }
    }

    /// Acts on a member just added: closes the branch, or applies the
    /// rules of one premise it is the principal member of, or notes it
    /// for a split or a witness.
    fn act(&mut self, branch: &mut Branch, member: Member) -> Option<u32> {
        match member {
            Member::Formula { world, node, right } => {
                let id = NodeId(node);
                let other = formula(world, id, !right);
                if Self::holds(branch, other) {
                    let (l, r) = if right {
                        (other, member)
                    } else {
                        (member, other)
                    };
                    return Some(self.closing(Rule::Axiom, vec![l, r]));
                }
                let w = world;
                let at = |k: NodeId, right| formula(w, k, right);
                match (self.sequent.formulas().node(id), right) {
                    (Node::False, false) => {
                        return Some(self.closing(Rule::FalseLeft, vec![member]));
                    }
                    (Node::True, true) => return Some(self.closing(Rule::TrueRight, vec![member])),
                    (Node::Atom(_) | Node::True | Node::False, _) => {}
                    (Node::Not(a), false) => {
                        self.linear(
                            branch,
                            Rule::NotLeft,
                            vec![member],
                            vec![at(a, true)],
                            false,
                        );
                    }
                    (Node::Not(a), true) => {
                        self.linear(
                            branch,
                            Rule::NotRight,
                            vec![member],
                            vec![at(a, false)],
                            false,
                        );
                    }
                    (Node::And(a, b), false) => {
                        self.linear(
                            branch,
                            Rule::AndLeft,
                            vec![member],
                            vec![at(a, false), at(b, false)],
                            false,
                        );
                    }
                    (Node::Or(a, b), true) => {
                        self.linear(
                            branch,
                            Rule::OrRight,
                            vec![member],
                            vec![at(a, true), at(b, true)],
                            false,
                        );
                    }
                    (Node::Implies(a, b), true) => {
                        self.linear(
                            branch,
                            Rule::ImpliesRight,
                            vec![member],
                            vec![at(a, false), at(b, true)],
                            false,
                        );
                    }
                    (Node::And(..), true)
                    | (Node::Or(..), false)
                    | (Node::Implies(..), false)
                    | (Node::Iff(..), _) => branch.betas.push((world, id, right)),
                    (Node::Box(a), false) => {
                        let sees = branch.worlds[world as usize].sees.clone();
                        self.owed = self.owed.saturating_add(sees.len() as u64);
                        for y in sees {
                            self.linear(
                                branch,
                                Rule::BoxLeft,
                                vec![member, Member::Edge(world, y)],
                                vec![formula(y, a, false)],
                                false,
                            );
                        }
                    }
                    (Node::Diamond(a), true) => {
                        let sees = branch.worlds[world as usize].sees.clone();
                        self.owed = self.owed.saturating_add(sees.len() as u64);
                        for y in sees {
                            self.linear(
                                branch,
                                Rule::DiamondRight,
                                vec![Member::Edge(world, y), member],
                                vec![formula(y, a, true)],
                                false,
                            );
                        }
                    }
                    (Node::Box(_), true) | (Node::Diamond(_), false) => {
                        branch.demands.push((world, id, right));
                    }
                }
                None
            }
            Member::Edge(x, y) => {
                self.edge(branch, x, y);
                None
            }
        }
    }

    /// Records a step that closes a branch and returns it.
    fn closing(&mut self, rule: Rule, written: Vec<Member>) -> u32 {
        self.record(Step {
            rule,
            written,
            premises: Vec::new(),
            located: None,
            new: Vec::new(),
        })
    }

    /// Acts on a new edge `x R y`: the `□` true and `◇` false of `x` reach
    /// `y`, and the frame rules add the edges it makes with the others.
    fn edge(&mut self, branch: &mut Branch, x: u32, y: u32) {
        let edge = Member::Edge(x, y);
        // The `□` true and `◇` false of `x`, word by word.
        self.owed = self.owed.saturating_add(self.words as u64);
        for word in 0..self.words {
            let w = &branch.worlds[x as usize];
            let mut boxes = w.truth[word] & self.boxes[word];
            let mut diamonds = w.falsity[word] & self.diamonds[word];
            while boxes != 0 {
                let id = NodeId((word * 64) as u32 + boxes.trailing_zeros());
                boxes &= boxes - 1;
                let Node::Box(a) = self.sequent.formulas().node(id) else {
                    unreachable!("a box's bit")
                };
                self.owed = self.owed.saturating_add(1);
                self.linear(
                    branch,
                    Rule::BoxLeft,
                    vec![formula(x, id, false), edge],
                    vec![formula(y, a, false)],
                    false,
                );
            }
            while diamonds != 0 {
                let id = NodeId((word * 64) as u32 + diamonds.trailing_zeros());
                diamonds &= diamonds - 1;
                let Node::Diamond(a) = self.sequent.formulas().node(id) else {
                    unreachable!("a diamond's bit")
                };
                self.owed = self.owed.saturating_add(1);
                self.linear(
                    branch,
                    Rule::DiamondRight,
                    vec![edge, formula(x, id, true)],
                    vec![formula(y, a, true)],
                    false,
                );
            }
        }
        if self.symmetric {
            self.linear(
                branch,
                Rule::Symmetric,
                vec![edge],
                vec![Member::Edge(y, x)],
                false,
            );
        }
        if self.transitive {
            let (sees, seen_by) = (
                branch.worlds[y as usize].sees.clone(),
                branch.worlds[x as usize].seen_by.clone(),
            );
            self.owed = self
                .owed
                .saturating_add((sees.len() + seen_by.len()) as u64);
            for z in sees {
                self.linear(
                    branch,
                    Rule::Transitive,
                    vec![edge, Member::Edge(y, z)],
                    vec![Member::Edge(x, z)],
                    false,
                );
            }
            for w in seen_by {
                self.linear(
                    branch,
                    Rule::Transitive,
                    vec![Member::Edge(w, x), edge],
                    vec![Member::Edge(w, y)],
                    false,
                );
            }
        }
        if self.euclidean {
            let sees = branch.worlds[x as usize].sees.clone();
            self.owed = self.owed.saturating_add(2 * sees.len() as u64);
            for z in sees {
                let other = Member::Edge(x, z);
                self.linear(
                    branch,
                    Rule::Euclidean,
                    vec![edge, other],
                    vec![Member::Edge(y, z)],
                    false,
                );
                self.linear(
                    branch,
                    Rule::Euclidean,
                    vec![other, edge],
                    vec![Member::Edge(z, y)],
                    false,
                );
            }
        }
    }

    /// Returns the step of the first rule of two premises that applies,
    /// the oldest world first, or `None`; drops those that a premise of
    /// holds already.
    fn split(
        &mut self,
        branch: &mut Branch,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<Option<u32>, Reason> {
        self.tick(branch.betas.len() as u64, stop)?;
        // Drop those a premise of holds already: they stay so.
        let mut betas = std::mem::take(&mut branch.betas);
        betas.retain(|&(world, id, right)| {
            let (_, premises) = self.beta(world, id, right);
            !premises
                .iter()
                .any(|p| p.iter().all(|&m| Self::holds(branch, m)))
        });
        branch.betas = betas;
        let best = (0..branch.betas.len())
            .min_by_key(|&i| branch.betas[i].0)
            .map(|index| {
                let (world, id, right) = branch.betas[index];
                let (rule, premises) = self.beta(world, id, right);
                (index, premises, rule)
            });
        let Some((index, [first, second], rule)) = best else {
            return Ok(None);
        };
        let (world, id, right) = branch.betas.remove(index);
        let step = self.record(Step {
            rule,
            written: vec![formula(world, id, right)],
            premises: vec![first, second],
            located: None,
            new: Vec::new(),
        });
        Ok(Some(step))
    }

    /// Returns the rule of two premises of a formula and its premises'
    /// members.
    fn beta(&self, world: u32, id: NodeId, right: bool) -> (Rule, [Vec<Member>; 2]) {
        let at = |k: NodeId, right| formula(world, k, right);
        match (self.sequent.formulas().node(id), right) {
            (Node::And(a, b), true) => (Rule::AndRight, [vec![at(a, true)], vec![at(b, true)]]),
            (Node::Or(a, b), false) => (Rule::OrLeft, [vec![at(a, false)], vec![at(b, false)]]),
            (Node::Implies(a, b), false) => {
                (Rule::ImpliesLeft, [vec![at(a, true)], vec![at(b, false)]])
            }
            (Node::Iff(a, b), false) => (
                Rule::IffLeft,
                [
                    vec![at(a, false), at(b, false)],
                    vec![at(a, true), at(b, true)],
                ],
            ),
            (Node::Iff(a, b), true) => (
                Rule::IffRight,
                [
                    vec![at(a, false), at(b, true)],
                    vec![at(b, false), at(a, true)],
                ],
            ),
            (
                Node::Atom(_)
                | Node::True
                | Node::False
                | Node::Not(_)
                | Node::And(..)
                | Node::Or(..)
                | Node::Implies(..)
                | Node::Box(_)
                | Node::Diamond(_),
                _,
            ) => unreachable!("only a formula of two premises waits for a split"),
        }
    }

    /// Makes a world for the first `□` false or `◇` true without a
    /// witness at a world that is not blocked, the oldest world first, or
    /// by `Ser`; returns whether it made one.
    fn witness(
        &mut self,
        branch: &mut Branch,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<bool, Reason> {
        // Each demand's test looks through the worlds its world sees.
        let cost = branch.demands.iter().fold(0u64, |n, &(world, ..)| {
            n.saturating_add(1 + branch.worlds[world as usize].sees.len() as u64)
        });
        self.tick(cost, stop)?;
        // Drop those with a witness: they keep it.
        let mut demands = std::mem::take(&mut branch.demands);
        demands.retain(|&(world, id, right)| !self.realised(branch, world, id, right));
        branch.demands = demands;
        let mut order: Vec<usize> = (0..branch.demands.len()).collect();
        order.sort_by_key(|&i| branch.demands[i].0);
        let mut best = None;
        for index in order {
            if !self.blocked(branch, branch.demands[index].0, stop)? {
                best = Some(index);
                break;
            }
        }
        if let Some(index) = best {
            let (x, id, right) = branch.demands.remove(index);
            let a = match self.sequent.formulas().node(id) {
                Node::Box(a) | Node::Diamond(a) => a,
                Node::Atom(_)
                | Node::True
                | Node::False
                | Node::Not(_)
                | Node::And(..)
                | Node::Or(..)
                | Node::Implies(..)
                | Node::Iff(..) => unreachable!("a demand is a modal formula"),
            };
            let y = self.world(branch)?;
            let rule = if right {
                Rule::BoxRight
            } else {
                Rule::DiamondLeft
            };
            self.linear(
                branch,
                rule,
                vec![formula(x, id, right)],
                vec![Member::Edge(x, y), formula(y, a, right)],
                true,
            );
            self.made(branch, y);
            return Ok(true);
        }
        if self.serial {
            for x in 0..branch.worlds.len() as u32 {
                self.tick(1, stop)?;
                let w = &branch.worlds[x as usize];
                let own = (0..self.words).any(|i| {
                    w.truth[i] & self.boxes[i] != 0 || w.falsity[i] & self.diamonds[i] != 0
                });
                if w.sees.is_empty() && own && !self.blocked(branch, x, stop)? {
                    let y = self.world(branch)?;
                    self.linear(
                        branch,
                        Rule::Serial,
                        Vec::new(),
                        vec![Member::Edge(x, y)],
                        true,
                    );
                    self.made(branch, y);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Gives a new world its reflexive edge.
    fn made(&mut self, branch: &mut Branch, y: u32) {
        if self.reflexive {
            self.linear(
                branch,
                Rule::Reflexive,
                Vec::new(),
                vec![Member::Edge(y, y)],
                true,
            );
        }
    }

    /// Returns whether a `□` false or `◇` true at `world` has a witness: a
    /// world it sees where its operand is false or true.
    fn realised(&self, branch: &Branch, world: u32, id: NodeId, right: bool) -> bool {
        let (Node::Box(a) | Node::Diamond(a)) = self.sequent.formulas().node(id) else {
            return true;
        };
        branch.worlds[world as usize]
            .sees
            .iter()
            .any(|&y| Self::holds(branch, formula(y, a, right)))
    }

    /// Makes a world's signature, unless it has one: its formulas true
    /// and false, with every `□` true and `◇` false at a world that sees
    /// it. In a system that blocks it is asked only where nothing is left
    /// to do but make a world, when every world's formulas are final:
    /// formulas reach only the worlds made later, which see no older
    /// world.
    fn signature(
        &mut self,
        branch: &mut Branch,
        world: u32,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<(), Reason> {
        let w = &branch.worlds[world as usize];
        if w.signature.is_some() {
            return Ok(());
        }
        let words = self.words as u64;
        self.tick(words.saturating_mul(1 + w.seen_by.len() as u64), stop)?;
        self.account.charge(self.words.saturating_mul(16));
        let (mut truth, mut falsity) = (w.truth.clone(), w.falsity.clone());
        for &v in &w.seen_by {
            let v = &branch.worlds[v as usize];
            for i in 0..self.words {
                truth[i] |= v.truth[i] & self.boxes[i];
                falsity[i] |= v.falsity[i] & self.diamonds[i];
            }
        }
        branch.worlds[world as usize].signature = Some((truth, falsity));
        Ok(())
    }

    /// Returns the world that blocks `world`, in a system that blocks:
    /// the first earlier world that no world blocks and whose signature
    /// includes `world`'s. Each world's blocker depends on earlier worlds'
    /// alone and is kept once found, so every earlier world's is found
    /// first, in order.
    fn blocker(
        &mut self,
        branch: &mut Branch,
        world: u32,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<Option<u32>, Reason> {
        if !self.blocking {
            return Ok(None);
        }
        match branch.worlds[world as usize].blocked {
            Blocked::No => return Ok(None),
            Blocked::By(v) => return Ok(Some(v)),
            Blocked::Unknown => {}
        }
        for w in 0..=world {
            if branch.worlds[w as usize].blocked != Blocked::Unknown {
                continue;
            }
            self.signature(branch, w, stop)?;
            let mut found = None;
            for v in 0..w {
                if branch.worlds[v as usize].blocked != Blocked::No {
                    continue;
                }
                self.tick(self.words as u64, stop)?;
                let (Some((t, f)), Some((tw, fw))) = (
                    &branch.worlds[v as usize].signature,
                    &branch.worlds[w as usize].signature,
                ) else {
                    unreachable!("an earlier world's signature is made first")
                };
                if (0..self.words).all(|i| tw[i] & !t[i] == 0 && fw[i] & !f[i] == 0) {
                    found = Some(v);
                    break;
                }
            }
            branch.worlds[w as usize].blocked = found.map_or(Blocked::No, Blocked::By);
        }
        Ok(match branch.worlds[world as usize].blocked {
            Blocked::By(v) => Some(v),
            Blocked::No | Blocked::Unknown => None,
        })
    }

    /// Returns whether a world is blocked, counting the tests that say
    /// so.
    fn blocked(
        &mut self,
        branch: &mut Branch,
        world: u32,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<bool, Reason> {
        let blocked = self.blocker(branch, world, stop)?.is_some();
        if blocked {
            self.counts.blocked += 1;
        }
        Ok(blocked)
    }

    /// Returns an open branch's worlds as a model: its relation, a blocked
    /// world given its blocker's edges, closed under the system's frame
    /// conditions, and a world without edges in a serial system given an
    /// edge to itself.
    fn model(
        &mut self,
        branch: &mut Branch,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<Model, Reason> {
        let n = branch.worlds.len();
        let edges: usize = branch.worlds.iter().map(|w| w.sees.len()).sum();
        self.account.charge(edges.saturating_mul(8));
        self.tick(n as u64, stop)?;
        let mut sees: Vec<Vec<u32>> = branch.worlds.iter().map(|w| w.sees.clone()).collect();
        // In a serial system a world without arrows that no world blocks
        // sees itself: it has no `□` true or `◇` false of its own (else
        // `Ser` would have made it a world), so what is true there stays.
        if self.serial {
            for x in 0..n as u32 {
                if sees[x as usize].is_empty() && self.blocker(branch, x, stop)?.is_none() {
                    sees[x as usize].push(x);
                }
            }
        }
        // The branch's relation is closed under the system's frame rules
        // already. Only a blocked world's arrows from its blocker need
        // closing, and only under transitivity: the systems that block
        // are neither symmetric nor euclidean. Each new pair is charged
        // and acted on once.
        if self.blocking {
            self.account.charge(edges.saturating_mul(8));
            let mut seen_by: Vec<Vec<u32>> =
                branch.worlds.iter().map(|w| w.seen_by.clone()).collect();
            for (x, seen) in sees.iter().enumerate() {
                if seen.as_slice() == [x as u32] {
                    let into = &mut seen_by[x];
                    if let Err(at) = into.binary_search(&(x as u32)) {
                        into.insert(at, x as u32);
                    }
                }
            }
            let mut pending: Vec<(u32, u32)> = Vec::new();
            let add = |x: u32,
                       y: u32,
                       sees: &mut [Vec<u32>],
                       seen_by: &mut [Vec<u32>],
                       pending: &mut Vec<(u32, u32)>| {
                if let Err(at) = sees[x as usize].binary_search(&y) {
                    sees[x as usize].insert(at, y);
                    let into = &mut seen_by[y as usize];
                    let at = into.binary_search(&x).unwrap_or_else(|at| at);
                    into.insert(at, x);
                    pending.push((x, y));
                    true
                } else {
                    false
                }
            };
            for x in 0..n as u32 {
                let w = &branch.worlds[x as usize];
                let waits = branch
                    .demands
                    .iter()
                    .any(|&(d, id, right)| d == x && !self.realised(branch, d, id, right))
                    || (self.serial && w.sees.is_empty());
                self.owed = self
                    .owed
                    .saturating_add(branch.demands.len() as u64 + w.sees.len() as u64);
                if waits && let Some(a) = self.blocker(branch, x, stop)? {
                    // The blocker's arrows as the model has them: with its
                    // loop where it is a dead end of a serial system.
                    let more = sees[a as usize].clone();
                    self.tick(more.len() as u64, stop)?;
                    for c in more {
                        if add(x, c, &mut sees, &mut seen_by, &mut pending) {
                            self.account.charge(24);
                        }
                    }
                }
            }
            while let Some((x, y)) = pending.pop() {
                let (after, before) = (sees[y as usize].clone(), seen_by[x as usize].clone());
                self.tick(1 + (after.len() + before.len()) as u64, stop)?;
                for z in after {
                    if add(x, z, &mut sees, &mut seen_by, &mut pending) {
                        self.account.charge(24);
                    }
                }
                for w in before {
                    if add(w, y, &mut sees, &mut seen_by, &mut pending) {
                        self.account.charge(24);
                    }
                }
            }
        }
        let formulas = self.sequent.formulas();
        let worlds = branch
            .worlds
            .iter()
            .zip(sees)
            .map(|(w, arrows)| {
                let atoms = (0..formulas.len())
                    .filter(|&i| bit(&w.truth, i))
                    .filter_map(|i| match formulas.node(NodeId(i as u32)) {
                        Node::Atom(a) => Some(formulas.atom_name(a).to_owned()),
                        Node::True
                        | Node::False
                        | Node::Not(_)
                        | Node::And(..)
                        | Node::Or(..)
                        | Node::Implies(..)
                        | Node::Iff(..)
                        | Node::Box(_)
                        | Node::Diamond(_) => None,
                    })
                    .collect();
                ModelWorld::new(atoms, false, arrows)
            })
            .collect();
        Ok(Model::new(worlds, 0))
    }
}
