// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Calculi given as data: the rule files of LK and LJ against the
//! ordinary checker, and the checker's parts on small calculi of each
//! kind it serves (natural deduction, a signed tableau, a labelled
//! modal calculus, an ordered calculus).

#![cfg(all(feature = "parse", feature = "serialize"))]

use deduxit::limits::Refusal;
use deduxit::ordinary::{self, Logic, Method, Verdict};
use deduxit::rules::{Calculus, Derivation, LJ, LK, LL, LM, Node, Step};
use deduxit::{Error, Limits, wire};
use serde_json::{Value, json};

/// Reads a rule file.
fn calculus(text: &str) -> Result<Calculus, Error> {
    let mut document = serde_json::Deserializer::from_str(text);
    wire::upgrade(&mut document, &Limits::default())
}

/// Reads a derivation and checks it against `calculus`.
fn check(calculus: &Calculus, derivation: &Value) -> Result<Derivation, Error> {
    let derivation: Derivation = wire::upgrade(derivation, &Limits::default())?;
    derivation.check(calculus, &Limits::default(), |_| false)?;
    Ok(derivation)
}

/// Returns why `calculus` refuses `derivation`.
fn refused(calculus: &Calculus, derivation: &Value) -> String {
    match check(calculus, derivation) {
        Err(Error::InvalidDerivation { reason, .. }) => reason,
        other => panic!("expected a refusal by the calculus, got {other:?}"),
    }
}

#[test]
fn the_rule_files_read_and_write_back() {
    for (text, name, rules) in [
        (LK, "LK", 29),
        (LJ, "LJ", 34),
        (LM, "LM", 32),
        (LL, "LL", 13),
        (deduxit::rules::TABLEAU, "tableau", 25),
    ] {
        let read = calculus(text).unwrap();
        assert_eq!(read.name(), name);
        assert_eq!(read.rules().len(), rules);
        let zones = match name {
            "LL" => &["right"][..],
            "tableau" => &["branch"],
            _ => &["left", "right"],
        };
        assert_eq!(read.zones().collect::<Vec<_>>(), zones);
        // The file is written back as it was read.
        let written = serde_json::to_value(&read).unwrap();
        assert_eq!(written, serde_json::from_str::<Value>(text).unwrap());
        assert_eq!(written["version"], 15, "the level of a rule file");
    }
    // The schema names every key the rule files use.
    let schema: Value = serde_json::from_str(deduxit::rules::SCHEMA).unwrap();
    let file: Value = serde_json::from_str(LJ).unwrap();
    let known = |keys: &Value, of: &Value| {
        let known = of["properties"].as_object().unwrap();
        keys.as_object()
            .unwrap()
            .keys()
            .all(|key| known.contains_key(key))
    };
    assert!(known(&file, &schema));
    for (list, part) in [
        ("connectives", "connective"),
        ("sequent", "zone"),
        ("rules", "rule"),
    ] {
        for entry in file[list].as_array().unwrap() {
            assert!(known(entry, &schema["$defs"][part]), "{entry}");
        }
    }
    let lk = calculus(LK).unwrap();
    // A node gives the place of its principal formula, and an axiom none.
    assert_eq!(lk.positions("∧R"), Some(1));
    assert_eq!(lk.positions("ax"), Some(0));
    assert_eq!(lk.positions("cut"), None);
}

/// The ordinary sequents the cross-check decides.
const SEQUENTS: [&str; 18] = [
    "a |- a",
    "a /\\ b |- b /\\ a",
    "a \\/ b |- b \\/ a",
    "|- a -> (b -> a)",
    "a -> b, b -> c |- a -> c",
    "|- ~~(a \\/ ~a)",
    "|- (a -> b) -> (~b -> ~a)",
    "a <-> b |- b <-> a",
    "|- a \\/ ~a",
    "false |- a",
    "|- true",
    "~(a /\\ b) |- ~a \\/ ~b",
    "(a -> b) -> a |- a",
    "a, ~a |- b",
    "a \\/ (b /\\ c) |- (a \\/ b) /\\ (a \\/ c)",
    "|- (a /\\ (a -> b)) -> b",
    "((a -> b) -> c) -> d, c |- d",
    "~a |- a -> b",
];

/// Every rule name of the ordinary derivation's form.
const RULES: [&str; 26] = [
    "ax", "WL", "WR", "CL", "CR", "∧L", "∧L₁", "∧L₂", "∧R", "∨L", "∨R", "∨R₁", "∨R₂", "→L", "→R",
    "¬L", "¬R", "↔L", "↔L₁", "↔L₂", "↔R", "⊥L", "⊥R", "⊤L", "⊤R", "cut",
];

/// Returns `document`, an ordinary derivation, with inference `at` broken
/// in each way the checker's guards answer: another rule, another
/// principal formula, a formula more or less, other premises.
fn broken(document: &Value, at: usize) -> Vec<Value> {
    let mut out = Vec::new();
    let mut change = |edit: &dyn Fn(&mut Value)| {
        let mut copy = document.clone();
        edit(&mut copy["inferences"][at]);
        if copy != *document {
            out.push(copy);
        }
    };
    for rule in RULES {
        change(&|inference| inference["rule"] = json!(rule));
    }
    for side in ["left", "right"] {
        for place in 0..3 {
            change(&|inference| inference["principal"] = json!([side, place]));
        }
        change(&|inference| {
            inference[side].as_array_mut().unwrap().pop();
        });
        for formula in 0..2 {
            change(&|inference| inference[side].as_array_mut().unwrap().push(json!(formula)));
        }
    }
    change(&|inference| {
        inference.as_object_mut().unwrap().remove("principal");
    });
    change(&|inference| {
        if let Some(premises) = inference.get_mut("premises") {
            premises.as_array_mut().unwrap().reverse();
        }
    });
    out
}

#[test]
fn terms_print_by_the_calculus() {
    let lk = calculus(LK).unwrap();
    let document = json!({"version": 15, "calculus": "LK",
        "terms": ["a", "b", ["∧", 0, 1], ["¬", 2], ["→", 3, 0], ["→", 1, 0], ["→", 0, 5],
            ["→", 5, 0], ["∧", 1, 0]],
        "nodes": [{"sequent": [[2], [8]], "rule": "ax"}]});
    let derivation: Derivation = wire::upgrade(&document, &Limits::default()).unwrap();
    let show = |index: u32| {
        let id = deduxit::rules::TermId::new(index);
        lk.show(derivation.terms(), id).to_string()
    };
    // Brackets where the precedences and the associativity ask for them.
    assert_eq!(show(4), "¬(a ∧ b) → a");
    assert_eq!(show(6), "a → b → a");
    assert_eq!(show(7), "(b → a) → a");
    let root = derivation.root().unwrap().id();
    assert_eq!(
        lk.show_sequent(&derivation, root).to_string(),
        "a ∧ b ⊢ b ∧ a"
    );
    // Past its bound the text ends with an ellipsis.
    assert_eq!(
        lk.show(derivation.terms(), deduxit::rules::TermId::new(4))
            .limit(5)
            .to_string(),
        "¬(a ∧…"
    );
    // A refused node is named with its sequent.
    let reason = refused(&lk, &document);
    assert!(reason.contains("concluding a ∧ b ⊢ b ∧ a"), "{reason}");
}

#[test]
fn the_rule_files_agree_with_the_ordinary_checker() {
    let limits = Limits::default();
    let methods = [
        Method::Embedding,
        Method::Ljt,
        #[cfg(feature = "sat")]
        Method::Sat,
        #[cfg(feature = "interactive")]
        Method::Tableau,
    ];
    let (mut derivations, mut accepted, mut refused, mut cuts) = (0, 0, 0, 0);
    for (logic, file) in [
        (Logic::Classical, LK),
        (Logic::Intuitionistic, LJ),
        (Logic::Minimal, LM),
    ] {
        let rules = calculus(file).unwrap();
        // Whether both checkers accept a derivation, which they must
        // agree on.
        let verdict = |derivation: &ordinary::Derivation, document: &Value| {
            let ordinary = derivation.check(&limits, |_| false).is_ok();
            let generic = Derivation::from_ordinary(derivation)
                .and_then(|generic| generic.check(&rules, &limits, |_| false));
            assert_eq!(
                ordinary,
                generic.is_ok(),
                "the checkers differ ({generic:?}) on {document}"
            );
            ordinary
        };
        for text in SEQUENTS {
            let sequent: ordinary::Sequent = text.parse().unwrap();
            for &method in methods.iter().filter(|method| method.decides(logic)) {
                let options = ordinary::Options::default()
                    .with_logic(logic)
                    .with_method(Some(method));
                let Ok(outcome) = ordinary::decide(&sequent, &options) else {
                    continue;
                };
                let Verdict::Valid(evidence) = &outcome.verdict else {
                    continue;
                };
                let Ok(derivation) = evidence.derivation() else {
                    continue;
                };
                let document = serde_json::to_value(derivation).unwrap();
                assert!(verdict(derivation, &document), "{text} in {logic}");
                derivations += 1;
                let inferences = document["inferences"].as_array().unwrap();
                cuts += inferences.iter().filter(|i| i["rule"] == "cut").count();
                if inferences.len() > 60 {
                    continue;
                }
                for at in 0..inferences.len() {
                    for mutant in broken(&document, at) {
                        // A document that is no derivation's is skipped.
                        let Ok(read) = wire::upgrade::<ordinary::Derivation, _>(&mutant, &limits)
                        else {
                            continue;
                        };
                        if verdict(&read, &mutant) {
                            accepted += 1;
                        } else {
                            refused += 1;
                        }
                    }
                }
            }
        }
    }
    // The ordinary checker asks for both implications of an equivalence
    // among the formulas, also where a rule writes out one: no engine
    // leaves one out, so this document is written by hand.
    let lone = json!({"version": 8, "logic": "classical",
        "sequent": {"formulas": [{"V": 0}, {"V": 1}, {"→": [0, 1]}, {"↔": [0, 1]}],
            "atoms": ["a", "b"], "left": [3, 0], "right": [1]},
        "inferences": [
            {"left": [0], "right": [0], "rule": "ax"},
            {"left": [1], "right": [1], "rule": "ax"},
            {"left": [0, 2], "right": [1], "rule": "→L", "principal": ["left", 1],
             "premises": [0, 1]},
            {"left": [0, 3], "right": [1], "rule": "↔L₁", "principal": ["left", 1],
             "premises": [2]}]});
    let read: ordinary::Derivation = wire::upgrade(&lone, &limits).unwrap();
    assert!(read.check(&limits, |_| false).is_err());
    assert!(Derivation::from_ordinary(&read).is_err());
    // The sample exercises both verdicts and LJ's cut.
    assert!(derivations >= 60, "{derivations} derivations");
    assert!(
        refused > 10_000 && accepted > 0,
        "{refused} refused, {accepted} accepted"
    );
    assert!(cuts > 0, "no derivation with a cut");
}

