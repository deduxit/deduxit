# Maintenance: dependencies, toolchains, WebAssembly, accessibility, release channels

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

## 1. What linlog has today

From the repository (nothing below may re-propose these):

- **Toolchain and MSRV.** `rust-toolchain.toml` follows `stable` (minimal profile, with `wasm32-unknown-unknown` and `wasm32-wasip1`); `Cargo.toml` sets edition 2024 and `rust-version = "1.92"` (README: "Rust 1.92 or later"); a flake check `msrv` (`modules/msrv.nix`) builds the workspace on that toolchain. The workspace lints (clippy pedantic plus picks, rustdoc lints) are in `Cargo.toml`.
- **Dependencies.** `Cargo.lock` has 171 packages. `deny.toml` runs licences, bans, sources and advisories; two RustSec ignores are recorded with reasons (`RUSTSEC-2026-0206` rustybuzz and `RUSTSEC-2026-0192` ttf-parser, both under usvg 0.47 and krilla, to be dropped when krilla moves). `cargo shear` (unused dependencies), `deadnix`, `typos`, `actionlint` and `cargo hack` feature checks are flake checks (`CLAUDE.md`, `modules/checks.nix`). `mutants/` (cargo-mutants runs) and `fuzz/` (cargo-fuzz targets on a nightly) exist and are described in `.claude/rules/evidence.md`.
- **Update practice.** The `update-deps` skill (`.claude/skills/update-deps/SKILL.md`) updates, by hand and on request, `Cargo.lock`, exact pins, `flake.lock`, the fuzz nightly, the actions' SHA pins and the client's npm, flake and actions, verifying each with `nix flake check` and committing one change per kind. There is no scheduled bot: `.github` has no `dependabot.yml` (listing of `.github/workflows` read: `ci`, `comparison`, `docs`, `install`, `packages`, `release`). `ci.yml` also runs `cargo deny check advisories` weekly.
- **Flake inputs and their ages** (`lastModified` in `flake.lock`, read 2026-11-08): nixpkgs 2026-10-20, rust-overlay 2026-10-22, rocq-linlog 2026-11-02, and the other inputs between 2026-04-13 and 2026-10-18 (the oldest, from 2026-04-13, is an input I did not identify, unsourced beyond the lock's timestamp).
- **Actions.** Every action is pinned to a full commit SHA with its version in a comment (checkout v7.0.1, install-nix-action v31.11.1, cachix-action v17, upload/download-artifact v7.0.2/v8.0.2, `actions/attest` v4.2.2, `rust-lang/crates-io-auth-action` v1.0.5, deploy-pages v5.0.1).
- **Release channel.** `release.yml` publishes to crates.io through Trusted Publishing, builds archives and packages in `packages.yml` (static musl binaries for x86_64 and aarch64 Linux via `modules/dist.nix`; `.deb`, `.rpm`, Arch `.pkg.tar.zst` by nfpm; macOS and Windows archives on runners), writes `SHA256SUMS` and a provenance attestation with `actions/attest` (verifiable with `gh attestation verify`), and creates the GitHub release. `install.yml` runs the installation chapter's commands on Debian, Ubuntu, Fedora, Arch, macOS, Windows and Nix, on pushes touching the packaging, weekly and after each release. The installation chapter says openly that the macOS and Windows binaries are not signed (`docs/src/installation.md`, lines about quarantine and SmartScreen). The static binary is musl-linked and the chapter notes musl's limits; a glibc binary is the follow-up named in 45d (`plan/later.md`).
- **WebAssembly.** `web/` is the `linlog-web` crate for `wasm32-unknown-unknown` without `parallel`, `wasm-bindgen` pinned exactly (`=0.2.129`, `=0.3.79` for the test crate); `modules/web.nix` takes the CLI at the version `Cargo.lock` pins and fails with a thrown message when nixpkgs lacks that version, runs `wasm-opt -O3`, ships cargo-about notices and runs the bindings' tests under Node. A `wasm` check builds the library for wasm32 with all features but `parallel`, and `web-core-wasm32` runs the integer-sensitive tests on `wasm32-wasip1`. No `-Ctarget-feature` flags are set (grep of `.cargo`, `web/`, `modules/web.nix`: none), so the module uses the Rust defaults of the day.
- **The client** (`linlog-prover.github.io`, `package.json`): Astro 7.3.7, Solid 1.9.17, Tailwind 4.3.3, TypeScript 6.0.3, Vitest 5.0.3, Playwright 1.64.1, Node 24 or later, all exact pins.
- **Accessibility built.** D33 in `plan/web/design.md` fixes WCAG 2.2 at AA for the page (keyboard 2.1.1, focus 2.4.7 and 2.4.11, dragging 2.5.7, target size 2.5.8, reflow 1.4.10, non-text contrast 1.4.11, status messages 4.1.3 in live regions); D34 honours reduced motion, colour scheme and contrast preferences; interactive SVG is not placed in `role="img"` (clickable formulas are HTML buttons beside a drawing that has a title and description). The sweep in `plan/reports/38-web-polish.md` (item 10) fixed ten defects and listed eleven judgement calls, among them 44-pixel targets and a repainted green at 4.5:1. The exports' SVG reading for screen readers is part of the SVG exporter (`core/src/export`).
- **Open with the author** (`plan/handoff.md`): code signing for installers, the WiX EULA for a Windows `.msi`, channels that need an account (a Homebrew tap, the AUR, winget, Scoop, COPR, Chocolatey), rocq-linlog's `CACHIX_AUTH_TOKEN`. nixpkgs submission is skipped at the author's word ("any nix user can already get" it from the flake).

## 2. The field

All facts below were read from the cited page on 2026-11-08.

**Rust release rhythm.** The Rust blog lists stable releases about every six weeks: 1.99.0 on 2026-10-01 and 1.99.1 on 2026-10-15 are the latest; 1.98.0 was 2026-08-20, 1.93.0 2026-01-22 [1]. linlog's `rust-version = "1.92"` is therefore seven minor releases behind (inference from [1]; 1.92 itself is not on the page's list, whose earliest row is 1.93.0 and is dated about December 2025, unsourced). Edition 2024 is already in use.

