// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The linear suite's kinds and operations.

use serde::{Deserialize, Serialize};

use super::ordinary::{FORMATS, Format, checked, term_value};
use super::promises::{DECIDES, EXACT, PROVES, RENDERS};
use super::shared::Text;
use super::{
    Call, Check, Class, Cost, Kind, Limit, Operation, Parameter, Produced, Role, Status, Suite,
    Verdict, document, kinds, object, parameters, verdict_of, written,
};
use crate::linear::nets::{Criterion, MellStructure, ProofStructure};
use crate::linear::occurrences::reading::Side;
use crate::linear::search::{Outcome, Verdict as Found};
use crate::linear::{GoalId, IllSequent, Interactive, Mode, Proof, Sequent};
use crate::{Error, Limits, Progress};

/// The linear suite.
pub(super) static LINEAR: Linear = Linear;

/// The linear suite.
pub(super) struct Linear;

/// Its kinds.
static KINDS: [Kind; 5] = [
    Kind {
        name: "linear.sequent",
        role: Role::Statement,
        form: "linear_statement",
        level: Some(1),
        stored: true,
        label: "A sequent of linear logic, claimed in a mode",
    },
    Kind {
        name: "linear.session",
        role: Role::Work,
        form: "session",
        level: Some(1),
        stored: true,
        label: "A proof of linear logic in progress",
    },
    Kind {
        name: "linear.outcome",
        role: Role::Evidence,
        form: "outcome",
        level: Some(1),
        stored: true,
        label: "A sequent decided: proved with a proof, or unprovable with a refutation",
    },
    Kind {
        name: "linear.proof",
        role: Role::Evidence,
        form: "proof",
        level: Some(1),
        stored: true,
        label: "A proof of linear logic",
    },
    Kind {
        name: "linear.net",
        role: Role::Evidence,
        form: "net",
        level: Some(1),
        stored: true,
        label: "A proof net",
    },
];

/// The settings keys a proof's check reads.
static PROOF_READS: [&str; 4] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.work",
];

/// The settings keys a cut elimination reads.
static ELIMINATE_READS: [&str; 5] = [
    "limits.memory_bytes",
    "limits.occurrences",
    "limits.terms",
    "limits.work",
    "cut",
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

/// Its operations.
static OPERATIONS: &[Operation] = &[
    Operation {
        name: "linear.prove",
        from: &[&["linear.sequent"]],
        to: "linear.outcome",
        class: Class::Derived,
        cost: Cost::Search,
        parameters: &[],
        reads: &super::SEARCH,
        promise: &DECIDES,
        label: "A search",
    },
    Operation {
        name: "linear.start",
        from: &[&["linear.sequent"]],
        to: "linear.session",
        class: Class::Work,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::START,
        promise: &EXACT,
        label: "A proof step by step",
    },
    Operation {
        name: "linear.goal",
        from: &[&["linear.session"]],
        to: "linear.sequent",
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
        name: "linear.proof",
        from: &[&["linear.session"]],
        to: "linear.proof",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &PROOF_READS,
        promise: &PROVES,
        label: "The proof",
    },
    Operation {
        name: "linear.net",
        from: &[&["linear.proof"]],
        to: "linear.net",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &PROOF_READS,
        promise: &PROVES,
        label: "The proof net",
    },
    Operation {
        name: "linear.eliminate",
        from: &[&["linear.proof"]],
        to: "linear.proof",
        class: Class::Derived,
        cost: Cost::Search,
        parameters: &[],
        reads: &ELIMINATE_READS,
        promise: &PROVES,
        label: "The cuts eliminated",
    },
    Operation {
        name: "linear.term",
        from: &[&["linear.session", "linear.proof"]],
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
        name: "linear.draw",
        from: &[&["linear.session", "linear.proof", "linear.net"]],
        to: "drawing",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &DRAW_READS,
        promise: &RENDERS,
        label: "A drawing",
    },
    Operation {
        name: "linear.export",
        from: &[&["linear.session", "linear.proof"]],
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
        name: "linear.certificate",
        from: &[&["linear.proof", "linear.outcome"]],
        to: "certificate",
        class: Class::Derived,
        cost: Cost::Instant,
        parameters: &[],
        reads: &CERTIFICATE_READS,
        promise: &super::promises::CERTIFIES,
        label: "A Rocq certificate",
    },
    Operation {
        name: "linear.duplicate",
        from: &[&["linear.sequent"]],
        to: "linear.sequent",
        class: Class::Copy,
        cost: Cost::Instant,
        parameters: &[],
        reads: &super::SIZES,
        promise: &EXACT,
        label: "A copy",
    },
];

/// The keys a statement adds to its sequent's document, and the key that
/// tells an intuitionistic sequent's document.
#[derive(Deserialize)]
struct Claim {
    /// The mode's name.
    mode: String,
    /// An intuitionistic sequent's hypotheses.
    hypotheses: Option<serde::de::IgnoredAny>,
}

/// A statement as written: the one-sided sequent's document with its mode.
#[derive(Serialize)]
struct Statement<'a> {
    /// The sequent's document.
    #[serde(flatten)]
    sequent: &'a Sequent,
    /// The mode it is claimed in.
    mode: Mode,
    /// The sequent as typed.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
}

