// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A search for a finite structure that refutes a first-order sequent,
//! in the manner of MACE and Paradox: for domains of 1, 2, … elements up
//! to a bound, the sequent's falsity in a structure of that size is
//! written as a propositional formula, and the SAT solver looks for a
//! model of it.
//!
//! A sequent `Γ ⊢ Δ` fails in a structure exactly when `⋀Γ → ⋁Δ` is false
//! there. Over the domain `{0, …, n−1}` the search takes that formula in
//! negation normal form, miniscoped (grounding's normal form, function
//! symbols kept), and writes `∀x. A` as the conjunction of `A` at every
//! element and `∃x. A` as the disjunction: no Skolem function is needed.
//! A function symbol `f` gets an atom `f(d̄) ↦ e` for each argument tuple
//! `d̄` the formula reaches and each value `e`, a constant `c ↦ e`; the
//! hypotheses say each such term has exactly one value (Paradox's
//! totality and functionality), and a term denotes `e` where its
//! arguments denote some `ē` and `f(ē) ↦ e` holds. A model of the
//! hypotheses that falsifies the formula is a structure of `n` elements
//! that refutes the sequent. Clauses after Paradox's put the constants in
//! the order they first name elements, which loses no structure up to
//! isomorphism.
//!
//! The sources: W. McCune, *MACE 2.0 reference manual and guide*
//! (arXiv cs/0106042) and *Mace4 reference manual and guide* (arXiv
//! cs/0310055); G. Reger, M. Suda and A. Voronkov, *Finding finite models
//! in multi-sorted first-order logic* (SAT 2016, arXiv 1604.08040), whose
//! §3 describes K. Claessen and N. Sörensson's Paradox.
//!
//! The search is untrusted. The structure read off the solver's model is
//! checked by [`FiniteCountermodel::check`] against the sequent as
//! written before it is an answer: one that fails is
//! [`Error::Countermodel`], a defect. A classical countermodel is a Kripke
//! model of one world, so "not valid" holds in every logic. No domain
//! without a model proves anything: past the bound the answer is unknown
//! ([`Reason::ModelSize`]).

use super::grounding::{self, G, Nnf, Term};
use super::kripke_search;
use super::kripke_structure::KripkeCountermodel;
use super::sat;
use super::structure::{FiniteCountermodel, Structure};
use super::{Formulas, Logic, Node, NodeId, Sequent, Verdict};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::search::{self, Reason};

/// No index.
const NONE: u32 = u32::MAX;

/// The bytes a formula of the expansion's arena holds, with its share of
/// the arena's index: grounding's figure.
const NODE_BYTES: u64 = grounding::NODE_BYTES;

/// The bytes an atom of the expansion holds beside 4 bytes an element of
/// its tuple: its entry, its key in the index, and its name.
const ATOM_BYTES: u64 = grounding::ATOM_BYTES;

/// The bytes an entry of a memo holds beside 4 bytes a value: its key's
/// vector and its slot in the table.
const MEMO_BYTES: u64 = grounding::MEMO_BYTES;

/// What the finite model search cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct Statistics {
    /// The largest domain tried, in elements.
    pub size: u64,
    /// The most worlds of the last Kripke model tried, in intuitionistic
    /// and minimal logic; 0 where only finite structures were tried (and
    /// then left out of the JSON form).
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "is_zero"))]
    pub worlds: u64,
    /// The atoms of the last domain's formula: the predicates' and the
    /// terms' values.
    pub atoms: u64,
    /// The formulas of the last domain's arena.
    pub nodes: u64,
    /// What the SAT solver's search on the last domain cost.
    pub sat: sat::Statistics,
}

/// Whether a count is zero, which the JSON form leaves out.
#[cfg(feature = "serialize")]
#[cfg_attr(
    target_pointer_width = "64",
    expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "serde's skip_serializing_if passes a reference"
    )
)]
const fn is_zero(count: &u64) -> bool {
    *count == 0
}

/// What the finite model search found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict: not valid, or unknown (past the bound, at another
    /// bound, or where the sequent names a term no structure interprets).
    /// Never valid.
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// The finite structure that refutes a sequent that is not valid,
    /// checked by [`FiniteCountermodel::check`].
    pub structure: Option<FiniteCountermodel>,
    /// The Kripke model with growing domains that refutes a sequent that is
    /// not valid in intuitionistic or minimal logic where no finite
    /// structure does, checked by [`KripkeCountermodel::check`].
    pub kripke: Option<KripkeCountermodel>,
    /// What the search cost.
    pub statistics: Statistics,
}

/// Looks for a finite structure that refutes `sequent`, on domains of 1,
/// 2, … elements up to `options.model_size`, and in intuitionistic and
/// minimal logic for a Kripke model with growing domains beside it, of
/// up to `options.model_worlds` worlds, within `limits`, asking `stop` as
/// it goes. For L = 1, 2, … it tries the structures of L elements, then
/// the Kripke models of L worlds (or the bound) of up to L elements a
/// world (or the bound): a model fits every larger pair.
///
/// The verdict is not valid with a checked [`Structure`] (in every logic:
/// a classical countermodel is a Kripke model of one world) or a checked
/// [`KripkeStructure`](super::KripkeStructure), or unknown:
/// [`Reason::ModelSize`] past the bound in classical logic and
/// [`Reason::KripkeSize`] past both in the others, the reason of the
/// bound or the stop that ended the search, and [`Reason::Incomplete`] for
/// a sequent over a derivation's arena that names an eigenvariable or an
/// open witness, which no structure interprets. Each domain is written within
/// `limits.occurrences` formulas and `limits.memory_bytes`, and the
/// solver gets `limits.work` on each.
///
/// # Errors
///
/// [`Error::Succedents`] for more than one formula right of `⊢` outside
/// classical logic; [`Error::Countermodel`] where the structure read off
/// does not refute the sequent, a defect.
pub fn decide(
    sequent: &Sequent,
    logic: Logic,
    options: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    // A sequent with `=` is searched with the equality axioms, which make
    // a structure's `=` a congruence that it is divided by.
    let bounds = (options.model_size, options.model_worlds);
    match super::equality::with_axioms(sequent, limits)? {
        Some(with) => run(&with, logic, bounds, limits, &mut stop),
        None => run(sequent, logic, bounds, limits, &mut stop),
    }
}

