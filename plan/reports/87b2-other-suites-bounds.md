# Step 87b2: bounds and misleading calls of the other suites (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-82-141614`, from `plan/87-refactor.md` (87b2, wave B)
and the planning session's briefs `start-87-common.md` and
`start-87b2.md`, on cores 10 to 12 (the panel's on 7 to 9), beside 87b1.
The base is `qzkqqqtp` (87a). Every commit keeps the public API but for
the additions named below; each fix but two has a test that fails
without it (the two are said where they are).

## Outcome, per finding: what a user of 0.8.0 sees change

| finding | commit | change | a user of 0.8.0 sees |
|---|---|---|---|
| F-6b-16 | `kvymzwuz` | `pdf::Date::checked`, called by `pdf::from_svg` | a date off the calendar (month 13, 30 February, 24:00) is `InvalidOption`, key `pdf.date`, where it was clamped (month 13 wrote December) |
| F-5b-11 | `tqsrnurm` | `rules::Calculus::parse_sequent`, `parse_list` count the whole text's tokens | a rule file's sequent of many short members past `limits.occurrences` is refused, as the docs said |
| F-5b-01 | `qtoqtrox` | the workspace's known settings keys come from `Reason::SETTINGS` and `Refusal::SETTINGS` (crate-private, behind `protocol`), tested against every variant | a run ended by `search.path_length`, `search.model_size` or `search.model_worlds` keeps that setting in its record and in `workspace show` |
| F-5b-07 | `wmppzmzm` | `Workspace::read_trusted` with a `# Security` section; `read_own` a deprecated wrapper | a new name and a deprecation warning; no behaviour |
| F-4c-10 | `rvnxtuzo` | `first_order::Refutation::check_within(limits, stop)` (work, memory, terms); `check(stop)` its wrapper as before; the search checks within its limits | a library caller gets the bounded form; the search's check of its own refutation keeps to the caller's work and memory bounds, and an atom past the term bound is that refusal, where it was a failed check |
| F-8-12 | `nstrknkl` | the protocol's replay of a first-order resolution asks the stop per step | a stopped check is part `refutation` `open`, where the check ran to its end (no clock in a workspace) |
| F-4e-07 (ordinary) | `nsnkzmwr` | `ordinary::Interactive::invert_within`, `invert_all_within`; the short forms are wrappers | a library caller can stop a long run; Ctrl-C ends one in `interact --logic` ("N invertible steps, then interrupted") |
| F-4e-02 | `rntmqqlr` | the command's forward step lets the library refuse a goal that is not open | nothing (the library's words are the command's copy's) |
| F-4c-09 | `tzlwrssv` | Herbrand's `skolem_terms` and `nesting` memoised per term, walked as graphs, `sort_by_cached_key` | nothing but speed (identical derivations) |
| F-4b-05 | `lvzqxwwv` | `WrittenTableau::review` refuses a calculus other than `rules::TABLEAU` (`is_tableau`) | another calculus passed is `NotApplicable` before anything is reviewed |
| F-4a-01 | `kvukwskk` | `ordinary::Derivation::check` refuses a broken rule with `InvalidDerivation`; the crate's own read-backs map it to `ReadBack` (`own_fault`) | a caller matching `ReadBack` from `check` matches `InvalidDerivation`; `deduxit check` of a broken derivation says "not a derivation in the calculus LK: …", code `invalid_derivation` |

The additions `breaks 0.8.0` shows: `Refutation::check_within`,
`Interactive::invert_within`, `Interactive::invert_all_within`,
`Workspace::read_trusted`, and `read_own` deprecated (cargo-semver-checks'
`type_method_marked_deprecated`); the removals and changes it lists
beside them are earlier steps', in the changelog's breaking list
already.

## Departures and what each fix left

- **F-4e-02** (departs from the plan): the rule session's `goal`,
  `members`, `hypotheses` and `lines` keep their panic on an id past the
  session's nodes, documented. Their `Result` is the API proposal's P13
  ("one goal of a session: `Result`"), part 2; part 1 keeps the API. What
  part 1 could fix is the command's copy of the library's refusal: the
  forward step now calls the library first and reads the goal's members
  after (a closed goal keeps its sequent). The refusal's text was the
  same, so no test can fail without it; the behaviour lock pins the
  message (`interact-calculus-labelled.txt`).
