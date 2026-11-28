// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The wire form of a lambda-term: a document of its own at the level
//! of the term's form, read with its shape checked.

use super::ordinary::CountermodelForm;
use crate::lambda::read::{FORM, Parts};
use crate::lambda::{
    Calculus, Count, End, Inhabitants, Inhabitation, Node, Reduction, Strategy, Term, TermId, Type,
    TypeId, TypeVarId, VarId, Variable,
};
use crate::limits::Limits;
use crate::linear::search::Reason;
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A term as it is written: the level first, then the term's parts.
#[derive(Serialize)]
struct Written<'a> {
    /// The wire level, left out where the term is nested in a document.
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<u32>,
    /// The calculus.
    calculus: Calculus,
    /// The atoms' names, left out in an untyped term.
    #[serde(skip_serializing_if = "Option::is_none")]
    atoms: Option<&'a [Box<str>]>,
    /// The types, operands first, left out in an untyped term.
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<&'a [Type]>,
    /// The variables.
    variables: &'a [Variable],
    /// The type variables' names, left out outside System F.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    type_variables: &'a [Box<str>],
    /// The variables of the hypotheses, in the order written.
    context: &'a [VarId],
    /// The term's type, left out in an untyped term.
    #[serde(skip_serializing_if = "Option::is_none")]
    goal: Option<TypeId>,
    /// The nodes, operands first.
    nodes: &'a [Node],
    /// The whole term.
    root: TermId,
}

impl Serialize for Term {
    /// Serializes the term as a document at the level of the term's
    /// form.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let level = level(self.calculus());
        written(self, Some(level)).serialize(serializer)
    }
}

/// Returns the level of a term of a calculus.
const fn level(calculus: Calculus) -> u32 {
    if calculus.is_polymorphic() {
        wire::POLYMORPHIC
    } else if calculus.is_typed() {
        wire::LAMBDA
    } else {
        wire::UNTYPED
    }
}

/// Returns a term's written form, with its level or without, as a
/// document nests it.
fn written(term: &Term, version: Option<u32>) -> Written<'_> {
    let typed = term.calculus().is_typed();
    Written {
        version,
        calculus: term.calculus(),
        atoms: typed.then(|| term.atoms()),
        types: typed.then(|| term.types()),
        variables: term.variables(),
        type_variables: term.type_variables(),
        context: term.context(),
        goal: term.goal(),
        nodes: term.nodes(),
        root: term.root(),
    }
}

/// The written form of a reduction.
#[derive(Serialize)]
struct ReductionForm<'a> {
    /// The wire level.
    version: u32,
    /// The strategy.
    strategy: Strategy,
    /// The term reached, nested.
    term: Written<'a>,
    /// The steps, each its redex.
    steps: &'a [StepForm],
    /// `normal_form`, `steps`, `nodes` or `refused`.
    end: &'static str,
    /// The bound reached, for `steps` and `nodes`.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
    /// Why the reduction was refused, in an error's form.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'a crate::Error>,
}

/// A step as a reduction's document writes it.
#[derive(Serialize)]
struct StepForm {
    /// The redex, a node of the term the step was taken on.
    redex: TermId,
}

impl Serialize for Reduction {
    /// Serializes the reduction as a document at level 30, written only:
    /// the term reached, the steps and how it ended.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (end, limit, reason) = match &self.end {
            End::NormalForm => ("normal_form", None, None),
            End::Steps { limit } => ("steps", Some(*limit), None),
            End::Nodes { limit } => ("nodes", Some(*limit), None),
            End::Refused(refusal) => (
                "refused",
                None,
                Some(crate::Error::Refused(refusal.clone())),
            ),
        };
        let steps: Vec<StepForm> = self
            .steps
            .iter()
            .map(|step| StepForm { redex: step.redex })
            .collect();
        ReductionForm {
            version: level(self.term.calculus()).max(wire::UNTYPED),
            strategy: self.strategy,
            term: written(&self.term, None),
            steps: &steps,
            end,
            limit,
            reason: reason.as_ref(),
        }
        .serialize(serializer)
    }
}

/// A variable as a document holds it.
#[derive(Deserialize)]
struct VariableForm {
    /// The type, left out in an untyped term.
    #[serde(rename = "type", default)]
    ty: Option<TypeId>,
    /// The name.
    name: String,
}

