# The workspace graph: the design (step 63g, session 63g-a)

Written on 2026-11-16 by session 63g-a (Fable 5.2 at `high`) from a
panel's proposals and criticism, for `plan/63g-workspace-graph.md`.
Design only: no code lands with it. Sessions 63a (the protocol), 63g-b
(the library and the command) and 63g-c (the page) build from this note;
"What the next sessions must know" in the step's report
(`plan/reports/63g-workspace-graph.md`) says what each takes.

Marks: **[V]** was read in the code by this session; **[M]** needs a
measurement, named in section 16. A choice says whose it is: **A** is the
proposal on Fable 5.2, **B** the proposal on Opus 5.5, **E** the critic
with the education skills, **F** the critic with the frontend-design
plugin, **S** this session.

## 1. The panel

| member | model | what it did |
|---|---|---|
| proposer A | Fable 5.2, `high` | a whole design from the prompt and the code, then a criticism of B |
| proposer B | Opus 5.5, `high` | the same, independently, then a criticism of A |
| critic E | Opus 5.5, `high` | the education skills (`cognitive-load-analyser`, `worked-example-fading-designer`, `explicit-instruction-sequence-builder`, `ai-feedback-design-principles`, `udl-barrier-anticipator`; all five loaded) on both first-time paths |
| critic F | Fable 5.2, `high` | the frontend-design plugin on the cards, the arrows and the canvas of both, against the client's code |
| two readers | Sonnet 5.5 | an inventory of each suite's artifacts and operations with file and line; a survey of comparable tools with sources (section 17) |

The two proposals agreed, without seeing each other, on the frame: an
immutable value apart from the place that holds it, no hash in the
document, kinds taken from the protocol, results as a cache of runs,
staleness computed and never stored, running kept out of the edit log
(B after A's criticism), a list docked beside the stage, a link of a
third version that an older page still half reads. They differed on
twelve points, which the cross-criticism and the critics settled; each
is named where it is decided below.

## 2. The model in brief

- A **value** is an artifact: a kind and its JSON. Values are immutable
  and interned: equal JSON is one value.
- A **node** is a place with a number that is never used twice. It is
  *given* (the user gave its value) or it has a **recipe**: an
  operation, its input nodes, its parameters and the settings it runs
  under. An **edge** is a recipe seen from its inputs; every node has at
  most one, so an edge is named by its node (A).
- A **record** is one run of a recipe on values: what went in, what came
  out, how it ended, and the check of what the operation promises. The
  records are the cache.
- A node's **state** (current, out of date, not run) is computed from
  its recipe, its inputs' values and the records. Nothing stores it.
- The **log** holds the edits of nodes and recipes, each a macro action.
  Running is no edit. A proof's steps are no edits.
- Kinds, operations and promises are names from the suites'
  **catalogue**, which step 63's protocol writes down. The workspace
  module has no list of them.
- On the page a **card** is a few nodes shown as one thing: a sequent
  with its proof. Only linked tabs are cards.

The prompt's two fixed points, "artifacts are immutable nodes" and "a
change upstream marks what follows stale", meet in the split of value
and place (A and B alike, and S before reading them): the value never
changes; an edit gives the place another value, and the old one stays
while the log or a record names it. Duplicating forces the same split:
two cards may hold one value and must stay two things.

In the prompt's words: the immutable nodes are the values, the edges
with their parameters are the records between them, and the recipes are
what the user arranged. `git` has the same three: objects, and refs that
move (section 17, Git objects; B named jj's commits and bookmarks).

## 3. Kinds and operations

### 3.1 Where they come from

Each suite gives a catalogue: rows for its kinds and its operations and
one function that runs an operation on JSON. The workspace stores names
and looks each up; a later suite adds rows and nothing in the workspace
changes (A and B alike). Step 63's `describe` call writes the catalogue
as JSON for the page, which builds its menus and card faces from it.

A kind's row:

| field | what |
|---|---|
| `kind` | its name, `suite.name`; a kind several suites share has no prefix |
| `role` | `statement` (a sequent with the calculus it is claimed in), `definition` (a rule file), `work` (a session: the user's steps), `evidence` (an outcome, a proof, a derivation, a net, an interpolant), `rendering` (a drawing, an export, a certificate, a term) |
| `form` | the wire form its value is written in, without `version` |
| `level` | the wire level at which the form joined |
| `stored` | whether a document holds its values; `false` for every rendering, which is made again when asked for |

An operation's row:

| field | what |
|---|---|
| `operation` | its name, `suite.name` |
| `from` | for each input in order, the kinds it takes |
| `to` | the kind of its one output |
| `class` | `derived` (a function of its inputs: it goes out of date and is run again), `work` (starts from its inputs, then holds the user's steps: it goes out of date and is never replaced unasked), `copy` (starts as a copy and is the user's from then on: it never goes out of date; the edge only says where it came from) |
| `cost` | `instant` (linear in its input, meets no limit but the size refusals) or `search` (may meet a limit) |
| `parameters` | the options form of its own parameters, with defaults |
| `reads` | the settings keys its result depends on |
| `promise` | what it promises of its output (section 9), and that check's own `cost` |
| `label` | its name in plain words, for a card and the command ("Glivenko's translation") |
| `call` | the binding that reaches it today |

The classes are A's (B took them). `label` and a `summary` function (a
kind's one line for a list: a sequent's text, a verdict's keyword) are
B's (A took them): without them the page would keep a table of words
per operation, which is a list of its own.

Rejected: an `enum` of kinds in the workspace module (both): the list
the prompt forbids. Operations with several outputs (A): with one output
a node has one recipe, and an interpolant's halves are reached by
`ordinary.half`.

### 3.2 The statement carries its calculus

A statement's value is the sequent with the calculus it is claimed in:
`{"logic": "classical", "sequent": {…}, "text": "⊢ a ∨ ¬a"}` for the
ordinary suite, `{"mode": …, "sequent": …, "text": …}` for the linear
one. `sequent` is the suite's own wire form and is what counts; `text`
is what was typed, kept for the field, and may be absent.

This is A's, against B, who kept the calculus as a parameter of every
operation because the library's sequents carry none [V,
`core/src/wire.rs`, the forms]. Reasons:
- A tab's calculus is one thing on the page (`Proof.calculus` [V,
  `state.tsx:182`]; D57), and a verdict is of a sequent in a logic.
- With the logic on each operation, `ordinary.translate` outputs a bare
  sequent and "into intuitionistic logic" lives only on the edge; a
  later `decide` given `logic: classical` answers for a card whose
  label says intuitionistic, and nothing is wrong by the rules (A's
  witness).
- A change of logic is one edit of one value, and what follows goes out
  of date by the ordinary rule.

B's objections are met: the value holds the parsed form, so no setting
decides how a text is read later, and the same sequent claimed in two
logics is two statements, one made from the other by a duplicate whose
logic the user then changes.

### 3.3 The suites' rows

"Today" names the binding or the library function that does it now
[V, the inventory against `web/src/lib.rs`, `core/src`]. **First** marks
what 63g-b wires; the others are named for the protocol and join when a
front end needs them.

**Linear** (`deduxit::linear`).

| kind | value | role |
|---|---|---|
| `linear.sequent` | `{"mode", "sequent", "text"?}`, the sequent a `Sequent` or an `IllSequent` (told by `hypotheses`, as the bindings tell them) | statement |
| `linear.session` | `Interactive`'s session form | work |
| `linear.outcome` | `search::Outcome` | evidence |
| `linear.proof` | `Proof` | evidence |
| `linear.net` | `ProofStructure` or `MellStructure` | evidence |

| operation | from → to | class, cost | promise | today | first |
|---|---|---|---|---|---|
| `linear.prove` | sequent → outcome | derived, search | `decides` | `prove` | yes |
| `linear.start` | sequent → session | work, instant | `exact` | `start` | yes |
| `linear.goal` (`path`) | session → sequent | copy, instant | `exact` | the page's `openGoal` | yes |
| `linear.proof` | session → proof | derived, instant | `proves` | `proof` | yes |
| `linear.net` | proof → net | derived, instant | `proves` | `draw_net`'s structure | |
| `linear.eliminate` (`at`) | proof → proof | derived, search | `proves` | `eliminate` | |
| `linear.term` | session or proof → `term` | derived, instant | `renders` | `term` | |
| `linear.draw` | session, proof or net → `drawing` | derived, instant | `renders` | `draw`, `draw_net` | yes |
| `linear.export` (`format`) | session or proof → `text` | derived, instant | `renders` | `export` | yes |
| `linear.certificate` | proof or outcome → `certificate` | derived, instant | `certifies` | `export` (rocq), `certificate` | yes |

**Ordinary** (`deduxit::ordinary`).

| kind | value | role |
|---|---|---|
| `ordinary.sequent` | `{"logic", "sequent", "text"?}` | statement |
| `ordinary.session` | `ordinary::Interactive`'s form | work |
| `ordinary.outcome` | `ordinary::Outcome`, its countermodel, table or tableau inside it | evidence |
| `ordinary.derivation` | `ordinary::Derivation` | evidence |
| `ordinary.interpolant` | `{"logic", "partition", "formula", "first", "second"}`: the interpolant's text and its two halves as sequents | evidence |

