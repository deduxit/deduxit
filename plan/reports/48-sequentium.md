# Step 48: the move to `sequentium`

The step's sessions each add a section here: 48a (everything prepared,
nothing pushed), then the author's acts, then 48b (the move finished
and checked).

## Session 48a: everything prepared, nothing pushed

### Outcome

The move to the organisation `sequentium` is prepared in four
repositories and a fifth, new one, with nothing pushed, created,
transferred or renamed:

- **linlog** (workspace `../linlog-step48a`, on `utzkqlzy`): four
  commits. `rpkkstus` names the Rocq library's new address in the help
  of `--kernel` (the doc comment in `cli/src/argument_parsing.rs`, the
  file of the command's flags, and the behaviour lock's three help
  outputs, a commit of its own as the lock asks); `xtvmuwom` moves every
  other address (35 files); `poqskvln` adds README's and the manual's
  sentence on the collection and rewrites the status chapter's planned
  list as the releases 0.3.0 to 0.7.0; `mzkukvsr`, from the review,
  says which repository name verifies the attestations of 0.2.0.
  This report is the fifth.
- **The Rocq library** (`../rocq-linlog-48a`, on `wyvsntvq`): one
  commit, `ksuqsuzm`, the addresses of the opam file, the citation,
  README, the Nix package's homepage and CLAUDE.md.
- **The web client** (`../linlog-prover.github.io-48a`, on
  `ymrkqtrq`): one commit, `ssyqpvvz`, the site's address, the flake's
  input `github:sequentium/sequentium`, and the path the shared
  repository's Pages site takes on the same origin, `/sequentium` in
  place of `/linlog`, wherever the client reserves it (the service
  worker, the build's guard, the `pages-paths` check, the test server
  and the tests), with About, `llms.txt`, the privacy notice, the 404
  page, the logo's licence and `package.json`.
- **The opam submission** (`~/Projects/own/rocq-opam`): the change
  `ywqlpnru` on top of `omkspqll`, the four fields and the archive's
  `url` at the new addresses, for the author to squash or keep.
- **The placeholder site**: a new jj repository in this session's
  scratchpad,
  `/tmp/claude-1000/-home-tux-Projects-own-linlog/bc292df7-13ed-4a06-ba88-7d25bbec8602/scratchpad/linlog-prover.github.io`,
  one commit; its table of redirects is below.

Every check passed with the moved inputs overridden to the local
workspaces: the library's `nix flake check`, linlog's (with
`--override-input rocq-linlog`), the client's (with its input `linlog`
and that input's `rocq-linlog` overridden), whose checks include its 124
end-to-end tests, and linlog's `cargo test --workspace`. The
placeholder's redirect logic and workflow were checked in the session.
The fresh-context reviewer found one reference to correct (the
attestations of 0.2.0), corrected, and two risks the author's
checklist already covers.

### The new addresses, chosen once

