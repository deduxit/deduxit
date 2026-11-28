// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The formula arena: the nodes of ordinary formulas, hash-consed so that
//! equal formulas are one id, their first-order tables, and the sequents
//! over them.

use crate::Error;
use crate::arena::Arena;
use crate::first_order::{Args, FirstOrder, FoTerm, FoTermId, Interner};
use crate::hash::HashMap;
use crate::linear::sequents::notation::TEXT;
use crate::linear::sequents::{Stop, Stops};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult, Write};

/// What an arm of a propositional method says of a binder: the method
/// refused the first-order sequent at its door
/// ([`Formulas::propositional`]), so none reaches it.
pub(crate) const REFUSED: &str = "a propositional method refuses a first-order sequent at its door";

/// The index of a formula in a [`Formulas`] arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub(super) u32);

impl NodeId {
    /// Returns the index as a `usize`, for indexing the arena.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// A node of an ordinary formula, which refers to its subformulas by
/// arena index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    /// An atom, by its index among the arena's atom names.
    Atom(u32),
    /// `⊤`, true.
    True,
    /// `⊥`, false.
    False,
    /// `¬A`
    Not(NodeId),
    /// `A ∧ B`
    And(NodeId, NodeId),
    /// `A ∨ B`
    Or(NodeId, NodeId),
    /// `A → B`
    Implies(NodeId, NodeId),
    /// `A ↔ B`
    Iff(NodeId, NodeId),
    /// `∀x. A`: the body `A`, whose bound variable `x` its atoms' terms
    /// name by the de Bruijn index of this binder.
    Forall(NodeId),
    /// `∃x. A`: the body `A`, as for [`Forall`](Self::Forall).
    Exists(NodeId),
}

/// A node stays a tag and at most two arena indices, since the arena and
/// the translation's tables grow with its size.
const _: () = assert!(size_of::<Node>() == 12);

impl Node {
    /// Returns the subformulas, in the order they are written.
    pub const fn operands(self) -> (Option<NodeId>, Option<NodeId>) {
        match self {
            Self::Atom(_) | Self::True | Self::False => (None, None),
            Self::Not(a) | Self::Forall(a) | Self::Exists(a) => (Some(a), None),
            Self::And(a, b) | Self::Or(a, b) | Self::Implies(a, b) | Self::Iff(a, b) => {
                (Some(a), Some(b))
            }
        }
    }
}

/// The formulas of ordinary sequents, as an arena in which every
/// subformula precedes the formulas built from it and equal formulas are
/// one node, so that two formulas are equal exactly when their ids are.
///
/// A bound variable is the de Bruijn index of its binder, so two formulas
/// that differ only in the names of their bound variables are one node.
/// A binder keeps the name it was first written with, for printing.
#[derive(Clone, Default)]
pub struct Formulas {
    /// The nodes and the atom names. The name of an atom with arguments
    /// is its canonical text, `p(f(a), #0)`.
    pub(super) arena: Arena<Node>,
    /// The first-order tables and their lookup maps: the symbols, the
    /// terms, each atom's predicate and arguments, and every binder's
    /// name. Made at the first binder, predicate with arguments or term;
    /// empty exactly when the arena has no binder and no predicate with
    /// arguments, and then every atom's entry is missing too. A
    /// propositional arena pays a pointer for it.
    pub(super) fo: Option<Box<Interner<NodeId>>>,
}

/// An arena read from first-order tables, the index in it of every atom
/// of the tables and the id in it of every term of the tables.
#[cfg(feature = "serialize")]
pub(crate) type OfTables = (Formulas, Vec<u32>, Vec<FoTermId>);

/// The tables of an arena that has none.
static NO_TABLES: FirstOrder<NodeId> = FirstOrder::new();

impl Debug for Formulas {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut out = f.debug_struct("Formulas");
        self.arena.fields(&mut out);
        out.field("fo", &self.fo).finish()
    }
}

impl crate::arena::Id for NodeId {
    fn at(index: u32) -> Self {
        Self(index)
    }

    fn place(self) -> usize {
        self.index()
    }
}

impl crate::arena::Shape for Node {
    type Id = NodeId;

    fn operands(self) -> (Option<NodeId>, Option<NodeId>) {
        Self::operands(self)
    }

    fn atom(self) -> Option<u32> {
        if let Self::Atom(atom) = self {
            Some(atom)
        } else {
            None
        }
    }
}

impl Formulas {
    /// Returns the id of a node the arena holds, if it holds it.
    pub(crate) fn find(&self, node: Node) -> Option<NodeId> {
        self.arena.find(node)
    }

