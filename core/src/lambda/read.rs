// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A term made from the parts a document holds, once its shape is
//! checked: every index names what it should, and what a node names
//! comes before it. Whether the term is well typed is
//! [`Term::check`]'s to say.

use super::{Calculus, Node, Term, TermId, Type, TypeId, Types, VarId, Variable, names_variable};
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
    /// Each variable's type and name.
    pub(crate) variables: Vec<(TypeId, String)>,
    /// The variables of the hypotheses.
    pub(crate) context: Vec<VarId>,
    /// The term's type.
    pub(crate) goal: TypeId,
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
        Type::Atom { .. } | Type::One | Type::Top | Type::Zero => {}
        Type::Bang { of } => f(of),
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
        Type::Atom { .. } | Type::One | Type::Top | Type::Zero => ty,
        Type::Bang { of } => Type::Bang { of: f(of) },
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
        | Node::Discard { .. } => {}
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
            context,
            goal,
            mut nodes,
            root,
        } = parts;
        let count = [
            atoms.len(),
            types.len(),
            variables.len(),
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
        // The atoms, one index per name: `merged` maps the document's
        // index. A name is composed as the parsers compose it, so that
        // two spellings of one name are one atom.
        let mut indices: HashMap<Box<str>, u32> = HashMap::default();
        let mut merged: Vec<u32> = Vec::with_capacity(atoms.len());
        let mut names: Vec<Box<str>> = Vec::new();
        for atom in &atoms {
            if calculus == Calculus::Simple {
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
                Type::One
                | Type::Top
                | Type::Zero
                | Type::Tensor { .. }
                | Type::With { .. }
                | Type::Plus { .. }
                | Type::Lollipop { .. }
                | Type::Bang { .. } => ty,
            };
            if !fits {
                return Err(malformed(format!(
                    "type {i} names a type that does not come before it, or an atom past the list"
                )));
            }
            ids.push(interned.add(retyped(ty, |operand| ids[operand.index()])));
        }
        let known = |ty: TypeId, what: &str| {
            ids.get(ty.index())
                .copied()
                .ok_or_else(|| malformed(format!("{what} names a type past the list")))
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
                    ty: known(ty, "a variable")?,
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
                | Node::False { goal: ty, .. } => *ty = known(*ty, "a node")?,
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
            goal: known(goal, "`goal`")?,
            types: interned.list,
            variables,
            context,
            nodes,
            root,
        })
    }
}
