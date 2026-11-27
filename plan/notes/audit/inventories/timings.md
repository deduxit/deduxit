# Compile and test times

Cores [13, 14, 15], `CARGO_BUILD_JOBS=3`, each build from an empty target directory but the second.

| what | command | wall s | CPU s | status |
|---|---|---:|---:|---:|
| the workspace, dev profile | `cargo build --workspace --locked --timings` | 87 | 203 | 0 |
| again after the core crate's root changed | `cargo build --workspace --locked` | 11 | 16 | 0 |
| the tests built | `cargo test --workspace --locked --no-run` | 120 | 323 | 0 |
| the tests run | `cargo test --workspace --locked` | 202 | 385 | 0 |
| the command, release profile | `cargo build --release --locked --package deduxit-cli --timings` | 302 | 471 | 0 |
| the core crate alone, dev profile, default features | `cargo build --locked --package deduxit` | 40 | 63 | 0 |
| the core crate alone, dev profile, no default features | `cargo build --locked --package deduxit --no-default-features` | 16 | 24 | 0 |

## The units that took longest: the workspace, dev profile

| unit | target | s |
|---|---|---:|
| deduxit 0.8.0 | lib | 50.3 |
| deduxit-cli 0.8.0 | lib | 17.7 |
| write-fonts 0.48.1 | lib | 12.2 |
| read-fonts 0.39.2 | lib | 8.9 |
| deduxit-bench 0.8.0 | deduxit-bench "bin" | 7.7 |
| deduxit-web 0.8.0 | lib | 7.5 |
| krilla 0.8.2 | lib | 4.8 |
| skrifa 0.42.1 | lib | 3.9 |
| syn 3.0.10 | lib | 3.7 |
| wasm-bindgen-macro-support 0.2.130 | lib | 3.5 |
| clap_builder 4.6.10 | lib | 3.2 |
| usvg 0.47.0 | lib | 3.0 |

## The units that took longest: the command, release profile

| unit | target | s |
|---|---|---:|
| deduxit 0.8.0 | lib | 129.3 |
| deduxit-cli 0.8.0 | deduxit "bin" | 126.6 |
| deduxit-cli 0.8.0 | lib | 81.1 |
| write-fonts 0.48.1 | lib | 24.9 |
| read-fonts 0.39.2 | lib | 12.3 |
| krilla 0.8.2 | lib | 10.2 |
| skrifa 0.42.1 | lib | 7.7 |
| usvg 0.47.0 | lib | 6.2 |
| clap_builder 4.6.10 | lib | 5.3 |
| rustybuzz 0.20.1 | lib | 4.7 |
| syn 3.0.10 | lib | 3.6 |
| ttf-parser 0.25.1 | lib | 2.9 |

The second row is the best case of a rebuild: the core crate's root got a new
modification time and no new content, so the incremental cache served all of it.
