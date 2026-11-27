# The audit's baselines (step 86, phase A)

The numbers the audit and the refactor compare against, taken on
2026-11-27 at main as the audit's workspace had it (`urvzlzxl`, "Plan:
step 86, the audit and refactor for 0.9.0"), version 0.8.0 plus what
landed since. Toolchain: rustc, cargo and Clippy 1.100.0; cargo-public-api
0.52.0. Everything ran on cores 13 to 15 in a scope capped at 8 GiB.

Steps 82 to 85 change the code after this commit. **Every measured
baseline here is taken again at the commit phase D starts from**, by the
commands given with each; the two the planning session deferred are
taken only then.

| baseline | state |
|---|---|
| line counts per module | taken |
| the public API's listing and counts | taken |
| Clippy, the allowed lints counted | taken |
| the ratchet's instruction counts and misses | taken |
| the wasm modules' bytes and the lock's packages | taken |
| dependencies | taken |
| the engines' times | the fifth benchmark baseline, `bench/results/2026-11-27/` |
| compile and test times | deferred to phase D's base (the planning session, 2026-11-27: wall times beside two step sessions and a benchmark are not worth comparing) |
| the focused and net target sets | deferred to phase D's base (the same answer: the benchmark baseline held cores 0 to 3 that night) |

## Performance: the fifth benchmark baseline

`bench/results/2026-11-27/` (finished 03:42 on 2026-11-27) is the timing
reference for every engine before the refactor. It ran beside the step
sessions, so its times are compared with the fourth baseline,
`bench/results/2026-11-02/`, before a difference is read as the code's.

## The ratchet's counts

`cargo run --release --locked -p deduxit-bench -- ratchet --jobs 3`:
each journey's instructions under callgrind against `bench/ceilings.csv`
and the cache model's misses against `bench/misses.csv`. Counts are a
function of the code and the toolchain, not of the machine's load. The
check fails past 2 % over a ceiling (`ratchet::TOLERANCE`); every journey
is within it, sixteen of twenty above their ceiling by up to 1.6 %.

| journey | instructions | ceiling | change | L1 misses | change | LL misses | change |
|---|--:|--:|--:|--:|--:|--:|--:|
| search-qbf-20-2 | 486191402 | 481419142 | +0.99 % | 3526651 | +2.67 % | 215962 | -0.17 % |
| search-chain-128 | 961525087 | 959285073 | +0.23 % | 1146774 | +16.21 % | 8225 | -2.56 % |
| search-partition-no-5 | 7082067 | 7028620 | +0.76 % | 3913 | +6.10 % | 1467 | +1.73 % |
| search-wide-m2-256 | 261407106 | 260393363 | +0.39 % | 406006 | -1.57 % | 4794 | +5.83 % |
| search-spec-chain | 23135916 | 22770649 | +1.60 % | 145042 | +0.92 % | 4684 | +5.16 % |
| search-chain-64-intuitionistic | 122103215 | 121737110 | +0.30 % | 155966 | +3.41 % | 3576 | -8.64 % |
| search-additive-14 | 583612298 | 582045966 | +0.27 % | 6536175 | -0.07 % | 1940503 | +0.06 % |
| read-text | 12590651 | 12591991 | -0.01 % | 100600 | -3.65 % | 6767 | -0.84 % |
| read-json | 12286883 | 12400183 | -0.91 % | 38712 | -11.21 % | 260 | +32.65 % |
| read-lltp | 28712014 | 28412648 | +1.05 % | 131435 | +2.56 % | 37937 | -1.00 % |
| read-tptp | 34192738 | 33812683 | +1.12 % | 76450 | +6.64 % | 22885 | +5.15 % |
| read-spec | 18418750 | 18341178 | +0.42 % | 114843 | +0.77 % | 36865 | -1.43 % |
| check-qbf-20-2 | 138102545 | 136738432 | +1.00 % | 216648 | -7.61 % | 9572 | +0.03 % |
| check-wide-m1-2048 | 8627575 | 8575388 | +0.61 % | 21126 | +0.75 % | 181 | +8.38 % |
| derivation-chain-64 | 9938826 | 9933453 | +0.05 % | 14756 | +5.17 % | 8676 | +3.30 % |
| render-latex | 5735317 | 5675903 | +1.05 % | 5566 | +1.98 % | 3876 | +4.53 % |
| render-typst | 4722103 | 4726140 | -0.09 % | 6318 | +3.34 % | 4452 | +4.24 % |
| render-svg | 79169276 | 78295779 | +1.12 % | 114943 | +6.42 % | 35014 | +3.29 % |
| batch-families | 52708944 | 52333250 | +0.72 % | 69099 | +6.93 % | 11543 | -0.86 % |
| ordinary-pigeons | 32119830 | 32394319 | -0.85 % | 149843 | +2.74 % | 29035 | +9.08 % |

The ratchet's own note: misses moved by more than 5 % and 10 000 on
`search-chain-128` and `check-qbf-20-2`, a question for pinned time, not
a failure.

## The wasm modules and the lock

`nix build .#checks.x86_64-linux.budget` (under the machine lock):

| what | now | ceiling (`modules/budget.nix`) | room |
|---|---:|---:|---:|
| packages in `Cargo.lock` | 175 | 180 | 5 |
| `deduxit_web_bg.wasm` at `gzip -9`, bytes | 2 885 650 | 2 900 000 | 14 350 |
| the renderers' module at `gzip -9`, bytes | 1 647 274 | 1 700 000 | 52 726 |

