// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Intuitionistic and minimal logic decided natively, by Dyckhoff's
//! contraction-free sequent calculus LJT, also called G4ip (R. Dyckhoff,
//! *Contraction-free sequent calculi for intuitionistic logic*, JSL 57(3),
//! 1992; its correction, JSL 83(4), 2018), whose proof search terminates
//! by the calculus: every premise is lighter than its conclusion in the
//! multiset ordering of formula weights, so no branch is infinite and no
//! loop check or copy bound is needed. The search may still take
//! exponential time, and stops at the work bound, the memory bound or the
//! caller's stop.
//!
//! A proof is translated into a derivation of LJ that
//! [`Derivation::check`] accepts before it is returned: the rules of LJT
//! that LJ has are its own, and each other left rule on an implication
//! becomes a derivation of LJ with a cut on the formula the rule makes,
//! whose first premise derives that formula from the implication taken
//! apart: the soundness half of the equivalence of G4ip and G3ip, which
//! the correction calls straightforward "using admissibility of Cut in
//! G3ip". Minimal logic is the same search with `⊥` an atom, whose left
//! rule it lacks. "Not valid" is the search's word: it was exhaustive.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::Limits;
//! use deduxit::ordinary::{self, Logic, Verdict, ljt};
//!
//! let sequent: ordinary::Sequent = "~~(a \\/ ~a)".parse()?;
//! let outcome = ljt::decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| false)?;
//! assert!(matches!(outcome.verdict, Verdict::Valid(_)));
//! let sequent: ordinary::Sequent = "a \\/ ~a".parse()?;
//! let outcome = ljt::decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| false)?;
//! assert!(matches!(outcome.verdict, Verdict::NotValid));
//! # Ok::<(), deduxit::Error>(())
//! ```

use super::{Derivation, Formulas, Inference, Logic, Node, NodeId, Rule, Sequent, Side, Verdict};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal, Space};
use crate::linear::proofs::InfId;
use crate::linear::search::Reason;
use crate::linear::search::memory::Account;

/// What deciding a sequent by LJT found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict, with the derivation of LJ of a valid sequent.
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// What the search cost.
    pub statistics: Statistics,
}

/// What a search by LJT cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Statistics {
    /// The sequents expanded, the search's unit of work.
    pub sequents: u64,
    /// The sequents whose verdict was found in the memo.
    pub memo_hits: u64,
    /// The sequents the memo holds at the end.
    pub memo_entries: u64,
}

/// How many sequents the search expands between two polls of its stop.
pub const POLL: u64 = 1 << 8;

/// Decides an intuitionistic or minimal sequent by LJT within `limits`,
/// asking `stop` every [`POLL`] sequents: the search counts a sequent
/// expanded as a unit of `limits.work` and charges what it holds to
/// `limits.memory_bytes`; a proof is translated into a derivation of LJ
/// within `limits.derivation_bytes` and `limits.memory_bytes`, asking
/// `stop` once per inference, and checked. A sequent with nothing right of
/// `⊢` is decided as `Γ ⊢ ⊥`, and its derivation concludes that sequent.
///
/// # Errors
///
/// [`Error::Logic`] for classical logic; [`Error::Succedents`] for more
/// than one formula right of `⊢`; [`Refusal::Output`],
/// [`Refusal::Memory`] or [`Refusal::Stopped`] where a bound or the stop
/// refused the derivation of a proof, which is no verdict; as
/// [`Derivation::check`], [`Error::ReadBack`] being a defect of the
/// translation.
pub fn decide(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    run(sequent, logic, limits, stop, None)
}

/// [`decide`], with a fault planted in the translation, which only the
/// tests plant.
fn run(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
    plant: Option<Plant>,
) -> Result<Outcome, Error> {
    if logic == Logic::Classical {
        return Err(Error::Logic { logic });
    }
    if sequent.right().len() > 1 {
        return Err(Error::Succedents {
            count: sequent.right().len(),
        });
    }
    let mut formulas = sequent.formulas().clone();
    let falsity = formulas.add(Node::False)?;
    let goal = sequent.right().first().copied().unwrap_or(falsity);
    let mut search = Search {
        formulas,
        falsity,
        logic,
        account: Account::holding(limits.memory_bytes, 0),
        memo: HashMap::default(),
        steps: Vec::new(),
        statistics: Statistics::default(),
        work: limits.work.unwrap_or(u64::MAX),
    };
    let found = search.prove((set(sequent.left()), goal), &mut stop);
    search.statistics.memo_entries = search.memo.len() as u64;
    let statistics = search.statistics;
    let verdict = match found {
        Err(reason) => Verdict::Unknown(reason),
        Ok(Known::Failed) => Verdict::NotValid,
        Ok(Known::Proved(root)) => {
            let mut lj = Lj::new(&search, limits, &mut stop);
            lj.plant = plant;
            let derivation = lj.translate(root, sequent.left(), goal)?;
            derivation.check(limits, &mut stop)?;
            Verdict::Valid(Box::new(derivation))
        }
    };
    Ok(Outcome {
        verdict,
        logic,
        statistics,
    })
}

/// Returns the hypotheses as a set: sorted, each once. Contraction is
/// admissible in G4ip (Dyckhoff and Negri, JSL 65(4), 2000), so a sequent
/// is provable exactly when its set is.
fn set(left: &[NodeId]) -> Box<[NodeId]> {
    let mut left = left.to_vec();
    left.sort_unstable();
    left.dedup();
    left.into_boxed_slice()
}

/// A sequent of the search: the hypotheses as a set, and the goal.
type Key = (Box<[NodeId]>, NodeId);

