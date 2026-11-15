# Step 60: rules as data, a generic derivation and its checker

## Session 60a: the rule language and the generic derivation

Session 60a, 2026-11-15, on Fable 5.2 at `high`. Brief:
`/tmp/linlog-handoff/start-60a.md`; prompt: `plan/60-rule-schemas.md`.

### Outcome

The design is written down (`plan/notes/rule-schemas.md`) and built as
the module `deduxit::rules`: a rule file in JSON (`Calculus`), a
derivation whose nodes name its rules (`Derivation`), and
`Derivation::check`, which matches every node against its rule within
the linear checker's kind of bounds. LK, LJ and minimal logic's calculus
ship as rule files, and on 30 000 broken copies of derivations from
every ordinary method the generic checker and
`ordinary::Derivation::check` give the same verdict on each. Natural
deduction with discharge and scopes, a signed tableau that closes on
complementary literals, a labelled modal calculus with a fresh label, an
ordered calculus and a context under a connective are each checked on a
small calculus in the tests. Both documents are at wire level 15.

Not reached, for 60b: the term printer from the print forms, the linear
calculus as a file with its cross-check, 56a's tableau as a rule file
with `Tableau`'s trees written in the generic form, the command, the
manual's chapter, the panel, a Kani harness and a fuzz target ("What
the next sessions must know").

### The decisions and their reasons

The design note has each with the alternatives weighed. In brief:

1. **One many-sorted first-order signature** (2.1). Formulas, labels,
   relational atoms, labelled and signed formulas are all terms:
   connectives with an arity and, where not every position is a
   formula, sorts; atoms of a sort in the derivation. One mechanism in
   place of a special case per method; MetTeL2 and Jape do the same.
2. **A sequent is a list of zones**, each a set, a multiset or a
   sequence, with a sort and bounds on its size (2.2). LJ is LK with
   `most: 1` on the right zone; a judgement of natural deduction is a
   sequent whose one zone holds one formula; a tableau branch is one
   set zone.
3. **Variables and contexts are declared, rule names are unique**
   (2.3). An undeclared name is refused, so a typo is never a variable.
   Where a textbook name covers two schemas the file has two rules
   (`∧R`, `∧R split`).
4. **JSON without a parser**: a schematic formula is an array
   `["∧", "A", "B"]`, nested 64 deep at most; a text syntax can follow
   from the print forms a connective declares.
5. **The menu of side conditions** (2.4): a context under a connective
   (`["!", "Γ"]`, which also states the modal rule `Γ ⊢ A / □Γ ⊢ □A`),
   `fresh`, and `atomic`. **`atomic` is this session's addition to the
   prompt's list** (decided here, for the planning session's review):
   G3's axiom and a tableau's closure "by complementary literals" need
   it and nothing else in the menu says it.
6. **Discharge is a property of a premise** (`discharges`), and
   **scopes carry it** (2.5, 3.3): every node lies in a scope, a rule
   that discharges takes its premise from a scope inside its own, and
   every hypothesis node of that scope must be one the rule discharges.
   Gentzen's trees and Fitch's subproofs are the same data; the
   "discharge link" is the scope, read through
   `Derivation::hypotheses(scope)`, not a second list that could
   disagree with it.
7. **Nodes store their sequents** (3.1), as `ordinary::Derivation`
   does: each node is checked alone, a drawing needs no check, and a
   stored value describes itself for the workspace graph. A derivation
   names its calculus and is read without it.
8. **No search in the checker** (4.1): a node gives the positions of
   the members its rule acts on (`at`), every member of the conclusion
   the rule writes out and those of a premise that nothing else
   determines. Given them, a node passes exactly when it is an instance
   of its rule. Contexts follow from the discipline (4.2); a rule whose
   contexts cannot be told apart is refused when the file is read.
9. **The module needs `serialize`** and has no feature of its own: a
   calculus enters only as a document.
10. **Freshness is forall x: Calgary's condition** (the name occurs in
    no hypothesis in force), stricter than Gentzen's by a renaming.

