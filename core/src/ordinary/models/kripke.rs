// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The propositional sequent that a Kripke model of K worlds with domains
//! of at most N elements refutes a first-order sequent, for the SAT
//! solver, and the model read off its answer.
//!
//! The worlds are `0 .. K`, world 0 the root. The order is the solver's:
//! an atom `le(v, u)` for `v < u`, transitive, world 0 below every world;
//! every finite poset with a least element can be numbered so that `v ≤ u`
//! implies `v ≤ u` as numbers, so no shape of frame is lost. A world's
//! domain is a prefix `0 .. sᵥ`: atoms `dom(v, d)`, `dom(v, 0)` true,
//! `dom(v, d + 1) → dom(v, d)`, and growing along the order. Predicates
//! are atoms per world and tuple, growing; a function symbol's value at a
//! world for a tuple is an atom per value, exactly one on the world's
//! domain, the value in it, agreeing along the order on the smaller
//! domain's tuples; a constant has one element for every world, in the
//! root's domain (Paradox's symmetry formulas on the constants only
//! permute the root's domain, which every domain holds). In minimal logic
//! `⊥` is an atom per world, growing.
//!
//! Forcing is written by its definition over the original formula (an
//! intuitionistic formula has no negation normal form): `Φ(v, A, ē)` per
//! world, subformula and values of its variables, memoised; `A → B`, `¬A`
//! and `∀x. A` at `v` are conjunctions over the worlds `u ≥ v` (as
//! numbers) guarded by `le(v, u)` (and `dom(u, d)`), `∃x. A` a disjunction
//! over `dom(v, d)`. The root forces the hypotheses and not the formula
//! right of `⊢`. The model read off is the caller's to check.

use super::encode::{Core, Denotes, Writes, over_tuples};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::{KripkeStructure, KripkeWorld, Logic, Node, NodeId, Sequent};

/// No index.
const NONE: u32 = u32::MAX;

/// The bytes a formula of the written arena holds, with its share of the
/// arena's index.
const NODE_BYTES: u64 = super::grounding::NODE_BYTES;

/// The bytes a memo entry holds beside 4 bytes a value of its key.
const MEMO_BYTES: u64 = super::grounding::MEMO_BYTES;

/// An atom of the written sequent.
enum Entry {
    /// `le(v, u)`: world `v` lies below world `u`.
    Below(u32, u32),
    /// `dom(v, d)`: world `v`'s domain holds `d`.
    Domain(u32, u32),
    /// World `v` forces `⊥` (minimal logic).
    Falsity(u32),
    /// A predicate, by its index in the sequent's tables (the atom's in a
    /// propositional arena), holds at the world of the tuple.
    Holds(u32, u32, Vec<u32>),
    /// The principal term, by its index in [`Writer::principals`], has the
    /// value at the world.
    Value(u32, u32, u32),
    /// The constant, by its symbol's index, denotes the element.
    Constant(u32, u32),
}

/// The written sequent and what reads a model off its answer.
pub(super) struct Written {
    /// The sequent: the frame's, domains', growth and terms' formulas and
    /// the root's forcing of the hypotheses left, the root's forcing of
    /// the formula right of `⊢` right.
    pub(super) sequent: Sequent,
    /// Each atom, by the number its name `k_N` carries.
    entries: Vec<Entry>,
    /// Each principal term: its symbol and its arguments.
    principals: Vec<(u32, Vec<u32>)>,
    /// The constants, by their symbols' indices.
    constants: Vec<u32>,
    /// The function symbols the sequent applies, with their arities.
    functions: Vec<(u32, u32)>,
    /// The worlds.
    worlds: u32,
    /// The most elements a domain has.
    size: u32,
    /// The bytes the writing held at its end, by its estimate.
    pub(super) bytes: u64,
}