/// A rule of the search, each one of LJT's or a rule LJ derives at once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// The goal is a hypothesis.
    Axiom,
    /// `⊥` is a hypothesis, outside minimal logic.
    FalseLeft,
    /// The goal is `⊤`.
    TrueRight,
    /// `A ∧ B` taken apart.
    AndLeft,
    /// `A ↔ B` taken apart into `A → B` and `B → A`.
    IffLeft,
    /// `A → B` or `¬A` as the goal: `A` assumed.
    ImpliesRight,
    /// `A ∧ B` as the goal.
    AndRight,
    /// `A ↔ B` as the goal: both implications.
    IffRight,
    /// `A ∨ B` taken apart, a premise for each.
    OrLeft,
    /// `A ∨ B` as the goal by `A`.
    OrRight1,
    /// `A ∨ B` as the goal by `B`.
    OrRight2,
    /// `A → B` with `A` a hypothesis gives `B`: LJT's rule for an atom,
    /// taken for any `A`, which LJ derives with the axiom `A ⊢ A`.
    ModusPonens,
    /// `⊤ → C` gives `C`.
    TrueImplies,
    /// `(A ∧ B) → C` gives `A → (B → C)`.
    AndImplies,
    /// `(A ↔ B) → C` gives `(A → B) → ((B → A) → C)`, the rule of `∧`.
    IffImplies,
    /// `(A ∨ B) → C` gives `A → C` and `B → C`.
    OrImplies,
    /// `(A → B) → C` with premises `B → C ⊢ A → B` and `C ⊢ G`.
    ImpliesImplies,
}

/// A rule applied to a principal formula.
#[derive(Clone, Copy, Debug)]
struct Move {
    /// The rule.
    kind: Kind,
    /// The principal formula: a hypothesis, or the goal.
    principal: NodeId,
}

/// A step of a proof found: the sequent, the rule and the proofs of its
/// premises, which precede it.
#[derive(Debug)]
struct Step {
    /// The sequent proved.
    key: Key,
    /// The rule.
    applied: Move,
    /// The premises' steps, as many as the rule has.
    premises: Vec<u32>,
}

/// What the search knows of a sequent.
#[derive(Clone, Copy, Debug)]
enum Known {
    /// Proved by the step given.
    Proved(u32),
    /// Unprovable.
    Failed,
}

/// A sequent being searched: its alternatives, the one tried and its
/// premises proved so far.
#[derive(Debug)]
struct Frame {
    /// The sequent.
    key: Key,
    /// The rules to try in turn: one, where an invertible rule applies.
    moves: Vec<Move>,
    /// The rule tried.
    at: usize,
    /// Its premises, once made.
    premises: Option<Vec<Key>>,
    /// The steps proving the premises proved so far.
    proved: Vec<u32>,
}

/// The state of one search.
struct Search {
    /// The formulas, with those the rules make added.
    formulas: Formulas,
    /// `⊥`, the goal of a sequent with nothing right of `⊢`.
    falsity: NodeId,
    /// Intuitionistic or minimal logic.
    logic: Logic,
    /// Who is charged what the search holds.
    account: Account,
    /// The verdict of every sequent decided.
    memo: HashMap<Key, Known>,
    /// The steps of the proofs found, premises first.
    steps: Vec<Step>,
    /// The costs.
    statistics: Statistics,
    /// The most sequents to expand.
    work: u64,
}

/// The bytes a sequent of `len` hypotheses is charged at where it is
/// kept: its slice and the key, twice over for a table's spare room.
const fn bytes(len: usize) -> usize {
    2 * (size_of::<Key>() + size_of::<Known>() + len * size_of::<NodeId>())
}

/// The bytes a frame is charged at while it is open.
const fn frame_bytes(frame: &Frame) -> usize {
    size_of::<Frame>() + bytes(frame.key.0.len()) + frame.moves.len() * size_of::<Move>()
}

impl Search {
    /// Returns `A → C`, written `¬A` where `C` is `⊥`, adding it.
    fn implication(&mut self, a: NodeId, c: NodeId) -> Result<NodeId, Reason> {
        let node = if c == self.falsity {
            Node::Not(a)
        } else {
            Node::Implies(a, c)
        };
        self.add(node)
    }

    /// Returns `A → C`, written `¬A` where `C` is `⊥`, which the search
    /// added.
    fn implication_of(&self, a: NodeId, c: NodeId) -> NodeId {
        let node = if c == self.falsity {
            Node::Not(a)
        } else {
            Node::Implies(a, c)
        };
        self.formulas
            .find(node)
            .unwrap_or_else(|| unreachable!("the search added it"))
    }

    /// Adds a node to the formulas, charging it.
    fn add(&mut self, node: Node) -> Result<NodeId, Reason> {
        let before = self.formulas.len();
        let id = self.formulas.add(node).map_err(|_| Reason::IndexLimit)?;
        if self.formulas.len() > before {
            // The node, its entry in the table of ids, spare room.
            self.account.charge(4 * size_of::<Node>());
        }
        Ok(id)
    }

    /// Returns `(A, C)` for `A → C` and for `¬A`, `C` being `⊥`.
    fn split(&self, id: NodeId) -> Option<(NodeId, NodeId)> {
        match self.formulas.node(id) {
            Node::Implies(a, c) => Some((a, c)),
            Node::Not(a) => Some((a, self.falsity)),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::And(..)
            | Node::Or(..)
            | Node::Iff(..) => None,
        }
    }