### What the source check changed

A research sub-agent (Sonnet 5.5, `high`) opened the tools' own files.
Two of its findings corrected the first draft: Isabelle's `ILL` does
not write promotion with `!` on a context variable (it uses an
auxiliary judgement), so the note no longer cites it for that; and the
freshness condition is forall x's and Carnap's, not "the name occurs
nowhere outside the subproof". Troelstra and Schwichtenberg's book was
not opened, so the note calls the shipped rules "the ordinary
checker's" and claims no more about G1. The note's last section lists
what was opened, what comes from `custom-rules.md` and what is from
memory.

### What was built

- `core/src/rules/`: `terms.rs` (the hash-consed n-ary arena `Terms`),
  `calculus.rs` (`Calculus`, its checks and the plan of each rule),
  `derivation.rs` (`Derivation`, `Node`, `NodeRef`, `Step`, scopes),
  `check.rs` (the checker), `ordinary.rs`
  (`Derivation::from_ordinary`), `calculi/{lk,lj,lm}.json` (generated
  once by a script, kept as data) and `calculus.schema.json`
  (`rules::SCHEMA`, JSON Schema 2020-12).
- `core/src/serialize/rules.rs`: both JSON forms; `wire::LEVEL` is 15
  (`wire::RULES`).
- `Error::InvalidCalculus`, `Error::InvalidDerivation`, `Error::Zones`,
  `Space::Scope`.
- `core/tests/rules.rs`: eight tests.
- `.claude/rules/core-rules.md`, and the lines of `core.md`, CLAUDE.md,
  `claude-infra.md`, `docs/src/status.md`, `CHANGELOG.md`, the crate
  docs and the guide's reference that name the module.

D15 and D16: the checker has no output of its own yet, so it has no
options value; its bounds are the caller's `Limits` (`memory_bytes`,
`work`, `occurrences`). D17: the note's 2.1 and `core-rules.md` say
where quantifiers go.

### Departures and assumptions

- **A premise of a discharging rule may lie in the node's own scope**
  (a vacuous discharge, Gentzen's `→I` over a formula that does not use
  the hypothesis). The first design asked for a scope of its own every
  time, which no plain line can enter without a reiteration rule.
- **The shipped LK has both forms of every binary rule under two
  names**, and LJ's cut as `cut` (split) and `cut shared` (only for a
  conclusion with nothing right of `⊢`), because that is exactly what
  the ordinary checker accepts.
- **`web/tests/ordinary.rs` pins the wire level**: 14 became 15.
- The prompt's "the linear calculus as a file" is not built (60b).

### What the next sessions must know

For 60b (Opus 5.5, `high`), in the order of its work:

- **The panel reviews `check.rs` and `Compiler::rule`/`equations`.**
  The claim to refute: given a node's positions, the node passes
  exactly when it is an instance of its rule; and every node's sequent
  follows from the hypotheses of its scope and the scopes around it
  (`core-rules.md` has both arguments). The cross-check covers
  multisets and LK/LJ's rules only; sets, sequences, mapped contexts,
  freshness and scopes have one test each and no reference
  implementation. A second, naive reference (search over all
  instantiations on small sequents) is what the counterexamples member
  should write.
- **Nothing prints a term yet.** The print forms (`symbol`, `fixity`,
  `precedence`, `associativity`) are read, kept and written back, and
  no code uses them. `Error::InvalidDerivation` names a node by index
  and a rule by name. A printer over `Terms` (a loop, as
  `sequents::fmt::Stops`; a term is a DAG and may unfold large, so it
  needs a bound) is the first thing the command, 61's `show` and the
  errors' `describe` need.
- **The linear calculus as a rule file** and its cross-check with
  `linear::Proof::check` on a sample: one zone marked `⊢`, multiset
  (sequence for the cyclic modes); `["?", "Γ"]` states promotion; the
  dual atom is a unary connective with `atomic`. The derivation comes
  from `Proof::derivation` (the one-sided view); the converter picks
  positions as `from_ordinary` does.
