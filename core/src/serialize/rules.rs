// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use crate::Error;
use crate::limits::{Limits, Refusal, Space};
use crate::rules::{Associativity, Calculus, Derivation, Discipline, Fixity, Step, TermId};
use crate::rules::{Node, Term};
use crate::wire::{self, Readable};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt::{Formatter, Result as FmtResult};

/// The deepest a pattern of a rule file is nested. No rule of a calculus
/// in use comes near it, and a reader that stops there recurses no deeper
/// whatever deserializer it is given.
pub(crate) const DEPTH: u32 = 64;

/// The serialized form of a calculus: a rule file.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CalculusForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The calculus's name, which a derivation names.
    pub(crate) name: String,
    /// What the calculus is, for a reader.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub(crate) description: String,
    /// The sorts beside `formula`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) sorts: Vec<String>,
    /// The connectives.
    pub(crate) connectives: Vec<ConnectiveForm>,
    /// The zones of a sequent, in the order written.
    pub(crate) sequent: Vec<ZoneForm>,
    /// The schematic variables, by sort.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub(crate) variables: BTreeMap<String, Vec<String>>,
    /// The context variables.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) contexts: Vec<String>,
    /// The rules.
    pub(crate) rules: Vec<RuleForm>,
}

/// The serialized form of a connective.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConnectiveForm {
    /// Its name, which terms and patterns use.
    pub(crate) name: String,
    /// How many terms it takes.
    pub(crate) arity: u32,
    /// The sorts of its arguments, then of the term it makes; absent
    /// where all are `formula`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) sorts: Option<Vec<String>>,
    /// What is printed for it, where that is not its name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) symbol: Option<String>,
    /// Where it is printed among its arguments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) fixity: Option<Fixity>,
    /// How tightly it binds when printed: a higher number binds tighter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) precedence: Option<u32>,
    /// How a chain of it is read without brackets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) associativity: Option<Associativity>,
}

/// The serialized form of a zone of a sequent.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ZoneForm {
    /// Its name, which a rule's sequents use as a key.
    pub(crate) name: String,
    /// The sort of its members; `formula` where absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) sort: Option<String>,
    /// Whether its members are a set, a multiset or a sequence.
    pub(crate) discipline: Discipline,
    /// What is written before it, as `⊢`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) mark: Option<String>,
    /// The fewest members it holds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) least: Option<u32>,
    /// The most members it holds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) most: Option<u32>,
}

/// The serialized form of a rule.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RuleForm {
    /// Its name, which a derivation's nodes use.
    pub(crate) name: String,
    /// Its premises, in order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) premises: Vec<PremiseForm>,
    /// Its conclusion.
    #[serde(deserialize_with = "once")]
    pub(crate) conclusion: SequentForm,
    /// The variables matched by atoms new to the conclusion.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) fresh: Vec<String>,
    /// The variables matched by atoms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) atomic: Vec<String>,
}

/// The serialized form of a schematic sequent: the entries of each zone,
/// by the zone's name. An absent zone is empty.
pub(crate) type SequentForm = BTreeMap<String, Vec<PatternForm>>;

/// The serialized form of a premise: a schematic sequent, and under
/// `discharges` the hypotheses it discharges.
#[derive(Clone, Debug, Default)]
pub(crate) struct PremiseForm {
    /// The sequent.
    pub(crate) sequent: SequentForm,
    /// The hypotheses the premise discharges, each a sequent of written
    /// members.
    pub(crate) discharges: Vec<SequentForm>,
}

/// The key of a premise that holds what it discharges. No zone has this
/// name.
pub(crate) const DISCHARGES: &str = "discharges";

