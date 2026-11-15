// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The wire forms of earlier releases and levels, read by this one.
//! `wire/level-1.txt` is the JSON lock (`lock/json.txt`) as version 0.1.0
//! wrote it, followed by the disproofs of every refutation and a proof and
//! a disproof of a goal off the roots, as 0.1.0 wrote them, one `NAME:
//! JSON` per line, `wire/level-2.txt` the forms level 2 added and
//! `wire/level-3.txt` those level 3 added, `wire/level-4.txt` those
//! level 4 added, `wire/level-5.txt` those level 5 added,
//! `wire/level-6.txt` those level 6 added, `wire/level-7.txt` those
//! level 7 added, `wire/level-8.txt` those level 8 added,
//! `wire/level-11.txt` those level 11 added, `wire/level-12.txt` the
//! ordinary outcomes of level 12, `wire/level-13.txt` a Kripke model and
//! an outcome that holds one (at level 12); none is ever
//! rewritten: where
//! the lock moves on, these files stay. Every
//! line of a form that is read back reads through
//! `wire::upgrade`, and writes the same bytes again; an outcome reads back
//! as the proof or the disproof it holds. A release that fails here can no
//! longer read what an earlier one wrote at a level it claims, which its
//! wire level promises.
#![cfg(all(feature = "serialize", feature = "interactive"))]

use deduxit::wire::{self, Readable};
use deduxit::{
    Limits,
    linear::{
        Disproof, Engine, IllSequent, Interactive, MellStructure, Outcome, Proof, ProofStructure,
        Sequent,
    },
};

/// Reads one document of the form `T`.
fn read<T: Readable>(name: &str, json: &str) -> T {
    let mut document = serde_json::Deserializer::from_str(json);
    let value = wire::upgrade(&mut document, &Limits::default())
        .unwrap_or_else(|error| panic!("{name}: {error}"));
    document.end().unwrap();
    value
}

/// Reads one document of the form `T` and asserts that it writes the same
/// bytes again.
fn again<T: Readable + serde::Serialize>(name: &str, json: &str) {
    let value: T = read(name, json);
    assert_eq!(serde_json::to_string(&value).unwrap(), json, "{name}");
}

/// Every line of the first release's forms reads, and a form read back
/// writes what it read.
#[test]
fn the_first_release_reads() {
    let lines = include_str!("wire/level-1.txt");
    let mut read_back = 0;
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        let verdict = |v: &str| json.contains(&format!(r#""verdict":"{v}""#));
        match name.split('-').next().unwrap() {
            "sequent" => again::<Sequent>(name, json),
            "proof" => again::<Proof>(name, json),
            "disproof" => again::<Disproof>(name, json),
            "structure" => again::<ProofStructure>(name, json),
            "session" => again::<Interactive>(name, json),
            "outcome" if verdict("proved") => {
                let _: Proof = read(name, json);
            }
            "outcome" if verdict("unprovable") => {
                let _: Disproof = read(name, json);
            }
            // An unknown outcome holds no proof or disproof;
            // `outcomes_read_back` reads it as an outcome.
            "outcome" => continue,
            other => panic!("{name}: no form {other}"),
        }
        read_back += 1;
    }
    assert_eq!(read_back, 37, "every line but the three unknown outcomes");
}

/// The forms level 2 added, as the release that made it wrote them: the
/// classical refutation of a disproof and of an unprovable outcome. Never
/// rewritten, as `wire/level-1.txt` is not.
#[test]
fn the_second_level_reads() {
    let lines = include_str!("wire/level-2.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "disproof" => again::<Disproof>(name, json),
            "outcome" => {
                let _: Disproof = read(name, json);
            }
            other => panic!("{name}: no form {other}"),
        }
    }
    assert_eq!(lines.lines().count(), 2);
}

