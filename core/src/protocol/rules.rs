// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The kinds and operations of calculi given as data.

use serde::{Deserialize, Serialize};

use super::ordinary::checked;
#[cfg(any(feature = "svg", feature = "latex", feature = "typst"))]
use super::promises::RENDERS;
use super::promises::{EXACT, PROVES};
#[cfg(any(feature = "latex", feature = "typst"))]
use super::shared::Text;
use super::{
    Call, Check, Class, Cost, Kind, Operation, Produced, Role, Status, Suite, document, object,
    written,
};
#[cfg(any(feature = "latex", feature = "typst"))]
use super::{Parameter, parameters};
use crate::rules::{Calculus, Derivation, Interactive};
use crate::{Error, Limits, Progress};

/// The suite of calculi given as data.
pub(super) static RULES: Rules = Rules;

/// The suite of calculi given as data.
pub(super) struct Rules;

/// Its kinds.
static KINDS: [Kind; 4] = [
    Kind {
        name: "rules.calculus",
        role: Role::Definition,
        form: "calculus",
        level: Some(15),
        stored: true,
        label: "A calculus given as a rule file",
    },
    Kind {
        name: "rules.sequent",
        role: Role::Statement,
        form: "rules_statement",
        level: None,
        stored: true,
        label: "A sequent to derive in a calculus given as data, with its assumptions",
    },
    Kind {
        name: "rules.session",
        role: Role::Work,
        form: "rules_session",
        level: Some(17),
        stored: true,
        label: "A derivation in progress in a calculus given as data",
    },
    Kind {
        name: "rules.derivation",
        role: Role::Evidence,
        form: "rules_derivation",
        level: Some(15),
        stored: true,
        label: "A derivation in a calculus given as data",
    },
];

/// The settings keys a session's derivation reads.
static FINISH_READS: [&str; 4] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.work",
];

/// The settings keys a draw reads.
#[cfg(feature = "svg")]
static DRAW_READS: [&str; 8] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "view",
    "styles.svg",
    "styles.fitch",
];

/// The settings keys an export reads.
#[cfg(any(feature = "latex", feature = "typst"))]
static EXPORT_READS: [&str; 9] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "view",
    "styles.latex",
    "styles.typst",
    "styles.fitch",
];

/// Its operations.
static OPERATIONS: &[Operation] = &[
    Operation {
        name: "rules.start",
        from: &[&["rules.calculus"], &["rules.sequent"]],
        to: "rules.session",
        class: Class::Work,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::START,
        promise: &EXACT,
        label: "A derivation step by step",
    },
    Operation {
        name: "rules.derivation",
        from: &[&["rules.session"]],
        to: "rules.derivation",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &FINISH_READS,
        promise: &PROVES,
        label: "The derivation",
    },
    Operation {
        name: "rules.from_ordinary",
        from: &[&["ordinary.derivation"]],
        to: "rules.derivation",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &FINISH_READS,
        promise: &PROVES,
        label: "The derivation in LK, LJ or LM as rule files",
    },
    #[cfg(feature = "svg")]
    Operation {
        name: "rules.draw",
        from: &[&["rules.session"]],
        to: "drawing",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &DRAW_READS,
        promise: &RENDERS,
        label: "A drawing",
    },
    #[cfg(any(feature = "latex", feature = "typst"))]
    Operation {
        name: "rules.export",
        from: &[&["rules.session"]],
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
        label: "An export",
    },
    Operation {
        name: "rules.duplicate",
        from: &[&["rules.sequent"]],
        to: "rules.sequent",
        class: Class::Copy,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::SIZES,
        promise: &EXACT,
        label: "A copy",
    },
];

/// A sequent of a calculus given as data: text the calculus reads.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Statement {
    /// The sequent to derive.
    goal: String,
    /// The hypotheses it may be derived from, each a sequent.
    #[serde(default)]
    assumptions: Vec<String>,
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

/// Returns a built-in rule file of LK, LJ or LM by its calculus's name.
fn built_in(name: &str) -> Option<&'static str> {
    match name {
        "LK" => Some(crate::rules::LK),
        "LJ" => Some(crate::rules::LJ),
        "LM" => Some(crate::rules::LM),
        _ => None,
    }
}

