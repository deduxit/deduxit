# Step 54: intuitionistic logic decided natively, by Dyckhoff's LJT

The step's sessions each add a section here: 54a (the engine), then 54b
(the default by measurement).

## Session 54a: the engine

Opus 5.5 at `high`, in the jj workspace `../linlog-step54a`, from
`plan/54-ljt-engine.md`, on 2026-11-09; paused from 11:18 to 12:26 for
the author's stop, resumed after the checkout's rename to
`~/Projects/own/deduxit`.

### Outcome

- **`ordinary::ljt::decide`** decides every intuitionistic and minimal
  propositional sequent by Dyckhoff's contraction-free calculus LJT
  (G4ip), with no copy bound: its search ends by the calculus, within
  `limits.work` (a sequent expanded is a unit), the stop (asked every
  256 expansions) and the memory account. A proof is translated into a
  derivation of LJ, which `ordinary::Derivation::check` validates before
  it is returned; "not valid" is the search's word. It is not yet the
  command's default.
- **On ILTP**, it gives every one of the 394 verdicts the embedding
  reaches at the command's 2 s (196 problems intuitionistically, 198
  minimally, by some translation of cbn, cbv and 01) the same verdict,
  and every proof it returns passes the checker
  (`core/tests/iltp.rs`, ignored by default, run against the recorded
  oracle `core/tests/iltp/embedding.csv`: "394 verdicts of the embedding
  agreed", 98 s in release). 390 of the 394 take under 2 s; SYJ202+1.005
  (a pigeonhole theorem) and SYJ208+1.007 (a non-theorem) take 35 to 93 s.
- **The checker gained Gentzen's cut in LJ** (departs from the design,
  below), the only change to what is trusted.

