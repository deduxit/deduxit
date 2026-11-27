// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A first-order countermodel laid out for the drawings: per world its
//! domain, the values of its constants and function symbols, its true
//! facts, `⊥` where it forces it, and the worlds directly above it. A
//! finite structure is the model of one world. Each world shows at most
//! sixteen values and sixteen facts ([`SHOWN`](crate::export::models::SHOWN))
//! and counts the rest, so a drawing
//! stays readable and its size grows with the worlds alone.

use crate::Error;
use crate::hash::HashMap;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::{
    FiniteCountermodel, KripkeCountermodel, KripkeStructure, Logic, Node, NodeId, Sequent,
};

/// The values, and the facts, a world shows at most.
pub(crate) const SHOWN: usize = 16;

/// The bytes a drawing is estimated at per byte of the text it shows, the
/// markup of every target included, and per world besides.
const PER_BYTE: u64 = 16;

/// The bytes a world's drawing is estimated at besides its text.
const PER_WORLD: u64 = 800;

/// A model laid out.
pub(crate) struct Laid {
    /// The drawing's title: `A countermodel of S`.
    #[cfg_attr(
        not(feature = "svg"),
        expect(dead_code, reason = "only the SVG drawing has a title")
    )]
    pub(crate) title: String,
    /// The sentence on the world where the sequent fails, and why.
    #[cfg_attr(
        not(feature = "svg"),
        expect(dead_code, reason = "only the SVG drawing has a description")
    )]
    pub(crate) last: String,
    /// The worlds.
    pub(crate) worlds: Vec<LaidWorld>,
    /// The world where the sequent fails.
    pub(crate) world: u32,
    /// Whether the model is a Kripke model, not a finite structure.
    pub(crate) kripke: bool,
}

/// A world laid out.
pub(crate) struct LaidWorld {
    /// The number of elements: the domain is `0 .. size`.
    pub(crate) size: u32,
    /// The worlds directly above, each once, in order.
    pub(crate) arrows: Vec<u32>,
    /// The values shown: a symbol, its arguments and its value; a constant
    /// has no argument.
    pub(crate) values: Vec<(String, Vec<u32>, u32)>,
    /// The values left out.
    pub(crate) more_values: usize,
    /// The true facts shown: a predicate and its arguments.
    pub(crate) facts: Vec<(String, Vec<u32>)>,
    /// The facts left out.
    pub(crate) more_facts: usize,
    /// Whether the world forces `⊥`.
    pub(crate) falsity: bool,
}

