// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Resolution and DPLL on clauses of classical logic, at teaching scale,
//! every refutation checked.
//!
//! A clause is a disjunction of literals, atoms and negated atoms; a set
//! of clauses is satisfiable when one assignment makes every clause true.
//! A sequent `Γ ⊢ Δ` is valid exactly when the clauses of its negation (`Γ`
//! with each formula of `Δ` negated) are unsatisfiable. [`Clauses`] reads
//! a set from DIMACS text ([`Clauses::from_dimacs`]) or takes the clauses of
//! a sequent ([`Clauses::of_sequent`], through a normal form of
//! [`normal`](super::normal)).
//!
//! The resolution rule takes two clauses, one with an atom `p` and one
//! with `¬p`, and derives their resolvent: the first without `p` and the
//! second without `¬p`, joined. A
//! [`Refutation`] derives the empty clause `□` from a set by resolution
//! steps, a graph of resolvents over the set. It is built by hand one step
//! at a time ([`Refutation::resolve`]) or found by [`decide`], and
//! [`Refutation::check`] checks every step again.
//!
//! [`decide`] answers whether a set is satisfiable by one of two
//! [`Method`]s, within the bounds of its [`Options`]:
//! - **saturation**: resolve the shortest clause not yet used with every
//!   clause used, leaving out tautologies and clauses that a clause
//!   already kept contains, until the empty clause appears (a refutation)
//!   or nothing new does (the set is satisfiable);
//! - **DPLL**: assign the literals of unit clauses, then pure literals,
//!   then decide an atom, backtracking at a conflict; every step is an
//!   [`Event`] of the trace. A model ends it, or the refutation read off
//!   the trace.
//!
//! Both answers are checked before they are returned: a refutation by
//! [`Refutation::check`], a model by evaluating every clause. A check
//! that fails is [`Error::ClaimFails`], a defect.
//!
//! With the cargo feature `sat`, [`check_proof`] checks a refutation in
//! the DRAT or LRAT format that an outside SAT solver wrote, by the
//! checker of the crate's own SAT solver.
//!
#![cfg_attr(feature = "parse", doc = "```")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::ordinary::resolution::{self, Clauses, Options, Verdict};
//!
//! let clauses = Clauses::from_dimacs("p cnf 2 3\n1 2 0\n-1 0\n-2 0\n", &deduxit::Limits::default())?;
//! let answer = resolution::decide(&clauses, &Options::default(), &deduxit::Limits::default(), |_| false)?;
//! let Verdict::Refuted(refutation) = &answer.verdict else { panic!("unsatisfiable") };
//! assert_eq!(refutation.resolvents().len(), 2);
//! # Ok::<(), deduxit::Error>(())
//! ```

use super::{Formulas, Node, NodeId, Sequent};
use crate::Error;
use crate::limits::{Limits, Progress, Refusal, Space};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// The DPLL procedure and its trace.
mod dpll;
/// DRAT and LRAT proofs of outside solvers, checked.
#[cfg(feature = "sat")]
mod outside;
/// Refutations: the graph of resolvents, built by hand and checked.
mod refutation;
/// The search by saturation.
mod saturate;
#[cfg(all(test, feature = "parse", feature = "serialize"))]
mod tests;

pub use dpll::Event;
#[cfg(feature = "sat")]
pub use outside::{ProofFormat, check_proof};
pub use refutation::{Refutation, Resolvent};

/// A literal: an atom of a clause set, or its negation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal(u32);

impl Literal {
    /// The most atoms a clause set holds: an atom and its negation are two
    /// codes of a `u32`.
    pub const MOST_ATOMS: u32 = u32::MAX / 2;

    /// Returns the literal of the atom given, negated unless `positive`;
    /// `None` for an atom from [`MOST_ATOMS`](Self::MOST_ATOMS) on, which
    /// no clause set holds.
    pub const fn new(atom: u32, positive: bool) -> Option<Self> {
        if atom < Self::MOST_ATOMS {
            Some(Self::of(atom, positive))
        } else {
            None
        }
    }