The web module has half a per cent of room: an API change that adds a
generic instantiation or a public entry point to the bindings shows
here first.

## Dependencies

`cargo tree --manifest-path core/Cargo.toml -e normal --prefix none`
prints 27 lines of dependencies with the default features and 14 with
`--no-default-features`. A consumer crate with the default features
compiles 19 crates, the core crate among them, in 41 s in the dev
profile on three efficiency cores (`consumer-test.md`).

## The public API

Built as `breaks` builds it: `RUSTC_BOOTSTRAP=1 cargo rustdoc
--manifest-path core/Cargo.toml --all-features --lib -- -Z
unstable-options --output-format json`, then `cargo public-api -sss diff
EMPTY.json target/doc/deduxit.json` against the rustdoc JSON of an empty
crate named `deduxit`, which lists every item as added (cargo-public-api
0.52 has no option that lists one JSON file). `-sss` leaves out blanket,
auto-trait and auto-derived impls. The listing is not committed: it is
400 KB and stale after step 85; phase B makes it again.

- 6 442 lines; 5 128 after removing repeated lines. An item reachable
  under several paths is listed once per path (`linear::Derivation`,
  `linear::proofs::Derivation`,
  `linear::proofs::derivation::Derivation`), so the count overstates the
  number of distinct items; phase B's inventory counts those.
- 600 lines carry `#[non_exhaustive]`.

| kind | lines |
|---|---:|
| public field or variant | 2 472 |
| `pub fn` | 1 079 |
| `pub const fn` | 287 |
| `impl` lines (inherent blocks and hand-written trait impls) | 498 |
| `pub struct` | 310 |
| `pub const` | 214 |
| `pub enum` | 176 |
| `pub mod` | 54 |
| `pub type` | 33 |
| `pub trait` | 4 |
| `pub static` | 1 |

