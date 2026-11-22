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
//! what it substitutes (`f M`, `fst(z)`), so that a cut-free derivation
//! reads back as a term in normal form, as in the linear read-back; a cut
//! binds its formula to its first premise's term, which may leave a redex.

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
    /// function to the term read last (`→L`'s and `¬L`'s second premise),
    /// or to that term itself (a cut's second premise).
    Applied {
        /// The premise.
        inference: InfId,
        /// The formula the application stands for.
        formula: NodeId,
        /// The function, `None` for the term itself.
        function: Option<TermId>,
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
/// with open goals, asking `stop` once per inference. A derivation that
/// breaks what the read-back rests on (a hypothesis out of scope, a
/// premise or an operand missing) is [`Error::ReadBack`]: the checker
/// refuses such a derivation, and the crate makes none, so it is a defect.
///
/// `limits` bounds nothing here, since the term is no larger than the
/// derivation, which its maker built within its own bounds. It holds at
/// most two variables per inference and one per hypothesis of the root, a
/// node per variable and at most two more per inference; the tasks
/// waiting lie on one branch.
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
    // The walk indexes premises and the root: each premise comes before
    // its inference (so the walk ends), and there is a root.
    if derivation.inferences().is_empty() {
        return Err(Error::ReadBack {
            calculus: "LJ",
            reason: "the derivation has no inference".to_owned(),
        });
    }
    for (n, inference) in derivation.inferences().iter().enumerate() {
        if inference.premises().iter().any(|p| p.index() >= n) {
            return Err(unreadable(
                InfId::new(n as u32),
                "a premise does not come before it",
            ));
        }
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
    /// B) ∧ (B → A)`.
    fn ty(&mut self, id: NodeId) -> TypeId {
        type_of(
            self.derivation.formulas(),
            &mut self.builder,
            &mut self.types,
            id,
        )
    }

    /// Returns the type of an inference's term: its goal, or `⊥`.
    fn goal(&mut self, id: InfId) -> TypeId {
        match self.derivation.inference(id).right().first() {
            Some(&c) => self.ty(c),
            None => self.builder.ty(Type::Zero),
        }
    }

    /// Returns the term of the innermost hypothesis of a formula, which
    /// inference `at` uses.
    fn of(&self, id: NodeId, at: InfId) -> Result<TermId, Error> {
        self.scope
            .get(&id)
            .and_then(|v| v.last())
            .copied()
            .ok_or_else(|| unreadable(at, "it uses a hypothesis no rule below gives"))
    }

    /// Returns the implication `A → B` of the derivation's arena, which
    /// holds both of every equivalence that inference `at` takes apart.
    fn implication(&self, a: NodeId, b: NodeId, at: InfId) -> Result<NodeId, Error> {
        self.derivation
            .formulas()
            .find(crate::ordinary::Node::Implies(a, b))
            .ok_or_else(|| unreadable(at, "the arena lacks an implication of its equivalence"))
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
                    self.enter(id, &mut tasks, &mut results)?;
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
                    let argument = results
                        .pop()
                        .ok_or_else(|| unreadable(inference, "a premise has no term"))?;
                    let t = match function {
                        Some(function) => self.builder.node(Node::Apply { function, argument }),
                        None => argument,
                    };
                    tasks.push(Task::Unbind(vec![formula]));
                    tasks.push(Task::Enter(inference));
                    tasks.push(Task::Bind(vec![(formula, t)]));
                }
                Task::Make(make) => {
                    let node = Self::make(make, &mut results)
                        .ok_or_else(|| unreadable(root, "a rule has no term for a premise"))?;
                    let t = self.builder.node(node);
                    results.push(t);
                }
            }
        }
        let root = results
            .pop()
            .ok_or_else(|| unreadable(root, "the root has no term"))?;
        let builder = std::mem::replace(&mut self.builder, Builder::new());
        Ok(builder.finish(Calculus::Simple, context, goal, root))
    }

    /// Returns the node a `Make` makes of the terms read last, `None`
    /// where too few were read.
    fn make(make: Make, results: &mut Vec<TermId>) -> Option<Node> {
        let mut pop = || results.pop();
        Some(match make {
            Make::Lambda(variable) => Node::Lambda {
                variable,
                body: pop()?,
            },
            Make::Apply(function) => Node::Apply {
                function,
                argument: pop()?,
            },
            Make::Pair => {
                let right = pop()?;
                Node::Pair {
                    left: pop()?,
                    right,
                }
            }
            Make::Left(sum) => Node::Left { term: pop()?, sum },
            Make::Right(sum) => Node::Right { term: pop()?, sum },
            Make::Case(term, left, right) => {
                let right_body = pop()?;
                Node::Case {
                    term,
                    left,
                    left_body: pop()?,
                    right,
                    right_body,
                }
            }
            Make::Abort(goal) => Node::False {
                context: Box::new([]),
                term: pop()?,
                goal,
            },
        })
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
    fn enter(
        &mut self,
        id: InfId,
        tasks: &mut Vec<Task>,
        results: &mut Vec<TermId>,
    ) -> Result<(), Error> {
        use Rule::*;
        let d = self.derivation;
        let inference = d.inference(id);
        let premises = inference.premises();
        let premise = |k: usize| {
            premises
                .get(k)
                .copied()
                .ok_or_else(|| unreadable(id, "it has fewer premises than its rule"))
        };
        let principal =
            match inference.principal() {
                Some((side, at)) => {
                    Some(inference.side(side).get(at).copied().ok_or_else(|| {
                        unreadable(id, "its principal formula is past its sequent")
                    })?)
                }
                None => None,
            };
        let operands = principal.map(|p| d.formulas().node(p).operands());
        let (a, b) = operands.unwrap_or((None, None));
        let a = || {
            a.ok_or_else(|| unreadable(id, "its principal formula lacks its rule's first operand"))
        };
        let b = || {
            b.ok_or_else(|| unreadable(id, "its principal formula lacks its rule's second operand"))
        };
        // The term of the principal formula, where it is a hypothesis.
        let z = match (inference.principal(), principal) {
            (Some((Side::Left, _)), Some(p)) => Some(self.of(p, id)),
            _ => None,
        };
        let z = || match z.clone() {
            Some(term) => term,
            None => Err(unreadable(
                id,
                "its rule takes a hypothesis apart and names none",
            )),
        };
        match inference.rule() {
            Axiom => {
                let goal = inference.right().first().copied().ok_or_else(|| {
                    unreadable(id, "an axiom has no formula right of the turnstile")
                })?;
                results.push(self.of(goal, id)?);
            }
            WeakenLeft | ContractLeft | TrueLeft | FalseRight => {
                tasks.push(Task::Enter(premise(0)?));
            }
            WeakenRight => {
                // A term of `⊥` is its own `abort` into `⊥`.
                let goal = self.goal(id);
                if self.builder.types.get(goal) != Type::Zero {
                    tasks.push(Task::Make(Make::Abort(goal)));
                }
                tasks.push(Task::Enter(premise(0)?));
            }
            TrueRight => {
                let t = self.builder.node(Node::True {
                    context: Box::new([]),
                });
                results.push(t);
            }
            FalseLeft => {
                let goal = self.goal(id);
                let t = if self.builder.types.get(goal) == Type::Zero {
                    z()?
                } else {
                    let term = z()?;
                    self.builder.node(Node::False {
                        context: Box::new([]),
                        term,
                        goal,
                    })
                };
                results.push(t);
            }
            AndLeft | AndLeft1 | AndLeft2 | IffLeft | IffLeft1 | IffLeft2 => {
                let iff = matches!(inference.rule(), IffLeft | IffLeft1 | IffLeft2);
                let (x, y) = if iff {
                    (
                        self.implication(a()?, b()?, id)?,
                        self.implication(b()?, a()?, id)?,
                    )
                } else {
                    (a()?, b()?)
                };
                let z = z()?;
                let mut binds = Vec::new();
                if !matches!(inference.rule(), AndLeft2 | IffLeft2) {
                    binds.push((x, self.builder.node(Node::First { term: z })));
                }
                if !matches!(inference.rule(), AndLeft1 | IffLeft1) {
                    binds.push((y, self.builder.node(Node::Second { term: z })));
                }
                Self::premise(tasks, premise(0)?, binds);
            }
            AndRight | IffRight => {
                let (first, second) = (premise(0)?, premise(1)?);
                tasks.push(Task::Make(Make::Pair));
                tasks.push(Task::Enter(second));
                tasks.push(Task::Enter(first));
            }
            OrLeft => {
                let (left, right) = (a()?, b()?);
                let (first, second) = (premise(0)?, premise(1)?);
                let (x, xt) = {
                    let ty = self.ty(left);
                    self.builder.variable(ty)
                };
                let (y, yt) = {
                    let ty = self.ty(right);
                    self.builder.variable(ty)
                };
                tasks.push(Task::Make(Make::Case(z()?, x, y)));
                Self::premise(tasks, second, vec![(right, yt)]);
                Self::premise(tasks, first, vec![(left, xt)]);
            }
            OrRight1 | OrRight2 => {
                let goal =
                    principal.ok_or_else(|| unreadable(id, "∨R names no principal formula"))?;
                let sum = self.ty(goal);
                tasks.push(Task::Make(if inference.rule() == OrRight1 {
                    Make::Left(sum)
                } else {
                    Make::Right(sum)
                }));
                tasks.push(Task::Enter(premise(0)?));
            }
            ImpliesRight | NotRight => {
                let antecedent = a()?;
                let ty = self.ty(antecedent);
                let (x, xt) = self.builder.variable(ty);
                tasks.push(Task::Make(Make::Lambda(x)));
                Self::premise(tasks, premise(0)?, vec![(antecedent, xt)]);
            }
            NotLeft if premises.len() == 1 => {
                tasks.push(Task::Make(Make::Apply(z()?)));
                tasks.push(Task::Enter(premise(0)?));
            }
            ImpliesLeft | NotLeft => {
                let consequent = if inference.rule() == NotLeft {
                    d.formulas()
                        .find(crate::ordinary::Node::False)
                        .ok_or_else(|| unreadable(id, "the arena lacks the ⊥ of its ¬L"))?
                } else {
                    b()?
                };
                let (first, second) = (premise(0)?, premise(1)?);
                tasks.push(Task::Applied {
                    inference: second,
                    formula: consequent,
                    function: Some(z()?),
                });
                tasks.push(Task::Enter(first));
            }
            Cut => {
                let (first, second) = (premise(0)?, premise(1)?);
                let formula = d.inference(first).right().first().copied().ok_or_else(|| {
                    unreadable(id, "its cut formula is not right of its first premise")
                })?;
                tasks.push(Task::Applied {
                    inference: second,
                    formula,
                    function: None,
                });
                tasks.push(Task::Enter(first));
            }
            // `read` refuses LK and open goals, which alone have these.
            ContractRight | OrRight | Open => {
                return Err(unreadable(id, "its rule is no rule of LJ"));
            }
            ForallLeft | ForallRight | ExistsLeft | ExistsRight => {
                unreachable!("{}", crate::ordinary::REFUSED)
            }
        }
        Ok(())
    }
}

