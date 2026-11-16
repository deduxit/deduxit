// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The kinds and operations of modal logic.

use serde::{Deserialize, Serialize};

use super::ordinary::checked;
#[cfg(any(feature = "svg", feature = "latex", feature = "typst"))]
use super::promises::RENDERS;
use super::promises::{DECIDES, EXACT, PROVES};
#[cfg(any(feature = "latex", feature = "typst"))]
use super::shared::Text;
use super::{
    Call, Check, Class, Cost, Kind, Limit, Operation, Parameter, Produced, Role, Status, Suite,
    document, object, parameters, written,
};
use crate::kripke::Model;
use crate::modal::{self, Evidence, Outcome, Sequent, System, Verdict};
use crate::{Error, Limits, Progress};

/// The suite of modal logic.
pub(super) static MODAL: Modal = Modal;

/// The suite of modal logic.
pub(super) struct Modal;

/// Its kinds.
static KINDS: [Kind; 4] = [
    Kind {
        name: "modal.sequent",
        role: Role::Statement,
        form: "modal_statement",
        level: Some(20),
        stored: true,
        label: "A sequent of modal logic, with the system it is claimed in",
    },
    Kind {
        name: "modal.outcome",
        role: Role::Evidence,
        form: "modal_outcome",
        level: Some(20),
        stored: true,
        label: "A modal sequent decided: its verdict with a derivation or a countermodel",
    },
    Kind {
        name: "modal.model",
        role: Role::Evidence,
        form: "kripke_model",
        level: Some(13),
        stored: true,
        label: "A Kripke model, each world's arrows the worlds it sees",
    },
    Kind {
        name: "modal.evaluation",
        role: Role::Evidence,
        form: "modal_evaluation",
        level: None,
        stored: true,
        label: "A Kripke model checked against a modal sequent",
    },
];

/// The settings keys a decision reads: every bound the search and its
/// evidence keep to.
static DECIDE_READS: [&str; 4] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.derivation_bytes",
    "limits.work",
];

/// The settings keys reading an outcome's derivation reads.
static DERIVATION_READS: [&str; 4] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.work",
];

/// The settings keys an evaluation reads: the readers' sizes, and the
/// bounds of the frame check and the model check.
static EVALUATE_READS: [&str; 4] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.work",
];

/// The settings keys an export reads.
#[cfg(any(feature = "latex", feature = "typst"))]
static EXPORT_READS: [&str; 7] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "styles.latex",
    "styles.typst",
];

/// The settings keys a draw reads.
#[cfg(feature = "svg")]
static DRAW_READS: [&str; 6] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "styles.svg",
];

/// Its operations.
static OPERATIONS: &[Operation] = &[
    Operation {
        name: "modal.decide",
        from: &[&["modal.sequent"]],
        to: "modal.outcome",
        class: Class::Derived,
        cost: Cost::Search,
        parameters: &[],
        reads: &DECIDE_READS,
        promise: &DECIDES,
        label: "A decision",
    },
    Operation {
        name: "modal.countermodel",
        from: &[&["modal.outcome"]],
        to: "modal.model",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &DERIVATION_READS,
        promise: &EXACT,
        label: "The countermodel",
    },
    Operation {
        name: "modal.evaluate",
        from: &[&["modal.sequent"], &["modal.model"]],
        to: "modal.evaluation",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[Parameter {
            name: "world",
            values: &[],
            default: None,
            description: "The world to check the sequent at, by its number; every world where \
                          left out",
        }],
        reads: &EVALUATE_READS,
        promise: &EXACT,
        label: "The model checked against the sequent",
    },
    Operation {
        name: "modal.derivation",
        from: &[&["modal.outcome"]],
        to: "rules.derivation",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &DERIVATION_READS,
        promise: &PROVES,
        label: "The derivation in the system's labelled calculus",
    },
    Operation {
        name: "modal.calculus",
        from: &[&["modal.sequent"]],
        to: "rules.calculus",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::SIZES,
        promise: &EXACT,
        label: "The system's labelled calculus as a rule file",
    },
    #[cfg(feature = "svg")]
    Operation {
        name: "modal.draw",
        from: &[&["modal.outcome"]],
        to: "drawing",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &DRAW_READS,
        promise: &RENDERS,
        label: "A drawing: the derivation, or the countermodel",
    },
    #[cfg(any(feature = "latex", feature = "typst"))]
    Operation {
        name: "modal.export",
        from: &[&["modal.outcome"]],
        to: "text",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[Parameter {
            name: "format",
            values: &["latex", "typst"],
            default: Some("latex"),
            description: "The format",
        }],
        reads: &EXPORT_READS,
        promise: &RENDERS,
        label: "The derivation in a format",
    },
    Operation {
        name: "modal.duplicate",
        from: &[&["modal.sequent"]],
        to: "modal.sequent",
        class: Class::Copy,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::SIZES,
        promise: &EXACT,
        label: "A copy",
    },
];