/// Reads a statement: its one-sided sequent and its mode. An
/// intuitionistic sequent's document reads as its lowering.
fn statement(value: &str, limits: &Limits) -> Result<(Sequent, Mode), Error> {
    let claim: Claim = object(value, "linear statement")?;
    let mode: Mode = claim.mode.parse()?;
    let sequent = if claim.hypotheses.is_some() {
        document::<IllSequent>(value, limits)?.into_sequent()
    } else {
        document(value, limits)?
    };
    Ok((sequent, mode))
}

/// Writes a statement.
pub(super) fn write_statement(sequent: &Sequent, mode: Mode) -> Result<String, Error> {
    typed_statement(sequent, mode, None)
}

/// Writes a statement with the text it was typed as.
pub(super) fn typed_statement(
    sequent: &Sequent,
    mode: Mode,
    text: Option<&str>,
) -> Result<String, Error> {
    written(&Statement {
        sequent,
        mode,
        text,
    })
}

/// The key that tells a session's document from a proof's.
#[derive(Deserialize)]
struct Probe {
    /// A session's history.
    history: Option<serde::de::IgnoredAny>,
    /// A proof structure's links.
    links: Option<serde::de::IgnoredAny>,
    /// An outcome's verdict.
    verdict: Option<serde::de::IgnoredAny>,
}

/// What an input of several kinds is.
enum Input {
    /// A session.
    Session(Box<Interactive>),
    /// A proof.
    Proof(Box<Proof>),
    /// A proof net, plain or with boxes.
    #[cfg_attr(
        not(feature = "svg"),
        expect(dead_code, reason = "only a drawing reads a net given as an input")
    )]
    Net(Box<Net>),
    /// An outcome.
    #[cfg_attr(
        not(feature = "rocq"),
        expect(
            dead_code,
            reason = "only a certificate reads an outcome given as an input"
        )
    )]
    Outcome(Box<Outcome>),
}

/// A proof net.
enum Net {
    /// Without boxes.
    Plain(ProofStructure),
    /// With boxes.
    Boxes(MellStructure),
}

/// The key of a structure with boxes.
#[derive(Deserialize)]
struct Boxes {
    /// Its vertices.
    vertices: Option<serde::de::IgnoredAny>,
}

/// Reads a proof net, plain or with boxes.
fn net_of(value: &str, limits: &Limits) -> Result<Net, Error> {
    Ok(
        if object::<Boxes>(value, "proof structure")?
            .vertices
            .is_some()
        {
            Net::Boxes(document(value, limits)?)
        } else {
            Net::Plain(document(value, limits)?)
        },
    )
}

/// Reads an input that may be a session, a proof, a net or an outcome.
fn input(value: &str, limits: &Limits) -> Result<Input, Error> {
    let probe: Probe = object(value, "linear value")?;
    Ok(if probe.history.is_some() {
        Input::Session(Box::new(document(value, limits)?))
    } else if probe.links.is_some() {
        Input::Net(Box::new(net_of(value, limits)?))
    } else if probe.verdict.is_some() {
        Input::Outcome(Box::new(document(value, limits)?))
    } else {
        Input::Proof(Box::new(document(value, limits)?))
    })
}

/// Reads a proof. An outcome of a proof, which the proof's reader takes
/// too, is refused: it is a value of another kind.
fn proof_of(value: &str, limits: &Limits) -> Result<Proof, Error> {
    if object::<Probe>(value, "proof")?.verdict.is_some() {
        return Err(Error::Json {
            form: "proof",
            message: "an outcome, which is no proof here".to_owned(),
        });
    }
    document(value, limits)
}

/// Returns a proof's mode: the one it records, else classical.
fn mode_of(proof: &Proof) -> Mode {
    proof.mode().unwrap_or(Mode::CLASSICAL)
}

