// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The TPTP reader of ordinary problems: no input panics.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|text: &str| {
    let _ = deduxit::ordinary::read_tptp(text, &deduxit::Limits::default());
});
