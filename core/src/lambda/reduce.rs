// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! β-reduction, one step at a time.
//!
//! A step replaces a redex `(λx. M) N` by `M` with `N` in place of every
//! use of `x`, or, in System F, a redex `(ΛX. M) [A]` by `M` with the type
//! `A` for `X` in every type `M` writes. Each step builds the term afresh
//! as a tree, every binder a variable of its own, a type abstraction's
//! too: a copy of `N` gets new variables for its binders, so no variable
//! is ever bound twice, and a use cannot be captured by its variable.
//! Names can still clash: where a binder would take in a free variable of
//! its name, the binder is renamed to a name the term does not use, so
//! that the text names the right variables.
//!
//! Every term a step makes is checked by [`Term::check`] before the next
//! step: a typed term keeps its type (subject reduction), and a linear or
//! affine term keeps using each variable as its calculus allows.

use super::{Node, Term, TermId, Type, TypeId, TypeVarId, Types, VarId, Variable, poly};
use crate::Error;
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal, Space};

/// How a reduction proceeds and when it stops.
///
/// # JSON
///
/// With the feature `serialize`, `{"strategy": "normal_order", "steps":
/// 1000, "nodes": 100000}`, a bound a number or `null` for none; a key
/// left out keeps its default, and an unknown one is refused.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct ReduceOptions {
    /// Which redex each step reduces, [`Strategy::NormalOrder`] by
    /// default.
    pub strategy: Strategy,
    /// The most steps taken, [`DEFAULT_STEPS`](Self::DEFAULT_STEPS) by
    /// default.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub steps: Option<u64>,
    /// The most nodes a term a step makes may have,
    /// [`DEFAULT_NODES`](Self::DEFAULT_NODES) by default.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub nodes: Option<u64>,
}

impl ReduceOptions {
    /// The default bound on steps: a thousand.
    pub const DEFAULT_STEPS: u64 = 1_000;
    /// The default bound on a term's nodes: a hundred thousand.
    pub const DEFAULT_NODES: u64 = 100_000;

    /// Returns the options with another [`strategy`](Self::strategy).
    #[must_use]
    pub const fn with_strategy(self, strategy: Strategy) -> Self {
        Self { strategy, ..self }
    }

    /// Returns the options with another bound on [`steps`](Self::steps).
    #[must_use]
    pub const fn with_steps(self, steps: Option<u64>) -> Self {
        Self { steps, ..self }
    }

    /// Returns the options with another bound on [`nodes`](Self::nodes).
    #[must_use]
    pub const fn with_nodes(self, nodes: Option<u64>) -> Self {
        Self { nodes, ..self }
    }
}

impl Default for ReduceOptions {
    /// Normal order, within [`DEFAULT_STEPS`](Self::DEFAULT_STEPS) and
    /// [`DEFAULT_NODES`](Self::DEFAULT_NODES).
    fn default() -> Self {
        Self {
            strategy: Strategy::default(),
            steps: Some(Self::DEFAULT_STEPS),
            nodes: Some(Self::DEFAULT_NODES),
        }
    }
}

/// Which redex a step reduces.
///
/// # JSON
///
/// With the feature `serialize`, `"normal_order"` or
/// `"applicative_order"`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Strategy {
    /// The leftmost outermost redex: the one whose text starts first.
    /// It reaches a term's normal form whenever the term has one.
    #[default]
    NormalOrder,
    /// The leftmost innermost redex: a function's argument is reduced
    /// before the function is applied to it. It may run forever on a term
    /// that has a normal form, such as `(λx. y) Ω`.
    ApplicativeOrder,
}

impl Strategy {
    /// Every strategy, in the order of their names.
    pub const ALL: &'static [Self] = &[Self::NormalOrder, Self::ApplicativeOrder];

