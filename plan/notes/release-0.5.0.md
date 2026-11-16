# The release of 0.5.0: notes, commands and checks

Written by session 64 (2026-11-16) for the planning session and the
author. The tag is `v0.5.0` in `deduxit/deduxit` alone: the Rocq library
has not moved since its `v0.4.0` (`ef979655`, which the flake pins), so
it gets no release, and deduxit 0.5.0's certificates name rocq-deduxit
0.4.0 (`export::rocq::DEDUXIT`). The author's yes for the tag is of
2026-11-16 (`plan/64-release-0.5.0.md`).

## What 0.5.0 is

"Custom rules and natural deduction" (D27): steps 60 to 63 with 63t and
63g, and on the page 61b's calculus of one's own, 62d's natural
deduction, 63t-b's translations and interpolants, 63g-c's linked tabs,
and 63b's and 63c's Learn and Practice by suite. The changelog's first
paragraph names the page's part, so the release holds until 63g-c's
page is on the client's `main` with the client's input `deduxit` moved
to this chain.

## The GitHub release notes

The release workflow writes the notes from the tagged tree, so the
draft at the end of this file is the text it will produce, generated
from the changelog at the commit to tag by the workflow's own `awk`:

- **deduxit** (`.github/workflows/release.yml`, job `release`): the
  title `deduxit 0.5.0`; the notes are `CHANGELOG.md`'s section
  `## [0.5.0] - 2026-11-16` (to the next `## [`), then the line naming
  the manual's Installation chapter and `SHA256SUMS`. Attached:
  `deduxit-0.5.0.crate` and `deduxit-cli-0.5.0.crate` from
  static.crates.io, the archives and Linux packages of `packages.yml`,
  and, new in this release, `wire.schema.json` and `rules.schema.json`
  (the rule language's schema, `deduxit::rules::SCHEMA`), all in
  `SHA256SUMS`, each attested.

A change of the text is a change of the changelog before the tag;
after it, `gh release edit v0.5.0 --notes-file …` (an administrative
act, the author's yes first).

**The date.** The changelog and `CITATION.cff` say 2026-11-16. If the
tag is made on another day, the planning session moves both to that day
before tagging (the workflow checks only that the section is dated and
the citation states the version).

## What a user can do now

What the release notes' first paragraph sums up, for an announcement:

- Write a calculus as a rule file in JSON (connectives, the zones of a
  sequent, rules over schematic formulas and contexts) and check a
  derivation in it: `deduxit check --calculus rules.json proof.json`,
  or a built-in calculus by name (`LK`, `LJ`, `LM`, `LL`, `tableau`).
  The verdict reads "valid in the calculus given". The rule language's
  JSON Schema is `deduxit::rules::SCHEMA` and the release's
  `rules.schema.json`.
- Build a derivation in such a calculus step by step:
  `deduxit interact --calculus NJ "…"`, backwards with `apply` and
  `assume`, forwards with `derive`, each step checked.
- Prove in natural deduction for classical, intuitionistic and minimal
  logic (`NK`, `NJ`, `NM`, and `NK-DNE`, `NK-LEM`, `NJ-defined`,
  `NK-defined`, `NM-defined`), each checked to derive exactly what the
  ordinary suite calls valid.
- Draw such a derivation as a textbook's tree (discharged hypotheses in
  brackets, numbered) or as a Fitch proof: `check --calculus …
  --layout fitch --format latex|typst|svg`, for the LaTeX package
  `fitch` and the Typst package `fitchcraft`.
- Translate a classical sequent into intuitionistic or minimal logic
  (`deduxit seq translate`, Kolmogorov, Gödel–Gentzen, Kuroda,
  Glivenko), with both verdicts decided and compared, and find a
  checked Craig interpolant (`deduxit interpolate`).
- Keep a sequent and what was made from it in a workspace file
  (`deduxit workspace`), with out-of-date marking, `run`, `undo` and
  `redo`, and each operation's promise checked (`run --checks`).
- In the library: `deduxit::rules`, `ordinary::negative`,
  `ordinary::interpolant`, `deduxit::protocol` (what each suite offers,
  run over JSON) and `deduxit::workspace`.
- On the page: natural deduction and a calculus of one's own on the
  ordinary tab, the translations and interpolants, linked tabs in a
  workspace, and Learn and Practice by suite, with a six-lesson course
  for ordinary logic.

## Before the tag

- 63g-c's page is on the client's `main`, and the client's input
  `deduxit` is moved to deduxit's `main` once this chain is pushed (until
  then the client builds only with the `git+file` override).
