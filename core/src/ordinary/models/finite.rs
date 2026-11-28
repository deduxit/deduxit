// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The formula that a structure of `n` elements refutes a first-order
//! sequent, written for the SAT solver, and the structure read off the
//! solver's model.

use super::encode::{Core, Denotes, Writes, lookup, over_tuples};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::grounding::{self, G, Nnf, Term};
use crate::ordinary::{Node, NodeId, Sequent, Structure};

/// No index.
const NONE: u32 = u32::MAX;

/// The bytes a formula of the expansion's arena holds, with its share of
/// the arena's index: grounding's figure.
const NODE_BYTES: u64 = grounding::NODE_BYTES;

/// The bytes an entry of a memo holds beside 4 bytes a value: its key's
/// vector and its slot in the table.
const MEMO_BYTES: u64 = grounding::MEMO_BYTES;

/// The constants and function symbols the normal form names.
pub(super) struct Signature {
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
    pub(super) fn of(nnf: &Nnf, root: u32) -> Self {
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
pub(super) enum Entry {
    /// A predicate, by its index (the symbol's in a first-order arena, the
    /// atom's in a propositional one), holds of a tuple.
    Holds(u32, Vec<u32>),
    /// A principal term, by its index in [`Expansion::principals`], has
    /// this value.
    Value(u32, u32),
}

/// The formula that a structure of `size` elements refutes the sequent,
/// with what reads a structure off its model.
pub(super) struct Expansion {
    /// `Ax ⊢ F`: the totality, functionality and symmetry formulas left,
    /// the expanded formula right.
    pub(super) sequent: Sequent,
    /// Each atom, by the number its name `m_N` carries.
    pub(super) entries: Vec<Entry>,
    /// Each principal term: its symbol, its arguments, and its value
    /// atoms, one per element.
    principals: Vec<(u32, Vec<u32>, Vec<NodeId>)>,
    /// The bytes the expansion held at its end, by its estimate: the SAT
    /// solver gets the memory bound less these.
    pub(super) bytes: u64,
    /// The domain's size.
    size: u32,
}

/// The state of an expansion while it is written.
struct Writer<'n> {
    /// The normal form.
    nnf: &'n Nnf,
    /// The domain's size.
    size: u32,
    /// The arena written, with its bounds.
    core: Core,
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
        self.core.tick_by(weight, stop)
    }

    /// Makes an atom of the entry and returns it.
    fn atom(&mut self, entry: Entry) -> Result<NodeId, Error> {
        let key = match &entry {
            Entry::Holds(_, tuple) => tuple.len() as u64,
            Entry::Value(..) => 1,
        };
        let id = self.core.atom("m", self.entries.len(), key)?;
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
        self.core.held = self
            .core
            .held
            .saturating_add(MEMO_BYTES)
            .saturating_add(4 * args.len() as u64);
        self.principal_of.insert((symbol, args.to_vec()), p);
        self.principals.push((symbol, args.to_vec(), atoms.clone()));
        Ok(atoms)
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
                let formula = over_tuples(self, &args, stop, |w, tuple| {
                    Ok(w.values(symbol, tuple)?[e as usize])
                })?;
                table.push(formula);
            }
            let k = key(b);
            self.core.held = self
                .core
                .held
                .saturating_add(MEMO_BYTES)
                .saturating_add(4 * (k.1.len() as u64 + u64::from(self.size)));
            self.denotes.insert(k, table);
        }
        Ok(Denotes::Table(self.denotes[&key(a)].clone()))
    }
}

impl Writes for Writer<'_> {
    fn core(&mut self) -> &mut Core {
        &mut self.core
    }

    fn size(&self) -> u32 {
        self.size
    }
}

impl Expansion {
    /// Writes the formula that a structure of `size` elements refutes the
    /// sequent whose normal form is `nnf`, within `limits`.
    pub(super) fn new(
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
            core: Core::new(limits),
            entries: Vec::new(),
            holds: HashMap::default(),
            principals: Vec::new(),
            principal_of: HashMap::default(),
            denotes: HashMap::default(),
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
            left.push(w.core.junction(false, &atoms)?);
            for (i, &a) in atoms.iter().enumerate() {
                for &b in &atoms[i + 1..] {
                    w.tick(stop)?;
                    let not_a = w.core.formulas.add(Node::Not(a))?;
                    let not_b = w.core.formulas.add(Node::Not(b))?;
                    left.push(w.core.formulas.add(Node::Or(not_a, not_b))?);
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
                    left.push(w.core.formulas.add(Node::Not(value))?);
                } else if m > 0 {
                    let mut parts = vec![w.core.formulas.add(Node::Not(value))?];
                    parts.extend(constant_values[..i].iter().map(|v| v[m - 1]));
                    left.push(w.core.junction(false, &parts)?);
                }
            }
        }
        w.tick(stop)?;
        let bytes = (w.core.formulas.len() as u64)
            .saturating_mul(NODE_BYTES)
            .saturating_add(w.core.held);
        Ok(Self {
            sequent: Sequent::new(w.core.formulas, left, vec![formula])?,
            entries: w.entries,
            principals: w.principals,
            bytes,
            size,
        })
    }

    /// Returns the structure the atoms `true_atoms` (indices among the
    /// expansion's atom names) give, its function tables charged to
    /// `limits.memory_bytes`.
    pub(super) fn structure(
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
        if tasks.len() as u64 > w.core.most {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: w.core.most.saturating_add(1),
                limit: w.core.most,
            }));
        }
        let beside = (tasks.capacity() as u64)
            .saturating_mul(40)
            .saturating_add((bindings.capacity() as u64).saturating_mul(12))
            .saturating_add((results.capacity() as u64).saturating_mul(4));
        if beside.saturating_add(w.core.held) > w.core.memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: w.core.memory,
                needed_bytes: None,
            }));
        }
        match task {
            Task::Store(n, values) => {
                let made = *results.last().expect("the node's formula");
                w.core.held = w
                    .core
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
                        over_tuples(w, &denotes, stop, |w, tuple| w.holds(predicate, tuple))?;
                    results.push(if *positive {
                        formula
                    } else {
                        w.core.formulas.add(Node::Not(formula))?
                    });
                }
                G::True => results.push(w.core.formulas.add(Node::True)?),
                G::False => results.push(w.core.formulas.add(Node::False)?),
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
                let joined = w.core.junction(and, &parts)?;
                results.push(joined);
            }
        }
    }
    Ok(results.pop().expect("the expanded formula"))
}
