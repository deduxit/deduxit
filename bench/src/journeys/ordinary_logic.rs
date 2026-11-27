// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The journeys of ordinary logic's own engines: the SAT method in
//! classical and intuitionistic logic, LJT, both connection provers,
//! propositional and first-order resolution, the finite and the Kripke
//! model search, and a session (`candidates`, `apply`, `close`).

use super::{deterministic, measured};
use anyhow::{Result, bail, ensure};
use deduxit::Limits;
use deduxit::linear::search::Options;
use deduxit::ordinary::{
    self, Logic, Sequent, Side, Status, Step, Verdict, connection, ljt, models, resolution, sat,
};

/// The pigeonhole principle for `n + 1` pigeons in `n` holes as a sequent:
/// each pigeon in some hole, so some hole holds two. Valid in classical
/// and in intuitionistic logic (ILTP's SYJ202).
pub fn pigeonhole(n: u32) -> String {
    let p = |i: u32, h: u32| format!("p{i}_{h}");
    let choices: Vec<String> = (0..=n)
        .map(|i| {
            let holes: Vec<String> = (0..n).map(|h| p(i, h)).collect();
            format!("({})", holes.join(" \\/ "))
        })
        .collect();
    let mut clashes = Vec::new();
    for h in 0..n {
        for i in 0..=n {
            for j in i + 1..=n {
                clashes.push(format!("({} /\\ {})", p(i, h), p(j, h)));
            }
        }
    }
    format!("{} |- {}", choices.join(", "), clashes.join(" \\/ "))
}

/// Fails unless an ordinary verdict is valid.
fn valid(verdict: &Verdict) -> Result<()> {
    match verdict {
        Verdict::Valid(_) => Ok(()),
        other => bail!("expected valid, got {other:?}"),
    }
}

/// Fails unless an ordinary verdict is not valid.
fn not_valid(verdict: &Verdict) -> Result<()> {
    match verdict {
        Verdict::NotValid => Ok(()),
        other => bail!("expected not valid, got {other:?}"),
    }
}

/// The SAT method in classical logic on a pigeonhole formula: the solver
/// and the check of its clausal proof.
pub fn sat_classical() -> Result<()> {
    let sequent: Sequent = pigeonhole(6).parse()?;
    let outcome =
        measured(|| sat::decide(&sequent, Logic::Classical, &Limits::default(), |_| false))?;
    valid(&outcome.verdict)
}

/// The SAT method in intuitionistic logic on ILTP's SYJ202: the solver's
/// loop over worlds, the derivation read off it and its check.
pub fn sat_intuitionistic() -> Result<()> {
    let sequent: Sequent = pigeonhole(4).parse()?;
    let outcome = measured(|| {
        sat::decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| {
            false
        })
    })?;
    valid(&outcome.verdict)
}

/// LJT on Korn and Kreitz's formula (ILTP's SYJ205) at `N`, valid: the
/// search, the derivation of LJ and its check.
pub fn ljt() -> Result<()> {
    const N: usize = 30;
    let chain: Vec<String> = (1..=N)
        .map(|i| format!("((b{} -> a{i}) -> a{})", i - 1, i - 1))
        .collect();
    let text = format!("a0 -> f, ((b{N} -> b0) -> a{N}), {} |- f", chain.join(", "));
    let sequent: Sequent = text.parse()?;
    let outcome = measured(|| {
        ljt::decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| {
            false
        })
    })?;
    valid(&outcome.verdict)
}

/// A first-order chain of `n` steps, `∀x. (p(x) → p(f(x))), p(a) ⊢
/// p(fⁿ(a))`, valid in every logic.
fn chain(n: usize) -> String {
    format!(
        "forall x. (p(x) -> p(f(x))), p(a) |- p({}a{})",
        "f(".repeat(n),
        ")".repeat(n)
    )
}

/// The classical connection prover on a first-order chain, with no bound
/// on the active path: the matrix, the search, the derivation of LK read
/// back and its check.
pub fn connection_classical() -> Result<()> {
    let sequent: Sequent = chain(24).parse()?;
    let search = Options::default().with_path_length(None);
    let outcome = measured(|| {
        connection::decide(
            &sequent,
            Logic::Classical,
            &search,
            &Limits::default(),
            |_| false,
        )
    })?;
    valid(&outcome.verdict)
}

/// The intuitionistic connection prover on a nest of implications whose
/// hypotheses are every atom of the nest: the prefixes, the search and
/// the derivation of LJ read back and checked.
pub fn connection_intuitionistic() -> Result<()> {
    const N: usize = 24;
    let atoms: Vec<String> = (0..N).map(|i| format!("a{i}")).collect();
    let nest: String = atoms
        .iter()
        .map(|a| format!("{a} -> ("))
        .collect::<String>()
        + "b"
        + &")".repeat(N);
    let sequent: Sequent = format!("{}, {nest} |- b", atoms.join(", ")).parse()?;
    let search = Options::default().with_path_length(None);
    let outcome = measured(|| {
        connection::decide(
            &sequent,
            Logic::Intuitionistic,
            &search,
            &Limits::default(),
            |_| false,
        )
    })?;
    valid(&outcome.verdict)
}

