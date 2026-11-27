// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The derivation of LJ read off a search that proved its sequent.
//!
//! The search proved `Γ ⊢ C` by a SAT call that found its flat clauses and
//! `¬C` unsatisfiable. Before it, the search learnt clauses `B → c`, each
//! from a call that found `B, a ⊢ b` for an implication `c = a → b` (or
//! `c = ¬a`, `b` being `⊥`). Each flat clause is a formula's own rule read
//! as an implication between its subformulas. So a classical refutation
//! of flat clauses is an intuitionistic derivation (Fiorentini, Goré and
//! Graham-Lengrand, TABLEAUX 2019, Lemma 2). A tableau finds that
//! derivation again: it applies the rules the clauses stand for, forward,
//! and splits on a disjunction only where nothing else applies. Every
//! branch closes, since the clauses entail the goal classically.
//!
//! A learnt clause becomes the formula `b₁ → (… → (bₙ → c))`. It is
//! derived once by `→R` from its tableau and cut below the derivation of
//! the sequent, so that each use is a run of `→L`. Every derivation keeps
//! only the hypotheses it uses: a step whose formulas the derivation above
//! it does not use is left out, and so is a split whose first branch does
//! not use its disjunct.

use super::super::derivation::{self, Builder};
use super::super::{Derivation, Formulas, Inference, Logic, Node, NodeId, Rule, Side};
use super::{Lemma, NEGATIVE, POSITIVE};
use crate::Error;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::linear::proofs::InfId;
use crate::linear::search::memory::Account;

/// How many facts the tableau derives between two polls of the stop.
const POLL: u64 = 1 << 8;

