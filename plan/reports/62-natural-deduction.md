# Step 62: natural deduction, in Gentzen's trees and in Fitch's style

## Session 62a: the rule sets and the checker's use

Session 62a, 2026-11-15, Opus 5.5 at `high`. Brief:
`/tmp/linlog-handoff/start-62a.md`; prompt: `plan/62-natural-deduction.md`.

### Outcome

Natural deduction for classical, intuitionistic and minimal
propositional logic ships as eight built-in rule files on step 60's
checker, each naming its sources. `check --calculus NJ` and `interact
--calculus NJ` take them by name, and a derivation that names one needs
no flag. A test shows, both ways, that each file derives exactly what
the ordinary suite calls valid in its logic: on fixed and generated
sequents, and on ILTP's small problems. The manual has a chapter, and
the guide, the status, the changelog and the rules files say so.

The chain on `main@origin` (`zzyruvqs`, rebased from the plan commit
of 62a, `xwyzmsnn`):

| change | what |
|---|---|
| `unpppxmx` | the rule files `core/src/rules/calculi/n*.json`, their constants `rules::NJ` … `rules::NM_DEFINED`, the cross-check `core/tests/natural.rs`, `DNE` in `typos.toml` |
| `mvpuyyuy` | the command: `BUILT_IN` and the two `--calculus` helps |
| `xnlqmxno` | the behaviour lock: `check-natural-nk`, `check-natural-nj`, and the two helps |
| `vuomwvyo` | the manual's chapter `natural-deduction.md`, `rules.md`'s list, the guide's reference, `status.md`, the changelog |
| `syprlmwk` | the rules files: `core-rules.md` (its paths gain `core/tests/natural.rs`), `core.md`, `cli.md`, `claude-infra.md` |

### The rule sets and their sources

A judgement is one formula (one multiset zone of exactly one member, so
no rule needs a position); a hypothesis is a node of its own, and
discharge is by scope, as 60b's chapter's NJ already did. Every
elimination takes its major premise first (`→E`: `A → B`, then `A`;
`¬E`: `¬A`, then `A`).

| file | logic | how it differs | source in its `description` |
|---|---|---|---|
| `NJ` | intuitionistic | `⊤I`, `∧I/E₁/E₂`, `∨I₁/I₂/E`, `→I/E`, `¬I/E`, `↔I/E₁/E₂`, ex falso `⊥E` | Gentzen 1935; `↔` from forall x: Calgary (Magnus, Button and others, 2019) |
| `NK` | classical | NJ and `RAA` (`⊥` from `¬A` gives `A`, discharging `¬A`) | Prawitz 1965 (`⊥c`) |
| `NM` | minimal | NJ without `⊥E`: `⊥` has no rule | Johansson 1937 |
| `NK-DNE` | classical | NJ and `¬¬E` | Huth and Ryan 2004 |
| `NK-LEM` | classical | NJ and `LEM` (`A ∨ ¬A` from nothing) | Gentzen 1935 (tertium non datur) |
| `NJ-defined`, `NK-defined`, `NM-defined` | the three | no `¬`: write `¬A` as `A → ⊥`; `RAA` discharges `A → ⊥` | Prawitz 1965, Troelstra and Schwichtenberg 2000 |

- **"`¬` defined" is a file without `¬`**: the rule language has no
  definitions, so the faithful form is a calculus in which `¬A` cannot
  be written and `A → ⊥` is. A definition key would raise the wire
  level and was not asked for.
- **"`⊥` as an atom in NM"** is NM's `⊥`: a constant with no rule
  behaves as an atom, as in the ordinary suite's minimal logic.
- **`↔` has rules** (not Gentzen's, who had none for it): ILTP's
  problems and the ordinary syntax have `↔`, and forall x's `↔I` (two
  subproofs) and `↔E` are what a course meets.
- The files were written once by a script and are kept as data, as
  60a's LK was; they validate against `rules::SCHEMA`
  (`check-jsonschema`, by hand, under the machine lock).

### The cross-check: method and numbers

`core/tests/natural.rs`, five tests. "A derivation exists in the file
exactly where the ordinary suite says valid" is shown both ways.

- **Every accepted derivation is valid**
  (`every_rule_is_valid_in_its_logic_and_no_weaker_one`). Each rule is
  read as the sequent `⋀(discharged → premise) ⊢ conclusion`, with its
  variables as atoms. It is decided in all three logics, and must be
  valid exactly in those at least as strong as the rule needs:
  classical for `RAA`, `¬¬E` and `LEM`, intuitionistic for `⊥E`,
  minimal for the rest. The three logics are closed under substitution,
  and the scope invariant of step 60's checker gives each premise from
  the hypotheses in force. So whatever the checker accepts in a file is
  valid in its logic. Each file also has `⊥E` exactly when it is not
  minimal, and a classical rule exactly when it is classical.