/// Natural deduction for implication and conjunction: a judgement is one
/// formula, and `→I` discharges a hypothesis.
const NJ: &str = r#"{"version": 15, "name": "NJ",
  "connectives": [{"name": "→", "arity": 2}, {"name": "∧", "arity": 2}],
  "sequent": [{"name": "formula", "discipline": "multiset", "least": 1, "most": 1}],
  "variables": {"formula": ["A", "B"]},
  "rules": [
    {"name": "→I", "premises": [{"formula": ["B"], "discharges": [{"formula": ["A"]}]}],
     "conclusion": {"formula": [["→", "A", "B"]]}},
    {"name": "→E", "premises": [{"formula": [["→", "A", "B"]]}, {"formula": ["A"]}],
     "conclusion": {"formula": ["B"]}},
    {"name": "∧I", "premises": [{"formula": ["A"]}, {"formula": ["B"]}],
     "conclusion": {"formula": [["∧", "A", "B"]]}}]}"#;

#[test]
fn natural_deduction_discharges_by_scope() {
    let nj = calculus(NJ).unwrap();
    // No rule of natural deduction needs a position.
    assert!(nj.rules().all(|rule| nj.positions(rule) == Some(0)));
    // a → (b → a ∧ b): two nested subproofs, the inner using the outer's
    // hypothesis.
    let terms = json!(["a", "b", ["∧", 0, 1], ["→", 1, 2], ["→", 0, 3], ["→", 1, 0]]);
    let nested = json!({"version": 15, "calculus": "NJ", "terms": terms, "scopes": [0, 1],
        "nodes": [
            {"sequent": [[0]], "hypothesis": true, "scope": 1},
            {"sequent": [[1]], "hypothesis": true, "scope": 2},
            {"sequent": [[2]], "rule": "∧I", "premises": [0, 1], "scope": 2},
            {"sequent": [[3]], "rule": "→I", "premises": [2], "scope": 1},
            {"sequent": [[4]], "rule": "→I", "premises": [3]}]});
    let derivation = check(&nj, &nested).unwrap();
    assert_eq!(derivation.assumptions().count(), 0);
    // The document is written back as it was read.
    assert_eq!(serde_json::to_value(&derivation).unwrap(), nested);

    // A hypothesis of the root scope is an assumption: a, a → b ⊢ b, and
    // b → a from it by a discharge of nothing.
    let open = json!({"version": 15, "calculus": "NJ",
        "terms": ["a", "b", ["→", 0, 1], ["→", 1, 0]],
        "nodes": [
            {"sequent": [[0]], "hypothesis": true},
            {"sequent": [[2]], "hypothesis": true},
            {"sequent": [[1]], "rule": "→E", "premises": [1, 0]},
            {"sequent": [[3]], "rule": "→I", "premises": [0]}]});
    assert_eq!(check(&nj, &open).unwrap().assumptions().count(), 2);

    // A line of a closed subproof is cited from outside it.
    let mut escaped = nested.clone();
    escaped["nodes"][4] = json!({"sequent": [[2]], "rule": "∧I", "premises": [0, 1]});
    assert!(refused(&nj, &escaped).contains("closed here"));
    // The subproof's hypothesis is not the one the rule discharges.
    let mut other = nested.clone();
    other["nodes"][1]["sequent"] = json!([[0]]);
    other["nodes"][2] = json!({"sequent": [[0]], "hypothesis": true, "scope": 2});
    other["nodes"][3] = json!({"sequent": [[5]], "rule": "→I", "premises": [2], "scope": 1});
    other["nodes"][4] = json!({"sequent": [[5]], "hypothesis": true});
    assert!(refused(&nj, &other).contains("none the rule discharges"));
    // A discharging premise must lie in a scope inside the node's.
    let mut same = nested.clone();
    same["nodes"][3]["scope"] = json!(2);
    assert!(refused(&nj, &same).contains("discharges hypotheses"));
    // The root lies in no subproof: its hypotheses would be neither
    // discharged nor assumptions.
    let inside = json!({"version": 15, "calculus": "NJ",
        "terms": ["a", "b", ["→", 0, 1]], "scopes": [0],
        "nodes": [
            {"sequent": [[0]], "hypothesis": true, "scope": 1},
            {"sequent": [[2]], "hypothesis": true, "scope": 1},
            {"sequent": [[1]], "rule": "→E", "premises": [1, 0], "scope": 1}]});
    assert!(refused(&nj, &inside).contains("no rule closes"));
    // An open goal is no derivation.
    let mut goal = nested;
    goal["nodes"][3] = json!({"sequent": [[3]], "scope": 1});
    assert!(refused(&nj, &goal).contains("open goal"));
}

/// A signed tableau: a branch is a set of signed formulas, and it closes
/// on an atom signed both ways.
const TABLEAU: &str = r#"{"version": 15, "name": "tableau", "sorts": ["signed"],
  "connectives": [{"name": "∧", "arity": 2},
    {"name": "T", "arity": 1, "sorts": ["formula", "signed"]},
    {"name": "F", "arity": 1, "sorts": ["formula", "signed"]}],
  "sequent": [{"name": "branch", "sort": "signed", "discipline": "set"}],
  "variables": {"formula": ["A", "B"]}, "contexts": ["S"],
  "rules": [
    {"name": "T∧", "premises": [{"branch": ["S", ["T", "A"], ["T", "B"]]}],
     "conclusion": {"branch": ["S", ["T", ["∧", "A", "B"]]]}},
    {"name": "×", "conclusion": {"branch": ["S", ["T", "A"], ["F", "A"]]}, "atomic": ["A"]}]}"#;

#[test]
fn a_tableau_closes_on_complementary_literals() {
    let tableau = calculus(TABLEAU).unwrap();
    // The closure cites both lines.
    assert_eq!(tableau.positions("×"), Some(2));
    let terms = json!([
        "a",
        "b",
        ["∧", 0, 1],
        ["T", 2],
        ["T", 0],
        ["T", 1],
        ["F", 0],
        ["F", 2]
    ]);
    let closed = json!({"version": 15, "calculus": "tableau", "terms": terms, "nodes": [
        {"sequent": [[3, 6, 4, 5]], "rule": "×", "at": [2, 1]},
        {"sequent": [[3, 6]], "rule": "T∧", "at": [0], "premises": [0]}]});
    check(&tableau, &closed).unwrap();
    // A set: the branch above may drop the formula expanded.
    let mut dropped = closed.clone();
    dropped["nodes"][0] = json!({"sequent": [[6, 4, 5]], "rule": "×", "at": [1, 0]});
    check(&tableau, &dropped).unwrap();
    // But it may not hold a formula the branch below lacks.
    let mut grown = closed.clone();
    grown["nodes"][0] = json!({"sequent": [[3, 6, 4, 5, 7]], "rule": "×", "at": [2, 1]});
    assert!(refused(&tableau, &grown).contains("neither written out"));
    // A branch does not close on a compound formula.
    let compound = json!({"version": 15, "calculus": "tableau", "terms": terms, "nodes": [
        {"sequent": [[3, 7]], "rule": "×", "at": [0, 1]}]});
    assert!(refused(&tableau, &compound).contains("atom"));
    // A set holds a member once, and a member has the zone's sort.
    let twice = json!({"version": 15, "calculus": "tableau", "terms": terms, "nodes": [
        {"sequent": [[4, 6, 4]], "rule": "×", "at": [0, 1]}]});
    assert!(refused(&tableau, &twice).contains("twice"));
    let unsigned = json!({"version": 15, "calculus": "tableau", "terms": terms, "nodes": [
        {"sequent": [[0, 6]], "rule": "×", "at": [0, 1]}]});
    assert!(refused(&tableau, &unsigned).contains("sort"));
}

/// Two rules of the labelled calculus of the modal logic K: labels and
/// relational atoms are terms of their own sorts.
const G3K: &str = r#"{"version": 15, "name": "G3K", "sorts": ["label", "judgement"],
  "connectives": [{"name": "□", "arity": 1},
    {"name": ":", "arity": 2, "sorts": ["label", "formula", "judgement"]},
    {"name": "R", "arity": 2, "sorts": ["label", "label", "judgement"]}],
  "sequent": [{"name": "left", "sort": "judgement", "discipline": "multiset"},
    {"name": "right", "sort": "judgement", "discipline": "multiset", "mark": "⊢"}],
  "variables": {"formula": ["A"], "label": ["x", "y"]}, "contexts": ["Γ", "Δ"],
  "rules": [
    {"name": "ax", "conclusion": {"left": ["Γ", [":", "x", "A"]], "right": [[":", "x", "A"], "Δ"]},
     "atomic": ["A"]},
    {"name": "R□", "premises": [{"left": ["Γ", ["R", "x", "y"]], "right": [[":", "y", "A"], "Δ"]}],
     "conclusion": {"left": ["Γ"], "right": [[":", "x", ["□", "A"]], "Δ"]}, "fresh": ["y"]},
    {"name": "L□",
     "premises": [{"left": ["Γ", [":", "x", ["□", "A"]], ["R", "x", "y"], [":", "y", "A"]],
                   "right": ["Δ"]}],
     "conclusion": {"left": ["Γ", [":", "x", ["□", "A"]], ["R", "x", "y"]], "right": ["Δ"]}}]}"#;

