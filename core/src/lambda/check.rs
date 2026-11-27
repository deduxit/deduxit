// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The type checker of lambda-terms: one pass over a term's nodes that
//! types each from its operands, counts how often each variable is used,
//! and checks both against the term's calculus.
//!
//! A variable has its type as part of it, so a node's type depends on
//! the node alone and a node that several others name is typed once.
//! How often a variable is used depends on the place, so a shared node's
//! uses count at every place that names it.

use super::print::{UNICODE, write_name, write_type};
use super::{Calculus, Node, Term, TermId, Type, TypeId, TypeVarId, Types, VarId, Variable, poly};
use crate::Error;
use crate::hash::{HashMap, HashSet};
use crate::limits::{Limits, Phase, Progress, Refusal};
use std::fmt::Write;

/// The type of every node of a checked term: its typing derivation.
///
/// [`Term::check`] makes one. A term of an untyped calculus has no
/// types, and its typing is empty. A term's types are in the term, on its
/// binders, so a term has one derivation at most, and it has the term's
/// shape: the rule at a node is the node's kind, its premises are the
/// node's subterms, and its conclusion says that the node has the type
/// [`ty`](Self::ty) gives.
///
/// The types are [`types`](Self::types): the term's own, at the ids the
/// term gives them, followed by those the rules made.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Typing {
    /// The term's types, then those the rules made, operands first.
    types: Vec<Type>,
    /// The type of every node.
    #[cfg_attr(feature = "serialize", serde(rename = "nodes"))]
    of: Vec<TypeId>,
}

impl Typing {
    /// Returns the types the typing names: the term's, then those the
    /// rules made, each after its operands.
    pub fn types(&self) -> &[Type] {
        &self.types
    }

    /// Returns the type of a node of the term, or `None` for a term of
    /// an untyped calculus, whose typing has no types.
    ///
    /// # Panics
    ///
    /// If `node` is past the nodes of a typed term, as an id of another
    /// term may be.
    pub fn ty(&self, node: TermId) -> Option<TypeId> {
        if self.of.is_empty() {
            None
        } else {
            Some(self.of[node.index()])
        }
    }
}

/// Why a node is ill typed. [`describe`] says it in words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Fault {
    /// A node or a type that the term's calculus does not have.
    Foreign {
        /// What it is.
        what: &'static str,
    },
    /// A term has another type than its place needs.
    Shape {
        /// The place.
        of: &'static str,
        /// What the place needs.
        needs: &'static str,
        /// The type the term has.
        found: TypeId,
    },
    /// An argument has another type than its function takes.
    Argument {
        /// The type the function takes.
        expected: TypeId,
        /// The argument's type.
        found: TypeId,
    },
    /// An injection's term has another type than its side of the sum.
    Injection {
        /// The sum.
        sum: TypeId,
        /// The side's type.
        expected: TypeId,
        /// The term's type.
        found: TypeId,
    },
    /// A pattern's variable has another type than the part it names.
    Pattern {
        /// The variable.
        variable: VarId,
        /// The part's type.
        expected: TypeId,
    },
    /// A pattern names one variable for two of its parts.
    Twice {
        /// The variable.
        variable: VarId,
    },
    /// The branches of a case have different types.
    Branches {
        /// The first branch's type.
        left: TypeId,
        /// The second branch's type.
        right: TypeId,
    },
    /// A `promote` has another number of terms than of variables.
    Promoted {
        /// How many terms.
        terms: usize,
        /// How many variables.
        variables: usize,
    },
    /// A variable is used more or less often than the calculus allows.
    Uses {
        /// The variable.
        variable: VarId,
        /// Whether it is not used at all.
        unused: bool,
    },
    /// Two components of a pair, or two branches of a case, use a
    /// variable differently in the linear calculus.
    Shared {
        /// The variable.
        variable: VarId,
    },
    /// The body of a `promote` uses a variable that is none of its own.
    Outside {
        /// The variable.
        variable: VarId,
    },
    /// A variable is free and the context does not declare it.
    Free {
        /// The variable.
        variable: VarId,
    },
    /// A variable is used under a binder of another variable of its
    /// name, so the printed term would name the other.
    Captured {
        /// The variable used.
        variable: VarId,
    },
    /// The term has another type than the one claimed.
    Goal {
        /// The term's type.
        found: TypeId,
        /// The type claimed.
        goal: TypeId,
    },
    /// A type variable is named outside the type abstraction that binds
    /// it: in a type written there, or in the type of a variable bound
    /// outside it.
    TypeFree {
        /// The type variable.
        variable: TypeVarId,
    },
    /// A type variable is named under a type abstraction of another type
    /// variable of its name, so the printed term would name the other.
    TypeCaptured {
        /// The type variable named.
        variable: TypeVarId,
    },
    /// A type abstraction's variable has the name of an atom, so the
    /// printed term would read the atom as the variable.
    TypeNamed {
        /// The type variable.
        variable: TypeVarId,
    },
    /// Two type abstractions bind one type variable.
    TypeTwice {
        /// The type variable.
        variable: TypeVarId,
    },
}

/// The most characters of a type that an error's message writes.
const SHOWN: usize = 200;

/// A text that takes [`SHOWN`] characters and refuses the rest.
struct Capped {
    /// The text.
    text: String,
    /// How many more characters it takes.
    left: usize,
}

impl Write for Capped {
    /// Appends `s`, or fails at the first character past the cap.
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            self.left = self.left.checked_sub(1).ok_or(std::fmt::Error)?;
            self.text.push(c);
        }
        Ok(())
    }
}