impl Serialize for PremiseForm {
    /// Writes the zones, then `discharges` where there are any.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        for (zone, entries) in &self.sequent {
            map.serialize_entry(zone, entries)?;
        }
        if !self.discharges.is_empty() {
            map.serialize_entry(DISCHARGES, &self.discharges)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for PremiseForm {
    /// Reads the zones and `discharges`, refusing a key written twice.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Reads a premise's map.
        struct Keys;
        impl<'de> Visitor<'de> for Keys {
            type Value = PremiseForm;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("a premise: its zones, and what it discharges")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<PremiseForm, A::Error> {
                let mut premise = PremiseForm::default();
                let mut discharges = false;
                while let Some(key) = map.next_key::<String>()? {
                    let twice = if key == DISCHARGES {
                        let hypotheses: Vec<Once> = map.next_value()?;
                        premise.discharges = hypotheses.into_iter().map(|h| h.0).collect();
                        std::mem::replace(&mut discharges, true)
                    } else {
                        let entries = map.next_value()?;
                        premise.sequent.insert(key.clone(), entries).is_some()
                    };
                    if twice {
                        return Err(serde::de::Error::custom(format!(
                            "the key {key:?} is written twice"
                        )));
                    }
                }
                Ok(premise)
            }
        }
        deserializer.deserialize_map(Keys)
    }
}

/// A schematic sequent read with every zone's key once. A plain map keeps
/// the last of two equal keys, and a rule whose conclusion wrote `"s"`
/// twice would read as the second alone.
struct Once(SequentForm);

impl<'de> Deserialize<'de> for Once {
    /// Reads the zones, refusing a key written twice.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Reads a sequent's map.
        struct Keys;
        impl<'de> Visitor<'de> for Keys {
            type Value = Once;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("a sequent: the entries of each zone, by the zone's name")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Once, A::Error> {
                let mut sequent = SequentForm::new();
                while let Some(key) = map.next_key::<String>()? {
                    let entries = map.next_value()?;
                    if sequent.insert(key.clone(), entries).is_some() {
                        return Err(serde::de::Error::custom(format!(
                            "the key {key:?} is written twice"
                        )));
                    }
                }
                Ok(Once(sequent))
            }
        }
        deserializer.deserialize_map(Keys)
    }
}

/// Reads a schematic sequent with every zone's key once.
fn once<'de, D: Deserializer<'de>>(deserializer: D) -> Result<SequentForm, D::Error> {
    Once::deserialize(deserializer).map(|once| once.0)
}

/// The serialized form of a pattern: a name, or a connective's name
/// followed by the patterns it is applied to, as an array.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum PatternForm {
    /// A variable, a context or a connective without arguments.
    Name(String),
    /// A connective applied to patterns.
    Apply(String, Vec<PatternForm>),
}

impl Serialize for PatternForm {
    /// Writes a name as a string and an application as an array. A
    /// pattern is nested [`DEPTH`] deep at most, so this recursion is
    /// bounded.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Name(name) => serializer.serialize_str(name),
            Self::Apply(connective, arguments) => {
                let mut seq = serializer.serialize_seq(Some(arguments.len() + 1))?;
                seq.serialize_element(connective)?;
                for argument in arguments {
                    seq.serialize_element(argument)?;
                }
                seq.end()
            }
        }
    }
}

/// Reads a pattern that lies this deep in another.
struct Nested(u32);

impl<'de> DeserializeSeed<'de> for Nested {
    type Value = PatternForm;

    /// Reads a string or an array, refusing an array past [`DEPTH`].
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<PatternForm, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Nested {
    type Value = PatternForm;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a name, or an array of a connective's name and its arguments")
    }

    fn visit_str<E: serde::de::Error>(self, name: &str) -> Result<PatternForm, E> {
        Ok(PatternForm::Name(name.to_owned()))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<PatternForm, A::Error> {
        if self.0 >= DEPTH {
            return Err(serde::de::Error::custom(format!(
                "a pattern is nested deeper than {DEPTH}"
            )));
        }
        let connective: String = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::custom("an empty array names no connective"))?;
        let mut arguments = Vec::new();
        while let Some(argument) = seq.next_element_seed(Nested(self.0 + 1))? {
            arguments.push(argument);
        }
        Ok(PatternForm::Apply(connective, arguments))
    }
}

impl<'de> Deserialize<'de> for PatternForm {
    /// Reads a pattern nested [`DEPTH`] deep at most.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Nested(0).deserialize(deserializer)
    }
}

