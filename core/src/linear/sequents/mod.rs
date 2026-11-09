// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

// A new variant must get its own arm in every match, not fall into an
// existing one.
#![deny(clippy::wildcard_enum_match_arm)]

/// Printing sequents and formulas in one-sided notation.
pub mod fmt;
/// The first-order terms, symbols and binders of a sequent.
pub mod fo;
/// Atom names: identifiers of the text syntax that no keyword takes.
pub(crate) mod name;
/// The printer of formulas and sequents, with a table of spellings for
/// each output format.
pub(crate) mod notation;
/// The terms an arena is built from.
pub mod term;

pub use fmt::Formula;
pub(crate) use fmt::{Stop, Stops};
pub use fmt::{Visit, Walk};
pub use fo::{Args, Eigen, FoTerm, FoTermId, Symbol, SymbolFault};
pub use term::{Atom, Kind, Term, TermId};

use crate::hash::HashMap;

/// A sequent of linear logic: its formulas, kept one-sided and in
/// negation normal form, over an arena of shared subterms.
///
/// Read a sequent from text with `parse` (feature `parse`) or from JSON
/// (feature `serialize`). `Display` writes it one-sided: `A |- A` prints
/// as `⊢ ~A, A`.
///
/// # Text syntax
///
/// A sequent is written `formulas ⊢ formulas`, or with `|-` for `⊢`.
/// Each side is a list of formulas separated by commas, and may be
/// empty. Whitespace may stand between any two tokens. Text that is no
/// sequent gives `Error::Parse`, whose `ParseError` says where it went
/// wrong.
///
/// The connectives, from the loosest to the tightest:
///
/// | connective | ASCII | Unicode | groups |
/// |---|---|---|---|
/// | linear implication | `-o` | `⊸` | to the right |
/// | plus | `+` | `⊕` | to the left |
/// | with | `&` | `&` | to the left |
/// | par | `\|` or `par` | `⅋` | to the left |
/// | tensor | `*` | `⊗` | to the left |
/// | negation, of course, why not | `~A`, `!A`, `?A` | | prefix |
/// | negation | `A^` | | postfix |
///
/// The constants are `0`, `1`, `bot` (`⊥`) and `top` (`⊤`). Parentheses
/// group.
///
/// An atom is named by a Unicode identifier: `_` or a character of
/// `XID_Start`, then characters of `XID_Continue`.
/// - A name is read in NFC, so one name is one atom however its accents
///   are encoded.
/// - `bot`, `top` and `par` are keywords: as whole identifiers they are
///   that constant or connective. `forall` and `exists` are the
///   quantifiers' keywords. No keyword names an atom.
/// - Every atom name of a sequent is such an identifier, however the
///   sequent was read, so a sequent written as text reads back as itself.
///
/// [`parse_ordered`](Self::parse_ordered) also reads the two divisions of
/// the Lambek calculus, `A \ B` (groups to the right) and `B / A`
/// (groups to the left). They bind as loosely as `-o`. Mixed, a division
/// takes everything to its right: `A \ B / C` is `A \ (B / C)`, and
/// `A / B \ C` is `A / (B \ C)`.
///
/// # First-order syntax
///
/// - `forall x. A` (`∀x. A`) and `exists x. A` (`∃x. A`) bind loosest.
///   Their scope runs right as far as it can: to a `)`, a `,`, the
///   turnstile or the end. `forall x y. A` binds both variables.
/// - An atom may be a predicate applied to terms: `p(x, f(a))`. In a
///   term, an identifier that an enclosing quantifier binds is that
///   variable. Any other identifier is a constant, or a function symbol
///   when a `(` follows it.
/// - A symbol has one arity in the whole sequent. A name is a predicate
///   or a function symbol, never both. No keyword names a symbol or a
///   variable.
/// - Every formula is closed: each variable is bound by a quantifier.
///
/// How the sequent stores them:
/// - A quantifier's variable is bound by index in the arena
///   ([`Term::Forall`]); its name is kept for printing
///   ([`binder_name`](Self::binder_name)). Two quantifiers that differ
///   only in their variable's name are one term.
/// - An atom with arguments is named by its text, `p(f(a), b)`. An atom
///   under a quantifier is named by its text with the bound variables as
///   indices, `p(#0)` ([`atom_name`](Self::atom_name)).
/// - The symbols and the terms are the sequent's tables
///   ([`symbols`](Self::symbols), [`fo_terms`](Self::fo_terms),
///   [`atom_predicate`](Self::atom_predicate)). A propositional sequent
///   has them empty.
///
/// # The one-sided form
///
/// - The formulas left of the turnstile are negated.
/// - `A ⊸ B` is `~A ⅋ B`.
/// - Negations are pushed down to the atoms.
///
/// So `A |- A` is `⊢ ~A, A`, which is what `Display` writes.
///
/// The formulas keep the order they were written in. The sequent also
/// remembers how many stood left of the turnstile
/// ([`antecedents`](Self::antecedents)). From that,
/// [`Reading`](crate::linear::Reading) reads an intuitionistic sequent
/// back two-sided. `Display` leaves the sides out, so its text reads back
/// as `⊢ Γ`. The reading's text and the JSON keep the sides.
///
/// # JSON
///
/// With the feature `serialize`, a sequent is the object
/// `{"version": 1, "terms": […], "roots": […], "atoms": […], "antecedents": k}`.
/// The `version` stands only at the top of a document
/// ([`wire`](crate::wire)). The command's `seq json` writes this form.
///
/// | key | holds |
/// |---|---|
/// | `terms` | the arena, each term naming only terms before it |
/// | `roots` | the root formulas, as indices into `terms`, in the order written |
/// | `atoms` | the atom names |
/// | `antecedents` | how many of the roots, the first ones, stand left of `⊢`; absent when the sides are not known |
/// | `cuts` | the cut formulas ([`cuts`](Self::cuts)), only when there are some |
/// | `symbols`, `fo`, `binders` | the first-order tables, only for a first-order sequent |
///
/// `antecedents` is written whenever the sides are known, `0` included.
///
/// A term of `terms` is one of these, `k` and `l` indices into `terms`
/// and `a` an index into `atoms`:
///
/// | term | JSON |
/// |---|---|
/// | a unit | its symbol: `"1"`, `"⊥"`, `"⊤"` or `"0"` |
/// | an atom, its dual | `{"V": a}`, `{"D": a}` |
/// | a binary connective | `{"⊗": [k, l]}`, and `⅋`, `&`, `⊕` alike |
/// | `!A`, `?A` | `{"!": k}`, `{"?": k}` |
/// | `∀x. A`, `∃x. A` | `{"∀": k}`, `{"∃": k}`, `k` the body |
///
/// Reading checks that:
/// - every term names only terms before it;
/// - `antecedents` is at most the number of roots;
/// - every atom name is an identifier of the text syntax
///   (`Error::AtomName` otherwise).
///
/// A name that the table repeats is one atom. Reading counts the
/// occurrences against the limits before anything unfolds.
///
/// `cuts` holds the term of each cut formula `A`; the reader builds its
/// dual. A document that holds a cut formula is at wire level 6.
///
/// A first-order sequent is at wire level 7 and writes:
/// - `"symbols": [["p", 1], …]`, each symbol's name and arity;
/// - `"fo": […]`, its terms, each before the terms that name it:
///   `{"B": n}` is a bound variable, `{"F": [symbol, [terms]]}` an
///   application;
/// - every atom as `{"P": [symbol, [terms]]}`;
/// - `"binders": ["x", …]`, the quantifiers' variable names in the order
///   of their terms.
///
/// A propositional sequent writes none of these keys.
///
/// # Examples
///
#[cfg_attr(all(feature = "parse", feature = "serialize"), doc = "```")]
#[cfg_attr(not(all(feature = "parse", feature = "serialize")), doc = "```ignore")]
/// use deduxit::linear::Sequent;
///
/// let sequent: Sequent = "A, A -o B |- B".parse()?;
/// assert_eq!(sequent.to_string(), "⊢ ~A, A ⊗ ~B, B");
/// assert_eq!(sequent.antecedents(), Some(2));
///
/// let sequent: Sequent = "A |- A".parse()?;
/// assert_eq!(
///     serde_json::to_string(&sequent)?,
///     r#"{"version":1,"terms":[{"D":0},{"V":0}],"roots":[0,1],"atoms":["A"],"antecedents":1}"#
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sequent {
    /// Every subformula; a term refers only to terms at lower indices.
    pub(crate) terms: Vec<Term>,
    /// The root formulas, as arena indices, in the order they were
    /// written. Nothing sorts them.
    pub(crate) roots: Vec<TermId>,
    /// The atom names that `Atom` and `DualAtom` refer to by index.
    pub(crate) atoms: Vec<String>,
    /// How many of the roots, the first ones, were written left of `⊢`,
    /// or `None` where no sides were given. At most the number of roots.
    pub(crate) antecedents: Option<u32>,
    /// The cut formulas, in the order they were added. Each is the term of
    /// a formula `A` and the term of its dual `A⊥`, which has the same
    /// shape. The forest numbers them after the roots, `A` before `A⊥`.
    pub(crate) cuts: Vec<[TermId; 2]>,
    /// The first-order tables: the symbols, the terms, each atom as a
    /// predicate over terms, and the binders' names. Empty exactly when
    /// every atom is nullary and there is no binder.
    pub(crate) fo: fo::FirstOrder,
}

