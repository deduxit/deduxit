# Step 87c6: the repeats of the linear suite's data model, proofs and refutations (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-87c1-64a1a5`, from `plan/87-refactor.md` (87c6,
wave C, panel) and the planning session's briefs `start-87-common.md`
and `start-87c6.md`, on cores 7 to 9. The base is `zvsuvwvs` (87c7's
head). Theme C9 for units 1a, 1b and 1c, and C7's F-1c-11. No change of
behaviour.

## What landed

- **`xywwkyxx` Change the checker's unrestricted sum in functions Kani
  checks** (F-1b-05): `State::add_unrestricted`, `remove_unrestricted`
  and `unite_unrestricted` hold the `+=`, the `-=` and the swap and
  pour of the unrestricted zone that the pass wrote inline at `Copy`,
  `Quest` and `&`; the pass calls them (at `&` with the pass's fields
  borrowed apart, so `shared` stays in place: a first form that took the
  list out for the call cost `check-qbf-20-2` 0.4 % more), and six
  new harnesses drive
  them: a copy of a new member and of one held, a `?` step on a member
  held and on one missing, and the union of zones of the same member
  and of two members. The swap before the pour is not among them (a
  zone of one member into one of both ran out of 8 GB; an empty zone
  into one of a member stops at a drain of a table never allocated,
  whose layout Kani cannot compute), as the linear zone's swap is not.
  A harness's zone
  stands for any zone the set can be: one member in the table and the
  others' weight any sum their number allows (at most 2³² − 2 members of
  a weight below 2³², written as a shift), on the zone the union does
  not pour.
- **`rqsopkyl` Share the refutation checks' failure, count walk,
  phrases and Horn reader**: one `refutation::Failed` with `fault()`
  for the saturated set's and the basis's checks (F-1c-09); one
  `excess` walk and one `predicate_open` test for the balance and the
  one-sided count, each caller summing as before (`check_balance`
  plainly, `check_copies` checked) and passing its arm for the
  exponentials (F-1c-05); one branch for the two-sided and the
  intuitionistic phrases, which differ only in the side test's source
  and the closing note, both notes kept word for word (F-1c-14); one
  reader of a Horn member, `weighted::horn_member`, for the weighted
  test's dead atoms and the basis check, the basis adding its two
  conditions (a head under `?`, one goal) and `runs` counting a list
  once (F-1c-04).
- **`mvxqzkzv` Choose the cut of a step and of an elimination in one
  function** (F-1c-08, in part): `chosen`, the `at` test with its `NoCut` and the
  strategy's pick; `step` turns `None` into `NoCut`, `eliminate` into
  `CutFree`, as before.