/// A term as a document holds it.
#[derive(Deserialize)]
struct TermForm {
    /// The wire level; not kept.
    #[serde(default, deserialize_with = "wire::version")]
    #[expect(dead_code, reason = "read to refuse a level above this build's")]
    version: Option<u32>,
    /// The calculus.
    calculus: Calculus,
    /// The atoms' names, none in an untyped term.
    #[serde(default)]
    atoms: Vec<String>,
    /// The types, operands first, none in an untyped term.
    #[serde(default)]
    types: Vec<ReadType>,
    /// The variables.
    variables: Vec<VariableForm>,
    /// The type variables' names, none outside System F.
    #[serde(default)]
    type_variables: Vec<String>,
    /// The variables of the hypotheses, in the order written.
    context: Vec<VarId>,
    /// The term's type, none in an untyped term.
    #[serde(default)]
    goal: Option<TypeId>,
    /// The nodes, operands first.
    nodes: Vec<ReadNode>,
    /// The whole term.
    root: TermId,
}

/// Reads a kind's name as serde reads the tag of an enum tagged by
/// `kind`: an identifier, which must be one of `names` (another is refused
/// with their list), and returns its index.
fn kind_index<'de, D: Deserializer<'de>>(
    deserializer: D,
    names: &'static [&'static str],
) -> Result<usize, D::Error> {
    /// The visitor of a kind's name.
    struct Name(&'static [&'static str]);
    impl serde::de::Visitor<'_> for Name {
        type Value = usize;
        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("variant identifier")
        }
        fn visit_str<E: serde::de::Error>(self, name: &str) -> Result<usize, E> {
            self.0
                .iter()
                .position(|known| *known == name)
                .ok_or_else(|| E::unknown_variant(name, self.0))
        }
    }
    deserializer.deserialize_identifier(Name(names))
}

/// A field a kind needs that its object lacks, said as serde says it.
struct Missing(&'static str);

impl std::fmt::Display for Missing {
    /// Writes "missing field `NAME`".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing field `{}`", self.0)
    }
}

/// A type read through its flat form, so that a field its kind lacks is
/// refused where the object ends, with its place.
#[derive(Deserialize)]
#[serde(try_from = "TypeForm")]
struct ReadType(Type);

impl TryFrom<TypeForm> for ReadType {
    type Error = Missing;
    /// Returns the type, or the field its kind lacks.
    fn try_from(form: TypeForm) -> Result<Self, Missing> {
        form.into_type().map(Self).map_err(Missing)
    }
}

/// A node read through its flat form, as [`ReadType`] reads a type.
#[derive(Deserialize)]
#[serde(try_from = "NodeForm")]
struct ReadNode(Node);

impl TryFrom<NodeForm> for ReadNode {
    type Error = Missing;
    /// Returns the node, or the field its kind lacks.
    fn try_from(form: NodeForm) -> Result<Self, Missing> {
        form.into_node().map(Self).map_err(Missing)
    }
}

/// The kind of a type, as a document names it: [`Type`]'s variants in
/// their order, so that an unknown kind is refused with their list.
#[derive(Clone, Copy)]
enum TypeKind {
    /// [`Type::Atom`]
    Atom,
    /// [`Type::One`]
    One,
    /// [`Type::Top`]
    Top,
    /// [`Type::Zero`]
    Zero,
    /// [`Type::Tensor`]
    Tensor,
    /// [`Type::With`]
    With,
    /// [`Type::Plus`]
    Plus,
    /// [`Type::Lollipop`]
    Lollipop,
    /// [`Type::Bang`]
    Bang,
    /// [`Type::Forall`]
    Forall,
    /// [`Type::Bound`]
    Bound,
    /// [`Type::Variable`]
    Variable,
}

impl TypeKind {
    /// The kinds' names, in the order of the variants.
    const NAMES: &'static [&'static str] = &[
        "atom", "one", "top", "zero", "tensor", "with", "plus", "lollipop", "bang", "forall",
        "bound", "variable",
    ];
    /// The kinds, in the order of [`NAMES`](Self::NAMES).
    const ALL: [Self; 12] = [
        Self::Atom,
        Self::One,
        Self::Top,
        Self::Zero,
        Self::Tensor,
        Self::With,
        Self::Plus,
        Self::Lollipop,
        Self::Bang,
        Self::Forall,
        Self::Bound,
        Self::Variable,
    ];
}

