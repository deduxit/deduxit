// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A term made from the parts a document holds, once its shape is
//! checked: every index names what it should, and what a node names
//! comes before it. Whether the term is well typed is
//! [`Term::check`]'s to say.

use super::{
    Calculus, Node, Term, TermId, Type, TypeId, TypeVarId, Types, VarId, Variable, names_variable,
    poly,
};
use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Refusal};
use crate::name;

/// The form's name, as an error names it.
pub(crate) const FORM: &str = "lambda-term";

/// A term as a document holds it.
pub(crate) struct Parts {
    /// The calculus.
    pub(crate) calculus: Calculus,
    /// The atoms' names.
    pub(crate) atoms: Vec<String>,
    /// The types, operands first.
    pub(crate) types: Vec<Type>,
    /// Each variable's type, none in an untyped term, and name.
    pub(crate) variables: Vec<(Option<TypeId>, String)>,
    /// The type variables' names, none outside System F.
    pub(crate) type_variables: Vec<String>,
    /// The variables of the hypotheses.
    pub(crate) context: Vec<VarId>,
    /// The term's type, none in an untyped term.
    pub(crate) goal: Option<TypeId>,
    /// The nodes, operands first.
    pub(crate) nodes: Vec<Node>,
    /// The whole term.
    pub(crate) root: TermId,
}

/// Returns the error for a document whose parts are no term.
fn malformed(message: String) -> Error {
    Error::Json {
        form: FORM,
        message,
    }
}

/// Hands every type a type names to `f`.
fn type_operands(ty: Type, mut f: impl FnMut(TypeId)) {
    match ty {
        Type::Atom { .. }
        | Type::One
        | Type::Top
        | Type::Zero
        | Type::Bound { .. }
        | Type::Variable { .. } => {}
        Type::Bang { of } | Type::Forall { body: of, .. } => f(of),
        Type::Tensor { left, right } | Type::With { left, right } | Type::Plus { left, right } => {
            f(left);
            f(right);
        }
        Type::Lollipop { from, to } => {
            f(from);
            f(to);
        }
    }
}

/// Returns a type with every type it names replaced by what `f` maps it
/// to.
fn retyped(ty: Type, f: impl Fn(TypeId) -> TypeId) -> Type {
    match ty {
        Type::Atom { .. }
        | Type::One
        | Type::Top
        | Type::Zero
        | Type::Bound { .. }
        | Type::Variable { .. } => ty,
        Type::Bang { of } => Type::Bang { of: f(of) },
        Type::Forall { variable, body } => Type::Forall {
            variable,
            body: f(body),
        },
        Type::Tensor { left, right } => Type::Tensor {
            left: f(left),
            right: f(right),
        },
        Type::With { left, right } => Type::With {
            left: f(left),
            right: f(right),
        },
        Type::Plus { left, right } => Type::Plus {
            left: f(left),
            right: f(right),
        },
        Type::Lollipop { from, to } => Type::Lollipop {
            from: f(from),
            to: f(to),
        },
    }
}

/// Hands every variable a node names to `f`.
fn variables_of(node: &Node, mut f: impl FnMut(VarId)) {
    match node {
        Node::Var { variable } | Node::Lambda { variable, .. } => f(*variable),
        Node::LetTensor { left, right, .. }
        | Node::Case { left, right, .. }
        | Node::Copy { left, right, .. } => {
            f(*left);
            f(*right);
        }
        Node::Promote { variables, .. } => variables.iter().copied().for_each(f),
        Node::Apply { .. }
        | Node::Tensor { .. }
        | Node::Unit
        | Node::LetUnit { .. }
        | Node::Pair { .. }
        | Node::First { .. }
        | Node::Second { .. }
        | Node::Left { .. }
        | Node::Right { .. }
        | Node::True { .. }
        | Node::False { .. }
        | Node::Derelict { .. }
        | Node::Discard { .. }
        | Node::TypeLambda { .. }
        | Node::TypeApply { .. } => {}
    }
}

