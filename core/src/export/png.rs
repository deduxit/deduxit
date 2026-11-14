// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Renders a drawing as a PNG image.
//!
//! [`from_svg`] takes the SVG text of a drawing of [`svg`](super::svg)
//! and the data of the font files to set its text in. It reads no other
//! font, so the image depends on the arguments alone. A text whose font
//! is not among them is left out without an error, so give it the font
//! the drawing names: Euler Math by default
//! ([`Style::font`](super::svg::Style::font)).
//!
//! The image is the drawing's size times [`Options::scale`]. A transparent
//! drawing stays transparent;
//! [`Style::background`](super::svg::Style::background) fills it. The
//! image declares its colours as sRGB and its density as the scale times
//! 96 pixels per inch, so that a viewer shows it at the drawing's size. It
//! carries the drawing's title and description, where it has them, as its
//! `Title` and `Description`.
//!
//! Needs the cargo feature `png` (off by default).
//!
//! # Examples
//!
//! A proof drawn and rendered with Euler Math, from a font file of your
//! own (TeX Live has `Euler-Math.otf`):
//!
#![cfg_attr(feature = "parse", doc = "```no_run")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::Limits;
//! use deduxit::export::{png, svg};
//! use deduxit::linear::{Mode, Options, Sequent, Verdict, prove};
//!
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! let outcome = prove(&sequent, Mode::INTUITIONISTIC, &Options::default())?;
//! let Verdict::Proved(proof) = &outcome.verdict else {
//!     panic!("provable");
//! };
//! let drawing = svg::to_string(&proof.derivation()?, &svg::Style::default())?;
//! let font = std::fs::read("Euler-Math.otf")?;
//! let image = png::from_svg(
//!     &drawing,
//!     &[font.as_slice()],
//!     &png::Options::default(),
//!     &Limits::default(),
//! )?;
//! std::fs::write("proof.png", image)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use super::{Measure, PNG, declared_size, parse, texts};
use crate::Error;
use resvg::tiny_skia::{Pixmap, Transform};

/// The options of a PNG image, beyond the drawing's style. Start from
/// `default()` and set the fields that differ.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// The pixels of the image per pixel of the SVG document.
    pub scale: u32,
    /// The most pixels the image may have, or `None` for no bound. An
    /// image takes up to eight bytes per pixel while it is drawn and
    /// encoded. The bound is checked before the SVG is parsed, against
    /// the size its root declares, and again after.
    pub pixels: Option<u64>,
}

impl Options {
    /// The default scale, sharp on a screen of twice the usual density.
    pub const DEFAULT_SCALE: u32 = 2;
    /// The default bound on the pixels, 64 million: about 512 MiB while
    /// drawn.
    pub const DEFAULT_PIXELS: u64 = 1 << 26;
}

impl Default for Options {
    /// Returns [`DEFAULT_SCALE`](Self::DEFAULT_SCALE) and
    /// [`DEFAULT_PIXELS`](Self::DEFAULT_PIXELS).
    fn default() -> Self {
        Self {
            scale: Self::DEFAULT_SCALE,
            pixels: Some(Self::DEFAULT_PIXELS),
        }
    }
}

impl Options {
    /// Returns the options with another [`scale`](Self::scale).
    #[must_use]
    pub fn with_scale(self, scale: u32) -> Self {
        Self { scale, ..self }
    }

    /// Returns the options with another [`pixels`](Self::pixels).
    #[must_use]
    pub fn with_pixels(self, pixels: Option<u64>) -> Self {
        Self { pixels, ..self }
    }
}