    /// The most nodes an arena holds: the ids are `u32`.
    pub const MOST: usize = Arena::<Node>::MOST;

    /// Returns the node with the id given, which must belong to this arena.
    ///
    /// # Panics
    ///
    /// If the id is past the arena's end, as an id of another arena may
    /// be.
    pub fn node(&self, id: NodeId) -> Node {
        self.arena.node(id)
    }

    /// Returns the nodes, each after its subformulas.
    pub(crate) fn nodes(&self) -> &[Node] {
        self.arena.nodes()
    }

    /// Returns the number of nodes.
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    /// Returns whether the arena has no node.
    pub fn is_empty(&self) -> bool {
        self.arena.len() == 0
    }

    /// Returns the name of the atom with the index given.
    pub fn atom_name(&self, atom: u32) -> &str {
        self.arena.atom_name(atom)
    }

    /// Returns the atom names, in the order of their indices.
    pub fn atom_names(&self) -> &[String] {
        self.arena.atom_names()
    }

    /// Returns the id of `node`, adding it unless the arena has it.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for a subformula that is no node of
    /// this arena (an id of another one) or an atom that is not one of its
    /// names, and [`Refusal::Index`](crate::Refusal::Index) when the arena
    /// is full.
    pub fn add(&mut self, node: Node) -> Result<NodeId, Error> {
        if let Some(id) = self.arena.find(node) {
            return Ok(id);
        }
        self.arena.admit(node)?;
        if matches!(node, Node::Forall(_) | Node::Exists(_)) {
            self.enter_first_order()?;
        }
        let id = self.arena.push(node);
        if matches!(node, Node::Forall(_) | Node::Exists(_)) {
            // A binder made without a name prints its variable as `x`.
            self.interner().tables.binders.push((id, "x".to_owned()));
        }
        Ok(id)
    }

    /// Fails unless `id` is a node of this arena.
    pub(super) fn holds(&self, id: NodeId) -> Result<(), Error> {
        self.arena.holds(id)
    }

    /// Returns the atom named `name`, adding it unless the arena has it.
    /// The name is kept in Unicode normal form NFC, so one name is one atom
    /// however its accents are encoded. It must be an identifier of the
    /// text syntax and no keyword, so that a formula written as text reads
    /// back as itself: not `true` or `false`, and no keyword of the linear
    /// syntax either, since the translations keep the atom's name.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// the quantifiers' `forall` and `exists` among them, and
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub fn atom(&mut self, name: &str) -> Result<NodeId, Error> {
        let Self { arena, fo } = self;
        let index = arena.atom(name, |name| {
            if fo.as_deref().is_some_and(|fo| !fo.tables.is_empty()) {
                nullary(fo, name)
            } else {
                Ok(())
            }
        })?;
        self.add(Node::Atom(index))
    }

    /// Returns the arena of the first-order `tables`, read from a
    /// document and checked, without a node yet, the index in it of every
    /// atom of the tables (an atom equal to an earlier one is that one)
    /// and the id in it of every term of the tables. Every symbol, term and argument list is interned again, so the
    /// arena holds each term once however often the document lists it.
    /// `None` where the tables have no term and every atom is nullary:
    /// the document is a propositional sequent, its atoms by name.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a symbol named by a keyword of the ordinary
    /// syntax.
    #[cfg(feature = "serialize")]
    pub(crate) fn of_tables(tables: &FirstOrder<NodeId>) -> Result<Option<OfTables>, Error> {
        let propositional = tables.terms.is_empty()
            && tables
                .atoms
                .iter()
                .all(|&(_, args)| tables.list(args).is_empty());
        if propositional {
            return Ok(None);
        }
        let mut fo = Interner::new();
        let mut predicates = vec![false; tables.symbols.len()];
        for &(p, _) in &tables.atoms {
            predicates[p.index()] = true;
        }
        let mut symbols = Vec::with_capacity(tables.symbols.len());
        for ((name, arity), &predicate) in tables.symbols.iter().zip(&predicates) {
            // Equality is a binary predicate, which `FirstOrder::check`
            // has held it to; no identifier is its name.
            let equals = name == crate::first_order::EQUALS;
            if !equals {
                crate::name::check_ordinary(name)?;
            }
            // `=` is a predicate though no atom uses it: interned as a
            // function, it would refuse the atoms the axioms make.
            let symbol = fo
                .symbol(name, *arity, predicate || equals)
                .map_err(|fault| Error::Symbol {
                    name: name.clone(),
                    fault,
                })?;
            symbols.push(symbol);
        }
        let mut terms: Vec<crate::first_order::FoTermId> = Vec::with_capacity(tables.terms.len());
        for &term in &tables.terms {
            let id = match term {
                crate::first_order::FoTerm::Bound(_) => fo.term(term),
                crate::first_order::FoTerm::App(f, args) => {
                    let items: Vec<_> =
                        tables.list(args).iter().map(|t| terms[t.index()]).collect();
                    let args = fo.list(&items);
                    fo.term(crate::first_order::FoTerm::App(symbols[f.index()], args))
                }
                // A derivation's or a goal state's terms, which the reader
                // takes where the document is one.
                crate::first_order::FoTerm::Eigen(_) | crate::first_order::FoTerm::Meta(_) => {
                    fo.term(term)
                }
            };
            terms.push(id);
        }
        let mut formulas = Self {
            fo: Some(Box::new(fo)),
            ..Self::default()
        };
        let mut indices = Vec::with_capacity(tables.atoms.len());
        for &(p, args) in &tables.atoms {
            let items: Vec<_> = tables.list(args).iter().map(|t| terms[t.index()]).collect();
            let args = formulas.interner().list(&items);
            let symbol = symbols[p.index()];
            let key = formulas.tables().atom_key(symbol, args);
            let (index, new) = formulas.arena.keyed(key);
            if new {
                formulas.interner().tables.atoms.push((symbol, args));
            }
            indices.push(index);
        }
        Ok(Some((formulas, indices, terms)))
    }