/// What a fault's message reads: the types, the atoms' and the
/// variables' names, and the calculus that spells the connectives.
#[derive(Clone, Copy)]
pub(crate) struct Names<'a> {
    /// The types.
    pub(crate) types: &'a [Type],
    /// The atoms' names.
    pub(crate) atoms: &'a [Box<str>],
    /// The variables.
    pub(crate) variables: &'a [Variable],
    /// The type variables' names.
    pub(crate) type_variables: &'a [Box<str>],
    /// The calculus.
    pub(crate) calculus: Calculus,
}

impl Names<'_> {
    /// Returns a type in Unicode, cut with `…` past [`SHOWN`] characters.
    fn ty(&self, ty: TypeId) -> String {
        let mut out = Capped {
            text: String::new(),
            left: SHOWN,
        };
        let connectives = UNICODE.connectives(self.calculus);
        let written = write_type(
            &mut out,
            self.types,
            self.atoms,
            self.type_variables,
            &UNICODE,
            connectives,
            ty,
            false,
        );
        if written.is_err() {
            out.text.push('…');
        }
        out.text
    }

    /// Returns a variable's name in Unicode.
    fn variable(&self, variable: VarId) -> String {
        let mut out = String::new();
        // Writing into a `String` does not fail.
        let _ = write_name(
            &mut out,
            self.variables[variable.index()].name(),
            super::print::Index::Subscript,
            crate::linear::sequents::notation::Atoms::Plain,
        );
        out
    }

    /// Returns a type variable's name in Unicode.
    fn type_variable(&self, variable: TypeVarId) -> String {
        let mut out = String::new();
        // Writing into a `String` does not fail.
        let _ = write_name(
            &mut out,
            &self.type_variables[variable.index()],
            super::print::Index::Subscript,
            crate::linear::sequents::notation::Atoms::Plain,
        );
        out
    }

    /// Returns the calculus's name in a sentence.
    const fn calculus(&self) -> &'static str {
        match self.calculus {
            Calculus::Linear => "the linear lambda calculus",
            Calculus::Affine => "the affine lambda calculus",
            Calculus::Simple => "the simply typed lambda calculus",
            Calculus::Untyped => "the untyped lambda calculus",
            Calculus::UntypedLinear => "the untyped linear lambda calculus",
            Calculus::UntypedAffine => "the untyped affine lambda calculus",
            Calculus::SystemF => "System F",
            Calculus::SystemFLinear => "linear System F",
            Calculus::SystemFAffine => "affine System F",
        }
    }
}

/// Says a fault in words, its types and variables written out.
pub(crate) fn describe(fault: Fault, names: Names<'_>) -> String {
    match fault {
        Fault::Foreign { what } => format!("{what} is no part of {}", names.calculus()),
        Fault::Shape { of, needs, found } => {
            format!("{of} has type {}, which is not {needs}", names.ty(found))
        }
        Fault::Argument { expected, found } => format!(
            "the function takes an argument of type {}, and its argument has type {}",
            names.ty(expected),
            names.ty(found)
        ),
        Fault::Injection {
            sum,
            expected,
            found,
        } => format!(
            "an injection into {} takes a term of type {} here, and its term has type {}",
            names.ty(sum),
            names.ty(expected),
            names.ty(found)
        ),
        Fault::Pattern { variable, expected } => format!(
            "the variable {} has type {}, and the part it names has type {}",
            names.variable(variable),
            names.ty(names.variables[variable.index()].typed()),
            names.ty(expected)
        ),
        Fault::Twice { variable } => format!(
            "a pattern names the variable {} for two of its parts",
            names.variable(variable)
        ),
        Fault::Branches { left, right } => format!(
            "the branches of a `case` have the types {} and {}, which differ",
            names.ty(left),
            names.ty(right)
        ),
        Fault::Promoted { terms, variables } => {
            format!("a `promote` has {terms} terms for {variables} variables")
        }
        Fault::Uses { variable, unused } => format!(
            "the variable {} is {}, and {} uses every variable {}",
            names.variable(variable),
            if unused {
                "not used"
            } else {
                "used more than once"
            },
            names.calculus(),
            if names.calculus.is_linear() {
                "exactly once"
            } else {
                "at most once"
            }
        ),
        Fault::Shared { variable } => format!(
            "the variable {} is used in one component of a pair, or one branch of a `case`, and not in the other; in {} both use the same variables",
            names.variable(variable),
            names.calculus()
        ),
        Fault::Outside { variable } => format!(
            "the body of a `promote` uses the variable {}, which is none of its own",
            names.variable(variable)
        ),
        Fault::Free { variable } => format!(
            "the variable {} is free, and the context does not declare it",
            names.variable(variable)
        ),
        Fault::Captured { variable } => {
            let name = names.variable(variable);
            format!(
                "the variable {name} is used under a binder of another variable named {name}, so the term's text would name the other; rename one"
            )
        }
        Fault::Goal { found, goal } => format!(
            "the term has type {}, not {}",
            names.ty(found),
            names.ty(goal)
        ),
        Fault::TypeFree { variable } => {
            let name = names.type_variable(variable);
            format!(
                "the type variable {name} is named outside the type abstraction Λ{name} that binds it: in a type written there, or in the type of a term variable bound outside it, which Λ{name} cannot generalize"
            )
        }
        Fault::TypeCaptured { variable } => {
            let name = names.type_variable(variable);
            format!(
                "the type variable {name} is named under a type abstraction of another type variable named {name}, so the term's text would name the other; rename one"
            )
        }
        Fault::TypeTwice { variable } => format!(
            "two type abstractions bind the type variable {}; give each its own",
            names.type_variable(variable)
        ),
        Fault::TypeNamed { variable } => {
            let name = names.type_variable(variable);
            format!(
                "the type variable {name} of a type abstraction has the name of an atom, so the term's text would read the atom as the variable; rename one"
            )
        }
    }
}