#[test]
fn a_labelled_rule_needs_a_fresh_label() {
    let g3k = calculus(G3K).unwrap();
    // w : □p ⊢ w : □p through a new world v.
    let derivation = |v: u32| {
        json!({"version": 15, "calculus": "G3K",
        "terms": [{"atom": "w", "sort": "label"}, {"atom": "v", "sort": "label"}, "p",
            ["□", 2], [":", 0, 3], ["R", 0, v], [":", v, 2]],
        "nodes": [
            {"sequent": [[4, 5, 6], [6]], "rule": "ax", "at": [2, 0]},
            {"sequent": [[4, 5], [6]], "rule": "L□", "at": [0, 1], "premises": [0]},
            {"sequent": [[4], [4]], "rule": "R□", "at": [0, 1], "premises": [1]}]})
    };
    check(&g3k, &derivation(1)).unwrap();
    // The world the rule introduces is the one the conclusion is about.
    assert!(refused(&g3k, &derivation(0)).contains("occurs in the conclusion"));
    // A label is no formula.
    let ill = json!({"version": 15, "calculus": "G3K",
        "terms": [{"atom": "w", "sort": "label"}, ["□", 0]], "nodes": [{"sequent": [[], []]}]});
    assert!(refused(&g3k, &ill).contains("term 1"));
}

/// An ordered calculus with one division, and a rule that boxes a whole
/// context.
const ORDERED: &str = r#"{"version": 15, "name": "ordered",
  "connectives": [{"name": "\\", "arity": 2}, {"name": "□", "arity": 1}],
  "sequent": [{"name": "left", "discipline": "sequence"},
    {"name": "right", "discipline": "multiset", "mark": "⊢", "least": 1, "most": 1}],
  "variables": {"formula": ["A", "B", "C"]}, "contexts": ["Γ", "Δ", "Δ′"],
  "rules": [
    {"name": "ax", "conclusion": {"left": ["A"], "right": ["A"]}},
    {"name": "\\L",
     "premises": [{"left": ["Γ"], "right": ["A"]}, {"left": ["Δ", "B", "Δ′"], "right": ["C"]}],
     "conclusion": {"left": ["Δ", "Γ", ["\\", "A", "B"], "Δ′"], "right": ["C"]}},
    {"name": "K", "premises": [{"left": ["Γ"], "right": ["A"]}],
     "conclusion": {"left": [["□", "Γ"]], "right": [["□", "A"]]}}]}"#;

#[test]
fn a_sequence_keeps_its_order_and_a_context_its_connective() {
    let ordered = calculus(ORDERED).unwrap();
    let terms = json!(["a", "b", ["\\", 0, 1], ["□", 0], ["□", 1], ["□", 2]]);
    let derivation = |left: Value, at: Value| {
        json!({"version": 15, "calculus": "ordered", "terms": terms, "nodes": [
            {"sequent": [[0], [0]], "rule": "ax"},
            {"sequent": [[1], [1]], "rule": "ax"},
            {"sequent": [left, [1]], "rule": "\\L", "at": at, "premises": [0, 1]}]})
    };
    // a, a \ b ⊢ b, and not with the two the other way round.
    check(&ordered, &derivation(json!([0, 2]), json!([1, 0]))).unwrap();
    assert!(refused(&ordered, &derivation(json!([2, 0]), json!([0, 0]))).contains("contexts"));
    // □a, □(a \ b) ⊢ □b from the sequent above; a formula without the
    // box is no member of the boxed context.
    let boxed = |left: Value| {
        let mut document = derivation(json!([0, 2]), json!([1, 0]));
        let nodes = document["nodes"].as_array_mut().unwrap();
        nodes.push(json!({"sequent": [left, [4]], "rule": "K", "premises": [2]}));
        document
    };
    check(&ordered, &boxed(json!([3, 5]))).unwrap();
    assert!(refused(&ordered, &boxed(json!([3, 2]))).contains("contexts"));
    assert!(refused(&ordered, &boxed(json!([5, 3]))).contains("contexts"));
}

#[test]
fn a_rule_file_that_is_no_calculus_is_refused() {
    let lk: Value = serde_json::from_str(LK).unwrap();
    let reason = |file: &Value| match calculus(&file.to_string()) {
        Err(Error::InvalidCalculus { reason, .. }) => reason,
        Err(Error::Json { message, .. }) => message,
        other => panic!("expected a refusal, got {other:?}"),
    };
    // A name that nothing declares, so no typo becomes a variable.
    let mut typo = lk.clone();
    typo["rules"][0]["conclusion"]["left"] = json!(["E"]);
    assert!(reason(&typo).contains("\"E\" is neither"));
    // A key that is none of a rule's.
    let mut key = lk.clone();
    key["rules"][0]["premise"] = json!([]);
    assert!(reason(&key).contains("premise"));
    // Two contexts that no zone tells apart.
    let mut apart = lk.clone();
    apart["rules"][0] = json!({"name": "ax",
        "premises": [{"left": ["Γ", "Γ′"]}], "conclusion": {"left": ["Γ", "Γ′"]}});
    assert!(reason(&apart).contains("contexts \"Γ\", \"Γ′\" cannot be told apart"));
    // A connective with another number of arguments than it takes.
    let mut arity = lk.clone();
    arity["rules"][0]["conclusion"]["left"] = json!([["∧", "A"]]);
    assert!(reason(&arity).contains("takes 2 arguments"));
    // A connective with more arguments than any takes.
    let mut wide = lk.clone();
    wide["connectives"][0]["arity"] = json!(u32::MAX);
    assert!(reason(&wide).contains("255 at most"));
    // A connective over a context takes one argument.
    for arity in [0, 2] {
        let mut over = lk.clone();
        over["connectives"][0]["arity"] = json!(arity);
        over["rules"][0]["conclusion"]["left"] = json!([["⊤", "Γ"]]);
        assert!(reason(&over).contains("only a connective of one argument"));
    }
    // A condition names a variable once, a sequent each zone once.
    let mut fresh = lk.clone();
    fresh["rules"][0]["atomic"] = json!(["A", "A"]);
    assert!(reason(&fresh).contains("named twice"));
    // A condition's variable that no sequent of the rule binds is named.
    let mut unbound = lk.clone();
    unbound["rules"][0]["atomic"] = json!(["B"]);
    assert!(reason(&unbound).contains("the variable \"B\" of a condition"));
    // A sort with an empty name is said as such, not as declared twice.
    let mut empty = lk.clone();
    empty["sorts"] = json!([""]);
    assert!(reason(&empty).contains("a sort has an empty name"));
    let twice = LK.replacen(
        r#""conclusion": {"left": ["A"], "right": ["A"]}"#,
        r#""conclusion": {"left": ["A"], "left": ["A"], "right": ["A"]}"#,
        1,
    );
    assert!(
        matches!(calculus(&twice), Err(Error::Json { message, .. }) if message.contains("twice"))
    );
    // A pattern nested past the reader's bound.
    let mut deep = json!("A");
    for _ in 0..100 {
        deep = json!(["¬", deep]);
    }
    let mut nested = lk;
    nested["rules"][0]["conclusion"]["left"] = json!([deep]);
    assert!(reason(&nested).contains("nested deeper"));
}

#[test]
fn the_check_keeps_to_its_bounds() {
    let lk = calculus(LK).unwrap();
    let sequent: ordinary::Sequent = SEQUENTS[14].parse().unwrap();
    let options = ordinary::Options::default()
        .with_logic(Logic::Classical)
        .with_method(Some(Method::Embedding));
    let outcome = ordinary::decide(&sequent, &options).unwrap();
    let Verdict::Valid(evidence) = &outcome.verdict else {
        panic!("the sequent is valid");
    };
    let derivation = Derivation::from_ordinary(evidence.derivation().unwrap()).unwrap();
    let limits = Limits::default();
    // The stop is asked at every node.
    let mut asked = 0;
    derivation
        .check(&lk, &limits, |_| {
            asked += 1;
            false
        })
        .unwrap();
    assert!(asked >= derivation.len());
    let stopped = derivation.check(&lk, &limits, |_| true);
    assert!(matches!(
        stopped,
        Err(Error::Refused(Refusal::Stopped { .. }))
    ));
    let work = derivation.check(&lk, &limits.with_work(Some(10)), |_| false);
    assert!(matches!(
        work,
        Err(Error::Refused(Refusal::Work { limit: 10, .. }))
    ));
    let memory = derivation.check(&lk, &limits.with_memory_bytes(Some(64)), |_| false);
    assert!(matches!(
        memory,
        Err(Error::Refused(Refusal::Memory { .. }))
    ));
    // A name that must be new is looked for in every hypothesis in
    // force, and each look is work.
    let fresh = calculus(
        r#"{"version": 15, "name": "F", "connectives": [],
            "sequent": [{"name": "s", "discipline": "multiset"}],
            "variables": {"formula": ["A", "B"]},
            "rules": [{"name": "ax", "conclusion": {"s": ["A"]}},
                      {"name": "gen", "premises": [{"s": ["A"]}], "conclusion": {"s": ["B"]},
                       "fresh": ["A"]}]}"#,
    )
    .unwrap();
    let mut nodes = vec![json!({"sequent": [[0]], "hypothesis": true}); 300];
    nodes.push(json!({"sequent": [[1]], "rule": "ax"}));
    nodes.extend(vec![
        json!({"sequent": [[2]], "rule": "gen", "premises": [300]});
        300
    ]);
    let many = json!({"version": 15, "calculus": "F", "terms": ["p", "a", "q"], "nodes": nodes});
    let many: Derivation = wire::upgrade(&many, &limits).unwrap();
    many.check(&fresh, &limits, |_| false).unwrap();
    let walked = many.check(&fresh, &limits.with_work(Some(6_000)), |_| false);
    assert!(matches!(walked, Err(Error::Refused(Refusal::Work { .. }))));
    // A derivation of another calculus, and a document above this
    // build's wire level.
    let lj = calculus(LJ).unwrap();
    let other = derivation.check(&lj, &limits, |_| false);
    assert!(matches!(other, Err(Error::InvalidDerivation { .. })));
    let newer = json!({"version": wire::LEVEL + 1, "calculus": "LK", "terms": [], "nodes": []});
    let version = wire::upgrade::<Derivation, _>(&newer, &limits);
    assert!(matches!(version, Err(Error::Version { .. })));
}