    /// Returns whether the arena is first-order: it holds a binder or a
    /// predicate with arguments. The methods that decide propositional
    /// logic refuse such an arena ([`Error::FirstOrder`]).
    pub fn is_first_order(&self) -> bool {
        !self.tables().is_empty()
    }

    /// Fails with [`Error::FirstOrder`] naming `method` where the arena is
    /// first-order.
    pub(crate) fn propositional(&self, method: &'static str) -> Result<(), Error> {
        if self.is_first_order() {
            Err(Error::FirstOrder { method })
        } else {
            Ok(())
        }
    }

    /// Returns the first-order tables, empty where the arena has none.
    pub(crate) fn first_order(&self) -> &FirstOrder<NodeId> {
        self.tables()
    }

    /// Returns the first-order tables, empty where the arena has none.
    pub(super) fn tables(&self) -> &FirstOrder<NodeId> {
        self.fo.as_deref().map_or(&NO_TABLES, |fo| &fo.tables)
    }

    /// Returns the first-order tables and their maps, made empty where the
    /// arena has none yet.
    pub(super) fn interner(&mut self) -> &mut Interner<NodeId> {
        interner(&mut self.fo)
    }

    /// Returns whether a symbol of the arena is named `name`.
    pub(super) fn has_symbol(&self, name: &str) -> bool {
        self.fo
            .as_ref()
            .is_some_and(|fo| fo.find_symbol(name).is_some())
    }

    /// Gives every atom without one its nullary predicate, so that the
    /// tables have an entry per atom, as a first-order arena's must.
    ///
    /// # Errors
    ///
    /// [`Error::Symbol`] where an atom's name is a function symbol's.
    #[cold]
    #[inline(never)]
    pub(super) fn enter_first_order(&mut self) -> Result<(), Error> {
        let had = self.tables().atoms.len();
        for n in had..self.arena.atom_names().len() {
            let Self { arena, fo } = self;
            let result = nullary(fo, &arena.atom_names()[n]);
            if let Err(error) = result {
                // All or none, so that the entries stay parallel.
                self.interner().tables.atoms.truncate(had);
                return Err(error);
            }
        }
        Ok(())
    }

    /// Returns the term made by the function symbol `name` applied to
    /// `args`, interned: a constant where `args` is empty.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// and [`Error::Symbol`] where the name is a predicate or has another
    /// arity.
    #[cfg(any(test, feature = "parse"))]
    pub(crate) fn function(&mut self, name: &str, args: &[FoTermId]) -> Result<FoTermId, Error> {
        let name = &*crate::name::normalized(name);
        crate::name::check_ordinary(name)?;
        let symbol = self
            .interner()
            .symbol(name, args.len() as u32, false)
            .map_err(|fault| Error::Symbol {
                name: name.to_owned(),
                fault,
            })?;
        let args = self.interner().list(args);
        Ok(self.interner().term(FoTerm::App(symbol, args)))
    }

    /// Returns the bound variable of the binder `index` binders out, as a
    /// term: `0` is the innermost binder's variable.
    #[cfg(any(test, feature = "parse"))]
    pub(crate) fn variable(&mut self, index: u32) -> FoTermId {
        self.interner().term(FoTerm::Bound(index))
    }

