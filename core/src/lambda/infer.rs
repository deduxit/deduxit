// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The principal simple type of an untyped term, by unification.
//!
//! Every variable gets a type variable, `λx. M` the type `α → τ` of its
//! variable's `α` and its body's `τ`, and `M N` a new `β` with the
//! equation that `M`'s type is `N`'s type `→ β` (Hindley 1969; Pierce,
//! *Types and Programming Languages*, 2002, ch. 22). The most general
//! solution of the equations, found by unification with the occurs
//! check, is the principal typing: every type of the term is an instance
//! of it. Equations with no solution make one type a part of itself,
//! and the term then has no simple type.
//!
//! The term is first written out as a tree, every binder a variable of
//! its own, since two places of one subterm may need different types.

use super::reduce::{Halt, budget, copied};
use super::{Calculus, Node, Term, TermId, Type, TypeId, Types, Variable};
use crate::Error;
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Progress, Refusal};

/// A type of the unification: a type variable or a function type.
#[derive(Clone, Copy, Debug)]
enum Shape {
    /// A type variable.
    Variable,
    /// `A → B`, by their indices.
    Arrow(u32, u32),
}

/// The types of the unification, and what each is set equal to.
struct Unifier {
    /// The types.
    shapes: Vec<Shape>,
    /// The type each was set equal to, itself for none.
    parent: Vec<u32>,
    /// The mark of the types the last occurs check visited.
    seen: Vec<u64>,
    /// The mark of the current occurs check.
    mark: u64,
    /// The pairs of function types set equal so far, each once: their
    /// operands are equal once the unification that met them is done.
    unified: HashSet<(u32, u32)>,
}

/// Why a unification failed: a type variable would be part of a type
/// equal to it.
struct Circular {
    /// The type variable.
    variable: u32,
    /// The type it would equal.
    ty: u32,
}

impl Unifier {
    /// Adds a type, and returns its index.
    fn add(&mut self, shape: Shape) -> u32 {
        // The types are fewer than three per node of a term whose nodes
        // a `u32` numbers, and the copy refuses a term near that bound
        // by its memory first.
        let id = self.shapes.len() as u32;
        self.shapes.push(shape);
        self.parent.push(id);
        self.seen.push(0);
        id
    }

    /// Returns the type that `t` was set equal to, through every link.
    fn find(&mut self, t: u32) -> u32 {
        let mut root = t;
        while self.parent[root as usize] != root {
            root = self.parent[root as usize];
        }
        let mut at = t;
        while self.parent[at as usize] != root {
            let next = self.parent[at as usize];
            self.parent[at as usize] = root;
            at = next;
        }
        root
    }

    /// Returns whether the type variable `v` is part of the type `t`.
    fn occurs(
        &mut self,
        v: u32,
        t: u32,
        work: &mut dyn FnMut(u64) -> Result<(), Refusal>,
    ) -> Result<bool, Refusal> {
        self.mark += 1;
        let mut stack = vec![t];
        while let Some(at) = stack.pop() {
            let at = self.find(at);
            if self.seen[at as usize] == self.mark {
                continue;
            }
            self.seen[at as usize] = self.mark;
            work(1)?;
            if at == v {
                return Ok(true);
            }
            if let Shape::Arrow(from, to) = self.shapes[at as usize] {
                stack.extend([to, from]);
            }
        }
        Ok(false)
    }

    /// Sets two types equal, or says which type variable would be part of
    /// a type equal to it.
    fn unify(
        &mut self,
        a: u32,
        b: u32,
        work: &mut dyn FnMut(u64) -> Result<(), Refusal>,
    ) -> Result<Result<(), Circular>, Refusal> {
        let mut pairs = vec![(a, b)];
        while let Some((a, b)) = pairs.pop() {
            work(1)?;
            let (a, b) = (self.find(a), self.find(b));
            if a == b {
                continue;
            }
            match (self.shapes[a as usize], self.shapes[b as usize]) {
                (Shape::Variable, _) | (_, Shape::Variable) => {
                    let (v, t) = if matches!(self.shapes[a as usize], Shape::Variable) {
                        (a, b)
                    } else {
                        (b, a)
                    };
                    if self.occurs(v, t, work)? {
                        return Ok(Err(Circular { variable: v, ty: t }));
                    }
                    self.parent[v as usize] = t;
                }
                // Two function types stay apart in the union-find, each its
                // own representative: linked before their operands were
                // unified, the one linked away would hide its operands
                // from the occurs check, which then let a variable equal
                // a type containing it. The pair is unified once.
                (Shape::Arrow(from_a, to_a), Shape::Arrow(from_b, to_b)) => {
                    if self.unified.insert((a.min(b), a.max(b))) {
                        pairs.extend([(to_a, to_b), (from_a, from_b)]);
                    }
                }
            }
        }
        Ok(Ok(()))
    }
}

/// Returns the name of the `n`th type variable: `a` to `z`, then `a1`.
fn letter(n: usize) -> String {
    let c = char::from(b'a' + (n % 26) as u8);
    match n / 26 {
        0 => c.to_string(),
        index => format!("{c}{index}"),
    }
}

