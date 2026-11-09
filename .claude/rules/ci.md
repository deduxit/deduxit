---
paths:
  - ".github/**"
---

# CI: the GitHub workflows

Loaded when a file under `.github/` is read or changed.

GitHub Actions runs `.github/workflows/ci.yml` on every push to `main`, on
every pull request and weekly: `nix flake check`, and the online
`cargo deny check advisories` in the devshell. A workflow installs nothing
but Nix, so CI checks with exactly the tools flake.lock pins.
`.github/workflows/docs.yml` publishes the flake's `doc` package, the rustdoc
of `main` with the manual under `manual/`, to GitHub Pages on every push to `main`. `.github/workflows/release.yml`
releases a version tag `vX.Y.Z` (below).

- **Every tool comes from the flake.** A job installs Nix and runs
  `nix flake check`, `nix build` or `nix develop --command …`. No
  `setup-rust`, `apt` or `cargo install`: they would check with other versions
  than the devshell does. A new check goes into the flake
  (`modules/checks.nix`), not into a workflow step. Two exceptions, each
  for what Nix cannot make: `packages.yml` builds the macOS and Windows
  binaries with rustup on their runners (a Nix-built macOS binary links
  the store's libraries, and Windows has no Nix), at the compiler version
  the flake pins, which its first job reads with `nix develop`; and
  `install.yml` runs the installation chapter's commands as users run
  them, with each system's own tools, which is what it tests.
- **Every action is pinned to a full commit SHA**, with the release as
  a comment: `uses: owner/repo@<sha> # vX.Y.Z`. The organisation
  `deduxit` requires it of all actions, GitHub's own included
  (its Actions policy, as `linlog-prover`'s since 2026-10-03): a `uses:` with a tag or a
  branch fails the run. Resolve a tag with
  `gh api repos/<owner>/<repo>/commits/<tag> --jq .sha`, or without `gh`
  by `nix flake prefetch --json github:<owner>/<repo>/<tag>` (its
  `locked.rev`; `flake metadata` fails on a repository without a
  `flake.nix`), and read that commit's `action.yml` for its inputs. Dependabot proposes the bumps, weekly,
  as one pull request. The pin of `cachix/install-nix-action` also fixes the
  Nix version CI runs.
- **Least privilege**: `permissions: contents: read` at the top, anything more
  per job; `actions/checkout` with `persist-credentials: false`.
- `install-nix-action` gets `github_access_token: ${{ github.token }}`: without
  it, fetching the flake's `github:` inputs from shared runners hits the
  anonymous rate limit.
- **No workflow runs locally.** `nix flake check` runs actionlint, with
  shellcheck on the `run:` scripts, over `.github/workflows/`: it catches
  syntax, unknown runner labels, bad expressions and shell mistakes, not
  behaviour on GitHub. Read run results with `gh run list` and
  `gh run view --log-failed`.
- **The cache: Cachix `linlog-prover`** (public,
  https://linlog-prover.cachix.org, key
  `linlog-prover.cachix.org-1:6K+DDVwwELnSYtfflwUdsHdYuolC7hprdD/r2/KOQZU=`,
  signed by Cachix; the name is Cachix's, not GitHub's, so it stayed
  when the repositories moved to `sequentium` and it became `deduxit`,
  and no Cachix page says a
  cache can be renamed). `cachix/cachix-action` follows `install-nix-action`
  in every job of the four workflows that installs Nix, with the
  repository secret `CACHIX_AUTH_TOKEN` (a write token) and `skipPush`
  unless the ref is `main` (`release.yml` only reads it): pushes to `main`, the weekly run and a dispatch on `main`
  upload, a pull request only downloads. A pull request from a fork gets
  no secret, and the action then only says that pushing is disabled. It
  pushes in daemon mode what the job realised and no upstream cache
  (cache.nixos.org) holds: the toolchain, the dependencies' builds, the
  packages, the documentation and each check's empty result (the
  flake inputs' sources, NanoYalla's among them, are fetched, not
  uploaded: none is in the cache, checked 2026-10-18), so a check whose
  derivation did not change is fetched, not rerun. Whatever a job builds
  goes into the public cache, which is redistribution: a check must not
  build the other provers or the problem libraries. How narrow those derivations are is the
  flake's (`flake.md`): a change to `cli/` leaves the core crate's checks
  as they were. A download is a realised path too: `pushFilter:
  '\.tar\.xz$'` keeps the toolchain's tarballs out (183 MiB of the first
  925 MiB upload, 2026-10-17), and the toolchain's `minimal` profile
  leaves out the manual `rust-docs` (81 MiB). The rest of that upload:
  the unpacked toolchain about 350 MiB, the dependencies' two builds
  (plain and with debug assertions) 323 MiB, deduxit's own 14 MiB. A
  push of code adds about the last; a lock or toolchain bump adds a new
  set of the others, and Cachix evicts the least recently used paths
  past the plan's storage.
- **Docs** deploys `result/share/doc` of `nix build .#doc` with GitHub's own
  Pages actions. The repository's Pages source must be "GitHub Actions".
  The site is `main`'s alone: `deploy-pages` replaces it whole at every
  deployment, so a released version's manual is the `docs/` of its tag
  (and its API docs.rs), not a directory of the site; keeping one per
  release would mean building every tag's manual at every push. The
  manual's links are checked by the flake check `manual-links`, not by
  this workflow, which only builds.
