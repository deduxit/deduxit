// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The check of a finite Kripke model ([`kripke`](crate::kripke)) against
//! a modal sequent and a system: whether its relation meets the system's
//! frame conditions, what each world makes true, and whether the sequent
//! holds or fails there.
//!
//! In modal logic a world's arrows are the accessibility relation itself:
//! `w.arrows` lists exactly the worlds `w` sees. Nothing is closed when a
//! model is checked, so a model of S4 lists its reflexive arrows, and a
//! missing one is named.

use super::{Condition, Node, NodeId, Sequent, System};
use crate::Error;
use crate::kripke::Model;
use crate::limits::{Limits, Phase, Progress, Refusal};
use crate::ordinary::MODEL_CELLS;
use std::fmt::{self, Display, Formatter};

/// The bytes the check holds per world besides the table: its arrows'
/// vector, its atoms' vector, its place in the failing list and a flag.
const WORLD_BYTES: u64 = 3 * 24 + 2;

/// The bytes the check holds per atom a world forces: its index, and at
/// most one entry of the map of names (a name and an index, with the
/// map's own share).
const ATOM_BYTES: u64 = 4 + 32;

/// The units of work between two asks of the stop in the frame check.
const POLL: u64 = 1 << 12;

/// A frame condition a model's relation does not meet, with the worlds
/// that show it: one world for a serial or a reflexive relation, two for
/// a symmetric one, three for a transitive or a euclidean one, in the
/// order [`Display`] names them.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    /// The condition the relation does not meet.
    pub condition: Condition,
    /// The worlds that show it.
    pub worlds: Vec<u32>,
}

impl Display for Frame {
    /// Writes what fails: `not transitive: world 0 sees world 1 and world
    /// 1 sees world 2, but world 0 does not see world 2`.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = |i: usize| self.worlds.get(i).copied().unwrap_or_default();
        match self.condition {
            Condition::Serial => write!(f, "not serial: world {} sees no world", w(0)),
            Condition::Reflexive => {
                write!(f, "not reflexive: world {} does not see itself", w(0))
            }
            Condition::Symmetric => write!(
                f,
                "not symmetric: world {} sees world {}, but world {1} does not see world {0}",
                w(0),
                w(1)
            ),
            Condition::Transitive => write!(
                f,
                "not transitive: world {} sees world {} and world {1} sees world {}, \
                 but world {0} does not see world {2}",
                w(0),
                w(1),
                w(2)
            ),
            Condition::Euclidean => write!(
                f,
                "not euclidean: world {} sees worlds {} and {}, but world {1} does not see \
                 world {2}",
                w(0),
                w(1),
                w(2)
            ),
        }
    }
}

/// What each world of a model makes true, for every formula of a modal
/// sequent's arena, as [`Model::truth`] computes it.
#[derive(Clone, Debug)]
pub struct Truth {
    /// The number of formulas in the sequent's arena: the bits per world.
    width: usize,
    /// A bit per world and formula.
    table: Vec<u64>,
    /// The worlds each world sees, sorted, each once.
    sees: Vec<Vec<u32>>,
    /// Per world, whether the sequent fails there: every formula left of
    /// `⊢` true and every one right of it false.
    fails: Vec<bool>,
    /// The model's world.
    world: u32,
}

impl Truth {
    /// Returns whether the formula `id` of the sequent's arena is true at
    /// `world`.
    ///
    /// # Panics
    ///
    /// For a world or a formula outside the model or the arena.
    pub fn holds(&self, world: u32, id: NodeId) -> bool {
        assert!(id.index() < self.width, "a formula of the sequent's arena");
        assert!((world as usize) < self.fails.len(), "a world of the model");
        let cell = world as usize * self.width + id.index();
        self.table[cell / 64] >> (cell % 64) & 1 == 1
    }

