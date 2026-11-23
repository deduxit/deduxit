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
        // A witness made a constant on one branch whose δ-formula a later
        // witness brings in.
        "forall w. (p(w) -> n), (exists u. s(u)) \\/ b, b -> forall y. (s(y) \\/ exists z. p(z)), forall v. ~s(v) |- n",
        // A unit that closes its clause under a γ-formula.
        "forall x. false |-",
        "|- exists x. true",
        "|- exists x. (false -> false)",
        "forall x. (p(x) /\\ false) |-",
        "|- forall x. exists y. true",
        // A witness whose δ-formula lies on another branch (ILTP's SYN375,
        // SYN374 and SYN069, Pelletier's 45).
        "|- (forall x. (p(x) <-> exists y. p(y))) <-> ((forall x. p(x)) <-> exists y. p(y))",
        "|- (exists x. forall y. (p(x) <-> p(y))) <-> ((exists x. p(x)) <-> forall y. p(y))",
        "forall x. ((f(x) /\\ forall y. ((g(y) /\\ h(x, y)) -> j(x, y))) -> forall y. (g(y) /\\ h(x, y) /\\ k(y))), ~exists y. (l(y) /\\ k(y)), exists x. (f(x) /\\ (forall y. (h(x, y) -> l(y))) /\\ forall y. ((g(y) /\\ h(x, y)) -> j(x, y))) |- exists x. (f(x) /\\ ~exists y. (g(y) /\\ h(x, y)))",
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
    // Intuitionistic logic has its prover; minimal logic none yet.
    let refused = decide(
        &sequent,
        Logic::Minimal,
        &options,
        &Limits::default(),
        |_| false,
    );
    assert!(matches!(
        refused,
        Err(crate::Error::Logic {
            method: Method::Connection,
            ..
        })
    ));
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
        // A bound may refuse a derivation; any other error is a defect.
        let ours = match decide(&sequent, Logic::Classical, &options, &limits, |_| false) {
            Ok(ours) => ours,
            Err(crate::Error::Refused(_)) => continue,
            Err(error) => panic!("{text}: {error}"),
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

/// The bounds hold where a search's cost is not in its steps: a deep nest
/// of quantifiers makes the matrix's tables quadratic, which the memory
/// bound refuses as unknown; a goal with thousands of failing extensions
/// meets the work bound at its step, not a thousand steps later; and a
/// bound of 0 on the active path is reported as 0.
#[test]
fn the_bounds_hold_where_steps_are_dear() {
    let k = 3_000;
    let binders: String = (0..k).map(|i| format!("forall x{i}. ")).collect();
    let body: Vec<String> = (0..k).map(|i| format!("q{i}(x0)")).collect();
    let text = format!("{binders}({}) |- r(a)", body.join(" /\\ "));
    let sequent: Sequent = text.parse().unwrap();
    let limits = Limits {
        memory_bytes: Some(16 << 20),
        ..Limits::default()
    };
    let outcome = decide(
        &sequent,
        Logic::Classical,
        &search::Options::default(),
        &limits,
        |_| false,
    )
    .unwrap();
    assert!(
        matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::MemoryLimit { .. })
        ),
        "{:?}",
        outcome.verdict
    );

    let hypotheses: Vec<String> = (0..2_000).map(|i| format!("p(c{i})")).collect();
    let goals: Vec<String> = (0..50).map(|j| format!("p(d{j})")).collect();
    let text = format!("{} |- {}", hypotheses.join(", "), goals.join(", "));
    let sequent: Sequent = text.parse().unwrap();
    let limits = Limits {
        work: Some(1_000),
        ..Limits::default()
    };
    let outcome = decide(
        &sequent,
        Logic::Classical,
        &search::Options::default(),
        &limits,
        |_| false,
    )
    .unwrap();
    assert!(matches!(
        outcome.verdict,
        Verdict::Unknown(Reason::WorkLimit { limit: 1_000 })
    ));
    assert!(outcome.statistics.steps <= 1_000 + 2 * 1_024);

    let sequent: Sequent = "forall x. (p(x) -> p(f(x))), p(a) |- p(f(f(a)))"
        .parse()
        .unwrap();
    let options = search::Options::default().with_path_length(Some(0));
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
        Verdict::Unknown(Reason::PathLength { length: 0 })
    ));
}