/// Returns a derivation of the linear suite, one-sided, written in the
/// rule file [`LL`]: its formulas as terms (the dual of an atom under
/// `~`), its inferences as nodes, each with the positions its rule needs.
fn from_linear(derivation: &deduxit::linear::proofs::Derivation<'_>, ll: &Calculus) -> Derivation {
    use deduxit::linear::{Kind, Term};
    let forest = derivation.forest();
    let sequent = forest.sequent();
    let mut out = Derivation::new("LL", 1);
    let mut ids = Vec::with_capacity(sequent.terms().len());
    for term in sequent.terms() {
        let terms = out.terms_mut();
        let id = |t: &deduxit::linear::TermId| ids[t.index()];
        let made = match *term {
            Term::Atom(a) => terms.atom(sequent.atom_name(a), "formula"),
            Term::DualAtom(a) => {
                let atom = terms.atom(sequent.atom_name(a), "formula").unwrap();
                terms.apply("~", &[atom])
            }
            Term::One => terms.apply("1", &[]),
            Term::Bot => terms.apply("⊥", &[]),
            Term::Top => terms.apply("⊤", &[]),
            Term::Zero => terms.apply("0", &[]),
            Term::Tensor(a, b) => terms.apply("⊗", &[id(&a), id(&b)]),
            Term::Par(a, b) => terms.apply("⅋", &[id(&a), id(&b)]),
            Term::With(a, b) => terms.apply("&", &[id(&a), id(&b)]),
            Term::Plus(a, b) => terms.apply("⊕", &[id(&a), id(&b)]),
            Term::Bang(a) => terms.apply("!", &[id(&a)]),
            Term::Quest(a) => terms.apply("?", &[id(&a)]),
            Term::Forall(_) | Term::Exists(_) => panic!("a first-order sequent in the sample"),
        };
        ids.push(made.unwrap());
    }
    for inference in derivation.inferences() {
        let members = inference.sequent();
        let zone: Vec<_> = members
            .iter()
            .map(|&m| ids[forest.term(derivation.occurrence(m)).index()])
            .collect();
        let name = inference.rule().rule.name();
        let positions: Vec<u32> = match ll.positions(name) {
            Some(2) => [Kind::Atom, Kind::DualAtom]
                .iter()
                .map(|&kind| {
                    let at = members
                        .iter()
                        .position(|&m| forest.kind(derivation.occurrence(m)) == kind);
                    u32::try_from(at.unwrap()).unwrap()
                })
                .collect(),
            Some(1) => vec![u32::try_from(inference.principal().unwrap()).unwrap()],
            _ => Vec::new(),
        };
        let premises: Vec<u32> = inference.premises().iter().map(|p| p.get()).collect();
        let sequent = [zone];
        out.push(
            Node::new(&sequent, Step::Rule(name))
                .premises(&premises)
                .positions(&positions),
        )
        .unwrap();
    }
    out
}

#[test]
fn the_linear_rule_file_agrees_with_the_linear_checker() {
    use deduxit::linear::proofs::{Compact, ViewOptions};
    use deduxit::linear::search::{Options, Verdict, prove_within};
    use deduxit::linear::{Mode, Sequent, families};
    let ll = calculus(LL).unwrap();
    let limits = Limits::default().with_work(Some(1 << 20));
    let mut view = ViewOptions::default();
    view.compact = Compact::Never;
    let mut sequents: Vec<(Sequent, Mode)> = [
        "|- a, ~a",
        "a * b |- b * a",
        "a & b |- a + b",
        "!a |- !a * !a",
        "!(a -o b), !a |- !b",
        "|- ?a, ?~a",
        "a, b |- (a * b) & (b * a)",
        "|- 1, top",
        "bot, a |- a",
        "!(a & b) |- !a * !b",
        "|- ?(a * ~a), !(~a par a)",
    ]
    .iter()
    .map(|text| (text.parse().unwrap(), Mode::CLASSICAL))
    .collect();
    for family in families::FAMILIES {
        for index in 0..family.instances {
            let Ok(instance) = family.instance(family.least, index) else {
                continue;
            };
            let classical =
                instance.mode == Mode::CLASSICAL || instance.mode == Mode::INTUITIONISTIC;
            if instance.provable && classical && !instance.sequent.is_first_order() {
                sequents.push((instance.sequent, instance.mode));
            }
        }
    }
    let (mut proofs, mut nodes, mut refused) = (0, 0, 0);
    for (sequent, mode) in &sequents {
        let Ok(outcome) = prove_within(sequent, *mode, &Options::default(), &limits, |_| false)
        else {
            continue;
        };
        let Verdict::Proved(proof) = outcome.verdict else {
            continue;
        };
        if proof.forest().has_cuts() {
            continue;
        }
        proof.check(*mode).unwrap();
        let Ok(linear) = proof.derivation_within(&view, &limits, |_| false) else {
            continue;
        };
        let derivation = from_linear(&linear, &ll);
        let checked = derivation.check(&ll, &limits, |_| false);
        assert!(checked.is_ok(), "{sequent}: {checked:?}");
        proofs += 1;
        nodes += derivation.len();
        // Another rule of the file at a node is refused, but where it
        // happens to fit as well.
        let document = serde_json::to_value(&derivation).unwrap();
        for at in 0..derivation.len().min(40) {
            let mut broken = document.clone();
            let rule = broken["nodes"][at]["rule"].as_str().unwrap().to_owned();
            let other = if rule == "⊕₁" { "⊕₂" } else { "⊕₁" };
            broken["nodes"][at]["rule"] = json!(other);
            if check(&ll, &broken).is_err() {
                refused += 1;
            }
        }
    }
    assert!(
        proofs >= 15 && nodes > 200,
        "{proofs} proofs, {nodes} nodes"
    );
    assert!(refused > nodes / 4, "{refused} refused");
}

#[cfg(feature = "interactive")]
#[test]
fn the_tableau_rule_file_agrees_with_the_tableaux() {
    use deduxit::ordinary::{Search, Tableau};
    let rules = calculus(deduxit::rules::TABLEAU).unwrap();
    let limits = Limits::default();
    let classical = ordinary::Options::default().with_logic(Logic::Classical);
    let (mut closed, mut open) = (0, 0);
    let not_valid = ["a |- b", "a \\/ b |- a", "|- (a -> b) -> a", "a <-> b |- a"];
    for text in SEQUENTS.iter().chain(&not_valid) {
        let sequent: ordinary::Sequent = text.parse().unwrap();
        let mut tableaux = Vec::new();
        // The tableau the method builds, closed or open.
        let outcome =
            ordinary::decide(&sequent, &classical.with_method(Some(Method::Tableau))).unwrap();
        if let Search::Tableau(tableau) = &outcome.search {
            tableaux.push((**tableau).clone());
        }
        // The tableau of the embedding's derivation, read upside down.
        let embedding = classical.with_method(Some(Method::Embedding));
        if let Ok(outcome) = ordinary::decide(&sequent, &embedding)
            && let Verdict::Valid(evidence) = &outcome.verdict
        {
            tableaux.push(Tableau::of(evidence.derivation().unwrap(), |_| false).unwrap());
        }
        for tableau in tableaux {
            let derivation = Derivation::from_tableau(&tableau).unwrap();
            let checked = derivation.check(&rules, &limits, |_| false);
            assert_eq!(tableau.is_closed(), checked.is_ok(), "{text}: {checked:?}");
            if checked.is_ok() {
                closed += 1;
                // A closure that cites another line is refused.
                let mut broken = serde_json::to_value(&derivation).unwrap();
                let leaf = &mut broken["nodes"][0];
                if leaf["rule"] == "×" {
                    leaf["at"] = json!([leaf["at"][1], leaf["at"][0]]);
                    assert!(check(&rules, &broken).is_err(), "{text}");
                }
            } else {
                open += 1;
            }
        }
    }
    assert!(closed >= 20 && open >= 4, "{closed} closed, {open} open");
}