There is no custom domain (the author's choice of 2026-11-08), so the
addresses are GitHub's: an organisation site is served at
`https://<organisation>.github.io/` from the repository of that name,
and a repository's project site at
`https://<organisation>.github.io/<repository>/` (GitHub's "About
GitHub Pages", read for the note). Hence:

| what | new address |
|---|---|
| the shared repository | `https://github.com/sequentium/sequentium` (`github:sequentium/sequentium`) |
| the Rocq library | `https://github.com/sequentium/rocq-linlog` (`github:sequentium/rocq-linlog`) |
| the web client's repository | `https://github.com/sequentium/sequentium.github.io` |
| the web page | `https://sequentium.github.io/` |
| the rustdoc (the shared repository's Pages site) | `https://sequentium.github.io/sequentium/`, the crate at `…/sequentium/linlog/` |
| the manual | `https://sequentium.github.io/sequentium/manual/` |
| the guide for AI models | `https://sequentium.github.io/sequentium/llms.txt` and `…/llms-full.txt` |
| the Rocq library's documentation | `https://sequentium.github.io/rocq-linlog/` |
| the logos | `https://sequentium.github.io/logo/…` (the client serves them) |

The reason for the layout: the shared repository's Pages site keeps its
inner layout (the rustdoc at its root, the manual under `manual/`, the
guide beside them) and only its prefix changes from
`linlog-prover.github.io/linlog/` to `sequentium.github.io/sequentium/`,
which is where GitHub serves the renamed repository's project site
anyway. So the placeholder's mapping is one prefix per site, every path
under it stays valid, and nothing in the doc package or the Docs
workflow changes but mdBook's `site-url` and the link check's root. The
rustdoc already gives each crate a directory of its own, so a later
suite's crate gets `…/sequentium/<crate>/` beside `linlog`; the manual
stays one book for the repository.

### The table of old and new addresses

GitHub redirects a transferred repository's web and git addresses
until a repository is created at the old name, and no Pages address
(the note's sources). The placeholder site answers every Pages address;
48b fetches each row: the old address answers with the placeholder, the
placeholder sends to the new address, the new address answers.

| old | new | answered by |
|---|---|---|
| `https://github.com/linlog-prover/linlog` (web, git, releases, archives, the API) | `https://github.com/sequentium/sequentium` | GitHub's redirect |
| `https://github.com/linlog-prover/rocq-linlog` | `https://github.com/sequentium/rocq-linlog` | GitHub's redirect |
| `https://github.com/linlog-prover/linlog-prover.github.io` | `https://github.com/sequentium/sequentium.github.io` | none: the placeholder repository takes the old name; its README says where everything went |
| `github:linlog-prover/linlog` (Nix) | `github:sequentium/sequentium` | GitHub's redirect, if Nix follows it (not verified, `distribution.md`) |
| `github:linlog-prover/rocq-linlog` (Nix) | `github:sequentium/rocq-linlog` | likewise |
| `https://linlog-prover.github.io/`, `/index.html`, with `?query` and `#fragment` | `https://sequentium.github.io/` with the same query and fragment | `site/index.html`: `location.replace`, a meta refresh without scripts (which loses the query) |
| `/linlog`, `/linlog/…` (the rustdoc, `/linlog/linlog/…` the crate) | `https://sequentium.github.io/sequentium`, `/sequentium/…` | `site/404.html` |
| `/linlog/manual/…` | `https://sequentium.github.io/sequentium/manual/…` | `site/404.html` |
| `/linlog/llms.txt` | `https://sequentium.github.io/sequentium/llms.txt` | a file: the new guide, its first line naming the new address |
| `/linlog/llms-full.txt` | `https://sequentium.github.io/sequentium/llms-full.txt` | a file, likewise |
| `/linlog/reference.md` (the guide links it beside itself) | `https://sequentium.github.io/sequentium/reference.md` | a file, likewise |
| `/llms.txt` (the web page's) | `https://sequentium.github.io/llms.txt` | a file, likewise |
| `/rocq-linlog`, `/rocq-linlog/…` | `https://sequentium.github.io/rocq-linlog/…` | `site/404.html` |
| `/logo/<file>`: `square-dark.svg`, `square-light.svg`, `favicon-32.png`, `wordmark-dark.svg`, `wordmark-light.svg`, `LICENSE.txt` and the other files of the client's `/logo/` | `https://sequentium.github.io/logo/<file>` | files: copies of the client's built `/logo/`, since an image does not follow a script's redirect and every published rustdoc on docs.rs embeds them from here |
| `/sw.js` (the old page's service worker) | none | a file: a worker that deletes the old page's caches, unregisters itself and reloads its open tabs |
| `/privacy/` and every other path | the same path on `https://sequentium.github.io` | `site/404.html` |

Unchanged: the Cachix cache `linlog-prover` (its address and key are
Cachix's, not GitHub's, and no Cachix page says a cache can be
renamed), the crates `linlog`, `linlog-cli` and `linlog-web`, the
command, the opam package `rocq-linlog` and the logical path `Linlog`,
both DOIs, the local checkouts' paths, and both `flake.lock` files,
which move in 48b after the transfers.

### Every occurrence, counted again

Counted with `grep -rIo linlog-prover` outside `plan/` (and `.jj/`,
`node_modules/`, and the client's untracked `.git` pointer file of the
jj workspace) at each workspace's base and after the commits: linlog
164 before (as the note counted), 30 after; the library 25, 7 after;
the client 36, 11 after (the note's 36; a raw count says 38 because
the workspace's `.git` file names its own directory twice). Every
occurrence that remains is one of these, each kept for its reason:

| kept | where | why |
|---|---|---|
| the Cachix cache `linlog-prover` (19 in linlog, 3 in the library, 5 in the client) | every workflow's `cachix-action` `name:`, the client's `cachix push`, `CLAUDE.md`, `.claude/rules/ci.md`, `docs/src/development.md` (the substituter and its key), the library's `CLAUDE.md`, the client's `ci.yml` and `modules/linlog.nix` comments and `.claude/rules/ci.md` | the cache keeps its name (D26): its address and key are Cachix's and independent of GitHub, and no Cachix page read says a cache can be renamed |
| `flake.lock` (2 in linlog, 4 in the client) | the inputs `rocq-linlog` and `linlog` | an input's lock moves only after the transfer, by its tool (48b); until then the checks override the inputs |
| the changelogs' released sections (4 in linlog, 4 in the library) | linlog's `CHANGELOG.md` lines 843, 911, 1072, 1073; the library's lines 53, 55, 62 | the record of 0.1.0 and 0.2.0 as released, with the addresses of their day; their GitHub links redirect for as long as no repository is created at the old names, which D26 rules out; 0.3.0's changelog says the repository moved |
| local checkout paths (2 in linlog, 2 in the client) | linlog's `.claude/skills/update-deps/SKILL.md` lines 9 and 152; the client's `CLAUDE.md` line 30 and `.claude/rules/claude-infra.md` line 27 | the directory `~/Projects/own/linlog-prover.github.io` and its workspaces' `../linlog-prover.github.io-wt-NAME` keep their paths (the prompt: the Claude Code settings, memory and scripts are keyed by them) |
| the old organisation named as such (2 in linlog) | `CLAUDE.md` (the `gh` login's reach) and `.claude/rules/ci.md` (where the pinning policy came from) | sentences about `linlog-prover` itself, which stays the author's |

Changed: everything else, 134 occurrences in linlog, 18 in the library,
25 in the client, all of the prompt's load-bearing list among them:
`Cargo.toml`'s `repository` (the workspace has no `homepage`; the
command's `documentation` in `cli/Cargo.toml` moved too);
`core/src/lib.rs`'s logo, favicon and `llms` links; `docs/book.toml`'s
`git-repository-url`, `edit-url-template` and, beyond the grep,
`site-url` (`/sequentium/manual/`, which mdBook's 404 page names);
`CITATION.cff` in both repositories; README; the manual's chapters
(installation, introduction, rocq, output formats, commands, status,
the results' links into the tree, the theme's logo); the guide for AI
models (`skills/linlog/`); `modules/manual.nix`'s three remaps and,
beyond the grep, the link check's root directory, `root/sequentium`;
the workflows (`release.yml`'s two user agents, the attestation
comment and the release notes' link, `install.yml`'s flake reference,
`nix profile add`, the latest-release API call and the download
address; the cache's `name:` kept); `flake.nix`'s input
`github:sequentium/rocq-linlog`; the client's input
`github:sequentium/sequentium`; `rocq-linlog.opam`'s four fields;
the client's `astro.config.ts` `site`, `package.json` (name and
description, and the lock's two copies of the name through `npm install
--package-lock-only`), the privacy page, `llms.txt` and the logo's
licence; CLAUDE.md and `.claude/` in all three (rules, the update
skill's text where it names the repository). `plan/push-main.sh` names
no remote address, only the local directories and `origin`, so it did
not change.

Beyond the string `linlog-prover`, the client reserves the path of the
shared repository's project site on its own origin: its service worker
leaves `/linlog` to the network (`src/offline/sw.js`, both in its fetch
handler and in its Static Routing API rules), its build refuses to write
a file there (`src/integrations/offline.ts`), the `pages-paths` check
fails on a file named so (`modules/checks.nix`), the test server serves
the docs' stand-in there (`tests/serve.ts`), and About, the 404 page and
`llms.txt.ts` link it. All of these now say `/sequentium`; left as
`/linlog`, the worker of `sequentium.github.io` would answer the
navigations to the manual and the rustdoc as its own.

### The placeholder site

A new jj repository (not colocated with git) in the scratchpad,
`…/scratchpad/linlog-prover.github.io`, one commit, for the author to
push into the repository `linlog-prover/linlog-prover.github.io` (act
9). Its files:

- `site/index.html`: the web page's old address; a script sends it to
  `https://sequentium.github.io/` with `location.search` and
  `location.hash` (a shared proof travels in them), a `<noscript>` meta
  refresh does the same without them, and the body links the new page,
  manual and source.
- `site/404.html`: Pages serves it for every path without a file, at
  that path; its script maps the prefix `/linlog` to `/sequentium` and
  `/rocq-linlog` to itself and every other path to itself, all on
  `https://sequentium.github.io`, query and fragment kept; the body
  lists the new addresses for a browser without scripts.
- `site/logo/`: the client's 13 files of `/logo/` (`square-dark.svg`, `square-light.svg`, `square-mono.svg`, `wordmark-dark.svg`, `wordmark-light.svg`, `wordmark-mono.svg`, `favicon-32.png`, `apple-touch-icon.png`, `icon-192.png`, `icon-512.png`, `icon-maskable-512.png`, `og.png`, `LICENSE.txt`), copied from the client's built site, under
  CC BY 4.0 (`logo/LICENSE.txt` among them).
- `site/llms.txt`, `site/linlog/llms.txt`, `site/linlog/llms-full.txt`
  and `site/linlog/reference.md` (which the guide links beside itself):
  the new texts (from the guide of this session's commits, made as the
  doc package makes them, and the client's `llms.txt`), each with a
  first line "This file moved to <new address>; this copy at the old
  address is not updated."
- `site/sw.js`: the replacement service worker (departures, below).
- `.github/workflows/pages.yml`: on a push to `main` or by dispatch, an
  upload job (`contents: read`) and a deploy job (`pages: write`,
  `id-token: write`, the `github-pages` environment, one deployment at a
  time), the actions pinned by commit as the client pins them.
- `README.md`: where everything went, a table of the repositories and
  sites, and what each file of `site/` is for; `LICENSE`: the EUPL-1.2,
  the client's file.

The licence header is on every source (the two HTML files, `sw.js`,
the workflow); README, the text copies, the logo files and `LICENSE`
carry none, as the client's conventions say. Checked in the session:
the 404 page's script run under Node on eleven old addresses (the
rustdoc's root with and without the slash, a manual page with a
fragment, a crate page with a query, a near miss `/linlogx/`, the
library's root and a page, `/privacy/`, an unknown logo file and the
root with query and fragment), each sent where the table says;
`actionlint` on the workflow, exit 0; `node --check` on `sw.js`.

### The opam change

The change `ywqlpnru` ("Move rocq-linlog.0.2.0 to the organisation
sequentium") on top of `omkspqll` in `~/Projects/own/rocq-opam` moves
`homepage`, `bug-reports`, `dev-repo` and `doc` to
`https://github.com/sequentium/rocq-linlog`, its `/issues`, its `.git`
and `https://sequentium.github.io/rocq-linlog/`, and also the `url`'s
`src` to `https://github.com/sequentium/rocq-linlog/archive/refs/tags/v0.2.0.tar.gz`.
The prompt names the four fields and says the SHA-512 is checked in 48b
"against the moved repository's archive", which reads as the `url`
naming the moved repository; a file sent after the move should not
name the old address either. The checksum is left as it is: the
archive's top directory is named after the repository, whose name
`rocq-linlog` does not change, so the moved repository's archive of the
same tag should hash alike, and 48b checks that before the author sends
the pull request. The library's own `rocq-linlog.opam` (commit
`ksuqsuzm`) has the same four fields moved and no `url`.

### The author's acts, in order

Each in a browser, as the owner of both organisations; acts 3 to 9 are
best done in one sitting and 48b right after, since from the transfer
of the site's repository until the placeholder is published the old
Pages addresses answer nothing, and until 48b pushes the new addresses
serve the old content or nothing.

- [x] **1. Create the organisation `sequentium`** (done 2026-11-08).
- [ ] **2. Mirror `linlog-prover`'s settings in `sequentium`**
  (Settings of each organisation side by side): two-factor
  authentication required for members; Actions allowed, with the
  policy that every action is pinned to a full-length commit SHA (the
  three repositories' workflows rely on it and `.claude/rules/ci.md`
  says the organisation requires it), and the default workflow
  permissions read-only; Pages allowed (public); the base permission
  for members as in `linlog-prover`. *Check*: each of these pages reads
  the same in both organisations.
- [ ] **3. Transfer `linlog-prover/rocq-linlog` to `sequentium`**, name
  kept (the repository's Settings, Danger Zone, Transfer). *Check*:
  `https://github.com/linlog-prover/rocq-linlog` lands on
  `https://github.com/sequentium/rocq-linlog`, with its release
  `v0.2.0` and its tag.
- [ ] **4. Transfer `linlog-prover/linlog` to `sequentium` and rename it
  `sequentium`** (the transfer's dialog takes the new name when the
  target is yours; otherwise transfer, then rename in Settings).
  *Check*: `https://github.com/linlog-prover/linlog` and
  `https://github.com/linlog-prover/linlog/releases/tag/v0.2.0` land on
  `https://github.com/sequentium/sequentium/…`.
- [ ] **5. Transfer `linlog-prover/linlog-prover.github.io` to
  `sequentium` and rename it `sequentium.github.io`**. *Check*: the
  repository is at `https://github.com/sequentium/sequentium.github.io`;
  `https://linlog-prover.github.io/` now answers 404 (expected until
  act 9).
- [ ] **6. On each of the three moved repositories, re-create what did
  not move** (Settings): Pages, source "GitHub Actions"; the
  environment `github-pages` with its deployment rule (`main` for the
  shared repository and the client, the tags `v*` for the library,
  whose release workflow deploys its documentation from a tag); in the
  shared repository the environment `crates-io` with its protection
  rules (the reviewer, if it had one); the secret `CACHIX_AUTH_TOKEN`
  in all three (the shared repository and the client push to the cache
  from `main`, the library once it has the secret); the branch rulesets;
  the topics; Dependabot's version updates and alerts. *Check*: each
  item is listed on its settings page. Then
  `https://sequentium.github.io/rocq-linlog/`: if it does not answer
  with the library's documentation (whether a Pages deployment survives
  a transfer is not sourced), open the library's Actions, the Release
  run of `v0.2.0`, and "Re-run all jobs": its `docs` job needs only
  `check` and deploys the documentation again, while its `release` job
  stops at `gh release create` because the release exists, which
  changes nothing. The shared repository's and the client's sites are
  deployed by 48b's pushes to `main`.
- [ ] **7. On crates.io, for `linlog` and for `linlog-cli`**: replace the
  Trusted Publishing configuration with owner `sequentium`,
  repository `sequentium`, workflow `release.yml`, environment
  `crates-io` (each crate's Settings, Trusted Publishing). *Check*: each
  crate's settings list exactly that one configuration. The proof is
  0.3.0's release (step 53's chain).
- [ ] **8. On Zenodo**: in GitHub, grant the Zenodo application access
  to `sequentium` (Settings, Applications, Zenodo, organisation
  access); in Zenodo's GitHub page, sync and switch on
  `sequentium/sequentium` and `sequentium/rocq-linlog`; edit the two
  concept records' metadata (10.5281/zenodo.23448588 and
  10.5281/zenodo.23899109): the related identifier and any URL to
  `https://github.com/sequentium/…`. *Check*: both repositories show as
  on in Zenodo's list; both records' pages show the new URL. Whether a
  release from the moved repository joins the existing concept record
  is checked at 0.3.0's release.
- [ ] **9. Create the public repository
  `linlog-prover/linlog-prover.github.io`** (empty: no README, licence
  or `.gitignore`), set its Pages source to "GitHub Actions" and push
  the placeholder site from this session's scratchpad (in nushell):
  ```
  cd /tmp/claude-1000/-home-tux-Projects-own-linlog/bc292df7-13ed-4a06-ba88-7d25bbec8602/scratchpad/linlog-prover.github.io
  jj git remote add origin git@github.com:linlog-prover/linlog-prover.github.io.git
  jj bookmark create main -r @-
  jj git push --bookmark main
  ```
  (the push signs the commit; the scratchpad is temporary, so do this
  before the machine reboots, or move the directory first). *Check*:
  the run of the workflow "Pages" passes; `https://linlog-prover.github.io/`
  and `https://linlog-prover.github.io/linlog/manual/` land on
  `https://sequentium.github.io/` and
  `https://sequentium.github.io/sequentium/manual/`, and
  `https://linlog-prover.github.io/logo/square-dark.svg` shows the logo.
  Never create a repository named `linlog` or `rocq-linlog` under
  `linlog-prover`: it would end GitHub's redirects.
- [ ] **10. Tell the planning session**, which starts 48b: the remotes,
  the locks, the pushes in order (the library, then linlog with its
  lock moved, then the client with its lock moved) and every row of the
  table fetched.
- [ ] **After 48b**: the opam pull request from the change `ywqlpnru`
  (45e's steps), once 48b has hashed the moved archive.

### Checks

Each `nix flake check` ran under the machine lock (`flock -o
/tmp/linlog-machine.lock`), detached in a user unit, with
`--print-build-logs --keep-going`, the inputs that name repositories
not yet moved overridden by `git+file:` to the workspaces (a `path:`
override of linlog's workspace would copy its untracked `target/`):

- **The Rocq library**, `nix flake check --max-jobs 2 --cores 6` at
  `ksuqsuzm`: "all checks passed!", exit 0 (15:45). Its Rocq checks were
  already built: the metadata moved is outside their sources; the
  `treefmt`, `actionlint`, `conventions`, `deadnix` and `typos` checks
  ran.
- **linlog**, `nix flake check --max-jobs 2 --cores 6 --override-input
  rocq-linlog git+file:///home/tux/Projects/own/rocq-linlog-48a
  --no-write-lock-file` at `poqskvln` (the three commits before the
  reviewer's correction): "all checks passed!", exit 0 (15:45 to
  16:00), `manual-links` (the new remaps and root), `conventions`
  (README and the manual name no plan), `typos`, the tests and the
  features among them. Again at this report's commit, with the
  reviewer's correction to the installation chapter, under `--max-jobs 4
  --cores 8`, its exit status in the session's final message to the
  supervisor.
- **The web client**, `nix flake check --max-jobs 4 --cores 8
  --override-input linlog git+file:///home/tux/Projects/own/linlog-step48a
  --override-input linlog/rocq-linlog
  git+file:///home/tux/Projects/own/rocq-linlog-48a --no-write-lock-file`
  at `ssyqpvvz`: "all checks passed!", exit 0 (16:00), its end-to-end
  run "124 passed" (the service worker's test that it leaves
  `/sequentium/` to the network, About's manual link, `llms.txt`'s
  links among them). These are the client's own checks: its `check`
  command is this flake check, which runs everything its CI runs.
- **`cargo test --workspace`** in linlog's workspace (capped scope,
  cores 10 to 13, `nix develop` with the same override), before the
  commits: exit 0, every test result "ok" (the README, manual and
  guide examples and names, the guide's budget, the behaviour lock's
  help outputs among them). A first run without the override failed
  before building, nix fetching `github:sequentium/rocq-linlog`, which
  does not exist yet (404): the reason every command here overrides.
- **The client's lock**: `npm install --package-lock-only --offline`
  with the client's pinned Node 24 changed only the two copies of the
  package's name ("up to date, audited 359 packages").
- **The placeholder**: its 404 script under Node on eleven addresses,
  all as the table says; `actionlint` exit 0; `node --check sw.js` exit
  0.
- Step 47's timed slot (15:52 to 15:54) ran while linlog's check was
  under way, which the supervisor let finish; nothing new started in
  it.

### The reviewer

One round by a fresh-context reviewer (Opus 5.5 at `high`, read-only),
given the table of addresses, the note's touch points (sections 1 and
5), the prompt's list and the commits of all four repositories and the
placeholder, asked to report only an address that would break or a
reference missed. It found every remaining occurrence on the kept list
with its reason holding, every touch point changed, and the
placeholder's pages and worker matching the table and the old worker's
registration and cache prefix. Its three findings, each answered:

1. **The attestations of 0.2.0 name the old repository** (inference):
   the release workflow of `linlog-prover/linlog` signed them, so
   `gh attestation verify FILE --repo sequentium/sequentium`, which the
   installation chapter gives while its examples download 0.2.0, would
   not verify those files. *Holds*; the chapter now says that the files
   of 0.2.0 and earlier are checked with `--repo linlog-prover/linlog`
   (commit `mzkukvsr`). The workflow's comment speaks of the releases it
   makes, which are the new repository's, and stays. Neither form was
   run (`gh` asks the author first); 48b or 0.3.0's release can check
   both with the author's yes.
2. **The Rocq library's Pages site may be empty at its new address**,
   since only its release workflow deploys it, on a tag. *Holds as a
   risk, already answered*: act 6 of the author's checklist checks
   `https://sequentium.github.io/rocq-linlog/` after the transfer and,
   if it is empty, re-runs the `v0.2.0` Release run, whose `docs` job
   needs only `check` (read in the workflow) and whose `release` job
   then stops at the existing release. No `workflow_dispatch` was added:
   the re-run does the same without a change to the library.
3. **crates.io's Trusted Publishing and Zenodo must be configured for
   the new repository**. *Already in the checklist*: acts 7 and 8.

### Departures and assumptions

- **The placeholder serves a replacement service worker, `sw.js`**
  (departs from the design, which lists redirect pages, the logos, the
  `llms` files, `404.html` and a README). The old page registered a
  worker at `/sw.js` for the whole origin that answers `/` from its
  cache before the network, so a returning visitor would keep seeing
  the old page and never reach the redirect. The browser checks a
  registered worker's script at each visit; the replacement takes over
  at once, deletes the old page's caches (the prefix `linlog-site-` of
  the client's worker), unregisters itself and reloads the open tabs,
  the known self-unregistering pattern. It leaves the old origin's
  storage, the visitors' stored proofs, alone.
- **The opam file's `url` moved too** (a reading): the prompt names four
  fields and has 48b check the SHA-512 against the moved repository's
  archive; the archive's top directory carries the unchanged name
  `rocq-linlog`, so the checksum is expected to hold, which 48b checks.
- **The client's reserved path moved with the address** (a reading, not
  in the prompt's list): the service worker, the build's guard, the
  `pages-paths` check and the test server reserved `/linlog`, the old
  project site's path on the same origin; they reserve `/sequentium`
  now, or the worker would answer the manual's and the rustdoc's
  navigations as its own.
- **The web page's own `/llms.txt` is a file in the placeholder too**
  (a reading): the prompt names `llms.txt` and `llms-full.txt`; the old
  origin served three such files (linlog's two under `/linlog/` and
  the page's at `/`), and a text client follows no redirect for any.
- **`package.json`'s name changed with its description**
  (`sequentium-site`), and the lock's two copies of it through `npm
  install --package-lock-only`, the only change npm made.
- **The status chapter keeps two items no release names yet** (a
  reading): first-order proving on the web page and the comparison
  with the engines of 0.2.0 stay, under "Not yet placed in a release",
  rather than being dropped from what is planned.
- **No changelog entry**: the changelog has no section for unreleased
  work; 0.3.0's release writes its section and should say there that
  the repository moved.
- **The placeholder's workflow has no check job**: the site is static
  files and nothing is built; it follows the client's deploy job (the
  same pinned actions, `permissions: {}` with rights per job, the
  `github-pages` environment, one deployment at a time). Its redirect
  logic was tested in the session (below) and its workflow linted with
  actionlint.


### What the next sessions must know

- **The new addresses**, which every later prompt uses:
  `https://github.com/sequentium/sequentium` (`github:sequentium/sequentium`),
  `https://github.com/sequentium/rocq-linlog`,
  `https://github.com/sequentium/sequentium.github.io`; the web page
  `https://sequentium.github.io/`; the rustdoc
  `https://sequentium.github.io/sequentium/` (the crate at
  `…/sequentium/linlog/`), the manual `…/sequentium/manual/`, the guide
  `…/sequentium/llms.txt` and `…/llms-full.txt`; the Rocq library's
  documentation `https://sequentium.github.io/rocq-linlog/`; the logos
  `https://sequentium.github.io/logo/`. The Cachix cache stays
  `linlog-prover`. The local checkouts keep their paths.
- **48b's remotes**: today `git@github.com:linlog-prover/linlog.git`,
  `…/rocq-linlog.git` and `…/linlog-prover.github.io.git`; after the
  transfers `jj git remote set-url origin
  git@github.com:sequentium/sequentium.git` in `~/Projects/own/linlog`,
  `…:sequentium/rocq-linlog.git` in `../rocq-linlog` and
  `…:sequentium/sequentium.github.io.git` in `../linlog-prover.github.io`.
  The commits to stack: linlog `rpkkstus`, `xtvmuwom`, `poqskvln`,
  `mzkukvsr` and this report's; the library `ksuqsuzm`; the client
  `ssyqpvvz`; then the locks, `nix flake update rocq-linlog` here and
  `nix flake update linlog` in the client, each "flake.lock: Update".
- **Until the locks move, every nix command in these trees needs the
  moved inputs overridden**: here `--override-input rocq-linlog
  git+file:///home/tux/Projects/own/rocq-linlog-48a --no-write-lock-file`
  (`nix develop` too), in the client `--override-input linlog
  git+file:///home/tux/Projects/own/linlog-step48a --override-input
  linlog/rocq-linlog git+file:///home/tux/Projects/own/rocq-linlog-48a`.
  A `path:` override of linlog's workspace would copy its `target/`.
- **The placeholder site** is the jj repository in this session's
  scratchpad (path in the outcome), one commit, not colocated with git;
  act 9 pushes it. It must keep existing: every published rustdoc on
  docs.rs embeds `https://linlog-prover.github.io/logo/…`.
- **The client reserves `/sequentium`** on its origin (its worker, its
  build, the `pages-paths` check, the test server). It does not reserve
  `/rocq-linlog`, the library's project site on the same origin, which
  its worker leaves to the network anyway (it answers only its own
  files and `/`); a guard there is a follow-up, not a defect.
- **The attestations**: files of 0.2.0 and earlier verify with `--repo
  linlog-prover/linlog`, later ones with `--repo sequentium/sequentium`
  (both unverified until someone runs `gh` with the author's yes).
- **0.3.0's release (step 53)** proves the chain: Trusted Publishing
  under `sequentium/sequentium`, the GitHub release, Zenodo's records;
  its changelog says the repository moved.

### What is left open

- The author's acts 2 to 10, then 48b.
- Visitors' stored proofs stay in the browser storage of the old origin
  `linlog-prover.github.io`, which the new page cannot read; the
  placeholder leaves them in place, so a later export from the old
  origin stays possible if the author wants one.
- Whether Nix follows GitHub's redirect for `github:linlog-prover/linlog`
  (published READMEs and docs name it) is still not verified; 48b can
  try `nix run github:linlog-prover/linlog -- --version` after the
  transfer.
- Whether a Pages deployment survives a transfer (act 6 checks it for
  the library).

### Checklist

- [x] Workspaces `../linlog-step48a` on `utzkqlzy`,
  `../rocq-linlog-48a` on `wyvsntvq`, `../linlog-prover.github.io-48a`
  on `ymrkqtrq`; the opam clone's change on `omkspqll`.
- [x] Read the brief, the prompt, `conduct.md`, the research note,
  D22 and D26 (and D24, D25, D27), `ci.md`, `flake.md` and `web.md`'s
  parts on the reserved path, 45e's section, both other CLAUDE.md files.
- [x] The old name counted again in the three repositories outside
  `plan/` (164, 25, 36), every occurrence changed or kept with its
  reason, the load-bearing list changed.
- [x] README and the introduction's sentence on the collection; the
  status chapter's planned list as 0.3.0 to 0.7.0, naming no plan.
- [x] linlog's commits `rpkkstus` (the help and its lock), `xtvmuwom`
  (the addresses), `poqskvln` (the front page and the plan of
  releases), `mzkukvsr` (the attestations of 0.2.0), and this report.
- [x] The library's commit `ksuqsuzm`; the client's `ssyqpvvz`; the
  opam change `ywqlpnru`.
- [x] The placeholder site, the jj repository in the scratchpad,
  commit `wmvquqvq`; its redirects in the table above, tested.
- [x] The author's acts as a checklist with what to check after each.
- [x] `nix flake check` in each repository, the moved inputs
  overridden: exit 0 in all three; the client's own checks included.
- [x] The fresh-context reviewer's round: three findings, one
  corrected, two already covered; answered above.
- [x] Nothing pushed, created, transferred or renamed.
- [x] `jj st` clean in every workspace and the placeholder; the final
  message to `linlog-orchestrator`.

## Session 48b: the move finished and checked

### Outcome

The move to `sequentium` is finished. The three repositories' `main`
hold 48a's commits and the moved locks, each pushed by the supervising
session after this session's message that the repository was ready, in
the order the step sets, and each checked at that head with `nix flake
check` and no override:

| repository | pushed head | check at that head |
|---|---|---|
| `sequentium/rocq-linlog` | `ksuqsuzm` (commit `ff02e644aa3c`), 48a's addresses | "all checks passed!", exit 0 |
| `sequentium/sequentium` | `vwouyluy` (commit `5f308848c815`), "flake.lock: Update" on 48a's chain rebased onto `main` | "all checks passed!", exit 0 |
| `sequentium/sequentium.github.io` | `utykqkvt` (commit `b45452fca458`), "flake.lock: Update" on 48a's `ssyqpvvz` | "all checks passed!", exit 0, its unit tests (557) and end-to-end run (124) among them |

Every row of 48a's table of addresses was fetched after the Docs and
Pages workflows had deployed, and answers as the table says; the
placeholder's redirect scripts, fetched from the live site, send
thirteen old addresses where the table says; `nix run
github:sequentium/sequentium -- --version` from an empty environment
and directory printed `linlog-cli 0.2.0`. The library's `v0.2.0`
archive at its new address hashes to the opam file's SHA-512. Nothing
failed. What could not be checked from here is listed below, each with
who checks it and when.

### What was done, in order

1. **The library.** Its workspace was clean on `ksuqsuzm`, whose parent
   was `main@origin` (`wyvsntvq`); its flake check passed; "rocq ready"
   went to the supervisor, which pushed it ("pushed": `main` is
   `ff02e644aa3c`, confirmed by a fetch).
2. **linlog.** 48a's chain (`rpkkstus`, `xtvmuwom`, `poqskvln`,
   `mzkukvsr`, `unxsytvk`) was rebased onto `main` (`wvpxvkqp`, which
   had gained two plan commits touching no file of the chain), without
   a conflict. `nix flake update rocq-linlog` moved the input from
   `github:linlog-prover/rocq-linlog/b895ecd1` (2026-11-02) to
   `github:sequentium/rocq-linlog/ff02e644` (2026-11-08), committed as
   "flake.lock: Update" with nix's summary as its body. The flake check
   passed; "linlog ready" went out; "pushed": `main` is `5f308848c815`.
3. **The client.** `nix flake update linlog` moved the input from
   `github:linlog-prover/linlog/1f4f986a` (2026-11-02) to
   `github:sequentium/sequentium/5f308848` and with it
   `linlog/rocq-linlog` to `github:sequentium/rocq-linlog/ff02e644`,
   committed as "flake.lock: Update". Its flake check (which is the
   client's `check`, everything its CI runs) passed; "client ready" went
   out; "pushed": `main` is `b45452fca458`.
4. **The addresses**, after the supervisor saw the Docs workflow of
   `sequentium/sequentium` deploy (the manual's introduction with the
   collection's sentence served) and the client's Pages deploy land
   (its `/llms.txt` naming `/sequentium/`).

Every commit is signed (the global hook signed each right after the jj
command that made it; the key was never locked).

### The addresses, fetched

Each with `curl -sSIL` or `curl -sSL` once (a script listing each
address, no loop against GitHub's API; one API request in all):

- **GitHub's redirects** (rows 1 and 2): `github.com/linlog-prover/linlog`
  and its `releases/tag/v0.2.0` land with one redirect on
  `github.com/sequentium/sequentium/…`, status 200;
  `github.com/linlog-prover/rocq-linlog` on
  `github.com/sequentium/rocq-linlog`; both old `archive/refs/tags/v0.2.0.tar.gz`
  addresses on `codeload.github.com/sequentium/…`; the API's
  `repos/linlog-prover/linlog` on `repositories/1393906688`; git's
  smart-HTTP `info/refs?service=git-upload-pack` at both old names
  answers 200 with the tag `v0.2.0` listed (a `HEAD` request there
  answers 405, so these two were `GET`s).
- **The site's repository** (row 3): `github.com/linlog-prover/linlog-prover.github.io`
  is the placeholder's repository, 200, no redirect, as the table says;
  `github.com/sequentium/sequentium.github.io` answers 200.
- **Nix** (rows 4 and 5, which 48a left unverified): `nix run
  github:linlog-prover/linlog -- --version` from an empty environment
  printed `linlog-cli 0.2.0`, exit 0, and `nix flake metadata
  github:linlog-prover/rocq-linlog` locked
  `github:linlog-prover/rocq-linlog/ff02e644…`, the pushed head: Nix
  follows GitHub's redirect, so every published README's and doc's old
  flake reference keeps working while no repository takes the old
  names.
- **The placeholder** (rows 6 to 16): `/` and `/index.html?proof=x#s`
  answer 200 with "linlog has moved"; `/linlog` (a 301 to `/linlog/`),
  `/linlog/`, `/linlog/linlog/index.html`, `/linlog/manual/`,
  `/linlog/manual/commands.html`, `/rocq-linlog/` and `/privacy/`
  answer 404 with the placeholder's page (Pages's `404.html`, whose
  script redirects); `/linlog/llms.txt`, `/linlog/llms-full.txt`,
  `/linlog/reference.md` and `/llms.txt` answer 200, each first line
  "This file moved to" its new address; `/sw.js` answers 200, the
  unregistering worker; the thirteen files of `/logo/` answer 200 (SVG,
  PNG and the licence's text).
- **The placeholder's mapping**, checked as 48a did: the inline scripts
  of the live `404.html` and `index.html`, fetched from
  `linlog-prover.github.io`, run under Node with a stubbed `location` on
  thirteen old addresses (the rustdoc's root with and without the
  slash, a manual page with a fragment, a crate page with a query, a
  near miss `/linlogx/`, the library's root, a page and a root with
  query and fragment, `/privacy/`, an unknown logo file, the root with a
  shared proof's query and fragment, `/index.html?x=1`): each is sent
  where the table says (`/linlog…` to `/sequentium…`, the rest to the
  same path, all on `https://sequentium.github.io`, query and fragment
  kept).
- **The new addresses**: `https://sequentium.github.io/` (the page),
  `/sequentium` (a 301 to `/sequentium/`), `/sequentium/`,
  `/sequentium/linlog/` ("linlog - Rust"), `/sequentium/manual/` (with
  the collection's sentence), `/sequentium/manual/commands.html`,
  `/sequentium/llms.txt`, `/sequentium/llms-full.txt` and
  `/sequentium/reference.md` (each naming `sequentium.github.io/sequentium/manual/`),
  `/rocq-linlog/` (the library's documentation, so act 6's re-run was
  either done or not needed), `/privacy/` and the thirteen files of
  `/logo/` answer 200; the client's `/llms.txt` answers 200 and names
  `https://sequentium.github.io/sequentium/llms.txt`, `…/llms-full.txt`
  and `…/sequentium/linlog/` (the new build, deployed at 16:03 UTC;
  a first pass before that deploy still saw the old build, which named
  `linlog-prover.github.io/linlog/`, so the whole table was fetched
  again after both deploys, with the results above).
- **The archive**: `https://github.com/sequentium/rocq-linlog/archive/refs/tags/v0.2.0.tar.gz`
  hashes to `sha512=14b5d239…39094b6e`, the opam file's checksum, equal
  (`sha512sum`), as the supervisor had found.

### Departures and assumptions

- None from the design in this session. 48a's departures stand as its
  section lists them (the placeholder's replacement service worker, the
  opam file's `url` moved, the client's reserved path moved, the page's
  `/llms.txt` as a file in the placeholder, `package.json`'s name, the
  status chapter's two unplaced items, no changelog entry, no check job
  in the placeholder's workflow).
- **The lock moved the library by more than 48a's commit** (a
  reading of "move the lock to the pushed library"): linlog's lock had
  pinned the library at `b895ecd1` of 2026-11-02, so `nix flake update
  rocq-linlog` brought in every library commit since, up to the pushed
  `ff02e644`; the flake check at that head passed. The client's lock
  likewise moved linlog from 2026-11-02 to the pushed head.

### What failed or stayed unchecked

Nothing that was checked failed. Not checked here, each with who
checks it:

- **The redirects in a browser**: the placeholder's scripts were run
  under Node, not in a browser, and the replacement service worker
  (`/sw.js`) was fetched, not seen to take over a browser that holds
  the old page's worker. *The author*, once, in a browser that visited
  the old page: open `https://linlog-prover.github.io/` and see it land
  on `https://sequentium.github.io/`.
- **The attestations of 0.2.0 and later**: `gh attestation verify FILE
  --repo linlog-prover/linlog` for 0.2.0's files, and `--repo
  sequentium/sequentium` for later ones, need `gh`, which asks the
  author. *Step 53* (0.3.0's release), with the author's yes.
- **crates.io's Trusted Publishing and Zenodo** for the moved
  repositories (acts 7 and 8): configured by the author, provable only
  by a release. *Step 53*.
- **The organisation's settings** (act 2) are not readable without
  `gh`; this session did not compare them.

### What the next sessions must know

- **Step 49 is next.** Every repository is at its new address and every
  lock names it: linlog's input is `github:sequentium/rocq-linlog`, the
  client's `github:sequentium/sequentium`. **No override is needed any
  more**; 48a's note that every nix command needs one no longer holds.
- **The addresses** are 48a's (its "What the next sessions must know"):
  the repositories `github.com/sequentium/sequentium`,
  `…/rocq-linlog`, `…/sequentium.github.io`; the page
  `https://sequentium.github.io/`; the rustdoc
  `https://sequentium.github.io/sequentium/` (the crate at
  `…/sequentium/linlog/`), the manual `…/sequentium/manual/`, the guide
  `…/sequentium/llms.txt`; the library's documentation
  `https://sequentium.github.io/rocq-linlog/`. The Cachix cache stays
  `linlog-prover`; the local checkouts keep their paths; the remotes
  (`origin`) of all three checkouts name `sequentium`.
- **The old addresses keep working** as long as no repository named
  `linlog` or `rocq-linlog` is created under `linlog-prover` (GitHub's
  redirects, which Nix follows too) and the placeholder repository
  `linlog-prover/linlog-prover.github.io` stays published (its site
  serves the logos every published rustdoc on docs.rs embeds). The
  placeholder's source is that repository; 48a's scratchpad copy is not
  needed.
- **The opam pull request** is the author's, from the change `ywqlpnru`
  in `~/Projects/own/rocq-opam`; its steps are in the final message to
  the supervisor and below. The checksum holds for the new `url`.
- **Step 53** proves the release chain from the new repository
  (Trusted Publishing, the GitHub release, Zenodo, the attestations) and
  says in 0.3.0's changelog that the repository moved.

### The author's opam pull request, step by step

From `~/Projects/own/rocq-opam` (its `origin` is
`https://github.com/rocq-prover/opam.git`; `omkspqll` "Add
rocq-linlog.0.2.0" with `ywqlpnru` "Move rocq-linlog.0.2.0 to the
organisation sequentium" on top; `@` empty):

1. Fold the move into the one commit, so the pull request adds the
   package at its new addresses only: `jj squash --from ywqlpnru --into
   omkspqll --use-destination-message`; `jj log -r omkspqll` then shows
   the commit and `jj file show -r omkspqll
   released/packages/rocq-linlog/rocq-linlog.0.2.0/opam` its file, with
   `sequentium` in all five addresses and the checksum unchanged.
2. Fork `rocq-prover/opam` on GitHub, if it is not forked yet.
3. `jj git remote add fork git@github.com:USER/opam.git` (USER the
   author's account).
4. `jj git push --remote fork --named "rocq-linlog.0.2.0=omkspqll"`,
   which creates the bookmark, pushes it and signs the commit.
5. Open the pull request from the fork's branch `rocq-linlog.0.2.0`
   against `master` of `rocq-prover/opam`, titled `Add
   rocq-linlog.0.2.0`, with the body of 45e's report ("The pull
   request's text"), its two old links changed: linlog at
   `https://github.com/sequentium/sequentium` and the documentation at
   `https://sequentium.github.io/rocq-linlog/`.
6. coqbot runs the repository's GitLab CI (lint, an install per
   compiler); a maintainer merges.

### Checks

Each `nix flake check` and `nix run` ran detached in a user unit on
cores 4 to 15 under the machine lock (`flock -o
/tmp/linlog-machine.lock`), with `--max-jobs 4 --cores 8
--print-build-logs --keep-going`, no override:

- the library at `ksuqsuzm`: "all checks passed!", exit 0;
- linlog at `vwouyluy`: "all checks passed!", exit 0, no line of the
  log matching `error` or `failed`;
- the client at `utykqkvt`: "all checks passed!", exit 0 ("Tests 557
  passed (557)", "124 passed");
- `nix run --refresh github:sequentium/sequentium -- --version` under
  `env -i` (only `HOME`, `USER` and the system's `PATH`) in a fresh
  temporary directory: `linlog-cli 0.2.0`, exit 0, once after linlog's
  push and again after both deploys (then resolving `main` to
  `cd0c3418`, the supervisor's plan commits on top of the lock); the
  same for `github:linlog-prover/linlog`: `linlog-cli 0.2.0`, exit 0.

### Checklist

- [x] Read the brief, the step's section, 48a's report and
  `conduct.md`.
- [x] The library's workspace clean and its check passed at
  `ksuqsuzm`; "rocq ready" sent; "pushed" received.
- [x] linlog's chain rebased onto `main`; the lock moved ("flake.lock:
  Update"); the check passed without overrides; "linlog ready" sent;
  "pushed" received.
- [x] The client's lock moved to `github:sequentium/sequentium`
  ("flake.lock: Update"); its check passed without overrides; "client
  ready" sent; "pushed" received.
- [x] The `v0.2.0` archive at its new address hashed against the opam
  file's SHA-512: equal.
- [x] Every row of 48a's table fetched after the Docs and Pages
  workflows; the placeholder's mapping run under Node from the live
  pages.
- [x] `nix run github:sequentium/sequentium -- --version` from a clean
  shell; the old flake references too.
- [x] What stayed unchecked written above, with who checks it.
- [x] This section committed on the pushed chain; `jj st` clean in all
  three workspaces; the final message to `linlog-orchestrator`.
