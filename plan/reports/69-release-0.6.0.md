# Step 69: the release of 0.6.0, prepared

Opus 5.5 at `high`, a reused worker started by `deduxit-orchestrator`
from `/tmp/linlog-handoff/start-69.md` on 2026-11-16, paused at the
weekly limit and resumed on 2026-11-20, cores 7 to 9. deduxit in the jj
workspace `../deduxit-wt-step-58a-610ed3`, begun on `main@origin`
`kqnxstnt` ("Plan: step 68 complete; 69 started") and rebased at the
end onto `zuswqzsu` ("Plan: 54x8 started", above 54x7); the Rocq library
in `../rocq-deduxit-69` on its `main` `ztysvovp`.

## Outcome

0.6.0 is ready to tag in both repositories, up to the author's acts.
The release is dated 2026-11-20 (moved from 2026-11-16 on the planning
session's word after the pause).

- **rocq-deduxit 0.6.0** (`qworrpqw`, then `nmklslnu` re-dated,
  `03f0af20`): the changelog's `[Unreleased]` became `## [0.6.0] -
  2026-11-20`, with a line on why there is no 0.5.0; the opam version
  and date, `CITATION.cff`'s version and date; README's install lines at
  `v0.6.0`, its opening naming modal logic, and its coverage section
  saying that deduxit 0.6.0's modal certificates compile against it.
  Its `nix flake check` passed on both commits. The planning session
  pushed both and tags `v0.6.0` at `03f0af20` when its CI is green.
- **deduxit 0.6.0**: the chain below, which pins that commit and names
  rocq-deduxit 0.6.0 in `export::rocq::DEDUXIT`. The release notes,
  what a user can do now, the order of the tags and the publishing and
  the author's checklist are in `plan/notes/release-0.6.0.md`.

## The break: `breaks 0.5.0`, and how each line is covered

`breaks 0.5.0` (cores 7–9, at `kqnxstnt`, before the version raise):

- cargo-semver-checks: 229 checks, 229 pass, 31 skip, "no semver update
  required".
- cargo-public-api: Removed (none), Changed (none), 615 lines Added
  (`modal`, the modal writers of `export::rocq` and `export::svg`,
  `rocq::Unsupported::Assumption`, `rules::Interactive::extend`,
  `ordinary::WrittenTableau` and the rest of steps 65 to 68).
- Again after the rebase onto 54x7, on the final tree: the same 229
  checks pass, "no semver update required"; Removed (none), Changed
  (none), 619 lines Added (54x7's four methods of
  `ordinary::Interactive`).

No line breaks a caller, so the changelog's "What breaks" says nothing
does and why: every new variant or field belongs to a
`#[non_exhaustive]` type, and `Unsupported::Assumption` came last (66c),
so no discriminant moved. The error codes are those of 0.5.0 (80, the
same list, compared with `v0.5.0`'s `Error::CODES`). "Changed" lists
what a program may notice: 54x7's `fresh` that is new to the rule's
other variables too, `check --format json`'s new key `system` and the
labels with `□`/`◇` (from the steps), the help of `check` and `prove`
naming `--system` and `--tableau` and that of `interact` naming
`--invertible` (the behaviour lock's only modified files since `v0.5.0`
besides the version), the protocol's fourth suite `modal`
(`protocol::suites()` had three at 0.5.0), and 0.5.0 refusing level 20.

**The two lists of types**, against the head's rustdoc JSON with every
feature by a script matching each type by its defining path and the
listed module as a subsequence of it: 19 closed enums, exactly the
list's, and 196 public enums and structs with a public field, all
`#[non_exhaustive]`, each found in the list, the same after the rebase
(54x7 adds no public type). Four marked structs
without a public field (`linear::Mode`, the two `Inference`s and
`workspace::Computed`) are outside the count; `Computed` is listed all
the same. `core.md` now gives the counts at 0.6.0 and says how the paths
read (`lupyzywt`).

## What was prepared

- **The version** (`tllxurrs`) is 0.6.0 in the manifests and both
  locks (`cargo metadata` moved the workspace's packages; `cargo update
  --offline -p deduxit` in `fuzz/`), `CITATION.cff` (2026-11-20), the
  dependency lines of README, the library chapter and the installation
  chapter, the installation chapter's file names, the JSON examples of
  `terms.md` and of the docs (`wire.rs`, `ordinary/decide.rs`,
  `negative.rs`, `interpolant.rs`), and the behaviour lock's inputs
  `cli/tests/lock/inputs/workspace.json` and `MODAL_OUTCOME` in
  `cli/tests/lock.rs`, in the bump's own commit as 63g-b asked. The
  compat crates keep their 0.3.0.
- **The bless** (`wvyoznru`): the JSON lock's 18 outcomes, the serialize
  test's five, the three modal `check` outputs of the CLI's lock (which
  echo the input's version) and `core/tests/wire/level-20.txt` change by
  the version string alone. Level 20's file was written by a build
  before any release; 0.6.0 is the first to write it, so it now says
  0.6.0 (`stability.rs` masks the writer's version, so this is the
  record's truth, not a test's need).
- **The status chapter** (`yutwxuyl`): the 0.6.0 bullet of "Planned"
  removed; modal certificates under the outputs; `Deduxit.Modal` in the
  library's list; the bindings' modal calls and the tableau review; the
  page's modal tab, model editor, tableau editor, course packs, check
  view, embed and print, worded from 68b's and 68c's reports.
- **The descriptions** (`vtkotvty`): README's list of suites reads as a
  list again (its modal clause had made it "… and … and …");
  `CITATION.cff`'s abstract and keywords, both crates' descriptions and
  the crate docs' "What you can do" name modal logic, its certificates
  and the tableau checker.
- **The lock** (`ysxyqokk`, "flake.lock: Update"): rocq-deduxit at
  `03f0af20`, its re-dated release commit, which the tag will name.
- **`DEDUXIT`** (`wnznmxto`): "0.6.0", its doc naming `Deduxit.Modal`;
  the Rocq chapter's pinned versions, its paragraph on the library's
  version (it named 0.5.0 and 0.4.0) and its install lines; the guide's
  tag line.
- **The changelog** (`pklrslzz`): `## [0.6.0] - 2026-11-20` under an
  empty `## [Unreleased]`, a first paragraph of what 0.6.0 brings (the
  page's part among it), "What breaks" (nothing, with the reason),
  "Changed" (above), the steps' "Added" as they wrote it, "Wire forms"
  for level 20 and `wire::LEVEL` 20, the link `[0.6.0]`. No "Fixed":
  every fault the reports fixed was in code new in 0.6.0.
- **54x7, folded in.** It landed on `main` after the chain was ready
  (`xvpnroyr..lwlyzqlw`), with its entries under `[Unreleased]`: the
  invertible rules of LK and LJ under "Added" and the `fresh` name
  under "Changed". The rebase put the first into 0.6.0's "Added" by
  itself; the second conflicted with the dated section and is now the
  first entry of 0.6.0's "Changed", where 54x7 had put it. The section's
  first paragraph names the invertible rules, "Changed" the help of
  `interact`, the status chapter's line on LK and LJ step by step that
  a session applies them by itself (`yutwxuyl`), and the notes' "What a
  user can do now" gives the command. Its modal fuzz run changes no
  text a user reads. Nothing new on the page from it.
- **The guide** is within its budget: the summary is unchanged, and
  `reference.md` changed one line; the examples test passed in the
  gate.
- **The release workflow's dry run**: its tag check run locally (the
  tag equals the manifests' 0.6.0, the changelog's section is dated,
  `CITATION.cff` states it); its crate selection against crates.io's API
  for `v0.6.0`: `linlog 0.3.0` and `linlog-cli 0.3.0` "on crates.io
  already", `new=` (none), `existing=deduxit deduxit-cli`, so the job
  publishes both through Trusted Publishing and uses no secret; its
  `cargo package --locked --package=deduxit --package=deduxit-cli`: exit
  0, both packaged and verified at 0.6.0, from a clean tree.
  `cargo publish --dry-run` is denied to this session, as before.
- **The release notes** (`nlwmnswz`) are the changelog's section as the
  workflow's `awk` writes it, generated into
  `plan/notes/release-0.6.0.md`, with what a user can do now and the
  checklist.

## The checks

Every build and test ran in a capped scope on cores 7–9
(`CARGO_BUILD_JOBS=3`, `RUST_TEST_THREADS=3`); every nix build under
`flock -o /tmp/linlog-machine.lock` with `--max-jobs 4 --cores 8`.

- **`breaks 0.5.0`**: above.
- **rocq-deduxit's `nix flake check`**: "all checks passed!" at
  `qworrpqw` and again at `nmklslnu`.
- **`cargo test --workspace`** after the version raise, before the
  bless: only `json_lock` and `outcome_json_format` failed, on the
  version alone, as at 0.5.0; 31 other test binaries passed. After the
  bless: both locks pass (`BLESS=1` runs, then the gate).
- **The Rocq checks against the new pin**: `nix build
  .#checks.x86_64-linux.rocq-deduxit .#checks.x86_64-linux.rocq-deduxit-library
  .#checks.x86_64-linux.check-certificate` at `03f0af20`: exit 0 (and
  the first two at `1ab69571` before the re-dating: exit 0).
- **The gate** at `wvyoznru`: exit 0, "gate: passed" (clippy, the
  tests, both `cargo hack` runs, the ratchet). The cache model reported
  moved misses on `render-svg`, never a failure, as at 0.5.0. Again
  after the reviewer's fixes and the rebase onto 54x7, on the final
  tree: "gate: passed", the same note.
- **`cargo package`**: above.
- **`nix flake check`** on the final tree, after the rebase: "all checks
  passed!" (x86_64-linux; the release workflow's `check` job runs the
  same).

## The reviewer

`fresh-reviewer` (Fable 5.2 at medium) read the chain against the brief
before 54x7 landed. Three findings, all fixed in the commits they
concern:

1. `docs/src/ordinary-logic.md` still showed `"deduxit": "0.5.0"` in
   three JSON examples. The manual's examples mask the version
   (`cli/tests/common/mod.rs`), so no test caught it. Fixed in the
   version raise (`tllxurrs`).
2. The changelog's "the help of `check` and `prove`" read as if `prove`
   took `--tableau`. Now each command's help is named with its own
   flags (`pklrslzz`).
3. `core.md`'s new sentence read as if `workspace::Computed` were in the
   count of 196. Reworded: it is listed but outside the count
   (`lupyzywt`).

It confirmed the rest: the version everywhere a user reads it, the
bless changing the version string alone, the lock and `DEDUXIT`, the
status chapter, the dates, and that the notes' last section was the
`awk`'s output at the time. The 54x7 fold came after it and is not
reviewed; the notes' last section is the `awk`'s output again.

## Departures

- `core/tests/wire/level-20.txt` blessed to say 0.6.0: the file keeps
  what a release wrote, and 0.6.0 is the first to write level 20
  (departs from the design).
- The rocq-deduxit lock moved in this chain (`ysxyqokk`) on the
  planning session's word, not by the planning session after the tag.
- Dated 2026-11-20, not 2026-11-16, on the planning session's word
  after the pause.
- README's list of suites, `CITATION.cff`'s abstract and keywords and
  both crates' descriptions gained modal logic; the brief named README
  for installation only (departs from the design).

## What is left open

- The tags and the publishing, in the order of
  `plan/notes/release-0.6.0.md`, with the author's checklist there:
  rocq-deduxit `v0.6.0` is tagged (`41dac0af` at `03f0af20`); deduxit's
  chain goes to `main`, `ci.yml` and `kani.yml` green, then the tag
  `v0.6.0`, the author's approval of `cargo publish`, the GitHub
  release.
- The web client moves its `deduxit` input to the tagged commit after
  the release, as at 0.5.0.

## What the next sessions must know

- The manual's examples mask `"deduxit": "…"`, so a version left behind
  in the docs passes every test: at a release, grep `docs/`, the doc
  comments and `skills/` for the old version.
- A feature that lands after the changelog's section is dated conflicts
  with it on rebase: its `[Unreleased]` entries go into the dated
  section by hand.

## The commits

On `main@origin` `zuswqzsu`, oldest first:

- `tllxurrs` Raise the version to 0.6.0
- `yutwxuyl` Keep the status chapter true for 0.6.0
- `vtkotvty` Describe 0.6.0's modal logic and tableau checker
- `ysxyqokk` flake.lock: Update
- `wnznmxto` Name rocq-deduxit 0.6.0, with Deduxit.Modal, as the
  certificates' library
- `pklrslzz` Date 0.6.0's section of the changelog, with what changed
  and its wire level
- `lupyzywt` Rules: count the marked types at 0.6.0
- `wvyoznru` Bless the behaviour lock: an outcome states version 0.6.0
- `nlwmnswz` Plan: the release notes of 0.6.0 and the author's checklist
- `xuptrvts` Report 69: the release of 0.6.0, prepared

In rocq-deduxit, on its `main`: `qworrpqw` Date the release of 0.6.0,
`nmklslnu` Date the release of 0.6.0 on 2026-11-20 (`03f0af20`, tagged).