- **`wvkyzptx` Walk first-order terms and frame chains once in cut
  elimination** (F-1c-07, C5's cut-elimination walks): `Postorder`, the
  walk with its own stack that the three term walks of `framed.rs`
  shared by copy, asks the caller what is done and what a term's
  arguments are, and gives each term after its arguments; `chain` is
  the walk up a frame chain to the first cell met before. The caller
  keeps its own state between two calls, so `substituted_term` adds to
  the table it reads (the verdict's warning).
- **`vwpwtwmw` Name a fragment through one index into its two tables of
  names** (F-1a-16): `named`, a `const fn`, gives the index into
  `NAMES` and a private table of the intuitionistic names; a test holds
  every named fragment, first-order ones too, to its names.
- **`mpqtlqpz` Make a parsed function application's term in one place**
  (F-1a-17): `applied`, `#[cold]` and `#[inline(never)]` as the
  parser's other first-order functions; the predicate's branch makes
  the arity and the list after its `equality_follows` test, which reads
  nothing they change.
- **`lzrryqxw` Read a rule's name through the names `Named::name` gives**
  (F-1b-12): `KNOWN` is made from `Named::name` at compile time, and
  `from_str` reads every name through it, its match keeping only the
  ASCII spellings; the test pins the written names (`WRITTEN`), so a
  name that changes still fails it. `core-export.md`'s procedure for a
  new rule says so.

## Departures

- **F-1c-11: the saturated set's check keeps its own sorted-list
  functions** (departs from the plan). The verdict confirmed the change
  because `Multiset` was used only under `proofs/` and `rules/`, "so
  using it does not join the check to `inverse/`, whose independence
  core-inverse.md:219 records". Since 87c7's F-2c-11 the inverse method
  evaluates with `proofs::multiset::{insert, remove, add, excess,
  within}`, so the same functions in the check would join the engine
  and its check after all: a fault in `within` would make the engine
  subsume too much and the check accept what it derived. The check's
  five functions stay.
- **F-1b-06 waits for part 2** (departs from the plan): `ClassicalRule`
  as a rule and a side is a change of a public type, which part 1 does
  not make.
- **F-1a-11: the sequent writers keep their own layouts** (departs from
  the plan): one `Notation::layout` for the six writers, with
  `#[inline]` or without, made `derivation-chain-64` 1.9 % slower than
  the base (10 013 834 instructions against 9 825 478 with the old
  writers, the rest of the chain the same), which the verdict made the
  test of whether the shape is kept. The commit was dropped.
- **F-1c-08: the mapping of an invalid made proof stays twice**
  (departs from the plan): of the check's errors only the `Invalid` arm,
  `Error::Reduction(Box::new(invalid))`, could live once (the verdict),
  and a helper for that one expression reads worse than the expression
  at its two sites.
- **F-1a-10: the two term writers stay** (departs from the plan): the
  instances' writer writes a symbol, an eigenvariable and an open
  witness through one closure of its caller (`atom`), so a shared writer
  with a closure per kind of term needs that closure twice at once,
  which the borrow rules refuse; a writer with one closure over an enum
  of pieces would cost more reading than the 30 lines it saves.

## The net

- **Rebased** onto `main@origin` (`xukolplx`, 87c7's review) without
  conflict before the gate.
- **The gate** at the final chain, on cores 7 to 9: Clippy on the
  workspace and on the core crate alone, warnings denied, clean; `cargo
  test --workspace`, all 39 suites passed; both `cargo hack` runs and
  the powerset of depth 2 with `--tests`, warnings denied, clean; the
  ratchet (`--check`) within its tolerance on every journey ("gate:
  passed"). A first gate, with the layout commit and the first form of
  the `&` union, failed on `check-qbf-20-2` (2.12 % over its ceiling,
  which the base already passed by 1.01 %); both were changed (above).
- **The target sets** on cores 2 and 3 (given by the planning session):
  `bench/targets/87c6.csv` against `86.csv`, no verdict differs, none of
  the 145 problems decided alike differs in a counter, nothing lost or
  gained, and the `checked` column (the refutations' checks) is the
  oracle's row for row; `net-87c6.csv` against `net-86.csv`, likewise
  for its 64.
- **The ratchet, head against base** (counted the same way in a
  workspace of its own at `zvsuvwvs`, 87c7's head, which `main@origin`
  extends by a plan file only), instructions, every journey that moved
  by 0.3 % or more:

  | journey | base | head | change |
  |---|--:|--:|--:|
  | check-qbf-20-2 | 138 120 530 | 139 075 800 | +0.69 % |
  | check-saturated | 22 242 880 | 22 331 585 | +0.40 % |
  | lambda-inhabit | 468 804 | 470 858 | +0.44 % |
  | ordinary-connection-intuitionistic | 7 260 743 | 7 232 336 | −0.39 % |
  | rules-check | 23 716 966 | 23 547 404 | −0.71 % |
  | ordinary-models | 630 276 | 622 461 | −1.24 % |

  No journey's heap blocks changed. The lambda, ordinary and rules
  journeys run no code of this chain (layout); `check-qbf-20-2` runs
  the pass with the moved arithmetic, at 0.69 % (the first form, with
  `shared` taken out for the call, was 1.10 %). No ceiling was lowered.
- **`breaks 0.8.0`**: nothing of this step; the same three failures
  and the same diff as at 87c7, all earlier steps' and in the
  changelog; everything this step adds is private or crate-private.
- **`web-core-wasm32`** under the machine lock: 723 tests passed on
  wasm32-wasip1, 2 ignored.
- **Kani**: the new harnesses alone first (`cargo kani --harness …`):
  six verified in 73 to 179 s each; the swap harness ran out of 8 GB as
  a zone of one member into one of both, and as an empty zone into one
  of a member stopped at an undetermined check of the standard
  library's table (above), so it was dropped. Then the package
  (`nix build .#kani-harnesses`, under the machine lock): "Complete - 17
  successfully verified harnesses, 0 failures, 17 total", the eleven of
  before and the six new.
- **`nix flake check`**: not run here. It waited behind the machine
  lock when the planning session moved every step's flake check to
  GitHub's runners (start-87-common.md), and was stopped before it
  began.

## The panel and the reviewer

The panel saw the chain with the layout commit and the first form of
the `&` union (`shared` taken out of the pass for the call); the
reviewer saw the final chain.

- **Argument** (`panel-argument`, Opus 5.5 at `high`): no break; each
  of the eight claims held. It stated each invariant from the code
  first: the unrestricted sum is the sum over the set at every moment,
  and the three functions keep it with the same `facts.used` and
  `shared`; the rest abstraction of the harnesses over-approximates any
  real zone and sits on the zone not poured; `Failed::fault`, the
  `excess` walk, `predicate_open`, the merged phrase (both notes byte
  for byte), `horn_member` (`None` exactly where each reader gave it),
  `chosen`, `Postorder` and `chain` (the same terms, in the same order,
  with the same memo entries, and the table's new terms seen by
  `substituted_term`), `named`, `applied` and `KNOWN` each equal what
  they replace.
- **Integers** (`panel-integers`, Sonnet 5.5 at `high`): no
  refutation. It drove the three functions at the largest sum the
  invariant allows (a start of (2³² − 2)(2³² − 1) and weights of
  `u32::MAX`), ran every one of the 256 fragment values against the old
  match (`named` below 12 for all), and type-checked the harnesses under
  `--cfg kani` with a stub; it named the swap harness as the one the
  solver might not finish, which is the one that ran out of memory.
- **Counterexamples** (`panel-counterexamples`, Opus 5.5 at `high`): no
  counterexample in 96 740 inputs, the head's and the base's commands
  giving the same exit status, standard output, standard error and
  files: `prove` in every format and mode, `seq`, `check` of proofs and
  disproofs and 17 207 mutants of them, every disproof kind and 10 512
  mutated disproofs, the refutation phrases, sessions with cuts and
  `eliminate` with `--at` at every node, 452 proofs with shared
  subproofs made by merging the engines' (so `&` unites zones over
  shared parts) and their mutants, ordinary logic, rule names from
  session files (every name and spelling, unknown ones), fragment
  names, 19 822 first-order texts and mutants, random sessions, the
  Lambek and cyclic modes. Eight runs without `--deterministic` vary from
  run to run in either build alike; with the flag they agree. Its own
  references of the excess interval (194 refutations) and of the
  fragment (400 sequents) agreed with the head.
- **The reviewer** (`fresh-reviewer`, Opus 5.5 at `high`): no break of
  behaviour; 377 tests of `linear::` passed in its run; one finding of
  the records: F-1c-08's second half (the `Invalid` arm written twice)
  was dropped without saying so, now marked "in part" with its reason
  under the departures.

## What the next steps must know

- The checker's arithmetic on the unrestricted zone is in
  `State::add_unrestricted`, `remove_unrestricted` and
  `unite_unrestricted`; a change of it goes there, where the harnesses
  check it.
- `weighted::horn_member` is the one reading of a goal's member as a
  Horn program for the two checks; `Program::read_with` (the engine's)
  is apart on purpose.