- **Every valid sequent of the samples has a derivation**
  (`derivations_of_the_ordinary_suite_pass_in_natural_deduction`,
  `generated_sequents_pass_where_they_are_valid`, and the ignored
  `iltp_s_small_problems_pass_in_natural_deduction`):
  - The ordinary suite's derivation of each valid sequent is translated
    into every file whose logic is at least as strong, by the
    embedding, LJT and the tableau.
  - The translation must pass the checker, its conclusion must be the
    sequent's formula (the disjunction of several formulas right of
    `⊢`, or `⊥` where there is none), and its assumptions must be
    hypotheses of the sequent.
  - The translator is in the test, as 60b's `from_linear` was, since
    the prompt keeps the translation from LK and LJ for later. Open
    leaves are named by formula, so a left rule substitutes a
    derivation for every open leaf of a formula and no occurrence is
    tracked. LK becomes `⊥` from `Γ, ¬Δ`, closed by the file's
    classical rule.
- **Samples of the first direction**: each derivation of the fixed list
  of at most 40 nodes is broken once per node (a formula, a rule, the
  premises' order, the scope), and every break the checker accepts is
  decided by the ordinary suite, which must call it valid.
- **A weaker file refuses for the rule**
  (`a_weaker_file_refuses_what_needs_a_stronger_rule`): proofs needing
  `RAA`, `¬¬E` or `⊥E`, renamed into NJ or NM, are refused naming that
  rule.

The numbers:

| run | derivations checked per file | left out for size | breaks accepted (each shown valid) | breaks refused |
|---|---|---|---|---|
| 22 fixed sequents, each logic | 24 (NM) to 96 (the classical files) | 0 | 6 996 | 63 551 |
| 300 generated sequents, each logic (343 valid, 557 not valid) | 184 (NM) to 686 (classical) | 0 | not run | not run |
| ILTP: SYJ, LCL, SYN, derivations of at most 2 000 inferences | 93 (NM) to 285 (classical), 1 707 in all | 108 (past 50 000 nodes unfolded) | not run | not run |

- **The ILTP run** found 301 derivations, counted per method: 103
  intuitionistic, 100 minimal, 98 classical. It took 1 090 s, release
  build, cores 7-9, reading the library from the store.
- **The tableau method is left out of the ILTP run.** On
  SYJ202+1.004 (the pigeonhole principle) it held several times its
  memory bound: through the command, `--method tableau --memory-limit
  512MiB` was killed at a 3 GB cap, while `64MiB` and `256MiB` refused
  properly. That is 56a's code, not this step's (follow-up below). The
  fixed and generated sequents still use the tableau.
- **The ILTP loop runs on a thread with a 512 MiB stack**: the
  embedding's search and the test's recursive translator overflowed
  the test thread's stack.

### The command

- `check --calculus NAME`, `interact --calculus NAME` and a
  derivation's own `calculus` take the eight names in any case
  (`cli/src/calculus.rs`, `BUILT_IN`). Both `--calculus` helps list
  them.
- The verdict names the rule set: `valid in the calculus given (NK): a,
  a derivation of 4 nodes from 1 assumption`. A proof using a rule its
  file lacks is `invalid … (RAA, concluding a): the calculus has no
  rule of this name`, exit 1.
- **The chapter** `docs/src/natural-deduction.md`, after `rules.md`:
  check a proof (∨E, two subproofs), choose the rule set (a table of
  the eight), NJ's rules (a table), a refusal, and building step by
  step with `interact --calculus NJ`, which works as it is. Every
  example runs in `cli/tests/readme.rs`.

### Checks

All on cores 7-9, capped at 8 GB.

- `cargo test -p deduxit --test natural`: four passed, one ignored. The
  ILTP part, release, `ILTP=` the store's library: passed.
- `cargo test -p deduxit-cli --test lock` (blessed in its own commit,
  `xnlqmxno`) and `--test readme`: passed.
- `check-jsonschema` on the eight files: valid.
- The gate: passed at the chain before the rebase. The rebase onto
  `zzyruvqs` brought plan files alone, and the one change after the
  gate renames the test's enum (below): Clippy and the test ran again
  on it. The gate noted that the cache misses of `render-svg` moved
  (+9.65 %). Nothing it renders was touched.
- `nix flake check` at the last commit, under the machine lock: passed
  on its second run. The first failed in `typos`, which read `DNE` as
  `DONE` and the test's enum `Nd` as `And`. `typos.toml` now takes
  `DNE` as a word, the enum is `Tree`, and both are folded into
  `unpppxmx`.

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 at `medium`, once, on the five changes,
briefed above all on the discharge and scope handling.

- **What it tried**: about 35 forged derivations, each refused for the
  right reason:
  - a hypothesis used outside its subproof;
  - a root inside a scope;
  - `→I`, `¬I` and `RAA` discharging the wrong formula, NK-defined's
    `A → ⊥` included;
  - a partial discharge;
  - a premise in a sibling scope;
  - a discharge from a grandchild scope;
  - `∨E` and `↔I` with their subproofs swapped, or both in one scope;
  - a judgement of zero or two formulas.
- **What it found sound**: the accepted oddities (a vacuous discharge,
  two `→I` sharing a scope) show valid sequents. Every file's
  discharges match the textbook rule, and each file is exactly as
  strong as its logic. The rule-level argument holds. The chapter's
  examples reproduce exactly.
- **One finding, fixed** (folded into `vuomwvyo`): the chapter
  introduced NJ's rules table as "the rules every file has", false for
  `⊥E` in NM and `¬I`/`¬E` in the `-defined` files. It now says "the
  rules of NJ", and the paragraph after it names the differences.
- **A question, answered**: NJ's description cited forall x: Calgary
  as "(Zach, 2019)". The book is Magnus and Button's with further
  contributors, so it now reads "(Magnus, Button and others, 2019)",
  folded into `unpppxmx`.

### Departures and decisions

- **The variants' names** (`NK-DNE`, `NK-LEM`, `NJ-defined`,
  `NK-defined`, `NM-defined`) are this session's choice. A built-in
  name is matched in any case, and ASCII keeps them easy to type.
