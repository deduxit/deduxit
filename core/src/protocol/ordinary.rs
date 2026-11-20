// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The ordinary suite's kinds and operations.

use serde::{Deserialize, Serialize};

use super::promises::{
    DECIDES, EQUISATISFIABLE, EQUIVALENT, EXACT, INTERPOLATES, PROVES, RENDERS, SAME_VERDICT,
};
use super::shared::{Term, Text};
use super::{
    Call, Check, Class, Cost, Kind, Limit, Operation, Parameter, Part, Produced, Role, Status,
    Suite, Verdict, document, object, parameters, part, same_verdict, takes, verdict_of, written,
};
use crate::ordinary::interpolant::{self, Partition};
use crate::ordinary::{
    self, Derivation, Interactive, Logic, Method, Outcome, Sequent, Translation, negative, normal,
};
use crate::{Error, ErrorKind, Limits, Progress};

/// The ordinary suite.
pub(super) static ORDINARY: Ordinary = Ordinary;

/// The ordinary suite.
pub(super) struct Ordinary;

/// Its kinds.
static KINDS: [Kind; 5] = [
    Kind {
        name: "ordinary.sequent",
        role: Role::Statement,
        form: "ordinary_statement",
        level: Some(8),
        stored: true,
        label: "A sequent of ordinary logic, claimed in classical, intuitionistic or minimal logic",
    },
    Kind {
        name: "ordinary.session",
        role: Role::Work,
        form: "ordinary_session",
        level: Some(8),
        stored: true,
        label: "A proof of ordinary logic in progress, in LK or LJ",
    },
    Kind {
        name: "ordinary.outcome",
        role: Role::Evidence,
        form: "ordinary_outcome",
        level: Some(12),
        stored: true,
        label: "A sequent decided: its verdict with a derivation or a countermodel",
    },
    Kind {
        name: "ordinary.derivation",
        role: Role::Evidence,
        form: "ordinary_derivation",
        level: Some(8),
        stored: true,
        label: "A derivation of LK or LJ",
    },
    Kind {
        name: "ordinary.interpolant",
        role: Role::Evidence,
        form: "interpolant",
        level: None,
        stored: true,
        label: "A Craig interpolant with its two halves",
    },
];

/// The parameters of a decision.
static DECIDE_PARAMETERS: [Parameter; 2] = [
    Parameter {
        name: "method",
        values: &["embedding", "ljt", "sat", "table", "tableau"],
        default: None,
        description: "How to decide; left out, the logic's default",
    },
    Parameter {
        name: "translation",
        values: &["affine", "cbn", "cbv", "01"],
        default: None,
        description: "The embedding into linear logic, for the method `embedding`; left out, the logic's default",
    },
];

/// The settings keys a draw reads.
#[cfg(feature = "svg")]
static DRAW_READS: [&str; 7] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "view",
    "styles.svg",
];

/// The settings keys an export reads.
static EXPORT_READS: [&str; 9] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "view",
    "styles.text",
    "styles.latex",
    "styles.typst",
];

/// The settings keys a certificate reads.
#[cfg(feature = "rocq")]
static CERTIFICATE_READS: [&str; 7] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "view",
    "styles.rocq",
];

/// The settings keys a lambda-term reads.
static TERM_READS: [&str; 7] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
    "view",
    "lambda",
];

/// The settings keys a normal form reads: the readers' sizes and the
/// rewriting's bounds.
static NORMAL_READS: [&str; 5] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.work",
    "limits.form_occurrences",
];

/// Returns the operation of a normal form.
const fn normal_form(
    name: &'static str,
    promise: &'static super::Promise,
    label: &'static str,
) -> Operation {
    Operation {
        name,
        from: &[&["ordinary.sequent"]],
        to: "ordinary.sequent",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &NORMAL_READS,
        promise,
        label,
    }
}

/// Returns the normal form an operation's name makes.
fn form_of(operation: &str) -> Option<normal::Form> {
    Some(match operation {
        "ordinary.nnf" => normal::Form::Nnf,
        "ordinary.cnf" => normal::Form::Cnf,
        "ordinary.dnf" => normal::Form::Dnf,
        "ordinary.tseitin" => normal::Form::Tseitin,
        "ordinary.plaisted_greenbaum" => normal::Form::PlaistedGreenbaum,
        _ => return None,
    })
}