#[test]
fn a_node_that_is_an_instance_passes() {
    // An empty context under a connective that no term is built with.
    let ll = calculus(LL).unwrap();
    let promoted = json!({"version": 15, "calculus": "LL", "terms": [["1"], ["!", 0]], "nodes": [
        {"sequent": [[0]], "rule": "1"},
        {"sequent": [[1]], "rule": "!", "at": [0], "premises": [0]}]});
    check(&ll, &promoted).unwrap();
    // A member found by value does not take the place a position names.
    let p1 = calculus(
        r#"{"version": 15, "name": "P1", "connectives": [],
            "sequent": [{"name": "s", "discipline": "multiset"}],
            "variables": {"formula": ["A", "B"]}, "contexts": ["Γ"],
            "rules": [{"name": "ax", "conclusion": {"s": ["Γ"]}},
                      {"name": "r", "premises": [{"s": ["A", "B", "Γ"]}],
                       "conclusion": {"s": ["A", "Γ"]}}]}"#,
    )
    .unwrap();
    let same = json!({"version": 15, "calculus": "P1", "terms": ["a"], "nodes": [
        {"sequent": [[0, 0]], "rule": "ax"},
        {"sequent": [[0]], "rule": "r", "at": [0, 0], "premises": [0]}]});
    check(&p1, &same).unwrap();
    // A discharged hypothesis in a set zone is compared as a set.
    let p2 = calculus(
        r#"{"version": 15, "name": "P2", "connectives": [{"name": "→", "arity": 2}],
            "sequent": [{"name": "s", "discipline": "set"}],
            "variables": {"formula": ["A", "B"]},
            "rules": [{"name": "d", "premises": [{"s": ["A", "B"], "discharges": [{"s": ["A", "B"]}]}],
                       "conclusion": {"s": [["→", "A", "B"]]}}]}"#,
    )
    .unwrap();
    let set = json!({"version": 15, "calculus": "P2", "terms": ["a", ["→", 0, 0]], "scopes": [0],
        "nodes": [
            {"sequent": [[0]], "hypothesis": true, "scope": 1},
            {"sequent": [[1]], "rule": "d", "premises": [0]}]});
    check(&p2, &set).unwrap();
}

#[test]
fn the_memory_bound_counts_every_context_and_zone() {
    // A thousand contexts each as large as a zone of ten thousand atoms.
    let contexts: Vec<String> = (0..1000).map(|k| format!("G{k}")).collect();
    let file = json!({"version": 15, "name": "C", "connectives": [],
        "sequent": [{"name": "s", "discipline": "set"}], "contexts": contexts,
        "rules": [{"name": "ax", "conclusion": {"s": contexts}},
                  {"name": "r", "premises": [{"s": contexts}], "conclusion": {"s": contexts}}]});
    let many = calculus(&file.to_string()).unwrap();
    let atoms: Vec<String> = (0..10_000).map(|k| format!("a{k}")).collect();
    let zone: Vec<usize> = (0..10_000).collect();
    let document = json!({"version": 15, "calculus": "C", "terms": atoms, "nodes": [
        {"sequent": [zone], "rule": "ax"},
        {"sequent": [zone], "rule": "r", "premises": [0]}]});
    let derivation: Derivation = wire::upgrade(&document, &Limits::default()).unwrap();
    let small = Limits::default().with_memory_bytes(Some(16 << 20));
    let refused = derivation.check(&many, &small, |_| false);
    assert!(
        matches!(refused, Err(Error::Refused(Refusal::Memory { .. }))),
        "{refused:?}"
    );
    // A rule file of a thousand premises over a thousand zones is short
    // and would compile into a grid of a million.
    let zones: Vec<Value> = (0..1000)
        .map(|k| json!({"name": format!("z{k}"), "discipline": "multiset"}))
        .collect();
    let grid = json!({"version": 15, "name": "G", "connectives": [], "sequent": zones,
        "rules": [{"name": "r", "premises": vec![json!({}); 1000], "conclusion": {}}]});
    let read = wire::upgrade::<Calculus, _>(&grid, &small);
    assert!(
        matches!(read, Err(Error::Refused(Refusal::Memory { .. }))),
        "{read:?}"
    );
}

/// Every rule of the calculus is a candidate on a goal: a legal step at
/// each choice of members, what it needs given, or one refusal with
/// apply's reason at its first choice; a refused rule is no rule `rules`
/// lists.
#[cfg(feature = "interactive")]
#[test]
fn candidates_give_every_rule_with_its_status() {
    use deduxit::StepFault;
    use deduxit::rules::{Apply, Interactive, NJ, Status};
    let limits = Limits::default();
    for (file, goal) in [(NJ, "a /\\ b -> b"), (LK, "a /\\ b |- b /\\ a")] {
        let calculus = calculus(file).unwrap();
        let names: Vec<String> = calculus.rules().map(str::to_owned).collect();
        // A rule that asks no member's position is tried first at no
        // members, the choice a refused candidate names.
        let unplaced: Vec<String> = names
            .iter()
            .filter(|name| calculus.positions(name) == Some(0))
            .cloned()
            .collect();
        let session = Interactive::within(calculus, goal, &[], &limits).unwrap();
        let root = session.goals()[0];
        let candidates = session.candidates(root).unwrap();
        for name in &names {
            assert!(
                candidates.iter().any(|c| &c.rule == name),
                "{name} in {goal}"
            );
        }
        let listed = session.rules(root).unwrap();
        for candidate in &candidates {
            let at = candidate.at.as_slice();
            let tried = session
                .clone()
                .apply(root, &Apply::new(&candidate.rule).at(at));
            match &candidate.status {
                Status::Applies => assert!(tried.is_ok(), "{candidate:?}"),
                Status::NeedsInput { needs, .. } => {
                    assert!(
                        matches!(&tried, Err(Error::StepNeeds { needs: n, .. }) if n == needs),
                        "{candidate:?}"
                    );
                }
                Status::Refused(error) => {
                    assert!(
                        !listed.iter().any(|a| a.rule == candidate.rule),
                        "{candidate:?}"
                    );
                    if !at.is_empty() || unplaced.contains(&candidate.rule) {
                        assert_eq!(tried.as_ref().err(), Some(error), "{candidate:?}");
                    }
                }
                other => panic!("a status this test does not know: {other:?}"),
            }
        }
        // `→I` is refused on a goal that is no implication, in fields.
        if file == NJ {
            let and_i = candidates.iter().find(|c| c.rule == "∧I").unwrap();
            assert!(
                matches!(&and_i.status, Status::Refused(Error::StepRefused { fault, .. })
                    if matches!(**fault, StepFault::Unmatched { .. })),
                "{and_i:?}"
            );
        }
    }
}

#[cfg(feature = "interactive")]
#[test]
fn a_session_builds_a_derivation_the_checker_accepts() {
    use deduxit::rules::{Apply, GoalId, Interactive, Undone};
    let limits = Limits::default();
    // LK: a ∧ b ⊢ b ∧ a, its goals numbered across both zones.
    let mut session =
        Interactive::within(calculus(LK).unwrap(), "a /\\ b |- b /\\ a", &[], &limits).unwrap();
    let root = session.goals()[0];
    let listed = session.rules(root).unwrap();
    let ready: Vec<(&str, &[u32])> = listed
        .iter()
        .filter(|a| a.needs.is_empty())
        .map(|a| (a.rule.as_str(), a.at.as_slice()))
        .collect();
    assert!(ready.contains(&("∧R", &[1][..])), "{listed:?}");
    assert!(ready.contains(&("∧L", &[0][..])), "{listed:?}");
    // A rule of two premises that splits needs the split given.
    let split = listed.iter().find(|a| a.rule == "∧R split").unwrap();
    assert_eq!(split.needs, ["Γ′", "Δ′"], "{split:?}");
    let goals = session.apply(root, &Apply::new("∧R").at(&[1])).unwrap();
    assert_eq!(goals.len(), 2);
    for (goal, rule) in goals.iter().zip(["∧L₂", "∧L₁"]) {
        let axiom = session.apply(*goal, &Apply::new(rule).at(&[0])).unwrap();
        session.apply(axiom[0], &Apply::new("ax")).unwrap();
    }
    assert!(session.is_finished());
    let derivation = session.finished(|_| false).unwrap();
    assert_eq!(derivation.len(), 5);
    // The session reads back as itself, and undo reopens the last goal.
    let document = serde_json::to_value(&session).unwrap();
    assert_eq!(document["version"], 17);
    let mut read: Interactive = wire::upgrade(&document, &limits).unwrap();
    assert_eq!(serde_json::to_value(&read).unwrap(), document);
    let Some(Undone::Reopened(reopened)) = read.undo() else {
        panic!("undo reopens the goal the last step closed");
    };
    assert_eq!(read.goals(), [reopened]);
    // A step the rule does not fit is refused, and the state stays.
    let wrong = read.apply(reopened, &Apply::new("∧R").at(&[1]));
    assert!(matches!(wrong, Err(Error::StepRefused { .. })), "{wrong:?}");
    assert_eq!(read.goals(), [reopened]);
    let _ = GoalId::index(reopened);
}

#[cfg(feature = "interactive")]
#[test]
fn a_session_discharges_by_scope() {
    use deduxit::rules::{Apply, Interactive};
    let limits = Limits::default();
    let nj = calculus(NJ).unwrap();
    // a → (b → a ∧ b), then (a → b) → a → b with →E's A given.
    let mut session = Interactive::within(nj.clone(), "a -> (b -> a /\\ b)", &[], &limits).unwrap();
    let inner = session
        .apply(session.goals()[0], &Apply::new("→I"))
        .unwrap();
    let pair = session.apply(inner[0], &Apply::new("→I")).unwrap();
    let both = session.apply(pair[0], &Apply::new("∧I")).unwrap();
    for goal in both {
        session.assume(goal).unwrap();
    }
    let derivation = session.finished(|_| false).unwrap();
    assert_eq!(derivation.assumptions().count(), 0);
    let mut modus = Interactive::within(nj.clone(), "b", &["a -> b", "a"], &limits).unwrap();
    let needs = modus.apply(modus.goals()[0], &Apply::new("→E"));
    assert!(
        matches!(needs, Err(Error::StepNeeds { ref needs, .. }) if needs == &["A"]),
        "{needs:?}"
    );
    let goals = modus
        .apply(modus.goals()[0], &Apply::new("→E").given(&[("A", "a")]))
        .unwrap();
    for goal in goals {
        modus.assume(goal).unwrap();
    }
    assert_eq!(modus.finished(|_| false).unwrap().assumptions().count(), 2);
    // A hypothesis that is not in force does not close a goal.
    let mut alone = Interactive::within(nj, "a -> a /\\ b", &[], &limits).unwrap();
    let body = alone.apply(alone.goals()[0], &Apply::new("→I")).unwrap();
    let parts = alone.apply(body[0], &Apply::new("∧I")).unwrap();
    alone.assume(parts[0]).unwrap();
    assert!(matches!(
        alone.assume(parts[1]),
        Err(Error::StepRefused { .. })
    ));
}