- `ci.yml` is green on `main` at the commit to tag, and `kani.yml`'s
  last run on `main` is green (`gh run list --repo deduxit/deduxit
  --workflow kani.yml`, a read).
- The baseline of 2026-11-16 (`bench/results/2026-11-16/`,
  `bench/RESULTS.md`) is the latest; no other timed run is needed for
  this release.

## After the tag

- The publish job asks the author's approval in the environment
  `crates-io`. `deduxit` and `deduxit-cli` exist on crates.io with
  their trusted publishers, so the job publishes both through Trusted
  Publishing (`crates-io-auth-action`); `linlog` and `linlog-cli` are
  skipped, since crates.io has their 0.3.0; no secret is used (the
  selection, run locally on 2026-11-16: `new=` none, `existing=deduxit
  deduxit-cli`).
- The release job attaches the two schemas; check that the release
  lists `wire.schema.json` and `rules.schema.json` and that
  `SHA256SUMS` names them.
- The planning session moves the client's input `deduxit` to the tagged
  commit, and checks the live site and docs.rs.
- The Zenodo check, as for 0.3.0 (`plan/notes/release-0.3.0.md`,
  "After the tags: the Zenodo check"), with `v0.5.0`: a record titled
  `deduxit` under 10.5281/zenodo.23448588. rocq-deduxit makes no new
  record.
- **opam waits** (the author, 2026-11-09, until 0.8.0 or later).

## The author's checklist

Before the tag:

- [ ] Nothing to set: the trusted publishers of `deduxit` and
  `deduxit-cli` name `deduxit`/`deduxit`/`release.yml`/`crates-io`, and
  no secret is needed. Confirm on crates.io that both still list
  theirs.
- [ ] Say yes to the tag once CI is green at the release commit (given
  on 2026-11-16 for 0.5.0).

After the tag (the planning session makes it, a signed tag object):

- [ ] Approve the job `cargo publish` in the environment `crates-io`.
- [ ] The Zenodo check above, if the planning session has not reported
  it.

## The notes as the workflow will write them

````markdown

Calculi given as data, and natural deduction. A rule file in JSON
declares a calculus: its connectives, the zones of a sequent and its
rules. deduxit checks a derivation in it, builds one step by step,
backwards from a goal or forwards from numbered lines, and draws it.
Natural deduction for classical, intuitionistic and minimal logic is
built in (NK, NJ, NM and the variants courses use), drawn as a
textbook's tree or as a Fitch proof in LaTeX, Typst and SVG. Classical
sequents are translated into intuitionistic and minimal logic by the
negative translations, and Craig interpolants are read off derivations;
each result is checked. `deduxit::protocol` lists what each suite offers
as data, with a JSON Schema of every form, and a workspace keeps a
sequent and what was made from it, with undo and redo. On the web page
the ordinary tab proves in natural deduction and in a calculus of your
own, translates and interpolates, and links the tabs made from one
sequent; Learn and Practice follow the tab's suite, with a course of six
lessons for ordinary logic. Each release now carries the JSON Schemas of
its version, `wire.schema.json` and `rules.schema.json`.

### What breaks, and what to do instead

Nothing that 0.4.0 offers was removed or changed: its calls, types,
command lines and JSON forms are as they were, and every new variant or
field belongs to a `#[non_exhaustive]` type. What a program may notice
is under "Changed".

### Changed

- **A default build of the library compiles `serde_json`**, which the
  new default feature `protocol` needs. To leave it out, turn the
  default features off and name those you use.
