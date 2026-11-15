# Step 61: building derivations with custom rules

## Session 61a: the library and the command

Session 61a, 2026-11-15, the conversation of 60a and 60b, on Opus 5.5
at `high`. Brief: `/tmp/linlog-handoff/start-61a.md`; prompt:
`plan/61-custom-rules-interactive.md`.

### Outcome

A derivation in a calculus given as data is built step by step, in the
library (`rules::Interactive`) and the command (`deduxit interact
--calculus CALCULUS SEQUENT [--assume SEQUENT…]`). A rule applies
backwards from a goal; what the goal leaves open is asked for by name;
a goal closes by a hypothesis in force; `undo` takes back the last
step. The calculus's checker checks every step before the state takes
it, and a session read from JSON (wire level 17) step by step. The
manual's custom-rules chapter has the interactive part, and a new
chapter, "Write rules as hypotheses", says what the Horn engine already
runs and when a calculus of one's own is the better tool. No search
and no certificate (D28).

### The backward step

`Interactive::apply(goal, &Apply { rule, at, given })` (`plan`, then
`commit`):

1. The conclusion's members the rule writes out are found at the
   positions `at`, numbered across the goal's zones (the conclusion's
   `Find::Position` members, in the compiled order), or at the forced
   places (`Start`, `End`), and matched first-order; the bindings come
   out.
2. `given` values are parsed in the calculus's syntax: a variable's
   term, or a context's members separated by `,`.
3. Each conclusion zone's contexts are solved by its discipline: a
   multiset's or a sequence segment's one unknown context is what is
   left; two unknown in one segment is a split, asked for by name
   (`Error::StepNeeds`, before anything else, since the rest follows
   from it); a set's contexts at their largest (a tableau keeps the
   expanded formula), intersected where one stands in several zones.
4. A variable the rule asks to be new and that is not given gets a new
   atom named after the variable (`y`, `y1`, …), unused anywhere.
5. The premises are built (`Terms::apply`), a set zone deduplicated;
   a variable no conclusion member binds is asked for by name; the
   premises' position members are located where they landed, so the
   node's `at` is complete.
6. `commit` appends the premises as open goals (a premise that
   discharges in a new scope that the goal owns, its hypotheses
   instantiated), sets the goal, and runs the checker on that node of
   the whole session's derivation (`check_node`); a refusal is rolled
   back and answered as `Error::StepRefused` with the checker's reason.

`assume(goal)` closes a goal by a hypothesis its scope chain offers,
innermost first: the goal's node becomes a hypothesis of that scope and
the owner rule node is re-checked, whose scope check verifies the
discharge; a root-scope hypothesis is one of the session's assumptions.
`rules(goal)` tries every rule at every choice of the conclusion's
position members (at most `RULE_CHOICES`, 4 096) on a clone and lists
those the checker takes, and those that need values, with their names.
`Calculus::positions` stays a count: `rules` gives the member numbers a
front end needs (decided here instead of a list of what each position
stands for; it gives what the user picks from directly).

### Decisions and their reasons

- **Every step is checked by the checker itself**, not by a second
  validation of the session's own: the backward solver is untrusted
  code, and the one argument that a session's state is sound is the
  checker's. The cost is a conversion of the session per step (linear
  in its size), well within interactive use.
