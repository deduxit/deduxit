// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Herbrand's theorem: a first-order sequent is valid in classical logic
//! exactly when finitely many ground instances of the clauses of its
//! negation have no model.
//!
//! [`herbrand`] makes the sequent's clausal form
//! ([`normal::Form::Clausal`], every step shown), then puts closed terms
//! for the clauses' variables, the shallowest first: the constants, then
//! each function symbol applied to them, and so on to a depth. At each
//! depth the SAT solver decides the ground clauses, whose atoms it reads
//! as propositional atoms; its clausal proof is checked. The first depth
//! whose instances have no model gives the certificate: those instances,
//! cut down to a set the solver still refutes.
//!
//! From the instances [`herbrand`] then builds a derivation of LK of the
//! sequent as written, which [`Derivation::check`] accepts. Each Skolem
//! term of the clausal form becomes an eigenvariable again, the least
//! nested first: `∀R` and `∃L` introduce the eigenvariable of the Skolem
//! term the quantifier was replaced by, and `∀L` and `∃R` take the terms
//! of the instances as witnesses. So a valid verdict rests on the checker
//! alone, not on the clausal form's construction.
//!
//! A sequent that is not valid has no such set, and the search ends
//! unknown at its depth: [`decide`](fn@super::decide) decides such a
//! sequent where grounding or the model search refutes it.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::herbrand::{self, Options};
//! use deduxit::ordinary::{Sequent, Verdict};
//!
//! let sequent: Sequent = "forall x. p(x) |- p(f(a))".parse()?;
//! let found = herbrand::herbrand_default(&sequent, &Options::default())?;
//! assert_eq!(found.ground.to_string(), "p(f(a)), ¬p(f(a)) ⊢");
//! assert!(matches!(found.verdict, Verdict::Valid(_)));
//! # Ok::<(), deduxit::Error>(())
//! ```

use super::bruijn::Replaced;
use super::derivation::Inference;
use super::normal::{self, NormalForm, Rule as NormalRule};
use super::sat;
use super::{
    Derivation, Evidence, Formulas, Logic, Node, NodeId, Outcome, Rule, Sequent, Side, Verdict,
};
use crate::Error;
use crate::first_order::{Eigen, FoTerm, FoTermId, Symbol};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal, Space};
use crate::linear::proofs::InfId;
use crate::linear::search::Reason;

/// How deep the terms of the instances go by default.
pub const DEFAULT_DEPTH: u32 = 3;

/// The most instances cut down one by one; a larger set is kept whole.
const MOST_CUT: usize = 256;

/// How [`herbrand`] searches.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The deepest term put for a variable, a constant being of depth 0;
    /// `None` deepens until a bound of the limits refuses.
    pub depth: Option<u32>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            depth: Some(DEFAULT_DEPTH),
        }
    }
}

impl Options {
    /// Returns the options with the depth given.
    #[must_use]
    pub const fn with_depth(self, depth: Option<u32>) -> Self {
        Self { depth }
    }
}

/// A ground instance of a clause.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instance {
    /// The clause's position among the clausal form's hypotheses.
    pub clause: usize,
    /// The closed terms put for the clause's variables, the outermost's
    /// first, in the arena of [`Herbrand::ground`].
    pub terms: Vec<FoTermId>,
    /// The instance, in that arena.
    pub formula: NodeId,
}

/// What the search cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Statistics {
    /// The closed terms of the deepest level tried.
    pub terms: u64,
    /// The instances decided at that level.
    pub instances: u64,
    /// The instances the certificate keeps.
    pub kept: u64,
    /// The calls of the SAT solver.
    pub calls: u64,
    /// The inferences of the derivation built.
    pub inferences: u64,
}

/// What [`herbrand`] found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Herbrand {
    /// The sequent decided.
    pub sequent: Sequent,
    /// Its clausal form, with the steps that made it.
    pub clausal: NormalForm,
    /// The depth of the terms of the last level tried.
    pub depth: u32,
    /// The ground instances of the last level tried, cut down to those the
    /// certificate keeps where the solver refuted them.
    pub instances: Vec<Instance>,
    /// The instances as a sequent, `C₁σ₁, …, Cₙσₙ ⊢`, whose arena holds the
    /// clausal form's.
    pub ground: Sequent,
    /// The SAT solver's outcome on the instances read as propositional
    /// clauses: valid with a checked clausal proof where they have no
    /// model.
    pub refutation: Option<Outcome>,
    /// The verdict: valid with the derivation of LK built from the
    /// instances, checked, or with the instances' clausal proof where a
    /// bound refused the derivation; unknown at the depth or a bound.
    pub verdict: Verdict,
    /// What the search cost.
    pub statistics: Statistics,
}

impl Herbrand {
    /// Returns what `instance` puts for its clause's variables: each
    /// variable's name, the outermost's first, with its term as text.
    pub fn bindings(&self, instance: &Instance) -> Vec<(String, String)> {
        let formulas = self.ground.formulas();
        let mut names = Vec::new();
        if let Some(&root) = self.clausal.result.left().get(instance.clause) {
            let mut node = root;
            while let Node::Forall(body) = formulas.node(node) {
                let name = formulas.first_order().binder_name(node);
                names.push(name.map_or_else(|| format!("x{}", names.len() + 1), str::to_owned));
                node = body;
            }
        }
        names
            .into_iter()
            .zip(&instance.terms)
            .map(|(name, &t)| (name, formulas.term_text(t)))
            .collect()
    }
}