/// The types of the unification's solution, as a term's types.
struct Solution {
    /// The types made, one id per type.
    types: Types,
    /// The atoms' names, a type variable's each.
    atoms: Vec<Box<str>>,
    /// The type made of each type variable or function type, by its
    /// representative.
    made: HashMap<u32, TypeId>,
}

impl Solution {
    /// Returns the type the solution gives `t`, its type variables named
    /// in the order the text writes them.
    fn ty(&mut self, unifier: &mut Unifier, t: u32) -> TypeId {
        let mut stack = vec![(t, false)];
        while let Some((at, ready)) = stack.pop() {
            let at = unifier.find(at);
            if self.made.contains_key(&at) {
                continue;
            }
            match unifier.shapes[at as usize] {
                Shape::Variable => {
                    // Fewer type variables than the term's nodes.
                    let atom = self.atoms.len() as u32;
                    self.atoms.push(letter(self.atoms.len()).into());
                    let id = self.types.add(Type::Atom { atom });
                    self.made.insert(at, id);
                }
                Shape::Arrow(from, to) if ready => {
                    let (from, to) = (unifier.find(from), unifier.find(to));
                    let id = self.types.add(Type::Lollipop {
                        from: self.made[&from],
                        to: self.made[&to],
                    });
                    self.made.insert(at, id);
                }
                Shape::Arrow(from, to) => stack.extend([(at, true), (to, false), (from, false)]),
            }
        }
        self.made[&unifier.find(t)]
    }

    /// Writes a type with its type variables named as they are met, for a
    /// message, cut after a few hundred characters.
    fn write(unifier: &mut Unifier, t: u32, names: &mut HashMap<u32, String>) -> String {
        const SHOWN: usize = 200;
        /// A piece of the text.
        enum Piece {
            /// A type, bracketed or not.
            Type(u32, bool),
            /// Text.
            Text(&'static str),
        }
        let mut out = String::new();
        let mut stack = vec![Piece::Type(t, false)];
        while let Some(piece) = stack.pop() {
            if out.chars().count() > SHOWN {
                out.push('…');
                break;
            }
            match piece {
                Piece::Text(text) => out.push_str(text),
                Piece::Type(at, bracket) => {
                    let at = unifier.find(at);
                    match unifier.shapes[at as usize] {
                        Shape::Variable => {
                            let next = names.len();
                            out.push_str(names.entry(at).or_insert_with(|| letter(next)));
                        }
                        Shape::Arrow(from, to) => {
                            if bracket {
                                stack.push(Piece::Text(")"));
                            }
                            stack.extend([
                                Piece::Type(to, false),
                                Piece::Text(" → "),
                                Piece::Type(from, true),
                            ]);
                            if bracket {
                                stack.push(Piece::Text("("));
                            }
                        }
                    }
                }
            }
        }
        out
    }
}

/// Returns the text of the subterm at `node` of a term, cut after a few
/// hundred characters.
fn subterm(term: &Term, node: TermId) -> String {
    const SHOWN: usize = 200;
    let text = Term {
        root: node,
        ..term.clone()
    }
    .to_string();
    if text.chars().count() > SHOWN {
        let mut cut: String = text.chars().take(SHOWN).collect();
        cut.push('…');
        cut
    } else {
        text
    }
}

impl Term {
    /// Returns the principal simple type of an untyped term, as the term
    /// typed at it: every type of the term in the simply typed lambda
    /// calculus is an instance of that one.
    ///
    /// The term returned is in [`Calculus::Simple`], written out as a tree
    /// with every `λ` carrying its variable's type, its free variables'
    /// types in its [`context`](Self::context) and its type in
    /// [`goal`](Self::goal). The type variables are atoms named `a`, `b`,
    /// `c`, … in the order the judgement writes them. It has passed
    /// [`check`](Self::check).
    ///
    #[cfg_attr(feature = "parse", doc = "```")]
    #[cfg_attr(not(feature = "parse"), doc = "```ignore")]
    /// use deduxit::{Error, Limits};
    /// use deduxit::lambda::{Calculus, Options, Term};
    ///
    /// let limits = Limits::default();
    /// let term = Term::parse(r"\f. \x. f (f x)", Calculus::Untyped, &limits)?;
    /// let typed = term.principal_type(&limits, |_| false)?;
    /// let mut judgement = String::new();
    /// typed.write_judgement(&mut judgement, &Options::default(), &limits, |_| false)?;
    /// assert_eq!(judgement, "⊢ λf. λx. f (f x) : (a → a) → (a → a)");
    ///
    /// // A variable applied to itself has no simple type.
    /// let twice = Term::parse(r"\x. x x", Calculus::Untyped, &limits)?;
    /// let error = twice.principal_type(&limits, |_| false).unwrap_err();
    /// assert!(matches!(error, Error::IllTyped { .. }));
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// - [`Error::NotApplicable`] for a term of a typed calculus, whose
    ///   `λ`s give its type.
    /// - The checker's error for a term its calculus refuses.
    /// - [`Error::IllTyped`] for a term with no simple type, with the
    ///   application whose types would have to contain themselves.
    /// - [`Refusal::Memory`] and [`Refusal::Work`] past the limits, and
    ///   [`Refusal::Stopped`] when `stop` fired.
    /// - [`Error::ClaimFails`] where the checker refuses the term typed:
    ///   a defect of this crate.
    pub fn principal_type(
        &self,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        self.principal_type_in(limits, &mut stop)
    }