/// Returns the type a node has by its rule, from its variables' types
/// (`variable`) and its operands' (`of`), or why it has none. A type the
/// rule makes is added to `types`.
///
/// The parser calls it for each node it closes, and the checker for
/// every node of a term, so the rules are written once.
#[expect(clippy::too_many_lines, reason = "one arm per node of the calculus")]
pub(crate) fn rule(
    node: &Node,
    calculus: Calculus,
    types: &mut Types,
    variable: impl Fn(VarId) -> TypeId,
    of: impl Fn(TermId) -> TypeId,
) -> Result<TypeId, Fault> {
    let simple = calculus.is_ordinary();
    let foreign = |what| {
        if simple {
            Err(Fault::Foreign { what })
        } else {
            Ok(())
        }
    };
    let pattern = |v: VarId, expected: TypeId| {
        if variable(v) == expected {
            Ok(())
        } else {
            Err(Fault::Pattern {
                variable: v,
                expected,
            })
        }
    };
    let apart = |left: VarId, right: VarId| {
        if left == right {
            Err(Fault::Twice { variable: left })
        } else {
            Ok(())
        }
    };
    let polymorphic = |what| {
        if calculus.is_polymorphic() {
            Ok(())
        } else {
            Err(Fault::Foreign { what })
        }
    };
    match node {
        Node::TypeLambda { variable: x, body } => {
            polymorphic("a type abstraction `ΛX. M`")?;
            Ok(poly::close(types, of(*body), *x))
        }
        Node::TypeApply { term, argument } => {
            polymorphic("a type application `M [A]`")?;
            let found = of(*term);
            let Type::Forall { body, .. } = types.get(found) else {
                return Err(Fault::Shape {
                    of: "the term a type is applied to",
                    needs: "a type ∀X. A",
                    found,
                });
            };
            Ok(poly::open(types, body, *argument))
        }
        Node::Var { variable: v } => Ok(variable(*v)),
        Node::Lambda { variable: v, body } => Ok(types.add(Type::Lollipop {
            from: variable(*v),
            to: of(*body),
        })),
        Node::Apply { function, argument } => {
            let found = of(*function);
            let Type::Lollipop { from, to } = types.get(found) else {
                return Err(Fault::Shape {
                    of: "the function of an application",
                    needs: "a function type",
                    found,
                });
            };
            let argument = of(*argument);
            if argument != from {
                return Err(Fault::Argument {
                    expected: from,
                    found: argument,
                });
            }
            Ok(to)
        }
        Node::Tensor { left, right } => {
            foreign("a pair `M ⊗ N`")?;
            Ok(types.add(Type::Tensor {
                left: of(*left),
                right: of(*right),
            }))
        }
        Node::LetTensor {
            term,
            left,
            right,
            body,
        } => {
            foreign("a `let` that takes a pair apart")?;
            let found = of(*term);
            let Type::Tensor { left: a, right: b } = types.get(found) else {
                return Err(Fault::Shape {
                    of: "the term a `let` takes apart",
                    needs: "a pair type A ⊗ B",
                    found,
                });
            };
            apart(*left, *right)?;
            pattern(*left, a)?;
            pattern(*right, b)?;
            Ok(of(*body))
        }
        Node::Unit => {
            foreign("the unit `∗`")?;
            Ok(types.add(Type::One))
        }
        Node::LetUnit { term, body } => {
            foreign("a `let` that takes the unit")?;
            let found = of(*term);
            if types.get(found) != Type::One {
                return Err(Fault::Shape {
                    of: "the term a `let … be ∗` takes",
                    needs: "the type 1",
                    found,
                });
            }
            Ok(of(*body))
        }
        Node::Pair { left, right } => Ok(types.add(Type::With {
            left: of(*left),
            right: of(*right),
        })),
        Node::First { term } | Node::Second { term } => {
            let found = of(*term);
            let Type::With { left, right } = types.get(found) else {
                return Err(Fault::Shape {
                    of: "the argument of a projection",
                    needs: "a product type",
                    found,
                });
            };
            Ok(if matches!(node, Node::First { .. }) {
                left
            } else {
                right
            })
        }
        Node::Left { term, sum } | Node::Right { term, sum } => {
            let Type::Plus { left, right } = types.get(*sum) else {
                return Err(Fault::Shape {
                    of: "the type an injection is given",
                    needs: "a sum type",
                    found: *sum,
                });
            };
            let expected = if matches!(node, Node::Left { .. }) {
                left
            } else {
                right
            };
            let found = of(*term);
            if found != expected {
                return Err(Fault::Injection {
                    sum: *sum,
                    expected,
                    found,
                });
            }
            Ok(*sum)
        }
        Node::Case {
            term,
            left,
            left_body,
            right,
            right_body,
        } => {
            let found = of(*term);
            let Type::Plus { left: a, right: b } = types.get(found) else {
                return Err(Fault::Shape {
                    of: "the term a `case` takes apart",
                    needs: "a sum type",
                    found,
                });
            };
            pattern(*left, a)?;
            pattern(*right, b)?;
            let (first, second) = (of(*left_body), of(*right_body));
            if first != second {
                return Err(Fault::Branches {
                    left: first,
                    right: second,
                });
            }
            Ok(first)
        }
        Node::True { context } => {
            if !context.is_empty() {
                foreign("a `true` that uses up terms")?;
            }
            Ok(types.add(Type::Top))
        }
        Node::False {
            context,
            term,
            goal,
        } => {
            if !context.is_empty() {
                foreign("a `false` that uses up terms")?;
            }
            let found = of(*term);
            if types.get(found) != Type::Zero {
                return Err(Fault::Shape {
                    of: if simple {
                        "the argument of `abort`"
                    } else {
                        "the last argument of `false`"
                    },
                    needs: "the empty type",
                    found,
                });
            }
            Ok(*goal)
        }
        Node::Promote {
            terms,
            variables,
            body,
        } => {
            foreign("a `promote`")?;
            if terms.len() != variables.len() {
                return Err(Fault::Promoted {
                    terms: terms.len(),
                    variables: variables.len(),
                });
            }
            let mut sorted = variables.to_vec();
            sorted.sort_unstable();
            if let Some(twice) = sorted.windows(2).find(|pair| pair[0] == pair[1]) {
                return Err(Fault::Twice { variable: twice[0] });
            }
            for (&t, &v) in terms.iter().zip(variables.iter()) {
                let found = banged(types, of(t), "a term a `promote` takes")?;
                pattern(v, found)?;
            }
            Ok(types.add(Type::Bang { of: of(*body) }))
        }
        Node::Derelict { term } => {
            foreign("a `derelict`")?;
            let found = banged(types, of(*term), "the argument of `derelict`")?;
            let Type::Bang { of: inner } = types.get(found) else {
                unreachable!("`banged` returns a type `!A`")
            };
            Ok(inner)
        }
        Node::Discard { term, body } => {
            foreign("a `discard`")?;
            banged(types, of(*term), "the term a `discard` takes")?;
            Ok(of(*body))
        }
        Node::Copy {
            term,
            left,
            right,
            body,
        } => {
            foreign("a `copy`")?;
            let found = banged(types, of(*term), "the term a `copy` takes")?;
            apart(*left, *right)?;
            pattern(*left, found)?;
            pattern(*right, found)?;
            Ok(of(*body))
        }
    }
}

