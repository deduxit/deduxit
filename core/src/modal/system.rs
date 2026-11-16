// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The normal modal systems from K to S5 as values: a [`System`] is the
//! set of [`Condition`]s its frames meet.

use crate::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A condition on a frame's accessibility relation `R`. Each is the frame
/// condition of one axiom added to the system K.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum Condition {
    /// Every world sees some world. The axiom D, `□A → ◇A`.
    Serial,
    /// Every world sees itself. The axiom T, `□A → A`.
    Reflexive,
    /// A world that sees another is seen by it. The axiom B, `A → □◇A`.
    Symmetric,
    /// A world sees what the worlds it sees see. The axiom 4, `□A → □□A`.
    Transitive,
    /// Two worlds that one world sees see each other. The axiom 5,
    /// `◇A → □◇A`.
    Euclidean,
}

impl Condition {
    /// Every condition, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[
        Self::Serial,
        Self::Reflexive,
        Self::Symmetric,
        Self::Transitive,
        Self::Euclidean,
    ];

    /// The conditions' names, which [`name`](Self::name) writes and
    /// `FromStr` reads.
    pub const NAMES: &'static [&'static str] = &[
        "serial",
        "reflexive",
        "symmetric",
        "transitive",
        "euclidean",
    ];

    /// Returns the condition's name in lower case.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Serial => "serial",
            Self::Reflexive => "reflexive",
            Self::Symmetric => "symmetric",
            Self::Transitive => "transitive",
            Self::Euclidean => "euclidean",
        }
    }

    /// Returns the name of the axiom whose frame condition this is: `D`,
    /// `T`, `B`, `4` or `5`.
    pub const fn axiom(self) -> &'static str {
        match self {
            Self::Serial => "D",
            Self::Reflexive => "T",
            Self::Symmetric => "B",
            Self::Transitive => "4",
            Self::Euclidean => "5",
        }
    }

    /// Returns the condition's bit in a [`System`].
    const fn bit(self) -> u8 {
        match self {
            Self::Serial => SERIAL,
            Self::Reflexive => REFLEXIVE,
            Self::Symmetric => SYMMETRIC,
            Self::Transitive => TRANSITIVE,
            Self::Euclidean => EUCLIDEAN,
        }
    }
}

impl Display for Condition {
    /// Writes the condition's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for Condition {
    type Err = Error;

    /// Reads a condition's [`name`](Self::name), one of
    /// [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        Self::ALL
            .iter()
            .copied()
            .find(|condition| condition.name() == name)
            .ok_or_else(|| Error::UnknownName {
                what: "frame condition",
                name: name.into(),
                known: Self::NAMES,
            })
    }
}

/// The bit of [`Condition::Serial`].
const SERIAL: u8 = 1;
/// The bit of [`Condition::Reflexive`].
const REFLEXIVE: u8 = 2;
/// The bit of [`Condition::Symmetric`].
const SYMMETRIC: u8 = 4;
/// The bit of [`Condition::Transitive`].
const TRANSITIVE: u8 = 8;
/// The bit of [`Condition::Euclidean`].
const EUCLIDEAN: u8 = 16;

/// Returns `bits` with every condition that follows from them on every
/// frame.
const fn closed(mut bits: u8) -> u8 {
    /// Whether `bits` has all of `all`.
    const fn has(bits: u8, all: u8) -> bool {
        bits & all == all
    }
    loop {
        let before = bits;
        // From `xRy`, symmetry gives `yRx` and transitivity `xRx`.
        if has(bits, SERIAL | SYMMETRIC | TRANSITIVE) {
            bits |= REFLEXIVE;
        }
        if has(bits, REFLEXIVE) {
            bits |= SERIAL;
        }
        // From `xRy` and `xRx`, a euclidean relation gives `yRx`.
        if has(bits, REFLEXIVE | EUCLIDEAN) {
            bits |= SYMMETRIC;
        }
        // From `xRy` and `xRz`, symmetry gives `yRx`, transitivity `yRz`.
        if has(bits, SYMMETRIC | TRANSITIVE) {
            bits |= EUCLIDEAN;
        }
        // From `xRy` and `yRz`, symmetry gives `yRx`, and a euclidean
        // relation `xRz`.
        if has(bits, SYMMETRIC | EUCLIDEAN) {
            bits |= TRANSITIVE;
        }
        if bits == before {
            return bits;
        }
    }
}

