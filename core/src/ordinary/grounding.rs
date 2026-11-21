// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A decision procedure for the first-order sequents of the
//! Bernays–Schönfinkel class (∃\*∀\* after Skolemization, no function
//! symbols) and of the monadic class (every predicate unary or nullary,
//! no function symbols), in classical logic.
//!
//! A sequent `Γ ⊢ Δ` is valid when `¬(⋀Γ → ⋁Δ)` has no model. The
//! procedure reads `⋀Γ → ⋁Δ` in negation normal form, its variables
//! named, and moves every quantifier as far in as it goes (miniscoping:
//! `∀x. (A ∧ B)` to `∀x. A ∧ ∀x. B`, `∀x. (A ∨ B)` to `A ∨ ∀x. B` where
//! `x` is not free in `A`, and the duals). The sequent is in the class
//! when then no `∀` lies in the scope of an `∃` whose variable is free in
//! it and no function symbol is applied to arguments. A `∀` of the formula
//! to prove is then a Skolem constant, and the Herbrand universe is the
//! finite set of the constants (one where there is none). Each `∃`
//! becomes the disjunction of its instances over that set, and the ground
//! formula, propositional, goes to the SAT solver
//! (Löwenheim 1915, Behmann 1922 for the monadic class; Bernays and
//! Schönfinkel 1928; Herbrand's theorem for the grounding).
//!
//! - **Valid** stands with the solver's clausal proof of the ground
//!   formula, checked by the crate's checker, as the propositional
//!   classical path's does.
//! - **Not valid** comes with the solver's falsifying assignment read as
//!   a finite [`Structure`] over the constants, which
//!   [`Structure::refutes`] checks against the sequent as written, not
//!   against the formula this module made of it: a structure that does
//!   not refute it is [`Error::Countermodel`], a defect. A classical
//!   countermodel is a Kripke model of one world, so the verdict holds in
//!   intuitionistic and minimal logic too.
//!
//! Every walk keeps a stack of its own, and the ground formula is held to
//! `limits.occurrences`.

use super::sat;
use super::structure::{FiniteCountermodel, Structure};
use super::{Formulas, Logic, Node, NodeId, Sequent, Verdict};
use crate::Error;
use crate::first_order::FoTerm;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::search::Reason;

/// No index.
const NONE: u32 = u32::MAX;

/// What deciding a sequent by grounding cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct Statistics {
    /// The elements of the domain: the sequent's constants and the Skolem
    /// constants, or one where there is none.
    pub domain: u64,
    /// The distinct ground atoms.
    pub atoms: u64,
    /// The nodes of the ground formula.
    pub nodes: u64,
    /// What the SAT solver's search cost.
    pub sat: sat::Statistics,
}

/// What deciding a sequent by grounding found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict: valid with a checked clausal proof, not valid, or
    /// unknown (in intuitionistic and minimal logic, where the sequent is
    /// valid classically, and at a bound).
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// The finite structure that refutes a sequent that is not valid,
    /// checked by [`FiniteCountermodel::check`]; `None` where it is too
    /// large to check within the limits, the verdict standing.
    pub structure: Option<FiniteCountermodel>,
    /// What the search cost.
    pub statistics: Statistics,
}

/// A term of the formula in negation normal form.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Term {
    /// A variable, by its binder's number.
    Var(u32),
    /// A constant, by its symbol's index in the sequent's tables.
    Const(u32),
}

/// A node of the formula in negation normal form, its variables named.
#[derive(Clone, Debug)]
enum G {
    /// A literal: whether it is positive, its predicate (the symbol's index
    /// in a first-order arena, the atom's in a propositional one) and its
    /// arguments.
    Lit(bool, u32, Vec<Term>),
    /// `⊤`.
    True,
    /// `⊥`.
    False,
    /// A conjunction of the nodes given, none a conjunction.
    And(Vec<u32>),
    /// A disjunction of the nodes given, none a disjunction.
    Or(Vec<u32>),
    /// `∀` of the variable over the node.
    Forall(u32, u32),
    /// `∃` of the variable over the node.
    Exists(u32, u32),
}

/// The formula in negation normal form: its nodes, each with its free
/// variables, sorted.
#[derive(Debug, Default)]
struct Nnf {
    /// The nodes.
    nodes: Vec<G>,
    /// Each node's free variables, sorted.
    free: Vec<Vec<u32>>,
    /// How many variables were numbered.
    vars: u32,
}

