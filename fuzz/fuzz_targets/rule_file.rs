// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A rule file, as `check --calculus` reads it: no input panics the
//! reader, which keeps to its memory bound; a calculus read writes back
//! as a document that reads back as the same calculus.
#![no_main]

use deduxit::rules::Calculus;
use deduxit::{Limits, wire};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let limits = Limits::default()
        .with_memory_bytes(Some(1 << 26))
        .with_occurrences(Some(1 << 16));
    let mut document = serde_json::Deserializer::from_slice(data);
    let Ok(calculus) = wire::upgrade::<Calculus, _>(&mut document, &limits) else {
        return;
    };
    let written = serde_json::to_vec(&calculus).expect("a calculus writes");
    let mut again = serde_json::Deserializer::from_slice(&written);
    let read = wire::upgrade::<Calculus, _>(&mut again, &limits).expect("a written calculus reads");
    assert_eq!(
        serde_json::to_vec(&read).expect("a calculus writes"),
        written
    );
});
