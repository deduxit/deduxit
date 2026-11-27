# Step 87c7: the linear engines' repeats, one way to build a refusal, the memory account at the root (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-87c1-64a1a5`, from `plan/87-refactor.md` (87c7,
wave C, panel) and the planning session's briefs `start-87-common.md`
and `start-87c7.md`, on cores 7 to 9. The base is `pwvpwkoq` (87c8's
head). Themes C9 for units 2a, 2b and 2c, C7's F-2c-11, C6, and the
crate-private half of F-arch-06. No change of behaviour.

## What landed

- **`womytprr` Build the stop's refusal through one constructor** (C6):
  `Refusal::stopped(phase)`, crate-private, the name the signed
  proposal gives it (P7), at the 74 sites that built
  `Refusal::Stopped { phase }` by hand; the five patterns among them
  stay patterns. The private `stopped()` helpers of the exports and the
  lambda suite now call it (their logic stays theirs).
- **`mlrltuwl` Move the memory account to the crate's root**
  (F-arch-06, crate-private half): `linear/search/memory.rs` is
  `core/src/memory.rs`; every user names `crate::memory`, ordinary and
  modal logic's among them.
- **`qqyvkqyt` Estimate a hash table's bytes in one place** (F-2b-07,
  in part): `memory::table_bytes`, which the memo and the waiting table
  call; on every capacity the standard library's tables report the two
  formulas agreed, so no charge moves.
- **`kxpowyqz` Grow the Horn engine's heaps and buffers through one
  helper each** (F-2c-15): `horn::heap_room` with the caller's growth
  (the frontier grows to the successors or doubles, the queue by
  `max(16)`), and `room` moved from `reach.rs` to the engine's root,
  which `cover.rs` and `mod.rs` call.
- **`nstyzmtn` Set an unknown's reason to the options and limits at the
  front door alone** (F-2a-06): the seven engine-side `as_set` calls
  go; `conclude` applies it to every engine's unknown, as before;
  `merged` lost the two parameters only that call read, and its test
  applies `as_set` to what it compares, as the front door does.
- **`tonlzomz` Drop the turns' dead test** (F-2a-07): `Turns::asked`
  and the second `probes` call in `take`, which the poll reaches only
  after `probes` held.
- **`soltvlot` Hand conclude the goal's members with the goal**
  (F-2a-11, in part): `Concluding` carries the members, `conclude` has
  seven arguments and lost its `too_many_arguments` expectation.
- **`uzmwuxvz` Share the focused engine's counters and the memo's
  collection** (F-2b-15): `own_counters`, which `statistics` and the
  pool's `report` call, and `collected` for `remember`'s two passes.
- **`kquzzsxw` Take and give the focused engine's lists through one pair
  of helpers** (F-2b-14, in part): `take_pooled` and `give_pooled` for
  the four pools of lists (members, words, trails, links).
- **`nmrqxwot` Share the focused engine's branch stack steps between
  its two paths** (F-2b-02): `push_stack`, `pop_stack`, `settled` and
  `failure_of`, each called before or after the recursion, never in a
  frame of it.
- **`mwytuosm` Write the first-order zone's two keys with one function**
  (F-2b-12): `write_key::<CANONICAL>`, a constant, so each key keeps a
  loop of its own and no branch per word.
- **`znmwpxnp` Make the inverse method's rule writer in one place**
  (F-2c-17): `Writer::new`.
- **`sutktzzv` Merge a transition's inputs and outputs by place in one
  iterator** (F-2c-06): `horn::effects`, for the distance (`moved`),
  the state equation's effects and the caps.
- **`vlqtwqxp` Walk a Horn clause's factors in one function**
  (F-2c-13, in part): `clause_walk` with a callback per body literal, so the
  reader numbers places in the walk's order as before;
  `clause_head` is the walk without one.
- **`pzqnrnnq` Share the sorted-list multiset operations between proofs
  and the inverse method** (F-2c-11): `proofs::multiset::{insert,
  remove, add, excess, within}` over sorted lists, which `Multiset`'s
  `insert`, `remove`, `sum`, `difference` and `is_subset` call and the
  inverse method uses for its zones; `Multiset`'s run-counting merge
  stays for its test-only `union`. The checker's `Bag` is untouched.