    /// Returns what `world` makes true of every formula of the sequent's
    /// arena, each operand before its formula.
    ///
    /// # Panics
    ///
    /// For a world outside the model.
    pub fn at(&self, world: u32) -> impl Iterator<Item = (NodeId, bool)> + '_ {
        (0..self.width).map(move |id| {
            let id = NodeId(id as u32);
            (id, self.holds(world, id))
        })
    }

    /// Returns whether the sequent holds at `world`: some formula left of
    /// `⊢` is false there, or some formula right of it true.
    ///
    /// # Panics
    ///
    /// For a world outside the model.
    pub fn sequent_holds(&self, world: u32) -> bool {
        !self.fails[world as usize]
    }

    /// Returns a world where the sequent fails, the model's own world
    /// where it fails there, or `None` where it holds at every world.
    pub fn failing(&self) -> Option<u32> {
        if self.fails[self.world as usize] {
            return Some(self.world);
        }
        self.fails.iter().position(|&fails| fails).map(|w| w as u32)
    }

    /// Returns why a modal formula has its value at `world`: for `□A`
    /// false or `◇A` true there, a world it sees where `A` is false or
    /// true. `None` for every other formula and value, whose reason is
    /// every world it sees, or none.
    ///
    /// # Panics
    ///
    /// For a world or a formula outside the model or the arena.
    pub fn witness(&self, sequent: &Sequent, world: u32, id: NodeId) -> Option<u32> {
        let value = self.holds(world, id);
        let (operand, wanted) = match sequent.formulas().node(id) {
            Node::Box(a) if !value => (a, false),
            Node::Diamond(a) if value => (a, true),
            Node::Atom(_)
            | Node::True
            | Node::False
            | Node::Not(_)
            | Node::And(..)
            | Node::Or(..)
            | Node::Implies(..)
            | Node::Iff(..)
            | Node::Box(_)
            | Node::Diamond(_) => return None,
        };
        self.sees[world as usize]
            .iter()
            .copied()
            .find(|&seen| self.holds(seen, operand) == wanted)
    }
}