| module an item's path starts with | lines but impls |
|---|---:|
| `ordinary` | 917 |
| `linear::proofs` | 591 |
| `linear` (re-exports at the suite's root) | 552 |
| `export` | 337 |
| `linear::search` | 322 |
| `linear::occurrences` | 226 |
| `rules` | 196 |
| `lambda` | 177 |
| `linear::sequents` | 177 |
| `limits` | 145 |
| `linear::nets` | 143 |
| `modal` | 129 |
| `protocol` | 92 |
| `linear::fragment` | 78 |
| `workspace` | 71 |
| `settings` | 37 |
| `linear::families` | 27 |
| `kripke` | 20 |
| `linear::lltp`, `wire` | 10 each |
| `linear::mist` | 8 |

The 54 public modules: `export` with `latex`, `pdf`, `png`, `rocq`,
`svg`, `typst`; `kripke`; `lambda`; `limits`; `linear` with `families`,
`fragment`, `lltp`, `mist`, `nets` (`cut`), `occurrences` (`ill`,
`instances`, `reading`, `written`), `proofs` (`check`, `cut`,
`derivation`, `interactive`, `size`, `style`), `search` (`batch`),
`sequents` (`fmt`, `fo`, `term`); `modal`; `ordinary` with `connection`,
`equality`, `grounding`, `herbrand`, `interpolant`, `ljt`, `models`,
`negative`, `normal`, `resolution` (`first_order`), `sat`; `protocol`;
`rules`; `settings`; `wire`; `workspace`.

## Clippy: the lints the workspace allows

`cargo clippy --workspace --all-targets --locked --message-format=json
-- -W LINT …` with every lint that `Cargo.toml` allows turned to a
warning (the 3 of rustc, the 10 pedantic lints set aside on purpose, the
9 picked ones and the 32 pedantic ones that fire today), counted once
per place. One of the 54 no longer fires. Attributes in the sources: 62
`#[expect(…)]`, 5 `#[allow(…)]`, 10 crate-level `#![deny]`, `#![warn]`
or `#![expect]`.

| lint | sites | linear::search | linear::proofs | linear::nets | linear (rest) | ordinary | modal | lambda | rules | export | protocol | workspace | serialize | core (root) | core tests | cli | bench | web* |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `clippy::cast_possible_truncation` | 939 | 176 | 115 | 84 | 45 | 236 | 38 | 23 | 81 | 49 | 1 |  | 37 | 8 | 10 | 4 | 1 | 31 |
| `clippy::must_use_candidate` | 549 | 27 | 76 | 36 | 128 | 140 | 22 | 19 | 46 | 11 | 4 | 8 |  | 26 |  |  |  | 6 |
| `clippy::too_many_lines` | 209 | 28 | 28 | 7 | 5 | 49 | 5 | 5 | 9 | 7 | 3 | 3 | 5 | 4 | 16 | 25 | 8 | 2 |
| `unused_qualifications` | 207 | 10 | 28 | 9 | 6 | 48 | 6 | 7 | 1 | 3 | 4 | 1 | 7 |  | 20 | 48 | 1 | 8 |
| `clippy::too_long_first_doc_paragraph` | 140 | 3 | 2 | 1 | 5 | 30 | 2 | 1 | 8 | 23 | 2 | 1 |  | 6 |  |  |  | 56 |
| `clippy::doc_markdown` | 101 | 2 |  |  | 2 | 2 | 1 | 2 |  | 22 |  |  |  | 1 | 6 | 57 | 6 |  |
| `clippy::many_single_char_names` | 88 | 19 | 21 | 7 | 3 | 26 | 1 | 4 |  | 3 |  |  | 2 | 1 | 1 |  |  |  |
| `clippy::similar_names` | 88 | 9 | 23 | 5 |  | 13 | 7 | 1 | 1 | 15 |  |  |  |  | 1 | 6 | 1 | 6 |
| `elided_lifetimes_in_paths` | 75 | 7 | 15 |  | 8 |  |  |  |  | 12 |  |  |  |  | 3 | 3 | 27 |  |
| `clippy::enum_glob_use` | 69 | 5 | 29 | 2 | 14 | 7 |  | 1 | 3 | 1 |  |  | 6 | 1 |  |  |  |  |
| `clippy::single_match_else` | 64 | 14 | 5 | 3 | 5 | 12 | 2 | 2 | 5 | 2 | 2 |  | 1 | 1 |  | 6 | 3 | 1 |
| `clippy::or_fun_call` | 50 | 4 | 17 | 1 | 2 | 8 |  |  |  | 4 | 1 |  |  |  |  | 5 | 7 | 1 |
| `clippy::cast_precision_loss` | 43 | 5 |  |  |  |  |  |  |  | 23 |  |  |  |  | 6 | 1 | 8 |  |
| `clippy::redundant_clone` | 39 | 7 | 5 | 1 | 3 | 4 |  | 1 |  |  |  | 1 |  |  | 10 | 4 |  | 3 |
| `clippy::trivially_copy_pass_by_ref` | 38 | 1 | 9 |  |  | 4 |  | 2 |  |  |  |  |  |  |  | 22 |  |  |
| `clippy::match_same_arms` | 36 | 5 | 7 |  | 3 | 10 | 1 | 1 | 4 | 1 |  |  |  |  |  | 3 | 1 |  |
| `clippy::items_after_statements` | 31 | 1 | 3 | 5 | 7 | 7 | 1 | 1 | 1 |  |  |  | 2 |  | 3 |  |  |  |
| `clippy::needless_pass_by_value` | 29 | 6 | 6 | 1 |  | 3 |  | 2 |  | 1 |  | 2 |  |  |  | 3 | 2 | 3 |
| `clippy::match_bool` | 28 |  |  |  |  | 1 |  |  |  | 27 |  |  |  |  |  |  |  |  |
| `clippy::inline_always` | 26 | 15 | 7 |  | 4 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::cast_possible_wrap` | 25 | 12 |  |  |  | 1 |  |  |  | 12 |  |  |  |  |  |  |  |  |
| `clippy::redundant_closure_for_method_calls` | 25 | 6 | 1 | 1 | 2 | 5 |  |  |  |  | 1 |  |  |  | 1 | 7 | 1 |  |
| `clippy::format_push_string` | 25 |  |  |  |  | 2 |  |  |  |  |  |  |  |  | 1 | 14 | 8 |  |
| `clippy::default_trait_access` | 18 |  | 8 | 1 | 5 | 1 |  | 3 |  |  |  |  |  |  |  |  |  |  |
| `clippy::semicolon_if_nothing_returned` | 14 | 2 |  |  | 2 | 3 |  | 1 |  | 2 | 4 |  |  |  |  |  |  |  |
| `clippy::match_wildcard_for_single_variants` | 12 | 11 |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::unnested_or_patterns` | 11 | 1 |  |  | 3 | 4 | 2 |  |  |  |  | 1 |  |  |  |  |  |  |
| `clippy::cast_sign_loss` | 11 | 5 |  |  |  | 3 |  |  |  | 3 |  |  |  |  |  |  |  |  |
| `clippy::struct_excessive_bools` | 11 | 4 |  |  | 2 |  | 1 |  |  |  |  |  |  |  |  | 3 | 1 |  |
| `clippy::unnecessary_wraps` | 11 | 1 |  |  | 1 | 1 |  |  |  |  | 2 |  |  |  |  | 1 | 5 |  |
| `clippy::needless_raw_string_hashes` | 10 |  | 1 |  |  |  |  |  |  |  |  |  |  |  | 3 | 3 | 3 |  |
| `clippy::nonminimal_bool` | 10 | 7 | 1 |  | 2 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::manual_midpoint` | 8 |  | 1 |  |  | 1 |  |  |  | 6 |  |  |  |  |  |  |  |  |
| `clippy::unused_self` | 7 | 1 | 2 |  |  | 2 |  | 1 |  |  |  | 1 |  |  |  |  |  |  |
| `clippy::format_collect` | 7 | 2 |  |  |  | 4 |  |  |  |  |  |  |  |  |  |  | 1 |  |
| `clippy::explicit_iter_loop` | 6 | 1 |  |  |  | 1 |  | 4 |  |  |  |  |  |  |  |  |  |  |
| `single_use_lifetimes` | 5 | 5 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::allow_attributes` | 5 | 4 |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |
| `clippy::allow_attributes_without_reason` | 5 | 4 |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |
| `clippy::if_not_else` | 5 | 1 |  |  | 1 | 2 |  |  |  |  |  |  |  |  |  | 1 |  |  |
| `clippy::assigning_clones` | 5 | 1 | 1 |  | 1 |  |  |  |  |  |  |  |  |  |  | 1 | 1 |  |
| `clippy::ref_option` | 3 |  | 1 |  |  |  |  |  |  |  |  |  | 2 |  |  |  |  |  |
| `clippy::elidable_lifetime_names` | 3 | 3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::inconsistent_struct_constructor` | 3 | 1 |  |  |  | 1 |  |  | 1 |  |  |  |  |  |  |  |  |  |
| `clippy::naive_bytecount` | 3 | 2 |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::branches_sharing_code` | 2 |  |  |  |  | 1 | 1 |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::needless_pass_by_ref_mut` | 2 |  |  | 1 |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::case_sensitive_file_extension_comparisons` | 2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 2 |  |  |
| `clippy::maybe_infinite_iter` | 1 |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::debug_assert_with_mut_call` | 1 |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::checked_conversions` | 1 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::unnecessary_semicolon` | 1 |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |
| `clippy::manual_string_new` | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |

Set aside on purpose, so no debt unless the reason no longer holds:
`must_use_candidate`, `many_single_char_names`, `similar_names`,
`too_many_lines`, `enum_glob_use`, `single_match_else`, `match_bool`,
`if_not_else`, `items_after_statements`, `naive_bytecount` (1 134 of the
sites). The other 1 974 sites are lints that go to `warn` once fixed;
939 of them are `cast_possible_truncation`.

## A few counts by `grep`

In `core/src`, tests included: 920 `.clone()`, 260 `dyn` in a `Box`,
`&dyn` or `&mut dyn`, 114 `#[inline…]` (26 of them `always`, by Clippy's
count), no `todo!`, `unimplemented!`, `TODO` or `FIXME` in `core/src`,
`cli/src` and `web/src`. They are sizes of what phase B reads, not
findings.

