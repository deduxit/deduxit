// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The kinds and operations of modal logic.

use serde::{Deserialize, Serialize};

use super::ordinary::checked;
#[cfg(feature = "svg")]
use super::promises::RENDERS;
use super::promises::{DECIDES, EXACT, PROVES};
use super::{
    Call, Check, Class, Cost, Kind, Limit, Operation, Produced, Role, Status, Suite, document,
    object, written,
};
use crate::modal::{self, Evidence, Outcome, Sequent, System, Verdict};
use crate::{Error, Limits, Progress};

/// The suite of modal logic.
pub(super) static MODAL: Modal = Modal;

/// The suite of modal logic.
pub(super) struct Modal;

/// Its kinds.
static KINDS: [Kind; 2] = [
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
