// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The kinds and operations of lambda-terms: a term checked, reduced and
//! typed at its principal type, a type's term found, and a type's terms
//! listed and counted.

use serde::{Deserialize, Serialize};

use super::ordinary::checked;
#[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
use super::promises::RENDERS;
use super::promises::{DECIDES, EXACT, PRINCIPAL, REDUCES, WELL_TYPED};
use super::{
    Call, Check, Class, Cost, Kind, Operation, Parameter, Part, Produced, Role, Status, Suite,
    document, object, parameters, written,
};
use crate::lambda::{self, Calculus, InhabitOptions, Inhabitation, ReduceOptions, Term, Typing};
use crate::ordinary::{Logic, Sequent};
use crate::{Error, Limits, Progress};

/// The suite of lambda-terms.
pub(super) static LAMBDA: Lambda = Lambda;

/// The suite of lambda-terms.
pub(super) struct Lambda;

/// Its kinds.
static KINDS: [Kind; 5] = [
    Kind {
        name: "lambda.term",
        role: Role::Statement,
        form: "typed_term",
        level: Some(23),
        stored: true,
        label: "A lambda-term with its free variables' types, claimed well typed; an untyped one \
                with its free variables, claimed to use them as its calculus allows",
    },
    Kind {
        name: "lambda.typing",
        role: Role::Evidence,
        form: "lambda_typing",
        level: Some(23),
        stored: true,
        label: "A lambda-term checked: the type of every node, and its judgement",
    },
    Kind {
        name: "lambda.inhabitation",
        role: Role::Evidence,
        form: "lambda_inhabitation",
        level: Some(24),
        stored: true,
        label: "Whether a type has a term: the term, or that it has none",
    },
    Kind {
        name: "lambda.inhabitants",
        role: Role::Evidence,
        form: "lambda_inhabitants",
        level: Some(24),
        stored: true,
        label: "A type's terms, listed and counted",
    },
    Kind {
        name: "lambda.reduction",
        role: Role::Evidence,
        form: "lambda_reduction",
        level: Some(30),
        stored: true,
        label: "A term reduced by β step by step: the term reached, each step's redex and how \
                the reduction ended",
    },
];

/// The settings keys a check of a term reads: the term's sizes, the
/// check's bounds, and how its judgement is written.
static CHECK_READS: [&str; 5] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.derivation_bytes",
    "limits.work",
    "lambda",
];

/// The settings keys a search for terms reads: the type's sizes, and the
/// bounds of the engines, the read-back and the checker.
static INHABIT_READS: [&str; 5] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.derivation_bytes",
    "limits.work",
    "limits.terms",
];

/// The settings keys a drawing or an export of a typing derivation
/// reads: the check's bounds, the drawing's, how a judgement is written,
/// and the formats' styles.
#[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
static DRAW_READS: [&str; 8] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.derivation_bytes",
    "limits.work",
    "lambda",
    "styles.svg",
    "styles.latex",
    "styles.typst",
];