- **Thirteen new error codes** (`Error::code`): `invalid_calculus`,
  `invalid_derivation`, `zones`, `step_refused`, `step_needs`,
  `not_fitch`, `negative_target`, `claim_fails`, `not_cut_free`,
  `partition`, `workspace_limit`, `not_applicable` and `not_given`. Only
  the new calls answer them.
- **`deduxit` without a command lists `interpolate` and `workspace`**
  beside the others, and the help of `check`, `interact`, `prove` and
  `seq print` names the new flags and the style `fitch`.
- **0.4.0 refuses what 0.5.0 writes at wire levels 15 to 19**, naming
  the level (`unsupported_version`), and a settings file that sets one
  of the new keys (`limits.workspace`, `view.layout`, the style
  `fitch`).

### Added

- **Natural deduction drawn as textbooks draw it**: in a tree each
  hypothesis stands alone at a leaf, the ones a rule discharges in
  brackets with a number that the rule carries (`[a]¹` … `→I¹`), in
  LaTeX, Typst and SVG. **A Fitch proof** of any derivation in a
  calculus given as data (`rules::Fitch`): numbered lines, each
  subproof's lines together under its hypotheses, a rule citing lines
  and subproofs (`→I 1–6`), written as text, as SVG, for the LaTeX
  package `fitch` and for the Typst package `fitchcraft`.
  `check --calculus` draws both (`--format latex|typst|svg`, `--layout
  fitch`, the settings key `view.layout`, the style `fitch`); a
  derivation whose lines cannot be put in Fitch's order is refused with
  the reason (`Error::NotFitch`). A session's Fitch proof
  (`rules::Interactive::fitch_derivation`) opens every subproof with its
  hypotheses, those no step has used yet too, and says which Fitch line
  shows each of the session's lines; `Drawing::nodes` and `Fitch::nodes`
  say which node each inference or line shows.

- **A calculus given as data** (the library, `deduxit::rules`): a rule
  file in JSON (connectives, the zones of a sequent, rules as premises
  and a conclusion over schematic formulas and contexts), a derivation
  whose steps name its rules, and `Derivation::check`, which matches
  every step against its rule. The verdict is "valid in the calculus
  given". Hypotheses, discharge and nested scopes serve natural
  deduction; sorts serve signed and labelled formulas. LK, LJ and
  minimal logic's calculus come as rule files, and
  `Derivation::from_ordinary` writes a derivation of LK or LJ in this
  form. Both documents are at wire level 15.
- **`check --calculus CALCULUS`** checks such a derivation against a
  rule file, or a built-in calculus by name (`LK`, `LJ`, `LM`, `LL`,
  `tableau`); a derivation that names a built-in calculus needs no
  flag. The verdict line reads "valid in the calculus given", and the
  derivation's nodes follow as a list indented by scope.
- **Natural deduction** as built-in calculi: `NJ`, `NK` and `NM` for
  intuitionistic, classical and minimal logic, and the variants
  `NK-DNE`, `NK-LEM`, `NJ-defined`, `NK-defined` and `NM-defined`, each
  rule file naming its sources (`rules::NJ`, `rules::NK`, … in the
  library). `check --calculus NJ` and `interact --calculus NJ` take
  them by name. Each derives exactly what the ordinary suite calls
  valid in its logic, which the tests check both ways.
- **`interact --calculus CALCULUS`** builds a derivation step by step
  in a calculus given as data: `rules` lists what applies, `apply`
  applies a rule backwards and asks what the goal leaves open by name,
  `assume` closes a goal by a hypothesis in force (`--assume` adds
  hypotheses), and every step is checked by the calculus's checker. In
  the library, `rules::Interactive`; its JSON is at wire level 17. A
  calculus's terms and sequents read
  from text in its own syntax (`Calculus::parse_term`,
  `parse_sequent`).
- **Forward steps in a session of a calculus given as data**: `derive G
  RULE L…` applies a rule forwards from lines in force and adds its
  conclusion as a new line, beside `apply`'s backward steps; each goal
  lists the lines in force with numbers that never change, and `assume
  G L` closes a goal by line L. In the library, `Interactive::derive`,
  `lines` and `cite`, and `undo` answers `rules::Undone`. A session
  that holds such a line is written at wire level 18.