/// Propositional resolution by saturation (the default method) on the
/// pigeonhole clauses of `n + 1` pigeons in `n` holes, unsatisfiable: the
/// saturation and the check of its refutation.
pub fn resolution() -> Result<()> {
    const N: usize = 3;
    let var = |p: usize, h: usize| p * N + h + 1;
    let mut clauses: Vec<String> = Vec::new();
    for p in 0..=N {
        let holes: Vec<String> = (0..N).map(|h| var(p, h).to_string()).collect();
        clauses.push(format!("{} 0", holes.join(" ")));
    }
    for h in 0..N {
        for p in 0..=N {
            for q in p + 1..=N {
                clauses.push(format!("-{} -{} 0", var(p, h), var(q, h)));
            }
        }
    }
    let text = format!(
        "p cnf {} {}\n{}\n",
        (N + 1) * N,
        clauses.len(),
        clauses.join("\n")
    );
    let clauses = resolution::Clauses::from_dimacs(&text, &Limits::default())?;
    // Saturation keeps more clauses than its default bound of 2 000 here.
    let options = resolution::Options::default().with_clauses(Some(20_000));
    let answer =
        measured(|| resolution::decide(&clauses, &options, &Limits::default(), |_| false))?;
    match answer.verdict {
        resolution::Verdict::Refuted(_) => Ok(()),
        other => bail!("expected a refutation, got {other:?}"),
    }
}

/// First-order resolution on a first-order chain: the clausal form, the
/// given-clause search, the refutation's check and the derivation of LK
/// read back.
pub fn resolution_first_order() -> Result<()> {
    let sequent: Sequent = chain(16).parse()?;
    let options = resolution::Options::default();
    let answer = measured(|| {
        resolution::first_order::resolve(&sequent, &options, &Limits::default(), |_| false)
    })?;
    match answer.answer {
        resolution::first_order::Answer::Refuted { .. } => Ok(()),
        other => bail!("expected a refutation, got {other:?}"),
    }
}

/// The finite model search in classical logic on a sequent whose least
/// countermodel has three elements: the encodings per size, the solver,
/// and the structure's check.
pub fn models_finite() -> Result<()> {
    let sequent: Sequent = "forall x. r(x, f(x)) |- exists x. r(f(x), x)".parse()?;
    let search = Options::default();
    let outcome = measured(|| {
        models::decide(
            &sequent,
            Logic::Classical,
            &search,
            &Limits::default(),
            |_| false,
        )
    })?;
    not_valid(&outcome.verdict)
}

/// The Kripke model search in intuitionistic logic on a sequent that
/// holds classically: the encodings per number of worlds and elements,
/// the solver, and the model's check.
pub fn models_kripke() -> Result<()> {
    let sequent: Sequent = "(forall x. p(x)) -> q |- exists x. (p(x) -> q)".parse()?;
    let search = Options::default();
    let outcome = measured(|| {
        models::decide(
            &sequent,
            Logic::Intuitionistic,
            &search,
            &Limits::default(),
            |_| false,
        )
    })?;
    not_valid(&outcome.verdict)
}

/// A session of classical logic on a pigeonhole formula: every goal's
/// candidate rules at every formula, each invertible step that applies
/// taken, and the goals left closed by the search, each closing graft
/// checked.
pub fn session() -> Result<()> {
    let sequent: Sequent = pigeonhole(2).parse()?;
    let mut session = ordinary::Interactive::new(&sequent, Logic::Classical)?;
    let search = deterministic();
    let limits = Limits::default();
    measured(|| -> Result<()> {
        loop {
            let Some(goal) = session.goals().next() else {
                break;
            };
            let (left, right) = session.goal(goal)?;
            let (left, right) = (left.to_vec(), right.to_vec());
            let sides = [(Side::Left, left.len()), (Side::Right, right.len())];
            let mut taken = None;
            for (side, width) in sides {
                for position in 0..width {
                    for candidate in session.candidates(goal, side, position)? {
                        let step = Step::new(side, position, candidate.rule);
                        if taken.is_none()
                            && candidate.status == Status::Applies
                            && session.is_invertible((&left, &right), &step)
                        {
                            taken = Some(step);
                        }
                    }
                }
            }
            match taken {
                Some(step) => {
                    session.apply(goal, &step)?;
                }
                None => {
                    let closed = session.close(goal, None, &search, &limits, |_| false)?;
                    valid(&closed.outcome.verdict)?;
                    ensure!(closed.grafted.is_ok(), "the derivation was not grafted");
                }
            }
        }
        Ok(())
    })
}
