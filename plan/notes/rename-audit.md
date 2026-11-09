# Rename audit: linlog → sequentium → deduxit (2026-11-09)

Read only. The trees were read at their pushed `main`:

- deduxit: `~/Projects/own/linlog`, `main` = `uktloxnv` `49c59774`. `main` moved from `ac9b8f06` during the audit; only the Status log and the 53 report changed.
- rocq-deduxit: `~/Projects/own/rocq-linlog-53s`, at `main` `52ff2f88`. The default workspace's `@` sits on an older commit, so it was not read.
- The client: `~/Projects/own/linlog-prover.github.io-53s`, at `main` `cfe62686`. Its default workspace is behind as well.
- The placeholder: `~/Projects/own/linlog-prover-placeholder`, at `main` `ec21c4b7`.

**Classes:**
- **B**: fix before the release.
- **R**: fix at the release. It changes only when 53b bumps the version or publishes.
- **S**: suggestion.

**Places where an old name is correct and not reported:**
- Everything listed as kept in the brief.
- `core/tests/wire/*` and `cli/tests/lock*`: the old `"linlog"` key as 0.1.0 and 0.2.0 wrote it.
- The serde aliases.
- `docs/src/installation.md:36`: attestations of 0.2.0 and earlier under `linlog-prover/linlog`, which is correct.
- `install.yml`'s "files named linlog" up to 0.2.0.
- Dated result pages and `bench/RESULTS.md` (the unit `linlog-baseline` as it ran).
- `bench/plots.py`'s `RENAMED`.
- Local operational names: zellij sessions `linlog-step-…`, `/tmp/linlog-machine.lock`, `../linlog-wt-NAME`. They are consistent with `plan/start-session.sh`.
- The placeholder's `site/logo/` lollipop files. These are deliberate: docs.rs of 0.1.0 and 0.2.0 embeds them.
- The word "lollipop" as the name of the connective `⊸` (`Binary::Lollipop` and similar), and the prover "lolli".
- The manual's violet UI accent (`docs/theme/deduxit.css`). It is the page's accent colour, not the mark's colour, as the client's design also keeps it.

---

## 1. deduxit/deduxit (local `linlog`)

The code, rustdoc, CLI help, `skills/deduxit/`, `CITATION.cff`, the `Cargo.toml` fields, the workflows, the flake, `.claude/rules` (except B2), CLAUDE.md, README and the manual are clean of "linlog" as the product's name, of "sequentium", of the lollipop and of the Rocq path `Linlog`. What remains is below.

**B1. The rustdoc renders the dictionary line with curly quotes**
- Place: `core/src/lib.rs:17`, as rendered at https://deduxit.github.io/deduxit/deduxit/
- Found: the source has `"it has derived"`. rustdoc's smart punctuation renders it as `“it has derived”` (U+201C/U+201D), so this copy is not word for word. The manual (`smart-punctuation = false`) and the READMEs keep straight quotes. docs.rs will render the same as the site.
- Fix: escape the quotes as the brackets are, `\"it has derived\"` (or `&quot;`), rebuild the docs and check the bytes. Add the point to `.claude/rules/core.md:155-156`.

**B2. A rules file says deduxit has no mark of its own**
- Place: `.claude/rules/core.md:148-149`
- Found: "deduxit has no mark of its own since the lollipop retired (2026-11-09)." This contradicts the paragraph it ends, which says the rustdoc shows deduxit's mark.
- Fix: "The lollipop, linlog's mark up to 0.2.0, retired on 2026-11-09; the linear suite has no mark of its own."

**B3. conduct.md points every session at the old guide path**
- Place: `plan/conduct.md:142`
- Found: "The guide for AI models, `skills/linlog/` …". conduct.md is appended to every step prompt.
- Fix: `skills/deduxit/`.

**B4. The handoff still describes the lollipop as the logo**
- Place: `plan/handoff.md:486-488`
- Found: "The logo is the lollipop in violet, CC BY 4.0, with its `--color-logo` token held by the colour test and a rules line."
- Fix: "The mark is dēdūxit in Gentium ExtraBold, its long marks lapis (#2456c9 light, #7aa7ff dark) as `--color-logo`, CC BY 4.0, held to the `.logo` rule by the colour test."

**B5. The handoff names the product and its paths linlog**
- Places and text:
  - `plan/handoff.md:3`: "the planning session of the linlog repository"
  - `:484`: "decides every one by linlog's command"
  - `:489`: "`skills/linlog/`"
  - `:550`: "`linlog check` ignores …; linlog's recursion limit"
