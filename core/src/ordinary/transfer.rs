// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Formulas copied from one arena into another: a goal of a proof in
//! progress into an arena of its own, which a search takes, and the
//! derivation the search found back into the proof's arena; a normal
//! form's input into the arena of its claims; a written tableau's
//! expected sequent into the tableau's arena.
//!
//! Every pass is in index order (an operand and a subterm precede what is
//! made of them), so nothing recurses over a formula.

use super::{Formulas, Node, NodeId};
use crate::Error;
use crate::first_order::{Eigen, FoTerm, FoTermId};
use crate::hash::{HashMap, HashSet};

/// What the eigenvariables and open witnesses of the source become in
/// the target.
pub(super) enum Parameters<'a> {
    /// Each a constant of its own: an eigenvariable named as the source
    /// prints it, an open witness by a name past those, apart from the
    /// symbols. A search takes them as the constants they stand for.
    Constants,
    /// A constant named in `constants` is the target's term there, the
    /// parameter it stood for; every eigenvariable of the source is one
    /// numbered `offset` higher, past every one the target's proof holds;
    /// a symbol or atom whose name `known` lacks, which the source made
    /// itself (a fresh constant, a Skolem symbol), is renamed apart from
    /// the target's names, so that it meets none of the target's other
    /// formulas in another role.
    #[cfg_attr(
        not(feature = "interactive"),
        expect(dead_code, reason = "only the goal state copies a derivation back")
    )]
    Back {
        /// The constants made by [`Parameters::Constants`], by name.
        constants: &'a HashMap<String, FoTermId>,
        /// The names of the arena that copy made ([`Copied::names`]).
        known: &'a HashSet<String>,
        /// What the source's eigenvariables' numbers are raised by.
        offset: u32,
    },
}

/// What was copied: the target's id of each node and term of the source
/// that was, and the constants made for parameters.
pub(super) struct Copied {
    /// By the source's node index.
    pub(super) nodes: Vec<Option<NodeId>>,
    /// By the source's term index.
    pub(super) terms: Vec<Option<FoTermId>>,
    /// Each constant made for a parameter, by its name, with the source's
    /// term it stands for.
    pub(super) constants: HashMap<String, FoTermId>,
    /// Whether one of them stands for an open witness.
    pub(super) metas: bool,
    /// The target's symbols' and atoms' names after the copy.
    pub(super) names: HashSet<String>,
}

impl Copied {
    /// Returns the target's id of the source's node `n`, which was copied.
    pub(super) fn node(&self, n: NodeId) -> NodeId {
        self.nodes[n.index()].expect("a node the copy reached")
    }
}

/// Copies into `target` the formulas `roots` of `source` with everything
/// they are made of, or every node and term of `source` for `None`, the
/// parameters as `parameters` say. A binder keeps its name. The target is
/// first-order where what is copied holds a binder or a term; otherwise
/// every atom is copied by its name alone.
///
/// # Errors
///
/// [`Error::Symbol`] where a symbol is the target's in another role or of
/// another arity, and [`Refusal::Index`](crate::Refusal::Index) when the
/// target is full.
#[cfg(feature = "interactive")]
pub(super) fn copy(
    target: &mut Formulas,
    source: &Formulas,
    roots: Option<&[NodeId]>,
    parameters: &Parameters<'_>,
) -> Result<Copied, Error> {
    copy_as(target, source, roots, parameters, false)
}

/// [`copy`], with the target first-order wherever the source is, even
/// where what is copied holds no binder and no term: each atom then keeps
/// its predicate, as a normal form's claims read the result's atoms.
///
/// # Errors
///
/// [`copy`]'s.
pub(super) fn copy_first_order(
    target: &mut Formulas,
    source: &Formulas,
    roots: Option<&[NodeId]>,
    parameters: &Parameters<'_>,
) -> Result<Copied, Error> {
    copy_as(target, source, roots, parameters, true)
}

