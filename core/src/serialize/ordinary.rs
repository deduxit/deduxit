// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use super::search::Why;
use crate::Error;
use crate::limits::{Limits, Refusal, Space};
use crate::ordinary::{
    Derivation, Formulas, Inference, Logic, Node, NodeId, Outcome, Rule, Sequent, Side,
    Translation, Verdict, bound,
};
#[cfg(feature = "interactive")]
use crate::ordinary::{Interactive, Parts};
use crate::proofs::InfId;
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Rule {
    /// Serializes the rule as its name, such as `"→L"`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.name())
    }
}

impl<'a> Deserialize<'a> for Rule {
    /// Deserializes a rule from its name, refusing one no rule has.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let name = <std::borrow::Cow<'a, str>>::deserialize(deserializer)?;
        Rule::ALL
            .into_iter()
            .find(|rule| rule.name() == name)
            .ok_or_else(|| serde::de::Error::custom(format!("no rule of LK or LJ is named {name}")))
    }
}

/// The serialized form of an ordinary formula, by its connective, its
/// operands as indices of formulas before it. Its tags are part of the
/// interchange format.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum FormulaForm {
    /// An atom, by its index among the names.
    #[serde(rename = "V")]
    Atom(u32),
    /// `⊤`
    #[serde(rename = "⊤")]
    True,
    /// `⊥`
    #[serde(rename = "⊥")]
    False,
    /// `¬A`
    #[serde(rename = "¬")]
    Not(u32),
    /// `A ∧ B`
    #[serde(rename = "∧")]
    And(u32, u32),
    /// `A ∨ B`
    #[serde(rename = "∨")]
    Or(u32, u32),
    /// `A → B`
    #[serde(rename = "→")]
    Implies(u32, u32),
    /// `A ↔ B`
    #[serde(rename = "↔")]
    Iff(u32, u32),
}

impl From<Node> for FormulaForm {
    /// Writes a node as its form, the arena's ids being the form's
    /// indices.
    fn from(node: Node) -> Self {
        let i = |k: NodeId| k.index() as u32;
        match node {
            Node::Atom(a) => Self::Atom(a),
            Node::True => Self::True,
            Node::False => Self::False,
            Node::Not(a) => Self::Not(i(a)),
            Node::And(a, b) => Self::And(i(a), i(b)),
            Node::Or(a, b) => Self::Or(i(a), i(b)),
            Node::Implies(a, b) => Self::Implies(i(a), i(b)),
            Node::Iff(a, b) => Self::Iff(i(a), i(b)),
        }
    }
}

/// The serialized form of an ordinary sequent: the formulas, the atom
/// names and the two sides.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SequentForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The formulas, every operand before the formula built from it.
    formulas: Vec<FormulaForm>,
    /// The atom names.
    atoms: Vec<String>,
    /// The hypotheses, as indices of formulas, in the order written.
    left: Vec<u32>,
    /// The formulas right of `⊢`, as indices, in the order written.
    right: Vec<u32>,
}

impl SequentForm {
    /// Returns the form of the sequent `left ⊢ right` over the formulas.
    fn of(formulas: &Formulas, left: &[NodeId], right: &[NodeId]) -> Self {
        let ids = |list: &[NodeId]| list.iter().map(|k| k.index() as u32).collect();
        Self {
            version: None,
            formulas: formulas
                .nodes()
                .iter()
                .map(|&n| FormulaForm::from(n))
                .collect(),
            atoms: formulas.atom_names().to_vec(),
            left: ids(left),
            right: ids(right),
        }
    }