/// A normal modal system: the logic of the frames that meet a set of
/// [`Condition`]s. With no condition it is K, the logic of all frames.
///
/// The five conditions give fifteen systems, since some sets of them
/// have the same frames: a reflexive and euclidean relation is symmetric
/// and transitive, so T with the axiom 5 is S5. A system holds every
/// condition that follows from the ones it was made of, so two systems
/// are equal exactly when they have the same frames, and so the same
/// valid sequents.
///
/// ```
/// use deduxit::modal::{Condition, System};
///
/// let system = System::T.with(Condition::Euclidean);
/// assert_eq!(system, System::S5);
/// assert!(system.has(Condition::Transitive));
/// assert_eq!("KT4".parse::<System>()?, System::S4);
/// # Ok::<(), deduxit::Error>(())
/// ```
///
/// # Names
///
/// [`name`](Self::name) writes one name per system: `K`, `D`, `T`, `KB`,
/// `KDB`, `B`, `K4`, `KD4`, `S4`, `K5`, `KD5`, `K45`, `KD45`, `KB5` and
/// `S5`. `FromStr` reads those, in either case, and every name made of
/// `K` and the axioms' letters in any order: `KT` is T, `KTB` is B, `KT4`
/// is S4, `KT5` and `KDB4` are S5, `KB4` is KB5. A leading `D` reads as
/// `KD`, so `DB` is KDB.
///
/// # JSON
///
/// With the cargo feature `serialize` a system is its name, `"S4"`. The
/// reader takes every name `FromStr` reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct System(u8);

impl System {
    /// K: every frame.
    pub const K: Self = Self(0);
    /// D, also written KD: the serial frames.
    pub const D: Self = Self(closed(SERIAL));
    /// T, also written KT: the reflexive frames.
    pub const T: Self = Self(closed(REFLEXIVE));
    /// KB: the symmetric frames.
    pub const KB: Self = Self(closed(SYMMETRIC));
    /// KDB: the serial and symmetric frames.
    pub const KDB: Self = Self(closed(SERIAL | SYMMETRIC));
    /// B, also written KTB: the reflexive and symmetric frames.
    pub const B: Self = Self(closed(REFLEXIVE | SYMMETRIC));
    /// K4: the transitive frames.
    pub const K4: Self = Self(closed(TRANSITIVE));
    /// KD4: the serial and transitive frames.
    pub const KD4: Self = Self(closed(SERIAL | TRANSITIVE));
    /// S4, also written KT4: the reflexive and transitive frames.
    pub const S4: Self = Self(closed(REFLEXIVE | TRANSITIVE));
    /// K5: the euclidean frames.
    pub const K5: Self = Self(closed(EUCLIDEAN));
    /// KD5: the serial and euclidean frames.
    pub const KD5: Self = Self(closed(SERIAL | EUCLIDEAN));
    /// K45: the transitive and euclidean frames.
    pub const K45: Self = Self(closed(TRANSITIVE | EUCLIDEAN));
    /// KD45: the serial, transitive and euclidean frames.
    pub const KD45: Self = Self(closed(SERIAL | TRANSITIVE | EUCLIDEAN));
    /// KB5, also written KB4: the symmetric and transitive frames, which
    /// are euclidean too.
    pub const KB5: Self = Self(closed(SYMMETRIC | TRANSITIVE));
    /// S5, also written KT5: the frames whose relation is an equivalence.
    pub const S5: Self = Self(closed(REFLEXIVE | EUCLIDEAN));

    /// Every system, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[
        Self::K,
        Self::D,
        Self::T,
        Self::KB,
        Self::KDB,
        Self::B,
        Self::K4,
        Self::KD4,
        Self::S4,
        Self::K5,
        Self::KD5,
        Self::K45,
        Self::KD45,
        Self::KB5,
        Self::S5,
    ];

    /// The systems' names, which [`name`](Self::name) writes.
    pub const NAMES: &'static [&'static str] = &[
        "K", "D", "T", "KB", "KDB", "B", "K4", "KD4", "S4", "K5", "KD5", "K45", "KD45", "KB5", "S5",
    ];

    /// Returns the system of the frames that meet every condition given.
    pub fn of(conditions: impl IntoIterator<Item = Condition>) -> Self {
        conditions.into_iter().fold(Self::K, Self::with)
    }

    /// Returns the system of this one's frames that also meet
    /// `condition`.
    #[must_use]
    pub const fn with(self, condition: Condition) -> Self {
        Self(closed(self.0 | condition.bit()))
    }

    /// Returns whether every frame of the system meets `condition`.
    pub const fn has(self, condition: Condition) -> bool {
        self.0 & condition.bit() != 0
    }

    /// Returns the conditions every frame of the system meets, in the
    /// order of [`Condition::ALL`].
    pub fn conditions(self) -> impl Iterator<Item = Condition> {
        Condition::ALL
            .iter()
            .copied()
            .filter(move |&condition| self.has(condition))
    }

    /// Returns whether the system extends `other`: its frames meet every
    /// condition of `other`, so a sequent valid in `other` is valid in
    /// it.
    pub const fn extends(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns the system's name, one of [`NAMES`](Self::NAMES).
    pub const fn name(self) -> &'static str {
        // A system holds a closed set of conditions, so it is one of `ALL`:
        // the last where it is none before.
        let mut index = 0;
        while index + 1 < Self::ALL.len() && Self::ALL[index].0 != self.0 {
            index += 1;
        }
        Self::NAMES[index]
    }
}

