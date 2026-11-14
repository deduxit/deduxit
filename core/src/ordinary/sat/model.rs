// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Finite Kripke models of intuitionistic and minimal logic, and the check
//! that one refutes a sequent, by computing what each world forces.

use super::super::{Logic, Node, NodeId, Sequent};
use crate::Error;
use crate::limits::{Phase, Progress, Refusal};

/// A finite Kripke model: worlds ordered by the reflexive and transitive
/// closure of the edges to the worlds above each, each world with the
/// atoms it forces, and the root, world `0`, below every other. A model
/// refutes an intuitionistic or minimal sequent `Γ ⊢ C` when its root
/// forces every formula of `Γ` and not `C` ([`refutes`](Self::refutes)).
///
/// In minimal logic `⊥` is a proposition like an atom, which a world may
/// force; in intuitionistic logic no world forces it. A model of one
/// world is an assignment of classical logic, the atoms it forces true
/// and the others false, which refutes a classical sequent `Γ ⊢ Δ` when
/// every formula of `Γ` is true and every formula of `Δ` false.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    /// The worlds, the root first and each before the worlds above it.
    pub worlds: Vec<World>,
}

/// A world of a [`Model`].
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct World {
    /// The atoms the world forces, by their index among the atom names of
    /// the sequent's formulas, in ascending order.
    pub atoms: Vec<u32>,
    /// Whether the world forces `⊥`, which only minimal logic allows.
    pub falsity: bool,
    /// The worlds directly above this one, by index, each after it.
    pub above: Vec<u32>,
}

impl Model {
    /// The most cells the check computes: a cell is a world, or an edge
    /// to a world above, times a formula of the sequent's arena. The
    /// forcing table holds a bit per world and formula, so the check
    /// holds 16 MiB at most.
    pub const MOST_CELLS: u64 = 1 << 27;

