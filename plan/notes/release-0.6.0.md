# The release of 0.6.0: notes, commands and checks

Written by session 69 (2026-11-16, resumed 2026-11-20) for the planning
session and the author. The tags are `v0.6.0` in `deduxit/rocq-deduxit`
first, then in `deduxit/deduxit`. The Rocq library gained
`Deduxit.Modal` since its 0.4.0, so it is released beside deduxit under
deduxit's number (there is no rocq-deduxit 0.5.0), and deduxit 0.6.0's
certificates name rocq-deduxit 0.6.0 (`export::rocq::DEDUXIT`, which
`modules/rocq-deduxit.nix` asserts equal to the pinned opam file's
version). The author's yes for the tags of 0.6.0 is asked at the review
of step 68 (the yes of 2026-11-16 was for 0.5.0).

## What 0.6.0 is

"Modal logics" (D27): steps 65 to 68, with 54x6's trimmed module and
wire docs and 54x7 (Ser with one label, auto-reverse in ordinary
sessions, the modal fuzz run). On the page: the modal tab, the model
editor, the tableau editor, course packs, the check view, the embed and
print, which ship with the client.

## The order of the tags and the publishing

1. **rocq-deduxit**: its release commit on its `main` (dated
   2026-11-20), CI green there, then the planning session makes the
   signed tag object `v0.6.0`, message "rocq-deduxit 0.6.0". Its
   release workflow checks the tag against the opam file, the citation
   and the changelog, runs `nix flake check`, makes the GitHub release
   with the changelog's section and the SHA-512 of the tag's source
   archive, and publishes the documentation on Pages.
2. **deduxit's chain on `main`**: it pins rocq-deduxit's release commit
   (`flake.lock: Update` in the chain), so the tag does not change what
   it pins. The chain is pushed with `plan/push-main.sh @-`.
3. **deduxit's CI**: `ci.yml` green on `main` at the commit to tag, and
   `kani.yml`'s last run on `main` green (`gh run list --repo
   deduxit/deduxit --workflow kani.yml`, a read).
4. **deduxit's tag**: the signed tag object `v0.6.0`, message "deduxit
   0.6.0", by the planning session after the author's yes.
5. **The publish**: the job `cargo publish` waits for the author's
   approval in the environment `crates-io`. Then the release job makes
   the GitHub release.
6. **After**: the client's input `deduxit` moves to the tagged commit;
   the Zenodo check for both repositories.

## The GitHub release notes

Each release workflow writes its notes from the tagged tree:

- **deduxit** (`.github/workflows/release.yml`, job `release`): the
  title `deduxit 0.6.0`; the notes are `CHANGELOG.md`'s section
  `## [0.6.0] - 2026-11-20` (to the next `## [`), then the line naming
  the manual's Installation chapter and `SHA256SUMS`. Attached:
  `deduxit-0.6.0.crate` and `deduxit-cli-0.6.0.crate` from
  static.crates.io, the archives and Linux packages of `packages.yml`,
  `wire.schema.json` and `rules.schema.json`, all in `SHA256SUMS`, each
  attested. The text the workflow will write is at the end of this
  file.
- **rocq-deduxit** (its `release.yml`): the title `rocq-deduxit 0.6.0`;
  the notes are its changelog's section `## [0.6.0] - 2026-11-20`, then
  the source archive's SHA-512 for opam.

A change of the text is a change of the changelog before the tag; after
it, `gh release edit v0.6.0 --notes-file …` (an administrative act, the
author's yes first).

**The date.** Both changelogs and both `CITATION.cff` say 2026-11-20,
and rocq-deduxit's opam file `date:2026-11-20`. If a tag is made on
another day, the planning session moves them to that day before tagging
(deduxit's workflow checks only that the section is dated and the
citation states the version; rocq-deduxit's the same, and the opam
version).

## What a user can do now

For an announcement:

- Decide a sequent of propositional modal logic in any of the fifteen
  normal systems from K to S5: `deduxit prove --system S4 '[]p ->
  [][]p'`. A valid sequent comes with a checked derivation in the
  system's labelled sequent calculus (G3K with the system's frame
  rules); one that is not valid with a checked Kripke countermodel,
  drawn in SVG, PNG and PDF with an arrow from each world to each world
  it sees.
