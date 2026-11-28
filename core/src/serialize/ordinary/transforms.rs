// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

#[cfg(all(feature = "interactive", feature = "parse"))]
use super::CountermodelForm;
#[cfg(feature = "sat")]
use super::DerivationForm;
use super::{NestedOutcome, SequentForm};
#[cfg(feature = "sat")]
use crate::ordinary::{Evidence, Verdict};
use crate::ordinary::{Logic, NodeId, Sequent};
#[cfg(feature = "sat")]
use crate::serialize::search::Why;
use crate::wire;
use serde::{Serialize, Serializer};

/// The serialized form of a negative translation compared: the
/// translation, the target logic, the agreement, both sequents and both
/// outcomes.
#[derive(Serialize)]
struct ComparisonForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The translation's name.
    translation: crate::ordinary::negative::Translation,
    /// The logic translated into.
    target: Logic,
    /// `valid`, `not_valid` or `unknown`.
    agreement: crate::ordinary::negative::Agreement,
    /// The classical sequent.
    sequent: SequentForm,
    /// Its translation.
    translated: SequentForm,
    /// The classical sequent decided.
    classical: NestedOutcome<'a>,
    /// The translation decided in the target logic.
    decided: NestedOutcome<'a>,
}

/// The written form of a review of a tableau written by hand.
#[cfg(all(feature = "interactive", feature = "parse"))]
#[derive(Serialize)]
struct ReviewForm<'a> {
    /// The wire level.
    version: u32,
    /// How the tableau stands.
    standing: &'static str,
    /// The sequent its first lines give.
    sequent: SequentForm,
    /// The text line the message is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    text_line: Option<u32>,
    /// The tableau's line the message is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
    /// What the review found.
    message: &'a str,
    /// The model an open branch gives, checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    countermodel: Option<CountermodelForm>,
}

#[cfg(all(feature = "interactive", feature = "parse"))]
impl Serialize for crate::ordinary::TableauReview {
    /// Serializes the review as a document at level 14, written only.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ReviewForm {
            version: wire::TABLES,
            standing: self.standing.word(),
            sequent: SequentForm::of_sequent(&self.sequent),
            text_line: self.text_line,
            line: self.line,
            message: &self.message,
            countermodel: self.countermodel.as_ref().map(CountermodelForm::of),
        }
        .serialize(serializer)
    }
}

impl Serialize for crate::ordinary::negative::Comparison {
    /// Serializes the comparison as a document at level 16.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ComparisonForm {
            version: wire::TRANSFORMS,
            deduxit: env!("CARGO_PKG_VERSION"),
            translation: self.translation,
            target: self.decided.logic,
            agreement: self.agreement,
            sequent: SequentForm::of_sequent(&self.sequent),
            translated: SequentForm::of_sequent(&self.translated),
            classical: NestedOutcome(&self.classical),
            decided: NestedOutcome(&self.decided),
        }
        .serialize(serializer)
    }
}

/// The serialized form of an interpolation: the logic, the partition, the
/// sequent's outcome and the interpolant.
#[derive(Serialize)]
struct InterpolationForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The logic.
    logic: Logic,
    /// The positions of the first half.
    partition: &'a crate::ordinary::interpolant::Partition,
    /// The sequent decided.
    outcome: NestedOutcome<'a>,
    /// The interpolant, where the sequent is valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    interpolant: Option<InterpolantForm<'a>>,
}

/// The serialized form of an interpolant: its text and both halves.
#[derive(Serialize)]
struct InterpolantForm<'a> {
    /// The interpolant as text.
    text: String,
    /// The first half with the interpolant right of `⊢`, decided.
    first: HalfForm<'a>,
    /// The second half with the interpolant as its first hypothesis,
    /// decided.
    second: HalfForm<'a>,
}

/// The serialized form of a half of an interpolation: its sequent and
/// its outcome.
#[derive(Serialize)]
struct HalfForm<'a> {
    /// The half's sequent.
    sequent: SequentForm,
    /// The half decided.
    outcome: NestedOutcome<'a>,
}

impl Serialize for crate::ordinary::interpolant::Interpolation {
    /// Serializes the interpolation as a document at level 16.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let half = |s: &'_ Sequent, outcome| HalfForm {
            sequent: SequentForm::of_sequent(s),
            outcome: NestedOutcome(outcome),
        };
        InterpolationForm {
            version: wire::TRANSFORMS,
            deduxit: env!("CARGO_PKG_VERSION"),
            logic: self.outcome.logic,
            partition: &self.partition,
            outcome: NestedOutcome(&self.outcome),
            interpolant: self.interpolant.as_ref().map(|i| InterpolantForm {
                text: i.formula().to_string(),
                first: half(&i.first, &i.first_outcome),
                second: half(&i.second, &i.second_outcome),
            }),
        }
        .serialize(serializer)
    }
}

