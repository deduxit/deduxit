// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The read-back of an intuitionistic connection proof to LJ: a search in
//! G3i that the proof guides, then the derivation of its steps, which the
//! checker judges.
//!
//! The proof's substitution says which terms each quantifier is
//! instantiated with (`σQ`), and its Skolem terms which eigenvariable
//! stands where. So the search may take, at a `∀L` or an `∃R`, only the
//! terms the substitution gave that quantifier's variables, each Skolem
//! term in them read as the eigenvariable its `∀R` or `∃L` made, and only
//! once that step is below. With the instances fixed, what is left is a
//! propositional search: the invertible rules first, then each
//! non-invertible choice, a sequent repeated on its branch cut (formulas as
//! a set). Kreitz and Schmitt show that the substitution's instances
//! suffice ("A uniform procedure for converting matrix proofs into
//! sequent-style systems", 2000); their conversion makes a proof with
//! several formulas right of `⊢`, so this read-back searches G3i directly
//! instead.
//!
//! The search is bounded by [`STEPS`] rules tried and by the recursion
//! depth the limits allow; past either it gives up, and the proof is not
//! taken. Every derivation it makes is checked by the caller.

use super::super::matrix::{Kind, Matrix, NONE, Symbol};
use super::super::sizes::TermBudget;
use super::super::terms::{Head, Terms};
use crate::Error;
use crate::first_order::{Eigen, FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::proofs::InfId;
use crate::ordinary::derivation::{self, Builder};
use crate::ordinary::{Derivation, Formulas, Inference, Logic, Node, NodeId, Rule, Sequent, Side};

/// The rules the search tries before it gives a proof up.
const STEPS: u64 = 1 << 20;

/// How many units of work the search does between two asks of the stop:
/// a rule tried, and a unit per [`HYPOTHESES_PER_UNIT`] hypotheses of its
/// sequent, which a level copies.
const POLL: u64 = 1 << 10;

/// The hypotheses a level of the search handles per unit of work.
const HYPOTHESES_PER_UNIT: usize = 1 << 6;

/// The bytes a level of the search holds per hypothesis while it is open:
/// the formula, its node, and the branch's key.
const LEVEL_BYTES: usize = size_of::<Formula>() + 2 * size_of::<NodeId>();

/// The bytes an instance in the memo holds, its key's terms apart.
const INSTANCE_BYTES: usize = 64;

/// The bytes a node the read-back adds to the arena holds, with its
/// share of the tables that intern it.
const NODE_BYTES: usize = 48;

/// A term of the proof: a symbol of the matrix applied to terms, or a
/// variable no connection bound, which stands for any term.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Tm {
    /// A symbol of the matrix's patterns applied to terms.
    App(u32, Box<[u32]>),
    /// A variable left unbound.
    Free,
}

/// A formula of the search: its formula of the arena, the occurrence it
/// instantiates ([`NONE`] for the `⊥` a goal or a negation adds), and the
/// terms its binders took.
#[derive(Clone, Copy, Debug)]
struct Formula {
    /// Its formula, closed.
    node: NodeId,
    /// The occurrence it instantiates.
    occ: u32,
    /// The terms of the binders above it, an entry of `Search::envs`.
    env: u32,
}

/// One step of a G3i derivation the search found.
#[derive(Clone, Debug)]
struct Step {
    /// The hypotheses, each once.
    left: Vec<NodeId>,
    /// The goal.
    goal: NodeId,
    /// The rule.
    rule: Rule,
    /// The principal formula, where the rule has one: a hypothesis, or the
    /// goal.
    principal: Option<(Side, NodeId)>,
    /// The rule's term, for a quantifier rule.
    term: Option<FoTermId>,
    /// The formulas the rule adds to the hypotheses of each premise.
    added: Vec<Vec<NodeId>>,
    /// The premises.
    premises: Vec<u32>,
}

/// The state of the search.
struct Search<'m, 's> {
    /// The matrix.
    matrix: &'m Matrix,
    /// The caller's stop, asked every [`POLL`] rules.
    stop: &'s mut dyn FnMut(Progress) -> bool,
    /// The arena, which gains the instances.
    formulas: Formulas,
    /// The logic: classical is never asked; minimal logic has no `⊥L`.
    logic: Logic,
    /// `⊥`.
    falsity: NodeId,
    /// The proof terms, each once.
    tms: Vec<Tm>,
    /// The index of every proof term.
    tm_ids: HashMap<Tm, u32>,
    /// Per slot, the terms the substitution gave its variables.
    candidates: Vec<Vec<u32>>,
    /// The environments: a parent, a binder occurrence, its proof term and
    /// the term of the arena that stood for it when it was bound. A
    /// formula's children read the latter, so they stay its parts when the
    /// branch later maps a Skolem term to another eigenvariable.
    envs: Vec<(u32, u32, u32, FoTermId)>,
    /// The eigenvariable of each Skolem term on the current branch, latest
    /// last, and whether a δ-rule of the branch introduced it (else it is a
    /// constant the branch took for a witness whose δ-rule lies elsewhere).
    eigens: Vec<(u32, u32, bool)>,
    /// The next eigenvariable's number.
    next_eigen: u32,
    /// The eigenvariable that stands for a variable left unbound.
    anonymous: u32,
    /// The sequents on the current branch.
    branch: HashSet<(Vec<NodeId>, NodeId)>,
    /// The steps found.
    steps: Vec<Step>,
    /// The rules tried.
    tried: u64,
    /// The work since the stop was last asked.
    since: u64,
    /// The bytes the open levels and the steps hold.
    held: usize,
    /// The memory bound, `None` for none.
    memory: Option<u64>,
    /// The deepest branch the search may take.
    depth: u32,
    /// Whether the search gave up at a bound.
    gave_up: bool,
    /// Whether the eigenvariables' numbers ran out, so that two may be one.
    out_of_numbers: bool,
    /// Every instance built: a formula and the terms put in for its
    /// binders' variables, innermost first.
    instances: HashMap<(NodeId, Box<[FoTermId]>), NodeId>,
    /// The arena's terms, held to `limits.terms`.
    budget: TermBudget,
}

