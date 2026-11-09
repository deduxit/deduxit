// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The `linlog` binary of the last release under that name: the command
//! `deduxit`, which it runs after saying so once on standard error.

/// Says that the command is now `deduxit`, then runs it.
fn main() -> std::process::ExitCode {
    eprintln!(
        "linlog is now deduxit: this runs the command deduxit (cargo install --locked deduxit-cli)"
    );
    deduxit_cli::main()
}