/// Searches for ground instances of the clausal form of `sequent` that
/// have no model, within `options`, `limits` and `stop`, and builds the
/// derivation of LK they give.
///
/// # Errors
///
/// [`Refusal::Form`], [`Refusal::Terms`] and [`Refusal::Occurrences`]
/// where the clausal form or the instances pass the bounds of `limits`;
/// [`Error::ReadBack`] where the derivation built from a refutation is
/// rejected, a defect; the SAT solver's errors. A stop, the memory and
/// the work end the search as unknown.
pub fn herbrand(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Herbrand, Error> {
    herbrand_dyn(sequent, options, limits, &mut stop)
}

/// Searches as [`herbrand`] does within the default [`Limits`] and with no
/// stop.
///
/// # Errors
///
/// As [`herbrand`].
pub fn herbrand_default(sequent: &Sequent, options: &Options) -> Result<Herbrand, Error> {
    herbrand(sequent, options, &Limits::default(), |_| false)
}

/// [`herbrand`], with the stop as a trait object.
fn herbrand_dyn(
    sequent: &Sequent,
    options: &Options,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Herbrand, Error> {
    let clausal = normal::rewrite(
        sequent,
        &normal::Options::default().with_form(normal::Form::Clausal),
        limits,
        &mut *stop,
    )?;
    let mut formulas = clausal.result.formulas().clone();
    let clauses = clauses_of(&formulas, clausal.result.left());
    let universe = Universe::of(&mut formulas, &clauses)?;
    let mut search = Search {
        formulas,
        clauses,
        universe,
        limits,
        statistics: Statistics::default(),
    };
    let mut depth = 0;
    let (instances, refutation, reason) = loop {
        let instances = match search.instances(depth, &mut *stop) {
            Ok(instances) => instances,
            Err(error) => {
                let reason = super::decide::ended(error)?;
                break (Vec::new(), None, Some(reason));
            }
        };
        let decided = search.decide(&instances, &mut *stop);
        let decided = match decided {
            Ok(decided) => decided,
            Err(error) => {
                let reason = super::decide::ended(error)?;
                break (instances, None, Some(reason));
            }
        };
        match decided.verdict {
            Verdict::Valid(_) => {
                let (kept, decided) = search.cut(instances, decided, &mut *stop)?;
                break (kept, Some(decided), None);
            }
            // A bound or the stop ended the solver: the instances may have
            // no model.
            Verdict::Unknown(reason) => break (instances, Some(decided), Some(reason)),
            Verdict::NotValid => {}
        }
        let complete = search.universe.functions.is_empty();
        let last = options.depth.is_some_and(|most| depth >= most);
        if complete || last {
            let reason = if complete {
                Reason::Incomplete
            } else {
                Reason::WitnessDepth { depth }
            };
            break (instances, Some(decided), Some(reason));
        }
        depth += 1;
    };
    search.statistics.kept = instances.len() as u64;
    let Search {
        formulas,
        universe,
        mut statistics,
        ..
    } = search;
    let ground = Sequent::new(
        formulas,
        instances.iter().map(|i| i.formula).collect(),
        Vec::new(),
    )?;
    let verdict = match (reason, &refutation) {
        (Some(reason), _) => Verdict::Unknown(reason),
        (None, Some(decided)) => {
            match deskolemized(sequent, &clausal, &ground, &universe, limits, &mut *stop) {
                Ok(derivation) => {
                    statistics.inferences = derivation.inferences().len() as u64;
                    Verdict::Valid(Evidence::Derivation(Box::new(derivation)))
                }
                Err(error) => {
                    // A bound refused the derivation: the instances' proof
                    // stands for it.
                    super::decide::ended(error)?;
                    decided.verdict.clone()
                }
            }
        }
        (None, None) => unreachable!("a refutation is kept with its verdict"),
    };
    Ok(Herbrand {
        sequent: sequent.clone(),
        clausal,
        depth,
        instances,
        ground,
        refutation: refutation.map(Outcome::from),
        verdict,
        statistics,
    })
}

/// A clause of the clausal form: its hypothesis, how many `∀` stand
/// before its matrix, and the matrix.
#[derive(Clone, Copy, Debug)]
struct Clause {
    /// The variables its prefix binds.
    variables: usize,
    /// The matrix under the prefix.
    matrix: NodeId,
}

/// Returns the clauses of the clausal form's hypotheses `left`.
fn clauses_of(formulas: &Formulas, left: &[NodeId]) -> Vec<Clause> {
    left.iter()
        .map(|&root| {
            let (mut variables, mut matrix) = (0, root);
            while let Node::Forall(body) = formulas.node(matrix) {
                variables += 1;
                matrix = body;
            }
            Clause { variables, matrix }
        })
        .collect()
}

/// The terms the instances are made of: the constants and function
/// symbols of the clauses, and the closed terms made so far by depth.
#[derive(Clone, Debug)]
struct Universe {
    /// The function symbols with arguments, with their arities.
    functions: Vec<(Symbol, usize)>,
    /// The closed terms by depth: the constants first.
    levels: Vec<Vec<FoTermId>>,
}

impl Universe {
    /// Returns the universe of `clauses`: their constants, or a new one
    /// where they have none.
    fn of(formulas: &mut Formulas, clauses: &[Clause]) -> Result<Self, Error> {
        let tables = formulas.first_order();
        let mut constants: Vec<FoTermId> = Vec::new();
        let mut functions: Vec<(Symbol, usize)> = Vec::new();
        let mut seen_nodes: HashSet<NodeId> = HashSet::default();
        let mut seen_terms: HashSet<FoTermId> = HashSet::default();
        let mut nodes: Vec<NodeId> = clauses.iter().map(|c| c.matrix).collect();
        let mut terms: Vec<FoTermId> = Vec::new();
        while let Some(n) = nodes.pop() {
            if !seen_nodes.insert(n) {
                continue;
            }
            match formulas.node(n) {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend_from_slice(tables.list(args));
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => nodes.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    nodes.extend([a, b]);
                }
            }
        }
        while let Some(t) = terms.pop() {
            if !seen_terms.insert(t) {
                continue;
            }
            if let FoTerm::App(f, args) = tables.terms[t.index()] {
                let list = tables.list(args);
                if list.is_empty() {
                    constants.push(t);
                } else if !functions.iter().any(|&(g, _)| g == f) {
                    functions.push((f, list.len()));
                }
                terms.extend_from_slice(list);
            }
        }
        constants.sort_unstable();
        functions.sort_unstable();
        // A clause with variables needs a term; a propositional sequent's
        // arena stays without tables.
        if constants.is_empty() && clauses.iter().any(|c| c.variables > 0) {
            constants.push(fresh_constant(formulas)?);
        }
        Ok(Self {
            functions,
            levels: vec![constants],
        })
    }

    /// Makes the terms of depth `depth`, each function applied to terms
    /// shallower whose deepest is one less, within `limit` terms in all.
    fn deepen(
        &mut self,
        formulas: &mut Formulas,
        limit: Option<u64>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let below: Vec<FoTermId> = self.levels.iter().flatten().copied().collect();
        let last: HashSet<FoTermId> = self.levels.last().into_iter().flatten().copied().collect();
        let mut made = Vec::new();
        for &(f, arity) in &self.functions {
            // Every tuple over `below`, in the order of a counter.
            let mut at = vec![0usize; arity];
            loop {
                let args: Vec<FoTermId> = at.iter().map(|&k| below[k]).collect();
                if args.iter().any(|a| last.contains(a)) {
                    let list = formulas.interner().list(&args);
                    made.push(formulas.interner().term(FoTerm::App(f, list)));
                    let all = below.len() + made.len();
                    asked(stop, all as u64)?;
                    if let Some(limit) = limit
                        && all as u64 > limit
                    {
                        return Err(Error::Refused(Refusal::Terms {
                            terms: all as u64,
                            limit,
                        }));
                    }
                }
                let Some(k) = (0..arity).rev().find(|&k| at[k] + 1 < below.len()) else {
                    break;
                };
                at[k] += 1;
                at[k + 1..].fill(0);
            }
        }
        self.levels.push(made);
        Ok(())
    }
}

/// Asks the stop once in 1 024 calls, `count` the things made so far, and
/// refuses when it fires.
fn asked(stop: &mut dyn FnMut(Progress) -> bool, count: u64) -> Result<(), Error> {
    if count.is_multiple_of(1024) && stop(Progress::new(Phase::Search, 1024, count)) {
        return Err(Error::Refused(Refusal::Stopped {
            phase: Phase::Search,
        }));
    }
    Ok(())
}

/// Returns a constant of a new name, `c1` or with `_` after `c` until no
/// name of the arena is `c` and digits.
fn fresh_constant(formulas: &mut Formulas) -> Result<FoTermId, Error> {
    let name = format!("{}1", normal::fresh_prefix(formulas, "c"));
    let symbol = formulas
        .interner()
        .symbol(&name, 0, false)
        .map_err(|fault| Error::Symbol { name, fault })?;
    let list = formulas.interner().list(&[]);
    Ok(formulas.interner().term(FoTerm::App(symbol, list)))
}

/// The search over the levels of the universe.
struct Search<'l> {
    /// The clausal form's arena, which the instances extend.
    formulas: Formulas,
    /// The clauses.
    clauses: Vec<Clause>,
    /// The universe so far.
    universe: Universe,
    /// The bounds.
    limits: &'l Limits,
    /// What the search cost so far.
    statistics: Statistics,
}

