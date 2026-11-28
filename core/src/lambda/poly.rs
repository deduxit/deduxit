// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The operations on the types of System F: a type abstraction's type
//! made from its body's (`close`), a type application's type made from
//! its function's (`open`), and a type variable replaced by a type
//! (`substitute`), for a type β step.
//!
//! A variable bound by a `∀` is its distance to its `∀` (a de Bruijn
//! index), and every type a term writes or a rule makes is closed: each
//! bound index points to a `∀` inside the type. So a type put in place of
//! a variable carries no index that a `∀` of the place could take in,
//! and no substitution captures. Each walk visits a type once per depth
//! of `∀` it is met at, on a stack of its own, and counts a unit of work
//! per visit in [`Types::work`].

use super::{Type, TypeId, TypeVarId, Types};
use crate::hash::HashMap;

/// Returns `root` with every leaf that `leaf` maps, at its depth of `∀`
/// in `root`, replaced by what it maps to; a type that `leaf` leaves
/// (`None`) is rebuilt from its operands.
fn rebuild(
    types: &mut Types,
    root: TypeId,
    leaf: impl Fn(&mut Types, TypeId, Type, u32) -> Option<TypeId>,
) -> TypeId {
    /// A step of the walk.
    enum Task {
        /// A type at a depth, its operands not yet rebuilt.
        Enter(TypeId, u32),
        /// The same, its operands rebuilt.
        Leave(TypeId, u32),
    }
    let mut made: HashMap<(TypeId, u32), TypeId> = HashMap::default();
    let mut stack = vec![Task::Enter(root, 0)];
    while let Some(task) = stack.pop() {
        match task {
            Task::Enter(t, depth) => {
                if made.contains_key(&(t, depth)) {
                    continue;
                }
                types.work = types.work.saturating_add(1);
                let ty = types.get(t);
                if let Some(to) = leaf(types, t, ty, depth) {
                    made.insert((t, depth), to);
                    continue;
                }
                stack.push(Task::Leave(t, depth));
                operands(ty, depth, |o, d| stack.push(Task::Enter(o, d)));
            }
            Task::Leave(t, depth) => {
                let ty = types.get(t);
                let new = |o: TypeId, d: u32| made[&(o, d)];
                // A `∀`'s body stands one binder deeper.
                let inner = if matches!(ty, Type::Forall { .. }) {
                    depth.saturating_add(1)
                } else {
                    depth
                };
                let rebuilt = ty.map_operands(|o| new(o, inner));
                let id = if rebuilt == ty { t } else { types.add(rebuilt) };
                made.insert((t, depth), id);
            }
        }
    }
    made[&(root, 0)]
}

/// Hands each operand of a type to `f` with the depth of `∀` it stands
/// at, the type itself at `depth`.
fn operands(ty: Type, depth: u32, mut f: impl FnMut(TypeId, u32)) {
    // A type of more than 2³² nested `∀` is not held in memory.
    let inner = if matches!(ty, Type::Forall { .. }) {
        depth.saturating_add(1)
    } else {
        depth
    };
    for operand in ty.operands() {
        f(operand, inner);
    }
}

/// Returns the type `∀X. A` of a type abstraction `ΛX. M` whose body `M`
/// has the type `A` (`body`), `X` being `variable`: every `X` in `A`
/// becomes bound by the new `∀`.
pub(crate) fn close(types: &mut Types, body: TypeId, variable: TypeVarId) -> TypeId {
    let closed = rebuild(types, body, |types, t, ty, depth| match ty {
        Type::Variable { variable: v } if v == variable => {
            Some(types.add(Type::Bound { index: depth }))
        }
        Type::Atom { .. }
        | Type::One
        | Type::Top
        | Type::Zero
        | Type::Bound { .. }
        | Type::Variable { .. } => Some(t),
        Type::Tensor { .. }
        | Type::With { .. }
        | Type::Plus { .. }
        | Type::Lollipop { .. }
        | Type::Bang { .. }
        | Type::Forall { .. } => None,
    });
    types.add(Type::Forall {
        variable,
        body: closed,
    })
}

