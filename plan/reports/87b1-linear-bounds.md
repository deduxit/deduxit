# Step 87b1: bounds and misleading calls of the linear suite (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the
workspace `../deduxit-wt-step-84-dc9a99`, from `plan/87-refactor.md`
(87b1, wave B, beside 87b2) and the planning session's briefs
`start-87-common.md` and `start-87b1.md`, on cores 4 to 6, the panel on
13 to 15, the target sets on 2 and 3. Based on 87a's head (`qzkqqqtp`).
It changes behaviour, each change a fix with a test that fails without
it.

## Outcome

Every finding of the step is fixed, the public API only added to: four
bounded forms and one error variant, each named below. Per finding,
what a user of 0.8.0 sees change (the changelog's "Fixed"):

| finding | change | what a user of 0.8.0 sees | commit |
|---|---|---|---|
| F-6a-01 | the readers of a proof, a structure and a session build their sequent within the caller's limits | `check --file … --term-limit 1` refuses a proof whose terms pass the bound, where it checked it; a page's `limits.terms` holds on its session | `vvxypqrt` |
| F-6a-02 | the command reads an intuitionistic JSON sequent through `wire::upgrade`; `Within`'s docs say its error is text | `prove -i` past `--occurrence-limit` says "…; raise it with --occurrence-limit", not "not an intuitionistic sequent in JSON" | `llzuvlxt` |
| F-1c-01 | `Disproof::check_within(&limits, stop)` (added); the protocol and `check` call it | `check` of a disproof ends at its time limit and Ctrl-C with no thread left behind, and honours `--occurrence-limit` and `--work-limit`; the protocol's check of `linear.decide` asks its stop | `qxwuypyl` |
| F-2a-01 | a refused re-check on a forest with cut formulas is `Unknown` through the one mapping `conclude` uses | `prove` of a sequent with a cut whose check meets the stop or a bound answers unknown, not `Error::Check` | `nvsrvqzy` |
| F-2b-01 | `Interactive::split_passes_within(goal, &step, &limits, stop)` (added), the count rows charged and the stop asked; the command and the bindings call it | one ⊗ step on a nested tensor of 16 000 atoms no longer takes 1.5 GiB in `interact` or on the page: it is refused at the memory bound | `mqvusllr` |
| F-4f-03 | `split_passes` judges the step's own rule | a Mix on a `⊗` formula is judged as a Mix (the page no longer greys a Mix that would close, and `interact` warns of a lost one); a `⊗` elsewhere is refused as `apply` refuses it, any other rule `StepError::NoSplit` | `mqvusllr` |
| F-4f-07 | the invertible run ends with `Refusal::Terms` (or `Index`) at a `∀` step past the session's bound | `invertible on` in a first-order session says the term limit stopped it, where it left the `∀` open and called the run finished | `wmtxkzzq` |
| F-4e-07 (linear half) | `invert_within`, `invert_all_within` (added), a stop per step; the command passes Ctrl-C | Ctrl-C ends a long invertible run in `interact`; the session's line names the refusal | `wmtxkzzq` |
| F-3-03 | the MLL parts indexed by label, the Mix branch's labels sorted (the MELL copies' fixes) | a disconnected structure's check and a Mix net's sequentialization at 80 000 parts take 5.8 ms and 0.88 s, not 1.96 s and 8.56 s | `uwmrllov` |
| F-8-01, F-3-01 | `nets::net_of_proof` (crate-private, feature `protocol`) is the choice of a proof's net; the protocol calls it | `linear.net` gives a net with boxes for a proof that cuts one occurrence twice, where it answered `invalid_net` | `wsorrzwm` |
| F-1a-07 | `Error::InstanceTable` (added, code `instance_table`, kind malformed) from `Proof::with_instances` | a table built in Rust is refused as an instance table, not as JSON; a table read from a proof's JSON stays `json` | `nunozopn` |
| F-1a-14 | `Sequent::atom` composes the name first | a decomposed `é` finds its atom | `qoospwyn` |
| F-2c-01 | `power` and the joins write their connective's unit for none | `three_partition`, `three_partition_mll` and `partition` give a sequent for an item of size 0, no bin or no item, where they panicked | `puwxsvrk` |
| F-2a-13 (part) | the name read for an engine this build does not know is kept (`Outcome::named`, crate-private) and written back | a read and re-written outcome keeps its engine's name, where it became `unknown` | `rqpwuzkt` |

## Departures

- **F-2a-13 is done in part** (departs from the plan): `Engine::Unknown`
  stays a value `Options::engine` takes, refused at run time as before;
  removing it from the options' type is the proposal's P8, in part 2.
  Part 1 keeps the name on a re-written outcome.
- **F-4e-07's linear half adds forms, the signatures stay**: the stop
  goes into `invert_within` and `invert_all_within`, `invert` and
  `invert_all` calling them with no stop, as 87b2 does for the ordinary
  session (the same names and `Phase::View`, agreed between the two
  sessions; `inverted_line` in the command is shared and both changed
  it).