/// The state of the writing.
struct Writer<'s> {
    /// The sequent refuted.
    sequent: &'s Sequent,
    /// Whether `⊥` is an atom per world.
    minimal: bool,
    /// The worlds.
    worlds: u32,
    /// The most elements a domain has.
    size: u32,
    /// The arena written, with its bounds.
    core: Core,
    /// Each atom made, by its number.
    entries: Vec<Entry>,
    /// The hypotheses: every formula the model must make true.
    axioms: Vec<NodeId>,
    /// `le(v, u)` for `v < u`, by `v * worlds + u`; `⊤` on the diagonal.
    below: Vec<NodeId>,
    /// `dom(v, d)`, by `v * size + d`.
    domain: Vec<NodeId>,
    /// `⊥` at each world: an atom in minimal logic, `⊥` otherwise.
    falsity: Vec<NodeId>,
    /// Each predicate's atoms at every world, by the predicate and tuple.
    holds: HashMap<(u32, Vec<u32>), Vec<NodeId>>,
    /// Each principal term's index, by its symbol and arguments.
    principal_of: HashMap<(u32, Vec<u32>), u32>,
    /// Each principal term: its symbol, its arguments and its value atoms
    /// by `world * size + value`.
    principals: Vec<(u32, Vec<u32>, Vec<NodeId>)>,
    /// Each constant's atoms, by element.
    constants: HashMap<u32, Vec<NodeId>>,
    /// The constants in the order they were met.
    constant_order: Vec<u32>,
    /// The function symbols met, with their arities.
    functions: Vec<(u32, u32)>,
    /// What each application denotes, by world, term and the values of the
    /// variables it holds.
    denotes: HashMap<(u32, u32, Vec<u32>), Vec<NodeId>>,
    /// How many innermost bound variables each term reads.
    term_needs: Vec<u32>,
    /// How many innermost bound variables each formula reads.
    needs: Vec<u32>,
}

impl Writes for Writer<'_> {
    fn core(&mut self) -> &mut Core {
        &mut self.core
    }

    fn size(&self) -> u32 {
        self.size
    }
}

/// Returns the element bound to de Bruijn index `index` in the bindings
/// from `at` back.
fn lookup(bindings: &[(u32, u32)], mut at: u32, index: u32) -> u32 {
    for _ in 0..index {
        at = bindings[at as usize].1;
    }
    bindings[at as usize].0
}

/// Returns the `count` innermost bound values from `at` back, innermost
/// first.
fn innermost(bindings: &[(u32, u32)], mut at: u32, count: u32) -> Vec<u32> {
    let mut values = Vec::with_capacity(count as usize);
    for _ in 0..count {
        values.push(bindings[at as usize].0);
        at = bindings[at as usize].1;
    }
    values
}

