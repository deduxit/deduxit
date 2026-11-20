// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A type's terms found and counted, under small bounds: the first byte
//! picks the depth and the number of terms listed, the rest is an
//! ordinary sequent whose formula right of `⊢` is the type. Nothing may
//! panic, an error of kind `defect` is a find, a term found checks
//! against its own judgement, and an exact count lists all its terms up to
//! the number asked for. That the judgement is the sequent's is the
//! library's own assertion in a debug build, which `fuzz/run.sh` makes.
#![no_main]

use deduxit::lambda::{Count, InhabitOptions, Inhabitation, enumerate, inhabit};
use deduxit::{ErrorKind, Limits, ordinary};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some((&first, rest)) = data.split_first() else {
        return;
    };
    let Ok(text) = std::str::from_utf8(rest) else {
        return;
    };
    let limits = Limits::default()
        .with_occurrences(Some(256))
        .with_memory_bytes(Some(1 << 26))
        .with_work(Some(20_000));
    let Ok(sequent) = ordinary::Sequent::parse_within(text, &limits) else {
        return;
    };
    match inhabit(&sequent, &limits, |_| false) {
        Ok(Inhabitation::Inhabited(term)) => {
            term.check(&limits, |_| false)
                .expect("an inhabitant checks");
        }
        Ok(_) => {}
        Err(error) => assert_ne!(error.kind(), ErrorKind::Defect, "{error}"),
    }
    let options = InhabitOptions::default()
        .with_depth(u32::from(first & 7))
        .with_max_terms(u32::from(first >> 3));
    match enumerate(&sequent, &options, &limits, |_| false) {
        Ok(found) => {
            for term in &found.terms {
                term.check(&limits, |_| false)
                    .expect("a term listed checks");
            }
            if let Count::Exactly { count, .. } = found.count {
                let listed = count.min(u64::from(options.max_terms));
                assert_eq!(found.terms.len() as u64, listed);
            }
        }
        Err(error) => assert_ne!(error.kind(), ErrorKind::Defect, "{error}"),
    }
});