/// Its operations.
static OPERATIONS: &[Operation] = &[
    Operation {
        name: "lambda.check",
        from: &[&["lambda.term"]],
        to: "lambda.typing",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &CHECK_READS,
        promise: &WELL_TYPED,
        label: "The term checked",
    },
    Operation {
        name: "lambda.reduce",
        from: &[&["lambda.term"]],
        to: "lambda.reduction",
        class: Class::Derived,
        cost: Cost::Search,
        parameters: &[
            Parameter {
                name: "strategy",
                values: lambda::Strategy::NAMES,
                default: Some("normal_order"),
                description: "Which redex each step reduces: the leftmost outermost, or the \
                              leftmost innermost",
            },
            Parameter {
                name: "steps",
                values: &[],
                default: Some("1000"),
                description: "The most steps taken, or null for no bound",
            },
            Parameter {
                name: "nodes",
                values: &[],
                default: Some("100000"),
                description: "The most nodes a term a step makes may have, or null for no bound",
            },
        ],
        reads: &CHECK_READS,
        promise: &REDUCES,
        label: "The term reduced step by step",
    },
    Operation {
        name: "lambda.infer",
        from: &[&["lambda.term"]],
        to: "lambda.term",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &CHECK_READS,
        promise: &PRINCIPAL,
        label: "The untyped term typed at its principal type",
    },
    Operation {
        name: "lambda.inhabit",
        from: &[&["ordinary.sequent"]],
        to: "lambda.inhabitation",
        class: Class::Derived,
        cost: Cost::Search,
        parameters: &[],
        reads: &INHABIT_READS,
        promise: &DECIDES,
        label: "A term of the type, or that it has none",
    },
    Operation {
        name: "lambda.enumerate",
        from: &[&["ordinary.sequent"]],
        to: "lambda.inhabitants",
        class: Class::Derived,
        cost: Cost::Search,
        parameters: &[
            Parameter {
                name: "depth",
                values: &[],
                default: Some("8"),
                description: "How deep a term may nest: variables applied inside each other's \
                              arguments",
            },
            Parameter {
                name: "max_terms",
                values: &[],
                default: Some("32"),
                description: "The most terms listed; the count goes on past them",
            },
        ],
        reads: &INHABIT_READS,
        promise: &WELL_TYPED,
        label: "The type's terms, listed and counted",
    },
    #[cfg(feature = "svg")]
    Operation {
        name: "lambda.draw",
        from: &[&["lambda.typing"]],
        to: "drawing",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &DRAW_READS,
        promise: &RENDERS,
        label: "The typing derivation drawn",
    },
    #[cfg(any(feature = "latex", feature = "typst"))]
    Operation {
        name: "lambda.export",
        from: &[&["lambda.typing"]],
        to: "text",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[Parameter {
            name: "format",
            values: &["latex", "typst"],
            default: Some("latex"),
            description: "The format",
        }],
        reads: &DRAW_READS,
        promise: &RENDERS,
        label: "The typing derivation in a format",
    },
    Operation {
        name: "lambda.duplicate",
        from: &[&["lambda.term"]],
        to: "lambda.term",
        class: Class::Copy,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::SIZES,
        promise: &EXACT,
        label: "A copy",
    },
];

/// A statement as written: the term's document and how it was typed.
#[derive(Serialize)]
struct Statement<'a> {
    /// The term's document.
    #[serde(flatten)]
    term: &'a Term,
    /// The term as typed.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
}

/// Reads a term's text in a calculus and writes it as a statement.
pub(super) fn typed_statement(
    text: &str,
    calculus: Calculus,
    limits: &Limits,
) -> Result<String, Error> {
    let term = Term::parse(text, calculus, limits)?;
    written(&Statement {
        term: &term,
        text: Some(text),
    })
}

/// A checked term as written: the term's document, the type of every
/// node and the judgement.
#[derive(Serialize)]
struct Typed<'a> {
    /// The term's document.
    #[serde(flatten)]
    term: &'a Term,
    /// The type of every node.
    typing: &'a Typing,
    /// The judgement with every type.
    judgement: String,
}

/// What a check reads of a checked term's value besides the term.
#[derive(Deserialize)]
struct TypedKeys {
    /// The type of every node.
    typing: serde_json::Value,
}

/// What a check and a summary read of an answer about a type.
#[derive(Deserialize)]
struct Answer {
    /// `inhabited`, `empty` or `unknown`.
    #[serde(default)]
    verdict: Option<String>,
    /// The term, for `inhabited`.
    #[serde(default)]
    term: Option<serde_json::Value>,
    /// The count, for a list of terms.
    #[serde(default)]
    count: Option<CountForm>,
    /// The terms listed.
    #[serde(default)]
    terms: Vec<serde_json::Value>,
}

/// A count as written.
#[derive(Deserialize)]
struct CountForm {
    /// `exactly`, `infinite` or `at_least`.
    kind: String,
    /// The number, where there is one.
    #[serde(default)]
    count: Option<u64>,
}

/// What a summary reads of a reduction.
#[derive(Deserialize)]
struct Reduced {
    /// The steps.
    steps: Vec<serde_json::Value>,
    /// How it ended.
    end: String,
}

/// The parameters of `lambda.export`.
#[cfg(any(feature = "latex", feature = "typst"))]
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Format {
    /// The format, `latex` by default.
    format: Option<String>,
}

/// Returns the typing derivation of a checked term's value, laid out
/// for the exports.
#[cfg(any(feature = "latex", feature = "typst", feature = "svg"))]
fn drawing_of(
    value: &str,
    settings: &crate::Settings,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<lambda::Drawing, Error> {
    let term: Term = document(value, &settings.limits)?;
    lambda::Drawing::new(&term, &settings.lambda, &settings.limits, stop)
}

/// The parameters of `lambda.enumerate`.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Bounds {
    /// How deep a term may nest.
    depth: Option<u32>,
    /// The most terms listed.
    max_terms: Option<u32>,
}

