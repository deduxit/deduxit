# Step 49: ordinary logic step by step, in the command and on the page

## Session 49a: the goal state

### Outcome

The library now proves an ordinary sequent step by step in LK and LJ (classical, intuitionistic and minimal logic). The new type `linlog::ordinary::Interactive` (feature `interactive`, file `core/src/ordinary/interactive.rs`) is a native goal state: its goals are two-sided ordinary sequents, its rules are the checker's `ordinary::Rule`, and a finished state is an `ordinary::Derivation` that `Derivation::check` validates. Nothing is added to what is trusted. The state offers these operations:

- `goals` and `goal`;
- `rules`, the applicable rules of a formula, by side and position;
- `apply`, with a `Step` and an optional `Context::Split`;
- `undo`;
- `close`, which decides the goal through `ordinary::decide` within the caller's `search::Options` and `Limits` and grafts the derivation found;
- `hint`, the first rule of that derivation as a `Step` that `apply` takes;
- `derivation_within` and `derivation_ids`;
- `finished`, the checked derivation.

The ordinary sequent, the derivation, the outcome of `decide` (written only) and the goal state are JSON forms at the new wire level 8 (`wire::ORDINARY`, file `core/src/serialize/ordinary.rs`). `ViewOptions::structural` draws any derivation of LK or LJ either with its weakenings and contractions left out where they are admissible (`Structural::Admissible`, the default) or with every one of them as an inference (`Explicit`). In LJ, a rule with two premises such as `→L` gives both premises the whole context by default. The contractions the checker needs for that are made below the rule and are left out of the default drawing, so a step reads as G3's rule while the finished derivation still passes `Derivation::check`.

The commits, in the workspace `../linlog-step49a` on `main`:

| change | commit | subject |
|---|---|---|
| `ozrwqnpp` | `81e1b2010444` | Add the goal state of ordinary sequents and their JSON forms at wire level 8 |
| `qqlowukt` | `96b7ff06c555` | Record the ordinary goal state's invariants in the rules |
| `nnxwnlsu` | `6ad422c60f2f` | Answer the review of the ordinary goal state |
| `mtvmuqvn` | `0a74c2eabd16` | Run the target sets at 49a: every decided row's counters equal 47's |
| `vwmnstuq` | `2452ec4c8441` | Report session 49a: the goal state of ordinary sequents (this report's first version) |
| `vukwtlxz` | `a10603058891` | Share the context of LJ's implication left by default |
| `vxtzvymu` | `cf16366a5427` | Bound the structural runs of an ordinary step by the derivation's size |

This report's last version is the change after them.

### What was checked, and by what

- **The gate** passed at `mtvmuqvn`, again at `vukwtlxz`, and at `vxtzvymu` (see the checklist): exit 0, ending `gate: passed`. It ran clippy, the 22 test binaries, the `cargo hack` runs over each feature (13 runs) and over every pair (58 runs), and the ratchet.
  - The first gate failed at `--no-default-features`: four helpers were dead without the features that use them. They were put behind those features.
  - Before the second gate, both `cargo hack` runs passed on their own with `RUSTFLAGS="-D warnings"` (`each-feature exit 0`, `powerset-depth-2 exit 0`).
- **The ratchet**, every journey within its ceiling:
  - `derivation-chain-64` +1.95 % and `search-wide-m2-256` +1.76 %, as the prompt reports them after step 47.
  - `ordinary-pigeons` +1.65 %, against +1.33 % in step 47's report: 99 k more of 31.2 M instructions.
  - I did not measure `main` after the dependency update. I attribute the rise, unverified, to the checker's per-inference function, which now first refuses an open goal and holds classical logic to LK's negation forms.
  - The client's flake check (biome built from source) ran beside these runs, as `linlog-orchestrator` said. It moves no counter.
- **The target sets**, each run at `nnxwnlsu` with `summary --against … --counters`. The later commits change only `ordinary::interactive`, the limits table's text and `ViewOptions`' default view of LK and LJ, none of which a search reads. I did not run the sets again.
  - `bench/targets.sh 49a` against `bench/targets/47.csv`: no verdict differs; "Of the 145 problems both decide alike on one thread, 0 differ in nodes, splits, memo_hits, memo_entries, links, tests".
  - `bench/net-targets.sh 49a` against `net-47.csv`: no verdict differs; 0 of 64 differ.
- **The focused tests** (`ordinary::interactive::tests`, one per behaviour):
  - `a_finished_state_checks`: a finished state checks in each logic.
  - `undo_restores`: undo restores the state, after a rule and after a graft.
  - `a_rule_that_does_not_fit_is_refused_with_why`: a refused rule says why, the error and its text pinned, and changes nothing.
  - `structural_rules_explicit_or_admissible`: the two views, pinned as text trees.
  - `a_shared_implication_left_checks`: a shared `→L` in LJ and in minimal logic gives both premises every hypothesis. Its finished derivation checks with its two contractions, and the default drawing is G3's `→L`.
  - `close_and_hint`: a close and a hint; "unknown" with its reason (`Reason::CopyBound { copies: 1 }` on `⊢ ¬¬(a ∨ ¬a)` with one copy), the goal left open.
  - `the_bounds_hold`: the occurrence bound, the derivation's estimate, the stop, and a shared `→L` over 200 hypotheses refused under a bound of a megabyte.
  - `random_steps_finish_as_checked_derivations`: random steps, splits and undos over 60 generated sequents in every logic. Each state round-trips through JSON, and every state whose goals the search closes finishes checked.
  - `a_forged_document_is_refused`: forged documents, the reviewer's four repros among them.
- **The old JSON forms still read**: `core/tests/stability.rs` reads levels 1 to 7 unchanged and writes the same bytes. `the_eighth_level_reads` pins level 8's sequent, derivation and session in `core/tests/wire/level-8.txt`, never to be rewritten.
- **`web-core-wasm32` was not run**: this step does not touch the integers of the linear checker, the size estimate or the memo. The ordinary checker compares sorted vectors and has no counter.
- **`nix flake check`**:
  - The first run, at `vwmnstuq`, failed in `doc-private` only: an intra-doc link ``[`decide`]`` was ambiguous between the function and the module in the private documentation. It was fixed in `vukwtlxz` (``[`decide()`]``), and the private documentation then built with `-D warnings`.
  - The run at the last commit: see the checklist.

### The reviewer's round and the answers

A fresh-context reviewer (Opus 5.5, high) judged soundness, the bounds and the stated requirements. It ran random sessions, hint-driven proofs, 18 000 mutated session documents, 3 600 mutated derivation documents, a formula 200 000 deep and a chain of 40 001 inferences. It found four breaks, each reproduced. All four hold, and all four are fixed in `nnxwnlsu` and pinned in `a_forged_document_is_refused`:

1. **A session document, two undos and a close gave a derivation the checker rejects.**
   - The document's history closed an LJ goal `a, ¬a ⊢` that has nothing right of `⊢`. Undo reopened it, and `close` grafted the search's derivation of `a, ¬a ⊢ ⊥` onto it.
   - Answer: the reader refuses a history entry of LJ whose inference has not exactly one formula right of `⊢`, the goal that its undo would reopen. `graft` refuses a root that does not conclude the goal as multisets (`Error::GoalMismatch`).
2. **The Rocq writer panicked on a checked classical derivation.**
   - The checker accepted the `A → ⊥` form of `¬R` in LK, which the classical certificate does not read.
   - Answer: the checker now takes LK's own forms of `¬R` and `¬L` in classical logic, as `Rule`'s documentation already said. The read-back never made the other form classically, and the target sets are unchanged.
3. **The admissible view indexed out of range on a forged derivation.**
   - Answer: a structural rule is left out only where its conclusion holds the principal formula of the rule above. Elsewhere it is drawn.
4. **A derivation document with an `"open"` inference that had premises made the text layout's height underflow.**
   - Answer: the derivation reader takes an open goal only as a leaf.

**A second round, on `vukwtlxz`**, asked by `linlog-orchestrator`. The reviewer judged the shared `→L`/`¬L` of LJ.

- It found no break:
  - 6 564 shared steps over every left side of up to five hypotheses with repeats, the principal first, last or alone, in intuitionistic and minimal logic: 2 926 finished, all checked.
  - Every undo restored the state.
  - The `unreachable!` was never reached.
- It found a bound gap that holds: the axiom's weakenings and the shared rule's contractions store formula ids quadratic in the goal (2 000 hypotheses gave 6 million ids). Nothing bounded that, against the limits table's "linear in the goal".
- Answer, in `vxtzvymu`:
  - `apply` refuses such a run before it is made when its upper estimate passes the `derivation_bytes` the state was started or read under (`StepError::Size`).
  - The limits table now says so.
  - The refusal is tested.

The reviewer also found something outside its criteria, by reading only: the Rocq writer would write an ill-typed term for an LJ derivation that uses LK's unary `¬L` (`Γ, ¬A ⊢` from `Γ ⊢ A`). The checker accepts that inference; neither the read-back nor the goal state makes it. It is predated by this step and recorded below as a follow-up, not fixed.

### Decisions

- **The rules are the checker's, and every inference a step makes is checked as it is made.**
  - `apply` builds the step's inferences from one rule table (`shape`). It runs the checker's own per-inference function (`check_inference`, factored out of `Derivation::check`) on each before the state changes.
  - `rules` lists exactly what `shape` accepts, so the list and `apply` cannot disagree.
  - A construction error would be an `unreachable!` panic, never a state that the final check refuses. The reviewer tried and found no input that reaches it.
- **The axiom closes a goal with a context** by a run of weakenings below `A ⊢ A`: one step, which undo takes back whole. The checker's axiom has no context. Requiring the user to weaken first would be faithful to G1 but tedious, and the admissible view shows the run as G3's axiom anyway.
- **A rule with two premises shares the context unless the step splits it, LJ's `→L` and `¬L` included.** The checker accepts a binary rule either sharing the context or splitting it, never both at once. LJ's `→L` gives its first premise its own formula right of `⊢`, so sharing there goes through `contracted`:
  - a `CL` of every other hypothesis below the rule;
  - then the rule split: the first copies to the first premise, the second copies and the goal's formula right of `⊢` to the second.

  The admissible view, now the default, leaves the contractions out and shows G3's `→L` with the context in both premises. The explicit view shows them. The rule's principal formula is not kept in the premises (G3i's `→L` keeps it); a hypothesis used twice is contracted by hand.

  This answers `linlog-orchestrator`'s correction of the first version, which made a first course split by hand. The checker could express the shared form through inserted contractions, so nothing departs from the design here and no trusted rule is added. `StepError::Succedents` remains for a split that sends the goal's formula to the first premise.
- **In intuitionistic and minimal logic every goal has exactly one formula right of `⊢` (departs from the design).** The design says the rules are the checker's. The goal state offers a subset of them in LJ:
  - `WR`, `CR`, `⊥R` and `∨R` with both disjuncts are refused there with `StepError::Logic`.
  - `¬R`/`¬L` take the form of `→R`/`→L` for `A → ⊥`.
  - `Γ ⊢` starts as `Γ ⊢ ⊥`, as `decide` reads it.

  This makes the search's derivation of a goal conclude the goal itself, so `close` and `hint` need no rewriting of the derivation. LJ without empty right sides but with `⊥L` is still complete for intuitionistic logic. In minimal logic the checker forbids an empty right side anyway.
- **The structural rules are a display option** (D15): `ViewOptions::structural`, constant `DEFAULT_STRUCTURAL` (`Admissible`, as `linlog-orchestrator` asked), JSON `"structural": "explicit" | "admissible"`.
  - It works on any derivation of LK or LJ (`Derivation::shown`), the one `decide` returns included, not only on a state's.
  - The admissible view leaves out each weakening and contraction whose branch the rules above close. A run above an open goal stays, so an open goal shows its own sequent.
  - What is shown admissible fails the checker by design; `finished` always checks the explicit derivation.
- **`close` and `hint` take `translation: Option<Translation>`** (`None` for the logic's default, as `ordinary::Options::translation`), beside the linear session's `search::Options` and `Limits`. The state's logic is fixed at its start, so it is not passed again.
- **The hint lives in the library.** The linear session's hint is the bindings'. The prompt asks the library for one, read off the found derivation's first rule. For two premises it also says whether the context was shared or which positions went to the first premise.
- **The outcome's JSON is written only**, as the linear outcome's is. It holds the derivation of a valid sequent under `"derivation"`, and the image's engine and statistics, not the linear proof.
- **Positions are `(side, position)`** on the goal's two lists in their stored order, the same addressing as `Inference::principal`. A premise keeps the goal's order, with the principal's place taken by its subformulas and a formula that crosses `⊢` appended.

### Options, and how each front end sets them (D15, D16)

- **`ViewOptions::structural`**: the command would map a flag onto it (49b: `--structural explicit|admissible`, a settings key `view.structural` that already reads), the page would hold it in its view settings JSON, and the bindings pass it through as they pass `view`. It has no flag yet; 49b adds it with a line of `--help`.
- **`close`/`hint`'s translation**: `settings.ordinary.translation` in the command and the bindings.
- **Bounds**: `search::Options` and `Limits` as for every search; the occurrence bound for `within` and the readers; the `derivation_bytes` given to `within` (or to the reader) also bounds a step's run of structural rules.

### Where quantifiers go (D17)

`Node` gains `Forall`/`Exists` as `core-ordinary.md` says. Each part of the goal state then grows as follows:

- `shape` gets the rows for `∀R`, `∀L`, `∃R` and `∃L`.
- `Step` gains a witness, as the linear `Step` has one.
- The state gains a term table with marks for undo, as the linear `Quantified` is.
- `Context` and the JSON forms keep their shape; the formulas' tags gain `∀` and `∃`.

### What the goal state copied from the linear session

This is the record that the kernel crate (step 67) is extracted from. Nothing generic was factored out.

- **Copied in shape, rewritten for two-sided sequents:**
  - The top-down arena: root 0, a step appending what it opens, so that a premise has a larger index than its conclusion.
  - The history as the goals closed, in order, and `undo` as "truncate to the smallest index of the closed goal's subtree, reopen the goal". `subtree`, `postorder` and the graft's reversed append with its renumbering are line-for-line counterparts.
  - `goals`, `goal`, `is_complete` and `steps`.
  - `Applicable`/`Needs`, `Closed { outcome, grafted }`, and `StepError` with a snake_case `kind` tag and a phrase per variant.
  - `derivation_within`, with an estimate by `Size`'s constants before building and a stop at `Phase::View`, and `derivation_ids`.
  - `finished` as the counterpart of `proof()`: `Error::OpenGoals`, then the checker, always.
  - The session's JSON as `{version, sequent, inferences, history}`, an open goal its sequent alone. The reader checks the tree's shape, replays every inference and walks the history from the last step back with the suffix test, without entering what a later step added.
  - `Rule::Open`, with `Drawn::OPEN` drawing it bare.
- **Shared, not copied:** `GoalId` (the linear session's type), `Size::BYTES_PER_INFERENCE` and `BYTES_PER_CHARACTER`, the `Drawn` layout, `Limits` and `Progress`, `wire::Readable`, and `Error::InconsistentSession`, `OpenGoals` and `GoalMismatch`.
- **Different by nature:**
  - Validation is the derivation's own checker applied per inference, where the linear session validates in `expand` and translates to a proof term that the checker checks.
  - `close` goes through `decide` (translation, search, read-back, check), not `prove_goal` and `Derivation::of_goal`.
  - Splits are positions per side, not a split of a sorted multiset.
  - There is no invertible-rules run, no cut and no first-order part yet.

### Assumptions

- "Rule sets as a display choice … where the checker's rule table allows both" is read as the explicit and the admissible view of one derivation, for every derivation of LK or LJ. The step's rules (what `apply` offers) are the same in both.
- The bounds "the linear session keeps" are read as those it has: the occurrence bound at the start and at reading, the derivation's estimate before building (for drawings, grafts and a step's run of structural rules), the stop in every long call, loops instead of recursion. Neither session bounds the number of steps a caller takes.

### Left open, for later sessions

- **Pre-existing, not this step's:** `cargo test -p linlog` alone (without `parallel`) warns on an unused import in `search/focus/tests.rs` and a dead `random_ill_text` in `search/reference_fo.rs`. The workspace runs, which unify `parallel` on, do not.
- **Pre-existing, found by the reviewer:** the Rocq writer's term for LJ's unary `¬L` has type `A`, not `False`. No derivation the library makes uses that form; a document can.
- **The manual and `status.md`** describe the command, which 49b changes. This session changed the guide for AI models' library section (`skills/linlog/reference.md`, section 16) in the code commit.

### What the next sessions must know

- **49b** maps `linlog interact --logic classical|intuitionistic|minimal` onto `ordinary::Interactive`.
  - A goal's formula is a side and a position. A split is `Step::split(&left_positions, &right_positions)` going to the first premise.
  - `rules` gives `Needs::Context` for rules with two premises.
  - In LJ, `→L` and `¬L` share the context by default (hidden contractions). A split stays possible for Gentzen's form.
  - `StepError::Size` refuses an axiom or a shared rule on a goal too large for the derivation bound the state was started under.
- **Drawing:**
  - `derivation_within(&view, &limits, stop)` draws a state with open goals as bare leaves, with the structural rules as `view.structural` says. `derivation_ids(view.structural)` maps a drawn inference to its goal.
  - The finished derivation is `finished(&limits, stop)`. It exports as every ordinary derivation does: text, LaTeX, Typst, SVG, PNG, PDF, and Rocq, which checks first and refuses an open goal.
- **The bindings' JSON:** the state is `{"version": 8, "logic", "sequent", "inferences", "history"}`, read by `wire::upgrade::<ordinary::Interactive>`. The sequent and the derivation read the same way, and the outcome serializes.
  - A front end that keeps the state between stateless calls reads and writes it per call. Undo then works as live, since the history is in the document.
  - `hint` returns a `Step` that `apply` takes.
- **The view settings JSON has a new key,** `"structural"` under `view`, default `"admissible"` (additive; an old settings document reads). 49b adds its flag and the manual's line, and decides whether the command draws `decide`'s derivation through `Derivation::shown` (today it draws it explicit).
- **The web module's size** is 49b's to measure against step 47's figure. This session adds about 2 000 lines to `linlog::ordinary` and 500 to `serialize`.
- **The ratchet's margins** (`derivation-chain-64` +1.95 %, `search-wide-m2-256` +1.76 %, `ordinary-pigeons` +1.65 %) are unchanged or small, but thin.

### Checklist

- [x] Read the brief, the prompt (with "What steps 47 and 48 left you"), `conduct.md`, `core-ordinary.md`, `core-derivations.md`, the research note's candidate 3 and section 5, `later.md`'s entries, `wire.rs`.
- [x] Workspace `../linlog-step49a` on `main` (`uuzskpnu`).
- [x] 1: the goal state, its operations, its JSON at level 8, behind `interactive` and `serialize`, with the bounds; one focused test per behaviour (`ozrwqnpp`).
- [x] 2: the structural rules explicit or admissible as `ViewOptions::structural`; "unknown" with its reason on `close`/`hint` (tested).
- [x] The rules (`core-ordinary.md`, `core-sequents.md`, `core.md`) and the guide's library section.
- [x] The reviewer's round: four findings, all fixed and pinned (`nnxwnlsu`).
- [x] The gate at `mtvmuqvn`, `vukwtlxz` and `vxtzvymu`: exit 0 each time, both `cargo hack` runs inside it, and both alone with warnings denied.
- [x] The target sets 49a and net-49a against 47 and net-47: no verdict differs, every counter equal (`mtvmuqvn`).
- [x] The supervisor's correction: LJ's `→L` shares its context through hidden contractions, the admissible view the default, a focused test, and the reviewer's second round (`vukwtlxz`). Its bound finding was fixed (`vxtzvymu`).
- [ ] `nix flake check` at the last commit.
- [ ] This report committed; `jj st` clean; the last message to `linlog-orchestrator`.