- **F-2b-01: `split_passes` itself stays unbounded**, its docs saying so
  and naming the bounded form, since a `StepError` cannot carry a
  refusal; the command and the bindings call `split_passes_within`.
- **F-6a-02: `wire::Within` stays public**, its docs saying that its
  error is text only (the verdict's refactor-plus-text reading); the
  command's one caller of it for an intuitionistic sequent moved to
  `wire::upgrade`. The one-sided path still uses it with no bound.
- **F-1c-01: the bindings' check of a refutation stays `check()`**: the
  call has no settings in hand (`refutation_checked`), and the bindings
  are 87b3's; noted for it.
- **F-3-01: the command's `net_of` and the bindings' `Net::of` keep
  their copies** until P15 makes `net_of_proof` public.

## The net

- **Target sets** (cores 2 and 3, lent by the planning session, at
  `snvtzzlq`): `summary --against bench/targets/86.csv
  bench/targets/87b1.csv --counters` gives no verdict that differs and
  no decided row whose counters differ (145 decided alike, 0 differ);
  the net set against `net-86.csv` likewise (64 alike, 0 differ). No
  counter moved, so no fix needs to explain one. The commits after
  (`kkqttvtn`, `snwrznpx`, `tnwnlokq`, `nxtnyorl`, `tmontzrm`) touch the
  interactive split test, the saturated set's check and the rules. Only
  `nxtnyorl` touches a file of the focused engine (`focus/counts.rs`):
  its `new_until` now calls `new_polled` with the same gate it had
  inline (`set_up_stopped`), so the engine's set-up polls exactly where
  it did, and its counters cannot move. The ratchet's search journeys
  measure it (below).
- **The gate** (cores 4 to 6): passed, last at `tmontzrm`, after the
  rebase and the review's fixes. That covers
  clippy on the workspace and the core crate without `parallel`, the
  tests, both `cargo hack` runs, and the ratchet. Every journey is
  within 2 % of its ceiling. The misses of `search-chain-128` (+41 %)
  and `check-qbf-20-2` (−8 %) are named after the table. Both moved the
  same way in this workspace's gates for steps 84 and 87a, before this
  step, so this step did not cause them.
- A first run of the gate failed the core crate's check without default
  features: `Outcome::named` is never read without `serialize`. It now
  carries the `cfg_attr` `expect(dead_code)`, folded into `rqpwuzkt`.
- **`breaks 0.8.0`**: nothing of this step but its additions.
  cargo-semver-checks' two failures, and the diff's removed and changed
  lines, are main's since 0.8.0: `StepError::Witness::reason` and the
  related fields, and System F's variants and `Option` types. The diff
  adds `Disproof::check_within`, `split_passes_within`,
  `invert_within`, `invert_all_within` and `Error::InstanceTable`.
- **`nix flake check`** under the machine lock at the rebased head
  (`wxwpxkyl`, this line still a placeholder there): all checks passed.
  The first run failed two checks, both fixed and folded into their
  commits:
  - `treefmt`: three files I edited from the shell were never
    formatted;
  - `wasm`: clippy's `useless_vec` on a test of `snwrznpx`, which the
    gate's clippy did not report.

## The panel

Three members on the refutations' check, the split test and the
invertible run, on cores 13 to 15, each on a copy of the tree at
`rqpwuzkt`.

- **Integers** (Sonnet 5.5, high): four findings, all fixed in
  `kkqttvtn`.
  - `split_passes_within` named `limits.work` as a bound, but the counts'
    pass tells its stop no units, so the bound never held. The claim
    is gone, and the docs say so.
  - Its `Refusal::Index` named the forest's length, which is below the
    limit. It now names `u32::MAX + 1` against `u32::MAX`.
  - `split_passes` reached `unreachable!` on a forest past the counts'
    indices. This was already there before the step (`Counts::new`
    panicked the same way). It now answers that the split may pass.
  - `check_within`'s docs promised a stop at `limits.work`. The check
    polls every 2¹⁶ units, so it can run up to one poll past the bound.
    The docs now say it ends at the first poll past it.
- **Argument** (Fable 5.2, high): one finding, fixed in `snwrznpx`.
  - `saturated::write` writes a rule's phase, and the goal's, without a
    poll. An `&` under each `⅋` doubles the holes, so a set's goal
    phase alone ran 18.9 s and 2.24 GB under `check --timeout 1s` (23
    pairs). Since `qxwuypyl` the command no longer runs the check on a
    thread of its own, which had masked this.
  - `write` now counts a unit per step and the members of each `&` it
    copies, and asks the stop. Both witnesses (23 pairs, and 22 under a
    `!`) end in 1.07 s with `unknown`, exit status 3.
  - Pinned by `a_saturated_set_s_goal_phase_asks_the_stop`.
- **Counterexamples** (Opus 5.5, high): it found no wrong answer. Its
  one finding is the goal phase's unpolled writing, the argument
  member's, already fixed in `snwrznpx`: `(p & q) ⅋ …` with an empty
  set ran to `Ok(())` without a poll, 5.9 s at 24 pairs.
  - The checks were made against a MALL prover of its own, which shares
    no code with `reference.rs`.
  - `check_within` agreed with `check()` on 26 916 disproofs of every
    kind. On top of those it ran 107 303 stops at a poll, about 849 500
    work bounds and 67 529 mutated refutations, and every answer was
    `check()`'s or the refusal.
  - The re-check on a forest with cuts: 8 486 runs, every proof checked.
  - The split test: 185 574 tests agreed with `apply`. Of the 24 560
    refused, neither prover proved both premises.
  - The MLL criterion and sequentialization were byte-identical to
    `qzkqqqtp` on 6 040 structures, up to 20 000 parts.
  - `linear.net`: 3 434 proofs with cuts, 1 797 of which cut one
    occurrence twice; each gave a correct net that held.

## The review

`fresh-reviewer` (Fable 5.2, medium) reviewed the chain at `tnwnlokq`,
rebased. It had four findings, all fixed:
- **`saturated::write` built the axioms' partners before a poll**: a
  focus on `n` literals beside `n` duals has `n²` of them, 2 GB at
  16 000 before the first poll. Each literal looked at is a unit of work
  now (`tmontzrm`). This is the same class as the panel's finding.
  `a_focus_s_partners_are_counted_work` fails without the fix (0 polls).
- **The split test asked its stop only on forests of 65 536 occurrences
  or more**, as a search's set-up does. Its rows are quadratic in a
  smaller forest too: 32 002 occurrences took 1.5 GB with no poll. A
  separate `Counts::new_polled` now polls whatever the size; the engine's
  `new_until` is unchanged (`nxtnyorl`).
  `the_bounded_split_test_keeps_to_its_bounds` now stops at 2 000 atoms
  and fails on the old poll.
- **The changelog said `interact` named the memory limit for every
  refusal.** 87b2's arms were already there before this step; what
  changed is that a run past the bound on terms printed no refusal. The
  entry says so now.
- **No test for F-3-03, and none for the structure with boxes in
  F-6a-01.** The second is added (`MellStructure` in
  `readers_build_their_sequent_within_the_caller_s_terms`). The first
  stays without one (departs from the plan): the fix changes no result,
  only the time, from quadratic to linear in the parts. A test that
  fails without it would be a timing test. The panel's byte-identical
  comparison against `qzkqqqtp` and the timings above are its evidence.

## What the next steps must know

- **87b3**: the bindings' `refutation_checked` (web/src/calls.rs) still
  calls `Disproof::check()` with no bound; `check_within` with the tab's
  limits is one line once a settings value is in hand. The command's
  `net_of` and the bindings' `Net::of` are the hand-made copies of
  `nets::net_of_proof`.
- **87c4** (the nets' kernel): `Graph::parts` and the Mix branch now
  have the MELL copies' shape, so sharing them is closer.
- **Part 2 (P15)**: `nets::net_of_proof` and `nets::Net` are
  crate-private behind `protocol`; making them public is the proposal's
  step. P8 takes `Unknown` out of `Engine`; the crate-private
  `Outcome::named` then becomes the outcome's engine type.

## Commits

Oldest first, change ids:
- `qoospwyn` Find an atom by its name composed or decomposed
- `puwxsvrk` Encode an item of size 0 and no bin or item in the public
  encodings
- `vvxypqrt` Build a document's sequent within the caller's limits
- `llzuvlxt` Read an intuitionistic JSON sequent through wire::upgrade
- `qxwuypyl` Check a refutation within limits and until a stop
- `nvsrvqzy` Answer unknown where the re-check on a forest with cuts is
  given up
- `mqvusllr` Judge a split's own rule, and bound the split test
- `wmtxkzzq` Give the linear invertible run a stop, and end it at the
  bound on terms
- `uwmrllov` Index a disconnected MLL net's parts, and sort Mix's labels
- `wsorrzwm` Choose a proof's net in one place, which the protocol calls
- `nunozopn` Refuse an instance table built in Rust as an instance table
- `rqpwuzkt` Write back an outcome's unknown engine by the name it was
  read with
- `snvtzzlq` Changelog: the linear suite's bounds and misleading calls
- `kkqttvtn` Split test: no claim of a work bound, no panic past the
  counts' indices
- `snwrznpx` Ask the stop while a saturated set's phases are written
- `tnwnlokq` Rules: the split test's bounds and the saturated check's
  polled writing
- `nxtnyorl` Ask the split test's stop on a forest of any size
- `tmontzrm` Count the partners a saturated check's axioms look at
- `wxwpxkyl` Plan: 87b1's report and target sets

Rebased onto `main@origin` at `uxqyqzlo` (87b2 landed), then again,
without a conflict, onto `nskzrsnk` (87c1 landed). The gate passed at
the final head. The full check passed at the head before that last
rebase. 87c1 changed the ordinary and modal suites, the arena and the
changelog, none of this step's code. The one
conflict was `inverted_line` in `cli/src/interact/mod.rs`, resolved in
`wmtxkzzq`: it keeps 87b2's `Stopped` arm and this step's `Terms` arm,
and its doc names both.
