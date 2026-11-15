// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A derivation in a calculus given as data, as `check` reads it: no input
//! panics the reader or the checker, and the checker keeps to its memory
//! and work bounds. The derivation is checked in the built-in calculus it
//! names, or in LK, which refuses it by name.
#![no_main]

use deduxit::rules::{self, Calculus, Derivation};
use deduxit::{Limits, wire};
use libfuzzer_sys::fuzz_target;
use std::sync::LazyLock;

/// The built-in calculi, read once.
static CALCULI: LazyLock<Vec<Calculus>> = LazyLock::new(|| {
    [rules::LK, rules::LJ, rules::LM, rules::LL, rules::TABLEAU]
        .iter()
        .map(|text| {
            let mut document = serde_json::Deserializer::from_str(text);
            wire::upgrade(&mut document, &Limits::default()).expect("a built-in calculus reads")
        })
        .collect()
});

fuzz_target!(|data: &[u8]| {
    let limits = Limits::default()
        .with_memory_bytes(Some(1 << 26))
        .with_occurrences(Some(1 << 16))
        .with_work(Some(1 << 20));
    let mut document = serde_json::Deserializer::from_slice(data);
    let Ok(derivation) = wire::upgrade::<Derivation, _>(&mut document, &limits) else {
        return;
    };
    let calculus = CALCULI
        .iter()
        .find(|c| c.name() == derivation.calculus())
        .unwrap_or(&CALCULI[0]);
    let _ = derivation.check(calculus, &limits, |_| false);
    if let Some(root) = derivation.root()
        && derivation.zones() == calculus.zones().len()
    {
        let _ = calculus
            .show_sequent(&derivation, root.id())
            .limit(1 << 12)
            .to_string();
    }
});
