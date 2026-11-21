// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The settings of a browser tab (`deduxit-web-tab`, which the renderers'
//! module reads too), and the stack this module is linked with.

pub use deduxit_web_tab::*;

/// The stack the module is linked with, in bytes, as `build.rs` sets it.
pub const STACK_BYTES: usize = parse(env!("DEDUXIT_WEB_STACK_BYTES"));

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
