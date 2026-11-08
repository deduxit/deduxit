# Structuring sequentium: crates, command and names

Research for the author, 2026-11-08. Read only: no file in any
repository was changed. Every claim about another project, a registry or
a guideline has its source linked; where no source was found, it says
so. Sources were read on 2026-11-08.

## 0. In one paragraph

Option (C) is recommended: rename the library and the command to
`sequentium` at 0.3.0 (step 53), the first release from the new
repository. Put the linear suite in a module `sequentium::linear` beside
`sequentium::ordinary`, keep the command's verbs, and let `--logic`
choose the suite. Retire `linlog` and `linlog-cli` with a final release
that points to the new crates; do not delete them. Rename the Rocq
library to `rocq-sequentium` before its opam pull request, which has not
been sent. Keep both Zenodo concept DOIs and change only the title from
0.3.0 on. The reasons: the roadmap already makes the `linlog` crate and
command the whole collection. Every comparable project gives its family
of crates one prefix and puts the brand on the main crate. A facade
needs several crates to gather, so one over a single crate (option B) is
not what the pattern is for. And the price will never be lower than now.
The cost is about three sessions plus the author's registry steps; (A)
costs nothing today and more with every release, and (B) costs about one
session plus double releases from then on.

## 1. Where the project stands (facts)

- The workspace (`Cargo.toml`) has
  members `core` (package `linlog`), `cli` (package `linlog-cli`,
  library `linlog_cli`, binary `linlog`, `doc = false` so that it does
  not overwrite the library's rustdoc), `web` (`linlog-web`,
  `publish = false`) and `bench` (`linlog-bench`, `publish = false`),
  all at version 0.2.0. The repository is
  `https://github.com/sequentium/sequentium`.
- crates.io today (the API, read now): `linlog` has 85 downloads (0.1.0:
  41, 0.2.0: 44) and `linlog-cli` 61 (22, 39); the author `flgrubm` is
  the only owner of both. `linlog`'s only reverse dependency is
  `linlog-cli`. These numbers are a little higher than the brief's 67
  and 43; they include mirrors and bots.
- The `linlog` crate is already more than the linear suite: it exports
  `ordinary` (LK, LJ, minimal) and `lambda` (step 50) next to the linear
  modules at its root (`core/src/lib.rs`). The command reads ordinary
  logic with `--logic classical|intuitionistic|minimal`; the verbs are
  `prove`, `check`, `eliminate`, `interact` and `seq`
  (`cli/src/argument_parsing.rs`).
- The roadmap keeps every new suite inside that crate until step 67.
  Step 60 says of the rule schemas: "a module of the `linlog` crate until
  step 67 decides the kernel". Step 65 gives the modal suite "a modal
  formula arena of its own, beside the ordinary one". Step 67 extracts a
  kernel "unpublished at 0.0.0, `linlog`'s paths kept" (D25).
- The Rocq library goes the same way. Step 55 adds a lemma for Kripke
  countermodels of LJ to "the Rocq library" (`rocq-linlog`). Step 66
  leaves the modal soundness proof to "a module of `rocq-linlog`, or a
  library of the collection of its own".
- The opam pull request for `rocq-linlog` has **not** been sent. GitHub's
  search finds no pull request naming `rocq-linlog` in `rocq-prover/opam`,
  `released/packages/rocq-linlog/…` answers 404, and so does
  `opam.ocaml.org/packages/rocq-linlog/`.
- The move to `sequentium` (step 48) is done: the organisation was
  created, the repositories were transferred and renamed, and
  `linlog-prover` is a placeholder whose site redirects. The manual is at
  `https://sequentium.github.io/sequentium/manual/`, the rustdoc at
  `…/sequentium/linlog/`, the guide at `…/sequentium/llms.txt`, and the
  page at `https://sequentium.github.io/`. The author has retired
  linlog's logo in favour of sequentium's mark (Status, 2026-11-08).
- Zenodo (the API, read now): version record 23899551 is titled "linlog"
  under concept 10.5281/zenodo.23448588, and record 23899110
  "rocq-linlog" under concept 10.5281/zenodo.23899109; their related
  identifiers already name `github.com/sequentium/…`.
- What the name `linlog` touches, counted with grep outside `plan/`:
  13 532 whole-word occurrences in 535 files, mostly prose. Among them:
  499 `linlog::` paths in Rust, 244 `$ linlog` console lines in README,
  the manual and the guide (`cli/tests/readme.rs` runs them), and
  strings such as `linlog-settings-*.json`, `--kernel linlog` and the
  Rocq preludes `LINLOG_*`. The web client has 527 occurrences in `src/`
  and 46 references to `linlog_web`/`linlog-web` in 18 files. The Rocq
  library has 55 occurrences of `Linlog` in 40 files.

## 2. What comparable projects do

### 2.1 Crates and names