- **56a's tableau as a rule file**: one set zone of sort `signed`, the
  α and β rules, closures `×` (with `atomic` or without, as 56a closes
  on any formula), `T⊥` and `F⊤`; a converter from `ordinary::Tableau`
  writes each tableau node as the set of the lines on its branch. The
  test's `TABLEAU` calculus is the pattern.
- **The command and the texts**: `check` with a rule file (the flag's
  name is 60b's to choose with `cli.md`; the built-in files by name),
  the manual's chapter with real invocations, the guide's lines (its
  reference has a first paragraph), `status.md` (its bullet says the
  command does not use the module yet). Every output says "valid in
  the calculus given".
- **Evidence files**: `core/tests/wire/level-15.txt` and a stability
  test (a rule file and a derivation read and written back byte for
  byte; the tests compare JSON values so far), lines in the behaviour
  lock, a fuzz target per reader (`wire::upgrade::<Calculus>` and
  `::<Derivation>` then `check`), and Kani harnesses if the panel's
  integers member finds an integer worth one (the meter saturates;
  `Find::End` goes through `checked_sub`; every index is checked at
  `push`).
- **The schema** (`rules::SCHEMA`) validates the three shipped files
  (`check-jsonschema` 0.38.0, run by hand this session). A flake check
  that does it, and a published copy if the schema gets an `$id`, are
  open; it has no `$id` now.
- **The web client stores the wire level**: `about.level` is 15 after
  this step. The planning session checks whether the client pins 14.

For the steps after:

- **61** needs a backward step: from a goal, a rule and positions, the
  premises. That is new code over the compiled `Rule` (`Shape`,
  `Entry`, `Find` are crate-private and fit): the contexts are solved
  from the conclusion, a premise's patterns are instantiated with
  `Terms::apply` (which adds terms), and what the conclusion leaves
  open is asked of the user: a variable no conclusion member binds (a
  cut formula, a new label) and how a split context divides.
  `Calculus::positions(rule)` says how many positions a node gives; a
  list of which member each position stands for is easy to add beside
  it.
- **62**: the Fitch layout rules (a subproof's lines contiguous, its
  assumption first, a citation of its last line) are not the
  checker's; the Fitch view checks them where it draws, or a rule file
  gains a key if a course wants them enforced. A vacuous discharge is
  accepted. The eigenvariable condition is forall x's.
- **66**: labels are atoms of a sort `label`, named after the model's
  worlds; `x : Γ` (a label on a whole context) cannot be written, only
  a unary connective over a context, which Negri's rules do not need.
- **67** (the kernel): `core-rules.md`, "What it copied".
- **Not in the language**, each a decision for the step that needs it:
  several schemas under one rule name, a condition that two labels
  differ, a rule label for display, LaTeX and Typst forms of a
  connective (a new key raises the wire level, since a rule file
  refuses a key it does not know).

### The reviewer's findings

`fresh-reviewer` on Opus 5.5 at `high`, once, on the two commits. Four
findings, three reproduced by it; all four hold and are fixed, each
with a test.

1. **A hypothesis escaped its scope when the root lay in an inner
   scope**: every node passed, and `a, a → b` in scope 1 with `→E` as
   the last node showed `b` with no assumption. The scope invariant is
   a statement per node. Fixed: `check` refuses a root outside the root
   scope (`natural_deduction_discharges_by_scope`).
2. **The cross-check claim was false for `↔L₁` and `↔L₂`**: the
   ordinary checker asks for both implications of the equivalence in
   the arena, the rule files only for the one written out. No engine
   makes such an arena, so the test's sample could not show it. Fixed
   in `from_ordinary`, which refuses such a derivation as the ordinary
   checker does; the reviewer's document is in the test.
3. **The freshness walk over the hypotheses in force was not counted**
   as work (quadratic time under a linear count; the stop was still
   asked). Fixed: `occurs` counts every member it is handed
   (`the_check_keeps_to_its_bounds`).
4. **A connective's arity was allocated unchecked**: `4294967295`
   reserved 16 GiB whatever the memory bound, and would panic on a
   32-bit target. Fixed: 255 arguments at most (`calculus::ARITY`, in
   the schema too).

It found sound: matching and lookup by value, the compiled plan (every
`Value` member bound, positions counted as consumed), the context
equations and the set zones, the per-node checks, the discharge rule
for premises, the indices, the converter's choice of names, and that
every requirement of the prompt's "Fixed now" is expressible in the
type.

### Checks

All on cores 10-12, capped at 8 GB, at the chain's last code commit
(change `nyuyswqn`) unless said otherwise.

- `cargo test -p deduxit --test rules`: 8 passed. The cross-check
  decided 18 sequents in three logics by every method that gives a
  derivation, broke each inference in about 45 ways, and compared the
  two checkers on every document the ordinary reader takes: more than
  30 000 refused alike, some accepted alike, none differing; LJ's cut
  occurs in the sample.
- `cargo test --workspace --no-fail-fast`: everything passed but the
  module's doc example, which was fixed and rerun alone (before the
  review's fixes; the gate below ran the workspace's tests after them).
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- `gate`: passed twice, the second time after the review's fixes. Its
  note that the cache misses of `check-qbf-20-2` moved (-5.5 %) is not
  this step's: nothing the journey runs was touched.