impl<'de> Deserialize<'de> for TypeKind {
    /// Reads a kind's name as serde reads an enum's tag.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        kind_index(deserializer, Self::NAMES).map(|index| Self::ALL[index])
    }
}

/// A type as a document holds it: its kind and every field a kind has,
/// read where they stand. A derived reader of [`Type`], tagged by `kind`,
/// would hold the whole object first and read it twice, which is twice
/// its code and buffers every type. A name no kind has is ignored; a
/// field of another kind is read, so one of the wrong type is refused.
#[derive(Deserialize)]
struct TypeForm {
    /// The kind.
    kind: TypeKind,
    /// The atom's index.
    atom: Option<u32>,
    /// The left operand.
    left: Option<u32>,
    /// The right operand.
    right: Option<u32>,
    /// The antecedent of an implication.
    from: Option<u32>,
    /// The consequent of an implication.
    to: Option<u32>,
    /// The operand of `!`.
    of: Option<u32>,
    /// The type variable.
    variable: Option<u32>,
    /// The body of a `∀`.
    body: Option<u32>,
    /// The de Bruijn index of a bound type variable.
    index: Option<u32>,
}

impl TypeForm {
    /// Returns the type, or the name of a field its kind lacks.
    fn into_type(self) -> Result<Type, &'static str> {
        let need = |field: Option<u32>, name: &'static str| field.ok_or(name);
        let ty = |field, name| need(field, name).map(TypeId);
        Ok(match self.kind {
            TypeKind::Atom => Type::Atom {
                atom: need(self.atom, "atom")?,
            },
            TypeKind::One => Type::One,
            TypeKind::Top => Type::Top,
            TypeKind::Zero => Type::Zero,
            TypeKind::Tensor => Type::Tensor {
                left: ty(self.left, "left")?,
                right: ty(self.right, "right")?,
            },
            TypeKind::With => Type::With {
                left: ty(self.left, "left")?,
                right: ty(self.right, "right")?,
            },
            TypeKind::Plus => Type::Plus {
                left: ty(self.left, "left")?,
                right: ty(self.right, "right")?,
            },
            TypeKind::Lollipop => Type::Lollipop {
                from: ty(self.from, "from")?,
                to: ty(self.to, "to")?,
            },
            TypeKind::Bang => Type::Bang {
                of: ty(self.of, "of")?,
            },
            TypeKind::Forall => Type::Forall {
                variable: need(self.variable, "variable").map(TypeVarId)?,
                body: ty(self.body, "body")?,
            },
            TypeKind::Bound => Type::Bound {
                index: need(self.index, "index")?,
            },
            TypeKind::Variable => Type::Variable {
                variable: need(self.variable, "variable").map(TypeVarId)?,
            },
        })
    }
}

/// The kind of a node, as a document names it: [`Node`]'s variants in
/// their order, so that an unknown kind is refused with their list.
#[derive(Clone, Copy)]
enum NodeKind {
    /// [`Node::Var`]
    Var,
    /// [`Node::Lambda`]
    Lambda,
    /// [`Node::Apply`]
    Apply,
    /// [`Node::Tensor`]
    Tensor,
    /// [`Node::LetTensor`]
    LetTensor,
    /// [`Node::Unit`]
    Unit,
    /// [`Node::LetUnit`]
    LetUnit,
    /// [`Node::Pair`]
    Pair,
    /// [`Node::First`]
    First,
    /// [`Node::Second`]
    Second,
    /// [`Node::Left`]
    Left,
    /// [`Node::Right`]
    Right,
    /// [`Node::Case`]
    Case,
    /// [`Node::True`]
    True,
    /// [`Node::False`]
    False,
    /// [`Node::Promote`]
    Promote,
    /// [`Node::Derelict`]
    Derelict,
    /// [`Node::Discard`]
    Discard,
    /// [`Node::Copy`]
    Copy,
    /// [`Node::TypeLambda`]
    TypeLambda,
    /// [`Node::TypeApply`]
    TypeApply,
}