| operation | from → to | class, cost | promise | today | first |
|---|---|---|---|---|---|
| `ordinary.decide` (`method`, `translation`) | sequent → outcome | derived, search | `decides` | `ordinary_close` on the root, `ordinary_table`, `ordinary_tableau`; `prove --logic` | yes |
| `ordinary.start` | sequent → session | work, instant | `exact` | `ordinary_start` | yes |
| `ordinary.goal` (`path`) | session → sequent | copy, instant | `exact` | `openGoal` | yes |
| `ordinary.derivation` | session or outcome → derivation | derived, instant | `proves` | inside `ordinary_draw` | yes |
| `ordinary.translate` (`translation`, `target`) | sequent → sequent | derived, instant | `same_verdict`, a search | `ordinary_translate` | yes |
| `ordinary.interpolate` (`partition`, `method`) | sequent → interpolant | derived, search | `interpolates`, a search | `ordinary_interpolate` | yes |
| `ordinary.half` (`which`) | interpolant → sequent | derived, instant | `exact` | none | yes |
| `ordinary.embed` (`translation`) | sequent → `linear.sequent` | derived, instant | `same_verdict`, a search | `ordinary::translate` (library) | |
| `ordinary.term` | session or derivation → `term` | derived, instant | `renders` | `ordinary_term` | |
| `ordinary.draw` | session, derivation or outcome → `drawing` | derived, instant | `renders` | `ordinary_draw`, `ordinary_model_drawing` | yes |
| `ordinary.export` (`format`) | session or derivation → `text` | derived, instant | `renders` | `ordinary_export` | yes |
| `ordinary.certificate` | derivation or outcome → `certificate` | derived, instant | `certifies` | `ordinary_export` (rocq), `ordinary_model_certificate` | yes |

**Calculi given as data** (`deduxit::rules`).

| kind | value | role |
|---|---|---|
| `rules.calculus` | the rule file, or `{"built_in": "LK"}` | definition |
| `rules.sequent` | `{"goal", "assumptions"}`, texts read under a calculus (B's question 9: there is no parsed form outside a session) | statement |
| `rules.session` | `rules::Interactive`'s form | work |
| `rules.derivation` | `rules::Derivation` | evidence |

| operation | from → to | class, cost | promise | today | first |
|---|---|---|---|---|---|
| `rules.start` | calculus, sequent → session | work, instant | `exact` | `rules_start` | yes |
| `rules.derivation` | session → derivation | derived, instant | `proves` | `rules_check` | yes |
| `rules.from_ordinary` | `ordinary.derivation` → derivation | derived, instant | `proves` | `Derivation::from_ordinary` | |
| `rules.draw` (`layout`) | session or derivation → `drawing` | derived, instant | `renders` | `rules_draw` | yes |
| `rules.export` (`format`) | session or derivation → `text` | derived, instant | `renders` | `rules_export` | yes |

**Shared.** Kinds `drawing` (SVG text), `text` (`{"format", "text"}`),
`certificate` (`{"kernel", "text"}`), `term` (the term's text and
judgement), all renderings and so never stored. One operation,
`duplicate` (a statement or a definition → the same kind, class copy).
PNG and PDF are not operations: they need the caller's fonts and, for
PDF, a date, and are made from a `drawing` by `render` as today.

What has no row, and why: the Kripke model alone (it is inside its
outcome, and the drawing and the certificate take the outcome, as
`ordinary_model_drawing` does [V]); the comparison of translations and
the interpolation of wire level 16 (written only [V]; the graph has
their parts as values instead); the record of a cut elimination's steps
(written only; a rendering if a front end wants it).

**A later suite** adds rows only. Step 70: `ordinary.nnf`,
`ordinary.cnf` (sequent → sequent, `equivalent`), `ordinary.tseitin`
(sequent → `ordinary.clauses`, `equisatisfiable`), `resolution.refute`.
Modal logic: `modal.sequent`, `modal.decide`. First order: the kinds
keep their names and the forms grow as the wire levels say.

## 4. Identity and immutability

- **A value's identity is its kind and its JSON** as deduxit's writers
  write it, which is deterministic. The workspace interns values in a
  table keyed by that text under the crate's fixed-seed hasher [V,
  `core/src/hash.rs`]; equality is of the bytes, so a collision costs
  time and never names another value. In a document a value is its
  index.
- **No hash is written** (A, B and S alike). A short hash lets a crafted
  document make two values collide, and the cache then answers for the
  wrong sequent; a cryptographic one is a dependency whose function
  would be promised for ever, and 64 characters a value in a link of
  2000. Two texts of one sequent are two values: that costs a run, never
  a wrong answer. The survey's advice to name nodes by content hash
  (section 17) is taken as identity by content, without the hash.
- **A node's identity is its number**, a `u32` given in order and never
  used twice, also not after a removal or an undo; the document keeps
  the next one.
- **A session is a `work` node.** Its value is the session's saved
  state; each step gives the node its next value without a log entry,
  and the session's own undo takes steps back. The node remembers the
  input values it was started from, so "this proof is of an earlier
  sequent" is a comparison of two indices.
- **Early cut-off.** A run whose output is the value the node had
  changes nothing below it (Build systems à la carte, section 17): a
  parent's next proof step leaves a goal opened from it as it was.

## 5. The cache

### 5.1 The key, and where the settings live

A record's key is the operation, its parameters with defaults filled
in, the settings it `reads`, and its input values. The match is exact.

- **Exact, not "under equal or larger bounds".** A proposed that a run
  which ended by itself matches under any larger bound, and S's first
  position was the same. B refuted it in the code: the ordinary
  default's evidence depends on the bounds when no bound is met.
  `with_evidence` looks for a derivation within the smaller of
  `limits.work` and 2²⁰ [V, `core/src/ordinary/decide.rs:258`], so a
  run at a small budget holds a clausal proof where a fresh run at a
  larger one holds a derivation, with a drawing, a certificate and a
  term the first cannot give. A narrowed its rule and S drops it: a
  cached answer must be what a fresh run gives, which is what 63g-b's
  test against a reference compares.
