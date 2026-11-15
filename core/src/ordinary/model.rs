// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Ordinary logic's check of a finite Kripke model: whether it is a model
//! of the logic, what each world forces, and whether a sequent holds or
//! fails there.

use super::{Logic, Node, NodeId, Sequent};
use crate::Error;
use crate::kripke::Model;
use crate::limits::{Phase, Progress, Refusal};
use std::collections::BTreeSet;

/// The most cells a check computes: a cell is a world, or an arrow to a
/// world above, times a formula of the sequent's arena. The forcing table
/// holds a bit per world and formula, so a check holds 16 MiB of it at
/// most; a model past the bound is not checked, and a countermodel past
/// it is reported as too large to show.
pub const MODEL_CELLS: u64 = 1 << 27;

/// A Kripke model that refutes an ordinary sequent, with the sequent: the
/// evidence of a "not valid".
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Countermodel {
    /// The sequent refuted.
    pub sequent: Sequent,
    /// The model, whose world forces every formula left of the sequent's
    /// `⊢` and not the formula right of it.
    pub model: Model,
}

impl Countermodel {
    /// Returns the countermodel of this sequent and model, unchecked.
    pub const fn new(sequent: Sequent, model: Model) -> Self {
        Self { sequent, model }
    }

    /// Checks that the model refutes the sequent in `logic`, as
    /// [`Model::refutes`] does.
    ///
    /// # Errors
    ///
    /// As [`Model::refutes`].
    pub fn check(&self, logic: Logic, stop: impl FnMut(Progress) -> bool) -> Result<(), Error> {
        self.model.refutes(&self.sequent, logic, stop)
    }
}

/// What each world of a model forces, for every formula of a sequent's
/// arena, as [`Model::forcing`] computes it.
#[derive(Clone, Debug)]
pub struct Forcing {
    /// The formulas of the sequent's arena, per world.
    width: usize,
    /// A bit per world and formula.
    table: Vec<u64>,
    /// Per world, whether the sequent holds there: at every world at or
    /// above it that forces every hypothesis, the conclusion is forced.
    holds: Vec<bool>,
    /// Per world, whether it forces every hypothesis and not the
    /// conclusion.
    fails: Vec<bool>,
    /// The arrows of each world, for the search of a failing world above.
    arrows: Vec<Vec<u32>>,
}

impl Forcing {
    /// Returns whether `world` forces the formula `id` of the sequent's
    /// arena.
    ///
    /// # Panics
    ///
    /// For a world or a formula outside the model or the arena.
    pub fn forces(&self, world: u32, id: NodeId) -> bool {
        assert!(id.index() < self.width, "a formula of the sequent's arena");
        let cell = world as usize * self.width + id.index();
        self.table[cell / 64] >> (cell % 64) & 1 == 1
    }

    /// Returns what `world` forces of every formula of the sequent's
    /// arena, each operand before its formula.
    ///
    /// # Panics
    ///
    /// For a world outside the model.
    pub fn at(&self, world: u32) -> impl Iterator<Item = (NodeId, bool)> + '_ {
        (0..self.width).map(move |id| {
            let id = NodeId(id as u32);
            (id, self.forces(world, id))
        })
    }

    /// Returns whether the sequent holds at `world`: every world at or
    /// above it that forces every formula left of `⊢` forces the formula
    /// right of it (classically, one of them).
    ///
    /// # Panics
    ///
    /// For a world outside the model.
    pub fn holds(&self, world: u32) -> bool {
        self.holds[world as usize]
    }

    /// Returns a world at or above `world` where the sequent fails, one
    /// that forces every formula left of `⊢` and not the formula right of
    /// it, or `None` where the sequent holds at `world`.
    ///
    /// # Panics
    ///
    /// For a world outside the model.
    pub fn failing(&self, world: u32) -> Option<u32> {
        let mut seen = vec![false; self.fails.len()];
        let mut stack = vec![world];
        while let Some(w) = stack.pop() {
            if std::mem::replace(&mut seen[w as usize], true) {
                continue;
            }
            if self.fails[w as usize] {
                return Some(w);
            }
            stack.extend(&self.arrows[w as usize]);
        }
        None
    }
}