impl Display for System {
    /// Writes the system's [`name`](Self::name).
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for System {
    type Err = Error;

    /// Reads a system's name, as [`System`] documents the names.
    fn from_str(name: &str) -> Result<Self, Error> {
        let unknown = || Error::UnknownName {
            what: "modal system",
            name: name.into(),
            known: Self::NAMES,
        };
        let upper = name.to_ascii_uppercase();
        if let Some(index) = Self::NAMES.iter().position(|&known| known == upper) {
            return Ok(Self::ALL[index]);
        }
        // `K` and the axioms' letters, or `D` for `KD` and more letters.
        let (start, letters) = match upper.as_bytes() {
            [b'K', letters @ ..] => (Self::K, letters),
            [b'D', letters @ ..] => (Self::D, letters),
            _ => return Err(unknown()),
        };
        letters.iter().try_fold(start, |system, letter| {
            Condition::ALL
                .iter()
                .find(|condition| condition.axiom().as_bytes() == [*letter])
                .map(|&condition| system.with(condition))
                .ok_or_else(unknown)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns whether the relation `r` on `n` worlds (bit `n * x + y` for
    /// `xRy`) meets `condition`.
    fn meets(r: u32, n: u32, condition: Condition) -> bool {
        let sees = |x: u32, y: u32| r >> (n * x + y) & 1 == 1;
        let worlds = || 0..n;
        match condition {
            Condition::Serial => worlds().all(|x| worlds().any(|y| sees(x, y))),
            Condition::Reflexive => worlds().all(|x| sees(x, x)),
            Condition::Symmetric => worlds().all(|x| worlds().all(|y| !sees(x, y) || sees(y, x))),
            Condition::Transitive => worlds().all(|x| {
                worlds().all(|y| worlds().all(|z| !(sees(x, y) && sees(y, z)) || sees(x, z)))
            }),
            Condition::Euclidean => worlds().all(|x| {
                worlds().all(|y| worlds().all(|z| !(sees(x, y) && sees(x, z)) || sees(y, z)))
            }),
        }
    }

    /// A system holds exactly the conditions that every frame of up to
    /// three worlds meets once it meets the ones the system was made of,
    /// and the 32 sets of conditions make the fifteen systems of `ALL`.
    #[test]
    fn a_system_holds_what_follows_on_every_frame() {
        let mut systems = Vec::new();
        for set in 0..32_u8 {
            let given: Vec<Condition> = Condition::ALL
                .iter()
                .copied()
                .filter(|c| set & c.bit() != 0)
                .collect();
            let system = System::of(given.iter().copied());
            for &condition in Condition::ALL {
                let follows = (1..=3_u32).all(|n| {
                    (0..1_u32 << (n * n))
                        .filter(|&r| given.iter().all(|&c| meets(r, n, c)))
                        .all(|r| meets(r, n, condition))
                });
                assert_eq!(system.has(condition), follows, "{given:?}: {condition}");
            }
            if !systems.contains(&system) {
                systems.push(system);
            }
        }
        assert_eq!(systems.len(), System::ALL.len());
        assert!(System::ALL.iter().all(|system| systems.contains(system)));
    }

    /// Every system reads back from its name, the other usual names read
    /// as the same systems, and a name that is none is refused.
    #[test]
    fn names_read_as_their_systems() {
        assert_eq!(System::ALL.len(), System::NAMES.len());
        for &system in System::ALL {
            assert_eq!(system.name().parse::<System>().unwrap(), system);
            assert_eq!(
                system.name().to_lowercase().parse::<System>().unwrap(),
                system
            );
        }
        for (name, system) in [
            ("KD", System::D),
            ("KT", System::T),
            ("KTB", System::B),
            ("KT4", System::S4),
            ("KT5", System::S5),
            ("KDB4", System::S5),
            ("KT45", System::S5),
            ("KB4", System::KB5),
            ("K4B", System::KB5),
            ("D45", System::KD45),
            ("DB", System::KDB),
            ("s4", System::S4),
        ] {
            assert_eq!(name.parse::<System>().unwrap(), system, "{name}");
        }
        for name in ["", "S3", "K6", "T4", "GL", "KTé"] {
            assert!(
                matches!(
                    name.parse::<System>(),
                    Err(Error::UnknownName { known, .. }) if known == System::NAMES
                ),
                "{name}"
            );
        }
        assert_eq!(System::default(), System::K);
        assert!(System::S5.extends(System::S4) && !System::S4.extends(System::KB));
        assert_eq!(
            System::S4.conditions().collect::<Vec<_>>(),
            [
                Condition::Serial,
                Condition::Reflexive,
                Condition::Transitive
            ]
        );
        for &condition in Condition::ALL {
            assert_eq!(condition.name().parse::<Condition>().unwrap(), condition);
        }
    }
}
