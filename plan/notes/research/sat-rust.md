# SAT solvers for deduxit: Rust options, proofs, recommendation

Research of 2026-11-09 for step 54s (`plan/54s-sat-engines.md`), by a
sub-agent of the planning session, at the author's request ("research the
best practices and availability of sat solvers in rust"). Every number below was read on
2026-11-09 from the crates.io API (`https://crates.io/api/v1/crates/NAME`),
the GitHub API (`api.github.com/repos/OWNER/REPO`, `.../commits`,
`.../issues`), or the crate sources downloaded from crates.io into the
scratchpad; "verified" means I ran it here, "read" means I read the source
or the page, "not verified" means I could not check it.

The throwaway crates, benchmark harness, instances and raw results were
kept in a scratch directory of the planning session, not in the
repository; the numbers below are their results.

## 0. Short answer

- **Intuitionistic engine (intuit/intuitR style): `batsat` 0.6.0, used
  directly (not through rustsat).** Pure Rust, MIT, MiniSat-derived, with
  exactly the MiniSat incremental API intuit and intuitR use (solve under
  assumptions, add clauses between calls, read the model, the failed
  assumptions as `unsat_core`, negated as in MiniSat's `conflict`,
  verified), a `Callbacks::stop` hook polled at every
  decision (deduxit's stop callback and a conflict budget fit there), no
  clock, no threads, no dependency but `bit-vec`. Verified: it builds for
  wasm32-unknown-unknown and runs under Node with **zero imports**.
- **Not through `rustsat-batsat`:** that wrapper reads a clock on every
  solve (verified: its wasm build imports `performance` and `now` through
  wasm-bindgen), exposes no stop or limits for batsat, and rustsat's
  solver API is about to be redesigned.
- **Classical engine: the same `batsat`, with deduxit's own checker.**
  batsat reports every learnt clause and every deleted clause through
  `Callbacks::on_new_clause` / `on_delete_clause` (that is how its DRAT
  output is made), so deduxit can record a DRUP proof in memory and check
  it with its own RUP checker. LRAT hints are not produced by batsat; the
  checker either finds them by unit propagation (forward DRUP check) or a
  small fork of batsat adds the antecedent ids in `analyze`.
- **Optional native speed tier, later:** CaDiCaL (MIT) behind a feature
  that is off for wasm, through `cadical-sys` 0.8.0 (CaDiCaL 3.0.1, cxx,
  no libclang needed) or `rustsat-cadical` 0.7.5 (CaDiCaL 2.2.1, needs
  libclang for bindgen at build time, verified failing here without it).
  CaDiCaL writes LRAT natively and its proof-tracer API hands the clause
  ids and antecedents to Rust in memory. It cannot build for
  wasm32-unknown-unknown (verified: cc needs clang, and C++ needs a
  standard library the target does not have; rustsat's open PR #762 adds
  Emscripten builds for exactly that reason).
- **Speed given up (my runs, section 3.3):** none on easy instances,
  about 1.5x CaDiCaL 1.9.5 on medium random 3-SAT and on an intuit-shaped
  run of 3000 incremental calls, and much more on SAT Competition 2025
  instances (batsat solved 3 of 6 in 100 s, CaDiCaL 5). In Node, batsat
  runs at about 1.2x its native time.
- **Fallback if batsat fails:** `varisat` 0.2.2 (pure Rust, MIT OR
  Apache-2.0, assumptions + failed core, DRAT and in-memory checked proof
  steps with hints = LRAT via `varisat-lrat`), but it has **no stop
  callback or budget** (issue #111 open since 2019) and no release since
  2020-09-09; using it means a fork. The last resort is deduxit's own
  CDCL of a few thousand lines, which also gives LRAT hints by design.
- **Excluded by licence:** `splr` (MPL-2.0; also no incremental mode since
  0.18), `pindakaas` (MPL-2.0; its solver build crates are MIT),
  `otter_sat` (AGPL-3.0), `saguaro` (GPL-3.0), `ipasir-loading`
  (GPL-3.0). Glucose (in `rustsat-glucose` and `minisat`) is labelled MIT
  on crates.io but its parallel part (Syrup) carries an extra clause
  (no use in competitive events without permission), so it is not plain MIT.

(The rest of this file has the evidence.)

## 1. The candidates

### 1.1 Table (crates.io and GitHub, read 2026-11-09)

"dl" = all-time downloads / last 90 days on crates.io. "Pushed" = last
push on GitHub (any branch); "last commit" = newest commit on the default
branch where I looked.

| crate | licence (crates.io) | latest | date | dl | under it | maintenance |
|---|---|---|---|---|---|---|
| batsat | MIT | 0.6.0 | 2025-01-20 | 137 337 / 49 690 | pure Rust (dep: bit-vec) | c-cube/batsat: last commits 2026-05-05 ("update deps", "CI"); 7 open (5 PRs from 2024 unmerged, 2 old issues); 32 stars |
| rustsat | MIT | 0.7.5 | 2026-01-30 | 119 879 / 29 189 | Rust (deps: anyhow, nom, itertools, thiserror, tempfile; cpu-time or web-time) | chrjabs/rustsat: last commit 2026-10-15, 35 open; SAT 2025 tool paper; typestate API redesign announced (#565-#573, 2026-01-24) |
| rustsat-batsat | MIT | 0.7.5 | 2026-01-30 | 28 991 / 13 434 | wraps batsat 0.6.0 | as rustsat |
| rustsat-cadical | MIT | 0.7.5 | 2026-01-30 | 46 772 / 3 996 | C++ CaDiCaL 2.2.1 (default; 1.5.0-2.2.1 selectable), cc + bindgen | as rustsat |
| rustsat-kissat | MIT | 0.7.5 | 2026-01-30 | 28 118 / 1 727 | C Kissat 4.0.4 | as rustsat; Kissat is not incremental |
| rustsat-minisat | MIT | 0.7.5 | 2026-01-30 | 61 346 / 10 343 | C++ MiniSat 2.2.0, cmake | as rustsat |
| rustsat-glucose | MIT (see note) | 0.7.5 | 2026-01-30 | 27 285 / 733 | C++ Glucose 4.2.1, cmake | as rustsat |
| rustsat-ipasir | MIT | 0.7.5 | 2026-01-30 | 10 522 / 25 | links any IPASIR library | as rustsat |
| varisat | MIT/Apache-2.0 | 0.2.2 | 2020-09-09 | 7 827 938 / 396 658 | pure Rust | jix/varisat: last commit 2022-11-02, 25 open; downloads come from `bool2cnf` and `pubgrub` |
| varisat-checker | MIT/Apache-2.0 | 0.2.2 | 2020-09-09 | 7 822 823 | pure Rust | as varisat |
| varisat-lrat | MIT/Apache-2.0 | 0.2.2 | 2020-09-09 | 4 382 | pure Rust | as varisat |
| splr | **MPL-2.0** | 0.19.0 | 2026-08-22 | 52 462 / 3 380 | pure Rust | shnarazk/splr active (2026-08-22); excluded by licence |
| CreuSAT | MIT (GitHub) | not on crates.io | - | - | pure Rust, verified with Creusot | sarsko/CreuSAT: last commit 2026-08-20; 705 stars |
| screwsat | MIT | 2.1.5 | 2021-01-27 | 24 501 / 56 | pure Rust | togatoga/screwsat: commit 2026-02-19 |
| ratsat | MIT | 0.0.1 | 2018-03-29 | 2 894 | pure Rust | qnighy/ratsat archived (2018) |
| cadical (mmaroti) | MIT | 0.1.16 | 2025-04-24 | 81 029 / 40 607 | C++ CaDiCaL 1.9.5, cc only | mmaroti/cadical-rs: last commit 2025-05-08 |
| cadical-sys | MIT | 0.8.0 | 2026-10-11 | 11 491 / 1 342 | C++ CaDiCaL 3.0.1, cxx + cc | sirandreww/cadical-sys: pushed 2026-10-11, 0 open, 5 stars |
| satif-cadical / satif-kissat | MIT | 0.2.2 / 0.5.2 | 2025-09-05 | 12 956 / 12 530 | C++/C | gipsyh, pushed 2026-10-24 |
| pindakaas-cadical / -kissat | MIT | 0.4.2 / 0.2.4 | 2026-10-19 | 35 010 / 1 405 | C++/C, builds for `pindakaas` (MPL-2.0) | pindakaashq, active |
| kissat (jleahy) | MIT | 0.1.0 | 2021-10-10 | 8 117 | C | GitHub repo gone (404) |
| cat_solver | MIT | 3.1.1 | 2023-10-13 | 4 626 | C Kissat | dormant |
| minisat (luteberget) | MIT | 0.4.4 | 2019-11-17 | 26 728 | C++ MiniSat + Glucose | last commit 2022-01-26 |
| cryptominisat | MIT | 5.8.0 | 2020-12-15 | 11 476 | C++ CryptoMiniSat 5.8, cmake | last push 2021-03-02 |
| ipasir / ipasir-sys | MIT/Apache-2.0 / MIT | 0.3.1 / 0.3.0 | 2020-04-12 / 2020-07-08 | 6 624 / 8 278 | FFI only | dormant |
| z3 | MIT | 0.22.0 | 2026-10-28 | 2 342 323 | C++ Z3 | active; an SMT solver, far heavier than needed |
| oxiz-sat, crabsat, combopt, sat-solver, logicaffeine-proof | Apache-2.0 / MIT / MIT OR Apache-2.0 / MIT / MIT | 0.3.4 / 0.1.3 / 0.1.0 / 0.2.1 / 0.11.0 | 2025-06 to 2026-11 | 17 481 / 58 / 13 / 4 506 / 1 765 | pure Rust | all new in 2025-2026, no track record; not recommended |
| otter_sat | **AGPL-3.0** | 0.1.0 | 2025-02-14 | 2 195 | pure Rust | excluded |
| saguaro | **GPL-3.0** | 0.1.0 | 2025-05-13 | 601 | pure Rust | excluded |
| pumpkin-solver | MIT OR Apache-2.0 | 0.5.0 | 2026-08-05 | 25 709 | pure Rust | a CP (lazy clause generation) solver with proof logging, not a SAT library |
| resolvo | BSD-3-Clause | 0.12.2 | 2026-10-08 | 926 647 | pure Rust | a package resolver with its own CDCL, no general CNF API |

Glucose note (read in `rustsat-glucose-0.7.5/cppsrc/LICENCE`): "The
parallel version of Glucose (all files modified since Glucose 3.0
releases, 2013) cannot be used in any competitive event (sat
competitions/evaluations) without the express permission of the
authors". The crate metadata says MIT; the bundled sources are MIT plus
that condition. `minisat` 0.4.4 bundles `glucose-syrup-4.1` too.

### 1.2 What each one offers that matters to deduxit

**batsat 0.6.0** (read: `src/interface.rs`, `callbacks.rs`, `core.rs`,
`drat.rs`, `theory.rs`)
- MiniSat 2.2 reimplemented in Rust (README: "forked from ratsat, a
  reimplementation of MiniSat"). About 4 900 lines.
- Incremental: `solve_limited(&[Lit])` under assumptions;
  `add_clause_reuse` between calls; `value_lit` / `get_model`;
  `unsat_core()`; `proved_at_lvl_0()`. Its own test `test_reg7` adds
  clauses after a solve and re-solves.
- `unsat_core()` returns MiniSat's final conflict clause, i.e. the
  **negations** of the failed assumptions, although its doc says "as a
  subset of assumptions". Verified (`wasm/t-core`): clauses (a or b),
  (not a or b), assumptions [c, not b] give core [b]; c is left out;
  adding (not c) afterwards and solving again works. Haskell MiniSat's
  `conflict` has the same polarity, which is why intuit writes
  `map neg` over it.
- Stop: `Callbacks::stop(&self) -> bool`, checked in `within_budget()` at
  every decision without conflict (core.rs:538, 980-985), so a stop
  callback answers within one propagation round. `BasicCallbacks::set_stop`
  takes a `'static Fn() -> bool`; a custom `Callbacks` type (the solver is
  generic over it) can hold anything, count conflicts in `on_new_clause`
  and so enforce a conflict budget. The fields `conflict_budget` /
  `propagation_budget` exist but have no public setter.
- Proof: no proof file of its own in the library, but every learnt clause
  goes through `Callbacks::on_new_clause(&[Lit], Kind)` and every clause
  removed by `reduce_db` through `on_delete_clause`; `drat::Proof` turns
  these into DRAT text. No clause ids, no antecedents (so no LRAT).
- SMT hook: a `Theory` trait (`partial_check`, `final_check`,
  `create_level`/`pop_levels`, propagate + `explain_propagation`) - the
  pure-Rust analogue of IPASIR-UP.
- No clock, no threads (`AsyncInterrupt` uses `Arc<AtomicBool>`, which
  compiles on wasm). Verified: builds for wasm32-unknown-unknown; a
  cdylib exporting a solver has **0 imports** and runs under Node.

**rustsat 0.7.5 + rustsat-batsat** (read: `rustsat-batsat-0.7.5/src/lib.rs`,
`rustsat-0.7.5/src/utils.rs`, Cargo.toml)
- rustsat-batsat implements only `Solve`, `SolveIncremental` (`solve_assumps`,
  `core`) and `SolveStats`; **no `Terminate`, `Interrupt` or `Limit*`**.
  `batsat_mut()` gives the inner solver, and the wrapper is generic over
  batsat's `Callbacks`, so a stop hook is still possible underneath.
- Every `solve_assumps` reads a clock: `Timer::now()` around the call,
  which is `cpu_time::ThreadTime` on unix/windows and `web_time::Instant`
  on other targets (wasm: JavaScript `performance.now()` through
  wasm-bindgen). That breaks deduxit's "the library reads no clock".
- Always pulls anyhow, nom, itertools, thiserror, tempfile. The author
  plans to drop anyhow (#577) and to replace the solver API with a
  typestate API (#565-#573), so the API will break; 0.7.5 is the last
  release (2026-01-30) though main moved on to 2026-10-15.
- SAT 2025 paper (Jabs, LIPIcs SAT 2025, arXiv:2505.15221): "Choosing
  BatSat allows for projects to be pure Rust, and therefore compile to,
  e.g., web assembly." Its table: Kissat 3.0.0-4.0.2 not incremental;
  CaDiCaL 1.5.0-2.1.3, MiniSat 2.2.0, Glucose 4.2.1, BatSat 0.6.0, IPASIR
  incremental. CaDiCaL implements every trait but `LimitPropagations`.
- rustsat-cadical: `Terminate`, `Learn`, `Interrupt`, `PhaseLit`,
  `FreezeVar`, `FlipLit`, `LimitConflicts`, `LimitDecisions`, `Propagate`,
  `trace_proof(path, ProofFormat::{Drat, Lrat, Frat, ...})` (a file) and
  `connect_proof_tracer` (an in-memory `TraceProof` trait with clause ids
  and antecedents, i.e. LRAT in memory). IPASIR-UP for CaDiCaL is an open
  issue (#575, #587).

**varisat 0.2.2** (read: `src/solver.rs`, `src/proof.rs`, checker)
- `assume(&[Lit])` + `solve()`, `model()`, `failed_core()`, clauses added
  between calls. Pure Rust; builds for wasm32-unknown-unknown (verified).
- Proof: `write_proof(target, ProofFormat::{Varisat, Drat, BinaryDrat})`,
  and `add_proof_processor(&mut dyn ProofProcessor)`, which runs varisat's
  own checker in-process and hands each `CheckedProofStep` with the unit
  propagation hints to the processor; `varisat-lrat` writes these as LRAT.
- **No stop callback, no budget**: `SolverError::Interrupted` exists but
  nothing public triggers it. Issue #111 "Solving timeout" (2019-08-21,
  open): the author planned an `ipasir_set_terminate` equivalent. Issue
  #37 "Compiling to WASM" open since 2019. No IPASIR (#1).
- README (docs.rs varisat-cli 0.1.3): "competitive performance for some
  problems that arise in practice, but there are also problems where
  Varisat's run-time will be orders of magnitude slower compared to state
  of the art solvers" (read via search result, not re-checked for 0.2.2).

**splr 0.19.0**: MPL-2.0 (not allowed). README: "As of version 0.18.0,
Splr no longer supports incremental mode". DRAT certificates (`--certify`).
Self-reported results on 40 SAT Competition 2025 instances (16 timeouts).
Builds for wasm32 (verified) with feature `platform_wasm` = `instant`.

**CreuSAT**: MIT, deductively verified with Creusot (SAT/UNSAT answers
proven correct, no panics), GitHub only, no incremental API or proof
output mentioned; thesis (Skotåm, Oslo 2022) claims "the world's fastest
deductively verified SAT solver". Not a library to depend on.

**screwsat 2.1.5**: no assumptions API; `solve(time_limit)` calls
`std::time::Instant::now()` unconditionally (lib.rs:1816), which std
does not support on wasm32-unknown-unknown (panics; read, not run).
Builds for wasm (verified) but unfit.

**ratsat 0.0.1**: archived 2018; batsat is its continuation.

**CaDiCaL bindings**
- `cadical` 0.1.16 (mmaroti): CaDiCaL 1.9.5; `solve_with(assumptions)`,
  `failed`, `value`, `set_limit("conflicts"| "decisions", n)`,
  `set_callbacks` (terminate, learn); builds with cc only (no libclang).
  No proof API.
- `cadical-sys` 0.8.0: CaDiCaL 3.0.1, a 1:1 cxx binding: `assume`,
  `failed`, `connect_terminator`, `connect_external_propagator`
  (IPASIR-UP), `limit`, `trace_proof`, `connect_proof_tracer`. Builds
  with cc + cxx-build (no libclang). Young (first release 2024-10-30), one
  maintainer.
- `rustsat-cadical` 0.7.5: see above; build needs libclang (bindgen):
  verified failing here with a panic in `bindgen-0.72.1/lib.rs:616`
  because the devshell has no libclang.
- None builds for wasm32-unknown-unknown (verified for `cadical` and
  `rustsat-cadical`, `rustsat-minisat`; see 1.3).

**Kissat bindings** (`rustsat-kissat`, `satif-kissat`, `cat_solver`,
`kissat`): Kissat is not incremental (rustsat docs: "a non-incremental
SAT solver"), so useless for the intuitionistic engine. For the classical
engine it is the strongest SAT solver, but `rustsat-kissat` 0.7.5 exposes
no proof output (its impls: `Solve`, `Terminate`, `Interrupt`,
`SolveStats`; read) and none of these builds for wasm. I did not check
the other Kissat crates for proof options.

### 1.3 wasm32-unknown-unknown, verified

Built with the devshell's rustc 1.99.1 (target installed), each crate a
throwaway lib in `wasm/`, `cargo build --release --target
wasm32-unknown-unknown`, in `systemd-run --user --scope -p MemoryMax=8G
-p MemorySwapMax=0 taskset -c 4-9`:

| crate | result |
|---|---|
| batsat 0.6.0 | builds (log `wasm/t-batsat.wasm.log`) |
| varisat 0.2.2 | builds |
| rustsat 0.7.5 (no default features) + rustsat-batsat 0.7.5 | builds |
| screwsat 2.1.5 | builds; not run: its `solve` calls `std::time::Instant::now()`, which std does not support on this target (a panic at run time) |
| splr 0.19.0 | builds (MPL-2.0 anyway) |
| rustsat-minisat 0.7.5 | fails: cmake/clang not found (`CMAKE_SYSTEM_NAME=unknown`, `--target=wasm32-unknown-unknown`) |
| rustsat-cadical 0.7.5 | fails: "failed to run test compilation" (no clang) |
| cadical 0.1.16 | fails: cc-rs "failed to find tool clang++" |

The C/C++ failures here are for want of clang, but clang alone would not
do: wasm32-unknown-unknown has no C or C++ standard library. rustsat PR
#762 "Emscripten support" (open, 2026-10-13, by C. Jefferson) says it
"adds support to build all solvers for 'emscripten wasm'. emscripten
compiles C++ to wasm by providing a full standard library", that
"doing 'pure wasm' with batsat is better" for many purposes, and that it
fixed a Kissat bug that "stopped it working on 32-bit systems". deduxit-web
uses wasm-bindgen on wasm32-unknown-unknown; I did not verify whether an
Emscripten-built solver can be linked into that.

Run-time check (verified): `bench/batwasm` (batsat as a cdylib with
`alloc`, `solve_dimacs`, `incr` exported) built for
wasm32-unknown-unknown is about 108 KB (release, not wasm-opt), has **0 imports**
(`WebAssembly.Module.imports(...).length === 0`) and runs under Node
with the same answers as native (numbers in section 3.3).

## 2. Proof checking in Rust

### 2.1 Checker crates (crates.io, 2026-11-09)

| crate | licence | latest, date | dl | what it checks | verdict |
|---|---|---|---|---|---|
| varisat-checker | MIT/Apache-2.0 | 0.2.2, 2020-09-09 | 7.8 M (via varisat's dependents) | varisat's own proof format; produces `CheckedProofStep`s with propagation hints for `ProofProcessor`s | tied to varisat; unmaintained since 2020 |
| varisat-lrat | MIT/Apache-2.0 | 0.2.2, 2020-09-09 | 4 382 | writes LRAT/CLRAT from varisat-checker's steps (a producer, not a checker) | as above |
| rate (+ rate-proof-utils) | MIT | 0.3.0, 2020-01-01 | 7 380 | DRAT and DPR | dormant (repo push 2022-02-12, 14 open) |
| dratify | Apache-2.0 | 0.1.11, 2026-11-09 | 481 | DRAT/DRUP, forward | first release 2026-08-29, 0 stars; too new |
| ordeal-lrat | Apache-2.0 | 0.36.0, 2026-11-05 | 22 072 | LRAT, "the sole trusted component" of a QF_BV SMT project | first release 2026-07-02, 42 versions in 4 months, 1 star; too new |
| logicaffeine-proof | MIT | 0.11.0, 2026-11-03 | 1 765 | a large proof library with "certified SAT/CDCL with DRAT/LRAT output" | 2026, not a focused dependency |

No crate named `lrat`, `lrat-check`, `drat-trim` or `frat-rs` exists on
crates.io. `frat-rs` (Carneiro, FRAT to LRAT elaborator in Rust) lives at
github.com/digama0/frat (last push 2023-04-08, no licence detected by
GitHub).

### 2.2 The standard and verified checkers (references)

- **drat-trim** (Heule; github.com/marijnheule/drat-trim, MIT, last push
  2024-11-25): the standard DRAT checker; it also turns DRAT into LRAT
  (backward checking finds the hints). Its repository holds
  `lrat-check.c`, a small LRAT checker in C (file listing via the GitHub
  API; I did not read the file).
- **LRAT** was introduced by Cruz-Filipe, Heule, Hunt, Kaufmann,
  Schneider-Kamp, "Efficient Certified RAT Verification", CADE 2017 (LNCS
  10395, pp. 220-236, arXiv:1612.02353), with two certified checkers
  extracted, **one in Coq** and one in ACL2; the paper reports certified
  checking almost as fast as uncertified checking (via search). The Coq
  one matters for deduxit's Rocq library.
- **cake_lpr** (Tan, Heule, Myreen, "cake_lpr: Verified Propagation
  Redundancy Checking in CakeML", TACAS 2021, PMC7984575; extended in STTT
  2023, doi 10.1007/s10009-022-00690-y; github.com/tanyongkiam/cake_lpr,
  pushed 2026-11-01): verified down to machine code, checks LRAT and LPR.
  The SAT Competition uses it: the 2024 checker document
  (satcompetition.github.io/2024/downloads/checkers/cakelpr.pdf) describes
  DRAT going through drat-trim to LRAT, then cake_lpr; "Solvers capable of
  generating LRAT proofs directly can skip the use of DRAT-trim" (via
  search). The 2025 slides (read directly): "Certified results of
  unsatisfiability using proof logging", an instance is "not solved" if
  the proof checker fails or times out; participants picked one of three
  verified checkers: cake_lpr (LRAT and LPR), GRAT (Lammich) or
  VeriPB/CakePB; and in the closing remarks "Checking time is roughly
  similar to solving time / However, LRAT checking is much faster".
- **LRAT straight from the solver**: Pollitt, Fleury, Biere, "Faster LRAT
  Checking than Solving with CaDiCaL", SAT 2023 (LIPIcs SAT.2023.20).
  Read on its pages 1-2: CaDiCaL produces LRAT natively for all its
  procedures; "neither our SAT solver CaDiCaL nor any top performing SAT
  solver in the SAT Competition over the last 2 years actually used" RAT,
  so the proofs are "LRUP" (RUP steps with hint chains); on the SAT
  Competition 2022 UNSAT problems, "(almost) no slow-down without proof
  production (0.3%) and only a small slow-down for producing LRAT (5%)",
  and checking plus producing LRAT costs "30% over pure solving, compared
  to 125% for FRAT and 180% in the SAT Competition mode". Their checker
  is lrat-trim.
- **Proof assistants write their own LRAT checker**: Lean 4's
  `bv_decide` (LeanSAT, merged into Lean core in 2024; Lean 4.12.0 ships
  CaDiCaL) runs CaDiCaL to get an LRAT proof and checks it with "an LRAT
  checker with a soundness proof in Lean" (Lean 4.12.0 release notes and
  the LeanSAT README, via search). This is the same shape as deduxit's
  rule that the library checks the certificate itself.

### 2.3 Is a checker of deduxit's own the usual practice?

Yes, for a tool whose answer must be trusted: the checker is the trusted
part, so it is kept small and separate from the solver (cake_lpr, the
ACL2 checker, Lean's checker). An LRUP checker is short: per added clause,
assign the negation of its literals, then walk the hint ids in order; each
hinted clause must be unit (extend the assignment) and the last one
falsified. Deletions free clauses. RAT steps are not needed for CaDiCaL's
proofs (above) nor for a MiniSat-style solver like batsat, whose learnt
clauses are all RUP. Without hints, a forward DRUP check (unit-propagate
over all live clauses) is equally simple and, for the sizes deduxit
decides, cheap; its cost grows with the proof, which the existing work
bounds can cap. deduxit's Rocq library could later carry a verified
checker of the same format, as ACL2, Coq (CADE 2017), CakeML and Lean
did.

## 3. Incremental SAT inside a library: what is current practice

### 3.1 What intuit and intuitR use (read today)

- intuit (github.com/koengit/intuit): Haskell; `intuit.cabal` has
  `build-depends: base >=4, minisat >=0.1 && <0.2, containers`. `Intuit.hs`
  calls `newSolver`, `newLit`, `addClause`, `solve sat (neg goal :
  assumps)`, `conflict sat` (the failed assumptions), `modelValue`, and
  adds clauses between calls (line 165).
- intuitR (github.com/cfiorentini/intuitR, BSD-3-Clause): Haskell;
  `intuitR.cabal` depends on `minisat`; `src/Prover.hs` imports `MiniSat`
  ("https://hackage.haskell.org/package/minisat"), calls `solve sat ( neg
  b : a : getWAtms w )`, `conflict sat`, `addClause`.
- The Hackage package `minisat` ("A Haskell bundle of the Minisat SAT
  solver", BSD-3-Clause, 0.1.4 uploaded 2025-07-12) wraps MiniSat (C++).
  The page does not say which MiniSat version; I did not check its source.
- So both engines need exactly: new variable, add clause at any time,
  solve under assumptions, model values, failed assumptions. No deletion
  of clauses, no theory propagation. batsat has all five.

### 3.2 IPASIR and IPASIR-UP

- IPASIR (github.com/biotomas/ipasir; SAT Race 2015, J. AI 2016): the C
  interface of the SAT competitions' incremental track: `signature`,
  `init`, `release`, `add`, `assume`, `solve`, `val`, `failed`,
  `set_terminate` (a callback the solver polls during search),
  `set_learn`. It is exactly the intuit API plus termination. Rust:
  `rustsat-ipasir` links any IPASIR library; `ipasir`, `ipasir-sys` are
  dormant since 2020.
- IPASIR-UP (Fazekas, Niemetz, Preiner, Kirchweger, Szeider, Biere, SAT
  2023, LIPIcs SAT.2023.8; JAIR 2024 "Satisfiability Modulo User
  Propagators"): external propagators for CDCL, implemented in CaDiCaL,
  used by cvc5 and SAT Modulo Symmetries. In Rust: `cadical-sys`'s
  `connect_external_propagator`; open issues in rustsat (#575, #587).
  batsat's `Theory` trait is the pure-Rust counterpart.
- IPASIR-2 (github.com/ipasir2): a working group's draft (configuration
  interface, asynchronous clause import); I found no released header.

For intuit-style search IPASIR-level functionality is enough: Claessen and
Rosén describe the method as DPLL(T)-like but implement it with plain
assumption calls. A user-propagator (IPASIR-UP or batsat's `Theory`)
would be an optimisation to try later, not a requirement.

### 3.3 Speed: what a pure-Rust solver gives up

Published evidence I found:
- No head-to-head study of Rust solvers against CaDiCaL or Kissat. The
  RustSAT paper measures wrapper overhead, not solvers (its Figure 1:
  RustSAT and C++ equal, PySAT about 3x slower, enumerating 1000 models
  with MiniSat 2.2.0).
- Varisat's README: "orders of magnitude slower" than the state of the
  art on some problems (qualitative).
- SAT Competition 2025 slides (satcompetition.github.io/2025/satcomp25slides.pdf,
  text extracted with pdftotext; 400 benchmarks, 26 sequential solvers):
  SAT ranking AE-Kissat-MAB (PAR-2 715.921, 173 solved), Kissat-public
  (1397.09, 163), Kissat-CURE (1525.91, 159); UNSAT ranking
  CaDiCaL-SC2025 (2327.00, 161), Kissat-VSA (2335.54, 160), AE-Kissat-bump
  (2358.10, 159); overall AE-Kissat-MAB (327), Kissat-public (321),
  Kissat-VSA (317). Every top entry is Kissat or CaDiCaL. No Rust
  solver appears in any official competition result I found; splr's
  results are self-reported (README: 0.17.0 solved 49 SAT + 34 UNSAT of
  the 2021 main track at 300 s; 0.19.0: 24 of 40 SC2025 problems within
  its runs, 16 timeouts).
- No evidence found for batsat beyond a link in its README to a
  benchpress page comparing it with MiniSat on easy problems; that page
  returned HTTP 500 today.

My own micro-benchmark (verified; a hint, not a study). Harness
`bench/satbench` (native, release, one core: `taskset -c 4`, cores 4-9
capped scope, sequential runs), `bench/batwasm` (batsat built for
wasm32-unknown-unknown, run under Node 0 imports). Times are wall clock
for adding the clauses and solving (the native harness parses before the
clock starts; the wasm one parses inside it). CaDiCaL is 1.9.5 through
the `cadical` crate (rustsat-cadical needs libclang, absent here). All
solvers agreed on every verdict.

(a) Random 3-SAT at ratio 4.26 (n = 200, 250, 300; generator
`scripts/gen.py`) and pigeonhole PHP(n, n-1); r3-n300-s0 left out (CaDiCaL
46.4 s; not run for the others):

| instance | cadical | batsat | varisat | batsat-wasm |
|---|---|---|---|---|
| php-10-9.cnf | UNSAT 2.747 | UNSAT 1.312 | UNSAT 7.479 | UNSAT 1.599 |
| php-8-7.cnf | UNSAT 0.033 | UNSAT 0.030 | UNSAT 0.041 | UNSAT 0.062 |
| php-9-8.cnf | UNSAT 0.213 | UNSAT 0.143 | UNSAT 0.638 | UNSAT 0.199 |
| r3-n200-s0.cnf | SAT 0.009 | SAT 0.051 | SAT 0.070 | SAT 0.086 |
| r3-n200-s1.cnf | SAT 0.027 | SAT 0.011 | SAT 0.003 | SAT 0.032 |
| r3-n200-s2.cnf | SAT 0.020 | SAT 0.007 | SAT 0.020 | SAT 0.028 |
| r3-n200-s3.cnf | SAT 0.162 | SAT 0.548 | SAT 0.478 | SAT 0.680 |
| r3-n200-s4.cnf | SAT 0.164 | SAT 0.258 | SAT 0.309 | SAT 0.319 |
| r3-n200-s5.cnf | UNSAT 0.431 | UNSAT 0.286 | UNSAT 0.268 | UNSAT 0.367 |
| r3-n250-s0.cnf | UNSAT 2.819 | UNSAT 3.217 | UNSAT 1.584 | UNSAT 3.801 |
| r3-n250-s1.cnf | SAT 0.273 | SAT 3.079 | SAT 2.442 | SAT 3.712 |
| r3-n250-s2.cnf | UNSAT 2.594 | UNSAT 4.016 | UNSAT 2.583 | UNSAT 4.468 |
| r3-n250-s3.cnf | SAT 0.079 | SAT 0.056 | SAT 0.011 | SAT 0.092 |
| r3-n250-s4.cnf | UNSAT 4.393 | UNSAT 4.119 | UNSAT 2.936 | UNSAT 4.534 |
| r3-n250-s5.cnf | UNSAT 2.860 | UNSAT 1.981 | UNSAT 1.958 | UNSAT 2.290 |
| r3-n300-s1.cnf | UNSAT 16.469 | UNSAT 26.458 | UNSAT 15.701 | UNSAT 30.767 |
| r3-n300-s2.cnf | SAT 0.575 | SAT 0.340 | SAT 2.103 | SAT 0.461 |
| r3-n300-s3.cnf | SAT 0.287 | SAT 3.014 | SAT 22.959 | SAT 3.702 |
| r3-n300-s4.cnf | SAT 0.019 | SAT 0.359 | SAT 0.046 | SAT 0.463 |
| r3-n300-s5.cnf | SAT 0.724 | SAT 3.028 | SAT 21.828 | SAT 3.708 |

- instances solved by all: 20
- cadical: total on those = 34.90 s, solved = 21 (with r3-n300-s0)
- batsat: total on those = 52.31 s, solved = 20
- varisat: total on those = 83.46 s, solved = 20
- batsat-wasm: total on those = 61.37 s, solved = 20

(b) Structured SATLIB instances (BMC by Biere, blocks world, Hanoi,
circuit bf/ssa; downloaded from www.cs.ubc.ca/~hoos/SATLIB): all easy,
so this measures start-up and preprocessing more than search:

| instance | cadical | batsat | varisat | batsat-wasm |
|---|---|---|---|---|
| bf0432-007.cnf | UNSAT 0.004 | UNSAT 0.004 | UNSAT 0.004 | UNSAT 0.027 |
| bf1355-638.cnf | UNSAT 0.001 | UNSAT 0.001 | UNSAT 0.001 | UNSAT 0.011 |
| bmc-galileo-8.cnf | SAT 0.654 | SAT 0.123 | SAT 0.064 | SAT 0.224 |
| bmc-galileo-9.cnf | SAT 0.419 | SAT 0.067 | SAT 0.055 | SAT 0.150 |
| bmc-ibm-10.cnf | SAT 0.311 | SAT 0.322 | SAT 0.365 | SAT 0.475 |
| bmc-ibm-11.cnf | SAT 0.669 | SAT 0.184 | SAT 0.085 | SAT 0.284 |
| bmc-ibm-12.cnf | SAT 1.447 | SAT 0.970 | SAT 1.349 | SAT 1.298 |
| bmc-ibm-13.cnf | SAT 0.598 | SAT 1.051 | SAT 0.318 | SAT 1.340 |
| bmc-ibm-1.cnf | SAT 0.044 | SAT 0.023 | SAT 0.023 | SAT 0.066 |
| bmc-ibm-2.cnf | SAT 0.002 | SAT 0.001 | SAT 0.002 | SAT 0.015 |
| bmc-ibm-3.cnf | SAT 0.097 | SAT 0.063 | SAT 0.058 | SAT 0.111 |
| bmc-ibm-4.cnf | SAT 0.058 | SAT 0.059 | SAT 0.102 | SAT 0.122 |
| bmc-ibm-5.cnf | SAT 0.009 | SAT 0.007 | SAT 0.010 | SAT 0.042 |
| bmc-ibm-6.cnf | SAT 0.159 | SAT 0.098 | SAT 0.185 | SAT 0.185 |
| bmc-ibm-7.cnf | SAT 0.007 | SAT 0.003 | SAT 0.006 | SAT 0.025 |
| bw_large.c.cnf | SAT 0.034 | SAT 0.028 | SAT 0.013 | SAT 0.062 |
| bw_large.d.cnf | SAT 0.114 | SAT 0.069 | SAT 0.101 | SAT 0.111 |
| hanoi4.cnf | SAT 0.026 | SAT 0.037 | SAT 0.044 | SAT 0.072 |
| hanoi5.cnf | SAT 0.360 | SAT 0.426 | SAT 0.470 | SAT 0.512 |
| ssa6288-047.cnf | UNSAT 0.004 | UNSAT 0.002 | UNSAT 0.004 | UNSAT 0.021 |

- instances solved by all: 20
- cadical: total on those = 5.02 s, solved = 20
- batsat: total on those = 3.54 s, solved = 20
- varisat: total on those = 3.26 s, solved = 20
- batsat-wasm: total on those = 5.15 s, solved = 20

(c) Incremental workload in the intuit shape (`satbench SOLVER incr`):
3000 variables, 6000 random 3-clauses, then 3000 calls, each under 5
random assumption literals followed by one added 3-clause; the call
sequence is fixed by a seeded xorshift, independent of the answers.

    batsat incr calls=3000 sat=2993 unsat=7 time=0.684s
    cadical incr calls=3000 sat=2993 unsat=7 time=0.454s
    varisat incr calls=3000 sat=2993 unsat=7 time=0.639s
    batsat-wasm incr sat=2993 unsat=7 time=0.782s

(d) Six instances of the SAT Competition 2025 main track, fetched from
the GBD (benchmark-database.de/file/HASH) by the hashes splr's README
lists, where splr 0.19.0 reports 12-34 s each: 0e1d... Break_12_50,
24bd... SC25_Timetable_C_4 (292 405 variables, 937 503 clauses), 53c2...
maximum_constrained, a1fd... oddball_22_5_ttf, a708... gm28sparrc, b3d3...
case20. Timeout 100 s, one core. CaDiCaL here is the old 1.9.5 of the
`cadical` crate, not the SC2025 version.

| instance | cadical | batsat | varisat |
|---|---|---|---|
| 0e1d562093d5f4fc9013cf4a14a03f70.cnf | SAT 1.641 | SAT 10.167 | SAT 67.417 |
| 24bde22f729a988fb2394b644cb60d39.cnf | timeout | timeout | timeout |
| 53c21f3e78f060883026b5a12ba691d8.cnf | SAT 10.746 | SAT 9.946 | SAT 7.993 |
| a1fdd60d2570f47fb14956ac9e96951f.cnf | UNSAT 56.100 | timeout | UNSAT 67.499 |
| a70883771fd1c210d94a916d52510a3a.cnf | UNSAT 0.914 | UNSAT 0.713 | UNSAT 0.101 |
| b3d3680b3287a989ce61a6db1054efd2.cnf | SAT 39.820 | timeout | timeout |

- instances solved by all: 3
- cadical: total on those = 13.30 s, solved = 5
- batsat: total on those = 20.83 s, solved = 3
- varisat: total on those = 75.51 s, solved = 4

Reading of (a)-(d): on small and easy instances batsat is as fast as
CaDiCaL 1.9.5, sometimes faster (CaDiCaL's preprocessing costs time
there). On medium random 3-SAT batsat needs about 1.5x CaDiCaL's time.
On competition instances the gap opens: CaDiCaL solved 5 of 6 in 100 s,
varisat 4, batsat 3, and on one solved by all three batsat took 6x
CaDiCaL's time and varisat 41x. Under Node, batsat ran 1.1x to 1.4x its
native time on non-trivial instances (17% more over set (a)); start-up
makes the tiny ones look worse. A modern CaDiCaL or Kissat would widen
the gap on hard instances further; I did not measure that.

## 4. Recommendation for deduxit

### 4.1 Intuitionistic engine: `batsat` 0.6.0, directly

Use batsat itself, not rustsat-batsat, behind a small trait of deduxit's
own (new variable, add clause, solve under assumptions, value, failed
assumptions, stop), in the `deduxit` crate only, behind the feature of
the engine that uses it.

Reasons:
- It is the API intuit and intuitR were built on: both use MiniSat
  through Haskell's `minisat` (section 3.1), and batsat is MiniSat 2.2 in
  Rust with the same calls (`solve_limited(assumps)`, `add_clause_reuse`
  between calls, `value_lit`, `unsat_core`).
- Pure Rust with one dependency (`bit-vec` 0.5.1, MIT/Apache-2.0, from
  2019; current is 0.10.1). Builds for wasm32-unknown-unknown and runs
  there with no imports, so no clock, no JavaScript glue, no threads
  (verified).
- Stop and work bounds: `Callbacks::stop` is polled at every decision;
  a custom `Callbacks` can count conflicts (`on_new_clause`) for a budget
  and sum clause sizes for deduxit's memory account (also
  `on_progress`'s `ProgressStatus` reports clause and learnt literal
  counts).
- MIT, allowed by deny.toml.
- Speed: the MiniSat class. Against CaDiCaL 1.9.5 it took 0.7x of the
  time on easy structured instances (3.54 s against 5.02 s over set (b)),
  1.5x on medium random 3-SAT (52.3 s against 34.9 s over set (a)), 1.5x
  on the intuit-shaped incremental run (0.684 s against 0.454 s for 3000
  calls), and on competition instances it solved 3 of 6 in 100 s where
  CaDiCaL solved 5. Under Node it ran about 17% slower than native over
  set (a) (61.4 s against 52.3 s) and 14% on (c) (0.782 s against
  0.684 s). I expect the problems deduxit decides (ILTP-sized formulas,
  many small incremental calls) to sit where the gap is small; I did not
  measure deduxit's own inputs.
- Its answers need not be trusted: an intuit-style engine ends in a proof
  or a Kripke countermodel, which deduxit can check itself.

Risks:
- Maintenance is thin: one release in five years (0.5.0 2021-01-12, 0.6.0
  2025-01-20), five 2024 pull requests unmerged, last commits 2026-05-05
  (dependency and CI updates). Mitigation: it is about 4 900 lines of
  MIT code; vendoring or a fork is cheap if it stalls or a bug shows.
- Incremental correctness: an incremental bug (#7) was fixed with a
  regression test; I did not audit the rest. deduxit's checking of the
  final answer covers this.
- No LRAT (see 4.2).
- `bit-vec` 0.5 is old; if deduxit or another dependency pulls a newer
  `bit-vec`, `cargo deny` may report a duplicate.

Not rustsat-batsat: it adds a clock read to every solve call
(`cpu_time::ThreadTime` natively; on wasm the module imports
`performance` and `now` through wasm-bindgen, verified by listing the
imports of a cdylib built with it), exposes no stop or limits for batsat,
pulls anyhow/nom/itertools/thiserror/tempfile, and its solver API is about
to be redesigned (#565). rustsat is the right choice only if deduxit wants
one API over several C/C++ solvers on native targets.

### 4.2 Classical engine: the same `batsat` and a checker of deduxit's own

- Encode the negation (Tseitin) with deduxit's own encoder, solve with
  batsat, and record a DRUP proof in memory from `on_new_clause`
  (`Kind::Learnt`) and `on_delete_clause`. Check it with deduxit's own
  RUP checker against the CNF deduxit produced; the encoder and the
  checker are the trusted parts. A SAT answer is a countermodel, checked
  by evaluation.
- LRAT is preferable for checking speed. batsat gives no clause ids or
  antecedents. Two ways to get them: (1) the checker computes the hints
  itself by forward unit propagation (simple; costs about as much as
  solving the proof's clauses again), or (2) a fork of batsat logs, in
  `analyze`, the reason clauses used (plus the level-0 units whose
  literals it drops), which is what LRAT-producing solvers do; CaDiCaL's
  experience (section 2.2) puts the cost of producing LRAT at about 5%.
  Since MiniSat-style learning only derives RUP clauses, the checker needs
  RUP steps only (LRUP), no RAT.
- Optional native tier, later: CaDiCaL (MIT, the top UNSAT solver of the
  2025 competition by the slides) behind a feature that is never on for
  wasm, preferably through `cadical-sys` 0.8.0 (CaDiCaL 3.0.1, cc and cxx,
  no libclang; `connect_proof_tracer` gives clause ids and antecedents in
  memory, `connect_terminator` the stop, `limit` the budgets). Risks:
  a C++ build in the flake and the wasm split; `cadical-sys` has one
  maintainer and a short history (first release 2024-10-30). The
  alternative binding is `rustsat-cadical` (CaDiCaL 2.2.1, needs libclang
  at build time, same tracer API, clock read per call as above).

### 4.3 Fallbacks

1. `varisat` 0.2.2 (MIT OR Apache-2.0, pure Rust, wasm builds): it has
   assumptions, failed cores, and in-process checked proofs with hints
   (LRAT through `varisat-lrat`), and was the fastest on set (b) and the
   slowest on set (a). It has no stop or budget at all, so deduxit would
   have to fork it (the author planned an `ipasir_set_terminate`
   equivalent in 2020); no release since 2020-09-09.
2. A CDCL of deduxit's own (two watched literals, VSIDS or VMTF, 1-UIP
   learning with antecedent ids, restarts, assumptions): a few thousand
   lines, LRAT by construction, fully under deduxit's rules (work bounds,
   memory account, no clock). batsat's MIT source is a reference for it.
3. Not fallbacks: splr (MPL-2.0, no incremental mode), Kissat (not
   incremental), screwsat (no assumptions, reads the clock in `solve`),
   the 2025-2026 newcomers (no track record).

## 5. What I could not verify

- No published numeric comparison of a pure-Rust solver with CaDiCaL or
  Kissat; the numbers above are my own small runs on one machine (core 4
  of this host), with `cadical` 1.9.5 rather than CaDiCaL 2.x/3.x.
- Whether CaDiCaL or Kissat built with Emscripten can be linked into a
  wasm-bindgen (wasm32-unknown-unknown) module like deduxit-web.
- The MiniSat version inside Hackage's `minisat` 0.1.4.
- The contents of `lrat-check.c` in drat-trim, and the LRAT paper's full
  text (its abstract and section titles only, via search).
- The 2025 SAT Competition rule text itself (only its slides, read, and
  the 2024 cake_lpr document, via search).
- batsat's benchmark page (HTTP 500).
- Whether the Coq LRAT checker of the CADE 2017 paper still builds with
  current Rocq, and its licence.
- That batsat's callback stream (`on_new_clause` for learnt clauses and
  final conflicts, `on_delete_clause` from `reduce_db`) is a complete DRUP
  proof: read from `core.rs`, not tested against a DRAT checker. Clauses
  shortened at level 0 when added are not logged, which RUP checking
  tolerates; satisfied clauses removed by `simplify` are not logged as
  deletions, which only costs checking time.
- Whether `cadical-sys` 0.8.0 builds in this flake (I built only the
  `cadical` crate natively).
