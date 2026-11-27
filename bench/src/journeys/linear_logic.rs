// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The journeys of the linear suite that the search journeys leave out: a
//! first-order search, the inverse method, the check of a saturated
//! refutation, cut elimination and a session (`candidates`, `apply`,
//! `close`).

use super::{decided, deterministic, instance, measured, options};
use anyhow::{Result, bail, ensure};
use deduxit::Limits;
use deduxit::linear::proofs::cut;
use deduxit::linear::proofs::interactive::Status;
use deduxit::linear::{
    CutFormula, Engine, InputOptions, Interactive, Mode, Options, Refutation, Sequent, Step,
    Verdict, ViewOptions,
};

/// The first-order family `chain-fo` at 32, `p(z), !∀x. (p(x) ⊸ p(s(x)))
/// ⊢ p(s³²(z))`, within its copies and witness depth: the focused engine's
/// unification, frames and witnesses.
pub fn search_first_order() -> Result<()> {
    let problem = instance("chain-fo", 32, 0)?;
    let options = options(&problem.sequent, problem.copies).with_witness_depth(
        problem
            .witness_depth
            .unwrap_or(Options::DEFAULT_WITNESS_DEPTH),
    );
    let outcome = measured(|| deduxit::linear::prove(&problem.sequent, problem.mode, &options))?;
    decided(&outcome.verdict, problem.provable)
}

/// The inverse method forced on `contraction/16#0` (ILTP's SYJ203 in
/// ILL), which it proves in about n²/2 derived sequents.
pub fn search_inverse() -> Result<()> {
    let problem = instance("contraction", 16, 0)?;
    let options = options(&problem.sequent, problem.copies).with_engine(Some(Engine::Inverse));
    let outcome = measured(|| deduxit::linear::prove(&problem.sequent, problem.mode, &options))?;
    decided(&outcome.verdict, problem.provable)
}

/// The check of the saturated set by which the inverse method refutes
/// `contraction/8#1` (ILTP's SYJ209 in ILL), found outside.
pub fn check_saturated() -> Result<()> {
    let problem = instance("contraction", 8, 1)?;
    let options = options(&problem.sequent, problem.copies).with_engine(Some(Engine::Inverse));
    let disproof = match deduxit::linear::prove(&problem.sequent, problem.mode, &options)?.verdict {
        Verdict::Unprovable(disproof) => disproof,
        other => bail!("expected a refutation, got {other:?}"),
    };
    ensure!(
        matches!(disproof.refutation(), Refutation::Saturated(_)),
        "expected a saturated set, got {:?}",
        disproof.refutation()
    );
    measured(|| disproof.check())?;
    Ok(())
}

/// The proof of `!a ⊢ a ⊗ … ⊗ a` (`N` factors) in ILL through a cut on
/// `!a`, made by a session outside, with every cut eliminated: a
/// contraction per copy, then the derelictions.
pub fn cut_elimination() -> Result<()> {
    const N: usize = 24;
    let limits = Limits::default();
    let mode = Mode::INTUITIONISTIC;
    let sequent: Sequent = format!("!a |- {}", vec!["a"; N].join(" * ")).parse()?;
    let mut session = Interactive::new(&sequent, mode)?;
    let goal = session.goals().next().expect("a new session has its goal");
    let formula = CutFormula::parse_within("!a", mode, &InputOptions::default(), &limits)?;
    session.cut(goal, &formula, &[0], &limits)?;
    for (goal, closed) in
        session.close_all(&deterministic(), &ViewOptions::default(), &limits, |_| {
            false
        })
    {
        let closed = closed?;
        ensure!(
            matches!(closed.outcome.verdict, Verdict::Proved(_)) && closed.grafted.is_ok(),
            "goal {goal:?} of the cut was not closed"
        );
    }
    let proof = session.proof(&limits, |_| false)?;
    let options = cut::Options::default();
    let elimination =
        measured(|| cut::eliminate(&proof, mode, None, &options, &limits, |_| false, |_, _| {}))?;
    ensure!(
        matches!(elimination.end, cut::End::CutFree),
        "the elimination ended at {:?}",
        elimination.end
    );
    Ok(())
}

/// A session in classical MLL with additives on `⊢ ((a₁ ⊗ b₁) ⅋ (a₁⊥ ⅋
/// b₁⊥)) & …` (`N` conjuncts): every goal's candidate rules at every
/// formula, each invertible step that applies taken (the `&` and the
/// `⅋`s), and the goals left, each `a ⊗ b, a⊥, b⊥`, closed by the search,
/// each closing graft checked.
pub fn session() -> Result<()> {
    const N: usize = 16;
    let conjuncts: Vec<String> = (1..=N)
        .map(|i| format!("((a{i} * b{i}) | (~a{i} | ~b{i}))"))
        .collect();
    let sequent: Sequent = format!("|- {}", conjuncts.join(" & ")).parse()?;
    let mut session = Interactive::new(&sequent, Mode::CLASSICAL)?;
    let options = deterministic();
    let limits = Limits::default();
    measured(|| -> Result<()> {
        loop {
            let Some(goal) = session.goals().next() else {
                break;
            };
            let members = session.goal(goal)?.to_vec();
            let mut taken = None;
            for position in 0..members.len() {
                for candidate in session.candidates(goal, position)? {
                    let step = Step::new(position, candidate.rule);
                    if taken.is_none()
                        && candidate.status == Status::Applies
                        && session.is_invertible(&members, &step)
                    {
                        taken = Some(step);
                    }
                }
            }
            match taken {
                Some(step) => {
                    session.apply(goal, &step)?;
                }
                None => {
                    let closed =
                        session
                            .close(goal, &options, &ViewOptions::default(), &limits, |_| false)?;
                    ensure!(
                        matches!(closed.outcome.verdict, Verdict::Proved(_))
                            && closed.grafted.is_ok(),
                        "goal {goal:?} was not closed: {:?}",
                        closed.outcome.verdict
                    );
                }
            }
        }
        Ok(())
    })
}