- Fix: deduxit's repository (local checkout `linlog`), deduxit's command, `skills/deduxit/`, `deduxit check` / deduxit's recursion limit. Also drop the `check`/"mode" item if 53a closed it.

**B6. The handoff's Rocq library and opam notes predate rocq-deduxit**
- `plan/handoff.md:172`: "the repositories `deduxit/deduxit`, `deduxit/rocq-linlog`, …". Add "(the library renamed `rocq-deduxit` the same day, 53r)".
- `:180-187`: the opam pull request "from 45e's commit moved to the new addresses by 48a", pushing `"rocq-linlog.0.2.0=REV"`. The prepared change in `~/Projects/own/rocq-opam` is now `mmsqlyzk` "Add rocq-deduxit.0.3.0". Its checksum is a placeholder, so the pull request follows the v0.3.0 tag. Rewrite the steps for `rocq-deduxit.0.3.0`.
- `:189`: "a module of `rocq-linlog`". Change to `rocq-deduxit`.
- `:206`: "rocq-linlog's `CACHIX_AUTH_TOKEN`". Change to rocq-deduxit's.

**B7. Step 53's prompt has an old address and a question already decided**
- Place: `plan/53-release-0.3.0.md:4`
  - Found: "(`../rocq-linlog`, `deduxit/rocq-linlog`)"
  - Fix: `deduxit/rocq-deduxit`.
- Place: `plan/53-release-0.3.0.md:316-328` (53b, the next session to run)
  - Found: "linlog's concept DOI 10.5281/zenodo.23448588 … a question for the author … whether linlog can be renamed … `title: linlog` after step 48 … either the concept DOI stays "linlog" … or the title changes … "deduxit: logic suites (linlog, …)"".
  - Why it is stale: the prompt's own line 47 already decides "From 0.3.0 the title is the new name, "deduxit" and "rocq-deduxit"", and `CITATION.cff` says `title: deduxit`.
  - Fix: replace the question with the check: "the 0.3.0 records are titled deduxit and rocq-deduxit from `CITATION.cff`; confirm that each lands under its concept DOI".

**B8. Step prompts 54 to 73 name the product, the crate and the library by their old names**
- Every one opens "You are working in linlog (`deduxit/deduxit`)" or "the linlog repository": 54–73, line 3. Fix: "in deduxit (`deduxit/deduxit`, the checkout `../linlog`)".
- `deduxit/rocq-linlog`: `55:4`, `59:4`, `64:4`. Fix: `deduxit/rocq-deduxit`.
- The check and module `rocq-linlog`: `55:43`, `66:31`, `66:35`. Fix: `rocq-deduxit`.
- `60:54`: "a module of the `linlog` crate". Fix: `deduxit`.
- `67:20, 34, 37, 40`: "every published path of `linlog`", "stay in `linlog`", "`linlog` re-exports", "beyond `linlog`". Fix: deduxit. Also note that 0.2.0's paths are now kept by `compat/linlog` and that the linear suite is `deduxit::linear`.
- `61:20`: `linlog interact`. Fix: `deduxit interact`.
- `72:12`: `skills/linlog/`. Fix: `skills/deduxit/`.
- "linlog's own" and "linlog has/verified": `62:34`, `70:30`, `71:23`, `72:18`. Fix: deduxit.
- `58:39`: `core/src/proofs/check/` is a path from before the move. Fix: `core/src/linear/proofs/check/`.

