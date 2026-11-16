# Step 68: tools for teachers: a model editor, a tableau checker, course packs

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`) and in the web client
(`deduxit/deduxit.github.io`). CLAUDE.md applies throughout: jj
only (never git), thematic commits as soon as a unit is done, doc
comments on every item, the verification table, no pushing. This prompt
was written on 2026-11-08 by step 46's second session, from the author's
decisions of that day, and is finished at the review of step 67: it says
what is fixed.

Read `plan/notes/research/next/modal-logics.md` (candidate 5),
`tableaux.md` (candidate 2), `scope-education.md` (education-4 and
education-8), `web-teaching.md` (candidates 2, 7 and 9), the reports of
steps 56, 63 and 65, and `plan/web/design.md` (D31, D36: no server, no
tracking).

## Goal

A teacher can set, restrict and check exercises without a server: a
Kripke model editor on the page (draw a model, evaluate formulas at its
worlds by step 55's and step 65's checker of a model the user gives,
the intuitionistic, minimal and modal suites' countermodels opened in
it), a checker for a
student's own tableau (a tree written by hand, checked rule by rule
with the failing step named), and course packs: a link that fixes a set
of problems and the allowed suites and methods, and a handed-in link a
teacher opens in a check view; the page embedded in a course's own
site (education-8's embed: a problem or a pack in a frame; the share
link is built already); a print stylesheet for handouts.

## Fixed now

- Nothing is stored or sent anywhere but in the links and the browser
  (`plan/web/design.md`'s D31 and D36); no grade book, no accounts, no
  LTI.
- The tableau checker reads a tableau in a written format (sourced from
  the textbooks the note names) and checks it with the library; its
  verdicts are the library's.

## How the step runs

Three sessions on Opus 5.5 at `high` (the library's tableau reader and
checker with its bindings; the model editor; the packs, the check view
and print), the client's with the `frontend-design` and education
plugins; a fresh-context reviewer per client session. Deliverables: the
commits, the client's commits, `plan/reports/68-teaching-tools.md`.

## What steps 63 to 67 left you (the review of 67a, 2026-11-16)

- **The model a user gives** is checked in both suites now: step 55's
  ordinary Kripke model (`check --logic … MODEL`) and 65's modal one
  (`check --system S --sequent SEQ MODEL [--world N]`, the bindings'
  `modal_evaluate`, `modal_model_drawing`); 65d's page has a model
  editor as a list of worlds with checkboxes. Its "What the next
  sessions must know" (68) says a drawing to edit on (dragging an
  arrow) is this step's.
- **Tableaux**: step 56's signed tableau is a calculus given as data
  (`tableau.json`), so a student's tableau can be a derivation in it,
  checked by step 60's checker; the written format is still to choose
  from the textbooks.
- **Course packs**: 63b's lesson format and stores by suite, 63g-c's
  links at version 3 (a group of linked tabs) and the shelf are the
  pieces a pack builds on; the ordinary store has no `literature`
  problems yet.
- **No kernel crate** (67a): if the tableau checker needs a meter, move
  the one of `rules/check.rs` to `limits` instead of writing a third.
- **The module's ceiling** is 3 750 000 bytes at gzip (54x6); the page
  is at 3 637 866 after 65d.

## The sessions (planned at this review)

- **68a, the tableau checker** (library, command, bindings): the written
  format, the reader and the rule-by-rule check with the failing step
  named.
- **68b, the model editor**: drawing and editing a model on the page,
  the countermodels of the ordinary and modal suites opened in it.
- **68c, packs, the check view, the embed and print** (the client).