/// Returns the derivation of LJ of `left ⊢ goal` (`left` as written),
/// read from the lemmas the search learnt in the order learnt, and the
/// number of facts the tableaux derived. The derivation's arena gains the
/// lemmas' formulas. `polarity` says which formulas the search's clauses
/// take apart (negative) and build (positive); `held` is the memory the
/// search still holds.
///
/// # Errors
///
/// [`Refusal::Memory`] or [`Refusal::Index`] where a bound refused the
/// derivation, [`Refusal::Work`] past `limits.work` facts, [`Refusal::Stopped`] when the stop fired; [`Error::ReadBack`]
/// where no rule closes a tableau, a defect.
#[expect(
    clippy::too_many_arguments,
    reason = "the search's results, each of its own kind"
)]
pub(super) fn derivation(
    formulas: Formulas,
    logic: Logic,
    polarity: &[u8],
    (falsity, goal): (NodeId, NodeId),
    left: &[NodeId],
    lemmas: &[Lemma],
    limits: &Limits,
    held: u64,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<(Derivation, u64), Error> {
    let mut tableau = Tableau::new(
        formulas, logic, polarity, falsity, lemmas, limits, held, stop,
    )?;
    let mut hypotheses = left.to_vec();
    hypotheses.sort_unstable();
    hypotheses.dedup();
    let mut given = hypotheses.clone();
    given.extend(tableau.lemmas.iter().map(|l| l.formula));
    let mut d = tableau.prove(&given, goal)?;
    for k in (0..tableau.lemmas.len()).rev() {
        let formula = tableau.lemmas[k].formula;
        if hypotheses.binary_search(&formula).is_ok() || !tableau.lj.uses(d, formula) {
            continue;
        }
        let lemma = tableau.lemma(k, &hypotheses)?;
        d = tableau.cut(lemma, d)?;
    }
    let d = tableau.lj.weaken(d, left)?;
    tableau.lj.in_order(d, left);
    let facts = tableau.done;
    let Tableau { formulas, lj, .. } = tableau;
    let inferences = lj.inferences;
    Ok((
        Derivation::from_parts(logic, formulas, (left.to_vec(), vec![goal]), inferences),
        facts,
    ))
}

/// A lemma as the read-back uses it: the clause learnt and its formula.
struct Learnt {
    /// The formula `b₁ → (… → (bₙ → c))` of the clause `B → c`.
    formula: NodeId,
    /// The clause's body `B`, in the order of the formula.
    body: Vec<NodeId>,
    /// `a` of `c = a → b`.
    antecedent: NodeId,
    /// `b` of `c = a → b`, `⊥` for `c = ¬a`.
    consequent: NodeId,
    /// `c`.
    made: NodeId,
}

/// A step of a tableau: a rule applied forward, which made one or two
/// formulas from hypotheses it had.
#[derive(Clone, Copy, Debug)]
struct Step {
    /// The rule: a left rule on `principal`, or a right rule that made
    /// `principal`, applied as a cut whose first premise derives that
    /// formula.
    rule: Rule,
    /// The formula taken apart, or the formula made.
    principal: NodeId,
    /// What the step made; the second is the first again where it made
    /// one.
    made: [NodeId; 2],
}

/// How a branch of the tableau closed.
#[derive(Clone, Copy, Debug)]
enum Close {
    /// The goal is a hypothesis.
    Axiom,
    /// `⊥` is a hypothesis, in intuitionistic logic.
    Falsity,
}

/// A frame of the tableau's search: a branch, from the facts it began
/// with to its split.
#[derive(Clone, Copy, Debug)]
struct Frame {
    /// Where its steps begin.
    steps: usize,
    /// Where its facts begin.
    facts: usize,
    /// The split, once the frame's facts are saturated without closing:
    /// the disjunction and where the frame's own steps and facts end.
    split: Option<Split>,
}

/// A frame's split on a disjunction.
#[derive(Clone, Copy, Debug)]
struct Split {
    /// The disjunction.
    or: NodeId,
    /// Its disjuncts.
    disjuncts: [NodeId; 2],
    /// Where the frame's steps end.
    steps: usize,
    /// Where the frame's facts end.
    facts: usize,
    /// The first branch's derivation, once made.
    first: Option<InfId>,
}

/// The tableaux of one read-back, which share the arena, the rules and the
/// derivation.
struct Tableau<'s, S> {
    /// The arena, with the lemmas' formulas.
    formulas: Formulas,
    /// The logic.
    logic: Logic,
    /// `⊥`.
    falsity: NodeId,
    /// The lemmas, in the order learnt.
    lemmas: Vec<Learnt>,
    /// Whether a hypothesis is taken apart by its left rule.
    eliminated: Vec<bool>,
    /// Whether a formula is made by its right rule.
    introduced: Vec<bool>,
    /// The formulas whose rule may apply once a formula is a hypothesis:
    /// the implications and negations it is the antecedent of, and the
    /// formulas it is a part of that are made by their right rule.
    watchers: Vec<Vec<NodeId>>,
    /// The formulas `⊤` made by their right rule, which needs no
    /// hypothesis.
    truths: Vec<NodeId>,
    /// The negations made by their right rule, which needs `⊥`.
    negations: Vec<NodeId>,
    /// Whether a formula is a hypothesis of the branch.
    have: Vec<bool>,
    /// The hypotheses of the branch, in the order they came.
    facts: Vec<NodeId>,
    /// The steps of the branch.
    steps: Vec<Step>,
    /// The facts derived, the read-back's unit of work.
    done: u64,
    /// The bound on it.
    work: u64,
    /// The derivation made.
    lj: Lj<'s, S>,
}

