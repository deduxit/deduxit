// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A finite structure for a first-order sequent: a domain of elements,
//! an element for every constant, and the tuples each predicate holds
//! of. One that makes every hypothesis true and every formula right of
//! `⊢` false shows the sequent not valid in classical logic, and, as a
//! Kripke model of one world, in intuitionistic and minimal logic too.
//!
//! [`Structure::refutes`] evaluates the sequent in it: a quantifier
//! ranges over the whole domain, so the cost is the formula's size times
//! the domain's size to the power of the quantifiers nested. The walk
//! keeps a stack of its own.

use super::{Logic, Node, NodeId, Sequent};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal};

/// A finite structure for the language of a first-order sequent.
///
/// # JSON
///
/// With the feature `serialize` it is `{"size": 2, "constants": [["a",
/// 0]], "functions": [["f", [1, 0]]], "predicates": [["p", [[0], [1]]],
/// ["q", []]]}`, inside an ordinary outcome under the key `structure`;
/// `functions` is left out where there is none.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(deny_unknown_fields)
)]
pub struct Structure {
    /// The number of elements, at least 1: the domain is `0 .. size`.
    pub size: u32,
    /// Each constant of the sequent, by name, with the element it denotes.
    pub constants: Vec<(String, u32)>,
    /// Each function symbol of one argument or more, by name, with its
    /// table: the values of its `size^arity` argument tuples in
    /// lexicographic order (`f(d₀, …, dₖ₋₁)` at `Σ dᵢ·size^(k−1−i)`).
    #[cfg_attr(
        feature = "serialize",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub functions: Vec<(String, Vec<u32>)>,
    /// Each predicate, by name, with the tuples of elements it holds of. A
    /// predicate left out holds of none; a nullary one holds where its list
    /// has the empty tuple.
    pub predicates: Vec<(String, Vec<Vec<u32>>)>,
}

/// What a structure makes of a sequent: whether it refutes it, else the
/// first formula it gets wrong.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evaluation {
    /// Every hypothesis is true and every formula right of `⊢` false.
    Refutes,
    /// The hypothesis at this position is false.
    FalseHypothesis(usize),
    /// The formula at this position right of `⊢` is true.
    TrueGoal(usize),
}

impl Structure {
    /// The most formula evaluations [`refutes`](Self::refutes) makes where
    /// `limits.work` sets no bound: 2²⁶.
    pub const WORK: u64 = 1 << 26;

    /// Returns the structure of `size` elements with these constants and
    /// predicates, unchecked.
    pub const fn new(
        size: u32,
        constants: Vec<(String, u32)>,
        predicates: Vec<(String, Vec<Vec<u32>>)>,
    ) -> Self {
        Self {
            size,
            constants,
            functions: Vec::new(),
            predicates,
        }
    }

    /// Returns the structure with these function tables, each by name with
    /// the values of its argument tuples in lexicographic order.
    #[must_use]
    pub fn with_functions(mut self, functions: Vec<(String, Vec<u32>)>) -> Self {
        self.functions = functions;
        self
    }