- `check-jsonschema` 0.38.0 (nixpkgs, by hand): `lk.json`, `lj.json`
  and `lm.json` validate against `calculus.schema.json`.
- `nix flake check`, under the machine lock: passed at the chain's
  last code commit, on its third run. The first failed because crane's
  cargo sources leave out `.json`, so the rule files that
  `include_str!` reads were missing from every Rust check
  (`modules/workspace.nix` now adds `core/src/rules/**/*.json` to the
  `core` part, `flake.md` says so); the second on three links in the
  doc of the private module `rules::ordinary`, which rustdoc resolves
  in the parent's scope (`doc-private`). The gate does not build the
  private docs and reads the working tree, so it saw neither. The two
  fixes after the second gate run change a doc comment and the flake's
  fileset only.

Not run: `breaks`, the fuzz targets' build (no reader's signature
changed), `nix build .#kani-harnesses` (the linear checker is
untouched), a benchmark.

### Checklist

- [x] The design note, each choice with reasons, alternatives and
  sources.
- [x] The data types, their JSON at wire level 15, the JSON Schema.
- [x] The checker with its bounds.
- [x] LK and LJ (and LM) as rule files, the cross-check with
  `ordinary::Derivation::check`.
- [x] `fresh-reviewer` once (Opus 5.5, `high`); every finding answered.
- [x] The gate.
- [ ] 60b: the printer, the linear rule file, 56a's tableau as a rule
  file, the command, the manual's chapter, the panel.

## Session 60b: rules as data, finished, and its panel

Session 60b, 2026-11-15, the same conversation as 60a, now on Opus 5.5
at `high`. Brief: `/tmp/linlog-handoff/start-60b.md`; its list was 60a's
"What the next sessions must know".

### Outcome

Everything on 60a's list for 60b is built, and the panel has reviewed
the checker. `deduxit check` checks a derivation in a calculus given as
data (`--calculus FILE` or a built-in name; a derivation naming a
built-in calculus needs no flag), the manual has a chapter for it with
real invocations, and the guide and `status.md` say so. Terms print by
the calculus's print forms. One-sided linear logic and the signed
tableau ship as rule files, checked against the linear checker's
proofs and the ordinary suite's tableaux. Wire level 15 has its
stability file and two fuzz targets. The panel found no unsoundness
and eleven other faults (one reader panic, three false refusals, memory
past its bound, uncounted work, three holes in the reader), all fixed
with a test each.

### What was built

