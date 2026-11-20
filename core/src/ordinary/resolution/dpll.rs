// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The DPLL procedure of Davis, Putnam, Logemann and Loveland, every step
//! an event of its trace, and the refutation read off the trace.
//!
//! The procedure repeats: a clause all of whose literals are false is a
//! conflict, and the last decision not yet flipped is flipped (both its
//! branches are tried, as DPLL does, without learning); a clause with one
//! literal unassigned and the rest false makes that literal true (unit
//! propagation); a literal whose negation is in no clause not yet true is
//! made true (a pure literal); otherwise the first atom unassigned is
//! decided true. Every clause true is a model; a conflict with no
//! decision left refutes the set.
//!
//! The refutation is read off as the search goes back: a conflict's
//! clause is false under the assignment; resolving it with the clause that
//! propagated a literal of it keeps it false under the assignment without
//! that literal; at a decision both branches' clauses are false under the
//! assignment below it, and resolved on the decision's atom (or one of
//! them taken as it is, where it lacks the atom) so is theirs. A pure
//! literal's negation is in no clause of the reading, since every clause
//! it meets was not true when the literal was found pure. Down at no
//! assignment the clause is empty. [`decide`] checks the result
//! again by [`Refutation::check`].

use super::{Bound, Clauses, Literal, Options, Refutation, Resolution, Statistics, Verdict};
use crate::Error;
use crate::limits::{Limits, Phase, Progress};

/// A step of DPLL's trace. A clause is numbered as in its set, from 0.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(tag = "event", rename_all = "snake_case"))]
pub enum Event {
    /// A clause with one literal left made it true.
    Unit {
        /// The literal made true.
        #[cfg_attr(feature = "serialize", serde(serialize_with = "dimacs"))]
        literal: Literal,
        /// The clause.
        clause: usize,
    },
    /// A literal whose negation is in no clause not yet true was made true.
    Pure {
        /// The literal.
        #[cfg_attr(feature = "serialize", serde(serialize_with = "dimacs"))]
        literal: Literal,
    },
    /// A literal was decided true, opening a branch.
    Decide {
        /// The literal.
        #[cfg_attr(feature = "serialize", serde(serialize_with = "dimacs"))]
        literal: Literal,
        /// The decisions open after it, this one counted.
        level: u32,
    },
    /// Every literal of a clause is false.
    Conflict {
        /// The clause.
        clause: usize,
    },
    /// The last decision was flipped: its negation made true, the second
    /// branch.
    Backtrack {
        /// The literal now true.
        #[cfg_attr(feature = "serialize", serde(serialize_with = "dimacs"))]
        literal: Literal,
        /// The decisions open after it, this one counted.
        level: u32,
    },
}

/// Writes a literal as DIMACS does: the atom counted from 1, negative for a
/// negated one.
#[cfg(feature = "serialize")]
fn dimacs<S: serde::Serializer>(literal: &Literal, serializer: S) -> Result<S::Ok, S::Error> {
    let atom = i64::from(literal.atom()) + 1;
    serializer.serialize_i64(if literal.is_positive() { atom } else { -atom })
}

/// The bytes an event of the trace is charged.
const EVENT_BYTES: u64 = size_of::<Event>() as u64 + size_of::<(Literal, Reason)>() as u64;

/// The bytes a resolvent read off the trace is charged beside its
/// literals.
const RESOLVENT_BYTES: u64 = 64;

/// Why a literal of the trail is true.
#[derive(Clone, Copy)]
enum Reason {
    /// A decision, its first branch.
    Decision,
    /// A decision flipped, its second branch; the clause of the
    /// refutation that the first branch ended with.
    Flipped(usize),
    /// The clause that made it true.
    Unit(usize),
    /// It was pure.
    Pure,
}