impl Writer<'_> {
    /// Refuses past the bounds and asks the stop every few steps.
    fn tick(&mut self, stop: &mut dyn FnMut(Progress) -> bool) -> Result<(), Error> {
        self.core.tick_by(1, stop)
    }

    /// Makes an atom of the entry and returns it.
    fn atom(&mut self, entry: Entry) -> Result<NodeId, Error> {
        let key = match &entry {
            Entry::Holds(_, _, tuple) => tuple.len() as u64,
            Entry::Below(..) | Entry::Domain(..) | Entry::Falsity(_) | Entry::Value(..) => 2,
            Entry::Constant(..) => 1,
        };
        let id = self.core.atom("k", self.entries.len(), key)?;
        self.entries.push(entry);
        Ok(id)
    }

    /// Returns `¬a`.
    fn not(&mut self, a: NodeId) -> Result<NodeId, Error> {
        self.core.formulas.add(Node::Not(a))
    }

    /// Returns `a → b`.
    fn implies(&mut self, a: NodeId, b: NodeId) -> Result<NodeId, Error> {
        self.core.formulas.add(Node::Implies(a, b))
    }

    /// Returns `le(v, u)` for `v ≤ u`.
    fn le(&self, v: u32, u: u32) -> NodeId {
        self.below[(v * self.worlds + u) as usize]
    }

    /// Returns `dom(v, d)`.
    fn dom(&self, v: u32, d: u32) -> NodeId {
        self.domain[(v * self.size + d) as usize]
    }

    /// Returns the predicate's atom at world `w` on the tuple, made at
    /// every world with the formulas that it grows.
    fn holds(&mut self, predicate: u32, w: u32, tuple: &[u32]) -> Result<NodeId, Error> {
        if let Some(atoms) = self.holds.get(&(predicate, tuple.to_vec())) {
            return Ok(atoms[w as usize]);
        }
        let mut atoms = Vec::with_capacity(self.worlds as usize);
        for v in 0..self.worlds {
            atoms.push(self.atom(Entry::Holds(predicate, v, tuple.to_vec()))?);
        }
        for v in 0..self.worlds {
            for u in v + 1..self.worlds {
                let both = self
                    .core
                    .junction(true, &[self.le(v, u), atoms[v as usize]])?;
                let grows = self.implies(both, atoms[u as usize])?;
                self.axioms.push(grows);
            }
        }
        self.core.held = self.core.held.saturating_add(MEMO_BYTES);
        self.holds
            .insert((predicate, tuple.to_vec()), atoms.clone());
        Ok(atoms[w as usize])
    }

    /// Returns a constant's atoms, one per element, made once with the
    /// formulas that it has exactly one element, in the root's domain.
    fn constant(&mut self, c: u32) -> Result<Vec<NodeId>, Error> {
        if let Some(atoms) = self.constants.get(&c) {
            return Ok(atoms.clone());
        }
        let mut atoms = Vec::with_capacity(self.size as usize);
        for e in 0..self.size {
            atoms.push(self.atom(Entry::Constant(c, e))?);
        }
        // The root's domain, which every world's holds.
        let root: Vec<NodeId> = self.domain[..self.size as usize].to_vec();
        self.exactly_one(&atoms, &root, None)?;
        self.core.held = self.core.held.saturating_add(MEMO_BYTES);
        self.constants.insert(c, atoms.clone());
        self.constant_order.push(c);
        Ok(atoms)
    }

    /// Adds the formulas that one of the atoms holds where `guard` does
    /// (always where it is none), at most one, and the one that holds names
    /// an element of the domain `dom` (`dom[e]` for element `e`).
    fn exactly_one(
        &mut self,
        atoms: &[NodeId],
        dom: &[NodeId],
        guard: Option<NodeId>,
    ) -> Result<(), Error> {
        let some = self.core.junction(false, atoms)?;
        let total = match guard {
            Some(g) => self.implies(g, some)?,
            None => some,
        };
        self.axioms.push(total);
        for (i, &a) in atoms.iter().enumerate() {
            let inside = self.implies(a, dom[i])?;
            self.axioms.push(inside);
            for &b in &atoms[i + 1..] {
                let (na, nb) = (self.not(a)?, self.not(b)?);
                let once = self.core.formulas.add(Node::Or(na, nb))?;
                self.axioms.push(once);
            }
        }
        Ok(())
    }

    /// Returns the value atoms of the principal term `symbol(args)` at
    /// world `w`, made at every world with the formulas that give it one
    /// value on the world's domain and agree along the order.
    fn values(&mut self, symbol: u32, w: u32, args: &[u32]) -> Result<Vec<NodeId>, Error> {
        let n = self.size as usize;
        if let Some(&p) = self.principal_of.get(&(symbol, args.to_vec())) {
            let atoms = &self.principals[p as usize].2;
            return Ok(atoms[w as usize * n..(w as usize + 1) * n].to_vec());
        }
        let p = self.principals.len() as u32;
        let mut atoms = Vec::with_capacity(self.worlds as usize * n);
        for v in 0..self.worlds {
            for e in 0..self.size {
                atoms.push(self.atom(Entry::Value(p, v, e))?);
            }
        }
        // The tuple lies in a world's domain where its largest element does.
        let top = args.iter().copied().max().unwrap_or(0);
        for v in 0..self.worlds {
            let row = atoms[v as usize * n..(v as usize + 1) * n].to_vec();
            let dom = self.domain[v as usize * n..(v as usize + 1) * n].to_vec();
            let guard = self.dom(v, top);
            self.exactly_one(&row, &dom, Some(guard))?;
        }
        for v in 0..self.worlds {
            for u in v + 1..self.worlds {
                for e in 0..n {
                    let parts = [self.le(v, u), self.dom(v, top), atoms[v as usize * n + e]];
                    let both = self.core.junction(true, &parts)?;
                    let agrees = self.implies(both, atoms[u as usize * n + e])?;
                    self.axioms.push(agrees);
                }
            }
        }
        self.core.held = self
            .core
            .held
            .saturating_add(MEMO_BYTES)
            .saturating_add(4 * args.len() as u64);
        self.principal_of.insert((symbol, args.to_vec()), p);
        let row = atoms[w as usize * n..(w as usize + 1) * n].to_vec();
        self.principals.push((symbol, args.to_vec(), atoms));
        if !self.functions.iter().any(|&(f, _)| f == symbol) {
            self.functions.push((symbol, args.len() as u32));
        }
        Ok(row)
    }

    /// Returns what the term denotes at world `w` under the bindings from
    /// `at` back, its subterms read on a stack of their own.
    fn term(
        &mut self,
        t: FoTermId,
        w: u32,
        bindings: &[(u32, u32)],
        at: u32,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Denotes, Error> {
        let sequent = self.sequent;
        let tables = sequent.formulas().first_order();
        let mut results: HashMap<u32, Denotes> = HashMap::default();
        let mut stack = vec![(t, false)];
        while let Some((u, ready)) = stack.pop() {
            if results.contains_key(&(u.index() as u32)) {
                continue;
            }
            let denotes = match tables.terms[u.index()] {
                FoTerm::Bound(index) => Denotes::Element(lookup(bindings, at, index)),
                FoTerm::App(f, list) if tables.list(list).is_empty() => {
                    Denotes::Table(self.constant(f.get())?)
                }
                FoTerm::App(f, list) => {
                    let kids = tables.list(list);
                    let key = (
                        w,
                        u.index() as u32,
                        innermost(bindings, at, self.term_needs[u.index()]),
                    );
                    if let Some(table) = self.denotes.get(&key) {
                        Denotes::Table(table.clone())
                    } else if !ready {
                        stack.push((u, true));
                        stack.extend(kids.iter().map(|&k| (k, false)));
                        continue;
                    } else {
                        let args: Vec<Denotes> = kids
                            .iter()
                            .map(|k| match &results[&(k.index() as u32)] {
                                Denotes::Element(e) => Denotes::Element(*e),
                                Denotes::Table(t) => Denotes::Table(t.clone()),
                            })
                            .collect();
                        let symbol = f.get();
                        let mut table = Vec::with_capacity(self.size as usize);
                        for e in 0..self.size {
                            let formula = over_tuples(self, &args, stop, |wr, tuple| {
                                Ok(wr.values(symbol, w, tuple)?[e as usize])
                            })?;
                            table.push(formula);
                        }
                        self.core.held = self
                            .core
                            .held
                            .saturating_add(MEMO_BYTES)
                            .saturating_add(4 * (key.2.len() as u64 + u64::from(self.size)));
                        self.denotes.insert(key, table.clone());
                        Denotes::Table(table)
                    }
                }
                FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                    unreachable!("the model search refuses a sequent with eigenvariables")
                }
            };
            results.insert(u.index() as u32, denotes);
        }
        Ok(results
            .remove(&(t.index() as u32))
            .expect("the term's denotation"))
    }
}

