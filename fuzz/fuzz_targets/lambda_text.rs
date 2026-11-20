// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Lambda-terms as text and as JSON: no text panics the parser in any
//! calculus and no document the JSON reader; a term either reads checks
//! or is refused without a panic, and a term that checks prints with its
//! types as a text that reads back and checks as the same text, unless a
//! bound refuses the longer text.
#![no_main]

use deduxit::lambda::{Calculus, Options, Term};
use deduxit::{ErrorKind, Limits, wire};
use libfuzzer_sys::fuzz_target;

/// Checks a term, and where it is well typed, that its text with types
/// reads back in its calculus as a term that prints the same.
fn round_trip(term: &Term, limits: &Limits) {
    if term.check(limits, |_| false).is_err() {
        return;
    }
    let options = Options::default().with_types(true);
    let mut text = String::new();
    if term
        .write_judgement(&mut text, &options, limits, |_| false)
        .is_err()
    {
        return;
    }
    // The text writes every type out, a shared `↔` as its two
    // implications, so it may pass a limit the term kept within: a bound's
    // refusal is no fault, any other error is.
    let again = match Term::parse(&text, term.calculus(), limits) {
        Ok(again) => again,
        Err(error) if error.kind() == ErrorKind::Limit => return,
        Err(error) => panic!("a printed term reads back: {error:?}"),
    };
    match again.check(limits, |_| false) {
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::Limit => return,
        Err(error) => panic!("a printed term checks: {error:?}"),
    }
    let mut twice = String::new();
    again
        .write_judgement(&mut twice, &options, limits, |_| false)
        .expect("a term read back prints");
    assert_eq!(twice, text);
}

fuzz_target!(|data: &[u8]| {
    let Some((&first, rest)) = data.split_first() else {
        return;
    };
    let Ok(text) = std::str::from_utf8(rest) else {
        return;
    };
    let limits = Limits::default()
        .with_occurrences(Some(4096))
        .with_memory_bytes(Some(1 << 26))
        .with_work(Some(100_000));
    let calculus = Calculus::ALL[usize::from(first) % Calculus::ALL.len()];
    if let Ok(term) = Term::parse(text, calculus, &limits) {
        round_trip(&term, &limits);
    }
    let mut document = serde_json::Deserializer::from_str(text);
    if let Ok(term) = wire::upgrade::<Term, _>(&mut document, &limits)
        && document.end().is_ok()
    {
        let written = serde_json::to_string(&term).expect("a term writes");
        let mut again = serde_json::Deserializer::from_str(&written);
        let read: Term = wire::upgrade(&mut again, &limits).expect("a written term reads");
        assert_eq!(serde_json::to_string(&read).expect("writes"), written);
        round_trip(&term, &limits);
    }
});
