// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Natural deduction proofs from sequent proofs. A derivation of LJ or LM
//! becomes a derivation in NJ or NM of the formula right of `⊢` from the
//! hypotheses left of it; one of LK becomes `⊥` from the hypotheses and
//! `¬B` for every `B` right of `⊢`, then the file's classical rule
//! (Gentzen 1935; Prawitz 1965, chapter I; Troelstra and Schwichtenberg,
//! *Basic Proof Theory*, 2nd ed., §3.3 and §6.3). A left rule replaces
//! every open leaf of the formulas it adds by a derivation of them from
//! its principal formula, so every elimination's major premise is a
//! hypothesis or another elimination, and the proof is normal: no
//! introduction is followed by an elimination of its connective. Where a
//! leaf so replaced by an introduction is an elimination's major premise,
//! the detour is reduced at once.
//!
//! Every pass is a loop over an explicit stack: a derivation may be as
//! high as it is long.

#[cfg(all(feature = "interactive", feature = "parse"))]
use super::TermId;
use super::{Calculus, Derivation, Node as Written, ScopeId, Step};
use crate::hash::HashMap;
use crate::ordinary::{self, Formulas, Logic, Node, NodeId, Rule, Side};
use crate::{Error, Limits, Phase, Progress, Refusal};

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

/// What a natural deduction file deduxit ships is: its logic, its
/// classical rule, and whether `¬A` is written `A → ⊥`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Profile {
    /// The logic its derivations are valid in.
    pub(crate) logic: Logic,
    /// Its classical rule, if it has one.
    classical: Option<Classical>,
    /// Whether it has no `¬`, `A → ⊥` in its place.
    defined: bool,
}

/// The rules every file has, `¬I` and `¬E` but where negation is defined.
const BASE: [&str; 14] = [
    "⊤I", "∧I", "∧E₁", "∧E₂", "∨I₁", "∨I₂", "∨E", "→I", "→E", "¬I", "¬E", "↔I", "↔E₁", "↔E₂",
];

/// A shipped file: its name, what it is, and the rules it has beyond
/// [`BASE`].
struct File {
    /// The calculus's name.
    name: &'static str,
    /// What it is.
    profile: Profile,
    /// Its rules beyond [`BASE`].
    extra: &'static [&'static str],
}

/// Returns the file `name`.
const fn file(
    name: &'static str,
    logic: Logic,
    classical: Option<Classical>,
    defined: bool,
    extra: &'static [&'static str],
) -> File {
    File {
        name,
        profile: Profile {
            logic,
            classical,
            defined,
        },
        extra,
    }
}

/// Every shipped natural deduction file.
const FILES: [File; 8] = [
    file("NJ", Logic::Intuitionistic, None, false, &["⊥E"]),
    file(
        "NK",
        Logic::Classical,
        Some(Classical::Raa),
        false,
        &["⊥E", "RAA"],
    ),
    file("NM", Logic::Minimal, None, false, &[]),
    file(
        "NK-DNE",
        Logic::Classical,
        Some(Classical::Dne),
        false,
        &["⊥E", "¬¬E"],
    ),
    file(
        "NK-LEM",
        Logic::Classical,
        Some(Classical::Lem),
        false,
        &["⊥E", "LEM"],
    ),
    file("NJ-defined", Logic::Intuitionistic, None, true, &["⊥E"]),
    file(
        "NK-defined",
        Logic::Classical,
        Some(Classical::Raa),
        true,
        &["⊥E", "RAA"],
    ),
    file("NM-defined", Logic::Minimal, None, true, &[]),
];

impl Profile {
    /// Returns what `calculus` is where it is one of natural deduction's
    /// files deduxit ships, told by its name and its rules, else `None`.
    /// A file of that name with other rules is not one: a translated proof
    /// is checked against the file in any case.
    pub(crate) fn of(calculus: &Calculus) -> Option<Self> {
        let file = FILES.iter().find(|file| file.name == calculus.name())?;
        let defined = file.profile.defined;
        let expected = BASE
            .iter()
            .filter(|rule| !defined || !matches!(**rule, "¬I" | "¬E"))
            .chain(file.extra);
        calculus
            .rules()
            .eq(expected.copied())
            .then_some(file.profile)
    }
}

/// A node of a natural deduction tree under construction; trees share
/// their parts.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Tree {
    /// A hypothesis: open, or discharged by the rule that took the label.
    Hypothesis(NodeId, Option<u32>),
    /// A rule's conclusion and its premises, each with the label of the
    /// hypotheses it discharges.
    Rule(&'static str, NodeId, Vec<(usize, Option<u32>)>),
}

/// The bytes a tree node and a derivation's node are charged to the memory
/// bound: the node, its premises' list, its sequent and its scope's slot.
const NODE_BYTES: u64 = 128;

/// How often the stop is asked, in nodes visited.
const POLL: u64 = 1 << 10;

/// Translates a checked derivation of LK, LJ or LM into a natural
/// deduction file.
struct Translator<'a> {
    /// The file.
    profile: Profile,
    /// The source derivation.
    source: &'a ordinary::Derivation,
    /// The formulas of natural deduction.
    formulas: Formulas,
    /// Each source formula's formula here, by its id.
    mapped: Vec<NodeId>,
    /// The trees, shared.
    trees: Vec<Tree>,
    /// The labels given so far.
    labels: u32,
    /// The nodes visited, against `limits.work`.
    visited: u64,
    /// The bounds.
    limits: &'a Limits,
    /// The caller's stop.
    stop: &'a mut dyn FnMut(Progress) -> bool,
}