**MSRV conventions.** The Cargo book defines `rust-version` as the oldest toolchain supported, which means all functionality under every feature is verified on it in CI, and says to choose and document a policy (examples: latest with a two-release window; every even release; the current calendar year with a one-year grace; or simply the latest stable). It says raising `rust-version` is treated as a minor-level incompatibility, `cargo add` honours it, and clippy's `incompatible_msrv` lint uses it; `cargo-msrv` is the named finder [2]. The page only touches the MSRV-aware resolver (`resolver.incompatible-rust-versions = "fallback"`); details of it were not read (see section 7).

**API-compatibility checkers.** `cargo-semver-checks` runs "many dozens" of lints, has a GitHub Action (`obi1kenobi/cargo-semver-checks-action@v2`) and uses exit code 100 for denied lints [3]; linlog already uses it with `cargo-public-api` through the `breaks` command (`CLAUDE.md`). Its release count and latest version were not on the page.

**RustSec and cargo-deny.** linlog's own `deny.toml` shows the practice in use: advisories are checked online weekly and a ignore needs a reason. No further source was fetched on cargo-deny itself (section 7).

**GitHub Actions pinning.** Dependabot can raise pull requests for outdated actions with `package-ecosystem: "github-actions"` and a schedule; the page does not say how SHA-pinned actions are handled or whether a cooldown exists [4]. linlog's alternative is the manual `update-deps` skill.

**Provenance and signatures.** `actions/attest` is what linlog runs. Sigstore's cosign keyless signing binds an ephemeral key to an OIDC identity (Fulcio), logs the event in Rekor, and verification checks signature, certificate and log entry against the identity; trust roots are distributed with TUF [5]. It signs bytes only: it does not satisfy macOS Gatekeeper or Windows SmartScreen (inference: those consult Apple's and Microsoft's own trust chains; not shown by [5]).

**macOS notarisation.** Apple's membership comparison lists Mac software notarisation and Developer ID certificates under the Developer Program, whose price the page did not give [6]. The yearly fee of US$99 is from general knowledge, unsourced here. Notarisation therefore costs money and an account (inference).