/// What a statement claims beside its sequent: the system.
#[derive(Deserialize)]
struct Claim {
    /// The system.
    system: System,
}

/// A statement as written: the sequent's document with its system and
/// how it was typed.
#[derive(Serialize)]
struct Statement<'a> {
    /// The sequent's document.
    #[serde(flatten)]
    sequent: &'a Sequent,
    /// The system it is claimed in.
    system: System,
    /// The sequent as typed.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
}

/// Writes a statement with the text it was typed as.
pub(super) fn typed_statement(
    sequent: &Sequent,
    system: System,
    text: Option<&str>,
) -> Result<String, Error> {
    written(&Statement {
        sequent,
        system,
        text,
    })
}

/// Reads a statement: its sequent and its system.
fn statement(value: &str, limits: &Limits) -> Result<(Sequent, System), Error> {
    let claim: Claim = object(value, "modal statement")?;
    Ok((document(value, limits)?, claim.system))
}

/// Returns the derivation a valid outcome holds, or the error that says
/// it has none.
fn derivation_of(outcome: &Outcome) -> Result<&crate::rules::Derivation, Error> {
    match &outcome.verdict {
        Verdict::Valid(Evidence::Derivation(derivation)) => Ok(derivation),
        _ => Err(Error::NoDerivation {
            evidence: "no proof: the sequent is not shown valid",
        }),
    }
}

/// A model checked against a statement, as `modal.evaluate` writes it:
/// the frame condition the model fails, or whether the sequent holds and
/// the values where it fails.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Evaluation {
    /// The system the model was checked in.
    system: System,
    /// The world asked about, where one was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    world: Option<u32>,
    /// The frame condition the model's arrows fail; then nothing else was
    /// checked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    frame: Option<FrameFault>,
    /// Whether the sequent holds at the world asked about, or at every
    /// world.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    holds: Option<bool>,
    /// The world where the sequent fails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    failing: Option<u32>,
    /// Each compound subformula's value at that world.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    values: Vec<Value>,
}

/// A frame condition a model fails, with the worlds that show it.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FrameFault {
    /// The condition, by its name: `reflexive`.
    condition: String,
    /// The worlds that show it, in the order the condition's words name
    /// them.
    worlds: Vec<u32>,
}

/// A formula's value at a world.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Value {
    /// The formula, as text.
    formula: String,
    /// Whether it is true there.
    #[serde(rename = "true")]
    holds: bool,
    /// For a false `□A` or a true `◇A`, the world that shows it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    witness: Option<u32>,
}

/// The parameters of an evaluation.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct At {
    /// The world to check the sequent at; every world where absent.
    world: Option<u32>,
}