/// [`decide`] with the bounds alone and the stop as a trait object.
fn run(
    sequent: &Sequent,
    logic: Logic,
    (bound, worlds_bound): (Option<u32>, Option<u32>),
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    if logic != Logic::Classical && sequent.right().len() > 1 {
        return Err(Error::Succedents {
            count: sequent.right().len(),
        });
    }
    let mut statistics = Statistics::default();
    let unknown = |reason, statistics| {
        Ok(Outcome {
            verdict: Verdict::Unknown(reason),
            logic,
            structure: None,
            kripke: None,
            statistics,
        })
    };
    let (nnf, root) = match grounding::normal_form(sequent, limits, &mut *stop, true) {
        Ok(Some(prepared)) => prepared,
        Ok(None) => return unknown(Reason::Incomplete, statistics),
        Err(error) => return unknown(unknown_for(error, limits)?, statistics),
    };
    let signature = Signature::of(&nnf, root);
    let most = bound.unwrap_or(u32::MAX);
    // A bound of no element leaves nothing to try.
    if most == 0 {
        let reason = match logic {
            Logic::Classical => Reason::ModelSize { size: 0 },
            Logic::Intuitionistic | Logic::Minimal => Reason::KripkeSize {
                worlds: worlds_bound.unwrap_or(u32::MAX),
                size: 0,
            },
        };
        return unknown(reason, statistics);
    }
    let worlds = match logic {
        Logic::Classical => 1,
        Logic::Intuitionistic | Logic::Minimal => worlds_bound.unwrap_or(u32::MAX).max(1),
    };
    // Minimal logic's `⊥` makes a model of one world more than a finite
    // structure, which forces it nowhere.
    // With `=` no Kripke model is tried: a model's worlds read `=` as an
    // atom, and "not valid" with `=` stands only on a structure that
    // reads it as identity.
    let least = if sequent.formulas().has_equality() {
        u32::MAX
    } else {
        match logic {
            Logic::Minimal => 1,
            Logic::Classical | Logic::Intuitionistic => 2,
        }
    };
    for level in 1..=most.max(worlds) {
        // A Kripke model, on a pair not tried yet.
        let (k, n) = (level.min(worlds), level.min(most));
        if k >= least && level > most {
            match kripke(sequent, logic, (k, n), &mut statistics, limits, &mut *stop) {
                Ok(Some(outcome)) => return Ok(outcome),
                Ok(None) => continue,
                Err(Ok(reason)) => return unknown(reason, statistics),
                Err(Err(error)) => return Err(error),
            }
        }
        if level > most {
            continue;
        }
        let size = level;
        statistics.size = u64::from(size);
        let expansion = match Expansion::new(&nnf, root, &signature, size, limits, &mut *stop) {
            Ok(expansion) => expansion,
            Err(error) => return unknown(unknown_for(error, limits)?, statistics),
        };
        statistics.atoms = expansion.entries.len() as u64;
        statistics.nodes = expansion.sequent.formulas().len() as u64;
        let rest = limits.with_memory_bytes(
            limits
                .memory_bytes
                .map(|m| m.saturating_sub(expansion.bytes)),
        );
        let (found, solved) = sat::falsify(&expansion.sequent, &rest, &mut *stop);
        statistics.sat = solved;
        let true_atoms = match found {
            Ok(Some(atoms)) => atoms,
            Ok(None) if k >= least => {
                match kripke(sequent, logic, (k, n), &mut statistics, limits, &mut *stop) {
                    Ok(Some(outcome)) => return Ok(outcome),
                    Ok(None) => continue,
                    Err(Ok(reason)) => return unknown(reason, statistics),
                    Err(Err(error)) => return Err(error),
                }
            }
            Ok(None) => continue,
            Err(Reason::MemoryLimit { limit_bytes }) => {
                let limit_bytes = limits.memory_bytes.unwrap_or(limit_bytes);
                return unknown(Reason::MemoryLimit { limit_bytes }, statistics);
            }
            Err(reason) => return unknown(reason, statistics),
        };
        let structure = match expansion.structure(sequent, &signature, &true_atoms, limits) {
            Ok(structure) => structure,
            Err(error) => return unknown(unknown_for(error, limits)?, statistics),
        };
        drop(expansion);
        // The structure satisfies the equality axioms, so its `=` is a
        // congruence; divided by it, `=` is identity, as a structure
        // reads it.
        let structure = if sequent.formulas().has_equality() {
            super::equality::identified(structure)
        } else {
            structure
        };
        let countermodel = FiniteCountermodel::new(sequent.clone(), structure);
        return match countermodel.check(logic, limits, &mut *stop) {
            Ok(()) => Ok(Outcome {
                verdict: Verdict::NotValid,
                logic,
                structure: Some(countermodel),
                kripke: None,
                statistics,
            }),
            // The verdict rests on the structure alone, so a check that a
            // bound or the stop gives up leaves the sequent unknown.
            Err(Error::Structure { reason }) => Err(Error::Countermodel {
                reason: format!("the finite structure the model search found is wrong: {reason}"),
            }),
            Err(error) => unknown(unknown_for(error, limits)?, statistics),
        };
    }
    let reason = if logic != Logic::Classical {
        Reason::KripkeSize { worlds, size: most }
    } else {
        Reason::ModelSize { size: most }
    };
    unknown(reason, statistics)
}