/// Reads a statement of classical logic, which a normal form takes.
fn classical(value: &str, limits: &Limits) -> Result<Sequent, Error> {
    let (sequent, logic) = statement(value, limits)?;
    if logic != Logic::Classical {
        return Err(Error::InvalidOption {
            key: "logic",
            message: "a normal form is made of a sequent of classical logic".to_owned(),
        });
    }
    Ok(sequent)
}

/// The settings keys a session's derivation reads.
static FINISH_READS: [&str; 5] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.derivation_bytes",
    "limits.work",
];

/// Its operations.
static OPERATIONS: &[Operation] = &[
    Operation {
        name: "ordinary.decide",
        from: &[&["ordinary.sequent"]],
        to: "ordinary.outcome",
        class: Class::Derived,
        cost: Cost::Search,
        parameters: &DECIDE_PARAMETERS,
        reads: &super::SEARCH,
        promise: &DECIDES,
        label: "A decision",
    },
    Operation {
        name: "ordinary.start",
        from: &[&["ordinary.sequent"]],
        to: "ordinary.session",
        class: Class::Work,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::START,
        promise: &EXACT,
        label: "A proof step by step",
    },
    Operation {
        name: "ordinary.goal",
        from: &[&["ordinary.session"]],
        to: "ordinary.sequent",
        class: Class::Copy,
        cost: Cost::Instant,
        parameters: &[Parameter {
            name: "goal",
            values: &[],
            default: None,
            description: "The goal, by its number in the session",
        }],
        reads: &super::SIZES,
        promise: &EXACT,
        label: "A goal opened on its own",
    },
    Operation {
        name: "ordinary.derivation",
        from: &[&["ordinary.session", "ordinary.outcome"]],
        to: "ordinary.derivation",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &FINISH_READS,
        promise: &PROVES,
        label: "The derivation",
    },
    Operation {
        name: "ordinary.translate",
        from: &[&["ordinary.sequent"]],
        to: "ordinary.sequent",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[
            Parameter {
                name: "translation",
                values: &["kolmogorov", "godel-gentzen", "kuroda", "glivenko"],
                default: Some("kolmogorov"),
                description: "The negative translation",
            },
            Parameter {
                name: "target",
                values: &["intuitionistic", "minimal"],
                default: None,
                description: "The logic the translation is claimed in; left out, the strongest result the translation gives",
            },
        ],
        reads: &super::SIZES,
        promise: &SAME_VERDICT,
        label: "A negative translation",
    },
    Operation {
        name: "ordinary.interpolate",
        from: &[&["ordinary.sequent"]],
        to: "ordinary.interpolant",
        class: Class::Derived,
        cost: Cost::Search,
        parameters: &[
            Parameter {
                name: "partition",
                values: &[],
                default: None,
                description: "The first part, `{\"left\": [positions], \"right\": [positions]}`; left out, every formula left of ⊢",
            },
            Parameter {
                name: "method",
                values: &["embedding", "ljt", "sat"],
                default: None,
                description: "How to decide the sequent; left out, the SAT solver, then the embedding for a derivation",
            },
        ],
        reads: &super::SEARCH,
        promise: &INTERPOLATES,
        label: "A Craig interpolant",
    },
    Operation {
        name: "ordinary.half",
        from: &[&["ordinary.interpolant"]],
        to: "ordinary.sequent",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[Parameter {
            name: "which",
            values: &["first", "second"],
            default: Some("first"),
            description: "The half: the first part with the interpolant right of ⊢, or the interpolant with the second part",
        }],
        reads: &super::SIZES,
        promise: &EXACT,
        label: "A part of an interpolant",
    },
    Operation {
        name: "ordinary.embed",
        from: &[&["ordinary.sequent"]],
        to: "linear.sequent",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[Parameter {
            name: "translation",
            values: &["affine", "cbn", "cbv", "01"],
            default: None,
            description: "The embedding; left out, the logic's default",
        }],
        reads: &super::SIZES,
        promise: &SAME_VERDICT,
        label: "The embedding into linear logic",
    },
    normal_form("ordinary.nnf", &EQUIVALENT, "Negation normal form"),
    normal_form("ordinary.cnf", &EQUIVALENT, "Conjunctive normal form"),
    normal_form("ordinary.dnf", &EQUIVALENT, "Disjunctive normal form"),
    normal_form("ordinary.tseitin", &EQUISATISFIABLE, "Tseitin's clauses"),
    normal_form(
        "ordinary.plaisted_greenbaum",
        &EQUISATISFIABLE,
        "Plaisted and Greenbaum's clauses",
    ),
    Operation {
        name: "ordinary.term",
        from: &[&["ordinary.session", "ordinary.derivation"]],
        to: "term",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &TERM_READS,
        promise: &RENDERS,
        label: "The lambda-term",
    },
    #[cfg(feature = "svg")]
    Operation {
        name: "ordinary.draw",
        from: &[&[
            "ordinary.session",
            "ordinary.derivation",
            "ordinary.outcome",
        ]],
        to: "drawing",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &DRAW_READS,
        promise: &RENDERS,
        label: "A drawing",
    },
    Operation {
        name: "ordinary.export",
        from: &[&["ordinary.session", "ordinary.derivation"]],
        to: "text",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[Parameter {
            name: "format",
            values: &["text", "latex", "typst"],
            default: Some("text"),
            description: "The format",
        }],
        reads: &EXPORT_READS,
        promise: &RENDERS,
        label: "An export",
    },
    #[cfg(feature = "rocq")]
    Operation {
        name: "ordinary.certificate",
        from: &[&["ordinary.derivation", "ordinary.outcome"]],
        to: "certificate",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &CERTIFICATE_READS,
        promise: &super::promises::CERTIFIES,
        label: "A Rocq certificate",
    },
    Operation {
        name: "ordinary.duplicate",
        from: &[&["ordinary.sequent"]],
        to: "ordinary.sequent",
        class: Class::Copy,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::SIZES,
        promise: &EXACT,
        label: "A copy",
    },
];

