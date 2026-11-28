// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Kripke models of first-order sequents with growing domains, and their
//! check, the forcing relation of Troelstra and van Dalen, *Constructivism
//! in Mathematics* I, §2.5: worlds partially ordered, a domain per world
//! that grows along the order, function symbols interpreted per world and
//! agreeing on the smaller domains, predicates growing.
//!
//! A world forces `A → B` (and `¬A`, which is `A → ⊥`) where every world
//! above it that forces `A` forces `B`, `∀x. A` where every world above it
//! forces `A` of every element of its domain, `∃x. A` where `A` holds of
//! some element of its own domain; `∧` and `∨` are as classically, and an
//! atom as the world's predicates say, `s = t` where `s` and `t` denote
//! one element (equality is identity). `⊥` is forced nowhere, but in
//! minimal logic where the world's `falsity` says so. Every world forces
//! what a world below it forces, since the domains, the predicates and
//! `falsity` grow; the check tests that growth and does not rely on it.

use super::semantics::{ENTRY_BYTES, Interpretation, Meter, tables_bytes};
use super::structure::{Evaluation, SHOWN, write_domain, write_function, write_predicate};
use super::{Logic, Node, NodeId, Sequent, Structure};
use crate::Error;
use crate::first_order::FoTerm;
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal};

/// A finite Kripke model of a first-order language: worlds with a domain
/// each, ordered by their arrows, and the world it is about.
///
/// # JSON
///
/// With the feature `serialize` it is `{"worlds": [{"size": 1,
/// "functions": [["f", [0]]], "predicates": [["p", [[0]]]], "falsity":
/// true, "arrows": [1]}, …], "constants": [["a", 0]], "world": 0}`: each
/// world's domain size (its elements are `0 .. size`), its function
/// tables over its own domain in the lexicographic order of the
/// arguments, its predicates' tuples, whether it forces `⊥` (minimal
/// logic), and the worlds directly above it; each constant's element,
/// which every world's domain holds; the world refuted (0 where left
/// out). The keys a world has none of are left out.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(deny_unknown_fields)
)]
pub struct KripkeStructure {
    /// The worlds.
    pub worlds: Vec<KripkeWorld>,
    /// Each constant of the sequent, by name, with the element it denotes
    /// in every world.
    #[cfg_attr(
        feature = "serialize",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub constants: Vec<(String, u32)>,
    /// The world the model is about: the one that refutes the sequent.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub world: u32,
}

/// A world of a [`KripkeStructure`].
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(deny_unknown_fields)
)]
pub struct KripkeWorld {
    /// The number of elements, at least 1: the domain is `0 .. size`, and
    /// no world above has fewer.
    pub size: u32,
    /// Each function symbol of one argument or more, by name, with its
    /// table over this world's domain: the values of its `size^arity`
    /// argument tuples in lexicographic order. A world above gives the
    /// tuples of this domain the same values.
    #[cfg_attr(
        feature = "serialize",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub functions: Vec<(String, Vec<u32>)>,
    /// Each predicate, by name, with the tuples it holds of here; a world
    /// above holds them too. Equality is not listed: every world reads it
    /// as identity.
    #[cfg_attr(
        feature = "serialize",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub predicates: Vec<(String, Vec<Vec<u32>>)>,
    /// Whether the world forces `⊥`: in minimal logic only, and then in
    /// every world above too.
    #[cfg_attr(
        feature = "serialize",
        serde(default, skip_serializing_if = "std::ops::Not::not")
    )]
    pub falsity: bool,
    /// The worlds directly above.
    #[cfg_attr(
        feature = "serialize",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub arrows: Vec<u32>,
}

impl KripkeWorld {
    /// Returns the world of `size` elements with these function tables,
    /// predicates, `falsity` and arrows, unchecked.
    pub const fn new(
        size: u32,
        functions: Vec<(String, Vec<u32>)>,
        predicates: Vec<(String, Vec<Vec<u32>>)>,
        falsity: bool,
        arrows: Vec<u32>,
    ) -> Self {
        Self {
            size,
            functions,
            predicates,
            falsity,
            arrows,
        }
    }
}

impl From<Structure> for KripkeStructure {
    /// A finite structure as the Kripke model of one world.
    fn from(structure: Structure) -> Self {
        Self {
            worlds: vec![KripkeWorld::new(
                structure.size,
                structure.functions,
                structure.predicates,
                false,
                Vec::new(),
            )],
            constants: structure.constants,
            world: 0,
        }
    }
}

/// The bytes a world holds in the check beside its tables: its entry, its
/// arrows, its list of the worlds above it and its tables' maps (a model of
/// three million worlds of one element measured 240 bytes a world).
const WORLD_BYTES: u64 = 256;