/// The parameters of opening a goal.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Goal {
    /// The goal's number.
    goal: u32,
}

/// Returns a goal of a session as a statement in the session's mode.
fn goal(session: &Interactive, goal: GoalId) -> Result<String, Error> {
    let members = session.goal(goal)?;
    let forest = session.forest();
    if members.iter().any(|member| member.index() >= forest.len()) {
        return Err(Error::InvalidOption {
            key: "goal",
            message: "the goal holds instances of quantified formulas, which no sequent of its own names yet"
                .to_owned(),
        });
    }
    let whole = forest.sequent();
    let mut occurrences: Vec<_> = members.iter().map(|member| member.occ()).collect();
    // In an intuitionistic mode the hypotheses come first, so that the
    // sides are written and the reading takes them as they were.
    let antecedents = match session.reading() {
        Some(reading) => {
            occurrences.sort_by_key(|&o| reading.position(o) != Side::Input);
            occurrences
                .iter()
                .filter(|&&o| reading.position(o) == Side::Input)
                .count()
        }
        None => 0,
    };
    let roots = occurrences.iter().map(|&o| forest.term(o)).collect();
    let mut sequent = Sequent::from_parts(
        whole.terms().to_vec(),
        roots,
        whole.atom_names().to_vec(),
        u32::try_from(antecedents).ok(),
    );
    sequent.optimize();
    write_statement(&sequent, session.mode())
}