/// Returns `found` if it is a type `!A`, or the fault of the place `of`.
fn banged(types: &Types, found: TypeId, of: &'static str) -> Result<TypeId, Fault> {
    if matches!(types.get(found), Type::Bang { .. }) {
        Ok(found)
    } else {
        Err(Fault::Shape {
            of,
            needs: "a type !A",
            found,
        })
    }
}

/// Returns the first type of an arena that a typed calculus does not
/// have, in words: the simple calculus and System F have no `⊗`, `1` or
/// `!`, and only System F has `∀`.
fn foreign_type(types: &[Type], calculus: Calculus) -> Option<&'static str> {
    let ordinary = calculus.is_ordinary();
    let polymorphic = calculus.is_polymorphic();
    types.iter().find_map(|ty| match ty {
        Type::Tensor { .. } if ordinary => Some("a pair type `A ⊗ B`"),
        Type::One if ordinary => Some("the type `1`"),
        Type::Bang { .. } if ordinary => Some("a type `!A`"),
        Type::Forall { .. } | Type::Bound { .. } | Type::Variable { .. } if !polymorphic => {
            Some("a type `∀X. A`")
        }
        Type::Atom { .. }
        | Type::One
        | Type::Top
        | Type::Zero
        | Type::Tensor { .. }
        | Type::With { .. }
        | Type::Plus { .. }
        | Type::Lollipop { .. }
        | Type::Bang { .. }
        | Type::Forall { .. }
        | Type::Bound { .. }
        | Type::Variable { .. } => None,
    })
}

/// Returns what a node of an untyped term is, where the untyped calculi
/// have no such node: they have variables, `λ` and application alone.
fn untyped_foreign(node: &Node) -> Option<&'static str> {
    match node {
        Node::Var { .. } | Node::Lambda { .. } | Node::Apply { .. } => None,
        Node::Tensor { .. } | Node::LetTensor { .. } => Some("a pair `M ⊗ N`"),
        Node::Unit | Node::LetUnit { .. } => Some("the unit `∗`"),
        Node::Pair { .. } | Node::First { .. } | Node::Second { .. } => Some("a pair `⟨M, N⟩`"),
        Node::Left { .. } | Node::Right { .. } | Node::Case { .. } => {
            Some("an injection `inl` or `inr`")
        }
        Node::True { .. } => Some("a `true`"),
        Node::False { .. } => Some("a `false`"),
        Node::Promote { .. } | Node::Derelict { .. } | Node::Discard { .. } | Node::Copy { .. } => {
            Some("a node of `!`")
        }
        Node::TypeLambda { .. } => Some("a type abstraction `ΛX. M`"),
        Node::TypeApply { .. } => Some("a type application `M [A]`"),
    }
}

/// How often each free variable of a subterm is used.
///
/// A count saturates at `u64::MAX`, which never accepts a term: the
/// simple calculus reads no count, and the other two compare a count
/// with 1 where its variable is bound, or refuse a variable that stays
/// free.
type Uses = HashMap<VarId, u64>;

/// What the check counts per node before it starts, in bytes: its type,
/// its count of users, and the place of its variables' uses.
const NODE_BYTES: u64 = 64;

/// What the check counts per type, in bytes: the type twice, in the
/// list and in the table of ids, a hash table at its least load.
const TYPE_BYTES: u64 = 48;

