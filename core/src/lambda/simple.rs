// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The read-back of a derivation of LJ as a term of the simply typed
//! lambda calculus, one construction per inference, by a walk from the
//! root with a stack of its own.
//!
//! The hypotheses in scope are kept per formula, the innermost last, as
//! the certificate over `Prop` keeps its names: equal formulas are one id
//! of the arena, and in this calculus any variable of a type may stand
//! for any other, so a contraction is a variable used twice and a
//! weakening one left unused. A left rule binds the formula it adds to
//! what it substitutes (`f M`, `fst(z)`), so that a derivation reads back
//! as a term in normal form, as in the linear read-back.

use super::{Builder, Calculus, NoTerm, Node, Term, TermId, Type, TypeId, VarId, stopped};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress};
use crate::linear::proofs::InfId;
use crate::ordinary::{Derivation, Logic, NodeId, Rule, Side};

/// What the walk has yet to do.
#[derive(Debug)]
enum Task {
    /// Read an inference back.
    Enter(InfId),
    /// Give formulas terms for the premise that follows.
    Bind(Vec<(NodeId, TermId)>),
    /// Take back the terms of the last `Bind` of these formulas.
    Unbind(Vec<NodeId>),
    /// Read a premise back with a formula bound to the application of a
    /// function to the term read last (`→L`'s and `¬L`'s second premise).
    Applied {
        /// The premise.
        inference: InfId,
        /// The formula the application stands for.
        formula: NodeId,
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
    /// `f M`
    Apply(TermId),
    /// `⟨M, N⟩`
    Pair,
    /// `inl(M)` of the sum given.
    Left(TypeId),
    /// `inr(M)` of the sum given.
    Right(TypeId),
    /// `case z of inl(x) → M ‖ inr(y) → N`
    Case(TermId, VarId, VarId),
    /// `abort(M)` of the type given.
    Abort(TypeId),
}

/// The state of one read-back.
struct ReadBack<'d> {
    /// The derivation.
    derivation: &'d Derivation,
    /// The term being built.
    builder: Builder,
    /// The type of every formula met.
    types: HashMap<NodeId, TypeId>,
    /// The terms of the hypotheses in scope, per formula, the innermost
    /// last.
    scope: HashMap<NodeId, Vec<TermId>>,
}

/// Reads a derivation of LJ back as a term, or refuses one of LK or one
/// with open goals, asking `stop` once per inference.
///
/// Memory: at most two nodes and two variables per inference, and the
/// tasks waiting, which lie on one branch: never more than the
/// derivation, which its maker built within its bounds.
pub(crate) fn read(
    derivation: &Derivation,
    _limits: &Limits,
    stop: impl FnMut(Progress) -> bool,
) -> Result<Term, Error> {
    let refuse = |reason| Err(Error::NoTerm { reason });
    if derivation.logic() == Logic::Classical {
        return refuse(NoTerm::Classical);
    }
    if derivation
        .inferences()
        .iter()
        .any(|i| i.rule() == Rule::Open)
    {
        return refuse(NoTerm::Open);
    }
    let mut walk = ReadBack {
        derivation,
        builder: Builder::new(),
        types: HashMap::default(),
        scope: HashMap::default(),
    };
    walk.run(crate::limits::counting(stop, Phase::ReadBack))
}