    /// Returns the rules to try on a sequent: a leaf rule or an invertible
    /// one alone where one applies, else every rule that does, each an
    /// alternative.
    fn moves(&self, (left, goal): (&[NodeId], NodeId)) -> Vec<Move> {
        let one = |kind, principal| vec![Move { kind, principal }];
        let f = &self.formulas;
        let has = |x: NodeId| left.binary_search(&x).is_ok();
        if has(goal) {
            return one(Kind::Axiom, goal);
        }
        if self.logic == Logic::Intuitionistic && has(self.falsity) {
            return one(Kind::FalseLeft, self.falsity);
        }
        let node = f.node(goal);
        match node {
            Node::True => return one(Kind::TrueRight, goal),
            Node::Implies(..) | Node::Not(_) => return one(Kind::ImpliesRight, goal),
            Node::Atom(_) | Node::False | Node::And(..) | Node::Or(..) | Node::Iff(..) => {}
        }
        // Invertible left rules with one premise.
        for &h in left {
            let kind = match f.node(h) {
                Node::And(..) => Some(Kind::AndLeft),
                Node::Iff(..) => Some(Kind::IffLeft),
                Node::Atom(_) | Node::True | Node::False | Node::Or(..) => None,
                Node::Implies(a, _) | Node::Not(a) => match f.node(a) {
                    _ if has(a) => Some(Kind::ModusPonens),
                    Node::True => Some(Kind::TrueImplies),
                    Node::And(..) => Some(Kind::AndImplies),
                    Node::Iff(..) => Some(Kind::IffImplies),
                    Node::Or(..) => Some(Kind::OrImplies),
                    Node::Atom(_) | Node::False | Node::Implies(..) | Node::Not(_) => None,
                },
            };
            if let Some(kind) = kind {
                return one(kind, h);
            }
        }
        // Invertible rules with two premises.
        match node {
            Node::And(..) => return one(Kind::AndRight, goal),
            Node::Iff(..) => return one(Kind::IffRight, goal),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::Or(..)
            | Node::Implies(..) => {}
        }
        if let Some(&h) = left.iter().find(|&&h| matches!(f.node(h), Node::Or(..))) {
            return one(Kind::OrLeft, h);
        }
        // The rules that are not invertible, each an alternative.
        let mut moves = Vec::new();
        if let Node::Or(..) = node {
            for kind in [Kind::OrRight1, Kind::OrRight2] {
                moves.push(Move {
                    kind,
                    principal: goal,
                });
            }
        }
        for &h in left {
            if let Some((a, _)) = self.split(h)
                && self.split(a).is_some()
            {
                moves.push(Move {
                    kind: Kind::ImpliesImplies,
                    principal: h,
                });
            }
        }
        moves
    }

    /// Returns the premises of a rule applied to a sequent, adding the
    /// formulas they need.
    fn premises(&mut self, (left, goal): (&[NodeId], NodeId), m: Move) -> Result<Vec<Key>, Reason> {
        let p = m.principal;
        // The hypotheses without the principal formula, with others added.
        let without = |added: &[NodeId]| {
            let mut list: Vec<NodeId> = left.iter().copied().filter(|&x| x != p).collect();
            list.extend_from_slice(added);
            set(&list)
        };
        let (a, b) = match self.formulas.node(p).operands() {
            (Some(a), Some(b)) => (a, b),
            (Some(a), None) => (a, self.falsity),
            (None, _) => (p, p),
        };
        Ok(match m.kind {
            Kind::Axiom | Kind::FalseLeft | Kind::TrueRight => Vec::new(),
            Kind::AndLeft => vec![(without(&[a, b]), goal)],
            Kind::IffLeft => {
                let (x, y) = (
                    self.add(Node::Implies(a, b))?,
                    self.add(Node::Implies(b, a))?,
                );
                vec![(without(&[x, y]), goal)]
            }
            Kind::ImpliesRight => {
                let mut list = left.to_vec();
                list.push(a);
                vec![(set(&list), b)]
            }
            Kind::AndRight => vec![(left.into(), a), (left.into(), b)],
            Kind::IffRight => {
                let (x, y) = (
                    self.add(Node::Implies(a, b))?,
                    self.add(Node::Implies(b, a))?,
                );
                vec![(left.into(), x), (left.into(), y)]
            }
            Kind::OrLeft => vec![(without(&[a]), goal), (without(&[b]), goal)],
            Kind::OrRight1 => vec![(left.into(), a)],
            Kind::OrRight2 => vec![(left.into(), b)],
            Kind::ModusPonens | Kind::TrueImplies => vec![(without(&[b]), goal)],
            Kind::AndImplies | Kind::IffImplies | Kind::OrImplies => {
                vec![(without(&self.made(m.kind, a, b)?), goal)]
            }
            Kind::ImpliesImplies => {
                let made = self.made(m.kind, a, b)?;
                vec![(without(&made), a), (without(&[b]), goal)]
            }
        })
    }

    /// Returns what a left rule on `antecedent → c` makes: `A → (B → C)`
    /// for `∧` (and `↔`, as the conjunction of its implications), `A → C`
    /// and `B → C` for `∨`, `B → C` for `A → B` (or `¬A`, `B` being `⊥`).
    fn made(&mut self, kind: Kind, antecedent: NodeId, c: NodeId) -> Result<Vec<NodeId>, Reason> {
        let (x, y) = match self.formulas.node(antecedent).operands() {
            (Some(x), Some(y)) => (x, y),
            (Some(x), None) => (x, self.falsity),
            (None, _) => (antecedent, antecedent),
        };
        Ok(match kind {
            Kind::IffImplies => {
                let (forward, backward) = (
                    self.add(Node::Implies(x, y))?,
                    self.add(Node::Implies(y, x))?,
                );
                let inner = self.implication(backward, c)?;
                vec![self.implication(forward, inner)?]
            }
            Kind::OrImplies => vec![self.implication(x, c)?, self.implication(y, c)?],
            Kind::ImpliesImplies => vec![self.implication(y, c)?],
            Kind::AndImplies => {
                let inner = self.implication(y, c)?;
                vec![self.implication(x, inner)?]
            }
            Kind::Axiom
            | Kind::FalseLeft
            | Kind::TrueRight
            | Kind::AndLeft
            | Kind::IffLeft
            | Kind::ImpliesRight
            | Kind::AndRight
            | Kind::IffRight
            | Kind::OrLeft
            | Kind::OrRight1
            | Kind::OrRight2
            | Kind::ModusPonens
            | Kind::TrueImplies => unreachable!("{kind:?} makes no formula"),
        })
    }