/// Checks `model` against `sequent` in `system`, at `world` or at every
/// world.
fn evaluate(
    sequent: &Sequent,
    system: System,
    model: &Model,
    world: Option<u32>,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Evaluation, Error> {
    let mut evaluation = Evaluation {
        system,
        world,
        frame: None,
        holds: None,
        failing: None,
        values: Vec::new(),
    };
    if let Some(w) = world {
        model.check_world(w)?;
    }
    if let Some(frame) = model.frame_fault(system, limits, &mut *stop)? {
        evaluation.frame = Some(FrameFault {
            condition: frame.condition.name().to_owned(),
            worlds: frame.worlds,
        });
        return Ok(evaluation);
    }
    let truth = model.truth(sequent, system, limits, &mut *stop)?;
    let failing = match world {
        Some(w) => (!truth.sequent_holds(w)).then_some(w),
        None => truth.failing(),
    };
    evaluation.holds = Some(failing.is_none());
    evaluation.failing = failing;
    if let Some(w) = failing {
        let formulas = sequent.formulas();
        evaluation.values = truth
            .at(w)
            .filter(|&(id, _)| {
                !matches!(
                    formulas.node(id),
                    modal::Node::Atom(_) | modal::Node::True | modal::Node::False
                )
            })
            .map(|(id, holds)| Value {
                formula: formulas.formula(id).to_string(),
                holds,
                witness: truth.witness(sequent, w, id),
            })
            .collect();
    }
    Ok(evaluation)
}

/// The formats an export writes.
#[cfg(any(feature = "latex", feature = "typst"))]
const FORMATS: &[&str] = &[
    #[cfg(feature = "latex")]
    "latex",
    #[cfg(feature = "typst")]
    "typst",
];

/// The parameters of an export.
#[cfg(any(feature = "latex", feature = "typst"))]
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Format {
    /// The format.
    format: Option<String>,
}

/// Returns the countermodel a not-valid outcome holds, or the error that
/// says it has none.
fn countermodel_of(outcome: &Outcome) -> Result<&Model, Error> {
    outcome
        .countermodel
        .as_ref()
        .map(|countermodel| &countermodel.model)
        .ok_or_else(|| Error::InvalidOption {
            key: "inputs",
            message: "modal.countermodel: the outcome holds no countermodel".to_owned(),
        })
}

impl Suite for Modal {
    fn name(&self) -> &'static str {
        "modal"
    }

    fn label(&self) -> &'static str {
        "Modal logic"
    }

    fn kinds(&self) -> &'static [Kind] {
        &KINDS
    }

    fn operations(&self) -> &'static [Operation] {
        OPERATIONS
    }

    fn read(&self, kind: &Kind, value: &str, limits: &Limits) -> Result<(), Error> {
        self.summary(kind, value, limits).map(drop)
    }

    fn summary(&self, kind: &Kind, value: &str, limits: &Limits) -> Result<String, Error> {
        Ok(match kind.name {
            "modal.sequent" => statement(value, limits)?.0.to_string(),
            "modal.model" => {
                let model: Model = document(value, limits)?;
                match model.worlds.len() {
                    1 => "A model of 1 world".to_owned(),
                    worlds => format!("A model of {worlds} worlds"),
                }
            }
            "modal.evaluation" => {
                let evaluation: Evaluation = object(value, "modal evaluation")?;
                match (evaluation.frame, evaluation.failing) {
                    (Some(frame), _) => {
                        format!("No model of {}: not {}", evaluation.system, frame.condition)
                    }
                    (None, Some(w)) => format!("Fails at world {w}"),
                    (None, None) => "Holds".to_owned(),
                }
            }
            _ => {
                let outcome: Outcome = document(value, limits)?;
                match outcome.verdict {
                    Verdict::Valid(_) => "Valid".to_owned(),
                    Verdict::NotValid => "Not valid".to_owned(),
                    _ => "Not decided".to_owned(),
                }
            }
        })
    }

    fn run(
        &self,
        call: &Call<'_>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Produced, Error> {
        let settings = call.settings;
        let limits = &settings.limits;
        let value = match call.operation.name {
            "modal.decide" => {
                let (sequent, system) = statement(call.input(0), limits)?;
                let options = modal::Options::default().with_system(system);
                let outcome = modal::decide_within(&sequent, &options, limits, stop)?;
                let limit = match &outcome.verdict {
                    Verdict::Unknown(reason) => Some(Limit {
                        reason: reason.name(),
                        setting: reason.setting(),
                    }),
                    _ => None,
                };
                return Ok(Produced {
                    value: written(&outcome)?,
                    limit,
                });
            }
            "modal.derivation" => {
                let outcome: Outcome = document(call.input(0), limits)?;
                written(derivation_of(&outcome)?)?
            }
            "modal.countermodel" => {
                let outcome: Outcome = document(call.input(0), limits)?;
                written(countermodel_of(&outcome)?)?
            }
            "modal.evaluate" => {
                let (sequent, system) = statement(call.input(0), limits)?;
                let model: Model = document(call.input(1), limits)?;
                let at: At = parameters(call)?;
                written(&evaluate(&sequent, system, &model, at.world, limits, stop)?)?
            }
            "modal.calculus" => {
                let (_, system) = statement(call.input(0), limits)?;
                written(&modal::calculus(system, limits)?)?
            }
            #[cfg(any(feature = "latex", feature = "typst"))]
            "modal.export" => {
                let outcome: Outcome = document(call.input(0), limits)?;
                let derivation = derivation_of(&outcome)?;
                let calculus = modal::calculus(outcome.system, limits)?;
                let drawing = crate::rules::Drawing::new(&calculus, derivation, limits)?;
                let p: Format = parameters(call)?;
                let format = p.format.unwrap_or_else(|| "latex".to_owned());
                let styles = &settings.styles;
                let mut text = String::new();
                match format.as_str() {
                    #[cfg(feature = "latex")]
                    "latex" => {
                        crate::export::latex::write(
                            &drawing,
                            &styles.latex,
                            &mut text,
                            &mut *stop,
                        )?;
                    }
                    #[cfg(feature = "typst")]
                    "typst" => {
                        crate::export::typst::write(
                            &drawing,
                            &styles.typst,
                            &mut text,
                            &mut *stop,
                        )?;
                    }
                    other => {
                        return Err(Error::UnknownName {
                            what: "format",
                            name: other.into(),
                            known: FORMATS,
                        });
                    }
                }
                written(&Text { format, text })?
            }
            #[cfg(feature = "svg")]
            "modal.draw" => {
                let outcome: Outcome = document(call.input(0), limits)?;
                let style = &settings.styles.svg;
                let svg = match (&outcome.verdict, &outcome.countermodel) {
                    (_, Some(countermodel)) => {
                        crate::export::svg::modal_countermodel(countermodel, style, limits, stop)?
                    }
                    _ => {
                        let derivation = derivation_of(&outcome)?;
                        let calculus = modal::calculus(outcome.system, limits)?;
                        let drawing = crate::rules::Drawing::new(&calculus, derivation, limits)?;
                        let mut svg = String::new();
                        crate::export::svg::write(&drawing, style, &mut svg, &mut *stop)?;
                        svg
                    }
                };
                written(&super::shared::Drawing { svg })?
            }
            // The copy: the input as it was given.
            _ => {
                statement(call.input(0), limits)?;
                call.input(0).to_owned()
            }
        };
        Ok(Produced::done(value))
    }

    fn check(
        &self,
        call: &Call<'_>,
        output: &str,
        _ask: &mut super::Ask<'_>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Check, Error> {
        let promise = call.operation.promise;
        let limits = &call.settings.limits;
        match promise.name {
            "decides" => {
                let outcome: Outcome = document(output, limits)?;
                let status = match (&outcome.verdict, &outcome.countermodel) {
                    (Verdict::Unknown(_), _) => Status::None,
                    (Verdict::NotValid, None) => Status::Recorded,
                    _ => checked("evidence", outcome.check(limits, stop))?.status,
                };
                Ok(Check::whole(promise, status))
            }
            "proves" => {
                let outcome: Outcome = document(call.input(0), limits)?;
                let derivation: crate::rules::Derivation = document(output, limits)?;
                let calculus = modal::calculus(outcome.system, limits)?;
                let part = checked("derivation", derivation.check(&calculus, limits, stop))?;
                Ok(Check::whole(promise, part.status))
            }
            _ => Ok(Check::whole(promise, Status::None)),
        }
    }
}