    /// Returns the literal of an atom of a clause set, which is below
    /// [`MOST_ATOMS`](Self::MOST_ATOMS) since a set holds at most that many
    /// atoms ([`Clauses::new`]), so `2·atom + 1` fits a `u32`.
    pub(crate) const fn of(atom: u32, positive: bool) -> Self {
        Self(2 * atom + if positive { 0 } else { 1 })
    }

    /// Returns the atom.
    pub const fn atom(self) -> u32 {
        self.0 / 2
    }

    /// Returns whether the literal is the atom itself.
    pub const fn is_positive(self) -> bool {
        self.0.is_multiple_of(2)
    }

    /// Returns the negation.
    #[must_use]
    pub const fn negated(self) -> Self {
        Self(self.0 ^ 1)
    }
}

/// A clause: its literals, sorted and each once.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Clause(Box<[Literal]>);

impl Clause {
    /// Returns the clause of the literals given, sorted and each once.
    pub fn new(mut literals: Vec<Literal>) -> Self {
        literals.sort_unstable();
        literals.dedup();
        Self(literals.into_boxed_slice())
    }

    /// Returns the literals, sorted.
    pub fn literals(&self) -> &[Literal] {
        &self.0
    }

    /// Returns whether the clause is empty, `□`, which no assignment makes
    /// true.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns whether the clause holds the literal.
    pub fn contains(&self, literal: Literal) -> bool {
        self.0.binary_search(&literal).is_ok()
    }

    /// Returns whether the clause holds an atom and its negation, so every
    /// assignment makes it true.
    pub fn is_tautology(&self) -> bool {
        // Sorted, a literal and its negation are neighbours.
        self.0.windows(2).any(|w| w[0].negated() == w[1])
    }

    /// Returns whether every literal of `self` is in `other`.
    pub fn subsumes(&self, other: &Self) -> bool {
        let mut rest = other.0.iter();
        self.0.iter().all(|l| rest.any(|m| m == l))
    }
}

/// A set of clauses over named atoms.
///
/// # JSON
///
/// With the feature `serialize` a clause set is written inside a
/// refutation's document as `"atoms": ["a", "b"], "clauses": [[1, -2],
/// [2]]`: the atom names, and each clause as DIMACS writes it, the atom
/// `k` (counted from 1) for itself and `-k` for its negation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clauses {
    /// The atom names, by atom.
    atoms: Vec<String>,
    /// The clauses, in order.
    clauses: Vec<Clause>,
}

impl Clauses {
    /// Returns the set of `clauses` over the atoms named, each clause's
    /// literals sorted and each once.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no atom of the ordinary
    /// syntax, [`Error::InvalidOption`] (key `atoms`) for one given twice;
    /// [`Error::IndexOutOfBounds`] for a literal
    /// of an atom past the names; [`Refusal::Index`] past
    /// [`Literal::MOST_ATOMS`] atoms.
    pub fn new(atoms: Vec<String>, clauses: Vec<Vec<Literal>>) -> Result<Self, Error> {
        if atoms.len() > Literal::MOST_ATOMS as usize {
            return Err(Error::Refused(Refusal::Index {
                what: Space::Atom,
                count: atoms.len() as u64,
                most: u64::from(Literal::MOST_ATOMS),
            }));
        }
        let mut names = Vec::with_capacity(atoms.len());
        for name in atoms {
            let normal = match crate::name::normalized(&name) {
                std::borrow::Cow::Borrowed(_) => name,
                std::borrow::Cow::Owned(normal) => normal,
            };
            crate::name::check_ordinary(&normal)?;
            names.push(normal);
        }
        let mut seen = crate::hash::HashSet::default();
        if let Some(name) = names.iter().find(|&name| !seen.insert(name.as_str())) {
            return Err(Error::InvalidOption {
                key: "atoms",
                message: format!("{name} names two atoms of the clauses"),
            });
        }
        let mut made = Vec::with_capacity(clauses.len());
        for clause in clauses {
            if let Some(l) = clause.iter().find(|l| l.atom() as usize >= names.len()) {
                return Err(Error::IndexOutOfBounds {
                    space: Space::Atom,
                    index: l.atom() as usize,
                    len: names.len(),
                });
            }
            made.push(Clause::new(clause));
        }
        Ok(Self {
            atoms: names,
            clauses: made,
        })
    }