impl KripkeStructure {
    /// Returns the model of these worlds, constants and refuted world,
    /// unchecked.
    pub const fn new(worlds: Vec<KripkeWorld>, constants: Vec<(String, u32)>, world: u32) -> Self {
        Self {
            worlds,
            constants,
            world,
        }
    }

    /// Checks that the model is a Kripke model of `logic` that interprets
    /// the sequent, and says whether its world refutes it: forces every
    /// hypothesis and not the formula right of `⊢` (none, in which case
    /// nothing is asked of the right). Asks `stop` every 4 096 evaluations.
    ///
    /// # Errors
    ///
    /// [`Error::Structure`] for a model that is no Kripke model (no world,
    /// an arrow out of range or in a cycle, a domain that shrinks, a
    /// predicate or `falsity` that does not grow, a table that does not
    /// agree with the world below, `falsity` outside minimal logic, a
    /// world that lists equality, which every world reads as identity) or that
    /// cannot interpret the sequent (as [`Structure::refutes`] says);
    /// [`Refusal::Memory`] past `limits.memory_bytes`; [`Refusal::Work`]
    /// past `limits.work` evaluations ([`Structure::WORK`] where it sets
    /// none); [`Refusal::Stopped`] when the stop fires.
    pub fn refutes(
        &self,
        sequent: &Sequent,
        logic: Logic,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Evaluation, Error> {
        self.evaluate(sequent, logic, limits, &mut stop)
    }

    /// [`refutes`](Self::refutes) with the stop as a trait object.
    #[expect(
        clippy::too_many_lines,
        reason = "one pass of checks in the order the errors are written"
    )]
    fn evaluate(
        &self,
        sequent: &Sequent,
        logic: Logic,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Evaluation, Error> {
        let wrong = |reason: String| Error::Structure {
            reason: format!("it {reason}"),
        };
        let count = self.worlds.len();
        if count == 0 {
            return Err(wrong("has no world".to_owned()));
        }
        let index = |w: u32| (w as usize) < count;
        if !index(self.world) {
            return Err(wrong(format!(
                "is about world {}, which it lacks",
                self.world
            )));
        }
        // What the model and the check's tables hold, before any is built.
        let memory = limits.memory_bytes.unwrap_or(u64::MAX);
        let mut held = (count as u64)
            .saturating_mul(WORLD_BYTES)
            .saturating_add((self.constants.len() as u64).saturating_mul(ENTRY_BYTES));
        for world in &self.worlds {
            held = held
                .saturating_add((world.arrows.len() as u64).saturating_mul(4))
                .saturating_add(tables_bytes(&world.functions, &world.predicates));
        }
        let refuse = |needed: u64| {
            Error::Refused(Refusal::Memory {
                phase: Phase::Check,
                limit_bytes: memory,
                needed_bytes: Some(needed),
            })
        };
        if held > memory {
            return Err(refuse(held));
        }
        // The arrows: in range, and no cycle (Kahn's order, a world after
        // every world below it).
        let mut below = vec![0u32; count];
        for (w, world) in self.worlds.iter().enumerate() {
            if world.size == 0 {
                return Err(wrong(format!("gives world {w} no element")));
            }
            for &u in &world.arrows {
                if !index(u) {
                    return Err(wrong(format!(
                        "draws an arrow from world {w} to world {u}, which it lacks"
                    )));
                }
                below[u as usize] = below[u as usize].saturating_add(1);
            }
        }
        let mut order: Vec<u32> = (0..count as u32)
            .filter(|&w| below[w as usize] == 0)
            .collect();
        let mut at = 0;
        while at < order.len() {
            let w = order[at] as usize;
            at += 1;
            poll(stop, at)?;
            for &u in &self.worlds[w].arrows {
                below[u as usize] -= 1;
                if below[u as usize] == 0 {
                    order.push(u);
                }
            }
        }
        if order.len() < count {
            return Err(wrong("draws its arrows in a cycle".to_owned()));
        }
        // Each world's arrows, each once: an arrow written twice is one.
        let ups: Vec<Vec<u32>> = self
            .worlds
            .iter()
            .map(|world| {
                let mut ups = world.arrows.clone();
                ups.sort_unstable();
                ups.dedup();
                ups
            })
            .collect();
        // The worlds at or above each world, the latest in Kahn's order
        // first: each holds its own and those above its arrows' ends.
        let mut above: Vec<Vec<u32>> = vec![Vec::new(); count];
        let mut mark = vec![u32::MAX; count];
        // The steps of the passes below, for the stop and the work bound.
        let most_work = limits.work.unwrap_or(Structure::WORK);
        let mut steps: u64 = 0;
        for &w in order.iter().rev() {
            let mut list = vec![w];
            mark[w as usize] = w;
            for &u in &ups[w as usize] {
                for &v in &above[u as usize] {
                    steps += 1;
                    if steps.is_multiple_of(4_096)
                        && stop(Progress::new(Phase::Check, 4_096, steps))
                    {
                        return Err(Error::Refused(Refusal::stopped(Phase::Check)));
                    }
                    if steps > most_work {
                        return Err(Error::Refused(Refusal::Work { limit: most_work }));
                    }
                    if mark[v as usize] != w {
                        mark[v as usize] = w;
                        list.push(v);
                    }
                }
            }
            held = held.saturating_add((list.len() as u64).saturating_mul(4));
            if held > memory {
                return Err(refuse(held));
            }
            above[w as usize] = list;
        }
        // Growth along every arrow: domains, falsity, predicates.
        let mut holds: Vec<HashMap<&str, HashSet<&[u32]>>> = Vec::with_capacity(count);
        for (w, world) in self.worlds.iter().enumerate() {
            if world.falsity && logic != Logic::Minimal {
                return Err(wrong(format!(
                    "makes world {w} force ⊥, which only minimal logic allows"
                )));
            }
            let mut sets: HashMap<&str, HashSet<&[u32]>> = HashMap::default();
            for (name, tuples) in &world.predicates {
                if name == crate::first_order::EQUALS {
                    return Err(wrong(format!(
                        "lists equality at world {w}, which every world reads as identity"
                    )));
                }
                let set = sets.entry(name.as_str()).or_default();
                for tuple in tuples {
                    if tuple.iter().any(|&e| e >= world.size) {
                        return Err(wrong(format!(
                            "puts an element past world {w}'s domain in {name}"
                        )));
                    }
                    set.insert(tuple.as_slice());
                }
            }
            holds.push(sets);
        }
        for (w, world) in self.worlds.iter().enumerate() {
            poll(stop, w)?;
            for &u in &ups[w] {
                let up = &self.worlds[u as usize];
                if up.size < world.size {
                    return Err(wrong(format!(
                        "gives world {u} fewer elements than world {w} below it"
                    )));
                }
                if world.falsity && !up.falsity {
                    return Err(wrong(format!(
                        "makes world {w} force ⊥ and world {u} above it not"
                    )));
                }
                for (name, set) in &holds[w] {
                    for tuple in set {
                        steps += 1;
                        if steps.is_multiple_of(4_096)
                            && stop(Progress::new(Phase::Check, 4_096, steps))
                        {
                            return Err(Error::Refused(Refusal::stopped(Phase::Check)));
                        }
                        if steps > most_work {
                            return Err(Error::Refused(Refusal::Work { limit: most_work }));
                        }
                        if !holds[u as usize]
                            .get(name)
                            .is_some_and(|s| s.contains(tuple))
                        {
                            return Err(wrong(format!(
                                "lets {name} hold of fewer tuples at world {u} than at world {w} \
                                 below it"
                            )));
                        }
                    }
                }
            }
        }
        // The constants, in every world's domain.
        let least = self.worlds.iter().map(|w| w.size).min().unwrap_or(0);
        let mut elements: HashMap<&str, u32> = HashMap::default();
        for (name, e) in &self.constants {
            if *e >= least {
                return Err(wrong(format!(
                    "names element {e} for {name}, which some world's domain lacks"
                )));
            }
            elements.insert(name.as_str(), *e);
        }
        // The terms the sequent's formulas use: each constant named, each
        // function symbol with its arity.
        let formulas = sequent.formulas();
        let tables = formulas.first_order();
        let roots = sequent.left().iter().chain(sequent.right()).copied();
        let mut arity: HashMap<&str, u32> = HashMap::default();
        for t in formulas.terms_in(roots, std::iter::empty()) {
            match tables.terms[t.index()] {
                FoTerm::App(f, args) => {
                    let name = tables.symbols[f.index()].0.as_str();
                    let args = tables.list(args);
                    if args.is_empty() {
                        if !elements.contains_key(name) {
                            return Err(wrong(format!("names no element for the constant {name}")));
                        }
                        continue;
                    }
                    arity.insert(name, u32::try_from(args.len()).unwrap_or(u32::MAX));
                }
                FoTerm::Bound(_) => {}
                FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                    return Err(wrong(
                        "cannot interpret an eigenvariable or an open witness".to_owned(),
                    ));
                }
            }
        }
        // Each function the sequent applies: a table at every world of one
        // value in its domain for each tuple, agreeing along every arrow
        // on the tuples of the smaller domain.
        let mut functions: Vec<HashMap<&str, &[u32]>> = Vec::with_capacity(count);
        for (w, world) in self.worlds.iter().enumerate() {
            let mut map: HashMap<&str, &[u32]> = HashMap::default();
            for (name, table) in &world.functions {
                map.insert(name.as_str(), table.as_slice());
            }
            for (&name, &k) in &arity {
                let cells = u64::from(world.size).checked_pow(k);
                match map.get(name) {
                    Some(table) if cells == Some(table.len() as u64) => {
                        if table.iter().any(|&e| e >= world.size) {
                            return Err(wrong(format!(
                                "gives {name} a value past world {w}'s domain"
                            )));
                        }
                    }
                    Some(_) => {
                        return Err(wrong(format!(
                            "gives {name} a table at world {w} that is not one value for each \
                             of its argument tuples"
                        )));
                    }
                    None => {
                        return Err(wrong(format!(
                            "has no table for the function {name} at world {w}"
                        )));
                    }
                }
            }
            functions.push(map);
        }
        for (w, world) in self.worlds.iter().enumerate() {
            for &u in &ups[w] {
                let up = &self.worlds[u as usize];
                for (&name, &k) in &arity {
                    let (low, high) = (functions[w][name], functions[u as usize][name]);
                    let (n, m) = (world.size as usize, up.size as usize);
                    for (i, &value) in low.iter().enumerate() {
                        // An entry costs its arguments, each a step.
                        let before = steps;
                        steps = steps.saturating_add(u64::from(k));
                        if steps / 4_096 != before / 4_096
                            && stop(Progress::new(Phase::Check, 4_096, steps))
                        {
                            return Err(Error::Refused(Refusal::stopped(Phase::Check)));
                        }
                        if steps > most_work {
                            return Err(Error::Refused(Refusal::Work { limit: most_work }));
                        }
                        // The tuple's place in the larger table.
                        let mut rest = i;
                        let mut place = 0usize;
                        let mut scale = 1usize;
                        for _ in 0..k {
                            place += (rest % n) * scale;
                            rest /= n;
                            scale *= m;
                        }
                        if high[place] != value {
                            return Err(wrong(format!(
                                "gives {name} another value at world {u} than at world {w} \
                                 below it, on an argument tuple of world {w}'s domain"
                            )));
                        }
                    }
                }
            }
        }
        // One world refutes classically only where no world lies above it.
        if logic == Logic::Classical && above[self.world as usize].len() > 1 {
            return Err(wrong(
                "has worlds above the one it is about, which classical logic does not read"
                    .to_owned(),
            ));
        }
        let mut eval = Forcing {
            sequent,
            worlds: &self.worlds,
            above: &above,
            elements,
            functions,
            holds,
            meter: Meter::new(limits.work.unwrap_or(Structure::WORK), stop),
            wrong: &wrong,
        };
        let root = self.world;
        // Nothing right of `⊢` is `⊥` there, which minimal logic's
        // `falsity` may force.
        if sequent.right().is_empty() && self.worlds[root as usize].falsity {
            return Err(wrong(
                "makes its world force ⊥, which the empty right side of the sequent denies"
                    .to_owned(),
            ));
        }
        for (at, &f) in sequent.left().iter().enumerate() {
            if !eval.forced(f, root)? {
                return Ok(Evaluation::FalseHypothesis(at));
            }
        }
        for (at, &f) in sequent.right().iter().enumerate() {
            if eval.forced(f, root)? {
                return Ok(Evaluation::TrueGoal(at));
            }
        }
        Ok(Evaluation::Refutes)
    }
}

