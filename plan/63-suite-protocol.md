# Step 63: the suites' written protocol, and Learn and Practice by suite

You are working in deduxit (`deduxit/deduxit`, the checkout `~/Projects/own/deduxit`, the bindings in
`web/`) and in the web client (`deduxit/deduxit.github.io`).
CLAUDE.md applies throughout: jj only (never git), thematic commits as
soon as a unit is done, doc comments on every item, the verification
table, no pushing. This prompt was written on 2026-11-08 by step 46's
second session, from the author's decisions of that day, and is finished
at the review of step 62: it says what is fixed.

Read `plan/notes/research/next/web-front-end.md` (candidates 3, 4, 5 and
7), `web-teaching.md` (candidate 5), `scope-education.md` (education-3
and education-10), `scope-interop.md` (interop-1), the reports of steps
47, 49, 56, 61 and 62, `plan/web/design.md`, `.claude/rules/web.md`, and
`plan/README.md`'s D29.

## Goal

The collection's frame for the page, written from the suites that work
by now (linear; ordinary with sequents, tableaux, truth tables and
natural deduction; custom rules) rather than before them (D29): one
protocol every suite's bindings speak, and the page's Learn and Practice
organised by suite with content for the ordinary suite.

## Fixed now

- **The protocol** (web-front-end-5): `read`, `start`, `view`, `rules`,
  `apply`, `undo`, `close`, `hint`, `draw`, `export_as`, `certificate`,
  `describe`, each stateless JSON with a wire level, and a `layout`
  field (`tree`, `list`, `graph`, `text`) that says which host the stage
  draws into; written down with a JSON Schema of the wire forms
  (interop-1) and tested under Node; the linear calls keep working.
- **One module** (D29) unless step 47's figures and this step's
  measurement show the module past the client's budget; a module per
  suite loaded lazily (web-front-end-4) is the answer then, and a suite
  from another repository the other trigger.
- **Learn and Practice keyed by suite** (web-front-end-7, web-teaching-5):
  a lesson format as data (education-10), the linear lessons and the
  store of 452 problems moved under the linear suite unchanged, and the
  ordinary suite's lessons and problems written with the education
  plugins, their verdicts decided by the command in a flake check as the
  linear store's are. A suite without lessons is a tool, not a course
  (the research's warning): say what each suite has.
- Step 38h applied invertible rules automatically; whether the
  auto-reverse of education-3 needs more on the new suites is checked
  here.

## How the step runs

Two to three sessions on Opus 5.5 at `high` (the protocol and the
bindings; the client's Learn and Practice with the education plugins,
a fresh-context reviewer per content session as step 43's).
Deliverables: the commits, the client's commits,
`plan/reports/63-suite-protocol.md`.

## What steps 60 to 62 left you (the review of 62c, 2026-11-16)

- **The suites that work now**: linear; ordinary (sequents by LJT, the
  SAT solver, the embeddings, truth tables, tableaux, Kripke
  countermodels, 63t's translations and interpolants); a calculus given
  as data (step 60's rule files, 61's sessions, 62's natural deduction
  NJ, NK, NM with forward and backward steps, Gentzen trees and Fitch
  proofs). Each speaks its own calls in `web/src/` today; the protocol
  makes them one set.
- **Wire levels**: 17 is a session in a calculus given as data, 18 one
  that holds a derived line (62b). Take the next free for anything new.
- **`layout`**: 62c's `ViewOptions::layout` (`Layout::Tree`, `Fitch`;
  the settings key `view.layout`) is the first view choice of a suite;
  the protocol's `layout` field (`tree`, `list`, `graph`, `text`) is the
  host the stage draws into, and the Fitch proof is a `list`.
- **The workspace graph (63g)** is designed first (63g-a, a Fable panel,
  `plan/notes/63g-design.md`): it names each suite's artifacts and
  operations. The protocol writes them down as its own, so that 63g-b
  takes every suite from the protocol.
- **Not overwhelming** (the author): the protocol changes no visitor's
  first screen; Learn and Practice by suite keep the linear course as
  it is and add the ordinary suite's beside it.
- **The module's size**: the client's ceiling is 3.3 MB at gzip
  (`budget`); measure the module after 62d before the module question.

## The sessions (planned at this review)

- **63a, the protocol and the bindings** (Opus 5.5 at `high`, the fresh
  reviewer on Fable 5.2): after 62d has landed (it owns `web/src/rules.rs`
  until then) and after 63g-a's design. The protocol written down with
  its JSON Schema (interop-1), every suite's bindings speaking it, the
  linear calls kept, the tests under Node, the module question
  answered with its measurement.
- **63b, Learn and Practice by suite** (Opus 5.5 at `high`, in the
  client, with the education plugins; a fresh-context reviewer per
  content session): the lesson format as data, the linear lessons and
  the 452 problems moved under the linear suite unchanged, the ordinary
  suite's lessons and problems (sequents, tableaux, natural deduction)
  with their verdicts decided by the command in a flake check.
- **63c** if the content needs a second session.