/// What the check counts per variable, in bytes: its name's entry in
/// the table that finds equal names, at a hash table's least load, and
/// its flag.
const VARIABLE_BYTES: u64 = 64;

/// What the check counts per use it keeps, in bytes: a variable with its
/// count, in a hash table at its least load and while it grows.
const USE_BYTES: u64 = 64;

/// How many units of work pass between two asks of the stop.
const POLL: u64 = 1024;

/// The check's count of work and of the uses it keeps, against the
/// limits.
struct Meter<'a> {
    /// The work done.
    work: u64,
    /// The most work, [`Limits::work`].
    most: Option<u64>,
    /// The work at which the stop is asked next.
    next: u64,
    /// The caller's stop.
    stop: &'a mut dyn FnMut() -> bool,
    /// The bytes held: the tables made at the start and the uses kept.
    held: u64,
    /// The most bytes, [`Limits::memory_bytes`].
    memory: Option<u64>,
}

impl Meter<'_> {
    /// Counts `units` of work, and refuses past the limit or when the
    /// stop fires.
    fn tick(&mut self, units: usize) -> Result<(), Error> {
        self.work = self.work.saturating_add(units as u64);
        if let Some(limit) = self.most.filter(|&most| self.work > most) {
            return Err(Error::Refused(Refusal::Work { limit }));
        }
        if self.work >= self.next {
            self.next = self.work.saturating_add(POLL);
            if (self.stop)() {
                return Err(Error::Refused(Refusal::stopped(Phase::Check)));
            }
        }
        Ok(())
    }

    /// Counts `uses` more uses kept, and refuses past the memory bound.
    fn hold(&mut self, uses: usize) -> Result<(), Error> {
        self.hold_bytes((uses as u64).saturating_mul(USE_BYTES))
    }

    /// Counts `bytes` more held, and refuses past the memory bound.
    fn hold_bytes(&mut self, bytes: u64) -> Result<(), Error> {
        self.held = self.held.saturating_add(bytes);
        match self.memory {
            Some(limit) if self.held > limit => Err(Error::Refused(Refusal::Memory {
                phase: Phase::Check,
                limit_bytes: limit,
                needed_bytes: None,
            })),
            _ => Ok(()),
        }
    }

    /// Counts `uses` fewer uses kept.
    const fn release(&mut self, uses: usize) {
        self.held = self
            .held
            .saturating_sub((uses as u64).saturating_mul(USE_BYTES));
    }
}

/// The pass that counts every variable's uses.
struct Pass<'t, 'm> {
    /// The term.
    term: &'t Term,
    /// Per variable, whether another variable has its name.
    twin: Vec<bool>,
    /// Per node, how many places still name it and have not taken its
    /// uses: its users, and one more for the root. A count is at most
    /// the operands of all nodes and one, which a `u64` holds for any
    /// term in memory.
    waiting: Vec<u64>,
    /// Per node, the uses of its free variables, until its last user
    /// takes them.
    uses: Vec<Option<Uses>>,
    /// The counts against the limits.
    meter: Meter<'m>,
}

impl Pass<'_, '_> {
    /// Returns the uses of a subterm for one place that names it: moved
    /// out for the last place, copied for the others.
    fn take(&mut self, id: TermId) -> Result<Uses, Error> {
        let i = id.index();
        self.waiting[i] -= 1;
        if self.waiting[i] == 0 {
            let uses = self.uses[i].take().expect("a node's operands come first");
            self.meter.release(uses.len());
            self.meter.tick(1)?;
            Ok(uses)
        } else {
            let uses = self.uses[i].clone().expect("a node's operands come first");
            self.meter.tick(uses.len() + 1)?;
            Ok(uses)
        }
    }

    /// Returns the uses of two subterms together.
    fn sum(&mut self, a: Uses, b: Uses) -> Result<Uses, Error> {
        let (mut large, small) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        self.meter.tick(small.len() + 1)?;
        for (v, n) in small {
            let count = large.entry(v).or_default();
            *count = count.saturating_add(n);
        }
        Ok(large)
    }

    /// Returns the uses of two subterms of which one is taken: equal in
    /// the linear calculus, the larger of each otherwise.
    fn share(&mut self, a: Uses, b: Uses) -> Result<Result<Uses, Fault>, Error> {
        self.meter.tick(a.len() + b.len() + 1)?;
        if self.term.calculus.is_linear() {
            let differs = |x: &Uses, y: &Uses| {
                x.iter()
                    .filter(|&(v, n)| y.get(v) != Some(n))
                    .map(|(&v, _)| v)
                    .min()
            };
            let variable = match (differs(&a, &b), differs(&b, &a)) {
                (Some(x), Some(y)) => Some(x.min(y)),
                (x, y) => x.or(y),
            };
            return Ok(match variable {
                Some(variable) => Err(Fault::Shared { variable }),
                None => Ok(a),
            });
        }
        let (mut large, small) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        for (v, n) in small {
            let count = large.entry(v).or_default();
            *count = (*count).max(n);
        }
        Ok(Ok(large))
    }

    /// Takes a variable out of the uses of the subterm that binds it,
    /// checking that it is used as the calculus allows and that the
    /// subterm leaves no other variable of its name free.
    fn bind(&mut self, uses: &mut Uses, variable: VarId) -> Result<Result<(), Fault>, Error> {
        let count = uses.remove(&variable).unwrap_or(0);
        let calculus = self.term.calculus;
        let fits = if calculus.is_linear() {
            count == 1
        } else {
            calculus.is_unrestricted() || count <= 1
        };
        if !fits {
            return Ok(Err(Fault::Uses {
                variable,
                unused: count == 0,
            }));
        }
        if self.twin[variable.index()] {
            self.meter.tick(uses.len() + 1)?;
            let name = self.term.variables[variable.index()].name();
            let captured = uses
                .keys()
                .copied()
                .filter(|other| self.term.variables[other.index()].name() == name)
                .min();
            if let Some(variable) = captured {
                return Ok(Err(Fault::Captured { variable }));
            }
        }
        Ok(Ok(()))
    }

