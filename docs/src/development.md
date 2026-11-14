# Development

This chapter is for working on deduxit itself: getting the tools,
building, testing, and running the checks a change must pass.

## Get the tools

The flake's development shell has the Rust toolchain and every tool the
checks use. Enter it with `nix develop`, or let direnv load it. `menu`
lists its commands:

| command | what it does |
|---|---|
| `launch ARGS` | builds and runs the command with your arguments |
| `tests` | runs the workspace's tests |
| `gate` | runs what every change that touches code must pass |
| `ratchet` | counts the instructions of fixed workloads against their ceilings |
| `check` | runs every flake check, as CI does |
| `manual` | draws the manual's figures and serves it at `localhost:3000` |
| `live` | checks again on every save, with bacon |

## Build and run

```sh
cargo build
cargo run -p deduxit-cli -- prove "A |- A"
nix build            # the command, as result/bin/deduxit
nix build .#manual   # the manual alone
```

## Test

```sh
cargo test --workspace
cargo test -p deduxit TEST_NAME        # one test of the library
cargo clippy --workspace --all-targets -- --deny warnings
```

`cargo test --workspace` also runs every example of README, the manual
and the guide for AI models, and compares the output with the text. A
change of what the command prints therefore changes the chapter that
shows it.

A change behind a feature, or of `[features]`, also needs both feature
checks, which build the library with each feature alone and with every
pair:

```sh
cargo hack check --each-feature -p deduxit
cargo hack check --feature-powerset --depth 2 -p deduxit
```

`cargo test -p deduxit` alone leaves the feature `parallel` off, and
`cargo test -p deduxit --features parallel` tests the parallel search
too. The web bindings' tests run under Node:

```sh
nix build .#checks.x86_64-linux.web-wasm-tests
```

## Before a commit

`gate` runs clippy, the tests, both feature checks and the ratchet, in
about four minutes on four cores. Every change that touches code passes
it. The ratchet fails when a workload's instruction count rises more than
2 % above its ceiling in `bench/ceilings.csv`. After a count went down,
the second command below writes the new ceiling:

```sh
gate
ratchet --lower
```

Format before committing. Every source file starts with the licence
header, as the existing files show.

```sh
nix fmt
```

## The full check

`nix flake check` runs what CI runs, except the online
`cargo deny check advisories`. Run it after a change of the flake, the
toolchain or a lock file, and before a push. It builds every crate and
the manual, runs the tests with and without debug assertions, compiles
the exports' LaTeX and Typst, and checks the Rocq certificates.

CI uploads what `main` builds to the public binary cache
`linlog-prover` on Cachix. With the cache, a local check fetches
everything a change does not touch instead of building it again. Add it
once, as a trusted user of the Nix daemon (otherwise `cachix` says what
to add to the system's configuration):

```sh
nix run nixpkgs#cachix -- use linlog-prover
```

Or add `https://linlog-prover.cachix.org` to `extra-substituters` and
`linlog-prover.cachix.org-1:6K+DDVwwELnSYtfflwUdsHdYuolC7hprdD/r2/KOQZU=`
to `extra-trusted-public-keys` in `nix.conf`.

## Benchmarks

Timings mean something only on an idle machine.
[Benchmarks and the comparison](benchmarks.md) says how to run the
harness and read its results.
