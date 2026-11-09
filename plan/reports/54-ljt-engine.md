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
