// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Intuitionistic and minimal logic decided by a SAT solver, the
//! algorithm of intuit (K. Claessen and D. Rosén, *SAT modulo
//! intuitionistic implications*, LPAR-20, 2015) with the restarts of
//! intuitR (C. Fiorentini, *Efficient SAT-based proof search in
//! intuitionistic propositional logic*, CADE-28, 2021).
//!
//! Every subformula of the sequent is a variable of the solver, and each
//! connective's rules become clauses between a formula and its parts: the
//! flat clauses, which hold classically and intuitionistically alike, and
//! for every implication `a → b` that the sequent needs to prove the
//! implication clause `(a → b) → (a → b)`, which the solver cannot read.
//! The search looks for worlds, models of the flat clauses in which the
//! goal is false. A world that lacks an implication needs a world above it
//! that has `a` and not `b`; the solver looks for one under the
//! assumptions of the world and `a`. Where there is none, the call's failed
//! assumptions `A` give the clause `A → (a → b)`, which is learnt, and the
//! search starts again. Where every world has the worlds it needs, they
//! are a Kripke model in which the goal fails. Where the goal holds in
//! every model of the clauses, the sequent is valid.
//!
//! Both answers are checked. "Valid" comes with a derivation of LJ that
//! [`Derivation::check`] accepts, read off the learnt clauses and the last
//! call (the correspondence of Fiorentini, Goré and Graham-Lengrand,
//! *A proof-theoretic perspective on SMT-solving for intuitionistic
//! propositional logic*, TABLEAUX 2019), or by [`ljt`] where a bound
//! refuses that derivation. "Not valid" comes with the Kripke model, which
//! [`Model::refutes`] checks. Minimal logic is the same search with `⊥` a
//! variable like an atom.
//!
//! Needs the cargo feature `sat`.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::Limits;
//! use deduxit::ordinary::{self, Logic, Verdict, sat};
//!
//! let sequent: ordinary::Sequent = "~~(a \\/ ~a)".parse()?;
//! let outcome = sat::decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| false)?;
//! assert!(matches!(outcome.verdict, Verdict::Valid(_)));
//! let sequent: ordinary::Sequent = "a \\/ ~a".parse()?;
//! let outcome = sat::decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| false)?;
//! assert!(matches!(outcome.verdict, Verdict::NotValid));
//! let model = outcome.model.expect("a model refutes it");
//! assert_eq!(model.worlds.len(), 2);
//! # Ok::<(), deduxit::Error>(())
//! ```

mod model;
mod read_back;
mod solver;

pub use model::{Model, World};

use self::solver::{Answer, Batsat, Lit, MOST_VARIABLES, Solver};
use super::{Derivation, Formulas, Logic, Node, NodeId, Sequent, Verdict, ljt};
use crate::Error;
use crate::limits::{Limits, Progress, Refusal};
use crate::linear::search::Reason;
use crate::linear::search::memory::Account;

/// What deciding a sequent by the SAT solver found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict, with the derivation of LJ of a valid sequent.
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// What the search cost.
    pub statistics: Statistics,
    /// The Kripke model that refutes a sequent that is not valid, checked.
    pub model: Option<Model>,
}

/// What a search by the SAT solver cost.
///
/// # JSON
///
/// With the feature `serialize` it is written (never read) as
/// `{"calls": 7, "conflicts": 2, "lemmas": 1, "worlds": 0, "facts": 12,
/// "ljt": null}`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Statistics {
    /// The calls of the SAT solver.
    pub calls: u64,
    /// The conflicts the SAT solver met.
    pub conflicts: u64,
    /// The clauses learnt from implications, each followed by a restart.
    pub lemmas: u64,
    /// The worlds of the model that refutes the sequent.
    pub worlds: u64,
    /// The facts the read-back of the derivation derived.
    pub facts: u64,
    /// LJT's statistics where its search gave the derivation, the
    /// read-back having been refused by a bound.
    pub ljt: Option<ljt::Statistics>,
}