impl std::default::Default for Sequent {
    /// Returns the empty sequent.
    fn default() -> Self {
        Self::new()
    }
}

impl Sequent {
    /// Returns the empty sequent.
    pub const fn new() -> Self {
        Self {
            terms: vec![],
            roots: vec![],
            atoms: vec![],
            antecedents: None,
            cuts: vec![],
            fo: fo::FirstOrder::new(),
        }
    }

    /// Returns the arena: every subformula, each referring only to terms at
    /// lower indices.
    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    /// Returns the term at `id`, which must belong to this sequent.
    ///
    /// # Panics
    ///
    /// If `id` is past the end of the arena, as an id of another sequent
    /// can be.
    pub fn term(&self, id: TermId) -> Term {
        self.terms[id.index()]
    }

    /// Returns the root formulas, in the order they were written.
    pub fn roots(&self) -> &[TermId] {
        &self.roots
    }

    /// Returns how many root formulas were written left of `⊢`.
    ///
    /// They are the first ones of [`roots`](Self::roots); the rest stood
    /// right of `⊢`. The result is `None` when the sides are not known: a
    /// JSON sequent without them, or the sum of two sequents
    /// ([`add`](Self::add)). Text always gives them, `Some(0)` for `⊢ Γ`.
    /// Only the intuitionistic reading uses them, to find the goal
    /// ([`Reading`](crate::linear::Reading)).
    pub fn antecedents(&self) -> Option<u32> {
        self.antecedents
    }