impl Nnf {
    /// Adds a node, computing its free variables, and returns its index.
    fn push(&mut self, node: G) -> u32 {
        let free = match &node {
            G::Lit(_, _, args) => {
                let mut v: Vec<u32> = args
                    .iter()
                    .filter_map(|t| match *t {
                        Term::Var(x) => Some(x),
                        Term::Const(_) => None,
                    })
                    .collect();
                v.sort_unstable();
                v.dedup();
                v
            }
            G::True | G::False => Vec::new(),
            G::And(kids) | G::Or(kids) => {
                let mut v: Vec<u32> = kids
                    .iter()
                    .flat_map(|&k| self.free[k as usize].iter().copied())
                    .collect();
                v.sort_unstable();
                v.dedup();
                v
            }
            G::Forall(x, body) | G::Exists(x, body) => self.free[*body as usize]
                .iter()
                .copied()
                .filter(|v| v != x)
                .collect(),
        };
        self.nodes.push(node);
        self.free.push(free);
        (self.nodes.len() - 1) as u32
    }

    /// Returns whether `x` is free in node `n`.
    fn has(&self, n: u32, x: u32) -> bool {
        self.free[n as usize].binary_search(&x).is_ok()
    }

    /// Returns the conjunction (`and`) or the disjunction of the nodes,
    /// flattened: a node of the same connective gives its operands, and one
    /// operand alone is itself.
    fn junction(&mut self, and: bool, kids: Vec<u32>) -> u32 {
        let mut flat = Vec::with_capacity(kids.len());
        for k in kids {
            match &self.nodes[k as usize] {
                G::And(inner) if and => flat.extend_from_slice(inner),
                G::Or(inner) if !and => flat.extend_from_slice(inner),
                G::And(_)
                | G::Or(_)
                | G::Lit(..)
                | G::True
                | G::False
                | G::Forall(..)
                | G::Exists(..) => flat.push(k),
            }
        }
        match flat.len() {
            0 => self.push(if and { G::True } else { G::False }),
            1 => flat[0],
            _ => self.push(if and { G::And(flat) } else { G::Or(flat) }),
        }
    }

    /// Returns `∀x. body` (`forall`) or `∃x. body` with the quantifier moved
    /// as far in as miniscoping takes it. It descends at most three levels:
    /// the operands of a flattened junction are no junction of its kind.
    fn quantify(&mut self, forall: bool, x: u32, body: u32) -> u32 {
        if !self.has(body, x) {
            return body;
        }
        let node = self.nodes[body as usize].clone();
        match (forall, node) {
            // ∀ distributes over ∧, ∃ over ∨.
            (true, G::And(kids)) => {
                let kids = kids
                    .into_iter()
                    .map(|k| self.quantify(true, x, k))
                    .collect();
                self.junction(true, kids)
            }
            (false, G::Or(kids)) => {
                let kids = kids
                    .into_iter()
                    .map(|k| self.quantify(false, x, k))
                    .collect();
                self.junction(false, kids)
            }
            // ∀ over ∨ and ∃ over ∧ leave out the operands without `x`.
            (true, G::Or(kids)) | (false, G::And(kids)) => {
                let (with, without): (Vec<u32>, Vec<u32>) =
                    kids.into_iter().partition(|&k| self.has(k, x));
                if without.is_empty() {
                    return self.push(if forall {
                        G::Forall(x, body)
                    } else {
                        G::Exists(x, body)
                    });
                }
                let and = !forall;
                let inner = self.junction(and, with);
                let quantified = if forall {
                    self.push(G::Forall(x, inner))
                } else {
                    self.push(G::Exists(x, inner))
                };
                let mut all = without;
                all.push(quantified);
                self.junction(and, all)
            }
            (_, G::Lit(..) | G::True | G::False | G::Forall(..) | G::Exists(..)) => {
                self.push(if forall {
                    G::Forall(x, body)
                } else {
                    G::Exists(x, body)
                })
            }
        }
    }
}

/// The ground problem: the formula to prove, propositional.
struct Ground {
    /// The propositional sequent `⊢ F`.
    sequent: Sequent,
    /// Each ground atom: the predicate's index and where its tuple of
    /// elements starts in `args` and how long it is, by the atom's index in
    /// the ground sequent.
    atoms: Vec<(u32, u32, u32)>,
    /// The atoms' tuples, one after another: one allocation for all, so
    /// that a large grounding is freed at once when the stop ends it.
    args: Vec<u32>,
    /// The constant each element is, [`NONE`] for a Skolem constant or the
    /// element added to an empty domain.
    elements: Vec<u32>,
}