/// The polarity of a formula that the search's clauses take apart, as a
/// hypothesis does.
const NEGATIVE: u8 = 1;
/// The polarity of a formula that the search's clauses build, as a goal
/// is built.
const POSITIVE: u8 = 2;

/// A clause learnt from an implication `c = a → b` (or `c = ¬a`, `b`
/// being `⊥`): `body → c`, from a call that found `body, a ⊢ b`.
#[derive(Clone, Debug)]
struct Lemma {
    /// The formulas assumed, in ascending order.
    body: Vec<NodeId>,
    /// `a`.
    antecedent: NodeId,
    /// `b`.
    consequent: NodeId,
    /// `c`.
    made: NodeId,
}

/// Decides an intuitionistic or minimal sequent by the SAT solver within
/// `limits`, the stop asked at every decision of the solver and at every
/// call of it. A conflict or a call is a unit of `limits.work`, and the
/// clauses and worlds are charged to `limits.memory_bytes`. A valid
/// sequent's derivation is read off the search within
/// `limits.derivation_bytes`, `limits.memory_bytes` and `limits.work`
/// facts, or where a bound refuses it found by [`ljt::decide`] within the
/// same limits, and checked; a sequent that is not valid gets its Kripke
/// model, checked. A sequent with nothing right of `⊢` is decided as
/// `Γ ⊢ ⊥`, and its derivation concludes that sequent.
///
/// # Errors
///
/// [`Error::Logic`] for classical logic; [`Error::Succedents`] for more
/// than one formula right of `⊢`; [`Refusal::Output`],
/// [`Refusal::Memory`], [`Refusal::Work`] or [`Refusal::Stopped`] where a
/// bound or the stop refused the derivation of a proof (the read-back's
/// refusal, where LJT gave none either) or the check of a model, none of
/// them a verdict; as [`Derivation::check`]; [`Error::ReadBack`] or
/// [`Error::Countermodel`] where the derivation or the model fails its
/// check, a defect.
pub fn decide(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    run(sequent, logic, limits, stop, MOST_VARIABLES, true)
}

/// [`decide`] with at most `most` variables, which the tests lower, and
/// the derivation read off the search only if `read_off`, else by LJT,
/// which the tests ask for.
fn run(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
    most: u32,
    read_off: bool,
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
    // `a ↔ b` holds where `a → b` and `b → a` hold, which are formulas of
    // the arena; every equivalence is a node before them, so a pass over
    // the arena's first part adds them all.
    for id in 0..formulas.len() {
        if let Node::Iff(a, b) = formulas.node(NodeId(id as u32)) {
            formulas.add(Node::Implies(a, b))?;
            formulas.add(Node::Implies(b, a))?;
        }
    }
    let goal = sequent.right().first().copied().unwrap_or(falsity);
    let polarity = polarities(&formulas, sequent.left(), goal, falsity);
    let account = Account::holding(limits.memory_bytes, 0);
    account.charge(formulas.len() * (4 * size_of::<Node>() + size_of::<u32>() + 1));
    let mut statistics = Statistics::default();
    let found = {
        let work = limits.work.unwrap_or(u64::MAX);
        let mut solver = Batsat::new(work, &account, &mut stop);
        let found = Search::new(&mut solver, &formulas, &polarity, logic, falsity, most).and_then(
            |mut search| {
                let found = search.run(sequent.left(), goal, &account);
                statistics.lemmas = search.lemmas.len() as u64;
                found.map(|found| (found, search.lemmas, search.nodes))
            },
        );
        statistics.calls = solver.calls();
        statistics.conflicts = solver.conflicts();
        found
    };
    let (verdict, model) = match found {
        Err(reason) => (Verdict::Unknown(reason), None),
        Ok((Found::Worlds(worlds), _, nodes)) => {
            statistics.worlds = worlds.len() as u64;
            // The check computes a cell per world and formula at least:
            // past its bound the model is not built. Below it the worlds'
            // indices fit a `u32`.
            let cells = statistics
                .worlds
                .saturating_mul(sequent.formulas().len() as u64);
            if cells > Model::MOST_CELLS {
                return Err(Error::Refused(Refusal::Memory {
                    phase: crate::limits::Phase::Check,
                    limit_bytes: Model::MOST_CELLS / 8,
                    needed_bytes: Some(cells / 8),
                }));
            }
            let model = kripke(&formulas, logic, falsity, &nodes, worlds);
            model.refutes(sequent, logic, &mut stop)?;
            (Verdict::NotValid, Some(model))
        }
        Ok((Found::Valid, lemmas, _)) => {
            let refused = Error::Refused(Refusal::Output {
                what: "derivation",
                estimate_bytes: 0,
                limit_bytes: 0,
                least_bytes: None,
            });
            let read = if read_off {
                read_back::derivation(
                    formulas,
                    logic,
                    &polarity,
                    (falsity, goal),
                    sequent.left(),
                    &lemmas,
                    limits,
                    account.used(),
                    &mut stop,
                )
            } else {
                Err(refused)
            };
            let derivation = match read {
                Ok((derivation, facts)) => {
                    statistics.facts = facts;
                    derivation
                }
                Err(Error::Refused(refusal)) if !matches!(refusal, Refusal::Stopped { .. }) => {
                    let (derivation, ljt) = by_ljt(sequent, logic, limits, &mut stop, refusal)?;
                    statistics.ljt = Some(ljt);
                    derivation
                }
                Err(error) => return Err(error),
            };
            derivation.check(limits, &mut stop)?;
            (Verdict::Valid(Box::new(derivation)), None)
        }
    };
    Ok(Outcome {
        verdict,
        logic,
        statistics,
        model,
    })
}