    /// Returns the formulas the form holds, every formula of it within
    /// `limits.occurrences` and the sequent too, with the id in them of
    /// every index of the form and the two sides.
    fn within(self, limits: &Limits) -> Result<(Read, Vec<NodeId>, Vec<NodeId>), Error> {
        if let Some(limit) = limits.occurrences
            && self.formulas.len() as u64 > limit
        {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: self.formulas.len() as u64,
                limit,
            }));
        }
        let mut formulas = Formulas::default();
        let mut ids: Vec<NodeId> = Vec::with_capacity(self.formulas.len());
        for (n, form) in self.formulas.iter().enumerate() {
            let at = |k: u32| {
                ids.get(k as usize).copied().ok_or(Error::IndexOutOfBounds {
                    space: Space::Formula,
                    index: k as usize,
                    len: n,
                })
            };
            let id = match *form {
                FormulaForm::Atom(a) => {
                    let name = self.atoms.get(a as usize).ok_or(Error::IndexOutOfBounds {
                        space: Space::Atom,
                        index: a as usize,
                        len: self.atoms.len(),
                    })?;
                    formulas.atom(name)?
                }
                FormulaForm::True => formulas.add(Node::True)?,
                FormulaForm::False => formulas.add(Node::False)?,
                FormulaForm::Not(a) => formulas.add(Node::Not(at(a)?))?,
                FormulaForm::And(a, b) => formulas.add(Node::And(at(a)?, at(b)?))?,
                FormulaForm::Or(a, b) => formulas.add(Node::Or(at(a)?, at(b)?))?,
                FormulaForm::Implies(a, b) => formulas.add(Node::Implies(at(a)?, at(b)?))?,
                FormulaForm::Iff(a, b) => formulas.add(Node::Iff(at(a)?, at(b)?))?,
            };
            ids.push(id);
        }
        let read = Read { formulas, ids };
        let left = read.ids(&self.left)?;
        let right = read.ids(&self.right)?;
        bound(&read.formulas, &left, &right, limits)?;
        Ok((read, left, right))
    }
}

/// The formulas a form held, and the id of each of its indices.
struct Read {
    /// The formulas.
    formulas: Formulas,
    /// The id of each index of the form.
    ids: Vec<NodeId>,
}

impl Read {
    /// Returns the ids of the indices given.
    fn ids(&self, indices: &[u32]) -> Result<Vec<NodeId>, Error> {
        indices
            .iter()
            .map(|&k| {
                self.ids
                    .get(k as usize)
                    .copied()
                    .ok_or(Error::IndexOutOfBounds {
                        space: Space::Formula,
                        index: k as usize,
                        len: self.ids.len(),
                    })
            })
            .collect()
    }
}

impl Serialize for Sequent {
    /// Serializes the sequent as a document at the level that added it.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = SequentForm::of(self.formulas(), self.left(), self.right());
        form.version = Some(wire::ORDINARY);
        form.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Sequent {
    /// Deserializes a sequent within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Sequent {}

impl Readable for Sequent {
    const FORM: &'static str = "ordinary sequent";

    /// Reads a sequent, its formulas and sides within the limits.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = SequentForm::deserialize(deserializer)?;
        sequent(form, limits).map_err(wire::fail)
    }
}

/// Returns the sequent a form holds, within the limits.
fn sequent(form: SequentForm, limits: &Limits) -> Result<Sequent, Error> {
    let (read, left, right) = form.within(limits)?;
    Sequent::new(read.formulas, left, right)
}

/// The serialized form of an inference: its two sides as indices of
/// formulas, its rule, the side and position of its principal formula
/// and its premises; an open goal of a proof in progress its sides alone.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct InferenceForm {
    /// The hypotheses.
    left: Vec<u32>,
    /// The formulas right of `⊢`.
    right: Vec<u32>,
    /// The rule, absent for an open goal of a proof in progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rule: Option<Rule>,
    /// The side and position of the principal formula, absent for an
    /// axiom and an open goal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    principal: Option<(Side, usize)>,
    /// The premises, as indices of inferences; absent for a leaf.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    premises: Vec<u32>,
}

impl InferenceForm {
    /// Returns the form of an inference, an open goal without its rule
    /// where `open` says so.
    fn of(inference: &Inference, open: bool) -> Self {
        let ids = |list: &[NodeId]| list.iter().map(|k| k.index() as u32).collect();
        Self {
            left: ids(&inference.left),
            right: ids(&inference.right),
            rule: (!(open && inference.rule == Rule::Open)).then_some(inference.rule),
            principal: inference.principal,
            premises: inference.premises.iter().map(|p| p.get()).collect(),
        }
    }