impl Term {
    /// Returns the term a document's parts hold, its types made one id
    /// per type, or why they are no term. The parts together are within
    /// `limits.occurrences`.
    pub(crate) fn from_parts(parts: Parts, limits: &Limits) -> Result<Self, Error> {
        let Parts {
            calculus,
            atoms,
            types,
            variables,
            type_variables,
            context,
            goal,
            mut nodes,
            root,
        } = parts;
        let count = [
            atoms.len(),
            types.len(),
            variables.len(),
            type_variables.len(),
            context.len(),
            nodes.len(),
        ]
        .iter()
        .fold(0u64, |sum, &n| sum.saturating_add(n as u64));
        if let Some(limit) = limits.occurrences.filter(|&limit| count > limit) {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: count,
                limit,
            }));
        }
        // Every index is a `u32`, so every list is shorter than 2³².
        if count >= u64::from(u32::MAX) {
            return Err(malformed("it has 2³² parts or more".to_owned()));
        }
        // A typed term has every type, an untyped one none.
        let typed = calculus.is_typed();
        if !typed && (!atoms.is_empty() || !types.is_empty()) {
            return Err(malformed(
                "an untyped term has no atoms and no types".to_owned(),
            ));
        }
        if goal.is_some() != typed || variables.iter().any(|(ty, _)| ty.is_some() != typed) {
            return Err(malformed(if typed {
                "a typed term gives its `goal` and every variable's `type`".to_owned()
            } else {
                "an untyped term gives no `goal` and no variable's `type`".to_owned()
            }));
        }
        if !calculus.is_polymorphic() && !type_variables.is_empty() {
            return Err(malformed(
                "only a term of System F has type variables".to_owned(),
            ));
        }
        // A type variable is named as an atom may be, and a name's
        // spellings are one.
        let type_variables = type_variables
            .iter()
            .map(|name| {
                if calculus.is_ordinary() {
                    name::check_ordinary(name)?;
                } else {
                    name::check(name)?;
                }
                Ok(name::normalized(name).into())
            })
            .collect::<Result<Vec<Box<str>>, Error>>()?;
        let named = |v: TypeVarId| v.index() < type_variables.len();
        // The atoms, one index per name: `merged` maps the document's
        // index. A name is composed as the parsers compose it, so that
        // two spellings of one name are one atom.
        let mut indices: HashMap<Box<str>, u32> = HashMap::default();
        let mut merged: Vec<u32> = Vec::with_capacity(atoms.len());
        let mut names: Vec<Box<str>> = Vec::new();
        for atom in &atoms {
            if calculus.is_ordinary() {
                name::check_ordinary(atom)?;
            } else {
                name::check(atom)?;
            }
            let composed: Box<str> = name::normalized(atom).into();
            let index = *indices.entry(composed.clone()).or_insert_with(|| {
                names.push(composed);
                // Fewer names than the parts counted above.
                (names.len() - 1) as u32
            });
            merged.push(index);
        }
        // The types, one id per type: `ids` maps the document's index.
        let mut interned = Types::default();
        let mut ids: Vec<TypeId> = Vec::with_capacity(types.len());
        for (i, &ty) in types.iter().enumerate() {
            let mut fits = true;
            type_operands(ty, |operand| fits &= operand.index() < i);
            let ty = match ty {
                Type::Atom { atom } => match merged.get(atom as usize) {
                    Some(&atom) => Type::Atom { atom },
                    None => {
                        fits = false;
                        ty
                    }
                },
                Type::Forall { variable, .. } | Type::Variable { variable } => {
                    fits &= named(variable);
                    ty
                }
                Type::One
                | Type::Top
                | Type::Zero
                | Type::Tensor { .. }
                | Type::With { .. }
                | Type::Plus { .. }
                | Type::Lollipop { .. }
                | Type::Bang { .. }
                | Type::Bound { .. } => ty,
            };
            if !fits {
                return Err(malformed(format!(
                    "type {i} names a type that does not come before it, or an atom or a type variable past the list"
                )));
            }
            ids.push(interned.add(retyped(ty, |operand| ids[operand.index()])));
        }
        // A type a term writes is closed: every `bound` index names a
        // `forall` inside it.
        let needs = poly::needs(&interned.list);
        let known = |ty: TypeId, what: &str| {
            let id = ids
                .get(ty.index())
                .copied()
                .ok_or_else(|| malformed(format!("{what} names a type past the list")))?;
            if needs[id.index()] > 0 {
                return Err(malformed(format!(
                    "{what} names a type with a `bound` index that no `forall` around it binds"
                )));
            }
            Ok(id)
        };
        let variables = variables
            .into_iter()
            .enumerate()
            .map(|(i, (ty, name))| {
                if !names_variable(&name) {
                    return Err(malformed(format!(
                        "variable {i} is named {name:?}, which is a keyword or no identifier"
                    )));
                }
                Ok(Variable {
                    ty: ty.map(|ty| known(ty, "a variable")).transpose()?,
                    name: name::normalized(&name).into(),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        for (i, node) in nodes.iter_mut().enumerate() {
            let mut fits = true;
            super::operands(node, |operand| fits &= operand.index() < i);
            variables_of(node, |variable| fits &= variable.index() < variables.len());
            if !fits {
                return Err(malformed(format!(
                    "node {i} names a node that does not come before it, or a variable past the list"
                )));
            }
            match node {
                Node::Left { sum: ty, .. }
                | Node::Right { sum: ty, .. }
                | Node::False { goal: ty, .. }
                | Node::TypeApply { argument: ty, .. } => *ty = known(*ty, "a node")?,
                Node::TypeLambda { variable, .. } => {
                    if !named(*variable) {
                        return Err(malformed(format!(
                            "node {i} names a type variable past the list"
                        )));
                    }
                }
                Node::Var { .. }
                | Node::Lambda { .. }
                | Node::Apply { .. }
                | Node::Tensor { .. }
                | Node::LetTensor { .. }
                | Node::Unit
                | Node::LetUnit { .. }
                | Node::Pair { .. }
                | Node::First { .. }
                | Node::Second { .. }
                | Node::Case { .. }
                | Node::True { .. }
                | Node::Promote { .. }
                | Node::Derelict { .. }
                | Node::Discard { .. }
                | Node::Copy { .. } => {}
            }
        }
        if root.index() >= nodes.len() {
            return Err(malformed("`root` names a node past the list".to_owned()));
        }
        let mut declared = vec![false; variables.len()];
        for &variable in &context {
            match declared.get_mut(variable.index()) {
                Some(seen) if !*seen => *seen = true,
                _ => {
                    return Err(malformed(
                        "`context` names a variable twice, or one past the list".to_owned(),
                    ));
                }
            }
        }
        Ok(Self {
            calculus,
            atoms: names,
            goal: goal.map(|goal| known(goal, "`goal`")).transpose()?,
            types: interned.list,
            variables,
            type_variables,
            context,
            nodes,
            root,
        })
    }
}
