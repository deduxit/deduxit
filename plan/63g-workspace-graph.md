# Step 63g: the workspace graph

You are working in deduxit (`deduxit/deduxit`, the checkout
`~/Projects/own/deduxit`, the bindings in `web/`) and, for the page, in
the web client (`../deduxit.github.io`). CLAUDE.md applies throughout:
jj only (never git), thematic commits as soon as a unit is done, doc
comments on every item, the verification table, no pushing. This prompt
was written on 2026-11-16 by the planning session from the author's
requests of 2026-11-15, after the reviews of 63t and 62c.

Read `plan/README.md`'s rows 63, 63t and 63g and the Status entries of
2026-11-15 about 13:10 and 13:15, the reports of steps 49 (the suite
registry), 61 (sessions in a calculus given as data), 62 and 63t (the
translations, "What the next sessions must know": 63g),
`plan/web/design.md`, `.claude/rules/web.md`, `core.md` (the wire
levels, the limits) and `.claude/rules/writing.md`.

## The author's words (2026-11-15)

- "I want the different features to interact with each other
  seamlessly", with graph-based internal representations as node-based
  video tools have, each macro action undoable.
- "Just make sure that it will not be overwhelming for users."
- "When designing that system, please involve multiple agents including
  some Fable agent; and use the education-agent-skills as well as
  frontend-design plugins."

## Goal

One representation in which the suites' results meet: artifacts as
immutable nodes, operations as edges, so that a sequent, its
translation, the translation's proof, a countermodel, a drawing and a
certificate are one workspace a user can branch, change upstream, undo
and share, in the library, the command and on the page.

## Fixed now

- **Artifacts are immutable nodes** with their JSON form: sequents in any
  logic (linear, ordinary, a calculus given as data), outcomes and their
  evidence (derivations, countermodels, certificates), drawings and
  exports, sessions of step-by-step proving as a node whose value is the
  session's saved state.
- **Operations are edges with their parameters**: decide (method,
  limits), transform and translate (63t's negative translations, the
  interpolant's partition; later NNF, CNF, Tseitin of step 70), embed,
  draw and export. An edge records what it promises (a translation's
  verdicts agree, CNF is equivalent, Tseitin equisatisfiable) and the
  check that holds it, shown beside the result.
- **Change upstream marks what follows stale** and runs it again on
  request or automatically by the page's setting; results are cached by
  their inputs, the operation's parameters and the limits they ran
  under, and an "unknown" from a limit says which limit.
- **Undo and redo over a log of the graph's edits**, apart from a proof
  session's own step undo (61a's and the ordinary session's).
- **Step 63's protocol names every suite's artifacts and operations**,
  so a later suite joins the graph without work of its own; the graph
  takes its node and edge kinds from the protocol, not from a list of
  its own.
- **Not overwhelming** (fixed):
  - one sequent looks and works exactly as it does without the graph;
  - the graph appears only when a user branches (transforms, translates
    or duplicates), first as a list of cards with arrows;
  - the node canvas is a second view the user opens;
  - Learn's lessons start without it;
  - the page's design is checked against the education skills'
    cognitive-load analysis, and a first-time task is tried in a browser
    before the step ends.
- **The design is a panel's**: several agents propose and criticise the
  data model and the interaction independently, one of them on Fable
  5.2, with the education-agent-skills plugin (cognitive-load-analyser,
  worked-example-fading-designer, explicit-instruction-sequence-builder,
  ai-feedback-design-principles, udl-barrier-anticipator) and the
  frontend-design plugin; the session merges their proposals and says
  which it took and why.
- **Deterministic and bounded**, as the library is: the graph reads no
  clock, every operation runs under the caller's limits, a workspace's
  size is bounded (nodes, edges, the log, its bytes) with a refusal at
  each bound.
- **A workspace shared as a link** on the page (as a session is today),
  nothing sent to a server; the command reads and writes a workspace
  file at a new wire level.

## How the step runs

Three to four sessions, each reviewed by the planning session and by
`fresh-reviewer` on Fable 5.2 at `medium` (on Opus 5.5 at `high` for the
Fable session):

- **63g-a, the design** (Fable 5.2 at `high`): the panel and the
  merged design, in `plan/notes/63g-design.md`. The data model (node and
  edge kinds, identity and hashing, the cache key, staleness, the edit
  log and its undo, the bounds, the JSON form and its wire level), the
  operations' promises and checks, the command's shape, and the page's
  interaction (the list of cards, the canvas, the first-time task, the
  links between tabs: 63t's "Prove it in <logic>" becomes the first
  edge). It names the artifacts and operations each existing suite
  offers, which step 63's protocol then writes down. Design only, with
  a sketch of the library's types; no code lands.
- **63g-b, the library and the command** (Opus 5.5 at `high`): the
  module (`deduxit::workspace` or the name the design fixes), its JSON
  form and wire level, its tests (a random walk of edits against a
  reference that recomputes everything; undo and redo restore exactly),
  the command's workspace file, the manual's chapter, the guide.
- **63g-c, the page** (Opus 5.5 at `high`, in the client, with the
  frontend-design plugin and the education skills): the bindings' calls,
  the cards and the canvas, the link, every state in `tests/states.ts`,
  the first-time task tried in a browser by a fresh agent that has not
  seen the design.
- **63g-d** if the page needs a second session.

63g-a runs beside step 62d and before step 63's protocol session; 63g-b
after 63's protocol has landed.

Deliverables: the commits, the client's commits,
`plan/notes/63g-design.md`, `plan/reports/63g-workspace-graph.md`.