- **"`¬` defined" is a file without `¬`**, not a definition the
  checker unfolds (above). A course that wants `¬A` typed and read as
  `A → ⊥` needs a definition key in the rule language: a wire-level
  change, left for the author to ask for.
- **The ILTP part leaves out the tableau method** (above). The
  prompt's cross-check is on "ILTP's small problems", which the
  embedding and LJT cover in all three logics.

None departs from the design as the prompt and the brief state it.

### What the next sessions must know

- **62b (the builder)**:
  - `interact --calculus NJ` already builds proofs backwards: the
    chapter's last example. `∨E` must be given `A=` and `B=`, `→E` its
    `A=`.
  - Forward steps, Fitch's habit, are missing: deriving `a` from a line
    `a ∧ b` in force means a backward `∧E₁` with `B=b`, then `assume`.
    A forward step adds a line in force, which `rules::Interactive` has
    no notion of beyond hypotheses. That is the addition the prompt
    foresaw.
  - `rules.md`'s own example rule file is named `NJ`. With `--calculus
    nj.json` it still wins, but a derivation naming `NJ` without the
    flag now reaches the built-in one. Rename the example if that
    confuses.
- **62c (drawings)**:
  - A derivation's node order need not be Fitch's. The test's
    translator writes nodes in post-order, a hypothesis where it is
    used, so a subproof's lines are not contiguous and its assumption
    is not first.
  - The Fitch view reorders, or checks the layout rules where it draws
    (60b's note).
  - A node may be the premise of several nodes: the derivation is a
    DAG, and a Gentzen tree unfolds it.
- **62d (the page)**:
  - The library has the files as `rules::NJ` … `rules::NM_DEFINED`, and
    the command's names are in `cli/src/calculus.rs`.
  - 61b's bindings will need the same names (a list of their own, or a
    shared one if the author wants a `rules::BUILT_IN` in the library;
    not added, since no caller asked).
  - The ordinary tab offers the method, never forced.
- **Follow-up, not this step's**: the tableau method undercounts its
  memory on large pigeonhole problems (`prove --logic classical
  --method tableau --memory-limit 512MiB --file SYJ202+1.004.p` passes
  3 GB). For 56a's area (`ordinary/tableau.rs`, `GOAL_BYTES`).

## Session 62b: the builder

Session 62b, 2026-11-16, Opus 5.5 at `high`, the conversation of 62a.
Brief: `/tmp/linlog-handoff/start-62b.md`.

### Outcome

A session in a calculus given as data now builds a proof forwards as
well as backwards, as a Fitch-style proof is written. `derive G RULE
L…` applies a rule to lines in force and adds its conclusion as a new
line, which later steps cite by its number. Backward steps (`apply`)
are unchanged, and a rule that discharges a hypothesis opens its
subproof that way. `assume G [L]` closes a goal by line L, or by the
line in force equal to it. Every step is still checked by step 60's
checker, the session's JSON reads every step again, and a session with
a derived line is written at wire level 18. The manual's natural
deduction chapter shows both directions with real sessions, and
`rules.md`'s example rule file is now `my-NJ`.

The chain on `main@origin` (`poolokwq`, rebased from `stlokosv`):

| change | what |
|---|---|
| `mxxquknp` | the library (`Interactive::derive`, `lines`, `cite`, `Derive`, `LineId`, `Undone`; the reader; level 18), its tests, the command's words, the bindings' undo |
| `wsssxwow` | the behaviour lock: `interact-calculus-forward` |
| `lppusyxq` | the manual (natural deduction's session section, `rules.md`'s `my-NJ` and its `lines:`), the guide, the status, the changelog |
| `lqrkvzwv` | the rules files: `core-rules.md`, `cli.md`, `web.md` |

### The forward step's design and its sources

The research sub-agent read the tools' own pages (Sonnet 5.5, `medium`);
ProofWeb's site and manual could not be reached, so nothing here rests
on it.

- **Jape** (Bornat, "Natural Deduction Proof and Disproof in Jape", the
  manual at cs.ox.ac.uk/people/bernard.sufrin/personal/jape.org):
  forward and backward steps in box proofs ("for a forward step you
  must choose a hypothesis and for a backward step an open (unproved)
  conclusion"). An unproved conclusion is a line below three dots, and
  rules open and discharge boxes themselves ("combining introduction
  and discharge into a single step… never a need to guess an
  assumption"). A step reaches only lines above it in its own box or
  in enclosing boxes. This is the design taken: forward and backward
  steps in one session, open goals as Jape's unproved conclusions, and
  subproofs opened by applying a discharging rule backwards.
- **forall x: Calgary** (chapters 17 and 18 at
  forallx.openlogicproject.org): lines are cited by number and a
  subproof by its range. A line may be cited where it comes before and
  lies in no closed subproof, outer lines inside an inner subproof
  included, so reiteration is never needed. The session's lines in
  force are exactly these: a line is in force at a goal when its scope
  encloses the goal's, and a citation of any other is refused.
- **Carnap** (carnap.github.io/Carnap-Manual/forallx.html): a
  justification is a rule followed by line numbers and ranges, and a
  subproof begins with an assumption line. Its manual says nothing of
  goals or of building backwards.
- **Lean 4** (the tactic reference): `assumption` "tries to solve the
  main goal using a hypothesis of compatible type". The session's
  `assume` likewise closes a goal by any line in force, derived lines
  included. That `have`'s facts count is the sub-agent's inference
  from the reference, not a sentence of it.

The choices that follow from these:
- **A line is a node, shared.** A derived line is one node however
  often it is cited (`Goal::line`), so the session and its derivation
  are a DAG, which step 60's checker and 61b's drawing already take.
  - `derived` emits the lines first, then the tree; a goal closed by a
    line (`Done::Line`) is that line's node.
  - A cited hypothesis becomes a hypothesis node of its own per
    citation, lying in the hypothesis's scope.
  - `verify_all` checks every closed node after each forward step,
    since a new hypothesis node joins a scope whose owner checks it.
- **Line numbers are stable and stored nowhere.** Lines are numbered
  from 1 in the order the steps made them (`line_places`): the
  assumptions, then each scope's hypotheses keyed by the premise node
  that opened the scope, and each derived line keyed by its node. Node
  places only grow, so a number never changes and the JSON needs no
  table of them.
- **What a forward step refuses.**
  - **A discharging rule** applies backwards, which opens its subproof,
    as Jape does: "→I discharges hypotheses: apply it backwards to a
    goal, which opens its subproof".
  - **A rule with contexts** applies backwards too, since the lines
    cannot tell the contexts apart. Natural deduction has none; LK's
    rules are refused forwards.
  - **What the lines leave open** is asked for by name, as backwards:
    `∨I₁` needs `B=…`.
- **Fitch's subproof citation by range (`→I 2–6`) has no forward
  form.** It is the backward `→I` in this session. A forward form would
  need a subproof opened by an assumption that no rule owns yet, which
  a scope of the session cannot be. That is the brief's "a subproof is
  opened by an assumption and closed by the rule that discharges it" in
  Jape's form, and 62c's Fitch view can still draw it as a range.

### The command

- **Session words.** `derive G RULE L… [NAME=TEXT…]` answers `line N:
  FORMULA`. `assume G [L]` closes by line L (the library's `cite`) or
  by the line equal to the goal (`assume`). `undo` answers `reopened
  goal …` or `removed line N`.
- **Goal lines.** Each goal now lists `lines: 1. a → b; 2. b → c` in
  place of 61a's `hypotheses: …`. These are the lines in force, in the
  order of their numbers, where 61a listed the innermost first.
- **Errors** name the line: `there is no line 9`, `line 2 is not goal
  0`, `line N is not in force at goal G: it lies in a subproof that is
  closed there`.
- **The library's `undo`** answers `rules::Undone` (`Reopened(goal)`
  or `Removed(line)`); `rules::Interactive` has not been released, so
  nothing released breaks. The bindings' `rules_undo` answers
  `reopened: null` for a forward step.

### The JSON

The session reader of level 17 ignores keys it does not know, so a
derived line could not be a field an old reader ignores: it would read
a line node as an orphan and a goal closed by one as open. A session
that holds a derived line is therefore written at level 18
(`wire::FORWARD`, `wire::LEVEL` 18): a node's `"line": true` and a
goal's `"cites": N`. One without stays at level 17, byte for byte.
The reader checks:
- a line is no root, does not discharge, and is cited only by later
  lines;
- a goal closed by a line equals it and has it in force;
- the history: a forward step's nodes are the last it made, and a goal
  closed by a line comes after the line's step.

### Checks

All on cores 7-9, capped at 8 GB.

- `cargo test -p deduxit --test rules`: 20 passed.
  - `random_forward_and_backward_steps_keep_a_checked_session`: seven
    sessions in NJ and NK, 40 walks of 30 random steps each. It reads
    every state back through `wire::upgrade`, which checks every step,
    and compares the JSON.
  - `a_session_derives_forwards_and_cites_its_lines`: a forward proof,
    level 18, the refusals, scope, and forged documents.
- `cargo test -p deduxit-cli --test readme --test lock`: passed. The
  lock's new call `interact-calculus-forward` was blessed in its own
  commit.
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- The gate passed at `lqrkvzwv`. It noted that the cache misses of
  `render-svg` moved (+8.77 %); nothing it renders was touched. The one
  change after it is the rustdoc of `Interactive` (the reviewer's
  finding), built with `cargo doc`.
- `nix flake check` under the machine lock: passed on `snqrmoss`, and
  again after the rebase onto `poolokwq` (54x5's tableau memory
  account), the bindings' tests under Node included (`about.level`
  18).

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 at `medium`, once, on the four changes.

- **Criteria 1 to 4 hold.**
  - Its own random test ran 1 200 sessions of up to 40 steps (`apply`,
    `derive`, `assume`, `cite`, `undo`, undo from finished states) in
    NJ, NK, LK, the tableau and a calculus of its own with `fresh`,
    `atomic` and a discharging rule. Totals: 12 122 steps, 1 402 derived
    lines, 301 finished states.
  - Every state read back to the same JSON, and every finished state
    passed `finished`.
- **Refused as they should be**:
  - lines of closed or sibling subproofs, line 0 and lines past the end
    for both `cite` and `derive`, and `assume G L` with a line that is
    not the goal;
  - about twenty forged level-18 documents: a goal citing a sibling's
    line, a line with `→I`, a line citing a later line, a tree node
    citing a line, moved scopes, reused hypothesis nodes, a misplaced
    `line` mark, version 19.
- **Stable numbers.** A line's number held across steps, undo, save and
  load. The chapter's sessions printed exactly what it shows.
- **One finding, fixed** (folded into `mxxquknp`): `Interactive`'s
  rustdoc, the library's manual for its JSON form, still said level 17
  and named neither the keys `line` and `cites`, nor `derive`, `lines`
  and `cite`. It now does.
- **Its observation, no break**: a level-18 document relabelled 17
  still reads, checked in full. A level-17 reader is this build's own,
  and the keys mean the same.

### Departures and decisions

- **`undo`'s answer changed** from `Option<GoalId>` to
  `Option<Undone>`, on an item not yet released.
- **The line numbers start at 1**, as Fitch proofs do on paper, while
  goals and members start at 0, as 61a's session has them.
- **The goal lines say `lines:`** in place of `hypotheses:`, which
  changes 61a's session output; the manual and the lock moved with it.

None departs from the design as the prompt and the brief state it.

### What the next sessions must know

- **62c (the drawings)**:
  - A session with derived lines gives a DAG: a line cited twice is one
    node. `derived` emits the lines first, and a goal closed by a line
    has no node of its own (`derivation_goals` leaves it out).
  - `core-rules.md`'s note that the drawing and `derived` number a
    session's tree alike in postorder holds for sessions without lines.
    With lines, the drawing's numbering, which unfolds a shared node
    under each parent, and `derivation_goals` differ: 62c's tree
    drawing must map shared nodes, or the view's `inferences` must
    become per drawn inference.
  - Each citation of a hypothesis is a hypothesis node of its own, so
    a Fitch view should show the hypotheses of one scope with equal
    sequents as one line, and the derived lines in the order of their
    numbers (`Interactive::lines` on the innermost goal gives them).
  - Fitch's range citation of a subproof is the backward rule's
    discharging premise: its scope's lines are the range.
- **62d (the page)**:
  - The bindings have no `derive` or `cite` call yet: `rules_view`
    lists no lines and `rules_assume` takes no line.
  - The page's forward step needs a call over `Interactive::derive`
    with lines as numbers (`calls::index`'s rule for integers), and the
    view needs `lines` per goal.
  - `rules_undo` answers `reopened: null` for a forward step.
  - `about.level` is 18.

## Session 62c: the drawings

Opus 5.5 at high, cores 10-12, the reviewer on Fable 5.2. From the brief
`start-62c.md`, on `main@origin` after 61b's push.

### Outcome

A derivation in a calculus given as data, natural deduction's above all,
is drawn as a textbook draws it, as a Gentzen tree or as a Fitch proof,
in SVG, LaTeX and Typst, from the library and from `check --calculus`.

### Gentzen trees

- **A hypothesis is a leaf that is its formula alone**: the exports'
  `Drawn` trait has a rule drawn bare (`bare()`, default none; the
  `Drawing` makes its hypothesis rule bare). Every layout takes it:
  LaTeX `\hypo{…}`, curryst a bare conclusion, deduxit's own Typst
  layout a node with premises `-1` (its script draws no line, dots or
  dash for one), SVG no line, the text tree no bar. Decided over a
  layout of its own: the four layouts already handle a leaf without a
  line for an open goal shown bare, so the change is one guard each,
  and everything else (label tables, `<desc>`, ids, the estimate)
  stays shared.
- **Discharge marks**: the scopes that rules discharge are numbered in
  the drawing's order, from the leaves up; a hypothesis of a discharged
  scope is written `[A]` with its number raised, an assumption bare;
  the rule that discharges carries the numbers of every scope it
  discharges (`∨E¹˒²`, `→I³`). A rule with marks is a label variant of
  its own, so the per-rule label tables of the layouts need nothing per
  inference. The marks are a raised group in the label markup
  (`style::RAISED`, `Part::Sup`) and a superscript of each notation
  (`Atoms::superscript`: LaTeX `^{1}`, Typst `^(1)`, SVG a raised piece,
  text `¹`). Snapshots `natural.*`.
- 61b's leaf labelled `hyp` (a departure then) is gone: hypotheses of a
  sequent calculus's assumptions are bare leaves too.

### The Fitch proof

- **`rules::Fitch::new(&calculus, &derivation, &FitchOptions, &limits)`**:
  numbered lines, a subproof per discharged scope. The lines are ordered
  per scope by Kahn's algorithm over its parts (a node, or a subproof as
  one part): every part after what it cites, hypotheses first, the
  scope's conclusion last (the root; the premise a rule takes from a
  subproof), ties in the derivation's order; then each subproof is
  expanded in place. So a derivation whose nodes are not in Fitch's
  order, as 62a's translator writes them (a hypothesis where it is
  used, a subproof's lines apart), is reordered.
- **Citations**: a premise in the line's subproof or one around it by
  its number, a premise taken from a subproof by the subproof's first and
  last line (`→I 1–6`); a node cited by several lines is one line.
- **Refusals** (`Error::NotFitch`, unsupported, `not_fitch`, with the
  reason): a last node inside a subproof, a premise out of reach, a
  subproof two rules take with different conclusions, and no order with
  the conclusion last (a line inside a subproof uses the subproof's
  conclusion, which `check` accepts: such a proof is still drawn as a
  tree). Its size is estimated first (`limits.derivation_bytes`) and its
  walk up the scopes counted against `limits.work`.
- **Formats**: `Display` (text, `│` per proof, `├───` under
  hypotheses); SVG of deduxit's own (`export/svg/fitch.rs`: numbers,
  a line per subproof and a bar under its hypotheses, a justification
  column, ids `l<n>`, a `<desc>` that reads each line as a sentence);
  LaTeX for the package **`fitch` 1.0** (Peter Selinger's, maintained by
  Richard Zach for the Open Logic Project, used by the *forall x*
  books; TeX Live's `fitch`, with `kvoptions`): `\begin{nd}`, `\open`,
  `\hypo`, `\have`, `\by{RULE}{2-4}`, `\close`; Typst for **fitchcraft
  1.0.1** (`#proof((open, …, assume, …, close))`), chosen over
  derive-it 1.1.0, the other Fitch package nixpkgs has, which marks no
  hypothesis (no bar under an assumption) and takes indentation depths
  where fitchcraft takes Fitch's own open, assume and close. Both
  versions are the flake's nixpkgs' (2026-11-13), `typst::FITCHCRAFT`
  names fitchcraft's and the export check reads it from the source, as
  curryst's. Snapshots `fitch.*`.