    /// The names, as the JSON form and the command write them.
    pub const NAMES: &'static [&'static str] = &["normal_order", "applicative_order"];

    /// Returns the strategy's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::NormalOrder => "normal_order",
            Self::ApplicativeOrder => "applicative_order",
        }
    }

    /// Returns the redex of `term` that the strategy reduces next: an
    /// application of a `λ`, or of a type abstraction `ΛX` to a type, or
    /// `None` for a term in normal form. In a term whose nodes are shared,
    /// it is the first place the term's text writes it.
    pub fn pick(self, term: &Term) -> Option<TermId> {
        let redex = |id: TermId| redex_of(term, id).is_some();
        match self {
            Self::NormalOrder => {
                let mut stack = vec![term.root];
                while let Some(id) = stack.pop() {
                    if redex(id) {
                        return Some(id);
                    }
                    let first = stack.len();
                    super::operands(term.node(id), |t| stack.push(t));
                    stack[first..].reverse();
                }
                None
            }
            Self::ApplicativeOrder => {
                // The first redex the walk leaves, its subterms first, holds
                // no other: it is innermost, and of those the leftmost.
                let mut stack = vec![(term.root, false)];
                while let Some((id, left)) = stack.pop() {
                    if left {
                        if redex(id) {
                            return Some(id);
                        }
                        continue;
                    }
                    stack.push((id, true));
                    let first = stack.len();
                    super::operands(term.node(id), |t| stack.push((t, false)));
                    stack[first..].reverse();
                }
                None
            }
        }
    }
}

impl std::str::FromStr for Strategy {
    type Err = Error;

    /// Reads a strategy by its name.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .copied()
            .find(|strategy| strategy.name() == s)
            .ok_or_else(|| Error::UnknownName {
                what: "reduction strategy",
                name: s.into(),
                known: Self::NAMES,
            })
    }
}

/// One step of a reduction.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Step {
    /// The redex reduced, an application of a `λ` or of a type
    /// abstraction: a node of the term the step was taken on.
    pub redex: TermId,
}

/// Why a reduction ended.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum End {
    /// The term reached has no redex: it is in normal form.
    NormalForm,
    /// The bound on steps was reached, and the term reached still has a
    /// redex. The reduction is not finished: an untyped term need not
    /// have a normal form at all.
    #[non_exhaustive]
    Steps {
        /// The bound, [`ReduceOptions::steps`].
        limit: u64,
    },
    /// The next step would make a term of more nodes than the bound.
    #[non_exhaustive]
    Nodes {
        /// The bound, [`ReduceOptions::nodes`].
        limit: u64,
    },
    /// A bound of the limits or the stop ended the reduction.
    Refused(Refusal),
}

/// The outcome of a reduction: the last term reached, the steps that led
/// there and why it ended.
///
/// # JSON
///
/// With the feature `serialize`, written only: `{"version": 30,
/// "strategy": "normal_order", "term": {…}, "steps": [{"redex": 4}, …],
/// "end": "normal_form"}`.
/// - `term` is the term reached in [`Term`]'s form without `version`.
/// - Each step's `redex` is a node of the term the step was taken on.
/// - `end` is `normal_form`, `steps` or `nodes` with the bound as
///   `limit`, or `refused` with `reason`, the refusal in an error's form.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Reduction {
    /// The strategy the steps followed.
    pub strategy: Strategy,
    /// The last term reached, which [`Term::check`] accepted: in normal
    /// form when the reduction ended at [`End::NormalForm`].
    pub term: Term,
    /// The steps taken, in order, each on the term the step before made.
    pub steps: Vec<Step>,
    /// Why the reduction ended.
    pub end: End,
}

/// What a node is taken to be by the copy.
enum Task {
    /// Copy the subterm at a node of the source.
    Enter(TermId),
    /// Make the copy of a node whose subterms are copied, and put back
    /// the variables its binders hid, a type abstraction's last.
    Exit(
        TermId,
        Vec<(VarId, Option<VarId>)>,
        Option<(TypeVarId, Option<TypeId>)>,
    ),
    /// Put back what a type variable stood for before a type redex's body
    /// was copied with its argument for it.
    Restore(TypeVarId, Option<TypeId>),
}

/// A redex of a term.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Redex {
    /// `(λx. M) N`: `x` and `N`.
    Term(VarId, TermId),
    /// `(ΛX. M) [A]`: `X` and `A`.
    Type(TypeVarId, TypeId),
}