    /// Returns the atom of the predicate `name` applied to `args`, adding
    /// it unless the arena has it. The parser calls it with arguments
    /// only; a nullary predicate is an atom ([`atom`](Self::atom)), and
    /// called with none it makes the arena first-order all the same.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// [`Error::Symbol`] where the name is a function symbol or has another
    /// arity, and [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    #[cfg(any(test, feature = "parse"))]
    pub(crate) fn predicate(&mut self, name: &str, args: &[FoTermId]) -> Result<NodeId, Error> {
        let name = &*crate::name::normalized(name);
        crate::name::check_ordinary(name)?;
        self.enter_first_order()?;
        let symbol = self
            .interner()
            .symbol(name, args.len() as u32, true)
            .map_err(|fault| Error::Symbol {
                name: name.to_owned(),
                fault,
            })?;
        let args = self.interner().list(args);
        self.predicate_atom(symbol, args)
    }

    /// Returns whether the arena uses equality, the predicate written `s =
    /// t`: a sequent over it is decided with the equality axioms
    /// ([`equality::with_axioms`](super::equality::with_axioms)).
    pub fn has_equality(&self) -> bool {
        self.fo
            .as_ref()
            .is_some_and(|fo| fo.find_symbol(crate::first_order::EQUALS).is_some())
    }

    /// Returns the atom `left = right`, adding it unless the arena has it,
    /// and making the arena first-order where it was not.
    ///
    /// # Errors
    ///
    /// [`Error::Symbol`] where an atom's name is a function symbol's, and
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn equality(&mut self, left: FoTermId, right: FoTermId) -> Result<NodeId, Error> {
        self.enter_first_order()?;
        let name = crate::first_order::EQUALS;
        let symbol = self
            .interner()
            .symbol(name, 2, true)
            .map_err(|fault| Error::Symbol {
                name: name.to_owned(),
                fault,
            })?;
        let args = self.interner().list(&[left, right]);
        self.predicate_atom(symbol, args)
    }

    /// Returns the atom of the predicate `symbol` of the tables applied to
    /// `args`, adding it unless the arena has it. The arena must be
    /// first-order, `args` as many as the symbol's arity.
    pub(crate) fn applied_atom(
        &mut self,
        symbol: crate::first_order::Symbol,
        args: &[FoTermId],
    ) -> Result<NodeId, Error> {
        let args = self.interner().list(args);
        self.predicate_atom(symbol, args)
    }

    /// Returns the atom of `symbol` applied to `args`, both of the tables,
    /// adding it unless the arena has it. The arena must be first-order.
    pub(super) fn predicate_atom(
        &mut self,
        symbol: crate::first_order::Symbol,
        args: Args,
    ) -> Result<NodeId, Error> {
        let key = self.tables().atom_key(symbol, args);
        // By `entry`, so that `atom`'s lookup and insertion keep the
        // parsers as their one caller and inline there.
        let (index, new) = self.arena.keyed(key);
        if new {
            self.interner().tables.atoms.push((symbol, args));
        }
        self.add(Node::Atom(index))
    }

    /// Returns `∀x. body` (`forall`) or `∃x. body`, adding it unless the
    /// arena has it. A binder added here keeps `name` for printing; one
    /// the arena had keeps the name it was first given.
    ///
    /// # Errors
    ///
    /// [`Error::AtomName`] for a name that is no identifier or a keyword,
    /// [`Error::IndexOutOfBounds`] for a body of another arena, and
    /// [`Refusal::Index`](crate::Refusal::Index) when the arena is full.
    pub(crate) fn binder(
        &mut self,
        forall: bool,
        body: NodeId,
        name: &str,
    ) -> Result<NodeId, Error> {
        let name = crate::name::normalized(name);
        crate::name::check_ordinary(&name)?;
        let len = self.len();
        let id = self.add(if forall {
            Node::Forall(body)
        } else {
            Node::Exists(body)
        })?;
        if self.len() > len
            && let Some(entry) = self.interner().tables.binders.last_mut()
        {
            // The node is new, so its entry is the last.
            entry.1 = name.into_owned();
        }
        Ok(id)
    }

