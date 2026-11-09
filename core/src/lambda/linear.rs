// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The read-back of a two-sided derivation of intuitionistic linear
//! logic as a term of the linear (or affine) lambda calculus, one
//! construction per inference, by a walk from the root with a stack of
//! its own.
//!
//! Every hypothesis of every sequent stands for a term: a variable bound
//! below, or what a left rule substituted for it (`f M`, `fst(z)`,
//! `derelict(z)`), shared wherever the hypothesis goes. The walk hands
//! each premise the terms of its hypotheses, matched to the conclusion's
//! by member: a member is an occurrence, and equal members are copies of
//! one formula, so any of them may take any copy's term. A context split
//! between two premises takes each term once; an additive rule gives both
//! premises every term.

use super::{Builder, Calculus, NoTerm, Node, Term, TermId, Type, TypeId, VarId, stopped};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::occurrences::{IllConnective, Member, OccId, Reading, Side};
use crate::linear::proofs::{Derivation, InfId, Inference, Rule};

/// What the walk has yet to do.
#[derive(Debug)]
enum Task {
    /// Read an inference back, its hypotheses' terms by position in its
    /// sequent (`None` at the goal).
    Enter(InfId, Vec<Option<TermId>>),
    /// Read an inference back once the hypothesis at a position is the
    /// application of a function to the term read last (`⊸L`'s second
    /// premise).
    Applied {
        /// The premise.
        inference: InfId,
        /// Its hypotheses' terms, the position's to be made.
        terms: Vec<Option<TermId>>,
        /// The position of the hypothesis the application is.
        at: usize,
        /// The function.
        function: TermId,
    },
    /// Make a node of the terms read last.
    Make(Make),
}

/// A node to make of the terms read last, the last read the rightmost.
#[derive(Debug)]
enum Make {
    /// `λx:A. M`
    Lambda(VarId),
    /// `M ⊗ N`
    Tensor,
    /// `let z be x ⊗ y in M`
    LetTensor(TermId, VarId, VarId),
    /// `let z be ∗ in M`
    LetUnit(TermId),
    /// `⟨M, N⟩`
    Pair,
    /// `inl(M)` of the sum given.
    Left(TypeId),
    /// `inr(M)` of the sum given.
    Right(TypeId),
    /// `case z of inl(x) → M ‖ inr(y) → N`
    Case(TermId, VarId, VarId),
    /// `promote M⃗ for x⃗ in N`
    Promote(Box<[TermId]>, Box<[VarId]>),
    /// `discard z in M`
    Discard(TermId),
    /// `copy z as x, y in M`
    Copy(TermId, VarId, VarId),
    /// `(λx:A. N) M` of the cut's two premises, `M` read first.
    Cut(VarId),
}

/// The terms of a conclusion's hypotheses that its premises take, by
/// member.
type Pool = HashMap<Member, Vec<TermId>>;

/// The state of one read-back.
struct ReadBack<'d, 'a> {
    /// The derivation.
    derivation: &'d Derivation<'a>,
    /// Its intuitionistic reading.
    reading: &'d Reading<'a>,
    /// The term being built.
    builder: Builder,
    /// The type of every occurrence met.
    types: HashMap<OccId, TypeId>,
}

/// Reads a two-sided derivation back as a term, or refuses one that has
/// none, asking `stop` once per inference.
///
/// Memory: the term holds at most two nodes per inference, one per
/// member a structural run of a compact derivation weakens or copies, and
/// a node per variable, each variable bound to a member: never more nodes
/// than the derivation has members. The tasks
/// waiting hold the terms of the sequents of the premises not yet read,
/// which lie on one branch: never more than the derivation itself, which
/// was built within `limits` before it reached here. The walk counts on
/// no integer but the ids, which count less than the derivation's
/// inferences and members, fewer than 2³² each.
pub(crate) fn read(
    derivation: &Derivation<'_>,
    _limits: &Limits,
    stop: impl FnMut(Progress) -> bool,
) -> Result<Term, Error> {
    let refuse = |reason| Err(Error::NoTerm { reason });
    let inferences = derivation.inferences();
    let mode = derivation.mode();
    if mode.is_some_and(crate::linear::Mode::is_ordered)
        || derivation.reading().is_some_and(Reading::is_ordered)
    {
        return refuse(NoTerm::Ordered);
    }
    if mode.is_some_and(crate::linear::Mode::has_mix)
        || inferences.iter().any(|i| i.rule().rule == Rule::Mix)
    {
        return refuse(NoTerm::Mix);
    }
    if derivation.framing().is_some()
        || derivation.forest().has_binders()
        || inferences
            .iter()
            .any(|i| matches!(i.rule().rule, Rule::Forall | Rule::Exists))
    {
        return refuse(NoTerm::Quantified);
    }
    let Some(reading) = derivation.reading() else {
        return refuse(NoTerm::Classical);
    };
    if inferences.iter().any(|i| i.rule().rule == Rule::Open) {
        return refuse(NoTerm::Open);
    }
    let affine = inferences
        .iter()
        .any(|i| i.rule().rule == Rule::AffineWeakening);
    let mut walk = ReadBack {
        derivation,
        reading,
        builder: Builder::new(),
        types: HashMap::default(),
    };
    walk.run(affine, crate::limits::counting(stop, Phase::ReadBack))
}