- **F-4c-09**: no test fails without it; it changes no output. Measured
  with callgrind on the doubling family's read-back (`resolve`, release):
  n = 12 101.7 M to 101.1 M instructions, n = 14 448.1 M to 445.5 M
  (0.6 %); wall time at n = 16 to 20 unchanged (0.14 to 3.4 s). 95 % of
  the instructions are `FirstOrder::atom_key`, the atoms' keys written as
  text (`predicate_atom`, `instantiate_frame`, the checker's `Find`):
  with fixup-31's bound the walks are a constant factor, as the verdict
  expected. The keys are the cost a later step of wave D could take on.
- **F-4b-05**: the full check needs serde_json, under `protocol` only
  (the default); without it the name and the rule names are compared,
  which a file of the same names with laxer patterns passes. Taking no
  calculus and building `TABLEAU` inside `review` is an API change, part 2.
- **F-4e-07**: the bindings have no stop to pass (a tab ends a call by
  stopping its worker) and keep the short forms. The stop's phase is
  `Phase::View`, the phase of the run's memory refusal; 87b1 was told
  (the linear session's half is theirs), and of the change to the
  command's shared `inverted_line`.
- **F-5b-01**: the list for `Reason` lives in
  `core/src/linear/search/mod.rs` (87b1's area): a crate-private const
  behind `protocol`, read by the workspace only, no search code.
- **F-4a-01**: the call sites keep calling `check` as they did and map the
  error on its cold path (`own_fault`, `#[cold]`), since a helper around
  the check was once instantiated anew and cost `ordinary-pigeons` 0.45 %
  (`core-ordinary.md`). `interpolant` keeps the plain check: it is given
  a derivation, and `interpolate` checks its own through decide first.
  The command's two read-back paths take `InvalidDerivation` as a defect
  too, so a broken read-back stays an error (exit 2) and is never hidden.
- **Found on the way, fixed in its commit**: `check_within` made fixup-31's
  read-back test refuse at its memory bound of 16 KiB before the
  read-back was reached (the arena copy is 17 KB); its bound is 24 KiB now,
  which the check fits and the read-back's key (32 KiB) does not.

## The net

- `cargo clippy --workspace --all-targets -- --deny warnings`: clean at
  the head.
- `cargo test --workspace` before the last two folds: every binary
  passed (39); the gate's run after them.
- The gate at the head (capped, cores 10-12): passed. Its first run
  failed in `cargo hack`'s `--no-default-features --features protocol`:
  `Phase` imported in the protocol module and read only under `sat`,
  fixed in `nstrknkl`. The ratchet: every journey within its tolerance;
  `ordinary-resolution-first-order` 5 961 965 instructions against
  5 930 262 (+0.53 %, the check's and the read-back's new bounds), the
  rest within ±0.1 %; misses moved on `search-chain-128` and
  `check-qbf-20-2`, a note and never a failure.
- Both target sets (cores 2 and 3, given by the planning session) equal
  86's: `bench/targets/87b2.csv` against `86.csv`, no verdict differs and
  0 of the 145 problems decided alike differ in any counter;
  `net-87b2.csv` against `net-86.csv`, none and 0 of 64.
- `breaks 0.8.0`: above.
- `nix flake check` at `kvukwskk` (the last code commit, on main at
  `oowyxznx`) under the machine lock, cores 10-12: all checks passed
  (`features`, `features-tests`, `ratchet`, `rocq-deduxit`, `wire-schema`,
  `web-wasm-tests`, `web-core-wasm32` among them).

## The panel

Run on the three commits that touch a checker's entry (`rvnxtuzo`,
`kvukwskk`, `lvzqxwwv`) and, for the integers, every new bound, on cores
7 to 9, as the brief asks where a fix touches a checker.

- `panel-argument` (Fable 5.2, high): the argument holds for all three.
  The first-order check accepts exactly what it did within the bounds,
  every new exit is a refusal, and the search's own check replays under
  the search's term bound; `own_fault` rebuilds the old error value bit
  for bit at every crate site; the tableau's comparison by JSON value is
  injective and fails closed. Two points, both answered:
  - the assertion on the laxer tableau file failed in a build with
    `parse`, `serialize` and `interactive` but not `protocol` (no check
    builds it: only `protocol` implies the three): now behind
    `#[cfg(feature = "protocol")]`, in `lvzqxwwv`;
  - the command's own read-back checks (`cli/src/ordinary.rs`, the
    drawing's and the embedding evidence's) reported a refusal as
    `InvalidDerivation`, code `invalid_derivation`, where it was a
    defect: the command now wraps it as "the derivation read back fails
    its check, a defect of deduxit to report" (`read_back_fault`), exit 2
    as before, in `kvukwskk`; the changelog says so.
- `panel-integers` (Sonnet 5.5, high): no wrap, truncation, panic or
  accepted wrong answer; each bound exact at 0, 1, n − 1, n, n + 1 and
  the largest values. One edge, answered: `Date::from_unix` clamped a
  year past 65 535 but kept the month and day, so the leap day of the
  year 80 000 became 29 February 65535, which the new check refuses: a
  moment past 65 535 is now its last second (in `kvymzwuz`, tested). Two
  notes kept: `resolve` answers a refusal of the bounded check as `Err`,
  as its search answers its own memory bound (first-order resolution's
  `Bound` has no work or memory case); and `Inverted::finished`'s doc in
  the linear session's file says only the memory bound ends a run (87b1's
  file, told).
