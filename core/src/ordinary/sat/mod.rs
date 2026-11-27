// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Intuitionistic and minimal logic decided by a SAT solver, the
//! algorithm of intuit (K. Claessen and D. Rosén, *SAT modulo
//! intuitionistic implications*, LPAR-20, 2015) with the restarts of
//! intuitR (C. Fiorentini, *Efficient SAT-based proof search in
//! intuitionistic propositional logic*, CADE-28, 2021).
//!
//! Every subformula of the sequent is a variable of the solver. Each
//! connective's rules become clauses between a formula and its parts.
//! Most are flat clauses, which hold classically and intuitionistically
//! alike. The exception is the implication clause `(a → b) → c`, where `c`
//! is the variable of `a → b`, one for every implication the sequent needs
//! to prove. The solver cannot read it, so the search handles it.
//!
//! The search looks for worlds: models of the flat clauses in which the
//! goal is false. A world where `a`, `b` and `c` are all false needs a
//! world above it that has `a` and not `b`. The solver looks for one, with
//! `a`, `¬b` and the world as assumptions. Where there is none, the
//! world's assumptions that the refutation used, `A`, give the clause
//! `A → c`. The search learns it and starts again. Where every world has
//! the worlds it needs, they are a Kripke model in which the goal fails.
//! Where the goal holds in every model of the clauses, the sequent is
//! valid.
//!
//! Both answers are checked. "Valid" comes with a derivation of LJ that
//! [`Derivation::check`] accepts. It is read off the learnt clauses and
//! the last call (the correspondence of Fiorentini, Goré and
//! Graham-Lengrand, *A proof-theoretic perspective on SMT-solving for
//! intuitionistic propositional logic*, TABLEAUX 2019), or found by
//! [`ljt`] where a bound refuses that derivation. "Not valid" comes with
//! the Kripke model, which
//! [`Model::refutes`](crate::kripke::Model::refutes) checks. Minimal logic
//! is the same search with `⊥` a variable like an atom.
//!
//! Classical logic takes one call of the solver on the clauses of the
//! sequent's negation (Tseitin's encoding with polarities). "Not valid"
//! comes with a model of one world, the assignment that falsifies the
//! sequent, which [`Model::refutes`](crate::kripke::Model::refutes)
//! checks by evaluation. "Valid" comes with the clauses the solver learnt
//! and deleted, recorded as a clausal proof (DRUP). This module's own
//! checker accepts it by unit propagation against the clauses
//! ([`ClausalProof`]). A clausal proof is no derivation:
//! [`decide`](super::decide()) with
//! [`Method::Embedding`](super::Method::Embedding) gives the derivation of
//! LK.
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
//! let countermodel = outcome.model.expect("a model refutes it");
//! assert_eq!(countermodel.model.worlds.len(), 2);
//! # Ok::<(), deduxit::Error>(())
//! ```

mod classical;
pub(crate) mod drup;
mod read_back;
pub(crate) mod solver;

pub(crate) use classical::falsify;
pub use drup::ClausalProof;

use crate::kripke::{Model, World};

use self::solver::{Answer, Batsat, Lit, MOST_VARIABLES, Solver};
use super::{Derivation, Evidence, Formulas, Logic, Node, NodeId, Sequent, Verdict, ljt};
use crate::Error;
use crate::limits::{Limits, Progress, Refusal};
use crate::linear::search::Reason;
use crate::memory::Account;

/// What deciding a sequent by the SAT solver found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict, with the evidence of a valid sequent: a derivation of
    /// LJ, or in classical logic a clausal proof.
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// What the search cost.
    pub statistics: Statistics,
    /// The Kripke model that refutes a sequent that is not valid, with the
    /// sequent, checked. `None` for any other verdict, and where the model
    /// is too large to check.
    pub model: Option<super::Countermodel>,
}

/// What a search by the SAT solver cost.
///
/// # JSON
///
/// With the feature `serialize` it is written and read back as
/// `{"calls": 7, "conflicts": 2, "lemmas": 1, "worlds": 0, "facts": 12,
/// "clauses": 0, "proof_steps": 0, "ljt": null}`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default))]
pub struct Statistics {
    /// The calls of the SAT solver.
    pub calls: u64,
    /// The conflicts the SAT solver met.
    pub conflicts: u64,
    /// The clauses learnt from implications, each followed by a restart.
    pub lemmas: u64,
    /// The worlds of the model that refutes the sequent, counted also where
    /// the model is too large to keep.
    pub worlds: u64,
    /// The facts the read-back of the derivation derived.
    pub facts: u64,
    /// In classical logic, the clauses of the sequent's negation the
    /// solver was given.
    pub clauses: u64,
    /// In classical logic, the steps of the clausal proof that the
    /// clauses have no model, which the checker accepted: the clauses the
    /// solver learnt and deleted, and the empty clause.
    pub proof_steps: u64,
    /// LJT's statistics where its search gave the derivation: where a
    /// bound refused the read-back, or where the logic's default prefers
    /// LJT's derivation ([`decide_preferring_ljt`]).
    pub ljt: Option<ljt::Statistics>,
}

