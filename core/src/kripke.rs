// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Finite Kripke models, the countermodels of the suites: worlds, an
//! order on them, and the atoms each world forces.
//!
//! A [`Model`] is data. Each suite checks it against its own sequents.
//! Ordinary logic checks it with [`Model::refutes`] and
//! [`Model::forcing`], against an `ordinary::Sequent`: the arrows must
//! form no cycle, so they order the worlds, and the atoms forced must grow
//! along them. A model of one world is an assignment of classical logic:
//! the atoms it forces are true, the others false.
//!
//! Modal logic checks it with [`Model::frame_fault`] and [`Model::truth`],
//! against a `modal::Sequent` in a `modal::System`: there each world's
//! arrows are the worlds it sees, nothing added, and the relation must
//! meet the system's frame conditions.
//!
//! # JSON
//!
//! With the cargo feature `serialize` a model is written and read as
//! `{"version": 13, "worlds": [{"atoms": [], "arrows": [1]}, {"atoms":
//! ["a"]}], "world": 0}`: each world's atoms by name, `"falsity": true`
//! where it forces `⊥` (minimal logic only), and its arrows to the worlds
//! directly above it, by index; `world` the world the model is about,
//! the one where the sequent fails for a countermodel. A key the
//! document does not know is refused by name, so that a misspelt key of a
//! model written by hand is not read as a missing one. Nested in an
//! outcome it has no `version`, and unknown keys are ignored there, as in
//! every form a program writes.

use crate::Error;
use crate::limits::Space;
use std::fmt::{self, Display, Formatter};

/// A finite Kripke model: worlds, arrows from each world to the worlds
/// directly above it, whose reflexive and transitive closure is the
/// order, and the atoms each world forces. It names one world, the one it
/// is about: for a countermodel, the world where the sequent fails.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    /// The worlds, by index.
    pub worlds: Vec<World>,
    /// The world the model is about, by index.
    pub world: u32,
}

/// A world of a [`Model`].
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct World {
    /// The atoms the world forces, by name.
    pub atoms: Vec<String>,
    /// Whether the world forces `⊥`, which minimal logic alone allows.
    pub falsity: bool,
    /// The worlds directly above this one, by index.
    pub arrows: Vec<u32>,
}

impl Model {
    /// Returns a model of these worlds about the world `world`.
    pub const fn new(worlds: Vec<World>, world: u32) -> Self {
        Self { worlds, world }
    }

    /// Checks that the model has the world `world`.
    ///
    /// # Errors
    ///
    /// [`Error::IndexOutOfBounds`] in the space of worlds otherwise.
    pub fn check_world(&self, world: u32) -> Result<(), Error> {
        if (world as usize) < self.worlds.len() {
            Ok(())
        } else {
            Err(Error::IndexOutOfBounds {
                space: Space::World,
                index: world as usize,
                len: self.worlds.len(),
            })
        }
    }
}

impl World {
    /// Returns a world that forces these atoms, `⊥` where `falsity` is
    /// set, with arrows to these worlds.
    pub const fn new(atoms: Vec<String>, falsity: bool, arrows: Vec<u32>) -> Self {
        Self {
            atoms,
            falsity,
            arrows,
        }
    }
}

impl Display for Model {
    /// Writes a line per world: what it forces and the worlds directly
    /// above it, the model's world marked.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (n, world) in self.worlds.iter().enumerate() {
            if n > 0 {
                writeln!(f)?;
            }
            write!(f, "world {n}")?;
            if n == self.world as usize {
                write!(f, " (the model's world)")?;
            }
            let mut forced: Vec<&str> = world.atoms.iter().map(String::as_str).collect();
            if world.falsity {
                forced.push("⊥");
            }
            if forced.is_empty() {
                write!(f, ": forces nothing")?;
            } else {
                write!(f, ": forces {}", forced.join(", "))?;
            }
            if !world.arrows.is_empty() {
                let above: Vec<String> = world.arrows.iter().map(u32::to_string).collect();
                write!(f, "; below {}", above.join(", "))?;
            }
        }
        Ok(())
    }
}
