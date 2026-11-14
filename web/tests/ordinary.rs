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

/// `about` lists both suites, each with the prefix of its calls, its term
/// call and the parameter its `start` takes, with every value and the
/// default, and the wire level, 10 since an outcome may hold a clausal
/// proof.
#[wasm_bindgen_test(unsupported = test)]
fn about_lists_the_suites() {
    let about = value(&calls::about());
    assert_eq!(about["level"], 10);
    let suites = about["suites"].as_array().unwrap();
    let names: Vec<&Value> = suites.iter().map(|s| &s["suite"]).collect();
    assert_eq!(names, [&json!("linear"), &json!("ordinary")]);
    let terms: Vec<&Value> = suites.iter().map(|s| &s["term"]).collect();
    assert_eq!(terms, [&json!("term"), &json!("ordinary_term")]);
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
    assert_eq!(outcome["version"], 8);
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