/// Decides `sequent` by grounding, if it lies in the Bernays–Schönfinkel
/// or the monadic class, within `limits`, asking `stop` as it goes;
/// `Ok(None)` where it does not.
///
/// In classical logic the verdict is valid with a checked clausal proof or
/// not valid with a checked [`Structure`]. In intuitionistic and minimal
/// logic it is not valid with the same structure, a Kripke model of one
/// world, or unknown ([`Reason::Incomplete`]) where the sequent is valid
/// classically. A structure too large to check within the limits is
/// dropped and the verdict kept, as a Kripke countermodel too large to
/// check is: the ground sequent's model was checked, and the verdict rests
/// on the grounding. A stop, during the set-up or the check, is unknown
/// ([`Reason::Stopped`]).
///
/// # Errors
///
/// [`Error::Succedents`] for more than one formula right of `⊢` outside
/// classical logic; [`Refusal::Occurrences`] for a ground formula past
/// `limits.occurrences`; the SAT solver's refusals and errors, as
/// `sat::decide`'s; and [`Error::Countermodel`] where the structure does
/// not refute the sequent, a defect.
pub fn decide(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Option<Outcome>, Error> {
    run(sequent, logic, limits, &mut stop)
}

/// Returns whether `sequent` lies in the classes this module decides.
///
/// # Errors
///
/// [`Refusal::Occurrences`] for a sequent whose normal form passes
/// `limits.occurrences`.
pub fn applies(sequent: &Sequent, limits: &Limits) -> Result<bool, Error> {
    Ok(prepare(sequent, limits, &mut |_| false)?.is_some())
}

/// [`decide`] with the stop as a trait object.
fn run(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Option<Outcome>, Error> {
    if logic != Logic::Classical && sequent.right().len() > 1 {
        return Err(Error::Succedents {
            count: sequent.right().len(),
        });
    }
    let stopped = |statistics| {
        Ok(Some(Outcome {
            verdict: Verdict::Unknown(Reason::Stopped),
            logic,
            structure: None,
            statistics,
        }))
    };
    let (nnf, root) = match prepare(sequent, limits, &mut *stop) {
        Ok(Some(prepared)) => prepared,
        Ok(None) => return Ok(None),
        Err(Error::Refused(Refusal::Stopped { .. })) => return stopped(Statistics::default()),
        Err(error) => return Err(error),
    };
    let ground = match ground(&nnf, root, limits, &mut *stop) {
        Ok(ground) => ground,
        Err(Error::Refused(Refusal::Stopped { .. })) => return stopped(Statistics::default()),
        Err(error) => return Err(error),
    };
    let mut statistics = Statistics {
        domain: ground.elements.len() as u64,
        atoms: ground.atoms.len() as u64,
        nodes: ground.sequent.formulas().len() as u64,
        sat: sat::Statistics::default(),
    };
    let decided = sat::decide(&ground.sequent, Logic::Classical, limits, &mut *stop)?;
    statistics.sat = decided.statistics;
    let (verdict, structure) = match decided.verdict {
        Verdict::Valid(evidence) => {
            let verdict = if logic == Logic::Classical {
                Verdict::Valid(evidence)
            } else {
                // Valid classically says nothing of the other logics.
                Verdict::Unknown(Reason::Incomplete)
            };
            (verdict, None)
        }
        Verdict::NotValid => match decided.model {
            Some(countermodel) => {
                let structure = structure_of(
                    sequent,
                    &ground,
                    &countermodel.model.worlds[countermodel.model.world as usize].atoms,
                );
                let countermodel = FiniteCountermodel::new(sequent.clone(), structure);
                match countermodel.check(logic, limits, &mut *stop) {
                    Ok(()) => (Verdict::NotValid, Some(countermodel)),
                    // The stop during the check is the stop, as for a
                    // Kripke countermodel.
                    Err(Error::Refused(Refusal::Stopped { .. })) => {
                        (Verdict::Unknown(Reason::Stopped), None)
                    }
                    // A structure too large to check is dropped, never the
                    // verdict: the ground sequent's model was checked, and
                    // the verdict rests on grounding as a verdict without
                    // its model rests on the search.
                    Err(Error::Refused(_)) => (Verdict::NotValid, None),
                    Err(Error::Structure { reason }) => {
                        return Err(Error::Countermodel {
                            reason: format!(
                                "the finite structure read off the ground sequent's model is wrong: {reason}"
                            ),
                        });
                    }
                    Err(error) => return Err(error),
                }
            }
            // The solver's model was too large to check; the verdict
            // stands without it, as the solver's own does.
            None => (Verdict::NotValid, None),
        },
        Verdict::Unknown(reason) => (Verdict::Unknown(reason), None),
    };
    Ok(Some(Outcome {
        verdict,
        logic,
        structure,
        statistics,
    }))
}

/// Counts a step of the set-up and asks the stop every [`POLL`] steps.
fn poll(steps: &mut u64, stop: &mut dyn FnMut(Progress) -> bool) -> Result<(), Error> {
    *steps += 1;
    if steps.is_multiple_of(POLL) && stop(Progress::new(Phase::Search, POLL, *steps)) {
        return Err(Refusal::Stopped {
            phase: Phase::Search,
        }
        .into());
    }
    Ok(())
}

/// The bytes a node of the ground formula's arena holds, with its share of
/// the arena's index.
const NODE_BYTES: u64 = 32;

/// The bytes a ground atom holds beside 4 bytes an argument: its entries in
/// the grounding's tables and the arena's, and its name.
const ATOM_BYTES: u64 = 128;

/// How many steps of the set-up pass between two asks of the stop.
const POLL: u64 = 4_096;

/// Returns the sequent's formula `⋀Γ → ⋁Δ` in negation normal form,
/// miniscoped, and its root, where the sequent lies in the classes this
/// module decides: no function symbol with arguments, no term that no
/// quantifier binds and the sequent does not name, and no `∀` in the scope
/// of an `∃` whose variable is free in it.
fn prepare(
    sequent: &Sequent,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Option<(Nnf, u32)>, Error> {
    let formulas = sequent.formulas();
    let tables = formulas.first_order();
    let most = limits.occurrences.unwrap_or(u64::MAX);
    let mut nnf = Nnf::default();
    // The scopes: a binder's variable and the scope around it.
    let mut scopes: Vec<(u32, u32)> = Vec::new();
    // Each task: the formula, whether it is negated, its scope, and the
    // node its result goes to: (parent, operand), or the root's slot.
    enum Task {
        /// Translate the formula.
        Enter(NodeId, bool, u32, (u32, u32)),
        /// Make the node from its operands' results.
        Make(u32),
    }
    // Pending nodes: the made operands of each.
    struct Pending {
        /// What the node will be: a junction (`and`), or a quantifier
        /// (`forall`) with its variable.
        kind: (u8, bool, u32),
        /// The operands' results.
        kids: Vec<u32>,
        /// Where its result goes.
        to: (u32, u32),
    }
    let mut pending: Vec<Pending> = Vec::new();
    // The root is a disjunction of the hypotheses negated and the goals.
    let roots: Vec<(NodeId, bool)> = sequent
        .left()
        .iter()
        .map(|&f| (f, true))
        .chain(sequent.right().iter().map(|&f| (f, false)))
        .collect();
    pending.push(Pending {
        kind: (0, false, 0),
        kids: vec![NONE; roots.len()],
        to: (NONE, 0),
    });
    let mut stack: Vec<Task> = vec![Task::Make(0)];
    for (k, &(f, negated)) in roots.iter().enumerate().rev() {
        stack.push(Task::Enter(f, negated, NONE, (0, k as u32)));
    }
    let mut root = NONE;
    let put = |pending: &mut Vec<Pending>, root: &mut u32, to: (u32, u32), made: u32| {
        if to.0 == NONE {
            *root = made;
        } else {
            pending[to.0 as usize].kids[to.1 as usize] = made;
        }
    };
    let mut steps: u64 = 0;
    while let Some(task) = stack.pop() {
        poll(&mut steps, stop)?;
        if nnf.nodes.len() as u64 > most {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: most.saturating_add(1),
                limit: most,
            }));
        }
        match task {
            Task::Make(p) => {
                let Pending { kind, kids, to } = std::mem::replace(
                    &mut pending[p as usize],
                    Pending {
                        kind: (0, false, 0),
                        kids: Vec::new(),
                        to: (NONE, 0),
                    },
                );
                let made = match kind {
                    (0, and, _) => nnf.junction(and, kids),
                    (_, forall, x) => {
                        let body = kids[0];
                        if forall {
                            nnf.push(G::Forall(x, body))
                        } else {
                            nnf.push(G::Exists(x, body))
                        }
                    }
                };
                put(&mut pending, &mut root, to, made);
            }
            Task::Enter(f, negated, scope, to) => {
                let node = formulas.node(f);
                let children: Vec<(NodeId, bool)>;
                let kind: (u8, bool, u32);
                let mut inner_scope = scope;
                match node {
                    Node::Atom(a) => {
                        let Some(lit) = literal(a, scope, &scopes, tables) else {
                            return Ok(None);
                        };
                        let made = nnf.push(G::Lit(!negated, lit.0, lit.1));
                        put(&mut pending, &mut root, to, made);
                        continue;
                    }
                    Node::True | Node::False => {
                        let truth = matches!(node, Node::True) != negated;
                        let made = nnf.push(if truth { G::True } else { G::False });
                        put(&mut pending, &mut root, to, made);
                        continue;
                    }
                    Node::Not(a) => {
                        stack.push(Task::Enter(a, !negated, scope, to));
                        continue;
                    }
                    Node::And(a, b) => {
                        kind = (0, !negated, 0);
                        children = vec![(a, negated), (b, negated)];
                    }
                    Node::Or(a, b) => {
                        kind = (0, negated, 0);
                        children = vec![(a, negated), (b, negated)];
                    }
                    Node::Implies(a, b) => {
                        kind = (0, negated, 0);
                        children = vec![(a, !negated), (b, negated)];
                    }
                    Node::Iff(a, b) => {
                        // (A ↔ B) is (¬A ∨ B) ∧ (A ∨ ¬B); negated, (A ∧ ¬B) ∨ (¬A ∧ B).
                        let p = pending.len() as u32;
                        pending.push(Pending {
                            kind: (0, !negated, 0),
                            kids: vec![NONE; 2],
                            to,
                        });
                        stack.push(Task::Make(p));
                        for (k, (x, y)) in [((a, true), (b, false)), ((a, false), (b, true))]
                            .into_iter()
                            .enumerate()
                            .rev()
                        {
                            let q = pending.len() as u32;
                            pending.push(Pending {
                                kind: (0, negated, 0),
                                kids: vec![NONE; 2],
                                to: (p, k as u32),
                            });
                            stack.push(Task::Make(q));
                            stack.push(Task::Enter(y.0, y.1 != negated, scope, (q, 1)));
                            stack.push(Task::Enter(x.0, x.1 != negated, scope, (q, 0)));
                        }
                        continue;
                    }
                    Node::Forall(body) | Node::Exists(body) => {
                        // Negation turns ∀ into ∃.
                        let forall = matches!(node, Node::Forall(_)) != negated;
                        let x = nnf.vars;
                        nnf.vars += 1;
                        scopes.push((x, scope));
                        inner_scope = (scopes.len() - 1) as u32;
                        kind = (1, forall, x);
                        children = vec![(body, negated)];
                    }
                }
                let p = pending.len() as u32;
                pending.push(Pending {
                    kind,
                    kids: vec![NONE; children.len()],
                    to,
                });
                stack.push(Task::Make(p));
                for (k, &(c, n)) in children.iter().enumerate().rev() {
                    stack.push(Task::Enter(c, n, inner_scope, (p, k as u32)));
                }
            }
        }
    }
    let root = miniscope(&mut nnf, root);
    Ok(in_class(&nnf, root).then_some((nnf, root)))
}