- **One options value (D15)**: `rules::FitchOptions` (`hypothesis`,
  the justification beside a hypothesis, empty by default as Fitch and
  the package `fitch` write it, `Hyp` or `PR` in some textbooks; `open`,
  beside an open line, `?`), held in `export::Styles::fitch`, so the
  command's `--style fitch.hypothesis=Hyp`, a settings file and the web
  page's settings set it. The choice between the views is
  `ViewOptions::layout` (`Layout::Tree`, `Fitch`; the key
  `view.layout`), which the page can hold as it holds `view.structural`.

### The command

`check --calculus` takes `--format latex|typst|svg` (the verdict as the
format's comment, then the drawing; nothing to draw or a refusal puts
the verdict and the reason on standard error) and `--layout fitch`
(the Fitch proof in every format, its text in place of the node list).
The manual's natural-deduction chapter has "Draw a proof" with real
invocations, the guide a paragraph, and the behaviour lock three new
calls (and `check-rules-latex`, which drew nothing before and draws the
tree now).

### After 62b

62b's forward steps landed on `main` while this session ran; the chain
was rebased onto them (`nnwtrksk`) and two of 62b's notes for 62c met:

- **Equal hypotheses are one line.** A session cites a hypothesis by a
  node of its own per citation, and 62a's translator writes one per use,
  so `a → a ∧ a` had two lines `a`. The Fitch proof now shows the
  hypotheses of one scope with equal sequents as one line, the first
  one's, which every citation of the others cites
  (`fitch_shows_a_hypothesis_used_twice_once`). The tree keeps a leaf
  per use, as a tree does.
