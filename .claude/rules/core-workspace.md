---
paths:
  - "core/src/workspace/**"
---

# deduxit core: the workspace

Loaded, beside `core.md`, when a file of `core/src/workspace/` is read
or changed: values linked by the protocol's operations, the records that
cache their runs, the states computed from them, the edit log with undo
and redo, the bounds and the JSON form. The design is
`plan/notes/63g-design.md` (sections 2 to 12); the protocol it runs is
`core-protocol.md`'s.

- **Layout.** `mod.rs`: the types (`NodeId`, `Node` with its `Body`:
  `Given`, `Derived` with a `Recipe`, `Work`, `Copy`; `Record`, `Key`,
  `End`, `Entry` of `Swap`s), `Workspace` with its reads (`state`,
  `nodes`, `value`, `layers`), the log (`edit`, `undo`, `redo`), the
  runs (`plan`, `compute`, `accept`, `run`), `settle`, `rebound`, `pack`
  and the private `collect`, and `Edit`, the only way to change a node.
  `form.rs`: the JSON form, written by `Serialize` and read through
  `wire::Readable`. `tests.rs`: the reference test and one test per
  bound. Behind the feature `protocol`, as the protocol is: no feature
  of its own (departs from the design, which named one).
- **A record's key is everything its run reads** (`Key`: the
  operation's name, its parameters as canonical JSON, the projection of
  the recipe's settings on the keys the operation's `reads` lists, on
  one thread, and the input values' ids). A run is then a function of
  its key, which is what makes a record shown current trustworthy and
  undo exact; a setting an operation reads and does not list would let
  a stale record show as current. A new operation lists its reads in its
  suite (`core-protocol.md`).
- **States are computed, never stored** (`state`): `Given`, `Current`
  (the record the node points at has the key the recipe gives now),
  `OutOfDate` with a `Cause` (an input changed, the recipe changed, the
  record is foreign, an input waits) or `NotRun`. Nothing caches a
  state, so no edit can forget to mark a node stale.
- **`settle` after every change** (`edit`, `undo`, `redo`, `accept`,
  reading a file): each node that is not current is pointed at a record
  of its key where one is kept, never at a foreign one. It runs
  nothing. This is how undo shows the earlier results without a run.
- **An edit is one entry of the log, whole or not at all**: `edit`
  records each node's before and after (`Swap`) as `Edit` changes it;
  an `f` that fails puts every node back and keeps no entry. A new edit
  drops the entries past the current one (no redo after it), and past
  `limits.workspace.log` the oldest entry is forgotten. Running is no
  edit, nor is a step in a work node's session (`Workspace::work`),
  which keeps its own undo.
- **Each entry pins the records its nodes showed before it**
  (`Entry::pins`): collection and eviction never drop a pinned record,
  so undo finds it. A record is dropped only when no node shows it and
  no kept entry pins it, the oldest first (`Record::made`).
- **Every bound is a refusal, `Refusal::Workspace` with `Held`**
  (`workspace_limit`, its setting `limits.workspace.nodes`, `records`,
  `log` or `bytes`): an edit past the nodes or bytes is refused and
  taken back; a run past the records first drops unpinned records and
  is refused only when none can go; reading a file past any bound is
  refused. The log alone forgets instead of refusing. `tests.rs` has a
  test per bound; a new bound adds one.
- **Values are documents with their `version`** where the kind's form
  is a wire form (as the protocol's), interned by kind and text: equal
  values are one id, which is what makes a key's inputs comparable. An
  input's kind must be one the operation takes (`connect` refuses
  otherwise, `not_applicable`), and a given node's value is set only on
  a given node (`not_given`).
- **The JSON form re-checks what it can** (`form.rs`, level 19,
  `wire::WORKSPACE`): every value is read by its kind's reader within
  the limits; each record of an instant operation is run again and
  dropped where its output differs; each instant check is made again;
  a check that needs a search reads back as `Status::Recorded` (departs
  from the design, which made every check again), which `workspace
  check` makes again. A record written by another version of deduxit
  (`deduxit` in the form) is foreign: kept, shown as out of date, never
  settled to. A file can forge a search's output; only its check, made
  again, tells.
- **`pack`** keeps the nodes asked for and what they are made from, no
  record and no log: what a link of the web page holds.
- **`compute` and `accept` are the run split in two**, for a front end
  that runs on a worker: `compute` reads the workspace and returns a
  `Computed` (its key's parts as text, the output or the refusal);
  `accept` puts it in only if the node's recipe on its inputs' values
  still asks for exactly that key, reading the output by its kind's
  reader first. A computed value from outside is never trusted.
- **The reference test** (`tests.rs`,
  `a_random_walk_keeps_current_nodes_fresh_and_undo_exact`): random edits,
  runs, undos and redos over a few seeds; after each step every node
  shown current equals its recipe run afresh on one thread, an undo
  restores exactly the views before the edit and a redo those after.
  A change to the model keeps it passing; a new kind of edit joins its
  moves.