/// The forms level 3 added, as the release that made it wrote them: the
/// refutation by a saturated set of a disproof and of an unprovable
/// outcome. Never rewritten, as `wire/level-1.txt` is not.
#[test]
fn the_third_level_reads() {
    let lines = include_str!("wire/level-3.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "disproof" => again::<Disproof>(name, json),
            "outcome" => {
                let disproof: Disproof = read(name, json);
                assert_eq!(disproof.check(), Ok(()), "{name}");
            }
            other => panic!("{name}: no form {other}"),
        }
    }
    assert_eq!(lines.lines().count(), 2);
}

/// The forms level 4 added, as the release that made it wrote them: a
/// proof structure with boxes. Never rewritten, as `wire/level-1.txt` is
/// not.
#[test]
fn the_fourth_level_reads() {
    let lines = include_str!("wire/level-4.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "structure" => {
                again::<MellStructure>(name, json);
                let net: MellStructure = read(name, json);
                assert_eq!(net.is_correct(|_| false), Ok(()), "{name}");
            }
            other => panic!("{name}: no form {other}"),
        }
    }
    assert_eq!(lines.lines().count(), 1);
}

/// The forms level 5 added, as the release that made it wrote them: an
/// intuitionistic sequent. Never rewritten, as `wire/level-1.txt` is not.
#[test]
fn the_fifth_level_reads() {
    let lines = include_str!("wire/level-5.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "sequent" => {
                again::<IllSequent>(name, json);
                let sequent: IllSequent = read(name, json);
                assert_eq!(sequent.to_string(), "!(A ⊸ B), ⊤, 0 ⊢ (A ⊗ 1) & (B ⊕ ⊤)");
            }
            other => panic!("{name}: no form {other}"),
        }
    }
    assert_eq!(lines.lines().count(), 1);
}

/// The forms level 6 added, as the release that made it wrote them: a
/// proof with a cut and a session with one, whose sequents hold their cut
/// formulas. Never rewritten, as `wire/level-1.txt` is not.
#[test]
fn the_sixth_level_reads() {
    let lines = include_str!("wire/level-6.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "proof" => {
                again::<Proof>(name, json);
                let proof: Proof = read(name, json);
                assert_eq!(
                    proof.check(deduxit::linear::Mode::CLASSICAL),
                    Ok(()),
                    "{name}"
                );
                assert_eq!(proof.sequent().cuts().len(), 1, "{name}");
            }
            "session" => {
                again::<Interactive>(name, json);
                let session: Interactive = read(name, json);
                assert_eq!(session.goals().count(), 1, "{name}");
            }
            other => panic!("{name}: no form {other}"),
        }
    }
    assert_eq!(lines.lines().count(), 2);
}

/// Every document of level 7, the first-order sequent in its one-sided and
/// its intuitionistic form, reads and writes the same bytes again.
#[test]
fn the_seventh_level_reads() {
    let lines = include_str!("wire/level-7.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "sequent" => {
                again::<Sequent>(name, json);
                let sequent: Sequent = read(name, json);
                assert!(sequent.is_first_order(), "{name}");
            }
            "intuitionistic" => {
                again::<IllSequent>(name, json);
                let sequent: IllSequent = read(name, json);
                assert!(sequent.sequent().fragment().has_quantifiers(), "{name}");
            }
            other => panic!("{other}: no form of level 7"),
        }
    }
}

/// Every document of level 8, the forms of ordinary logic (a sequent, a
/// derivation of LK and a proof in progress in LJ), reads and writes the
/// same bytes again; the derivation still checks and the proof in
/// progress still has its open goal. Never rewritten, as
/// `wire/level-1.txt` is not.
#[test]
fn the_eighth_level_reads() {
    use deduxit::ordinary;
    let lines = include_str!("wire/level-8.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "sequent" => again::<ordinary::Sequent>(name, json),
            "derivation" => {
                again::<ordinary::Derivation>(name, json);
                let derivation: ordinary::Derivation = read(name, json);
                assert_eq!(derivation.check(&Limits::default(), |_| false), Ok(()));
            }
            "session" => {
                again::<ordinary::Interactive>(name, json);
                let session: ordinary::Interactive = read(name, json);
                assert_eq!(session.goals().count(), 1, "{name}");
            }
            other => panic!("{other}: no form of level 8"),
        }
    }
    assert_eq!(lines.lines().count(), 3);
}