/// Asks the stop once every 4 096 steps of a pass over the worlds.
fn poll(stop: &mut dyn FnMut(Progress) -> bool, step: usize) -> Result<(), Error> {
    if step % 4_096 == 4_095 && stop(Progress::new(Phase::Check, 4_096, step as u64)) {
        return Err(Error::Refused(Refusal::stopped(Phase::Check)));
    }
    Ok(())
}

/// A step of the forcing evaluation.
enum Step {
    /// Decide whether the world forces the formula.
    Eval(NodeId, u32),
    /// The first operand of `∧` or `∨` was decided at the world (its value
    /// on top of the stack): it decides the formula, or the second's value
    /// is the formula's.
    Second(Node, NodeId, u32),
    /// `A → B` (or `¬A`, `B` none) at the world, the worlds above it before
    /// `next` done: at `stage` 0 the next world is taken, at 1 `A`'s value
    /// there is on the stack, at 2 `B`'s.
    Implies {
        /// The antecedent.
        a: NodeId,
        /// The consequent, or `⊥`.
        b: Option<NodeId>,
        /// The world.
        world: u32,
        /// The next world above, by its place in the list.
        next: usize,
        /// Where the step stands.
        stage: u8,
    },
    /// The first implication of `A ↔ B` was decided (its value on top of
    /// the stack); the second is `B → A`.
    Iff(NodeId, NodeId, u32),
    /// `∀x. A` at the world: the worlds above it before `next`, and the
    /// elements of the next one before `element`, done; `started` where the
    /// last element's value is on top of the stack.
    Forall {
        /// The body.
        body: NodeId,
        /// The world.
        world: u32,
        /// The next world above, by its place in the list.
        next: usize,
        /// The next element of it.
        element: u32,
        /// Whether a value waits on the stack.
        started: bool,
    },
    /// `∃x. A` at the world: the elements before `element` done; `started`
    /// where the last element's value is on top of the stack.
    Exists {
        /// The body.
        body: NodeId,
        /// The world.
        world: u32,
        /// The next element.
        element: u32,
        /// Whether a value waits on the stack.
        started: bool,
    },
}

