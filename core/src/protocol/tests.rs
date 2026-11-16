// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The protocol's tests: the registry, and each kind of operation run and
//! checked as a front end would.

use super::{
    OPERATIONS, Status, Verdict, catalogue, check, kind, operation, read, run, suites, summary,
    verdict_of,
};
use crate::{Error, Settings};

/// The settings every test runs under: one thread, so a run is a function
/// of its inputs.
fn settings() -> Settings {
    Settings::default().deterministic()
}

/// Returns an ordinary statement of `text` in `logic`.
fn ordinary(text: &str, logic: crate::ordinary::Logic) -> String {
    super::ordinary::write_statement(&text.parse().unwrap(), logic).unwrap()
}

/// Runs an operation under the tests' settings and returns its output.
fn ran(operation: &str, inputs: &[&str], parameters: &str) -> String {
    run(operation, inputs, parameters, &settings(), |_| false)
        .unwrap()
        .value
}

/// Makes an operation's check under the tests' settings.
fn checked(operation: &str, inputs: &[&str], parameters: &str, output: &str) -> super::Check {
    check(operation, inputs, parameters, output, &settings(), |_| {
        false
    })
    .unwrap()
}

#[test]
fn the_names_list_every_operation_and_kind_once() {
    let names: Vec<&str> = suites()
        .iter()
        .flat_map(|suite| suite.operations().iter().map(|operation| operation.name))
        .collect();
    assert_eq!(names, OPERATIONS);
    for suite in suites() {
        for operation in suite.operations() {
            assert!(operation.name.starts_with(&format!("{}.", suite.name())));
            for kinds in operation.from {
                for name in *kinds {
                    kind(name).unwrap();
                }
            }
            kind(operation.to).unwrap();
            assert!(operation.reads.iter().all(|key| key.starts_with("limits.")
                || ["search", "view", "cut", "lambda"].contains(key)
                || key.starts_with("styles.")));
        }
    }
    let json = serde_json::to_value(catalogue()).unwrap();
    assert_eq!(json["suites"].as_array().unwrap().len(), 3);
    assert_eq!(
        json["suites"][1]["operations"][4]["promise"]["name"],
        "same_verdict"
    );
    assert!(matches!(
        operation("ordinary.nothing"),
        Err(Error::UnknownName { .. })
    ));
}

