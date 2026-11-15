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
