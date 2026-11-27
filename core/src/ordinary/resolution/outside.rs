// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Refutations that outside SAT solvers write, in the DRAT and LRAT
//! formats, read and checked by the checker of the crate's own SAT
//! solver.
//!
//! A DRAT proof is a list of clauses added and deleted; an LRAT proof
//! numbers each clause and gives hints, the clauses each addition uses.
//! The checker checks every addition by reverse unit propagation (RUP)
//! against the clauses kept, LRAT's hints left aside, and accepts at the
//! first empty clause added. An addition that is only RAT (resolution
//! asymmetric tautology, which extended techniques use) fails the check,
//! since the checker has no RAT step.

use super::{Clauses, Literal};
use crate::Error;
use crate::limits::{Limits, Progress, Refusal, Space};
use crate::memory::Account;
use crate::ordinary::sat::ClausalProof;
use crate::ordinary::sat::drup::{Flaw, Rejection, Step};
use crate::ordinary::sat::solver::{Lit, MOST_VARIABLES};

/// The format of a proof.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum ProofFormat {
    /// DRAT, as text (`d` before a clause deleted) or in its binary form
    /// (bytes `a` and `d`, then each literal a variable-length number),
    /// told apart by the bytes.
    #[default]
    Drat,
    /// LRAT as text: `ID LITERALS 0 HINTS 0` for a clause added, `ID d IDS 0`
    /// for clauses deleted, the set's clauses numbered from 1.
    Lrat,
}

/// Checks `proof`, written in `format`, as a refutation of `clauses`, and
/// returns what was checked.
///
/// # Errors
///
/// [`Error::ClauseFile`] for a proof that is not of the format;
/// [`Error::InvalidRefutation`] for an addition that does not follow by
/// unit propagation, a literal of no atom of the set, or no empty clause;
/// [`Refusal::Occurrences`] past `limits.occurrences` literals,
/// [`Refusal::Memory`], [`Refusal::Work`] and [`Refusal::Stopped`] as the
/// checker gives them; [`Refusal::Index`] for a set of more atoms than
/// the checker numbers.
pub fn check_proof(
    clauses: &Clauses,
    proof: &[u8],
    format: ProofFormat,
    limits: &Limits,
    stop: impl FnMut(Progress) -> bool,
) -> Result<ClausalProof, Error> {
    let atoms = clauses.atoms().len();
    if atoms > MOST_VARIABLES as usize {
        return Err(Error::Refused(Refusal::Index {
            what: Space::Atom,
            count: atoms as u64,
            most: u64::from(MOST_VARIABLES),
        }));
    }
    let lit = |l: &Literal| {
        if l.is_positive() {
            Lit::positive(l.atom())
        } else {
            Lit::negative(l.atom())
        }
    };
    let formula: Vec<Vec<Lit>> = clauses
        .clauses()
        .iter()
        .map(|c| c.literals().iter().map(lit).collect())
        .collect();
    let account = Account::holding(limits.memory_bytes, 0);
    let steps = match format {
        ProofFormat::Drat if is_binary(proof) => binary_drat(proof, limits, &account)?,
        ProofFormat::Drat => text_drat(text(proof)?, limits, &account)?,
        ProofFormat::Lrat => lrat(text(proof)?, &formula, limits, &account)?,
    };
    // `atoms ≤ MOST_VARIABLES`, checked above, fits a `u32`.
    ClausalProof::checked(atoms as u32, &formula, &steps, limits, &account, stop).map_err(
        |rejection| match rejection {
            Rejection::Refused(refusal) => Error::Refused(refusal),
            Rejection::Flaw(flaw) => Error::InvalidRefutation {
                step: match flaw {
                    Flaw::NotRup { step } | Flaw::Variable { step: Some(step) } => {
                        Some(step as u64 + 1)
                    }
                    Flaw::Variable { step: None } | Flaw::NoEmptyClause => None,
                },
                reason: match flaw {
                    Flaw::NotRup { .. } => {
                        "the clause it adds does not follow by unit propagation".to_owned()
                    }
                    Flaw::Variable { step: Some(_) } => {
                        "it names a variable past the clauses'".to_owned()
                    }
                    Flaw::Variable { step: None } => {
                        "a clause names a variable past the clauses'".to_owned()
                    }
                    Flaw::NoEmptyClause => "the proof never adds the empty clause".to_owned(),
                },
            },
        },
    )
}