**Windows signing.** The SignPath Foundation page for free open-source signing returned a 404 (section 7); no sourced figure for Authenticode costs was obtained. The winget manifest page shows that signing is not a required field of a winget manifest: the schema asks for installer type (including `zip`-style portable use is not stated), URL and SHA-256, and says every installer must support silent install [7].

**Install channels, as read.**
- Homebrew: a tap is a GitHub repository named `homebrew-NAME` with `Formula/`; users run `brew install user/repo/formula`; non-official taps need `brew trust` for the formula or tap; the page describes bottle automation but no version-bump automation [8].
- winget: manifests (YAML, singleton or multi-file, manifest version 1.12.0) go to `microsoft/winget-pkgs` by pull request, easiest through `wingetcreate`, which can submit it; one pull request per version; the package identifier must be unique [7].
- Scoop: a bucket is a Git repository of JSON manifests, with a template that adds Actions updating manifests every four hours; the rules for the Main and Extras buckets are on a page not read [9].
- Chocolatey: community packages are moderated (automated validator, verifier and cleaner, plus human review unless the package is trusted); downloads must come from the project's official location; a stalled package is rejected after about 35 days [10].
- AUR and COPR: the pages could not be read (the Arch wiki answered with a bot-check; the COPR page was a 404). Nothing is claimed.

**WebAssembly.** The target's defaults, per the rustc book: `multivalue`, `mutable-globals`, `reference-types`, `sign-ext`, and, from Rust 1.87 with LLVM 20, `nontrapping-fptoint` and `bulk-memory`; `wasm32v1-none` is the MVP-level target; `-Cpanic=abort` remains the default and unwinding needs nightly `-Zbuild-std` with no plan to change the default [11]. Bulk-memory support is 97.15 percent worldwide, from Chrome 75, Firefox 79, Safari 15 [12]; the other features' support was not obtainable (the webassembly.org table loads by script, [13]). wasm-bindgen releases irregularly, about every three weeks (0.2.130 on 30 Oct 2026, 0.2.129 on 25 Sep, 0.2.128 on 5 Sep); 0.2.128 bumped the schema ("Requires matching `wasm-bindgen` and CLI versions") and 0.2.129 made the mismatch error name the file and binary [14]. linlog pins 0.2.129 while 0.2.130 is out. The component model and wasm GC were not read (section 7).

**Documentation tooling.** mdBook's latest release is v0.5.4 (6 July, year not on the page) in a 0.5 series [15]; the version linlog's lock uses was not read from the repository here. rustdoc's `doc_cfg` is still unstable: the tracking issue is open with a stabilisation pull request (#150055) open, `doc_auto_cfg` was folded into it on 2026-06-07 and the last event is dated 2026-11-04 [16]. A feature-gated item cannot therefore show its gate on stable docs.

**Accessibility sources.** WCAG 2.2 is a W3C Recommendation of 12 December 2024 that adds nine criteria (2.4.11, 2.4.12, 2.4.13, 2.5.7, 2.5.8, 3.2.6, 3.3.7, 3.3.8, 3.3.9) and drops 4.1.1 Parsing [17]. MDN advises inline SVG to use `role="img"` with `<title>` first and `<desc>`, tied by `aria-labelledby` [18]; MathML is Baseline "widely available" since January 2023 [19], but the page says nothing on its accessibility. The European Accessibility Act page read does not give application dates or exempt microenterprises, and does not cover public-sector sites [20]; no source on whether a university host triggers a duty was found.

## 3. Candidates

Effort is in sessions of the plan's kind.

### maintenance-1: A written dependency and toolchain cadence, with an MSRV policy

- *Brings, to whom:* the maintainer and any packager. Today the MSRV (1.92) is an unexplained number, seven minors behind stable [1], and updates happen when someone asks. The Cargo book asks for a chosen, documented policy [2].
- *In linlog's terms:* a section in `docs/src/` and `.claude/rules/flake.md` stating the policy (for example "stable minus four releases, raised only when a dependency or a language feature earns it", an inference from [2]'s examples), the `msrv` check unchanged, and a monthly `update-deps` run recorded in the plan. Optionally `resolver.incompatible-rust-versions = "fallback"` once its behaviour is read (not verified here).
- *Cost:* 0.5 session to write; each cadence run about 0.5 to 1 session (existing skill).
- *Risk:* a low MSRV blocks dependency updates (krilla and usvg already hold back, `deny.toml`); a high one excludes Debian and Fedora toolchains, which linlog avoids by shipping static binaries.
- *Prerequisites:* none.
- *Sources:* [1], [2].