    /// Evaluates `sequent` in the structure, its hypotheses first, and says
    /// whether the structure refutes it. Asks `stop` every 4 096
    /// evaluations.
    ///
    /// # Errors
    ///
    /// [`Error::Structure`] for a structure that cannot interpret the
    /// sequent: an empty domain, an element out of it, a constant with no
    /// element, a function symbol without a table of one value in the
    /// domain for each argument tuple, or a term no quantifier
    /// binds and the sequent does not name (an eigenvariable of a
    /// derivation's arena); [`Refusal::Work`] past `limits.work`
    /// evaluations ([`WORK`](Self::WORK) where it sets none), and
    /// [`Refusal::Stopped`] when the stop fires.
    pub fn refutes(
        &self,
        sequent: &Sequent,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Evaluation, Error> {
        self.evaluate(sequent, limits, &mut stop)
    }

    /// [`refutes`](Self::refutes) with the stop as a trait object.
    fn evaluate(
        &self,
        sequent: &Sequent,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Evaluation, Error> {
        let wrong = |reason: String| Error::Structure {
            reason: format!("it {reason}"),
        };
        if self.size == 0 {
            return Err(wrong("has no element".to_owned()));
        }
        // What the structure and the evaluator's tables hold: a structure
        // read from JSON is many times its text (an empty tuple is two
        // bytes there and a vector here).
        let memory = limits.memory_bytes.unwrap_or(u64::MAX);
        let tables = self.functions.iter().fold(0u64, |held, (_, table)| {
            held.saturating_add(ENTRY_BYTES)
                .saturating_add((table.len() as u64).saturating_mul(4))
        });
        let held = self.predicates.iter().fold(
            (self.constants.len() as u64)
                .saturating_mul(ENTRY_BYTES)
                .saturating_add(tables),
            |held, (_, tuples)| {
                tuples.iter().fold(held, |held, tuple| {
                    held.saturating_add(ENTRY_BYTES)
                        .saturating_add((tuple.len() as u64).saturating_mul(4))
                })
            },
        );
        if held > memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Check,
                limit_bytes: memory,
                needed_bytes: Some(held),
            }));
        }
        let mut elements: HashMap<&str, u32> = HashMap::default();
        for (name, e) in &self.constants {
            if *e >= self.size {
                return Err(wrong(format!(
                    "names element {e} for {name}, past its domain"
                )));
            }
            elements.insert(name.as_str(), *e);
        }
        let mut holds: HashMap<&str, HashSet<&[u32]>> = HashMap::default();
        for (name, tuples) in &self.predicates {
            let set = holds.entry(name.as_str()).or_default();
            for tuple in tuples {
                if tuple.iter().any(|&e| e >= self.size) {
                    return Err(wrong(format!("puts an element past its domain in {name}")));
                }
                set.insert(tuple.as_slice());
            }
        }
        let mut functions: HashMap<&str, &[u32]> = HashMap::default();
        for (name, table) in &self.functions {
            if table.iter().any(|&e| e >= self.size) {
                return Err(wrong(format!("gives {name} a value past its domain")));
            }
            functions.insert(name.as_str(), table.as_slice());
        }
        // Every term of the sequent's formulas is interpreted, whether the
        // evaluation reaches it or not; a term of the arena no formula uses
        // is no part of the sequent.
        let formulas = sequent.formulas();
        let tables = formulas.first_order();
        let mut seen = vec![false; formulas.len()];
        let mut stack: Vec<NodeId> = sequent
            .left()
            .iter()
            .chain(sequent.right())
            .copied()
            .collect();
        let mut terms: Vec<FoTermId> = Vec::new();
        while let Some(n) = stack.pop() {
            if std::mem::replace(&mut seen[n.index()], true) {
                continue;
            }
            match formulas.node(n) {
                Node::Atom(a) => {
                    if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                        terms.extend_from_slice(tables.list(args));
                    }
                }
                Node::True | Node::False => {}
                Node::Not(a) | Node::Forall(a) | Node::Exists(a) => stack.push(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    stack.push(a);
                    stack.push(b);
                }
            }
        }
        // The terms and their subterms, each once.
        let mut met = vec![false; tables.terms.len()];
        while let Some(t) = terms.pop() {
            if std::mem::replace(&mut met[t.index()], true) {
                continue;
            }
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
                    let cells = u32::try_from(args.len())
                        .ok()
                        .and_then(|k| u64::from(self.size).checked_pow(k));
                    match functions.get(name) {
                        Some(table) if cells == Some(table.len() as u64) => {}
                        Some(_) => {
                            return Err(wrong(format!(
                                "gives {name} a table that is not one value for each of its \
                                 argument tuples"
                            )));
                        }
                        None => {
                            return Err(wrong(format!("has no table for the function {name}")));
                        }
                    }
                    terms.extend_from_slice(args);
                }
                FoTerm::Bound(_) => {}
                FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                    return Err(wrong(
                        "cannot interpret an eigenvariable or an open witness".to_owned(),
                    ));
                }
            }
        }
        let mut eval = Evaluator {
            sequent,
            size: self.size,
            elements,
            functions,
            holds,
            work: 0,
            most: limits.work.unwrap_or(Self::WORK),
            stop,
            wrong: &wrong,
        };
        for (at, &f) in sequent.left().iter().enumerate() {
            if !eval.value(f)? {
                return Ok(Evaluation::FalseHypothesis(at));
            }
        }
        for (at, &f) in sequent.right().iter().enumerate() {
            if eval.value(f)? {
                return Ok(Evaluation::TrueGoal(at));
            }
        }
        Ok(Evaluation::Refutes)
    }
}

/// The bytes a constant or a tuple holds in a structure and in the
/// evaluator's tables beside 4 bytes an element: its vector and its entry
/// in a set.
const ENTRY_BYTES: u64 = 48;

