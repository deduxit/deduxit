# Step 63g: the workspace graph

## Session 63g-a: the design

Fable 5.2 at `high`, a reused worker (its conversation cleared for this
step), in the workspace `../deduxit-wt-step-58a-610ed3` on `main@origin`
`qsllsruo`, from the brief `/tmp/linlog-handoff/start-63g-a.md` and the
prompt `plan/63g-workspace-graph.md`, on 2026-11-16. No build, test or
call of the command but one: three calls of the existing debug build on
cores 7 to 9, capped, to take real JSON for the note's example.

### Outcome

The design is `plan/notes/63g-design.md` (about 1 450 lines, eighteen
sections): the data model, each suite's kinds and operations, the cache
and its key, the states, the edit log, the bounds, the JSON form at
wire level 19, the promises and their checks, a sketch of the library's
types, the bindings' calls, the command's shape, and the page (one
sequent unchanged, the list of linked tabs, the graph as a second view,
the first-time task, the link). Every choice names the proposal it took
and what it rejected. No code lands.

The design in ten lines:
- A **value** (an artifact) is immutable and interned; a **node** is a
  place that holds one; a node is given or has a **recipe** (operation,
  inputs, parameters, the settings it runs under). A **record** is one
  run. The records are the cache.
- Kinds, operations and promises are rows of a catalogue the suites
  give, which step 63's protocol writes down.
- Identity is by content, without a hash in the document.
- The cache's match is exact on operation, parameters, settings read
  and input values. A recipe holds its own settings.
- A node's state is computed, never stored. Running is no edit.
- The log holds macro actions as swaps; a session's steps are not in
  it. An entry names the records its change put out of date, so undo
  restores what was shown without a run.
- A check is apart from the run that made the output, and runs through
  the same records.
- On the page a workspace exists from the first link only, and only
  linked tabs are in it. One sequent is today's page.
- The list is the linked tabs unfolded, left of the stage, each under
  an inference line that names how it was made and carries the check.
- A link of version 3 holds the recipes and the proofs, never results.

### The panel

| member | model, effort | what it gave |
|---|---|---|
| proposer A (`deduxit-63g-propose-a`) | Fable 5.2, `high` | a whole design (735 lines), then a criticism of B (twelve witnessed objections) |
| proposer B (`deduxit-63g-propose-b`) | Opus 5.5, `high` | a whole design (860 lines), then a criticism of A (nine) |
| critic E (`deduxit-63g-critic-education`) | Opus 5.5, `high` | the five education skills, all loaded, on both first-time paths |
| critic F (`deduxit-63g-critic-frontend`) | Fable 5.2, `high` | the frontend-design plugin, loaded, on the cards, arrows and canvas, against the client's code |
| `deduxit-63g-inventory` | Sonnet 5.5, `medium` | every artifact and operation of the three suites with file and line |
| `deduxit-63g-sources` | Sonnet 5.5, `high` | comparable tools and research, with sources |

The proposers worked from the prompt and the code alone and did not see
each other's file. Before reading either, this session wrote down its
own position; where the merge follows that and not a proposal, the note
says S. Critic F read that position file, which lay in the panel's
folder, and says where it leaned on it (that only linked tabs are
cards); its other findings rest on the client's code.

What each proposed, and what was taken:

| point | A | B | taken |
|---|---|---|---|
| value apart from place | yes | yes (artifacts and cards) | both |
| hash in the document | none | none | both |
| what a node is | every artifact; cards a view | cards; artifacts beneath | A's nodes in the library, B's fewer things on the page |
| the calculus | in the statement's value | a parameter of each operation | A |
| edge classes derived, work, copy | yes | sessions and copies as special cases | A (B took it) |
| the cache's match | a finished run matches under larger bounds | exact on the settings read | B, by its witness in `decide.rs` |
| where the settings live | the caller's, at each call | the recipe's | B |
| "Update" in the log | no | yes | A (B withdrew) |
| what undo restores | by a cache lookup | entries pin what they showed | B's condition on A's log |
| the bounds | in `Settings` | in `Limits` | B, because the reader gets `Limits` |
| check and by-products | the check decides through the cache | `also`: by-products under their own keys | both |
| a workspace on the page | under every tab | from the first branch | B |
| a goal opened in a new tab | a copy, no panel | a branch that shows the panel | A (B took it) |
| what runs by itself | instant operations | nothing, by default | A, with B's rule that a check's cost counts |
| the bindings | sessions in a `work` map | `compute` and `accept` | both |
| the panel's side | left | right | left (E and F: Learn's frame is right) |
| the check's place | on the arrow | on the made card | the arrow (E and F) |
| the canvas | every artifact a box; no drag | cards; optional drag; a `tree` role | cards, no drag, a list's roles (E and F) |
| Learn | buttons stay | branching buttons hidden | A (B withdrew) |

