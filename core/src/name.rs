// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Atom names: identifiers of the text syntax, in Unicode's composed
//! normal form (NFC), that are no keyword.
//!
//! The rules make a sequent written as text read back as itself, and make
//! a name mean one atom however its accents were encoded.

use crate::Error;
use std::borrow::Cow;
use unicode_normalization::{IsNormalized, UnicodeNormalization, is_nfc_quick};

/// The keywords of the linear syntax: `par` is a connective, `top` and
/// `bot` are constants.
pub(crate) const KEYWORDS: [&str; 3] = ["par", "top", "bot"];

/// The keywords of the ordinary syntax: its constants.
pub(crate) const ORDINARY_KEYWORDS: [&str; 2] = ["true", "false"];

/// The quantifiers' keywords, `forall x.` and `exists x.`. They name no
/// atom, variable or symbol in either syntax.
pub(crate) const RESERVED: [&str; 2] = ["forall", "exists"];

/// Returns whether an identifier may start with `c`: `_` or a character
/// of `XID_Start`.
pub(crate) fn starts_identifier(c: char) -> bool {
    c == '_' || unicode_ident::is_xid_start(c)
}

/// Returns whether `name` is an identifier: `_` or a character of
/// `XID_Start`, then characters of `XID_Continue`. An ASCII name, as most
/// are, is checked without the Unicode tables, since every JSON reader
/// asks this of every name.
pub(crate) fn is_identifier(name: &str) -> bool {
    if name.is_ascii() {
        let mut bytes = name.bytes();
        return bytes
            .next()
            .is_some_and(|b| b == b'_' || b.is_ascii_alphabetic())
            && bytes.all(|b| b == b'_' || b.is_ascii_alphanumeric());
    }
    let mut chars = name.chars();
    chars.next().is_some_and(starts_identifier) && chars.all(unicode_ident::is_xid_continue)
}

/// Returns `name` in NFC, borrowed where it is in NFC already, as every
/// ASCII name is.
pub(crate) fn normalized(name: &str) -> Cow<'_, str> {
    if name.is_ascii() || is_nfc_quick(name.chars()) == IsNormalized::Yes {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(name.nfc().collect())
    }
}

/// Checks that `name` can name an atom of a sequent: an identifier that is
/// no keyword of the linear syntax and no quantifier's keyword.
///
/// Returns [`Error::AtomName`] otherwise.
pub(crate) fn check(name: &str) -> Result<(), Error> {
    if is_identifier(name) && !KEYWORDS.contains(&name) && !RESERVED.contains(&name) {
        Ok(())
    } else {
        Err(Error::AtomName {
            name: name.to_owned(),
        })
    }
}

/// Checks that `name` can name an atom of an ordinary sequent: it passes
/// [`check`], since the atom keeps its name in the translations to linear
/// logic, and is no keyword of the ordinary syntax (`true`, `false`).
///
/// Returns [`Error::AtomName`] otherwise.
pub(crate) fn check_ordinary(name: &str) -> Result<(), Error> {
    check(name)?;
    if ORDINARY_KEYWORDS.contains(&name) {
        Err(Error::AtomName {
            name: name.to_owned(),
        })
    } else {
        Ok(())
    }
}

/// Returns what is wrong with an atom name that [`check`] or
/// [`check_ordinary`] refused, as a phrase.
pub(crate) fn fault(name: &str) -> &'static str {
    if RESERVED.contains(&name) || KEYWORDS.contains(&name) || ORDINARY_KEYWORDS.contains(&name) {
        "is a keyword of the syntax"
    } else {
        "is no identifier of the syntax"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Identifiers pass, and keywords, reserved words and other text are
    /// refused, each with its reason; `true` and `false` are keywords of
    /// the ordinary syntax only.
    #[test]
    fn names_are_identifiers_but_no_keywords() {
        for name in [
            "A",
            "_",
            "a1",
            "x_y",
            "é",
            "P‿start_1",
            "merge·s1",
            "parity",
        ] {
            assert!(check_ordinary(name).is_ok(), "{name}");
        }
        assert!(check("false").is_ok());
        assert!(check_ordinary("false").is_err());
        assert_eq!(fault("true"), "is a keyword of the syntax");
        for (name, why) in [
            ("par", "is a keyword of the syntax"),
            ("top", "is a keyword of the syntax"),
            ("forall", "is a keyword of the syntax"),
            ("exists", "is a keyword of the syntax"),
            ("", "is no identifier of the syntax"),
            ("1a", "is no identifier of the syntax"),
            ("a b", "is no identifier of the syntax"),
            ("a\u{7}", "is no identifier of the syntax"),
        ] {
            for checked in [check(name), check_ordinary(name)] {
                assert!(
                    matches!(checked, Err(Error::AtomName { name: n }) if n == name),
                    "{name}"
                );
            }
            assert_eq!(fault(name), why, "{name}");
        }
    }

    /// A name is composed: `é` written as `e` and a combining acute becomes
    /// the `é` of one code point, and a name already composed is borrowed.
    #[test]
    fn names_are_composed() {
        assert_eq!(normalized("e\u{301}x"), "\u{e9}x");
        assert!(matches!(normalized("\u{e9}x"), Cow::Borrowed(_)));
        assert!(matches!(normalized("Ax"), Cow::Borrowed(_)));
    }
}