/// Returns the redex at `id`, an application of a `λ` or of a type
/// abstraction, with what its reduct puts for what.
fn redex_of(term: &Term, id: TermId) -> Option<Redex> {
    let node = term.node(id);
    if let Node::Apply { function, argument } = node
        && let Node::Lambda { variable, .. } = term.node(*function)
    {
        return Some(Redex::Term(*variable, *argument));
    }
    if let Node::TypeApply {
        term: function,
        argument,
    } = node
        && let Node::TypeLambda { variable, .. } = term.node(*function)
    {
        return Some(Redex::Type(*variable, *argument));
    }
    None
}

/// The work and the bytes a reduction may spend, and its stop.
pub(super) struct Budget<'s> {
    /// The work done, a unit per node made or visited.
    work: u64,
    /// The most work, [`Limits::work`].
    most: Option<u64>,
    /// The most bytes a term may hold, [`Limits::memory_bytes`].
    memory: Option<u64>,
    /// The work at which the stop is asked next.
    next: u64,
    /// The caller's stop.
    pub(super) stop: &'s mut dyn FnMut(Progress) -> bool,
}

/// How many units of work pass between two asks of the stop.
const POLL: u64 = 4096;

/// What a term holds per node, in bytes: the node and its variable, at
/// most, and what the check keeps for it.
const NODE_BYTES: u64 = 192;

impl Budget<'_> {
    /// Counts `units` of work, and refuses past the bound or at the stop;
    /// `done` is what the progress reports.
    pub(super) fn tick(&mut self, units: u64, done: u64) -> Result<(), Refusal> {
        self.work = self.work.saturating_add(units);
        if let Some(limit) = self.most.filter(|&most| self.work > most) {
            return Err(Refusal::Work { limit });
        }
        if self.work >= self.next {
            self.next = self.work.saturating_add(POLL);
            if (self.stop)(Progress::new(Phase::Reduce, 1, done)) {
                return Err(Refusal::Stopped {
                    phase: Phase::Reduce,
                });
            }
        }
        Ok(())
    }
}

/// Why a copy was given up.
pub(super) enum Halt {
    /// The copy has more nodes than the bound.
    Nodes(u64),
    /// A bound of the limits, or the stop.
    Refused(Refusal),
}

/// The copy of a term as a tree, every binder a variable of its own.
struct Copier<'t> {
    /// The source.
    term: &'t Term,
    /// The variable of the copy each variable of the source stands for
    /// at the place being copied.
    standing: Vec<Option<VarId>>,
    /// The variables of the copy.
    variables: Vec<Variable>,
    /// The types of the copy: the source's, then those the copy made.
    types: Types,
    /// The type variables of the copy: the source's, then a new one for
    /// every type abstraction copied.
    type_variables: Vec<Box<str>>,
    /// The type each type variable of the source stands for at the place
    /// being copied: its copy's, or a type redex's argument.
    type_standing: Vec<Option<TypeId>>,
    /// The nodes of the copy.
    nodes: Vec<Node>,
    /// The node of the source each node of the copy was copied from.
    origin: Vec<TermId>,
    /// The redex to reduce and what its reduct puts for what.
    redex: Option<(TermId, Redex)>,
    /// The most nodes the copy may have.
    limit: Option<u64>,
}