    /// Searches the sequent depth first with a stack of its own, and
    /// returns what it found, or why it stopped.
    fn prove(
        &mut self,
        key: Key,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<Known, Reason> {
        let mut frames: Vec<Frame> = Vec::new();
        // The verdict of the sequent opened or closed last, for the frame
        // below it; `None` while that frame is still open.
        let mut answer = self.open(key, &mut frames, stop)?;
        loop {
            let Some(top) = frames.last_mut() else {
                return Ok(answer.unwrap_or_else(|| unreachable!("the root has its verdict")));
            };
            match answer.take() {
                Some(Known::Proved(step)) => top.proved.push(step),
                Some(Known::Failed) => {
                    top.at += 1;
                    top.premises = None;
                    top.proved.clear();
                }
                None => {}
            }
            if top.premises.is_none() {
                let Some(&m) = top.moves.get(top.at) else {
                    let frame = frames.pop().unwrap_or_else(|| unreachable!("a frame"));
                    answer = Some(self.close(frame, false));
                    continue;
                };
                let key = (top.key.0.clone(), top.key.1);
                let premises = self.premises((&key.0, key.1), m)?;
                let top = frames.last_mut().unwrap_or_else(|| unreachable!("a frame"));
                top.premises = Some(premises);
            }
            let top = frames.last_mut().unwrap_or_else(|| unreachable!("a frame"));
            let premises = top.premises.as_deref().unwrap_or_default();
            let Some(next) = premises.get(top.proved.len()).cloned() else {
                let frame = frames.pop().unwrap_or_else(|| unreachable!("a frame"));
                answer = Some(self.close(frame, true));
                continue;
            };
            answer = self.open(next, &mut frames, stop)?;
            if self.account.over() {
                return Err(Reason::MemoryLimit {
                    limit_bytes: self.account.limit(),
                });
            }
        }
    }

    /// Opens a sequent: its verdict where the memo has it, else `None`
    /// with a frame pushed for it, counting the work and asking the stop.
    fn open(
        &mut self,
        key: Key,
        frames: &mut Vec<Frame>,
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<Option<Known>, Reason> {
        if let Some(&known) = self.memo.get(&key) {
            self.statistics.memo_hits += 1;
            return Ok(Some(known));
        }
        let done = self.statistics.sequents;
        if done >= self.work {
            return Err(Reason::WorkLimit { limit: self.work });
        }
        if done.is_multiple_of(POLL) && stop(Progress::new(Phase::Search, POLL.min(done), done)) {
            return Err(Reason::Stopped);
        }
        // Below the work bound, or below 2⁶⁴ expansions: it cannot wrap.
        self.statistics.sequents += 1;
        let moves = self.moves((&key.0, key.1));
        let frame = Frame {
            key,
            moves,
            at: 0,
            premises: None,
            proved: Vec::new(),
        };
        self.account.charge(frame_bytes(&frame));
        frames.push(frame);
        Ok(None)
    }

    /// Records a frame's verdict in the memo and, for a proof, its step,
    /// and returns the verdict.
    fn close(&mut self, frame: Frame, proved: bool) -> Known {
        self.account.release(frame_bytes(&frame));
        self.account.charge(bytes(frame.key.0.len()));
        let known = if proved {
            // A step per sequent proved, each charged: within the memory
            // bound, and without one below 2³² in any address space that
            // holds the memo of this many sequents.
            let id = u32::try_from(self.steps.len()).unwrap_or(u32::MAX);
            self.account
                .charge(size_of::<Step>() + bytes(frame.key.0.len()) + frame.proved.len() * 4);
            self.steps.push(Step {
                key: frame.key.clone(),
                applied: frame.moves[frame.at],
                premises: frame.proved,
            });
            Known::Proved(id)
        } else {
            Known::Failed
        };
        self.memo.insert(frame.key, known);
        known
    }
}

/// A fault planted in the translation, for the test that the checker
/// refuses a wrong one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Plant {
    /// The weakenings onto a derivation leave the last formula out.
    Weakening,
    /// The derivation of `A → (B → C)` from `(A ∧ B) → C` proves `A`
    /// where it needs `B`.
    Conjunct,
    /// The two cuts of `(A ∨ B) → C` both cut `A → C`.
    Disjunct,
    /// The contractions below a rule leave one copy too many.
    Contraction,
}

/// The translation of a proof found into a derivation of LJ.
struct Lj<'s, S> {
    /// The search, with its formulas and steps.
    search: &'s Search,
    /// The characters of every formula, for the size estimate.
    widths: Vec<u64>,
    /// The inferences made, premises first.
    inferences: Vec<Inference>,
    /// Their estimated size.
    bytes: u64,
    /// The bound on it.
    limit: Option<u64>,
    /// The memory the inferences hold.
    account: Account,
    /// The caller's stop.
    stop: S,
    /// A fault to plant, in tests.
    plant: Option<Plant>,
}

impl<'s, S: FnMut(Progress) -> bool> Lj<'s, S> {
    /// A translation of a proof of `search`, within `limits`.
    fn new(search: &'s Search, limits: &Limits, stop: S) -> Self {
        Self {
            search,
            widths: super::derivation::widths(&search.formulas),
            inferences: Vec::new(),
            bytes: 0,
            limit: limits.derivation_bytes,
            account: Account::holding(limits.memory_bytes, 0),
            stop,
            plant: None,
        }
    }

