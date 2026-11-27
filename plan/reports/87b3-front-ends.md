# Step 87b3: front ends that answer wrongly (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the
workspace `../deduxit-wt-step-84-dc9a99`, from `plan/87-refactor.md`
(87b3, wave B, after 87b1) and the planning session's briefs
`start-87-common.md` and `start-87b3.md`, on cores 4 to 6, the reviewer
on 13 to 15. Based on 87b1's head (`wxwpxkyl`). It changes behaviour,
each change a fix with a test that fails without it. Only `cli/` and
`web/` change, with their docs: the library's API is untouched.

## Outcome

Every finding of the step is fixed. Per finding, what a user of 0.8.0
sees change (the changelog's "Fixed", and one "Added"):

| finding | change | what a user of 0.8.0 sees | commit |
|---|---|---|---|
| F-7b-01 | `verdict_status` reads a node's verdict by its kind, as its command decides it | `workspace apply`/`run` exit 1 for a resolution that finds the clauses satisfiable and an interpolation of a sequent that is not valid, where they exited 0; a Herbrand search without instances is 3, an empty type and a count of no term 1 | `trsunpqz` |
| F-7b-02 | `eliminate --net`'s verdict goes through `io::write` | piping it into `head` keeps the exit status, where it panicked with 101 | `pvwmsxyq` |
| F-7a-07 | `Cli::dash_is_standard_output`, after parsing, reads `--output -` as no path | `-o -` writes to standard output, where it made a file named `-` | `tpqtsrtr` |
| F-7a-02 | every list of mode flags names `--cyclic`, `--lambek` (and `--empty-antecedents` where it matters) | `prove`/`interact --logic … --cyclic` are refused (exit 2), where the flag was dropped; a settings file's `batch.mode` no longer beats `--cyclic` or `--lambek`; `interact --calculus`, `--state` and `grade --calculus` refuse them | `tpqtsrtr` |
| F-7b-03 | `CheckArgs` splits into `ProofArgs` (the proof file, mode, bounds, output, settings), which `eliminate` flattens alone | `eliminate --tableau`, `--dimacs`, `--system`, `--world`, `--layout`, … are clap's errors and out of its help, and `--lambda` is refused, where they ran and were ignored | `tpqtsrtr` |
| F-7b-04 | `TermLimitsArgs` (`--timeout`, `--memory-limit`, `--work-limit`) flattened into `term check`, `reduce`, `infer`; a `Deadline` and Ctrl-C as their stop; a refusal through `refused` | `term reduce --steps none` ends at the time limit, unknown (exit 3), where it ran until killed; a refusal is `unknown:` naming the flag, where it was an error naming a settings key | `tpqtsrtr` |
| F-8-02 | the view builds one derivation, its size bounded by the tab's memory | a page's view of a large session keeps to the tab's 96 MiB, where it built two derivations under no size bound | `wvrnszsm` |
| 87b1's note | `refutation_checked` is `check_within` under the tab's limits | `prove` and `close` answer `null` where a bound ended the check, where the check ran unbounded | `wvrnszsm` |

## Departures

- **F-8-02 takes the interim fix** (departs from the plan's "add
  `invertible_steps()` and a cut list to the linear `Interactive`"):
  part 1 adds nothing public that the step does not name, and the brief
  keeps this step to `cli/` and `web/`. The view builds one derivation
  in place of two. A finding the audit missed: `derivation_within`
  charges no memory at all, so the tab's limits with
  `derivation_bytes` lifted (the verdict's interim) would bound nothing.
  The derivation's estimated size is bounded by the tab's
  `memory_bytes` instead. Past it the lists are empty, as before past
  any refusal. The library's lists, which need no derivation, are part
  2's.
- **F-7b-01 maps the kinds in the command**, not through a
  `protocol::verdict` in the library (the verdict's "until then" path):
  the library's API stays. A new kind with a verdict needs its arm in
  `verdict_status` (cli.md says so).
- **F-7b-03: `--lambda` stays in `eliminate`'s help** (it belongs to the
  shared output group, as the verdict foresaw) and is refused at run
  time. The other `check`-only flags left its help.
- **`refutation_checked`'s `null`** now also means "a bound ended the
  check". The client reads it as a status light; a `null` beside an
  unprovable outcome was impossible before. (departs from the plan:
  the brief named the call, not the meaning of the value.)

## The net

- **No target set**: nothing under `core/` changed, so neither engine
  can have moved.
- **The gate** (cores 4 to 6): passed on the chain before the review's
  fixes, and again at the final head (`vknkznko`), after the pause. The ratchet's journeys are
  within their ceilings. The misses of `search-chain-128` and
  `check-qbf-20-2` are named after the table, as in every gate since
  step 84.
- **`breaks 0.8.0`**: nothing of this step, which changes no library
  code. Its three failures are main's: `StepError::Witness::reason` and
  the related fields, System F's variants, and
  `Workspace::read_own` marked deprecated (new since 87b1, by a step
  that landed on main).
- **The bindings under Node** (`web-wasm-tests`, under the machine
  lock): passed. `the_view_keeps_to_the_tab_s_memory` and the unit test
  `a_refutation_is_checked_within_the_limits` fail on the old code (599
  steps listed under a 1 MiB bound; `Some(true)` under a work bound of
  1).
- **The behaviour lock** was reblessed in a commit of its own
  (`otuzlyzu`). Three help pages moved: `--output`'s line, and in
  `check` the shared group's flags come first, so `--derivation-limit`
  and `--layout` change places.
- **`nix flake check`** at the final head, under the machine lock: all
  checks passed (this line still a placeholder there). The first run
  failed only `treefmt`: two files edited from the shell were never
  formatted. The formatting is folded into `wvrnszsm` and `tpqtsrtr`.

## The review

`fresh-reviewer` (Fable 5.2, medium) reviewed the chain. It confirmed
each finding fixed and the workspace's statuses equal to the commands'
on eleven sequents across every kind. Its four findings, all fixed in
their commits:
- **`term infer` still exited 2 at a reading bound**, where `check` and
  `reduce` had come to answer unknown. That move was not asked for, so
  the reading's refusal is an error again in all three (as in `term
  inhabit`): only the check, the reduction and the inference answer
  unknown at a bound. `term check` keeps an ill-typed reading a verdict
  (exit 1).
- **`eliminate` still took `--lambda-notation` and `--lambda-types`**
  and ignored them. They are refused with `--lambda` now, and the test
  covers both. All three stay in its help, being the shared output
  group's (departs from the plan).
- **A changelog clause** listed `--sequent` and `--calculus` among the
  flags `eliminate` ran with. It refused them before; the entry says
  so now.
- **An interpolant with an undecided half**: `interpolate` exits 3 for
  one, and the workspace exited 0 for any interpolant. The protocol does
  write one, so a node's interpolant now counts only once its check
  held. No input that reaches the case was found.

## What the next steps must know

- **87c9** (the command, the flags declared once): `ProofArgs` is the
  first shared group of flags (`check` and `eliminate`), and
  `TermLimitsArgs` the term commands'; `Cli::dash_is_standard_output`
  lists every command with an `--output`, so a new one joins it. The
  workspace's `verdict_status` is a table by kind that a library
  `protocol::verdict` would replace.
- **Part 2**: the linear `Interactive` would give `invertible_steps()`
  and its cuts without a derivation, as the ordinary session does; the
  bindings' `View::of` then drops its derivation.

## Commits

Oldest first, change ids:
- `wvrnszsm` Keep a session's view and a refutation's check to the tab's
  limits
- `trsunpqz` Exit a workspace node as its kind's command does
- `pvwmsxyq` Write eliminate --net's verdict through the command's
  output
- `tpqtsrtr` Honour or refuse every flag a command offers
- `otuzlyzu` Lock: the help of --output and check's regrouped flags
- `vknkznko` Changelog: the command's and the bindings' wrong answers
- `rvykkpsr` Plan: 87b3's report

## Paused

The step paused at 20:28 for the author's machine, the final gate cut a
few minutes in, and resumed the same evening: the gate ran again in
full.

A concurrent operation left the last four commits divergent after the
formatting was folded in: one copy formatted, one from before. The
stale copies and the old working copy were abandoned by commit id; the
chain kept is the one the flake check ran on.
