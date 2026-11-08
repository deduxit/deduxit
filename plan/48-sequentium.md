# Step 48: the move to `sequentium`

You are working in the linlog repository. CLAUDE.md applies throughout: jj
only (never git), thematic commits as soon as a unit is done, no pushing,
no `gh`. The step changes three repositories and prepares a fourth, and
the author acts between its two sessions: creating, transferring and
renaming on GitHub and configuring crates.io and Zenodo are theirs alone,
never a session's. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day. It is complete.

Read `plan/notes/research/next/name-and-organisation.md` whole (what a
move touches at GitHub, crates.io, Zenodo, opam, Cachix and Nix, each
from the service's own page, and what could not be sourced),
`plan/notes/distribution.md`, `plan/README.md`'s D22 and D26,
`.claude/rules/ci.md` (the release workflow, Trusted Publishing, the
cache), `.claude/rules/flake.md`, `.claude/rules/web.md`, the section
"Session 45e" of `plan/reports/45-release-0.2.0.md` (the opam
submission, prepared and not sent), and the CLAUDE.md files of the Rocq
library (`../rocq-linlog`) and of the web client
(`../linlog-prover.github.io`).

## What the author decided (2026-11-08)

- **The collection's home is the organisation `sequentium`** (Latinised
  from Gentzen's *Sequenz*, "of sequents"). The author created it on
  2026-11-08; it is empty.
- **One shared repository for every suite** (D25): `linlog-prover/linlog`
  moves there and is renamed `sequentium/sequentium`. The crates
  `linlog`, `linlog-cli`, `linlog-web` and the command `linlog` keep their
  names: `linlog` is the linear suite.
- **The Rocq library and the site move too**: `rocq-linlog` keeps its
  name (`sequentium/rocq-linlog`, opam package `rocq-linlog`, logical
  path `Linlog`); the site repository `linlog-prover.github.io` becomes
  `sequentium/sequentium.github.io`.
- **`linlog-prover` stays as a placeholder** that the author keeps
  owning, so the name cannot be claimed by someone else, and its site
  redirects the published URLs to their new places.
- **All of this before the Rocq library's opam pull request**, which the
  author sends after the move (decision 9); crates.io gets no
  placeholder crate (its usage policy forbids squatting).

## What a move does, and what it does not (from the note's sources)

- A transferred repository's web and git URLs redirect, until a
  repository is created at the old location: so **no repository named
  `linlog` or `rocq-linlog` is ever created under `linlog-prover` again**.
  The site repository is the exception the plan accepts: the placeholder
  site needs a repository `linlog-prover/linlog-prover.github.io`, which
  ends the git redirect of the old site repository (whose only clone is
  the author's checkout, whose remote this step updates).
- GitHub redirects no Pages URL ("we don't redirect GitHub Pages
  associated with the repository"). Every address published under
  `linlog-prover.github.io` must therefore be answered by the
  placeholder site: the page itself (`/`, with its query and fragment,
  which carry a shared proof), the manual (`/linlog/manual/…`), the
  rustdoc (`/linlog/…`), the Rocq library's documentation
  (`/rocq-linlog/…`), the logos (`/logo/…`, embedded by the immutable
  rustdoc of every published version on docs.rs, so they are served as
  files, since an image does not follow a redirect), `llms.txt` and
  `llms-full.txt` (served as files with a first line naming the new
  address), and every other path by a `404.html` that maps the old
  prefix to the new one.
- The project site of a repository is served at
  `<owner>.github.io/<repository>/`: the rename moves the manual and the
  rustdoc from `/linlog/` to `/sequentium/` unless the step lays them
  out otherwise. Choose the new addresses once, with the reason, and
  list them; a custom domain is the author's to choose (open with them;
  if they have chosen one before the step starts, use it, otherwise the
  `github.io` addresses).
- crates.io's Trusted Publishing is configured per crate by repository
  owner, repository name, workflow and environment, so it must be edited
  by a crate owner before the next release; published packages keep
  their old repository URL, which GitHub's redirect answers. The
  `crates-io` and `github-pages` environments, the rulesets and the
  Pages source may or may not move with a transfer (the transfer page
  lists secrets, not environments): the author checks each after the
  transfer.
- Zenodo's GitHub integration must be enabled again for the new owner;
  the DOIs (linlog's concept 10.5281/zenodo.23448588, the library's
  10.5281/zenodo.23899109) stay valid, and whether a release from the
  moved repository joins the existing concept record is not sourced:
  0.3.0's release checks it (step 53).
- The Cachix cache keeps its name `linlog-prover` (no page read says a
  cache can be renamed; its name is independent of GitHub).
- The local checkouts keep their paths (`~/Projects/own/linlog`,
  `../rocq-linlog`, `../linlog-prover.github.io`): the Claude Code
  settings, memory and scripts are keyed by them. Only their remotes
  change.

## Session 48a: everything prepared, nothing pushed

Opus 5.5 at `high`, in jj workspaces of the three repositories based on
their `main`. Requirements:

1. **Every reference to the old addresses**, outside `plan/` (whose
   history keeps them): the counts of the note (164 in this repository,
   25 in the library, 36 in the client) re-taken with `grep` and each
   occurrence changed or kept with its reason. The load-bearing ones:
   `Cargo.toml`'s `repository` (the crates inherit it) and `homepage` if
   any; `core/src/lib.rs`'s logo, favicon and `llms` URLs;
   `docs/book.toml`'s `git-repository-url` and `edit-url-template`;
   `CITATION.cff` in both repositories; README; the manual's chapters
   (installation, introduction, the Rocq chapter, output formats,
   development, status); the guide for AI models (`skills/linlog/`,
   which `cli/tests/readme.rs` reads); `modules/manual.nix`'s link-check
   remaps; the workflows (`release.yml`'s attestation comment and user
   agents, `install.yml`'s flake reference, release API call and
   download URL; the cache's `name: linlog-prover` stays);
   `flake.nix`'s input `rocq-linlog` (`github:sequentium/rocq-linlog`)
   and the client's input on `github:sequentium/sequentium`;
   `rocq-linlog.opam`'s `homepage`, `bug-reports`, `dev-repo`, `doc`;
   the client's `astro.config.ts` `site`, `package.json`, privacy page,
   `llms.txt` and the logo licence; CLAUDE.md, `.claude/` (rules, the
   hooks' messages) and `plan/push-main.sh`'s comments where they name
   the remote. The locks move only after the push (48b).
2. **The front page**: README and the manual's introduction say in one
   sentence that the repository `sequentium` is a collection of logic
   suites whose linear suite is linlog (the crate and command keep the
   name), and `docs/src/status.md`'s planned list follows the roadmap of
   0.3.0 to 0.7.0 (D27), without naming the plan (the `conventions`
   check fails on it). The crates' descriptions on crates.io do not
   change.
3. **The placeholder site**, prepared as a new jj repository in a
   directory of your scratchpad (not created on GitHub): a static site
   with a Pages workflow in the client's style, the redirect pages, the
   logos and `llms` files copied, a `404.html` mapping every old prefix,
   a README saying where everything went, the licence header where the
   conventions need one. Its every redirect is listed in a table of old
   and new addresses in the report, the source of 48b's checks.
4. **The opam submission**: in `~/Projects/own/rocq-opam` (a jj clone of
   `rocq-prover/opam`, its commit `40954aee` "Add rocq-linlog.0.2.0"),
   the four fields moved to the new addresses in a new change on top,
   which the author squashes or keeps; the `url`'s SHA-512 is checked
   again in 48b against the moved repository's archive.
5. **The author's acts, in order**, written into the report as a
   checklist they can follow in a browser (below), with what to check
   after each.
6. **Checks**, in each repository: its `nix flake check` (in linlog under
   the machine lock, `flock -o /tmp/linlog-machine.lock nix flake check
   --max-jobs 2 --cores 6`, in a capped scope on cores 2 to 7), which
   includes `manual-links`, `conventions` and `typos`; the client's own
   checks. A link to a new address that does not exist yet is checked
   by the remap rules, not fetched.

The commits stay in the workspaces, unpushed; the planning session
stacks them and pushes after the author's acts (48b).

## The author's acts, between 48a and 48b

1. *(done 2026-11-08)* Create the organisation `sequentium`.
2. In `sequentium`'s settings, mirror `linlog-prover`'s: two-factor
   authentication required, Actions allowed with read-only default
   workflow permissions, Pages allowed, base permissions for members.
3. Transfer `linlog-prover/rocq-linlog` to `sequentium` (name kept).
4. Transfer `linlog-prover/linlog` to `sequentium` and rename it
   `sequentium`.
5. Transfer `linlog-prover/linlog-prover.github.io` to `sequentium` and
   rename it `sequentium.github.io`.
6. On each moved repository, check and re-create what did not move: the
   Pages source (GitHub Actions), the `github-pages` and `crates-io`
   environments with their protection rules, the secret
   `CACHIX_AUTH_TOKEN`, the branch rulesets, the topics, Dependabot.
7. On crates.io, for `linlog` and for `linlog-cli`: replace the Trusted
   Publishing configuration with owner `sequentium`, repository
   `sequentium`, workflow `release.yml`, environment `crates-io`.
8. On Zenodo: grant the Zenodo application access to `sequentium` on
   GitHub, switch on `sequentium/sequentium` and
   `sequentium/rocq-linlog`, and edit the two concept records' metadata
   (the repository URL).
9. Create the public repository `linlog-prover/linlog-prover.github.io`
   and push the placeholder site 48a prepared (its commands in the
   report), with Pages from GitHub Actions. Never create a repository
   named `linlog` or `rocq-linlog` under `linlog-prover`.
10. Tell the planning session, which starts 48b.

After 48b: the author's opam pull request (45e's steps, from the moved
change), and the next release (step 53) proves the chain: Trusted
Publishing, the GitHub release and Zenodo's new version records.

## Session 48b: the move finished and checked

Opus 5.5 at `high`, after the author's acts. Requirements:

1. The three checkouts' remotes moved (`jj git remote set-url origin …`
   in each), and the push order handed to the planning session: the
   library first, then this repository with its lock moved to the new
   library input (`nix flake update rocq-linlog`, "flake.lock: Update"),
   then the client with its lock moved to `github:sequentium/sequentium`.
2. After the pushes and the Docs workflows: every row of 48a's table of
   addresses fetched (`curl -sSIL`, a few requests, never in a loop
   against GitHub's API): the old address answers with the placeholder,
   the placeholder sends to the new address, the new address answers.
   The release archive of `v0.2.0` at its new address hashed against the
   opam file's SHA-512.
3. `nix flake check` in each repository at its pushed head, and `nix
   run github:sequentium/sequentium -- --version` from a clean shell.
4. What failed or could not be checked said in the report, with what the
   author or step 53 must do about it.

## How the step runs

Two sessions on Opus 5.5 at `high` with the author's acts between; a
fresh-context reviewer on Opus 5.5 at `high` in 48a reads the table of
addresses and the diff against the note's list of touch points, and
reports only an address that would break or a reference missed.
Deliverables: the commits in the three repositories and the placeholder
site's repository, the opam change, `plan/reports/48-sequentium.md` with
the table of addresses, the author's checklist and "What the next
sessions must know" (the new addresses every later prompt uses).