    /// Reads a set of clauses in the DIMACS CNF format: comment lines
    /// starting `c`, the header `p cnf VARIABLES CLAUSES`, then the
    /// clauses as numbers, `k` the variable `k` and `-k` its negation, each
    /// clause ended by `0`. A line `%` ends the input, as SATLIB's files
    /// do. The variable `k` is the atom `xk`.
    ///
    /// # Errors
    ///
    /// [`Error::ClauseFile`] for a text that is no DIMACS CNF, a variable
    /// past the header's or a clause not ended by `0`;
    /// [`Refusal::Occurrences`] for more variables, or literals and clauses
    /// together, than `limits.occurrences`.
    pub fn from_dimacs(text: &str, limits: &Limits) -> Result<Self, Error> {
        let fault = |line: usize, reason: &str| Error::ClauseFile {
            at: line as u64 + 1,
            reason: reason.to_owned(),
        };
        let mut variables: Option<u64> = None;
        let mut clauses: Vec<Vec<Literal>> = Vec::new();
        let mut open: Vec<Literal> = Vec::new();
        let mut size = 0u64;
        let bound = |size: u64| match limits.occurrences {
            Some(limit) if size > limit => Err(Error::Refused(Refusal::Occurrences {
                occurrences: size,
                limit,
            })),
            _ => Ok(()),
        };
        for (n, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('c') {
                continue;
            }
            if line.starts_with('%') {
                break;
            }
            if let Some(header) = line.strip_prefix('p') {
                if variables.is_some() {
                    return Err(fault(n, "a second header"));
                }
                let words: Vec<&str> = header.split_whitespace().collect();
                let (["cnf", v, c] | ["CNF", v, c]) = words[..] else {
                    return Err(fault(n, "the header is not `p cnf VARIABLES CLAUSES`"));
                };
                let (Ok(v), Ok(_)) = (v.parse::<u64>(), c.parse::<u64>()) else {
                    return Err(fault(n, "the header's counts are no numbers"));
                };
                // Each atom's name, about `ATOM_BYTES`, is held at once.
                if let Some(limit_bytes) = limits.memory_bytes
                    && v.saturating_mul(ATOM_BYTES) > limit_bytes
                {
                    return Err(Error::Refused(Refusal::Memory {
                        phase: crate::limits::Phase::Read,
                        limit_bytes,
                        needed_bytes: Some(v.saturating_mul(ATOM_BYTES)),
                    }));
                }
                if v > u64::from(Literal::MOST_ATOMS) {
                    return Err(Error::Refused(Refusal::Index {
                        what: Space::Atom,
                        count: v,
                        most: u64::from(Literal::MOST_ATOMS),
                    }));
                }
                bound(v)?;
                variables = Some(v);
                continue;
            }
            let Some(most) = variables else {
                return Err(fault(n, "a clause before the header `p cnf …`"));
            };
            for word in line.split_whitespace() {
                let number: i64 = word
                    .parse()
                    .map_err(|_| fault(n, &format!("`{word}` is no literal")))?;
                if number == 0 {
                    size = size.saturating_add(1);
                    bound(size)?;
                    clauses.push(std::mem::take(&mut open));
                    continue;
                }
                let atom = number.unsigned_abs();
                if atom > most {
                    return Err(fault(
                        n,
                        &format!("the variable {atom} is past the header's {most}"),
                    ));
                }
                size = size.saturating_add(1);
                bound(size)?;
                // `atom ≤ most ≤ MOST_ATOMS`, checked at the header.
                open.push(Literal::of((atom - 1) as u32, number > 0));
            }
        }
        let Some(most) = variables else {
            return Err(fault(0, "no header `p cnf VARIABLES CLAUSES`"));
        };
        if !open.is_empty() {
            return Err(Error::ClauseFile {
                at: text.lines().count() as u64,
                reason: "the last clause is not ended by 0".to_owned(),
            });
        }
        let atoms = (1..=most).map(|k| format!("x{k}")).collect();
        Self::new(atoms, clauses)
    }

