// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Renders a drawing as a PDF document of one page.
//!
//! [`from_svg`] takes the SVG text of a drawing of [`svg`](super::svg),
//! the data of the font files to set its text in, and the date the
//! document was made ([`Options::date`]). Every PDF/A document needs a
//! date, and the crate reads no clock, so without one `from_svg` answers
//! [`Error::NoDate`]. It reads no other font and writes no random
//! identifier, so the same arguments give the same bytes. A text whose
//! font is not among the fonts given is left out without an error, so
//! give it the font the drawing names: Euler Math by default
//! ([`Style::font`](super::svg::Style::font)).
//!
//! The document is written by krilla and is always PDF/A, an archival
//! standard. krilla checks it against the standard before returning it:
//! - PDF/A-4 (PDF 2.0) by default;
//! - PDF/A-2u (PDF 1.7) with [`Options::compatible`], for a tool or an
//!   archive that does not take PDF 2.0;
//! - PDF/A-2a with PDF/UA-1 (PDF 1.7) with [`Options::accessible`].
//!
//! The page has the drawing's size. A pixel of the drawing is three
//! quarters of a point, as in CSS, so text of 16 pixels is set at 12
//! points. Text stays text, in a subset of its font embedded in the
//! document; with [`Options::embed_text`] off it is drawn as outlines.
//!
//! Needs the cargo feature `pdf` (off by default).
//!
//! # Examples
//!
//! A proof drawn and rendered with Euler Math, from a font file of your
//! own (TeX Live has `Euler-Math.otf`), dated now:
//!
#![cfg_attr(feature = "parse", doc = "```no_run")]
#![cfg_attr(not(feature = "parse"), doc = "```ignore")]
//! use deduxit::Limits;
//! use deduxit::export::{pdf, svg};
//! use deduxit::linear::{Mode, Options, Sequent, Verdict, prove};
//! use std::time::{SystemTime, UNIX_EPOCH};
//!
//! let sequent: Sequent = "A, A -o B |- B".parse()?;
//! let outcome = prove(&sequent, Mode::INTUITIONISTIC, &Options::default())?;
//! let Verdict::Proved(proof) = &outcome.verdict else {
//!     panic!("provable");
//! };
//! let drawing = svg::to_string(&proof.derivation()?, &svg::Style::default())?;
//! let font = std::fs::read("Euler-Math.otf")?;
//! let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
//! let options = pdf::Options::default().with_date(Some(pdf::Date::from_unix(now)));
//! let document = pdf::from_svg(&drawing, &[font.as_slice()], &options, &Limits::default())?;
//! std::fs::write("proof.pdf", document)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use super::{Measure, PDF, PDF_OUTLINES, parse, texts};
use crate::Error;
use krilla::configure::{Accessibility, Archival, ConfigurationBuilder, PdfVersion};
use krilla::destination::XyzDestination;
use krilla::geom::{Point, Size};
use krilla::metadata::{DateTime, Metadata};
use krilla::outline::{Outline, OutlineNode};
use krilla::page::PageSettings;
use krilla::tagging::{ContentTag, Tag, TagGroup, TagTree};
use krilla::{Document, SerializeSettings};
use krilla_svg::{SurfaceExt, SvgSettings};

/// Points per pixel of the drawing, as CSS has them.
const POINTS_PER_PIXEL: f32 = 0.75;

/// The options of a PDF document, beyond the drawing's style. Start from
/// `default()` and set the fields that differ; [`date`](Self::date) has
/// no default.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(default, deny_unknown_fields))]
pub struct Options {
    /// Whether text is embedded as text, selectable with its font, rather
    /// than drawn as outlines.
    pub embed_text: bool,
    /// Whether the document is PDF/A-2u (PDF 1.7) in place of PDF/A-4
    /// (PDF 2.0), for a tool or an archive that does not take PDF 2.0,
    /// such as Typst's PDF images and several national archives.
    pub compatible: bool,
    /// Whether the document is accessible: PDF/UA-1 with PDF/A-2a (PDF
    /// 1.7), whatever `compatible` says. The document is tagged, and its
    /// drawing is one figure whose alternative text is the drawing's
    /// description: the numbered steps of a derivation, or the links of a
    /// net. A screen reader reads that text, not the drawing, and a long
    /// proof read out line by line may still be hard to follow.
    pub accessible: bool,
    /// The document's title, or `None` for the drawing's title.
    pub title: Option<String>,
    /// The language of the document's text, as a language tag.
    pub language: String,
    /// The date the document was made, which every PDF/A part requires.
    /// The crate reads no clock, so the caller gives it: [`from_svg`]
    /// refuses to render without one.
    pub date: Option<Date>,
}

