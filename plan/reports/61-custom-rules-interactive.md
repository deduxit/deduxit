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

## Session 61b: the bindings, the drawing and the page

Opus 5.5 at high, cores 10-12, the reviewer on Fable 5.2. The prompt is
`plan/61-custom-rules-interactive.md`, its second session, from the brief
`start-61b.md`.

### Outcome

A visitor can now build a derivation in a calculus given as a rule file
on the page: LJ or the tableau to start from, or a rule file of their
own, each step checked by deduxit before the session takes it, the
derivation drawn as LK's is, and the finished one checked whole, "valid
in the calculus given". The page shows nothing of it until the
calculus popover's one button "Your own calculus" is pressed.

### The drawing

- **`rules::Drawing::new(&calculus, &derivation, &limits)`** lays a
  derivation out for the exports, and `Drawable::Rules` takes it
  (feature `serialize`), so `svg`, `latex` and `typst` draw it as they
  draw LK's: each node under its rule's name, the formulas in the rule
  file's print forms, an open goal as the style draws one, a hypothesis
  as a leaf labelled `hyp`. The SVG has its `<title>` (the conclusion)
  and its `<desc>` (the numbered steps, "by ∧R"), as every tree's.
- **`Drawn` became per value**: the rule count, the open rule and the
  labels were constants of the type (`RULES`, `OPEN`, `markup` without
  `self`), which a calculus known only at run time cannot give; they
  are methods now (`rules()`, `open()`, `markup(&self, …)`), the label
  tables still built once per drawing, so nothing per inference changed
  (the ratchet's counts stayed within their ceilings).
- **A derivation is a DAG in general**: `Drawing::new` unfolds it from
  its last node into a tree, a shared node drawn once per use, and
  refuses past `limits.derivation_bytes` by an estimate counted as it
  unfolds, so a derivation whose sharing would unfold exponentially is
  refused before anything is laid out. A session's derivation is a tree,
  numbered alike in `Interactive::derived` and in the drawing, which
  `Interactive::derivation_goals` (new) and the page's view rely on.
- **Formulas in each notation**: the printer of rule files
  (`rules/print.rs`) takes a `Spelling`. A connective whose symbol a
  notation knows (the linear and ordinary symbols, `⊢`) takes that
  notation's spelling (`\land`, `\vdash`), any other symbol is set
  upright (`\mathrm{T}`, `upright("T") `), atoms as the notation's atoms
  (italic letters in SVG). Each sequent is cut at `Shown::LIMIT`
  characters a whole piece at a time, so no LaTeX command is cut in two.
  pdfLaTeX knows only the characters `escape` spells; `×`, the
  tableau's closing rule, was added. Snapshots `rules.*` (LJ) and
  `rules-tableau.*`, which the `export` check compiles.
- Rocq refuses such a derivation (`Unsupported::Calculus`) and it has no
  lambda-term (`NoTerm::Calculus`).

### The bindings

`web/src/rules.rs`, exported with the prefix `rules_`: `calculus` (a
built-in rule file's text), `read` (a rule file checked and described),
`start` (calculus, goal, assumptions), `view` (a saved session read back,
every step checked again: the load), `rules`, `apply` (`{"rule", "at",
"given"}`), `assume`, `undo`, `check` (the finished derivation, "valid in
the calculus given", and its document), `draw` and `export` (LaTeX,
Typst, SVG). The session's document holds the rule file, so it is the
whole state, which is what the save is. The view gives each open goal as
text, its zones' members in the order of their numbers, the hypotheses
in force, the root sequent and the assumptions, and `inferences`, the
session node of each drawn `i<n>`. `about` lists the suite `rules` with
the parameter `calculus` and `term` `null`. `rules::GoalId` serializes
as its number, as the linear one does.

The module grew to 3 111 006 bytes at gzip -9, past the ceiling of
3 000 000: the bindings now link the rule-file reader, checker, text
syntax and session. The ceiling is 3 300 000, in a commit of its own
that says so.

### The page

In the client (`deduxit.github.io`):
- **A suite of the registry** (`rules`, parameter `calculus`, the rule
  file's name), with no segment on the calculus's segmented control:
  one button in the calculus popover, "Your own calculus", opens it. A
  visitor who never presses it sees the popover's one extra button and
  nothing else (the author's "not overwhelming").
- **The panel** (`src/islands/Custom.tsx`) stands in the stage's place,
  as the truth table does. Its first screen says in one sentence what a
  rule file is, starts from LJ or the tableau, and keeps the file and
  the assumptions in two closed parts under it; the goal is typed in the
  sequent field, its example in the placeholder. Once started: the
  derivation drawn by deduxit, the open goals, the steps deduxit offers
  for the goal chosen (one button per text, "∧R on b ∧ a"), a step that
  needs values asking for each by name, "Close by a hypothesis", the
  bottom band's Undo, and the check of the finished derivation:
  "Valid in the calculus given: every step follows a rule of LJ.
  Nothing says the rules themselves are sound." After a step the panel
  moves to the goal the step opened, after an undo to the goal it
  reopened.
- **The state** keeps a custom proof's work in `Proof.custom` (the file
  as typed, the assumptions, the session; never stored the view, the
  verdict, the problem and the goal), never in `Proof.session`, so no
  linear or ordinary call ever gets such a session and every path that
  reads `session` sees the proof as not started.
- **The link and the autosave keep the rule file**: the session holds
  it, and the link writes it (`u=rules&m=NAME&d=…`), the shelf too
  (`StoredProof.custom`); both are read back through `rules_view`, which
  checks every step again. Nothing is sent anywhere: the calls run in
  the page's worker.
- The education skills used as references: cognitive load (one new thing
  per screen: the file and the assumptions stay closed until wanted; the
  goal is typed where every other goal is), and the feedback principles
  (a refused step shows deduxit's reason under the steps, and a step
  that needs values names each).

### Checks

All on cores 10-12, every nix build under the machine lock.

| check | where | result |
|---|---|---|
| the gate (clippy, tests, both `cargo hack` runs, the ratchet) | deduxit at `tzmrqoux`, rebased onto 62a | passed (also at `wzqmmxvk` and `uqktymvt` before the rebase) |
| the bindings' tests natively | `cargo test -p deduxit-web --test rules --test ordinary` | 4 and 10 passed |
| the bindings' tests under Node | `checks.web-wasm-tests` (in `nix flake check`) | passed; `tests/rules.rs` 4 passed under Node at `wzqmmxvk` |
| the export snapshots compiled | `checks.export` | passed (`rules.*`, `rules-tableau.*` by pdfLaTeX, Typst, resvg) |
| `nix flake check` | deduxit at `sxwwwyql`, the report's commit | passed (also at `wzqmmxvk`) |
| the client's unit suite (`tests`) | client at `uqqwsrmr` | 617 passed |
| the client's lint (`lint`: Biome, `astro check`) | client at `uqqwsrmr` | clean |
| the client's e2e suite (`e2e`, desktop and phone, axe in light and dark) | client at `uqqwsrmr` | 260 passed |
| the client's `nix flake check` | client at `uqqwsrmr` | passed |

The client was built against the deduxit chain with a local override,
its committed lock untouched: `nix develop --override-input deduxit
git+file:///home/tux/Projects/own/deduxit-wt-step-58b-8c0bbc -c …`
for the dev shell's `tests`, `lint` and `e2e`, and `nix flake check
--override-input deduxit git+file:///…` (a `git+file` input, so only
tracked files go to the store). The client's `flake.lock` moves to
deduxit's `main` once the planning session has pushed this chain.

### Commits

deduxit, on `main@origin` (`ntlzslyx`, 62a's review):

| change | what |
|---|---|
| `zqztkpmt` | Draw a derivation in a calculus given as data |
| `zlwrnprn` | Build a derivation in a calculus given as data in the web bindings |
| `wzqmmxvk` | Raise the web module's ceiling for calculi given as data |
| `uqktymvt` | Keep drawings of rule files well formed (the review's findings 1 to 4) |
| `tzmrqoux` | Offer natural deduction's rule files in the web bindings |
| `sxwwwyql` | Report of session 61b |

The client, on its `main@origin` (`uxuxvlps`, 63t-b):

| change | what |
|---|---|
| `sutqntom` | Build a derivation in a calculus of one's own |
| `uqqwsrmr` | Leave out a stored custom proof without its rule file (finding 5) |

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 (medium), once, with the page served from a
build and driven by Playwright. The central criterion held: 19 200
random apply, assume and undo operations over LJ, LK, LM, the tableau and
a natural-deduction file with discharges, and 60 000 mutated session
documents (58 accepted, each of those sound down its undo chain), never
gave a session `rules_view` or `rules_check` refused; forged links and
stored proofs were refused with deduxit's reason; a derivation whose
sharing unfolds to 2^60 inferences was refused in about 3 ms. Five
findings, all fixed:

1. **A sequent cut at the printer's bound left LaTeX and Typst groups
   open** (`\mathit{ab\dots`, `italic("abc dots`), which pdfLaTeX and
   Typst refuse. Fixed: an atom or a symbol is written as one piece, a
   cut sequent closes the brackets and member marks it opened, and a
   bracket's closer is pushed only once its opener was written
   (`uqktymvt`; `rules_drawings_stay_well_formed`, 800 nested `g(…)` and
   a 5000-letter atom, balanced in both).
2. **`_` and `*` in a rule's name were read as label markup**: `_|_E`
   gave a double subscript (LaTeX and Typst refuse it) and an SVG
   element with `font-size` twice. Fixed: `style::LITERAL` keeps a
   character as text, and a rule file's names write `_` and `*` so
   (`uqktymvt`).
3. **A control character U+0002 or U+0003 in a symbol panicked the SVG
   drawing** (`unreachable` in the module, no drawing on the page).
   Fixed: the rule-file reader refuses a control character in the
   calculus's name, a connective's name or symbol, a zone's name or mark
   and a rule's name (`uqktymvt`).
4. **`Drawing`'s doc comment** claimed inference `n` is node `n` for
   every tree; it holds only where the nodes are stored in the
   drawing's order, as a session's are. Reworded (`uqktymvt`).
5. **A stored proof whose `custom` record lacks its file threw on
   restore** (client). Fixed: the shelf's reader leaves such a proof out,
   as it does any proof it cannot read (`uqqwsrmr` in the client).

Not tested by the reviewer: a link of a file of one's own past 2000
characters (its own padded file compressed below the limit).

### Departures and assumptions

- **A hypothesis is drawn as a leaf labelled `hyp`** with an inference
  line, not as a bare formula, so that the drawings need no new kind of
  leaf; a Gentzen-style tree of natural deduction draws hypotheses bare
  and marks the discharged ones (departs from the design).
- **The calculus popover shows one more button**, "Your own calculus",
  to every visitor who opens it; nothing else of the suite shows until
  it is pressed. The brief's "sees nothing new" is read as "nothing on
  the page itself" (departs from the design).
- **Undo in a custom proof is the last step**, not the active branch's:
  a session in a calculus given as data keeps the steps' order, and its
  undo is the library's (departs from the design).
- **The panel offers LJ and the tableau to start from**; LK, LM and LL
  are read by name from a link or a stored proof, and their files are
  one edit away. **`rules_export` has no text format**: `rules_check`
  answers the finished derivation's document.
- **A link without its session** (past 2000 characters) restores a
  built-in calculus's goal, not a file of one's own: the file travels in
  the session (web.md says so).
- A user's `Labels::Table` keys the linear rules and does not touch a
  rule file's labels; `Labels::Off` turns them off.

### What the next sessions must know (62b to 62d: natural deduction's builder, drawings and page)

- **62a landed while this session ran**; the chain is rebased onto it.
  The bindings' built-in names are now the command's thirteen
  (`rules_calculus("NJ")` and the rest, `about`'s `values`), as 62a's
  report asked of 61b; the page still offers LJ and the tableau to start
  from, so a third button for NJ is 62d's choice.

- **The drawing** is `rules::Drawing` over the exports' generic tree
  layout; `Drawn` now gives its rule count, open rule and labels per
  value, so a calculus known at run time draws. A tree of natural
  deduction wants hypotheses bare and discharged ones bracketed with
  their mark (`[A]¹`), and the rule's label carrying the mark: that is a
  kind of leaf the trait does not have (today a hypothesis is a rule
  `hyp`); add it to `Drawn` (default off) and to the four layouts, or
  draw natural deduction with a layout of its own.
- **The Fitch view is a list, not a tree**: a derivation's scopes are
  its boxes (`Derivation::scope_parents`, each scope's owner node and
  premise in the session), the nodes in postorder are its lines. Number
  the lines as `derived` numbers nodes, so that the page's
  `view.inferences` (`Interactive::derivation_goals`) maps a line to its
  goal as it maps `i<n>`.
- **The page**: the custom suite is `rules` in the client's registry,
  its panel `src/islands/Custom.tsx`, its state `Proof.custom`; a Fitch
  view would be a second view of the panel's drawing (as Table is of the
  stage's), its first screen staying one sentence. Each new state joins
  `customising` in `tests/states.ts` (the walk is about 4 s per scheme
  and width now).
- **The module** is 3 111 006 bytes at gzip -9 under a ceiling of
  3 300 000; a layout of its own for natural deduction should stay
  within it.
- **Printing**: `rules::print::Spelling` writes a rule file's terms in
  any export's notation; a new symbol a notation should spell itself
  joins `Spelling::symbol`'s table.