/// The state of a forcing evaluation.
struct Forcing<'a, 's> {
    /// The sequent.
    sequent: &'a Sequent,
    /// The worlds.
    worlds: &'a [KripkeWorld],
    /// The worlds at or above each world.
    above: &'a [Vec<u32>],
    /// Each constant's element.
    elements: HashMap<&'a str, u32>,
    /// Each world's function tables, checked up front.
    functions: Vec<HashMap<&'a str, &'a [u32]>>,
    /// Each world's predicates' tuples.
    holds: Vec<HashMap<&'a str, HashSet<&'a [u32]>>>,
    /// The evaluations made and the stop.
    meter: Meter<'s>,
    /// Makes the error of a model that cannot interpret the sequent.
    wrong: &'s dyn Fn(String) -> Error,
}

impl Forcing<'_, '_> {
    /// Returns whether `world` forces the closed formula `root`.
    #[expect(
        clippy::too_many_lines,
        reason = "one match over the steps of the forcing relation"
    )]
    fn forced(&mut self, root: NodeId, world: u32) -> Result<bool, Error> {
        let formulas = self.sequent.formulas();
        let mut env: Vec<u32> = Vec::new();
        let mut values: Vec<bool> = Vec::new();
        let mut stack = vec![Step::Eval(root, world)];
        while let Some(step) = stack.pop() {
            match step {
                Step::Eval(n, w) => {
                    self.tick()?;
                    let node = formulas.node(n);
                    match node {
                        Node::Atom(a) => values.push(self.atom(a, w, &env)?),
                        Node::True => values.push(true),
                        Node::False => values.push(self.worlds[w as usize].falsity),
                        Node::Not(a) => stack.push(Step::Implies {
                            a,
                            b: None,
                            world: w,
                            next: 0,
                            stage: 0,
                        }),
                        Node::And(a, b) | Node::Or(a, b) => {
                            stack.push(Step::Second(node, b, w));
                            stack.push(Step::Eval(a, w));
                        }
                        Node::Implies(a, b) => stack.push(Step::Implies {
                            a,
                            b: Some(b),
                            world: w,
                            next: 0,
                            stage: 0,
                        }),
                        Node::Iff(a, b) => {
                            stack.push(Step::Iff(a, b, w));
                            stack.push(Step::Implies {
                                a,
                                b: Some(b),
                                world: w,
                                next: 0,
                                stage: 0,
                            });
                        }
                        Node::Forall(body) => stack.push(Step::Forall {
                            body,
                            world: w,
                            next: 0,
                            element: 0,
                            started: false,
                        }),
                        Node::Exists(body) => stack.push(Step::Exists {
                            body,
                            world: w,
                            element: 0,
                            started: false,
                        }),
                    }
                }
                Step::Second(node, b, w) => {
                    let a = values.pop().expect("an operand's value");
                    let decided = match node {
                        Node::And(..) => (!a).then_some(false),
                        Node::Or(..) => a.then_some(true),
                        Node::Atom(_)
                        | Node::True
                        | Node::False
                        | Node::Not(_)
                        | Node::Implies(..)
                        | Node::Iff(..)
                        | Node::Forall(_)
                        | Node::Exists(_) => unreachable!("∧ or ∨"),
                    };
                    match decided {
                        Some(value) => values.push(value),
                        None => stack.push(Step::Eval(b, w)),
                    }
                }
                Step::Iff(a, b, w) => {
                    if values.pop().expect("the first implication's value") {
                        stack.push(Step::Implies {
                            a: b,
                            b: Some(a),
                            world: w,
                            next: 0,
                            stage: 0,
                        });
                    } else {
                        values.push(false);
                    }
                }
                Step::Implies {
                    a,
                    b,
                    world: w,
                    next,
                    stage,
                } => {
                    let above = &self.above[w as usize];
                    match stage {
                        0 => match above.get(next) {
                            None => values.push(true),
                            Some(&u) => {
                                stack.push(Step::Implies {
                                    a,
                                    b,
                                    world: w,
                                    next,
                                    stage: 1,
                                });
                                stack.push(Step::Eval(a, u));
                            }
                        },
                        1 => {
                            let u = above[next];
                            let skip = Step::Implies {
                                a,
                                b,
                                world: w,
                                next: next + 1,
                                stage: 0,
                            };
                            if !values.pop().expect("the antecedent's value") {
                                stack.push(skip);
                            } else if let Some(b) = b {
                                stack.push(Step::Implies {
                                    a,
                                    b: Some(b),
                                    world: w,
                                    next,
                                    stage: 2,
                                });
                                stack.push(Step::Eval(b, u));
                            } else if self.worlds[u as usize].falsity {
                                stack.push(skip);
                            } else {
                                values.push(false);
                            }
                        }
                        _ => {
                            if values.pop().expect("the consequent's value") {
                                stack.push(Step::Implies {
                                    a,
                                    b,
                                    world: w,
                                    next: next + 1,
                                    stage: 0,
                                });
                            } else {
                                values.push(false);
                            }
                        }
                    }
                }
                Step::Forall {
                    body,
                    world: w,
                    mut next,
                    mut element,
                    started,
                } => {
                    if started {
                        env.pop();
                        if !values.pop().expect("the body's value") {
                            values.push(false);
                            continue;
                        }
                        element += 1;
                    }
                    let above = &self.above[w as usize];
                    // The next world above with an element left.
                    while let Some(&u) = above.get(next) {
                        if element < self.worlds[u as usize].size {
                            break;
                        }
                        next += 1;
                        element = 0;
                    }
                    let Some(&u) = above.get(next) else {
                        values.push(true);
                        continue;
                    };
                    stack.push(Step::Forall {
                        body,
                        world: w,
                        next,
                        element,
                        started: true,
                    });
                    env.push(element);
                    stack.push(Step::Eval(body, u));
                }
                Step::Exists {
                    body,
                    world: w,
                    mut element,
                    started,
                } => {
                    if started {
                        env.pop();
                        if values.pop().expect("the body's value") {
                            values.push(true);
                            continue;
                        }
                        element += 1;
                    }
                    if element == self.worlds[w as usize].size {
                        values.push(false);
                        continue;
                    }
                    stack.push(Step::Exists {
                        body,
                        world: w,
                        element,
                        started: true,
                    });
                    env.push(element);
                    stack.push(Step::Eval(body, w));
                }
            }
        }
        Ok(values.pop().expect("the formula's value"))
    }

    /// Counts an evaluation, refusing past the bound and asking the stop.
    fn tick(&mut self) -> Result<(), Error> {
        self.meter.tick_by(1)
    }

    /// Returns whether world `w` forces the atom `a` under the bound
    /// elements `env`, the innermost last.
    fn atom(&mut self, a: u32, w: u32, env: &[u32]) -> Result<bool, Error> {
        let world = Interpretation {
            size: self.worlds[w as usize].size,
            elements: &self.elements,
            functions: &self.functions[w as usize],
            holds: &self.holds[w as usize],
        };
        world.atom(self.sequent, a, env, &mut self.meter, self.wrong)
    }
}