impl Default for Options {
    /// Returns PDF/A-4 with text embedded as text, the drawing's title,
    /// English, and no date: a caller must still give one.
    fn default() -> Self {
        Self {
            embed_text: true,
            compatible: false,
            accessible: false,
            title: None,
            language: "en".to_owned(),
            date: None,
        }
    }
}

impl Options {
    /// Returns the options with another [`embed_text`](Self::embed_text).
    #[must_use]
    pub fn with_embed_text(self, embed_text: bool) -> Self {
        Self { embed_text, ..self }
    }

    /// Returns the options with another [`compatible`](Self::compatible).
    #[must_use]
    pub fn with_compatible(self, compatible: bool) -> Self {
        Self { compatible, ..self }
    }

    /// Returns the options with another [`accessible`](Self::accessible).
    #[must_use]
    pub fn with_accessible(self, accessible: bool) -> Self {
        Self { accessible, ..self }
    }

    /// Returns the options with another [`title`](Self::title).
    #[must_use]
    pub fn with_title(self, title: Option<String>) -> Self {
        Self { title, ..self }
    }

    /// Returns the options with another [`language`](Self::language).
    #[must_use]
    pub fn with_language(self, language: String) -> Self {
        Self { language, ..self }
    }

    /// Returns the options with another [`date`](Self::date).
    #[must_use]
    pub fn with_date(self, date: Option<Date>) -> Self {
        Self { date, ..self }
    }
}

/// A moment in UTC, to the second.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Date {
    /// The year.
    pub year: u16,
    /// The month, 1 to 12.
    pub month: u8,
    /// The day of the month, 1 to 31.
    pub day: u8,
    /// The hour, 0 to 23.
    pub hour: u8,
    /// The minute, 0 to 59.
    pub minute: u8,
    /// The second, 0 to 59.
    pub second: u8,
}

impl Date {
    /// Returns the moment `seconds` after the Unix epoch, in UTC.
    ///
    /// `SOURCE_DATE_EPOCH`, the date of reproducible builds, counts the
    /// same way. A year past 65 535 is given as 65 535.
    ///
    /// # Examples
    ///
    /// ```
    /// use deduxit::export::pdf::Date;
    ///
    /// let date = Date::from_unix(951_782_400);
    /// assert_eq!((date.year, date.month, date.day), (2000, 2, 29));
    /// ```
    pub fn from_unix(seconds: u64) -> Self {
        let (days, rest) = (seconds / 86_400, seconds % 86_400);
        // The civil date of a day count, after Howard Hinnant's
        // `civil_from_days`, for days on or after 1970-01-01.
        let z = days + 719_468;
        let era = z / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = yoe + era * 400 + u64::from(month <= 2);
        Self {
            year: u16::try_from(year).unwrap_or(u16::MAX),
            month: month as u8,
            day: day as u8,
            hour: (rest / 3600) as u8,
            minute: (rest % 3600 / 60) as u8,
            second: (rest % 60) as u8,
        }
    }
}

