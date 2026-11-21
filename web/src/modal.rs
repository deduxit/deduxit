// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of a tab of modal logic, exported with the prefix `modal_`.
//!
//! A tab holds a statement: the modal sequent's JSON with `system`
//! beside its keys, as [`read`] writes it. [`decide`] decides it; the
//! outcome's derivation and countermodel are drawn by [`draw`], its
//! derivation exported by [`export`], its model taken by
//! [`countermodel`]. A model the visitor edits is checked against the
//! statement by [`evaluate`] and, where the sequent fails, drawn by
//! [`model_drawing`]. Each call runs the protocol's operation of that
//! name (`modal.decide`, …) under the tab's settings.

use crate::Failure;
use crate::tab;
use deduxit::export::svg;
use deduxit::kripke::Model;
use deduxit::modal::{Countermodel, Sequent, System};
use deduxit::{Progress, Settings, protocol, wire};
use serde::Deserialize;

/// The stop of every call. The page cancels a call by stopping its
/// worker, so only the bounds of the settings end a call early.
const fn never(_: Progress) -> bool {
    false
}

/// Runs a protocol operation of the modal suite under the tab's
/// settings and returns its output.
fn ran(
    operation: &str,
    inputs: &[&str],
    parameters: &str,
    settings: &Settings,
) -> Result<String, Failure> {
    Ok(protocol::run(operation, inputs, parameters, settings, never)?.value)
}

/// Returns the statement of a modal sequent typed in the modal syntax
/// (`[]p -> p`), claimed in `system` (one of the fifteen names, or K
/// followed by the axioms' letters): the sequent's JSON with `system` and
/// the text as typed.
///
/// # Errors
///
/// An unknown system, the parser's error, a sequent past
/// `limits.occurrences`.
pub fn read(text: &str, system: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    Ok(protocol::statement(
        "modal",
        text,
        system,
        &settings.limits,
    )?)
}

/// Decides a statement in its system: the outcome, with a derivation in
/// the system's labelled calculus where it is valid, or a checked
/// countermodel where it is not and the model is small enough to check.
///
/// # Errors
///
/// A statement that cannot be read, a bound that refuses the evidence.
/// A search a bound ended is an outcome whose verdict is `unknown`.
pub fn decide(statement: &str, settings: &str) -> Result<String, Failure> {
    ran("modal.decide", &[statement], "", &tab::read(settings)?)
}

/// Returns the countermodel of a not-valid outcome, as a Kripke model's
/// JSON: each world's atoms and the worlds it sees, `world` where the
/// sequent fails.
///
/// # Errors
///
/// An outcome that holds no countermodel (`invalid_option`, key
/// `inputs`), or one that cannot be read.
pub fn countermodel(outcome: &str, settings: &str) -> Result<String, Failure> {
    ran("modal.countermodel", &[outcome], "", &tab::read(settings)?)
}

/// Checks a Kripke model against a statement: the frame condition the
/// model's arrows fail with the worlds that show it (`frame`), or whether
/// the sequent holds at `world` or at every world (`holds`), and where it
/// fails (`failing`) each compound formula's value there with the world
/// that shows a false `□` or a true `◇` (`values`).
///
/// # Errors
///
/// A statement or model that cannot be read, a world the model lacks.
pub fn evaluate(
    statement: &str,
    model: &str,
    world: Option<u32>,
    settings: &str,
) -> Result<String, Failure> {
    let parameters = world.map_or_else(String::new, |w| format!(r#"{{"world": {w}}}"#));
    ran(
        "modal.evaluate",
        &[statement, model],
        &parameters,
        &tab::read(settings)?,
    )
}

/// The `drawing` value's form.
#[derive(Deserialize)]
struct Drawing {
    /// The SVG text.
    svg: String,
}

/// Draws an outcome as SVG: a valid one's derivation as a tree, a not
/// valid one's countermodel with an arrow from each world to each world
/// it sees.
///
/// # Errors
///
/// An outcome with neither (`no_derivation`), a drawing past
/// `limits.derivation_bytes`.
pub fn draw(outcome: &str, settings: &str) -> Result<String, Failure> {
    let drawn = ran("modal.draw", &[outcome], "", &tab::read(settings)?)?;
    let drawing: Drawing = serde_json::from_str(&drawn).map_err(Failure::json)?;
    Ok(drawing.svg)
}

/// The text value's form.
#[derive(Deserialize)]
struct Text {
    /// The text.
    text: String,
}

/// Writes a valid outcome's derivation in `latex` or `typst`, in the
/// settings' styles.
///
/// # Errors
///
/// An unknown format, an outcome without a derivation, a text past
/// `limits.derivation_bytes`.
pub fn export(outcome: &str, format: &str, settings: &str) -> Result<String, Failure> {
    let parameters = serde_json::json!({ "format": format }).to_string();
    let written = ran(
        "modal.export",
        &[outcome],
        &parameters,
        &tab::read(settings)?,
    )?;
    let text: Text = serde_json::from_str(&written).map_err(Failure::json)?;
    Ok(text.text)
}

/// Returns the system's labelled calculus as a rule file, the one a
/// valid outcome's derivation is in.
///
/// # Errors
///
/// A statement that cannot be read.
pub fn calculus(statement: &str, settings: &str) -> Result<String, Failure> {
    ran("modal.calculus", &[statement], "", &tab::read(settings)?)
}

/// What a statement claims beside its sequent.
#[derive(Deserialize)]
struct Claim {
    /// The system.
    system: System,
}

/// Draws a model the visitor gave as the countermodel of the statement at
/// `world`, after checking that it is one: the model's arrows meet the
/// system's frame conditions, and at `world` every formula left of `⊢` is
/// true and every formula right of it false.
///
/// # Errors
///
/// A model that is none of the system (`invalid_model`), or where the sequent
/// holds at `world` (`invalid_model`, naming the formula), a drawing past
/// `limits.derivation_bytes`.
pub fn model_drawing(
    statement: &str,
    model: &str,
    world: u32,
    settings: &str,
) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let limits = &settings.limits;
    let claim: Claim = serde_json::from_str(statement).map_err(Failure::json)?;
    let sequent: Sequent =
        wire::upgrade(&mut serde_json::Deserializer::from_str(statement), limits)?;
    let read: Model = wire::upgrade(&mut serde_json::Deserializer::from_str(model), limits)?;
    read.check_world(world)?;
    let model = Model::new(read.worlds, world);
    let countermodel = Countermodel::new(sequent, claim.system, model);
    // The stop as the protocol's `modal.draw` passes it, so the drawing is
    // compiled once for both.
    let stop: &mut dyn FnMut(Progress) -> bool = &mut never;
    Ok(svg::modal_countermodel(
        &countermodel,
        &settings.styles.svg,
        limits,
        stop,
    )?)
}