/// Returns the type of a type application `M [A]` whose function `M` has
/// the type `∀X. B` with `body` its `B`, `A` being `argument`: `B` with
/// `A` for `X`. `argument` is closed, so a `∀` of `B` takes in nothing of
/// it.
pub(crate) fn open(types: &mut Types, body: TypeId, argument: TypeId) -> TypeId {
    rebuild(types, body, |_, t, ty, depth| match ty {
        Type::Bound { index } if index == depth => Some(argument),
        Type::Atom { .. }
        | Type::One
        | Type::Top
        | Type::Zero
        | Type::Bound { .. }
        | Type::Variable { .. } => Some(t),
        Type::Tensor { .. }
        | Type::With { .. }
        | Type::Plus { .. }
        | Type::Lollipop { .. }
        | Type::Bang { .. }
        | Type::Forall { .. } => None,
    })
}

/// Returns `ty` with every type variable of a `ΛX. M` that `by` maps to
/// a closed type replaced by it: a type β step `(ΛX. M) [A]` puts `A` for
/// `X` in the types of `M`, and a copy puts a type variable of its own
/// for each it copies.
pub(crate) fn substitute(
    types: &mut Types,
    ty: TypeId,
    by: impl Fn(TypeVarId) -> Option<TypeId>,
) -> TypeId {
    rebuild(types, ty, |_, t, found, _| match found {
        Type::Variable { variable } => Some(by(variable).unwrap_or(t)),
        Type::Atom { .. } | Type::One | Type::Top | Type::Zero | Type::Bound { .. } => Some(t),
        Type::Tensor { .. }
        | Type::With { .. }
        | Type::Plus { .. }
        | Type::Lollipop { .. }
        | Type::Bang { .. }
        | Type::Forall { .. } => None,
    })
}

/// Returns, for every type of an arena in order, how many `∀` around it
/// its bound indices need: 0 for a closed type. Each type's is made from
/// its operands', which come before it, so the walk is linear.
#[cfg(feature = "serialize")]
pub(crate) fn needs(types: &[Type]) -> Vec<u32> {
    let mut needs: Vec<u32> = Vec::with_capacity(types.len());
    for &ty in types {
        // An index of a type in memory is below 2³², and one more
        // saturates where it is the last `∀`'s.
        let mut need = match ty {
            Type::Bound { index } => index.saturating_add(1),
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
            | Type::Variable { .. } => 0,
        };
        operands(ty, 0, |o, depth| {
            need = need.max(needs[o.index()].saturating_sub(depth));
        });
        needs.push(need);
    }
    needs
}

/// A walk that finds the type variables of `ΛX. M` a type names, each
/// type of its DAG visited once per walk. It keeps nothing per type but a
/// mark, so that asking only for the types a term writes holds memory
/// linear in the arena: a list per type of the arena was quadratic for a
/// chain of arrows over as many variables.
pub(crate) struct Walker {
    /// The walk that last visited each type.
    marks: Vec<u32>,
    /// The current walk's mark.
    mark: u32,
    /// The types still to visit.
    stack: Vec<TypeId>,
}

impl Walker {
    /// Returns a walker over an arena of `types` types.
    pub(crate) fn new(types: usize) -> Self {
        Self {
            marks: vec![0; types],
            mark: 0,
            stack: Vec::new(),
        }
    }

    /// Returns the type variables `root` names, sorted and each once, and
    /// how many types the walk visited, its units of work.
    pub(crate) fn variables(&mut self, types: &[Type], root: TypeId) -> (Vec<TypeVarId>, u64) {
        if self.mark == u32::MAX {
            self.marks.fill(0);
            self.mark = 0;
        }
        self.mark += 1;
        let mut found = Vec::new();
        let mut visited = 0u64;
        self.stack.push(root);
        while let Some(t) = self.stack.pop() {
            if std::mem::replace(&mut self.marks[t.index()], self.mark) == self.mark {
                continue;
            }
            visited += 1;
            let ty = types[t.index()];
            if let Type::Variable { variable } = ty {
                found.push(variable);
            }
            operands(ty, 0, |o, _| self.stack.push(o));
        }
        found.sort_unstable();
        found.dedup();
        (found, visited)
    }
}
