// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The workspace's calls as the page makes them, under WebAssembly in Node
//! and as ordinary tests elsewhere.

use deduxit_web::workspace;
use serde_json::{Value, json};
use wasm_bindgen_test::wasm_bindgen_test;

/// Reads a call's JSON.
fn value(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// The workspace's document of a call's answer, as the page keeps it: the
/// text of the answer's string.
fn document(answer: &str) -> String {
    value(answer)["workspace"].as_str().unwrap().to_owned()
}

/// A classical sequent's value.
fn classical(text: &str) -> String {
    workspace::statement("ordinary", text, "classical", "").unwrap()
}

/// The action "Prove it in intuitionistic logic" makes, on an empty
/// workspace: the sequent, its translation and a proof of it.
fn translated() -> String {
    let action = json!({"name": "translation", "edits": [
        {"add": {"kind": "ordinary.sequent", "value": classical("|- a \\/ ~a")}},
        {"connect": {"operation": "ordinary.translate", "from": [{"made": 0}],
            "parameters": {"translation": "glivenko", "target": "intuitionistic"}}},
        {"connect": {"operation": "ordinary.start", "from": [{"made": 1}]}},
    ]});
    let answer = workspace::edit("", &action.to_string(), "", "").unwrap();
    assert_eq!(value(&answer)["made"], json!([0, 1, 2]));
    document(&answer)
}

/// The state a node of a view is in.
fn state(view: &Value, node: u64) -> Value {
    view["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| n["node"] == node)
        .unwrap()["state"]
        .clone()
}

/// A link is one action: the translation and the proof of it are made at
/// once and current, and undo takes all three back by the action's name.
#[wasm_bindgen_test(unsupported = test)]
fn a_link_is_one_action_that_undo_takes_back() {
    let made = translated();
    let view = value(&workspace::view(&made, "", "").unwrap());
    assert_eq!(state(&view, 1)["state"], "current");
    assert_eq!(state(&view, 2)["state"], "current");
    assert_eq!(view["nodes"][1]["label"], "A negative translation");
    assert_eq!(view["nodes"][1]["cost"], "instant");
    assert_eq!(view["nodes"][2]["class"], "work");
    assert_eq!(view["nodes"][1]["column"], 1);
    assert_eq!(view["undo"], "translation");
    // The page still sends the made tab's session, whose node the undo takes away.
    let view = value(&workspace::view(&made, "", "").unwrap());
    let work = json!({"2": view["nodes"][2]["value"]}).to_string();
    let undone = value(&workspace::undo(&made, &work, "", false).unwrap());
    assert_eq!(undone["name"], "translation");
    let gone = undone["workspace"].as_str().unwrap();
    assert!(workspace::view(gone, &work, "").is_ok());
    let undone_document = document(&workspace::undo(&made, "", "", false).unwrap());
    let view = value(&workspace::view(&undone_document, "", "").unwrap());
    assert_eq!(view["nodes"], json!([]));
    assert_eq!(view["redo"], "translation");
}

/// A computed check is accepted while the node still asks for it, and a
/// changed sequent puts what follows out of date, naming the input.
#[wasm_bindgen_test(unsupported = test)]
fn a_check_is_computed_apart_and_a_change_puts_what_follows_out_of_date() {
    let made = translated();
    let computed = workspace::compute(&made, 1, "check", "", "").unwrap();
    let accepted = value(&workspace::accept(&made, &computed, "", "").unwrap());
    assert_eq!(accepted["accepted"], true);
    let checked = document(&workspace::accept(&made, &computed, "", "").unwrap());
    let view = value(&workspace::view(&checked, "", "").unwrap());
    assert_eq!(view["nodes"][1]["check"]["status"], "held");
    let action = json!({"name": "sequent change", "edits": [
        {"set": {"node": 0, "value": classical("|- a -> b \\/ ~b")}},
    ]});
    let changed = document(&workspace::edit(&checked, &action.to_string(), "", "").unwrap());
    let view = value(&workspace::view(&changed, "", "").unwrap());
    assert_eq!(
        state(&view, 1),
        json!({"state": "out_of_date", "cause": "input", "node": 0})
    );
    assert_eq!(state(&view, 2)["cause"], "waits");
    // The computed run of the earlier sequent no longer fits.
    let late = value(&workspace::accept(&changed, &computed, "", "").unwrap());
    assert_eq!(late["accepted"], false);
    // A restart makes the translation it needs in the same action.
    let action = json!({"name": "restart", "edits": [{"restart": {"node": 2}}]});
    let restarted = document(&workspace::edit(&changed, &action.to_string(), "", "").unwrap());
    let view = value(&workspace::view(&restarted, "", "").unwrap());
    assert_eq!(state(&view, 2)["state"], "current");
    let undone = value(&workspace::undo(&restarted, "", "", false).unwrap());
    assert_eq!(undone["name"], "restart");
    assert!(undone["sessions"]["2"].is_object());
}

/// A link holds the recipes and the proofs, no runs: read back, the
/// translation is not run, and the proof keeps its session.
#[wasm_bindgen_test(unsupported = test)]
fn a_packed_link_reads_back_without_its_runs() {
    let made = translated();
    let packed = workspace::pack(&made, "[2]", "", "").unwrap();
    assert!(!packed.contains("\"records\":[{"));
    let opened = workspace::open(&packed, "").unwrap();
    let view = value(&workspace::view(&opened, "", "").unwrap());
    assert_eq!(view["nodes"].as_array().unwrap().len(), 3);
    assert_eq!(state(&view, 1)["state"], "not_run");
    assert!(view["nodes"][2]["value"].is_object());
    let ran = value(&workspace::run(&opened, "[1]", "output", "", "").unwrap());
    assert_eq!(ran["ran"], json!([1]));
}

/// The tab's bounds refuse an action past them with the library's code
/// and its setting, and keep nothing of it; a malformed action is refused
/// before any edit.
#[wasm_bindgen_test(unsupported = test)]
fn an_action_past_a_bound_or_malformed_is_refused_whole() {
    let action = json!({"name": "two", "edits": [
        {"add": {"kind": "ordinary.sequent", "value": classical("a |- a")}},
        {"add": {"kind": "ordinary.sequent", "value": classical("b |- b")}},
    ]});
    let tight = r#"{"limits": {"workspace": {"nodes": 1}}}"#;
    let refused = workspace::edit("", &action.to_string(), "", tight).unwrap_err();
    let refused = value(&refused.0);
    assert_eq!(refused["code"], "workspace_limit");
    assert_eq!(refused["setting"], "limits.workspace.nodes");
    for action in [
        json!({"name": "x"}),
        json!({"edits": [{"add": {}, "set": {}}]}),
        json!({"edits": [{"connect": {"operation": "ordinary.start", "from": [{"made": 0}]}}]}),
        json!({"edits": [{"fly": {}}]}),
    ] {
        let failure = workspace::edit("", &action.to_string(), "", "").unwrap_err();
        assert_eq!(value(&failure.0)["code"], "json", "{action}");
    }
}

/// `work` lays a proof's session in before a call, without an entry of
/// the log; a key that is no node's number is refused.
#[wasm_bindgen_test(unsupported = test)]
fn a_proof_s_session_is_laid_in_without_an_edit() {
    let made = translated();
    let view = value(&workspace::view(&made, "", "").unwrap());
    let session = view["nodes"][2]["value"].clone();
    let work = json!({"2": session}).to_string();
    let view = value(&workspace::view(&made, &work, "").unwrap());
    assert_eq!(view["undo"], "translation");
    assert!(workspace::view(&made, r#"{"two": {}}"#, "").is_err());
}
