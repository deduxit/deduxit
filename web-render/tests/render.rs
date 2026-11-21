// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The renderers as the page relies on them, natively and under Node.

use deduxit_web::calls;
use deduxit_web_render::render;
use serde_json::{Value, json};
use wasm_bindgen_test::wasm_bindgen_test;

/// The session of a call's answer.
fn session(answer: &str) -> String {
    serde_json::from_str::<Value>(answer).unwrap()["session"].to_string()
}

/// The code of a failure's JSON.
fn code(failure: &deduxit_web_render::Failure) -> String {
    let json: Value = serde_json::from_str(&failure.0).unwrap();
    json["code"].as_str().unwrap().to_owned()
}

/// The drawings render to PNG and PDF from the font the page passes.
#[wasm_bindgen_test(unsupported = test)]
fn drawings_render_with_the_page_s_font() {
    let font = include_bytes!("../../cli/fonts/Euler-Math.otf");
    let started = calls::start("A |- A", "classical", "").unwrap();
    let drawing = calls::draw(&session(&started), "").unwrap();
    let png = render(&drawing, "png", font, "").unwrap();
    assert_eq!(&png[1..4], b"PNG");
    let dated = json!({"styles": {"pdf": {"date": {"year": 2026, "month": 10, "day": 19, "hour": 12, "minute": 0, "second": 0}}}});
    let pdf = render(&drawing, "pdf", font, &dated.to_string()).unwrap();
    assert_eq!(&pdf[..5], b"%PDF-");
}

/// The settings choose an export's math font by name: the drawing asks
/// for it and is laid out in its advances, and only its file sets the
/// text of the render.
#[wasm_bindgen_test(unsupported = test)]
fn an_export_is_set_in_the_font_the_settings_name() {
    let euler = include_bytes!("../../cli/fonts/Euler-Math.otf");
    let stix = include_bytes!("../../cli/fonts/STIXTwoMath-Regular.otf");
    let started = calls::start("A ⊗ B |- A ⊗ B", "classical", "").unwrap();
    let chosen = json!({"styles": {"svg": {"font": "stix-two"}}}).to_string();
    let drawing = calls::export(&session(&started), "svg", &chosen).unwrap();
    let default = calls::export(&session(&started), "svg", "").unwrap();
    assert!(
        drawing.contains("font-family=\"'STIX Two Math', serif\""),
        "{drawing}"
    );
    assert!(default.contains("'Euler Math'"));
    let unnamed = |svg: &str, family: &str| svg.replace(family, "");
    assert_ne!(
        unnamed(&drawing, "'STIX Two Math', serif"),
        unnamed(&default, "'Euler Math', 'Neo Euler', serif"),
    );
    let set = render(&drawing, "png", stix, "").unwrap();
    let unset = render(&drawing, "png", euler, "").unwrap();
    assert_ne!(set, unset, "the drawing's text needs its font's file");
}

/// A render fails as the bindings' calls fail: an unknown format, a
/// misspelt setting and a render past the tab's memory bound each throw
/// the library's JSON with its code.
#[wasm_bindgen_test(unsupported = test)]
fn a_failure_is_the_bindings_json() {
    let font = include_bytes!("../../cli/fonts/Euler-Math.otf");
    let started = calls::start("A |- A", "classical", "").unwrap();
    let drawing = calls::draw(&session(&started), "").unwrap();
    assert_eq!(
        code(&render(&drawing, "jpeg", font, "").unwrap_err()),
        "unknown_name"
    );
    let misspelt = json!({"limits": {"budget": 5}}).to_string();
    assert_eq!(
        code(&render(&drawing, "png", font, &misspelt).unwrap_err()),
        "json"
    );
    let small = json!({"limits": {"memory_bytes": 1}}).to_string();
    let refused: Value =
        serde_json::from_str(&render(&drawing, "png", font, &small).unwrap_err().0).unwrap();
    assert_eq!(refused["setting"], "limits.memory_bytes", "{refused}");
}