/// A step of the evaluation.
enum Step {
    /// Evaluate the formula.
    Eval(NodeId),
    /// Apply the connective to the values on top of the stack.
    Apply(Node),
    /// The first operand of `∧`, `∨` or `→` was evaluated (its value on top
    /// of the stack): it decides the formula, or the second operand's value
    /// is the formula's.
    Second(Node, NodeId),
    /// The quantifier's body was evaluated for the elements before `next`
    /// (its value on top of the stack, but before the first), and `acc` is
    /// what they gave.
    Quantifier {
        /// Whether it is `∀`.
        forall: bool,
        /// The body.
        body: NodeId,
        /// The next element.
        next: u32,
        /// The value so far.
        acc: bool,
    },
}

/// The state of an evaluation.
struct Evaluator<'a, 's> {
    /// The sequent.
    sequent: &'a Sequent,
    /// The domain's size.
    size: u32,
    /// Each constant's element.
    elements: HashMap<&'a str, u32>,
    /// Each function's table, checked up front to hold `size^arity` values.
    functions: HashMap<&'a str, &'a [u32]>,
    /// Each predicate's tuples.
    holds: HashMap<&'a str, HashSet<&'a [u32]>>,
    /// The evaluations made.
    work: u64,
    /// The most evaluations allowed.
    most: u64,
    /// The caller's stop.
    stop: &'s mut dyn FnMut(Progress) -> bool,
    /// Makes the error of a structure that cannot interpret the sequent.
    wrong: &'s dyn Fn(String) -> Error,
}