/// Returns LJT's derivation of a sequent the search proved, and its
/// statistics; where LJT gives none, the read-back's `refusal`.
fn by_ljt(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    stop: &mut impl FnMut(Progress) -> bool,
    refusal: Refusal,
) -> Result<(Derivation, ljt::Statistics), Error> {
    let outcome = match ljt::decide(sequent, logic, limits, &mut *stop) {
        Ok(outcome) => outcome,
        Err(Error::Refused(_)) => return Err(Error::Refused(refusal)),
        Err(other) => return Err(other),
    };
    match outcome.verdict {
        Verdict::Valid(derivation) => Ok((*derivation, outcome.statistics)),
        Verdict::Unknown(_) => Err(Error::Refused(refusal)),
        Verdict::NotValid => Err(Error::ReadBack {
            calculus: logic.calculus(),
            reason: "LJT refutes a sequent the SAT search proved".to_owned(),
        }),
    }
}

/// Returns the polarity of every formula of the arena that the clauses
/// of `left ⊢ goal` name: negative for those taken apart, as hypotheses
/// are, positive for those built, as the goal is, both for the parts of
/// an equivalence; `⊥` takes the polarity of every negation.
fn polarities(formulas: &Formulas, left: &[NodeId], goal: NodeId, falsity: NodeId) -> Vec<u8> {
    let mut polarity = vec![0u8; formulas.len()];
    for h in left {
        polarity[h.index()] |= NEGATIVE;
    }
    polarity[goal.index()] |= POSITIVE;
    // Every formula comes after its parts, so a pass from the last formula
    // to the first meets each before its parts.
    for id in (0..formulas.len()).rev() {
        let p = polarity[id];
        if p == 0 {
            continue;
        }
        let flipped = ((p & NEGATIVE) << 1) | ((p & POSITIVE) >> 1);
        match formulas.node(NodeId(id as u32)) {
            Node::Atom(_) | Node::True | Node::False => {}
            Node::Not(a) => {
                polarity[a.index()] |= flipped;
                polarity[falsity.index()] |= p;
            }
            Node::And(a, b) | Node::Or(a, b) => {
                polarity[a.index()] |= p;
                polarity[b.index()] |= p;
            }
            Node::Implies(a, b) => {
                polarity[a.index()] |= flipped;
                polarity[b.index()] |= p;
            }
            Node::Iff(a, b) => {
                // The implications may come after the equivalence: their
                // parts get both polarities here, as they would from them.
                polarity[a.index()] |= NEGATIVE | POSITIVE;
                polarity[b.index()] |= NEGATIVE | POSITIVE;
                for i in [Node::Implies(a, b), Node::Implies(b, a)] {
                    if let Some(i) = formulas.find(i) {
                        polarity[i.index()] |= p;
                    }
                }
            }
        }
    }
    polarity
}