- **A derivation in a calculus given as data is drawn** as one of LK
  is: `rules::Drawing::new(&calculus, &derivation, &limits)` gives what
  `export::svg`, `latex` and `typst` draw, each node under its rule's
  name and the formulas in the rule file's symbols. Rocq has no
  certificate for it (`Unsupported::Calculus`) and it has no
  lambda-term (`NoTerm::Calculus`). The web bindings build such a
  derivation step by step (`rules_start`, `rules_apply`, `rules_assume`,
  `rules_undo`, `rules_check`, `rules_draw`), in a built-in calculus or a
  rule file the page holds.
- The manual has a chapter on rule systems written as hypotheses and
  decided by the Horn engine, and when a calculus of your own is the
  better tool.
- The library also ships one-sided linear logic and signed tableaux as
  rule files (`rules::LL`, `rules::TABLEAU`), writes a tableau in the
  latter (`Derivation::from_tableau`), and prints a calculus's terms by
  its print forms (`Calculus::show`, `show_sequent`).
- **Negative translations, checked** (`deduxit seq translate`, the
  library's `ordinary::negative`, the bindings' `ordinary_translate`):
  Kolmogorov's and Gödel and Gentzen's translations of classical logic
  into minimal logic, Kuroda's and Glivenko's into intuitionistic logic.
  The sequent is decided classically and its translation in the target
  logic, and both verdicts are shown, with their derivations or
  countermodels in the JSON; verdicts that differ are an error.
- **Craig interpolants, checked** (`deduxit interpolate`, the library's
  `ordinary::interpolant`, the bindings' `ordinary_interpolate`): read
  off a derivation of LK or LJ without cuts by Maehara's method, for a
  split of the sequent you name (`--first`), and checked by deciding
  both halves and comparing their atoms.
- The answers of both are documents at wire level 16, above the rule
  files' 15.
- **What each suite offers, as data** (the library's `protocol`, a
  default feature): the kinds of values, the operations between them
  (`ordinary.translate`, `linear.prove`, …) with their parameters, the
  settings each reads and what each promises, run and checked over JSON
  (`protocol::run`, `protocol::check`, `protocol::catalogue`). The
  manual's chapter "What each suite offers" lists them, with a JSON
  Schema of every form.
- **A workspace** (`deduxit workspace`, the library's `workspace`):
  a JSON file that keeps sequents and what was made from them by those
  operations, each node with its parameters and the limits it ran
  under. Changing a sequent marks what was made from it out of date
  until `workspace run` makes it again; `undo` and `redo` take back and
  bring back whole edits and show the earlier results without running
  them again; `--checks` checks what each operation promises. Reading a
  file checks every value and runs again each run that needs no search.
  Its bounds are the settings `limits.workspace` (nodes, runs kept,
  edits undo keeps, bytes); past one, a change is refused
  (`workspace_limit`). Its JSON is at wire level 19: the highest level
  the library now reads and writes.

### Wire forms

- Level 15 adds a calculus given as data: the rule file, and a
  derivation in such a calculus.
- Level 16 adds a classical sequent's negative translations compared
  and an interpolation, each holding ordinary outcomes nested without
  `version` and `deduxit`. They are written only.
- Level 17 adds a session of step-by-step proving in a calculus given as
  data: the rule file, the terms, the scopes, the nodes from the
  conclusion up and the steps.
- Level 18 adds a line that a forward step derived, a node's `line` and
  a goal's `cites`, in such a session; a session without one is written
  at level 17.
- Level 19 adds the workspace: its values, each its kind's document
  whole, the nodes, given or made by an operation, the records of runs
  and the log of edits. `wire::LEVEL` is 19. Every document of levels 1
  to 14 reads and is written as before.


Install it as the manual's [Installation](https://deduxit.github.io/deduxit/manual/installation.html) says; `SHA256SUMS` holds the files' sums.
````
