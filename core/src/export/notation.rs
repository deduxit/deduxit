// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The walk over a derivation that every format shares, which hands the
//! writers one inference at a time. The printer of formulas and sequents
//! is `sequents::notation`, whose table each format fills.

use crate::Error;
use crate::linear::proofs::InfId;
use crate::linear::proofs::style::Drawn;
use std::fmt::Write;

/// One step of the walk over a derivation.
#[derive(Clone, Copy)]
pub(crate) enum Step {
    /// An inference is reached, before its premises; the root is at depth 0.
    Enter(InfId, usize),
    /// An inference is left, after its premises.
    #[cfg_attr(
        not(any(feature = "typst", feature = "svg")),
        expect(
            dead_code,
            reason = "only the layouts of their own read the depth on the way up"
        )
    )]
    Exit(InfId, usize),
}

/// Walks a derivation depth-first from the root, premises in their order.
///
/// It hands `visit` each inference twice: on entering it, before its
/// premises, and on leaving it, after them. It stops at the first error
/// `visit` returns. The exits alone come in postfix order. The walk keeps
/// its own stack, so a derivation of any height fits.
pub(crate) fn walk<T: Drawn, E>(
    derivation: &T,
    mut visit: impl FnMut(Step) -> Result<(), E>,
) -> Result<(), E> {
    let mut stack = vec![Step::Enter(derivation.root(), 0)];
    while let Some(step) = stack.pop() {
        visit(step)?;
        if let Step::Enter(id, depth) = step {
            stack.push(Step::Exit(id, depth));
            for &p in derivation.premises(id).iter().rev() {
                stack.push(Step::Enter(p, depth + 1));
            }
        }
    }
    Ok(())
}

/// Writes what `buffer` holds to `out`, empties it, then asks `stop` and
/// refuses with `Refusal::Stopped` if it says so.
///
/// The writers make one inference at a time in a buffer and hand it on
/// here, so they hold one inference's text and can stop between two.
pub(crate) fn flush(
    out: &mut impl Write,
    buffer: &mut String,
    stop: &mut impl FnMut() -> bool,
) -> Result<(), Error> {
    out.write_str(buffer)?;
    buffer.clear();
    if stop() {
        Err(Error::Refused(crate::limits::Refusal::Stopped {
            phase: crate::limits::Phase::Write,
        }))
    } else {
        Ok(())
    }
}