impl<'s, S: FnMut(Progress) -> bool> Tableau<'s, S> {
    /// Returns the tableaux of a read-back: adds the lemmas' formulas to
    /// the arena and builds the rules' tables.
    #[expect(
        clippy::too_many_arguments,
        reason = "the search's results, each of its own kind"
    )]
    fn new(
        mut formulas: Formulas,
        logic: Logic,
        polarity: &[u8],
        falsity: NodeId,
        lemmas: &[Lemma],
        limits: &Limits,
        held: u64,
        stop: &'s mut S,
    ) -> Result<Self, Error> {
        let mut chains = Vec::new();
        let mut learnt = Vec::with_capacity(lemmas.len());
        for lemma in lemmas {
            let mut formula = lemma.made;
            for &b in lemma.body.iter().rev() {
                formula = formulas.add(Node::Implies(b, formula))?;
                chains.push(formula);
            }
            learnt.push(Learnt {
                formula,
                body: lemma.body.clone(),
                antecedent: lemma.antecedent,
                consequent: lemma.consequent,
                made: lemma.made,
            });
        }
        let len = formulas.len();
        let sign = |id: usize, s: u8| polarity.get(id).is_some_and(|&p| p & s != 0);
        let mut eliminated: Vec<bool> = (0..len).map(|id| sign(id, NEGATIVE)).collect();
        for &c in &chains {
            eliminated[c.index()] = true;
        }
        let introduced: Vec<bool> = (0..len).map(|id| sign(id, POSITIVE)).collect();
        let mut watchers: Vec<Vec<NodeId>> = vec![Vec::new(); len];
        let (mut truths, mut negations) = (Vec::new(), Vec::new());
        for id in 0..len {
            let p = NodeId(id as u32);
            let node = formulas.node(p);
            if eliminated[id] {
                match node {
                    Node::Implies(a, _) | Node::Not(a) => watchers[a.index()].push(p),
                    Node::Atom(_)
                    | Node::True
                    | Node::False
                    | Node::And(..)
                    | Node::Or(..)
                    | Node::Iff(..) => {}
                    Node::Forall(_) | Node::Exists(_) => {
                        unreachable!("{}", crate::ordinary::REFUSED)
                    }
                }
            }
            if introduced[id] {
                match node {
                    Node::And(a, b) | Node::Or(a, b) => {
                        watchers[a.index()].push(p);
                        if b != a {
                            watchers[b.index()].push(p);
                        }
                    }
                    Node::Implies(_, b) => watchers[b.index()].push(p),
                    Node::Iff(a, b) => {
                        for i in [Node::Implies(a, b), Node::Implies(b, a)] {
                            if let Some(i) = formulas.find(i) {
                                watchers[i.index()].push(p);
                            }
                        }
                    }
                    Node::Not(_) => negations.push(p),
                    Node::True => truths.push(p),
                    Node::Atom(_) | Node::False => {}
                    Node::Forall(_) | Node::Exists(_) => {
                        unreachable!("{}", crate::ordinary::REFUSED)
                    }
                }
            }
        }
        let lj = Lj::new(&formulas, limits, held, stop);
        lj.account.charge(
            watchers.iter().map(|w| w.capacity()).sum::<usize>() * size_of::<NodeId>()
                + len * (2 * size_of::<bool>() + size_of::<Vec<NodeId>>()),
        );
        Ok(Self {
            formulas,
            logic,
            falsity,
            lemmas: learnt,
            eliminated,
            introduced,
            watchers,
            truths,
            negations,
            have: vec![false; len],
            facts: Vec::new(),
            steps: Vec::new(),
            done: 0,
            work: limits.work.unwrap_or(u64::MAX),
            lj,
        })
    }

    /// Returns the derivation of lemma `k`'s formula from `hypotheses` and
    /// the formulas of the lemmas learnt before it.
    fn lemma(&mut self, k: usize, hypotheses: &[NodeId]) -> Result<InfId, Error> {
        let lemma = &self.lemmas[k];
        let (a, b, c) = (lemma.antecedent, lemma.consequent, lemma.made);
        let body = lemma.body.clone();
        let mut given = hypotheses.to_vec();
        given.extend(self.lemmas[..k].iter().map(|l| l.formula));
        given.extend_from_slice(&body);
        given.push(a);
        let d = self.prove(&given, b)?;
        let rule = if matches!(self.formulas.node(c), Node::Not(_)) {
            Rule::NotRight
        } else {
            Rule::ImpliesRight
        };
        let mut d = self.lj.right(d, a, c, rule)?;
        let mut made = c;
        for &x in body.iter().rev() {
            made = self
                .formulas
                .find(Node::Implies(x, made))
                .unwrap_or_else(|| unreachable!("the lemma's formula is in the arena"));
            d = self.lj.right(d, x, made, Rule::ImpliesRight)?;
        }
        Ok(d)
    }

    /// Returns the cut of the formula `first` concludes against `second`,
    /// which uses it.
    fn cut(&mut self, first: InfId, second: InfId) -> Result<InfId, Error> {
        let cut = self.lj.goal(first);
        let mut left = self.lj.left(first).to_vec();
        left.extend(without(self.lj.left(second), &[cut]));
        let right = self.lj.goal(second);
        let d = self
            .lj
            .push(left, vec![right], Rule::Cut, None, vec![first, second])?;
        self.lj.tidy(d)
    }

    /// Returns a derivation of `goal` from `given`, or from a part of it,
    /// found by the tableau.
    fn prove(&mut self, given: &[NodeId], goal: NodeId) -> Result<InfId, Error> {
        if matches!(self.formulas.node(goal), Node::True) {
            return self.lj.push(
                Vec::new(),
                vec![goal],
                Rule::TrueRight,
                Some((Side::Right, 0)),
                Vec::new(),
            );
        }
        let mut frames = vec![Frame {
            steps: 0,
            facts: 0,
            split: None,
        }];
        for &x in given {
            self.add(x);
        }
        for t in self.truths.clone() {
            if !self.have[t.index()] {
                self.make(Rule::TrueRight, t, [t, t]);
            }
        }
        let mut next = 0;
        // The derivation of the frame popped last, for the frame below.
        let mut made: Option<InfId> = None;
        let result = loop {
            let Some(&frame) = frames.last() else {
                break made.ok_or_else(|| unreachable!("the root frame's derivation"));
            };
            if let Some(d) = made.take() {
                let mut split = frame.split.unwrap_or_else(|| unreachable!("a split"));
                let [a, b] = split.disjuncts;
                let d = match split.first {
                    None if self.lj.uses(d, a) => {
                        split.first = Some(d);
                        self.undo(split.steps, split.facts);
                        let top = frames.len() - 1;
                        frames[top].split = Some(split);
                        frames.push(Frame {
                            steps: self.steps.len(),
                            facts: self.facts.len(),
                            split: None,
                        });
                        self.add(b);
                        next = self.facts.len() - 1;
                        continue;
                    }
                    None => d,
                    Some(first) if self.lj.uses(d, b) => match self.or_left(split.or, first, d) {
                        Ok(d) => d,
                        Err(e) => break Err(e),
                    },
                    Some(_) => d,
                };
                frames.pop();
                self.undo(split.steps, split.facts);
                made = match self.unwind(frame.steps, split.steps, d) {
                    Ok(d) => Some(d),
                    Err(e) => break Err(e),
                };
                self.undo(frame.steps, frame.facts);
                continue;
            }
            let closed = match self.saturate(&mut next, goal) {
                Ok(closed) => closed,
                Err(e) => break Err(e),
            };
            if let Some(close) = closed {
                let leaf = match close {
                    Close::Axiom => self.lj.axiom(goal),
                    Close::Falsity => self.lj.push(
                        vec![self.falsity],
                        vec![goal],
                        Rule::FalseLeft,
                        Some((Side::Left, 0)),
                        Vec::new(),
                    ),
                };
                let d = match leaf.and_then(|d| self.unwind(frame.steps, self.steps.len(), d)) {
                    Ok(d) => d,
                    Err(e) => break Err(e),
                };
                frames.pop();
                self.undo(frame.steps, frame.facts);
                made = Some(d);
                continue;
            }
            let Some((or, disjuncts)) = self.disjunction() else {
                break Err(Error::ReadBack {
                    calculus: self.logic.calculus(),
                    reason: "no rule closes a branch of the tableau of the SAT search".to_owned(),
                });
            };
            let top = frames.len() - 1;
            frames[top].split = Some(Split {
                or,
                disjuncts,
                steps: self.steps.len(),
                facts: self.facts.len(),
                first: None,
            });
            frames.push(Frame {
                steps: self.steps.len(),
                facts: self.facts.len(),
                split: None,
            });
            self.add(disjuncts[0]);
            next = self.facts.len() - 1;
        };
        // A refusal leaves the branch's facts behind; the tableau is left
        // empty either way.
        self.undo(0, 0);
        result
    }

    /// Takes back the steps and facts from `steps` and `facts` on.
    fn undo(&mut self, steps: usize, facts: usize) {
        self.steps.truncate(steps);
        for x in self.facts.drain(facts..) {
            self.have[x.index()] = false;
        }
    }

    /// Makes `x` a hypothesis of the branch, unless it is one already.
    fn add(&mut self, x: NodeId) {
        if !self.have[x.index()] {
            self.have[x.index()] = true;
            self.facts.push(x);
        }
    }

    /// Records a step and makes what it made hypotheses of the branch.
    fn make(&mut self, rule: Rule, principal: NodeId, made: [NodeId; 2]) {
        self.steps.push(Step {
            rule,
            principal,
            made,
        });
        for x in made {
            self.add(x);
        }
    }

    /// Applies every rule that makes something new to the facts from
    /// `next` on, until none does or a fact closes the branch.
    fn saturate(&mut self, next: &mut usize, goal: NodeId) -> Result<Option<Close>, Error> {
        while let Some(&x) = self.facts.get(*next) {
            *next += 1;
            self.done += 1;
            if self.done > self.work {
                return Err(Error::Refused(Refusal::Work { limit: self.work }));
            }
            if self.done % POLL == 1 && self.lj.stopped(self.done) {
                return Err(Error::Refused(Refusal::stopped(Phase::ReadBack)));
            }
            if x == goal {
                return Ok(Some(Close::Axiom));
            }
            if x == self.falsity && self.logic == Logic::Intuitionistic {
                return Ok(Some(Close::Falsity));
            }
            self.fire(x);
            for k in 0..self.watchers[x.index()].len() {
                let w = self.watchers[x.index()][k];
                self.fire(w);
            }
            if x == self.falsity {
                for k in 0..self.negations.len() {
                    let n = self.negations[k];
                    self.fire(n);
                }
            }
        }
        Ok(None)
    }

    /// Applies the rules of the formula `p` that make something new: its
    /// left rule where it is a hypothesis, its right rule where it is not.
    fn fire(&mut self, p: NodeId) {
        if let Some((rule, made)) = self.rule_of(p) {
            self.make(rule, p, made);
        }
    }

    /// Returns the rule of the formula `p` that makes something new, and
    /// what it makes: its left rule where it is a hypothesis, its right
    /// rule where it is not.
    fn rule_of(&self, p: NodeId) -> Option<(Rule, [NodeId; 2])> {
        let have = |x: NodeId| self.have[x.index()];
        let node = self.formulas.node(p);
        if have(p) && self.eliminated[p.index()] {
            return match node {
                Node::And(a, b) if !have(a) || !have(b) => Some((Rule::AndLeft, [a, b])),
                Node::Iff(a, b) => {
                    let (i, j) = self.implications(a, b);
                    (!have(i) || !have(j)).then_some((Rule::IffLeft, [i, j]))
                }
                Node::Implies(a, b) if have(a) && !have(b) => Some((Rule::ImpliesLeft, [b, b])),
                Node::Not(a) if have(a) && !have(self.falsity) => {
                    Some((Rule::NotLeft, [self.falsity, self.falsity]))
                }
                Node::Atom(_)
                | Node::True
                | Node::False
                | Node::And(..)
                | Node::Or(..)
                | Node::Implies(..)
                | Node::Not(_) => None,
                Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            };
        }
        if have(p) || !self.introduced[p.index()] {
            return None;
        }
        let rule = match node {
            Node::And(a, b) if have(a) && have(b) => Rule::AndRight,
            Node::Or(a, _) if have(a) => Rule::OrRight1,
            Node::Or(_, b) if have(b) => Rule::OrRight2,
            Node::Implies(_, b) if have(b) => Rule::ImpliesRight,
            Node::Not(_) if have(self.falsity) => Rule::NotRight,
            Node::Iff(a, b) => {
                let (i, j) = self.implications(a, b);
                if !(have(i) && have(j)) {
                    return None;
                }
                Rule::IffRight
            }
            Node::True => Rule::TrueRight,
            Node::Atom(_)
            | Node::False
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Not(_) => return None,
            Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
        };
        Some((rule, [p, p]))
    }

    /// Returns the implications `a → b` and `b → a` of `a ↔ b`, which the
    /// arena holds.
    fn implications(&self, a: NodeId, b: NodeId) -> (NodeId, NodeId) {
        let find = |node| {
            self.formulas
                .find(node)
                .unwrap_or_else(|| unreachable!("the search added the implications"))
        };
        (find(Node::Implies(a, b)), find(Node::Implies(b, a)))
    }

    /// Returns the first hypothesis that is a disjunction taken apart by
    /// its left rule, with neither disjunct a hypothesis, and its
    /// disjuncts.
    fn disjunction(&self) -> Option<(NodeId, [NodeId; 2])> {
        self.facts
            .iter()
            .find_map(|&x| match self.formulas.node(x) {
                Node::Or(a, b)
                    if self.eliminated[x.index()]
                        && !self.have[a.index()]
                        && !self.have[b.index()] =>
                {
                    Some((x, [a, b]))
                }
                Node::Atom(_)
                | Node::True
                | Node::False
                | Node::Not(_)
                | Node::And(..)
                | Node::Or(..)
                | Node::Implies(..)
                | Node::Iff(..) => None,
                Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            })
    }

    /// Returns `d` with the steps from `from` to `to` applied below it,
    /// the last first, each only where `d` uses what it made.
    fn unwind(&mut self, from: usize, to: usize, mut d: InfId) -> Result<InfId, Error> {
        for k in (from..to).rev() {
            let step = self.steps[k];
            if !step.made.iter().any(|&x| self.lj.uses(d, x)) {
                continue;
            }
            d = self.apply(step, d)?;
        }
        Ok(d)
    }

    /// Returns `d` with the step applied below it.
    fn apply(&mut self, step: Step, d: InfId) -> Result<InfId, Error> {
        let p = step.principal;
        let goal = self.lj.goal(d);
        let node = self.formulas.node(p);
        match step.rule {
            Rule::AndLeft | Rule::IffLeft => {
                let context = without(self.lj.left(d), &step.made);
                let premise = self
                    .lj
                    .weaken(d, &[&context[..], &step.made[..]].concat())?;
                let mut left = context;
                left.push(p);
                let at = left.len() - 1;
                let d = self.lj.push(
                    left,
                    vec![goal],
                    step.rule,
                    Some((Side::Left, at)),
                    vec![premise],
                )?;
                self.lj.tidy(d)
            }
            Rule::ImpliesLeft | Rule::NotLeft => {
                let (Node::Implies(a, _) | Node::Not(a)) = node else {
                    unreachable!("an implication or a negation")
                };
                let first = self.lj.axiom(a)?;
                let mut left = vec![a];
                left.extend(without(self.lj.left(d), &[step.made[0]]));
                left.push(p);
                let at = left.len() - 1;
                let d = self.lj.push(
                    left,
                    vec![goal],
                    step.rule,
                    Some((Side::Left, at)),
                    vec![first, d],
                )?;
                self.lj.tidy(d)
            }
            Rule::AndRight
            | Rule::OrRight1
            | Rule::OrRight2
            | Rule::ImpliesRight
            | Rule::NotRight
            | Rule::IffRight
            | Rule::TrueRight => {
                let first = self.introduction(step.rule, p)?;
                self.cut(first, d)
            }
            Rule::Axiom
            | Rule::WeakenLeft
            | Rule::WeakenRight
            | Rule::ContractLeft
            | Rule::ContractRight
            | Rule::AndLeft1
            | Rule::AndLeft2
            | Rule::OrLeft
            | Rule::OrRight
            | Rule::IffLeft1
            | Rule::IffLeft2
            | Rule::FalseLeft
            | Rule::FalseRight
            | Rule::TrueLeft
            | Rule::Cut
            | Rule::Open
            | Rule::ForallLeft
            | Rule::ForallRight
            | Rule::ExistsLeft
            | Rule::ExistsRight => unreachable!("no step takes it"),
        }
    }

    /// Returns the derivation of `p` by its right rule from the
    /// hypotheses it is made of.
    fn introduction(&mut self, rule: Rule, p: NodeId) -> Result<InfId, Error> {
        let lj = &mut self.lj;
        let right = Some((Side::Right, 0));
        match (rule, self.formulas.node(p)) {
            (Rule::TrueRight, _) => lj.push(Vec::new(), vec![p], rule, right, Vec::new()),
            (Rule::AndRight | Rule::IffRight, Node::And(a, b) | Node::Iff(a, b)) => {
                let (a, b) = if rule == Rule::IffRight {
                    self.implications(a, b)
                } else {
                    (a, b)
                };
                let lj = &mut self.lj;
                let both = if a == b { vec![a] } else { vec![a, b] };
                let first = lj.axiom(a)?;
                let first = lj.weaken(first, &both)?;
                let second = lj.axiom(b)?;
                let second = lj.weaken(second, &both)?;
                lj.push(both, vec![p], rule, right, vec![first, second])
            }
            (Rule::OrRight1 | Rule::OrRight2, Node::Or(a, b)) => {
                let x = if rule == Rule::OrRight1 { a } else { b };
                let premise = lj.axiom(x)?;
                lj.push(vec![x], vec![p], rule, right, vec![premise])
            }
            (Rule::ImpliesRight, Node::Implies(a, b)) => {
                let premise = lj.axiom(b)?;
                let premise = lj.weaken(premise, &[b, a])?;
                lj.push(vec![b], vec![p], rule, right, vec![premise])
            }
            (Rule::NotRight, Node::Not(a)) => {
                let falsity = self.falsity;
                let premise = lj.axiom(falsity)?;
                let premise = lj.weaken(premise, &[falsity, a])?;
                lj.push(vec![falsity], vec![p], rule, right, vec![premise])
            }
            _ => unreachable!("a right rule of its formula"),
        }
    }

    /// Returns `∨L` on `or` below the branches' derivations, each using
    /// its disjunct.
    fn or_left(&mut self, or: NodeId, first: InfId, second: InfId) -> Result<InfId, Error> {
        let Node::Or(a, b) = self.formulas.node(or) else {
            unreachable!("a disjunction")
        };
        let mut context = without(self.lj.left(first), &[a]);
        for x in without(self.lj.left(second), &[b]) {
            if !context.contains(&x) {
                context.push(x);
            }
        }
        let goal = self.lj.goal(first);
        let first = self.lj.weaken(first, &[&context[..], &[a]].concat())?;
        let second = self.lj.weaken(second, &[&context[..], &[b]].concat())?;
        let mut left = context;
        left.push(or);
        let at = left.len() - 1;
        let d = self.lj.push(
            left,
            vec![goal],
            Rule::OrLeft,
            Some((Side::Left, at)),
            vec![first, second],
        )?;
        self.lj.tidy(d)
    }
}

