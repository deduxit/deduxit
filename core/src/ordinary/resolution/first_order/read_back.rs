// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The read-back of a refutation to a derivation of LK, through the
//! ground instances it used.

use super::clauses::map_term;
use super::{Clause, Clauses, MOST_INSTANCES, Refutation, Step};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::derivation::ArenaSize;
use crate::ordinary::normal::NormalForm;
use crate::ordinary::{Derivation, Formulas, Node, NodeId, Sequent, normal};

/// Reads a checked refutation back as a derivation of LK of `sequent`:
/// each clause the refutation uses gets the ground substitutions its
/// consequences need, `□` the empty one, so the given clauses' instances
/// are a ground refutation's; from them Herbrand's construction builds the
/// derivation, each Skolem term an eigenvariable, and checks it.
///
/// # Errors
///
/// [`Refusal::Occurrences`] past [`MOST_INSTANCES`] instances;
/// [`Refusal::Terms`] for an instance's atom past `limits.terms`;
/// [`Refusal::Memory`] where the instances pass `limits.memory_bytes`;
/// [`Refusal::Stopped`] when the stop fires; Herbrand's construction's
/// errors.
pub(super) fn read_back(
    sequent: &Sequent,
    clausal: &NormalForm,
    refutation: &Refutation,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Derivation, Error> {
    // A ground instance of a paramodulant follows from its parents' only
    // with equality's axioms, which the refutation does not use: LK reads
    // no such step back.
    if refutation
        .inferences
        .iter()
        .any(|i| matches!(i.step, Step::Paramodulation { .. }))
    {
        return Err(Error::ReadBack {
            calculus: "LK",
            reason: "a refutation by paramodulation is checked step by step; LK has no rule for \
                     the step"
                .to_owned(),
        });
    }
    let mut set = Clauses {
        formulas: refutation.clauses.formulas.clone(),
        items: refutation.clauses.items[..refutation.clauses.given].to_vec(),
        given: refutation.clauses.given,
        most_terms: refutation.clauses.most_terms,
        sizes: HashMap::default(),
    };
    // The terms put for variables no consequence fixes: a constant of the
    // clauses, or a new one.
    // A propositional arena stays without tables: it has no variable to
    // put a constant for.
    let default = if set.formulas.is_first_order() {
        Some(default_constant(&mut set)?)
    } else {
        None
    };
    let mut made = Vec::with_capacity(refutation.inferences.len());
    for inference in &refutation.inferences {
        let Ok(m) = set.made(inference.step)? else {
            return Err(Error::ReadBack {
                calculus: "LK",
                reason: "a checked step does not apply".to_owned(),
            });
        };
        set.items.push(m.clause.clone());
        made.push(m);
    }
    let total = set.items.len();
    let mut needed: Vec<Vec<Vec<FoTermId>>> = vec![Vec::new(); total];
    let empty = if refutation.inferences.is_empty() {
        set.items
            .iter()
            .position(Clause::is_empty)
            .unwrap_or(total - 1)
    } else {
        total - 1
    };
    needed[empty].push(Vec::new());
    let mut count = 0usize;
    // The read-back's steps, each an instance of a clause, for the stop.
    let mut steps = 0u64;
    let mut poll = |steps: &mut u64| {
        *steps += 1;
        if stop(Progress::new(Phase::ReadBack, 1, *steps)) {
            return Err(Error::Refused(Refusal::stopped(Phase::ReadBack)));
        }
        Ok(())
    };
    for k in (set.given..total).rev() {
        let m = &made[k - set.given];
        let wants = std::mem::take(&mut needed[k]);
        for theta in wants {
            poll(&mut steps)?;
            // Each variable of the step ground: its term under the unifier
            // with the variables left free put as the clause's.
            let mut ground = Vec::with_capacity(m.terms.len());
            for &t in &m.terms {
                let mut memo = HashMap::default();
                let image = map_term(
                    &mut set.formulas,
                    t,
                    &mut |_, j| {
                        m.renamed
                            .get(j as usize)
                            .copied()
                            .flatten()
                            .and_then(|r| theta.get(r as usize).copied())
                            .or(default)
                            .unwrap_or(t)
                    },
                    &mut memo,
                );
                ground.push(image);
            }
            let split = m.first as usize;
            let (first, second) = match refutation.inferences[k - set.given].step {
                Step::Resolution { first, second, .. } => (first, Some(second)),
                Step::Factor { clause, .. } => (clause, None),
                Step::Paramodulation { from, into, .. } => (from, Some(into)),
            };
            let mut add = |c: usize, sub: Vec<FoTermId>| {
                if !needed[c].contains(&sub) {
                    needed[c].push(sub);
                    count += 1;
                }
            };
            add(first, ground[..split].to_vec());
            if let Some(second) = second {
                add(second, ground[split..].to_vec());
            }
            if count > MOST_INSTANCES {
                return Err(Error::Refused(Refusal::Occurrences {
                    occurrences: count as u64,
                    limit: MOST_INSTANCES as u64,
                }));
            }
        }
    }
    // The given clauses' ground instances, as formulas. Each atom is held
    // to the bound on terms before its key is written, and the arena they
    // grow to the memory bound: a unifier that doubles a term per variable
    // made keys of millions of symbols.
    let memory = limits.memory_bytes.unwrap_or(u64::MAX);
    let mut held = ArenaSize::of(&set.formulas, &ArenaSize::default());
    let start = held.bytes;
    // Refuses the instances past the bound where they have grown the
    // arena by `grown` bytes: the copy into the derivation's arena below
    // holds as much again.
    let within = |grown: u64| {
        let needed_bytes = grown.saturating_mul(2);
        if needed_bytes > memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::ReadBack,
                limit_bytes: memory,
                needed_bytes: Some(needed_bytes),
            }));
        }
        Ok(())
    };
    let mut roots = Vec::new();
    for c in 0..set.given {
        for theta in &needed[c] {
            poll(&mut steps)?;
            let clause = set.items[c].clone();
            let mut instance: Option<NodeId> = None;
            for l in &clause.literals {
                // What the instances hold once this atom's key is written
                // (its text is at least a byte a symbol, held twice),
                // charged before the key is written.
                let grown = held.bytes.saturating_sub(start);
                let mut charge = |size: u64| within(grown.saturating_add(size.saturating_mul(2)));
                let atom = set.mapped_atom_charged(
                    l.atom,
                    &mut |formulas, j| {
                        theta
                            .get(j as usize)
                            .copied()
                            .or(default)
                            .unwrap_or_else(|| formulas.interner().term(FoTerm::Meta(j)))
                    },
                    &mut charge,
                )?;
                held = ArenaSize::of(&set.formulas, &held);
                within(held.bytes.saturating_sub(start))?;
                let literal = if l.positive {
                    atom
                } else {
                    set.formulas.add(Node::Not(atom))?
                };
                instance = Some(match instance {
                    None => literal,
                    Some(before) => set.formulas.add(Node::Or(before, literal))?,
                });
            }
            let instance = match instance {
                Some(i) => i,
                None => set.formulas.add(Node::False)?,
            };
            roots.push(instance);
        }
    }
    // A derivation's arena holds no metavariable: the instances go into a
    // copy of the clausal form's, which the input's ids are ids of.
    let mut clean = clausal.result.formulas().clone();
    let mut copied = Copied::default();
    let mut ground_roots = Vec::with_capacity(roots.len());
    for root in roots {
        poll(&mut steps)?;
        ground_roots.push(copied.formula(&set.formulas, &mut clean, root)?);
    }
    let default = match default {
        Some(d) => vec![copied.term(&set.formulas, &mut clean, d)?],
        None => Vec::new(),
    };
    let ground = Sequent::new(clean, ground_roots, Vec::new())?;
    crate::ordinary::herbrand::deskolemized(sequent, clausal, &ground, &default, limits, stop)
}