impl Copier<'_> {
    /// Adds a node of the copy, copied from `from`.
    fn push(&mut self, node: Node, from: TermId, budget: &mut Budget<'_>) -> Result<TermId, Halt> {
        let made = self.nodes.len() as u64;
        if let Some(limit) = self.limit.filter(|&limit| made >= limit) {
            return Err(Halt::Nodes(limit));
        }
        if let Some(limit) = budget
            .memory
            .filter(|&limit| (made + 1).saturating_mul(NODE_BYTES) > limit)
        {
            return Err(Halt::Refused(Refusal::Memory {
                phase: Phase::Reduce,
                limit_bytes: limit,
                needed_bytes: None,
            }));
        }
        // A node's id and a variable's are `u32`s, and a node makes at
        // most as many variables as it binds, each a node of its own.
        let most = u64::from(u32::MAX);
        if made >= most || self.variables.len() as u64 >= most {
            return Err(Halt::Refused(Refusal::Index {
                what: Space::Term,
                count: made.saturating_add(1),
                most,
            }));
        }
        budget.tick(1, made).map_err(Halt::Refused)?;
        self.nodes.push(node);
        self.origin.push(from);
        Ok(TermId(made as u32))
    }

    /// Returns a new variable of the copy, as the source's `v`, its type
    /// as the place being copied has it.
    fn fresh(&mut self, v: VarId, budget: &mut Budget<'_>) -> Result<VarId, Halt> {
        // `push` holds the variables below 2³², a node binding at most
        // as many variables as it has nodes.
        let id = VarId(self.variables.len() as u32);
        let mut variable = self.term.variables[v.index()].clone();
        if let Some(ty) = variable.ty {
            variable.ty = Some(self.retyped(ty, budget)?);
        }
        self.variables.push(variable);
        Ok(id)
    }

    /// Returns a type of the source as the place being copied has it:
    /// each type variable of a type abstraction around replaced by what
    /// it stands for there.
    fn retyped(&mut self, ty: TypeId, budget: &mut Budget<'_>) -> Result<TypeId, Halt> {
        if self.type_standing.iter().all(Option::is_none) {
            return Ok(ty);
        }
        let before = self.types.work;
        let standing = &self.type_standing;
        let made = poly::substitute(&mut self.types, ty, |v| standing[v.index()]);
        budget
            .tick(self.types.work - before, self.nodes.len() as u64)
            .map_err(Halt::Refused)?;
        Ok(made)
    }

    /// Returns a new type variable of the copy, named as the source's `x`.
    fn fresh_type(&mut self, x: TypeVarId) -> TypeVarId {
        // One per type abstraction copied, each a node of the copy, which
        // `push` holds below 2³².
        let id = TypeVarId(self.type_variables.len() as u32);
        self.type_variables
            .push(self.term.type_variables[x.index()].clone());
        id
    }

    /// Copies the subterm at `root`, the redex replaced by its reduct,
    /// and returns the copy's node.
    fn subterm(&mut self, root: TermId, budget: &mut Budget<'_>) -> Result<TermId, Halt> {
        let mut tasks = vec![Task::Enter(root)];
        let mut made: Vec<TermId> = Vec::new();
        while let Some(task) = tasks.pop() {
            match task {
                Task::Enter(id) => {
                    let node = self.term.node(id);
                    match self.redex {
                        Some((redex, Redex::Term(variable, argument))) if redex == id => {
                            let Node::Apply { function, .. } = node else {
                                unreachable!("a redex is an application");
                            };
                            let Node::Lambda { body, .. } = self.term.node(*function) else {
                                unreachable!("a redex applies a λ");
                            };
                            debug_assert_eq!(
                                Some(Redex::Term(variable, argument)),
                                redex_of(self.term, id)
                            );
                            tasks.push(Task::Enter(*body));
                            continue;
                        }
                        Some((redex, Redex::Type(variable, argument))) if redex == id => {
                            let Node::TypeApply { term: function, .. } = node else {
                                unreachable!("a type redex is a type application");
                            };
                            let Node::TypeLambda { body, .. } = self.term.node(*function) else {
                                unreachable!("a type redex applies a type abstraction");
                            };
                            let argument = self.retyped(argument, budget)?;
                            let before = self.type_standing[variable.index()].replace(argument);
                            tasks.push(Task::Restore(variable, before));
                            tasks.push(Task::Enter(*body));
                            continue;
                        }
                        Some(_) | None => {}
                    }
                    if let Node::Var { variable } = node {
                        if let Some((_, Redex::Term(x, argument))) = self.redex
                            && x == *variable
                        {
                            tasks.push(Task::Enter(argument));
                            continue;
                        }
                        let variable = self.standing[variable.index()]
                            .expect("a checked term binds or declares every variable it uses");
                        let copy = self.push(Node::Var { variable }, id, budget)?;
                        made.push(copy);
                        continue;
                    }
                    let mut hidden = Vec::new();
                    for v in binders(node) {
                        let fresh = self.fresh(v, budget)?;
                        hidden.push((v, self.standing[v.index()].replace(fresh)));
                    }
                    let mut hidden_type = None;
                    if let Node::TypeLambda { variable: x, .. } = node {
                        let fresh = self.fresh_type(*x);
                        let ty = self.types.add(Type::Variable { variable: fresh });
                        hidden_type = Some((*x, self.type_standing[x.index()].replace(ty)));
                    }
                    tasks.push(Task::Exit(id, hidden, hidden_type));
                    let first = tasks.len();
                    super::operands(node, |t| tasks.push(Task::Enter(t)));
                    tasks[first..].reverse();
                }
                Task::Restore(x, before) => self.type_standing[x.index()] = before,
                Task::Exit(id, hidden, hidden_type) => {
                    let mut node = self.term.node(id).clone();
                    let mut count = 0;
                    super::operands(&node, |_| count += 1);
                    let operands = made.split_off(made.len() - count);
                    // The operands are renumbered in the order written,
                    // which is the order their copies were made in.
                    let next = std::cell::Cell::new(0);
                    super::renumber(&mut node, |_| {
                        let i = next.get();
                        next.set(i + 1);
                        operands[i]
                    });
                    let standing = &self.standing;
                    rebind(&mut node, |v| {
                        standing[v.index()].expect("a node's variables stand for one of the copy")
                    });
                    if let Node::Left { sum: ty, .. }
                    | Node::Right { sum: ty, .. }
                    | Node::False { goal: ty, .. }
                    | Node::TypeApply { argument: ty, .. } = &mut node
                    {
                        *ty = self.retyped(*ty, budget)?;
                    }
                    if let Node::TypeLambda { variable, .. } = &mut node {
                        let stands =
                            self.type_standing[variable.index()].map(|ty| self.types.get(ty));
                        let Some(Type::Variable { variable: copy }) = stands else {
                            unreachable!("a type abstraction copied stands for its copy's");
                        };
                        *variable = copy;
                    }
                    for (v, before) in hidden.into_iter().rev() {
                        self.standing[v.index()] = before;
                    }
                    if let Some((x, before)) = hidden_type {
                        self.type_standing[x.index()] = before;
                    }
                    let copy = self.push(node, id, budget)?;
                    made.push(copy);
                }
            }
        }
        Ok(made.pop().expect("the root's copy"))
    }
}

