// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The settings of a browser tab (`deduxit-web-tab`, which the renderers'
//! module reads too), and the stack this module is linked with.

pub use deduxit_web_tab::*;

/// The stack the module is linked with, in bytes, as `build.rs` sets it.
pub const STACK_BYTES: usize = match usize::from_str_radix(env!("DEDUXIT_WEB_STACK_BYTES"), 10) {
    Ok(bytes) => bytes,
    Err(_) => panic!("build.rs writes the stack's size in decimal"),
};