impl Model {
    /// Returns the first frame condition of `system` that the model's
    /// relation does not meet, in the order of [`CHECKED`], with
    /// the worlds that show it; `None` where it meets them all, so that
    /// the model is one of the system.
    ///
    /// The relation is the arrows as written: an arrow written twice is
    /// one arrow, and nothing is added to it.
    ///
    /// The stop is asked every few thousand units of work: a world, an
    /// arrow, and for a transitive or a euclidean system every world
    /// looked up among the arrows of another.
    ///
    /// # Errors
    ///
    /// [`Error::Model`] for a model of no world, whose own world is none
    /// of its worlds, with an arrow to a world it does not have, or with a
    /// world that forces `⊥`, which no modal model does;
    /// [`Refusal::Work`] past `limits.work` and [`Refusal::Stopped`] when
    /// the stop fired, neither a verdict on the model.
    pub fn frame_fault(
        &self,
        system: System,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Option<Frame>, Error> {
        let sees = self.shape()?;
        let mut meter = Meter::new(limits);
        frame_fault(&sees, system, &mut meter, &mut stop)
    }

    /// Checks that the model is one of `system` ([`frame_fault`](Self::frame_fault))
    /// and computes what each of its worlds makes true, for every formula
    /// of the sequent's arena.
    ///
    /// An atom is true at a world where the world forces it. `¬`, `∧`,
    /// `∨`, `→` and `↔` are read at the world, classically. `□A` is true
    /// at a world where `A` is true at every world it sees, `◇A` where `A`
    /// is true at some world it sees. An atom the sequent does not name
    /// changes nothing.
    ///
    /// # Errors
    ///
    /// As [`frame_fault`](Self::frame_fault), and [`Error::Model`] naming
    /// the frame condition the model does not meet, with its worlds;
    /// [`Refusal::Memory`] where the check would compute more than
    /// [`MODEL_CELLS`] cells (a world or an arrow, times a formula of the
    /// arena) or hold more than `limits.memory_bytes`, neither a verdict
    /// on the model.
    pub fn truth(
        &self,
        sequent: &Sequent,
        system: System,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
    ) -> Result<Truth, Error> {
        let worlds = self.worlds.len();
        let width = sequent.formulas().len();
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
        let held = (worlds as u64)
            .saturating_mul(WORLD_BYTES)
            .saturating_add(arrows.saturating_mul(4))
            .saturating_add(atoms.saturating_mul(ATOM_BYTES))
            .saturating_add(cells / 8);
        if let Some(limit) = limits.memory_bytes
            && held > limit
        {
            return refused(limit, held);
        }
        let sees = self.shape()?;
        let mut meter = Meter::new(limits);
        if let Some(frame) = frame_fault(&sees, system, &mut meter, &mut stop)? {
            return Err(Error::Model {
                reason: frame.to_string(),
            });
        }
        // Each world's atoms of the sequent, as sorted indices.
        let formulas = sequent.formulas();
        let index: crate::hash::HashMap<&str, u32> = formulas
            .atom_names()
            .iter()
            .enumerate()
            .map(|(i, name)| (name.as_str(), i as u32))
            .collect();
        let mut forced: Vec<Vec<u32>> = Vec::with_capacity(worlds);
        for world in &self.worlds {
            meter.tick(1 + world.atoms.len() as u64, &mut stop)?;
            let mut ids: Vec<u32> = world
                .atoms
                .iter()
                .filter_map(|name| index.get(name.as_str()).copied())
                .collect();
            ids.sort_unstable();
            ids.dedup();
            forced.push(ids);
        }
        // A bit per world and formula; below `MODEL_CELLS` the index fits.
        // Formula by formula, each operand before its formula, so that a
        // modal formula reads its operand at every world already.
        let mut table = vec![0u64; (worlds * width).div_ceil(64)];
        let get = |table: &[u64], cell: usize| table[cell / 64] >> (cell % 64) & 1 == 1;
        for id in 0..width {
            meter.tick((worlds as u64).saturating_add(arrows), &mut stop)?;
            let node = formulas.node(NodeId(id as u32));
            for w in 0..worlds {
                let t = &table[..];
                let at = |k: NodeId| get(t, w * width + k.index());
                let seen = |k: NodeId| {
                    sees[w]
                        .iter()
                        .map(move |&s| get(t, s as usize * width + k.index()))
                };
                let value = match node {
                    Node::Atom(a) => forced[w].binary_search(&a).is_ok(),
                    Node::True => true,
                    Node::False => false,
                    Node::Not(a) => !at(a),
                    Node::And(a, b) => at(a) && at(b),
                    Node::Or(a, b) => at(a) || at(b),
                    Node::Implies(a, b) => !at(a) || at(b),
                    Node::Iff(a, b) => at(a) == at(b),
                    Node::Box(a) => seen(a).all(|v| v),
                    Node::Diamond(a) => seen(a).any(|v| v),
                };
                let cell = w * width + id;
                table[cell / 64] |= u64::from(value) << (cell % 64);
            }
        }
        let is = |w: usize, k: NodeId| get(&table, w * width + k.index());
        let fails = (0..worlds)
            .map(|w| {
                sequent.left().iter().all(|&h| is(w, h))
                    && sequent.right().iter().all(|&c| !is(w, c))
            })
            .collect();
        Ok(Truth {
            width,
            table,
            sees,
            fails,
            world: self.world,
        })
    }

    /// Checks the model's shape for modal logic and returns each world's
    /// arrows, sorted, each once.
    fn shape(&self) -> Result<Vec<Vec<u32>>, Error> {
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
        let mut sees = Vec::with_capacity(worlds);
        for (n, world) in self.worlds.iter().enumerate() {
            if world.falsity {
                return Err(flaw(format!(
                    "world {n} forces ⊥, which no modal model does"
                )));
            }
            if let Some(&up) = world.arrows.iter().find(|&&up| up as usize >= worlds) {
                return Err(flaw(format!(
                    "world {n} has an arrow to world {up}, which the model does not have"
                )));
            }
            let mut arrows = world.arrows.clone();
            arrows.sort_unstable();
            arrows.dedup();
            sees.push(arrows);
        }
        Ok(sees)
    }
}

/// Counts the work of a check against `limits.work` and asks the stop
/// every [`POLL`] units.
struct Meter {
    /// The bound, or `u64::MAX` for none.
    limit: u64,
    /// The units done, saturating.
    done: u64,
    /// The units done at the last ask.
    asked: u64,
}

impl Meter {
    /// Returns a meter with nothing done.
    const fn new(limits: &Limits) -> Self {
        Self {
            limit: match limits.work {
                Some(limit) => limit,
                None => u64::MAX,
            },
            done: 0,
            asked: 0,
        }
    }

