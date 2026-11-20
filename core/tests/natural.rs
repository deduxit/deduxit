// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Natural deduction's rule files against the ordinary suite. A
//! derivation in NJ (NK, NM, and their variants) exists exactly where the
//! ordinary suite says the sequent is valid in intuitionistic (classical,
//! minimal) logic:
//! - every rule of a file is valid in the file's logic, read as a
//!   sequent, and the rule that makes a file intuitionistic or classical
//!   is not valid in the weaker logic, so whatever the checker accepts is
//!   valid there;
//! - the ordinary suite's derivation of every valid sequent of a sample,
//!   translated into natural deduction by the converter below, passes the
//!   checker, concluding the sequent's formula from its hypotheses;
//! - a derivation broken at one node that the checker still accepts
//!   shows a sequent the ordinary suite calls valid.

#![cfg(all(feature = "parse", feature = "serialize"))]

use deduxit::ordinary::{self, Formulas, Logic, Method, NodeId, Rule, Side, Verdict};
use deduxit::rules::{self, Calculus, Derivation, Node, ScopeId, Step, Term, TermId};
use deduxit::{Limits, wire};
use serde_json::Value;
use std::collections::HashMap;

/// How a classical file reasons classically.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Classical {
    /// `RAA`: `⊥` from `¬A` gives `A`.
    Raa,
    /// `¬¬E`: `¬¬A` gives `A`.
    Dne,
    /// `LEM`: `A ∨ ¬A` from nothing.
    Lem,
}

/// A rule file of natural deduction and what it is.
struct File {
    /// The calculus's name.
    name: &'static str,
    /// The rule file.
    text: &'static str,
    /// The logic it is meant for.
    logic: Logic,
    /// Its classical rule, if it has one.
    classical: Option<Classical>,
    /// Whether `¬A` is written `A → ⊥`.
    defined: bool,
}

/// Every natural deduction file the crate ships.
const FILES: [File; 8] = [
    File {
        name: "NJ",
        text: rules::NJ,
        logic: Logic::Intuitionistic,
        classical: None,
        defined: false,
    },
    File {
        name: "NK",
        text: rules::NK,
        logic: Logic::Classical,
        classical: Some(Classical::Raa),
        defined: false,
    },
    File {
        name: "NM",
        text: rules::NM,
        logic: Logic::Minimal,
        classical: None,
        defined: false,
    },
    File {
        name: "NK-DNE",
        text: rules::NK_DNE,
        logic: Logic::Classical,
        classical: Some(Classical::Dne),
        defined: false,
    },
    File {
        name: "NK-LEM",
        text: rules::NK_LEM,
        logic: Logic::Classical,
        classical: Some(Classical::Lem),
        defined: false,
    },
    File {
        name: "NJ-defined",
        text: rules::NJ_DEFINED,
        logic: Logic::Intuitionistic,
        classical: None,
        defined: true,
    },
    File {
        name: "NK-defined",
        text: rules::NK_DEFINED,
        logic: Logic::Classical,
        classical: Some(Classical::Raa),
        defined: true,
    },
    File {
        name: "NM-defined",
        text: rules::NM_DEFINED,
        logic: Logic::Minimal,
        classical: None,
        defined: true,
    },
];

/// The strength of a logic: what it proves includes what a weaker one
/// proves.
const fn strength(logic: Logic) -> u8 {
    match logic {
        Logic::Minimal => 0,
        Logic::Intuitionistic => 1,
        _ => 2,
    }
}

/// Reads a rule file.
fn calculus(text: &str) -> Calculus {
    let mut document = serde_json::Deserializer::from_str(text);
    wire::upgrade(&mut document, &Limits::default()).unwrap()
}

/// Whether the ordinary suite decides `sequent` valid in `logic`.
fn valid(sequent: &ordinary::Sequent, logic: Logic) -> bool {
    let options = ordinary::Options::default().with_logic(logic);
    match ordinary::decide(sequent, &options).unwrap().verdict {
        Verdict::Valid(_) => true,
        Verdict::NotValid => false,
        other => panic!("{sequent} in {logic}: {other:?}"),
    }
}

/// A rule file's pattern in the ordinary syntax, its variables as atoms.
fn pattern(value: &Value) -> String {
    match value {
        Value::String(name) => match name.as_str() {
            "⊤" => "true".to_owned(),
            "⊥" => "false".to_owned(),
            variable => variable.to_lowercase(),
        },
        Value::Array(parts) => {
            let operands: Vec<String> = parts[1..].iter().map(pattern).collect();
            match (parts[0].as_str().unwrap(), &operands[..]) {
                ("¬", [a]) => format!("~({a})"),
                ("∧", [a, b]) => format!("({a}) /\\ ({b})"),
                ("∨", [a, b]) => format!("({a}) \\/ ({b})"),
                ("→", [a, b]) => format!("({a}) -> ({b})"),
                ("↔", [a, b]) => format!("({a}) <-> ({b})"),
                other => panic!("no connective of natural deduction: {other:?}"),
            }
        }
        other => panic!("no pattern: {other}"),
    }
}

/// The logic a rule needs to be valid: classical for the classical
/// rules, intuitionistic for ex falso, minimal for the rest.
fn needs(rule: &str) -> Logic {
    match rule {
        "RAA" | "¬¬E" | "LEM" => Logic::Classical,
        "⊥E" => Logic::Intuitionistic,
        _ => Logic::Minimal,
    }
}