    /// Checks that the model refutes the sequent in `logic`: its worlds are
    /// ordered (each edge goes to a later world), each world forces the
    /// atoms of every world below it, and `⊥` only in minimal logic; the
    /// root forces every formula left of `⊢` and not the formula right of
    /// it (`⊥` where there is none). Forcing is computed for every formula
    /// of the sequent's arena at every world, the last world first, an
    /// implication or negation (`¬A` read as `A → ⊥`) from what the world
    /// forces and what the worlds directly above it force. The stop is asked once per world.
    ///
    /// In classical logic the model must have one world, which forces no
    /// `⊥`: forcing there is truth under the assignment, and the world
    /// forces no formula right of `⊢`, however many there are.
    ///
    /// # Errors
    ///
    /// [`Error::Countermodel`] naming what fails; [`Error::Succedents`] for
    /// more than one formula right of `⊢` outside classical logic;
    /// [`Refusal::Memory`] where the check would compute more
    /// than [`MOST_CELLS`](Self::MOST_CELLS) cells, and
    /// [`Refusal::Stopped`] when the stop fired, neither a verdict on the
    /// model.
    pub fn refutes(
        &self,
        sequent: &Sequent,
        logic: Logic,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        if logic != Logic::Classical && sequent.right().len() > 1 {
            return Err(Error::Succedents {
                count: sequent.right().len(),
            });
        }
        let flaw = |reason: String| Error::Countermodel { reason };
        let worlds = self.worlds.len();
        if worlds == 0 {
            return Err(flaw("no world".to_owned()));
        }
        if logic == Logic::Classical && worlds > 1 {
            return Err(flaw(format!(
                "{worlds} worlds where classical logic has one"
            )));
        }
        let formulas = sequent.formulas();
        let atoms = formulas.atom_names().len();
        let edges = self
            .worlds
            .iter()
            .map(|w| w.above.len() as u64)
            .sum::<u64>();
        // Each count is below 2^32 and their product below 2^64.
        let cells = (worlds as u64 + edges).saturating_mul(formulas.len() as u64);
        if cells > Self::MOST_CELLS {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Check,
                limit_bytes: Self::MOST_CELLS / 8,
                needed_bytes: Some(cells / 8),
            }));
        }
        for (n, world) in self.worlds.iter().enumerate() {
            if world.atoms.windows(2).any(|w| w[0] >= w[1]) {
                return Err(flaw(format!(
                    "world {n}: its atoms are not in ascending order"
                )));
            }
            if let Some(&a) = world.atoms.iter().find(|&&a| a as usize >= atoms) {
                return Err(flaw(format!("world {n}: no atom has the index {a}")));
            }
            if world.falsity && logic != Logic::Minimal {
                return Err(flaw(format!("world {n} forces ⊥ in {logic} logic")));
            }
            for &up in &world.above {
                let Some(above) = self.worlds.get(up as usize).filter(|_| up as usize > n) else {
                    return Err(flaw(format!(
                        "world {n}: world {up} above it does not come after it"
                    )));
                };
                let monotone = world
                    .atoms
                    .iter()
                    .all(|a| above.atoms.binary_search(a).is_ok())
                    && (above.falsity || !world.falsity);
                if !monotone {
                    return Err(flaw(format!(
                        "world {up} above world {n} does not force all it forces"
                    )));
                }
            }
        }
        // Every edge goes to a later world, so one pass in order finds
        // every world above the root.
        let mut reached = vec![false; worlds];
        reached[0] = true;
        for (n, world) in self.worlds.iter().enumerate() {
            if !reached[n] {
                return Err(flaw(format!("world {n} is not above the root")));
            }
            for &up in &world.above {
                reached[up as usize] = true;
            }
        }
        let width = formulas.len();
        // A bit per world and formula; below `MOST_CELLS` the index fits.
        let mut table = vec![0u64; (worlds * width).div_ceil(64)];
        let get = |table: &[u64], cell: usize| table[cell / 64] >> (cell % 64) & 1 == 1;
        for n in (0..worlds).rev() {
            if stop(Progress::new(Phase::Check, 1, (worlds - n) as u64)) {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::Check,
                }));
            }
            let world = &self.worlds[n];
            for id in 0..width {
                let at = |table: &[u64], k: NodeId| get(table, n * width + k.index());
                // An implication holds where it holds locally and at every
                // world directly above, which holds it at every world above.
                let above = |table: &[u64]| {
                    world
                        .above
                        .iter()
                        .all(|&up| get(table, up as usize * width + id))
                };
                let node = formulas.node(NodeId(id as u32));
                let t = &table[..];
                let value = match node {
                    Node::Atom(a) => world.atoms.binary_search(&a).is_ok(),
                    Node::True => true,
                    Node::False => world.falsity,
                    Node::And(a, b) => at(t, a) && at(t, b),
                    Node::Or(a, b) => at(t, a) || at(t, b),
                    Node::Not(a) => (!at(t, a) || world.falsity) && above(t),
                    Node::Implies(a, b) => (!at(t, a) || at(t, b)) && above(t),
                    Node::Iff(a, b) => at(t, a) == at(t, b) && above(t),
                };
                let cell = n * width + id;
                table[cell / 64] |= u64::from(value) << (cell % 64);
            }
        }
        let forced = |k: NodeId| get(&table, k.index());
        if let Some(&h) = sequent.left().iter().find(|&&h| !forced(h)) {
            return Err(flaw(format!(
                "the root does not force the hypothesis {}",
                formulas.formula(h)
            )));
        }
        if let Some(&c) = sequent.right().iter().find(|&&c| forced(c)) {
            return Err(flaw(format!(
                "the root forces the conclusion {}",
                formulas.formula(c)
            )));
        }
        if sequent.right().is_empty() && self.worlds[0].falsity {
            return Err(flaw("the root forces ⊥".to_owned()));
        }
        Ok(())
    }
}