/// How many pairs of a world and an implication the search examines
/// between two polls of the stop.
const POLL: u64 = 1 << 8;

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

/// Decides a sequent by the SAT solver within `limits`.
///
/// The stop is asked:
/// - at every call of the solver, the first included, and at every
///   decision in it;
/// - while the worlds are scanned for an implication they do not realise,
///   every 256 pairs;
/// - while the worlds of a model are ordered, every 256 comparisons, and
///   by the model's check
///   ([`Countermodel::check`](super::Countermodel::check)), once per world
///   in each of its passes;
/// - while a valid sequent's derivation is read off the search, by LJT
///   where it gives that derivation, and by the derivation's check
///   ([`Derivation::check`]);
/// - in classical logic, while the sequent is encoded, every 4 096
///   formulas, while its clauses are handed to the solver, every 4 096
///   clauses, and by the clausal proof's check.
///
/// A conflict or a call is a unit of `limits.work`, and the clauses
/// and worlds are charged to `limits.memory_bytes`. A valid sequent's
/// derivation of LJ is read off the search within
/// `limits.memory_bytes` and `limits.work`
/// facts. Where a bound, not the stop, refuses it, [`ljt::decide`]
/// finds one within the same limits. Either derivation is checked. A
/// sequent that is not valid gets its Kripke model, checked, unless the
/// model is too large to check: then the verdict stands without it. A
/// sequent with nothing right of `⊢` is decided as `Γ ⊢ ⊥`, and its
/// derivation concludes that sequent.
///
/// In classical logic a valid sequent's evidence is its clausal proof
/// ([`ClausalProof`]), checked within the same limits. A step of it is a
/// unit of `limits.work`; the solver and the check each have the whole
/// bound. A clausal proof is no derivation: [`decide`](super::decide())
/// with [`Method::Embedding`](super::Method::Embedding) gives one. A
/// sequent that is not valid gets a model of one world, checked.
///
/// # Errors
///
/// - [`Error::Succedents`] for more than one formula right of `⊢` in
///   intuitionistic or minimal logic.
/// - As [`Derivation::check`], but for its bounds: a bound or the stop
///   that ends the derivation of a proof (where LJT gave none either), the
///   check of a model or, in classical logic, the check of the clausal
///   proof is no error, the outcome unknown with that reason and the
///   search's statistics, as for a bound that ends the search.
/// - [`Error::ReadBack`] where the derivation or the clausal proof fails
///   its check, or where LJT refutes a sequent the search proved, and
///   [`Error::Countermodel`] where the model fails its check: each a
///   defect.
pub fn decide(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    run(
        sequent,
        logic,
        limits,
        &mut stop,
        MOST_VARIABLES,
        Derive::ReadOff,
    )
}

/// Decides a sequent as [`decide`] does, but takes a valid sequent's
/// derivation of LJ from [`ljt::decide`] where LJT gives one within the
/// limits and a bound of its own on the sequents it expands, whose drawing
/// fits `limits.derivation_bytes`, before the stop, and otherwise the one
/// read off the search. LJT's derivations are
/// mostly smaller, and their lambda-terms have fewer redexes; where LJT's
/// could not be drawn, the one read off often can (ILTP's SYJ201 from
/// size 6). The verdict is the search's either way. The logic's default
/// decides intuitionistic and minimal logic this way
/// ([`decide`](super::decide()) with no method given). In classical logic
/// this function is [`decide`].
///
/// # Errors
///
/// As [`decide`].
pub fn decide_preferring_ljt(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    run(
        sequent,
        logic,
        limits,
        &mut stop,
        MOST_VARIABLES,
        Derive::PreferLjt,
    )
}

/// Where the derivation of a valid sequent comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Derive {
    /// Read off the search, and by LJT where a bound refuses that.
    ReadOff,
    /// By LJT alone, which the tests ask for.
    #[cfg(all(test, feature = "parse"))]
    Ljt,
    /// By LJT, and read off the search where LJT gives none.
    PreferLjt,
}

