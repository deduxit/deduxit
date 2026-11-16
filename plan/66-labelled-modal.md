# Step 66: the labelled modal calculus step by step, and its soundness in Rocq

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and in a Rocq
library (below). CLAUDE.md applies throughout: jj only (never git),
thematic commits as soon as a unit is done, doc comments on every item,
the verification table, no pushing. This prompt was written on
2026-11-08 by step 46's second session, from the author's decisions of
that day, and is finished at the review of step 65: it says what is
fixed.

Read `plan/notes/research/next/modal-logics.md` (candidates 2 and 3),
`custom-rules.md`, the reports of steps 60, 61 and 65, the Rocq
library's rules, and `plan/README.md`'s D28 and D31.

## Goal

The frame conditions made visible: Negri's labelled sequent calculus
for the systems of step 65, proved step by step on step 60's generic
derivation (labels and relational atoms are the sort it was built to
carry), in the command and on the page; and the labelled derivations'
soundness for Kripke semantics proved once in Rocq, so a labelled proof
exported as a certificate is checked by the library.

## Fixed now

- The labelled calculus is a rule file per system, checked by step 60's
  checker, so its verdict is the generic one ("valid in the calculus
  given") until the certificate makes it more; the page uses step 61's
  builder.
- **Where the proof lives** is settled at the review of step 65 with the
  author: a module of `rocq-deduxit`, or a library of the collection of
  its own (a new repository and opam package, created by the author;
  46a's structure note expects a generic checker's library to be a new
  package). Either way the flake checks the certificates against the
  pinned library, as `rocq-deduxit`'s check does.

## How the step runs

Three sessions on Opus 5.5 at `high` for the calculus and the page, two
for the Rocq proof on Fable 5.2 at `high` (the proof of a calculus's
metatheory); a fresh-context reviewer on the rule files against the note's
sources. Deliverables: the commits in the repositories,
`plan/reports/66-labelled-modal.md`.

## What step 65 left you (the review of 65c, 2026-11-16)

- **The labelled calculus exists already** (65b): `modal::calculus(system)`
  builds Negri's G3K with the system's frame rules as a calculus given as
  data, for all fifteen systems of the cube; `check --calculus G3S4` and
  `interact --calculus G3S4 'w0 : □p |- w0 : p'` already read it
  (`calculus::named`). Its departures from Negri's Table II (kept
  principal formulas, `L¬`, `R¬`, `R⊤`, the `↔` rules, no initial
  sequent on a relational atom) are listed in `calculus()`'s doc comment;
  the panel's argument member found every rule sound.
- **Open from 65c**: the rule file's syntax takes `□` and `◇` but not
  `[]` and `<>`; a session's forward steps (62b) on labelled rules are
  untried; the page's builder (61b, 62d) has no modal tab until 65d.
- **The certificate**: the exporter's second kernel (`--kernel deduxit`,
  `rocq-deduxit`) checks linear proofs; a labelled derivation's
  certificate is new: its term in Rocq, the soundness theorem for Kripke
  semantics over the frame conditions, and the flake check of the
  exported certificates against the pinned library.

## The sessions (planned at this review)

- **66a, the calculus step by step** (Opus 5.5 at `high`): the ASCII
  spellings, sessions in the labelled calculi backwards and forwards
  (62b's lines), the command's chapter, the page's modal tab gaining the
  builder (after 65d), the certificate's exporter side designed with the
  Rocq session.
- **66b and 66c, the Rocq proof** (Fable 5.2 at `high`): the labelled
  calculus and its soundness for Kripke semantics, the checker as a
  function and the theorem that a term it accepts proves its sequent,
  in the place the author chooses (below); the flake check of 66a's
  certificates.
- **Where the proof lives**: asked of the author at this review, with
  the recommendation of a module `Deduxit.Modal` of `rocq-deduxit`
  (one opam package, its CI and the flake's input already in place; a
  new repository is the author's act).