## Departures

- **The one meter waits for part 2** (C6's F-2a-10 and F-5a-05, departs
  from the brief's "one crate-private meter"): the verdicts put it with
  F-arch-03's public `limits::Meter` (P7), and the lambda suite's three
  meters report `Progress` in three units a caller's stop sees, which a
  shared meter would change (F-5a-05's verdict: "nearer an API change").
  The front door's three copies share three lines each, on the poll
  that `core-search.md` records as sensitive to seven instructions.
- **F-2b-07: the first-order zone keeps its own table charge** (the
  verdict): folding it in raises its charge and can move a first-order
  row's counters.
- **F-2b-05 stays** (the split searches' two loops): merged, the ground
  search's debug frame grows on the recursion cycle, which needs
  `Limits::PER_LEVEL` measured again and pinned time on `Ground`
  (the verdict); a step of wave D can take it with those instruments.
- **F-2b-14: the pools of zones, tallies and splits stay** (the
  verdict's "two helpers, one per shape"): the lists' shape has its
  helper; the other shape needs the run's charges inside the helper,
  which a borrow of the whole run would cost on the hottest path.
- **F-2c-13: the proof builder still walks a fired clause again**
  (`horn/proof.rs`, through `clause_head`): keeping each clause's head in
  the `Program` instead costs a head per clause of every class of
  clauses used once (the verdict's correction), memory beside the
  markings for a walk the proof does once per firing.
- **F-2a-11: the three structs stay**; only `conclude`'s arguments
  went. **F-2a-07**: the dispatch table is scanned twice now, not three
  times; passing the row's flag from `prepare` would change its
  signature for one scan.

## The net

- **Rebased** onto `main@origin` (`mrqtknsz`, 87b3 and 87c8 in it)
  without conflict before the gate.
- **The gate** at the rebased head, on cores 7 to 9: Clippy on the
  workspace and on the core crate alone, warnings denied, clean; `cargo
  test --workspace`, all 39 suites passed; both `cargo hack` runs and
  the powerset of depth 2 with `--tests`, warnings denied, clean; the
  ratchet (`--check`) within its tolerance on every journey.
- **The target sets** on cores 2 and 3 (given by the planning session),
  before the rebase (which brought no file of the search):
  `bench/targets/87c7.csv` against `86.csv`, no verdict differs and none
  of the 145 problems decided alike differs in a counter, nothing lost
  or gained; `net-87c7.csv` against `net-86.csv`, likewise for its 64.
- **The ratchet, head against base** (built and counted the same way in
  a workspace of its own at `0084dea8`, `main@origin` when it was made:
  the chain's root `mrqtknsz` and 87d4's Rocq export, which no journey
  runs), instructions:

  | journey | base | head | change |
  |---|--:|--:|--:|
  | search-qbf-20-2 | 486 179 848 | 487 740 877 | +0.32 % |
  | search-chain-128 | 961 506 496 | 962 852 713 | +0.14 % |
  | search-spec-chain | 22 927 982 | 22 770 370 | −0.69 % |
  | search-chain-64-intuitionistic | 122 093 973 | 122 293 399 | +0.16 % |
  | search-chain-fo-32 | 24 876 686 | 24 893 947 | +0.07 % |
  | search-inverse | 4 456 399 | 4 459 188 | +0.06 % |
  | check-qbf-20-2 | 138 102 545 | 138 825 952 | +0.52 % |
  | cut-elimination | 11 685 960 | 11 730 072 | +0.38 % |
  | derivation-chain-64 | 9 912 813 | 9 825 179 | −0.88 % |
  | ordinary-pigeons | 32 291 936 | 31 819 977 | −1.46 % |
  | session-ordinary | 39 444 838 | 38 949 430 | −1.26 % |
  | session-linear | 2 217 394 | 2 201 094 | −0.74 % |
  | workspace-run | 3 007 884 | 2 988 645 | −0.64 % |

  Every other journey moved by less than 0.5 %. Five make fewer heap
  blocks: `ordinary-pigeons` 59 651 to 58 629, `session-ordinary`
  76 985 to 75 963, `session-linear` 4 181 to 4 133,
  `derivation-chain-64` 1 078 to 1 045, `workspace-run` 4 241 to 4 211:
  `Multiset::is_subset` no longer builds the difference to test it.
  No ceiling was lowered (as at 87c1 and 87c8: the planning session
  lowers them once).
- **`breaks 0.8.0`**: nothing of this step; the same three failures and
  diff as at 87c1 and 87c8, all earlier steps' and in the changelog;
  what this step adds is crate-private.
- **`nix flake check`** at `zvsuvwvs`, under the machine lock: "all
  checks passed!" (`doc`, `doc-private`, `features`, `features-tests`,
  `wasm`, the `ratchet` in the sandbox, the web and Rocq checks among
  them). Only this report changed after it.

## The panel and the reviewer

- **Argument** (`panel-argument`, Opus 5.5 at `high`, the Fable share
  being spent; a first run on Fable was stopped by the pause): nothing
  breaks any of its eight claims. Every rewritten refusal keeps its
  phase and is an expression; every engine's answer reaches `conclude`
  through `Answered::concluded`, and between them only a reason's
  variant is read (the copy bound is `conclude`'s own, the memory bound
  the same limits); the turns' test was dead; the stack helpers, the
  Horn helpers, the key writer, the multiset functions, `Concluding`,
  `own_counters` and `collected` each equal what they replace, with and
  without `parallel`.
- **Integers** (`panel-integers`, Sonnet 5.5 at `high`): no refutation.
  `table_bytes` equals the memo's former formula at every capacity and
  the waiting table's at every capacity a standard table reports; no
  wrap on 32 bits below an allocation past `isize::MAX`; `heap_room`,
  `effects`, the stack helpers, the multiset functions and `write_key`
  each the same arithmetic as the code they replace; 63 tests of the
  Horn engine, the multisets, the memory account, the memo and the
  inverse method passed.
- **Counterexamples** (`panel-counterexamples`, Opus 5.5 at `high`): no
  counterexample. The head's and the base's commands gave the same exit
  status, standard output and error on 36 900 generated inputs (every
  mode of linear logic, Horn programs, first-order sequents, ordinary
  logic by every method, with random work, memory, copy and memo bounds
  and forced engines), on 11 000 exports and ordinary methods (eight
  runs ended by a 30 s time limit vary from run to run within each
  build), and on 250 inputs swept over memory bounds from 1.5 KB to
  64 MiB, with every point where the base's answer changes compared
  (1 652); `deduxit-bench run --all-families` gave equal rows (176
  decided, every counter), also under a 3 MB bound and a memo of 64; a
  second reference of the multiset operations, a count map, agreed on
  2 000 000 rounds; and the 74 refusal sites keep their phases. Its base
  was the ratchet's (above), so `--format rocq` was left out: 87d4
  changed it.
- **The reviewer** (`fresh-reviewer`, Opus 5.5 at `high`): no break of
  behaviour; three findings of the records, all answered: three rules
  lines the move left stale (`core-search.md`, `core.md`'s module list,
  `core-ordinary.md`) and one new line that called `table_bytes` the
  only estimate of a table's bytes, corrected in `ymznvrpp` and in this
  report; F-2c-13 done in part without saying so, now marked and its
  reason given above.

## What the next steps must know

- `crate::memory` is where every suite's account is; `table_bytes` is
  the estimate of a hash table's buckets that the memo and the waiting
  failures charge. The first-order zone (`framed.rs`) and the checker's
  zones (`check/zones.rs`, whose bound Kani checks) keep their own.
- `Refusal::stopped` is crate-private until P7 makes it public with
  `Refusal::work`, `Refusal::memory` and `limits::Meter`.
- `proofs::multiset`'s functions over sorted lists are crate-visible
  (`pub(crate) mod multiset`); 87c6's F-1c-11 (the saturated set's
  check) can use them.
