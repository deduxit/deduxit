// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! LJT against the embedding on the ILTP library's propositional
//! problems: every problem one of the embedding's translations decides at
//! the command's default time limit of 2 s gets the same verdict from
//! LJT, and every proof LJT returns passes the checker.
//!
//! The embedding's verdicts are `tests/iltp/embedding.csv`, recorded by a
//! run of the command; the library (`nix build .#iltp -o bench/iltp`)
//! states no licence and is not in the repository, so the test is ignored
//! by default and runs with `cargo test -p deduxit --test iltp --
//! --ignored`, reading the library from `ILTP` or `bench/iltp/Problems`.

#![cfg(feature = "parse")]

use deduxit::Limits;
use deduxit::ordinary::{Logic, Verdict, ljt, read_tptp};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The most sequents LJT may expand on one problem: far more than any
/// problem the embedding decides needs, and a bound, so that a regression
/// fails instead of running on.
const WORK: u64 = 200_000_000;

/// The most memory one problem may hold. A proof of the larger SYJ201
/// instances unfolds to a derivation of millions of inferences, past the
/// default bound on a derivation's size, which is lifted here so that
/// every proof is checked: the memory bound keeps the run within reach.
const MEMORY: u64 = 6 << 30;

/// Returns the embedding's verdicts, per problem and logic: `true` for
/// valid, `false` for not valid, from every translation that decided.
fn oracle() -> BTreeMap<(String, String), Vec<bool>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/iltp/embedding.csv");
    let text = std::fs::read_to_string(&path).unwrap();
    let mut verdicts: BTreeMap<(String, String), Vec<bool>> = BTreeMap::new();
    for line in text.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        let [problem, logic, _translation, verdict] = fields[..] else {
            panic!("a line of four fields: {line}");
        };
        let valid = match verdict {
            "valid" => true,
            "not_valid" => false,
            _ => continue,
        };
        let key = (problem.to_owned(), logic.to_owned());
        verdicts.entry(key).or_default().push(valid);
    }
    verdicts
}

/// The library's `Problems` directory.
fn library() -> PathBuf {
    std::env::var_os("ILTP").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../bench/iltp/Problems"),
        PathBuf::from,
    )
}

#[test]
#[ignore = "needs the ILTP library, which is not in the repository: nix build .#iltp -o bench/iltp"]
fn ljt_agrees_with_the_embedding_on_iltp() {
    let library = library();
    let limits = Limits::default()
        .with_work(Some(WORK))
        .with_derivation_bytes(None)
        .with_memory_bytes(Some(MEMORY));
    let verdicts = oracle();
    assert!(!verdicts.is_empty());
    let mut agreed = 0;
    for ((problem, logic), embedded) in &verdicts {
        // The translations never disagree with each other.
        assert!(
            embedded.windows(2).all(|w| w[0] == w[1]),
            "{problem} in {logic} logic: {embedded:?}"
        );
        let text = std::fs::read_to_string(library.join(problem)).unwrap();
        let sequent = read_tptp(&text, &Limits::default()).unwrap().sequent;
        let logic: Logic = logic.parse().unwrap();
        let outcome = ljt::decide(&sequent, logic, &limits, |_| false)
            .unwrap_or_else(|e| panic!("{problem} in {logic} logic: {e}"));
        let valid = match &outcome.verdict {
            // `decide` checked it; checked again here.
            Verdict::Valid(derivation) => {
                derivation.check(&limits, |_| false).unwrap();
                true
            }
            Verdict::NotValid => false,
            other => panic!("{problem} in {logic} logic: {other:?}"),
        };
        assert_eq!(valid, embedded[0], "{problem} in {logic} logic");
        agreed += 1;
    }
    println!("{agreed} verdicts of the embedding agreed");
}

/// The most units of work LJT and the SAT solver may each spend on one
/// problem in [`sat_agrees_with_ljt_on_iltp`]: what a test run affords,
/// a few seconds a problem at most.
#[cfg(feature = "sat")]
const AFFORDED: u64 = 2_000_000;

/// The SAT solver against LJT on every propositional problem of the
/// library, in intuitionistic and minimal logic: where LJT decides within
/// [`AFFORDED`] sequents, the SAT solver gives the same verdict, every
/// derivation it returns passes the checker and every model refutes its
/// sequent.
#[cfg(feature = "sat")]
#[test]
#[ignore = "needs the ILTP library, which is not in the repository: nix build .#iltp -o bench/iltp"]
fn sat_agrees_with_ljt_on_iltp() {
    use deduxit::ordinary::sat;
    let library = library();
    let limits = Limits::default()
        .with_work(Some(AFFORDED))
        .with_derivation_bytes(None)
        .with_memory_bytes(Some(MEMORY));
    let mut problems: Vec<PathBuf> = Vec::new();
    let mut directories = vec![library.clone()];
    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(&directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                directories.push(path);
            } else if path.extension().is_some_and(|e| e == "p") {
                problems.push(path);
            }
        }
    }
    problems.sort();
    assert_eq!(problems.len(), 274);
    let (mut agreed, mut undecided) = (0, Vec::new());
    for path in &problems {
        let name = path.strip_prefix(&library).unwrap().display();
        let text = std::fs::read_to_string(path).unwrap();
        let sequent = read_tptp(&text, &Limits::default()).unwrap().sequent;
        for logic in [Logic::Intuitionistic, Logic::Minimal] {
            let decided = |verdict: &Verdict| match verdict {
                Verdict::Valid(_) => Some(true),
                Verdict::NotValid => Some(false),
                _ => None,
            };
            let by_ljt = ljt::decide(&sequent, logic, &limits, |_| false)
                .ok()
                .and_then(|o| decided(&o.verdict));
            let Some(expected) = by_ljt else { continue };
            let outcome = sat::decide(&sequent, logic, &limits, |_| false)
                .unwrap_or_else(|e| panic!("{name} in {logic} logic: {e}"));
            match (&outcome.verdict, &outcome.model) {
                // `decide` checked both; checked again here.
                (Verdict::Valid(derivation), None) => derivation.check(&limits, |_| false).unwrap(),
                (Verdict::NotValid, Some(model)) => {
                    model.refutes(&sequent, logic, |_| false).unwrap()
                }
                (Verdict::Unknown(_), None) => {
                    undecided.push(format!("{name} ({logic})"));
                    continue;
                }
                other => panic!("{name} in {logic} logic: {other:?}"),
            }
            assert_eq!(
                decided(&outcome.verdict),
                Some(expected),
                "{name} in {logic} logic"
            );
            agreed += 1;
        }
    }
    println!("{agreed} verdicts of LJT agreed; undecided by the SAT solver: {undecided:?}");
}