impl Model {
    /// Checks that the model is one of `logic` and computes what each of
    /// its worlds forces, for every formula of the sequent's arena.
    ///
    /// A model of the logic: at least one world, the model's world among
    /// them; every arrow to a world of the model, and no arrows round
    /// (the order is a partial order, worlds above each other being one
    /// world); every world forcing the atoms, and `⊥`, of every world
    /// below it; `⊥` only in minimal logic; in classical logic one world.
    /// An atom the sequent does not name changes nothing it forces.
    ///
    /// Forcing is computed world by world, the worlds above first: an
    /// atom where the world forces it, `⊥` where it forces it (minimal
    /// logic), `∧` and `∨` from their parts there, an implication, a
    /// negation (`¬A` read as `A → ⊥`) or an equivalence where it holds
    /// there and at every world directly above, which holds it at every
    /// world above. The stop is asked once per world.
    ///
    /// # Errors
    ///
    /// [`Error::Countermodel`] for a model that is not one of the logic,
    /// naming what fails; [`Error::Succedents`] for more than one formula
    /// right of `⊢` outside classical logic; [`Refusal::Memory`] where the
    /// check would compute more than [`MODEL_CELLS`] cells, and
    /// [`Refusal::Stopped`] when the stop fired, neither a verdict on the
    /// model.
    pub fn forcing(
        &self,
        sequent: &Sequent,
        logic: Logic,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Forcing, Error> {
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
        if self.world as usize >= worlds {
            return Err(flaw(format!(
                "the model's world {} is not one of its {worlds} worlds",
                self.world
            )));
        }
        if logic == Logic::Classical && worlds > 1 {
            return Err(flaw(format!(
                "{worlds} worlds where classical logic has one"
            )));
        }
        let formulas = sequent.formulas();
        let width = formulas.len();
        let arrows: u64 = self.worlds.iter().map(|w| w.arrows.len() as u64).sum();
        // Each count is below 2^64 saturating, so the product saturates.
        let cells = (worlds as u64)
            .saturating_add(arrows)
            .saturating_mul(width as u64);
        if cells > MODEL_CELLS {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Check,
                limit_bytes: MODEL_CELLS / 8,
                needed_bytes: Some(cells / 8),
            }));
        }
        // Each world's atoms as a set of names, and as the sequent's atom
        // indices it forces.
        let index: std::collections::HashMap<&str, u32> = formulas
            .atom_names()
            .iter()
            .enumerate()
            .map(|(i, name)| (name.as_str(), i as u32))
            .collect();
        let names: Vec<BTreeSet<&str>> = self
            .worlds
            .iter()
            .map(|w| w.atoms.iter().map(String::as_str).collect())
            .collect();
        let known: Vec<Vec<u32>> = names
            .iter()
            .map(|set| {
                let mut atoms: Vec<u32> =
                    set.iter().filter_map(|n| index.get(n).copied()).collect();
                atoms.sort_unstable();
                atoms
            })
            .collect();
        for (n, world) in self.worlds.iter().enumerate() {
            if world.falsity && logic != Logic::Minimal {
                return Err(flaw(format!("world {n} forces ⊥ in {logic} logic")));
            }
            for &up in &world.arrows {
                let Some(above) = self.worlds.get(up as usize) else {
                    return Err(flaw(format!(
                        "world {n} has an arrow to world {up}, which the model does not have"
                    )));
                };
                if !names[n].is_subset(&names[up as usize]) || (world.falsity && !above.falsity) {
                    return Err(flaw(format!(
                        "world {up} above world {n} does not force all it forces"
                    )));
                }
            }
        }
        // The worlds in an order where each comes after every world above
        // it (Kahn's algorithm on the arrows reversed); a world left out
        // lies on arrows that go round.
        let mut pending: Vec<u32> = self
            .worlds
            .iter()
            .map(|w| u32::try_from(w.arrows.len()).unwrap_or(u32::MAX))
            .collect();
        let mut into: Vec<Vec<u32>> = vec![Vec::new(); worlds];
        for (n, world) in self.worlds.iter().enumerate() {
            for &up in &world.arrows {
                into[up as usize].push(n as u32);
            }
        }
        let mut ready: Vec<u32> = (0..worlds as u32)
            .filter(|&n| pending[n as usize] == 0)
            .collect();
        let mut order = Vec::with_capacity(worlds);
        while let Some(n) = ready.pop() {
            order.push(n);
            for &low in &into[n as usize] {
                pending[low as usize] -= 1;
                if pending[low as usize] == 0 {
                    ready.push(low);
                }
            }
        }
        if order.len() < worlds {
            let round = (0..worlds).find(|&n| pending[n] > 0).unwrap_or(0);
            return Err(flaw(format!(
                "the arrows go round through world {round}: worlds above each other are one world"
            )));
        }
        // A bit per world and formula; below `MODEL_CELLS` the index fits.
        let mut table = vec![0u64; (worlds * width).div_ceil(64)];
        let get = |table: &[u64], cell: usize| table[cell / 64] >> (cell % 64) & 1 == 1;
        for (done, &n) in order.iter().enumerate() {
            if stop(Progress::new(Phase::Check, 1, done as u64 + 1)) {
                return Err(Error::Refused(Refusal::Stopped {
                    phase: Phase::Check,
                }));
            }
            let n = n as usize;
            let world = &self.worlds[n];
            for id in 0..width {
                let at = |table: &[u64], k: NodeId| get(table, n * width + k.index());
                // An implication holds where it holds locally and at every
                // world directly above, which holds it at every world above.
                let above = |table: &[u64]| {
                    world
                        .arrows
                        .iter()
                        .all(|&up| get(table, up as usize * width + id))
                };
                let t = &table[..];
                let value = match formulas.node(NodeId(id as u32)) {
                    Node::Atom(a) => known[n].binary_search(&a).is_ok(),
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
        let forced = |n: usize, k: NodeId| get(&table, n * width + k.index());
        let fails: Vec<bool> = (0..worlds)
            .map(|n| {
                sequent.left().iter().all(|&h| forced(n, h))
                    && match sequent.right() {
                        [] => !self.worlds[n].falsity,
                        right => right.iter().all(|&c| !forced(n, c)),
                    }
            })
            .collect();
        let mut holds = vec![true; worlds];
        for &n in &order {
            let n = n as usize;
            holds[n] = !fails[n] && self.worlds[n].arrows.iter().all(|&up| holds[up as usize]);
        }
        Ok(Forcing {
            width,
            table,
            holds,
            fails,
            arrows: self.worlds.iter().map(|w| w.arrows.clone()).collect(),
        })
    }

    /// Checks that the model refutes the sequent in `logic`: it is a model
    /// of the logic ([`forcing`](Self::forcing)), and its world forces
    /// every formula left of `⊢` and not the formula right of it (`⊥`
    /// where there is none; classically, none of them).
    ///
    /// # Errors
    ///
    /// As [`forcing`](Self::forcing), and [`Error::Countermodel`] naming
    /// the hypothesis the model's world does not force or the conclusion
    /// it forces.
    pub fn refutes(
        &self,
        sequent: &Sequent,
        logic: Logic,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let forcing = self.forcing(sequent, logic, stop)?;
        let formulas = sequent.formulas();
        let world = self.world;
        let flaw = |reason: String| Error::Countermodel { reason };
        if let Some(&h) = sequent.left().iter().find(|&&h| !forcing.forces(world, h)) {
            return Err(flaw(format!(
                "world {world} does not force the hypothesis {}",
                formulas.formula(h)
            )));
        }
        if let Some(&c) = sequent.right().iter().find(|&&c| forcing.forces(world, c)) {
            return Err(flaw(format!(
                "world {world} forces the conclusion {}",
                formulas.formula(c)
            )));
        }
        if sequent.right().is_empty() && self.worlds[world as usize].falsity {
            return Err(flaw(format!("world {world} forces ⊥")));
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::{Logic, Sequent};
    use crate::Error;
    use crate::kripke::{Model, World};

    /// Returns a world of these atoms with arrows to these worlds.
    fn world(atoms: &[&str], arrows: &[u32]) -> World {
        World::new(
            atoms.iter().map(|&a| a.to_owned()).collect(),
            false,
            arrows.to_vec(),
        )
    }

    /// The excluded middle fails at the root of two worlds and holds at
    /// the world above, whichever order the worlds are given in; minimal
    /// logic reads `⊥` as an atom some worlds force, monotone; a model of
    /// another logic is refused, naming why; the model reads back.
    #[test]
    fn forcing_says_where_a_sequent_holds_and_fails() {
        let sequent: Sequent = "|- a \\/ ~a".parse().unwrap();
        let logic = Logic::Intuitionistic;
        // The world above first: the order is the arrows', not the indices'.
        let model = Model::new(vec![world(&["a"], &[]), world(&[], &[0])], 1);
        let forcing = model.forcing(&sequent, logic, |_| false).unwrap();
        assert!(!forcing.holds(1) && forcing.holds(0));
        assert_eq!(forcing.failing(1), Some(1));
        assert_eq!(forcing.failing(0), None);
        assert_eq!(model.refutes(&sequent, logic, |_| false), Ok(()));
        assert_eq!(
            model.to_string(),
            "world 0: forces a\nworld 1 (the model's world): forces nothing; below 0"
        );

        // `¬a` holds in minimal logic where no world above forces `a`,
        // even where a world forces `⊥`, and falsity must grow upward.
        let negation: Sequent = "~a".parse().unwrap();
        let mut minimal = Model::new(vec![world(&[], &[1]), world(&[], &[])], 0);
        minimal.worlds[1].falsity = true;
        let forcing = minimal
            .forcing(&negation, Logic::Minimal, |_| false)
            .unwrap();
        assert!(forcing.holds(0));
        minimal.worlds[0].falsity = true;
        minimal.worlds[1].falsity = false;
        let Err(Error::Countermodel { reason }) =
            minimal.forcing(&negation, Logic::Minimal, |_| false)
        else {
            panic!("falsity that shrinks upward accepted");
        };
        assert!(reason.contains("does not force all it forces"), "{reason}");
        let Err(Error::Countermodel { reason }) = minimal.forcing(&negation, logic, |_| false)
        else {
            panic!("⊥ forced in intuitionistic logic");
        };
        assert!(reason.contains("forces ⊥"), "{reason}");

        #[cfg(feature = "serialize")]
        {
            let json = serde_json::to_string(&model).unwrap();
            assert_eq!(
                json,
                r#"{"version":13,"worlds":[{"atoms":["a"]},{"atoms":[],"arrows":[0]}],"world":1}"#
            );
            let read: Model = serde_json::from_str(&json).unwrap();
            assert_eq!(read, model);
            let keyword = r#"{"worlds":[{"atoms":["par"]}]}"#;
            assert!(serde_json::from_str::<Model>(keyword).is_err());
        }
    }
}
