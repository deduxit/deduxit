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
