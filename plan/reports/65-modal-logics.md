# Step 65: modal logics K to S5, decided with Kripke countermodels

## Session 65a: the design and the arena

Fable 5.2 at `high` (a reused worker, its conversation cleared), in the
workspace `../deduxit-wt-step-58b-8c0bbc`, from the brief
`/tmp/linlog-handoff/start-65a.md`, on 2026-11-16, cores 10 to 12.

### Outcome

- **The design is written**: `plan/notes/modal-design.md`, fourteen
  sections: the arena, the systems, the Kripke model and its check, the
  search for all fifteen systems of the modal cube with the loop check
  each needs, what it costs, the countermodel, the proof and its check,
  the outcome and options, the JSON forms, a user's model, the suite in
  63a's protocol, the tests 65b holds the search to, what each later
  session builds, and every source with how it was read.
- **The code reached the brief's list**: the library has
  `deduxit::modal` with the formula arena, the sequent, its text parser
  and its JSON form, and `System`, the fifteen systems as values.
  Nothing decides a modal sequent yet.
- **The design's main bet was tried**: a labelled rule file for S4 and
  a derivation of axiom 4 in it pass `deduxit check --calculus` on
  `main` as it is, and the same derivation with a label that is not
  fresh is refused. The generic checker of step 60 can therefore be the
  check of every modal proof without a change.

### The design's decisions and their sources

Each is argued, with its alternatives, in the note's section named.

1. **`□` and `◇` are both nodes of an arena of its own** (section 2),
   a copy of the ordinary arena as D25 asks. A formula prints as
   written.
2. **A sequent is read at one world** (section 2): validity of
   `Γ ⊢ Δ` is validity of `⋀Γ → ⋁Δ`. Global consequence is not built;
   the note says where it goes.
3. **A system is a closed set of five frame conditions** (section 3),
   not an enum of names: the 32 sets close to the fifteen logics of the
   cube, and `==` is equality of frame classes. Brünnler needs the same
   closure ("45-complete" sets) for his rules to be complete. All
   fifteen are values, the prompt's seven among them.
4. **A modal model's arrows are the relation itself** (section 4):
   nothing is closed for the reader, so the frame conditions can be
   checked and the failing one named, as the prompt asks. The ordinary
   suite keeps reading the same key as a generating relation.