/// What the search found.
enum Found {
    /// The goal holds in every model of the clauses.
    Valid,
    /// The worlds of a Kripke model in which it fails, each the
    /// variables true in it, the first the least.
    Worlds(Vec<Bits>),
}

/// A set of variables, as bits.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Bits(Vec<u64>);

impl Bits {
    /// The empty set of `len` variables.
    fn new(len: usize) -> Self {
        Self(vec![0; len.div_ceil(64)])
    }

    /// Returns whether the set holds `v`.
    fn has(&self, v: u32) -> bool {
        self.0[v as usize / 64] >> (v % 64) & 1 == 1
    }

    /// Adds `v`.
    fn insert(&mut self, v: u32) {
        self.0[v as usize / 64] |= 1 << (v % 64);
    }

    /// Returns whether every member of `self` is one of `other`.
    fn within(&self, other: &Self) -> bool {
        self.0.iter().zip(&other.0).all(|(a, b)| a & !b == 0)
    }

    /// Returns the members, in ascending order.
    fn members(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().enumerate().flat_map(|(k, &word)| {
            (0..64u32)
                .filter(move |b| word >> b & 1 == 1)
                .map(move |b| k as u32 * 64 + b)
        })
    }

    /// Returns the bytes it holds.
    fn bytes(&self) -> usize {
        self.0.len() * size_of::<u64>()
    }
}

/// The search: the solver with the clauses of a sequent, and the
/// implications it must realise.
struct Search<'a, S> {
    /// The solver.
    solver: &'a mut S,
    /// The formula of every variable.
    nodes: Vec<NodeId>,
    /// The variable of every formula that has one.
    vars: Vec<Option<u32>>,
    /// The implications `(a → b) → c`, by their variables `[a, b, c]`.
    implications: Vec<[u32; 3]>,
    /// The clauses learnt.
    lemmas: Vec<Lemma>,
}