**B9. plan/README.md above the Status log**
- `:3`: "The steps that take linlog from …". Fix: deduxit.
- `:92` (step 67's row): "`linlog`'s paths kept (D25)". Fix: deduxit's.
- D24, `:1014-1016`: "linlog grows into a collection …; `linlog` stays the linear suite, its name, crates and command unchanged". The rename contradicts it. Fix: add "(superseded 2026-11-09: the crates and the command are deduxit, the linear suite is the module `deduxit::linear`)".
- D25, `:1028`: "the proof term stay in `linlog`". Fix: deduxit.
- D26, `:1048`: "`rocq-linlog` keeping its name". This is false since 53r. Fix: "`rocq-linlog` renamed `rocq-deduxit` the same day (53r)".
- Older decisions name the crate as it was: D4 `:725`, D13 `:837`/`:843`, D14 `:864`/`:871-872`, D20 `:931-942`, D22 `:955-983`. D14's `cargo hack … -p linlog` no longer runs.
  - Fix: one dated note at the head of "Design decisions" ("names as of their day: linlog is deduxit, rocq-linlog is rocq-deduxit, linlog-web is deduxit-web"), and D14's commands to `-p deduxit`.

**B10. later.md still points at the lollipop and an old address**
- `plan/later.md:2429-2432` (Zenodo community):
  - Found: `zenodo.org/communities/linlog`, "its logo `~/Projects/own/linlog-logo/for-github/zenodo-community.png`, which 38g drew". That image is the lollipop.
  - Fix: the community `deduxit`, its logo drawn from the new square by the client's `modules/logo.nix`.
- `:2536-2537`: "any Nix user gets linlog through its own flake (`nix run github:linlog-prover/linlog`)".
  - Fix: deduxit, `github:deduxit/deduxit`.
- `:2495-2515` (MCP server): "an MCP server for linlog", "`skills/linlog/`", "`linlog mcp`".
  - Fix: deduxit, `skills/deduxit/`, `deduxit mcp`.

**B11. Licence headers in three plan scripts still say linlog**
- Places: `plan/push-main.sh:2`, `plan/start-session.sh:2`, `plan/keys-warm.sh:2`
- Found: `# linlog © Fabian Lukas Grubmüller 2026`. The `conventions` check skips `plan/`, so nothing caught them.
- Fix: `# deduxit © …`.

**B12. The spec's erratum names linlog and a moved path**
- Place: `proof-search-specifications.md:54-55`
- Found: "linlog decides these programs with an engine of its own (`core/src/search/horn/`, …)". Errata override the spec, so this text is current.
- Fix: deduxit, and the path `core/src/linear/search/horn/`.

**R1. README's version paragraph**
- Place: `README.md:21-25`
- Found: "Version 0.2.0 adds … the logo, and deduxit's Rocq library …". The logo meant is the retired lollipop.
- Fix: 53b rewrites the paragraph for 0.3.0: the rename, `deduxit::linear` and the new mark.

**R2. Versions and dates that move at the release**
- Places: `Cargo.toml:14` (`version = "0.2.0"`, and the `deduxit`/`deduxit-cli` dependency versions), `CITATION.cff` (`version: 0.2.0`, `date-released: "2026-11-02"`).
- Fix: 0.3.0 and the release date (53b).

**R3. Publishing the crates under the new names**
- Place: `.github/workflows/release.yml:84, 95`
- Found: it publishes `deduxit` and `deduxit-cli` only. The final `linlog` 0.3.0 and `linlog-cli` 0.3.0 are left to 53b, which is known.
- Also check before the tag: `deduxit` and `deduxit-cli` do not exist on crates.io yet (the API answers "does not exist"). As far as I know, crates.io configures a trusted publisher only for a crate that already exists, so the first upload may need a token. `linlog`'s trusted publisher was set up when the repository was `linlog-prover/linlog`.
- Fix: settle the first publication of the new crates and the trusted-publisher entries for `deduxit/deduxit` before tagging.

**R4. The changelog's link for 0.3.0**
- Place: `CHANGELOG.md:1118-1119`
- Found: the only version links are 0.2.0's and 0.1.0's, on `linlog-prover/linlog`. These are kept.
- Fix: at the release, add `[0.3.0]: https://github.com/deduxit/deduxit/releases/tag/v0.3.0`.

**S1. The web front end's plan uses the names of 2026-10-19**
- Places: `plan/web/README.md`, which CLAUDE.md calls "the web front end's plan", and `plan/web/design.md`.
- Found:
  - `linlog-web` (`README.md:6, 16, 23, 110`)
  - `linlog interact` (`:50`, `:85`)
  - the rustdoc at `/linlog` (`:64-65`, `:73`)
  - `` `about()` | `{"linlog", …}` `` (`:129`); the binding's key is now `"deduxit"` (`web/src/calls.rs:42`)
  - `linlog::Settings` (`:120`)
- Fix: a dated note at the head, or rename the API table.

**S2. Other open later.md entries name the old command or crate**
- Places: `plan/later.md:1276`, `:1906`, `:2046-2047`, `:2251` (`linlog::lltp::read`), `:2348-2360`, `:2411`, `:2417-2423` (`linlog-cli`, `share/doc/linlog-cli/`).
- Fix: rename where an entry is still open (`deduxit check`, `deduxit::linear::lltp::read`, `deduxit-cli`), or close the entries that 53a or the notices already did.

**S3. The changelog does not mention the new mark**
- Place: `CHANGELOG.md`, `[Unreleased]`
- Found: 0.2.0's entry (`:954`) announces "linlog has a logo, the word with its i drawn as `⊸`". Nothing says that a new mark replaces it.
- Fix: add a line: "deduxit's mark, the word dēdūxit with lapis long marks (CC BY 4.0, served at deduxit.github.io/logo/), replaces linlog's logo".

## 2. deduxit/rocq-deduxit (local `rocq-linlog`)

The theories, `_RocqProject` (`-Q theories Deduxit`), the opam file, `CITATION.cff`, the flake, the workflows, `.claude/`, CLAUDE.md and README are clean. Every remaining "linlog" or `Linlog` describes the rename. The dictionary line in `README.md:3` matches exactly.

**R5. Version, date and published documentation**
- `CITATION.cff` (`version: 0.2.0`, `date-released: "2026-11-02"`) and `rocq-deduxit.opam` (`"date:2026-11-02"` among its tags).
  - Fix: 0.3.0 and the release date. The prepared opam change in `~/Projects/own/rocq-opam` carries the same old `date:` tag and a zero checksum.
- The published documentation shows 0.2.0's `Linlog.*` modules. It is redeployed only by `release.yml` at a tag.
  - Fix: the v0.3.0 tag publishes `Deduxit.Linear.*`.

**S4. The README gives the wrong repository for 0.2.0**
- Place: `README.md:43-44`
- Found: "Up to 0.2.0 the library was `rocq-linlog`, with the logical path `Linlog`, in the repository `deduxit/rocq-linlog`". 0.2.0 was released from `linlog-prover/rocq-linlog`; `deduxit/rocq-linlog` existed for a few hours on 2026-11-09.
- Fix: name `linlog-prover/rocq-linlog`. GitHub redirects both addresses.

## 3. deduxit/deduxit.github.io (the client, local `linlog-prover.github.io`)

The source is clean:
- Titles, `og:` metadata, the manifest (`name`/`short_name` "deduxit"), `src/content/llms.txt`, About and Learn.
- `src/logo/*`: Gentium with lapis #2456c9/#7aa7ff, titles "deduxit", `LICENSE.txt` CC BY 4.0.
- `package.json`, the flake description and the workflows.

Every "linlog" left is a kept browser key, the cache prefix `linlog-site-`, the Cachix cache, a local directory, or the forward page for `/rocq-linlog/`. The dictionary line in `README.md:3` and in `Logo.tsx` (`Dictionary`, shown in the band's card and in About) renders word for word: the tests in `page.spec.ts:81` and `learn.spec.ts:98` hold it.

**B13. The rules copy of the dictionary line drops its final period**
- Place: `.claude/rules/web.md:627-629`
- Found: "… to lead down, derive (approved by the author, 2026-11-09; keep it word for word)". The line it tells sessions to keep word for word lacks its final ".".
- Fix: "… to lead down, derive. (Approved by the author, …)", or quote the line as a block.

## 4. linlog-prover/linlog-prover.github.io (the placeholder)

**B14. The README says linlog keeps its name**
- Place: `README.md:7-8`
- Found: "linlog is its linear suite and keeps its name, its crates and its command." This is false since 53s.
- Fix: "linlog was renamed deduxit at 0.3.0: the crates `deduxit` and `deduxit-cli`, the command `deduxit`, the linear suite the module `deduxit::linear`."

**B15. The 404 page links and forwards to a rustdoc path that does not exist**
- Place: `site/404.html:40`, and its `MOVED` map at `:28-31`
- Found:
  - The link for "the library's API" is `https://deduxit.github.io/deduxit/linlog/`, which is 404 live.
  - The map sends 0.2.0's rustdoc paths `/linlog/linlog/…` to `/deduxit/linlog/…`, which are also 404.
- Fix: link `https://deduxit.github.io/deduxit/deduxit/`. Add `["/linlog/linlog", "/deduxit/deduxit"]` before `["/linlog", "/deduxit"]`; the items moved under `linear::`, so sending deep links to the crate root is the safe choice.

**S5. The frozen guide copies keep a dead link and an old address**
- Places:
  - `site/llms.txt:13`, `site/linlog/llms.txt:89`, `site/linlog/reference.md:826`, `site/linlog/llms-full.txt:826`: the same dead `/deduxit/linlog/`.
  - `site/linlog/llms.txt:94`, `reference.md:909`, `llms-full.txt:909`: `github.com/deduxit/rocq-linlog`.
- Fix: these copies are "not updated" by design, but their links can point at `/deduxit/deduxit/` and `deduxit/rocq-deduxit`. Alternatively, cut each copy down to its first line ("moved to …").

## 5. Public metadata

**B16. deduxit/deduxit's description is linlog's**
- Place: https://api.github.com/repos/deduxit/deduxit
- Found: "A linear logic suite: decides, checks, proves step by step and exports sequents of classical and intuitionistic linear logic, with proofs an independent checker validates".
- Fix: core's Cargo description, "Logic suites, linear logic first: decides, checks, proves step by step and exports sequents of classical and intuitionistic linear logic, and ordinary propositional logic through its embeddings." (A gh admin act, the author's.)

**B17. rocq-deduxit's description names linlog three times**
- Place: https://api.github.com/repos/deduxit/rocq-deduxit
- Found: "linlog's Rocq library: the calculi of linear logic, linlog's proof terms with checkers proved sound, and the certificates linlog writes".
- Fix: "The Rocq library of deduxit, a collection of logic suites: the calculi of linear logic under Deduxit.Linear, deduxit's proof terms with checkers proved sound, and the certificates deduxit writes".

**B18. The client repository's description says linlog**
- Place: https://api.github.com/repos/deduxit/deduxit.github.io
- Found: "linlog in the browser: prove sequents of linear logic step by step; …".
- Fix: "deduxit in the browser: prove sequents of linear and ordinary logic step by step; the prover runs in the page, nothing is sent to a server".

**B19. The placeholder repository still points at sequentium**
- Place: https://api.github.com/repos/linlog-prover/linlog-prover.github.io
- Found: description "linlog has moved to sequentium.github.io: this site redirects the old addresses", homepage `https://sequentium.github.io/`, which is 404.
- Fix: "linlog has moved to deduxit.github.io: this site redirects the old addresses", homepage `https://deduxit.github.io/`.

**Checked and correct:**
- The organisation `deduxit`: name, description, blog `https://deduxit.github.io`; its avatar is the new square "dē".
- The social previews of all three repositories are the new word mark.
- The topics carry no old name.
- The old addresses all redirect: `linlog-prover/linlog`, `linlog-prover/rocq-linlog`, `deduxit/rocq-linlog` and `sequentium/sequentium` answer 301 to the new repositories.

**R6. crates.io still shows linlog 0.2.0's metadata**
- Places: https://crates.io/api/v1/crates/linlog and `…/linlog-cli`
- Found:
  - `linlog` 0.2.0: "A linear logic suite: …", repository `https://github.com/linlog-prover/linlog`.
  - `linlog-cli` 0.2.0: documentation `https://linlog-prover.github.io/linlog/manual/`.
  - `deduxit` and `deduxit-cli` do not exist yet.
- Fix: this changes with the publication of `linlog`/`linlog-cli` 0.3.0 (compat: "Renamed deduxit…", `deduxit/deduxit`) and of the new crates.

**R7. Zenodo's next records**
- Places: https://zenodo.org/api/records?q=conceptrecid:23448588&all_versions=true and `…conceptrecid:23899109`
- Found: v0.1.0 and v0.2.0 are titled "linlog" (described as "A linear logic suite…"), and v0.2.0 of the library "rocq-linlog". These are kept as published.
- Fix: at the release, the new versions take "deduxit" and "rocq-deduxit" from `CITATION.cff`. Confirm that Zenodo's GitHub hook still fires for the renamed repositories and that each record joins its concept (53b's check).