/// Looks for a Kripke model of `worlds` worlds with domains of at most
/// `size` elements that refutes the sequent, and checks the one found:
/// the outcome where one refutes it, `None` where none does, the reason
/// (`Err(Ok(…))`) where a bound or the stop ends the search, and an error
/// (`Err(Err(…))`) for a model that fails its check, a defect, or another
/// refusal.
fn kripke(
    sequent: &Sequent,
    logic: Logic,
    (worlds, size): (u32, u32),
    statistics: &mut Statistics,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Option<Outcome>, Result<Reason, Error>> {
    statistics.worlds = u64::from(worlds);
    statistics.size = u64::from(size);
    let written = kripke_search::write(sequent, logic, worlds, size, limits, &mut *stop)
        .map_err(|error| unknown_for(error, limits))?;
    statistics.atoms = written.atoms();
    statistics.nodes = written.sequent.formulas().len() as u64;
    let rest =
        limits.with_memory_bytes(limits.memory_bytes.map(|m| m.saturating_sub(written.bytes)));
    let (found, solved) = sat::falsify(&written.sequent, &rest, &mut *stop);
    statistics.sat = solved;
    let true_atoms = match found {
        Ok(Some(atoms)) => atoms,
        Ok(None) => return Ok(None),
        Err(Reason::MemoryLimit { limit_bytes }) => {
            let limit_bytes = limits.memory_bytes.unwrap_or(limit_bytes);
            return Err(Ok(Reason::MemoryLimit { limit_bytes }));
        }
        Err(reason) => return Err(Ok(reason)),
    };
    let model = written
        .model(sequent, &true_atoms, limits)
        .map_err(|error| unknown_for(error, limits))?;
    drop(written);
    let countermodel = KripkeCountermodel::new(sequent.clone(), model);
    match countermodel.check(logic, limits, &mut *stop) {
        Ok(()) => Ok(Some(Outcome {
            verdict: Verdict::NotValid,
            logic,
            structure: None,
            kripke: Some(countermodel),
            statistics: *statistics,
        })),
        Err(Error::Structure { reason }) => Err(Err(Error::Countermodel {
            reason: format!("the Kripke model the model search found is wrong: {reason}"),
        })),
        Err(error) => Err(unknown_for(error, limits)),
    }
}

/// Returns the reason the stop or a bound on work or memory leaves the
/// search unknown for, and any other error as it is (a domain whose
/// formula passes `limits.occurrences` among them).
fn unknown_for(error: Error, limits: &Limits) -> Result<Reason, Error> {
    let Error::Refused(refusal) = &error else {
        return Err(error);
    };
    if let Refusal::Stopped { .. } = refusal {
        return Ok(Reason::Stopped);
    }
    if let Refusal::Memory { limit_bytes, .. } = refusal {
        return Ok(Reason::MemoryLimit {
            limit_bytes: limits.memory_bytes.unwrap_or(*limit_bytes),
        });
    }
    if let Refusal::Work { limit } = refusal {
        return Ok(Reason::WorkLimit { limit: *limit });
    }
    Err(error)
}

/// The constants and function symbols the normal form names.
struct Signature {
    /// The constants, by their symbols' indices, in the order the normal
    /// form first names them.
    constants: Vec<u32>,
    /// The function symbols with arguments, each with its arity, by first
    /// use.
    functions: Vec<(u32, u32)>,
}

impl Signature {
    /// Collects the signature of the nodes reachable from `root`, in the
    /// order of a walk from the root, operands left to right.
    fn of(nnf: &Nnf, root: u32) -> Self {
        let mut constants = Vec::new();
        let mut functions: Vec<(u32, u32)> = Vec::new();
        let mut seen_function: crate::hash::HashSet<u32> = crate::hash::HashSet::default();
        let mut seen_node = vec![false; nnf.nodes.len()];
        let mut seen_app = vec![false; nnf.apps.len()];
        let mut seen_constant: crate::hash::HashSet<u32> = crate::hash::HashSet::default();
        let mut stack = vec![root];
        // The terms met, walked with a stack of their own.
        let mut terms: Vec<Term> = Vec::new();
        while let Some(n) = stack.pop() {
            if std::mem::replace(&mut seen_node[n as usize], true) {
                continue;
            }
            match &nnf.nodes[n as usize] {
                G::Lit(_, _, args) => terms.extend(args.iter().rev()),
                G::True | G::False => {}
                G::And(k) | G::Or(k) => stack.extend(k.iter().rev()),
                G::Forall(_, b) | G::Exists(_, b) => stack.push(*b),
            }
            while let Some(t) = terms.pop() {
                match t {
                    Term::Var(_) => {}
                    Term::Const(c) => {
                        if seen_constant.insert(c) {
                            constants.push(c);
                        }
                    }
                    Term::App(a) => {
                        if std::mem::replace(&mut seen_app[a as usize], true) {
                            continue;
                        }
                        let app = &nnf.apps[a as usize];
                        if seen_function.insert(app.symbol) {
                            functions.push((app.symbol, app.args.len() as u32));
                        }
                        terms.extend(app.args.iter().rev());
                    }
                }
            }
        }
        Self {
            constants,
            functions,
        }
    }
}

/// An atom of the expansion.
enum Entry {
    /// A predicate, by its index (the symbol's in a first-order arena, the
    /// atom's in a propositional one), holds of a tuple.
    Holds(u32, Vec<u32>),
    /// A principal term, by its index in [`Expansion::principals`], has
    /// this value.
    Value(u32, u32),
}

/// A term's value as the expansion writes it.
enum Denotes {
    /// An element, fixed by the bindings.
    Element(u32),
    /// For each element, the formula that says the term denotes it.
    Table(Vec<NodeId>),
}

/// The formula that a structure of `size` elements refutes the sequent,
/// with what reads a structure off its model.
struct Expansion {
    /// `Ax ⊢ F`: the totality, functionality and symmetry formulas left,
    /// the expanded formula right.
    sequent: Sequent,
    /// Each atom, by the number its name `m_N` carries.
    entries: Vec<Entry>,
    /// Each principal term: its symbol, its arguments, and its value
    /// atoms, one per element.
    principals: Vec<(u32, Vec<u32>, Vec<NodeId>)>,
    /// The bytes the expansion held at its end, by its estimate: the SAT
    /// solver gets the memory bound less these.
    bytes: u64,
    /// The domain's size.
    size: u32,
}

/// The state of an expansion while it is written.
struct Writer<'n> {
    /// The normal form.
    nnf: &'n Nnf,
    /// The domain's size.
    size: u32,
    /// The arena written.
    formulas: Formulas,
    /// Each atom made, by its number.
    entries: Vec<Entry>,
    /// The predicates' atoms, by predicate and tuple.
    holds: HashMap<(u32, Vec<u32>), NodeId>,
    /// The principal terms: symbol, arguments, value atoms.
    principals: Vec<(u32, Vec<u32>, Vec<NodeId>)>,
    /// Each principal term's index, by symbol and arguments.
    principal_of: HashMap<(u32, Vec<u32>), u32>,
    /// Each application's formulas, by its index and the values of its
    /// free variables.
    denotes: HashMap<(u32, Vec<u32>), Vec<NodeId>>,
    /// The bytes the atoms and memos hold beside the arena.
    held: u64,
    /// The most formulas, `limits.occurrences`.
    most: u64,
    /// The memory bound.
    memory: u64,
    /// The steps taken, for the stop.
    steps: u64,
}

