# One command or several, one crate or several: deduxit

Research for the author, 2026-11-09. Read only: no repository file was
changed. It builds on `plan/notes/structure.md` (2026-11-08, "structure"
below), which compared the names and the rename, and on step 46's
`plan/notes/research/next/structure.md` ("step 46's note"), which read
the Cargo book, matklad's "Large Rust Workspaces", the perf book and the
Rust workspaces of comparable shape. Neither is repeated here: this note
adds the current guidance on each shape of the command and the library,
what each would cost this repository, and what should keep the name
linlog now that the umbrella is deduxit. Sources were read on
2026-11-09 and are numbered in "Sources"; [S12] means source 12.
"Inference" marks a conclusion of mine that no source states.

## 0. In one paragraph

One binary, `deduxit`, with the verbs first and the logic chosen by an
option (`deduxit prove --logic s4 …`), linear logic the default during
0.x, and no binary, subcommand or alias per suite. One library crate,
`deduxit`, with a module per suite (`deduxit::linear`,
`deduxit::ordinary`, `deduxit::lambda`, later `deduxit::modal` …) and no
Cargo feature per suite yet. It becomes a facade over `deduxit-*` crates
only when a trigger of section 2.4 fires. The earliest likely trigger is
step 67 or a suite written by someone else. The name linlog keeps the
frozen crates.io pages, the final "renamed" releases and the history,
and nothing that grows. The Rocq library is renamed with the rest,
before its opam pull request, because step 55 puts intuitionistic, not
linear, metatheory into it. structure.md's recommendation holds. This
note adds the evidence behind the parts it settled in a line (the
command's shape and the module layout), plus the costs and triggers it
did not cover.

## 1. The command

### 1.1 Current guidance

- **clig.dev** [S1] is the most-cited general guide.
  - Subcommands: "If you’ve got a tool that’s sufficiently complex, you
    can reduce its complexity by making a set of subcommands." Closely
    related tools "you can make them easier to use and discover by
    combining them into a single command", with RCS versus Git as the
    example. Subcommands are "useful for sharing stuff—global flags,
    help text, configuration, storage mechanisms".
  - Consistency: "Be consistent across subcommands" ("Use the same flag
    names for the same things, have similar output formatting").
  - Two levels of subcommand (a noun and a verb, as in `docker container
    create`) are for software with "lots of objects and operations that
    can be performed on those objects". "Either `noun verb` or `verb
    noun` ordering works, but `noun verb` seems to be more common."
  - Distribution: "If possible, distribute as a single binary."
  - Naming: "Make it a simple, memorable word. But not too generic, or
    you’ll step on the toes of other commands", and "Keep it short".
  - Aliases: "There’s nothing wrong with aliases—saving on typing is
    good—but they should be explicit and remain stable." Also: "Don’t
    have a catch-all subcommand".
  - The guide says nothing on plug-ins.
- **The Rust CLI book** [S2] gives no rule on how many binaries to
  ship. It notes that `cargo install` "compile[s] all the binary targets
  it contains", that `cargo install` suits tools "targeted at other Rust
  developers", and that one should "Start with `cargo install` and add
  binary releases before finally distributing your tool using system
  package managers".
- **clap 4.6.7**, the version `Cargo.lock` pins [S3] (read from the
  local registry copy; it matches docs.rs):
  - `Command::multicall` builds "a single executable that contains a
    variety of applets, and decides which applet to run based on the
    name of the file". Links are made "by creating hard links or
    symbolic links to it". The docs list three uses: "Easy
    distribution", "Minimal binary size by sharing common code", and
    shells or REPLs. They add that "Applets are slightly semantically
    different from subcommands".
  - `allow_external_subcommands` is the door for plug-ins. It warns that
    "a truly unexpected argument … will **not** cause an error and
    instead be treated as a potential subcommand".
  - `next_help_heading` sets the heading for the flags that follow. It
    is the tool for grouping one long help text by suite.
- **Completions.** clap_complete 4.6.11 [S4] generates static scripts
  (`generate`, `generate_to`). Its "clap-native completion system"
  (`CompleteEnv`, `ArgValueCompleter`) is still behind the feature
  `unstable-dynamic`. A static script cannot hide the modal flags after
  `--logic linear` (inference from the split; the page says nothing on
  values that depend on other arguments). The command ships no
  completions today (`grep` finds no `clap_complete` in `cli/`).
- **cargo cannot install a binary under a second name.** The issue
  "Provide a way to generate hardlinks for binaries under different
  names" (rust-lang/cargo#8213, opened 2020) was closed as "not
  planned" on 2026-06-15. The team cited "concerns about the potential
  complexity and highly-specific nature", and named a BusyBox-style
  `--install` as a way out [S5].

### 1.2 The five shapes, with precedent and cost

The costs are those the brief lists, judged against this repository:
`cli/tests/readme.rs` runs every `$ linlog` line and checks each name
against `--help`. The guide `skills/linlog/` is held to a budget. The
manual has a chapter per suite already (`ordinary-logic.md`,
`lambek.md`, `cyclic.md`, `first-order.md`). The page's tab chooses a
suite, a method and parameters (D29).

**(a) One binary, verbs first, the logic an option** (`deduxit prove
--logic linear …`).

- Precedent:
  - Logic tools: Isabelle's `-l`, Carnap's `system=`, LWB, Z3, and TPTP's
    logic in the file (structure §3).
  - The nearest linear prover, APLL: `-ill` "Choose ILL as the logical
    system. The system is LL by default" [S6].
  - Lolli, a Rust "linear logic workbench" on crates.io since 2025-12,
    has the verbs `parse`, `prove`, `extract`, `codegen`, `viz` and
    `repl` [S7].
  - Rocq 9 replaced `coqc`, `coqtop` and `coq_makefile` by one `rocq`
    with verbs (structure §3). The compatibility binaries now live in a
    package of their own, built for Rocq 9.3.0 in Fedora 45: "This
    package includes compatibility binaries to call Rocq through
    previous Coq commands like coqc coqtop" [S8], [S9].
  - Tools nearby that unified on one binary with verbs: Deno ("A full
    toolchain, no plumbing") [S10], Typst (`typst compile`, `typst
    watch`, `typst fonts`) [S11], uv and Bun (with an alias each, see
    (d)).
- Discoverability: one name to install and search for, and one `--help`
  listing every verb. A suite is found as a value of `--logic`, which
  `--help` lists with clap's possible values.
- Help text: `deduxit prove --help` grows with each suite. Group the
  flags by suite with `next_help_heading` ("Linear logic", "Modal
  logic"). Each suite's syntax is already shown after the help
  (`after_help = ORDINARY` today). A flag that does not apply is refused,
  as `--logic` refuses the linear mode flags now. A two-pass parse that
  hides other suites' flags once `--logic` is known is possible with
  clap (inference; not tried).
- Completions: one static script per shell. It offers every flag,
  whatever `--logic` says, until clap's dynamic completion is stable.
- Manual: the commands chapter stays the reference for the verbs. Each
  suite's chapter documents its `--logic` value and its flags, as the
  ordinary chapter does today. Only the name changes.
- Guide for AI models: one Skill, as today. Anthropic's current
  practice is a `SKILL.md` "under 500 lines" with per-domain reference
  files one level deep ("Pattern 2: Domain-specific organization",
  `reference/finance.md`, `reference/sales.md`) [S12]. That fits
  `reference/linear.md`, `reference/ordinary.md` and so on. One limit
  grows: the single `description` (at most 1,024 characters [S12]) must
  name every suite's triggers. If it no longer can, the guide can split
  into one Skill per suite while the command stays one (inference: the
  Skills need not mirror the binaries).
- The page's settings: a tab is one command line, since suite = `--logic`,
  method = a future `--method`, and parameters = flags. The settings
  keys already mirror the flags. A per-record `logic` in the batch's JSON
  Lines lets one batch mix suites, as TPTP puts the logic in the problem
  (inference).
- The readme test: unchanged except the program name it matches
  (`CARGO_BIN_EXE_linlog`, `program == "linlog"`, `"linlog "`).
- What a linear logician types: `deduxit prove "A, A -o B |- B"`, with
  `-i` for ILL, as with APLL and as with `linlog` today, because linear
  stays the default.
- A naming trap to settle: "classical" and "intuitionistic" each name
  two things. `-i` is intuitionistic *linear* logic, while `--logic
  intuitionistic` is LJ. Under one binary the values of `--logic` need
  to be unambiguous: accept `--logic linear` explicitly, keep `-i`, `-a`
  and `--mix` as the linear suite's parameters, and make the error for
  `->` or `/\` without `--logic` name the flag (inference).

**(b) One binary, a subcommand per suite** (`deduxit linear prove …`).

- Precedent:
  - Noun-first groups exist where each noun has its own set of verbs
    over its own resources: `uv pip`, `uv tool`, `rustup toolchain`,
    `gh pr` (structure §3), and clig.dev's `docker container create`
    [S1].
  - No logic tool among those read uses a subcommand per logic
    (structure §3).
- Help text and completions: the one real gain. Each suite's `--help`
  and static completions show only its own flags.
- Costs:
  - Every invocation gets one more word, including the linear default.
    Changing the default later becomes impossible without breaking
    `deduxit prove`, unless `deduxit prove` stays an alias of `deduxit
    linear prove`, which then needs two help texts.
  - The shared verbs, exit statuses (0, 1, 2, 3), formats and JSON forms
    are documented or generated once per suite.
  - The manual's commands chapter splits by suite.
  - The guide's examples and the readme test's name check gain a level.
  - The page has one stage with suite as a parameter (D29), so the page
    and the command would no longer share one shape.
- What a linear logician types: `deduxit linear prove "…"`, every time.

**(c) Separate binaries per suite** (`linlog`, `lk`, `modal` …), from one
package (several `[[bin]]`) or several packages.

- Precedent:
  - Separate binaries appear where the tools are separate products with
    separate audiences. Ruff and ty are developed in one repository but
    shipped apart; ty is in beta, "0.0.x" with no stable API [S13]. Oxc
    ships `oxlint` and `oxfmt` apart, `oxfmt` installed by `pnpm add -D
    oxfmt` [S14].
  - They also appear where there is a reason of build or role: E's
    `eprover-ho`, and Lean's `lean`, `lake` and `elan` (structure §3).
  - No source shows a logic toolkit with one binary per logic over one
    library.
- Distribution:
  - Several `[[bin]]` in one package: `cargo install` builds and
    installs all of them (structure §3). Each is a full binary, since
    each links the library and the embedded fonts. A release build of
    `linlog` is about 17 MB today (16 936 512 bytes, a local build of
    2026-11-03).
  - Several packages: a release, a crates.io name and a Trusted
    Publishing set-up each (structure §2.2).
  - Nix: nixpkgs' rule is that a package whose executables include no
    main one "should not set `meta.mainProgram`" [S15], so `nix run
    github:…` would need one flake app per binary (inference from
    [S15]).
  - Debian: "Two different packages must not install programs with
    different functionality but with the same filenames", and "one of
    the programs must be renamed" [S16]. Short per-suite names collide:
    `/usr/bin/nd` is already Debian's `nd`, "small command line
    interface to WebDAV servers" (trixie, 0.8.2) [S17].
  - cargo-binstall finds each binary by the `{ bin }` template, with
    names "inferred from the crate" [S18]. Whether it installs every
    `[[bin]]` was not stated.
- Costs: one help text, completion script, manual part, Skill and
  readme-test program list per binary. The shared verbs and exit
  statuses are repeated in each. The page stays one stage, so a page
  tab no longer maps to one command.
- What a linear logician types: `linlog prove "…"`. This is the one
  argument for (c), and it serves the linear suite only.

**(d) One binary plus an alias per suite** (multicall on `argv[0]`, or a
thin wrapper).

- Precedent:
  - rustup's proxies are one binary under many names (structure §3).
  - `bunx` is "an alias for `bun x`" and "auto-installed when you
    install `bun`" [S19].
  - `uvx` is a separate small binary. On Unix it `exec`s the `uv` found
    in its own directory with `["tool", "uvx"]` and its own arguments,
    and on Windows it spawns `uv` [S20].
  - uutils' coreutils builds "a multicall (BusyBox-type) binary, named
    'coreutils'", or the utilities as separate binaries [S21].
- Distribution of an alias by channel:
  - **cargo install** can make no link (cargo#8213, [S5]). The choices
    are a second full `[[bin]]` (about 17 MB more), a `uvx`-style
    wrapper (small, but it fails unless `deduxit` sits beside it), or a
    `deduxit --install-links` subcommand of the user's choosing,
    BusyBox-style (inference from [S3], [S5]).
  - **Nix**: one `ln -s` in `postInstall`, with `meta.mainProgram =
    "deduxit"` kept.
  - **The `.deb`, `.rpm` and Arch packages**: nfpm, which
    `modules/dist.nix` already uses, has `type: symlink` entries [S22].
  - **The release archives**: a symlink in the tarballs. Windows' zip
    holds no symlink, so it would need a copy or a `.cmd` file
    (inference).
- Costs: the alias is a second name in the manual, the guide, the
  completions (registered once per name) and the readme test. A symlink
  is no cargo target, so `CARGO_BIN_EXE_*` does not exist for it, and
  the test would have to make the link itself (inference). clig.dev's
  "explicit and remain stable" makes it a lasting commitment.
- Worth it when the alias saves typing on every call, as `uvx` and
  `bunx` do for a frequent action. A suite alias saves only
  `--logic X`, and for the linear default it saves nothing.

**(e) Plug-ins on `PATH`** (`deduxit foo` runs `deduxit-foo`).

- Precedent:
  - cargo, git and opam (structure §3).
  - GitHub CLI: an extension's repository "must start with `gh-` and it
    must contain an executable of the same name". "An extension cannot
    override any of the core gh commands", and one shadowed by a later
    built-in runs through `gh extension exec` [S23].
- When it fits: when someone else writes a suite in another repository,
  which D25 leaves open. The contract would be step 63's written suite
  protocol with its JSON Schema (inference). In clap it is
  `allow_external_subcommands` plus a lookup on `PATH`.
- Cost now: nothing if left until then. A built-in verb added later
  shadows a plug-in of that name (the `gh` rule above), so the built-in
  verbs should be settled before plug-ins exist.

### 1.3 Summary of the command

| | (a) verbs, `--logic` | (b) suite subcommands | (c) binary per suite | (d) aliases | (e) plug-ins |
|---|---|---|---|---|---|
| precedent in logic tools | Isabelle, APLL, Carnap, Z3, TPTP; Rocq 9's unification | none found | none over one library; separate products (ruff/ty, oxlint/oxfmt) | rustup, bunx, uvx (for actions, not logics) | cargo, git, gh, opam (third parties) |
| help and completions | one long help, grouped by heading; static completions offer every flag | per suite, clean | per binary, clean | as (a), plus one registration per name | each plug-in its own |
| manual, guide, readme test | rename only | a level more everywhere | one set per binary | a second name everywhere | a protocol chapter |
| packaging | one binary | one binary | full binary each, or a package each; name collisions (Debian 10.1) | cargo cannot link; nfpm and Nix can | none until a plug-in exists |
| the page (D29) | tab = command line | differs | differs | as (a) | as (a) |
| a linear logician types | `deduxit prove "…"` | `deduxit linear prove "…"` | `linlog prove "…"` | `linlog prove "…"` | – |

## 2. The library

### 2.1 Current guidance

- **Cargo book, features** [S24]:
  - Features "should be _additive_", and "A feature should not
    introduce a SemVer-incompatible change". Mutually exclusive
    features should be avoided; the first alternative named is "Split
    the functionality into separate packages".
  - "Because features are a form of conditional compilation, they
    require an exponential number of configurations and test cases to
    be 100% covered."
  - crates.io caps a version at 300 features.
- **Cargo book, workspaces** [S25]: members share one `Cargo.lock` and
  one `target`, inherit `version` and the rest from
  `[workspace.package]`, and dependencies from `[workspace.dependencies]`.
  `default-members` sets what a bare `cargo build` builds.
- **Rust API Guidelines, C-STABLE** [S26]: "A crate cannot be stable
  (>=1.0.0) without all of its public dependencies being stable", where
  public dependencies are "crates from which types are used in the
  public API". A facade that re-exports suite crates makes them public
  dependencies, so a `deduxit` 1.0 would need every re-exported suite at
  1.0 (inference from [S26]).
- **matklad**:
  - On the crate graph: "the shape of this graph affects the compilation
    time a lot", and "The most important property of a crate is which
    crates it doesn’t (transitively) depend on". A chain-shaped split
    gains little: "if you ended up with a graph like the first one, you
    are not winning much". A good layout has "a common vocabulary crate,
    a number of independent features, and a leaf crate to tie
    everything together" [S27].
  - On boundaries: "Boundaries have a high cost. They prevent changes"
    [S28].
  - "Large Rust Workspaces" (flat `crates/`, `0.0.0` for unpublished
    crates, publish deliberately) is in step 46's note.
- **Compile time**:
  - The perf book: "any large crates that serialize compilation should
    be broken up", as `cargo build --timings` shows [S29].
  - The parallel front end, which would cut a single crate's front-end
    time, is still not stable. The 2026 project goal plans a test suite,
    then "the feature stabilization process" [S30].
  - This repository's own figures (step 47's report, section 2):
    - a dev build after touching one file of `ordinary/` takes 3.0 s,
      of which `linlog` is 2.0 s;
    - a release rebuild after the same touch takes 116.3 s of a clean
      137.4 s, because fat LTO and one codegen unit relink every binary
      whole (`linlog-cli`'s binary 66.5 s);
    - `linlog`'s release codegen is 42.7 s of one unit.
    The report's own conclusion: a split "does not shorten the release
    relink; it can only shorten `linlog`'s own frontend (9.3 s
    release, 7.7 s dev)".
- **The cost of releasing many crates fell in 2025**:
  - Rust 1.90 (2025-09-18) made "`cargo publish --workspace`" publish
    "all of the crates in a workspace … in the right order (following
    any dependencies between them)" [S31]. The workspace's MSRV is 1.92
    (README), so the release job could use it. Step 46's note had found
    no source for the order.
  - release-plz automates release pull requests, per-package tags,
    git-cliff changelogs and "API breaking changes detection with
    cargo-semver-checks" for workspaces [S32].
  - cargo-release "Updates dependent crates in workspace when changing
    version" and has "Change detection" [S33].
  - Every new crate name still needs one publish with a token before
    Trusted Publishing works (structure §2.2).
- **cargo-semver-checks** [S34] has "many dozens" of lints but does not
  catch "breaking type changes, for example in the type of a field or
  function parameter", "breaking changes in generics or lifetimes", or
  "breaking changes that exist when only a subset of all crate features
  are activated". The last gap makes features per suite a blind spot
  for `breaks`, while separate crates would each be checked whole
  (inference).
- **docs.rs and facades**:
  - docs.rs builds each crate with the features named in
    `[package.metadata.docs.rs]` [S35].
  - rustdoc inlines a re-export whose source is private and otherwise
    shows a re-export, unless `#[doc(inline)]` forces inlining [S36].
    Bevy's docs.rs page shows its re-exported crates as modules
    (structure §2.1).
  - The feature badges moved in 2025: `doc_auto_cfg` was folded into
    `doc_cfg` (rust-lang/rust#138907, RFC 3631), and crates that kept
    the old gate failed on docs.rs [S37], [S38].
- **Precedents at a single maintainer's scale**:
  - BurntSushi's regex was split into `regex`, `regex-automata` and
    `regex-syntax` (2023), so that internals could be used by others
    and have "breaking change releases at a much faster cadence". `regex`
    became "a thin wrapper around the meta regex engine from the
    `regex-automata` crate" [S39]. ripgrep's `grep` is "ripgrep, as a
    library" over `grep-*` crates, and its README says it "isn't ready
    for wide use yet" [S40].
  - nom 8.0 (2025-01-25) split off `nom-language` to "hold tools more
    focused on language parsing than the rest of nom" [S41].
  - RustSAT keeps one main crate (`rustsat`, with features) and a
    `rustsat-<satsolver>` crate per solver interface (cadical, kissat,
    glucose, minisat), with no facade [S42]. The README gives no reason;
    each wraps a separate native solver (inference).
  - Lolli split by pipeline stage from its first release: seven crates
    (`lolli-core`, `-parse`, `-prove`, `-extract`, `-codegen`, `-viz`,
    `-cli`), no facade, no crate named `lolli`. The crates.io API shows
    946 downloads for `lolli-core` and about 245 for each of the others
    [S7], [S43].
  So the splits that held were made for a reason that showed up: an
  API others wanted on its own cadence, an area with its own audience,
  or a native dependency.

### 2.2 The three options against this repository

**(a) One crate `deduxit`, a module per suite, features optional.**

- Fits today:
  - `ordinary` and `lambda` reach the linear core (they decide through
    the embeddings into linear logic), so the suites are not
    independent yet.
  - Step 47 measured the ordinary suite at 4.6 KB of the wasm module, so
    a feature for it would save almost nothing.
  - One docs.rs page and rustdoc tree; one semver line; one `breaks`
    run; one Trusted Publishing set-up; one entry for `cargo add`.
- A feature per suite now would cost more than it saves:
  - The gate's `cargo hack --feature-powerset --depth 2` checks the empty
    set, each feature and each pair: 1 + 10 + 45 = 56 builds with
    today's ten features. Seven suite features would make it 1 + 17 +
    136 = 154 (inference, arithmetic).
  - The convention that no item is dead in any feature combination
    would have to hold for every shared item under every suite subset.
  - cargo-semver-checks does not see a break that only a feature subset
    shows [S34].
  - Add a feature only for a suite that brings a dependency or a
    measured size (the same triggers as section 2.4), and at that point
    prefer a crate.
- Module layout: if the crate is renamed, every path changes anyway
  (`linlog::…` to `deduxit::…`). Moving the linear items under
  `deduxit::linear` in the same release costs users one break instead of
  two. structure's "C" against "C1" was a choice of the author's
  sessions; this argues for C, if the rename lands in one release
  (inference).

**(b) A facade `deduxit` over `deduxit-*` crates behind features** (Bevy,
Polars, gitoxide).

- What it buys:
  - separately compiled and incrementally rebuilt suites in dev builds,
    once the graph is wide (matklad's vocabulary crate, independent
    suites, a leaf);
  - a crate that a third-party suite can depend on without the rest
    (`deduxit-kernel`);
  - suites with their own stability (an experimental suite stays 0.x in
    its own crate);
  - feature-gated suites for the page.
- What it costs this project:
  - Releases: one crate per suite plus the kernel. Lockstep versions
    (Typst, Polars, step 46's note) and `cargo publish --workspace`
    keep this mechanical. Each new name still needs a token publish and
    a Trusted Publishing entry.
  - C-STABLE ties a 1.0 facade to 1.0 suites.
  - Speed: the hot engines stay with the arena, the forest and the proof
    term (D17). Generics and inlining across a crate boundary need
    checking, though fat LTO recovers most of it in release builds (step
    46's note). The target sets' counters and the ratchet are the
    check.
  - The rules files, CLAUDE.md and the `cargo hack` runs change per
    crate.
- Why not now: the facade would gather one crate (structure §5 (B)),
  and the dependency graph would be a chain (ordinary → linear), which
  is the shape that gains little [S27]. The release relink, which
  dominates the release build, does not shrink (step 47).

**(c) Independent crates per suite, no facade** (RustSAT's solver
crates, Lolli).

- Right only for a suite that is its own product with its own audience
  and cadence (the Ruff and ty pattern, structure §2.1), or a binding to
  something heavy.
- Here the command and the bindings link every suite, the suites share
  a kernel, and the page is one stage (D29). Users and AI models would
  have to learn crate names per suite, and docs.rs would have no single
  entry.

### 2.3 Summary of the library

| | (a) one crate | (b) facade | (c) independent crates |
|---|---|---|---|
| fits the graph today (ordinary → linear) | yes | a chain: little gain | no |
| releases per version | 1 library + CLI | 1 + kernel + one per suite (mechanical since Cargo 1.90) | one per suite, no single entry |
| semver checking | one run, but blind to feature subsets | per crate, whole | per crate |
| docs.rs | one page | one page per crate, the facade shows modules | scattered |
| dev rebuild | `linlog` 2.0 s of 3.0 s (step 47) | shorter once the graph is wide | as (b) |
| release rebuild | 116 s, mostly relink | unchanged | unchanged |
| a third-party suite | must depend on all of it | depends on `deduxit-kernel` | depends on whichever |

### 2.4 When to move from (a) to (b)

Move when one of these fires, and not before:

1. **A second consumer of part of the library.** A suite written by
   someone else (D25), or step 67's inventory finding a kernel that
   two suites copied. The kernel then becomes a crate (`deduxit-kernel`,
   unpublished at `0.0.0` until a second suite crate depends on it, as
   step 67 fixes), and `deduxit` re-exports it.
2. **An independent suite.** The first non-embedded suite, modal (step
   65), is the first that does not depend on the linear core. Only then
   does the graph become wide enough for a split to help dev builds
   ([S27]; D25 waits for the same suite).
3. **A dependency or a build cost of its own.** A suite that needs a
   heavy or native dependency (a SAT solver, as in RustSAT [S42]), or
   one that `cargo build --timings` shows serialising the build [S29].
4. **A different cadence or stability.** A suite that should break more
   often than the rest, or the stable parts reaching 1.0 while a suite
   stays experimental (the regex-automata reason [S39]; C-STABLE
   [S26]).
5. **The page needs separately linked suites.** A wasm module per suite
   loaded lazily (D29's condition: a measured size, or a suite from
   another repository).

These are not triggers: the release build time (fat LTO relinks
everything anyway), the line count alone, and tidiness. matklad's "Boundaries
have a high cost" applies [S28].

## 3. What keeps the name "linlog"

- **The crates `linlog` and `linlog-cli` on crates.io keep the name, as
  frozen pointers.** A version "can never be overwritten, and the code
  cannot be deleted", and a deleted name is free to anyone a day later
  (structure §2.2). Their 0.1.0 and 0.2.0 pages, docs.rs/linlog and the
  0.2.0 README's links stay valid. One final 0.3.0 of each, with the
  description "renamed to deduxit", a README notice and a pass-through
  (gitoxide's `git-repository` precedent, structure §2.3), is the
  ecosystem's way to say so. The pass-through matters most for the
  command: `cargo install linlog-cli` then installs a `linlog` that runs
  as `deduxit`. It should be a dependency on the `deduxit-cli` library,
  so it works standalone (a full-size binary); a `uvx`-style exec would
  fail without `deduxit` beside it [S20]. It should print one line to
  standard error that names the new command (inference).
- **No module or crate `linlog` inside deduxit.** Suites are named for
  their logic (`ordinary`, `lambda`, later `modal`), so the linear suite
  is `linear`. Two more reasons:
  - A family with two prefixes has no precedent among the comparable
    projects (structure §2.1).
  - To linear logicians, "LinLog" already names Andreoli's 1992 fragment
    of linear logic, the target of the focusing normalisation. Dale
    Miller's overview: "LO is a subset of the LinLog" [S44]. A module
    `linlog` would read as that fragment, which a linear suite with a
    focused engine might one day expose (inference).
- **No documented `linlog` alias of the command.** For existing users,
  the final `linlog-cli` above covers the habit. The new packages carry
  `deduxit` only. An alias in the manual and the guide is a commitment
  [S1] for about 150 downloads and no known user. Rocq's compatibility
  binaries show where an alias belongs if it is ever wanted: a package
  of its own [S8].
- **The Rocq library: rename it before the opam pull request**, to
  `rocq-deduxit` with the logical path `Deduxit` and today's content as
  `Deduxit.Linear`.
  - The reason is the roadmap: step 55 puts the lemma behind LJ's
    Kripke countermodels into "the Rocq library" (`plan/55-…`, lines
    3–4, 26), and step 66 may put modal soundness there. That is not
    linear metatheory. The alternative is a second Rocq package later,
    with its own opam entry, Zenodo concept, flake input and release.
  - The pull request is unsent (structure §1), and the Rocq opam
    repository's README gives no rename procedure [S45]. The opam
    practice found is a transition package plus `flags: deprecated`,
    from secondary sources [S46].
  - Keep `rocq-linlog` only if the author holds to D25's "the Rocq
    library stays the linear library" and gives the intuitionistic and
    modal metatheory a library of their own.
  - `--kernel linlog` would follow the library's name.
- **Zenodo**: keep both concept DOIs (10.5281/zenodo.23448588 and the
  Rocq library's 10.5281/zenodo.23899109) and change the title from the
  first deduxit version on. A concept DOI resolves to the latest version
  (structure §4), so a citation of linlog's DOI reaches deduxit.
  Versions up to 0.2.0 keep their titles.
- **The flake**: `nix profile add …#linlog-cli`, as README 0.2.0 shows,
  records that attribute path. Keeping `packages.linlog-cli` as an
  alias of the new package for a release or two would spare an
  installed profile a broken upgrade (inference; how `nix profile
  upgrade` treats a vanished attribute was not checked).

## 4. Recommendation

- **The command: (a).**
  - One package `deduxit-cli` with one binary `deduxit`.
  - The verbs first (`prove`, `check`, `eliminate`, `interact`, `seq`).
    `--logic` chooses the suite: `linear` is the default through 0.x
    and is accepted explicitly. Each suite's parameters are flags that
    other suites refuse. A `--method` comes when a suite has a second
    method (D29), and suite-specific verbs refuse other suites.
  - Help grouped by suite with `next_help_heading`.
  - No suite subcommands, no binaries per suite, no aliases in the new
    packages.
  - Plug-ins (`deduxit-foo`) only when a third-party suite exists, on
    step 63's protocol.
- **The library: (a) now, (b) on a trigger.**
  - One crate `deduxit`: `deduxit::linear` (moved in the rename's
    release, so users break once), `deduxit::ordinary`,
    `deduxit::lambda`, and new suites as modules.
  - No features per suite until a suite brings a dependency or a
    measured size.
  - The facade over `deduxit-kernel` and `deduxit-<suite>` crates comes
    when section 2.4's first trigger fires, most likely at step 67 after
    the modal suite. Then lockstep versions, `cargo publish
    --workspace`, and release-plz or the existing job.
- **The name linlog**: the frozen crates with a final "renamed" release
  whose `linlog` runs `deduxit`, the Zenodo concept, the tags and
  history. Not a module, not a crate in the family, not a documented
  alias. The Rocq library is renamed with the rest unless D25's "linear
  library" is kept on purpose.

## 5. What it costs now and later

| part | now (at the rename's release) | later |
|---|---|---|
| command (a) | the rename only: binary, help, 244 console lines, the readme test's program name (structure §1); `--logic linear` accepted; help headings | each suite adds a `--logic` value, a heading and a manual chapter; static completions show every flag until clap's dynamic completion is stable |
| no aliases | the final `linlog-cli` with a pass-through (part of structure's estimate) | none |
| library (a) with `deduxit::linear` | structure's "about three sessions" (about two without the module move), with the D17 check for the move | nothing until a trigger; then the facade split at step 67 keeps users' paths (`deduxit::linear` re-exported, as `bevy::app`) |
| features per suite | none | when a suite brings a dependency: the feature, its `cargo hack` cost, the dead-code rule for it |
| facade (b), when triggered | – | a token publish and Trusted Publishing entry per new crate; lockstep releases through `cargo publish --workspace` (Cargo ≥ 1.90); `breaks` per crate; rules files per crate; the target sets and ratchet across the boundary |
| Rocq rename | about one session in `rocq-linlog` first (structure §5 (C) step 4), a GitHub rename (redirects), the Zenodo title | none for steps 55 and 66; without the rename, a second Rocq library at step 55 or 66 |
| flake alias | one attribute | drop it after a release or two |

## 6. Sources

Project files: `plan/notes/structure.md`,
`plan/notes/research/next/structure.md`, `plan/README.md` (D24–D32,
steps 47–73, the Status entries of 2026-11-08 and 2026-11-09),
`plan/reports/47-measurement.md` (section 2 and its notes for step 67),
`plan/55-kripke-countermodels.md`, `plan/66-labelled-modal.md`,
`plan/67-kernel-crate.md`, `README.md`, `CLAUDE.md`, `cli/Cargo.toml`,
`core/Cargo.toml`, `cli/src/argument_parsing.rs`, `cli/tests/readme.rs`,
`skills/linlog/SKILL.md`, `modules/dist.nix`, `target/release/linlog`
(size), and the local registry copy of `clap_builder` 4.6.7.

Outside, all read 2026-11-09:

1. Command Line Interface Guidelines: https://clig.dev/
2. Command Line Applications in Rust, packaging: https://rust-cli.github.io/book/tutorial/packaging.html
3. clap 4.6.7, `Command` (`multicall`, `allow_external_subcommands`, `next_help_heading`): https://docs.rs/clap/4.6.7/clap/struct.Command.html
4. clap_complete 4.6.11: https://docs.rs/clap_complete/latest/clap_complete/
5. rust-lang/cargo#8213: https://github.com/rust-lang/cargo/issues/8213
6. APLL (LL_prover) README: https://github.com/wujuihsuan2016/LL_prover
7. Lolli: https://github.com/ibrahimcesar/lolli
8. Fedora, `coq-core-compat`: https://packages.fedoraproject.org/pkgs/rocq/coq-core-compat
9. Rocq 9.0 release page: https://rocq-prover.org/releases/9.0.0 ; opam `coq-core` 9.1.1: https://opam.ocaml.org/packages/coq-core/coq-core.9.1.1/
10. Deno, all-in-one tooling: https://docs.deno.com/examples/all-in-one_tooling
11. Typst README: https://github.com/typst/typst/blob/main/README.md
12. Anthropic, Skill authoring best practices: https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices
13. ty README: https://github.com/astral-sh/ty
14. Oxfmt beta: https://oxc.rs/blog/2026-02-24-oxfmt-beta ; joint release tags (secondary): https://newreleases.io/project/github/oxc-project/oxc/release/apps_v1.31.0
15. nixpkgs `pkgs/README.md`, `meta.mainProgram`: https://github.com/NixOS/nixpkgs/blob/master/pkgs/README.md
16. Debian Policy, 10.1 Binaries: https://www.debian.org/doc/debian-policy/ch-files.html
17. Debian contents search for `nd`: https://packages.debian.org/search?searchon=contents&keywords=nd&mode=exactfilename&suite=stable&arch=any ; package: https://packages.debian.org/stable/nd
18. cargo-binstall SUPPORT.md: https://github.com/cargo-bins/cargo-binstall/blob/main/SUPPORT.md
19. Bun, bunx: https://bun.com/docs/pm/bunx
20. uv, `uvx.rs`: https://github.com/astral-sh/uv/blob/main/crates/uv/src/bin/uvx.rs
21. uutils coreutils README: https://github.com/uutils/coreutils/blob/main/README.md
22. nfpm configuration (symlinks): https://nfpm.goreleaser.com/docs/configuration/
23. GitHub CLI, `gh extension`: https://cli.github.com/manual/gh_extension
24. Cargo book, features: https://doc.rust-lang.org/cargo/reference/features.html
25. Cargo book, workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
26. Rust API Guidelines, necessities (C-STABLE): https://rust-lang.github.io/api-guidelines/necessities.html
27. matklad, "Fast Rust Builds" (2021): https://matklad.github.io/2021/09/04/fast-rust-builds.html
28. matklad, "Almost Rules" (2022): https://matklad.github.io/2022/07/10/almost-rules.html
29. The Rust Performance Book, compile times: https://nnethercote.github.io/perf-book/compile-times.html
30. Rust project goals 2026, parallel front end: https://goals.rust-lang.org/2026/parallel-front-end.html
31. Rust 1.90.0 announcement: https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/ ; `cargo publish`: https://doc.rust-lang.org/cargo/commands/cargo-publish.html
32. release-plz: https://release-plz.dev/docs
33. cargo-release: https://github.com/crate-ci/cargo-release
34. cargo-semver-checks README: https://github.com/obi1kenobi/cargo-semver-checks/blob/main/README.md
35. docs.rs metadata: https://docs.rs/about/metadata
36. rustdoc book, re-exports: https://doc.rust-lang.org/rustdoc/write-documentation/re-exports.html
37. RFC 3631, rustdoc cfg handling: https://rust-lang.github.io/rfcs/3631-rustdoc-cfgs-handling.html
38. "Doc_auto_cfg is gone" (users forum): https://users.rust-lang.org/t/doc-auto-cfg-is-gone-what-am-i-supposed-to-do/135070 ; datafusion's switch: https://github.com/apache/datafusion/pull/17845
39. Andrew Gallant, "Regex engine internals as a library" (2023-07-05): https://burntsushi.net/regex-internals/
40. ripgrep, `grep` crate README: https://github.com/BurntSushi/ripgrep/blob/master/crates/grep/README.md
41. nom CHANGELOG (8.0.0): https://github.com/rust-bakery/nom/blob/main/CHANGELOG.md
42. RustSAT README: https://github.com/chrjabs/rustsat
43. crates.io API: https://crates.io/api/v1/crates/lolli-cli , https://crates.io/api/v1/crates?q=lolli , and for `deduxit`, `deduxit-cli`, `linlog`, `linlog-cli`: https://crates.io/api/v1/crates/NAME
44. Dale Miller, "An Overview of Linear Logic Programming": https://www.cs.uoregon.edu/research/summerschool/summer02/lectures/llp.pdf
45. Rocq opam repository README: https://github.com/rocq-prover/opam
46. opam archiving and `flags: deprecated` (secondary): https://discuss.ocaml.org/t/proposed-package-archiving-policy-for-the-opam-repository/15713 ; the `depext` transition package: https://opam-5.ocaml.org/packages/depext

## 7. Not verified

- **Ecosystem behaviour**:
  - Whether cargo-binstall installs every `[[bin]]` of a package, and
    whether it keeps a symlink found in a release archive [S18].
  - How `nix profile upgrade` treats an entry whose flake attribute was
    renamed (section 3's flake alias rests on this).
  - Whether Rocq's compatibility binaries print a deprecation notice;
    Fedora's page lists no files [S8].
  - How the Rocq opam repository renames a merged package: no policy
    text was found. The transition-package practice comes from
    secondary sources [S46].
  - The Oxc joint release tags come from a third-party release tracker
    [S14], and ty's separation from Ruff is not explained by Astral in
    any source found [S13].
  - The exact rustc version that removed `doc_auto_cfg` (forum posts
    say 1.92) [S38].
- **Inferences that a trial would settle**:
  - The two-pass parse that would hide other suites' flags once
    `--logic` is known (clap's `ignore_errors` plus `mut_arg`) was not
    tried.
  - That static completions cannot depend on `--logic` follows from
    clap_complete's split [S4]; dynamic completion was not tried.
  - The `cargo hack` counts (56 and 154) are arithmetic on
    `--feature-powerset --depth 2`, not runs.
- **This repository's figures**:
  - The 17 MB binary is a local release build of 2026-11-03, not a
    release artefact; the shipped musl build may differ.
  - Step 47's timings are the report's (one run on the slower cores),
    not a fresh measurement.
- **LinLog**: Andreoli's fragment is confirmed through Miller's overview
  [S44]; Andreoli's 1992 paper itself was not opened.
- **The pass-through and its ranges**: whether a final `linlog-cli`
  0.3.0 depending on `deduxit-cli = "0.3"` keeps working as `deduxit`
  moves to 0.4 (it would not follow without a new release; Cargo's
  semver ranges, inference).
- **Session estimates** are structure's judgements, not measurements.
