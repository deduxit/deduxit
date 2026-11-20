# No test run twice, and CI in parallel (2026-11-20)

## Session ci-speed

Brief `/tmp/linlog-handoff/start-ci-speed.md` (Opus 5.5 at `high`,
2026-11-20, cores 4-6), in the jj workspace
`../deduxit-wt-step-54s-c1-161790`, started on `main@origin` `xuptrvts`
and rebased onto `uplqtntoqwys` (0.6.0 tagged and published). The
author's request: look at every test that runs, remove duplicated
effort, and let CI run in parallel, by current idiomatic practice.

| change | what |
|---|---|
| `uktnswln` | CI's checks as a matrix of groups (`modules/ci.nix`, `checks.yml`), Pages deployed from CI (`docs.yml` gone), the release through the same checks with its packages beside them, `ci/NAME` branches checked as `main`; `ci.md`, `flake.md` |
| `zrkvumso` | Install after a release tests the release's own files instead of building them again |
| `uspksmlk` | the local procedure: `CLAUDE.md`'s table and CI section, `plan/handoff.md` items 5 and 7, `plan/push-ci.sh` |
| `ymoomvxv` | this report |

Not to land: the probe commits `rptpvnsl` (branch `ci-speed-probe`, PR
2), `kktvnkko` (`ci-speed-before`) and `tmtvlrnk` (`ci/ci-speed`), each
a comment line in `core/src/lib.rs`. Deleting their remote branches is a push that needs the author's yes.

## The inventory

Every place a test, lint or build runs, before this session. Times:
CI's from `gh` (the runs named), local ones from
`plan/notes/step-durations.md` (69 gate runs and 72 flake checks of
step sessions) and the report of 2026-11-15. "Dup" marks the same work
run twice for no gain.

| where | what runs | covers | time | dup |
|---|---|---|---|---|
| step session, each commit touching code | the gate: Clippy (workspace), `cargo test --workspace` (dev), `cargo hack` each feature and pairs, the ratchet | lints, tests with debug assertions, feature sets, instruction counts | 6 min median, 10 at p90 | at the head, yes: the full check holds all five parts |
| step session, at its head | `nix flake check` (46 checks, every output evaluated) | everything, built in the sandbox from the committed tree | 5 min median, 11 at p90, plus the wait for the lock | no (but see CI) |
| step session, by its areas | `nix build .#checks…NAME` (manual, export, Rocq, web) | one area | seconds to minutes | no: after a flake check of the same tree they are lookups |
| planning session, stacking | the gate and `nix flake check` on the combined head | as above, for code from two chains | the two again | the gate, yes; the flake check is a lookup when the tree is a session's checked head, real work when chains met |
| planning session, review | `nix build` of the Rocq checks | the certificates and the library | a lookup after a flake check | no |
| planning session, the client | the client's `nix flake check` under the lock | the web page | 14 min 39 s (2026-11-15) | yes against the client's CI, which follows |
| `ci.yml`, job `check` | `nix flake check --keep-going` on one runner | everything | 40 to 60 min when the core crate changed (43944277827: 41 min; 43962037921: 41 min; 44705050006 on `main` today: over 60), 1 to 2 min when all cached | yes against the step session's local check (another machine, nothing shared): unavoidable while this machine does not read the cache |
| `ci.yml`, job `advisories` | `cargo deny check advisories` (online), both locks | advisories | 1 min | no |
| `docs.yml` | `nix build .#doc`, deploy to Pages | the site | 1 to 7 min beside CI | yes: the same derivation as the check `doc`, built twice on every push |
| `kani.yml` | `nix build .#kani-harnesses` | the checker's integers | 31 to 37 min, weekly and on its paths | no |
| `release.yml`, job `check` | the tag's checks, then `nix flake check` | everything again | 1.0 and 1.3 min at 0.4.0 and 0.5.0 (cached); a full run when the tag comes before `main`'s CI ended | in name: a lookup when `main`'s run finished |
| `release.yml`, `packages` (`packages.yml`) | Linux by Nix on two runners, macOS and Windows by rustup on four | the shipped files | 13 to 19 min wall | no |
| `install.yml`, on push (paths), weekly | `packages.yml`, then the chapter's commands on 11 systems | installation | 10 to 28 min | no |
| `install.yml`, after each Release | `packages.yml` again, then the same | the released version's installation | 15 to 19 min, six builds of 5 to 14 min | yes: the six builds the release run made minutes before |
| `comparison.yml` | the comparison, by dispatch | other provers | hours | no |
| the client's `ci.yml` | `nix flake check` (its e2e 6.4 of 7.2 min), Pages | the web page | 7 to 15 min | no within it |

