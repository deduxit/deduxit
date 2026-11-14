# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

<!--
Maintainer note (stripped before it reaches Claude's context). This file is
loaded into every session, so each line is paid in each: keep it to commands,
rules, conventions and pointers. Guidance for one area of the tree goes in
`.claude/rules/<area>.md` behind a `paths:` glob; procedures go in
`.claude/skills/<name>/`. `.claude/rules/claude-infra.md` explains the split.
-->

## Project

deduxit is a collection of logic suites, linear logic first: it decides, checks, proves step by step
and exports sequents of classical and intuitionistic linear logic and
of cyclic MLL (the order of the formulas kept), and
decides ordinary propositional logic through its embeddings (the
manual's "What exists and what is planned", `docs/src/status.md`). It is written in Rust with
WebAssembly in mind for the `deduxit-web` crate: the library reads
no clock and uses threads only behind a feature. Performance is a stated
goal: the data structures are compact and cache-friendly.

Workspace crates:
- `core/` is package **`deduxit`**: all logic. The linear suite is the
  module `deduxit::linear` (sequents, the forest, proofs and the checker,
  nets, the search, the readers and families); `ordinary` and `lambda`
  are suites beside it, and what they share with it stays at the root
  (the errors, the limits, the settings, the exports, the wire forms).
  It has eight optional
  default features, `parse` (the text parser, the LLTP reader and the
  generated families),
  `serialize` (serde), `interactive` (step-by-step proving),
  `latex`, `typst`, `svg` and `rocq` (the exports), and `sat` (batsat:
  the SAT-based engine `ordinary::sat`), and three off by
  default, `parallel` (rayon: the search on a thread pool, off for
  wasm), `png` (resvg) and `pdf` (krilla), which render the SVG with
  fonts the caller gives; the CLI enables the last nine.
- `cli/` is package **`deduxit-cli`**, library **`deduxit_cli`** and binary
  **`deduxit`** (one call into the library; `doc = false` because it shares
  the core crate's name): `prove` (one sequent, or a batch of many),
  `check`, `interact` and `seq print|json|fragment`. README is a front
  page; the manual under `docs/` (mdBook, `docs/src/SUMMARY.md` its
  chapters, published at `deduxit.github.io/deduxit/manual/`) shows every command and
  format and the main flags with their output, and `cli/tests/readme.rs`
  runs those examples, README's and the guide's; `deduxit <command> --help` is the doc comments of
  `cli/src/argument_parsing.rs`, every flag that sets a key naming
  it. The guide for AI models, `skills/deduxit/` (an Agent Skill:
  `SKILL.md` the summary, `reference.md` the full reference, served on
  the rustdoc site as `llms.txt` and `llms-full.txt`), describes the
  command and the library for models that use deduxit (not a skill of
  this repository's Claude Code setup); the same test runs its examples
  and checks every name it uses. Exit status 0 proved/valid, 1 unprovable/invalid, 2 error, 3
  unknown, 130 at a second Ctrl-C.
- `bench/` is package **`deduxit-bench`** (not published, `doc = false`):
  the benchmark harness, the baselines under `bench/results/DAY/`
  (`bench/RESULTS.md`, `bench/COMPARISON.md`) and the focused engine's
  target set (`bench/targets.sh`, `bench/TARGETS.md`).
- `compat/linlog/` and `compat/linlog-cli/` are the last releases under
  the names the crates had up to 0.2.0, both 0.3.0 and never released
  again: `linlog` re-exports `deduxit` (its root and `deduxit::linear`),
  and `linlog-cli`'s binary `linlog` says once on standard error that it
  is now `deduxit` and runs `deduxit_cli::main`.
- `web/` is package **`deduxit-web`** (not published): the WebAssembly
  bindings of a web page, the library for `wasm32-unknown-unknown`
  without `parallel` behind calls of JSON in and JSON, SVG or bytes out,
  with a browser tab's defaults; `modules/web.nix` builds the package
  `deduxit-web` (wasm-bindgen's CLI at the lock's version, `wasm-opt`,
  cargo-about's notices) that the client's repository takes, and runs
  the tests under Node. `plan/web/README.md` is the web front end's plan.
- The Rocq library **`rocq-deduxit`** (logical root `Deduxit`, its
  linear logic under `Deduxit.Linear`) lives in its own repository,
  `github.com/deduxit/rocq-deduxit` (`../rocq-deduxit` beside this checkout, with its own CLAUDE.md and rules; under `rocq/`
  here until 2026-11-02): the one-sided calculus of linear logic with Mix and
  weakening as parameters and the two-sided calculus of intuitionistic
  linear logic and its affine variant as inductives, deduxit's proof
  terms as datatypes, checkers as functions and the theorems that a term
  they accept proves its sequent, which the certificates of the
  exporter's second kernel (`--kernel deduxit`) rest on, and the
  invariants every derivation keeps (the classical reading, an atom's
  balance, the count equation) that its certificates of
  non-derivability rest on, and the
  reduction of Mix to `?(⊥⊗⊥)`, with its tests and its bridges to
  NanoYalla and Yalla. This flake takes it as the input `rocq-deduxit`,
  pinned by flake.lock to a commit of its `main` and following this
  flake's nixpkgs (one Rocq for both), and `modules/rocq-deduxit.nix`
  compiles the exporter's certificates for it against that package; a
  change of the library is made, checked and pushed there first, then
  the lock moves here (`nix flake update rocq-deduxit`, which moves that input alone), or
  a local build tries it with `--override-input rocq-deduxit
  path:../rocq-deduxit`.

`plan/README.md` is the proof-search plan the code follows, `plan/reports/`
what each step of it did. `proof-search-specifications.md` is the brief
written before the engines ("the spec" in the rules); its errata and the
rules files override it where the code departs from it.

## Where the rules are

A file under `.claude/rules/` loads when a file under its `paths` is
read, and holds that area's invariants, the reasons behind them, what a
check cannot catch, and where a new engine, format, family or option
plugs in. Read it before changing the area, and record there, one point
per bullet, what a later session must know and cannot see in the code.

| rules file | loads for | holds |
|---|---|---|
| `core.md` | `core/**` | the crate's layout and API in brief, its crate-wide rules, and the table of the fifteen module files `core-*.md` (sequents and parsing, the forest, proofs and the checker, derivations and interactive proving, the search's front door, the focused engine, the Horn engine, the inverse method, the parallel runtime, proof nets and the net engine, the exports, the batch, the benchmark inputs, ordinary logic, lambda-terms), each loaded for its own module |
| `cli.md` | `cli/**` | the command's layout, exit statuses, defaults, time limit, outputs, README's and the manual's examples as a test, extension points |
| `bench.md` | `bench/**` | the harness, the CSV columns as its interface, the baselines, the target set, extension points |
| `web.md` | `web/**`, `modules/web.nix` | the bindings' layout, why nothing is kept between calls, the tab's settings, the errors, the stack measured, the build and its checks, extension points |
| `flake.md` | `flake.nix`, `flake.lock`, `modules/**` | what each flake module holds and checks |
| `evidence.md` | `mutants/**`, `fuzz/**`, `.cargo/mutants.toml` | the mutation run's scope and passes, the fuzz targets, their seeds and runs |
| `ci.md` | `.github/**` | how the workflows are written and pinned |
| `claude-infra.md` | `.claude/**`, `CLAUDE.md` | this Claude Code setup |

## Commands

The dev environment is the flake's devshell (direnv, or `nix develop`): the
toolchain `rust-toolchain.toml` names (edition 2024; rustfmt, clippy and
rust-analyzer included), cargo-hack, cargo-deny, cargo-mutants,
cargo-nextest, valgrind, bacon and treefmt; the `fuzz` devshell has a nightly
compiler and cargo-fuzz for `fuzz/` alone. `menu`
lists its commands: `check`, `gate`, `tests`, `ratchet`, `launch` (the CLI), `manual` (the manual served locally),
`live` (bacon), `dev` (zellij) and `up`.

```sh
cargo build
cargo test --workspace
cargo test -p deduxit <test_name>                           # single test in core
cargo clippy --workspace --all-targets -- --deny warnings
cargo hack check --each-feature -p deduxit                  # each feature alone, all, none
cargo hack check --feature-powerset --depth 2 -p deduxit    # and every pair
cargo deny check                                           # licenses, bans, sources + advisories (online)
cargo run -p deduxit-cli -- <args>
cargo run --release -p deduxit-bench -- run --family partition-no=3,4 --engines focus,net
nix build .#lltp -o bench/lltp   # the LLTP library (1.1 GB, GPL-3.0, fetched at a pinned commit)
nix build .#iltp -o bench/iltp   # the ILTP library's 274 propositional problems (no licence stated, fetched at v1.1.2)
nix build .#qcover -o bench/qcover   # the qcover coverability suite's 176 .spec problems (Apache-2.0, the licence of its repository, fetched at a pinned commit); deduxit-bench run --spec
nix build .#checks.x86_64-linux.rocq-deduxit   # the exporter's certificates for --kernel deduxit against the pinned Rocq library (the input rocq-deduxit)
nix build .#checks.x86_64-linux.rocq-deduxit-library   # that library's own check: its tests, its theorems' assumptions closed
bench/targets.sh LABEL            # the focused engine's target set into bench/targets/LABEL.csv (two cores, about 15 s at 0.2.0, detached)
bench/net-targets.sh LABEL        # the net engine's target set into bench/targets/net-LABEL.csv (one core, about 10 s at 0.2.0, detached)
python3 bench/pinned-time.py run BASE LIFT RUNS.csv   # two builds' pinned CPU time side by side on cores 2 and 3, resumable (in a user unit under the machine lock; `summary RUNS.csv OUT.csv` after)
bench/baseline.sh --detach --fresh   # the whole baseline in a user unit, started when the author gives the machine (about 3 h; --arm waits for the night slot): bench/results/DAY/, bench/RESULTS.md

nix flake check   # build, clippy-CRATE, test-CRATE, test-debug-assertions-CRATE (the tests with debug_assert! on, which the release profile drops; CRATE is core, cli, bench, web or compat, each from its own crate's sources), doc, doc-private (the docs of private items, whose links must resolve), manual (the manual built, its figures drawn), manual-links (every link of the manual and README into the repository or the site resolves, offline), wasm (the library built for wasm32 with every feature but parallel), web-package, web-wasm-tests (the bindings' tests under Node), web-core-wasm32 (the checker's, size estimate's, memo's and memory account's tests on wasm32-wasip1), web-bindgen (the CLI's version against Cargo.lock's), deny, features (cargo-hack), package (both crates packaged and built as crates.io gets them), msrv (the workspace on the `rust-version` toolchain), export (the LaTeX and Typst output compiles, the SVG renders), rocq (NanoYalla checks the certificates), rocq-deduxit (the exporter's certificates against the pinned Rocq library), rocq-deduxit-library (the pinned library's own check, re-exported: its tests compiled, its theorems' assumptions closed under the global context), check-certificate (the app below accepts a certificate of each kernel and refuses a broken one), bench (the harness on the smallest problems), ratchet (the journeys' instruction counts against their ceilings), deadnix, actionlint, treefmt, conventions (licence headers, no comment, README line or line of the manual naming the plan), typos (spelling, typos.toml), shear (no unused dependency)
nix build .#checks.x86_64-linux.rocq   # the certificates alone: Rocq is a 1.2 GB closure from the binary cache
nix run .#check-certificate -- proof.v   # one certificate checked against the pinned NanoYalla or rocq-deduxit, the kernel read from its import lines (the manual's Rocq chapter leads with it)
gate              # what every commit that touches code passes: clippy, the tests (the behaviour lock among them), both cargo hack runs, the ratchet (about 4 min on four cores)
breaks 0.1.0      # what breaks a user of the library since that release: cargo-semver-checks, then cargo-public-api's diff (core.md says how to read them)
ratchet           # the journeys' instruction counts under callgrind against bench/ceilings.csv, and the cache model's misses beside them (bench/misses.csv, never a failure); ratchet --lower after a count went down, --record-misses after misses moved on purpose
mutants/run.sh    # mutation testing of the checker, the readers, the front door, the Horn refutations (cores 6-11, detached): mutants/baseline/
nix develop .#fuzz -c fuzz/run.sh   # every fuzz target until its coverage stops growing (nightly, cores 12-15, detached), after fuzz/seed.sh
nix fmt           # nixfmt, rustfmt, taplo, shfmt, shellcheck, ruff-format (a hook runs it on each file changed with Write or Edit; a file changed from the shell needs it run by hand before the commit, or the treefmt check fails)
nix build         # deduxit-cli, whose binary is result/bin/deduxit
nix build .#deduxit-web   # the web bindings: deduxit_web.js, deduxit_web_bg.wasm, LICENSE, THIRD-PARTY-NOTICES.txt
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test -p deduxit-web --target wasm32-unknown-unknown   # the bindings' tests under Node
nix build .#doc   # the Pages site, as the Docs workflow publishes it: the rustdoc and the manual under manual/
nix build .#manual   # the manual alone (docs/, mdBook), its figures drawn by bench/plots.py; `manual` in the devshell draws them and serves it
```

Verify as much as the change needs:

| the change | the proof |
|---|---|
| any commit that touches code | `gate` passes (capped: `systemd-run --user --scope -p MemoryMax=8G -p MemorySwapMax=0 taskset -c 2-5 nix develop -c gate`); a change of the behaviour lock's files or of a ceiling is a commit of its own that says why |
| any `.rs` edit | `cargo clippy …` and `cargo test --workspace` |
| touches `#[cfg(feature = …)]` or `[features]` | add `cargo hack check --each-feature -p deduxit` and `cargo hack check --feature-powerset --depth 2 -p deduxit` (both cover `parallel`, which is off by default: `--each-feature`'s `--all-features` run is the one that differs from the defaults) |
| touches `bench/` or `core/src/linear/families.rs` | add `cargo run --release -p deduxit-bench -- run --all-families --timeout 5` for the verdicts (a `MISMATCH` in `summary` is a bug); timings only from `bench/baseline.sh` on an idle machine |
| changes how the focused engine searches, or must not (a refactoring) | add `bench/targets.sh LABEL` and compare `bench/targets/LABEL.csv` with the oracle, `bench/targets/47.csv` (`bench/TARGETS.md` says why it is the one): `deduxit-bench summary --against bench/targets/47.csv bench/targets/LABEL.csv --counters` lists the verdicts that differ and every row decided alike whose `nodes`, `splits`, `memo_hits` or `memo_entries` differ; on one thread they are a function of the input, so a decided row has them equal exactly when the search is |
| changes how the net engine searches, or must not (a retype of `ProofStructure`) | add `bench/net-targets.sh LABEL` and compare the columns `verdict`, `links` and `tests` of `bench/targets/net-LABEL.csv` with the oracle `bench/targets/net-47.csv` on every decided row (`summary --against … --counters` compares them) |
| changes what the command prints, or a console block of README or the manual | `cargo test --workspace` runs README's and the manual's examples (`cli/tests/readme.rs`); a change of output changes the chapter that shows it in the same commit |
| adds a command, flag or format | add it to its chapter of the manual (`docs/src/`) with a real invocation, as README's usage section was; `cargo test -p deduxit-cli --test readme` runs it and checks every name, and `nix build .#checks.x86_64-linux.manual .#checks.x86_64-linux.manual-links` builds the manual and resolves its links |
| touches `docs/`, `bench/plots.py` or a CSV a figure reads | `nix build .#checks.x86_64-linux.manual .#checks.x86_64-linux.manual-links`; the harness's chapters' names are `cargo test -p deduxit-bench --test manual` |
| changes a feature the guide for AI models describes (a command, flag, format, JSON form, exit status, verdict or library entry point) | change `skills/deduxit/` in the same commit, as the manual; `cargo test -p deduxit-cli --test readme` runs its examples, checks every command, flag, session command and settings key it names, and holds the summary to its budget |
| touches `search/parallel.rs`, `focus/parallel.rs` or `net::parallel` | `cargo test --workspace` covers them (the CLI depends on `parallel`, and cargo unifies features across a workspace run); `cargo test -p deduxit` alone needs `--features parallel` |
| changes the `--kernel deduxit` certificates, or moves the input `rocq-deduxit` | `nix build .#checks.x86_64-linux.rocq-deduxit .#checks.x86_64-linux.rocq-deduxit-library` (the exporter's certificates against the pinned library, and the library's own check with its theorems' assumptions), under the machine lock while a timed run may hold it; a change of the library itself is checked in its own repository (`nix flake check` there) |
| adds or changes a dependency | add `cargo deny check`. New deps must use a license `deny.toml` allows: EUPL-1.2, MIT, Apache-2.0 (± LLVM-exception), Unicode-3.0, Zlib, BSD-2-Clause or BSD-3-Clause |
| touches `web/` | add the bindings' tests under Node (`nix build .#checks.x86_64-linux.web-wasm-tests`, or the command above in the devshell) |
| touches the checker's, the size estimate's or the memo's integers | add `nix build .#checks.x86_64-linux.web-core-wasm32` (their tests on a 32-bit target) |
| `flake.nix`, `modules/`, `.github/`, the toolchain, a lock bump, or before a push | `nix flake check`, which runs all of the above |

## The flake

Dendritic flake-parts: `flake.nix` only declares inputs, and import-tree
loads every `.nix` file under `modules/` (a path segment starting with `_`
skipped) as a flake-parts module, one aspect per file contributing to
every output it needs; no file is ever added to an imports list.

## CI

`ci.yml` runs `nix flake check` and the online `cargo deny check
advisories` on every push to `main`, every pull request and weekly, with
nothing installed but Nix; `docs.yml` publishes the rustdoc of `main` to
GitHub Pages; `release.yml` releases a version tag (crates.io through
Trusted Publishing from 0.1.1 on, and a GitHub release). Every job that
installs Nix pulls from the public Cachix cache `linlog-prover`, and
the jobs on `main` push what they build, so a check whose inputs did not
change is fetched, not rerun. Locally, `nix run
nixpkgs#cachix -- use linlog-prover` (once, as a trusted user) does the
same for `nix flake check`; the flake sets no `nixConfig`.

## Version control: jj only

A jj repository, colocated with git only so that nix and the GitHub remote
`origin` (`github.com/deduxit/deduxit`) keep working. **Never run git, not
even to read.** Every operation goes through `jj`, including lock updates:
`nix … --commit-lock-file` commits through git. The `Bash(git *)` deny rule and
the machine's `claude-jj-guard` (git writes, `--commit-lock-file`) enforce this.

- Reading: `jj st`, `jj log`, `jj diff`, `jj show`, `jj file annotate`, `jj root`.
- **Commit thematically, without being asked**: one logical unit per change,
  committed as soon as it is done with `jj commit -m`, which describes `@` and
  opens a fresh change on top. `jj describe -m` only names `@`; `jj squash`
  folds `@` into its parent (the amend) and, when the parent is described,
  needs `--use-destination-message` or `-m`: without one it wants an editor,
  which in a Claude session is `false`, so it fails. Never `jj new -m`: it describes a new,
  empty change and leaves the work behind (a hook blocks it).
- Edits land in `@` retroactively. If `@` already holds unrelated work, run
  `jj new` before editing. Another session or the author may commit in the
  same working copy meanwhile, so run `jj st` right before every commit
  rather than assuming `@` and `@-` from the session's start.
- nix sees only files in the git tree, which any jj command updates: run
  `jj st` after creating a file, before a `nix` command that must see it.
- Every step session works in a jj workspace of its own
  (`../deduxit-wt-…`, made by the machine's `claude-jj-worktree`), which is a
  git worktree too, so nix sees it. `plan/housekeeping.py` lists stale
  workspaces, leftover directories and old changes off `main`, and the
  planning session cleans them up regularly (`plan/handoff.md`,
  "Housekeeping").
- A workspace whose commit another workspace rewrote updates itself on its
  next jj command (the machine's jj config).
- Two sessions rewriting one change at once leave it divergent (`X/0`,
  `X/1` in `jj log`):
  - Try `jj converge --no-interactive -r 'change_id(X)'`, then check in
    `jj log` that `X/0` is gone: when the command cannot decide, it only
    warns.
  - When it cannot decide (different descriptions), compare them (`jj
    interdiff --from X/0 --to X/1`) and `jj abandon` the one to drop.
- Subjects are short, imperative and capitalised, with no type prefix
  ("Fix bug in reachability analysis for exponentials"). Add a body only when
  the why is not obvious. jj writes commits unsigned (the machine's jj
  config): a global hook signs a session's new commits right after the
  jj command that made them and reports their new commit ids, and the
  push signs any still unsigned. A lapsed passphrase cache stops nothing:
  the hook says which commits wait, and the first jj command after the
  author unlocks, or the push, signs them. Cite change ids, or the
  commit ids the hook reported; never pass `--config
  signing.behavior=…`.
- Lock bumps are changes of their own, "Cargo update" and "flake.lock: Update".
  `/update-deps` updates every dependency of both repositories (the locks,
  major releases and exact pins, the toolchain, the fuzz nightly, the
  actions, the web client), verifying before it commits; the shell's `up`
  is the unverified shortcut.
- Work lands on `main` directly or through a GitHub pull request. Pushing is
  outward-facing, so only when asked: `plan/push-main.sh @-`, which moves
  `main` and pushes it, failing at once rather than waiting on a
  passphrase prompt when the SSH key is locked, and checks
  `main@origin` after (by change id: the push signs what is unsigned,
  which gives those commits new ids).
- The GitHub CLI (`gh`) is logged in with the author's own rights, over the
  organisations `deduxit` and `linlog-prover` (the old one, kept as a
  placeholder that redirects) and everything else the account reaches.
  Ask before every administrative task through it (a setting of the
  organization or a repository, Pages, Actions, branch rules, secrets,
  releases, a new repository, a workflow run): say what the call does and
  wait for a yes, which covers that task only. Deleting and transferring
  are the author's own acts. A permission rule asks before any `gh`
  command.

## Conventions

- Every source file starts with the license header, in the file's comment
  syntax: `//` in Rust, `#` in Nix, TOML, shell, Python and YAML.
  ```
  // deduxit © Fabian Lukas Grubmüller 2026
  // Licensed under the EUPL
  ```
  Prose and configuration carry none: Markdown (CLAUDE.md, `.claude/`,
  `plan/`, README), JSON, `.gitignore`, `LICENSE` and the lock files. The
  project is EUPL-1.2.
- Every text a person reads (README, the manual, doc comments and
  comments, the command's help and messages, the web page's text and
  teaching material) follows `.claude/rules/writing.md`, the author's
  standing instruction of 2026-11-09:
  - concise, plain language;
  - self-contained;
  - written for someone using it, an API change that would serve them
    asked of the author first;
  - free of the patterns that make text hard to read.
- Comments are short and targeted: they say why, or what the code cannot.
  Types and names carry the rest.
- Comments are self-contained: they make sense from inside this repository,
  with no references to other repositories, machines or conversations, and
  none to the Claude Code sessions, prompts, plan steps or decision numbers
  (`plan/`) that produced the code. A comment says what the code does or
  why; how the work was organised belongs in `plan/` and the jj history.
- Every Rust item gets a concise doc comment that a human understands at
  first read: a function says what it does and returns, not how; a type,
  field, variant or module says what it is. The workspace lints in
  `Cargo.toml` enforce it (rustc's `missing_docs` for public items, clippy's
  `missing_docs_in_private_items` for the rest). clap shows the docs on
  `cli/`'s argument types as `--help` text.
- The lints are the workspace's, in `Cargo.toml`: clippy's pedantic group
  and a list picked from the nursery and restriction groups, with the
  ones that fire today allowed under a comment that says so. A commit
  that fixes a lint's last site turns it on in the same change; a new
  allowance is an `#[expect(lint, reason = "…")]` at the site, never a
  crate-wide `allow`.
- Dependencies are welcome where they earn their place: prefer well-made
  library code over an ad-hoc implementation, and among candidates the more
  popular, better maintained and faster one. Each serves a particular reason
  and is scoped to it: only the crate that uses it, behind the feature that
  needs it (or in `[dev-dependencies]`), with only the crate features used.
  Never add one for its own sake.
- No item is dead in any feature combination: an item that only tests
  use is `#[cfg(test)]` (`all(test, feature = "parse")` where its tests
  parse), one that a feature's code uses is behind that feature's
  `cfg`, and a variant or field that only some features build or read
  takes `#[cfg_attr(not(feature = …), expect(dead_code, reason = …))]`.
  Both `cargo hack` runs and the test builds without default features
  stay free of warnings (the gate and the `features` check deny them in
  the `cargo hack` runs).
- `scratchpad*.md` are the author's gitignored notes. `scratchpad1.md` is about
  250 KB, so don't read it in full.

## Claude Code setup

The git and `jj new -m` guards, the jj notes and the commit signer are
the machine's (`claude-*` commands from the author's NixOS flake,
`modules/claude-code.nix`). `.claude/` is checked in: the formatter hook,
the registration of the machine's `claude-jj-worktree` (jj workspaces in
place of Claude Code's git worktrees for isolated agents and sessions,
`../deduxit-wt-NAME`, in which the planning session starts each step
session from the Claude app and whose empty leftovers
`claude-jj-worktree prune` removes), permission rules,
the `crate-source-explorer` agent (use it before guessing at a
dependency's API), the reviewers `fresh-reviewer` and the panel's
`panel-counterexamples`, `panel-argument` and `panel-integers`, a
`Notification` hook that logs a session waiting on a prompt for the
planning session (it runs in the Claude app and starts step sessions
there; `plan/handoff.md` says how), the `update-deps` and `new-tool` skills (the latter
whenever a crate or tool is added or adopted), the `graceful-stop`
skill (whenever the author says "stop", or a supervisor sends "Pause")
and the rules above.
Claude Code's built-in git instructions and status snapshot are off.
When the repo changes shape, amend `.claude/` and this file in the same
change.

## Compact instructions

When the context is compacted, keep the prompt file the session works
from (`plan/NN-*.md`) and where in it the work stands, the report and
checklist it keeps, every instruction from the author or the
supervising session that is not yet in a file, and what was verified
and by what (a killed or partial run is not a result). After a
compaction, read the prompt's section for the current work and the
checklist again before going on.
