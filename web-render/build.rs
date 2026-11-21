// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Gives the renderers' module the stack of the bindings' module, so that
//! a render runs as it did there. rustc links one of 1 MiB for
//! `wasm32-unknown-unknown`.

/// The stack of the module, in bytes: the bindings' (`web/build.rs`).
const STACK_BYTES: usize = 8 << 20;

/// Links a wasm32 build with the stack.
fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32") {
        println!("cargo::rustc-link-arg=-zstack-size={STACK_BYTES}");
    }
}