    /// Returns whether the fault is planted.
    fn planted(&self, plant: Plant) -> bool {
        self.plant == Some(plant)
    }

    /// Adds an inference, within the bounds and until the stop fires.
    fn push(
        &mut self,
        left: Vec<NodeId>,
        right: Vec<NodeId>,
        rule: Rule,
        principal: Option<(Side, usize)>,
        premises: Vec<InfId>,
    ) -> Result<InfId, Error> {
        let n = self.inferences.len() as u64;
        if (self.stop)(Progress::new(Phase::ReadBack, 1, n + 1)) {
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::ReadBack,
            }));
        }
        let estimate = super::derivation::inference_bytes(&self.widths, &left, &right);
        self.bytes = self.bytes.saturating_add(estimate);
        if let Some(limit) = self.limit
            && self.bytes > limit
        {
            return Err(Error::Refused(Refusal::Output {
                what: "derivation",
                estimate_bytes: self.bytes,
                limit_bytes: limit,
                least_bytes: None,
            }));
        }
        let held = (left.len() + right.len() + premises.len()) * size_of::<u32>();
        self.account.charge(size_of::<Inference>() + held);
        if self.account.over() {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: self.account.limit(),
                needed_bytes: None,
            }));
        }
        // The ids of a derivation are `u32`: past them is a refusal.
        let id = u32::try_from(n).map_err(|_| {
            Error::Refused(Refusal::Index {
                what: Space::Inference,
                count: n + 1,
                most: u64::from(u32::MAX),
            })
        })?;
        self.inferences.push(Inference {
            left,
            right,
            rule,
            principal,
            premises,
        });
        Ok(InfId::new(id))
    }

    /// `A ⊢ A`.
    fn axiom(&mut self, a: NodeId) -> Result<InfId, Error> {
        self.push(vec![a], vec![a], Rule::Axiom, None, Vec::new())
    }

    /// A rule on the hypothesis `p` of `left ⊢ right`.
    fn left_rule(
        &mut self,
        rule: Rule,
        left: Vec<NodeId>,
        right: NodeId,
        p: NodeId,
        premises: Vec<InfId>,
    ) -> Result<InfId, Error> {
        let principal = left.iter().position(|&x| x == p).map(|at| (Side::Left, at));
        self.push(left, vec![right], rule, principal, premises)
    }

    /// A rule on the goal of `left ⊢ goal`.
    fn right_rule(
        &mut self,
        rule: Rule,
        left: Vec<NodeId>,
        goal: NodeId,
        premises: Vec<InfId>,
    ) -> Result<InfId, Error> {
        self.push(left, vec![goal], rule, Some((Side::Right, 0)), premises)
    }

    /// The rule of `→`, or of `¬` for a negation, on a side.
    fn implication_rule(&self, id: NodeId, side: Side) -> Rule {
        let not = matches!(self.search.formulas.node(id), Node::Not(_));
        match (not, side) {
            (true, Side::Left) => Rule::NotLeft,
            (true, Side::Right) => Rule::NotRight,
            (false, Side::Left) => Rule::ImpliesLeft,
            (false, Side::Right) => Rule::ImpliesRight,
        }
    }

    /// Weakens the derivation `d` until it concludes `want` left of `⊢`,
    /// which holds its hypotheses.
    fn weaken_to(&mut self, d: InfId, want: &[NodeId]) -> Result<InfId, Error> {
        let inference = &self.inferences[d.index()];
        let (mut have, right) = (inference.left.clone(), inference.right.clone());
        let mut extra = difference(want, &have);
        if self.planted(Plant::Weakening) {
            extra.pop();
        }
        let mut d = d;
        for x in extra {
            have.push(x);
            let at = have.len() - 1;
            d = self.push(
                have.clone(),
                right.clone(),
                Rule::WeakenLeft,
                Some((Side::Left, at)),
                vec![d],
            )?;
        }
        Ok(d)
    }

    /// Contracts the derivation `d` until it concludes `want` left of
    /// `⊢`, which holds each of its hypotheses once at least.
    fn contract_to(&mut self, d: InfId, want: &[NodeId]) -> Result<InfId, Error> {
        let inference = &self.inferences[d.index()];
        let (mut have, right) = (inference.left.clone(), inference.right.clone());
        let mut extra = difference(&have, want);
        if self.planted(Plant::Contraction) {
            extra.pop();
        }
        let mut d = d;
        for x in extra {
            if let Some(at) = have.iter().position(|&y| y == x) {
                have.remove(at);
            }
            let at = have.iter().position(|&y| y == x);
            d = self.push(
                have.clone(),
                right.clone(),
                Rule::ContractLeft,
                at.map(|at| (Side::Left, at)),
                vec![d],
            )?;
        }
        Ok(d)
    }

    /// Returns `p ⊢ x → c` for `p` the implication `antecedent → c` and
    /// `antecedent` a disjunction of which `rule` takes `x`.
    fn disjunct(
        &mut self,
        p: NodeId,
        (x, c, xc): (NodeId, NodeId, NodeId),
        rule: Rule,
    ) -> Result<InfId, Error> {
        let (antecedent, _) = self
            .search
            .split(p)
            .unwrap_or_else(|| unreachable!("an implication"));
        let ax = self.axiom(x)?;
        let or = self.right_rule(rule, vec![x], antecedent, vec![ax])?;
        let cc = self.axiom(c)?;
        let left = self.implication_rule(p, Side::Left);
        let m = self.left_rule(left, vec![p, x], c, p, vec![or, cc])?;
        let right = self.implication_rule(xc, Side::Right);
        self.right_rule(right, vec![p], xc, vec![m])
    }

    /// Translates the proof whose last step is `root` into a derivation
    /// of LJ of `left ⊢ goal`, `left` as written, with a stack of its own:
    /// each step's premises first, then its rule.
    fn translate(mut self, root: u32, left: &[NodeId], goal: NodeId) -> Result<Derivation, Error> {
        /// What the walk has yet to do with a step.
        enum Task {
            /// Translate its premises, then it.
            Enter(u32),
            /// Translate it, its premises' derivations the last made.
            Exit(u32),
        }
        let mut tasks = vec![Task::Enter(root)];
        let mut made: Vec<InfId> = Vec::new();
        while let Some(task) = tasks.pop() {
            match task {
                Task::Enter(step) => {
                    tasks.push(Task::Exit(step));
                    let premises = &self.search.steps[step as usize].premises;
                    tasks.extend(premises.iter().rev().map(|&p| Task::Enter(p)));
                }
                Task::Exit(step) => {
                    let count = self.search.steps[step as usize].premises.len();
                    let premises = made.split_off(made.len() - count);
                    let d = self.step(step, &premises)?;
                    made.push(d);
                }
            }
        }
        let d = made
            .pop()
            .unwrap_or_else(|| unreachable!("the root's derivation"));
        self.weaken_to(d, left)?;
        let search = self.search;
        Ok(Derivation::from_parts(
            search.logic,
            search.formulas.clone(),
            (left.to_vec(), vec![goal]),
            self.inferences,
        ))
    }

    /// Returns the derivation of a step's sequent from its premises'.
    fn step(&mut self, step: u32, d: &[InfId]) -> Result<InfId, Error> {
        let search = self.search;
        let s = &search.steps[step as usize];
        let (gamma, goal) = (s.key.0.to_vec(), s.key.1);
        let p = s.applied.principal;
        let f = &search.formulas;
        // The hypotheses without the principal formula, with others added.
        let delta = |added: &[NodeId]| {
            let mut list = gamma.clone();
            if let Some(at) = list.iter().position(|&x| x == p) {
                list.remove(at);
            }
            list.extend_from_slice(added);
            list
        };
        let (a, b) = match f.node(p).operands() {
            (Some(a), Some(b)) => (a, b),
            (Some(a), None) => (a, search.falsity),
            (None, _) => (p, p),
        };
        let implies = |x, y| {
            f.find(Node::Implies(x, y))
                .unwrap_or_else(|| unreachable!("the search added it"))
        };
        match s.applied.kind {
            Kind::Axiom => {
                let ax = self.axiom(goal)?;
                self.weaken_to(ax, &gamma)
            }
            Kind::FalseLeft => self.left_rule(Rule::FalseLeft, gamma, goal, p, Vec::new()),
            Kind::TrueRight => self.right_rule(Rule::TrueRight, gamma, goal, Vec::new()),
            Kind::AndLeft => {
                let premise = self.weaken_to(d[0], &delta(&[a, b]))?;
                self.left_rule(Rule::AndLeft, gamma, goal, p, vec![premise])
            }
            Kind::IffLeft => {
                let premise = self.weaken_to(d[0], &delta(&[implies(a, b), implies(b, a)]))?;
                self.left_rule(Rule::IffLeft, gamma, goal, p, vec![premise])
            }
            Kind::ImpliesRight => {
                let premise = self.weaken_to(d[0], &[&gamma[..], &[a]].concat())?;
                let rule = self.implication_rule(goal, Side::Right);
                self.right_rule(rule, gamma, goal, vec![premise])
            }
            Kind::AndRight => self.right_rule(Rule::AndRight, gamma, goal, d.to_vec()),
            Kind::IffRight => self.right_rule(Rule::IffRight, gamma, goal, d.to_vec()),
            Kind::OrRight1 => self.right_rule(Rule::OrRight1, gamma, goal, d.to_vec()),
            Kind::OrRight2 => self.right_rule(Rule::OrRight2, gamma, goal, d.to_vec()),
            Kind::OrLeft => {
                let first = self.weaken_to(d[0], &delta(&[a]))?;
                let second = self.weaken_to(d[1], &delta(&[b]))?;
                self.left_rule(Rule::OrLeft, gamma, goal, p, vec![first, second])
            }
            Kind::TrueImplies => {
                let second = self.weaken_to(d[0], &delta(&[b]))?;
                let truth = self.right_rule(Rule::TrueRight, Vec::new(), a, Vec::new())?;
                let rule = self.implication_rule(p, Side::Left);
                self.left_rule(rule, gamma, goal, p, vec![truth, second])
            }
            Kind::ModusPonens => {
                // `a ⊢ a` and `Δ, b ⊢ G` split below `Δ, a, p`, `a` being
                // in `Δ`, then `a` contracted.
                let second = self.weaken_to(d[0], &delta(&[b]))?;
                let ax = self.axiom(a)?;
                let rule = self.implication_rule(p, Side::Left);
                let m = self.left_rule(rule, delta(&[a, p]), goal, p, vec![ax, second])?;
                self.contract_to(m, &gamma)
            }
            Kind::AndImplies | Kind::IffImplies => {
                let (Some(x), Some(y)) = f.node(a).operands() else {
                    unreachable!("a binary antecedent")
                };
                let (x, y, rule) = if s.applied.kind == Kind::AndImplies {
                    (x, y, Rule::AndRight)
                } else {
                    (implies(x, y), implies(y, x), Rule::IffRight)
                };
                // `x, y ⊢ a` by its right rule, then `p, x, y ⊢ b`,
                // `p, x ⊢ y → b` and `p ⊢ x → (y → b)`, the cut formula.
                let ax = self.axiom(x)?;
                let first = self.weaken_to(ax, &[x, y])?;
                let ax = self.axiom(if self.planted(Plant::Conjunct) { x } else { y })?;
                let second = self.weaken_to(ax, &[x, y])?;
                let pair = self.right_rule(rule, vec![x, y], a, vec![first, second])?;
                let bb = self.axiom(b)?;
                let left = self.implication_rule(p, Side::Left);
                let m = self.left_rule(left, vec![p, x, y], b, p, vec![pair, bb])?;
                let yb = search.implication_of(y, b);
                let right = self.implication_rule(yb, Side::Right);
                let inner = self.right_rule(right, vec![p, x], yb, vec![m])?;
                let made = search.implication_of(x, yb);
                let right = self.implication_rule(made, Side::Right);
                let schema = self.right_rule(right, vec![p], made, vec![inner])?;
                let premise = self.weaken_to(d[0], &delta(&[made]))?;
                self.push(gamma, vec![goal], Rule::Cut, None, vec![schema, premise])
            }
            Kind::OrImplies => {
                let (Some(x), Some(y)) = f.node(a).operands() else {
                    unreachable!("a disjunction")
                };
                let (xb, yb) = (search.implication_of(x, b), search.implication_of(y, b));
                let first = self.disjunct(p, (x, b, xb), Rule::OrRight1)?;
                let second = self.disjunct(p, (y, b, yb), Rule::OrRight2)?;
                let premise = self.weaken_to(d[0], &delta(&[xb, yb]))?;
                // Cut `y → b`, then `x → b`, each derived from a copy of
                // `p`, which is contracted below.
                let second = if self.planted(Plant::Disjunct) {
                    first
                } else {
                    second
                };
                let inner = self.push(
                    delta(&[p, xb]),
                    vec![goal],
                    Rule::Cut,
                    None,
                    vec![second, premise],
                )?;
                let outer = self.push(
                    delta(&[p, p]),
                    vec![goal],
                    Rule::Cut,
                    None,
                    vec![first, inner],
                )?;
                self.contract_to(outer, &gamma)
            }
            Kind::ImpliesImplies => {
                // `p` is `a → b`, `a` is `x → y`.
                let (x, y) = search
                    .split(a)
                    .unwrap_or_else(|| unreachable!("an implication"));
                let yb = search.implication_of(y, b);
                // `y ⊢ a`, then `p, y ⊢ b` and `p ⊢ y → b`, the cut formula.
                let ax = self.axiom(y)?;
                let w = self.weaken_to(ax, &[y, x])?;
                let right = self.implication_rule(a, Side::Right);
                let ya = self.right_rule(right, vec![y], a, vec![w])?;
                let bb = self.axiom(b)?;
                let left = self.implication_rule(p, Side::Left);
                let m = self.left_rule(left, vec![p, y], b, p, vec![ya, bb])?;
                let right = self.implication_rule(yb, Side::Right);
                let schema = self.right_rule(right, vec![p], yb, vec![m])?;
                // `Γ ⊢ a` by the cut and `Δ, b ⊢ G` from the second
                // premise split below `Γ, Δ, p`, which is `Γ, Γ`,
                // contracted to `Γ`.
                let first = self.weaken_to(d[0], &delta(&[yb]))?;
                let cut =
                    self.push(gamma.clone(), vec![a], Rule::Cut, None, vec![schema, first])?;
                let second = self.weaken_to(d[1], &delta(&[b]))?;
                let m = self.left_rule(
                    left,
                    [&gamma[..], &gamma[..]].concat(),
                    goal,
                    p,
                    vec![cut, second],
                )?;
                self.contract_to(m, &gamma)
            }
        }
    }
}