#[test]
fn every_rule_is_valid_in_its_logic_and_no_weaker_one() {
    for file in &FILES {
        let read = calculus(file.text);
        assert_eq!(read.name(), file.name);
        // No rule of natural deduction needs a position.
        assert!(read.rules().all(|rule| read.positions(rule) == Some(0)));
        let document: Value = serde_json::from_str(file.text).unwrap();
        for rule in document["rules"].as_array().unwrap() {
            let name = rule["name"].as_str().unwrap();
            // Each premise is an implication from the hypotheses it
            // discharges.
            let mut hypotheses = Vec::new();
            for premise in rule["premises"].as_array().into_iter().flatten() {
                let formula = pattern(&premise["formula"][0]);
                let discharged: Vec<String> = premise["discharges"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .map(|h| format!("({})", pattern(&h["formula"][0])))
                    .collect();
                hypotheses.push(if discharged.is_empty() {
                    format!("({formula})")
                } else {
                    format!("(({}) -> ({formula}))", discharged.join(" /\\ "))
                });
            }
            let conclusion = pattern(&rule["conclusion"]["formula"][0]);
            let text = format!("{} |- {conclusion}", hypotheses.join(", "));
            let sequent: ordinary::Sequent = text.parse().unwrap();
            assert!(
                strength(needs(name)) <= strength(file.logic),
                "{name} in {}",
                file.name
            );
            for logic in [Logic::Minimal, Logic::Intuitionistic, Logic::Classical] {
                assert_eq!(
                    valid(&sequent, logic),
                    strength(needs(name)) <= strength(logic),
                    "{name} of {} as {text} in {logic}",
                    file.name
                );
            }
        }
        // The file's logic is reached: an intuitionistic file has ex
        // falso, a classical one its classical rule.
        let names: Vec<&str> = read.rules().collect();
        assert_eq!(
            names.contains(&"⊥E"),
            file.logic != Logic::Minimal,
            "{}",
            file.name
        );
        assert_eq!(
            names.iter().any(|&name| needs(name) == Logic::Classical),
            file.logic == Logic::Classical,
            "{}",
            file.name
        );
    }
}

/// A node of a natural deduction tree under construction.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Tree {
    /// A hypothesis: open, or discharged by the rule that took the label.
    Hypothesis(NodeId, Option<u32>),
    /// A rule's conclusion and its premises, each with the label of the
    /// hypotheses it discharges.
    Rule(&'static str, NodeId, Vec<(usize, Option<u32>)>),
}

/// The most nodes a translated derivation may unfold to.
const MOST_NODES: u64 = 50_000;

/// Translates a derivation of LK, LJ or LM into a natural deduction file.
///
/// The hypotheses of the sequent concluded are open leaves, named by
/// formula: an open leaf may be replaced by any derivation of its formula
/// from the conclusion's hypotheses, so no occurrence needs tracking.
/// A derivation of LJ or LM concludes the formula right of `⊢` (or `⊥`);
/// one of LK concludes `⊥` with `¬B` open for every `B` right of `⊢`.
struct Translator<'a> {
    /// The file.
    file: &'a File,
    /// The source derivation.
    source: &'a ordinary::Derivation,
    /// The formulas of natural deduction.
    formulas: Formulas,
    /// Each source formula's formula here.
    mapped: HashMap<NodeId, NodeId>,
    /// The trees, shared.
    trees: Vec<Tree>,
    /// The labels given so far.
    labels: u32,
}

impl<'a> Translator<'a> {
    /// A translator of `source` into `file`.
    fn new(file: &'a File, source: &'a ordinary::Derivation) -> Self {
        Self {
            file,
            source,
            formulas: Formulas::default(),
            mapped: HashMap::new(),
            trees: Vec::new(),
            labels: 0,
        }
    }

    /// Adds a formula.
    fn add(&mut self, node: ordinary::Node) -> NodeId {
        self.formulas.add(node).unwrap()
    }

    /// `⊥`.
    fn falsum(&mut self) -> NodeId {
        self.add(ordinary::Node::False)
    }

    /// `¬a`, or `a → ⊥` where negation is defined.
    fn not(&mut self, a: NodeId) -> NodeId {
        if self.file.defined {
            let falsum = self.falsum();
            self.add(ordinary::Node::Implies(a, falsum))
        } else {
            self.add(ordinary::Node::Not(a))
        }
    }

    /// A source formula's formula here.
    fn formula(&mut self, id: NodeId) -> NodeId {
        if let Some(&done) = self.mapped.get(&id) {
            return done;
        }
        let node = match self.source.formulas().node(id) {
            ordinary::Node::Atom(atom) => {
                let name = self.source.formulas().atom_name(atom).to_owned();
                let mapped = self.formulas.atom(&name).unwrap();
                self.mapped.insert(id, mapped);
                return mapped;
            }
            ordinary::Node::Not(a) => {
                let a = self.formula(a);
                let mapped = self.not(a);
                self.mapped.insert(id, mapped);
                return mapped;
            }
            ordinary::Node::Forall(_) | ordinary::Node::Exists(_) => {
                unreachable!("the samples are propositional")
            }
            ordinary::Node::True => ordinary::Node::True,
            ordinary::Node::False => ordinary::Node::False,
            ordinary::Node::And(a, b) => ordinary::Node::And(self.formula(a), self.formula(b)),
            ordinary::Node::Or(a, b) => ordinary::Node::Or(self.formula(a), self.formula(b)),
            ordinary::Node::Implies(a, b) => {
                ordinary::Node::Implies(self.formula(a), self.formula(b))
            }
            ordinary::Node::Iff(a, b) => ordinary::Node::Iff(self.formula(a), self.formula(b)),
        };
        let mapped = self.add(node);
        self.mapped.insert(id, mapped);
        mapped
    }

    /// The operands of a formula here.
    fn operands(&self, id: NodeId) -> (NodeId, NodeId) {
        match self.formulas.node(id) {
            ordinary::Node::And(a, b)
            | ordinary::Node::Or(a, b)
            | ordinary::Node::Implies(a, b)
            | ordinary::Node::Iff(a, b) => (a, b),
            ordinary::Node::Not(a) => (a, a),
            other => panic!("no operands: {other:?}"),
        }
    }

    /// Adds a tree node.
    fn push(&mut self, node: Tree) -> usize {
        self.trees.push(node);
        self.trees.len() - 1
    }

    /// An open hypothesis.
    fn hypothesis(&mut self, formula: NodeId) -> usize {
        self.push(Tree::Hypothesis(formula, None))
    }

    /// A rule's conclusion over premises that discharge nothing.
    fn rule(&mut self, name: &'static str, formula: NodeId, premises: &[usize]) -> usize {
        let premises = premises.iter().map(|&p| (p, None)).collect();
        self.push(Tree::Rule(name, formula, premises))
    }

