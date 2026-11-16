// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The kinds no suite owns: the drawings and texts every suite makes.

use serde::{Deserialize, Serialize};

use super::{Kind, Role, object};
use crate::Error;

/// The shared kinds.
pub(super) static KINDS: [Kind; 4] = [
    Kind {
        name: "drawing",
        role: Role::Rendering,
        form: "drawing",
        level: None,
        stored: false,
        label: "A drawing, as SVG",
    },
    Kind {
        name: "text",
        role: Role::Rendering,
        form: "text",
        level: None,
        stored: false,
        label: "A text in a format: plain, LaTeX or Typst",
    },
    Kind {
        name: "certificate",
        role: Role::Rendering,
        form: "certificate",
        level: None,
        stored: false,
        label: "A certificate that the proof assistant Rocq checks",
    },
    Kind {
        name: "term",
        role: Role::Rendering,
        form: "lambda_term",
        level: None,
        stored: false,
        label: "A proof read back as a lambda-term",
    },
];

/// A drawing's value.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Drawing {
    /// The SVG text.
    pub(super) svg: String,
}

/// A text's value.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Text {
    /// Its format: `text`, `latex` or `typst`.
    pub(super) format: String,
    /// The text.
    pub(super) text: String,
}

/// A certificate's value.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Certificate {
    /// The certificate's text, a Rocq file.
    pub(super) text: String,
}

/// A lambda-term's value.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Term {
    /// The term.
    pub(super) text: String,
    /// Its typing judgement.
    pub(super) judgement: String,
}

/// Checks that `value` is a value of the shared kind `kind`.
pub(super) fn read(kind: &Kind, value: &str) -> Result<(), Error> {
    summary(kind, value).map(drop)
}

/// Returns one line that names a value of the shared kind `kind`.
pub(super) fn summary(kind: &Kind, value: &str) -> Result<String, Error> {
    Ok(match kind.name {
        "drawing" => {
            object::<Drawing>(value, "drawing")?;
            "A drawing".to_owned()
        }
        "text" => {
            let text: Text = object(value, "text")?;
            match text.format.as_str() {
                "latex" => "LaTeX".to_owned(),
                "typst" => "Typst".to_owned(),
                _ => "Text".to_owned(),
            }
        }
        "certificate" => {
            object::<Certificate>(value, "certificate")?;
            "A Rocq certificate".to_owned()
        }
        _ => object::<Term>(value, "term")?.text,
    })
}
