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
- [x] `nix flake check`: exit 0 at `kousyoxv`, the code's last change, and run again at the change that ticks this box, which changes this file alone.
- [x] This report committed and `jj st` clean. The last message to `linlog-orchestrator` is sent after this commit.

## Session 49b: the command and the bindings

### Outcome

The command and the web bindings now prove an ordinary sequent step by step in LK and LJ, over 49a's goal state `linlog::ordinary::Interactive`.

- **The command**: `linlog interact --logic classical|intuitionistic|minimal SEQUENT` (file `cli/src/interact/ordinary.rs`) runs a session with the linear session's commands where they mean the same: `goals`, `rules`, `apply`, `undo`, `close`, `show`, `proof`, `save`, `load`, `help` and `quit`. `cut` and `invertible` are refused by name. A formula is named by its side and position, `L0` or `R1`; a rule by its name or an ASCII spelling (`andR`, `->L` or `impL`, `notR`, `iffL1`, `falseL`, `WL`). `proof` checks the finished derivation and prints it as an LK or LJ tree in every format of `prove`: text, JSON, LaTeX, Typst, SVG, PNG, PDF, and Rocq. `save` writes the session, and `--state FILE` resumes a session of either kind.
- **The view's structural option**: the flag `--structural admissible|explicit` on `prove` and `interact`, onto the settings key `view.structural` (admissible by default, the library's `DEFAULT_STRUCTURAL`). `prove --logic` now draws its derivation through it too; a certificate is always explicit.
- **The bindings** (file `web/src/ordinary.rs`): `ordinary_start`, `ordinary_view`, `ordinary_rules`, `ordinary_apply`, `ordinary_undo`, `ordinary_close`, `ordinary_hint`, `ordinary_draw` and `ordinary_export`, stateless JSON in and out as the linear calls are. `about` lists the suites with their names, the prefix of their calls, their parameters and the settings that act on them.
- **The library** gained one entry point: `FromStr` for `ordinary::Rule`, so that the command reads rule names as the library spells them and in ASCII.
- **The manual** (`docs/src/ordinary-logic.md`, a new section "Step by step"; `docs/src/interactive.md`; `docs/src/status.md`) and **the guide for AI models** (`skills/linlog/reference.md` sections 13, 16 and 17, one sentence in `SKILL.md`) changed in the same commits, every example run by `cli/tests/readme.rs`.

The commits, in the workspace `../linlog-step49b` on `main`:

| change | subject |
|---|---|
| `xzkoytro` | Prove ordinary sequents step by step in the command |
| `vmrsorxs` | Bless the behaviour lock for ordinary sessions and --structural |
| `qzorzsym` | Add an ordinary tab's calls to the web bindings |
| `umllmppw` | Run the target sets at 49b: every decided row's counters equal 47's |
| `tqtszpkr` | Hint the first steps of an ordinary goal up to a rule that is not structural |

This report is the change after them.

### What was checked, and by what