/// The written form of a normal form: the form, its promise, the input,
/// the result, the steps and the checks.
#[derive(Serialize)]
struct NormalFormForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The form's name.
    form: crate::ordinary::normal::Form,
    /// What it keeps of the input.
    promise: crate::ordinary::normal::Promise,
    /// Whether every claim of the promise was decided valid.
    checked: bool,
    /// The input.
    sequent: SequentForm,
    /// The normal form, whose formulas the steps index.
    result: SequentForm,
    /// The steps.
    steps: Vec<StepForm>,
    /// The claims decided.
    checks: Vec<CheckedForm<'a>>,
}

/// The written form of a step: its rule and law, and indices of the
/// result's formulas.
#[derive(Serialize)]
struct StepForm {
    /// The rule.
    rule: crate::ordinary::normal::Rule,
    /// The law the rule applies, as text.
    law: &'static str,
    /// The subformula rewritten.
    before: u32,
    /// What replaces it.
    after: u32,
    /// The input's formulas left of `⊢` after the step.
    left: Vec<u32>,
    /// The input's formulas right of `⊢` after the step.
    right: Vec<u32>,
    /// The clauses the step adds as hypotheses.
    added: Vec<u32>,
    /// The term a Skolem or Herbrand step puts in, an index of the result's
    /// terms (`fo.terms`); left out for every other step.
    #[serde(skip_serializing_if = "Option::is_none")]
    term: Option<u32>,
}

/// The written form of a claim decided.
#[derive(Serialize)]
struct CheckedForm<'a> {
    /// The claim.
    claim: crate::ordinary::normal::Claim,
    /// The step whose law the claim is, by its index; left out for the
    /// other claims.
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<usize>,
    /// The sequent whose validity is the claim.
    sequent: SequentForm,
    /// The sequent decided.
    outcome: NestedOutcome<'a>,
}

/// Returns the written form of a normal form's step.
fn step_form(step: &crate::ordinary::normal::Step) -> StepForm {
    let ids = |list: &[NodeId]| list.iter().map(|k| k.index() as u32).collect();
    StepForm {
        rule: step.rule,
        law: step.rule.law(),
        before: step.before.index() as u32,
        after: step.after.index() as u32,
        left: ids(&step.left),
        right: ids(&step.right),
        added: ids(&step.added),
        term: step.term.map(|t| t.index() as u32),
    }
}

/// The written form of Herbrand's theorem applied.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct HerbrandForm<'a> {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// `valid` or `unknown`.
    verdict: &'static str,
    /// Why the search could not decide, for `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<Why>,
    /// The sequent decided.
    sequent: SequentForm,
    /// Its clausal form and the steps that made it.
    clausal: ClausalForm,
    /// The depth of the last terms tried.
    depth: u32,
    /// The instances as a sequent.
    ground: SequentForm,
    /// Each instance's clause, terms and formula.
    instances: Vec<InstanceForm>,
    /// The SAT solver's outcome on the instances.
    #[serde(skip_serializing_if = "Option::is_none")]
    refutation: Option<NestedOutcome<'a>>,
    /// The derivation built from the instances, for `valid` by one.
    #[serde(skip_serializing_if = "Option::is_none")]
    derivation: Option<DerivationForm>,
    /// What the search cost.
    statistics: crate::ordinary::herbrand::Statistics,
}

/// The written form of a clausal form within Herbrand's theorem: the
/// result and the steps.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct ClausalForm {
    /// The clausal form, whose formulas the steps index.
    result: SequentForm,
    /// The steps.
    steps: Vec<StepForm>,
}

/// The written form of a ground instance.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct InstanceForm {
    /// The clause, by its position among the clausal form's hypotheses.
    clause: usize,
    /// The terms put for its variables, the outermost's first, indices of
    /// the ground sequent's terms.
    terms: Vec<u32>,
    /// The instance, an index of the ground sequent's formulas.
    formula: u32,
}

#[cfg(feature = "sat")]
impl Serialize for crate::ordinary::herbrand::Herbrand {
    /// Serializes Herbrand's theorem applied as a document at level 28.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let form = SequentForm::of_sequent;
        let (verdict, reason, derivation) = match &self.verdict {
            Verdict::Valid(Evidence::Derivation(d)) => {
                ("valid", None, Some(DerivationForm::from(&**d)))
            }
            Verdict::Valid(_) => ("valid", None, None),
            Verdict::NotValid => ("not_valid", None, None),
            Verdict::Unknown(r) => ("unknown", Some(Why::from(*r)), None),
        };
        HerbrandForm {
            version: wire::HERBRAND,
            deduxit: env!("CARGO_PKG_VERSION"),
            verdict,
            reason,
            sequent: form(&self.sequent),
            clausal: ClausalForm {
                result: form(&self.clausal.result),
                steps: self.clausal.steps.iter().map(step_form).collect(),
            },
            depth: self.depth,
            ground: form(&self.ground),
            instances: self
                .instances
                .iter()
                .map(|i| InstanceForm {
                    clause: i.clause,
                    terms: i.terms.iter().map(|t| t.index() as u32).collect(),
                    formula: i.formula.index() as u32,
                })
                .collect(),
            refutation: self.refutation.as_ref().map(NestedOutcome),
            derivation,
            statistics: self.statistics,
        }
        .serialize(serializer)
    }
}