- `panel-counterexamples` (Opus 5.5, high): no refutation of `kvukwskk` or `lvzqxwwv`: about
  2 800 derivations the crate built in every logic and method checked Ok,
  54 000 mutants of them were refused with `InvalidDerivation` (never
  `ReadBack`) or accepted with a valid root, `deduxit check` exited 1 on
  every broken document its reader takes; the tableau file and three
  respellings were accepted, ten other calculi and all 640 mutants of the
  tableau file that read were refused. For `rvnxtuzo` 25 600 mutants of
  real refutations gave `check` and `check_within(default)` the same
  answer, and a text-level reference checker of its own agreed. Its
  refutation of `rvnxtuzo`, kept as the change asked for (departs from
  0.8.0's behaviour, which the step's entry allows): a search that finds
  `□` under a work or memory bound that its check passes answers that
  refusal where 0.8.0 answered valid (`forall x. (p(x) -> p(f(x))), p(a)
  |- p(f⁶⁰(a))` under `--work-limit 100`: exit 3, where 0.8.0 exited 0
  with a check that ran past the bound; 194 of 1 200 generated sequents
  under memory bounds of 2 to 64 KiB). F-4c-10 asks for the check within
  the caller's bounds, and a verdict whose check a bound refused is
  unknown by the crate's rule for a check after a decided search
  (`decide::ended`, fixup-21); the changelog says it. Two smaller points:
  a minimal-logic derivation is refused naming `LJ` (`Logic::calculus`),
  the changelog corrected; and the test's `protocol`-only assertion, as
  the argument member found.

## The reviewer

`fresh-reviewer` (Fable 5.2, medium) on the eleven commits: ten correct
and complete for their findings and verdicts. One break, reproduced and
fixed in `nsnkzmwr`: Ctrl-C's flag is sticky, so after a `close` that
Ctrl-C stopped, the next invertible run in `interact --logic` stopped
before its first step ("0 invertible steps, then interrupted"); the
session now clears the flag before each run, as `close` does. Its
witness (pigeonhole 9 into 8, `close`, SIGINT after 3 s, `invertible on`)
now takes 9 497 steps to the memory bound, as without the interrupt. No
committed test covers a signal in a session (the reviewer's script is in
the scratchpad); the behaviour lock's sessions take no signal. Also
answered: four new comments told the old defect in the past tense
(`writing.md` rule 2), reworded to say why; and the report's placeholders,
filled here.

## Follow-ups (outside this step)

- **A document the command writes that its own `check` refuses** (the
  counterexamples member, found while testing): `deduxit prove --logic
  classical --method connection --format json "(false /\ ~false) /\
  ~(false /\ a) |-"` writes a derivation at level 8 with `"atoms": []`
  and `"fo": [{"E": 0}]`, and `deduxit check` of it exits 2, "a term
  refers to atom 0, but the atom list has 0 names" (3 of 14 sampled valid
  documents). A defect of the writer or the reader, older than this step.

## What the next steps must know

- 87b1 gives the linear session the same `invert_within` and
  `invert_all_within` with `Phase::View`, and adds to the shared
  `inverted_line` (`cli/src/interact/mod.rs`) an arm for `Refusal::Terms`
  beside this step's `Stopped` arm: the second of the two to land keeps
  both arms.

- Part 2's P13 gives the rule session's accessors their `Result`
  (F-4e-02's other half) and may remove `Workspace::read_own`.
- `own_fault` is the one place a crate-built derivation's refusal becomes
  a defect; a new engine of ordinary logic calls `check` and maps its
  error with it.
- The workspace's known keys are `Reason::SETTINGS` and
  `Refusal::SETTINGS`; a new reason or refusal with a setting adds its key
  there, and `every_bound_keeps_its_setting` fails to compile until its
  variant is listed.