- **`cargo test -p linlog-cli --test readme`**: 7 passed, among them `readme_examples` (README and the manual), `guide_examples`, `guide_names_what_exists` and `the_summary_keeps_its_budget` (the summary's body at 7 996 bytes and 94 lines after the new sentence; a first wording passed 8 000 and was shortened).
- **The session test** `ordinary_session` (`cli/tests/cli.rs`) pins a scripted session's exact output: a split `→L` in LJ, `WR` refused with its reason, undo, save, `--state`, the checked proof and the exit statuses 1 and 0, and an axiom with a context drawn as G3's and under `--structural explicit`. Its first version shared a temporary directory's name, `cli-ordinary`, with another test, which removed it while the session was resumed: the gate's parallel run failed where a run of the test alone passed. It has a directory of its own now.
- **The settings test** `each_flag_overrides_its_key_in_a_file` has `--structural` on `prove` and on `interact`, the file's value `explicit` against the flag's `admissible`, so that the flag's default is shown not to override a file.
- **The behaviour lock** changed in its own commit (`vmrsorxs`): the three help texts, and `file-tptp`, whose derivation of Peirce's law now draws the axiom `p ⊢ q, p` with its context instead of `WR` over `p ⊢ p`.
- **The bindings' tests** (`web/tests/ordinary.rs`, five, one per behaviour the page relies on: `about`'s suites, a proof by steps exported and drawn with an id per inference, the structural setting, `close` and `hint`, malformed input refused with its reason) pass natively and under Node: `nix build .#checks.x86_64-linux.web-wasm-tests`, exit 0, 5 passed and the existing 28 passed (at `umllmppw`; again inside the final `nix flake check`, after the hint's fix).
- **The gate** (capped, cores 2–5): exit 0, `gate: passed`, at `umllmppw` and again at `tqtszpkr`, the last change of code. The first run failed in clippy (`assert_is_empty` in the new web test), the second in the session test above; both causes were fixed in the commits they belong to. The ratchet's journeys stand where 49a left them: `search-wide-m2-256` +1.76 %, `derivation-chain-64` +1.95 %, `ordinary-pigeons` +1.65 %.
- **The target sets**, run at `qzorzsym` and committed in `umllmppw`, each with `linlog-bench summary --against … --counters`:
  - `bench/targets.sh 49b` against `bench/targets/47.csv`: no verdict differs; "Of the 145 problems both decide alike on one thread, 0 differ in nodes, splits, memo_hits, memo_entries, links, tests".
  - `bench/net-targets.sh 49b` against `net-47.csv`: no verdict differs; 0 of 64 differ.
- **The manual's checks**: `nix build .#checks.x86_64-linux.manual .#checks.x86_64-linux.manual-links`, exit 0.

### The web module's size against step 47

Measured as step 47 measured it: the package `nix build .#linlog-web`, then `stat`, `gzip -9` and `brotli -q 11` of `linlog_web_bg.wasm`; and `bench/footprint.py wasm` over twiggy's CSV of the same build with its names kept (`cargo build --release --target wasm32-unknown-unknown`, `wasm-bindgen`, `wasm-opt -O3 -g`, capped on cores 2–5).

| module | raw | gzip -9 | brotli -11 |
|---|--:|--:|--:|
| step 47 | 6 913 020 | 2 603 759 | 1 854 124 |
| `main` after 49a | 6 925 937 (+0.19 %) | 2 605 392 (+0.06 %) | 1 851 296 (−0.15 %) |
| 49b | 7 148 412 (+3.40 %) | 2 689 130 (+3.28 %) | 1 911 671 (+3.10 %) |

`linlog_web.js` grew from 34 908 to 54 402 bytes raw (3 608 at brotli). The growth beyond a few percent is this session's, and it is the second suite becoming reachable from the module: 49a's goal state was in the library but not called by the bindings, so the linker dropped it. Against step 47's breakdown (6 890 629 bytes of code and data then, 7 126 149 now, +235 520):

| part | step 47 | 49b | growth |
|---|--:|--:|--:|
| the bindings' shims (no crate) | 393 608 | 489 278 | +95 670 |
| `linlog` | 1 655 716 | 1 742 104 | +86 388 |
| of it, `linlog::ordinary` | about 4 600 | 67 229 | about +62 600 |
| `linlog_web` | 217 538 | 272 374 | +54 836 |
| `core` | 815 989 | 858 250 | +42 261 |

The shims are wasm-bindgen's glue for the nine new exports, each taking and returning strings and throwing the failure's JSON. `linlog::ordinary` holds the goal state, the decision through the embeddings, the read-back, its checker, the admissible view and the ordinary Rocq writer, none of which the bindings reached before. `linlog_web` holds the nine calls and their JSON forms, and `core` the generic instances they use. Nothing here can be trimmed without dropping a call the page needs. The PNG and PDF renderers' crates are still the largest share (37.7 %).

### Decisions

- **A formula is named by side and position (`L0`, `R1`), not by one numbering across both sides** as the linear intuitionistic session prints it. The library's refusals name a side and a position (`the goal has no formula 3 left of ⊢`); with one numbering they would have named other numbers than the user typed. The bindings take `side` and `position` as two arguments for the same reason.
- **Rule names come from the library** (`ordinary::Rule::from_str`), with word spellings beside the symbolic ones: the session's word splitter, shared with the linear session, takes `\` as an escape, so `/\R` works only in quotes, and `andR` always does. Rocq-style names were not added; the serde form stays the exact name.
- **`prove --logic` draws through `view.structural`** (49a left this to 49b). One derivation is then drawn alike by `prove` and by a session, and the default reads as G3, as the session's admissible view does. The behaviour lock's one changed derivation shows the difference.
- **An ordinary `close` searches on one thread unless `--jobs N` asks for a count from the start (departs from the design).** The linear session's `close` races one thread against a pool after `--pool-after`, as `prove` does. The ordinary `close` goes through the library's `Interactive::close` and `ordinary::decide`, which have no such race; with the command's default `jobs: auto` it would have taken every core from the start. The library is unchanged, so this is the command's choice; a race there would need `decide` to take a search function, which is step 67's to weigh.
- **`--structural` is not refused on a linear session or `prove` without `--logic`**: a linear derivation has no admissible view (its structural rules are `--compact`'s), and `--classical-sides` is likewise ignored where it does not apply. `--invertible`, `-i`, `-a` and `--mix` are refused beside `--logic`, since they would change what the session proves.
- **No `hint` command in the command.** The linear command has none either (its hint is the bindings'); the library's `hint` is in the bindings' `ordinary_hint`.
- **The bindings' ordinary calls are a module of their own**, not a branch in each linear call: the session documents, the step's form and the addressing differ, and none of the linear view's keys (the fragment, the nets, the cuts, the invertible steps) applies. `about` names the prefix of each suite's calls, so the client's registry maps a suite to its calls without a table of its own.
- **`ordinary_view`'s goals are text by side** (`{"goal", "left": [..], "right": [..]}`), a formula's position its index; `inferences` follows `view.structural`, so that the drawing's `i<n>` names its goal in either view.

### Options, and how each front end sets them (D15, D16)

- **`view.structural`** (`ViewOptions::structural`, default `admissible`): the command's `--structural` on `prove` and `interact`, a settings file's `view.structural`, the page's settings JSON as for every view key.
- **`ordinary.translation`** (the translation `close` and `hint` decide a goal by): the command's `--translation` (or a settings file's key), the page's settings JSON.
- **The bounds**: the settings' `limits` as for every call, and the command's `--derivation-limit`, `--occurrence-limit`, `--memory-limit`, `--timeout`, `--work-limit`.

### What the goal state's front ends copied from the linear ones

For the kernel crate (step 67), as 49a recorded for the library:

- **Shared, not copied, in the command**: the loop of commands (`run` over a `Commands` trait), the word splitter, the reading of `show`'s and `proof`'s arguments and the writing of their output (`show_target`, `so_far`, `proof_target`, `proof_written`), the session file's reader (`load`, which tells the two kinds by the `logic` key), the standard-input refusal, and the verdict line of `prove --logic`.
- **Copied in shape**: the ordinary `Session`'s `goals`, `opened`, `close` (the deadline, the notice, the refused graft's message) and `load`'s refusals of another sequent or logic follow the linear `Session`'s. The bindings' `Changed`, `View::of`, `session_of`, `written`, and the `Reopened`/`Closing`/`Hint` answers follow `calls.rs`.
- **Different by nature**: the addressing (side and position), the step's form (a split by side instead of `left`), no invertible-rules run, no cut, no nets, no first-order part.

### Where quantifiers go (D17)

The command's `apply` would take a witness after an `∃R` or `∀L` as the linear session's does (`apply G R0 ∃R TERM`), and the bindings' `StepForm` would gain `witness` as `calls::StepForm` has it; the goal line and the view print a formula as text already.

### The reviewer's round and the answers

A fresh-context reviewer (Opus 5.5, high) judged three criteria: malformed input to the command and the bindings answered with an error and its reason, never a panic or a hang; every call within the bounds of the settings; every new example and claim of the manual and the guide real. It ran the readme test and the bindings' tests, a probe program over every ordinary call, and the command on bounded inputs (a sequent of 10⁶ nested negations, a crafted `--state` document of 320 000 structural steps, `close` under each bound). It found **no break of the three criteria**. It reported two things outside them:

1. **Following `ordinary_hint` could loop for ever.** On `a ∧ (b ∨ c) ⊢ (a ∧ b) ∨ (a ∧ c)` in LJ the hint was `CL`, and the hint on the contracted goal `WL`, which gave back the first goal; likewise `⊢ ¬¬(a ∨ ¬a)` (intuitionistic and minimal) and `(a → b) → a, a → b ⊢ b`.
   - Answer: it holds. The library's `hint` read only the root rule of the derivation found, which can be structural; the linear bindings' hint had the same fault and returns the structural steps with the rule that uses them. Fixed in the library in `tqtszpkr`: `Hint::steps` (it replaces `Hint::step`, before the first release, D18) is the run of weakenings and contractions at the root and the first rule above it, read off one derivation by applying them in a copy of the state. The bindings answer `"steps"`, which the page applies in turn. `following_the_hints_proves_a_sequent` follows the hints on the reviewer's four sequents to a checked derivation; before the fix the first alternated.
2. **A session's PNG or PDF render is bounded by the pixel and memory limits and by Ctrl-C, not by the time limit**: `show d.pdf` of a sequent 32 000 negations wide ran past a minute. This is the shared rendering code of every session, linear ones included, older than this step, and `prove` stops the same render at `--timeout`.
   - Answer: it holds, and it is not this step's: recorded below as a follow-up, not fixed.

### Left open, for later sessions

- **A session's render and the time limit**: `show FILE.png|pdf` and `proof --png|--pdf FILE` in `interact`, linear or ordinary, render without the deadline `prove` gives its render (the reviewer's second note). A session has no deadline outside `close`; the fix is a deadline for each command that renders, in `interact/mod.rs`'s `so_far` and in `show::shown`'s caller.
- **An ordinary `close` with a pool**: see the departure above; the library's `decide` would take the race as a function, or the command would translate and race itself.
- **No `hint` command** in either session of the command.
- **Pre-existing, still open from 49a**: the Rocq writer's term for LJ's unary `¬L` (a document can hold it; no derivation the library makes uses it).