    /// The formula a tree concludes.
    fn conclusion(&self, tree: usize) -> NodeId {
        match self.trees[tree] {
            Tree::Hypothesis(formula, _) | Tree::Rule(_, formula, _) => formula,
        }
    }

    /// `tree` with every open leaf of `formula` replaced by `with`.
    fn replace(
        &mut self,
        tree: usize,
        formula: NodeId,
        with: usize,
        memo: &mut HashMap<usize, usize>,
    ) -> usize {
        if let Some(&done) = memo.get(&tree) {
            return done;
        }
        let replaced = match self.trees[tree].clone() {
            Tree::Hypothesis(leaf, None) if leaf == formula => with,
            Tree::Hypothesis(..) => tree,
            Tree::Rule(name, conclusion, premises) => {
                let new: Vec<_> = premises
                    .iter()
                    .map(|&(premise, label)| (self.replace(premise, formula, with, memo), label))
                    .collect();
                if new == premises {
                    tree
                } else {
                    self.push(Tree::Rule(name, conclusion, new))
                }
            }
        };
        memo.insert(tree, replaced);
        replaced
    }

    /// `tree` with every open leaf of `formula` replaced by `with`, a
    /// derivation of it.
    fn substitute(&mut self, tree: usize, formula: NodeId, with: usize) -> usize {
        assert_eq!(self.conclusion(with), formula);
        self.replace(tree, formula, with, &mut HashMap::new())
    }

    /// `tree` as a premise that discharges its open leaves of `formula`.
    fn bind(&mut self, tree: usize, formula: NodeId) -> (usize, Option<u32>) {
        self.labels += 1;
        let label = self.labels;
        let leaf = self.push(Tree::Hypothesis(formula, Some(label)));
        let bound = self.replace(tree, formula, leaf, &mut HashMap::new());
        (bound, Some(label))
    }

    /// `¬a` from `tree`, a derivation of `⊥` with `a` open.
    fn not_intro(&mut self, tree: usize, a: NodeId) -> usize {
        let not = self.not(a);
        let premise = self.bind(tree, a);
        let name = if self.file.defined { "→I" } else { "¬I" };
        self.push(Tree::Rule(name, not, vec![premise]))
    }

    /// `⊥` from a derivation of `¬a` and one of `a`.
    fn not_elim(&mut self, not: usize, a: usize) -> usize {
        let name = if self.file.defined { "→E" } else { "¬E" };
        let falsum = self.falsum();
        self.rule(name, falsum, &[not, a])
    }

    /// `goal` from `tree`, a derivation of `⊥`.
    fn absurd(&mut self, tree: usize, goal: NodeId) -> usize {
        if goal == self.falsum() {
            tree
        } else {
            self.rule("⊥E", goal, &[tree])
        }
    }

    /// `a` from `tree`, a derivation of `⊥` with `¬a` open, by the
    /// file's classical rule.
    fn classical(&mut self, tree: usize, a: NodeId) -> usize {
        let not = self.not(a);
        match self.file.classical.unwrap() {
            Classical::Raa => {
                let premise = self.bind(tree, not);
                self.push(Tree::Rule("RAA", a, vec![premise]))
            }
            Classical::Dne => {
                let twice = self.not_intro(tree, not);
                self.rule("¬¬E", a, &[twice])
            }
            Classical::Lem => {
                let either = self.add(ordinary::Node::Or(a, not));
                let lem = self.rule("LEM", either, &[]);
                let leaf = self.hypothesis(a);
                let first = self.bind(leaf, a);
                let absurd = self.absurd(tree, a);
                let second = self.bind(absurd, not);
                self.push(Tree::Rule("∨E", a, vec![(lem, None), first, second]))
            }
        }
    }

    /// The source formula on a side of an inference.
    fn at(&mut self, inference: &ordinary::Inference, (side, at): (Side, usize)) -> NodeId {
        let id = match side {
            Side::Left => inference.left()[at],
            Side::Right => inference.right()[at],
        };
        self.formula(id)
    }

    /// `tree` with the open leaves of a conjunction's parts replaced.
    fn and_left(&mut self, tree: usize, rule: Rule, conjunction: NodeId) -> usize {
        let (a, b) = self.operands(conjunction);
        let mut tree = tree;
        if rule != Rule::AndLeft2 {
            let leaf = self.hypothesis(conjunction);
            let first = self.rule("∧E₁", a, &[leaf]);
            tree = self.substitute(tree, a, first);
        }
        if rule != Rule::AndLeft1 {
            let leaf = self.hypothesis(conjunction);
            let second = self.rule("∧E₂", b, &[leaf]);
            tree = self.substitute(tree, b, second);
        }
        tree
    }

    /// `tree` with the open leaves of an equivalence's implications
    /// replaced.
    fn iff_left(&mut self, tree: usize, rule: Rule, iff: NodeId) -> usize {
        let (a, b) = self.operands(iff);
        let mut tree = tree;
        for (from, to, name, used) in [
            (a, b, "↔E₁", rule != Rule::IffLeft2),
            (b, a, "↔E₂", rule != Rule::IffLeft1),
        ] {
            if !used {
                continue;
            }
            let implication = self.add(ordinary::Node::Implies(from, to));
            let major = self.hypothesis(iff);
            let minor = self.hypothesis(from);
            let step = self.rule(name, to, &[major, minor]);
            let premise = self.bind(step, from);
            let derived = self.push(Tree::Rule("→I", implication, vec![premise]));
            tree = self.substitute(tree, implication, derived);
        }
        tree
    }

    /// `a ↔ b` from derivations of `a → b` and `b → a`.
    fn iff_right(&mut self, forward: usize, backward: usize, iff: NodeId) -> usize {
        let (a, b) = self.operands(iff);
        let first = {
            let leaf = self.hypothesis(a);
            let step = self.rule("→E", b, &[forward, leaf]);
            self.bind(step, a)
        };
        let second = {
            let leaf = self.hypothesis(b);
            let step = self.rule("→E", a, &[backward, leaf]);
            self.bind(step, b)
        };
        self.push(Tree::Rule("↔I", iff, vec![first, second]))
    }