## Line counts

`wc` over every `.rs` file, by module; "test files" are files under a
`tests` directory or named `tests.rs`, `reference.rs`,
`reference_fo.rs`, `generate.rs` or `oracle.rs`, so tests inside a
source file count as source.

### core/src, top level

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `errors/` | 4 | 2885 | 0 | `errors/mod.rs` (1820) |
| `export/` | 17 | 13665 | 0 | `export/svg/mod.rs` (2438) |
| `first_order.rs` | 1 | 695 | 0 | `first_order.rs` (695) |
| `goal_tree.rs` | 1 | 173 | 0 | `goal_tree.rs` (173) |
| `hash.rs` | 1 | 23 | 0 | `hash.rs` (23) |
| `kripke.rs` | 1 | 125 | 0 | `kripke.rs` (125) |
| `lambda/` | 10 | 9670 | 1641 | `lambda/tests.rs` (1641) |
| `lib.rs` | 1 | 414 | 0 | `lib.rs` (414) |
| `limits.rs` | 1 | 886 | 0 | `limits.rs` (886) |
| `linear/` | 107 | 90122 | 10469 | `linear/search/mod.rs` (5552) |
| `modal/` | 9 | 6427 | 635 | `modal/search.rs` (1959) |
| `name.rs` | 1 | 150 | 0 | `name.rs` (150) |
| `ordinary/` | 51 | 55577 | 1430 | `ordinary/interactive.rs` (5258) |
| `protocol/` | 8 | 6255 | 909 | `protocol/ordinary.rs` (1939) |
| `rules/` | 12 | 7905 | 0 | `rules/interactive.rs` (2280) |
| `serialize/` | 14 | 7869 | 0 | `serialize/ordinary.rs` (2065) |
| `settings.rs` | 1 | 359 | 0 | `settings.rs` (359) |
| `unify.rs` | 1 | 322 | 0 | `unify.rs` (322) |
| `wire.rs` | 1 | 604 | 0 | `wire.rs` (604) |
| `workspace/` | 3 | 3562 | 861 | `workspace/mod.rs` (1927) |
| **total** | 245 | 207688 | 15945 | |

### core/src/linear

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `families.rs` | 1 | 1292 | 0 | `families.rs` (1292) |
| `fragment.rs` | 1 | 877 | 0 | `fragment.rs` (877) |
| `lltp.rs` | 1 | 566 | 0 | `lltp.rs` (566) |
| `mist.rs` | 1 | 806 | 0 | `mist.rs` (806) |
| `mod.rs` | 1 | 47 | 0 | `mod.rs` (47) |
| `nets/` | 13 | 9170 | 1456 | `nets/mod.rs` (2177) |
| `occurrences/` | 6 | 5084 | 0 | `occurrences/mod.rs` (1101) |
| `parse/` | 1 | 1265 | 0 | `parse/mod.rs` (1265) |
| `proofs/` | 30 | 25316 | 1700 | `proofs/interactive/mod.rs` (2117) |
| `search/` | 47 | 42690 | 7313 | `search/mod.rs` (5552) |
| `sequents/` | 5 | 3009 | 0 | `sequents/notation.rs` (1250) |
| **total** | 107 | 90122 | 10469 | |