/// What a copy of ground formulas into another arena has made, by the
/// source's ids.
#[derive(Debug, Default)]
struct Copied {
    /// Each node copied.
    nodes: HashMap<NodeId, NodeId>,
    /// Each term copied.
    terms: HashMap<FoTermId, FoTermId>,
}

impl Copied {
    /// Returns the ground formula `root` of `from` made in `to`, its
    /// symbols by name.
    fn formula(
        &mut self,
        from: &Formulas,
        to: &mut Formulas,
        root: NodeId,
    ) -> Result<NodeId, Error> {
        let mut stack = vec![(root, false)];
        while let Some((n, ready)) = stack.pop() {
            if self.nodes.contains_key(&n) {
                continue;
            }
            let node = from.node(n);
            if !ready {
                stack.push((n, true));
                let (a, b) = node.operands();
                stack.extend([a, b].into_iter().flatten().map(|k| (k, false)));
                continue;
            }
            let image = match node {
                Node::Atom(a) => match from.first_order().atoms.get(a as usize) {
                    Some(&(p, args)) => {
                        let args = from.first_order().list(args).to_vec();
                        let mut items = Vec::with_capacity(args.len());
                        for t in args {
                            items.push(self.term(from, to, t)?);
                        }
                        let symbol = symbol_in(from, to, p, true)?;
                        let list = to.interner().list(&items);
                        to.predicate_atom(symbol, list)?
                    }
                    None => to.atom(from.atom_name(a))?,
                },
                Node::True => to.add(Node::True)?,
                Node::False => to.add(Node::False)?,
                Node::Not(a) => to.add(Node::Not(self.nodes[&a]))?,
                Node::Or(a, b) => to.add(Node::Or(self.nodes[&a], self.nodes[&b]))?,
                Node::And(a, b) => to.add(Node::And(self.nodes[&a], self.nodes[&b]))?,
                Node::Implies(..) | Node::Iff(..) | Node::Forall(_) | Node::Exists(_) => {
                    return Err(Error::ReadBack {
                        calculus: "LK",
                        reason: "a ground instance is no clause".to_owned(),
                    });
                }
            };
            self.nodes.insert(n, image);
        }
        Ok(self.nodes[&root])
    }