/// How a node's forcing formula is made from its operands' formulas.
#[derive(Clone, Copy)]
enum Make {
    /// The conjunction (`true`) or disjunction of two at the world.
    Junction(bool),
    /// `¬A` at the world: one formula of `A` per world at or above it.
    Not(u32),
    /// `A → B` at the world: `A`'s and `B`'s formula per world at or above.
    Implies(u32),
    /// `A ↔ B` at the world, as `Implies`.
    Iff(u32),
    /// `∀x. A` at the world: a formula per world at or above it and element.
    Forall(u32),
    /// `∃x. A` at the world: a formula per element.
    Exists(u32),
}

/// A step of the writing of forcing.
enum Task {
    /// Write the node at the world under the bindings, from the memo where
    /// it was written before.
    Enter(NodeId, u32, u32),
    /// Write the node at the world under the bindings.
    Expand(NodeId, u32, u32),
    /// Make a node's formula from the last results.
    Make(Make, usize),
    /// Remember the last result under the key.
    Store(NodeId, u32, Vec<u32>),
}

/// Writes the sequent that a Kripke model of `worlds` worlds with domains
/// of at most `size` elements refutes `sequent` in `logic`, within
/// `limits`.
///
/// # Errors
///
/// The stop, and `limits.occurrences` and `limits.memory_bytes` reached,
/// as refusals; an arena's own refusals.
pub(super) fn write(
    sequent: &Sequent,
    logic: Logic,
    worlds: u32,
    size: u32,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Written, Error> {
    let formulas = sequent.formulas();
    let tables = formulas.first_order();
    // How many innermost bound variables each term and formula reads: the
    // tables and the arena put every operand before its user.
    let mut term_needs = vec![0u32; tables.terms.len()];
    for (i, term) in tables.terms.iter().enumerate() {
        term_needs[i] = match *term {
            FoTerm::Bound(index) => index + 1,
            FoTerm::App(_, list) => tables
                .list(list)
                .iter()
                .map(|k| term_needs[k.index()])
                .max()
                .unwrap_or(0),
            FoTerm::Eigen(_) | FoTerm::Meta(_) => 0,
        };
    }
    let mut needs = vec![0u32; formulas.len()];
    for i in 0..formulas.len() {
        needs[i] = match formulas.node(NodeId(i as u32)) {
            Node::Atom(a) => tables.atoms.get(a as usize).map_or(0, |&(_, args)| {
                tables
                    .list(args)
                    .iter()
                    .map(|k| term_needs[k.index()])
                    .max()
                    .unwrap_or(0)
            }),
            Node::True | Node::False => 0,
            Node::Not(a) => needs[a.index()],
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                needs[a.index()].max(needs[b.index()])
            }
            Node::Forall(b) | Node::Exists(b) => needs[b.index()].saturating_sub(1),
        };
    }
    let mut w = Writer {
        sequent,
        minimal: logic == Logic::Minimal,
        worlds,
        size,
        core: Core::new(limits),
        entries: Vec::new(),
        axioms: Vec::new(),
        below: Vec::new(),
        domain: Vec::new(),
        falsity: Vec::new(),
        holds: HashMap::default(),
        principal_of: HashMap::default(),
        principals: Vec::new(),
        constants: HashMap::default(),
        constant_order: Vec::new(),
        functions: Vec::new(),
        denotes: HashMap::default(),
        term_needs,
        needs,
    };
    frame(&mut w, stop)?;
    // The constants in their order of first use, so the symmetry formulas
    // have them in an order of their own.
    let mut met = vec![false; tables.terms.len()];
    let mut order: Vec<u32> = Vec::new();
    for root in sequent.left().iter().chain(sequent.right()) {
        let mut stack = vec![*root];
        let mut seen = vec![false; formulas.len()];
        while let Some(n) = stack.pop() {
            if std::mem::replace(&mut seen[n.index()], true) {
                continue;
            }
            match formulas.node(n) {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        let mut terms: Vec<FoTermId> = tables.list(args).to_vec();
                        while let Some(t) = terms.pop() {
                            if std::mem::replace(&mut met[t.index()], true) {
                                continue;
                            }
                            if let FoTerm::App(f, list) = tables.terms[t.index()] {
                                if tables.list(list).is_empty() {
                                    order.push(f.get());
                                } else {
                                    terms.extend_from_slice(tables.list(list));
                                }
                            }
                        }
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => stack.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    stack.push(b);
                    stack.push(a);
                }
            }
        }
    }
    let mut constant_atoms = Vec::new();
    for &c in &order {
        constant_atoms.push(w.constant(c)?);
    }
    symmetry(&mut w, &constant_atoms, stop)?;
    let mut left = Vec::new();
    for &f in sequent.left() {
        left.push(force(&mut w, f, stop)?);
    }
    let mut right = Vec::new();
    for &f in sequent.right() {
        right.push(force(&mut w, f, stop)?);
    }
    if right.is_empty() && w.minimal {
        right.push(w.falsity[0]);
    }
    w.tick(stop)?;
    let bytes = (w.core.formulas.len() as u64)
        .saturating_mul(NODE_BYTES)
        .saturating_add(w.core.held);
    let mut hypotheses = std::mem::take(&mut w.axioms);
    hypotheses.extend(left);
    Ok(Written {
        sequent: Sequent::new(w.core.formulas, hypotheses, right)?,
        entries: w.entries,
        principals: w
            .principals
            .into_iter()
            .map(|(f, args, _)| (f, args))
            .collect(),
        constants: w.constant_order,
        functions: w.functions,
        worlds,
        size,
        bytes,
    })
}

