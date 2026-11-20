// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The wire form of a lambda-term: a document of its own at the level
//! of the term's form, read with its shape checked.

use crate::lambda::read::{FORM, Parts};
use crate::lambda::{Calculus, Node, Term, TermId, Type, TypeId, VarId, Variable};
use crate::limits::Limits;
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A term as it is written: the level first, then the term's parts.
#[derive(Serialize)]
struct Written<'a> {
    /// The wire level.
    version: u32,
    /// The calculus.
    calculus: Calculus,
    /// The atoms' names.
    atoms: &'a [Box<str>],
    /// The types, operands first.
    types: &'a [Type],
    /// The variables.
    variables: &'a [Variable],
    /// The variables of the hypotheses, in the order written.
    context: &'a [VarId],
    /// The term's type.
    goal: TypeId,
    /// The nodes, operands first.
    nodes: &'a [Node],
    /// The whole term.
    root: TermId,
}

impl Serialize for Term {
    /// Serializes the term as a document at the level of the term's
    /// form.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Written {
            version: wire::LAMBDA,
            calculus: self.calculus(),
            atoms: self.atoms(),
            types: self.types(),
            variables: self.variables(),
            context: self.context(),
            goal: self.goal(),
            nodes: self.nodes(),
            root: self.root(),
        }
        .serialize(serializer)
    }
}

/// A variable as a document holds it.
#[derive(Deserialize)]
struct VariableForm {
    /// The type.
    #[serde(rename = "type")]
    ty: TypeId,
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
    /// The atoms' names.
    atoms: Vec<String>,
    /// The types, operands first.
    types: Vec<Type>,
    /// The variables.
    variables: Vec<VariableForm>,
    /// The variables of the hypotheses, in the order written.
    context: Vec<VarId>,
    /// The term's type.
    goal: TypeId,
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
            context: form.context,
            goal: form.goal,
            nodes: form.nodes,
            root: form.root,
        };
        Term::from_parts(parts, limits).map_err(wire::fail)
    }
}