/// A session's Fitch proof opens each subproof with its hypotheses,
/// one no step cited (a vacuous discharge, or a goal still open) too.
#[cfg(feature = "interactive")]
#[test]
fn a_sessions_fitch_proof_shows_every_hypothesis() {
    use deduxit::rules::{Apply, Fitch, FitchOptions, Interactive};
    let limits = Limits::default();
    let nj = calculus(deduxit::rules::NJ).unwrap();
    let mut session = Interactive::within(nj.clone(), "a -> (b -> a)", &[], &limits).unwrap();
    let inner = session
        .apply(session.goals()[0], &Apply::new("→I"))
        .unwrap();
    let last = session.apply(inner[0], &Apply::new("→I")).unwrap();
    let fitch = |session: &Interactive| {
        let (derivation, _) = session.fitch_derivation().unwrap();
        Fitch::new(&nj, &derivation, &FitchOptions::default(), &limits)
            .unwrap()
            .to_string()
    };
    // Open: both hypotheses stand before the goal a, the session's lines
    // 1 and 2 its first two.
    let open = fitch(&session);
    let (derivation, lines) = session.fitch_derivation().unwrap();
    let drawn = Fitch::new(&nj, &derivation, &FitchOptions::default(), &limits).unwrap();
    assert_eq!(drawn.nodes().take(2).collect::<Vec<_>>(), lines);
    assert!(open.starts_with("1 │ │ a\n"), "{open}");
    assert!(open.contains("2 │ │ │ b\n"), "{open}");
    session.assume(last[0]).unwrap();
    // Closed by line 1, b never cited: still the second subproof's line.
    // Its →I cites line 1 itself, which lies outside the subproof (a
    // Fitch proof has no reiteration line).
    let closed = fitch(&session);
    assert_eq!(closed.lines().count(), 6, "{closed}");
    assert!(closed.contains("2 │ │ │ b\n"), "{closed}");
    assert!(closed.contains("→I 1–3"), "{closed}");
    // The tree has a node per use alone: a, b → a, a → (b → a).
    assert_eq!(session.derivation().unwrap().len(), 3);
}

/// The reviewer's witnesses of 62d: a goal closed by a hypothesis is a
/// Fitch proof, with or without lines derived beside it; equal
/// assumptions are one line; a line derived before a subproof opens
/// stands above it.
#[cfg(feature = "interactive")]
#[test]
fn a_sessions_fitch_proof_keeps_hypotheses_and_lines_in_order() {
    use deduxit::rules::{Apply, Derive, Fitch, FitchOptions, Interactive, LineId};
    let limits = Limits::default();
    let nj = calculus(deduxit::rules::NJ).unwrap();
    let drawn = |session: &Interactive| {
        let (derivation, lines) = session.fitch_derivation().unwrap();
        let fitch = Fitch::new(&nj, &derivation, &FitchOptions::default(), &limits).unwrap();
        let numbers: Vec<usize> = lines
            .iter()
            .map(|node| fitch.nodes().position(|n| n == *node).unwrap() + 1)
            .collect();
        (fitch.to_string(), numbers)
    };
    // a ⊢ a, the goal closed by its assumption: one line.
    let mut alone = Interactive::within(nj.clone(), "a", &["a"], &limits).unwrap();
    alone.cite(alone.goals()[0], LineId::new(1)).unwrap();
    assert_eq!(drawn(&alone).0, "1 │ a\n  ├───\n");
    // ⊢ a → a with ⊤ derived in the subproof: the hypothesis first, and
    // the →I citing the whole subproof.
    let mut beside = Interactive::within(nj.clone(), "a -> a", &[], &limits).unwrap();
    let body = beside.apply(beside.goals()[0], &Apply::new("→I")).unwrap();
    beside.derive(body[0], &Derive::new("⊤I")).unwrap();
    beside.cite(body[0], LineId::new(1)).unwrap();
    let (text, _) = drawn(&beside);
    assert!(text.contains("→I 1–2"), "{text}");
    // Two equal assumptions are the one line 1.
    let twice = Interactive::within(nj.clone(), "a /\\ a", &["a", "a"], &limits).unwrap();
    assert_eq!(drawn(&twice).1, [1, 1]);
    // ⊤ derived in the main proof before the subproof's first line stands
    // above it, and the numbers follow the drawing, not the steps.
    let mut order = Interactive::within(nj.clone(), "(a -> a) /\\ ⊤", &[], &limits).unwrap();
    let parts = order.apply(order.goals()[0], &Apply::new("∧I")).unwrap();
    order.apply(parts[0], &Apply::new("→I")).unwrap();
    order.derive(parts[1], &Derive::new("⊤I")).unwrap();
    let (text, numbers) = drawn(&order);
    assert!(text.starts_with("1 │ ⊤"), "{text}");
    assert_eq!(numbers, [2, 1], "{text}");
}

#[cfg(feature = "interactive")]
#[test]
fn a_session_builds_a_closed_tableau() {
    use deduxit::rules::{Apply, Interactive};
    let limits = Limits::default();
    let tableau = calculus(deduxit::rules::TABLEAU).unwrap();
    let mut session = Interactive::within(tableau, "T (a /\\ b), F a", &[], &limits).unwrap();
    let branch = session
        .apply(session.goals()[0], &Apply::new("T∧").at(&[0]))
        .unwrap();
    // The branch keeps the expanded formula: T (a ∧ b), F a, T a, T b.
    assert_eq!(session.goal(branch[0])[0].len(), 4);
    let members = session.members(branch[0]);
    let shown = |n: usize| {
        session
            .calculus()
            .show(session.terms(), members[n].1)
            .to_string()
    };
    let t = (0..4).find(|&n| shown(n) == "T a").unwrap() as u32;
    let f = (0..4).find(|&n| shown(n) == "F a").unwrap() as u32;
    session
        .apply(branch[0], &Apply::new("×").at(&[t, f]))
        .unwrap();
    session.finished(|_| false).unwrap();
}

#[cfg(feature = "interactive")]
#[test]
fn no_step_leaves_a_state_the_checker_refuses() {
    use deduxit::rules::{Apply, Interactive};
    let limits = Limits::default();
    // A refused step leaves no goal behind: LM's right zone holds one
    // formula, and a split that empties it is refused whole.
    let mut lm = Interactive::within(
        calculus(LM).unwrap(),
        "a \\/ b, ~a |- b \\/ a",
        &[],
        &limits,
    )
    .unwrap();
    let split = Apply::new("∨L split")
        .at(&[0])
        .given(&[("Γ′", ""), ("Δ′", "")]);
    assert!(matches!(
        lm.apply(lm.goals()[0], &split),
        Err(Error::StepRefused { .. })
    ));
    assert_eq!(lm.goals().len(), 1);
    let saved = serde_json::to_value(&lm).unwrap();
    wire::upgrade::<Interactive, _>(&saved, &limits).unwrap();
    // A name given as new that a later hypothesis holds is refused at
    // that hypothesis, so the session never finishes into a refusal.
    let fresh = calculus(
        r#"{"version": 15, "name": "F",
            "connectives": [{"name": "→", "arity": 2}, {"name": "□", "arity": 1}],
            "sequent": [{"name": "formula", "discipline": "multiset", "least": 1, "most": 1}],
            "variables": {"formula": ["A", "P"]},
            "rules": [{"name": "gen", "premises": [{"formula": [["→", "P", "A"]]}],
                       "conclusion": {"formula": [["□", "A"]]}, "fresh": ["P"]}]}"#,
    )
    .unwrap();
    let mut session = Interactive::within(fresh, "□ a", &["p -> a"], &limits).unwrap();
    let goal = session
        .apply(session.goals()[0], &Apply::new("gen").given(&[("P", "p")]))
        .unwrap();
    assert!(matches!(
        session.assume(goal[0]),
        Err(Error::StepRefused { .. })
    ));
    assert_eq!(session.goals(), goal);
    // A member found by position in a sequence stands where it was
    // built, not at the first equal member.
    let ordered = calculus(
        r#"{"version": 15, "name": "O", "connectives": [{"name": "•", "arity": 2}],
            "sequent": [{"name": "left", "discipline": "sequence"},
                        {"name": "right", "discipline": "multiset", "mark": "⊢", "least": 1, "most": 1}],
            "variables": {"formula": ["A", "B", "C"]}, "contexts": ["Δ", "Δ′"],
            "rules": [{"name": "•L", "premises": [{"left": ["Δ", "A", "B", "Δ′"], "right": ["C"]}],
                       "conclusion": {"left": ["Δ", ["•", "A", "B"], "Δ′"], "right": ["C"]}}]}"#,
    )
    .unwrap();
    let mut product = Interactive::within(ordered, "a, a • b |- c", &[], &limits).unwrap();
    product
        .apply(product.goals()[0], &Apply::new("•L").at(&[1]))
        .unwrap();
}