#[test]
fn a_call_with_other_inputs_or_parameters_is_refused() {
    let sequent = ordinary("|- a", crate::ordinary::Logic::Classical);
    let refused = |inputs: &[&str], parameters: &str| {
        run(
            "ordinary.translate",
            inputs,
            parameters,
            &settings(),
            |_| false,
        )
        .unwrap_err()
        .code()
    };
    assert_eq!(refused(&[], ""), "invalid_option");
    assert_eq!(
        refused(&[&sequent], r#"{"by": "kuroda"}"#),
        "invalid_option"
    );
    assert_eq!(refused(&["{}"], ""), "json");
    // A translation is of a classical sequent.
    let intuitionistic = ordinary("|- a", crate::ordinary::Logic::Intuitionistic);
    assert_eq!(refused(&[&intuitionistic], ""), "invalid_option");
}

#[test]
fn a_translation_keeps_the_verdict_and_its_check_says_so() {
    let sequent = ordinary("|- a \\/ ~a", crate::ordinary::Logic::Classical);
    let parameters = r#"{"translation": "glivenko"}"#;
    let translated = ran("ordinary.translate", &[&sequent], parameters);
    let limits = settings().limits;
    assert_eq!(
        summary("ordinary.sequent", &translated, &limits).unwrap(),
        "⊢ ¬¬(a ∨ ¬a)"
    );
    // The output is claimed in the translation's target.
    assert!(translated.contains(r#""logic":"intuitionistic""#));
    let check = checked("ordinary.translate", &[&sequent], parameters, &translated);
    assert_eq!(check.status, Status::Held);
    assert_eq!(check.parts.len(), 2);
    assert!(
        check
            .parts
            .iter()
            .all(|p| p.verdict == Some(Verdict::Valid))
    );
    // The translation decided, and its evidence checked.
    let decided = ran("ordinary.decide", &[&translated], "");
    assert_eq!(verdict_of(&decided).unwrap(), Verdict::Valid);
    assert_eq!(
        checked("ordinary.decide", &[&translated], "", &decided).status,
        Status::Held
    );
}

#[test]
fn a_sequent_that_is_not_valid_has_a_checked_countermodel() {
    let sequent = ordinary("a \\/ b |- a", crate::ordinary::Logic::Intuitionistic);
    let decided = ran("ordinary.decide", &[&sequent], r#"{"method": "sat"}"#);
    let limits = settings().limits;
    assert_eq!(
        summary("ordinary.outcome", &decided, &limits).unwrap(),
        "Not valid"
    );
    assert_eq!(
        checked("ordinary.decide", &[&sequent], "", &decided).status,
        Status::Held
    );
    #[cfg(feature = "svg")]
    {
        let drawing = ran("ordinary.draw", &[&decided], "");
        read("drawing", &drawing, &limits).unwrap();
        assert!(drawing.contains("<svg"));
    }
    // A sequent that is not valid has no derivation.
    let error = run("ordinary.derivation", &[&decided], "", &settings(), |_| {
        false
    })
    .unwrap_err();
    assert_eq!(error.code(), "no_derivation");
}

#[test]
fn an_unknown_names_the_bound_that_ended_it() {
    let sequent = ordinary("|- ((a -> b) -> a) -> a", crate::ordinary::Logic::Classical);
    let tight = settings().with_limits(settings().limits.with_work(Some(1)));
    let produced = run(
        "ordinary.decide",
        &[&sequent],
        r#"{"method": "embedding"}"#,
        &tight,
        |_| false,
    )
    .unwrap();
    let limit = produced.limit.unwrap();
    assert_eq!(limit.setting, Some("limits.work"));
    assert_eq!(verdict_of(&produced.value).unwrap(), Verdict::Unknown);
}

#[test]
fn an_interpolant_and_its_halves() {
    let sequent = ordinary("a /\\ b |- b \\/ c", crate::ordinary::Logic::Classical);
    let interpolant = ran("ordinary.interpolate", &[&sequent], "");
    let limits = settings().limits;
    assert!(
        summary("ordinary.interpolant", &interpolant, &limits)
            .unwrap()
            .starts_with("Interpolant ")
    );
    let check = checked("ordinary.interpolate", &[&sequent], "", &interpolant);
    assert_eq!(check.status, Status::Held, "{check:?}");
    assert_eq!(check.parts.len(), 3);
    let second = ran("ordinary.half", &[&interpolant], r#"{"which": "second"}"#);
    read("ordinary.sequent", &second, &limits).unwrap();
    let session = ran("ordinary.start", &[&second], "");
    assert_eq!(
        summary("ordinary.session", &session, &limits).unwrap(),
        "1 open goal"
    );
}

#[test]
fn an_embedding_keeps_the_verdict() {
    let sequent = ordinary("a, a -> b |- b", crate::ordinary::Logic::Intuitionistic);
    let image = ran("ordinary.embed", &[&sequent], "");
    read("linear.sequent", &image, &settings().limits).unwrap();
    let check = checked("ordinary.embed", &[&sequent], "", &image);
    assert_eq!(check.status, Status::Held);
}

#[test]
fn a_linear_proof_its_net_and_its_certificate() {
    let sequent = super::linear::write_statement(
        &"|- a * b, ~a, ~b".parse().unwrap(),
        crate::linear::Mode::CLASSICAL,
    )
    .unwrap();
    let outcome = ran("linear.prove", &[&sequent], "");
    assert_eq!(
        checked("linear.prove", &[&sequent], "", &outcome).status,
        Status::Held
    );
    // An outcome of a proof reads as the proof.
    let proof: crate::linear::Proof = super::document(&outcome, &settings().limits).unwrap();
    let proof = super::written(&proof).unwrap();
    let net = ran("linear.net", &[&proof], "");
    assert_eq!(
        checked("linear.net", &[&proof], "", &net).status,
        Status::Held
    );
    #[cfg(feature = "rocq")]
    {
        let certificate = ran("linear.certificate", &[&outcome], "");
        assert_eq!(
            checked("linear.certificate", &[&outcome], "", &certificate).status,
            Status::Outside
        );
    }
    let session = ran("linear.start", &[&sequent], "");
    let goal = ran("linear.goal", &[&session], r#"{"goal": 0}"#);
    assert_eq!(
        summary("linear.sequent", &goal, &settings().limits).unwrap(),
        summary("linear.sequent", &sequent, &settings().limits).unwrap()
    );
}

#[test]
fn an_intuitionistic_goal_keeps_its_sides() {
    let sequent = super::linear::write_statement(
        &crate::linear::IllSequent::parse_within("a, a -o b |- b", &settings().limits)
            .unwrap()
            .into_sequent(),
        crate::linear::Mode::INTUITIONISTIC,
    )
    .unwrap();
    let session = ran("linear.start", &[&sequent], "");
    let goal = ran("linear.goal", &[&session], r#"{"goal": 0}"#);
    let outcome = ran("linear.prove", &[&goal], "");
    assert_eq!(verdict_of(&outcome).unwrap(), Verdict::Valid);
}

#[test]
fn a_calculus_given_as_data_starts_a_derivation() {
    let session = ran(
        "rules.start",
        &[crate::rules::LJ, r#"{"goal": "a |- a"}"#],
        "",
    );
    assert_eq!(
        summary("rules.session", &session, &settings().limits).unwrap(),
        "1 open goal"
    );
    // A derivation of LJ, read as the rule file's.
    let sequent = ordinary("a /\\ b |- b", crate::ordinary::Logic::Intuitionistic);
    let decided = ran("ordinary.decide", &[&sequent], r#"{"method": "ljt"}"#);
    let derivation = ran("ordinary.derivation", &[&decided], "");
    let generic = ran("rules.from_ordinary", &[&derivation], "");
    assert_eq!(
        checked("rules.from_ordinary", &[&derivation], "", &generic).status,
        Status::Held
    );
}
