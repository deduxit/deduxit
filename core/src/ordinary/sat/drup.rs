// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Clausal proofs of unsatisfiability in the DRUP format (clauses added
//! and deleted, each added clause a consequence of the clauses before it
//! by unit propagation), and a checker of them.
//!
//! A clause `C` follows from clauses `F` by reverse unit propagation (RUP)
//! when assigning every literal of `C` false and propagating the unit
//! clauses of `F` meets a clause with every literal false. Propagation
//! only assigns what `F` and that assignment imply, so `F` entails `C`.
//! The checker keeps the clauses of the formula and of the proof that
//! are not deleted, checks each added clause by RUP against them, and
//! accepts the proof at the first empty clause it adds: every clause it
//! keeps follows from the formula, so the formula entails the empty
//! clause and is unsatisfiable. Deleting a clause only weakens what is
//! kept, so deletions need no check, and a deletion of a clause that is
//! not kept is ignored, as DRAT checkers do.
//!
//! Each check starts from no assignment, propagates the unit clauses
//! kept and the negated clause, and undoes everything after, so nothing
//! of one check carries over to the next but the clauses and their
//! watches. A clause's two watched literals are its first two, which
//! with nothing assigned may be any two.

use super::solver::Lit;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::search::memory::Account;

/// A step of a clausal proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Step {
    /// A clause added, which must follow by RUP.
    Add(Box<[Lit]>),
    /// A clause deleted.
    Delete(Box<[Lit]>),
}

impl Step {
    /// Returns the bytes the step is charged to the memory account.
    pub(crate) const fn bytes(len: usize) -> usize {
        size_of::<Self>() + len * size_of::<Lit>()
    }
}

/// Why the checker refuses a proof: the proof is wrong, whatever the
/// bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Flaw {
    /// A literal of a clause names no variable of the formula.
    Variable {
        /// The step, by its index; `None` for a clause of the formula.
        step: Option<usize>,
    },
    /// A clause added does not follow by RUP.
    NotRup {
        /// The step, by its index.
        step: usize,
    },
    /// The proof never adds the empty clause.
    NoEmptyClause,
}

impl std::fmt::Display for Flaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable { step: None } => {
                write!(f, "a clause of the formula names no variable of it")
            }
            Self::Variable { step: Some(step) } => {
                write!(f, "step {step} names no variable of the formula")
            }
            Self::NotRup { step } => write!(
                f,
                "the clause of step {step} does not follow by unit propagation"
            ),
            Self::NoEmptyClause => write!(f, "the proof never adds the empty clause"),
        }
    }
}

/// Why a proof was not accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Rejection {
    /// The proof is wrong.
    Flaw(Flaw),
    /// A bound or the stop ended the check; the proof may be right.
    Refused(Refusal),
}

/// How many units of the checker's work (a step, a clause of the formula
/// kept, a unit clause assigned, a literal propagated, a visit of a
/// clause) lie between two polls of the stop.
const POLL: u64 = 1 << 12;

/// How a clause kept is charged to the memory account besides 8 bytes per
/// literal (its literals in the clause and in the index's key): the clause,
/// the index's entry (its key, its vector of indices, a slot of the
/// table), two watches with the slack of their vectors, and the headers of
/// three allocations.
const CLAUSE_BYTES: usize = 224;

/// A clause the checker keeps.
struct Clause {
    /// Its literals, by code ([`code`]), without repeats; the first two are
    /// watched.
    literals: Box<[u32]>,
    /// Whether it is kept, not deleted. A deleted clause leaves the watch
    /// lists when propagation next meets it.
    live: bool,
}

/// Returns a literal's code: twice its variable, plus one for a negation.
/// Codes index the assignment and the watch lists. The variable is below
/// the formula's count, which the solver keeps below `u32::MAX / 2`
/// (`MOST_VARIABLES`), so the code fits a `u32`.
const fn code(literal: Lit) -> u32 {
    literal.var() * 2 + !literal.is_positive() as u32
}

