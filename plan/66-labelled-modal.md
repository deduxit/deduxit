# Step 66: the labelled modal calculus step by step, and its soundness in Rocq

You are working in linlog (`deduxit/deduxit`) and in a Rocq
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
  author: a module of `rocq-linlog`, or a library of the collection of
  its own (a new repository and opam package, created by the author;
  46a's structure note expects a generic checker's library to be a new
  package). Either way the flake checks the certificates against the
  pinned library, as `rocq-linlog`'s check does.

## How the step runs

Three sessions on Opus 5.5 at `high` for the calculus and the page, two
for the Rocq proof (Fable 5.1 at `xhigh` only if Opus visibly falls
short); a fresh-context reviewer on the rule files against the note's
sources. Deliverables: the commits in the repositories,
`plan/reports/66-labelled-modal.md`.
