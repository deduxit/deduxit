# Structure of a collection of logic suites

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

The author's question (plan/46, "The author's direction"): how to structure a project that is not one linear logic suite but a collection of suites for many logics, with one web stage that picks the calculus per tab, perhaps under a new organisation name. This note settles the structure only. The name and rename are another note's; the logics are others'.

## 1. What linlog has today

From `CLAUDE.md` ("Workspace crates"), `docs/src/architecture.md`, `.claude/rules/core.md` and the root `Cargo.toml`:

- One Cargo workspace (`resolver = "3"`, members `bench`, `cli`, `core`, `web`) in one repository, `linlog-prover/linlog`. `core/` is package `linlog` (about 110 000 lines of Rust by a line count of `core/**/*.rs`; `cli` 13 600, `bench` 6 700, `web` 4 200). The members inherit `version`, `edition`, `license`, `repository` from `[workspace.package]`, and `linlog` is declared once in `[workspace.dependencies]`.
- Published to crates.io: `linlog` and `linlog-cli` (release workflow `.github/workflows/release.yml`: `cargo publish --locked --package "$crate"` in order, through Trusted Publishing, with an environment named `crates-io`). `bench` and `web` are not published.
- Seven default features inside `linlog` (`parse`, `serialize`, `interactive`, `latex`, `typst`, `svg`, `rocq`) and three off by default (`parallel`, `png`, `pdf`). The wasm build is the library without `parallel`.
- Two other repositories already exist beside it, and both are separate artefacts that the flake pins: `rocq-linlog` (opam package `rocq-linlog`, logical path `Linlog`, with the checkers and soundness theorems the exporter's certificates rest on) and the web client `linlog-prover.github.io` (Astro and Solid). The client imports `linlog-web/linlog_web.js` and `linlog_web_bg.wasm?url` in one worker (`src/worker/linlog.worker.ts`): one wasm module for everything, built by the flake package `linlog-web` (`modules/web.nix`).
- A second logic already lives inside the crate: `core/src/ordinary/` (ordinary classical, intuitionistic and minimal propositional logic). Its design is the datum that matters here. It has its own hash-consed arena (`Formulas`, `Node`), its own two-sided `Sequent`, its own `Derivation` of LK or LJ with its own checker, its own text parser and TPTP reader, and its own Rocq certificate; it reuses the linear engines by translating into them (`core-ordinary.md`). It reuses `Limits`, `Error`, `Settings` and the exports' options, not the arena, forest, proof term or derivation view. So the first non-linear logic was added beside the linear one, not on a shared kernel, and the decision to do that is recorded as a result of D17 and D19 (below).
- Plan decisions that bind any restructuring (`plan/README.md`, "Design decisions"): D17, "the propositional engines must not become meaningfully slower" for first-order support (target counters identical, pinned CPU time within a few percent, "by generics or by a duplicated fast path where that is what it takes"); D19, every sequent gets the engine fastest for it, a dispatch table kept as data, and D7, "one engine per algorithm, never two implementations of one". The calculus's enums are deliberately closed (`core.md`, "Decisions"), so that a new variant is a planned 0.y bump and a compile error at every `match`.
- The crate's performance-relevant shapes are tied to the linear calculus: a proof node is a 16-byte `Copy` enum asserted by a static assertion, a sequent inside a search is an `OccSet` of occurrence ids, the focused engine is generic over a `Zone` (`Linear`, `Unrestricted`, `Ground`) that carries first-order terms without charging the propositional case (`core-focus.md`, `zone.rs`).

## 2. The field

How comparable multi-logic or multi-crate projects are laid out. Each entry says what it shows (inference marked).

### 2.1 Proof assistants and frameworks

- **Isabelle** is one distribution, one repository (mirror: [mirror-isabelle's `src`](https://github.com/isabelle-prover/mirror-isabelle/tree/master/src), read 2026-11-08), with directories `Pure`, `HOL`, `FOL`, `FOLP`, `ZF`, `CTT`, `Cube`, `LCF`, `CCL`, `Sequents`, `Provers`, `Tools`, `Doc`, `Benchmarks`. The library index ([page](http://dists.isabelle.cit.tum.de/dist/library/?token=Isabelle)) describes Pure as "a logical framework, an intuitionistic higher-order logic for declaratively expressing natural deduction rules" and FOL as first-order logic "classical or intuitionistic", "domain-theory (LCF) or set-theory (ZF)". The overview page ([isabelle.in.tum.de/overview](https://isabelle.in.tum.de/overview.html)) says the framework is "suitable for a variety of formal calculi" and that large collections of applications are in the separate Archive of Formal Proofs. An object logic is a theory file over Pure: `Sequents/LK.thy` has `theory LK imports LK0` and states its rules as meta-level implications over a `\<turnstile>` judgement with `$H`, `$F` for lists of formulas ([file](https://github.com/isabelle-prover/mirror-isabelle/blob/master/src/Sequents/LK.thy)). Shows: one generic kernel (Pure) plus object logics as data (rule files) in one repository, with the heavy application libraries kept outside (AFP). The cost is a kernel of generic strength (higher-order unification) and slower, less specialised proof search than a native engine (inference).
- **Rocq (formerly Coq)** is one repository with a kernel and plugins and its `theories` ([repository](https://github.com/rocq-prover/rocq), read 2026-11-08), and libraries are distributed as opam packages through the Rocq opam repository ([rocq-prover.org/packages](https://rocq-prover.org/packages)). **MathComp** is one repository that ships several opam packages from top-level `.opam` files (`rocq-mathcomp-boot`, `-algebra`, `-field`, `-solvable`, ...) ([repository](https://github.com/math-comp/math-comp)). Shows: for the Rocq side, several libraries from one repository is a normal pattern, and the opam package names are independent of the repository's name. How the build maps directories to packages was not shown (unsourced).
- **Lean 4 and Mathlib**: Mathlib is one repository with a `Mathlib/` module tree, depends on separate repositories such as Batteries and Aesop through `lake-manifest.json`, uses a build cache (`lake exe cache get`) and publishes its API docs from a separate repository (`mathlib4_docs`) ([repository](https://github.com/leanprover-community/mathlib4)). Shows: one large repository with one module tree; the cache is what keeps the contributor's build time bearable (inference: Cargo's counterpart is a shared target directory and, for CI, the Cachix cache linlog already uses).

### 2.2 Teaching tools with many systems

- **Carnap** (GPL-3.0) is one Haskell repository with folders `Carnap`, `Carnap-Server`, `Carnap-Client`, `Carnap-GHCJS`, `Carnap-Book`; the application serves carnap.io for "online logic instruction and learning" ([repository](https://github.com/Carnap/Carnap)). Its library has a generic `Carnap.Core` (data types, unification, a SAT interface), `Carnap.Calculi` (natural deduction with a parser per presentation style: Fitch, Lemmon, Hardegree, Montague, Hilbert; tableaux) and `Carnap.Languages` with one folder per language: `PurePropositional`, `PureFirstOrder`, `PureSecondOrder`, `ModalPropositional`, `ModalFirstOrder`, `ClassicalSequent`, `SetTheory`, `Arithmetic`, ... ([cabal file](https://github.com/Carnap/Carnap/blob/master/Carnap/Carnap.cabal), [Languages](https://github.com/Carnap/Carnap/tree/master/Carnap/src/Carnap/Languages)). It is a single cabal library, not a package per logic. Shows: the closest precedent to the author's picture (a teaching site that offers many calculi and proof methods), organised as one package with modules per language and per proof method over a shared core. The repository page gave no last-commit date (unsourced).
- **Jape** (Bornat and Sufrin, GPL-2.0, [repository](https://github.com/RBornat/jape)): "a configurable proof calculator" uncommitted to a logic, "customized by a description of a collection of inference rules and the syntax of judgements" ([search result summarising the CADE paper](https://www.cs.ox.ac.uk/people/bernard.sufrin/jape.org.uk/DOCUMENTS/CURRENT/JapeForIFP.pdf); the paper itself was not read). The distribution carries example encodings of natural deduction, several sequent calculi, Hindley-Milner typing and syllogisms, and a manual "Roll your own Jape logic" ([README on GitHub](https://github.com/RBornat/jape)). Shows: the "one engine, logics as data" end of the spectrum, in one repository. Relevant to the custom-rules candidate; the format of its logic files I did not see (unsourced).
- **Logitext** (BSD-3-Clause, [repository](https://github.com/ezyang/logitext)): an interactive, browser-based proof tool that drives a `coqtop` process through a server; its files name classical and intuitionistic first-order logic and a linear module (`Linear.hs`). Shows: one thin web tool over a real prover, with per-logic modules; a server architecture that linlog's wasm bundle avoids (inference).
- **Incredible Proof Machine** (MIT, [repository](https://github.com/nomeata/incredible)): logic core in Haskell compiled to JavaScript, a `logics/` folder and a `logic/` folder, a `SPEC.md` and `CORE-API.md`. The page does not say how logics are defined or whether they are configurable beyond the folder names (unsourced).

### 2.3 Provers and standards

- **Vampire** (BSD-3-Clause, [repository](https://github.com/vprover/vampire)): one repository, one C++ code base with directories per concern (`Kernel`, `Inferences`, `Saturation`, `Indexing`, `Parse`, `SAT`, `FMB`, ...), taking problems in TPTP format. Shows: a monolithic prover with the problem format as the common interface (the page did not state whether it is a single binary; the layout suggests so (inference)).
- **TPTP World** has grown by languages that share a syntax family: the non-classical forms NXF (over the typed first-order TXF) and NHF (over the higher-order THF) add connectives and "a syntax for specifying the logic in which the problem is formulated", with "minimal changes to existing parsing and reasoning software", since TPTP v9.0.0 ([Steen and Sutcliffe, arXiv 2508.09318](https://arxiv.org/pdf/2508.09318); [arXiv 2202.09836](https://arxiv.org/pdf/2202.09836), both known only from the search listing; I did not open the papers). Shows: the standard way to carry many logics in one input language is to declare the logic in the problem. It is the interchange format any collection would want to read (inference).

### 2.4 Rust workspaces of comparable shape

All read from their root `Cargo.toml` on 2026-11-08:

- **rust-analyzer** ([Cargo.toml](https://github.com/rust-lang/rust-analyzer/blob/master/Cargo.toml)): `crates/*` flat, internal crates as path dependencies with `version = "0.0.0"` and the comment that these "aren't published to crates.io"; a few in-tree libraries (`line-index`, `la-arena`, `lsp-server`) are published with real semver, kept under `lib/`. matklad's advice for such workspaces ([Large Rust Workspaces](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)): a flat `crates/` layout, "tree structure tends to deteriorate over time, while flat structure doesn't need maintenance", `0.0.0` for unpublished crates, and "If you do want to publish a subset of crates with proper semver API, be very deliberate about them", in a separate `libs/` folder. Shows: many crates, few published, publishing only what has a promised API.
- **typst** ([Cargo.toml](https://github.com/typst/typst/blob/main/Cargo.toml)): about 17 crates under `crates/` (`typst-syntax`, `typst-library`, `typst-layout`, `typst-pdf`, `typst-svg`, `typst-cli`, ...) all at one version (0.15.1), the version repeated in each internal dependency; `default-members` is the CLI only. Shows: splitting by pipeline stage, lockstep versions, every crate published.
- **wasmtime** ([Cargo.toml](https://github.com/bytecodealliance/wasmtime/blob/main/Cargo.toml)): lockstep version for most crates, Cranelift versioned separately, internal crates renamed `wasmtime-internal-*` and stated to be unsupported for external use, and many cargo features on the CLI to trim a build. Shows: a published-but-unsupported tier of crates, a way to publish internals without promising their API.
- **polars** ([Cargo.toml](https://github.com/pola-rs/polars/blob/main/Cargo.toml)): `crates/polars-*` about 37 default members at one shared version (0.55.1), a facade crate `polars` with `default-features = false` for its dependency entry, a few crates versioned independently. Shows: facade crate over many internals, lockstep versions.
- **tree-sitter** (MIT, [repository](https://github.com/tree-sitter/tree-sitter)): a core library plus CLI and bindings (including `lib/binding_web` for Wasm). The page did not say that grammars live in separate repositories (I believe they do, but this is unsourced).

### 2.5 The mechanics of crates, features, publishing and wasm

- Workspace members share one `Cargo.lock` and one `target` directory, and `workspace.package` keys are inherited with `key.workspace = true` ([Cargo book, Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)).
- Features "should be additive"; when a dependency is used by several packages "Cargo will use the union of all features enabled", and mutually exclusive features "should be avoided if at all possible" with the alternatives "split the functionality into separate packages" or select at runtime ([Cargo book, Features](https://doc.rust-lang.org/cargo/reference/features.html)). Inference: a feature per logic is fine because logics add up, but a feature that changes how an existing logic behaves is not.
- SemVer: removing a feature or an item is major; adding one is minor; adding a variant to a `#[non_exhaustive]` enum is "generally minor", to an exhaustive one major; for `0.y.z`, a change of `y` counts as major ([Cargo book, SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)). Linlog's closed calculus enums make every new connective a 0.y bump of the crate that owns the enum (inference from `core.md`).
- Publishing: a version "can never be overwritten, and the code cannot be deleted"; the `.crate` file is limited to 10 MB; crate names are "allocated on a first-come-first-serve basis" ([Cargo book, Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)). `cargo publish` has `--workspace`, `--exclude` and `-p` ([cargo publish](https://doc.rust-lang.org/cargo/commands/cargo-publish.html)); whether it orders members by dependency and since which Cargo version was not shown (unsourced). Linlog's own release job already publishes per package in order.
- Namespaces: RFC 3243, "Packages as (optional) namespaces" (`serde/derive` style), was merged 2024-03-11, with tracking issues still open and no statement of implementation ([rfcs#3243](https://github.com/rust-lang/rfcs/pull/3243)). Inference: do not plan on namespaced crate names; plan on a prefix (`linlog-…`, or a new family prefix) and claim names early, since names are first-come.
- docs.rs builds with the features you list in `[package.metadata.docs.rs]` (`features`, `all-features`, `targets`) ([docs.rs metadata](https://docs.rs/about/metadata)). Build-time and size limits were not on that page (unsourced). Linlog's rustdoc is already published from `main` on Pages with every feature, and docs.rs would show one page per crate (inference).
- Compile time and speed: `cargo build --timings` shows "if any large crates that serialize compilation should be broken up"; generic code is instantiated, and optimised, in the crate that uses it; "if the opt-level is 2 or 3, then a crate will not use monomorphized generics from other crates"; more codegen units "may produce slower code"; LTO ("thin" or "fat") works across crates at a cost in compile time ([perf book, compile times](https://nnethercote.github.io/perf-book/compile-times.html), [Cargo book, profiles](https://doc.rust-lang.org/cargo/reference/profiles.html), [perf book, build configuration](https://nnethercote.github.io/perf-book/build-configuration.html), [inlining](https://nnethercote.github.io/perf-book/inlining.html): inlining depends on "whether the function is generic, and if the inlining is across a crate boundary"). Inference: a split of the hot engines from the types they use across a crate boundary can cost speed unless the types stay generic-instantiated in the engine's crate, or `#[inline]` and LTO are used, and the ratchet and target sets (D17) are the measurement that decides it.
- Wasm size: LTO, `opt-level = "s"` or `"z"`, `wasm-opt` ("another 15-20%"), `twiggy` to see what takes space, and avoidance of formatting code ([Rust and WebAssembly book, code size](https://rustwasm.github.io/docs/book/reference/code-size.html)). The size of linlog's current `linlog_web_bg.wasm` is not recorded in the files I read (unsourced), so what one module for all logics would weigh is a number the supervisor must measure, not one this note can give.
- Trusted Publishing: crates.io's documentation page did not load through the fetch tool, so the binding of a publisher to a repository, workflow and environment is not sourced here. The repository's own `release.yml` comment says the trusted publisher names the environment `crates-io`.

## 3. Candidates

The candidates are structures. Costs are in sessions (a session is a working day of about 4 to 8 hours producing reviewed, tested code). All are inferences from sections 1 and 2 unless a source is given.

### structure-1: one repository, one workspace, a kernel crate plus a crate per logic family

**What it brings and to whom.** A single place for the library, one `Cargo.lock`, one flake, one CI, one set of conventions (the licence header, doc comments, the lints, the rules files), and refactoring across the kernel and the suites in one commit. For the author as sole maintainer this is the least overhead (inference; the matklad and typst, polars, rust-analyzer layouts all do this). For the AI models that use linlog through `skills/linlog/`: one guide and one command with subcommands per logic. For the web page: one `linlog-web` crate that depends on the suites it offers, behind features if a smaller bundle is wanted.

**What it offers in linlog's terms.** Layout, flat as matklad advises:

```
crates/
  kernel/     (name to be chosen)  formulas, judgements, rule schemas, derivation, checker, interactive goals, limits, errors, settings, wire
  linear/     today's `linlog` crate, its engines, nets, ordinary embeddings
  classical/  LK, LJ, minimal, ND, tableaux, resolution (native)
  modal/      ...
  lambda/     simply typed lambda calculus, System F
  export/     the trees (LaTeX, Typst, SVG, PNG, PDF) over the kernel's derivation
  cli/  web/  bench/
```

The kernel's derivation checker is "a checker against rule schemas": a calculus is a table of rules (premise and conclusion patterns over the judgement), the derivation is a tree, and `check` verifies each node against its schema (this is what Isabelle's Pure and Jape do as data: section 2.1 and 2.2). Search stays native per suite. Decide, check, prove step by step, export and certify are each offered per suite; the export and interactive proving come from the kernel.

**Cost.** Moving the repository's layout (`core/` to `crates/linear/`) is one session of mechanical work with the flake, the rules paths (`paths:` globs in `.claude/rules/*.md`) and the manual's examples; extracting a kernel is 3 to 5 sessions (structure-5 below says what to extract and what not to); every further logic is its own steps. Trusted base: the kernel's checker becomes the base of every suite that uses it, so it must be small, and the Rocq library gets a matching generic checker theorem (inference).

**Risk.** Build time rises with the number of suites in the default workspace (inference; `default-members` as in typst and wasmtime restrains it). A kernel written before a second logic needs it is a guess (the first lesson of `ordinary/`, section 1, is that the second logic was built beside, not on, the linear types). Cross-crate speed loss (section 2.5). Release coupling: lockstep versions as in typst and polars give every crate a bump when one changes; the closed enums make that frequent.

**Prerequisites.** The author's decision on names (note on name); a decision whether `linlog` keeps its crate name (structure-5); a measured wasm size and `cargo build --timings` of the present crate.

**Sources.** [Cargo book, Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html); [matklad](https://matklad.github.io/2021/08/22/large-rust-workspaces.html); typst, polars, wasmtime, rust-analyzer manifests (section 2.4); [Carnap's cabal](https://github.com/Carnap/Carnap/blob/master/Carnap/Carnap.cabal) for the kernel and calculi and languages split.

### structure-2: a repository per suite, each a workspace, sharing a kernel published on crates.io

**What it brings and to whom.** Independent release rhythms and independent contributors: a modal-logic suite could be written by someone else, with its own repository, issues and version. The author's words ("incorporate the new repos for the new features", plan/46) suggest this picture. Linlog already follows it for the Rocq library and the web client, with a flake input pinning each.

**What it offers.** The same per-logic functions as structure-1, but across repositories: the kernel must be a stable, published, semver crate before any suite can depend on it (a version "can never be overwritten", section 2.5), so every kernel change is a release and a lock bump in each suite. `linlog` today shows the cost: the Rocq library's update is "made, checked and pushed there first, then the lock moves here" (`CLAUDE.md`), a two-repository change per library change, plus Cachix, Zenodo and Pages settings per repository.

**Cost.** Kernel extraction as in structure-1 plus a release process for it (the release workflow, Zenodo entries, trusted publisher per crate and repository), then per suite a new repository with its flake, CI, rules, docs and Zenodo. I would count 2 to 3 sessions of infrastructure per new repository (inference, from step 45's evidence: the Rocq repository split took its own step, 45a, per `plan/README.md`, D20's note) before any logic is written. The trusted base: each suite's checker depends on a published kernel version.

**Risk.** The web page needs one wasm module that links several suites, so a web crate in some repository has to depend on all of them: either that repository becomes the integration point (it is then the monorepo in disguise) or the page loads one module per logic (below). Version skew between suites and kernel; the closed enums make every kernel bump a break for each suite (`core.md`). Contributors' experience: more repositories to clone and keep in sync (inference). The kernel API must be right early, because on crates.io it cannot be retracted, only yanked.

**Prerequisites.** structure-1's kernel, first, as a published crate; and a rename or new organisation decision (names of repositories under the new organisation).

**Sources.** [Cargo book, Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html); the Rocq example in `CLAUDE.md`; rust-analyzer's separately published `lib/` crates (section 2.4) as the precedent for publishing a small, semver-stable subset; tree-sitter and Isabelle/AFP (kernel in one place, applications elsewhere) as precedents, the last known from the overview page.

### structure-3: one crate, one feature per logic

**What it brings and to whom.** The lowest friction for a user: one `cargo add linlog --features modal`. One docs.rs page, one name on crates.io, one set of exports and one error family.

**What it offers.** Every logic as a module of the existing crate behind a feature, as `ordinary/` already is (it is not feature-gated today but its `parse` and `rocq` parts are). The bundle for the web is selected by the web crate's feature choice, so a page that wants only the tab's logic builds a module per feature set (the Cargo book says features unify to a union across a build, so one build cannot hold two feature sets; two wasm builds are needed (inference)).

**Cost.** The cheapest start (no restructuring), then a rising price: compile time of the one crate (110 000 lines today; `cargo hack --feature-powerset --depth 2`, which the gate already runs, grows combinatorially with the features: the project's own rule, from `CLAUDE.md`, is that "no item is dead in any feature combination", which gets hard as logics share items (inference)).

**Risk.** Features are additive, so a logic that needs a different `Term` type or a different arena cannot be a feature of a crate whose enums are closed: a new variant is a 0.y bump for all users (SemVer section above; `core.md` keeps the enums closed on purpose). Release coupling is total: a fix to the modal suite is a new `linlog` version, and every linlog user's lock moves. docs.rs shows everything on one page unless features are listed (section 2.5). A suite that someone else writes must be merged into this crate. It is the layout of Carnap's cabal library (section 2.2), which is a precedent for it working in a teaching tool with one maintainer group, not for a multi-author collection (inference).

**Prerequisites.** None. It is the default if nothing is decided.

**Sources.** [Cargo book, Features](https://doc.rust-lang.org/cargo/reference/features.html), [SemVer](https://doc.rust-lang.org/cargo/reference/semver.html); Carnap.

### structure-4: one engine, calculi as data (the Jape and Isabelle end), beside the native suites

**What it brings and to whom.** The author's last bullet, "a way to add custom derivation rules and a way to interactively build derivations with them", is a different structure from per-logic suites: a single generic kernel interpreting rule schemas. Teachers could define a calculus in a file and get checking, step-by-step building and export; Jape did it for teaching (section 2.2) and Isabelle's Pure does it for proof assistants (section 2.1).

**What it offers.** Only the rule-schema checker and interactive proving are generic; decision procedures are not (a schema interpreter has no focusing, no nets, no Horn engine). So: checkable and steppable under custom rules, searchable only by a generic and slow fallback such as bounded backward search (inference; for what is searchable under such rules see the note on custom rules, which owns that question).

**Cost.** This is the kernel of structure-1 with a data syntax for rules and a loader; the data format is 2 to 3 sessions on top of the kernel (inference). It also brings the largest addition to the trusted base: the schema matcher and the loaded rule files are trusted for every user-defined calculus, and the Rocq library would need a matching generic theorem.

**Risk.** Rule schemas for substructural logics need explicit context handling (Isabelle's `Sequents` and `LK` use list variables `$H`, `$F` in the quoted file above, which is what linlog's multiset and ordered contexts would also need). Modal logics with side conditions on contexts, binders and eigenvariable conditions make the schema language larger than it first looks (inference, not sourced beyond Isabelle's `LK0` using a framework with variables for lists).

**Prerequisites.** structure-1's kernel (or structure-2's), with a derivation type whose rules are data (tags that are rule names plus schema references rather than the closed `Rule` enum). This is where linlog's closed enums (`core.md`) are in tension, since a data-defined rule cannot be a variant of a compile-time enum (inference).

**Sources.** Jape (section 2.2), Isabelle (section 2.1), and the note on custom rules.

### structure-5: the kernel's boundary and the relation to `linlog` (the proposal the recommendation uses)

This is a design, not a competing option, so it is separate.

**GENERIC in the core** (judged from the rules files; each marked as inference, because the code was not read beyond the rules):

- The error family, `Limits`, `Progress`, `Refusal`, the "every long call takes `&Limits` and `stop`" rule, `Settings` and `Clock` (`core.md`, `limits.rs`, `settings.rs`): nothing in them mentions a connective.
- The wire policy (`wire.rs`: `upgrade`, `Within`, the written error form) and the "nothing recurses over a formula" discipline with `sequents::Walk` (`Visit::{Enter, Between, Exit}` describes "a walk over any tree" per `core.md`).
- The checker's shape: one pass over a proof term of 16-byte nodes against a forest, with a memory bound and a typed fault (`core-proofs.md`), and the pass's observers: the derivation view, the size estimate (`size.rs`), the text tree, and interactive proving's goal, rule and undo (`core-derivations.md`). The shape is generic; the rules inside are the dyadic linear calculus's. Reusing it for another calculus is a new rule table, not new machinery, but the proof node's rule tag is the closed linear `Rule` enum, which is the tension noted under structure-4.
- The exports' layouts: the tree by subtree widths for LaTeX, Typst and SVG, the options values, the one printer pattern (`export::notation`, `Notation::term`), the PNG and PDF renderers over SVG, fonts (`core-export.md`). They walk a `Derivation` whose `Inference` is a one-sided sequent of `Member`s, so the layout code is generic and its input type is not (inference).
- The batch (`search/batch.rs`: ordered results, workers, a stream), the front door's `Outcome`/`Verdict`/`Reason` and the "check every proof" discipline (`core-search.md`), the parallel runtime (`core-parallel.md`: one pool per search, no global), the benchmark harness (`bench.md`) and the web bindings' JSON-in, JSON-out shape (`web.md`).
- The hash-consed arena idea and the precedence-parser pattern: `ordinary/` has already re-implemented both for its own formulas, which says they are reusable as patterns but not yet as code (inference from section 1).

**LINEAR-SPECIFIC** (mark: inference from the rules files):

- Negation normal form and the one-sided dual; the fragments MLL to LL and the modes (`core-sequents.md`); the occurrence forest as the numbering of a one-sided sequent's subformulas, with `OccSet` (`core-forest.md`) and the intuitionistic reading as a lowering to it (`core-forest.md`).
- The focused engine's dyadic zones, copy bound and memo keys; the Horn engine's reading as a Petri net and its state equation; the inverse method's derived rules; proof nets, criteria and MELL boxes; the additive recursion (`core-focus.md`, `core-horn.md`, `core-inverse.md`, `core-nets.md`, `core-search.md`).
- Counts-based refutations (`Disproof`) and the Rocq invariants behind them (`CLAUDE.md`).
- Part of `export::rocq` (the NanoYalla kernel script); the LLTP reader, the `.spec` reader and the generated families (`core-inputs.md`).
- A boundary case: the first-order `Zone`/`Ground` machinery and `sequents/fo.rs` are for first-order linear logic; first-order classical logic would need its own terms, but `FoTerm` (closed, in `core.md`) is a candidate for the kernel (inference).

**Proposed boundary.** The kernel crate holds the generic list: errors, limits, settings, wire, the walk, the batch skeleton, the options values, the export layouts over a small trait for a derivation tree, and the interactive-proving goal/undo shape. It does not hold a formula arena, a forest or a proof term: those are designed once a second logic that is not a translation into linear logic (for example a modal sequent calculus or a typed lambda calculus) needs them (inference). Until then `ordinary/`'s pattern (an arena and a checker per logic, the engines reused by translation) stays the working method for any logic that has an embedding in linear logic, and `linlog` stays the linear suite.

**Relation to the existing `linlog` crate.** It stays the linear suite with its name and its API, so that the 0.2.0 users and the published DOIs (the library's `10.5281/zenodo.23899109`) keep meaning what they meant; it depends on the kernel when the kernel exists, as a re-export of the generic parts keeps `linlog::Error`, `linlog::Limits` and the rest in place (inference: re-exports preserve paths, and semver treats a moved item as unchanged if it is re-exported at the old path; I know this from the semver rules, which list removing or renaming a public item as major but do not discuss re-exports (unsourced)).

## 4. Considered and rejected

- **A feature per logic as the long-term structure** (structure-3) is not rejected for a start but is rejected as the end state, for the closed-enum and release-coupling reasons above.
- **A fully generic kernel first, extracted before a second logic needs it.** Rejected: `ordinary/` is the evidence that the second logic found its own types more useful than a shared one, and the decision D17 demands measurement for any change under the hot engines. Extract what two suites actually share (inference).
- **A server per logic like Logitext** ([repository](https://github.com/ezyang/logitext)): linlog's design goal is wasm in the browser, with no server. Rejected as a structure.
- **Making the kernel a copy of Isabelle's Pure** (higher-order unification and a meta-logic): a research system with a different cost profile; the Rocq library already provides the verified base (inference).
- **A wasm module per logic loaded on demand, as a first step.** Not rejected, deferred: it is an optimisation that needs the size of a module, which is not measured (section 2.5), and the web crate's feature flags allow it later (inference).
- **Cargo namespaces** (RFC 3243): merged as an RFC, with no statement of implementation found ([rfcs#3243](https://github.com/rust-lang/rfcs/pull/3243)); not usable to plan names.

## 5. What the structure must allow

For this area, from `.claude/rules/core.md` and the module rules: what is reusable for any sequent-style logic is the error, limits and settings machinery, the wire forms, the walk, the batch, the export layouts and options, interactive proving's shape and the harness; what is linear-specific is everything under the engines, the forest and the proof term's rule tags, plus counts-based refutation and the NanoYalla script. The structure must therefore:

1. Keep the propositional linear case at its measured speed: any move of a hot type across a crate boundary is a change under D17, to be checked by `bench/targets.sh` against `bench/targets/after-efficiency.csv` and `ratchet`, and the engines stay in the crate that owns the types they use (section 2.5, inference).
2. Let a new logic add a crate without a version bump of the others (this favours structure-1 or structure-2 over structure-3, inference), and without a change in the closed `Rule` enum of the linear calculus.
3. Let the web bindings link the suites a tab offers and keep a way to build fewer (a feature or a second wasm package), since the client loads one wasm module in one worker today (`src/worker/linlog.worker.ts`).
4. Let the Rocq side add a package per logic from one repository when the logics share a base, as MathComp does, with `rocq-linlog` kept as the linear package (inference from section 2.1); a repository per logic only if its author is another person.
5. Keep the conventions that make the repository checkable (licence headers, doc comments, no wildcard arms over the calculus's enums, the README and manual examples run as tests): they are per-repository today (`cli/tests/readme.rs`, the `conventions` flake check), so a second workspace repeats them, which is a cost of structure-2 (inference).

## Recommendation (inference)

**Choose structure-1 for the code, with structure-5's boundary, and keep structure-2 open for any suite a third party writes.**

- One repository and one workspace, converting the present four members to a flat `crates/` layout only when a second crate beyond `linlog` appears (the rename of `core/` is optional; rust-analyzer and typst show a flat layout works at 17 to 37 crates). The default members stay the CLI and the library, as typst's do, so a contributor's build does not grow with the number of suites.
- `linlog` stays the linear suite and the crate name. A kernel crate (name to be settled with the organisation's name) takes the generic list of structure-5, starts small and is published only when a second suite uses it. Until then it is a path crate at version `0.0.0`, unpublished, as rust-analyzer and matklad do, so that no API is promised that cannot be retracted.
- Names on crates.io: claim the kernel and per-logic crate names early (first-come, first-served), using a common prefix, since namespaces are not available. The wasm bindings stay one `linlog-web`-style crate behind per-suite features; a split of the module per logic waits for a measured size.
- Rocq: keep `rocq-linlog` as the linear library in its repository. A generic checker library for rule schemas, when structure-4 is chosen, is a new opam package; per-logic libraries of the same family can be several packages of one repository (MathComp precedent), or several repositories if the author wants the symmetry with the code.
- **First generalisation step**, in order: (1) measure: the wasm size, `cargo build --timings`, the ratchet and the target set on the current commit (a supervisor's decision, since I ran nothing; 1 session with the reports); (2) build the second non-linear logic that is not an embedding (the modal or lambda suite) the way `ordinary/` was built, in its own crate depending on `linlog` for the generic parts it can already reach (errors, limits, settings, exports' options), and write down in its report what it copied; (3) extract the kernel from what the second and `ordinary/` copied (3 to 5 sessions), with `linlog` re-exporting at the old paths and `breaks 0.2.0` as the check; (4) only then the rule-schema checker (structure-4), 2 to 3 sessions.
- **Cost** in sessions (inference): 1 for the measurements, 1 for the layout, 3 to 5 for the kernel, a suite for each logic after that. **Risk to the propositional performance**: low if the engines and their types stay in `linlog` and only the generic list moves (the engines do not call into the kernel in their inner loops: errors, limits and the stop poll are the only calls, and the stop poll is a closure already); high if the arena, forest or proof node move into a kernel, which is why they do not.

## 6. Sources

All read 2026-11-08 unless noted.

1. Cargo book, Workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
2. Cargo book, Features: https://doc.rust-lang.org/cargo/reference/features.html
3. Cargo book, SemVer compatibility: https://doc.rust-lang.org/cargo/reference/semver.html
4. Cargo book, Publishing: https://doc.rust-lang.org/cargo/reference/publishing.html
5. Cargo book, cargo publish: https://doc.rust-lang.org/cargo/commands/cargo-publish.html
6. Cargo book, Profiles: https://doc.rust-lang.org/cargo/reference/profiles.html
7. Rust RFC 3243, Packages as namespaces: https://github.com/rust-lang/rfcs/pull/3243
8. docs.rs metadata: https://docs.rs/about/metadata
9. The Rust Performance Book, compile times, build configuration, inlining: https://nnethercote.github.io/perf-book/compile-times.html , https://nnethercote.github.io/perf-book/build-configuration.html , https://nnethercote.github.io/perf-book/inlining.html
10. Rust and WebAssembly book, code size: https://rustwasm.github.io/docs/book/reference/code-size.html
11. matklad, Large Rust Workspaces: https://matklad.github.io/2021/08/22/large-rust-workspaces.html
12. rust-analyzer Cargo.toml: https://github.com/rust-lang/rust-analyzer/blob/master/Cargo.toml
13. typst Cargo.toml: https://github.com/typst/typst/blob/main/Cargo.toml
14. wasmtime Cargo.toml: https://github.com/bytecodealliance/wasmtime/blob/main/Cargo.toml
15. polars Cargo.toml: https://github.com/pola-rs/polars/blob/main/Cargo.toml
16. tree-sitter: https://github.com/tree-sitter/tree-sitter
17. Isabelle mirror, `src` and `Sequents/LK.thy`: https://github.com/isabelle-prover/mirror-isabelle/tree/master/src , https://github.com/isabelle-prover/mirror-isabelle/blob/master/src/Sequents/LK.thy
18. Isabelle library index: http://dists.isabelle.cit.tum.de/dist/library/?token=Isabelle ; overview: https://isabelle.in.tum.de/overview.html
19. Rocq repository: https://github.com/rocq-prover/rocq ; packages: https://rocq-prover.org/packages
20. MathComp: https://github.com/math-comp/math-comp
21. Mathlib4: https://github.com/leanprover-community/mathlib4
22. Carnap: https://github.com/Carnap/Carnap ; cabal file https://github.com/Carnap/Carnap/blob/master/Carnap/Carnap.cabal ; Languages https://github.com/Carnap/Carnap/tree/master/Carnap/src/Carnap/Languages ; Calculi https://github.com/Carnap/Carnap/tree/master/Carnap/src/Carnap/Calculi
23. Jape: https://github.com/RBornat/jape ; a web search listing of its CADE paper summary and Sufrin's document, https://www.cs.ox.ac.uk/people/bernard.sufrin/jape.org.uk/DOCUMENTS/CURRENT/JapeForIFP.pdf (listing only)
24. Logitext: https://github.com/ezyang/logitext
25. Incredible Proof Machine: https://github.com/nomeata/incredible
26. Vampire: https://github.com/vprover/vampire
27. TPTP non-classical logics: https://arxiv.org/pdf/2508.09318 and https://arxiv.org/pdf/2202.09836 (known from a web search summary, papers not opened)
28. In the workspace: `CLAUDE.md`, `Cargo.toml`, `docs/src/architecture.md`, `.claude/rules/core.md` and the `core-*.md` header paragraphs, `plan/README.md` D17, D19, D20, `.github/workflows/release.yml`, `plan/notes/research/scope/types-higher.md`, and the client's `src/worker/linlog.worker.ts`.

## 7. Not verified

- The size of `linlog_web_bg.wasm` today, and of a module per logic, and linlog's compile-time profile: not in the files I read; I ran nothing. Would run, under the supervisor's capped scope: `nix build .#linlog-web` then `ls -l` and `twiggy top` on the module, and `cargo build --timings -p linlog`.
- crates.io's own documentation on Trusted Publishing, crate-name policy and namespaces (the pages did not load through the fetch tool): how a publisher is bound to a repository, workflow and environment, and the policy on squatting.
- Whether `cargo publish --workspace` orders members by their dependencies, and since which Cargo version.
- docs.rs build limits.
- Jape's logic file format and its CADE paper (only a search summary was read); Incredible's definition of logics; Carnap's and Jape's last activity dates (pages showed none); how MathComp's build maps directories to packages; whether tree-sitter's grammars are separate repositories; Vampire's packaging as one binary.
- The two TPTP non-classical papers were known only from a search summary.
- Claims about generic and linear-specific code are judged from the rules files and the module lists, not from reading the Rust; the first session of any extraction must check them against the code (for example whether `export` reaches `Derivation` only through accessors).
- The re-export claim in structure-5 (that moving an item and re-exporting it at the old path is not a break) is the usual reading of the SemVer rules but the Cargo page I read does not state it; `breaks 0.2.0` (cargo-semver-checks) would show it.
- The cost estimates in sessions are my judgement, not sourced.
