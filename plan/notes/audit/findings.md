# The audit's findings (step 86, phase B)

What an audit of deduxit's Rust libraries and its Rocq library found
before 0.9.0, deduplicated into themes and ranked. This file is
generated: `plan/audit/themes.py` writes the themes from
`plan/audit/themes.json` and the units' files, between this head and
the list of rejected findings; change those, not this file.

## What was read, and how

The tree at main after step 85 (`vqwqlqkl`, 2026-11-27) and
`rocq-deduxit` at its main: 239 000 lines of source in 21 units, each
read whole by one finder (Opus 5.5 at `high`) for five dimensions
(idiom, architecture and duplication, performance, the public API, text)
against the criteria of `criteria.md`; and the core crate as one design
by an architecture reader (Fable 5.2 at `high`). Every finding then went
to a verifier on the other model, told to refute it: it opened the place,
checked the witness, read the criterion and the rules files for a
recorded choice, and looked for callers the change would break
(`method.md`). Tools made the inventories the agents worked from
(`baselines.md`, `inventories/`).

| | findings | confirmed | candidates to measure | rejected |
|---|---:|---:|---:|---:|
| the architecture reader | 27 | 25 | 2 | 0 |
| 21 units | 352 | 300 | 44 | 8 |
| **all** | **379** | **325** | **46** | **8** |

A finding's full text (its witness, the change, the value, the net) is
`units/UNIT/findings.md`, and its verifier's reasons and corrections
`units/UNIT/verdicts.md`: read both before acting on one, since most
verdicts of "confirmed" correct the place, the cost, the kind or the
change (216 of 256 in stages 3 to 5).
`findings-index.md` lists all 379 in one table. `rejected.md` holds the
eight a verifier refuted, `standing.md` the recorded decisions a
proposal must answer, `handovers.md` what one unit asked of another.

No file of either repository was changed by the audit. Two defects it
found are being fixed beside it (theme A1).

## The ranking

The 371 findings that stand are in 45 themes and nine groups. The groups
are in the order the refactor takes them; inside a group the themes are
in the order of their value against their cost and risk: first what
many users gain from at small cost with an instrument that shows
behaviour is kept, last what is large, gains tidiness alone, or touches
a checker.

| group | themes | findings | what it is | when |
|---|---:|---:|---|---|
| A | 4 | 35 | defects and bounds that do not hold | first, as fixes |
| B | 10 | 96 | the public API for 0.9.0 | after the author signs `api-proposal.md` |
| C | 12 | 90 | duplication | beside and after B, unit by unit |
| D | 5 | 57 | performance | journeys first, then by measurement |
| E | 3 | 26 | files and modules to divide or move | after B's moves |
| F | 4 | 38 | docs, messages and help | one pass per area, after B |
| G | 1 | 4 | the command, beside its selector | with B's command step |
| H | 4 | 18 | the Rocq library | in its own repository |
| I | 2 | 7 | features and the web module's size | with B, by measurement |

What stands out:

- **The checkers are sound but for one case**, now being fixed: the
  first-order Kripke countermodel's check accepted `=` unread (A1). No
  other finding shows a wrong "proved" or "valid". Eleven calls do not
  keep the crate's own rule on limits and stops (A2), and the front
  ends answer wrongly in a few places because each chooses by hand what
  the library should choose once (A3).
- **The linear suite is still the crate's second shared layer.** About
  forty findings, from every unit that is not linear, are one fact:
  what all suites use lives under `linear` because it came first (B2,
  B3). Moving it is the largest single gain for a user of another
  suite, and it changes no behaviour.