impl Search<'_> {
    /// Returns every instance of every clause over the terms up to depth
    /// `depth`, within `limits.occurrences` atoms in all.
    fn instances(
        &mut self,
        depth: u32,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Vec<Instance>, Error> {
        while self.universe.levels.len() <= depth as usize {
            self.universe
                .deepen(&mut self.formulas, self.limits.terms, &mut *stop)?;
        }
        let terms: Vec<FoTermId> = self.universe.levels.iter().flatten().copied().collect();
        self.statistics.terms = terms.len() as u64;
        let mut instances = Vec::new();
        let mut size = 0u64;
        for (k, clause) in self.clauses.iter().enumerate() {
            let mut at = vec![0usize; clause.variables];
            loop {
                let chosen: Vec<FoTermId> = at.iter().map(|&i| terms[i]).collect();
                // The frame is the innermost binder's first.
                let frame: Vec<FoTermId> = chosen.iter().rev().copied().collect();
                let formula = self.formulas.instantiate_frame(clause.matrix, &frame)?;
                size = size.saturating_add(1);
                asked(stop, size)?;
                if let Some(limit) = self.limits.occurrences
                    && size > limit
                {
                    return Err(Error::Refused(Refusal::Occurrences {
                        occurrences: size,
                        limit,
                    }));
                }
                instances.push(Instance {
                    clause: k,
                    terms: chosen,
                    formula,
                });
                let Some(i) = (0..clause.variables)
                    .rev()
                    .find(|&i| at[i] + 1 < terms.len())
                else {
                    break;
                };
                at[i] += 1;
                at[i + 1..].fill(0);
            }
        }
        self.statistics.instances = instances.len() as u64;
        Ok(instances)
    }

    /// Decides `instances` by the SAT solver, their ground atoms read as
    /// propositional atoms.
    fn decide(
        &mut self,
        instances: &[Instance],
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<sat::Outcome, Error> {
        let sequent = propositional(&self.formulas, instances)?;
        self.statistics.calls += 1;
        sat::decide(&sequent, Logic::Classical, self.limits, &mut *stop)
    }

    /// Returns the instances the solver still refutes when every other is
    /// dropped that it refutes without, one at a time, with the outcome
    /// on those; a set larger than [`MOST_CUT`] is kept whole.
    fn cut(
        &mut self,
        mut instances: Vec<Instance>,
        mut decided: sat::Outcome,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(Vec<Instance>, sat::Outcome), Error> {
        if instances.len() > MOST_CUT {
            return Ok((instances, decided));
        }
        let mut k = 0;
        while k < instances.len() {
            let mut fewer = instances.clone();
            fewer.remove(k);
            let tried = match self.decide(&fewer, &mut *stop) {
                Ok(tried) => tried,
                // A bound ends the cut, not the search.
                Err(error) => {
                    super::decide::ended(error)?;
                    break;
                }
            };
            if matches!(tried.verdict, Verdict::Valid(_)) {
                instances = fewer;
                decided = tried;
            } else {
                k += 1;
            }
        }
        Ok((instances, decided))
    }
}

/// Returns `instances` as a propositional sequent `C₁σ₁, …, Cₙσₙ ⊢`, each
/// ground atom a propositional atom of its own.
fn propositional(formulas: &Formulas, instances: &[Instance]) -> Result<Sequent, Error> {
    let mut out = Formulas::default();
    let mut made: HashMap<NodeId, NodeId> = HashMap::default();
    let mut left = Vec::with_capacity(instances.len());
    for instance in instances {
        // Operands first, each node once.
        let mut stack = vec![(instance.formula, false)];
        while let Some((n, ready)) = stack.pop() {
            if made.contains_key(&n) {
                continue;
            }
            let node = formulas.node(n);
            if !ready {
                stack.push((n, true));
                let (a, b) = node.operands();
                stack.extend([a, b].into_iter().flatten().map(|k| (k, false)));
                continue;
            }
            let of = |k: NodeId| made[&k];
            let image = match node {
                // The solver reads atoms alone: a number names each.
                Node::Atom(_) => out.atom(&format!("g{}", out.atom_names().len() + 1))?,
                Node::True => out.add(Node::True)?,
                Node::False => out.add(Node::False)?,
                Node::Not(a) => out.add(Node::Not(of(a)))?,
                Node::And(a, b) => out.add(Node::And(of(a), of(b)))?,
                Node::Or(a, b) => out.add(Node::Or(of(a), of(b)))?,
                Node::Implies(a, b) => out.add(Node::Implies(of(a), of(b)))?,
                Node::Iff(a, b) => out.add(Node::Iff(of(a), of(b)))?,
                Node::Forall(_) | Node::Exists(_) => unreachable!("an instance is ground"),
            };
            made.insert(n, image);
        }
        left.push(made[&instance.formula]);
    }
    Sequent::new(out, left, Vec::new())
}

/// Returns the derivation of LK of `sequent` that the instances of
/// `ground` give, each Skolem term of `clausal` an eigenvariable, checked.
fn deskolemized(
    sequent: &Sequent,
    clausal: &NormalForm,
    ground: &Sequent,
    universe: &Universe,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Derivation, Error> {
    // The clausal arena began as a copy of the input's, so the input's
    // ids are its ids.
    let mut formulas = ground.formulas().clone();
    let skolems: Vec<(NodeId, FoTermId)> = clausal
        .steps
        .iter()
        .filter(|s| s.rule == NormalRule::Skolemize)
        .filter_map(|s| s.term.map(|t| (s.before, t)))
        .collect();
    let symbols: HashSet<Symbol> = skolems
        .iter()
        .filter_map(|&(_, t)| match formulas.first_order().terms[t.index()] {
            FoTerm::App(f, _) => Some(f),
            FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => None,
        })
        .collect();
    let mut witnesses = witnesses_of(&formulas, ground.left());
    // A first `∀L` or `∃R` may need a witness before any `∀R` or `∃L` has
    // introduced a Skolem term: a constant of the universe that is none.
    let skolem = |t: FoTermId| match formulas.first_order().terms[t.index()] {
        FoTerm::App(f, _) => symbols.contains(&f),
        FoTerm::Bound(_) | FoTerm::Eigen(_) | FoTerm::Meta(_) => false,
    };
    let plain = universe
        .levels
        .first()
        .into_iter()
        .flatten()
        .copied()
        .find(|&t| !skolem(t));
    let plain = match plain {
        Some(t) => Some(t),
        None if sequent.formulas().is_first_order() => Some(fresh_constant(&mut formulas)?),
        None => None,
    };
    if let Some(plain) = plain
        && !witnesses.contains(&plain)
    {
        witnesses.insert(0, plain);
    }
    let mut build = Build {
        formulas,
        skolems,
        symbols,
        witnesses,
        keys: HashMap::default(),
        instances: HashMap::default(),
        made: Vec::new(),
        inferences: Vec::new(),
        bytes: 0,
        limits,
    };
    let mut root = Branch::default();
    for &node in sequent.left() {
        root.items.push(Item::new(node, true));
    }
    for &node in sequent.right() {
        root.items.push(Item::new(node, false));
    }
    build.derive(root, stop)?;
    let derivation = build.deskolemized(sequent)?;
    derivation.check(limits, &mut *stop)?;
    Ok(derivation)
}

/// Returns every closed term of the instances `roots` and every term
/// inside one, the shallowest first.
fn witnesses_of(formulas: &Formulas, roots: &[NodeId]) -> Vec<FoTermId> {
    let tables = formulas.first_order();
    let mut seen_nodes: HashSet<NodeId> = HashSet::default();
    let mut seen: HashSet<FoTermId> = HashSet::default();
    let mut nodes = roots.to_vec();
    let mut terms: Vec<FoTermId> = Vec::new();
    while let Some(n) = nodes.pop() {
        if !seen_nodes.insert(n) {
            continue;
        }
        match formulas.node(n) {
            Node::Atom(a) => {
                if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                    terms.extend_from_slice(tables.list(args));
                }
            }
            Node::True | Node::False => {}
            Node::Not(a) | Node::Forall(a) | Node::Exists(a) => nodes.push(a),
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                nodes.extend([a, b]);
            }
        }
    }
    let mut found = Vec::new();
    while let Some(t) = terms.pop() {
        if !seen.insert(t) {
            continue;
        }
        found.push(t);
        if let FoTerm::App(_, args) = tables.terms[t.index()] {
            terms.extend_from_slice(tables.list(args));
        }
    }
    let depth = term_depths(formulas, &found);
    found.sort_unstable_by_key(|t| (depth[t], *t));
    found
}

/// Returns the depth of each of `terms` and of every term inside them, a
/// constant being of depth 0.
fn term_depths(formulas: &Formulas, terms: &[FoTermId]) -> HashMap<FoTermId, u32> {
    let tables = formulas.first_order();
    let mut depth: HashMap<FoTermId, u32> = HashMap::default();
    let mut stack: Vec<(FoTermId, bool)> = terms.iter().map(|&t| (t, false)).collect();
    while let Some((t, ready)) = stack.pop() {
        if depth.contains_key(&t) {
            continue;
        }
        let FoTerm::App(_, args) = tables.terms[t.index()] else {
            depth.insert(t, 0);
            continue;
        };
        let list = tables.list(args);
        if !ready {
            stack.push((t, true));
            stack.extend(list.iter().map(|&u| (u, false)));
            continue;
        }
        let deepest = list.iter().map(|u| depth[u] + 1).max().unwrap_or(0);
        depth.insert(t, deepest);
    }
    depth
}

/// A formula on a branch of the derivation built.
#[derive(Clone, Debug)]
struct Item {
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
    const fn new(node: NodeId, left: bool) -> Self {
        Self {
            node,
            left,
            done: Vec::new(),
            inert: false,
        }
    }
}

/// A branch: its formulas and the Skolem terms it introduced.
#[derive(Clone, Debug, Default)]
struct Branch {
    /// The formulas, in the order they came.
    items: Vec<Item>,
    /// The Skolem terms of the `∀R` and `∃L` below it.
    introduced: Vec<FoTermId>,
    /// The Skolem terms a witness below it holds that the branch never
    /// introduced: constants here, which no `∀R` or `∃L` above may
    /// introduce.
    constants: Vec<FoTermId>,
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

/// Returns the branch without item `at` and with `added`.
fn replaced(branch: &Branch, at: usize, added: Vec<Item>) -> Branch {
    let mut premise = branch.clone();
    premise.items.remove(at);
    premise.items.extend(added);
    premise
}

/// A rule applied to a branch.
struct Step {
    /// The rule.
    rule: Rule,
    /// The principal formula.
    principal: (Side, usize),
    /// The term of a quantifier rule, a Skolem term for `∀R` and `∃L`.
    term: Option<FoTermId>,
    /// The premises.
    premises: Vec<Branch>,
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

/// The construction of the derivation, with Skolem terms where its
/// eigenvariables will be.
struct Build<'l> {
    /// The arena: the clausal form's and the instances'.
    formulas: Formulas,
    /// The clausal form's Skolem steps: the quantifier replaced, open, and
    /// its Skolem term over its free variables.
    skolems: Vec<(NodeId, FoTermId)>,
    /// The Skolem symbols.
    symbols: HashSet<Symbol>,
    /// The witnesses, the shallowest first.
    witnesses: Vec<FoTermId>,
    /// The Skolem term of each strong quantifier met, by its formula and
    /// side.
    keys: HashMap<(NodeId, bool), FoTermId>,
    /// Each instance made, by the binder's operand and the term.
    instances: HashMap<(NodeId, FoTermId), NodeId>,
    /// Every Skolem term a `∀R` or `∃L` introduced, in order.
    made: Vec<FoTermId>,
    /// The inferences so far, premises first.
    inferences: Vec<Inference>,
    /// The bytes they hold, estimated.
    bytes: u64,
    /// The bounds.
    limits: &'l Limits,
}

impl Build<'_> {
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
                    let at = results.len() - premises;
                    let premises = results.split_off(at);
                    let id = self.push(left, right, rule, principal, term, premises, stop)?;
                    results.push(id);
                }
            }
        }
        Ok(())
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
        if stop(Progress::new(Phase::ReadBack, 1, n + 1)) {
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::ReadBack,
            }));
        }
        let held =
            size_of::<Inference>() + (left.len() + right.len() + premises.len()) * size_of::<u32>();
        self.bytes = self.bytes.saturating_add(held as u64);
        if let Some(limit) = self.limits.memory_bytes
            && self.bytes > limit
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: limit,
                needed_bytes: None,
            }));
        }
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

    /// Closes the branch if it can, else applies the first rule its order
    /// gives.
    fn expand(
        &mut self,
        mut branch: Branch,
        frames: &mut Vec<Frame>,
        results: &mut Vec<InfId>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
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
        let (left, right) = sides(branch);
        if let Some(at) = right
            .iter()
            .position(|&k| self.formulas.node(k) == Node::True)
        {
            let id = self.push(
                left,
                right,
                Rule::TrueRight,
                Some((Side::Right, at)),
                None,
                Vec::new(),
                stop,
            )?;
            return Ok(Some(id));
        }
        if let Some(at) = left
            .iter()
            .position(|&k| self.formulas.node(k) == Node::False)
        {
            let id = self.push(
                left,
                right,
                Rule::FalseLeft,
                Some((Side::Left, at)),
                None,
                Vec::new(),
                stop,
            )?;
            return Ok(Some(id));
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

    /// Returns the first rule the order gives on the branch, if any: one
    /// premise and no term; `∀R` and `∃L`; `∀L` and `∃R` with a witness
    /// the branch has every Skolem term of; two premises, one closing at
    /// once first; a witness with a Skolem term of another branch, the
    /// least nested first.
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
        let closing = (0..branch.items.len()).find(|&at| self.closes_a_premise(branch, at));
        if let Some(at) = closing
            && let Some(step) = self.binary(branch, at)?
        {
            return Ok(Some(step));
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

    /// Returns whether a rule of two premises on item `at` closes one of
    /// them at once.
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
        let normal = normal::negation_normal(&mut self.formulas, moved, self.limits)?;
        let found: Vec<(FoTermId, Vec<Option<FoTermId>>)> = self
            .skolems
            .iter()
            .filter_map(|&(before, term)| {
                matched(&self.formulas, before, normal).map(|values| (term, values))
            })
            .collect();
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

    /// Returns the Skolem terms in `t`, `t` among them.
    fn skolem_terms(&self, t: FoTermId) -> Vec<FoTermId> {
        let tables = self.formulas.first_order();
        let (mut found, mut stack) = (Vec::new(), vec![t]);
        while let Some(u) = stack.pop() {
            if let FoTerm::App(f, args) = tables.terms[u.index()] {
                if self.symbols.contains(&f) && !found.contains(&u) {
                    found.push(u);
                }
                stack.extend_from_slice(tables.list(args));
            }
        }
        found
    }

    /// Returns how deep the Skolem terms in `t` nest: 0 for none, else one
    /// more than the deepest inside the deepest.
    fn nesting(&self, t: FoTermId) -> u32 {
        let tables = self.formulas.first_order();
        let mut most = 0;
        let mut stack = vec![(t, 0u32)];
        while let Some((u, above)) = stack.pop() {
            if let FoTerm::App(f, args) = tables.terms[u.index()] {
                let here = above + u32::from(self.symbols.contains(&f));
                most = most.max(here);
                stack.extend(tables.list(args).iter().map(|&v| (v, here)));
            }
        }
        most
    }

    /// Returns the derivation of `sequent` made: each Skolem term an
    /// introduced eigenvariable, numbered past the arena's, the least
    /// nested first.
    fn deskolemized(mut self, sequent: &Sequent) -> Result<Derivation, Error> {
        let base = self
            .formulas
            .first_order()
            .terms
            .iter()
            .filter_map(|t| match t {
                FoTerm::Eigen(e) => Some(e.get().saturating_add(1)),
                FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => None,
            })
            .max()
            .unwrap_or(0);
        let mut made = std::mem::take(&mut self.made);
        made.sort_by_key(|&s| self.nesting(s));
        let mut image: HashMap<FoTermId, FoTermId> = HashMap::default();
        for (k, &s) in made.iter().enumerate() {
            let e = Eigen::new(base.saturating_add(k as u32));
            image.insert(s, self.formulas.interner().term(FoTerm::Eigen(e)));
        }
        let mut cache = Replaced::default();
        let mut inferences = std::mem::take(&mut self.inferences);
        for inference in &mut inferences {
            for node in inference.left.iter_mut().chain(inference.right.iter_mut()) {
                *node = self.formulas.replaced(*node, &image, &mut cache)?;
            }
            if let Some(t) = inference.term {
                inference.term = Some(self.formulas.replaced_term(t, &image, &mut cache));
            }
        }
        Ok(Derivation::from_parts(
            Logic::Classical,
            self.formulas,
            (sequent.left().to_vec(), sequent.right().to_vec()),
            inferences,
        ))
    }
}

/// Returns the values of the variables free in `pattern`, an open formula,
/// under which it is the closed formula `target`, by their outside index;
/// `None` where it is no instance.
fn matched(formulas: &Formulas, pattern: NodeId, target: NodeId) -> Option<Vec<Option<FoTermId>>> {
    let tables = formulas.first_order();
    let mut values: Vec<Option<FoTermId>> = Vec::new();
    let mut nodes = vec![(pattern, target, 0u32)];
    let mut terms: Vec<(FoTermId, FoTermId, u32)> = Vec::new();
    while let Some((p, t, depth)) = nodes.pop() {
        match (formulas.node(p), formulas.node(t)) {
            (Node::Atom(a), Node::Atom(b)) => {
                match (tables.atoms.get(a as usize), tables.atoms.get(b as usize)) {
                    (Some(&(f, x)), Some(&(g, y))) => {
                        let (x, y) = (tables.list(x), tables.list(y));
                        if f != g || x.len() != y.len() {
                            return None;
                        }
                        terms.extend(x.iter().zip(y).map(|(&u, &v)| (u, v, depth)));
                    }
                    (None, None) if a == b => {}
                    _ => return None,
                }
            }
            (Node::True, Node::True) | (Node::False, Node::False) => {}
            (Node::Not(a), Node::Not(b)) => nodes.push((a, b, depth)),
            (Node::And(a, b), Node::And(c, d))
            | (Node::Or(a, b), Node::Or(c, d))
            | (Node::Implies(a, b), Node::Implies(c, d))
            | (Node::Iff(a, b), Node::Iff(c, d)) => {
                nodes.push((a, c, depth));
                nodes.push((b, d, depth));
            }
            (Node::Forall(a), Node::Forall(b)) | (Node::Exists(a), Node::Exists(b)) => {
                nodes.push((a, b, depth + 1));
            }
            _ => return None,
        }
    }
    while let Some((u, v, depth)) = terms.pop() {
        match (tables.terms[u.index()], tables.terms[v.index()]) {
            (FoTerm::Bound(i), _) if i >= depth => {
                if !formulas.is_closed(v) {
                    return None;
                }
                let k = (i - depth) as usize;
                if values.len() <= k {
                    values.resize(k + 1, None);
                }
                match values[k] {
                    Some(w) if w != v => return None,
                    Some(_) => {}
                    None => values[k] = Some(v),
                }
            }
            (FoTerm::App(f, x), FoTerm::App(g, y)) => {
                let (x, y) = (tables.list(x), tables.list(y));
                if f != g || x.len() != y.len() {
                    return None;
                }
                terms.extend(x.iter().zip(y).map(|(&a, &b)| (a, b, depth)));
            }
            _ if u == v => {}
            _ => return None,
        }
    }
    Some(values)
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;

    /// Returns what Herbrand's theorem finds for `text` by default.
    fn found(text: &str) -> Herbrand {
        let sequent: Sequent = text.parse().unwrap();
        herbrand_default(&sequent, &Options::default()).unwrap()
    }

    /// A valid sequent gets instances the solver refutes and a checked
    /// derivation of LK of the sequent as written, its Skolem terms
    /// eigenvariables again.
    #[test]
    fn a_valid_sequent_gets_its_instances_and_a_derivation() {
        for text in [
            "forall x. p(x) |- p(f(a))",
            "|- exists x. (p(x) -> forall y. p(y))",
            "forall x. exists y. r(x, y) |- forall x. exists y. exists z. (r(x, y) /\\ r(x, z))",
            "exists x. forall y. r(x, y) |- forall y. exists x. r(x, y)",
            "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))",
            "|- (forall x. p(x)) <-> ~(exists x. ~p(x))",
            "p |- p \\/ q",
            // An instance of the first hypothesis is the second: the kept
            // instances' term decides which Skolem term it stands for.
            "forall y. exists x. p(x, y), exists x. p(x, a) |- exists x. ~~p(x, a)",
        ] {
            let found = found(text);
            let Verdict::Valid(Evidence::Derivation(derivation)) = &found.verdict else {
                panic!("{text}: {:?}", found.verdict);
            };
            let (left, right) = derivation.sequent();
            assert_eq!(left, found.sequent.left(), "{text}");
            assert_eq!(right, found.sequent.right(), "{text}");
            derivation.check(&Limits::default(), |_| false).unwrap();
            assert!(
                found
                    .refutation
                    .as_ref()
                    .is_some_and(|r| matches!(r.verdict, Verdict::Valid(_))),
                "{text}"
            );
            assert!(!found.instances.is_empty(), "{text}");
        }
    }

    /// Two steps of the rule need the terms up to depth 1, whose instances
    /// are six: the certificate keeps the four the solver needs, none of
    /// `q`.
    #[test]
    fn the_certificate_keeps_the_instances_it_needs() {
        let found = found("forall x. (p(x) -> p(f(x))), forall x. q(x), p(a) |- p(f(f(a)))");
        assert_eq!(found.depth, 1);
        assert_eq!(found.statistics.instances, 6);
        assert_eq!(found.instances.len(), 4);
        assert_eq!(found.statistics.kept, 4);
    }

    /// A sequent that is not valid ends unknown: at the depth where
    /// function symbols make the universe grow, and as incomplete where
    /// its universe is finite and every instance was tried.
    #[test]
    fn a_sequent_not_valid_ends_unknown() {
        let deep = found("forall x. (p(x) -> p(f(x))) |- p(a)");
        assert!(matches!(
            deep.verdict,
            Verdict::Unknown(Reason::WitnessDepth {
                depth: DEFAULT_DEPTH
            })
        ));
        let finite = found("exists x. p(x) |- forall y. p(y)");
        assert!(matches!(
            finite.verdict,
            Verdict::Unknown(Reason::Incomplete)
        ));
    }

    /// Two different quantifiers of one variable get two eigenvariables:
    /// the witness of one is no witness of the other.
    #[test]
    fn different_quantifiers_get_different_eigenvariables() {
        let valid =
            found("(exists x. p(x)) /\\ (exists x. q(x)) |- exists x. p(x) /\\ exists y. q(y)");
        assert!(matches!(
            valid.verdict,
            Verdict::Valid(Evidence::Derivation(_))
        ));
        let not = found("(exists x. p(x)) /\\ (exists x. q(x)) |- exists x. (p(x) /\\ q(x))");
        assert!(matches!(not.verdict, Verdict::Unknown(_)));
    }

    /// A propositional sequent keeps an arena without first-order tables,
    /// so its ground sequent and derivation read back as written.
    #[test]
    fn a_propositional_sequent_stays_propositional() {
        let found = found("p |- p \\/ q");
        assert!(!found.ground.formulas().is_first_order());
        let Verdict::Valid(Evidence::Derivation(derivation)) = &found.verdict else {
            panic!("{:?}", found.verdict);
        };
        assert!(!derivation.formulas().is_first_order());
    }

    /// A bound that ends the solver, or the stop while the instances are
    /// made, ends the search unknown for that reason, never as instances
    /// with a model.
    #[test]
    fn a_bound_or_the_stop_ends_the_search_for_its_reason() {
        let sequent: Sequent = "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))"
            .parse()
            .unwrap();
        let small = Limits::default().with_memory_bytes(Some(2048));
        let options = Options::default().with_depth(None);
        let found = herbrand(&sequent, &options, &small, |_| false).unwrap();
        assert!(
            matches!(found.verdict, Verdict::Unknown(Reason::MemoryLimit { .. })),
            "{:?}",
            found.verdict
        );
        let deep: Sequent = "forall x y z. (q(x, y, z) -> q(g(x, y), z, x)) |- q(a, a, a) /\\ r"
            .parse()
            .unwrap();
        let found = herbrand(&deep, &options, &Limits::default(), |p| {
            p.phase == Phase::Search && p.done >= 4096
        })
        .unwrap();
        assert!(
            matches!(found.verdict, Verdict::Unknown(Reason::Stopped)),
            "{:?}",
            found.verdict
        );
    }

    /// Generated first-order sequents: a valid verdict always has its
    /// derivation checked and is never refuted classically, and most
    /// sequents the default prover finds valid are found valid here.
    #[test]
    fn generated_sequents_agree_with_the_default_prover() {
        use crate::linear::search::generate::Rng;
        use crate::ordinary::normal::first_order_tests::formula;
        let mut rng = Rng::new(77);
        let limits = Limits::default().with_work(Some(500_000));
        let options = Options::default().with_depth(Some(2));
        let (mut sequents, mut valid, mut found) = (0, 0, 0);
        while sequents < 300 {
            let (left, right) = (
                formula(&mut rng, 3, &mut Vec::new()),
                formula(&mut rng, 3, &mut Vec::new()),
            );
            let Ok(sequent) = format!("{left} |- {right}").parse::<Sequent>() else {
                continue;
            };
            if !sequent.formulas().is_first_order() {
                continue;
            }
            sequents += 1;
            let classical = crate::ordinary::decide_within(
                &sequent,
                &crate::ordinary::Options::default().with_logic(Logic::Classical),
                &crate::linear::search::Options::default(),
                &limits,
                |_| false,
            );
            let classical = classical.map(|o| o.verdict);
            let herbrand = match herbrand(&sequent, &options, &limits, |_| false) {
                Ok(herbrand) => herbrand,
                Err(Error::Refused(_)) => continue,
                Err(error) => panic!("{sequent}: {error}"),
            };
            if matches!(classical, Ok(Verdict::Valid(_))) {
                valid += 1;
            }
            if let Verdict::Valid(evidence) = &herbrand.verdict {
                assert!(!matches!(classical, Ok(Verdict::NotValid)), "{sequent}");
                if let Evidence::Derivation(derivation) = evidence {
                    derivation.check(&Limits::default(), |_| false).unwrap();
                    found += 1;
                }
            }
        }
        assert!(found * 4 >= valid * 3, "{found} of {valid} valid found");
    }
}