Within the flake check nothing repeats for no gain: `test-CRATE`
(release profile, doc tests) and `test-debug-assertions-CRATE` (debug
assertions and overflow checks, which the release profile drops) test
two builds; `clippy-core`, `wasm` and `web-clippy-wasm32` lint for
different targets; `package` builds the command from its crates.io
package, `build` from the workspace (each catches what the other
cannot); `doc` and `doc-private` document different items.

## The duplicates and what was done

| duplicate | what was done | what still covers it |
|---|---|---|
| `docs.yml` built `.#doc` beside CI's check `doc` | removed; the `docs` group uploads the site it built, `ci.yml`'s job `pages` deploys it after every group passed | the check `doc` and `manual-links`, as before; the deployment now also waits for them |
| `release.yml` ran `nix flake check` again at the tag | it calls `checks.yml`, a lookup of `main`'s uploaded results; its packages build beside the checks | the same checks, every one, on the tag's commit; crates.io and the GitHub release still wait for them |
| `install.yml` after a Release built the six packages again | it installs the release run's own artifacts | the same install jobs on the same systems, now on the files published; on push and weekly it still builds its own |
| the gate beside the full check at one head | the gate is left out at a head that gets the full check (`CLAUDE.md`'s table) | the full check holds all five of its parts (below) |
| the planning session's gate and `nix flake check` on a stack | one full check: `plan/push-ci.sh` on GitHub's runners, or `nix flake check` here as the fallback | the same checks |
| a check built here and again by CI | `ci/NAME` branches upload to the cache, so `main`'s run after a branch check is a lookup; this machine still builds its own (it does not read the cache: for the author) | unchanged |
| the client's local check before its CI | kept; a `ci/` branch there would remove it (below) | |

## The CI design

Sources, read at their current versions: nix-community/nix-github-actions
(`default.nix` and README at `f4158fa0`, 2025-01-21: a flake output
`githubActions.matrix` of entries `{name, system, os, attr}`, read by a
first job with `nix eval --json` and fed to `strategy.matrix` with
`fromJSON`); GitHub's docs on reusable workflows (`workflow_call`,
`secrets`, `inputs`), on matrices (`fail-fast`) and on job dependencies;
the pinned `actions/download-artifact`'s `action.yml` (`run-id`,
`github-token`); the web client's own CI, which already deploys Pages
from a job after its checks.

- **The matrix comes from the flake.** `modules/ci.nix` defines
  `githubActions.matrix` in nix-github-actions' shape, but with a list
  of checks per entry. The library itself makes one job per check: 46
  jobs, each installing Nix and fetching the toolchain and the
  dependencies' builds (about 700 MiB) to build one check, past the
  free plan's 20 jobs at once. So the grouping is written in `ci.nix`
  (about 20 lines, no new input), and named as the library names it so
  that a reader who knows the pattern finds it.
- **Groups.** Twelve: the three longest checks alone (`features`,
  `test-core`, `test-debug-assertions-core`); the checks that build one
  package together (the command with `export`, `rocq`, `rocq-deduxit`,
  `rocq-deduxit-library`, `wire-schema`, `check-certificate`; the
  harness with `bench`, `ratchet` and its tests; the web module with
  `web-features`, `budget`, `web-wasm-tests`, `web-clippy-wasm32`,
  `web-bindgen`; the site with `doc`, `doc-private`, `manual`,
  `manual-links`), so that no derivation is built on two runners; each
  crate's three Rust checks together (`cli`, `web`, `compat` with
  `package` and `msrv`); the library on wasm32 (`web-core-wasm32`,
  `wasm`); and `rest`, every check no group names (the lints and text
  checks), so a new check is never left out. An assertion fails the
  evaluation when a group names a check that does not exist or one
  twice.