- **A drawn inference's node.** A session with lines is a DAG, and the
  tree unfolds a shared line under each parent, so a drawing's
  inference `n` is no longer node `n`. `Drawing::nodes()` gives the node
  each inference draws, as `Fitch::nodes()` gives each line's;
  `Interactive::derivation_goals`'s doc says so. The bindings' view
  still gives `inferences` per node (see below).

### Checks

- **The gate** passed at the head after the rebase onto 62b and after
  the last fix (clippy, the tests, both `cargo hack` runs, the ratchet:
  `render-svg` +1.47 % and `render-typst` +0.93 % in instructions,
  within their ceilings). The export suite is 41 tests: the trees'
  snapshots `natural.*` and the Fitch proof's `fitch.*`, the marks
  (`only_a_discharge_is_marked`), the refusal, the description, equal
  hypotheses, and `rules_drawings_stay_well_formed`.
- **The export check** (`nix build .#checks.x86_64-linux.export`)
  compiled every snapshot, `fitch.tex` with TeX Live's `fitch` and
  `fitch.typ` with fitchcraft 1.0.1 among them, before the review, and
  again inside the flake check below.
- **The behaviour lock**: five calls of `check --calculus` (the Fitch
  text, its Typst, the SVG tree, the Fitch style, and
  `check-rules-latex`), and the help pages.