    /// Returns the cut formulas, in the order they were added: for each
    /// cut, the term of the formula `A` and the term of its dual `A⊥`.
    ///
    /// Cut formulas are not part of the conclusion (the
    /// [`roots`](Self::roots)). A proof with cuts
    /// ([`Node::Cut`](crate::linear::Node::Cut)) proves a sequent that
    /// holds its cut formulas, and that sequent's
    /// [`Forest`](crate::linear::Forest) numbers each pair after the roots.
    /// A sequent read from text holds none, and `Display` writes only the
    /// conclusion.
    pub fn cuts(&self) -> &[[TermId; 2]] {
        &self.cuts
    }

    /// Adds a cut formula `A`, given as `formula`, a sequent of one root
    /// formula.
    ///
    /// An atom of `formula` is this sequent's atom of the same name. `A`
    /// and its dual `A⊥` join the arena, and the pair is numbered after
    /// the roots and the earlier cut formulas. The roots and their sides
    /// do not change.
    ///
    /// # Errors
    ///
    /// [`Error::CutFormula`](crate::Error::CutFormula) when `formula`
    /// has more or fewer than one root formula, or cut formulas of its
    /// own.
    pub fn add_cut(&mut self, formula: Self) -> Result<(), crate::Error> {
        if formula.roots.len() != 1 || !formula.cuts.is_empty() {
            return Err(crate::Error::CutFormula {
                formulas: formula.roots.len(),
            });
        }
        let offset_atoms = self.atoms.len() as u32;
        let offset_terms = self.terms.len() as u32;
        self.terms.extend(
            formula
                .terms
                .into_iter()
                .map(|e| e.offset(offset_atoms, offset_terms)),
        );
        self.atoms.extend(formula.atoms);
        self.join_first_order(formula.fo, offset_terms);
        self.merge_atoms();
        self.push_cuts(&[TermId::new(formula.roots[0].get() + offset_terms)]);
        Ok(())
    }

    /// Adds the cut formulas whose terms are `cuts`, terms of the arena, in
    /// that order.
    ///
    /// Each one's dual joins the arena, with a term for each term below
    /// it, so the two have the same shape. Where the arena already has an
    /// equal term, the dual uses the first one, so a sequent written with
    /// its duals reads back as it was. Each term is dualised once, however
    /// many cut formulas reach it: the pass costs the arena plus the cut
    /// formulas, not their product. It does not recurse.
    pub(crate) fn push_cuts(&mut self, cuts: &[TermId]) {
        // A sequent without cut formulas pays nothing here.
        if cuts.is_empty() {
            return;
        }
        let mut known =
            HashMap::<Term, TermId>::with_capacity_and_hasher(self.terms.len(), Default::default());
        // Not `entry`: while `optimize_terms` is its only user, the
        // compiler inlines it into the parser's path.
        for (n, &term) in self.terms.iter().enumerate().rev() {
            known.insert(term, TermId::new(n as u32));
        }
        let mut dual = HashMap::<TermId, TermId>::default();
        // A term waits until its subterms have their duals; a subterm comes
        // before its parent, so no term waits on itself.
        let mut waiting: Vec<(TermId, bool)> = Vec::new();
        for &a in cuts {
            waiting.push((a, false));
            while let Some((t, ready)) = waiting.pop() {
                if dual.contains_key(&t) {
                    continue;
                }
                if !ready {
                    waiting.push((t, true));
                    let below = self.terms[t.index()].subterms();
                    waiting.extend(below.filter(|k| !dual.contains_key(k)).map(|k| (k, false)));
                    continue;
                }
                let term = self.terms[t.index()].dual().map_subterms(|k| dual[&k]);
                let id = match known.get(&term) {
                    Some(&id) => id,
                    None => {
                        let fresh = TermId::new(self.terms.len() as u32);
                        known.insert(term, fresh);
                        self.terms.push(term);
                        // A dual binder's variable has its binder's name.
                        // The fresh term is the last, so the names stay in
                        // ascending order of terms.
                        if let Some(name) = self.fo.binder_name(t) {
                            let name = name.to_owned();
                            self.fo.binders.push((fresh, name));
                        }
                        fresh
                    }
                };
                dual.insert(t, id);
            }
            self.cuts.push([a, dual[&a]]);
        }
    }

    /// Removes the last cut formula and the terms and atoms past those
    /// that the roots and the other cut formulas reach. For the cut
    /// formula added last, this gives back the arena as it was before.
    #[cfg(feature = "interactive")]
    pub(crate) fn pop_cut(&mut self) {
        self.pop_cut_keeping(0, false);
    }