    /// Returns the inference the form holds, its formulas' ids those the
    /// read gives, and its premises below `len` and its principal within
    /// its sides.
    fn read(self, read: &Read, len: usize) -> Result<Inference, Error> {
        let premises = self
            .premises
            .iter()
            .map(|&p| match p as usize {
                p if p < len => Ok(InfId::new(p as u32)),
                index => Err(Error::IndexOutOfBounds {
                    space: Space::Inference,
                    index,
                    len,
                }),
            })
            .collect::<Result<_, _>>()?;
        let inference = Inference {
            left: read.ids(&self.left)?,
            right: read.ids(&self.right)?,
            rule: self.rule.unwrap_or(Rule::Open),
            principal: self.principal,
            premises,
        };
        if let Some((side, at)) = inference.principal
            && at >= inference.side(side).len()
        {
            return Err(Error::IndexOutOfBounds {
                space: Space::Formula,
                index: at,
                len: inference.side(side).len(),
            });
        }
        Ok(inference)
    }
}

/// The serialized form of a derivation of LK or LJ: the logic, the
/// sequent with every formula the inferences name, and the inferences,
/// premises before conclusions, the root last.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DerivationForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The logic.
    logic: Logic,
    /// The sequent derived.
    sequent: SequentForm,
    /// The inferences.
    inferences: Vec<InferenceForm>,
}

impl From<&Derivation> for DerivationForm {
    /// Writes the derivation's form, every rule named.
    fn from(derivation: &Derivation) -> Self {
        let (left, right) = derivation.sequent();
        Self {
            version: None,
            logic: derivation.logic(),
            sequent: SequentForm::of(derivation.formulas(), left, right),
            inferences: derivation
                .inferences()
                .iter()
                .map(|i| InferenceForm::of(i, false))
                .collect(),
        }
    }
}

impl DerivationForm {
    /// Returns the derivation the form holds, its sequent within the
    /// limits, in the shape every derivation has: an inference at least,
    /// every premise before its conclusion and the premise of exactly one
    /// inference, the root last. Whether it is a derivation of its logic
    /// is [`Derivation::check`]'s to say.
    fn within(self, limits: &Limits) -> Result<Derivation, Error> {
        let (read, left, right) = self.sequent.within(limits)?;
        let len = self.inferences.len();
        let mut parents = vec![0u8; len];
        let mut inferences = Vec::with_capacity(len);
        for (n, form) in self.inferences.into_iter().enumerate() {
            let inference = form.read(&read, n)?;
            // An open goal is a leaf, which the drawings draw bare.
            if inference.rule == Rule::Open
                && (inference.principal.is_some() || !inference.premises.is_empty())
            {
                return Err(Error::Json {
                    form: "ordinary derivation",
                    message: format!("inference {n} is an open goal with a principal or premises"),
                });
            }
            for p in &inference.premises {
                parents[p.index()] = parents[p.index()].saturating_add(1);
            }
            inferences.push(inference);
        }
        let shaped = parents
            .split_last()
            .is_some_and(|(&root, rest)| root == 0 && rest.iter().all(|&n| n == 1));
        if !shaped {
            return Err(Error::Json {
                form: "ordinary derivation",
                message: "the inferences are no tree with its root last".to_owned(),
            });
        }
        Ok(Derivation::from_parts(
            self.logic,
            read.formulas,
            (left, right),
            inferences,
        ))
    }
}

impl Serialize for Derivation {
    /// Serializes the derivation as a document at the level that added
    /// it.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = DerivationForm::from(self);
        form.version = Some(wire::ORDINARY);
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
    const FORM: &'static str = "ordinary derivation";

    /// Reads a derivation in the shape every derivation has, its sequent
    /// within the limits.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = DerivationForm::deserialize(deserializer)?;
        form.within(limits).map_err(wire::fail)
    }
}

