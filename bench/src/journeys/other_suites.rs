// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The journeys of modal logic, lambda-terms, the calculi given as rule
//! files and the workspace.

use super::ordinary_logic::pigeonhole;
use super::{deterministic, measured};
use anyhow::{Result, bail, ensure};
use deduxit::lambda::{self, Calculus, InhabitOptions, ReduceOptions, Term};
use deduxit::rules::{self, Apply, Status};
use deduxit::workspace::{State, What, Workspace};
use deduxit::{Limits, Settings, modal, ordinary, wire};

/// Halpern and Moses's formula of depth `m`, which forces a binary tree
/// of `2^m` worlds: a sequent of K with it as hypothesis and nothing
/// right, not valid.
fn halpern_moses(m: usize) -> String {
    let d = |i: usize| format!("d{i}");
    let p = |i: usize| format!("p{i}");
    let depth: Vec<String> = (1..=m + 1)
        .map(|i| format!("({} -> {})", d(i), d(i - 1)))
        .collect();
    let determined: Vec<String> = (1..=m)
        .map(|i| {
            format!(
                "({d} -> (({p} -> [](({d}) -> {p})) /\\ (~{p} -> []({d} -> ~{p}))))",
                d = d(i),
                p = p(i)
            )
        })
        .collect();
    let branching: Vec<String> = (0..m)
        .map(|i| {
            format!(
                "(({} /\\ ~{}) -> (<>({} /\\ ~{} /\\ {}) /\\ <>({} /\\ ~{} /\\ ~{})))",
                d(i),
                d(i + 1),
                d(i + 1),
                d(i + 2),
                p(i + 1),
                d(i + 1),
                d(i + 2),
                p(i + 1)
            )
        })
        .collect();
    let body = format!(
        "({}) /\\ ({}) /\\ ({})",
        depth.join(" /\\ "),
        determined.join(" /\\ "),
        branching.join(" /\\ ")
    );
    let boxes: Vec<String> = (0..=m)
        .map(|i| format!("{}({body})", "[]".repeat(i)))
        .collect();
    format!("{} /\\ ~{} /\\ {} |-", d(0), d(1), boxes.join(" /\\ "))
}

/// The modal tableau in K on Halpern and Moses's formula: the search, its
/// countermodel of eight worlds and the model's check.
pub fn modal_tableau() -> Result<()> {
    let sequent: modal::Sequent = halpern_moses(3).parse()?;
    let options = modal::Options::default();
    let outcome =
        measured(|| modal::decide_within(&sequent, &options, &Limits::default(), |_| false))?;
    match outcome.verdict {
        modal::Verdict::NotValid => Ok(()),
        other => bail!("expected not valid, got {other:?}"),
    }
}

/// A simply typed term of `n` nested redexes, `λx:a. (λy:a. y) ((λy:a. y)
/// (… x))`.
fn redexes(n: usize) -> Result<Term> {
    let text = format!(r"\x:a. {}x", r"(\y:a. y) (".repeat(n)) + &")".repeat(n);
    Ok(Term::parse(&text, Calculus::Simple, &Limits::default())?)
}

/// The type checker on a term of many redexes: a judgement per node.
pub fn lambda_check() -> Result<()> {
    let term = redexes(4_000)?;
    measured(|| term.check(&Limits::default(), |_| false))?;
    Ok(())
}

/// β-reduction in normal order to the normal form of a term of many
/// redexes, each term reached checked.
pub fn lambda_reduce() -> Result<()> {
    let term = redexes(100)?;
    let options = ReduceOptions::default();
    let reduction = measured(|| term.reduce(&options, &Limits::default(), |_| false, |_, _| {}))?;
    ensure!(
        matches!(reduction.end, lambda::End::NormalForm),
        "the reduction ended at {:?}",
        reduction.end
    );
    Ok(())
}

/// The terms of `a → … → a` (`N` arguments), each a projection: found,
/// listed, counted and checked.
pub fn lambda_inhabit() -> Result<()> {
    const N: usize = 8;
    let sequent: ordinary::Sequent = format!("|- {}a", "a -> ".repeat(N)).parse()?;
    let options = InhabitOptions::default();
    let found = measured(|| lambda::enumerate(&sequent, &options, &Limits::default(), |_| false))?;
    ensure!(
        found.terms.len() == N,
        "expected {N} terms, got {}",
        found.terms.len()
    );
    Ok(())
}

