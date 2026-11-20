// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The resolution suite's tests: by hand, the checker's refusals, the two
//! searches against the SAT solver, the bounds, DIMACS and outside proofs.

use super::*;
use crate::ordinary::normal::{self, Form};

/// The four clauses over `a`, `b` that no assignment satisfies.
fn four() -> Clauses {
    Clauses::from_dimacs(
        "p cnf 2 4\n1 2 0\n-1 2 0\n1 -2 0\n-1 -2 0\n",
        &Limits::default(),
    )
    .unwrap()
}

/// A refutation built by hand passes the check, a step without its pivot
/// is refused, and the refutation reads back from its JSON as itself.
#[test]
fn a_refutation_by_hand_is_checked() {
    let mut r = Refutation::new(four());
    assert!(matches!(r.resolve(0, 1, 1), Err(Error::StepRefused { .. })));
    assert!(matches!(r.resolve(0, 9, 0), Err(Error::StepRefused { .. })));
    let b = r.resolve(1, 0, 0).unwrap();
    let not_b = r.resolve(2, 3, 0).unwrap();
    let empty = r.resolve(b, not_b, 1).unwrap();
    assert_eq!(r.clauses().text(r.clause(b).unwrap()).to_string(), "x2");
    assert!(r.clause(empty).unwrap().is_empty() && r.is_finished());
    r.check(&Limits::default(), |_| false).unwrap();
    // The first clause of a step holds the atom: `resolve` puts it first.
    assert_eq!((r.resolvents()[0].first, r.resolvents()[0].second), (0, 1));
    let json = serde_json::to_string(&r).unwrap();
    let back: Refutation = crate::wire::upgrade(
        &mut serde_json::Deserializer::from_str(&json),
        &Limits::default(),
    )
    .unwrap();
    assert_eq!(back, r);
}

/// A clause that holds an atom and its negation keeps the other literal
/// when resolved on it: `{a ∨ ¬a}` is satisfiable and has no refutation,
/// by hand or read from JSON.
#[test]
fn a_tautology_is_no_way_to_the_empty_clause() {
    let limits = Limits::default();
    let json = r#"{"version":22,"atoms":["a"],"clauses":[[1,-1]],"resolvents":[{"first":0,"second":0,"atom":1,"clause":[]}]}"#;
    let read: Refutation =
        crate::wire::upgrade(&mut serde_json::Deserializer::from_str(json), &limits).unwrap();
    assert!(matches!(
        read.check(&limits, |_| false),
        Err(Error::InvalidRefutation { .. })
    ));
    // Satisfiable with x1 true and x2 false.
    let set = Clauses::from_dimacs(
        "p cnf 2 3
1 2 0
-1 -2 0
-2 0
",
        &limits,
    )
    .unwrap();
    let mut r = Refutation::new(set);
    let both = r.resolve(0, 1, 0).unwrap();
    assert_eq!(
        r.clauses().text(r.clause(both).unwrap()).to_string(),
        "x2 ∨ ¬x2"
    );
    let next = r.resolve(both, 2, 1).unwrap();
    assert_eq!(r.clauses().text(r.clause(next).unwrap()).to_string(), "¬x2");
    assert!(!r.is_finished());
    // An atom past the set's is refused, never wrapped onto another.
    assert!(matches!(
        r.resolve(0, 1, 1 << 31),
        Err(Error::StepRefused { .. })
    ));
    assert_eq!(Literal::new(Literal::MOST_ATOMS, true), None);
}

