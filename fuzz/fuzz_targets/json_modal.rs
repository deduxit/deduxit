// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The JSON forms of modal logic: no document panics the sequent's or the
//! outcome's reader, and what either reads writes back as a document that
//! reads back as itself. An outcome's derivation is checked in its
//! system's calculus and its model against the system, under small
//! bounds: either may be refused, but no check may panic.
#![no_main]

use deduxit::modal::{Evidence, Outcome, Sequent, Verdict, calculus};
use deduxit::{Limits, wire};
use libfuzzer_sys::fuzz_target;

/// Reads a document of the form `T` within the limits.
fn read<T: wire::Readable>(text: &str, limits: &Limits) -> Option<T> {
    let mut document = serde_json::Deserializer::from_str(text);
    let value = wire::upgrade(&mut document, limits).ok()?;
    document.end().ok()?;
    Some(value)
}

fuzz_target!(|text: &str| {
    let limits = Limits::default()
        .with_occurrences(Some(4096))
        .with_memory_bytes(Some(1 << 26))
        .with_work(Some(100_000));
    if let Some(sequent) = read::<Sequent>(text, &limits) {
        let written = serde_json::to_string(&sequent).expect("a sequent writes");
        let again: Sequent = read(&written, &limits).expect("a written sequent reads");
        assert_eq!(serde_json::to_string(&again).expect("writes"), written);
    }
    if let Some(outcome) = read::<Outcome>(text, &limits) {
        let written = serde_json::to_string(&outcome).expect("an outcome writes");
        let again: Outcome = read(&written, &limits).expect("a written outcome reads");
        assert_eq!(serde_json::to_string(&again).expect("writes"), written);
        if let Verdict::Valid(Evidence::Derivation(derivation)) = &outcome.verdict
            && let Ok(calculus) = calculus(outcome.system, &limits)
        {
            let _ = derivation.check(&calculus, &limits, |_| false);
        }
        if let Some(countermodel) = &outcome.countermodel {
            let _ = countermodel.check(&limits, |_| false);
        }
    }
});