impl ReadBack<'_, '_> {
    /// Returns the type of the formula at `o` as the reading reads it,
    /// building its operands' first with a stack of its own.
    fn ty(&mut self, o: OccId) -> TypeId {
        if let Some(&t) = self.types.get(&o) {
            return t;
        }
        let forest = self.reading.forest();
        let mut stack = vec![o];
        while let Some(&top) = stack.last() {
            let operands: Vec<OccId> = match self.reading.connective(top) {
                IllConnective::Lollipop => {
                    let (from, to) = self.reading.implication(top).expect("an implication");
                    vec![from, to]
                }
                IllConnective::Atom
                | IllConnective::One
                | IllConnective::Zero
                | IllConnective::Top
                | IllConnective::Bang
                | IllConnective::Tensor
                | IllConnective::With
                | IllConnective::Plus
                | IllConnective::Under
                | IllConnective::Over
                | IllConnective::Forall
                | IllConnective::Exists => forest.children(top).collect(),
            };
            let missing: Vec<OccId> = operands
                .iter()
                .copied()
                .filter(|p| !self.types.contains_key(p))
                .collect();
            if !missing.is_empty() {
                stack.extend(missing);
                continue;
            }
            stack.pop();
            let of = |i: usize| self.types[&operands[i]];
            let ty = match self.reading.connective(top) {
                IllConnective::Atom => {
                    let atom = forest.atom(top).expect("an atom");
                    Type::Atom {
                        atom: self.builder.atom(forest.sequent().atom_name(atom)),
                    }
                }
                IllConnective::One => Type::One,
                IllConnective::Zero => Type::Zero,
                IllConnective::Top => Type::Top,
                IllConnective::Bang => Type::Bang { of: of(0) },
                IllConnective::Tensor => Type::Tensor {
                    left: of(0),
                    right: of(1),
                },
                IllConnective::With => Type::With {
                    left: of(0),
                    right: of(1),
                },
                IllConnective::Plus => Type::Plus {
                    left: of(0),
                    right: of(1),
                },
                IllConnective::Lollipop => Type::Lollipop {
                    from: of(0),
                    to: of(1),
                },
                // `read` refuses the ordered readings and the quantifiers.
                IllConnective::Under
                | IllConnective::Over
                | IllConnective::Forall
                | IllConnective::Exists => unreachable!("no type of the calculus"),
            };
            let id = self.builder.ty(ty);
            self.types.insert(top, id);
        }
        self.types[&o]
    }

    /// Returns a new variable of the type of the formula at `o`, and the
    /// node that uses it.
    fn variable(&mut self, o: OccId) -> (VarId, TermId) {
        let ty = self.ty(o);
        self.builder.variable(ty)
    }

    /// Returns whether a member is a hypothesis, in input position.
    fn is_input(&self, m: Member) -> bool {
        self.reading.position(m.occ()) == Side::Input
    }

    /// Returns the terms of an inference's hypotheses but the one at
    /// `skip`, by member.
    fn pool(&self, inference: &Inference, terms: &[Option<TermId>], skip: Option<usize>) -> Pool {
        let mut pool = Pool::default();
        for (p, (&m, t)) in inference.sequent().iter().zip(terms).enumerate() {
            if let Some(t) = t
                && Some(p) != skip
            {
                pool.entry(m).or_default().push(*t);
            }
        }
        pool
    }