- **Evaluation beside the builds.** `nix flake check` also evaluates
  every other output (packages no check builds, the devshells, the
  apps, the assertions at evaluation). The job `evaluate` keeps that:
  `nix flake check --no-build`, after a plain `nix eval` of every
  package's and shell's `drvPath`, since `--no-build` fetches nothing
  the evaluation reads (`kani.nix` reads the nightly toolchain's
  `Cargo.lock`; the first probe run failed there).
- **One workflow, three callers.** `checks.yml` is a reusable workflow;
  `ci.yml` calls it on pushes to `main` and `ci/**`, pull requests,
  weekly and by dispatch, with the inputs `push` (upload to Cachix:
  not for a pull request) and `pages` (on `main`); `release.yml` calls
  it with neither.
- **Fail-fast off**, so one run reports every failing group.
- **The cache** stays the store the jobs share: a group's job uploads
  what it built (daemon mode), so a later run, or the release, fetches
  it. On a lock or toolchain bump every group that needs the
  dependencies' builds makes them itself, in parallel; a first stage
  that builds them once would add its own time to every run, for the
  rare bump.
- **Pages** is deployed by `ci.yml`'s job `pages`, after every group
  passed, from the artifact the `docs` group uploads: the site is built
  once (`docs.yml` built the same derivation beside CI) and a `main`
  that fails a check is no longer published. It is the client's
  pattern. `pages: write` and `id-token: write` are that job's alone.
- **Concurrency** is unchanged: a newer push cancels the run of the
  same ref. A deployment under way is then cancelled too, which Pages
  survives with the site it had; the newer run deploys.
- **The release** checks through `checks.yml`, which on the tag of
  `main`'s commit fetches the results `main`'s run uploaded (the old
  `nix flake check` took 1.3 and 1.0 minutes for the last two releases
  for the same reason), and builds what a tag pushed early finds
  missing, in parallel. Its packages are built beside the checks
  instead of after them: they publish nothing themselves, crates.io's
  job still waits for the checks, and the GitHub release for both.
- **`ci/NAME` branches** run the same CI and upload to the cache, so
  the planning session can check a chain on GitHub's runners and
  `main`'s run after it is a lookup (`plan/push-ci.sh`). Pushing from
  a branch to the public cache is new: until now only `main` uploaded.
  The branches are the planning session's, under the author's standing
  yes for pushes of branches; a fork's pull request still gets no
  secret. (departs from the design: `ci.md` said only `main` uploads.)
- **Install after a release** downloads the release run's artifacts
  (`run-id` of the `workflow_run`, `actions: read` on the jobs that
  download) instead of calling `packages.yml` again: six builds, about
  50 runner-minutes, and the jobs now install the very files published.
  After a failed release they are skipped, since nothing was published.

## Measured times

Before and after on the same change: two branches off `main@origin`
(`xuptrvts`), each with one comment line added to `core/src/lib.rs`, so
that every crate's checks build again, as draft pull requests (no
upload to the cache, so neither helped the other), pushed at the same
minute:

