// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The protocol every suite's calls speak, as the page reads it from
//! `describe`, under WebAssembly in Node and as ordinary tests elsewhere.

use deduxit_web::protocol::{CALLS, SUITES, describe};
use deduxit_web::{calls, ordinary, rules};
use serde_json::{Value, json};
use wasm_bindgen_test::wasm_bindgen_test;

/// Reads a call's JSON.
fn value(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// The module's exports, as `lib.rs` declares them.
const EXPORTS: &str = include_str!("../src/lib.rs");

/// The JSON Schema of the wire forms, which the manual publishes.
const SCHEMA: &str = include_str!("../../docs/src/wire.schema.json");

#[wasm_bindgen_test(unsupported = test)]
fn the_schema_describes_every_kind_s_form() {
    let schema = value(SCHEMA);
    let catalogue = &value(&describe())["catalogue"];
    let kinds = catalogue["suites"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|suite| suite["kinds"].as_array().unwrap())
        .chain(catalogue["shared"].as_array().unwrap());
    for kind in kinds {
        let form = kind["form"].as_str().unwrap();
        assert!(
            schema["$defs"][form].is_object(),
            "{}'s form {form} is not in the schema",
            kind["name"]
        );
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn describe_names_every_suite_s_calls_and_each_exists() {
    let described = value(&describe());
    assert_eq!(described["calls"], json!(CALLS));
    let suites = described["suites"].as_array().unwrap();
    assert_eq!(suites.len(), SUITES.len());
    for suite in suites {
        let calls = suite["calls"].as_array().unwrap();
        let names: Vec<&str> = calls
            .iter()
            .map(|call| call["call"].as_str().unwrap())
            .collect();
        assert_eq!(names, CALLS, "{}", suite["suite"]);
        let exports = calls
            .iter()
            .filter_map(|call| call["export"].as_str())
            .chain(
                suite["extra"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|extra| extra["export"].as_str().unwrap()),
            );
        for export in exports {
            assert!(
                EXPORTS.contains(&format!("pub fn {export}("))
                    || EXPORTS.contains(&format!("js_name = \"{export}\")")),
                "{export} is no export of the module"
            );
        }
        assert!(
            suite["layouts"]
                .as_array()
                .unwrap()
                .contains(&json!("tree"))
        );
    }
    // The library's catalogue rides along, for the workspace.
    assert_eq!(
        described["catalogue"]["suites"][1]["suite"],
        json!("ordinary")
    );
    // A suite's own `methods` stay as `about` writes them.
    assert_eq!(
        suites[1]["methods"],
        value(&calls::about())["suites"][1]["methods"]
    );
    assert_eq!(suites[1]["builds"][1]["runs"], json!("rules"));
}

#[wasm_bindgen_test(unsupported = test)]
fn every_view_says_its_layout() {
    let linear = calls::start("A |- A", "classical", "").unwrap();
    assert_eq!(value(&linear)["view"]["layout"], json!("tree"));
    let session = ordinary::start("a |- a", "intuitionistic", "").unwrap();
    assert_eq!(value(&session)["view"]["layout"], json!("tree"));
    let derivation = rules::start("LJ", "a |- a", "", "").unwrap();
    assert_eq!(value(&derivation)["view"]["layout"], json!("tree"));
    let fitch = r#"{"view": {"layout": "fitch"}}"#;
    let session = value(&derivation)["session"].to_string();
    assert_eq!(
        value(&rules::view(&session, fitch).unwrap())["layout"],
        json!("list")
    );
}

#[wasm_bindgen_test(unsupported = test)]
fn the_ordinary_suite_reads_and_certifies() {
    let read = value(&ordinary::read("a, a -> b |- b", "intuitionistic", "").unwrap());
    assert_eq!(read["kind"], json!("sequent"));
    assert_eq!(read["text"], json!("a, a → b ⊢ b"));
    // Two formulas right of ⊢ are refused outside classical logic.
    let refused = ordinary::read("|- a, b", "minimal", "").unwrap_err();
    assert_eq!(value(&refused.0)["code"], json!("succedents"));
    // An outcome's certificate: a valid one's derivation, a refuted one's
    // countermodel.
    let settings = r#"{"ordinary": {"method": "ljt"}}"#;
    let session = value(&ordinary::start("a |- a", "intuitionistic", settings).unwrap());
    let closed = value(&ordinary::close(&session["session"].to_string(), 0, "").unwrap());
    let certificate = ordinary::certificate(&closed["outcome"].to_string(), "").unwrap();
    assert!(certificate.contains("Proof."));
    let session = value(&ordinary::start("a \\/ b |- a", "intuitionistic", "").unwrap());
    let closed = value(&ordinary::close(&session["session"].to_string(), 0, "").unwrap());
    let certificate = ordinary::certificate(&closed["outcome"].to_string(), "").unwrap();
    assert!(certificate.contains("Proof."));
}
