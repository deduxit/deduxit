// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The read-back of a connection proof as a derivation of LK in G3's
//! form.
//!
//! The proof is an oracle, not a derivation: the substitution and the
//! literals it used, each in its copy. From them the read-back takes the
//! instances every `∀` hypothesis and `∃` goal needs (the values of a
//! copy's variables, the Skolem term of a δ-formula read as an
//! eigenvariable) and builds a derivation of the sequent as written by a
//! fixed order of rules on each branch:
//! 1. an axiom where a formula stands on both sides (with weakenings down
//!    to `A ⊢ A`), `⊤R` or `⊥L`;
//! 2. a rule of one premise and no eigenvariable;
//! 3. `∀R` and `∃L`, with the eigenvariable of the formula's Skolem term;
//! 4. `∀L` and `∃R` keeping their principal formula, with each needed
//!    witness whose eigenvariables the branch has introduced;
//! 5. a rule of two premises.
//!
//! Only formulas above a literal the proof used are taken apart; the rest
//! is weakened at the axioms. A δ-formula whose eigenvariable the branch
//! has is a copy of one taken apart there, and is left alone.
//!
//! Nothing here is trusted: the caller checks the derivation, and a branch
//! no rule closes is [`Error::ReadBack`], a defect, never a verdict. Every
//! walk keeps a stack of its own.

use super::matrix::{Kind, Matrix, NONE, Symbol};
use super::sizes::TermBudget;
use super::terms::{Head, Seen, Terms};
use crate::Error;
use crate::first_order::{Eigen, FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal, Space};
use crate::linear::proofs::InfId;
use crate::ordinary::{Derivation, Formulas, Inference, Logic, Node, NodeId, Rule, Sequent, Side};

// The arena's tables and indices grow by doubling, so each figure below is
// twice what an entry takes: with them at their lengths the estimate was
// 1.5 to 1.8 times short of the resident memory a read-back refused at.

/// The bytes the arena holds a node, a first-order term or an argument
/// list: the entry and its share of the index that makes each once.
const ARENA_NODE_BYTES: u64 = 96;

/// The bytes the arena holds an argument of a list: the table's and the
/// index's copy.
const ARGUMENT_BYTES: u64 = 16;

/// The bytes the arena holds an atom beside its key's text.
const ARENA_ATOM_BYTES: u64 = 128;

/// The bytes an atom's key takes per term its arguments unfold to: a name
/// and a separator, about, in the atom table and in its index.
const KEY_BYTES: u64 = 12;

/// A formula on a branch: its occurrence, its side, the formula as the
/// instances made it, the witnesses of the γ-formulas around it and,
/// for a γ-formula, the witnesses it was instantiated with.
#[derive(Clone, Debug)]
struct Item {
    /// The occurrence.
    occ: u32,
    /// Its side: true for left of `⊢`.
    left: bool,
    /// The formula.
    node: NodeId,
    /// The witnesses of the γ-formulas around it, a list of
    /// [`ReadBack::envs`], [`NONE`] for none.
    env: u32,
    /// The witnesses it was instantiated with, for a γ-formula.
    done: Vec<FoTermId>,
    /// Whether it is left alone: a δ-formula the branch took apart already.
    inert: bool,
}

/// A branch: its formulas and the eigenvariables it introduced.
#[derive(Clone, Debug, Default)]
struct Branch {
    /// The formulas, hypotheses and goals in the order they came.
    items: Vec<Item>,
    /// The eigenvariables of `∀R` and `∃L` below it.
    introduced: Vec<FoTermId>,
    /// The eigenvariables of Skolem terms a witness below it holds though
    /// the branch never introduced them: constants here, which no `∀R` or
    /// `∃L` above may introduce.
    constants: Vec<FoTermId>,
}

/// A step of the construction.
enum Frame {
    /// Derive the branch.
    Expand(Branch),
    /// Make the inference whose premises are the last results.
    Emit {
        /// Its hypotheses.
        left: Vec<NodeId>,
        /// Its goals.
        right: Vec<NodeId>,
        /// Its rule.
        rule: Rule,
        /// Its principal formula.
        principal: Option<(Side, usize)>,
        /// Its term.
        term: Option<FoTermId>,
        /// How many premises.
        premises: usize,
    },
}

