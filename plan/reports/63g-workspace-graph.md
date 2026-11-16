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

## Session 63g-b: the library and the command

Opus 5.5 at `high`, the same worker as 63g-a and 63a (its conversation
kept, compacted once), in `../deduxit-wt-step-58a-610ed3`, from the
brief `/tmp/linlog-handoff/start-63g-b.md`, begun on `main@origin`
`xuuyqnsq` on 2026-11-16 and rebased onto `pktprmmq` (63c's report and
the plan's commits since) and then `upnmvust` (plan files only); no
conflict. Cores 7 to 9, every build and
test capped.

### Outcome

`deduxit::workspace` and `deduxit workspace …` exist: values linked by
the protocol's operations, the runs kept as an exact cache, the states
computed, the edit log with undo and redo, the bounds as refusals, the
JSON form at wire level 19, read back with what can be checked checked
again. The command keeps a workspace in a file; the manual has a
chapter with real invocations ("Keep a sequent and what is made from
it", `docs/src/workspace.md`), the guide a section (reference.md 11),
the changelog an entry (with 63a's protocol, which had none), the
behaviour lock four calls. The web bindings are untouched: nothing of
the page needs the type yet (63g-c's calls are section 12.1).

### The module

`core/src/workspace/` (`mod.rs` the model and the runs, `form.rs` the
JSON form, `tests.rs`), behind the feature `protocol`.
- Values are interned by kind and text; nodes are `Given`, `Derived`
  (a `Recipe`: operation, inputs, parameters as canonical JSON, the
  settings projected on the operation's `reads`, on one thread), `Work`
  (a session) or `Copy`. A record's `Key` is operation, parameters,
  settings and input value ids; `settle` points every node that is not
  current at a record of its key, never a foreign one.
- `State` is computed (`Given`, `Current`, `OutOfDate(Cause)`,
  `NotRun`); `Cause` is `Input`, `Recipe`, `Version` or `Waits`.
- `edit(name, limits, f)` is one entry of the log, taken back whole if
  `f` fails; an entry pins the records its nodes showed, so undo shows
  them again without a run. Eviction drops the oldest record no node
  shows and no entry pins.
- `compute`/`accept` split a run for a worker; `run` is both in plan
  order. `rebound`, `pack`, `layers`, `work` as section 11 says.
- `Limits::workspace` (`WorkspaceLimits`: nodes, records, log, bytes);
  `Refusal::Workspace { what: Held, count, limit }`, code
  `workspace_limit`, setting `limits.workspace.*`; `Error::NotApplicable`
  and `Error::NotGiven` (`not_applicable`, `not_given`), none `invalid`.
- `wire::LEVEL` is 19 (`wire::WORKSPACE`).

Departures, each marked in the code's rules or here:
- The module is behind the feature `protocol`, with no feature
  `workspace` of its own (departs from the design): it needs nothing
  `protocol` does not bring, and a second feature would only double the
  `cargo hack` pairs.
- `Bounds` is `limits::WorkspaceLimits`, its fields `Option<u64>` as
  every other bound of `Limits` (departs from the design: name and
  `u32`); `Held` names the bound in a refusal.
- `End::Limit` holds the protocol's `Limit`, `End::Refused` the
  library's error as its JSON text, since a record must read back from
  a file (departs from the design: `Box<Error>`).
- `protocol::Check::promise` and `Part::name` are `String`, so a check
  reads back from a file (departs from 63a's `&'static str`).
- `undo`/`redo` answer `Option<String>`; there is no `get::<T>` (a
  caller reads `value` and `wire::upgrade`s it) and no `check(node)`
  (`NodeView` carries it); `plan(targets)` takes no `What` and answers
  node ids; `accept(computed, limits)` takes no node, `Computed` names
  it; `pack(nodes)` takes no `Keep`, keeping recipes and sessions and
  never records; `Computed` is serde, not `wire::Readable` (departs
  from the design's sketch in these signatures).
- Reading a file runs every instant record again and drops one whose
  output differs, beyond making instant checks again (departs from the
  design: stricter). A search's check reads back `recorded`, as
  designed.

### The reference test

`core/src/workspace/tests.rs`,
`a_random_walk_keeps_current_nodes_fresh_and_undo_exact`: twelve seeds
of forty random moves (add a sequent, set a given sequent, connect a
translation by one of four, connect `ordinary.decide`, `embed` or
`start`, remove, undo, redo, run outputs or checks) over five ordinary
sequents. After every move each node shown `Current` equals
its recipe run afresh on one thread on its inputs' values
(`current_is_fresh`); an undo restores exactly the views before the
edit (values, states, recipes, records shown) and a redo those after.
Beside it, one test per behaviour: an upstream change and its undo, a
session following its sequent, the JSON form read back as written, a
forged record dropped on reading, every bound a refusal (nodes, bytes,
records after eviction, the log forgetting, a document past a bound on
reading), refused edits
leaving nothing, `rebound`, `pack`.

### The command

`deduxit workspace new|add|apply|set|run|remove|undo|redo|log|show|check|operations`
(`cli/src/workspace.rs`), the feature `protocol` on in the command. The
file is read through `wire::upgrade` within `limits.workspace` and
written whole, beside and renamed. Each node is listed as `N  KIND
SUMMARY`, then how it was made and its check, the bound that stopped
it, and its state when not current. `apply` and `run` exit with the
worst verdict run (a refused run or failed check 2, a stop 3);
`workspace check` answers for the checks alone (1 one fails, 3 one not
made), which a test pins.
- `apply` and `set` take the limit flags a node keeps; `run` and
  `check` take only `--timeout` and `--settings` (for
  `limits.workspace`), since a node runs under its own limits, as
  section 12.2 says. They first took every limit flag, ignored: fixed
  here, in a change of its own.
- Departures from section 12.2: the calculus is `--logic` or
  `--mode NAME` (default `classical`), not the mode flags; `--set` takes
  the operation's parameters only, settings come from the limit flags
  and `--settings`; there is no `set --all` (`rebound`), no `work` and
  no `run --restart` (the library has `rebound`, `work` and `restart`;
  the command can add them when a use asks). The listing's lines are
  the kinds' names and the protocol's summaries, not the design's
  example's words.

### Measurements (design section 16) and the defaults

On the debug and release builds, cores 7 to 9:
1. Bytes: an ordinary sequent's value 113 to 182 bytes, a session 206
   to 255, an outcome 1.65 to 2.44 KB; a workspace of three linked tabs
   (sequent, translation, decision, a session) 17 295 bytes. A link
   packed by `pack` (no records): one, two and three tabs 754, 911 and
   1 024 characters of base64url after deflate-raw (565, 683, 768
   bytes), well under 2 000.
2. Determinism: every ordinary method (embedding, LJT, SAT, table,
   tableau) and every linear engine gave the same output twice on one
   thread for the same key; the reference test relies on it.
3. A session's JSON is byte-equal when the same state is written twice.
4. One parse of the inputs per run: reading a sequent's value 1.2 µs
   against 27 µs for the cheapest decision; no typed side table.

Defaults (`WorkspaceLimits::default()`): `nodes` 1 024, `records` 4 096
(four runs kept per node, enough for an undo history of each), `log`
256 edits, `bytes` 64 MiB, the derivation bound's default, so the
largest outcome the library writes fits in a workspace. A typical
workspace is under 100 KB, so the command never meets them in use.

### Checks

- The gate, capped on cores 7 to 9, passed on the chain with the
  review's fixes (`rtpuzwtz`): clippy, the tests with the behaviour lock,
  both `cargo hack` runs, the ratchet (no ceiling passed; render-svg's
  cache misses moved by more than 5 %, a note, not a failure). Earlier
  runs caught `pack`'s missing `#[must_use]` and a web test that pinned
  the wire level at 18 (it now reads `deduxit::wire::LEVEL`), each folded
  into its change.
- `nix flake check --keep-going` under the machine lock passed at the
  head before the last rebase, whose new base changed only `plan/`.
  Earlier runs caught three protocol functions left unformatted and a
  redundant intra-doc link in the module's docs, both folded into their
  changes.
- The reviewer's witnesses run again on fresh copies with the debug
  build: a stopped run leaves the node not run and the next run decides
  it; another build's records stay out of date after a save and are
  replaced by a run; the undo restores the stale node's "Valid, out of
  date"; the `next`, `order` and `many` files read in an instant or are
  refused (`a recipe names a node that is not before it`, the bound on
  nodes); `workspace check` makes a recorded check `held`, and a forged
  `failed` one is made again and holds (exit 0).

### The reviewer's findings

`fresh-reviewer` once, on Fable 5.2 (the writer is Opus 5.5), with the
four criteria of the brief. It reproduced six findings with the debug
binary and two by reading, and found every criterion broken; all eight
are fixed in `rtpuzwtz`, each with a test in `tests.rs`, and its
witnesses run again on fresh copies give the right answers.
1. **A run the time limit or Ctrl-C stopped was kept and shown
   current** (criteria 1, 3): the searches answer a stop with an unknown
   outcome, which `compute` passed on and `accept` recorded under a key
   that holds no time. Fixed: `compute` watches the caller's stop and
   refuses such a run (`a_run_the_caller_stopped_is_refused_and_kept_nowhere`).
2. **Another build's record became current after any save, and a run
   could not replace it** (criteria 1, 3): `foreign` was one flag for the
   file, and `accept` reused any indexed record. Fixed: each record keeps
   `foreign` in the form, and `accept` replaces a foreign record in place
   (`another_build_s_record_stays_out_of_date_until_run_again`).
3. **Undo did not restore a stale node an edit's settle had moved**
   (criterion 2): settle re-pointed nodes the edit never touched, with no
   swap. Fixed: those moves are swaps of the edit's entry
   (`undo_shows_again_what_a_stale_node_showed`). The random walk never
   reached the state; the test is the reviewer's sequence.
4. **A 313-byte file made the reader allocate 16 GB** (criterion 4): a
   vector of `0..next` per swap. Fixed: gone; a log's node is checked by
   the replay below.
5. **The reader did not validate the log**: undo panicked on a swap of
   another node, and undo passed the bound on nodes (criterion 4).
   Fixed: each swap names its own node below `next`, and the reader
   replays the whole log both ways, checking that each swap stands where
   its node stands and each state's bound and inputs
   (`a_log_that_does_not_fit_its_nodes_is_refused_on_reading`).
6. **`workspace check` did not make a `recorded` check again, and a
   failed check read back as `recorded`**: fixed; `run` with checks makes
   again a check that is `recorded`, `failed` or `open`, and a failed one
   reads back failed
   (`a_recorded_check_is_made_again_and_a_failed_one_stays`).
7. **A check could answer from any kept record** (read only): a foreign
   one, or a search's output a file claimed. Fixed: only a trusted
   record (made by this build, or an instant one made again on reading)
   answers a check's runs (`a_check_answers_only_from_this_build_s_runs`).
8. **`rebound` was undone by undo** (read only): the log's recipes kept
   the document's bounds. Fixed: `rebound` rewrites them too
   (`rebound_reaches_the_recipes_undo_brings_back`).

Found while writing these tests: `serde_json::to_value` reorders a
value's keys, which makes it another value, so the forged-record test
passed whatever the forgery; the tests now edit the written text.

### What the next sessions must know

**63g-c (the page).**
- The library's calls for section 12.1 exist on `Workspace`: `edit`
  with `Edit::{add, set, connect, set_parameters, set_settings,
  restart, work, detach, remove}`, `undo`/`redo`, `compute`/`accept`
  (a `Computed` is serde both ways, for the worker), `run`, `nodes`
  (`NodeView`: id, kind, value, operation, from, parameters, settings,
  state, end, check), `layers`, `rebound`, `pack`. The bindings add the
  JSON calls; `web/` has nothing yet.
- Set the tab's `limits.workspace` lower than the command's: `bytes`
  about 2 MiB (three tabs are 17 KB; a link must stay a link), the rest
  as the command's. Every bound refuses with `workspace_limit` and names
  its key, which the page can show as it shows other limits.
- A `Work` recipe needs its input run first: connecting a session to an
  outcome that has no value yet is `not_applicable`. Run the input,
  then connect.
- `pack` keeps recipes and sessions, never records: a link opened runs
  again, so it never shows a verdict the opening build did not compute.
- Reading a document re-runs instant records; a search's check reads
  back `recorded`. A link from another version shows every result out
  of date (`Cause::Version`).
- `compute` refuses a run the caller's stop ended (`stopped`, kind
  `stopped`): the page's Stop leaves the node as it was, with nothing to
  `accept`.
- The reader replays the log against the nodes; a document the page
  edits by hand must keep its log's swaps true to its nodes, or drop the
  log.

**64 (the release).**
- `cli/tests/lock/inputs/workspace.json` names the deduxit version that
  wrote it: at the version bump its records become foreign and the
  lock's `workspace-show` output shows them out of date. Rebless it in
  the bump's own commit (or write it again with the new build).
- The marked list in `core.md` has the nine new types
  (`workspace::{Cause, Computed, End, NodeView, Ran, State, What}`,
  `limits::{Held, WorkspaceLimits}`); `Limits` gained a public field and
  `Refusal` and `Error` variants, which `breaks 0.4.0` lists: each is a
  bullet of the changelog's breaking list or explained as none (they
  are `#[non_exhaustive]`).
- `wire::LEVEL` is 19: a 0.4.0 reader refuses a workspace file, as it
  should.

## Session 63g-c: the page

Opus 5.5 at `high`, the same worker as 63g-a, 63a and 63g-b, from the
brief `/tmp/linlog-handoff/start-63g-c.md`, on 2026-11-16. deduxit in
`../deduxit-wt-step-58a-610ed3` on `main@origin` `muovpmwk`; the client
in its own jj workspace `../deduxit.github.io-63gc` (`step63gc`) on its
`main@origin` `wqlqqwyo`, built against this chain with
`--override-input deduxit
git+file:///home/tux/Projects/own/deduxit-wt-step-58a-610ed3` (never
`path:`); its `flake.lock` does not move until the planning session has
pushed the deduxit chain. Cores 7 to 9.

### Outcome

A visitor who presses "Prove it in ⟨logic⟩" on the translation sheet, or
"Duplicate this proof" in the Logic popover, gets two linked tabs: a
Workspace button in the top band, and a panel (docked left of the stage
where the window is 960 pixels wide and Learn is off, else a sheet from
the button) that lists the linked tabs, each made tab under an inference
line naming how it was made and the check of what that promised. A
changed sequent puts what was made from it out of date until Start
again (Update when the proof has no steps); the panel's Undo and Redo
take back whole actions by name; three linked tabs can be shown as a
graph; a linked tab's link (version 3) opens its whole linked group on
another page. Before the first link the page is as it was: no
document, no workspace call, no button.

### deduxit: the library and the bindings

- `ywswyost` **Make the instant runs a proof needs within the edit that
  starts it**: `Edit::ready` (in `connect` and `restart` of a proof or
  copy) makes each instant run its inputs need and points the node at
  the record by a swap of the edit, so "a translation and its proof" is
  one action, which undo takes back whole. Undo and redo settle every
  node but those their entry changed, which show exactly their state
  before or after (a fresh record of a node the entry touched no longer
  replaces what it showed before). A test (`a_proof_of_a_translation_is_made_in_one_action`).
- `mpwqoysw` **`Workspace::read_own`**: a document the program wrote and
  kept itself is read with every check of the claim reader but the
  runs, its records trusted and its checks as written. Through the
  claim reader on every stateless call, a search's check turned
  `recorded` after each call and ran again. A test of both readers.
- `nxlrwxnz` **The bindings** (`web/src/workspace.rs`, exported
  `workspace_…`): `statement`, `open` (the claim reader and `rebound`,
  for a link or a file), `edit` (an action `{"name", "edits"}` read
  whole before any edit, since the library's error variants cannot be
  built outside it; a node is a number or `{"made": K}`), `undo` and
  `redo` (with the sessions they changed), `compute` and `accept`, `run`,
  `view` (each node's summary, label, cost, class, state, end, check,
  column), `pack`. Every answer carries the document as a JSON string
  of its text: written again through `serde_json::Value`, or by
  JavaScript, a value's keys could come out in another order, which is
  another value, and every record dropped on the next read. The tab's
  `limits.workspace.bytes` is 2 MiB. Five tests under Node and natively.
- `qwzvqqpr` **A node an undo took away is left out of `work`** (the
  reviewer's first finding, below).

### The page (the client)

- `qtlzrsyv` the store (`src/islands/Workspace.tsx`: the document, the
  tabs' nodes, the view; every call queued), the panel, the button, the
  graph, "Prove it in" and "Duplicate this proof" as actions, the pure
  rows and words (`src/lib/workspace.ts`), the worker's calls, the
  styles and the tokens `--size-workspace` (320 pixels) and
  `--size-card` (240).
- `rxqmkwnt` link version 3 (`#v=3&u&m&s&n&w`) and arriving by it.
- `rqornqsw` the `linking` walk in `tests/states.ts` (linked, out of
  date, the graph, the sheet on a phone), run by axe and the target
  sizes; the client's rules section "The workspace of linked tabs".
- `zoqwuwxw` the review's fixes (below), and `xwvkwlsl` the rules'
  update.

Departures, each "(departs from the design)":
- The workspace's code is in the island's bundle, not a lazy chunk; it
  makes no call and shows nothing before the first link.
- A row has no menu (Translate…, Duplicate, Remove); Duplicate is in the
  Logic popover, removing is closing the tab.
- The interpolant's sheet has no "Prove the first part" and "Prove the
  second part".
- A made tab's sequent field is not read-only: a made tab's or a copy's
  own change unlinks it ("unlinking", which Undo takes back).
- No setting "Update linked tabs by themselves"; what needs no search is
  made again by itself after each action, a check waits for Check.
- Arriving by a link says "This link holds N linked tabs. The Workspace
  button shows them." in the shown tab's banner, with no "Show
  workspace" button, and runs no search or check when the panel opens.
- The shelf does not keep the workspace: a restored visit's tabs come
  back unlinked.
- No "This Undo takes back proof steps" for the likely slip.
- The graph's columns are by a row's link depth, without arrow keys,
  over the stage while shown.
- The link carries `n`, the shown tab's node.
- Closing a linked tab removes its node; the tabs made from it keep their
  lines to the sequent (no `detach`).
- The page's document keeps the tabs' sessions (12.1 kept them apart);
  `work` lays the tabs' sessions in before each call.

### The first-time task

A `general-purpose` agent on Sonnet 5.5 that had not seen the design,
given only the address and the task in a student's words (prove
`a ∨ ¬a`, translate and prove the translation, change the sequent to
`a → (b ∨ ¬b)` and have the translation follow, take the change back),
in the built-in browser at a desktop width. It reached every part. Its
path: the calculus mark, Ordinary and Classical, the sequent, ∨R and ¬R
by keys, Close goal; it looked in Export for the translation first,
then found "Translate into intuitionistic logic" in the Logic popover;
"Prove it in intuitionistic logic" gave the Workspace button and the
docked panel with "Glivenko's translation"; it proved the translation
by Close goal; it changed the first tab's sequent and read "Out of date.
It is now ⊢ ¬¬(a → (b ∨ ¬b)). This proof is of the earlier one. Undo
brings it back." with Start again; it pressed Start again and proved the
new translation; then Undo restart and Undo sequent change brought back
the first sequent with both proofs. Where it hesitated: the translation
behind the calculus mark (today's page); it expected the translated
proof to follow by itself rather than by Start again; the two undos by
name needed reading; one Undo seemed to do nothing for a few seconds
before it showed (the calls run one after another; no busy state on the
panel's Undo). Its frame was 400 by 300 in the pane's screenshots, so it
read the page's text more than its pictures.

### The cognitive-load check

`cognitive-load-analyser` on the same path: at the link, about five new
elements (a tab made from a tab, the line, its check, the panel's Undo,
the button), each where it is used; the status sentence and the panel's
first-time sentence say different things, so no redundancy. Out of date
is told three ways at once (the row's keyword and reason, the dashed
line, the button's warn style), each in its own place, which the reader
need not integrate. "Update" and "Start again" keep apart a proof
without steps from one with steps. No change made from it; the trial's
"seemed to do nothing" is the one extraneous load seen, and a busy state
on the panel's Undo is a follow-up.

### Checks

- deduxit: the gate (clippy, the tests with the behaviour lock, both
  `cargo hack` runs, the ratchet) passed on `nxlrwxnz`; the workspace's
  library tests (19) and the bindings' (5) passed at each later commit.
  `nix flake check --keep-going` under the machine lock failed the
  `budget` alone (the module at 3 670 553 bytes against 3 300 000;
  `main`'s at 3 166 704, measured by building its `deduxit-web`), which
  `oppkoukm` raises with the reason; it passes at the head.
- The client, against this chain: `lint` (Biome, `astro check`) clean;
  Vitest 667 passed (28 files); Playwright 300 passed (6.6 min), among
  them the eleven of `tests/workspace.spec.ts` and the `linking` walk
  under axe in both schemes at both widths and the target sizes (24 and
  44 pixels); its `nix flake check --keep-going` with the `git+file`
  override passed.

### The reviewer's findings

`fresh-reviewer` once, on Fable 5.2, with the four criteria of the
brief; it could not reach the built-in browser and drove the built page
with the client's Playwright. Criteria 2 and 3 broken, 1 held, 4 held
but for one gap; six findings, all reproduced and all fixed, each with a
test in `tests/workspace.spec.ts`:
1. **Undo of an action that made a tab failed and left the workspace
   dead**: the page sent the made tab's session for a node the undo took
   away, which the bindings refused, the document undone and the view
   not. Fixed in deduxit (`qwzvqqpr`: such a node is left out of
   `work`) and on the page: after an undo a tab whose node is gone is
   closed if the workspace opened it, else unlinked and parked, and
   relinked when Redo brings its node back.
2. **Undo of a sequent change or a closing lost the steps taken since
   the last workspace action**: the document held the session as of the
   last call. Fixed: `latest` keeps each linked tab's latest session,
   laid in before the change or the closing.
3. **A change of a typed tab's logic was not seen**, the line keeping a
   check of the classical statement as current. Fixed: the change is
   keyed by logic and sequent, and a logic change is an action of its
   own, "logic change".
4. **Editing a copy rewrote the sequent it shares with its original**,
   putting the original out of date. Fixed: only the tab that owns a
   given sequent changes it; a copy's or a made tab's own change unlinks
   it.
5. **A link held only the shown tab's neighbours**, dropping sibling
   tabs. Fixed: the whole linked group.
6. **Unlinking was logged as "closing"** and its undo opened a stray
   tab. Fixed: "unlinking", whose undo relinks the same tab once it
   shows the session back.

### What the next sessions must know

**64 (the release).**
- The client's `flake.lock` moves to deduxit's `main` once this chain is
  pushed; until then the client's commits build only with the
  `git+file` override.
- The bindings' answers carry the workspace's document as a string; a
  front end that parses it and writes it again breaks every record.
- The library gained `Workspace::read_own` and the `Edit::ready`
  behaviour, both under the unreleased `workspace` module; the
  changelog's workspace entry covers them.
- The departures above are the page's list of what 0.5.0's workspace
  lacks against the design; the shelf's version 3 (autosave) is the one
  a returning visitor meets first.

**65d (the modal page).**
- A new suite's tabs join the workspace through its suite's kinds: a
  given statement (`workspace_statement` with the suite's name), a work
  node of its start operation, and the page's `rowsOf` reading a work
  node's input chain. `src/lib/workspace.ts` names `ordinary.start` and
  `ordinary.sequent` today; a modal suite adds its names there.
- The workspace's effects compare each linked tab's logic and sequent
  (`keyOf`); a suite whose calculus has another parameter (a modal
  system) adds it to the key, or a change of it goes unseen.
