// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A connection prover of classical and intuitionistic logic, first-order
//! included: [`decide`] answers "valid" with a checked derivation of LK or
//! LJ, "not valid" in classical logic where its search is exhaustive, and
//! "unknown" otherwise.
//!
//! The search is in Otten's non-clausal connection calculus, the calculus
//! of the prover nanoCoP ("A Non-clausal Connection Calculus", TABLEAUX
//! 2011; "nanoCoP: A Non-clausal Connection Prover", IJCAR 2016). It works
//! on the formulas' own structure: no clausal form. A sequent is valid
//! when every path through its matrix holds a connection, two literals of
//! one predicate, of opposite polarity, that the rigid substitution makes
//! equal. The search
//! - copies a clause by renaming its variables where it needs a quantified
//!   formula more than once;
//! - reads `∀` on the right and `∃` on the left as Skolem terms, so that
//!   unification with its occurs check keeps the eigenvariable condition;
//! - keeps every literal at most once on the active path (regularity) and
//!   closes a literal equal to one solved before (lemmata);
//! - deepens a bound on the active path's length for steps that make a
//!   fresh variable ([`search::Options::path_length`]), so that no proof
//!   is out of reach for want of copies.
//!
//! A proof is not trusted: its substitution and the literals it used
//! guide the construction of a derivation of LK in G3's form, which
//! [`Derivation::check`](super::Derivation::check) accepts or the call fails ([`Error::ReadBack`], a
//! defect). Where the matrix has no variable at all (no quantifier needs
//! a witness term: every `∀` is one to prove and every `∃` one to assume,
//! negations and implications taken into account), every copy of a
//! clause is the clause, the
//! search is the calculus's whole search and ends, and a search that finds
//! no proof shows the sequent not valid (the calculus's completeness,
//! Otten 2011, Theorem 4, with positive start clauses and regularity,
//! Lemmas 4 and 5). With variables a failed search is "unknown": its
//! bound on the path, or [`Reason::Incomplete`] where no bound was met.
//! Restricted backtracking ([`search::Options::restricted_backtracking`])
//! never answers "not valid".

mod intuitionistic;
mod matrix;
mod prove;
mod read_back;
mod terms;
#[cfg(all(test, feature = "parse"))]
mod tests;

use super::{Evidence, Logic, Method, Sequent, Verdict};
use crate::limits::{Limits, Progress};
use crate::linear::search::{self, Reason};
use crate::{Error, Refusal};

/// What the connection prover counted.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct Statistics {
    /// The steps taken: goals taken up and ways tried to solve them. It is
    /// the search's unit of work, which `Limits::work` bounds.
    pub steps: u64,
    /// The extension steps made: a literal connected to one in a copy of a
    /// clause, whose other literals become goals.
    pub extensions: u64,
    /// The reduction steps made: a literal connected to one of the active
    /// path.
    pub reductions: u64,
    /// The literals closed because one equal to them was solved before.
    pub lemmata: u64,
    /// The bound on the active path of the search's last round.
    pub path_length: u32,
}

/// What the connection prover found: the verdict and what the search
/// counted.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The verdict, with a checked derivation of LK or LJ for a valid
    /// sequent.
    pub verdict: Verdict,
    /// The logic decided.
    pub logic: Logic,
    /// What the search counted.
    pub statistics: Statistics,
}

/// Decides a classical or intuitionistic sequent, first-order or not, by
/// the connection prover within `limits`, asking `stop` as it goes. It reads the search
/// options' [`path_length`](search::Options::path_length) and
/// [`restricted_backtracking`](search::Options::restricted_backtracking)
/// and nothing else of them.
///
/// The verdict is "valid" with a derivation of LK (or LJ) that
/// [`Derivation::check`](super::Derivation::check) accepted, "not valid" only in classical logic, where the sequent's
/// matrix has no variable and the whole search failed, and otherwise
/// "unknown": [`Reason::PathLength`] at the bound on the active path,
/// [`Reason::Incomplete`] where a search with variables failed without
/// meeting it, [`Reason::WorkLimit`], [`Reason::MemoryLimit`] or
/// [`Reason::Stopped`].
///
/// # Errors
///
/// [`Error::Logic`] for minimal logic; [`Error::Succedents`] for an
/// intuitionistic sequent with more than one formula right of `⊢`;
/// [`Refusal::Occurrences`] for a sequent that
/// unfolds past `limits.occurrences` (an equivalence's operands unfold
/// twice); a refusal of the derivation by a bound or the stop, as from
/// [`Derivation::check`](super::Derivation::check), never a verdict; and [`Error::ReadBack`] where a
/// proof does not read back or its derivation fails the check, a defect.
pub fn decide(
    sequent: &Sequent,
    logic: Logic,
    options: &search::Options,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    run(sequent, logic, options, limits, &mut stop)
}

/// [`decide`] with the stop as a trait object, so that the search is
/// compiled once.
fn run(
    sequent: &Sequent,
    logic: Logic,
    options: &search::Options,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Outcome, Error> {
    match logic {
        Logic::Classical => {}
        Logic::Intuitionistic => {
            return intuitionistic::decide(sequent, logic, options, limits, stop);
        }
        Logic::Minimal => {
            return Err(Error::Logic {
                logic,
                method: Method::Connection,
            });
        }
    }
    // The stop or the memory bound ending the matrix's build is an unknown
    // verdict, as one ending the search.
    let unknown = |reason| Outcome {
        verdict: Verdict::Unknown(reason),
        logic,
        statistics: Statistics::default(),
    };
    let matrix = match matrix::Matrix::of(sequent, limits, &mut *stop) {
        Ok(matrix) => matrix,
        Err(Error::Refused(Refusal::Stopped { .. })) => return Ok(unknown(Reason::Stopped)),
        Err(Error::Refused(Refusal::Memory { limit_bytes, .. })) => {
            return Ok(unknown(Reason::MemoryLimit { limit_bytes }));
        }
        Err(error) => return Err(error),
    };
    let config = prove::Config {
        path_length: options.path_length,
        restricted: options.restricted_backtracking,
    };
    let mut counts = prove::Counts::default();
    let end = prove::search(&matrix, config, limits, stop, &mut counts);
    let verdict = match end {
        prove::End::Proved { used, terms } => {
            let derivation = read_back::read_back(sequent, &matrix, &used, &terms, limits, stop)?;
            derivation.check(limits, &mut *stop)?;
            Verdict::Valid(Evidence::Derivation(Box::new(derivation)))
        }
        // Restricted backtracking may miss a proof.
        prove::End::Exhausted if config.restricted => Verdict::Unknown(Reason::Incomplete),
        prove::End::Exhausted => Verdict::NotValid,
        prove::End::Unknown(reason) => Verdict::Unknown(reason),
    };
    Ok(Outcome {
        verdict,
        logic,
        statistics: Statistics {
            steps: counts.steps,
            extensions: counts.extensions,
            reductions: counts.reductions,
            lemmata: counts.lemmata,
            path_length: counts.path_length,
        },
    })
}