    /// Counts `units`, asks the stop where a poll is due, and refuses past
    /// the bound.
    fn tick(&mut self, units: u64, stop: &mut impl FnMut(Progress) -> bool) -> Result<(), Error> {
        self.done = self.done.saturating_add(units);
        if self.done > self.limit {
            return Err(Error::Refused(Refusal::Work { limit: self.limit }));
        }
        // `asked` is at most `done`, so the difference does not wrap.
        if self.done - self.asked >= POLL || self.asked == 0 {
            let since = self.done - self.asked;
            self.asked = self.done.max(1);
            if stop(Progress::new(Phase::Check, since, self.done)) {
                return Err(Error::Refused(Refusal::stopped(Phase::Check)));
            }
        }
        Ok(())
    }
}

/// The order the frame conditions are checked in: reflexive before
/// serial, since a world without arrows is better told that it does not
/// see itself where the system asks it to.
pub const CHECKED: &[Condition] = &[
    Condition::Reflexive,
    Condition::Serial,
    Condition::Symmetric,
    Condition::Transitive,
    Condition::Euclidean,
];

/// Returns the first condition of `system` the relation `sees` (each
/// world's arrows, sorted, each once) does not meet, with its worlds.
fn frame_fault(
    sees: &[Vec<u32>],
    system: System,
    meter: &mut Meter,
    stop: &mut impl FnMut(Progress) -> bool,
) -> Result<Option<Frame>, Error> {
    let fault = |condition, worlds: &[u32]| {
        Ok(Some(Frame {
            condition,
            worlds: worlds.to_vec(),
        }))
    };
    let has = |x: u32, y: u32| sees[x as usize].binary_search(&y).is_ok();
    for &condition in CHECKED.iter().filter(|&&c| system.has(c)) {
        for (x, seen) in sees.iter().enumerate() {
            let x = x as u32;
            meter.tick(1 + seen.len() as u64, stop)?;
            match condition {
                Condition::Serial => {
                    if seen.is_empty() {
                        return fault(condition, &[x]);
                    }
                }
                Condition::Reflexive => {
                    if !has(x, x) {
                        return fault(condition, &[x]);
                    }
                }
                Condition::Symmetric => {
                    if let Some(&y) = seen.iter().find(|&&y| !has(y, x)) {
                        return fault(condition, &[x, y]);
                    }
                }
                // For every arrow x → y: what y sees, x sees.
                Condition::Transitive => {
                    for &y in seen {
                        meter.tick(sees[y as usize].len() as u64, stop)?;
                        if let Some(&z) = sees[y as usize].iter().find(|&&z| !has(x, z)) {
                            return fault(condition, &[x, y, z]);
                        }
                    }
                }
                // For every arrow x → y: what x sees, y sees.
                Condition::Euclidean => {
                    for &y in seen {
                        meter.tick(seen.len() as u64, stop)?;
                        if let Some(&z) = seen.iter().find(|&&z| !has(y, z)) {
                            return fault(condition, &[x, y, z]);
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

/// A Kripke model that refutes a modal sequent in a system, with the
/// sequent and the system: the evidence of a "not valid".
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Countermodel {
    /// The sequent refuted.
    pub sequent: Sequent,
    /// The system the model is one of.
    pub system: System,
    /// The model. Its relation meets the system's frame conditions, and at
    /// its world every formula left of the sequent's `⊢` is true and every
    /// one right of it false, as [`check`](Self::check) checks.
    pub model: Model,
}

impl Countermodel {
    /// Returns the countermodel of this sequent, system and model,
    /// unchecked.
    pub const fn new(sequent: Sequent, system: System, model: Model) -> Self {
        Self {
            sequent,
            system,
            model,
        }
    }

    /// Checks that the model is one of the system and that the sequent
    /// fails at the model's world.
    ///
    /// # Errors
    ///
    /// As [`Model::truth`], and [`Error::Model`] naming the formula left of
    /// `⊢` that is false at the model's world or the formula right of it
    /// that is true.
    pub fn check(&self, limits: &Limits, stop: impl FnMut(Progress) -> bool) -> Result<(), Error> {
        let truth = self.model.truth(&self.sequent, self.system, limits, stop)?;
        let world = self.model.world;
        let formulas = self.sequent.formulas();
        if let Some(&h) = self
            .sequent
            .left()
            .iter()
            .find(|&&h| !truth.holds(world, h))
        {
            return Err(Error::Model {
                reason: format!("{} is false at world {world}", formulas.formula(h)),
            });
        }
        if let Some(&c) = self
            .sequent
            .right()
            .iter()
            .find(|&&c| truth.holds(world, c))
        {
            return Err(Error::Model {
                reason: format!("{} is true at world {world}", formulas.formula(c)),
            });
        }
        Ok(())
    }

    /// Returns the countermodel a search found, checked: `None` where it
    /// is too large to check (past [`MODEL_CELLS`] or
    /// `limits.memory_bytes`).
    ///
    /// # Errors
    ///
    /// [`Error::Countermodel`] where the model does not refute the
    /// sequent, a defect of the search; [`Refusal::Stopped`] and
    /// [`Refusal::Work`] as [`check`](Self::check) answers them.
    #[cfg(feature = "serialize")]
    pub(crate) fn found(
        sequent: Sequent,
        system: System,
        model: Model,
        limits: &Limits,
        stop: impl FnMut(Progress) -> bool,
    ) -> Result<Option<Self>, Error> {
        let countermodel = Self::new(sequent, system, model);
        match countermodel.check(limits, stop) {
            Ok(()) => Ok(Some(countermodel)),
            Err(Error::Refused(Refusal::Memory { .. })) => Ok(None),
            Err(Error::Model { reason }) => Err(Error::Countermodel { reason }),
            Err(error) => Err(error),
        }
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::kripke::World;

    /// Returns a model of worlds with these atoms and arrows, about world
    /// 0.
    fn model(worlds: &[(&[&str], &[u32])]) -> Model {
        let worlds = worlds
            .iter()
            .map(|(atoms, arrows)| {
                World::new(
                    atoms.iter().map(|&a| a.to_owned()).collect(),
                    false,
                    arrows.to_vec(),
                )
            })
            .collect();
        Model::new(worlds, 0)
    }

    /// Returns the frame fault of a model in a system, within the default
    /// limits.
    fn fault(model: &Model, system: System) -> Option<Frame> {
        model
            .frame_fault(system, &Limits::default(), |_| false)
            .unwrap()
    }

    /// Each condition that fails is named with the worlds that show it,
    /// first in the order of the conditions; a model meets the conditions
    /// of a system it is one of, whatever condition it was made of.
    #[test]
    fn a_frame_condition_that_fails_is_named() {
        let chain = model(&[(&[], &[1]), (&[], &[2]), (&[], &[])]);
        assert_eq!(fault(&chain, System::K), None);
        let f = |condition, worlds: &[u32]| {
            Some(Frame {
                condition,
                worlds: worlds.to_vec(),
            })
        };
        assert_eq!(fault(&chain, System::D), f(Condition::Serial, &[2]));
        assert_eq!(fault(&chain, System::KB), f(Condition::Symmetric, &[0, 1]));
        assert_eq!(
            fault(&chain, System::K4),
            f(Condition::Transitive, &[0, 1, 2])
        );
        let fork = model(&[(&[], &[1, 2]), (&[], &[]), (&[], &[])]);
        assert_eq!(
            fault(&fork, System::K5),
            f(Condition::Euclidean, &[0, 1, 1])
        );
        let loops = model(&[(&[], &[0, 1]), (&[], &[1])]);
        assert_eq!(fault(&loops, System::S4), None);
        assert_eq!(fault(&loops, System::B), f(Condition::Symmetric, &[0, 1]));
        let missing = model(&[(&[], &[1]), (&[], &[1])]);
        assert_eq!(fault(&missing, System::T), f(Condition::Reflexive, &[0]));
        assert_eq!(
            f(Condition::Transitive, &[0, 1, 2]).unwrap().to_string(),
            "not transitive: world 0 sees world 1 and world 1 sees world 2, but world 0 \
             does not see world 2"
        );
        let cluster = model(&[(&[], &[0, 1]), (&[], &[0, 1])]);
        for &system in System::ALL {
            assert_eq!(fault(&cluster, system), None, "{system}");
        }
    }

    /// A model of no world, an arrow to a world it lacks, a world that
    /// forces `⊥` and a world of its own it lacks are refused.
    #[test]
    fn a_model_of_another_shape_is_refused() {
        let mut bad = model(&[(&[], &[3])]);
        let refused = |m: &Model| {
            matches!(
                m.frame_fault(System::K, &Limits::default(), |_| false),
                Err(Error::Model { .. })
            )
        };
        assert!(refused(&bad));
        bad.worlds[0].arrows.clear();
        bad.worlds[0].falsity = true;
        assert!(refused(&bad));
        assert!(refused(&Model::new(Vec::new(), 0)));
        assert!(refused(&model(&[(&[], &[])]).with_world(1)));
    }

    impl Model {
        /// Returns the model about another world.
        fn with_world(mut self, world: u32) -> Self {
            self.world = world;
            self
        }
    }

    /// `□` and `◇` read the worlds a world sees; the sequent fails where
    /// its left is true and its right false; a witness is a world seen
    /// that shows a false `□A` or a true `◇A`.
    #[test]
    fn truth_reads_the_worlds_seen() {
        let sequent: Sequent = "[]p, <>q |- []q, p".parse().unwrap();
        let m = model(&[(&[], &[1, 2]), (&["p", "q"], &[]), (&["p"], &[])]);
        let truth = m
            .truth(&sequent, System::K, &Limits::default(), |_| false)
            .unwrap();
        assert!(!truth.sequent_holds(0));
        assert_eq!(truth.failing(), Some(0));
        let boxed_q = sequent.right()[0];
        assert_eq!(truth.witness(&sequent, 0, boxed_q), Some(2));
        let diamond_q = sequent.left()[1];
        assert_eq!(truth.witness(&sequent, 0, diamond_q), Some(1));
        // At a world that sees nothing every `□` is true and every `◇`
        // false.
        assert!(truth.sequent_holds(1));
        let countermodel = Countermodel::new(sequent.clone(), System::K, m.clone());
        assert!(countermodel.check(&Limits::default(), |_| false).is_ok());
        // Not a model of T: the check names the condition.
        let refused = m.truth(&sequent, System::T, &Limits::default(), |_| false);
        assert!(
            matches!(refused, Err(Error::Model { reason }) if reason.starts_with("not reflexive"))
        );
        let holds = Countermodel::new(sequent, System::K, m.with_world(1));
        assert!(matches!(
            holds.check(&Limits::default(), |_| false),
            Err(Error::Model { .. })
        ));
    }

    /// The check refuses past its cells, its memory and its work, and
    /// stops when asked.
    #[test]
    fn the_check_keeps_to_its_bounds() {
        let sequent: Sequent = "[]p -> p".parse().unwrap();
        let worlds: Vec<(&[&str], &[u32])> = (0..64).map(|_| (&[][..], &[0][..])).collect();
        let m = model(&worlds);
        let tight = Limits::default().with_memory_bytes(Some(64));
        assert!(matches!(
            m.truth(&sequent, System::K, &tight, |_| false),
            Err(Error::Refused(Refusal::Memory { .. }))
        ));
        let work = Limits::default().with_work(Some(10));
        assert!(matches!(
            m.frame_fault(System::D, &work, |_| false),
            Err(Error::Refused(Refusal::Work { .. }))
        ));
        assert!(matches!(
            m.frame_fault(System::D, &Limits::default(), |_| true),
            Err(Error::Refused(Refusal::Stopped { .. }))
        ));
    }
}