| project | layout on crates.io | the main crate | the binary | source |
|---|---|---|---|---|
| Bevy | `bevy` plus about fifty `bevy_*` crates, one prefix | `bevy` is "a container crate that makes it easier to consume Bevy subcrates"; it re-exports `bevy_internal`, which has `pub use bevy_app as app;` and so on behind features; docs.rs shows each as a module of `bevy` (`bevy::app`, `bevy::asset`) | none (a library) | [Cargo.toml](https://github.com/bevyengine/bevy/blob/main/Cargo.toml), [src/lib.rs](https://github.com/bevyengine/bevy/blob/main/src/lib.rs), [bevy_internal/src/lib.rs](https://github.com/bevyengine/bevy/blob/main/crates/bevy_internal/src/lib.rs), [docs.rs/bevy](https://docs.rs/bevy/latest/bevy/) |
| Polars | `polars` facade over `polars-*`, all at one version | `polars`; `polars-core`'s README calls it "an internal sub-crate … not intended for external usage" | none in the Rust repository's facade | [crates/polars/Cargo.toml](https://github.com/pola-rs/polars/blob/main/crates/polars/Cargo.toml), [polars-core README](https://github.com/pola-rs/polars/blob/main/crates/polars-core/README.md) |
| gitoxide | `gix` (the library hub) over `gix-*` plumbing; the package `gitoxide` holds the command | `gix` | `gix` and `ein`, two `[[bin]]` in one package, `default-run = "gix"` | [Cargo.toml](https://github.com/GitoxideLabs/gitoxide/blob/main/Cargo.toml), [gix/Cargo.toml](https://github.com/GitoxideLabs/gitoxide/blob/main/gix/Cargo.toml) |
| Typst | `typst` (the compiler library) plus `typst-*`, one version, `default-members` the CLI | `typst` | `typst`, from the package `typst-cli`, the shape linlog has today | [Cargo.toml](https://github.com/typst/typst/blob/main/Cargo.toml), [typst-cli/Cargo.toml](https://github.com/typst/typst/blob/main/crates/typst-cli/Cargo.toml) |
| Nushell | `nu` plus `nu-*` (e.g. `nu-protocol`, "Nushell's internal protocols"), plugins as separate `nu_plugin_*` binaries | `nu` | `nu` | [Cargo.toml](https://github.com/nushell/nushell/blob/main/Cargo.toml) |
| Wasmtime | `wasmtime` (library), `wasmtime-cli`, and internal crates renamed `wasmtime-internal-*` "to signal that these are internal unsupported crates for external use" | `wasmtime` | `wasmtime`, from `wasmtime-cli` | [Cargo.toml](https://github.com/bytecodealliance/wasmtime/blob/main/Cargo.toml) |
| uv (Astral) | `uv` plus `uv-*` published at `0.0.x`; each README says "an internal component of uv. The Rust API … is unstable" | `uv`, versioned "by the binary command-line interface" | `uv`, plus `uvx`, "a convenient alias for `uv tool run`" | [Cargo.toml](https://github.com/astral-sh/uv/blob/main/Cargo.toml), [uv-cache README](https://github.com/astral-sh/uv/blob/main/crates/uv-cache/README.md), [versioning policy](https://docs.astral.sh/uv/reference/policies/versioning/), [tools guide](https://docs.astral.sh/uv/guides/tools/) |
| Ruff and ty (Astral) | two products with two names in **one** workspace: `ruff_*` and `ty_*` crates side by side; ty's own repository says its development "takes place in the Ruff repository" | `ruff`; `ty` is not on crates.io, and the name `ty` there belongs to an unrelated crate ("A tool to say thank you") | `ruff`, `ty` | [ruff Cargo.toml](https://github.com/astral-sh/ruff/blob/main/Cargo.toml), [ty README](https://github.com/astral-sh/ty/blob/main/README.md), crates.io API for `ty` |

So the brief's list has three shapes. Bevy, Polars and gitoxide are
facades: one brand crate gathers many internal crates under the same
prefix. Typst, Nushell, Wasmtime and uv put the brand on the main
library or command and publish their internal crates under the same
prefix, often marked internal. Astral, the organisation, has no crate
named `astral`. Its products keep their own names (ruff, ty, uv), and
two of them live in one workspace. In every one of these projects, the
crates of one product share one prefix. None mixes an old product name
with a new prefix in a single family.

### 2.2 The guidelines and the registry

- **Rust API Guidelines**
  ([naming](https://rust-lang.github.io/api-guidelines/naming.html)):
  - crate-name casing is listed as "unclear";
  - "Crate names should not use `-rs` or `-rust` as a suffix or prefix";
  - feature names carry no placeholder words (C-FEATURE).
  
  The guidelines say nothing on facades, families of crates or renames;
  I found no source for those there.
- **Cargo book, publishing**
  ([publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)):
  - names are "allocated on a first-come-first-serve basis";
  - "a publish is generally permanent. The version can never be
    overwritten, and the code cannot be deleted";
  - a yank means "no new dependencies can be created against that
    version, but all existing dependencies continue to work".
  
  The page describes no rename.
- **Cargo book, manifest**
  ([manifest](https://doc.rust-lang.org/cargo/reference/manifest.html)):
  - `[badges] maintenance = { status = "deprecated" }` exists, but the
    book says it "is currently not used by crates.io";
  - crates.io "previously displayed badges … that functionality has been
    removed".
  
  So the registry has no "deprecated" or "renamed" flag. The ways left
  are the description, the README and a final release.
- **Cargo book, features**
  ([features](https://doc.rust-lang.org/cargo/reference/features.html),
  read for `structure.md`): features "should be additive". A feature per
  suite is fine; a feature that changes what a suite does is not.
- **crates.io's policies**
  ([source of the policies page](https://github.com/rust-lang/crates.io/blob/main/svelte/src/routes/policies/%2Bpage.svelte)):
  - no crate that "exists only to reserve a name for a prolonged period
    of time (often called "name squatting")";
  - no transfer "without the explicit approval of the current owner";
  - owners may delete a crate if it is under 72 hours old, or if it has
    a single owner, fewer than 1000 downloads for each month it has been
    published, and no reverse dependencies on crates.io.
  
  The deletion code
  ([`src/controllers/krate/delete.rs`](https://github.com/rust-lang/crates.io/blob/main/src/controllers/krate/delete.rs))
  sets `AVAILABLE_AFTER: TimeDelta = TimeDelta::hours(24)`: a deleted
  name can be taken again a day later. Its RFC
  ([RFC 3660](https://rust-lang.github.io/rfcs/3660-crates-io-crate-deletions.html))
  had left that question open. `linlog` and `linlog-cli` meet the
  conditions, `linlog-cli` first, since it depends on `linlog`.
- **Trusted Publishing**
  ([source of crates.io's page](https://github.com/rust-lang/crates.io/blob/main/svelte/src/routes/docs/trusted-publishing/%2Bpage.svelte)).
  The prerequisites are "Your crate must already be published to
  crates.io (initial publish requires an API token)" and "You must be an
  owner". A new crate name therefore needs one publish with a token
  before the release workflow can publish it.
- **Yank messages.** crates.io's version update endpoint takes a
  `yank_message`
  ([`src/controllers/version/update.rs`](https://github.com/rust-lang/crates.io/blob/main/src/controllers/version/update.rs)).
  I did not check whether cargo shows it.
- **Namespaces.** RFC 3243, "Packages as (optional) namespaces", is
  still open: [tracking issue
  rust-lang/rust#122349](https://github.com/rust-lang/rust/issues/122349),
  last updated 2026-08-05. A `sequentium/linear` style of name cannot be
  planned on; a prefix (`sequentium-…`) is the way.

### 2.3 How renames have been done on crates.io

- **`git-repository` to `gix`** (gitoxide, 2023). The final
  `git-repository` 0.35.0 (2023-02-11) changed its description to "This
  crate is now named 'gix' and not available under this name anymore".
  Its changelog opens with "This is the last release under this name and
  merely a notice to inform that `git-repository` from now on is `gix`.
  Furthermore, all `git-*` crates … will be renamed to `gix-*`" (read
  from the published package,
  `static.crates.io/crates/git-repository/git-repository-0.35.0.crate`;
  crates.io API for
  [`git-repository`](https://crates.io/crates/git-repository)). It was
  not yanked and not deleted. A RustSec informational advisory followed:
  "All crates in the gitoxide project have been renamed from git-<crate>
  to gix-<crate>. The git- prefixed crates are no longer being updated"
  ([RUSTSEC-2023-0026](https://rustsec.org/advisories/RUSTSEC-2023-0026.html)).
- **`libcrux-aesgcm` to `libcrux-aes`** (2026). This rename too was
  announced by a RustSec advisory naming the final version under the old
  name ([RUSTSEC-2026-0210](https://rustsec.org/advisories/RUSTSEC-2026-0210.html)).
- **The semver trick**
  ([dtolnay/semver-trick](https://github.com/dtolnay/semver-trick)): "one
  version of your library declare[s] a dependency on a newer version of
  the same library" and re-exports it. My inference: the same move works
  across names. A final `linlog` can depend on `sequentium` and re-export
  it, so an old `Cargo.toml` that picks up the final version still
  compiles.

## 3. The command line: what comparable tools do

| tool | shape | what selects the logic or the part | source |
|---|---|---|---|
| cargo | one binary; `cargo foo` runs an external `cargo-foo` on `$PATH` | the subcommand (verbs) | [Cargo book, custom subcommands](https://doc.rust-lang.org/cargo/reference/external-tools.html) |
| rustup | one binary that is also `rustc`, `cargo`, … : "rustup itself and the rustup proxies are the same binary", chosen by the name it is called by (multicall) | the toolchain, by a file or `+toolchain` | [rustup-init.rs](https://github.com/rust-lang/rustup/blob/main/src/bin/rustup-init.rs) (header comment and the `match process.name()`), [proxies](https://rust-lang.github.io/rustup/concepts/proxies.html) |
| uv | one binary, nouns for areas (`uv pip`, `uv tool`), plus `uvx` as an alias | subcommands | [tools guide](https://docs.astral.sh/uv/guides/tools/) |
| git | one binary; "`git-<command>` that is runnable by the user in a directory on `$PATH` is invoked" for an unknown command | subcommands | [git(1), PATH](https://git-scm.com/docs/git) |
| opam | one binary; an unknown `opam foo` runs a plugin `opam-foo`, a package with the `plugin` flag | subcommands | [opam manual, plugins](https://opam.ocaml.org/doc/Manual.html) |
| Rocq 9.0 | **many binaries replaced by one**: "a new single binary `rocq` that dispatches commands" (`rocq compile`, `rocq repl`, `rocq makefile`), with "compatibility shims" so that `coqc`, `coqtop` and `coq_makefile` still work, and the `coq-*` opam packages kept as a compatibility layer while ported packages take `rocq-*` names | subcommands are verbs; the library comes from the load path | [Rocq 9.0 changes](https://rocq-prover.org/doc/V9.0.0/refman/changes.html), [release page](https://rocq-prover.org/releases/9.0.0) |
| Isabelle | one wrapper, `isabelle TOOL [ARGS ...]`; components add tools through `ISABELLE_TOOLS` | **the logic is an option**: `-l NAME  logic session name (default ISABELLE_LOGIC="HOL")` | [System Manual](https://isabelle.in.tum.de/doc/system.pdf), §1.1–1.2 |
| Lean | separate binaries by **role**: `lean`, `lake` (build), `elan` (version manager, which "places `lean` and `lake` binaries in your PATH", proxies as rustup's) | not applicable | [elan README](https://github.com/leanprover/elan/blob/master/README.md), [Lake README](https://github.com/leanprover/lean4/blob/master/src/lake/README.md) |
| Logics Workbench (LWB) | one system for classical and intuitionistic propositional logic and the modal logics K, KT, S4, S5, G; a session starts `load(k);` and then calls the same functions (`provable(…)`) in every logic | a module loaded by name | [Balsiger, Heuerding, Schwendimann 2000, §6](https://home.inf.unibe.ch/ltg/publications/2000/bhs00.pdf) |
| Tree Proof Generator | one web tool: "classical propositional logic and first-order predicate logic … a few normal modal logics"; "If you enter a modal formula, you will see a choice of how the accessibility relation should be constrained" | the formula, then options | [umsu.de/trees](https://www.umsu.de/trees/) |
| Carnap | one web application; an exercise names its calculus with `system="…"` (e.g. `LogicBookSD`, `montagueSC`) | an option | [Carnap manual, systems](https://carnap.github.io/Carnap-Manual/systems.html), [derivations](https://carnap.github.io/Carnap-Manual/derivations.html) |
| Vampire | one binary, modes as options (`--mode casc`) | input syntax and options | [README](https://github.com/vprover/vampire/blob/master/README.md) |
| E | `eprover`, and a **separately built** `eprover-ho` for higher-order logic ("Replace `eprover` by `eprover-ho` for the higher-order-enabled version", built with `--enable-ho`) | the binary, for a compile-time reason | [README](https://github.com/eprover/eprover/blob/master/README.md) |
| Z3 | one binary; the input format is a flag (`-smt2`, `-dimacs`, `-tptp`, …) | the input (SMT-LIB's `set-logic`) and flags | [src/shell/main.cpp](https://github.com/Z3Prover/z3/blob/master/src/shell/main.cpp) |
| TPTP | non-classical problems carry their logic in the file: "An annotated formula with the role logic is used to specify the semantics of formulae" | the input | [TPTP, non-classical logics](https://tptp.org/UserDocs/TPTPLanguage/NonClassicalLogics.html) |
| multicall | clap supports it: "`hostname` and `dnsdomainname` are provided by the same executable and which behaviour to use is based on the executable file name"; Busybox is the classic example | the name it is called by | [clap 4.6.7, `Command::multicall`](https://docs.rs/clap/4.6.7/clap/struct.Command.html#method.multicall) (the version in `Cargo.lock`) |

What follows for sequentium (inference from the table):

- **Several logics with the same verbs: the logic is a parameter.**
  Isabelle (`-l`), LWB (`load(k)`), Carnap (`system=`), Tree Proof
  Generator, Z3 and TPTP all work this way. So does linlog today
  (`--logic`), and so does the page's registry (D29: suite, method,
  parameters). The verbs (`prove`, `check`, `interact`, `seq`, exports,
  JSON) are what the suites share.
- **Noun-first subcommands** (`uv pip …`, `rustup toolchain …`) group
  different sets of verbs over different resources. `sequentium linear
  prove …` would repeat the shared verbs under every suite and change
  every invocation shape for no gain. A verb that only one suite has
  (`eliminate`, `seq fragment`, later a model check) fits as a verb that
  refuses suites without it, as `--logic` already refuses flags that do
  not apply.
- **A binary per logic** appears where there is a compile-time reason (E's
  `eprover-ho`) or a different role (Lean's `lean`/`lake`/`elan`,
  rustup's proxies). Neither applies here: the suites share one library,
  and the bindings link them into one wasm module.
- **External subcommands** (`sequentium foo` running `sequentium-foo`,
  as cargo, git and opam do) are the idiomatic door for a suite that a
  third party writes in another repository, which D25 keeps open. Add
  it when such a suite appears, not before.
- **Keeping an old command beside a new umbrella** has two precedents:
  Rocq's compatibility shims (`coqc` beside `rocq compile`) and uv's
  `uvx` alias, both cheap through multicall or a second `[[bin]]`. With
  `cargo install`, a second `[[bin]]` is a second binary of the full
  size. `cargo install` installs every binary of a package by default
  ("`--bins` … This is the default behavior"), and two packages
  installing the same binary name clash unless the user passes
  `--force`
  ([cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html)).
- **The binary's name.** `sequentium` is ten letters. A short alias
  must not be `seq`, which is GNU coreutils' number-sequence command
  ([coreutils manual](https://www.gnu.org/software/coreutils/manual/html_node/seq-invocation.html)).
  Shell completion, which clap generates, makes the length tolerable;
  `isabelle` and `wasmtime` are eight letters long.

## 4. The other names

- **The Rocq library.** Rocq's own convention is a project prefix for
  opam packages: "Ported packages should use `rocq-*` names", with
  `coq-*` kept for the compatibility layer
  ([Rocq 9.0 changes](https://rocq-prover.org/doc/V9.0.0/refman/changes.html)).
  One repository may publish several packages under one prefix
  (MathComp, in `structure.md`). Two idiomatic choices, then:
  - (i) **`rocq-sequentium`**, logical path `Sequentium`, with
    `Sequentium.Linear`, later `.Intuitionistic` and `.Modal`;
  - (ii) **`rocq-linlog`** kept as the linear package, with further
    `rocq-sequentium-*` packages from the same repository for the other
    logics.
  
  Step 55 already puts LJ's Kripke lemma in this library, so under (ii)
  the linear-named package would hold non-linear metatheory unless
  another package is made at step 55. The opam pull request is unsent,
  so renaming now costs no opam step. No source I read says how a
  package is renamed in the Rocq opam repository once it is merged;
  `name-and-organisation.md` says the same. A rename in GitHub
  (`sequentium/rocq-linlog` to `sequentium/rocq-sequentium`) redirects
  git and web links but not the Pages site (GitHub's "Renaming a
  repository", cited in `name-and-organisation.md`). So
  `sequentium.github.io/rocq-linlog/` needs a stub in the client's site,
  which serves the organisation's root (inference).
- **Zenodo.** A new version is "a completely new record with separate
  metadata" ([manage versions](https://help.zenodo.org/docs/deposit/manage-versions/)),
  and a concept DOI "resolves to the landing page of the latest version"
  ([versioning](https://zenodo.org/help/versioning)). So the concept
  DOIs 10.5281/zenodo.23448588 and 10.5281/zenodo.23899109 can stay, and
  the title can change from 0.3.0 on through `CITATION.cff`, while
  versions up to 0.2.0 keep "linlog" and "rocq-linlog". A new concept
  would cut the line of citations and has no advantage that I can see
  (inference).
- **The web bindings.** `linlog-web` is unpublished, so a rename to
  `sequentium-web` (JS `sequentium_web.js`, flake package
  `.#sequentium-web`) touches only this repository and the client (46
  references in 18 files there). On npm, `sequentium`, `linlog-web` and
  `@sequentium/web` were all free today (404 from the registry), if a
  package is ever published.
- **The manual and the rustdoc.** The manual's address already says
  `sequentium` and needs no change. The rustdoc's crate path would move
  from `…/sequentium/linlog/` to `…/sequentium/sequentium/`; a stub page
  at the old path, written by the Docs workflow, keeps links working
  (inference). The guide moves from `skills/linlog/` to
  `skills/sequentium/`; `llms.txt` stays where it is.
- **Cachix** keeps `linlog-prover` (D26); a cache's name is invisible to
  users.
- **PyPI**: `sequentium` is taken by an unrelated package for integer
  sequences
  ([pypi.org/project/sequentium](https://pypi.org/project/sequentium/),
  by the JSON API). This matters only if Python bindings are ever made.

## 5. The options compared

### (A) Keep `linlog` as is; an umbrella later or never

- **What users see.** `cargo add linlog` and the command `linlog`, with
  ordinary, lambda and later modal, natural deduction, resolution and
  STLC as `linlog::…` modules and `linlog --logic s4`. The page, the
  manual's address and the logo say sequentium while the crate and the
  command say linlog.
- **crates.io and docs.rs.** Nothing changes, and the existing pages and
  Trusted Publishing keep working.
- **Migration.** None; it costs 0 sessions.
- **The roadmap.** Steps 54 to 66 put every new suite into a crate named
  for linear logic. At step 67 the kernel and any split-off suites need
  names: `linlog-kernel` and `linlog-modal` would name a modal suite
  after linear logic, while `sequentium-kernel` under `linlog` would mix
  two prefixes, which none of the precedents in §2.1 does. Steps 55 and
  66 add non-linear metatheory to `rocq-linlog` or start a second Rocq
  name.
- **The work done today (47 to 50).** Untouched.
- **Risks.** The name says less about the product with every suite. A
  rename later costs more: more immutable releases, opam once submitted,
  more Zenodo versions and citations, users if any arrive.
  - A variant considered, **(A′)**: separate product names for later
    suites in the one workspace, like Ruff and ty. It contradicts the one
    command with `--logic`, the one page and the one guide (D29), so I
    do not pursue it.

### (B) A `sequentium` facade crate and command now; `linlog` beneath as the linear suite

- **What users see.** Two names for one library: `sequentium` would be
  `pub use linlog::*` plus module aliases, and `linlog` the same code.
  Two commands, or one command under two names.
- **crates.io and docs.rs.** Four published crates (`sequentium`,
  `sequentium-cli`, `linlog`, `linlog-cli`) released in lockstep, and two
  docs.rs pages with the same items. The two new crates need a first
  publish with a token, then Trusted Publishing.
- **Migration.** A facade crate, a CLI package or second binary, and the
  docs and guide explaining both names. About one session now, then a
  double release at every version.
- **The roadmap.** Today `linlog` holds everything, so the facade
  gathers nothing; a facade aggregates several crates (bevy, polars,
  gix). New suites either go into `linlog`, which keeps (A)'s misnomer
  behind a facade, or become `sequentium-modal` and so on depending on
  `linlog` for generic infrastructure. That reverses the dependency
  until step 67, and it gives the family two prefixes for good. Step 67
  would then face renaming `linlog` anyway, at a higher price.
- **The work done today.** Untouched, except for docs.
- **Risks.** Users and AI models (through the guide) asking which crate
  to add, permanent double maintenance, and an inconsistent family.

### (C) Rename to `sequentium` now; `linlog` retired with a final release pointing to it

- **What users see.**
  - `cargo add sequentium` gives `sequentium::linear`,
    `sequentium::ordinary`, `sequentium::lambda` and later
    `sequentium::modal` and the rest.
  - `cargo install sequentium-cli` gives the command `sequentium prove
    …`, which defaults to the linear suite, with `--logic` as today.
  - The page, the manual, the logo, the crate and the command all say
    one name.
- **crates.io and docs.rs.**
  - New crates `sequentium` and `sequentium-cli` (both free, §7): first
    publish with a token, then Trusted Publishing configured for each
    (the prerequisite quoted in §2.2).
  - Final `linlog` 0.3.0 and `linlog-cli` 0.3.0, published by the
    existing configuration, following the gitoxide precedent (§2.3): the
    description "renamed to `sequentium`", a README notice, and
    preferably a small shim (`linlog` re-exports `sequentium`;
    `linlog-cli`'s `main` calls `sequentium_cli`), so that an old
    `cargo install linlog-cli` still gets a working, current command.
  - No deletion: a deleted name is free to anyone after 24 hours (§2.2),
    and 0.2.0's README, the Zenodo records and docs.rs's frozen pages
    name `linlog`.
  - docs.rs builds the new crate's pages; the old pages stay as they
    are.
- **The migration steps.**
  1. In this repository (about one session). Rename the packages, the
     libraries `sequentium` and `sequentium_cli` and the binary
     `sequentium` (keeping the `doc = false` trick). Then:
     - replace `linlog::` with `sequentium::` (499 paths);
     - change the console examples (244) and the help texts, with the
       behaviour lock as a commit of its own (CLAUDE.md);
     - change the file names `linlog-settings-*`, `skills/linlog/`, the
       flake's package names, the release workflow's crate list,
       `CITATION.cff`'s title, `.claude/` and CLAUDE.md, and README;
     - add the final shim crates for `linlog` and `linlog-cli`.
     
     The directories (`core/`, `cli/`) can keep their names until step
     67 moves them to `crates/`, which spares the rules' `paths:` globs.
  2. The module layout (about one more session, with D17's check). Move
     today's linear modules (`sequents`, `proofs`, `nets`, `search`,
     `occurrences`, `fragment`, `lltp`, `mist`, `families`) under
     `sequentium::linear`. Keep at the root only what `ordinary` already
     uses from outside its own module (errors, `Limits`, `Settings`,
     `export`, `wire`). That is evidence of what two suites share, not a
     guessed kernel, and step 67 still decides the crate. Moving modules
     changes neither types nor crate boundaries, so the target sets'
     counters should come out identical and the ratchet should not move
     (inference, which `bench/targets.sh` and `ratchet` would confirm).
     This step can be left out, keeping the linear items at the root
     ("C1"). That makes 0.3.0 cheaper and moves the break to step 67.
  3. The client (about half a session): the bindings' import and name,
     prose, and the Pages paths of the rustdoc.
  4. The Rocq library, if (i) of §4 is chosen (about one session, in its
     own repository first): the package and repository name, the
     logical path `Linlog` to `Sequentium` (55 occurrences in 40
     files), the exporter's preludes and `--kernel linlog` (to `--kernel
     sequentium`), the flake input and `modules/rocq-linlog.nix`, and
     the opam file prepared again for its 0.3.0.
  5. The author's steps:
     - publish the two new crates once with a token, and configure their
       Trusted Publishing;
     - publish the final `linlog`/`linlog-cli` (through the existing
       workflow);
     - rename `rocq-linlog` on GitHub, if chosen;
     - check Zenodo's titles after the tag.
     
     A RustSec informational advisory is optional; with about 150
     downloads in all I would not send one.
  
  In all, about three sessions plus the author's steps, or about two
  without step 2.
- **The roadmap.**
  - Step 53 grows by the sessions above (as a session "53r" before 53b,
    after 51b and 52 have landed, and beside 53m's mark placement, which
    edits the same README and rustdoc logo).
  - Steps 54 to 73: names replaced in their prompts at their reviews,
    which are finished at the previous step's review anyway.
  - Step 67 becomes the facade split of §2.1: a `sequentium-kernel` (or
    `-core`) beneath `sequentium`, and later perhaps `sequentium-linear`
    and the rest. The user's paths stay the same if step 2's layout was
    taken, because `sequentium::linear` is then a re-exported crate, as
    `bevy::app` is.
  - Steps 55 and 66's open Rocq placement becomes "the collection's
    library".
  - D24 ("`linlog` stays the linear suite, its name, crates and command
    unchanged") would be superseded; D25 and D26 keep their substance
    with new names.
- **The work done today (47 to 50).** Renamed mechanically with
  everything else. The reports in `plan/` keep the old names as history.
  The ordinary bindings (`ordinary_*`) and the lambda read-back keep
  their shapes.
- **Risks.**
  - One-time churn: stale "linlog" mentions are likely. A check could
    refuse the old name outside `plan/` and the history files, as the
    `conventions` check refuses names of the plan.
  - The binary is longer.
  - Search by name on crates.io finds "sequentium" where "linlog"
    explained itself; keywords and the description carry
    `linear-logic`.
  - The trademark question. The USPTO application SEQUENT in classes 9
    and 42 (Status, 2026-11-08) comes closer to a crate and a command
    than to an organisation name. The author has said "keep the name";
    this report does not reopen it.
  - Sessions running in other workspaces (51b, 52, 53m) collide with a
    tree-wide rename, so it needs a quiet moment.

### Summary table

| | (A) keep `linlog` | (B) facade now | (C) rename now |
|---|---|---|---|
| names users see | linlog (crate, command), sequentium (site, logo) | both, for the same library | sequentium everywhere |
| crates.io | unchanged | 4 crates in lockstep | 2 new crates, 2 final notices |
| cost now | 0 | about 1 session | about 3 sessions (about 2 without the module move) plus the author's steps |
| cost later | grows with each release, citation and opam | a double release at every version; a rename at 67 likely | none from naming; step 67 becomes a standard facade split |
| fits the precedents (§2.1) | as an org-over-products model, but the product is the collection | no: a facade over one crate, two prefixes | yes: one prefix, the brand on the main crate |
| roadmap 54–73 | unchanged text; suites under a linear name | unchanged; the facade grows awkward at 67 | names in prompts; 67 simpler |

## 6. Considered and rejected

- **Noun-first subcommands** (`sequentium linear prove`): §3. They would
  break every invocation and repeat the shared verbs.
- **A binary per suite**: §3. There is no compile-time or role reason,
  and the page already has one module.
- **Deleting `linlog`/`linlog-cli`**: allowed by crates.io's conditions,
  but the name is free to anyone a day later, and published artefacts
  name it (§2.2).
- **Placeholder crates** for `sequentium-kernel`, `-modal` and so on:
  against crates.io's squatting rule and D26. Each name is claimed with
  its first real release; all were free today (§7).
- **Keeping `linlog` as a second binary of `sequentium-cli`**: it doubles
  the installed size, and the final `linlog-cli` shim gives old users
  the same thing (§3, `cargo install`). As a Nix-only symlink it is
  cheap, but the manual would have to document two names; I would not.

## 7. Name availability (crates.io API, 2026-11-08)

`https://crates.io/api/v1/crates/NAME` with the User-Agent
`linlog-structure-research`. 404 means free:

| name | answer |
|---|---|
| `sequentium` | 404 free |
| `sequentium-cli` | 404 free |
| `sequentium-web` | 404 free |
| `sequentium-bench` | 404 free |
| `sequentium-kernel` | 404 free |
| `sequentium-core` | 404 free |
| `sequentium-linear` | 404 free |
| `sequentium-ordinary` | 404 free |
| `sequentium-classical` | 404 free |
| `sequentium-intuitionistic` | 404 free |
| `sequentium-modal` | 404 free |
| `sequentium-lambda` | 404 free |
| `sequentium-stlc` | 404 free |
| `sequentium-resolution` | 404 free |
| `sequentium-tableaux` | 404 free |
| `sequentium-nd` | 404 free |
| `sequentium-rules` | 404 free |
| `sequentium-prop`, `sequentium-propositional`, `sequentium-macros` | 404 free |
| `linlog-kernel`, `linlog-core` | 404 free |

Elsewhere: opam `rocq-sequentium` was not found
(`opam.ocaml.org/packages/rocq-sequentium/`, 404), and npm `sequentium`
and `@sequentium/web` were free. PyPI `sequentium` is taken (§4).
crates.io treats `-` and `_` as the same name; I did not check the
underscore spellings separately.

## 8. Recommendation

**(C), at 0.3.0.** The parts:

- `sequentium` is the library (today's `core/`), with the linear suite
  in `sequentium::linear` and the generic parts at the root only where
  `ordinary` already shares them (step 2 of §5 (C)).
- `sequentium-cli` (library `sequentium_cli`) has the binary
  `sequentium`. The verbs stay, `--logic` chooses the suite and defaults
  to `linear`, and there are no suite subcommands and no extra binaries.
- `sequentium-web` replaces the bindings' name.
- `linlog` and `linlog-cli` get final shim releases with notices, and
  are not deleted.
- The Rocq library becomes `rocq-sequentium` (logical path
  `Sequentium`) before its opam pull request.
- Both Zenodo concepts are kept, with new titles from 0.3.0.
- Step 67 later turns `sequentium` into a facade over
  `sequentium-kernel` and, if useful, `sequentium-<suite>` crates.

The reasons:

1. **It names what the crate already is.** By the roadmap's own design
   (D25, D29, steps 60 and 65), one library and one command hold every
   suite until step 67, and `linlog` already holds `ordinary` and
   `lambda`.
2. **It follows every comparable project** (§2.1): one prefix per
   family, the brand on the main crate, internal crates under the same
   prefix, and a facade only once there are several crates to gather.
   A facade now (B) has nothing to gather. A family with two prefixes,
   where (A) and (B) lead, has no precedent among them.
3. **The cost is lowest now and only grows.** About 150 downloads in
   all, no reverse dependency outside the project, an unsent opam pull
   request, and an organisation, site, manual and logo already renamed.
   Step 53 is the first release from the new repository anyway: Trusted
   Publishing and Zenodo must be proven again then in any case.
4. **It is the documented way to rename on crates.io**: a final release
   under the old name that points to the new one (gitoxide, §2.3).
   crates.io offers no rename, and deletion frees the name to anyone
   (§2.2).
5. **The command keeps its shape.** Verbs first with a logic option is
   what the logic tools do (Isabelle, LWB, Carnap, Tree Proof Generator,
   TPTP, Z3, §3) and what linlog does now. Only the binary's name
   changes.

If the author wants the smallest step that still settles the name: C1
(the rename without the module move) at 0.3.0, the move to
`sequentium::linear` at step 67 together with the kernel, and
`rocq-linlog` kept as the linear Rocq package with a new
`rocq-sequentium-*` package at step 55 or 66. That is about two sessions
now and one planned break later.

If the author keeps the decision of D24 as it stands: (A) is coherent
only if each later suite is its own product (A′), which the one
command, one page and one guide rule out. So I would not choose (A) for
the roadmap as written.

## 9. Not verified

- Whether cargo shows crates.io's yank message to users; whether crates.io
  has any other way to flag a renamed crate (none was found in the
  policies, the manifest chapter or the code read).
- How the Rocq opam repository renames or deprecates a merged package
  (no source; it matters only if the rename waits until after the pull
  request).
- Whether the module move of §5 (C) step 2 leaves the target sets' counters and
  the ratchet exactly unchanged; expected from the absence of type or
  crate-boundary changes, to be shown by `bench/targets.sh` and `ratchet`.
- The session estimates are judgements from the counts in §1, not
  measurements.
- LWB's current distribution and whether its interface still uses
  `load(…)`: the source is the 2000 paper's description of LWB 1.0.
- The Tree Proof Generator's page does not say how propositional and
  first-order input are told apart; the summary above follows its text.
- The Rocq 9.0 changelog page was read only in part (its first 100 000
  characters); the quotes come from that part.
- docs.rs's behaviour for a facade was checked on Bevy's page only.
- Name checks are first-come and valid only for the moment they were
  made; no trademark search was made beyond what Status records.

## 10. Sources

Project files: `Cargo.toml`, `core/Cargo.toml`, `cli/Cargo.toml`,
`web/Cargo.toml`, `bench/Cargo.toml`, `core/src/lib.rs`,
`cli/src/argument_parsing.rs`, `CLAUDE.md`,
`plan/reports/46-research-next.md` (the structure section and the decisions),
`plan/notes/research/next/structure.md`,
`plan/notes/research/next/name-and-organisation.md`, `plan/README.md`
(steps 47–73, D21–D32, Status), `plan/reports/48-sequentium.md`,
`plan/53-release-0.3.0.md`, `plan/55-kripke-countermodels.md`,
`plan/60-rule-schemas.md`, `plan/65-modal-logics.md`,
`plan/66-labelled-modal.md`, `plan/67-kernel-crate.md`,
`plan/71-typed-lambda.md`, `../rocq-linlog`, `../linlog-prover.github.io`.

Outside, all read 2026-11-08:

1. Rust API Guidelines, naming: https://rust-lang.github.io/api-guidelines/naming.html
2. Cargo book, publishing: https://doc.rust-lang.org/cargo/reference/publishing.html
3. Cargo book, manifest (name, badges, default-run): https://doc.rust-lang.org/cargo/reference/manifest.html
4. Cargo book, features: https://doc.rust-lang.org/cargo/reference/features.html
5. Cargo book, custom subcommands: https://doc.rust-lang.org/cargo/reference/external-tools.html
6. Cargo book, cargo install: https://doc.rust-lang.org/cargo/commands/cargo-install.html
7. Cargo book, cargo publish: https://doc.rust-lang.org/cargo/commands/cargo-publish.html
8. crates.io policies (source): https://github.com/rust-lang/crates.io/blob/main/svelte/src/routes/policies/%2Bpage.svelte
9. crates.io crate deletion (code): https://github.com/rust-lang/crates.io/blob/main/src/controllers/krate/delete.rs
10. RFC 3660, crate deletions: https://rust-lang.github.io/rfcs/3660-crates-io-crate-deletions.html
11. crates.io Trusted Publishing docs (source): https://github.com/rust-lang/crates.io/blob/main/svelte/src/routes/docs/trusted-publishing/%2Bpage.svelte
12. crates.io version update (yank message): https://github.com/rust-lang/crates.io/blob/main/src/controllers/version/update.rs
13. Namespaces tracking issue: https://github.com/rust-lang/rust/issues/122349
14. The crates.io API, for `linlog`, `linlog-cli`, `git-repository`, `ty`, `ruff`, `uv`, `gix`, `gitoxide`, `typst`, `typst-cli`, `nu`, `polars`, `polars-core`, `nu-protocol`, `typst-library`, `bevy_internal`, `gix-hash`, `wasmtime-internal-core`, `bevy`, `wasmtime`, `wasmtime-cli`, and every name in §7: https://crates.io/api/v1/crates/NAME
15. `git-repository` 0.35.0 package: https://static.crates.io/crates/git-repository/git-repository-0.35.0.crate
16. RUSTSEC-2023-0026: https://rustsec.org/advisories/RUSTSEC-2023-0026.html
17. RUSTSEC-2026-0210: https://rustsec.org/advisories/RUSTSEC-2026-0210.html
18. The semver trick: https://github.com/dtolnay/semver-trick
19. Bevy: https://github.com/bevyengine/bevy/blob/main/Cargo.toml , https://github.com/bevyengine/bevy/blob/main/src/lib.rs , https://github.com/bevyengine/bevy/blob/main/crates/bevy_internal/src/lib.rs , https://docs.rs/bevy/latest/bevy/
20. Polars: https://github.com/pola-rs/polars/blob/main/Cargo.toml , https://github.com/pola-rs/polars/blob/main/crates/polars/Cargo.toml , https://github.com/pola-rs/polars/blob/main/crates/polars-core/README.md
21. gitoxide: https://github.com/GitoxideLabs/gitoxide/blob/main/Cargo.toml , https://github.com/GitoxideLabs/gitoxide/blob/main/gix/Cargo.toml
22. Typst: https://github.com/typst/typst/blob/main/Cargo.toml , https://github.com/typst/typst/blob/main/crates/typst-cli/Cargo.toml
23. Nushell: https://github.com/nushell/nushell/blob/main/Cargo.toml
24. Wasmtime: https://github.com/bytecodealliance/wasmtime/blob/main/Cargo.toml
25. uv: https://github.com/astral-sh/uv/blob/main/Cargo.toml , https://github.com/astral-sh/uv/blob/main/crates/uv/Cargo.toml , https://github.com/astral-sh/uv/blob/main/crates/uv-cache/README.md , https://docs.astral.sh/uv/reference/policies/versioning/ , https://docs.astral.sh/uv/guides/tools/
26. Ruff and ty: https://github.com/astral-sh/ruff/blob/main/Cargo.toml , https://github.com/astral-sh/ty/blob/main/README.md
27. rustup: https://github.com/rust-lang/rustup/blob/main/src/bin/rustup-init.rs , https://rust-lang.github.io/rustup/concepts/proxies.html
28. git(1): https://git-scm.com/docs/git ; git-help: https://git-scm.com/docs/git-help
29. opam manual: https://opam.ocaml.org/doc/Manual.html
30. Rocq 9.0: https://rocq-prover.org/releases/9.0.0 , https://rocq-prover.org/doc/V9.0.0/refman/changes.html
31. Rocq opam repository README: https://github.com/rocq-prover/opam
32. Isabelle System Manual (Isabelle2026): https://isabelle.in.tum.de/doc/system.pdf
33. elan: https://github.com/leanprover/elan/blob/master/README.md ; Lake: https://github.com/leanprover/lean4/blob/master/src/lake/README.md
34. LWB: Balsiger, Heuerding, Schwendimann, "A Benchmark Method for the Propositional Modal Logics K, KT, S4", J. Autom. Reasoning 24(3), 2000, §6: https://home.inf.unibe.ch/ltg/publications/2000/bhs00.pdf
35. Tree Proof Generator: https://www.umsu.de/trees/
36. Carnap manual: https://carnap.github.io/Carnap-Manual/systems.html , https://carnap.github.io/Carnap-Manual/derivations.html
37. Vampire README: https://github.com/vprover/vampire/blob/master/README.md
38. E README: https://github.com/eprover/eprover/blob/master/README.md
39. Z3 shell: https://github.com/Z3Prover/z3/blob/master/src/shell/main.cpp
40. TPTP non-classical logics: https://tptp.org/UserDocs/TPTPLanguage/NonClassicalLogics.html
41. clap 4.6.7, `Command::multicall` (read from the local registry copy of `clap_builder` 4.6.7): https://docs.rs/clap/4.6.7/clap/struct.Command.html#method.multicall
42. GNU coreutils, seq: https://www.gnu.org/software/coreutils/manual/html_node/seq-invocation.html
43. Zenodo: https://help.zenodo.org/docs/deposit/manage-versions/ , https://zenodo.org/help/versioning ; records via https://zenodo.org/api/records/23899551 and …/23899110
44. PyPI `sequentium`: https://pypi.org/pypi/sequentium/json ; npm registry: https://registry.npmjs.org/sequentium , https://registry.npmjs.org/@sequentium%2fweb , https://registry.npmjs.org/linlog-web ; opam: https://opam.ocaml.org/packages/rocq-sequentium/ , https://opam.ocaml.org/packages/rocq-linlog/
45. GitHub search for a `rocq-linlog` pull request in `rocq-prover/opam`: https://api.github.com/search/issues?q=rocq-linlog+repo:rocq-prover/opam+is:pr
