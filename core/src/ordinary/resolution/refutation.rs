// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A refutation by resolution: a set of clauses and resolvents derived
//! from it, each from two earlier clauses, which the checker checks again.

use super::{Clause, Clauses, Literal};
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::{Error, StepFault};

/// A resolvent: the clause resolution derives from two earlier clauses on
/// an atom. The clauses of a refutation are numbered from 0: first the
/// set's, then the resolvents in order.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolvent {
    /// The clause that holds the atom.
    pub first: usize,
    /// The clause that holds its negation.
    pub second: usize,
    /// The atom resolved on, the pivot.
    pub atom: u32,
    /// The resolvent: the first clause's literals but the atom, and the
    /// second's but its negation.
    pub clause: Clause,
}

/// A refutation by resolution of a set of clauses: resolvents, each from
/// two clauses before it, up to the empty clause. It is also a proof in
/// progress: [`resolve`](Self::resolve) adds a step, and
/// [`is_finished`](Self::is_finished) says whether `□` is derived. Its
/// clauses and resolvents are the nodes of a graph, each resolvent with an
/// edge from each of its two clauses.
///
/// # JSON
///
/// With the feature `serialize` a refutation is the document `{"version":
/// 22, "atoms": ["a", "b"], "clauses": [[1, 2], [-1], [-2]],
/// "resolvents": [{"first": 0, "second": 1, "atom": 1, "clause": [2]},
/// {"first": 3, "second": 2, "atom": 2, "clause": []}]}`
/// ([`wire`](crate::wire)): the set as [`Clauses`] writes it, and each
/// resolvent's clauses by number, the atom counted from 1 and the clause
/// as DIMACS writes it. Reading one checks its numbers, not its steps,
/// which [`check`](Self::check) does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refutation {
    /// The set refuted.
    clauses: Clauses,
    /// The resolvents, in order.
    resolvents: Vec<Resolvent>,
}

/// Returns the resolvent of `a` and `b` on `atom`, and whether `a` holds
/// the atom (else `b` does), where one holds the atom and the other its
/// negation: the literals of both but the atom from the clause that holds
/// it and its negation from the other. A clause that holds both keeps the
/// other one, so `p ∨ ¬p` and `¬p` resolve to `¬p`, never to `□`.
fn resolvent(a: &Clause, b: &Clause, atom: u32) -> Option<(Clause, bool)> {
    let (positive, negative) = (Literal::of(atom, true), Literal::of(atom, false));
    let a_first = if a.contains(positive) && b.contains(negative) {
        true
    } else if a.contains(negative) && b.contains(positive) {
        false
    } else {
        return None;
    };
    let (from_a, from_b) = if a_first {
        (positive, negative)
    } else {
        (negative, positive)
    };
    let literals = a
        .literals()
        .iter()
        .copied()
        .filter(|&l| l != from_a)
        .chain(b.literals().iter().copied().filter(|&l| l != from_b))
        .collect();
    Some((Clause::new(literals), a_first))
}

impl Refutation {
    /// Returns a refutation of `clauses` with no resolvent yet.
    pub fn new(clauses: Clauses) -> Self {
        Self {
            clauses,
            resolvents: Vec::new(),
        }
    }

    /// Returns the refutation of `clauses` by `resolvents`, unchecked: for
    /// a reader, which [`check`](Self::check) follows.
    #[cfg(feature = "serialize")]
    pub(crate) fn of_parts(clauses: Clauses, resolvents: Vec<Resolvent>) -> Self {
        Self {
            clauses,
            resolvents,
        }
    }

    /// Returns the set refuted.
    pub fn clauses(&self) -> &Clauses {
        &self.clauses
    }

    /// Returns the resolvents, in order.
    pub fn resolvents(&self) -> &[Resolvent] {
        &self.resolvents
    }

    /// Returns how many clauses the refutation numbers: the set's and the
    /// resolvents.
    pub fn len(&self) -> usize {
        self.clauses.clauses().len() + self.resolvents.len()
    }

    /// Returns whether the refutation numbers no clause.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the clause numbered `k`, the set's or a resolvent.
    pub fn clause(&self, k: usize) -> Option<&Clause> {
        let given = self.clauses.clauses();
        given
            .get(k)
            .or_else(|| self.resolvents.get(k - given.len()).map(|r| &r.clause))
    }

    /// Returns whether a clause numbered is the empty clause `□`: the set
    /// is refuted.
    pub fn is_finished(&self) -> bool {
        self.clauses.clauses().iter().any(Clause::is_empty)
            || self.resolvents.iter().any(|r| r.clause.is_empty())
    }

    /// Resolves the clauses numbered `first` and `second` on `atom`, one
    /// holding the atom and the other its negation in either order, adds
    /// the resolvent as the next step and returns its number.
    ///
    /// # Errors
    ///
    /// [`Error::StepRefused`] for a number of no clause yet, or clauses
    /// that do not hold the atom and its negation.
    pub fn resolve(&mut self, first: usize, second: usize, atom: u32) -> Result<usize, Error> {
        let refused = |fault: StepFault| Error::StepRefused {
            fault: Box::new(fault),
        };
        let (Some(a), Some(b)) = (self.clause(first), self.clause(second)) else {
            return Err(refused(StepFault::NoClause {
                clauses: self.len(),
            }));
        };
        let Some(named) = self.clauses.atoms().get(atom as usize).cloned() else {
            return Err(refused(StepFault::NoAtom {
                atoms: self.clauses.atoms().len(),
            }));
        };
        let (clause, a_first) = resolvent(a, b, atom).ok_or_else(|| {
            refused(StepFault::NoClash {
                first,
                second,
                atom: named.clone(),
            })
        })?;
        let (first, second) = if a_first {
            (first, second)
        } else {
            (second, first)
        };
        self.resolvents.push(Resolvent {
            first,
            second,
            atom,
            clause,
        });
        Ok(self.len() - 1)
    }