/// Returns the proof as text.
fn text(proof: &[u8]) -> Result<&str, Error> {
    std::str::from_utf8(proof).map_err(|e| Error::ClauseFile {
        at: line_of(proof, e.valid_up_to()),
        reason: "the proof is not text".to_owned(),
    })
}

/// Returns the line, counted from 1, of the byte at `offset`.
fn line_of(text: &[u8], offset: usize) -> u64 {
    text[..offset.min(text.len())]
        .iter()
        .filter(|&&b| b == b'\n')
        .count() as u64
        + 1
}

/// Returns whether a DRAT proof is in the binary form: it starts with
/// `a`, which starts no text proof, or one of its first bytes is a control
/// character other than a blank or a byte past ASCII, which no text proof
/// holds.
fn is_binary(proof: &[u8]) -> bool {
    proof.first() == Some(&b'a')
        || proof
            .iter()
            .take(64)
            .any(|&b| b >= 0x80 || (b < 0x20 && !b.is_ascii_whitespace()))
}

/// Counts `n` literals against `limits.occurrences` and charges a step's
/// bytes.
fn charge(size: &mut u64, n: usize, limits: &Limits, account: &Account) -> Result<(), Error> {
    *size = size.saturating_add(n as u64 + 1);
    if let Some(limit) = limits.occurrences
        && *size > limit
    {
        return Err(Error::Refused(Refusal::Occurrences {
            occurrences: *size,
            limit,
        }));
    }
    account.charge(Step::bytes(n));
    if account.over() {
        return Err(Error::Refused(Refusal::Memory {
            phase: crate::limits::Phase::Read,
            limit_bytes: account.limit(),
            needed_bytes: None,
        }));
    }
    Ok(())
}

/// Returns the literal a DIMACS number names, or `None` for one past what
/// the checker numbers; the checker refuses a variable past the set's.
fn literal(number: i64) -> Option<Lit> {
    let var = u32::try_from(number.unsigned_abs().checked_sub(1)?).ok()?;
    (var < MOST_VARIABLES).then(|| {
        if number > 0 {
            Lit::positive(var)
        } else {
            Lit::negative(var)
        }
    })
}

/// Reads a DRAT proof written as text.
fn text_drat(text: &str, limits: &Limits, account: &Account) -> Result<Vec<Step>, Error> {
    let mut steps = Vec::new();
    let mut open: Vec<Lit> = Vec::new();
    let mut delete = false;
    let mut size = 0u64;
    for (n, line) in text.lines().enumerate() {
        let fault = |reason: String| Error::ClauseFile {
            at: n as u64 + 1,
            reason,
        };
        let line = line.trim();
        if line.starts_with('c') {
            continue;
        }
        for word in line.split_whitespace() {
            if word == "d" && open.is_empty() && !delete {
                delete = true;
                continue;
            }
            let number: i64 = word
                .parse()
                .map_err(|_| fault(format!("`{word}` is no literal")))?;
            if number == 0 {
                charge(&mut size, open.len(), limits, account)?;
                let clause = std::mem::take(&mut open).into_boxed_slice();
                steps.push(if delete {
                    Step::Delete(clause)
                } else {
                    Step::Add(clause)
                });
                delete = false;
                continue;
            }
            open.push(literal(number).ok_or_else(|| fault(format!("{number} is too large")))?);
        }
    }
    if !open.is_empty() || delete {
        return Err(Error::ClauseFile {
            at: text.lines().count() as u64,
            reason: "the last clause is not ended by 0".to_owned(),
        });
    }
    Ok(steps)
}

