// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Tests of the intuitionistic connection prover.

use super::decide;
use crate::Limits;
use crate::linear::search::{self, Reason};
use crate::ordinary::{Evidence, Logic, Sequent, Verdict};

/// Decides `text` intuitionistically within a bound on the active path.
fn decided(text: &str) -> Verdict {
    let sequent: Sequent = text.parse().unwrap();
    let options = search::Options::default().with_path_length(Some(8));
    decide(&sequent, Logic::Intuitionistic, &options, &Limits::default(), &mut |_| false)
        .unwrap()
        .verdict
}

/// Intuitionistically valid sequents get a derivation of LJ the checker
/// takes; classical laws that intuitionistic logic lacks are unknown.
#[test]
fn intuitionistic_laws_are_proved_and_classical_ones_are_not() {
    let valid = [
        "a |- a",
        "a /\\ b |- b /\\ a",
        "a \\/ b |- b \\/ a",
        "a -> b, b -> c |- a -> c",
        "|- a -> (b -> a)",
        "|- ~~(a \\/ ~a)",
        "~a \\/ b |- a -> b",
        "forall x. p(x) |- p(c)",
        "p(c) |- exists x. p(x)",
        "forall x. (p(x) -> q(x)), p(c) |- q(c)",
        "exists x. p(x), forall x. (p(x) -> q(x)) |- exists x. q(x)",
        "exists x. forall y. r(x, y) |- forall y. exists x. r(x, y)",
        "forall x. p(x) /\\ forall x. q(x) |- forall x. (p(x) /\\ q(x))",
        "~(exists x. p(x)) |- forall x. ~p(x)",
        "(a <-> b), a |- b",
        "false |- a",
    ];
    for text in valid {
        let verdict = decided(text);
        let Verdict::Valid(Evidence::Derivation(d)) = &verdict else {
            panic!("{text}: {verdict:?}");
        };
        d.check(&Limits::default(), |_| false).unwrap();
    }
    let classical_only = [
        "|- a \\/ ~a",
        "|- ((a -> b) -> a) -> a",
        "~~a |- a",
        "|- exists x. (p(x) -> forall y. p(y))",
        "forall x. (p(x) \\/ a) |- (forall x. p(x)) \\/ a",
        "~(forall x. p(x)) |- exists x. ~p(x)",
    ];
    for text in classical_only {
        let verdict = decided(text);
        assert!(
            matches!(verdict, Verdict::Unknown(Reason::PathLength { .. } | Reason::Incomplete)),
            "{text}: {verdict:?}"
        );
    }
}

/// On generated propositional sequents the prover never proves what LJT
/// refutes, every proof whose prefixes unify reads back, and it proves
/// most of what LJT proves within a small bound.
#[test]
fn agrees_with_ljt() {
    use crate::linear::search::generate::Rng;
    let mut rng = Rng::new(75);
    let options = search::Options::default().with_path_length(Some(6));
    let limits = Limits::default().with_work(Some(200_000));
    let (mut valid, mut proved, mut unread) = (0, 0, 0);
    for _ in 0..300 {
        let sequent = crate::ordinary::tests::generated_sequent(&mut rng);
        if sequent.right().len() > 1 {
            continue;
        }
        let ljt = crate::ordinary::ljt::decide(&sequent, Logic::Intuitionistic, &Limits::default(), |_| false)
            .unwrap();
        let expected = matches!(ljt.verdict, Verdict::Valid(_));
        let (outcome, missed) =
            super::decide_counting(&sequent, Logic::Intuitionistic, &options, &limits, &mut |_| false)
                .unwrap();
        unread += missed;
        valid += u32::from(expected);
        match outcome.verdict {
            Verdict::Valid(evidence) => {
                assert!(expected, "{sequent}: LJT refutes it");
                evidence.derivation().unwrap().check(&Limits::default(), |_| false).unwrap();
                proved += 1;
            }
            Verdict::NotValid => panic!("{sequent}: the prover never refutes"),
            Verdict::Unknown(_) => {}
        }
    }
    assert_eq!(unread, 0, "proofs whose prefixes unified but did not read back");
    assert!(proved * 10 >= valid * 8, "{proved} of {valid}");
}