**S6. The Zenodo records' links are inconsistent**
- Found:
  - rocq-linlog v0.2.0's related identifier is `https://github.com/deduxit/rocq-linlog/tree/v0.2.0`, an old address that redirects.
  - linlog v0.1.0's is `https://github.com/linlog-prover/linlog/tree/v0.1.0`, while v0.2.0's was edited to `deduxit/deduxit`.
- Fix: edit them to `deduxit/rocq-deduxit/tree/v0.2.0` and `deduxit/deduxit/tree/v0.1.0`, or leave all three as published.

**S7. The kept organisation still shows the lollipop**
- Found: the organisation `linlog-prover` has the lollipop as its avatar and no description.
- Fix: optional. Use the new square, or a description such as "Moved to github.com/deduxit".

## 6. Live sites

**B20 (not yet deployed). https://deduxit.github.io/ is a build from before the rename**
- `last-modified: Sun, 08 Nov 2026 16:03:36 GMT`. This is the sequentium-era build:
  - `<title>linlog</title>`
  - `manifest.webmanifest` `"name": "linlog"`
  - `/llms.txt` "# linlog in the browser"
  - 10 "sequentium" mentions in the page
- The client's CI was still running at the audit: `cfe62686` pending and `ebf3abf6` in progress. The four runs before failed or were cancelled.
- Not a repository defect: the source is correct (section 3). Fix: deploy.