/// The keys a statement adds to its sequent's document.
#[derive(Deserialize)]
struct Claim {
    /// The logic.
    logic: Logic,
}

/// A statement as written: the sequent's document with its logic and how
/// it was typed.
#[derive(Serialize)]
struct Statement<'a> {
    /// The sequent's document.
    #[serde(flatten)]
    sequent: &'a Sequent,
    /// The logic it is claimed in.
    logic: Logic,
    /// The sequent as typed.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
}

/// Reads a statement: its sequent and its logic.
fn statement(value: &str, limits: &Limits) -> Result<(Sequent, Logic), Error> {
    let claim: Claim = object(value, "ordinary statement")?;
    Ok((document(value, limits)?, claim.logic))
}

/// Writes a statement.
pub(crate) fn write_statement(sequent: &Sequent, logic: Logic) -> Result<String, Error> {
    typed_statement(sequent, logic, None)
}

/// Writes a statement with the text it was typed as.
pub(super) fn typed_statement(
    sequent: &Sequent,
    logic: Logic,
    text: Option<&str>,
) -> Result<String, Error> {
    written(&Statement {
        sequent,
        logic,
        text,
    })
}

/// An interpolant's value.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Interpolated {
    /// The logic.
    logic: Logic,
    /// The partition: the first part.
    partition: Partition,
    /// The sequent's outcome, as `ordinary.decide` writes one.
    outcome: serde_json::Value,
    /// The interpolant, where the sequent is valid and one was found.
    interpolant: Option<Found>,
}

/// An interpolant found.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Found {
    /// The formula, as typed.
    formula: String,
    /// The first half, `Γ₁ ⊢ Δ₁, I`, as a statement.
    first: serde_json::Value,
    /// The second half, `I, Γ₂ ⊢ Δ₂`, as a statement.
    second: serde_json::Value,
}

/// Which input a value is, where an operation takes several kinds.
#[derive(Deserialize)]
struct Probe {
    /// An outcome's verdict.
    verdict: Option<serde::de::IgnoredAny>,
    /// A session's history.
    history: Option<serde::de::IgnoredAny>,
}

/// What an input of several kinds is.
enum Input {
    /// A session.
    Session(Box<Interactive>),
    /// An outcome.
    Outcome(Box<Outcome>),
    /// A derivation.
    Derivation(Box<Derivation>),
}

/// Reads the first input of `call`, which may be a session, an outcome or
/// a derivation, refusing a kind the operation does not take.
fn input(call: &Call<'_>, limits: &Limits) -> Result<Input, Error> {
    let value = call.input(0);
    let probe: Probe = object(value, "ordinary value")?;
    Ok(if probe.verdict.is_some() {
        takes(call, "ordinary.outcome")?;
        Input::Outcome(Box::new(document(value, limits)?))
    } else if probe.history.is_some() {
        takes(call, "ordinary.session")?;
        Input::Session(Box::new(document(value, limits)?))
    } else {
        takes(call, "ordinary.derivation")?;
        Input::Derivation(Box::new(document(value, limits)?))
    })
}

