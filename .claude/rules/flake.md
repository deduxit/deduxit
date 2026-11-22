---
paths:
  - "flake.nix"
  - "flake.lock"
  - "modules/**"
---

# The flake's modules

Loaded when `flake.nix`, its lock or a file under `modules/` is read or changed.
CLAUDE.md says how the flake is put together (dendritic flake-parts,
import-tree, one aspect per file); this is what each module holds.

Modules share values through `_module.args`:
`rustToolchain` and `craneLib` (`toolchain.nix`), `workspace` (the crane
arguments, `workspace.nix`). `checks.nix`, `devshell.nix`, `treefmt.nix`
and `systems.nix` are what their names say.

- **`dist.nix`: what a release ships for Linux.** `deduxit-static` is
  the command linked statically against musl (crane's musl example: the
  toolchain of rust-toolchain.toml with the musl target added,
  `CARGO_BUILD_TARGET` and `+crt-static`), with `pkgs.pkgsStatic`'s C
  compiler as `CC_<target>` and linker, since mimalloc, the musl
  build's allocator, is C; `dist` packs it into the archive and, by
  nfpm from one JSON list of files (JSON is YAML), the `.deb`, `.rpm`
  and Arch Linux package, `arch` given in nfpm's names (`amd64`,
  `arm64`), which it maps to each format's; `notices` is cargo-about's
  `THIRD-PARTY-NOTICES.txt` for `cli/about.toml`'s targets, built once
  (crane's vendored sources, `--frozen`) and installed by both
  `deduxit-cli` and `deduxit-static`. The release workflow builds `dist`
  on an x86_64 and an aarch64 runner (`packages.yml`); nothing checks it
  in `nix flake check`, the install workflow installs what it makes.
- **Each check reads only its crate's parts.** `workspace.nix` cuts the
  workspace into parts (`core`, `core-tests`, `cli`, `cli-tests`,
  `bench`, `bench-tests`, `web`, `web-tests`: a crate's `src/` with its
  manifest (the bindings' `build.rs` and cargo-about files too; the core
  crate's rule files and schema, `core/src/rules/**/*.json`, which
  `include_str!` reads and crane's cargo sources leave out), and its
  `tests/` with what they read: the core crate's export tests the font
  in `cli/fonts`, the command's README's examples), and `argsOf PARTS`
  gives the crane arguments of a source of those parts. Every source
  keeps every member's manifest and the lock, since cargo loads every member,
  and `stubMembers` (the `postPatch`) gives a member left out an empty
  file for each target its manifest names (`cli/src/lib.rs`,
  `cli/src/main.rs`, `bench/src/main.rs`, `web/src/lib.rs`, `web-render/src/lib.rs`,
  `web-tab/src/lib.rs`): a new explicit target path in
  a manifest goes into that list. A file no part names is in no Rust
  check: `plan/`, the benchmark data, the scripts, `fuzz/`.
- `checks.nix`'s `clippy-CRATE`, `test-CRATE` and
  `test-debug-assertions-CRATE` (CRATE `core`, `cli`, `bench`, `web`,
  `compat`; the bindings' natively, from `core`, `web`, `web-tests`;
  the last releases of `linlog` and `linlog-cli` from `core`, `cli` and
  `compat`, whose part holds both crates' manifests and sources) are one
  check per crate from `perCrate`: the core crate's from `core` and
  `core-tests` with `--all-features` (the set the command enables, so
  that `parallel`, `png` and `pdf` are tested; `cargo test --workspace`
  unifies to the same set), the command's from `core`,
  `cli`, `cli-tests`, the harness's from `core`, `bench`,
  `bench-tests`. So a change to `cli/` reruns the command's checks only
  and one to `core/src/` all three. `test-CRATE` is crane's, in the
  release profile; `test-debug-assertions-CRATE` the same tests with
  debug assertions and overflow checks on, from their own dependencies'
  build (`cargoArtifactsDebugAssertions`), since the flags are in every
  artifact. Checks install no build tree (`doInstallCargoArtifacts =
  false`): the cache holds only that they passed. The packages
  (`deduxit-cli` from `core` and `cli`, `deduxit-bench` from `core` and
  `bench`) leave out the tests, so a test's edit reruns neither `export`,
  `rocq`, `bench` nor `ratchet`; `doc`, `doc-private` read `core` and
  `cli`, `wasm` and `features` `core` alone, `deny` and `shear` every
  part.
- `clippy-core-default` is Clippy on the core crate and its tests as
  `cargo test --package deduxit` builds them, with the default features
  (from `core` and `core-tests`): `clippy-core` takes every feature and
  `cargo test --workspace` unifies `parallel` in, so a test item that
  only the pool's tests use warned in the crate's own run and in no
  check but `wasm`, which builds for wasm32 with `png` and `pdf`. The
  gate runs the same line after the workspace's Clippy, so
  the full check still holds every part of the gate. It costs about
  15 s on the shared dependencies' build.
- The dependencies' build (`buildDepsOnly`) reads the manifests and the
  lock alone (crane's dummy source), so it is the same derivation for
  every change but a manifest's or the lock's.

`manual.nix` is the manual (`docs/`): the package `manual` runs
`docs/figures.sh`, which draws every figure with `bench/plots.py` from
the benchmarks' CSV rows (Python with matplotlib, the comparison's), and
`mdbook build` (nixpkgs' mdBook; `create-missing = false`, so a chapter
`SUMMARY.md` lists and the tree lacks fails the build); the check
`manual` is that package, and `manual-links` runs lychee offline over
the manual's pages and README with `--include-fragments`, remapping the
repository's GitHub URLs (`blob`, `tree`, `edit`, raw) to the tree and
`deduxit.github.io/deduxit/` to the `doc` package, so a link to a
file, a heading or a page of the site that does not exist fails; links
elsewhere (the web page among them) are not fetched. lychee needs
`cacert` even offline, to build its client. The figures are drawn at
every build and never committed: `svg.hashsalt` and no date make them a
function of the rows. `workspace.nix`'s `doc` puts the rustdoc (the
`rustdoc` binding) and the manual together after both are built, so a
change of the manual leaves the rustdoc's derivation as it was, and
the reverse. The `cli-tests` part keeps `docs/src` (README's test runs
the manual's examples) and `bench-tests` keeps `docs/src` and the
scripts `bench/*.sh` (the harness's test of the manual's names).

`doc` is the published site and `doc-private` the
docs of every item with `--document-private-items`, both with
rustdoc's warnings denied, so a link in a private item's doc resolves too
(a split of a module once left four such links pointing nowhere). `export.nix` is the `export`
check, which compiles the snapshots (the fragments inside a document of
its own; `core/tests/snapshots/deduxit/`, the Rocq library's
certificates, is left out of its fileset, since it copies the files at
the top and `rocq-deduxit` compiles those), the command's default fragments (LaTeX and Typst, wrapped the
same way) and a CLI proof with pdfLaTeX and with Typst and the curryst of
nixpkgs (the version `export::typst::CURRYST` names, read from the source
at evaluation, so the package and the import line it writes cannot
drift), with no font but
Typst's own, renders the SVG snapshots and CLI drawings with resvg with
only the Euler Math font of nixpkgs' TeX Live, and checks the CLI's PNG
(pngcheck) and PDF (poppler: the font embedded, the text extractable;
veraPDF: every profile conforms), offline, and then once per math font
of `--font` (the list `fonts`): three drawings rendered by resvg with that
font's TeX Live directory alone, a PNG, and a PDF with the font embedded
that conforms to PDF/A-4; `rocq.nix` is the `rocq`
check, which compiles against NanoYalla as `check-certificate.nix`
builds it from the non-flake input `nanoyalla`
(Click & coLLecT pinned to a commit; `export::rocq::NANOYALLA` names
the version, which an assertion compares with the kernel's README at
evaluation) with nixpkgs' Rocq and standard library and compiles the
`.v` snapshots and seven CLI certificates against it (five of them over
`Prop`, of ordinary logic, classical and intuitionistic, one from the
SAT solver's one-world model, two of `true`,
which bind nothing; the intuitionistic linear one names `--kernel
nanoyalla`, since the mode alone would choose deduxit's library),
requiring Rocq to print nothing; `rocq-deduxit.nix` is the check
`rocq-deduxit`, which compiles, in a directory of their own, the
exporter's snapshots for deduxit's own Rocq library
(`core/tests/snapshots/deduxit/`) and the certificates the CLI writes for
it (Mix, affine, intuitionistic, intuitionistic affine, a `⊤`/`0` goal,
classical with `--kernel deduxit`, four refutations, five first-order
ones, one of them from a session's open witness, four Kripke
countermodels: two by `prove`, one by `check` of an outcome, one of a
model given; modal ones: a valid sequent per frame rule, a labelled root
from a session's `proof --rocq`, one by `check` of an outcome, and a
countermodel in K, T, S4, KD45 and S5), against the package
`rocq-deduxit` of the flake input `rocq-deduxit`
(`github:deduxit/rocq-deduxit`, pinned by the lock; `ROCQPATH`
names its `user-contrib` and the standard library's), requires silence
from each and that a first-order certificate with a wrong witness, a weight, a
countermodel whose valuation shrinks, a modal `Trans` on the wrong edges
and a modal model of S4 without a loop, each planted by `sed`, be
refused, and compiles a file that requires an
intuitionistic certificate as a module and applies its lemma to the
two-sided goal (the Rocq chapter's example, `-R . Proofs`, kept equal to
it by hand); an assertion at evaluation compares
`export::rocq::DEDUXIT` with the pinned package's `version` (its opam
file's). The input follows this flake's `nixpkgs`, `flake-parts`,
`import-tree`, `treefmt-nix` and `nanoyalla`, so the library is built by
the Rocq the `rocq` check uses (the same derivation as when the library
was under `rocq/`); its own `yalla` stays in the lock for its bridges,
which this flake never evaluates. The check `rocq-deduxit-library` is
the input's own `checks.SYSTEM.rocq-deduxit`, re-exported, since the
certificates rest on the library's theorems: it compiles the library's
tests and requires `Print Assumptions` of every theorem to answer
`Closed under the global context` (counted per line), so every pin is
verified here as well; with the inputs followed it is the same
derivation as the library's repository builds, so a pin checked there
costs nothing here. The bridges are that repository's check alone
(they need its `yalla`, and no certificate rests on them); `bench.nix` is the `deduxit-bench` package, the `bench`
check (the harness on the smallest instance of every family, the
problem file, and an LLTP and a `.spec` file of the command's lock
inputs, failing through `summary --strict` on a verdict against a known
one, an `error` row, a crash, a kill or no rows)
and the `lltp` package, the LLTP library fetched at a pinned commit with
its Petri-net archives unpacked and its one malformed file repaired,
which no check uses, and the `iltp` package, the ILTP library's
propositional part (v1.1.2, 274 TPTP problems, the input of the
ordinary-logic run) as its site serves it, fetched because it states no
licence; `nix build .#iltp -o bench/iltp` puts it where `.gitignore`
expects it. `iltp-first-order` is the library's first-order part at the
same version (2 670 problems, `Problems/` and the `Axioms/` they
include), fetched for the same reason (`nix build .#iltp-first-order -o
bench/iltp-first-order`). The `qcover` package is `github:blondimi/qcover` at its last
commit (2021-03-03), 442 MB, whose 176 coverability problems under
`stable/benchmarks/` (Mist 27, Soter 50, Wahl–Kroening 46, medical 12,
bug tracking 41) the harness reads with `--spec`; the repository is
Apache-2.0, the problem files state nothing of their own, so they are
fetched, not committed (`nix build .#qcover -o bench/qcover`).
`bench.nix` also has the `ratchet` check: `deduxit-bench ratchet --check`
under nixpkgs' valgrind (`valgrind.out`, since the package's first output
is its manual) against `bench/ceilings.csv` (`bench.md`).
`conventions.nix` holds three checks over the tracked tree but `plan/`
and the benchmark data: `conventions` (the licence header on every
`.rs`, `.nix`, `.toml`, `.sh`, `.py`, `.yml`, `.pl` and `.ml` file,
within the first four lines so that a shebang may come first, the
EUPL's on every file but `modules/_provers/ileancop_main.pl`, which
follows leanCoP 2.1's GPL main and carries `SPDX-License-Identifier:
GPL-3.0-or-later`, the check naming that one file; no comment and no README
line naming a step, a decision number or a file of the plan; `.claude/`
is exempt), `typos` (configured by `typos.toml`, whose words of the
domain are TPTP's `fof`, Rocq's `falso` and the like) and `shear`
(cargo-shear in a crane derivation, which reads the vendored
dependencies offline). The spec is outside the plan grep because it
names Andreoli's rules D1 and D2.
`systems.nix` declares x86_64-linux, aarch64-linux and aarch64-darwin;
CI builds and checks x86_64-linux alone, the other two evaluate and are
built nowhere. valgrind is broken in nixpkgs on aarch64-darwin, so the
devshell takes it and `ratchet.nix` defines the `ratchet` check only
where it runs (`lib.meta.availableOn` and not `meta.broken`, written once
as the module argument `valgrindRuns`, which the devshell reads; the
devshell once failed to evaluate there). The crates inherit `version` and
`edition` from `[workspace.package]`, so crane's package versions come
from the root `Cargo.toml` (`crateNameFromCargoToml` on a member's falls
back to 0.0.1).
`checks.nix`'s `wasm` runs Clippy on the core crate and its tests for
`wasm32-unknown-unknown` (the target `rust-toolchain.toml` names) with
every feature but `parallel`, warnings denied, without the host's
artifacts: the rule that the library stays fit for a browser was held by
convention alone before it, and a lint can fire on one width alone (a
boxed `State` is small on 32 bits, a `u64` by reference is no longer
trivially copied there, a comparison with `u32::MAX as usize` is one
with `usize::MAX`). Such a site takes `#[cfg_attr(target_pointer_width
= "32"|"64", expect(…))]`, never an `allow`.
`checks.nix`'s `package` packages both crates as `cargo publish`
uploads them and builds each from its package: `cargo package
--registry crates-io --package deduxit` (which verifies the library's
package by building it), then the command's package without
verification and without a lock, both unpacked outside the source tree
(whose manifest would claim them as members) and the command built in
release from its package with the library's patched in (`[patch.crates-io]`)
and the workspace's `Cargo.lock`. Offline, the vendored dependencies
replace crates.io, so cargo can neither see the unpublished library as
the command's dependency (its local overlay fails) nor package without
`--registry crates-io`. A package that needs a file from outside its
directory, or a test that does not travel left out of `exclude`, fails
there; what only crates.io validates at the upload (keywords, at most
five of at most 20 ASCII characters; categories from its list, an
unknown one only warned about; the licence's SPDX expression) no local
command checks, so a change of those fields is checked by hand. Its source is the whole
workspace with the licence files (`LICENSE` and the links
`core/LICENSE`, `cli/LICENSE`). The `deduxit-cli` package installs
`LICENSE` and every font's licence (`cli/fonts/*.txt`) under
`share/doc/deduxit-cli/`, since a copy of the binary (the public cache
holds one) must carry the EUPL's text and the embedded fonts' licences;
for the same reason no check keeps its target directory
(`doInstallCargoArtifacts = false`, the `test-*` checks too: crane keeps
it by default, and CI pushes every output to the public cache), and any
store path that holds a font file holds its licence beside it (the
export check's directory for New Computer Modern); the `cli` part carries `LICENSE`
for that. The upload
needs the network and the author's token; `cargo publish --dry-run
--locked --workspace` in the shell packages and verifies both against
the registry, the command through cargo's local overlay of the
unpublished library, and must run on a clean working copy (`jj new` on
the release commit): cargo's dirty check reads git, to which a non-empty
`@` is uncommitted, and `--allow-dirty` would record that in the
package's `.cargo_vcs_info.json`.
`msrv.nix` is the `msrv` check: `cargo check --workspace --all-targets
--all-features` with the stable toolchain of `rust-version` in
`[workspace.package]` (read at evaluation, `1.92` written as rust-overlay's
`1.92.0`), its dependencies built with it too, nothing shared with the
other checks. A `cargo update` that takes a dependency past that
version, or code that needs a newer compiler, fails it; raising
`rust-version` is a new 0.y version and a line of the changelog. When it
rises and how far is the `update-deps` skill's "When" (never to a Rust
less than six months old; resolver 3 prefers dependency versions that
build on it, and takes a newer one only where none fits).
`checks.nix`'s `features` (cargo-hack, each feature and every pair),
`features-tests` (the core crate's tests in every pair of features, each
alone and none among them: a test or a test-only item names the
features it needs, an integration test by a `#![cfg(…)]` at its top and
a test that needs more by its own `cfg`; `Derive::Ljt` was
`#[cfg(test)]` where its tests need `parse`, which only such a build
shows) and
the gate's two `cargo hack` runs deny warnings (`RUSTFLAGS="-D
warnings"`; the gate's in `target/hack`, since the flags are part of
every artifact and would rebuild the tests'): an import that a move left
unused without `serialize` once warned in ten feature sets while both
passed.
`devshell.nix` carries cargo-mutants and cargo-nextest for
`mutants/run.sh` and valgrind for the ratchet, and its `ratchet` command; `fuzz.nix` is the `fuzz` devshell, a nightly compiler
pinned by date against the locked rust-overlay and cargo-fuzz, for the
workspace under `fuzz/` alone (`evidence.md`).
`comparison.nix` builds the other provers of the comparison from their
own sources at pinned revisions (never vendored: several state no licence,
or GPL), each a `prover-NAME` package whose derivation is in
`modules/_provers/` (a directory import-tree skips): the Maude prover on
nixpkgs' Maude 3 unchanged, Click & coLLecT's prover with a driver of our
own in place of its web server and its 3 s limit read from the
environment, llprover, linTAP and LinearOne on SWI-Prolog with drivers of
our own for the last two (linTAP's `prove/1` never ends on a formula it
cannot prove), Sympli on MLton with a patch of no-op functions for the
newer SML/NJ `PP_DEVICE`, lolli with its `cargoHash`, APLL native with
OCaml 4.14 (it was written for 4.06), intuit and intuitR with the GHC of
nixpkgs and its Haskell package `minisat` (`haskellPackages.mkDerivation`
written out, no import from a derivation; intuit with its pragma
`MonadFailDesugaring` removed, which GHC 9.10 refuses, intuitR with
`partial-order` built unbroken and without its tests), and Otten's
leanCoP 2.1, nanoCoP 2.0, nanoCoP-i 2.0 and iLeanCoP 1.2 from their
leancop.de archives on SWI-Prolog (`cop.nix`, the driver `cop.pl` of
ours, iLeanCoP's two ECLiPSe directives replaced by `substituteInPlace`
and its TPTP read by `ileancop_main.pl`, `bench/TOOLS.md`). Each carries
`passthru.source` (url, revision, date, licence), which `comparison`'s
`share/deduxit-comparison/tools.json` records per `--tools` name.
`comparison` joins deduxit's command and harness, BenchExec, the provers
and Python with matplotlib; no check builds it.
- **`check-certificate.nix`: one command that checks a certificate.**
  The app `check-certificate` (`nix run .#check-certificate -- FILE`,
  the program `check-certificate.sh` beside it, a `writeShellApplication`
  with `inheritPath = false`) puts nixpkgs' Rocq standard library,
  NanoYalla and the pinned rocq-deduxit on `ROCQPATH` at once and lets
  the file's own import lines pick the kernel; its first line names the
  kernel it read (`From NanoYalla`, `From Deduxit`, other imports the
  standard library, none Rocq alone). NanoYalla is built here as a Rocq
  library (`nanoll.v` and `macroll.v` under `user-contrib/NanoYalla`,
  with the LGPL licence beside them) and handed to `rocq.nix` as the
  module argument `nanoyalla` (`_module.args`, as `toolchain.nix` hands
  `craneLib`), whose check compiles every certificate against it: the
  kernel is built once, and the app and the check read the same
  compiled files. The app is also `packages.check-certificate`, for `nix
  shell` and `nix profile install`. The file is copied into a temporary directory as
  `certificate.v` (Rocq names a module after its file, and a file name
  may be no identifier or a keyword such as `if`; Rocq's messages get
  the user's name back), compiled with `-Q . Checked`, and then the
  `.glob` file Rocq wrote beside it says what it defines, one line
  `KIND START:END SECTION NAME` each, from Rocq's own parse: `prf`
  (every theorem keyword) and `def` at the top level are its lemmas, `binder` and the
  references are allowed, any other kind refuses the file (`sec` and
  `var`: a section's hypothesis becomes a premise of the lemma and
  vanishes from its assumptions; `mod`, `modtype`, `ax`). A grep of the
  text found lemmas by line and missed `Lemma a … Qed. Lemma b … Admitted.`
  on one line; a grep for `Section` would refuse an atom named
  `Section`, which is a valid identifier there. Each lemma is then given
  to `Print Assumptions` from a second file: `Admitted` and an `Axiom`
  compile in silence, and only `Closed under the global context`, or the
  standard library's `Classical_Prop.classic` alone in a file that
  imports `Classical_Prop` (the classical certificates over `Prop`),
  passes; an axiom of another module of that name prints longer. Exit 0
  holds, 1 refused, 2 unreadable, as the command's. What it cannot see:
  a file that gives the statement's own names another meaning (a
  `Definition ll`); the statement is read from the file. The check
  `check-certificate` runs it on a certificate of each kernel, one
  over `Prop` and a modal one (`From Deduxit.Modal`, read as
  rocq-deduxit), and expects status 1 for a changed statement, an
  `Admitted` proof, a modal term the library's checker refuses, a lemma
  proved in a section and a fragment without import lines.
  The manual's Rocq chapter leads with the app and shows its output in a
  `readme-check: skip` block (README's test runs no `nix`), kept equal to
  it by hand: a bump of nixpkgs' Rocq or of either kernel changes its
  first line there.
- `web.nix` is the bindings for a browser (`web.md` has the details):
  the package `deduxit-web` (the bindings' module and the renderers',
  each built for wasm32 by a cargo of its own and by the
  `wasm-bindgen` CLI that nixpkgs has for the crate's version in
  `Cargo.lock`, `wasm-opt`, and cargo-about's notices), with its own
  dependencies' build for wasm32 (`wasmArtifacts`); the checks
  `web-package`, `web-wasm-tests` (the bindings' tests under Node),
  `web-core-wasm32` (the library's unit tests on wasm32-wasip1 under
  wasmtime, from `core` and `core-tests` alone), `web-clippy-wasm32`,
  `web-features` and `web-bindgen` (the CLI's version against the
  lock's); and the devshell's CLI, `wasm-opt`, Node and wasmtime, added
  from this module. `web-core-wasm32` runs the whole unit suite but the
  ten tests `core32Skipped` names (each with why), preopening the
  source tree for the tests that read files by the path cargo compiled
  in; `web-clippy-wasm32` is Clippy on the bindings and their tests for
  the browser's target; `web-features` validates the package's module
  with nixpkgs' `wasm-tools` against the feature baseline (`web.md`).
- **`kani.nix`: Kani and the package `kani-harnesses`.** nixpkgs has no Kani, so the
  package `kani` builds it from the input `kani` (the repository at a
  release tag, `flake = false`) with the nightly of its own
  `rust-toolchain.toml` through rust-overlay (`rustc-dev` and
  `llvm-tools`: kani-compiler links rustc's driver, whose path its
  `build.rs` takes from rustup's directories, patched to the nightly's),
  Kani's and the standard library's lock vendored by crane
  (`vendorMultipleCargoDeps`: the library's `-Z build-std` sysroot
  rebuilds std), and nixpkgs' CBMC, asserted equal to the version
  `kani-dependencies` names. `build-dev --release` makes the binaries and
  the sysroot; the install keeps the release bundle's layout
  (`lib/kani/{bin,lib,playback,no_core,library,toolchain}`, where
  kani-driver finds its compiler, its libraries and the cargo it runs by
  its own path), and `bin/kani` and `bin/cargo-kani` wrap kani-driver
  with CBMC, Kissat and a C compiler on `PATH` (`goto-cc` calls it).
  The `charon` submodule, the optional Aeneas backend's, is missing from
  GitHub's tarball and dropped from kani-compiler's manifest. The
  standard library's lock is read from the toolchain at evaluation, so
  both packages exist on x86_64-linux alone. `kani-harnesses` runs
  `cargo kani --package deduxit --no-default-features --jobs 1
  --output-format terse` (terse prints one summary per harness; `--jobs
  1` only says what plain `cargo kani` does without `--jobs`) from
  `core` alone, with the flags of `[workspace.metadata.kani]`
  (`Cargo.toml`). It is a package, not a check: a harness takes CBMC up
  to ten minutes and 8 GB here (the union of two zones, 582 s on
  2026-11-15) and up to 25 minutes on a CI runner beside other builds, and the
  check `kani` that ran two at a time beside the other checks starved
  GitHub's runner (`ci.md`, Kani). `nix flake check` skips it;
  `.github/workflows/kani.yml` builds it, and locally `nix build
  .#kani-harnesses` (32 minutes for the eleven on 2026-11-15, under
  the machine lock). Kani is not in the devshell, which every session loads: `nix
  shell .#kani -c cargo kani -p deduxit --no-default-features` runs it
  there (`core-proofs.md` has the harnesses).
- **`schema.nix`: the check `wire-schema`.** `_schema/documents.sh`
  writes one JSON document of every form the command writes
  (`FORM-CASE.json`, small inputs, each case's exit status expected) with
  the `deduxit-cli` package, and `_schema/check.py` (Python's
  `jsonschema`, `Draft202012Validator`) validates each against
  `$defs/FORM` of `docs/src/wire.schema.json`, the schema the manual
  publishes, fails a document that still passes with a key added (so the
  schema says what each form holds), and wants a document of every form
  that requires `version` but those `UNWRITTEN` lists with why. Its source
  is those files alone, so editing them rebuilds no Rust. The protocol's
  own forms have no command that writes them; `web/tests/protocol.rs`
  holds every kind's form to the schema by name (the `web-tests` part
  keeps the schema for it).
- **`dataset.nix`: the check `dataset`.** The dataset under `dataset/`
  (`problems.jsonl`, made by `dataset/build.py build`, which the check
  never runs: the families come from the harness and the stores from the
  web client's repository) is checked against `SHA256SUMS`, and `build.py
  verify` decides every tenth problem again with the `deduxit-cli`
  package (`prove --deterministic --work-limit`, the bound the build
  used, so the verdict is the machine's no more than the file's) and
  grades its stored answer with `grade`, which must say `correct`. Its
  source is `dataset/` alone, so editing it rebuilds no Rust; a change of
  the command's verdicts or of `grade` fails it until the dataset is
  built again and its version raised.
- **`budget.nix`: the check `budget`.** Cargo.lock's packages and each web
  module's bytes at `gzip -9` (the bindings' and the renderers'), each
  under a ceiling written in the file
  with the figure it was set at; a change that passes one raises it
  there, in a commit that says what earned the room.
- **`ci.nix`: the checks as CI runs them.** The output
  `githubActions.matrix` is one entry per group of x86_64-linux checks,
  which `.github/workflows/checks.yml` runs as one job each (`ci.md`).
  A group holds the checks that build one package, so that two runners
  never build the same derivation (the command's `build`, `export`,
  `rocq`, `rocq-deduxit`, `wire-schema`, `check-certificate`; the
  harness's `bench` and `ratchet`; the web module's `web-package`,
  `web-features`, `budget`; the site's `doc` and `manual-links`), or one
  check that fills a runner alone (`features`, `features-tests`,
  `test-core`,
  `test-debug-assertions-core`). Every check no group names goes to the
  group `rest`, and an assertion fails the evaluation when a group names
  a check that does not exist or one twice, so a renamed check cannot
  drop out of CI. A check that takes minutes gets a group of its own or
  joins the group of the package it builds.