/// The state of a read-back.
struct ReadBack<'m, 's> {
    /// The matrix.
    matrix: &'m Matrix,
    /// The terms and substitution of the proof.
    terms: &'m Terms,
    /// The derivation's arena.
    formulas: Formulas,
    /// The witness lists: a term and the list before it, each list once,
    /// so that a list is known by its index.
    envs: Vec<(FoTermId, u32)>,
    /// The index of every witness list, by its last term and the list
    /// before it.
    env_ids: HashMap<(u32, FoTermId), u32>,
    /// Whether each occurrence lies above a literal the proof used.
    relevant: Vec<bool>,
    /// The witnesses each γ-formula needs, by its occurrence and the
    /// witnesses of the γ-formulas around it.
    needed: HashMap<(u32, u32), Vec<FoTermId>>,
    /// Every witness [`needed`](Self::needed) holds, with its key.
    needed_seen: HashSet<(u32, u32, FoTermId)>,
    /// The eigenvariable of each Skolem term, by its index and arguments.
    skolems: HashMap<(u32, Vec<FoTermId>), FoTermId>,
    /// The terms the proof's terms became, by pattern and copy.
    converted: HashMap<(u32, u32), FoTermId>,
    /// The number of the next eigenvariable.
    next_eigen: u32,
    /// The eigenvariables numbered at or past it are Skolem terms'.
    first_skolem: u32,
    /// The term an unbound variable stands for, once one is met: an
    /// eigenvariable no rule introduces, a constant to the checker.
    default: Option<FoTermId>,
    /// The number of that eigenvariable.
    default_number: u32,
    /// The inferences made, premises first.
    inferences: Vec<Inference>,
    /// The bytes they hold, against the memory bound.
    bytes: u64,
    /// The caller's limits.
    limits: &'s Limits,
    /// Every instance built, by body and term: building one writes its
    /// atoms' keys, which a shared witness makes long.
    instances: HashMap<(NodeId, FoTermId), NodeId>,
    /// The arena's terms, held to `limits.terms`.
    budget: TermBudget,
}

