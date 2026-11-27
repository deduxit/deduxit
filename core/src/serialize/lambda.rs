// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The wire form of a lambda-term: a document of its own at the level
//! of the term's form, read with its shape checked.

use super::ordinary::CountermodelForm;
use crate::lambda::read::{FORM, Parts};
use crate::lambda::{
    Calculus, Count, End, Inhabitants, Inhabitation, Node, Reduction, Strategy, Term, TermId, Type,
    TypeId, VarId, Variable,
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
    types: Vec<Type>,
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
    nodes: Vec<Node>,
    /// The whole term.
    root: TermId,
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
        let parts = Parts {
            calculus: form.calculus,
            atoms: form.atoms,
            types: form.types,
            variables: form
                .variables
                .into_iter()
                .map(|variable| (variable.ty, variable.name))
                .collect(),
            type_variables: form.type_variables,
            context: form.context,
            goal: form.goal,
            nodes: form.nodes,
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