/// The checker refuses a resolvent that is not one, a clause numbered
/// after the step, a pivot in neither clause, and a refutation without
/// the empty clause.
#[test]
fn the_checker_refuses_what_is_no_refutation() {
    let mut good = Refutation::new(four());
    good.resolve(0, 1, 0).unwrap();
    good.resolve(2, 3, 0).unwrap();
    good.resolve(4, 5, 1).unwrap();
    let limits = Limits::default();
    let wrong = |change: &dyn Fn(&mut Vec<Resolvent>)| {
        let mut steps = good.resolvents().to_vec();
        change(&mut steps);
        Refutation::of_parts(good.clauses().clone(), steps).check(&limits, |_| false)
    };
    assert!(wrong(&|_| {}).is_ok());
    let changes: [fn(&mut Vec<Resolvent>); 5] = [
        |s| s[0].clause = Clause::new(Vec::new()),
        |s| s[2].first = 6,
        |s| s[0].atom = 1,
        |s| {
            let r = &mut s[0];
            std::mem::swap(&mut r.first, &mut r.second);
        },
        |s| {
            s.pop();
        },
    ];
    for change in changes {
        assert!(matches!(
            wrong(&change),
            Err(Error::InvalidRefutation { .. })
        ));
    }
}

/// Both searches decide the clauses of generated sequents as the SAT
/// solver decides the sequents, through every form that gives clauses;
/// `decide` checked every refutation and model.
#[test]
fn both_searches_agree_with_the_sat_solver() {
    use crate::linear::search::generate::Rng;
    let mut rng = Rng::new(70);
    let limits = Limits::default();
    let mut decided = 0;
    for _ in 0..80 {
        let sequent = super::super::tests::generated_sequent(&mut rng);
        let valid = matches!(
            super::super::decide(
                &sequent,
                &super::super::Options::default().with_logic(super::super::Logic::Classical)
            )
            .unwrap()
            .verdict,
            super::super::Verdict::Valid(_)
        );
        for form in [Form::Tseitin, Form::PlaistedGreenbaum] {
            let options = normal::Options::default().with_form(form);
            let clauses = Clauses::of_sequent(
                &normal::rewrite(&sequent, &options, &limits, |_| false)
                    .unwrap()
                    .result,
            )
            .unwrap();
            for &method in Method::ALL {
                let options = Options::default().with_method(method);
                let answer = decide(&clauses, &options, &limits, |_| false).unwrap();
                match answer.verdict {
                    Verdict::Refuted(_) => assert!(valid, "{sequent} by {}", method.name()),
                    Verdict::Satisfiable { .. } => {
                        assert!(!valid, "{sequent} by {}", method.name())
                    }
                    Verdict::Unknown(_) => continue,
                }
                decided += 1;
            }
        }
    }
    assert!(decided > 250, "{decided} decided");
}

/// DPLL's trace on the four clauses: a decision, a conflict, the
/// backtrack, another conflict; its refutation resolves on both atoms.
#[test]
fn the_trace_shows_every_step() {
    let options = Options::default().with_method(Method::Dpll);
    let answer = decide(&four(), &options, &Limits::default(), |_| false).unwrap();
    let a = Literal::of(0, true);
    let b = Literal::of(1, true);
    assert_eq!(
        answer.trace,
        [
            Event::Decide {
                literal: a,
                level: 1
            },
            Event::Unit {
                literal: b,
                clause: 1
            },
            Event::Conflict { clause: 3 },
            Event::Backtrack {
                literal: a.negated(),
                level: 1
            },
            Event::Unit {
                literal: b,
                clause: 0
            },
            Event::Conflict { clause: 2 },
        ]
    );
    let Verdict::Refuted(r) = answer.verdict else {
        panic!("unsatisfiable")
    };
    assert_eq!(r.resolvents().len(), 3);
}

