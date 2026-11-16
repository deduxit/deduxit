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

## Session 58b: the accessibility audit

The client's accessibility audit of `plan/58-maintenance.md`
(maintenance-4, web-teaching-4), Opus 5.5 at `high`, 2026-11-14, in the
client's jj workspace `../deduxit.github.io-58b` (workspace `step58b`) on
the client's `main@origin` (`runpsyxs`). Nothing was pushed. This
deduxit workspace holds only this section.

### Outcome

axe (`@axe-core/playwright` 4.14.0 over axe-core 4.14.0, MPL-2.0, a
dev-dependency only, past the fourteen-day gate) now runs as e2e tests
on every page of the client and in every state of the prover's walk, in
the light and the dark scheme, at a desktop's (1280 × 800) and a phone's
(Pixel 7) width: 48 tests, WCAG 2.0, 2.1 and 2.2 at A and AA plus axe's
best practices, failing on any violation. The first run found 38 of 40
tests failing on ten rules; every finding is fixed but one, which is
listed below with its reason. The screen-reader pass found the drawings'
text alternatives complete, fixed one naming inconsistency in the
client, and leaves two proposals; no structural twin was built.

### Tools and pages audited

- **Tool**: axe-core through `@axe-core/playwright` in the client's
  Playwright (nixpkgs' Chromium), `tests/accessibility.spec.ts`. Tags
  `wcag2a`, `wcag2aa`, `wcag21a`, `wcag21aa`, `wcag22a`, `wcag22aa`,
  `best-practice`. Each violation is a line with axe's own reason (the
  ratio, the covering element), so a failure says what to fix.
  Lighthouse was not run: its accessibility audit is axe's rules.
- **Pages**: the prover (`/`), `/privacy/`, the 404 page, and the Rocq
  library's old address `/rocq-linlog/` (its meta refresh taken out by a
  route, so that the page stays to be read). `/llms.txt` and
  `/problems.json` are not HTML.
- **States**: every state of the walk that `tests/targets.spec.ts`
  measured, now one module, `tests/states.ts`, which both specs walk: the
  empty page, a parse error, a proof with the rules popover, the split
  chooser, a refusal, a hint, a proof closed, its net; every menu of the
  band (Export, Steps, Import, Settings, Keys, About), the status, the
  calculus and the restart warning; Practice and its filters; the tour;
  Learn's Tutorial and Reference and the lesson states (worked proof,
  dead end, faded proof, check, attempt, wrong step, unknown,
  elimination, refusal); the restore bar; a goal's reasons (lost goal,
  long reason); Learn's answers; an ordinary tab; the witness chooser;
  the cut chooser and the elimination; the stage in each math font.
  Twelve walks, each a test of its own per scheme and width.