/// Reads a built-in rule file.
fn calculus(text: &str) -> Result<rules::Calculus> {
    let mut document = serde_json::Deserializer::from_str(text);
    Ok(wire::upgrade(&mut document, &Limits::default())?)
}

/// A session in natural deduction (NJ) on `a₁ ∧ (… ∧ a_N) → (a_N ∧ …) ∧
/// a₁`: every goal's candidate rules, `→I` and `∧I` taken where one
/// applies, and the goals left closed by the search.
pub fn rules_session() -> Result<()> {
    const N: usize = 3;
    let atoms: Vec<String> = (1..=N).map(|i| format!("a{i}")).collect();
    let mut reversed = atoms.clone();
    reversed.reverse();
    let goal = format!("{} -> {}", atoms.join(" /\\ "), reversed.join(" /\\ "));
    let limits = Limits::default();
    let mut session = rules::Interactive::within(calculus(rules::NJ)?, &goal, &[], &limits)?;
    let search = deterministic();
    measured(|| -> Result<()> {
        while let Some(&goal) = session.goals().first() {
            let taken = session
                .candidates(goal)?
                .into_iter()
                .find(|c| c.status == Status::Applies && (c.rule == "→I" || c.rule == "∧I"));
            match taken {
                Some(candidate) => {
                    session.apply(goal, &Apply::new(&candidate.rule).at(&candidate.at))?;
                }
                None => {
                    let closed = session.close(goal, &search, &limits, |_| false)?;
                    ensure!(
                        closed.grafted.is_ok() && !session.goals().contains(&goal),
                        "goal {goal:?} was not closed"
                    );
                }
            }
        }
        session.finished(|_| false)?;
        Ok(())
    })
}

/// The checker of a rule file: the derivation of LK of a pigeonhole
/// formula, read back from the embedding outside, checked against the
/// rule file of LK.
pub fn rules_check() -> Result<()> {
    let sequent: ordinary::Sequent = pigeonhole(2).parse()?;
    let options = ordinary::Options::default()
        .with_logic(ordinary::Logic::Classical)
        .with_method(Some(ordinary::Method::Embedding));
    let outcome = ordinary::decide_within(
        &sequent,
        &options,
        &deterministic(),
        &Limits::default(),
        |_| false,
    )?;
    let ordinary::Verdict::Valid(evidence) = outcome.verdict else {
        bail!("the pigeonhole formula is valid: {:?}", outcome.verdict);
    };
    let derivation = rules::Derivation::from_ordinary(evidence.derivation()?)?;
    let lk = calculus(rules::LK)?;
    measured(|| derivation.check(&lk, &Limits::default(), |_| false))?;
    Ok(())
}

/// A workspace that holds a chain of `N` implications in intuitionistic
/// logic, `p0, p0 → p1, … ⊢ pN`, and decides it: the run of the operation
/// and the check of its promise. Intuitionistic, so that the SAT method
/// decides alone: the classical default searches the embedding for its
/// evidence too, which would be most of the count.
pub fn workspace() -> Result<()> {
    const N: usize = 16;
    let steps: Vec<String> = (1..=N).map(|i| format!("p{} -> p{i}", i - 1)).collect();
    let sequent: ordinary::Sequent = format!("p0, {} |- p{N}", steps.join(", ")).parse()?;
    let mut form = serde_json::to_value(&sequent)?;
    form["logic"] = "intuitionistic".into();
    let form = form.to_string();
    let limits = Limits::default();
    let settings = Settings::default();
    let mut workspace = Workspace::new();
    let decided = workspace.edit("decide", &limits, |edit| {
        let given = edit.add("ordinary.sequent", &form)?;
        edit.connect("ordinary.decide", &[given], "", &settings)
    })?;
    measured(|| workspace.run(&[decided], What::Check, &limits, |_| false))?;
    ensure!(
        workspace.state(decided) == State::Current,
        "the decision is {:?}",
        workspace.state(decided)
    );
    Ok(())
}
