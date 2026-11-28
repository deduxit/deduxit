// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! What the SAT encodings of first-order models share: the arena they
//! write with its bounds and the stop, the conjunction and disjunction of
//! a list, the disjunction over the tuples a term's arguments may denote,
//! and the lookup of a variable's element in a chain of bindings. The
//! finite model search and the Kripke model search each hold a `Core`;
//! grounding shares the lookup.

use crate::Error;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::grounding::{ATOM_BYTES, NODE_BYTES, poll_by};
use crate::ordinary::{Formulas, Node, NodeId};

/// The end of a chain of bindings.
const NONE: u32 = u32::MAX;

/// What a term denotes, as an encoding writes it.
pub(super) enum Denotes {
    /// An element, fixed by the bindings.
    Element(u32),
    /// For each element, the formula that says the term denotes it.
    Table(Vec<NodeId>),
}

/// The arena an encoding writes, with what bounds it.
pub(super) struct Core {
    /// The arena written.
    pub(super) formulas: Formulas,
    /// The bytes the atoms and memos hold beside the arena.
    pub(super) held: u64,
    /// The most formulas, `limits.occurrences`.
    pub(super) most: u64,
    /// The memory bound.
    pub(super) memory: u64,
    /// The steps taken, for the stop.
    steps: u64,
}

impl Core {
    /// Returns an empty arena under `limits`.
    pub(super) fn new(limits: &Limits) -> Self {
        Self {
            formulas: Formulas::default(),
            held: 0,
            most: limits.occurrences.unwrap_or(u64::MAX),
            memory: limits.memory_bytes.unwrap_or(u64::MAX),
            steps: 0,
        }
    }

    /// Counts a step of `weight` units, asking the stop as grounding does,
    /// and refuses past the formulas' bound or the memory bound.
    pub(super) fn tick_by(
        &mut self,
        weight: usize,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        poll_by(&mut self.steps, weight, stop)?;
        if self.formulas.len() as u64 > self.most {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: self.most.saturating_add(1),
                limit: self.most,
            }));
        }
        let held = (self.formulas.len() as u64)
            .saturating_mul(NODE_BYTES)
            .saturating_add(self.held);
        if held > self.memory {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Search,
                limit_bytes: self.memory,
                needed_bytes: None,
            }));
        }
        Ok(())
    }

    /// Makes the atom `prefix_number` and charges it with a key of `key`
    /// numbers.
    pub(super) fn atom(&mut self, prefix: &str, number: usize, key: u64) -> Result<NodeId, Error> {
        let id = self.formulas.atom(&format!("{prefix}_{number}"))?;
        self.held = self.held.saturating_add(ATOM_BYTES).saturating_add(4 * key);
        Ok(id)
    }

    /// Returns the conjunction (`and`) or the disjunction of the formulas,
    /// the first leftmost, `⊤` or `⊥` for none.
    pub(super) fn junction(&mut self, and: bool, parts: &[NodeId]) -> Result<NodeId, Error> {
        let Some((&first, rest)) = parts.split_first() else {
            return self
                .formulas
                .add(if and { Node::True } else { Node::False });
        };
        let mut acc = first;
        for &p in rest {
            acc = self.formulas.add(if and {
                Node::And(acc, p)
            } else {
                Node::Or(acc, p)
            })?;
        }
        Ok(acc)
    }
}

/// An encoding that holds a [`Core`] over domains of at most `size`
/// elements.
pub(super) trait Writes {
    /// Returns the arena written.
    fn core(&mut self) -> &mut Core;
    /// Returns the most elements a domain has.
    fn size(&self) -> u32;
}

/// Returns, for the tuples the arguments may denote, the disjunction over
/// them of "the arguments denote the tuple and `make(tuple)`": each fixed
/// argument's element, each other's choice of element, every choice a step.
pub(super) fn over_tuples<W: Writes>(
    w: &mut W,
    args: &[Denotes],
    stop: &mut dyn FnMut(Progress) -> bool,
    mut make: impl FnMut(&mut W, &[u32]) -> Result<NodeId, Error>,
) -> Result<NodeId, Error> {
    let open: Vec<usize> = (0..args.len())
        .filter(|&i| matches!(args[i], Denotes::Table(_)))
        .collect();
    let mut tuple: Vec<u32> = args
        .iter()
        .map(|a| match a {
            Denotes::Element(e) => *e,
            Denotes::Table(_) => 0,
        })
        .collect();
    if open.is_empty() {
        return make(w, &tuple);
    }
    // An odometer over the open arguments' elements.
    let size = w.size();
    let mut choice = vec![0u32; open.len()];
    let mut disjuncts = Vec::new();
    loop {
        w.core().tick_by(1, stop)?;
        let mut conjuncts = Vec::with_capacity(open.len() + 1);
        for (k, &i) in open.iter().enumerate() {
            tuple[i] = choice[k];
            if let Denotes::Table(table) = &args[i] {
                conjuncts.push(table[choice[k] as usize]);
            }
        }
        conjuncts.push(make(w, &tuple)?);
        let core = w.core();
        disjuncts.push(core.junction(true, &conjuncts)?);
        core.held = core.held.saturating_add(4);
        let mut k = 0;
        loop {
            if k == choice.len() {
                return w.core().junction(false, &disjuncts);
            }
            choice[k] += 1;
            if choice[k] < size {
                break;
            }
            choice[k] = 0;
            k += 1;
        }
    }
}

/// Returns the element bound to the variable `x` in the bindings from `at`
/// back, each a variable, its element and the binding before it.
pub(in crate::ordinary) fn lookup(bindings: &[(u32, u32, u32)], mut at: u32, x: u32) -> u32 {
    while at != NONE {
        let (y, e, before) = bindings[at as usize];
        if y == x {
            return e;
        }
        at = before;
    }
    unreachable!("every variable is bound in a closed formula")
}