- Check a Kripke model you write against a modal sequent: `deduxit
  check --system K --sequent '[]p |- p' model.json`, which names the
  frame condition that fails, or the world where the sequent fails and
  why.
- Prove a labelled sequent step by step in any system's calculus:
  `deduxit interact --calculus G3S4 'w0 : []p |- w0 : [][]p'`, with
  forward steps (`derive`) for the rules that add before `⊢`.
- Get a Rocq certificate of either answer, a derivation's or a
  countermodel's: `--format rocq` under `--system`, `check --format
  rocq`, and a session's `proof --rocq`. They compile against
  rocq-deduxit 0.6.0, whose `Deduxit.Modal` proves the labelled
  calculus sound for Kripke models, with no axiom.
- Check a signed tableau written by hand, as text: `deduxit check
  --tableau tableau.txt`, which names the first step that breaks a rule,
  a branch marked wrongly or the first branch not finished.
- Let a session in LK or LJ apply the invertible rules by itself:
  `deduxit interact --logic intuitionistic --invertible 'c, a -> b, a |- b /\ c'`,
  or `invertible on` in a session. LJ leaves `→L`, `¬L` and `∨R` to
  the user, since they can lose a proof.
- In the library: `deduxit::modal` (`decide`, `System`, `Condition`,
  `Model::truth`, `Model::frame_fault`), `rules::Interactive::extend`,
  `ordinary::WrittenTableau`, `ordinary::Interactive::invert_all`, and
  the modal writers of `export::rocq` and `export::svg`.
- On the page: the modal tab (decide, draw, prove step by step), the
  model editor (change or build a countermodel and check it), the
  tableau editor, and for teachers course packs as links, the check view
  of the students' hand-ins, the embed and print. The page sends
  nothing.

## Before the tag

- 54x7 is on `main` and folded into the changelog's 0.6.0 section.
- rocq-deduxit's `v0.6.0` exists (README's install lines name it and
  fail until it does).
- `ci.yml` green on `main` at the commit to tag; `kani.yml`'s last run
  on `main` green.
- The client's `main` has the page's 0.6.0 part (68b, 68c, 68d, on its
  `main` since their reviews) and moves its input `deduxit` once this
  chain is pushed.

## After the tag

- The publish job asks the author's approval in the environment
  `crates-io`. `deduxit` and `deduxit-cli` exist on crates.io with
  their trusted publishers, so the job publishes both through Trusted
  Publishing (`crates-io-auth-action`); `linlog` and `linlog-cli` are
  skipped, since crates.io has their 0.3.0; no secret is used (the
  selection, run locally on 2026-11-20: `new=` none, `existing=deduxit
  deduxit-cli`).
- The release job attaches `wire.schema.json` and `rules.schema.json`;
  check that the release lists both and that `SHA256SUMS` names them.
- The planning session moves the client's input `deduxit` to the tagged
  commit, and checks the live site and docs.rs.
- The Zenodo check, as for 0.3.0 (`plan/notes/release-0.3.0.md`, "After
  the tags: the Zenodo check"), with `v0.6.0`: a record titled `deduxit`
  under 10.5281/zenodo.23448588 and one titled `rocq-deduxit` under
  10.5281/zenodo.23899109.
- **opam waits** (the author, 2026-11-09, until 0.8.0 or later).

## The author's checklist

Before the tags:

- [ ] Nothing to set: the trusted publishers of `deduxit` and
  `deduxit-cli` name `deduxit`/`deduxit`/`release.yml`/`crates-io`, and
  no secret is needed. Confirm on crates.io that both still list
  theirs.
- [ ] Say yes to the tags of 0.6.0, rocq-deduxit's first, once CI is
  green at each release commit.

After the tags (the planning session makes them, signed tag objects):

- [ ] Approve the job `cargo publish` in the environment `crates-io`.
- [ ] The Zenodo check above, for both releases, if the planning
  session has not reported it.

## The notes as the workflow will write them

````markdown

Modal logic, and tools for teachers. deduxit decides a sequent of
propositional modal logic in any of the fifteen normal systems from K
to S5: a valid one with a checked derivation in the system's labelled
sequent calculus, one that is not valid with a checked Kripke
countermodel. Both become Rocq certificates for rocq-deduxit 0.6.0,
whose `Deduxit.Modal` proves the labelled calculus sound. A labelled
sequent can be proved step by step, with forward steps, and a Kripke
model you write is checked against a sequent. `check --tableau` checks a
signed tableau written by hand and names its first wrong step. On the
web page, the modal tab decides, draws and proves step by step; a model
editor changes or builds a countermodel and checks it; a tableau editor
checks a tableau as it is written. In LK and LJ, a session proving
step by step can apply the invertible rules by itself. A teacher hands
out a course pack of problems as a link, reads the students' hand-ins
in a check view, embeds a tab in another page and prints a handout.

### What breaks, and what to do instead

Nothing that 0.5.0 offers was removed or changed: its calls, types,
command lines, error codes and JSON forms are as they were. Every new
variant or field belongs to a `#[non_exhaustive]` type, and the new
`rocq::Unsupported::Assumption` comes after the other variants, so
their numbers stay. What a program may notice is under "Changed".