    /// Returns the clauses a sequent holds: its formulas right of `⊢` must
    /// be none, and each hypothesis a clause or a conjunction of clauses,
    /// each taken apart. `⊤` holds anyway and is left out; `⊥` is the
    /// empty clause, as a hypothesis or a disjunct. This is what a normal
    /// form of [`normal`](super::normal) gives: Tseitin's or Plaisted and
    /// Greenbaum's clauses, or a conjunctive normal form of a sequent
    /// whose formulas right of `⊢` were moved left negated.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidOption`] (key `clauses`) for a formula right of `⊢`
    /// or a hypothesis that is no conjunction of clauses.
    pub fn of_sequent(sequent: &Sequent) -> Result<Self, Error> {
        let formulas = sequent.formulas();
        formulas.propositional("resolution")?;
        let not_clauses = |what: String| Error::InvalidOption {
            key: "clauses",
            message: format!(
                "{what}; rewrite the sequent into clauses first, by conjunctive normal form or \
                 Tseitin's encoding"
            ),
        };
        // Each literal's atom must be below `MOST_ATOMS` before it is made.
        if formulas.atom_names().len() > Literal::MOST_ATOMS as usize {
            return Err(Error::Refused(Refusal::Index {
                what: Space::Atom,
                count: formulas.atom_names().len() as u64,
                most: u64::from(Literal::MOST_ATOMS),
            }));
        }
        if let Some(&d) = sequent.right().first() {
            return Err(not_clauses(format!(
                "{} stands right of ⊢",
                formulas.formula(d)
            )));
        }
        let mut clauses = Vec::new();
        let mut parts: Vec<NodeId> = Vec::new();
        for &h in sequent.left() {
            parts.push(h);
            while let Some(id) = parts.pop() {
                match formulas.node(id) {
                    Node::And(a, b) => parts.extend([b, a]),
                    Node::True => {}
                    Node::Atom(_) | Node::False | Node::Not(_) | Node::Or(..) => {
                        if let Some(clause) = clause_of(formulas, id).ok_or_else(|| {
                            not_clauses(format!("{} is no clause", formulas.formula(id)))
                        })? {
                            clauses.push(clause);
                        }
                    }
                    Node::Implies(..) | Node::Iff(..) => {
                        return Err(not_clauses(format!(
                            "{} is no clause",
                            formulas.formula(id)
                        )));
                    }
                    Node::Forall(_) | Node::Exists(_) => {
                        unreachable!("{}", crate::ordinary::REFUSED)
                    }
                }
            }
        }
        Self::new(formulas.atom_names().to_vec(), clauses)
    }

    /// Returns the sequent `C₁, …, Cₙ ⊢` of the clauses, each a disjunction
    /// of its literals and the empty clause `⊥`, valid exactly when the
    /// clauses are unsatisfiable.
    ///
    /// # Errors
    ///
    /// The arena's, [`Refusal::Index`] when it is full.
    pub fn to_sequent(&self) -> Result<Sequent, Error> {
        let mut formulas = Formulas::default();
        let mut atoms = Vec::with_capacity(self.atoms.len());
        for name in &self.atoms {
            atoms.push(formulas.atom(name)?);
        }
        let mut left = Vec::with_capacity(self.clauses.len());
        for clause in &self.clauses {
            let mut any: Option<NodeId> = None;
            for &l in clause.literals() {
                let atom = atoms[l.atom() as usize];
                let lit = if l.is_positive() {
                    atom
                } else {
                    formulas.add(Node::Not(atom))?
                };
                any = Some(match any {
                    None => lit,
                    Some(before) => formulas.add(Node::Or(before, lit))?,
                });
            }
            left.push(match any {
                Some(id) => id,
                None => formulas.add(Node::False)?,
            });
        }
        Sequent::new(formulas, left, Vec::new())
    }

    /// Returns the atom names, by atom.
    pub fn atoms(&self) -> &[String] {
        &self.atoms
    }

    /// Returns the clauses, in order.
    pub fn clauses(&self) -> &[Clause] {
        &self.clauses
    }

