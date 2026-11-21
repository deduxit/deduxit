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
//! In K4, KD4 and S4 the worlds are made in depth: a new world acts
//! before the world that made it acts again. A world's part of the
//! branch (the world and the worlds made after it, until it is
//! finished) depends only on its key: the formula it was made for and
//! the `□` true and `◇` false at the worlds that see it. So the search
//! keeps a cache of keys across branches, as Goré and Nguyen's tableaux
//! with global caching do ("EXPTIME tableaux with global caching for
//! description logics with transitive roles, inverse roles and role
//! hierarchies", TABLEAUX 2007), with two sides. A key whose part was
//! finished with a model of its own (no world of it waits on a world
//! outside it) keeps that model, and a later world with the key is not
//! expanded: the countermodel takes the kept worlds, and the model check
//! decides it as any other. A key whose world closed its branch by a
//! proof that uses nothing outside the key closes the branch of a later
//! world with the key; such a branch has no derivation, so where the
//! proof rests on one the search runs again without those keys.
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
use crate::hash::{HashMap, HashSet};
use crate::kripke::{Model, World as ModelWorld};
use crate::limits::{Limits, Phase, Progress};
use crate::linear::search::Reason;
use crate::linear::search::memory::Account;
use std::collections::VecDeque;
use std::rc::Rc;

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
    /// For a step that makes a world in a system that blocks, the world's
    /// key ([`Search::keyed`]).
    key: Option<Rc<[u64]>>,
    /// Whether it closes a branch because the cache holds its last
    /// world's key as one no model satisfies ([`Search::refuted`]): no
    /// rule of the calculus, so a derivation never holds it.
    cached: bool,
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
    /// What decides its part of the branch, in a system that blocks: see
    /// [`Search::keyed`]. `None` for the first world and in other systems.
    key: Option<Rc<[u64]>>,
    /// The world that made it (the first world names itself).
    maker: u32,
    /// In a system that blocks, once it is finished, the end of its part
    /// of the branch: it is `self..end`.
    end: u32,
    /// The entry its own finished part was kept as, on this branch.
    kept: Option<u32>,
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
    /// A world of another branch with the same key had a finished part of
    /// its own that a model satisfies, kept as this entry of the cache:
    /// the world is not expanded, and the model takes the entry's worlds.
    Cached(u32),
}

/// A world's finished part of a branch, kept for every later world with
/// the same key: its worlds as a model, the first the world itself.
#[derive(Debug)]
struct Entry {
    /// The worlds; arrows are indices of this list.
    worlds: Vec<Piece>,
}

