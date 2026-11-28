# Step 87e1: the linear search's front door in five parts (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the
workspace `../deduxit-wt-step-84-dc9a99`, from the planning session's
briefs `start-87-common.md` and `start-87e1.md`: `plan/87-refactor.md`'s
87e1, F-arch-19. Based on 87c9b's head (`onxkuoxx`), on cores 4 to 6,
the target sets on 2 and 3 by the planning session's leave. Wave E
started before wave C ended, the planning session's decision (departs
from the plan): neither 87c5 nor 87c6 touches this file.

## Checklist

- [x] The five parts moved, one commit each, each building in four
  feature sets (all features with the tests, the defaults with the
  tests, none, `parallel` alone).
- [x] The rules files (`core-search.md` with the new files in its
  paths, `core.md`, `claude-infra.md`).
- [x] Both target sets against `86.csv` and `net-86.csv`: every counter
  equal.
- [x] `fresh-reviewer` (Opus, high): three findings, two fixed, one for
  the planning session.
- [x] The gate, `breaks 0.8.0` (nothing new).
- [x] No local `nix flake check` (the planning session's change of
  2026-11-28: it runs every flake check on GitHub's runners before
  anything lands); the one started here was stopped while it waited for
  the lock. At the head after the reviewer's fixes: clippy on the core
  crate with every feature and its tests, `--deny warnings`, clean; the
  search's 205 unit tests pass; the docs with and without private items,
  `-D warnings`, clean.

## What landed

`core/src/linear/search/mod.rs` was 5 621 lines; it is 3 086 now, the
front door (`prove`, `prove_within`, `prove_goal`, `race`, `engine_for`
and the passes they share) and its tests. The five parts:

| file | lines | holds | commit |
|---|--:|---|---|
| `outcome.rs` | 551 | `Outcome`, `Verdict`, `Reason`, `Statistics` | `rpqrqvwr` |
| `options.rs` | 736 | `Options`, `Bias`, `Jobs`, `Cadence`, `Schedule` | `mxpooryl` |
| `engine.rs` | 833 | `Engine` with its documented table, `Counter`, `NotTaken`, `Engine::implementation`; `the_documented_dispatch_is_the_dispatch`, which now reads `engine.rs` | `ykutmlpy` |
| `dispatch.rs` | 363 | `DISPATCH`, `Row`, `Modes`, `Feature`, the inverse method's turns, `goal_fragment` | `mozqtukm` |
| `interface.rs` | 167 | `Task`, `Finished`, `Answer`, `Decide` | `zlztwymz` |

The rules: `mnsxovlz`. The target sets' CSVs: `lmmquwoy`.

What changed beyond the moves, each needed by them:
- `use` lines, and `#[cfg(doc)]` imports where a doc link would no
  longer resolve (the crate's pattern, `work.rs`). A child module's
  inner doc resolves in `mod.rs`'s scope, since its `mod` line has an
  outer doc too, so `mod.rs` imports `ProofStructure` for the docs of
  `essential.rs`, `net.rs` and `planar.rs`, which it imported for code
  before.
- `pub(super)` on what a part uses of another's private items: the
  dispatch's items (`dispatch`, `probes`, `DISPATCH`, `PROBE_AFTER`,
  `TURN_SHARE`, `TURN`, `turn_bounds`, `Turns` with its fields and
  `take`, `Row` with its fields, `Modes`, `Feature`),
  `Engine::implementation`, and `Task::program`, which `horn/mod.rs`
  reads.
- A module doc per new file, a doc on each new `mod` line, the moved
  test's `include_str!("engine.rs")`, and three `use` lines in
  `mod.rs`'s tests for what they named through the old imports
  (`Node`, `NodeId`, `ProofStructure`, and the dispatch's test
  constants).

Every public path `deduxit::linear::search::…` is re-exported from
`mod.rs`, and so are the crate-private `goal_fragment`, `Answer`,
`Decide`, `Finished` and `Task`.

## The net

- **Target sets** (cores 2 and 3, `bench/targets.sh 87e1` and
  `bench/net-targets.sh 87e1`, the release build of `mnsxovlz`):
  - the focused set, 284 rows: no verdict differs, and of the 145
    problems both decide alike on one thread, 0 differ in any counter;
  - the net set, 64 rows: no verdict differs, and 0 of 64 differ in any
    counter.
- **The gate** at `mnsxovlz`, cores 4 to 6: passed (clippy, the tests,
  both `cargo hack` runs, the ratchet within its ceilings; the cache
  model's misses moved on `search-chain-128` and `check-qbf-20-2`, which
  it reports and does not fail on, as at 87c9b, whose code those
  journeys ran too). The reviewer's two fixes after it changed docs and
  one test's doc comment only; the full check below covers them.
- **The docs**, `cargo doc -p deduxit --all-features --no-deps` with
  `-D warnings`, with and without private items: clean at the head.
- **`breaks 0.8.0`**: cargo-semver-checks fails the same three checks
  as at 87c9b (`enum_struct_variant_field_missing`, `enum_variant_added`,
  `type_method_marked_deprecated`), all the base's. cargo-public-api's
  diff now names the twelve moved public types by another of their
  public paths (`deduxit::linear::Outcome` where it said
  `deduxit::linear::search::Outcome`), since their defining module is
  private now; both paths stay. With those names made one, 9 lines are
  removed, exactly the base's known breaks (the step errors' `reason`
  fields, `lambda::Term::goal`, `Typing::ty`, `Variable::ty`), and
  nothing this step moved.

## The reviewer's findings

`fresh-reviewer` on Opus at `high`. It compared each commit's removed
and added lines as ordered sequences and found every moved line byte for
byte what it was, but for:

1. **The moved test's doc comment stayed in `mod.rs`** (`ykutmlpy`), on
   top of `dispatch_by_mode`'s. Fixed in that commit: the five lines
   moved with the test.
2. **`mozqtukm` failed the doc checks**: `dispatch.rs`'s module doc
   linked `DISPATCH`, which resolves in `mod.rs`'s scope, and the
   `#[cfg(doc)]` import came one commit later. Fixed in that commit: the
   doc links it by path (`dispatch::DISPATCH`), and `mod.rs` needs no
   import for it.
3. **`evidence.md` and the mutation batch** (for the planning session):
   `mutants/run.sh`'s `search` batch names `core/src/linear/search/mod.rs`
   alone, so the dispatch, the options' setters, the outcome's methods,
   `Answer::of_arena` and the engine catalogue left the mutation scope;
   `.claude/rules/evidence.md` (lines 17-24) still says the front door is
   `search/mod.rs`, and its note on `cargo mutants --list --file
   …/search/mod.rs` listing the `Options::with_*` field deletions now
   needs `options.rs`. Both files are outside this step's paths. The
   natural fix is a batch per file or the batch's file list widened to
   the six files, which the next mutation run decides.

## What the next steps must know

- F-arch-01, F-arch-02, F-arch-04 and F-arch-17 each change one of the
  parts: the options (`options.rs`), the outcome (`outcome.rs`), the
  engine interface (`interface.rs`) or the catalogue (`engine.rs`).
- The documented table of the dispatch is `Engine`'s rustdoc in
  `engine.rs`; `the_documented_dispatch_is_the_dispatch` reads it from
  there and the rows from `dispatch.rs`.
- A doc link in a child module's `//!` docs resolves in `mod.rs`'s
  scope: name the item by path there, or import it in `mod.rs` under
  `#[cfg(doc)]`.