impl ReadBack<'_> {
    /// Returns the type of a formula, `¬A` as `A → ⊥` and `A ↔ B` as `(A →
    /// B) ∧ (B → A)`, building its operands' first with a stack of its own.
    fn ty(&mut self, id: NodeId) -> TypeId {
        use crate::ordinary::Node as F;
        if let Some(&t) = self.types.get(&id) {
            return t;
        }
        let formulas = self.derivation.formulas();
        let mut stack = vec![id];
        while let Some(&top) = stack.last() {
            let node = formulas.node(top);
            let operands = node.operands();
            let missing: Vec<NodeId> = [operands.0, operands.1]
                .into_iter()
                .flatten()
                .filter(|o| !self.types.contains_key(o))
                .collect();
            if !missing.is_empty() {
                stack.extend(missing);
                continue;
            }
            stack.pop();
            let of = |o: Option<NodeId>| self.types[&o.expect("an operand")];
            let (a, b) = (operands.0, operands.1);
            let ty = match node {
                F::Atom(atom) => {
                    let atom = self.builder.atom(formulas.atom_name(atom));
                    self.builder.ty(Type::Atom { atom })
                }
                F::True => self.builder.ty(Type::Top),
                F::False => self.builder.ty(Type::Zero),
                F::Not(_) => {
                    let to = self.builder.ty(Type::Zero);
                    self.builder.ty(Type::Lollipop { from: of(a), to })
                }
                F::And(..) => self.builder.ty(Type::With {
                    left: of(a),
                    right: of(b),
                }),
                F::Or(..) => self.builder.ty(Type::Plus {
                    left: of(a),
                    right: of(b),
                }),
                F::Implies(..) => self.builder.ty(Type::Lollipop {
                    from: of(a),
                    to: of(b),
                }),
                F::Iff(..) => {
                    let (a, b) = (of(a), of(b));
                    let left = self.builder.ty(Type::Lollipop { from: a, to: b });
                    let right = self.builder.ty(Type::Lollipop { from: b, to: a });
                    self.builder.ty(Type::With { left, right })
                }
            };
            self.types.insert(top, ty);
        }
        self.types[&id]
    }

    /// Returns the type of an inference's term: its goal, or `⊥`.
    fn goal(&mut self, id: InfId) -> TypeId {
        match self.derivation.inference(id).right().first() {
            Some(&c) => self.ty(c),
            None => self.builder.ty(Type::Zero),
        }
    }

    /// Returns the term of the innermost hypothesis of a formula.
    fn of(&self, id: NodeId) -> TermId {
        *self
            .scope
            .get(&id)
            .and_then(|v| v.last())
            .expect("a checked derivation uses only hypotheses in scope")
    }

    /// Returns the implication `A → B` of the derivation's arena, which
    /// holds both of every equivalence.
    fn implication(&self, a: NodeId, b: NodeId) -> NodeId {
        self.derivation
            .formulas()
            .find(crate::ordinary::Node::Implies(a, b))
            .expect("the arena holds an equivalence's implications")
    }

    /// Reads the derivation back from its root.
    fn run(&mut self, mut stop: impl FnMut() -> bool) -> Result<Term, Error> {
        let d = self.derivation;
        let root = d.root();
        let mut context = Vec::new();
        let mut bound = Vec::new();
        for &h in d.inference(root).left() {
            let ty = self.ty(h);
            let (v, t) = self.builder.variable(ty);
            context.push(v);
            bound.push((h, t));
        }
        let goal = self.goal(root);
        let mut tasks = vec![Task::Enter(root), Task::Bind(bound)];
        let mut results: Vec<TermId> = Vec::new();
        while let Some(task) = tasks.pop() {
            match task {
                Task::Enter(id) => {
                    if stop() {
                        return Err(stopped());
                    }
                    self.enter(id, &mut tasks, &mut results);
                }
                Task::Bind(binds) => {
                    for (f, t) in binds {
                        self.scope.entry(f).or_default().push(t);
                    }
                }
                Task::Unbind(formulas) => {
                    for f in formulas {
                        self.scope.get_mut(&f).and_then(Vec::pop);
                    }
                }
                Task::Applied {
                    inference,
                    formula,
                    function,
                } => {
                    let argument = results.pop().expect("the first premise's term");
                    let t = self.builder.node(Node::Apply { function, argument });
                    tasks.push(Task::Unbind(vec![formula]));
                    tasks.push(Task::Enter(inference));
                    tasks.push(Task::Bind(vec![(formula, t)]));
                }
                Task::Make(make) => {
                    let node = Self::make(make, &mut results);
                    let t = self.builder.node(node);
                    results.push(t);
                }
            }
        }
        let root = results.pop().expect("the root's term");
        let builder = std::mem::replace(&mut self.builder, Builder::new());
        Ok(builder.finish(Calculus::Simple, context, goal, root))
    }

    /// Returns the node a `Make` makes of the terms read last.
    fn make(make: Make, results: &mut Vec<TermId>) -> Node {
        let mut pop = || results.pop().expect("a premise's term");
        match make {
            Make::Lambda(variable) => Node::Lambda {
                variable,
                body: pop(),
            },
            Make::Apply(function) => Node::Apply {
                function,
                argument: pop(),
            },
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
            Make::Abort(goal) => Node::False {
                context: Box::new([]),
                term: pop(),
                goal,
            },
        }
    }

    /// Pushes the tasks that read a premise with formulas bound.
    fn premise(tasks: &mut Vec<Task>, id: InfId, binds: Vec<(NodeId, TermId)>) {
        let keys = binds.iter().map(|&(f, _)| f).collect();
        tasks.push(Task::Unbind(keys));
        tasks.push(Task::Enter(id));
        tasks.push(Task::Bind(binds));
    }

    /// Reads one inference back: pushes what its premises and its node
    /// need, or the term of a rule without premises.
    fn enter(&mut self, id: InfId, tasks: &mut Vec<Task>, results: &mut Vec<TermId>) {
        use Rule::*;
        let d = self.derivation;
        let inference = d.inference(id);
        let premises = inference.premises();
        let principal = inference.principal().map(|(side, at)| match side {
            Side::Left => inference.left()[at],
            Side::Right => inference.right()[at],
        });
        let operands = principal.map(|p| d.formulas().node(p).operands());
        let (a, b) = operands.unwrap_or((None, None));
        let a = || a.expect("the rule's first operand");
        let b = || b.expect("the rule's second operand");
        // The term of the principal formula, where it is a hypothesis.
        let z = match inference.principal() {
            Some((Side::Left, _)) => principal.map(|p| self.of(p)),
            _ => None,
        };
        let z = || z.expect("the rule's hypothesis has its term");
        match inference.rule() {
            Axiom => {
                let t = self.of(inference.right()[0]);
                results.push(t);
            }
            WeakenLeft | ContractLeft | TrueLeft | FalseRight => {
                tasks.push(Task::Enter(premises[0]));
            }
            WeakenRight => {
                // A term of `⊥` is its own `abort` into `⊥`.
                let goal = self.goal(id);
                if self.builder.types[goal.index()] != Type::Zero {
                    tasks.push(Task::Make(Make::Abort(goal)));
                }
                tasks.push(Task::Enter(premises[0]));
            }
            TrueRight => {
                let t = self.builder.node(Node::True {
                    context: Box::new([]),
                });
                results.push(t);
            }
            FalseLeft => {
                let goal = self.goal(id);
                let t = if self.builder.types[goal.index()] == Type::Zero {
                    z()
                } else {
                    self.builder.node(Node::False {
                        context: Box::new([]),
                        term: z(),
                        goal,
                    })
                };
                results.push(t);
            }
            AndLeft | AndLeft1 | AndLeft2 | IffLeft | IffLeft1 | IffLeft2 => {
                let iff = matches!(inference.rule(), IffLeft | IffLeft1 | IffLeft2);
                let (x, y) = if iff {
                    (self.implication(a(), b()), self.implication(b(), a()))
                } else {
                    (a(), b())
                };
                let z = z();
                let mut binds = Vec::new();
                if !matches!(inference.rule(), AndLeft2 | IffLeft2) {
                    binds.push((x, self.builder.node(Node::First { term: z })));
                }
                if !matches!(inference.rule(), AndLeft1 | IffLeft1) {
                    binds.push((y, self.builder.node(Node::Second { term: z })));
                }
                Self::premise(tasks, premises[0], binds);
            }
            AndRight | IffRight => {
                tasks.push(Task::Make(Make::Pair));
                tasks.push(Task::Enter(premises[1]));
                tasks.push(Task::Enter(premises[0]));
            }
            OrLeft => {
                let (x, xt) = {
                    let ty = self.ty(a());
                    self.builder.variable(ty)
                };
                let (y, yt) = {
                    let ty = self.ty(b());
                    self.builder.variable(ty)
                };
                tasks.push(Task::Make(Make::Case(z(), x, y)));
                Self::premise(tasks, premises[1], vec![(b(), yt)]);
                Self::premise(tasks, premises[0], vec![(a(), xt)]);
            }
            OrRight1 | OrRight2 => {
                let sum = self.ty(principal.expect("∨R"));
                tasks.push(Task::Make(if inference.rule() == OrRight1 {
                    Make::Left(sum)
                } else {
                    Make::Right(sum)
                }));
                tasks.push(Task::Enter(premises[0]));
            }
            ImpliesRight | NotRight => {
                let ty = self.ty(a());
                let (x, xt) = self.builder.variable(ty);
                tasks.push(Task::Make(Make::Lambda(x)));
                Self::premise(tasks, premises[0], vec![(a(), xt)]);
            }
            NotLeft if premises.len() == 1 => {
                tasks.push(Task::Make(Make::Apply(z())));
                tasks.push(Task::Enter(premises[0]));
            }
            ImpliesLeft | NotLeft => {
                let consequent = if inference.rule() == NotLeft {
                    d.formulas()
                        .find(crate::ordinary::Node::False)
                        .expect("a checked ¬L has ⊥")
                } else {
                    b()
                };
                tasks.push(Task::Applied {
                    inference: premises[1],
                    formula: consequent,
                    function: z(),
                });
                tasks.push(Task::Enter(premises[0]));
            }
            // `read` refuses LK and open goals, which alone have these.
            ContractRight | OrRight | Open => {
                unreachable!("no rule of LJ the read-back takes")
            }
        }
    }
}