### Changed

- **A name a rule asks to be new is new to the rule's other variables
  too**: a derivation or a session in a labelled modal calculus no
  longer takes `Ser` with one new label at both ends of its edge
  (`w1 R w1`), which made "a new world that sees itself". The rule
  file's `fresh` says so.
- **`check --format json` names the modal system** under the new key
  `system`, for a modal outcome or a model checked under `--system`.
- **Rule labels with `□` or `◇`** are set as symbols in every export:
  `\Box` and `\Diamond` in LaTeX.
- **The help of `check`** names the new flags `--system` and
  `--tableau`, and says that `--sequent` takes a modal sequent or the
  sequent a tableau starts from; **the help of `prove`** names
  `--system`; **the help of `interact`** names `--invertible`.
- **The protocol has a fourth suite**, `modal`, with its kinds and
  operations, so a program that walks `protocol::suites()` or the
  catalogue meets them.
- **0.5.0 refuses what 0.6.0 writes at wire level 20**, naming the
  level (`unsupported_version`).

### Added

- **The invertible rules in LK and LJ** (`ordinary::Interactive::invert`,
  `invert_all`, `is_invertible` and `invertible_steps`; `interact --logic … --invertible` and
  the session command `invertible on|off`; the bindings'
  `interactive.invertible` and `ordinary_invert_all`): the axiom and the
  rules of the connectives with a shared context are applied by
  themselves, but in LJ `→L`, `¬L` and `∨R₁`, `∨R₂`, which can lose a
  proof.
- **Modal formulas and systems** (the library, `deduxit::modal`): a
  sequent of propositional modal logic is read from text, the syntax of
  ordinary logic with the prefixes `[]` or `□` and `<>` or `◇`
  (`[]p -> [][]p`), and from JSON, and written as both.
  `modal::System` is one of the fifteen normal systems from K to S5,
  held as the conditions its frames meet (`modal::Condition`: serial,
  reflexive, symmetric, transitive, euclidean), so
  `System::T.with(Condition::Euclidean)` is `System::S5`; it reads the
  usual names (`S4`, `KT4`, `KD45`). A sequent with `□` or `◇` is
  written at wire level 20.
- **Modal sequents decided in all fifteen systems** (the library,
  `modal::decide`): a valid sequent comes with a derivation in the
  system's labelled sequent calculus (`modal::calculus`, Negri's G3K
  with the frame rules of the system, as a calculus given as data),
  which the checker of `deduxit::rules` accepted; a sequent that is not
  valid comes with a Kripke model whose relation meets the system's
  frame conditions and that refutes it, checked too where it is small
  enough to check. `Model::truth`
  and `Model::frame_fault` check a model you give against a modal
  sequent and a system, naming the frame condition that fails and the
  worlds that show it. The outcome has a JSON form at level 20.
