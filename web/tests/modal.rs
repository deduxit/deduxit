// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of a tab of modal logic, as the page makes them, under
//! WebAssembly in Node and as ordinary tests elsewhere.

use deduxit_web::modal;
use serde_json::{Value, json};
use wasm_bindgen_test::wasm_bindgen_test;

/// Reads a call's JSON.
fn value(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// The statement of `[]p |- [][]p` in `system`.
fn four(system: &str) -> String {
    modal::read("[]p |- [][]p", system, "").unwrap()
}

#[wasm_bindgen_test(unsupported = test)]
fn a_statement_is_decided_and_drawn() {
    let statement = four("S4");
    assert_eq!(value(&statement)["system"], json!("S4"));
    assert_eq!(value(&statement)["text"], json!("[]p |- [][]p"));
    let valid = modal::decide(&statement, "").unwrap();
    assert_eq!(value(&valid)["verdict"], json!("valid"));
    let tree = modal::draw(&valid, "").unwrap();
    assert!(tree.starts_with("<svg") && tree.contains("Trans"), "{tree}");
    let tex = modal::export(&valid, "latex", "").unwrap();
    assert!(tex.contains(r"\Box"), "{tex}");
    let calculus = modal::calculus(&statement, "").unwrap();
    assert_eq!(value(&calculus)["name"], json!("G3S4"));
    // Not valid in K: the countermodel, its drawing, and none of a valid
    // outcome.
    let refuted = modal::decide(&four("K"), "").unwrap();
    assert_eq!(value(&refuted)["verdict"], json!("not_valid"));
    let model = modal::countermodel(&refuted, "").unwrap();
    assert_eq!(value(&model)["worlds"].as_array().unwrap().len(), 3);
    let drawing = modal::draw(&refuted, "").unwrap();
    assert!(drawing.contains("sees"), "{drawing}");
    let none = modal::countermodel(&valid, "").unwrap_err();
    assert_eq!(value(&none.0)["code"], json!("invalid_option"));
}

#[wasm_bindgen_test(unsupported = test)]
fn a_model_the_visitor_gives_is_checked_and_drawn() {
    let statement = four("K");
    let model = json!({"version": 13, "worlds": [
        {"atoms": [], "arrows": [1]},
        {"atoms": ["p"], "arrows": [2]},
        {"atoms": []},
    ]})
    .to_string();
    let every = value(&modal::evaluate(&statement, &model, None, "").unwrap());
    assert_eq!(every["holds"], json!(false));
    assert_eq!(every["failing"], json!(0));
    assert_eq!(
        every["values"][1],
        json!({"formula": "□□p", "true": false, "witness": 1})
    );
    let at = value(&modal::evaluate(&statement, &model, Some(2), "").unwrap());
    assert_eq!(at["holds"], json!(true));
    let drawing = modal::model_drawing(&statement, &model, 0, "").unwrap();
    assert!(
        drawing.contains("A countermodel of □p ⊢ □□p in K"),
        "{drawing}"
    );
    // At world 2 the sequent holds: no countermodel to draw.
    let holds = modal::model_drawing(&statement, &model, 2, "").unwrap_err();
    assert_eq!(value(&holds.0)["code"], json!("invalid_model"));
    // No world sees itself: no model of T, the condition named.
    let frame = value(&modal::evaluate(&four("T"), &model, None, "").unwrap());
    assert_eq!(
        frame["frame"],
        json!({"condition": "reflexive", "worlds": [0]})
    );
}
