# The name and the organisation: renaming `linlog-prover`, and a name for a collection of logic suites

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research
sub-agent; sources read that day.

The author's words (2026-11-08): "I might need/want to rename the
linlog-prover org to something else and incorporate the new repos for the
new features." This note settles what a rename touches, how each service
behaves on a rename or a transfer, which names are free, and what it costs.
Nothing was registered, renamed or created; every availability check below
is a read of a public page.

## 1. What linlog has today

Counted by `grep` over the workspaces on 2026-11-08 (the string
`linlog-prover`):

- **Repository `linlog`**: 88 files and 305 occurrences, of which 47 files
  and 164 occurrences lie outside `plan/` (the rest is plan history, which
  may keep the old name as history). The load-bearing ones:
  - `Cargo.toml` line 21, `repository = "https://github.com/linlog-prover/linlog"`
    (inherited by `linlog` and `linlog-cli`); `core/src/lib.rs` lines 5 and 6
    (`html_logo_url`, `html_favicon_url` on `linlog-prover.github.io/logo/`)
    and lines 29 and 31 (`llms.txt`, `llms-full.txt`); `docs/book.toml` lines
    24 to 26 (`git-repository-url`, `edit-url-template`); `CITATION.cff`
    (`repository-code`, `url`, and the reference to `rocq-linlog`).
  - README (19 occurrences: the logo, the manual, the releases,
    `nix run github:linlog-prover/linlog`, `nix profile add ...`), the manual's
    chapters (`installation.md` 15, `introduction.md` 11, `rocq.md` 5,
    `output-formats.md` 4 raw.githubusercontent.com image URLs,
    `development.md` 4, `status.md` 3), `skills/linlog/SKILL.md` (5) and
    `reference.md` (6), which the test `cli/tests/readme.rs` reads.
  - `modules/manual.nix` lines 69 to 71: three `--remap` rules of the link
    checker that name the org, the raw URL and the Pages address.
  - `.github/workflows/release.yml`: `cachix-action` `name: linlog-prover`
    (twice), the `curl` user agents, the comment on
    `gh attestation verify FILE --repo linlog-prover/linlog`, and the
    `publish` job with `environment: crates-io` and `id-token: write`
    through `rust-lang/crates-io-auth-action`, which is how Trusted
    Publishing is used; `install.yml` (5: `FLAKE: github:linlog-prover/linlog/...`,
    the cache, the latest-release API call and the download URL);
    `plan/push-main.sh` (the remote's name).
  - `.claude/rules/ci.md` records that the cache is Cachix `linlog-prover`
    (public, `https://linlog-prover.cachix.org`, key
    `linlog-prover.cachix.org-1:6K+DDVwwELnSYtfflwUdsHdYuolC7hprdD/r2/KOQZU=`),
    and `docs/src/development.md` tells users to add that substituter and key.
- **Repository `rocq-linlog`** (`../rocq-linlog`): 25 occurrences in
  `rocq-linlog.opam` (`homepage`, `bug-reports`, `dev-repo`, `doc:
  "https://linlog-prover.github.io/rocq-linlog/"`), `CITATION.cff`,
  `README.md`, `CHANGELOG.md`, both workflows and `modules/rocq-linlog.nix`.
  linlog's `flake.lock` pins it as the input `rocq-linlog`.
- **Repository `linlog-prover.github.io`** (the web client, a user/organisation
  site): 36 occurrences in 17 files, among them `astro.config.ts` (`site:
  "https://linlog-prover.github.io"`), `package.json`, `flake.nix`,
  `flake.lock` and `modules/linlog.nix` (the input on
  `github:linlog-prover/linlog`), the CI, the privacy page, `llms.txt` and
  the logo licence. It serves the logos that README and the rustdoc embed
  (`.claude/rules/core.md`, "The rustdoc shows linlog's logo": the paths
  there "must stay stable").
- **Published artefacts** (`plan/reports/45-release-0.2.0.md`):
  crates.io `linlog` and `linlog-cli` at 0.2.0, each carrying the
  repository URL of `Cargo.toml` in its immutable package; the GitHub
  release `v0.2.0`; Zenodo's concept DOI 10.5281/zenodo.23448588 (linlog;
  version record 23899551) and 10.5281/zenodo.23899109 (rocq-linlog; version
  record 23899110); the opam package `rocq-linlog.0.2.0`, whose submission to
  `rocq-prover/opam` is prepared as a local commit (`40954aee`, section 45e)
  and not yet merged or even submitted ("ready for the author to submit");
  the Cachix cache `linlog-prover`.