impl Evaluator<'_, '_> {
    /// Returns the value of the closed formula `root`.
    fn value(&mut self, root: NodeId) -> Result<bool, Error> {
        let formulas = self.sequent.formulas();
        let mut env: Vec<u32> = Vec::new();
        let mut values: Vec<bool> = Vec::new();
        let mut stack = vec![Step::Eval(root)];
        while let Some(step) = stack.pop() {
            match step {
                Step::Eval(n) => {
                    self.tick()?;
                    let node = formulas.node(n);
                    match node {
                        Node::Atom(a) => values.push(self.atom(a, &env)?),
                        Node::True => values.push(true),
                        Node::False => values.push(false),
                        Node::Not(a) => {
                            stack.push(Step::Apply(node));
                            stack.push(Step::Eval(a));
                        }
                        Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) => {
                            stack.push(Step::Second(node, b));
                            stack.push(Step::Eval(a));
                        }
                        Node::Iff(a, b) => {
                            stack.push(Step::Apply(node));
                            stack.push(Step::Eval(b));
                            stack.push(Step::Eval(a));
                        }
                        Node::Forall(body) | Node::Exists(body) => {
                            let forall = matches!(node, Node::Forall(_));
                            stack.push(Step::Quantifier {
                                forall,
                                body,
                                next: 0,
                                acc: forall,
                            });
                        }
                    }
                }
                Step::Apply(node) => {
                    let b = values.pop().expect("an operand's value");
                    let value = match node {
                        Node::Not(_) => !b,
                        Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {
                            let a = values.pop().expect("an operand's value");
                            match node {
                                Node::And(..) => a && b,
                                Node::Or(..) => a || b,
                                Node::Implies(..) => !a || b,
                                Node::Iff(..) => a == b,
                                Node::Atom(_)
                                | Node::True
                                | Node::False
                                | Node::Not(_)
                                | Node::Forall(_)
                                | Node::Exists(_) => unreachable!("a binary connective"),
                            }
                        }
                        Node::Atom(_)
                        | Node::True
                        | Node::False
                        | Node::Forall(_)
                        | Node::Exists(_) => {
                            unreachable!("only connectives are applied")
                        }
                    };
                    values.push(value);
                }
                Step::Second(node, b) => {
                    let a = values.pop().expect("an operand's value");
                    let decided = match node {
                        Node::And(..) => (!a).then_some(false),
                        Node::Or(..) => a.then_some(true),
                        Node::Implies(..) => (!a).then_some(true),
                        Node::Atom(_)
                        | Node::True
                        | Node::False
                        | Node::Not(_)
                        | Node::Iff(..)
                        | Node::Forall(_)
                        | Node::Exists(_) => unreachable!("∧, ∨ or →"),
                    };
                    match decided {
                        Some(value) => values.push(value),
                        None => stack.push(Step::Eval(b)),
                    }
                }
                Step::Quantifier {
                    forall,
                    body,
                    next,
                    mut acc,
                } => {
                    if next > 0 {
                        let v = values.pop().expect("the body's value");
                        env.pop();
                        acc = if forall { acc && v } else { acc || v };
                    }
                    // An element that decides the quantifier ends it.
                    if next == self.size || acc != forall {
                        values.push(acc);
                        continue;
                    }
                    env.push(next);
                    stack.push(Step::Quantifier {
                        forall,
                        body,
                        next: next + 1,
                        acc,
                    });
                    stack.push(Step::Eval(body));
                }
            }
        }
        Ok(values.pop().expect("the formula's value"))
    }

    /// Counts an evaluation, refusing past the bound and asking the stop.
    fn tick(&mut self) -> Result<(), Error> {
        self.work += 1;
        if self.work > self.most {
            return Err(Error::Refused(Refusal::Work { limit: self.most }));
        }
        if self.work.is_multiple_of(4_096)
            && (self.stop)(Progress::new(Phase::Check, 4_096, self.work))
        {
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::Check,
            }));
        }
        Ok(())
    }

    /// Returns the value of the atom `a` under the bound elements `env`,
    /// the innermost last.
    fn atom(&mut self, a: u32, env: &[u32]) -> Result<bool, Error> {
        let formulas = self.sequent.formulas();
        let tables = formulas.first_order();
        let (name, args): (&str, &[FoTermId]) = match tables.atoms.get(a as usize) {
            Some(&(symbol, args)) => (tables.symbols[symbol.index()].0.as_str(), tables.list(args)),
            None => (formulas.atom_name(a), &[]),
        };
        let mut tuple = Vec::with_capacity(args.len());
        for &t in args {
            tuple.push(self.term(t, env)?);
        }
        Ok(self
            .holds
            .get(name)
            .is_some_and(|set| set.contains(tuple.as_slice())))
    }

    /// Returns the element the term `t` denotes under `env`, the innermost
    /// binding last. A term is evaluated with a stack of its own: terms
    /// may nest as deep as a document is long. Each application of a
    /// function symbol is an evaluation ([`tick`](Self::tick)): a term
    /// read from JSON may share its subterms, and is read as the tree it
    /// stands for.
    fn term(&mut self, t: FoTermId, env: &[u32]) -> Result<u32, Error> {
        let sequent = self.sequent;
        let tables = sequent.formulas().first_order();
        let mut values: Vec<u32> = Vec::new();
        // Each entry: a term, and whether its arguments' values are on
        // `values` already.
        let mut stack = vec![(t, false)];
        while let Some((t, applied)) = stack.pop() {
            match tables.terms[t.index()] {
                FoTerm::Bound(index) => {
                    let at = env.len().checked_sub(index as usize + 1).ok_or_else(|| {
                        (self.wrong)("meets a variable no quantifier binds".to_owned())
                    })?;
                    values.push(env[at]);
                }
                FoTerm::App(f, args) => {
                    let name = tables.symbols[f.index()].0.as_str();
                    let args = tables.list(args);
                    if args.is_empty() {
                        let e = self.elements.get(name).copied().ok_or_else(|| {
                            (self.wrong)(format!("names no element for the constant {name}"))
                        })?;
                        values.push(e);
                    } else if applied {
                        self.tick()?;
                        let table = self.functions.get(name).ok_or_else(|| {
                            (self.wrong)(format!("has no table for the function {name}"))
                        })?;
                        let start = values.len() - args.len();
                        let index = values
                            .drain(start..)
                            .fold(0usize, |i, e| i * self.size as usize + e as usize);
                        let e = table.get(index).copied().ok_or_else(|| {
                            (self.wrong)(format!("gives {name} too short a table"))
                        })?;
                        values.push(e);
                    } else {
                        stack.push((t, true));
                        for &a in args.iter().rev() {
                            stack.push((a, false));
                        }
                    }
                }
                FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                    return Err((self.wrong)(
                        "cannot interpret an eigenvariable or an open witness".to_owned(),
                    ));
                }
            }
        }
        Ok(values.pop().expect("the term's value"))
    }
}

/// A finite structure said to refute a first-order sequent: what grounding
/// answers for a sequent that is not valid.
///
/// # JSON
///
/// With the feature `serialize` it is `{"sequent": …, "structure": …}`,
/// the ordinary sequent's form and the [`Structure`]'s, inside an ordinary
/// outcome under the key `structure`.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct FiniteCountermodel {
    /// The sequent refuted.
    pub sequent: Sequent,
    /// The structure. It makes every formula left of the sequent's `⊢`
    /// true and every one right of it false, as
    /// [`check`](Self::check) checks.
    pub structure: Structure,
}