| run | CI | wall time |
|---|---|---|
| 44713189669 (`ci-speed-before`, PR) | old: one job, `nix flake check` | 55 min 30 s |
| 44713185396 (`ci-speed-probe`, PR 2) | new: the matrix | 9 min 32 s |
| 44705050006 (`main`, today, the release's preparation) | old | 66 min 16 s |
| 44717258314 (`ci/ci-speed`, push, the whole chain) | new, with the evaluation fixed, uploading | 9 min 15 s, every job passed |

On the same change the matrix took a sixth of the time: 9 min 32 s
against 55 min 30 s, both runs passing. A run with every result in
the cache took 1 to 2 minutes before; the matrix's should take as long
(each group only fetches), which no run here measured.

The matrix run, job by job (each job's builds, from the log):

| group | job | its longest build |
|---|---|---|
| `core-tests-debug-assertions` | 9.0 min | `test-debug-assertions-core` 8.2 |
| `core-tests` | 8.5 min | `test-core` 7.7 |
| `features` | 8.2 min | `features` 7.4 |
| `core-wasm32` | 8.2 min | `web-core-wasm32` 7.4 |
| `compat` | 7.8 min | `package` 7.0, `test-compat` 6.2 |
| `web` | 6.4 min | `test-debug-assertions-web` 5.8 |
| `bench` | 6.3 min | the harness's tests 2.8, the package 2.5, the ratchet after it |
| `cli` | 6.2 min | `test-debug-assertions-cli` 5.4 |
| `cli-outputs` | 4.8 min | the command 2.5, then its checks |
| `web-package` | 3.7 min | the module 3.0 |
| `docs`, `rest` | 1.0 min each | |
| `the groups of checks` | 0.4 min | before the groups start |
| `nix flake check --no-build` | 0.6 min | failed in that run (the evaluation fix came after) |

Each job spends about a minute before its first build (Nix installed,
the flake's inputs and the toolchain and dependencies' builds fetched
from the caches). The five long groups end within 1.2 minutes of each
other, so splitting one further gains little: the floor is the longest
single check, `test-debug-assertions-core`, about 8 minutes of building.

The run on `ci/ci-speed` showed the rest working on GitHub: a push to a
`ci/` branch starts CI without a pull request; the job `evaluate` now
checks every output (`apps` to `packages`, `kani` and
`kani-harnesses` among them) and ends with "all checks passed!" in 71
s; the groups upload what they build (`Pushed …-deduxit-cli-test-0.6.0`
in the group `cli`); the job `pages` is skipped off `main`. Locally,
`nix flake check` passed at the chain's head before the rebase (7 s:
the chain changes no Rust, and this machine had built `main`'s checks)
and at the report's commit after it (458 s, under the lock).

## The local procedure

What each local run catches, and what else catches it:
- The gate's five parts (Clippy over the workspace, `cargo test
  --workspace` in the dev profile, both `cargo hack` runs, the ratchet)
  are all in `nix flake check`: `clippy-CRATE` per crate (the core
  crate's with every feature, which is what the workspace's feature
  unification gives it), `test-debug-assertions-CRATE` (the tests with
  debug assertions and overflow checks, as the dev profile has them)
  and `test-CRATE` (the release profile, and the doc tests), `features`
  (the same two `cargo hack` runs), `ratchet` (the same counts against
  the same ceilings). The full check also builds from the committed
  tree in the sandbox, which the gate cannot. So at a head that gets the
  full check, the gate is the same work twice; between heads it is the
  quick incremental check it was made for.
- A `nix build .#checks.x86_64-linux.NAME` after `nix flake check` of
  the same tree is a lookup in the local store: the planning session's
  Rocq checks after a flake check cost nothing, and naming them is no
  duplicate.
- What did repeat: the gate beside the full check at one head (in each
  step session, and again in the planning session's stack check), and
  every check built here and again on GitHub, whose results this
  machine does not fetch (its only substituter is cache.nixos.org).

The procedure now (`CLAUDE.md`'s verification table, `plan/handoff.md`
items 5 and 7):
1. A step session runs the gate at each commit that touches code, and
   `nix build` of the checks the table names for its areas.
2. The full check runs once per head. For a stack, the planning session
   runs `plan/push-ci.sh NAME HEAD` in the background: it pushes
   `ci/NAME`, waits for that CI run and exits with its outcome; the run
   uploads to the cache, so `main`'s run after `plan/push-main.sh` is a
   lookup. `nix flake check` here under the lock stays the fallback,
   and is what a brief asks for when the session must see the full
   result before it ends; that head then skips the gate.
3. The text checks alone for a stack where only plan files came
   between, as before.

Nothing stops being caught: the gate's parts are in the full check, and
the full check is the same set of derivations whether it runs here or
on GitHub (the matrix plus `evaluate` is `nix flake check`).

## The web client

Left as it is. Its CI is one job of 7 to 15 minutes, and in the run
measured (43961510801, 2026-11-16) the flake check took 7.2 of its 8
minutes, of which the e2e check took 6.4: every other check of the
client builds in under half a minute, in parallel on the one runner,
and its Rust comes from deduxit's cache. A matrix would leave the e2e
check as long as it is; only splitting it would help: Playwright's
`--shard=i/N` as N checks `e2e-1`… on N runners would bring the run to
about 3 or 4 minutes. That needs more than the workflow: the run keeps
its store in GitHub's Actions cache, saved by the one job that roots
`ci-cache` (which holds every check's result), so N jobs would each
restore 3.5 GB and the store root would have to change, and every local
`nix flake check` would start N servers and N browsers at once. Three
or four minutes a run is not worth that today; it is the change to make
if the e2e suite grows. Nothing in it runs twice: the steps after the
flake check (`ci-cache`, the site) are lookups. The one repetition is
the planning session's local check of the client before its push,
which the client's CI then repeats; a `ci/` branch there, as here, would
remove it.

## Review

`fresh-reviewer` (Fable 5.2 at its `medium`), once, on `uktnswln`,
`zrkvumso`, `uspksmlk`. It found nothing that lets a check go uncaught:
the groups give 46 of 46 checks, none missing or twice; the gate's five
parts are in the flake checks; the release still waits for every check
before crates.io and the GitHub release; Pages deploys from `main`
alone, after the checks; the pins, permissions and `persist-credentials`
are as `ci.md` asks; `push-ci.sh`'s flags exist in jj 0.47.1 and gh
2.104.0. Its findings and what was done:
1. The job `matrix` installed Nix without `cachix-action`, which
   `ci.md` asks of every such job. Added, read only (`uktnswln`).
2. `CLAUDE.md`'s command list and `modules/workspace.nix` still named
   the Docs workflow. Both now say CI publishes the site (`uspksmlk`,
   `uktnswln`).
3. (question) The `evaluate` job as fixed had not run on GitHub. The
   run on `ci/ci-speed` settles it (Measured times).
4. (question) Install passed `github-token` on every event, which on a
   push takes download-artifact's API path against the run in progress,
   never exercised. The token is now passed after a release alone; on
   push and weekly the downloads are as they were (`zrkvumso`).

## For the author

- **Let this machine read the cache** (a setting of the author's NixOS
  flake: `linlog-prover.cachix.org` and its key among `substituters` and
  `trusted-public-keys`, or `nix run nixpkgs#cachix -- use
  linlog-prover` as a trusted user). Then a local `nix flake check`
  after a CI run of the same tree, or of `main`, fetches what CI built
  instead of building it again: the last duplicate this report could
  not remove from the repository.
- **Cachix uploads from `ci/` branches** (departs from the design:
  until now only `main` uploaded to the public cache). A `ci/` branch is
  pushed by the planning session alone; what it uploads is deduxit's
  own builds, as `main`'s are.
- **Pages deploys only a checked `main`**, after every group passed,
  about ten minutes after a push instead of one to seven. A `main` that
  fails a check no longer replaces the site.
- No repository setting, secret or runner type changes: Pages already
  deploys from GitHub Actions, the `github-pages` environment already
  allows `main`, and every job runs on `ubuntu-latest` as before (twelve
  at once for the checks, within the free plan's twenty).
- **A follow-up, not done here:** the after-release job "from the
  release" of `install.yml` fails on GitHub's API rate limit (`curl` of
  `api.github.com/…/releases/latest` without a token: 403 in runs
  43904365236 and 43735074378, after 0.4.0 and 0.5.0). Passing
  `GH_TOKEN` to that request, or `gh release view`, would fix it.
