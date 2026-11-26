// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The calls of a tab of lambda-terms, exported with the prefix
//! `lambda_`.
//!
//! A tab holds a term or a type. A term is a statement, the term's JSON
//! with the text as typed, which [`read`] writes; [`check`] checks it,
//! and the checked term's typing derivation is drawn by [`draw`] and
//! exported by [`export`]. A term is reduced step by step by [`reduce`],
//! and an untyped one typed at its principal type by [`infer`]. A type is
//! an ordinary statement in intuitionistic logic, which [`read_type`]
//! writes; [`inhabit`] finds a term of it or shows it has none, and
//! [`enumerate`] lists and counts its terms. Each call runs the
//! protocol's operation of that name (`lambda.check`, …) under the tab's
//! settings.

use crate::Failure;
use crate::tab;
use deduxit::lambda::{ReduceOptions, Term};
use deduxit::{Progress, Settings, protocol, wire};
use serde::Deserialize;
use serde_json::Value;

/// The stop of every call. The page cancels a call by stopping its
/// worker, so only the bounds of the settings end a call early.
const fn never(_: Progress) -> bool {
    false
}

/// Runs a protocol operation of the lambda suite under the tab's
/// settings and returns its output.
fn ran(
    operation: &str,
    inputs: &[&str],
    parameters: &str,
    settings: &Settings,
) -> Result<String, Failure> {
    Ok(protocol::run(operation, inputs, parameters, settings, never)?.value)
}

/// Returns the statement of a lambda-term typed as `--lambda-types`
/// prints terms (`\x:a. x`, or a judgement `x : a |- f x`), in
/// `calculus` (`simple`, `linear` or `affine`), or an untyped term
/// (`\x. x x`) in `untyped`, `untyped_linear` or `untyped_affine`: the
/// term's JSON with the text as typed.
///
/// # Errors
///
/// An unknown calculus, the parser's error with its place, a term whose
/// types do not fit (`ill_typed`, with the place), a text past
/// `limits.occurrences`.
pub fn read(text: &str, calculus: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    Ok(protocol::statement(
        "lambda",
        text,
        calculus,
        &settings.limits,
    )?)
}

/// Checks a term: its JSON with the type of every node under `typing`
/// and its judgement with every type, as the protocol's `lambda.check`
/// writes it.
///
/// # Errors
///
/// A term that is not well typed (`ill_typed`, with the reason and the
/// node), a check past the settings' bounds.
pub fn check(term: &str, settings: &str) -> Result<String, Failure> {
    ran("lambda.check", &[term], "", &tab::read(settings)?)
}

/// The drawing value's form.
#[derive(Deserialize)]
struct Drawing {
    /// The SVG text.
    svg: String,
}

/// Draws a checked term's typing derivation as SVG: a judgement per
/// subterm, each under its rule of natural deduction.
///
/// # Errors
///
/// A term that is not well typed, a drawing past
/// `limits.derivation_bytes`.
pub fn draw(typing: &str, settings: &str) -> Result<String, Failure> {
    let drawn = ran("lambda.draw", &[typing], "", &tab::read(settings)?)?;
    let drawing: Drawing = serde_json::from_str(&drawn).map_err(Failure::json)?;
    Ok(drawing.svg)
}

/// The text value's form.
#[derive(Deserialize)]
struct Text {
    /// The text.
    text: String,
}

/// Writes a checked term's typing derivation in `latex` or `typst`, in
/// the settings' styles.
///
/// # Errors
///
/// An unknown format, a term that is not well typed, a text past
/// `limits.derivation_bytes`.
pub fn export(typing: &str, format: &str, settings: &str) -> Result<String, Failure> {
    let parameters = serde_json::json!({ "format": format }).to_string();
    let written = ran(
        "lambda.export",
        &[typing],
        &parameters,
        &tab::read(settings)?,
    )?;
    let text: Text = serde_json::from_str(&written).map_err(Failure::json)?;
    Ok(text.text)
}

/// Returns the statement of a simple type typed as a formula of ordinary
/// logic (`a -> b -> a`, or with free variables' types before `|-`): an
/// ordinary statement claimed in intuitionistic logic.
///
/// # Errors
///
/// The parser's error with its place, a text past `limits.occurrences`.
pub fn read_type(text: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    Ok(protocol::statement(
        "ordinary",
        text,
        "intuitionistic",
        &settings.limits,
    )?)
}

/// Adds to a term's document its `text` and its `judgement`, written as
/// the settings' `lambda` says, as the bindings' `term` call writes a
/// proof's term.
fn printed(term: &mut Value, settings: &Settings) -> Result<(), Failure> {
    let read = read_term(term, settings)?;
    let (mut written, mut judgement) = (String::new(), String::new());
    read.write(&mut written, &settings.lambda, &settings.limits, never)?;
    read.write_judgement(&mut judgement, &settings.lambda, &settings.limits, never)?;
    if let Value::Object(keys) = term {
        keys.insert("text".to_owned(), Value::String(written));
        keys.insert("judgement".to_owned(), Value::String(judgement));
    }
    Ok(())
}