impl FiniteCountermodel {
    /// Returns the countermodel of this sequent and structure, unchecked.
    pub const fn new(sequent: Sequent, structure: Structure) -> Self {
        Self { sequent, structure }
    }

    /// Checks that the structure refutes the sequent in `logic`, asking
    /// `stop` as [`Structure::refutes`] does. A structure that refutes a
    /// sequent classically refutes it in every logic, since what is valid
    /// in minimal or intuitionistic logic is valid classically.
    ///
    /// # Errors
    ///
    /// [`Error::Succedents`] for more than one formula right of `⊢`
    /// outside classical logic; [`Error::Structure`] for a structure that
    /// cannot interpret the sequent or does not refute it, naming the
    /// first formula it gets wrong; the refusals of
    /// [`Structure::refutes`].
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
        let (side, at) = match self.structure.evaluate(&self.sequent, limits, &mut stop)? {
            Evaluation::Refutes => return Ok(()),
            Evaluation::FalseHypothesis(at) => ("hypothesis", at),
            Evaluation::TrueGoal(at) => ("formula right of ⊢", at),
        };
        let formula = if side == "hypothesis" {
            self.sequent.left()[at]
        } else {
            self.sequent.right()[at]
        };
        let written = self.sequent.formulas().formula(formula);
        let value = if side == "hypothesis" {
            "false"
        } else {
            "true"
        };
        Err(Error::Structure {
            reason: format!("it makes the {side} {written} {value}"),
        })
    }
}

/// The most entries of a function's table or tuples of a predicate a
/// structure's text shows, the rest counted: a table holds `size^arity`
/// values, and its text would be some fifty times its bytes.
const SHOWN: usize = 64;

