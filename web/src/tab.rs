// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The settings of a browser tab, and how a page's settings are read over
//! them. They are the command's defaults where those fit a tab, and
//! smaller bounds where they do not.

use crate::failure::Failure;
use deduxit::export::Styles;
use deduxit::linear::search::{Jobs, Options};
use deduxit::{Clock, Limits, Settings};
use serde_json::Value;

/// The stack the module is linked with, in bytes, as `build.rs` sets it.
pub const STACK_BYTES: usize = parse(env!("DEDUXIT_WEB_STACK_BYTES"));

/// The memory bound of a call: 96 MiB. A tab on iOS gets about 256 MiB of
/// WebAssembly memory in all. What the bound does not count has the rest:
/// the forest, the stack, the documents in and out, and the drawing.
pub const MEMORY_BYTES: u64 = 96 << 20;

/// The bound on a sequent's formula occurrences: a million. Their forest
/// takes about 25 MB, and a session, a proof and a net each hold a copy
/// of it.
pub const OCCURRENCES: u64 = 1_000_000;

/// The tab's bound on first-order terms (`limits.terms`): as many as its
/// occurrences, the library's rule, so that a sequent's or a session's
/// terms take no more memory than its forest. The library's fifty million
/// would pass the tab's memory before a session's table of terms refused
/// a step.
pub const TERMS: u64 = OCCURRENCES;

/// The bound on a derivation's estimated size: 4 MiB, a sixteenth of the
/// command's. A drawing's peak is several times its estimate, and a tree
/// of that size is far wider than a screen.
pub const DERIVATION_BYTES: u64 = 4 << 20;

/// The work a call may do. It is the page's budget: the module reads no
/// clock, so a search ends at its work bound or not at all. It is about a
/// second of search. The backward search did 68 million units a second in
/// Node on a laptop's slower cores, and the forward one fewer or more.
pub const WORK: u64 = 1 << 26;

/// The work the classical refuter may do after a search that did not
/// prove its goal, in the refuter's own units: a sixteenth of the
/// library's default. The refuter looks for truth values of the atoms
/// that make the goal's classical reading false.
pub const CLASSICAL_WORK: u64 = Options::DEFAULT_CLASSICAL_WORK >> 4;

/// The deepest recursion of a search: 384 levels.
///
/// The engine's frames are on the module's stack ([`STACK_BYTES`]) and on
/// the host's own. A page cannot set the host's stack, which is about
/// 1 MiB in Node and in a browser, and it runs out first. On the path
/// whose levels are the largest, a Mix of pairs, the optimized module
/// recursed to 768 levels and not to 1 024 in Node. So the limit is half
/// the deepest that held.
pub const RECURSION_DEPTH: u32 = 384;

/// The bound on the bytes the drawings of an elimination's steps take
/// together: 16 MiB, four drawings at the tab's bound on one
/// ([`DERIVATION_BYTES`]), as the command's is four at its own. The page
/// holds every step's drawing, and the command's 256 MiB is more than a
/// tab's whole memory bound ([`MEMORY_BYTES`]).
pub const DRAWN_BYTES: u64 = 4 * DERIVATION_BYTES;

/// Returns the tab's settings. They are the command's except for:
///
/// - the bounds above;
/// - one thread;
/// - no bound on copies, since the work bound ends a search;
/// - no clock;
/// - drawings with an id on every formula and a description for a screen
///   reader.
pub fn defaults() -> Settings {
    let limits = Limits::default()
        .with_memory_bytes(Some(MEMORY_BYTES))
        .with_occurrences(Some(OCCURRENCES))
        .with_terms(Some(TERMS))
        .with_derivation_bytes(Some(DERIVATION_BYTES))
        .with_work(Some(WORK))
        .with_recursion_depth(RECURSION_DEPTH);
    let search = Options::default()
        .with_copies(None)
        .with_jobs(Jobs::Count(1))
        .with_classical_work(CLASSICAL_WORK);
    let clock = Clock::default()
        .with_time_limit_ms(None)
        .with_pool_after_ms(None);
    let mut styles = Styles::default();
    styles.svg = styles.svg.with_ids(true).with_description(true);
    let cut = deduxit::linear::proofs::cut::Options::default().with_drawn_bytes(Some(DRAWN_BYTES));
    Settings::default()
        .with_cut(cut)
        .with_clock(clock)
        .with_limits(limits)
        .with_search(search)
        .with_styles(styles)
}

/// Reads a page's settings over the tab's. Every key the page gives
/// replaces the tab's value there, an object key by key. Every other key
/// keeps the tab's value. An empty text is the tab's settings.
///
/// # Errors
///
/// A text that is no JSON, and a key or value the settings refuse.
pub fn read(json: &str) -> Result<Settings, Failure> {
    if json.trim().is_empty() {
        return Ok(defaults());
    }
    let given: Value = serde_json::from_str(json).map_err(Failure::json)?;
    let mut merged = serde_json::to_value(defaults()).map_err(Failure::json)?;
    overlay(&mut merged, given, "the settings")?;
    serde_json::from_value(merged).map_err(Failure::json)
}

/// Writes `given` over `base`: objects key by key, every other value
/// whole.
///
/// Where `base` is an object, `given` must be an object or a string, a
/// name the option reads, such as a math font's for `styles.svg.font`.
/// Anything else is refused. An array matters most: the settings' reader
/// would take it for the fields in order and the command's defaults for
/// the rest.
fn overlay(base: &mut Value, given: Value, at: &str) -> Result<(), Failure> {
    match (base, given) {
        (Value::Object(base), Value::Object(given)) => {
            for (key, value) in given {
                match base.get_mut(&key) {
                    Some(slot) => overlay(slot, value, &format!("`{key}`"))?,
                    None => {
                        base.insert(key, value);
                    }
                }
            }
            Ok(())
        }
        (Value::Object(_), given) if !given.is_string() => Err(Failure::refused(
            "json",
            "malformed",
            &format!("{at} must be a JSON object"),
        )),
        (base, given) => {
            *base = given;
            Ok(())
        }
    }
}

/// Reads a decimal number at compile time.
const fn parse(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut value = 0;
    let mut i = 0;
    while i < bytes.len() {
        value = value * 10 + (bytes[i] - b'0') as usize;
        i += 1;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use deduxit::export::svg::MathFont;

    /// A page's key replaces the tab's value and leaves its siblings. A
    /// misspelt key is refused, and so is a value that is no object where
    /// one belongs.
    #[test]
    fn a_page_s_key_is_read_over_the_tab_s() {
        let settings = read(r#"{"limits": {"work": 5}}"#).unwrap();
        assert_eq!(settings.limits.work, Some(5));
        assert_eq!(settings.limits.memory_bytes, Some(MEMORY_BYTES));
        assert_eq!(read("").unwrap(), defaults());
        assert!(read(r#"{"limits": {"budget": 5}}"#).is_err());
        // An array is no object, though serde would read one as a struct.
        for array in ["[]", r#"{"limits": []}"#, r#"{"limits": [null]}"#] {
            assert!(read(array).is_err(), "{array}");
        }
    }

    /// A math font is chosen by its name over the tab's font, and a name
    /// no option reads is refused.
    #[test]
    fn a_font_is_chosen_by_its_name() {
        let settings = read(r#"{"styles": {"svg": {"font": "stix-two"}}}"#).unwrap();
        assert_eq!(settings.styles.svg.font, MathFont::StixTwo.font());
        assert!(settings.styles.svg.ids);
        assert!(read(r#"{"styles": {"svg": {"font": "stix"}}}"#).is_err());
        assert!(read(r#"{"limits": "none"}"#).is_err());
    }
}