/// Writes the frame: the order's atoms (transitive, the root below every
/// world), the domains' atoms (non-empty prefixes, growing), and `⊥` per
/// world in minimal logic (growing).
fn frame(w: &mut Writer<'_>, stop: &mut dyn FnMut(Progress) -> bool) -> Result<(), Error> {
    let (k, n) = (w.worlds, w.size);
    let top = w.core.formulas.add(Node::True)?;
    let bottom = w.core.formulas.add(Node::False)?;
    w.below = vec![top; (k * k) as usize];
    for v in 0..k {
        for u in v + 1..k {
            let le = if v == 0 {
                top
            } else {
                w.atom(Entry::Below(v, u))?
            };
            w.below[(v * k + u) as usize] = le;
        }
    }
    for a in 1..k {
        for b in a + 1..k {
            for c in b + 1..k {
                w.tick(stop)?;
                let both = w.core.junction(true, &[w.le(a, b), w.le(b, c)])?;
                let trans = w.implies(both, w.le(a, c))?;
                w.axioms.push(trans);
            }
        }
    }
    for v in 0..k {
        for d in 0..n {
            let atom = w.atom(Entry::Domain(v, d))?;
            w.domain.push(atom);
        }
        w.axioms.push(w.dom(v, 0));
        for d in 1..n {
            let prefix = w.implies(w.dom(v, d), w.dom(v, d - 1))?;
            w.axioms.push(prefix);
        }
    }
    for v in 0..k {
        for u in v + 1..k {
            for d in 0..n {
                w.tick(stop)?;
                let both = w.core.junction(true, &[w.le(v, u), w.dom(v, d)])?;
                let grows = w.implies(both, w.dom(u, d))?;
                w.axioms.push(grows);
            }
        }
    }
    for v in 0..k {
        let falsity = if w.minimal {
            w.atom(Entry::Falsity(v))?
        } else {
            bottom
        };
        w.falsity.push(falsity);
    }
    if w.minimal {
        for v in 0..k {
            for u in v + 1..k {
                let both = w
                    .core
                    .junction(true, &[w.le(v, u), w.falsity[v as usize]])?;
                let grows = w.implies(both, w.falsity[u as usize])?;
                w.axioms.push(grows);
            }
        }
    }
    Ok(())
}