### maintenance-2: Scheduled notifications of outdated actions and advisories

- *Brings:* the weekly advisory job exists; actions are not watched between `update-deps` runs. A `dependabot.yml` for `github-actions` (and `cargo` limited to advisories, if wanted) raises pull requests [4].
- *In linlog's terms:* a config file only; each pull request must still pass `nix flake check`, so it feeds the `update-deps` skill rather than replacing it. Whether Dependabot rewrites SHA pins with their version comments was not read [4].
- *Cost:* 0.25 session. Adds no code to the trusted base but adds a bot with write access to pull requests.
- *Risk:* noise; the jj-only workflow does not merge GitHub pull requests by default (`CLAUDE.md` allows them), so a PR must be turned into a local change.
- *Prerequisites:* the author's yes (repository setting, a `gh` administrative act).
- *Sources:* [4].

### maintenance-3: A WebAssembly feature baseline and a bindgen policy

- *Brings:* the browser tab's reach. With Rust above 1.87 the module already uses bulk memory and non-trapping float-to-int [11]; bulk memory reaches 97.15 percent of users [12]. Recording this as the stated baseline, plus a test that parses the built module's features section (a check beside `web-package`), would stop a toolchain bump from silently adding SIMD or other features. For wasm-bindgen: the CLI must equal the crate [14]; `modules/web.nix` already enforces it with a clear error, so the policy is only "bump crate and CLI together when nixpkgs has the CLI, at most monthly".
- *In linlog's terms:* `web.md` rules entry, a check using `wasm-tools`/`wasm-objdump` (a new devshell tool, `new-tool` skill), a documented fallback of `-Ctarget-cpu=mvp` with `-Zbuild-std` if a user needs it [11].
- *Cost:* 1 session. Trusted base: none (the module is untrusted by design; the checker is a Rust library).
- *Risk:* the feature-support table of other features was not read, so the baseline's reach beyond bulk memory is unquantified.
- *Prerequisites:* none.
- *Sources:* [11], [12], [13], [14].

### maintenance-4: An accessibility audit of the page and the SVGs with tools, and a screen-reader pass

