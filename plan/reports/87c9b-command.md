# Step 87c9b: the command's decision, its flags and its sessions (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the
workspace `../deduxit-wt-step-84-dc9a99`, from the planning session's
briefs `start-87-common.md` and `start-87c9b.md`: the second half of
`plan/87-refactor.md`'s 87c9, as `plan/reports/87c9-command.md` divides
it. On cores 4 to 6, based on 87c9's head (`ptypmmzu`).

## Checklist (kept current, so a compaction loses nothing)

- [x] F-7b-08: one session driver over the linear and the ordinary
  session (`cli/src/interact/`). Done, `tzunntvu`: `Driven` and the
  shared `undo`, `invertible`, `save`, `invert_all`, `stepped`, `goals`,
  `opened_line`; the session tests pass unchanged.
- [x] F-7b-07: one writer of a rules derivation; PNG and PDF for `check`
  (`cli/src/calculus.rs`). Done, `woovtwku`: `made` and
  `list_past_limit`; the manual's PNG example is its test (refused
  before, exit 2).
- [x] F-7a-13: the common flags declared once (`argument_parsing.rs`,
  `settings.rs`). Done, `skkxpnlw` and the lock `vvqpzzpw`: `LimitArgs`
  with one `apply`; `mut_args` keeps five commands' words. `--settings`
  stays per command (its help names each command's keys).
- [x] F-7b-09: one map from a refusal's setting to its flag, on those
  groups. Done, `twsnrzks`: `settings::raise(setting, offered)` over
  `RAISED_BY`, seven files; the spelling with the value (`--work-limit
  N`) everywhere; two tests and one manual line moved, no lock line.
- [x] F-7a-12: one decision of a sequent for `prove` and the batch.
  Done, `nrmvqtro`: `cli/src/decide.rs` (`Read`, `Run`, `Answer`,
  `decide`), `ordinary::natively` in place of two `by_native`s; the
  writers take the answer. `close` and `grade` left (departs from the
  plan, below).
- [x] F-7b-05: **held** (departs from the plan), the planning session's
  decision: the command keeps its order; the library's `decide_within`
  is to give the model search a turn under a clock in a fixup after
  87c5, and the command takes the default there, measured against
  `bench/defaults/command-first-order-before.csv`. The held change is the
  side change `psvsuuvz`, parked off this chain as it is.
- [x] The lock reblessed in its own commit where a help page moved
  (`vvqpzzpw`); the manual and the guide with the output (F-7b-07's PNG
  example, F-7b-09's terms line); the changelog (three lines).
- [x] Gate (at `nrmvqtro` before the review), `breaks 0.8.0`,
  `fresh-reviewer` (Opus, high; six findings, all fixed), the CLI's
  tests after the fixes.
- [x] `nix flake check` under the lock at the head: all checks passed
  (a first run found an unresolved doc link in `decide.rs` and an
  unformatted test line, both fixed in their commits).

## F-7b-05 held: the library's default loses the model search under a clock

The library's default (`ordinary::decide_within`) runs the model search
last, only where the embedding ends unknown and was not stopped. The
command gives it one stop, the time limit. The embedding nearly always
runs until that stop: 1 375 classical and 1 785 intuitionistic rows of
the baseline end that way. So the model search never runs, and the 474
rows of the baseline it refuted (207 classical, 267 intuitionistic)
become unknown.

- The manual's example `prove --logic intuitionistic --format latex
  '~(forall x. p(x)) |- exists x. ~p(x)'` (a Kripke model of two worlds
  before) now ends "unknown … the time limit of 2s was reached at a copy
  bound of 281". That is the one failing readme example.
- A sample of the baseline's decided rows on the held build, a release
  build, `--jobs 1 --timeout 2s`, on core 4, six per (logic, status,
  method). The model search's refutations went 11 of 12 to unknown
  (classical 6 of 6, intuitionistic 5 of 6). Every other method's 63
  rows kept their verdict.
- No valid/not-valid verdict flipped in the sample. The full after run
  has not been made: a change that loses a whole method's refutations
  needs a decision first.

The planning session decided: keep the command's order in this step.

The fix is not in `cli/`. The library's default would need to give the
model search a turn under a clock: a work budget for the embedding, or
the model search before it within a work budget, as the connection
prover has `CONNECTION_WORK`. That is `core/src/ordinary/` (87c5's
area). The alternative is to keep the command's order and close the
finding as "the library's default is unfit under a time limit".

## What landed

| finding | change | commit |
|---|---|---|
| F-7b-08 | `interact::Driven`, the trait both sessions implement, and one `undo`, `invertible`, `invert_all`, `stepped`, `goals`, `opened_line` and `save` over it, in place of two copies | `tzunntvu` |
| F-7b-07 | `calculus::made` makes a rules derivation's certificate, Fitch proof or tree for both `check` and `prove --system`, and `list_past_limit` refuses past the bound once; `check` now renders PNG and PDF (additive, the manual shows it) | `woovtwku` |
| F-7a-13 | `LimitArgs` (`--timeout`, `--memory-limit`, `--work-limit`) flattened into every command that takes them, with one `apply`; `mut_args` keeps each command's own wording in place | `skkxpnlw`, lock `vvqpzzpw` |
| F-7b-09 | `settings::raise(setting, offered)` over one table, `RAISED_BY`, from a refusal's settings key to its flag, offered only where the command has the flag | `twsnrzks` |
| F-7a-12 | `decide::decide` decides one sequent for `prove` and a batch's entry; `Run` holds what differs (the batch's limit, one thread, `prove`'s race, notices, Ctrl-C, translation within the deadline, refusal advice, the output check); `ordinary::natively` is the one native step | `nrmvqtro` |
| F-7b-09 (rest) | `prove --system`'s refused evidence and `interact`'s `load` call `settings::raise` too; the changelog names `seq normalize`, not `interpolate` | `zzpnoutr` |
| F-7b-07 (fix) | `prove --system` draws a tree whatever `view.layout` says, as it did | `xnuqnmrt` |
| F-7b-05 | held, parked on `psvsuuvz` (above) | none |

Departures (each marked "(departs from the plan)"):

- **`close` keeps its own call** (departs from the plan). A session's
  goal is no sequent read: the linear session searches a goal inside
  its forest (`Goal::new`) and grafts the proof, and the ordinary
  session calls the library's `Interactive::close`. What it shares with
  the decision is the linear search's call, `searched`, which it
  already calls.
- **`grade` reads no answer value** (departs from the plan). `grade`
  decides no sequent: it grades an answer by checking it, and reads the
  verdict of a dozen check writers (`check.rs`, `modal.rs`,
  `calculus.rs`, `tableau.rs`) as their status and first written line
  (`kept`). A value there means each writer returning its verdict line,
  which is a change of `check`'s writers, not of the decision. Left for
  a later step of check's output (a candidate for 87i's suite split).
- F-7b-05 held, the planning session's decision (above).

## What the next steps must know

- The order of the methods for one sequent is written once, in
  `decide::decide` and `ordinary::natively`; a change of it is made
  there and holds for `prove` and the batch alike. `Run::own` is
  `prove`; a batch's entry answers as `prove --deterministic` does.
- The fixup that gives the library's default a turn for the model
  search under a clock replaces the command's first-order branch in
  `natively` (`first_order_share`, `by_connection`, `Race`) and the
  race in `decide` (`races_the_embedding`, `model_first`); `psvsuuvz`
  shows one way, and its CSV baseline is
  `bench/defaults/command-first-order-before.csv`.
- `LimitArgs` is the group 87i1 flattens into each suite's subcommand;
  `SEARCH_*`, `PROOF_*` and `WORKSPACE_*` are the per-command words.

## The net

- The CLI's tests at each commit (`cargo test -p deduxit-cli`, the
  lock, README's and the manual's examples): pass. Clippy with
  `--deny warnings`: clean.
- The gate at `nrmvqtro`, cores 4 to 6: passed (clippy, the tests, both
  `cargo hack` runs, the ratchet within its ceilings; misses moved on
  `search-chain-128` and `check-qbf-20-2`, which the ratchet reports
  and does not fail on: this step changes no code those journeys run).
- `breaks 0.8.0`: semver-checks reports 2 major and 1 minor checks,
  the `reason: String` fields of the step errors and the lambda
  accessors (`Term::goal`, `Typing::ty`, `Variable::ty`). All are the
  base's (`ptypmmzu`): this step changes no file under `core/`.
- F-7b-05's sample (above); the full after run was not made (held).
- After the review's fixes: the CLI's tests pass; every command's help
  equals the base's but for the reorder the lock commit names; the
  reviewer's witnesses for items 1, 2 and 5 answer as the base does.
- `nix flake check` at the head, under the machine lock: all checks
  passed.

## The reviewer's findings

`fresh-reviewer` on Opus at `high`, once, over `ptypmmzu..nrmvqtro`. It
compared the old `proved` and the batch's `decide` path by path with the
new decision and found it kept, but for item 5. Six findings, all
confirmed, each fixed and checked against the base's release build:

1. **The linear session's invertible runs cleared an earlier Ctrl-C**
   (`tzunntvu`), which only the ordinary session's did. Fixed in that
   commit: `Driven::CLEARS_STOP`, true for the ordinary session alone.
   The reviewer's script (Ctrl-C after `goals`, then `invertible on`)
   prints the same as the base.
2. **`prove --system` followed `view.layout`** (`woovtwku`), so a
   settings file with `"layout": "fitch"` drew a Fitch proof where a
   tree was drawn. Fixed by `xnuqnmrt` (its own commit: folding it in
   conflicted with the later commits' lines): `made` takes the layout,
   `check` passes the settings', `prove --system` a tree. The
   changelog's PNG line no longer claims the old behaviour.
3. **`workspace apply -h` and `set -h` hid `--work-limit`** (`skkxpnlw`):
   `LimitArgs` hides it from `-h`. Fixed in that commit:
   `workspace_helps` shows it.
4. **Seven commands lost their own words for the three limits**
   (`skkxpnlw`: `seq translate`, `seq normalize`, `resolve`, `herbrand`,
   `interpolate`, `term inhabit`, `term enumerate`). Fixed in that
   commit: `TransformArgs` keeps them by `mut_args(transform_helps)`.
   Every command's `-h` and `--help` now equals the base's but for the
   reorder the lock commit names (`prove`, `interact`, `check`, `grade`,
   and `eliminate`, whose help is not in the lock).
5. **`prove` needed its large-stack thread for steps that answered
   without one** (`nrmvqtro`): with `--recursion-limit 4294967295` the
   model search's refutation became "cannot start a search thread",
   exit 2. Fixed in that commit: `Run::stacked` puts the native step and
   the search on the large stack, and `Answer::Native::on_stack` writes
   what they found there; the model search first and the translation
   stay on the calling thread. The witness answers "not valid", exit 1,
   on both builds.
6. **F-7b-09 missed two sites, and the changelog named `interpolate`**.
   Fixed by `zzpnoutr`: `prove --system`'s refused evidence names the one
   flag of the refusing bound where it listed three, and `load` names
   its bound's flag where it said `--occurrence-limit` for every bound
   (both in the changelog's line). `ordinary::natively`'s refusal, which
   offers `--method embedding` too, stays a mapping of its own (cli.md).
