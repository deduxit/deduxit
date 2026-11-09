// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The `deduxit` binary: one call into the library of the same package.

/// The allocator of a static build for musl, whose own allocator made the
/// search up to three and a half times slower with four threads, and
/// slower on one.
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Runs the command line interface.
fn main() -> std::process::ExitCode {
    deduxit_cli::main()
}
