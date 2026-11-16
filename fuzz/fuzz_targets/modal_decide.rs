// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Modal sequents decided: no input panics the search, the derivation it
//! gives or its check, or the countermodel's check, and every answer's
//! evidence passes its check (an error of kind `defect` says it did not).
//! The first byte picks the system, the rest is the modal text.
#![no_main]

use deduxit::modal::{Options, Sequent, System, decide_within};
use deduxit::{ErrorKind, Limits};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some((&bits, text)) = data.split_first() else {
        return;
    };
    let Ok(text) = std::str::from_utf8(text) else {
        return;
    };
    let system = System::ALL[usize::from(bits) % System::ALL.len()];
    let limits = Limits::default()
        .with_occurrences(Some(4096))
        .with_memory_bytes(Some(1 << 26))
        .with_derivation_bytes(Some(1 << 22))
        .with_work(Some(20_000));
    let Ok(sequent) = Sequent::parse_within(text, &limits) else {
        return;
    };
    let options = Options::default().with_system(system);
    if let Err(error) = decide_within(&sequent, &options, &limits, |_| false) {
        assert_ne!(error.kind(), ErrorKind::Defect, "{error}");
    }
});
