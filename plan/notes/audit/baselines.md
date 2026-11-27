# The audit's baselines (step 86)

The numbers the audit and the refactor compare against, taken on
2026-11-27 at main after step 85's library part (`vqwqlqkl`, "Plan:
step 84 reviewed, 85's docs fixed forward"): version 0.8.0 with steps 82
to 85. Toolchain: rustc, cargo and Clippy 1.100.0; cargo-public-api
0.52.0; valgrind 3.27.1; Rocq 9.1.1. Everything ran on cores 13 to 15 in
a scope capped at 8 GiB, the target sets on cores 2 and 3.

Phase A took the cheap ones once before, at `urvzlzxl` (0.8.0 before
steps 82 to 85); where a number moved, the earlier one is beside it.
Each baseline names the command that takes it again: `plan/audit/stage0.sh
PART` writes into `target/audit/`, and the tables and lists it wrote at
this commit are `inventories/` beside this file. A refactor step
compares against these numbers; the release compares the whole.

| baseline | where |
|---|---|
| the focused and the net target set | below; `bench/targets/86.csv`, `net-86.csv` |
| the ratchet's instruction counts and misses | below; `inventories/ratchet.out` |
| compile and test times | below; `inventories/timings.md` |
| the wasm modules' bytes and the lock's packages | below |
| the public API's listing and counts | below; `inventories/surface.md` |
| Clippy, the allowed lints counted | below; `inventories/clippy.md` |
| duplicate code | below; `inventories/duplicates.md` |
| which module names which | `inventories/deps.md` |
| each journey's allocations | below; `inventories/dhat.md` |
| the functions with the most LLVM IR | `inventories/llvm-lines.txt` |
| the command's help and flags | below; `inventories/help.md` |
| the Rocq library's build, warnings and assumptions | below; `inventories/rocq.md` |
| line counts per module | below; `inventories/lines.md`, `units.md` |
| the engines' times | the fifth benchmark baseline, `bench/results/2026-11-27/` |

## Again at the refactor's base (step 87a)

fixup-31 changed sources after `vqwqlqkl` (the Kripke model's check of
`=`, first-order resolution's bounded terms, ordinary logic's protocol),
so step 87a took the measured baselines again at `kmxoxyxv`, main with
fixup-31, on 2026-11-27. The sections below keep the numbers of
`vqwqlqkl`; where a measurement was taken again, the inventory beside
this file is the new one.

