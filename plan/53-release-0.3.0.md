# Step 53: the release of 0.3.0, the first from `sequentium`

You are working in linlog (`sequentium/sequentium`) and, for its release,
in the Rocq library (`../rocq-linlog`, `sequentium/rocq-linlog`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. Tagging and publishing are the author's acts. The
step has two sessions and comes after steps 47 to 52 are done, reviewed
and on `main`. This prompt was written on 2026-11-08 by step 46's second
session, from the author's decisions of that day. It is complete; the
planning session adds what steps 47 to 52 leave at their reviews.

Read `plan/45-release-0.2.0.md` and its report (how 0.2.0 was made: the
release prepared, the author's acts last, the Rocq library's release
beside it, Zenodo), `plan/notes/distribution.md`, `.claude/rules/ci.md`
(the release workflow, Trusted Publishing), `CHANGELOG.md`, the reports
of steps 47 to 52 (and 48's table of addresses), and the `later.md`
entries named below.

**Held (the author, 2026-11-08): "Let's figure out a name and logo
later. In the meantime don't make another tag until I figure it out."**

- No tag until the author has chosen the name and the mark:
  - the name: the rename to `sequentium` and its form, or none, as
    weighed in `plan/notes/structure.md`;
  - the mark: one of 53m's three rounds.
- 53m's placement and 53b wait for those choices.
- 53a needs no tag and may run.
- If the name changes, 53b's question on the citation title is answered
  by it, and a session renaming the crates comes before 53b.

**Settled (the author, 2026-11-09).**

- **The name.** The name is **deduxit**. The organisation, repositories
  and sites were renamed that night, and session 53n moved every address.
- **The scope: the idiomatic recommendation.**
  - one command `deduxit` with `--logic`;
  - one crate `deduxit` with the linear suite under `deduxit::linear`;
  - `deduxit-cli` and `deduxit-web`;
  - final "renamed" releases of `linlog` and `linlog-cli`;
  - the Rocq library as `rocq-deduxit` (logical path `Deduxit`, today's
    content as `Deduxit.Linear`).

  `plan/notes/cli-library.md` and `plan/notes/structure.md` hold the
  reasons.
- **The citation title.** The concept DOIs stay. From 0.3.0 the title
  is the new name, "deduxit" and "rocq-deduxit".
- **The mark.** The author prefers the macron ("dēdūxit", the long
  marks as inference lines). Its variations of face and colour are
  being drawn, and the author chooses among them.

**The sessions, in order** (each Opus 5.5 at `high`, each prompt
written by the planning session when it starts):

1. **53n, the move of the addresses** (running when this was written).
2. **53a, the small defects**, as below. It runs beside 53r, which is
   in another repository.
3. **53r, the Rocq library renamed**, in `../rocq-linlog`:
   - the package `rocq-deduxit`;
   - the logical path `Deduxit`, today's modules under `Deduxit.Linear`;
   - the opam file, README, the citation and the flake;
   - its own `nix flake check`.

   The GitHub repository is renamed to `deduxit/rocq-deduxit` first,
   with the author's yes.
4. **53s, the crates and the command renamed**, after 53a:
   - the packages `deduxit` (core), `deduxit-cli` (binary `deduxit`),
     `deduxit-web` and `deduxit-bench`;
   - the linear modules moved under `deduxit::linear`, with what
     `ordinary` already shares at the root;
   - the final shim crates `linlog` and `linlog-cli` 0.3.0, which
     re-export or call the new crates;
   - the exporter's preludes and `--kernel linlog` follow the renamed
     library, and the lock moves to it;
   - the manual, the guide for AI models, the behaviour lock (a commit
     of its own), the release workflow's crate list, `.claude/` and
     CLAUDE.md.

   The target sets must equal 47's (a move of modules changes no
   search) and the ratchet must hold. In the client, after linlog: the
   bindings' new name and the lock.
5. **53m's placement of the chosen mark.**
6. **53b, the release prepared.** The author publishes the new crates
   once with a token, then sets up their Trusted Publishing.

## Goal

0.3.0 is "ordinary logic on the page": LK and LJ step by step in the
command and on the page (step 49), proofs read back as lambda-terms (50),
two more kinds of refutation checked and certified (51), the inverse
method's rows (52), all from the new home (48). It is a 0.y bump because
new variants of closed enums are planned breaks (`core.md`,
"Decisions"); the release says each break plainly. Its second purpose is
to prove the chain after the move: crates.io's Trusted Publishing from
`sequentium/sequentium`, the GitHub release, and Zenodo's version records
joining (or not) the concept DOIs.

## Session 53m: a mark for `sequentium`, before 53a

The author's decision of 2026-11-08: the collection gets a logo and a
word mark of its own before 0.3.0, the first release from `sequentium`.
linlog's lollipop (violet, CC BY 4.0, sessions 38f and 38g) stays the
linear suite's mark (corrected the same day: the author retires the lollipop, and the new mark replaces it everywhere; `linlog` stays the linear suite's name without a mark of its own). The organisation's avatar waits for the new mark.
Two parts, as 38f and 38g were:

- **The design** (Fable 5.1 at `high`, with the `frontend-design`
  plugin, after step 49 has put the suite tabs on the page):
  - Candidates for `sequentium`'s mark, each a word mark and a square
    form, related to the lollipop as a family's mark is to a member's.
    The report of 38f in `plan/reports/38-web-polish.md` and the files
    in `~/Projects/own/linlog-logo/` show how the lollipop was made and
    judged.
  - The author's idea, to weigh first but not alone (2026-11-08: "I was
    thinking about the 't' becoming the turnstile, but not sure. Also
    explore other options"): the "t" of `sequentium` drawn as the
    turnstile `⊢`, as linlog's "i" is the lollipop `⊸`. Other options
    beside it, each judged by the same criteria.
  - **Two designers who exchange ideas** (the author, 2026-11-08: "I
    would like to have Fable and Opus both come up and exchange
    ideas"): the session, on Fable 5.1, starts a design partner on Opus
    5.5 as a sub-agent (named `linlog-mark-opus`) and keeps talking to
    it with its context. There are three rounds:
    1. Apart: each designs three to five candidates from the same brief
       without seeing the other's, each drawn as SVG with its reason.
    2. Exchange: each gets the other's set, critiques it, says what it
       would borrow and proposes refinements or hybrids; a second
       exchange if they are still converging.
    3. Together: one shortlist, each candidate labelled by who proposed
       it and where the other disagreed.
    The report keeps both rounds' sets.
  - Each candidate is shown at every size and place it will take (the
    favicon, the page's header beside the suite tabs, the social image,
    a GitHub avatar, README's head on GitHub and crates.io) in a private
    artifact for the author.
  - Its parts are checked as 38f's were: the font's licence, no clash
    with a known mark of the field, and the trademark searches the
    author may run by hand.
  - The author chooses. The colour becomes a token of its own, as the
    lollipop's is, under the client's colour-vision check.
- **The placement** (Opus 5.5 at `high`, in the client and in this
  repository):
  - the client's favicon, icons, header, social image and the page's
    name;
  - README's head and the rustdoc's logo, with the logo files, under
    the licence the author picks, served from the site as the
    lollipop's are;
  - the lollipop retired everywhere (the author, 2026-11-08), the new
    mark in its place, the client's `--color-logo` token and its guards
    moved to the new mark's colour;
  - the author's checklist: the organisation's avatar and the
    repositories' social previews, which only the browser can set.

## Session 53a: the small defects, before the release

Opus 5.5 at `high`. The open defects of `plan/later.md` that a first user
meets, each a small fix with one focused test, and each closed in
`later.md` with the commit; one that turns out larger is kept there with
its reason:

- the command: `linlog check FILE` ignores the file's own `"mode"`;
  `check -i --mix --net` prints "proof net with Mix" without checking the
  mode; `eliminate --net --output FILE` without `--format` writes
  nothing; `linlog eliminate` names a cut by its raw id (`cut on #8`)
  and has no flag to name it; `interact`'s `cut G C P…` reads the formula
  as one word; the eigenvariable refusal message at `∃L` in the
  two-sided reading; `--style labels.table.RULE=LABEL` fails on an unset
  table and the rule keys are not listed; `check --file` (`check` takes
  the proof as an argument only);
- the library: the division by zero in
  the rules look's label narrowing at a zero-width advance; the essential
  net's input side drawn green by `svg::Style`'s default, and its drawing
  turned off for nets with cut links; the panic past the proof
  structure's vertex bound (`nets::MOST`), made a refusal in
  `Nets::admits` with its test; the batch's and `check`'s records
  writing `wire::LEVEL` as a constant; the derivation builder's
  recursion to the derivation's height; the unused `Member` import in
  `search/focus/tests.rs` without `parallel`; the test
  `a_raised_limit_holds_on_the_pool` overflowing the test thread's stack
  in a debug build;
- the documents: the library's front page whose examples read `Instant`,
  and where the Euler Math font's source is said; the Rocq entries of
  "Follow-ups: a first user's obstacles" (a refused certificate losing
  the verdict, the batch's certificate file names, applying a lemma to a
  two-sided goal), each checked against today's code first;
- the older lists that name steps long done (the exports' lists of steps
  11 and 22, the command's output's of steps 18 and 20, the interactive
  session's of steps 22, 28 and 34: the reading recomputed per
  operation, the filtered `rules` list, a per-goal budget for
  `close_all`, the map from a drawing's inferences): each entry checked
  against the code and closed as done or kept with its reason.

## Session 53b: the release prepared

Opus 5.5 at `high`, after 53a. As 0.2.0's: `breaks 0.2.0` (both tools)
and every break listed in `CHANGELOG.md`; the version raised in both
repositories (the library's 0.3.0 if step 51 changed it, its own
changelog); the command's third-party notices in the packages
(`later.md`, "Follow-ups: the release's licences"), or added; the
manual's `status.md` true; the guide for AI models
within its budget; `nix flake check` in both; the release workflow's
dry run as 0.2.0's was; the GitHub release notes drafted; the Zenodo
check written down for the author: after the tag, whether the new
version record sits under linlog's concept DOI 10.5281/zenodo.23448588
(and the library's under 10.5281/zenodo.23899109) or started a new
concept, and what to do if it did. Before that, a question for the
author with a recommendation (they asked on 2026-11-08 whether linlog
can be renamed): what the release is called in citations. Zenodo titles
a version from `CITATION.cff` (`title: linlog` after step 48), and
crates.io renames no crate. So either the concept DOI stays "linlog",
the linear suite, and the collection gets a record of its own later; or
the title changes from 0.3.0 on, for example to "sequentium: logic
suites (linlog, …)", in `CITATION.cff` and on Zenodo. 0.2.0's records
keep their title. Then the author tags and publishes.

## How the step runs

Session 53m first (its design on Fable 5.1 at `high`, the author
choosing, its placement on Opus 5.5 at `high`), then 53a and 53b on
Opus 5.5 at `high`, reviewed by the planning session between them; no benchmark (the engines changed only by step 52's rows,
whose measurement stands). Deliverables: the commits, the draft release
notes, the author's checklist (tag, crates.io through the workflow,
GitHub release, Zenodo, the library's tag), and
`plan/reports/53-release-0.3.0.md`.