- **An outside engine cannot yet be written** (B4): the consumer test's
  thirteen points are confirmed from inside the code, three are
  corrected (`Progress` can be built; `ordinary::Options` has
  `with_logic`; `Proof`'s docs do say that `check` is the boundary).
- **Duplication is the most frequent kind of finding** (90, and half
  of group B's 96 are the same thing seen from a user's side): two arenas, two
  connection provers, two text exporters, two net kernels, three
  sessions and their drivers in two front ends, and many small tables.
  The copies have already drifted apart, and several of group A's
  defects are that drift: the protocol's choice of a net, the lists of
  mode flags, the workspace's list of settings keys.
- **Performance has few measured facts and many candidates**, because
  the instruments run the linear suite and the exports and little else
  (D1). Where DHAT runs, it points at a handful of allocation sites
  that make most of a journey's blocks (D2).
- **The text is in good state by the patterns** (seven matches of the
  writing rules' list in 68 000 lines of prose) and behind in its
  content: front pages that lead with one suite, lists that fell behind
  the code, messages that do not name the fault (F).

## A. Defects: answers that are wrong or bounds that do not hold

These change behaviour, so they are fixes, not refactors. Each is small. They come first because a refactor must not carry a known defect along, and because two of them touch a checker. (35 findings in 4 themes.)

### A1. Two defects that fixup-31 mends

2 findings (2 confirmed, 0 candidates); kinds: API break, refactor.

**What.** The check of a first-order Kripke countermodel read `=` as an ordinary predicate, so `deduxit check` accepted a forged one-world model as a checked refutation of `⊢ a = a` (reproduced by the finder, the verifier and the leader). First-order resolution's read-back wrote a unifier's terms out in full with no bound, so `resolve --timeout 1s` ran 22 s on a family whose unifier doubles a term per variable.

**Change.** Being fixed by fixup-31, which the planning session started on 2026-11-27; not proposed again here. The refactor starts from the commit that holds the fix.

**Net.** A new test of the check with the forged document; the panel, since a checker is touched.

- F-4d-01 (confirmed, API break, small), `core/src/ordinary/kripke_structure.rs:908`: `KripkeCountermodel::check` reads `=` as an ordinary predicate and never refuses it, so it accepts a one-world model with no facts as a checked refutation of `⊢ a = a`, which `prove` proves valid.
- F-4c-01 (confirmed, refactor, small), `core/src/ordinary/resolution/first_order.rs:702 (mapped_atom, called from read_back at :2004) and :216 (bindings_text)`: First-order resolution writes terms out in full with no bound: a unifier that doubles a term per variable makes `resolve --timeout 1s` run 22 s at n = 24, growing about fourfold every two variables.

### A2. Bounds that are not kept: calls without limits or a stop

11 findings (9 confirmed, 2 candidates); kinds: API break, refactor.

**What.** The crate's rule is that every long call takes `&Limits` and a stop. Eleven places break it: the JSON readers of proofs, nets and sessions build their sequent under the default limits whatever the caller gave; `Disproof::check` takes neither; `Interactive::split_passes`, documented as cheap, builds the forest's count rows on every call with no account and no stop; `invert` and `invert_all` of two sessions run to the end without a stop; the bindings build a session's whole derivation twice per view under the default limits; a rule file's `parse_sequent` applies the bound per member; the `term` commands take no `--timeout`; the first-order refutation's check takes no `&Limits`; the protocol's `ordinary.resolve` and Herbrand's term walks write or walk terms without bound (the part of F-4c-01 that fixup-31 does not cover); two steps of the MLL criterion are quadratic and ask no stop.

**Change.** Give each call the limits and the stop its neighbours take, with the short form kept as a wrapper where one exists (`check` beside `check_within`). `split_passes` gets a bounded form and its docs corrected.

**Net.** The tests of limits (`core/tests/limits.rs`), a test per call that a small bound refuses; `Disproof::check` and the refutation's check touch a checker's entry, not what it accepts.

- F-6a-01 (confirmed, refactor, medium), `core/src/serialize/sequents.rs:578`: A proof, a proof structure (with or without boxes) and a session read through `wire::upgrade` build their sequent within the default `Limits`, not the caller's, so `limits.terms` set lower than the default is ignored on those readers.
- F-1c-01 (confirmed, refactor, medium), `core/src/linear/search/refutation.rs:1173`: `Disproof::check` takes no limits and no stop, so a saturated set or a basis read from elsewhere is checked without bound, and every caller that has a stop works around it.
- F-2b-01 (confirmed, API break, medium), `core/src/linear/search/focus/counts.rs:1254`: The public `Interactive::split_passes`, documented as a cheap test, builds the whole forest's count rows with no memory account and no stop on every call, so one ⊗ step on a nested tensor takes quadratic memory past the 1 GiB the search itself honours.
- F-4e-07 (confirmed, API break, small), `core/src/ordinary/invert.rs:109`: `invert` and `invert_all` run every invertible step to the end with no stop, bounded only by `limits.memory_bytes` (1 GiB by default), though in LK a shared `∧R` doubles the goals at each step.
- F-8-02 (confirmed, API break, medium), `web/src/calls.rs:479; web/src/calls.rs:505`: Every bindings call that answers a linear session's view builds the session's whole derivation twice under the library's default limits (1 GiB of memory, no work bound, no size bound) instead of the tab's (96 MiB), and drops `cuts` and `invertible` silently when that fails, because the linear session has no `invertible_steps` or cut list in the library while the ordinary session has one.
- F-5b-11 (confirmed, refactor, small), `core/src/rules/text.rs:503`: `parse_sequent` and `parse_list` check `limits.occurrences` per member, not per text as their docs say, so a sequent of many short members passes the bound.
- F-7b-04 (confirmed, API break, small), `cli/src/term.rs:253`: `term reduce`, `term check` and `term infer` take no `--timeout`, `--memory-limit` or `--work-limit`, and `reduce` ignores the settings' clock, so a reduction with `--steps none` runs until it is killed.
- F-4c-10 (confirmed, API break, small), `core/src/ordinary/resolution/first_order.rs:1136`: The first-order refutation's check takes a stop but no `&Limits`, unlike the propositional `Refutation::check(&limits, stop)` beside it and the crate's rule that every long call takes both.
- F-8-12 (confirmed, refactor, small), `core/src/protocol/ordinary.rs:1700; core/src/serialize/ordinary.rs:1995`: The protocol's `ordinary.resolve` on a first-order sequent writes each inference's unifier in full, and its check writes every unifier again per step, with no bound and no stop, and the page reaches it through the workspace, where no clock ends a call.
- F-4c-09 (candidate, refactor, small), `core/src/ordinary/herbrand.rs:1824, :1840, :1870`: Herbrand's builder walks witness terms as trees with no visited set, once per witness, item and step, and sorts by a key that walks the term on every comparison, where the connection read-back walks shared terms as graphs.
- F-3-03 (candidate, refactor, small), `core/src/linear/nets/graph.rs:504; core/src/linear/nets/sequentialize.rs:261`: Two steps of the MLL criterion and sequentialization are quadratic in the number of parts, and neither asks the stop: the `Disconnected` witness and the Mix branch of a stage. The MELL copies removed both after the integers panel measured them.

### A3. Front ends that answer wrongly

11 findings (11 confirmed, 0 candidates); kinds: API break, refactor.

**What.** Which net a proof gets (a structure of multiplicative linear logic, or one with boxes when the proof has exponentials or cuts one occurrence twice) is chosen by hand in the command, the bindings and the protocol, and the protocol's copy lacks the fallback: `linear.net` answers `invalid_net` for a proof the checker accepts. Beside it: `split_passes` ignores its step's rule, so the page can grey a Mix that would close; `invert` skips a `∀` step at the bound on terms and reports that it finished; a workspace run ended by one of ordinary logic's three bounds loses its setting; the workspace's exit status reads a verdict from a string; `eliminate --net` panics on a closed pipe; `--output -` writes a file named `-`; three hand-written lists of the linear mode flags miss `--cyclic` and `--lambek`; `eliminate` offers flags of `check` that it ignores.

**Change.** One library call chooses a proof's net (a public sum of the two structures), and the three front ends call it. The others are local mends.

**Net.** A protocol test with the proof of `nets/cut/tests.rs:556`; the command's tests and the behaviour lock; the bindings' tests under Node.

- F-8-01 (confirmed, refactor, medium), `core/src/protocol/linear.rs:553; web/src/calls.rs:2295; cli/src/show.rs:1476`: The protocol's `linear.net` answers `invalid_net` (kind malformed, "not a proof net") for a proof the checker accepts when that proof cuts one occurrence twice, because the choice between `ProofStructure` and `MellStructure` is written three times and the protocol's copy lacks the fallback the command and the bindings have.
- F-3-01 (confirmed, API break, large), `core/src/protocol/linear.rs:556`: Choosing the net of a proof (MELL with boxes for exponentials; for MLL `ProofStructure`, falling back to `MellStructure` on `Placement::Cut`) is left to every front end. Three front ends write it by hand, each with an either-net enum of its own, and the protocol's copy lacks the fallback.
- F-7b-14 (confirmed, API break, medium), `cli/src/show.rs:1476`: The choice of a proof's net (a net with boxes for exponentials, else the MLL structure, falling back to boxes where the proof cuts one occurrence twice) is written by hand in the command and in the bindings, while the protocol's `linear.net` lacks the fallback.
- F-4f-03 (confirmed, API break, small), `core/src/linear/proofs/interactive/expand.rs:750; cli/src/interact/mod.rs:524`: `split_passes` takes a `Step` but ignores its rule and judges a `⊗` on a `⊗` formula and a Mix on any other, so the command repeats that choice before every call to avoid a wrong answer.
- F-4f-07 (confirmed, API break, small), `core/src/linear/proofs/interactive/invert.rs:182`: In a session with quantifiers, a `∀` step the invertible run cannot take (past the session's bound on terms) is skipped silently, and the run reports `finished: Ok(())`, which its docs say means no invertible rule applies to any goal left open.
- F-5b-01 (confirmed, refactor, small), `core/src/workspace/mod.rs:1541`: The workspace's hand-written list of settings keys that a limit names lacks the three keys of ordinary logic's searches, so a record read back from a file or accepted from a worker loses the setting that lifts its bound.
- F-7b-01 (confirmed, API break, medium), `cli/src/workspace.rs:441`: The workspace's exit status reads a node's verdict from the string at the JSON key `verdict`, so a resolution node whose clauses are satisfiable, and an interpolant node of a sequent that is not valid, both exit 0 where the commands `resolve` and `interpolate` exit 1 and the manual promises 1.
- F-7b-02 (confirmed, refactor, small), `cli/src/eliminate.rs:442`: `eliminate --net` with a drawn format writes its verdict with `println!`, which bypasses the command's closed-pipe handling, so a closed pipe makes it panic and exit 101.
- F-7a-07 (confirmed, API break, small), `cli/src/io.rs:906`: `--output -` writes a file named `-`, while `--file -` reads standard input.
- F-7a-02 (confirmed, refactor, small), `cli/src/prove.rs:748`: The linear mode flags are listed by hand in five places, and three lists miss `--cyclic` and `--lambek`, so `prove` and `interact` run with a flag silently dropped and a settings file beats a flag given.
- F-7b-03 (confirmed, API break, small), `cli/src/eliminate.rs:44`: `eliminate` flattens all of `CheckArgs`, so its `--help` offers about ten of `check`'s flags. It refuses three of them and runs with four others silently ignored, exiting 0.

### A4. Library calls that mislead or panic on a valid use

11 findings (11 confirmed, 0 candidates); kinds: API break, refactor.

**What.** A refused proof check becomes `Verdict::Unknown` in one place and an `Err` in another; `ordinary::Derivation::check` reports any derivation that breaks a rule, a user's own included, as a defect of the read-back; `Sequent::atom` compares a name without composing it as every reader does; an instance table built through the API is refused as a JSON error; `wire::Within` drops the typed error; the public Partition encodings panic on a size of 0; the rule-file session panics on an id that is not its own; `pdf::Date` documents ranges nothing checks; `WrittenTableau::review` and `Workspace::read_own` trust what their names do not say they trust; `Engine::Unknown` is a value the options accept and the front door refuses at run time.

**Change.** Each returns the error that says what happened, or refuses at construction.

**Net.** The tests of each call; F-2a-01 and F-4a-01 change what a caller sees, so the changelog names them.

- F-2a-01 (confirmed, API break, small), `core/src/linear/search/mod.rs:387; core/src/linear/search/mod.rs:1150`: The check of a proof is mapped to a verdict in two places that disagree: `conclude` turns a refused check into `Verdict::Unknown`, while `on_own_forest` (a goal of a forest with cut formulas) returns it as `Err(Error::Check)`, against `prove`'s documented contract.
- F-4a-01 (confirmed, API break, small), `core/src/ordinary/derivation.rs:755`: `ordinary::Derivation::check` reports every derivation that breaks a rule, including one a user wrote or another engine built, as `Error::ReadBack`, whose kind is `defect` and whose message says "the proof read back", where the generic checker reports the same claim as `InvalidDerivation` (kind `invalid`).
- F-1a-14 (confirmed, API break, small), `core/src/linear/sequents/mod.rs:449`: `Sequent::atom(name)` compares the name as given. A decomposed `é` therefore misses the atom that every reader stored composed, while the ordinary arena composes before it looks up.
- F-1a-07 (confirmed, API break, small), `core/src/linear/occurrences/instances.rs:513`: An instance table built through the Rust API (`Instances::new`, `apply`, `frame`, `member`) and handed to `Proof::with_instances` is refused as a JSON document. Its error has the code `json` and the message "not an instance table of deduxit's wire form".
- F-6a-02 (confirmed, API break, small), `core/src/wire.rs:434`: `wire::Within`, the public seed, returns only the deserializer's text: the library's typed error (`Refusal`, `Version`, `IndexOutOfBounds`) that `wire::fail` stores in the thread-local `CAUSE` is read back by `upgrade` alone, so its one outside user reports a refusal as malformed input.
- F-2c-01 (confirmed, refactor, small), `core/src/linear/families.rs:1111`: The public encodings `three_partition`, `three_partition_mll` and `partition`, offered "to build a problem from data of your own", panic on an item of size 0 or on 0 bins, which none of their `# Panics` sections names.
- F-4e-02 (confirmed, API break, small), `core/src/rules/interactive.rs:439`: The rule-file session's `goal`, `members`, `hypotheses` and `lines` index the session directly and panic on an id that is not one of its nodes, so the command checks every goal itself and writes its own copy of the library's refusal.
- F-6b-16 (confirmed, API break, small), `core/src/export/pdf.rs:160`: `pdf::Date` documents ranges for its public fields and derives `Deserialize`, but nothing checks them, and krilla clamps an out-of-range month or day silently, so a settings file with `"month": 13` writes December.
- F-4b-05 (confirmed, API break, small), `core/src/ordinary/written/mod.rs:633`: `WrittenTableau::review` takes any `rules::Calculus` and says only in its docs that it must be the built-in `TABLEAU`, so a standing of `Closed` rests on whatever rule file the caller passed, and each front end reads `TABLEAU` through serde_json itself.
- F-5b-07 (confirmed, API break, small), `core/src/workspace/form.rs:381`: `Workspace::read_own` is a public reader that trusts every record and check of whatever document it is given, but its name does not mark it as the unchecked path.
- F-2a-13 (confirmed, API break, medium), `core/src/linear/search/mod.rs:2643; core/src/linear/search/mod.rs:1331`: `Engine::Unknown`, which only describes an outcome read from a later version, is a value `Options::engine` accepts and the front door refuses at run time, and writing such an outcome back loses the engine's name.

## B. The public API for 0.9.0

What a user of the library or of the command meets. `api-proposal.md` turns these into the surface the author signs off; the refactor implements it after the sign-off. (96 findings in 10 themes.)

### B1. No logic is the default (D1)

3 findings (3 confirmed, 0 candidates); kinds: API break.

**What.** Every command that reads a sequent falls back to classical linear logic by five mechanisms, the batch has a default mode of its own in the library, and `ordinary::Logic` defaults to intuitionistic, so the first call a user writes answers for a logic nobody named. The Rocq library's README and the bindings' `about` rest on the same defaults.

**Change.** A required selector of the suite on every command that reads a sequent; in the library, no `Default` for a logic, a mode or a modal system (`api-proposal.md`, sections 1 and 6).

**Net.** The examples of README, the manual and the guide, the behaviour lock, the settings tests. No checker or search is touched.

- F-7a-01 (confirmed, API break, large), `cli/src/argument_parsing.rs:2629`: Every command that reads a sequent picks linear logic when no flag names a logic, through five different mechanisms; D1 asks for one required selector.
- F-2a-12 (confirmed, API break, medium), `core/src/linear/search/batch.rs:80; core/src/linear/search/batch.rs:181`: The batch is the one entry of the linear search with a default logic: `batch::Options::mode` defaults to classical and every `Problem` without a mode takes it, where `prove`, `prove_within`, `prove_goal` and `race` all require the mode.
- F-4a-03 (confirmed, API break, medium), `core/src/ordinary/mod.rs:171`: `Logic` has `#[default] Intuitionistic` and `ordinary::Options` derives `Default`, so the first call a library user writes answers for intuitionistic logic without the logic being named, and the module's leading example relies on it.

### B2. What every suite uses leaves the linear suite (D4)

15 findings (15 confirmed, 0 candidates); kinds: API break, refactor.

**What.** Because linear logic came first, the vocabulary of every front door (`Reason`, four options only ordinary logic reads), the drawing layer (`Drawn`, `Labels`, `OpenGoal`, `TextOptions`, `Structural`, `InfId`, `GoalId`, `ViewOptions`), the formula walk and the printers' table, the memory account, the first-order terms' public path, TPTP's clause syntax and the session's `Inverted` and its options live under `linear`. A user of ordinary or modal logic writes `deduxit::linear::…` to name them, six bindings calls build a linear `Options` only to pass it on, and a user's own rule labels reach linear rules alone.

**Change.** Shared root modules, each named for what it holds; the linear suite re-exports none of them (`api-proposal.md`, section 2).

**Net.** No behaviour changes: the target sets' counters, the ratchet and the tests. Public paths change, so `breaks 0.8.0` lists them and the changelog carries the migration table.

- F-arch-01 (confirmed, API break, medium), `core/src/linear/search/mod.rs:3481`: `Reason`, why any suite's search stopped without deciding, is a type of the linear suite, and holds reasons that only ordinary logic's engines give.
- F-arch-02 (confirmed, API break, large), `core/src/linear/search/mod.rs:3016; core/src/ordinary/decide.rs:751`: Four options that only ordinary logic's engines read are fields of the linear search's `Options`, so deciding an ordinary sequent takes two options values, one of them another suite's.
- F-arch-06 (confirmed, refactor, medium), `core/src/linear/search/memory.rs:21; core/src/linear/proofs/style.rs:231; core/src/linear/sequents/notation.rs:20; core/src/ordinary/mod.rs:154`: Four crate-private pieces that every suite uses live in the linear suite, one of them making the linear suite name the ordinary one.
- F-arch-07 (confirmed, API break, large), `core/src/linear/sequents/mod.rs:21; core/src/linear/proofs/mod.rs:47; core/src/linear/proofs/mod.rs:52; core/src/ordinary/parse.rs:15`: Public types that belong to every suite's derivations, sessions and readers are reachable only as items of the linear suite, and stand in the public signatures of `ordinary`, `export` and `Settings`.
- F-1a-02 (confirmed, API break, large), `core/src/linear/sequents/fo.rs:9`: The first-order types are shared by both arenas but are public only under the linear suite's path. Only a linear sequent can read an atom's predicate and arguments. An ordinary sequent has no accessor for the same tables.
- F-1a-03 (confirmed, API break, large), `core/src/linear/sequents/notation.rs:20`: The crate-wide formula walk (`Stop`, `Stops`, the public `Walk` and `Visit`) and the printers' spelling table (`Notation`, `Atoms`) live in `linear::sequents`. Ordinary logic, modal logic, the exports, the lambda printer and the rule files import them from there. The linear suite in turn imports ordinary logic only to hold its symbols.
- F-1b-01 (confirmed, API break, large), `core/src/linear/proofs/derivation/view.rs:130`: The drawing layer that every suite uses lives inside the linear suite. This covers the view options (`ViewOptions` with `Compact`, `Sides`, `ClassicalSides`, `Structural`, `Layout`), the text tree (`TextOptions`, `fmt::write_text`, `text_size`, `write_steps`), the labels (`Labels`, `OpenGoal`, the trait `Drawn`) and `InfId`. So ordinary logic, lambda-terms, rule files, the protocol and `Settings` name `crate::linear::proofs::…` for a notion that is not linear logic.
- F-1b-04 (confirmed, API break, medium), `core/src/linear/proofs/style.rs:53`: `Labels::Table`, the one public way to give rules your own labels, is keyed by the linear rule `Named`. Four of the five drawings silently ignore it: the classical two-sided view, ordinary logic, lambda-terms and rule files.
- F-4b-15 (confirmed, API break, medium), `core/src/ordinary/normal.rs:3160; core/src/ordinary/normal.rs:1074; core/src/ordinary/interpolant.rs:249; core/src/ordinary/interpolant.rs:350`: Following F-arch-02: the public calls of normal forms and interpolation take the linear search's `Options` as a parameter, so a user who only wants a CNF or an interpolant must construct another suite's options.
- F-4c-04 (confirmed, API break, medium), `core/src/ordinary/connection/mod.rs:120 and core/src/ordinary/resolution/mod.rs:842`: The connection prover's public entry takes the linear suite's `search::Options` and reads two of its fields; first-order resolution's `clauses_of` takes the same type and only passes it on to `normalize`.
- F-4d-14 (confirmed, API break, small), `core/src/modal/search.rs:65`: The modal suite takes its unknown reasons and its memory account from the linear search (`linear::search::Reason`, `linear::search::memory::Account`), so the shared module F-arch-01 proposes must offer what modal uses of them, which is a small part.
- F-4e-03 (confirmed, API break, large), `core/src/ordinary/interactive.rs:28; core/src/ordinary/invert.rs:15; core/src/rules/interactive.rs:19; core/src/goal_tree.rs:13`: The ordinary session's public signatures use the linear suite's `GoalId` and `Inverted`, which `ordinary` does not re-export, while the rule-file session defines a second `GoalId` of its own; even the shared `goal_tree` walks the linear `InfId`.
- F-4f-09 (confirmed, API break, small), `core/src/linear/proofs/interactive/invert.rs:36; core/src/settings.rs:146`: The setting that says whether a front end applies the invertible rules, `interactive::Options { invertible }`, lives in the linear session's module, though the command and the page read it for ordinary logic's sessions as well.
- F-8-10 (confirmed, API break, medium), `web/src/ordinary.rs:841; web/src/ordinary.rs:1076; core/src/protocol/ordinary.rs:969`: Six ordinary-logic calls of the bindings build a default `linear::search::Options` only because the ordinary functions take one, so the page's `search` settings are ignored there while the protocol passes them to the same functions: the same decision runs under other options by front end.
- F-5a-09 (confirmed, API break, medium), `core/src/lambda/draw.rs:606`: A typing derivation ignores the user's label choice: `Subscript` and a user's `Table` draw the same fixed labels as `Upright`, and the labels live in two copies of one table.

### B3. The suites as peers in settings, errors, the registry and the front ends (D4)

9 findings (9 confirmed, 0 candidates); kinds: API break, refactor.

**What.** `Settings` gives the linear suite's options the generic keys (`search`, `view`, `input`) and `Error` has `Step` beside `OrdinaryStep`; the registry and the bindings list the linear suite first and export its calls without a prefix; the `seq` commands and the batch serve linear logic alone; the linear session's invertible steps, hint and cuts are computed in the bindings where ordinary logic's are library methods; the shared Kripke model documents itself in ordinary logic's words and has its methods in two suites; LaTeX and Typst have no call for a Kripke or modal countermodel; the labels of ordinary and modal statistics exist in the command alone.

**Change.** Each suite's items carry its name or none does; the page's order (ordinary, linear, modal, lambda) wherever an order shows (`api-proposal.md`, sections 2, 5 and 6).

**Net.** The wire forms change their keys once, with a wire level; the JSON Schema check, the behaviour lock and the client's calls show every site.

- F-arch-14 (confirmed, API break, large), `core/src/settings.rs:135; core/src/errors/mod.rs:302`: In `Settings` and in `Error` the linear suite's items carry no suite's name and the other suites' do, so the names make it the one that needs none.
- F-arch-15 (confirmed, API break, large), `web/src/lib.rs:97; core/src/protocol/mod.rs:418; core/src/protocol/mod.rs:732`: The bindings export the linear suite's calls without a prefix and every other suite's with one, and the protocol lists the linear suite first.
- F-6a-09 (confirmed, API break, large), `core/src/settings.rs:129`: `Settings`, "everything that configures a front end's calls", gives linear logic's options the generic names (`search`, `view`, `input`, `cut`, `interactive`) beside one key per other suite, and the bounds of ordinary logic's engines are set through `search`, which is `linear::search::Options`.
- F-7a-04 (confirmed, API break, medium), `cli/src/seq.rs:60`: The `seq` commands and the batch serve linear logic only: `seq print --logic` prints the linear image instead of the sequent, `seq json` and `seq fragment` take no other suite, nothing prints a modal sequent, and a batch refuses `--system`.
- F-7b-15 (confirmed, API break, medium), `cli/src/ordinary.rs:1127`: The library labels the linear engines' statistics (`Engine::counters`), but the labels of ordinary logic's methods and of the modal tableau are hand-written in the command alone, so a front end other than the command has none.
- F-4f-01 (confirmed, refactor, medium), `web/src/calls.rs:504; web/src/calls.rs:1184; web/src/calls.rs:478; core/src/ordinary/interactive.rs:1190; core/src/ordinary/interactive.rs:2542`: The linear session's invertible steps, its hint and its list of cuts are computed in the bindings, while the ordinary session has `invertible_steps` and `hint` in the library, and to read what the session already holds the bindings build two whole derivations per view with the derivation bound switched off.
- F-6b-10 (confirmed, API break, medium), `core/src/export/latex.rs:526; core/src/export/typst.rs:596`: LaTeX and Typst write a first-order countermodel as a table but have no call for a propositional Kripke countermodel or a modal one, and the reason the rules give for that no longer holds.
- F-4a-12 (confirmed, API break, small), `core/src/kripke.rs:36`: The Kripke model shared by ordinary and modal logic documents and prints its arrows in ordinary logic's reading (an order, "directly above", "below"), so the modal command writes its own printer.
- F-arch-21 (confirmed, API break, small), `core/src/ordinary/model.rs:175 and core/src/modal/model.rs:176; core/src/modal/model.rs:18`: The shared Kripke model has its methods written in two suites, undivided in rustdoc, and the modal check is bounded by a constant of the ordinary suite.

### B4. Building values and running an engine from outside the crate (X1 to X5)

18 findings (18 confirmed, 0 candidates); kinds: API break, refactor, text.

**What.** The consumer test's findings, confirmed from inside: a linear sequent, an ordinary derivation and a lambda-term can be made only from text or JSON, and a term or an atom with arguments of ordinary logic not at all; `Refusal` has no constructor and `Proof::build`, the stoppable form, is private; `Goal` hides its members; the engine interface is crate-private and an `Outcome` cannot be built; an outside engine can build three of the ten refutations; the exports draw a closed list of derivations; a rule file's positions and the ids of its derivation are not said. The crate itself works around the missing sequent builder in two places.

**Change.** Checked builders, constructors for refusals, a public `Decide` with `prove_with`, and the page of the manual that uses them (`api-proposal.md`, section 3).

**Net.** Additive but for `Outcome`'s fields, `linear::Sequent::new()`, and the ids of a rule file's derivation and of a lambda-term's atoms. The checkers stay the only way to a checked verdict; the manual's page is run by the readme test.

- F-arch-03 (confirmed, refactor, small), `core/src/limits.rs:429; core/src/limits.rs:593`: An engine outside the crate can neither poll the stop it is given nor refuse as the crate's engines do, because `Progress` and `Refusal` have no public constructor.
- F-arch-04 (confirmed, refactor, medium), `core/src/linear/search/mod.rs:1477; core/src/linear/search/mod.rs:3389`: No engine from another crate can answer in the crate's terms or be run by the front door: the engine interface is crate-private, and `Outcome` cannot be built.
- F-arch-05 (confirmed, refactor, medium), `core/src/linear/sequents/mod.rs:415; core/src/ordinary/derivation.rs:605; core/src/lambda/mod.rs:212`: Three values of the data model can be made only from text or JSON: a linear `Sequent`, an `ordinary::Derivation` and a `lambda::Term`.
- F-arch-10 (confirmed, API break, small), `core/src/protocol/mod.rs:354`: The docs of `protocol::Suite` say a suite of another crate may implement it, and the types make that impossible.
- F-arch-26 (confirmed, refactor, medium), `core/src/export/mod.rs:235; core/src/linear/proofs/style.rs:231`: The exports take a closed list of the crate's own derivations, so a derivation from outside is drawn only by rewriting it as a `rules::Derivation`, which the docs do not say.
- F-1a-01 (confirmed, API break, medium), `core/src/linear/sequents/mod.rs:217`: A linear sequent cannot be built through the public API without text or JSON. Ordinary and modal logic both can, through `Formulas::{atom, add}` and `Sequent::new(formulas, left, right)`.
- F-1a-08 (confirmed, refactor, small), `core/src/linear/occurrences/mod.rs:595`: The one test of axiom partners is crate-private, so an outside engine writes its own. That copy will be wrong for atoms with arguments if the test changes, which is the reason the rules give for having one predicate.
- F-1b-09 (confirmed, API break, small), `core/src/linear/proofs/mod.rs:443`: An outside engine cannot bound or stop the building of its proof. `Proof::new` polls a stop that never fires, and the variant that takes the engine's stop, `Proof::build`, is crate-private. Yet the code says an engine's proof can have hundreds of millions of nodes.
- F-2a-02 (confirmed, refactor, medium), `core/src/linear/search/mod.rs:1477; core/src/linear/search/mod.rs:2253; core/src/linear/search/focus/mod.rs:178`: Judged from inside the front door, the public engine interface of F-arch-04 can be smaller than proposed and needs three things the proposal leaves out: the stop as units (the front door already owns the work count and `Progress`), `admits` answering a `NotTaken` instead of an `Error`, and a name and counters for an outside engine, which `Engine::External` and the private `Counter::new` cannot carry.
- F-2a-03 (confirmed, refactor, small), `core/src/linear/search/mod.rs:407; core/src/linear/search/mod.rs:460`: A `Goal` hides its members: its fields and `occurrences()` are private, so whoever is handed a `Goal` (an outside engine under F-arch-04's interface, a front end) cannot read what it must prove.
- F-6a-03 (confirmed, API break, medium), `core/src/limits.rs:593`: An outside engine cannot answer in the crate's refusal: every variant of `Refusal` is a `#[non_exhaustive]` struct variant with no constructor, `Progress::new` is `pub(crate)`, and the crate itself spells out the same bound check by hand at about thirty places.
- F-1c-02 (confirmed, refactor, small), `core/src/linear/search/refutation.rs:222`: An outside engine can build only the refutations `Exhausted`, `Saturated` and `Basis`, and only for the sequent's roots: seven of the ten payload structs are `#[non_exhaustive]` without a constructor, and `Disproof::of_goal` is crate-private.
- F-4a-02 (confirmed, API break, medium), `core/src/ordinary/mod.rs:1123`: A first-order ordinary formula can be neither built nor read through the public API: the constructors of terms, predicates and binders are crate-private and behind `parse`, an atom with arguments is readable only as its canonical text, and `Inference::term` returns a term id with no public way to read the term.
- F-4a-16 (confirmed, refactor, medium), `core/src/ordinary/translate.rs:677`: The embedding builds its linear image with a private hash-consing builder and a struct expression of `linear::Sequent`'s crate-private fields: the crate itself needs the linear sequent builder that F-arch-05 and R59 propose for outside engines.
- F-5a-02 (confirmed, API break, medium), `core/src/lambda/mod.rs:809`: A `lambda::Term` can be made only from text or JSON: the builder and `from_parts` are crate-private, the shape check is behind `serialize`, and the ids have no constructor (confirms F-arch-05 from inside the module).
- F-5a-04 (confirmed, API break, small), `core/src/lambda/print.rs:348`: A user cannot print a type: the type printer is crate-private, and `Typing::types()` holds types the term does not, which only a judgement of the whole term ever prints.
- F-5b-03 (confirmed, text, small), `core/src/rules/calculus.rs:362`: Which written members of a rule need a position in a node's `at`, and in what order, is decided by the compiler and said nowhere public, so a rule-file author or an outside engine cannot write a node's positions without trial and error.
- F-5b-04 (confirmed, API break, medium), `core/src/rules/derivation.rs:66`: Building a derivation by hand mixes the ids `push` and `scope` return with raw `u32` indices that `Node` and `scope` take, so every builder in the crate converts back and forth.

### B5. What a value in hand promises (X3)

7 findings (7 confirmed, 0 candidates); kinds: API break, text.

**What.** Whether a proof has passed its checker is a private flag in one suite and a writable `bool` beside the verdict in another; `Verdict`, `Refutation` and the countermodel types do not say which values are certificates and which are only a search's word; `NormalForm` and `Interpolant` keep checked results in public mutable fields; what refutes an ordinary sequent is three optional fields beside the verdict; a first-order refutation says why it was not read back in a `String`.

**Change.** One rule, stated on the types: a value in a verdict has passed its check unless the verdict says otherwise, and a value built or read has not until `check` is called; the evidence moves into the verdict's variants (`api-proposal.md`, section 4).

**Net.** The outcome's JSON form changes with a wire level; the tests of each front door.

- F-arch-23 (confirmed, text, small), `core/src/ordinary/derivation.rs:510; core/src/linear/search/mod.rs:3408`: Whether a proof in hand has passed its checker is recorded in two ways in two suites, and neither is said on the types a user holds.
- F-arch-13 (confirmed, API break, medium), `core/src/ordinary/decide.rs:72; core/src/modal/decide.rs:106`: What refutes a sequent that is not valid is three optional fields beside the verdict in the ordinary outcome and one in the modal one, where the linear verdict carries its disproof in the variant; an outcome can then say "valid" and hold a countermodel.
- F-2a-09 (confirmed, text, small), `core/src/linear/search/mod.rs:3432; core/src/linear/search/mod.rs:1058`: `Verdict` does not say what its values promise: a `Proved` proof has passed the checker only when the outcome's `checked` says so, and an `Unprovable` refutation from the counts or the classical refuter is computed and not checked by `Disproof::check` before it is returned (only a saturated set or a basis is).
- F-1c-03 (confirmed, text, small), `core/src/linear/search/refutation.rs:15`: The docs of `Refutation` and `Disproof::new` do not say what a refutation promises: they say most refutations rest on a search, when nine of the ten kinds are certificates that `Disproof::check` verifies and only `Exhausted` rests on the search and can be built by anyone.
- F-4d-13 (confirmed, text, small), `core/src/modal/model.rs:498; core/src/ordinary/structure.rs:579; core/src/ordinary/kripke_structure.rs:996`: The countermodel types' field docs state as fact what only `check` establishes, while their `new` builds them unchecked, so a reader takes a value built by any crate for a checked one.
- F-4b-04 (confirmed, API break, medium), `core/src/ordinary/normal.rs:632; core/src/ordinary/interpolant.rs:176`: `NormalForm` and `Interpolant` keep their result and their checks in public mutable fields, so after `check` a caller can replace the result (or swap an outcome) and `is_checked()` still answers true for a value no check saw.
- F-4c-05 (confirmed, API break, medium), `core/src/ordinary/resolution/first_order.rs:1197`: A first-order refutation keeps why it was not read back as a `String` beside an `Option` derivation, so a stop, a memory refusal, the planned refusal for paramodulation and a read-back defect all become one text.

### B6. Names that agree across the suites

7 findings (7 confirmed, 0 candidates); kinds: API break.

**What.** The five formula stores name nodes, ids and atoms in five ways (`atom(name)` looks up in one suite and constructs in two); the three sessions call the same operations differently (`is_complete` and `is_finished`, `proof` and `derivation`, a `Result` and a panic); four functions of ordinary logic are `X` and `X_default` where the crate has `X` and `X_within`; `Error`'s variant names disagree with their stable codes; the names of values are snake case in one enum and kebab case in another, and the command mirrors them in 18 enums of its own.

**Change.** One table of names (`api-proposal.md`, section 5), applied in one step per suite.

**Net.** Renames only: the compiler, the tests and `breaks 0.8.0`. Value names in JSON change once, with the wire level.

- F-1a-05 (confirmed, API break, large), `core/src/linear/sequents/mod.rs:449`: The five formula arenas name the same things in five ways. One name, `atom(name)`, is a lookup in linear logic and a constructor in ordinary and modal logic. Two type names (`Side`, `Sign`) mean different things in different suites.
- F-4e-01 (confirmed, API break, medium), `core/src/rules/interactive.rs:430; core/src/ordinary/interactive.rs:1173; core/src/rules/interactive.rs:1484; core/src/ordinary/interactive.rs:3115`: The ordinary session and the rule-file session give different names and shapes to the operations they share (finished, goals, a goal, undo, the drawn inference's goal, the finished derivation), and the linear session differs again.
- F-4f-06 (confirmed, API break, medium), `core/src/linear/proofs/interactive/mod.rs:905; core/src/linear/proofs/interactive/terms.rs:38; core/src/linear/proofs/interactive/expand.rs:781; core/src/linear/proofs/interactive/mod.rs:1022`: The linear session's column of F-4e-01's table, which F-arch-12 asks for: it shares the ordinary session's names for goals, steps and undo, but calls its result `proof` where the other two say `finished`, and has five operations neither of the others has and lacks two they have.
- F-4a-13 (confirmed, API break, small), `core/src/ordinary/negative.rs:520`: The transformations name their short and full calls the other way round from the rest of the crate: `compare_default` is the short form and `compare` takes the search options, bounds and stop, where `prove`/`prove_within` and `decide`/`decide_within` use the plain name for the short form.
- F-6a-13 (confirmed, API break, large), `core/src/errors/mod.rs:1113`: `Error`'s variant names do not follow one pattern and disagree with their stable codes: `Model` is `invalid_model`, `Structure` is `invalid_structure`, `InvalidRefutation` (clauses) is `invalid_clause_refutation` while `Refutation` (a linear disproof) is `invalid_refutation`, `Reduction` is `reduction_rejected`, `Unsupported` (Rocq) is `no_certificate`.
- F-5a-06 (confirmed, API break, medium), `core/src/lambda/mod.rs:345`: `Calculus::NAMES` and `Strategy::NAMES` say the command writes these names, but the command spells them in kebab case, so the JSON's names do not read back as flags and the library does not read the flags' names.
- F-7a-03 (confirmed, API break, medium), `cli/src/argument_parsing.rs:2079`: Flag values are a second spelling of the library's names, held in 18 hand-made mirror enums, so a name the command writes in JSON or a settings file is refused as a flag value.

### B7. One path for each type, and nothing public without a user (C-HIDDEN, X9)

8 findings (8 confirmed, 0 candidates); kinds: API break, text.

**What.** 93 items are reachable under two or three paths because leaf modules are public and re-exported; the crate root re-exports items of two suites; 82 items are named by nothing outside `core/src`; a public field is documented as always false; a public constant sits in a private module.

**Change.** Leaf modules become private where their items are re-exported at the suite's root; the 82 are judged one by one against `inventories/surface.md` (`api-proposal.md`, section 7).

**Net.** `breaks 0.8.0` and the workspace's other crates, which name the paths that stay.

- F-1a-04 (confirmed, API break, medium), `core/src/linear/sequents/mod.rs:9`: In this unit 33 public types are reachable under two or three paths. The cause is that every leaf module is `pub` and is also re-exported by its parent and again by `linear`.
- F-1b-08 (confirmed, API break, small), `core/src/linear/proofs/mod.rs:21`: The modules `check`, `derivation`, `size` and `style` are public, and their items are re-exported at `proofs::` and again at `linear::`. So the unit's types are reachable under two or three paths each.
- F-3-08 (confirmed, API break, small), `core/src/linear/mod.rs:30`: The eight public types of proof nets are reachable as both `linear::X` and `linear::nets::X`. The rest of the nets API (`nets::exist`, `nets::Scratch`, `nets::cut`) exists only under `nets`, so a user writes both paths for one task.
- F-6a-06 (confirmed, API break, medium), `core/src/lib.rs:430`: The crate root mixes shared items with one suite's, and re-exports items under two paths: `Owner` and `Described` serve only linear errors, `StepFault` and `Backwards` only rule-file sessions and resolution by hand, while `limits` and `settings` are public modules whose items the root re-exports again (six items under two paths each).
- F-2c-14 (confirmed, text, small), `core/src/linear/lltp.rs:27; core/src/linear/mist.rs:46; core/src/linear/families.rs:45; core/src/linear/families.rs:63`: The three readers' examples import `prove`, `Options` and `Verdict` from `linear::search`, the second path of each, where the crate tells users to write `deduxit::linear::prove`.
- F-4a-14 (confirmed, API break, small), `core/src/ordinary/mod.rs:641`: `Options::method()` is public but answers wrongly for a first-order sequent and has no caller outside `ordinary`: every front end asks `method_for(sequent)`, which `decide` follows.
- F-4a-15 (confirmed, API break, small), `core/src/ordinary/decide.rs:403`: `CONNECTION_WORK` is declared `pub` and documented like the public `EVIDENCE_WORK` beside it, but its module is private and nothing re-exports it, so no user can name the bound that ends the default connection prover.
- F-6a-12 (confirmed, API break, small), `core/src/errors/parse.rs:31`: `ParseError::reserved` is a public field documented as always `false`; nothing sets it otherwise, and the `Display` arm that reads it is dead.

### B8. Common traits, conversions and small signatures

10 findings (10 confirmed, 0 candidates); kinds: API break, refactor.

**What.** `Proof`, `Forest`, `ProofStructure`, `ordinary::Sequent` and the readers' problems have no `Eq`, so users and the crate's tests compare by fields or by text; the public `Walk` is neither `Send` nor `Sync`; two `into_sequent` clone what they consume; `Proof::with_instances` takes by value what it reads; `Derivation::sequent` returns id slices; `race` has two thread knobs; three variants of a marked enum lack `#[non_exhaustive]`.

**Change.** Derive or implement the trait, borrow where the body borrows, return the value the caller wants (`api-proposal.md`, section 7).

**Net.** The compiler and the tests.

- F-1b-10 (confirmed, API break, small), `core/src/linear/proofs/mod.rs:414`: `Proof` implements neither `PartialEq` nor `Eq`, and neither does its `Forest`. So a user, and the crate's own tests, compare two proofs field by field.
- F-3-05 (confirmed, API break, small), `core/src/linear/nets/mod.rs:778`: The docs promise that two proofs differing in rule order give "the same net". Yet `ProofStructure` has no equality and no canonical link list, so the library's printers and exports and four tests each normalise and sort the links by hand.
- F-4a-17 (confirmed, refactor, small), `core/src/ordinary/mod.rs:1669`: `ordinary::Sequent` and `Formulas` implement neither `PartialEq` nor `Eq` while `linear::Sequent` does, so a user compares two ordinary sequents by their printed text.
- F-2c-16 (confirmed, refactor, small), `core/src/linear/lltp.rs:65; core/src/linear/mist.rs:75; core/src/linear/families.rs:84`: `lltp::Problem`, `mist::Problem` and `families::Instance` derive only `Clone` and `Debug`, though all their fields are `Eq`, so a user cannot compare two problems read or generated.
- F-1a-20 (confirmed, refactor, small), `core/src/linear/sequents/fmt.rs:165`: The public `Walk` boxes its operands' closure without `Send + Sync`, so a walk over a `Sync` sequent is neither `Send` nor `Sync`. No test asserts either.
- F-1a-13 (confirmed, refactor, small), `core/src/linear/occurrences/ill.rs:188`: `IllSequent::into_sequent` and `ClassicalSequent::into_sequent` consume the value but clone the sequent anyway, because `Forest` cannot give its sequent up.
- F-1b-11 (confirmed, API break, small), `core/src/linear/proofs/mod.rs:470`: `Proof::with_instances` takes the instance table by value but only reads it, so a caller that keeps its table must clone it.
- F-4a-09 (confirmed, API break, medium), `core/src/ordinary/derivation.rs:647`: `Derivation::sequent` returns the conclusion as a pair of id slices although the derivation holds an arena and both sides, so a caller who wants to print or decide the conclusion rebuilds a `Sequent` or its text by hand.
- F-2a-04 (confirmed, API break, small), `core/src/linear/search/mod.rs:560`: `race` takes a thread count beside the options and ignores `options.jobs` entirely, so the one search that the command and the harness run by default has two thread knobs, and both callers write the same branch around it.
- F-2a-05 (confirmed, API break, small), `core/src/linear/search/mod.rs:3559; core/src/linear/search/mod.rs:3566; core/src/linear/search/mod.rs:3576`: Three struct variants of the marked enum `Reason` lack `#[non_exhaustive]`, which every other struct variant of it has and the crate's decision requires, so adding a field to them later is a break.

### B9. Errors a front end can act on

4 findings (4 confirmed, 0 candidates); kinds: API break, refactor.

**What.** Only the linear checker's faults are structured: the Kripke model, the structures, a rule file's derivation and a calculus answer with a `String`, so a front end cannot point at the world or the node at fault, though the checker knows it. A refused step is three error types with two codes. The bindings write errors of the library's codes by hand, without the details the library writes.

**Change.** A structured fault per checker, one refusal of a step, and a library call that writes an error's JSON form (`api-proposal.md`, section 4).

**Net.** The errors' JSON forms (`docs/src`, the schema check) and the page's hints, which read them.

- F-6a-15 (confirmed, API break, large), `core/src/errors/mod.rs:694`: Only the linear checker's faults are structured (`CheckError` with the node, the rule, the premises and a `Fault`, and `describe` to print formulas); the checks of Kripke models, finite structures, rule-file derivations and calculi answer prose in a `reason: String`, so a front end cannot point at the world or node at fault.
- F-4e-06 (confirmed, API break, medium), `core/src/errors/mod.rs:1121`: A refused step is three error types with two stable codes: the linear and ordinary sessions answer `Error::Step` or `Error::OrdinaryStep` with code `step`, and the rule-file session answers `Error::StepRefused` with code `step_refused`, so the same refusal (no open goal 3) differs by suite in type, code and field type.
- F-8-05 (confirmed, API break, medium), `web-tab/src/failure.rs:42; web/src/calls.rs:1621; web/src/ordinary.rs:1173; web/src/ordinary.rs:1298`: The bindings write errors of the library's codes by hand without the details the library writes for them, and refuse the same condition with another code than the protocol does, because `Error::UnknownName` and the other variants cannot be built outside the crate.
- F-5b-05 (confirmed, refactor, small), `core/src/rules/check.rs:347`: The checker knows which node fails but writes it only into the message, so a front end or engine that wants to point at the node must parse text.

### B10. One description of what a suite offers: the protocol, the sessions, the documents

15 findings (15 confirmed, 0 candidates); kinds: API break, refactor.

**What.** The protocol's registry is written four times in one file and has drifted; the bindings hold a second protocol for the sessions, with every session call written three times; `describe` answers three hand-kept descriptions that already disagree; which document a text is gets probed in six places; reading one document from text is three lines written 45 times across the front ends; the table of built-in rule files by name is written twice, in the command and in the bindings, with different contents; the protocol's lambda operations answer without the text its other terms carry; the workspace's states and the id newtypes have no serde form, which costs about 600 lines of mirror types.

**Change.** The registry written once and read for everything else; the session as a trait over JSON in the protocol; `read_document`, a table of the built-in calculi and a document's kind as library calls (`api-proposal.md`, section 5).

**Net.** The bindings' tests under Node, the protocol's tests, the wire-schema check; the client's calls change names once.

- F-arch-11 (confirmed, refactor, small), `core/src/protocol/mod.rs:505; core/src/protocol/mod.rs:432; core/src/protocol/mod.rs:706`: The protocol's registry is written four times in one file, and one copy has drifted: `KINDS` names no kind of the lambda suite.
- F-arch-12 (confirmed, API break, large), `web/src/protocol.rs:151; web/src/lib.rs:557; core/src/rules/interactive.rs:430; core/src/linear/proofs/interactive/mod.rs:905`: The three sessions have no interface in common, so every session call is written three times in the bindings and described in a second protocol there, outside the library's.
- F-8-06 (confirmed, API break, medium), `web/src/calls.rs:63; web/src/protocol.rs:482; core/src/protocol/modal.rs:420`: `describe` answers three hand-kept descriptions of the suites in one document (the bindings' `about`, their `SUITES` table and the library's catalogue), which already disagree on a suite's name, and `about`, the bindings' largest function, hand-writes each suite's start parameter because `protocol::Suite` does not give it.
- F-8-13 (confirmed, refactor, small), `core/src/protocol/linear.rs:636; core/src/protocol/modal.rs:519; core/src/protocol/lambda.rs:537; core/src/protocol/linear.rs:529`: Following F-arch-11: the protocol writes "export a drawable in a format" five times with four `Format` structs and three `FORMATS` lists (the lambda copy lists formats the build may lack), and builds a `Limit` from a `Reason` five times.
- F-8-16 (confirmed, API break, medium), `web/src/calls.rs:215; core/src/protocol/linear.rs:329; web/src/calls.rs:2322; core/src/protocol/linear.rs:371`: Which linear document a text is (a session, a proof, a net with or without boxes, an outcome, an intuitionistic sequent) is told by probing keys in six places across the bindings and the protocol, by different keys (the bindings tell a session by `inferences`, the protocol by `history`), each parsing the text twice.
- F-8-04 (confirmed, API break, medium), `core/src/protocol/mod.rs:802; web/src/calls.rs:1720; web/src/modal.rs:191; web/src/ordinary.rs:1007`: The library has no public call that reads one document from text and refuses what follows it, so its consumers write the three-line `Deserializer` + `wire::upgrade` + `end()` 45 times and eight bindings sites forget `end()`, accepting trailing text that the other calls refuse; the protocol keeps exactly that call private.
- F-7a-17 (confirmed, refactor, small), `cli/src/check.rs:342`: `read_document` reads a whole document of a given type six times by the same five lines.
- F-5b-02 (confirmed, API break, large), `cli/src/calculus.rs:28; web/src/rules.rs:38; web/src/rules.rs:55; core/src/protocol/rules.rs:207; core/src/rules/natural.rs:84`: The library exports thirteen rule-file constants but no table of them by name, so the command, the bindings (twice) and the protocol each write their own, and the protocol's knows only three.
- F-7b-10 (confirmed, API break, medium), `cli/src/calculus.rs:28`: Three front ends keep their own tables of the built-in calculi by name, with different contents: the command knows 13 names plus the modal `G3…` calculi, the bindings know 13, and the protocol knows 3.
- F-8-07 (confirmed, API break, small), `web/src/rules.rs:38; cli/src/calculus.rs:28; core/src/protocol/rules.rs:207`: The table of built-in rule files by name is written three times, twice in full with thirteen rows and a hand-kept list of names beside one of them, because the library exports the files as constants without a lookup by name.
- F-8-09 (confirmed, API break, medium), `web/src/lambda.rs:143; web/src/modal.rs:122; core/src/protocol/lambda.rs:498`: The protocol's lambda operations answer terms without the `text` and `judgement` that its `linear.term` and `ordinary.term` write, and its drawings and texts come wrapped in JSON, so the bindings parse each answer, re-read every term from a re-serialized `Value` to print it, and unwrap every SVG from its JSON string.
- F-8-11 (confirmed, API break, small), `web/src/workspace.rs:461; web/src/workspace.rs:520; core/src/workspace/mod.rs:263`: The workspace's `State`, `Cause` and `End` have no serde form, so the bindings spell each variant by hand with an `"other"` fallback, and they spell `protocol::Cost` and `Class` by hand too although both derive `Serialize` in snake case.
- F-6a-07 (confirmed, refactor, large), `core/src/serialize/errors.rs:238`: Most id newtypes have no serde while a few have it transparent, so about 600 lines of `serialize/` write forms by hand: mirror enums with two converters (`Why` for `Reason`, `WhyNot` for `Refutation`) and hand-written `Serialize` for `Fault`, `ShapeError`, `NetError`, `NotTaken` and `Placement`, where `Refusal`, `StepFault` and `WitnessFault` derive the same tagged shape.
- F-8-08 (confirmed, API break, small), `web/src/calls.rs:1588; web/src/calls.rs:1853; cli/src/show.rs:849`: Drawing a classical proof two-sided needs the sequent's writing, and a session or proof read from JSON has none, so the same fallback (read the one-sided sequent again with unbounded limits, then `TwoSided::new`) is written three times in two front ends.
- F-7b-06 (confirmed, API break, small), `cli/src/interact/rules.rs:434`: In a session in a calculus given as data, `proof FILE` always writes JSON and `show` has no formats, where the other two sessions take `--latex`, `--typst`, `--svg`, `--png` and `--pdf` and pick the format from the file's extension. `proof c.tex` thus writes JSON into a `.tex` file.

## C. Duplication: one thing written more than once

Behaviour-preserving refactors. Each names the shared form; the net is the tests and, in the linear search, the target sets' counters. They rank by how much code one copy removes and how often the copies have already drifted apart. (90 findings in 12 themes.)

### C1. The arena of propositional formulas, twice

2 findings (2 confirmed, 0 candidates); kinds: refactor.

**What.** Ordinary and modal logic each have the hash-consed arena, its sequent, its parser and its JSON form: 17 duplicate pairs. The plan recorded the copy as waiting for step 74, which has landed.

**Change.** A crate-private `Arena<N>` at the root with the node's shape as a trait; the two `Formulas` stay public types that hold one each; the modal wire form is the ordinary one with two more tags.

**Net.** The parsers' and serializers' tests of both suites; `read-tptp` and `ordinary-pigeons` in the ratchet.

- F-arch-08 (confirmed, refactor, medium), `core/src/modal/mod.rs:170 and core/src/ordinary/mod.rs:824; core/src/modal/parse.rs:333 and core/src/ordinary/parse.rs:771; core/src/serialize/modal.rs:67 and core/src/serialize/ordinary.rs:71`: The hash-consed arena of propositional formulas, its sequent, its parser and its JSON form exist twice, in the ordinary and the modal suite, and the step the plan waited for has landed.
- F-6a-11 (confirmed, refactor, medium), `core/src/serialize/modal.rs:36`: The modal forms copy ordinary logic's: the formula form, its conversion, the sequent form and its reader, and the verdict word are written twice, though the modal sequent is "an ordinary sequent's form with the formulas `□` and `◇`" (`wire.rs:159-163`).

### C2. Ordinary logic's provers and their builders of LK

5 findings (5 confirmed, 0 candidates); kinds: refactor.

**What.** The intuitionistic connection prover is the classical one copied with seven hook points (the crate's largest duplicates, up to 115 lines), and nothing pins either search's counters. Six private builders of a derivation of LK or LJ write the same `push` with a memory estimate of their own, and Herbrand's builder repeats the connection read-back's.

**Change.** One connection search generic over what the intuitionistic prover adds, monomorphised; one derivation builder with one charge rule. Pin the connection searches' statistics on a set of problems first.

**Net.** The new pin (a set of ILTP rows with their statistics), then equality of it. A merge without the pin is a search change.

- F-arch-24 (confirmed, refactor, medium), `core/src/ordinary/connection/prove.rs:650 and core/src/ordinary/connection/intuitionistic/prove.rs:705`: The intuitionistic connection prover repeats the classical one's search loop: thirteen duplicates of up to 115 lines.
- F-4c-02 (confirmed, refactor, medium), `core/src/ordinary/connection/prove.rs:347 and core/src/ordinary/connection/intuitionistic/prove.rs:358`: F-arch-24 is confirmed from inside both files: the intuitionistic search is the classical one copied, with seven hook points, and nothing pins either search's counters today.
- F-4c-03 (confirmed, refactor, medium), `core/src/ordinary/herbrand.rs:1065 (Build) and core/src/ordinary/connection/read_back.rs:113 (ReadBack)`: Herbrand's derivation builder repeats the classical connection read-back's construction of LK: frames, steps, positions, the axiom with weakenings, the inference push with its memory account, the rule order, and the one- and two-premise rules.
- F-4b-01 (confirmed, refactor, large), `core/src/ordinary/ljt.rs:931; core/src/ordinary/sat/read_back.rs:840; core/src/ordinary/connection/intuitionistic/read_back.rs:1233; core/src/ordinary/connection/read_back.rs:514; core/src/ordinary/herbrand.rs:1176; core/src/ordinary/derivation.rs:1779`: Six private builders of a derivation of LK or LJ each write the same `push` (ask the stop, charge memory by an estimate of its own, refuse past 2³² inferences) and four of them the same `weaken_to`, `contract_to`, `in_order` and multiset `difference`, while no public way exists for an outside engine to build such a derivation for the checker.
- F-4c-08 (confirmed, refactor, small), `core/src/ordinary/connection/matrix.rs:46 and core/src/ordinary/connection/intuitionistic/prefix.rs:44; matrix.rs:1048 and prefix.rs:492; connection/read_back.rs:178, intuitionistic/read_back.rs:176, herbrand.rs:1858`: Small helpers of the connection module and of Herbrand exist two to four times word for word: the build's `Budget`, the sorted `union`, the scan for the first eigenvariable number past the arena's, the statistics copied from `Counts`, and the mapping of a stopped build to unknown.

### C3. The exports: two writers over two spellings, and the certificate writers

8 findings (8 confirmed, 0 candidates); kinds: refactor.

**What.** LaTeX and Typst are one writer over two tables of spellings (six duplicates, the dispatch from `Drawable` in five places, an escape written four times); the two SVG net drawings repeat their emitters; the Rocq writers repeat their formula walks, their constructor tables and their helpers under three conventions for bounding text.

**Change.** One text writer parameterised by a format's spellings; one set of SVG emitters; one module of certificate helpers and one table of the Rocq library's constructor names, which is also the contract F-9-01 asks to be listed.

**Net.** The snapshot tests (`core/tests/export.rs`) byte for byte, the flake's `export`, `rocq` and `rocq-deduxit` checks. A certificate's text must not change.

- F-arch-25 (confirmed, refactor, medium), `core/src/export/latex.rs:480 and core/src/export/typst.rs:550`: The LaTeX and the Typst writer are the same code over two spellings: six duplicates between the two files, the longest 90 lines.
- F-6b-13 (confirmed, refactor, small), `core/src/export/latex.rs:676; core/src/export/typst.rs:756; core/src/export/svg/mod.rs:1483; core/src/export/rocq/mod.rs:1035`: The match from `Drawable` to a format's generic writer is written in four writers and in `judgement`, so a new kind of derivation adds an arm in five places.
- F-6b-14 (confirmed, refactor, small), `core/src/export/typst.rs:160; core/src/export/latex.rs:231`: The Typst string escape is written four times and the LaTeX upright-text escape twice.
- F-6b-04 (confirmed, refactor, medium), `core/src/export/svg/mell.rs:1694 and core/src/export/svg/net.rs:782`: The drawings of nets with and without boxes repeat the same emitters: the group buffers and their document body, the link arcs, the cut bars, the title, the description and the per-occurrence size estimate.
- F-6b-11 (confirmed, refactor, small), `core/src/export/models.rs:107 and core/src/export/svg/mod.rs:1849`: The ordinary countermodel's closing sentence and its sequent-size estimate are written twice, in `models.rs` and in `svg::countermodel`.
- F-6b-05 (confirmed, refactor, small), `core/src/export/rocq/deduxit.rs:896 and core/src/export/rocq/deduxit.rs:943`: The propositional formula writers `formula` and `iformula` are one walk written twice, and each repeats inline the constructor table that `one_sided` and `two_sided` in the same file already hold.
- F-4d-08 (confirmed, refactor, small), `core/src/ordinary/rocq.rs:928; core/src/ordinary/rocq_models.rs:442; core/src/modal/rocq.rs:95; core/src/modal/rocq.rs:522`: The Rocq library's formula constructors (`var`, `top`, `bot`, `neg`, `wedge`, `vee`, `imp`, `equiv`, `box`, `dia`, `fall`, `fex`), part of the certificates' contract with `rocq-deduxit`, are spelt out in four walks of this unit, beside the two of the exporter (F-6b-05).
- F-4d-09 (confirmed, refactor, small), `core/src/modal/rocq.rs:24 and core/src/ordinary/rocq_models.rs:44; core/src/modal/rocq.rs:57, core/src/ordinary/rocq.rs:969 and core/src/export/rocq/deduxit.rs:1301`: The five certificate writers of this unit repeat their helpers (a bounded text, the Rocq string quote, the stop's refusal) and follow three conventions for bounding their text and two for their stop.

### C4. Proof nets: the graph kernel and the sequentialization, twice

4 findings (4 confirmed, 0 candidates); kinds: refactor.

**What.** The correctness criterion's graph code (the layout, the scratch, Tarjan's bridges, the deletion rounds) and the assembly of a sequentialized proof exist once for nets without boxes and once for nets with them. The two structures stay two types: a vertex is an occurrence on the net engine's hot path.

**Change.** A shared graph kernel and a shared proof assembler, generic over what a vertex is, monomorphised.

**Net.** The net target set's `verdict`, `links` and `tests` equal to `net-47.csv`; touches the correctness criterion, a checker, so the panel.

- F-3-02 (confirmed, refactor, medium), `core/src/linear/nets/graph.rs:283; core/src/linear/nets/mell/graph.rs:502`: The graph kernel of the correctness criterion is written twice, once for `ProofStructure` and once for `MellStructure`: the CSR layout, `Scratch`, the iterative Tarjan bridge search, Yeo's deletion rounds and the isolation of a cycle witness. The copies differ only in how a switched vertex finds its parent edge.
- F-3-06 (confirmed, refactor, medium), `core/src/linear/nets/sequentialize.rs:177; core/src/linear/nets/mell/sequentialize.rs:187`: The two sequentializations repeat their machinery for assembling a proof: the explicit step stack, `push`, `premise`, and the `Mix`, `Tensor` and `Cut` arms of the loop.
- F-3-07 (confirmed, refactor, small), `core/src/linear/nets/mod.rs:1562; core/src/linear/nets/mell/mod.rs:815`: The two structures repeat three small pieces: the criterion's verdict line, the cut-link accessors, and the axiom case of cut elimination.
- F-3-13 (confirmed, refactor, small), `core/src/linear/search/essential.rs:337`: The set-up of the essential engine's closure writes the same row-filling loop twice, once for input occurrences from the last up and once for outputs from the first down.

### C5. Walks over formulas and terms, and copies between arenas

9 findings (9 confirmed, 0 candidates); kinds: refactor.

**What.** The walk that visits every node and term a formula reaches once is written nine times in ordinary logic with different seen-sets, three more times in cut elimination and ten times over lambda-terms; copying formulas from one arena into another is written five times beside `ordinary::transfer::copy`; two walkers substitute for de Bruijn variables; the polarity pass exists three times.

**Change.** One iterator per arena (public for lambda-terms, where a user needs it) and `transfer::copy` everywhere.

**Net.** The tests; `ordinary-pigeons` and `read-tptp` in the ratchet.

- F-4a-08 (confirmed, refactor, medium), `core/src/ordinary/instance.rs:122; core/src/ordinary/herbrand.rs:760`: The walk "every term in these formulas, each shared node and term once" is written five times, with two seen-sets and two stacks each.
- F-4d-07 (confirmed, refactor, small), `core/src/ordinary/transfer.rs:290; core/src/ordinary/structure.rs:192; core/src/ordinary/kripke_structure.rs:396; core/src/ordinary/rocq_models.rs:309`: The walk "the nodes and terms the sequent's formulas reach, each once" is written four more times in this unit, beside the five F-4a-08 found, here with `Vec<bool>` seen-sets.
- F-4c-16 (confirmed, refactor, small), `core/src/ordinary/herbrand.rs:350, :759 and core/src/ordinary/resolution/first_order.rs:1396`: The walk that collects the terms of a formula's atoms, nodes and terms each seen once, is written again in Herbrand's universe, Herbrand's witnesses, resolution's open-witness check and three times in `instance.rs`.
- F-4b-07 (confirmed, refactor, medium), `core/src/ordinary/normal.rs:1338; core/src/ordinary/normal.rs:1437; core/src/ordinary/written/mod.rs:769; core/src/ordinary/transfer.rs:74`: Copying formulas from one arena into another by atom and symbol names is written three more times in this unit beside the crate's `transfer::copy`, which already takes a set of roots or the whole arena.
- F-4e-10 (confirmed, refactor, small), `core/src/ordinary/interactive.rs:2654; core/src/ordinary/written/mod.rs:776`: The copy of a propositional arena into another, node by node, is written twice by hand (`Interactive::ids_of` and `WrittenTableau::differs`), though `transfer::copy` does it, every atom by name, for `roots: None`.
- F-4a-11 (confirmed, refactor, medium), `core/src/ordinary/bruijn.rs:222; core/src/ordinary/instance.rs:621`: Two walkers substitute for de Bruijn variables over the same arena, each with its own memo of `(NodeId, depth)` and its own term walk: `bruijn.rs` for lifting, lowering, open substitution and abstraction, `instance.rs` for closed instances, frames and open witnesses.
- F-5a-03 (confirmed, refactor, medium), `core/src/lambda/mod.rs:1304`: The walks over a node's subterms and variables and over a type's operands are written ten times across the module, each listing every variant, and none is public, so a user who follows a term's subterms writes an eleventh.
- F-1c-07 (confirmed, refactor, small), `core/src/linear/proofs/cut/framed.rs:311`: `framed.rs` writes the memoised postorder walk over first-order terms three times and the walk down a frame chain three times.
- F-4b-06 (confirmed, refactor, medium), `core/src/ordinary/sat/classical.rs:57; core/src/ordinary/normal.rs:2872; core/src/ordinary/sat/mod.rs:539`: The pass that gives every subformula its polarity is written three times, two of them the same function (the classical encoder's and the Plaisted–Greenbaum normal form's, the latter over a SipHash `HashMap`), the third differing only in `⊥` and `↔`.

### C6. Meters, refusals and memory helpers written per module

4 findings (4 confirmed, 0 candidates); kinds: refactor.

**What.** The front door writes the same work meter three times, lambda-terms have three meters in three units, `rules` and `modal` hold one each, and `Refusal::Stopped` is built by hand at 103 sites with four private `stopped()` helpers. A hash table's bytes are estimated in three ways in one engine, and growing a heap within the account is written twice.

**Change.** The public `limits::Meter` and `Refusal` constructors of group B serve the crate first; one estimate of a table's bytes in the memory module.

**Net.** The tests of limits; the target sets' counters, since a meter counts work.

- F-2a-10 (confirmed, refactor, medium), `core/src/linear/search/mod.rs:924; core/src/linear/search/mod.rs:1720; core/src/linear/search/mod.rs:1082`: The front door writes the same meter three times (count units, tell the caller a `Progress`, test the work bound), the shape F-arch-03 proposes as a public `limits::Meter`.
- F-5a-05 (confirmed, refactor, medium), `core/src/lambda/check.rs:762`: The suite has three work meters of its own (`check::Meter`, `reduce::Budget`, `inhabit::Search::tick`), and they report `Progress` in three different units, none as `Progress` documents.
- F-2b-07 (confirmed, refactor, small), `core/src/linear/search/focus/memo.rs:238; core/src/linear/search/focus/waiting.rs:309; core/src/linear/search/focus/framed.rs:1878`: The bytes of a hash table are estimated in three different ways in this unit, and the first-order zone's way counts fewer than the other two.
- F-2c-15 (confirmed, refactor, small), `core/src/linear/search/horn/reach.rs:479; core/src/linear/search/horn/cover.rs:503`: Growing a `BinaryHeap` within the memory account is written twice in the Horn engine, beside the `Vec` helper `room`, which lives in `reach.rs` though `cover.rs` and `mod.rs` use it too.

### C7. Sorted-multiset operations beside `proofs::multiset::Multiset`

2 findings (2 confirmed, 0 candidates); kinds: refactor.

**What.** The saturated set's check and the inverse method each write insert, remove, sum, difference and inclusion over sorted vectors, which the proof checker's `Multiset` has.

**Change.** Use `Multiset`, moved where both can name it.

**Net.** The saturated check is a checker (the panel); the inverse method's rows of the target set.

- F-1c-11 (confirmed, refactor, small), `core/src/linear/search/saturated.rs:489`: The saturated set's check writes its own sorted-multiset operations, which `proofs::multiset::Multiset`, the proof checker's, already has.
- F-2c-11 (confirmed, refactor, medium), `core/src/linear/search/inverse/eval.rs:47; core/src/linear/proofs/multiset.rs:64`: The inverse method writes its own sorted-multiset operations over `Vec<OccId>` (insert, remove, sum, difference, inclusion), which the crate-private `proofs::multiset::Multiset` already provides to derivations, cut elimination and the sessions.

### C8. The sessions and their drivers

8 findings (8 confirmed, 0 candidates); kinds: refactor.

**What.** The reader of a witness term and `derivation_within` are written in two sessions; `formula` is one body three times; each session repeats blocks of its own; the command implements undo, the invertible switch, close, show, save and load once per session, and the bindings repeat their glue per suite.

**Change.** Shared functions at the root for what the sessions share; in the front ends, one driver over the session trait of B10.

**Net.** The sessions' tests, the command's tests of `interact`, the bindings' tests.

- F-4e-04 (confirmed, refactor, medium), `core/src/ordinary/interactive.rs:2061; core/src/linear/proofs/interactive/witness.rs:43`: The reader of a witness term typed as text is written twice, in the ordinary session and the linear one: the same tokenizer, the same stack of open applications, and the same `WitnessFault` refusals.
- F-4e-11 (confirmed, refactor, small), `core/src/ordinary/interactive.rs:3027; core/src/linear/proofs/interactive/mod.rs:984`: `derivation_within` is the same in the ordinary and linear sessions (the estimate refused past `derivation_bytes`, one stop, the postorder renumbered), and the ordinary file renumbers the postorder a second time in `derivation_ids`.
- F-4e-12 (confirmed, refactor, small), `core/src/ordinary/interactive.rs:2719; core/src/ordinary/interactive.rs:2784`: Inside `ordinary/interactive.rs` three blocks are written several times: the fitting of a graft's root onto the goal (twice), the mapping from a rule's shape to what it needs (twice), and the next eigenvariable number or its refusal (four times).
- F-4e-13 (confirmed, refactor, small), `core/src/rules/interactive.rs:925`: The rule-file session looks a rule up by name, and turns `Unplanned` into an `Error`, by the same lines in `apply`, `derive`, `extend` and `premise_bindings`, and maps a checker's refusal to `StepFault::Check` by the same closure in `verify` and `verify_all`.
- F-4f-08 (confirmed, refactor, small), `core/src/linear/proofs/interactive/cut.rs:431; core/src/linear/proofs/interactive/expand.rs:526`: Inside the unit six small blocks are written two to five times: the open goal's `Inference` literal and the bookkeeping of a step (`record` and `cut`), the step's number with its cast, the mapping of a graft's error, the order of a goal's formulas for the search, the check of a split's positions, and the list of predicates.
- F-1b-07 (confirmed, refactor, small), `core/src/linear/proofs/mod.rs:679`: `Proof::formula`, `Derivation::formula` and `Interactive::formula` are three copies of one function. Each builds a `String` and returns it as `impl Display + '_`, and only the first cuts an instance's terms.
- F-7b-08 (confirmed, refactor, medium), `cli/src/interact/mod.rs:594`: The linear and the ordinary sessions each implement `undo` with its step groups, `invertible on|off`, the `close` loop, `show`, `save`, `load`, `invert_all`, `stepped`, `goals` and `opened` (about 250 lines) as near-identical copies. The `Commands` trait shares only the read loop.
- F-8-14 (confirmed, refactor, small), `web/src/calls.rs:885; web/src/calls.rs:936; web/src/ordinary.rs:406`: The bindings' modules repeat their glue: the loop that applies the invertible rules after a step three times, the `written`/`session_of`/`sequent_of` helpers in each module, and per-suite copies of `never`, `ran`, `Drawing` and `Text`.

### C9. Repeats inside the linear suite

23 findings (23 confirmed, 0 candidates); kinds: API break, refactor.

**What.** Tables and loops written two to seven times within one module: the printer's two-sided layout, the fragment's names, the parser's application, the rule names' parallel tables, the 37 variants of `ClassicalRule` over six tables, the excess interval, two Horn readers, `Reason::as_set` applied by the engines and again by the front door, the focused engine's split loops, keys and buffer pools, the Horn engine's merge ladders. One touches the checker: the unrestricted zone's weight sum is changed inline, outside the functions Kani checks.

**Change.** One function or table each. F-1b-05 moves the arithmetic into the checked functions and changes Kani's harnesses with it.

**Net.** Both target sets' counters and the ratchet; Kani's harnesses and the panel for F-1b-05.

- F-1a-10 (confirmed, refactor, small), `core/src/linear/occurrences/instances.rs:1030`: The term printer that uses an explicit stack is written twice, line for line: once over the sequent's tables and once over a proof's two arenas. A trait over both stores already exists.
- F-1a-11 (confirmed, refactor, small), `core/src/linear/sequents/notation.rs:998`: The printer writes the layout of a two-sided sequent four times and that of a one-sided sequent three times. The layout is the formulas separated by commas, the alignment and the turnstile. The pair of mark characters is spelled three times, without a name.
- F-1a-16 (confirmed, refactor, small), `core/src/linear/fragment.rs:220`: `Fragment::name_in` repeats the twelve-arm table of `Fragment::name`, differing only by an `I` before each name. `duplicates.md` lists the pair (209 tokens).
- F-1a-17 (confirmed, refactor, small), `core/src/linear/parse/mod.rs:556`: The parser closes an application into a term with the same four lines in three places.
- F-1b-05 (confirmed, refactor, small), `core/src/linear/proofs/check/pass.rs:790`: The pass changes the weight sum of the unrestricted zone (`State::unrestricted`) with plain `+=` and `-=` inline in `Pass`. These are outside the `State` functions Kani's harnesses call, and the harnesses fix that sum at 0. So the rules file's claim that "the arithmetic of the pass lives in functions the harnesses call" does not hold for it.
- F-1b-06 (confirmed, API break, medium), `core/src/linear/proofs/derivation/classical.rs:28`: `ClassicalRule` spells out every pair of a one-sided rule and a side as its own variant (37 of them). Six parallel tables map it back. This is the shape that `core-proofs.md`'s decision removed from `Rule`/`Named`.
- F-1b-12 (confirmed, refactor, small), `core/src/linear/proofs/rule.rs:498`: Rule names are spelled in parallel tables kept in step by hand. Inside `rule.rs`, `KNOWN` and the Unicode half of `FromStr` repeat what `Named::name` returns for every entry of `Named::ALL`.
- F-1c-04 (confirmed, refactor, small), `core/src/linear/search/weighted.rs:284; core/src/linear/search/basis.rs:124`: The two checks that read a goal as a Horn program, the state equation's dead atoms and the basis, each write their own reader: the same head test, the same clause loop and the same least-fixpoint of reached places.
- F-1c-05 (confirmed, refactor, small), `core/src/linear/search/refutation.rs:1379; core/src/linear/search/refutation.rs:1508`: The balance check and the one-sided count check each compute the excess interval over the goal with the same walk, and each repeats the test that no atom of the predicate is open.
- F-1c-08 (confirmed, refactor, small), `core/src/linear/proofs/cut/mod.rs:341; core/src/linear/proofs/cut/mod.rs:464`: `step` and `eliminate` each write the choice of the cut (the caller's `at` if it is a cut, else `NoCut`, else the strategy's) and check the proof a step made in two different ways.
- F-1c-09 (confirmed, refactor, small), `core/src/linear/search/saturated.rs:81; core/src/linear/search/basis.rs:58`: The saturated set's check and the basis's check each define the same private `Failed` enum, so every caller matches the two alike twice.
- F-1c-14 (confirmed, refactor, small), `core/src/linear/search/refutation.rs:746`: `write_unbalanced` builds the two-sided phrase and the intuitionistic phrase with the same code twice, differing only in the parenthesis that ends them.
- F-2a-06 (confirmed, refactor, medium), `core/src/linear/search/additive.rs:115; core/src/linear/search/mod.rs:1137`: `Reason::as_set` is applied by the engines and again by `conclude`, which applies it to every engine's unknown; the engine-side calls are leftovers of one rule written seven times.
- F-2a-07 (confirmed, refactor, small), `core/src/linear/search/mod.rs:788; core/src/linear/search/mod.rs:1704; core/src/linear/search/mod.rs:1532`: `Turns::asked` and the `probes` call inside `Turns::take` are dead: `take` is reached only from the branch that already found `probes(&task, options)` true for the same task and options, and the dispatch table is scanned three times for the one row.
- F-2a-11 (confirmed, refactor, small), `core/src/linear/search/mod.rs:696; core/src/linear/search/mod.rs:985; core/src/linear/search/mod.rs:1028`: The goal's description travels through three private structs with the same fields (`Task`, `Answered`, `Concluding`), and `conclude` takes eight arguments, two of them the parts of an `Answer` and one the goal's members that its `Goal` already has.
- F-2b-02 (confirmed, refactor, small), `core/src/linear/search/focus/keyed.rs:151; core/src/linear/search/focus/mod.rs:1344`: The propositional and first-order paths through the memo and the loop check repeat their bookkeeping: the branch-stack push is copied verbatim, and the pop with its depth, the settling of a dependency on the sequent itself and the choice between Complete and Hereditary are each written two or three times.
- F-2b-05 (confirmed, refactor, medium), `core/src/linear/search/focus/split.rs:607; core/src/linear/search/focus/split.rs:661`: `search_splits` and `search_splits_framed` are the same loop over `next_split` with the same join, and the first-order one adds only the choice log's three calls.
- F-2b-12 (confirmed, refactor, small), `core/src/linear/search/focus/framed.rs:1275; core/src/linear/search/focus/framed.rs:1311`: `own_key` and `canonical_key` write the same two-zone layout in the same double loop and differ only in the sort order and in how a word is mapped.
- F-2b-14 (confirmed, refactor, medium), `core/src/linear/search/focus/scratch.rs:22`: The engine's buffer pools are nine hand-written take/give pairs of two shapes, one per kind of buffer.
- F-2b-15 (confirmed, refactor, small), `core/src/linear/search/focus/mod.rs:1589; core/src/linear/search/focus/mod.rs:754`: In mod.rs, `remember` writes its collect-and-rename `match` twice, differing only in one flag, and `statistics` and `report` repeat the first-order counters' addition.
- F-2c-06 (confirmed, refactor, medium), `core/src/linear/search/horn/reach.rs:684; core/src/linear/search/horn/equation.rs:303; core/src/linear/search/horn/cover.rs:316; core/src/linear/search/horn/mod.rs:241`: The Horn engine merges a transition's two sorted arc lists, inputs and outputs, by place in four hand-written loops, two of them with the same five-arm `match` ladder.
- F-2c-13 (confirmed, refactor, small), `core/src/linear/search/horn/mod.rs:620; core/src/linear/search/horn/mod.rs:853`: `Reader::clause` repeats `clause_head`'s walk over a clause's factors, and the proof builder calls `clause_head` again twice for every firing, on clauses the program has already read.
- F-2c-17 (confirmed, refactor, small), `core/src/linear/search/inverse/rules.rs:768; core/src/linear/search/inverse/rules.rs:818`: `compile` and `goal` each build the rule writer with the same eleven-field struct literal.

### C10. Repeats inside ordinary and modal logic

8 findings (8 confirmed, 0 candidates); kinds: refactor.

**What.** The rules of LK and LJ and their spellings exist in the library and again in the command; four enums repeat a `FromStr`; the finite structure and the Kripke model with domains each have an evaluator of one semantics; three SAT encodings write one writer; the SAT method's two entry points and the normal forms' helpers repeat themselves; the modal search iterates masked bits in four loops.

**Change.** One function or table each; the rules' table becomes public, since the command needs it.

**Net.** The tests; the model checks are checkers (the panel for F-4d-02).

- F-4a-07 (confirmed, refactor, small), `core/src/ordinary/derivation.rs:188; cli/src/interact/ordinary.rs:625`: The list of LK and LJ rules and their ASCII spellings are crate-private, so the command keeps its own copies of both tables, which drift independently of the parser that reads them.
- F-4a-18 (confirmed, refactor, small), `core/src/ordinary/mod.rs:212; core/src/ordinary/negative.rs:145`: Four enums of this unit repeat the same `FromStr` body (find by name in `ALL`, else `UnknownName`), and `Method`'s feature list is restated with its `cfg`s in six places.
- F-4b-10 (confirmed, refactor, small), `core/src/ordinary/sat/classical.rs:253; core/src/ordinary/sat/classical.rs:376`: `classical::decide` and `falsify` repeat the same steps from the encoding to the solver's answer: a variable per formula, every clause added with a poll each `POLL` clauses, one call, the statistics.
- F-4b-11 (confirmed, refactor, small), `core/src/ordinary/normal.rs:2592; core/src/ordinary/normal.rs:2710; core/src/ordinary/normal.rs:2625; core/src/ordinary/normal.rs:2732`: `normal.rs` writes two of its helpers twice: the fresh name prefix (`fresh_prefix` and `prefix`) and the loop that moves the formulas right of `⊢` left negated, each a `Negate` step.
- F-4b-16 (confirmed, refactor, small), `core/src/ordinary/sat/mod.rs:498; core/src/ordinary/sat/mod.rs:458`: The two places that take LJT's derivation for the SAT method handle the same impossible evidences differently (one invents errors, one ignores them) and write the same defect message twice, and the release build makes a placeholder refusal of zeros that only a test path uses.
- F-4d-02 (confirmed, refactor, medium), `core/src/ordinary/structure.rs:510 and core/src/ordinary/kripke_structure.rs:927`: The finite structure and the Kripke model with growing domains each have their own evaluator of one semantics: the term walk, the atom lookup, the work tick, the up-front pass over the sequent's terms and the `check` wrapper are written twice, and the copies have already drifted on `=` (F-4d-01).
- F-4d-03 (confirmed, refactor, medium), `core/src/ordinary/models.rs:605 and core/src/ordinary/kripke_search.rs:363; core/src/ordinary/grounding.rs:1282 and core/src/ordinary/models.rs:726`: The three SAT encodings of first-order models (grounding, the finite model search, the Kripke model search) each write their own copy of one writer: the bounded `tick`, `junction`, `over_tuples`, `Denotes`, the symmetry formulas, the function-table read-off, and the memoised walk of the normal form under a chain of bindings.
- F-4d-15 (confirmed, refactor, small), `core/src/modal/search.rs:735`: The modal search iterates the set bits of a world's truth and falsity masked by its box, diamond or atom bits in four hand-written loops, and tests "has a `□` true or `◇` false of its own" in three.

### C11. Repeats in lambda-terms, rule files, the workspace and the wire forms

8 findings (8 confirmed, 0 candidates); kinds: refactor.

**What.** Untyped terms have a second parser for a subset of the typed syntax; naming code is written twice; a rule file's drawing and the typing derivation implement one tree layout with the same code; the mapping between ordinary logic's nodes and a rule file's connectives is written three times; how a connective is written and how it is read are two copies of the same defaults; the workspace rebuilds its index in three places; wire levels are bare numbers in four places.

**Change.** One function each.

**Net.** The tests of each module; the round trip of a rule file's text.

- F-5a-08 (confirmed, refactor, medium), `core/src/lambda/parse.rs:1328`: The untyped terms have a second parser beside the typed one, with its own stack of open constructs, context reader, `λ` reader and application run, for a subset of the same syntax.
- F-5a-12 (confirmed, refactor, small), `core/src/lambda/reduce.rs:766; core/src/lambda/print.rs:536`: Two pieces of naming code are each written twice: the fresh name with the least free index, and the read-back's letters for a new variable.
- F-5a-14 (confirmed, refactor, small), `core/src/lambda/draw.rs:619; core/src/rules/draw.rs:298`: The typing derivation and a rule file's drawing implement `Drawn`'s tree layout (`root`, `premises` over an end list, `times`) with the same code.
- F-5b-08 (confirmed, refactor, small), `core/src/rules/ordinary.rs:24; core/src/rules/natural.rs:858; core/src/rules/natural.rs:1084`: The mapping between ordinary logic's formula nodes and a rule file's connective names is written three times in `rules`, twice in the same direction.
- F-5b-09 (confirmed, refactor, small), `core/src/rules/print.rs:79; core/src/rules/text.rs:196`: How a connective without a declared fixity or precedence is written and how it is read are computed by two copies of the same defaults, and the round trip "what `show` writes reads back" rests on them staying equal.
- F-5b-17 (confirmed, refactor, small), `core/src/workspace/mod.rs:1508; core/src/workspace/form.rs:568; core/src/workspace/form.rs:760`: The record index is rebuilt from the records by the same loop in three places.
- F-5b-18 (confirmed, refactor, small), `core/src/rules/draw.rs:90; core/src/rules/fitch.rs:190`: The tree drawing and the Fitch proof open with the same checks of zones and root, written twice.
- F-6a-18 (confirmed, refactor, small), `core/src/wire.rs:620`: The wire levels are named constants scattered out of order across `wire.rs`, while levels 2 and 3 are bare numbers at four places, and nothing ties `LEVEL` to the highest level a form is written at.

### C12. Repeats in the command

9 findings (9 confirmed, 0 candidates); kinds: refactor, search change.

**What.** The decision of one sequent is written in `prove` and again in the batch; the command decides first-order ordinary sequents by its own copy of the library's default; the common flags are declared and applied per command; `check`'s JSON verdict is a ten-field literal written 16 times; the mapping from a refusal's setting to the flag that raises it is written about sixteen times with different coverage; a derivation in a rule file has two writers that support different formats; drawing an SVG and rendering it is written five times.

**Change.** One function each, the decision first: `prove` and the batch call one answer type, which `grade` reads instead of parsing text. F-7b-05 is a search change: the library's default replaces the command's copy, and the verdicts of the ILTP rows show whether anything moves.

**Net.** The behaviour lock and the examples; for F-7b-05 the benchmark's verdicts on the first-order ILTP problems.

- F-7a-12 (confirmed, refactor, large), `cli/src/prove.rs:296`: The decision of one sequent is written twice, in `proved` (382 lines) and in the batch's `Shared::decide`, and both interleave the suites through `Option`s rather than choosing the suite once.
- F-7a-13 (confirmed, refactor, medium), `cli/src/settings.rs:336`: The common flags are declared and applied per command: the time limit 6 times, the memory limit 8 times, the work limit 5 times and `--settings` 11 times, each with its own doc and its own `apply` block.
- F-7a-14 (confirmed, refactor, medium), `cli/src/check.rs:1236`: `check`'s JSON verdict is built by a ten-field struct literal 16 times, with the status-to-word match 7 times, and it names its suite by which of four optional fields is set.
- F-7b-05 (confirmed, search change, large), `cli/src/ordinary.rs:358`: The command decides first-order ordinary sequents by its own copy of the library's default: a grounding branch copied from `by_grounding`, plus a connection-prover and model-search race with time shares. The library's `ordinary::decide_within` (which the bindings use) follows a different policy, so the command and the library answer one sequent by different paths.
- F-7b-07 (confirmed, refactor, small), `cli/src/calculus.rs:155`: A derivation in a calculus given as data has two writers in one file, `check_rules` with `drawing` and `shown`. They support different formats: `check` refuses PNG and PDF for the same kind of derivation that `prove --system` renders, and only `check` has the Fitch layout.
- F-7b-09 (confirmed, refactor, medium), `cli/src/tableau.rs:135`: The mapping from a refusal's settings key to the flag that raises it is written about sixteen times, with different spellings and coverage, and one copy sends the user to a settings file.
- F-7b-16 (confirmed, refactor, small), `cli/src/show.rs:673`: "Draw an SVG, write it or render it to PNG or PDF, and say why it is left out" is written five times, and the copies say different things when a stop ends the render.
- F-7b-17 (confirmed, refactor, small), `cli/src/eliminate.rs:183`: `eliminate.rs` repeats itself three times: a closure equal to its own `sequent_of`, the invalid-or-unknown report written again in `unchecked`, and the MLL/MELL dispatch of `nets::cut::eliminate` written twice.
- F-7b-21 (confirmed, refactor, small), `cli/src/eliminate.rs:689`: The helper that writes a count with its noun is defined three times, plus a fourth variant, and `grade` borrows `eliminate`'s copy.

## D. Performance

Measured hot spots first, then what reading suggests. A candidate is not a finding of fact: the refactor step measures before and after on the instrument named and drops the change if nothing moves. Most candidates have no instrument yet, which is itself the first item. (57 findings in 5 themes.)

### D1. Instruments first: journeys for what nothing measures

4 findings (3 confirmed, 1 candidates); kinds: build, refactor.

**What.** The ratchet and DHAT run the linear search, the readers, the linear checker and the exports. Nothing runs ordinary logic's own engines (the SAT method, LJT, the connection provers, resolution), a first-order search of either suite, modal logic, the model searches, lambda-terms, a session, cut elimination, a refutation's check, the inverse method, a rule file's checker or the workspace. Where a journey exists, its input generation weighs in its table: 51 % of one journey's blocks are the generator's.

**Change.** One journey per engine or checker named, each with a ceiling, and the generators' share taken out of the measured part. This is the first step of the performance work: without it the candidates of D4 cannot be judged.

**Net.** `bench/ceilings.csv` gains rows; a change of the ceilings is a commit of its own.

- F-4b-03 (confirmed, build, medium), `bench/src/journeys.rs:137`: No journey of the ratchet or of DHAT runs any engine of this unit: `ordinary-pigeons` decides through the embedding, so LJT, the SAT method (the default of intuitionistic and minimal logic), the classical SAT method with its DRUP check, the normal forms, interpolation and the tableau have no instrument.
- F-5a-13 (candidate, build, medium), `core/src/lambda/print.rs:775`: No journey of the ratchet or of DHAT runs the lambda suite, so its reading candidates (allocation per node printed, quadratic list merging, arena clones per term listed) cannot be measured.
- F-2c-05 (confirmed, build, small), `core/src/linear/search/inverse/mod.rs:88`: No instrument measures the inverse method: it runs in no journey of the ratchet or DHAT and in no row of the focused target set, although the default dispatch gives it a turn on two rows. A slowdown or a change in what it visits shows nowhere.
- F-2c-12 (confirmed, refactor, small), `core/src/linear/families.rs:804; core/src/linear/families.rs:954`: The family generators write each formula piece as its own `String` and join them, and that input generation is 40 % of the `read-text` journey's DHAT blocks and 51 % of `search-additive-14`'s, so those tables measure the input rather than the library.

### D2. Hot spots that DHAT or a run already shows

7 findings (4 confirmed, 3 candidates); kinds: refactor.

**What.** In the SVG layout a fresh `String` per piece of text is 87 % of the journey's heap blocks. In the linear checker a boxed state and a hash table per leaf are 98 % of `check-qbf-20-2`'s blocks. The TPTP clause reader collects every line into a `Vec<char>` (57 % of `read-lltp`'s blocks, a third of `read-tptp`'s). `mist::merged` copies where `dedup_by` works in place (16 % of `read-spec`). An atom's name is stored twice per atom (43 % of `read-tptp`). The ordinary checker allocates several vectors per inference. Every step of a System F reduction copies all earlier steps' types, so a reduct's arena grows with the square of the steps.

**Change.** Each is a local change with its journey as the measure: blocks under DHAT and instructions under callgrind, before and after.

**Net.** The ratchet's ceilings (lowered when a count falls) and DHAT; the checker's change touches a checker's data layout, not what it accepts (Kani's harnesses, the panel).

- F-6b-01 (confirmed, refactor, small), `core/src/export/svg/mod.rs:1120; core/src/export/svg/tree.rs:110`: The SVG layout allocates a fresh String for every piece of text it lays out, even where only the width is wanted, and these allocations are 87 % of the render-svg journey's heap blocks.
- F-1b-02 (candidate, refactor, medium), `core/src/linear/proofs/check/pass.rs:764`: Almost every heap allocation of a check is the checker making a leaf's sequent. Each leaf (`ax`, `1`, `⊤`) gets a boxed `State` and then a hash table for its linear zone, so a proof of n leaves costs about 2n allocations.
- F-2c-02 (confirmed, refactor, small), `core/src/linear/lltp.rs:300`: `lltp::clauses`, which both the LLTP reader and ordinary logic's TPTP reader call, collects every line into a fresh `Vec<char>`, and these allocations are 57 % of the `read-lltp` journey's blocks and 32 % of `read-tptp`'s.
- F-2c-07 (confirmed, refactor, small), `core/src/linear/mist.rs:419`: `mist::merged` copies its sorted counts into a second vector to merge repeats, where `Vec::dedup_by` does it in place. Its allocations are 16 % of the `read-spec` journey's blocks.
- F-4a-06 (candidate, refactor, small), `core/src/ordinary/mod.rs:935`: `Formulas::atom` stores every atom name twice, in `atoms` and as the key of `names`, two allocations per atom, which are 43 % of the `read-tptp` journey's blocks.
- F-4a-05 (candidate, refactor, medium), `core/src/ordinary/derivation.rs:783`: The ordinary checker and read-back allocate several vectors per inference (a `Vec<&Inference>` of the premises, copies of both sides of the conclusion, one more per premise), and the `ordinary-pigeons` journey makes 2 226 allocations per million instructions, the most of every journey.
- F-5a-01 (confirmed, refactor, small), `core/src/lambda/reduce.rs:627`: Every step of a System F reduction copies the term with all the types and type variables of every earlier step, so a reduct carries a type arena that grows with the square of the steps.

### D3. Long bodies generic over the stop, compiled once per caller

9 findings (5 confirmed, 4 candidates); kinds: build, refactor.

**What.** The crate's rule is a `&mut dyn FnMut(Progress) -> bool` body behind a one-line generic shim. Bodies in the nets, the interpolants, the modal tableau, the exports, cut elimination and the rule files' checker are generic throughout, and the bindings' five `never` stops compile 51 library functions again. `Pass::rule` of the checker is compiled twelve times. It costs compile time and the web module's bytes, not run time.

**Change.** The shim at each entry. Measure with `cargo llvm-lines` and the module's bytes after wasm-opt.

**Net.** `inventories/llvm-lines.txt`, the budget check, the ratchet (a `dyn` call in a loop must not show).

- F-3-10 (candidate, refactor, small), `core/src/linear/nets/essential.rs:131`: Several long net entry points keep their whole body generic over the caller's stop closure, against the crate's rule that a long body takes `&mut dyn FnMut(Progress) -> bool` behind a one-line shim.
- F-4b-02 (confirmed, refactor, small), `core/src/ordinary/interpolant.rs:784; core/src/ordinary/interpolant.rs:385; core/src/ordinary/normal.rs:1074`: Maehara's method, the interpolant's check and `NormalForm::check` are generic over the stop, so each is compiled twice, the second copy for the closure of the `_default` helper, against the crate's rule that a long body takes its stop as `&mut dyn FnMut`.
- F-4d-04 (confirmed, refactor, small), `core/src/modal/decide.rs:255`: `modal::decide_within` passes the caller's stop closure generically through the whole tableau and the derivation's emission, against the crate's rule that a long body takes its stop as `&mut dyn`, so the search is compiled once per caller.
- F-6b-02 (confirmed, refactor, small), `core/src/export/svg/kripke.rs:138; core/src/export/svg/mod.rs:1863, 2042, 2140`: `kripke::draw` takes its stop as `impl FnMut() -> bool`, and its three callers each hand-write their own polling closure, so the drawing is compiled three times where `limits::counting` over a `&mut dyn` stop would compile it once.
- F-6b-06 (confirmed, refactor, small), `core/src/export/latex.rs:660; core/src/export/typst.rs:740`: `latex::write` and `typst::write` pass the caller's own stop closure down into their generic tree writers, so each caller compiles its own copy of every tree emitter, where `svg::write` turns the stop into `&mut dyn` first.
- F-1c-12 (candidate, refactor, small), `core/src/linear/proofs/cut/mod.rs:426`: `cut::eliminate` and `cut::step` are long bodies generic over the caller's stop (and `eliminate` over `each`), so each caller compiles its own copy, against the crate's rule of a one-line shim over a `&mut dyn` body.
- F-5b-12 (candidate, refactor, small), `core/src/rules/check.rs:221; core/src/rules/search.rs:126; core/src/workspace/mod.rs:1003; core/src/workspace/mod.rs:1303`: Four long bodies of this unit are generic over the stop, against the crate's rule that a long body takes `&mut dyn FnMut` behind a one-line shim, so each is compiled once per caller.
- F-8-03 (confirmed, build, small), `web/src/calls.rs:32; web/src/ordinary.rs:30; web/src/rules.rs:33; web/src/modal.rs:25; web/src/lambda.rs:27`: The bindings define five `never` stops, each a distinct fn-item type, so 51 library functions generic over the stop are compiled again for the bindings beside the copy the protocol's `&mut dyn FnMut` stop makes: 164 030 raw bytes in the bindings-stop copies.
- F-1b-03 (candidate, refactor, medium), `core/src/linear/proofs/check/pass.rs:176`: The derivation's record and the size estimate choose an `Instancing` (`Ground` or `Framed`), and then `examine` chooses again. This compiles the whole pass for observer/instancing pairs that never run together. At first order it also runs the frame pass twice for each pass of the checker.

### D4. SipHash tables beside the crate's own hasher

5 findings (4 confirmed, 1 candidates); kinds: build, refactor.

**What.** About thirty library sites build `std::collections` tables with SipHash beside the crate's `hash` module (foldhash), most of them cold, and nothing keeps new ones out.

**Change.** The crate's tables everywhere, and a `clippy.toml` with `disallowed-types` so that the lint holds it.

**Net.** The ratchet; iteration order must not reach an output (the behaviour lock shows it if it does).

- F-1a-12 (candidate, refactor, medium), `core/src/linear/sequents/notation.rs:236`: The crate's fast hasher has std's SipHash tables beside it in ten places of library code. No `clippy.toml` with `disallowed-types` keeps out new ones.
- F-1b-13 (confirmed, build, small), `core/src/linear/proofs/size.rs:30`: The crate keeps a fast hasher for its tables (`crate::hash`, foldhash), but 13 places still build `std::collections` tables with SipHash, and nothing stops a new one.
- F-4a-19 (confirmed, refactor, small), `core/src/ordinary/instance.rs:771; core/src/ordinary/model.rs:299`: Two sites of this unit use std's SipHash tables beside the crate's foldhash `hash` module; neither is on a journey's hot path.
- F-4e-14 (confirmed, refactor, small), `core/src/unify.rs:15`: The unifier the sessions share uses the standard library's `HashSet` (SipHash) for its two sets of pairs and terms seen, beside the crate's `hash` module (foldhash), which the ordinary session uses for the same kind of set.
- F-4f-12 (confirmed, refactor, small), `core/src/linear/proofs/interactive/quantified.rs:959; core/src/linear/proofs/interactive/quantified.rs:984`: The linear session's first-order part uses the standard library's `HashSet` (SipHash) in two walks beside the crate's `hash` module, which the same file imports for its maps.

### D5. Candidates from reading, each waiting for a measurement

32 findings (2 confirmed, 30 candidates); kinds: refactor, search change.

**What.** Allocations in inner loops, clones of whole values to escape a borrow, quadratic scans, sorts whose key walks a term on every comparison, `#[inline(always)]` without a recorded measurement. Thirty-two findings, most in code no journey runs.

**Change.** After D1: measure each on its journey, keep the changes that move a count, record the others as tried in the unit's rules file so that they are not raised again.

**Net.** The journey of each; the target sets' counters where a search's loop is touched (F-1c-13 and F-2a-08 would be search changes).

- F-1a-06 (candidate, refactor, small), `core/src/linear/occurrences/mod.rs:738`: Building a forest allocates a `Vec` for every compound occurrence, only to push its children in reverse. A forest of the LLTP library's largest problem has 27.8 million occurrences, and every search and every proof read from JSON builds one.
- F-1b-15 (candidate, refactor, small), `core/src/linear/proofs/derivation/view.rs:343`: Every inference of a derivation (88 bytes) allocates a vector for its premises, though a rule has two at most. It also stores its principal position as an `Option<usize>` of 16 bytes.
- F-1c-10 (candidate, refactor, small), `core/src/linear/search/saturated.rs:640`: The saturated set's check clones each premise's zones for every choice of premises and partners it evaluates, and allocates two more lists per sequent derived, in its innermost loop.
- F-1c-13 (candidate, search change, small), `core/src/linear/search/classical.rs:132`: The classical refuter evaluates every term the goal reaches at every node of its backtracking, with no propagation, so its work bound of 2²⁶ term evaluations allows about 2²⁶ / terms assignments before a search that ended `Unknown` stays `Unknown`.
- F-2a-08 (candidate, refactor, medium), `core/src/linear/search/mod.rs:591; core/src/linear/search/mod.rs:771`: `race`, the command's default search, runs the set-up passes twice before any engine polls (three times once the pool starts): `engine_for` computes the fragment, the intuitionistic reading and the dispatch (with a Horn program's reading), and `answer_goal` computes them again.
- F-2b-09 (candidate, refactor, small), `core/src/linear/search/focus/framed.rs:681`: The first-order zone's term walks allocate a new vector and a new hash set on every call, and `passes` runs one per recorded pattern after every unification while a rule's first premise is searched.
- F-2b-10 (candidate, refactor, small), `core/src/linear/search/focus/counts.rs:729`: Nine `#[inline(always)]` sites of the unit rest on a stated reason but no recorded measurement, where core-focus.md records one only for the five on the recursion cycle.
- F-2b-16 (candidate, refactor, small), `core/src/linear/search/focus/mod.rs:1759`: Each stable sequent's focus candidates are sorted with a stable sort whose key, `focus_class`, walks the tensor's children and their forcing on every comparison, although the keys are unique and an unstable sort gives the same order.
- F-2c-08 (candidate, refactor, small), `core/src/linear/search/horn/mod.rs:699; core/src/linear/search/horn/equation.rs:328`: Reading a Horn program clones both arc lists of every clause under `?` only to look them up in a map, and the simplex's set-up keeps an uncharged clone of every distinct effect in a set.
- F-2c-09 (candidate, refactor, small), `core/src/linear/search/inverse/index.rs:77`: Every subsumption query of the inverse method's tries allocates a fresh stack, and `within` allocates another for each node it visits, while the Horn engine's trie of the same kind keeps one stack for all its queries.
- F-2c-10 (candidate, refactor, medium), `core/src/linear/search/inverse/mod.rs:538; core/src/linear/search/inverse/mod.rs:865; core/src/linear/search/inverse/mod.rs:468`: The inverse method's saturation allocates on every derived sequent and every premise choice: it clones a hole list in `fill`, builds a vector in `fits`, and collects and sorts a copy of the zone in `coherent`.
- F-4b-09 (candidate, refactor, small), `core/src/ordinary/tableau.rs:620`: Writing a tableau as text finds each node's parent by scanning every earlier node's children, so printing a tableau of n nodes costs about n²/2 scans.
- F-4b-12 (candidate, refactor, small), `core/src/ordinary/interpolant.rs:262`: `interpolate` clones the whole derivation out of the outcome and then keeps the outcome too, so the derivation is held twice where a borrow would do.
- F-4b-13 (candidate, refactor, small), `core/src/ordinary/ljt.rs:518`: Ordering LJT's `→→L` alternatives allocates a fresh hash set in every call of `heads`, which `levels` calls twice per hypothesis per round for up to one round per hypothesis, so one choice point can make thousands of allocations.
- F-4b-14 (candidate, refactor, small), `core/src/ordinary/sat/drup.rs:342`: The DRUP checker keeps every clause's literals twice, once in the clause and once as the key of the index that only deletions use, and charges both, so the check reaches the memory bound at about half the proof it could hold.
- F-4c-06 (candidate, refactor, small), `core/src/ordinary/connection/prove.rs:532, :563, :815 and core/src/ordinary/connection/intuitionistic/prove.rs:582, :613, :645, :862`: The connection searches allocate on every decomposition, every extension-clause test and (intuitionistic) every regularity or lemma comparison, where a borrowed slice or a reused buffer would do.
- F-4c-07 (candidate, refactor, small), `core/src/ordinary/resolution/first_order.rs:1692`: The first-order given-clause search queues a resolution step for every pair of opposite-sign literals whatever their predicates, and each attempt clones both parent clauses and interns renamed copies of the second's atoms before `pairs` finds the predicates differ.
- F-4c-14 (confirmed, refactor, small), `core/src/ordinary/resolution/refutation.rs:248`: The propositional refutation checker clones both parent clauses of every resolvent only to borrow them.
- F-4d-05 (candidate, refactor, medium), `core/src/ordinary/grounding.rs:1394; core/src/ordinary/models.rs:541; core/src/ordinary/kripke_search.rs:196`: The three encoders name every atom they make `g_N`, `m_N` or `k_N` and parse the number back out of the name to find its entry, though the atom's index in the arena is that number.
- F-4d-06 (candidate, refactor, small), `core/src/ordinary/models.rs:555; core/src/ordinary/kripke_search.rs:247; core/src/ordinary/grounding.rs:1365`: The encoders' memo tables are keyed by `(u32, Vec<u32>)`, so every lookup, a hit included, allocates the key with `to_vec` or `clone`, and an insert allocates it twice more.
- F-4d-11 (candidate, refactor, small), `core/src/modal/rocq.rs:401`: The certificate of a valid modal outcome checks its derivation twice and builds the system's calculus twice: once in `Outcome::check`, then again in `labelled`.
- F-4d-12 (candidate, refactor, small), `core/src/modal/search.rs:1163`: Each saturation of a modal branch rebuilds, for every pending formula of two premises, both premises' member vectors only to test whether one already holds, and again to pick the split.
- F-4e-09 (candidate, refactor, medium), `core/src/rules/interactive.rs:868`: Each choice `rules::Interactive::candidates` tries clones the whole session, `plan`, `give` and `fresh` each clone the whole `Calculus` again to escape a borrow, and every trial step rebuilds the session's whole derivation (cloning its `Terms`) to check one node, so one call and one session grow with the session's size times the choices.
- F-4e-16 (candidate, refactor, small), `core/src/rules/interactive.rs:2209`: The reader of a rule-file session scans every scope once per scope and once per history entry, so a document with many scopes, which a session of many discharging steps writes, is read in time quadratic in its scopes.
- F-4f-10 (candidate, refactor, medium), `core/src/linear/proofs/interactive/quantified.rs:1006; core/src/linear/proofs/interactive/witness.rs:206`: In a session with quantifiers every step remakes the whole shown table, and every step with a given witness or a cut builds a parent list of every inference, so a session of n such steps costs time quadratic in n.
- F-5b-13 (candidate, refactor, small), `core/src/rules/check.rs:544`: The checker builds a message string for every member it matches, whether or not the match fails, and allocates several vectors per node and zone.
- F-5b-14 (candidate, refactor, medium), `core/src/workspace/mod.rs:700`: `Workspace::state` of one node computes every node's state, and `states` clones each recipe's parameters and settings to build a key, so `run` and the front ends' loops are quadratic in the nodes.
- F-5b-15 (candidate, refactor, small), `core/src/workspace/mod.rs:572`: Interning a value copies its whole text to look it up, keeps every text twice, and sums all values on each insert, so the bound on bytes covers half of what is held.
- F-5b-16 (candidate, refactor, small), `core/src/rules/search.rs:135`: `Interactive::hint` clones the whole session, its calculus included, up to three times per call, the same pattern F-4e-09 found in `candidates`.
- F-6a-16 (candidate, refactor, medium), `core/src/serialize/proofs.rs:260`: Writing a value builds an owned copy of it first: a proof's nodes, its sequent's arena and names, a Kripke model's worlds, a structure and a calculus are cloned into proxies only to be written, where `serialize/lambda.rs` writes through borrowed fields.
- F-6b-07 (confirmed, refactor, small), `core/src/export/latex.rs:747; core/src/export/typst.rs:1064; core/src/export/svg/tree.rs:249`: Each tree writer allocates a small string per inference for a number it could write into its buffer.
- F-6b-12 (candidate, refactor, medium), `core/src/export/rocq/deduxit.rs:590`: The term of a certificate for deduxit's library writes every node's subformulas out in full, so its text grows with the formulas' depth times the derivation's size (F-9-18 from the exporter's side).

## E. Structure: files and modules that hold more than one thing

Moves and divisions with no change of behaviour. Their gain is for a reader of the code; they come after the API moves, which decide where things belong. (26 findings in 3 themes.)

### E1. Files that hold several things, with their seams

10 findings (10 confirmed, 0 candidates); kinds: refactor.

**What.** `linear/search/mod.rs` (5 552 lines: the front door, the engine interface, the catalogue of engines, the options, the outcome types, and a third of it tests), `ordinary/interactive.rs` (5 900 lines where the linear session is nine files), `ordinary/mod.rs`, `ordinary/normal.rs`, `resolution/first_order.rs`, `herbrand.rs`, `focus/framed.rs`, `serialize/ordinary.rs`, and functions of 150 to 1 390 lines (`svg::mell::draw` is the crate's largest by LLVM lines). Each finding names the seams with line ranges.

**Change.** Divide along the seams named, moving code without changing it. One file per step, after the API moves of group B.

**Net.** No behaviour changes: the tests, the target sets' counters, the snapshots. `the_documented_dispatch_is_the_dispatch` reads `mod.rs` by path and moves with `Engine`.

- F-arch-19 (confirmed, refactor, medium), `core/src/linear/search/mod.rs:1362; core/src/linear/search/mod.rs:1976; core/src/linear/search/mod.rs:2855; core/src/linear/search/mod.rs:3389`: `linear/search/mod.rs` holds five things in 5 552 lines, each with a clear edge.
- F-arch-20 (confirmed, refactor, medium), `core/src/ordinary/mod.rs:709; core/src/ordinary/interactive.rs:1069`: `ordinary/mod.rs` is the decision's vocabulary and the formula arena in one file, and `ordinary/interactive.rs` is one file of 5 258 lines where the linear session, its model, is nine.
- F-2b-06 (confirmed, refactor, medium), `core/src/linear/search/focus/framed.rs:888`: framed.rs (2 119 lines) holds four separable parts: the zone's variables, unification and answers; the keys of first-order sequents; the `Zone` impl; and the closing of a proof.
- F-2b-13 (confirmed, refactor, small), `core/src/linear/search/focus/counts.rs:356`: `Counts::new_until` is one function of 253 lines that makes four passes in a row, each with its own state.
- F-3-11 (confirmed, refactor, medium), `core/src/linear/nets/cut/mell.rs:209`: Seven functions of the unit run past 100 lines, each with visible seams. The longest are the MELL step of cut elimination (154 lines, one arm per case) and the MELL sequentialization's stage (158 lines).
- F-4b-08 (confirmed, refactor, medium), `core/src/ordinary/normal.rs:1`: `normal.rs` (3 688 lines) holds five things along clear seams: the public vocabulary and its check, the propositional rewriting, the first-order rewriting with Skolem and Herbrand terms, the clausal encodings, and the form test.
- F-4c-11 (confirmed, refactor, medium), `core/src/ordinary/resolution/first_order.rs:173 and core/src/ordinary/herbrand.rs:1063`: `resolution/first_order.rs` (2 563 lines) and `herbrand.rs` (2 211) each hold several things with clear seams.
- F-4e-05 (confirmed, refactor, medium), `core/src/ordinary/interactive.rs:1069`: `ordinary/interactive.rs` (5 900 lines) holds nine separable parts, among them a 600-line session reader and 1 900 lines of tests, where the linear session it was modelled on is a directory of nine files.
- F-6a-19 (confirmed, refactor, small), `core/src/serialize/ordinary.rs:1600`: `serialize/ordinary.rs` (2 065 lines) holds three things a reader keeps apart: the forms read back (sequent, derivation, outcome, session), the tables and tableaux, and the written-only transforms (comparison, interpolation, review, normal form, Herbrand, first-order resolution), and it rebuilds the sequent form from its three parts at eleven places.
- F-6b-03 (confirmed, refactor, medium), `core/src/export/svg/mell.rs:363`: `mell::draw` is one function of about 1 390 lines, the largest function of the crate by LLVM lines, holding a dozen separate layout passes that share state only through local vectors.

### E2. Modules in the wrong place

5 findings (5 confirmed, 0 candidates); kinds: API break, refactor.

**What.** `rules`, which belongs to no suite, holds 2 213 lines of ordinary logic, and the two modules name each other; the count invariants and the atom bias serve four engines and the session from inside the focused engine; `occurrences` imports `proofs` to renumber a proof's instance table; the checks of refutations, a trust boundary, are spread over four files beside the engines; ordinary logic's countermodels are seven flat files with near names.

**Change.** Move each to where its users are: natural deduction and the tableau to `ordinary`, the counts and the bias beside the engines, the refutations' checks into a module of their own, the countermodels into a directory.

**Net.** No behaviour changes; the target sets' counters for the counts' move.

- F-arch-09 (confirmed, API break, medium), `core/src/rules/ordinary.rs:131; core/src/rules/search.rs:15; core/src/ordinary/written/mod.rs:32`: The module of calculi given as data, "which belongs to no suite", holds 2 213 lines of ordinary logic, and the two modules name each other.
- F-2b-03 (confirmed, refactor, medium), `core/src/linear/search/focus/mod.rs:60`: The count invariants and the atom bias are shared machinery of four engines and the interactive session, yet they live as modules of the focused engine, and the counts module reaches back into the engine for `Switches`.
- F-1a-09 (confirmed, refactor, medium), `core/src/linear/occurrences/instances.rs:20`: `occurrences/instances.rs` holds the proofs' canonicalisation (`canonical` and `Canonical`, about 430 lines). That code renumbers `proofs::Node`s, so the data module `occurrences` names `proofs`, while `proofs` names `occurrences`.
- F-1c-06 (confirmed, refactor, medium), `core/src/linear/search/refutation.rs:1245`: The checks of refutations, the crate's second trust boundary in the linear suite, are spread over four files of `search/` beside the engines, one of them also holding the refutation types and 450 lines of phrases.
- F-4d-10 (confirmed, refactor, medium), `core/src/ordinary/mod.rs:89`: Countermodels of ordinary logic are seven flat files of `ordinary/` with near names (`model.rs`, `models.rs`, `structure.rs`, `kripke_structure.rs`, `kripke_search.rs`, `rocq_models.rs`, beside `grounding.rs` that feeds them), among some thirty modules, so a reader cannot tell the data and checks from the searches and the certificates.

### E3. Small matters of idiom

11 findings (11 confirmed, 0 candidates); kinds: refactor.

**What.** A wildcard arm where the module's siblings deny one; the crate's only `#[allow]` attributes without a reason; a sentinel `u32::MAX` five callers test by hand; nine items each behind their own `cfg` where a module would hold them; production code after a test module; an `if` as the condition of an `if`; `is_some_and` followed by an unwrap with a dead default; six sites of a lint the workspace allows only because it fires; a hand-written const parser where the standard library has one.

**Change.** The idiomatic form at each place; a lint whose last site goes is turned on in the same commit.

**Net.** Clippy and the tests.

- F-1a-15 (confirmed, refactor, small), `core/src/linear/parse/mod.rs:1005`: The parser matches `Term` with a wildcard arm. The parse module is the one module of the calculus's syntax that does not deny `wildcard_enum_match_arm`, so a new connective would be passed by unchanged in the ordered dualisation without a compile error.
- F-2b-04 (confirmed, refactor, small), `core/src/linear/search/focus/split.rs:606; core/src/linear/search/focus/split.rs:660; core/src/linear/search/focus/mod.rs:1698; core/src/linear/search/focus/scratch.rs:218`: The core crate's only four `#[allow(` attributes are in this unit, without a reason, where the crate's rule is `#[expect(lint, reason = …)]` at the site, and the same file uses that form for the same lint.
- F-2b-08 (confirmed, refactor, small), `core/src/linear/search/focus/schedule.rs:324`: The threaded schedule's nine items each carry their own `#[cfg(feature = "parallel")]`, placed before the doc comment, where one module behind one attribute would hold them.
- F-2b-11 (confirmed, refactor, small), `core/src/linear/search/focus/choices.rs:99`: A choice point that must fail at once is signalled by `start == u32::MAX`, which five callers test by hand, and a Mix's yes or no is logged as a `u32`.
- F-3-14 (confirmed, refactor, small), `core/src/linear/nets/mod.rs:2171`: In `nets/mod.rs` a production `impl` comes after the test module. The test module, already behind `cfg(all(test, feature = "parse"))`, repeats `#[cfg(feature = "parse")]` on 11 of its items, and `sequentialize.rs`' test module does the same on 4.
- F-3-15 (confirmed, refactor, small), `core/src/linear/search/net.rs:274`: `counts_admit` uses an `if` expression as the condition of another `if`, which reads as a typo.
- F-4c-13 (confirmed, refactor, small), `core/src/ordinary/resolution/refutation.rs:228; saturate.rs:103, :115; dpll.rs:138`: The propositional resolution code tests an `Option` with `is_some_and` and then unwraps the same option with a dead `u64::MAX` default, where a `let` chain binds it once, as `first_order.rs:1722` already does.
- F-4c-15 (confirmed, refactor, small), `core/src/ordinary/connection/intuitionistic/prefix.rs:447; intuitionistic/prove.rs:434; intuitionistic/read_back.rs:1079; matrix.rs:259`: Leftovers obscure the code: a parameter kept and discarded, a field bound and discarded, a method that only forwards, and a four-tuple of `&mut` passed as a type alias.
- F-6b-15 (confirmed, refactor, small), `core/src/export/svg/net.rs:79; core/src/export/svg/kripke.rs:342; core/src/export/svg/tree.rs:147`: Six of the eight sites of `manual_midpoint`, a lint the workspace allows only because it fires today, are in the SVG layouts, beside code that already calls `i64::midpoint`.
- F-7b-20 (confirmed, refactor, small), `cli/src/show.rs:1513`: `net_into` silences a lint with `#[allow]` and no reason, where the workspace's convention (and every other site in the unit) is `#[expect(lint, reason = …)]`.
- F-8-15 (confirmed, refactor, small), `web/src/tab.rs:13`: The bindings parse the stack size from an environment string with a hand-written const decimal parser, where `usize::from_str_radix` has been usable in const context since Rust 1.82 (the workspace needs 1.92), and the size itself is written in both `build.rs` files.

## F. Text: docs, messages and help

Text a person reads, judged by `.claude/rules/writing.md` and the documentation items of the API guidelines. Each is small; together they are one pass per area, made after the API is fixed so that no text is written twice. (38 findings in 4 themes.)

### F1. Docs that give one suite the place of honour (D4)

6 findings (6 confirmed, 0 candidates); kinds: text.

**What.** The crate docs call intuitionistic logic the default and give four of their worked sections to linear logic; the exports' module docs start from a linear proof; the lambda module presents terms as proofs read back; the help's syntax section is linear logic's with ordinary logic appended, and modal syntax is nowhere; the manual's chapter on proving step by step describes the linear session alone; the Rocq library's opam description omits modal logic.

**Change.** One pass over the front pages in the page's order: ordinary logic, linear logic, modal logic, lambda-terms.

**Net.** The doc tests and the readme test, which runs the examples.

- F-arch-27 (confirmed, text, small), `core/src/lib.rs:18; core/src/lib.rs:111`: The crate docs call intuitionistic logic "the default" and give four of their worked sections to the linear suite, and say nowhere where the author of an engine starts.
- F-6a-05 (confirmed, text, small), `core/src/lib.rs:111`: The crate docs give linear logic every walk-through section and the shared notions: after one ordinary example, "A proof, its fragment and its JSON", "Intuitionistic input", "Limits and stops" and "The command's answers" are all linear, the "Errors" section lists only linear errors, and occurrences, a linear notion, are described as what "proofs, nets and errors" name.
- F-6b-09 (confirmed, text, small), `core/src/export/mod.rs:6; core/src/export/latex.rs:22; core/src/export/typst.rs:22`: The exports' module docs tell a reader to start from a linear proof, and the LaTeX and Typst modules show no example for any other suite, though every format takes a derivation of LK or LJ, a rule file's drawing and a typing derivation.
- F-5a-10 (confirmed, text, small), `core/src/lambda/mod.rs:4`: The module docs present lambda-terms as proofs read back, and lead with a linear proof search, while the module is a suite of its own that reads, checks, reduces, types and inhabits terms.
- F-7a-10 (confirmed, text, small), `cli/src/argument_parsing.rs:79`: The command's help and the manual's first how-to give linear logic the place of honour: the syntax shown under every command is linear logic's, ordinary logic's follows as "With --logic", modal logic's is nowhere, and `prove`'s description speaks of fragments and engines alone.
- F-7b-13 (confirmed, text, small), `docs/src/interactive.md:266`: The chapter "Prove step by step" and `interact --help` describe the linear session alone. The chapter's one paragraph on the ordinary session wrongly says it has no `invertible`, and the session in a calculus given as data is not named.

### F2. Messages that do not say what is wrong

12 findings (12 confirmed, 0 candidates); kinds: refactor, text.

**What.** A refusal that lists every cause it could have though the place that raises it knows which holds; a learner in minimal logic told about LJ; an axiom's refusal that says the opposite of the rule; refusals of a stored session and of a rule file that state a rule of the form instead of the fault, or leave out the name that is wrong; an index message that speaks of a proof's nodes for a workspace's; a `.spec` reader that names a token and never its line; a net's mode error that answers for four other cases; a session's advice in words the session does not have; the three sessions wording the same answers differently.

**Change.** Each message names the thing at fault and what to do, in the words of the calculus or command the user chose.

**Net.** The behaviour lock and the tests that pin messages; the page's hints, which show them.

- F-4f-04 (confirmed, text, small), `core/src/linear/proofs/interactive/mod.rs:265; core/src/linear/proofs/interactive/mod.rs:287`: Two refusals a learner meets list every cause they could have, though each place that raises them knows which one holds: a split position is "out of range, repeated, or the formula the rule acts on", and an eigenvariable is out of scope "either … or" made too late.
- F-4e-08 (confirmed, text, small), `core/src/ordinary/interactive.rs:238`: A learner proving in minimal logic who tries `WR`, `CR`, `⊥R` or `∨R` reads that "every goal of LJ has exactly one" formula right of ⊢, which names another calculus than the one they chose.
- F-4f-02 (confirmed, text, small), `core/src/linear/proofs/interactive/mod.rs:253; core/src/linear/proofs/interactive/expand.rs:660`: An axiom on a goal that is not exactly two formulas is refused with `NotAlone`, whose message tells a learner the axiom needs the formula "without other formulas in the goal", which is wrong for the axiom and contradicts itself on a goal of one formula.
- F-4f-11 (confirmed, text, small), `core/src/linear/proofs/interactive/replay.rs:124; core/src/linear/proofs/interactive/cut.rs:616`: Several refusals of a stored session state a rule of the form instead of what is wrong with the document, so the user reads "not a proof in progress: an open goal has no principal formula and no premise".
- F-4e-18 (confirmed, text, small), `core/src/rules/interactive.rs:2078`: Some refusals of the rule-file session's reader state a rule as if it were a fact, or use "is no", so the message does not say what is wrong with the document.
- F-5b-06 (confirmed, text, small), `core/src/rules/calculus.rs:882`: Several errors a rule-file author meets leave out the name that is wrong, and one names the wrong fault.
- F-5b-10 (confirmed, text, small), `core/src/workspace/mod.rs:1565`: A node number the workspace lacks is reported as a proof's node out of an empty list, which is false and points to the wrong thing.
- F-6a-08 (confirmed, text, small), `core/src/errors/mod.rs:1186`: The message of `Error::IndexOutOfBounds` for the spaces `Term` and `Node` is worded for a linear sequent's root and a linear proof's premise, but those spaces are used for first-order terms, rule-file terms, refutation clauses and workspace nodes, where the message is false.
- F-2c-03 (confirmed, refactor, small), `core/src/linear/mist.rs:434`: Every error of the `.spec` reader names a token but never its line, so a user whose file has thousands of rules cannot find the fault. The command passes the message on unchanged.
- F-3-04 (confirmed, text, small), `core/src/linear/nets/mod.rs:627`: `NetError::Mode` says proof nets exist "in linear mode only, with or without Mix". It also answers for a cyclic criterion with Mix, for cut links under the cyclic order, and for a structure with boxes under the cyclic order. The cyclic mode is linear, so in those three cases the message names the wrong reason.
- F-7b-11 (confirmed, text, small), `cli/src/show.rs:450`: A session's `proof` past the derivation limit is advised to use `--format json`, which is no word of the session (there it is `proof --json`), because `show.rs`'s refusal lines are written for `prove`.
- F-7b-19 (confirmed, text, small), `cli/src/interact/rules.rs:294`: The three sessions word the same answers differently: a goal that is no number, a goal with no rule, an unknown command, and a saved file of another kind.

### F3. Docs that say what the code does not, or leave out what it does

13 findings (13 confirmed, 0 candidates); kinds: text.

**What.** Doc comments that describe an earlier state of the code, name a parameter that is gone or a private trait, omit JSON keys the code writes, omit System F or the modal certificates, or list eleven of thirteen kinds; the reference table of long calls and their bounds without five modules; an `# Errors` section that contradicts its own function; lists of commands in the manual that have fallen behind; a help text that is false about `--output`.

**Change.** Correct each from the code; where a list repeats a table of the code, generate or test it as `Engine::NAMES` is tested.

**Net.** The doc tests; `cargo test -p deduxit-cli --test readme` for the manual's names.

- F-1b-14 (confirmed, text, small), `core/src/linear/proofs/style.rs:21`: The public module doc of `style`, and the doc of `Drawn`, say that the linear derivations and those of LK and LJ implement `Drawn`. Five types implement it. The module doc also names a crate-private trait to a reader who cannot use it.
- F-2a-14 (confirmed, text, small), `core/src/linear/search/mod.rs:3470`: `Reason`'s reference of its JSON form lists eleven of its thirteen kinds and misses a separator, while `Reason::NAMES` has all thirteen.
- F-4a-04 (confirmed, text, small), `core/src/ordinary/mod.rs:546`: Three doc comments of `ordinary/mod.rs` state what the code does not: `Method::decides` says the connection prover decides classical logic only and omits grounding and the model search, `mod table` carries the translation module's doc, and `Logic` and `Sequent` call themselves propositional.
- F-4e-15 (confirmed, text, small), `core/src/ordinary/interactive.rs:1242`: The doc comment of `widths` says no step adds a formula, but every quantifier step adds its instance to the arena, and the invertible run re-computes the widths for that reason.
- F-4f-05 (confirmed, text, small), `core/src/linear/proofs/interactive/mod.rs:617`: The public docs of a session's JSON form leave out two keys the session writes and reads: an inference's `cut` and a first-order session's `instances`, which are documented only on private types.
- F-5a-11 (confirmed, text, small), `core/src/lambda/parse.rs:1473`: `Term::parse`'s syntax reference omits System F (`ΛX. M`, `M [A]`, `∀X. A`) and still speaks of "the other two" calculi, and `Term::reduce` says only a `λ` application is a redex after naming the type redex.
- F-6a-10 (confirmed, text, small), `core/src/limits.rs:15`: The module's reference table of "the long calls, the fields of Limits that bound each, and how often each asks its stop" lists linear calls, some ordinary ones and the exports, and none of modal logic, lambda-terms, rule files, the protocol or the workspace, nor most of ordinary logic's engines.
- F-6b-08 (confirmed, text, small), `core/src/export/rocq/mod.rs:10`: The Rocq module's table of calls and the docs of `Options::prelude` leave out the modal suite's three certificates and their preludes, so a reader of the module's head finds no way to certify a modal proof or countermodel.
- F-6b-17 (confirmed, text, small), `core/src/export/latex.rs:453`: The doc comment of `begin` names a parameter `tree` that the function does not have.
- F-2c-04 (confirmed, text, small), `core/src/linear/lltp.rs:108`: The `# Errors` section of `lltp::read` says that a role other than `axiom`, `hypothesis` or `conjecture` is refused, while the same doc comment and the code take `lemma` and `theorem` too.
- F-7a-08 (confirmed, text, small), `cli/src/argument_parsing.rs:2672`: `--output`'s help says the file is "always in JSON", which is false.
- F-7a-09 (confirmed, text, small), `cli/src/argument_parsing.rs:2469`: The help of `--logic`, of its values and of `--input-format tptp` says "propositional" although `--logic` decides first-order sequents and TPTP problems with quantifiers.
- F-7a-15 (confirmed, text, small), `docs/src/settings.md:3`: Lists of commands in the crate docs, two doc comments, the manual's settings page and a comment have fallen behind the command.

### F4. Docs hard to read or without an example

7 findings (7 confirmed, 0 candidates); kinds: text.

**What.** A comment that needs knowledge of code generation to follow; an example that uses `unwrap` where its neighbours use `?`; the connection module, the public entry of the first-order prover, without an example, its key promise one sentence of five lines; a test comment that is two comments interleaved; `--help` texts without an example or a link, one of 538 lines, with headings in a different order per command. The inventory counts 963 of 985 inherent methods and 210 of 242 structs without an example: the proposal says which public types get one.

**Change.** Shorter sentences, an example that runs on every type a user starts from, the help's long form cut to what the manual does not say.

**Net.** The doc tests; the readme test, which checks every flag the manual names.

- F-1a-18 (confirmed, text, small), `core/src/linear/sequents/mod.rs:325`: The comment that explains why `push_cuts` calls `insert` and not `entry` cannot be understood without knowing the codegen it refers to.
- F-1a-19 (confirmed, text, small), `core/src/linear/sequents/fmt.rs:210`: The one example of the public walk, which `core.md` points every outside printer to, uses `unwrap` where every other example of the unit uses `?`.
- F-4c-12 (confirmed, text, small), `core/src/ordinary/connection/mod.rs:4`: The connection module, the public entry to the first-order prover of both classical and intuitionistic logic, has no example on the module or on `decide`, while `herbrand` and `resolution` lead with one.
- F-4c-17 (confirmed, text, small), `core/src/ordinary/connection/mod.rs:29`: The module's key promise, when "not valid" can be trusted, is one sentence of five lines with two nested parentheses.
- F-4e-17 (confirmed, text, small), `core/src/ordinary/interactive.rs:4912`: A test comment is two comments interleaved line by line, so neither reads.
- F-7a-06 (confirmed, text, medium), `cli/src/argument_parsing.rs:699`: No `--help` has an example or a link to the manual, and the long help of the most used flags is a wall: `prove --help` is 538 lines, `--method`'s help one paragraph of about 400 words.
- F-7a-16 (confirmed, text, small), `target/audit/help/check.txt:17`: The help headings come in a different order on each command, and `seq print` has none.

## G. The command

What the command's user meets beside the selector of the logic, which is in group B. (4 findings in 1 themes.)

### G1. Flags and what the command offers

4 findings (4 confirmed, 0 candidates); kinds: API break, build.

**What.** One flag name means two things (`--to`, `--calculus`) and one thing has three names; a bound has a flag on one command and sends the user to a settings file on two others; a session in a rule file writes JSON alone where the other two draw; `interact` on a terminal prints a bare prompt; a command of 34 subcommands and 80 flags ships no man page and no shell completions.

**Change.** One name per meaning; the common flags on every command that honours them; a first line in `interact` that shows the goal and names `help`; `clap_complete` and `clap_mangen` behind a hidden subcommand or at build time (`api-proposal.md`, section 6).

**Net.** The readme test, the behaviour lock; two dev-dependencies or two small dependencies of the command, within the budget of 180 packages.

- F-7a-05 (confirmed, API break, small), `cli/src/argument_parsing.rs:1791`: One flag name means two things under different subcommands, and one thing has three names.
- F-7a-11 (confirmed, build, medium), `cli/Cargo.toml:46`: The command has eleven subcommands and 80 flags but ships no man page and no shell completions.
- F-7b-12 (confirmed, API break, small), `cli/src/interact/mod.rs:216`: On a terminal, `interact` prints only the prompt `> `. It does not show the goal it starts from or say that `help` lists the commands and `quit` (or Ctrl-D) ends the session.
- F-7b-18 (confirmed, API break, small), `cli/src/resolve.rs:72`: The bound on a normal form's size has a flag on `seq normalize` (`--form-limit`), but `resolve` and `herbrand` refuse past the same bound and send the user to a settings file.

## H. The Rocq library

All of unit 9. The library's own repository, its own checks; a change of a name the exporter writes moves deduxit's exporter in the same pair of changes. (18 findings in 4 themes.)

### H1. The Rocq library: packaging, CI and the contract with the exporter

6 findings (6 confirmed, 0 candidates); kinds: build.

**What.** The release publishes the checksum of an archive GitHub generates; CI never installs the package from its opam file on each Rocq it admits; `rocq-stdlib` has no upper bound; the project file passes no warning flags and sets no goal selector; the README's examples are compiled by no check; the names the exporter writes are a contract no file lists.

**Change.** An uploaded release archive, an install job per admitted version, a bound, named warnings as errors in the flake's check, a check that compiles the README's blocks, and `tests/Interface.v` with a README section that lists the contract.

**Net.** The library's own `nix flake check` and its CI.

- F-9-01 (confirmed, build, medium), `rocq-deduxit/README.md:151`: The certificates' interface (the modules, theorems, record fields, constructors and helpers deduxit's exporter writes) is listed nowhere in the library and checked by none of its tests, so a rename here fails only in deduxit's flake once its lock moves.
- F-9-02 (confirmed, build, small), `rocq-deduxit/.github/workflows/release.yml:87`: The release publishes the SHA-512 of GitHub's generated tag archive, which the Rocq opam archive's guidance warns keeps no stable checksum, instead of an archive the release uploads.
- F-9-03 (confirmed, build, medium), `rocq-deduxit/.github/workflows/ci.yml:45`: CI never installs the package from its opam file: the per-version opam install and `opam lint` run only by hand (`scripts/opam-install.sh`), `opam lint` runs without `--check-upstream`, and the bridges compile with one Rocq only.
- F-9-09 (confirmed, build, small), `rocq-deduxit/README.md:128`: No check compiles the README's six Rocq examples or runs its command line, so an example can drift from the library and the command without anything failing.
- F-9-11 (confirmed, build, small), `rocq-deduxit/_RocqProject:1`: The project file sets no warnings as errors and no `Default Goal Selector "!"`, so the library's own build accepts any new warning or deprecation, and a tactic may act on every open goal unnoticed.
- F-9-12 (confirmed, build, small), `rocq-deduxit/rocq-deduxit.opam:45`: `rocq-stdlib` has no upper bound, so a future standard library is admitted untested.

### H2. The Rocq library: proofs that rely on generated names and deprecated forms

3 findings (3 confirmed, 0 candidates); kinds: refactor.

**What.** 18 of 36 files fail the `-mangle-names` linter; 77 abbreviations use a form deprecated from Rocq 9.2, so the library warns on two of its four versions; one proof selects a goal by its number.

**Change.** Name what is introduced, write the abbreviations out, select the goal by its shape. Robustness alone is the gain, at a large cost, so it ranks last in the library.

**Net.** The build with `-mangle-names` in the audit's inventory (`plan/audit/rocq.py`), then as a check; every theorem's statement unchanged (`Print Assumptions`, the certificates).

- F-9-15 (confirmed, refactor, large), `rocq-deduxit/theories/Linear/Mix.v:43`: Half the library's files (18 of 36) rely on names Rocq generates, so they fail the manual's `-mangle-names` linter and can break when a Rocq release changes its naming.
- F-9-04 (confirmed, refactor, large), `rocq-deduxit/theories/Linear/Calculus.v:38`: The theories declare 77 abbreviations with `Notation`, deprecated in Rocq 9.2, so the library compiles with deprecation warnings on two of the four versions the opam file admits.
- F-9-13 (confirmed, refactor, small), `rocq-deduxit/theories/Linear/FirstOrderIntuitionisticCheck.v:658`: The soundness proof of the first-order two-sided checker selects the cut's case by its goal number, which shifts when a rule is added before `ICut`.

### H3. The Rocq library: one copy of what the checkers share

6 findings (6 confirmed, 0 candidates); kinds: API break, refactor.

**What.** The checkers' multiset core and its lemmas are written up to four times, the two-sided checker's helpers and tactics twice, helpers of two namespaces under three names, and the first-order weights repeat the propositional file's lemmas. The rules record that the first-order files leave the propositional ones untouched; no reason beyond that is recorded, and the trusted statements are outside the code that would move.

**Change.** `Linear/Multiset.v` and `Deduxit.Base` for what two namespaces share; facts of first-order formulas beside their definitions; names and shapes of lemmas made to agree, with the changelog's list of renamings.

**Net.** The library's tests and `tests/Assumptions.v`; the certificates of deduxit's exporter against the library (the pair of changes).

- F-9-05 (confirmed, refactor, large), `rocq-deduxit/theories/Linear/Check.v:39 and rocq-deduxit/theories/Linear/FirstOrderCheck.v:125`: The multiset core of the checkers (`remove_one`/`pick`, `remove_all`, `take_all`, `diff`, `take`, the option bind and their permutation lemmas and tactics) is written out once per checker, four times in all, though only the formula type and its equality differ.
- F-9-06 (confirmed, refactor, medium), `rocq-deduxit/theories/Linear/IntuitionisticCheck.v:389 and rocq-deduxit/theories/Linear/FirstOrderIntuitionisticCheck.v:536`: The two-sided checker's cut and promotion machinery and the four tactics of its soundness proof are copied verbatim from the propositional file into the first-order one, and the same pass `iderive` is written in two styles.
- F-9-07 (confirmed, refactor, medium), `rocq-deduxit/theories/Intuitionistic/KripkeCheck.v:46 and rocq-deduxit/theories/Modal/Countermodel.v:46`: List helpers that two namespaces need are written once per file under different names, and `Deduxit.Base`, the library's stated place for them, does not exist.
- F-9-14 (confirmed, refactor, medium), `rocq-deduxit/theories/Linear/Weights.v:271 and rocq-deduxit/theories/Linear/FirstOrderWeights.v:116`: The first-order weighted count repeats the propositional one's list lemmas and its invariant's case analysis, and the first-order `ill_ll` repeats the propositional one's 22 cases verbatim.
- F-9-16 (confirmed, refactor, small), `rocq-deduxit/theories/Linear/FirstOrderWeights.v:403`: Basic lemmas about first-order formulas and terms live in the file that first needed them, one of them twice.
- F-9-17 (confirmed, API break, medium), `rocq-deduxit/theories/Modal/Check.v:85`: The same notions carry different lemma names and shapes across the namespaces: one-way `_eq` and `_in` lemmas beside `reflect`-valued `_spec` and iff-valued `_In` ones.

### H4. The Rocq library: text and two candidates of speed

3 findings (1 confirmed, 2 candidates); kinds: refactor, text.

**What.** The opam description and the README name linear logic first and omit modal logic. Two candidates: every structural equality joins its recursive calls with `&&`, which `vm_compute` evaluates in full; a certificate's term restates each tensor's subformulas, so its text grows quadratically with a formula's depth.

**Change.** The texts in the page's order; measure the two candidates on `rocq compile` time of the snapshot certificates and of `!A ⊢ A^⊗n`, and mend the exporter (a `let` inside the term) only if the time shows.

**Net.** The library's check; for the exporter, the `rocq-deduxit` check and the snapshots.

- F-9-08 (confirmed, text, small), `rocq-deduxit/rocq-deduxit.opam:9`: The opam description leaves modal logic out of what deduxit does and names linear logic first; the README and the Nix package description put linear logic first too, against the suites' order.
- F-9-10 (candidate, refactor, medium), `rocq-deduxit/theories/Linear/Formulas.v:78`: Every structural equality the checkers compute with joins its recursive calls with `&&`, which `vm_compute` evaluates in full even when the first side is already false, though the library itself avoids `&&` elsewhere for that reason.
- F-9-18 (candidate, refactor, medium), `rocq-deduxit/tests/FromCommand.v:64`: A certificate's proof term repeats each `⊗` node's whole subformulas, so its text grows quadratically with the depth of a formula: a 192-node term for `!A ⊢ A ⊗ … ⊗ A` (64 copies) makes a 44 KB file.

## I. The build: features and the web module's size

What a user compiles and what a visitor downloads. (7 findings in 2 themes.)

### I1. The features

5 findings (4 confirmed, 1 candidates); kinds: API break, build.

**What.** `serialize`, named for the JSON forms, also turns on the rule files' checker and modal logic's decision procedure; `latex`, `typst` and `svg` are three features with no dependency of their own, 258 `cfg` sites and 21 of the 66 pairs the checks build; no feature gives the data model and the checkers without the engines; the serde feature is not named `serde`.

**Change.** One feature `export`; `serde` as the feature's name; `rules` and modal logic out from behind the JSON feature where their code does not need it. Whether a feature `search` is worth its `cfg` sites is a measurement the proposal asks for, and it meets the author's decision of 2026-11-09 against a feature per suite (`standing.md`).

**Net.** Both `cargo hack` runs and the `features` check, which get cheaper; the manifests of the workspace's other crates.

- F-arch-16 (confirmed, build, medium), `core/src/lib.rs:350; core/src/modal/mod.rs:45`: The feature `serialize`, named for the JSON forms, also turns on a proof checker (`rules`) and the decision procedure of modal logic.
- F-arch-17 (candidate, build, large), `core/Cargo.toml:48; core/src/linear/mod.rs:25`: No feature gives the data model and the checkers alone: `default-features = false` removes every reader and keeps every engine of every suite.
- F-arch-22 (confirmed, build, medium), `core/Cargo.toml:60; core/src/export/mod.rs:85`: `latex`, `typst` and `svg` are three features that no consumer in the workspace enables apart, with no dependency of their own, and they cost 258 `cfg` sites and 21 of the 66 feature pairs the checks build.
- F-6a-04 (confirmed, build, large), `core/src/lib.rs:350`: The JSON feature `serialize` gates whole suites: calculi given as rule files exist only with it, because `rules::Calculus` stores the serde proxy, and with them the modal decision; a user who wants modal logic without serde, or the data model and checkers alone (consumer test 13), has no cut that gives it.
- F-6a-17 (confirmed, API break, small), `core/Cargo.toml:65`: The feature that adds serde is named `serialize`; the guidelines (C-SERDE) name it `serde`, which Cargo allows beside `dep:serde`.

### I2. The web module's size

2 findings (0 confirmed, 2 candidates); kinds: build.

**What.** The module grew by 101 634 bytes at gzip in steps 82 to 85 and stands 1 % under its ceiling. `inventories/wasm.md` attributes its raw bytes: the linear suite a fifth, ordinary logic a seventh, the serde code a ninth. `lambda::Node` and `lambda::Type` deserialize through serde's buffered `Content`, and their two functions are a third of the lambda suite's bytes.

**Change.** In this order: the stop shims of D3 and one `never` in the bindings; serde forms that do not buffer; a registry the bindings build from the page's list of suites, so that the workspace links only what the page uses (it needs the registry as a value, B10); a module per suite only if the measurement after these still asks for it.

**Net.** The budget check; the module's bytes after wasm-opt (`inventories/wasm-opt.md`).

- F-arch-18 (candidate, build, large), `core/src/workspace/mod.rs:1062; core/src/protocol/mod.rs:418`: The web module grew by 549 168 bytes at gzip in six days and every visitor loads it whole; D29's condition for a module per suite ("a measured size") is met, and the library has one obstacle to it: the workspace runs operations through a static registry of every suite.
- F-5a-07 (candidate, build, small), `core/src/lambda/mod.rs:615`: `Node` and `Type` deserialize as internally tagged enums, which serde buffers through `Content`, and their two functions are among the web module's largest, about a third of the lambda suite's bytes.

## The rejected findings

Eight findings a verifier refuted, kept so that none is raised twice
(`rejected.md` is the same list).

### Stage 2

- **F-6a-14**, `core/src/errors/mod.rs:460`. Claim: the message of
  `GlivenkoFirstOrder` is a nine-line essay with a countermodel and
  Rocq lemma names instead of the cause and the remedy. Rejected:
  `core-ordinary.md:2219-2226` records the message's content on purpose
  (no search of deduxit can show that countermodel, so the message
  carries it) and ties it to the manual and the guide; the finding
  neither names the choice nor says why its reason no longer holds.
- **F-7a-18**, `cli/src/batch.rs:565`. Claim: the batch edits
  serde_json's output as text where a `Serialize` record struct with
  `#[serde(flatten)]` would do. Rejected: an outcome writes its own
  `version`, so a flattened record writes it twice; `preserve_order`
  would change the key order of every `Value` in the workspace and add
  a package; `seconds` written by `{}` differs from serde_json's float
  form. `cli.md` records why the record is assembled as text.
- **F-7a-19**, `cli/src/prove.rs:876`. Claim: times in verdict lines are
  `Debug` of `Duration`, so users read `µs`, a unit `--timeout` does not
  take. Rejected: a limit echoed in a message reads back (`1.5s`,
  `1500ms`); `µs` appears only in elapsed times, which no flag takes,
  and users understand it. A finding on relying on `Debug`'s unpromised
  format would need another criterion and a shown harm (27 lines of the
  manual show such times).

Unit 9 was verified twice: its first verifier started before the
findings' full text was on disk and judged from the one-line claims
(`units/9/verdicts-first.md`), rejecting F-9-01; the second, with the
full text, confirmed all 18 (16 confirmed, 2 candidates), F-9-01 with
its witness corrected (the library's tests do use the exporter's names;
what is missing is the list that marks them as the contract). The
second verification stands.

### Stage 3

- **F-3-09**, `core/src/linear/nets/mod.rs:275`. Claim:
  `NetError::NoVertex` names its vertex as a bare `u32` where every
  other variant has a `VertexId`. Rejected: the number names no vertex
  of the structure (passing it to `partner` or `occurrence` answers
  `None`, `core-nets.md:36-39`), and the bare number says so.
- **F-3-12**, `core/src/linear/nets/mod.rs:1278`. Claim: the nets module
  holds 36 `must_use_candidate` sites, predicates among them. Rejected:
  `must_use_candidate` is one of the ten lints set aside on purpose
  (`Cargo.toml:95-100`); E7 admits a finding on one only if its reason
  no longer holds, and marking 36 of 556 sites would not turn the lint
  on. The same holds for the other nine: `many_single_char_names`,
  `similar_names`, `too_many_lines`, `enum_glob_use`,
  `single_match_else`, `match_bool`, `if_not_else`,
  `items_after_statements`, `naive_bytecount`.
- **F-4a-10**, `core/src/ordinary/decide.rs:885`. Claim: two places
  decide between "unknown" and "defect" by comparing an error's message
  with a sentinel string. Rejected: both sides use the one private
  constant `OPEN_WITNESS`, so rewording cannot turn one into the other;
  and the criterion cited (an AIP) applies to the JSON wire forms only.
- **F-4a-20**, `core/src/ordinary/mod.rs:720`. Claim: seven comments
  use "refers to" and "stands as". Rejected: at those places the verbs
  carry their own meaning (a node refers to its subformulas by index;
  terms "stand as they are", that is, stay unchanged). `text-read.md`
  lists the seven places in the crate where "stands as" does replace
  "is"; a text finding of this kind names one of those or shows the
  sentence.

Stage 4 rejected none of its 80 findings.

### Stage 5

- **F-5b-19**, `core/src/rules/fitch.rs:187`. Claim: `Fitch::new`
  borrows its options and then clones them, where taking them by value
  would spare a caller the copy. Rejected: every caller outside the
  tests borrows the options from styles it keeps, so by value each
  would clone at the call site instead, and borrowing them for the
  drawing's lifetime breaks the documented call with
  `&FitchOptions::default()`.