The commits, in order: `sxlozrrw` (the cut), `morqmskz` (the engine and
its translation), `lkrqkywm` (this report at the pause), `xuovvqpk` (the
search's prunes), `ulpsvomt` (the ILTP test and its oracle), `otpoxkoq`
(the ranking visits each node once), `nusnsoqy` (the status chapter and
the guide), and this report's last commit.

### Decisions

- **Cut in the checker rather than G4ip's rules or a cut-free
  translation** (departs from the design, which named either a sourced
  translation into LJ or the checker gaining G4ip's rules). LJT's
  `∧→L`, `↔→L`, `∨→L` and `→→L` are derivable in LJ only with a cut: a
  cut-free LJ derivation has the subformula property, and their premises
  hold formulas (`A → (B → C)`, `B → C`) that are no subformulas of the
  conclusion. Dyckhoff's correction (JSL 83(4), 2018, p. 1680) says the
  soundness half of G4ip's equivalence with G3ip "is straightforward,
  using admissibility of Cut in G3ip". Gentzen's LJ has cut as a rule,
  so the checker gains one rule whose soundness is evident instead of
  four, and the translation derives each LJT rule with a cut on the
  formula it makes. Eliminating the cuts would be cut elimination on LJ,
  with its blow-up, for no gain in trust. deduxit-orchestrator accepted
  it at the pause and asked the panel to read the cut's check and the
  translation's cuts, which round one's argument did.
- **The cut is LJ's only**: LK refuses it, since the embedding's
  read-backs have none and the classical certificate has no term for
  one. A form holding one is written at wire level 9.
- **The search, beyond the calculus** (each sound, each pinned by a
  test, the first round of the panel run before them and the second on
  them):
  - hypotheses as a set (contraction admissible, Dyckhoff and Negri
    2000), the axiom for any formula and LJT's atom rule for any
    antecedent among the hypotheses, invertible rules first and alone;
  - `→→L`'s second premise proved first: it is invertible (Dyckhoff 2018,
    Prop. 2.2), so its failure fails the sequent;
  - the `→→L` alternatives ordered by how near their consequent is to the
    goal, an order only (Korn and Kreitz's SYJ205 went from doubling with
    each size, 737 229 sequents at 15, to `15 + 5n`);
  - failure subsumption: a sequent fails without search when a failed one
    of the same goal has hypotheses entailing each of its own by
    `x ∈ Δ`, `⊤`, `A → A`, `A → C` with `C ∈ Δ` or `(A → B) → B` with
    `A ∈ Δ`; it makes no proof. Franzén's SYJ209 and the invalid Korn
    and Kreitz formulas SYJ211, which the embedding refutes only through
    the inverse method's saturation, went from `2.1ⁿ` and `3.6ⁿ`
    sequents to 542 and 25 040 at size 20.
  These three were added after the first ILTP run, which found the plain
  search running out of 6 GiB on SYJ205+1.020 and the backward search
  unable to refute SYJ209 and SYJ211 at the larger sizes.
- **The API is a module of its own**, `ordinary::ljt` with `decide`,
  `Outcome`, `Statistics` (`sequents`, `memo_hits`, `memo_entries`,
  `subsumed`) and `POLL`, beside `ordinary::decide`, whose outcome holds
  the image's linear search; 54b joins the two under the options and the
  command. No options value yet (D15): the engine has no choice a user
  would vary beyond `Limits`; 54b adds the engine's choice to
  `ordinary::Options`.
- **The ILTP test's limits**: `limits.work` at 200 million sequents, no
  bound on a derivation's size and 6 GiB of memory. SYJ201+1.004's proof
  unfolds to a derivation past the default 64 MiB estimate, which
  `decide` refuses (a refusal, not a verdict, as the embedding's
  read-back of such proofs is); step 25's check of the embedding's large
  proofs lifted the same bound. The oracle was recorded by the command
  of `main` (`deduxit prove --logic L --translation T --file
  bench/iltp/Problems --workers 2 --cores across --format json`, the
  default 2 s, cores 2 and 3 under the machine lock, 12:27 to 12:37); the
  batch writes the image's outcome, so `proved` is valid and `unprovable`
  not valid. It decides more than step 25's run (196 against 108), the
  engines having grown since.
- **Where quantifiers go** (D17): `Kind` gains the rules of `∀` and `∃`,
  and a frame's key would need the terms in scope; LJT has no
  terminating first-order counterpart, so the first-order engine would
  carry a bound of its own.
- **What it copies from the linear engines** (for the kernel crate,
  D25): the memo keyed by the sequent, an arena of proof steps shared
  between branches, the stop's poll every 256 units, the work count, and
  the memory account `linear::search::memory::Account`, used as it is.

### The panel

Step 26's panel, as named agents: the argument on Fable 5.1 at `high`
(`panel-argument`), counterexamples on Opus 5.5 at `high`
(`panel-counterexamples`), the integers on Sonnet 5.5 at `high`
(`panel-integers`), each from a fresh context with the commits, the
claims and the criteria. Round one read `sxlozrrw` and `morqmskz`;
round two read `xuovvqpk` (counterexamples ran on both).

Round one:
- **Argument**: no refutation of soundness. It stated the invariants
  from the code, checked every arm of the translation against the
  checker, the termination by the weights (with `↔` weighing
  `2w(A) + 2w(B) + 4`) and the completeness of each departure from
  G4ip. One finding: the doc of `Rule::Cut` and the rules said the cut's
  context is "shared or split", but the shared form is refused whenever
  the conclusion has a formula right of `⊢`. *Answer*: it holds, as a
  description; the doc and `core-ordinary.md` now name the split form,
  and the shared one only for a conclusion with nothing right of `⊢`
  (in `xuovvqpk`). The checker is stricter than its doc, so nothing
  unsound was accepted.
- **Integers**:
  - `Derivation::term` panics on a derivation read from JSON whose cut
    has a first premise with nothing right of `⊢` (the index
    `right()[0]` in the lambda read-back), without the check. *Answer*:
    it holds, and it is a case of a class the read-back had before: an
    unchecked `→L` with the same first premise panics at the same kind
    of index. The read-back assumes a checked derivation and its doc
    does not say so. A follow-up (below), not fixed here.
  - The translation's memory account started from nothing while the
    search's memo and steps were still held, and charged neither the
    widths nor the arena copy, nor the inference list by capacity:
    VmHWM was 1.8 times the bound on its input. *Answer*: it holds;
    fixed in `xuovvqpk` (the account starts from what the search holds,
    charges both, and the list by capacity), and round two measured at
    most 1.1 times.
  - `decide` applies `limits.work` to the search and again to the
    check's inferences, which can refuse a proved sequent, and its doc
    did not name that refusal. *Answer*: it holds as a doc gap; the doc
    now names `Refusal::Work` (in `xuovvqpk`). The behaviour is the
    embedding's own (`ordinary::decide` checks under the same limits).
  - Driven without a break: the work count, the poll's cadence, the
    search's charges, the step ids, the doubling formulas, 50 000 and
    400 000 deep on a 256 KiB stack, the tests on wasm32-wasip1.
- **Counterexamples**: no refutation, over 13.4 million sequents (26.8
  million decisions, both logics: every formula over two atoms, `⊥` and
  `⊤` up to three connectives, pairs of hypotheses, random sequents of up
  to 6 atoms and 7 hypotheses, instances of 25 schemas and their
  mutants) against its own loop-checked G3ip prover, a Kripke
  countermodel search, the truth table and the embedding, and 1.15
  million mutated derivations against the checker, none accepted for an
  invalid conclusion. It reported 20 `Unknown(MemoryLimit)` at 1 GiB on
  random sequents of 4 or 5 atoms with nested `↔` (a broad search, not a
  loop) and 60 refusals of the default derivation size: bounds, not
  verdicts.

Round two:
- **Integers**: one refutation. `heads`, which ranks the `→→L`
  alternatives, walked a consequent as a tree, so a formula whose
  operands share nodes (`Wₖ₊₁ = Wₖ ∧ Wₖ`) took `2ᵏ` steps and an
  uncharged vector of that length, polled by no stop (1 GB at `k = 28`).
  *Answer*: it holds; fixed in `otpoxkoq` (each node visited once), and
  `shared_formulas_cost_their_nodes` runs it at `k = 40`. Driven without
  a break: the levels' integers, `SUBSUMING`'s cost, the charges of the
  failures, the translation's account (now within 1.1 of the bound), the
  second-premise prune leaving no frame or charge.
- **Argument**: no refutation. Stated from the code first: every
  `Failed` in the memo and every set in `failed` names an unprovable
  sequent, established where a failure is written (all alternatives
  failed, or `→→L`'s second premise failed, `C ⊢ (A → B) → C` making it
  a consequence) and kept by subsumption (each clause of `entailed` a
  theorem of minimal logic, multicut and contraction giving `Δ ⊢ G`);
  no refusal writes a failure; the order only permutes; `close` gives
  `Lj::step` the premises in the rule's order. It noted that a sequent
  refuted by subsumption enters the memo but not `failed`, narrower than
  the brief said, sound either way: no change. Its probe ran the six
  unit tests and the ILTP test in release (passed).
- **Counterexamples**: no refutation. It pinned copies of the engine at
  `morqmskz` and at `xuovvqpk` (its first round's runs had been built
  while `xuovvqpk` was being edited in the working copy) and ran both
  rounds' batteries on each: the exhaustive sets (7 019 936 and 6 089 076
  sequents, all decided by its G3ip prover and all agreeing), 60 000
  schema instances, random sequents, 972 600 mutated derivations
  (136 721 accepted, every conclusion valid), and a generator aimed at
  the prunes (hypotheses `(p → q) → r`, `¬¬p`, `¬(p → q)`, `(p → q) → q`,
  `p → p`, repeated subsets and copies, goals among atoms, `⊥`, none,
  disjunctions and negations): 350 000 such sequents against the
  references, and on 150 000 of them the verdicts of `morqmskz` and
  `xuovvqpk` identical everywhere. 1 441 `NotValid` verdicts its first
  checks could not confirm were confirmed by a stronger G3ip (300 million
  steps) or a Kripke model of up to 9 worlds. The pruned engine had no
  `Unknown` where the old one had 13. *Answer*: none needed. It ran on
  `xuovvqpk`, before `otpoxkoq`, which changes only the cost of the
  ranking (each node visited once gives the same heads, so the same
  minimum level and the same order).