    /// Returns the uses of a node's free variables, from its operands'.
    #[expect(clippy::too_many_lines, reason = "one arm per node of the calculus")]
    fn node(&mut self, node: &Node) -> Result<Result<Uses, Fault>, Error> {
        /// Leaves the function with a fault.
        macro_rules! sound {
            ($checked:expr) => {
                match $checked? {
                    Ok(value) => value,
                    Err(fault) => return Ok(Err(fault)),
                }
            };
        }
        Ok(Ok(match node {
            Node::Var { variable } => Uses::from_iter([(*variable, 1)]),
            Node::Unit => Uses::default(),
            Node::Lambda { variable, body } => {
                let mut uses = self.take(*body)?;
                sound!(self.bind(&mut uses, *variable));
                uses
            }
            Node::Apply {
                function: left,
                argument: right,
            }
            | Node::Tensor { left, right }
            | Node::LetUnit {
                term: left,
                body: right,
            }
            | Node::Discard {
                term: left,
                body: right,
            } => {
                let (a, b) = (self.take(*left)?, self.take(*right)?);
                self.sum(a, b)?
            }
            Node::LetTensor {
                term,
                left,
                right,
                body,
            }
            | Node::Copy {
                term,
                left,
                right,
                body,
            } => {
                let taken = self.take(*term)?;
                let mut uses = self.take(*body)?;
                // The later variable of a pattern is the inner one, as
                // the text reads it.
                sound!(self.bind(&mut uses, *right));
                sound!(self.bind(&mut uses, *left));
                self.sum(taken, uses)?
            }
            Node::Pair { left, right } => {
                let (a, b) = (self.take(*left)?, self.take(*right)?);
                sound!(self.share(a, b))
            }
            Node::First { term }
            | Node::Second { term }
            | Node::Left { term, .. }
            | Node::Right { term, .. }
            | Node::Derelict { term }
            | Node::TypeApply { term, .. } => self.take(*term)?,
            Node::TypeLambda { body, .. } => self.take(*body)?,
            Node::Case {
                term,
                left,
                left_body,
                right,
                right_body,
            } => {
                let taken = self.take(*term)?;
                let mut first = self.take(*left_body)?;
                sound!(self.bind(&mut first, *left));
                let mut second = self.take(*right_body)?;
                sound!(self.bind(&mut second, *right));
                let branches = sound!(self.share(first, second));
                self.sum(taken, branches)?
            }
            Node::True { context } => {
                let mut uses = Uses::default();
                for &t in context.iter() {
                    let taken = self.take(t)?;
                    uses = self.sum(uses, taken)?;
                }
                uses
            }
            Node::False { context, term, .. } => {
                let mut uses = self.take(*term)?;
                for &t in context.iter() {
                    let taken = self.take(t)?;
                    uses = self.sum(uses, taken)?;
                }
                uses
            }
            Node::Promote {
                terms,
                variables,
                body,
            } => {
                let mut uses = Uses::default();
                for &t in terms.iter() {
                    let taken = self.take(t)?;
                    uses = self.sum(uses, taken)?;
                }
                let mut inner = self.take(*body)?;
                // The later variable is the inner one.
                for &v in variables.iter().rev() {
                    sound!(self.bind(&mut inner, v));
                }
                if let Some(&variable) = inner.keys().min() {
                    return Ok(Err(Fault::Outside { variable }));
                }
                uses
            }
        }))
    }
}

/// Returns the error for a fault at a node of a term.
fn ill_typed(term: &Term, types: &Types, node: usize, fault: Fault) -> Error {
    let names = Names {
        types: &types.list,
        atoms: &term.atoms,
        variables: &term.variables,
        type_variables: &term.type_variables,
        calculus: term.calculus,
    };
    Error::IllTyped {
        reason: describe(fault, names),
        // A term's nodes are fewer than 2³²: their ids are `u32`.
        node: Some(node as u32),
        span: None,
    }
}