impl Serialize for Calculus {
    /// Serializes the calculus as the rule file it was read from, at
    /// level 15.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = self.form().clone();
        form.version = Some(wire::RULES);
        form.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Calculus {
    /// Deserializes a calculus within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Calculus {}

impl Readable for Calculus {
    const FORM: &'static str = "calculus";

    /// Reads a rule file and checks that it is a calculus.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let mut form = CalculusForm::deserialize(deserializer)?;
        form.version = None;
        Self::from_form(form, limits).map_err(wire::fail)
    }
}

/// The serialized form of a term of a derivation: an atom of sort
/// `formula` as its name, an atom of another sort as `{"atom": name,
/// "sort": sort}`, a connective applied to earlier terms as `[name,
/// index, …]`.
#[derive(Clone, Debug)]
enum TermForm {
    /// An atom: its name and its sort.
    Atom(String, String),
    /// A connective and the indices of its arguments.
    Apply(String, Vec<u32>),
}

/// The serialized form of an atom of a sort other than `formula`.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AtomForm {
    /// The atom's name.
    atom: String,
    /// The name of its sort.
    #[serde(default = "formula")]
    sort: String,
}

/// Returns the name of the sort every calculus has.
fn formula() -> String {
    crate::rules::FORMULA.to_owned()
}

impl Serialize for TermForm {
    /// Writes the term in the shortest of its three forms.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Atom(name, sort) if sort == crate::rules::FORMULA => {
                serializer.serialize_str(name)
            }
            Self::Atom(name, sort) => AtomForm {
                atom: name.clone(),
                sort: sort.clone(),
            }
            .serialize(serializer),
            Self::Apply(connective, arguments) => {
                let mut seq = serializer.serialize_seq(Some(arguments.len() + 1))?;
                seq.serialize_element(connective)?;
                for argument in arguments {
                    seq.serialize_element(argument)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for TermForm {
    /// Reads a string, an object or an array.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Reads a term's three forms.
        struct Forms;
        impl<'de> Visitor<'de> for Forms {
            type Value = TermForm;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("an atom's name, an atom with its sort, or a connective and indices")
            }

            fn visit_str<E: serde::de::Error>(self, name: &str) -> Result<TermForm, E> {
                Ok(TermForm::Atom(name.to_owned(), formula()))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<TermForm, A::Error> {
                let atom =
                    AtomForm::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                Ok(TermForm::Atom(atom.atom, atom.sort))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<TermForm, A::Error> {
                let connective: String = seq.next_element()?.ok_or_else(|| {
                    serde::de::Error::custom("an empty array names no connective")
                })?;
                let mut arguments = Vec::new();
                while let Some(argument) = seq.next_element()? {
                    arguments.push(argument);
                }
                Ok(TermForm::Apply(connective, arguments))
            }
        }
        deserializer.deserialize_any(Forms)
    }
}

/// The serialized form of a node: its sequent as the term indices of
/// each zone, what justifies it, and where it stands.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeForm {
    /// The members of each zone, as indices of terms.
    sequent: Vec<Vec<u32>>,
    /// The rule's name; absent for a hypothesis and an open goal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rule: Option<String>,
    /// Whether the node is a hypothesis.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    hypothesis: bool,
    /// The positions of the members the rule finds by position.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    at: Vec<u32>,
    /// The premises, as indices of nodes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    premises: Vec<u32>,
    /// The node's scope; the root scope where absent.
    #[serde(default, skip_serializing_if = "is_zero")]
    scope: u32,
}

/// Returns whether a scope is the root scope, which is not written.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde's skip_serializing_if passes a reference"
)]
const fn is_zero(scope: &u32) -> bool {
    *scope == 0
}

/// The serialized form of a derivation.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DerivationForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The name of the calculus the derivation is in.
    calculus: String,
    /// The terms, every argument before the term built from it.
    terms: Vec<TermForm>,
    /// The parent of each scope after the root scope.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    scopes: Vec<u32>,
    /// The nodes, premises before conclusions, the root last.
    nodes: Vec<NodeForm>,
}