    /// The tree of an inference of LJ or LM, given its premises' trees.
    fn intuitionistic(&mut self, inference: &ordinary::Inference, trees: &[usize]) -> usize {
        let goal = match inference.right().first() {
            Some(&right) => self.formula(right),
            None => self.falsum(),
        };
        let principal = inference.principal().map(|place| self.at(inference, place));
        let tree = match inference.rule() {
            Rule::Axiom => self.hypothesis(goal),
            Rule::WeakenLeft | Rule::ContractLeft | Rule::TrueLeft | Rule::FalseRight => trees[0],
            Rule::WeakenRight => self.absurd(trees[0], goal),
            Rule::FalseLeft => {
                let falsum = self.falsum();
                let leaf = self.hypothesis(falsum);
                self.absurd(leaf, goal)
            }
            Rule::TrueRight => self.rule("⊤I", goal, &[]),
            rule @ (Rule::AndLeft | Rule::AndLeft1 | Rule::AndLeft2) => {
                self.and_left(trees[0], rule, principal.unwrap())
            }
            Rule::AndRight => self.rule("∧I", goal, trees),
            Rule::OrLeft => {
                let p = principal.unwrap();
                let (a, b) = self.operands(p);
                let major = self.hypothesis(p);
                let first = self.bind(trees[0], a);
                let second = self.bind(trees[1], b);
                self.push(Tree::Rule("∨E", goal, vec![(major, None), first, second]))
            }
            Rule::OrRight1 => self.rule("∨I₁", goal, &[trees[0]]),
            Rule::OrRight2 => self.rule("∨I₂", goal, &[trees[0]]),
            Rule::ImpliesLeft => {
                let p = principal.unwrap();
                let (_, b) = self.operands(p);
                let major = self.hypothesis(p);
                let step = self.rule("→E", b, &[major, trees[0]]);
                self.substitute(trees[1], b, step)
            }
            Rule::ImpliesRight => {
                let (a, _) = self.operands(goal);
                let premise = self.bind(trees[0], a);
                self.push(Tree::Rule("→I", goal, vec![premise]))
            }
            Rule::NotLeft => {
                let p = principal.unwrap();
                let major = self.hypothesis(p);
                let step = self.not_elim(major, trees[0]);
                if trees.len() == 1 {
                    step
                } else {
                    let falsum = self.falsum();
                    self.substitute(trees[1], falsum, step)
                }
            }
            Rule::NotRight => {
                let (a, _) = self.operands(goal);
                self.not_intro(trees[0], a)
            }
            rule @ (Rule::IffLeft | Rule::IffLeft1 | Rule::IffLeft2) => {
                self.iff_left(trees[0], rule, principal.unwrap())
            }
            Rule::IffRight => self.iff_right(trees[0], trees[1], goal),
            Rule::Cut => {
                let first = self.source.inference(inference.premises()[0]);
                let cut = self.formula(first.right()[0]);
                self.substitute(trees[1], cut, trees[0])
            }
            other => panic!("no rule of LJ: {other:?}"),
        };
        assert_eq!(self.conclusion(tree), goal, "{:?}", inference.rule());
        tree
    }

    /// The tree of an inference of LK, a derivation of `⊥`, given its
    /// premises' trees.
    fn classical_inference(&mut self, inference: &ordinary::Inference, trees: &[usize]) -> usize {
        let principal = inference.principal().map(|place| self.at(inference, place));
        let tree = match inference.rule() {
            Rule::Axiom => {
                let a = self.formula(inference.right()[0]);
                let not = self.not(a);
                let major = self.hypothesis(not);
                let minor = self.hypothesis(a);
                self.not_elim(major, minor)
            }
            Rule::WeakenLeft
            | Rule::WeakenRight
            | Rule::ContractLeft
            | Rule::ContractRight
            | Rule::TrueLeft
            | Rule::FalseRight
            | Rule::NotLeft => trees[0],
            Rule::FalseLeft => {
                let falsum = self.falsum();
                self.hypothesis(falsum)
            }
            Rule::TrueRight => {
                let truth = self.add(ordinary::Node::True);
                let not = self.not(truth);
                let major = self.hypothesis(not);
                let minor = self.rule("⊤I", truth, &[]);
                self.not_elim(major, minor)
            }
            rule @ (Rule::AndLeft | Rule::AndLeft1 | Rule::AndLeft2) => {
                self.and_left(trees[0], rule, principal.unwrap())
            }
            Rule::AndRight => {
                let p = principal.unwrap();
                let (a, b) = self.operands(p);
                let a = self.classical(trees[0], a);
                let b = self.classical(trees[1], b);
                let both = self.rule("∧I", p, &[a, b]);
                self.refute(p, both)
            }
            Rule::OrLeft => {
                let p = principal.unwrap();
                let (a, b) = self.operands(p);
                let major = self.hypothesis(p);
                let first = self.bind(trees[0], a);
                let second = self.bind(trees[1], b);
                let falsum = self.falsum();
                self.push(Tree::Rule("∨E", falsum, vec![(major, None), first, second]))
            }
            rule @ (Rule::OrRight | Rule::OrRight1 | Rule::OrRight2) => {
                let p = principal.unwrap();
                let (a, b) = self.operands(p);
                let mut tree = trees[0];
                for (part, name, used) in [
                    (a, "∨I₁", rule != Rule::OrRight2),
                    (b, "∨I₂", rule != Rule::OrRight1),
                ] {
                    if used {
                        let leaf = self.hypothesis(part);
                        let either = self.rule(name, p, &[leaf]);
                        let refuted = self.refute(p, either);
                        let not = self.not_intro(refuted, part);
                        let negated = self.not(part);
                        tree = self.substitute(tree, negated, not);
                    }
                }
                tree
            }
            Rule::ImpliesLeft => {
                let p = principal.unwrap();
                let (a, b) = self.operands(p);
                let a = self.classical(trees[0], a);
                let major = self.hypothesis(p);
                let step = self.rule("→E", b, &[major, a]);
                self.substitute(trees[1], b, step)
            }
            Rule::ImpliesRight => {
                let p = principal.unwrap();
                let (a, b) = self.operands(p);
                let b = self.classical(trees[0], b);
                let premise = self.bind(b, a);
                let implication = self.push(Tree::Rule("→I", p, vec![premise]));
                self.refute(p, implication)
            }
            Rule::NotRight => {
                let p = principal.unwrap();
                let (a, _) = self.operands(p);
                let not = self.not_intro(trees[0], a);
                self.refute(p, not)
            }
            rule @ (Rule::IffLeft | Rule::IffLeft1 | Rule::IffLeft2) => {
                self.iff_left(trees[0], rule, principal.unwrap())
            }
            Rule::IffRight => {
                let p = principal.unwrap();
                let (a, b) = self.operands(p);
                let forward = self.add(ordinary::Node::Implies(a, b));
                let backward = self.add(ordinary::Node::Implies(b, a));
                let forward = self.classical(trees[0], forward);
                let backward = self.classical(trees[1], backward);
                let iff = self.iff_right(forward, backward, p);
                self.refute(p, iff)
            }
            other => panic!("no rule of LK: {other:?}"),
        };
        assert_eq!(
            self.conclusion(tree),
            self.falsum(),
            "{:?}",
            inference.rule()
        );
        tree
    }

