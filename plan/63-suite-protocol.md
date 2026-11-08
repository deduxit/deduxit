# Step 63: the suites' written protocol, and Learn and Practice by suite

You are working in linlog (`sequentium/sequentium`, the bindings in
`web/`) and in the web client (`sequentium/sequentium.github.io`).
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