    /// Returns the terms of a premise's hypotheses: each of `new` once,
    /// for a member the rule added, and the others taken from the pool.
    /// A member the pool lacks has no term yet (`None`), which only a
    /// position the caller fills next may be.
    fn premise(&self, id: InfId, pool: &mut Pool, new: &[(Member, TermId)]) -> Vec<Option<TermId>> {
        let mut new = new.to_vec();
        self.derivation
            .inference(id)
            .sequent()
            .iter()
            .map(|&m| {
                if !self.is_input(m) {
                    return None;
                }
                if let Some(i) = new.iter().position(|&(n, _)| n == m) {
                    return Some(new.swap_remove(i).1);
                }
                pool.get_mut(&m).and_then(Vec::pop)
            })
            .collect()
    }

    /// Reads the derivation back from its root.
    fn run(&mut self, affine: bool, mut stop: impl FnMut() -> bool) -> Result<Term, Error> {
        let d = self.derivation;
        let root = d.root();
        let mut context = Vec::new();
        let mut terms = Vec::new();
        let mut goal = None;
        for &m in d.inference(root).sequent() {
            let ty = self.ty(m.occ());
            if self.is_input(m) {
                let (v, t) = self.builder.variable(ty);
                context.push(v);
                terms.push(Some(t));
            } else {
                goal = Some(ty);
                terms.push(None);
            }
        }
        let goal = goal.expect("a two-sided sequent has its goal");
        let mut tasks = vec![Task::Enter(root, terms)];
        let mut results: Vec<TermId> = Vec::new();
        while let Some(task) = tasks.pop() {
            match task {
                Task::Enter(id, terms) => {
                    if stop() {
                        return Err(stopped());
                    }
                    self.enter(id, &terms, &mut tasks, &mut results);
                }
                Task::Applied {
                    inference,
                    mut terms,
                    at,
                    function,
                } => {
                    let argument = results.pop().expect("the first premise's term");
                    let applied = self.builder.node(Node::Apply { function, argument });
                    terms[at] = Some(applied);
                    tasks.push(Task::Enter(inference, terms));
                }
                Task::Make(make) => {
                    let node = self.make(make, &mut results);
                    let t = self.builder.node(node);
                    results.push(t);
                }
            }
        }
        let root = results.pop().expect("the root's term");
        let calculus = if affine {
            Calculus::Affine
        } else {
            Calculus::Linear
        };
        let builder = std::mem::replace(&mut self.builder, Builder::new());
        Ok(builder.finish(calculus, context, goal, root))
    }

    /// Returns the node a `Make` makes of the terms read last.
    fn make(&mut self, make: Make, results: &mut Vec<TermId>) -> Node {
        let mut pop = || results.pop().expect("a premise's term");
        match make {
            Make::Lambda(variable) => Node::Lambda {
                variable,
                body: pop(),
            },
            Make::Tensor => {
                let right = pop();
                Node::Tensor { left: pop(), right }
            }
            Make::LetTensor(term, left, right) => Node::LetTensor {
                term,
                left,
                right,
                body: pop(),
            },
            Make::LetUnit(term) => Node::LetUnit { term, body: pop() },
            Make::Pair => {
                let right = pop();
                Node::Pair { left: pop(), right }
            }
            Make::Left(sum) => Node::Left { term: pop(), sum },
            Make::Right(sum) => Node::Right { term: pop(), sum },
            Make::Case(term, left, right) => {
                let right_body = pop();
                Node::Case {
                    term,
                    left,
                    left_body: pop(),
                    right,
                    right_body,
                }
            }
            Make::Promote(terms, variables) => Node::Promote {
                terms,
                variables,
                body: pop(),
            },
            Make::Discard(term) => Node::Discard { term, body: pop() },
            Make::Copy(term, left, right) => Node::Copy {
                term,
                left,
                right,
                body: pop(),
            },
            Make::Cut(variable) => {
                let body = pop();
                let argument = pop();
                let function = self.builder.node(Node::Lambda { variable, body });
                Node::Apply { function, argument }
            }
        }
    }