    /// Checks every resolvent: its two clauses come before it, the first
    /// holds the atom and the second its negation, and its clause is
    /// exactly their other literals; and some clause is empty. A
    /// resolvent is a unit of `limits.work`, and the stop is asked once per
    /// resolvent.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidRefutation`] for the first resolvent that fails,
    /// or no empty clause; [`Refusal::Work`] past `limits.work`;
    /// [`Refusal::Stopped`] where the stop fires.
    pub fn check(&self, limits: &Limits, stop: impl FnMut(Progress) -> bool) -> Result<(), Error> {
        self.checked(true, limits, stop)
    }

    /// Checks every resolvent as [`check`](Self::check) does, the empty
    /// clause not asked for and every resolvent checked, after one that is
    /// empty too: a refutation in progress, built by hand.
    ///
    /// # Errors
    ///
    /// As [`check`](Self::check), but for the empty clause.
    pub fn check_steps(
        &self,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        self.checked(false, limits, stop)
    }

    /// Checks the resolvents, and where `finished` stops at the first
    /// empty clause and fails without one.
    fn checked(
        &self,
        finished: bool,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let given = self.clauses.clauses().len();
        let atoms = self.clauses.atoms().len();
        let wrong = |step: usize, reason: String| Error::InvalidRefutation {
            step: Some(step as u64 + 1),
            reason,
        };
        if finished && self.clauses.clauses().iter().any(Clause::is_empty) {
            return Ok(());
        }
        for (k, r) in self.resolvents.iter().enumerate() {
            if let Some(limit) = limits.work
                && k as u64 >= limit
            {
                return Err(Error::Refused(Refusal::Work { limit }));
            }
            if stop(Progress::new(Phase::Check, 1, k as u64)) {
                return Err(Error::Refused(Refusal::stopped(Phase::Check)));
            }
            let here = given + k;
            if r.first >= here || r.second >= here {
                return Err(wrong(
                    k,
                    format!("it resolves a clause numbered {here} or later"),
                ));
            }
            if r.atom as usize >= atoms {
                return Err(wrong(k, "its atom is none of the clauses'".to_owned()));
            }
            let (a, b) = (
                &self.clause_unchecked(r.first),
                &self.clause_unchecked(r.second),
            );
            let positive = Literal::of(r.atom, true);
            if !a.contains(positive) || !b.contains(positive.negated()) {
                return Err(wrong(
                    k,
                    "its first clause does not hold the atom, or its second the atom's negation"
                        .to_owned(),
                ));
            }
            let (clause, _) =
                resolvent(a, b, r.atom).unwrap_or_else(|| unreachable!("both clauses were tested"));
            if clause != r.clause {
                return Err(wrong(k, "its clause is not the resolvent".to_owned()));
            }
            if r.clause
                .literals()
                .iter()
                .any(|l| l.atom() as usize >= atoms)
            {
                return Err(wrong(k, "its clause names no atom of the set".to_owned()));
            }
            if finished && r.clause.is_empty() {
                return Ok(());
            }
        }
        if !finished {
            return Ok(());
        }
        Err(Error::InvalidRefutation {
            step: None,
            reason: "no clause is empty".to_owned(),
        })
    }

    /// Returns the clause numbered `k`, which is below [`len`](Self::len).
    fn clause_unchecked(&self, k: usize) -> Clause {
        self.clause(k)
            .cloned()
            .unwrap_or_else(|| unreachable!("a number below the length"))
    }

    /// Returns the refutation cut down to the resolvents the first empty
    /// clause needs, renumbered in order; the set stays whole. A finished
    /// refutation stays one.
    #[must_use]
    pub fn trimmed(&self) -> Self {
        let given = self.clauses.clauses().len();
        let Some(end) = self.resolvents.iter().position(|r| r.clause.is_empty()) else {
            return self.clone();
        };
        if self.clauses.clauses().iter().any(Clause::is_empty) {
            return Self::new(self.clauses.clone());
        }
        let mut needed = vec![false; end + 1];
        needed[end] = true;
        for k in (0..=end).rev() {
            if needed[k] {
                let r = &self.resolvents[k];
                for parent in [r.first, r.second] {
                    if parent >= given {
                        needed[parent - given] = true;
                    }
                }
            }
        }
        let mut number = vec![0usize; end + 1];
        let mut resolvents = Vec::new();
        for k in 0..=end {
            if needed[k] {
                let r = &self.resolvents[k];
                let renumber = |p: usize| if p < given { p } else { number[p - given] };
                resolvents.push(Resolvent {
                    first: renumber(r.first),
                    second: renumber(r.second),
                    atom: r.atom,
                    clause: r.clause.clone(),
                });
                number[k] = given + resolvents.len() - 1;
            }
        }
        Self {
            clauses: self.clauses.clone(),
            resolvents,
        }
    }
}