impl<'a, S: Solver> Search<'a, S> {
    /// The solver given the clauses of the formulas `polarity` names,
    /// at most `most` variables.
    fn new(
        solver: &'a mut S,
        formulas: &Formulas,
        polarity: &[u8],
        logic: Logic,
        falsity: NodeId,
        most: u32,
    ) -> Result<Self, Reason> {
        let mut vars = vec![None; formulas.len()];
        let mut nodes = Vec::new();
        for (id, var) in vars.iter_mut().enumerate() {
            if polarity[id] != 0 || id == falsity.index() {
                if nodes.len() >= most as usize {
                    return Err(Reason::IndexLimit);
                }
                *var = Some(solver.new_var());
                nodes.push(NodeId(id as u32));
            }
        }
        let mut search = Self {
            solver,
            nodes,
            vars,
            implications: Vec::new(),
            lemmas: Vec::new(),
        };
        let var = |x: NodeId| search.vars[x.index()].unwrap_or_else(|| unreachable!("a name"));
        let (pos, neg) = (|x| Lit::positive(var(x)), |x| Lit::negative(var(x)));
        let mut clauses: Vec<Vec<Lit>> = Vec::new();
        let mut implications = Vec::new();
        if logic == Logic::Intuitionistic {
            clauses.push(vec![neg(falsity)]);
        }
        for (id, &p) in polarity.iter().enumerate() {
            if p == 0 {
                continue;
            }
            let x = NodeId(id as u32);
            let (negative, positive) = (p & NEGATIVE != 0, p & POSITIVE != 0);
            match formulas.node(x) {
                Node::Atom(_) | Node::False => {}
                Node::True => {
                    if positive {
                        clauses.push(vec![pos(x)]);
                    }
                }
                Node::And(a, b) => {
                    if negative {
                        clauses.push(vec![neg(x), pos(a)]);
                        clauses.push(vec![neg(x), pos(b)]);
                    }
                    if positive {
                        clauses.push(vec![neg(a), neg(b), pos(x)]);
                    }
                }
                Node::Or(a, b) => {
                    if negative {
                        clauses.push(vec![neg(x), pos(a), pos(b)]);
                    }
                    if positive {
                        clauses.push(vec![neg(a), pos(x)]);
                        clauses.push(vec![neg(b), pos(x)]);
                    }
                }
                Node::Implies(a, b) => {
                    if negative {
                        clauses.push(vec![neg(x), neg(a), pos(b)]);
                    }
                    if positive {
                        clauses.push(vec![neg(b), pos(x)]);
                        implications.push([var(a), var(b), var(x)]);
                    }
                }
                Node::Not(a) => {
                    if negative {
                        clauses.push(vec![neg(x), neg(a), pos(falsity)]);
                    }
                    if positive {
                        clauses.push(vec![neg(falsity), pos(x)]);
                        implications.push([var(a), var(falsity), var(x)]);
                    }
                }
                Node::Iff(a, b) => {
                    let i = formulas.find(Node::Implies(a, b));
                    let j = formulas.find(Node::Implies(b, a));
                    let (Some(i), Some(j)) = (i, j) else {
                        unreachable!("the implications of every equivalence were added")
                    };
                    if negative {
                        clauses.push(vec![neg(x), pos(i)]);
                        clauses.push(vec![neg(x), pos(j)]);
                    }
                    if positive {
                        clauses.push(vec![neg(i), neg(j), pos(x)]);
                    }
                }
            }
        }
        for clause in &clauses {
            search.solver.add_clause(clause);
        }
        search.implications = implications;
        Ok(search)
    }

    /// Runs intuitR's loop on `left ⊢ goal`, charging the worlds to the
    /// account.
    fn run(&mut self, left: &[NodeId], goal: NodeId, account: &Account) -> Result<Found, Reason> {
        let var = |x: NodeId| self.vars[x.index()].unwrap_or_else(|| unreachable!("a name"));
        let units: Vec<Lit> = left.iter().map(|&h| Lit::positive(var(h))).collect();
        for unit in units {
            self.solver.add_clause(&[unit]);
        }
        let goal = var(goal);
        loop {
            match self.solver.solve(&[Lit::negative(goal)]) {
                Answer::Unsatisfiable => return Ok(Found::Valid),
                Answer::Stopped(reason) => return Err(reason),
                Answer::Satisfiable => {}
            }
            let mut worlds = vec![self.world()];
            let mut held = worlds[0].bytes();
            account.charge(held);
            let learnt = self.realise(&mut worlds, account, &mut held);
            account.release(held);
            match learnt {
                Ok(true) => {}
                Ok(false) => return Ok(Found::Worlds(worlds)),
                Err(reason) => return Err(reason),
            }
        }
    }