    /// Removes the last cut formula as [`pop_cut`](Self::pop_cut) does,
    /// but keeps the first `terms` terms of the first-order arena and,
    /// with `symbols`, every symbol: what a session's table still names.
    #[cfg(feature = "interactive")]
    pub(crate) fn pop_cut_keeping(&mut self, terms: usize, symbols: bool) {
        self.cuts.pop();
        // A subterm precedes its parent, so the highest term any kept
        // formula reaches is a root or a cut formula itself.
        let end = self
            .roots
            .iter()
            .chain(self.cuts.iter().flatten())
            .map(|t| t.index() + 1)
            .max()
            .unwrap_or(0);
        self.terms.truncate(end);
        let atoms = self
            .terms
            .iter()
            .filter_map(|t| t.atom())
            .map(|a| a.index() + 1)
            .max()
            .unwrap_or(0);
        // An atom that no term names goes too. Only a sequent read from
        // JSON can have one.
        self.atoms.truncate(atoms);
        self.fo.atoms.truncate(atoms.min(self.fo.atoms.len()));
        self.fo.binders.retain(|&(t, _)| t.index() < end);
        if !self.fo.is_empty() {
            self.fo.trim(terms, symbols);
        }
    }

    /// Returns the sequent of an arena that a reader in the crate built.
    /// The caller keeps the invariants: every term's subterms come before
    /// it, every atom is named, the roots are terms of the arena, and
    /// `antecedents` is at most their number.
    #[cfg(feature = "parse")]
    pub(crate) fn from_parts(
        terms: Vec<Term>,
        roots: Vec<TermId>,
        atoms: Vec<String>,
        antecedents: Option<u32>,
    ) -> Self {
        let sequent = Self {
            terms,
            roots,
            atoms,
            antecedents,
            cuts: vec![],
            fo: fo::FirstOrder::new(),
        };
        debug_assert!(sequent.check().is_ok());
        sequent
    }

    /// Returns the atom names, in the order `Atom` indexes them.
    pub fn atom_names(&self) -> &[String] {
        &self.atoms
    }

    /// Returns the name of `atom`, which must belong to this sequent.
    ///
    /// # Panics
    ///
    /// If `atom` is past the end of the atom table, as an atom of another
    /// sequent can be.
    pub fn atom_name(&self, atom: Atom) -> &str {
        &self.atoms[atom.index()]
    }

    /// Returns the atom called `name`, if the sequent has one.
    pub fn atom(&self, name: &str) -> Option<Atom> {
        self.atoms
            .iter()
            .position(|n| n == name)
            .map(|i| Atom::new(i as u32))
    }

    /// Returns whether the sequent is first-order: an atom has arguments
    /// or a formula has a quantifier. A propositional sequent has empty
    /// first-order tables, and its JSON holds none of their keys.
    pub fn is_first_order(&self) -> bool {
        !self.fo.is_empty()
    }

    /// Returns the predicate symbol of `atom` and its argument list.
    ///
    /// Returns `None` in a propositional sequent, which has no symbol
    /// table: there an atom is just its name ([`atom_name`](Self::atom_name)).
    ///
    /// # Panics
    ///
    /// If `atom` is past the end of the table, as an atom of another
    /// sequent can be.
    pub fn atom_predicate(&self, atom: Atom) -> Option<(Symbol, Args)> {
        self.fo.atoms.get(atom.index()).copied()
    }

    /// Returns how many first-order terms the atoms' arguments unfold to
    /// when written out as trees, saturating at `u64::MAX`.
    ///
    /// A term that several atoms or terms share counts once for each
    /// place. This is what [`Limits::terms`](crate::Limits::terms) bounds.
    /// A propositional sequent gives `0`.
    pub fn fo_size(&self) -> u64 {
        if self.fo.atoms.is_empty() {
            return 0;
        }
        let unfolded = self.fo.unfolded();
        self.fo.atoms.iter().fold(0u64, |sum, &(_, args)| {
            self.fo
                .list(args)
                .iter()
                .fold(sum, |sum, t| sum.saturating_add(unfolded[t.index()]))
        })
    }

    /// Returns whether an argument of `atom` holds a bound variable, which
    /// only an atom under a quantifier can.
    ///
    /// A ground atom pairs with its dual as a propositional atom does. An
    /// open one is instantiated through the quantifiers above it.
    ///
    /// # Panics
    ///
    /// If `atom` is past the end of the table, as an atom of another
    /// sequent can be.
    pub fn atom_is_open(&self, atom: Atom) -> bool {
        self.atom_predicate(atom).is_some_and(|(_, args)| {
            let loose = self.fo.loose();
            self.fo.list(args).iter().any(|t| loose[t.index()] > 0)
        })
    }

    /// Returns the symbols, each a name with its arity, in the order
    /// `Symbol` indexes them. Empty in a propositional sequent.
    pub fn symbols(&self) -> &[(String, u32)] {
        &self.fo.symbols
    }

    /// Returns the first-order terms, in the order `FoTermId` indexes
    /// them, each referring only to terms before it.
    pub fn fo_terms(&self) -> &[FoTerm] {
        &self.fo.terms
    }

    /// Returns the terms of the argument list `args`, which must belong
    /// to this sequent.
    ///
    /// # Panics
    ///
    /// If `args` is past the end of the table, as a list of another
    /// sequent can be.
    pub fn arguments(&self, args: Args) -> &[FoTermId] {
        self.fo.list(args)
    }

    /// Returns the name of the variable that the quantifier `term` binds,
    /// for printing, or `None` where `term` is no quantifier.
    pub fn binder_name(&self, term: TermId) -> Option<&str> {
        self.fo.binder_name(term)
    }