/// Returns the derivation an input holds: a session's, finished or not
/// as `finished` asks, or a valid outcome's.
fn derivation_of(
    input: Input,
    finished: bool,
    call: &Call<'_>,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Derivation, Error> {
    let settings = call.settings;
    match input {
        Input::Session(session) if finished => session.finished(&settings.limits, stop),
        Input::Session(session) => {
            session.derivation_within(&settings.view, &settings.limits, stop)
        }
        Input::Outcome(outcome) => match &outcome.verdict {
            ordinary::Verdict::Valid(evidence) => evidence.derivation().cloned(),
            _ => Err(Error::NoDerivation {
                evidence: "no proof: the sequent is not shown valid",
            }),
        },
        Input::Derivation(derivation) => Ok(*derivation),
    }
}

/// Returns the bound that ended a decision, where one did.
fn limit_of(outcome: &Outcome) -> Option<Limit> {
    match &outcome.verdict {
        ordinary::Verdict::Unknown(reason) => Some(Limit {
            reason: reason.name(),
            setting: reason.setting(),
        }),
        _ => None,
    }
}

/// The parameters of a decision.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Decide {
    /// The method.
    method: Option<Method>,
    /// The embedding.
    translation: Option<Translation>,
}

/// The parameters of opening a goal.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Goal {
    /// The goal's number.
    goal: u32,
}

/// The parameters of an interpolant.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Interpolate {
    /// The first part.
    partition: Option<Partition>,
    /// The method.
    method: Option<Method>,
}

/// The parameters of taking a half.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Half {
    /// `second` for the second half.
    which: Option<String>,
}

/// The parameters of an embedding.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Embed {
    /// The embedding.
    translation: Option<Translation>,
}

/// The parameters of an export.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Format {
    /// The format.
    pub(super) format: Option<String>,
}

/// The formats an export writes.
pub(super) const FORMATS: &[&str] = &[
    "text",
    #[cfg(feature = "latex")]
    "latex",
    #[cfg(feature = "typst")]
    "typst",
];

/// Returns a check's part for a check's error: open where a bound or the
/// stop ended it, failed where it found a fault; any other error is
/// returned.
pub(super) fn checked(name: &'static str, result: Result<(), Error>) -> Result<Part, Error> {
    let status = match result {
        Ok(()) => Status::Held,
        Err(error) => match error.kind() {
            ErrorKind::Invalid | ErrorKind::Defect => Status::Failed,
            ErrorKind::Limit | ErrorKind::Stopped => Status::Open,
            _ => return Err(error),
        },
    };
    Ok(Part {
        name: name.to_owned(),
        status,
        verdict: None,
    })
}

/// The keys every ordinary outcome writes, and the clausal proof's.
#[derive(Deserialize)]
struct OutcomeKeys {
    /// The verdict's name.
    verdict: String,
    /// The logic decided, which a linear outcome lacks.
    #[expect(dead_code, reason = "read to refuse a value that has no logic")]
    logic: Logic,
    /// The proof's counts, for a valid sequent by the SAT solver.
    clausal_proof: Option<serde::de::IgnoredAny>,
}

/// Returns the verdict of an ordinary outcome, read by its reader; an
/// outcome valid by a clausal proof, which has no reader (its counts hold
/// no proof), by its keys alone.
fn outcome_verdict(value: &str, limits: &Limits) -> Result<Verdict, Error> {
    let keys: OutcomeKeys = object(value, "ordinary outcome")?;
    if keys.clausal_proof.is_some() && keys.verdict == "valid" {
        return Ok(Verdict::Valid);
    }
    document::<Outcome>(value, limits)?;
    verdict_of(value)
}

/// Returns a formula's text, as an interpolant's is written.
fn text_of(sequent: &Sequent, id: ordinary::NodeId) -> String {
    sequent.formulas().formula(id).to_string()
}

/// Reads an interpolant's value: where it holds one, its halves are
/// statements of its logic, the first with the formula last right of ⊢
/// and the second with it first left of ⊢.
fn interpolated(value: &str, limits: &Limits) -> Result<Interpolated, Error> {
    let read: Interpolated = object(value, "interpolant")?;
    if let Some(found) = &read.interpolant {
        let fault = |message: &str| Error::Json {
            form: "interpolant",
            message: message.to_owned(),
        };
        let (first, first_logic) = statement(&written(&found.first)?, limits)?;
        let (second, second_logic) = statement(&written(&found.second)?, limits)?;
        if first_logic != read.logic || second_logic != read.logic {
            return Err(fault("its halves are claimed in another logic"));
        }
        let (Some(&last), Some(&head)) = (first.right().last(), second.left().first()) else {
            return Err(fault("a half lacks the interpolant"));
        };
        if text_of(&first, last) != found.formula || text_of(&second, head) != found.formula {
            return Err(fault(
                "its halves hold another formula than the interpolant",
            ));
        }
    }
    Ok(read)
}