### core/src/linear/search

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `additive.rs` | 1 | 514 | 0 | `additive.rs` (514) |
| `basis.rs` | 1 | 595 | 0 | `basis.rs` (595) |
| `batch.rs` | 1 | 987 | 0 | `batch.rs` (987) |
| `classical.rs` | 1 | 302 | 0 | `classical.rs` (302) |
| `essential.rs` | 1 | 945 | 0 | `essential.rs` (945) |
| `focus/` | 19 | 15803 | 2237 | `focus/tests.rs` (2237) |
| `generate.rs` | 1 | 1242 | 1242 | `generate.rs` (1242) |
| `horn/` | 6 | 3949 | 812 | `horn/mod.rs` (866) |
| `inverse/` | 5 | 2736 | 259 | `inverse/mod.rs` (1007) |
| `memory.rs` | 1 | 309 | 0 | `memory.rs` (309) |
| `mod.rs` | 1 | 5552 | 0 | `mod.rs` (5552) |
| `net.rs` | 1 | 1779 | 0 | `net.rs` (1779) |
| `parallel.rs` | 1 | 538 | 0 | `parallel.rs` (538) |
| `planar.rs` | 1 | 626 | 0 | `planar.rs` (626) |
| `reference.rs` | 1 | 1180 | 1180 | `reference.rs` (1180) |
| `reference_fo.rs` | 1 | 1583 | 1583 | `reference_fo.rs` (1583) |
| `refutation.rs` | 1 | 2072 | 0 | `refutation.rs` (2072) |
| `saturated.rs` | 1 | 1342 | 0 | `saturated.rs` (1342) |
| `weighted.rs` | 1 | 501 | 0 | `weighted.rs` (501) |
| `work.rs` | 1 | 135 | 0 | `work.rs` (135) |
| **total** | 47 | 42690 | 7313 | |

### core/src/linear/proofs

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `check/` | 6 | 4401 | 0 | `check/mod.rs` (1425) |
| `cut/` | 3 | 3073 | 869 | `cut/mod.rs` (1525) |
| `derivation/` | 4 | 4121 | 0 | `derivation/classical.rs` (1486) |
| `first_order.rs` | 1 | 1151 | 0 | `first_order.rs` (1151) |
| `fmt.rs` | 1 | 522 | 0 | `fmt.rs` (522) |
| `interactive/` | 9 | 7640 | 0 | `interactive/mod.rs` (2117) |
| `mod.rs` | 1 | 1091 | 0 | `mod.rs` (1091) |
| `multiset.rs` | 1 | 172 | 0 | `multiset.rs` (172) |
| `oracle.rs` | 1 | 831 | 831 | `oracle.rs` (831) |
| `rule.rs` | 1 | 532 | 0 | `rule.rs` (532) |
| `size.rs` | 1 | 1329 | 0 | `size.rs` (1329) |
| `style.rs` | 1 | 453 | 0 | `style.rs` (453) |
| **total** | 30 | 25316 | 1700 | |

### core/src/ordinary

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `bruijn.rs` | 1 | 458 | 0 | `bruijn.rs` (458) |
| `connection/` | 12 | 8206 | 695 | `connection/intuitionistic/read_back.rs` (1476) |
| `decide.rs` | 1 | 1307 | 0 | `decide.rs` (1307) |
| `derivation.rs` | 1 | 2919 | 0 | `derivation.rs` (2919) |
| `equality.rs` | 1 | 505 | 0 | `equality.rs` (505) |
| `grounding.rs` | 1 | 1739 | 0 | `grounding.rs` (1739) |
| `herbrand.rs` | 1 | 2211 | 0 | `herbrand.rs` (2211) |
| `instance.rs` | 1 | 893 | 0 | `instance.rs` (893) |
| `interactive.rs` | 1 | 5258 | 0 | `interactive.rs` (5258) |
| `interpolant.rs` | 1 | 1344 | 0 | `interpolant.rs` (1344) |
| `invert.rs` | 1 | 305 | 0 | `invert.rs` (305) |
| `kripke_search.rs` | 1 | 1068 | 0 | `kripke_search.rs` (1068) |
| `kripke_structure.rs` | 1 | 1274 | 0 | `kripke_structure.rs` (1274) |
| `ljt.rs` | 1 | 1643 | 0 | `ljt.rs` (1643) |
| `mod.rs` | 1 | 2379 | 0 | `mod.rs` (2379) |
| `model.rs` | 1 | 594 | 0 | `model.rs` (594) |
| `models.rs` | 1 | 1429 | 0 | `models.rs` (1429) |
| `negative.rs` | 1 | 733 | 0 | `negative.rs` (733) |
| `normal.rs` | 1 | 3688 | 0 | `normal.rs` (3688) |
| `parse.rs` | 1 | 1411 | 0 | `parse.rs` (1411) |
| `resolution/` | 7 | 5063 | 328 | `resolution/first_order.rs` (2563) |
| `rocq.rs` | 1 | 971 | 0 | `rocq.rs` (971) |
| `rocq_models.rs` | 1 | 552 | 0 | `rocq_models.rs` (552) |
| `sat/` | 5 | 4302 | 0 | `sat/mod.rs` (1477) |
| `structure.rs` | 1 | 845 | 0 | `structure.rs` (845) |
| `table.rs` | 1 | 382 | 0 | `table.rs` (382) |
| `tableau.rs` | 1 | 1109 | 0 | `tableau.rs` (1109) |
| `translate.rs` | 1 | 742 | 0 | `translate.rs` (742) |
| `written/` | 2 | 2247 | 407 | `written/mod.rs` (1840) |
| **total** | 51 | 55577 | 1430 | |