    /// Adds worlds until every implication is realised at every world,
    /// answering `false`, or until a call learns a clause, answering
    /// `true`, which restarts the search.
    fn realise(
        &mut self,
        worlds: &mut Vec<Bits>,
        account: &Account,
        held: &mut usize,
    ) -> Result<bool, Reason> {
        // A pair passed over is realised, and stays so as worlds are
        // added, so one cursor walks the pairs.
        let (mut w, mut k) = (0, 0);
        loop {
            if k == self.implications.len() {
                (w, k) = (w + 1, 0);
            }
            if w == worlds.len() {
                return Ok(false);
            }
            let [a, b, c] = self.implications[k];
            if !unrealised(worlds, w, [a, b, c]) {
                k += 1;
                continue;
            }
            let mut assumptions: Vec<Lit> = worlds[w].members().map(Lit::positive).collect();
            assumptions.push(Lit::positive(a));
            assumptions.push(Lit::negative(b));
            match self.solver.solve(&assumptions) {
                Answer::Satisfiable => {
                    let world = self.world();
                    *held += world.bytes();
                    account.charge(world.bytes());
                    worlds.push(world);
                    if account.over() {
                        return Err(Reason::MemoryLimit {
                            limit_bytes: account.limit(),
                        });
                    }
                    k += 1;
                }
                Answer::Unsatisfiable => {
                    let mut body: Vec<u32> = self
                        .solver
                        .failed()
                        .into_iter()
                        .filter(|l| l.is_positive() && l.var() != a)
                        .map(Lit::var)
                        .collect();
                    body.sort_unstable();
                    body.dedup();
                    let mut clause: Vec<Lit> = body.iter().map(|&v| Lit::negative(v)).collect();
                    clause.push(Lit::positive(c));
                    self.solver.add_clause(&clause);
                    let node = |v: u32| self.nodes[v as usize];
                    let mut body: Vec<NodeId> = body.into_iter().map(node).collect();
                    body.sort_unstable();
                    self.lemmas.push(Lemma {
                        body,
                        antecedent: node(a),
                        consequent: node(b),
                        made: node(c),
                    });
                    return Ok(true);
                }
                Answer::Stopped(reason) => return Err(reason),
            }
        }
    }

    /// Returns the variables true in the solver's model.
    fn world(&self) -> Bits {
        let mut world = Bits::new(self.nodes.len());
        for v in 0..self.nodes.len() as u32 {
            if self.solver.value(Lit::positive(v)) {
                world.insert(v);
            }
        }
        world
    }
}

/// Returns whether the implication `(a → b) → c` is unrealised at the
/// world `w`: `a`, `b` and `c` are false there, and no world above it has
/// `a` and not `b` (intuitR's realisability).
fn unrealised(worlds: &[Bits], w: usize, [a, b, c]: [u32; 3]) -> bool {
    let world = &worlds[w];
    if world.has(a) || world.has(b) || world.has(c) {
        return false;
    }
    // A world that has `a` is not `w`, so it is above `w` where it holds
    // `w`.
    !worlds
        .iter()
        .any(|other| other.has(a) && !other.has(b) && world.within(other))
}

