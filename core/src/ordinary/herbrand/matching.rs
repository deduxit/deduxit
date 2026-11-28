// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The matching of terms and formulas against the instances found: which
//! closed terms they hold, and which values make an open formula one of
//! them.

use crate::first_order::{FoTerm, FoTermId, Symbol};
use crate::hash::{HashMap, HashSet};
use crate::ordinary::{Formulas, Node, NodeId};

/// Returns every closed term of the instances `roots` and every term
/// inside one, the shallowest first.
pub(super) fn witnesses_of(formulas: &Formulas, roots: &[NodeId]) -> Vec<FoTermId> {
    let mut found: Vec<FoTermId> = formulas
        .terms_in(roots.iter().copied(), std::iter::empty())
        .collect();
    let depth = term_depths(formulas, &found);
    found.sort_unstable_by_key(|t| (depth[t], *t));
    found
}

/// Returns the atoms of each of the ground instances `roots`: its
/// predicate and its arguments.
pub(super) fn ground_atoms(
    formulas: &Formulas,
    roots: &[NodeId],
) -> Vec<Vec<(Symbol, Vec<FoTermId>)>> {
    let tables = formulas.first_order();
    roots
        .iter()
        .map(|&root| {
            let mut atoms = Vec::new();
            let mut nodes = vec![root];
            while let Some(n) = nodes.pop() {
                match formulas.node(n) {
                    Node::Atom(a) => {
                        if let Some(&(symbol, args)) = tables.atoms.get(a as usize) {
                            atoms.push((symbol, tables.list(args).to_vec()));
                        }
                    }
                    Node::True | Node::False => {}
                    Node::Not(a) | Node::Forall(a) | Node::Exists(a) => nodes.push(a),
                    Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                        nodes.extend([a, b]);
                    }
                }
            }
            atoms
        })
        .collect()
}

/// Returns the atoms of the body `body` of a quantifier, each with its
/// predicate, its arguments and how many binders inside the body stand
/// above it: there the quantifier's variable is that bound index.
pub(super) fn body_atoms(formulas: &Formulas, body: NodeId) -> Vec<(Symbol, Vec<FoTermId>, u32)> {
    let tables = formulas.first_order();
    let mut seen: HashSet<(NodeId, u32)> = HashSet::default();
    let mut atoms = Vec::new();
    let mut nodes = vec![(body, 0u32)];
    while let Some((n, depth)) = nodes.pop() {
        if !seen.insert((n, depth)) {
            continue;
        }
        match formulas.node(n) {
            Node::Atom(a) => {
                if let Some(&(symbol, args)) = tables.atoms.get(a as usize) {
                    atoms.push((symbol, tables.list(args).to_vec(), depth));
                }
            }
            Node::True | Node::False => {}
            Node::Not(a) => nodes.push((a, depth)),
            Node::Forall(a) | Node::Exists(a) => nodes.push((a, depth + 1)),
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                nodes.extend([(a, depth), (b, depth)]);
            }
        }
    }
    atoms
}

/// How a body's atom matches a ground atom.
pub(super) enum Match {
    /// They do not match.
    No,
    /// They match whatever the quantifier's variable is: it does not occur.
    Free,
    /// They match where the variable is this term.
    At(FoTermId),
}

/// Matches the arguments `pattern` of a body's atom, where the
/// quantifier's variable is the bound index `depth` and a lower index is a
/// variable bound inside the body (any term), against the ground arguments
/// `ground`.
pub(super) fn match_terms(
    tables: &crate::first_order::FirstOrder<NodeId>,
    pattern: &[FoTermId],
    ground: &[FoTermId],
    depth: u32,
) -> Match {
    let mut value: Option<FoTermId> = None;
    let mut pairs: Vec<(FoTermId, FoTermId)> = pattern
        .iter()
        .copied()
        .zip(ground.iter().copied())
        .collect();
    while let Some((p, g)) = pairs.pop() {
        if p == g {
            continue;
        }
        match (tables.terms[p.index()], tables.terms[g.index()]) {
            (FoTerm::Bound(k), _) if k == depth => match value {
                Some(v) if v != g => return Match::No,
                _ => value = Some(g),
            },
            (FoTerm::Bound(_), _) => {}
            (FoTerm::App(f, pa), FoTerm::App(h, ga)) if f == h => {
                let (pa, ga) = (tables.list(pa), tables.list(ga));
                if pa.len() != ga.len() {
                    return Match::No;
                }
                pairs.extend(pa.iter().copied().zip(ga.iter().copied()));
            }
            (FoTerm::App(..) | FoTerm::Eigen(_) | FoTerm::Meta(_), _) => return Match::No,
        }
    }
    value.map_or(Match::Free, Match::At)
}