### core/src/export

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `latex.rs` | 1 | 867 | 0 | `latex.rs` (867) |
| `mod.rs` | 1 | 697 | 0 | `mod.rs` (697) |
| `models.rs` | 1 | 306 | 0 | `models.rs` (306) |
| `notation.rs` | 1 | 71 | 0 | `notation.rs` (71) |
| `pdf.rs` | 1 | 344 | 0 | `pdf.rs` (344) |
| `png.rs` | 1 | 225 | 0 | `png.rs` (225) |
| `rocq/` | 2 | 2939 | 0 | `rocq/mod.rs` (1636) |
| `styles.rs` | 1 | 203 | 0 | `styles.rs` (203) |
| `svg/` | 7 | 6755 | 0 | `svg/mod.rs` (2438) |
| `typst.rs` | 1 | 1258 | 0 | `typst.rs` (1258) |
| **total** | 17 | 13665 | 0 | |

### core/tests

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `depth.rs` | 1 | 301 | 0 | `depth.rs` (301) |
| `export.rs` | 1 | 3469 | 0 | `export.rs` (3469) |
| `first_order.rs` | 1 | 1763 | 0 | `first_order.rs` (1763) |
| `iltp.rs` | 1 | 265 | 0 | `iltp.rs` (265) |
| `limits.rs` | 1 | 238 | 0 | `limits.rs` (238) |
| `lock.rs` | 1 | 186 | 0 | `lock.rs` (186) |
| `natural.rs` | 1 | 1376 | 0 | `natural.rs` (1376) |
| `parse.rs` | 1 | 501 | 0 | `parse.rs` (501) |
| `rules.rs` | 1 | 1677 | 0 | `rules.rs` (1677) |
| `serialize.rs` | 1 | 1251 | 0 | `serialize.rs` (1251) |
| `stability.rs` | 1 | 592 | 0 | `stability.rs` (592) |
| **total** | 11 | 11619 | 0 | |

### cli

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `src/argument_parsing.rs` | 1 | 3286 | 0 | `src/argument_parsing.rs` (3286) |
| `src/batch.rs` | 1 | 1618 | 0 | `src/batch.rs` (1618) |
| `src/calculus.rs` | 1 | 607 | 0 | `src/calculus.rs` (607) |
| `src/check.rs` | 1 | 1442 | 0 | `src/check.rs` (1442) |
| `src/eliminate.rs` | 1 | 832 | 0 | `src/eliminate.rs` (832) |
| `src/grade.rs` | 1 | 843 | 0 | `src/grade.rs` (843) |
| `src/hint.rs` | 1 | 235 | 0 | `src/hint.rs` (235) |
| `src/interact/` | 3 | 2413 | 0 | `src/interact/mod.rs` (1290) |
| `src/io.rs` | 1 | 992 | 0 | `src/io.rs` (992) |
| `src/lib.rs` | 1 | 388 | 0 | `src/lib.rs` (388) |
| `src/limit.rs` | 1 | 232 | 0 | `src/limit.rs` (232) |
| `src/main.rs` | 1 | 16 | 0 | `src/main.rs` (16) |
| `src/modal.rs` | 1 | 535 | 0 | `src/modal.rs` (535) |
| `src/ordinary.rs` | 1 | 1420 | 0 | `src/ordinary.rs` (1420) |
| `src/prove.rs` | 1 | 963 | 0 | `src/prove.rs` (963) |
| `src/resolve.rs` | 1 | 611 | 0 | `src/resolve.rs` (611) |
| `src/seq.rs` | 1 | 151 | 0 | `src/seq.rs` (151) |
| `src/settings.rs` | 1 | 1145 | 0 | `src/settings.rs` (1145) |
| `src/show.rs` | 1 | 1733 | 0 | `src/show.rs` (1733) |
| `src/style.rs` | 1 | 113 | 0 | `src/style.rs` (113) |
| `src/tableau.rs` | 1 | 144 | 0 | `src/tableau.rs` (144) |
| `src/term.rs` | 1 | 319 | 0 | `src/term.rs` (319) |
| `src/transform.rs` | 1 | 533 | 0 | `src/transform.rs` (533) |
| `src/workspace.rs` | 1 | 558 | 0 | `src/workspace.rs` (558) |
| `tests/cli.rs` | 1 | 3887 | 3887 | `tests/cli.rs` (3887) |
| `tests/common/` | 1 | 71 | 71 | `tests/common/mod.rs` (71) |
| `tests/lock.rs` | 1 | 1495 | 1495 | `tests/lock.rs` (1495) |
| `tests/readme.rs` | 1 | 972 | 972 | `tests/readme.rs` (972) |
| **total** | 30 | 27554 | 6425 | |