/// The bounds end a search as unknown: the pigeons of five into four holes
/// need more than a few steps.
#[test]
fn the_bounds_end_a_search() {
    let pigeons = 5;
    let holes = 4;
    let var = |p: usize, h: usize| (p * holes + h) as u32;
    let mut clauses = Vec::new();
    for p in 0..pigeons {
        clauses.push((0..holes).map(|h| Literal::of(var(p, h), true)).collect());
    }
    for h in 0..holes {
        for p in 0..pigeons {
            for q in p + 1..pigeons {
                clauses.push(vec![
                    Literal::of(var(p, h), false),
                    Literal::of(var(q, h), false),
                ]);
            }
        }
    }
    let atoms = (0..pigeons * holes).map(|k| format!("p{k}")).collect();
    let set = Clauses::new(atoms, clauses).unwrap();
    let limits = Limits::default();
    for method in [Method::Saturation, Method::Dpll] {
        let options = Options::default().with_method(method).with_steps(Some(50));
        let answer = decide(&set, &options, &limits, |_| false).unwrap();
        assert!(matches!(
            answer.verdict,
            Verdict::Unknown(Bound::Steps { limit: 50 })
        ));
        assert!(answer.statistics.steps <= 50);
    }
    // A bound as large as the trace decides; one less does not.
    let dpll = Options::default().with_method(Method::Dpll);
    for (steps, decided) in [(6, true), (5, false)] {
        let answer = decide(&four(), &dpll.with_steps(Some(steps)), &limits, |_| false).unwrap();
        assert_eq!(
            !matches!(answer.verdict, Verdict::Unknown(_)),
            decided,
            "{steps}"
        );
    }
    let options = Options::default().with_clauses(Some(60));
    let answer = decide(&set, &options, &limits, |_| false).unwrap();
    assert!(matches!(
        answer.verdict,
        Verdict::Unknown(Bound::Clauses { limit: 60 })
    ));
}

/// The DIMACS reader takes comments and SATLIB's `%`, and refuses a
/// variable past the header and a clause not ended.
#[test]
fn dimacs_is_read_and_refused() {
    let limits = Limits::default();
    let read = Clauses::from_dimacs("c two\np cnf 2 2\n1 -2\n0 2 0\n%\n0\n", &limits).unwrap();
    assert_eq!(read.clauses().len(), 2);
    assert_eq!(read.text(&read.clauses()[0]).to_string(), "x1 ∨ ¬x2");
    for text in [
        "p cnf 1 1\n2 0\n",
        "p cnf 2 1\n1 2\n",
        "1 0\n",
        "p dnf 1 1\n",
    ] {
        assert!(
            matches!(
                Clauses::from_dimacs(text, &limits),
                Err(Error::ClauseFile { .. })
            ),
            "{text}"
        );
    }
    // A header of many atoms is refused at the memory bound before their
    // names are made.
    let tight = Limits::default().with_memory_bytes(Some(1 << 20));
    assert!(matches!(
        Clauses::from_dimacs("p cnf 50000000 1\n1 0\n", &tight),
        Err(Error::Refused(Refusal::Memory { .. }))
    ));
    let small = Limits::default().with_occurrences(Some(3));
    assert!(matches!(
        Clauses::from_dimacs("p cnf 2 2\n1 2 0\n-1 0\n", &small),
        Err(Error::Refused(Refusal::Occurrences { .. }))
    ));
}

/// An outside proof is checked in each format: DRAT as text and binary,
/// LRAT; a proof whose step does not follow is refused.
#[cfg(feature = "sat")]
#[test]
fn outside_proofs_are_checked() {
    let limits = Limits::default();
    let set = four();
    for (proof, format) in [
        (&b"2 0\n0\n"[..], ProofFormat::Drat),
        (b"c drat\n2 0\nd 1 2 0\n0\n", ProofFormat::Drat),
        (b"a\x04\x00a\x00", ProofFormat::Drat),
        (b"5 2 0 1 2 0\n5 d 1 0\n6 0 5 3 4 0\n", ProofFormat::Lrat),
    ] {
        let checked = check_proof(&set, proof, format, &limits, |_| false);
        assert!(
            checked.is_ok(),
            "{:?}: {checked:?}",
            String::from_utf8_lossy(proof)
        );
    }
    assert!(matches!(
        check_proof(&set, b"0\n", ProofFormat::Drat, &limits, |_| false),
        Err(Error::InvalidRefutation { step: Some(1), .. })
    ));
    assert!(matches!(
        check_proof(&set, b"3 0\n0\n", ProofFormat::Drat, &limits, |_| false),
        Err(Error::InvalidRefutation { .. })
    ));
    assert!(matches!(
        check_proof(&set, b"2 x 0\n", ProofFormat::Drat, &limits, |_| false),
        Err(Error::ClauseFile { at: 1, .. })
    ));
}
