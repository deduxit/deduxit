// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use super::kripke::ModelForm;
use super::rules::DerivationForm;
use super::search::Why;
use crate::Error;
use crate::limits::{Limits, Refusal, Space};
use crate::modal::{
    Countermodel, Evidence, Formulas, Node, NodeId, Outcome, Sequent, Statistics, System, Verdict,
    bound,
};
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for System {
    /// Serializes the system as its name, such as `"S4"`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.name())
    }
}

impl<'a> Deserialize<'a> for System {
    /// Deserializes a system from a name `FromStr` reads, refusing one no
    /// system has.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        <std::borrow::Cow<'a, str>>::deserialize(deserializer)?
            .parse()
            .map_err(wire::fail)
    }
}

/// The serialized form of a modal formula: its connective, with its
/// operands as indices of formulas before it. Its tags are part of the
/// interchange format, and those of an ordinary formula are among them.
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
    /// `□A`
    #[serde(rename = "□")]
    Box(u32),
    /// `◇A`
    #[serde(rename = "◇")]
    Diamond(u32),
}

impl From<Node> for FormulaForm {
    /// Writes a node as its form. The arena's ids are the form's indices.
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
            Node::Box(a) => Self::Box(i(a)),
            Node::Diamond(a) => Self::Diamond(i(a)),
        }
    }
}

/// The serialized form of a modal sequent: the formulas, the atom names
/// and the two sides.
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
    /// Returns the form of the sequent, without its level.
    fn of(sequent: &Sequent) -> Self {
        let ids = |list: &[NodeId]| list.iter().map(|k| k.index() as u32).collect();
        Self {
            version: None,
            formulas: sequent
                .formulas()
                .nodes()
                .iter()
                .map(|&n| FormulaForm::from(n))
                .collect(),
            atoms: sequent.formulas().atom_names().to_vec(),
            left: ids(sequent.left()),
            right: ids(sequent.right()),
        }
    }

    /// Returns the sequent the form holds. Both the form's formulas and
    /// the sequent's unfolding must be within `limits.occurrences`.
    fn within(self, limits: &Limits) -> Result<Sequent, Error> {
        if let Some(limit) = limits.occurrences
            && self.formulas.len() as u64 > limit
        {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: self.formulas.len() as u64,
                limit,
            }));
        }
        let mut formulas = Formulas::default();
        // The id of each index of the form: equal formulas are one id.
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
                FormulaForm::Box(a) => formulas.add(Node::Box(at(a)?))?,
                FormulaForm::Diamond(a) => formulas.add(Node::Diamond(at(a)?))?,
            };
            ids.push(id);
        }
        let side = |indices: &[u32]| -> Result<Vec<NodeId>, Error> {
            indices
                .iter()
                .map(|&k| {
                    ids.get(k as usize).copied().ok_or(Error::IndexOutOfBounds {
                        space: Space::Formula,
                        index: k as usize,
                        len: ids.len(),
                    })
                })
                .collect()
        };
        let (left, right) = (side(&self.left)?, side(&self.right)?);
        bound(&formulas, &left, &right, limits)?;
        Sequent::new(formulas, left, right)
    }
}

/// Returns the level of a document that holds `sequent`: that of the
/// modal forms where its arena has a `□` or a `◇`, used or not, since
/// the form writes every node; else that of ordinary logic, whose
/// sequent it then is.
fn level(sequent: &Sequent) -> u32 {
    if sequent
        .formulas()
        .nodes()
        .iter()
        .any(|node| node.is_modal())
    {
        wire::MODAL
    } else {
        wire::ORDINARY
    }
}

impl Serialize for Sequent {
    /// Serializes the sequent as a document at the level of the modal
    /// forms, or at that of ordinary logic where it has neither `□` nor
    /// `◇`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = SequentForm::of(self);
        form.version = Some(level(self));
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
    const FORM: &'static str = "modal sequent";

    /// Reads a sequent, its formulas and sides within the limits.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        SequentForm::deserialize(deserializer)?
            .within(limits)
            .map_err(wire::fail)
    }
}

/// The verdict of a modal outcome, as written.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum VerdictWord {
    /// `valid`.
    Valid,
    /// `not_valid`.
    NotValid,
    /// `unknown`.
    Unknown,
}

/// A countermodel as an outcome holds it: the model alone, since the
/// outcome holds the sequent and the system.
#[derive(Serialize, Deserialize)]
struct CountermodelForm {
    /// The model.
    model: ModelForm,
}