- *Brings, to whom:* learners and teachers; D33 sets AA, and the sweep was a judgement review, not a tool run (`plan/reports/38-web-polish.md`). An automated run (axe through Playwright, already in the client's stack, or Lighthouse) over each tab state, plus a manual pass with a screen reader on the exported SVG (title, description, reading order as `core` writes it) closes the gap between the stated floor and a measured one. The page must also be checked against the criteria 2.2 added [17] (2.4.11, 2.5.7, 2.5.8 are already in D33).
- *In linlog's terms:* Playwright tests in the client that fail on axe violations; the SVG exporter's output checked against MDN's pattern (`role="img"`, `<title>` first, `aria-labelledby` [18]); a short accessibility statement in the manual.
- *Cost:* 1.5 sessions (a dependency in the client, `@axe-core/playwright`, whose current version and licence were not read).
- *Risk:* tools find about a part of the issues (inference; unsourced figure not given); the interactive stage's drag-free design already avoids the worst.
- *Prerequisites:* none. A MathML rendering of formulas is rejected below.
- *Sources:* [17], [18].

### maintenance-5: Keyless signatures with cosign beside the existing attestation

- *Brings:* an identity-bound signature file for each archive that users can verify without the `gh` CLI, logged in Rekor [5]. Reaches users who verify with package tooling rather than GitHub's.
- *In linlog's terms:* one step in `release.yml` after `SHA256SUMS`; the manual's installation chapter gets a `cosign verify-blob` line. It does not remove the macOS and Windows warnings (inference from [5], [6]).
- *Cost:* 0.5 session. New trusted action pinned by SHA.
- *Risk:* two verification paths to document; the `actions/attest` attestation already gives provenance, so the added value is small (inference).
- *Prerequisites:* none.
- *Sources:* [5].

### maintenance-6: Platform code signing (macOS notarisation, Windows Authenticode)

- *Brings:* removes the quarantine and SmartScreen warnings documented in `docs/src/installation.md`; matters for teachers installing on students' machines.
- *In linlog's terms:* needs the author's accounts: Apple's Developer Program for Developer ID and notarisation [6], and a Windows certificate (cost and free options for open source not sourced, the SignPath page was a 404). Signing steps in `packages.yml`; secrets in the repository.
- *Cost:* 1.5 to 2 sessions plus the author's yearly fees and identity checks.
- *Risk:* secrets in CI; certificate expiry; a `.msi` also needs the WiX EULA (`plan/handoff.md`).
- *Prerequisites:* the author's decision and accounts. Not recommended before demand appears: the web page covers the no-install user.
- *Sources:* [6], `docs/src/installation.md`.

### maintenance-7: Install channels, ranked

Ranked by users reached against cost (the ranking is an inference, since no download figures were read):

1. **winget manifest** (Windows): a pull request per release to `microsoft/winget-pkgs`, no account beyond GitHub, `wingetcreate` can generate and submit it, installer URL plus SHA-256 from the existing release [7]; whether a plain `.zip` of a console tool is accepted as `portable` was not on the page read. Cost 0.5 session plus 0.1 per release unless automated in `release.yml`.
2. **Homebrew tap** (macOS, Linux): own repository `linlog-prover/homebrew-tap` with a formula pointing to the release archives and their SHA-256 [8]; the tap needs `brew trust` for users. Cost 0.5 session. Creating the repository is a `gh` administrative act that needs the author's yes.
3. **Scoop bucket**: a repository of one JSON manifest with auto-update [9]. Cost 0.25 session.
4. **Chocolatey**: human moderation and a verifier that installs silently [10]. Cost 0.5 session and slow reviews; fewer users than winget (inference).
5. **AUR and COPR**: not readable today; the Arch `.pkg.tar.zst` and the `.rpm` already installable directly. Defer.
- *Risk:* each channel needs updating per release; `install.yml` already tests the chapter's commands and would have to cover them.
- *Prerequisites:* the author's accounts, per `plan/handoff.md`.
- *Sources:* [7], [8], [9], [10].

### maintenance-8: A glibc prebuilt binary (45d's follow-up)

- *Brings:* a dynamically linked build for users who need glibc behaviour (musl's allocator is slower, and musl's other limits are in the installation chapter). Whether the speed difference is measurable for linlog is not sourced; the benchmarks in `bench/` could settle it.
- *In linlog's terms:* a second target in `modules/dist.nix` (`x86_64-unknown-linux-gnu`) with a stated glibc floor; the packages could depend on it.
- *Cost:* 1 session. Risk: glibc version floor, support burden.
- *Prerequisites:* a measurement showing the gain (a timed run, coordinated with the machine lock).
- *Sources:* `docs/src/installation.md`, `plan/later.md`.

### maintenance-9: Keep the heavier future cheap: a crate and wasm module budget

- *Brings:* if the multi-logic future adds crates or modules, each adds lock entries (171 now), advisory exposure and a wasm size. A rule that every new crate or wasm module states its dependencies, size delta and CI cost (the `new-tool` skill already covers dependencies) keeps update runs bounded.
- *In linlog's terms:* a size ceiling for the `linlog-web` module in the ratchet or a check (the web design's first-load budget, D35, is the stated target, about 170 KB compressed [`plan/web/design.md`]); one `wasm-bindgen` module shared among logics rather than one each, if the glue allows (inference).
- *Cost:* 0.5 session. Risk: low.
- *Prerequisites:* none.
- *Sources:* `plan/web/design.md` D35.

## 4. Considered and rejected

