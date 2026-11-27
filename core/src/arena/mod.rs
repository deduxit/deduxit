// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The hash-consed arena of formulas that ordinary and modal logic share:
//! every subformula before the formulas built from it, equal formulas one
//! node, atoms by name. Each suite's `Formulas` holds one, over its own
//! nodes, and adds what only it has (ordinary logic's first-order tables).

#[cfg(feature = "parse")]
pub(crate) mod parse;

use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Refusal, Space};
use std::collections::hash_map::Entry;
use std::fmt::{Debug, DebugStruct};
use std::hash::Hash;

/// The index of a node in an arena: a `u32`, since arenas hold fewer than
/// 2³² nodes.
pub(crate) trait Id: Copy + Debug + Eq + Hash {
    /// Returns the id of the node at `index`.
    fn at(index: u32) -> Self;

    /// Returns the index of the node.
    fn place(self) -> usize;
}

/// A node of an arena: a connective, a constant or an atom, its operands
/// ids of nodes before it.
pub(crate) trait Shape: Copy + Debug + Eq + Hash {
    /// The id of a node.
    type Id: Id;

    /// Returns the subformulas, in the order they are written.
    fn operands(self) -> (Option<Self::Id>, Option<Self::Id>);

    /// Returns the index among the arena's names of the atom this node
    /// is, if it is one.
    fn atom(self) -> Option<u32>;
}

/// The nodes of an arena, its atom names and the maps that share both.
#[derive(Clone)]
pub(crate) struct Arena<N: Shape> {
    /// The nodes, each after its subformulas.
    nodes: Vec<N>,
    /// The atom names, by the index an atom node holds.
    atoms: Vec<String>,
    /// The index of every atom name.
    names: HashMap<String, u32>,
    /// The id of every node, for sharing.
    ids: HashMap<N, N::Id>,
}

impl<N: Shape> Default for Arena<N> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            atoms: Vec::new(),
            names: HashMap::default(),
            ids: HashMap::default(),
        }
    }
}

impl<N: Shape> Arena<N> {
    /// The most nodes an arena holds: the ids are `u32`.
    pub(crate) const MOST: usize = u32::MAX as usize;

    /// Adds the arena's fields to the `Debug` form of the `Formulas` that
    /// holds it, under the names they had there.
    pub(crate) fn fields(&self, out: &mut DebugStruct<'_, '_>) {
        out.field("nodes", &self.nodes)
            .field("atoms", &self.atoms)
            .field("names", &self.names)
            .field("ids", &self.ids);
    }

    /// Returns the node with the id given.
    ///
    /// # Panics
    ///
    /// If the id is past the arena's end.
    pub(crate) fn node(&self, id: N::Id) -> N {
        self.nodes[id.place()]
    }

    /// Returns the nodes, each after its subformulas.
    pub(crate) fn nodes(&self) -> &[N] {
        &self.nodes
    }

    /// Returns the number of nodes.
    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the name of the atom with the index given.
    pub(crate) fn atom_name(&self, atom: u32) -> &str {
        &self.atoms[atom as usize]
    }

    /// Returns the atom names, in the order of their indices.
    pub(crate) fn atom_names(&self) -> &[String] {
        &self.atoms
    }

    /// Returns the id of a node the arena holds, if it holds it.
    pub(crate) fn find(&self, node: N) -> Option<N::Id> {
        self.ids.get(&node).copied()
    }

    /// Returns the id of `node`, adding it unless the arena has it.
    ///
    /// # Errors
    ///
    /// As [`admit`](Self::admit).
    pub(crate) fn add(&mut self, node: N) -> Result<N::Id, Error> {
        if let Some(id) = self.find(node) {
            return Ok(id);
        }
        self.admit(node)?;
        Ok(self.push(node))
    }