/// Returns the literal of atom `a` under the scope `scope`: its predicate
/// and its arguments as terms, or `None` where a term is no constant and
/// no bound variable.
fn literal(
    a: u32,
    scope: u32,
    scopes: &[(u32, u32)],
    tables: &crate::first_order::FirstOrder<NodeId>,
) -> Option<(u32, Vec<Term>)> {
    let Some(&(symbol, args)) = tables.atoms.get(a as usize) else {
        return Some((a, Vec::new()));
    };
    let mut terms = Vec::new();
    for &t in tables.list(args) {
        let term = match tables.terms[t.index()] {
            FoTerm::Bound(index) => {
                let mut s = scope;
                for _ in 0..index {
                    s = scopes[s as usize].1;
                }
                Term::Var(scopes[s as usize].0)
            }
            FoTerm::App(f, list) if tables.list(list).is_empty() => Term::Const(f.get()),
            FoTerm::App(..) | FoTerm::Eigen(_) | FoTerm::Meta(_) => return None,
        };
        terms.push(term);
    }
    Some((symbol.get(), terms))
}

/// Returns the root of the formula miniscoped: every node made again
/// bottom-up, each quantifier moved in as far as it goes.
fn miniscope(nnf: &mut Nnf, root: u32) -> u32 {
    // Postorder over the nodes reachable from the root; each made once.
    let mut made: HashMap<u32, u32> = HashMap::default();
    let mut stack = vec![(root, false)];
    while let Some((n, done)) = stack.pop() {
        if made.contains_key(&n) {
            continue;
        }
        let node = nnf.nodes[n as usize].clone();
        let kids: Vec<u32> = match &node {
            G::And(k) | G::Or(k) => k.clone(),
            G::Forall(_, b) | G::Exists(_, b) => vec![*b],
            G::Lit(..) | G::True | G::False => Vec::new(),
        };
        if !done && kids.iter().any(|k| !made.contains_key(k)) {
            stack.push((n, true));
            stack.extend(kids.iter().map(|&k| (k, false)));
            continue;
        }
        let result = match node {
            G::And(k) => {
                let k = k.iter().map(|c| made[c]).collect();
                nnf.junction(true, k)
            }
            G::Or(k) => {
                let k = k.iter().map(|c| made[c]).collect();
                nnf.junction(false, k)
            }
            G::Forall(x, b) => nnf.quantify(true, x, made[&b]),
            G::Exists(x, b) => nnf.quantify(false, x, made[&b]),
            G::Lit(..) | G::True | G::False => n,
        };
        made.insert(n, result);
    }
    made[&root]
}

