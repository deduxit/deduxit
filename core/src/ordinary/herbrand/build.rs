// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The derivation of LK read back from the instances, each Skolem term
//! an eigenvariable, on the branches the shared construction keeps.

use super::matching::{Match, body_atoms, match_terms, matched};
use crate::Error;
use crate::first_order::{Eigen, FoTerm, FoTermId, Symbol};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::proofs::InfId;
use crate::ordinary::branch::{Emits, Placed, closes_a_premise, position, replaced, sides};
use crate::ordinary::bruijn::Replaced;
use crate::ordinary::derivation::{ArenaSize, Inference};
use crate::ordinary::{Derivation, Formulas, Logic, Node, NodeId, Rule, Sequent, Side, normal};
use std::cell::RefCell;
use std::rc::Rc;

/// A Skolem step of the clausal form: the quantifier replaced, open, what
/// replaced it, and its Skolem term over the quantifier's free variables.
#[derive(Clone, Copy, Debug)]
pub(super) struct Skolemized {
    /// The quantifier replaced.
    pub(super) before: NodeId,
    /// Its operand with the Skolem term in place of its variable.
    pub(super) after: NodeId,
    /// The Skolem term.
    pub(super) term: FoTermId,
}

/// A formula on a branch of the derivation built.
#[derive(Clone, Debug)]
pub(super) struct Item {
    /// The formula.
    node: NodeId,
    /// Its side: true for left of `⊢`.
    left: bool,
    /// The witnesses a `∀` hypothesis or an `∃` goal was instantiated
    /// with on this branch.
    done: Vec<FoTermId>,
    /// Whether it is left alone: a `∀` goal or `∃` hypothesis whose Skolem
    /// term the branch introduced already, through an equal one.
    inert: bool,
}

impl Item {
    /// Returns the item of `node` on its side, untouched.
    pub(super) const fn new(node: NodeId, left: bool) -> Self {
        Self {
            node,
            left,
            done: Vec::new(),
            inert: false,
        }
    }
}

/// A branch of the construction.
pub(super) type Branch = crate::ordinary::branch::Branch<Item>;
/// A rule applied to a branch.
type Step = crate::ordinary::branch::Step<Item>;
/// A step of the construction's stack.
type Frame = crate::ordinary::branch::Frame<Item>;

impl Frame {
    /// Returns the bytes the frame holds, estimated: a branch's formulas
    /// with the witnesses each took, or an inference's two sides.
    fn bytes(&self) -> u64 {
        let id = size_of::<u32>() as u64;
        match self {
            Self::Expand(branch) => {
                let items = branch.items.iter().fold(0u64, |sum, item| {
                    sum.saturating_add(size_of::<Item>() as u64)
                        .saturating_add(item.done.len() as u64 * id)
                });
                items.saturating_add((branch.introduced.len() + branch.constants.len()) as u64 * id)
            }
            Self::Emit { left, right, .. } => (left.len() + right.len()) as u64 * id,
        }
    }
}

/// The construction of the derivation, with Skolem terms where its
/// eigenvariables will be.
pub(super) struct Build<'l> {
    /// The arena: the clausal form's and the instances'.
    pub(super) formulas: Formulas,
    /// The clausal form's Skolem steps, in order.
    pub(super) skolems: Vec<Skolemized>,
    /// The Skolem symbols.
    pub(super) symbols: HashSet<Symbol>,
    /// The witnesses, the shallowest first.
    pub(super) witnesses: Vec<FoTermId>,
    /// The kept ground instances, each as its atoms: a predicate and its
    /// arguments.
    pub(super) clauses: Vec<Vec<(Symbol, Vec<FoTermId>)>>,
    /// The witnesses the instances give each `∀` hypothesis or `∃` goal,
    /// in the witnesses' order, by the quantifier's formula; `None` where
    /// they give none.
    pub(super) given: HashMap<NodeId, Option<Vec<FoTermId>>>,
    /// The Skolem term of each strong quantifier met, by its formula and
    /// side.
    pub(super) keys: HashMap<(NodeId, bool), FoTermId>,
    /// Each instance made, by the binder's operand and the term.
    pub(super) instances: HashMap<(NodeId, FoTermId), NodeId>,
    /// Every Skolem term a `∀R` or `∃L` introduced, in order.
    pub(super) made: Vec<FoTermId>,
    /// The inferences so far, premises first.
    pub(super) inferences: Vec<Inference>,
    /// The bytes they and the arena's growth hold, estimated.
    pub(super) bytes: u64,
    /// The arena's size when its growth was last charged.
    pub(super) charged: ArenaSize,
    /// The branches expanded so far.
    pub(super) expanded: u64,
    /// The Skolem terms in each term asked about, as `skolem_terms` found
    /// them: a term shares its parts, and a walk of its tree per witness,
    /// item and step would cost its unfolded size each time.
    pub(super) skolem_memo: RefCell<HashMap<FoTermId, Rc<[FoTermId]>>>,
    /// How deep the Skolem terms in each term walked nest.
    pub(super) nesting_memo: RefCell<HashMap<FoTermId, u32>>,
    /// The bounds.
    pub(super) limits: &'l Limits,
}