/// Returns an SVG document rendered as a PNG image, its text set in
/// `fonts` (the data of font files), within `limits.memory_bytes`.
///
/// The memory is estimated before the SVG is parsed, since the renderer
/// cannot be stopped once it runs. The estimate counts the drawing's
/// glyphs (usvg sets each as a path, some 1 000 bytes), its elements, the
/// arcs usvg strokes to find their bounds, and the image's pixels. Time
/// is counted too, as the memory the renderer fills in that time, so the
/// bound also keeps a render short: the default bound is at most about
/// 8 s of rendering on a slow core.
///
/// # Errors
///
/// - [`Refusal::Pixels`](crate::Refusal::Pixels) past [`Options::pixels`],
///   checked before the SVG is parsed and again after.
/// - [`Refusal::Memory`](crate::Refusal::Memory) past the memory bound,
///   checked before the SVG is parsed.
/// - [`Error::NotSvg`] for a text that is no document the renderer reads,
///   or that has what no drawing of [`svg`](super::svg) has: an element
///   other than `svg`, `title`, `desc`, `g`, `text`, `path`, `rect` and
///   `circle`, a reference or a document type. The estimate cannot see
///   their cost.
/// - [`Error::RenderFailed`] when the PNG encoder fails.
pub fn from_svg(
    svg: &str,
    fonts: &[&[u8]],
    options: &Options,
    limits: &crate::Limits,
) -> Result<Vec<u8>, Error> {
    let scale = options.scale.max(1);
    let pixels_of = |(width, height): (u64, u64)| {
        let side = |n: u64| n.saturating_mul(u64::from(scale));
        (
            side(width),
            side(height),
            side(width).saturating_mul(side(height)),
        )
    };
    let too_large = |pixels| {
        Error::Refused(crate::limits::Refusal::Pixels {
            pixels,
            limit: options.pixels.unwrap_or(u64::MAX),
        })
    };
    let measure = Measure::of(svg)?;
    // Within the bounds, or why not, for an image of `pixels`.
    let check = |pixels: u64| {
        if options.pixels.is_some_and(|limit| pixels > limit) {
            return Err(too_large(pixels));
        }
        let estimate = measure.estimate(&PNG, pixels);
        match limits.memory_bytes {
            Some(limit) if estimate > limit => {
                Err(Error::Refused(crate::limits::Refusal::Memory {
                    phase: crate::limits::Phase::Render,
                    limit_bytes: limit,
                    needed_bytes: Some(estimate),
                }))
            }
            _ => Ok(()),
        }
    };
    // usvg rounds the declared size up to whole pixels.
    let declared = declared_size(svg).map(|(w, h)| pixels_of((w.ceil() as u64, h.ceil() as u64)));
    check(declared.map_or(0, |(_, _, pixels)| pixels))?;
    let tree = parse(svg, fonts)?;
    let size = tree.size().to_int_size();
    let (width, height, pixels) = pixels_of((u64::from(size.width()), u64::from(size.height())));
    check(pixels)?;
    // tiny-skia refuses what its own sizes cannot hold.
    let fits = |n: u64| u32::try_from(n).ok();
    let mut pixmap = fits(width)
        .zip(fits(height))
        .and_then(|(w, h)| Pixmap::new(w, h))
        .ok_or_else(|| too_large(pixels))?;
    let factor = scale as f32;
    resvg::render(
        &tree,
        Transform::from_scale(factor, factor),
        &mut pixmap.as_mut(),
    );
    let (title, description) = texts(svg);
    encode(&pixmap, scale, title, description).map_err(|e| Error::RenderFailed {
        message: e.to_string(),
    })
}

/// Returns a pixmap as PNG bytes with the image's colour space, density,
/// title and description.
fn encode(
    pixmap: &Pixmap,
    scale: u32,
    title: Option<String>,
    description: Option<String>,
) -> Result<Vec<u8>, png::EncodingError> {
    let mut data = Vec::with_capacity(pixmap.data().len());
    for pixel in pixmap.pixels() {
        let colour = pixel.demultiply();
        data.extend([colour.red(), colour.green(), colour.blue(), colour.alpha()]);
    }
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, pixmap.width(), pixmap.height());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_srgb(png::SrgbRenderingIntent::Perceptual);
    // 96 pixels per inch at scale 1, per metre and rounded.
    let density = u32::try_from((u64::from(scale) * 960_000 + 127) / 254).unwrap_or(u32::MAX);
    encoder.set_pixel_dims(Some(png::PixelDimensions {
        xppu: density,
        yppu: density,
        unit: png::Unit::Meter,
    }));
    for (keyword, text) in [("Title", title), ("Description", description)] {
        if let Some(text) = text {
            encoder.add_itxt_chunk(keyword.to_owned(), text)?;
        }
    }
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&data)?;
    writer.finish()?;
    Ok(bytes)
}