/// Reads a DRAT proof in the binary form.
fn binary_drat(proof: &[u8], limits: &Limits, account: &Account) -> Result<Vec<Step>, Error> {
    let fault = |at: usize, reason: &str| Error::ClauseFile {
        at: at as u64 + 1,
        reason: format!("{reason} (the byte counted from 1)"),
    };
    let mut steps = Vec::new();
    let mut size = 0u64;
    let mut at = 0;
    while at < proof.len() {
        let delete = match proof[at] {
            b'a' => false,
            b'd' => true,
            _ => return Err(fault(at, "a step starts with neither `a` nor `d`")),
        };
        at += 1;
        let mut clause = Vec::new();
        loop {
            // A number of seven bits a byte, the lowest first; the high
            // bit says another byte follows.
            let mut number = 0u64;
            let mut shift = 0;
            loop {
                let Some(&byte) = proof.get(at) else {
                    return Err(fault(at, "the proof ends inside a step"));
                };
                at += 1;
                if shift > 35 {
                    return Err(fault(at, "a literal is too large"));
                }
                number |= u64::from(byte & 0x7f) << shift;
                shift += 7;
                if byte & 0x80 == 0 {
                    break;
                }
            }
            if number == 0 {
                break;
            }
            // `2·v + 1` is `-v`, `2·v` is `v`; `number < 2^42` fits `i64`.
            let var = (number >> 1) as i64;
            let signed = if number & 1 == 1 { -var } else { var };
            clause.push(literal(signed).ok_or_else(|| fault(at, "a literal is too large"))?);
        }
        charge(&mut size, clause.len(), limits, account)?;
        let clause = clause.into_boxed_slice();
        steps.push(if delete {
            Step::Delete(clause)
        } else {
            Step::Add(clause)
        });
    }
    Ok(steps)
}

/// Reads an LRAT proof written as text: each clause added by its number,
/// deletions by number, the hints left aside.
fn lrat(
    text: &str,
    formula: &[Vec<Lit>],
    limits: &Limits,
    account: &Account,
) -> Result<Vec<Step>, Error> {
    let mut numbered: crate::hash::HashMap<u64, Box<[Lit]>> = crate::hash::HashMap::default();
    for (k, clause) in formula.iter().enumerate() {
        numbered.insert(k as u64 + 1, clause.clone().into_boxed_slice());
    }
    let mut steps = Vec::new();
    let mut size = 0u64;
    for (n, line) in text.lines().enumerate() {
        let fault = |reason: String| Error::ClauseFile {
            at: n as u64 + 1,
            reason,
        };
        let line = line.trim();
        if line.is_empty() || line.starts_with('c') {
            continue;
        }
        let mut words = line.split_whitespace();
        let id: u64 = words
            .next()
            .and_then(|w| w.parse().ok())
            .ok_or_else(|| fault("a line starts with no clause number".to_owned()))?;
        let mut words = words.peekable();
        if words.peek() == Some(&"d") {
            words.next();
            for word in words {
                let gone: u64 = word
                    .parse()
                    .map_err(|_| fault(format!("`{word}` is no clause number")))?;
                if gone == 0 {
                    break;
                }
                if let Some(clause) = numbered.remove(&gone) {
                    charge(&mut size, clause.len(), limits, account)?;
                    steps.push(Step::Delete(clause));
                }
            }
            continue;
        }
        let mut clause = Vec::new();
        let mut ended = false;
        for word in words.by_ref() {
            let number: i64 = word
                .parse()
                .map_err(|_| fault(format!("`{word}` is no literal")))?;
            if number == 0 {
                ended = true;
                break;
            }
            clause.push(literal(number).ok_or_else(|| fault(format!("{number} is too large")))?);
        }
        if !ended {
            return Err(fault("a clause is not ended by 0".to_owned()));
        }
        if numbered.contains_key(&id) {
            return Err(fault(format!("the clause number {id} is taken")));
        }
        // The step and the copy the numbering keeps.
        charge(&mut size, clause.len(), limits, account)?;
        account.charge(Step::bytes(clause.len()));
        let clause = clause.into_boxed_slice();
        numbered.insert(id, clause.clone());
        steps.push(Step::Add(clause));
    }
    Ok(steps)
}