/// A Kripke model with growing domains said to refute a first-order
/// sequent: what the model search answers in intuitionistic and minimal
/// logic.
///
/// # JSON
///
/// With the feature `serialize` it is `{"sequent": …, "model": …}`, the
/// ordinary sequent's form and the [`KripkeStructure`]'s, inside an
/// ordinary outcome under the key `kripke_structure`.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct KripkeCountermodel {
    /// The sequent refuted.
    pub sequent: Sequent,
    /// The model. Its world forces every formula left of the sequent's `⊢`
    /// and not the one right of it, as [`check`](Self::check) checks.
    pub model: KripkeStructure,
}

impl KripkeCountermodel {
    /// Returns the countermodel of this sequent and model, unchecked.
    pub const fn new(sequent: Sequent, model: KripkeStructure) -> Self {
        Self { sequent, model }
    }

    /// Checks that the model is one of `logic` and refutes the sequent,
    /// asking `stop` as [`KripkeStructure::refutes`] does. A model that
    /// refutes a sequent in intuitionistic logic refutes it in minimal
    /// logic too; in classical logic only a world with none above it
    /// refutes.
    ///
    /// # Errors
    ///
    /// [`Error::Succedents`] for more than one formula right of `⊢`
    /// outside classical logic; [`Error::Structure`] for a model that is
    /// none or does not refute the sequent, naming the first formula it
    /// gets wrong; the refusals of [`KripkeStructure::refutes`].
    pub fn check(
        &self,
        logic: Logic,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let count = self.sequent.right().len();
        if logic != Logic::Classical && count > 1 {
            return Err(Error::Succedents { count });
        }
        let (side, formula, value) =
            match self
                .model
                .evaluate(&self.sequent, logic, limits, &mut stop)?
            {
                Evaluation::Refutes => return Ok(()),
                Evaluation::FalseHypothesis(at) => {
                    ("hypothesis", self.sequent.left()[at], "does not force")
                }
                Evaluation::TrueGoal(at) => {
                    ("formula right of ⊢", self.sequent.right()[at], "forces")
                }
            };
        let written = self.sequent.formulas().formula(formula);
        Err(Error::Structure {
            reason: format!("its world {value} the {side} {written}"),
        })
    }
}