/// The written form of first-order resolution.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct FirstOrderForm {
    /// The wire level.
    version: u32,
    /// The crate's version, which wrote the document.
    deduxit: &'static str,
    /// The search, saturation.
    method: crate::ordinary::resolution::Method,
    /// `unsatisfiable`, `saturated` or `unknown`.
    verdict: &'static str,
    /// The bound that ended the search, for `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    bound: Option<crate::ordinary::resolution::Bound>,
    /// The sequent decided.
    sequent: SequentForm,
    /// The clausal form of its negation and its steps.
    clausal: ClausalForm,
    /// The clauses the search started from, as text.
    clauses: Vec<String>,
    /// The refutation's inferences, for `unsatisfiable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    inferences: Option<Vec<InferenceStepForm>>,
    /// The derivation of LK read back, where it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    derivation: Option<DerivationForm>,
    /// Why the read-back did not reach, where it did not.
    #[serde(skip_serializing_if = "Option::is_none")]
    not_read_back: Option<String>,
    /// What the search cost.
    statistics: crate::ordinary::resolution::first_order::Statistics,
}

/// The written form of a first-order inference: its step, the unifier and
/// the clause it made, as text.
#[cfg(feature = "sat")]
#[derive(Serialize)]
struct InferenceStepForm {
    /// The step: `rule` and its positions.
    #[serde(flatten)]
    step: crate::ordinary::resolution::first_order::Step,
    /// The unifier, a binding per line of text.
    unifier: Vec<String>,
    /// The clause made.
    clause: String,
}

#[cfg(feature = "sat")]
impl Serialize for crate::ordinary::resolution::first_order::FirstOrder {
    /// Serializes the decided clauses as a document at level 29.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use crate::ordinary::resolution::first_order::Answer;
        let form = SequentForm::of_sequent;
        let (verdict, bound, inferences, derivation, not_read_back) = match &self.answer {
            Answer::Refuted {
                refutation,
                derivation,
                not_read_back,
            } => {
                let set = refutation.clauses();
                let steps = refutation
                    .inferences()
                    .iter()
                    .map(|i| InferenceStepForm {
                        step: i.step,
                        unifier: set.bindings_text(i),
                        clause: set.text(i.clause),
                    })
                    .collect();
                (
                    "unsatisfiable",
                    None,
                    Some(steps),
                    derivation.as_deref().map(DerivationForm::from),
                    not_read_back.clone(),
                )
            }
            Answer::Saturated => ("saturated", None, None, None, None),
            Answer::Unknown(bound) => ("unknown", Some(*bound), None, None, None),
        };
        FirstOrderForm {
            version: wire::FIRST_ORDER_CLAUSES,
            deduxit: env!("CARGO_PKG_VERSION"),
            method: crate::ordinary::resolution::Method::Saturation,
            verdict,
            bound,
            sequent: form(&self.sequent),
            clausal: ClausalForm {
                result: form(&self.clausal.result),
                steps: self.clausal.steps.iter().map(step_form).collect(),
            },
            clauses: (0..self.clauses.clauses().len())
                .map(|k| self.clauses.text(k))
                .collect(),
            inferences,
            derivation,
            not_read_back,
            statistics: self.statistics,
        }
        .serialize(serializer)
    }
}

impl Serialize for crate::ordinary::normal::NormalForm {
    /// Serializes the normal form as a document at level 21, at 25 where
    /// its sequents are first-order.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let form = SequentForm::of_sequent;
        let version = if self.result.formulas().is_first_order() {
            wire::FORMS.max(wire::ORDINARY_TERMS)
        } else {
            wire::FORMS
        };
        NormalFormForm {
            version,
            deduxit: env!("CARGO_PKG_VERSION"),
            form: self.form,
            promise: self.form.promise(),
            checked: self.is_checked(),
            sequent: form(&self.sequent),
            result: form(&self.result),
            steps: self.steps.iter().map(step_form).collect(),
            checks: self
                .checks
                .iter()
                .map(|c| CheckedForm {
                    claim: c.claim,
                    step: c.step,
                    sequent: form(&c.sequent),
                    outcome: NestedOutcome(&c.outcome),
                })
                .collect(),
        }
        .serialize(serializer)
    }
}
