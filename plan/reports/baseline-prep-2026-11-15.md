# Baseline preparation, 2026-11-15

Opus 5.5 at `high`, in the jj workspace `../deduxit-wt-step-54s-c1-161790`
(a reused worker), from the brief `start-baseline-prep.md`, cores 4 to 6.
No timed run.

## The stage

The baseline measured the linear suite alone. Stage 1 of
`bench/baseline.sh` now also runs ILTP v1.1.2's 274 propositional
problems under every method of ordinary logic, with the settings of the
runs that chose the defaults (54s-c2, `bench/defaults/sat-*.csv`):
`--copies none --timeout 2 --repeat 3 --repeat-under 1`, one thread,
`--append --resume` as every run of the script.

- `iltp`: intuitionistic and minimal logic; the embedding, LJT, the SAT
  solver and the default (`auto`). After `lltp-rarer`, in the stream of
  `cores[0]`.
- `iltp-classical`: classical logic; the embedding, the SAT solver, the
  default, the truth table and the tableau. At the end of the stream of
  `cores[3]`, after `intuitionistic`.

Two files rather than one pass over three logics, so that neither has
`refused` rows (LJT refuses classical logic, the table and the tableau
the other two). In the fourth baseline (2026-11-02) those two streams'
runs searched about 11 and 14 minutes against the 45 of the longest
(`lltp-intuitionistic` and `families`), and each ILTP pass is about a
quarter of an hour on one core (54s-c2's runs: 11 minutes on two
shards for four methods in three logics; the tableau's timeouts at most
274 × 2.5 s), so stage 1 should stay about fifty minutes. The
script's estimate (`estimate`, 4 h) is unchanged; its header names the
fourth baseline's 2 h 23 min and the ILTP passes.

The harness had no `table` or `tableau` method (`the harness does not run
table`). `bench/src/ordinary.rs`'s new `classical` times
`ordinary::decide_within` with the method named, within
`Limits::default()` (the command's bound of ten atoms on a table) and the
run's memory bound; the tableau's derivation and either method's
countermodel are checked inside the time. `nodes` is the table's rows or
the tableau's nodes, `work` the rows or the lines; a table past ten atoms
is `unknown` with the reason `too_many_atoms`. The harness now enables
the feature `interactive`, which `Method::Tableau` needs.

The script refuses to start, arm or detach when `bench/lltp` or
`bench/iltp` is missing, naming the `nix build` that makes it: the night
would otherwise write error rows for every problem of the library.

`bench/RESULTS.md` is the fourth baseline's generated copy and names no
stage; it is left as it is. `.claude/rules/bench.md` says the new
methods (the `--tptp` bullet) and the new passes (the `baseline.sh`
bullet).

## The smoke run

The release harness of `kkpyuwyq`, pinned to core 4, both passes as the
script runs them with `--only` on eight of ILTP's smallest problems
(SYJ101+1, SYJ102+1, SYJ103+1, SYJ104+1, SYJ105+1.002: theorems;
SYJ207+1.001, SYJ209+1.001, SYJ211+1.001: intuitionistic non-theorems,
classical theorems):

- `iltp`: 192 rows (8 problems, 2 logics, 4 methods, 3 runs each).
  Intuitionistic: every method 5 proved, 3 refuted, as ILTP's status
  says; minimal: 5 proved (no known verdict), 3 refuted (as known).
- `iltp-classical`: 120 rows (8 problems, 5 methods, 3 runs). Every
  method proves all 8; 5 have the known verdict "provable", 3 none.
- `deduxit-bench summary --strict` on both: exit 0, no `MISMATCH`.

And the two ways a classical row ends without a verdict, on three larger
problems (`--engines table,tableau`, 2 s): the table refuses
SYJ201+1.010, SYJ202+1.008 and SYJ206+1.020 (`unknown`,
`too_many_atoms`, under 0.02 ms); the tableau proves SYJ201+1.010 in
22 ms (2 607 nodes, 3 890 lines) and times out on the other two
(`unknown`, `timeout`).

## The checklist

`plan/notes/baseline-night.md`: the day before (the libraries, the
rooted devshell, a warm build), the hour before (sessions paused, the
timers of 2026-11-15 and who stops them, power, the system slices), the
start (`bench/baseline.sh --detach --fresh`, or `--arm`, and the cores
when the desktop stays in use), and the morning (finished or resumed,
`starts.txt`, the header, the tables against the known mismatches and
against the ILTP counts of `bench/defaults/`, timers back, the commit).

## Checks

- The gate, capped on cores 4 to 6: passed twice, before the
  reviewer's fix and at the final code (834 tests, the ratchet within
  its ceilings).
- `cargo clippy -p deduxit-bench --all-targets -- --deny warnings`:
  clean.
- `cargo run --release -p deduxit-bench -- run --all-families --timeout
  5` (cores 4 to 6): 195 rows (100 proved, 67 refuted, 28 unknown), no
  `MISMATCH` in `summary`.
- `nix flake check` under the machine lock, at the chain rebased onto
  `main@origin` `moprkuwt`: all checks passed.
- `bash -n bench/baseline.sh`: clean. The script itself was not run: a
  baseline is the night's.

## The reviewer

`fresh-reviewer` (Fable 5.2, medium) read the chain, the brief and the
code, and ran single problems on core 6. Three findings, all answered:

1. **`--resume` reran every table and tableau row.** `classical` left
   `bias`, `forward_copies`, `pool_after` and `memo_limit` empty, and
   `run` keys a row by them (`forward_copies` 30), so no row matched:
   run twice into one file, the classical pass went from 85 to 119
   lines, the 34 new ones its table and tableau rows. A baseline resumed
   after the slot's stop would have run 548 problems again and written
   duplicate rows. Fixed in `kkpyuwyq`: the columns are written as
   `native` writes them. The same witness run twice now leaves 85 lines,
   and the second run's log is empty.
2. **The checklist said the script refuses battery.** It does only by
   hand: the unit runs it with `--force`, so `--detach` starts on
   battery and `--arm` starts at its latest start whatever the power,
   recording `BATTERY`. The checklist now says to check mains by hand.
3. **The checklist gave the embedding 188 in both logics.** 188
   intuitionistic, 192 minimal (`bench/defaults/sat-default.csv`);
   corrected.

## What the next sessions must know

- **The tableau stops up to half a second late.** At 2 s it stopped at
  2 518 ms on SYJ202+1.008 and 2 491 ms on SYJ206+1.020: it asks the
  stop once per step, and a step on a large goal is slow
  (`Interactive::apply` and `check_inference` on goals of many
  formulas). The row is still `timeout`, within the harness's grace.
  Worth a look if the tableau is to answer a web page's limit.