/// Returns the texts of the formulas at `ids` in `sequent`, sorted: a part
/// compared as a multiset.
fn texts(sequent: &Sequent, ids: impl IntoIterator<Item = ordinary::NodeId>) -> Vec<String> {
    let mut texts: Vec<String> = ids.into_iter().map(|id| text_of(sequent, id)).collect();
    texts.sort();
    texts
}

/// Returns whether an interpolant's halves are the input's parts by its
/// partition: `Γ₁ ⊢ Δ₁, I` and `I, Γ₂ ⊢ Δ₂`, each part compared as a
/// multiset of formulas.
fn halves_fit(input: &Sequent, partition: &Partition, one: &Sequent, two: &Sequent) -> bool {
    let first_side = |ids: &[ordinary::NodeId], chosen: &[usize], wanted: bool| {
        let picked = ids
            .iter()
            .enumerate()
            .filter(|(position, _)| chosen.contains(position) == wanted)
            .map(|(_, &id)| id);
        texts(input, picked)
    };
    let right = one.right();
    first_side(input.left(), &partition.left, true) == texts(one, one.left().iter().copied())
        && first_side(input.right(), &partition.right, true)
            == texts(one, right[..right.len() - 1].iter().copied())
        && first_side(input.left(), &partition.left, false)
            == texts(two, two.left()[1..].iter().copied())
        && first_side(input.right(), &partition.right, false)
            == texts(two, two.right().iter().copied())
}

/// Returns the names of the atoms of `roots` in `sequent`.
fn atom_names(sequent: &Sequent, roots: &[ordinary::NodeId]) -> Vec<String> {
    let formulas = sequent.formulas();
    let reached = interpolant::atoms_of(formulas, roots);
    formulas
        .atom_names()
        .iter()
        .zip(reached)
        .filter(|&(_, reached)| reached)
        .map(|(name, _)| name.clone())
        .collect()
}