impl NodeKind {
    /// The kinds' names, in the order of the variants.
    const NAMES: &'static [&'static str] = &[
        "var",
        "lambda",
        "apply",
        "tensor",
        "let_tensor",
        "unit",
        "let_unit",
        "pair",
        "first",
        "second",
        "left",
        "right",
        "case",
        "true",
        "false",
        "promote",
        "derelict",
        "discard",
        "copy",
        "type_lambda",
        "type_apply",
    ];
    /// The kinds, in the order of [`NAMES`](Self::NAMES).
    const ALL: [Self; 21] = [
        Self::Var,
        Self::Lambda,
        Self::Apply,
        Self::Tensor,
        Self::LetTensor,
        Self::Unit,
        Self::LetUnit,
        Self::Pair,
        Self::First,
        Self::Second,
        Self::Left,
        Self::Right,
        Self::Case,
        Self::True,
        Self::False,
        Self::Promote,
        Self::Derelict,
        Self::Discard,
        Self::Copy,
        Self::TypeLambda,
        Self::TypeApply,
    ];
}

impl<'de> Deserialize<'de> for NodeKind {
    /// Reads a kind's name as serde reads an enum's tag.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        kind_index(deserializer, Self::NAMES).map(|index| Self::ALL[index])
    }
}

/// A node as a document holds it: its kind and every field a kind has,
/// read where they stand, as [`TypeForm`] reads a type. A name that is a
/// term in one kind and a variable or a type in another (`left`,
/// `variable`, `argument`) is read as an index, which the kind then names.
#[derive(Deserialize)]
struct NodeForm {
    /// The kind.
    kind: NodeKind,
    /// The variable bound or used.
    variable: Option<u32>,
    /// The body.
    body: Option<u32>,
    /// The function applied.
    function: Option<u32>,
    /// The argument.
    argument: Option<u32>,
    /// The left part.
    left: Option<u32>,
    /// The right part.
    right: Option<u32>,
    /// The term taken apart.
    term: Option<u32>,
    /// The sum an injection goes into.
    sum: Option<u32>,
    /// A case's left branch.
    left_body: Option<u32>,
    /// A case's right branch.
    right_body: Option<u32>,
    /// The terms of the context a `⊤` or a `0` takes.
    context: Option<Vec<u32>>,
    /// The type a `0` proves.
    goal: Option<u32>,
    /// The terms a promotion takes.
    terms: Option<Vec<u32>>,
    /// The variables a promotion binds.
    variables: Option<Vec<u32>>,
}

impl NodeForm {
    /// Returns the node, or the name of a field its kind lacks.
    fn into_node(self) -> Result<Node, &'static str> {
        let need = |field: Option<u32>, name: &'static str| field.ok_or(name);
        let term = |field, name| need(field, name).map(TermId);
        let var = |field, name| need(field, name).map(VarId);
        let ty = |field, name| need(field, name).map(TypeId);
        let terms = |field: Option<Vec<u32>>, name: &'static str| {
            field
                .ok_or(name)
                .map(|ids| ids.into_iter().map(TermId).collect())
        };
        Ok(match self.kind {
            NodeKind::Var => Node::Var {
                variable: var(self.variable, "variable")?,
            },
            NodeKind::Lambda => Node::Lambda {
                variable: var(self.variable, "variable")?,
                body: term(self.body, "body")?,
            },
            NodeKind::Apply => Node::Apply {
                function: term(self.function, "function")?,
                argument: term(self.argument, "argument")?,
            },
            NodeKind::Tensor => Node::Tensor {
                left: term(self.left, "left")?,
                right: term(self.right, "right")?,
            },
            NodeKind::LetTensor => Node::LetTensor {
                term: term(self.term, "term")?,
                left: var(self.left, "left")?,
                right: var(self.right, "right")?,
                body: term(self.body, "body")?,
            },
            NodeKind::Unit => Node::Unit,
            NodeKind::LetUnit => Node::LetUnit {
                term: term(self.term, "term")?,
                body: term(self.body, "body")?,
            },
            NodeKind::Pair => Node::Pair {
                left: term(self.left, "left")?,
                right: term(self.right, "right")?,
            },
            NodeKind::First => Node::First {
                term: term(self.term, "term")?,
            },
            NodeKind::Second => Node::Second {
                term: term(self.term, "term")?,
            },
            NodeKind::Left => Node::Left {
                term: term(self.term, "term")?,
                sum: ty(self.sum, "sum")?,
            },
            NodeKind::Right => Node::Right {
                term: term(self.term, "term")?,
                sum: ty(self.sum, "sum")?,
            },
            NodeKind::Case => Node::Case {
                term: term(self.term, "term")?,
                left: var(self.left, "left")?,
                left_body: term(self.left_body, "left_body")?,
                right: var(self.right, "right")?,
                right_body: term(self.right_body, "right_body")?,
            },
            NodeKind::True => Node::True {
                context: terms(self.context, "context")?,
            },
            NodeKind::False => Node::False {
                context: terms(self.context, "context")?,
                term: term(self.term, "term")?,
                goal: ty(self.goal, "goal")?,
            },
            NodeKind::Promote => Node::Promote {
                terms: terms(self.terms, "terms")?,
                variables: self
                    .variables
                    .ok_or("variables")?
                    .into_iter()
                    .map(VarId)
                    .collect(),
                body: term(self.body, "body")?,
            },
            NodeKind::Derelict => Node::Derelict {
                term: term(self.term, "term")?,
            },
            NodeKind::Discard => Node::Discard {
                term: term(self.term, "term")?,
                body: term(self.body, "body")?,
            },
            NodeKind::Copy => Node::Copy {
                term: term(self.term, "term")?,
                left: var(self.left, "left")?,
                right: var(self.right, "right")?,
                body: term(self.body, "body")?,
            },
            NodeKind::TypeLambda => Node::TypeLambda {
                variable: need(self.variable, "variable").map(TypeVarId)?,
                body: term(self.body, "body")?,
            },
            NodeKind::TypeApply => Node::TypeApply {
                term: term(self.term, "term")?,
                argument: ty(self.argument, "argument")?,
            },
        })
    }
}