/// Returns the Kripke model of the worlds: ordered by inclusion, the
/// least first, each with the atoms of its variables, and `⊥` in minimal
/// logic.
fn kripke(
    formulas: &Formulas,
    logic: Logic,
    falsity: NodeId,
    nodes: &[NodeId],
    mut worlds: Vec<Bits>,
) -> Model {
    // A world strictly above another has more variables.
    worlds.sort_by_key(|w| w.members().count());
    let n = worlds.len();
    // Every world strictly below each, as bits over the worlds.
    let mut below: Vec<Bits> = Vec::with_capacity(n);
    for j in 0..n {
        let mut set = Bits::new(n);
        for i in 0..j {
            if worlds[i] != worlds[j] && worlds[i].within(&worlds[j]) {
                set.insert(i as u32);
            }
        }
        below.push(set);
    }
    // The worlds directly below: below, and below no world below.
    let mut above = vec![Vec::new(); n];
    for j in 0..n {
        let mut direct = below[j].clone();
        for i in below[j].members() {
            for (word, under) in direct.0.iter_mut().zip(&below[i as usize].0) {
                *word &= !under;
            }
        }
        for i in direct.members() {
            above[i as usize].push(j as u32);
        }
    }
    let models = worlds.iter().zip(above).map(|(world, above)| {
        let mut atoms = Vec::new();
        let mut forces_falsity = false;
        for v in world.members() {
            let x = nodes[v as usize];
            match formulas.node(x) {
                Node::Atom(a) => atoms.push(a),
                Node::False if x == falsity && logic == Logic::Minimal => forces_falsity = true,
                Node::True
                | Node::False
                | Node::Not(_)
                | Node::And(..)
                | Node::Or(..)
                | Node::Implies(..)
                | Node::Iff(..) => {}
            }
        }
        atoms.sort_unstable();
        World {
            atoms,
            falsity: forces_falsity,
            above,
        }
    });
    Model {
        worlds: models.collect(),
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::linear::search::generate::Rng;

    /// Decides `text` by the SAT solver in `logic` within the default
    /// limits.
    fn sat(text: &str, logic: Logic) -> Outcome {
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

    /// On generated sequents the SAT solver decides every one in
    /// intuitionistic and minimal logic as LJT does; each derivation is
    /// read off the search and passes the checker again, and each model
    /// refutes its sequent again.
    #[test]
    fn agrees_with_ljt() {
        const SEQUENTS: usize = 400;
        let limits = Limits::default();
        let mut rng = Rng::new(54);
        let (mut proved, mut refuted) = (0, 0);
        for _ in 0..SEQUENTS {
            let sequent = super::super::tests::generated_sequent(&mut rng);
            for logic in [Logic::Intuitionistic, Logic::Minimal] {
                let outcome = decide(&sequent, logic, &limits, |_| false)
                    .unwrap_or_else(|e| panic!("{sequent} in {logic} logic: {e}"));
                let answer = match &outcome.verdict {
                    Verdict::Valid(derivation) => {
                        assert_eq!(derivation.check(&limits, |_| false), Ok(()), "{sequent}");
                        assert_eq!(outcome.statistics.ljt, None, "{sequent}: read off");
                        proved += 1;
                        true
                    }
                    Verdict::NotValid => {
                        let model = outcome.model.as_ref().unwrap();
                        assert_eq!(model.refutes(&sequent, logic, |_| false), Ok(()));
                        refuted += 1;
                        false
                    }
                    Verdict::Unknown(reason) => panic!("{sequent} in {logic} logic: {reason:?}"),
                };
                let ljt = ljt::decide(&sequent, logic, &limits, |_| false).unwrap();
                assert_eq!(
                    answer,
                    matches!(ljt.verdict, Verdict::Valid(_)),
                    "{sequent} in {logic} logic"
                );
            }
        }
        assert!(
            proved >= SEQUENTS / 4 && refuted >= SEQUENTS / 4,
            "{proved}, {refuted}"
        );
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
            ("(a <-> b) <-> (b <-> a)", true, true),
            ("|- true", true, true),
            ("false |- false", true, true),
        ];
        for (text, intuitionistic, minimal) in cases {
            assert_eq!(
                valid(&sat(text, Logic::Intuitionistic)),
                intuitionistic,
                "{text}"
            );
            assert_eq!(
                valid(&sat(text, Logic::Minimal)),
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

    /// A model changed in any of the ways the check guards against is
    /// refused, each for its own reason.
    #[test]
    fn a_planted_wrong_model_is_refused() {
        let text = "((a -> b) -> a) -> a";
        let sequent: Sequent = text.parse().unwrap();
        let logic = Logic::Intuitionistic;
        let model = sat(text, logic).model.unwrap();
        assert_eq!(model.refutes(&sequent, logic, |_| false), Ok(()));
        assert!(model.worlds.len() >= 2, "{model:?}");
        let refused = |model: &Model, why: &str| {
            let Err(Error::Countermodel { reason }) = model.refutes(&sequent, logic, |_| false)
            else {
                panic!("{why}: {model:?} is accepted");
            };
            assert!(reason.contains(why), "{reason} for {why}");
        };
        // The root's conclusion made true everywhere.
        let mut planted = model.clone();
        for world in &mut planted.worlds {
            world.atoms = vec![0, 1];
        }
        refused(&planted, "forces the conclusion");
        // A world above that forgets an atom below it.
        let mut planted = model.clone();
        let up = planted.worlds[0].above[0] as usize;
        planted.worlds[0].atoms.push(0);
        planted.worlds[up].atoms.clear();
        refused(&planted, "does not force all it forces");
        // The edges taken away: the root alone forces the formula.
        let mut planted = model.clone();
        planted.worlds.truncate(1);
        planted.worlds[0].above.clear();
        refused(&planted, "forces the conclusion");
        // An edge back, an atom that does not exist, ⊥ forced.
        let mut planted = model.clone();
        let last = planted.worlds.len() - 1;
        planted.worlds[last].above.push(0);
        refused(&planted, "does not come after it");
        let mut planted = model.clone();
        planted.worlds[0].atoms = vec![7];
        refused(&planted, "no atom has the index 7");
        let mut planted = model.clone();
        planted.worlds[0].falsity = true;
        refused(&planted, "forces ⊥ in intuitionistic logic");
        let mut planted = model;
        planted.worlds[0].above.clear();
        refused(&planted, "is not above the root");
    }

    /// The search stops at the work bound, the stop and the memory bound,
    /// refuses a sequent of more formulas than it has variables, and a
    /// derivation past its bound is refused when LJT gives none either;
    /// the model's check refuses past its cells.
    #[test]
    fn the_search_keeps_to_its_bounds() {
        let sequent: Sequent = "((a -> b) -> a) -> a".parse().unwrap();
        let logic = Logic::Intuitionistic;
        let free = decide(&sequent, logic, &Limits::default(), |_| false).unwrap();
        assert!(matches!(free.verdict, Verdict::NotValid));
        let calls = free.statistics.calls;
        assert!(calls > 2, "{calls}");
        let at = |work| Limits::default().with_work(Some(work));
        let outcome = decide(&sequent, logic, &at(calls - 1), |_| false).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::WorkLimit { .. })
        ));
        let outcome = decide(&sequent, logic, &Limits::default(), |_| true).unwrap();
        assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
        let tight = Limits::default().with_memory_bytes(Some(64));
        let outcome = decide(&sequent, logic, &tight, |_| false).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::MemoryLimit { .. })
        ));
        // Five formulas and `⊥` need six variables.
        let outcome = run(&sequent, logic, &Limits::default(), |_| false, 5, true).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::IndexLimit)
        ));
        assert!(matches!(
            run(&sequent, logic, &Limits::default(), |_| false, 6, true)
                .unwrap()
                .verdict,
            Verdict::NotValid
        ));
        let sequent: Sequent = "a, b, c |- a /\\ (b /\\ c)".parse().unwrap();
        let small = Limits::default().with_derivation_bytes(Some(100));
        assert!(matches!(
            decide(&sequent, logic, &small, |_| false),
            Err(Error::Refused(Refusal::Output { .. }))
        ));
        // A model of more cells than the check computes.
        let chain: Vec<String> = (0..1 << 14).map(|i| format!("a{i}")).collect();
        let sequent: Sequent = format!("|- {}", chain.join(" /\\ ")).parse().unwrap();
        let worlds = (Model::MOST_CELLS as usize).div_ceil(sequent.formulas().len()) + 1;
        let model = Model {
            worlds: vec![World::default(); worlds],
        };
        assert!(matches!(
            model.refutes(&sequent, logic, |_| false),
            Err(Error::Refused(Refusal::Memory { .. }))
        ));
    }

    /// Where a bound refuses the derivation read off the search, LJT
    /// gives it.
    #[test]
    fn ljt_gives_the_derivation_the_read_back_cannot() {
        let sequent: Sequent = "a, a -> b, b -> c |- c".parse().unwrap();
        let logic = Logic::Intuitionistic;
        let outcome = run(
            &sequent,
            logic,
            &Limits::default(),
            |_| false,
            MOST_VARIABLES,
            false,
        );
        let outcome = outcome.unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)), "{outcome:?}");
        assert!(outcome.statistics.ljt.is_some(), "{:?}", outcome.statistics);
    }
}