#[cfg(feature = "interactive")]
#[test]
fn a_session_document_whose_scopes_do_not_fit_is_refused() {
    use deduxit::rules::Interactive;
    let mut nj: Value = serde_json::from_str(NJ).unwrap();
    nj.as_object_mut().unwrap().remove("version");
    let terms = json!(["a", ["→", 0, 0], ["∧", 1, 1]]);
    let refused = |scopes: Value, nodes: Value, history: Value| {
        let document = json!({"version": 17, "calculus": nj, "terms": terms,
            "scopes": scopes, "nodes": nodes, "history": history});
        matches!(
            wire::upgrade::<Interactive, _>(&document, &Limits::default()),
            Err(Error::InconsistentSession { .. })
        )
    };
    // A scope owned by an open goal.
    assert!(refused(
        json!([{}, {"parent": 0, "owner": [1, 0]}, {"parent": 0, "owner": [2, 0], "hypotheses": [[[0]]]}]),
        json!([{"sequent": [[2]], "rule": "∧I", "premises": [1, 2]}, {"sequent": [[1]]},
               {"sequent": [[1]], "rule": "→I", "premises": [3]}, {"sequent": [[0]], "scope": 2}]),
        json!([]),
    ));
    // A hypothesis in a sibling's scope.
    assert!(refused(
        json!([{}, {"parent": 0, "owner": [1, 0], "hypotheses": [[[0]]]},
               {"parent": 0, "owner": [2, 0], "hypotheses": [[[0]]]}]),
        json!([{"sequent": [[2]], "rule": "∧I", "premises": [1, 2]},
               {"sequent": [[1]], "rule": "→I", "premises": [3]},
               {"sequent": [[1]], "rule": "→I", "premises": [4]},
               {"sequent": [[0]], "hypothesis": true, "scope": 2, "opened": 1},
               {"sequent": [[0]], "scope": 2}]),
        json!([2]),
    ));
}

#[cfg(feature = "interactive")]
#[test]
fn a_session_derives_forwards_and_cites_its_lines() {
    use deduxit::rules::{Apply, Derive, Interactive, LineId, NJ, Undone};
    let limits = Limits::default();
    let nj = calculus(NJ).unwrap();
    // From the assumption a ∧ b (line 1), forwards to b ∧ a.
    let mut session = Interactive::within(nj.clone(), "b /\\ a", &["a /\\ b"], &limits).unwrap();
    let goal = session.goals()[0];
    let one = [LineId::new(1)];
    let b = session
        .derive(goal, &Derive::new("∧E₂").lines(&one))
        .unwrap();
    let a = session
        .derive(goal, &Derive::new("∧E₁").lines(&one))
        .unwrap();
    assert_eq!((b, a), (LineId::new(2), LineId::new(3)));
    let both = session
        .derive(goal, &Derive::new("∧I").lines(&[b, a]))
        .unwrap();
    assert_eq!(session.lines(goal).len(), 4);
    session.cite(goal, both).unwrap();
    let derivation = session.finished(|_| false).unwrap();
    assert_eq!(derivation.assumptions().count(), 2);
    // A session with derived lines is written at level 18 and reads back.
    let document = serde_json::to_value(&session).unwrap();
    assert_eq!(document["version"], 18);
    let mut read: Interactive = wire::upgrade(&document, &limits).unwrap();
    assert_eq!(serde_json::to_value(&read).unwrap(), document);
    // A forged document is refused: a goal closed by a node that is no
    // derived line, a line no longer marked as one, a goal closed by a
    // line in the history before the line's step.
    let nodes = document["nodes"].as_array().unwrap();
    let line = nodes.iter().rposition(|n| n["line"] == true).unwrap();
    let mut forged = document.clone();
    forged["nodes"][0]["cites"] = json!(1);
    assert!(wire::upgrade::<Interactive, _>(&forged, &limits).is_err());
    let mut forged = document.clone();
    forged["nodes"][line]
        .as_object_mut()
        .unwrap()
        .remove("line");
    assert!(wire::upgrade::<Interactive, _>(&forged, &limits).is_err());
    let mut forged = document.clone();
    let history = forged["history"].as_array_mut().unwrap();
    let last = history.pop().unwrap();
    history.insert(0, last);
    assert!(wire::upgrade::<Interactive, _>(&forged, &limits).is_err());
    assert_eq!(read.undo(), Some(Undone::Reopened(goal)));
    assert_eq!(read.undo(), Some(Undone::Removed(both)));
    assert_eq!(read.lines(goal).len(), 3);

    // What a forward step refuses: a rule that discharges, a line that
    // is no instance of its premise, a variable only the conclusion
    // holds, a wrong count of lines.
    let refused = |step: Result<LineId, Error>, words: &str| match step {
        Err(Error::StepRefused { fault, .. }) => {
            assert!(fault.to_string().contains(words), "{fault}");
        }
        other => panic!("expected a refusal about {words:?}, got {other:?}"),
    };
    refused(
        read.derive(goal, &Derive::new("→I").lines(&one)),
        "apply it backwards",
    );
    refused(
        read.derive(goal, &Derive::new("→E").lines(&[LineId::new(1), b])),
        "no instance of premise 1",
    );
    refused(
        read.derive(goal, &Derive::new("∧I").lines(&one)),
        "cites 1 lines",
    );
    assert!(matches!(
        read.derive(goal, &Derive::new("∨I₁").lines(&one)),
        Err(Error::StepNeeds { needs, .. }) if needs == ["B"]
    ));
    read.derive(goal, &Derive::new("∨I₁").lines(&one).given(&[("B", "c")]))
        .unwrap();

    // A line of a subproof is in force inside it and nowhere else.
    let mut nested = Interactive::within(nj, "(a -> a /\\ a) /\\ (a -> a)", &[], &limits).unwrap();
    let opened = nested.apply(nested.goals()[0], &Apply::new("∧I")).unwrap();
    let inner = nested.apply(opened[0], &Apply::new("→I")).unwrap()[0];
    let hypothesis = LineId::new(1);
    let line = nested
        .derive(inner, &Derive::new("∧I").lines(&[hypothesis, hypothesis]))
        .unwrap();
    nested.assume(inner).unwrap();
    let outer = opened[1];
    match nested.derive(outer, &Derive::new("∧E₁").lines(&[line])) {
        Err(Error::StepRefused { fault, .. }) => {
            assert!(
                matches!(*fault, deduxit::StepFault::NotInForce { .. }),
                "{fault}"
            );
        }
        other => panic!("a line of a closed subproof was cited: {other:?}"),
    }
    assert!(nested.cite(outer, hypothesis).is_err());
    let last = nested.apply(outer, &Apply::new("→I")).unwrap()[0];
    // Line 3 is the second subproof's hypothesis.
    nested.cite(last, LineId::new(3)).unwrap();
    nested.finished(|_| false).unwrap();
}

#[cfg(feature = "interactive")]
#[test]
fn random_forward_and_backward_steps_keep_a_checked_session() {
    use deduxit::rules::{Apply, Derive, Interactive, LineId, NJ, NK};
    let limits = Limits::default();
    let mut seed = 62_u64;
    let mut next = |below: usize| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % below as u64) as usize
    };
    let (mut steps, mut derived, mut refused) = (0, 0, 0);
    for (file, goal, assumptions) in [
        (NJ, "(a -> b) -> (b -> c) -> a -> c", &[][..]),
        (NJ, "b \\/ a", &["a \\/ b"][..]),
        (NJ, "~(a /\\ ~a)", &[][..]),
        (NK, "a", &["~~a"][..]),
        (NJ, "(a <-> b) -> b -> a", &[][..]),
        (
            NJ,
            "d /\\ b",
            &["a /\\ b", "a -> c", "c <-> d", "~d \\/ a"][..],
        ),
        (NK, "c \\/ ~c", &["a /\\ ~b", "a -> b \\/ c"][..]),
    ] {
        let rules: Vec<String> = calculus(file).unwrap().rules().map(str::to_owned).collect();
        for _ in 0..40 {
            let mut session =
                Interactive::within(calculus(file).unwrap(), goal, assumptions, &limits).unwrap();
            for _ in 0..30 {
                let goals = session.goals();
                if goals.is_empty() {
                    break;
                }
                let goal = goals[next(goals.len())];
                let in_force: Vec<LineId> = session
                    .lines(goal)
                    .into_iter()
                    .map(|(line, _)| line)
                    .collect();
                let pick = |next: &mut dyn FnMut(usize) -> usize| {
                    if in_force.is_empty() || next(4) == 0 {
                        // Now and then a line out of force, or none at all.
                        LineId::new(next(12) as u32)
                    } else {
                        in_force[next(in_force.len())]
                    }
                };
                let rule = &rules[next(rules.len())];
                let outcome = match next(9) {
                    0 | 1 => {
                        // Mostly a rule that applies as it is.
                        let fits: Vec<_> = session
                            .rules(goal)
                            .unwrap()
                            .into_iter()
                            .filter(|r| r.needs.is_empty())
                            .collect();
                        if fits.is_empty() || next(5) == 0 {
                            session.apply(goal, &Apply::new(rule)).map(|_| ())
                        } else {
                            let fit = &fits[next(fits.len())];
                            session
                                .apply(goal, &Apply::new(&fit.rule).at(&fit.at))
                                .map(|_| ())
                        }
                    }
                    2 | 6 | 7 => {
                        // A rule that applies forwards, with as many lines
                        // as it has premises, mostly; now and then any.
                        const FORWARD: [(&str, usize); 10] = [
                            ("∧E₁", 1),
                            ("∧E₂", 1),
                            ("∧I", 2),
                            ("→E", 2),
                            ("¬E", 2),
                            ("↔E₁", 2),
                            ("↔E₂", 2),
                            ("∨I₁", 1),
                            ("∨I₂", 1),
                            ("⊥E", 1),
                        ];
                        let (name, count) = if next(5) == 0 {
                            (rule.as_str(), next(3))
                        } else {
                            FORWARD[next(FORWARD.len())]
                        };
                        let lines: Vec<LineId> = (0..count).map(|_| pick(&mut next)).collect();
                        let given = [("A", "a"), ("B", "b")];
                        let given = &given[..next(3)];
                        let line =
                            session.derive(goal, &Derive::new(name).lines(&lines).given(given));
                        derived += usize::from(line.is_ok());
                        line.map(|_| ())
                    }
                    3 | 4 => session.assume(goal),
                    5 => session.cite(goal, pick(&mut next)),
                    _ => {
                        session.undo();
                        Ok(())
                    }
                };
                match outcome {
                    Ok(()) => steps += 1,
                    Err(Error::StepRefused { .. } | Error::StepNeeds { .. }) => refused += 1,
                    Err(other) => panic!("{other:?}"),
                }
                // Every state reads back, every step checked again.
                let document = serde_json::to_value(&session).unwrap();
                let read: Interactive =
                    wire::upgrade(&document, &limits).unwrap_or_else(|e| panic!("{e}: {document}"));
                assert_eq!(serde_json::to_value(&read).unwrap(), document);
            }
            if session.is_finished() {
                session.finished(|_| false).unwrap();
            }
        }
    }
    assert!(
        steps > 1000 && refused > 1000 && derived > 100,
        "{steps} steps, {refused} refused, {derived} lines derived"
    );
}

