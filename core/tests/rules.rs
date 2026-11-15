// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Calculi given as data: the rule files of LK and LJ against the
//! ordinary checker, and the checker's parts on small calculi of each
//! kind it serves (natural deduction, a signed tableau, a labelled
//! modal calculus, an ordered calculus).

#![cfg(all(feature = "parse", feature = "serialize"))]

use deduxit::limits::Refusal;
use deduxit::ordinary::{self, Logic, Method, Verdict};
use deduxit::rules::{Calculus, Derivation, LJ, LK, LM};
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
    for (text, name, rules) in [(LK, "LK", 29), (LJ, "LJ", 34), (LM, "LM", 32)] {
        let read = calculus(text).unwrap();
        assert_eq!(read.name(), name);
        assert_eq!(read.rules().len(), rules);
        assert_eq!(read.zones().collect::<Vec<_>>(), ["left", "right"]);
        // The file is written back as it was read.
        let written = serde_json::to_value(&read).unwrap();
        assert_eq!(written, serde_json::from_str::<Value>(text).unwrap());
        assert_eq!(written["version"], wire::LEVEL);
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
fn the_rule_files_agree_with_the_ordinary_checker() {
    let limits = Limits::default();
    let mut methods = vec![Method::Embedding, Method::Ljt];
    #[cfg(feature = "sat")]
    methods.push(Method::Sat);
    #[cfg(feature = "interactive")]
    methods.push(Method::Tableau);
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
    assert!(reason(&apart).contains("told apart"));
    // A connective with another number of arguments than it takes.
    let mut arity = lk.clone();
    arity["rules"][0]["conclusion"]["left"] = json!([["∧", "A"]]);
    assert!(reason(&arity).contains("takes 2 arguments"));
    // A connective with more arguments than any takes.
    let mut wide = lk.clone();
    wide["connectives"][0]["arity"] = json!(u32::MAX);
    assert!(reason(&wide).contains("255 at most"));
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