impl Suite for Linear {
    fn name(&self) -> &'static str {
        "linear"
    }

    fn label(&self) -> &'static str {
        "Linear"
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
            "linear.sequent" => statement(value, limits)?.0.to_string(),
            "linear.session" => {
                let session: Interactive = document(value, limits)?;
                match session.goals().count() {
                    0 => "Proved".to_owned(),
                    1 => "1 open goal".to_owned(),
                    goals => format!("{goals} open goals"),
                }
            }
            "linear.outcome" => {
                document::<Outcome>(value, limits)?;
                match verdict_of(value)? {
                    Verdict::Valid => "Proved".to_owned(),
                    Verdict::NotValid => "Unprovable".to_owned(),
                    Verdict::Unknown => "Not decided".to_owned(),
                }
            }
            "linear.proof" => {
                proof_of(value, limits)?;
                "A proof".to_owned()
            }
            _ => {
                net_of(value, limits)?;
                "A proof net".to_owned()
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
            "linear.prove" => {
                let (sequent, mode) = statement(call.input(0), limits)?;
                let outcome =
                    crate::linear::prove_within(&sequent, mode, &settings.search, limits, stop)?;
                let limit = match &outcome.verdict {
                    Found::Unknown(reason) => Some(Limit {
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
            "linear.start" => {
                let (sequent, mode) = statement(call.input(0), limits)?;
                written(&Interactive::within(&sequent, mode, limits)?)?
            }
            "linear.goal" => {
                let session: Interactive = document(call.input(0), limits)?;
                let p: Goal = parameters(call)?;
                goal(&session, GoalId::new(p.goal))?
            }
            "linear.proof" => {
                let session: Interactive = document(call.input(0), limits)?;
                written(&session.proof(limits, stop)?)?
            }
            "linear.net" => {
                let proof = proof_of(call.input(0), limits)?;
                let mode = mode_of(&proof);
                let criterion = Criterion::of(mode)?;
                if proof.sequent().fragment().has_exponentials() {
                    written(&MellStructure::from_proof(&proof, criterion, limits, stop)?)?
                } else {
                    written(&ProofStructure::from_proof(
                        &proof, criterion, limits, stop,
                    )?)?
                }
            }
            "linear.eliminate" => {
                let proof = proof_of(call.input(0), limits)?;
                let mode = mode_of(&proof);
                let elimination = crate::linear::proofs::cut::eliminate(
                    &proof,
                    mode,
                    None,
                    &settings.cut,
                    limits,
                    stop,
                    |_, _| {},
                )?;
                written(&elimination.proof)?
            }
            "linear.term" => {
                let term = match input(call.input(0), limits)? {
                    Input::Session(session) => session
                        .derivation_within(&settings.view, limits, &mut *stop)?
                        .term(limits, &mut *stop)?,
                    Input::Proof(proof) => proof
                        .derivation_within(&settings.view, limits, &mut *stop)?
                        .term(limits, &mut *stop)?,
                    _ => return Err(kinds(call)),
                };
                term_value(&term, settings, stop)?
            }
            #[cfg(feature = "svg")]
            "linear.draw" => {
                let style = &settings.styles.svg;
                let svg = match input(call.input(0), limits)? {
                    Input::Session(session) => {
                        let derivation =
                            session.derivation_within(&settings.view, limits, &mut *stop)?;
                        let mut out = String::new();
                        crate::export::svg::write(&derivation, style, &mut out, stop)?;
                        out
                    }
                    Input::Proof(proof) => {
                        let derivation =
                            proof.derivation_within(&settings.view, limits, &mut *stop)?;
                        let mut out = String::new();
                        crate::export::svg::write(&derivation, style, &mut out, stop)?;
                        out
                    }
                    Input::Net(net) => match *net {
                        Net::Plain(net) => crate::export::svg::net(&net, style, limits, stop)?,
                        Net::Boxes(net) => crate::export::svg::mell_net(&net, style, limits, stop)?,
                    },
                    Input::Outcome(_) => return Err(kinds(call)),
                };
                written(&super::shared::Drawing { svg })?
            }
            "linear.export" => {
                let p: Format = parameters(call)?;
                let format = p.format.unwrap_or_else(|| "text".to_owned());
                let input = input(call.input(0), limits)?;
                let session;
                let proof;
                let derivation = match &input {
                    Input::Session(state) => {
                        session = state;
                        session.derivation_within(&settings.view, limits, &mut *stop)?
                    }
                    Input::Proof(read) => {
                        proof = read;
                        proof.derivation_within(&settings.view, limits, &mut *stop)?
                    }
                    _ => return Err(kinds(call)),
                };
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
            "linear.certificate" => {
                let rocq = &settings.styles.rocq;
                let view = settings
                    .view
                    .with_compact(crate::linear::proofs::derivation::Compact::Never);
                let mut out = String::new();
                match input(call.input(0), limits)? {
                    Input::Proof(proof) => {
                        let derivation = proof.derivation_within(&view, limits, &mut *stop)?;
                        crate::export::rocq::write(&derivation, rocq, &mut out, stop)?;
                    }
                    Input::Outcome(outcome) => match &outcome.verdict {
                        Found::Proved(proof) => {
                            let derivation = proof.derivation_within(&view, limits, &mut *stop)?;
                            crate::export::rocq::write(&derivation, rocq, &mut out, stop)?;
                        }
                        Found::Unprovable(disproof) => {
                            crate::export::rocq::write_disproof(disproof, rocq, &mut out)?;
                        }
                        Found::Unknown(_) => {
                            return Err(Error::InvalidOption {
                                key: "inputs",
                                message: "an outcome that is not decided has no certificate"
                                    .to_owned(),
                            });
                        }
                    },
                    _ => return Err(kinds(call)),
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
        _ask: &mut super::Ask<'_>,
        stop: &mut dyn FnMut(Progress) -> bool,
    ) -> Result<Check, Error> {
        let promise = call.operation.promise;
        let limits = &call.settings.limits;
        match promise.name {
            "decides" => {
                let outcome: Outcome = document(output, limits)?;
                let status = match &outcome.verdict {
                    Found::Proved(proof) => {
                        let checked_proof = proof
                            .check_within(outcome.mode, limits, &mut *stop)
                            .map_err(Error::Check);
                        checked("proof", checked_proof)?.status
                    }
                    Found::Unprovable(disproof) => match disproof.check() {
                        Err(error) if error.code() == "uncertified" => Status::Recorded,
                        result => checked("refutation", result)?.status,
                    },
                    Found::Unknown(_) => Status::None,
                };
                Ok(Check::whole(promise, status))
            }
            "proves" if call.operation.name == "linear.net" => {
                let result = match net_of(output, limits)? {
                    Net::Plain(net) => net.is_correct(&mut *stop),
                    Net::Boxes(net) => net.is_correct(&mut *stop),
                };
                let part = checked("net", result.map_err(|error| Error::Net(Box::new(error))))?;
                Ok(Check::whole(promise, part.status))
            }
            "proves" => {
                let proof: Proof = document(output, limits)?;
                let result = proof
                    .check_within(mode_of(&proof), limits, &mut *stop)
                    .map_err(Error::Check);
                Ok(Check::whole(promise, checked("proof", result)?.status))
            }
            "certifies" => Ok(Check::whole(promise, Status::Outside)),
            _ => Ok(Check::whole(promise, Status::None)),
        }
    }
}