/// [`decide`] with at most `most` variables, which the tests lower, and
/// the derivation from where `derive` says. The stop is a trait object,
/// so that the search is compiled once, not once per caller's closure.
fn run(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    mut stop: &mut dyn FnMut(Progress) -> bool,
    most: u32,
    derive: Derive,
) -> Result<Outcome, Error> {
    sequent
        .formulas()
        .propositional(super::Method::Sat.title())?;
    if logic == Logic::Classical {
        return classical::decide(sequent, limits, stop, most);
    }
    if sequent.right().len() > 1 {
        return Err(Error::Succedents {
            count: sequent.right().len(),
        });
    }
    let mut formulas = sequent.formulas().clone();
    let falsity = formulas.add(Node::False)?;
    // The clauses of `a ↔ b` go through `a → b` and `b → a`, which are
    // added here. The range is fixed before the loop adds any, and an
    // implication added is no equivalence, so one pass adds them all.
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
        let found = Search::new(
            &mut solver,
            &formulas,
            &polarity,
            logic,
            falsity,
            most,
            &account,
        )
        .and_then(|mut search| {
            let found = search.run(sequent.left(), goal, &account);
            statistics.lemmas = search.lemmas.len() as u64;
            found.map(|found| (found, search.lemmas, search.nodes))
        });
        statistics.calls = solver.calls();
        statistics.conflicts = solver.conflicts();
        found
    };
    let (verdict, model) = match found {
        Err(reason) => (Verdict::Unknown(reason), None),
        Ok((Found::Worlds(worlds), _, nodes)) => {
            statistics.worlds = worlds.len() as u64;
            // The check computes at least a cell per world and formula.
            // Past its bound the model is not built, and a model the check
            // refuses for its size (its arrows count too) is dropped. Below
            // the bound the worlds' indices fit a `u32`.
            let cells = statistics
                .worlds
                .saturating_mul(sequent.formulas().len() as u64);
            if cells > super::MODEL_CELLS {
                // Too large to show and to check: the verdict stands
                // without its model, and the statistics count its worlds.
                (Verdict::NotValid, None)
            } else {
                match kripke(
                    &formulas,
                    logic,
                    (falsity, &nodes),
                    worlds,
                    &account,
                    &mut stop,
                ) {
                    Ok(model) => {
                        match super::Countermodel::found(
                            sequent.clone(),
                            model,
                            logic,
                            limits,
                            &mut stop,
                        ) {
                            Ok(model) => (Verdict::NotValid, model),
                            // The stop ended the model's check.
                            Err(error) => (Verdict::Unknown(super::decide::ended(error)?), None),
                        }
                    }
                    // The worlds' order would pass the memory bound: the
                    // verdict stands without its model.
                    Err(Reason::MemoryLimit { .. }) => (Verdict::NotValid, None),
                    Err(reason) => (Verdict::Unknown(reason), None),
                }
            }
        }
        Ok((Found::Valid, lemmas, _)) => {
            let read = if matches!(derive, Derive::ReadOff | Derive::PreferLjt) {
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
                // Only the tests ask for LJT's derivation alone: the
                // read-back's refusal stands in for a bound it met.
                Err(Error::Refused(Refusal::Output {
                    what: "derivation",
                    estimate_bytes: 0,
                    limit_bytes: 0,
                    least_bytes: None,
                }))
            };
            let derivation = match read {
                Ok((derivation, facts)) => {
                    statistics.facts = facts;
                    Ok(derivation)
                }
                Err(Error::Refused(refusal)) if !matches!(refusal, Refusal::Stopped { .. }) => {
                    by_ljt(sequent, logic, limits, &mut stop, refusal).map(|(derivation, ljt)| {
                        statistics.ljt = Some(ljt);
                        derivation
                    })
                }
                Err(error) => Err(error),
            };
            let checked = match derivation {
                Ok(derivation) => derivation
                    .check(limits, &mut stop)
                    .map_err(|e| derivation.own_fault(e))
                    .map(|()| derivation),
                Err(error) => Err(error),
            };
            match checked {
                Ok(derivation) => {
                    let derivation = if derive == Derive::PreferLjt && statistics.ljt.is_none() {
                        prefer_ljt(
                            sequent,
                            logic,
                            limits,
                            &mut stop,
                            derivation,
                            &mut statistics,
                        )?
                    } else {
                        derivation
                    };
                    (
                        Verdict::Valid(Evidence::Derivation(Box::new(derivation))),
                        None,
                    )
                }
                // A bound or the stop that ends the read-back (and LJT
                // after it) or the check leaves the sequent unknown, the
                // search's statistics kept.
                Err(error) => (Verdict::Unknown(super::decide::ended(error)?), None),
            }
        }
    };
    Ok(Outcome {
        verdict,
        logic,
        statistics,
        model,
    })
}