- **The recipe holds its settings** (B). When an edge is made, the
  workspace keeps the caller's values of the keys the operation reads;
  the edge runs under those from then on. A change of the page's
  Settings or of the command's flags touches no existing edge, so
  raising the budget for one card does not put every decided card out
  of date (A's worry about an exact key), and a workspace file says
  under what each result holds, whoever reads it. Changing an edge's
  settings is an edit (`set_settings`): the card's "Try 40 units".
- **Every run stays within a caller's limits.** A recipe's bounds come
  from the caller that made the edge, or from the caller that took the
  document over: `Workspace::rebound` gives every recipe the caller's
  `limits.*` values and drops the records that then no longer match. A
  caller that reads a document it did not write calls it before it
  runs anything. The bindings' `workspace_read`, which reads a link or
  an imported file, does, so a link cannot ask a browser tab for a
  gibibyte and need not carry bounds at all. Reading alone changes no
  recipe: the page's own document, read back from the browser's
  storage or between two calls, keeps the bounds each edge was given.
  (The first draft had the reader lower every bound to its own; the
  reviewer's witness: a card tried again with twice the budget and
  decided would be undecided again after a reload.)
- **What is not in the key**, said as marimo's documentation says its
  own (section 17): the build (deduxit's version and the target's
  pointer width, on which the memory count depends) and the number of
  threads. The workspace runs every search on one thread
  (`Settings::deterministic()` [V]), where a result is a function of
  the key.

### 5.2 How a run ends

- `done`: it ran to its end, whatever the verdict.
- `{"limit": {"reason": "work_limit", "setting": "limits.work",
  "value": 67108864}}`: a bound ended it. `reason` is the name the
  library gives it (`Reason`'s, `Refusal`'s code) and is always there;
  `setting` is the settings key that lifts it, from `Reason::setting()`
  and `Refusal::setting()` [V, `search/mod.rs:3509`, `limits.rs:540`],
  and is absent where no key does (an index space that is full, the
  memory the machine has left): the card then names the limit and
  offers no "Try". The
  output is still kept (an outcome with `"verdict": "unknown"`). The
  record matches its key like any other: the run is deterministic, so
  the same key gives the same unknown. Raising the bound is a change
  of the edge's settings, so the node is out of date and runs again.
- `{"error": {…}}`: the operation refused (the library's error JSON:
  `open_goals` for `linear.proof` on an unfinished session, a size
  refusal). Deterministic, so kept and matched (B).
- A **stop** (the command's `--timeout`, Ctrl-C, the page's Stop) makes
  no record. The node stays as it was, and the caller says "Stopped".
  Time is not in the key, so a kept time-out would be wrong on a faster
  run (both).

### 5.3 What a document holds, and what a reader keeps

- **Written**: every value of a stored kind that its kind's reader
  takes back, and the records whose values are written. A rendering is
  never written. A value its own reader refuses is not written either:
  an ordinary outcome valid by a clausal proof is one [V,
  `core/src/serialize/ordinary.rs:730`], which both proposals' example
  documents held and neither could have read back (each found it in the
  other).
- **Read**: every value goes through its kind's reader within the
  limits. Every record's check is made again where that needs no
  search (a derivation's or a proof's checker, a model's check, a
  session's own reader, an interpolant's atoms). A record whose check
  fails is dropped. One whose check would need a search (an outcome
  "unprovable" by an exhausted search, which `Disproof::check` calls
  uncertified [V]) is kept with the status `recorded` and is shown so.
- **Another version's records** are kept and shown, and their nodes are
  out of date with the cause `version` (B; A took it), since the build
  is not in the key.
- **A link holds no records** (B): results are made again on the page
  that opens it, so a link cannot show a verdict deduxit did not
  compute there.

### 5.4 Which records stay

A record stays while a node shows it, a log entry names it (section
7), or the check of a record that stays names it as a part. The others
are the cache proper: kept for the next undo, redo or
typed-back sequent, and dropped oldest first at the bounds (section 8).
The order is by a counter of edits, so it is the same on every run.

## 6. States and running

### 6.1 A node's state

Computed in one pass over the nodes, inputs first (they are kept in
that order):

| state | when | shown value |
|---|---|---|
| `given` | no recipe, or class `copy` | its own |
| `current` | a record matches the recipe on the inputs' values, and every input is `given` or `current` | the record's output |
| `out_of_date` | it shows a record that no longer matches, or an input is not current | the last record's output |
| `not_run` | it shows no record | none |

`out_of_date` has a cause (B): `input` with the nearest node whose
value changed, `recipe` (its own parameters or settings changed),
`version`, or `waits` with the node above that is not run or whose
check failed. A `work` node is `current` while every input is `given`
or `current` and holds the value the node was started from; else it is
`out_of_date`, with `waits` where an input is not current and `input`
where an input's value changed. So a proof made through an interpolant
is out of date while the interpolant waits for its search, not only
once the half has changed.

A flag per node could disagree with the values; a function of them
cannot (both; the survey's Blueprint "dirty" icon that sticks is the
failure avoided).

### 6.2 What the library does and what the caller does

The library answers and runs on request; the caller decides when.

- `state`, `plan` (what would run for some nodes, in order, each a
  cache hit, an instant run or a search) and `layers` are pure.
- `settle` points every node at a record that matches, where one
  exists. It costs no run; after an undo it makes what follows current
  again.
- `compute` runs one node's recipe, or its promise's check, and returns
  the result without changing the workspace; `accept` puts a result in
  if its key is still the node's (B's pair, with A's condition). A
  worker that is stopped mid-run then loses nothing, and an answer is a
  few kilobytes. `run` is both for one caller in one thread.
- A `work` node is never run again by `run`. `restart` is an edit.
- A node below a failed check or a refused run is not run (B).

The command runs what an `apply` made, and the rest on `workspace run`.
The page's rule is section 13.5.

## 7. The edit log

- **An entry is one macro action**: everything one intent of the user
  changed, made in one transaction that the caller names ("translation",
  "sequent change", "copy", "restart", "removal"). Inside it are the
  primitive edits: `add` (a given node), `set` (a given or copied
  node's value), `connect` (a node with its recipe), `set_parameters`,
  `set_settings`, `restart` (a `work` node started again from its
  inputs' values now), `detach` (a node keeps its value and loses its
  recipe: it becomes given; refused, `not_applicable`, for a node that
  shows no value), `remove` (a node with all that follows
  it). One transaction per user action is how tldraw's marks and Vim's
  undo blocks group (section 17).
- **Not edits**: running, settling and checking (records are a
  function of the graph), and a session's steps. This is A's; B had
  made "Update" an entry and withdrew it when A showed its own example
  ending in an undo that restores nothing.
- **An entry stores swaps**: a node inserted or deleted whole, a value,
  parameters or settings exchanged. Undo and redo apply the same swaps
  and keep what the graph held, so a card that undo removes comes back
  by redo with the proof it had when it went (both).
- **An entry names the records** that the nodes below its change showed
  before it (B's condition on A's design). Undo points those nodes back
  at them, so what was proved before the change is proved again after
  the undo without a run, whatever the cache dropped meanwhile.
- **A new edit drops the entries after the position.** No undo tree
  (both rejected it: a second dimension of history is one more thing to
  learn; the survey's Vim branches are for experts).
- **The bound**: at most `limits.workspace.log` entries; a new edit past
  it forgets the oldest, with the records only it named. Reading a
  document whose log is longer is refused. Section 8 says why this
  bound does not refuse an edit.
- **A link's receiver gets no log** (section 14).

## 8. The bounds and their refusals

`Limits` gains `workspace` (keys `limits.workspace.*`). The bounds are
in `Limits`, not in `Settings` beside it as A first had them, because
the one reader of every form gets `&Limits` and nothing else [V,
`wire.rs`, `Readable::read` and `upgrade`] (B; A agreed).

| bound | counted | library and command | a browser tab | past it |
|---|---|---|---|---|
| `nodes` | nodes alive | 1 024 | 128 | the edit is refused |
| `records` | records kept | 4 096 | 512 | cache records dropped, oldest first; refused if those that must stay pass it |
| `log` | entries | 256 | 64 | the oldest entry is forgotten |
| `bytes` | the JSON length of every value held, renderings in memory included | 64 MiB | 8 MiB | renderings dropped, then cache records, then the oldest log entries; refused if what is left passes it |

The numbers are first guesses [M]. A refusal is
`Error::Refused(Refusal::Workspace { what, count, limit })`, code
**`workspace_limit`**, kind `limit`, `setting()` the key, so the command
and the page name what to raise as for every bound. Reading a document
past any of the four is refused before anything runs, and every sequent
in it is counted against `limits.occurrences` first, as `wire::Within`
does.

Two readings of the prompt's "bounded (nodes, edges, the log, its
bytes) with a refusal at each bound", both marked as departures in the
report:
- **Edges.** A node has one recipe and a recipe has as many inputs as
  its operation's row says, two at most today, so the edges of the
  recipes can never pass the nodes' bound (B's point against A's
  `edges`). The edges that do grow are the records, the edges between
  the immutable values, and the bound is on them.
- **The log** forgets its oldest entry where the other bounds refuse
  (A and B alike). Refusing to edit a workspace because its history is
  long helps nobody, and every editor's history is bounded this way
  (Vim's `undolevels`, section 17). The refusal is at reading a
  document whose log is past the bound.

A session's step that makes the values held pass `bytes` after
everything droppable is dropped is refused. On the page the sessions of
open tabs stay with their tabs as today (section 12.1) and are not held
by the workspace between calls, so a proof step is never refused for
the workspace's size (B's witness against A's megabyte).

Other new codes: `not_applicable` (kind `malformed`: the operation does
not take these kinds; the message names what it takes) and `not_given`
(`malformed`: a value set on a node with a recipe of class `derived` or
`work`). The kinds are `malformed`, never `invalid`, which alone says a
claim is wrong and makes the command exit 1 (B, from `core.md`). An
unknown kind, operation or promise is the existing `unknown_name`; a
node number the workspace lacks is `index_out_of_bounds`; a document
whose nodes are not inputs first is `not_topological`. No cycle can be
made by an edit: a recipe names nodes that exist. Undo with nothing to
undo is `None`, no error.

## 9. Promises and their checks

A promise is a name in an operation's row; a record stores it with the
check's result, and both are shown on the arrow (section 13.3).

| promise | what it claims | the check | its cost |
|---|---|---|---|
| `exact` | the output is read off the input (a goal, a half, a start) | none needed | |
| `decides` | the verdict has its evidence | the evidence's own checker: `Proof::check`, `Disproof::check`, `Derivation::check`, `Countermodel::check`; a clausal proof was checked when made [V] | instant |
| `proves` | the derivation proves its sequent | the calculus's checker, `rules::Derivation::check` for a rule file | instant |
| `same_verdict` | input and output are valid together or not | both decided, each by its logic's default method, and the verdicts compared, as `negative::compare` does [V] | a search |
| `interpolates` | the three conditions | the atoms occur in both parts (instant); each half decided valid, as `interpolant::checked` does [V] | a search |
| `equivalent`, `equisatisfiable` (step 70) | as named | both decided and compared | a search |
| `renders` | the output shows the input | none here; the `export` flake check compiles the formats | |
| `certifies` | a kernel accepts the certificate | outside deduxit (`nix run .#check-certificate`); deduxit checked the proof it wrote | |

**A check is apart from the run.** `ordinary.translate` makes its
sequent at once, and its check decides twice. A proposal that folds the
check into the run must either call a translation a search, so that
nothing about it is ever instant, or start two searches whenever an
upstream edit makes it again (B's witness against A). So a record has
its output from the run and its check from a second call, which may be
absent: "not checked yet".

**A check runs through the cache.** The check of `same_verdict` asks
the workspace for `ordinary.decide` on the input and on the output;
each is a record like any other, made or found. The translation's
decision then costs nothing a second time when the user adds it as a
node, and a check finds what the user already ran. An operation may
also hand over by-products under the keys their own operations would
have (B's `also`; A took it): `ordinary.interpolate` decides both
halves on its way and gives those outcomes, so its check finds them.

Check statuses, by name:

| status | meaning |
|---|---|
| `held` | made, and it holds |
| `failed` | made, and it does not hold: a fault in deduxit |
| `open` | a part was not decided; the part's record names the limit |
| `recorded` | held when made; this document holds nothing to make it again by |
| `outside` | made by another program |
| `none` | nothing to check |

**A failed promise is stored, never thrown away.** The record has
`"status": "failed"` and the parts that disagree, each naming its
record, so both outcomes are in the document. `run` and `compute`
answer `Ok` with it; `Ran::into_result()` gives today's
`Error::ClaimFails` (kind `defect`, code `claim_fails` [V]), on which
the command exits 2. Nothing below a failed check is run.

## 10. The JSON form, wire level 19

`wire::LEVEL` becomes 19: "a workspace". The writer computes a
document's level as the greatest of 19 and its values' own levels, so a
kind a later level adds raises the document, and an older reader
refuses it by version first. Nested values carry no `version` and no
`deduxit`. Every enumeration is a name. The reader refuses an unknown
kind, operation, promise, status, way of ending or swap by name, and a
`parameters` or `settings` key the operation's row does not list; other
unknown keys it ignores, as every data form does. It checks: node
numbers rising and below `next`, indices in range, inputs before their
nodes, kinds fitting the rows, the bounds.

Keys: `version`, `deduxit` (the build that wrote it), `values` (each
`kind` and `value`), `nodes`, `records`, `log`, `next`. A node is `id`
with either `value` (given), or `operation`, `from`, `parameters`,
`settings` and then `record` (class `derived`: the record it shows), or
`value` and `started` (class `work`: its session and the input values
it was started from), or `value` (class `copy`).

`⊢ a ∨ ¬a` in classical logic, Glivenko's translation, the translation
decided, and a certificate asked for. The sequents' and outcomes' forms
are those the command writes today [V, `seq translate --format json` on
the build of 2026-11-16]; `…` cuts a derivation's inferences.

```json
{
  "version": 19,
  "deduxit": "0.5.0",
  "next": 4,
  "values": [
    {"kind": "ordinary.sequent", "value": {"logic": "classical", "text": "⊢ a ∨ ¬a",
      "sequent": {"formulas": [{"V": 0}, {"¬": 0}, {"∨": [0, 1]}],
                  "atoms": ["a"], "left": [], "right": [2]}}},
    {"kind": "ordinary.sequent", "value": {"logic": "intuitionistic",
      "sequent": {"formulas": [{"V": 0}, {"¬": 0}, {"∨": [0, 1]}, {"¬": 2}, {"¬": 3}],
                  "atoms": ["a"], "left": [], "right": [4]}}},
    {"kind": "ordinary.outcome", "value": {"verdict": "valid", "logic": "classical",
      "search": {"method": "embedding", "translation": "affine", "image": {…}},
      "derivation": {"logic": "classical", "sequent": {…}, "inferences": […]}}},
    {"kind": "ordinary.outcome", "value": {"verdict": "valid", "logic": "intuitionistic",
      "search": {"method": "sat", "statistics": {…}},
      "derivation": {"logic": "intuitionistic", "sequent": {…}, "inferences": […]}}}
  ],
  "nodes": [
    {"id": 0, "value": 0},
    {"id": 1, "operation": "ordinary.translate", "from": [0],
     "parameters": {"translation": "glivenko", "target": "intuitionistic"},
     "settings": {"limits": {"work": 67108864, "memory_bytes": 100663296}},
     "record": 0},
    {"id": 2, "operation": "ordinary.decide", "from": [1], "parameters": {},
     "settings": {"limits": {"work": 67108864, "memory_bytes": 100663296}},
     "record": 2},
    {"id": 3, "operation": "ordinary.certificate", "from": [2],
     "parameters": {}, "settings": {"styles": {"rocq": {"kernel": null}}}}
  ],
  "records": [
    {"operation": "ordinary.translate", "inputs": [0], "output": 1, "end": "done",
     "parameters": {"translation": "glivenko", "target": "intuitionistic"},
     "settings": {"limits": {"work": 67108864, "memory_bytes": 100663296}},
     "check": {"promise": "same_verdict", "status": "held", "parts": [
       {"name": "sequent", "record": 1, "verdict": "valid"},
       {"name": "translation", "record": 2, "verdict": "valid"}]}},
    {"operation": "ordinary.decide", "inputs": [0], "output": 2, "end": "done",
     "parameters": {},
     "settings": {"limits": {"work": 67108864, "memory_bytes": 100663296}},
     "check": {"promise": "decides", "status": "held"}},
    {"operation": "ordinary.decide", "inputs": [1], "output": 3, "end": "done",
     "parameters": {},
     "settings": {"limits": {"work": 67108864, "memory_bytes": 100663296}},
     "check": {"promise": "decides", "status": "held"}}
  ],
  "log": {"at": 4, "entries": [
    {"name": "sequent", "swaps": [{"insert": 0}]},
    {"name": "translation", "swaps": [{"insert": 1}]},
    {"name": "decision", "swaps": [{"insert": 2}]},
    {"name": "certificate", "swaps": [{"insert": 3}]}]}
}
```

Reading it: record 1, the classical decision, belongs to no node: the
translation's check made it. Record 2 is both the
check's second part and what node 2 shows, so deciding the translation
ran once. Node 3 has no `record`: a certificate is a rendering, made
again when asked for. An applied insert names its node; an undone one
holds the node whole, for redo. `log.at` is how many entries are
applied.

## 11. The library's types (a sketch)

Module `deduxit::workspace`, at the root beside `wire` and `settings`:
every suite shares it and it is no part of a calculus. A cargo feature
`workspace`, on by default, which needs `serialize` and brings
`serde_json` as an optional dependency of the core crate (a
dev-dependency today [V, `core/Cargo.toml`]; in `Cargo.lock` through
the command and the bindings; MIT or Apache-2.0): values are canonical
JSON text and parameters are JSON values. An operation is in the
catalogue only where its own features are on. The catalogue's trait
and its registry belong to step 63's protocol (`deduxit::protocol`
here); the shapes below are what the workspace needs of it.

```rust
// deduxit::protocol (step 63's): what the workspace needs.

#[non_exhaustive] pub enum Role { Statement, Definition, Work, Evidence, Rendering }
#[non_exhaustive] pub enum Class { Derived, Work, Copy }
#[non_exhaustive] pub enum Cost { Instant, Search }

/// A kind of artifact, as its suite's catalogue names it.
#[non_exhaustive]
pub struct Kind { pub name: &'static str, pub role: Role, pub stored: bool, pub level: u32 }

/// An operation, as its suite's catalogue names it.
#[non_exhaustive]
pub struct Operation {
    pub name: &'static str,
    pub from: &'static [&'static [&'static str]],
    pub to: &'static str,
    pub class: Class,
    pub cost: Cost,
    pub reads: &'static [&'static str],
    pub promise: &'static Promise,
    pub label: &'static str,
}

/// What an operation promises of its output, and what the check costs.
#[non_exhaustive]
pub struct Promise { pub name: &'static str, pub cost: Option<Cost> }

/// What a suite offers the workspace.
pub trait Suite: Sync {
    fn kinds(&self) -> &'static [Kind];
    fn operations(&self) -> &'static [Operation];
    /// Checks a value against its kind's reader, within the limits.
    fn read(&self, kind: &Kind, value: &str, limits: &Limits) -> Result<(), Error>;
    /// One line that names a value in a list.
    fn summary(&self, kind: &Kind, value: &str, limits: &Limits) -> Result<Summary, Error>;
    /// Runs an operation on its inputs' JSON under the recipe's settings.
    fn run(&self, call: &Call<'_>, stop: &mut dyn FnMut(Progress) -> bool)
        -> Result<Produced, Error>;
    /// Makes the check of an operation's promise; `ask` runs another
    /// operation through the workspace's records.
    fn check(&self, call: &Call<'_>, output: &str, ask: &mut dyn Ask,
             stop: &mut dyn FnMut(Progress) -> bool) -> Result<Check, Error>;
}

/// Every suite of this build; `describe` writes it as JSON.
pub fn suites() -> &'static [&'static dyn Suite];
```

```rust
// deduxit::workspace

/// A node's number in its workspace, never used twice.
pub struct NodeId(u32);

/// The bounds of a workspace, `Limits::workspace`. `None` is no bound.
#[non_exhaustive]
pub struct Bounds { pub nodes: Option<u32>, pub records: Option<u32>,
                    pub log: Option<u32>, pub bytes: Option<u64> }

/// Whether a node shows what its recipe gives now.
#[non_exhaustive]
pub enum State { Given, Current, OutOfDate(Cause), NotRun }
#[non_exhaustive]
pub enum Cause { Input(NodeId), Recipe, Version, Waits(NodeId) }

/// How a run ended.
#[non_exhaustive]
pub enum End {
    Done,
    Limit { reason: &'static str, setting: Option<&'static str>, value: Option<u64> },
    Refused(Box<Error>),
}

/// The result of a promise's check, with the records of its parts.
#[non_exhaustive]
pub struct Check { pub promise: &'static str, pub status: Status, pub parts: Vec<Part> }
#[non_exhaustive]
pub enum Status { Held, Failed, Open, Recorded, Outside, None }

/// What to compute for a node: its recipe's output, or its promise's check.
pub enum What { Output, Check }

/// A workspace: nodes over immutable values, the records and the edit log.
/// Owned and `Send`; it starts no thread and reads no clock.
pub struct Workspace { /* next, values, nodes, records, log */ }

impl Workspace {
    pub fn new() -> Self;

    /// Makes one macro action: every edit `f` makes is one log entry
    /// named `name`, and none is kept if `f` fails.
    pub fn edit<T>(&mut self, name: &str, limits: &Limits,
                   f: impl FnOnce(&mut Edit<'_>) -> Result<T, Error>) -> Result<T, Error>;
    pub fn undo(&mut self) -> Option<&str>;      // the entry's name
    pub fn redo(&mut self) -> Option<&str>;
    pub fn undoable(&self) -> Option<&str>;
    pub fn redoable(&self) -> Option<&str>;

    /// A session's next state: no log entry.
    pub fn work(&mut self, node: NodeId, session: &str, limits: &Limits) -> Result<(), Error>;

    pub fn nodes(&self) -> impl Iterator<Item = Node<'_>>;   // id, kind, recipe, value, record
    pub fn value(&self, node: NodeId) -> Option<&str>;
    pub fn get<T: wire::Readable>(&self, node: NodeId, limits: &Limits) -> Result<Option<T>, Error>;
    pub fn state(&self, node: NodeId) -> State;
    pub fn check(&self, node: NodeId) -> Option<&Check>;
    pub fn plan(&self, targets: &[NodeId], what: What) -> Vec<Step>;
    pub fn layers(&self) -> Vec<Vec<NodeId>>;    // a node's column: its longest path from a given node

    /// Points each node at a record that matches its recipe; runs nothing.
    pub fn settle(&mut self) -> Vec<NodeId>;
    /// Computes one node's output or check and leaves the workspace as it is.
    pub fn compute(&self, node: NodeId, what: What,
                   stop: impl FnMut(Progress) -> bool) -> Result<Computed, Error>;
    /// Puts a computed result in if its key is still the node's; says whether it was.
    pub fn accept(&mut self, node: NodeId, computed: Computed, limits: &Limits) -> Result<bool, Error>;
    /// `compute` and `accept` for the nodes a plan names, in order.
    pub fn run(&mut self, targets: &[NodeId], what: What, limits: &Limits,
               stop: impl FnMut(Progress) -> bool) -> Result<Vec<Ran>, Error>;

    /// Gives every recipe these bounds and drops the records that no
    /// longer match: for a document the caller did not write.
    pub fn rebound(&mut self, limits: &Limits) -> Vec<NodeId>;
    /// The part a link or a file holds (section 14).
    pub fn pack(&self, nodes: &[NodeId], keep: Keep) -> Workspace;
}

/// The edits of one macro action.
impl Edit<'_> {
    pub fn add(&mut self, kind: &str, value: &str) -> Result<NodeId, Error>;
    pub fn set(&mut self, node: NodeId, value: &str) -> Result<(), Error>;
    pub fn connect(&mut self, operation: &str, from: &[NodeId], parameters: &str,
                   settings: &Settings) -> Result<NodeId, Error>;
    pub fn set_parameters(&mut self, node: NodeId, parameters: &str) -> Result<(), Error>;
    pub fn set_settings(&mut self, node: NodeId, settings: &Settings) -> Result<(), Error>;
    pub fn restart(&mut self, node: NodeId) -> Result<(), Error>;
    pub fn detach(&mut self, node: NodeId) -> Result<(), Error>;
    pub fn remove(&mut self, node: NodeId) -> Result<Vec<NodeId>, Error>;
    /// A session's state inside an action (a copied proof): part of the entry.
    pub fn work(&mut self, node: NodeId, session: &str) -> Result<(), Error>;
}

impl Serialize for Workspace { /* the level computed from the content */ }
impl wire::Readable for Workspace { /* "workspace"; changes no recipe */ }
impl Serialize for Computed {}
impl wire::Readable for Computed {}
```

Why these shapes:
- **Values are JSON text inside**, so the module names no suite's type;
  a Rust caller reads one with `get::<ordinary::Outcome>` (A). Rejected:
  `Box<dyn Any>` values (no equality, no serde) and a `Workspace<S>`
  per suite (two suites in one graph is the point). One parse a run is
  linear and small beside a search; a typed side table is a
  measurement [M].
- **`edit` takes a closure** (S): "Prove it" is two connects and
  closing a tab is some detaches and a remove, each one entry; a
  failure inside leaves nothing half made.
- **`compute` and `accept`** (B) with **sessions passed beside** (A):
  section 12.
- **Plain data with defaults**: `Bounds` is serde with `default` and
  `deny_unknown_fields` as `Limits` is; the command maps flags onto
  it, the page holds it as JSON, and no bound is a constant inside
  (the plan's D15 and D16). Every public type a later step may extend
  is `#[non_exhaustive]`.
- **Quantifiers** (D17) need nothing here: a first-order sequent is a
  value like any other, and its kinds keep their names.
- The `Suite` trait is not sealed (B, against A's sketch): the wire
  forms stay sealed, and a suite from another crate is one of the
  cases step 63's prompt names. 63a decides the trait's final shape.

## 12. The bindings and the command

### 12.1 The bindings

Stateless, as every call [V, `.claude/rules/web.md`]: the page holds the
workspace document and sends it.

| call | does |
|---|---|
| `workspace_read(document, settings)` | reads a document from a link or an imported file, gives its recipes the tab's bounds (`rebound`) and answers it as the page holds it |
| `workspace_edit(workspace, action, work, settings)` | one macro action, `{"name", "edits": […]}`; answers the workspace |
| `workspace_undo(workspace, work)`, `workspace_redo(…)` | answers the workspace, the entry's name and the sessions that came back |
| `workspace_compute(workspace, node, what, work)` | one node's output or check: a `Computed` |
| `workspace_accept(workspace, node, computed)` | puts it in; answers the workspace and whether its key still fitted |
| `workspace_view(workspace, work, settings)` | the cards: per node its kind, summary, state with its cause, recipe's label, check, and what applies to it |
| `workspace_pack(workspace, work, nodes, keep)` | the document for a link or a file |

`work` maps a `work` node to its session document (A). The page keeps
each open tab's session with the tab, as today, and hands over those
the call needs; the document the page holds between calls has those
nodes without their values. So a proof step sends nothing of the
workspace and costs what it costs today. B's alternative, the page
writing sessions into its copy of the document by hand, would have
TypeScript give out value indices without the library's interning.

Every call reads the document it is given within the tab's limits (the
sizes, the workspace's bounds) and changes no recipe's bounds; only
`workspace_read` rebounds.

All calls go to the one module worker [V, D28]. A workspace edit asked
for while a run is pending first stops the worker, as Stop does, then
makes the edit (B's correction of its own text).

### 12.2 The command

One command group, the file first, each subcommand one action or one
read. The file is rewritten whole, beside and renamed, as outputs are
[V, `.claude/rules/cli.md`].

```
deduxit workspace new FILE [SEQUENT] [--logic L | -i | --affine | …]   a file, with one given sequent
deduxit workspace add FILE SEQUENT [--logic L | …]                    another given sequent
deduxit workspace apply FILE NODE… OPERATION [--set KEY=VALUE]… [limits] [--no-run]
deduxit workspace set FILE NODE (SEQUENT | --set KEY=VALUE… | limits)   a value, parameters or settings
deduxit workspace set FILE --all limits                                every edge's bounds (`rebound`)
deduxit workspace work FILE NODE SESSION                               a saved session into its node
deduxit workspace run FILE [NODE…] [--checks] [--restart] [--timeout D]
deduxit workspace remove FILE NODE
deduxit workspace undo FILE | redo FILE | log FILE
deduxit workspace show FILE [NODE] [--format text|json] [--output OUT]
deduxit workspace check FILE
deduxit workspace operations [KIND]
```

- A node is its number. `--set` is `--style`'s spelling [V]: a plain
  key is a parameter of the operation, a dotted one a settings key.
  The limit flags (`--work-limit`, `--memory-limit`, …) are the
  existing ones and set the edge's settings; `run` takes none of them,
  since an edge runs under its own (section 5.1). `set --all` gives
  every edge the bounds named, which is what to do first with a file
  from someone else. `--timeout` is the
  caller's stop and is never written.
- `apply` makes a node and runs it and its check. `run` runs what is
  out of date or not run among the nodes named, or all; `--checks`
  also makes checks not yet made; `--restart` starts sessions again.
- `show FILE` lists the nodes; `show FILE NODE --format json` writes
  the node's value in its own form with `version`, which `check` and
  `interact --state` read, so a node's value goes on to every existing
  command (B), and `work` brings a session back. `show` of a rendering
  writes its text.
- `check` makes every record's check again, searches included.
- Exit status of `apply` and `run`: the worst of what ran, in the
  batch's order (error 2, unknown 3, not valid 1, valid 0) [V]; a
  failed check is 2; a stop is 3. The others: 0, or 2. `check`: 0 all
  hold, 1 one fails, 3 one could not be made within the limits.

```console
$ deduxit workspace new w.json '|- a \/ ~a' --logic classical
0  sequent, classical: ⊢ a ∨ ¬a
$ deduxit workspace apply w.json 0 ordinary.translate --set translation=glivenko
1  sequent, intuitionistic: ⊢ ¬¬(a ∨ ¬a)
   from 0 by Glivenko's translation. Both valid, so the verdicts agree.
$ deduxit workspace apply w.json 1 ordinary.decide
2  valid in intuitionistic logic, by a checked derivation
   from 1 by a decision
$ deduxit workspace set w.json 0 '|- a -> b \/ ~b'
0  sequent, classical: ⊢ a → b ∨ ¬b
out of date: 1, 2. Run them with: deduxit workspace run w.json
$ deduxit workspace run w.json --checks; echo "exit $?"
1  sequent, intuitionistic: ⊢ ¬¬(a → b ∨ ¬b)
   from 0 by Glivenko's translation. Both valid, so the verdicts agree.
2  valid in intuitionistic logic, by a checked derivation
exit 0
$ deduxit workspace undo w.json
undone: sequent change. Nothing is out of date.
```

The lines are a sketch; the manual's block shows the real output.
Rejected: flags on `prove` (it has some sixty already, and an edit is
no proof) and a prompt like `interact`'s (a file changed by one call
each scripts better) (A).

## 13. The page

Every text below follows `.claude/rules/writing.md`. F read the client's
code for each constraint named here; E applied the five skills to both
proposals' first-time paths. Where they disagreed it is said.

### 13.1 One sequent stays as it is

While no tab is linked, the page is today's in every pixel, key,
message, focus order, link and stored proof, and in every control but
the one named below. It builds no workspace document, keeps no log,
shows no button and makes no workspace call: the two sheets of the
Logic popover call what they call today, under the settings they send
today, so they answer as today. The workspace's code is a lazy chunk
loaded at the first link (B's rule, kept against A's "a node under every tab": with a
node per tab and `add` in the log, the panel's second Undo closes a tab
the user opened before any link and never linked, B's witness).

One control is new and reachable from one sequent: **Duplicate this
proof**, in the Logic popover's row under the switches, where 63t's two
buttons stand, for every calculus (F: a tab has no menu [V,
`TopBand.tsx`], so B's "Duplicate tab" had no home). The popover is
opened on purpose, so a visitor who does not open it sees nothing new.
The prompt names duplicating as a way to branch, so it needs a place
before any card exists.

A walk in `tests/states.ts` holds today's states with the workspace's
code in the build.

### 13.2 When the workspace appears

At the first link of a visit. A link is made by:
- **Prove it in ⟨logic⟩** on the translation's sheet;
- **Duplicate this proof**;
- **Prove the first part** and **Prove the second part**, two new
  buttons on the interpolant's sheet (A's; B took them).

The two tabs of a link become cards, and a button **Workspace** with
their number appears in the top band, a sibling before the row of tabs
(F: inside the row it scrolls away on a phone, where two tabs already
fill the row [V, `app.css:2276`, `:385`]; first in the row, as A had
it, it moves every tab).

A tab that no link touches is no card, and opening, changing or closing
it is no edit of the workspace (B; A and E agreed for the list, S takes
it for the data on the page too). **A goal opened in a new tab** stays
what it is today and by itself makes no workspace and no button (A,
against B: the prompt names transforming, translating and duplicating,
and opening a goal is today's everyday action); once one of its two
tabs is linked, it is drawn as a dotted line, "A goal of ⟨sequent⟩".

The page says "linked tabs", never "cards" or "nodes" (E: the learner
knows tabs; a tab is named by its sequent, never by a number, which no
tab shows [V, D53; `short` in `words.ts`]; B's "Tab 2" and "c3" are
out). The button keeps the word "Workspace" (A, B and F), against E's
"Links": the Export menu has "Share a link", and the command, the file
and the manual say workspace.

### 13.3 The panel, a linked tab's row, the line above it

**Where** (F, with E for the side). Left of the stage, inside the
window's stage row, so both bands keep their width and Learn keeps the
right [V, `app.css:1711`: Learn's frame is the right column]. One rule
in TypeScript and no new media query (the client has one breakpoint,
640 pixels, held by a test [V]): the panel **docks** when the window's
stage row is at least 320 plus 640 pixels wide, else it is a **sheet**
(`ui.menu === "workspace"`: light dismiss, Escape, its title focused,
as Practice's and the transformations' sheets).

| width | Learn off | Learn on |
|---|---|---|
| 1280 | docked; the stage keeps 960 | a sheet at the left |
| 1024 | docked; the stage keeps 704 | a sheet at the left |
| 768 | a sheet at the left | the same |
| 375 | the phone's sheet, up from the bottom band | the same |

**By itself** only at the first link of a visit and only where it docks
(F): the new tab is fitted into the narrowed stage at the same moment.
A sheet would cover the tab the reader just asked for, so there the
button appears and the status line says the link. Showing it at the
link's own moment is E's temporal contiguity: later it would arrive
together with "out of date", two new things at once. Nothing is stored
for it.

**The row of a linked tab** is that tab unfolded (F): no box, no
shadow, no chips.
1. The tab's state shape on the rail (the five shapes a tab's dot has
   today), the sequent in the formulas' font at the tab's size, two
   lines at most, broken after a comma or before `⊢`; a menu at the
   right.
2. One small line: the state in the tab's own words ("Not started", "2
   open goals", "Proved", a verdict's keyword), then the calculus's
   mark.
3. Only when something is not as it should be: a keyword, its reason
   under it, and one action (section 13.4).

Rows 1 and 2 are one button that shows the tab. The shown tab's row
looks as the shown tab does. The menu lists what the catalogue offers
for the tab's kind ("Translate into intuitionistic logic…", "Find an
interpolant…", the sheets' own titles), then "Duplicate" and "Remove".
E's finding that each row repeats its tab's verdict (D44: no fact
twice) is met for the shown tab, whose row says nothing the tab does
not; the other rows are where a reader sees a tab that is not shown.

**The line above a made tab** is drawn as an inference line (F): a bar
from the rail with the operation's name at its end, as a rule's name
stands at its line. "From this sequent, by Glivenko's translation, this
one" reads as a derivation does, which is what deduxit draws
everywhere else; the names are words in the text font, so they are not
read as rules of the calculus. This is the workspace's one bold
element; it adds no colour and no font.
- Line 1: the operation's label ("Glivenko's translation"). The target
  logic is on the tab below and is not said twice.
- Line 2: the promise's check, a shape and a sentence, with its action
  as a sibling button, never inside (A's line was a button holding
  buttons, which axe fails [V, F]). On the line, not on the made tab
  (A and E, against B: on the tab it is a second verdict beside the
  tab's own). While the tab below shows a proof of an earlier sequent,
  the line is dashed and shows the operation's name alone: the check
  is of the new sequent, which the tab does not show yet.

| check | shape, words | action |
|---|---|---|
| held | a check mark: "Both valid, so the verdicts agree." | |
| not made yet | a dashed circle: "Not checked yet." | Check |
| open | a question mark: "Not decided: the budget of 20 units ran out in intuitionistic logic." | Try 40 units |
| failed | a cross, "Check failed": "This sequent is valid and its translation is not. This is a fault in deduxit." | Save workspace file |
| none to make | no shape: "Keeps the verdict (a theorem, not checked here)." | |

"Both valid" is the sheet's own keyword, kept so that it is recognised
(E); "budget" and "units" are the page's words [V, `tests/states.ts`],
not the library's "work limit" (E and F against B). The line is one
disclosure button that opens the evidence in place, one line per
logic, as the sheet shows it.

**Line styles**, each with its words, never colour alone: solid for a
made tab; dashed when the tab below is out of date; dotted for a copy
("Copy") or a goal. Tabs made from a tab are indented under it, three
steps at most, then flat with "From ⟨sequent⟩". The list is an `ol`,
never a `tree`: a tab made from two has two parents, and its line says
"From ⟨A⟩ and ⟨B⟩", each a button that moves the focus (F and E, from
the ARIA patterns; B's `tree` role is out).

**A made tab's sequent is read-only**, and so is its logic: the field
says "Made by Glivenko's translation of ⊢ a ∨ ¬a. Change that sequent,
or **Edit a copy**" (A; E's wording). Editing in place would silently
cut the line, a hidden dependency. The page changes no parameters of an
existing line in this step: another translation is another tab (F: the
sheet previews for the shown tab and cannot also rewrite a line).

The first time in a visit, under the panel's head until the next
action there: "These are your linked tabs. The line above a tab says
what it was made from." No button to dismiss it (E).

### 13.4 Out of date

| state | what the row shows | action |
|---|---|---|
| as it should be | rows 1 and 2; a solid line above | |
| out of date, no steps taken | the line dashed; "Out of date"; "Made from ⟨sequent⟩, which has changed." | Update |
| out of date, a proof with steps | the same keyword; "It is now ⟨new sequent⟩. This proof is of the earlier one. Undo brings it back." | Start again |
| not run | "Not run" | Run |
| running | "Updating…" with `aria-busy`, no animation | Stop |
| stopped | "Stopped"; it stays as it was | Run again |
| waits | "Waits for ⟨sequent⟩" | |

- **A tab never shows a new sequent over an old proof** (A found it in
  B's path and in its own; F and E reached the same rule). A made tab
  whose proof has steps keeps its tab, sequent and proof exactly as
  they are until **Start again**; the new sequent is named in the row.
  One state covers "a proof of an earlier sequent"; B's second choice,
  "Keep this proof", is not needed: not pressing is keeping.
- **Start again asks no question** (F, against E's confirmation): the
  row has already said what it does and that Undo brings the proof
  back, the button's name is the act, and the status line after it
  names the way back: "Started again. Workspace has Undo restart."
  It shows the tab it starts again. **Remove** asks when it would take
  more than one tab.
- **What is no row counts for the row below it.** A tab made through
  an interpolant is out of date while the interpolant waits for its
  search (section 6.1): its row says "Made from ⟨sequent⟩, which has
  changed." with **Update**, which runs what stands between the two
  tabs; only then does the row know whether its own sequent changed,
  and a proof with steps then offers **Start again**.
- The cause names the sequent (A's wording, which E judged specific
  where B's "the sequent it came from" was not).
- The button's name says the state, "Workspace, 3 linked tabs, 1 out
  of date", and the button takes the warn style: the one sign in the
  top band while the panel is closed. **Update all** stands in the
  panel's head only while two or more rows wait for a run; it runs
  every search and check that waits, in order, and starts no proof
  again.
- A change is announced once for all it touched, in the polite status
  region: "1 linked tab is out of date. Workspace has Undo sequent
  change." (E and F, from WCAG 4.1.3.) The second sentence is there
  because a sequent is edited in the top band, where the panel may be
  closed.

### 13.5 What runs by itself

One rule, said to the user in Settings as one sentence: **what needs no
search is made again by itself; a search waits for Update; a proof is
started again only when you say so.**

- An `instant` output of class `derived` is made again after every
  edit above it: a translation's new sequent, an interpolant's half, a
  drawing. (A goal opened in a new tab is a copy and is never made
  again.) A translation
  shown beside a sequent it is no longer the translation of is the
  state a newcomer misreads (A), and it costs nothing.
- A `search` waits, and a check that needs one says "Not checked yet"
  with **Check**. **Update**, **Start again** and **Run** on a row also
  make that row's check: one press brings the row up to date.
- Settings gains "Update linked tabs by themselves", off at first, the
  proving behaviours' group. With it on, searches and checks run after
  each edit, in order, one at a time, with Stop. Sessions with steps
  are never touched by it.

This is A's rule as B and E corrected it: a check's cost counts (B),
and the line to the user is "your proofs never change unless you say
so" (E). B's "nothing runs by itself" was rejected by E as a press per
free result. Two modes named in one setting is what marimo, VsRocq and
coq-lsp do (section 17).

### 13.6 The two undos

- **Place.** The proof's Undo and Redo stay in the bottom band with
  their keys. The workspace's stand in the panel's head on a row of
  their own.
- **Label.** Words named by the edit: "Undo translation", "Redo
  sequent change" (the names: translation, copy, sequent change, logic
  change, restart, closing, removal). The proof's are icons named
  "Undo" and "Redo", so the two pairs differ in form, place and name
  (F; A's visible names over B's tooltip, which no touch reaches). The
  row is shown while there is an edit to take back or bring back.
  After each: "Undone: sequent change."
- **No key in this step** (E and F). A's "the proof's key acts on the
  panel while the focus is there" is a mode error: keys are one handler
  on the window [V, `App.tsx:437`], and pressing a row leaves the focus
  in the panel. If the author wants keys later: `Z` and Shift+`Z`,
  global, free today (F); never Ctrl+Z, which is the field's own.
- **The likely slip** (E): pressing the proof's Undo to take back a
  change of sequent. Answered once a visit, at that press, when the
  proof has no step to take back and the workspace's last edit was on
  this tab: "This Undo takes back proof steps. Workspace has Undo
  sequent change."
- What a workspace edit is on the page: making a link, changing a
  linked tab's sequent or logic (with the restart of its own proof, as
  today's page restarts it, in the same edit), Start again, closing or
  removing a linked tab. **Closing a linked tab** closes that tab
  alone: the tabs made from it keep their sequents and proofs and lose
  their lines (`detach`), in one edit that Undo takes back whole (S:
  today a closed tab takes nothing with it, and that must stay). A tab
  made from it whose sequent is not made yet (a search above it waits)
  cannot be detached; it leaves the workspace in the same edit and
  stays open as a tab that is not linked.

### 13.7 The graph, a second view

- **What it is** (F): the list's second layout, "Graph", in the stage's
  place, as a method panel like Table and Tableau [V]: it covers the
  drawing, scrolls as a page does, and the stage's pointer and wheel
  handlers already leave such a panel alone. The proof, its camera and
  its dock stay as they were underneath. Both proposals' "the stage's
  own pan and zoom" does not hold: the camera is private to the stage
  and bound to the proof's drawing [V, `Stage.tsx:172`], and zoomed
  buttons fall under the 24 and 44 pixels the tests measure.
- **Opened** by **Show as graph** in the panel's head, never by itself;
  the button is shown from three linked tabs on (E: for two in a line
  the graph shows nothing the list does not). **Closed** by **Back to
  the proof**, by pressing a tab's box, and by any action on the proof.
- **It draws** the linked tabs and the lines between them, the same
  component over the same data in another layout, and adds no action.
  A's box per artifact (two linked tabs become eight to ten boxes) is
  the overwhelming view and a second set of things to learn (E and F);
  B's "Show every step" is a third level, where two work best (NN/g).
  Every node is in the command's `workspace show`.
- **Layout**: columns by a node's longest path from a given one
  (`Workspace::layers`), rows by a small tidy pass; no positions
  stored, nothing dragged (A; B withdrew its drag), so a link and a
  file hold no layout and WCAG 2.5.7 asks for nothing.
- **Roles and keys**: a section named "Workspace graph" holding the
  same `ol` in the list's order; Tab reaches everything; on a focused
  box Left goes to its first input, Right to the first tab made from
  it, Up and Down within a column. No arrow key has another meaning on
  the page today [V, F].
- **No library** (F): React Flow is out by the client's "no React"
  rule [V], and forty lines of layout need no dependency.
- A phone gets the same layout, panned by a finger; the list stays its
  first view.

### 13.8 The first-time task

The task 63g-c's fresh agent tries in a browser. A desktop width; each
step has at most one new thing. The example is B's, which keeps the
verdict: A's `⊢ a ∨ ¬b` flips it, so one press would bring a new
verdict, a countermodel and an unprovable tab at once (E).

| step | the user does | they see | the one new thing | the page says |
|---|---|---|---|---|
| 1 | types `⊢ a ∨ ¬a` in a classical ordinary tab, starts | today's page | | today's |
| 2 | Logic, "Translate into intuitionistic logic" | today's sheet: Glivenko, "Both valid", `⊢ ¬¬(a ∨ ¬a)` | | today's |
| 3 | **Prove it in intuitionistic logic** | the new tab as today; the Workspace button; the panel docked with two rows and one line: "Glivenko's translation", "Both valid, so the verdicts agree." | a tab can come from another, and the line says how | in the panel: "These are your linked tabs. The line above a tab says what it was made from." Status: "⊢ ¬¬(a ∨ ¬a) was made from ⊢ a ∨ ¬a by Glivenko's translation." |
| 4 | proves the translation | its row's line 2 becomes "Proved" | nothing to read | today's |
| 5 | shows the first tab, changes its sequent to `⊢ a → b ∨ ¬b` | as today the first tab starts anew; the second row's line is dashed and names the translation alone; the row says "Out of date", "It is now ⊢ ¬¬(a → b ∨ ¬b). This proof is of the earlier one. Undo brings it back." with **Start again**; the Workspace button takes its warn style | out of date | status: "1 linked tab is out of date. Workspace has Undo sequent change." |
| 6 | **Start again** | the second tab is shown with the new sequent, not started; its line is solid again: "Both valid, so the verdicts agree." | Start again makes it anew | status: "Started again. Workspace has Undo restart." |
| 7 | presses the bottom band's Undo (the likely slip) | nothing changes in the proof | which undo is which | once: "This Undo takes back proof steps. Workspace has Undo restart." |
| 8 | **Undo restart**, then **Undo sequent change** | the finished proof is back, out of date; then the first sequent and its proof, and nothing is out of date | the panel's Undo takes back changes to tabs | status: "Undone: sequent change." |

On a phone step 3 differs: the button appears, the panel does not open,
and the status line ends "Workspace shows the link." Whether a newcomer
finds Start again and the workspace's Undo from the button and the
status line alone is a question for the trial (section 16).

E counted about eleven new elements at step 3 in each proposal, at the
moment a double negation has just been read. Here step 3 adds: the
button, the panel with its title and its close, two rows of things the
user already knows (their tabs, each with its menu), one line with two
short texts, one sentence in the panel, one status sentence, and "Undo
translation". Gone are the dismiss button, the chips, the canvas button
(until three tabs), "Update all" (until two rows wait for a run) and
Redo (until something is undone). Step 5 adds the word "Out of date"
with its reason and its one action, the dashed line, and the button's
warn style: one fact, shown where the panel is open and where it is
closed. E would also hide the undo row until
a later edit; it stays, because the link just made is the edit most
likely to be regretted and a row that appears later shifts the list.

### 13.9 Arriving by a link

A student who opens a teacher's link meets a workspace they did not
build (E: neither proposal designed this). The page opens the shown
tab as today, with its proof; the panel stays closed, also where it
would dock; one line says "This link holds 3 linked tabs" with **Show
workspace**. What needs no search is made at once, as section 13.5
says, so every tab the link opens has its sequent. Opening the panel
then runs the searches and checks the link left out, in order, one at
a time, with Stop, and says so once ("Making 3 linked tabs again"):
the sender had them made, and nothing should first be seen as "Not
run" (E). No search runs before the user opens the panel (A's
objection to B: a link that runs five searches at load keeps the one
worker busy before the visitor can act).

### 13.10 The first edge, and today's links between tabs

- **Prove it in ⟨logic⟩** becomes one edit, "translation": the source
  tab's sequent and proof join the workspace, `ordinary.translate` is
  connected with the sheet's translation and target, and
  `ordinary.start` on its output. The tab opens as today. The edge
  takes the settings the sheet's own call ran under (the tab's, as
  63t-b's sheets send them), and its check is made at the press, so
  the line says what the sheet just said. That decides the two
  sequents a second time; the sheet's answer is not taken over, since
  a result the workspace did not compute must not be recorded as its
  own (A's objection to B's "accept what the sheet holds"), and the
  sheet must not call the workspace before any link (section 13.1).
- **Find an interpolant**: the sheet as today, with **Prove the first
  part** and **Prove the second part**, each `ordinary.interpolate`,
  `ordinary.half` and `ordinary.start` in one edit. The interpolant is
  a node between two tabs and is no row: the line above the new tab
  joins the labels ("The first part of an interpolant") and carries
  the check `interpolates`.
- **A goal opened in a new tab** (`origin` [V, `state.tsx:2121`]) is a
  `linear.goal` or `ordinary.goal` edge (class copy), read from `origin`
  when a workspace is
  built. It never goes out of date.
- **Fill** stays a step of the parent's proof (both): as an edge it
  would close a cycle, and the step copies a proof, which a value does
  without an edge.
- **Table, Tableau, a goal's countermodel, cut elimination** stay views
  of the stage in this step. 63t-b's two sheets keep their place in the
  Logic popover.

## 14. The link and the stored workspace

- **A tab that is not linked** writes today's link, `#v=2…`, byte for
  byte, also when other tabs are linked (A, after its own criticism of
  B: an unlinked tab must not share work the user did not mean to).
- **A linked tab** writes `#v=3&u=…&m=…&s=…&w=…`. `u`, `m` and `s` are
  the shown tab's, as in a link of version 2, so the address stays
  readable and a page that does not know version 3 (an installed one,
  offline) opens that sequent and says the rest could not be restored,
  by its existing path [V, `link.ts`, `problem: "version"`] (both). `w`
  is `workspace_pack` of the shown tab's linked group: the given
  values, the recipes without their bounds and the sessions; no
  records, no log, no renderings (B). It goes deflate-raw and base64url as `d` does. The
  shown node is in the document, so no further key is needed.
- **Too long** (2000 characters [V]): first the sessions of the tabs
  not shown are left out, then every session; if the recipes alone do
  not fit, the link is today's link of the shown tab. The Export menu
  says which was written, with the sizes (the survey: say the size,
  never cut silently): "This link holds your linked tabs without their
  proofs." or "This link holds this tab only." and then **Save
  workspace file**, the whole level-19 document for the command, which
  Import reads as it reads a session file.
- **Reading**: at most `UNPACKED_LIMIT` unpacked [V], then the
  library's reader under the tab's limits and bounds. A `w` the reader
  refuses opens the `s` sequent with "The linked tabs could not be
  restored." Links of versions 1 and 2 read as today.
- **Nothing is sent anywhere**: the fragment never reaches a server.
- **Autosave**: the shelf's version 3 keeps its proofs as today and
  adds `workspace`, the document with the open tabs' sessions left with
  their proofs. Records of this build are kept; renderings never.
  Versions 1 and 2 read as before, an `origin` staying what it is. The
  restore bar restores linked tabs with their lines. The size against
  the browser's storage is measured [M].

## 15. Learn and Practice

- A lesson's worked proof, a lesson's exercise and a Practice problem
  each open as one tab that is not linked: lessons start without the
  graph, and nothing of it is loaded.
- With Learn on, the Workspace button appears under the same rule as
  elsewhere and the panel never opens by itself (A; F: with Learn's
  frame open it is a sheet at every common width anyway). The buttons
  that link stay where they are today (A and E; B withdrew hiding
  them: "Open in a new tab" and the two sheets exist in a lesson's tab
  today [V]). E would hide the button too; it stays, because a hidden
  button hides a tab that is out of date.
- Practice judges a problem on its own tab; a tab made from it changes
  nothing of that.
- The tour gets no stop for the workspace.
- A lesson on linked tabs, with section 13.8 as its worked example
  faded from the end (B's plan, which E judged right), is content for
  a later step and no part of this one.

## 16. What needs a measurement or a browser

For 63g-b:
1. The bytes of typical values (a session, an outcome, a workspace of
   three linked tabs), to set `limits.workspace`'s defaults, and the
   deflated link of two and of three linked tabs against 2000
   characters.
2. Whether every engine's result is a function of its key on one
   thread (the inverse method, the Horn engine, the SAT solver): the
   test against a reference relies on it (B's question 4).
3. Whether a session's JSON is byte-equal when the same state is
   written twice (A's question 2): interning and early cut-off rely on
   it.
4. Whether one parse of the inputs per run is seen beside a search;
   only then a typed side table.

For 63g-c (F's list, shortened):
5. The top band at 375 and 360 pixels with the Workspace button before
   the tabs, and whether the field's width rule must count a sixth
   control.
6. A drawn proof when the stage narrows by 320 pixels: refit, and the
   active goal in view.
7. The workspace in the browser's storage beside the shelf: the quota.
8. How "running" and "check failed" are put on the screen for the axe
   walk; whether a tab made from two is reachable in this step.
9. A screen-reader pass of the line's name and the "From" buttons, and
   the first-time task on a phone.
10. The bytes the lazy chunk adds, against the budget.

## 17. Sources

What the design takes, and from where. The survey read the PDFs marked
"read" in full; the other pages were read as summaries, and what it
could not confirm is not used as a reason here.

- **A cache as traces, early cut-off, and exactness.** Mokhov, Mitchell
  and Peyton Jones, "Build systems à la carte" (ICFP 2018, read):
  https://dl.acm.org/doi/10.1145/3236774 . A record is a constructive
  trace; a run whose output is unchanged stops the rebuild below it
  (section 2.3 there).
- **Marking down, computing on request.** Houdini's cooking:
  https://www.sidefx.com/docs/hdk/_h_d_k__op_basics__overview__cooking.html ;
  Salsa's algorithm: https://salsa-rs.github.io/salsa/reference/algorithm.html .
- **Naming by content; what a key holds.** Git objects:
  https://git-scm.com/book/en/v2/Git-Internals-Git-Objects ; Nix's
  content-addressed outputs:
  https://nix.dev/manual/nix/stable/store/derivation/outputs/content-address.html ;
  Bazel's remote cache: https://bazel.build/remote/caching ; marimo's
  caching, which says what its key leaves out:
  https://docs.marimo.io/api/caching/ .
- **Two named modes of running, one setting.** marimo's lazy runtime:
  https://docs.marimo.io/guides/reactivity/ ; VsRocq's manual and
  continuous modes: https://github.com/rocq-prover/vscoq ; coq-lsp:
  https://github.com/ejgallego/coq-lsp .
- **An outdated result stays on screen.** Wenzel, "Asynchronous user
  interaction and tool integration in Isabelle/PIDE" (ITP 2014, read):
  https://www21.in.tum.de/~wenzelm/papers/itp-pide.pdf .
- **One document, a simple view first.** marimo's app view:
  https://docs.marimo.io/guides/apps/ ; `git log --graph`, a list with
  rails: https://git-scm.com/docs/git-log ; Nielsen Norman Group on
  progressive disclosure, two levels:
  https://www.nngroup.com/articles/progressive-disclosure/ .
- **Hidden dependencies and viscosity.** Green and Blackwell's
  tutorial on the cognitive dimensions (read):
  https://www.cl.cam.ac.uk/~afb21/CognitiveDimensions/CDtutorial.pdf .
- **Undo grouped by intent, results outside it, a bounded history.**
  tldraw's history: https://tldraw.dev/sdk-features/history ; Vim's
  undo: https://vimhelp.org/undo.txt.html ; a field's own undo:
  https://w3c.github.io/input-events/ .
- **State in the fragment, versioned, with the size said.** The
  TypeScript playground:
  https://www.typescriptlang.org/_playground-handbook/url-structure.html ;
  Compiler Explorer on what a server store costs:
  https://xania.org/202505/compiler-explorer-urls-forever .
- **Accessibility.** WCAG 2.2: use of colour
  (https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html),
  target size
  (https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html),
  dragging
  (https://www.w3.org/WAI/WCAG22/Understanding/dragging-movements.html),
  status messages
  (https://www.w3.org/WAI/WCAG22/Understanding/status-messages.html);
  the ARIA tree pattern, which is for trees:
  https://www.w3.org/WAI/ARIA/apg/patterns/treeview/ ; React Flow's
  keyboard model, the floor for a canvas:
  https://reactflow.dev/learn/advanced-use/accessibility ; Figma on a
  canvas's accessibility tree:
  https://www.figma.com/blog/building-accessibility-into-a-canvas-based-product/ .

## 18. Decisions that were the author's, made here

The prompt leaves these open, and the planning session works
unattended; each is decided above by the panel and the sources and is
listed for the author's review.

1. **A new dependency of the core crate**: `serde_json`, optional,
   behind the feature `workspace` (section 11).
2. **A new control on today's page**: "Duplicate this proof" in the
   Logic popover (section 13.1).
3. **The word "Workspace"** on the page, and "linked tabs" for its
   cards (section 13.2).
4. **No key** for the workspace's undo in this step (section 13.6).
5. **Closing a linked tab** leaves the tabs made from it open, without
   their lines (section 13.6).
6. **A statement's value holds its calculus**, a form of level 19
   around the library's sequent (section 3.2).
7. **The settings an edge runs under are the edge's**, taken from the
   caller when it is made; `workspace run` takes no limit flags, and a
   document from elsewhere is given the reader's bounds by `rebound`
   (sections 5.1, 12.2).
8. **"Update linked tabs by themselves"** is off at first (section
   13.5).
9. **A link holds no results** (section 14).
10. **Two readings of the bounds**: the bound on edges is a bound on
    records, and the log forgets its oldest entry where the others
    refuse (section 8).