What the critics changed beyond the table: the first-time example keeps
the verdict (E); a tab never shows a new sequent over an old proof, and
"Start again" is its one action (A, E and F together); the workspace's
undo is words named by the edit, in the panel, with no key (E and F);
the link above a made tab is drawn as an inference line (F); the graph
is a method panel in the stage's place, not the stage's camera (F);
arriving by a link is designed (E); the page says "linked tabs" (E).

### The sources

The survey's findings and their addresses are section 17 of the note.
The design takes: a record as a constructive trace and early cut-off
(Mokhov, Mitchell and Peyton Jones, "Build systems à la carte", read in
full); marking down and computing on request (Houdini, Salsa); naming
by content (Git's objects, Nix); a key that says what it leaves out
(marimo, Bazel); two modes of running in one setting (marimo, VsRocq,
coq-lsp); an outdated result kept on screen (Isabelle/PIDE, read in
full); a simple view first (marimo's app view, `git log --graph`,
Nielsen Norman Group); undo grouped by intent with results outside it
(tldraw, Vim); state in the fragment with the size said (the TypeScript
playground, Compiler Explorer); WCAG 2.2 and the ARIA patterns for the
list and the graph. The survey read most pages as summaries; what it
marked unconfirmed is not used as a reason.

### The reviewer's findings

`fresh-reviewer` on Opus 5.5 at `high` (the writer is on Fable), on
change `wxuzwskv`, judged against the prompt's fixed points, the
brief's contents, the first-time path, the note's consistency with
itself and its claims about the code. It found every checked claim
about the code true and the brief's contents all there, and nine
faults, each with its witness. All nine hold; change `xpsroztm` answers
them.

1. **Section 13.5 listed a goal among what is made again, though a goal
   is a copy.** Holds. The list names derived outputs only, and says a
   goal is never made again.
2. **A proof made through a search stayed "current" after its source
   changed.** Holds: a `work` node compared values only. It is now
   current only while every input is current and unchanged, and a row
   counts what stands between it and its source.
3. **A reader that lowers every recipe's bounds to its own undid the
   recipe's settings**: a card decided at twice the budget was
   undecided again after a reload. Holds. Reading changes no recipe;
   `Workspace::rebound` gives a document the caller's bounds, and the
   bindings call it for a link or an imported file only.
4. **Steps 3 and 5 of the first-time task showed more than the note
   said.** Holds. While a tab shows a proof of an earlier sequent its
   line shows the operation's name alone; "Update all" stands only
   while two or more rows wait for a run and starts no proof again; the
   inventory of step 3 names the menus and the status sentence.
5. **The one-sequent page was not today's**: the translation's sheet
   was to call the workspace before any link, under other settings.
   Holds. The sheets call what they call today; the first edge takes
   the sheet's settings and its check is made at the press, a second
   run of the two decisions. `workspace_try` is gone.
6. **Step 7's message could not appear**, the second tab not being
   shown. Holds. "Start again" shows the tab it starts again.
7. **A check could name a record the cache had dropped.** Holds. A
   record stays while the check of a kept record names it.
8. **`detach` of a node that never ran left a given node without a
   value.** Holds. It is refused there; on the page such a tab leaves
   the workspace and stays open, and a link's sequents are made at
   once.
9. **An "unknown" from a limit that no setting lifts had no place.**
   Holds. A run's end names the library's reason always and the
   setting where there is one.

### Departures from the prompt

- **The bound on edges is a bound on records (departs from the
  design).** The prompt asks for a refusal at a bound on nodes, edges,
  the log and bytes. A node has one recipe, so recipes can never pass
  the nodes' bound; the edges that grow are the records. The bounds are
  `nodes`, `records`, `log` and `bytes`.
- **The log forgets its oldest entry where the other bounds refuse
  (departs from the design).** Both proposers chose it independently:
  refusing an edit because the history is long helps nobody. A document
  whose log is past the bound is refused on reading.
- **The graph view draws the linked tabs, not every artifact.** The
  prompt asks for a node canvas as a second view. Every node is in the
  library and in the command's `workspace show`; on the page a box per
  artifact is the view both critics called overwhelming.
- **A workspace on the page holds linked tabs only.** The prompt's
  "one workspace" is met in the library and the command; on the page a
  tab that no link touches is no part of it, so that one sequent stays
  as it is and the workspace's undo cannot close a tab it never held.

### Decided here in the author's place

The planning session reviews these (section 18 of the note lists them
with their places):
1. `serde_json` becomes an optional dependency of the core crate,
   behind a feature `workspace` that is on by default.
2. One new control on today's page: "Duplicate this proof" in the Logic
   popover.
3. The page says "Workspace" for the button and "linked tabs" for what
   it lists.
4. No key for the workspace's undo in this step.
5. Closing a linked tab leaves the tabs made from it open, without
   their lines.
6. A statement's value holds its calculus.
7. An edge runs under its own settings; `workspace run` takes no limit
   flags.
8. "Update linked tabs by themselves" is off at first.
9. A link holds no results.

### Checks

- `nix build .#checks.x86_64-linux.typos .#checks.x86_64-linux.conventions`
  under the machine lock: passed at `wxuzwskv`, and again at this
  section's commit (the final message says so).
- No gate: no code changed.

### What the next sessions must know

**63a (the protocol).** The workspace takes its kinds and operations
from the protocol, in the library, not only in the bindings' JSON.
- The rows are sections 3.1 and 3.3 of the note: per kind its name,
  role, form, level and whether it is stored; per operation its name,
  input kinds, output kind, class, cost, parameters, the settings it
  reads, its promise with that check's cost, its label and its call.
  The names in 3.3 are the ones to write down.
- The library needs a trait a suite implements (kinds, operations,
  `read`, `summary`, `run`, `check`) and a registry of the suites of a
  build; section 11 sketches both as `deduxit::protocol`. The trait's
  final shape is 63a's. It should not be sealed.
- `describe` writes the catalogue as JSON, labels included; the page
  builds its menus and its rows from that.
- A statement kind's form is the suite's sequent with its calculus
  (`{"logic", "sequent", "text"?}`, `{"mode", …}`), a nested form of
  level 19.
- `ordinary.interpolant` is a new form made of readable parts. The
  level-16 documents and the cut elimination's record stay written
  only and are no kinds.
- Each operation's `reads` must be complete: the cache's key is built
  from it. `ordinary.decide` reads `limits.work`, `limits.occurrences`,
  `limits.memory_bytes`, `limits.derivation_bytes`,
  `limits.recursion_depth`, `search.copies`, `search.engine` and the
  ordinary options; 63a checks each list against the code.

**63g-b (the library and the command).**
- The module, its types and the reasons are sections 2 to 11; the
  command is 12.2. The first operations to wire are marked in 3.3.
- The cache's match is exact. Do not reuse a record across bounds: the
  ordinary default's evidence depends on them
  (`core/src/ordinary/decide.rs:258`).
- A value is written only if its kind's reader takes it back (an
  outcome valid by a clausal proof is not); renderings are never
  written; on reading, each record's check is made again where that
  needs no search, and `recorded` marks the rest.
