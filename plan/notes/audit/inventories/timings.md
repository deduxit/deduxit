# Compile and test times

Cores [7, 8, 9], `CARGO_BUILD_JOBS=3`, each build from an empty target directory but the second.

| what | command | wall s | CPU s | status |
|---|---|---:|---:|---:|
| the workspace, dev profile | `cargo build --workspace --locked --timings` | 68 | 162 | 0 |
| again after the core crate's root changed | `cargo build --workspace --locked` | 8 | 14 | 0 |
| the tests built | `cargo test --workspace --locked --no-run` | 96 | 248 | 0 |
| the tests run | `cargo test --workspace --locked` | 179 | 346 | 0 |
| the command, release profile | `cargo build --release --locked --package deduxit-cli --timings` | 265 | 405 | 0 |
| the core crate alone, dev profile, default features | `cargo build --locked --package deduxit` | 32 | 50 | 0 |
| the core crate alone, dev profile, no default features | `cargo build --locked --package deduxit --no-default-features` | 13 | 20 | 0 |

## The units that took longest: the workspace, dev profile

| unit | target | s |
|---|---|---:|
| deduxit |  | 39.0 |
| deduxit-cli |  | 13.7 |
| write-fonts |  | 10.2 |
| deduxit-bench | deduxit-bench "bin" | 6.9 |
| read-fonts |  | 6.4 |
| deduxit-web |  | 6.0 |
| krilla |  | 3.9 |
| skrifa |  | 3.4 |
| syn |  | 3.0 |
| wasm-bindgen-macro-support |  | 2.7 |
| usvg |  | 2.6 |
| clap_builder |  | 2.5 |

## The units that took longest: the command, release profile

| unit | target | s |
|---|---|---:|
| deduxit-cli | deduxit "bin" | 116.5 |
| deduxit |  | 112.1 |
| deduxit-cli |  | 67.6 |
| write-fonts |  | 19.9 |
| read-fonts |  | 9.9 |
| krilla |  | 8.2 |
| skrifa |  | 6.3 |
| usvg |  | 4.9 |
| clap_builder |  | 4.4 |
| rustybuzz |  | 4.0 |
| syn |  | 2.5 |
| resvg |  | 2.5 |
