// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Saturation by resolution: the shortest clause not yet used is resolved
//! with every clause used, until the empty clause appears or nothing new
//! does.
//!
//! Complete for propositional clauses: a tautology is never needed, and a
//! clause that another kept clause subsumes (contains) is never needed
//! either, since whatever it resolves to, the smaller one resolves to a
//! clause that subsumes that. So a set saturated without the empty clause
//! is satisfiable, which [`dpll`] then shows by a model.

use super::{
    Bound, Clause, Clauses, Literal, Options, Refutation, Resolution, Statistics, Verdict,
};
use crate::Error;
use crate::limits::{Limits, Phase, Progress, Refusal};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Where a clause of the search comes from.
#[derive(Clone, Copy)]
enum Origin {
    /// The set's clause of that number.
    Given(usize),
    /// A resolvent of two clauses of the search, on an atom: the clause
    /// that holds the atom first.
    Resolved(usize, usize, u32),
}

/// The bytes a clause of the search is charged beside its literals.
const CLAUSE_BYTES: u64 = 96;

/// Saturates `clauses` within `options`' bounds.
pub(super) fn run(
    clauses: &Clauses,
    options: &Options,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Resolution, Error> {
    let mut made: Vec<(Clause, Origin)> = Vec::new();
    let mut seen: crate::hash::HashSet<Clause> = crate::hash::HashSet::default();
    // The clauses waiting, shortest first, then oldest.
    let mut waiting: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    let mut used: Vec<usize> = Vec::new();
    let mut statistics = Statistics::default();
    let mut held = 0u64;
    let mut hold = |clause: &Clause| -> Result<(), Error> {
        held = held
            .saturating_add(CLAUSE_BYTES)
            .saturating_add(4 * clause.literals().len() as u64);
        match limits.memory_bytes {
            Some(limit_bytes) if held > limit_bytes => Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes,
                needed_bytes: None,
            })),
            _ => Ok(()),
        }
    };
    for (k, clause) in clauses.clauses().iter().enumerate() {
        if clause.is_tautology() || seen.contains(clause) {
            continue;
        }
        hold(clause)?;
        seen.insert(clause.clone());
        waiting.push(Reverse((clause.literals().len(), made.len())));
        made.push((clause.clone(), Origin::Given(k)));
    }
    let unknown = |bound, statistics| {
        Ok(Resolution {
            method: options.method,
            clauses: clauses.clone(),
            verdict: Verdict::Unknown(bound),
            trace: Vec::new(),
            statistics,
        })
    };
    while let Some(Reverse((_, c))) = waiting.pop() {
        statistics.clauses = made.len() as u64;
        if stop(Progress::new(Phase::Search, 1, statistics.steps)) {
            return unknown(Bound::Stopped, statistics);
        }
        let clause = made[c].0.clone();
        if clause.is_empty() {
            return Ok(Resolution {
                method: options.method,
                clauses: clauses.clone(),
                verdict: Verdict::Refuted(Box::new(refutation(clauses, &made, c)?)),
                trace: Vec::new(),
                statistics,
            });
        }
        if used.iter().any(|&u| made[u].0.subsumes(&clause)) {
            continue;
        }
        used.push(c);
        for &other in &used {
            let Some((resolved, origin)) = resolve(&made, c, other) else {
                continue;
            };
            if let Some(limit) = options.steps
                && statistics.steps >= limit
            {
                return unknown(Bound::Steps { limit }, statistics);
            }
            statistics.steps += 1;
            if resolved.is_tautology() || seen.contains(&resolved) {
                continue;
            }
            if let Some(limit) = options.clauses
                && made.len() as u64 >= limit
            {
                return unknown(Bound::Clauses { limit }, statistics);
            }
            hold(&resolved)?;
            seen.insert(resolved.clone());
            waiting.push(Reverse((resolved.literals().len(), made.len())));
            made.push((resolved, origin));
        }
    }
    statistics.clauses = made.len() as u64;
    // Saturated: the set is satisfiable, and DPLL finds its model.
    let model = super::dpll::run(clauses, options, limits, stop)?;
    Ok(Resolution {
        method: options.method,
        clauses: clauses.clone(),
        verdict: model.verdict,
        trace: Vec::new(),
        statistics,
    })
}

/// Returns the resolvent of the clauses `c` and `d` of the search, where
/// exactly one atom is in one and negated in the other (two or more give a
/// tautology), with its origin.
fn resolve(made: &[(Clause, Origin)], c: usize, d: usize) -> Option<(Clause, Origin)> {
    let (a, b) = (&made[c].0, &made[d].0);
    let mut pivot: Option<Literal> = None;
    for &l in a.literals() {
        if b.contains(l.negated()) {
            if pivot.is_some() {
                return None;
            }
            pivot = Some(l);
        }
    }
    let pivot = pivot?;
    // `pivot` from `a`, its negation from `b`, as the rule takes them.
    let literals = a
        .literals()
        .iter()
        .copied()
        .filter(|&l| l != pivot)
        .chain(
            b.literals()
                .iter()
                .copied()
                .filter(|&l| l != pivot.negated()),
        )
        .collect();
    let origin = if pivot.is_positive() {
        Origin::Resolved(c, d, pivot.atom())
    } else {
        Origin::Resolved(d, c, pivot.atom())
    };
    Some((Clause::new(literals), origin))
}

/// Returns the refutation of the empty clause `end` of the search: the
/// resolvents it comes from, in the order they were made.
fn refutation(
    clauses: &Clauses,
    made: &[(Clause, Origin)],
    end: usize,
) -> Result<Refutation, Error> {
    let mut needed = vec![false; made.len()];
    let mut stack = vec![end];
    while let Some(k) = stack.pop() {
        if std::mem::replace(&mut needed[k], true) {
            continue;
        }
        if let Origin::Resolved(a, b, _) = made[k].1 {
            stack.extend([a, b]);
        }
    }
    let mut refutation = Refutation::new(clauses.clone());
    let mut number = vec![0usize; made.len()];
    for k in 0..made.len() {
        if !needed[k] {
            continue;
        }
        number[k] = match made[k].1 {
            Origin::Given(g) => g,
            Origin::Resolved(a, b, atom) => refutation.resolve(number[a], number[b], atom)?,
        };
    }
    Ok(refutation)
}