- **Raising the MSRV to the latest stable now.** It would bring nothing the code needs; the policy in maintenance-1 decides.
- **Replacing `update-deps` by Renovate or Dependabot.** The skill verifies with `nix flake check` and keeps one change per kind; a bot cannot (inference). Maintenance-2 keeps it and adds notifications.
- **MathML for the formulas.** MathML is widely available since 2023 [19], but the page says nothing on screen-reader behaviour, and the formulas are drawn in SVG with text alternatives; changing it would redo D33's approach for no sourced gain.
- **Enabling `-Cpanic=unwind` or wasm exceptions in the module.** Needs nightly `-Zbuild-std` and has no plan to become default [11].
- **Using `doc_cfg` now.** Unstable with its stabilisation pull request open [16]; revisit when it lands.
- **nixpkgs submission.** Skipped at the author's word (`plan/handoff.md`).
- **AUR and COPR now.** Sources unreadable today; the Arch and RPM packages exist.

## 5. What the structure must allow

Nothing in `core/` is touched by this area, except the SVG exporter's alternative texts (maintenance-4). A multi-logic future affects maintenance in three places: a generic kernel crate would add a workspace member (update, MSRV, `cargo shear`, `cargo hack` and mutation runs scale with members); wasm modules should share one bindings crate and one CLI-matching rule (`modules/web.nix`); and the checks named in `CLAUDE.md` (`wasm`, `web-core-wasm32`, `msrv`, `features`) should accept a new crate by a list, not by a new module file each (judged from `CLAUDE.md`; the module rule files `.claude/rules/flake.md` and `core.md` were not re-read for this).

## 6. Sources

1. https://blog.rust-lang.org/releases/ (2026-11-08)
2. https://doc.rust-lang.org/cargo/reference/rust-version.html (2026-11-08)
3. https://github.com/obi1kenobi/cargo-semver-checks (2026-11-08; no activity date shown)
4. https://docs.github.com/en/code-security/dependabot/working-with-dependabot/keeping-your-actions-up-to-date-with-dependabot (2026-11-08)
5. https://docs.sigstore.dev/cosign/signing/overview/ (2026-11-08)
6. https://developer.apple.com/support/compare-memberships/ (2026-11-08)
7. https://learn.microsoft.com/en-us/windows/package-manager/package/manifest (updated 2026-09-15; read 2026-11-08)
8. https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap (2026-11-08)
9. https://github.com/ScoopInstaller/Scoop/wiki/Buckets (2026-11-08)
10. https://docs.chocolatey.org/en-us/community-repository/moderation/ (2026-11-08)
11. https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html (2026-11-08)
12. https://caniuse.com/wasm-bulk-memory (2026-11-08)
13. https://webassembly.org/features/ (2026-11-08; table not rendered)
14. https://github.com/wasm-bindgen/wasm-bindgen/releases (2026-11-08)
15. https://github.com/rust-lang/mdBook/releases (2026-11-08)
16. https://github.com/rust-lang/rust/issues/43781 (2026-11-08; last event 2026-11-04)
17. https://www.w3.org/TR/WCAG22/ (2026-11-08)
18. https://developer.mozilla.org/en-US/docs/Web/SVG/Guides/SVG_in_HTML (2026-11-08)
19. https://developer.mozilla.org/en-US/docs/Web/MathML (2026-11-08)
20. https://commission.europa.eu/strategy-and-policy/policies/justice-and-fundamental-rights/disability/union-equality-strategy-rights-persons-disabilities-2021-2030/european-accessibility-act_en (2026-11-08)

## 7. Not verified

- AUR submission rules (the Arch wiki blocked the fetch), COPR rules (404), SignPath Foundation's free signing and any Windows Authenticode cost (404), Apple's yearly fee (the page gave none).
- The EU Accessibility Act's dates and a university host's duties; the Web Accessibility Directive; WCAG 2.2 as an ISO standard.
- Browser support of reference types, multi-value, SIMD, exception handling and tail calls (the table did not load); wasm GC and the component model; the 1.82 ABI change of the target (not fetched).
- The MSRV-aware resolver's details; `cargo-msrv`, `cargo-public-api`, cargo-deny and RustSec pages (not fetched); cargo-semver-checks' latest version.
- How Dependabot treats SHA pins; whether a winget `zip`/portable entry is accepted; nixpkgs' state of `wasm-opt`/binaryen and its wasm-bindgen-cli versions (no page read).
- Typst, ebproof and curryst versions the exports target; mdBook's version in the lock; the client's flake inputs.
- Which flake input is dated 2026-04-13; the exact cost of every candidate is an estimate.
- Nothing needed a run; I would have run nothing.
