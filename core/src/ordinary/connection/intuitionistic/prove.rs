// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The proof search of the prefixed non-clausal connection calculus
//! (Otten, "Non-clausal connection calculi for non-classical logics",
//! TABLEAUX 2017, Figure 2, the calculus of nanoCoP-i): the classical
//! search, with prefixes beside it.
//!
//! - Each connection's two prefixes must be unifiable on their own
//!   (Otten and Kreitz's test), and their equation is kept.
//! - Once every path holds a connection, the equations of the whole proof
//!   are unified together, with one more per eigenvariable a term variable
//!   is bound to (the domain condition); where none unifies, the search
//!   backtracks into its last choice, as nanoCoP-i does.
//! - A proof that unifies is handed to the caller, which reads it back to
//!   LJ; where that fails, the search backtracks too.
//! - Regularity and lemmata compare prefixes as well as terms, and a
//!   literal solved is never settled: another of its solutions may be the
//!   one whose prefixes unify.
//! - A copy of a clause renames its prefix characters, so the bound on the
//!   active path holds for every copy that renames a variable or a
//!   character; the search is no decision procedure, even without
//!   quantifiers, and a failure is "unknown".

use super::super::matrix::{Matrix, NONE, Shape, Symbol};
use super::super::prove::{self, Calculus, Config, Counts, Ended, Search};
use super::super::terms::{Head, Terms};
use super::prefix::{AUXILIARY, NoUnifier, Prefixes, Sym, unifiable, unify};
use crate::Error;
use crate::hash::HashSet;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::search::Reason;
use crate::ordinary::Derivation;

/// How many steps the search takes between two asks of its stop.
const POLL: u64 = 1 << 10;

/// The prefix characters read that count as one step between two asks
/// of the stop.
const PREFIX_CHARS: u64 = 1 << 6;

/// The steps the unification of a proof's prefixes may take before
/// the proof is given up.
const UNIFY_STEPS: u64 = 1 << 16;

/// How a search ended.
pub(super) enum End {
    /// A proof, read back.
    Proved(Box<Derivation>),
    /// No proof, for the reason given.
    Unknown(Reason),
}

/// What the caller makes of a proof whose prefixes unify: its derivation,
/// `None` where it does not read back (the search goes on), or an error
/// that ends the search.
pub(super) type Accept<'a> = dyn FnMut(&Terms) -> Result<Option<Derivation>, Error> + 'a;

/// What the prefixes add to the classical search, a round's state and the
/// caller's hook.
struct Prefixed<'p, 'a> {
    /// The prefixes' characters.
    prefixes: &'p mut Prefixes,
    /// Per clause node, whether a copy of it renames a character.
    renames: Vec<bool>,
    /// The prefix equation of every connection made in the round.
    eqs: Vec<(Vec<Sym>, Vec<Sym>)>,
    /// Proofs whose prefixes did not unify, and proofs that did but did
    /// not read back.
    rejected: (u64, u64),
    /// What the caller makes of a proof whose prefixes unify.
    accept: &'p mut Accept<'a>,
}

impl Calculus for Prefixed<'_, '_> {
    type Found = Derivation;
    type Key = Vec<Sym>;
    type Mark = u32;
    const CLASSICAL: bool = false;

    fn round(&mut self) {
        self.eqs = Vec::new();
    }

    // A prefix read costs its length, which a deep nest makes dear: the
    // stop is asked by that work too.
    fn work(&mut self) -> u64 {
        self.prefixes.take_read() / PREFIX_CHARS
    }

    fn longest(&self) -> usize {
        self.eqs.len()
    }

    fn held_bytes(&self) -> usize {
        self.prefixes.held_bytes()
            + self.eqs.capacity() * size_of::<(Vec<Sym>, Vec<Sym>)>()
            + self
                .eqs
                .iter()
                .map(|(a, b)| (a.capacity() + b.capacity()) * size_of::<Sym>())
                .sum::<usize>()
    }

    fn mark(&self) -> u32 {
        self.eqs.len() as u32
    }

    fn restore(&mut self, mark: u32) {
        self.eqs.truncate(mark as usize);
    }

    // Equal terms and the same prefix: equal under every substitution.
    fn key(&mut self, matrix: &Matrix, terms: &Terms, lit: u32, copy: u32) -> Vec<Sym> {
        self.prefixes.of_literal(matrix, terms, lit, copy)
    }

    // Keeps the prefix equation of the connection where the two prefixes
    // can be unified at all.
    fn connect(
        &mut self,
        matrix: &Matrix,
        terms: &Terms,
        (a, ca): (u32, u32),
        (b, cb): (u32, u32),
    ) -> bool {
        let pa = self.prefixes.of_literal(matrix, terms, a, ca);
        let pb = self.prefixes.of_literal(matrix, terms, b, cb);
        if !unifiable(&pa, &pb) {
            return false;
        }
        self.eqs.push((pa, pb));
        true
    }

    fn renames(&self, clause: u32) -> bool {
        self.renames[clause as usize]
    }