- **Crate and package names inside the code** (`linlog`, `linlog_cli`,
  `linlog_web`, the opam package `rocq-linlog`, the Rocq logical path
  `Linlog`) do not contain the organisation's name. A rename of the
  organisation touches none of them.
- `plan/notes/distribution.md` (2026-10-03) records why the organisation was
  chosen before the first publication: "the repository's URL goes into crate
  metadata that cannot be changed, into the Pages address and into Zenodo's
  records". The same facts are the cost of a rename now.

## 2. The field: how each service behaves

Each claim below is from the service's own page; where the page is silent
the gap is said.

### GitHub

- **Renaming an organization**
  ([docs](https://docs.github.com/en/organizations/managing-organization-settings/renaming-an-organization),
  read 2026-11-08). Only owners can. Web links to the repositories redirect
  (it can take a few minutes); GitHub Packages move to the new namespace.
  Not redirected: the profile page (404), API requests under the old name
  (404), @mentions of teams. A push to the old git URL still works but
  GitHub recommends updating remotes, "because the old name becomes available
  for anyone to claim", and a new owner of the old name "could create
  repositories that override the redirects". A namespace with a Marketplace
  action, or more than 100 clones or Actions uses in the prior week, has the
  old owner/repository combination permanently retired. The page does not
  mention GitHub Pages or Sponsors.
- **Transferring a repository**
  ([docs](https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository)).
  Web links, `git clone`, `git fetch` and `git push` redirect. Issues, pull
  requests, wiki, stars, watchers, webhooks, secrets and deploy keys stay.
  The page says "we don't redirect GitHub Pages associated with the
  repository" (quoted exactly at the review of 2026-11-08; the first
  reading paraphrased it). Redirects are permanently
  deleted if a new repository or fork is created at the old location. A
  repository with more than 100 clones or Actions uses in the prior week is
  retired at its old name. To transfer into an organization one needs
  admin on the repository and the right to create repositories there; to
  rename during the transfer one must own the target.
