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
    assert_eq!(json["suites"].as_array().unwrap().len(), 5);
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

/// Every normal form is an edge between two sequents whose check decides
/// its promise's claims, and a wrong output fails that check.
#[test]
fn a_normal_form_keeps_its_promise_and_its_check_says_so() {
    let classical = crate::ordinary::Logic::Classical;
    let sequent = ordinary("a -> b |- ~(b /\\ c)", classical);
    let limits = settings().limits;
    for (operation, parts, written) in [
        ("ordinary.nnf", 1, "¬a ∨ b ⊢ ¬b ∨ ¬c"),
        ("ordinary.cnf", 1, "¬a ∨ b ⊢ ¬b ∨ ¬c"),
        ("ordinary.dnf", 1, "¬a ∨ b ⊢ ¬b ∨ ¬c"),
        (
            "ordinary.tseitin",
            3,
            "t1, t3, (¬t1 ∨ ¬a) ∨ b, t1 ∨ a, t1 ∨ ¬b, ¬t2 ∨ b, ¬t2 ∨ c, (t2 ∨ ¬b) ∨ ¬c, \
             ¬t3 ∨ t2, t3 ∨ ¬t2 ⊢",
        ),
        (
            "ordinary.plaisted_greenbaum",
            3,
            "t1, t3, (¬t1 ∨ ¬a) ∨ b, ¬t2 ∨ b, ¬t2 ∨ c, ¬t3 ∨ t2 ⊢",
        ),
    ] {
        let output = ran(operation, &[&sequent], "");
        assert_eq!(
            summary("ordinary.sequent", &output, &limits).unwrap(),
            written,
            "{operation}"
        );
        let check = checked(operation, &[&sequent], "", &output);
        assert_eq!(check.status, Status::Held, "{operation}");
        assert_eq!(check.parts.len(), parts, "{operation}");
        let wrong = ordinary("a |- b", classical);
        let check = checked(operation, &[&sequent], "", &wrong);
        assert_eq!(check.status, Status::Failed, "{operation}");
    }
    // The same formula claimed in intuitionistic logic, where `a ∨ ¬a`
    // is not valid, is no normal form of the classical sequent.
    let excluded = ordinary("a \\/ ~a", classical);
    let claimed = ordinary("a \\/ ~a", crate::ordinary::Logic::Intuitionistic);
    let check = checked("ordinary.nnf", &[&excluded], "", &claimed);
    assert_eq!(check.status, Status::Failed);
    let intuitionistic = ordinary("a", crate::ordinary::Logic::Intuitionistic);
    assert!(matches!(
        run("ordinary.cnf", &[&intuitionistic], "", &settings(), |_| {
            false
        }),
        Err(Error::InvalidOption { key: "logic", .. })
    ));
}

/// The first-order forms are edges between two sequents too, and their
/// checks hold a right output and fail a wrong one.
#[test]
fn a_first_order_normal_form_keeps_its_promise() {
    let classical = crate::ordinary::Logic::Classical;
    let sequent = ordinary(
        "forall x. exists y. r(x, y) |- exists z. r(a, z)",
        classical,
    );
    let limits = settings().limits;
    for (operation, written) in [
        ("ordinary.prenex", "∀x. ∃y. r(x, y) ⊢ ∃z. r(a, z)"),
        ("ordinary.miniscope", "∀x. ∃y. r(x, y) ⊢ ∃z. r(a, z)"),
        ("ordinary.skolem", "∀x. r(x, sk1(x)), ∀z. ¬r(a, z) ⊢"),
        ("ordinary.herbrand", "∀x. r(x, sk1(x)) ⊢ ∃z. r(a, z)"),
        ("ordinary.clausal", "∀x. r(x, sk1(x)), ∀z. ¬r(a, z) ⊢"),
    ] {
        let output = ran(operation, &[&sequent], "");
        assert_eq!(
            summary("ordinary.sequent", &output, &limits).unwrap(),
            written,
            "{operation}"
        );
        let check = checked(operation, &[&sequent], "", &output);
        assert_eq!(check.status, Status::Held, "{operation}");
        let wrong = ordinary("forall x. p(x) |-", classical);
        let check = checked(operation, &[&sequent], "", &wrong);
        assert_eq!(check.status, Status::Failed, "{operation}");
    }
}