    /// Returns per node how many binders it needs above it: one more than
    /// the largest index of a bound variable that no binder inside it
    /// binds, `0` for a closed formula.
    fn loose(&self) -> Vec<u32> {
        let terms = self.tables().loose();
        let mut loose: Vec<u32> = Vec::with_capacity(self.len());
        for &node in self.nodes() {
            let of = |k: NodeId| loose[k.index()];
            loose.push(match node {
                Node::Atom(a) => match self.tables().atoms.get(a as usize) {
                    Some(&(_, args)) => self
                        .tables()
                        .list(args)
                        .iter()
                        .map(|t| terms[t.index()])
                        .max()
                        .unwrap_or(0),
                    None => 0,
                },
                Node::True | Node::False => 0,
                Node::Not(a) => of(a),
                Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                    of(a).max(of(b))
                }
                Node::Forall(a) | Node::Exists(a) => of(a).saturating_sub(1),
            });
        }
        loose
    }

    /// Returns the formula at `id` as a value that prints it.
    pub fn formula(&self, id: NodeId) -> Formula<'_> {
        Formula { formulas: self, id }
    }

    /// Writes the formula at `id` with the spellings given, every binary
    /// subformula in brackets, and the whole in brackets too if it is
    /// binary and `brackets` is set. `atom` writes an atom of the arena,
    /// `name` a symbol's or a bound variable's name, each in the target's
    /// markup.
    pub(crate) fn write<W: Write>(
        &self,
        out: &mut W,
        id: NodeId,
        brackets: bool,
        symbols: &Symbols,
        atom: impl Fn(&mut W, u32) -> FmtResult,
        name: impl Fn(&mut W, &str) -> FmtResult,
    ) -> FmtResult {
        if self.is_first_order() {
            return self.write_first_order(out, id, brackets, symbols, &atom, &name);
        }
        for visit in Stops::new(id, brackets, |k| self.node(k).operands()) {
            match visit {
                Stop::Enter(k, nested) => match self.node(k) {
                    Node::Atom(a) => atom(out, a)?,
                    Node::True => out.write_str(symbols.truth)?,
                    Node::False => out.write_str(symbols.falsity)?,
                    Node::Not(_) => out.write_str(symbols.not)?,
                    Node::Forall(_) | Node::Exists(_) => {
                        unreachable!("an arena with a binder is first-order")
                    }
                    _ if nested => out.write_char('(')?,
                    Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {}
                },
                Stop::Between(k) => self.between(out, k, symbols)?,
                Stop::Exit(k, nested) => {
                    if nested && self.node(k).operands().1.is_some() {
                        out.write_char(')')?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Writes the connective of the binary formula `k` with a space on
    /// either side.
    fn between<W: Write>(&self, out: &mut W, k: NodeId, symbols: &Symbols) -> FmtResult {
        out.write_char(' ')?;
        out.write_str(match self.node(k) {
            Node::And(..) => symbols.and,
            Node::Or(..) => symbols.or,
            Node::Implies(..) => symbols.implies,
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::Iff(..)
            | Node::Forall(_)
            | Node::Exists(_) => symbols.iff,
        })?;
        out.write_char(' ')
    }

    /// Writes the formula at `id` of a first-order arena as
    /// [`write`](Self::write) does. A binder's variable is written by its
    /// recorded name, or by that name with `_` and a number after it where
    /// a symbol or an enclosing binder has the name, so the text reads
    /// back as the formula. A binder inside another formula is in
    /// brackets, but the body of a binder, since a binder reaches as far
    /// right as it can.
    #[cold]
    #[inline(never)]
    fn write_first_order<W: Write>(
        &self,
        out: &mut W,
        id: NodeId,
        brackets: bool,
        symbols: &Symbols,
        atom: &impl Fn(&mut W, u32) -> FmtResult,
        name: &impl Fn(&mut W, &str) -> FmtResult,
    ) -> FmtResult {
        let tables = self.tables();
        let eigens = self.eigen_names();
        // A lookup per candidate, never a set of every symbol: the
        // printer runs once per formula, and a sequent's formulas share
        // one arena.
        let taken = |candidate: &str| self.has_symbol(candidate) || self.is_eigen_name(candidate);
        // The names of the open binders, the innermost last, and how many
        // open binders have each, so that a clash costs one lookup.
        let mut scope: Vec<String> = Vec::new();
        let mut open_names: HashMap<String, u32> = HashMap::default();
        // Per recorded name, the suffix to try first (0 the name itself):
        // every smaller one is taken. A binder's exit lowers it again, so
        // the name written is the first free one, found in constant time
        // where binders of one name nest deep.
        let mut next: HashMap<&str, u32> = HashMap::default();
        // Per open binder, its recorded name and the suffix written.
        let mut suffixes: Vec<(&str, u32)> = Vec::new();
        // Per compound formula entered, whether a bracket was opened.
        let mut opened: Vec<bool> = Vec::new();
        // Whether the next formula entered is a binder's body.
        let mut body = false;
        for visit in Stops::new(id, brackets, |k| self.node(k).operands()) {
            match visit {
                Stop::Enter(k, nested) => {
                    let in_body = std::mem::take(&mut body);
                    match self.node(k) {
                        Node::Atom(a) => self.write_atom(out, a, &scope, eigens, atom, name)?,
                        Node::True => out.write_str(symbols.truth)?,
                        Node::False => out.write_str(symbols.falsity)?,
                        Node::Not(_) => {
                            out.write_str(symbols.not)?;
                            opened.push(false);
                        }
                        Node::And(..) | Node::Or(..) | Node::Implies(..) | Node::Iff(..) => {
                            if nested {
                                out.write_char('(')?;
                            }
                            opened.push(nested);
                        }
                        node @ (Node::Forall(_) | Node::Exists(_)) => {
                            let open = nested && !in_body;
                            if open {
                                out.write_char('(')?;
                            }
                            opened.push(open);
                            out.write_str(if matches!(node, Node::Forall(_)) {
                                symbols.forall
                            } else {
                                symbols.exists
                            })?;
                            let recorded = tables.binder_name(k).unwrap_or("x");
                            let clashes = |candidate: &str| {
                                taken(candidate) || open_names.contains_key(candidate)
                            };
                            let candidate = |n: u32| {
                                if n == 0 {
                                    recorded.to_owned()
                                } else {
                                    format!("{recorded}_{n}")
                                }
                            };
                            let mut n = next.get(recorded).copied().unwrap_or(0);
                            let mut written = candidate(n);
                            while clashes(&written) {
                                n += 1;
                                written = candidate(n);
                            }
                            next.insert(recorded, n + 1);
                            suffixes.push((recorded, n));
                            name(out, &written)?;
                            out.write_str(symbols.dot)?;
                            *open_names.entry(written.clone()).or_insert(0) += 1;
                            scope.push(written);
                            body = true;
                        }
                    }
                }
                Stop::Between(k) => self.between(out, k, symbols)?,
                Stop::Exit(k, _) => {
                    if matches!(self.node(k), Node::Forall(_) | Node::Exists(_))
                        && let Some(name) = scope.pop()
                        && let Some((recorded, n)) = suffixes.pop()
                        && let Some(count) = open_names.get_mut(&name)
                    {
                        let first = next.entry(recorded).or_insert(n);
                        *first = (*first).min(n);
                        *count -= 1;
                        if *count == 0 {
                            open_names.remove(&name);
                        }
                    }
                    if opened.pop() == Some(true) {
                        out.write_char(')')?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Writes the atom `a` of a first-order arena: by `atom` where it has
    /// no arguments, else as its predicate applied to its terms, a bound
    /// variable by the name `scope` gives its binder, an eigenvariable by
    /// its name in `eigens` and an open witness as `$1`, `$2`, ….
    fn write_atom<W: Write>(
        &self,
        out: &mut W,
        a: u32,
        scope: &[String],
        eigens: &[(u32, String)],
        atom: &impl Fn(&mut W, u32) -> FmtResult,
        name: &impl Fn(&mut W, &str) -> FmtResult,
    ) -> FmtResult {
        let tables = self.tables();
        let Some(&(symbol, args)) = tables.atoms.get(a as usize) else {
            return atom(out, a);
        };
        let list = tables.list(args);
        if list.is_empty() {
            return atom(out, a);
        }
        let term = |out: &mut W, t: FoTermId| {
            tables.write_with(
                out,
                t,
                &mut |o, s| name(o, s),
                &mut |o, index| match scope.len().checked_sub(index as usize + 1) {
                    Some(at) => name(o, &scope[at]),
                    None => write!(o, "#{index}"),
                },
                &mut |o, other| match other {
                    FoTerm::Eigen(e) => {
                        let at = eigens
                            .binary_search_by_key(&e.get(), |&(k, _)| k)
                            .expect("every eigenvariable of the arena has its name");
                        name(o, &eigens[at].1)
                    }
                    FoTerm::Meta(k) => {
                        name(o, &crate::linear::occurrences::instances::meta_name(k))
                    }
                    FoTerm::Bound(_) | FoTerm::App(..) => Ok(()),
                },
            )
        };
        let predicate = tables.symbols[symbol.index()].0.as_str();
        if let [left, right] = *list
            && predicate == crate::first_order::EQUALS
        {
            term(out, left)?;
            out.write_str(" = ")?;
            return term(out, right);
        }
        name(out, predicate)?;
        out.write_char('(')?;
        for (i, &t) in list.iter().enumerate() {
            if i > 0 {
                out.write_str(", ")?;
            }
            term(out, t)?;
        }
        out.write_char(')')
    }
}

/// Returns the first-order tables and their maps of an arena, made empty
/// where it has none yet.
fn interner(fo: &mut Option<Box<Interner<NodeId>>>) -> &mut Interner<NodeId> {
    let fo = fo.get_or_insert_with(Box::default);
    // The names printed follow the tables, which the caller may change.
    fo.eigens.take();
    fo
}

/// Adds the nullary predicate `name` to an arena's tables, as the entry of
/// the next atom.
#[cold]
#[inline(never)]
fn nullary(fo: &mut Option<Box<Interner<NodeId>>>, name: &str) -> Result<(), Error> {
    let symbol = interner(fo)
        .symbol(name, 0, true)
        .map_err(|fault| Error::Symbol {
            name: name.to_owned(),
            fault,
        })?;
    let args = interner(fo).list(&[]);
    interner(fo).tables.atoms.push((symbol, args));
    Ok(())
}

/// How a target spells the connectives and constants of ordinary
/// formulas.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Symbols {
    /// `¬`, written directly before its operand.
    pub(crate) not: &'static str,
    /// `∧`
    pub(crate) and: &'static str,
    /// `∨`
    pub(crate) or: &'static str,
    /// `→`
    pub(crate) implies: &'static str,
    /// `↔`
    pub(crate) iff: &'static str,
    /// `⊤`
    pub(crate) truth: &'static str,
    /// `⊥`
    pub(crate) falsity: &'static str,
    /// `∀`, with what stands between it and the variable's name.
    pub(crate) forall: &'static str,
    /// `∃`, with what stands between it and the variable's name.
    pub(crate) exists: &'static str,
    /// What stands between a binder's variable and its body.
    pub(crate) dot: &'static str,
}

impl Symbols {
    /// The Unicode symbols, which `Display` writes.
    pub(crate) const UNICODE: Self = Self {
        not: "¬",
        and: "∧",
        or: "∨",
        implies: "→",
        iff: "↔",
        truth: "⊤",
        falsity: "⊥",
        forall: "∀",
        exists: "∃",
        dot: ". ",
    };
}

/// A formula of an arena, as a value that prints it with the Unicode
/// symbols, every binary subformula in brackets.
#[derive(Clone, Copy, Debug)]
pub struct Formula<'a> {
    /// The arena.
    formulas: &'a Formulas,
    /// The formula.
    id: NodeId,
}

impl Display for Formula<'_> {
    /// Writes the formula: `(a ∧ b) → ¬c`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.formulas.write(
            f,
            self.id,
            false,
            &Symbols::UNICODE,
            |o, a| o.write_str(self.formulas.atom_name(a)),
            |o, name| o.write_str(name),
        )
    }
}

/// A sequent of ordinary propositional logic, `Γ ⊢ Δ`: the hypotheses `Γ`
/// left of the turnstile, and the formulas `Δ` right of it. Classically
/// `Δ` reads as the disjunction of its formulas. In intuitionistic and
/// minimal logic `Δ` holds one goal at most, and an empty `Δ` means
/// false.
///
/// # Syntax
///
/// With the cargo feature `parse`, `"…".parse::<ordinary::Sequent>()`
/// reads a sequent written `formulas |- formulas` (or `⊢`), each side a
/// list separated by commas and possibly empty; a text without a turnstile
/// is one formula to prove. From the loosest binding to the tightest, a
/// formula is built with `<->`/`↔` (to the right), `->`/`→` (to the
/// right), `\/`/`∨`, `/\`/`∧` (these to the left) and the prefix `~`/`¬`,
/// over the constants `true`/`⊤` and `false`/`⊥`, atoms and parentheses.
/// An atom is a Unicode identifier other than a keyword: `true`, `false`,
/// the linear syntax's `par`, `top` and `bot`, and `forall` and `exists`.
/// The symbols of linear logic (`&`, `|`, `*`, `-o`, …) are not read
/// here, so that a linear connective is never taken for an ordinary one.
///
/// A first-order formula has predicates and quantifiers:
/// `forall x. p(x) -> q(x), p(a) |- q(a)`. An atom may be a predicate
/// applied to terms, `p(x, f(a))`, with no space before the bracket; a
/// term is a variable, a constant or a function symbol applied to terms.
/// `forall x y. A` (or `∀x y. A`) and `exists x. A` (or `∃x. A`) bind
/// loosest and reach as far right as they can, so `forall x. p(x) ->
/// q(x)` is `∀x. (p(x) → q(x))`. A name that a quantifier around it binds
/// is that variable; any other name in a term is a constant or a function
/// symbol. A name has one arity and is a predicate or a function symbol,
/// not both. Every formula is closed. `s = t` says two terms are equal,
/// and `s != t` (or `s ≠ t`) is `¬(s = t)`; a sequent that uses `=` is
/// decided with the equality axioms ([`equality::with_axioms`](super::equality::with_axioms)).
///
/// `Display` writes the Unicode symbols with every nested binary formula
/// in brackets, which this syntax reads back.
///
/// # JSON
///
/// With the cargo feature `serialize` a sequent is `{"version": 8,
/// "formulas": [{"V": 0}, {"V": 1}, {"→": [0, 1]}], "atoms": ["a", "b"],
/// "left": [0], "right": [1]}` for `a ⊢ b` with the arena `a, b, a → b`
/// ([`wire`](crate::wire)): the formulas in the arena's order, every
/// operand before the formula built from it, as `{"V": atom}`, `"⊤"`,
/// `"⊥"`, `{"¬": a}`, `{"∧": [a, b]}`, `{"∨": [a, b]}`, `{"→": [a, b]}`
/// and `{"↔": [a, b]}`, and the two sides as indices into them.
///
/// A first-order sequent is written at level 25 with the keys of a
/// first-order linear sequent ([`linear::Sequent`](crate::linear::Sequent)):
/// `symbols`, `fo` (the terms), the atoms as `{"P": [symbol, [terms]]}`,
/// and `binders`, the name of each binder's variable in the order of the
/// binders among the formulas, which are `{"∀": a}` and `{"∃": a}`.
///
#[cfg_attr(feature = "parse", doc = "```")]
#[cfg_attr(not(feature = "parse"), doc = "```ignore")]
/// let sequent: deduxit::ordinary::Sequent = "a /\\ b -> c |- ~c -> ~a \\/ ~b".parse()?;
/// assert_eq!(sequent.to_string(), "(a ∧ b) → c ⊢ ¬c → (¬a ∨ ¬b)");
/// # Ok::<(), deduxit::Error>(())
/// ```
#[derive(Clone, Debug)]
pub struct Sequent {
    /// The formulas.
    pub(super) formulas: Formulas,
    /// The hypotheses, in the order written.
    pub(super) left: Vec<NodeId>,
    /// The formulas right of the turnstile, in the order written.
    pub(super) right: Vec<NodeId>,
}

impl Sequent {
    /// Returns the sequent `left ⊢ right` over the formulas given.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] for an id of the two lists that is no
    /// node of `formulas`, and [`Error::LooseVariable`] for a formula with
    /// a bound variable that no binder of it binds.
    pub fn new(formulas: Formulas, left: Vec<NodeId>, right: Vec<NodeId>) -> Result<Self, Error> {
        for &id in left.iter().chain(&right) {
            formulas.holds(id)?;
        }
        if formulas.is_first_order() {
            closed(&formulas, left.iter().chain(&right))?;
        }
        Ok(Self {
            formulas,
            left,
            right,
        })
    }

    /// Returns the arena of the formulas.
    pub fn formulas(&self) -> &Formulas {
        &self.formulas
    }

    /// Returns the formulas left of the turnstile, in the order written.
    pub fn left(&self) -> &[NodeId] {
        &self.left
    }

    /// Returns the formulas right of the turnstile, in the order written.
    pub fn right(&self) -> &[NodeId] {
        &self.right
    }
}

impl Display for Sequent {
    /// Writes `Γ ⊢ Δ` with the Unicode symbols.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let sides = (&self.left[..], &self.right[..]);
        TEXT.ordinary(f, &self.formulas, sides, false, false)
    }
}

/// Fails with [`Error::LooseVariable`] at the first of `roots` that has a
/// bound variable no binder of it binds.
#[cold]
#[inline(never)]
pub(crate) fn closed<'a>(
    formulas: &Formulas,
    roots: impl Iterator<Item = &'a NodeId>,
) -> Result<(), Error> {
    let loose = formulas.loose();
    for id in roots {
        if loose[id.index()] > 0 {
            return Err(Error::LooseVariable { term: id.index() });
        }
    }
    Ok(())
}

/// Checks that a sequent over the formulas unfolds to at most
/// `limits.occurrences` subformula occurrences.
pub(crate) fn bound(
    formulas: &Formulas,
    left: &[NodeId],
    right: &[NodeId],
    limits: &crate::Limits,
) -> Result<(), Error> {
    formulas.arena.bound(left, right, limits)
}