    /// `⊥` from a derivation of `a`, with `¬a` open.
    fn refute(&mut self, a: NodeId, tree: usize) -> usize {
        let not = self.not(a);
        let major = self.hypothesis(not);
        self.not_elim(major, tree)
    }

    /// The whole derivation as a tree, and the formula it concludes.
    fn translate(&mut self) -> usize {
        let classical = self.source.logic() == Logic::Classical;
        let mut trees: Vec<usize> = Vec::new();
        for inference in self.source.inferences() {
            let premises: Vec<usize> = inference
                .premises()
                .iter()
                .map(|&p| trees[p.index()])
                .collect();
            let tree = if classical {
                self.classical_inference(inference, &premises)
            } else {
                self.intuitionistic(inference, &premises)
            };
            trees.push(tree);
        }
        let mut tree = *trees.last().unwrap();
        if classical {
            let (_, right) = self.source.sequent();
            let right: Vec<NodeId> = right.iter().map(|&r| self.formula(r)).collect();
            match right[..] {
                [] => {}
                [one] => tree = self.classical(tree, one),
                [first, ref rest @ ..] => {
                    // Several formulas right of `⊢` are their disjunction,
                    // the first leftmost.
                    let mut disjunction = first;
                    for &next in rest {
                        disjunction = self.add(ordinary::Node::Or(disjunction, next));
                    }
                    for (i, &part) in right.iter().enumerate() {
                        let leaf = self.hypothesis(part);
                        let mut injected = leaf;
                        let mut prefix = right[0];
                        if i > 0 {
                            for &next in &right[1..i] {
                                prefix = self.add(ordinary::Node::Or(prefix, next));
                            }
                            prefix = self.add(ordinary::Node::Or(prefix, part));
                            injected = self.rule("∨I₂", prefix, &[injected]);
                        }
                        for &next in &right[i.max(1)..] {
                            if prefix == disjunction {
                                break;
                            }
                            prefix = self.add(ordinary::Node::Or(prefix, next));
                            injected = self.rule("∨I₁", prefix, &[injected]);
                        }
                        let refuted = self.refute(disjunction, injected);
                        let not = self.not_intro(refuted, part);
                        let negated = self.not(part);
                        tree = self.substitute(tree, negated, not);
                    }
                    tree = self.classical(tree, disjunction);
                }
            }
        }
        tree
    }

    /// The number of nodes `tree` unfolds to, saturating.
    fn unfolded(&self, tree: usize, memo: &mut HashMap<usize, u64>) -> u64 {
        if let Some(&n) = memo.get(&tree) {
            return n;
        }
        let n = match &self.trees[tree] {
            Tree::Hypothesis(..) => 1,
            Tree::Rule(_, _, premises) => premises.iter().fold(1_u64, |sum, &(p, _)| {
                sum.saturating_add(self.unfolded(p, memo))
            }),
        };
        memo.insert(tree, n);
        n
    }

    /// The tree as a derivation, each discharging premise in a scope of
    /// its own and each discharged leaf in its binder's scope; `None`
    /// past [`MOST_NODES`].
    fn derivation(&self, tree: usize) -> Option<Derivation> {
        if self.unfolded(tree, &mut HashMap::new()) > MOST_NODES {
            return None;
        }
        let mut emitter = Emitter {
            translator: self,
            derivation: Derivation::new(self.file.name, 1),
            terms: HashMap::new(),
            scopes: HashMap::new(),
        };
        emitter.emit(tree, ScopeId::ROOT);
        Some(emitter.derivation)
    }
}

/// Writes a tree into a derivation.
struct Emitter<'t, 'a> {
    /// The translator whose tree it writes.
    translator: &'t Translator<'a>,
    /// The derivation written.
    derivation: Derivation,
    /// Each formula's term.
    terms: HashMap<NodeId, TermId>,
    /// The scope of each label, for the subtree being written.
    scopes: HashMap<u32, ScopeId>,
}

impl Emitter<'_, '_> {
    /// A formula's term.
    fn term(&mut self, id: NodeId) -> TermId {
        if let Some(&term) = self.terms.get(&id) {
            return term;
        }
        let formulas = &self.translator.formulas;
        let (connective, operands) = match formulas.node(id) {
            ordinary::Node::Atom(atom) => {
                let name = formulas.atom_name(atom).to_owned();
                let term = self
                    .derivation
                    .terms_mut()
                    .atom(&name, rules::FORMULA)
                    .unwrap();
                self.terms.insert(id, term);
                return term;
            }
            ordinary::Node::Forall(_) | ordinary::Node::Exists(_) => {
                unreachable!("the samples are propositional")
            }
            ordinary::Node::True => ("⊤", vec![]),
            ordinary::Node::False => ("⊥", vec![]),
            ordinary::Node::Not(a) => ("¬", vec![a]),
            ordinary::Node::And(a, b) => ("∧", vec![a, b]),
            ordinary::Node::Or(a, b) => ("∨", vec![a, b]),
            ordinary::Node::Implies(a, b) => ("→", vec![a, b]),
            ordinary::Node::Iff(a, b) => ("↔", vec![a, b]),
        };
        let operands: Vec<TermId> = operands.into_iter().map(|o| self.term(o)).collect();
        let term = self
            .derivation
            .terms_mut()
            .apply(connective, &operands)
            .unwrap();
        self.terms.insert(id, term);
        term
    }