/// Returns `list` without one copy of each of `out` it holds.
fn without(list: &[NodeId], out: &[NodeId]) -> Vec<NodeId> {
    let mut rest = list.to_vec();
    for x in out {
        if let Some(at) = rest.iter().position(|y| y == x) {
            rest.remove(at);
        }
    }
    rest
}

/// The derivation being made, within the bounds and until the stop
/// fires.
struct Lj<'s, S> {
    /// The inferences made, premises first.
    inferences: Vec<Inference>,
    /// The memory the read-back holds.
    account: Account,
    /// The caller's stop.
    stop: &'s mut S,
}

impl<S: FnMut(Progress) -> bool> Builder for Lj<'_, S> {
    fn inference(&mut self, d: InfId) -> &mut Inference {
        &mut self.inferences[d.index()]
    }

    fn step(
        &mut self,
        left: Vec<NodeId>,
        right: Vec<NodeId>,
        rule: Rule,
        principal: Option<(Side, usize)>,
        premise: InfId,
    ) -> Result<InfId, Error> {
        self.push(left, right, rule, principal, vec![premise])
    }
}

impl<'s, S: FnMut(Progress) -> bool> Lj<'s, S> {
    /// Starts an empty derivation over the arena, within `limits`, while
    /// the search still holds `held` bytes.
    fn new(formulas: &Formulas, limits: &Limits, held: u64, stop: &'s mut S) -> Self {
        let account = Account::holding(limits.memory_bytes, held);
        // The copy of the arena the derivation keeps (a node and its entry
        // in the table of ids). The derivation is bounded by the memory
        // alone: `limits.derivation_bytes` bounds what is drawn.
        account.charge(formulas.len() * 4 * size_of::<Node>());
        Self {
            inferences: Vec::new(),
            account,
            stop,
        }
    }