- **The session embeds its rule file** in its JSON, so a saved session
  is one self-contained value (the workspace graph's wish) and `load`
  refuses a session of another calculus, sequent or assumptions.
- **A text syntax for terms was needed** (`Calculus::parse_term`,
  `parse_sequent`): the command and the page take the sequent and the
  given values as text. It reads what `show` writes, by the calculus's
  names, symbols and marks, with ASCII spellings (`->`, `/\`, `|-`) only
  where the calculus declares the target and not the spelling; a
  shunting-yard over an explicit stack, so nothing recurses.
- **Assumptions are given at the start** (`--assume`), not discovered:
  the derivation's root-scope hypotheses must be among them, checked by
  `assume` and by the reader.
- **The command's words** follow the other two sessions: `goals`,
  `rules G`, `apply G RULE [N…] [NAME=TEXT…]`, `undo`, `show`,
  `proof [FILE]`, `save`, `load`, `help`, `quit`; new is `assume G`.
  There is no `close` (no search). Output: `opened goal N: 0: a ∧ b ⊢
  1: b ∧ a`, the hypotheses in force on the next line, `closed`, and
  `valid in the calculus given (NAME): …` for `proof`.

### The JSON (wire level 17)

`{"version": 17, "calculus": {…}, "terms": […], "scopes": [{"parent",
"owner": [node, premise], "hypotheses"}, …], "nodes": [{"sequent",
"rule", "hypothesis", "at", "premises", "scope", "opened"}, …],
"history": [goals]}`, the nodes from the conclusion up. Level 16 is
63t-a's; `wire::LEVEL` is 17 here, and the two chains meet in
`wire.rs`'s table and constants. The reader
(`Interactive::from_parts`) checks the tree, the scopes and owners,
where each goal was opened, the root-scope hypotheses against the
assumptions, the open goals' bounds, every closed node by `check_open`,
and the history's suffix property from the last step back.

### What was built

- `core/src/rules/interactive.rs` (feature `interactive`):
  `Interactive`, `Apply`, `Applicable`, `GoalId`.
- `core/src/rules/text.rs`: the text syntax.
- `check.rs`: `check_node` and `check_open` beside `check`.
- `print.rs`: `Calculus::show_zones` (a sequent given as zones);
  `calculus.rs`: `Calculus::marks`, the variables' and contexts' names.
- `Error::StepRefused` (invalid), `Error::StepNeeds` (malformed).
- `serialize/rules.rs`: the session's form and reader, the terms'
  writer and reader factored out.
- `cli/src/interact/rules.rs`; `--calculus` and `--assume` on
  `interact`; `calculus::named`, `sequent_of`, `lines`.
- Tests: `a_session_builds_a_derivation_the_checker_accepts` (LK, the
  listing, a split asked for, JSON round trip, undo, a refused step),
  `a_session_discharges_by_scope` (natural deduction with discharge,
  `→E` asking for `A`, assumptions, a hypothesis not in force refused),
  `a_session_builds_a_closed_tableau`; the manual's examples; a lock
  call (its own commit).
- Manual: `docs/src/rules.md` ("Build a derivation step by step"),
  `docs/src/theories.md` (new), `status.md`; the guide's reference; the
  changelog.

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 (not the writer's model), once, against
the brief's criterion: no operation yields a state the checker rejects.
Its random walks (apply from `rules`, random applies and given values,
assume, undo, JSON round trips, stale goal ids) on LK, LJ, LL, the
tableau, an ordered calculus and natural deduction found five faults,
all reproduced and fixed in `unqqwxlr`, each with a test
(`no_step_leaves_a_state_the_checker_refuses`,
`a_session_document_whose_scopes_do_not_fit_is_refused`):

1. **A refused step left orphan goals**: `commit` returned on a
   premise's shape (LM's right zone, one formula) after pushing the
   earlier premises, without rolling back; `goals` listed a goal the
   derivation did not hold, no `undo` removed it, and the session's own
   JSON was refused. Every premise's shape is now checked first.
2. **A name given as new, then assumed, finished into a refusal**:
   `assume` re-checked only the discharging rule, but a rule's `fresh`
   condition reads the hypotheses in force, which the new one joins.
   `assume` now re-checks every closed node (`check_open`).
3. **A document with a scope owned by an open goal passed the reader**,
   and a step, an `undo` and an `assume` then panicked on a removed
   scope. **4. A hypothesis in a sibling's scope passed it too**, and
   `undo` left it in a removed scope. The reader now asks each scope's
   owner to be a rule node whose discharging premise was opened in it,
   inside that node's scope, and each hypothesis to lie in the scope it
   was opened in or one around it.
5. **A sequence premise's member found by position was located by
   value**, the first equal member, so `a, a • b ⊢ c` could not be
   taken apart by `•L` (a completeness gap). Members are now located at
   the place they were built.

It found sound: the text parser (no recursion on 200 000 nested
brackets, no panic on 20 000 junk strings per shipped calculus, every
one of 4 000 random terms and 500 sequents read back as `show` wrote
it), every finished state of its walks passing the checker, the JSON
round trips, and the manual's and the lock's outputs.

### Checks

All on cores 10-12, capped at 8 GB.

- `cargo test -p deduxit --test rules`: 18 passed at the last code
  commit (`unqqwxlr`), the five session tests and the two of the
  review's fixes among them.
- `cargo test -p deduxit-cli --test readme --test lock`: 7 and 1
  passed; the manual's new examples run there. The lock's new call and
  `help-interact` were blessed in their own commit (`kysuslqo`).
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean;
  both `cargo hack` runs (each feature, every pair) clean after the
  text syntax and the session were put behind `parse` and
  `interactive` (the first gate run found `ParseError` absent without
  `parse`).
- `gate`: passed at `unqqwxlr`.
- `cargo doc` of the library and the command, private items included,
  with warnings denied: clean after one redundant link was fixed (the
  flake's `doc-private` found it).
- `nix flake check`, under the machine lock: passed at `unqqwxlr`, on its
  second run; the first failed on the redundant doc link above.
- After the rebase onto 63t-a's chain (`zxwmkkxu`): `cargo test` of the
  rules, serialize and stability tests and of the command's readme and
  lock tests passed, then the gate and `nix flake check` again, both
  passed.

### Meeting 63t-a

The chain was rebased onto 63t-a's (level 16, `wire::TRANSFORMS`).
`wire::LEVEL` is 17; `core/tests/rules.rs` keeps main's line for the
rule file's level and `web/tests/ordinary.rs` asserts 17. Main's table
of levels in `core/src/wire.rs` had no row for 16; this chain adds one
from what `core-sequents.md` says it holds (written only: negative
translations compared and interpolations), so the table runs 15, 16,
17. The changelog keeps the planning session's wording of the highest
level (stated once).

### Departures and assumptions

- **`Calculus::positions` was not extended with a list** of what each
  position stands for (the prompt's "add the list"): `rules` lists, per
  applicable rule, the member numbers to pass, which is what a front
  end needs (departs from the design).
- **No drawing** of a session's derivation: the page's tree drawing is
  61b's (the exports' `Drawable` over `rules::Derivation`).

### What the next sessions must know (61b: the bindings, the drawing, the page)

- **The bindings** take a session as JSON (level 17) in and out, as the
  linear and ordinary sessions do; nothing is kept between calls. The
  calls map onto `Interactive::{within, rules, apply, assume, undo,
  derivation, finished}`; `Apply`'s `at` are member numbers across the
  zones (`Interactive::members`), `given` name and text pairs.
  `Error::StepNeeds` carries the names to ask the user for.
- **The page**: a calculus is one more suite; its first screen says in
  a sentence what a rule file is, with LJ or the tableau to start from
  (the author's "not overwhelming"). The rule file travels inside the
  session's JSON, so the link and the autosave hold it.
- **The drawing** is the missing piece: `rules::Derivation` is a DAG in
  general but a session's derivation is a tree (every node one parent),
  so the exports' tree layout fits it once a `Drawable` is written over
  it; 62 adds the Fitch list.
- **`rules` cost**: it clones the session per choice of positions and
  rule; for large sequents in a calculus with many two-position rules
  (the tableau's `×` is quadratic in the branch) a front end should ask
  it for one goal at a time, as the command does.