- **The printer** (`core/src/rules/print.rs`): `Calculus::show(terms,
  id)` and `Calculus::show_sequent(derivation, node)` return `Shown`, a
  `Display` bounded to `Shown::LIMIT` (4 096) characters, `limit(n)` to
  change it, a loop over an explicit stack. A connective without
  `fixity` is prefix at arity 1, infix at 2, a function otherwise;
  brackets follow precedence and associativity. `core/tests/depth.rs`
  prints and checks terms 50 000 deep on a stack of 256 KiB. An
  `InvalidDerivation` now names the failing node's sequent (200
  characters at most) and, where a member is missing, the sequent and
  zone that lack it.
- **`rules::LL`** (`calculi/ll.json`): one-sided linear logic,
  propositional, no cut, no Mix, not affine; the dual of an atom is the
  atom under `~` with `atomic`; promotion keeps `["?", "Γ"]`. The test
  `the_linear_rule_file_agrees_with_the_linear_checker` writes the
  linear suite's proofs of 28 sequents (eleven by hand, the families'
  smallest provable classical or intuitionistic instances) through a
  converter in the test, uncompacted and one-sided: 27 proofs, 505
  nodes, all accepted; another rule at each of 369 nodes refused.
- **`rules::TABLEAU`** (`calculi/tableau.json`) and
  `Derivation::from_tableau` (`tableau.rs`, feature `interactive`):
  Smullyan's signed tableau, one `set` zone of signed formulas, an
  expansion keeping the formula it expands as a branch does, closures
  `×`, `T⊥`, `F⊤`. The test checks the tableaux of the method `tableau`
  and those `Tableau::of` reads off the embedding's derivations, on the
  18 sequents of the ordinary cross-check and four not valid: closed
  ones pass, open ones are refused, and a closure that cites another
  line is refused.
- **The command** (`cli/src/calculus.rs`, `check.rs`,
  `argument_parsing.rs`): `check` reads a document with the key
  `calculus` as such a derivation; `--calculus CALCULUS` names a rule
  file or `LK`, `LJ`, `LM`, `LL`, `tableau` in any case. Verdict lines:
  `valid in the calculus given (NAME): SEQUENT, a derivation of N nodes
  [from K assumptions]`, then the nodes as a list indented by scope;
  `invalid in the calculus given (NAME): REASON`, exit 1; `unknown: …`
  at a limit, exit 3; a rule file that is no calculus, exit 2. JSON is
  `Checked` with `calculus`. Text and JSON only; `eliminate` refuses
  the flag. The behaviour lock gains five calls (its own commit).
- **The manual's chapter** `docs/src/rules.md`, "Check a derivation in
  your own rules", after `check.md`: a derivation in LK, a natural
  deduction rule file with a discharge and nested scopes, a derivation
  that uses a closed subproof's line, the rule file's keys, the exit
  statuses. The guide's reference has the command beside the library.