/// Writes a session's derivation as the settings' layout draws it, with
/// `write` given the drawable.
#[cfg(any(feature = "svg", feature = "latex", feature = "typst"))]
fn drawn(
    session: &Interactive,
    call: &Call<'_>,
    write: impl FnOnce(crate::export::Drawable<'_>) -> Result<(), Error>,
) -> Result<(), Error> {
    let settings = call.settings;
    let calculus = session.calculus();
    if settings.view.layout == crate::linear::proofs::derivation::Layout::Fitch {
        // A session's Fitch proof opens every subproof with its
        // hypotheses, cited or not.
        let (derivation, _) = session.fitch_derivation()?;
        let fitch = crate::rules::Fitch::new(
            calculus,
            &derivation,
            &settings.styles.fitch,
            &settings.limits,
        )?;
        write((&fitch).into())
    } else {
        let derivation = session.derivation()?;
        let drawing = crate::rules::Drawing::new(calculus, &derivation, &settings.limits)?;
        write((&drawing).into())
    }
}

impl Suite for Rules {
    fn name(&self) -> &'static str {
        "rules"
    }

    fn label(&self) -> &'static str {
        "Custom calculus"
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
            "rules.calculus" => document::<Calculus>(value, limits)?.name().to_owned(),
            "rules.sequent" => object::<Statement>(value, "rules statement")?.goal,
            "rules.session" => {
                let session: Interactive = document(value, limits)?;
                match session.goals().len() {
                    0 => "Derived".to_owned(),
                    1 => "1 open goal".to_owned(),
                    goals => format!("{goals} open goals"),
                }
            }
            _ => {
                document::<Derivation>(value, limits)?;
                "A derivation".to_owned()
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
            "rules.start" => {
                let calculus: Calculus = document(call.input(0), limits)?;
                let statement: Statement = object(call.input(1), "rules statement")?;
                let assumptions: Vec<&str> =
                    statement.assumptions.iter().map(String::as_str).collect();
                written(&Interactive::within(
                    calculus,
                    &statement.goal,
                    &assumptions,
                    limits,
                )?)?
            }
            "rules.derivation" => {
                let session: Interactive = document(call.input(0), limits)?;
                written(&session.finished(stop)?)?
            }
            "rules.from_ordinary" => {
                let derivation: crate::ordinary::Derivation = document(call.input(0), limits)?;
                written(&Derivation::from_ordinary(&derivation)?)?
            }
            #[cfg(feature = "svg")]
            "rules.draw" => {
                let session: Interactive = document(call.input(0), limits)?;
                let mut svg = String::new();
                drawn(&session, call, |drawable| {
                    crate::export::svg::write(drawable, &settings.styles.svg, &mut svg, &mut *stop)
                })?;
                written(&super::shared::Drawing { svg })?
            }
            #[cfg(any(feature = "latex", feature = "typst"))]
            "rules.export" => {
                let session: Interactive = document(call.input(0), limits)?;
                let p: Format = parameters(call)?;
                let format = p.format.unwrap_or_else(|| "latex".to_owned());
                let styles = &settings.styles;
                let mut out = String::new();
                drawn(&session, call, |drawable| match format.as_str() {
                    #[cfg(feature = "latex")]
                    "latex" => {
                        crate::export::latex::write(drawable, &styles.latex, &mut out, &mut *stop)
                    }
                    #[cfg(feature = "typst")]
                    "typst" => {
                        crate::export::typst::write(drawable, &styles.typst, &mut out, &mut *stop)
                    }
                    other => Err(Error::UnknownName {
                        what: "format",
                        name: other.into(),
                        known: FORMATS,
                    }),
                })?;
                written(&Text { format, text: out })?
            }
            // The copy: the input as it was given.
            _ => {
                object::<Statement>(call.input(0), "rules statement")?;
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
        if promise.name != "proves" {
            return Ok(Check::whole(promise, Status::None));
        }
        let derivation: Derivation = document(output, limits)?;
        let calculus: Calculus = if call.operation.name == "rules.from_ordinary" {
            let from: crate::ordinary::Derivation = document(call.input(0), limits)?;
            let text = built_in(from.logic().calculus()).ok_or_else(|| Error::UnknownName {
                what: "calculus",
                name: from.logic().calculus().into(),
                known: &["LK", "LJ", "LM"],
            })?;
            document(text, limits)?
        } else {
            let session: Interactive = document(call.input(0), limits)?;
            session.calculus().clone()
        };
        let part = checked("derivation", derivation.check(&calculus, limits, stop))?;
        Ok(Check::whole(promise, part.status))
    }
}