| baseline | at `kmxoxyxv` | against `vqwqlqkl` |
|---|---|---|
| the target sets (`bench/targets/87a-base.csv`, `net-87a-base.csv`) | 145 rows decided alike, 0 counters apart; 64 and 0 | equal to `86.csv` and `net-86.csv` |
| the ratchet, the first twenty journeys (`inventories/ratchet.out`, now with all 42 at the step's head) | `check-qbf-20-2` 138 825 952 (+0.52 %), `ordinary-pigeons` 32 133 670 (+0.04 %), every other count within 0.2 % | the shell's build; the sandbox's counts `render-latex` +2.10 % and `render-svg` +2.02 % over their ceilings, which fails the flake's `ratchet` check (87a raised both) |
| the web module, raw (`inventories/wasm.md`) | 8 400 items, 11 220 210 bytes | +4 074 bytes, `ordinary` +3 800 |
| LLVM lines (`inventories/llvm-lines.txt`) | 3 345 135 lines, 68 842 copies | +1 634 lines, +34 copies |
| allocations (`inventories/dhat.md`) | all 42 journeys of step 87a's head, the first twenty as before | unchanged on the first twenty |
| compile and test times (`inventories/timings.md`) | on cores 7 to 9 (below) | not comparable: other cores |

Compile and test times on cores 7 to 9, three jobs, nothing else on
them (`plan/audit/stage0.sh timings` with `AUDIT_CORES=7-9`, at the
step's head, whose changes are in `bench/` alone); they compare with
later runs on those cores only:

| what | wall s | CPU s |
|---|---:|---:|
| the workspace, dev profile | 68 | 162 |
| again after the core crate's root changed | 8 | 14 |
| the tests built | 96 | 248 |
| the tests run | 179 | 346 |
| the command, release profile | 265 | 405 |
| the core crate alone, dev profile, default features | 32 | 50 |
| the core crate alone, dev profile, no default features | 13 | 20 |

Each journey's heap blocks inside its measured part are now the
ratchet's second count (`bench/ceilings.csv`, `journey,instructions,
blocks`), which a refactor step compares instead of DHAT's totals:
DHAT profiles the whole process, the input's generation included.

## Performance: the fifth benchmark baseline

`bench/results/2026-11-27/` (finished 03:42 on 2026-11-27) is the timing
reference for every engine before the refactor. It ran beside the step
sessions, so its times are compared with the fourth baseline,
`bench/results/2026-11-02/`, before a difference is read as the code's.

## The target sets

`bench/targets.sh 86` and `bench/net-targets.sh 86`, compared with the
oracles by `deduxit-bench summary --against bench/targets/47.csv
bench/targets/86.csv --counters` (and `net-47.csv`, `net-86.csv`):

| set | rows both decide alike | verdicts that differ | counters that differ |
|---|---:|---:|---:|
| focused, against `47.csv` | 145 | 0 | 0 |
| net, against `net-47.csv` | 64 | 0 | 0 |

So the searches are, at the refactor's base, the ones the oracles
recorded: a refactor step that moves a counter of `86.csv` or
`net-86.csv` changed a search.

## The ratchet's counts

`cargo run --release --locked -p deduxit-bench -- ratchet --jobs 3`:
each journey's instructions under callgrind against `bench/ceilings.csv`
and the cache model's misses against `bench/misses.csv`. Counts are a
function of the code and the toolchain, not of the machine's load. The
check fails past 2 % over a ceiling (`ratchet::TOLERANCE`); every journey
is within it, sixteen of twenty above their ceiling by up to 1.6 %.
Steps 82 to 85 moved four counts, by at most 0.011 %.

| journey | instructions | ceiling | change | L1 misses | change | LL misses | change |
|---|--:|--:|--:|--:|--:|--:|--:|
| search-qbf-20-2 | 486191402 | 481419142 | +0.99 % | 3511162 | +2.22 % | 215976 | -0.16 % |
| search-chain-128 | 961525087 | 959285073 | +0.23 % | 1060114 | +7.43 % | 8232 | -2.48 % |
| search-partition-no-5 | 7082067 | 7028620 | +0.76 % | 3906 | +5.91 % | 1474 | +2.22 % |
| search-wide-m2-256 | 261407106 | 260393363 | +0.39 % | 405235 | -1.76 % | 4800 | +5.96 % |
| search-spec-chain | 23135928 | 22770649 | +1.60 % | 144969 | +0.87 % | 4678 | +5.03 % |
| search-chain-64-intuitionistic | 122103215 | 121737110 | +0.30 % | 155664 | +3.21 % | 3582 | -8.48 % |
| search-additive-14 | 583612286 | 582045966 | +0.27 % | 6540910 | +0.00 % | 1940714 | +0.07 % |
| read-text | 12590651 | 12591991 | -0.01 % | 100579 | -3.67 % | 6767 | -0.84 % |
| read-json | 12286883 | 12400183 | -0.91 % | 38723 | -11.18 % | 261 | +33.16 % |
| read-lltp | 28712014 | 28412648 | +1.05 % | 131424 | +2.55 % | 37942 | -0.98 % |
| read-tptp | 34192738 | 33812683 | +1.12 % | 76396 | +6.56 % | 22894 | +5.19 % |
| read-spec | 18420712 | 18341178 | +0.43 % | 114875 | +0.80 % | 36869 | -1.42 % |
| check-qbf-20-2 | 138102545 | 136738432 | +1.00 % | 214121 | -8.69 % | 9569 | +0.00 % |
| check-wide-m1-2048 | 8627575 | 8575388 | +0.61 % | 21125 | +0.75 % | 177 | +5.99 % |
| derivation-chain-64 | 9938826 | 9933453 | +0.05 % | 14744 | +5.08 % | 8673 | +3.26 % |
| render-latex | 5735317 | 5675903 | +1.05 % | 5575 | +2.14 % | 3883 | +4.72 % |
| render-typst | 4722103 | 4726140 | -0.09 % | 6273 | +2.60 % | 4447 | +4.12 % |
| render-svg | 79169276 | 78295779 | +1.12 % | 113895 | +5.45 % | 35012 | +3.28 % |
| batch-families | 52708940 | 52333250 | +0.72 % | 69065 | +6.88 % | 11550 | -0.80 % |
| ordinary-pigeons | 32119830 | 32394319 | -0.85 % | 149796 | +2.71 % | 29030 | +9.06 % |

The ratchet's own note: misses moved by more than 5 % and 10 000 on
`search-chain-128` and `check-qbf-20-2`, a question for pinned time, not
a failure.

No journey runs modal logic, lambda-terms or a calculus given as a rule
file, and two run ordinary logic (`ordinary-pigeons`, and `read-tptp`
for its reader): a refactor of those suites has the tests and the
examples as its net, not an instruction count.

## Compile and test times

`plan/audit/stage0.sh timings` (`timings.py`): each build from an empty
target directory, on cores 13 to 15 with three jobs, nothing else on
them. Wall times on those cores compare with later runs on the same
cores only.

| what | command | wall s | CPU s |
|---|---|---:|---:|
| the workspace, dev profile | `cargo build --workspace --locked` | 87 | 203 |
| the tests built | `cargo test --workspace --locked --no-run` | 120 | 323 |
| the tests run | `cargo test --workspace --locked` | 202 | 385 |
| the command, release profile | `cargo build --release --locked --package deduxit-cli` | 302 | 471 |
| the core crate alone, dev profile, default features | `cargo build --locked --package deduxit` | 40 | 63 |
| the core crate alone, dev profile, no default features | `cargo build --locked --package deduxit --no-default-features` | 16 | 24 |

The core crate is one compilation unit and the longest of both builds:
50 s of the dev build's 87 s, 129 s in the release profile, where the
command's binary takes 127 s more and its library 81 s
(`inventories/timings.md`). A rebuild after the core crate's root got a
new modification time and no new content took 11 s; that is the best
case, not what an edit costs.

## The wasm modules and the lock

The `budget` check (`modules/budget.nix`), whose comments record the
module's bytes at each step; `Cargo.lock`'s packages counted by `grep -c
'^name = ' Cargo.lock`. Not rebuilt here: the check ran on CI for this
commit.

| what | now | at 0.8.0 (phase A) | ceiling | room |
|---|---:|---:|---:|---:|
| packages in `Cargo.lock` | 175 | 175 | 180 | 5 |
| `deduxit_web_bg.wasm` at `gzip -9`, bytes | 2 987 284 | 2 885 650 | 3 020 000 | 32 716 |
| the renderers' module at `gzip -9`, bytes | 1 647 274 (phase A; not measured again) | 1 647 274 | 1 700 000 | 52 726 |

The web module grew by 101 634 bytes (3.5 %) in steps 82 to 85, and its
ceiling was raised three times in them. Every visitor of the page
downloads it; whether suites could load as modules of their own, as the
renderers do, is a question of the audit.

## Dependencies

`cargo tree --manifest-path core/Cargo.toml -e normal --prefix none`
prints 27 lines of dependencies with the default features and 14 with
`--no-default-features` (phase A; the manifest did not change since).

## The public API

`plan/audit/stage0.sh api`: the rustdoc JSON as `breaks` builds it
(`RUSTC_BOOTSTRAP=1 cargo rustdoc --manifest-path core/Cargo.toml
--all-features --lib -- -Z unstable-options --output-format json`),
listed by `cargo public-api -sss diff` against the JSON of an empty
crate named `deduxit` (cargo-public-api 0.52 has no option that lists
one JSON file), then read by `surface.py`. The listing is not committed
(550 KB).

| what | now | at 0.8.0 (phase A) |
|---|---:|---:|
| lines of the listing | 6 529 | 6 442 |
| after removing repeated lines | 5 211 | 5 128 |
| lines with `#[non_exhaustive]` | 610 | 600 |
| public field or variant | 2 516 | 2 472 |
| `pub fn` | 1 091 | 1 079 |
| `pub const fn` | 294 | 287 |
| `impl` lines | 504 | 498 |
| `pub struct` | 317 | 310 |
| `pub const` | 218 | 214 |
| `pub enum` | 178 | 176 |
| `pub mod` | 54 | 54 |
| `pub type` | 34 | 33 |
| `pub trait` | 4 | 4 |

An item reachable under several paths is listed once per path, so the
listing overstates the distinct items. From the rustdoc JSON
(`inventories/surface.md`):
- 531 distinct items at module level (types, traits, functions,
  constants), 93 of them reachable under several paths;
- 82 that nothing outside `core/src` names (the other crates, the
  tests, the manual, the guide), counted by identifier: candidates for
  `pub(crate)`;
- docs without an example: 210 of 242 structs, 131 of 133 enums, 84 of
  104 functions, all 4 traits, 963 of 985 inherent methods.

By the module an item's path starts with (lines but `impl` lines):
`ordinary` 918, `linear::proofs` 591, `linear` (re-exports at the
suite's root) 552, `export` 337, `linear::search` 322, `lambda` 238
(177 at 0.8.0), `linear::occurrences` 226, `rules` 209 (196),
`linear::sequents` 177, `limits` 146, `linear::nets` 143, `modal` 129,
`protocol` 92, `linear::fragment` 78, `workspace` 71, `settings` 37,
`linear::families` 27, `kripke` 20, `linear::lltp` and `wire` 10 each,
`linear::mist` 8.

## Clippy: the lints the workspace allows

`plan/audit/stage0.sh clippy`: `cargo clippy --workspace --all-targets
--locked --message-format=json -- -W LINT …` with every lint that
`Cargo.toml` allows turned to a warning, counted once per place and
tabled by area in `inventories/clippy.md`.

| what | now | at 0.8.0 (phase A) |
|---|---:|---:|
| sites | 3 184 | 3 108 |
| lints that fire | 53 | 53 |
| sites of the ten lints set aside on purpose | 1 158 | 1 134 |
| sites of the others, which go to `warn` once fixed | 2 026 | 1 974 |
| of those, `cast_possible_truncation` | 965 | 939 |
| functions over Clippy's limit of 100 lines | 216 | 209 |

Set aside on purpose: `must_use_candidate` (556), `too_many_lines`
(216), `similar_names` (94), `many_single_char_names` (88),
`enum_glob_use` (69), `single_match_else` (67),
`items_after_statements` (32), `match_bool` (28), `if_not_else` (5),
`naive_bytecount` (3). The longest functions: 392 lines
(`core/src/linear/proofs/first_order.rs:137`), 389
(`core/src/export/svg/net.rs:374`), 382 (`cli/src/prove.rs:296`), 328
(`core/src/protocol/ordinary.rs:953`), 319
(`core/src/errors/mod.rs:1311`).

## Duplicate code

`plan/audit/stage0.sh duplicates`: PMD's CPD 7.28 over the sources with
comments and literals emptied, test files apart, at 100 tokens or more.

| what | now | at 0.8.0 (phase A) |
|---|---:|---:|
| duplicates | 173 | 165 |
| between different files | 125 | 120 |

The largest are between the two connection provers
(`ordinary/connection/intuitionistic/prove.rs` and
`ordinary/connection/prove.rs`: 751, 621 and 341 tokens), between
`ordinary/kripke_search.rs` and `ordinary/models.rs` (399), and between
the LaTeX and the Typst export (336).

## Allocations

`plan/audit/stage0.sh dhat`: each journey of `bench/ceilings.csv` under
`valgrind --tool=dhat`, summed by `dhat.py`. The journeys that allocate
most often for the work they do:

| journey | bytes allocated | blocks | instructions per block |
|---|---:|---:|---:|
| ordinary-pigeons | 5 956 104 | 60 124 | 449 |
| search-spec-chain | 8 165 203 | 52 685 | 815 |
| read-tptp | 3 355 368 | 18 418 | 1 912 |
| read-spec | 4 790 489 | 12 440 | 1 926 |
| read-json | 4 793 827 | 18 951 | 1 943 |
| search-additive-14 | 104 927 998 | 167 253 | 4 074 |
| check-qbf-20-2 | 39 259 838 | 129 111 | 5 029 |

`inventories/dhat.md` has all twenty with the frames of deduxit that
allocate most. In `check-qbf-20-2`, two frames make 98 % of the blocks:
the rehash of a map from eigenvariables to terms (50 %) and a boxed
`check::zones::State` (48 %).

## The command's help

`plan/audit/stage0.sh help`: 34 commands and subcommands, 80 flags; 16
flags differ between commands in short form, value or heading
(`--calculus`, `--cyclic`, `--depth`, `--file`, `--format`,
`--intuitionistic`, `--lambek`, `--logic`, `--memory-limit`, `--method`,
`--node-limit`, `--output`, `--standalone`, `--steps`, `--system`,
`--to`). At 0.8.0: 32 commands, 79 flags, 14 that differ.

## The Rocq library

`plan/audit/stage0.sh rocq` (`rocq.py`), in a copy, with the Rocq of
the library's devshell (9.1.1), at the library's `main` (`uytplsqq`):
- the 36 files of `_RocqProject` compile in 10 s, one after another;
- with every warning on (`-w all`): 34 warnings
  `warn-transitive-library-file-stdlib-vector` and 5
  `disj-pattern-notation`; `_RocqProject` sets no warning flags;
- with `-mangle-names`, 18 of the 36 files are refused: their proofs
  name a hypothesis or variable Rocq generated;
- `Print Assumptions` of all 275 theorems and lemmas answers "Closed
  under the global context"; `tests/Assumptions.v` asks about 53;
- the exporter's certificates import 15 of the 36 modules.

## Lines

`plan/audit/stage0.sh lines` and `plan/audit/units.py`: `wc` over every
`.rs` file, by module; test files are those under a `tests` directory or
named `tests.rs`, `reference.rs`, `reference_fo.rs`, `generate.rs`,
`oracle.rs` or `…_tests.rs`, so tests inside a source file count as
source.

| what | now | at 0.8.0 (phase A) |
|---|---:|---:|
| `core/src` | 214 915 | 207 688 |
| of which test files | 17 009 | |
| `core/src/linear` | 90 122 | 90 122 |
| `core/src/ordinary` | 56 540 | 55 577 |
| `core/src/lambda` | 13 705 | |
| `core/src/modal` | 6 427 | |
| `core/src/rules` | 9 824 | |
| `cli` (sources and tests) | 28 142 | |
| `web` | 10 362 | |
| Rust files over 1 000 lines | 95 | 94 |
| over 5 000 lines | 2 | 2 |
| `rocq-deduxit/theories` | 8 965 | 8 965 |

The two largest files are `ordinary/interactive.rs` (5 900 lines; 5 258
at 0.8.0) and `linear/search/mod.rs` (5 552). The audit's 21 units hold
239 238 lines of source (`inventories/units.md`).

In `core/src`, tests included, at 0.8.0: 920 `.clone()`, 260 `dyn` in a
`Box`, `&dyn` or `&mut dyn`, 114 `#[inline…]`, no `todo!`,
`unimplemented!`, `TODO` or `FIXME` in `core/src`, `cli/src` and
`web/src`. They are sizes of what the audit reads, not findings.
