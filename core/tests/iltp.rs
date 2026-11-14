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

/// The longest the SAT solver may search one problem in
/// [`sat_agrees_with_ljt_on_iltp`], by the test's own clock: what a test
/// run affords.
#[cfg(feature = "sat")]
const AFFORDED: std::time::Duration = std::time::Duration::from_secs(20);

/// The SAT solver against LJT on the library: every problem LJT decides
/// at the command's 2 s (`tests/iltp/ljt.csv`, its verdicts in the run
/// that made it the default) gets the same verdict from the SAT solver
/// where that decides within [`AFFORDED`], every derivation it returns
/// passes the checker and every model refutes its sequent. The problems
/// it leaves undecided are printed.
#[cfg(feature = "sat")]
#[test]
#[ignore = "needs the ILTP library, which is not in the repository: nix build .#iltp -o bench/iltp"]
fn sat_agrees_with_ljt_on_iltp() {
    use deduxit::ordinary::sat;
    use std::time::Instant;
    let library = library();
    let limits = Limits::default()
        .with_work(Some(WORK))
        .with_derivation_bytes(None)
        .with_memory_bytes(Some(MEMORY));
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/iltp/ljt.csv");
    let oracle = std::fs::read_to_string(&path).unwrap();
    let (mut agreed, mut undecided) = (0, Vec::new());
    for line in oracle.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        let [problem, logic, verdict] = fields[..] else {
            panic!("a line of three fields: {line}");
        };
        let expected = verdict == "valid";
        let text = std::fs::read_to_string(library.join(problem)).unwrap();
        let sequent = read_tptp(&text, &Limits::default()).unwrap().sequent;
        let logic: Logic = logic.parse().unwrap();
        let start = Instant::now();
        let outcome = sat::decide(&sequent, logic, &limits, |_| start.elapsed() > AFFORDED)
            .unwrap_or_else(|e| panic!("{problem} in {logic} logic: {e}"));
        let time = start.elapsed();
        let valid = match (&outcome.verdict, &outcome.model) {
            // `decide` checked both; checked again here.
            (Verdict::Valid(derivation), None) => {
                derivation.check(&limits, |_| false).unwrap();
                true
            }
            (Verdict::NotValid, Some(model)) => {
                model.refutes(&sequent, logic, |_| false).unwrap();
                false
            }
            (Verdict::Unknown(reason), None) => {
                undecided.push(format!("{problem} ({logic}): {reason}"));
                continue;
            }
            other => panic!("{problem} in {logic} logic: {other:?}"),
        };
        eprintln!(
            "{problem} {logic} {valid} {:.3}s {:?}",
            time.as_secs_f64(),
            outcome.statistics
        );
        assert_eq!(valid, expected, "{problem} in {logic} logic");
        agreed += 1;
    }
    println!("{agreed} verdicts of LJT agreed; undecided: {undecided:#?}");
}

/// The most work the embedding and the SAT solver may do on one problem
/// in [`sat_agrees_with_the_embedding_in_classical_logic`]: what a test
/// run affords.
#[cfg(feature = "sat")]
const CLASSICAL_WORK: u64 = 2_000_000;

/// The SAT solver against the embedding in classical logic: every
/// problem of the library that the embedding decides within
/// [`CLASSICAL_WORK`] gets the same verdict from the SAT solver within
/// the same limits, its clausal proof or its assignment checked; the
/// problems only the SAT solver decides, those it proves without the
/// embedding's derivation and those where it or a check stops are
/// printed.
#[cfg(feature = "sat")]
#[test]
#[ignore = "needs the ILTP library, which is not in the repository: nix build .#iltp -o bench/iltp"]
fn sat_agrees_with_the_embedding_in_classical_logic() {
    use deduxit::ordinary::{Options, decide, sat};
    use std::collections::BTreeSet;
    let library = library();
    let limits = Limits::default()
        .with_work(Some(CLASSICAL_WORK))
        .with_memory_bytes(Some(2 << 30));
    // Every problem of the library has rows in the embedding's verdicts.
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/iltp/embedding.csv");
    let problems: BTreeSet<String> = std::fs::read_to_string(&path)
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.split(',').next().unwrap().to_owned())
        .collect();
    assert_eq!(problems.len(), 274);
    let options = Options::default().with_logic(Logic::Classical);
    let search = deduxit::linear::Options::default();
    let (mut agreed, mut only_sat, mut proved_alone, mut stopped) =
        (0, Vec::new(), Vec::new(), Vec::new());
    for problem in &problems {
        let text = std::fs::read_to_string(library.join(problem)).unwrap();
        let sequent = read_tptp(&text, &Limits::default()).unwrap().sequent;
        let embedded = match decide(&sequent, &options, &search, &limits, |_| false) {
            Ok(outcome) => match outcome.verdict {
                Verdict::Valid(_) => Some(true),
                Verdict::NotValid => Some(false),
                _ => None,
            },
            Err(deduxit::Error::Refused(_)) => None,
            Err(error) => panic!("{problem}: {error}"),
        };
        let solved = match sat::decide(&sequent, Logic::Classical, &limits, |_| false) {
            Ok(outcome) => match (&outcome.verdict, &outcome.model) {
                (Verdict::Valid(derivation), None) => {
                    derivation.check(&limits, |_| false).unwrap();
                    Some(true)
                }
                (Verdict::NotValid, Some(model)) => {
                    model
                        .refutes(&sequent, Logic::Classical, |_| false)
                        .unwrap();
                    Some(false)
                }
                // The clausal proof passed; the embedding's search for the
                // derivation stopped.
                (Verdict::Unknown(_), None) if outcome.statistics.proof_steps > 0 => {
                    proved_alone.push(problem.clone());
                    None
                }
                (Verdict::Unknown(reason), None) => {
                    stopped.push(format!("{problem}: {reason}"));
                    None
                }
                other => panic!("{problem}: {other:?}"),
            },
            // A bound refused the check of the clausal proof, or the
            // read-back of the embedding's proof.
            Err(deduxit::Error::Refused(refusal)) => {
                stopped.push(format!("{problem}: {refusal}"));
                None
            }
            Err(error) => panic!("{problem}: {error}"),
        };
        match (embedded, solved) {
            (Some(e), Some(s)) => {
                assert_eq!(e, s, "{problem}");
                agreed += 1;
            }
            (Some(e), None) => assert!(e, "{problem}: refuted by the embedding alone"),
            (None, Some(s)) => only_sat.push((problem.clone(), s)),
            (None, None) => {}
        }
    }
    println!(
        "{agreed} verdicts of the embedding agreed; decided by the solver alone: {only_sat:?}; \
         proved by a checked clausal proof without the embedding's derivation: {proved_alone:?}; \
         the solver stopped: {stopped:#?}"
    );
}