### Sources read

- R. Dyckhoff, "Contraction-free sequent calculi for intuitionistic
  logic: a correction", JSL 83(4), 2018, 1680–1682 (the St Andrews
  repository's copy): the rules of G4ip, the multiset calculus, the
  soundness by Cut, the invertibility of `→→L`'s second premise (Prop.
  2.2).
- R. Dyckhoff, "Intuitionistic decision procedures since Gentzen"
  (2016, the author's manuscript): `¬A` as `A → ⊥`, no rule for
  `⊥ → B`, the invertibility of the left rules but `→→L`.
- R. Iemhoff, "The G4i analogue of a G3i sequent calculus",
  arXiv:2011.11847, §2: the weight function (atoms and `⊥` 1, `∧` adds
  2, `∨` and `→` 1) and the multiset ordering.
- Dyckhoff and Negri, JSL 65(4), 2000: its abstract (contraction and cut
  admissible in G4ip), which the set of hypotheses rests on. The 1992
  paper was not reachable.
- ILTP's headers of SYJ205, SYJ209 and SYJ211 (Dyckhoff's benchmark
  formulas, 1997).

### The checklist

| item | state | evidence |
|---|---|---|
| cut in LJ's checker | done | `sxlozrrw`; `the_checker_takes_a_cut_in_lj` |
| engine, translation, unit tests | done | `morqmskz`, `xuovvqpk`, `otpoxkoq`; six tests in `ordinary::ljt` |
| a planted wrong translation refused | done | `a_planted_wrong_translation_is_refused`, four faults |
| bounds: work, stop, memory, no recursion | done | `the_search_keeps_to_its_bounds`; `core/tests/depth.rs` 50 000 deep on 256 KiB |
| ILTP oracle (timed slot, cores 2-3, lock) | done | `core/tests/iltp/embedding.csv` |
| ILTP agreement | done | `ulpsvomt`; 394 agreed, every proof checked |
| target sets 47 | done | 0 of 145 focused and 0 of 64 net rows differ in counters or verdicts (label `54a`, 13:09) |
| the panel, both rounds | see above | |
| status chapter and guide | done | `nusnsoqy`; `readme` test passed |
| rebase onto `main@origin` (`qlxumwzq`) | done | no conflict |
| the gate and `nix flake check` at the last commit | done | exit 0 both, at this report's commit, in the final message |

### What the next sessions must know

- **The engine's outcome is its own type** (`ordinary::ljt::Outcome`),
  not `ordinary::Outcome`, which holds the image's linear search. 54b
  joins them under the options and the command (an engine choice in
  `ordinary::Options`, the outcome's JSON form, the harness's engine
  axis); neither `ljt::Outcome` nor `Statistics` has a JSON form yet.
- **`Error::Logic`** (code `logic`) refuses classical logic; its message
  names LJT, which a second native engine would generalise.
- **Derivations from the engine hold cuts**, so their JSON is wire level
  9, their lambda-terms may hold redexes, and their certificates hold
  `let`s; LK's derivations and the embedding's read-backs never do.
- **The derivation can be far larger than the search**: proofs share
  steps between branches and the translation unfolds them into a tree,
  with contractions below every `→L` split and weakenings at every leaf.
  SYJ201+1.004 passes the default `derivation_bytes`; the depth test and
  the ILTP test lift it. 54b should measure how often the default bound
  refuses a proof the search found.
- **The ILTP test** is `#[ignore]`d (the library is not in the
  repository): `ILTP=…/bench/iltp/Problems cargo test --release -p
  deduxit --test iltp -- --ignored`, about 100 s; the oracle is the
  command's verdicts at 2 s and changes only by a new timed run.
- **Hard cases left**: random sequents of 4 or 5 atoms with nested `↔`
  can exhaust 1 GiB (a broad search); SYJ202+1.005 and SYJ208+1.007 take
  35 to 93 s. Success subsumption (a proved sequent proving a stronger
  one) would need a derivation of the entailments and was not built.
- **Follow-up**: `Derivation::term` (the lambda read-back) and the
  Rocq writer's twin assume a checked derivation; `term` does not check
  first, so a malformed JSON derivation panics there (an `→L` or a cut
  whose first premise has nothing right of `⊢`). Checking in `term`, or
  saying it needs a checked derivation, is the fix.

## Session 54b: the default by measurement

Opus 5.5 at `high`, in the jj workspace `../linlog-step54b`, from
`plan/54-ljt-engine.md` ("The sessions", 54b) and 54a's "What the next
sessions must know", on 2026-11-09.

### Outcome

