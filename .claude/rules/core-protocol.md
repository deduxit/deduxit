---
paths:
  - "core/src/protocol/**"
---

# deduxit core: the suites' protocol

Loaded, beside `core.md`, when a file of `core/src/protocol/` is read or
changed: what each suite offers as data (kinds, operations, promises)
and the functions that read, summarize, run and check them over JSON.
The design is `plan/notes/63g-design.md` (sections 3 and 11); the
workspace graph builds on it.

- **Layout.** `mod.rs`: the types (`Kind`, `Operation`, `Parameter`,
  `Promise`, `Call`, `Produced`, `Limit`, `Check`, `Part`, `Status`,
  `Verdict`), the trait `Suite`, the registry (`suites`, `shared`,
  `operation`, `kind`), the front doors `read`, `summary`, `run`,
  `check`, the catalogue's JSON (`catalogue`), the promises and the
  shared lists of settings keys; `linear.rs`, `ordinary.rs`, `rules.rs`
  one suite each; `shared.rs` the renderings no suite owns (`drawing`,
  `text`, `certificate`, `term`), never stored. The feature `protocol`
  needs `serialize`, `parse` and `interactive` and brings `serde_json`,
  since values cross as JSON text; the renderings' operations are behind
  `svg` and `rocq`, a format behind `latex` or `typst`.
- **A value is a document with its `version`** where its kind's form is
  a wire form, and a statement is the sequent's document with `logic`
  (ordinary) or `mode` (linear) beside its keys (`#[serde(flatten)]`),
  which the sequent's reader ignores, so it raises no level. A linear
  statement may hold an intuitionistic sequent's document (told by
  `hypotheses`); what the protocol writes is one-sided. The protocol's
  own forms (`rules.sequent`, `ordinary.interpolant`, the renderings)
  carry no `version`. `Kind::form` names the entry of the manual's
  schema (`docs/src/wire.schema.json`) the value matches.
- **`reads` is a cache key**: the workspace keeps a run for the
  settings an operation reads, so a key missing from `reads` is a wrong
  answer from the cache, and a key too many only a miss. List every key
  the called functions read, the readers' `limits.occurrences` and
  `limits.terms` included; a key that names an object (`search`) stands
  for all its keys. When an operation's code comes to read another key,
  add it. The time limits are no part of it: a stop leaves no output.
  A check runs its operations under the call's settings, so its result
  depends on what they read.
- **A check never calls a fault what a bound left open**: a promise
  between two verdicts (`same_verdict`) with one side unknown is `Open`,
  the decided side `Held`; only two decided verdicts that differ fail.
  `protocol::check` hands the caller's stop to the runs it asks for
  (one stop through a `RefCell`, since the check's own work and its runs
  take turns).
- **An interpolant is held to its input**: reading its value checks that
  each half holds the formula where it belongs (last right of ⊢ in the
  first, first left of ⊢ in the second, the same text as `formula`) and
  is claimed in its logic, so no check indexes past an end; the check of
  `interpolates` compares the halves with the input's parts by the
  partition (`halves_fit`, as multisets of formulas' texts) before the
  atoms and the two decisions.
- **An input of several kinds is told by its keys** (`input` in each
  suite: `verdict` an outcome, `history` a session, `links` a net,
  `vertices` a net with boxes), since a `Call` carries values and not
  their kinds; the kind found must be one the operation's `from` lists
  (`takes`), and a linear outcome, which the proof's reader takes too,
  is no `linear.proof` (`proof_of`).
- **An ordinary outcome valid by a clausal proof has no reader** (its
  counts hold no proof): `summary` (`outcome_verdict`) and the check of
  `decides` read its keys alone, `logic` among them so that a linear
  outcome is refused; every other ordinary outcome goes through its
  reader, and that check is `Recorded`. A check
  of a refutation the checker calls uncertified (an exhausted search) is
  `Recorded` too.
- **A check's errors**: kind `invalid` or `defect` is `Failed`, `limit`
  or `stopped` is `Open`, any other is returned (`ordinary::checked`).
  A check that needs a search asks for it through `Ask`, so a caller
  with a cache answers it from there; `protocol::check` runs it
  directly.
- **No new error code**: an unknown operation, kind, format or half is
  `UnknownName` (its `known` lists are the static `OPERATIONS` and
  `KINDS` of `mod.rs`, which a test holds to the registry), a call with
  another number of inputs or bad parameters `InvalidOption` (keys
  `inputs`, `parameters`, `logic`, `goal`, `which`), a value of another
  form `Json`.
- **`linear.goal`** builds the goal's sequent from the session's arena
  (`Sequent::from_parts`, then `optimize`), its inputs first with
  `antecedents` their number in an intuitionistic mode, so that the
  reading takes the sides as they were; a goal holding instances of
  quantified formulas (members past the forest) is refused.
- **A run is a function of its key only on one thread**
  (`Settings::deterministic`); the tests run under it.
- **Adding an operation**: its row in the suite's `OPERATIONS`, its name
  in `mod.rs`'s `OPERATIONS` in the same place, its arm in `run`, its
  promise's arm in `check`, a schema entry for a new kind's form, the
  manual's chapter, and a test. A new suite: a module here implementing
  `Suite`, one entry in `SUITES`.
