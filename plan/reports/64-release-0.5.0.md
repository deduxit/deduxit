# Step 64: the release of 0.5.0, prepared

Opus 5.5 at `high`, a reused worker started by `deduxit-orchestrator`
from `/tmp/linlog-handoff/start-64.md` on 2026-11-16, cores 7 to 9, in
the jj workspace `../deduxit-wt-step-58a-610ed3`, on `main@origin`
`wnxtnzyn` ("Plan: 64 started").

## Outcome

0.5.0 is ready to tag in deduxit, up to the author's acts. The chain
below ends in this report's commit. The tag is `v0.5.0`, message
"deduxit 0.5.0", once the chain is on `main`, CI is green there and
63g-c's page is on the client's `main` with its input moved.

The Rocq library has not moved since its `v0.4.0` (`ef979655`, which
the flake pins and which is rocq-deduxit's `main@origin`), so it gets no
release: `export::rocq::DEDUXIT` stays "0.4.0" and no workspace of it
was made. The release notes, what a user can do now, the checks before
and after the tag and the author's checklist are in
`plan/notes/release-0.5.0.md`.

## The break: `breaks 0.4.0`, and how each line is covered

`breaks 0.4.0` (cores 7–9, at `main@origin` `wnxtnzyn`):

- cargo-semver-checks: 229 checks, 228 pass, 1 fail, 0 warnings.
- cargo-public-api: Removed (none), Changed (none), the rest Added.

| line | covered by |
|---|---|
| discriminant changed: `rocq::Unsupported::Saturated` 8→9, `Quantifiers` 9→10, `Exhausted` 10→11, `Basis` 11→12 | removed: `Unsupported::Calculus` (61b) was inserted before them; `xzvlmnot` moves it last, as 0.4.0 did with `Basis`. `core.md`'s paragraph on the break now says a new variant of a fieldless enum goes last. |

Nothing else breaks: every new variant and field is of a
`#[non_exhaustive]` type (`Error`, `Refusal`, `Limits`, `Space`, the
styles), which the type lists below confirm. The changelog's "What
breaks" says so in a paragraph and points to "Changed", which lists
what a program may notice: `serde_json` in a default build (the default
feature `protocol`), thirteen new error codes (checked against
`Error::CODES` at `v0.4.0`: 67 then, 80 now), `deduxit` without a
command listing `interpolate` and `workspace` with the new flags in the
help (the behaviour lock's only modified files since `v0.4.0` besides
four Typst snapshots of the own layout), and 0.4.0 refusing levels 15
to 19 and the new settings keys.

**The two lists of types**, checked against the head's rustdoc JSON
with every feature by a script: 18 closed enums, exactly the list's
(its count said seventeen; `rules::Step` came at step 60), and 186
public enums and structs with a public field, all `#[non_exhaustive]`.
Fourteen were missing from `core.md`'s list and are added in
`xkrrquum`: `ordinary::interactive::{Applicable, Closed, Needs, Step,
StepError}` (since step 49: the 0.4.0 script matched them by name to
the linear session's types of the same names), `ordinary::interpolant::
{Interpolant, Interpolation, Partition}`, `ordinary::negative::
{Agreement, Comparison, Options, Translation}` (63t) and `rules::
{Applicable, Apply, Derive, Undone}` (61, 62). `workspace::Computed`
is listed and marked but has no public field, so the script does not
count it; the list keeps it.

## What was prepared

- **The version** is 0.5.0 in the manifests and both locks (a cargo
  run moved the workspace's own packages; `cargo update --offline -p
  deduxit` in `fuzz/`), `CITATION.cff` (2026-11-16), the dependency
  lines of README, the library chapter and the installation chapter,
  the installation chapter's file names, the JSON examples of the
  manual (`ordinary-logic.md`, `terms.md`) and of the docs (`wire.rs`,
  `ordinary/decide.rs`), and `cli/tests/lock/inputs/workspace.json`
  (the document and its outcome's `deduxit`), in the bump's own commit
  `lvzklvol` as 63g-b asked, so its records stay the build's own. The
  compat crates keep their 0.3.0.
- **The bless** (`qmrrlvrz`, a commit of its own): the JSON lock's 18
  outcomes and the serialize test's five change by the version string
  alone (the lock compared with the version masked: equal).
- **The stability test** (`olnqpyyw`, departs from 0.4.0's practice):
  `core/tests/wire/level-11.txt` to `level-14.txt` are what 0.4.0 wrote
  and are never rewritten, but an ordinary outcome read and written
  again names the version that writes it, so the levels 12 to 14 failed
  on the version alone. `again` now compares with the top-level
  `deduxit` key as this build writes it (`written_now`) and the rest
  byte for byte. At 0.4.0 the files were blessed to 0.4.0, since no
  release had written them; now one has. The reviewer checked that every
  line of those files has one such key and nothing else is masked.
  `core-sequents.md` records it.
- **The schemas published with the release** (`kzyvqnyx`): the release
  job copies `docs/src/wire.schema.json` and
  `core/src/rules/calculus.schema.json` (as `rules.schema.json`) into
  its files before the checksums, so both are attached, summed and
  attested; `ci.md`, the protocol chapter and the rules chapter say so.
  `wire.schema.json` stays published at the manual's address too. I
  chose release assets over a second copy in the manual: one source per
  schema, and a release's assets are that version's.
- **The status chapter** (`wxwsmuxp`): the 0.5.0 bullet of "Planned"
  removed; the page now shows the translations, interpolants, the
  workspace, natural deduction and a calculus of one's own, and Learn
  and Practice by suite; the command lists `interpolate` and
  `workspace`; drawings of derivations in a calculus as trees and Fitch
  proofs; the bindings' new calls; the baseline of 2026-11-16 named.
- **The Rocq library's version** (`pyxswzmo`): the Rocq chapter said
  "Its version is deduxit's", and the guide "the tag of deduxit's own
  version", both false at 0.5.0; they now say the library is released
  beside deduxit under deduxit's number when it changes, and that
  deduxit 0.5.0's certificates compile against rocq-deduxit 0.4.0,
  which `export::rocq::DEDUXIT` names.
- **The descriptions** (`vwxxxurr`): README's and `CITATION.cff`'s
  first paragraph name rule files and natural deduction (one sentence
  each, a keyword "natural deduction"); the crate docs' "What you can
  do" gains the transformations, the rules' drawings and the workspace,
  and the feature `protocol` names the workspace; the installation
  chapter's features are a table: each feature, whether it is on by
  default, what it adds and the dependencies it brings (`protocol`:
  `serde_json`).
- **The changelog** (`zpqvwzpk`): `## [0.5.0] - 2026-11-16` under an
  empty `## [Unreleased]`, a first paragraph of what 0.5.0 brings (the
  page's part and the schemas among it), "What breaks" (nothing, with
  the reason), "Changed" (above), the steps' "Added" as they wrote it,
  "Wire forms" for levels 15 to 19 and `wire::LEVEL` 19, the link
  `[0.5.0]`. No "Fixed": every fault the reports fixed was in code new
  in 0.5.0.
- **The guide** is within its budget: the summary is unchanged, and
  `reference.md` changed one sentence; the examples test passed in the
  gate.
- **The release workflow's dry run**: its tag check run locally (the
  tag equals the manifests' 0.5.0, the changelog's section is dated,
  `CITATION.cff` states it); its crate selection against crates.io's
  API for `v0.5.0`: `linlog 0.3.0` and `linlog-cli 0.3.0` "on crates.io
  already", `new=` (none), `existing=deduxit deduxit-cli`, so the job
  publishes both through Trusted Publishing and uses no secret; its
  `cargo package --locked --package=deduxit --package=deduxit-cli`: exit
  0, both packaged and verified at 0.5.0, from a clean tree.
  `cargo publish --dry-run` is denied to this session, as before.
- **The release notes** are the changelog's section as the workflow's
  `awk` writes it, generated into `plan/notes/release-0.5.0.md`, with
  what a user can do now for an announcement.

## The checks

Every build and test ran in a capped scope on cores 7–9
(`CARGO_BUILD_JOBS=3`, `RUST_TEST_THREADS=3`); `nix flake check` under
`flock -o /tmp/linlog-machine.lock` with `--max-jobs 4 --cores 8`.

- **`breaks 0.4.0`**: above.
- **`cargo test --workspace`** after the version raise, before the
  bless: only the tests that pin the version failed (`json_lock`,
  `outcome_json_format`, and `stability.rs`'s `outcomes_read_back` and
  levels 12 to 14); the CLI's lock passed with the restated workspace
  input. After the bless and the test change: the three files pass (1,
  25, 14 tests).
- **The gate** at `xkrrquum`: exit 0, "gate: passed" (clippy, the
  tests, both `cargo hack` runs, the ratchet). The cache model reported
  moved misses on `render-svg`, never a failure.
- **`nix flake check`** at `omkxwwnw` before the reviewer's fixes:
  exit 0, "all checks passed!". The fixes after it changed a sentence
  of the installation chapter and two rules and plan files; the
  manual's checks ran again on the final tree:
  `nix build .#checks.x86_64-linux.manual
  .#checks.x86_64-linux.manual-links`: exit 0.
- **The workflow's dry run**: above (tag check, selection, `cargo
  package`).

## The reviewer

`fresh-reviewer` (Fable 5.2 at medium) read the chain against eight
criteria (the break, the versions and dates and the Rocq library's,
the changelog against the code and the workflow's check, the status
chapter against the client, the stability test, the type lists, the
schemas' step, the notes and checklist). It built nothing; it ran the
workflow's `awk`, compared `Error::CODES` with `v0.4.0`'s and counted
the types in the rustdoc JSON itself. It found the rest sound. Its
three findings held and are fixed:

1. The marked list lacked `ordinary::negative::{Options, Translation}`
   (my script matched them by name to `ordinary::{Options,
   Translation}`): added in `xkrrquum`.
2. The installation chapter's new sentence said `serde_json` need be
   added only where `protocol` is off; a caller names it in its own
   manifest whatever deduxit's features are. The old sentence is back
   (`vwxxxurr`).
3. The notes' last section was empty: the workflow's text is in it now
   (`omkxwwnw`).

## Departures from the design

- **The stability test compares the writer's version as this build
  writes it**, where 0.4.0 blessed the files of its new levels; the
  files are now what a release wrote and stay so. (departs from the
  design)
- **The schemas are release assets**, `wire.schema.json` and
  `rules.schema.json`, added to the release workflow; the prompt says
  "published with the release" without the form. (departs from the
  design)
- **README, `CITATION.cff` and the crate docs gained a sentence on rule
  files and natural deduction**, beyond the versions, since the
  release's title is that. (departs from the design)
- **The date is 2026-11-16** in the changelog and `CITATION.cff`. If
  the tag is made on another day, the planning session moves both.
- **No Rocq release**: the library did not move, so the brief's
  condition for one is not met.

## What is left open

- The client's input `deduxit` moves to `main` once this chain is
  pushed (63g-c); the page's part of the changelog's first paragraph
  holds until then.
- `kani.yml`'s last run on `main` should be green before the tag (the
  notes' checklist).
- `docs/src/results/` has no chapter for the baseline of 2026-11-16;
  `bench/RESULTS.md` has it, and the status chapter points there.

## What the next sessions must know

- A new variant of a fieldless public enum goes last (`core.md`, "The
  break against the last release"): `rocq::Unsupported` needed the move
  at 0.4.0 and again at 0.5.0.
- The type-list script must match by full path: a name shared by two
  modules (`Applicable`, `Step`, `Options`, `Translation`) hid seven
  unlisted types.
- `core/tests/wire/` keeps what a release wrote; `stability.rs`
  compares the writer's version as the current build writes it, so a
  version bump no longer touches those files.
- The Rocq library's version is its own once deduxit moves without it;
  `DEDUXIT` and the flake's assertion follow the pinned library.

## Commits

deduxit, in `../deduxit-wt-step-58a-610ed3`, on `main@origin`
`wnxtnzyn`, oldest first:

- `xzvlmnot` Keep rocq::Unsupported's numbers as 0.4.0 had them: Calculus last
- `lvzklvol` Raise the version to 0.5.0
- `qmrrlvrz` Bless the behaviour lock: an outcome states version 0.5.0
- `olnqpyyw` Read back what 0.4.0 wrote at levels 11 to 14 with this build's version
- `kzyvqnyx` Publish both JSON Schemas with each release
- `wxwsmuxp` Keep the status chapter true for 0.5.0
- `pyxswzmo` Name the Rocq library's version apart from deduxit's
- `vwxxxurr` Describe 0.5.0's rule files, natural deduction and features
- `zpqvwzpk` Date 0.5.0's section of the changelog, with what changed and its wire levels
- `xkrrquum` Rules: count the marked types at 0.5.0
- `omkxwwnw` Plan: the release notes of 0.5.0 and the author's checklist
- this report