/// The read-back of a congruence axiom of many arguments instantiates a
/// nest of quantifiers one step at a time, each instance as large as the
/// rest of the nest, before it makes any inference: it asks the stop at
/// every step of that branch, and its arena is charged to the memory
/// bound, which answers unknown where the search alone fits.
#[test]
fn the_read_back_asks_the_stop_and_charges_its_arena() {
    let n = 200;
    let left = vec!["a"; n].join(", ");
    let right = vec!["a"; n - 1].join(", ");
    let text = format!("p(f({left})), a = b |- p(f(b, {right}))");
    let sequent: Sequent = text.parse().unwrap();
    let mut early = 0;
    let outcome = decide(
        &sequent,
        Logic::Classical,
        &search::Options::default(),
        &Limits::default(),
        |progress| {
            // A poll of the read-back before its first inference.
            if progress.phase == crate::Phase::ReadBack && progress.work == 1 && progress.done == 1
            {
                early += 1;
            }
            false
        },
    )
    .unwrap();
    assert!(
        matches!(outcome.verdict, Verdict::Valid(_)),
        "{:?}",
        outcome.verdict
    );
    assert!(early >= n, "{early} polls before the first inference");
    let limits = Limits {
        memory_bytes: Some(4 << 20),
        ..Limits::default()
    };
    let outcome = decide(
        &sequent,
        Logic::Classical,
        &search::Options::default(),
        &limits,
        |_| false,
    )
    .unwrap();
    assert!(
        matches!(
            outcome.verdict,
            Verdict::Unknown(Reason::MemoryLimit { .. })
        ),
        "{:?}",
        outcome.verdict
    );
}

/// The matrix's set-up asks the stop by the work it does, not by the
/// nodes it lays out: under a nest of thousands of quantifiers each of a
/// hundred literals has a list of slots as long as the nest, though the
/// matrix has a few hundred nodes; and a literal whose terms are Skolem
/// terms, each over every slot around it, makes its patterns between two
/// polls no more.
#[test]
fn the_matrix_asks_the_stop_by_its_work() {
    // Polled by the node or once per literal, as before, each made at most
    // five polls (the unfolding's, the build's two, one more).
    const POLLS_OF_SLOTS: usize = 10;
    const POLLS_OF_SKOLEMS: usize = 7;
    let polls = |text: &str| {
        let sequent: Sequent = text.parse().unwrap();
        let mut polls = 0;
        let built = super::matrix::Matrix::of(&sequent, &Limits::default(), &mut |_| {
            polls += 1;
            false
        });
        assert!(built.is_ok());
        polls
    };
    let binders: String = (0..3_000).map(|i| format!("forall x{i}. ")).collect();
    let body: Vec<String> = (0..100).map(|j| format!("q{j}(c)")).collect();
    let slots = polls(&format!("{binders}({}) |- r", body.join(" /\\ ")));
    assert!(slots >= POLLS_OF_SLOTS, "{slots} polls");
    let nest: String = (0..1_000)
        .map(|i| format!("forall x{i}. exists y{i}. "))
        .collect();
    let ys: Vec<String> = (0..1_000).map(|i| format!("y{i}")).collect();
    let skolems = polls(&format!("{nest}p({}) |- r", ys.join(", ")));
    assert!(skolems >= POLLS_OF_SKOLEMS, "{skolems} polls");
}