**B21 (not yet deployed, the most visible). The site's mark files are still the lollipop**
- Places: https://deduxit.github.io/logo/wordmark-light.svg, and `wordmark-dark.svg`, `square-*.svg`, `favicon-32.png`, `og.png`, `LICENSE.txt`.
- Found:
  - The wordmark is the lollipop "linlog" wordmark (`aria-label="linlog"`, `<title>linlog</title>`, Figtree).
  - `LICENSE.txt` reads "linlog's logo … identify the linlog project (https://github.com/sequentium/sequentium)".
- Effect: the heads of deduxit's and rocq-deduxit's READMEs, the rustdoc's logo and favicon, and the manual's menu-bar mark and favicon all embed these URLs. All of them show the lollipop until the client deploys.
- Fix: deploy.

**B22 (not yet deployed). The Rocq library's old address answers 404**
- Place: https://deduxit.github.io/rocq-linlog/
- Found: 404. The client's forward page `src/pages/rocq-linlog/` is not live yet.
- Fix: deploy.

**Current and correct: the rustdoc and manual site**
- https://deduxit.github.io/deduxit/ (`/manual/`, `/deduxit/`, `llms.txt`, `llms-full.txt`, `reference.md`): `last-modified: Mon, 09 Nov 2026 08:22:29 GMT`.
- The only "linlog" is "Up to 0.2.0 it was called linlog."
- The manual's dictionary line matches exactly. The rustdoc's has curly quotes (B1).
- https://deduxit.github.io/deduxit/linlog/ is 404, which is the target of B15.