/// Returns the variables a node binds.
fn binders(node: &Node) -> Vec<VarId> {
    match node {
        Node::Lambda { variable, .. } => vec![*variable],
        Node::LetTensor { left, right, .. }
        | Node::Case { left, right, .. }
        | Node::Copy { left, right, .. } => vec![*left, *right],
        Node::Promote { variables, .. } => variables.to_vec(),
        Node::Var { .. }
        | Node::Apply { .. }
        | Node::Tensor { .. }
        | Node::Unit
        | Node::LetUnit { .. }
        | Node::Pair { .. }
        | Node::First { .. }
        | Node::Second { .. }
        | Node::Left { .. }
        | Node::Right { .. }
        | Node::True { .. }
        | Node::False { .. }
        | Node::Derelict { .. }
        | Node::Discard { .. }
        | Node::TypeLambda { .. }
        | Node::TypeApply { .. } => Vec::new(),
    }
}

/// Replaces every variable a node names, bound or used, by what `f` maps
/// it to.
fn rebind(node: &mut Node, f: impl Fn(VarId) -> VarId) {
    match node {
        Node::Var { variable } | Node::Lambda { variable, .. } => *variable = f(*variable),
        Node::LetTensor { left, right, .. }
        | Node::Case { left, right, .. }
        | Node::Copy { left, right, .. } => {
            *left = f(*left);
            *right = f(*right);
        }
        Node::Promote { variables, .. } => variables.iter_mut().for_each(|v| *v = f(*v)),
        Node::Apply { .. }
        | Node::Tensor { .. }
        | Node::Unit
        | Node::LetUnit { .. }
        | Node::Pair { .. }
        | Node::First { .. }
        | Node::Second { .. }
        | Node::Left { .. }
        | Node::Right { .. }
        | Node::True { .. }
        | Node::False { .. }
        | Node::Derelict { .. }
        | Node::Discard { .. }
        | Node::TypeLambda { .. }
        | Node::TypeApply { .. } => {}
    }
}