/// A world of an [`Entry`].
#[derive(Debug)]
struct Piece {
    /// The atoms true there, as indices of the arena.
    atoms: Vec<u32>,
    /// The worlds it sees, indices of the entry's worlds.
    arrows: Vec<u32>,
    /// The entry whose worlds stand in for this one, for a world that was
    /// itself taken from the cache.
    graft: Option<u32>,
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
    /// In a system that blocks, the worlds whose part of the branch is
    /// not finished yet, oldest first: each the one that made the next.
    open: Vec<u32>,
    /// A world just made, and the world that made it, whose key the cache
    /// holds as one no model satisfies: the branch closes.
    refuted: Option<(u32, u32)>,
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
    /// The worlds taken from the cache.
    pub(super) cached: u64,
    /// The branches closed by the cache.
    pub(super) refuted: u64,
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
    /// A bit per formula: whether it is an atom.
    atoms: Vec<u64>,
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
    /// In a system that blocks, the entry of each key whose world's part
    /// of a branch was finished with a model, on any branch.
    cache: HashMap<Rc<[u64]>, u32>,
    /// The entries of the cache.
    entries: Vec<Entry>,
    /// The bytes the entries hold.
    cache_bytes: usize,
    /// In a system that blocks, the keys whose world no model satisfies,
    /// found on any branch ([`Search::fold`]).
    unsat: HashSet<Rc<[u64]>>,
    /// Whether a world whose key is in `unsat` closes its branch: until a
    /// proof rests on one, after which the search runs again without.
    refute: bool,
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
        let mut atoms = vec![0u64; words];
        for i in 0..width {
            match sequent.formulas().node(NodeId(i as u32)) {
                Node::Box(_) => boxes[i / 64] |= 1 << (i % 64),
                Node::Diamond(_) => diamonds[i / 64] |= 1 << (i % 64),
                Node::Atom(_) => atoms[i / 64] |= 1 << (i % 64),
                Node::True
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
            atoms,
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
            cache: HashMap::default(),
            entries: Vec::new(),
            cache_bytes: 0,
            unsat: HashSet::default(),
            refute: has(Condition::Transitive)
                && !has(Condition::Symmetric)
                && !has(Condition::Euclidean),
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
        // Two bit sets, two more for a signature and two for a key.
        let world = self.words.saturating_mul(48).saturating_add(104);
        let edges: usize = branch.worlds.iter().map(|w| w.sees.len()).sum();
        branch
            .worlds
            .len()
            .saturating_mul(world)
            .saturating_add(edges.saturating_mul(8))
            .saturating_add(branch.agenda.len().saturating_mul(MEMBER_BYTES))
            .saturating_add((branch.betas.len() + branch.demands.len()).saturating_mul(12))
            .saturating_add((branch.segment.len() + branch.open.len()).saturating_mul(4))
    }

    /// Returns a `u32` index of a vector of this length, noting where it
    /// passes what a `u32` holds.
    fn index(&mut self, len: usize) -> u32 {
        u32::try_from(len).unwrap_or_else(|_| {
            self.full = true;
            u32::MAX
        })
    }

    /// Runs the search to its end. Where the proof found rests on a key
    /// the cache holds as one no model satisfies, it has no derivation
    /// there: the search runs again without those keys, and its proof is
    /// made of the calculus's rules alone. Both runs count their work.
    pub(super) fn run(&mut self, mut stop: impl FnMut(Progress) -> bool) -> Result<Found, Reason> {
        let stop = &mut stop;
        loop {
            match self.tableau(stop)? {
                Found::Closed(root) if self.refute && self.rests_on_cache(root, stop)? => {
                    self.steps = Vec::new();
                    self.proofs = Vec::new();
                    self.unsat = HashSet::default();
                    self.refute = false;
                    // What the first run held but the entries of the cache
                    // is given back.
                    self.account.rewind(self.cache_bytes as u64);
                }
                found @ (Found::Closed(_) | Found::Open(_)) => return Ok(found),
            }
        }
    }

    /// Returns whether the proof whose root node is `root` holds a step
    /// that closes by the cache.
    fn rests_on_cache(
        &mut self,
        root: u32,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<bool, Reason> {
        let mut stack = vec![root];
        let mut seen = 0u64;
        let mut found = false;
        while let Some(node) = stack.pop() {
            seen += 1;
            let proof = &self.proofs[node as usize];
            if self.steps[proof.step as usize].cached {
                found = true;
                break;
            }
            stack.extend_from_slice(&proof.premises);
        }
        self.tick(seen, stop)?;
        Ok(found)
    }

    /// Searches the tableau to its end.
    fn tableau(&mut self, stop: &mut impl FnMut(Progress) -> bool) -> Result<Found, Reason> {
        let mut branch = Branch {
            worlds: Vec::new(),
            agenda: VecDeque::new(),
            betas: Vec::new(),
            demands: Vec::new(),
            segment: Vec::new(),
            open: Vec::new(),
            refuted: None,
        };
        self.world(&mut branch)?;
        if self.blocking {
            branch.open.push(0);
        }
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
            if self.refute
                && let Some(key) = self.steps[step as usize].key.clone()
            {
                self.learn(step, key, &closed.1);
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

    /// Keeps the key of the world that `step` makes as one no model
    /// satisfies, where the proof above the step uses (`needed`) only the
    /// world's own formula, the edge to it, edges between older worlds,
    /// and `□` true and `◇` false at older worlds that the key holds.
    ///
    /// Why that is enough: older worlds get no members after the step
    /// (formulas reach only the worlds made later), so the proof uses them
    /// only to send a `□` true or `◇` false along an edge into a new world,
    /// or in `Trans` towards one. Read every older world as one label `o`:
    /// the proof stays one (a `Trans` whose premise is now its conclusion
    /// drops out), of `o R y`, `y`'s own formula, and those `□` and `◇` at
    /// `o`. A world `y'` of another branch with the same key, true at a
    /// point `m` of a model, would satisfy that: `o` is `m` itself in S4,
    /// and in K4 and KD4 a new point that sees `m` and every point `m`
    /// sees, where every such `□B` has `B` true since a world that sees
    /// `y'` holds it. The run that ends with a proof resting on such a
    /// key runs again without them ([`Search::run`]), so no verdict rests
    /// on this argument alone.
    fn learn(&mut self, step: u32, key: Rc<[u64]>, needed: &HashSet<Member>) {
        let words = self.words;
        let Some(Member::Edge(x, y)) = self.steps[step as usize].located else {
            return;
        };
        let own = key[2 * words];
        self.owed = self.owed.saturating_add(needed.len() as u64);
        let holds = |set: &[u64], i: u32| bit(set, i as usize);
        let fits = needed.iter().all(|&m| match m {
            Member::Formula { world, node, right } if world == y => {
                own != 0 && own == (u64::from(node) + 1) << 1 | u64::from(right)
            }
            Member::Formula { world, node, right } => {
                world < y
                    && if right {
                        holds(&self.diamonds, node) && holds(&key[words..], node)
                    } else {
                        holds(&self.boxes, node) && holds(&key[..words], node)
                    }
            }
            Member::Edge(a, b) => (a, b) == (x, y) || (a < y && b < y),
        });
        if fits && !self.unsat.contains(&key) {
            let bytes = key.len() * 8 + 48;
            if self.account.spares(bytes) {
                self.account.charge(bytes);
                self.unsat.insert(key);
            }
        }
    }

    /// Returns the step that closes a branch whose last world `y`, made by
    /// `x`, has a key no model satisfies. It writes out what a proof of
    /// that would use at most: the edge `x R y`, `y`'s own formula, and at
    /// `x` and the worlds that see it their `□` true, `◇` false and the
    /// edges between them.
    fn refuted(&mut self, branch: &Branch, x: u32, y: u32) -> u32 {
        let words = self.words;
        let mut written = vec![Member::Edge(x, y)];
        let own = branch.worlds[y as usize]
            .key
            .as_ref()
            .map_or(0, |key| key[2 * words]);
        if own != 0 {
            written.push(Member::Formula {
                world: y,
                node: (own >> 1) as u32 - 1,
                right: own & 1 == 1,
            });
        }
        let from = &branch.worlds[x as usize];
        let mut older: Vec<u32> = from.seen_by.clone();
        if let Err(at) = older.binary_search(&x) {
            older.insert(at, x);
        }
        for &w in &older {
            let world = &branch.worlds[w as usize];
            self.owed = self.owed.saturating_add((words + world.sees.len()) as u64);
            for i in 0..words {
                let mut boxes = world.truth[i] & self.boxes[i];
                while boxes != 0 {
                    let node = (i * 64) as u32 + boxes.trailing_zeros();
                    boxes &= boxes - 1;
                    written.push(Member::Formula {
                        world: w,
                        node,
                        right: false,
                    });
                }
                let mut diamonds = world.falsity[i] & self.diamonds[i];
                while diamonds != 0 {
                    let node = (i * 64) as u32 + diamonds.trailing_zeros();
                    diamonds &= diamonds - 1;
                    written.push(Member::Formula {
                        world: w,
                        node,
                        right: true,
                    });
                }
            }
            for &v in &world.sees {
                if older.binary_search(&v).is_ok() {
                    written.push(Member::Edge(w, v));
                }
            }
        }
        self.counts.refuted += 1;
        self.record(Step {
            rule: Rule::Axiom,
            written,
            premises: Vec::new(),
            located: None,
            new: Vec::new(),
            key: None,
            cached: true,
        })
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

    /// Returns whether a world was taken from the cache.
    fn cached(branch: &Branch, world: u32) -> bool {
        matches!(branch.worlds[world as usize].blocked, Blocked::Cached(_))
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
            key: None,
            cached: false,
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
            key: None,
            maker: index,
            end: 0,
            kept: None,
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
            if let Some((x, y)) = branch.refuted.take() {
                return Ok(Saturated::Closed(self.refuted(branch, x, y)));
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
                    // A world taken from the cache is not expanded: its
                    // entry's model stands for it.
                    (Node::And(..), true)
                    | (Node::Or(..), false)
                    | (Node::Implies(..), false)
                    | (Node::Iff(..), _) => {
                        if !Self::cached(branch, world) {
                            branch.betas.push((world, id, right));
                        }
                    }
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
                        if !Self::cached(branch, world) {
                            branch.demands.push((world, id, right));
                        }
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
            key: None,
            cached: false,
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
            key: None,
            cached: false,
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
    /// witness at a world that is not blocked, the oldest world first (the
    /// newest in a system that blocks, [`Search::deepest`]), or by `Ser`;
    /// returns whether it made one.
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
        if self.blocking {
            return self.deepest(branch, stop);
        }
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

    /// In a system that blocks, makes the next world of a walk in depth
    /// over the worlds, or returns that none is left to make. The world
    /// made last that is not finished makes a world for its first `□`
    /// false or `◇` true without a witness, or by `Ser` where it applies,
    /// unless it is blocked, and the new world is the next to act; a world
    /// that makes none is finished, and so is its part of the branch: it
    /// and the worlds made after it, which the cache keeps where it is a
    /// model of its own ([`Search::keep`]). A new world is taken from the
    /// cache where its key has an entry ([`Search::keyed`]).
    fn deepest(
        &mut self,
        branch: &mut Branch,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<bool, Reason> {
        while let Some(&x) = branch.open.last() {
            let demand = branch.demands.iter().position(|&(w, ..)| w == x);
            self.owed = self.owed.saturating_add(branch.demands.len() as u64);
            let w = &branch.worlds[x as usize];
            let serial = demand.is_none()
                && self.serial
                && !Self::cached(branch, x)
                && w.sees.is_empty()
                && (0..self.words).any(|i| {
                    w.truth[i] & self.boxes[i] != 0 || w.falsity[i] & self.diamonds[i] != 0
                });
            if (demand.is_some() || serial) && !self.blocked(branch, x, stop)? {
                let y = self.make(branch, x, demand)?;
                branch.open.push(y);
                return Ok(true);
            }
            branch.open.pop();
            branch.worlds[x as usize].end = branch.worlds.len() as u32;
            self.keep(branch, x);
            // Each finished part was scanned: count it now, not all at
            // the end of the walk.
            self.tick(0, stop)?;
        }
        Ok(false)
    }

    /// Makes a world from `x`, for the demand of that index or by `Ser`,
    /// and returns it.
    fn make(&mut self, branch: &mut Branch, x: u32, demand: Option<usize>) -> Result<u32, Reason> {
        let y = self.world(branch)?;
        branch.worlds[y as usize].maker = x;
        match demand {
            Some(index) => {
                let (x, id, right) = branch.demands.remove(index);
                let (Node::Box(a) | Node::Diamond(a)) = self.sequent.formulas().node(id) else {
                    unreachable!("a demand is a modal formula")
                };
                let key = self.keyed(branch, x, y, (u64::from(a.0) + 1) << 1 | u64::from(right));
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
                if let Some(step) = self.steps.last_mut() {
                    step.key = Some(key);
                }
            }
            None => {
                let key = self.keyed(branch, x, y, 0);
                self.linear(
                    branch,
                    Rule::Serial,
                    Vec::new(),
                    vec![Member::Edge(x, y)],
                    true,
                );
                if let Some(step) = self.steps.last_mut() {
                    step.key = Some(key);
                }
            }
        }
        self.made(branch, y);
        Ok(y)
    }

    /// Gives the world `y` that `x` makes its key, and takes it from the
    /// cache where the key has an entry. The key is what decides the part
    /// of the branch the new world starts, in a system that blocks: the
    /// formula it is made for (`own`: the operand's index and its side,
    /// or 0 for `Ser`), and every `□` true and `◇` false at a world that
    /// will see it, `x` and the worlds that see `x`. Formulas reach only
    /// the worlds made later, so nothing else reaches that part, and two
    /// worlds with one key start the same search.
    fn keyed(&mut self, branch: &mut Branch, x: u32, y: u32, own: u64) -> Rc<[u64]> {
        let words = self.words;
        let mut key = vec![0u64; 2 * words + 1];
        key[2 * words] = own;
        let from = &branch.worlds[x as usize];
        self.owed = self
            .owed
            .saturating_add((words * (2 + from.seen_by.len())) as u64);
        for v in std::iter::once(x).chain(from.seen_by.iter().copied()) {
            let v = &branch.worlds[v as usize];
            for i in 0..words {
                key[i] |= v.truth[i] & self.boxes[i];
                key[words + i] |= v.falsity[i] & self.diamonds[i];
            }
        }
        let key: Rc<[u64]> = key.into();
        self.account.charge(16 * words + 24);
        let world = &mut branch.worlds[y as usize];
        if let Some(&entry) = self.cache.get(&key) {
            world.blocked = Blocked::Cached(entry);
            self.counts.cached += 1;
        } else if self.refute && self.unsat.contains(&key) {
            branch.refuted = Some((x, y));
        }
        world.key = Some(key.clone());
        key
    }

    /// Keeps the part of the branch that the world `k` started, now
    /// finished, as the entry of `k`'s key, unless the key has one or the
    /// part is no model of its own: a world of it that waits (a demand
    /// without a witness, or a dead end where `Ser` applies) must be
    /// blocked by a world of the part, whose arrows it then takes, so
    /// that the part's model needs no world outside it. A dead end where
    /// `Ser` does not apply sees itself, as in [`Search::model`]. The
    /// entry holds each world's direct arrows only (to the worlds it made,
    /// and the loops of S4 and of a dead end): the model closes them under
    /// transitivity, which gives the branch's relation. A world of the
    /// part whose own part was kept already stands in it as that entry,
    /// so a world is copied into one entry at most along a branch. The
    /// entry is taken only where the memory bound spares it, as the linear
    /// search's memo is.
    fn keep(&mut self, branch: &mut Branch, k: u32) {
        let Some(key) = branch.worlds[k as usize].key.clone() else {
            return;
        };
        if Self::cached(branch, k) || self.cache.contains_key(&key) {
            return;
        }
        let (start, end) = (k as usize, branch.worlds.len());
        // Each world's place in the entry, `u32::MAX` for a world inside
        // a part that stands as its entry.
        let mut place = vec![u32::MAX; end - start];
        let mut pieces: Vec<Piece> = Vec::new();
        let mut worlds: Vec<u32> = Vec::new();
        let mut d = start;
        while d < end {
            let w = &branch.worlds[d];
            place[d - start] = pieces.len() as u32;
            worlds.push(d as u32);
            let graft = match (w.blocked, w.kept) {
                (Blocked::Cached(entry), _) => Some(entry),
                (_, Some(entry)) if d != start => Some(entry),
                _ => None,
            };
            let atoms: Vec<u32> = if graft.is_some() {
                Vec::new()
            } else {
                self.owed = self.owed.saturating_add(self.words as u64);
                let mut atoms: Vec<u32> = (0..self.words)
                    .flat_map(|j| {
                        let mut bits = w.truth[j] & self.atoms[j];
                        std::iter::from_fn(move || {
                            (bits != 0).then(|| {
                                let b = bits.trailing_zeros();
                                bits &= bits - 1;
                                (j * 64) as u32 + b
                            })
                        })
                    })
                    .collect();
                atoms.shrink_to_fit();
                atoms
            };
            pieces.push(Piece {
                atoms,
                arrows: Vec::new(),
                graft,
            });
            // A kept part stands as its entry: skip its other worlds.
            d = match (w.kept, d != start) {
                (Some(_), true) => w.end as usize,
                _ => d + 1,
            };
        }
        self.owed = self
            .owed
            .saturating_add((branch.demands.len() + pieces.len()) as u64);
        let at = |y: u32| -> Option<u32> {
            let i = y.checked_sub(k)? as usize;
            place.get(i).copied().filter(|&p| p != u32::MAX)
        };
        // The direct arrows: from each world's maker, and the loops.
        for (i, &y) in worlds.iter().enumerate() {
            let w = &branch.worlds[y as usize];
            if y != k {
                let Some(m) = at(w.maker) else {
                    return;
                };
                pieces[m as usize].arrows.push(i as u32);
            }
            if pieces[i].graft.is_none()
                && (self.reflexive || (self.serial && w.sees.is_empty()))
            {
                pieces[i].arrows.push(i as u32);
            }
        }
        // A world that waits takes its blocker's arrows; every demand
        // left has no witness.
        let mut waits = vec![false; pieces.len()];
        for &(d, ..) in &branch.demands {
            if let Some(i) = at(d) {
                waits[i as usize] = true;
            }
        }
        for (i, &y) in worlds.iter().enumerate() {
            let w = &branch.worlds[y as usize];
            if pieces[i].graft.is_some() {
                continue;
            }
            let own = (0..self.words)
                .any(|j| w.truth[j] & self.boxes[j] != 0 || w.falsity[j] & self.diamonds[j] != 0);
            let dead = self.serial && w.sees.is_empty();
            if waits[i] || (dead && own) {
                let Blocked::By(v) = w.blocked else {
                    return;
                };
                let Some(v) = at(v) else {
                    return;
                };
                if pieces[v as usize].graft.is_some() {
                    return;
                }
                // The blocker does not wait, so its arrows are final; the
                // waiting world's loop as a dead end goes (not its loop
                // in S4).
                let more = pieces[v as usize].arrows.clone();
                let arrows = &mut pieces[i].arrows;
                if !self.reflexive {
                    arrows.retain(|&a| a != i as u32);
                }
                for c in more {
                    if !arrows.contains(&c) {
                        arrows.push(c);
                    }
                }
                self.owed = self.owed.saturating_add(arrows.len() as u64);
            }
        }
        let mut bytes = key.len() * 16 + 64;
        for piece in &mut pieces {
            piece.arrows.shrink_to_fit();
            bytes = bytes.saturating_add(
                std::mem::size_of::<Piece>()
                    + 4 * (piece.atoms.capacity() + piece.arrows.capacity())
                    + 32,
            );
        }
        if !self.account.spares(bytes) {
            return;
        }
        self.account.charge(bytes);
        self.cache_bytes = self.cache_bytes.saturating_add(bytes);
        let entry = self.index(self.entries.len());
        self.entries.push(Entry { worlds: pieces });
        self.cache.insert(key, entry);
        branch.worlds[k as usize].kept = Some(entry);
    }

    /// Returns the bytes a world of an entry takes in a model: its arrows,
    /// and its atoms as indices and as the names a model's world holds.
    fn model_bytes(piece: &Piece) -> usize {
        48 * piece.atoms.len() + 4 * piece.arrows.len() + 64
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
            Blocked::No | Blocked::Cached(_) => return Ok(None),
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
            Blocked::No | Blocked::Unknown | Blocked::Cached(_) => None,
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

    /// Returns an open branch's worlds as a model: its relation, a world
    /// taken from the cache given its entry's worlds, a blocked world
    /// given its blocker's edges, closed under the system's frame
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
        // The atoms true at each world, by index of the arena: the
        // branch's, and an entry's for a world taken from the cache and
        // for the worlds its entry adds.
        let mut atoms: Vec<Vec<u32>> = Vec::with_capacity(n);
        for w in &branch.worlds {
            self.owed = self.owed.saturating_add(self.words as u64);
            atoms.push(
                (0..self.words)
                    .flat_map(|j| {
                        let mut bits = w.truth[j] & self.atoms[j];
                        std::iter::from_fn(move || {
                            (bits != 0).then(|| {
                                let b = bits.trailing_zeros();
                                bits &= bits - 1;
                                (j * 64) as u32 + b
                            })
                        })
                    })
                    .collect(),
            );
        }
        // In a serial system a world without arrows that no world blocks
        // sees itself: it has no `□` true or `◇` false of its own (else
        // `Ser` would have made it a world), so what is true there stays.
        // A world taken from the cache has its entry's arrows instead.
        if self.serial {
            for x in 0..n as u32 {
                if sees[x as usize].is_empty()
                    && !Self::cached(branch, x)
                    && self.blocker(branch, x, stop)?.is_none()
                {
                    sees[x as usize].push(x);
                }
            }
        }
        // The branch's relation is closed under the system's frame rules
        // already. Only a blocked world's arrows from its blocker, and the
        // arrows of the worlds an entry adds, need closing, and only under
        // transitivity: the systems that block are neither symmetric nor
        // euclidean. Each new pair is charged and acted on once.
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
            // Each world taken from the cache becomes its entry's first
            // world, and the entry's other worlds are added once, an entry
            // within an entry in turn: a second world with the entry sees
            // the same worlds, which satisfy what both need, since what
            // reaches them from the worlds that see either is the key's.
            let mut grafts: Vec<(u32, u32)> = (0..n as u32)
                .filter_map(|x| match branch.worlds[x as usize].blocked {
                    Blocked::Cached(entry) => Some((x, entry)),
                    Blocked::Unknown | Blocked::No | Blocked::By(_) => None,
                })
                .collect();
            let mut added: HashMap<u32, usize> = HashMap::default();
            while let Some((root, entry)) = grafts.pop() {
                if let Some(&base) = added.get(&entry) {
                    let first = &self.entries[entry as usize].worlds[0];
                    let arrows = first.arrows.clone();
                    let (count, held) = (first.atoms.len(), Self::model_bytes(first));
                    self.account.charge(held);
                    self.tick((1 + arrows.len() + count) as u64, stop)?;
                    atoms[root as usize].clone_from(&self.entries[entry as usize].worlds[0].atoms);
                    for y in arrows {
                        let y = if y == 0 {
                            root
                        } else {
                            (base + y as usize - 1) as u32
                        };
                        if add(root, y, &mut sees, &mut seen_by, &mut pending) {
                            self.account.charge(24);
                        }
                    }
                    continue;
                }
                added.insert(entry, sees.len());
                let (count, held, cells) = {
                    let pieces = &self.entries[entry as usize].worlds;
                    let held: usize = pieces.iter().map(Self::model_bytes).sum();
                    let cells: usize = pieces.iter().map(|p| p.atoms.len() + p.arrows.len()).sum();
                    (pieces.len(), held, cells)
                };
                self.account.charge(held);
                self.tick((count + cells) as u64, stop)?;
                let pieces = &self.entries[entry as usize].worlds;
                let base = sees.len();
                let at = |i: u32| {
                    if i == 0 {
                        root
                    } else {
                        (base + i as usize - 1) as u32
                    }
                };
                for _ in 1..count {
                    sees.push(Vec::new());
                    seen_by.push(Vec::new());
                    atoms.push(Vec::new());
                }
                for (i, piece) in pieces.iter().enumerate() {
                    let x = at(i as u32);
                    atoms[x as usize].clone_from(&piece.atoms);
                    if let Some(inner) = piece.graft {
                        grafts.push((x, inner));
                    }
                    self.owed = self.owed.saturating_add(piece.arrows.len() as u64);
                    for &y in &piece.arrows {
                        if add(x, at(y), &mut sees, &mut seen_by, &mut pending) {
                            self.account.charge(24);
                        }
                    }
                }
            }
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
        let worlds = atoms
            .into_iter()
            .zip(sees)
            .map(|(atoms, arrows)| {
                let atoms = atoms
                    .into_iter()
                    .filter_map(|i| match formulas.node(NodeId(i)) {
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
