// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

// A new variant must get a form of its own and never fall into another's.
#![deny(clippy::wildcard_enum_match_arm)]

/// The written form of errors.
mod errors;

pub(crate) use errors::envelope;

/// Serde for an optional choice: `"auto"` for `None`, else the value's
/// name.
pub(crate) mod auto {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::fmt::Display;
    use std::str::FromStr;

    /// Writes `"auto"` or the value's name.
    pub(crate) fn serialize<T: Display, S: Serializer>(
        value: &Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            None => serializer.serialize_str("auto"),
            Some(value) => serializer.collect_str(value),
        }
    }

    /// Reads `"auto"` as `None` and any other name as the value it names.
    pub(crate) fn deserialize<'a, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr<Err: Display>,
        D: Deserializer<'a>,
    {
        let name = String::deserialize(deserializer)?;
        if name == "auto" {
            return Ok(None);
        }
        name.parse().map(Some).map_err(serde::de::Error::custom)
    }
}
/// Serde for an optional bound that JavaScript reads back exactly.
///
/// A bound past 2⁵³ is written `null`, no bound, which it is in effect.
/// Reading one is an error that says `null` lifts the bound.
pub(crate) mod exact {
    use serde::{Deserialize, Deserializer, Serializer};

    /// The largest integer JavaScript reads exactly.
    const MOST: u64 = 1 << 53;

    /// Writes the bound, or `null` for none or one past 2⁵³.
    pub(crate) fn serialize<S: Serializer>(
        value: &Option<u64>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match *value {
            Some(bound) if bound <= MOST => serializer.serialize_u64(bound),
            _ => serializer.serialize_none(),
        }
    }

    /// Reads a bound up to 2⁵³, or `null` for none.
    pub(crate) fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error> {
        match Option::<u64>::deserialize(deserializer)? {
            Some(bound) if bound > MOST => Err(serde::de::Error::custom(format!(
                "the bound {bound} is past 2^53, which JavaScript cannot read exactly; null lifts it"
            ))),
            bound => Ok(bound),
        }
    }
}
/// Serde for `IllSequent`: an arena of intuitionistic formulas, the
/// hypotheses and the goal.
mod ill;
/// Serde for `Interactive`: the sequent, the mode, the inferences and the
/// steps.
#[cfg(feature = "interactive")]
mod interactive;
/// Serde for `ProofStructure` and `MellStructure`: the sequent and the
/// links.
mod nets;
/// Serde for ordinary logic: the sequent, the derivation, the outcome of
/// deciding and a proof in progress.
mod ordinary;
/// Serde for `Proof`, through a proxy with short tags.
mod proofs;
/// Serde for `Fragment`, `Mode`, the outcome of a search and its parts.
mod search;
/// Serde for `Sequent`, through a proxy with short tags.
mod sequents;