/// Reads the proof back as a derivation of LK of `sequent`, unchecked.
///
/// # Errors
///
/// [`Error::ReadBack`] where a branch stays open, a defect;
/// [`Refusal::Memory`] past `limits.memory_bytes`,
/// [`Refusal::Terms`] where an instance's atoms would pass `limits.terms`,
/// [`Refusal::Index`] past `u32` inferences and [`Refusal::Stopped`] when
/// the stop fires.
pub(super) fn read_back(
    sequent: &Sequent,
    matrix: &Matrix,
    used: &[(u32, u32)],
    terms: &Terms,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Derivation, Error> {
    let formulas = sequent.formulas().clone();
    // The eigenvariables of the arena are parameters: number past them.
    let base = formulas
        .first_order()
        .terms
        .iter()
        .filter_map(|t| match t {
            FoTerm::Eigen(e) => Some(e.get().saturating_add(1)),
            FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
        })
        .max()
        .unwrap_or(0);
    let budget = TermBudget::new(&formulas, limits.terms);
    let mut back = ReadBack {
        matrix,
        terms,
        formulas,
        envs: Vec::new(),
        env_ids: HashMap::default(),
        relevant: vec![false; matrix.occs.len()],
        needed: HashMap::default(),
        needed_seen: HashSet::default(),
        skolems: HashMap::default(),
        converted: HashMap::default(),
        next_eigen: base.saturating_add(1),
        first_skolem: base.saturating_add(1),
        default: None,
        default_number: base,
        inferences: Vec::new(),
        bytes: 0,
        limits,
        instances: HashMap::default(),
        budget,
    };
    back.oracle(used, stop)?;
    let mut root = Branch::default();
    for &occ in &matrix.roots {
        let o = matrix.occs[occ as usize];
        root.items.push(Item {
            occ,
            left: o.left,
            node: o.node,
            env: NONE,
            done: Vec::new(),
            inert: false,
        });
    }
    let (left, right) = sides(&root);
    back.derive(root, stop)?;
    let inferences = std::mem::take(&mut back.inferences);
    Ok(Derivation::from_parts(
        Logic::Classical,
        back.formulas,
        (left, right),
        inferences,
    ))
}

/// Returns the hypotheses and the goals of a branch, in order.
fn sides(branch: &Branch) -> (Vec<NodeId>, Vec<NodeId>) {
    let left = branch.items.iter().filter(|i| i.left).map(|i| i.node);
    let right = branch.items.iter().filter(|i| !i.left).map(|i| i.node);
    (left.collect(), right.collect())
}

/// Returns the position of item `at` among the items of its side.
fn position(branch: &Branch, at: usize) -> (Side, usize) {
    let left = branch.items[at].left;
    let before = branch.items[..at].iter().filter(|i| i.left == left).count();
    (if left { Side::Left } else { Side::Right }, before)
}

impl ReadBack<'_, '_> {
    /// Returns the witness list `before` with `t` after it.
    fn env_with(&mut self, before: u32, t: FoTermId) -> u32 {
        if let Some(&id) = self.env_ids.get(&(before, t)) {
            return id;
        }
        self.envs.push((t, before));
        let id = (self.envs.len() - 1) as u32;
        self.env_ids.insert((before, t), id);
        id
    }

    /// Records that the γ-occurrence `occ` under the witness list `env`
    /// needs the witness `t`.
    fn need(&mut self, occ: u32, env: u32, t: FoTermId) {
        if self.needed_seen.insert((occ, env, t)) {
            self.needed.entry((occ, env)).or_default().push(t);
        }
    }

    /// Takes from the proof the occurrences it reaches and the witnesses
    /// its copies need, asking `stop` as it goes.
    fn oracle(
        &mut self,
        used: &[(u32, u32)],
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        // `⊤` right of `⊢` and `⊥` left of it close every path through
        // them, with no literal.
        for o in 0..self.matrix.occs.len() {
            let occ = self.matrix.occs[o];
            if matches!(
                (occ.kind, occ.left),
                (Kind::True, false) | (Kind::False, true)
            ) {
                let mut up = o as u32;
                while up != NONE && !self.relevant[up as usize] {
                    self.relevant[up as usize] = true;
                    up = self.matrix.occs[up as usize].parent;
                }
                // The γ-formulas around it need a witness, any: the term
                // an unbound variable stands for.
                let env = self.matrix.env(o as u32);
                let default = self.default();
                let mut list = NONE;
                for &slot in &env {
                    self.need(self.matrix.slot_occ[slot as usize], list, default);
                    list = self.env_with(list, default);
                }
            }
        }
        let mut seen: HashSet<(u32, u32)> = HashSet::default();
        for (n, &(lit, copy)) in used.iter().enumerate() {
            if n % 1024 == 1023 && stop(Progress::new(Phase::ReadBack, 1024, n as u64)) {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::ReadBack,
                }));
            }
            if !seen.insert((lit, copy)) {
                continue;
            }
            let occ = self.matrix.literals[lit as usize].occ;
            let mut o = occ;
            while o != NONE && !self.relevant[o as usize] {
                self.relevant[o as usize] = true;
                o = self.matrix.occs[o as usize].parent;
            }
            let env = self.matrix.env(occ);
            let values: Vec<FoTermId> = env
                .iter()
                .map(|&slot| {
                    let v = self.terms.var(self.matrix, copy, slot);
                    self.var_term(v)
                })
                .collect();
            let mut list = NONE;
            for (i, &slot) in env.iter().enumerate() {
                self.need(self.matrix.slot_occ[slot as usize], list, values[i]);
                list = self.env_with(list, values[i]);
            }
        }
        Ok(())
    }

    /// Returns the term variable `v` is bound to, the default where it is
    /// unbound.
    fn var_term(&mut self, v: u32) -> FoTermId {
        let (p, c) = self.terms.bound[v as usize];
        if p == NONE {
            self.default()
        } else {
            self.convert((p, c))
        }
    }

    /// Returns the term of the arena a term of the proof stands for under
    /// the substitution: a Skolem term as its eigenvariable.
    fn convert(&mut self, term: (u32, u32)) -> FoTermId {
        // Each entry: a term, and whether its arguments are done.
        let mut stack = vec![(term, false)];
        while let Some((t, done)) = stack.pop() {
            if self.converted.contains_key(&t) {
                continue;
            }
            let made = match self.terms.head(self.matrix, t) {
                Head::Var(_) => self.default(),
                Head::App(f, (s, len), c) => {
                    let args: Vec<(u32, u32)> = self.matrix.pat_args
                        [s as usize..(s + len) as usize]
                        .iter()
                        .map(|&a| (a, c))
                        .collect();
                    if !done && args.iter().any(|a| !self.converted.contains_key(a)) {
                        stack.push((t, true));
                        stack.extend(args.iter().map(|&a| (a, false)));
                        continue;
                    }
                    let items: Vec<FoTermId> = args.iter().map(|a| self.converted[a]).collect();
                    match self.matrix.symbols[f as usize] {
                        Symbol::Function(symbol) => {
                            let list = self.formulas.interner().list(&items);
                            self.formulas.interner().term(FoTerm::App(symbol, list))
                        }
                        Symbol::Parameter(term) => term,
                        Symbol::Skolem(k) => self.skolem(k, items),
                    }
                }
            };
            self.converted.insert(t, made);
        }
        self.converted[&term]
    }

    /// Returns the term an unbound variable stands for, made at the first
    /// call.
    fn default(&mut self) -> FoTermId {
        if let Some(term) = self.default {
            return term;
        }
        let term = self
            .formulas
            .interner()
            .term(FoTerm::Eigen(Eigen::new(self.default_number)));
        self.default = Some(term);
        term
    }

    /// Returns the eigenvariable of the Skolem term `k` applied to `args`.
    fn skolem(&mut self, k: u32, args: Vec<FoTermId>) -> FoTermId {
        if let Some(&e) = self.skolems.get(&(k, args.clone())) {
            return e;
        }
        let e = self
            .formulas
            .interner()
            .term(FoTerm::Eigen(Eigen::new(self.next_eigen)));
        self.next_eigen = self.next_eigen.saturating_add(1);
        self.skolems.insert((k, args), e);
        e
    }

    /// Returns the witnesses of a list, outermost first.
    fn env_values(&self, mut env: u32) -> Vec<FoTermId> {
        let mut values = Vec::new();
        while env != NONE {
            let (t, before) = self.envs[env as usize];
            values.push(t);
            env = before;
        }
        values.reverse();
        values
    }

    /// Returns whether every eigenvariable of a Skolem term in `t` is one
    /// the branch introduced or holds as a constant.
    fn available(&self, t: FoTermId, branch: &Branch) -> bool {
        let tables = self.formulas.first_order();
        let mut stack = vec![t];
        let mut seen = Seen::new();
        while let Some(u) = stack.pop() {
            if !seen.first(u) {
                continue;
            }
            match tables.terms[u.index()] {
                FoTerm::Eigen(e) if e.get() >= self.first_skolem => {
                    if !branch.introduced.contains(&u) && !branch.constants.contains(&u) {
                        return false;
                    }
                }
                FoTerm::App(_, args) => stack.extend_from_slice(tables.list(args)),
                FoTerm::Eigen(_) | FoTerm::Bound(_) | FoTerm::Meta(_) => {}
            }
        }
        true
    }

    /// Returns the eigenvariables of Skolem terms in `t`.
    fn skolem_eigens(&self, t: FoTermId) -> Vec<FoTermId> {
        let tables = self.formulas.first_order();
        let (mut found, mut stack) = (Vec::new(), vec![t]);
        let mut seen = Seen::new();
        while let Some(u) = stack.pop() {
            if !seen.first(u) {
                continue;
            }
            match tables.terms[u.index()] {
                FoTerm::Eigen(e) if e.get() >= self.first_skolem => {
                    if !found.contains(&u) {
                        found.push(u);
                    }
                }
                FoTerm::App(_, args) => stack.extend_from_slice(tables.list(args)),
                FoTerm::Eigen(_) | FoTerm::Bound(_) | FoTerm::Meta(_) => {}
            }
        }
        found
    }

    /// Returns an eigenvariable no Skolem term has.
    fn fresh_eigen(&mut self) -> FoTermId {
        let e = self
            .formulas
            .interner()
            .term(FoTerm::Eigen(Eigen::new(self.next_eigen)));
        self.next_eigen = self.next_eigen.saturating_add(1);
        e
    }

    /// Derives the root branch, on stacks of its own.
    fn derive(
        &mut self,
        root: Branch,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let mut frames = vec![Frame::Expand(root)];
        let mut results: Vec<InfId> = Vec::new();
        while let Some(frame) = frames.pop() {
            match frame {
                Frame::Expand(branch) => self.expand(branch, &mut frames, &mut results, stop)?,
                Frame::Emit {
                    left,
                    right,
                    rule,
                    principal,
                    term,
                    premises,
                } => {
                    let premises = results.split_off(results.len() - premises);
                    // `push` charges the sides it keeps, which `expand`
                    // charged while they waited.
                    let waited = (left.len() + right.len()) * size_of::<NodeId>();
                    self.bytes = self.bytes.saturating_sub(waited as u64);
                    let id = self.push(left, right, rule, principal, term, premises, stop)?;
                    results.push(id);
                }
            }
        }
        Ok(())
    }

    /// Adds an inference and returns its id, refusing past a bound or once
    /// the stop fires.
    #[expect(
        clippy::too_many_arguments,
        reason = "an inference's parts, as the checker reads them"
    )]
    fn push(
        &mut self,
        left: Vec<NodeId>,
        right: Vec<NodeId>,
        rule: Rule,
        principal: Option<(Side, usize)>,
        term: Option<FoTermId>,
        premises: Vec<InfId>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<InfId, Error> {
        let n = self.inferences.len() as u64;
        let held =
            size_of::<Inference>() + (left.len() + right.len() + premises.len()) * size_of::<u32>();
        self.bytes = self.bytes.saturating_add(held as u64);
        self.account(stop)?;
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
            term,
        });
        Ok(InfId::new(id))
    }

    /// Asks the stop, and refuses where the derivation's inferences and its
    /// arena (the instances' nodes, terms and atoms' keys) pass the memory
    /// bound.
    fn account(&mut self, stop: &mut dyn FnMut(Progress) -> bool) -> Result<(), Error> {
        let n = self.inferences.len() as u64;
        if stop(Progress::new(Phase::ReadBack, 1, n + 1)) {
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::ReadBack,
            }));
        }
        // The derivation is bounded by the memory it holds;
        // `limits.derivation_bytes` bounds what is drawn.
        if let Some(limit) = self.limits.memory_bytes
            && self.bytes.saturating_add(self.arena_bytes()) > limit
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: limit,
                needed_bytes: None,
            }));
        }
        Ok(())
    }

    /// Returns the bytes the derivation's arena holds, by its estimate: a
    /// node or a term with its share of the index, an argument list twice
    /// (the table and the index's key), an atom with its key, which writes
    /// every argument out.
    fn arena_bytes(&self) -> u64 {
        let tables = self.formulas.first_order();
        let entries = (self.formulas.len() + tables.terms.len() + tables.lists.len()) as u64;
        entries
            .saturating_mul(ARENA_NODE_BYTES)
            .saturating_add((tables.args.len() as u64).saturating_mul(ARGUMENT_BYTES))
            .saturating_add((tables.atoms.len() as u64).saturating_mul(ARENA_ATOM_BYTES))
            .saturating_add(self.budget.held().saturating_mul(KEY_BYTES))
    }

    /// Closes the branch if it can, else applies the first rule its order
    /// gives, pushing the inference and its premises.
    fn expand(
        &mut self,
        mut branch: Branch,
        frames: &mut Vec<Frame>,
        results: &mut Vec<InfId>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        // A branch is expanded before its inference is made, so a long
        // branch of quantifier steps, each an instance as large as the
        // formula, asks the stop and the memory bound here.
        self.account(stop)?;
        if let Some(id) = self.close(&branch, stop)? {
            results.push(id);
            return Ok(());
        }
        let Some(step) = self.step(&mut branch)? else {
            return Err(Error::ReadBack {
                calculus: Logic::Classical.calculus(),
                reason: "the connection proof leaves a branch open".to_owned(),
            });
        };
        let (left, right) = sides(&branch);
        // The inference waits for its premises with its sides.
        let sides_bytes = (left.len() + right.len()) * size_of::<NodeId>();
        self.bytes = self.bytes.saturating_add(sides_bytes as u64);
        frames.push(Frame::Emit {
            left,
            right,
            rule: step.rule,
            principal: Some(step.principal),
            term: step.term,
            premises: step.premises.len(),
        });
        for premise in step.premises.into_iter().rev() {
            frames.push(Frame::Expand(premise));
        }
        Ok(())
    }

    /// Closes the branch by an axiom (with weakenings), `⊤R` or `⊥L`, and
    /// returns the inference concluding it, if any closes it.
    fn close(
        &mut self,
        branch: &Branch,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Option<InfId>, Error> {
        let (left, right) = sides(branch);
        if let Some(at) = right
            .iter()
            .position(|&k| self.formulas.node(k) == Node::True)
        {
            return self
                .push(
                    left,
                    right,
                    Rule::TrueRight,
                    Some((Side::Right, at)),
                    None,
                    Vec::new(),
                    stop,
                )
                .map(Some);
        }
        if let Some(at) = left
            .iter()
            .position(|&k| self.formulas.node(k) == Node::False)
        {
            return self
                .push(
                    left,
                    right,
                    Rule::FalseLeft,
                    Some((Side::Left, at)),
                    None,
                    Vec::new(),
                    stop,
                )
                .map(Some);
        }
        let Some(&shared) = left.iter().find(|k| right.contains(k)) else {
            return Ok(None);
        };
        // `A ⊢ A`, then every other formula weakened in, in order.
        let mut id = self.push(
            vec![shared],
            vec![shared],
            Rule::Axiom,
            None,
            None,
            Vec::new(),
            stop,
        )?;
        let (mut l, mut r) = (vec![shared], vec![shared]);
        let mut skipped = (false, false);
        for item in &branch.items {
            let side = if item.left {
                &mut skipped.0
            } else {
                &mut skipped.1
            };
            if item.node == shared && !*side {
                *side = true;
                continue;
            }
            let (rule, principal) = if item.left {
                l.push(item.node);
                (Rule::WeakenLeft, (Side::Left, l.len() - 1))
            } else {
                r.push(item.node);
                (Rule::WeakenRight, (Side::Right, r.len() - 1))
            };
            id = self.push(
                l.clone(),
                r.clone(),
                rule,
                Some(principal),
                None,
                vec![id],
                stop,
            )?;
        }
        Ok(Some(id))
    }

    /// Returns the first rule the order gives for the branch, with its
    /// premises, or `None` where none applies.
    fn step(&mut self, branch: &mut Branch) -> Result<Option<Step>, Error> {
        // 1: weaken what can no longer help, so that no inference above
        // carries it: a formula above no literal the proof used, a copy
        // of a δ-formula taken apart, a γ-formula with every needed
        // witness in.
        for at in 0..branch.items.len() {
            if self.useless(branch, at) {
                let item = &branch.items[at];
                let rule = if item.left {
                    Rule::WeakenLeft
                } else {
                    Rule::WeakenRight
                };
                return Ok(Some(Step {
                    rule,
                    principal: position(branch, at),
                    term: None,
                    premises: vec![Self::replaced(branch, at, Vec::new())],
                }));
            }
        }
        // 2: one premise, no eigenvariable.
        for at in 0..branch.items.len() {
            let item = &branch.items[at];
            if !self.relevant[item.occ as usize] || item.inert {
                continue;
            }
            if let Some(step) = self.unary(branch, at)? {
                return Ok(Some(step));
            }
        }
        // 3: δ-formulas.
        for at in 0..branch.items.len() {
            let item = &branch.items[at];
            if !self.relevant[item.occ as usize] || item.inert {
                continue;
            }
            let Kind::Delta(k) = self.matrix.occs[item.occ as usize].kind else {
                continue;
            };
            let args = self.env_values(item.env);
            let mut e = self.skolem(k, args);
            if branch.introduced.contains(&e) {
                // A copy of a δ-formula the branch took apart.
                branch.items[at].inert = true;
                continue;
            }
            if branch.constants.contains(&e) {
                // The branch used it as a constant: a fresh one stands in.
                e = self.fresh_eigen();
            }
            return self.instance(branch, at, e, false).map(Some);
        }
        // 4: γ-formulas, each needed witness once.
        for at in 0..branch.items.len() {
            let item = &branch.items[at];
            if !self.relevant[item.occ as usize] || item.inert {
                continue;
            }
            if !matches!(self.matrix.occs[item.occ as usize].kind, Kind::Gamma(_)) {
                continue;
            }
            let key = (item.occ, item.env);
            let Some(witnesses) = self.needed.get(&key) else {
                continue;
            };
            let next = witnesses
                .iter()
                .copied()
                .find(|t| !item.done.contains(t) && self.available(*t, branch));
            if let Some(t) = next {
                branch.items[at].done.push(t);
                return self.instance(branch, at, t, true).map(Some);
            }
        }
        // 5: two premises, one that closes a premise at once first.
        let closing = (0..branch.items.len()).find(|&at| {
            let item = &branch.items[at];
            self.relevant[item.occ as usize] && !item.inert && self.closes_a_premise(branch, at)
        });
        if let Some(at) = closing
            && let Some(step) = self.binary(branch, at)?
        {
            return Ok(Some(step));
        }
        for at in 0..branch.items.len() {
            let item = &branch.items[at];
            if !self.relevant[item.occ as usize] || item.inert {
                continue;
            }
            if let Some(step) = self.binary(branch, at)? {
                return Ok(Some(step));
            }
        }
        // 6: a needed witness with an eigenvariable the branch did not
        // introduce, whose δ-formula lies on another branch: a constant
        // here, kept from every `∀R` and `∃L` above. Of all pending
        // witnesses, one whose Skolem terms nest least: a δ-instance
        // arrives only through a witness that is an argument of its Skolem
        // term, which nests strictly less, so none of the eigenvariables
        // made constants here arrives later.
        let nesting = self.nesting();
        let mut best: Option<(u32, usize, FoTermId)> = None;
        for (at, item) in branch.items.iter().enumerate() {
            if !self.relevant[item.occ as usize] || item.inert {
                continue;
            }
            if !matches!(self.matrix.occs[item.occ as usize].kind, Kind::Gamma(_)) {
                continue;
            }
            let key = (item.occ, item.env);
            let Some(witnesses) = self.needed.get(&key) else {
                continue;
            };
            for &t in witnesses.iter().filter(|t| !item.done.contains(t)) {
                let rank = self
                    .skolem_eigens(t)
                    .iter()
                    .map(|e| nesting.get(e).copied().unwrap_or(0))
                    .max()
                    .unwrap_or(0);
                if best.is_none_or(|(least, _, _)| rank < least) {
                    best = Some((rank, at, t));
                }
            }
        }
        if let Some((_, at, t)) = best {
            let constants = self.skolem_eigens(t);
            branch.items[at].done.push(t);
            let mut step = self.instance(branch, at, t, true)?;
            for premise in &mut step.premises {
                for &e in &constants {
                    if !premise.introduced.contains(&e) && !premise.constants.contains(&e) {
                        premise.constants.push(e);
                    }
                }
            }
            return Ok(Some(step));
        }
        Ok(None)
    }

    /// Returns how deep each Skolem term's eigenvariable nests: 1 more than
    /// the deepest of those in its arguments. An argument's eigenvariable
    /// is made before the term's, so one pass in the order of their numbers
    /// sees each argument first.
    fn nesting(&self) -> HashMap<FoTermId, u32> {
        let mut terms: Vec<(FoTermId, &Vec<FoTermId>)> = self
            .skolems
            .iter()
            .map(|((_, args), &e)| (e, args))
            .collect();
        let tables = self.formulas.first_order();
        let number = |e: FoTermId| match tables.terms[e.index()] {
            FoTerm::Eigen(n) => n.get(),
            FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => 0,
        };
        terms.sort_unstable_by_key(|&(e, _)| number(e));
        let mut nesting: HashMap<FoTermId, u32> = HashMap::default();
        for (e, args) in terms {
            let inner = args
                .iter()
                .flat_map(|&a| self.skolem_eigens(a))
                .map(|d| nesting.get(&d).copied().unwrap_or(0))
                .max()
                .unwrap_or(0);
            nesting.insert(e, inner.saturating_add(1));
        }
        nesting
    }

    /// Returns whether item `at` can no longer help close the branch: it
    /// lies above no literal the proof used, or is a copy of a δ-formula
    /// taken apart, or is a γ-formula whose needed witnesses are all in.
    fn useless(&self, branch: &Branch, at: usize) -> bool {
        let item = &branch.items[at];
        if !self.relevant[item.occ as usize] || item.inert {
            return true;
        }
        if !matches!(self.matrix.occs[item.occ as usize].kind, Kind::Gamma(_)) {
            return false;
        }
        let key = (item.occ, item.env);
        self.needed
            .get(&key)
            .is_none_or(|witnesses| witnesses.iter().all(|t| item.done.contains(t)))
    }

    /// Returns whether a rule of two premises on item `at` would close one
    /// of them at once: an operand that stands on the other side, or `⊤`
    /// going right or `⊥` going left.
    fn closes_a_premise(&self, branch: &Branch, at: usize) -> bool {
        let item = &branch.items[at];
        let operands = match (self.formulas.node(item.node), item.left) {
            (Node::And(a, b), false) => [(a, false), (b, false)],
            (Node::Or(a, b), true) => [(a, true), (b, true)],
            (Node::Implies(a, b), true) => [(a, false), (b, true)],
            _ => return false,
        };
        operands.iter().any(|&(node, left)| {
            let shape = self.formulas.node(node);
            (left && shape == Node::False)
                || (!left && shape == Node::True)
                || branch
                    .items
                    .iter()
                    .any(|other| other.left != left && other.node == node)
        })
    }

    /// Returns the item for operand `k` of item `at`'s occurrence, with
    /// the formula `node`.
    fn child(&self, branch: &Branch, at: usize, k: usize, node: NodeId) -> Item {
        let item = &branch.items[at];
        let occ = self.matrix.occs[item.occ as usize].children[k];
        Item {
            occ,
            left: self.matrix.occs[occ as usize].left,
            node,
            env: item.env,
            done: Vec::new(),
            inert: false,
        }
    }

    /// Returns the branch without item `at` and with `added`.
    fn replaced(branch: &Branch, at: usize, added: Vec<Item>) -> Branch {
        let mut premise = branch.clone();
        premise.items.remove(at);
        premise.items.extend(added);
        premise
    }

    /// The rule of one premise on item `at`, if its formula has one.
    fn unary(&mut self, branch: &Branch, at: usize) -> Result<Option<Step>, Error> {
        let item = &branch.items[at];
        let principal = position(branch, at);
        let occ = self.matrix.occs[item.occ as usize];
        let node = self.formulas.node(item.node);
        let (rule, added) = match (occ.kind, item.left, node) {
            (Kind::And, true, Node::And(a, b)) => (
                Rule::AndLeft,
                vec![self.child(branch, at, 0, a), self.child(branch, at, 1, b)],
            ),
            (Kind::Or, false, Node::Or(a, b)) => (
                Rule::OrRight,
                vec![self.child(branch, at, 0, a), self.child(branch, at, 1, b)],
            ),
            (Kind::Implies | Kind::Part { .. }, false, Node::Implies(a, b)) => (
                Rule::ImpliesRight,
                vec![self.child(branch, at, 0, a), self.child(branch, at, 1, b)],
            ),
            (Kind::Not, true, Node::Not(a)) => (Rule::NotLeft, vec![self.child(branch, at, 0, a)]),
            (Kind::Not, false, Node::Not(a)) => {
                (Rule::NotRight, vec![self.child(branch, at, 0, a)])
            }
            (Kind::Iff, true, Node::Iff(a, b)) => {
                let forward = self.formulas.add(Node::Implies(a, b))?;
                let backward = self.formulas.add(Node::Implies(b, a))?;
                (
                    Rule::IffLeft,
                    vec![
                        self.child(branch, at, 0, forward),
                        self.child(branch, at, 1, backward),
                    ],
                )
            }
            (Kind::True, true, Node::True) => (Rule::TrueLeft, Vec::new()),
            (Kind::False, false, Node::False) => (Rule::FalseRight, Vec::new()),
            _ => return Ok(None),
        };
        Ok(Some(Step {
            rule,
            principal,
            term: None,
            premises: vec![Self::replaced(branch, at, added)],
        }))
    }

    /// The rule of two premises on item `at`, if its formula has one.
    fn binary(&mut self, branch: &Branch, at: usize) -> Result<Option<Step>, Error> {
        let item = &branch.items[at];
        let principal = position(branch, at);
        let occ = self.matrix.occs[item.occ as usize];
        let node = self.formulas.node(item.node);
        let (rule, first, second) = match (occ.kind, item.left, node) {
            (Kind::And, false, Node::And(a, b)) => (
                Rule::AndRight,
                self.child(branch, at, 0, a),
                self.child(branch, at, 1, b),
            ),
            (Kind::Or, true, Node::Or(a, b)) => (
                Rule::OrLeft,
                self.child(branch, at, 0, a),
                self.child(branch, at, 1, b),
            ),
            (Kind::Implies | Kind::Part { .. }, true, Node::Implies(a, b)) => (
                Rule::ImpliesLeft,
                self.child(branch, at, 0, a),
                self.child(branch, at, 1, b),
            ),
            (Kind::Iff, false, Node::Iff(a, b)) => {
                let forward = self.formulas.add(Node::Implies(a, b))?;
                let backward = self.formulas.add(Node::Implies(b, a))?;
                (
                    Rule::IffRight,
                    self.child(branch, at, 0, forward),
                    self.child(branch, at, 1, backward),
                )
            }
            _ => return Ok(None),
        };
        Ok(Some(Step {
            rule,
            principal,
            term: None,
            premises: vec![
                Self::replaced(branch, at, vec![first]),
                Self::replaced(branch, at, vec![second]),
            ],
        }))
    }

    /// The quantifier rule on item `at` with the term `t`: `∀L` or `∃R`
    /// keeping the principal formula where `keep`, else `∀R` or `∃L`
    /// introducing the eigenvariable `t`.
    fn instance(
        &mut self,
        branch: &Branch,
        at: usize,
        t: FoTermId,
        keep: bool,
    ) -> Result<Step, Error> {
        let item = &branch.items[at];
        let principal = position(branch, at);
        let (rule, body) = match (self.formulas.node(item.node), item.left) {
            (Node::Forall(body), true) => (Rule::ForallLeft, body),
            (Node::Forall(body), false) => (Rule::ForallRight, body),
            (Node::Exists(body), true) => (Rule::ExistsLeft, body),
            (Node::Exists(body), false) => (Rule::ExistsRight, body),
            _ => unreachable!("a γ- or δ-occurrence holds a binder"),
        };
        let node = if let Some(&node) = self.instances.get(&(body, t)) {
            node
        } else {
            self.budget.check(&self.formulas, body, &[t])?;
            let node = self.formulas.instantiate(body, t)?;
            self.budget.count(&self.formulas);
            self.instances.insert((body, t), node);
            node
        };
        let mut child = self.child(branch, at, 0, node);
        let premise = if keep {
            child.env = self.env_with(item.env, t);
            let mut premise = branch.clone();
            premise.items.push(child);
            premise
        } else {
            let mut premise = Self::replaced(branch, at, vec![child]);
            premise.introduced.push(t);
            premise
        };
        Ok(Step {
            rule,
            principal,
            term: Some(t),
            premises: vec![premise],
        })
    }
}

/// A rule applied to a branch: the inference's rule, principal formula
/// and term, and the premises' branches.
struct Step {
    /// The rule.
    rule: Rule,
    /// The principal formula.
    principal: (Side, usize),
    /// The term of a quantifier rule.
    term: Option<FoTermId>,
    /// The premises.
    premises: Vec<Branch>,
}