impl<'a> Translator<'a> {
    /// Returns a translator of `source`, a propositional derivation, into
    /// the file `profile` describes, with every source formula mapped.
    fn new(
        profile: Profile,
        source: &'a ordinary::Derivation,
        limits: &'a Limits,
        stop: &'a mut dyn FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        let mut translator = Self {
            profile,
            source,
            formulas: Formulas::default(),
            mapped: Vec::with_capacity(source.formulas().len()),
            trees: Vec::new(),
            labels: 0,
            visited: 0,
            limits,
            stop,
        };
        // Every operand precedes its formula.
        for &node in source.formulas().nodes() {
            let at = |k: NodeId| translator.mapped[k.index()];
            let mapped = match node {
                Node::Atom(atom) => translator
                    .formulas
                    .atom(source.formulas().atom_name(atom))?,
                Node::Not(a) => translator.not(at(a))?,
                Node::True | Node::False => translator.formulas.add(node)?,
                Node::And(a, b) => translator.formulas.add(Node::And(at(a), at(b)))?,
                Node::Or(a, b) => translator.formulas.add(Node::Or(at(a), at(b)))?,
                Node::Implies(a, b) => translator.formulas.add(Node::Implies(at(a), at(b)))?,
                Node::Iff(a, b) => translator.formulas.add(Node::Iff(at(a), at(b)))?,
                Node::Forall(_) | Node::Exists(_) => {
                    unreachable!("the translation takes propositional derivations")
                }
            };
            translator.mapped.push(mapped);
        }
        Ok(translator)
    }

    /// Counts a node visited against `limits.work` and the memory bound,
    /// and asks the stop every [`POLL`] of them.
    fn tick(&mut self) -> Result<(), Error> {
        self.visited += 1;
        if let Some(limit) = self.limits.work
            && self.visited > limit
        {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        let held = (self.trees.len() as u64).saturating_mul(NODE_BYTES);
        if let Some(limit) = self.limits.memory_bytes
            && held > limit
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: limit,
                needed_bytes: None,
            }));
        }
        if self.visited % POLL == 1
            && (self.stop)(Progress::new(Phase::ReadBack, POLL, self.visited))
        {
            return Err(Error::Refused(Refusal::stopped(Phase::ReadBack)));
        }
        Ok(())
    }

    /// `⊥`.
    fn falsum(&mut self) -> Result<NodeId, Error> {
        self.formulas.add(Node::False)
    }

    /// `¬a`, or `a → ⊥` where negation is defined.
    fn not(&mut self, a: NodeId) -> Result<NodeId, Error> {
        if self.profile.defined {
            let falsum = self.falsum()?;
            self.formulas.add(Node::Implies(a, falsum))
        } else {
            self.formulas.add(Node::Not(a))
        }
    }

    /// The operands of a compound formula here; a negation's twice.
    fn operands(&self, id: NodeId) -> (NodeId, NodeId) {
        match self.formulas.node(id) {
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => (a, b),
            Node::Not(a) => (a, a),
            Node::Atom(_) | Node::True | Node::False | Node::Forall(_) | Node::Exists(_) => {
                unreachable!("a checked rule's principal formula is compound")
            }
        }
    }

    /// Adds a tree node.
    fn push(&mut self, node: Tree) -> Result<usize, Error> {
        self.trees.push(node);
        self.tick()?;
        Ok(self.trees.len() - 1)
    }

    /// An open hypothesis.
    fn hypothesis(&mut self, formula: NodeId) -> Result<usize, Error> {
        self.push(Tree::Hypothesis(formula, None))
    }

    /// A rule's conclusion over premises that discharge nothing.
    fn rule(
        &mut self,
        name: &'static str,
        formula: NodeId,
        premises: &[usize],
    ) -> Result<usize, Error> {
        let premises = premises.iter().map(|&p| (p, None)).collect();
        self.push(Tree::Rule(name, formula, premises))
    }

    /// The formula a tree concludes.
    fn conclusion(&self, tree: usize) -> NodeId {
        match self.trees[tree] {
            Tree::Hypothesis(formula, _) | Tree::Rule(_, formula, _) => formula,
        }
    }

    /// `tree` with every leaf `leaf` (a formula with its label, `None` for
    /// an open one) replaced by `with`, a derivation of that formula. An
    /// elimination of `→` or `¬` whose major premise is such a leaf, where
    /// `with` is the introduction of that connective, is reduced at once:
    /// the introduction's premise with its discharged leaves replaced by
    /// the elimination's minor premise, so no detour arises.
    fn replace(
        &mut self,
        tree: usize,
        leaf: (NodeId, Option<u32>),
        with: usize,
    ) -> Result<usize, Error> {
        let mut memo: HashMap<usize, usize> = HashMap::default();
        // Each tree is entered, then left once its premises are done.
        let mut stack: Vec<(usize, bool)> = vec![(tree, false)];
        while let Some((at, left)) = stack.pop() {
            if memo.contains_key(&at) {
                continue;
            }
            self.tick()?;
            match self.trees[at].clone() {
                Tree::Hypothesis(formula, label) => {
                    let done = if (formula, label) == leaf { with } else { at };
                    memo.insert(at, done);
                }
                Tree::Rule(name, conclusion, premises) => {
                    if !left {
                        stack.push((at, true));
                        stack.extend(premises.iter().map(|&(p, _)| (p, false)));
                        continue;
                    }
                    let reduced = match (name, premises.as_slice(), &self.trees[with]) {
                        (
                            "→E" | "¬E",
                            [(major, None), (minor, None)],
                            Tree::Rule(intro, _, body),
                        ) if self.trees[*major] == Tree::Hypothesis(leaf.0, leaf.1)
                            && matches!((name, *intro), ("→E", "→I") | ("¬E", "¬I")) =>
                        {
                            match body.as_slice() {
                                &[(body, Some(label))] => Some((body, label, memo[minor])),
                                _ => None,
                            }
                        }
                        _ => None,
                    };
                    let done = if let Some((body, label, minor)) = reduced {
                        let discharged = self.operands(self.conclusion(with)).0;
                        self.replace(body, (discharged, Some(label)), minor)?
                    } else {
                        let new: Vec<(usize, Option<u32>)> = premises
                            .iter()
                            .map(|&(p, label)| (memo[&p], label))
                            .collect();
                        if new == premises {
                            at
                        } else {
                            self.push(Tree::Rule(name, conclusion, new))?
                        }
                    };
                    memo.insert(at, done);
                }
            }
        }
        Ok(memo[&tree])
    }

    /// `tree` with every open leaf of `formula` replaced by `with`, a
    /// derivation of it.
    fn substitute(&mut self, tree: usize, formula: NodeId, with: usize) -> Result<usize, Error> {
        self.replace(tree, (formula, None), with)
    }

    /// `tree` as a premise that discharges its open leaves of `formula`.
    fn bind(&mut self, tree: usize, formula: NodeId) -> Result<(usize, Option<u32>), Error> {
        self.labels += 1;
        let label = self.labels;
        let leaf = self.push(Tree::Hypothesis(formula, Some(label)))?;
        let bound = self.replace(tree, (formula, None), leaf)?;
        Ok((bound, Some(label)))
    }

    /// `¬a` from `tree`, a derivation of `⊥` with `a` open.
    fn not_intro(&mut self, tree: usize, a: NodeId) -> Result<usize, Error> {
        let not = self.not(a)?;
        let premise = self.bind(tree, a)?;
        let name = if self.profile.defined { "→I" } else { "¬I" };
        self.push(Tree::Rule(name, not, vec![premise]))
    }

    /// `⊥` from a derivation of `¬a` and one of `a`.
    fn not_elim(&mut self, not: usize, a: usize) -> Result<usize, Error> {
        let name = if self.profile.defined { "→E" } else { "¬E" };
        let falsum = self.falsum()?;
        self.rule(name, falsum, &[not, a])
    }

    /// `goal` from `tree`, a derivation of `⊥`.
    fn absurd(&mut self, tree: usize, goal: NodeId) -> Result<usize, Error> {
        if goal == self.falsum()? {
            Ok(tree)
        } else {
            self.rule("⊥E", goal, &[tree])
        }
    }

    /// `a` from `tree`, a derivation of `⊥` with `¬a` open, by the
    /// file's classical rule.
    fn classical(&mut self, tree: usize, a: NodeId) -> Result<usize, Error> {
        let not = self.not(a)?;
        match self.profile.classical {
            Some(Classical::Raa) => {
                let premise = self.bind(tree, not)?;
                self.push(Tree::Rule("RAA", a, vec![premise]))
            }
            Some(Classical::Dne) => {
                let twice = self.not_intro(tree, not)?;
                self.rule("¬¬E", a, &[twice])
            }
            Some(Classical::Lem) => {
                let either = self.formulas.add(Node::Or(a, not))?;
                let lem = self.rule("LEM", either, &[])?;
                let leaf = self.hypothesis(a)?;
                let first = self.bind(leaf, a)?;
                let absurd = self.absurd(tree, a)?;
                let second = self.bind(absurd, not)?;
                self.push(Tree::Rule("∨E", a, vec![(lem, None), first, second]))
            }
            None => unreachable!("a derivation of LK goes to a classical file"),
        }
    }

    /// `one` from `tree`, a derivation of `⊥` with `¬one` open: for `⊥`
    /// itself each `¬⊥` leaf is `¬I` over `⊥`'s own hypothesis, which the
    /// elimination it stands in takes at once; where `¬one` is a hypothesis
    /// of the sequent, the derivation of `one` that `tree` refutes it with,
    /// or `⊥E`; otherwise the file's classical rule. A proof
    /// that ends in the classical rule on a goal `⊥`, or that discharges a
    /// `¬one` already in force, gives a hint that leads back to the same
    /// goal.
    fn concluded(&mut self, tree: usize, one: NodeId) -> Result<usize, Error> {
        let falsum = self.falsum()?;
        let negated = self.not(one)?;
        if one == falsum {
            let leaf = self.hypothesis(falsum)?;
            let not = self.not_intro(leaf, falsum)?;
            return self.substitute(tree, negated, not);
        }
        let (left, _) = self.source.sequent();
        if left.iter().any(|l| self.mapped[l.index()] == negated) {
            // `⊥` as `¬E` of the hypothesis `¬one` and a derivation of
            // `one` is that derivation, with `¬one` in force.
            if let Tree::Rule("¬E" | "→E", _, premises) = &self.trees[tree]
                && let [(major, None), (minor, None)] = premises[..]
                && self.trees[major] == Tree::Hypothesis(negated, None)
                && self.conclusion(minor) == one
            {
                return Ok(minor);
            }
            return self.absurd(tree, one);
        }
        self.classical(tree, one)
    }

    /// The formula on a side of an inference, here.
    fn at(&self, inference: &ordinary::Inference, (side, at): (Side, usize)) -> NodeId {
        let id = match side {
            Side::Left => inference.left()[at],
            Side::Right => inference.right()[at],
        };
        self.mapped[id.index()]
    }

    /// `tree` with the open leaves of a conjunction's parts replaced.
    fn and_left(&mut self, tree: usize, rule: Rule, conjunction: NodeId) -> Result<usize, Error> {
        let (a, b) = self.operands(conjunction);
        let mut tree = tree;
        if rule != Rule::AndLeft2 {
            let leaf = self.hypothesis(conjunction)?;
            let first = self.rule("∧E₁", a, &[leaf])?;
            tree = self.substitute(tree, a, first)?;
        }
        if rule != Rule::AndLeft1 {
            let leaf = self.hypothesis(conjunction)?;
            let second = self.rule("∧E₂", b, &[leaf])?;
            tree = self.substitute(tree, b, second)?;
        }
        Ok(tree)
    }

    /// `tree` with the open leaves of an equivalence's implications
    /// replaced: each by `→I` over `↔E`, which an elimination of the
    /// implication takes at once as `↔E` on its minor premise.
    fn iff_left(&mut self, tree: usize, rule: Rule, iff: NodeId) -> Result<usize, Error> {
        let (a, b) = self.operands(iff);
        let mut tree = tree;
        for (from, to, name, used) in [
            (a, b, "↔E₁", rule != Rule::IffLeft2),
            (b, a, "↔E₂", rule != Rule::IffLeft1),
        ] {
            if !used {
                continue;
            }
            let implication = self.formulas.add(Node::Implies(from, to))?;
            let major = self.hypothesis(iff)?;
            let minor = self.hypothesis(from)?;
            let step = self.rule(name, to, &[major, minor])?;
            let premise = self.bind(step, from)?;
            let derived = self.push(Tree::Rule("→I", implication, vec![premise]))?;
            tree = self.substitute(tree, implication, derived)?;
        }
        Ok(tree)
    }

    /// `a ↔ b` from derivations of `a → b` and `b → a`; one that ends in
    /// `→I` gives its premise at once, with no detour.
    fn iff_right(&mut self, forward: usize, backward: usize, iff: NodeId) -> Result<usize, Error> {
        let (a, b) = self.operands(iff);
        let mut halves = Vec::with_capacity(2);
        for (tree, from, to) in [(forward, a, b), (backward, b, a)] {
            if let Tree::Rule("→I", _, premises) = &self.trees[tree]
                && let [(body, Some(label))] = premises[..]
            {
                halves.push((body, Some(label)));
                continue;
            }
            let leaf = self.hypothesis(from)?;
            let step = self.rule("→E", to, &[tree, leaf])?;
            halves.push(self.bind(step, from)?);
        }
        self.push(Tree::Rule("↔I", iff, halves))
    }

    /// The tree of an inference of LJ or LM, given its premises' trees.
    fn intuitionistic(
        &mut self,
        inference: &ordinary::Inference,
        trees: &[usize],
    ) -> Result<usize, Error> {
        let goal = match inference.right().first() {
            Some(&right) => self.mapped[right.index()],
            None => self.falsum()?,
        };
        let principal = inference.principal().map(|place| self.at(inference, place));
        let principal = || principal.expect("a rule with a principal formula");
        match inference.rule() {
            Rule::Axiom => self.hypothesis(goal),
            Rule::WeakenLeft | Rule::ContractLeft | Rule::TrueLeft | Rule::FalseRight => {
                Ok(trees[0])
            }
            Rule::WeakenRight => self.absurd(trees[0], goal),
            Rule::FalseLeft => {
                let falsum = self.falsum()?;
                let leaf = self.hypothesis(falsum)?;
                self.absurd(leaf, goal)
            }
            Rule::TrueRight => self.rule("⊤I", goal, &[]),
            rule @ (Rule::AndLeft | Rule::AndLeft1 | Rule::AndLeft2) => {
                self.and_left(trees[0], rule, principal())
            }
            Rule::AndRight => self.rule("∧I", goal, trees),
            Rule::OrLeft => {
                let p = principal();
                let (a, b) = self.operands(p);
                let major = self.hypothesis(p)?;
                let first = self.bind(trees[0], a)?;
                let second = self.bind(trees[1], b)?;
                self.push(Tree::Rule("∨E", goal, vec![(major, None), first, second]))
            }
            Rule::OrRight1 => self.rule("∨I₁", goal, &[trees[0]]),
            Rule::OrRight2 => self.rule("∨I₂", goal, &[trees[0]]),
            Rule::ImpliesLeft => {
                let p = principal();
                let (_, b) = self.operands(p);
                let major = self.hypothesis(p)?;
                let step = self.rule("→E", b, &[major, trees[0]])?;
                self.substitute(trees[1], b, step)
            }
            Rule::ImpliesRight => {
                let (a, _) = self.operands(goal);
                let premise = self.bind(trees[0], a)?;
                self.push(Tree::Rule("→I", goal, vec![premise]))
            }
            Rule::NotLeft => {
                let major = self.hypothesis(principal())?;
                let step = self.not_elim(major, trees[0])?;
                if trees.len() == 1 {
                    Ok(step)
                } else {
                    let falsum = self.falsum()?;
                    self.substitute(trees[1], falsum, step)
                }
            }
            Rule::NotRight => {
                let (a, _) = self.operands(goal);
                self.not_intro(trees[0], a)
            }
            rule @ (Rule::IffLeft | Rule::IffLeft1 | Rule::IffLeft2) => {
                self.iff_left(trees[0], rule, principal())
            }
            Rule::IffRight => self.iff_right(trees[0], trees[1], goal),
            Rule::Cut => {
                let first = self.source.inference(inference.premises()[0]);
                let cut = self.mapped[first.right()[0].index()];
                self.substitute(trees[1], cut, trees[0])
            }
            Rule::OrRight
            | Rule::ContractRight
            | Rule::Open
            | Rule::ForallLeft
            | Rule::ForallRight
            | Rule::ExistsLeft
            | Rule::ExistsRight => {
                unreachable!("no rule of a checked propositional derivation of LJ")
            }
        }
    }

    /// The tree of an inference of LK, a derivation of `⊥`, given its
    /// premises' trees.
    fn classical_inference(
        &mut self,
        inference: &ordinary::Inference,
        trees: &[usize],
    ) -> Result<usize, Error> {
        let principal = inference.principal().map(|place| self.at(inference, place));
        let principal = || principal.expect("a rule with a principal formula");
        match inference.rule() {
            Rule::Axiom => {
                let a = self.mapped[inference.right()[0].index()];
                let not = self.not(a)?;
                let major = self.hypothesis(not)?;
                let minor = self.hypothesis(a)?;
                self.not_elim(major, minor)
            }
            Rule::WeakenLeft
            | Rule::WeakenRight
            | Rule::ContractLeft
            | Rule::ContractRight
            | Rule::TrueLeft
            | Rule::FalseRight
            | Rule::NotLeft => Ok(trees[0]),
            Rule::FalseLeft => {
                let falsum = self.falsum()?;
                self.hypothesis(falsum)
            }
            Rule::TrueRight => {
                let truth = self.formulas.add(Node::True)?;
                let minor = self.rule("⊤I", truth, &[])?;
                self.refute(truth, minor)
            }
            rule @ (Rule::AndLeft | Rule::AndLeft1 | Rule::AndLeft2) => {
                self.and_left(trees[0], rule, principal())
            }
            Rule::AndRight => {
                let p = principal();
                let (a, b) = self.operands(p);
                let a = self.classical(trees[0], a)?;
                let b = self.classical(trees[1], b)?;
                let both = self.rule("∧I", p, &[a, b])?;
                self.refute(p, both)
            }
            Rule::OrLeft => {
                let p = principal();
                let (a, b) = self.operands(p);
                let major = self.hypothesis(p)?;
                let first = self.bind(trees[0], a)?;
                let second = self.bind(trees[1], b)?;
                let falsum = self.falsum()?;
                self.push(Tree::Rule("∨E", falsum, vec![(major, None), first, second]))
            }
            rule @ (Rule::OrRight | Rule::OrRight1 | Rule::OrRight2) => {
                let p = principal();
                let (a, b) = self.operands(p);
                let mut tree = trees[0];
                for (part, name, used) in [
                    (a, "∨I₁", rule != Rule::OrRight2),
                    (b, "∨I₂", rule != Rule::OrRight1),
                ] {
                    if used {
                        let leaf = self.hypothesis(part)?;
                        let either = self.rule(name, p, &[leaf])?;
                        let refuted = self.refute(p, either)?;
                        let not = self.not_intro(refuted, part)?;
                        let negated = self.not(part)?;
                        tree = self.substitute(tree, negated, not)?;
                    }
                }
                Ok(tree)
            }
            Rule::ImpliesLeft => {
                let p = principal();
                let (a, b) = self.operands(p);
                let a = self.classical(trees[0], a)?;
                let major = self.hypothesis(p)?;
                let step = self.rule("→E", b, &[major, a])?;
                self.substitute(trees[1], b, step)
            }
            Rule::ImpliesRight => {
                let p = principal();
                let (a, b) = self.operands(p);
                let b = self.classical(trees[0], b)?;
                let premise = self.bind(b, a)?;
                let implication = self.push(Tree::Rule("→I", p, vec![premise]))?;
                self.refute(p, implication)
            }
            Rule::NotRight => {
                let p = principal();
                let (a, _) = self.operands(p);
                let not = self.not_intro(trees[0], a)?;
                self.refute(p, not)
            }
            rule @ (Rule::IffLeft | Rule::IffLeft1 | Rule::IffLeft2) => {
                self.iff_left(trees[0], rule, principal())
            }
            Rule::IffRight => {
                let p = principal();
                let (a, b) = self.operands(p);
                let forward = self.formulas.add(Node::Implies(a, b))?;
                let backward = self.formulas.add(Node::Implies(b, a))?;
                let forward = self.classical(trees[0], forward)?;
                let backward = self.classical(trees[1], backward)?;
                let iff = self.iff_right(forward, backward, p)?;
                self.refute(p, iff)
            }
            Rule::Cut
            | Rule::Open
            | Rule::ForallLeft
            | Rule::ForallRight
            | Rule::ExistsLeft
            | Rule::ExistsRight => {
                unreachable!("no rule of a checked propositional derivation of LK")
            }
        }
    }

    /// `⊥` from a derivation of `a`, with `¬a` open.
    fn refute(&mut self, a: NodeId, tree: usize) -> Result<usize, Error> {
        let not = self.not(a)?;
        let major = self.hypothesis(not)?;
        self.not_elim(major, tree)
    }

    /// The formula the translation concludes: the one right of `⊢`, `⊥`
    /// for none, the disjunction of several, the first leftmost.
    fn goal(&mut self) -> Result<NodeId, Error> {
        let (_, right) = self.source.sequent();
        let mut right = right.iter().map(|r| self.mapped[r.index()]);
        match right.next() {
            None => self.falsum(),
            Some(first) => {
                let rest: Vec<NodeId> = right.collect();
                let mut disjunction = first;
                for next in rest {
                    disjunction = self.formulas.add(Node::Or(disjunction, next))?;
                }
                Ok(disjunction)
            }
        }
    }

    /// The whole derivation as a tree, concluding [`goal`](Self::goal).
    fn translate(&mut self) -> Result<usize, Error> {
        let classical = self.source.logic() == Logic::Classical;
        let mut trees: Vec<usize> = Vec::with_capacity(self.source.inferences().len());
        for inference in self.source.inferences() {
            let premises: Vec<usize> = inference
                .premises()
                .iter()
                .map(|&p| trees[p.index()])
                .collect();
            let tree = if classical {
                self.classical_inference(inference, &premises)?
            } else {
                self.intuitionistic(inference, &premises)?
            };
            trees.push(tree);
        }
        let mut tree = *trees.last().expect("a derivation has a root");
        if classical {
            let (_, right) = self.source.sequent();
            let right: Vec<NodeId> = right.iter().map(|r| self.mapped[r.index()]).collect();
            match right[..] {
                [] => {}
                [one] => tree = self.concluded(tree, one)?,
                [_, _, ..] => {
                    // Each `¬B` comes from `¬(B₁ ∨ … ∨ Bₙ)`, B injected.
                    let disjunction = self.goal()?;
                    for (i, &part) in right.iter().enumerate() {
                        let leaf = self.hypothesis(part)?;
                        let mut injected = leaf;
                        let mut prefix = right[0];
                        if i > 0 {
                            for &next in &right[1..i] {
                                prefix = self.formulas.add(Node::Or(prefix, next))?;
                            }
                            prefix = self.formulas.add(Node::Or(prefix, part))?;
                            injected = self.rule("∨I₂", prefix, &[injected])?;
                        }
                        for &next in &right[i + 1..] {
                            if prefix == disjunction {
                                break;
                            }
                            prefix = self.formulas.add(Node::Or(prefix, next))?;
                            injected = self.rule("∨I₁", prefix, &[injected])?;
                        }
                        let refuted = self.refute(disjunction, injected)?;
                        let not = self.not_intro(refuted, part)?;
                        let negated = self.not(part)?;
                        tree = self.substitute(tree, negated, not)?;
                    }
                    tree = self.classical(tree, disjunction)?;
                }
            }
        }
        Ok(tree)
    }

    /// The number of nodes `tree` unfolds to, saturating.
    fn unfolded(&self, tree: usize) -> u64 {
        let mut sizes: HashMap<usize, u64> = HashMap::default();
        let mut stack: Vec<(usize, bool)> = vec![(tree, false)];
        while let Some((at, left)) = stack.pop() {
            if sizes.contains_key(&at) {
                continue;
            }
            match &self.trees[at] {
                Tree::Hypothesis(..) => {
                    sizes.insert(at, 1);
                }
                Tree::Rule(_, _, premises) if !left => {
                    stack.push((at, true));
                    stack.extend(premises.iter().map(|&(p, _)| (p, false)));
                }
                Tree::Rule(_, _, premises) => {
                    let size = premises
                        .iter()
                        .fold(1_u64, |sum, (p, _)| sum.saturating_add(sizes[p]));
                    sizes.insert(at, size);
                }
            }
        }
        sizes[&tree]
    }

    /// The tree as a derivation of the file named `name`: each discharging
    /// premise in a scope of its own, each discharged leaf in its binder's
    /// scope, every open leaf an assumption of the root scope.
    fn derivation(&self, tree: usize, name: &str) -> Result<Derivation, Error> {
        let mut derivation = Derivation::new(name, 1);
        // Every formula's term, operands first; the translation takes
        // propositional derivations alone.
        let terms = super::ordinary::terms_of(&self.formulas, derivation.terms_mut())?;
        /// What the emitter has yet to do.
        enum Frame {
            /// Write a tree in a scope.
            Enter(usize, ScopeId),
            /// Write a rule's next premise, or the rule once they are
            /// written, its premises' nodes from `start` of the results.
            Rule {
                tree: usize,
                scope: ScopeId,
                next: usize,
                start: usize,
            },
        }
        let mut scopes: HashMap<u32, ScopeId> = HashMap::default();
        let mut results: Vec<u32> = Vec::new();
        let mut stack = vec![Frame::Enter(tree, ScopeId::ROOT)];
        while let Some(frame) = stack.pop() {
            match frame {
                Frame::Enter(at, scope) => match &self.trees[at] {
                    &Tree::Hypothesis(formula, label) => {
                        let place = label.map_or(ScopeId::ROOT, |label| scopes[&label]);
                        let sequent = [vec![terms[formula.index()]]];
                        let node = derivation
                            .push(Written::new(&sequent, Step::Hypothesis).scope(place))?;
                        results.push(node.index() as u32);
                    }
                    Tree::Rule(..) => stack.push(Frame::Rule {
                        tree: at,
                        scope,
                        next: 0,
                        start: results.len(),
                    }),
                },
                Frame::Rule {
                    tree: at,
                    scope,
                    next,
                    start,
                } => {
                    let Tree::Rule(name, formula, premises) = &self.trees[at] else {
                        unreachable!("a rule's frame holds a rule")
                    };
                    if let Some(&(premise, label)) = premises.get(next) {
                        let inner = match label {
                            Some(label) => {
                                let inner = derivation.scope(scope.index() as u32)?;
                                scopes.insert(label, inner);
                                inner
                            }
                            None => scope,
                        };
                        stack.push(Frame::Rule {
                            tree: at,
                            scope,
                            next: next + 1,
                            start,
                        });
                        stack.push(Frame::Enter(premise, inner));
                    } else {
                        let indices: Vec<u32> = results.drain(start..).collect();
                        let sequent = [vec![terms[formula.index()]]];
                        let node = derivation.push(
                            Written::new(&sequent, Step::Rule(name))
                                .premises(&indices)
                                .scope(scope),
                        )?;
                        results.push(node.index() as u32);
                    }
                }
            }
        }
        Ok(derivation)
    }
}