/// A resolution decides a sequent's clauses by either method; its check
/// holds the clauses to the input's and the refutation or model to the
/// clauses, and fails a refutation changed after it was made.
#[test]
fn a_resolution_is_checked() {
    let classical = crate::ordinary::Logic::Classical;
    let limits = settings().limits;
    let valid = ordinary("a -> b, a |- b", classical);
    for parameters in ["", r#"{"method": "dpll", "form": "tseitin"}"#] {
        let output = ran("ordinary.resolve", &[&valid], parameters);
        assert_eq!(
            summary("ordinary.resolution", &output, &limits).unwrap(),
            "Unsatisfiable"
        );
        let check = checked("ordinary.resolve", &[&valid], parameters, &output);
        assert_eq!(check.status, Status::Held, "{parameters}");
        let broken = output.replace(r#""clause":[]"#, r#""clause":[1]"#);
        assert_ne!(broken, output);
        let check = checked("ordinary.resolve", &[&valid], parameters, &broken);
        assert_eq!(check.status, Status::Failed, "{parameters}");
    }
    let not_valid = ordinary("a \\/ b |- a", classical);
    let output = ran("ordinary.resolve", &[&not_valid], "");
    assert_eq!(
        summary("ordinary.resolution", &output, &limits).unwrap(),
        "Satisfiable"
    );
    assert_eq!(
        checked("ordinary.resolve", &[&not_valid], "", &output).status,
        Status::Held
    );
    let other = ran("ordinary.resolve", &[&valid], "");
    assert_eq!(
        checked("ordinary.resolve", &[&not_valid], "", &other).status,
        Status::Failed
    );
}

/// A first-order sequent the model search refutes by a Kripke model with
/// growing domains: the check of `decides` checks the model again, and
/// fails a forged one.
#[test]
fn a_kripke_model_is_checked_again() {
    let sequent = ordinary(
        "(forall x. p(x)) -> q |- exists x. (p(x) -> q)",
        crate::ordinary::Logic::Intuitionistic,
    );
    let decided = ran("ordinary.decide", &[&sequent], r#"{"method": "models"}"#);
    assert!(decided.contains("kripke_structure"), "{decided}");
    assert_eq!(
        checked(
            "ordinary.decide",
            &[&sequent],
            r#"{"method": "models"}"#,
            &decided
        )
        .status,
        Status::Held
    );
    // The upper world's predicate taken away: the model forces the goal.
    let forged = decided.replace(r#","predicates":[["p",[[0]]]]"#, "");
    assert_ne!(forged, decided);
    assert_eq!(
        checked(
            "ordinary.decide",
            &[&sequent],
            r#"{"method": "models"}"#,
            &forged
        )
        .status,
        Status::Failed
    );
}

/// Herbrand's theorem gives a valid sequent its instances and a derivation,
/// which the check holds to the input and passes; an unknown at the depth
/// leaves nothing to check.
#[cfg(feature = "sat")]
#[test]
fn herbrand_s_instances_are_checked() {
    let classical = crate::ordinary::Logic::Classical;
    let limits = settings().limits;
    let valid = ordinary("|- exists x. (p(x) -> forall y. p(y))", classical);
    let output = ran("ordinary.instances", &[&valid], "");
    assert_eq!(
        summary("ordinary.instances", &output, &limits).unwrap(),
        "Valid by Herbrand's theorem"
    );
    let check = checked("ordinary.instances", &[&valid], "", &output);
    assert_eq!(check.status, Status::Held);
    let other = ordinary("|- exists x. (q(x) -> forall y. q(y))", classical);
    let check = checked("ordinary.instances", &[&other], "", &output);
    assert_eq!(check.status, Status::Failed);
    let open = ordinary("forall x. (p(x) -> p(f(x))) |- p(a)", classical);
    let produced = run(
        "ordinary.instances",
        &[&open],
        r#"{"depth": 1}"#,
        &settings(),
        |_| false,
    )
    .unwrap();
    assert!(produced.limit.is_some());
    let check = checked("ordinary.instances", &[&open], "", &produced.value);
    assert_eq!(check.status, Status::None);
}

/// A first-order model is drawn, exported as text and as tables, and
/// certified, the Kripke model and the finite structure alike.
#[test]
fn a_first_order_model_is_drawn_and_certified() {
    let kripke = ordinary(
        "(forall x. p(x)) -> q |- exists x. (p(x) -> q)",
        crate::ordinary::Logic::Intuitionistic,
    );
    let structure = ordinary(
        "exists x. p(x) |- forall x. p(x)",
        crate::ordinary::Logic::Classical,
    );
    for (sequent, words, theorem) in [
        (&kripke, "growing domains", "fo_kripke_refutation"),
        (&structure, "finite structure", "fo_classical_refutation"),
    ] {
        let decided = ran("ordinary.decide", &[sequent], r#"{"method": "models"}"#);
        #[cfg(feature = "svg")]
        assert!(ran("ordinary.draw", &[&decided], "").contains(words));
        assert!(ran("ordinary.export", &[&decided], "").contains("domain"));
        #[cfg(feature = "latex")]
        assert!(ran("ordinary.export", &[&decided], r#"{"format": "latex"}"#).contains("tabular"));
        #[cfg(feature = "typst")]
        assert!(ran("ordinary.export", &[&decided], r#"{"format": "typst"}"#).contains("#table"));
        #[cfg(feature = "rocq")]
        assert!(ran("ordinary.certificate", &[&decided], "").contains(theorem));
        let _ = (words, theorem);
    }
}

/// First-order resolution answers a first-order statement: its check makes
/// the clauses and every inference again, and fails a refutation whose
/// unifier was changed.
#[cfg(feature = "sat")]
#[test]
fn a_first_order_resolution_is_checked() {
    let classical = crate::ordinary::Logic::Classical;
    let limits = settings().limits;
    let valid = ordinary(
        "forall x. (p(x) -> q(f(x))), p(a) |- exists y. q(y)",
        classical,
    );
    let output = ran("ordinary.resolve", &[&valid], "");
    assert_eq!(
        summary("ordinary.resolution", &output, &limits).unwrap(),
        "Unsatisfiable"
    );
    let check = checked("ordinary.resolve", &[&valid], "", &output);
    assert_eq!(check.status, Status::Held);
    let broken = output.replace("X1 := a", "X1 := f(a)");
    assert_ne!(broken, output);
    let check = checked("ordinary.resolve", &[&valid], "", &broken);
    assert_eq!(check.status, Status::Failed);
    let dpll = run(
        "ordinary.resolve",
        &[&valid],
        r#"{"method": "dpll"}"#,
        &settings(),
        |_| false,
    );
    assert!(matches!(dpll, Err(Error::InvalidOption { .. })));
    // A given `□` is a refutation of no step, which holds.
    let given = ordinary("false, p(a) |-", classical);
    let output = ran("ordinary.resolve", &[&given], "");
    let check = checked("ordinary.resolve", &[&given], "", &output);
    assert_eq!(check.status, Status::Held);
    // An atom named `clausal` leaves a propositional output propositional.
    let named = ordinary("clausal |- clausal", classical);
    let output = ran("ordinary.resolve", &[&named], "");
    assert_eq!(
        summary("ordinary.resolution", &output, &limits).unwrap(),
        "Unsatisfiable"
    );
    let check = checked("ordinary.resolve", &[&named], "", &output);
    assert_eq!(check.status, Status::Held);
    // With `=`, the derivation read back has reflexivity among its
    // hypotheses, and the check holds it to that sequent.
    let reflexive = ordinary("|- forall x. x = x", classical);
    let output = ran("ordinary.resolve", &[&reflexive], "");
    assert!(output.contains(r#""derivation":{"#), "{output}");
    let check = checked("ordinary.resolve", &[&reflexive], "", &output);
    assert_eq!(check.status, Status::Held);
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
    assert_eq!(check.parts.len(), 4);
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

#[test]
fn a_check_with_a_side_not_decided_is_open() {
    let sequent = ordinary("|- a \\/ ~a", crate::ordinary::Logic::Classical);
    let parameters = r#"{"translation": "glivenko"}"#;
    let translated = ran("ordinary.translate", &[&sequent], parameters);
    let tight = settings().with_limits(settings().limits.with_work(Some(1)));
    let check = check(
        "ordinary.translate",
        &[&sequent],
        parameters,
        &translated,
        &tight,
        |_| false,
    )
    .unwrap();
    assert_eq!(check.status, Status::Open, "{check:?}");
    assert!(check.parts.iter().all(|p| p.status != Status::Failed));
}

/// Glivenko's translation of a first-order sequent is refused with the
/// reason its theorem fails there.
#[test]
fn glivenko_s_translation_refuses_a_first_order_sequent() {
    let sequent = ordinary(
        "|- forall x. (p(x) \\/ ~p(x))",
        crate::ordinary::Logic::Classical,
    );
    let refused = run(
        "ordinary.translate",
        &[&sequent],
        r#"{"translation": "glivenko"}"#,
        &settings(),
        |_| false,
    )
    .unwrap_err();
    assert_eq!(refused.code(), "glivenko_first_order");
    assert!(refused.to_string().contains("chain_excluded_middle"));
}

#[test]
fn an_interpolant_is_held_to_its_input_and_its_halves() {
    let sequent = ordinary("a /\\ b |- b \\/ c", crate::ordinary::Logic::Classical);
    let interpolant = ran("ordinary.interpolate", &[&sequent], "");
    let mut value: serde_json::Value = serde_json::from_str(&interpolant).unwrap();
    let limits = settings().limits;
    // A half without the interpolant in its place is refused, not read.
    let mut broken = value.clone();
    broken["interpolant"]["first"] =
        serde_json::from_str(&ordinary("a |-", crate::ordinary::Logic::Classical)).unwrap();
    let broken = broken.to_string();
    assert_eq!(
        read("ordinary.interpolant", &broken, &limits)
            .unwrap_err()
            .code(),
        "json"
    );
    // An interpolant of another sequent fails its check.
    let other = ordinary("a /\\ c |- b \\/ c", crate::ordinary::Logic::Classical);
    let check = checked("ordinary.interpolate", &[&other], "", &interpolant);
    assert_eq!(check.status, Status::Failed);
    assert_eq!(check.parts[0].name, "halves");
    value["interpolant"]["formula"] = serde_json::json!("c");
    let renamed = value.to_string();
    assert!(read("ordinary.interpolant", &renamed, &limits).is_err());
}

#[test]
fn a_value_of_another_kind_is_refused() {
    let sequent = ordinary("a /\\ b |- b", crate::ordinary::Logic::Intuitionistic);
    let decided = ran("ordinary.decide", &[&sequent], r#"{"method": "ljt"}"#);
    let derivation = ran("ordinary.derivation", &[&decided], "");
    let refused = |operation: &str, input: &str| {
        run(operation, &[input], "", &settings(), |_| false)
            .unwrap_err()
            .code()
    };
    assert_eq!(refused("ordinary.derivation", &derivation), "json");
    assert_eq!(refused("ordinary.term", &decided), "json");
    let limits = settings().limits;
    assert!(read("ordinary.outcome", r#"{"verdict": "banana"}"#, &limits).is_err());
    // A linear outcome is no linear proof here.
    let linear = super::linear::write_statement(
        &"|- ~a, a".parse().unwrap(),
        crate::linear::Mode::CLASSICAL,
    )
    .unwrap();
    let outcome = ran("linear.prove", &[&linear], "");
    assert_eq!(refused("linear.net", &outcome), "json");
    assert!(read("linear.proof", &outcome, &limits).is_err());
}

#[test]
fn a_modal_sequent_is_decided_and_its_evidence_checked() {
    let limits = settings().limits;
    let valid = super::statement("modal", "[]p |- [][]p", "S4", &limits).unwrap();
    assert_eq!(
        summary("modal.sequent", &valid, &limits).unwrap(),
        "□p ⊢ □□p"
    );
    let outcome = ran("modal.decide", &[&valid], "");
    assert_eq!(verdict_of(&outcome).unwrap(), Verdict::Valid);
    assert_eq!(
        checked("modal.decide", &[&valid], "", &outcome).status,
        Status::Held
    );
    let derivation = ran("modal.derivation", &[&outcome], "");
    assert_eq!(
        checked("modal.derivation", &[&outcome], "", &derivation).status,
        Status::Held
    );
    // Not valid in K: the countermodel is checked, and has no derivation.
    let refuted = super::statement("modal", "[]p |- [][]p", "K", &limits).unwrap();
    let outcome = ran("modal.decide", &[&refuted], "");
    assert_eq!(
        summary("modal.outcome", &outcome, &limits).unwrap(),
        "Not valid"
    );
    assert_eq!(
        checked("modal.decide", &[&refuted], "", &outcome).status,
        Status::Held
    );
    let error = run("modal.derivation", &[&outcome], "", &settings(), |_| false).unwrap_err();
    assert_eq!(error.code(), "no_derivation");
    // A forged countermodel fails its check.
    let forged = outcome.replacen(r#""arrows":[1]"#, r#""arrows":[]"#, 1);
    assert_ne!(forged, outcome);
    assert_eq!(
        checked("modal.decide", &[&refuted], "", &forged).status,
        Status::Failed
    );
}

#[cfg(feature = "svg")]
#[test]
fn a_modal_outcome_is_drawn() {
    let limits = settings().limits;
    for system in ["S4", "K"] {
        let statement = super::statement("modal", "[]p |- [][]p", system, &limits).unwrap();
        let outcome = ran("modal.decide", &[&statement], "");
        let drawing = ran("modal.draw", &[&outcome], "");
        assert!(drawing.contains("<svg"), "{system}: {drawing}");
    }
}

#[test]
fn a_modal_model_is_taken_from_an_outcome_and_evaluated() {
    let limits = settings().limits;
    let k = super::statement("modal", "[]p |- [][]p", "K", &limits).unwrap();
    let outcome = ran("modal.decide", &[&k], "");
    let model = ran("modal.countermodel", &[&outcome], "");
    assert_eq!(
        summary("modal.model", &model, &limits).unwrap(),
        "A model of 3 worlds"
    );
    let evaluation = ran("modal.evaluate", &[&k, &model], "");
    assert_eq!(
        summary("modal.evaluation", &evaluation, &limits).unwrap(),
        "Fails at world 0"
    );
    assert!(
        evaluation.contains(r#""formula":"□□p","true":false,"witness":1"#),
        "{evaluation}"
    );
    // At world 2, which sees nothing, every □ holds.
    let at = ran("modal.evaluate", &[&k, &model], r#"{"world": 2}"#);
    assert_eq!(summary("modal.evaluation", &at, &limits).unwrap(), "Holds");
    // The model is no model of T: no world sees itself.
    let t = super::statement("modal", "[]p |- [][]p", "T", &limits).unwrap();
    let frame = ran("modal.evaluate", &[&t, &model], "");
    assert_eq!(
        summary("modal.evaluation", &frame, &limits).unwrap(),
        "No model of T: not reflexive"
    );
    // A valid outcome holds no countermodel.
    let s4 = super::statement("modal", "[]p |- [][]p", "S4", &limits).unwrap();
    let valid = ran("modal.decide", &[&s4], "");
    let error = run("modal.countermodel", &[&valid], "", &settings(), |_| false).unwrap_err();
    assert_eq!(error.code(), "invalid_option");
    // The calculus is the system's rule file, which reads back.
    let calculus = ran("modal.calculus", &[&s4], "");
    assert_eq!(
        summary("rules.calculus", &calculus, &limits).unwrap(),
        "G3S4"
    );
}

#[cfg(feature = "latex")]
#[test]
fn a_modal_derivation_is_exported() {
    let limits = settings().limits;
    let s4 = super::statement("modal", "[]p |- [][]p", "S4", &limits).unwrap();
    let outcome = ran("modal.decide", &[&s4], "");
    let text = ran("modal.export", &[&outcome], "");
    assert!(text.contains(r"\\Box"), "{text}");
}

#[test]
fn a_term_is_checked_and_a_type_s_terms_found_and_counted() {
    let limits = settings().limits;
    let term = super::statement("lambda", r"\f:a -> b. \x:a. f x", "simple", &limits).unwrap();
    let typed = ran("lambda.check", &[&term], "");
    assert_eq!(
        summary("lambda.typing", &typed, &limits).unwrap(),
        "λf. λx. f x"
    );
    assert_eq!(
        checked("lambda.check", &[&term], "", &typed).status,
        Status::Held
    );
    let other = super::statement("lambda", r"\x:a. x", "simple", &limits).unwrap();
    assert_eq!(
        checked("lambda.check", &[&other], "", &typed).status,
        Status::Failed
    );
    let intuitionistic = crate::ordinary::Logic::Intuitionistic;
    let ty = ordinary("|- a /\\ b -> b /\\ a", intuitionistic);
    let found = ran("lambda.inhabit", &[&ty], "");
    assert_eq!(
        summary("lambda.inhabitation", &found, &limits).unwrap(),
        "Has a term"
    );
    assert_eq!(
        checked("lambda.inhabit", &[&ty], "", &found).status,
        Status::Held
    );
    // A term of another type fails the check.
    let swapped = ordinary("|- a /\\ b -> a /\\ b", intuitionistic);
    let wrong = ran("lambda.inhabit", &[&swapped], "");
    assert_eq!(
        checked("lambda.inhabit", &[&ty], "", &wrong).status,
        Status::Failed
    );
    let pick = ordinary("|- a -> a -> a", intuitionistic);
    let listed = ran("lambda.enumerate", &[&pick], r#"{"max_terms": 1}"#);
    assert_eq!(
        summary("lambda.inhabitants", &listed, &limits).unwrap(),
        "Exactly 2 terms"
    );
    assert_eq!(
        checked("lambda.enumerate", &[&pick], "", &listed).status,
        Status::Held
    );
    // A checked term's typing derivation is drawn and exported.
    #[cfg(feature = "svg")]
    {
        let drawing = ran("lambda.draw", &[&typed], "");
        assert!(drawing.contains("by →I from"), "{drawing}");
    }
    #[cfg(feature = "latex")]
    {
        let exported = ran("lambda.export", &[&typed], "");
        assert!(
            exported.contains(r"\\infer2[$\\to\\mathrm{E}$]"),
            "{exported}"
        );
    }
    let classical = ordinary("|- a -> a", crate::ordinary::Logic::Classical);
    assert!(matches!(
        run("lambda.enumerate", &[&classical], "", &settings(), |_| {
            false
        }),
        Err(Error::InvalidOption { key: "logic", .. })
    ));
}

#[test]
fn an_untyped_term_is_reduced_and_typed_at_its_principal_type() {
    let limits = settings().limits;
    let term = super::statement("lambda", r"(\x. \y. x) (\z. z)", "untyped", &limits).unwrap();
    let reduced = ran("lambda.reduce", &[&term], "");
    assert_eq!(
        summary("lambda.reduction", &reduced, &limits).unwrap(),
        "Normal form after 1 step"
    );
    assert_eq!(
        checked("lambda.reduce", &[&term], "", &reduced).status,
        Status::Held
    );
    let unfinished = ran("lambda.reduce", &[&term], r#"{"steps": 0}"#);
    assert_eq!(
        checked("lambda.reduce", &[&term], "", &unfinished).status,
        Status::Failed
    );
    let typed = ran("lambda.infer", &[&term], "");
    assert_eq!(
        checked("lambda.infer", &[&term], "", &typed).status,
        Status::Held
    );
    // The typed term is a term like any, which the check takes.
    assert_eq!(
        checked(
            "lambda.check",
            &[&typed],
            "",
            &ran("lambda.check", &[&typed], "")
        )
        .status,
        Status::Held
    );
}

/// A first-order statement is decided through the embedding, starts a
/// session and its derivation is exported, drawn and summarized; LJT and
/// the certificate, which take propositional sequents, refuse it by name.
#[test]
fn a_first_order_statement_is_proved_step_by_step_and_rendered() {
    use crate::ordinary::{Interactive, Logic, Rule, Side, Step};
    let text = "forall x. p(x) |- p(c)";
    let statement = ordinary(text, Logic::Classical);
    #[cfg(feature = "rocq")]
    let refused = |operation: &str, inputs: &[&str]| {
        run(operation, inputs, "", &settings(), |_| false)
            .unwrap_err()
            .code()
    };
    let decided = ran("ordinary.decide", &[&statement], "");
    assert!(
        decided.contains(r#""version":25"#) && decided.contains(r#""verdict":"valid""#),
        "{decided}"
    );
    let ljt = run(
        "ordinary.decide",
        &[&statement],
        r#"{"method": "ljt"}"#,
        &settings(),
        |_| false,
    );
    assert_eq!(ljt.unwrap_err().code(), "first_order");
    let session = ran("ordinary.start", &[&statement], "");
    assert!(session.contains(r#""version":25"#), "{session}");
    let mut state = Interactive::new(&text.parse().unwrap(), Logic::Classical).unwrap();
    let goal = state.goals().next().unwrap();
    let witness = state.witness(goal, "c").unwrap();
    let step = Step::new(Side::Left, 0, Rule::ForallLeft).with_witness(witness);
    let [goal] = state.apply(goal, &step).unwrap()[..] else {
        panic!()
    };
    state
        .apply(goal, &Step::new(Side::Right, 0, Rule::Axiom))
        .unwrap();
    let saved = serde_json::to_string(&state).unwrap();
    let derivation = ran("ordinary.derivation", &[&saved], "");
    let limits = settings().limits;
    read("ordinary.derivation", &derivation, &limits).unwrap();
    #[cfg(feature = "latex")]
    assert!(ran("ordinary.export", &[&derivation], r#"{"format": "latex"}"#).contains(r"\forall"));
    #[cfg(feature = "svg")]
    assert!(ran("ordinary.draw", &[&derivation], "").contains("∀L"));
    #[cfg(feature = "rocq")]
    assert_eq!(
        refused("ordinary.certificate", &[&derivation]),
        "first_order"
    );
}

#[test]
fn a_drawing_past_the_derivation_bound_is_refused() {
    let sequent = ordinary(
        "a -> b, b -> c |- a -> c",
        crate::ordinary::Logic::Intuitionistic,
    );
    // The decision builds its derivation whatever the bound; the drawing
    // is held to it.
    let mut small = settings();
    small.limits = small.limits.with_derivation_bytes(Some(100));
    let ran_small = |operation: &str, input: &str| run(operation, &[input], "", &small, |_| false);
    let decided = ran_small("ordinary.decide", &sequent).unwrap().value;
    let derivation = ran_small("ordinary.derivation", &decided).unwrap().value;
    assert!(ran("ordinary.export", &[&derivation], "").contains("→R"));
    let error = ran_small("ordinary.export", &derivation).unwrap_err();
    assert_eq!(error.kind(), crate::ErrorKind::Limit);
}

/// Every outcome a decision writes reads back, a bound in the read-back
/// included: through the embedding a proved image beside an unknown
/// verdict was refused by the outcome's reader.
#[test]
fn every_decided_outcome_reads_back() {
    let chain = format!("{}a", "~".repeat(100));
    let sequent = ordinary(
        &format!("{chain} |- {chain}"),
        crate::ordinary::Logic::Classical,
    );
    for work in [5, 20, 50, 90, 150] {
        let mut bounded = settings();
        bounded.limits = bounded.limits.with_work(Some(work));
        let run = run(
            "ordinary.decide",
            &[&sequent],
            r#"{"method": "embedding"}"#,
            &bounded,
            |_| false,
        );
        if let Ok(produced) = run {
            read("ordinary.outcome", &produced.value, &bounded.limits).unwrap();
        }
    }
}
