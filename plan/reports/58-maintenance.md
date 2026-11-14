# Step 58: maintenance, and Kani on the checker's integers

The step's two sessions each add a section here (`plan/58-maintenance.md`,
"How the step runs"): 58a, the repository's policies, Kani and the wasm
checks; 58b, the web client's accessibility audit.

## Session 58a: policies, Kani and the wasm checks

Session 1 of `plan/58-maintenance.md` (Opus 5.5 at `high`, 2026-11-14,
cores 7-9), in the jj workspace `../deduxit-wt-step-58a-610ed3`.

Kani 0.68.0 is built from source in the flake and checks eleven integer
facts of the checker, each verified here; Clippy runs for wasm32 on the
library and the bindings, and the library's unit suite runs on 32 bits
but nine tests; the module is held to a feature baseline and a budget;
the MSRV, the update cadence and the advisory check are written down.
Nothing is pushed.

| commit | what |
|---|---|
| `rrswyoyo` | the zone arithmetic of the pass in functions of its own, unchanged |
| `szkqtxqo` | Kani (`modules/kani.nix`, the input `kani`), the harnesses, the check `kani` |
| `oxyqsnum` | Clippy for wasm32 (`wasm`, `web-clippy-wasm32`), the unit suite on 32 bits (`web-core-wasm32`) |
| `soupwyvk` | the feature baseline (`web-features`), the budget (`budget`), the wasm-bindgen policy |
| `qroykluv` | the MSRV, the cadence, the advisory check of the fuzz lock |
| (this one) | this report |

### The policies, and where each is written