/// The checker's state: the clauses kept, their watches, and the
/// assignment of one check.
struct Checker<'a> {
    /// Every clause added, kept or deleted, by index. An index is below
    /// the vector's length, which memory bounds far below `usize::MAX`.
    clauses: Vec<Clause>,
    /// The clauses kept, by their sorted literals, for deletions.
    index: HashMap<Box<[u32]>, Vec<usize>>,
    /// Per literal code, the clauses of two or more literals watching it.
    watches: Vec<Vec<usize>>,
    /// The clauses of one literal, kept or deleted.
    units: Vec<usize>,
    /// How many empty clauses are kept.
    empty: usize,
    /// Per literal code, whether the check assigned it true.
    holds: Vec<bool>,
    /// The literals the check assigned true, in order.
    trail: Vec<u32>,
    /// The variables of the formula.
    variables: u32,
    /// The memory account the clauses are charged to.
    account: &'a Account,
    /// Steps and visits of a clause since the stop was last asked, at
    /// most `POLL`.
    ticks: u64,
    /// The steps checked, at most `Limits::work`, which a `u64` holds
    /// since each costs a step of the proof in memory.
    steps: u64,
}

/// Checks that `proof` refutes the clauses `formula` over `variables`
/// variables: each clause added follows by RUP from the formula's and the
/// proof's clauses kept before it, and an empty clause is added. A step
/// is a unit of `limits.work`; the clauses kept are charged to `account`,
/// which refuses past `limits.memory_bytes`; the stop is asked every
/// [`POLL`] units of the checker's work.
pub(crate) fn check(
    variables: u32,
    formula: &[Vec<Lit>],
    proof: &[Step],
    limits: &Limits,
    account: &Account,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<(), Rejection> {
    let codes = 2 * variables as usize;
    // Saturating, the charge refuses on a 32-bit target before the
    // vectors are made, never wraps.
    account.charge(codes.saturating_mul(size_of::<Vec<usize>>() + size_of::<bool>()));
    if account.over() {
        return Err(over(account));
    }
    let mut checker = Checker {
        clauses: Vec::new(),
        index: HashMap::default(),
        watches: vec![Vec::new(); codes],
        units: Vec::new(),
        empty: 0,
        holds: vec![false; codes],
        trail: Vec::new(),
        variables,
        account,
        ticks: 0,
        steps: 0,
    };
    for clause in formula {
        checker.tick(1, &mut stop)?;
        let literals = checker
            .normal(clause)
            .ok_or(Rejection::Flaw(Flaw::Variable { step: None }))?;
        checker.keep(literals)?;
    }
    for (n, step) in proof.iter().enumerate() {
        checker.steps += 1;
        if limits.work.is_some_and(|work| checker.steps > work) {
            return Err(Rejection::Refused(Refusal::Work {
                limit: limits.work.unwrap_or(u64::MAX),
            }));
        }
        checker.tick(1, &mut stop)?;
        let (Step::Add(clause) | Step::Delete(clause)) = step;
        let literals = checker
            .normal(clause)
            .ok_or(Rejection::Flaw(Flaw::Variable { step: Some(n) }))?;
        match step {
            Step::Add(_) => {
                if !checker.conflicts(clause, &mut stop)? {
                    return Err(Rejection::Flaw(Flaw::NotRup { step: n }));
                }
                if clause.is_empty() {
                    return Ok(());
                }
                checker.keep(literals)?;
            }
            Step::Delete(_) => checker.delete(literals),
        }
    }
    Err(Rejection::Flaw(Flaw::NoEmptyClause))
}

impl Checker<'_> {
    /// Returns the clause's codes sorted without repeats, `Some(None)`
    /// within for a tautology, which holds under every assignment and is
    /// never kept; `None` where a literal names no variable.
    #[expect(
        clippy::option_option,
        reason = "a clause of a missing variable, a tautology and a clause are three answers"
    )]
    fn normal(&self, clause: &[Lit]) -> Option<Option<Box<[u32]>>> {
        if clause.iter().any(|l| l.var() >= self.variables) {
            return None;
        }
        let mut literals: Vec<u32> = clause.iter().map(|&l| code(l)).collect();
        literals.sort_unstable();
        literals.dedup();
        // Sorted, a literal and its negation are neighbours.
        let tautology = literals.windows(2).any(|w| w[0] ^ 1 == w[1]);
        Some((!tautology).then(|| literals.into_boxed_slice()))
    }

    /// Keeps a clause, sorted without repeats, unless it is a tautology,
    /// charging it to the account.
    fn keep(&mut self, literals: Option<Box<[u32]>>) -> Result<(), Rejection> {
        let Some(literals) = literals else {
            return Ok(());
        };
        let id = self.clauses.len();
        self.account
            .charge(2 * literals.len() * size_of::<u32>() + CLAUSE_BYTES);
        match literals.len() {
            0 => self.empty += 1,
            1 => self.units.push(id),
            _ => {
                self.watches[literals[0] as usize].push(id);
                self.watches[literals[1] as usize].push(id);
            }
        }
        self.index.entry(literals.clone()).or_default().push(id);
        self.clauses.push(Clause {
            literals,
            live: true,
        });
        if self.account.over() {
            return Err(over(self.account));
        }
        Ok(())
    }

    /// Deletes one kept copy of a clause, sorted without repeats; a clause
    /// not kept, or a tautology, is ignored.
    fn delete(&mut self, literals: Option<Box<[u32]>>) {
        let Some(literals) = literals else {
            return;
        };
        let Some(ids) = self.index.get_mut(&literals) else {
            return;
        };
        let Some(id) = ids.pop() else {
            return;
        };
        if ids.is_empty() {
            self.index.remove(&literals);
        }
        self.clauses[id].live = false;
        if literals.is_empty() {
            self.empty -= 1;
        }
    }

    /// Counts `n` units of work, asking the stop once [`POLL`] of them
    /// have passed since it was last asked.
    fn tick(&mut self, n: u64, stop: &mut impl FnMut(Progress) -> bool) -> Result<(), Rejection> {
        // At most `POLL` before and a vector's length added: no wrap.
        self.ticks += n;
        if self.ticks >= POLL {
            let ticks = std::mem::take(&mut self.ticks);
            if stop(Progress::new(Phase::Check, ticks, self.steps)) {
                return Err(Rejection::Refused(Refusal::Stopped {
                    phase: Phase::Check,
                }));
            }
        }
        Ok(())
    }

    /// Assigns the literal true; returns `false` where its negation holds
    /// already, a conflict.
    fn assign(&mut self, literal: u32) -> bool {
        if self.holds[(literal ^ 1) as usize] {
            return false;
        }
        if !self.holds[literal as usize] {
            self.holds[literal as usize] = true;
            self.trail.push(literal);
        }
        true
    }

    /// Returns whether assigning every literal of `clause` false and
    /// propagating the unit clauses kept meets a conflict, undoing the
    /// assignment after.
    fn conflicts(
        &mut self,
        clause: &[Lit],
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<bool, Rejection> {
        let found = self.propagate(clause, stop);
        for &literal in &self.trail {
            self.holds[literal as usize] = false;
        }
        self.trail.clear();
        found
    }

    /// Assigns the negation of every literal of `clause` and of the unit
    /// clauses kept, then propagates; returns whether a conflict was met.
    fn propagate(
        &mut self,
        clause: &[Lit],
        stop: &mut impl FnMut(Progress) -> bool,
    ) -> Result<bool, Rejection> {
        if self.empty > 0 {
            return Ok(true);
        }
        for &literal in clause {
            if !self.assign(code(literal) ^ 1) {
                return Ok(true);
            }
        }
        // Deleted unit clauses leave the list here.
        let mut units = std::mem::take(&mut self.units);
        units.retain(|&id| self.clauses[id].live);
        let mut conflict = Ok(false);
        for &id in &units {
            if let Err(refused) = self.tick(1, stop) {
                conflict = Err(refused);
                break;
            }
            if !self.assign(self.clauses[id].literals[0]) {
                conflict = Ok(true);
                break;
            }
        }
        self.units = units;
        if conflict != Ok(false) {
            return conflict;
        }
        let mut head = 0;
        while let Some(&holds) = self.trail.get(head) {
            head += 1;
            self.tick(1, stop)?;
            let falsified = holds ^ 1;
            let mut watching = std::mem::take(&mut self.watches[falsified as usize]);
            let mut kept = 0;
            let mut at = 0;
            let mut conflict = false;
            while at < watching.len() {
                let id = watching[at];
                at += 1;
                if let Err(refused) = self.tick(1, stop) {
                    // Keep the watches whole before giving up.
                    watching.drain(kept..at - 1);
                    self.watches[falsified as usize] = watching;
                    return Err(refused);
                }
                let clause = &mut self.clauses[id];
                if !clause.live {
                    continue;
                }
                if clause.literals[0] == falsified {
                    clause.literals.swap(0, 1);
                }
                let first = clause.literals[0];
                if self.holds[first as usize] {
                    watching[kept] = id;
                    kept += 1;
                    continue;
                }
                let other = clause.literals[2..]
                    .iter()
                    .position(|&l| !self.holds[(l ^ 1) as usize]);
                if let Some(k) = other {
                    clause.literals.swap(1, k + 2);
                    let watched = clause.literals[1];
                    self.watches[watched as usize].push(id);
                    continue;
                }
                watching[kept] = id;
                kept += 1;
                if !self.assign(first) {
                    conflict = true;
                    break;
                }
            }
            // The clauses not visited stay watched.
            watching.drain(kept..at);
            self.watches[falsified as usize] = watching;
            if conflict {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// The refusal of a check past the memory account's bound.
fn over(account: &Account) -> Rejection {
    Rejection::Refused(Refusal::Memory {
        phase: Phase::Check,
        limit_bytes: account.limit(),
        needed_bytes: None,
    })
}

/// Writes the formula in DIMACS's CNF format and the proof in DRUP's text
/// format, for an outside checker.
#[cfg(all(test, feature = "parse"))]
pub(crate) fn dimacs(variables: u32, formula: &[Vec<Lit>], proof: &[Step]) -> (String, String) {
    use std::fmt::Write;
    let number = |l: Lit| {
        let n = i64::from(l.var()) + 1;
        if l.is_positive() { n } else { -n }
    };
    let mut cnf = format!("p cnf {variables} {}\n", formula.len());
    for clause in formula {
        for &l in clause {
            write!(cnf, "{} ", number(l)).unwrap();
        }
        cnf.push_str("0\n");
    }
    let mut drup = String::new();
    for step in proof {
        let (Step::Add(clause) | Step::Delete(clause)) = step;
        if matches!(step, Step::Delete(_)) {
            drup.push_str("d ");
        }
        for &l in clause.iter() {
            write!(drup, "{} ", number(l)).unwrap();
        }
        drup.push_str("0\n");
    }
    (cnf, drup)
}

#[cfg(test)]
mod tests {
    use super::{Flaw, Rejection, Step, check};
    use crate::limits::{Limits, Refusal};
    use crate::linear::search::memory::Account;
    use crate::ordinary::sat::solver::Lit;

    /// The literal of a DIMACS number.
    fn lit(n: i32) -> Lit {
        let var = n.unsigned_abs() - 1;
        if n > 0 {
            Lit::positive(var)
        } else {
            Lit::negative(var)
        }
    }

    /// The clauses of DIMACS numbers.
    fn clauses(numbers: &[&[i32]]) -> Vec<Vec<Lit>> {
        numbers
            .iter()
            .map(|c| c.iter().map(|&n| lit(n)).collect())
            .collect()
    }

    /// A step adding the clause.
    fn add(numbers: &[i32]) -> Step {
        Step::Add(numbers.iter().map(|&n| lit(n)).collect())
    }

    /// A step deleting the clause.
    fn delete(numbers: &[i32]) -> Step {
        Step::Delete(numbers.iter().map(|&n| lit(n)).collect())
    }

    /// Checks the proof within the default limits.
    fn run(variables: u32, formula: &[&[i32]], proof: &[Step]) -> Result<(), Rejection> {
        let limits = Limits::default();
        let account = Account::holding(limits.memory_bytes, 0);
        check(
            variables,
            &clauses(formula),
            proof,
            &limits,
            &account,
            |_| false,
        )
    }

    /// Every pair of two variables' literals as a clause: unsatisfiable,
    /// refuted by learning `x₁` and then the empty clause.
    const SQUARE: &[&[i32]] = &[&[1, 2], &[1, -2], &[-1, 2], &[-1, -2]];

    /// A right proof is accepted; a planted wrong step, a missing empty
    /// clause, a deletion of a clause a later step needs and an unknown
    /// variable are refused, each for its reason.
    #[test]
    fn a_planted_wrong_step_is_refused() {
        assert_eq!(run(2, SQUARE, &[add(&[1]), add(&[])]), Ok(()));
        // A clause deleted that nothing kept, and a tautology, are ignored.
        let ignored = [delete(&[1, 2, -2]), delete(&[2]), add(&[1]), add(&[])];
        assert_eq!(run(2, SQUARE, &ignored), Ok(()));
        // `x₁ ∨ x₂` alone does not give `x₂` by propagation.
        let wrong = [add(&[2]), add(&[1]), add(&[])];
        let partial: &[&[i32]] = &[&[1, 2], &[1, -2], &[-1, -2]];
        assert_eq!(
            run(2, partial, &wrong),
            Err(Rejection::Flaw(Flaw::NotRup { step: 0 }))
        );
        assert_eq!(
            run(2, SQUARE, &[add(&[1])]),
            Err(Rejection::Flaw(Flaw::NoEmptyClause))
        );
        // Without `¬x₁ ∨ x₂` the empty clause no longer follows from `x₁`.
        let deleted = [add(&[1]), delete(&[-1, 2]), add(&[])];
        assert_eq!(
            run(2, SQUARE, &deleted),
            Err(Rejection::Flaw(Flaw::NotRup { step: 2 }))
        );
        assert_eq!(
            run(2, SQUARE, &[add(&[3]), add(&[])]),
            Err(Rejection::Flaw(Flaw::Variable { step: Some(0) }))
        );
        assert_eq!(
            run(1, SQUARE, &[add(&[])]),
            Err(Rejection::Flaw(Flaw::Variable { step: None }))
        );
    }

    /// The work, the memory and the stop refuse a check, none a flaw.
    #[test]
    fn the_bounds_refuse_a_check() {
        let proof = [add(&[1]), add(&[])];
        let limits = Limits::default().with_work(Some(1));
        let account = Account::holding(limits.memory_bytes, 0);
        let formula = clauses(SQUARE);
        assert_eq!(
            check(2, &formula, &proof, &limits, &account, |_| false),
            Err(Rejection::Refused(Refusal::Work { limit: 1 }))
        );
        let limits = Limits::default().with_memory_bytes(Some(64));
        let account = Account::holding(limits.memory_bytes, 0);
        assert!(matches!(
            check(2, &formula, &proof, &limits, &account, |_| false),
            Err(Rejection::Refused(Refusal::Memory { .. }))
        ));
        // `x₁`, a chain of implications `xᵢ → xᵢ₊₁` and `¬xₙ`, long enough
        // that the one check visits more clauses than two polls' worth.
        let n = 3 * super::POLL as i32;
        let mut chain: Vec<Vec<i32>> = (1..n).map(|i| vec![-i, i + 1]).collect();
        chain.push(vec![1]);
        chain.push(vec![-n]);
        let chain: Vec<&[i32]> = chain.iter().map(Vec::as_slice).collect();
        let limits = Limits::default();
        let account = Account::holding(limits.memory_bytes, 0);
        let mut asked = 0;
        let result = check(
            n as u32,
            &clauses(&chain),
            &[add(&[])],
            &limits,
            &account,
            |_| {
                asked += 1;
                asked > 1
            },
        );
        assert_eq!(
            result,
            Err(Rejection::Refused(Refusal::Stopped {
                phase: crate::limits::Phase::Check
            }))
        );
        assert_eq!(run(n as u32, &chain, &[add(&[])]), Ok(()));
        // Many unit clauses, `x₁` last: each check assigns them all before
        // the conflict, and asks the stop at least once on the way.
        let n = super::POLL as i32;
        let mut units: Vec<Vec<i32>> = (2..=n).map(|i| vec![i]).collect();
        units.push(vec![1]);
        let units: Vec<&[i32]> = units.iter().map(Vec::as_slice).collect();
        let steps = 8;
        let proof: Vec<Step> = (0..steps).map(|_| add(&[1, n + 1])).collect();
        let mut asked = 0;
        let result = check(
            n as u32 + 1,
            &clauses(&units),
            &proof,
            &limits,
            &account,
            |_| {
                asked += 1;
                false
            },
        );
        assert_eq!(result, Err(Rejection::Flaw(Flaw::NoEmptyClause)));
        assert!(asked >= steps, "{asked} polls for {steps} checks");
    }
}