/// Reads the proof `terms` of `matrix`, a matrix of `sequent`, back to a
/// derivation of LJ, or `None` where the guided search finds none within
/// its bounds.
///
/// # Errors
///
/// A refusal of the derivation by `limits` or `stop`, and
/// [`Refusal::Index`] past the arena's indices.
pub(super) fn read_back(
    sequent: &Sequent,
    matrix: &Matrix,
    terms: &Terms,
    logic: Logic,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Option<Derivation>, Error> {
    let mut formulas = sequent.formulas().clone();
    crate::ordinary::translate::Extra::add(&mut formulas)?;
    let falsity = formulas.add(Node::False)?;
    let first = crate::ordinary::derivation::next_eigen(&formulas);
    let budget = TermBudget::new(&formulas, limits.terms);
    let mut search = Search {
        matrix,
        stop: &mut *stop,
        formulas,
        logic,
        falsity,
        tms: Vec::new(),
        tm_ids: HashMap::default(),
        candidates: vec![Vec::new(); matrix.slot_occ.len()],
        envs: Vec::new(),
        eigens: Vec::new(),
        next_eigen: first.saturating_add(1),
        anonymous: first,
        branch: HashSet::default(),
        steps: Vec::new(),
        tried: 0,
        since: 0,
        held: 0,
        memory: limits.memory_bytes,
        // A level of `prove` takes up to 15 KiB of stack unoptimized,
        // against the 13 KiB a level of `recursion_depth` is sized for
        // (1.5 KiB optimized, against 3 KiB): half the levels fit.
        depth: (limits.recursion_depth / 2).min(1 << 14),
        gave_up: false,
        out_of_numbers: first == u32::MAX,
        instances: HashMap::default(),
        budget,
    };
    search.collect_candidates(terms);
    let roots = &matrix.roots;
    let hypotheses = sequent.left().len();
    let mut left = Vec::new();
    for &r in &roots[..hypotheses] {
        left.push(search.formula(r, NONE)?);
    }
    let goal = match roots.get(hypotheses) {
        Some(&r) => search.formula(r, NONE)?,
        None => Formula {
            node: falsity,
            occ: NONE,
            env: NONE,
        },
    };
    let mut set: Vec<Formula> = Vec::new();
    for f in left {
        if !set.iter().any(|g| g.node == f.node) {
            set.push(f);
        }
    }
    let Some(root) = search.prove(&set, goal, 0)? else {
        return Ok(None);
    };
    if search.out_of_numbers {
        return Ok(None);
    }
    let written: Vec<NodeId> = sequent.left().to_vec();
    let Search {
        formulas, steps, ..
    } = search;
    let inferences = {
        let mut emit = Emit::new(limits, stop);
        let d = emit.derivation(&steps, root)?;
        let d = emit.weaken_to(d, &written)?;
        emit.in_order(d, &written);
        emit.inferences
    };
    Ok(Some(Derivation::from_parts(
        logic,
        formulas,
        (written, vec![goal.node]),
        inferences,
    )))
}

impl Search<'_, '_> {
    /// Returns the proof term of `t`, read in the substitution, each once.
    /// `memo` holds the terms read so far by pattern and copy: bindings
    /// share terms, which read as trees would be exponential.
    fn tm_of(&mut self, terms: &Terms, t: (u32, u32), memo: &mut HashMap<(u32, u32), u32>) -> u32 {
        // Each entry: a term and whether its arguments are made.
        let mut stack: Vec<((u32, u32), bool)> = vec![(t, false)];
        let mut made: Vec<u32> = Vec::new();
        while let Some((u, done)) = stack.pop() {
            if !done && let Some(&tm) = memo.get(&u) {
                made.push(tm);
                continue;
            }
            match terms.head(self.matrix, u) {
                Head::Var(_) => made.push(self.intern(Tm::Free)),
                Head::App(f, (s, len), c) => {
                    if !done {
                        stack.push((u, true));
                        for k in (0..len).rev() {
                            stack.push(((self.matrix.pat_args[(s + k) as usize], c), false));
                        }
                        continue;
                    }
                    let args = made.split_off(made.len() - len as usize);
                    let tm = self.intern(Tm::App(f, args.into_boxed_slice()));
                    memo.insert(u, tm);
                    made.push(tm);
                }
            }
        }
        made.pop()
            .unwrap_or_else(|| unreachable!("the term is made"))
    }

    /// Returns the index of `tm`, adding it where new.
    fn intern(&mut self, tm: Tm) -> u32 {
        if let Some(&id) = self.tm_ids.get(&tm) {
            return id;
        }
        let id = self.tms.len() as u32;
        self.tms.push(tm.clone());
        self.tm_ids.insert(tm, id);
        id
    }

    /// Gathers, per slot, the terms the substitution gave the variables of
    /// the slot in every copy.
    fn collect_candidates(&mut self, terms: &Terms) {
        let mut memo = HashMap::default();
        for k in 0..terms.copies.len() {
            let c = terms.copies[k];
            let made: Vec<(u32, u32)> = if c.clause == NONE {
                (0..c.vars.1).map(|s| (s, terms.vars[s as usize])).collect()
            } else {
                self.matrix
                    .clause_slots(c.clause)
                    .iter()
                    .enumerate()
                    .filter(|&(_, &(_, free))| free)
                    .map(|(at, &(slot, _))| (slot, terms.vars[(c.vars.0 + at as u32) as usize]))
                    .collect()
            };
            for (slot, var) in made {
                let (pat, copy) = terms.bound[var as usize];
                let tm = if pat == NONE {
                    self.intern(Tm::Free)
                } else {
                    self.tm_of(terms, (pat, copy), &mut memo)
                };
                if !self.candidates[slot as usize].contains(&tm) {
                    self.candidates[slot as usize].push(tm);
                }
            }
        }
        // A variable no connection bound is tried last: any term serves it,
        // and a bound one is what the proof used.
        let free = self.intern(Tm::Free);
        for list in &mut self.candidates {
            list.sort_by_key(|&tm| tm == free);
        }
    }

    /// Returns the eigenvariable of the Skolem term `tm` on the branch.
    fn eigen_of(&self, tm: u32) -> Option<u32> {
        self.mapped(tm).map(|(e, _)| e)
    }

    /// Returns the eigenvariable of the Skolem term `tm` on the branch,
    /// and whether a δ-rule of the branch introduced it.
    fn mapped(&self, tm: u32) -> Option<(u32, bool)> {
        self.eigens
            .iter()
            .rev()
            .find(|&&(k, _, _)| k == tm)
            .map(|&(_, e, introduced)| (e, introduced))
    }

    /// Gives every Skolem term in `tm` without an eigenvariable on the
    /// branch a fresh constant, and returns how many it gave: a witness
    /// whose δ-rule lies on another branch names a constant on this one.
    fn constants(&mut self, tm: u32) -> usize {
        let mut given = 0;
        let mut stack = vec![tm];
        // Proof terms share their parts: each is visited once.
        let mut seen = HashSet::default();
        while let Some(t) = stack.pop() {
            if !seen.insert(t) {
                continue;
            }
            if let Tm::App(f, args) = self.tms[t as usize].clone() {
                if matches!(
                    self.matrix.symbols.get(f as usize),
                    Some(Symbol::Skolem(_)) | None
                ) && self.eigen_of(t).is_none()
                {
                    let e = self.fresh_eigen();
                    self.eigens.push((t, e, false));
                    given += 1;
                }
                stack.extend(args.iter().copied());
            }
        }
        given
    }

    /// Returns the candidates of `slot`, those whose Skolem terms all have
    /// their eigenvariables on the branch first, the variable left unbound
    /// last: it fits any proof, so it is the choice of last resort.
    fn ordered(&self, slot: u32) -> Vec<u32> {
        let list = &self.candidates[slot as usize];
        let (mut ready, rest): (Vec<u32>, Vec<u32>) = list
            .iter()
            .filter(|&&tm| self.tms[tm as usize] != Tm::Free)
            .partition(|&&tm| self.ready(tm));
        ready.extend(rest);
        ready.extend(list.iter().filter(|&&tm| self.tms[tm as usize] == Tm::Free));
        ready
    }

    /// Returns whether every Skolem term in `tm` has its eigenvariable on
    /// the branch.
    fn ready(&self, tm: u32) -> bool {
        let mut stack = vec![tm];
        let mut seen = HashSet::default();
        while let Some(t) = stack.pop() {
            if !seen.insert(t) {
                continue;
            }
            if let Tm::App(f, args) = &self.tms[t as usize] {
                if matches!(
                    self.matrix.symbols.get(*f as usize),
                    Some(Symbol::Skolem(_)) | None
                ) && self.eigen_of(t).is_none()
                {
                    return false;
                }
                stack.extend(args.iter().copied());
            }
        }
        true
    }

    /// Returns the term of the arena `tm` stands for on the branch: a
    /// Skolem term its eigenvariable, a variable left unbound the
    /// anonymous one. The caller has checked it is [`ready`](Self::ready).
    fn term(&mut self, tm: u32) -> FoTermId {
        let mut stack: Vec<(u32, bool)> = vec![(tm, false)];
        let mut made: Vec<FoTermId> = Vec::new();
        // Proof terms share their parts: each is made once.
        let mut memo: HashMap<u32, FoTermId> = HashMap::default();
        while let Some((t, done)) = stack.pop() {
            if !done && let Some(&id) = memo.get(&t) {
                made.push(id);
                continue;
            }
            let made_term = match self.tms[t as usize].clone() {
                Tm::Free => FoTerm::Eigen(Eigen::new(self.anonymous)),
                Tm::App(f, args) => match self.matrix.symbols.get(f as usize).copied() {
                    // A Skolem term, or one of a δ-formula whose variable
                    // no literal holds, which has no symbol in the matrix.
                    Some(Symbol::Skolem(_)) | None => {
                        let e = self.eigen_of(t).unwrap_or(self.anonymous);
                        FoTerm::Eigen(Eigen::new(e))
                    }
                    Some(Symbol::Parameter(p)) => {
                        made.push(p);
                        continue;
                    }
                    Some(Symbol::Function(symbol)) => {
                        if !done {
                            stack.push((t, true));
                            stack.extend(args.iter().rev().map(|&a| (a, false)));
                            continue;
                        }
                        let items = made.split_off(made.len() - args.len());
                        let list = self.formulas.interner().list(&items);
                        FoTerm::App(symbol, list)
                    }
                },
            };
            let id = self.formulas.interner().term(made_term);
            memo.insert(t, id);
            made.push(id);
        }
        made.pop()
            .unwrap_or_else(|| unreachable!("the term is made"))
    }

    /// Returns the formula of occurrence `occ` under the environment
    /// `env`: its formula with the environment's terms for its binders'
    /// variables.
    fn formula(&mut self, occ: u32, env: u32) -> Result<Formula, Error> {
        let o = self.matrix.occs[occ as usize];
        let base = match o.kind {
            Kind::Part { forward } => {
                let Node::Iff(a, b) = self.formulas.node(o.node) else {
                    unreachable!("a part of an equivalence");
                };
                let (from, to) = if forward { (a, b) } else { (b, a) };
                self.formulas.add(Node::Implies(from, to))?
            }
            Kind::Atom(_)
            | Kind::True
            | Kind::False
            | Kind::Not
            | Kind::And
            | Kind::Or
            | Kind::Implies
            | Kind::Iff
            | Kind::Gamma(_)
            | Kind::Delta(_) => o.node,
        };
        let mut terms = Vec::new();
        let mut e = env;
        while e != NONE {
            let (parent, _, _, value) = self.envs[e as usize];
            terms.push(value);
            e = parent;
        }
        let key = (base, terms.into_boxed_slice());
        if let Some(&node) = self.instances.get(&key) {
            return Ok(Formula { node, occ, env });
        }
        // Building an instance is the search's costly part: it counts
        // toward its bound and the stop's cadence as a rule tried does.
        self.tick()?;
        let before = self.formulas.len();
        self.budget.check(&self.formulas, key.0, &key.1)?;
        let node = self.formulas.instantiate_frame(key.0, &key.1)?;
        self.budget.count(&self.formulas);
        let added = self.formulas.len() - before;
        self.work((added / HYPOTHESES_PER_UNIT) as u64)?;
        self.charge(added.saturating_mul(NODE_BYTES))?;
        self.instances.insert(key, node);
        Ok(Formula { node, occ, env })
    }

    /// Returns the formula of child `k` of `f`, under `f`'s environment,
    /// with `bound` for a binder's variable.
    fn child(&mut self, f: Formula, k: usize, bound: Option<u32>) -> Result<Formula, Error> {
        // Every child made is work: the choices of a sequent with many
        // quantified hypotheses and witnesses make many.
        self.tick()?;
        let occ = self.matrix.occs[f.occ as usize].children[k];
        let env = match bound {
            Some(tm) => {
                let value = self.term(tm);
                self.envs.push((f.env, f.occ, tm, value));
                (self.envs.len() - 1) as u32
            }
            None => f.env,
        };
        // `f.node` is `f`'s instance, so a connective's child is its
        // operand, and a binder's the body with the bound term put in:
        // building the child's instance from the matrix's formula again
        // would cost the whole subformula per child.
        let before = self.formulas.len();
        let node = match (self.kind(f), self.formulas.node(f.node)) {
            (Kind::And | Kind::Or | Kind::Implies | Kind::Part { .. }, node) => match node {
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) => {
                    if k == 0 {
                        a
                    } else {
                        b
                    }
                }
                Node::Atom(_)
                | Node::True
                | Node::False
                | Node::Not(_)
                | Node::Iff(..)
                | Node::Forall(_)
                | Node::Exists(_) => return self.formula(occ, env),
            },
            (Kind::Not, Node::Not(a)) => a,
            (Kind::Iff, Node::Iff(a, b)) => {
                let (from, to) = if k == 0 { (a, b) } else { (b, a) };
                self.formulas.add(Node::Implies(from, to))?
            }
            (Kind::Gamma(_) | Kind::Delta(_), Node::Forall(body) | Node::Exists(body)) => {
                let Some(&(_, _, _, value)) = self.envs.get(env as usize) else {
                    return self.formula(occ, env);
                };
                if bound.is_none() {
                    return self.formula(occ, env);
                }
                // Each instance once: building one writes its atoms' keys,
                // which a shared witness makes long.
                let key = (body, vec![value].into_boxed_slice());
                if let Some(&node) = self.instances.get(&key) {
                    node
                } else {
                    self.budget.check(&self.formulas, body, &[value])?;
                    let node = self.formulas.instantiate(body, value)?;
                    self.budget.count(&self.formulas);
                    self.instances.insert(key, node);
                    node
                }
            }
            _ => return self.formula(occ, env),
        };
        // An instance costs the nodes it adds: work for the stop's cadence,
        // bytes for the memory bound.
        let added = self.formulas.len() - before;
        self.work((added / HYPOTHESES_PER_UNIT) as u64)?;
        self.charge(added.saturating_mul(NODE_BYTES))?;
        Ok(Formula { node, occ, env })
    }

    /// Returns `⊥` as a formula of the search.
    const fn bottom(&self) -> Formula {
        Formula {
            node: self.falsity,
            occ: NONE,
            env: NONE,
        }
    }

    /// Returns what formula `f` is, its occurrence's kind or `⊥`.
    fn kind(&self, f: Formula) -> Kind {
        if f.occ == NONE {
            Kind::False
        } else {
            self.matrix.occs[f.occ as usize].kind
        }
    }

    /// Returns the Skolem term of the δ-formula `f`: its symbol applied to
    /// the terms of the binders around it, outermost first.
    fn skolem(&mut self, f: Formula, k: u32) -> u32 {
        // A δ-formula whose variable no literal holds has no symbol: one
        // past the matrix's names its Skolem terms, which no literal shares.
        let symbol = self
            .matrix
            .symbols
            .iter()
            .position(|s| *s == Symbol::Skolem(k))
            .map_or_else(|| self.matrix.symbols.len() as u32 + k, |at| at as u32);
        // The terms of the γ-formulas around, outermost first.
        let slots = self.matrix.env(f.occ);
        let mut args = Vec::with_capacity(slots.len());
        for slot in slots {
            let binder = self.matrix.slot_occ[slot as usize];
            let mut e = f.env;
            let mut found = None;
            while e != NONE {
                let (parent, occ, tm, _) = self.envs[e as usize];
                if occ == binder {
                    found = Some(tm);
                    break;
                }
                e = parent;
            }
            args.push(found.unwrap_or_else(|| self.intern(Tm::Free)));
        }
        self.intern(Tm::App(symbol, args.into_boxed_slice()))
    }

    /// Returns whether eigenvariable `e` occurs in a formula of `left` or
    /// in `goal`.
    fn occurs(&mut self, e: u32, left: &[Formula], goal: Formula) -> bool {
        let term = self.formulas.interner().term(FoTerm::Eigen(Eigen::new(e)));
        let roots = left.iter().map(|f| f.node).chain([goal.node]);
        self.formulas.mentions(roots, term)
    }

    /// Returns the eigenvariable a δ-rule on `f` takes in the sequent
    /// `left ⊢ goal`, `f` among them: its Skolem term's, where that is
    /// mapped and occurs nowhere there, else a fresh one, which the caller
    /// maps (`Some` with whether to map). `None` for a δ-formula on the
    /// left whose eigenvariable is in use: what it would add is there.
    fn eigen_for(
        &mut self,
        f: Formula,
        k: u32,
        left: &[Formula],
        goal: Formula,
    ) -> Option<(u32, u32, bool)> {
        let key = self.skolem(f, k);
        match self.mapped(key) {
            Some((e, true)) if !self.occurs(e, left, goal) => Some((key, e, false)),
            Some((_, true)) if self.matrix.occs[f.occ as usize].left => None,
            // A constant the branch took, or one in use: a fresh one.
            _ => {
                let e = self.fresh_eigen();
                Some((key, e, true))
            }
        }
    }

    /// Returns a new eigenvariable's number. Past the last number it gives
    /// up the search: an arena may hold eigenvariables up to it.
    fn fresh_eigen(&mut self) -> u32 {
        let e = self.next_eigen;
        match self.next_eigen.checked_add(1) {
            Some(next) => self.next_eigen = next,
            None => self.out_of_numbers = true,
        }
        e
    }

    /// Counts a unit of the search's work, and asks the stop every
    /// [`POLL`] units.
    fn tick(&mut self) -> Result<(), Error> {
        self.tried += 1;
        self.work(1)
    }

    /// Counts `units` of work, and asks the stop once [`POLL`] units
    /// passed since it was last asked.
    fn work(&mut self, units: u64) -> Result<(), Error> {
        self.since = self.since.saturating_add(units);
        if self.since >= POLL {
            let since = std::mem::take(&mut self.since);
            if (self.stop)(Progress::new(Phase::ReadBack, since, self.tried)) {
                return Err(Error::Refused(Refusal::stopped(Phase::ReadBack)));
            }
        }
        Ok(())
    }

    /// Adds `bytes` to what the search holds, refusing past the memory
    /// bound ([`Refusal::Memory`]).
    fn charge(&mut self, bytes: usize) -> Result<(), Error> {
        self.held = self.held.saturating_add(bytes);
        let total = self
            .held
            .saturating_add(self.instances.len().saturating_mul(INSTANCE_BYTES));
        match self.memory {
            Some(limit) if total as u64 > limit => Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: limit,
                needed_bytes: None,
            })),
            _ => Ok(()),
        }
    }

    /// Adds a step and returns its index; the step is charged to the
    /// memory bound.
    fn step(&mut self, step: Step) -> Result<u32, Error> {
        let ids =
            step.left.len() + step.added.iter().map(Vec::len).sum::<usize>() + step.premises.len();
        self.charge(size_of::<Step>() + ids * size_of::<NodeId>())?;
        self.steps.push(step);
        Ok((self.steps.len() - 1) as u32)
    }

    /// Searches a derivation of `left ⊢ goal` (`left` each formula once)
    /// and returns its last step, `None` where there is none within the
    /// bounds.
    fn prove(&mut self, left: &[Formula], goal: Formula, depth: u32) -> Result<Option<u32>, Error> {
        if self.gave_up || self.out_of_numbers {
            return Ok(None);
        }
        self.tick()?;
        if self.tried > STEPS || depth > self.depth {
            self.gave_up = true;
            return Ok(None);
        }
        let nodes: Vec<NodeId> = left.iter().map(|f| f.node).collect();
        let mut key = nodes.clone();
        key.sort_unstable();
        let key = (key, goal.node);
        if self.branch.contains(&key) {
            return Ok(None);
        }
        let leaf = |rule: Rule, principal: Option<(Side, NodeId)>| Step {
            left: nodes.clone(),
            goal: goal.node,
            rule,
            principal,
            term: None,
            added: Vec::new(),
            premises: Vec::new(),
        };
        // The leaves.
        if nodes.contains(&goal.node) {
            return Ok(Some(self.step(leaf(Rule::Axiom, None))?));
        }
        if self.logic != Logic::Minimal && nodes.contains(&self.falsity) {
            let s = leaf(Rule::FalseLeft, Some((Side::Left, self.falsity)));
            return Ok(Some(self.step(s)?));
        }
        if self.kind(goal) == Kind::True {
            let s = leaf(Rule::TrueRight, Some((Side::Right, goal.node)));
            return Ok(Some(self.step(s)?));
        }
        // A level copies its sequent: work and memory by its size.
        self.work((left.len() / HYPOTHESES_PER_UNIT) as u64)?;
        let frame = left.len().saturating_mul(LEVEL_BYTES);
        self.charge(frame)?;
        self.branch.insert(key.clone());
        let found = self.rules(left, &nodes, goal, depth);
        self.branch.remove(&key);
        self.held = self.held.saturating_sub(frame);
        found
    }

    /// Tries the rules on `left ⊢ goal`: one invertible rule where any
    /// applies, else each non-invertible one.
    fn rules(
        &mut self,
        left: &[Formula],
        nodes: &[NodeId],
        goal: Formula,
        depth: u32,
    ) -> Result<Option<u32>, Error> {
        // Invertible rules on a hypothesis.
        for (at, &f) in left.iter().enumerate() {
            let without = || {
                let mut rest = left.to_vec();
                rest.remove(at);
                rest
            };
            match self.kind(f) {
                Kind::True => {
                    return self.unary(
                        nodes,
                        goal,
                        f,
                        Rule::TrueLeft,
                        without(),
                        &[],
                        goal,
                        None,
                        depth,
                    );
                }
                Kind::And | Kind::Iff => {
                    let (a, b) = (self.child(f, 0, None)?, self.child(f, 1, None)?);
                    let rule = if self.kind(f) == Kind::And {
                        Rule::AndLeft
                    } else {
                        Rule::IffLeft
                    };
                    return self.unary(nodes, goal, f, rule, without(), &[a, b], goal, None, depth);
                }
                Kind::Or => {
                    let (a, b) = (self.child(f, 0, None)?, self.child(f, 1, None)?);
                    let rest = without();
                    let mut ds = Vec::new();
                    for x in [a, b] {
                        let premise = with(&rest, &[x]);
                        let Some(d) = self.prove(&premise, goal, depth + 1)? else {
                            return Ok(None);
                        };
                        ds.push(d);
                    }
                    let s = Step {
                        left: nodes.to_vec(),
                        goal: goal.node,
                        rule: Rule::OrLeft,
                        principal: Some((Side::Left, f.node)),
                        term: None,
                        added: vec![vec![a.node], vec![b.node]],
                        premises: ds,
                    };
                    return Ok(Some(self.step(s)?));
                }
                Kind::Delta(k) => {
                    let Some((tm, e, map)) = self.eigen_for(f, k, left, goal) else {
                        continue;
                    };
                    if map {
                        self.eigens.push((tm, e, true));
                    }
                    let a = self.child(f, 0, Some(tm))?;
                    let term = self.formulas.interner().term(FoTerm::Eigen(Eigen::new(e)));
                    let found = self.unary(
                        nodes,
                        goal,
                        f,
                        Rule::ExistsLeft,
                        without(),
                        &[a],
                        goal,
                        Some(term),
                        depth,
                    );
                    if map {
                        self.eigens.pop();
                    }
                    return found;
                }
                Kind::Atom(_)
                | Kind::False
                | Kind::Not
                | Kind::Implies
                | Kind::Part { .. }
                | Kind::Gamma(_) => {}
            }
        }
        // Invertible rules on the goal.
        match self.kind(goal) {
            Kind::And | Kind::Iff => {
                let (a, b) = (self.child(goal, 0, None)?, self.child(goal, 1, None)?);
                let rule = if self.kind(goal) == Kind::And {
                    Rule::AndRight
                } else {
                    Rule::IffRight
                };
                let mut ds = Vec::new();
                for x in [a, b] {
                    let Some(d) = self.prove(left, x, depth + 1)? else {
                        return Ok(None);
                    };
                    ds.push(d);
                }
                let s = Step {
                    left: nodes.to_vec(),
                    goal: goal.node,
                    rule,
                    principal: Some((Side::Right, goal.node)),
                    term: None,
                    added: vec![Vec::new(), Vec::new()],
                    premises: ds,
                };
                return Ok(Some(self.step(s)?));
            }
            Kind::Implies | Kind::Part { .. } | Kind::Not => {
                let a = self.child(goal, 0, None)?;
                let b = if self.kind(goal) == Kind::Not {
                    self.bottom()
                } else {
                    self.child(goal, 1, None)?
                };
                let rule = if self.kind(goal) == Kind::Not {
                    Rule::NotRight
                } else {
                    Rule::ImpliesRight
                };
                return self.right_unary(nodes, left, goal, rule, &[a], b, None, depth);
            }
            Kind::Delta(k) => {
                let Some((tm, e, map)) = self.eigen_for(goal, k, left, goal) else {
                    return Ok(None);
                };
                if map {
                    self.eigens.push((tm, e, true));
                }
                let a = self.child(goal, 0, Some(tm))?;
                let term = self.formulas.interner().term(FoTerm::Eigen(Eigen::new(e)));
                let found = self.right_unary(
                    nodes,
                    left,
                    goal,
                    Rule::ForallRight,
                    &[],
                    a,
                    Some(term),
                    depth,
                );
                if map {
                    self.eigens.pop();
                }
                return found;
            }
            Kind::Atom(_) | Kind::True | Kind::False | Kind::Or | Kind::Gamma(_) => {}
        }
        // The choices: the goal's whose witnesses the branch has, then each
        // hypothesis's, then the goal's whose witnesses name a Skolem term
        // the branch has not met. That one becomes a constant here, so a
        // hypothesis whose instance would bring its δ-rule in comes first.
        match self.kind(goal) {
            Kind::Or => {
                for (k, rule) in [(0, Rule::OrRight1), (1, Rule::OrRight2)] {
                    let a = self.child(goal, k, None)?;
                    if let Some(d) =
                        self.right_unary(nodes, left, goal, rule, &[], a, None, depth)?
                    {
                        return Ok(Some(d));
                    }
                }
            }
            Kind::Gamma(slot) => {
                if let Some(d) = self.exists_right(nodes, left, goal, slot, true, depth)? {
                    return Ok(Some(d));
                }
            }
            Kind::Atom(_)
            | Kind::True
            | Kind::False
            | Kind::Not
            | Kind::And
            | Kind::Implies
            | Kind::Iff
            | Kind::Part { .. }
            | Kind::Delta(_) => {}
        }
        // The implications first, then the quantified hypotheses: an
        // implication uses what the sequent holds, an instance adds to it.
        for (pass, (at, &f)) in left
            .iter()
            .enumerate()
            .map(|x| (0, x))
            .chain(left.iter().enumerate().map(|x| (1, x)))
        {
            match self.kind(f) {
                Kind::Implies | Kind::Part { .. } | Kind::Not if pass == 0 => {
                    let a = self.child(f, 0, None)?;
                    let b = if self.kind(f) == Kind::Not {
                        self.bottom()
                    } else {
                        self.child(f, 1, None)?
                    };
                    if let Some(d) = self.implication_left(nodes, left, at, goal, a, b, depth)? {
                        return Ok(Some(d));
                    }
                }
                Kind::Gamma(slot) if pass == 1 => {
                    for tm in self.ordered(slot) {
                        let given = self.constants(tm);
                        let a = self.child(f, 0, Some(tm))?;
                        if nodes.contains(&a.node) {
                            self.eigens.truncate(self.eigens.len() - given);
                            continue;
                        }
                        let term = self.instance_term(tm);
                        let premise = with(left, &[a]);
                        let found = self.prove(&premise, goal, depth + 1);
                        self.eigens.truncate(self.eigens.len() - given);
                        if let Some(d) = found? {
                            let s = Step {
                                left: nodes.to_vec(),
                                goal: goal.node,
                                rule: Rule::ForallLeft,
                                principal: Some((Side::Left, f.node)),
                                term: Some(term),
                                added: vec![vec![a.node]],
                                premises: vec![d],
                            };
                            return Ok(Some(self.step(s)?));
                        }
                    }
                }
                Kind::Implies
                | Kind::Part { .. }
                | Kind::Not
                | Kind::Gamma(_)
                | Kind::Atom(_)
                | Kind::True
                | Kind::False
                | Kind::And
                | Kind::Or
                | Kind::Iff
                | Kind::Delta(_) => {}
            }
        }
        if let Kind::Gamma(slot) = self.kind(goal) {
            return self.exists_right(nodes, left, goal, slot, false, depth);
        }
        Ok(None)
    }

    /// Tries `∃R` on `goal`, the γ-formula of `slot`, with each candidate
    /// whose Skolem terms all have their eigenvariables on the branch
    /// (`ready`), or with each of the others, the variable left unbound
    /// last.
    fn exists_right(
        &mut self,
        nodes: &[NodeId],
        left: &[Formula],
        goal: Formula,
        slot: u32,
        ready: bool,
        depth: u32,
    ) -> Result<Option<u32>, Error> {
        let candidates: Vec<u32> = self
            .ordered(slot)
            .into_iter()
            .filter(|&tm| (self.tms[tm as usize] != Tm::Free && self.ready(tm)) == ready)
            .collect();
        for tm in candidates {
            let given = self.constants(tm);
            let a = self.child(goal, 0, Some(tm))?;
            let term = self.instance_term(tm);
            let found = self.right_unary(
                nodes,
                left,
                goal,
                Rule::ExistsRight,
                &[],
                a,
                Some(term),
                depth,
            );
            self.eigens.truncate(self.eigens.len() - given);
            if let Some(d) = found? {
                return Ok(Some(d));
            }
        }
        Ok(None)
    }

    /// Returns the term of the arena a quantifier rule takes for `tm`.
    fn instance_term(&mut self, tm: u32) -> FoTermId {
        self.term(tm)
    }

    /// A rule on hypothesis `f` with one premise: `rest` and `added` left
    /// of `⊢`, `goal` right.
    #[expect(clippy::too_many_arguments, reason = "a rule's parts")]
    fn unary(
        &mut self,
        nodes: &[NodeId],
        conclusion_goal: Formula,
        f: Formula,
        rule: Rule,
        rest: Vec<Formula>,
        added: &[Formula],
        goal: Formula,
        term: Option<FoTermId>,
        depth: u32,
    ) -> Result<Option<u32>, Error> {
        let premise = with(&rest, added);
        let Some(d) = self.prove(&premise, goal, depth + 1)? else {
            return Ok(None);
        };
        let s = Step {
            left: nodes.to_vec(),
            goal: conclusion_goal.node,
            rule,
            principal: Some((Side::Left, f.node)),
            term,
            added: vec![added.iter().map(|x| x.node).collect()],
            premises: vec![d],
        };
        Ok(Some(self.step(s)?))
    }

    /// A rule on the goal with one premise: `left` and `added` left of
    /// `⊢`, `premise_goal` right.
    #[expect(clippy::too_many_arguments, reason = "a rule's parts")]
    fn right_unary(
        &mut self,
        nodes: &[NodeId],
        left: &[Formula],
        goal: Formula,
        rule: Rule,
        added: &[Formula],
        premise_goal: Formula,
        term: Option<FoTermId>,
        depth: u32,
    ) -> Result<Option<u32>, Error> {
        let premise = with(left, added);
        let Some(d) = self.prove(&premise, premise_goal, depth + 1)? else {
            return Ok(None);
        };
        let s = Step {
            left: nodes.to_vec(),
            goal: goal.node,
            rule,
            principal: Some((Side::Right, goal.node)),
            term,
            added: vec![added.iter().map(|x| x.node).collect()],
            premises: vec![d],
        };
        Ok(Some(self.step(s)?))
    }

    /// G3i's `→L` (or `¬L`) on hypothesis `at`, `a → b`: `left ⊢ a`, the
    /// implication kept, and `left`, `b` without it `⊢ goal`.
    #[expect(clippy::too_many_arguments, reason = "a rule's parts")]
    fn implication_left(
        &mut self,
        nodes: &[NodeId],
        left: &[Formula],
        at: usize,
        goal: Formula,
        a: Formula,
        b: Formula,
        depth: u32,
    ) -> Result<Option<u32>, Error> {
        let f = left[at];
        let Some(first) = self.prove(left, a, depth + 1)? else {
            return Ok(None);
        };
        let mut rest = left.to_vec();
        rest.remove(at);
        let second_left = with(&rest, &[b]);
        let Some(second) = self.prove(&second_left, goal, depth + 1)? else {
            return Ok(None);
        };
        let rule = if self.kind(f) == Kind::Not {
            Rule::NotLeft
        } else {
            Rule::ImpliesLeft
        };
        let s = Step {
            left: nodes.to_vec(),
            goal: goal.node,
            rule,
            principal: Some((Side::Left, f.node)),
            term: None,
            added: vec![Vec::new(), vec![b.node]],
            premises: vec![first, second],
        };
        Ok(Some(self.step(s)?))
    }
}