    /// Returns the principal type as [`principal_type`](Self::principal_type)
    /// does, the stop a trait object, so that the body is compiled once.
    fn principal_type_in(
        &self,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Self, Error> {
        if self.calculus.is_typed() {
            return Err(Error::NotApplicable {
                operation: "the principal type".into(),
                reason: "the term is typed: its `λ`s give its type".to_owned(),
            });
        }
        self.check(limits, &mut *stop)?;
        let mut budget = budget(limits, stop);
        let (tree, origin) = match copied(self, None, None, &mut budget) {
            Ok(copy) => copy,
            Err(Halt::Refused(refusal)) => return Err(Error::Refused(refusal)),
            Err(Halt::Nodes(_)) => unreachable!("the copy has no bound on nodes"),
        };
        let mut done = 0u64;
        let mut work = |units: u64| {
            done = done.saturating_add(units);
            budget.tick(units, done)
        };
        let mut unifier = Unifier {
            shapes: Vec::with_capacity(tree.variables.len() + 2 * tree.nodes.len()),
            parent: Vec::new(),
            seen: Vec::new(),
            mark: 0,
            unified: HashSet::default(),
        };
        let of_variable: Vec<u32> = (0..tree.variables.len())
            .map(|_| unifier.add(Shape::Variable))
            .collect();
        let mut of: Vec<u32> = Vec::with_capacity(tree.nodes.len());
        for (i, node) in tree.nodes.iter().enumerate() {
            work(1)?;
            let ty = match node {
                Node::Var { variable } => of_variable[variable.index()],
                Node::Lambda { variable, body } => unifier.add(Shape::Arrow(
                    of_variable[variable.index()],
                    of[body.index()],
                )),
                Node::Apply { function, argument } => {
                    let result = unifier.add(Shape::Variable);
                    let wanted = unifier.add(Shape::Arrow(of[argument.index()], result));
                    if let Err(circular) = unifier.unify(of[function.index()], wanted, &mut work)? {
                        let mut names = HashMap::default();
                        let variable = Solution::write(&mut unifier, circular.variable, &mut names);
                        let ty = Solution::write(&mut unifier, circular.ty, &mut names);
                        // A tree's nodes are numbered by `u32`, and so are
                        // the term's it was copied from.
                        let node = origin[i];
                        return Err(Error::IllTyped {
                            reason: format!(
                                "in `{}`, the types would have to satisfy {variable} = \
                                 {ty}, and no type contains itself",
                                subterm(&tree, TermId(i as u32)),
                            ),
                            node: Some(node.0),
                            span: None,
                        });
                    }
                    result
                }
                Node::Tensor { .. }
                | Node::LetTensor { .. }
                | Node::Unit
                | Node::LetUnit { .. }
                | Node::Pair { .. }
                | Node::First { .. }
                | Node::Second { .. }
                | Node::Left { .. }
                | Node::Right { .. }
                | Node::Case { .. }
                | Node::True { .. }
                | Node::False { .. }
                | Node::Promote { .. }
                | Node::Derelict { .. }
                | Node::Discard { .. }
                | Node::Copy { .. } => {
                    unreachable!("the check refuses every node an untyped calculus lacks")
                }
            };
            of.push(ty);
        }
        let mut solution = Solution {
            types: Types::default(),
            atoms: Vec::new(),
            made: HashMap::default(),
        };
        // The type variables are named in the order the judgement writes
        // them: the context, then the term's type, then the binders'.
        for &v in &tree.context {
            solution.ty(&mut unifier, of_variable[v.index()]);
        }
        let goal = solution.ty(&mut unifier, of[tree.root.index()]);
        let variables = tree
            .variables
            .iter()
            .zip(&of_variable)
            .map(|(variable, &t)| Variable {
                ty: Some(solution.ty(&mut unifier, t)),
                name: variable.name.clone(),
            })
            .collect();
        let typed = Self {
            calculus: Calculus::Simple,
            atoms: solution.atoms,
            types: solution.types.list,
            variables,
            context: tree.context,
            goal: Some(goal),
            nodes: tree.nodes,
            root: tree.root,
        };
        match typed.check(limits, &mut *budget.stop) {
            Ok(_) => Ok(typed),
            Err(refused @ Error::Refused(_)) => Err(refused),
            Err(error) => Err(Error::ClaimFails {
                claim: "the principal type types the term".to_owned(),
                reason: error.to_string(),
            }),
        }
    }
}