/// Checks a term, as [`Term::check`] does, the stop a plain condition.
fn check(term: &Term, limits: &Limits, stop: &mut dyn FnMut() -> bool) -> Result<Typing, Error> {
    let nodes = term.nodes.len();
    let needed = (nodes as u64)
        .saturating_mul(NODE_BYTES)
        .saturating_add((term.types.len() as u64).saturating_mul(TYPE_BYTES))
        // A type per node at most is made.
        .saturating_add((nodes as u64).saturating_mul(TYPE_BYTES))
        .saturating_add((term.variables.len() as u64).saturating_mul(VARIABLE_BYTES));
    if let Some(limit) = limits.memory_bytes.filter(|&limit| needed > limit) {
        return Err(Error::Refused(Refusal::Memory {
            phase: Phase::Check,
            limit_bytes: limit,
            needed_bytes: Some(needed),
        }));
    }
    let mut types = Types::of(&term.types, nodes);
    let root = term.root.index();
    if term.calculus.is_typed()
        && let Some(what) = foreign_type(&term.types, term.calculus)
    {
        return Err(ill_typed(term, &types, root, Fault::Foreign { what }));
    }
    // How many variables have each name. The table is made at its full
    // size, so that it holds what the estimate counted.
    let mut names: HashMap<&str, u32> =
        HashMap::with_capacity_and_hasher(term.variables.len(), Default::default());
    for variable in &term.variables {
        let count = names.entry(variable.name()).or_default();
        *count = count.saturating_add(1);
    }
    let mut waiting = vec![0u64; nodes];
    for node in &term.nodes {
        super::operands(node, |t| waiting[t.index()] += 1);
    }
    waiting[root] += 1;
    let mut pass = Pass {
        term,
        twin: term
            .variables
            .iter()
            .map(|variable| names[variable.name()] > 1)
            .collect(),
        waiting,
        uses: Vec::with_capacity(nodes),
        meter: Meter {
            work: 0,
            most: limits.work,
            next: POLL,
            stop,
            held: needed,
            memory: limits.memory_bytes,
        },
    };
    let typed = term.calculus.is_typed();
    let mut of: Vec<TypeId> = Vec::with_capacity(if typed { nodes } else { 0 });
    for (i, node) in term.nodes.iter().enumerate() {
        pass.meter.tick(1)?;
        if typed {
            let (work, made) = (types.work, types.len());
            let ty = rule(
                node,
                term.calculus,
                &mut types,
                |v| term.variables[v.index()].typed(),
                |t| of[t.index()],
            )
            .map_err(|fault| ill_typed(term, &types, i, fault))?;
            // The estimate counted one type made per node; System F's
            // rules can make more, and count their work.
            pass.meter.tick((types.work - work) as usize)?;
            let more = (types.len() - made).saturating_sub(1) as u64;
            pass.meter.hold_bytes(more.saturating_mul(TYPE_BYTES))?;
            of.push(ty);
        } else if let Some(what) = untyped_foreign(node) {
            return Err(ill_typed(term, &types, i, Fault::Foreign { what }));
        }
        let uses = pass
            .node(node)?
            .map_err(|fault| ill_typed(term, &types, i, fault))?;
        if pass.waiting[i] == 0 {
            // No node names it and it is not the root: nothing reads its
            // uses.
            pass.uses.push(None);
        } else {
            pass.meter.hold(uses.len())?;
            pass.uses.push(Some(uses));
        }
    }
    let mut free = pass.take(term.root)?;
    // A later declaration is the inner one.
    for &variable in term.context.iter().rev() {
        pass.bind(&mut free, variable)?
            .map_err(|fault| ill_typed(term, &types, root, fault))?;
    }
    if let Some(&variable) = free.keys().min() {
        return Err(ill_typed(term, &types, root, Fault::Free { variable }));
    }
    if let Some(goal) = term.goal {
        let found = of[root];
        if found != goal {
            return Err(ill_typed(term, &types, root, Fault::Goal { found, goal }));
        }
    }
    if term.calculus.is_polymorphic() {
        scope(term, &types, &mut pass.meter)?;
    }
    Ok(Typing {
        types: types.list,
        of,
    })
}

/// Returns the types a node writes: its variables' (of a `λ` or a
/// pattern), an injection's sum, `false`'s type and a type application's
/// argument.
pub(crate) fn written(term: &Term, node: &Node, mut f: impl FnMut(TypeId)) {
    let mut variable = |v: &VarId| f(term.variables[v.index()].typed());
    match node {
        Node::Lambda { variable: v, .. } => variable(v),
        Node::LetTensor { left, right, .. }
        | Node::Copy { left, right, .. }
        | Node::Case { left, right, .. } => {
            variable(left);
            variable(right);
        }
        Node::Promote { variables, .. } => variables.iter().for_each(variable),
        Node::Left { sum, .. } | Node::Right { sum, .. } => f(*sum),
        Node::False { goal, .. } => f(*goal),
        Node::TypeApply { argument, .. } => f(*argument),
        Node::Var { .. }
        | Node::Apply { .. }
        | Node::Tensor { .. }
        | Node::Unit
        | Node::LetUnit { .. }
        | Node::Pair { .. }
        | Node::First { .. }
        | Node::Second { .. }
        | Node::True { .. }
        | Node::Derelict { .. }
        | Node::Discard { .. }
        | Node::TypeLambda { .. } => {}
    }
}