**R5 (live). The Rocq documentation is still 0.2.0's**
- Place: https://deduxit.github.io/rocq-deduxit/
- Found: `last-modified: Mon, 02 Nov 2026 22:53:01 GMT`. It is 0.2.0's coqdoc, with every module `Linlog.*`. It is redeployed at the v0.3.0 tag.

**Current and correct: the old organisation site**
- https://linlog-prover.github.io/: `last-modified: Mon, 09 Nov 2026 06:27:09 GMT`, matching the placeholder's `main`.
- The redirects work; `/linlog/manual/` gives 404.html, which redirects by script.
- Except: the API link in B15.
- https://sequentium.github.io/ is 404, as expected. Only B19's homepage still points there.

## 7. The dictionary line

**Exact copies:**
- READMEs: deduxit `README.md:3`, rocq-deduxit `README.md:3`, the client's `README.md:3`.
- Manual: `docs/src/introduction.md:15`.
- Guide: `skills/deduxit/reference.md:11`.
- Rules: `.claude/rules/core.md:155-156`.
- The client: `Logo.tsx`'s `Dictionary`, and the tests' strings.

**Copies that differ:**
- `core/src/lib.rs:17`: the source is exact, with the brackets escaped. The rendered rustdoc has curly quotes (B1).
- The client's `.claude/rules/web.md:627-629`: the final period is missing (B13).

**Not checked:**
- The client's live card. The live build predates the line.

**S8. Places a reader meets the name without the line**
- The organisation `deduxit` has no profile README; `deduxit/.github` answers 404. Add one with the mark and the line.
- `skills/deduxit/SKILL.md` (served as `/deduxit/llms.txt`), if its 8 000-byte budget allows.
- The client's `src/content/llms.txt`.
- The Rocq documentation, whose coqdoc index has no front page: a front page with the mark and the line.
- `compat/linlog/README.md` and `compat/linlog-cli/README.md`, the crates.io pages of the old names from 0.3.0.
- The v0.3.0 GitHub release notes and the Zenodo descriptions.

## Areas not checked

- The client's built pages after deployment. Only the source was read; the live site is the old build. Learn's lesson texts were grepped, not read.
- Settings that only the author's GitHub account can read: Pages sources, Actions logs, crates.io trusted-publisher entries, Zenodo's GitHub hook, Cachix. No `gh` command was run.
- docs.rs: the old versions keep the lollipop on purpose, and the new crates do not exist yet.
- The design repositories, `~/Projects/own/linlog-logo` and the history under `plan/reports/`, `plan/notes/`, done prompts, and dated bench records were not audited. `~/Projects/own/rocq-opam` was only looked at for B6 and R5.
