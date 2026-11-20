// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The terms of a derivation: atoms and connectives applied to terms, in
//! an arena where equal terms are one id.

use crate::Error;
use crate::hash::{BuildHasher, HashMap};
use crate::limits::{Refusal, Space};
use crate::name;
use std::hash::BuildHasher as _;

/// A term of a [`Terms`] arena, by its index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TermId(u32);

impl TermId {
    /// Returns the id at `index`, a term of an arena only if it holds
    /// that many terms.
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the term's index in its arena.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// A term as [`Terms::term`] shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Term<'a> {
    /// A name the calculus does not declare: a propositional atom, a
    /// label.
    Atom {
        /// The atom's name.
        name: &'a str,
        /// The name of its sort.
        sort: &'a str,
    },
    /// A connective applied to terms.
    Apply {
        /// The connective's name.
        connective: &'a str,
        /// The terms it is applied to, each before this term in the
        /// arena.
        arguments: &'a [TermId],
    },
}

/// What a term is built with: an atom or a connective, by its index in
/// the arena's table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Head {
    /// The atom at this index.
    Atom(u32),
    /// The connective at this index.
    Connective(u32),
}

/// One term: its head and where its arguments lie.
#[derive(Clone, Copy, Debug)]
struct Entry {
    /// The atom or connective.
    head: Head,
    /// The index of its first argument among the arena's arguments.
    start: u32,
    /// How many arguments it has.
    len: u32,
}

/// No term: the end of a chain of terms with one hash.
const NONE: u32 = u32::MAX;

/// The terms of a [`Derivation`](super::Derivation): atoms and
/// connectives applied to terms. Every argument of a term lies before it,
/// and equal terms are one id, so comparing two formulas is comparing two
/// numbers.
///
/// The arena knows names only. Whether a connective exists, takes that
/// many arguments and takes them at those sorts is the calculus's to say,
/// when the derivation is checked.
#[derive(Clone, Debug, Default)]
pub struct Terms {
    /// The names of the connectives used, in the order first used.
    connectives: Vec<String>,
    /// The index of each connective's name.
    connective_ids: HashMap<String, u32>,
    /// The names of the atoms' sorts, in the order first used.
    sorts: Vec<String>,
    /// The index of each sort's name.
    sort_ids: HashMap<String, u32>,
    /// The atoms: each its name and the index of its sort.
    atoms: Vec<(String, u32)>,
    /// The index of each atom, by sort and name.
    atom_ids: HashMap<(u32, String), u32>,
    /// The terms.
    entries: Vec<Entry>,
    /// Every term's arguments, one after the other.
    arguments: Vec<TermId>,
    /// The latest term of each hash.
    latest: HashMap<u64, u32>,
    /// Per term, the term before it with the same hash, or [`NONE`].
    earlier: Vec<u32>,
}

impl Terms {
    /// The most terms an arena holds: an id is a `u32`, and one value
    /// marks the end of a chain.
    const MOST: usize = (u32::MAX - 1) as usize;