impl Derivation {
    /// Returns the derivation of natural deduction that `source`, a
    /// derivation of LK, LJ or LM, translates to in `calculus`, one of the
    /// natural deduction files deduxit ships ([`NJ`](super::NJ),
    /// [`NK`](super::NK), [`NM`](super::NM) and their variants), checked
    /// against it. It concludes the formula right of `source`'s `⊢` (`⊥`
    /// for none, the disjunction of several) from the hypotheses left of
    /// it, which stand as assumptions. The derivation is normal: no
    /// introduction rule's conclusion is an elimination's major premise
    /// of the same connective. `source` is checked first, within `limits`
    /// and until `stop` fires.
    ///
    /// The file's logic must prove what `source` shows: LJ goes to an
    /// intuitionistic or a classical file, LM to any, LK to a classical
    /// one.
    ///
    /// # Errors
    ///
    /// [`Error::FirstOrder`] for a first-order derivation, which no file
    /// takes; [`Error::NotApplicable`] for a calculus that is no shipped
    /// natural deduction file or one of a weaker logic than `source`'s; the
    /// errors of [`ordinary::Derivation::check`] and of
    /// [`check`](Self::check); and [`Refusal::Work`], [`Refusal::Memory`]
    /// and [`Refusal::Stopped`] while the proof is built.
    pub fn natural(
        source: &ordinary::Derivation,
        calculus: &Calculus,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        natural_dyn(source, calculus, limits, &mut stop)
    }
}

