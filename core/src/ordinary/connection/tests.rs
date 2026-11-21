// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The connection prover against the truth table, the embedding and
//! sequents decided by hand.

use super::decide;
use crate::Limits;
use crate::linear::search::generate::Rng;
use crate::linear::search::{self, Reason};
use crate::ordinary::{Evidence, Logic, Method, Options, Sequent, Verdict};

/// Decides `sequent` by the connection prover under the default bounds.
fn connection(sequent: &Sequent) -> super::Outcome {
    decide(
        sequent,
        Logic::Classical,
        &search::Options::default(),
        &Limits::default(),
        |_| false,
    )
    .unwrap_or_else(|e| panic!("{sequent}: {e}"))
}

/// On generated propositional sequents the prover's verdicts are the
/// truth table's, and it never answers "unknown": a propositional matrix
/// has no variable, so its search is exhaustive.
#[test]
fn propositional_verdicts_are_the_truth_table() {
    let mut rng = Rng::new(7_502);
    for _ in 0..600 {
        let sequent = crate::ordinary::tests::generated_sequent(&mut rng);
        let valid = crate::ordinary::tests::valid_by_truth_table(&sequent);
        let outcome = connection(&sequent);
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "every other verdict and evidence fails the test"
        )]
        match outcome.verdict {
            Verdict::Valid(Evidence::Derivation(_)) => assert!(valid, "{sequent}"),
            Verdict::NotValid => assert!(!valid, "{sequent}"),
            other => panic!("{sequent}: {other:?}"),
        }
    }
}

/// Sequents decided by hand, first-order and propositional: each valid one
/// gets a derivation the checker accepted, each with no variable in its
/// matrix and not valid is refuted, and the others are unknown.
#[test]
fn sequents_decided_by_hand() {
    let valid = [
        "|- ((a -> b) -> a) -> a",
        "a <-> b, b <-> c |- a <-> c",
        "~~a |- a",
        "|- true",
        "false |- a",
        "forall x. p(x) |- p(a)",
        "forall x. p(x) |- exists y. p(y)",
        "|- exists x. (p(x) -> forall y. p(y))",
        "forall x. (p(x) -> q(x)), p(a) |- q(a)",
        "exists x. forall y. r(x, y) |- forall y. exists x. r(x, y)",
        "forall x. p(x) /\\ forall x. q(x) |- forall x. (p(x) /\\ q(x))",
        "forall x. (p(x) /\\ q(x)) |- p(a) /\\ p(b)",
        "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(f(a))))",
        "|- (forall x. p(x)) <-> ~exists x. ~p(x)",
        "exists x. p(x) \\/ exists x. q(x) |- exists x. (p(x) \\/ q(x))",
        "forall x. exists y. r(x, y), forall x. forall y. (r(x, y) -> s(y)) |- exists z. s(z)",
        "|- exists x. forall y. (p(x) -> p(y))",
    ];
    for text in valid {
        let sequent: Sequent = text.parse().unwrap();
        let outcome = connection(&sequent);
        let Verdict::Valid(Evidence::Derivation(derivation)) = &outcome.verdict else {
            panic!("{text}: {:?}", outcome.verdict);
        };
        derivation.check(&Limits::default(), |_| false).unwrap();
        assert_eq!(derivation.logic(), Logic::Classical);
    }
    // No variable in the matrix: refuted.
    let not_valid = [
        "a |- b",
        "p(a) |- p(b)",
        "exists x. p(x) |- forall y. p(y)",
        "|- forall x. p(x)",
        "|- a <-> ~a",
        "b \\/ c, ((false /\\ false) /\\ true) \\/ ((c -> c) -> ~c) |- (b /\\ (c /\\ a)) /\\ (a /\\ (c -> c))",
    ];
    for text in not_valid {
        let sequent: Sequent = text.parse().unwrap();
        let outcome = connection(&sequent);
        assert!(
            matches!(outcome.verdict, Verdict::NotValid),
            "{text}: {:?}",
            outcome.verdict
        );
    }
    // Variables: never refuted.
    let open = [
        "forall x. (p(x) -> p(f(x))), p(a) |- p(b)",
        "forall x. p(x) |- q(a)",
        "|- exists x. p(x)",
        "forall y. exists x. r(x, y) |- false",
    ];
    for text in open {
        let sequent: Sequent = text.parse().unwrap();
        let outcome = connection(&sequent);
        assert!(
            matches!(
                outcome.verdict,
                Verdict::Unknown(Reason::PathLength { .. } | Reason::Incomplete)
            ),
            "{text}: {:?}",
            outcome.verdict
        );
    }
}

