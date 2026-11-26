// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Formulas copied from one arena into another: a goal of a proof in
//! progress into an arena of its own, which a search takes, and the
//! derivation the search found back into the proof's arena.
//!
//! Every pass is in index order (an operand and a subterm precede what is
//! made of them), so nothing recurses over a formula.

use super::{Formulas, Node, NodeId};
use crate::Error;
use crate::first_order::{Eigen, FoTerm, FoTermId};
use crate::hash::HashMap;

/// What the eigenvariables and open witnesses of the source become in
/// the target.
pub(super) enum Parameters<'a> {
    /// Each a constant of its own: an eigenvariable named as the source
    /// prints it, an open witness by a name past those, apart from the
    /// symbols. A search takes them as the constants they stand for.
    Constants,
    /// A constant named in `constants` is the target's term there, the
    /// parameter it stood for; every eigenvariable of the source is one
    /// numbered `offset` higher, past every one the target's proof holds.
    Back {
        /// The constants made by [`Parameters::Constants`], by name.
        constants: &'a HashMap<String, FoTermId>,
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
pub(super) fn copy(
    target: &mut Formulas,
    source: &Formulas,
    roots: Option<&[NodeId]>,
    parameters: &Parameters<'_>,
) -> Result<Copied, Error> {
    let tables = source.first_order();
    let (nodes, terms) = reached(source, roots);
    let first_order = terms.iter().any(|&t| t)
        || nodes
            .iter()
            .enumerate()
            .any(|(n, &r)| r && matches!(source.nodes[n], Node::Forall(_) | Node::Exists(_)));
    if first_order {
        target.enter_first_order()?;
    }
    let mut copied = Copied {
        nodes: vec![None; source.len()],
        terms: vec![None; tables.terms.len()],
        constants: HashMap::default(),
        metas: false,
    };
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
                        let symbol = symbol_of(target, name, *arity, false)?;
                        let args = target.interner().list(&args);
                        target.interner().term(FoTerm::App(symbol, args))
                    }
                }
            }
        };
        copied.terms[t] = Some(made);
    }
    for (n, &node) in source.nodes.iter().enumerate() {
        if !nodes[n] {
            continue;
        }
        let at = |k: NodeId| copied.nodes[k.index()].expect("an operand precedes its formula");
        let made = match node {
            Node::Atom(a) if first_order => {
                let (symbol, args) = tables.atoms[a as usize];
                let (name, arity) = &tables.symbols[symbol.index()];
                let args: Vec<FoTermId> = tables
                    .list(args)
                    .iter()
                    .map(|t| copied.terms[t.index()].expect("an atom's term was reached"))
                    .collect();
                let symbol = symbol_of(target, name, *arity, true)?;
                target.applied_atom(symbol, &args)?
            }
            Node::Atom(a) => target.atom(source.atom_name(a))?,
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
    Ok(copied)
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
    let mut stack: Vec<NodeId> = roots.to_vec();
    let mut term_stack: Vec<FoTermId> = Vec::new();
    while let Some(k) = stack.pop() {
        if std::mem::replace(&mut nodes[k.index()], true) {
            continue;
        }
        match source.nodes[k.index()] {
            Node::Atom(a) => {
                if let Some(&(_, args)) = tables.atoms.get(a as usize) {
                    term_stack.extend_from_slice(tables.list(args));
                }
            }
            Node::True | Node::False => {}
            Node::Not(a) | Node::Forall(a) | Node::Exists(a) => stack.push(a),
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                stack.push(a);
                stack.push(b);
            }
        }
    }
    while let Some(t) = term_stack.pop() {
        if std::mem::replace(&mut terms[t.index()], true) {
            continue;
        }
        if let FoTerm::App(_, args) = tables.terms[t.index()] {
            term_stack.extend_from_slice(tables.list(args));
        }
    }
    (nodes, terms)
}
