// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The connection prover of intuitionistic logic, first-order included:
//! the prefixed non-clausal connection calculus of nanoCoP-i (Otten,
//! "Non-clausal connection calculi for non-classical logics", TABLEAUX
//! 2017; "nanoCoP-i", ARQNL 2016).
//!
//! The matrix is the classical prover's. Beside it every position whose
//! rule changes the world of a Kripke model gets a prefix character
//! ([`prefix`]), and a classical connection proof is an intuitionistic one
//! when its prefixes unify and its eigenvariables meet the domain
//! condition (Otten 2017, Definition 2 and Theorem 1). The search
//! ([`prove`]) checks each connection's prefixes on their own and the
//! whole proof's at its end, then reads the proof back to LJ by a search
//! that the proof guides ([`read_back`]); the derivation is checked, so
//! "valid" rests on the checker.
//!
//! The search is no decision procedure, even without quantifiers: prefix
//! variables need copies of clauses, and the bound on the active path
//! deepens without end. A failure is "unknown", never "not valid".

mod prefix;
mod prove;
mod read_back;
#[cfg(all(test, feature = "parse"))]
mod tests;

use super::matrix::Matrix;
use super::prove::{Config, Counts};
use super::{Outcome, Statistics};
use crate::Error;
use crate::Refusal;
use crate::limits::{Limits, Progress};
use crate::linear::search::{self, Reason};
use crate::ordinary::{Evidence, Logic, Sequent, Verdict};

/// Decides an intuitionistic sequent by the prefixed connection prover
/// within `limits`, asking `stop` as it goes.
///
/// # Errors
///
/// [`Error::Succedents`] for more than one formula right of `⊢`;
/// [`Refusal::Occurrences`] for a sequent that
/// unfolds past `limits.occurrences`; [`Refusal::Terms`] where an
/// instance would pass `limits.terms`; and [`Error::ReadBack`] where a
/// derivation read back fails the check, a defect. Another bound or the
/// stop that ends a read-back or its check is no error: the outcome is
/// unknown with that reason and the search's statistics.
pub(super) fn decide(
    sequent: &Sequent,
    logic: Logic,
    options: &search::Options,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    decide_counting(sequent, logic, options, limits, stop).map(|(outcome, _)| outcome)
}

/// [`decide`], with the number of proofs whose prefixes unified but that
/// did not read back.
fn decide_counting(
    sequent: &Sequent,
    logic: Logic,
    options: &search::Options,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<(Outcome, u64), Error> {
    if sequent.right().len() > 1 {
        return Err(Error::Succedents {
            count: sequent.right().len(),
        });
    }
    // The stop or the memory bound ending the matrix's build is an unknown
    // verdict, as one ending the search.
    let unknown = |reason| Outcome {
        verdict: Verdict::Unknown(reason),
        logic,
        statistics: Statistics::default(),
    };
    let built = Matrix::of(sequent, limits, &mut *stop).and_then(|matrix| {
        let prefixes = prefix::Prefixes::of(&matrix, sequent.formulas(), limits, &mut *stop)?;
        Ok((matrix, prefixes))
    });
    let (matrix, mut prefixes) = match built {
        Ok(built) => built,
        Err(Error::Refused(Refusal::Stopped { .. })) => return Ok((unknown(Reason::Stopped), 0)),
        Err(Error::Refused(Refusal::Memory { limit_bytes, .. })) => {
            return Ok((unknown(Reason::MemoryLimit { limit_bytes }), 0));
        }
        Err(error) => return Err(error),
    };
    let config = Config {
        path_length: options.path_length,
        restricted: options.restricted_backtracking,
    };
    let mut counts = Counts::default();
    let mut rejected = (0, 0);
    // The read-back is a search of its own, which the prover's stop does
    // not reach: it is bounded by its steps, and asks the stop as it emits.
    let stop_cell = std::cell::RefCell::new(stop);
    let mut accept = |terms: &super::terms::Terms| {
        let mut guard = stop_cell.borrow_mut();
        let stop: &mut dyn FnMut(Progress) -> bool = &mut **guard;
        let Some(derivation) = read_back::read_back(sequent, &matrix, terms, logic, limits, stop)?
        else {
            return Ok(None);
        };
        // A derivation the checker refuses is a defect; a bound or the stop
        // refusing the check is that refusal.
        derivation
            .check(limits, &mut *stop)
            .map_err(|e| derivation.own_fault(e))?;
        Ok(Some(derivation))
    };
    let mut search_stop = |progress: Progress| (*stop_cell.borrow_mut())(progress);
    let end = prove::search(
        &matrix,
        &mut prefixes,
        config,
        limits,
        &mut search_stop,
        &mut accept,
        &mut counts,
        &mut rejected,
    );
    let verdict = match end {
        Ok(prove::End::Proved(derivation)) => Verdict::Valid(Evidence::Derivation(derivation)),
        Ok(prove::End::Unknown(reason)) => Verdict::Unknown(reason),
        // A bound or the stop that ends a read-back or its check leaves the
        // sequent unknown, the search's statistics kept.
        Err(error) => Verdict::Unknown(super::super::decide::ended(error)?),
    };
    let outcome = Outcome {
        verdict,
        logic,
        statistics: Statistics {
            steps: counts.steps,
            extensions: counts.extensions,
            reductions: counts.reductions,
            lemmata: counts.lemmata,
            path_length: counts.path_length,
        },
    };
    Ok((outcome, rejected.1))
}