/// Returns `a` less `b` as multisets, each member of `b` taken from `a`
/// once where `a` has it.
fn difference(a: &[NodeId], b: &[NodeId]) -> Vec<NodeId> {
    let mut rest = a.to_vec();
    for x in b {
        if let Some(at) = rest.iter().position(|y| y == x) {
            rest.swap_remove(at);
        }
    }
    rest
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::linear::search::generate::Rng;

    /// Decides `text` by LJT in `logic` without bounds.
    fn ljt(text: &str, logic: Logic) -> Outcome {
        let sequent: Sequent = text.parse().unwrap();
        decide(&sequent, logic, &Limits::default(), |_| false).unwrap()
    }

    /// Returns whether the verdict is valid, panicking on an unknown one.
    fn valid(outcome: &Outcome) -> bool {
        match &outcome.verdict {
            Verdict::Valid(_) => true,
            Verdict::NotValid => false,
            Verdict::Unknown(reason) => panic!("unknown: {reason:?}"),
        }
    }

    /// On generated sequents LJT decides every one in intuitionistic and
    /// minimal logic, each proof it returns passes the checker again, no
    /// verdict contradicts the embedding's where that decides, and no
    /// valid sequent is refuted by the truth table.
    #[test]
    fn agrees_with_the_embedding() {
        const SEQUENTS: usize = 400;
        let search = crate::linear::Options::default().with_copies(Some(6));
        let bounded = Limits::default().with_work(Some(50_000));
        let unbounded = Limits::default();
        let mut rng = Rng::new(54);
        let mut proved = 0;
        for _ in 0..SEQUENTS {
            let sequent = super::super::tests::generated_sequent(&mut rng);
            for logic in [Logic::Intuitionistic, Logic::Minimal] {
                let outcome = decide(&sequent, logic, &unbounded, |_| false).unwrap();
                let answer = match &outcome.verdict {
                    Verdict::Valid(derivation) => {
                        assert_eq!(derivation.check(&unbounded, |_| false), Ok(()), "{sequent}");
                        proved += 1;
                        true
                    }
                    Verdict::NotValid => false,
                    Verdict::Unknown(reason) => panic!("{sequent} in {logic} logic: {reason:?}"),
                };
                let options = super::super::Options::default().with_logic(logic);
                let embedded =
                    super::super::decide(&sequent, &options, &search, &bounded, |_| false).unwrap();
                match embedded.verdict {
                    Verdict::Valid(_) => assert!(
                        answer,
                        "{sequent} in {logic} logic: the embedding proves it"
                    ),
                    Verdict::NotValid => assert!(
                        !answer,
                        "{sequent} in {logic} logic: the embedding refutes it"
                    ),
                    Verdict::Unknown(_) => {}
                }
                let classical = super::super::Options::default().with_logic(Logic::Classical);
                let affine =
                    super::super::decide(&sequent, &classical, &search, &unbounded, |_| false)
                        .unwrap();
                if answer {
                    assert!(
                        matches!(affine.verdict, Verdict::Valid(_)),
                        "{sequent} is not even classically valid"
                    );
                }
            }
        }
        assert!(proved >= SEQUENTS / 4, "{proved} proofs");
    }

    /// The sequents that separate the logics, and those of every rule.
    #[test]
    fn decides_the_separating_sequents() {
        // (sequent, intuitionistic, minimal)
        let cases = [
            ("a -> a", true, true),
            ("a \\/ ~a", false, false),
            ("~~(a \\/ ~a)", true, true),
            ("~~a -> a", false, false),
            ("((a -> b) -> a) -> a", false, false),
            ("false -> a", true, false),
            ("a \\/ b, ~a |- b", true, false),
            ("(a /\\ b -> c) |- a -> b -> c", true, true),
            ("(a \\/ b -> c) |- (a -> c) /\\ (b -> c)", true, true),
            ("((a -> b) -> c), b |- c", true, true),
            ("((a <-> b) -> c), a, b |- c", true, true),
            ("(true -> a) |- a", true, true),
            ("~(a /\\ b), a |- ~b", true, true),
            ("~(a \\/ b) |- ~a /\\ ~b", true, true),
            ("~(a -> b) |- ~b", true, true),
            ("a <-> b, a |- b", true, true),
            ("a, a, b |- a /\\ b", true, true),
            ("|- ~(a /\\ ~a)", true, true),
            ("a |-", false, false),
            ("a, ~a |-", true, true),
            ("~~~a -> ~a", true, true),
            ("((((a -> b) -> a) -> a) -> b) -> b", true, true),
        ];
        for (text, intuitionistic, minimal) in cases {
            assert_eq!(
                valid(&ljt(text, Logic::Intuitionistic)),
                intuitionistic,
                "{text}"
            );
            assert_eq!(
                valid(&ljt(text, Logic::Minimal)),
                minimal,
                "{text} minimally"
            );
        }
        let sequent: Sequent = "a |- a".parse().unwrap();
        assert!(matches!(
            decide(&sequent, Logic::Classical, &Limits::default(), |_| false),
            Err(Error::Logic { .. })
        ));
    }

    /// Each fault planted in the translation is refused by the checker,
    /// so that `decide` answers a defect and no verdict.
    #[test]
    fn a_planted_wrong_translation_is_refused() {
        let cases = [
            (Plant::Weakening, "a, b |- a"),
            (Plant::Conjunct, "(a /\\ b -> c), a, b |- c"),
            (Plant::Disjunct, "(a \\/ b -> c) |- (a -> c) /\\ (b -> c)"),
            (Plant::Contraction, "a, a -> b |- b"),
        ];
        for (plant, text) in cases {
            let sequent: Sequent = text.parse().unwrap();
            let honest = run(
                &sequent,
                Logic::Intuitionistic,
                &Limits::default(),
                |_| false,
                None,
            );
            assert!(
                matches!(honest.unwrap().verdict, Verdict::Valid(_)),
                "{text}"
            );
            let planted = run(
                &sequent,
                Logic::Intuitionistic,
                &Limits::default(),
                |_| false,
                Some(plant),
            );
            assert!(
                matches!(planted, Err(Error::ReadBack { .. })),
                "{plant:?}: {planted:?}"
            );
        }
    }

    /// The search stops at the work bound, at the caller's stop and at the
    /// memory bound, each an unknown verdict, and charges what it holds.
    #[test]
    fn the_search_keeps_to_its_bounds() {
        // Peirce's law fails after a search of several sequents.
        let sequent: Sequent = "((a -> b) -> a) -> a".parse().unwrap();
        let free = decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| {
            false
        })
        .unwrap();
        let sequents = free.statistics.sequents;
        assert!(sequents > 2, "{sequents}");
        let at = |work| Limits::default().with_work(Some(work));
        let outcome = decide(&sequent, Logic::Intuitionistic, &at(sequents - 1), |_| {
            false
        })
        .unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::WorkLimit { .. })
        ));
        let outcome = decide(&sequent, Logic::Intuitionistic, &at(sequents), |_| false).unwrap();
        assert!(matches!(outcome.verdict, Verdict::NotValid));
        let outcome = decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| {
            true
        })
        .unwrap();
        assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
        let tight = Limits::default().with_memory_bytes(Some(64));
        let outcome = decide(&sequent, Logic::Intuitionistic, &tight, |_| false).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::MemoryLimit { .. })
        ));
        // A proof whose derivation passes the bound on its size is refused.
        let sequent: Sequent = "a, b, c |- a /\\ (b /\\ c)".parse().unwrap();
        let small = Limits::default().with_derivation_bytes(Some(100));
        assert!(matches!(
            decide(&sequent, Logic::Intuitionistic, &small, |_| false),
            Err(Error::Refused(Refusal::Output { .. }))
        ));
    }
}
