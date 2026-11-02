# Step 45: the fourth baseline, and the second release, 0.2.0

The step's sessions each add a section here: 45b (the families that
grow), 45a (the Rocq library in its own repository), 45c (the fourth
baseline), 45d (the release prepared) and 45e (the package archives).

## Session 45d, part 1: the release prepared

### Outcome

The 0.2.0 release's mechanics are prepared in both repositories, all
but the fourth baseline's results, the final words of README and the
manual, and the date, which part 2 does. In linlog
(`../linlog-step45d1`, on `qknxxpoo`): what 0.2.0 breaks was found by
two tools and is listed in the changelog's `## [0.2.0] - unreleased`
with what to do instead; `core.md`'s lists of closed and marked types
were checked against the rustdoc JSON and corrected (15 closed, 119
marked); `Error::Quantifiers`, raised nowhere and never released, is
retired; the version is 0.2.0 in the manifests, both locks,
`CITATION.cff` and through them the web bindings' package; `cargo
publish --dry-run --locked --workspace` packages and verifies both
crates (exit 0). The command has prebuilt files for every system: the
flake builds Linux's (a static musl binary with mimalloc, in an archive,
a `.deb`, an `.rpm` and an Arch Linux package, `nix build .#dist`), the
release workflow builds macOS's and Windows' archives on their runners
and attaches everything with `SHA256SUMS` and a build provenance
attestation; every package carries the licences, the crates' notices
among them. The manual's Installation chapter covers Debian and Ubuntu,
Fedora, Arch Linux, any Linux, NixOS and Nix, macOS and Windows, with
what each needs, how to update and remove, and what an unsigned file
meets; every Linux instruction ran in that distribution's root
filesystem here, the Nix ones in a temporary profile, and a new workflow
(`install.yml`) runs them all on GitHub's runners and containers. The
Rocq library (`../rocq-linlog-45d1`) is ready for its own 0.2.0: checked
with Rocq 9.0 to 9.3 by its flake, installed from its opam file into a
fresh opam switch for each, `opam lint` passing, with a changelog,
`CITATION.cff`, README, documentation and a release workflow; the
supervisor pushed it (`main` = `2ad6487e8388`, after the review's README
fix on `52e23df76340`) and linlog's lock is on it, every one of the 49
certificates linlog's check compiles closed under the global context.

### The break, found by tools

- **Tools**: cargo-semver-checks 0.51.0 and cargo-public-api 0.52.0 from
  the pinned nixpkgs, adopted through `new-tool` as the devshell's
  `breaks VERSION` (`modules/devshell.nix`; `core.md`, "The break against
  the last release", says how to read them). cargo-semver-checks has no
  lint for a changed field or signature type, so the second tool diffs
  both public APIs, each built as rustdoc JSON with every feature (the
  published 0.1.0 from its crates.io package). cargo-semver-checks'
  own JSON documents private items and its baseline overwrites
  `target/doc/linlog.json`, which a first attempt at the diff ran into.
- **Its findings** (`enum_variant_added`, `enum_no_repr_variant_discriminant_changed`,
  `partial_ord_enum_variants_reordered`; the diff's "Changed"): the new
  variants `Term::{Forall, Exists}`, `Kind::{Forall, Exists}`,
  `Node::{Cut, Forall, Exists}`, `Rule::{Cut, Forall, Exists}`;
  `Rule::ALL` `[Rule; 16]` → `[Rule; 19]` and `Named::ALL` `[Named; 34]`
  → `[Named; 41]`; `Rule::Open` 15 → 18 and after the new rules in the
  derived order; `export::rocq::Unsupported::AffineWeakening` 2 → 3 and
  `Compact` 3 → 4. A test crate showed that a downstream `as` cast of
  the `#[non_exhaustive]` fieldless `Unsupported` compiles, so its moved
  numbers are a break, if a small one. The diff's other lines are none:
  `svg::Font`'s derived `Deserialize` became a hand-written one (removed
  and added again, the `where` bound alone differing), and
  `view::Inference`'s hand-written `PartialEq` loses
  `StructuralPartialEq`, which matters only to a constant pattern, which
  its private fields already forbid.
- **The plan's list corrected** (departs from the design's accounting):
  `Placed::parent` is no break, since `Placed` came with the MELL nets on
  2026-10-23, after 0.1.0; nor are the Rocq certificates with a record
  literal, since 0.1.0's exporter wrote NanoYalla's alone (its crate has
  no `Kernel`); `FoTerm`, `Fault`, `Refusal` and `Space` are new or
  marked. The changelog said otherwise for the first two and now says
  this.
- **The command**: 0.1.0's binary (built from crates.io) and this one's
  `--help` of every subcommand compared flag by flag and value by value:
  nothing removed, 29 flags and the subcommand `eliminate` added, the
  engines `essential` and `inverse` among `--engine`'s values. Three
  changes of behaviour a script sees are in the break list with what
  to do instead: `-i` reads ILL's syntax alone (`--ill-syntax
  one-sided`), exit 1 instead of 3 after the classical refutation
  (`--classical-work 0`), `--format rocq` without `--kernel` for linlog's
  library (`--kernel nanoyalla`).
- **`core.md`'s lists** were checked by a script over the head's JSON:
  0.1.0 had 13 closed and 81 marked types; now 15 closed (`Division`
  and `FoTerm` added; the list lacked `FoTerm`) and 119 marked (23 were
  missing: the net looks, `Order`, the MELL and net-cut types,
  `IllSyntax`, `InputOptions`, `TermNode`, `Witness`, the refutation
  types, `SymbolFault`). `Named::ALL` stays an array: `Named` is a
  struct over the closed `Rule`, `Side` and `Division`, so its length
  changes only where a closed enum's does.
- **`Error::Quantifiers`** is retired: not in 0.1.0, constructed by
  the tests alone, its code `quantifiers` named by no document, the
  guide or the web client. No item is `#[deprecated]`.

### The versions

0.2.0 in `[workspace.package]` and the workspace dependency, both
`Cargo.lock`s (`cargo update --workspace`, `-p linlog` in `fuzz/`),
`CITATION.cff` (its `date-released` removed until part 2 dates it),
README's dependency line; the web bindings take the workspace's version.
Every outcome writes the crate's version, so the behaviour lock
(`core/tests/lock/json.txt`) and `core/tests/serialize.rs` change by it
alone, in the version's commit (checked: masked, the files are equal).
docs.rs builds `linlog` with every feature on x86_64 Linux and
`linlog-cli`'s library, as the flake's `doc` check does.

### Installing on each system

