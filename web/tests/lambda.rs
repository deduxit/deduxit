// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of a tab of lambda-terms, as the page makes them, under
//! WebAssembly in Node and as ordinary tests elsewhere.

use deduxit_web::lambda;
use serde_json::{Value, json};
use wasm_bindgen_test::wasm_bindgen_test;

/// Reads a call's JSON.
fn value(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// Returns the code of the error a call threw.
fn code(thrown: deduxit_web::Failure) -> Value {
    value(&thrown.0)["code"].clone()
}

#[wasm_bindgen_test(unsupported = test)]
fn a_term_is_checked_and_its_typing_derivation_drawn() {
    let term = lambda::read(r"\f:a -> b. \x:a. f x", "simple", "").unwrap();
    assert_eq!(value(&term)["text"], json!(r"\f:a -> b. \x:a. f x"));
    let typing = lambda::check(&term, "").unwrap();
    assert_eq!(
        value(&typing)["judgement"],
        json!("⊢ λf:(a → b). λx:a. f x : (a → b) → (a → b)")
    );
    let tree = lambda::draw(&typing, "").unwrap();
    assert!(tree.starts_with("<svg") && tree.contains("by →E"), "{tree}");
    let tex = lambda::export(&typing, "latex", "").unwrap();
    assert!(tex.contains(r"\to\mathrm{I}"), "{tex}");
    // A term whose types do not fit is refused where it is read, one
    // that uses a variable twice where it is checked in the linear
    // calculus, and text that is no term at its place.
    let thrown = lambda::read(r"\x:a. x x", "simple", "").unwrap_err();
    assert_eq!(code(thrown), json!("ill_typed"));
    let twice = lambda::read(r"\x:a. x * x", "linear", "").unwrap();
    assert_eq!(
        code(lambda::check(&twice, "").unwrap_err()),
        json!("ill_typed")
    );
    let thrown = lambda::read(r"\x. x", "simple", "").unwrap_err();
    let error = value(&thrown.0);
    assert_eq!(error["code"], json!("parse"));
    assert_eq!(error["details"]["span_utf16"]["start"], json!(2));
}

#[wasm_bindgen_test(unsupported = test)]
fn a_type_s_terms_are_found_and_counted() {
    let ty = lambda::read_type("a -> a -> a", "").unwrap();
    let found = lambda::inhabit(&ty, "").unwrap();
    assert_eq!(value(&found)["verdict"], json!("inhabited"));
    assert_eq!(
        value(&found)["term"]["judgement"],
        json!("⊢ λx. λy. y : a → (a → a)")
    );
    let listed = lambda::enumerate(&ty, r#"{"max_terms": 1}"#, "").unwrap();
    let listed = value(&listed);
    assert_eq!(listed["terms"].as_array().unwrap().len(), 1);
    assert_eq!(listed["terms"][0]["text"], json!("λx. λy. x"));
    assert_eq!(listed["count"], json!({"kind": "exactly", "count": 2}));
    let peirce = lambda::read_type("((a -> b) -> a) -> a", "").unwrap();
    let none = lambda::inhabit(&peirce, "").unwrap();
    assert_eq!(value(&none)["verdict"], json!("empty"));
}

#[wasm_bindgen_test(unsupported = test)]
fn an_untyped_term_is_reduced_step_by_step_and_typed() {
    let term = lambda::read(r"(\x. \y. x) (\z. z) w", "untyped", "").unwrap();
    // No step: the term as it is, its first redex marked.
    let marked = value(&lambda::reduce(&term, r#"{"steps": 0}"#, "").unwrap());
    let text = marked["term"]["text"].as_str().unwrap();
    assert_eq!(text, "(λx. λy. x) (λz. z) w");
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let (start, end) = (
        marked["next"]["start"].as_u64().unwrap() as usize,
        marked["next"]["end"].as_u64().unwrap() as usize,
    );
    assert_eq!(
        String::from_utf16(&utf16[start..end]).unwrap(),
        "(λx. λy. x) (λz. z)"
    );
    assert_eq!(marked["end"], json!("steps"));
    // To the end: the normal form, nothing left to mark.
    let done = value(&lambda::reduce(&term, "", "").unwrap());
    assert_eq!(done["end"], json!("normal_form"));
    assert_eq!(done["term"]["text"], json!("λz. z"));
    assert_eq!(done["next"], Value::Null);
    // The principal type, and a term without one.
    let typed = value(&lambda::infer(&term, "").unwrap());
    assert_eq!(
        typed["judgement"],
        json!("w : a ⊢ (λx:(b → b). λy:a. x) (λz:b. z) w : b → b")
    );
    let twice = lambda::read(r"\x. x x", "untyped", "").unwrap();
    assert_eq!(
        code(lambda::infer(&twice, "").unwrap_err()),
        json!("ill_typed")
    );
    let linear = lambda::read(r"\x. x x", "untyped_linear", "").unwrap();
    assert_eq!(
        code(lambda::check(&linear, "").unwrap_err()),
        json!("ill_typed")
    );
}