/// Returns `rest` with each of `added` not among it.
fn with(rest: &[Formula], added: &[Formula]) -> Vec<Formula> {
    let mut out = rest.to_vec();
    for &x in added {
        if !out.iter().any(|f| f.node == x.node) {
            out.push(x);
        }
    }
    out
}

/// The derivation of LJ made from the steps.
struct Emit<'s> {
    /// The inferences, premises first.
    inferences: Vec<Inference>,
    /// The bytes the inferences hold, the derivation's one bound
    /// (`limits.derivation_bytes` bounds what is drawn).
    held: usize,
    /// The memory bound, `None` for none.
    memory: Option<u64>,
    /// The caller's stop.
    stop: &'s mut dyn FnMut(Progress) -> bool,
}

impl Builder for Emit<'_> {
    fn inference(&mut self, d: InfId) -> &mut Inference {
        &mut self.inferences[d.index()]
    }

    fn step(
        &mut self,
        left: Vec<NodeId>,
        right: Vec<NodeId>,
        rule: Rule,
        principal: Option<(Side, usize)>,
        premise: InfId,
    ) -> Result<InfId, Error> {
        self.push(left, right, rule, principal, vec![premise], None)
    }
}

impl<'s> Emit<'s> {
    /// The emitter of a derivation within `limits`.
    fn new(limits: &Limits, stop: &'s mut dyn FnMut(Progress) -> bool) -> Self {
        Self {
            inferences: Vec::new(),
            held: 0,
            memory: limits.memory_bytes,
            stop,
        }
    }