/// The forms level 11 added, as the release that made it wrote them: the
/// refutation by the basis of a backward search, of a disproof and of an
/// unprovable outcome, which still checks. Never rewritten, as
/// `wire/level-1.txt` is not.
#[test]
fn the_eleventh_level_reads() {
    let lines = include_str!("wire/level-11.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "disproof" => {
                again::<Disproof>(name, json);
                let disproof: Disproof = read(name, json);
                assert_eq!(disproof.check(), Ok(()), "{name}");
            }
            "outcome" => {
                let disproof: Disproof = read(name, json);
                assert_eq!(disproof.check(), Ok(()), "{name}");
            }
            other => panic!("{name}: no form {other}"),
        }
    }
    assert_eq!(lines.lines().count(), 2);
}

/// Every ordinary outcome of level 12 (LJT's, the embedding's with the
/// image's proof nested, the SAT solver's refutation) reads and writes
/// the same bytes again, and a derivation it holds still checks. Never
/// rewritten, as `wire/level-1.txt` is not.
#[test]
fn the_twelfth_level_reads() {
    use deduxit::ordinary;
    let lines = include_str!("wire/level-12.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        again::<ordinary::Outcome>(name, json);
        let outcome: ordinary::Outcome = read(name, json);
        if let ordinary::Verdict::Valid(evidence) = &outcome.verdict {
            let derivation = evidence.derivation().unwrap();
            assert_eq!(derivation.check(&Limits::default(), |_| false), Ok(()));
        }
    }
    assert_eq!(lines.lines().count(), 3);
}

/// The forms level 13 added, a Kripke model, and an ordinary outcome that
/// holds one (still written at level 12: the key raises no level), read
/// and write the same bytes again, and the outcome's
/// countermodel still refutes its sequent. Never rewritten, as
/// `wire/level-1.txt` is not.
#[test]
fn the_thirteenth_level_reads() {
    use deduxit::{kripke, ordinary};
    let lines = include_str!("wire/level-13.txt");
    for line in lines.lines() {
        let (name, json) = line.split_once(": ").unwrap();
        match name.split('-').next().unwrap() {
            "model" => again::<kripke::Model>(name, json),
            "outcome" => {
                again::<ordinary::Outcome>(name, json);
                let outcome: ordinary::Outcome = read(name, json);
                let countermodel = outcome.countermodel.expect("a countermodel");
                assert_eq!(countermodel.check(outcome.logic, |_| false), Ok(()));
            }
            other => panic!("{name}: no form {other}"),
        }
    }
    assert_eq!(lines.lines().count(), 2);
}

/// Every outcome the JSON lock holds (each verdict, refutation and reason,
/// at levels 1 and 2) reads as an `Outcome` and writes the same bytes
/// again; one the first release wrote reads too, as does an engine this
/// build does not know.
#[test]
fn outcomes_read_back() {
    let mut levels = std::collections::BTreeSet::new();
    for line in include_str!("lock/json.txt").lines() {
        let (name, json) = line.split_once(": ").unwrap();
        if name.starts_with("outcome-") {
            again::<Outcome>(name, json);
            let level: serde_json::Value = serde_json::from_str(json).unwrap();
            levels.insert(level["version"].as_u64().unwrap());
        }
    }
    assert_eq!(levels.into_iter().collect::<Vec<_>>(), [1, 2]);
    for line in include_str!("wire/level-1.txt").lines() {
        let (name, json) = line.split_once(": ").unwrap();
        if name.starts_with("outcome-") {
            let _: Outcome = read(name, json);
        }
    }

    let (name, json) = include_str!("lock/json.txt")
        .lines()
        .find_map(|line| {
            line.split_once(": ")
                .filter(|(name, _)| *name == "outcome-mll")
        })
        .unwrap();
    let later = json.replace(r#""engine":"focus""#, r#""engine":"sat""#);
    assert_ne!(later, json);
    let outcome: Outcome = read(name, &later);
    assert_eq!(outcome.engine, Engine::Unknown);
}