- **Modal logic in the command** (`--system`): `deduxit prove --system
  S4 '[]p -> [][]p'` decides a modal sequent and prints the derivation,
  or the countermodel's worlds and what each sees; `--format json`
  writes the outcome, which `check` checks again. `check --system K
  --sequent SEQUENT MODEL` checks a Kripke model you write. A derivation
  is drawn in LaTeX, Typst and SVG, and a countermodel in SVG, PNG and
  PDF, with an arrow from each world to each world it sees.
  `check --calculus` takes the labelled calculi by name (`G3S4`), and
  `workspace new --system` keeps a modal sequent.
- **Modal logic in the protocol**: the kinds `modal.sequent`,
  `modal.outcome`, `modal.model` and `modal.evaluation`; the operations
  `modal.decide`, `modal.countermodel`, `modal.evaluate` (a model
  checked against a sequent), `modal.derivation`, `modal.calculus` (the
  system's rule file), `modal.draw`, `modal.export` and
  `modal.duplicate`; and the schema's forms `modal_sequent`,
  `modal_outcome`, `modal_statement` and `modal_evaluation`.
- **Modal proofs step by step**: `deduxit interact --calculus G3S4
  'w0 : []p |- w0 : [][]p'` proves a labelled sequent in any system's
  calculus. `derive G RULE N…` applies a rule forwards to members of the
  goal in a sequent calculus: `L□` derives `y : p` from `x : □p` and
  `x R y`, `Trans` derives `x R z`; a rule that acts on or adds a
  member after `⊢`, splits the goal or closes it is refused forwards
  with the reason. In the
  library, `rules::Interactive::extend` and `rules::Calculus::contexts`.
- **Certificates of modal logic** (`--format rocq` under `--system`):
  a valid sequent's labelled derivation as a term of deduxit's Rocq
  library (`Deduxit.Modal`), proving the sequent valid over the system's
  frames by `modal_soundness`, and a countermodel proving it not valid
  by `modal_refutation`; `check --format rocq` of a modal outcome or of
  a derivation in a `G3…` calculus, and a session's `proof --rocq`, the
  same. In the library, `export::rocq::write_labelled`,
  `write_modal_countermodel` and `write_modal_outcome`, and
  `rocq::Unsupported::Assumption` for a derivation that rests on an
  assumption.
- **`[]` and `<>` in a calculus given as data**: a calculus's text
  syntax reads them as `□` and `◇` where it has those connectives.

- **A tableau written by hand, checked step by step** (`check
  --tableau`, the library's `ordinary::WrittenTableau`, the bindings'
  `ordinary_tableau_review`): a signed tableau of classical logic as a
  student writes it, a numbered line per signed formula with the line it
  expands, branches as an indented list or as `prove --method tableau`
  draws them, `×` and `open` under a branch. The check names the first
  step that breaks a rule (its line, the rule, what the rule gives), a
  branch marked wrongly and the first branch not finished; a tableau
  without a fault is checked again as a derivation in the rule file
  `tableau`, and an open one's branch gives a checked countermodel.
  `--sequent` names the sequent it must start from. Exit 0 for a right
  and finished tableau, 1 for a wrong or unfinished one.
- **The textbooks' rules in the rule file `tableau`**: `T↔′` and
  `F↔′` expand an equivalence by its sides' values (`T p, T q` and `F
  p, F q`), beside LK's two implications; `F→₁` and `F→₂` take one
  result of `F→` alone, as `T∧₁` and `F∨₂` already did.

### Wire forms

- Level 20 adds modal logic: a sequent whose formulas hold `□` or `◇`
  (one without is written at level 8, as the ordinary sequent it is),
  and the outcome of deciding one, with its derivation or its
  countermodel. `wire::LEVEL` is 20. Every document of levels 1 to 19
  reads and is written as before.


Install it as the manual's [Installation](https://deduxit.github.io/deduxit/manual/installation.html) says; `SHA256SUMS` holds the files' sums.
````