    /// Returns a value that writes `clause` with this set's atom names:
    /// `¬a ∨ b`, and `□` for the empty clause.
    pub fn text<'a>(&'a self, clause: &'a Clause) -> ClauseText<'a> {
        ClauseText {
            atoms: &self.atoms,
            clause,
        }
    }

    /// Returns a value that writes `literal` with this set's atom names.
    pub fn literal(&self, literal: Literal) -> LiteralText<'_> {
        LiteralText {
            atoms: &self.atoms,
            literal,
        }
    }

    /// Returns whether the assignment that makes exactly the atoms of
    /// `true_atoms` true satisfies every clause, and the first clause it
    /// makes false otherwise.
    pub fn falsified(&self, true_atoms: &[u32]) -> Option<usize> {
        let mut value = vec![false; self.atoms.len()];
        for &a in true_atoms {
            if let Some(v) = value.get_mut(a as usize) {
                *v = true;
            }
        }
        self.clauses.iter().position(|c| {
            !c.literals()
                .iter()
                .any(|l| value[l.atom() as usize] == l.is_positive())
        })
    }
}

/// The bytes an atom of a clause set is charged: its name and the searches'
/// arrays per atom.
const ATOM_BYTES: u64 = 64;

/// The bytes a clause of a set is charged beside its literals.
const CLAUSE_BYTES: u64 = 48;

/// Returns the clause a formula is, a disjunction of literals: `None` for
/// a formula that is no clause, `Some(None)` for one that holds `⊤` and so
/// is left out.
#[expect(
    clippy::option_option,
    reason = "no clause, a clause left out and a clause are three answers"
)]
fn clause_of(formulas: &Formulas, root: NodeId) -> Option<Option<Vec<Literal>>> {
    let mut literals = Vec::new();
    let mut parts = vec![root];
    let mut holds = false;
    while let Some(id) = parts.pop() {
        match formulas.node(id) {
            Node::Or(a, b) => parts.extend([b, a]),
            Node::Atom(a) => literals.push(Literal::of(a, true)),
            Node::False => {}
            Node::True => holds = true,
            Node::Not(a) => match formulas.node(a) {
                Node::Atom(a) => literals.push(Literal::of(a, false)),
                Node::True => {}
                Node::False => holds = true,
                Node::Not(_) | Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {
                    return None;
                }
                Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            },
            Node::And(..) | Node::Implies(..) | Node::Iff(..) => return None,
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        }
    }
    Some((!holds).then_some(literals))
}

/// A clause written with its set's atom names.
#[derive(Clone, Copy, Debug)]
pub struct ClauseText<'a> {
    /// The atom names.
    atoms: &'a [String],
    /// The clause.
    clause: &'a Clause,
}

impl Display for ClauseText<'_> {
    /// Writes `¬a ∨ b`, or `□` for the empty clause.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.clause.is_empty() {
            return f.write_str("□");
        }
        for (k, &l) in self.clause.literals().iter().enumerate() {
            if k > 0 {
                f.write_str(" ∨ ")?;
            }
            write!(
                f,
                "{}",
                LiteralText {
                    atoms: self.atoms,
                    literal: l
                }
            )?;
        }
        Ok(())
    }
}

/// A literal written with its set's atom names.
#[derive(Clone, Copy, Debug)]
pub struct LiteralText<'a> {
    /// The atom names.
    atoms: &'a [String],
    /// The literal.
    literal: Literal,
}

impl Display for LiteralText<'_> {
    /// Writes `a` or `¬a`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if !self.literal.is_positive() {
            f.write_str("¬")?;
        }
        f.write_str(&self.atoms[self.literal.atom() as usize])
    }
}

/// How [`decide`] searches.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Method {
    /// Saturation by resolution, the shortest clause first.
    #[default]
    Saturation,
    /// DPLL: unit clauses, pure literals, decisions and backtracking.
    Dpll,
}

impl Method {
    /// Every method, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[Self::Saturation, Self::Dpll];

    /// The methods' names.
    pub const NAMES: &'static [&'static str] = &["saturation", "dpll"];

    /// Returns the method's name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Saturation => "saturation",
            Self::Dpll => "dpll",
        }
    }
}

