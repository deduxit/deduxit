// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Serde for resolution: a refutation (written and read) and a clause set
//! decided (written only), at wire level 22.

use crate::Error;
use crate::limits::{Limits, Refusal};
use crate::ordinary::resolution::{
    Bound, Clause, Clauses, Event, Literal, Method, Refutation, Resolution, Resolvent, Statistics,
    Verdict,
};
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Returns a literal as DIMACS writes it.
fn number(literal: Literal) -> i64 {
    let atom = i64::from(literal.atom()) + 1;
    if literal.is_positive() { atom } else { -atom }
}

/// Returns a clause as DIMACS writes it.
fn numbers(clause: &Clause) -> Vec<i64> {
    clause.literals().iter().map(|&l| number(l)).collect()
}

/// The written form of a resolvent.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResolventForm {
    /// The clause that holds the atom.
    first: u64,
    /// The clause that holds its negation.
    second: u64,
    /// The atom, counted from 1.
    atom: u64,
    /// The resolvent as DIMACS writes it.
    clause: Vec<i64>,
}

/// The written form of a refutation.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RefutationForm {
    /// The wire level.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The atom names.
    atoms: Vec<String>,
    /// The clauses, as DIMACS writes them.
    clauses: Vec<Vec<i64>>,
    /// The resolvents.
    resolvents: Vec<ResolventForm>,
}

impl RefutationForm {
    /// Returns the form of a refutation, with its level.
    fn of(refutation: &Refutation) -> Self {
        let clauses = refutation.clauses();
        Self {
            version: Some(wire::CLAUSES),
            atoms: clauses.atoms().to_vec(),
            clauses: clauses.clauses().iter().map(numbers).collect(),
            resolvents: refutation
                .resolvents()
                .iter()
                .map(|r| ResolventForm {
                    first: r.first as u64,
                    second: r.second as u64,
                    atom: u64::from(r.atom) + 1,
                    clause: numbers(&r.clause),
                })
                .collect(),
        }
    }
}

/// Returns the literal of a DIMACS number over `atoms` atoms.
fn literal(number: i64, atoms: usize) -> Result<Literal, Error> {
    let atom = number.unsigned_abs();
    if number == 0 || atom > atoms as u64 {
        return Err(Error::IndexOutOfBounds {
            space: crate::limits::Space::Atom,
            index: usize::try_from(atom).unwrap_or(usize::MAX),
            len: atoms,
        });
    }
    // `1 ≤ atom ≤ atoms`, below `Literal::MOST_ATOMS`, which `Clauses::new`
    // keeps to.
    Ok(Literal::of((atom - 1) as u32, number > 0))
}

impl Serialize for Refutation {
    /// Serializes the refutation as a document at level 22.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        RefutationForm::of(self).serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Refutation {
    /// Deserializes a refutation within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Refutation {}

impl Readable for Refutation {
    const FORM: &'static str = "refutation";

    /// Reads a refutation: its atoms and clauses as [`Clauses::new`] does,
    /// every number in range, the literals of clauses and resolvents
    /// together within `limits.occurrences`. Whether its steps hold is
    /// for [`Refutation::check`] to say.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = RefutationForm::deserialize(deserializer)?;
        let build = || {
            let size = form
                .clauses
                .iter()
                .map(Vec::len)
                .chain(form.resolvents.iter().map(|r| r.clause.len()))
                .fold(0u64, |sum, n| sum.saturating_add(n as u64 + 1));
            if let Some(limit) = limits.occurrences
                && size > limit
            {
                return Err(Error::Refused(Refusal::Occurrences {
                    occurrences: size,
                    limit,
                }));
            }
            let atoms = form.atoms.len();
            let mut clauses = Vec::with_capacity(form.clauses.len());
            for clause in &form.clauses {
                clauses.push(
                    clause
                        .iter()
                        .map(|&n| literal(n, atoms))
                        .collect::<Result<Vec<_>, _>>()?,
                );
            }
            let set = Clauses::new(form.atoms.clone(), clauses)?;
            let given = set.clauses().len();
            let mut resolvents = Vec::with_capacity(form.resolvents.len());
            for (k, r) in form.resolvents.iter().enumerate() {
                let before = (given + k) as u64;
                for n in [r.first, r.second] {
                    if n >= before {
                        return Err(Error::IndexOutOfBounds {
                            space: crate::limits::Space::Node,
                            index: usize::try_from(n).unwrap_or(usize::MAX),
                            len: given + k,
                        });
                    }
                }
                let atom = literal(i64::try_from(r.atom).unwrap_or(i64::MAX), atoms)?.atom();
                let clause = r
                    .clause
                    .iter()
                    .map(|&n| literal(n, atoms))
                    .collect::<Result<Vec<_>, _>>()?;
                resolvents.push(Resolvent {
                    // Below `before`, a count of what is held.
                    first: r.first as usize,
                    second: r.second as usize,
                    atom,
                    clause: Clause::new(clause),
                });
            }
            Ok(Refutation::of_parts(set, resolvents))
        };
        build().map_err(wire::fail)
    }
}

/// The written form of a model: the atoms true, counted from 1.
#[derive(Serialize)]
struct TrueAtoms<'a>(#[serde(serialize_with = "plus_one")] &'a [u32]);

/// Writes atoms counted from 1.
fn plus_one<S: Serializer>(atoms: &&[u32], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(atoms.iter().map(|&a| u64::from(a) + 1))
}

/// The written form of a clause set decided.
#[derive(Serialize)]
struct ResolutionForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The method.
    method: Method,
    /// `unsatisfiable`, `satisfiable` or `unknown`.
    verdict: &'static str,
    /// The atom names.
    atoms: &'a [String],
    /// The clauses, as DIMACS writes them.
    clauses: Vec<Vec<i64>>,
    /// The refutation's resolvents, for an unsatisfiable set.
    #[serde(skip_serializing_if = "Option::is_none")]
    resolvents: Option<Vec<ResolventForm>>,
    /// The atoms a model makes true, for a satisfiable set.
    #[serde(skip_serializing_if = "Option::is_none")]
    true_atoms: Option<TrueAtoms<'a>>,
    /// The bound that ended the search, for an unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    bound: Option<Bound>,
    /// DPLL's events.
    #[serde(skip_serializing_if = "<[Event]>::is_empty")]
    trace: &'a [Event],
    /// What the search cost.
    statistics: Statistics,
}

impl Serialize for Resolution {
    /// Serializes the clause set decided as a document at level 22,
    /// written only.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (verdict, resolvents, true_atoms, bound) = match &self.verdict {
            Verdict::Refuted(r) => (
                "unsatisfiable",
                Some(RefutationForm::of(r).resolvents),
                None,
                None,
            ),
            Verdict::Satisfiable { true_atoms } => {
                ("satisfiable", None, Some(TrueAtoms(true_atoms)), None)
            }
            Verdict::Unknown(bound) => ("unknown", None, None, Some(*bound)),
        };
        ResolutionForm {
            version: wire::CLAUSES,
            deduxit: env!("CARGO_PKG_VERSION"),
            method: self.method,
            verdict,
            atoms: self.clauses.atoms(),
            clauses: self.clauses.clauses().iter().map(numbers).collect(),
            resolvents,
            true_atoms,
            bound,
            trace: &self.trace,
            statistics: self.statistics,
        }
        .serialize(serializer)
    }
}