    /// Fails unless the arena may take `node` as a new node.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a subformula that is no node of
    /// this arena (an id of another one) or an atom that is not one of its
    /// names, and [`Refusal::Index`] when the arena is full.
    #[cfg_attr(
        target_pointer_width = "32",
        expect(
            clippy::absurd_extreme_comparisons,
            reason = "`MOST` is `usize::MAX` on 32 bits, where the test still holds"
        )
    )]
    pub(crate) fn admit(&self, node: N) -> Result<(), Error> {
        let (a, b) = node.operands();
        for id in [a, b].into_iter().flatten() {
            self.holds(id)?;
        }
        if let Some(atom) = node.atom()
            && atom as usize >= self.atoms.len()
        {
            return Err(Error::IndexOutOfBounds {
                space: Space::Atom,
                index: atom as usize,
                len: self.atoms.len(),
            });
        }
        if self.nodes.len() >= Self::MOST {
            return Err(Error::Refused(Refusal::Index {
                what: Space::Formula,
                count: Self::MOST as u64 + 1,
                most: Self::MOST as u64,
            }));
        }
        Ok(())
    }

    /// Adds `node`, which [`admit`](Self::admit) took and the arena does
    /// not hold, and returns its id.
    pub(crate) fn push(&mut self, node: N) -> N::Id {
        let id = N::Id::at(self.nodes.len() as u32);
        self.nodes.push(node);
        self.ids.insert(node, id);
        id
    }

    /// Fails unless `id` is a node of this arena.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for an id past the arena's end.
    pub(crate) fn holds(&self, id: N::Id) -> Result<(), Error> {
        if id.place() < self.nodes.len() {
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds {
                space: Space::Formula,
                index: id.place(),
                len: self.nodes.len(),
            })
        }
    }

    /// Returns the index of the atom named `name`, adding the name unless
    /// the arena has it. The name is kept in Unicode normal form NFC, and
    /// a new one must pass `name::check_ordinary` and then `new`, which
    /// sees it in NFC before it is added.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// and what `new` answers.
    pub(crate) fn atom(
        &mut self,
        name: &str,
        new: impl FnOnce(&str) -> Result<(), Error>,
    ) -> Result<u32, Error> {
        let name = &*crate::name::normalized(name);
        Ok(match self.names.get(name) {
            Some(&index) => index,
            None => {
                crate::name::check_ordinary(name)?;
                new(name)?;
                let index = self.atoms.len() as u32;
                self.atoms.push(name.to_owned());
                self.names.insert(name.to_owned(), index);
                index
            }
        })
    }

    /// Returns the index of the atom keyed `key`, which the caller has
    /// checked, adding it unless the arena has it, and whether it is new.
    pub(crate) fn keyed(&mut self, key: String) -> (u32, bool) {
        let len = self.atoms.len() as u32;
        match self.names.entry(key) {
            Entry::Occupied(found) => (*found.get(), false),
            Entry::Vacant(vacant) => {
                self.atoms.push(vacant.key().clone());
                vacant.insert(len);
                (len, true)
            }
        }
    }

    /// Returns the index of the atom keyed `key`, if the arena has it.
    pub(crate) fn keyed_index(&self, key: &str) -> Option<u32> {
        self.names.get(key).copied()
    }

    /// Replaces every atom's key by the one of `keys` at its index, as
    /// many keys as atoms; a key given twice names the later index.
    #[cfg(feature = "interactive")]
    pub(crate) fn rekey(&mut self, keys: Vec<String>) {
        self.names.clear();
        for (i, key) in keys.into_iter().enumerate() {
            // As many atoms as a `u32` numbers.
            self.names.insert(key.clone(), i as u32);
            self.atoms[i] = key;
        }
    }

    /// Returns the number of subformula occurrences the formulas at
    /// `roots` unfold to, saturating.
    fn occurrences<'a>(&self, roots: impl Iterator<Item = &'a N::Id>) -> u64
    where
        N::Id: 'a,
    {
        // Every subformula precedes the formulas built from it.
        let mut size: Vec<u64> = Vec::with_capacity(self.nodes.len());
        for node in &self.nodes {
            let (a, b) = node.operands();
            let of = |k: Option<N::Id>| k.map_or(0, |k| size[k.place()]);
            size.push(1u64.saturating_add(of(a)).saturating_add(of(b)));
        }
        roots.fold(0, |sum, id| sum.saturating_add(size[id.place()]))
    }

    /// Checks that the sequent `left ⊢ right` over the arena unfolds to at
    /// most `limits.occurrences` subformula occurrences.
    ///
    /// # Errors
    ///
    /// [`Refusal::Occurrences`] past the bound.
    pub(crate) fn bound(
        &self,
        left: &[N::Id],
        right: &[N::Id],
        limits: &crate::Limits,
    ) -> Result<(), Error> {
        let occurrences = self.occurrences(left.iter().chain(right));
        match limits.occurrences {
            Some(limit) if occurrences > limit => {
                Err(Error::Refused(Refusal::Occurrences { occurrences, limit }))
            }
            _ => Ok(()),
        }
    }
}