- **Comparison** (`comparison.yml`, `workflow_dispatch` only, inputs
  `timeout` and `quick`) runs `bench/comparison.sh` in ten shards, one
  job each (two sequential streams on cores 0 and 1, deduxit's default on
  2–3), uploads each shard's rows and records, and a last job joins
  them, writes `ranking.md` and the figures, puts the ranking in the
  run's summary and uploads it all as the artifact `comparison`; it
  commits nothing. BenchExec needs a cgroup from the user's systemd, so
  the job enables lingering for the runner's user first and exports its
  bus. It only downloads from the cache (`skipPush: true`, no token):
  it builds the other provers and the LLTP library, and their licences
  do not all let this project redistribute them (APLL and the Maude LL
  prover state none; LLTP is GPL-3.0, and `bench.nix` repairs one of
  its files). It has never run: the first run is the author's.
- **Packages** (`packages.yml`, `workflow_call` only, called by the
  release and the install test): the Linux files from `nix build .#dist`
  on `ubuntu-24.04` and `ubuntu-24.04-arm` (the static musl binary in a
  `.tar.gz`, a `.deb`, an `.rpm` and an Arch Linux package), the macOS
  `.tar.gz` on `macos-26` (aarch64) and `macos-26-intel` (x86_64) and the
  Windows `.zip` on `windows-2025` and `windows-11-arm` (the C runtime
  linked in, `+crt-static`, so that no Visual C++ redistributable is
  needed; the notices come from the first job's `nix build
  .#notices`, the artifact `notices`), each with the
  binary, README, `LICENSE`, the fonts' licences and
  `THIRD-PARTY-NOTICES.txt`, uploaded as artifacts `dist-NAME`. Nothing
  is signed (Apple's notarisation and a Windows certificate are the
  author's decision); the installation chapter says what a user sees.
  No `.msi` and no macOS `.pkg`: WiX's current toolset asks its user to
  accept a EULA with a maintenance fee for revenue above a threshold,
  which is the author's to accept, and an unsigned `.pkg` meets
  Gatekeeper where a `curl`-downloaded archive does not.
- **Install** (`install.yml`): the installation chapter's commands on
  pushes to `main` that touch packaging or the chapter, weekly, after
  each Release run and by dispatch: the packages of the commit installed,
  run and removed in Debian 13, Ubuntu 24.04 and 26.04, Fedora 44 and
  Arch containers (Debian and Fedora also on arm64), the archives on
  macOS and Windows runners, the flake by `nix run` and `nix profile`,
  `cargo install` with Fedora's, Arch's and Ubuntu 26.04's Rust, with
  rustup on Debian 13 and on the macOS and Windows runners, and (not on
  a push) the chapter's downloads from the latest release, skipped while
  it carries no `SHA256SUMS`. A chapter's command that changes changes
  this workflow in the same commit.
- **Release** (`release.yml`, on a tag `vX.Y.Z` only), four jobs. `check`:
  the tag equals `[workspace.package]`'s version, CITATION.cff states it,
  and CHANGELOG.md has a dated section `## [X.Y.Z] - YYYY-MM-DD` (a
  section headed `- unreleased` fails it), then `nix flake check`.
  `publish` (skipped for `v0.1.0`, which the author published by hand,
  since crates.io's Trusted Publishing works only for a crate that
  exists): in the environment `crates-io`, which the trusted publishers
  on crates.io name and whose protection rules ask for the author's
  approval, with `id-token: write`. Its first step lists the crates
  due: `deduxit` and `deduxit-cli` at the tag's version, then `linlog`
  and `linlog-cli` at 0.3.0 (the old names' last release, `compat/`),
  each skipped when crates.io has that version, so the old names
  publish once and never again, and a rerun after a failure finishes
  what is left. It packages the due crates before any token, then
  publishes the crates crates.io lacks with the environment secret
  `CRATES_IO_FIRST_PUBLISH_TOKEN` (scoped to `deduxit` and
  `deduxit-cli` for their first version, removed once they have their
  trusted publishers), and only if a crate crates.io has is due asks
  `rust-lang/crates-io-auth-action` (pinned by commit; one token for
  every crate whose trusted publisher names `deduxit`/`deduxit`,
  `release.yml` and `crates-io`, revoked at the job's end) and publishes
  those. New crates go first: they are the library and the command the
  old names depend on. crates.io finds a trusted publisher by the
  owner's and the repository's names (case-insensitive), then checks
  the owner's id, the workflow and the environment, so a renamed
  repository needs a new entry on every crate (the `linlog-prover`/`linlog`
  entries matched nothing after the move). `release` (`contents:
  write`, after `publish` succeeded or was skipped, never after it
  failed): `gh release create` with the changelog's section as the notes
  (and a line naming the installation chapter), the two `.crate` files
  downloaded from static.crates.io, the ones published, and the
  `packages` job's files with their `SHA256SUMS` and a build provenance
  attestation of each (`actions/attest`, `id-token: write`,
  `attestations: write`), which `gh attestation verify FILE --repo
  deduxit/deduxit` checks; it waits for `packages` too. The cache is only read (`skipPush`): a tag's builds are
  `main`'s. Nothing of it can run before the author's first publication
  and tag; actionlint is all that checks it.
