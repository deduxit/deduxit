// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of an ordinary tab as the page makes them, under WebAssembly
//! in Node and as ordinary tests elsewhere.

use deduxit_web::ordinary::{self, SplitForm, StepForm};
use deduxit_web::{Failure, calls};
use serde_json::{Value, json};
use wasm_bindgen_test::wasm_bindgen_test;

/// Reads a call's JSON.
fn value(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// Reads a failure's JSON.
fn failure(failure: &Failure) -> Value {
    value(&failure.0)
}

/// The session of a call's answer, as the page keeps it.
fn session(answer: &str) -> String {
    value(answer)["session"].to_string()
}

/// A step's JSON: the rule on the formula at a position of a side, the
/// split's positions going to the first premise if given.
fn step(side: &str, position: usize, rule: &str, split: Option<(&[usize], &[usize])>) -> String {
    let side = serde_json::from_value(json!(side)).unwrap();
    let rule = serde_json::from_value(json!(rule)).unwrap();
    serde_json::to_string(&StepForm {
        side,
        position,
        rule,
        split: split.map(|(left, right)| SplitForm {
            left: left.to_vec(),
            right: right.to_vec(),
        }),
    })
    .unwrap()
}

/// The open goals of a call's answer.
fn open_goals(answer: &str) -> Vec<u32> {
    let view = value(answer);
    let view = if view["view"].is_null() {
        view
    } else {
        view["view"].clone()
    };
    view["goals"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| u32::try_from(g["goal"].as_u64().unwrap()).unwrap())
        .collect()
}

/// `about` lists the suites, each with the prefix of its calls, its term
/// call and the parameter its `start` takes, with every value and the
/// default, the ordinary suite's methods, and the wire level.
#[wasm_bindgen_test(unsupported = test)]
fn about_lists_the_suites() {
    let about = value(&calls::about());
    assert_eq!(about["level"], deduxit::wire::LEVEL);
    assert_eq!(about["suites"][1]["methods"][1]["call"], "ordinary_tableau");
    let suites = about["suites"].as_array().unwrap();
    let names: Vec<&Value> = suites.iter().map(|s| &s["suite"]).collect();
    assert_eq!(
        names,
        [
            &json!("linear"),
            &json!("ordinary"),
            &json!("rules"),
            &json!("modal")
        ]
    );
    let terms: Vec<&Value> = suites.iter().map(|s| &s["term"]).collect();
    assert_eq!(
        terms,
        [
            &json!("term"),
            &json!("ordinary_term"),
            &Value::Null,
            &Value::Null
        ]
    );
    assert_eq!(suites[3]["calls"], "modal_");
    assert_eq!(suites[3]["parameters"][0]["default"], "K");
    let ordinary = &suites[1];
    assert_eq!(ordinary["calls"], "ordinary_");
    assert_eq!(
        ordinary["parameters"],
        json!([{
            "name": "logic",
            "values": ["classical", "intuitionistic", "minimal"],
            "default": "intuitionistic",
        }])
    );
    assert_eq!(suites[0]["parameters"][0]["name"], "mode");
    assert_eq!(suites[0]["parameters"][0]["default"], "classical");
}

/// A proof of LJ by clicking: the rules a formula takes, the steps, an
/// undo, the finished derivation exported and certified, and the drawing
/// with an id on every inference the view names.
#[wasm_bindgen_test(unsupported = test)]
fn an_ordinary_proof_by_steps_is_exported() {
    let started = ordinary::start("|- a /\\ b -> b /\\ a", "intuitionistic", "").unwrap();
    let view = &value(&started)["view"];
    assert_eq!(
        (&view["logic"], &view["calculus"]),
        (&json!("intuitionistic"), &json!("LJ"))
    );
    assert_eq!(view["goals"][0]["right"][0], "(a ∧ b) → (b ∧ a)");
    let state = session(&started);
    let rules = value(&ordinary::rules(&state, 0, "right", 0, "").unwrap());
    assert_eq!(rules, json!([{"rule": "→R", "split": false}]));
    let answer = ordinary::apply(&state, 0, &step("right", 0, "→R", None), "").unwrap();
    let mut state = session(&answer);
    let answer = ordinary::apply(&state, 1, &step("right", 0, "∧R", None), "").unwrap();
    assert_eq!(value(&answer)["opened"], json!([2, 3]));
    state = session(&answer);
    let undone = ordinary::undo(&state, "").unwrap();
    assert_eq!(value(&undone)["reopened"], 1);
    assert_eq!(open_goals(&undone), [1]);
    let mut answer =
        ordinary::apply(&session(&undone), 1, &step("right", 0, "∧R", None), "").unwrap();
    for (goal, rule) in [(2, "∧L₂"), (3, "∧L₁")] {
        state = session(&answer);
        answer = ordinary::apply(&state, goal, &step("left", 0, rule, None), "").unwrap();
        let opened = value(&answer)["opened"][0].as_u64().unwrap();
        state = session(&answer);
        answer = ordinary::apply(
            &state,
            u32::try_from(opened).unwrap(),
            &step("left", 0, "ax", None),
            "",
        )
        .unwrap();
    }
    let state = session(&answer);
    let view = value(&ordinary::view(&state, "").unwrap());
    assert_eq!(
        (&view["complete"], &view["steps"]),
        (&json!(true), &json!(6))
    );
    let text = ordinary::export(&state, "text", "").unwrap();
    assert!(
        text.contains("∧L₂") && text.ends_with("⊢ (a ∧ b) → (b ∧ a)"),
        "{text}"
    );
    for format in ["latex", "typst"] {
        assert_ne!(ordinary::export(&state, format, "").unwrap(), "");
    }
    let certificate = ordinary::export(&state, "rocq", "").unwrap();
    assert!(
        certificate.starts_with("Lemma certificate"),
        "{certificate}"
    );
    let drawing = ordinary::draw(&state, "").unwrap();
    let drawn = view["inferences"].as_array().unwrap().len();
    assert_eq!(drawn, 6);
    for n in 0..drawn {
        assert!(drawing.contains(&format!(r#"id="i{n}""#)), "no i{n}");
    }
}

/// The structural rules are a setting of the view. An axiom with other
/// formulas beside it is drawn as G3's axiom by default, and with its
/// weakening under `view.structural` `explicit`. The view's inferences
/// follow the setting.
#[wasm_bindgen_test(unsupported = test)]
fn the_structural_rules_are_a_setting() {
    let started = ordinary::start("a, b |- a", "classical", "").unwrap();
    let answer = ordinary::apply(&session(&started), 0, &step("left", 0, "ax", None), "").unwrap();
    let state = session(&answer);
    let admissible = ordinary::export(&state, "text", "").unwrap();
    assert!(!admissible.contains("WL"), "{admissible}");
    let explicit = r#"{"view": {"structural": "explicit"}}"#;
    let shown = ordinary::export(&state, "text", explicit).unwrap();
    assert!(shown.contains("WL"), "{shown}");
    let view = value(&ordinary::view(&state, explicit).unwrap());
    assert_eq!(view["inferences"].as_array().unwrap().len(), 2);
}

/// `close` grafts the derivation of a valid goal, and `hint` names first
/// steps that `apply` takes. A goal that is not valid stays open, and the
/// outcome says so. A classical goal is decided by the embedding, whose
/// evidence is a derivation, never by a clausal proof, which has none to
/// graft.
#[wasm_bindgen_test(unsupported = test)]
fn close_and_hint_decide_a_goal() {
    let peirce = "|- ((a -> b) -> a) -> a";
    let state = session(&ordinary::start(peirce, "intuitionistic", "").unwrap());
    let hint = value(&ordinary::hint(&state, 0, "").unwrap());
    assert_eq!(
        (&hint["outcome"]["verdict"], &hint["steps"]),
        (&json!("not_valid"), &json!([]))
    );
    let closed = ordinary::close(&state, 0, "").unwrap();
    assert_eq!(open_goals(&closed), [0]);
    let state = session(&ordinary::start(peirce, "classical", "").unwrap());
    let hint = value(&ordinary::hint(&state, 0, "").unwrap());
    assert_eq!(hint["steps"][0]["rule"], "→R");
    let stepped = ordinary::apply(&state, 0, &hint["steps"][0].to_string(), "").unwrap();
    let closed = ordinary::close(&session(&stepped), 1, "").unwrap();
    let closed = value(&closed);
    assert_eq!(
        (
            &closed["outcome"]["verdict"],
            &closed["refused"],
            &closed["view"]["complete"]
        ),
        (&json!("valid"), &Value::Null, &json!(true))
    );
    let outcome = &closed["outcome"];
    assert!(outcome["derivation"].is_object(), "{outcome}");
    assert!(outcome.get("clausal_proof").is_none(), "{outcome}");
    assert_eq!(outcome["version"], 12);
    assert_eq!(outcome["search"]["method"], "embedding");
    assert_eq!(outcome["search"]["image"]["verdict"], "proved");
}

/// Malformed input is refused with its reason, never with a panic:
///
/// - an unknown logic or side;
/// - a step that is no step's JSON or names no rule;
/// - a rule that does not fit;
/// - a goal that is not open;
/// - a linear session;
/// - a sequent of LJ with two formulas right of `⊢`;
/// - an unknown format.
#[wasm_bindgen_test(unsupported = test)]
fn malformed_input_is_refused_with_its_reason() {
    let code = |result: Result<String, Failure>| failure(&result.unwrap_err())["code"].clone();
    assert_eq!(
        code(ordinary::start("a |- a", "linear", "")),
        "unknown_name"
    );
    assert_eq!(
        code(ordinary::start("a |- b, c", "minimal", "")),
        "succedents"
    );
    assert_eq!(code(ordinary::start("a |- (b", "classical", "")), "parse");
    let state = session(&ordinary::start("a |- a /\\ a", "intuitionistic", "").unwrap());
    assert_eq!(code(ordinary::rules(&state, 0, "middle", 0, "")), "json");
    assert_eq!(code(ordinary::rules(&state, 0, "left", 5, "")), "step");
    assert_eq!(code(ordinary::rules(&state, 7, "left", 0, "")), "step");
    let bad = [
        r#"{"side": "left", "position": 0}"#,
        r#"{"side": "left", "position": 0, "rule": "∧R", "left": [0]}"#,
        r#"{"side": "left", "position": 0, "rule": "mix"}"#,
        "[1, 2]",
    ];
    for step in bad {
        assert_eq!(code(ordinary::apply(&state, 0, step, "")), "json", "{step}");
    }
    // Cut is a rule of LJ, but no step can take it.
    let cut = r#"{"side": "left", "position": 0, "rule": "cut"}"#;
    assert_eq!(code(ordinary::apply(&state, 0, cut, "")), "step");
    let refused = ordinary::apply(&state, 0, &step("left", 0, "∧R", None), "").unwrap_err();
    let refused = failure(&refused);
    assert_eq!(refused["code"], "step");
    assert!(
        refused["message"]
            .as_str()
            .unwrap()
            .contains("does not act on"),
        "{refused}"
    );
    let split = step("right", 0, "∧R", Some((&[3], &[])));
    assert_eq!(code(ordinary::apply(&state, 0, &split, "")), "step");
    let linear = session(&calls::start("A |- A", "classical", "").unwrap());
    assert!(ordinary::view(&linear, "").is_err());
    assert!(ordinary::view("{", "").is_err());
    assert_eq!(code(ordinary::export(&state, "png", "")), "unknown_name");
    assert_eq!(code(ordinary::export(&state, "rocq", "")), "open_goals");
}

/// A finished derivation of LJ reads back as a simply typed term; one of
/// LK and one with open goals are answered with the library's refusal.
#[wasm_bindgen_test(unsupported = test)]
fn a_finished_derivation_reads_back_as_a_lambda_term() {
    let finished = |sequent: &str, logic: &str| {
        let started = ordinary::start(sequent, logic, "").unwrap();
        let closed = ordinary::close(&session(&started), 0, "").unwrap();
        assert_eq!(value(&closed)["view"]["complete"], true, "{sequent}");
        session(&closed)
    };
    let composition = "|- (a -> b) -> (b -> c) -> a -> c";
    let term = value(&ordinary::term(&finished(composition, "intuitionistic"), "").unwrap());
    assert_eq!(term["calculus"], "simple");
    assert_eq!(term["text"], "λf. λg. λx. g (f x)");
    let classical = value(&ordinary::term(&finished(composition, "classical"), "").unwrap());
    assert_eq!(classical["error"]["code"], "no_term");
    let started = ordinary::start(composition, "minimal", "").unwrap();
    let open = value(&ordinary::term(&session(&started), "").unwrap());
    assert_eq!(open["error"]["code"], "no_term");
}

/// `model` checks a Kripke model the page gives against a sequent: the
/// excluded middle fails at the root of two worlds, with what the root
/// forces of each subformula, and holds at the world above; a model of
/// two worlds is no model of classical logic.
#[wasm_bindgen_test(unsupported = test)]
fn a_model_is_checked_against_a_sequent() {
    let model = r#"{"worlds":[{"atoms":[],"arrows":[1]},{"atoms":["a"]}]}"#;
    let checked = value(&ordinary::model(model, "a \\/ ~a", "intuitionistic", None, "").unwrap());
    assert_eq!(checked["holds"], false);
    assert_eq!(checked["failing"]["world"], 0);
    assert_eq!(
        checked["failing"]["forcing"][2],
        json!({"formula": "a ∨ ¬a", "forced": false})
    );
    let checked =
        value(&ordinary::model(model, "a \\/ ~a", "intuitionistic", Some(1), "").unwrap());
    assert_eq!(
        (&checked["holds"], &checked["failing"]),
        (&json!(true), &Value::Null)
    );
    let refused = ordinary::model(model, "a", "classical", None, "").unwrap_err();
    assert_eq!(failure(&refused)["code"], "invalid_model");
}

/// A goal found not valid comes with the SAT solver's countermodel in
/// `close`'s and `hint`'s outcome, which `model_drawing` draws and
/// `model_certificate` certifies; an outcome without one is refused with
/// `no_countermodel`.
#[wasm_bindgen_test(unsupported = test)]
fn a_countermodel_is_drawn_and_certified() {
    let state = session(&ordinary::start("|- a \\/ ~a", "intuitionistic", "").unwrap());
    let closed = value(&ordinary::close(&state, 0, "").unwrap());
    let outcome = &closed["outcome"];
    assert_eq!(outcome["verdict"], "not_valid");
    assert_eq!(
        outcome["countermodel"]["model"],
        json!({"worlds": [{"atoms": [], "arrows": [1]}, {"atoms": ["a"]}], "world": 0})
    );
    let hint = value(&ordinary::hint(&state, 0, "").unwrap());
    assert_eq!(hint["outcome"]["countermodel"], outcome["countermodel"]);
    let drawing = ordinary::model_drawing(&outcome.to_string(), "").unwrap();
    assert!(drawing.contains("<title>A countermodel of ⊢ a ∨ ¬a</title>"));
    assert!(drawing.contains("World 0, the model's world, forces nothing; world 1 is above it."));
    let settings = r#"{"styles": {"rocq": {"form": "standalone"}}}"#;
    let certificate = ordinary::model_certificate(&outcome.to_string(), settings).unwrap();
    assert!(certificate.starts_with("From Stdlib Require Import List String."));
    assert!(certificate.contains(
        r#"Lemma certificate : ~ lj (atom := string) intuitionistic [] (vee (var "a") (neg (var "a")))."#
    ));
    let valid = value(
        &ordinary::close(
            &session(&ordinary::start("a |- a", "intuitionistic", "").unwrap()),
            0,
            "",
        )
        .unwrap(),
    );
    let refused = ordinary::model_drawing(&valid["outcome"].to_string(), "").unwrap_err();
    assert_eq!(failure(&refused)["code"], "no_countermodel");
}

/// The truth table and the tableau answer the outcome of their method:
/// the rows and the falsifying ones, the tableau's nodes and how each
/// branch ends, the countermodel of a sequent that is not valid; a
/// table past the bound on atoms is refused by its code.
#[wasm_bindgen_test]
#[test]
fn a_sequent_is_decided_by_its_table_or_a_tableau() {
    let table = value(&ordinary::table("a -> b |- b -> a", "").unwrap());
    assert_eq!(table["version"], 14);
    assert_eq!(table["verdict"], "not_valid");
    assert_eq!(table["search"]["table"]["falsifying"], json!([2]));
    assert_eq!(
        table["countermodel"]["model"]["worlds"][0]["atoms"],
        json!(["b"])
    );
    let tableau = value(&ordinary::tableau("a /\\ b |- b /\\ a", "").unwrap());
    assert_eq!(tableau["verdict"], "valid");
    let nodes = tableau["search"]["tableau"]["nodes"].as_array().unwrap();
    assert_eq!(nodes[0]["expands"], json!([0, "∧L"]));
    assert_eq!(
        nodes[2]["end"],
        json!({"kind": "closed", "true_line": 3, "false_line": 4})
    );
    assert!(tableau["derivation"].is_object());
    let wide = "a, b, c, d, e, f, g, h, i, j, k |- a";
    let refused = ordinary::table(wide, "").unwrap_err();
    assert_eq!(failure(&refused)["code"], "too_many_atoms");
}

/// With `interactive.invertible` on, a session applies the invertible
/// rules of its logic by itself at the start and after each step, and the
/// view lists the steps they took; `ordinary_invert_all` applies them
/// whatever the settings say. LJ leaves `→L` to the visitor.
#[wasm_bindgen_test(unsupported = test)]
fn the_invertible_rules_apply_by_themselves() {
    let on = r#"{"interactive": {"invertible": true}}"#;
    let started = value(&ordinary::start("a /\\ b |- b /\\ a", "classical", on).unwrap());
    assert_eq!(started["inverted"], 4);
    assert_eq!(started["view"]["complete"], true);
    assert_eq!(started["view"]["invertible"], json!([0, 1, 2, 3]));
    // A split context is no invertible step.
    let split = ordinary::start("c, d |- a /\\ b", "classical", "").unwrap();
    let step =
        r#"{"side": "right", "position": 0, "rule": "∧R", "split": {"left": [0], "right": []}}"#;
    let split = value(&ordinary::apply(&session(&split), 0, step, "").unwrap());
    assert_eq!(split["view"]["invertible"], json!([]));
    let lj = value(&ordinary::start("c, a -> b, a |- b /\\ c", "intuitionistic", on).unwrap());
    assert_eq!(lj["view"]["goals"].as_array().unwrap().len(), 1);
    let off = ordinary::start("a /\\ b |- b /\\ a", "classical", "").unwrap();
    assert_eq!(value(&off)["inverted"], 0);
    let all = value(&ordinary::invert_all(&session(&off), "").unwrap());
    assert_eq!(all["inverted"], 4);
    assert_eq!(all["view"]["complete"], true);
}

/// A tableau written by hand is reviewed: a closed one, an open one with
/// its countermodel, a wrong step at its text line, the sequent given;
/// a text that is no tableau is refused as a parse error placed in it.
#[wasm_bindgen_test(unsupported = test)]
fn a_tableau_written_by_hand_is_reviewed() {
    let closed =
        "1 T p \\/ q\n2 F q \\/ p\n3 F q 2\n4 F p 2\n- 5 T p 1\n  × 4, 5\n- 6 T q 1\n  × 3, 6\n";
    let review = value(&ordinary::tableau_review(closed, "p \\/ q |- q \\/ p", "").unwrap());
    assert_eq!(review["version"], 14);
    assert_eq!(review["standing"], "closed");
    let open = "1 T a -> b\n2 F b -> a\n3 T b 2\n4 F a 2\n- 5 F a 1\n  open\n- 6 T b 1\n  open\n";
    let review = value(&ordinary::tableau_review(open, "", "").unwrap());
    assert_eq!(review["standing"], "open");
    assert_eq!(
        review["countermodel"]["model"]["worlds"][0]["atoms"],
        json!(["b"])
    );
    let wrong = "1 T p /\\ q\n2 F p\n3 T p 1\n4 T r 1\n";
    let review = value(&ordinary::tableau_review(wrong, "", "").unwrap());
    assert_eq!(review["standing"], "wrong");
    assert_eq!(
        (review["text_line"].clone(), review["line"].clone()),
        (json!(4), json!(4))
    );
    let other = value(&ordinary::tableau_review(closed, "p |- p", "").unwrap());
    assert_eq!(other["standing"], "wrong");
    let refused = ordinary::tableau_review("1 T p\n   2 F q\n", "", "").unwrap_err();
    let refused = failure(&refused);
    assert_eq!(refused["code"], "parse");
    assert_eq!(refused["details"]["line"], 2);
}

/// A negative translation answers both verdicts, which agree, and an
/// interpolant answers both halves decided; `about` names both calls,
/// and a target the translation does not reach and a partition that is
/// not one are refused by their codes.
#[wasm_bindgen_test(unsupported = test)]
fn a_sequent_is_translated_and_interpolated() {
    let compared = value(&ordinary::translate("a \\/ ~a", "", "").unwrap());
    assert_eq!(compared["version"], 16);
    assert_eq!(compared["agreement"], "valid");
    assert_eq!(compared["target"], "minimal");
    assert_eq!(compared["decided"]["verdict"], "valid");
    let glivenko = r#"{"translation": "glivenko"}"#;
    let compared = value(&ordinary::translate("a |- b", glivenko, "").unwrap());
    assert_eq!(compared["agreement"], "not_valid");
    assert_eq!(compared["target"], "intuitionistic");
    let refused = ordinary::translate("a", r#"{"translation": "kuroda", "target": "minimal"}"#, "");
    assert_eq!(failure(&refused.unwrap_err())["code"], "negative_target");

    let found = value(&ordinary::interpolate("a /\\ b |- b \\/ c", "classical", "", "").unwrap());
    assert_eq!(found["version"], 16);
    assert_eq!(found["interpolant"]["text"], "b");
    assert_eq!(found["interpolant"]["first"]["outcome"]["verdict"], "valid");
    let partition = r#"{"left": [], "right": [0]}"#;
    let refused = ordinary::interpolate("a |- a", "intuitionistic", partition, "");
    assert_eq!(failure(&refused.unwrap_err())["code"], "partition");

    let about = value(&calls::about());
    let transformations = &about["suites"][1]["transformations"];
    assert_eq!(transformations[0]["call"], "ordinary_translate");
    assert_eq!(transformations[0]["translations"][1]["target"], "minimal");
    assert_eq!(transformations[1]["call"], "ordinary_interpolate");
}

/// A normal form answers its steps and checks; a resolution its checked
/// refutation, by either method, or a model; a refutation by hand grows a
/// step at a time to the empty clause, refuses a step the clauses do not
/// allow and a document changed by hand; `about` names the calls.
#[wasm_bindgen_test(unsupported = test)]
fn a_sequent_is_normalized_and_resolved() {
    let cnf = value(&ordinary::normalize("a -> b /\\ c", "", "").unwrap());
    assert_eq!(cnf["version"], 21);
    assert_eq!(cnf["checked"], true);
    assert_eq!(cnf["steps"][0]["rule"], "implication");
    let encoded = ordinary::normalize("a", r#"{"form": "tseitin"}"#, "").unwrap();
    assert_eq!(value(&encoded)["promise"], "equisatisfiable");

    let resolved = value(&ordinary::resolve("a -> b, a |- b", "", "").unwrap());
    assert_eq!(resolved["version"], 22);
    assert_eq!(resolved["verdict"], "unsatisfiable");
    let dpll = value(&ordinary::resolve("a \\/ b |- a", r#"{"method": "dpll"}"#, "").unwrap());
    assert_eq!(dpll["verdict"], "satisfiable");
    assert!(dpll["trace"].as_array().is_some_and(|t| !t.is_empty()));

    // `¬a ∨ b`, `a`, `¬b`: resolve 0 with 1 on a, then that with 2 on b.
    let start = ordinary::clauses("a -> b, a |- b", "", "").unwrap();
    assert_eq!(
        value(&start)["clauses"],
        serde_json::json!([[-1, 2], [1], [-2]])
    );
    let refused = ordinary::resolve_step(&start, 0.0, 2.0, 1.0, "");
    assert_eq!(failure(&refused.unwrap_err())["code"], "step_refused");
    let one = value(&ordinary::resolve_step(&start, 0.0, 1.0, 1.0, "").unwrap());
    assert_eq!(one["added"], 3);
    assert_eq!(one["finished"], false);
    let one = one["refutation"].to_string();
    let two = value(&ordinary::resolve_step(&one, 3.0, 2.0, 2.0, "").unwrap());
    assert_eq!(two["finished"], true);
    let checked = ordinary::refutation_check(&two["refutation"].to_string(), "").unwrap();
    assert_eq!(value(&checked)["finished"], true);
    let changed = one.replace(r#""clause":[2]"#, r#""clause":[]"#);
    let refused = ordinary::refutation_check(&changed, "");
    assert_eq!(
        failure(&refused.unwrap_err())["code"],
        "invalid_clause_refutation"
    );

    let about = value(&calls::about());
    let transformations = &about["suites"][1]["transformations"];
    assert_eq!(transformations[2]["call"], "ordinary_normalize");
    assert_eq!(transformations[3]["call"], "ordinary_resolve");
}