impl std::str::FromStr for Method {
    type Err = Error;

    /// Reads a method's [`name`](Self::name).
    fn from_str(name: &str) -> Result<Self, Error> {
        Self::ALL
            .iter()
            .copied()
            .find(|m| m.name() == name)
            .ok_or_else(|| Error::UnknownName {
                what: "resolution method",
                name: name.into(),
                known: Self::NAMES,
            })
    }
}

/// How [`decide`] searches and how far.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The method, saturation by default.
    pub method: Method,
    /// The most clauses saturation keeps, the input's included, or `None`;
    /// past it the answer is unknown.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub clauses: Option<u64>,
    /// The most steps: resolvents made by saturation, events of DPLL's
    /// trace; past it the answer is unknown.
    #[cfg_attr(feature = "serialize", serde(with = "crate::serialize::exact"))]
    pub steps: Option<u64>,
}

impl Options {
    /// The default bound on clauses: two thousand, more than a page shows.
    pub const DEFAULT_CLAUSES: u64 = 2_000;
    /// The default bound on steps: a hundred thousand, a fraction of a
    /// second.
    pub const DEFAULT_STEPS: u64 = 100_000;

    /// Returns the options with the method given.
    #[must_use]
    pub const fn with_method(self, method: Method) -> Self {
        Self { method, ..self }
    }

    /// Returns the options with another bound on clauses.
    #[must_use]
    pub const fn with_clauses(self, clauses: Option<u64>) -> Self {
        Self { clauses, ..self }
    }

    /// Returns the options with another bound on steps.
    #[must_use]
    pub const fn with_steps(self, steps: Option<u64>) -> Self {
        Self { steps, ..self }
    }
}

impl Default for Options {
    /// Saturation within the default bounds.
    fn default() -> Self {
        Self {
            method: Method::Saturation,
            clauses: Some(Self::DEFAULT_CLAUSES),
            steps: Some(Self::DEFAULT_STEPS),
        }
    }
}

/// What [`decide`] found.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Verdict {
    /// The clauses are unsatisfiable: a checked refutation.
    Refuted(Box<Refutation>),
    /// The clauses are satisfiable: the atoms a checked model makes true,
    /// every other atom false.
    Satisfiable {
        /// The atoms true, ascending.
        true_atoms: Vec<u32>,
    },
    /// A bound ended the search before it decided.
    Unknown(Bound),
}

/// The bound that ended a search.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "kind", rename_all = "snake_case"))]
pub enum Bound {
    /// [`Options::clauses`].
    Clauses {
        /// Its value.
        limit: u64,
    },
    /// [`Options::steps`].
    Steps {
        /// Its value.
        limit: u64,
    },
    /// The caller's stop.
    Stopped,
}

impl Display for Bound {
    /// Writes why the search ended: "the bound of 2000 clauses was reached".
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Clauses { limit } => write!(
                f,
                "the bound of {limit} clause{} was reached",
                if *limit == 1 { "" } else { "s" }
            ),
            Self::Steps { limit } => write!(
                f,
                "the bound of {limit} step{} was reached",
                if *limit == 1 { "" } else { "s" }
            ),
            Self::Stopped => f.write_str("the search was stopped"),
        }
    }
}

/// A clause set decided: the method, the verdict with its evidence, and
/// for DPLL the trace.
///
/// # JSON
///
/// With the feature `serialize` it is the document `{"version": 22,
/// "deduxit": "0.7.0", "method": "dpll", "verdict": "unsatisfiable",
/// "atoms": […], "clauses": […], "resolvents": […], "true_atoms": […],
/// "bound": {…}, "trace": […], "statistics": {…}}` ([`wire`](crate::wire)),
/// written only: `verdict` is `unsatisfiable` with the refutation's
/// `resolvents`, `satisfiable` with `true_atoms` (atom numbers counted from
/// 1), or `unknown` with the `bound`; `trace` the events of DPLL.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Resolution {
    /// The method.
    pub method: Method,
    /// The clauses decided.
    pub clauses: Clauses,
    /// The verdict.
    pub verdict: Verdict,
    /// DPLL's events, in order; empty for saturation.
    pub trace: Vec<Event>,
    /// What the search cost.
    pub statistics: Statistics,
}