/// [`copy`], or with `keep` the target first-order wherever the source is,
/// entering it at the first atom, after the terms, as a normal form's
/// claims always made their arena (so the target's symbols are numbered
/// as before: the function symbols first).
fn copy_as(
    target: &mut Formulas,
    source: &Formulas,
    roots: Option<&[NodeId]>,
    parameters: &Parameters<'_>,
    keep: bool,
) -> Result<Copied, Error> {
    let tables = source.first_order();
    let (nodes, terms) = reached(source, roots);
    let first_order = (keep && source.is_first_order())
        || terms.iter().any(|&t| t)
        || nodes
            .iter()
            .enumerate()
            .any(|(n, &r)| r && matches!(source.nodes()[n], Node::Forall(_) | Node::Exists(_)));
    if first_order && !keep {
        target.enter_first_order()?;
    }
    let mut copied = Copied {
        nodes: vec![None; source.len()],
        terms: vec![None; tables.terms.len()],
        constants: HashMap::default(),
        metas: false,
        names: HashSet::default(),
    };
    let mut renamed: HashMap<String, String> = HashMap::default();
    // The names of the open witnesses' constants follow the eigenvariables'.
    let eigens = source.eigen_names();
    let mut fresh = Vec::new();
    let mut metas = 0;
    for (t, term) in tables.terms.iter().enumerate() {
        if !terms[t] {
            continue;
        }
        let made = match (*term, parameters) {
            (FoTerm::Bound(i), _) => target.interner().term(FoTerm::Bound(i)),
            (FoTerm::Eigen(e), Parameters::Constants) => {
                let name = eigens
                    .binary_search_by_key(&e.get(), |&(k, _)| k)
                    .map(|at| eigens[at].1.clone())
                    .expect("every eigenvariable of the arena has a name");
                parameter(target, &mut copied, name, t)?
            }
            (FoTerm::Meta(_), Parameters::Constants) => {
                let name = next_name(&mut fresh, eigens.len(), metas, tables);
                metas += 1;
                copied.metas = true;
                parameter(target, &mut copied, name, t)?
            }
            (FoTerm::Eigen(e), Parameters::Back { offset, .. }) => {
                let number =
                    e.get()
                        .checked_add(*offset)
                        .ok_or(Error::Refused(crate::Refusal::Index {
                            what: crate::limits::Space::Term,
                            count: u64::from(e.get()) + u64::from(*offset),
                            most: u64::from(u32::MAX),
                        }))?;
                target.interner().term(FoTerm::Eigen(Eigen::new(number)))
            }
            (FoTerm::Meta(k), Parameters::Back { .. }) => target.interner().term(FoTerm::Meta(k)),
            (FoTerm::App(symbol, args), _) => {
                let (name, arity) = &tables.symbols[symbol.index()];
                let back = match parameters {
                    Parameters::Back { constants, .. } if *arity == 0 => {
                        constants.get(name).copied()
                    }
                    Parameters::Back { .. } | Parameters::Constants => None,
                };
                match back {
                    Some(term) => term,
                    None => {
                        let args: Vec<FoTermId> = tables
                            .list(args)
                            .iter()
                            .map(|a| copied.terms[a.index()].expect("a subterm precedes its term"))
                            .collect();
                        let name = apart(target, parameters, &mut renamed, name);
                        let symbol = symbol_of(target, &name, *arity, false)?;
                        let args = target.interner().list(&args);
                        target.interner().term(FoTerm::App(symbol, args))
                    }
                }
            }
        };
        copied.terms[t] = Some(made);
    }
    for (n, &node) in source.nodes().iter().enumerate() {
        if !nodes[n] {
            continue;
        }
        let at = |k: NodeId| copied.nodes[k.index()].expect("an operand precedes its formula");
        let made = match node {
            Node::Atom(a) if first_order => {
                if keep {
                    target.enter_first_order()?;
                }
                let (symbol, args) = tables.atoms[a as usize];
                let (name, arity) = &tables.symbols[symbol.index()];
                let args: Vec<FoTermId> = tables
                    .list(args)
                    .iter()
                    .map(|t| copied.terms[t.index()].expect("an atom's term was reached"))
                    .collect();
                let name = apart(target, parameters, &mut renamed, name);
                let symbol = symbol_of(target, &name, *arity, true)?;
                target.applied_atom(symbol, &args)?
            }
            Node::Atom(a) => {
                let name = apart(target, parameters, &mut renamed, source.atom_name(a));
                target.atom(&name)?
            }
            Node::True | Node::False => target.add(node)?,
            Node::Not(a) => target.add(Node::Not(at(a)))?,
            Node::And(a, b) => target.add(Node::And(at(a), at(b)))?,
            Node::Or(a, b) => target.add(Node::Or(at(a), at(b)))?,
            Node::Implies(a, b) => target.add(Node::Implies(at(a), at(b)))?,
            Node::Iff(a, b) => target.add(Node::Iff(at(a), at(b)))?,
            Node::Forall(a) | Node::Exists(a) => {
                // As many nodes as the arena's ids number.
                let id = NodeId(n as u32);
                let name = tables.binder_name(id).unwrap_or("x");
                target.binder(matches!(node, Node::Forall(_)), at(a), name)?
            }
        };
        copied.nodes[n] = Some(made);
    }
    copied.names = target
        .first_order()
        .symbols
        .iter()
        .map(|(name, _)| name.clone())
        .chain(target.atom_names().iter().cloned())
        .collect();
    Ok(copied)
}