/// The serialized form of the outcome of deciding an ordinary sequent:
/// the verdict, the logic and translation, the image's search, and the
/// derivation of a valid sequent.
#[derive(Serialize)]
struct OutcomeForm {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the outcome.
    deduxit: &'static str,
    /// `valid`, `not_valid` or `unknown`.
    verdict: &'static str,
    /// Why the search could not decide.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<Why>,
    /// The logic decided.
    logic: Logic,
    /// The translation the image was made by.
    translation: Translation,
    /// The engine that searched the image.
    engine: crate::search::Engine,
    /// What the image's search cost.
    statistics: crate::search::Statistics,
    /// The derivation, for `valid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    derivation: Option<DerivationForm>,
}

impl Serialize for Outcome {
    /// Serializes the outcome as its verdict, with the reason of an
    /// unknown one and the derivation of a valid sequent.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (verdict, reason, derivation) = match &self.verdict {
            Verdict::Valid(d) => ("valid", None, Some(DerivationForm::from(&**d))),
            Verdict::NotValid => ("not_valid", None, None),
            Verdict::Unknown(r) => ("unknown", Some(Why::from(*r)), None),
        };
        OutcomeForm {
            version: wire::ORDINARY,
            deduxit: env!("CARGO_PKG_VERSION"),
            verdict,
            reason,
            logic: self.logic,
            translation: self.translation,
            engine: self.linear.engine,
            statistics: self.linear.statistics,
            derivation,
        }
        .serialize(serializer)
    }
}

/// The serialized form of a proof in progress of an ordinary sequent: the
/// logic, the sequent with every formula the inferences name, the
/// inferences with the root first, and the goals the steps closed.
#[cfg(feature = "interactive")]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SessionForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The logic.
    logic: Logic,
    /// The sequent being proved.
    sequent: SequentForm,
    /// The inferences.
    inferences: Vec<InferenceForm>,
    /// The steps, each the inference it closed.
    history: Vec<u32>,
}

#[cfg(feature = "interactive")]
impl Serialize for Interactive {
    /// Serializes the proof in progress as a document at the level that
    /// added it.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (logic, formulas, inferences, history) = self.parts();
        let (left, right) = self.sequent();
        SessionForm {
            version: Some(wire::ORDINARY),
            logic,
            sequent: SequentForm::of(formulas, left, right),
            inferences: inferences
                .iter()
                .map(|i| InferenceForm::of(i, true))
                .collect(),
            history: history.iter().map(|h| h.get()).collect(),
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "interactive")]
impl<'a> Deserialize<'a> for Interactive {
    /// Deserializes a proof in progress within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

#[cfg(feature = "interactive")]
impl crate::sealed::Readable for Interactive {}

#[cfg(feature = "interactive")]
impl Readable for Interactive {
    const FORM: &'static str = "ordinary session";

    /// Reads a proof in progress and checks it as one built step by step,
    /// its sequent within the limits.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = SessionForm::deserialize(deserializer)?;
        session(form, limits).map_err(wire::fail)
    }
}

/// Returns the proof in progress a form holds, within the limits.
#[cfg(feature = "interactive")]
fn session(form: SessionForm, limits: &Limits) -> Result<Interactive, Error> {
    let (read, left, right) = form.sequent.within(limits)?;
    let len = form.inferences.len();
    let inferences = form
        .inferences
        .into_iter()
        .map(|i| i.read(&read, len))
        .collect::<Result<Vec<_>, _>>()?;
    if inferences
        .first()
        .is_some_and(|root| root.left != left || root.right != right)
    {
        return Err(Error::InconsistentSession {
            reason: "the root does not conclude the sequent",
        });
    }
    let history = form
        .history
        .iter()
        .map(|&h| match h as usize {
            h if h < len => Ok(InfId::new(h as u32)),
            index => Err(Error::IndexOutOfBounds {
                space: Space::Inference,
                index,
                len,
            }),
        })
        .collect::<Result<_, _>>()?;
    Interactive::from_parts(
        Parts {
            logic: form.logic,
            formulas: read.formulas,
            inferences,
            history,
        },
        limits,
    )
}