/// What a search cost.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Statistics {
    /// The clauses kept: the input's and the resolvents saturation added.
    pub clauses: u64,
    /// The steps: resolvents made, or DPLL's events.
    pub steps: u64,
}

/// Returns the clauses of `sequent`'s negation, made by the normal form
/// `form` and checked as [`normal::normalize`](super::normal::normalize)
/// checks it, with that normal form: the clauses are unsatisfiable
/// exactly when the sequent is valid. For conjunctive normal form the
/// formulas right of `⊢` move left negated first, then each hypothesis is
/// rewritten and taken apart into its clauses; Tseitin's and Plaisted and
/// Greenbaum's encodings negate the sequent themselves.
///
/// # Errors
///
/// [`Error::InvalidOption`] (key `clauses`) for a form that gives no
/// clauses; [`normal::normalize`](super::normal::normalize)'s errors.
pub fn clauses_of(
    sequent: &Sequent,
    form: super::normal::Form,
    search: &crate::linear::search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(Clauses, super::normal::NormalForm), Error> {
    use super::normal::{self, Form};
    let options = normal::Options::default().with_form(form);
    let made = match form {
        Form::Cnf => {
            let mut formulas = sequent.formulas().clone();
            let mut left = sequent.left().to_vec();
            for &d in sequent.right() {
                left.push(formulas.add(Node::Not(d))?);
            }
            let negation = Sequent::new(formulas, left, Vec::new())?;
            normal::normalize(&negation, &options, search, limits, &mut stop)?
        }
        Form::Tseitin | Form::PlaistedGreenbaum => {
            normal::normalize(sequent, &options, search, limits, &mut stop)?
        }
        Form::Nnf | Form::Dnf => {
            return Err(Error::InvalidOption {
                key: "clauses",
                message: format!(
                    "{} gives no clauses: take cnf, tseitin or plaisted-greenbaum",
                    form.title()
                ),
            });
        }
    };
    Ok((Clauses::of_sequent(&made.result)?, made))
}

/// Decides whether `clauses` are satisfiable by the method `options`
/// names, within its bounds, `limits` and `stop`, and checks the answer.
///
/// # Errors
///
/// [`Error::ClaimFails`] where the refutation or the model found fails its
/// check, a defect; [`Refusal::Memory`] past `limits.memory_bytes`.
pub fn decide(
    clauses: &Clauses,
    options: &Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Resolution, Error> {
    // The set itself and the arrays per atom, before any search.
    let base = clauses.clauses().iter().fold(
        (clauses.atoms().len() as u64).saturating_mul(ATOM_BYTES),
        |sum, c| {
            sum.saturating_add(CLAUSE_BYTES)
                .saturating_add(4 * c.literals().len() as u64)
        },
    );
    if let Some(limit_bytes) = limits.memory_bytes
        && base > limit_bytes
    {
        return Err(Error::Refused(Refusal::Memory {
            phase: crate::limits::Phase::Search,
            limit_bytes,
            needed_bytes: Some(base),
        }));
    }
    let found = match options.method {
        Method::Saturation => saturate::run(clauses, options, limits, &mut stop)?,
        Method::Dpll => dpll::run(clauses, options, limits, &mut stop)?,
    };
    let fails = |reason: String| Error::ClaimFails {
        claim: format!("the {} search", options.method.name()),
        reason,
    };
    match &found.verdict {
        Verdict::Refuted(refutation) => {
            if refutation.clauses() != clauses {
                return Err(fails("its refutation is of other clauses".to_owned()));
            }
            refutation.check(limits, &mut stop).map_err(|e| {
                if let Error::Refused(_) = e {
                    e
                } else {
                    fails(format!("its refutation fails the check: {e}"))
                }
            })?;
        }
        Verdict::Satisfiable { true_atoms } => {
            if let Some(k) = clauses.falsified(true_atoms) {
                return Err(fails(format!("its model makes clause {} false", k + 1)));
            }
        }
        Verdict::Unknown(_) => {}
    }
    Ok(found)
}