/// Returns the name a symbol or atom of the source called `name` takes in
/// the target: itself, but in [`Parameters::Back`] for a name `known`
/// lacks, the first of `name_1`, `name_2`, … that names nothing of the
/// target's and nothing renamed before, the same each time it is met.
fn apart(
    target: &Formulas,
    parameters: &Parameters<'_>,
    renamed: &mut HashMap<String, String>,
    name: &str,
) -> String {
    let Parameters::Back { known, .. } = parameters else {
        return name.to_owned();
    };
    if known.contains(name) {
        return name.to_owned();
    }
    if let Some(done) = renamed.get(name) {
        return done.clone();
    }
    let taken = |candidate: &str| {
        target.has_symbol(candidate)
            || target.atom_names().iter().any(|a| a == candidate)
            || known.contains(candidate)
            || renamed.values().any(|r| r == candidate)
    };
    let fresh = (1u64..)
        .map(|k| format!("{name}_{k}"))
        .find(|candidate| !taken(candidate))
        .expect("the names are endless");
    renamed.insert(name.to_owned(), fresh.clone());
    fresh
}

/// Returns the name of the `k`-th open witness's constant: the name after
/// the `eigens` eigenvariables' and the `k` before, apart from the
/// symbols of `tables`, `fresh` the names made so far, doubled as needed.
fn next_name(
    fresh: &mut Vec<String>,
    eigens: usize,
    k: usize,
    tables: &crate::first_order::FirstOrder<NodeId>,
) -> String {
    let at = eigens + k;
    if fresh.len() <= at {
        *fresh = super::instance::eigen_names(tables, 2 * at + 1);
    }
    fresh[at].clone()
}

/// Returns the constant named `name` in `target`, made for the source's
/// parameter term `t`, and records it.
fn parameter(
    target: &mut Formulas,
    copied: &mut Copied,
    name: String,
    t: usize,
) -> Result<FoTermId, Error> {
    let symbol = symbol_of(target, &name, 0, false)?;
    let args = target.interner().list(&[]);
    let term = target.interner().term(FoTerm::App(symbol, args));
    // As many terms as a `u32` numbers.
    copied.constants.insert(name, FoTermId::new(t as u32));
    Ok(term)
}

/// Returns the symbol `name` of `arity` in `target`'s tables, a predicate
/// or a function symbol as `predicate` says.
fn symbol_of(
    target: &mut Formulas,
    name: &str,
    arity: u32,
    predicate: bool,
) -> Result<crate::first_order::Symbol, Error> {
    target
        .interner()
        .symbol(name, arity, predicate)
        .map_err(|fault| Error::Symbol {
            name: name.to_owned(),
            fault,
        })
}

/// Returns which nodes and which terms of `source` the formulas `roots`
/// are made of, every one for `None`.
fn reached(source: &Formulas, roots: Option<&[NodeId]>) -> (Vec<bool>, Vec<bool>) {
    let tables = source.first_order();
    let Some(roots) = roots else {
        return (vec![true; source.len()], vec![true; tables.terms.len()]);
    };
    let mut nodes = vec![false; source.len()];
    let mut terms = vec![false; tables.terms.len()];
    let mut walk = source.terms_in(roots.iter().copied(), std::iter::empty());
    for t in walk.by_ref() {
        terms[t.index()] = true;
    }
    for k in walk.nodes() {
        nodes[k.index()] = true;
    }
    (nodes, terms)
}