/// Returns whether no `∀` reachable from `root` lies in the scope of an
/// `∃` whose variable is free in it: then every Skolem term is a constant.
fn in_class(nnf: &Nnf, root: u32) -> bool {
    // Each entry: a node and the ∃-variables around it, as a list in
    // `outer`, NONE for none.
    let mut outer: Vec<(u32, u32)> = Vec::new();
    let mut stack = vec![(root, NONE)];
    while let Some((n, around)) = stack.pop() {
        match &nnf.nodes[n as usize] {
            G::Lit(..) | G::True | G::False => {}
            G::And(k) | G::Or(k) => stack.extend(k.iter().map(|&c| (c, around))),
            G::Exists(x, b) => {
                outer.push((*x, around));
                stack.push((*b, (outer.len() - 1) as u32));
            }
            G::Forall(_, b) => {
                let mut a = around;
                while a != NONE {
                    if nnf.has(n, outer[a as usize].0) {
                        return false;
                    }
                    a = outer[a as usize].1;
                }
                stack.push((*b, around));
            }
        }
    }
    true
}

/// Grounds the miniscoped formula over the constants: each `∀` a Skolem
/// constant, each `∃` the disjunction of its instances, into the
/// propositional sequent `⊢ F` within `limits.occurrences`.
fn ground(
    nnf: &Nnf,
    root: u32,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Ground, Error> {
    let most = limits.occurrences.unwrap_or(u64::MAX);
    // The domain: the constants the formula names, then one Skolem
    // constant per ∀ reachable, or one element where there is none.
    let mut elements: Vec<u32> = Vec::new();
    let mut element_of: HashMap<u32, u32> = HashMap::default();
    let mut skolem_of: HashMap<u32, u32> = HashMap::default();
    let mut stack = vec![root];
    let mut seen = vec![false; nnf.nodes.len()];
    while let Some(n) = stack.pop() {
        if std::mem::replace(&mut seen[n as usize], true) {
            continue;
        }
        match &nnf.nodes[n as usize] {
            G::Lit(_, _, args) => {
                for t in args {
                    if let Term::Const(c) = *t {
                        element_of.entry(c).or_insert_with(|| {
                            elements.push(c);
                            (elements.len() - 1) as u32
                        });
                    }
                }
            }
            G::True | G::False => {}
            G::And(k) | G::Or(k) => stack.extend_from_slice(k),
            G::Forall(x, b) => {
                skolem_of.insert(*x, NONE);
                stack.push(*b);
            }
            G::Exists(_, b) => stack.push(*b),
        }
    }
    let mut skolems: Vec<u32> = skolem_of.keys().copied().collect();
    skolems.sort_unstable();
    for x in skolems {
        elements.push(NONE);
        skolem_of.insert(x, (elements.len() - 1) as u32);
    }
    if elements.is_empty() {
        elements.push(NONE);
    }
    let size = elements.len() as u32;
    let mut formulas = Formulas::default();
    let mut atoms: Vec<(u32, u32, u32)> = Vec::new();
    let mut args_of: Vec<u32> = Vec::new();
    // The atoms by their key's hash: the first atom of each hash, and for
    // each atom the next one of the same hash (`NONE` at the end) and its
    // node, so that no atom holds an allocation of its own.
    let mut first: HashMap<u64, u32> = HashMap::default();
    let mut next: Vec<u32> = Vec::new();
    let mut ids: Vec<NodeId> = Vec::new();
    let mut tuple: Vec<u32> = Vec::new();
    let hasher = crate::hash::BuildHasher::default();
    let index_of = |count: usize| {
        u32::try_from(count).map_err(|_| {
            Error::Refused(Refusal::Index {
                what: crate::limits::Space::Atom,
                count: count as u64,
                most: u64::from(u32::MAX),
            })
        })
    };
    // Each binding: a variable, its element and the bindings before it.
    let mut bindings: Vec<(u32, u32, u32)> = Vec::new();
    let lookup = |bindings: &[(u32, u32, u32)], mut at: u32, x: u32| -> u32 {
        while at != NONE {
            let (y, e, before) = bindings[at as usize];
            if y == x {
                return e;
            }
            at = before;
        }
        unreachable!("every variable is bound in a closed formula")
    };
    enum Task {
        /// Ground the node under the bindings.
        Enter(u32, u32),
        /// Join the last `count` results by `∧` (`and`) or `∨`.
        Join(bool, usize),
    }
    let mut results: Vec<NodeId> = Vec::new();
    let mut tasks = vec![Task::Enter(root, NONE)];
    let mut steps: u64 = 0;
    // The bytes the atoms' keys and names hold, which grow with their
    // arguments.
    let mut atom_bytes: u64 = 0;
    let memory = limits.memory_bytes.unwrap_or(u64::MAX);
    while let Some(task) = tasks.pop() {
        poll(&mut steps, stop)?;
        let held = (formulas.len() as u64)
            .saturating_mul(NODE_BYTES)
            .saturating_add(atom_bytes)
            .saturating_add((tasks.capacity() as u64).saturating_mul(16))
            .saturating_add((bindings.capacity() as u64).saturating_mul(12))
            .saturating_add((results.capacity() as u64).saturating_mul(4));
        if held > memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: memory,
                needed_bytes: None,
            }));
        }
        if formulas.len() as u64 > most || tasks.len() as u64 > most {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: most.saturating_add(1),
                limit: most,
            }));
        }
        match task {
            Task::Enter(n, at) => match &nnf.nodes[n as usize] {
                G::Lit(positive, predicate, args) => {
                    tuple.clear();
                    tuple.extend(args.iter().map(|t| match *t {
                        Term::Var(x) => lookup(&bindings, at, x),
                        Term::Const(c) => element_of[&c],
                    }));
                    let hash = std::hash::BuildHasher::hash_one(&hasher, (*predicate, &tuple));
                    let mut found = first.get(&hash).copied().unwrap_or(NONE);
                    while found != NONE {
                        let (p, start, len) = atoms[found as usize];
                        let (start, len) = (start as usize, len as usize);
                        if p == *predicate && args_of[start..start + len] == tuple[..] {
                            break;
                        }
                        found = next[found as usize];
                    }
                    let atom = if found == NONE {
                        let index = index_of(atoms.len())?;
                        let start = index_of(args_of.len())?;
                        let id = formulas.atom(&format!("g_{index}"))?;
                        atom_bytes = atom_bytes
                            .saturating_add(ATOM_BYTES.saturating_add(4 * tuple.len() as u64));
                        args_of.extend_from_slice(&tuple);
                        atoms.push((*predicate, start, tuple.len() as u32));
                        next.push(first.insert(hash, index).unwrap_or(NONE));
                        ids.push(id);
                        id
                    } else {
                        ids[found as usize]
                    };
                    let node = if *positive {
                        atom
                    } else {
                        formulas.add(Node::Not(atom))?
                    };
                    results.push(node);
                }
                G::True => results.push(formulas.add(Node::True)?),
                G::False => results.push(formulas.add(Node::False)?),
                G::And(k) | G::Or(k) => {
                    let and = matches!(nnf.nodes[n as usize], G::And(_));
                    tasks.push(Task::Join(and, k.len()));
                    for &c in k.iter().rev() {
                        tasks.push(Task::Enter(c, at));
                    }
                }
                G::Forall(x, b) => {
                    bindings.push((*x, skolem_of[x], at));
                    tasks.push(Task::Enter(*b, (bindings.len() - 1) as u32));
                }
                G::Exists(x, b) => {
                    tasks.push(Task::Join(false, size as usize));
                    for e in (0..size).rev() {
                        bindings.push((*x, e, at));
                        tasks.push(Task::Enter(*b, (bindings.len() - 1) as u32));
                    }
                }
            },
            Task::Join(and, count) => {
                let start = results.len() - count;
                let parts = results.split_off(start);
                let mut iter = parts.into_iter();
                let mut acc = iter.next().expect("a junction has operands");
                for p in iter {
                    acc = formulas.add(if and {
                        Node::And(acc, p)
                    } else {
                        Node::Or(acc, p)
                    })?;
                }
                results.push(acc);
            }
        }
    }
    let formula = results.pop().expect("the ground formula");
    Ok(Ground {
        sequent: Sequent::new(formulas, Vec::new(), vec![formula])?,
        atoms,
        args: args_of,
        elements,
    })
}