/// The serialized form of the outcome of deciding a modal sequent.
#[derive(Serialize, Deserialize)]
struct OutcomeForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The crate's version, which wrote the outcome; not read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    deduxit: Option<String>,
    /// `valid`, `not_valid` or `unknown`.
    verdict: VerdictWord,
    /// Why the search could not decide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason: Option<Why>,
    /// The system decided in.
    system: System,
    /// What the search counted.
    #[serde(default)]
    statistics: Statistics,
    /// The sequent decided.
    sequent: SequentForm,
    /// The derivation, for `valid`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    derivation: Option<DerivationForm>,
    /// The countermodel, for `not_valid` where there is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    countermodel: Option<CountermodelForm>,
}

impl Serialize for Outcome {
    /// Serializes the outcome as a document at the level of the modal
    /// forms.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (verdict, reason, derivation) = match &self.verdict {
            Verdict::Valid(Evidence::Derivation(derivation)) => (
                VerdictWord::Valid,
                None,
                Some(DerivationForm::from(&**derivation)),
            ),
            Verdict::NotValid => (VerdictWord::NotValid, None, None),
            Verdict::Unknown(reason) => (VerdictWord::Unknown, Some(Why::from(*reason)), None),
        };
        OutcomeForm {
            version: Some(wire::MODAL),
            deduxit: Some(env!("CARGO_PKG_VERSION").to_owned()),
            verdict,
            reason,
            system: self.system,
            statistics: self.statistics,
            sequent: SequentForm::of(&self.sequent),
            derivation,
            countermodel: self.countermodel.as_ref().map(|c| CountermodelForm {
                model: ModelForm::from(&c.model),
            }),
        }
        .serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Outcome {
    /// Deserializes an outcome within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for Outcome {}

impl Readable for Outcome {
    const FORM: &'static str = "modal outcome";

    /// Reads an outcome: its sequent, derivation and model by their
    /// readers, unchecked; a verdict without the evidence it needs, or
    /// with evidence of another verdict, is refused.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        let form = OutcomeForm::deserialize(deserializer)?;
        outcome(form, limits).map_err(wire::fail)
    }
}