- **The MSRV** (`rust-version`, 1.92): the oldest stable that builds
  every crate with every feature, held by the `msrv` check; it rises only
  with a new 0.y version, to what a dependency or the code needs, and
  never to a Rust less than six months old. A raise past a
  distribution's Rust that the installation chapter lists as enough
  moves that distribution to the other list there, with `install.yml`.
  Resolver 3 (the workspace's, and the default of edition 2024) prefers
  dependency versions that build on the MSRV
  (`incompatible-rust-versions = "fallback"`, read in the Cargo
  reference on 2026-11-14): a crate it holds back shows in
  `update-deps`' step 1; where no version that builds fits, it takes a
  newer one and the `msrv` check fails, and that crate is held back by
  hand until the raise. Written in the
  `update-deps` skill ("When"), `flake.md` (the `msrv` bullet), the
  comment on `rust-version` in `Cargo.toml`, and for users one clause of
  the installation chapter ("never to a Rust less than six months
  old").
- **The update cadence**: `update-deps` once per Rust stable release
  (six weeks), in the week after it, and before every release of the
  crates; the planning session schedules it. Kani moves by its tag in
  `flake.nix`, when nixpkgs has the CBMC it names. Written in the skill.
- **The advisory check between runs**: CI's weekly run (already in
  `ci.yml`) now audits the fuzz workspace's own lock too
  (`cargo deny --manifest-path fuzz/Cargo.toml check advisories`, run
  here and passing; the root's `deny.toml` serves it, its two ignores
  matching nothing there). An advisory against a locked crate is met at
  once (`cargo update -p`, or an ignore with its reason where no fix
  exists), not at the next run. Written in the skill and `ci.md`. The
  workflow change is checked by actionlint in `nix flake check` and has
  not run on GitHub.
- **The wasm feature baseline**: the module may use what Rust enables
  for wasm32 by default since 1.87 (LLVM's `generic` CPU: mutable
  globals, sign extension, non-trapping float-to-int, multi-value,
  reference types, bulk memory; the rustc book's wasm32 page, read
  2026-11-14). The check `web-features` validates the package's module
  against exactly those with nixpkgs' `wasm-tools` 1.263.0, and fails on
  plain MVP (tried: "func type returns multiple values but the
  multi-value feature is not enabled"), so it discriminates. Taking
  another feature is a decision recorded in `web.md`.
- **The wasm-bindgen policy**: the crate stays pinned with `=` and moves
  only at an `update-deps` run whose nixpkgs has the matching CLI, the
  test crate with it, never ahead of nixpkgs; the client's update follows
  the push. Written in `web.md` (the pin and `web-bindgen` were there
  already).
- **The budget** (`modules/budget.nix`, the check `budget`): Cargo.lock's
  packages at most 180 (173 now) and the module at most 3 000 000 bytes
  at `gzip -9` (2 842 389 now). Step 47 measured the module at 0.2.0 at
  6 913 020 bytes raw, 2 603 759 at gzip and 1 854 124 at brotli; now it
  is 7 578 139, 2 842 389 and 2 016 466 (+9.6 %, +9.2 %, +8.8 %: the
  ordinary engines and the lambda-terms since). A change that passes a
  ceiling raises it in a commit that says what earned the room. Written
  in the module and `web.md`.

### Kani

Adopted through the `new-tool` skill's steps: Kani 0.68.0 (the latest
release, 2026-09-16), nixpkgs has none, so `modules/kani.nix` builds it
from the input `kani` (`github:model-checking/kani/kani-0.68.0`,
`flake = false`, locked to `0d2328a9`) with its own nightly
(`nightly-2026-08-21`, from its `rust-toolchain.toml` through
rust-overlay) and nixpkgs' CBMC 6.11.0, the version its
`kani-dependencies` names (asserted at evaluation). The build is
`build-dev --release`, the install the release bundle's layout, with
wrappers `kani` and `cargo-kani`. What it took that the community flake
(`fd/nix-kani`, Kani 0.45, 2024) did differently: crane's
`vendorMultipleCargoDeps` over Kani's lock and the standard library's
(no fixed-output derivation of a cargo home), and the `charon`
submodule's optional backend dropped from kani-compiler's manifest (the
tarball lacks it). Its build phase took 1 min 26 s on three cores here;
`flake.md` has the details. Kani is in no devshell (every session loads that one):
`nix shell .#kani -c cargo kani -p deduxit --no-default-features`.

The harnesses (`core/src/linear/proofs/check/harnesses.rs`,
`#[cfg(kani)] mod harnesses` in `check/mod.rs`; `cfg(kani)` declared to
the `unexpected_cfgs` lint in the workspace's lints), each with the fact
it checks, the bound, and its time and peak memory alone on one core:

| harness | the fact | bound | time, peak |
|---|---|---|---|
| `table_bytes_stay_below_two_to_the_38` | a table's bytes never overflow, stay below 2³⁸ and hold its entries, for the three entries the checker keeps | every `most` up to 2³² | 6 s, 0.7 GB |
| `a_sequent_takes_fewer_than_two_to_the_39_bytes` | a sequent's bytes stay below 2³⁹, an eigenvariable set's below 2³⁸ plus its box | every `most` of each table up to 2³² | 8 s, 0.8 GB |
| `the_zone_bound_is_capped_below_a_saturated_count` | `room` is `goal + 2·later` capped at `Bag::MOST`, never overflows, and `Bag::MOST < u32::MAX` | every `goal` and `later` | 5 s, 0.7 GB |
| `a_copy_put_is_counted_or_refused` | a put keeps counts, outputs and both weights exact, or the zone is over `Bag::MOST` (where `within` refuses it); it never shortens a zone | a zone of one member, put again; any count, weight, position | 239 s, 5.7 GB |
| `a_new_member_put_is_counted_or_refused` | the same for a member the zone lacks | a zone of one member, another put | 238 s, 5.5 GB |
| `a_copy_taken_is_counted` | a take from a zone `within` let through subtracts without going below zero and keeps everything exact | a zone of one member | 161 s, 3.3 GB |
| `a_member_missing_is_not_taken` | taking a member the zone lacks changes nothing | a zone of one member, another taken | 142 s, 3.2 GB |
| `the_sum_of_two_zones_is_counted_or_refused` | `⊗`, Mix and the cut's sum keeps counts and sums exact or is over `Bag::MOST`; it absorbs when either does | two zones of the same member | 246 s, 5.9 GB |
| `the_sum_of_two_zones_apart_is_counted_or_refused` | the same, adding a member to the table | two zones of different members | 263 s, 5.8 GB |
| `the_union_of_two_zones_is_counted_or_refused` | `&`'s union with the sums counted anew (a count times a weight fits a `u64`) is exact or over `Bag::MOST` | two zones of the same member | 420 s, 8.0 GB |
| `a_kept_proof_is_ordered_and_reached` | a proof `Proof::new` keeps has its premises before their nodes, the root last, every other node read by a later one, so node `i` of `n` has at most `n − i` readers | the arena `1`, Mix, Mix with any premises up to one past the end, the root last | 37 s, 1.4 GB |

Every value in a bound ranges over its whole type (counts and weights
over `u32`, the sums over `u64`), and Kani fails a harness on any
overflow, underflow or panic on the way, besides its assertions. The
times are one harness alone on one of the cores 7–9, compile included,
each in its own 8 GB scope, on the code before the rebase onto `main`
and before `State::remove` took a closure (below); three at once in one
8 GB scope ran out of memory. The flake check `kani` ran all eleven
again on the final head (Checks).

To reach the code that runs, the pass's arithmetic moved into functions
the harnesses call, with no change of behaviour: `State::put`,
`State::remove`, `State::add_linear` and `State::recount` (from the
pass's `put`, `take`, `join` and `recount`), and `pass::room` (from
`within`). The checker's tests, the gate and the 32-bit run pass on
them.

What made the harnesses tractable, found by trying (`core-proofs.md`
keeps it): with a member's id symbolic, one harness ran past ten
minutes and 4 GB (the standard library's table hashes and probes a
symbolic key), and one insertion and lookup of a constant key took 77 s
with Kani's default checks, 35 s without its memory-safety checks and
14 s at an unwind bound of 5; a harness that compared sums of products
computed two ways ran past ten minutes again, since a SAT solver cannot
match two multiplications; a bound "weight ≤ members × (2³² − 1)" took a
join 16 minutes, and "weight ≤ members · 2³²", a shift, 4; the arena
harness ran out of 8 GB first on `expect` and the drop of the crate's
`Error` (both walk every variant), then on a free root (every table
`Proof::reach` keeps gets a free length). So the ids are constants, the
sums are checked incrementally (a put adds the member's weight, a take
subtracts it, a join adds the two sums), a sequent's weights are any
values with the facts exact sums have (each member held weighs in, a
weight at most its members times 2³², the goal's part within the
whole), the error is forgotten, and the root is the last node. The
memory-safety checks are off for the crate (`[workspace.metadata.kani]` in `Cargo.toml`): the
crate forbids `unsafe`, and they only re-check the standard library's
table; Kani still fails an overflowing add and an underflowing
subtraction with them off (tried on a scratch crate).

The check `kani` runs `cargo kani` on the core crate alone, two
harnesses at a time: one takes up to 8 GB, and CI's runner has 16. The
eleven take about 30 minutes of CPU, so the check is not in the gate;
CLAUDE.md's verification table asks for it when the checker's integers
change.

### The wasm checks

- **Clippy for `wasm32-unknown-unknown`**: the `wasm` check now runs
  Clippy (warnings denied) on the library and its tests with every
  feature but `parallel`, and `web-clippy-wasm32` on the bindings and
  theirs. Five sites fired on the library, each right on both widths:
  three `unnecessary_box_returns` in the pass (`just`, `join`, `over`: a
  `State` is small on 32 bits), `absurd_extreme_comparisons` in
  `ordinary::Formulas::add` (`MOST` is `usize::MAX` there), and the
  `expect(trivially_copy_pass_by_ref)` on `search`'s `is_zero` went
  unfulfilled (a `u64` by reference is not trivially copied on 32 bits).
  Each takes a `cfg_attr` on `target_pointer_width`. The bindings were
  clean. `later.md`'s entry named four sites; the fifth is newer.
- **The unit suite on 32 bits**: `web-core-wasm32` runs the library's
  whole unit suite on `wasm32-wasip1` (it ran the checker's, the size
  estimate's, the memo's and the memory account's tests): 403 of 414
  pass in about three minutes on one core in a debug build, run here
  each alone and then together. Nine are left out with their reasons:
  seven start a thread (WASI has none: the tests of a small stack, the
  batch's panic, two of the focused engine's stops), and two pin byte
  counts of 64-bit layouts (`horn::tests::charges_what_it_grows`, whose
  least bytes for a search are less on 32 bits, and
  `search::tests::a_check_past_the_memory_bound_is_unknown`, where on 32
  bits the proof's check fits at the first bound the search does). The
  behaviour there is right; the figures are not width-free. The other
  two that failed at first read files by the path cargo compiled in, and
  pass with the source tree preopened, which the check now does.

### Checks

All on the chain rebased onto `main@origin` (`zvxmpnzk`, 54s-b2's
report), which rebased without a conflict.

- **The gate** passed (`systemd-run … taskset -c 7-9 nix develop -c
  gate`, 9 min 50 s): Clippy, the tests, both `cargo hack` runs, the
  ratchet (every journey within 2 %; `check-qbf-20-2` +0.68 %,
  `derivation-chain-64` +1.94 %, see below). The first gate, before the
  closure in `State::remove`, failed on the ratchet.
- **`nix flake check`** (under the machine lock, `--max-jobs 2 --cores
  4`, 18 min) passed every check but `kani`, which failed at once on a
  flag Kani needs beside `--jobs` (`--output-format terse`); with it,
  `nix build .#checks.x86_64-linux.kani` (and `treefmt`, `typos`,
  `conventions` for the files that changed) passed: "Complete - 11
  successfully verified harnesses, 0 failures, 11 total", 13 min 40 s.
- **The new wasm checks** built and passed before the rebase too
  (`wasm`, `web-clippy-wasm32`, `web-core-wasm32` with 403 passed and 9
  filtered out in 192 s, `web-features`, `budget`), and again in the
  flake check after it.
- **Native Clippy** on the workspace with every target, warnings denied:
  clean. `cargo hack` beyond the gate's: no feature changed.
- **The cargo-deny run on the fuzz lock**: `cargo deny --manifest-path
  fuzz/Cargo.toml check advisories`, "advisories ok".
- **Kani itself** on a scratch crate: proves a saturating add, fails a
  plain add's overflow and a subtraction's underflow, with and without
  the memory-safety checks.

### The reviewer's findings

The `fresh-reviewer` agent (Fable 5.2 at `medium`, read-only, from the
five code commits, the brief and the prompt) found nothing against the
harnesses (each states a fact the checker relies on, runs the code the
pass runs, and says its bound truthfully; no assumption excludes a
reachable state an assertion depends on; no assertion cannot fail) or
the refactoring (line for line the removed code, the one difference
being the order in which `take` asked the observer, both pure). It found
five false or inconsistent sentences, each corrected in the commit that
wrote it:

1. `modules/web.nix`: the comment on the skipped tests named "the stop's
   timers"; the seven tests start a search thread or a batch's workers.
   Corrected.
2. `core-proofs.md`: "up to five minutes and 6 GB" against the measured
   seven minutes and 8 GB elsewhere. Corrected.
3. The skill's MSRV bullet said "the newest minus four", which is about
   5.5 months, against "never to a Rust less than six months old"
   everywhere else. Corrected to the latter.
4. The skill and `flake.md` said resolver 3 keeps `cargo update` below
   the MSRV; `fallback` is a preference, and takes a newer version where
   none that builds fits. Corrected: the skill says so and that such a
   crate is held back by `cargo update --precise` until the raise.
5. `flake.md`'s `web.nix` bullet kept the old description of
   `web-core-wasm32` beside the new one and lacked the two new checks in
   its list. Corrected.

### Decisions and departures

- **Kani built from source in the flake (departs from the design)**:
  the prompt allowed a flake check "or a CI job ... as the tool's
  toolchain allows"; nixpkgs has no Kani and Kani's own installer
  downloads a bundle and a rustup toolchain, which a check cannot. The
  build follows its `build-dev` with patches for the two rustup paths
  and the missing submodule.
- **Memory-safety checks off (departs from the design)**: Kani's
  default checks would re-check hashbrown's unsafe code in every
  harness at twice the time; the crate forbids `unsafe`.
- **The pass's arithmetic moved into functions** so the harnesses check
  the code that runs, not a copy; a refactoring of the checker without a
  change of behaviour (the checker's tests and the gate).
- **The zone harnesses are bounded by shape (departs from the design)**:
  zones of one member with fixed ids; the counts, weights and positions
  range over their whole types. Left out, each past ten minutes of CBMC
  on one core: zones of two members, the union of two different members
  (21 minutes before it was stopped), and the swap `Bag::add` makes
  before pouring the smaller table into the larger (it only chooses the
  table that is iterated). The arena harness fixes the kinds (`1`, Mix,
  Mix) and the root (last); a free root ran out of 8 GB.
- **One line in `core/src/ordinary/mod.rs`** (54s-b2's area): the
  `cfg_attr` on `Formulas::add`, which Clippy for wasm32 needs.

### What the ratchet found

- **The refactoring first cost the checker 1.4 % in instructions**, which
  the gate's ratchet caught (`check-qbf-20-2` +2.15 % of its ceiling).
  Counted against `main` on the same machine (a temporary workspace,
  removed): 137 699 762 instructions at `main`, 139 681 739 at the head.
  callgrind put it in `State::remove` (18.1 M against main's
  `Bag::remove`, 16.2 M), which asked the observer for the weight and
  the position before it knew whether the member was there, and was not
  inlined. With the question in a closure asked only after a copy was
  taken, and `#[inline]` on `put`, `remove` and `add_linear`, the head
  counts 137 671 520 (`check-wide-m1-2048` 8 586 626 against 8 588 694,
  `derivation-chain-64` equal). The two take harnesses were run again
  on the closure.
- **`derivation-chain-64` stands at +1.94 % of its ceiling at `main`
  already** (9 890 260 at the head, 9 892 258 at `main`): drift from
  before this session, close to the 2 % tolerance.

### What the next sessions must know

- **The planning session**: `later.md`'s row "Clippy for wasm32; the
  unit suite on 32 bits" is done here (five sites, not four; nine tests
  left out, with reasons); and `update-deps` is due once per Rust
  release now, which the handoff's schedule should carry.
- **A change of the checker's arithmetic goes into the functions the
  harnesses call** (`State::put`, `remove`, `add_linear`, `recount`,
  `pass::room`, `table_bytes`); arithmetic written inline in the pass is
  checked by no harness. The flake check `kani` costs about 15 minutes
  of wall time and up to 16 GB at two at a time whenever `core/` changes;
  if CI's time matters more, it can move to the weekly run, which is a
  change of `ci.yml` and `flake.md` alone.
- **What Kani does not reach here**: zones of two members, the union of
  two different members, the swap before a sum or a union, arenas of
  more than three nodes, and every fact that needs more than one rule
  (the pass as a whole). The facts it checks are also the specifications
  a deductive verifier would start from (D31's later trial of Verus and
  Creusot).
- **Two unit tests pin 64-bit sizes** (`horn::tests::charges_what_it_grows`,
  `search::tests::a_check_past_the_memory_bound_is_unknown`): the step
  that next touches either can make its figures width-free and take it
  off `web.nix`'s list.
- **The module is at 2 842 389 bytes at gzip of a budget of 3 000 000**
  (5.5 % room); the PNG and PDF renderers' crates are two fifths of it
  (step 47). The first suite that is not an embedding is what will move
  it.
- **Nothing of `.github/` has run**: the fuzz lock's advisory step is
  checked by actionlint only; its first weekly run is the author's.
- **Kani moves by its tag** (`flake.nix`), when nixpkgs has the CBMC its
  `kani-dependencies` names; the skill says so. Kani's nightly warns
  that a pattern at `core/src/errors/mod.rs:964` will become a hard
  error in a future release (a future-incompatibility lint of the
  nightly, not of the pinned stable); the step that next moves the
  toolchain meets it.