/// Seriality gives a new world that the world named sees, never a new
/// world that sees itself: `Ser` with one new label on both ends of its
/// edge is refused, and with a label of the conclusion at its start
/// passes.
#[test]
fn seriality_s_new_world_is_not_the_one_that_sees_it() {
    let calculus = deduxit::modal::calculus(deduxit::modal::System::D, &Limits::default()).unwrap();
    let derivation = |edge: [u32; 2]| {
        json!({"version": 15, "calculus": "G3D",
            "terms": [{"atom": "w0", "sort": "label"}, "p", [":", 0, 1],
                {"atom": "w1", "sort": "label"}, ["R", edge[0], edge[1]]],
            "nodes": [{"sequent": [[2, 4], [2]], "rule": "ax", "at": [0, 0]},
                {"sequent": [[2], [2]], "rule": "Ser", "at": [1], "premises": [0]}]})
    };
    check(&calculus, &derivation([0, 3])).unwrap();
    assert!(
        refused(&calculus, &derivation([3, 3]))
            .contains("a name the rule asks to be new is one another of its variables names")
    );
}

/// In every modal system's calculus the rules that read forwards are
/// exactly those that reason from hypotheses and add hypotheses: `L∧`,
/// `L□`, `L◇` and the system's frame rules. Each rule is tried at every
/// place `rules` lists on a goal where every rule applies, `R¬` and `R→`
/// among them on goals whose premise would add only before `⊢`.
#[cfg(feature = "interactive")]
#[test]
fn forward_steps_are_the_rules_that_add_hypotheses() {
    use deduxit::modal::{self, System};
    use deduxit::rules::{Apply, Interactive};
    use std::collections::BTreeSet;
    let limits = Limits::default();
    let goal = "w0 : p /\\ q, w0 : ~p, w0 : p \\/ q, w0 : p -> q, w0 : p <-> q, w0 : []p, \
                w0 : <>p, w0 R w1, w1 R w2 |- w0 : r /\\ s, w0 : ~t, w0 : r \\/ s, \
                w0 : p -> q, w0 : q, w0 : r <-> s, w0 : []s, w0 : <>s";
    for &system in System::ALL {
        let calculus = modal::calculus(system, &limits).unwrap();
        let mut expected: BTreeSet<String> = ["L∧", "L□", "L◇"].map(str::to_owned).into();
        for rule in calculus.rules() {
            if ["Ref", "Ser", "Sym", "Trans", "Eucl"].contains(&rule) {
                expected.insert(rule.to_owned());
            }
        }
        let mut session = Interactive::within(calculus, goal, &[], &limits).unwrap();
        let root = session.goals()[0];
        let mut forward = BTreeSet::new();
        for fit in session.rules(root).unwrap() {
            let given = [("x", "w0"), ("y", "w1")];
            let given = &given[..fit.needs.len().min(2)];
            if session
                .extend(root, &Apply::new(&fit.rule).at(&fit.at).given(given))
                .is_ok()
            {
                forward.insert(fit.rule.clone());
                session.undo();
            }
        }
        assert_eq!(forward, expected, "{system}");
    }
}

/// Random sessions in the labelled calculus of every modal system, with
/// backward steps, forward steps on the goal's members (`extend`), the
/// ASCII spellings `[]` and `<>`, refusals and undo: every state reads
/// back with every step checked again, and every finished state is
/// checked whole.
#[cfg(feature = "interactive")]
#[test]
fn random_labelled_steps_keep_a_checked_session() {
    use deduxit::modal::{self, System};
    use deduxit::rules::{Apply, Interactive};
    let limits = Limits::default();
    let mut seed = 66_u64;
    let mut next = |below: usize| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % below as u64) as usize
    };
    // The axioms K, D, T, B, 4 and 5, each valid in the systems that have
    // its condition, and a sequent of every connective.
    let sequents = [
        "w0 : [](p -> q), w0 : []p |- w0 : []q",
        "w0 : []p |- w0 : <>p",
        "w0 : []p |- w0 : p",
        "w0 : p |- w0 : []<>p",
        "w0 : []p |- w0 : [][]p",
        "w0 : <>p |- w0 : []<>p",
        "w0 : <>(p /\\ q), w0 : ~r \\/ s, w0 R w1 |- w1 : p <-> q, w0 : <>q",
    ];
    // Rules that read forwards, each with the members it acts on.
    const FORWARD: [(&str, usize); 8] = [
        ("L∧", 1),
        ("L□", 2),
        ("L◇", 1),
        ("Ref", 0),
        ("Ser", 0),
        ("Sym", 1),
        ("Trans", 2),
        ("Eucl", 2),
    ];
    let (mut steps, mut extended, mut refused, mut finished) = (0, 0, 0, 0);
    for &system in System::ALL {
        let rules: Vec<String> = modal::calculus(system, &limits)
            .unwrap()
            .rules()
            .map(str::to_owned)
            .collect();
        for text in sequents {
            for _ in 0..2 {
                let calculus = modal::calculus(system, &limits).unwrap();
                let mut session = Interactive::within(calculus, text, &[], &limits).unwrap();
                for _ in 0..20 {
                    let goals = session.goals();
                    if goals.is_empty() {
                        break;
                    }
                    let goal = goals[next(goals.len())];
                    let members = session.members(goal).len();
                    let fits = session.rules(goal).unwrap();
                    let pick = |next: &mut dyn FnMut(usize) -> usize, count: usize| {
                        // Now and then a number past the goal's members.
                        (0..count)
                            .map(|_| next(members + 1) as u32)
                            .collect::<Vec<u32>>()
                    };
                    let labels = ["w0", "w1", "y"];
                    let given = [("x", labels[next(3)]), ("y", labels[next(3)])];
                    let given = &given[..next(3)];
                    let outcome = match next(8) {
                        0..=2 => {
                            // A closing rule first where one applies, so
                            // that walks finish.
                            let closing: Vec<_> = fits
                                .iter()
                                .filter(|r| ["ax", "L⊥", "R⊤"].contains(&r.rule.as_str()))
                                .collect();
                            let fits: Vec<_> = if closing.is_empty() || next(4) == 0 {
                                fits.iter().filter(|r| r.needs.is_empty()).collect()
                            } else {
                                closing
                            };
                            if fits.is_empty() || next(5) == 0 {
                                let rule = &rules[next(rules.len())];
                                let count = next(3);
                                let at = pick(&mut next, count);
                                session.apply(goal, &Apply::new(rule).at(&at))
                            } else {
                                let fit = &fits[next(fits.len())];
                                session.apply(goal, &Apply::new(&fit.rule).at(&fit.at))
                            }
                            .map(|_| ())
                        }
                        3..=5 => {
                            // Mostly a rule that reads forwards where it
                            // applies, its label given where it needs one;
                            // now and then any rule on any members.
                            let forward: Vec<_> = fits
                                .iter()
                                .filter(|r| FORWARD.iter().any(|(name, _)| *name == r.rule))
                                .collect();
                            let (name, at) = if forward.is_empty() || next(4) == 0 {
                                let (name, count) = if next(3) == 0 {
                                    (rules[next(rules.len())].as_str(), next(3))
                                } else {
                                    FORWARD[next(FORWARD.len())]
                                };
                                (name, pick(&mut next, count))
                            } else {
                                let fit = forward[next(forward.len())];
                                (fit.rule.as_str(), fit.at.clone())
                            };
                            let step = Apply::new(name).at(&at).given(given);
                            let opened = session.extend(goal, &step);
                            extended += usize::from(opened.is_ok());
                            opened.map(|_| ())
                        }
                        6 => session.assume(goal),
                        _ => {
                            session.undo();
                            Ok(())
                        }
                    };
                    match outcome {
                        Ok(()) => steps += 1,
                        Err(Error::StepRefused { .. } | Error::StepNeeds { .. }) => refused += 1,
                        Err(other) => panic!("{system}, {text}: {other:?}"),
                    }
                    let document = serde_json::to_value(&session).unwrap();
                    let read: Interactive = wire::upgrade(&document, &limits)
                        .unwrap_or_else(|e| panic!("{e}: {document}"));
                    assert_eq!(serde_json::to_value(&read).unwrap(), document);
                }
                if session.is_finished() {
                    session.finished(|_| false).unwrap();
                    finished += 1;
                }
            }
        }
    }
    assert!(
        steps > 1500 && refused > 500 && extended > 300 && finished > 3,
        "{steps} steps, {refused} refused, {extended} forward, {finished} finished"
    );
}

/// A sequent's text is held to `limits.occurrences` as a whole: a
/// sequent of many short members passed when each member was bounded
/// alone.
#[test]
fn a_sequent_s_text_is_bounded_as_a_whole() {
    let lk = calculus(LK).unwrap();
    let mut terms = deduxit::rules::Terms::default();
    let text = "a, b, c, d ⊢ a";
    let bound = |n| Limits::default().with_occurrences(Some(n));
    // Nine tokens: five atoms, three commas and the turnstile.
    assert!(lk.parse_sequent(&mut terms, text, &bound(9)).is_ok());
    assert!(matches!(
        lk.parse_sequent(&mut terms, text, &bound(8)),
        Err(Error::Refused(Refusal::Occurrences {
            occurrences: 9,
            limit: 8,
            ..
        }))
    ));
}