### What the next sessions must know

- **49c (the page)** takes the ordinary tab's calls from `about()`'s `suites`:
  - `{"suite": "linear", "name": "Linear", "calls": "", "parameters": [{"name": "mode", "values": [the modes], "default": "classical"}], "settings": ["interactive.invertible", "view.classical", "view.compact"]}`;
  - `{"suite": "ordinary", "name": "Ordinary", "calls": "ordinary_", "parameters": [{"name": "logic", "values": ["classical", "intuitionistic", "minimal"], "default": "intuitionistic"}], "settings": ["ordinary.translation", "view.structural"]}`. The default logic is the library's (`ordinary::Options::default()`), intuitionistic; the page may start a tab classical by passing the logic it wants.
- **The calls and their JSON** (`plan/web/README.md`'s table has every row):
  - `ordinary_start(sequent, logic, settings)` → `{"session", "view"}`; the sequent in the ordinary syntax (`->`, `/\`, `\/`, `~`, `<->`, `true`, `false`) or the ordinary sequent's JSON.
  - `ordinary_view(session, settings)` → `{"logic", "calculus", "complete", "steps", "goals": [{"goal", "left": [text], "right": [text]}], "inferences"}`; a formula's position is its index on its side; `inferences[n]` is the goal of the drawing's `i<n>` under the settings' `view.structural`, so the page asks the view again when it switches the setting.
  - `ordinary_rules(session, goal, side, position, settings)` → `[{"rule", "split"}]`, `side` `"left"` or `"right"`, `split` true for a rule with two premises.
  - `ordinary_apply(session, goal, step, settings)` → `{"session", "view", "opened"}`; a step is `{"side", "position", "rule", "split": {"left": [positions], "right": [positions]}}`, the rule by its exact name (`"→L"`, `"∧L₁"`), `split` left out to share the context (G3's form; the default a first course wants). A refusal throws `{"code": "step", "kind": "invalid", "message", "details"}`, `details` the library's `StepError` with its own `kind` tag (`rule`, `logic`, `no_partner`, `succedents`, `split`, `no_split`, `size`, …) and its sides and positions, which the page words itself.
  - `ordinary_undo` → `{"session", "view", "reopened"}`; `ordinary_close` → `{"session", "view", "outcome", "refused"}`, `outcome.verdict` `valid`, `not_valid` or `unknown` with `reason`; `ordinary_hint` → `{"outcome", "steps"}`, the steps applied in turn, each on the goal the one before opened.
  - `ordinary_draw` → SVG with `i<n>`; `ordinary_export(session, format)` with `text`, `latex`, `typst`, `svg`, `rocq` (the finished derivation, checked); PNG and PDF through `render` on the SVG, as for a linear tab.
  - The session document is `ordinary::Interactive` at wire level 8 (`{"version": 8, "logic", "sequent", "inferences", "history"}`); the link and the stored proofs carry it under the suite field 49c adds.
- **The page draws LK and LJ, never the linear image**: nothing in the ordinary calls exposes the image, except `outcome`'s `engine` and `statistics` (of the search on the image), which the page may show or leave out.
- **The command**: a session of LK or LJ names formulas `L0`/`R1` and reads rule names through `ordinary::Rule::from_str`; `cli.md` holds the rest.
- **The ratchet** is where 49a left it (`derivation-chain-64` +1.95 %, `search-wide-m2-256` +1.76 %, `ordinary-pigeons` +1.65 %).

### Checklist

- [x] Read the brief, the prompt, 49a's section of this report, `conduct.md`, `core-ordinary.md`, `cli.md`, `web.md`.
- [x] Workspace `../linlog-step49b` on `main` (`zspyrqny`).
- [x] Requirement 2: `linlog interact --logic classical|intuitionistic|minimal` with the linear session's commands where they mean the same, the derivation printed as an LK or LJ tree and exported, `--structural` and `view.structural`; the manual and the guide in the same commit (`xzkoytro`); the behaviour lock in its own (`vmrsorxs`).
- [x] Requirement 3: the bindings' ordinary calls and `about`'s suites, with their tests natively and under Node (`qzorzsym`).
- [x] `cargo test -p linlog-cli --test readme`: 7 passed.
- [x] The web module's size against step 47 with `bench/footprint.py`: +3.40 % raw, +3.10 % at brotli, explained above.
- [x] The target sets 49b and net-49b against 47: no verdict differs, every decided row's counters equal (`umllmppw`).
- [x] The manual's checks: exit 0.
- [x] The reviewer's round: no break of its criteria; the hint loop it noted fixed (`tqtszpkr`), the render's time limit recorded as a follow-up.
- [x] The gate at `tqtszpkr`: exit 0, `gate: passed`.
- [x] `nix flake check`: exit 0 at `umllmppw`, before the hint's fix (all checks passed, web-wasm-tests among them). It runs again at the commit of this report, which changes nothing else after `tqtszpkr`; its exit status is in the last message to `linlog-orchestrator`.
- [x] This report committed and `jj st` clean. The last message to `linlog-orchestrator` is sent after this commit.

## Session 49c: the page

### Outcome

The web page now proves ordinary propositional logic. A tab's calculus mark opens a popover that lists the registered suites, Linear and Ordinary. Ordinary proves sequents of classical, intuitionistic and minimal logic step by step in Gentzen's LK and LJ, through linlog's `ordinary_…` calls at the lock of 49b (`864bca2c`).

- **The suite registry** (client file `src/lib/suites.ts`): a tab's calculus is a `Calculus`, its suite, its method and the suite's parameter as linlog's `about` names them, in place of the closed linear `Mode`. `SUITES` holds each suite's name, tip, methods, parameter, the values the page offers, the calculus a new tab starts in and the prefix of its calls. `suites.test.ts` holds the registry to `about()`'s `suites`.
- **The link and the stored proofs carry the suite.** The link is at version 2, `#v=2&u=<suite>&m=<parameter>&s=…&d=…`. The shelf of stored proofs is at version 2, each proof with its `calculus`. A link of version 1 and a shelf of version 1 read as the linear suite in their mode, each tested in `link.test.ts` and `shelf.test.ts`, and on the page in `tests/ordinary.spec.ts`.
- **The Ordinary segment** (client file `src/lib/ordinary.ts`, and branches in `state.tsx`, `Stage.tsx`, `BottomBand.tsx`, `TopBand.tsx`, `Calculus.tsx`): a segmented control Classical | Intuitionistic | Minimal (marks LK, LJ, LM) and a switch "Structural rules", which sets `view.structural` to draw every weakening and contraction. The stage draws the LK or LJ derivation that `ordinary_draw` writes, never the linear image. A click on a formula lists `ordinary_rules`, and a rule applies at once (a rule of two premises shared, G3's form). The dock has Close goal, the hint ladder and the open-in-a-new-tab button. The bottom band has undo on a branch, redo, the step list, close all, and the exports: SVG, PNG, PDF, LaTeX, Typst, text, the session and the Rocq certificate over `Prop`.
- **Examples in the sequent field's menu** (client file `src/content/examples.json`, CC BY): fifteen sequents, a lesson's worth, from modus ponens through distribution, contraposition, ex falso in minimal logic and the excluded middle in LJ and in LK, to Peirce's law and both halves of De Morgan. Each holds linlog's verdict in all three logics, and `src/content/examples.test.ts` checks every verdict with the module within a hundredth of the page's budget.
- **README** says that the page proves ordinary logic and gives the store's true counts: 452 problems, 407 provable, and eleven lessons each with a worked proof. About, `llms.txt`, the page's description and lesson 11 (which said the page had no ordinary mode) say so too.
- **The four entries of `plan/later.md`** this step names, all answered:
  - the phone's goal bar and dock: fixed;
  - the tour's 39-pixel stop: fixed at its cause;
  - the replayed classical cut: no longer seen at linlog's `main`, now pinned by a test;
  - the compact net's edges: fixed in step 39, seen on the page.

Learn and Practice stay linear, as the prompt says.

The client's commits, in the workspace `../linlog-prover.github.io-49c` on `main` (signed by the hook; commit ids as it reported them):

| change | commit | subject |
|---|---|---|
| `suutnlrw` | `a5b638d70728` | flake.lock: Update (linlog `fe0cb97a` → `864bca2c`) |
| `kztzlwwt` | `5012acf2ef68` | Choose a tab's calculus among registered suites, and prove ordinary logic in LK and LJ |
| `knzvqkqu` | `5d3db23d5c0f` | Lay the formula buttons only over the current session's drawing, and keep the phone's goal bar on one row |
| `vxqzpkqn` | `5c0f003ca1d0` | Say in README that the page proves ordinary logic, and correct its counts |
| `usulopkw` | `37715afd0be1` | Measure an ordinary tab's controls in a walk of their own |
| `vnxtqrox` | `c06ebe7dcecd` | Answer the review of the page's suites |

### Decisions

- **The registry's shape: `Calculus` is a discriminated union, `{suite: "linear", method, mode}` or `{suite: "ordinary", method, logic}`.** Each suite's parameter field carries the name `about` gives it. TypeScript then narrows a linear call's mode and an ordinary call's logic at every site. A record of string parameters would have hidden a linear mode handed to an ordinary call.
- **The method is in the type and in the shelf, not in the link.** Each suite has one method today (`sequent`), so the link writes the suite and the parameter alone, and `calculusOf` reads a missing method as `sequent`. The second method adds a field `k` to the link at the same version. Writing `k=sequent` into every link now would lengthen each link for a choice no visitor has.
- **The registry is in the page, and a test holds it to `about()`'s `suites`.** I did not build it from `about` at run time. The page needs its suites' names, tips and starting calculus before the module loads (D35: the first paint does not wait on the module). The test checks each suite's name, its call prefix, its parameter, that the page's values are among linlog's (the linear suite's cyclic and Lambek modes are not on the page), and that a new tab starts in linlog's default.
- **An ordinary view is read into the shapes the stage already shows** (`asView`). A goal's formulas are numbered left of `⊢` first, then right, as the drawing's `i<n>-<p>` ids number them. So the stage's buttons, the rules popover, the hint ladder, the dock and the step list work unchanged. `ordinaryStep` turns a place back into a side and a position.
  - A hint's first step carries its place, so that the ladder points at the formula.
  - Its later steps carry their `side`, since each acts on a goal the one before opened, whose formulas the page has not seen.
  - The adapter is small, it keeps one stage (D29: one module), and it copies none of the stage's 1 900 lines.
- **A rule of two premises is applied shared in an ordinary proof; the page offers no split there.** This is linlog's default and G3's form, which 49a's report calls the default a first course wants. A split by hand stays possible in the command (`interact`, `Step::split`).
- **Undo on a branch (the author's D13) cuts the step's subtree out of the session document** (`retract` in the client's `ordinary.ts`).
  - `ordinary_undo` takes back the session's last step, as the library's undo does.
  - The linear tab meets D13 by replaying the other steps. For the ordinary tab, rebuilding steps from inferences would have to guess the form of a shared LJ `→L`, whose contractions sit hidden below the rule.
  - Instead, an ordinary session's history lists each step's goal, and a step's inferences are that goal's subtree, appended as one block after it. Taking the subtree of the step that opened the active goal out of the document, and numbering the rest anew in their order, leaves the document the other steps would have made.
  - linlog's reader (`ordinary_view`, which replays and checks every inference and the history's order) must accept it before the page shows or stores it; a refusal leaves the session as it was and says so. No search runs again, so a closed goal stays exactly as closed.
  - Redo restores the session as it was before each undo (`Proof.later`, kept with the stored proofs). The step list goes back by `ordinary_undo`, which is exact for the last steps.
  - `linlog-orchestrator` agreed to this design on 2026-11-08, on three conditions: nothing unaccepted is shown or stored; the dependency is recorded (below and in the client's rules); and a retraction call in the bindings is named as the cleaner option (below).
- **An ordinary graft is marked by the path the page saw the search close** (`Proof.engines`), not by `searchMade`. An ordinary session cannot tell a graft from a step's own run of weakenings or contractions, so `searchMade` would mark a shared `→L`'s contraction as the search's. A link therefore opens an ordinary proof with its grafts unmarked. The linear tab's marks of a graft are read off the document and survive a link.
- **The outcome's engine is shown in an ordinary tab** (the chip beside a note, the label beside a graft, the step list). The engine is the one that searched linlog's translation. 49b's report leaves this to the page. An engine's name is not the linear image, and the chip's link to Learn's reference says what the engine does. Settings' engine choice is the linear suite's, and an ordinary tab says so there. Ordinary calls are never sent a forced engine (`ordinaryJson` leaves `search.engine` and the invertible rules out).
- **The examples are a content collection of their own** (`src/content/examples.json`, Astro's `file()` loader), not entries of the problem store. The store's schema, its flake check (`problems.py`, which runs linlog's command on linear sequents) and Practice are linear, and step 63 keys them by suite. The menu sits beside the field of a tab whose suite Practice does not serve, which today means only ordinary tabs. The examples' verdicts are checked by the module in Vitest, not by the command in the flake's `problems` check. Extending that check to ordinary sequents is step 63's, when the store is keyed by suite.
- **A change of suite reads the field's text anew in the other suite's syntax.** A linear session's sequent document cannot be started as an ordinary one. Text that does not parse in the other syntax is marked in the field as a typed error is. A change of logic within the ordinary suite starts from the session's sequent document, as a change of mode does in the linear suite.

### What the later.md entries came to

- **The phone's goal bar and dock** (41d, 42k): on a Pixel 7 the bar was 380 pixels wide with its items on three heights; the open-in-a-new-tab button had wrapped since "Cut" joined the bar. On a phone the navigator now says "Goal N", with the count of open goals left to the tab's status, by the page's `wide-only` class. The desktop's wording and the tests that read it are unchanged. `tests/phone.spec.ts` ("the goal bar keeps one row …") measures one row with Cut in it.
- **The tour's flaky stop** (41d, 42k): it did not reproduce in 25 runs of the phone walk (10 at four workers, 15 at one), before the fix.
  - The cause was found by reading. When the active proof or its session changes, the stage's active-goal effect re-decorated at once and laid the new view's buttons over the old drawing, whose `i<n>-<p>` ids the new view may match, until the new SVG arrived.
  - Those buttons were sized at a zoom fitted to the other drawing, and the zoom floor that keeps a formula a control wide holds only for the drawing it was fitted to.
  - The tour's first stop opens its example in a new tab, and the walk measures as soon as the tour's caption shows, so it could land in that window.
  - The stage now lays buttons only over the drawing made for the current session (`hostSession`). The walk passed in every run since: four local runs (two of them the full e2e suite) and the flake check's last two runs in its sandbox.
  - The flake check's first run failed in the phone walk at a step this session had appended (the ordinary tab's controls), past the sandbox's minute. There the tour's caption was still open: the Escape the walk presses after the tour's measure did not always close it in the slow sandbox, and the caption covered the calculus popover. The ordinary controls now have a walk of their own at both widths (`usulopkw`). The Escape race is older than this step, and the walk never relied on it (it reloads next); it is recorded below.
- **The replayed classical cut** (41d): it no longer shows one-sided. At linlog's `main` the sequent's JSON records each occurrence's side (`antecedents`, step 40b's surface), and a replay's `start` keeps it, as a probe of the module showed. `tests/cut.spec.ts` ("a replayed classical cut is drawn two-sided as before the replay") pins it on the page: Two-sided on, a cut, a step, an undo that replays, the drawing's texts equal. I did not run the test against the old lock, so whether it would have failed at 41d's lock is not shown.
- **The compact net's edges** (38b): fixed in linlog by step 39 (`xmlrsppp`, the edges begun at each font's own depth, tested by `net_edges_clear_the_font_s_descenders`). A screenshot of `A ⊗ B ⊢ B ⊗ A`'s compact net on the page at 195 % shows every edge begun clear below its literal. The entry is ticked with that reason.

### Departures from the design

None. The per-branch undo of D13 holds in ordinary tabs, by the retraction above, with the supervisor's agreement. The design's D2 and D57 (Ordinary on the tab, never mixed into the linear page's controls) are what the segment does.

### What the next sessions must know

- **The registry's shape, for steps 56, 62 and 65** (client file `src/lib/suites.ts`):
  - A suite is an entry of `SUITES`: `key`, `name`, `tip`, `methods`, `parameter` (the field `about` names), `values` (the page's, a part of linlog's), `start` (a new tab's calculus) and `calls` (the prefix).
  - A tab's calculus is a variant of `Calculus`, `{suite, method, <parameter>}`.
  - Adding a suite (65's modal logics) takes a variant, an entry, its calls in `api.ts` and the worker's table, `markOf` and `calculusWords`, `calculusOf`'s construction, and a branch wherever a call is made: `state.tsx`'s actions branch on `ordinaryOf(p)` today, and a third suite is the moment to make that a lookup by suite. `suites.test.ts` holds the entry to `about()`.
  - Adding a method (56's tableaux and truth tables, 62's natural deduction) widens `Method`. A method of its own needs a field `k` in the link (`link.ts`, at version 2, read as `sequent` when absent; the shelf stores it already). It also needs a view kind on the stage: the stage draws trees, and a Fitch list or a table is a second host element (the research note's web-front-end-6).
- **The ordinary tab rests on linlog's layout of the ordinary session at wire level 8**: each history entry is the goal a step expanded, and a step's inferences are that goal's subtree, appended as one block after it. The client's `retract` cuts subtrees out of that document for undo on a branch, and `ordinary.test.ts`'s property against the real module catches a change of the layout: every retraction is read by `ordinary_view`, and equals `ordinary_undo` when it takes back the last step. A change of the layout in linlog must change the client too.
  - **The cleaner option, for step 63's suite protocol**: a call `ordinary_retract(session, goal)` in the bindings, in Rust where the format lives, so that a third suite does not copy the surgery into its client. Not built now.
- **The ordinary tab's grafts are marked by the page's record** (`Proof.engines`), not read off the document. A link opens them unmarked.
- **Learn and Practice are linear**: `Example.mode` and the problem store's `mode` are linear modes, and `prover.example` opens them with `linear(mode)`. Step 63 keys them by suite, and the examples of `src/content/examples.json` (their verdicts in all three logics) are ready to move into an ordinary store then. The flake's `problems` check covers the linear store alone; the examples are checked by Vitest.
- **The exports' "Not derivable" item is hidden in an ordinary tab**: linlog's ordinary certificate of non-validity (`rocq::write_not_valid`) is not among the bindings' calls (`plan/web/README.md`'s table).
- **An ordinary `close` from the page searches on one thread**, under the page's work budget (the tab's `jobs` is 1). Its outcome's `engine` names the engine that searched linlog's translation, which the chip and the step list show.

### The reviewer's round and the answers

A fresh-context reviewer (`linlog-49c-reviewer`, Opus 5.5, high, cores 10–13) judged five criteria:

1. old links and stored proofs open as linear sessions;
2. the suite field round-trips;
3. an ordinary tab never reaches the linear image or a linear call;
4. the per-branch retraction is right and validated before it is shown;
5. the e2e tests cover the segment.

It reproduced five breaks in a browser against the built site. All five hold, and all five are fixed in `vnxtqrox`:

1. **A stored proof in a linear mode the switches do not build made the whole shelf unreadable, and its visit's proofs were deleted**, a version-1 shelf included. A session pasted in `cyclic` or `lambek` opens as such, the registry refused that mode when the shelf was read, and `parseShelf` refused the whole shelf.
   - Answer: the registry reads every linear mode linlog's `about` lists. `Mode` gains `cyclic`, `lambek` and `lambek-star`, with marks CyLL, L and L*. The switches still build six.
   - `parseShelf` leaves out a proof it cannot read alone.
   - `shelf.test.ts` pins a version-1 shelf with a `cyclic` proof, and an unknown suite beside a readable proof. The test that pinned a whole shelf refused for one unreadable proof now pins that proof left out alone, as that is the change.
2. **"Fill from" was offered on an ordinary tab and sent its session to linlog's linear `fill`.** An ordinary LJ goal's formulas key alike to an ILL tab's.
   - Answer: `fillable` and `fill` are kept to linear proofs. The e2e test "an ordinary goal is never offered a fill from a linear tab" covers it.
3. **The invertible switch sent ordinary sessions to `invert_all`**, which refused them, so nothing showed.
   - Answer: `setInvertible` skips proofs of other suites. There is no test, since nothing is visible.
4. **After an undo, a step taken by hand where a search's graft had been was drawn and listed as the search's.** `Proof.engines` kept the retracted graft's entry.
   - Answer: undo and the step list's going back keep only the entries of grafts the session still holds (`keptGrafts`). Redo restores the session with its entries (`Proof.later` holds both).
   - The e2e test "a search's graft taken back by undo leaves no mark …" covers close, undo, redo, undo and a hand step.
5. **The structural switch with a goal open left the view's map of the drawing's ids from the other setting**, so the marks and buttons sat on a closed branch's sequent.
   - Answer: the switch is `setStructural`, which asks every ordinary proof's view again and changes the setting and the views together.
   - The e2e test "the structural switch with a goal open …" covers it; the earlier test toggled it on a finished proof only.

Each of the three new e2e tests failed with the state and popover from before the fixes and passed after them. Its gaps without a break, answered:

- **An ordinary undo that linlog's reader refuses** has no test. The reviewer confirmed by reading that it returns before anything changes. No document the page makes is refused; the property test shows the reader accepts every retraction of random sessions.
- **The version-1 shelf through `restore`** is covered by `parseShelf`'s unit tests. `restore` takes the parsed shelf as it takes a version-2 one.
- **A version-1 link with a session** is read by the unit test, and on the page by `prover.spec.ts`'s older link tests, which open version-1 links with `d=` (unchanged in this step).

The reviewer found criteria 1 to 4 sound apart from these. For the retraction, it judged by reading against linlog's `from_parts`, `record`, `graft` and `undo` that cutting a step's subtree keeps every remaining step's block contiguous, in history order and after its goal. That holds for shared rules with contractions, axioms with weakenings, grafts, and steps on several branches in any order.

### What was checked, and by what

- **The client's `nix flake check` at its last commit, `vnxtqrox` (`c06ebe7dcecd`): exit 0, "all checks passed!".**
  - Its checks: `actionlint`, `claude-hooks`, `claude-worktree-hook`, `csp`, `deadnix`, `e2e` (136 passed in Chromium, the new segment's tests among them), `licence-headers`, `licences`, `lint` (Biome), `no-style-attributes`, `pages-paths`, `playwright-version`, `problems`, `site`, `treefmt`, `typecheck` and `unit` (Vitest, 569 passed).
  - Earlier runs: at `usulopkw`, exit 0 (e2e 133 passed). At `vxqzpkqn`, exit 1: the phone walk's timeout described above, fixed in `usulopkw`.
- **Locally**, capped on cores 4–9, before each commit: `lint` (Biome with warnings as errors, then `astro check`, 0 errors), `tests` (569 passed at the end) and `e2e` with four workers (136 passed at the end).
- **The unit tests of this step**:
  - `suites.test.ts`: the registry against `about()`, and the calculus read back by suite, method and parameter;
  - `ordinary.test.ts`: positions against the drawing's ids, the sequent's text read back by the parser, undo on a branch across a split, and the retraction's property over 60 random sessions in all three logics, which asserts at least 31 retractions and at least one comparison with `ordinary_undo` per run, stable in eight repeats;
  - `examples.test.ts`: the fifteen examples in three logics each;
  - `link.test.ts` and `shelf.test.ts`: version 2, and version 1 read as the linear suite.
- **The e2e tests of this step**:
  - `tests/ordinary.spec.ts`, eight tests: the popover's suites and an LK proof with no linear sign in the drawing; Not valid in LJ and valid in LK from the examples menu; undo per branch across a split, and redo; the structural switch and the exports (LaTeX, text, Rocq, session, SVG); the link and the stored copy with their suite, and a version-1 link opening linear; and the three of the reviewer's round.
  - `tests/targets.spec.ts`, an ordinary tab's controls at 24 and at 44 pixels.
  - `tests/phone.spec.ts`, the goal bar on one row.
  - `tests/cut.spec.ts`, the replayed two-sided cut.
- **Screenshots**, read and not kept: the popover and the examples menu at desktop and phone widths, an ordinary proof in progress and finished, the phone's dock, and the compact net.
- **linlog's text checks** for this report: see the checklist.

### Left open, for later sessions

- **The ordinary graft's mark does not survive a link**: a link carries the session, not `Proof.engines`. The bindings could say which history entries were grafts; step 63's protocol may add it.
- **The tour's Escape race** (older than this step): an Escape pressed right after the tour's caption appears, while its first stop opens its example, does not always close the tour, as the flake check's sandbox showed. A visitor presses later; the walk no longer depends on it.
- **The examples are checked by Vitest, not by the flake's `problems` check**, which runs linlog's command on the linear store; step 63, keying the store by suite, takes them into it.
- **`ordinary_retract(session, goal)` in the bindings**, as said above, for step 63's protocol.
- **Settings has no row for `ordinary.translation`**: the page decides through linlog's default translation. A row is a small addition when a course asks for another.

### Checklist

- [x] Read the brief, the prompt, 49a's and 49b's sections, `plan/web/README.md`'s table, `plan/web/design.md` (D2, D35, D50, D56, D57, D58), the client's CLAUDE.md and rules, `plan/conduct.md`, and the research note's web-front-end-1 and -2.
- [x] Workspaces `../linlog-prover.github.io-49c` and `../linlog-step49c` on `main` (`864bca2c`).
- [x] The client's lock moved to linlog's `main`: `suutnlrw`, "flake.lock: Update".
- [x] The `frontend-design` plugin: loaded before the design; the page's look kept, no new tokens, the header's mark untouched.
- [x] Requirement 4: the registry (`kztzlwwt`); the Ordinary segment with classical, intuitionistic and minimal; the stage drawing LK and LJ; the exports; fifteen examples in the field's menu; README's counts (`vxqzpkqn`).
- [x] Per-branch undo in ordinary tabs (D13), agreed with `linlog-orchestrator` on its three conditions.
- [x] The later.md entries: the phone's goal bar and dock, and the tour's flaky stop, fixed (`knzvqkqu`); the replayed classical cut no longer seen and pinned; the compact net's edges fixed by step 39 and seen. All ticked in `plan/later.md`.
- [x] The reviewer's round: five breaks, all fixed and tested (`vnxtqrox`).
- [x] The client's `nix flake check` at `vnxtqrox`: exit 0.
- [x] linlog's text checks (typos, treefmt, conventions) at this report's commit.
- [x] This report and `plan/later.md` committed; `jj st` clean in both workspaces. The last message to `linlog-orchestrator` is sent after that.