    /// Adds an inference and returns its id, refusing past the bound or
    /// once the stop fires.
    fn push(
        &mut self,
        left: Vec<NodeId>,
        right: Vec<NodeId>,
        rule: Rule,
        principal: Option<(Side, usize)>,
        premises: Vec<InfId>,
        term: Option<FoTermId>,
    ) -> Result<InfId, Error> {
        let n = self.inferences.len() as u64;
        derivation::ask_read_back(&mut *self.stop, n)?;
        // The derivation is held in memory too, whatever its own bound.
        self.held = self.held.saturating_add(
            size_of::<Inference>()
                + (left.len() + right.len()) * size_of::<NodeId>()
                + premises.len() * size_of::<InfId>(),
        );
        if let Some(limit) = self.memory
            && self.held as u64 > limit
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: limit,
                needed_bytes: None,
            }));
        }
        derivation::add(
            &mut self.inferences,
            Inference {
                term,
                left,
                right,
                rule,
                principal,
                premises,
            },
        )
    }

    /// Returns the derivation of the steps below `root`, premises first,
    /// on a stack of its own.
    fn derivation(&mut self, steps: &[Step], root: u32) -> Result<InfId, Error> {
        /// What the walk has yet to do with a step.
        enum Task {
            /// Make its premises, then it.
            Enter(u32),
            /// Make it from its premises' derivations, the last made.
            Exit(u32),
        }
        let mut tasks = vec![Task::Enter(root)];
        let mut made: Vec<InfId> = Vec::new();
        while let Some(task) = tasks.pop() {
            match task {
                Task::Enter(s) => {
                    tasks.push(Task::Exit(s));
                    tasks.extend(
                        steps[s as usize]
                            .premises
                            .iter()
                            .rev()
                            .map(|&p| Task::Enter(p)),
                    );
                }
                Task::Exit(s) => {
                    let step = &steps[s as usize];
                    let premises = made.split_off(made.len() - step.premises.len());
                    let d = self.emit(step, &premises)?;
                    made.push(d);
                }
            }
        }
        Ok(made
            .pop()
            .unwrap_or_else(|| unreachable!("the root's derivation")))
    }

    /// Returns the derivation of `step` from its premises' derivations.
    fn emit(&mut self, step: &Step, d: &[InfId]) -> Result<InfId, Error> {
        let left = step.left.clone();
        let goal = step.goal;
        let at_left = |p: NodeId| left.iter().position(|&x| x == p).map(|at| (Side::Left, at));
        let without = |p: NodeId| {
            let mut rest = left.clone();
            if let Some(at) = rest.iter().position(|&x| x == p) {
                rest.remove(at);
            }
            rest
        };
        match (step.rule, step.principal) {
            (Rule::Axiom, _) => {
                let ax = self.push(vec![goal], vec![goal], Rule::Axiom, None, Vec::new(), None)?;
                self.weaken_to(ax, &left)
            }
            (Rule::FalseLeft | Rule::TrueRight, Some((side, p))) => {
                let principal = match side {
                    Side::Left => at_left(p),
                    Side::Right => Some((Side::Right, 0)),
                };
                self.push(left, vec![goal], step.rule, principal, Vec::new(), None)
            }
            (Rule::ForallLeft, Some((Side::Left, p))) => {
                // G3's `∀L` keeps its principal formula in the premise.
                let mut want = left.clone();
                want.extend_from_slice(&step.added[0]);
                let premise = self.weaken_to(d[0], &want)?;
                self.push(
                    left.clone(),
                    vec![goal],
                    Rule::ForallLeft,
                    at_left(p),
                    vec![premise],
                    step.term,
                )
            }
            (Rule::ImpliesLeft | Rule::NotLeft, Some((Side::Left, p))) => {
                // Split: `Δ₁ ⊢ a` (Δ₁ the premise's hypotheses, the
                // implication among them) and `Δ₂, b ⊢ goal`, then the
                // copies contracted.
                let first_left = self.inferences[d[0].index()].left.clone();
                let delta2 = without(p);
                let mut second_want = delta2.clone();
                second_want.extend_from_slice(&step.added[1]);
                let second = self.weaken_to(d[1], &second_want)?;
                let mut conclusion = first_left;
                conclusion.extend_from_slice(&delta2);
                conclusion.push(p);
                let at = conclusion.len() - 1;
                let m = self.push(
                    conclusion,
                    vec![goal],
                    step.rule,
                    Some((Side::Left, at)),
                    vec![d[0], second],
                    None,
                )?;
                self.contract_to(m, &left)
            }
            (rule, Some((Side::Left, p))) => {
                // A rule on a hypothesis, the principal not kept.
                let rest = without(p);
                let mut premises = Vec::new();
                for (k, &di) in d.iter().enumerate() {
                    let mut want = rest.clone();
                    want.extend_from_slice(&step.added[k]);
                    premises.push(self.weaken_to(di, &want)?);
                }
                self.push(
                    left.clone(),
                    vec![goal],
                    rule,
                    at_left(p),
                    premises,
                    step.term,
                )
            }
            (rule, Some((Side::Right, _))) => {
                let mut premises = Vec::new();
                for (k, &di) in d.iter().enumerate() {
                    let mut want = left.clone();
                    want.extend_from_slice(&step.added[k]);
                    premises.push(self.weaken_to(di, &want)?);
                }
                self.push(
                    left,
                    vec![goal],
                    rule,
                    Some((Side::Right, 0)),
                    premises,
                    step.term,
                )
            }
            (_, None) => unreachable!("every rule but the axiom has a principal formula"),
        }
    }
}