/// Restricted backtracking never answers "not valid", and the prover
/// decides classical logic only.
#[test]
fn restricted_backtracking_never_refutes_and_other_logics_are_refused() {
    let sequent: Sequent = "a |- b".parse().unwrap();
    let options = search::Options::default().with_restricted_backtracking(true);
    let outcome = decide(
        &sequent,
        Logic::Classical,
        &options,
        &Limits::default(),
        |_| false,
    )
    .unwrap();
    assert!(matches!(
        outcome.verdict,
        Verdict::Unknown(Reason::Incomplete)
    ));
    for logic in [Logic::Intuitionistic, Logic::Minimal] {
        let refused = decide(&sequent, logic, &options, &Limits::default(), |_| false);
        assert!(matches!(
            refused,
            Err(crate::Error::Logic {
                method: Method::Connection,
                ..
            })
        ));
    }
}

/// Returns a formula over `p/1`, `q/2`, `r/0` and the terms `a`, `b`,
/// `f(t)` and the bound variables in scope, written in the text syntax.
fn formula(rng: &mut Rng, depth: usize, scope: &mut Vec<String>) -> String {
    let term = |rng: &mut Rng, scope: &[String]| {
        let leaf = match rng.below(scope.len() + 2) {
            0 => "a".to_owned(),
            1 => "b".to_owned(),
            k => scope[k - 2].clone(),
        };
        if rng.one_in(5) {
            format!("f({leaf})")
        } else {
            leaf
        }
    };
    if depth == 0 || rng.one_in(4) {
        return match rng.below(5) {
            0 => "r".to_owned(),
            1 | 2 => format!("p({})", term(rng, scope)),
            _ => format!("q({}, {})", term(rng, scope), term(rng, scope)),
        };
    }
    match rng.below(7) {
        0 => format!("~{}", formula(rng, depth - 1, scope)),
        1 | 2 if scope.len() < 2 => {
            let name = ["x", "y"][scope.len()].to_owned();
            let quantifier = if rng.one_in(2) { "forall" } else { "exists" };
            scope.push(name.clone());
            let body = formula(rng, depth - 1, scope);
            scope.pop();
            format!("({quantifier} {name}. {body})")
        }
        k => {
            let a = formula(rng, depth - 1, scope);
            let b = formula(rng, depth - 1, scope);
            let op = ["/\\", "\\/", "->", "<->"][k % 4];
            format!("({a} {op} {b})")
        }
    }
}

/// On generated first-order sequents the prover and the embedding never
/// contradict each other: no sequent the embedding proves is refuted, and
/// every derivation either gives is checked. The prover proves at least
/// what the embedding proves here, with its unbounded path.
#[test]
fn first_order_sequents_agree_with_the_embedding() {
    let mut rng = Rng::new(7_503);
    let limits = Limits {
        work: Some(1 << 16),
        ..Limits::default()
    };
    let (mut proved, mut embedded) = (0, 0);
    for _ in 0..300 {
        let hypotheses: Vec<String> = (0..rng.below(3))
            .map(|_| formula(&mut rng, 3, &mut Vec::new()))
            .collect();
        let goal = formula(&mut rng, 3, &mut Vec::new());
        let text = format!("{} |- {goal}", hypotheses.join(", "));
        let sequent: Sequent = text.parse().unwrap();
        let options = search::Options::default();
        let ours = decide(&sequent, Logic::Classical, &options, &limits, |_| false);
        let Ok(ours) = ours else {
            continue;
        };
        let theirs = crate::ordinary::decide_within(
            &sequent,
            &Options::default()
                .with_logic(Logic::Classical)
                .with_method(Some(Method::Embedding)),
            &options,
            &limits,
            |_| false,
        );
        let embedding_valid = matches!(&theirs, Ok(o) if matches!(o.verdict, Verdict::Valid(_)));
        if embedding_valid {
            embedded += 1;
            assert!(!matches!(ours.verdict, Verdict::NotValid), "{text}");
        }
        if matches!(ours.verdict, Verdict::Valid(_)) {
            proved += 1;
        }
    }
    assert!(proved >= embedded, "{proved} against {embedded}");
    assert!(proved > 0);
}