    /// Reads one inference back: pushes what its premises and its node
    /// need, or the term of a rule without premises.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per rule of the calculus, each a few lines"
    )]
    fn enter(
        &mut self,
        id: InfId,
        terms: &[Option<TermId>],
        tasks: &mut Vec<Task>,
        results: &mut Vec<TermId>,
    ) {
        use Side::{Input, Output};
        let d = self.derivation;
        let forest = self.reading.forest();
        let inference = d.inference(id);
        let named = inference.rule();
        let premises = inference.premises();
        let at = inference.principal();
        let principal = at.map(|p| inference.sequent()[p]);
        let o = principal.map(Member::occ);
        let z = at.and_then(|p| terms[p]);
        // The principal occurrence's operands, and their members.
        let left = o.and_then(|o| forest.left(o));
        let right = o.and_then(|o| forest.right(o));
        let member = |o: Option<OccId>| Member::from(o.expect("the rule's operand"));
        let mut pool = self.pool(inference, terms, at);
        let z = || z.expect("the principal hypothesis has its term");
        match (named.rule, named.side) {
            (Rule::Ax, _) => {
                let t = terms
                    .iter()
                    .flatten()
                    .next()
                    .expect("the axiom's hypothesis");
                results.push(*t);
            }
            (Rule::Par, Some(Output)) => {
                let (from, _) = self.reading.implication(o.expect("⊸R")).expect("⊸R");
                let (x, xt) = self.variable(from);
                let terms = self.premise(premises[0], &mut pool, &[(Member::from(from), xt)]);
                tasks.push(Task::Make(Make::Lambda(x)));
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::Tensor, Some(Input)) => {
                let (_, to) = self.reading.implication(o.expect("⊸L")).expect("⊸L");
                let first = self.premise(premises[0], &mut pool, &[]);
                let second = self.premise(premises[1], &mut pool, &[]);
                let to = Member::from(to);
                let at = d
                    .inference(premises[1])
                    .sequent()
                    .iter()
                    .zip(&second)
                    .position(|(&m, t)| m == to && t.is_none())
                    .expect("the consequent is a hypothesis of the second premise");
                tasks.push(Task::Applied {
                    inference: premises[1],
                    terms: second,
                    at,
                    function: z(),
                });
                tasks.push(Task::Enter(premises[0], first));
            }
            (Rule::Par, Some(Input)) => {
                let (x, xt) = self.variable(left.expect("⊗L"));
                let (y, yt) = self.variable(right.expect("⊗L"));
                let new = [(member(left), xt), (member(right), yt)];
                let terms = self.premise(premises[0], &mut pool, &new);
                tasks.push(Task::Make(Make::LetTensor(z(), x, y)));
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::Tensor, Some(Output)) => {
                let first = self.premise(premises[0], &mut pool, &[]);
                let second = self.premise(premises[1], &mut pool, &[]);
                tasks.push(Task::Make(Make::Tensor));
                tasks.push(Task::Enter(premises[1], second));
                tasks.push(Task::Enter(premises[0], first));
            }
            (Rule::One, _) => {
                let t = self.builder.node(Node::Unit);
                results.push(t);
            }
            (Rule::Bot, _) => {
                let terms = self.premise(premises[0], &mut pool, &[]);
                tasks.push(Task::Make(Make::LetUnit(z())));
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::With, Some(Output)) => {
                let first = self.premise(premises[0], &mut pool.clone(), &[]);
                let second = self.premise(premises[1], &mut pool, &[]);
                tasks.push(Task::Make(Make::Pair));
                tasks.push(Task::Enter(premises[1], second));
                tasks.push(Task::Enter(premises[0], first));
            }
            (Rule::PlusLeft | Rule::PlusRight, Some(Input)) => {
                let (kept, node) = if named.rule == Rule::PlusLeft {
                    (left, Node::First { term: z() })
                } else {
                    (right, Node::Second { term: z() })
                };
                let t = self.builder.node(node);
                let terms = self.premise(premises[0], &mut pool, &[(member(kept), t)]);
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::With, Some(Input)) => {
                let (x, xt) = self.variable(left.expect("⊕L"));
                let (y, yt) = self.variable(right.expect("⊕L"));
                let first = self.premise(premises[0], &mut pool.clone(), &[(member(left), xt)]);
                let second = self.premise(premises[1], &mut pool, &[(member(right), yt)]);
                tasks.push(Task::Make(Make::Case(z(), x, y)));
                tasks.push(Task::Enter(premises[1], second));
                tasks.push(Task::Enter(premises[0], first));
            }
            (Rule::PlusLeft | Rule::PlusRight, Some(Output)) => {
                let sum = self.ty(o.expect("⊕R"));
                let terms = self.premise(premises[0], &mut pool, &[]);
                tasks.push(Task::Make(if named.rule == Rule::PlusLeft {
                    Make::Left(sum)
                } else {
                    Make::Right(sum)
                }));
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::Top, Some(Output)) => {
                let context = terms.iter().flatten().copied().collect();
                let t = self.builder.node(Node::True { context });
                results.push(t);
            }
            (Rule::Top, Some(Input)) => {
                let at = at.expect("0L");
                let context = terms
                    .iter()
                    .enumerate()
                    .filter(|&(p, _)| p != at)
                    .filter_map(|(_, t)| *t)
                    .collect();
                let goal = inference
                    .sequent()
                    .iter()
                    .find(|&&m| !self.is_input(m))
                    .expect("a two-sided sequent has its goal");
                let goal = self.ty(goal.occ());
                let t = self.builder.node(Node::False {
                    context,
                    term: z(),
                    goal,
                });
                results.push(t);
            }
            (Rule::Dereliction, _) => {
                let t = self.builder.node(Node::Derelict { term: z() });
                let terms = self.premise(premises[0], &mut pool, &[(member(left), t)]);
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::Promotion, _) => {
                let mut bound = Vec::new();
                let mut variables = Vec::new();
                let mut promoted = Vec::new();
                for (&m, t) in inference.sequent().iter().zip(terms) {
                    if let Some(t) = t {
                        let (v, vt) = self.variable(m.occ());
                        promoted.push(*t);
                        variables.push(v);
                        bound.push((m, vt));
                    }
                }
                let terms = self.premise(premises[0], &mut Pool::default(), &bound);
                tasks.push(Task::Make(Make::Promote(
                    promoted.into_boxed_slice(),
                    variables.into_boxed_slice(),
                )));
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::Weakening | Rule::AffineWeakening, _) => {
                let mut pool = self.pool(inference, terms, None);
                let terms = self.premise(premises[0], &mut pool, &[]);
                // What the premise did not take is weakened: discarded
                // under `!`, unused in the affine calculus.
                if named.rule == Rule::Weakening {
                    let mut left: Vec<TermId> = pool.into_values().flatten().collect();
                    left.sort_unstable();
                    for t in left {
                        tasks.push(Task::Make(Make::Discard(t)));
                    }
                }
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::Contraction, _) => {
                let mut pool = self.pool(inference, terms, None);
                let premise = d.inference(premises[0]);
                let mut needed: HashMap<Member, usize> = HashMap::default();
                for &m in premise.sequent() {
                    if self.is_input(m) {
                        *needed.entry(m).or_default() += 1;
                    }
                }
                let mut copies = Vec::new();
                let mut members: Vec<(Member, usize)> = needed.into_iter().collect();
                members.sort_unstable();
                for (m, count) in members {
                    let have = pool.get(&m).map_or(0, Vec::len);
                    if count <= have {
                        continue;
                    }
                    let entry = pool
                        .get_mut(&m)
                        .expect("a contracted formula is in the conclusion");
                    let mut current = entry.pop().expect("a contracted formula has its term");
                    let ty = self.ty(m.occ());
                    for _ in have..count {
                        let (x, xt) = self.builder.variable(ty);
                        let (y, yt) = self.builder.variable(ty);
                        copies.push(Make::Copy(current, x, y));
                        let entry = pool.get_mut(&m).expect("present");
                        entry.push(xt);
                        current = yt;
                    }
                    pool.get_mut(&m).expect("present").push(current);
                }
                let terms = self.premise(premises[0], &mut pool, &[]);
                for copy in copies {
                    tasks.push(Task::Make(copy));
                }
                tasks.push(Task::Enter(premises[0], terms));
            }
            (Rule::Cut, _) => {
                let a = inference.cut().expect("a cut names its formula").occ();
                let dual = forest.dual(a).expect("a cut formula has its dual");
                let (x, xt) = self.variable(dual);
                let first = self.premise(premises[0], &mut pool, &[]);
                let second = self.premise(premises[1], &mut pool, &[(Member::from(dual), xt)]);
                tasks.push(Task::Make(Make::Cut(x)));
                tasks.push(Task::Enter(premises[1], second));
                tasks.push(Task::Enter(premises[0], first));
            }
            // `read` refuses these, and a two-sided derivation names every
            // other rule with its side.
            (Rule::Mix | Rule::Forall | Rule::Exists | Rule::Open, _)
            | (
                Rule::Par
                | Rule::Tensor
                | Rule::With
                | Rule::PlusLeft
                | Rule::PlusRight
                | Rule::Top,
                None,
            ) => unreachable!("no rule of a two-sided derivation the read-back takes"),
        }
    }
}