impl From<&Derivation> for DerivationForm {
    /// Writes the derivation's form.
    fn from(derivation: &Derivation) -> Self {
        let terms = derivation.terms();
        let index = |id: &TermId| id.index() as u32;
        Self {
            version: None,
            calculus: derivation.calculus().to_owned(),
            terms: (0..terms.len())
                .map(|n| match terms.term(TermId::new(n as u32)) {
                    Term::Atom { name, sort } => TermForm::Atom(name.to_owned(), sort.to_owned()),
                    Term::Apply {
                        connective,
                        arguments,
                    } => TermForm::Apply(
                        connective.to_owned(),
                        arguments.iter().map(index).collect(),
                    ),
                })
                .collect(),
            scopes: derivation.scope_parents().iter().skip(1).copied().collect(),
            nodes: derivation
                .nodes()
                .map(|node| NodeForm {
                    sequent: (0..derivation.zones())
                        .map(|zone| node.zone(zone).iter().map(index).collect())
                        .collect(),
                    rule: match node.step() {
                        Step::Rule(name) => Some(name.to_owned()),
                        Step::Hypothesis | Step::Open => None,
                    },
                    hypothesis: node.step() == Step::Hypothesis,
                    at: node.positions().to_vec(),
                    premises: node.premises().iter().map(|p| p.index() as u32).collect(),
                    scope: node.scope().index() as u32,
                })
                .collect(),
        }
    }
}

impl DerivationForm {
    /// Returns the derivation the form holds, its terms within
    /// `limits.occurrences`. Every index is checked; whether it is a
    /// derivation of its calculus is for [`Derivation::check`] to say.
    fn within(self, limits: &Limits) -> Result<Derivation, Error> {
        if let Some(limit) = limits.occurrences
            && self.terms.len() as u64 > limit
        {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: self.terms.len() as u64,
                limit,
            }));
        }
        let zones = self.nodes.first().map_or(0, |node| node.sequent.len());
        let mut derivation = Derivation::new(&self.calculus, zones);
        // Equal terms of the document are one term of the arena.
        let mut ids: Vec<TermId> = Vec::with_capacity(self.terms.len());
        let at = |ids: &[TermId], index: u32| {
            ids.get(index as usize)
                .copied()
                .ok_or(Error::IndexOutOfBounds {
                    space: Space::Term,
                    index: index as usize,
                    len: ids.len(),
                })
        };
        for form in &self.terms {
            let id = match form {
                TermForm::Atom(name, sort) => derivation.terms_mut().atom(name, sort)?,
                TermForm::Apply(connective, arguments) => {
                    let arguments = arguments
                        .iter()
                        .map(|&index| at(&ids, index))
                        .collect::<Result<Vec<_>, _>>()?;
                    derivation.terms_mut().apply(connective, &arguments)?
                }
            };
            ids.push(id);
        }
        for parent in self.scopes {
            derivation.scope(parent)?;
        }
        for form in self.nodes {
            let sequent = form
                .sequent
                .iter()
                .map(|zone| zone.iter().map(|&index| at(&ids, index)).collect())
                .collect::<Result<Vec<Vec<TermId>>, _>>()?;
            let step = match (&form.rule, form.hypothesis) {
                (Some(rule), false) => Step::Rule(rule),
                (None, true) => Step::Hypothesis,
                (None, false) => Step::Open,
                (Some(_), true) => {
                    return Err(Error::Json {
                        form: Derivation::FORM,
                        message: "a node is both a rule's and a hypothesis".to_owned(),
                    });
                }
            };
            derivation.push(Node {
                sequent: &sequent,
                step,
                premises: &form.premises,
                positions: &form.at,
                scope: form.scope,
            })?;
        }
        if derivation.is_empty() {
            return Err(Error::Json {
                form: Derivation::FORM,
                message: "a derivation has at least one node".to_owned(),
            });
        }
        Ok(derivation)
    }
}

impl Serialize for Derivation {
    /// Serializes the derivation as a document at level 15.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = DerivationForm::from(self);
        form.version = Some(wire::RULES);
        form.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Derivation {
    /// Deserializes a derivation within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Derivation {}

impl Readable for Derivation {
    const FORM: &'static str = "derivation";

    /// Reads a derivation and checks its indices.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = DerivationForm::deserialize(deserializer)?;
        form.within(limits).map_err(wire::fail)
    }
}