- **LJT is the default of intuitionistic and minimal logic**, in the
  library (`ordinary::decide` with no method or translation given) and
  in the command (`prove --logic intuitionistic|minimal`, single and
  batch). Where a bound other than the time limit stops LJT (its
  derivation past `--derivation-limit`, the memory, the work), the
  default hands the sequent to the embedding with the time left
  (`ordinary::gives_way`). The embedding stays the default of classical
  logic and an option: `--method embedding` (the key `ordinary.method`),
  or `--translation`/`--linear` alone.
- **Measured on ILTP at 2 s** (tables below): per logic LJT alone
  decides 200 of 274 problems, the embedding 188, the default 202. The
  default decides every problem the embedding decides, and no verdict
  differs between any two methods, nor from 54a's oracle of every
  translation (`core/tests/iltp/embedding.csv`). The default is slower
  than the embedding beyond 20 % and 0.5 ms on no problem; its largest
  loss is SYJ201+1.005, 31 ms (5 %) over the embedding's 666 ms, which is
  LJT's attempt before it gives way.
- **The command's LJT path**: the verdict line `valid (L logic by
  LJT)`, `not valid (L logic by LJT): the search was exhaustive`, or
  `unknown (…)` with the advice; the checked derivation of LJ in every
  format, `--lambda`, `--stats` (`sequents expanded`, `memo hits`, `memo
  entries`, `refuted by an earlier failure`), and the JSON form of the
  ordinary outcome (`engine` `ljt`, LJT's `statistics`, no
  `translation`; wire level 9 where the derivation holds a cut). A
  session's `close` and `hint` keep the embedding.
- **The harness's engine axis has the methods**: `deduxit-bench run
  --tptp PATH --logics …` with `--engines embedding,ljt,auto`, ILTP's
  generated families as families (`SYJ201` at its 20 sizes), and
  `summary`'s "Verdicts that disagree", which `--strict` (the `bench`
  check, now with a TPTP problem) fails on.
- **Found on the way and fixed**: the root of LJT's derivation named its
  hypotheses in the order of the search's set, so `Derivation::term`'s
  context, the certificate's statement and the drawing's root differed
  from the sequent as written (a type check of the term in the lambda
  tests failed once LJT became the default); the root is now rewritten
  in the written order (`wqnsllop`).
- **54a's follow-up on `Derivation::term`**: kept as documentation
  (`# Panics`), not a check in `term` (decisions below).