/// Returns the depth of each of `terms` and of every term inside them, a
/// constant being of depth 0.
fn term_depths(formulas: &Formulas, terms: &[FoTermId]) -> HashMap<FoTermId, u32> {
    let tables = formulas.first_order();
    let mut depth: HashMap<FoTermId, u32> = HashMap::default();
    let mut stack: Vec<(FoTermId, bool)> = terms.iter().map(|&t| (t, false)).collect();
    while let Some((t, ready)) = stack.pop() {
        if depth.contains_key(&t) {
            continue;
        }
        let FoTerm::App(_, args) = tables.terms[t.index()] else {
            depth.insert(t, 0);
            continue;
        };
        let list = tables.list(args);
        if !ready {
            stack.push((t, true));
            stack.extend(list.iter().map(|&u| (u, false)));
            continue;
        }
        let deepest = list.iter().map(|u| depth[u] + 1).max().unwrap_or(0);
        depth.insert(t, deepest);
    }
    depth
}

/// Returns the values of the variables free in `pattern`, an open formula,
/// under which it is the closed formula `target`, by their outside index;
/// `None` where it is no instance.
pub(super) fn matched(
    formulas: &Formulas,
    pattern: NodeId,
    target: NodeId,
) -> Option<Vec<Option<FoTermId>>> {
    let tables = formulas.first_order();
    let mut values: Vec<Option<FoTermId>> = Vec::new();
    let mut nodes = vec![(pattern, target, 0u32)];
    let mut terms: Vec<(FoTermId, FoTermId, u32)> = Vec::new();
    while let Some((p, t, depth)) = nodes.pop() {
        match (formulas.node(p), formulas.node(t)) {
            (Node::Atom(a), Node::Atom(b)) => {
                match (tables.atoms.get(a as usize), tables.atoms.get(b as usize)) {
                    (Some(&(f, x)), Some(&(g, y))) => {
                        let (x, y) = (tables.list(x), tables.list(y));
                        if f != g || x.len() != y.len() {
                            return None;
                        }
                        terms.extend(x.iter().zip(y).map(|(&u, &v)| (u, v, depth)));
                    }
                    (None, None) if a == b => {}
                    _ => return None,
                }
            }
            (Node::True, Node::True) | (Node::False, Node::False) => {}
            (Node::Not(a), Node::Not(b)) => nodes.push((a, b, depth)),
            (Node::And(a, b), Node::And(c, d))
            | (Node::Or(a, b), Node::Or(c, d))
            | (Node::Implies(a, b), Node::Implies(c, d))
            | (Node::Iff(a, b), Node::Iff(c, d)) => {
                nodes.push((a, c, depth));
                nodes.push((b, d, depth));
            }
            (Node::Forall(a), Node::Forall(b)) | (Node::Exists(a), Node::Exists(b)) => {
                nodes.push((a, b, depth + 1));
            }
            _ => return None,
        }
    }
    while let Some((u, v, depth)) = terms.pop() {
        match (tables.terms[u.index()], tables.terms[v.index()]) {
            (FoTerm::Bound(i), _) if i >= depth => {
                if !formulas.is_closed(v) {
                    return None;
                }
                let k = (i - depth) as usize;
                if values.len() <= k {
                    values.resize(k + 1, None);
                }
                match values[k] {
                    Some(w) if w != v => return None,
                    Some(_) => {}
                    None => values[k] = Some(v),
                }
            }
            (FoTerm::App(f, x), FoTerm::App(g, y)) => {
                let (x, y) = (tables.list(x), tables.list(y));
                if f != g || x.len() != y.len() {
                    return None;
                }
                terms.extend(x.iter().zip(y).map(|(&a, &b)| (a, b, depth)));
            }
            _ if u == v => {}
            _ => return None,
        }
    }
    Some(values)
}
