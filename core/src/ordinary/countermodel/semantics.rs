// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What the check of a finite structure and that of a Kripke model with
//! growing domains share: one world's reading of terms and atoms, with
//! `=` read as identity, and the meter of evaluations. Each check keeps its
//! own walk over the formulas (truth, or forcing over the worlds above);
//! a change of first-order semantics, such as equality, is made here once.

use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};
use crate::limits::{Phase, Progress, Refusal};
use crate::ordinary::Sequent;

/// The bytes a constant, a table or a tuple holds in a model and in a
/// check's tables beside 4 bytes an element: its vector and its entry in a
/// set.
pub(super) const ENTRY_BYTES: u64 = 48;

/// Returns the bytes a world's function tables and predicates hold, by
/// [`ENTRY_BYTES`] and 4 bytes an element, saturating.
pub(super) fn tables_bytes(
    functions: &[(String, Vec<u32>)],
    predicates: &[(String, Vec<Vec<u32>>)],
) -> u64 {
    let tables = functions.iter().fold(0u64, |held, (_, table)| {
        held.saturating_add(ENTRY_BYTES)
            .saturating_add((table.len() as u64).saturating_mul(4))
    });
    predicates.iter().fold(tables, |held, (_, tuples)| {
        tuples.iter().fold(held, |held, tuple| {
            held.saturating_add(ENTRY_BYTES)
                .saturating_add((tuple.len() as u64).saturating_mul(4))
        })
    })
}

/// The evaluations a check makes, held to its bound, and its stop.
pub(super) struct Meter<'s> {
    /// The evaluations made.
    work: u64,
    /// The most evaluations allowed.
    most: u64,
    /// The caller's stop.
    stop: &'s mut dyn FnMut(Progress) -> bool,
}

impl<'s> Meter<'s> {
    /// Returns a meter of no evaluations, at most `most`, asking `stop`.
    pub(super) fn new(most: u64, stop: &'s mut dyn FnMut(Progress) -> bool) -> Self {
        Self {
            work: 0,
            most,
            stop,
        }
    }

    /// Counts `units` evaluations, refusing past the bound and asking the
    /// stop each time the count passes a multiple of 4 096.
    pub(super) fn tick_by(&mut self, units: u64) -> Result<(), Error> {
        let before = self.work;
        self.work = self.work.saturating_add(units);
        if self.work > self.most {
            return Err(Error::Refused(Refusal::Work { limit: self.most }));
        }
        if self.work / 4_096 != before / 4_096
            && (self.stop)(Progress::new(Phase::Check, 4_096, self.work))
        {
            return Err(Error::Refused(Refusal::stopped(Phase::Check)));
        }
        Ok(())
    }
}

/// One world's reading of the sequent's terms and atoms: its domain's
/// size, the constants' elements (one for every world), its function
/// tables and its predicates' tuples.
pub(super) struct Interpretation<'a, 'w> {
    /// The domain's size: the elements are `0 .. size`.
    pub(super) size: u32,
    /// Each constant's element.
    pub(super) elements: &'w HashMap<&'a str, u32>,
    /// Each function's table, checked up front to hold `size^arity`
    /// values.
    pub(super) functions: &'w HashMap<&'a str, &'a [u32]>,
    /// Each predicate's tuples.
    pub(super) holds: &'w HashMap<&'a str, HashSet<&'a [u32]>>,
}

impl Interpretation<'_, '_> {
    /// Returns whether the atom `a` of `sequent` holds under the bound
    /// elements `env`, the innermost last. Equality is identity: an
    /// element is the same at every world that has it, and terms keep
    /// their values along the order, so identity grows as a predicate must
    /// and satisfies every equality axiom.
    pub(super) fn atom(
        &self,
        sequent: &Sequent,
        a: u32,
        env: &[u32],
        meter: &mut Meter<'_>,
        wrong: &dyn Fn(String) -> Error,
    ) -> Result<bool, Error> {
        let formulas = sequent.formulas();
        let tables = formulas.first_order();
        let (name, args): (&str, &[FoTermId]) = match tables.atoms.get(a as usize) {
            Some(&(symbol, args)) => (tables.symbols[symbol.index()].0.as_str(), tables.list(args)),
            None => (formulas.atom_name(a), &[]),
        };
        let mut tuple = Vec::with_capacity(args.len());
        for &t in args {
            tuple.push(self.term(sequent, t, env, meter, wrong)?);
        }
        if let [left, right] = tuple[..]
            && name == crate::first_order::EQUALS
        {
            return Ok(left == right);
        }
        Ok(self
            .holds
            .get(name)
            .is_some_and(|set| set.contains(tuple.as_slice())))
    }

    /// Returns the element the term `t` of `sequent` denotes under `env`,
    /// the innermost binding last. A term is evaluated with a stack of its
    /// own: terms may nest as deep as a document is long. Each application
    /// of a function symbol is an evaluation of `1 + arity` units: a
    /// function may take millions of arguments, and a term read from JSON
    /// may share its subterms and is read as the tree it stands for.
    pub(super) fn term(
        &self,
        sequent: &Sequent,
        t: FoTermId,
        env: &[u32],
        meter: &mut Meter<'_>,
        wrong: &dyn Fn(String) -> Error,
    ) -> Result<u32, Error> {
        let tables = sequent.formulas().first_order();
        let size = self.size as usize;
        let mut values: Vec<u32> = Vec::new();
        // Each entry: a term, and whether its arguments' values are on
        // `values` already.
        let mut stack = vec![(t, false)];
        while let Some((t, applied)) = stack.pop() {
            match tables.terms[t.index()] {
                FoTerm::Bound(index) => {
                    let at = env
                        .len()
                        .checked_sub(index as usize + 1)
                        .ok_or_else(|| wrong("meets a variable no quantifier binds".to_owned()))?;
                    values.push(env[at]);
                }
                FoTerm::App(f, args) => {
                    let name = tables.symbols[f.index()].0.as_str();
                    let args = tables.list(args);
                    if args.is_empty() {
                        let e = self.elements.get(name).copied().ok_or_else(|| {
                            wrong(format!("names no element for the constant {name}"))
                        })?;
                        values.push(e);
                    } else if applied {
                        meter.tick_by(1 + args.len() as u64)?;
                        let table = self.functions.get(name).ok_or_else(|| {
                            wrong(format!("has no table for the function {name}"))
                        })?;
                        let start = values.len() - args.len();
                        let index = values
                            .drain(start..)
                            .fold(0usize, |i, e| i * size + e as usize);
                        let e = table
                            .get(index)
                            .copied()
                            .ok_or_else(|| wrong(format!("gives {name} too short a table")))?;
                        values.push(e);
                    } else {
                        stack.push((t, true));
                        for &a in args.iter().rev() {
                            stack.push((a, false));
                        }
                    }
                }
                FoTerm::Eigen(_) | FoTerm::Meta(_) => {
                    return Err(wrong(
                        "cannot interpret an eigenvariable or an open witness".to_owned(),
                    ));
                }
            }
        }
        Ok(values.pop().expect("the term's value"))
    }
}