- **Research** (three sub-agents on Sonnet 5.5 at `medium`, sources
  read that day): ripgrep, fd, bat, hyperfine, typst, just, starship,
  zoxide, Kani and Verus install pages; every one offers prebuilt
  archives and a source build, most lead with distribution channels and
  Homebrew, only typst says how to update and none how to remove.
  rustc per system (packages.debian.org, packages.ubuntu.com,
  packages.fedoraproject.org, archlinux.org, formulae.brew.sh): Debian
  13 1.85.1 (backports 1.96.1), Ubuntu 24.04 1.75 (1.91 versioned),
  Ubuntu 26.04 1.93.1, Fedora 43/44 1.99.1, Arch 1.99.1, Homebrew
  1.99.1; MSRV 1.92. rustup's commands and Windows' MSVC need
  (rust-lang.org/tools/install, the rustup book). `nix profile install`
  became `add` in Nix 2.30 (its release notes); flakes are off on the
  official installer (nix.conf reference). Release tooling: dist 0.34.0
  owns its release workflow and builds `.msi` with WiX v3 only
  (axodotdev's book); ripgrep, fd and typst build by hand-written
  matrices, ripgrep a static musl binary and `.deb`, with jemalloc on
  musl because musl's allocator is slow (its `main.rs`); Flathub takes
  no console software ("Console softwares will not be accepted",
  docs.flathub.org requirements); AppImage's documents say nothing for
  a single static binary to gain; `actions/attest` is the current
  provenance action. The Rocq practice is in its own bullet below.
- **Offered for 0.2.0** (needs no account): prebuilt archives for
  `x86_64`/`aarch64-unknown-linux-musl`, `aarch64`/`x86_64-apple-darwin`,
  `x86_64`/`aarch64-pc-windows-msvc`; `.deb`, `.rpm` and an Arch Linux
  package for both Linux architectures (one more than the plan named,
  free with nfpm); `cargo install`; the flake. Not offered: Flatpak and
  AppImage (above); an `.msi` (WiX v7 asks its user to accept a EULA
  with the Open Source Maintenance Fee, for organisations above $10 000
  of revenue, docs.firegiant.com/wix/osmf: the author's to accept;
  departs from the design); a macOS `.pkg` (no surveyed tool ships one,
  and an unsigned one meets Gatekeeper where a `curl`-fetched archive
  does not); `cargo binstall` (it would need `[package.metadata.binstall]`
  to find archives named `linlog-…`, untested, a follow-up).
- **How they are made**: `modules/dist.nix` (`linlog-static`: crane's
  musl example, `pkgsStatic`'s C compiler for mimalloc; `dist`: a
  deterministic `.tar.gz` and nfpm 2.47.0's `.deb`, `.rpm` and
  `.pkg.tar.zst` from one list of files; `notices`: cargo-about over
  `cli/about.toml`'s eight targets); nfpm instead of cargo-deb and
  cargo-generate-rpm (departs from the design: cargo-generate-rpm is not
  in nixpkgs, and nfpm, GoReleaser's packager, makes all three formats
  in the sandbox without running cargo). `.github/workflows/packages.yml`
  builds them on `ubuntu-24.04` and `ubuntu-24.04-arm` with Nix, and
  macOS's and Windows' archives with rustup at the flake's compiler
  version on `macos-26`, `macos-26-intel`, `windows-2025` and
  `windows-11-arm` (ci.md's exception, written down). `release.yml`
  calls it after `check`, writes `SHA256SUMS`, attests every file
  (`actions/attest` v4.2.2 by SHA) and attaches them with the crates.
  Every package holds the binary, README, `LICENSE`, the five fonts'
  licences and `THIRD-PARTY-NOTICES.txt` (143 crates' entries); the
  flake's default package now installs the notices too.
- **musl and speed** (the supervisor's request, two timed runs it
  cleared, cores 4–7, medians of three, the flake's glibc build against
  the static one of the same commit): with musl's allocator `chain` at
  512 took 3.63 s against 4.70 s on one thread and 3.73 against 5.46 on
  four; `witness-fo` #3 at 10 3.80 against 6.81 and 2.67 against 9.83.
  With mimalloc (`#[global_allocator]` under `cfg(target_env =
  "musl")`, MIT, `cargo deny check` ok): 3.62/4.67, 3.74/4.73,
  3.82/5.50, 2.65/3.84, so 1.26 to 1.45 times on one thread and four
  alike, the rest musl's libc. The supervisor decided: the static binary
  for 0.2.0, the chapter states the numbers and recommends `cargo
  install` or Nix for long searches, a glibc-linked prebuilt binary is a
  follow-up.
- **Unsigned, what a user meets** (in the chapter): apt and pacman say
  nothing for a file; dnf5 installs it with "Warning: skipped OpenPGP
  checks for 1 package from repository: @commandline" (seen); macOS
  runs a `curl`-fetched binary and refuses a browser-fetched one until
  `xattr -d com.apple.quarantine` or "Open Anyway" (Apple's support page
  102445; not seen here); Windows may show SmartScreen's "Windows
  protected your PC" (Microsoft's SmartScreen page; `Unblock-File`
  clears the mark; not seen here).

### Every instruction, and how it was tried

| instruction | tried how | result |
|---|---|---|
| Debian 13: `curl -LO …deb`, `apt install ./…deb`, `apt remove linlog` | LXC root filesystem under bubblewrap (a user namespace, root mapped, capabilities inside it; apt's sandbox user set to root for the trial), the file served at 127.0.0.1 for the URL | installed, ran, removed |
| Ubuntu 24.04: the same | the same | installed, ran, removed |
| Fedora 44: `dnf install URL`, `dnf remove linlog` | the same; also `./file` | installed (the OpenPGP warning), ran, removed |
| Arch: `curl -LO`, `pacman -U ./…`, `pacman -R linlog` | the same (pacman's `DownloadUser` off for the trial) | installed, ran, removed |
| any Linux: `curl -LO` the archive and `SHA256SUMS`, `sha256sum --check --ignore-missing`, `tar -xzf`, `install -D … ~/.local/bin/linlog`, `rm` | in all four | OK, ran, removed |
| `cargo install --locked linlog-cli`, `cargo uninstall` with Fedora's Rust (`dnf install cargo`) | the same (crates.io serves 0.1.0 until the author publishes) | built in 56 s, ran, removed |
| the same with Arch's Rust (`pacman -S rust`) | the same, after `pacman-key --init --populate`, which the image skips (the first run failed for that; the rerun's output is in the session and appended to the trial log) | built, ran, removed |
| rustup on Debian 13 (`curl … sh.rustup.rs \| sh`), `rustup self uninstall` | the same | built, ran, removed; rustup removed |
| Debian 13's and Ubuntu 24.04's own rustc too old | `apt install rustc` there | 1.85.1 and 1.75.0, as the chapter says |
| Nix: `nix run github:…`, `nix profile add`, `remove linlog` | a temporary profile, at origin's `main` (`ceed014d`) for the tag | ran; the entry is named `linlog`; added, removed. `nix profile upgrade linlog` on that locked entry only warned that it "can't be upgraded" (a first record here read it as done; the reviewer found it): the chapter now updates a tagged entry by adding the next tag, and `install.yml` tries `upgrade` on an untagged one |
| Ubuntu 26.04's `apt install cargo build-essential`, macOS's archive and `shasum`, Windows' PowerShell steps, the arm64 `.deb` and `.rpm`, `cargo install` on macOS and Windows, the C compilers the source routes now name | not here (no root filesystem fetched for 26.04; no macOS or Windows) | by `install.yml` on the supervisor's push |
| the Arch package on ARM | nowhere (Arch Linux ARM has no container image on GitHub) | not offered by the chapter: ARM takes the archive; the file is still built |
| the chapter's downloads from the release (`v0.2.0` URLs, `dnf install URL`) | after the tag only: `install.yml`'s `release` jobs, skipped while the latest release has no `SHA256SUMS` | after the tag |
| the Rocq library: `opam install` of the package, per Rocq version | `nix develop .#opam -c scripts/opam-install.sh 9.0.1 9.1.1 9.2.0 9.3.0` in a temporary opam root (OCaml 4.14 as `ocaml-system`) | lint Passed; installed, every module imported, removed: 9.0.1 with stdlib 9.1.0, 9.1.1/9.2.0, 9.2.0/9.2.0, 9.3.0/9.2.0 |
| the library by Nix (`nix build`, a shell with the package) | a shell of Rocq, its stdlib and the package | `From Linlog Require Import Soundness` compiled without `ROCQPATH` |

### The Rocq library, released

- **Complete for 0.2.0**: linlog's check `rocq-linlog` now prints
  `Print Assumptions` of every certificate's lemma: 49 certificates
  (the 34 exporter snapshots: one-sided classical, Mix, affine, cut,
  MALL, MELL, two-sided, first-order with cut, instances, witnesses and
  eigenvariables, every refutation kind; and 15 the command writes),
  each `Closed under the global context`. What it lacks, in its README:
  theorems for linlog's count of one sign under an exponential, a state
  equation, a saturated set and any first-order refutation, so linlog
  writes no certificate for them; the ordered calculus; cut elimination.
- **By the Rocq community's practice** (a sub-agent on Sonnet 5.5 at
  `medium`, read that day): rocq-prover/opam's README (the `released`
  repository, tags `logpath:`, `category:`, `keyword:`, `date:` "the date
  the software was last updated"), its CI (`opam lint --check-upstream`,
  one Rocq picked by the solver), rocq-partial, rocq-pi-leibniz,
  rocq-smt-lib, rocq-sulfur and rocq-laproof (tag archives with their
  checksums, open or bounded `rocq-core`); rocq-stdlib's versions do
  not follow Rocq's (9.3 takes stdlib 9.2.0), so the file bounds
  `rocq-stdlib` below only; `rocq doc` is the current name and `make
  html` its target; the rocq-community templates emit `doc:` for a
  Pages site.
- **What it has now** (commits `zllvwomt`, `pxzymvww`, `oquqzrlr`,
  `opkuqzzo`): the flake checks it with every admitted Rocq
  (`rocq-linlog-9_0`, `-9_2`, `-9_3` beside the default 9.1; 9.2 and 9.3
  print the deprecation of `Notation` for abbreviations, whose
  replacement 9.0 and 9.1 lack); `nix build .#doc`; the opam shell and
  `scripts/opam-install.sh`; the opam file at 0.2.0 with `rocq-core
  {>= "9.0" & < "9.4~"}`, `rocq-stdlib {>= "9.0"}` and `doc:`; a
  changelog section `## [0.2.0] - unreleased`; `CITATION.cff`; README's
  install by opam and Nix and what it covers; `.github/workflows/release.yml`
  (the tag against the opam file, changelog and citation file, `nix
  flake check`, a GitHub release with the changelog's section and the
  tag archive's SHA-512 for the opam file, the documentation on Pages);
  CLAUDE.md and `rocq.md`, with the line on linlog's
  `rocq-linlog-library` that 45a left. Its `nix flake check` exited 0 at
  `52e23df7`; "rocq ready" was sent and answered (pushed), and linlog's
  `nix flake update rocq-linlog` moved the lock there with `LINLOG =
  "0.2.0"`.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-review-45d1`),
read only, judged four areas: the break list against the tools'
outputs and `core.md`'s lists, every instruction against its trial and
`install.yml`, the release's packages, and the Rocq library's
readiness. The break list, the type lists, the packages' names,
artifacts, pins and permissions, and the library held; seven findings:

1. *A Nix entry added by the tag is never upgraded, and the trial
   record claimed it was; the same for an opam pin.* Holds: the
   chapter, `rocq.md`, the library's README (`2ad6487e`, pushed, the
   lock moved) and `install.yml` now say and try what updates each, and
   this report corrects the record.
2. *macOS's and Windows' archives lack `THIRD-PARTY-NOTICES.txt`.*
   Holds: `packages.yml` builds the notices once with Nix and puts them
   into every archive.
3. *Windows' binary needs the Visual C++ runtime, yet the table says it
   needs nothing.* Holds as a risk the runners cannot show (they have
   Visual Studio): the binaries link the C runtime in (`+crt-static`),
   as ripgrep's do.
4. *The rustup route omits the C compiler.* Holds: every source route
   names it (`build-essential`, `gcc`, `xcode-select --install`), and
   `install.yml` installs what the chapter says.
5. *Arch on ARM is untried and no job tries it.* Holds: the chapter
   offers the Arch package for x86_64 alone.
6. *The Arch `cargo install` trial is unlogged.* Partly: it ran in the
   session after the keyring setup (its output is in the session's
   record), and its log now holds it; `install.yml` tries it too.
7. *`core-sequents.md` still names the retired variants.* Holds: the
   sentence says what refuses a binder today.

Its side note, that `nix build .#dist` on the ARM runner may build
`pkgsStatic`'s compiler when no cache has it, is in "What is left
open".

### Departures and assumptions

- No `.msi` (departs from the design): WiX v7's EULA is the author's to
  accept; a WiX v5 build (MS-RL, no EULA) or v7 is the proposal.
- nfpm instead of cargo-deb and cargo-generate-rpm (departs from the
  design), and an Arch Linux package beside them.
- The plan's break list was corrected by the tools (departs from the
  design's accounting): `Placed::parent` and the record-literal
  certificates are no breaks against 0.1.0; the moved discriminants and
  the arrays' lengths are.
- `cargo binstall` is not offered, though the research recommended it.
- Linux trials ran under bubblewrap, not proot (both need no system
  change; bubblewrap was on the machine), with three trial-only
  settings a real system does not need: apt's sandbox user, pacman's
  download user, and the keyring an LXC Arch image ships empty.
- The README test's list of other programs (`TOOLS`) gained `sudo`,
  `pacman`, `gh` and `xattr`, the programs the chapter's prose shows
  with flags: the list is the test's place for them, and every flag of
  `linlog` is still checked.
- The library's `doc:` field names its Pages site before the author
  switches Pages on; its release workflow deploys the documentation, so
  the switch belongs before its tag.

### What the next sessions must know

- **Part 2 (tomorrow)**: the baseline's results and figures, README's
  and the manual's final words, and the dates. Dating means, in linlog,
  `CHANGELOG.md`'s `## [0.2.0] - unreleased` becoming `## [0.2.0] -
  YYYY-MM-DD` and `CITATION.cff`'s `date-released: "YYYY-MM-DD"` (both
  workflows refuse a tag without them); in rocq-linlog the same two, and
  the opam file's `"date:YYYY-MM-DD"` tag, which the Rocq opam archive
  asks for (it is also what 45e's file states). Both repositories' heads
  then need their checks again and a push.
- **The chapter's commands name 0.2.0's files**, so a later release
  changes the version there; `packages.yml` and `dist.nix` name the
  files by the workspace's version, and `install.yml` finds them by
  pattern or reads the latest tag, so neither needs a change.
- **`install.yml` is the instructions' test**: a change of a chapter's
  command changes it in the same commit (ci.md). Its first runs come
  with the supervisor's push of this session's commits; its `release`
  jobs and the release workflow's packaging run first at the tag.
- **Nothing of the release workflow's new jobs has run**: actionlint
  passes; the first real run is at the tag. If `packages` fails at the
  tag, the GitHub release waits (the `release` job needs it), while the
  crates are already published: fix and run the release job again (the
  publish job skips versions already on crates.io).
- **The prebuilt Linux binary is 1.26 to 1.45 times slower** than a
  glibc build (`cli.md`), stated in the chapter; a glibc-linked prebuilt
  binary built on an old-glibc runner is the follow-up.
- **`breaks VERSION`** in the devshell lists a release's breaks; every
  line is a changelog bullet or explained as none (core.md).
- **45e** takes the library's opam file from its repository at the tag,
  adds `url { src … checksum }` from the release notes' SHA-512 (the
  release workflow writes it), drops `version:` as recent submissions
  do, and runs `opam lint --check-upstream`; `nix develop .#opam` in the
  library's repository has opam and an OCaml all admitted Rocq versions
  accept.

### What is left open

- The runs of `install.yml` and `packages.yml` on GitHub after the
  supervisor's push (read below, when they have run); the release
  workflow's new jobs and `install.yml`'s `release` jobs first run at the
  tag.
- `nix build .#dist` on `ubuntu-24.04-arm` may build `pkgsStatic`'s
  aarch64 C compiler when no cache holds it, which costs time on the
  runner (the reviewer's note).
- Follow-ups: a glibc-linked prebuilt Linux binary (built on an
  old-glibc runner) for long searches; `[package.metadata.binstall]` for
  `cargo binstall`; an `.msi` once the author decides on WiX; signing.
- The channels that need an account, proposed to the author in the
  last message: a Homebrew tap (`linlog-prover/homebrew-tap`), the AUR
  (`linlog-bin`), winget (a manifest in `microsoft/winget-pkgs`, the
  `.zip` as a portable installer), a Scoop bucket, Fedora COPR; Homebrew
  core and Debian or Fedora proper later (Homebrew core asks 225 stars
  of a self-submission, Debian packages every crate).

### Checklist

- [x] Read the brief, the step's prompt, conduct.md, step 30's prompt,
  `distribution.md`, 45a's and 45b's reports, `ci.md`, `cli.md`,
  `flake.md`, `core.md`, the changelog, `CITATION.cff`, `release.yml`,
  the installation chapter, the library's CLAUDE.md and `rocq.md`, the
  `new-tool` skill; research by three sub-agents on Sonnet 5.5 at
  `medium` (install channels, release packaging, Rocq practice) and the
  author's reminders by a fourth.
- [x] The break by cargo-semver-checks and cargo-public-api
  (`breaks 0.1.0`), the command's flags by both binaries' help; the
  changelog's `## [0.2.0] - unreleased` with the break list;
  `core.md`'s lists checked (15 closed, 119 marked);
  `Error::Quantifiers` retired.
- [x] 0.2.0 in the manifests, both locks, `CITATION.cff`, the bindings
  by the workspace; the behaviour lock by the version alone; `cargo
  publish --dry-run --locked --workspace`: exit 0.
- [x] The Linux files from the flake (`linlog-static`, `dist`,
  `notices`), mimalloc on musl after two timed runs the supervisor
  cleared, `cargo deny check` ok; `packages.yml`, `release.yml` with
  `SHA256SUMS` and attestations; the chapter; `install.yml`.
- [x] Every Linux instruction in Debian 13, Ubuntu 24.04, Fedora 44 and
  Arch under bubblewrap; the cargo route with Fedora's and Arch's Rust
  and rustup on Debian; Nix in a temporary profile; the rest by
  `install.yml` (its runs below).
- [x] The Rocq library: four commits and the review's one (`2ad6487e`),
  checked with Rocq 9.0 to 9.3, the opam install per version, `nix flake
  check` exit 0 at `52e23df7` and at `2ad6487e`; "rocq ready" sent and
  answered twice (pushed); linlog's lock on `2ad6487e`; 49 certificates
  closed under the global context.
- [x] D17: `45d.csv` against `45b.csv`, 0 of 145 decided rows differ;
  `net-45d.csv` against `net-45b.csv`, 0 of 64; the ratchet within its
  ceilings in every gate, none raised.
- [x] The gate at every code commit: `tznykwpm`, `zrsxssrs`,
  `ytyxokvy`, `slwypkqv`, `knmskrro`, and `xxtlmlst` (the head, with
  `ktntzwks`'s test change), each `gate: passed`; linlog's `nix flake
  check` exit 0 at `mykmzxwl` and again at this report's commit (in
  the final message).
- [x] The fresh-context reviewer's round, each finding answered and
  fixed.
- [ ] `install.yml`'s and `packages.yml`'s runs after the supervisor's
  push, read through GitHub's API (below once read).

## Session 45b: families that grow

### Outcome

Twelve families with a growing set and a claim, five of them new
(`contraction`, `pigeonhole`, `euler`, `duplicate`, `reach-fo`) and seven
reused (`wide-m1`, `counter`, `mix`, `chain`, `chain-fo`, `witness-fo`,
`lambek-fo`). Each claim is written from measured rows, a fresh-context
reviewer checked every verdict by construction and every claim, and its
four findings that held are fixed. The library gained `Family::growth`
and `Family::claim` (public fields of a `#[non_exhaustive]` struct, an
addition; no enum variant). The harness gained `run --growing`, which
runs every growing set and skips a configuration's larger sizes after a
run refused or at its limit, written as `skipped` rows that no table
counts, and `run --memo-limit N`, a configuration axis (column
`memo_limit`), so that the memo's claim has its comparison line.
`bench/growing.sh` is the nights' run with each claim's comparison
passes, and `bench/comparison.sh --growing` runs the other provers on
the growing sets. `bench/plots.py growth` draws a line per engine,
configuration or prover and writes the fitted growth beside each figure;
the manual's chapter `results/growing-2026-10-31.md` shows both figures,
drawn at build time from the rows of this session's timed runs (by day,
cores 2 and 3, the runs that placed each family's sizes); the nights'
rows replace them. `--all-families` shows no mismatch; both target sets
equal 42k's counters; every code commit passed the gate, and `nix flake
check` passed at the session's last commit. The signing key lapsed
during the session and the last commits waited for the author (below,
"What is left open"); the session was paused on 2026-10-31 at 07:25 and
finished on 2026-11-02.

### The research, and what each source gave

Three sub-agents on Sonnet 5.5 at `medium` read the sources below (the
literature's hard families for linear and intuitionistic logic, ILTP's
and TPTP's parametrised classes, the other provers' own sets and
documented limits); what they could not open they marked, and nothing
below rests on what was only recalled.

- **Chaudhuri, *The Focused Inverse Method for Linear Logic*, PhD thesis,
  CMU-CS-06-162 (2006), chapter 7** (read in full): Euler tours on the
  complete graph `K_n` as `edge₁ ⊗ … ⊗ edgeₘ ⊸ a ⊸ a` with an edge `(a ⊸
  b) & (b ⊸ a)` (§7.2.2, pp. 195–196; the inverse method took 9 s, 152 s
  and 3 043 s on its three sets, the unfocused prover failing on all),
  Hamiltonian tours, blocks world, QBF in MALL, affine encodings and the
  SICS problems under two embeddings. It gave `euler`; its verdict (a
  circuit exists iff every degree is even) is this session's derivation
  from Euler's theorem, and the graph is the square of a cycle instead
  of `K_n` (below).
- **Raths, Otten and Kreitz, "The ILTP problem library for
  intuitionistic logic", JAR 38 (2007), §2.1**, and the ILTP v1.1.2
  files themselves (`nix build .#iltp`: the headers of SYJ201 to SYJ212,
  each citing Dyckhoff, "Some benchmark formulas for intuitionistic
  propositional logic", 1997, and for SYJ203 and SYJ209 Franzén's SICS
  reports R87010B and R-89/89006), and the library's comparison table
  `ILTP-v1.1.2-prop-comparison.txt` (600 s per problem, 2007: SYJ202
  fails every prover from size 7 to 10, SYJ209 every prover from 11).
  It gave `contraction` (SYJ203 and SYJ209: "proof in LJ needs n
  contractions") and `pigeonhole` (SYJ202, Cook's pigeonhole principle);
  their linear readings are this session's, argued at the generators.
- **Olarte, de Paiva, Pimentel and Reis, "The ILLTP library for
  intuitionistic linear logic", EPTCS 292 (2019), arXiv:1904.06850,
  §3–4**: the SYJ classes kept at all 20 sizes in three translations,
  none of the non-theorems SYJ207 to SYJ212 refuted by the Maude prover
  in 5 minutes; Petri-net reachability as `⊗ !(•t ⊸ t•) ⊢ M ⊸ M'`. It
  confirmed that the intuitionistic non-theorems are where refutation
  is least explored, which `contraction` #1 takes up.
- **Hodas and Miller, "Logic programming in a fragment of intuitionistic
  linear logic", I&C 110 (1994), introduction**: the exponentially many
  splits of a context under `⊗`, delayed as long as possible; the
  claim of `euler`.
- **Lincoln and Winkler's announcement of 3-Partition in constant-only
  MLL (Types list, 1992)** and the abstracts of Kanovich's Horn
  fragments: linlog's families already have both.
- **The other provers' own sources**, read at the revisions the flake
  pins (`modules/_provers/`), and runs of the comparison's binaries by
  the agent (cores 4–7, 10 to 60 s): llprover (`llprover.pl` lines
  335–347 and 421–432, and its guide: a per-path threshold of 5
  contractions and cuts, "initial value is 5"), the Maude ILL prover
  (`ILL-Prover-flat.maude` lines 142, 286–310, and the LLTP script's
  copy limit 5), APLL (`prover.ml:10`, a bound of 3; `foc_ill_bwd.ml`,
  copies refilled once per round, `2^|Γ|` splits), Click & coLLecT
  (`focused_proof.ml` lines 460–476), linTAP (`lintap.pl` lines 56–58),
  lolli (`lolli-prove/src/search.rs` at `cb09efc`, lines about 293–352:
  a clause of the unrestricted zone used at most twice on a branch),
  LinearOne (`mill1.pl`, MILL1 only) and Sympli (its README). It gave
  `duplicate`, `!(a ⊸ a ⊗ a), a ⊢ a ⊗ … ⊗ a`, which the agent ran: lolli
  fails from 4 factors, APLL from 5, llprover and the Maude prover from
  7, Click & coLLecT times out from 8, Sympli proves 6 and 12.
- Not used: Tarau's generators of implicational formulas
  (arXiv:2009.10241, 1910.01775), which enumerate rather than grow;
  TPTP's SYN007 (Pelletier 71), classically valid and intuitionistically
  not, which needs the additive reading of `⇔`; Heijltjes and Houston's
  MLL with units, a question of proof equivalence, not provability.

### The families and their claims

Each family's verdicts are argued at its generator; "comparison" is the
line its claim is set against in the figure. Sizes are the growing set
(`Family::growth`), the ceiling the size where every line that admits
the family has met the 60 s limit or a bound in the timed run, or a
stated ceiling.

| family | claim (the line) | comparison | verdicts by construction | growing set, ceiling |
|---|---|---|---|---|
| `wide-m1` (reused) | the net engine past the focused engine's memory bound, to which the default hands over: 16 384 pairs in 35 s, quadratic | the focused engine (memory bound at 4 096); the inverse method forced is faster still (32 768 in 47 s), and no row gives it a turn | provable: a proof's identification of atoms keeps it a proof | 256 to 32 768, doubling; every line at the limit or memory by 32 768 |
| `counter` (reused) | the Horn engine, linear (262 144 tokens in 0.09 s) | the focused engine forced, about cubic (256 in 0.8 s, past a minute at 1 024); the inverse method forced, 16 in 16 s | provable within log₂ n copies (doubling clauses) | 2 to 262 144; stated ceiling 262 144 tokens, where the Horn engine takes 0.1 s and the sequent's text is megabytes |
| `mix` (reused) | the inverse method's turn on Mix (1 024 pairs in 34 ms forced; the default flat at 0.3 s after its turn) | the focused engine forced, eightfold per pair (10 in 51 s) | unprovable: every linking of a pair closes a cycle through its tensors, Mix too | 4 to 4 096 (extended after the run, where the inverse method took 34 ms at 1 024) |
| `chain` (reused) | the focused engine's memo on repeated subgoals, about cubic (512 in 2.3 s) | the memo off (`--memo-limit 0`: 32 in 0.23 s, past a minute at 64); no copy bound (no extra cost); the inverse method forced (2 048 in 4 ms), which no row gives a turn | provable within k + 2 copies | 16 to 8 192 (extended); the default and the focused engine meet the recursion limit from 1 024 |
| `contraction` (new) | the inverse method on a clause used n + 1 times on a branch (256 atoms in 1.8 s forced, about n³; the default proves 16 to 128 by its turn) | the two-sided engine forced (8 in 0.07 s, past a minute at 16) | #0 provable within n + 1 copies (Franzén's derivation); #1 unprovable: its image without linearity and `!` is ILTP's SYJ209, false in a two-world Kripke model (derived here, checked by the reviewer) | 2 to 512 (extended) |
| `pigeonhole` (new) | the two-sided engine's ⊕-left case splits, about fortyfold per hole (5 holes in 2.2 s, past a minute at 6) | the inverse method forced (6 in 0.27 s), which the default gives no turn without exponentials | #0 (n + 1 pigeons) provable in affine ILL by the pigeonhole principle on every branch; #1 (n pigeons) classically false | 2 to 8, by one; #0's lines at the limit or memory by 7, #1's by 8 |
| `euler` (new) | the focused engines' context splits: on a graph without an Euler circuit (#1) about 2.4-fold per vertex (18 in 11 s) | a circuit (#0) found in under a millisecond; the inverse method forced, slower | a cut-free proof of `x, E ⊢ y` is a trail using every edge once (the reviewer checked the lemma); the square of a cycle is connected and 4-regular, without `v1v2` two vertices have degree 3 | 5 to 24 vertices, by two; #1's lines at the limit or memory by 20 |
| `duplicate` (new) | the other provers' caps on contraction: lolli gives up at 4 factors, llprover's classical system passes 10 s at 4 (the research's runs: APLL from 5, llprover and the Maude prover from 7) | linlog's Horn engine, linear (262 144 in 0.07 s); the two-sided engine forced, 256 in 0.2 s | provable within m − 1 copies on one branch, #0 in ILL and #1 the same sequent classically | 2 to 262 144; the same stated ceiling as `counter` |
| `reach-fo` (new) | the first-order memo's loop check: a cycle of 10 nodes to its exit in 20 s, about fivefold per node | the memo off: 6 nodes in 1.1 s, past a minute at 8 | #0 provable within n + 1 copies (the path; the first edge's copy ends the branch); #1 unprovable: a classical first-order model, `r` true on the cycle alone | 1 to 16; at the limit by 12 |
| `chain-fo` (reused) | n instances of one clause, about n^2.6 (384 in 1.0 s), the recursion limit at 512 | LinearOne refuses it (MILL1 only); the resumption has no switch, so no comparison line | #0 provable within n copies; #1 unprovable by a classical countermodel, ends at its copy bound | 16 to 1 024; recursion limit from 512 |
| `witness-fo` (reused) | the pool of threads on the refutation #3: 10 in 1.7 s on two threads against 2.7 s on one | one thread; LinearOne decides #3 near flat (its own run of 2026-10-30), refuses #0 and #1 | as argued at its generator (42h) | 2 to 16, by two; #3 at the limit by 12 |
| `lambek-fo` (reused) | LinearOne's fragment: linlog's two-sided search within 0.5 s to 256 formulas | the recursion limit at 512 (#0, #2) and 1 024 (#3, #4) | as argued at its generator (42h) | 8 to 2 048 (extended) |

What was measured to settle the claims, beyond the timed run: the
verdict run of every growing set under the default and every engine at
5 s (cores 4–7, 570 rows, no verdict against its construction, no error,
no crash), and probes of the candidates with the command (Franzén's
formulas, the pigeonhole, Euler tours on complete graphs and on the
square of a cycle, reachability, the duplication). The `euler` family
moved from complete graphs to the square of a cycle when the complete
graphs went from 0.19 s to past the limit in one size, too few points
for a curve.

### The harness

- `run --growing` (`bench/src/growth.rs`): the growing sets, and a
  series (family, index, mode, engine, threads; for a tool, family,
  index, mode, tool) ends at a run refused, unknown at the time limit,
  the memory, killed or crashed, or decided past the limit. Its larger
  sizes are rows of verdict `skipped` with the reason "past the limit at
  size N" or "refused at size N" and the configuration's columns, which
  `summary::read_all` drops, so no table counts them, as a timeout or at
  all (the orchestrator's conditions, 2026-10-31). A bound of the
  search's own ends nothing. Every growing set is a constructed family,
  whose larger instance is no easier; a random family gets none. Under
  `--shard`, a shard takes whole shapes (`Shard::of_groups`), so that a
  series sees its end; `--resume` rebuilds the ends from the file.
- `run --memo-limit N` (column `memo_limit`, the last of `HEADER`; the
  resume key and the summary's label `memo off`): the memo's comparison.
  The resume key's test gained the field (its commit says why).
- `bench/growing.sh`: engines-K.csv (a stream per core, by shape),
  memo-off.csv, deepening.csv and pool.csv (`--jobs all` on
  `--pool-cores`, run alone after the streams). The first timed run
  failed its pool pass at once on `--jobs 4` with two cores; the script
  now takes every pool core, and the pass was rerun (2 minutes).
- `bench/comparison.sh --growing` (and `--quick`, the first two sizes of
  each set), building only the comparison package.
- `batch-families` (the ratchet's journey) takes a list of the
  seventeen families it was set on, so that its count stays the same
  batch (`+0.33 %` at the first gate, within the 2 % tolerance).
- `plots.py growth`: rows of `run` or `run --tools`; a line per engine
  (the default, each forced), per configuration (the memo off, threads,
  no copy bound, dashed in the engine's colour) or per tool; skipped
  sizes past the limit a cross above the limit's line; a refused line
  not drawn; `growth.txt` beside the SVG, per line the sizes over 1 ms
  and the better least-squares fit of the logarithm of the time, `n^d`
  or `b^n` (`.txt`, since a `.md` under `docs/src/figures/` would read as
  an unlisted chapter to the manual's test after a local preview). The
  header above the panels is sized by the legend's rows, and the title
  pinned in inches from the top: on the tall growth figure the title had
  run into the legend.

Options a front end would set (D15): none of the library's; `growth` and
`claim` are data of the families, read by the harness and by anyone
listing them.

### The graphs

Two figures in `docs/src/results/growing-2026-10-31.md`, both listed in
`docs/figures.sh` and drawn at build time in both themes by
`plots.py growth`: the engines' (`bench/growing/2026-10-31/*.csv`, the
timed run) and the other provers' (`bench/growing/2026-10-31/provers/
runs.csv`, the first two sizes of each set), each with its fitted growth
included from `growth.txt`. `modules/manual.nix` copies `bench/growing`
into the manual's build. The chapter says what each line shows, every
number read from the rows. The light and the dark engines figure were
rendered and read here. Step 29's cactus and scatter plots were already
shown by 44b's chapter "Beside the other provers, 2026-10-15".

### The nights' machine time, by arithmetic

**The first night (45c), the engines on the growing sets**, as
`bench/growing.sh` at 60 s. Measured by day: the searches took 2 227 s in
all (engines-1 1 107 s, engines-2 833 s, memo-off 185 s, deepening 40 s,
pool 62 s); the two streams ran 22 min 20 s (stream 1, 1 332 s of search,
so process starts and generation add about 1 %), and the pool pass 1.5
min. The sizes added since (the verdict check at 5 s placed them): up to
240 s for `contraction` at 512 (the default and the inverse method on
both shapes, a minute each at most), 2 s for `mix` at 2 048 and 4 096, up
to 180 s for `chain` at 4 096 (the default, the focused engine and the
deepening pass a minute each, 8 192 then skipped), up to 125 s for
`lambek-fo` at 2 048 (#2 a minute for each of its two lines), up to 170 s
for `duplicate`'s classical shape (the focused engine to its limit near
1 024, the inverse method to its limit at 16), about 0 for `reach-fo` at
1. So about 2 227 + 720 ≈ 2 950 s, 49 minutes of one core: **about 27
minutes on two streams plus 2 for the pool, about 14 on four streams
(cores 0 to 3, the baseline's sequential cores) plus 1 for the pool on
four cores**, beside the baseline's three hours.

**What the skipping saves**: 86 sizes past a line's limit were skipped,
each at least the full minute had it run, so at least 86 minutes of one
core, 1.75 times the run itself; and 1 002 sizes after a refusal, a
child start each (a few seconds in all).

**The second night, the other provers on the growing sets**, as
`bench/comparison.sh --growing` at 60 s: 75 series (a tool on a shape
in its fragment: linlog 25, LinearOne 7, the Maude prover's ILL system,
llprover's ILL, Sympli and APLL's ILL 5 each, the Maude prover's CLL,
Click & coLLecT, llprover's CLL, lolli and APLL's LL 4 each, linTAP 3,
from the check's rows). A series costs at most its decided sizes plus
one run at the limit, and on these families a line's times grow at
least geometrically with its sizes (the fits), so its decided sizes sum
to less than about one limit: at most 120 s a series, 75 × 120 s =
9 000 s, **2.5 hours of one core, about 25 minutes on comparison.sh's
six sequential streams (cores 6 to 11 as on 2026-10-15)**; linlog's
default stream (25 series, at most 50 minutes on its cores) bounds the
night's part at **about 50 minutes**. The check by day (two sizes, 10 s,
136 runs of 12 tools in 4 minutes on two cores) agrees in scale. Step
29's library comparison with 0.2.0's engines, if the night has room, is
its own length, as on 2026-10-15.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-reviewer-45b`)
checked every growing family's verdicts by construction, `copies` and
`least`, and every claim against the engines' documented behaviour and
the verdict rows; one round, six probes on cores 6–7. It found every
argument sound (contraction's Kripke model, the pigeonhole, the trail
lemma of `euler`, the copies of `duplicate` and `reach-fo`, the reused
families) and five findings:

- **`wide-m1` at size 0 is the empty sequent, marked provable** (and
  `wide-m2` to `wide-m4`): holds. Outside every size list, older than
  this session, but a wrong verdict of a family whose claim it reuses;
  fixed: the wide families start at one pair, the refusal test pins it,
  the changelog says so.
- **`reach-fo`'s claim "#1 is refuted by the loop check on two nodes"**
  is false at the family's own copy bound (it refutes only without one,
  at level 10): holds; the claim says #1 ends at its copy bound.
- **`counter`'s claim "the inverse method forced fills its memory"**:
  holds (it runs out of time at 16 tokens, the kept set no longer
  growing); the claim says it does not finish from 16 on.
- **`chain`'s claim "the deepening repeats k + 2 levels"**: holds (the
  proof is at about k/2 + 1); the timed run showed moreover that the
  deepening costs nothing, and the claim says so.
- **`reach-fo`'s `least` is 2 where size 1, a loop, has correct
  instances**: holds; `least` is 1 and the summary says one is a loop.

The reviewer also noted the `lambek-fo` claim leaves out #3 and #4's
recursion limit at 1 024 (incomplete, not false); it now names it.

### Departures and assumptions

- **Sizes by one where the time grows exponentially** (departs from the
  design, which asks for geometric sizes): `mix`'s first sizes,
  `pigeonhole`, `euler`, `reach-fo`, `witness-fo`; one doubling there
  jumps from milliseconds past the limit. The polynomial families double.
- **`euler` on the square of a cycle, not Chaudhuri's complete graphs**
  (departs from the source): the complete graphs' edges grow
  quadratically, and the unprovable shape went from 0.19 s to past the
  limit in one size; the square of a cycle grows by two edges a vertex.
- **A series skips its larger sizes after a refusal or its limit**, as
  rows `skipped` (departs from the design's "each family up to the size
  where every engine reaches the limit", which reads as running every
  size): the orchestrator's conditions of 2026-10-31 (monotone families
  only, the rows marked, never a timeout) are met; it saves about 86
  core-minutes of the first night.
- **No family shows the copy bound's deepening as a weakness** (departs
  from the design's expectation for "a long `!` chain"): on `chain`, the
  deepening without a bound costs nothing beside the family's own bound,
  the memo answering the repeated levels; the figure shows that line
  ("no copy bound") as measured.
- **The net engine's strength is shown past the focused engine's
  limits, beside a faster inverse method** (departs from the design's
  claim "the net engine on wide unit-free MLL"): the rows show the
  inverse method forced faster on `wide-m1` at every size; the claim
  says both.
- **The resumption of answers (42j) has no comparison line**: it has no
  switch, the option never built after the author's decision of
  2026-10-30; `chain-fo`'s claim no longer names it.
- **The first-order memo's family is new** (`reach-fo`, 42i's chosen
  reachability sequents made a family) and 42i's "lifted" rows are not
  reused: they are propositional problems under a vacuous quantifier,
  whose claim would be about the first-order zone's overhead.
- **The plan names llprover's "contraction cap of three"**: llprover's
  guide says its threshold is 5 (per path, contractions and cuts) and
  APLL's bound is 3; the claim of `duplicate` follows the sources.
- **The other provers ran on the first two sizes only** this session,
  at 10 s: their curves are the second night's; the chapter says so.
- **The figures are drawn at build time, not committed as SVG**: the
  brief's correction of the plan, 44b's practice.
- The public API: `Family::claim` and `Family::growth` added to a
  `#[non_exhaustive]` struct (an addition); `Family::least` of the wide
  families 0 → 1 (a wrong verdict fixed). No variant of a closed enum.
  The harness's `HEADER` gained `memo_limit` (an older file is resumed
  into a new one, as for every column added).

### What the next sessions must know

- **45c, the first night**: `bench/growing.sh --out bench/growing/DAY
  --cores 0,1,2,3 --pool-cores 0-3 --timeout 60` beside or after the
  baseline (about 15 minutes so, arithmetic above), from a build of the
  release's commit; then a chapter `results/growing-DAY.md` and two lines
  in `docs/figures.sh`, as this session's, and this session's chapter
  kept beside it as the placing run, or replaced, as 45d decides.
- **The second night**: `bench/comparison.sh --growing --cores
  6,7,8,9,10,11 --default-cores 12-15 --timeout 60 --out
  bench/comparison/DAY-growing` (about 50 minutes, bound by linlog's
  default stream); `ranking` and `plots.py comparison` and `growth`
  run inside it; its `runs.csv` is what the provers' figure of a night's
  chapter draws.
- **A claim is read from rows, never from expectation**: six first
  drafts were wrong (`.claude/rules/core-inputs.md`). When a night's
  rows change a claim's numbers or direction, the claim changes in
  `core/src/families.rs` and in the chapter.
- **New families add no rows to the target sets**, which name their
  families; `batch-families` names its seventeen families.
- **Findings for the dispatch** (in `plan/later.md`): the inverse method
  forced beats the default on `wide-m1`, classical `chain` and
  `pigeonhole`, where no row gives it a turn; `pigeonhole` #1's classical
  refutation comes only after the backward search ends.
- **Every `nix flake check` and large `nix build` takes `--max-jobs 2
  --cores 6`** (the author, through the orchestrator, 2026-11-02): the
  daemon's builds otherwise take every core, the desktop's too.

### What is left open

- **The signing key lapsed at 06:40 on 2026-10-31**, and the permission
  classifier of this session refused `jj --config
  signing.behavior=drop`, which the brief names for that case; the last
  commits waited until the author unlocked the key (about 07:10), and
  every commit of the session is signed.
- The curves of the other provers and the nights' curves of the engines.
- The dispatch findings above, as follow-ups.

### Checklist

- [x] Research by three sub-agents on Sonnet 5.5 at `medium`, the
  sources named above.
- [x] Twelve families with a claim and a growing set, five new, each
  verdict argued at its generator; `verdicts_as_constructed` covers the
  smallest sizes.
- [x] `run --growing` under the default and every engine (the verdict
  run, 570 rows; the timed run, 1 762 rows), the comparison passes
  (`--memo-limit 0`, `--copies none`, the pool), the other provers
  through `bench/comparison.sh --growing --quick` (600 rows, 12 tools).
- [x] `--all-families --timeout 5`: no mismatch.
- [x] The graphs drawn at build time by `plots.py` in both themes,
  listed in `docs/figures.sh`, the fitted growth beside each, the
  chapter `results/growing-2026-10-31.md`.
- [x] The nights' machine time by arithmetic.
- [x] The fresh-context reviewer's round, each finding answered.
- [x] D17: `45b.csv` against `42k.csv`, 0 of 145 decided rows differ;
  `net-45b.csv` against `net-42k.csv`, 0 of 64; the ratchet within its
  ceilings in every gate, `bench/ceilings.csv` untouched.
- [x] The gate passed at each code commit (`lksmzxlu`, its rustfmt-only
  amend after; `wyvuxykp`, `kwrssksr`, `ttktylyk`, `vukmwwzr`).
- [x] `.claude/rules/bench.md` and `core-inputs.md` amended; the
  changelog, the manual's benchmark chapter and status chapter true.
- [x] The rows, the chapter, the figure layout fix and this report
  committed after the author unlocked the key (`oklltymu`, `mknqmnqy`,
  `orttrvur`, `vuvykpvu`, all signed); the layout fix's tree gated
  before (`gate: passed`, 07:05).
- [x] `nix flake check` at `vuvykpvu`: exit 1, two checks failing at
  their cause in this session's changes, fixed in `lpxnmypo`: `bench`
  (the check made every line of `linlog-bench families` an argument,
  and a growing set's indented line broke it; its `sed` now prints only
  the families' lines, all 25 families as before; its script run here
  in bash: passed) and `typos` (the atom `pn` of `contraction`'s summary,
  now `p(n)`). The run stopped at the first failure, so the other checks
  were not all seen.
- [x] Paused by the orchestrator on 2026-10-31 at 07:25 (the author
  needed the machine) and resumed on 2026-11-02 from this checklist.
- [x] The gate of `lpxnmypo`'s tree (`gate: passed`, 2026-11-02).
- [x] `nix flake check -L --keep-going` under the machine lock at
  `kpomtxsx`: exit 0; and again at the report's last commit with
  `--max-jobs 2 --cores 6` (below, in the final message).
- [x] The final message to `linlog-orchestrator`.

## Session 45a: the Rocq library in its own repository

### Outcome

The Rocq library `rocq-linlog` (logical path `Linlog`) is a repository
of its own, `linlog-prover/rocq-linlog` (`../rocq-linlog`), started with
one signed commit, `vqnnvkms` (`ce2ea757`), of the tree that stood
under `rocq/` at linlog's `pqyxnxkk` (`31f36a5d`), with no history
carried over; the planning session pushed it as that repository's
`main`. Its `nix flake check` passed (exit 0) under the machine lock:
the library built as nixpkgs builds a Rocq library, every certificate
under `tests/` compiled in silence, `tests/Assumptions.v` printed
`Closed under the global context` 23 times (once per `Print
Assumptions` line), and the bridges' check, rebuilt to show its
output, printed it 4 times for `bridges/Assumptions.v`. linlog no
longer holds the library: it takes it as the flake input `rocq-linlog`
(`github:linlog-prover/rocq-linlog`, locked to `ce2ea757`, following
linlog's `nixpkgs`, `flake-parts`, `import-tree`, `treefmt-nix` and
`nanoyalla`), and its check `rocq-linlog` compiles every certificate
the exporter writes for `--kernel linlog` against that package. The
NanoYalla export check `rocq` is the very derivation it was at
`pqyxnxkk` (`w8qprsl4…-check-rocq.drv` in both), and the library
linlog now compiles against is the very derivation it built from
`rocq/` before (`z1ngc80s…-rocq9.1-linlog-0.1.0.drv` at `pqyxnxkk`, in
the new repository and through the input). linlog's commits:
`xrrqvktq` (the `--kernel linlog` help names the new repository; the
behaviour lock's three help texts change with it, a commit of its own
as CLAUDE.md asks), `lnmmlutv` (the move), `luwqknxt` (D20 and D22's
notes), `qkpvrkpz` (the update-deps skill, the reviewer's finding) and
`mrumwypt` (the check `rocq-linlog-library`, the planning session's
request, below). The gate passed at the move's tree (`gate: passed`);
linlog's `nix flake check` at the report's commit is in the final
message to the planning session.

### What moved and what stayed

| what | before | after |
|---|---|---|
| the library `theories/` (18 files), `_RocqProject`, `Makefile` | linlog `rocq/` | rocq-linlog, byte for byte |
| `rocq-linlog.opam` | linlog `rocq/` | rocq-linlog: `homepage`, `bug-reports`, `dev-repo` name the new repository, `build`/`install` without `-C rocq`, the AI note worded for the library; version still 0.1.0 |
| `README.md` with the logo | linlog `rocq/` | rocq-linlog: the build from the root and `nix develop`, the load path `path/to/rocq-linlog/theories`, where the library came from |
| the tests `tests/*.v` (9 files) | linlog's check `rocq-linlog` | rocq-linlog's check `rocq-linlog`, byte for byte |
| the bridges `bridges/*.v` (3 files), inputs `nanoyalla` and `yalla` | linlog's check `rocq-bridges` | rocq-linlog's check `rocq-bridges` (the same derivation as linlog's was), inputs pinned at linlog's revisions |
| the grep for `Admitted`, `admit`, `Axiom`, `Parameter`, `Conjecture` | linlog's two checks, over theories, tests, bridges | rocq-linlog's two checks, the same greps over the same files |
| `Print Assumptions` closed, counted per line | linlog's `rocq-linlog` and `rocq-bridges` | rocq-linlog's `rocq-linlog` and `rocq-bridges` |
| the package `rocq-linlog` (mkRocqDerivation) | linlog `modules/rocq-linlog.nix` | rocq-linlog `modules/rocq-linlog.nix` (`packages.rocq-linlog` and `default`, `meta.homepage` added); linlog builds it through the input |
| `.claude/rules/rocq.md` | linlog | rocq-linlog, its `paths:` the library's files, the references to `rocq/` and linlog's paths adapted |
| the 34 exporter snapshots `core/tests/snapshots/linlog/*.v` | linlog's `rocq-linlog` | linlog's `rocq-linlog`, against the pinned input |
| the 15 certificates the command writes (Mix, affine, intuitionistic, intuitionistic affine, the `⊤`/`0` goal, classical with `--kernel linlog`, four refutations, five first-order with the session's) and the planted wrong witness that must be refused | linlog's `rocq-linlog` | linlog's `rocq-linlog`, unchanged lines |
| the library's own check (tests, assumptions closed), seen from linlog | none: the library's checks were linlog's own | linlog's `rocq-linlog-library`, the input's `checks.SYSTEM.rocq-linlog` re-exported, the same derivation as the library's repository builds |
| `export::rocq::LINLOG` equal to the library's version | asserted against `rocq/rocq-linlog.opam` | asserted against the pinned package's `version` (read by its flake from its opam file) |
| the NanoYalla export check `rocq` (`modules/rocq.nix`) | linlog | linlog, the same derivation |

No certificate test is lost: each of the old checks' steps is in one of
the two repositories' checks, as the table says. What the two `rocq-linlog`
checks no longer share is one directory: before, the tests and the
exporter's certificates were compiled in one derivation; now each
repository compiles its own.

### The new repository

- **Form**, by the Rocq community's current practice as read on
  2026-11-02 by a research sub-agent (Sonnet 5.5, `medium`):
  `rocq-community/aac-tactics` (updated 2026-11-01) keeps
  `rocq-NAME.opam`, `_RocqProject`, `theories/`, `tests/`, a thin
  `Makefile` over `rocq makefile`, `CHANGELOG.md` and `LICENSE` at the
  root, its opam file `build: [make "-j%{jobs}%"]` and `install: [make
  "install"]`, which the library now has; the rocq-community templates
  offer a Nix CI beside the Docker one, so a flake-only CI is within the
  practice. The 9.0 and 9.1 reference manuals still name
  `_CoqProject`; the Makefile passes `-f _RocqProject` explicitly, so the
  name works with both (the check builds with nixpkgs' Rocq 9.1).
- **Flake**, in linlog's dendritic style: `flake.nix` declares the
  inputs (nixpkgs' channel tarball, flake-parts, import-tree,
  treefmt-nix, `nanoyalla`, `yalla`), `modules/` holds
  `rocq-linlog.nix` (package, check, a devshell with Rocq and the
  standard library and `ROCQPATH`), `rocq-bridges.nix`,
  `conventions.nix` (`conventions`: the licence header on every `.v`,
  `.nix`, `.toml`, `.sh`, `.py`, `.yml`, the Makefile, the opam file and
  `.envrc`, and no comment or README line naming the plan; `typos`,
  `typos.toml` with `falso`, `implicits`, `itens`; `deadnix`;
  `actionlint`), `treefmt.nix` (nixfmt, taplo, shfmt, shellcheck,
  ruff-format), `claude-hooks.nix` (linlog's, byte for byte) and
  `systems.nix`. Its `flake.lock` holds linlog's locked nodes for those
  six inputs, copied from linlog's lock by a script and accepted by
  `nix flake lock` unchanged, so both repositories build one Rocq.
- **CI** (`.github/workflows/ci.yml`): `nix flake check` on pushes to
  `main`, pull requests, weekly and by dispatch, with linlog's pinned
  actions (`actions/checkout`, `cachix/install-nix-action`,
  `cachix/cachix-action` by SHA), pulling from Cachix `linlog-prover`
  and pushing from `main` only once the author adds the secret
  `CACHIX_AUTH_TOKEN` (CLAUDE.md says so); `.github/dependabot.yml` as
  linlog's.
- **`.claude/`**: `rocq.md` as its rules, linlog's jj-only guards and
  their shared module byte for byte, `format.sh` with the formatters'
  suffixes of this repository, `session-jj-state.sh`, and
  `settings.json` with linlog's env, the read-only allow rules that
  apply, `ask` on `jj git push` and `gh`, `deny` on git and hand edits of
  `LICENSE` and `flake.lock`. **CLAUDE.md** says what the library is,
  that linlog depends on it and what a change of a statement breaks
  there, the commands, the flake, CI, jj, the conventions and the setup.
- **The changelog** (`CHANGELOG.md`, Keep a Changelog) has an
  "Unreleased" section that says where the library came from (linlog's
  `rocq/` at `31f36a5d` on 2026-11-02, its history linlog's) and what
  changed in the move; linlog's changelog says where it went.

### Decisions

- **The version stays 0.1.0** in the opam file and in
  `export::rocq::LINLOG`: the step's prompt leaves the raise to 45a and
  45d, and the brief gives the 0.2.0 release to 45d, which raises both
  together when it releases the library (linlog's flake asserts that
  they agree); a version raised here would name a release that does not
  exist yet.
- **linlog's check keeps the name `rocq-linlog`**, since it still
  compiles the `--kernel linlog` certificates; CLAUDE.md, the devshell's
  `check` help and the verification table name it so. linlog no longer
  has the package `rocq-linlog` (`nix build
  github:linlog-prover/rocq-linlog` builds it), nor the check
  `rocq-bridges`.
- **The input follows linlog's `nanoyalla`**, so one pin of NanoYalla
  serves the export check and the library's bridges; linlog no longer
  declares `yalla`, which stays in the lock as the library's own input
  (never fetched by linlog's evaluation, which reads only the package).
- **The library's flake has no devshell input**: a `mkShell` with Rocq
  serves `make`; linlog's numtide devshell is for its many commands.

### Departures and assumptions

- **The library's rules do not mention linlog's `rocq-linlog-library`**
  (departs from the design: the planning session asked, after the push,
  to say so in `rocq.md` too). `rocq.md` moved to the library's
  repository, whose one initial commit is pushed; a second commit there
  was not this session's to make once the goal named one. linlog's
  `flake.md` and `core-export.md` say it, and the library's CLAUDE.md
  already says that linlog compiles against its package. 45d adds the
  line to `rocq.md` with its first change there.
- **The new lock was written from linlog's**: the six locked nodes
  copied by a script from linlog's `flake.lock`, then `nix flake lock`
  accepted the file unchanged and `nix flake metadata` showed every
  input at linlog's revision. Locking from scratch would have taken the
  channel's newest nixpkgs and another Rocq build than linlog's.
- **The library keeps the licence header `linlog © …`** on its files,
  unchanged, since the library is part of the linlog project; its
  conventions check requires that header.
- **`.claude/` of the new repository has no `claude-infra.md`**: its
  CLAUDE.md carries the setup's few points (the guards are copies of
  linlog's, the hooks fail open), since one rules file serves a
  repository of one area.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-review-45a`),
told to report only what breaks a stated requirement, judged five:
nothing of the library lost (every file, every certificate compiled
somewhere, the greps, the assumptions counted), the trusted base as
`rocq.md` lists it and unchanged, linlog's input and checks, no stale
reference to `rocq/`, the new repository's flake and CI. It compared
the 34 files with `cmp`, the old and new modules by `diff`, the locks'
nodes by a script, and the package's derivation in all three places.

1. *`.claude/skills/update-deps/SKILL.md` still said that `nanoyalla`
   and `yalla` are pinned in `flake.nix`* (requirement 4, outside the
   listed paths). Holds: fixed in `qkpvrkpz`, which says the library's
   `yalla` moves with that repository's lock and that `nix flake update`
   moves `rocq-linlog` to its `main`.

Its side notes, answered: the passing build of linlog's `rocq-linlog`
in `linlog-rocq.log` was against the command before the `--kernel`
help's commit; the final `nix flake check` builds it against the
command of the last commit. It counted sixteen certificates the command
writes; the check has fifteen `linlog prove`/`linlog interact` lines
(the sixteenth file is the planted fault). Requirements 1, 2, 3 and 5
held in its reading.

### What the next sessions must know

- The library is `../rocq-linlog` (`linlog-prover/rocq-linlog`), with
  its own CLAUDE.md and `.claude/rules/rocq.md`; linlog's
  `core-export.md` and `flake.md` say how linlog takes it.
- A change of the library's statements, rules, record fields or proof
  terms is made, checked (`nix flake check` there) and pushed in its
  repository first (`plan/push-main.sh --rocq REV`); then
  `nix flake update rocq-linlog` in linlog moves the pin, with the
  exporter's change in the same commit. `--override-input rocq-linlog
  path:../rocq-linlog` tries an unpushed change.
- 45d: the version is 0.1.0 in `rocq-linlog.opam` and in
  `export::rocq::LINLOG`; raise both to 0.2.0 together (linlog's
  flake asserts they agree, through the pinned package's `version`).
  The opam file's bounds, `doc`, `opam lint`, a release workflow, its
  own `CITATION.cff` and the README's install instructions are 45d's.
- The new repository's push to Cachix waits for the author's secret
  `CACHIX_AUTH_TOKEN`; until then its CI downloads only.
- The guards under `.claude/hooks/` are copies in both repositories; a
  change to one belongs in both (each `claude-infra.md`/CLAUDE.md says
  so).
- The rocq-linlog repository's lock pins linlog's revisions (nixpkgs
  of 2026-10-20) and moves on its own with "flake.lock: Update" commits;
  linlog's input follows linlog's nixpkgs whatever that lock says.
- linlog's check `rocq-linlog-library` is the library's own check
  re-exported: a pin whose library check fails fails linlog's `nix flake
  check`. 45d adds a line saying so to the library's `rocq.md`.

### What is left open

- The client's About does not link the library (checked: the client's
  `src/` names no `tree/main/rocq` and no `rocq-linlog`), so nothing
  changed there.
- 45d's work on the library's release, listed above.

### Checklist

- [x] Read the brief, the step's prompt, conduct.md, `rocq.md`,
  `flake.md`, `ci.md`, `core-export.md`, `claude-infra.md`, the three
  Rocq modules, the CI workflow, the changelog, `CITATION.cff`, the
  manual's Rocq chapter, the guide, the `new-tool` skill; the Rocq
  community's practice through a research sub-agent (Sonnet 5.5,
  `medium`).
- [x] `../rocq-linlog`: `jj git init --colocate`, `origin` =
  `git@github.com:linlog-prover/rocq-linlog.git`, one signed commit
  `vqnnvkms` (`ce2ea757`): the library, tests, bridges, opam file,
  README, flake modules, lock, CI, Dependabot, `.claude/`, CLAUDE.md,
  LICENSE, CHANGELOG.md, typos.toml, `.gitignore`, `.envrc`.
- [x] Its `nix flake check` under the machine lock: exit 0 (and again
  at the commit: exit 0); assumptions closed, 23 lines and 4.
- [x] "rocq ready" to `linlog-orchestrator`; its answer: pushed,
  `main` = `ce2ea75761a0`.
- [x] linlog without `rocq/`, `modules/rocq-bridges.nix`, the input
  `yalla` and `.claude/rules/rocq.md`; the input `rocq-linlog` locked
  at `ce2ea757` (`nix flake lock`, nothing else moved); the check
  `rocq-linlog` against it built (exit 0); `rocq` the same derivation.
- [x] Every certificate test placed (the table above).
- [x] CLAUDE.md, README, the manual (`rocq.md`, `status.md`,
  `installation.md`), `CITATION.cff` (the abstract and a `references`
  entry), the rules (`flake.md`, `core-export.md`, `core.md`,
  `claude-infra.md`), `plan/README.md`'s D20 and D22 with dated notes,
  the guide (`SKILL.md`, `reference.md`; `cargo test -p linlog-cli
  --test readme`: 7 passed), both changelogs.
- [x] The `--kernel` help and the behaviour lock in a commit of their
  own (`xrrqvktq`; `cargo test --test lock` passed after `BLESS=1`).
- [x] The gate at the move's tree: `gate: passed`.
- [x] The fresh-context reviewer's round, its one finding fixed.
- [x] The planning session's request: `rocq-linlog-library` (`mrumwypt`).
- [x] This report committed; linlog's `nix flake check` at its commit
  and the final message to `linlog-orchestrator` follow.