impl<'a> Deserialize<'a> for Term {
    /// Deserializes a term within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Term {}

impl Readable for Term {
    const FORM: &'static str = FORM;

    /// Reads a term, its parts within the limits and its shape checked.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = TermForm::deserialize(deserializer)?;
        let types = form.types.into_iter().map(|ReadType(ty)| ty).collect();
        let nodes = form.nodes.into_iter().map(|ReadNode(node)| node).collect();
        let parts = Parts {
            calculus: form.calculus,
            atoms: form.atoms,
            types,
            variables: form
                .variables
                .into_iter()
                .map(|variable| (variable.ty, variable.name))
                .collect(),
            type_variables: form.type_variables,
            context: form.context,
            goal: form.goal,
            nodes,
            root: form.root,
        };
        Term::from_parts(parts, limits).map_err(wire::fail)
    }
}

/// The written form of a type's terms listed and counted.
#[derive(Serialize)]
struct InhabitantsForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// How many terms the type has.
    count: Count,
    /// The terms listed.
    terms: &'a [Term],
}

impl Serialize for Inhabitants {
    /// Serializes the terms and their count as a document at level 24,
    /// written only.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        InhabitantsForm {
            version: wire::INHABITANTS,
            deduxit: env!("CARGO_PKG_VERSION"),
            count: self.count,
            terms: &self.terms,
        }
        .serialize(serializer)
    }
}

/// The written form of whether a type has a term.
#[derive(Serialize)]
struct InhabitationForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// `inhabited`, `empty` or `unknown`.
    verdict: &'static str,
    /// The term, for `inhabited`.
    #[serde(skip_serializing_if = "Option::is_none")]
    term: Option<&'a Term>,
    /// The Kripke model that refutes the type's formula, for `empty` where
    /// the engine gave one.
    #[serde(skip_serializing_if = "Option::is_none")]
    countermodel: Option<CountermodelForm>,
    /// Why the engines did not decide, for `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<Reason>,
}

impl Serialize for Inhabitation {
    /// Serializes the answer as a document at level 24, written only.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (verdict, term, countermodel, reason) = match self {
            Self::Inhabited(term) => ("inhabited", Some(&**term), None, None),
            Self::Empty { countermodel } => (
                "empty",
                None,
                countermodel.as_ref().map(CountermodelForm::of),
                None,
            ),
            Self::Unknown(reason) => ("unknown", None, None, Some(*reason)),
        };
        InhabitationForm {
            version: wire::INHABITANTS,
            deduxit: env!("CARGO_PKG_VERSION"),
            verdict,
            term,
            countermodel,
            reason,
        }
        .serialize(serializer)
    }
}
