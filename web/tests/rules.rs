// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of a tab that builds a derivation in a calculus given as
//! data, as the page makes them, under WebAssembly in Node and as
//! ordinary tests elsewhere.

use deduxit_web::rules::{self, DeriveForm, StepForm};
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

/// A step's JSON: the rule at these members, with these values given.
fn step(rule: &str, at: &[u32], given: &[(&str, &str)]) -> String {
    serde_json::to_string(&StepForm {
        rule: rule.to_owned(),
        at: at.to_vec(),
        given: given
            .iter()
            .map(|&(name, text)| (name.to_owned(), text.to_owned()))
            .collect(),
    })
    .unwrap()
}

/// A forward step's JSON: the rule from these lines.
fn derive(rule: &str, lines: &[u32]) -> String {
    serde_json::to_string(&DeriveForm {
        rule: rule.to_owned(),
        lines: lines.to_vec(),
        given: std::collections::BTreeMap::new(),
    })
    .unwrap()
}

/// Applies the first step `rules` offers of `rule` to `goal` that needs
/// nothing given, and returns the answer.
fn take(state: &str, goal: u32, rule: &str) -> String {
    let offered = value(&rules::rules(state, goal, "").unwrap());
    let found = offered
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["rule"] == rule && o["needs"] == json!([]))
        .unwrap_or_else(|| panic!("{rule} is offered for goal {goal}: {offered}"));
    let at: Vec<u32> = serde_json::from_value(found["at"].clone()).unwrap();
    rules::apply(state, goal, &step(rule, &at, &[]), "").unwrap()
}

/// The open goals of a call's answer.
fn open_goals(answer: &str) -> Vec<u32> {
    value(answer)["view"]["goals"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| u32::try_from(g["goal"].as_u64().unwrap()).unwrap())
        .collect()
}