/// Returns a copy of `term` as a tree, every binder a variable of its own
/// and the context's variables first, with the redex at `redex` reduced
/// where one is given, and the node of `term` each node of the copy was
/// copied from.
pub(super) fn copied(
    term: &Term,
    redex: Option<TermId>,
    limit: Option<u64>,
    budget: &mut Budget<'_>,
) -> Result<(Term, Vec<TermId>), Halt> {
    let mut copy = Copier {
        term,
        standing: vec![None; term.variables.len()],
        variables: Vec::with_capacity(term.variables.len()),
        types: Types::of(&term.types, 0),
        type_variables: term.type_variables.clone(),
        type_standing: vec![None; term.type_variables.len()],
        nodes: Vec::new(),
        origin: Vec::new(),
        redex: redex.map(|id| (id, redex_of(term, id).expect("a redex applies a λ or a Λ"))),
        limit,
    };
    let mut context: Vec<VarId> = Vec::with_capacity(term.context.len());
    for &v in &term.context {
        let fresh = copy.fresh(v, budget)?;
        copy.standing[v.index()] = Some(fresh);
        context.push(fresh);
    }
    let root = copy.subterm(term.root, budget)?;
    let mut made = Term {
        calculus: term.calculus,
        atoms: term.atoms.clone(),
        types: copy.types.list,
        variables: copy.variables,
        type_variables: copy.type_variables,
        context,
        goal: term.goal,
        nodes: copy.nodes,
        root,
    };
    rename(&mut made, budget).map_err(Halt::Refused)?;
    rename_types(&mut made, budget).map_err(Halt::Refused)?;
    Ok((made, copy.origin))
}

/// Renames every type abstraction of a tree whose type variable would
/// take in a free type variable of its name, or has an atom's name, to a
/// name the term does not use, so that its text names the right types.
fn rename_types(term: &mut Term, budget: &mut Budget<'_>) -> Result<(), Refusal> {
    let mut counts: HashMap<Box<str>, u32> = HashMap::default();
    for name in term.atoms.iter().chain(&term.type_variables) {
        let count = counts.entry(name.clone()).or_default();
        *count = count.saturating_add(1);
    }
    let clashes = term.nodes.iter().any(|node| {
        matches!(node, Node::TypeLambda { variable, .. }
            if counts[&term.type_variables[variable.index()]] > 1)
    });
    if !clashes {
        return Ok(());
    }
    let atoms: HashSet<Box<str>> = term.atoms.iter().cloned().collect();
    let mut taken: HashSet<Box<str>> = counts.into_keys().collect();
    let mut walker = poly::Walker::new(term.types.len());
    // The free type variables of each node, moved up the tree: each node
    // of a tree has one user.
    let mut of: Vec<Option<Vec<TypeVarId>>> = Vec::with_capacity(term.nodes.len());
    for i in 0..term.nodes.len() {
        let node = &term.nodes[i];
        let mut own: Vec<TypeVarId> = Vec::new();
        let mut writes: Vec<TypeId> = Vec::new();
        super::check::written(term, node, |t| writes.push(t));
        for t in writes {
            let (found, visited) = walker.variables(&term.types, t);
            budget.tick(visited, i as u64)?;
            own.extend(found);
        }
        super::operands(node, |t| {
            own.extend(of[t.index()].take().unwrap_or_default())
        });
        own.sort_unstable();
        own.dedup();
        budget.tick(own.len() as u64 + 1, i as u64)?;
        if let Node::TypeLambda { variable, .. } = *node {
            own.retain(|&v| v != variable);
            let name = &term.type_variables[variable.index()];
            let captures = own.iter().any(|v| term.type_variables[v.index()] == *name);
            if atoms.contains(name) || captures {
                let fresh = super::print::fresh(name, |c| taken.contains(c));
                taken.insert(fresh.clone());
                term.type_variables[variable.index()] = fresh;
            }
        }
        of.push(Some(own));
    }
    Ok(())
}