5. **One search for every system: backward proof search in Negri's
   labelled calculus G3K with the frame rules of the system's
   conditions**, run as a tableau over a graph of worlds whose relation
   is kept closed under the frame rules (section 5). Twelve systems need
   no loop check (modal depth falls in K, D, T, KB, KDB, B; the
   euclidean six have one cluster that shares its witnesses); K4, KD4
   and S4 block a world by a proper ancestor whose formulas, the
   inherited box formulas among them, include its own (Garg, Genovese
   and Negri's condition).
6. **The proof is a `rules::Derivation` in the system's labelled rule
   file, checked by the generic checker** (section 7). No new checker
   is trusted for "valid". This is the design question the prompt left
   open ("a proof in the calculus the search uses"); it moves the rule
   files of the labelled calculus from step 66 into 65b, and leaves
   step 66 the sessions on them.
7. **The countermodel is the open branch as a `kripke::Model`, checked
   by a modal model check** (sections 4 and 6), the one new trusted
   piece, for 65b's panel.
8. **The search is untrusted**: a wrong argument about termination or
   blocking can only end in an error of kind `defect`, never a wrong
   verdict.
9. **Complexity, stated honestly** (section 5.4): the problems are
   PSPACE-complete without the axiom 5 and coNP-complete with it, as
   the prompt says for K, T, S4 and S5. The search keeps a branch's
   whole tree of worlds, because the countermodel and the labelled
   proof are that tree, so it is not a PSPACE procedure in the nine
   systems without 5; the limits bound it.
10. **The suite's kinds and operations** (section 11): four kinds
    (`modal.sequent`, `modal.outcome`, `modal.model`,
    `modal.evaluation`) and eight operations. The model is a kind here,
    since checking a user's model takes one as an input.

Sources (the note's section 14 has the locators): Negri 2005 and 2009;
Garg, Genovese and Negri 2012; Brünnler 2009 and his habilitation;
Massacci 2000; Fitting 2012; Goré and Ramanayake 2012; Goré 1999; Wu
and Goré 2019; Horrocks, Sattler and Tobies 2000; Halpern and Moses
1992; Halpern and Rêgo 2007; Hemaspaandra and Schnoor 2008; Rybakov and
Shkatov 2019; MOIN, TPTP, the Modal Logic Playground, CEGARBox, Carnap
and FormalizedFormalLogic for syntax and names. Three sub-agents on
Sonnet 5.5 at `high` read them; their notes, with quotations and what
each could not obtain, are `plan/notes/research/65-modal/`. Ladner 1977
and Heuerding, Seyfried and Zimmermann 1996 were not obtained; their
results are cited through the papers that state them.

What is the session's own argument and no source's (marked "(ours)" in
the note, for 65b's panel-argument): three of the five implications
between the conditions (each two lines, and all five checked against
every frame of up to three worlds by a test); that the ancestors' sets
are final when a world is tested for blocking; the termination of the
search in the euclidean systems (the frame shape is Halpern and Rêgo's
Lemma 3.2; Brünnler's uniform check is the fallback); the reflexive
arrow of a dead end in a serial system; the pruning of a split whose
first premise does not use it.

### The code

- `core/src/modal/mod.rs`: `Formulas`, `Node` (the ordinary nodes with
  `Box` and `Diamond`), `NodeId`, `Formula`, `Sequent`, `Display`.
- `core/src/modal/system.rs`: `Condition` (non-exhaustive) and `System`
  (a `Copy` value; the fifteen constants, `ALL`, `NAMES`, `of`, `with`,
  `has`, `conditions`, `extends`, `name`, `Display`, `FromStr`).
- `core/src/modal/parse.rs` (feature `parse`): `Sequent::parse_within`
  and `FromStr`, the ordinary syntax with `[]`/`□` and `<>`/`◇`.
- `core/src/serialize/modal.rs` (feature `serialize`): the sequent's
  form (the ordinary one with `{"□": a}` and `{"◇": a}`), a
  `wire::Readable`; `System` as its name.
- `core/src/wire.rs`: `MODAL`, the table's row, the forms' entry.
- Tests, nine: the implications against every frame of up to three
  worlds and the fifteen systems; the names; the arena's sharing and
  refusals; the syntax; the occurrence bound of the text reader; the
  JSON's level, bytes and round trip; the JSON reader's checks; a
  system's JSON; formulas nested 50 000 deep on a small stack
  (`core/tests/depth.rs`). Three doc tests.
- Docs: `CHANGELOG.md` (Unreleased), one bullet of the manual's
  `status.md`, a paragraph of the guide's reference, the wire docs,
  `.claude/rules/core-modal.md` (new) and `core.md` (the table, the
  layout, the API, both lists of types: `modal::Node` closed,
  `modal::Condition` marked, 161), `CLAUDE.md` and `claude-infra.md`
  (the module and its rules file listed).

Options (D15, D16): none exists yet; the note's section 8 gives
`modal::Options { system }` with K as the default, the command's
`--system`, and the page's tab setting.

### What it copied (for step 67)

- **From `ordinary`**: the arena whole (`Formulas`, `Node`, `NodeId`,
  `add`, `atom`, `holds`, `node`, `len`, `atom_name`, `atom_names`,
  `formula`, `Formula`, the 12-byte assertion, `MOST`), `Sequent`
  (`new`, the accessors), `occurrences` and `bound`, the parser but for
  its TPTP dialect (`Parser`, `Pending`, `Binary`, `End`, `formula`,
  `wait`, `within`, `add`, `constant_or_atom`, `reduce`,
  `count_within`, the second-turnstile error), the sequent's serde form
  and its reader, and three tests' shapes. The differences are two
  variants, the prefix table in place of one negation, and a level
  read from the content.
- **Used as they are**: `linear::sequents::{Stop, Stops}` (the walk a
  printer follows), `linear::sequents::name` (`normalized`,
  `check_ordinary`, `starts_identifier`), `wire::{Readable, version,
  fail}`, `errors::ParseError`, `limits`.
- **Not copied**: `Symbols` and the `Notation` table (the modal
  `Display` writes Unicode directly; 65c's exports will need the table
  with two more symbols), the TPTP reader.
- What step 67 can read from it: an arena generic over its node type
  with `operands()`, and a parser generic over its prefix and binary
  tables, would serve both suites; the ordinary and modal sequent forms
  differ by two tags.

### Checks

- **The modal tests** (nine, and three doc tests): pass (`cargo test -p
  deduxit modal`, with the defaults and with no feature).
- **Clippy** on the workspace with warnings denied, `RUSTFLAGS="-W
  unreachable_pub -W unnameable_types" cargo check -p deduxit
  --all-features`, and `cargo hack check --each-feature -p deduxit`:
  clean.
- **The gate** (`nix develop -c gate`, capped on cores 10 to 12): the
  first run failed at one test, `about_lists_the_suites` of the
  bindings, which pins the wire level `about` reports (18); the level
  was meant to change, so the test now pins 20 with its reason, in the
  same commit as the level. The second run passed (Clippy, the tests
  with the behaviour lock, both `cargo hack` runs, the ratchet within
  its ceilings). Four lines of doc comments changed after it (the
  reviewer's finding 5); the flake check below ran on them.
- **`nix flake check`** under the machine lock at `llurntnq`: the first
  run failed at `conventions`, which read `D45` in a doc comment of
  `system.rs` as a plan decision's number (its pattern is a `D` and one
  or two digits in a comment); the comment now gives `DB` as its
  example. The second run passed ("all checks passed!"). One line of
  `.claude/rules/core-modal.md`, which records that pitfall, was added
  after it; no check reads `.claude/`.
- **The rule file tried**: `deduxit check --calculus
  plan/notes/research/65-modal/g3s4-draft.json
  plan/notes/research/65-modal/four-draft.json` answers "valid in the
  calculus given (G3S4-one-zone): ⊢ x : □p → □□p, a derivation of 6
  nodes"; a derivation that takes a label of its conclusion as fresh is
  refused. The reviewer ran both again.
- Not run: the fuzz targets (none exists for the modal readers yet),
  the mutation run, any benchmark.

### The reviewer's findings

`fresh-reviewer` on Opus 5.5 at `high` (the writer on Fable) read both
commits, the brief and the code the note builds on, ran the tried rule
file, tried odd inputs on the parser and the JSON reader by reading,
and attacked each argument of the note's sections 5 and 6. It found no
counterexample to an argument (the implications, termination by modal
depth with symmetric edges and Ser, the blocking condition and the
model read from a blocked branch, the euclidean systems' one cluster,
the reflexive arrow of a dead end). Six findings:

1. **The code's commit failed a workspace test**: the fix of the
   bindings' pinned level was still uncommitted when it read the chain.
   Held; the fix is folded into `llurntnq`.
2. **The wire level was taken against the brief**, and a build at 20
   reads a document that says 19, which no form of this chain defines.
   Held as a departure, not changed: the planning session confirmed 20
   as the placeholder, the chain lands after 63g-b's 19, and the final
   rebase checks that `main` has 19. Landing this chain before 63g-b
   would leave `main` reading an undefined level; the stack order
   prevents it.
3. **The note's list of where the number stands was short** (the
   changelog, the forms' entry of `wire.rs` and the bindings' test were
   missing). Held; the note and this report list every place.
4. **The note's protocol table contradicted 63a's code**: `modal.decide`
   was classed "work" (a decision is `derived` with cost `search`;
   `work` is a session), its promises named "none" and `checks`, which
   are not among 63a's seven, and three kinds had prose for a role.
   Held; the table now uses `Class`, `Cost`, `Role` and the existing
   promises (`decides`, `proves`, `exact`, `renders`).
5. **The JSON level is read from the whole arena**, so a sequent whose
   arena holds an unused `□` is written at 20, which the docs' "a
   sequent without `□` and `◇`" did not say. Held for the docs, not for
   the code: the form writes every node of the arena, and a reader of
   level 8 refuses the tag `□`, so the whole arena is what the level
   must read. The doc comment on `modal::Sequent`, the wire table's
   row, `serialize::modal::level`'s comment, `core-modal.md` and the
   note now say "whose arena holds".
6. **Four citations said more than the sources read**: a phrase in
   quotation marks that was the research note's paraphrase of Garg,
   Genovese and Negri; "Lemmon's codes", which no source read names;
   Goré and Ramanayake for nested sequents, which they prove for
   tree-hypersequents and only sketch for nested ones; Halpern and
   Moses's exponential models for nine systems, which they show for K,
   T and S4. Held; each sentence is corrected.

### Commits

- `ntmsxtsv` "Plan: the modal suite's design (65a)": the note, the
  research notes and the tried rule file.
- `llurntnq` "Add modal formulas, sequents and the systems from K to
  S5": the module, its JSON, the wire level, the tests, the docs and
  the rules files.
- This report, on top.

### Departures and decisions

- **The wire level is taken as 20** (departs from the brief: level
  taken before 19 landed): a form that reads back needs a number at
  most `wire::LEVEL`, so the chain sets `wire::MODAL` and `wire::LEVEL`
  to 20, leaving 19 to 63g-b. The planning session confirmed 20 as the
  placeholder during the session and asked for a check at the final
  rebase that `main` has 19 and not 20. At the session's end
  `main@origin` had not moved (`pktprmmq`, level 18: 63g-b had not
  landed), so there was nothing to rebase onto and that check is still
  owed at stacking: if 63g-b's level is no longer 19, this chain takes
  the next free one. The number stands in `core/src/wire.rs` (the
  constant, `LEVEL`, the table's row, the forms' entry), in the JSON
  example on `modal::Sequent` (`core/src/modal/mod.rs`), in the
  changelog's entry and in `web/tests/ordinary.rs`
  (`about_lists_the_suites` pins the level `about` reports, 18 before);
  the library's tests read the constant. Stacking after 63g-b meets a
  conflict on the `LEVEL` line, the table and that test. No
  `core/tests/wire/level-N.txt` is written until the number stands.
- **The suite is not registered in the protocol** (departs from the
  prompt's "implements `Suite` from the start"): a suite without an
  operation would show empty in `describe()`, which the client reads.
  65c registers it with its first operation; the note's section 11 has
  the kinds and operations.
- **No entry in `docs/src/wire.schema.json`**: the `wire-schema` check
  wants a document of every form from the command, which writes none
  yet. 65c adds the entry with the command.
- **All fifteen systems are values**, where the prompt names seven: the
  five conditions give them, and the model check serves all alike.
  Whether the search offers the other eight is for the author (below).
- **The proof's calculus** (decision 6 above) is the prompt's open
  question, decided here by the literature and by D28.
- **The research notes are kept** under `plan/notes/research/65-modal/`
  with the tried rule file, since 65b's panel will want the quotations.

### For the author, through the planning session

1. Should the eight systems beyond K, D, T, B, K4, S4 and S5 be offered
   in 0.6.0 where they pass the same tests? Recommended: yes; KD45 is
   the logic of belief every course on epistemic logic uses, and they
   cost no engine of their own.
2. The command's spelling for the modal suite, `--system S4` selecting
   it as `--logic` selects the ordinary one, is a new flag; 65c
   proposes it with its help text.

### What the next sessions must know

**65b (the search and its panel)** builds, in this order:
- `modal::calculus(system)`: the `CalculusForm` built in code (the
  core crate has no JSON parser outside the feature `protocol`), the
  shape that was tried: `plan/notes/research/65-modal/g3s4-draft.json`,
  one zone pair `left`/`right` of sort `item`, discipline `set`, `:` and
  `R` both making an `item`. A node's `at` lists the conclusion's
  written members, then a premise's member that binds a new variable
  (R□ takes two positions). Label atoms carry `"sort": "label"`.
- The model check (`modal/model.rs`, note section 4) first, since
  every countermodel and every test goes through it; then the search
  (section 5), the model's reading (section 6), `decide`, the outcome's
  JSON (section 9).
- The search needs `serialize` (it returns a `rules::Derivation`); the
  arena, the systems and the model check need no feature. Both `cargo
  hack` runs must stay free of warnings.
- The panel: the model check (counterexamples, integers), the rule
  table against Negri's Table II and p. 515 (argument), and the
  arguments the note marks "(ours)" (argument). The tests of the note's
  section 12 are the second references.
- The seven systems are required; the other eight are the same code
  and are offered only where they pass the same tests.
- `MODEL_CELLS` moves to `kripke` when the modal check comes to share
  it, re-exported from `ordinary`.
- `Model`'s inherent methods `refutes` and `forcing` are the ordinary
  suite's, so the modal ones need other names (`meets`, `truth`).
- A fuzz target for the modal text reader and one for its JSON reader
  (`fuzz/`), as every reader has.

**65c (the command)**: the wire level's final number in
`core/tests/wire/`, the schema's entries, the suite's registration, the
symbols per export target (the SVG font's advances for `□` and `◇`),
`kripke::Model`'s `Display` says "below" for an arrow, which is the
ordinary reading.

**65d (the page)**: a modal model's arrows are no order (loops,
cycles), so step 55's drawing needs a layout for them; implied arrows
may be hidden by an option of the drawing, never in the data.

## Session 65b: the search and the panel

Opus 5.5 at `high` (the 65a worker, its conversation kept), in the
workspace `../deduxit-wt-step-58b-8c0bbc`, from the brief
`/tmp/linlog-handoff/start-65b.md`, on 2026-11-16, cores 10 to 12.

### Outcome

- **The library decides modal logic in all fifteen systems of the
  cube** (`modal::decide`, `decide_within`): a valid sequent comes with
  a derivation in the system's labelled calculus (`modal::calculus`,
  Negri's G3K with the frame rules of the system, a calculus given as
  data), which the generic checker of step 60 accepted; a sequent that
  is not valid comes with a Kripke model whose relation meets the
  system's frame conditions and that refutes it, both checked before
  the outcome is given. All fifteen pass the same tests, so all fifteen
  are offered (65a's question 1, as the planning session decided).
- **A model the user gives is checked** (`Model::frame_fault`,
  `Model::truth`, `Truth`): the first frame condition the arrows as
  written fail, with the worlds that show it, then each formula's value
  at each world, whether the sequent holds at a world or anywhere, and
  for a false `□A` or a true `◇A` the world that shows it.
- **The outcome has a JSON form** at level 20 (`serialize/modal.rs`),
  which reads back unchecked; `Outcome::check` checks a read-back
  outcome's evidence against its sequent and system.
- **Three fuzz targets**: `modal_text`, `json_modal`, `modal_decide`
  (built, seeded; not run, below).
- **The panel** of three found no unsound answer; it found four bound
  faults, one termination fault in practice (KD4 and S4 at tens of
  millions of units), and three claims the docs made wrongly. All are
  answered in code or in the docs (below).

### The search, the proof and the countermodel

As the design note's sections 4 to 7 say, with these choices made here:
- **One branch** holds worlds (two bit sets over the arena each, sorted
  arrows both ways), an agenda, the formulas waiting for a split, the
  `□` false and `◇` true waiting for a witness, and the steps since the
  last split. The relation is kept closed under the system's frame
  rules as each edge is acted on, each derived edge a step of the
  derivation, so a `□` reaches a world along a derived edge by `L□`.
- **Blocking only in K4, KD4 and S4, by any earlier world that no world
  blocks and whose signature includes the world's** (departs from the
  design, which took Garg, Genovese and Negri's proper ancestors): the
  counterexamples member found `⊢ □(□□q ↔ r) → ◇□(□◇q ↔ r)` of
  fourteen nodes taking 40.7 million units in KD4 and `□(□(p ↔ q) ↔ □q
  ∨ p) ⊢ ◇□◇(p ∧ q), q` reaching the 1 GiB memory bound, both not
  valid: sibling worlds of equal content each expanded. With blocking
  by any earlier world, as description-logic tableaux without inverse
  roles block anywhere, they end in 105 000 and 162 000 units, and the
  third witness, which also ran out of memory in S4, in 38 000. The
  argument for the model is the ancestors' one: in these systems no
  edge leads to an older world, so every existing world's signature is
  final when a world is first tested, and a blocked world that waits
  takes its blocker's arrows (with the blocker's loop first where it is
  a serial system's dead end, so never none), then the transitive
  closure. The model check decides each model anyway.
- **A step is kept only where the proof above uses its new members**,
  and a split whose first premise closes without its new members is
  not searched further. Measured before it was kept (the design asked
  for it): twenty worlds that each split once beside one that closes
  took 20 splits with it, and without it 2²⁰ branches, unknown at 2²⁶
  units (`independent_splits_do_not_multiply` pins it at 16).
- **Every principal formula is kept in its premises** (departs from the
  design, which named G3c's rules): a tableau branch keeps what it
  expands, and with the calculus's set zones the branch is then the
  derivation's sequent as it is. The calculus also has `L¬`, `R¬`, `R⊤`
  and the `↔` rules, which Negri's Table II lacks, and no initial
  sequent on a relational atom; the doc comment of `calculus()` lists
  each, the argument member checked every rule sound.
- **The derivation** is emitted by a walk over the kept steps, each
  node's sequent the root's and its path's premise lists (whole, not
  only the new members, so a member a left-out step put first is still
  there), each zone sorted, positions by the rule compiler's order. Its
  size is estimated first, terms included, against
  `limits.derivation_bytes`; the build asks the stop.
- **A model too large to check is dropped and "not valid" stands**, as
  in the ordinary suite (the design's section 6); the argument member
  found the docs saying otherwise in three places, now corrected:
  there the verdict rests on the search's arguments, which the panel
  found sound.
- **No `Error::Frame`** (departs from the design): the failing
  condition is data in `Model::frame_fault`'s answer (`Frame`), and
  `truth` refuses with `Error::Model` and the frame's text, so the
  shared error family is untouched while 63g changes it. `MODEL_CELLS`
  stays `ordinary`'s (departs from the design, which moved it to
  `kripke`): the modal check reads it there.
- **`CHECKED`**: the frame conditions are checked reflexive first, then
  serial, symmetric, transitive, euclidean, so a world without arrows in
  T is told it does not see itself.
- **What `decide` needs**: the feature `serialize` (the generic
  derivation does); the arena, the systems and the model check need no
  feature.

### The panel's findings and answers

**`panel-counterexamples`** (Opus 5.5, its own formula type,
evaluator, frame check, checker of the G3 rules and a shrinker; 474 750
decisions over 31 650 sequents in all fifteen systems, 1 545 000 model
checks of 103 000 random models, 173 686 mutants of 2 300 derivations):
- No "valid" refuted on any frame of up to three worlds or 200 sampled
  four-world frames; every "not valid" with a model its evaluator and
  frame check confirmed; no defect error, no panic; `frame_fault` and
  `truth` agreed with its own code on every model; every mutant the
  generic checker accepted was a sound instance of a rule by its own
  checker.
- **Refuted: termination within 2²⁴ units** in K4, KD4 and S4 on small
  sequents (13 of 474 750 decisions, three shrunk witnesses above).
  Answered by blocking by any earlier world (`lntxmttw`), which ends all
  nine of its unknowns.
- **The rerun on the new blocking** (630 150 decisions over 42 010
  sequents, 126 000 of them in K4, KD4 and S4; 750 000 model checks;
  120 691 derivation mutants): no unsound answer, no defect, no panic,
  every "not valid" model confirmed by its own evaluator and frame
  check (blocked K4, KD4 and S4 models among them). **Still refuted in
  KD4 alone**: two sequents that are not valid in KD4 (KD45, a stronger
  system, refutes them with a model of 10 worlds) end above 2²⁴ units:
  `⊢ ◇((q → (◇⊤ ↔ ⊥)) → □□(p → q)), (◇¬◇(s ↔ q) → ⊤) → ◇(((p ↔ □□s) ∧
  ◇(◇s ↔ ◇q)) ↔ s)` (37 nodes) in 20.5 million units, and its 50-node
  original reaches the 1 GiB memory bound, so the default `decide`
  answers unknown on it. S4 decides both within 1.4 million units, K4
  in a few. Not answered in this session: it is a cost, not a fault
  (termination and soundness hold), of the search's branching where a
  system has neither reflexivity nor a cluster, with `Ser` making
  worlds at every dead end. What would answer it is a cache of the
  verdicts of world signatures across branches (the "global caching" of
  Goré and Nguyen's tableaux for K4-like logics, not read), for a later
  step; the author is told below.

**`panel-argument`** (Fable 5.2): every rule sound under the checker's
reading of set zones and contexts; the emitted derivation's invariant
(every needed member in the kept node's sequent, positions in the
compiler's order, fresh labels absent from the conclusion) holds at
every site; the five "(ours)" arguments of the design hold (termination
by modal depth, ancestor blocking and its model, the euclidean cluster,
the serial loop, the implications); the model check is the Kripke
semantics. Three findings:
1. "Not valid" without a checked model past `MODEL_CELLS` (a run of 500
   diamonds in S5), against three sentences of the docs. Answered in
   the docs (above); the verdict is the design's.
2. **A derivation read back was not tied to its sequent**: a document
   whose derivation proves `⊢ w0 : p` passed `Derivation::check` for any
   outcome. Answered by `Outcome::check` (the root must be `w0 : Γ ⊢
   w0 : Δ`, no assumption, the system's calculus), tested with a forged
   sequent and a forged system.
3. `calculus()`'s doc comment named two departures where there are
   five: corrected.

**`panel-integers`** (Sonnet 5.5): nothing wrapped or panicked; four
bounds did not hold:
1. **The model's closure** built every 2-path of the relation, uncharged:
   7 to 9 times the memory bound (S4 chain of 1 000 worlds at 1.48 GB
   under 200 MiB). Now only a blocked world's new arrows are closed, by
   a charged worklist: 175 MB under 200 MiB, 57 MB under 50 MiB.
2. **The derivation's terms** were made by a rescan of the arena per
   member, quadratic and unpolled (46 s at 500 000 hypotheses). Now made
   once in the arena's order, the build polled: 0.15 s at 200 000.
3. **The size estimate** left out the terms, a hundredfold short. Now
   counted (103 MB estimated against 81 MB built at 200 000).
4. **Scans the work count did not count** (every edge scanned the
   arena, a demand's test the worlds seen, a signature the bit sets).
   Each now owes its length to the next tick: the padded chain that ran
   5.3 s on 1.4 million units stops at its 2-million bound in 36 ms.
- Also: the step and proof indices are `u32` (a refusal now, `full`,
  past them), and the byte estimate of a branch saturates.

### Checks

- **The modal tests**: 25 in the library (the model check four, the
  calculus one, the search ten, the JSON five, the arena's and the
  parser's), with the defaults, with `serialize` and `parse` alone, and
  with no feature (the three that need neither); the doc tests.
- **Clippy** on the workspace with warnings denied, `cargo hack check
  --each-feature -p deduxit`, `RUSTFLAGS="-W unreachable_pub -W
  unnameable_types" cargo check -p deduxit --all-features`, and
  `cargo doc` with every feature, public and private, warnings denied:
  clean.
- **The fuzz targets** build (`nix develop .#fuzz -c cargo check
  --manifest-path fuzz/Cargo.toml --bins`); not run.
- **The gate** (capped, cores 10 to 12) on the rebased chain at
  `lzrknpwx`: passed, the ratchet within its ceilings.
- **`nix flake check`** under the machine lock: passed at `lzrknpwx`
  before the reviewer's fixes, and again at `pqmqzuoy`, the last code
  commit ("all checks passed!"); the guide and manual test
  (`cargo test -p deduxit-cli --test readme`) passed on the fixes too.
- **The panel's probes, rerun after the fixes** (the integers member's
  `run.sh`, the counterexamples member's harness against a copy of the
  fixed tree): the figures in the panel's section above.

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 at `medium` (the writer on Opus) read the
five commits, the rebase's resolution, the brief and the code, ran a
probe against the public API and the builds without default features.
It found nothing against the new blocking or the model's reading, and
seven findings, all held:
1. **`Outcome::check` took a countermodel of another sequent** (an
   outcome of `⊢ p ∨ ¬p` with the model of `⊢ p` passed; only through
   the public fields, since the JSON reader builds the countermodel
   from the outcome's own sequent). The check now compares the two
   sequents' texts; `forged_evidence_is_refused` pins it.
2. **A test's comment claimed a truth-table check its body did not
   make.** The comment is corrected, and `classical_sequents_keep_their_verdicts`
   now compares sixty generated sequents without modalities with their
   truth tables in all fifteen systems.
3. **The design's section 12 was only partly in the tree.** Added: a
   derivation forged once per node is refused
   (`forged_evidence_is_refused`). Left as they are, and recorded here:
   the small frames sample 48 of the 512 relations on three worlds (all
   512 times every valuation in fifteen systems was minutes in a debug
   build; the counterexamples member ran all of them out of tree); "a
   countermodel of an extension refutes in the system" is checked by
   the verdicts, not on the model; Halpern and Moses's formulas run in
   K for `m` up to 3, not in T and S4, and no test drives one to the
   memory bound (the integers member did, out of tree).
4. **`Statistics` and the rules file still named an ancestor** as the
   blocker: corrected.
5. **Four texts said every "not valid" comes with a checked model**
   (the changelog, the status chapter, the guide, `core.md`): each now
   says where it is small enough to check.
6. **The wire docs gave level 20 to the sequent alone**, where every
   modal outcome is written at it: the table's row, the list of forms
   and `MODAL`'s comment now name the outcome.
7. **CLAUDE.md and `core.md` described 65a's module**: corrected.

### Departures

Each marked above "(departs from the design)": blocking by any earlier
world; kept principal formulas; no `Error::Frame`; `MODEL_CELLS` not
moved. The tests of the design's section 12 are in the tree as the
reviewer's finding 3 says, the rest out of tree by the panel. Not done
here: the suite's registration in the protocol (65c,
with its first operation, as 65a said), a run of the fuzz targets
(below), the wire file `core/tests/wire/level-20.txt` (with the
command's documents, 65c).

### For the author, through the planning session

- **KD4 is offered with a known cost**: on some sequents of a few dozen
  nodes it needs tens of millions of units or more memory than the
  default bound, where S4 and KD45 decide them at once (the panel's two
  witnesses above). The other fourteen systems ended every one of the
  panel's 630 150 decisions within 2²⁴ units. Recommended: offer KD4
  with the rest, as decided, and keep a cache of world signatures across
  branches in `plan/later.md` for when a user meets it.

### What the next sessions must know

**65c (the command)**:
- `modal::decide_within(&sequent, &modal::Options::default().with_system(s), &limits, stop)`
  is the call; the time limit comes through `Settings::stop` as the
  other suites'. `Outcome`'s JSON is written at `wire::MODAL` (20);
  `Outcome::check` is the check a stored outcome needs (the protocol's
  `decides` promise).
- The derivation is a `rules::Derivation` in `modal::calculus(system)`:
  the command's `check --calculus` and its drawings take it as they
  take any (the calculus is not one of `rules`' built-in names: write
  it out with `Calculus`'s serde, or give `--calculus` a system name,
  which is 65c's to propose).
- `kripke::Model`'s `Display` says "below" for an arrow (the ordinary
  reading); a modal model needs "sees".
- A user's model: `model.frame_fault(system, …)` then
  `model.truth(&sequent, system, …)`; `Truth::failing`, `sequent_holds`,
  `holds`, `witness` answer the prompt's "which world fails and why".
- The schema entries for the modal sequent and outcome, and
  `core/tests/wire/level-20.txt`, go in with the command's documents.
- The fuzz targets were built, not run: run `modal_text`, `json_modal`
  (seed it with the command's documents) and `modal_decide` at the next
  fuzz run.

**65d (the page)**: a modal model's arrows are no order (loops,
cycles); the labelled derivation is a `rules` derivation, which the
page's tree view of calculi given as data draws; `Frame`'s worlds mark
the missing arrow.

**Step 66 (the labelled calculus step by step)**: `modal::calculus`
is the rule file; a session in it is `rules::Interactive` as it is.