### bench

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `src/compare.rs` | 1 | 548 | 0 | `src/compare.rs` (548) |
| `src/growth.rs` | 1 | 160 | 0 | `src/growth.rs` (160) |
| `src/journeys.rs` | 1 | 513 | 0 | `src/journeys.rs` (513) |
| `src/main.rs` | 1 | 528 | 0 | `src/main.rs` (528) |
| `src/ordinary.rs` | 1 | 564 | 0 | `src/ordinary.rs` (564) |
| `src/problems.rs` | 1 | 492 | 0 | `src/problems.rs` (492) |
| `src/ranking.rs` | 1 | 564 | 0 | `src/ranking.rs` (564) |
| `src/ratchet.rs` | 1 | 440 | 0 | `src/ratchet.rs` (440) |
| `src/run.rs` | 1 | 1259 | 0 | `src/run.rs` (1259) |
| `src/summary.rs` | 1 | 753 | 0 | `src/summary.rs` (753) |
| `src/tools.rs` | 1 | 1480 | 0 | `src/tools.rs` (1480) |
| `src/translate.rs` | 1 | 682 | 0 | `src/translate.rs` (682) |
| `tests/compare.rs` | 1 | 199 | 199 | `tests/compare.rs` (199) |
| `tests/manual.rs` | 1 | 229 | 229 | `tests/manual.rs` (229) |
| **total** | 14 | 8411 | 428 | |

### web

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `build.rs` | 1 | 17 | 0 | `build.rs` (17) |
| `src/calls.rs` | 1 | 2609 | 0 | `src/calls.rs` (2609) |
| `src/lambda.rs` | 1 | 191 | 0 | `src/lambda.rs` (191) |
| `src/lib.rs` | 1 | 823 | 0 | `src/lib.rs` (823) |
| `src/modal.rs` | 1 | 205 | 0 | `src/modal.rs` (205) |
| `src/ordinary.rs` | 1 | 1383 | 0 | `src/ordinary.rs` (1383) |
| `src/protocol.rs` | 1 | 494 | 0 | `src/protocol.rs` (494) |
| `src/rules.rs` | 1 | 740 | 0 | `src/rules.rs` (740) |
| `src/tab.rs` | 1 | 22 | 0 | `src/tab.rs` (22) |
| `src/workspace.rs` | 1 | 610 | 0 | `src/workspace.rs` (610) |
| `tests/calls.rs` | 1 | 1478 | 1478 | `tests/calls.rs` (1478) |
| `tests/lambda.rs` | 1 | 67 | 67 | `tests/lambda.rs` (67) |
| `tests/modal.rs` | 1 | 78 | 78 | `tests/modal.rs` (78) |
| `tests/ordinary.rs` | 1 | 691 | 691 | `tests/ordinary.rs` (691) |
| `tests/protocol.rs` | 1 | 128 | 128 | `tests/protocol.rs` (128) |
| `tests/rules.rs` | 1 | 388 | 388 | `tests/rules.rs` (388) |
| `tests/workspace.rs` | 1 | 164 | 164 | `tests/workspace.rs` (164) |
| **total** | 17 | 10088 | 2994 | |

### web-render

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `build.rs` | 1 | 16 | 0 | `build.rs` (16) |
| `src/lib.rs` | 1 | 63 | 0 | `src/lib.rs` (63) |
| `tests/render.rs` | 1 | 82 | 82 | `tests/render.rs` (82) |
| **total** | 3 | 161 | 82 | |

### web-tab

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `src/failure.rs` | 1 | 67 | 0 | `src/failure.rs` (67) |
| `src/lib.rs` | 1 | 207 | 0 | `src/lib.rs` (207) |
| **total** | 2 | 274 | 0 | |

### compat

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `linlog-cli/src/` | 1 | 13 | 0 | `linlog-cli/src/main.rs` (13) |
| `linlog/src/` | 1 | 24 | 0 | `linlog/src/lib.rs` (24) |
| **total** | 2 | 37 | 0 | |

### The 40 largest Rust files

| lines | file |
|---:|---|
| 5552 | `core/src/linear/search/mod.rs` |
| 5258 | `core/src/ordinary/interactive.rs` |
| 3887 | `cli/tests/cli.rs` |
| 3688 | `core/src/ordinary/normal.rs` |
| 3469 | `core/tests/export.rs` |
| 3286 | `cli/src/argument_parsing.rs` |
| 2919 | `core/src/ordinary/derivation.rs` |
| 2609 | `web/src/calls.rs` |
| 2563 | `core/src/ordinary/resolution/first_order.rs` |
| 2438 | `core/src/export/svg/mod.rs` |
| 2379 | `core/src/ordinary/mod.rs` |
| 2280 | `core/src/rules/interactive.rs` |
| 2237 | `core/src/linear/search/focus/tests.rs` |
| 2226 | `core/src/linear/search/focus/mod.rs` |
| 2211 | `core/src/ordinary/herbrand.rs` |
| 2177 | `core/src/linear/nets/mod.rs` |
| 2119 | `core/src/linear/search/focus/framed.rs` |
| 2117 | `core/src/linear/proofs/interactive/mod.rs` |
| 2072 | `core/src/linear/search/refutation.rs` |
| 2065 | `core/src/serialize/ordinary.rs` |
| 1959 | `core/src/modal/search.rs` |
| 1939 | `core/src/protocol/ordinary.rs` |
| 1927 | `core/src/workspace/mod.rs` |
| 1840 | `core/src/ordinary/written/mod.rs` |
| 1820 | `core/src/errors/mod.rs` |
| 1779 | `core/src/linear/search/net.rs` |
| 1777 | `core/src/export/svg/mell.rs` |
| 1763 | `core/tests/first_order.rs` |
| 1739 | `core/src/ordinary/grounding.rs` |
| 1733 | `cli/src/show.rs` |
| 1677 | `core/tests/rules.rs` |
| 1643 | `core/src/ordinary/ljt.rs` |
| 1641 | `core/src/lambda/tests.rs` |
| 1636 | `core/src/export/rocq/mod.rs` |
| 1618 | `cli/src/batch.rs` |
| 1583 | `core/src/linear/search/reference_fo.rs` |
| 1525 | `core/src/linear/proofs/cut/mod.rs` |
| 1495 | `cli/tests/lock.rs` |
| 1486 | `core/src/linear/proofs/derivation/classical.rs` |
| 1480 | `bench/src/tools.rs` |