/// Writes Paradox's symmetry formulas on the constants: the i-th (from 0)
/// takes one of the first i + 1 elements, and an element m > 0 only where
/// an earlier one takes m − 1.
fn symmetry(
    w: &mut Writer<'_>,
    constants: &[Vec<NodeId>],
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<(), Error> {
    for (i, values) in constants.iter().enumerate() {
        for (m, &value) in values.iter().enumerate() {
            w.tick(stop)?;
            if m > i {
                let not = w.not(value)?;
                w.axioms.push(not);
            } else if m > 0 {
                let mut parts = vec![w.not(value)?];
                parts.extend(constants[..i].iter().map(|v| v[m - 1]));
                let earlier = w.core.junction(false, &parts)?;
                w.axioms.push(earlier);
            }
        }
    }
    Ok(())
}

/// Returns the formula that the root forces the closed formula `root`.
fn force(
    w: &mut Writer<'_>,
    root: NodeId,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<NodeId, Error> {
    let formulas = w.sequent.formulas();
    let (k, n) = (w.worlds, w.size);
    let mut bindings: Vec<(u32, u32)> = Vec::new();
    let mut memo: HashMap<(NodeId, u32, Vec<u32>), NodeId> = HashMap::default();
    let mut results: Vec<NodeId> = Vec::new();
    let mut tasks = vec![Task::Enter(root, 0, NONE)];
    while let Some(task) = tasks.pop() {
        w.tick(stop)?;
        if tasks.len() as u64 > w.core.most {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: w.core.most.saturating_add(1),
                limit: w.core.most,
            }));
        }
        let beside = (tasks.capacity() as u64)
            .saturating_mul(40)
            .saturating_add((bindings.capacity() as u64).saturating_mul(8))
            .saturating_add((results.capacity() as u64).saturating_mul(4));
        if beside.saturating_add(w.core.held) > w.core.memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: w.core.memory,
                needed_bytes: None,
            }));
        }
        match task {
            Task::Store(node, world, values) => {
                let made = *results.last().expect("the node's formula");
                w.core.held = w
                    .core
                    .held
                    .saturating_add(MEMO_BYTES)
                    .saturating_add(4 * values.len() as u64);
                memo.insert((node, world, values), made);
            }
            Task::Enter(node, world, at) => {
                let values = innermost(&bindings, at, w.needs[node.index()]);
                match memo.get(&(node, world, values.clone())) {
                    Some(&made) => results.push(made),
                    None => {
                        tasks.push(Task::Store(node, world, values));
                        tasks.push(Task::Expand(node, world, at));
                    }
                }
            }
            Task::Expand(node, world, at) => match formulas.node(node) {
                Node::Atom(a) => {
                    let tables = formulas.first_order();
                    let made = match tables.atoms.get(a as usize) {
                        None => w.holds(a, world, &[])?,
                        Some(&(symbol, args)) => {
                            let mut denotes = Vec::new();
                            for &t in tables.list(args) {
                                denotes.push(w.term(t, world, &bindings, at, stop)?);
                            }
                            let predicate = symbol.get();
                            over_tuples(w, &denotes, stop, |wr, tuple| {
                                wr.holds(predicate, world, tuple)
                            })?
                        }
                    };
                    results.push(made);
                }
                Node::True => results.push(w.core.formulas.add(Node::True)?),
                Node::False => results.push(w.falsity[world as usize]),
                Node::And(a, b) | Node::Or(a, b) => {
                    let and = matches!(formulas.node(node), Node::And(..));
                    tasks.push(Task::Make(Make::Junction(and), 2));
                    tasks.push(Task::Enter(b, world, at));
                    tasks.push(Task::Enter(a, world, at));
                }
                Node::Not(a) => {
                    tasks.push(Task::Make(Make::Not(world), (k - world) as usize));
                    for u in (world..k).rev() {
                        tasks.push(Task::Enter(a, u, at));
                    }
                }
                Node::Implies(a, b) | Node::Iff(a, b) => {
                    let make = if matches!(formulas.node(node), Node::Iff(..)) {
                        Make::Iff(world)
                    } else {
                        Make::Implies(world)
                    };
                    tasks.push(Task::Make(make, 2 * (k - world) as usize));
                    for u in (world..k).rev() {
                        tasks.push(Task::Enter(b, u, at));
                        tasks.push(Task::Enter(a, u, at));
                    }
                }
                Node::Forall(body) => {
                    tasks.push(Task::Make(Make::Forall(world), ((k - world) * n) as usize));
                    for u in (world..k).rev() {
                        for d in (0..n).rev() {
                            bindings.push((d, at));
                            tasks.push(Task::Enter(body, u, (bindings.len() - 1) as u32));
                        }
                    }
                }
                Node::Exists(body) => {
                    tasks.push(Task::Make(Make::Exists(world), n as usize));
                    for d in (0..n).rev() {
                        bindings.push((d, at));
                        tasks.push(Task::Enter(body, world, (bindings.len() - 1) as u32));
                    }
                }
            },
            Task::Make(make, count) => {
                let start = results.len() - count;
                let parts = results.split_off(start);
                let made = match make {
                    Make::Junction(and) => w.core.junction(and, &parts)?,
                    Make::Not(v) => {
                        let mut conjuncts = Vec::with_capacity(parts.len());
                        for (i, &a) in parts.iter().enumerate() {
                            let u = v + i as u32;
                            let refuted = w.implies(a, w.falsity[u as usize])?;
                            conjuncts.push(guarded(w, v, u, refuted)?);
                        }
                        w.core.junction(true, &conjuncts)?
                    }
                    Make::Implies(v) | Make::Iff(v) => {
                        let iff = matches!(make, Make::Iff(_));
                        let mut conjuncts = Vec::with_capacity(parts.len());
                        for (i, pair) in parts.chunks(2).enumerate() {
                            let u = v + i as u32;
                            let forward = w.implies(pair[0], pair[1])?;
                            conjuncts.push(guarded(w, v, u, forward)?);
                            if iff {
                                let back = w.implies(pair[1], pair[0])?;
                                conjuncts.push(guarded(w, v, u, back)?);
                            }
                        }
                        w.core.junction(true, &conjuncts)?
                    }
                    Make::Forall(v) => {
                        let mut conjuncts = Vec::with_capacity(parts.len());
                        for (i, &body) in parts.iter().enumerate() {
                            let u = v + (i as u32) / n;
                            let d = (i as u32) % n;
                            let inside = w.implies(w.dom(u, d), body)?;
                            conjuncts.push(guarded(w, v, u, inside)?);
                        }
                        w.core.junction(true, &conjuncts)?
                    }
                    Make::Exists(v) => {
                        let mut disjuncts = Vec::with_capacity(parts.len());
                        for (d, &body) in parts.iter().enumerate() {
                            disjuncts.push(w.core.junction(true, &[w.dom(v, d as u32), body])?);
                        }
                        w.core.junction(false, &disjuncts)?
                    }
                };
                results.push(made);
            }
        }
    }
    Ok(results.pop().expect("the root's forcing formula"))
}