impl Placed for Item {
    fn left(&self) -> bool {
        self.left
    }

    fn node(&self) -> NodeId {
        self.node
    }
}

impl Emits for Build<'_> {
    fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    fn emit(
        &mut self,
        left: Vec<NodeId>,
        right: Vec<NodeId>,
        rule: Rule,
        principal: Option<(Side, usize)>,
        term: Option<FoTermId>,
        premises: Vec<InfId>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<InfId, Error> {
        self.push(left, right, rule, principal, term, premises, stop)
    }
}

impl Build<'_> {
    /// Derives the root branch, on stacks of its own.
    pub(super) fn derive(
        &mut self,
        root: Branch,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let mut frames = vec![Frame::Expand(root)];
        let mut results: Vec<InfId> = Vec::new();
        // The frames waiting are charged while they wait: a long branch
        // leaves an inference to make per step, each holding the branch's
        // formulas.
        let mut waiting = frames.iter().map(Frame::bytes).sum::<u64>();
        self.charge(waiting)?;
        while let Some(frame) = frames.pop() {
            let freed = frame.bytes();
            waiting = waiting.saturating_sub(freed);
            self.bytes = self.bytes.saturating_sub(freed);
            match frame {
                Frame::Expand(branch) => {
                    let before = frames.len();
                    self.expand(branch, &mut frames, &mut results, stop)?;
                    let added = frames[before.min(frames.len())..]
                        .iter()
                        .map(Frame::bytes)
                        .sum::<u64>();
                    waiting = waiting.saturating_add(added);
                    self.charge(added)?;
                }
                Frame::Emit {
                    left,
                    right,
                    rule,
                    principal,
                    term,
                    premises,
                } => {
                    let at = results.len() - premises;
                    let premises = results.split_off(at);
                    let id = self.push(left, right, rule, principal, term, premises, stop)?;
                    results.push(id);
                }
            }
        }
        Ok(())
    }

    /// Charges what the arena grew by since the last charge to the
    /// memory bound: instances and replaced formulas are built in it, and
    /// an atom of many arguments holds its key's text and its list.
    pub(super) fn charge_arena(&mut self) -> Result<(), Error> {
        let now = ArenaSize::of(&self.formulas, &self.charged);
        let grown = now.bytes.saturating_sub(self.charged.bytes);
        self.charged = now;
        self.charge(grown)
    }

    /// Adds `bytes` to what the construction holds, or refuses past the
    /// memory bound.
    fn charge(&mut self, bytes: u64) -> Result<(), Error> {
        self.bytes = self.bytes.saturating_add(bytes);
        match self.limits.memory_bytes {
            Some(limit) if self.bytes > limit => Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: limit,
                needed_bytes: None,
            })),
            _ => Ok(()),
        }
    }

    /// Adds an inference and returns its id, within the memory bound.
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
        crate::ordinary::derivation::ask_read_back(stop, n)?;
        // The inference in its list, which doubles as it grows, and each of
        // its three vectors with the allocator's header and rounding.
        let block = |len: usize| {
            if len == 0 {
                0
            } else {
                (len * size_of::<u32>()).next_multiple_of(16) + 16
            }
        };
        let held = 2 * size_of::<Inference>()
            + block(left.len())
            + block(right.len())
            + block(premises.len());
        self.charge(held as u64)?;
        crate::ordinary::derivation::add(
            &mut self.inferences,
            Inference {
                left,
                right,
                rule,
                principal,
                premises,
                term,
            },
        )
    }

    /// Closes the branch if it can, else applies the first rule its order
    /// gives.
    fn expand(
        &mut self,
        mut branch: Branch,
        frames: &mut Vec<Frame>,
        results: &mut Vec<InfId>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        // A branch may grow long before any inference closes: the stop and
        // the bound of work are asked at each step taken.
        self.expanded = self.expanded.saturating_add(1);
        if stop(Progress::new(Phase::ReadBack, 1, self.expanded)) {
            return Err(Error::Refused(Refusal::stopped(Phase::ReadBack)));
        }
        if let Some(limit) = self.limits.work
            && self.expanded > limit
        {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        if let Some(id) = self.close(&branch, stop)? {
            results.push(id);
            return Ok(());
        }
        let Some(step) = self.step(&mut branch)? else {
            return Err(Error::ReadBack {
                calculus: Logic::Classical.calculus(),
                reason: "the ground instances leave a branch open".to_owned(),
            });
        };
        let (left, right) = sides(&branch);
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

    /// Closes the branch by an axiom with weakenings, `⊤R` or `⊥L`, and
    /// returns the inference concluding it, if any closes it.
    fn close(
        &mut self,
        branch: &Branch,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Option<InfId>, Error> {
        crate::ordinary::branch::close(self, branch, stop)
    }

    /// Returns the first rule the order gives on the branch, if any: one
    /// premise and no term; `∀R` and `∃L`; `∀L` and `∃R` with a witness
    /// the formula's instances give; two premises, one closing at once;
    /// `∀L` and `∃R` with any witness the branch has every Skolem term
    /// of; any rule of two premises; a witness with a Skolem term of
    /// another branch, the least nested first.
    fn step(&mut self, branch: &mut Branch) -> Result<Option<Step>, Error> {
        // A formula that can no longer help is weakened away.
        for at in 0..branch.items.len() {
            if self.useless(&branch.items[at]) {
                let rule = if branch.items[at].left {
                    Rule::WeakenLeft
                } else {
                    Rule::WeakenRight
                };
                return Ok(Some(Step {
                    rule,
                    principal: position(branch, at),
                    term: None,
                    premises: vec![replaced(branch, at, Vec::new())],
                }));
            }
        }
        for at in 0..branch.items.len() {
            if let Some(step) = self.unary(branch, at)? {
                return Ok(Some(step));
            }
        }
        for at in 0..branch.items.len() {
            let item = &branch.items[at];
            let strong = matches!(
                (self.formulas.node(item.node), item.left),
                (Node::Forall(_), false) | (Node::Exists(_), true)
            );
            if !strong || item.inert {
                continue;
            }
            let s = self.skolem(item.node, item.left)?;
            if branch.introduced.contains(&s) || branch.constants.contains(&s) {
                // An equal quantifier taken apart already, or its term
                // used as a constant: this one is left alone.
                branch.items[at].inert = true;
                continue;
            }
            return self.instance(branch, at, s, false).map(Some);
        }
        // The witnesses the kept instances give each quantifier first (all
        // of them where they give none), then a rule of two premises that
        // closes one at once, and only then every other witness: where the
        // instances say which terms a quantifier takes, nested quantifiers
        // over many constants no longer take every witness each before a
        // branch can close. The other rules of two premises still wait for
        // every witness, so that no branch repeats them after a split.
        for at in 0..branch.items.len() {
            if !self.weak(&branch.items[at]) {
                continue;
            }
            let given = self.given(branch.items[at].node);
            let item = &branch.items[at];
            let next = given
                .as_deref()
                .unwrap_or(&self.witnesses)
                .iter()
                .copied()
                .find(|&t| !item.done.contains(&t) && self.available(t, branch));
            if let Some(t) = next {
                branch.items[at].done.push(t);
                return self.instance(branch, at, t, true).map(Some);
            }
        }
        let closing =
            (0..branch.items.len()).find(|&at| closes_a_premise(&self.formulas, branch, at));
        if let Some(at) = closing
            && let Some(step) = self.binary(branch, at)?
        {
            return Ok(Some(step));
        }
        for at in 0..branch.items.len() {
            if !self.weak(&branch.items[at]) {
                continue;
            }
            let item = &branch.items[at];
            let next = self
                .witnesses
                .iter()
                .copied()
                .find(|&t| !item.done.contains(&t) && self.available(t, branch));
            if let Some(t) = next {
                branch.items[at].done.push(t);
                return self.instance(branch, at, t, true).map(Some);
            }
        }
        for at in 0..branch.items.len() {
            if let Some(step) = self.binary(branch, at)? {
                return Ok(Some(step));
            }
        }
        // A witness whose Skolem term lies on another branch is a constant
        // here. Of all pending witnesses, one whose Skolem terms nest least:
        // a quantifier's instance arrives only through a witness that is an
        // argument of its Skolem term, which nests strictly less, so none
        // made a constant here arrives later.
        let mut best: Option<(u32, usize, FoTermId)> = None;
        for (at, item) in branch.items.iter().enumerate() {
            if !self.weak(item) {
                continue;
            }
            for &t in self.witnesses.iter().filter(|t| !item.done.contains(t)) {
                let rank = self.nesting(t);
                if best.is_none_or(|(least, _, _)| rank < least) {
                    best = Some((rank, at, t));
                }
            }
        }
        if let Some((_, at, t)) = best {
            let constants = self.skolem_terms(t);
            branch.items[at].done.push(t);
            let mut step = self.instance(branch, at, t, true)?;
            for premise in &mut step.premises {
                for &e in constants.iter() {
                    if !premise.introduced.contains(&e) && !premise.constants.contains(&e) {
                        premise.constants.push(e);
                    }
                }
            }
            return Ok(Some(step));
        }
        Ok(None)
    }

    /// Returns the witnesses the kept instances give the quantifier
    /// `node`, a `∀x. B` or `∃x. B`, in the witnesses' order, or `None`
    /// where they give none. A ground instance gives the values of `x`
    /// that all its atoms agree on: each atom matches some atom of `B`,
    /// with `x` at that value or absent; an instance with an atom no atom
    /// of `B` matches comes from another formula, and one whose atoms all
    /// leave `x` free says nothing.
    fn given(&mut self, node: NodeId) -> Option<Vec<FoTermId>> {
        if let Some(given) = self.given.get(&node) {
            return given.clone();
        }
        let (Node::Forall(body) | Node::Exists(body)) = self.formulas.node(node) else {
            return None;
        };
        let tables = self.formulas.first_order();
        let patterns = body_atoms(&self.formulas, body);
        let mut values: HashSet<FoTermId> = HashSet::default();
        for clause in &self.clauses {
            // `None`: every value so far; `Some(set)`: those values.
            let mut agreed: Option<HashSet<FoTermId>> = None;
            let mut from_here = true;
            for (symbol, args) in clause {
                let mut any = false;
                let mut these: HashSet<FoTermId> = HashSet::default();
                let mut matched_one = false;
                for (pattern_symbol, pattern, depth) in &patterns {
                    if pattern_symbol != symbol || pattern.len() != args.len() {
                        continue;
                    }
                    match match_terms(tables, pattern, args, *depth) {
                        Match::No => {}
                        Match::Free => {
                            matched_one = true;
                            any = true;
                        }
                        Match::At(v) => {
                            matched_one = true;
                            these.insert(v);
                        }
                    }
                }
                if !matched_one {
                    from_here = false;
                    break;
                }
                if !any {
                    agreed = Some(match agreed {
                        None => these,
                        Some(set) => set.intersection(&these).copied().collect(),
                    });
                }
            }
            if from_here && let Some(set) = agreed {
                values.extend(set);
            }
        }
        let given = (!values.is_empty()).then(|| {
            self.witnesses
                .iter()
                .copied()
                .filter(|t| values.contains(t))
                .collect()
        });
        self.given.insert(node, given.clone());
        given
    }

    /// Returns whether `item` is a `∀` hypothesis or an `∃` goal.
    fn weak(&self, item: &Item) -> bool {
        matches!(
            (self.formulas.node(item.node), item.left),
            (Node::Forall(_), true) | (Node::Exists(_), false)
        )
    }

    /// Returns whether `item` can no longer help close its branch: a
    /// quantifier left alone, or a `∀` hypothesis or `∃` goal with every
    /// witness in.
    fn useless(&self, item: &Item) -> bool {
        item.inert || (self.weak(item) && self.witnesses.iter().all(|t| item.done.contains(t)))
    }

    /// The rule of one premise on item `at`, if its formula has one.
    fn unary(&mut self, branch: &Branch, at: usize) -> Result<Option<Step>, Error> {
        let item = &branch.items[at];
        let principal = position(branch, at);
        let (rule, added) = match (item.left, self.formulas.node(item.node)) {
            (true, Node::And(a, b)) => {
                (Rule::AndLeft, vec![Item::new(a, true), Item::new(b, true)])
            }
            (false, Node::Or(a, b)) => (
                Rule::OrRight,
                vec![Item::new(a, false), Item::new(b, false)],
            ),
            (false, Node::Implies(a, b)) => (
                Rule::ImpliesRight,
                vec![Item::new(a, true), Item::new(b, false)],
            ),
            (true, Node::Not(a)) => (Rule::NotLeft, vec![Item::new(a, false)]),
            (false, Node::Not(a)) => (Rule::NotRight, vec![Item::new(a, true)]),
            (true, Node::Iff(a, b)) => {
                let forward = self.formulas.add(Node::Implies(a, b))?;
                let backward = self.formulas.add(Node::Implies(b, a))?;
                (
                    Rule::IffLeft,
                    vec![Item::new(forward, true), Item::new(backward, true)],
                )
            }
            (true, Node::True) => (Rule::TrueLeft, Vec::new()),
            (false, Node::False) => (Rule::FalseRight, Vec::new()),
            _ => return Ok(None),
        };
        Ok(Some(Step {
            rule,
            principal,
            term: None,
            premises: vec![replaced(branch, at, added)],
        }))
    }

    /// The rule of two premises on item `at`, if its formula has one.
    fn binary(&mut self, branch: &Branch, at: usize) -> Result<Option<Step>, Error> {
        let item = &branch.items[at];
        let principal = position(branch, at);
        let (rule, first, second) = match (item.left, self.formulas.node(item.node)) {
            (false, Node::And(a, b)) => (Rule::AndRight, Item::new(a, false), Item::new(b, false)),
            (true, Node::Or(a, b)) => (Rule::OrLeft, Item::new(a, true), Item::new(b, true)),
            (true, Node::Implies(a, b)) => {
                (Rule::ImpliesLeft, Item::new(a, false), Item::new(b, true))
            }
            (false, Node::Iff(a, b)) => {
                let forward = self.formulas.add(Node::Implies(a, b))?;
                let backward = self.formulas.add(Node::Implies(b, a))?;
                (
                    Rule::IffRight,
                    Item::new(forward, false),
                    Item::new(backward, false),
                )
            }
            _ => return Ok(None),
        };
        Ok(Some(Step {
            rule,
            principal,
            term: None,
            premises: vec![
                replaced(branch, at, vec![first]),
                replaced(branch, at, vec![second]),
            ],
        }))
    }

    /// The quantifier rule on item `at` with the term `t`: `∀L` or `∃R`
    /// keeping the principal formula where `keep`, else `∀R` or `∃L`
    /// introducing the Skolem term `t`.
    fn instance(
        &mut self,
        branch: &Branch,
        at: usize,
        t: FoTermId,
        keep: bool,
    ) -> Result<Step, Error> {
        let item = &branch.items[at];
        let left = item.left;
        let principal = position(branch, at);
        let (rule, body) = match (self.formulas.node(item.node), left) {
            (Node::Forall(body), true) => (Rule::ForallLeft, body),
            (Node::Forall(body), false) => (Rule::ForallRight, body),
            (Node::Exists(body), true) => (Rule::ExistsLeft, body),
            (Node::Exists(body), false) => (Rule::ExistsRight, body),
            _ => unreachable!("a quantifier rule takes a binder"),
        };
        let node = if let Some(&node) = self.instances.get(&(body, t)) {
            node
        } else {
            let node = self.formulas.instantiate(body, t)?;
            self.instances.insert((body, t), node);
            self.charge_arena()?;
            node
        };
        let premise = if keep {
            let mut premise = branch.clone();
            premise.items.push(Item::new(node, left));
            premise
        } else {
            let mut premise = replaced(branch, at, vec![Item::new(node, left)]);
            premise.introduced.push(t);
            if !self.made.contains(&t) {
                self.made.push(t);
            }
            premise
        };
        Ok(Step {
            rule,
            principal,
            term: Some(t),
            premises: vec![premise],
        })
    }

    /// Returns the Skolem term of the strong quantifier `node` on its side
    /// (`left`): the clausal form's term of the step whose quantifier
    /// meets the formula's negation normal form, moved left of `⊢`.
    fn skolem(&mut self, node: NodeId, left: bool) -> Result<FoTermId, Error> {
        if let Some(&s) = self.keys.get(&(node, left)) {
            return Ok(s);
        }
        let moved = if left {
            node
        } else {
            self.formulas.add(Node::Not(node))?
        };
        let mut normal = normal::negation_normal(&mut self.formulas, moved, self.limits)?;
        // The clausal form took its steps in order, and an earlier one may
        // have replaced a quantifier inside this one (an equal formula of
        // another hypothesis, Skolemized first): each step met as a whole
        // is a candidate, and each met by a closed part of the formula is
        // taken there before the next.
        let mut found: Vec<(FoTermId, Vec<Option<FoTermId>>)> = Vec::new();
        for k in 0..self.skolems.len() {
            let step = self.skolems[k];
            if let Some(values) = matched(&self.formulas, step.before, normal) {
                found.push((step.term, values));
            } else {
                normal = self.taken(normal, step)?;
            }
        }
        // An instance of one quantifier can be another quantifier of the
        // sequent: of the steps it meets, the one whose term the kept
        // instances use.
        let mut terms = Vec::with_capacity(found.len());
        for (term, values) in &found {
            terms.push(self.applied(*term, values));
        }
        let Some(&first) = terms.first() else {
            return Err(Error::ReadBack {
                calculus: Logic::Classical.calculus(),
                reason: "a quantifier of the sequent meets no Skolem step of its clausal form"
                    .to_owned(),
            });
        };
        let s = terms
            .iter()
            .copied()
            .find(|t| self.witnesses.contains(t))
            .unwrap_or(first);
        self.keys.insert((node, left), s);
        Ok(s)
    }

    /// Returns `root` with the step's quantifier, where it is closed and
    /// stands inside `root`, replaced by what the step made.
    fn taken(&mut self, root: NodeId, step: Skolemized) -> Result<NodeId, Error> {
        let mut made: HashMap<NodeId, NodeId> = HashMap::default();
        let mut stack = vec![(root, false)];
        while let Some((n, ready)) = stack.pop() {
            if made.contains_key(&n) {
                continue;
            }
            let node = self.formulas.node(n);
            // The clausal form replaced the step's own node wherever it
            // stood, and no other: a closed quantifier that is that node.
            if n == step.before && n != root && self.formulas.free_variables(n).is_empty() {
                made.insert(n, step.after);
                continue;
            }
            if !ready {
                stack.push((n, true));
                let (a, b) = node.operands();
                stack.extend([a, b].into_iter().flatten().map(|k| (k, false)));
                continue;
            }
            let at = |k: NodeId| made[&k];
            let image = match node {
                Node::Atom(_) | Node::True | Node::False => n,
                Node::Not(a) => self.formulas.add_like(Node::Not(at(a)), n)?,
                Node::And(a, b) => self.formulas.add_like(Node::And(at(a), at(b)), n)?,
                Node::Or(a, b) => self.formulas.add_like(Node::Or(at(a), at(b)), n)?,
                Node::Implies(a, b) => self.formulas.add_like(Node::Implies(at(a), at(b)), n)?,
                Node::Iff(a, b) => self.formulas.add_like(Node::Iff(at(a), at(b)), n)?,
                Node::Forall(a) => self.formulas.add_like(Node::Forall(at(a)), n)?,
                Node::Exists(a) => self.formulas.add_like(Node::Exists(at(a)), n)?,
            };
            made.insert(n, image);
        }
        Ok(made[&root])
    }

    /// Returns the Skolem term `term`, over variables, with each variable
    /// `i` the closed term `values[i]`.
    fn applied(&mut self, term: FoTermId, values: &[Option<FoTermId>]) -> FoTermId {
        let tables = self.formulas.first_order();
        let FoTerm::App(f, args) = tables.terms[term.index()] else {
            return term;
        };
        let args: Vec<FoTermId> = tables
            .list(args)
            .iter()
            .map(|&a| match tables.terms[a.index()] {
                FoTerm::Bound(i) => values
                    .get(i as usize)
                    .copied()
                    .flatten()
                    .unwrap_or_else(|| unreachable!("a matched step binds its variables")),
                FoTerm::App(..) | FoTerm::Eigen(_) | FoTerm::Meta(_) => a,
            })
            .collect();
        let list = self.formulas.interner().list(&args);
        self.formulas.interner().term(FoTerm::App(f, list))
    }

    /// Returns whether every Skolem term in `t` is one the branch
    /// introduced or holds as a constant.
    fn available(&self, t: FoTermId, branch: &Branch) -> bool {
        self.skolem_terms(t)
            .iter()
            .all(|s| branch.introduced.contains(s) || branch.constants.contains(s))
    }

    /// Returns the Skolem terms in `t`, `t` among them, in the order a
    /// walk of its tree meets them first.
    fn skolem_terms(&self, t: FoTermId) -> Rc<[FoTermId]> {
        if let Some(found) = self.skolem_memo.borrow().get(&t) {
            return Rc::clone(found);
        }
        let tables = self.formulas.first_order();
        let (mut found, mut stack) = (Vec::new(), vec![t]);
        // A part met again was walked whole already (the stack takes a
        // part's arguments before anything under it), so skipping it
        // keeps the order and every term found.
        let mut seen: HashSet<FoTermId> = HashSet::default();
        while let Some(u) = stack.pop() {
            if !seen.insert(u) {
                continue;
            }
            if let FoTerm::App(f, args) = tables.terms[u.index()] {
                if self.symbols.contains(&f) {
                    found.push(u);
                }
                stack.extend_from_slice(tables.list(args));
            }
        }
        let found: Rc<[FoTermId]> = found.into();
        self.skolem_memo.borrow_mut().insert(t, Rc::clone(&found));
        found
    }

    /// Returns how deep the Skolem terms in `t` nest: 0 for none, else one
    /// more than the deepest inside the deepest. Each part's depth is kept,
    /// so a part shared on many paths is walked once.
    fn nesting(&self, t: FoTermId) -> u32 {
        let tables = self.formulas.first_order();
        let mut memo = self.nesting_memo.borrow_mut();
        let mut stack = vec![(t, false)];
        while let Some((u, ready)) = stack.pop() {
            if memo.contains_key(&u) {
                continue;
            }
            let FoTerm::App(f, args) = tables.terms[u.index()] else {
                memo.insert(u, 0);
                continue;
            };
            let list = tables.list(args);
            if !ready {
                stack.push((u, true));
                stack.extend(list.iter().map(|&v| (v, false)));
                continue;
            }
            let below = list.iter().map(|v| memo[v]).max().unwrap_or(0);
            memo.insert(u, below + u32::from(self.symbols.contains(&f)));
        }
        memo[&t]
    }

    /// Returns the derivation of `sequent` made: each Skolem term an
    /// introduced eigenvariable, numbered past the arena's, the least
    /// nested first.
    pub(super) fn deskolemized(mut self, sequent: &Sequent) -> Result<Derivation, Error> {
        let base = crate::ordinary::derivation::next_eigen(&self.formulas);
        let mut made = std::mem::take(&mut self.made);
        made.sort_by_cached_key(|&s| self.nesting(s));
        let mut image: HashMap<FoTermId, FoTermId> = HashMap::default();
        for (k, &s) in made.iter().enumerate() {
            let e = Eigen::new(base.saturating_add(k as u32));
            image.insert(s, self.formulas.interner().term(FoTerm::Eigen(e)));
        }
        let mut cache = Replaced::new(&image);
        let mut inferences = std::mem::take(&mut self.inferences);
        for inference in &mut inferences {
            for node in inference.left.iter_mut().chain(inference.right.iter_mut()) {
                *node = self.formulas.replaced(*node, &mut cache)?;
            }
            if let Some(t) = inference.term {
                inference.term = Some(self.formulas.replaced_term(t, &mut cache));
            }
            self.charge_arena()?;
        }
        Ok(Derivation::from_parts(
            Logic::Classical,
            self.formulas,
            (sequent.left().to_vec(), sequent.right().to_vec()),
            inferences,
        ))
    }
}