    /// Returns whether the stop fired, asked with `done` units of work.
    fn stopped(&mut self, done: u64) -> bool {
        (self.stop)(Progress::new(Phase::ReadBack, POLL, done))
    }

    /// Returns the hypotheses `d` concludes.
    fn left(&self, d: InfId) -> &[NodeId] {
        &self.inferences[d.index()].left
    }

    /// Returns the formula right of `⊢` that `d` concludes.
    fn goal(&self, d: InfId) -> NodeId {
        self.inferences[d.index()].right[0]
    }

    /// Returns whether `d` has `x` among its hypotheses.
    fn uses(&self, d: InfId, x: NodeId) -> bool {
        self.left(d).contains(&x)
    }

    /// Adds an inference, within the bounds and until the stop fires.
    fn push(
        &mut self,
        left: Vec<NodeId>,
        right: Vec<NodeId>,
        rule: Rule,
        principal: Option<(Side, usize)>,
        premises: Vec<InfId>,
    ) -> Result<InfId, Error> {
        let n = self.inferences.len() as u64;
        derivation::ask_read_back(&mut *self.stop, n)?;
        self.account
            .charge((left.len() + right.len() + premises.len()) * size_of::<u32>());
        if self.account.over() {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: self.account.limit(),
                needed_bytes: None,
            }));
        }
        let before = self.inferences.capacity();
        let id = derivation::add(
            &mut self.inferences,
            Inference {
                term: None,
                left,
                right,
                rule,
                principal,
                premises,
            },
        )?;
        let size = size_of::<Inference>();
        self.account
            .resize(before * size, self.inferences.capacity() * size);
        Ok(id)
    }

    /// Adds the axiom `a ⊢ a`.
    fn axiom(&mut self, a: NodeId) -> Result<InfId, Error> {
        self.push(vec![a], vec![a], Rule::Axiom, None, Vec::new())
    }

    /// Weakens `d` until it concludes `want` left of `⊢`; `want` holds the
    /// hypotheses of `d`.
    fn weaken(&mut self, d: InfId, want: &[NodeId]) -> Result<InfId, Error> {
        let mut have = self.left(d).to_vec();
        let right = self.inferences[d.index()].right.clone();
        let mut d = d;
        for x in without(want, &have) {
            have.push(x);
            let at = have.len() - 1;
            d = self.push(
                have.clone(),
                right.clone(),
                Rule::WeakenLeft,
                Some((Side::Left, at)),
                vec![d],
            )?;
        }
        Ok(d)
    }

    /// Contracts `d` until it holds each of its hypotheses once.
    fn tidy(&mut self, d: InfId) -> Result<InfId, Error> {
        let mut have = self.left(d).to_vec();
        let right = self.inferences[d.index()].right.clone();
        let mut d = d;
        let mut k = 0;
        while k < have.len() {
            let x = have[k];
            if let Some(at) = have[k + 1..].iter().position(|&y| y == x) {
                have.remove(k + 1 + at);
                d = self.push(
                    have.clone(),
                    right.clone(),
                    Rule::ContractLeft,
                    Some((Side::Left, k)),
                    vec![d],
                )?;
            } else {
                k += 1;
            }
        }
        Ok(d)
    }

    /// Returns the right rule `rule` below `d`: `d` derives what `made` is
    /// made of, and the conclusion moves the antecedent `a` from the
    /// hypotheses into `made`.
    fn right(&mut self, d: InfId, a: NodeId, made: NodeId, rule: Rule) -> Result<InfId, Error> {
        let context = without(self.left(d), &[a]);
        let premise = self.weaken(d, &[&context[..], &[a]].concat())?;
        self.push(
            context,
            vec![made],
            rule,
            Some((Side::Right, 0)),
            vec![premise],
        )
    }
}