- **The manual and the guide**: `cargo test -p deduxit-cli --test
  readme` runs the chapter's "Draw a proof" block and checks the names.
- **The cost**: the reviewer's inputs above, timed after the fixes.
- **`nix flake check`** at the last code commit (`uusxrkvr`), under the
  machine lock: passed (`--keep-going`). Its first run failed only the
  typos check, on `nd`, the package `fitch`'s environment
  (`\begin{nd}`), now allowed in `typos.toml` (folded into
  `otztspzs`).

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 at medium, once, on the chain before the
fixes. Its runs: 4 500 accepted derivations and 12 000 forged ones
through the Fitch ordering found no wrong order and no wrong citation;
about 55 outputs compiled with `pdflatex` and `typst`; every SVG was
well formed. Its five findings:

1. **A premise of an outer scope was marked as a discharge**: `∧I`
   with a premise from the scope around it got `→I`'s mark and a
   bracketed assumption. Fixed: only a premise in a scope inside the
   node's is discharged (`fitch::Climb::inside`, counted against
   `limits.work`); `only_a_discharge_is_marked`, and the lock's
   `check-natural-svg` lost its wrong brackets.
2. **The Fitch proof cost quadratic time and memory**: the estimate
   left out the `│` per depth (a proof nested 20 000 deep wrote 1.28 GB
   under an estimate of 6.4 MB), a scan per citation and per subproof
   (a fan of 200 000 citations took 16 s), the writers scanned every
   block per line (100 000 sibling subproofs took 14 to 18 s), and the
   CLI's drawing ignored the time limit. Fixed: `BAR` per depth in the
   estimate, a set of edges, each scope's first node found once, flags
   per line (`opens`, `closes`, `closes_hypotheses`), and the drawing
   asks check's stop. Measured after: nested 20 000 and 200 000
   refused in 0.2 s; the fan in 0.28 s; 100 000 subproofs in 0.25 to
   0.54 s in text, LaTeX and SVG.