    // Every path holds a connection: the prefixes decide, and then the
    // read-back.
    fn proved(
        search: &mut Search<'_, '_, Self>,
        counts: &Counts,
    ) -> Result<Option<Derivation>, Ended> {
        let unified = match prefixes_unify(search, counts.steps) {
            Ok(unified) => unified,
            Err(NoUnifier::Stopped) => return Err(Ended::Reason(Reason::Stopped)),
            Err(NoUnifier::Failed) => false,
        };
        if !unified {
            search.calculus.rejected.0 += 1;
            return Ok(None);
        }
        let read = (search.calculus.accept)(&search.terms).map_err(|error| {
            // The stop or the memory bound during the read-back ends the
            // search, as during the search.
            if let Error::Refused(crate::Refusal::Memory { limit_bytes, .. }) = error {
                Ended::Reason(Reason::MemoryLimit { limit_bytes })
            } else if matches!(error, Error::Refused(crate::Refusal::Stopped { .. })) {
                Ended::Reason(Reason::Stopped)
            } else {
                Ended::Error(error)
            }
        })?;
        if read.is_none() {
            search.calculus.rejected.1 += 1;
        }
        Ok(read)
    }
}

/// Searches for a proof of `matrix` whose prefixes unify and which
/// `accept` reads back, deepening the bound on the active path from 1,
/// and asks `stop` every [`POLL`] units of work: the classical search
/// ([`prove::search`]) with the prefixes beside it.
///
/// # Errors
///
/// What `accept` answers.
#[expect(
    clippy::too_many_arguments,
    reason = "the matrix, its prefixes, the bounds and the caller's hooks"
)]
pub(super) fn search(
    matrix: &Matrix,
    prefixes: &mut Prefixes,
    config: Config,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
    accept: &mut Accept<'_>,
    counts: &mut Counts,
    rejected: &mut (u64, u64),
) -> Result<End, Error> {
    let renames: Vec<bool> = (0..matrix.nodes.len() as u32)
        .map(|n| matrix.nodes[n as usize].shape == Shape::Clause && prefixes.renames(n))
        .collect();
    let mut calculus = Prefixed {
        prefixes,
        renames,
        eqs: Vec::new(),
        rejected: (0, 0),
        accept,
    };
    let end = prove::search(matrix, &mut calculus, config, limits, stop, counts);
    rejected.0 += calculus.rejected.0;
    rejected.1 += calculus.rejected.1;
    Ok(match end? {
        prove::End::Proved { found, .. } => End::Proved(Box::new(found)),
        prove::End::Unknown(reason) => End::Unknown(reason),
        // Prefixes need copies even without quantifiers, so no round is
        // the calculus's whole search.
        prove::End::Exhausted => unreachable!("only the classical search is exhausted"),
    })
}

/// Returns whether the prefix equations of the proof, with the domain
/// condition's, have a unifier: for every term variable bound to a term
/// with an eigenvariable (a Skolem term), the eigenvariable's prefix
/// begins the variable's. `NoUnifier::Stopped` where the stop fired.
fn prefixes_unify(
    search: &mut Search<'_, '_, Prefixed<'_, '_>>,
    steps: u64,
) -> Result<bool, NoUnifier> {
    let matrix = search.matrix;
    let ask = &mut *search.stop;
    let mut stop = || ask(Progress::new(Phase::Search, 0, steps));
    let mut eqs = search.calculus.eqs.clone();
    for k in 0..search.terms.copies.len() as u32 {
        let c = search.terms.copies[k as usize];
        // The variables this copy made, with their slots.
        let made: Vec<(u32, u32)> = if c.clause == NONE {
            (0..c.vars.1)
                .map(|s| (s, search.terms.vars[s as usize]))
                .collect()
        } else {
            matrix
                .clause_slots(c.clause)
                .iter()
                .enumerate()
                .filter(|&(_, &(_, free))| free)
                .map(|(at, &(slot, _))| (slot, search.terms.vars[(c.vars.0 + at as u32) as usize]))
                .collect()
        };
        for (slot, var) in made {
            let (pat, copy) = search.terms.bound[var as usize];
            if pat == NONE {
                continue;
            }
            let mut skolems = Vec::new();
            let mut stack = vec![(pat, copy)];
            // Bindings share terms: each part is visited once, and each
            // δ-occurrence in a copy gives one equation.
            let mut seen: HashSet<(u32, u32)> = HashSet::default();
            let mut deltas: HashSet<(u32, u32)> = HashSet::default();
            while let Some(t) = stack.pop() {
                if !seen.insert(t) {
                    continue;
                }
                if seen.len().is_multiple_of(POLL as usize) && stop() {
                    return Err(NoUnifier::Stopped);
                }
                if let Head::App(f, (s, len), d) = search.terms.head(matrix, t) {
                    if let Symbol::Skolem(sk) = matrix.symbols[f as usize]
                        && deltas.insert((matrix.skolem_occ[sk as usize], d))
                    {
                        skolems.push((matrix.skolem_occ[sk as usize], d));
                    }
                    for &a in &matrix.pat_args[s as usize..(s + len) as usize] {
                        stack.push((a, d));
                    }
                }
            }
            if skolems.is_empty() {
                continue;
            }
            let gamma = matrix.slot_occ[slot as usize];
            let of_var = search
                .calculus
                .prefixes
                .of_occurrence(matrix, &search.terms, gamma, k);
            for (delta, d) in skolems {
                let mut of_eigen =
                    search
                        .calculus
                        .prefixes
                        .of_occurrence(matrix, &search.terms, delta, d);
                of_eigen.push(search.calculus.prefixes.fresh());
                eqs.push((of_var.clone(), of_eigen));
            }
        }
    }
    if search.calculus.prefixes.exhausted() {
        return Ok(false);
    }
    let mut budget = UNIFY_STEPS;
    match unify(&eqs, AUXILIARY, &mut budget, &mut stop) {
        Ok(_) => Ok(true),
        Err(NoUnifier::Failed) => Ok(false),
        Err(NoUnifier::Stopped) => Err(NoUnifier::Stopped),
    }
}