    /// Returns, per arena term, the number of subformula occurrences of the
    /// formula it is the top of, itself included. A subterm counts once
    /// for every place it stands in. Saturates at `u64::MAX`.
    pub(crate) fn sizes(&self) -> Vec<u64> {
        // A subterm precedes its parent, so its size is there when the
        // parent's is added up.
        let mut sizes = vec![0u64; self.terms.len()];
        for (n, term) in self.terms.iter().enumerate() {
            sizes[n] = term
                .subterms()
                .fold(1u64, |sum, k| sum.saturating_add(sizes[k.index()]));
        }
        sizes
    }

    /// Returns the number of subformula occurrences of the sequent: the
    /// nodes of its formulas, cut formulas included, written out as trees.
    ///
    /// A term that several terms or formulas share counts once for each
    /// place. This is how many occurrences a
    /// [`Forest`](crate::linear::Forest) of the sequent has. It can be
    /// exponentially more than the arena has terms, and saturates at
    /// `u64::MAX`. It takes one pass over the arena.
    pub fn occurrences(&self) -> u64 {
        self.occurrences_of(&self.sizes())
    }

    /// Returns the number of occurrences of the sequent from the
    /// [`sizes`](Self::sizes) of its terms: the roots' sizes, then those
    /// of each cut formula and its dual, saturating.
    pub(crate) fn occurrences_of(&self, sizes: &[u64]) -> u64 {
        self.roots
            .iter()
            .chain(self.cuts.iter().flatten())
            .fold(0u64, |sum, r| sum.saturating_add(sizes[r.index()]))
    }

    /// Checks the invariants that every public way to a sequent keeps.
    ///
    /// Every term names only earlier terms and atoms of the table, every
    /// root and cut formula is a term, and `antecedents` is at most the
    /// number of roots. A first-order sequent also has valid tables
    /// ([`fo::FirstOrder::check`]), closed root and cut formulas, and a
    /// quantifier term for every recorded variable name.
    #[cfg(any(test, feature = "parse", feature = "serialize"))]
    pub(crate) fn check(&self) -> Result<(), crate::Error> {
        let num_atoms = self.atoms.len() as u32;
        let num_terms = self.terms.len() as u32;

        // Every term names only earlier terms and atoms of the table.
        self.terms
            .iter()
            .enumerate()
            .try_for_each(|(n, e)| e.check_bounds(num_atoms, n as u32))?;

        // Every root and cut formula is a term of the arena.
        self.roots
            .iter()
            .chain(self.cuts.iter().flatten())
            .try_for_each(|n| {
                if n.get() >= num_terms {
                    Err(crate::Error::IndexOutOfBounds {
                        space: crate::limits::Space::Term,
                        index: n.index(),
                        len: num_terms as usize,
                    })
                } else {
                    Ok(())
                }
            })?;

        match self.antecedents {
            Some(k) if k as usize > self.roots.len() => {
                return Err(crate::Error::Antecedents {
                    antecedents: k as usize,
                    roots: self.roots.len(),
                });
            }
            _ => {}
        }
        if self.fo.is_empty() {
            return Ok(());
        }
        self.fo.check(self.atoms.len())?;
        // Per term, how many binders it needs above it. A root or cut
        // formula is closed and needs none. A subterm precedes its parent,
        // so one ascending pass has every subterm's count ready.
        let fo_loose = self.fo.loose();
        let mut loose = vec![0u32; self.terms.len()];
        for (n, term) in self.terms.iter().enumerate() {
            loose[n] = match *term {
                Term::Atom(a) | Term::DualAtom(a) => {
                    self.fo.atoms.get(a.index()).map_or(0, |&(_, args)| {
                        self.fo
                            .list(args)
                            .iter()
                            .map(|t| fo_loose[t.index()])
                            .max()
                            .unwrap_or(0)
                    })
                }
                Term::Forall(k) | Term::Exists(k) => loose[k.index()].saturating_sub(1),
                Term::One
                | Term::Bot
                | Term::Top
                | Term::Zero
                | Term::Tensor(..)
                | Term::Par(..)
                | Term::With(..)
                | Term::Plus(..)
                | Term::Bang(_)
                | Term::Quest(_) => term.subterms().map(|k| loose[k.index()]).max().unwrap_or(0),
            };
        }
        for root in self.roots.iter().chain(self.cuts.iter().flatten()) {
            if loose[root.index()] > 0 {
                return Err(crate::Error::LooseVariable { term: root.index() });
            }
        }
        for &(t, _) in &self.fo.binders {
            if t.index() >= self.terms.len() || !self.terms[t.index()].kind().is_binder() {
                return Err(crate::Error::IndexOutOfBounds {
                    space: crate::limits::Space::Term,
                    index: t.index(),
                    len: self.terms.len(),
                });
            }
        }
        Ok(())
    }