/// The most sequents LJT expands for a derivation the SAT method may
/// prefer to the one it read off. On ILTP's propositional problems every
/// derivation the default kept from LJT took at most 3 440 (SYJ206+1.009),
/// while LJT's search for SYJ202+1.005 expands about a million in 5 s
/// where the solver decides in 30 ms; this bound costs LJT under 0.1 s.
const PREFER_SEQUENTS: u64 = 1 << 14;

/// Returns LJT's derivation of a sequent the search proved and whose
/// `derivation` it read off and checked, with LJT's statistics in
/// `statistics`, or that derivation where LJT gives none within the
/// limits and [`PREFER_SEQUENTS`] sequents, with a drawing within
/// `limits.derivation_bytes`, and before the stop.
fn prefer_ljt(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    stop: &mut impl FnMut(Progress) -> bool,
    derivation: Derivation,
    statistics: &mut Statistics,
) -> Result<Derivation, Error> {
    let decided = ljt::decide_preferred(
        sequent,
        logic,
        limits,
        PREFER_SEQUENTS,
        limits.derivation_bytes,
        &mut *stop,
    );
    let Ok(outcome) = decided else {
        return Ok(derivation);
    };
    match ljt_answer(outcome.verdict, logic)? {
        Ok(ljt) => {
            statistics.ljt = Some(outcome.statistics);
            Ok(*ljt)
        }
        // An unknown, from a bound or the stop, leaves the derivation read
        // off the search.
        Err(_) => Ok(derivation),
    }
}

/// Returns LJT's derivation of a sequent the search proved, and LJT's
/// statistics. Where the stop ends LJT, returns that stop; where another
/// bound does, the read-back's `refusal`; where LJT refutes the sequent,
/// [`Error::ReadBack`].
fn by_ljt(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    stop: &mut impl FnMut(Progress) -> bool,
    refusal: Refusal,
) -> Result<(Derivation, ljt::Statistics), Error> {
    let outcome = match ljt::decide(sequent, logic, limits, &mut *stop) {
        Ok(outcome) => outcome,
        // A stop stays a stop; another bound is the read-back's refusal.
        Err(Error::Refused(stopped @ Refusal::Stopped { .. })) => {
            return Err(Error::Refused(stopped));
        }
        Err(Error::Refused(_)) => return Err(Error::Refused(refusal)),
        Err(other) => return Err(other),
    };
    match ljt_answer(outcome.verdict, logic)? {
        Ok(derivation) => Ok((*derivation, outcome.statistics)),
        Err(Reason::Stopped) => Err(Error::Refused(Refusal::stopped(
            crate::limits::Phase::Search,
        ))),
        Err(_) => Err(Error::Refused(refusal)),
    }
}