/// The strength of a logic: what it proves includes what a weaker one
/// proves.
pub(crate) const fn strength(logic: Logic) -> u8 {
    match logic {
        Logic::Minimal => 0,
        Logic::Intuitionistic => 1,
        Logic::Classical => 2,
    }
}

/// [`Derivation::natural`] with the stop as a trait object.
fn natural_dyn(
    source: &ordinary::Derivation,
    calculus: &Calculus,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Derivation, Error> {
    source
        .formulas()
        .propositional("natural deduction's translation")?;
    let Some(profile) = Profile::of(calculus) else {
        return Err(Error::NotApplicable {
            operation: "natural deduction's translation".into(),
            reason: format!(
                "{} is not one of the natural deduction files deduxit ships",
                calculus.name()
            ),
        });
    };
    if strength(profile.logic) < strength(source.logic()) {
        return Err(Error::NotApplicable {
            operation: "natural deduction's translation".into(),
            reason: format!(
                "{} is a file of {} logic, which does not prove what a derivation of {} shows",
                calculus.name(),
                profile.logic,
                source.logic().calculus()
            ),
        });
    }
    source.check(limits, &mut *stop)?;
    let mut translator = Translator::new(profile, source, limits, stop)?;
    let tree = translator.translate()?;
    let nodes = translator.unfolded(tree);
    let estimate = nodes.saturating_mul(NODE_BYTES);
    if let Some(limit) = limits.memory_bytes
        && estimate > limit
    {
        return Err(Error::Refused(Refusal::Memory {
            phase: Phase::ReadBack,
            limit_bytes: limit,
            needed_bytes: Some(estimate),
        }));
    }
    let derivation = translator.derivation(tree, calculus.name())?;
    derivation.check(calculus, limits, &mut *translator.stop)?;
    Ok(derivation)
}

/// Returns the ordinary formula of the term `id` of `terms`, a formula of
/// a natural deduction file, added to `formulas`; operands precede the
/// terms made of them, so one pass in order suffices.
///
/// # Errors
///
/// [`Error::NotApplicable`] for a connective no natural deduction file has,
/// and the arena's refusals.
#[cfg(all(feature = "interactive", feature = "parse"))]
pub(crate) fn ordinary_of(
    terms: &super::Terms,
    id: TermId,
    formulas: &mut Formulas,
    memo: &mut HashMap<TermId, NodeId>,
) -> Result<NodeId, Error> {
    let mut stack = vec![(id, false)];
    while let Some((at, left)) = stack.pop() {
        if memo.contains_key(&at) {
            continue;
        }
        match terms.term(at) {
            super::Term::Atom { name, .. } => {
                let node = formulas.atom(name)?;
                memo.insert(at, node);
            }
            super::Term::Apply { arguments, .. } if !left => {
                stack.push((at, true));
                stack.extend(arguments.iter().map(|&a| (a, false)));
            }
            super::Term::Apply {
                connective,
                arguments,
            } => {
                let operands: Vec<NodeId> = arguments.iter().map(|a| memo[a]).collect();
                let node = match (connective, &operands[..]) {
                    ("⊤", []) => Node::True,
                    ("⊥", []) => Node::False,
                    ("¬", &[a]) => Node::Not(a),
                    ("∧", &[a, b]) => Node::And(a, b),
                    ("∨", &[a, b]) => Node::Or(a, b),
                    ("→", &[a, b]) => Node::Implies(a, b),
                    ("↔", &[a, b]) => Node::Iff(a, b),
                    _ => {
                        return Err(Error::NotApplicable {
                            operation: "natural deduction's search".into(),
                            reason: format!("{connective} is no connective of natural deduction"),
                        });
                    }
                };
                let node = formulas.add(node)?;
                memo.insert(at, node);
            }
        }
    }
    Ok(memo[&id])
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::ordinary::{Options, Sequent, Verdict};

    /// Every shipped natural deduction file, read.
    fn files() -> Vec<Calculus> {
        [
            super::super::NJ,
            super::super::NK,
            super::super::NM,
            super::super::NK_DNE,
            super::super::NK_LEM,
            super::super::NJ_DEFINED,
            super::super::NK_DEFINED,
            super::super::NM_DEFINED,
        ]
        .iter()
        .map(|text| {
            let mut document = serde_json::Deserializer::from_str(text);
            crate::wire::upgrade(&mut document, &Limits::default()).unwrap()
        })
        .collect()
    }

    /// Whether no elimination's major premise is the conclusion of the
    /// introduction of its connective.
    fn normal(derivation: &Derivation) -> bool {
        derivation.nodes().all(|node| {
            let Step::Rule(rule) = node.step() else {
                return true;
            };
            let intros: &[&str] = match rule {
                "∧E₁" | "∧E₂" => &["∧I"],
                "∨E" => &["∨I₁", "∨I₂"],
                "→E" => &["→I"],
                "¬E" => &["¬I"],
                "↔E₁" | "↔E₂" => &["↔I"],
                _ => return true,
            };
            let major = derivation.node(node.premises()[0]);
            !matches!(major.step(), Step::Rule(intro) if intros.contains(&intro))
        })
    }

    /// The derivations of the embedding of sequents of every connective,
    /// translated into every file whose logic proves them, are normal and
    /// pass the checker (which `natural` runs), and a file of a weaker
    /// logic refuses the translation.
    #[test]
    fn translations_are_normal_and_checked() {
        let limits = Limits::default();
        let files = files();
        for text in [
            "a /\\ b |- b /\\ a",
            "a \\/ b |- b \\/ a",
            "a -> b, b -> c |- a -> c",
            "a, ~a |- b",
            "|- ~~(a \\/ ~a)",
            "|- a \\/ ~a",
            "~~a |- a",
            "|- ((a -> b) -> a) -> a",
            "a <-> b, a |- b",
            "|- (a <-> b) <-> (b <-> a)",
            "(a <-> b) /\\ (b <-> c) |- a -> c",
            "|- true",
            "false |- a",
            "~(a \\/ b) |- ~a /\\ ~b",
            "~(a /\\ b) |- ~a \\/ ~b",
            "|- a, ~a",
            "|- a, b, ~b",
            "a -> b |- ~a, b",
            "|- (a -> b) \\/ (b -> a)",
        ] {
            let sequent: Sequent = text.parse().unwrap();
            for logic in [Logic::Minimal, Logic::Intuitionistic, Logic::Classical] {
                if logic != Logic::Classical && sequent.right().len() > 1 {
                    continue;
                }
                let options = Options::default()
                    .with_logic(logic)
                    .with_method(Some(ordinary::Method::Embedding));
                let outcome = ordinary::decide(&sequent, &options).unwrap();
                let Verdict::Valid(evidence) = &outcome.verdict else {
                    continue;
                };
                let source = evidence.derivation().unwrap();
                for calculus in &files {
                    let profile = Profile::of(calculus).unwrap();
                    let translated = Derivation::natural(source, calculus, &limits, |_| false);
                    if strength(profile.logic) < strength(logic) {
                        assert!(
                            matches!(translated, Err(Error::NotApplicable { .. })),
                            "{text} in {logic} into {}",
                            calculus.name()
                        );
                        continue;
                    }
                    let translated = translated.unwrap_or_else(|e| {
                        panic!("{text} in {logic} into {}: {e}", calculus.name())
                    });
                    assert!(
                        normal(&translated),
                        "{text} in {logic} into {}",
                        calculus.name()
                    );
                }
            }
        }
    }

    /// A calculus that is no shipped natural deduction file, here LJ, is
    /// refused, as is a first-order derivation.
    #[test]
    fn other_calculi_and_first_order_derivations_are_refused() {
        let limits = Limits::default();
        let lj = {
            let mut document = serde_json::Deserializer::from_str(super::super::LJ);
            crate::wire::upgrade::<Calculus, _>(&mut document, &limits).unwrap()
        };
        let sequent: Sequent = "a |- a".parse().unwrap();
        let options = Options::default().with_logic(Logic::Intuitionistic);
        let outcome = ordinary::decide(&sequent, &options).unwrap();
        let Verdict::Valid(evidence) = &outcome.verdict else {
            panic!()
        };
        let source = evidence.derivation().unwrap();
        assert!(matches!(
            Derivation::natural(source, &lj, &limits, |_| false),
            Err(Error::NotApplicable { .. })
        ));
        let sequent: Sequent = "forall x. p(x) |- p(c)".parse().unwrap();
        let outcome = ordinary::decide(&sequent, &options).unwrap();
        let Verdict::Valid(evidence) = &outcome.verdict else {
            panic!()
        };
        let source = evidence.derivation().unwrap();
        assert!(matches!(
            Derivation::natural(source, &files()[0], &limits, |_| false),
            Err(Error::FirstOrder { .. })
        ));
    }
}
