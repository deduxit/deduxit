// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! deduxit's PNG and PDF renderers for a web page, a WebAssembly module of
//! their own. The page loads it on its first PNG or PDF export, so that a
//! visitor who never exports one never downloads it.
//!
//! [`render`] takes an SVG drawing of the bindings' module (`deduxit-web`),
//! the file of the font it names and the page's settings, read over the
//! same tab's defaults as theirs (`deduxit-web-tab`), and returns the
//! bytes. A failure throws the error's JSON as a string, in the bindings'
//! form: `{"code", "kind", "message", "setting", "details"}`.
//!
//! The module reads no clock and starts no thread.

use deduxit::export::{pdf, png};
pub use deduxit_web_tab::Failure;

/// Renders an SVG drawing of the bindings as `png` or `pdf`, and returns
/// its bytes.
///
/// `font` is the bytes of the file of the font the drawing names, the
/// export's `styles.svg.font`, by default Euler Math. No other font sets
/// the drawing's text. A PDF needs `styles.pdf.date`, which the page takes
/// from its clock, since the module reads none.
///
/// # Errors
///
/// An unknown format, a render past `limits.memory_bytes` or the pixel
/// bound, a PDF without a date, and a text that is no drawing of the
/// bindings.
pub fn render(svg: &str, format: &str, font: &[u8], settings: &str) -> Result<Vec<u8>, Failure> {
    let settings = deduxit_web_tab::read(settings)?;
    let (limits, styles) = (&settings.limits, &settings.styles);
    match format {
        "png" => png::from_svg(svg, &[font], &styles.png, limits),
        "pdf" => pdf::from_svg(svg, &[font], &styles.pdf, limits),
        _ => {
            return Err(Failure::refused(
                "unknown_name",
                "malformed",
                "the renders are png and pdf",
            ));
        }
    }
    .map_err(Failure::from)
}

/// The export of the module, which wraps [`render`] for JavaScript.
mod exports {
    use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

    /// See [`crate::render`].
    #[wasm_bindgen]
    pub fn render(
        svg: &str,
        format: &str,
        font: &[u8],
        settings: &str,
    ) -> Result<Vec<u8>, JsValue> {
        crate::render(svg, format, font, settings).map_err(|failure| JsValue::from_str(&failure.0))
    }
}