/// Returns what LJT says of a sequent the SAT search proved: its
/// derivation, or the reason it gave none; a refutation, or evidence that
/// is no derivation, is [`Error::ReadBack`], a defect.
fn ljt_answer(verdict: Verdict, logic: Logic) -> Result<Result<Box<Derivation>, Reason>, Error> {
    let defect = |reason: &str| Error::ReadBack {
        calculus: logic.calculus(),
        reason: reason.to_owned(),
    };
    match verdict {
        Verdict::Valid(Evidence::Derivation(derivation)) => Ok(Ok(derivation)),
        Verdict::Unknown(reason) => Ok(Err(reason)),
        Verdict::NotValid => Err(defect("LJT refutes a sequent the SAT search proved")),
        // LJT never gives a clausal proof, an image's proof or a table.
        Verdict::Valid(Evidence::Clausal(_) | Evidence::Image | Evidence::Table) => {
            Err(defect("LJT gives no derivation of a sequent it proved"))
        }
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
                // The implications may come after the equivalence, and this
                // pass has then gone by them: the parts get here what the
                // implications would give them, both polarities.
                polarity[a.index()] |= NEGATIVE | POSITIVE;
                polarity[b.index()] |= NEGATIVE | POSITIVE;
                for i in [Node::Implies(a, b), Node::Implies(b, a)] {
                    if let Some(i) = formulas.find(i) {
                        polarity[i.index()] |= p;
                    }
                }
            }
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
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
    /// Gives the solver the clauses of the formulas `polarity` names, with
    /// at most `most` variables. Each variable and clause is charged to the
    /// account as it is made, and the setup is refused once the account is
    /// over.
    fn new(
        solver: &'a mut S,
        formulas: &Formulas,
        polarity: &[u8],
        logic: Logic,
        falsity: NodeId,
        most: u32,
        account: &Account,
    ) -> Result<Self, Reason> {
        let over = || {
            account.over().then(|| Reason::MemoryLimit {
                limit_bytes: account.limit(),
            })
        };
        let mut vars = vec![None; formulas.len()];
        let mut nodes = Vec::new();
        for (id, var) in vars.iter_mut().enumerate() {
            if polarity[id] != 0 || id == falsity.index() {
                if nodes.len() >= most as usize {
                    return Err(Reason::IndexLimit);
                }
                *var = Some(solver.new_var());
                nodes.push(NodeId(id as u32));
                if let Some(reason) = over() {
                    return Err(reason);
                }
            }
        }
        let var = |x: NodeId| vars[x.index()].unwrap_or_else(|| unreachable!("a name"));
        let (pos, neg) = (|x| Lit::positive(var(x)), |x| Lit::negative(var(x)));
        let mut clauses: Vec<Vec<Lit>> = Vec::new();
        let mut implications = Vec::new();
        // The clauses of one formula at a time go to the solver, so that
        // nothing beside it holds them all.
        let mut flush = |clauses: &mut Vec<Vec<Lit>>| {
            for clause in clauses.drain(..) {
                solver.add_clause(&clause);
            }
            over()
        };
        if logic == Logic::Intuitionistic {
            clauses.push(vec![neg(falsity)]);
        }
        for (id, &p) in polarity.iter().enumerate() {
            if let Some(reason) = flush(&mut clauses) {
                return Err(reason);
            }
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
                Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            }
        }
        if let Some(reason) = flush(&mut clauses) {
            return Err(reason);
        }
        Ok(Self {
            solver,
            nodes,
            vars,
            implications,
            lemmas: Vec::new(),
        })
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
            match learnt {
                Ok(true) => account.release(held),
                // The worlds stay charged while the model is made of them.
                Ok(false) => return Ok(Found::Worlds(worlds)),
                Err(reason) => {
                    account.release(held);
                    return Err(reason);
                }
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
        // added, so each world keeps a cursor over the implications. The
        // oldest world is examined first: newest first leaves ILTP's
        // SYJ212+1.020 undecided after 20 s, which this order refutes in
        // 130 calls.
        let mut next: Vec<usize> = vec![0; worlds.len()];
        let mut scanned = 0u64;
        loop {
            let mut pair = None;
            'scan: for (w, at) in next.iter_mut().enumerate() {
                while let Some(&[a, b, c]) = self.implications.get(*at) {
                    scanned += 1;
                    if scanned.is_multiple_of(POLL)
                        && let Some(reason) = self.solver.poll(POLL)
                    {
                        return Err(reason);
                    }
                    if unrealised(worlds, w, [a, b, c]) {
                        pair = Some((w, [a, b, c]));
                        break 'scan;
                    }
                    *at += 1;
                }
            }
            let Some((w, [a, b, c])) = pair else {
                return Ok(false);
            };
            // `a` and `¬b` first: the solver blames a conflict on the first
            // assumptions that imply it. So what `a` implies is not blamed
            // on the world, whose variables would tie the clause learnt to
            // that one world.
            let mut assumptions = vec![Lit::positive(a), Lit::negative(b)];
            assumptions.extend(worlds[w].members().map(Lit::positive));
            match self.solver.solve(&assumptions) {
                Answer::Satisfiable => {
                    let world = self.world();
                    *held += world.bytes();
                    account.charge(world.bytes());
                    worlds.push(world);
                    next.push(0);
                    if account.over() {
                        return Err(Reason::MemoryLimit {
                            limit_bytes: account.limit(),
                        });
                    }
                    next[w] += 1;
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
    // every variable of `w`.
    !worlds
        .iter()
        .any(|other| other.has(a) && !other.has(b) && world.within(other))
}

/// Returns the Kripke model of the worlds: ordered by inclusion, the
/// least first, each with the atoms whose variables it holds, and `⊥`
/// where it holds `⊥` in minimal logic. Finding the order takes a bit per
/// pair of worlds, charged to the account. Returns the reason past the
/// memory bound, or when the stop fires (asked every `POLL` comparisons
/// of worlds).
fn kripke(
    formulas: &Formulas,
    logic: Logic,
    (falsity, nodes): (NodeId, &[NodeId]),
    mut worlds: Vec<Bits>,
    account: &Account,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<Model, Reason> {
    // A world strictly above another has more variables.
    worlds.sort_by_key(|w| w.members().count());
    let n = worlds.len();
    let bytes = n.saturating_mul(Bits::new(n).bytes());
    account.charge(bytes);
    if account.over() {
        account.release(bytes);
        return Err(Reason::MemoryLimit {
            limit_bytes: account.limit(),
        });
    }
    let mut compared = 0u64;
    let mut poll = |by: usize| {
        let before = compared;
        compared = compared.saturating_add(by as u64);
        if before / POLL != compared / POLL
            && stop(Progress::new(crate::limits::Phase::Search, POLL, compared))
        {
            return Err(Reason::Stopped);
        }
        Ok(())
    };
    let found = (|| -> Result<Vec<Vec<u32>>, Reason> {
        // Every world strictly below each, as bits over the worlds.
        let mut below: Vec<Bits> = Vec::with_capacity(n);
        for j in 0..n {
            let mut set = Bits::new(n);
            for i in 0..j {
                poll(1)?;
                if worlds[i] != worlds[j] && worlds[i].within(&worlds[j]) {
                    set.insert(i as u32);
                }
            }
            below.push(set);
        }
        // The worlds directly below each: below it, and below no other
        // world below it.
        let mut above = vec![Vec::new(); n];
        for (j, set) in below.iter().enumerate() {
            let mut direct = set.clone();
            for i in set.members() {
                poll(1)?;
                for (word, under) in direct.0.iter_mut().zip(&below[i as usize].0) {
                    *word &= !under;
                }
            }
            for i in direct.members() {
                above[i as usize].push(j as u32);
            }
        }
        Ok(above)
    })();
    account.release(bytes);
    let above = found?;
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
                Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            }
        }
        atoms.sort_unstable();
        let names = formulas.atom_names();
        World::new(
            atoms.iter().map(|&a| names[a as usize].clone()).collect(),
            forces_falsity,
            above,
        )
    });
    Ok(Model::new(models.collect(), 0))
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
                    Verdict::Valid(evidence) => {
                        let derivation = evidence.derivation().unwrap();
                        assert_eq!(derivation.check(&limits, |_| false), Ok(()), "{sequent}");
                        assert_eq!(outcome.statistics.ljt, None, "{sequent}: read off");
                        proved += 1;
                        true
                    }
                    Verdict::NotValid => {
                        let model = &outcome.model.as_ref().unwrap().model;
                        assert_eq!(
                            model.refutes(&sequent, logic, &Limits::default(), |_| false),
                            Ok(())
                        );
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
        // Classical logic is decided too, by the clauses of the negation.
        assert!(valid(&sat("|- a \\/ ~a", Logic::Classical)));
    }

    /// A model changed in any of the ways the check guards against is
    /// refused, each for its own reason.
    #[test]
    fn a_planted_wrong_model_is_refused() {
        let text = "((a -> b) -> a) -> a";
        let sequent: Sequent = text.parse().unwrap();
        let logic = Logic::Intuitionistic;
        let model = sat(text, logic).model.unwrap().model;
        assert_eq!(
            model.refutes(&sequent, logic, &Limits::default(), |_| false),
            Ok(())
        );
        assert!(model.worlds.len() >= 2, "{model:?}");
        let refused = |model: &Model, why: &str| {
            let Err(Error::Model { reason }) =
                model.refutes(&sequent, logic, &Limits::default(), |_| false)
            else {
                panic!("{why}: {model:?} is accepted");
            };
            assert!(reason.contains(why), "{reason} for {why}");
        };
        let both = || vec!["a".to_owned(), "b".to_owned()];
        // The root's conclusion made true everywhere.
        let mut planted = model.clone();
        for world in &mut planted.worlds {
            world.atoms = both();
        }
        refused(&planted, "forces the conclusion");
        // A world above that forgets an atom below it.
        let mut planted = model.clone();
        let up = planted.worlds[0].arrows[0] as usize;
        planted.worlds[0].atoms.push("a".to_owned());
        planted.worlds[up].atoms.clear();
        refused(&planted, "does not force all it forces");
        // The arrows taken away: the root alone forces the formula.
        let mut planted = model.clone();
        planted.worlds.truncate(1);
        planted.worlds[0].arrows.clear();
        refused(&planted, "forces the conclusion");
        // An arrow back, an arrow to no world, ⊥ forced, the model's world
        // missing.
        let mut planted = model.clone();
        let last = planted.worlds.len() - 1;
        planted.worlds[last].arrows.push(last as u32);
        refused(&planted, "the arrows go round");
        let mut planted = model.clone();
        planted.worlds[0].arrows.push(99);
        refused(&planted, "which the model does not have");
        let mut planted = model.clone();
        planted.worlds[0].falsity = true;
        refused(&planted, "forces ⊥ in intuitionistic logic");
        let mut planted = model;
        planted.world = 99;
        refused(&planted, "is not one of its");
    }

    /// The search stops at the work bound, the stop and the memory bound,
    /// and refuses a sequent of more formulas than it has variables. A
    /// derivation past its bound is refused when LJT gives none either.
    /// The model's check refuses past its cells.
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
        let outcome = run(
            &sequent,
            logic,
            &Limits::default(),
            &mut |_| false,
            5,
            Derive::ReadOff,
        )
        .unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::IndexLimit)
        ));
        assert!(matches!(
            run(
                &sequent,
                logic,
                &Limits::default(),
                &mut |_| false,
                6,
                Derive::ReadOff
            )
            .unwrap()
            .verdict,
            Verdict::NotValid
        ));
        // The bound on a drawing's size bounds no derivation that is built.
        let sequent: Sequent = "a, b, c |- a /\\ (b /\\ c)".parse().unwrap();
        let small = Limits::default().with_derivation_bytes(Some(100));
        let outcome = decide(&sequent, logic, &small, |_| false).unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)));
        // A model of more cells than the check computes.
        let chain: Vec<String> = (0..1 << 14).map(|i| format!("a{i}")).collect();
        let sequent: Sequent = format!("|- {}", chain.join(" /\\ ")).parse().unwrap();
        let worlds = (super::super::MODEL_CELLS as usize).div_ceil(sequent.formulas().len()) + 1;
        let model = Model::new(vec![World::default(); worlds], 0);
        assert!(matches!(
            model.refutes(&sequent, logic, &Limits::default(), |_| false),
            Err(Error::Refused(Refusal::Memory { .. }))
        ));
    }

    /// The stop is asked while the worlds are scanned for an unrealised
    /// pair, which takes no call of the solver: a goal of `k` implications
    /// in a disjunction is refuted by `k + 1` worlds, whose last scan
    /// examines every pair.
    #[test]
    fn the_scan_of_the_worlds_asks_the_stop() {
        let k = 64;
        let chain: Vec<String> = (0..k).map(|i| format!("(a{i} -> a{})", i + 1)).collect();
        let sequent: Sequent = format!("|- {}", chain.join(" \\/ ")).parse().unwrap();
        let logic = Logic::Intuitionistic;
        let mut scans = 0;
        let free = decide(&sequent, logic, &Limits::default(), |p| {
            scans += u64::from(p.work == POLL);
            false
        })
        .unwrap();
        assert_eq!(free.statistics.worlds, k + 1);
        assert!(scans >= k * k / POLL, "{scans}");
        let mut asked = 0;
        let outcome = decide(&sequent, logic, &Limits::default(), |p| {
            asked += u64::from(p.work == POLL);
            asked == scans
        })
        .unwrap();
        assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
    }

    /// The order of a model's worlds is found within the memory bound and
    /// until the stop fires: a chain of worlds, each a variable more.
    #[test]
    fn the_model_s_order_keeps_to_its_bounds() {
        let sequent: Sequent = "|- a".parse().unwrap();
        let formulas = sequent.formulas();
        let n = 1000;
        let chain = || {
            (0..n)
                .map(|k| {
                    let mut world = Bits::new(n);
                    (0..k).for_each(|v| world.insert(v as u32));
                    world
                })
                .collect::<Vec<_>>()
        };
        let nodes = vec![NodeId(0); n];
        let free = Account::holding(None, 0);
        let model = kripke(
            formulas,
            Logic::Intuitionistic,
            (NodeId(0), &nodes),
            chain(),
            &free,
            &mut |_| false,
        );
        let model = model.unwrap();
        assert_eq!(model.worlds[0].arrows, vec![1]);
        assert_eq!(free.used(), 0);
        let small = Account::holding(Some(n as u64 * 8), 0);
        let refused = kripke(
            formulas,
            Logic::Intuitionistic,
            (NodeId(0), &nodes),
            chain(),
            &small,
            &mut |_| false,
        );
        assert!(matches!(refused, Err(Reason::MemoryLimit { .. })));
        let stopped = kripke(
            formulas,
            Logic::Intuitionistic,
            (NodeId(0), &nodes),
            chain(),
            &free,
            &mut |_| true,
        );
        assert!(matches!(stopped, Err(Reason::Stopped)));
    }

    /// Returns ILTP's SYJ202+1.`n` as its file writes it: `n + 1` pigeons
    /// in `n` holes, each pigeon in some hole, so some hole holds two;
    /// every disjunction nested to the right, as the file brackets it (the
    /// reader nests a flat one to the left, a sequent LJT searches longer).
    fn pigeons_tptp(n: usize) -> String {
        /// The disjunction of `terms`, nested to the right.
        fn right(terms: &[String]) -> String {
            match terms {
                [last] => last.clone(),
                [first, rest @ ..] => format!("( {first} | {} )", right(rest)),
                [] => unreachable!("a hole and a pigeon at least"),
            }
        }
        let mut text = String::new();
        for i in 1..=n + 1 {
            let holes: Vec<String> = (1..=n).map(|h| format!("o{i}{h}")).collect();
            text.push_str(&format!("fof(axiom{i},axiom,( {} )).\n", right(&holes)));
        }
        let mut pairs = Vec::new();
        for h in 1..=n {
            for i in 1..=n + 1 {
                for j in i + 1..=n + 1 {
                    pairs.push(format!("( o{i}{h} & o{j}{h} )"));
                }
            }
        }
        text.push_str(&format!("fof(con,conjecture,( {} )).\n", right(&pairs)));
        text
    }

    /// Where the solver decides at once and LJT's search would run long,
    /// the default gives LJT up after `PREFER_SEQUENTS` and keeps the
    /// derivation read off: ILTP's SYJ202+1.005, which LJT takes about a
    /// million sequents for. The stop's polls count the work done.
    #[test]
    fn ljt_is_given_up_where_its_search_runs_long() {
        let problem = crate::ordinary::read_tptp(&pigeons_tptp(5), &Limits::default()).unwrap();
        // The searches' polls: the solver's and LJT's, which asks every
        // `ljt::POLL` sequents and every `ljt::POLL` hypotheses, so its
        // million sequents would be tens of thousands (the read-back and
        // the check ask once per inference).
        let mut searched = 0_u64;
        let outcome = decide_preferring_ljt(
            &problem.sequent,
            Logic::Intuitionistic,
            &Limits::default(),
            |p| {
                searched += u64::from(p.phase == crate::limits::Phase::Search);
                false
            },
        )
        .unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)), "{outcome:?}");
        assert_eq!(outcome.statistics.ljt, None);
        assert!(searched < 2_000, "{searched} polls of the searches");
    }

    /// A stop that fires after the search decided, in the read-back or
    /// the check, leaves the sequent unknown with the search's statistics,
    /// as a stop during the search does.
    #[test]
    fn a_stop_after_the_search_is_an_unknown() {
        let sequent: Sequent = "a -> b, b -> c |- a -> c".parse().unwrap();
        let outcome = decide(&sequent, Logic::Intuitionistic, &Limits::default(), |p| {
            p.phase != crate::limits::Phase::Search
        })
        .unwrap();
        assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
        assert!(outcome.statistics.calls > 0, "{:?}", outcome.statistics);
    }

    /// The default prefers LJT's derivation only where its drawing fits
    /// `limits.derivation_bytes`, and keeps the one read off otherwise.
    #[test]
    fn ljt_is_preferred_where_its_drawing_fits() {
        let sequent: Sequent = "a, a -> b, b -> c |- c".parse().unwrap();
        let logic = Logic::Intuitionistic;
        let roomy = decide_preferring_ljt(&sequent, logic, &Limits::default(), |_| false);
        assert!(roomy.unwrap().statistics.ljt.is_some());
        let small = Limits::default().with_derivation_bytes(Some(10));
        let outcome = decide_preferring_ljt(&sequent, logic, &small, |_| false).unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)), "{outcome:?}");
        assert_eq!(outcome.statistics.ljt, None);
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
            &mut |_| false,
            MOST_VARIABLES,
            Derive::Ljt,
        );
        let outcome = outcome.unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)), "{outcome:?}");
        assert!(outcome.statistics.ljt.is_some(), "{:?}", outcome.statistics);
        // The stop during LJT is the stop, an unknown, not the read-back's
        // refusal: it fires at the first poll of the search after the
        // solver's.
        let search = |p: &Progress| p.phase == crate::limits::Phase::Search;
        let mut solver = 0;
        let mut counted = |p: Progress| {
            solver += u64::from(search(&p));
            false
        };
        run(
            &sequent,
            logic,
            &Limits::default(),
            &mut counted,
            MOST_VARIABLES,
            Derive::ReadOff,
        )
        .unwrap();
        let mut asked = 0;
        let mut ljt = |p: Progress| {
            asked += u64::from(search(&p));
            asked > solver
        };
        let stopped = run(
            &sequent,
            logic,
            &Limits::default(),
            &mut ljt,
            MOST_VARIABLES,
            Derive::Ljt,
        );
        assert!(
            matches!(
                stopped.as_ref().map(|outcome| outcome.verdict.clone()),
                Ok(Verdict::Unknown(Reason::Stopped))
            ),
            "{stopped:?}"
        );
    }
}