    /// Returns how many terms the arena holds.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the arena holds no term.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the term at `id`, which must be one of this arena.
    ///
    /// # Panics
    ///
    /// For an id of another arena, past this one's end.
    pub fn term(&self, id: TermId) -> Term<'_> {
        let entry = self.entries[id.index()];
        match entry.head {
            Head::Atom(atom) => {
                let (name, sort) = &self.atoms[atom as usize];
                Term::Atom {
                    name,
                    sort: &self.sorts[*sort as usize],
                }
            }
            Head::Connective(connective) => Term::Apply {
                connective: &self.connectives[connective as usize],
                arguments: self.arguments_of(entry),
            },
        }
    }

    /// Returns the atom named `name` of the sort named `sort`, adding it
    /// unless the arena has it. The name is kept in Unicode normal form
    /// NFC, so one name is one atom however its accents are encoded.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name or a sort that is no identifier (a
    /// letter or `_`, then letters, digits and `_`), and
    /// [`Refusal::Index`] when the arena is full.
    pub fn atom(&mut self, name: &str, sort: &str) -> Result<TermId, Error> {
        let name = &*name::normalized(name);
        for word in [name, sort] {
            if !name::is_identifier(word) {
                return Err(Error::AtomName {
                    name: word.to_owned(),
                });
            }
        }
        let sort = match self.sort_ids.get(sort) {
            Some(&index) => index,
            None => {
                // A sort is added with an atom, so there are no more
                // sorts than atoms, which `add` bounds.
                let index = self.sorts.len() as u32;
                self.sorts.push(sort.to_owned());
                self.sort_ids.insert(sort.to_owned(), index);
                index
            }
        };
        let key = (sort, name.to_owned());
        let atom = match self.atom_ids.get(&key) {
            Some(&atom) => atom,
            None => {
                let atom = self.atoms.len() as u32;
                self.atoms.push((key.1.clone(), sort));
                self.atom_ids.insert(key, atom);
                atom
            }
        };
        self.add(Head::Atom(atom), &[])
    }

    /// Returns the term that applies the connective named `connective` to
    /// `arguments`, adding it unless the arena has it.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for an argument that is no term of this
    /// arena, and [`Refusal::Index`] when the arena is full.
    pub fn apply(&mut self, connective: &str, arguments: &[TermId]) -> Result<TermId, Error> {
        if let Some(outside) = arguments.iter().find(|id| id.index() >= self.len()) {
            return Err(Error::IndexOutOfBounds {
                space: Space::Term,
                index: outside.index(),
                len: self.len(),
            });
        }
        let index = match self.connective_ids.get(connective) {
            Some(&index) => index,
            None => {
                let index = self.connectives.len() as u32;
                self.connectives.push(connective.to_owned());
                self.connective_ids.insert(connective.to_owned(), index);
                index
            }
        };
        self.add(Head::Connective(index), arguments)
    }

    /// Returns whether an atom of any sort has the name `name`.
    #[cfg(all(feature = "interactive", feature = "parse"))]
    pub(crate) fn names(&self, name: &str) -> bool {
        self.atoms.iter().any(|(atom, _)| atom == name)
    }

    /// Returns the names of the connectives the terms use, by index.
    pub(crate) fn connectives(&self) -> &[String] {
        &self.connectives
    }

    /// Returns the names of the sorts the atoms have, by index.
    pub(crate) fn sorts(&self) -> &[String] {
        &self.sorts
    }

    /// Returns the head of the term at `id` and its arguments.
    pub(crate) fn parts(&self, id: TermId) -> (Head, &[TermId]) {
        let entry = self.entries[id.index()];
        (entry.head, self.arguments_of(entry))
    }

    /// Returns the index of the sort of the atom at `atom`.
    pub(crate) fn sort_of(&self, atom: u32) -> u32 {
        self.atoms[atom as usize].1
    }

    /// Returns the term with this head and these arguments, if the arena
    /// has it. Nothing is added.
    pub(crate) fn find(&self, head: Head, arguments: &[TermId]) -> Option<TermId> {
        let mut at = *self.latest.get(&Self::hash(head, arguments))?;
        while at != NONE {
            let entry = self.entries[at as usize];
            if entry.head == head && self.arguments_of(entry) == arguments {
                return Some(TermId(at));
            }
            at = self.earlier[at as usize];
        }
        None
    }

    /// Returns the arguments of a term.
    fn arguments_of(&self, entry: Entry) -> &[TermId] {
        &self.arguments[entry.start as usize..][..entry.len as usize]
    }

    /// Returns the hash of a term with this head and these arguments.
    fn hash(head: Head, arguments: &[TermId]) -> u64 {
        BuildHasher::default().hash_one((head, arguments))
    }

    /// Returns the id of the term with this head and these arguments,
    /// which are terms of the arena, adding it unless the arena has it.
    fn add(&mut self, head: Head, arguments: &[TermId]) -> Result<TermId, Error> {
        if let Some(id) = self.find(head, arguments) {
            return Ok(id);
        }
        // Both counts stay below `u32::MAX`, so no index wraps.
        let full = |count: usize| {
            Error::Refused(Refusal::Index {
                what: Space::Term,
                count: count as u64,
                most: Self::MOST as u64,
            })
        };
        if self.entries.len() >= Self::MOST {
            return Err(full(self.entries.len() + 1));
        }
        let end = self.arguments.len() + arguments.len();
        if end > Self::MOST {
            return Err(full(end));
        }
        let id = self.entries.len() as u32;
        self.entries.push(Entry {
            head,
            start: self.arguments.len() as u32,
            len: arguments.len() as u32,
        });
        self.arguments.extend_from_slice(arguments);
        let before = self.latest.insert(Self::hash(head, arguments), id);
        self.earlier.push(before.unwrap_or(NONE));
        Ok(TermId(id))
    }
}