impl Suite for Ordinary {
    fn name(&self) -> &'static str {
        "ordinary"
    }

    fn label(&self) -> &'static str {
        "Ordinary"
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
            "ordinary.sequent" => statement(value, limits)?.0.to_string(),
            "ordinary.session" => {
                let session: Interactive = document(value, limits)?;
                match session.goals().count() {
                    0 => "Proved".to_owned(),
                    1 => "1 open goal".to_owned(),
                    goals => format!("{goals} open goals"),
                }
            }
            // An outcome valid by a clausal proof has no reader: its words
            // are read alone.
            "ordinary.outcome" => match outcome_verdict(value, limits)? {
                Verdict::Valid => "Valid".to_owned(),
                Verdict::NotValid => "Not valid".to_owned(),
                Verdict::Unknown => "Not decided".to_owned(),
            },
            "ordinary.derivation" => {
                let derivation: Derivation = document(value, limits)?;
                format!("A derivation of {}", derivation.logic().calculus())
            }
            _ => {
                let read = interpolated(value, limits)?;
                match read.interpolant {
                    Some(found) => format!("Interpolant {}", found.formula),
                    None => "No interpolant".to_owned(),
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
            "ordinary.decide" => {
                let (sequent, logic) = statement(call.input(0), limits)?;
                let p: Decide = parameters(call)?;
                let options = ordinary::Options::default()
                    .with_logic(logic)
                    .with_translation(p.translation)
                    .with_method(p.method);
                let outcome =
                    ordinary::decide_within(&sequent, &options, &settings.search, limits, stop)?;
                return Ok(Produced {
                    value: written(&outcome)?,
                    limit: limit_of(&outcome),
                });
            }
            "ordinary.start" => {
                let (sequent, logic) = statement(call.input(0), limits)?;
                written(&Interactive::within(&sequent, logic, limits)?)?
            }
            "ordinary.goal" => {
                let session: Interactive = document(call.input(0), limits)?;
                let p: Goal = parameters(call)?;
                let (left, right) = session.goal(crate::linear::GoalId::new(p.goal))?;
                let sequent =
                    Sequent::new(session.formulas().clone(), left.to_vec(), right.to_vec())?;
                write_statement(&sequent, session.logic())?
            }
            "ordinary.derivation" => {
                let input = input(call, limits)?;
                written(&derivation_of(input, true, call, stop)?)?
            }
            "ordinary.translate" => {
                let (sequent, logic) = statement(call.input(0), limits)?;
                if logic != Logic::Classical {
                    return Err(Error::InvalidOption {
                        key: "logic",
                        message: "a negative translation translates a sequent of classical logic"
                            .to_owned(),
                    });
                }
                let options: negative::Options = parameters(call)?;
                let translated = negative::translate(&sequent, &options, limits)?;
                write_statement(&translated, options.target())?
            }
            "ordinary.interpolate" => {
                let (sequent, logic) = statement(call.input(0), limits)?;
                let p: Interpolate = parameters(call)?;
                let partition = p.partition.unwrap_or_default();
                let options = ordinary::Options::default()
                    .with_logic(logic)
                    .with_method(p.method);
                let interpolation = interpolant::interpolate(
                    &sequent,
                    &options,
                    &partition,
                    &settings.search,
                    limits,
                    stop,
                )?;
                let found = match &interpolation.interpolant {
                    Some(found) => Some(Found {
                        formula: found.formula().to_string(),
                        first: serde_json::from_str(&write_statement(&found.first, logic)?)
                            .map_err(|e| json_error(&e))?,
                        second: serde_json::from_str(&write_statement(&found.second, logic)?)
                            .map_err(|e| json_error(&e))?,
                    }),
                    None => None,
                };
                let outcome = serde_json::from_str(&written(&interpolation.outcome)?)
                    .map_err(|e| json_error(&e))?;
                return Ok(Produced {
                    value: written(&Interpolated {
                        logic,
                        partition: interpolation.partition.clone(),
                        outcome,
                        interpolant: found,
                    })?,
                    limit: limit_of(&interpolation.outcome),
                });
            }
            "ordinary.half" => {
                let read = interpolated(call.input(0), limits)?;
                let p: Half = parameters(call)?;
                let found = read.interpolant.ok_or_else(|| Error::InvalidOption {
                    key: "which",
                    message: "the sequent has no interpolant to take a half of".to_owned(),
                })?;
                let half = match p.which.as_deref() {
                    None | Some("first") => found.first,
                    Some("second") => found.second,
                    Some(other) => {
                        return Err(Error::UnknownName {
                            what: "half",
                            name: other.into(),
                            known: &["first", "second"],
                        });
                    }
                };
                let half = written(&half)?;
                statement(&half, limits)?;
                half
            }
            name if form_of(name).is_some() => {
                let sequent = classical(call.input(0), limits)?;
                let options =
                    normal::Options::default().with_form(form_of(name).unwrap_or_default());
                let made = normal::rewrite(&sequent, &options, limits, &mut *stop)?;
                write_statement(&made.result, Logic::Classical)?
            }
            "ordinary.embed" => {
                let (sequent, logic) = statement(call.input(0), limits)?;
                let p: Embed = parameters(call)?;
                let translation = p.translation.unwrap_or(Translation::default_for(logic));
                let image = ordinary::translate(&sequent, logic, translation, limits)?;
                super::linear::write_statement(image.sequent(), image.mode())?
            }
            "ordinary.term" => {
                let input = input(call, limits)?;
                let derivation = derivation_of(input, false, call, stop)?;
                let term = derivation.term(limits, &mut *stop)?;
                term_value(&term, settings, stop)?
            }
            #[cfg(feature = "svg")]
            "ordinary.draw" => {
                let input = input(call, limits)?;
                let svg = match input {
                    Input::Outcome(outcome) if outcome.countermodel.is_some() => {
                        let countermodel = outcome
                            .countermodel
                            .as_ref()
                            .unwrap_or_else(|| unreachable!("the guard found a countermodel"));
                        crate::export::svg::countermodel(
                            countermodel,
                            outcome.logic,
                            &settings.styles.svg,
                            limits,
                            stop,
                        )?
                    }
                    input => {
                        let derivation = derivation_of(input, false, call, stop)?;
                        let mut out = String::new();
                        crate::export::svg::write(
                            &derivation,
                            &settings.styles.svg,
                            &mut out,
                            stop,
                        )?;
                        out
                    }
                };
                written(&super::shared::Drawing { svg })?
            }
            "ordinary.export" => {
                let input = input(call, limits)?;
                let derivation = derivation_of(input, false, call, stop)?;
                let p: Format = parameters(call)?;
                let format = p.format.unwrap_or_else(|| "text".to_owned());
                let styles = &settings.styles;
                let mut out = String::new();
                match format.as_str() {
                    "text" => derivation.write_text(&styles.text, &mut out, stop)?,
                    #[cfg(feature = "latex")]
                    "latex" => {
                        crate::export::latex::write(&derivation, &styles.latex, &mut out, stop)?
                    }
                    #[cfg(feature = "typst")]
                    "typst" => {
                        crate::export::typst::write(&derivation, &styles.typst, &mut out, stop)?
                    }
                    other => {
                        return Err(Error::UnknownName {
                            what: "format",
                            name: other.into(),
                            known: FORMATS,
                        });
                    }
                }
                written(&Text { format, text: out })?
            }
            #[cfg(feature = "rocq")]
            "ordinary.certificate" => {
                let input = input(call, limits)?;
                let mut out = String::new();
                match input {
                    Input::Outcome(outcome) if outcome.countermodel.is_some() => {
                        let countermodel = outcome
                            .countermodel
                            .as_ref()
                            .unwrap_or_else(|| unreachable!("the guard found a countermodel"));
                        crate::export::rocq::write_countermodel(
                            countermodel,
                            outcome.logic,
                            &settings.styles.rocq,
                            limits,
                            &mut out,
                            stop,
                        )?;
                    }
                    input => {
                        let derivation = derivation_of(input, true, call, stop)?;
                        crate::export::rocq::write(
                            &derivation,
                            &settings.styles.rocq,
                            &mut out,
                            stop,
                        )?;
                    }
                }
                written(&super::shared::Certificate { text: out })?
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
        ask: &mut super::Ask<'_>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Check, Error> {
        let promise = call.operation.promise;
        let limits = &call.settings.limits;
        match promise.name {
            "decides" => decides(output, limits, stop),
            "proves" => {
                let derivation: Derivation = document(output, limits)?;
                let part = checked("derivation", derivation.check(limits, stop))?;
                Ok(Check::whole(promise, part.status))
            }
            "same_verdict" if call.operation.name == "ordinary.embed" => {
                let sequent = verdict_of(&ask("ordinary.decide", &[call.input(0)], "")?)?;
                let image = verdict_of(&ask("linear.prove", &[output], "")?)?;
                Ok(same_verdict(
                    promise,
                    ["sequent", "image"],
                    [sequent, image],
                ))
            }
            "same_verdict" => {
                let sequent = verdict_of(&ask("ordinary.decide", &[call.input(0)], "")?)?;
                let translation = verdict_of(&ask("ordinary.decide", &[output], "")?)?;
                Ok(same_verdict(
                    promise,
                    ["sequent", "translation"],
                    [sequent, translation],
                ))
            }
            "interpolates" => {
                let read = interpolated(output, limits)?;
                let Some(found) = read.interpolant else {
                    return Ok(Check::whole(promise, Status::None));
                };
                let (first, second) = (written(&found.first)?, written(&found.second)?);
                let (one, _) = statement(&first, limits)?;
                let (two, _) = statement(&second, limits)?;
                // `interpolated` read both halves with the formula in place.
                let (input, _) = statement(call.input(0), limits)?;
                let halves = Part {
                    name: "halves".to_owned(),
                    status: if halves_fit(&input, &read.partition, &one, &two) {
                        Status::Held
                    } else {
                        Status::Failed
                    },
                    verdict: None,
                };
                let interpolant = atom_names(&one, &one.right()[one.right().len() - 1..]);
                let mut rest_one = one.left().to_vec();
                rest_one.extend_from_slice(&one.right()[..one.right().len() - 1]);
                let mut rest_two = two.left()[1..].to_vec();
                rest_two.extend_from_slice(two.right());
                let (rest_one, rest_two) =
                    (atom_names(&one, &rest_one), atom_names(&two, &rest_two));
                let shared = interpolant
                    .iter()
                    .all(|atom| rest_one.contains(atom) && rest_two.contains(atom));
                let atoms = Part {
                    name: "atoms".to_owned(),
                    status: if shared { Status::Held } else { Status::Failed },
                    verdict: None,
                };
                let first = verdict_of(&ask("ordinary.decide", &[&first], "")?)?;
                let second = verdict_of(&ask("ordinary.decide", &[&second], "")?)?;
                Ok(Check::of(
                    promise,
                    vec![
                        halves,
                        atoms,
                        part("first", first, Verdict::Valid),
                        part("second", second, Verdict::Valid),
                    ],
                ))
            }
            "equivalent" | "equisatisfiable" => normal_check(call, output, limits, ask, stop),
            "certifies" => Ok(Check::whole(promise, Status::Outside)),
            _ => Ok(Check::whole(promise, Status::None)),
        }
    }
}

/// The parameters of the decision of a normal form's claim: the SAT
/// solver, whose clausal proof is checked, where the build has it.
#[cfg(feature = "sat")]
const CLAIMS: &str = r#"{"method": "sat"}"#;
/// The parameters of the decision of a normal form's claim: the SAT
/// solver, whose clausal proof is checked, where the build has it.
#[cfg(not(feature = "sat"))]
const CLAIMS: &str = "";

/// Checks a normal form's promise: makes the form again from the input
/// (for an encoding, whose definitions the output does not hold, and
/// compares the output with it), then decides each claim of the promise
/// in classical logic.
fn normal_check(
    call: &Call<'_>,
    output: &str,
    limits: &Limits,
    ask: &mut super::Ask<'_>,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Check, Error> {
    let promise = call.operation.promise;
    let form = form_of(call.operation.name).unwrap_or_default();
    let input = classical(call.input(0), limits)?;
    let (given, logic) = statement(output, limits)?;
    // The output is claimed in classical logic, where its promise holds.
    let classical = Part {
        name: "logic".to_owned(),
        status: if logic == Logic::Classical {
            Status::Held
        } else {
            Status::Failed
        },
        verdict: None,
    };
    if classical.status == Status::Failed {
        return Ok(Check::of(promise, vec![classical]));
    }
    let options = normal::Options::default().with_form(form);
    let mut made = match normal::rewrite(&input, &options, limits, &mut *stop) {
        Ok(made) => made,
        Err(error) => return Ok(Check::of(promise, vec![checked("form", Err(error))?])),
    };
    let mut parts = Vec::new();
    match form.promise() {
        // The output itself is held to the input.
        normal::Promise::Equivalent => made.result = given,
        // The definitions are the form's made again, so the output must
        // be that form.
        _ => {
            let same = made.result.to_string() == given.to_string();
            parts.push(Part {
                name: "form".to_owned(),
                status: if same { Status::Held } else { Status::Failed },
                verdict: None,
            });
            if !same {
                return Ok(Check::of(promise, parts));
            }
        }
    }
    let claims = match made.claims() {
        Ok(claims) => claims,
        Err(error) => {
            parts.push(checked("definitions", Err(error))?);
            return Ok(Check::of(promise, parts));
        }
    };
    for (claim, sequent) in claims {
        let statement = write_statement(&sequent, Logic::Classical)?;
        let verdict = verdict_of(&ask("ordinary.decide", &[&statement], CLAIMS)?)?;
        parts.push(part(claim.name(), verdict, Verdict::Valid));
    }
    Ok(Check::of(promise, parts))
}

/// Checks a decision's evidence.
fn decides(
    output: &str,
    limits: &Limits,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<Check, Error> {
    let promise = &DECIDES;
    let keys: OutcomeKeys = object(output, "ordinary outcome")?;
    if keys.clausal_proof.is_some() && keys.verdict == "valid" {
        // The SAT solver's proof was checked when it was made, and the
        // outcome keeps its counts alone.
        return Ok(Check::whole(promise, Status::Recorded));
    }
    let outcome: Outcome = document(output, limits)?;
    let status = match &outcome.verdict {
        ordinary::Verdict::Valid(ordinary::Evidence::Derivation(derivation)) => {
            checked("derivation", derivation.check(limits, stop))?.status
        }
        ordinary::Verdict::Valid(_) => Status::Recorded,
        ordinary::Verdict::NotValid => match &outcome.countermodel {
            Some(countermodel) => {
                checked(
                    "countermodel",
                    countermodel.check(outcome.logic, limits, stop),
                )?
                .status
            }
            None => Status::Recorded,
        },
        ordinary::Verdict::Unknown(_) => Status::None,
    };
    Ok(Check::whole(promise, status))
}

/// Returns a lambda-term's value, written with the settings' `lambda`.
pub(super) fn term_value(
    term: &crate::lambda::Term,
    settings: &crate::Settings,
    stop: &mut dyn FnMut(Progress) -> bool,
) -> Result<String, Error> {
    let (options, limits) = (&settings.lambda, &settings.limits);
    let (mut text, mut judgement) = (String::new(), String::new());
    term.write(&mut text, options, limits, &mut *stop)?;
    term.write_judgement(&mut judgement, options, limits, &mut *stop)?;
    written(&Term { text, judgement })
}

/// Returns the error of a value this module wrote and could not read back.
fn json_error(error: &serde_json::Error) -> Error {
    Error::Json {
        form: "value",
        message: error.to_string(),
    }
}