- Reading changes no recipe. `rebound` is the one way a document takes
  the caller's bounds.
- The test against a reference: after any walk of edits, runs, undos
  and redos, every node that is `current` shows what its recipe gives
  on its inputs' values when run afresh, on one thread; undo and redo
  restore nodes, values, recipes and the records shown, exactly.
- `Limits` gains `workspace`; `Error::CODES` gains `workspace_limit`,
  `not_applicable` and `not_given`, none of kind `invalid`.
- `wire::LEVEL` becomes 19; `core/tests/rules.rs` and every test that
  compares a level with `LEVEL` is to be read again (63t's note to
  60b is the precedent).
- Measure what section 16 lists under 63g-b, and set the bounds'
  defaults from it.

**63g-c (the page).**
- Sections 13 and 14 are the page; F's file names the classes and
  tokens to reuse, which the note keeps in words. Two new tokens
  (`--size-workspace` 320 pixels, `--size-card` 240), one docking rule
  in TypeScript, no new media query.
- One sequent is today's page: no workspace call before the first
  link, the sheets as they are, the link of version 2 byte for byte.
  A walk in `tests/states.ts` holds it.
- The walks to add: the first link (docked, and the button alone on a
  phone); out of date, Start again, the two undos; the graph.
- The first-time task for the fresh agent is section 13.8, as written,
  at a desktop width and on a phone.
- Today a changed sequent replaces its proof without a word
  (`submit` calls `startText`); only a change of logic warns. The
  design relies on that and adds no dialog.
- Measure what section 16 lists under 63g-c, the storage first.