- **Evidence**: `core/tests/wire/level-15.txt` (a rule file and two
  derivations, written once by the library) and
  `the_fifteenth_level_reads`; the fuzz targets `rule_file` and
  `json_derivation`, in `fuzz/run.sh` and seeded by `fuzz/seed.sh`.
  `fuzz/fuzz_targets/ordinary_decide.rs` had stopped building (it called
  `ordinary::decide` with `decide_within`'s arguments): fixed, since the
  brief's fuzz check needs every target to build.

D15 and D16: the printer's bound is a named constant with a setter; the
command's list keeps to `--abbreviate` and `--derivation-limit`, its
check to `--timeout`, `--work-limit` and `--memory-limit`. No settings
key is new: `--calculus` is an input, as the proof file is.

### The panel

Three members, each in a jj workspace of its own on `main`, briefed
with the claim of `core-rules.md` (instance, scopes, bounds) and the
machine's rules (`scratchpad/panel/claim.md`).

- **Counterexamples** (`panel-counterexamples`, Opus 5.5 at `high`):
  a second reference of its own (no code shared with the checker; it
  reads the rule file's JSON and searches every placement of the
  written members, every split of the rest among the contexts, every
  subset for a set) compared with `check` on every position vector of
  each node: 22 212 random rule files (13 356 compiled) with 397 506
  nodes and 14.6 million position vectors; 80 000 odd rule files for
  the reader; 1.2 million natural deduction derivations with nested
  scopes against a dependency judgement of its own; the committed
  calculi's 106 rules. **No unsound accept anywhere.** Five
  refutations: a reader panic and four false refusals (below).
- **Argument** (`panel-argument`, Fable 5.2 at `high`): the invariants
  stated from the code first, then checked at every site; the
  compiler's plan, the matching, the equations, the set zones and the
  scope induction hold. Gaps: the same panic, three of the same false
  refusals, a conclusion's zone key written twice read as the last
  (a rule file is the trusted base), and two places where the design
  note's words differ from the code (`Calculus::located`, "mapped on
  the way in and out").
- **Integers** (`panel-integers`, Sonnet 5.5 at `high`): no wrap,
  truncation or integer panic; the same reader panic; three structures
  the memory estimate left out (the freshness walk's stack, grown by the
  arity per level, 40.8 MB past a 1 MB bound; the contexts, 200 MB past
  64 MiB; the slots of empty zones, 108 MB past 1 MiB); a rule file of
  a thousand premises over a thousand zones compiled into 56 MB with no
  bound; and work no unit counted: the compiler's equations and
  duplicate test (quadratic, 2.9 s for a chain of 8 000 premises), sort
  lookups by linear scan (n² for n sorts, in the reader and the
  checker), the discharge comparison (copies per pattern), the cover
  check of set zones.

What was fixed (`ynwwwqxz`), each with a test in `core/tests/rules.rs`
(`a_rule_file_that_is_no_calculus_is_refused`,
`a_node_that_is_an_instance_passes`,
`the_memory_bound_counts_every_context_and_zone`):

- the reader refuses a connective over a context that does not take
  one argument, a variable named twice in `fresh` or `atomic`, a zone's
  key written twice in any sequent, and a file whose rules would
  compile past `limits.memory_bytes`; names, sorts and zones are looked
  up in hash maps;
- `Compiler::equations` is a worklist, and each equation keeps its
  segment's contexts, so neither the compiler nor the checker rescans a
  zone (the chain of 8 000: 22 ms);
- within a zone, members found by position come before those found by
  value; an empty context under a connective maps to nothing; a set
  zone's discharge is compared as a set;
- the freshness walk marks a term as it pushes it, so its stack holds a
  term once; the memory estimate counts the walk's stack, every
  context, every zone's slots and each rule's patterns and discharges;
  the discharge comparison sorts each hypothesis once and counts its
  work, and the cover check and the conditions count theirs.

All five counterexamples and the integers member's seven probes were
run again on the fixed code: the panics and false refusals are gone
(the panic case and the duplicate condition are now refused when the
file is read), the memory cases are refused by the bound, and the
quadratic cases end in milliseconds.

### The reviewer's findings

`fresh-reviewer` on Fable 5.2 (not the writer's model), once, on the
seven commits. Four findings, none a soundness fault; all hold and are
fixed in `twmqwmlu`:

1. **`check --abbreviate none` wrote the verdict's sequent with no
   bound** and no stop: a derivation of 1 KB whose term doubles 60
   times ran past `--timeout 1s` for 30 s. Now every sequent `check`
   writes is cut at `--abbreviate`, or with `none` at
   `--derivation-limit` characters (`cli/src/calculus.rs`, `bound`); the
   witness answers at once.
2. **The memory estimate was below what the check holds** in two
   places: the freshness walk's stack is a vector that may hold twice
   its length (1.5 times the bound measured), and the checker's tables
   sized by the calculus were not counted (1.6 MB held under a bound of
   4 KiB with 200 000 connectives). The estimate now counts 32 bytes a
   term and a slot per connective, variable and context.
3. **`Shown` wrote `limit` characters and then `…`**, where its doc said
   at most `limit`: the doc now says so.
4. **The manual said "three conditions" and listed two**: fixed.

It found sound: the printer (no recursion or panic; brackets
injective on 49 573 terms of LK and LJ, 143 254 of LL and 233 304 of
TABLEAU), `from_tableau` (29 950 forged tableaux: no panic, no open one
accepted, every accepted root classically unsatisfiable), the panel's
fixes (the worklist computes the same fixpoint; positions keep the `at`
order), the command's statuses and examples, and the fuzz targets' use
of the API.

### Checks

All on cores 10-12, capped at 8 GB.

- `cargo test -p deduxit --test rules --test depth --test stability`:
  13, 3 and 14 passed at the last code commit (`twmqwmlu`), the
  cross-checks with LK, LJ, LM, LL and the tableau among them.
- `cargo test -p deduxit-cli --test readme --test lock`: 7 and 1
  passed; the lock's five new records and `help-check` were blessed in
  their own commit (`tyospvtu`).
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean.
- `gate`: passed at `nwwrnmpl` and again at `twmqwmlu`.
- `nix develop .#fuzz -c cargo check --manifest-path fuzz/Cargo.toml
  --bins`: every target builds (after the fix to `ordinary_decide`).
- `nix flake check`, under the machine lock: passed at `twmqwmlu`, on its
  second run; the first failed on shellcheck's SC2094 at a line of
  `fuzz/seed.sh` (a file read and named in one command), rewritten.
- The panel's and the reviewer's witnesses, run again on the fixed
  code through the command or their probes: refused or accepted as the
  claim says, in milliseconds.

Not run: a fuzz run, `breaks`, `nix build .#kani-harnesses`, a
benchmark.

### Departures and assumptions

- **The converter from the linear suite is in the test, not the
  library** (departs from the design, which named neither): the
  library ships `rules::LL` and the cross-check; a `from_linear` would
  be API that no caller asked for, and adding it later breaks nothing.
- **`check` shows such a derivation as text or JSON only**: the
  drawings (LaTeX, Typst, SVG) need the derivation's layout through the
  exports' `Drawable`, which is 61's or 62's (the Fitch view) to add.
- **The tableau's `×` has no `atomic`**, as 60a's report foresaw: 56a
  closes a branch on any formula.

### What the next sessions must know

No 60c is needed: what 60a left for 60b is done. For 61 and later:

- **61 (building with custom rules)** needs a backward step: from a
  goal, a rule and the positions, the premises. The compiled `Rule`
  has what it needs (`Shape`, `Entry`, `Find`, the ordered
  `equations`), all crate-private; a premise's patterns are
  instantiated with `Terms::apply`, and what the conclusion leaves
  open is asked of the user (a variable no conclusion member binds,
  how a split context divides). `Calculus::positions(rule)` is a count;
  61 adds a list of which member each position stands for. An open
  goal is `Step::Open`, which the checker refuses, so a session's
  finished derivation is checked whole as the ordinary goal state's is.
  `interact --calculus` takes the flag's name and its built-in names
  from `check` (`cli/src/calculus.rs`, `BUILT_IN`).
- **The drawings**: `check` shows such a derivation as text or JSON
  only. A drawing needs the exports' `Drawable` over a
  `rules::Derivation` (a tree where every node has one parent; a Fitch
  list for natural deduction, 62).
- **62 (natural deduction)**: the Fitch layout's own rules (a
  subproof's lines contiguous, its assumption first, a citation of its
  last line) are not the checker's; a vacuous discharge is accepted; a
  set zone's discharge is compared as a set. The chapter's NJ is the
  pattern of a rule file.
- **The panel's references** live outside the repository (under the
  session's scratchpad, gone with it): the counterexamples member's
  exhaustive reference and generators would be worth a committed
  test-only reference, as step 26 made for the linear engines, if a
  later step changes the checker.
- **Fuzzing**: the two new targets build; no fuzz run was made (none
  was named). `fuzz/seed.sh` seeds them from the shipped rule files and
  `wire/level-15.txt`.
- **Kani**: no harness was added. The panel's integers member found no
  integer that can wrap; the checker's counters saturate and its
  indices are checked when a derivation is built.
- **The web client** stores the wire level 15 since 60a (`about.level`).