    /// Writes `tree` in `scope` and returns its node's index.
    fn emit(&mut self, tree: usize, scope: ScopeId) -> u32 {
        let node = match &self.translator.trees[tree] {
            &Tree::Hypothesis(formula, label) => {
                let at = label.map_or(ScopeId::ROOT, |label| self.scopes[&label]);
                let sequent = [vec![self.term(formula)]];
                self.derivation
                    .push(Node::new(&sequent, Step::Hypothesis).scope(at))
                    .unwrap()
            }
            Tree::Rule(name, formula, premises) => {
                let mut indices = Vec::new();
                for &(premise, label) in premises {
                    let inner = match label {
                        Some(label) => {
                            let inner = self.derivation.scope(scope.index() as u32).unwrap();
                            self.scopes.insert(label, inner);
                            inner
                        }
                        None => scope,
                    };
                    indices.push(self.emit(premise, inner));
                }
                let sequent = [vec![self.term(*formula)]];
                self.derivation
                    .push(
                        Node::new(&sequent, Step::Rule(name))
                            .premises(&indices)
                            .scope(scope),
                    )
                    .unwrap()
            }
        };
        node.index() as u32
    }
}

/// A rule file's term as an ordinary formula.
fn ordinary_of(terms: &rules::Terms, id: TermId, formulas: &mut Formulas) -> NodeId {
    match terms.term(id) {
        Term::Atom { name, .. } => formulas.atom(name).unwrap(),
        Term::Apply {
            connective,
            arguments,
            ..
        } => {
            let operands: Vec<NodeId> = arguments
                .iter()
                .map(|&a| ordinary_of(terms, a, formulas))
                .collect();
            let node = match (connective, &operands[..]) {
                ("⊤", []) => ordinary::Node::True,
                ("⊥", []) => ordinary::Node::False,
                ("¬", &[a]) => ordinary::Node::Not(a),
                ("∧", &[a, b]) => ordinary::Node::And(a, b),
                ("∨", &[a, b]) => ordinary::Node::Or(a, b),
                ("→", &[a, b]) => ordinary::Node::Implies(a, b),
                ("↔", &[a, b]) => ordinary::Node::Iff(a, b),
                other => panic!("no connective of natural deduction: {other:?}"),
            };
            formulas.add(node).unwrap()
        }
        _ => panic!("a term of another kind"),
    }
}

/// The sequent a checked derivation shows: its assumptions left of `⊢`,
/// its conclusion right.
fn shown(derivation: &Derivation) -> ordinary::Sequent {
    let mut formulas = Formulas::default();
    let terms = derivation.terms();
    let left: Vec<NodeId> = derivation
        .assumptions()
        .map(|node| ordinary_of(terms, node.zone(0)[0], &mut formulas))
        .collect();
    let root = derivation.root().unwrap().zone(0)[0];
    let right = vec![ordinary_of(terms, root, &mut formulas)];
    ordinary::Sequent::new(formulas, left, right).unwrap()
}

/// What a translation showed: the derivations checked, and those too
/// large to unfold.
#[derive(Debug, Default)]
struct Tally {
    /// Derivations translated and checked, per file.
    checked: HashMap<&'static str, usize>,
    /// Derivations left out for their size.
    large: usize,
    /// Mutants the checker accepted, each shown valid.
    accepted: usize,
    /// Mutants the checker refused.
    refused: usize,
}

/// Translates `source`, a derivation of `sequent`, into every file whose
/// logic proves what the source's does, and checks each; with `mutate`,
/// breaks each small one at every node.
fn cross_check(
    sequent: &ordinary::Sequent,
    source: &ordinary::Derivation,
    calculi: &[Calculus],
    tally: &mut Tally,
    mutate: bool,
) {
    let limits = Limits::default();
    for (file, calculus) in FILES.iter().zip(calculi) {
        if strength(file.logic) < strength(source.logic()) {
            continue;
        }
        let mut translator = Translator::new(file, source);
        let tree = translator.translate();
        let Some(derivation) = translator.derivation(tree) else {
            tally.large += 1;
            continue;
        };
        if let Err(error) = derivation.check(calculus, &limits, |_| false) {
            panic!(
                "{sequent} from {} into {}: {error}",
                source.logic(),
                file.name
            );
        }
        // It shows the sequent: its conclusion, from hypotheses of it.
        let (left, goal) = translator.statement();
        let terms = derivation.terms();
        let root = derivation.root().unwrap().zone(0)[0];
        let concluded = ordinary_of(terms, root, &mut translator.formulas);
        assert_eq!(concluded, goal, "{sequent} into {}", file.name);
        for assumption in derivation.assumptions() {
            let id = ordinary_of(terms, assumption.zone(0)[0], &mut translator.formulas);
            assert!(
                left.contains(&id),
                "{sequent} into {}: an assumption that is no hypothesis",
                file.name
            );
        }
        *tally.checked.entry(file.name).or_default() += 1;
        if mutate && derivation.len() <= 40 {
            mutants(&derivation, calculus, file, tally);
        }
    }
}

impl Translator<'_> {
    /// The source's sequent in this file's formulas: its hypotheses, and
    /// the formula a translation concludes.
    fn statement(&mut self) -> (Vec<NodeId>, NodeId) {
        let (left, right) = self.source.sequent();
        let left: Vec<NodeId> = left.iter().map(|&l| self.formula(l)).collect();
        let right: Vec<NodeId> = right.iter().map(|&r| self.formula(r)).collect();
        let goal = match right[..] {
            [] => self.falsum(),
            [one] => one,
            [first, ref rest @ ..] => rest
                .iter()
                .fold(first, |d, &next| self.add(ordinary::Node::Or(d, next))),
        };
        (left, goal)
    }
}

