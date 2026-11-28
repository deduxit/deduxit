// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The check of a finite Kripke model ([`kripke`](crate::kripke): worlds
//! joined by arrows to the worlds above them, each forcing some atoms)
//! against an ordinary sequent: whether it is a model of the logic, what
//! each world forces, and whether the sequent holds or fails there.

use super::{Logic, Node, NodeId, Sequent};
use crate::Error;
use crate::kripke::Model;
use crate::limits::{Limits, Phase, Progress, Refusal};

/// The most cells a check computes: a cell is a world, or an arrow to a
/// world above, times a formula of the sequent's arena. The forcing table
/// holds a bit per world and formula, so a check holds at most 16 MiB of
/// it. A model past the bound is not checked, and an outcome drops a
/// countermodel past it as too large to show.
pub const MODEL_CELLS: u64 = 1 << 27;

/// The bytes the check holds per world besides the table: its atoms' and
/// arrows' vectors, the arrows into it, its place in the order, its
/// pending count and two flags.
const WORLD_BYTES: u64 = 4 * 24 + 4 + 8 + 2;

/// The bytes the check holds per atom a world forces: its index, and at
/// most one entry of the map of names (a name and an index, with the
/// map's own share).
const ATOM_BYTES: u64 = 4 + 32;

/// A Kripke model that refutes an ordinary sequent, with the sequent: the
/// evidence of a "not valid".
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Countermodel {
    /// The sequent refuted.
    pub sequent: Sequent,
    /// The model. Its world forces every formula left of the sequent's
    /// `⊢` and none right of it, as [`Model::refutes`] checks.
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
    pub fn check(
        &self,
        logic: Logic,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        self.model.refutes(&self.sequent, logic, limits, stop)
    }

    /// Returns the countermodel an engine found, checked: `None` where it
    /// is too large to check (past [`MODEL_CELLS`] or
    /// `limits.memory_bytes`).
    ///
    /// # Errors
    ///
    /// [`Error::Countermodel`] where the model does not refute the
    /// sequent, a defect of the engine; [`Refusal::Stopped`] when the stop
    /// fired.
    pub(crate) fn found(
        sequent: Sequent,
        model: Model,
        logic: Logic,
        limits: &Limits,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Option<Self>, Error> {
        match model.refutes(&sequent, logic, limits, stop) {
            Ok(()) => Ok(Some(Self::new(sequent, model))),
            Err(Error::Refused(Refusal::Memory { .. })) => Ok(None),
            Err(Error::Model { reason }) => Err(Error::Countermodel { reason }),
            Err(error) => Err(error),
        }
    }
}

/// What each world of a model forces, for every formula of a sequent's
/// arena, as [`Model::forcing`] computes it.
#[derive(Clone, Debug)]
pub struct Forcing {
    /// The number of formulas in the sequent's arena: the bits per world.
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
        assert!((world as usize) < self.holds.len(), "a world of the model");
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
    /// right of it (`⊥` where there is none; classically, one of them).
    ///
    /// # Panics
    ///
    /// For a world outside the model.
    pub fn holds(&self, world: u32) -> bool {
        self.holds[world as usize]
    }

    /// Returns a world where the sequent fails, one that forces every
    /// formula left of `⊢` and not the formula right of it, or `None` where
    /// it holds at every world.
    pub fn failing_anywhere(&self) -> Option<u32> {
        self.fails.iter().position(|&fails| fails).map(|w| w as u32)
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
    /// A model of the logic has:
    /// - at least one world, the model's world among them;
    /// - every arrow to a world of the model, and no cycle of arrows (the
    ///   order is a partial order: two worlds each above the other would
    ///   be one world);
    /// - every world forcing the atoms, and `⊥`, of every world below it;
    /// - `⊥` forced only in minimal logic;
    /// - one world in classical logic.
    ///
    /// An atom the sequent does not name changes nothing the sequent's
    /// formulas force, though it must grow along the arrows as the others
    /// do. An arrow written twice is one arrow.
    ///
    /// Forcing is computed world by world, the worlds above first. An atom
    /// is forced where the world forces it, and `⊥` where the world forces
    /// it (minimal logic). `∧` and `∨` follow from their parts there. An
    /// implication, a negation (`¬A` read as `A → ⊥`) or an equivalence is
    /// forced where it holds there and is forced at every world directly
    /// above, which then forces it at every world above. The stop is asked
    /// once per world in each pass over the worlds. Besides the table
    /// ([`MODEL_CELLS`]) the check holds a few words per world, arrow and
    /// atom, which `limits.memory_bytes` bounds.
    ///
    /// # Errors
    ///
    /// [`Error::Model`] for a model that is not one of the logic, naming
    /// what fails; [`Error::Succedents`] for more than one formula right
    /// of `⊢` outside classical logic; [`Refusal::Memory`] where the check
    /// would compute more than [`MODEL_CELLS`] cells or hold more than
    /// `limits.memory_bytes`, and [`Refusal::Stopped`] when the stop
    /// fired, neither a verdict on the model.
    pub fn forcing(
        &self,
        sequent: &Sequent,
        logic: Logic,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Forcing, Error> {
        self.forcing_with(sequent, logic, limits, &mut stop)
    }

    /// [`forcing`](Self::forcing) with the stop as a trait object, so that
    /// the check is compiled once, not once per caller's closure.
    fn forcing_with(
        &self,
        sequent: &Sequent,
        logic: Logic,
        limits: &Limits,
        mut stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Forcing, Error> {
        sequent.formulas().propositional("a Kripke model's check")?;
        if logic != Logic::Classical && sequent.right().len() > 1 {
            return Err(Error::Succedents {
                count: sequent.right().len(),
            });
        }
        let flaw = |reason: String| Error::Model { reason };
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
        let (arrows, atoms) = self.worlds.iter().fold((0u64, 0u64), |(r, a), w| {
            (
                r.saturating_add(w.arrows.len() as u64),
                a.saturating_add(w.atoms.len() as u64),
            )
        });
        // Each count is below 2^64 saturating, so the product saturates.
        let cells = (worlds as u64)
            .saturating_add(arrows)
            .saturating_mul(width as u64);
        let refused = |limit_bytes: u64, needed: u64| {
            Err(Error::Refused(Refusal::Memory {
                phase: Phase::Check,
                limit_bytes,
                needed_bytes: Some(needed),
            }))
        };
        if cells > MODEL_CELLS {
            return refused(MODEL_CELLS / 8, cells / 8);
        }
        // What the passes below hold besides the table: `WORLD_BYTES` per
        // world, two indices per arrow and `ATOM_BYTES` per atom.
        let held = (worlds as u64)
            .saturating_mul(WORLD_BYTES)
            .saturating_add(arrows.saturating_mul(8))
            .saturating_add(atoms.saturating_mul(ATOM_BYTES))
            .saturating_add(cells / 8);
        if let Some(limit) = limits.memory_bytes
            && held > limit
        {
            return refused(limit, held);
        }
        let mut polls = 0u64;
        let mut poll = |stop: &mut dyn FnMut(Progress) -> bool| {
            polls += 1;
            if stop(Progress::new(Phase::Check, 1, polls)) {
                Err(Error::Refused(Refusal::stopped(Phase::Check)))
            } else {
                Ok(())
            }
        };
        // Each world's atoms as sorted indices: the sequent's atoms keep
        // their own, below `named`, and the other names follow.
        let named = formulas.atom_names().len();
        let mut index: crate::hash::HashMap<&str, u32> = formulas
            .atom_names()
            .iter()
            .enumerate()
            .map(|(i, name)| (name.as_str(), i as u32))
            .collect();
        let mut forced: Vec<Vec<u32>> = Vec::with_capacity(worlds);
        for world in &self.worlds {
            poll(&mut stop)?;
            let mut ids: Vec<u32> = world
                .atoms
                .iter()
                .map(|name| {
                    let next = u32::try_from(index.len()).unwrap_or(u32::MAX);
                    *index.entry(name.as_str()).or_insert(next)
                })
                .collect();
            ids.sort_unstable();
            ids.dedup();
            forced.push(ids);
        }
        // Each world's arrows, each once, checked: to a world of the
        // model that forces all this one forces. A world's atoms are
        // looked up in each world directly above it, between two polls.
        let mut above: Vec<Vec<u32>> = Vec::with_capacity(worlds);
        for (n, world) in self.worlds.iter().enumerate() {
            poll(&mut stop)?;
            if world.falsity && logic != Logic::Minimal {
                return Err(flaw(format!("world {n} forces ⊥ in {logic} logic")));
            }
            let mut ups = world.arrows.clone();
            ups.sort_unstable();
            ups.dedup();
            for &up in &ups {
                let Some(higher) = self.worlds.get(up as usize) else {
                    return Err(flaw(format!(
                        "world {n} has an arrow to world {up}, which the model does not have"
                    )));
                };
                let superset = &forced[up as usize];
                if forced[n].iter().any(|a| superset.binary_search(a).is_err())
                    || (world.falsity && !higher.falsity)
                {
                    return Err(flaw(format!(
                        "world {up} above world {n} does not force all it forces"
                    )));
                }
            }
            above.push(ups);
        }
        // The worlds in an order where each comes after every world above
        // it (Kahn's algorithm on the arrows reversed); a world left out
        // lies on arrows that go round.
        let mut pending: Vec<usize> = above.iter().map(Vec::len).collect();
        let mut into: Vec<Vec<u32>> = vec![Vec::new(); worlds];
        for (n, ups) in above.iter().enumerate() {
            for &up in ups {
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
        drop(into);
        if order.len() < worlds {
            let round = (0..worlds).find(|&n| pending[n] > 0).unwrap_or(0);
            return Err(flaw(format!(
                "the arrows go round through world {round}: worlds above each other are one world"
            )));
        }
        // A bit per world and formula; below `MODEL_CELLS` the index fits.
        let mut table = vec![0u64; (worlds * width).div_ceil(64)];
        let get = |table: &[u64], cell: usize| table[cell / 64] >> (cell % 64) & 1 == 1;
        for &n in &order {
            poll(&mut stop)?;
            let n = n as usize;
            let falsity = self.worlds[n].falsity;
            // The sequent's atoms the world forces: a prefix of its own.
            let known = &forced[n][..forced[n].partition_point(|&a| (a as usize) < named)];
            for id in 0..width {
                let at = |table: &[u64], k: NodeId| get(table, n * width + k.index());
                // An implication holds where it holds locally and at every
                // world directly above, which holds it at every world above.
                let higher = |table: &[u64]| {
                    above[n]
                        .iter()
                        .all(|&up| get(table, up as usize * width + id))
                };
                let t = &table[..];
                let value = match formulas.node(NodeId(id as u32)) {
                    Node::Atom(a) => known.binary_search(&a).is_ok(),
                    Node::True => true,
                    Node::False => falsity,
                    Node::And(a, b) => at(t, a) && at(t, b),
                    Node::Or(a, b) => at(t, a) || at(t, b),
                    Node::Not(a) => (!at(t, a) || falsity) && higher(t),
                    Node::Implies(a, b) => (!at(t, a) || at(t, b)) && higher(t),
                    Node::Iff(a, b) => at(t, a) == at(t, b) && higher(t),
                    Node::Forall(_) | Node::Exists(_) => {
                        unreachable!("{}", crate::ordinary::REFUSED)
                    }
                };
                let cell = n * width + id;
                table[cell / 64] |= u64::from(value) << (cell % 64);
            }
        }
        let is = |n: usize, k: NodeId| get(&table, n * width + k.index());
        let fails: Vec<bool> = (0..worlds)
            .map(|n| {
                sequent.left().iter().all(|&h| is(n, h))
                    && match sequent.right() {
                        [] => !self.worlds[n].falsity,
                        right => right.iter().all(|&c| !is(n, c)),
                    }
            })
            .collect();
        let mut holds = vec![true; worlds];
        for &n in &order {
            let n = n as usize;
            holds[n] = !fails[n] && above[n].iter().all(|&up| holds[up as usize]);
        }
        Ok(Forcing {
            width,
            table,
            holds,
            fails,
            arrows: above,
        })
    }

    /// Checks that the model refutes the sequent in `logic`: it is a model
    /// of the logic ([`forcing`](Self::forcing)), and its world forces
    /// every formula left of `⊢` and not the formula right of it (`⊥`
    /// where there is none; classically, none of them).
    ///
    /// # Errors
    ///
    /// As [`forcing`](Self::forcing), and [`Error::Model`] naming the
    /// hypothesis the model's world does not force or the conclusion it
    /// forces.
    pub fn refutes(
        &self,
        sequent: &Sequent,
        logic: Logic,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<(), Error> {
        let forcing = self.forcing(sequent, logic, limits, stop)?;
        let formulas = sequent.formulas();
        let world = self.world;
        let flaw = |reason: String| Error::Model { reason };
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
    use super::super::{Countermodel, Logic, Sequent};
    use crate::kripke::{Model, World};
    use crate::{Error, Limits, Refusal};

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
        let limits = Limits::default();
        let sequent: Sequent = "|- a \\/ ~a".parse().unwrap();
        let logic = Logic::Intuitionistic;
        // The world above first: the order is the arrows', not the indices'.
        let model = Model::new(vec![world(&["a"], &[]), world(&[], &[0])], 1);
        let forcing = model.forcing(&sequent, logic, &limits, |_| false).unwrap();
        assert!(!forcing.holds(1) && forcing.holds(0));
        assert_eq!(forcing.failing(1), Some(1));
        assert_eq!(forcing.failing(0), None);
        assert_eq!(model.refutes(&sequent, logic, &limits, |_| false), Ok(()));
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
            .forcing(&negation, Logic::Minimal, &limits, |_| false)
            .unwrap();
        assert!(forcing.holds(0));
        minimal.worlds[0].falsity = true;
        minimal.worlds[1].falsity = false;
        let Err(Error::Model { reason }) =
            minimal.forcing(&negation, Logic::Minimal, &limits, |_| false)
        else {
            panic!("falsity that shrinks upward accepted");
        };
        assert!(reason.contains("does not force all it forces"), "{reason}");
        let Err(Error::Model { reason }) = minimal.forcing(&negation, logic, &limits, |_| false)
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
            // A misspelt key is refused by name, not read as no arrows.
            let misspelt = r#"{"worlds":[{"atoms":[],"arrow":[1]},{"atoms":["a"]}]}"#;
            let error = serde_json::from_str::<Model>(misspelt).unwrap_err();
            assert!(
                error.to_string().contains("unknown field `arrow`"),
                "{error}"
            );
        }
    }

    /// The check asks the stop before its first pass over the atoms, reads
    /// an arrow written twice as one, finds a failing world without a
    /// search per world, and refuses past the memory bound, where an
    /// engine's model is dropped and its verdict kept.
    #[test]
    fn the_check_asks_the_stop_and_keeps_to_its_memory() {
        let sequent: Sequent = "a |- b".parse().unwrap();
        let logic = Logic::Intuitionistic;
        let model = Model::new(vec![world(&["a"], &[1, 1]), world(&["a"], &[])], 0);
        let limits = Limits::default();
        assert_eq!(
            model.forcing(&sequent, logic, &limits, |_| true).err(),
            Some(Error::Refused(Refusal::stopped(crate::Phase::Check)))
        );
        let forcing = model.forcing(&sequent, logic, &limits, |_| false).unwrap();
        assert_eq!(forcing.failing_anywhere(), Some(0));
        let small = limits.with_memory_bytes(Some(16));
        let Err(Error::Refused(Refusal::Memory { .. })) =
            model.forcing(&sequent, logic, &small, |_| false)
        else {
            panic!("a check past the memory bound ran");
        };
        let found = Countermodel::found(sequent.clone(), model.clone(), logic, &small, &mut |_| {
            false
        });
        assert!(matches!(found, Ok(None)), "{found:?}");
        let wrong = Model::new(vec![world(&["b"], &[])], 0);
        let found = Countermodel::found(sequent, wrong, logic, &limits, &mut |_| false);
        assert!(
            matches!(found, Err(Error::Countermodel { .. })),
            "{found:?}"
        );
    }
}