/// The read-back's refusal of inference `at` of a derivation that breaks
/// what it reads by, for `reason`: [`Error::ReadBack`], a defect where
/// the derivation was checked or made by the crate.
fn unreadable(at: InfId, reason: &str) -> Error {
    Error::ReadBack {
        calculus: "LJ",
        reason: format!("inference {}: {reason}", at.index()),
    }
}

/// Returns the type of a formula of `formulas` in `builder`'s arena,
/// `¬A` as `A → ⊥` and `A ↔ B` as `(A → B) ∧ (B → A)`, building its
/// operands' first with a stack of its own; `types` keeps the type of
/// every formula met.
pub(super) fn type_of(
    formulas: &crate::ordinary::Formulas,
    builder: &mut Builder,
    types: &mut HashMap<NodeId, TypeId>,
    id: NodeId,
) -> TypeId {
    use crate::ordinary::Node as F;
    if let Some(&t) = types.get(&id) {
        return t;
    }
    let mut stack = vec![id];
    while let Some(&top) = stack.last() {
        let node = formulas.node(top);
        let operands = node.operands();
        let missing: Vec<NodeId> = [operands.0, operands.1]
            .into_iter()
            .flatten()
            .filter(|o| !types.contains_key(o))
            .collect();
        if !missing.is_empty() {
            stack.extend(missing);
            continue;
        }
        stack.pop();
        let of = |o: Option<NodeId>| types[&o.expect("an operand")];
        let (a, b) = (operands.0, operands.1);
        let ty = match node {
            F::Atom(atom) => {
                let atom = builder.atom(formulas.atom_name(atom));
                builder.ty(Type::Atom { atom })
            }
            F::Forall(_) | F::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            F::True => builder.ty(Type::Top),
            F::False => builder.ty(Type::Zero),
            F::Not(_) => {
                let to = builder.ty(Type::Zero);
                builder.ty(Type::Lollipop { from: of(a), to })
            }
            F::And(..) => builder.ty(Type::With {
                left: of(a),
                right: of(b),
            }),
            F::Or(..) => builder.ty(Type::Plus {
                left: of(a),
                right: of(b),
            }),
            F::Implies(..) => builder.ty(Type::Lollipop {
                from: of(a),
                to: of(b),
            }),
            F::Iff(..) => {
                let (a, b) = (of(a), of(b));
                let left = builder.ty(Type::Lollipop { from: a, to: b });
                let right = builder.ty(Type::Lollipop { from: b, to: a });
                builder.ty(Type::With { left, right })
            }
        };
        types.insert(top, ty);
    }
    types[&id]
}