/// Returns `le(v, u) → formula`, or the formula itself on the diagonal.
fn guarded(w: &mut Writer<'_>, v: u32, u: u32, formula: NodeId) -> Result<NodeId, Error> {
    if v == u {
        Ok(formula)
    } else {
        w.implies(w.le(v, u), formula)
    }
}

impl Written {
    /// Returns how many atoms the written sequent has.
    pub(super) fn atoms(&self) -> u64 {
        self.entries.len() as u64
    }

    /// Returns the model the atoms `true_atoms` (indices among the written
    /// sequent's atom names) give, its tables charged to
    /// `limits.memory_bytes`. Its worlds are those of the frame, its arrows
    /// the order's covering pairs.
    pub(super) fn model(
        &self,
        sequent: &Sequent,
        true_atoms: &[u32],
        limits: &Limits,
    ) -> Result<KripkeStructure, Error> {
        let (k, n) = (self.worlds as usize, self.size as usize);
        let names = self.sequent.formulas().atom_names();
        let mut le = vec![false; k * k];
        for v in 0..k {
            le[v * k + v] = true;
            le[v] = true;
        }
        let mut dom = vec![false; k * n];
        let mut falsity = vec![false; k];
        let mut value: HashMap<(u32, u32), u32> = HashMap::default();
        let mut constant: HashMap<u32, u32> = HashMap::default();
        let mut holds: Vec<HashMap<u32, Vec<Vec<u32>>>> = vec![HashMap::default(); k];
        for &a in true_atoms {
            let Some(number) = names[a as usize]
                .strip_prefix("k_")
                .and_then(|m| m.parse::<usize>().ok())
            else {
                continue;
            };
            match &self.entries[number] {
                Entry::Below(v, u) => le[*v as usize * k + *u as usize] = true,
                Entry::Domain(v, d) => dom[*v as usize * n + *d as usize] = true,
                Entry::Falsity(v) => falsity[*v as usize] = true,
                Entry::Holds(p, v, tuple) => {
                    holds[*v as usize]
                        .entry(*p)
                        .or_default()
                        .push(tuple.clone());
                }
                Entry::Value(p, v, e) => {
                    value.insert((*p, *v), *e);
                }
                Entry::Constant(c, e) => {
                    constant.insert(*c, *e);
                }
            }
        }
        let sizes: Vec<u32> = (0..k)
            .map(|v| (0..n).filter(|&d| dom[v * n + d]).count().max(1) as u32)
            .collect();
        let tables = sequent.formulas().first_order();
        let formulas = sequent.formulas();
        let symbol = |s: u32| tables.symbols[s as usize].0.clone();
        let memory = limits.memory_bytes.unwrap_or(u64::MAX);
        let refused = |needed: u64| {
            Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: memory,
                needed_bytes: Some(needed),
            })
        };
        let mut cells: u64 = 0;
        let mut worlds = Vec::with_capacity(k);
        for v in 0..k {
            let size = sizes[v];
            // The worlds directly above: above, and no world between.
            let arrows: Vec<u32> = (v + 1..k)
                .filter(|&u| le[v * k + u] && !(v + 1..u).any(|m| le[v * k + m] && le[m * k + u]))
                .map(|u| u as u32)
                .collect();
            let mut functions = Vec::with_capacity(self.functions.len());
            for &(f, arity) in &self.functions {
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
                for (p, (s, args)) in self.principals.iter().enumerate() {
                    if *s != f || args.iter().any(|&e| e >= size) {
                        continue;
                    }
                    if let Some(&e) = value.get(&(p as u32, v as u32)) {
                        let cell = args
                            .iter()
                            .fold(0usize, |i, &d| i * size as usize + d as usize);
                        table[cell] = e;
                    }
                }
                functions.push((symbol(f), table));
            }
            let mut predicates: Vec<(String, Vec<Vec<u32>>)> = holds[v]
                .iter()
                .map(|(&p, tuples)| {
                    let mut kept: Vec<Vec<u32>> = tuples
                        .iter()
                        .filter(|t| t.iter().all(|&e| e < size))
                        .cloned()
                        .collect();
                    kept.sort_unstable();
                    let name = if tables.atoms.is_empty() {
                        formulas.atom_name(p).to_owned()
                    } else {
                        symbol(p)
                    };
                    (name, kept)
                })
                .filter(|(_, kept)| !kept.is_empty())
                .collect();
            predicates.sort_unstable();
            worlds.push(KripkeWorld::new(
                size, functions, predicates, falsity[v], arrows,
            ));
        }
        let constants = self
            .constants
            .iter()
            .map(|&c| (symbol(c), constant.get(&c).copied().unwrap_or(0)))
            .collect();
        Ok(KripkeStructure::new(worlds, constants, 0))
    }
}