/// Lays out a finite structure, after checking that it refutes its
/// sequent in `logic` under `limits`.
pub(crate) fn structure(
    countermodel: &FiniteCountermodel,
    logic: Logic,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Laid, Error> {
    countermodel.check(logic, limits, &mut *stop)?;
    let sequent = &countermodel.sequent;
    let model = KripkeStructure::from(countermodel.structure.clone());
    let worlds = laid(sequent, &model, limits, stop)?;
    Ok(Laid {
        title: format!("A countermodel of {sequent}"),
        last: format!(
            "The structure makes every formula left of ⊢ true and every formula right of it \
             false, so {sequent} is not valid in {logic} logic."
        ),
        worlds,
        world: 0,
        kripke: false,
    })
}

/// Lays out a Kripke model with growing domains, after checking that it
/// refutes its sequent in `logic` under `limits`.
pub(crate) fn kripke(
    countermodel: &KripkeCountermodel,
    logic: Logic,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Laid, Error> {
    countermodel.check(logic, limits, &mut *stop)?;
    let sequent = &countermodel.sequent;
    let model = &countermodel.model;
    let worlds = laid(sequent, model, limits, stop)?;
    let w = model.world;
    let last = last(sequent, logic, w);
    Ok(Laid {
        title: format!("A countermodel of {sequent}"),
        last,
        worlds,
        world: w,
        kripke: true,
    })
}

/// Returns the sentence that ends a countermodel's description: why
/// `sequent` fails at the model's world `w` in `logic`.
pub(crate) fn last(sequent: &Sequent, logic: Logic, w: u32) -> String {
    match (logic, sequent.right().is_empty()) {
        (Logic::Classical, _) => format!(
            "World {w} makes every formula left of ⊢ true and every formula right of it \
             false, so {sequent} is not valid in classical logic."
        ),
        (Logic::Intuitionistic | Logic::Minimal, true) => format!(
            "World {w} forces each formula left of ⊢ and not ⊥, so {sequent} is not valid in \
             {logic} logic."
        ),
        (Logic::Intuitionistic | Logic::Minimal, false) => format!(
            "World {w} forces each formula left of ⊢ and not the one right of it, so \
             {sequent} is not valid in {logic} logic."
        ),
    }
}

/// Returns the bytes of a sequent's text, bottom up over its arena,
/// saturating: a shared formula is written each time it occurs.
pub(crate) fn sequent_bytes(sequent: &Sequent) -> u64 {
    let formulas = sequent.formulas();
    let binder = formulas
        .first_order()
        .binders
        .iter()
        .map(|(_, name)| name.len() as u64)
        .max()
        .unwrap_or(0);
    let mut bytes: Vec<u64> = Vec::with_capacity(formulas.len());
    for &node in formulas.nodes() {
        let of = |k: NodeId| bytes[k.index()];
        let b = match node {
            Node::Atom(a) => (formulas.atom_names()[a as usize].len() as u64).saturating_add(2),
            Node::True | Node::False => 3,
            Node::Not(a) => of(a).saturating_add(4),
            Node::Forall(a) | Node::Exists(a) => of(a).saturating_add(binder).saturating_add(8),
            Node::And(a, b) | Node::Or(a, b) | Node::Implies(a, b) | Node::Iff(a, b) => {
                of(a).saturating_add(of(b)).saturating_add(9)
            }
        };
        bytes.push(b);
    }
    sequent
        .left()
        .iter()
        .chain(sequent.right())
        .fold(0u64, |sum, id| sum.saturating_add(bytes[id.index()]))
}

/// Returns the worlds of a checked model laid out, refused past
/// `limits.derivation_bytes` by their estimate as they are laid out, and
/// before the sequent's text is written: the estimate grows with each
/// value, fact and arrow before it is built, so a refused drawing holds
/// no more than the bound's worth.
fn laid(
    sequent: &Sequent,
    model: &KripkeStructure,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Vec<LaidWorld>, Error> {
    let tables = sequent.formulas().first_order();
    let arities: HashMap<&str, u32> = tables
        .symbols
        .iter()
        .map(|(symbol, arity)| (symbol.as_str(), *arity))
        .collect();
    // A table is shown where it has one value per tuple of its symbol's
    // arguments: the sequent's arity, or for a symbol the sequent does not
    // name the arity the table's length gives. The check holds only the
    // tables the sequent applies to that shape.
    let arity = |name: &str, entries: usize, size: u32| -> Option<usize> {
        let size = u64::from(size);
        let shape = |k: u32| size.checked_pow(k) == Some(entries as u64);
        match arities.get(name) {
            Some(&k) => (k as usize <= SHOWN && shape(k)).then_some(k as usize),
            None => (1..=SHOWN as u32).find(|&k| shape(k)).map(|k| k as usize),
        }
    };
    let mut polls = 0u64;
    let mut estimate = sequent_bytes(sequent).saturating_mul(PER_BYTE);
    let mut charge = |bytes: u64| -> Result<(), Error> {
        estimate = estimate.saturating_add(bytes.saturating_mul(PER_BYTE));
        match limits.derivation_bytes {
            Some(limit) if estimate > limit => Err(Refusal::Output {
                what: "drawing",
                estimate_bytes: estimate,
                limit_bytes: limit,
                least_bytes: None,
            }
            .into()),
            _ => Ok(()),
        }
    };
    charge(0)?;
    let mut worlds: Vec<LaidWorld> = Vec::with_capacity(model.worlds.len().min(1024));
    for world in &model.worlds {
        polls += 1;
        if stop(Progress::new(Phase::Write, 1, polls)) {
            return Err(Refusal::stopped(Phase::Write).into());
        }
        charge(PER_WORLD / PER_BYTE + 12 * world.arrows.len() as u64)?;
        let mut arrows = world.arrows.clone();
        arrows.sort_unstable();
        arrows.dedup();
        let last_of: HashMap<&str, usize> = world
            .functions
            .iter()
            .enumerate()
            .map(|(at, (name, _))| (name.as_str(), at))
            .collect();
        let mut values: Vec<(String, Vec<u32>, u32)> = Vec::new();
        let mut more_values = 0usize;
        for (name, e) in model.constants.iter().take(SHOWN) {
            charge(name.len() as u64 + 12)?;
            values.push((name.clone(), Vec::new(), *e));
        }
        more_values += model.constants.len().saturating_sub(SHOWN);
        for (at, (name, table)) in world.functions.iter().enumerate() {
            // A symbol listed twice has its last table, as in the check.
            if last_of.get(name.as_str()) != Some(&at) {
                continue;
            }
            // A table of another shape, or of a symbol of more arguments
            // than a row can show, is counted, not shown.
            let Some(k) = arity(name, table.len(), world.size) else {
                more_values += table.len();
                continue;
            };
            for (index, &value) in table.iter().enumerate() {
                if values.len() < SHOWN {
                    charge((name.len() as u64).saturating_add(12 * (k as u64 + 1)))?;
                    values.push((name.clone(), tuple(index, k, world.size), value));
                } else {
                    more_values += table.len() - index;
                    break;
                }
            }
        }
        let mut facts: Vec<(String, Vec<u32>)> = Vec::new();
        let mut more_facts = 0usize;
        for (name, tuples) in &world.predicates {
            let room = SHOWN - facts.len();
            for tuple in tuples.iter().take(room) {
                // A tuple of more elements than a row can show is cut.
                let shown = &tuple[..tuple.len().min(SHOWN)];
                charge((name.len() as u64).saturating_add(12 * shown.len() as u64))?;
                facts.push((name.clone(), shown.to_vec()));
            }
            more_facts += tuples.len().saturating_sub(room);
        }
        worlds.push(LaidWorld {
            size: world.size,
            arrows,
            values,
            more_values,
            facts,
            more_facts,
            falsity: world.falsity,
        });
    }
    Ok(worlds)
}

/// Returns the argument tuple at `index` of a table of `arity` arguments
/// over `size` elements, the first argument the most significant.
fn tuple(mut index: usize, arity: usize, size: u32) -> Vec<u32> {
    let size = size.max(1) as usize;
    let mut out = vec![0u32; arity];
    for slot in out.iter_mut().rev() {
        *slot = (index % size) as u32;
        index /= size;
    }
    out
}

/// Returns the elements of a domain of `size` written as a set: `{0, 1}`.
pub(crate) fn domain(size: u32) -> String {
    let elements: Vec<String> = (0..size.min(SHOWN as u32)).map(|e| e.to_string()).collect();
    let more = if size as usize > SHOWN {
        format!(", ..., {}", size - 1)
    } else {
        String::new()
    };
    format!("{{{}{more}}}", elements.join(", "))
}

/// Returns a list of numbers separated by commas: `0, 1`.
pub(crate) fn numbers(list: &[u32]) -> String {
    let words: Vec<String> = list.iter().map(u32::to_string).collect();
    words.join(", ")
}