/// The inferences that close a branch keep their sides, which the read-back
/// charges to the memory bound: an axiom under thousands of hypotheses is
/// as many weakenings, each a sequent one formula longer.
#[test]
fn the_read_back_charges_the_sides_of_an_axiom_s_weakenings() {
    let hypotheses: Vec<String> = (0..3_000).map(|i| format!("q{i}")).collect();
    let text = format!("{}, p |- p", hypotheses.join(", "));
    let sequent: Sequent = text.parse().unwrap();
    let decided = |limits: &Limits| {
        decide(
            &sequent,
            Logic::Classical,
            &search::Options::default(),
            limits,
            |_| false,
        )
        .unwrap()
        .verdict
    };
    assert!(matches!(decided(&Limits::default()), Verdict::Valid(_)));
    let limits = Limits {
        memory_bytes: Some(8 << 20),
        ..Limits::default()
    };
    let verdict = decided(&limits);
    assert!(
        matches!(verdict, Verdict::Unknown(Reason::MemoryLimit { .. })),
        "{verdict:?}"
    );
}

/// A chain of witnesses `f(x, x)`, each bound to the one before, shares
/// its parts: walked as trees, its terms are exponential in its length.
/// A short chain is proved classically; a long one is refused in both
/// logics by the bound on terms, before an atom past it is built.
#[test]
fn shared_witnesses_are_walked_once_and_held_to_the_terms_bound() {
    let chain = |n: usize| -> Sequent {
        let mut hypotheses = vec!["p0(a)".to_owned()];
        hypotheses.extend((0..n).map(|i| format!("forall x. (p{i}(x) -> p{}(f(x, x)))", i + 1)));
        format!("{} |- exists y. p{n}(y)", hypotheses.join(", "))
            .parse()
            .unwrap()
    };
    let small = connection(&chain(6));
    assert!(
        matches!(small.verdict, Verdict::Valid(_)),
        "{:?}",
        small.verdict
    );
    let limits = Limits::default().with_terms(Some(100_000));
    // No bound on the active path, as the command searches.
    let options = search::Options::default().with_path_length(None);
    for logic in [Logic::Classical, Logic::Intuitionistic] {
        let refused = decide(&chain(40), logic, &options, &limits, |_| false);
        assert!(
            matches!(
                refused,
                Err(crate::Error::Refused(crate::Refusal::Terms { .. }))
            ),
            "{logic:?}: {refused:?}"
        );
    }
}

/// A part of a shared term is its symbol, its arguments and its copy: a
/// constant's empty list starts where the next pattern's arguments do,
/// and keyed by that start alone the pair `(a, a)` stood for `(f(x),
/// f(x))`, which unification then skipped. Past the walk's first visits
/// it unified `t(g(f(b), …))` with `t(g(f(c), …))` and read back an open
/// branch (`Error::ReadBack`). The not-valid sequent stays unknown; and
/// the bound on terms takes an instance whose atom the arena holds as no
/// growth.
#[test]
fn a_shared_part_is_named_whole_and_the_terms_bound_is_exact() {
    let deep = format!("{}e{}", "k(".repeat(70), ")".repeat(70));
    let sequent: Sequent = format!(
        "r(a), forall x. t(g(f(x), a, {deep}), x), \
         forall y. forall z. (t(y, b) /\\ t(z, c) -> u(y, z)), \
         forall w. (u(w, w) -> goal) |- goal"
    )
    .parse()
    .unwrap();
    let outcome = connection(&sequent);
    assert!(
        matches!(outcome.verdict, Verdict::Unknown(_)),
        "{:?}",
        outcome.verdict
    );
    let held: Sequent = "p(f(f(f(f(c))))), forall x. (p(x) -> q) |- q"
        .parse()
        .unwrap();
    let limits = Limits::default().with_terms(Some(6));
    let outcome = decide(
        &held,
        Logic::Classical,
        &search::Options::default(),
        &limits,
        |_| false,
    )
    .unwrap();
    assert!(
        matches!(outcome.verdict, Verdict::Valid(_)),
        "{:?}",
        outcome.verdict
    );
}
