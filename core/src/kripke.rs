// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Finite Kripke models, the countermodels of the suites: worlds, an
//! order on them, and the atoms each world forces.
//!
//! A [`Model`] is data. Each suite checks it against its own sequents:
//! ordinary logic reads the order as a partial order and requires the
//! atoms forced to grow along it (`ordinary::Sequent`'s checks, on
//! [`Model::refutes`] and [`Model::forcing`]). A model of one world is
//! an assignment of classical logic: the atoms it forces are true, the
//! others false.
//!
//! # JSON
//!
//! With the feature `serialize` a model is written and read as
//! `{"version": 13, "worlds": [{"atoms": [], "arrows": [1]}, {"atoms":
//! ["a"]}], "world": 0}`: each world's atoms by name, `"falsity": true`
//! where it forces `⊥` (minimal logic only), and its arrows to the worlds
//! directly above it, by index; `world` the world the model is about,
//! the one where the sequent fails for a countermodel. Nested in an
//! outcome it has no `version`.

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