impl Writer<'_> {
    /// Refuses past the bounds and asks the stop every few steps.
    fn tick(&mut self, stop: &mut dyn FnMut(Progress) -> bool) -> Result<(), Error> {
        self.tick_by(1, stop)
    }

    /// [`tick`](Self::tick) for a step that costs `weight` units.
    fn tick_by(
        &mut self,
        weight: usize,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        grounding::poll_by(&mut self.steps, weight, stop)?;
        if self.formulas.len() as u64 > self.most {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: self.most.saturating_add(1),
                limit: self.most,
            }));
        }
        let held = (self.formulas.len() as u64)
            .saturating_mul(NODE_BYTES)
            .saturating_add(self.held);
        if held > self.memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: self.memory,
                needed_bytes: None,
            }));
        }
        Ok(())
    }

    /// Makes an atom of the entry and returns it.
    fn atom(&mut self, entry: Entry) -> Result<NodeId, Error> {
        let number = self.entries.len();
        let id = self.formulas.atom(&format!("m_{number}"))?;
        let tuple = match &entry {
            Entry::Holds(_, tuple) => tuple.len() as u64,
            Entry::Value(..) => 1,
        };
        self.held = self
            .held
            .saturating_add(ATOM_BYTES)
            .saturating_add(4 * tuple);
        self.entries.push(entry);
        Ok(id)
    }

    /// Returns the predicate's atom on the tuple, made once.
    fn holds(&mut self, predicate: u32, tuple: &[u32]) -> Result<NodeId, Error> {
        if let Some(&id) = self.holds.get(&(predicate, tuple.to_vec())) {
            return Ok(id);
        }
        let id = self.atom(Entry::Holds(predicate, tuple.to_vec()))?;
        self.holds.insert((predicate, tuple.to_vec()), id);
        Ok(id)
    }

    /// Returns the value atoms of the principal term `symbol(args)`, one
    /// per element, made once.
    fn values(&mut self, symbol: u32, args: &[u32]) -> Result<Vec<NodeId>, Error> {
        if let Some(&p) = self.principal_of.get(&(symbol, args.to_vec())) {
            return Ok(self.principals[p as usize].2.clone());
        }
        let p = self.principals.len() as u32;
        let mut atoms = Vec::with_capacity(self.size as usize);
        for e in 0..self.size {
            atoms.push(self.atom(Entry::Value(p, e))?);
        }
        self.held = self
            .held
            .saturating_add(MEMO_BYTES)
            .saturating_add(4 * args.len() as u64);
        self.principal_of.insert((symbol, args.to_vec()), p);
        self.principals.push((symbol, args.to_vec(), atoms.clone()));
        Ok(atoms)
    }

    /// Returns the conjunction (`and`) or the disjunction of the formulas.
    fn junction(&mut self, and: bool, parts: &[NodeId]) -> Result<NodeId, Error> {
        let Some((&first, rest)) = parts.split_first() else {
            return self
                .formulas
                .add(if and { Node::True } else { Node::False });
        };
        let mut acc = first;
        for &p in rest {
            acc = self.formulas.add(if and {
                Node::And(acc, p)
            } else {
                Node::Or(acc, p)
            })?;
        }
        Ok(acc)
    }

    /// Returns, for the tuples the arguments may denote, the disjunction
    /// over them of "the arguments denote the tuple and `make(tuple)`":
    /// each fixed argument's element, each other's choice of element.
    fn over_tuples(
        &mut self,
        args: &[Denotes],
        stop: &mut dyn FnMut(Progress) -> bool,
        mut make: impl FnMut(&mut Self, &[u32]) -> Result<NodeId, Error>,
    ) -> Result<NodeId, Error> {
        let open: Vec<usize> = (0..args.len())
            .filter(|&i| matches!(args[i], Denotes::Table(_)))
            .collect();
        let mut tuple: Vec<u32> = args
            .iter()
            .map(|a| match a {
                Denotes::Element(e) => *e,
                Denotes::Table(_) => 0,
            })
            .collect();
        if open.is_empty() {
            return make(self, &tuple);
        }
        // An odometer over the open arguments' elements.
        let mut choice = vec![0u32; open.len()];
        let mut disjuncts = Vec::new();
        loop {
            self.tick(stop)?;
            let mut conjuncts = Vec::with_capacity(open.len() + 1);
            for (k, &i) in open.iter().enumerate() {
                tuple[i] = choice[k];
                if let Denotes::Table(table) = &args[i] {
                    conjuncts.push(table[choice[k] as usize]);
                }
            }
            conjuncts.push(make(self, &tuple)?);
            disjuncts.push(self.junction(true, &conjuncts)?);
            self.held = self.held.saturating_add(4);
            let mut k = 0;
            loop {
                if k == choice.len() {
                    return self.junction(false, &disjuncts);
                }
                choice[k] += 1;
                if choice[k] < self.size {
                    break;
                }
                choice[k] = 0;
                k += 1;
            }
        }
    }

    /// Returns what the term denotes under the bindings `at`.
    fn term(
        &mut self,
        t: Term,
        bindings: &[(u32, u32, u32)],
        at: u32,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Denotes, Error> {
        let a = match t {
            Term::Var(x) => return Ok(Denotes::Element(lookup(bindings, at, x))),
            Term::Const(c) => return Ok(Denotes::Table(self.values(c, &[])?)),
            Term::App(a) => a,
        };
        let nnf = self.nnf;
        let key = |b: u32| -> (u32, Vec<u32>) {
            let values = nnf.apps[b as usize]
                .free
                .iter()
                .map(|&v| lookup(bindings, at, v))
                .collect();
            (b, values)
        };
        if let Some(table) = self.denotes.get(&key(a)) {
            return Ok(Denotes::Table(table.clone()));
        }
        // The applications under `a` not written yet: an argument's index
        // is below its application's, so ascending order is bottom-up.
        let mut todo: Vec<u32> = Vec::new();
        let mut stack = vec![a];
        let mut seen: crate::hash::HashSet<u32> = crate::hash::HashSet::default();
        while let Some(b) = stack.pop() {
            if !seen.insert(b) || self.denotes.contains_key(&key(b)) {
                continue;
            }
            todo.push(b);
            for arg in &nnf.apps[b as usize].args {
                if let Term::App(c) = *arg {
                    stack.push(c);
                }
            }
        }
        todo.sort_unstable();
        for b in todo {
            let app = &nnf.apps[b as usize];
            let mut args = Vec::with_capacity(app.args.len());
            for &arg in &app.args {
                args.push(match arg {
                    Term::Var(x) => Denotes::Element(lookup(bindings, at, x)),
                    Term::Const(c) => Denotes::Table(self.values(c, &[])?),
                    Term::App(c) => Denotes::Table(self.denotes[&key(c)].clone()),
                });
            }
            let symbol = app.symbol;
            let mut table = Vec::with_capacity(self.size as usize);
            for e in 0..self.size {
                let formula = self.over_tuples(&args, stop, |w, tuple| {
                    Ok(w.values(symbol, tuple)?[e as usize])
                })?;
                table.push(formula);
            }
            let k = key(b);
            self.held = self
                .held
                .saturating_add(MEMO_BYTES)
                .saturating_add(4 * (k.1.len() as u64 + u64::from(self.size)));
            self.denotes.insert(k, table);
        }
        Ok(Denotes::Table(self.denotes[&key(a)].clone()))
    }
}