/// Returns the structure the ground atoms `true_atoms` (by name) give over
/// the ground problem's domain.
fn structure_of(sequent: &Sequent, ground: &Ground, true_atoms: &[String]) -> Structure {
    let tables = sequent.formulas().first_order();
    let formulas = sequent.formulas();
    let name = |predicate: u32| -> String {
        if tables.atoms.is_empty() {
            formulas.atom_name(predicate).to_owned()
        } else {
            tables.symbols[predicate as usize].0.clone()
        }
    };
    let constants: Vec<(String, u32)> = ground
        .elements
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c != NONE)
        .map(|(e, &c)| (tables.symbols[c as usize].0.clone(), e as u32))
        .collect();
    let mut holds: HashMap<u32, Vec<Vec<u32>>> = HashMap::default();
    for atom in true_atoms {
        let Some(index) = atom
            .strip_prefix("g_")
            .and_then(|n| n.parse::<usize>().ok())
        else {
            continue;
        };
        let (predicate, start, len) = ground.atoms[index];
        let (start, len) = (start as usize, len as usize);
        holds
            .entry(predicate)
            .or_default()
            .push(ground.args[start..start + len].to_vec());
    }
    let mut predicates: Vec<(String, Vec<Vec<u32>>)> = holds
        .into_iter()
        .map(|(p, mut tuples)| {
            tuples.sort_unstable();
            (name(p), tuples)
        })
        .collect();
    predicates.sort_unstable();
    Structure::new(ground.elements.len() as u32, constants, predicates)
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::decide;
    use crate::Limits;
    use crate::linear::search::generate::Rng;
    use crate::linear::search::{self, Reason};
    use crate::ordinary::{Evaluation, Evidence, Logic, Sequent, Verdict};

    /// Decides `text` by grounding in `logic`, or `None` outside the
    /// classes.
    fn grounded(text: &str, logic: Logic) -> Option<super::Outcome> {
        let sequent: Sequent = text.parse().unwrap();
        decide(&sequent, logic, &Limits::default(), |_| false)
            .unwrap_or_else(|e| panic!("{text}: {e}"))
    }

    /// Sequents of both classes decided by hand: the valid ones with a
    /// checked clausal proof, the others with a structure that refutes
    /// them; outside the classes, no answer; in the other logics, the
    /// refutations stand and the classically valid ones are unknown.
    #[test]
    fn sequents_of_the_classes_decided_by_hand() {
        let valid = [
            "forall x. (p(x) -> q(x)), p(a) |- q(a)",
            "exists x. forall y. r(x, y) |- forall y. exists x. r(x, y)",
            "|- exists x. (p(x) -> forall y. p(y))",
            "|- exists x. forall y. (p(x) -> p(y))",
            "forall x. (p(x) \\/ q(x)), exists x. ~p(x) |- exists x. q(x)",
            "forall x. p(x) |- exists y. p(y)",
            "|- (forall x. p(x)) <-> ~exists x. ~p(x)",
            "|- true",
        ];
        for text in valid {
            let outcome = grounded(text, Logic::Classical).unwrap_or_else(|| panic!("{text}"));
            assert!(
                matches!(outcome.verdict, Verdict::Valid(Evidence::Clausal(_))),
                "{text}: {:?}",
                outcome.verdict
            );
        }
        let refuted = [
            "exists x. p(x) |- forall x. p(x)",
            "p(a) |- p(b)",
            "forall x. (p(x) \\/ q(x)) |- forall x. p(x)",
            "exists x. p(x), exists x. q(x) |- exists x. (p(x) /\\ q(x))",
            "|- false",
        ];
        for text in refuted {
            for &logic in Logic::ALL {
                let outcome = grounded(text, logic).unwrap_or_else(|| panic!("{text}"));
                assert!(matches!(outcome.verdict, Verdict::NotValid), "{text}");
                let countermodel = outcome.structure.expect("a structure");
                let sequent: Sequent = text.parse().unwrap();
                assert_eq!(
                    countermodel
                        .structure
                        .refutes(&sequent, &Limits::default(), |_| false)
                        .unwrap(),
                    Evaluation::Refutes
                );
                countermodel
                    .check(logic, &Limits::default(), |_| false)
                    .unwrap();
            }
        }
        for text in [
            "forall x. exists y. r(x, y) |- r(a, a)",
            "p(f(a)) |- p(a)",
            "forall x. exists y. r(x, y) |- exists y. forall x. r(x, y)",
        ] {
            assert!(grounded(text, Logic::Classical).is_none(), "{text}");
        }
        let excluded = grounded("|- p(a) \\/ ~p(a)", Logic::Intuitionistic).unwrap();
        assert!(matches!(
            excluded.verdict,
            Verdict::Unknown(Reason::Incomplete)
        ));
    }

    /// Returns a function-free formula over `p/1`, `q/2`, `r/0` and the
    /// constants `a`, `b` and the bound variables in scope.
    fn formula(rng: &mut Rng, depth: usize, scope: &mut Vec<String>) -> String {
        let term = |rng: &mut Rng, scope: &[String]| match rng.below(scope.len() + 2) {
            0 => "a".to_owned(),
            1 => "b".to_owned(),
            k => scope[k - 2].clone(),
        };
        if depth == 0 || rng.one_in(4) {
            return match rng.below(5) {
                0 => "r".to_owned(),
                1 | 2 => format!("p({})", term(rng, scope)),
                _ => format!("q({}, {})", term(rng, scope), term(rng, scope)),
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

    /// On generated function-free sequents, grounding and the connection
    /// prover never contradict each other, and every structure grounding
    /// gives refutes its sequent.
    #[test]
    fn grounding_agrees_with_the_connection_prover() {
        let mut rng = Rng::new(7_504);
        let limits = Limits {
            work: Some(1 << 16),
            ..Limits::default()
        };
        let (mut decided, mut refuted) = (0, 0);
        for _ in 0..400 {
            let hypotheses: Vec<String> = (0..rng.below(3))
                .map(|_| formula(&mut rng, 3, &mut Vec::new()))
                .collect();
            let goal = formula(&mut rng, 3, &mut Vec::new());
            let text = format!("{} |- {goal}", hypotheses.join(", "));
            let sequent: Sequent = text.parse().unwrap();
            let Some(ours) = decide(&sequent, Logic::Classical, &limits, |_| false)
                .unwrap_or_else(|e| panic!("{text}: {e}"))
            else {
                continue;
            };
            decided += 1;
            let theirs = crate::ordinary::connection::decide(
                &sequent,
                Logic::Classical,
                &search::Options::default(),
                &limits,
                |_| false,
            );
            match (&ours.verdict, theirs.map(|o| o.verdict)) {
                (Verdict::NotValid, Ok(Verdict::Valid(_)))
                | (Verdict::Valid(_), Ok(Verdict::NotValid)) => panic!("{text}"),
                _ => {}
            }
            if matches!(ours.verdict, Verdict::NotValid) {
                refuted += 1;
                let countermodel = ours.structure.expect("a structure");
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
        assert!(decided > 100 && refuted > 20, "{decided} {refuted}");
    }
}