impl std::fmt::Display for KripkeStructure {
    /// Writes the model in lines: each constant's element (`a = 0`), then a
    /// line per world, its domain, its function tables, its predicates and
    /// `⊥` where it forces it, and the worlds directly above it (`world 0
    /// (the model's world): domain {0}; p: {0}; below 1`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for (name, e) in &self.constants {
            if !first {
                writeln!(f)?;
            }
            first = false;
            write!(f, "{name} = {e}")?;
        }
        for (w, world) in self.worlds.iter().enumerate() {
            if !first {
                writeln!(f)?;
            }
            first = false;
            write!(f, "world {w}")?;
            if w as u32 == self.world {
                write!(f, " (the model's world)")?;
            }
            write!(f, ": domain ")?;
            write_domain(f, world.size)?;
            for (name, table) in &world.functions {
                write!(f, "; ")?;
                write_function(f, name, table, world.size)?;
            }
            for (name, tuples) in &world.predicates {
                if !tuples.is_empty() {
                    write!(f, "; ")?;
                    write_predicate(f, name, tuples)?;
                }
            }
            if world.falsity {
                write!(f, "; ⊥")?;
            }
            if !world.arrows.is_empty() {
                write!(f, "; below ")?;
                for (i, u) in world.arrows.iter().enumerate().take(SHOWN) {
                    let comma = if i == 0 { "" } else { ", " };
                    write!(f, "{comma}{u}")?;
                }
                if world.arrows.len() > SHOWN {
                    write!(f, ", … ({} more)", world.arrows.len() - SHOWN)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::{KripkeCountermodel, KripkeStructure, KripkeWorld};
    use crate::ordinary::{Evaluation, Logic, Sequent, Structure};
    use crate::{Error, Limits};

    /// Returns a world of `size` elements with `p` holding of `holds`, `f`
    /// given by `table`, and these arrows.
    fn world(size: u32, holds: &[u32], table: Vec<u32>, arrows: Vec<u32>) -> KripkeWorld {
        let predicates = if holds.is_empty() {
            Vec::new()
        } else {
            vec![("p".to_owned(), holds.iter().map(|&e| vec![e]).collect())]
        };
        KripkeWorld::new(
            size,
            vec![("f".to_owned(), table)],
            predicates,
            false,
            arrows,
        )
    }

    /// Two worlds, the upper one with an element more, `p` holding of both
    /// elements there and of none below, `f` the identity: it refutes
    /// excluded middle and double negation elimination; and a finite
    /// structure read as a model of one world says what the structure says.
    #[test]
    fn a_model_with_a_growing_domain_refutes() {
        let model = KripkeStructure::new(
            vec![
                world(1, &[], vec![0], vec![1]),
                world(2, &[0, 1], vec![0, 1], Vec::new()),
            ],
            vec![("a".to_owned(), 0)],
            0,
        );
        let refutes = |text: &str| {
            let sequent: Sequent = text.parse().unwrap();
            KripkeCountermodel::new(sequent, model.clone()).check(
                Logic::Intuitionistic,
                &Limits::default(),
                |_| false,
            )
        };
        refutes("|- p(a) \\/ ~p(a)").unwrap();
        refutes("|- p(f(a)) \\/ ~p(f(a))").unwrap();
        refutes("~~p(a) |- p(a)").unwrap();
        // `∀x. p(x)` fails at the root, where `p` holds of nothing, and the
        // world above forces it, so the root forces `¬¬∀x. p(x)`.
        refutes("~~forall x. p(x) |- forall x. p(x)").unwrap();
        assert!(refutes("|- ~~(p(a) \\/ ~p(a))").is_err());
        // One world: as the finite structure.
        let structure = Structure::new(
            2,
            vec![("a".to_owned(), 1)],
            vec![("p".to_owned(), vec![vec![0]])],
        )
        .with_functions(vec![("f".to_owned(), vec![1, 0])]);
        for text in [
            "p(f(a)) |- p(a)",
            "|- exists x. p(f(x))",
            "forall x. p(x) |-",
        ] {
            let sequent: Sequent = text.parse().unwrap();
            let limits = Limits::default();
            assert_eq!(
                KripkeStructure::from(structure.clone())
                    .refutes(&sequent, Logic::Classical, &limits, |_| false)
                    .unwrap(),
                structure.refutes(&sequent, &limits, |_| false).unwrap(),
                "{text}"
            );
        }
        assert!(matches!(
            model.refutes(
                &"|- p(a)".parse().unwrap(),
                Logic::Classical,
                &Limits::default(),
                |_| false
            ),
            Err(Error::Structure { .. })
        ));
        assert_eq!(
            model
                .refutes(
                    &"|- p(a)".parse().unwrap(),
                    Logic::Intuitionistic,
                    &Limits::default(),
                    |_| false
                )
                .unwrap(),
            Evaluation::Refutes
        );
    }

    /// A model that is no Kripke model is refused, each rule by its words.
    /// The passes over the order count against the work bound: a chain's
    /// closure takes more steps than a bound of one.
    #[test]
    fn the_closure_keeps_to_the_work_bound() {
        let sequent: Sequent = "|- p(a) \\/ ~p(a)".parse().unwrap();
        let model = KripkeStructure::new(
            vec![
                world(1, &[], Vec::new(), vec![1]),
                world(1, &[], Vec::new(), vec![2]),
                world(1, &[0], Vec::new(), Vec::new()),
            ],
            vec![("a".to_owned(), 0)],
            0,
        );
        let limits = Limits::default().with_work(Some(1));
        assert!(matches!(
            model.refutes(&sequent, Logic::Intuitionistic, &limits, |_| false),
            Err(Error::Refused(crate::Refusal::Work { limit: 1 }))
        ));
        assert!(
            model
                .refutes(&sequent, Logic::Intuitionistic, &Limits::default(), |_| {
                    false
                })
                .is_ok()
        );
    }

    /// Every world reads `=` as identity: a model with no facts does not
    /// refute `⊢ a = a` (it read `=` as a predicate and did), a world that
    /// lists `=` is refused, and a model whose domain grows refutes what
    /// identity leaves false.
    #[test]
    fn equality_is_identity_at_every_world() {
        let check = |text: &str, model: &KripkeStructure| {
            let sequent: Sequent = text.parse().unwrap();
            KripkeCountermodel::new(sequent, model.clone()).check(
                Logic::Intuitionistic,
                &Limits::default(),
                |_| false,
            )
        };
        let one = KripkeStructure::new(
            vec![KripkeWorld::new(
                1,
                Vec::new(),
                Vec::new(),
                false,
                Vec::new(),
            )],
            vec![("a".to_owned(), 0)],
            0,
        );
        let error = check("|- a = a", &one).unwrap_err();
        assert!(
            matches!(&error, Error::Structure { reason } if reason.contains("forces the formula right of ⊢ a = a")),
            "{error}"
        );
        let mut listed = one.clone();
        listed.worlds[0].predicates = vec![("=".to_owned(), vec![vec![0, 0]])];
        let error = check("|- a = a", &listed).unwrap_err();
        assert!(
            matches!(&error, Error::Structure { reason } if reason.contains("lists equality at world 0")),
            "{error}"
        );
        // At the root only `a` is there; the world above adds an element.
        let growing = KripkeStructure::new(
            vec![
                world(1, &[], Vec::new(), vec![1]),
                world(2, &[], Vec::new(), Vec::new()),
            ],
            vec![("a".to_owned(), 0)],
            0,
        );
        check("|- exists x. ~x = a", &growing).unwrap();
        check("|- forall x. x = a", &growing).unwrap();
        assert!(check("|- forall x. x = x", &growing).is_err());
        assert!(check("|- exists x. x = a", &growing).is_err());
    }

    #[test]
    fn a_model_that_breaks_a_rule_is_refused() {
        let sequent: Sequent = "|- p(f(a)) \\/ ~p(f(a))".parse().unwrap();
        let good = || {
            KripkeStructure::new(
                vec![
                    world(1, &[], vec![0], vec![1]),
                    world(2, &[0, 1], vec![0, 1], Vec::new()),
                ],
                vec![("a".to_owned(), 0)],
                0,
            )
        };
        let mut cases: Vec<(KripkeStructure, &str)> = Vec::new();
        let mut shrinks = good();
        shrinks.worlds[0].size = 3;
        shrinks.worlds[0].functions = vec![("f".to_owned(), vec![0, 0, 0])];
        cases.push((shrinks, "fewer elements"));
        let mut lost = good();
        lost.worlds[0].predicates = vec![("p".to_owned(), vec![vec![0]])];
        lost.worlds[1].predicates = vec![("p".to_owned(), vec![vec![1]])];
        cases.push((lost, "fewer tuples"));
        let mut disagrees = good();
        disagrees.worlds[1].functions = vec![("f".to_owned(), vec![1, 1])];
        cases.push((disagrees, "another value"));
        let mut falsity = good();
        falsity.worlds[1].falsity = true;
        cases.push((falsity, "only minimal logic"));
        let mut cycle = good();
        cycle.worlds[1].arrows = vec![0];
        cases.push((cycle, "cycle"));
        let mut outside = good();
        outside.constants = vec![("a".to_owned(), 1)];
        cases.push((outside, "domain lacks"));
        let mut table = good();
        table.worlds[1].functions = vec![("f".to_owned(), vec![0])];
        cases.push((table, "not one value"));
        for (model, words) in cases {
            let error = KripkeCountermodel::new(sequent.clone(), model)
                .check(Logic::Intuitionistic, &Limits::default(), |_| false)
                .unwrap_err();
            assert!(
                matches!(&error, Error::Structure { reason } if reason.contains(words)),
                "{words}: {error}"
            );
        }
    }
}
