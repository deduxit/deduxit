// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The wire form of a Kripke model: its worlds, each world's atoms by
//! name, `⊥` where it forces it, and its arrows; and the world it is
//! about.

use crate::Error;
use crate::kripke::{Model, World};
use crate::limits::Limits;
use crate::linear::sequents::name;
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The serialized form of a world.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct WorldForm {
    /// The atoms the world forces, by name.
    atoms: Vec<String>,
    /// Whether the world forces `⊥`, written only where it does.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    falsity: bool,
    /// The worlds directly above, written only where there are some.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    arrows: Vec<u32>,
}

/// The serialized form of a Kripke model.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct ModelForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The worlds.
    worlds: Vec<WorldForm>,
    /// The world the model is about; the first where it is missing.
    #[serde(default)]
    world: u32,
}

/// A world of a model document, which refuses a key it does not know.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StrictWorld {
    /// The atoms the world forces, by name.
    atoms: Vec<String>,
    /// Whether the world forces `⊥`.
    #[serde(default)]
    falsity: bool,
    /// The worlds directly above.
    #[serde(default)]
    arrows: Vec<u32>,
}

/// A model document, which refuses a key it does not know: a model is
/// often written by hand, and a misspelt `arrows` would otherwise read as
/// a world without arrows. A model nested in an outcome is read as
/// [`ModelForm`], which ignores unknown keys as every data form does.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StrictModel {
    /// The wire level.
    #[serde(default, deserialize_with = "wire::version")]
    version: Option<u32>,
    /// The worlds.
    worlds: Vec<StrictWorld>,
    /// The world the model is about; the first where it is missing.
    #[serde(default)]
    world: u32,
}

impl From<StrictModel> for ModelForm {
    /// Returns the form of the model the document holds.
    fn from(strict: StrictModel) -> Self {
        Self {
            version: strict.version,
            worlds: strict
                .worlds
                .into_iter()
                .map(|w| WorldForm {
                    atoms: w.atoms,
                    falsity: w.falsity,
                    arrows: w.arrows,
                })
                .collect(),
            world: strict.world,
        }
    }
}

impl From<&Model> for ModelForm {
    /// Writes the model's form, nested: without its level.
    fn from(model: &Model) -> Self {
        Self {
            version: None,
            worlds: model
                .worlds
                .iter()
                .map(|w| WorldForm {
                    atoms: w.atoms.clone(),
                    falsity: w.falsity,
                    arrows: w.arrows.clone(),
                })
                .collect(),
            world: model.world,
        }
    }
}

impl ModelForm {
    /// Returns the model the form holds, every atom's name composed (NFC)
    /// and checked as a sequent's names are; more worlds, arrows and atoms
    /// together than `limits.occurrences` are refused. Whether the model is
    /// one of a logic is for that logic's check to say.
    pub(super) fn within(self, limits: &Limits) -> Result<Model, Error> {
        let size = self.worlds.iter().fold(self.worlds.len() as u64, |n, w| {
            n.saturating_add(w.atoms.len() as u64)
                .saturating_add(w.arrows.len() as u64)
        });
        if let Some(limit) = limits.occurrences
            && size > limit
        {
            return Err(Error::Refused(crate::Refusal::Occurrences {
                occurrences: size,
                limit,
            }));
        }
        let mut worlds = Vec::with_capacity(self.worlds.len());
        for form in self.worlds {
            let mut atoms = Vec::with_capacity(form.atoms.len());
            for atom in form.atoms {
                let atom = name::normalized(&atom).into_owned();
                name::check(&atom)?;
                atoms.push(atom);
            }
            worlds.push(World::new(atoms, form.falsity, form.arrows));
        }
        Ok(Model::new(worlds, self.world))
    }
}

impl Serialize for Model {
    /// Serializes the model as a document at level 13.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = ModelForm::from(self);
        form.version = Some(wire::KRIPKE);
        form.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Model {
    /// Deserializes a model within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Model {}

impl Readable for Model {
    const FORM: &'static str = "Kripke model";

    /// Reads a model document, its names checked, not whether it is a
    /// model of a logic; a key it does not know is refused by name.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        ModelForm::from(StrictModel::deserialize(deserializer)?)
            .within(limits)
            .map_err(wire::fail)
    }
}