- **Incomplete results** (what axe cannot decide) are not asserted. A
  probe of five states in both schemes found them all of one rule,
  `color-contrast`: text over a gradient (the stage's dotted field, and
  now every keyword's wash), under an overlapping element, an image, or
  symbols alone. `colours.test.ts` holds the keywords instead.

### Findings and what was done

1. **Label in name (SC 2.5.3, A)**, `label-content-name-mismatch`, the
   most frequent finding (every state with a tab, every lesson button,
   every Practice row, Watch). Fixed in `omrrppxn`:
   - The tab's calculus mark showed "Linear (LL) MLL" and was named
     "Logic: Classical; fragment MLL". Its name is now "Logic:", then
     what it shows at that width, then the switches' words ("Logic:
     Linear (LL) MLL, Classical; fragment MLL"), by `aria-labelledby`
     over its own label and fragment and two `hidden` spans beside it.
     An `aria-label` could not do it: axe drops what stands in
     parentheses, so the phone's "LL" was missing from "Linear (LL)".
     Spans inside the button would have put screen-reader words into its
     `textContent`, which a dozen tests read with `toContainText` (some
     would then pass on those words alone). This replaces 54u-c's "the
     accessible name stays Logic: …" (W9), keeping its "Logic" first.
   - Lesson buttons: the name gains the line shown under the title
     ("Lesson 2: The multiplicatives. A worked proof, your turn,
     practice, a check"); Practice's rows gain the fragment shown; Watch
     is "Watch the proof of …", as it shows "Watch the proof".
   - axe compares words and joins an element's text nodes without a
     space between elements ("1Reading", "LLMLL"): the parts of these
     buttons now have a space between them in the markup, which their
     flex and grid layouts ignore.
2. **ARIA parents and children of the Export menu**
   (`aria-required-parent`, `aria-required-children`, `list`, A): the
   menu items sat in `ul`/`li`, so their parent was a list item, not the
   menu. The lists are `role="none"` (`toxvrvvo`); the separators stay
   `aria-hidden` list items (Biome refuses `role="separator"` on an
   `li`, and an `hr` is no child of a `ul`).
3. **Contrast (SC 1.4.3, AA)**: a `.kw.bad` keyword on Learn's frame was
   4.29:1 in both schemes (its wash over the frame's tint). Recolouring
   `--color-bad` could not reach 4.5:1 without breaking
   `colours.test.ts`'s colour-vision distances (to warn under
   deuteranopia and to the dominator colour). Instead every keyword's
   wash is laid on the panel whatever stands under it (`wtzmlttl`), as
   the stage's banner already did; `colours.test.ts` gains a test that
   each tone reads at 4.5:1 on its wash over the panel (lowest: accent
   in dark, 4.51). The keyword chips look the same on the panel and a
   shade lighter on the field and in Learn.
4. **Links in text (SC 1.4.1, A)**, `link-in-text-block`: the privacy
   and 404 pages' links were told from the text by colour alone (2.48:1
   and 2.8:1 against the text). Underlined (`wtzmlttl`), the word mark
   excepted.
5. **The Rocq library's old address** (best practice): no `main`, no
   `h1`. Given both (`qnxxqwtz`).
6. **Target size on a phone, the stage's buttons misplaced (SC 2.5.8,
   AA)**: in lesson 7's search that gives up, on a phone with Learn open,
   the dock's buttons were half covered by the stage's view controls and
   the band. Two causes, both fixed in `truzxtpr`:
   - The formula buttons lay 30 to 90 pixels off their formulas (a
     516-pixel button over a 120-pixel formula): the effect that lays
     them again after a zoom runs before the world's transform follows
     the camera, so boxes were read at the old scale and divided by the
     new. `rendered()` measures the scale actually shown (the drawing's
     box over its layout width) for every box read off the drawing
     (formulas, rule labels, cuts, engine labels); the sizes a button
     must reach on the screen still come from the camera. This is a real
     bug beyond the audit: on that screen the formulas could not be
     clicked.
   - The dock, placed below the goal for want of room above, ran under
     the view controls. Where it fits neither above nor below without
     that, the goal now moves down until the dock fits above it, if the
     goal then still clears the controls, and only while the camera
     stands where the page put it: after the visitor's own drag, wheel,
     zoom or pinch the dock never moves the drawing (`rykuknrs`, the
     review's finding 1).
   - `phone.spec.ts` checks both in that state. Run against the old
     stage it fails on the dock; the misplaced buttons are a race, which
     that run did not hit (the axe runs and a probe did, every time).
7. **Target size of the rule labels on a phone (SC 2.5.8), listed, not
   fixed.** A rule label's button over a drawing fitted to a phone is 44
   pixels square and overlaps its neighbours (an elimination's
   derivation, lesson 9 and 10's drawings, a proof under More's sheet).
   For a linear rule, what it opens, the rule's hint, is also in Learn's
   Reference, whose controls meet the criterion: SC 2.5.8's "equivalent"
   exception. For the rules the Reference lacks (LK's and LJ's, the
   Lambek calculus's, Mix, weakening) it is a violation that remains
   (the review's finding 2). A screen reader hears the hint as the
   label's description in every case. The spec filters exactly these
   nodes (`excepted`: rule `target-size`, a `button` of class `lb`) and
   says which part is an exception and which a violation waiting for a
   decision. A fix is a proposal below.
8. **The phone's walk of a proof** ran past the dev shell's 30 seconds
   once with axe in every state: `proving` is split into `proving` and
   `menus` (`rkqlzsls`, `mqpvyvqn`).

Two test fixes came with the runs: `ordinary.spec.ts`'s link test read
the URL before the step's link was written (2 of 6 failures on `main`'s
stage, 5 of 6 with the new one): fixed at its cause as 54u-d fixed its
twin in `prover.spec.ts` (`xswszwoz`, 6 of 6 after). Practice's,
Learn's, lessons' and the chip's tests name the controls by their new
names (`omrrppxn`), the cut test by its new name (`rswnysrn`).

### The screen-reader pass

No screen reader runs headless on this machine, so the pass read what
Chromium gives one: the accessibility tree (CDP
`Accessibility.getFullAXTree`) and the stage's ARIA snapshot, for a
proof begun, a step taken, a proof closed and its net, an intuitionistic
proof and its essential net, a MELL proof and its net with a box, an
ordinary proof, a refuted sequent and a cut elimination. What it found:

- **Every drawing is one image, named by its sequent, described by its
  steps.** deduxit's SVG keeps `role="img"`, its `<title>` the name
  ("⊢ A⊥ ⅋ B⊥, B ⊗ A"), its `<desc>` the description: a derivation's
  numbered inferences, premises first ("1. ⊢ ~B, B, by ax. … 4. ⊢ ~A ⅋
  ~B, B ⊗ A, by ⅋ from 3."), an open goal as "open", a net's links in
  its text form. The image's children are presentational, so a reader
  does not spell out the italic letters. After the image come the
  formulas' buttons ("A⊥ ⅋ B⊥, formula 1 of goal 1") and the rule
  labels' buttons ("Rule ⊗", the rule's hint as description), then the
  dock. That is a complete, ordered alternative.
- **Countermodels are text**: a classical countermodel and every other
  refutation is the note's sentence, read by the live region; there is
  no drawing of one to describe.
- **Notation differs**: the description writes a negated atom `~A`
  (deduxit's text notation), while the name, the formulas' buttons and
  the screen write `A⊥`. A reader hears "tilde A" in one and "A up
  tack" in the other. The cut buttons were named in `~A` too: fixed in
  the client (`rswnysrn`). The description is deduxit's: a proposal.
- **A structural twin is not needed yet.** The description is a
  complete twin of small and medium proofs, and the Steps list lists
  the steps taken. For a long proof it is one long string with no
  navigation; that is what a twin (a nested list, conclusion, rule,
  premises) would give, and it waits for a screen-reader user's review
  (web-teaching-4: no session can supply one). "Rule ax" twice in a
  drawing does not say which inference: a proposal.

### The checks

| check | result |
|---|---|
| Vitest in the dev shell (`--maxWorkers=3`, cores 13-15) | 587 passed (586 on `main`: the keywords' contrast test is new) |
| `biome lint --error-on-warnings`, `astro check`, `tsc --noEmit` | 0 errors |
| axe spec, first run on `main`'s page (ten walks then, 40 tests) | 38 failed, 2 passed |
| axe spec after the fixes | 40 passed (before the walk's split) |
| the whole e2e suite in the dev shell (`--workers=3`, cores 13-15) | first 199 passed, 5 failed (the phone's proof walk past 30 s, Practice's two tests by the old names, the ordinary link race, `prover.spec.ts`'s link test once); after the split and the renames 207 passed, 1 failed (the ordinary link race), fixed, 6 of 6 |
| the new phone test against `main`'s stage | fails (the dock) |
| `prover.spec.ts`'s link test, repeated (`--repeat-each`) | 23 of 24 passed on this chain, 24 of 24 on `main@origin` (a workspace of its own, since removed); it failed at its broken-link step (`&d=AAAA`, reload, "Proof not restored"), which this chain does not touch, and 54u-c saw it fail once under load: left as a follow-up |
| the client's `nix flake check --keep-going --max-jobs 4 --cores 8`, under the machine lock, at `mqpvyvqn` | exit 0, "all checks passed!": 587 unit tests, 208 e2e tests passed, none flaky |
| after the review: `phone.spec.ts` and the axe spec (`--workers=3`) | 56 passed (48 axe, 8 phone); the new drag check fails on the stage before `rykuknrs` (177.7 against < 137.7) |
| the client's `nix flake check`, as above, at `wptvvnrm`, the chain's last commit | exit 0, "all checks passed!": 587 unit tests, 212 e2e tests passed, none flaky |

### The review

The `fresh-reviewer` agent (Fable 5.2 at `medium`) read the chain up to
`mqpvyvqn` against the brief and the prompt, and drove the built page
with scratch Playwright scripts on cores 13-15 until deduxit-orchestrator
paused browser runs for a timed run; two checks it planned were then
left (below).

1. **The dock's lift undid the visitor's upward drag** (Pixel 7, Learn
   open, lesson 7's search: a 120-pixel drag up ended at the starting
   transform, each move taken back). Upheld: the dock effect ran on
   every camera change. Fixed in `rykuknrs`: `move` records whether the
   visitor moved the camera (drag, wheel, zoom, pinch), and the dock
   moves the drawing only where the page placed it. `phone.spec.ts`
   drags up 50 pixels in that state and checks that the drawing stays
   up. The reviewer also saw the dock trail the goal by one event during
   a drag; whether `main` does the same was not compared (no build of
   `main` was at hand), and it is left as a follow-up.
2. **The exception's reason is false for some labels**: the Reference
   holds no hint of LK's, LJ's, the Lambek calculus's, Mix's or
   weakening's rules, so their labels have no equivalent control.
   Upheld, from the code. The spec's comment, the client's `web.md` and
   finding 7 above now say that those labels remain a violation, waiting
   for the proposal (`wptvvnrm`). The filter itself is unchanged: it
   removes only `target-size` on buttons of class `lb`, which the
   reviewer confirmed.
3. **`web.md` said axe walks every state of `tests/states.ts`**, but the
   font walk was not among the axe walks. Upheld: `wptvvnrm` adds it
   (the stage in each math font, a proof started and its split chooser).

The reviewer found the rest sound: `rendered()` (its fallback at zero
width, the net's paths), the mark's names at both widths and its ids
unique per tab, the menu's roles, the keyword CSS against every rule
that sets a background, every changed assertion (none weakened, two
stricter), and the walk's states and order unchanged in
`targets.spec.ts`.

### The client's commits

On the client's `main@origin` (`runpsyxs`), oldest first, by change id:

| change | what |
|---|---|
| `xswszwoz` | `ordinary.spec.ts`'s link test waits for the step's link |
| `truzxtpr` | the stage's buttons measured at the rendered scale; the dock kept clear of the view controls; the phone test |
| `rswnysrn` | a cut's button named with `A⊥` |
| `omrrppxn` | each control's visible text in its accessible name: the calculus mark, lessons, Practice's rows, Watch; the tests by the new names |
| `toxvrvvo` | the Export menu's lists `role="none"` |
| `wtzmlttl` | keywords' washes on the panel and their contrast test; links in text underlined |
| `qnxxqwtz` | the Rocq library's old address: `main` and `h1` |
| `rkqlzsls` | `tests/states.ts`, the walk `targets.spec.ts` and the axe spec share |
| `mqpvyvqn` | `@axe-core/playwright`, `tests/accessibility.spec.ts`; the client's `web.md` and README |
| `rykuknrs` | the review's finding 1: the dock moves the drawing only where the page placed it; the drag in `phone.spec.ts` |
| `wptvvnrm` | the review's findings 2 and 3: the exception's reason narrowed to the linear rules, the font walk audited |

### Departures from the brief

- One finding is listed rather than fixed: the rule labels' targets on a
  phone (finding 7), filtered in the spec by name and reason; for the
  linear rules it is SC 2.5.8's "equivalent" exception, for the others
  a violation that remains. (departs from the design)
- The tab's calculus mark is no longer named "Logic: …" alone (54u-c's
  W9 kept that name); its name now starts with "Logic:" and holds what
  it shows. (departs from the design)
- Keyword chips no longer let their ground show through their wash: they
  are the panel's colour with the wash on it. (departs from the design)
- On a short stage the dock may move the drawing down to fit above the
  goal (only where the page placed the camera): a change of how the
  stage behaves, made because the dock's buttons were otherwise covered.
  (departs from the design)

### Proposals beyond the audit, for the author

- **deduxit's `<desc>` in the drawing's notation**: `write_steps` writes
  `~A` where the `<title>` and the page write `A⊥`; a screen reader hears
  two notations for one formula (`core/src/export/svg/`, `fmt.rs`). The
  `<desc>` of a net is its text form run together ("⊢ ~A ⅋ ~B, B ⊗ A
  ~A[1] — A[5] ~B[2] — B[4] proof net"), with no sentence breaks. Both
  are deduxit's, not the client's.
- **Rule labels named by their step**: "Rule ax, step 1" (the
  description's numbering), so that a reader tabbing through a drawing
  knows which inference each label belongs to.
- **Rule labels' targets on a phone**: a zoom floor for a fitted
  finished drawing (as `floor` does for the active goal's formulas), or
  labels that share the gap with their neighbours as the formulas'
  buttons do; either changes how a drawing fits a phone, which is the
  author's call.
- **The atom-balance reason says "A is ahead by -1"** when the dual is
  the one ahead (`words.ts`, `unbalanced`): it should name the atom that
  is ahead, or say "behind".
- **The dock trails the goal by one event during a drag** (the
  reviewer's observation; not compared with `main`): the dock effect
  could run after the world's transform, as the buttons now are
  measured.
- **`prover.spec.ts`'s broken-link step** (`&d=AAAA`, reload) failed 1
  of 24 runs on this chain: a race to find and fix at its cause.
- **A review by a screen-reader user**, as web-teaching-4 says: the
  pass above read the tree Chromium builds, not what NVDA, JAWS,
  VoiceOver or Orca say.

### What the next sessions must know

- A new control, popover or state gets a step in `tests/states.ts`;
  `targets.spec.ts` measures it and `accessibility.spec.ts` audits it in
  both schemes at both widths. A walk that grows past about fifteen
  states with axe in each nears the dev shell's 30 seconds on a phone:
  split it, as `proving` and `menus` are.
- axe's label-in-name check compares words: a control's `aria-label`
  repeats every word it shows, parentheses do not count, and the parts
  of a button's label need a space between them in the markup.
- Every box read off the drawing is divided by `rendered()`, never by
  `camera().k`.
- The keyword washes are background images, so axe leaves their
  contrast "incomplete"; `colours.test.ts` is what holds them.
- The client's `web.md` records all of these.

## Fix: the bottom band (2026-11-16)

The author's report (about 18:50): on the live site, at a wide desktop
window, the bottom band's icons and labels were drawn on top of each
other in one run at the centre.

- **Reproduced** in the built-in browser at 2000 × 1080, both schemes,
  on the live site and on the client's build of `main@origin`: every
  label `position: absolute`, unclipped, over its icon and its
  neighbours. With Learn on the band was right, and on a phone too.
- **The cause**: two components shared the class `lb`. The band's labels
  (`BottomBand.tsx`, `<span class="lb">`) had no rule at a desktop's
  width with Learn off, so they stood beside their icons as D54 asks.
  The client's `xxvkxvzm` ("Give every rule label a hint", 2026-11-09)
  gave the rule labels' buttons over the drawing the class `lb` with a
  global rule `.lb { position: absolute; … }`, which caught the band's
  labels too. Builds of `xxvkxvzm` and of its parent `vupupxnp`, served
  as the e2e suite serves them, show the band broken and right. So the
  bug is a week old, from none of today's commits the brief listed.
- **The fix** (`vzmpmrxp`): the band's labels get a class of their own,
  `act-name`; its four rules in `app.css` and `learn.spec.ts`'s locator
  follow. The drawing's `.lb` is unchanged. Checked in the browser at
  2000 × 1080 in both schemes: each action's icon and label stand apart.
- **Why the suite missed it**: the target-size walks and axe measure the
  buttons, and the band's buttons kept their size and did not overlap;
  only their label spans spilled out. No test looked at a button's
  contents.
- **The guard** (`qlzlvuwm`): `page.spec.ts`, "the bottom band's
  actions and their labels stand apart at every width, Learn off and
  on", at 320, 412, 641, 800, 1024, 1280, 1920 and 2560 pixels: no two
  actions overlap, and each action's icon, label and key lie inside its
  button and apart from each other (a label clipped for a screen reader
  is skipped). Against the build of `xxvkxvzm` it fails at 641 with
  Learn off, naming every label of the author's report; against its
  parent and against the fix it passes.
- **Recorded** (`unnzuxur`): a bullet in the client's `web.md` on the
  band's labels and why a class belongs to one component.
- **Checks**: on `qtwlokpn`, the affected e2e specs against a fresh
  build (`page`, `learn`, `hints`, `targets`, `accessibility`, `phone`:
  140 passed); after the rebase onto `qxuysxlm`, the client's Vitest
  (677 passed), `lint` (Biome and `astro check`, 0 errors) and its `nix
  flake check`, the whole e2e suite among its checks: all checks passed).
- No reviewer: the fix is the band's class and styles alone, as the
  brief allows.
- The client's chain, rebased onto its `main@origin` (`qxuysxlm`): `vzmpmrxp`,
  `qlzlvuwm`, `unnzuxur`, in the workspace
  `../deduxit.github.io-fixband`. Nothing pushed.