Files: 324; over 1 000 lines: 94; over 3 000: 6; over 5 000: 2.

### rocq-deduxit theories

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `Intuitionistic/` | 9 | 1965 | 0 | `Intuitionistic/FirstOrderCheck.v` (489) |
| `Linear/` | 20 | 5682 | 0 | `Linear/FirstOrderIntuitionisticCheck.v` (770) |
| `Modal/` | 7 | 1318 | 0 | `Modal/Soundness.v` (371) |
| **total** | 36 | 8965 | 0 | |

| lines | file |
|---:|---|
| 770 | `theories/Linear/FirstOrderIntuitionisticCheck.v` |
| 745 | `theories/Linear/FirstOrderSoundness.v` |
| 697 | `theories/Linear/Weights.v` |
| 534 | `theories/Linear/Refutations.v` |
| 505 | `theories/Linear/IntuitionisticCheck.v` |
| 501 | `theories/Linear/FirstOrderWeights.v` |
| 489 | `theories/Intuitionistic/FirstOrderCheck.v` |
| 422 | `theories/Intuitionistic/FirstOrderKripke.v` |
| 371 | `theories/Modal/Soundness.v` |
| 352 | `theories/Intuitionistic/FirstOrderClassical.v` |
| 342 | `theories/Linear/Soundness.v` |
| 327 | `theories/Linear/FirstOrderCheck.v` |
| 287 | `theories/Modal/Countermodel.v` |
| 272 | `theories/Modal/Check.v` |
| 227 | `theories/Intuitionistic/KripkeCheck.v` |
| 200 | `theories/Linear/Check.v` |
| 176 | `theories/Modal/Kripke.v` |
| 157 | `theories/Intuitionistic/Kripke.v` |
| 149 | `theories/Linear/FirstOrderIntuitionistic.v` |
| 120 | `theories/Linear/Intuitionistic.v` |
| 120 | `theories/Intuitionistic/FirstOrderCalculus.v` |
| 118 | `theories/Linear/TermsCheck.v` |
| 117 | `theories/Modal/Calculus.v` |
| 115 | `theories/Linear/Formulas.v` |
| 100 | `theories/Linear/FirstOrderFormulas.v` |
| 99 | `theories/Linear/Mix.v` |
| 82 | `theories/Linear/FirstOrderCalculus.v` |
| 81 | `theories/Intuitionistic/FirstOrderFormulas.v` |
| 78 | `theories/Intuitionistic/Calculus.v` |
| 77 | `theories/Linear/Calculus.v` |
| 73 | `theories/Linear/FirstOrderProofs.v` |
| 65 | `theories/Linear/Proofs.v` |
| 63 | `theories/Linear/Terms.v` |
| 62 | `theories/Modal/Labelled.v` |
| 39 | `theories/Intuitionistic/Formulas.v` |
| 33 | `theories/Modal/Formulas.v` |

### rocq-deduxit tests

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `Assumptions.v` | 1 | 71 | 0 | `Assumptions.v` (71) |
| `Certificates.v` | 1 | 114 | 0 | `Certificates.v` (114) |
| `Cuts.v` | 1 | 92 | 0 | `Cuts.v` (92) |
| `FirstOrder.v` | 1 | 141 | 0 | `FirstOrder.v` (141) |
| `FirstOrderTwoSided.v` | 1 | 77 | 0 | `FirstOrderTwoSided.v` (77) |
| `FirstOrderWeights.v` | 1 | 67 | 0 | `FirstOrderWeights.v` (67) |
| `FromCommand.v` | 1 | 75 | 0 | `FromCommand.v` (75) |
| `Intuitionistic/` | 2 | 374 | 0 | `Intuitionistic/FirstOrder.v` (231) |
| `Modal/` | 2 | 317 | 0 | `Modal/Certificates.v` (201) |
| `Modes.v` | 1 | 57 | 0 | `Modes.v` (57) |
| `Refutations.v` | 1 | 112 | 0 | `Refutations.v` (112) |
| `TwoSided.v` | 1 | 137 | 0 | `TwoSided.v` (137) |
| `Weights.v` | 1 | 160 | 0 | `Weights.v` (160) |
| **total** | 15 | 1794 | 0 | |

### rocq-deduxit bridges

| module | files | lines | of which test files | largest file (lines) |
|---|---:|---:|---:|---|
| `Assumptions.v` | 1 | 13 | 0 | `Assumptions.v` (13) |
| `NanoIllBridge.v` | 1 | 169 | 0 | `NanoIllBridge.v` (169) |
| `NanoYallaBridge.v` | 1 | 154 | 0 | `NanoYallaBridge.v` (154) |
| **total** | 3 | 336 | 0 | |