/// Returns the outcome a form holds, within the limits.
fn outcome(form: OutcomeForm, limits: &Limits) -> Result<Outcome, Error> {
    let shape = |message: &str| Error::Json {
        form: "modal outcome",
        message: message.to_owned(),
    };
    let sequent = form.sequent.within(limits)?;
    let verdict = match (form.verdict, form.reason, form.derivation) {
        (VerdictWord::Valid, None, Some(derivation)) => {
            Verdict::Valid(Evidence::Derivation(Box::new(derivation.within(limits)?)))
        }
        (VerdictWord::Valid, ..) => return Err(shape("a valid outcome holds its derivation alone")),
        (VerdictWord::NotValid, None, None) => Verdict::NotValid,
        (VerdictWord::Unknown, Some(reason), None) => Verdict::Unknown(reason.into()),
        (VerdictWord::NotValid | VerdictWord::Unknown, ..) => {
            return Err(shape("an outcome holds a reason for unknown only, a derivation for valid only"));
        }
    };
    let countermodel = match form.countermodel {
        None => None,
        Some(_) if !matches!(verdict, Verdict::NotValid) => {
            return Err(shape("only a not_valid outcome holds a countermodel"));
        }
        Some(c) => Some(Countermodel::new(
            sequent.clone(),
            form.system,
            c.model.within(limits)?,
        )),
    };
    Ok(Outcome {
        verdict,
        sequent,
        system: form.system,
        countermodel,
        statistics: form.statistics,
    })
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;

    /// Reads a modal sequent document within `limits`.
    fn read(json: &str, limits: &Limits) -> Result<Sequent, Error> {
        wire::upgrade(&mut serde_json::Deserializer::from_str(json), limits)
    }

    /// A sequent with `□` or `◇` is written at the modal level and reads
    /// back as the same bytes; one without is written as the ordinary
    /// sequent it is, which reads as a modal one.
    #[test]
    fn a_sequent_is_written_at_its_level_and_reads_back() {
        let limits = Limits::default();
        let sequent: Sequent = "[]p -> p".parse().unwrap();
        let json = serde_json::to_string(&sequent).unwrap();
        let level = wire::MODAL;
        assert_eq!(
            json,
            format!(
                r#"{{"version":{level},"formulas":[{{"V":0}},{{"□":0}},{{"→":[1,0]}}],"atoms":["p"],"left":[],"right":[2]}}"#
            )
        );
        let back = read(&json, &limits).unwrap();
        assert_eq!(serde_json::to_string(&back).unwrap(), json);
        let text = "a /\\ ~b, true |- a <-> b, false";
        let plain = serde_json::to_string(&text.parse::<Sequent>().unwrap()).unwrap();
        let ordinary: crate::ordinary::Sequent = text.parse().unwrap();
        assert_eq!(plain, serde_json::to_string(&ordinary).unwrap());
        assert_eq!(
            read(&plain, &limits).unwrap().to_string(),
            ordinary.to_string()
        );
        let diamond: Sequent = "<>a |- [](a \\/ b)".parse().unwrap();
        let json = serde_json::to_string(&diamond).unwrap();
        assert_eq!(read(&json, &limits).unwrap().to_string(), "◇a ⊢ □(a ∨ b)");
    }

    /// The reader shares a formula written twice, and refuses an index
    /// past the formulas before it, a keyword as an atom's name and a
    /// sequent past the occurrence bound.
    #[test]
    fn the_reader_checks_what_it_reads() {
        let limits = Limits::default();
        let twice = r#"{"formulas":[{"V":0},{"□":0},{"□":0},{"∧":[1,2]}],"atoms":["p"],"left":[],"right":[3]}"#;
        let sequent = read(twice, &limits).unwrap();
        assert_eq!(sequent.to_string(), "⊢ □p ∧ □p");
        assert_eq!(sequent.formulas().len(), 3);
        let ahead = r#"{"formulas":[{"□":0}],"atoms":[],"left":[],"right":[0]}"#;
        assert!(matches!(
            read(ahead, &limits),
            Err(Error::IndexOutOfBounds {
                space: Space::Formula,
                ..
            })
        ));
        let keyword = r#"{"formulas":[{"V":0}],"atoms":["top"],"left":[],"right":[0]}"#;
        assert!(matches!(
            read(keyword, &limits),
            Err(Error::AtomName { .. })
        ));
        let json = serde_json::to_string(&"[]a, []a, []a".parse::<Sequent>().unwrap()).unwrap();
        let bound = |n| Limits::default().with_occurrences(Some(n));
        assert!(read(&json, &bound(6)).is_ok());
        assert!(matches!(
            read(&json, &bound(5)),
            Err(Error::Refused(Refusal::Occurrences { .. }))
        ));
    }

    /// Every verdict's outcome reads back as the same bytes, its
    /// evidence still passing its check; a verdict without its evidence,
    /// or with another's, is refused.
    #[test]
    fn an_outcome_reads_back() {
        use crate::modal::{Options, calculus, decide_within};
        let limits = Limits::default();
        let read = |json: &str| -> Result<Outcome, Error> {
            wire::upgrade(&mut serde_json::Deserializer::from_str(json), &limits)
        };
        let s4 = Options::default().with_system(System::S4);
        for (text, options, work) in [
            ("[]p -> [][]p", s4, None),
            ("[]p -> [][]p", Options::default(), None),
            ("[]p -> [][]p", s4, Some(2)),
        ] {
            let sequent: Sequent = text.parse().unwrap();
            let bounded = limits.with_work(work);
            let outcome = decide_within(&sequent, &options, &bounded, |_| false).unwrap();
            let json = serde_json::to_string(&outcome).unwrap();
            assert!(json.starts_with(&format!(r#"{{"version":{},"deduxit":"#, wire::MODAL)));
            let back = read(&json).unwrap();
            assert_eq!(serde_json::to_string(&back).unwrap(), json);
            match &back.verdict {
                Verdict::Valid(Evidence::Derivation(d)) => {
                    let calculus = calculus(back.system, &limits).unwrap();
                    assert!(d.check(&calculus, &limits, |_| false).is_ok());
                }
                Verdict::NotValid => {
                    let model = back.countermodel.as_ref().unwrap();
                    assert!(model.check(&limits, |_| false).is_ok());
                }
                Verdict::Unknown(_) => assert!(json.contains(r#""reason":{"kind":"work_limit""#)),
            }
        }
        let valid = serde_json::to_string(
            &decide_within(&"[]p -> [][]p".parse().unwrap(), &s4, &limits, |_| false).unwrap(),
        )
        .unwrap();
        let bare = valid.replace(r#""verdict":"valid""#, r#""verdict":"not_valid""#);
        assert!(matches!(read(&bare), Err(Error::Json { .. })));
        let start = valid.find(r#","derivation""#).unwrap();
        let without = format!("{}}}", &valid[..start]);
        assert!(matches!(read(&without), Err(Error::Json { .. })));
    }

    /// A system is written as its name and read from any of its names.
    #[test]
    fn a_system_is_its_name() {
        assert_eq!(serde_json::to_string(&System::S4).unwrap(), r#""S4""#);
        assert_eq!(
            serde_json::from_str::<System>(r#""KT4""#).unwrap(),
            System::S4
        );
        assert!(serde_json::from_str::<System>(r#""S3""#).is_err());
        assert!(serde_json::from_str::<System>("4").is_err());
    }
}