/// Returns an SVG document as a PDF document of one page, its text set in
/// `fonts` (the data of font files), within `limits.memory_bytes`.
///
/// The memory is estimated before the SVG is parsed, since the renderer
/// cannot be stopped once it runs. The estimate counts the drawing's
/// glyphs (some 1 300 bytes each embedded as text, 2 000 as outlines),
/// its elements, and the arcs usvg strokes to find their bounds. Time is
/// counted too, as the memory the renderer fills in that time, so the
/// bound also keeps a render short: the default bound is at most about
/// 8 s of rendering on a slow core.
///
/// # Errors
///
/// - [`Error::NoDate`] without [`Options::date`].
/// - [`Refusal::Memory`](crate::Refusal::Memory) past the memory bound,
///   checked before the SVG is parsed.
/// - [`Error::NotSvg`] for a text that is no document the renderer reads,
///   or that has what no drawing of [`svg`](super::svg) has, as
///   [`png::from_svg`](super::png::from_svg) says.
/// - [`Error::RenderFailed`] for a document that does not conform.
pub fn from_svg(
    svg: &str,
    fonts: &[&[u8]],
    options: &Options,
    limits: &crate::Limits,
) -> Result<Vec<u8>, Error> {
    let date = options.date.ok_or(Error::NoDate)?;
    let costs = if options.embed_text {
        PDF
    } else {
        PDF_OUTLINES
    };
    let estimate = Measure::of(svg)?.estimate(&costs, 0);
    if let Some(limit) = limits.memory_bytes.filter(|&limit| estimate > limit) {
        return Err(Error::Refused(crate::limits::Refusal::Memory {
            phase: crate::limits::Phase::Render,
            limit_bytes: limit,
            needed_bytes: Some(estimate),
        }));
    }
    let tree = parse(svg, fonts)?;
    let failed = |what: String| Error::RenderFailed { message: what };
    let size = Size::from_wh(
        tree.size().width() * POINTS_PER_PIXEL,
        tree.size().height() * POINTS_PER_PIXEL,
    )
    .ok_or_else(|| failed("the drawing has no size".to_owned()))?;
    let (drawn_title, description) = texts(svg);
    let title = options.title.clone().or(drawn_title).unwrap_or_default();

    let builder = match (options.accessible, options.compatible) {
        (true, _) => ConfigurationBuilder::new()
            .with_version(PdfVersion::Pdf17)
            .with_archival_validator(Archival::A2_A)
            .with_accessibility_validator(Accessibility::UA1),
        (false, true) => ConfigurationBuilder::new()
            .with_version(PdfVersion::Pdf17)
            .with_archival_validator(Archival::A2_U),
        (false, false) => ConfigurationBuilder::new()
            .with_version(PdfVersion::Pdf20)
            .with_archival_validator(Archival::A4),
    };
    let configuration = builder.finish().map_err(|e| failed(format!("{e:?}")))?;
    let mut document = Document::new_with(SerializeSettings {
        configuration,
        enable_tagging: options.accessible,
        ..SerializeSettings::default()
    });
    let created = DateTime::new(date.year)
        .month(date.month)
        .day(date.day)
        .hour(date.hour)
        .minute(date.minute)
        .second(date.second)
        .utc_offset_hour(0);
    document.set_metadata(
        Metadata::new()
            .title(title.clone())
            .language(options.language.clone())
            .creation_date(created),
    );

    let mut page = document.start_page_with(PageSettings::new(size));
    let mut surface = page.surface();
    let settings = SvgSettings {
        embed_text: options.embed_text,
        ..SvgSettings::default()
    };
    let figure = options
        .accessible
        .then(|| surface.start_tagged(ContentTag::Other));
    let drawn = surface.draw_svg(&tree, size, settings);
    if figure.is_some() {
        surface.end_tagged();
    }
    surface.finish();
    page.finish();
    drawn.ok_or_else(|| failed("the drawing could not be converted".to_owned()))?;
    if let Some(figure) = figure {
        let mut group = TagGroup::new(Tag::Figure(Some(description.unwrap_or(title.clone()))));
        group.push(figure);
        let mut tags = TagTree::new();
        tags.push(group);
        document.set_tag_tree(tags);
        let mut outline = Outline::new();
        outline.push_child(OutlineNode::new(
            title,
            XyzDestination::new(0, Point::from_xy(0.0, 0.0)),
        ));
        document.set_outline(outline);
    }
    document.finish().map_err(|e| failed(format!("{e:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The Unix epoch, a leap day and a moment past 2100 come out as their
    /// calendar dates.
    #[test]
    fn dates_from_unix_seconds() {
        let date = |s| {
            let d = Date::from_unix(s);
            (d.year, d.month, d.day, d.hour, d.minute, d.second)
        };
        assert_eq!(date(0), (1970, 1, 1, 0, 0, 0));
        assert_eq!(date(951_782_400), (2000, 2, 29, 0, 0, 0));
        assert_eq!(date(1_791_158_399), (2026, 10, 4, 23, 59, 59));
        assert_eq!(date(4_107_542_400), (2100, 3, 1, 0, 0, 0));
    }
}