The commits, in order: `tnunknxk` (the method in the library, the
outcome's `search`, LJT's JSON form), `xwoynwpv` (the harness's TPTP
source, the methods on the engine axis, the summary's cross-check),
`wqnsllop` (the root's written order), `xxlnqlnq` (`term`'s doc),
`nkolnqsz` (the first run's rows), `kyvnrmly` (the default, its
dispatch, the command, the manual, the guide, the rules, the second
run's rows), `nvynpnzw` (the behaviour lock reblessed), then the
panel's and the reviewer's answers: `zlyoltsk` (the stop asked by the
hypotheses expanded), `qpmywnsk` (`gives_way` and the message
narrowed), `qzlputqs` (LJT's test compares with the embedding again),
`xtyuxxwn` and `rtuztqok` (`check`'s refusal said truly, the help
corrected, the dispatch tested), `mrxsmuxm` (the help's lock), and this
report's commit. The chain is rebased onto `main@origin` (`ryuqpssm`)
without a conflict.

### The measurement

Two timed runs on cores 2 and 3 under `/tmp/linlog-machine.lock`, in
slots deduxit-orchestrator gave (15:02 to 15:07 and 15:22 to 15:30), each
one release build of `deduxit-bench` (so the methods' builds are alike),
two shards of ILTP v1.1.2's 274 propositional problems pinned one to
each core, one thread per problem, `--copies none --timeout 2 --repeat 3
--repeat-under 1` (runs under a second repeated, the median taken), both
logics:

- `bench/defaults/ljt-iltp.csv`: `--engines embedding,ljt` (5 minutes);
- `bench/defaults/ljt-default.csv`: `--engines embedding,ljt,auto`, auto
  being the library's default call with its dispatch (7.5 minutes).

The embedding is the default translation, cbn, as `--method embedding`
runs it; its time is the search alone (its read-back and check are the
row's `check_ms`). LJT's time is `ljt::decide`, its translation into LJ
and the check included. The default's (`auto`) is the library's whole
call, checks included. Every verdict was checked against ILTP's
intuitionistic status where it has one and across the methods
(`summary --strict`: no mismatch, no verdict that disagrees).

Per problem class (ILTP's domains, and its generated families by name),
the problems decided within 2 s by each method and the time all three
took on the problems all three decide (`ljt-default.csv`):

| logic | class | problems | decided: embedding | LJT | default | time of all three's common: embedding | LJT | default |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| intuitionistic | LCL | 2 | 2 | 2 | 2 | 0.2 ms | 0.0 ms | 0.1 ms |
| intuitionistic | SYJ | 5 | 5 | 5 | 5 | 0.4 ms | 0.1 ms | 0.2 ms |
| intuitionistic | SYJ105 | 3 | 3 | 3 | 3 | 0.3 ms | 0.1 ms | 0.1 ms |
| intuitionistic | SYJ107 | 4 | 4 | 4 | 4 | 23.0 ms | 0.3 ms | 0.3 ms |
| intuitionistic | SYJ201 | 20 | 5 | 3 | 5 | 6.5 ms | 2.9 ms | 3.0 ms |
| intuitionistic | SYJ202 | 20 | 3 | 3 | 3 | 82.8 ms | 0.9 ms | 0.8 ms |
| intuitionistic | SYJ203 | 20 | 20 | 20 | 20 | 1.69 s | 4.7 ms | 4.7 ms |
| intuitionistic | SYJ204 | 20 | 20 | 20 | 20 | 5.9 ms | 1.0 ms | 1.1 ms |
| intuitionistic | SYJ205 | 20 | 20 | 20 | 20 | 1.59 s | 10.7 ms | 10.6 ms |
| intuitionistic | SYJ206 | 20 | 7 | 9 | 9 | 543.5 ms | 2.6 ms | 2.6 ms |
| intuitionistic | SYJ207 | 20 | 3 | 5 | 5 | 461.5 ms | 4.0 ms | 4.0 ms |
| intuitionistic | SYJ208 | 20 | 3 | 6 | 6 | 326.9 ms | 0.2 ms | 0.2 ms |
| intuitionistic | SYJ209 | 20 | 20 | 20 | 20 | 2.61 s | 2.8 ms | 2.7 ms |
| intuitionistic | SYJ210 | 20 | 20 | 20 | 20 | 22.1 ms | 0.6 ms | 0.6 ms |
| intuitionistic | SYJ211 | 20 | 20 | 20 | 20 | 1.80 s | 287.9 ms | 286.4 ms |
| intuitionistic | SYJ212 | 20 | 14 | 20 | 20 | 2.45 s | 1.0 ms | 1.0 ms |
| intuitionistic | SYN | 19 | 19 | 19 | 19 | 1.7 ms | 0.4 ms | 0.4 ms |
| intuitionistic | SYN007 | 1 | 0 | 1 | 1 | 0.0 ms | 0.0 ms | 0.0 ms |
| minimal | LCL | 2 | 2 | 2 | 2 | 0.2 ms | 0.0 ms | 0.0 ms |
| minimal | SYJ | 5 | 5 | 5 | 5 | 0.6 ms | 0.1 ms | 0.1 ms |
| minimal | SYJ105 | 3 | 3 | 3 | 3 | 0.3 ms | 0.1 ms | 0.1 ms |
| minimal | SYJ107 | 4 | 4 | 4 | 4 | 23.0 ms | 0.3 ms | 0.3 ms |
| minimal | SYJ201 | 20 | 5 | 3 | 5 | 6.6 ms | 3.0 ms | 3.0 ms |
| minimal | SYJ202 | 20 | 3 | 3 | 3 | 79.9 ms | 0.8 ms | 0.8 ms |
| minimal | SYJ203 | 20 | 20 | 20 | 20 | 1.69 s | 4.7 ms | 4.6 ms |
| minimal | SYJ204 | 20 | 20 | 20 | 20 | 5.9 ms | 1.0 ms | 1.1 ms |
| minimal | SYJ205 | 20 | 20 | 20 | 20 | 1.59 s | 10.6 ms | 10.6 ms |
| minimal | SYJ206 | 20 | 7 | 9 | 9 | 545.1 ms | 2.6 ms | 2.5 ms |
| minimal | SYJ207 | 20 | 3 | 5 | 5 | 460.1 ms | 3.9 ms | 4.0 ms |
| minimal | SYJ208 | 20 | 3 | 6 | 6 | 482.2 ms | 0.2 ms | 0.2 ms |
| minimal | SYJ209 | 20 | 20 | 20 | 20 | 2.83 s | 2.6 ms | 2.5 ms |
| minimal | SYJ210 | 20 | 20 | 20 | 20 | 15.2 ms | 0.3 ms | 0.3 ms |
| minimal | SYJ211 | 20 | 20 | 20 | 20 | 1.84 s | 328.4 ms | 335.0 ms |
| minimal | SYJ212 | 20 | 14 | 20 | 20 | 2.25 s | 0.7 ms | 0.7 ms |
| minimal | SYN | 19 | 19 | 19 | 19 | 1.8 ms | 0.4 ms | 0.4 ms |
| minimal | SYN007 | 1 | 0 | 1 | 1 | 0.0 ms | 0.0 ms | 0.0 ms |
| intuitionistic | **all** | 274 | 188 | 200 | 202 | 11.61 s | 320.3 ms | 318.8 ms |
| minimal | **all** | 274 | 188 | 200 | 202 | 11.82 s | 359.8 ms | 366.3 ms |

Over both logics together, the embedding decides 186 + 194 (valid +
not valid, late verdicts included), LJT 186 + 214, the default 190 +
214.
The first run (`ljt-iltp.csv`) agrees: LJT 200 and the embedding 188 per
logic, LJT slower on 2 of the 372 problems both decide, by under 0.08
ms, the median of LJT's time over the embedding's 0.08.

Where they differ:
- **Decided by the embedding alone**: SYJ201+1.004 and +1.005 in both
  logics (167 ms and 670 ms). LJT finds their proofs in milliseconds,
  but the derivation of LJ passes the default 64 MiB
  (`derivation_limit`); the default gives way and decides them.
- **Decided by LJT alone**, 14 per logic: SYJ206 (2), SYJ207 (2),
  SYJ208 (3), SYJ212 (6, sizes 15 to 20), and SYN007+1.014, whose image
  the embedding refuses past the occurrence limit.
- **LJT stopped at a bound**: `derivation_limit` on 17 sizes of
  SYJ201, 8 of SYJ206 and one of SYJ202; `memory_limit` (1 GiB, the library's default) on 13 of
  SYJ202 and 9 or 10 of SYJ208. On these the default runs the embedding,
  which decides only the two SYJ201 problems above.
- **Neither**: the rest of SYJ201, SYJ202, SYJ207 and SYJ208, at the time
  limit or a bound.

The comparison is `compare.py` and `three.py`, kept below; `python3 -I
three.py bench/defaults/ljt-default.csv` prints the table.

`three.py`:

```python
# Per problem class and logic: what each method decides at 2 s and in how long.
import csv, sys, statistics, collections
rows = list(csv.DictReader(open(sys.argv[1])))
methods = ['embedding', 'ljt', 'auto']
runs = collections.defaultdict(list)
for r in rows:
    runs[(r['family'], r['problem'], r['mode'], r['engine_requested'])].append(r)
def get(k):
    rs = runs[k]; t = statistics.median(float(x['time_ms'] or 0) for x in rs)
    return rs[0]['verdict'], t, rs[0]['verdict'] in ('proved', 'unprovable') and t <= 2000
cls = lambda f: f.split('/')[1] if f.startswith('Problems/') else f
table = collections.defaultdict(collections.Counter)
for (f, p, m) in sorted({k[:3] for k in runs}):
    c = table[(m, cls(f))]
    c['n'] += 1
    got = {e: get((f, p, m, e)) for e in methods}
    for e in methods:
        v, t, d = got[e]
        if d:
            c[e] += 1; c[e + ' proved'] += v == 'proved'; c[e + ' ms'] += t
    if all(got[e][2] for e in methods):
        c['all'] += 1
        for e in methods: c['all ' + e] += got[e][1]
def ms(v): return f"{v/1000:.2f} s" if v >= 1000 else f"{v:.1f} ms"
print("| logic | class | problems | decided: embedding | LJT | default | time of all three's common: embedding | LJT | default |")
print("|---|---|--:|--:|--:|--:|--:|--:|--:|")
tot = collections.defaultdict(collections.Counter)
for (m, k), c in sorted(table.items()):
    tot[m].update(c)
    print(f"| {m} | {k} | {c['n']} | {c['embedding']} | {c['ljt']} | {c['auto']} | {ms(c['all embedding'])} | {ms(c['all ljt'])} | {ms(c['all auto'])} |")
for m, c in sorted(tot.items()):
    print(f"| {m} | **all** | {c['n']} | {c['embedding']} | {c['ljt']} | {c['auto']} | {ms(c['all embedding'])} | {ms(c['all ljt'])} | {ms(c['all auto'])} |")
```

`compare.py` (two methods side by side, `python3 -I compare.py FILE embedding,ljt`):

```python
# Pairs the embedding's and LJT's rows of one harness file, problem by problem.
import csv, sys, statistics, collections
rows = list(csv.DictReader(open(sys.argv[1])))
methods = sys.argv[2].split(',') if len(sys.argv) > 2 else ['embedding', 'ljt']
runs = collections.defaultdict(list)
for r in rows:
    runs[(r['family'], r['problem'], r['mode'], r['engine_requested'])].append(r)
def get(f, p, m, e):
    rs = runs.get((f, p, m, e))
    if not rs: return None
    first = rs[0]
    t = statistics.median(float(x['time_ms'] or 0) for x in rs)
    decided = first['verdict'] in ('proved', 'unprovable') and t <= float(first['timeout_s']) * 1000
    return first['verdict'], first['reason'], t, decided
keys = sorted({(f, p, m) for (f, p, m, e) in runs})
a, b = methods
classes = collections.defaultdict(lambda: collections.Counter())
lost, slower, both = [], [], []
for f, p, m in keys:
    x, y = get(f, p, m, a), get(f, p, m, b)
    cls = (f if not f.startswith('Problems/') else f.split('/')[1], m)
    c = classes[cls]
    c['problems'] += 1
    c[a + ' decided'] += x[3]; c[b + ' decided'] += y[3]
    if x[3]: c[a + ' ms'] += x[2]
    if y[3]: c[b + ' ms'] += y[2]
    if x[3] and y[3]:
        c['both'] += 1; c['both ' + a + ' ms'] += x[2]; c['both ' + b + ' ms'] += y[2]
        both.append((y[2] / x[2], f, p, m, x[2], y[2]))
        if y[2] > x[2] * 1.2 and y[2] - x[2] > 0.5:
            slower.append((f, p, m, x[2], y[2]))
    if x[3] and not y[3]:
        lost.append((f, p, m, x[0], x[2], y[0], y[1]))
    if y[3] and not x[3]:
        c['only ' + b] += 1
    if x[3] and not y[3]:
        c['only ' + a] += 1
print(f"| class | logic | problems | {a} decided | {b} decided | only {a} | only {b} | {a} time of its decided | {b} time of its decided | decided by both: {a} | {b} |")
print("|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
tot = collections.defaultdict(collections.Counter)
def ms(v):
    return f"{v/1000:.2f} s" if v >= 1000 else f"{v:.1f} ms"
for (cls, m), c in sorted(classes.items(), key=lambda kv: (kv[0][1], kv[0][0])):
    tot[m].update(c)
    print(f"| {cls} | {m} | {c['problems']} | {c[a+' decided']} | {c[b+' decided']} | {c['only '+a]} | {c['only '+b]} | {ms(c[a+' ms'])} | {ms(c[b+' ms'])} | {ms(c['both '+a+' ms'])} | {ms(c['both '+b+' ms'])} |")
for m, c in tot.items():
    print(f"| **all** | {m} | {c['problems']} | {c[a+' decided']} | {c[b+' decided']} | {c['only '+a]} | {c['only '+b]} | {ms(c[a+' ms'])} | {ms(c[b+' ms'])} | {ms(c['both '+a+' ms'])} | {ms(c['both '+b+' ms'])} |")
print("\nDecided by", a, "and not by", b, ":")
for l in lost: print("  ", l)
print("\nDecided by both,", b, "slower by over 20 % and 0.5 ms:")
for l in slower: print("  ", l)
both.sort()
print("\nratio", b, "/", a, "on both: min", both[0][:4] if both else None, "median", statistics.median(r[0] for r in both) if both else None, "max", both[-1][:4] if both else None)
over = [r for r in both if r[0] > 1]
print(b, "slower than", a, "on", len(over), "of", len(both), "; the largest difference in ms:", max(((r[5]-r[4]), r[1:4]) for r in over) if over else None)
print(a, "time, most among those:", max((r[4] for r in over), default=None))
```

### Decisions

- **The default is LJT with a dispatch to the embedding** (D19): LJT
  alone would lose SYJ201+1.004 and .005, which the embedding decides at
  the command's limit, so the default gives way where LJT ends at a
  bound that is not the time limit. A dispatch at run time, rather than
  by the sequent's shape: nothing in a sequent's shape tells that its
  derivation will outgrow the limit, and the attempt costs LJT
  milliseconds there. `--method ljt` reports the bound (exit 2 for a
  derivation it could not check, naming `--derivation-limit`,
  `--memory-limit` and `--method embedding`). The time limit is no such
  bound: the default does not run the embedding after LJT timed out.
- **"The families" are ILTP's generated families** (SYJ201 to SYJ212 at
  20 sizes each, SYJ105, SYJ107, SYN007): the library has no generator
  of ordinary sequents; deduxit-orchestrator accepted this reading before
  the run. "Both builds alike" was read as both methods measured from
  one build in one run.
- **The library's API**: `ordinary::Method` (`Embedding`, `Ljt`, with
  `ALL`, `NAMES`, `default_for`, `decides`, `FromStr`),
  `ordinary::Options::method` (`Option<Method>`, `None` the default)
  with `with_method` and `method()`, `ordinary::gives_way`, and
  `ordinary::Outcome::search` (`ordinary::Search::Embedding {
  translation, linear }` or `Search::Ljt(statistics)`), which replaces
  the fields `translation` and `linear`, a break the changelog states.
  A translation given names the embedding when no method is given, so
  that every existing call with a translation keeps its meaning.
  `ljt::Outcome` stays LJT's own; `From<ljt::Outcome>` makes the
  ordinary one.
- **The options for every front end** (D15): the method is the
  settings' `ordinary.method` (serde, `"embedding"`/`"ljt"`/`null`),
  the command's `--method` on `prove`, the page's JSON settings unchanged
  in shape (the key is optional). The page's settings name no ordinary
  engine and its ordinary tab only proves step by step, whose `close`
  stays the embedding, so the client is not touched.
- **JSON of an LJT outcome**: the ordinary outcome's own form, which
  `ordinary::Outcome` already had, with `engine` `ljt` and LJT's
  statistics; under the embedding the command still writes the image's
  outcome. Two shapes for `prove --logic --format json`, one per method,
  each the outcome of what decided; the guide and the manual say which.
- **A session's `close` and `hint` keep the embedding**: LJT's
  derivations hold cuts, which no step of the goal state takes and a
  hint cannot name (later.md).
- **`Derivation::term` does not check first**: checking would refuse the
  admissible view, which `term` reads too (the lambda tests read both),
  and a session's derivation with open goals, which `term` refuses by
  name. Its doc now says it reads a checked derivation or a view of one
  and may panic otherwise (`# Panics`); no caller in the repository
  reads a derivation from JSON and takes its term unchecked. Turning the
  read-back's `expect`s into errors is in later.md.
- **The interactive session's verdict line** under `close` reads the
  outcome's `Search::Embedding`, an `unreachable!` otherwise, since the
  state passes `Method::Embedding`.
- **Where quantifiers go** (D17): `Method` names a calculus of
  propositional logic; a first-order engine is a variant of its own, and
  `default_for` takes the logic as now.
- **What the engine copies from the linear engines** is 54a's list; this
  session added none.

### The panel and the reviewer

The panel of step 26's prompt, as named agents from fresh contexts, on
`tnunknxk` to `nvynpnzw` (the argument on Fable 5.1, counterexamples on
Opus 5.5, the integers on Sonnet 5.5), each on cores of its own; then
the fresh-context reviewer (Opus 5.5) on the whole chain to `xtyuxxwn`.

- **Argument** (deduxit-54b-argument): no refutation. It stated from the
  code that a `Valid` is built only where the derivation returned is the
  one `Derivation::check` just accepted, after `in_order`, and that the
  check compares the root with the declared sequent as a multiset, so
  the rewrite can only make a check fail; that the fallback runs only
  with no method given and only on an unknown at a bound or a refusal;
  that the exit statuses are the verdicts'; that a session's `close` and
  `hint` pass `Method::Embedding`. One finding: `Error::is_refusal` is
  true for unsupported inputs too, so `--method ljt --logic classical`
  and two formulas right of `⊢` were said as "LJT found a proof, but its
  derivation was not checked", and `gives_way` held for them (harmless,
  the embedding refused alike). *Answer*: it holds; `qpmywnsk` makes
  `gives_way` take `Error::Refused` other than the stop alone, and the
  command's message "LJT gave no verdict: … raise the limit
  (--derivation-limit, --memory-limit, --work-limit), or decide through
  the embedding with --method embedding".
- **Integers** (deduxit-54b-integers): one refutation. LJT asked the stop
  every 256 expansions, each costing about its sequent's size, so a
  chain of 100 000 implications ran 11.8 s past a 1 s limit (debug
  build), where the embedding had stopped at 1.04 s; the outcome stayed
  `unknown`, exit 3. *Answer*: it holds; `zlyoltsk` also asks the stop
  once the sequents expanded since the last ask hold 256 hypotheses
  (`a_large_sequent_asks_the_stop_at_each_expansion`; verdicts and
  counters unchanged, the ILTP test and the target sets after it).
  After the fix the same chain, in a release build, ends at 1.04 s by
  LJT and 1.15 s through the embedding (the session's own run, `prove
  --timeout 1s`, exit 3 both). Driven without a break: `in_order`'s
  re-found position on about 32 000 valid proofs with repeated
  hypotheses (both branches of `same`), 2 000 to 40 000 hypotheses from
  the command, `--derivation-limit`, `--memory-limit` and `--work-limit`
  from their least to `none` (a refusal or an unknown, never a verdict;
  the default falls back, `--method ljt` exits 2), the time limit hit in
  the search, the translation and the read, nesting of 30 000, the
  occurrence bound on text and TPTP (LJT counts the source, the
  embedding its larger image), the batch's `seconds`, and the harness's
  columns. Two oddities, not breaks: an LJT row has no
  `recursion_limit` (it has none), and the embedding's memory message
  read "limit of 0 B" at `--memory-limit 1MiB` (later.md, not traced).
- **Counterexamples** (deduxit-54b-counterexamples): no refutation, with
  a G4ip prover of its own in Python (truth table, generator, a
  self-test), over 7 312 random sequents in both logics through the
  library's three paths (LJT, the default, the embedding), each valid
  derivation checked again, its root's order and its lambda-term's
  context and goal typed against the written sequent; 2 460 single
  calls of the command in JSON and text; batches of up to 3 000 lines
  (7 978 records, 3 015 LJT derivations read back and checked); 3 600
  harness rows on 600 generated TPTP files, `summary --strict` passing.
  Three findings: `deduxit check` refused LJT's JSON saying it "holds
  no proof to check" (*answer*: it holds as a wrong message; `check`
  reads proofs of linear logic, and `xtyuxxwn` says what the file is and
  how to get one it reads; reading LJ derivations in `check` is a new
  feature, in later.md); `agrees_with_the_embedding` in `ljt.rs` now
  compared LJT with itself (*answer*: it holds; `qzlputqs` names the
  embedding); `--method ljt` refuses some medium sequents at the
  default derivation limit (*answer*: the documented behaviour, no
  change).
- **The fresh reviewer** (deduxit-54b-reviewer): no wrong verdict, exit
  status, panic or lost verdict; it re-derived the report's tables from
  the rows and found D19 held against the command's own oracle (380 cbn
  verdicts, all decided by the default within 2 s). Four findings of
  false text, each *holding* and answered: `check`'s new message fired
  for a `not_valid` outcome and a bare derivation (`rtuztqok` names
  each); `--method`'s help said LJT "always comes to a verdict"
  (`rtuztqok`: "whose search always ends"); `--format json`'s help
  promised the linear outcome under `--logic` (`rtuztqok` names the
  ordinary outcome); the report's draft said 4 GiB for the memory bound
  (1 GiB, the library's default) and "per logic" for counts over both
  logics (corrected here). Its question, that no test drives the
  dispatch: answered in `rtuztqok`, a library test of `gives_way` (a
  refused derivation gives way, a stop and classical logic do not) and
  a command test (`--derivation-limit 100` under the default answers
  through the cbn translation).


### The checklist

| item | state | evidence |
|---|---|---|
| the harness's engine axis gains LJT | done | `xwoynwpv`; `--tptp`, `--logics`, `--engines embedding,ljt,auto`; the `bench` check runs a TPTP problem |
| ILTP and its families at 2 s, both methods, one build, timed (cores 2-3, lock) | done | `bench/defaults/ljt-iltp.csv`, `ljt-default.csv`; the slots given by deduxit-orchestrator |
| the default by D19 | done | `kyvnrmly`: LJT with the dispatch; tables above |
| ILTP's verdicts where the embedding decides | unchanged | no verdict differs from `core/tests/iltp/embedding.csv` in either run; `ILTP=… cargo test --release -p deduxit --test iltp -- --ignored`: "394 verdicts of the embedding agreed" |
| the target sets 47 | unchanged | label `54b`: 0 of 145 focused and 0 of 64 net rows differ in verdicts or counters |
| the command's option, help, JSON form | done | `--method`, `ordinary.method`, LJT's outcome JSON; the lock's new calls |
| the manual, the guide, the status chapter, README, the changelog | done | `kyvnrmly`; `cargo test -p deduxit-cli --test readme` |
| the page's settings | untouched | they name no ordinary engine; `close` stays the embedding |
| `Derivation::term` checks first | kept, with its reason | `xxlnqlnq` (`# Panics`), later.md |
| the panel, the fresh reviewer | see above | |
| the gate and `nix flake check` at the last commit | run at this report's commit | exit statuses in the message to deduxit-orchestrator |


### What the next sessions must know

- **The default dispatch lives in two places**: the library's
  `ordinary::decide` and the command's `by_ljt` (single and batch), both
  through `ordinary::gives_way`; a change of when LJT gives way is a
  change of that function, which both read.
- **LJT's derivations outgrow its search**: 26 sizes of SYJ201, SYJ202
  and SYJ206 per logic are proved in milliseconds and refused at 64 MiB.
  A derivation with shared subderivations and a checker that checks each
  once would make them checked verdicts (later.md); until then the
  default hands them to the embedding, which times out on all but two.
- **The harness's ordinary rows**: `--tptp`, `--logics`, the methods on
  `--engines`; `auto` is the library's whole call and times its checks,
  `embedding` times the search alone, `ljt` its search, translation and
  check. Compare times across methods knowing that.
- **The JSON of `prove --logic`** is the ordinary outcome under LJT and
  the image's outcome under the embedding; a front end reads `engine`
  to tell them apart.
- **Step 55's countermodels** start from LJT's `NotValid`, still the
  search's word ("the search was exhaustive" on the verdict line).