    /// Returns the ground term `t` of `from` made in `to`.
    fn term(&mut self, from: &Formulas, to: &mut Formulas, t: FoTermId) -> Result<FoTermId, Error> {
        let mut stack = vec![(t, false)];
        while let Some((u, ready)) = stack.pop() {
            if self.terms.contains_key(&u) {
                continue;
            }
            let FoTerm::App(f, args) = from.first_order().terms[u.index()] else {
                return Err(Error::ReadBack {
                    calculus: "LK",
                    reason: "a ground instance holds a variable".to_owned(),
                });
            };
            let list = from.first_order().list(args).to_vec();
            if !ready {
                stack.push((u, true));
                stack.extend(list.into_iter().map(|v| (v, false)));
                continue;
            }
            let items: Vec<FoTermId> = list.iter().map(|v| self.terms[v]).collect();
            let symbol = symbol_in(from, to, f, false)?;
            let args = to.interner().list(&items);
            let image = to.interner().term(FoTerm::App(symbol, args));
            self.terms.insert(u, image);
        }
        Ok(self.terms[&t])
    }
}

/// Returns the symbol of `to` named and of the arity of `symbol` of `from`.
fn symbol_in(
    from: &Formulas,
    to: &mut Formulas,
    symbol: crate::first_order::Symbol,
    predicate: bool,
) -> Result<crate::first_order::Symbol, Error> {
    let (name, arity) = from.first_order().symbols[symbol.index()].clone();
    to.interner()
        .symbol(&name, arity, predicate)
        .map_err(|fault| Error::Symbol { name, fault })
}

/// Returns a constant of the given clauses, or a new one where they have
/// none.
fn default_constant(set: &mut Clauses) -> Result<FoTermId, Error> {
    let tables = set.formulas.first_order();
    for clause in &set.items {
        for l in &clause.literals {
            let Node::Atom(a) = set.formulas.node(l.atom) else {
                continue;
            };
            let Some(&(_, args)) = tables.atoms.get(a as usize) else {
                continue;
            };
            let mut stack: Vec<FoTermId> = tables.list(args).to_vec();
            while let Some(t) = stack.pop() {
                if let FoTerm::App(_, args) = tables.terms[t.index()] {
                    let list = tables.list(args);
                    if list.is_empty() {
                        return Ok(t);
                    }
                    stack.extend_from_slice(list);
                }
            }
        }
    }
    let name = format!("{}1", normal::fresh_prefix(&set.formulas, "c"));
    let symbol = set
        .formulas
        .interner()
        .symbol(&name, 0, false)
        .map_err(|fault| Error::Symbol { name, fault })?;
    let list = set.formulas.interner().list(&[]);
    Ok(set.formulas.interner().term(FoTerm::App(symbol, list)))
}