    /// Drops the terms that no root or cut formula reaches and merges equal
    /// terms. The arena must be topologically sorted, and stays so.
    fn optimize_terms(&mut self) {
        let num_terms = self.terms.len();

        let mut reachable = vec![false; num_terms];
        for n in self.roots.iter().chain(self.cuts.iter().flatten()) {
            reachable[n.index()] = true;
        }

        // A subterm precedes its parent, so one pass from the top marks
        // every reachable term before the pass gets to it.
        for n in (0..num_terms).rev() {
            if !reachable[n] {
                continue;
            }
            for k in self.terms[n].subterms() {
                debug_assert!(k.index() < n, "a subterm precedes its parent");
                reachable[k.index()] = true;
            }
        }

        // In index order, every subterm has its final index before its
        // parents are rebuilt, so hashing the rebuilt terms merges equal
        // terms at any depth.
        let mut new_index = vec![None; num_terms];
        let mut terms = Vec::with_capacity(num_terms);
        let mut kept = HashMap::<Term, TermId>::default();

        for (n, e) in self.terms.iter().enumerate() {
            if !reachable[n] {
                continue;
            }
            // The subterms of a reachable term are reachable and precede it.
            let e = e.map_subterms(|k| new_index[k.index()].unwrap());
            let fresh_index = TermId::new(terms.len() as u32);
            let index = *kept.entry(e).or_insert(fresh_index);
            if index == fresh_index {
                terms.push(e);
            }
            new_index[n] = Some(index);
        }

        self.terms = terms;
        self.roots = self
            .roots
            .iter()
            .map(|k| new_index[k.index()].unwrap())
            .collect();
        for pair in &mut self.cuts {
            *pair = pair.map(|k| new_index[k.index()].unwrap());
        }
        if !self.fo.binders.is_empty() {
            // A binder's name follows its term. Where two binders merged,
            // the first name wins; a dropped binder's name goes.
            let mut binders = Vec::with_capacity(self.fo.binders.len());
            for (t, name) in std::mem::take(&mut self.fo.binders) {
                if let Some(t) = new_index[t.index()] {
                    binders.push((t, name));
                }
            }
            binders.sort_by_key(|&(t, _)| t);
            binders.dedup_by_key(|&mut (t, _)| t);
            self.fo.binders = binders;
        }
    }

    /// Numbers the atoms in the order the terms first name them, and drops
    /// the entries no term names. Every literal must name an atom of the
    /// table. The names move into the new table; none is copied.
    fn renumber_atoms(&mut self) {
        use Term::*;
        /// No term names the atom.
        const UNUSED: u32 = u32::MAX;
        // Per entry of the table its new number, and per new number the
        // entry. A table has fewer entries than `u32::MAX`, since `Atom`
        // indexes it, so `UNUSED` is no number.
        let mut number = vec![UNUSED; self.atoms.len()];
        let mut entries = Vec::with_capacity(self.atoms.len());
        for e in &mut self.terms {
            let (Atom(a) | DualAtom(a)) = e else {
                continue;
            };
            let slot = &mut number[a.index()];
            if *slot == UNUSED {
                *slot = entries.len() as u32;
                entries.push(a.index());
            }
            *a = term::Atom::new(*slot);
        }
        let mut names = std::mem::take(&mut self.atoms);
        if !self.fo.atoms.is_empty() {
            let predicates = std::mem::take(&mut self.fo.atoms);
            self.fo.atoms = entries.iter().map(|&entry| predicates[entry]).collect();
        }
        self.atoms = entries
            .into_iter()
            .map(|entry| std::mem::take(&mut names[entry]))
            .collect();
    }

    /// Merges atoms of the same name and equal terms, and drops the terms
    /// that no root or cut formula reaches.
    ///
    /// The root formulas keep the order they were written in. The parser
    /// optimizes what it reads; after [`add`](Self::add), this merges the
    /// terms the two sequents share.
    pub fn optimize(&mut self) {
        // Equal names first, so that terms over them are equal terms.
        self.merge_atoms();
        self.optimize_terms();
        self.renumber_atoms();
    }

    /// Gives atoms of the same name one index, the first one's, and drops
    /// the later entries of the table, so that a name is one atom. A table
    /// of distinct names stays as it is. The terms must refer to atoms of
    /// the table.
    pub(crate) fn merge_atoms(&mut self) {
        use Term::*;
        let mut seen = HashMap::<&str, term::Atom>::with_capacity_and_hasher(
            self.atoms.len(),
            Default::default(),
        );
        let mut merged = Vec::with_capacity(self.atoms.len());
        for name in &self.atoms {
            let fresh = term::Atom::new(seen.len() as u32);
            merged.push(*seen.entry(name).or_insert(fresh));
        }
        let distinct = seen.len();
        drop(seen);
        if distinct == self.atoms.len() {
            return;
        }
        for e in &mut self.terms {
            *e = match *e {
                Atom(a) => Atom(merged[a.index()]),
                DualAtom(a) => DualAtom(merged[a.index()]),
                other @ One
                | other @ Bot
                | other @ Top
                | other @ Zero
                | other @ Tensor(..)
                | other @ Par(..)
                | other @ With(..)
                | other @ Plus(..)
                | other @ Bang(_)
                | other @ Quest(_)
                | other @ Forall(_)
                | other @ Exists(_) => other,
            };
        }
        // An entry stays when it is the first of its name: when its new
        // index is the number of entries kept before it.
        let (mut old, mut kept) = (0, 0);
        self.atoms.retain(|_| {
            let first = merged[old].index() == kept;
            old += 1;
            kept += usize::from(first);
            first
        });
        if !self.fo.atoms.is_empty() {
            let (mut old, mut kept) = (0, 0);
            self.fo.atoms.retain(|_| {
                let first = merged[old].index() == kept;
                old += 1;
                kept += usize::from(first);
                first
            });
        }
    }