/// Returns the element bound to `x` in the bindings from `at` back.
fn lookup(bindings: &[(u32, u32, u32)], mut at: u32, x: u32) -> u32 {
    while at != NONE {
        let (y, e, before) = bindings[at as usize];
        if y == x {
            return e;
        }
        at = before;
    }
    unreachable!("every variable is bound in a closed formula")
}

impl Expansion {
    /// Writes the formula that a structure of `size` elements refutes the
    /// sequent whose normal form is `nnf`, within `limits`.
    fn new(
        nnf: &Nnf,
        root: u32,
        signature: &Signature,
        size: u32,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        let mut w = Writer {
            nnf,
            size,
            formulas: Formulas::default(),
            entries: Vec::new(),
            holds: HashMap::default(),
            principals: Vec::new(),
            principal_of: HashMap::default(),
            denotes: HashMap::default(),
            held: 0,
            most: limits.occurrences.unwrap_or(u64::MAX),
            memory: limits.memory_bytes.unwrap_or(u64::MAX),
            steps: 0,
        };
        // Every constant's value atoms first, in the signature's order,
        // which the symmetry formulas read.
        let mut constant_values = Vec::with_capacity(signature.constants.len());
        for &c in &signature.constants {
            constant_values.push(w.values(c, &[])?);
        }
        let formula = expand(&mut w, root, stop)?;
        let mut left = Vec::new();
        // Totality and functionality: each principal term has exactly one
        // value.
        for p in 0..w.principals.len() {
            let atoms = w.principals[p].2.clone();
            left.push(w.junction(false, &atoms)?);
            for (i, &a) in atoms.iter().enumerate() {
                for &b in &atoms[i + 1..] {
                    w.tick(stop)?;
                    let not_a = w.formulas.add(Node::Not(a))?;
                    let not_b = w.formulas.add(Node::Not(b))?;
                    left.push(w.formulas.add(Node::Or(not_a, not_b))?);
                }
            }
        }
        // Symmetry: the i-th constant (from 0) takes one of the first
        // i + 1 elements, and an element m > 0 only where an earlier
        // constant takes m − 1.
        for (i, values) in constant_values.iter().enumerate() {
            for (m, &value) in values.iter().enumerate() {
                w.tick(stop)?;
                if m > i {
                    left.push(w.formulas.add(Node::Not(value))?);
                } else if m > 0 {
                    let mut parts = vec![w.formulas.add(Node::Not(value))?];
                    parts.extend(constant_values[..i].iter().map(|v| v[m - 1]));
                    left.push(w.junction(false, &parts)?);
                }
            }
        }
        w.tick(stop)?;
        let bytes = (w.formulas.len() as u64)
            .saturating_mul(NODE_BYTES)
            .saturating_add(w.held);
        Ok(Self {
            sequent: Sequent::new(w.formulas, left, vec![formula])?,
            entries: w.entries,
            principals: w.principals,
            bytes,
            size,
        })
    }

    /// Returns the structure the atoms `true_atoms` (indices among the
    /// expansion's atom names) give, its function tables charged to
    /// `limits.memory_bytes`.
    fn structure(
        &self,
        sequent: &Sequent,
        signature: &Signature,
        true_atoms: &[u32],
        limits: &Limits,
    ) -> Result<Structure, Error> {
        let size = self.size;
        let names = self.sequent.formulas().atom_names();
        let mut value = vec![0u32; self.principals.len()];
        let mut holds: HashMap<u32, Vec<Vec<u32>>> = HashMap::default();
        for &a in true_atoms {
            let Some(number) = names[a as usize]
                .strip_prefix("m_")
                .and_then(|n| n.parse::<usize>().ok())
            else {
                continue;
            };
            match &self.entries[number] {
                Entry::Holds(predicate, tuple) => {
                    holds.entry(*predicate).or_default().push(tuple.clone());
                }
                Entry::Value(p, e) => value[*p as usize] = *e,
            }
        }
        let tables = sequent.formulas().first_order();
        let formulas = sequent.formulas();
        let symbol = |s: u32| tables.symbols[s as usize].0.clone();
        let mut of_constant: HashMap<u32, u32> = HashMap::default();
        let mut cells: u64 = 0;
        let memory = limits.memory_bytes.unwrap_or(u64::MAX);
        let mut functions: Vec<(String, Vec<u32>)> = Vec::new();
        let mut index_of: HashMap<u32, usize> = HashMap::default();
        // A table past what the memory bound, the address space or the
        // allocator holds is the memory bound's refusal, never an abort.
        let refused = |needed: u64| {
            Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: memory,
                needed_bytes: Some(needed),
            })
        };
        for &(f, arity) in &signature.functions {
            let count = u64::from(size)
                .checked_pow(arity)
                .filter(|&c| c <= isize::MAX as u64 / 4)
                .ok_or_else(|| refused(u64::MAX))?;
            cells = cells.saturating_add(count);
            if cells.saturating_mul(4) > memory {
                return Err(refused(cells.saturating_mul(4)));
            }
            let mut table: Vec<u32> = Vec::new();
            table
                .try_reserve_exact(count as usize)
                .map_err(|_| refused(count.saturating_mul(4)))?;
            table.resize(count as usize, 0);
            index_of.insert(f, functions.len());
            functions.push((symbol(f), table));
        }
        for (p, (s, args, _)) in self.principals.iter().enumerate() {
            if args.is_empty() {
                of_constant.insert(*s, value[p]);
                continue;
            }
            let Some(&at) = index_of.get(s) else {
                continue;
            };
            let cell = args
                .iter()
                .fold(0usize, |i, &e| i * size as usize + e as usize);
            functions[at].1[cell] = value[p];
        }
        let constants = signature
            .constants
            .iter()
            .map(|&c| (symbol(c), of_constant.get(&c).copied().unwrap_or(0)))
            .collect();
        let mut predicates: Vec<(String, Vec<Vec<u32>>)> = holds
            .into_iter()
            .map(|(p, mut tuples)| {
                tuples.sort_unstable();
                let name = if tables.atoms.is_empty() {
                    formulas.atom_name(p).to_owned()
                } else {
                    symbol(p)
                };
                (name, tuples)
            })
            .collect();
        predicates.sort_unstable();
        Ok(Structure::new(size, constants, predicates).with_functions(functions))
    }
}