/// Reads a type: an ordinary statement claimed in intuitionistic logic,
/// whose formula right of `⊢` is the type and whose formulas left of it
/// are the free variables' types.
fn type_of(value: &str, limits: &Limits) -> Result<Sequent, Error> {
    let (sequent, logic) = super::ordinary::statement(value, limits)?;
    if logic == Logic::Intuitionistic {
        Ok(sequent)
    } else {
        Err(Error::InvalidOption {
            key: "logic",
            message: format!(
                "a type is a formula of intuitionistic logic, not of {} logic",
                logic.name()
            ),
        })
    }
}

/// Reads a term out of a JSON value.
fn term_in(value: &serde_json::Value, limits: &Limits) -> Result<Term, Error> {
    document(&value.to_string(), limits)
}

/// Checks a term found for a type: well typed, and of that type with
/// those free variables.
fn holds(
    term: &Term,
    sequent: &Sequent,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Part, Error> {
    if term.calculus() != Calculus::Simple {
        return Ok(Part {
            name: "calculus".to_owned(),
            status: Status::Failed,
            verdict: None,
        });
    }
    let part = checked("typing", term.check(limits, &mut *stop).map(drop))?;
    if part.status != Status::Held {
        return Ok(part);
    }
    Ok(Part {
        name: "type".to_owned(),
        status: if lambda::has_type(term, sequent) {
            Status::Held
        } else {
            Status::Failed
        },
        verdict: None,
    })
}

impl Suite for Lambda {
    fn name(&self) -> &'static str {
        "lambda"
    }

    fn label(&self) -> &'static str {
        "Lambda-terms"
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
            "lambda.term" | "lambda.typing" => {
                let term: Term = document(value, limits)?;
                term.to_string()
            }
            "lambda.reduction" => {
                let reduced: Reduced = object(value, "lambda reduction")?;
                let steps = match reduced.steps.len() {
                    1 => "1 step".to_owned(),
                    n => format!("{n} steps"),
                };
                match reduced.end.as_str() {
                    "normal_form" => format!("Normal form after {steps}"),
                    _ => format!("Not finished after {steps}"),
                }
            }
            "lambda.inhabitation" => {
                let answer: Answer = object(value, "lambda inhabitation")?;
                match answer.verdict.as_deref() {
                    Some("inhabited") => "Has a term".to_owned(),
                    Some("empty") => "Has no term".to_owned(),
                    _ => "Not decided".to_owned(),
                }
            }
            _ => {
                let answer: Answer = object(value, "lambda inhabitants")?;
                let count = answer.count.ok_or_else(|| Error::Json {
                    form: "lambda inhabitants",
                    message: "missing field `count`".to_owned(),
                })?;
                match (count.kind.as_str(), count.count) {
                    ("exactly", Some(1)) => "Exactly 1 term".to_owned(),
                    ("exactly", Some(n)) => format!("Exactly {n} terms"),
                    ("infinite", _) => "Infinitely many terms".to_owned(),
                    (_, n) => format!("At least {} terms", n.unwrap_or(0)),
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
            "lambda.check" => {
                let term: Term = document(call.input(0), limits)?;
                let typing = term.check(limits, &mut *stop)?;
                let mut judgement = String::new();
                let options = settings.lambda.with_types(true);
                term.write_judgement(&mut judgement, &options, limits, &mut *stop)?;
                written(&Typed {
                    term: &term,
                    typing: &typing,
                    judgement,
                })?
            }
            "lambda.reduce" => {
                let term: Term = document(call.input(0), limits)?;
                let options: ReduceOptions = parameters(call)?;
                let reduced = term.reduce(&options, limits, &mut *stop, |_, _| {})?;
                written(&reduced)?
            }
            "lambda.infer" => {
                let term: Term = document(call.input(0), limits)?;
                written(&term.principal_type(limits, &mut *stop)?)?
            }
            "lambda.inhabit" => {
                let sequent = type_of(call.input(0), limits)?;
                let answer = lambda::inhabit(&sequent, limits, &mut *stop)?;
                let limit = match &answer {
                    Inhabitation::Unknown(reason) => Some(super::Limit {
                        reason: reason.name(),
                        setting: reason.setting(),
                    }),
                    _ => None,
                };
                return Ok(Produced {
                    value: written(&answer)?,
                    limit,
                });
            }
            "lambda.enumerate" => {
                let sequent = type_of(call.input(0), limits)?;
                let bounds: Bounds = parameters(call)?;
                let defaults = InhabitOptions::default();
                let options = defaults
                    .with_depth(bounds.depth.unwrap_or(defaults.depth))
                    .with_max_terms(bounds.max_terms.unwrap_or(defaults.max_terms));
                let found = lambda::enumerate(&sequent, &options, limits, &mut *stop)?;
                written(&found)?
            }
            #[cfg(feature = "svg")]
            "lambda.draw" => {
                let drawing = drawing_of(call.input(0), settings, &mut *stop)?;
                let mut svg = String::new();
                crate::export::svg::write(&drawing, &settings.styles.svg, &mut svg, &mut *stop)?;
                written(&super::shared::Drawing { svg })?
            }
            #[cfg(any(feature = "latex", feature = "typst"))]
            "lambda.export" => {
                let drawing = drawing_of(call.input(0), settings, &mut *stop)?;
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
                            known: &["latex", "typst"],
                        });
                    }
                }
                written(&super::shared::Text { format, text })?
            }
            // The copy: the input as it was given.
            _ => {
                document::<Term>(call.input(0), limits)?;
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
        match (call.operation.name, promise.name) {
            ("lambda.check", "well_typed") => {
                let given: Term = document(call.input(0), limits)?;
                let term: Term = document(output, limits)?;
                if term != given {
                    return Ok(Check::whole(promise, Status::Failed));
                }
                let typing = match term.check(limits, &mut *stop) {
                    Ok(typing) => typing,
                    Err(error) => {
                        let part = checked("typing", Err(error))?;
                        return Ok(Check::whole(promise, part.status));
                    }
                };
                let keys: TypedKeys = object(output, "lambda typing")?;
                let again = serde_json::to_value(&typing).ok();
                let status = if again.as_ref() == Some(&keys.typing) {
                    Status::Held
                } else {
                    Status::Failed
                };
                Ok(Check::whole(promise, status))
            }
            ("lambda.reduce", "reduces") => {
                let term: Term = document(call.input(0), limits)?;
                let options: ReduceOptions = parameters(call)?;
                let again = match term.reduce(&options, limits, &mut *stop, |_, _| {}) {
                    Ok(again) => written(&again)?,
                    Err(error) => {
                        let part = checked("reduction", Err(error))?;
                        return Ok(Check::whole(promise, part.status));
                    }
                };
                let same = serde_json::from_str::<serde_json::Value>(output).ok()
                    == serde_json::from_str::<serde_json::Value>(&again).ok();
                let status = if same { Status::Held } else { Status::Failed };
                Ok(Check::whole(promise, status))
            }
            ("lambda.infer", "principal") => {
                let given: Term = document(call.input(0), limits)?;
                let typed: Term = document(output, limits)?;
                let part = checked("typing", typed.check(limits, &mut *stop).map(drop))?;
                if part.status != Status::Held {
                    return Ok(Check::whole(promise, part.status));
                }
                let status = match given.principal_type(limits, &mut *stop) {
                    Ok(again) if again == typed => Status::Held,
                    Ok(_) => Status::Failed,
                    Err(error) => checked("principal type", Err(error))?.status,
                };
                Ok(Check::whole(promise, status))
            }
            ("lambda.inhabit", "decides") => {
                let sequent = type_of(call.input(0), limits)?;
                let answer: Answer = object(output, "lambda inhabitation")?;
                let status = match (answer.verdict.as_deref(), &answer.term) {
                    (Some("inhabited"), Some(term)) => {
                        holds(&term_in(term, limits)?, &sequent, limits, stop)?.status
                    }
                    // The engines checked the countermodel, or LJT's search
                    // was exhaustive; the value holds no evidence to check.
                    (Some("empty"), _) => Status::Recorded,
                    (Some("unknown"), _) => Status::None,
                    _ => Status::Failed,
                };
                Ok(Check::whole(promise, status))
            }
            ("lambda.enumerate", "well_typed") => {
                let sequent = type_of(call.input(0), limits)?;
                let answer: Answer = object(output, "lambda inhabitants")?;
                let mut status = Status::Held;
                for term in &answer.terms {
                    let part = holds(&term_in(term, limits)?, &sequent, limits, stop)?;
                    if part.status != Status::Held {
                        status = part.status;
                        break;
                    }
                }
                Ok(Check::whole(promise, status))
            }
            _ => Ok(Check::whole(promise, Status::None)),
        }
    }
}