3. **`--style fitch.hypothesis=Hyp` was refused** (the style keys'
   formats lacked `fitch`). Fixed, in the lock
   (`check-natural-fitch-style`).
4. **The SVG's description called the first line of every subproof a
   hypothesis**, a rule's line or an open line too. Fixed;
   `fitch_describes_each_line_by_what_it_is`.
5. **An atom of 4 096 letters overflows TeX's largest dimension** in
   the LaTeX tree. Not changed: it is the TeX limit
   `core-export.md` already records for every LaTeX drawing ("no
   layout fixes"), not this step's.

### Departures and decisions

- **A one-line subproof is cited as `n–n`** (departs from the design):
  some textbooks write `n` alone, but the range says that a subproof,
  not a line, is cited, which `→I`'s reader must tell.
- **`check --calculus --layout fitch` writes the Fitch text in place of
  the node list** (departs from the design): the brief asks for the
  view as an option of the drawing formats; the text format follows,
  since a list of nodes beside a Fitch proof would show the
  derivation twice.
- **`FitchOptions` holds the hypothesis's and the open line's
  justification, and no choice between boxes and bars**: the SVG draws
  bars, as `fitch` and fitchcraft do; boxes would be a drawing of their
  own for no reader who asked.
- **Typst through fitchcraft 1.0.1, not derive-it 1.1.0**, and LaTeX
  through `fitch` 1.0: above, with the reasons.

### What the next sessions must know

- **62d (the page)**:
  - The views are `ViewOptions::layout` (`Layout::Tree`, `Fitch`; the
    settings key `view.layout`) and the Fitch text's choices
    `export::Styles::fitch` (`fitch.hypothesis`, `fitch.open`). The
    bindings take both through the tab's settings already; the `draw`
    and `export` calls of `web/src/rules.rs` build a `Drawing` alone and
    must build a `rules::Fitch` when `view.layout` is `fitch`, as
    `cli/src/calculus.rs`'s `drawing` does, and pass its
    `Error::NotFitch` reason on (code `not_fitch`, kind unsupported).
  - The SVG's ids: a tree's inferences `i<n>` (postorder, as before),
    a Fitch proof's lines `l<n>` (from 1). `Drawing::nodes()` and
    `Fitch::nodes()` map them to the derivation's nodes, and
    `Interactive::derivation_goals` maps a node to its goal.
  - The view's `inferences` (`derivation_goals`, per node) equals the
    drawing's numbering only for a session without lines. Once the page
    has forward steps, it must give the goal per drawn inference
    through `Drawing::nodes()` (a shared line is drawn under each
    parent, so several inferences have one goal).
  - fitchcraft's version moves with nixpkgs: `typst::FITCHCRAFT` is
    checked against the package the export check finds.

### Commits

- `otztspzs` Draw natural deduction as trees with discharge marks and
  as Fitch proofs (the library, the export check, the rules files).
- `trmttyso` Draw and write Fitch proofs in check --calculus (the
  command, the manual, the guide, the changelog).
- `ztsotvnw` Pin check --calculus's drawings in the behaviour lock.
- `tsvoryqn` Mark only the discharges in natural deduction's trees, and
  bound the Fitch proof's cost (the review's findings 1 to 4).
- `yslqntyr` Pin the corrected marks and the Fitch style in the
  behaviour lock.
- `uusxrkvr` Show a hypothesis used several times as one line of a
  Fitch proof (62b's note; `Drawing::nodes`).
- The report.