/// Runs DPLL on `clauses` within `options.steps` events.
pub(super) fn run(
    clauses: &Clauses,
    options: &Options,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Resolution, Error> {
    let given = clauses.clauses();
    let atoms = clauses.atoms().len();
    // Per atom: `None` unassigned, else its value.
    let mut value: Vec<Option<bool>> = vec![None; atoms];
    let mut trail: Vec<(Literal, Reason)> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    let mut level = 0u32;
    // The bytes the trace and the refutation hold, estimated.
    let mut held = 0u64;
    let mut refutation = Refutation::new(clauses.clone());
    let truth =
        |value: &[Option<bool>], l: Literal| value[l.atom() as usize].map(|v| v == l.is_positive());
    let finish = |verdict, events: Vec<Event>| {
        let steps = events.len() as u64;
        Ok(Resolution {
            method: options.method,
            clauses: clauses.clone(),
            verdict,
            trace: events,
            statistics: Statistics {
                clauses: given.len() as u64,
                steps,
            },
        })
    };
    // Whether the trace holds as many events as the bound allows: the
    // next event is past it.
    let full = |events: &[Event]| options.steps.is_some_and(|s| events.len() as u64 >= s);
    let past = Verdict::Unknown(Bound::Steps {
        limit: options.steps.unwrap_or(u64::MAX),
    });
    loop {
        if stop(Progress::new(Phase::Search, 1, events.len() as u64)) {
            return finish(Verdict::Unknown(Bound::Stopped), events);
        }
        held = held.saturating_add(EVENT_BYTES);
        if let Some(limit_bytes) = limits.memory_bytes
            && held > limit_bytes
        {
            return Err(Error::Refused(crate::limits::Refusal::Memory {
                phase: Phase::Search,
                limit_bytes,
                needed_bytes: None,
            }));
        }
        // A conflict, else a unit clause; and whether a clause is open.
        let mut unit: Option<(Literal, usize)> = None;
        let mut conflict: Option<usize> = None;
        let mut open = false;
        for (k, clause) in given.iter().enumerate() {
            let mut free: Option<Literal> = None;
            let mut frees = 0;
            let mut holds = false;
            for &l in clause.literals() {
                match truth(&value, l) {
                    Some(true) => {
                        holds = true;
                        break;
                    }
                    Some(false) => {}
                    None => {
                        frees += 1;
                        free = Some(l);
                    }
                }
            }
            if holds {
                continue;
            }
            open = true;
            match (frees, free) {
                (0, _) => {
                    conflict = Some(k);
                    break;
                }
                (1, Some(l)) if unit.is_none() => unit = Some((l, k)),
                _ => {}
            }
        }
        if let Some(k) = conflict {
            if full(&events) {
                return finish(past, events);
            }
            events.push(Event::Conflict { clause: k });
            let made = refutation.resolvents().len();
            let found = back(&mut trail, &mut value, &mut refutation, &mut level, k)?;
            held = refutation.resolvents()[made..].iter().fold(held, |sum, r| {
                sum.saturating_add(RESOLVENT_BYTES + 4 * r.clause.literals().len() as u64)
            });
            match found {
                Some(_) if full(&events) => return finish(past, events),
                Some(flipped) => events.push(Event::Backtrack {
                    literal: flipped,
                    level,
                }),
                None => {
                    return finish(Verdict::Refuted(Box::new(refutation.trimmed())), events);
                }
            }
            continue;
        }
        if !open {
            let mut true_atoms: Vec<u32> = (0..atoms)
                .filter(|&a| value[a] == Some(true))
                .map(|a| a as u32)
                .collect();
            true_atoms.sort_unstable();
            return finish(Verdict::Satisfiable { true_atoms }, events);
        }
        if let Some((l, k)) = unit {
            value[l.atom() as usize] = Some(l.is_positive());
            trail.push((l, Reason::Unit(k)));
            if full(&events) {
                return finish(past, events);
            }
            events.push(Event::Unit {
                literal: l,
                clause: k,
            });
            continue;
        }
        // A pure literal: of an atom unassigned, whose negation is in no
        // clause not yet true.
        let mut seen = vec![[false; 2]; atoms];
        for clause in given {
            if clause
                .literals()
                .iter()
                .any(|&l| truth(&value, l) == Some(true))
            {
                continue;
            }
            for &l in clause.literals() {
                if truth(&value, l).is_none() {
                    seen[l.atom() as usize][usize::from(!l.is_positive())] = true;
                }
            }
        }
        if let Some(a) = (0..atoms).find(|&a| seen[a][0] != seen[a][1]) {
            let l = Literal::of(a as u32, seen[a][0]);
            value[a] = Some(l.is_positive());
            trail.push((l, Reason::Pure));
            if full(&events) {
                return finish(past, events);
            }
            events.push(Event::Pure { literal: l });
            continue;
        }
        // Decide the first atom unassigned of a clause not yet true.
        let Some(a) = (0..atoms).find(|&a| seen[a][0] || seen[a][1]) else {
            return Err(Error::ClaimFails {
                claim: "the DPLL search".to_owned(),
                reason: "a clause is open with no atom unassigned".to_owned(),
            });
        };
        let l = Literal::of(a as u32, true);
        value[a] = Some(true);
        trail.push((l, Reason::Decision));
        level += 1;
        if full(&events) {
            return finish(past, events);
        }
        events.push(Event::Decide { literal: l, level });
    }
}

/// Goes back from a conflict at the clause `conflict`: undoes the trail
/// to the last decision of its first branch, resolving the clause down as
/// the module says, and flips that decision, returning the literal now
/// true; `None` when no decision is left and the refutation is done.
fn back(
    trail: &mut Vec<(Literal, Reason)>,
    value: &mut [Option<bool>],
    refutation: &mut Refutation,
    level: &mut u32,
    conflict: usize,
) -> Result<Option<Literal>, Error> {
    let fails = |reason: &str| Error::ClaimFails {
        claim: "the DPLL search".to_owned(),
        reason: reason.to_owned(),
    };
    let holds = |refutation: &Refutation, c: usize, l: Literal| {
        refutation
            .clause(c)
            .is_some_and(|clause| clause.contains(l))
    };
    let mut c = conflict;
    while let Some((l, reason)) = trail.pop() {
        value[l.atom() as usize] = None;
        match reason {
            Reason::Unit(r) => {
                if holds(refutation, c, l.negated()) {
                    c = refutation.resolve(r, c, l.atom())?;
                }
            }
            Reason::Pure => {
                if holds(refutation, c, l.negated()) {
                    return Err(fails("a pure literal's negation is in the clause read off"));
                }
            }
            Reason::Decision => {
                let flipped = l.negated();
                value[flipped.atom() as usize] = Some(flipped.is_positive());
                trail.push((flipped, Reason::Flipped(c)));
                return Ok(Some(flipped));
            }
            Reason::Flipped(first) => {
                *level -= 1;
                // `first` is false under the assignment below with `¬l`
                // true, `c` with `l` true.
                c = if !holds(refutation, first, l) {
                    first
                } else if !holds(refutation, c, l.negated()) {
                    c
                } else {
                    refutation.resolve(first, c, l.atom())?
                };
            }
        }
    }
    if refutation.clause(c).is_some_and(|clause| clause.is_empty()) {
        Ok(None)
    } else {
        Err(fails("the clause read off at no assignment is not empty"))
    }
}