/// Checks that every type variable of a term of System F is bound by one
/// type abstraction `ΛX. M` at most and named only inside it, under no
/// other type variable's abstraction of its name, and with no atom's
/// name.
///
/// A node's free type variables are those of the types it writes and of
/// its subterms, less its own at a `ΛX`; the root's, with those of the
/// context's types and the goal, must be none. A term variable's type is
/// written where it is bound, so a variable bound outside `ΛX. M` whose
/// type names `X` leaves `X` free: this is `∀`'s side condition, that `X`
/// is free in no hypothesis. It needs `X` bound by that one `ΛX`: under a
/// second `ΛX` inside the first, a use of the outer variable `x : X`
/// would be generalized by the inner one and `X` taken out by the outer,
/// and `ΛX. λx:X. ΛX. x` would have the type `∀X. X → ∀X. X`. Sets are
/// per node, whatever the place, so a shared node is scoped once.
fn scope(term: &Term, types: &Types, meter: &mut Meter<'_>) -> Result<(), Error> {
    let root = term.root.index();
    let fault = |node: usize, fault| Err(ill_typed(term, types, node, fault));
    let mut bound = vec![false; term.type_variables.len()];
    for (i, node) in term.nodes.iter().enumerate() {
        if let Node::TypeLambda { variable, .. } = node
            && std::mem::replace(&mut bound[variable.index()], true)
        {
            return fault(
                i,
                Fault::TypeTwice {
                    variable: *variable,
                },
            );
        }
    }
    let mut atoms: HashSet<&str> =
        HashSet::with_capacity_and_hasher(term.atoms.len(), Default::default());
    atoms.extend(term.atoms.iter().map(|a| &**a));
    // The free type variables of each type a term writes, found when
    // first asked, within the limits.
    let mut walker = poly::Walker::new(term.types.len());
    let mut known: HashMap<TypeId, Box<[TypeVarId]>> = HashMap::default();
    let mut free = |t: TypeId, meter: &mut Meter<'_>| -> Result<Box<[TypeVarId]>, Error> {
        if let Some(found) = known.get(&t) {
            return Ok(found.clone());
        }
        let (found, visited) = walker.variables(&term.types, t);
        meter.tick(visited as usize)?;
        meter.hold_bytes((found.len() as u64).saturating_mul(16))?;
        let found: Box<[TypeVarId]> = found.into();
        known.insert(t, found.clone());
        Ok(found)
    };
    let mut of: Vec<Box<[TypeVarId]>> = Vec::with_capacity(term.nodes.len());
    for (i, node) in term.nodes.iter().enumerate() {
        let mut writes: Vec<TypeId> = Vec::new();
        written(term, node, |t| writes.push(t));
        let mut own: Vec<TypeVarId> = Vec::new();
        for t in writes {
            own.extend_from_slice(&free(t, meter)?);
        }
        super::operands(node, |t| own.extend_from_slice(&of[t.index()]));
        own.sort_unstable();
        own.dedup();
        if let Node::TypeLambda { variable, .. } = node {
            let name = &term.type_variables[variable.index()];
            if atoms.contains(&**name) {
                return fault(
                    i,
                    Fault::TypeNamed {
                        variable: *variable,
                    },
                );
            }
            own.retain(|v| v != variable);
            if let Some(&other) = own.iter().find(|v| term.type_variables[v.index()] == *name) {
                return fault(i, Fault::TypeCaptured { variable: other });
            }
        }
        meter.tick(own.len() + 1)?;
        meter.hold_bytes((own.len() as u64).saturating_mul(8))?;
        of.push(own.into());
    }
    let mut outside: Vec<TypeVarId> = of[root].to_vec();
    let context = term
        .context
        .iter()
        .map(|v| term.variables[v.index()].typed());
    for t in context.chain(term.goal) {
        outside.extend_from_slice(&free(t, meter)?);
    }
    match outside.into_iter().min() {
        Some(variable) => fault(root, Fault::TypeFree { variable }),
        None => Ok(()),
    }
}

impl Term {
    /// Checks that the term is well typed in its calculus, and returns
    /// the type of every node.
    ///
    /// A term of an untyped calculus has no types: the check is then that
    /// it has variables, `λ` and application alone and uses every
    /// variable as its calculus allows, and the typing it returns is
    /// empty.
    ///
    /// The term is well typed when:
    /// - every node has a type by its rule, which for a function and its
    ///   argument, a pattern and what it takes apart, an injection and
    ///   its sum, and the two branches of a `case` asks that the types
    ///   fit;
    /// - every variable is used as the [`Calculus`] allows, inside the
    ///   node that binds it, and the variables left free are those of
    ///   the [`context`](Self::context);
    /// - the whole term has the type [`goal`](Self::goal);
    /// - no variable is used under a binder of another variable with the
    ///   same name, so that the printed term reads back as this term;
    /// - in System F, a type variable is named only inside the type
    ///   abstraction `ΛX. M` that binds it, so that no variable bound
    ///   outside it has `X` in its type (the side condition of `∀`
    ///   introduction), and a `ΛX`'s variable has neither an atom's name
    ///   nor that of another type variable named inside it.
    ///
    /// The check is one pass over the nodes. It keeps the uses of every
    /// subterm's free variables until the subterms that name it have
    /// read them, within `limits.memory_bytes`, and counts a unit of
    /// work per node and per use it moves or compares, within
    /// `limits.work`.
    ///
    #[cfg_attr(feature = "parse", doc = "```")]
    #[cfg_attr(not(feature = "parse"), doc = "```ignore")]
    /// use deduxit::{Error, Limits};
    /// use deduxit::lambda::{Calculus, Term};
    ///
    /// let limits = Limits::default();
    /// let term = Term::parse(r"\f:a -> b. \x:a. f x", Calculus::Simple, &limits)?;
    /// let typing = term.check(&limits, |_| false)?;
    /// assert_eq!(typing.ty(term.root()), term.goal());
    ///
    /// // A linear term uses every variable exactly once.
    /// let twice = Term::parse(r"\x:a. x * x", Calculus::Linear, &limits)?;
    /// let error = twice.check(&limits, |_| false).unwrap_err();
    /// assert!(matches!(error, Error::IllTyped { .. }));
    ///
    /// // So does an untyped linear term.
    /// let twice = Term::parse(r"\x. x x", Calculus::UntypedLinear, &limits)?;
    /// let error = twice.check(&limits, |_| false).unwrap_err();
    /// assert!(matches!(error, Error::IllTyped { .. }));
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// - [`Error::IllTyped`] for a term that is not well typed, with the
    ///   reason in words and the node.
    /// - [`Refusal::Memory`] and [`Refusal::Work`] past the limits.
    /// - [`Refusal::Stopped`] when `stop` fired.
    pub fn check(
        &self,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Typing, Error> {
        check(
            self,
            limits,
            &mut crate::limits::counting(stop, Phase::Check),
        )
    }
}