/// Renames every binder of a tree that would take in a free variable of
/// its name, to a name the term does not use, so that its text names
/// the right variables.
fn rename(term: &mut Term, budget: &mut Budget<'_>) -> Result<(), Refusal> {
    let mut names: HashMap<Box<str>, u32> = HashMap::default();
    for variable in &term.variables {
        let count = names.entry(variable.name.clone()).or_default();
        *count = count.saturating_add(1);
    }
    if names.values().all(|&count| count == 1) {
        return Ok(());
    }
    let taken: HashSet<Box<str>> = names.keys().cloned().collect();
    let mut taken = taken;
    // The free variables of each node, moved up the tree: each node of a
    // tree has one user.
    let mut free: Vec<Option<HashSet<VarId>>> = Vec::with_capacity(term.nodes.len());
    for i in 0..term.nodes.len() {
        budget.tick(1, i as u64)?;
        let node = &term.nodes[i];
        let mut set: HashSet<VarId> = HashSet::default();
        let mut parts: Vec<HashSet<VarId>> = Vec::new();
        super::operands(node, |t| {
            parts.push(free[t.index()].take().unwrap_or_default());
        });
        for part in parts {
            if part.len() > set.len() {
                let small = std::mem::replace(&mut set, part);
                set.extend(small);
            } else {
                set.extend(part);
            }
        }
        if let Node::Var { variable } = node {
            set.insert(*variable);
        }
        for v in binders(node) {
            set.remove(&v);
            let name = &term.variables[v.index()].name;
            if names.get(name).copied().unwrap_or(0) > 1 {
                budget.tick(set.len() as u64, i as u64)?;
                if set.iter().any(|&w| term.variables[w.index()].name == *name) {
                    let fresh = super::print::fresh(name, |c| taken.contains(c));
                    taken.insert(fresh.clone());
                    term.variables[v.index()].name = fresh;
                }
            }
        }
        free.push(Some(set));
    }
    Ok(())
}

impl Term {
    /// Reduces the term step by step by β, until it is in normal form or
    /// a bound or `stop` ends the reduction.
    ///
    /// Each step reduces the redex `(λx. M) N` that the strategy of
    /// `options` picks to `M` with `N` for `x`, or the type redex
    /// `(ΛX. M) [A]` of System F to `M` with `A` for `X` in its types,
    /// renaming a binder where it would take in a free variable of its
    /// name. A term whose nodes are
    /// shared, such as one read back from a proof, is first written out
    /// as a tree, which is the term the first step is taken on. Only the
    /// application of a `λ` is a redex: a pair taken apart, a `case` of
    /// an injection and the linear calculus's other constructs stay as
    /// they are.
    ///
    /// Every term a step makes is checked ([`check`](Self::check)) within
    /// `limits` before the next step is taken on it. `each` gets every
    /// step with the term it was taken on, once the term the step made
    /// has passed its check: a caller prints every step there.
    ///
    /// An untyped term need not have a normal form: `(λx. x x) (λx. x x)`
    /// reduces to itself. So a reduction that reaches the bound on steps
    /// ends with [`End::Steps`], which says that it is not finished, not
    /// that the term has no normal form. A typed term, and an untyped
    /// linear or affine one, always reaches one.
    ///
    #[cfg_attr(feature = "parse", doc = "```")]
    #[cfg_attr(not(feature = "parse"), doc = "```ignore")]
    /// use deduxit::Limits;
    /// use deduxit::lambda::{Calculus, End, ReduceOptions, Term};
    ///
    /// let limits = Limits::default();
    /// let term = Term::parse(r"(\x. \y. x) a b", Calculus::Untyped, &limits)?;
    /// let options = ReduceOptions::default();
    /// let mut seen = Vec::new();
    /// let done = term.reduce(&options, &limits, |_| false, |before, _| {
    ///     seen.push(before.to_string());
    /// })?;
    /// assert_eq!(done.end, End::NormalForm);
    /// assert_eq!(done.term.to_string(), "a");
    /// assert_eq!(seen, ["(λx. λy. x) a b", "(λy. a) b"]);
    ///
    /// // Ω does not normalise.
    /// let omega = Term::parse(r"(\x. x x) (\x. x x)", Calculus::Untyped, &limits)?;
    /// let options = options.with_steps(Some(3));
    /// let done = omega.reduce(&options, &limits, |_| false, |_, _| {})?;
    /// assert!(matches!(done.end, End::Steps { limit: 3, .. }));
    /// # Ok::<(), deduxit::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// - The checker's error for a term that is not well typed in its
    ///   calculus, or its refusal of that first check.
    /// - [`Error::ClaimFails`] where the checker refuses a term a step
    ///   made: a defect of this crate.
    pub fn reduce(
        &self,
        options: &ReduceOptions,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
        mut each: impl FnMut(&Self, &Step),
    ) -> Result<Reduction, Error> {
        self.reduce_in(options, limits, &mut stop, &mut each)
    }