impl std::fmt::Display for Structure {
    /// Writes the structure in lines: the domain (`domain: {0, 1}`, past
    /// eight elements `{0, …, 41}`), each constant's element (`a = 0`),
    /// each function's table (`f: 0 ↦ 1, 1 ↦ 0`, `g: (0, 1) ↦ 0, …`, or
    /// `f: always 0` where every value is one element), and each predicate
    /// as the set it holds of (`p: {0}`, `r: {(0, 1)}`; a nullary one `q:
    /// true`), a predicate that holds of nothing left out. Past 64 entries
    /// of a table or tuples of a predicate the rest are counted (`, … (448
    /// more)`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.size {
            0 => write!(f, "domain: {{}}")?,
            1..=8 => {
                let items: Vec<String> = (0..self.size).map(|e| e.to_string()).collect();
                write!(f, "domain: {{{}}}", items.join(", "))?;
            }
            n => write!(f, "domain: {{0, …, {}}}", n - 1)?,
        }
        for (name, e) in &self.constants {
            write!(f, "\n{name} = {e}")?;
        }
        for (name, table) in &self.functions {
            if let Some(&first) = table.first()
                && table.iter().all(|&v| v == first)
            {
                write!(f, "\n{name}: always {first}")?;
                continue;
            }
            // The arity: the tuples are `size^arity`, and a table of two
            // values or more needs two elements or more.
            let n = u64::from(self.size.max(2));
            let mut arity = 0u32;
            let mut cells = 1u64;
            while cells < table.len() as u64 {
                cells = cells.saturating_mul(n);
                arity += 1;
            }
            write!(f, "\n{name}: ")?;
            for (i, v) in table.iter().enumerate().take(SHOWN) {
                let mut tuple = Vec::with_capacity(arity as usize);
                let mut rest = i as u64;
                for _ in 0..arity {
                    tuple.push((rest % n).to_string());
                    rest /= n;
                }
                tuple.reverse();
                let comma = if i == 0 { "" } else { ", " };
                match tuple.len() {
                    1 => write!(f, "{comma}{} ↦ {v}", tuple[0])?,
                    _ => write!(f, "{comma}({}) ↦ {v}", tuple.join(", "))?,
                }
            }
            if table.len() > SHOWN {
                write!(f, ", … ({} more)", table.len() - SHOWN)?;
            }
        }
        for (name, tuples) in &self.predicates {
            if tuples.is_empty() {
                continue;
            }
            if tuples.iter().all(Vec::is_empty) {
                write!(f, "\n{name}: true")?;
                continue;
            }
            let items: Vec<String> = tuples
                .iter()
                .take(SHOWN)
                .map(|tuple| {
                    let elements: Vec<String> = tuple.iter().map(u32::to_string).collect();
                    match elements.len() {
                        1 => elements.join(""),
                        _ => format!("({})", elements.join(", ")),
                    }
                })
                .collect();
            let more = match tuples.len().saturating_sub(SHOWN) {
                0 => String::new(),
                k => format!(", … ({k} more)"),
            };
            write!(f, "\n{name}: {{{}{more}}}", items.join(", "))?;
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::{Evaluation, FiniteCountermodel, Structure};
    use crate::ordinary::{Logic, Sequent};
    use crate::{Error, Limits};

    /// Evaluates `text` in `structure` within the default limits.
    fn evaluate(structure: &Structure, text: &str) -> Result<Evaluation, Error> {
        let sequent: Sequent = text.parse().unwrap();
        structure.refutes(&sequent, &Limits::default(), |_| false)
    }

    /// A structure with function tables evaluates terms through them:
    /// `f` swaps the two elements, `g` is the first projection.
    #[test]
    fn function_tables_interpret_terms() {
        let structure = Structure::new(
            2,
            vec![("a".to_owned(), 0)],
            vec![("p".to_owned(), vec![vec![0]])],
        )
        .with_functions(vec![
            ("f".to_owned(), vec![1, 0]),
            ("g".to_owned(), vec![0, 0, 1, 1]),
        ]);
        let cases = [
            ("p(a) |- p(f(a))", Evaluation::Refutes),
            ("p(a) |- p(f(f(a)))", Evaluation::TrueGoal(0)),
            ("forall x. p(g(a, x)) |- p(f(a))", Evaluation::Refutes),
            ("|- exists x. p(f(x))", Evaluation::TrueGoal(0)),
            ("exists x. p(g(x, f(x))) |-", Evaluation::Refutes),
            ("p(f(a)) |-", Evaluation::FalseHypothesis(0)),
        ];
        for (text, expected) in cases {
            assert_eq!(evaluate(&structure, text).unwrap(), expected, "{text}");
        }
        let sequent: Sequent = "p(a) |- p(f(a))".parse().unwrap();
        FiniteCountermodel::new(sequent, structure.clone())
            .check(Logic::Intuitionistic, &Limits::default(), |_| false)
            .unwrap();
        assert_eq!(
            structure.to_string(),
            "domain: {0, 1}\na = 0\nf: 0 ↦ 1, 1 ↦ 0\ng: (0, 0) ↦ 0, (0, 1) ↦ 0, (1, 0) ↦ 1, (1, 1) ↦ 1\np: {0}"
        );
    }

    /// A table's text shows its first 64 entries and counts the rest, and
    /// a term's applications are evaluations the work bound counts.
    #[test]
    fn a_large_table_is_shown_in_part_and_its_terms_are_work() {
        let table: Vec<u32> = (0..128).map(|i| i % 2).collect();
        let structure = Structure::new(2, vec![("a".to_owned(), 0)], Vec::new())
            .with_functions(vec![("g".to_owned(), table)]);
        let text = structure.to_string();
        assert!(text.ends_with(", … (64 more)"), "{text}");
        assert_eq!(text.matches('↦').count(), 64);
        let sequent: Sequent = "|- p(g(a, a, a, a, a, a, a))".parse().unwrap();
        let limits = Limits {
            work: Some(1),
            ..Limits::default()
        };
        assert!(matches!(
            structure.refutes(&sequent, &limits, |_| false),
            Err(Error::Refused(crate::Refusal::Work { .. }))
        ));
    }

    /// A table of the wrong length, a value past the domain and a missing
    /// table are each refused, even on a branch the evaluation skips.
    #[test]
    fn a_table_that_does_not_fit_is_refused() {
        let base = || Structure::new(2, vec![("a".to_owned(), 0)], Vec::new());
        let cases = [
            (
                base().with_functions(vec![("f".to_owned(), vec![1])]),
                "table",
            ),
            (
                base().with_functions(vec![("f".to_owned(), vec![0, 2])]),
                "domain",
            ),
            (base(), "no table"),
        ];
        for (structure, words) in cases {
            let error = evaluate(&structure, "|- true \\/ p(f(a))").unwrap_err();
            assert!(
                matches!(&error, Error::Structure { reason } if reason.contains(words)),
                "{words}: {error}"
            );
        }
    }
}