/// Breaks `derivation` once per node in each of four ways, and asserts
/// that every break the checker accepts shows a sequent valid in the
/// file's logic.
fn mutants(derivation: &Derivation, calculus: &Calculus, file: &File, tally: &mut Tally) {
    let limits = Limits::default();
    let document = serde_json::to_value(derivation).unwrap();
    let nodes = document["nodes"].as_array().unwrap().len();
    let terms = document["terms"].as_array().unwrap().len();
    let names: Vec<&str> = calculus.rules().collect();
    let mut broken = Vec::new();
    for at in 0..nodes {
        let node = &document["nodes"][at];
        // Another formula.
        for term in (0..terms).step_by(3) {
            let mut mutant = document.clone();
            mutant["nodes"][at]["sequent"] = serde_json::json!([[term]]);
            broken.push(mutant);
        }
        // Another rule.
        if node.get("rule").is_some() {
            for name in &names {
                let mut mutant = document.clone();
                mutant["nodes"][at]["rule"] = Value::from(*name);
                broken.push(mutant);
            }
        }
        // The premises the other way round.
        if let Some(premises) = node["premises"].as_array().filter(|p| p.len() > 1) {
            let mut mutant = document.clone();
            let reversed: Vec<Value> = premises.iter().rev().cloned().collect();
            mutant["nodes"][at]["premises"] = Value::from(reversed);
            broken.push(mutant);
        }
        // Out of its scope.
        if node.get("scope").is_some() {
            let mut mutant = document.clone();
            mutant["nodes"][at].as_object_mut().unwrap().remove("scope");
            broken.push(mutant);
        }
    }
    for mutant in broken {
        let Ok(read) = wire::upgrade::<Derivation, _>(&mutant, &limits) else {
            continue;
        };
        if read.check(calculus, &limits, |_| false).is_ok() {
            let sequent = shown(&read);
            assert!(
                valid(&sequent, file.logic),
                "{} accepts {mutant}, which shows {sequent}, not valid",
                file.name
            );
            tally.accepted += 1;
        } else {
            tally.refused += 1;
        }
    }
}

/// The calculi of [`FILES`], read.
fn calculi() -> Vec<Calculus> {
    FILES.iter().map(|file| calculus(file.text)).collect()
}

/// The methods that give a derivation in `logic`.
fn methods(logic: Logic) -> Vec<Method> {
    let mut methods = vec![Method::Embedding];
    if logic != Logic::Classical {
        methods.push(Method::Ljt);
    }
    #[cfg(feature = "interactive")]
    if logic == Logic::Classical {
        methods.push(Method::Tableau);
    }
    methods
}

/// Decides `sequent` in `logic` by every method that gives a
/// derivation, and cross-checks each derivation.
fn every_method(
    sequent: &ordinary::Sequent,
    logic: Logic,
    calculi: &[Calculus],
    tally: &mut Tally,
    mutate: bool,
) -> Option<bool> {
    let mut verdict = None;
    for method in methods(logic) {
        let options = ordinary::Options::default()
            .with_logic(logic)
            .with_method(Some(method));
        let Ok(outcome) = ordinary::decide(sequent, &options) else {
            continue;
        };
        match &outcome.verdict {
            Verdict::Valid(evidence) => {
                verdict = Some(true);
                if let Ok(derivation) = evidence.derivation() {
                    cross_check(sequent, derivation, calculi, tally, mutate);
                }
            }
            Verdict::NotValid => verdict = Some(false),
            _ => {}
        }
    }
    verdict
}

/// Sequents of every connective, valid in some of the three logics.
const SEQUENTS: [&str; 22] = [
    "a /\\ b |- b /\\ a",
    "a \\/ b |- b \\/ a",
    "a -> b, b -> c |- a -> c",
    "|- a -> b -> a",
    "a, ~a |- b",
    "a, ~a |- false",
    "|- ~~(a \\/ ~a)",
    "|- a \\/ ~a",
    "~~a |- a",
    "|- ((a -> b) -> a) -> a",
    "a <-> b, a |- b",
    "|- (a <-> b) <-> (b <-> a)",
    "|- true",
    "false |- a",
    "a /\\ (b \\/ c) |- (a /\\ b) \\/ (a /\\ c)",
    "~(a \\/ b) |- ~a /\\ ~b",
    "~a /\\ ~b |- ~(a \\/ b)",
    "~(a /\\ b) |- ~a \\/ ~b",
    "|- a, ~a",
    "a -> b |- ~a, b",
    "|- (a -> b) \\/ (b -> a)",
    "a, b |- a /\\ b, c",
];

#[test]
fn derivations_of_the_ordinary_suite_pass_in_natural_deduction() {
    let calculi = calculi();
    let mut tally = Tally::default();
    for text in SEQUENTS {
        let sequent: ordinary::Sequent = text.parse().unwrap();
        for logic in [Logic::Minimal, Logic::Intuitionistic, Logic::Classical] {
            // Two formulas right of `⊢` are classical logic's alone.
            if logic != Logic::Classical && sequent.right().len() > 1 {
                continue;
            }
            every_method(&sequent, logic, &calculi, &mut tally, true);
        }
    }
    println!("{tally:?}");
    // Every file checked derivations, and the breaks exercised both
    // verdicts.
    for file in &FILES {
        assert!(
            tally.checked.get(file.name).copied().unwrap_or(0) >= 10,
            "{tally:?}"
        );
    }
    assert!(tally.refused > 10_000 && tally.accepted > 100, "{tally:?}");
}