    /// Reduces the term as [`reduce`](Self::reduce) does, the stop and
    /// the callback as trait objects, so that the body is compiled once.
    fn reduce_in(
        &self,
        options: &ReduceOptions,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
        each: &mut dyn FnMut(&Self, &Step),
    ) -> Result<Reduction, Error> {
        self.check(limits, &mut *stop)?;
        let mut budget = budget(limits, stop);
        let mut steps = Vec::new();
        let ended = |term: Self, steps: Vec<Step>, end: End| Reduction {
            strategy: options.strategy,
            term,
            steps,
            end,
        };
        let mut current = if needs_copy(self) {
            match copied(self, None, options.nodes, &mut budget) {
                Ok((tree, _)) => tree,
                Err(Halt::Nodes(limit)) => {
                    return Ok(ended(self.clone(), steps, End::Nodes { limit }));
                }
                Err(Halt::Refused(refusal)) => {
                    return Ok(ended(self.clone(), steps, End::Refused(refusal)));
                }
            }
        } else {
            self.clone()
        };
        loop {
            let Some(redex) = options.strategy.pick(&current) else {
                return Ok(ended(current, steps, End::NormalForm));
            };
            if let Some(limit) = options.steps
                && steps.len() as u64 >= limit
            {
                return Ok(ended(current, steps, End::Steps { limit }));
            }
            let made = match copied(&current, Some(redex), options.nodes, &mut budget) {
                Ok((made, _)) => made,
                Err(Halt::Nodes(limit)) => return Ok(ended(current, steps, End::Nodes { limit })),
                Err(Halt::Refused(refusal)) => {
                    return Ok(ended(current, steps, End::Refused(refusal)));
                }
            };
            match made.check(limits, &mut *budget.stop) {
                Ok(_) => {}
                Err(Error::Refused(refusal)) => {
                    return Ok(ended(current, steps, End::Refused(refusal)));
                }
                Err(error) => {
                    return Err(Error::ClaimFails {
                        claim: "a β step keeps the term's type and its variables' uses".to_owned(),
                        reason: error.to_string(),
                    });
                }
            }
            let step = Step { redex };
            each(&current, &step);
            steps.push(step);
            current = made;
        }
    }
}

/// Returns whether a step cannot be taken on the term as it is: a node
/// other than a variable's has more than one place, or a variable or a
/// type variable is bound twice, or a variable bound and declared, so
/// that a copy of a subterm could be taken in by a binder of its own
/// variable.
fn needs_copy(term: &Term) -> bool {
    let mut users = vec![0u32; term.nodes.len()];
    users[term.root.index()] = 1;
    for node in &term.nodes {
        super::operands(node, |t| {
            users[t.index()] = users[t.index()].saturating_add(1)
        });
    }
    let mut bound = vec![false; term.variables.len()];
    for &v in &term.context {
        bound[v.index()] = true;
    }
    let mut type_bound = vec![false; term.type_variables.len()];
    term.nodes.iter().zip(&users).any(|(node, &count)| {
        let shared = count > 1 && !matches!(node, Node::Var { .. });
        let twice = binders(node)
            .into_iter()
            .any(|v| std::mem::replace(&mut bound[v.index()], true));
        let type_twice = matches!(node, Node::TypeLambda { variable, .. }
            if std::mem::replace(&mut type_bound[variable.index()], true));
        shared || twice || type_twice
    })
}

/// Returns a budget of `limits`'s work and memory, asking `stop`.
pub(super) fn budget<'s>(limits: &Limits, stop: &'s mut dyn FnMut(Progress) -> bool) -> Budget<'s> {
    Budget {
        work: 0,
        most: limits.work,
        memory: limits.memory_bytes,
        next: POLL,
        stop,
    }
}