/// Writes the normal form from `root` over the writer's domain: each
/// node under each value of its free variables once.
fn expand(
    w: &mut Writer<'_>,
    root: u32,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<NodeId, Error> {
    let nnf = w.nnf;
    let size = w.size;
    /// A step of the walk.
    enum Task {
        /// Write the node under the bindings, from the memo where it was
        /// written before.
        Enter(u32, u32),
        /// Write the node under the bindings.
        Expand(u32, u32),
        /// Join the last `count` results by `∧` (`and`) or `∨`.
        Join(bool, usize),
        /// Remember the last result as the node's under these values of
        /// its free variables.
        Store(u32, Vec<u32>),
    }
    let mut bindings: Vec<(u32, u32, u32)> = Vec::new();
    let mut memo: HashMap<(u32, Vec<u32>), NodeId> = HashMap::default();
    let mut results: Vec<NodeId> = Vec::new();
    let mut tasks = vec![Task::Enter(root, NONE)];
    while let Some(task) = tasks.pop() {
        // A node's step looks up its free variables' values, one walk of
        // the bindings each: counted as one unit, a nest of thousands of
        // quantifiers let the stop wait seconds past a time limit.
        let weight = match &task {
            Task::Enter(n, _) | Task::Expand(n, _) => nnf.free[*n as usize].len(),
            Task::Join(_, count) => *count,
            Task::Store(_, values) => values.len(),
        };
        w.tick_by(weight, stop)?;
        if tasks.len() as u64 > w.most {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: w.most.saturating_add(1),
                limit: w.most,
            }));
        }
        let beside = (tasks.capacity() as u64)
            .saturating_mul(40)
            .saturating_add((bindings.capacity() as u64).saturating_mul(12))
            .saturating_add((results.capacity() as u64).saturating_mul(4));
        if beside.saturating_add(w.held) > w.memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: w.memory,
                needed_bytes: None,
            }));
        }
        match task {
            Task::Store(n, values) => {
                let made = *results.last().expect("the node's formula");
                w.held = w
                    .held
                    .saturating_add(MEMO_BYTES)
                    .saturating_add(4 * values.len() as u64);
                memo.insert((n, values), made);
            }
            Task::Enter(n, at)
                if !matches!(nnf.nodes[n as usize], G::Lit(..) | G::True | G::False) =>
            {
                let values: Vec<u32> = nnf.free[n as usize]
                    .iter()
                    .map(|&v| lookup(&bindings, at, v))
                    .collect();
                match memo.get(&(n, values.clone())) {
                    Some(&made) => results.push(made),
                    None => {
                        tasks.push(Task::Store(n, values));
                        tasks.push(Task::Expand(n, at));
                    }
                }
            }
            Task::Enter(n, at) | Task::Expand(n, at) => match &nnf.nodes[n as usize] {
                G::Lit(positive, predicate, args) => {
                    let mut denotes = Vec::with_capacity(args.len());
                    for &t in args {
                        denotes.push(w.term(t, &bindings, at, stop)?);
                    }
                    let predicate = *predicate;
                    let formula =
                        w.over_tuples(&denotes, stop, |w, tuple| w.holds(predicate, tuple))?;
                    results.push(if *positive {
                        formula
                    } else {
                        w.formulas.add(Node::Not(formula))?
                    });
                }
                G::True => results.push(w.formulas.add(Node::True)?),
                G::False => results.push(w.formulas.add(Node::False)?),
                G::And(k) | G::Or(k) => {
                    let and = matches!(nnf.nodes[n as usize], G::And(_));
                    tasks.push(Task::Join(and, k.len()));
                    for &c in k.iter().rev() {
                        tasks.push(Task::Enter(c, at));
                    }
                }
                G::Forall(x, b) | G::Exists(x, b) => {
                    let and = matches!(nnf.nodes[n as usize], G::Forall(..));
                    tasks.push(Task::Join(and, size as usize));
                    for e in (0..size).rev() {
                        bindings.push((*x, e, at));
                        tasks.push(Task::Enter(*b, (bindings.len() - 1) as u32));
                    }
                }
            },
            Task::Join(and, count) => {
                let start = results.len() - count;
                let parts = results.split_off(start);
                let joined = w.junction(and, &parts)?;
                results.push(joined);
            }
        }
    }
    Ok(results.pop().expect("the expanded formula"))
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::decide;
    use crate::Limits;
    use crate::linear::search::generate::Rng;
    use crate::linear::search::{self, Reason};
    use crate::ordinary::{Evaluation, Logic, Sequent, Verdict};

    /// Searches `text` in `logic` within the default limits and a bound
    /// of `size`.
    fn search(text: &str, logic: Logic, size: u32) -> super::Outcome {
        let sequent: Sequent = text.parse().unwrap();
        let options = search::Options::default().with_model_size(Some(size));
        decide(&sequent, logic, &options, &Limits::default(), |_| false)
            .unwrap_or_else(|e| panic!("{text}: {e}"))
    }

    /// Sequents decided by hand: those that a finite structure refutes get
    /// one of the size expected, checked, in every logic; the valid ones,
    /// and one that only infinite structures refute, are unknown past the
    /// bound.
    #[test]
    fn sequents_decided_by_hand() {
        let refuted = [
            ("p(a) |- p(f(a))", 2),
            ("exists x. p(x) |- forall x. p(x)", 2),
            (
                "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)",
                2,
            ),
            ("forall x. r(x, f(x)) |- exists x. r(f(x), x)", 3),
            ("forall x. p(f(x)) |- p(a)", 2),
            ("forall x. p(g(x, x)) |- p(g(a, b))", 2),
            ("forall x. p(g(x, f(x))) |- forall x. p(x)", 2),
            ("p(a), ~p(b), ~p(c) |- p(f(b)) \\/ p(f(c)) \\/ ~p(f(a))", 2),
            ("forall x. (p(x) -> p(f(x))) |- forall x. p(x)", 1),
        ];
        for (text, size) in refuted {
            for &logic in Logic::ALL {
                let outcome = search(text, logic, 4);
                assert!(
                    matches!(outcome.verdict, Verdict::NotValid),
                    "{text}: {:?}",
                    outcome.verdict
                );
                let countermodel = outcome.structure.expect("a structure");
                assert_eq!(countermodel.structure.size, size, "{text}");
                assert_eq!(outcome.statistics.size, u64::from(size));
                let sequent: Sequent = text.parse().unwrap();
                assert_eq!(
                    countermodel
                        .structure
                        .refutes(&sequent, &Limits::default(), |_| false)
                        .unwrap(),
                    Evaluation::Refutes,
                    "{text}"
                );
            }
        }
        let unknown = [
            "forall x. p(x) |- p(f(a))",
            "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))",
            "|- exists x. (p(x) -> forall y. p(y))",
            // Only an infinite structure refutes it: a strict order with
            // no greatest element.
            "forall x. ~r(x, x), forall x y z. (r(x, y) /\\ r(y, z) -> r(x, z)), \
             forall x. exists y. r(x, y) |-",
        ];
        for text in unknown {
            let outcome = search(text, Logic::Classical, 3);
            assert!(
                matches!(
                    outcome.verdict,
                    Verdict::Unknown(Reason::ModelSize { size: 3 })
                ),
                "{text}: {:?}",
                outcome.verdict
            );
        }
        // A table no allocation holds is the memory bound's refusal, an
        // unknown, with or without a bound.
        let wide = format!(
            "p(a), ~p(b), forall x. r(g({})) |- r(a)",
            vec!["x"; 70].join(", ")
        );
        for memory in [None, Some(1 << 26)] {
            let sequent: Sequent = wide.parse().unwrap();
            let limits = Limits::default().with_memory_bytes(memory);
            let options = search::Options::default().with_model_size(Some(3));
            let outcome = decide(&sequent, Logic::Classical, &options, &limits, |_| false).unwrap();
            assert!(
                matches!(
                    outcome.verdict,
                    Verdict::Unknown(Reason::MemoryLimit { .. })
                ),
                "{:?}",
                outcome.verdict
            );
        }
    }

    /// The default of a first-order sequent the provers and the embedding
    /// leave open asks the model search last, in every logic, and the
    /// method named runs alone.
    #[test]
    fn the_default_asks_the_model_search_last() {
        use crate::ordinary::{Method, Options, Search};
        let sequent: Sequent = "forall x. (p(x) -> p(f(x))), p(a) |- p(b)".parse().unwrap();
        for &logic in Logic::ALL {
            let outcome = crate::ordinary::decide(&sequent, &Options::default().with_logic(logic))
                .unwrap_or_else(|e| panic!("{logic}: {e}"));
            assert!(matches!(outcome.verdict, Verdict::NotValid), "{logic}");
            assert!(matches!(outcome.search, Search::Models(_)), "{logic}");
            assert!(outcome.structure.is_some());
        }
        // A theorem keeps the prover's derivation.
        let valid: Sequent = "forall x. p(x) |- p(f(a))".parse().unwrap();
        let outcome = crate::ordinary::decide(&valid, &Options::default()).unwrap();
        assert!(matches!(outcome.verdict, Verdict::Valid(_)));
        let named = Options::default().with_method(Some(Method::Models));
        let outcome = crate::ordinary::decide(&valid, &named.with_logic(Logic::Classical)).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::ModelSize { size: 6 })
        ));
        let outcome =
            crate::ordinary::decide(&valid, &named.with_logic(Logic::Intuitionistic)).unwrap();
        assert!(matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::KripkeSize { worlds: 4, size: 6 })
        ));
    }

    /// Searches `text` in `logic` with bounds on elements and worlds.
    fn kripke(text: &str, logic: Logic, size: u32, worlds: u32) -> super::Outcome {
        let sequent: Sequent = text.parse().unwrap();
        let options = search::Options::default()
            .with_model_size(Some(size))
            .with_model_worlds(Some(worlds));
        decide(&sequent, logic, &options, &Limits::default(), |_| false)
            .unwrap_or_else(|e| panic!("{text}: {e}"))
    }

    /// Sequents valid classically and not intuitionistically get a checked
    /// Kripke model of at most the worlds and elements expected; minimal logic's
    /// `⊥` is forced where it must be; the double-negation shift and the
    /// double negation of excluded middle at first order, which only
    /// infinite models refute, are unknown at the bounds.
    #[test]
    fn kripke_models_decided_by_hand() {
        let refuted = [
            ("|- p(a) \\/ ~p(a)", Logic::Intuitionistic, 2, 1),
            ("|- forall x. (p(x) \\/ ~p(x))", Logic::Intuitionistic, 2, 2),
            ("~~p(a) |- p(a)", Logic::Intuitionistic, 2, 1),
            ("forall x. ~~p(x) |- p(f(a))", Logic::Intuitionistic, 2, 1),
            (
                "(forall x. p(x)) -> q |- exists x. (p(x) -> q)",
                Logic::Intuitionistic,
                2,
                2,
            ),
            ("false |- p(a)", Logic::Minimal, 1, 1),
            ("|- ~p(a) \\/ ~~p(a)", Logic::Minimal, 3, 3),
        ];
        for (text, logic, worlds, size) in refuted {
            let outcome = kripke(text, logic, 4, 3);
            assert!(
                matches!(outcome.verdict, Verdict::NotValid),
                "{text}: {:?}",
                outcome.verdict
            );
            let countermodel = outcome.kripke.expect("a Kripke model");
            assert!(countermodel.model.worlds.len() <= worlds, "{text}");
            let most = countermodel.model.worlds.iter().map(|w| w.size).max();
            assert!(most <= Some(size), "{text}");
            countermodel
                .check(logic, &Limits::default(), |_| false)
                .unwrap();
            if logic == Logic::Intuitionistic {
                countermodel
                    .check(Logic::Minimal, &Limits::default(), |_| false)
                    .unwrap();
            }
        }
        for text in [
            "forall x. ~~p(x) |- ~~forall x. p(x)",
            "|- ~~forall x. (p(x) \\/ ~p(x))",
        ] {
            let outcome = kripke(text, Logic::Intuitionistic, 3, 3);
            assert!(
                matches!(
                    outcome.verdict,
                    Verdict::Unknown(Reason::KripkeSize { worlds: 3, size: 3 })
                ),
                "{text}: {:?}",
                outcome.verdict
            );
        }
    }

    /// The stop ends the search as unknown, and a sequent with two goals
    /// outside classical logic is refused.
    #[test]
    fn the_stop_and_the_goals() {
        let sequent: Sequent = "forall x. p(x) |- p(f(a))".parse().unwrap();
        let options = search::Options::default().with_model_size(None);
        let outcome = decide(
            &sequent,
            Logic::Classical,
            &options,
            &Limits::default(),
            |p| p.done > 10_000,
        )
        .unwrap();
        assert!(matches!(outcome.verdict, Verdict::Unknown(Reason::Stopped)));
        // A bound of no element is unknown at once, in every logic.
        let none = search::Options::default().with_model_size(Some(0));
        for &logic in Logic::ALL {
            let outcome = decide(&sequent, logic, &none, &Limits::default(), |_| false).unwrap();
            assert!(matches!(
                outcome.verdict,
                Verdict::Unknown(
                    Reason::ModelSize { size: 0 } | Reason::KripkeSize { size: 0, .. }
                )
            ));
        }
        let two: Sequent = "|- p(a), p(b)".parse().unwrap();
        assert!(
            decide(
                &two,
                Logic::Intuitionistic,
                &options,
                &Limits::default(),
                |_| false
            )
            .is_err()
        );
    }

    /// Returns a closed term over `a`, `b`, `f/1`, `g/2` and the bound
    /// variables in scope.
    fn term(rng: &mut Rng, depth: usize, scope: &[String]) -> String {
        if depth == 0 || rng.one_in(2) {
            return match rng.below(scope.len() + 2) {
                0 => "a".to_owned(),
                1 => "b".to_owned(),
                k => scope[k - 2].clone(),
            };
        }
        if rng.one_in(2) {
            format!("f({})", term(rng, depth - 1, scope))
        } else {
            format!(
                "g({}, {})",
                term(rng, depth - 1, scope),
                term(rng, depth - 1, scope)
            )
        }
    }

    /// Returns a formula over `p/1`, `q/2`, `r/0` and those terms.
    fn formula(rng: &mut Rng, depth: usize, scope: &mut Vec<String>) -> String {
        if depth == 0 || rng.one_in(4) {
            return match rng.below(5) {
                0 => "r".to_owned(),
                1 | 2 => format!("p({})", term(rng, 2, scope)),
                _ => format!("q({}, {})", term(rng, 1, scope), term(rng, 1, scope)),
            };
        }
        match rng.below(7) {
            0 => format!("~{}", formula(rng, depth - 1, scope)),
            1 | 2 if scope.len() < 2 => {
                let name = ["x", "y"][scope.len()].to_owned();
                let quantifier = if rng.one_in(2) { "forall" } else { "exists" };
                scope.push(name.clone());
                let body = formula(rng, depth - 1, scope);
                scope.pop();
                format!("({quantifier} {name}. {body})")
            }
            k => {
                let a = formula(rng, depth - 1, scope);
                let b = formula(rng, depth - 1, scope);
                let op = ["/\\", "\\/", "->", "<->"][k % 4];
                format!("({a} {op} {b})")
            }
        }
    }

    /// On generated sequents with function symbols, the model search never
    /// refutes what the connection prover proves, and every structure it
    /// gives refutes its sequent.
    #[test]
    fn the_model_search_agrees_with_the_connection_prover() {
        let mut rng = Rng::new(7_601);
        let limits = Limits {
            work: Some(1 << 16),
            ..Limits::default()
        };
        let options = search::Options::default().with_model_size(Some(3));
        let (mut refuted, mut proved) = (0, 0);
        for _ in 0..300 {
            let hypotheses: Vec<String> = (0..rng.below(3))
                .map(|_| formula(&mut rng, 3, &mut Vec::new()))
                .collect();
            let goal = formula(&mut rng, 3, &mut Vec::new());
            let text = format!("{} |- {goal}", hypotheses.join(", "));
            let sequent: Sequent = text.parse().unwrap();
            let ours = decide(&sequent, Logic::Classical, &options, &limits, |_| false)
                .unwrap_or_else(|e| panic!("{text}: {e}"));
            let theirs = crate::ordinary::connection::decide(
                &sequent,
                Logic::Classical,
                &search::Options::default(),
                &limits,
                |_| false,
            );
            let valid = matches!(theirs.map(|o| o.verdict), Ok(Verdict::Valid(_)));
            proved += usize::from(valid);
            if matches!(ours.verdict, Verdict::NotValid) {
                assert!(!valid, "{text}");
                refuted += 1;
                let countermodel = ours.structure.expect("a structure");
                // The structure as a Kripke model of one world says the same.
                assert_eq!(
                    crate::ordinary::KripkeStructure::from(countermodel.structure.clone())
                        .refutes(&sequent, Logic::Classical, &limits, |_| false)
                        .unwrap(),
                    Evaluation::Refutes,
                    "{text}"
                );
                assert_eq!(
                    countermodel
                        .structure
                        .refutes(&sequent, &limits, |_| false)
                        .unwrap(),
                    Evaluation::Refutes,
                    "{text}"
                );
            }
        }
        assert!(refuted > 200 && proved > 10, "{refuted} {proved}");
    }

    /// On generated intuitionistic sequents with function symbols, the
    /// model search never refutes what the embedding proves, and every
    /// Kripke model it gives refutes its sequent.
    #[test]
    fn kripke_models_agree_with_the_embedding() {
        use crate::ordinary::{Method, Options};
        let mut rng = Rng::new(7_602);
        let limits = Limits {
            work: Some(1 << 16),
            ..Limits::default()
        };
        let options = search::Options::default()
            .with_model_size(Some(2))
            .with_model_worlds(Some(3));
        let (mut kripke, mut proved) = (0, 0);
        for _ in 0..200 {
            let hypotheses: Vec<String> = (0..rng.below(3))
                .map(|_| formula(&mut rng, 3, &mut Vec::new()))
                .collect();
            let goal = formula(&mut rng, 3, &mut Vec::new());
            // Half the goals are classically valid: excluded middle or
            // double negation elimination of a generated formula.
            let goal = match rng.below(4) {
                0 => format!("({goal}) \\/ ~({goal})"),
                1 => format!("~~({goal}) -> ({goal})"),
                _ => goal,
            };
            let text = format!("{} |- {goal}", hypotheses.join(", "));
            let sequent: Sequent = text.parse().unwrap();
            let ours = decide(&sequent, Logic::Intuitionistic, &options, &limits, |_| {
                false
            })
            .unwrap_or_else(|e| panic!("{text}: {e}"));
            let embedded = crate::ordinary::decide_within(
                &sequent,
                &Options::default()
                    .with_logic(Logic::Intuitionistic)
                    .with_method(Some(Method::Embedding)),
                &search::Options::default(),
                &limits,
                |_| false,
            );
            let valid = matches!(embedded.map(|o| o.verdict), Ok(Verdict::Valid(_)));
            proved += usize::from(valid);
            if matches!(ours.verdict, Verdict::NotValid) {
                assert!(!valid, "{text}");
            }
            if let Some(countermodel) = ours.kripke {
                kripke += 1;
                countermodel
                    .check(Logic::Intuitionistic, &limits, |_| false)
                    .unwrap_or_else(|e| panic!("{text}: {e}"));
            }
        }
        assert!(kripke > 50 && proved > 5, "{kripke} {proved}");
    }
}