    /// Appends the sequent `s` to this one: its root formulas after this
    /// one's, its cut formulas after this one's, its arena after this one's.
    ///
    /// An atom of `s` becomes this sequent's atom of the same name, if
    /// there is one. The sides of the sum are not known, so
    /// [`antecedents`](Self::antecedents) becomes `None`.
    pub fn add(&mut self, s: Self) {
        let offset_atoms = self.atoms.len() as u32;
        let offset_terms = self.terms.len() as u32;

        self.terms.extend(
            s.terms
                .into_iter()
                .map(|e| e.offset(offset_atoms, offset_terms)),
        );

        self.roots.extend(
            s.roots
                .into_iter()
                .map(|n| TermId::new(n.get() + offset_terms)),
        );

        self.cuts.extend(
            s.cuts
                .into_iter()
                .map(|pair| pair.map(|n| TermId::new(n.get() + offset_terms))),
        );

        self.atoms.extend(s.atoms);
        self.join_first_order(s.fo, offset_terms);
        // The roots of `s` follow this sequent's, so its antecedents are
        // not the first roots of the sum.
        self.antecedents = None;
        self.merge_atoms();
    }

    /// Joins the first-order tables of another sequent, whose atoms follow
    /// this one's in the atom table, to this one's.
    ///
    /// - The symbols, terms and lists of both are interned anew.
    /// - A name is one symbol. A symbol of the same name but another arity
    ///   or role is renamed with `_` after it, so nothing is lost.
    /// - An atom without a predicate in its side's tables, as every atom
    ///   of a propositional side is, gets a nullary predicate of its name.
    /// - Every atom's name becomes its canonical text over the joined
    ///   symbols.
    ///
    /// `terms_offset` is where the other sequent's formula terms start.
    fn join_first_order(&mut self, other: fo::FirstOrder, terms_offset: u32) {
        if other.is_empty() && self.fo.is_empty() {
            return;
        }
        let own_atoms = if self.fo.is_empty() {
            self.atoms.len() - other_atoms(&other, self.atoms.len())
        } else {
            self.fo.atoms.len()
        };
        /// How many atoms of the joined table are the other sequent's.
        fn other_atoms(other: &fo::FirstOrder, total: usize) -> usize {
            if other.is_empty() {
                0
            } else {
                other.atoms.len()
            }
            .min(total)
        }
        let mut interner = fo::Interner::new();
        let mut atoms = Vec::with_capacity(self.atoms.len());
        let names = std::mem::take(&mut self.atoms);
        let intern_symbol = |interner: &mut fo::Interner, name: &str, arity, predicate| {
            let mut name = name.to_owned();
            loop {
                match interner.symbol(&name, arity, predicate) {
                    Ok(symbol) => return symbol,
                    Err(_) => name.push('_'),
                }
            }
        };
        let sides = [(&self.fo, 0, own_atoms), (&other, own_atoms, names.len())];
        for (tables, from, to) in sides {
            // A symbol's role on its side: a predicate where an atom has
            // it, else a function symbol.
            let mut predicate = vec![false; tables.symbols.len()];
            for &(p, _) in &tables.atoms {
                predicate[p.index()] = true;
            }
            let symbols: Vec<Symbol> = tables
                .symbols
                .iter()
                .enumerate()
                .map(|(i, (name, arity))| intern_symbol(&mut interner, name, *arity, predicate[i]))
                .collect();
            let mut terms: Vec<FoTermId> = Vec::with_capacity(tables.terms.len());
            let mut lists: Vec<Option<Args>> = vec![None; tables.lists.len()];
            for term in &tables.terms {
                let term = match *term {
                    FoTerm::Bound(i) => FoTerm::Bound(i),
                    FoTerm::Eigen(a) => FoTerm::Eigen(a),
                    FoTerm::Meta(k) => FoTerm::Meta(k),
                    FoTerm::App(f, args) => FoTerm::App(
                        symbols[f.index()],
                        join_list(&mut interner, tables, &terms, &mut lists, args),
                    ),
                };
                terms.push(interner.term(term));
            }
            for (a, name) in names[from..to].iter().enumerate() {
                atoms.push(match tables.atoms.get(a) {
                    Some(&(p, args)) => (
                        symbols[p.index()],
                        join_list(&mut interner, tables, &terms, &mut lists, args),
                    ),
                    None => (
                        intern_symbol(&mut interner, name, 0, true),
                        interner.list(&[]),
                    ),
                });
            }
        }
        /// Returns the list `args` of `tables`, interned over `terms` (the
        /// tables' terms as interned). Each list is interned once.
        fn join_list(
            interner: &mut fo::Interner,
            tables: &fo::FirstOrder,
            terms: &[FoTermId],
            lists: &mut [Option<Args>],
            args: Args,
        ) -> Args {
            *lists[args.index()].get_or_insert_with(|| {
                let items: Vec<FoTermId> =
                    tables.list(args).iter().map(|t| terms[t.index()]).collect();
                interner.list(&items)
            })
        }
        let mut fo = interner.tables;
        fo.atoms = atoms;
        fo.binders = std::mem::take(&mut self.fo.binders);
        fo.binders.extend(
            other
                .binders
                .into_iter()
                .map(|(t, name)| (TermId::new(t.get() + terms_offset), name)),
        );
        self.atoms = fo
            .atoms
            .iter()
            .map(|&(p, args)| fo.atom_key(p, args))
            .collect();
        self.fo = fo;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a sequent from raw parts, as a test would write them.
    pub(crate) fn raw(terms: Vec<Term>, roots: &[u32], atoms: &[&str]) -> Sequent {
        Sequent {
            terms,
            roots: roots.iter().map(|&n| TermId::new(n)).collect(),
            atoms: atoms.iter().map(|s| s.to_string()).collect(),
            antecedents: None,
            cuts: vec![],
            fo: fo::FirstOrder::new(),
        }
    }

    /// Shorthand for `Term::Atom`.
    pub(crate) const fn var(a: u32) -> Term {
        Term::Atom(Atom::new(a))
    }

    /// Shorthand for `Term::DualAtom`.
    pub(crate) const fn dual_var(a: u32) -> Term {
        Term::DualAtom(Atom::new(a))
    }

    /// Shorthand for `Term::Tensor`.
    pub(crate) const fn tensor(k: u32, l: u32) -> Term {
        Term::Tensor(TermId::new(k), TermId::new(l))
    }

    /// Shorthand for `Term::Bang`.
    pub(crate) const fn bang(k: u32) -> Term {
        Term::Bang(TermId::new(k))
    }

    /// Adding a sequent identifies its atoms with those of the same name,
    /// and leaves the others in the order they came.
    #[test]
    fn add_merges_atoms_by_name() {
        let mut s = raw(vec![dual_var(0), var(1)], &[0, 1], &["A", "B"]);
        s.add(raw(
            vec![var(0), var(1), var(2)],
            &[0, 1, 2],
            &["C", "A", "B"],
        ));
        assert_eq!(s.atoms, ["A", "B", "C"]);
        assert_eq!(s.terms, [dual_var(0), var(1), var(2), var(0), var(1)]);
        assert_eq!(s.roots.len(), 5);
    }

    /// Optimizing the empty sequent leaves it empty.
    #[test]
    fn optimize_empty() {
        let mut s = Sequent::new();
        s.optimize();
        assert!(s.terms.is_empty() && s.roots.is_empty() && s.atoms.is_empty());
    }

    /// Atoms of one name merge, and the rest are renumbered to match, also
    /// when a repeated name comes before a new one.
    #[test]
    fn optimize_merges_atoms() {
        let mut s = raw(
            vec![var(0), dual_var(1), var(2)],
            &[0, 1, 2],
            &["A", "A", "B"],
        );
        s.optimize();
        assert_eq!(s.atoms, ["A", "B"]);
        assert_eq!(s.terms, [var(0), dual_var(0), var(1)]);
    }

    /// The atoms are numbered in the order the terms first name them, not
    /// in the table's order, and an atom no term names leaves the table.
    #[test]
    fn optimize_numbers_atoms_as_the_terms_name_them() {
        let mut s = raw(
            vec![var(2), dual_var(0), var(2)],
            &[0, 1, 2],
            &["B", "C", "A"],
        );
        s.optimize();
        assert_eq!(s.atoms, ["A", "B"]);
        assert_eq!(s.terms, [var(0), dual_var(1)]);
        assert_eq!(s.roots, [TermId::new(0), TermId::new(1), TermId::new(0)]);
    }

    /// References to a duplicate term point to its kept copy, also after
    /// unreachable terms before it are dropped.
    #[test]
    fn optimize_redirects_duplicates() {
        use Term::*;
        let mut s = raw(vec![Zero, One, Top, Top, tensor(1, 3)], &[2, 4], &[]);
        s.optimize();
        assert_eq!(s.terms, [One, Top, tensor(0, 1)]);
        assert_eq!(s.roots, [TermId::new(1), TermId::new(2)]);
    }

    /// A subterm that does not precede its parent is reported with both
    /// indices, each in its place.
    #[test]
    fn subterm_order_error_names_both_terms() {
        use Term::*;
        let s = raw(vec![One, bang(2), Bot], &[1], &[]);
        assert_eq!(
            s.check().unwrap_err().to_string(),
            "term 1 refers to term 2, but a subterm must come before the terms that use it"
        );
    }

    /// Equal terms merge at every depth, not only equal leaves.
    #[test]
    fn optimize_merges_equal_subterms() {
        let mut s = raw(
            vec![
                var(0),
                var(1),
                tensor(0, 1),
                bang(2),
                var(0),
                var(1),
                tensor(4, 5),
                bang(6),
            ],
            &[3, 7],
            &["A", "B"],
        );
        s.optimize();
        assert_eq!(s.terms, [var(0), var(1), tensor(0, 1), bang(2)]);
        assert_eq!(s.roots, [TermId::new(3), TermId::new(3)]);
    }

    /// Appending a sequent shifts its term and atom indices past the existing
    /// ones.
    #[test]
    fn add_offsets_indices() {
        let mut s = raw(vec![var(0)], &[0], &["A"]);
        s.add(raw(vec![dual_var(0), bang(0)], &[1], &["B"]));
        assert_eq!(s.terms, [var(0), dual_var(1), bang(1)]);
        assert_eq!(s.roots, [TermId::new(0), TermId::new(2)]);
        assert_eq!(s.atoms, ["A", "B"]);
    }
}