/// `about` lists the suite with its prefix, the built-in calculi it
/// starts from and no lambda-term; each built-in rule file reads, and
/// an unknown name is refused.
#[wasm_bindgen_test(unsupported = test)]
fn the_built_in_calculi_read() {
    let about = value(&calls::about());
    let suite = &about["suites"][2];
    assert_eq!(suite["calls"], "rules_");
    assert_eq!(
        suite["parameters"][0],
        json!({"name": "calculus", "values": rules::NAMES, "default": "LJ"})
    );
    for name in rules::NAMES {
        let file = rules::calculus(name).unwrap();
        let described = value(&rules::read(&file, "").unwrap());
        assert_eq!(described, value(&rules::read(name, "").unwrap()));
        assert_ne!(described["rules"], json!([]));
    }
    let lj = value(&rules::read("LJ", "").unwrap());
    assert_eq!(
        lj["zones"],
        json!([{"name": "left", "mark": null}, {"name": "right", "mark": "⊢"}])
    );
    let unknown = failure(&rules::calculus("NQ").unwrap_err());
    assert_eq!(unknown["code"], "unknown_name");
    let broken = failure(&rules::read(r#"{"version": 15, "name": "X"}"#, "").unwrap_err());
    assert_eq!(broken["kind"], "malformed");
}

/// A derivation of LJ built by steps: the rules a goal takes, a step the
/// calculus refuses, an undo, the drawing with an id on every inference
/// the view names, and the finished derivation's verdict.
#[wasm_bindgen_test(unsupported = test)]
fn a_derivation_in_lj_by_steps_is_checked() {
    let started = rules::start("LJ", "a ∧ b ⊢ b ∧ a", "", "").unwrap();
    let view = &value(&started)["view"];
    assert_eq!(view["calculus"]["name"], "LJ");
    assert_eq!(view["goals"][0]["sequent"], "a ∧ b ⊢ b ∧ a");
    assert_eq!(view["goals"][0]["zones"], json!([["a ∧ b"], ["b ∧ a"]]));
    let mut state = session(&started);

    // ∧R acts on the right's conjunction, member 1, never on the left's.
    let refused = rules::apply(&state, 0, &step("∧R", &[0], &[]), "").unwrap_err();
    assert_eq!(failure(&refused)["code"], "step_refused");
    let answer = take(&state, 0, "∧R");
    assert_eq!(value(&answer)["opened"], json!([1, 2]));
    state = session(&answer);
    let undone = rules::undo(&state, "").unwrap();
    assert_eq!(value(&undone)["reopened"], 0);
    assert_eq!(open_goals(&undone), [0]);
    state = session(&take(&session(&undone), 0, "∧R"));

    // A cut needs what the goal leaves open given, named as its rule
    // names it: how the context splits.
    let needs = failure(&rules::apply(&state, 1, &step("cut", &[], &[]), "").unwrap_err());
    assert_eq!(needs["code"], "step_needs");
    assert_eq!(needs["details"]["needs"], json!(["Γ′"]), "{needs}");

    let unfinished = failure(&rules::check(&state, "").unwrap_err());
    assert_eq!(unfinished["code"], "open_goals");
    for (goal, kept) in [(1, "∧L₂"), (2, "∧L₁")] {
        state = session(&take(&state, goal, kept));
    }
    let leaves = open_goals(&format!(
        r#"{{"view": {}}}"#,
        rules::view(&state, "").unwrap()
    ));
    for goal in leaves {
        state = session(&take(&state, goal, "ax"));
    }
    let view = value(&rules::view(&state, "").unwrap());
    assert_eq!(view["finished"], true);
    assert_eq!(view["steps"], 5);

    let drawing = rules::draw(&state, "").unwrap();
    assert!(
        drawing.contains("<title>a ∧ b ⊢ b ∧ a</title>"),
        "{drawing}"
    );
    assert!(drawing.contains("by ∧R"), "{drawing}");
    let inferences = view["inferences"].as_array().unwrap();
    assert_eq!(inferences.len(), 5);
    for n in 0..inferences.len() {
        assert!(drawing.contains(&format!("id=\"i{n}\"")), "i{n}");
    }
    // The root, drawn last, is the session's first node.
    assert_eq!(inferences.last().unwrap(), 0);
    for format in ["latex", "typst"] {
        assert_ne!(rules::export(&state, format, "").unwrap(), "");
    }

    let checked = value(&rules::check(&state, "").unwrap());
    assert_eq!(checked["verdict"], "valid in the calculus given");
    assert_eq!(checked["derivation"]["calculus"], "LJ");
}

/// A goal closed by an assumption, a session saved and read again, and a
/// saved session whose step was changed refused when it is read.
#[wasm_bindgen_test(unsupported = test)]
fn assumptions_close_goals_and_saved_sessions_are_checked() {
    let started = rules::start("LJ", "a ⊢ b", r#"["a ⊢ b"]"#, "").unwrap();
    let view = &value(&started)["view"];
    assert_eq!(view["goals"][0]["hypotheses"], json!(["a ⊢ b"]));
    assert_eq!(
        (&view["sequent"], &view["assumptions"]),
        (&json!("a ⊢ b"), &json!(["a ⊢ b"]))
    );
    let state = session(&started);
    let closed = rules::assume(&state, 0, "").unwrap();
    assert_eq!(value(&closed)["view"]["finished"], true);
    let saved = session(&closed);
    assert!(rules::check(&saved, "").is_ok());

    let other = rules::start("LJ", "a ⊢ c", "", "").unwrap();
    let refused = failure(&rules::assume(&session(&other), 0, "").unwrap_err());
    assert_eq!(refused["code"], "step_refused");

    // A step whose rule was renamed in the saved document is refused.
    let built = session(&take(
        &session(&rules::start("LJ", "a ⊢ a", "", "").unwrap()),
        0,
        "ax",
    ));
    let mut forged = value(&built);
    let nodes = forged["nodes"].as_array_mut().unwrap();
    nodes[0]["rule"] = json!("WL");
    assert!(rules::view(&forged.to_string(), "").is_err());
    assert!(rules::view(&built, "").is_ok());
}

/// A calculus of the visitor's own, written as a rule file: a session in
/// it, and a rule file the library refuses.
#[wasm_bindgen_test(unsupported = test)]
fn a_calculus_of_ones_own_is_used() {
    let file = json!({
        "version": 15,
        "name": "Mini",
        "connectives": [{"name": "&", "arity": 2, "fixity": "infix"}],
        "sequent": [{"name": "left", "discipline": "multiset"}, {"name": "right", "mark": "⊢", "discipline": "multiset"}],
        "variables": {"formula": ["A", "B"]},
        "contexts": ["Γ"],
        "rules": [
            {"name": "id", "conclusion": {"left": ["Γ", "A"], "right": ["A"]}},
            {"name": "&R", "premises": [{"left": ["Γ"], "right": ["A"]}, {"left": ["Γ"], "right": ["B"]}],
             "conclusion": {"left": ["Γ"], "right": [["&", "A", "B"]]}}
        ]
    })
    .to_string();
    let described = value(&rules::read(&file, "").unwrap());
    assert_eq!(described["rules"], json!(["id", "&R"]));
    let mut state = session(&rules::start(&file, "p, q ⊢ q & p", "", "").unwrap());
    state = session(&take(&state, 0, "&R"));
    for goal in [1, 2] {
        state = session(&take(&state, goal, "id"));
    }
    let checked = value(&rules::check(&state, "").unwrap());
    assert_eq!(checked["calculus"], "Mini");

    let unknown = json!({
        "version": 15, "name": "Bad", "connectives": [],
        "sequent": [{"name": "s", "discipline": "set"}],
        "variables": {"formula": ["A"]},
        "rules": [{"name": "r", "conclusion": {"s": ["X"]}}]
    });
    let refused = failure(&rules::read(&unknown.to_string(), "").unwrap_err());
    assert_eq!(refused["code"], "invalid_calculus");
}

/// Natural deduction built both ways: a subproof opened backwards, lines
/// derived forwards from it, an undo of a forward step, a line cited by
/// two goals, the tree and the Fitch proof of the result.
#[wasm_bindgen_test(unsupported = test)]
fn natural_deduction_is_built_forwards_and_backwards() {
    let started = rules::start("NJ", "a ∧ b → (b ∧ a) ∧ (b ∧ a)", "", "").unwrap();
    let opened = take(&session(&started), 0, "→I");
    assert_eq!(value(&opened)["opened"], json!([1]));
    let goal = &value(&opened)["view"]["goals"][0];
    assert_eq!(
        goal["lines"],
        json!([{"line": 1, "fitch": 1, "sequent": "a ∧ b"}])
    );
    let mut state = session(&opened);
    for (rule, lines, number) in [
        ("∧E₂", vec![1], 2),
        ("∧E₁", vec![1], 3),
        ("∧I", vec![2, 3], 4),
    ] {
        let answer = rules::derive(&state, 1, &derive(rule, &lines), "").unwrap();
        assert_eq!(value(&answer)["line"], number);
        state = session(&answer);
    }
    let undone = rules::undo(&state, "").unwrap();
    let said = value(&undone);
    assert_eq!(
        (&said["reopened"], &said["removed"]),
        (&json!(null), &json!(4))
    );
    let again = rules::derive(&session(&undone), 1, &derive("∧I", &[2, 3]), "").unwrap();
    assert_eq!(value(&again)["line"], 4);
    state = session(&again);

    // A rule that discharges applies backwards only; a line must exist.
    let discharging = failure(&rules::derive(&state, 1, &derive("→I", &[4]), "").unwrap_err());
    assert_eq!(discharging["code"], "step_refused");
    let missing = failure(&rules::derive(&state, 1, &derive("∧I", &[4, 9]), "").unwrap_err());
    assert!(
        missing["message"].as_str().unwrap().contains("no line 9"),
        "{missing}"
    );

    let split = take(&state, 1, "∧I");
    let goals = open_goals(&split);
    state = session(&split);
    for goal in goals {
        state = session(&rules::cite(&state, goal, 4, "").unwrap());
    }
    let view = value(&rules::view(&state, "").unwrap());
    assert_eq!(view["finished"], true);
    // Line 4 is drawn under each parent: two inferences, one node.
    let inferences = view["inferences"].as_array().unwrap();
    let tree = rules::draw(&state, "").unwrap();
    assert_eq!(inferences.len(), 12);
    assert!(
        tree.contains("id=\"i11\"") && !tree.contains("id=\"i12\""),
        "{tree}"
    );

    let fitch = r#"{"view": {"layout": "fitch"}}"#;
    let latex = rules::export(&state, "latex", fitch).unwrap();
    assert!(latex.contains(r"\hypo{1}{a \land b}"), "{latex}");
    assert!(latex.contains(r"\by{$\land\mathrm{I}$}{4, 4}"), "{latex}");
    let svg = rules::draw(&state, fitch).unwrap();
    assert!(
        svg.contains("id=\"l6\"") && !svg.contains("id=\"l7\""),
        "{svg}"
    );
    let checked = value(&rules::check(&state, "").unwrap());
    assert_eq!(checked["verdict"], "valid in the calculus given");
}

/// A goal closed by its assumption has a Fitch proof, and two equal
/// assumptions are its one line 1.
#[wasm_bindgen_test(unsupported = test)]
fn equal_assumptions_are_one_fitch_line() {
    let fitch = r#"{"view": {"layout": "fitch"}}"#;
    let started = rules::start("NJ", "a", r#"["a"]"#, "").unwrap();
    let closed = session(&rules::cite(&session(&started), 0, 1, "").unwrap());
    let svg = rules::draw(&closed, fitch).unwrap();
    assert!(
        svg.contains("id=\"l1\"") && !svg.contains("id=\"l2\""),
        "{svg}"
    );
    let twice = value(&rules::start("NJ", "a ∧ a", r#"["a", "a"]"#, "").unwrap());
    let numbers: Vec<&Value> = twice["view"]["goals"][0]["lines"]
        .as_array()
        .unwrap()
        .iter()
        .map(|line| &line["fitch"])
        .collect();
    assert_eq!(numbers, [&json!(1), &json!(1)]);
}

/// A line of a closed subproof is out of reach of a goal outside it, and
/// citing it is refused with the reason.
#[wasm_bindgen_test(unsupported = test)]
fn a_line_out_of_scope_is_refused() {
    let mut state = session(&take(
        &session(&rules::start("NJ", "(a → a) ∧ (b → b)", "", "").unwrap()),
        0,
        "∧I",
    ));
    for goal in [1, 2] {
        state = session(&take(&state, goal, "→I"));
    }
    let view = value(&rules::view(&state, "").unwrap());
    // The second subproof's hypothesis is the session's line 2 and the
    // Fitch proof's line 4, under the first subproof (lines 1 and 2) and
    // the line 3 that discharges it.
    let lines = |at: usize| view["goals"][at]["lines"].clone();
    assert_eq!(lines(0), json!([{"line": 1, "fitch": 1, "sequent": "a"}]));
    assert_eq!(lines(1), json!([{"line": 2, "fitch": 4, "sequent": "b"}]));
    let goal = u32::try_from(view["goals"][1]["goal"].as_u64().unwrap()).unwrap();
    let refused = failure(&rules::cite(&state, goal, 1, "").unwrap_err());
    assert_eq!(refused["code"], "step_refused");
    assert!(
        refused["message"].as_str().unwrap().contains("line 1"),
        "{refused}"
    );
}

/// Every rule of the calculus comes with its status on a goal: `→I`
/// applying, `∧I` refused with the fault as a field.
#[wasm_bindgen_test(unsupported = test)]
fn candidates_give_every_rule_its_status() {
    let started = rules::start("NJ", "a ∧ b → b", "", "").unwrap();
    let judged = value(&rules::candidates(&session(&started), 0, "").unwrap());
    let of = |rule: &str| {
        judged
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["rule"] == rule)
            .unwrap_or_else(|| panic!("{rule} in {judged}"))
            .clone()
    };
    assert_eq!(
        of("→I"),
        json!({"rule": "→I", "at": [], "status": "applies"})
    );
    let and = of("∧I");
    assert_eq!(and["status"], "refused");
    assert_eq!(and["refusal"]["code"], "step_refused");
    assert_eq!(and["refusal"]["details"]["fault"], "unmatched");
}