- **Renaming a repository**
  ([docs](https://docs.github.com/en/repositories/creating-and-managing-repositories/renaming-a-repository)).
  Issues, wikis, stars and followers redirect, as do git operations; "project
  site URLs" do not. GitHub recommends a custom domain before renaming to keep
  a Pages URL stable. Workflows that `uses:` a renamed action repository fail
  (linlog's workflows use others' actions, not its own, so this does not apply).
- **Pages** ([about](https://docs.github.com/en/pages/getting-started-with-github-pages/about-github-pages)).
  A user or organisation site must be in a repository named
  `<owner>.github.io` and is served at `https://<owner>.github.io`; project
  sites at `https://<owner>.github.io/<repository>`. So the name of
  `linlog-prover.github.io` is bound to the organisation's name, and the
  manual (`/linlog/manual/`), rustdoc (`/linlog/`), rocq-linlog's docs
  (`/rocq-linlog/`) and the logo paths all move with it.
- **Custom domains**
  ([about](https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site/about-custom-domains-and-github-pages)).
  A custom domain set on the organisation site is used "for all project
  sites owned by the same account", the project sites appearing as
  `domain/repository`. The page does not say whether the old
  `github.io` address redirects to the custom domain.

### crates.io and docs.rs

- **A crate cannot be renamed** (inference). The crates.io policies page and
  the trusted-publishing documentation pages are single-page applications
  that WebFetch could not read, so the policy text was read only through a
  search result's quotation of
  [crates.io/policies](https://www.crates.io/policies): name-squatting
  ("exists only to reserve a name for a prolonged period ... without any
  genuine functionality, purpose, or significant development activity") is
  forbidden, names may not be traded for money, and ownership of an existing
  crate is not transferred without the current owner's approval. Cargo's
  [publishing chapter](https://doc.rust-lang.org/cargo/reference/publishing.html)
  says names are first come, first served and a publish "is generally
  permanent. The version can never be overwritten, and the code cannot be
  deleted". It describes no rename. A new name is therefore a new crate;
  the old one stays, can be yanked and can say in its README where the
  successor is. The same chapter does not mention the `repository` field's
  permanence; that the published packages keep their old URL follows from
  the package being immutable (inference).
- **Trusted Publishing** is configured per crate by repository owner,
  repository name, workflow filename and an optional environment
  ([RFC 3691](https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html):
  "required-claims"; any owner of the crate can create, delete or edit the
  configurations; the RFC mentions no numeric repository id and says nothing
  on rename, transfer or reuse; the
  [notification-template change](https://github.com/rust-lang/crates.io/pull/12049/files)
  confirms the stored fields). Hence after an owner or repository rename the
  configuration for `linlog` and `linlog-cli` must be edited by a crate owner
  (inference: the claim in the OIDC token carries the new name, which no
  longer matches). Whether crates.io matches on an id is not documented in
  anything read. The workflow uses environment `crates-io`, which lives in
  the repository's settings and moves with a transfer (inference; the
  transfer page lists secrets and webhooks, not environments).
- **docs.rs** ([about](https://docs.rs/about)) page read: it says nothing on
  repository URLs or renames (unsourced beyond that). docs.rs builds from the
  crate package, not from GitHub, so a rename leaves it building (inference);
  the repository link on a published version's page is that version's
  manifest value, which stays the old URL and then works through GitHub's
  redirect until the old name is reused.

### Zenodo

- The [enable-repository page](https://help.zenodo.org/docs/github/enable-repository/)
  says a linked GitHub account, a toggle per repository and that "new
  releases ... will be automatically ingested and archived". It says
  nothing on rename or transfer; nor does the
  [linking page](https://help.zenodo.org/docs/profile/linking-accounts/)
  (unsourced on both).
- A search for guidance
  ([results](https://github.com/zenodo/zenodo/issues/826)) found only user
  reports: a repository moved to an organisation and re-synced kept its old
  releases and took the new URL; another user edited the URL in the metadata
  without a new version; whether a release still triggers after a transfer
  to another owner is an unanswered question; a GitHub repository cannot be
  attached to an existing concept DOI. All anecdotal.
- The two DOIs are resolved by doi.org to Zenodo's own records, which are
  independent of the GitHub URL; what stays stale is the record's metadata
  (the related identifier to the tree at `v0.2.0`, the description). A
  published record's metadata can be edited by its owner in Zenodo (common
  practice; unsourced in the pages read). The organisation's Zenodo app
  authorisation is per organisation (`plan/notes/distribution.md` line 183
  mentions the owner approving the app), so a new organisation needs it again
  (inference).

### opam

- The Rocq opam repository README
  ([github.com/rocq-prover/opam](https://github.com/rocq-prover/opam)) says
  how to add a package under `released/packages/` (the `tags` fields
  `keyword:`, `category:`, `logpath:`; `homepage:`, `author:`, `maintainer:`
  and `doc:` feed the website) and does not describe renaming, deprecation or
  fixing the metadata of a released package. The opam-repository's
  [archiving policy](https://github.com/ocaml/opam-repository/blob/master/governance/policies/archiving.md)
  covers archiving only. The "renamed package is a new package, the old one
  marked" convention in the task's brief is therefore not sourced here.
- The package name `rocq-linlog` contains no organisation name. Only the
  fields `homepage`, `bug-reports`, `dev-repo` and `doc` of
  `rocq-linlog.opam` do, and the pending pull request's file is one commit in
  a local clone, so those lines can be edited before anything is submitted
  (inference from the report's "ready for the author to submit"). The `url`
  of the file is the `v0.2.0` tag's archive with a SHA-512 over it; the old
  GitHub archive URL keeps working through the redirect as long as the old
  name is not reused.

### Cachix

- The [documentation](https://docs.cachix.org/) and
  [getting-started](https://docs.cachix.org/getting-started.html) pages say
  a cache name "appears in your Nix configuration and is visible to anyone
  using it" and describe tokens and signing keys; neither says a cache can be
  renamed or deleted (unsourced). The cache's name, URL and public key are
  independent of GitHub, so keeping `linlog-prover` as the cache name works
  with any organisation name (inference); the name only looks odd.

### Nix flake URLs

`github:linlog-prover/linlog` is resolved by nix through GitHub; whether
nix follows GitHub's redirect for a renamed owner is "inferred, not
verified" already in `plan/notes/distribution.md` and was not read here
(unsourced). `flake.lock` files of the web client and of linlog pin the
owner and repository, so they need a lock refresh after a move.

## 3. Candidates

### name-and-organisation-1: Keep the organisation `linlog-prover`; add the new repositories to it

- *What it brings and to whom*: nothing breaks. New repositories (for
  example a generic kernel, ordinary-logic suites, a lambda-calculus suite)
  join the organisation and the site grows beside `linlog`. To the author:
  no work and no risk to the published artefacts.
- *In linlog's terms*: no change; the site's address stays
  `linlog-prover.github.io`.
- *Cost*: 0 sessions. The cost is a brand: an organisation named after one
  logic ("linlog-prover") holding a Lambek, modal or lambda suite, a name
  that says "linear logic prover" over a collection that is not.
- *Risk*: a later rename costs more, since every release adds URLs (crate
  metadata, Zenodo records, opam fields) that cannot be changed.
- *Prerequisites*: none.
- *Sources*: sections 1 and 2.

### name-and-organisation-2: Rename the organisation in place

- *What it brings*: one organisation, one new name, repositories stay where
  they are.
- *What breaks* (all from the pages above): repository web and git URLs
  keep redirecting, but only until someone claims the old name, which GitHub
  frees at once; the Pages sites (`linlog-prover.github.io`, `/linlog/`,
  `/linlog/manual/`, `/rocq-linlog/`, the logos, `llms.txt`) are not
  redirected, so README's logo, the rustdoc's logo (every version of the
  crate on docs.rs that embeds `https://linlog-prover.github.io/logo/...`
  loses it: published 0.1.x and 0.2.0 docs are immutable), every manual
  link in published `.crate` files, the Zenodo records' text and the opam
  fields 404 or point to nothing; the profile URL and the API 404; the
  Cachix cache name is untouched. A squatter on `linlog-prover` could serve
  anything at `linlog-prover.github.io/logo/...`, which the immutable
  published rustdoc would load (inference: this is a supply-chain-style
  risk, not an exploit anyone reported).
- *Cost*: about 1 session of mechanical edits in three repositories (the
  164 + 25 + 36 occurrences), a flake lock refresh in two, the manual
  rebuilt and its link check passed; plus the author's own acts in the
  browser: the rename itself, the Trusted Publishing configuration of two
  crates, Zenodo's app and metadata, README badges. Added trusted base: none.
- *Risk*: the unredirected Pages URLs and the squatting window, as above.
- *Prerequisites*: a name (candidate 4); ideally before the opam submission
  and before the next release.
- *Sources*: sections 1 and 2.

### name-and-organisation-3: Create the umbrella organisation, transfer the repositories, keep `linlog-prover` as a small redirecting organisation

- *What it brings*: the same end state as candidate 2 for the new
  collection, with the old name still owned, so it cannot be squatted.
  Transferring a repository redirects its web and git URLs (transfer page);
  the old organisation can keep a minimal `linlog-prover.github.io` site
  whose pages redirect to the new addresses (inference: a Pages site there
  is allowed since one account can hold one user/organisation site, and
  Pages is not redirected automatically, so the redirect has to be a stub
  with meta refresh or a link; the mechanism was not sourced).
  `linlog` stays the name of the repository and of the two crates; `rocq-linlog`
  moves with its repository name.
- *What must be done by hand*: transfer `linlog`, `rocq-linlog` and the site
  repository (renamed to `<new>.github.io` during the transfer, which
  requires owning the target); re-create the `crates-io` environment and
  re-check secrets if they do not move (inference); edit the Trusted
  Publishing configuration of `linlog` and `linlog-cli`; re-enable Zenodo for
  the new owner (an existing concept DOI cannot be attached to a new
  repository link by the user reports of section 2, but a transfer is said
  to keep the releases; unconfirmed); update the opam file before
  submission; keep the Cachix cache name.
- *Cost*: about 1.5 sessions (the edits of candidate 2 plus a stub site and
  a check that the redirects behave) and the author's browser steps. Nothing
  added to the trusted base.
- *Risk*: the repository's old name is retired for repositories above 100
  clones or Actions uses in the prior week (the page says so for transfer
  too), which is no loss. Redirects die if a repository is created at the
  old location (so none must be). The stub site is a place that must keep
  existing.
- *Prerequisites*: a name; one session's agreement with the web client's
  CI (its flake input).
- *Sources*: GitHub pages in section 2.

### name-and-organisation-4: Choose the umbrella name (the table)

Checks of 2026-11-08, each a read of a public page: GitHub as
`https://github.com/NAME` (HTTP 200 means a user or organisation of that
name exists, 404 means none; GitHub may reserve names not visible this way,
unsourced); crates.io as `https://crates.io/api/v1/crates/NAME` (200 taken,
404 free; the crates.io web page itself is an application WebFetch could not
read); opam as `https://opam.ocaml.org/packages/NAME/` (an exact package
page only, a package with the prefix `NAME-` is not detected; the check
worked on a known package, `dune`); PyPI as `https://pypi.org/pypi/NAME/json`
(optional); domains by the registries' RDAP services
(`rdap.publicinterestregistry.org` for `.org`, `pubapi.registry.google` for
`.dev`; 200 registered, 404 not registered, which is not a guarantee that a
registrar sells it). "n/c" is not checked.

| name | GitHub | crates.io | opam | PyPI | .org | .dev | a well-known thing of that name |
|---|---|---|---|---|---|---|---|
| `linlog` (keep for the linear suite) | taken (a user, "linse") | taken (ours) | free | free | free | free | none found |
| `gentzen` | taken | free | free | free | free | free | Gerhard Gentzen, who invented the sequent calculus: the name fits the field, which is also why a user holds it |
| `apodeixis` | taken | free | free | free | taken | taken | Greek for "demonstration, proof" (unsourced) |
| `syllogos` | free | free | free | free | taken | free | Greek "syllogismos" for syllogism; nothing found |
| `sequentium` | free | free | free | n/c | free | free | nothing found in one search |
| `sequentia` | taken | free | free | taken | taken | taken | a PyPI package; "Sequentia" companies (unsourced) |
| `gentzenlab` | free | free | free | n/c | free | free | nothing found |
| `turnstilelab` | free | free | free | n/c | free | free | the turnstile is the sequent arrow; see `turnstile` |
| `prooflings` | free | free | free | n/c | free | free | nothing found |
| `organon` | taken | taken | free | taken | taken | taken | Aristotle's logical writings; many users of the word |
| `turnstile` | taken | taken | free | taken | taken | taken | a Racket framework for typed languages (POPL 2017) [stchang/macrotypes](https://github.com/racket-tw/macrotypes) |

For comparison `linlog-prover` itself: the GitHub organisation is ours
(200), `linlog-prover.org` and `.dev` are unregistered by RDAP.

Reading the table (inference): the short, classical names (`gentzen`,
`organon`, `turnstile`, `apodeixis`, `sequentia`) are all taken on GitHub as
accounts or organisations, so as an organisation name they are unavailable;
four candidates are free on every platform checked: `sequentium`,
`gentzenlab`, `turnstilelab`, `prooflings`, and `syllogos` is free but for
the `.org`. `sequentium` is the shortest of the free ones and says
"sequents" in a made-up Latin; `gentzenlab` states the lineage. Names that
carry "lab" or "suite" read as an organisation and not as a crate, which
suits an organisation whose crates are `linlog`, `linlog-cli` and the new
suites' names. The final choice is the author's: taste, pronounceability
and the likelihood that "sequent-" is read by someone looking for linear
logic are not checkable. One search (WebSearch) found nothing notable under
`sequentium`, `syllogos`, `gentzenlab` or `prooflings`; that is a weak
check, not a trademark search.

- *Cost*: 0 sessions; the author reserves the name (an organisation on
  github.com is created in the browser, `plan/notes/distribution.md`).
- *Risk*: a name free today is free for no longer than the time it is not
  registered; GitHub's reservation rules and registrars' premium lists were
  not read.

## 4. Considered and rejected

- **Renaming the crates `linlog` and `linlog-cli`**: crates cannot be
  renamed (Cargo's publishing chapter: first come, first served; permanent
  publishes), so the cost is a new crate, a yank and a notice, for a name
  nobody asked to change. The brief of this step wants `linlog` kept for the
  linear suite. Keeping the crate names also keeps Trusted Publishing's crate
  side unchanged.
- **Renaming `rocq-linlog` to another opam name**: the package name has no
  organisation in it; a renamed package is a new package and the pending
  submission is the only free moment, but there is no reason.
- **Registering placeholder crates to reserve names for new suites**: the
  crates.io policy quoted forbids a crate that "exists only to reserve a
  name" (also `plan/notes/distribution.md`).
- **Registering a domain for the web page now**: GitHub's docs say a custom
  domain keeps Pages URLs stable across a rename and that one set on the
  organisation site applies to all project sites; it is a good insurance
  but costs money and a standing commitment, and only the author can do it
  (not recommended here as a precondition; section 5).
- **One monorepo for all suites** to avoid the organisation question: out of
  the scope of this note (the structure is the topic of the other notes of
  this directory).

## 5. What the structure must allow

- Names inside the code carry no organisation name. Where the organisation
  appears is a short list: crate metadata (`Cargo.toml` line 21), the
  rustdoc's logo URLs, README and manual links, `CITATION.cff`, the opam
  file, flake URLs, the workflows' cache and attestation lines, and the
  remap rules of `modules/manual.nix`. A future rename is cheapest if these
  are generated or concentrated (inference): one session could turn the
  repository URL into one place per file type, but the published packages
  stay as they are.
- The logo and site paths are an interface (`.claude/rules/core.md` says
  their paths "must stay stable"); with the site in the organisation's
  `<owner>.github.io`, they are the part a rename breaks hardest, so the
  umbrella organisation should serve them from a host that does not change:
  a custom domain on the organisation site, which GitHub's pages say moves
  every project site under it, or the old organisation's stub.
- Each new repository should carry its own `CITATION.cff` and its own Zenodo
  concept (the library `rocq-linlog` has its own, 10.5281/zenodo.23899109,
  beside linlog's), and its own Trusted Publishing configuration: all three
  are per owner and repository, so they are created once under the final
  organisation, not moved.

## Recommendation (inference)

1. **Decide the umbrella name before more is published**, and prefer
   candidate 3 over candidate 2: create the new organisation, transfer
   `linlog`, `rocq-linlog` and the site repository (renamed
   `<new>.github.io`), and keep `linlog-prover` as a held name with a stub
   site. This avoids the squatting window the organisation-rename page
   warns of, keeps the repositories' web and git redirects, and keeps
   `linlog` as the name of the linear suite, its repository and its crates.
2. **Do it before the opam submission is merged**, indeed before it is
   submitted: the three-line edit of `homepage`, `bug-reports`, `dev-repo` and
   `doc` in the prepared commit and the same in `rocq-linlog.opam` need
   neither a rename nor a deprecation, whereas after the merge no source I
   read says how a released package's fields are changed.
3. **By hand, after the transfer** (the author's own acts, each asked
   before): edit the Trusted Publishing configuration of `linlog` and
   `linlog-cli` on crates.io; re-enable Zenodo for the new owner and edit
   the records' metadata; check that the `crates-io` environment and the
   Cachix secret exist; keep the Cachix cache name `linlog-prover`; refresh
   the two `flake.lock` files.
4. The next release after the move proves the whole chain (the workflow's
   `check`, `publish`, `release` jobs and the Zenodo hook). The 0.2.0
   artefacts keep the old URLs and work through GitHub's redirects until
   the old name is reused, which is why the old organisation should stay.
5. If the author prefers not to rename, candidate 1 costs nothing today and
   no source read says the old name will not do; the cost grows with every
   release.

Cost summary: about 1.5 sessions of repository work (edits in three
repositories, a stub site, the manual's link check and the gate) and a list
of browser tasks for the author; no change of the trusted base.

## 6. Sources

All read 2026-11-08.

1. GitHub, Renaming an organization:
   https://docs.github.com/en/organizations/managing-organization-settings/renaming-an-organization
2. GitHub, Transferring a repository:
   https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository
3. GitHub, Renaming a repository:
   https://docs.github.com/en/repositories/creating-and-managing-repositories/renaming-a-repository
4. GitHub, About GitHub Pages:
   https://docs.github.com/en/pages/getting-started-with-github-pages/about-github-pages
5. GitHub, About custom domains and GitHub Pages:
   https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site/about-custom-domains-and-github-pages
6. crates.io policies (through a search result's quotation only):
   https://www.crates.io/policies
7. Cargo book, Publishing on crates.io:
   https://doc.rust-lang.org/cargo/reference/publishing.html
8. RFC 3691, Trusted publishing for crates.io:
   https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html
9. crates.io pull request 12049 (the fields of a trusted publishing
   configuration):
   https://github.com/rust-lang/crates.io/pull/12049/files
10. docs.rs, About: https://docs.rs/about
11. Zenodo, enabling a repository:
    https://help.zenodo.org/docs/github/enable-repository/
12. Zenodo, linking accounts:
    https://help.zenodo.org/docs/profile/linking-accounts/
13. Zenodo issue 826 (user reports, anecdotal):
    https://github.com/zenodo/zenodo/issues/826
14. Rocq opam repository README: https://github.com/rocq-prover/opam
15. opam-repository archiving policy:
    https://github.com/ocaml/opam-repository/blob/master/governance/policies/archiving.md
16. Cachix documentation: https://docs.cachix.org/ and
    https://docs.cachix.org/getting-started.html
17. Availability pages, one read each: `https://github.com/NAME`,
    `https://crates.io/api/v1/crates/NAME`,
    `https://opam.ocaml.org/packages/NAME/`,
    `https://pypi.org/pypi/NAME/json`,
    `https://rdap.publicinterestregistry.org/rdap/domain/NAME.org`,
    `https://pubapi.registry.google/rdap/domain/NAME.dev`, for the names of
    the table.
18. Turnstile (Racket): https://github.com/racket-tw/macrotypes
19. In the repository: `Cargo.toml`, `core/src/lib.rs`, `CITATION.cff`,
    `.github/workflows/release.yml`, `.github/workflows/install.yml`,
    `docs/book.toml`, `modules/manual.nix`, `.claude/rules/ci.md`,
    `.claude/rules/core.md`, `plan/notes/distribution.md`,
    `plan/reports/45-release-0.2.0.md` (45e, Zenodo), `../rocq-linlog/rocq-linlog.opam`,
    `../linlog-prover.github.io/astro.config.ts`.

## 7. Not verified

- crates.io's own pages (policies, trusted publishing, FAQ) could not be read
  (single-page application); the squatting and transfer policy comes from a
  search result's quotation, and what a rename does to a Trusted Publishing
  configuration, and whether matching uses a numeric id, is not in any
  page read.
- Whether Zenodo's integration keeps archiving after a transfer to another
  owner, and how to edit records' metadata: no Zenodo page read says;
  only anecdotes.
- Whether nix follows GitHub's redirect for `github:OLD/linlog` (already
  open in `plan/notes/distribution.md`).
- Whether the `crates-io` environment, secrets and the Cachix token move with a
  transferred repository (the transfer page names secrets, webhooks and deploy
  keys, not environments).
- How a released opam package's metadata is corrected, or a package renamed,
  in the Rocq opam repository.
- Whether the old `github.io` address redirects after a custom domain is
  set, and whether a stub site in the old organisation is the right
  mechanism for redirects.
- Whether Cachix can rename a cache.
- Domain availability is RDAP "not registered", not a registrar's price or
  premium list; GitHub's reserved or formerly used names are not visible
  through `github.com/NAME`; opam's check covers exact names only; PyPI was
  not checked for several names; the "well-known project" column rests on
  one or two searches and common knowledge, not on a trademark search.
- Whether the web search names ("Sequentia" companies, "apodeixis" meaning)
  are right: unsourced.
- The counts of occurrences are from `grep` over the three workspaces and
  include history under `plan/`.