/// Reduces a term by β, as the protocol's `lambda.reduce` answers: the
/// term reached, each step's redex and how the reduction ended, with the
/// term reached as `text` and `judgement`, and under `next` the place of
/// the redex the next step would reduce in `text`, `{"start", "end"}` in
/// UTF-16 code units, or `null` in normal form. `options` is
/// `{"strategy", "steps", "nodes"}`, each key optional, or `""` for the
/// defaults; `{"steps": 1}` takes one step, and `{"steps": 0}` marks the
/// first redex of the term as it is.
///
/// # Errors
///
/// A term its calculus refuses (`ill_typed`, with the reason), options
/// that are no such object.
pub fn reduce(term: &str, options: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let reduced = ran("lambda.reduce", &[term], options, &settings)?;
    let mut answer: Value = serde_json::from_str(&reduced).map_err(Failure::json)?;
    let chosen: ReduceOptions = if options.trim().is_empty() {
        ReduceOptions::default()
    } else {
        serde_json::from_str(options).map_err(Failure::json)?
    };
    if let Some(reached) = answer.get_mut("term") {
        printed(reached, &settings)?;
        let text = reached["text"].as_str().unwrap_or_default().to_owned();
        let read = read_term(reached, &settings)?;
        let next = match chosen.strategy.pick(&read) {
            Some(redex) => {
                let mut written = String::new();
                let span = read.write_marked(
                    &mut written,
                    redex,
                    &settings.lambda,
                    &settings.limits,
                    never,
                )?;
                debug_assert_eq!(written, text);
                let utf16 = |s: &str| s.encode_utf16().count();
                let start = utf16(&written[..span.start]);
                serde_json::json!({ "start": start, "end": start + utf16(&written[span]) })
            }
            None => Value::Null,
        };
        if let Value::Object(keys) = &mut answer {
            keys.insert("next".to_owned(), next);
        }
    }
    Ok(answer.to_string())
}

/// Types an untyped term at its principal simple type, as the protocol's
/// `lambda.infer` answers: the typed term's JSON, a term of the simple
/// calculus that `check` and `draw` take, with its `text` and its
/// `judgement` written with every type.
///
/// # Errors
///
/// A term of a typed calculus (`not_applicable`), a term its calculus
/// refuses or that has no simple type (`ill_typed`, with the reason).
pub fn infer(term: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let typed = ran("lambda.infer", &[term], "", &settings)?;
    let mut answer: Value = serde_json::from_str(&typed).map_err(Failure::json)?;
    let mut shown = settings;
    shown.lambda = shown.lambda.with_types(true);
    printed(&mut answer, &shown)?;
    Ok(answer.to_string())
}

/// Reads a term out of a JSON value, under the settings' limits.
fn read_term(value: &Value, settings: &Settings) -> Result<Term, Failure> {
    let text = value.to_string();
    Ok(wire::upgrade(
        &mut serde_json::Deserializer::from_str(&text),
        &settings.limits,
    )?)
}

/// Finds a term of a type, as the protocol's `lambda.inhabit` answers:
/// the term, checked, with its `text` and `judgement`, or that the type
/// has none, with the Kripke model that shows it where there is one, or
/// that the bounds left it undecided.
///
/// # Errors
///
/// A type that cannot be read, a bound that refused the evidence.
pub fn inhabit(ty: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let found = ran("lambda.inhabit", &[ty], "", &settings)?;
    let mut answer: Value = serde_json::from_str(&found).map_err(Failure::json)?;
    if let Some(term) = answer.get_mut("term").filter(|term| term.is_object()) {
        printed(term, &settings)?;
    }
    Ok(answer.to_string())
}

/// Lists and counts a type's terms up to βη-equality, as the protocol's
/// `lambda.enumerate` answers, each term with its `text` and
/// `judgement`; `options` is `{"depth", "max_terms"}`, each key
/// optional, or `""` for the defaults.
///
/// # Errors
///
/// A type that cannot be read or that has `∨` or `⊥`, options that are
/// no such object.
pub fn enumerate(ty: &str, options: &str, settings: &str) -> Result<String, Failure> {
    let settings = tab::read(settings)?;
    let listed = ran("lambda.enumerate", &[ty], options, &settings)?;
    let mut answer: Value = serde_json::from_str(&listed).map_err(Failure::json)?;
    if let Some(terms) = answer.get_mut("terms").and_then(Value::as_array_mut) {
        for term in terms {
            printed(term, &settings)?;
        }
    }
    Ok(answer.to_string())
}