#[test]
fn a_weaker_file_refuses_what_needs_a_stronger_rule() {
    let calculi = calculi();
    let limits = Limits::default();
    for (text, logic, from, into, rule) in [
        ("|- a \\/ ~a", Logic::Classical, "NK", "NJ", "RAA"),
        ("~~a |- a", Logic::Classical, "NK-DNE", "NJ", "¬¬E"),
        ("false |- a", Logic::Intuitionistic, "NJ", "NM", "⊥E"),
        (
            "a, ~a |- b",
            Logic::Intuitionistic,
            "NJ-defined",
            "NM-defined",
            "⊥E",
        ),
    ] {
        let sequent: ordinary::Sequent = text.parse().unwrap();
        let options = ordinary::Options::default()
            .with_logic(logic)
            .with_method(Some(Method::Embedding));
        let outcome = ordinary::decide(&sequent, &options).unwrap();
        let Verdict::Valid(evidence) = &outcome.verdict else {
            panic!("{text} in {logic}");
        };
        let source = evidence.derivation().unwrap();
        let place = |name: &str| FILES.iter().position(|f| f.name == name).unwrap();
        let mut translator = Translator::new(&FILES[place(from)], source);
        let tree = translator.translate();
        let derivation = translator.derivation(tree).unwrap();
        assert!(
            derivation
                .check(&calculi[place(from)], &limits, |_| false)
                .is_ok()
        );
        // The same derivation, named as one of the weaker file, is
        // refused for a rule that file lacks.
        let mut renamed = serde_json::to_value(&derivation).unwrap();
        renamed["calculus"] = Value::from(into);
        let renamed: Derivation = wire::upgrade(&renamed, &limits).unwrap();
        match renamed.check(&calculi[place(into)], &limits, |_| false) {
            Err(deduxit::Error::InvalidDerivation { reason, .. }) => {
                assert!(reason.contains(rule), "{into} on {text}: {reason}");
            }
            other => panic!("{into} on the derivation of {text} in {from}: {other:?}"),
        }
    }
}

/// A random formula over three atoms, at most `depth` deep.
fn formula(seed: &mut u64, depth: u32) -> String {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    let pick = *seed % if depth == 0 { 5 } else { 11 };
    let operand = |seed: &mut u64| formula(seed, depth - 1);
    match pick {
        0 => "a".to_owned(),
        1 => "b".to_owned(),
        2 => "c".to_owned(),
        3 => "true".to_owned(),
        4 => "false".to_owned(),
        5 => format!("~({})", operand(seed)),
        6 => format!("({}) /\\ ({})", operand(seed), operand(seed)),
        7 => format!("({}) \\/ ({})", operand(seed), operand(seed)),
        8 | 9 => format!("({}) -> ({})", operand(seed), operand(seed)),
        _ => format!("({}) <-> ({})", operand(seed), operand(seed)),
    }
}

#[test]
fn generated_sequents_pass_where_they_are_valid() {
    let calculi = calculi();
    let mut tally = Tally::default();
    let mut seed = 62_u64;
    let (mut valid, mut not_valid) = (0, 0);
    for _ in 0..300 {
        let hypotheses: Vec<String> = (0..seed % 3).map(|_| formula(&mut seed, 2)).collect();
        let goal = formula(&mut seed, 3);
        let text = format!("{} |- {goal}", hypotheses.join(", "));
        let sequent: ordinary::Sequent = text.parse().unwrap();
        for logic in [Logic::Minimal, Logic::Intuitionistic, Logic::Classical] {
            match every_method(&sequent, logic, &calculi, &mut tally, false) {
                Some(true) => valid += 1,
                Some(false) => not_valid += 1,
                None => {}
            }
        }
    }
    println!("{valid} valid, {not_valid} not valid, {tally:?}");
    assert!(
        valid > 200 && not_valid > 200,
        "{valid} valid, {not_valid} not valid"
    );
}

/// The ILTP library's `Problems` directory.
fn library() -> std::path::PathBuf {
    std::env::var_os("ILTP").map_or_else(
        || std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../bench/iltp/Problems"),
        std::path::PathBuf::from,
    )
}

/// The ILTP library's problems that the ordinary suite proves with a
/// derivation of at most [`SMALL`] inferences, in each logic.
const SMALL: usize = 2_000;

#[test]
#[ignore = "needs the ILTP library, which is not in the repository: nix build .#iltp -o bench/iltp"]
fn iltp_s_small_problems_pass_in_natural_deduction() {
    // The embedding's search and the translator recurse as deep as a
    // proof, so they run on a thread with the stack the command gives
    // its search.
    std::thread::Builder::new()
        .stack_size(STACK)
        .spawn(iltp)
        .unwrap()
        .join()
        .unwrap();
}

/// The stack of the thread the ILTP problems run on.
const STACK: usize = 512 << 20;

/// The body of [`iltp_s_small_problems_pass_in_natural_deduction`].
fn iltp() {
    use deduxit::ordinary::read_tptp;
    let calculi = calculi();
    let limits = Limits::default().with_work(Some(1 << 22));
    let mut tally = Tally::default();
    let mut proved = HashMap::<Logic, usize>::new();
    let mut entries: Vec<_> = std::fs::read_dir(library().join("SYJ"))
        .into_iter()
        .chain(std::fs::read_dir(library().join("LCL")))
        .chain(std::fs::read_dir(library().join("SYN")))
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "p"))
        .collect();
    entries.sort();
    assert!(
        !entries.is_empty(),
        "no problem under {}",
        library().display()
    );
    for path in entries {
        let text = std::fs::read_to_string(&path).unwrap();
        let Ok(problem) = read_tptp(&text, &Limits::default()) else {
            continue;
        };
        let sequent = problem.sequent;
        for logic in [Logic::Minimal, Logic::Intuitionistic, Logic::Classical] {
            // The tableau method is left out: on the larger pigeonhole
            // problems it holds several times its memory bound.
            for method in methods(logic).into_iter().filter(|&m| m != Method::Tableau) {
                let options = ordinary::Options::default()
                    .with_logic(logic)
                    .with_method(Some(method));
                let search = deduxit::linear::Options::default();
                let Ok(outcome) =
                    ordinary::decide_within(&sequent, &options, &search, &limits, |_| false)
                else {
                    continue;
                };
                let Verdict::Valid(evidence) = &outcome.verdict else {
                    continue;
                };
                let Ok(derivation) = evidence.derivation() else {
                    continue;
                };
                if derivation.inferences().len() > SMALL {
                    continue;
                }
                cross_check(&sequent, derivation, &calculi, &mut tally, false);
                *proved.entry(logic).or_default() += 1;
            }
        }
    }
    println!("{proved:?}, {tally:?}");
    assert!(proved.values().all(|&n| n > 50), "{proved:?}");
}
