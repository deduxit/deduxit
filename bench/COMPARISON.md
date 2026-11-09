# The four baselines compared

Four baselines, each taken by `bench/baseline.sh` on the same machine
under the same frequency settings (platform profile `performance`,
governor `powersave`, energy preference `balance_performance`, turbo
on):

- `bench/results/2026-09-30/`, the engines before the focused engine's
  performance pass;
- `bench/results/2026-10-02/`, after it;
- `bench/results/2026-10-17/`, the engines of the first release, 0.1.0
  (commit `ed74387d9bf3`), with the pass under the command's default
  that is new in it (`lltp-default`);
- `bench/results/2026-11-02/`, the engines of the second release, 0.2.0
  (commit `a917eaf4772d`).

The third started detached at 19:32 and took 2 h 58 min; the fourth
started by day at 14:34 and took 2 h 23 min. The compiler and the
dependencies moved with `flake.lock` and `Cargo.lock` between the third
and the fourth. The tables after "At a glance" are printed by

```sh
deduxit-bench summary --before bench/results/2026-09-30 --before bench/results/2026-10-02 \
  --before bench/results/2026-10-17 bench/results/2026-11-02/*.csv
deduxit-bench summary --against bench/results/2026-11-02/lltp-intuitionistic.csv \
  bench/results/2026-11-02/lltp-rarer.csv bench/results/2026-11-02/lltp-forward.csv
```

the first setting every file of the four side by side and then
comparing every file with the file of the same name in the third
baseline, the second the two passes under one atom bias with the
default pass, problem by problem. A verdict found after the time limit
counts as late and is kept apart.

## Where the fourth baseline ran differently

The fourth baseline was taken by day, beside other work, and **not on
the third's cores**:

- **The sequential streams** went on CPUs 4, 5, 2 and 3 where the
  third's went on 1, 0, 2 and 3. The first two streams therefore ran on
  efficiency cores (4.0 GHz, an L2 shared by four) instead of
  performance cores (5.1 GHz, an L2 of their own): `lltp-rarer` and
  `long-1` on CPU 4, `lltp-forward` and `long-2` on CPU 5, and in the
  last stage `lltp-intuitionistic-generous` and
  `lltp-classical-generous`. Their times are about 1.3 to 1.6 times the
  third's for the cores: the median ratio of the problems both decide is
  1.29 to 1.58 in every collection of `lltp-rarer` and `lltp-forward`
  but KLE-IMP-CONJ's non-theorems, where the same collections of
  `lltp-intuitionistic`, on CPU 2 in both, stay at 0.89 to 1.08; the
  non-theorems are 1.56 in `lltp-intuitionistic` too, and 1.89 and 1.90
  in the two passes. The long runs are 1.27 to 1.40 (`long-1`) and 1.12
  and 1.19 (`long-2`'s `wide` families; its one QBF instance, 2.60, has
  the slowdown of refuted QBFs below as well). The other two streams (`lltp-intuitionistic`,
  `families`, `lltp-copies-10` on CPU 2; `lltp-classical`, `engines`,
  `period-*`, `intuitionistic`, `lltp-recursion` on CPU 3) ran on the
  third's cores.
- **Nothing was shielded.** The run asked to be confined to CPUs 2 to 9
  and the user's other slices to the rest, but a user unit's or slice's
  `AllowedCPUs` needs the cpuset controller, which is not delegated to
  the user's systemd on this machine, so neither took effect (the
  script now confines a run by its affinity and says what it had). The
  streams were pinned by `taskset`, and other processes could share
  their CPUs: 46 sequential problems waited for a CPU for over 1 % of
  their time (`†` in `RESULTS.md`), where the third counted 75.
- **The parallel stage is not comparable.** `parallel`, `parallel-net`,
  `lltp-all-cores` and `lltp-default` ran with "every core" as 16
  threads over the whole machine (the rows' `jobs`), while two other
  projects' sessions worked on it: their times, speedups and the
  problems they decide within the limit are measured beside foreign
  load, and every parallel table below says so. A rerun of that stage on
  a quiet machine would replace these four files and the sentences on
  them. One thing in it holds whatever the load: on `mix`, one thread
  decides 8 to 11 pairs in 0.27 to 0.30 s by the inverse method's turn,
  which the dispatch gives only to a search of one thread, while 2 to 16
  threads race the focused engine alone (0.8 to 1.4 s at 8 pairs, past
  the 120 s limit at 11).

## At a glance

**No decided verdict differs** between the third and fourth baselines
on the same problem in the same configuration, and every proof of the
fourth was checked (`ok`). **Five problems that the third baseline
decided the fourth does not**: four of the pass under the command's
default (`lltp-default`, the parallel stage: `SYJ205+1.008` proved in
1.68 s before, `SYJ212+1.012` to `.014` of the cbn translation refuted
in 0.16 to 1.07 s before, each now at the 2 s limit, `SYJ212+1.012`
with about 6.5 of its 16 threads' worth of CPU time in its 2 s), and
`SYJ205+1.009` in `lltp-forward`, proved in 3.46 s on CPU 0 before and
past 5 s on CPU 5 now, as the slower core accounts for.

**What 0.2.0's engines add**, problem by problem against the third
(each derived from the rows' `engine`, `verdict` and `reason`):

- **The inverse method's turns** (the dispatch gives it a turn beside
  the two-sided engine on an intuitionistic goal with an exponential,
  and beside the focused engine under Mix): it decides 104 problems of
  `lltp-default` that ended at the time limit before (45 proved, 59
  refuted), 107 of `lltp-forward`'s (100 that ended at the time limit
  and 7 at the copy bound), 94 of `lltp-copies-10`'s (76 and 18), and
  the Mix family: `mix/10` took 62.5 s in the
  third and 259 ms now, `mix/11`, past 1 200 s before, 270 ms.
- **The refutation by the classical reading** after a search that ended
  at its copy bound: 53 problems of `lltp-intuitionistic` and 53 of
  `lltp-classical` that ended at the copy bound before are refuted (and
  54 of `lltp-rarer`, 27 of `lltp-copies-10`, 3 of `lltp-forward`), and
  `growing`, at the copy bound or the recursion limit at every size
  before, is refuted at every size (1 024 in 113 ms). Each such
  refutation is an assignment that falsifies the sequent's classical
  reading, so it holds whatever the LLTP header says: the one new
  verdict against a header is `KLE083+1` of `KLE-01` (refuted), whose
  other translations the third already refuted.
- **The net engine's symmetry breaking and balance check**: forced on
  `engines.csv`, it decides every instance of `3-partition-mll-no` and
  `partition-table` (2 of 4 and 9 of 13 before), two more of
  `partition-yes` and one more of `partition-no`, the ones both decide in
  1.4e-4 to 1.4e-2 of the time.
- **The focused engine on wide MLL**: `wide-m1` and `wide-m2`, which the
  command now gives to the focused engine instead of the net engine,
  take 0.60 and 0.61 of the time in the median (2 048 literals: 523 →
  203 ms and 556 → 186 ms).

**What got slower**: the refuted QBF instances of the smaller sizes. On
CPU 2 in both baselines, `qbf/16#1` took 1.1 ms and now 8 ms, the
refuted `qbf/20` instances 5 to 8 ms and now about 80 ms, with the same
nodes, splits and units of work, so the cost lies after the search:
since 0.1.0 a search that refutes by exhausting its space looks for a
classical assignment for the disproof within the classical work bound
(`core/src/linear/search/mod.rs`), which on these instances finds none. That is
read from the code and the rows, not traced. The larger instances got
faster (`qbf/48#1` 207 → 166 s); in the parallel stage the same fixed
cost makes the median ratio of the decided QBF instances 3.2 to 3.7 on 2
to 16 threads, those decided being the small ones.

Families, one thread, 300 s (stage 1, CPU 2 in both; the long runs
1 200 s, CPUs 4 and 5 now):

| family | third baseline | fourth baseline |
|---|---|---|
| 3-partition-no (bins of b) | every size in under 1 ms (b = 5: 346 µs) | the same (b = 5: 320 µs) |
| partition-yes (n items) | n = 24: 74.4 s, 28 > 1 200 s | n = 24: 71.6 s, 28 > 1 200 s |
| partition-no | n = 12: 3.8 s, 14 and 15 > 300 s | n = 12: 3.2 s, 14 and 15 > 300 s |
| qbf (n variables, four instances) | n = 48: three of four, 162–222 s (#0 at the memory bound after 85 s); `qbf/44#3` at the memory bound | n = 48: three of four, 142–185 s (#0 at the memory bound); `qbf/44#3` at the memory bound |
| wide-m1 (k pairs) | k = 2 048: 523 ms | k = 2 048: 203 ms |
| wide-m3 (k literals) | k = 2 048: 173 ms | k = 2 048: 163 ms |
| wide-m4 | k = 2 048: 167 ms | k = 2 048: 154 ms |
| mix (k pairs) | k = 10: 62.5 s (11 > 1 200 s) | k = 11: 270 ms |
| counter (n tokens) | n = 64: 55 µs | n = 64: 53 µs |
| counter-over | refuted at every size, 64 in 45 µs | the same, 64 in 45 µs |
| chain (k clauses) | k = 256: 275 ms | k = 256: 276 ms |
| additive (depth d) | d = 16: 182 ms | d = 16: 192 ms |
| growing (copy bound n) | the copy bound or the recursion limit at every size | refuted at every size, 1 024 in 113 ms |

The families new in 0.2.0 (`chain-fo`, `contraction`, `duplicate`,
`euler`, `lambek-fo`, `pigeonhole`, `reach-fo`, `witness-fo`) have no
column before; their growing sets are measured apart
(`bench/growing/2026-11-02/`, the manual's "Families that grow").

LLTP, one thread, 5 s, problems solved within the limit (the counts of
"Decided by group" below, summed per file):

| file | first | second | third | fourth |
|---|--:|--:|--:|--:|
| `lltp-intuitionistic` (4 495 problems, copy bound 3) | 737 | 2 047 | 3 610 | 3 663 |
| `lltp-classical` (4 512) | 742 | 2 006 | 3 621 | 3 674 |
| `lltp-rarer` (the backward search alone; CPU 4 in the fourth) | – | 969 | 3 608 | 3 662 |
| `lltp-forward` (the forward search alone, 30 copies; CPU 5 in the fourth) | – | 2 391 | 4 026 | 4 135 |
| `lltp-copies-10` (1 003 that ended at the copy bound) | 411 | 458 | 592 | 713 |
| `lltp-recursion` (995 that ended at the recursion limit) | 56 | 280 | 895 | 895 |
| `lltp-all-cores` (1 102, every core; not comparable in the fourth) | 29 | 623 | 993 | 993 |
| `lltp-default` (4 495, the command's default, 2 s; not comparable in the fourth) | – | – | 4 024 | 4 124 |

The Petri nets are unchanged (3 071 of 3 137 in each one-thread pass,
3 069 under the default); the gains are all outside them, in the SYJ and
KLE collections, by the inverse method's turns and the classical
reading as above.

The default against its two components (intuitionistic library, one
thread, 5 s; the default on CPU 2, the components on CPUs 4 and 5 in
the fourth, so its time over theirs mostly measures the cores):

| | default | `--bias rarer` | `--bias factors --copies 30` |
|---|--:|--:|--:|
| Petri nets decided | 3 071 | 3 071 | 3 071 |
| other collections decided | 592 | 591 | 1 064 |
| of those, the default does not decide | – | 0 | 472 (at the default's copy bound of 3) |
| decided by the default and not by it | – | 1 | 0 |
| default's time over it, median of both decided, nets | – | 0.75× (third: 1.01×) | 0.76× (third: 1.04×) |

**The pass under the command's default** (`lltp-default`: no copy
bound, one thread for 0.1 s and then a pool of the other fifteen beside
it, 2 s; the parallel stage, beside foreign load) decides 4 124 of the
4 495 problems, 3 069 nets and 1 055 others, where the third decided
4 024. It decides every problem that the copy-bound-3 pass decides
within 2 s but one, `KLE083+1` of `KLE-01`, which it refutes late, by
the classical reading after its time limit.

**Every core against one thread** (`lltp-all-cores`, the parallel
stage, not comparable): 993 of the 1 102 decided within 5 s, as in the
third, the median time of the nets both decide 1.01 times the third's.

## Every baseline

### Decided by group

The rows of `parallel`, `parallel-net`, `lltp-all-cores` and `lltp-default` in the fourth baseline are its parallel stage, 16 threads beside foreign load: not comparable with the earlier baselines' (above, "Where the fourth baseline ran differently").

Problems decided (late: after the time limit) of the problems run, per group of the last.

| file | family | configuration | 2026-09-30 | 2026-10-02 | 2026-10-17 | 2026-11-02 |
|---|---|---|--:|--:|--:|--:|
| engines | 3-partition-mll-no | classical focus j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| engines | 3-partition-mll-no | classical net j1 | 2 of 4 | 2 of 4 | 2 of 4 | 4 of 4 |
| engines | 3-partition-mll-yes | classical focus j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| engines | 3-partition-mll-yes | classical net j1 | 3 of 4 | 3 of 4 | 4 of 4 | 4 of 4 |
| engines | additive | classical focus j1 | 3 of 4 | 3 of 4 | 4 of 4 | 4 of 4 |
| engines | cancellation | classical focus j1 | 1 of 1 | 1 of 1 | 1 of 1 | 1 of 1 |
| engines | partition-no | classical focus j1 | 2 of 2 | 4 of 4 | 4 of 4 | 4 of 4 |
| engines | partition-no | classical net j1 | 1 of 2 | 1 of 4 | 1 of 4 | 2 of 4 |
| engines | partition-table | classical focus j1 | 12 of 13 | 13 of 13 | 13 of 13 | 13 of 13 |
| engines | partition-table | classical net j1 | 9 of 13 | 9 of 13 | 9 of 13 | 13 of 13 |
| engines | partition-yes | classical focus j1 | 3 of 3 | 5 of 5 | 5 of 5 | 5 of 5 |
| engines | partition-yes | classical net j1 | 1 of 3 | 1 of 5 | 1 of 5 | 3 of 5 |
| engines | wide-m1 | classical focus j1 | 4 of 6 | 5 of 6 | 6 of 6 | 6 of 6 |
| engines | wide-m1 | classical net j1 | 6 of 6 | 6 of 6 | 6 of 6 | 6 of 6 |
| engines | wide-m2 | classical focus j1 | 4 of 6 | 5 of 6 | 6 of 6 | 6 of 6 |
| engines | wide-m2 | classical net j1 | 6 of 6 | 6 of 6 | 6 of 6 | 6 of 6 |
| engines | wide-m3 | classical focus j1 | 3 of 3 | 4 of 5 | 5 of 5 | 5 of 5 |
| engines | wide-m3 | classical net j1 | 3 of 3 | 5 of 5 | 5 of 5 | 5 of 5 |
| engines | wide-m4 | classical focus j1 | 3 of 3 | 4 of 5 | 5 of 5 | 5 of 5 |
| engines | wide-m4 | classical net j1 | 3 of 3 | 5 of 5 | 5 of 5 | 5 of 5 |
| families | 3-partition-mll-no | classical auto j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| families | 3-partition-mll-yes | classical auto j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| families | 3-partition-no | classical auto j1 | 1 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| families | 3-partition-yes | classical auto j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| families | additive | classical auto j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| families | chain | classical auto j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| families | chain-fo | classical auto j1 |  |  |  | 3 of 8 |
| families | contraction | intuitionistic auto j1 |  |  |  | 4 of 6 |
| families | counter | classical auto j1 | 3 of 4 | 6 of 6 | 6 of 6 | 6 of 6 |
| families | counter-over | classical auto j1 | 0 of 4 | 4 of 6 | 6 of 6 | 6 of 6 |
| families | duplicate | classical auto j1 |  |  |  | 3 of 3 |
| families | duplicate | intuitionistic auto j1 |  |  |  | 3 of 3 |
| families | euler | intuitionistic auto j1 |  |  |  | 6 of 6 |
| families | growing | classical auto j1 | 0 of 4 | 0 of 4 | 0 of 4 | 4 of 4 |
| families | lambek-fo | intuitionistic auto j1 |  |  |  | 18 of 20 |
| families | mix | mix auto j1 | 5 of 6 | 5 of 6 | 5 of 6 | 6 of 6 |
| families | partition-no | classical auto j1 | 2 of 3 | 5 of 7 | 5 of 7 | 5 of 7 |
| families | partition-yes | classical auto j1 | 3 of 4 | 8 of 9 | 8 of 9 | 8 of 9 |
| families | pigeonhole | intuitionistic-affine auto j1 |  |  |  | 6 of 6 |
| families | qbf | classical auto j1 | 15 of 20 | 34 of 36 | 34 of 36 | 34 of 36 |
| families | reach-fo | intuitionistic auto j1 |  |  |  | 2 of 4 |
| families | wide-m1 | classical auto j1 | 6 of 6 | 6 of 6 | 6 of 6 | 6 of 6 |
| families | wide-m2 | classical auto j1 | 6 of 6 | 6 of 6 | 6 of 6 | 6 of 6 |
| families | wide-m3 | classical auto j1 | 3 of 4 | 6 of 7 | 7 of 7 | 7 of 7 |
| families | wide-m4 | classical auto j1 | 4 of 5 | 7 of 8 | 8 of 8 | 8 of 8 |
| families | witness-fo | intuitionistic auto j1 |  |  |  | 15 of 16 |
| intuitionistic | 3-partition-no | intuitionistic auto j1 | 1 of 1 | 1 of 1 | 1 of 1 | 1 of 1 |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| intuitionistic | chain | intuitionistic auto j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| intuitionistic | counter | intuitionistic auto j1 | 3 of 4 | 6 of 6 | 6 of 6 | 6 of 6 |
| intuitionistic | counter-over | intuitionistic auto j1 | 0 of 4 | 4 of 6 | 6 of 6 | 6 of 6 |
| lltp-all-cores | ILL/ILLTP-SYJ-01 | intuitionistic auto j16 | 0 of 31 | 0 of 31 | 0 of 31 | 0 of 31 |
| lltp-all-cores | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 | 0 of 31 | 0 of 31 | 0 of 31 | 0 of 31 |
| lltp-all-cores | ILL/ILLTP-SYJ-cbv | intuitionistic auto j16 | 1 of 31 | 2 of 31 | 2 of 31 | 2 of 31 |
| lltp-all-cores | ILL/petri-nets/MCC | intuitionistic auto j16 | 31 (3 late) of 1009 | 623 (2 late) of 1009 | 991 of 1009 | 991 of 1009 |
| lltp-all-cores-generous | ILL/ILLTP-SYJ-01 | intuitionistic auto j16 | 0 of 5 | 0 of 5 | 0 of 5 | 0 of 5 |
| lltp-all-cores-generous | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 | 0 of 4 | 0 of 4 | 0 of 4 | 0 of 4 |
| lltp-all-cores-generous | ILL/ILLTP-SYJ-cbv | intuitionistic auto j16 | 0 of 3 | 0 of 3 | 0 of 3 | 0 of 3 |
| lltp-classical | CLL/Non-theorems | classical auto j1 | 3 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| lltp-classical | CLL/misc | classical auto j1 | 8 of 14 | 8 of 14 | 8 of 14 | 8 of 14 |
| lltp-classical | ILL/ILLTP-LCL-01 | classical auto j1 | 0 of 2 | 0 of 2 | 0 of 2 | 1 of 2 |
| lltp-classical | ILL/ILLTP-LCL-cbn | classical auto j1 | 1 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| lltp-classical | ILL/ILLTP-LCL-cbv | classical auto j1 | 0 of 2 | 0 of 2 | 0 of 2 | 1 of 2 |
| lltp-classical | ILL/ILLTP-SYJ-01 | classical auto j1 | 11 of 252 | 11 of 252 | 11 of 252 | 34 of 252 |
| lltp-classical | ILL/ILLTP-SYJ-cbn | classical auto j1 | 34 of 252 | 35 of 252 | 35 of 252 | 38 of 252 |
| lltp-classical | ILL/ILLTP-SYJ-cbv | classical auto j1 | 39 of 252 | 39 of 252 | 40 of 252 | 43 of 252 |
| lltp-classical | ILL/ILLTP-SYN-01 | classical auto j1 | 11 of 19 | 11 of 19 | 13 of 19 | 14 of 19 |
| lltp-classical | ILL/ILLTP-SYN-cbn | classical auto j1 | 16 of 19 | 16 of 19 | 18 of 19 | 18 of 19 |
| lltp-classical | ILL/ILLTP-SYN-cbv | classical auto j1 | 16 of 19 | 16 of 19 | 18 of 19 | 18 of 19 |
| lltp-classical | ILL/KLE-01 | classical auto j1 | 50 of 88 | 50 of 88 | 51 of 88 | 58 of 88 |
| lltp-classical | ILL/KLE-IMP-CONJ | classical auto j1 | 162 of 222 | 162 of 222 | 162 of 222 | 162 of 222 |
| lltp-classical | ILL/KLE-IMP-CONJ/ALT | classical auto j1 | 27 of 27 | 27 of 27 | 27 of 27 | 27 of 27 |
| lltp-classical | ILL/KLE-IMP-CONJ/NON-THEOREMS | classical auto j1 | 22 of 22 | 22 of 22 | 22 of 22 | 22 of 22 |
| lltp-classical | ILL/KLE-cbn | classical auto j1 | 68 of 88 | 68 of 88 | 69 of 88 | 76 of 88 |
| lltp-classical | ILL/KLE-cbv | classical auto j1 | 67 of 88 | 67 of 88 | 68 of 88 | 75 of 88 |
| lltp-classical | ILL/Non-theorems | classical auto j1 | 0 of 1 | 0 of 1 | 0 of 1 | 0 of 1 |
| lltp-classical | ILL/misc | classical auto j1 | 3 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| lltp-classical | ILL/petri-nets/MCC | classical auto j1 | 204 of 3137 | 1468 (2 late) of 3137 | 3071 of 3137 | 3071 of 3137 |
| lltp-classical-generous | ILL/ILLTP-SYJ-01 | classical auto j1 | 0 of 4 | 0 of 4 | 0 of 4 | 0 of 4 |
| lltp-classical-generous | ILL/ILLTP-SYJ-cbn | classical auto j1 | 0 of 2 | 0 of 2 | 0 of 2 | 0 of 2 |
| lltp-classical-generous | ILL/ILLTP-SYJ-cbv | classical auto j1 | 0 of 3 | 0 of 3 | 0 of 3 | 0 of 3 |
| lltp-classical-generous | ILL/petri-nets/MCC | classical auto j1 | 0 of 25 | 15 of 25 | 25 of 25 | 25 of 25 |
| lltp-copies-10 | ILL/ILLTP-LCL-01 | intuitionistic auto j1 | 1 of 2 | 1 of 2 | 2 of 2 | 2 of 2 |
| lltp-copies-10 | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| lltp-copies-10 | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 | 1 of 2 | 1 of 2 | 2 of 2 | 2 of 2 |
| lltp-copies-10 | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 37 of 209 | 37 of 209 | 51 of 209 | 94 of 209 |
| lltp-copies-10 | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 60 of 209 | 75 of 209 | 88 of 209 | 124 of 209 |
| lltp-copies-10 | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 83 of 209 | 83 of 209 | 93 of 209 | 123 of 209 |
| lltp-copies-10 | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | 5 of 8 | 5 of 8 | 7 of 8 | 7 of 8 |
| lltp-copies-10 | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 | 5 of 8 | 5 of 8 | 8 of 8 | 8 of 8 |
| lltp-copies-10 | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 | 5 of 8 | 5 of 8 | 8 of 8 | 8 of 8 |
| lltp-copies-10 | ILL/KLE-01 | intuitionistic auto j1 | 29 of 38 | 29 of 38 | 31 of 38 | 37 of 38 |
| lltp-copies-10 | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | 60 of 60 | 60 of 60 | 60 of 60 | 60 of 60 |
| lltp-copies-10 | ILL/KLE-cbn | intuitionistic auto j1 | 32 of 38 | 34 of 38 | 36 of 38 | 38 of 38 |
| lltp-copies-10 | ILL/KLE-cbv | intuitionistic auto j1 | 32 of 38 | 32 of 38 | 33 of 38 | 37 of 38 |
| lltp-copies-10 | ILL/Non-theorems | intuitionistic auto j1 | 0 of 1 | 0 of 1 | 0 of 1 | 0 of 1 |
| lltp-copies-10 | ILL/petri-nets/MCC | intuitionistic auto j1 | 59 of 171 | 89 of 171 | 171 of 171 | 171 of 171 |
| lltp-default | ILL/ILLTP-LCL-01 | intuitionistic auto j16 pool after 0.1s |  |  | 2 of 2 | 2 of 2 |
| lltp-default | ILL/ILLTP-LCL-cbn | intuitionistic auto j16 pool after 0.1s |  |  | 2 of 2 | 2 of 2 |
| lltp-default | ILL/ILLTP-LCL-cbv | intuitionistic auto j16 pool after 0.1s |  |  | 2 of 2 | 2 of 2 |
| lltp-default | ILL/ILLTP-SYJ-01 | intuitionistic auto j16 pool after 0.1s |  |  | 101 of 252 | 137 of 252 |
| lltp-default | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 pool after 0.1s |  |  | 137 of 252 | 164 of 252 |
| lltp-default | ILL/ILLTP-SYJ-cbv | intuitionistic auto j16 pool after 0.1s |  |  | 117 of 252 | 154 of 252 |
| lltp-default | ILL/ILLTP-SYN-01 | intuitionistic auto j16 pool after 0.1s |  |  | 19 of 19 | 19 of 19 |
| lltp-default | ILL/ILLTP-SYN-cbn | intuitionistic auto j16 pool after 0.1s |  |  | 19 of 19 | 19 of 19 |
| lltp-default | ILL/ILLTP-SYN-cbv | intuitionistic auto j16 pool after 0.1s |  |  | 19 of 19 | 19 of 19 |
| lltp-default | ILL/KLE-01 | intuitionistic auto j16 pool after 0.1s |  |  | 87 of 88 | 88 (1 late) of 88 |
| lltp-default | ILL/KLE-IMP-CONJ | intuitionistic auto j16 pool after 0.1s |  |  | 222 of 222 | 222 of 222 |
| lltp-default | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j16 pool after 0.1s |  |  | 27 of 27 | 27 of 27 |
| lltp-default | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j16 pool after 0.1s |  |  | 22 of 22 | 22 of 22 |
| lltp-default | ILL/KLE-cbn | intuitionistic auto j16 pool after 0.1s |  |  | 88 of 88 | 88 of 88 |
| lltp-default | ILL/KLE-cbv | intuitionistic auto j16 pool after 0.1s |  |  | 88 of 88 | 88 of 88 |
| lltp-default | ILL/Non-theorems | intuitionistic auto j16 pool after 0.1s |  |  | 0 of 1 | 0 of 1 |
| lltp-default | ILL/misc | intuitionistic auto j16 pool after 0.1s |  |  | 3 of 3 | 3 of 3 |
| lltp-default | ILL/petri-nets/MCC | intuitionistic auto j16 pool after 0.1s |  |  | 3069 of 3137 | 3069 of 3137 |
| lltp-forward | ILL/ILLTP-LCL-01 | intuitionistic auto j1 bias factors |  | 1 of 2 | 2 of 2 | 2 of 2 |
| lltp-forward | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 bias factors |  | 2 of 2 | 2 of 2 | 2 of 2 |
| lltp-forward | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 bias factors |  | 1 of 2 | 2 of 2 | 2 of 2 |
| lltp-forward | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 bias factors |  | 53 of 252 | 99 of 252 | 138 of 252 |
| lltp-forward | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 bias factors |  | 98 of 252 | 142 of 252 | 170 of 252 |
| lltp-forward | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 bias factors |  | 94 of 252 | 118 of 252 | 157 of 252 |
| lltp-forward | ILL/ILLTP-SYN-01 | intuitionistic auto j1 bias factors |  | 16 of 19 | 19 of 19 | 19 of 19 |
| lltp-forward | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 bias factors |  | 16 of 19 | 19 of 19 | 19 of 19 |
| lltp-forward | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 bias factors |  | 16 of 19 | 19 of 19 | 19 of 19 |
| lltp-forward | ILL/KLE-01 | intuitionistic auto j1 bias factors |  | 80 of 88 | 86 of 88 | 87 of 88 |
| lltp-forward | ILL/KLE-IMP-CONJ | intuitionistic auto j1 bias factors |  | 222 of 222 | 222 of 222 | 222 of 222 |
| lltp-forward | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j1 bias factors |  | 27 of 27 | 27 of 27 | 27 of 27 |
| lltp-forward | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j1 bias factors |  | 22 of 22 | 22 of 22 | 22 of 22 |
| lltp-forward | ILL/KLE-cbn | intuitionistic auto j1 bias factors |  | 84 of 88 | 87 of 88 | 88 of 88 |
| lltp-forward | ILL/KLE-cbv | intuitionistic auto j1 bias factors |  | 82 of 88 | 86 of 88 | 87 of 88 |
| lltp-forward | ILL/Non-theorems | intuitionistic auto j1 bias factors |  | 0 of 1 | 0 of 1 | 0 of 1 |
| lltp-forward | ILL/misc | intuitionistic auto j1 bias factors |  | 3 of 3 | 3 of 3 | 3 of 3 |
| lltp-forward | ILL/petri-nets/MCC | intuitionistic auto j1 bias factors |  | 1576 (2 late) of 3137 | 3071 of 3137 | 3071 of 3137 |
| lltp-intuitionistic | ILL/ILLTP-LCL-01 | intuitionistic auto j1 | 0 of 2 | 0 of 2 | 0 of 2 | 1 of 2 |
| lltp-intuitionistic | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 | 1 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| lltp-intuitionistic | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 | 0 of 2 | 0 of 2 | 0 of 2 | 1 of 2 |
| lltp-intuitionistic | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 11 of 252 | 11 of 252 | 11 of 252 | 34 of 252 |
| lltp-intuitionistic | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 34 of 252 | 35 of 252 | 35 of 252 | 38 of 252 |
| lltp-intuitionistic | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 39 of 252 | 39 of 252 | 40 of 252 | 43 of 252 |
| lltp-intuitionistic | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | 11 of 19 | 11 of 19 | 13 of 19 | 14 of 19 |
| lltp-intuitionistic | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 | 16 of 19 | 16 of 19 | 18 of 19 | 18 of 19 |
| lltp-intuitionistic | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 | 16 of 19 | 16 of 19 | 18 of 19 | 18 of 19 |
| lltp-intuitionistic | ILL/KLE-01 | intuitionistic auto j1 | 50 of 88 | 50 of 88 | 51 of 88 | 58 of 88 |
| lltp-intuitionistic | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | 162 of 222 | 162 of 222 | 162 of 222 | 162 of 222 |
| lltp-intuitionistic | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j1 | 27 of 27 | 27 of 27 | 27 of 27 | 27 of 27 |
| lltp-intuitionistic | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j1 | 22 of 22 | 22 of 22 | 22 of 22 | 22 of 22 |
| lltp-intuitionistic | ILL/KLE-cbn | intuitionistic auto j1 | 68 of 88 | 68 of 88 | 69 of 88 | 76 of 88 |
| lltp-intuitionistic | ILL/KLE-cbv | intuitionistic auto j1 | 67 of 88 | 67 of 88 | 68 of 88 | 75 of 88 |
| lltp-intuitionistic | ILL/Non-theorems | intuitionistic auto j1 | 0 of 1 | 0 of 1 | 0 of 1 | 0 of 1 |
| lltp-intuitionistic | ILL/misc | intuitionistic auto j1 | 3 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| lltp-intuitionistic | ILL/petri-nets/MCC | intuitionistic auto j1 | 210 of 3137 | 1520 (2 late) of 3137 | 3071 of 3137 | 3071 of 3137 |
| lltp-intuitionistic-generous | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 0 of 4 | 0 of 4 | 0 of 4 | 0 of 4 |
| lltp-intuitionistic-generous | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 0 of 2 | 0 of 2 | 0 of 2 | 0 of 2 |
| lltp-intuitionistic-generous | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 0 of 3 | 0 of 3 | 0 of 3 | 0 of 3 |
| lltp-intuitionistic-generous | ILL/petri-nets/MCC | intuitionistic auto j1 | 0 of 19 | 11 of 19 | 19 of 19 | 19 of 19 |
| lltp-rarer | ILL/ILLTP-LCL-01 | intuitionistic auto j1 bias rarer |  | 0 of 2 | 0 of 2 | 1 of 2 |
| lltp-rarer | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 bias rarer |  | 1 of 2 | 1 of 2 | 2 of 2 |
| lltp-rarer | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 bias rarer |  | 0 of 2 | 0 of 2 | 1 of 2 |
| lltp-rarer | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 bias rarer |  | 11 of 252 | 11 of 252 | 34 of 252 |
| lltp-rarer | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 bias rarer |  | 34 of 252 | 34 of 252 | 37 of 252 |
| lltp-rarer | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 bias rarer |  | 39 of 252 | 40 of 252 | 43 of 252 |
| lltp-rarer | ILL/ILLTP-SYN-01 | intuitionistic auto j1 bias rarer |  | 11 of 19 | 13 of 19 | 14 of 19 |
| lltp-rarer | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 bias rarer |  | 16 of 19 | 18 of 19 | 18 of 19 |
| lltp-rarer | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 bias rarer |  | 16 of 19 | 18 of 19 | 18 of 19 |
| lltp-rarer | ILL/KLE-01 | intuitionistic auto j1 bias rarer |  | 50 of 88 | 51 of 88 | 58 of 88 |
| lltp-rarer | ILL/KLE-IMP-CONJ | intuitionistic auto j1 bias rarer |  | 162 of 222 | 162 of 222 | 162 of 222 |
| lltp-rarer | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j1 bias rarer |  | 27 of 27 | 27 of 27 | 27 of 27 |
| lltp-rarer | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j1 bias rarer |  | 22 of 22 | 22 of 22 | 22 of 22 |
| lltp-rarer | ILL/KLE-cbn | intuitionistic auto j1 bias rarer |  | 68 of 88 | 69 of 88 | 76 of 88 |
| lltp-rarer | ILL/KLE-cbv | intuitionistic auto j1 bias rarer |  | 67 of 88 | 68 of 88 | 75 of 88 |
| lltp-rarer | ILL/Non-theorems | intuitionistic auto j1 bias rarer |  | 0 of 1 | 0 of 1 | 0 of 1 |
| lltp-rarer | ILL/misc | intuitionistic auto j1 bias rarer |  | 3 of 3 | 3 of 3 | 3 of 3 |
| lltp-rarer | ILL/petri-nets/MCC | intuitionistic auto j1 bias rarer |  | 442 of 3137 | 3071 of 3137 | 3071 of 3137 |
| lltp-recursion | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 0 of 22 | 0 of 22 | 0 of 22 | 0 of 22 |
| lltp-recursion | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 0 of 22 | 0 of 22 | 0 of 22 | 0 of 22 |
| lltp-recursion | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 0 of 22 | 0 of 22 | 0 of 22 | 0 of 22 |
| lltp-recursion | ILL/petri-nets/MCC | intuitionistic auto j1 | 56 of 929 | 280 of 929 | 895 of 929 | 895 of 929 |
| lltp-recursion-generous | ILL/petri-nets/MCC | intuitionistic auto j1 | 2 (2 late) of 9 | 5 of 9 | 9 of 9 | 9 of 9 |
| long-1 | 3-partition-no | classical auto j1 | 1 of 1 | 1 of 1 | 1 of 1 | 1 of 1 |
| long-1 | counter | classical auto j1 | 0 of 1 | 2 of 2 | 2 of 2 | 2 of 2 |
| long-1 | partition-no | classical auto j1 | 1 of 1 | 1 of 2 | 1 of 2 | 1 of 2 |
| long-1 | partition-yes | classical auto j1 | 0 of 1 | 1 of 2 | 1 of 2 | 1 of 2 |
| long-2 | mix | mix auto j1 | 0 of 1 | 0 of 1 | 0 of 1 | 1 of 1 |
| long-2 | qbf | classical auto j1 | 0 of 1 | 1 of 2 | 1 of 2 | 1 of 2 |
| long-2 | wide-m3 | classical auto j1 | 1 of 1 | 1 of 1 | 1 of 1 | 1 of 1 |
| long-2 | wide-m4 | classical auto j1 | 1 of 1 | 1 of 1 | 1 of 1 | 1 of 1 |
| parallel | 3-partition-no | classical auto j1 | 1 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| parallel | 3-partition-no | classical auto j16 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| parallel | 3-partition-no | classical auto j2 | 1 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| parallel | 3-partition-no | classical auto j4 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| parallel | 3-partition-no | classical auto j8 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| parallel | 3-partition-yes | classical auto j1 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| parallel | 3-partition-yes | classical auto j16 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| parallel | 3-partition-yes | classical auto j2 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| parallel | 3-partition-yes | classical auto j4 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| parallel | 3-partition-yes | classical auto j8 | 4 of 4 | 4 of 4 | 4 of 4 | 4 of 4 |
| parallel | counter | classical auto j1 | 3 of 4 | 6 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter | classical auto j16 | 3 of 4 | 6 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter | classical auto j2 | 3 of 4 | 6 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter | classical auto j4 | 3 of 4 | 6 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter | classical auto j8 | 3 of 4 | 6 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter-over | classical auto j1 | 0 of 4 | 4 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter-over | classical auto j16 | 0 of 4 | 4 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter-over | classical auto j2 | 0 of 4 | 4 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter-over | classical auto j4 | 0 of 4 | 4 of 6 | 6 of 6 | 6 of 6 |
| parallel | counter-over | classical auto j8 | 0 of 4 | 4 of 6 | 6 of 6 | 6 of 6 |
| parallel | mix | mix auto j1 | 2 of 4 | 2 of 4 | 3 of 4 | 4 of 4 |
| parallel | mix | mix auto j16 | 2 of 4 | 2 of 4 | 2 of 4 | 4 (1 late) of 4 |
| parallel | mix | mix auto j2 | 2 of 4 | 2 of 4 | 3 of 4 | 4 (1 late) of 4 |
| parallel | mix | mix auto j4 | 2 of 4 | 2 of 4 | 3 of 4 | 4 (1 late) of 4 |
| parallel | mix | mix auto j8 | 2 of 4 | 2 of 4 | 3 of 4 | 4 (1 late) of 4 |
| parallel | partition-no | classical auto j1 | 1 of 2 | 3 of 4 | 3 of 4 | 3 of 4 |
| parallel | partition-no | classical auto j16 | 1 of 2 | 3 of 4 | 3 of 4 | 3 of 4 |
| parallel | partition-no | classical auto j2 | 1 of 2 | 3 of 4 | 3 of 4 | 3 of 4 |
| parallel | partition-no | classical auto j4 | 1 of 2 | 3 of 4 | 3 of 4 | 3 of 4 |
| parallel | partition-no | classical auto j8 | 1 of 2 | 3 of 4 | 3 of 4 | 3 of 4 |
| parallel | partition-yes | classical auto j1 | 2 of 3 | 5 of 5 | 5 of 5 | 5 of 5 |
| parallel | partition-yes | classical auto j16 | 2 of 3 | 4 of 5 | 5 of 5 | 5 of 5 |
| parallel | partition-yes | classical auto j2 | 2 of 3 | 4 of 5 | 5 of 5 | 5 of 5 |
| parallel | partition-yes | classical auto j4 | 2 of 3 | 4 of 5 | 5 of 5 | 5 of 5 |
| parallel | partition-yes | classical auto j8 | 2 of 3 | 4 of 5 | 5 of 5 | 5 of 5 |
| parallel | qbf | classical auto j1 | 7 of 12 | 19 of 20 | 19 of 20 | 19 of 20 |
| parallel | qbf | classical auto j16 | 7 of 12 | 19 of 20 | 14 of 20 | 14 of 20 |
| parallel | qbf | classical auto j2 | 7 of 12 | 19 of 20 | 14 of 20 | 14 of 20 |
| parallel | qbf | classical auto j4 | 7 of 12 | 19 of 20 | 14 of 20 | 14 of 20 |
| parallel | qbf | classical auto j8 | 7 of 12 | 19 of 20 | 14 of 20 | 14 of 20 |
| parallel | wide-m3 | classical auto j1 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m3 | classical auto j16 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m3 | classical auto j2 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m3 | classical auto j4 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m3 | classical auto j8 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m4 | classical auto j1 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m4 | classical auto j16 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m4 | classical auto j2 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m4 | classical auto j4 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel | wide-m4 | classical auto j8 | 2 of 3 | 3 of 3 | 3 of 3 | 3 of 3 |
| parallel-net | 3-partition-mll-no | classical net j1 | 2 of 3 | 2 of 3 | 2 of 3 | 3 of 3 |
| parallel-net | 3-partition-mll-no | classical net j16 | 2 of 3 | 2 of 3 | 2 of 3 | 3 of 3 |
| parallel-net | 3-partition-mll-no | classical net j2 | 2 of 3 | 2 of 3 | 2 of 3 | 3 of 3 |
| parallel-net | 3-partition-mll-no | classical net j4 | 2 of 3 | 2 of 3 | 2 of 3 | 3 of 3 |
| parallel-net | 3-partition-mll-no | classical net j8 | 2 of 3 | 2 of 3 | 2 of 3 | 3 of 3 |
| parallel-net | partition-table | classical net j1 | 9 of 13 | 9 of 13 | 9 of 13 | 13 of 13 |
| parallel-net | partition-table | classical net j16 | 10 of 13 | 10 of 13 | 10 of 13 | 13 of 13 |
| parallel-net | partition-table | classical net j2 | 9 of 13 | 9 of 13 | 9 of 13 | 13 of 13 |
| parallel-net | partition-table | classical net j4 | 10 of 13 | 10 of 13 | 10 of 13 | 13 of 13 |
| parallel-net | partition-table | classical net j8 | 10 of 13 | 10 of 13 | 10 of 13 | 13 of 13 |
| period-1 | 3-partition-mll-no | classical net j1 period 1 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-1 | partition-no | classical net j1 period 1 | 1 of 2 | 1 of 2 | 1 of 2 | 2 of 2 |
| period-1 | partition-yes | classical net j1 period 1 | 1 of 1 | 1 of 1 | 1 of 1 | 1 of 1 |
| period-1 | wide-m1 | classical net j1 period 1 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-1 | wide-m2 | classical net j1 period 1 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-16 | 3-partition-mll-no | classical net j1 period 16 | 1 of 2 | 1 of 2 | 1 of 2 | 2 of 2 |
| period-16 | partition-no | classical net j1 period 16 | 0 of 2 | 0 of 2 | 0 of 2 | 2 of 2 |
| period-16 | partition-yes | classical net j1 period 16 | 0 of 1 | 0 of 1 | 0 of 1 | 1 of 1 |
| period-16 | wide-m1 | classical net j1 period 16 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-16 | wide-m2 | classical net j1 period 16 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-2 | 3-partition-mll-no | classical net j1 period 2 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-2 | partition-no | classical net j1 period 2 | 1 of 2 | 1 of 2 | 1 of 2 | 2 of 2 |
| period-2 | partition-yes | classical net j1 period 2 | 1 of 1 | 1 of 1 | 1 of 1 | 1 of 1 |
| period-2 | wide-m1 | classical net j1 period 2 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-2 | wide-m2 | classical net j1 period 2 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-8 | 3-partition-mll-no | classical net j1 period 8 | 1 of 2 | 1 of 2 | 1 of 2 | 2 of 2 |
| period-8 | partition-no | classical net j1 period 8 | 0 of 2 | 0 of 2 | 0 of 2 | 2 of 2 |
| period-8 | partition-yes | classical net j1 period 8 | 0 of 1 | 0 of 1 | 0 of 1 | 1 of 1 |
| period-8 | wide-m1 | classical net j1 period 8 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |
| period-8 | wide-m2 | classical net j1 period 8 | 2 of 2 | 2 of 2 | 2 of 2 | 2 of 2 |

### Generated and listed problems

The rows of `parallel`, `parallel-net`, `lltp-all-cores` and `lltp-default` in the fourth baseline are its parallel stage, 16 threads beside foreign load: not comparable with the earlier baselines' (above, "Where the fourth baseline ran differently").

| file | family | configuration | problem | 2026-09-30 | 2026-10-02 | 2026-10-17 | 2026-11-02 |
|---|---|---|---|--:|--:|--:|--:|
| engines | 3-partition-mll-no | classical focus j1 | 3-partition-mll-no/4 | 15 µs ✗ | 20 µs ✗ | 39 µs ✗ | 37 µs ✗ |
| engines | 3-partition-mll-no | classical focus j1 | 3-partition-mll-no/5 | 18 µs ✗ | 20 µs ✗ | 41 µs ✗ | 39 µs ✗ |
| engines | 3-partition-mll-no | classical focus j1 | 3-partition-mll-no/6 | 17 µs ✗ | 22 µs ✗ | 41 µs ✗ | 42 µs ✗ |
| engines | 3-partition-mll-no | classical focus j1 | 3-partition-mll-no/8 | 16 µs ✗ | 20 µs ✗ | 46 µs ✗ | 44 µs ✗ |
| engines | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/4 | 1.80 s ✗ | 1.86 s ✗ | 1.83 s ✗ | 250 µs ✗ |
| engines | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/5 | 56.31 s ✗ | 57.68 s ✗ | 56.13 s ✗ | 689 µs ✗ |
| engines | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/6 | > 60 s | > 60 s | > 60 s | 1.2 ms ✗ |
| engines | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/8 | > 60 s | > 60 s | > 60 s | 3.0 ms ✗ |
| engines | 3-partition-mll-yes | classical focus j1 | 3-partition-mll-yes/12 | 37 µs ✓ | 42 µs ✓ | 57 µs ✓ | 55 µs ✓ |
| engines | 3-partition-mll-yes | classical focus j1 | 3-partition-mll-yes/4 | 31 µs ✓ | 40 µs ✓ | 64 µs ✓ | 50 µs ✓ |
| engines | 3-partition-mll-yes | classical focus j1 | 3-partition-mll-yes/6 | 31 µs ✓ | 42 µs ✓ | 55 µs ✓ | 49 µs ✓ |
| engines | 3-partition-mll-yes | classical focus j1 | 3-partition-mll-yes/8 | 33 µs ✓ | 45 µs ✓ | 55 µs ✓ | 52 µs ✓ |
| engines | 3-partition-mll-yes | classical net j1 | 3-partition-mll-yes/12 | > 60 s | > 60 s | 170 µs ✓ | 222 µs ✓ |
| engines | 3-partition-mll-yes | classical net j1 | 3-partition-mll-yes/4 | 802 µs ✓ | 806 µs ✓ | 62 µs ✓ | 76 µs ✓ |
| engines | 3-partition-mll-yes | classical net j1 | 3-partition-mll-yes/6 | 114.7 ms ✓ | 117.3 ms ✓ | 88 µs ✓ | 102 µs ✓ |
| engines | 3-partition-mll-yes | classical net j1 | 3-partition-mll-yes/8 | 5.55 s ✓ | 5.68 s ✓ | 107 µs ✓ | 132 µs ✓ |
| engines | additive | classical focus j1 | additive/12 | 354.7 ms ✓ | 320.2 ms ✓ | 38.1 ms ✓ | 31.6 ms ✓ |
| engines | additive | classical focus j1 | additive/14 | 6.19 s ✓ | 5.41 s ✓ | 391.5 ms ✓ | 328.5 ms ✓ |
| engines | additive | classical focus j1 | additive/16 | 12.21 s unknown | 9.95 s unknown | 5.79 s ✓ | 5.28 s ✓ |
| engines | additive | classical focus j1 | additive/8 | 1.6 ms ✓ | 1.5 ms ✓ | 873 µs ✓ | 740 µs ✓ |
| engines | cancellation | classical focus j1 | cancellation/3-partition-4 | 50.68 s ✓ | 319 µs ✓ | 300 µs ✓ | 278 µs ✓ |
| engines | partition-no | classical focus j1 | partition-no/12 |  | 3.91 s ✗ | 3.81 s ✗ | 3.47 s ✗ |
| engines | partition-no | classical focus j1 | partition-no/3 | 4.2 ms ✗ | 59 µs ✗ | 71 µs ✗ | 82 µs ✗ |
| engines | partition-no | classical focus j1 | partition-no/4 | 111.1 ms ✗ | 138 µs ✗ | 150 µs ✗ | 147 µs ✗ |
| engines | partition-no | classical focus j1 | partition-no/9 |  | 86.1 ms ✗ | 89.0 ms ✗ | 80.2 ms ✗ |
| engines | partition-no | classical net j1 | partition-no/12 |  | > 60 s | > 60 s | > 60 s |
| engines | partition-no | classical net j1 | partition-no/3 | 3.45 s ✗ | 3.57 s ✗ | 3.68 s ✗ | 1.5 ms ✗ |
| engines | partition-no | classical net j1 | partition-no/4 | > 60 s | > 60 s | > 60 s | 9.6 ms ✗ |
| engines | partition-no | classical net j1 | partition-no/9 |  | > 60 s | > 60 s | > 60 s |
| engines | partition-table | classical focus j1 | partition-table/1-1 | 38 µs ✓ | 33 µs ✓ | 51 µs ✓ | 50 µs ✓ |
| engines | partition-table | classical focus j1 | partition-table/1-1-1-1-1-7 | 58.37 s ✗ | 1.3 ms ✗ | 1.2 ms ✗ | 1.1 ms ✗ |
| engines | partition-table | classical focus j1 | partition-table/1-1-1-5 | 61.2 ms ✗ | 149 µs ✗ | 141 µs ✗ | 132 µs ✗ |
| engines | partition-table | classical focus j1 | partition-table/1-1-2-4 | 526 µs ✓ | 48 µs ✓ | 70 µs ✓ | 58 µs ✓ |
| engines | partition-table | classical focus j1 | partition-table/1-1-4 | 2.3 ms ✗ | 56 µs ✗ | 81 µs ✗ | 72 µs ✗ |
| engines | partition-table | classical focus j1 | partition-table/1-2-3-4-5-5 | 23.15 s ✓ | 80 µs ✓ | 97 µs ✓ | 86 µs ✓ |
| engines | partition-table | classical focus j1 | partition-table/1-2-5 | 5.8 ms ✗ | 59 µs ✗ | 83 µs ✗ | 74 µs ✗ |
| engines | partition-table | classical focus j1 | partition-table/1-3 | 131 µs ✗ | 29 µs ✗ | 50 µs ✗ | 50 µs ✗ |
| engines | partition-table | classical focus j1 | partition-table/2-1-1 | 165 µs ✓ | 45 µs ✓ | 67 µs ✓ | 60 µs ✓ |
| engines | partition-table | classical focus j1 | partition-table/2-2-1-1 | 2.2 ms ✓ | 62 µs ✓ | 82 µs ✓ | 74 µs ✓ |
| engines | partition-table | classical focus j1 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | 124 µs ✓ | 129 µs ✓ | 106 µs ✓ |
| engines | partition-table | classical focus j1 | partition-table/2-3-2-1 | 6.1 ms ✓ | 68 µs ✓ | 75 µs ✓ | 76 µs ✓ |
| engines | partition-table | classical focus j1 | partition-table/3-3-3-1 | 112.2 ms ✗ | 143 µs ✗ | 137 µs ✗ | 138 µs ✗ |
| engines | partition-table | classical net j1 | partition-table/1-1 | 36 µs ✓ | 34 µs ✓ | 51 µs ✓ | 53 µs ✓ |
| engines | partition-table | classical net j1 | partition-table/1-1-1-1-1-7 | > 60 s | > 60 s | > 60 s | 106.5 ms ✗ |
| engines | partition-table | classical net j1 | partition-table/1-1-1-5 | 7.50 s ✗ | 7.59 s ✗ | 7.43 s ✗ | 3.1 ms ✗ |
| engines | partition-table | classical net j1 | partition-table/1-1-2-4 | 1.03 s ✓ | 1.10 s ✓ | 1.04 s ✓ | 889 µs ✓ |
| engines | partition-table | classical net j1 | partition-table/1-1-4 | 39.9 ms ✗ | 40.7 ms ✗ | 40.2 ms ✗ | 559 µs ✗ |
| engines | partition-table | classical net j1 | partition-table/1-2-3-4-5-5 | > 60 s | > 60 s | > 60 s | 1.08 s ✓ |
| engines | partition-table | classical net j1 | partition-table/1-2-5 | 3.66 s ✗ | 3.75 s ✗ | 3.68 s ✗ | 1.3 ms ✗ |
| engines | partition-table | classical net j1 | partition-table/1-3 | 394 µs ✗ | 372 µs ✗ | 388 µs ✗ | 118 µs ✗ |
| engines | partition-table | classical net j1 | partition-table/2-1-1 | 657 µs ✓ | 644 µs ✓ | 626 µs ✓ | 169 µs ✓ |
| engines | partition-table | classical net j1 | partition-table/2-2-1-1 | 51.5 ms ✓ | 55.2 ms ✓ | 52.2 ms ✓ | 928 µs ✓ |
| engines | partition-table | classical net j1 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | > 60 s | > 60 s | 4.01 s ✓ |
| engines | partition-table | classical net j1 | partition-table/2-3-2-1 | 4.58 s ✓ | 4.67 s ✓ | 4.68 s ✓ | 2.5 ms ✓ |
| engines | partition-table | classical net j1 | partition-table/3-3-3-1 | > 60 s | > 60 s | > 60 s | 8.3 ms ✗ |
| engines | partition-yes | classical focus j1 | partition-yes/12 |  | 15.2 ms ✓ | 13.3 ms ✓ | 12.8 ms ✓ |
| engines | partition-yes | classical focus j1 | partition-yes/20 |  | 573.7 ms ✓ | 471.1 ms ✓ | 426.4 ms ✓ |
| engines | partition-yes | classical focus j1 | partition-yes/4 | 2.3 ms ✓ | 60 µs ✓ | 104 µs ✓ | 68 µs ✓ |
| engines | partition-yes | classical focus j1 | partition-yes/5 | 169.0 ms ✓ | 77 µs ✓ | 99 µs ✓ | 79 µs ✓ |
| engines | partition-yes | classical focus j1 | partition-yes/6 | 12.15 s ✓ | 130 µs ✓ | 140 µs ✓ | 130 µs ✓ |
| engines | partition-yes | classical net j1 | partition-yes/12 |  | > 60 s | > 60 s | > 60 s |
| engines | partition-yes | classical net j1 | partition-yes/20 |  | > 60 s | > 60 s | > 60 s |
| engines | partition-yes | classical net j1 | partition-yes/4 | 2.53 s ✓ | 2.83 s ✓ | 2.83 s ✓ | 1.8 ms ✓ |
| engines | partition-yes | classical net j1 | partition-yes/5 | > 60 s | > 60 s | > 60 s | 28.5 ms ✓ |
| engines | partition-yes | classical net j1 | partition-yes/6 | > 60 s | > 60 s | > 60 s | 476.1 ms ✓ |
| engines | wide-m1 | classical focus j1 | wide-m1/16 | 765 µs ✓ | 68 µs ✓ | 77 µs ✓ | 75 µs ✓ |
| engines | wide-m1 | classical focus j1 | wide-m1/2048 | 50.3 ms ? wide | 271.0 ms ? depth | 206.6 ms ✓ | 209.2 ms ✓ |
| engines | wide-m1 | classical focus j1 | wide-m1/24 | 177.5 ms ✓ | 104 µs ✓ | 101 µs ✓ | 89 µs ✓ |
| engines | wide-m1 | classical focus j1 | wide-m1/256 | 971 µs ? wide | 5.6 ms ✓ | 4.0 ms ✓ | 3.3 ms ✓ |
| engines | wide-m1 | classical focus j1 | wide-m1/32 | 45.53 s ✓ | 160 µs ✓ | 148 µs ✓ | 121 µs ✓ |
| engines | wide-m1 | classical focus j1 | wide-m1/8 | 39 µs ✓ | 43 µs ✓ | 52 µs ✓ | 50 µs ✓ |
| engines | wide-m1 | classical net j1 | wide-m1/16 | 111 µs ✓ | 110 µs ✓ | 120 µs ✓ | 129 µs ✓ |
| engines | wide-m1 | classical net j1 | wide-m1/2048 | 638.4 ms ✓ | 637.3 ms ✓ | 586.4 ms ✓ | 598.9 ms ✓ |
| engines | wide-m1 | classical net j1 | wide-m1/24 | 220 µs ✓ | 207 µs ✓ | 199 µs ✓ | 212 µs ✓ |
| engines | wide-m1 | classical net j1 | wide-m1/256 | 9.3 ms ✓ | 9.3 ms ✓ | 9.0 ms ✓ | 9.0 ms ✓ |
| engines | wide-m1 | classical net j1 | wide-m1/32 | 196 µs ✓ | 179 µs ✓ | 182 µs ✓ | 188 µs ✓ |
| engines | wide-m1 | classical net j1 | wide-m1/8 | 45 µs ✓ | 43 µs ✓ | 62 µs ✓ | 61 µs ✓ |
| engines | wide-m2 | classical focus j1 | wide-m2/16 | 714 µs ✓ | 66 µs ✓ | 69 µs ✓ | 67 µs ✓ |
| engines | wide-m2 | classical focus j1 | wide-m2/2048 | 26.9 ms ? wide | 242.7 ms ? depth | 209.9 ms ✓ | 200.5 ms ✓ |
| engines | wide-m2 | classical focus j1 | wide-m2/24 | 171.1 ms ✓ | 97 µs ✓ | 105 µs ✓ | 86 µs ✓ |
| engines | wide-m2 | classical focus j1 | wide-m2/256 | 566 µs ? wide | 4.3 ms ✓ | 3.2 ms ✓ | 2.8 ms ✓ |
| engines | wide-m2 | classical focus j1 | wide-m2/32 | 43.50 s ✓ | 141 µs ✓ | 128 µs ✓ | 106 µs ✓ |
| engines | wide-m2 | classical focus j1 | wide-m2/8 | 38 µs ✓ | 40 µs ✓ | 55 µs ✓ | 52 µs ✓ |
| engines | wide-m2 | classical net j1 | wide-m2/16 | 109 µs ✓ | 103 µs ✓ | 113 µs ✓ | 124 µs ✓ |
| engines | wide-m2 | classical net j1 | wide-m2/2048 | 629.2 ms ✓ | 635.2 ms ✓ | 610.6 ms ✓ | 593.2 ms ✓ |
| engines | wide-m2 | classical net j1 | wide-m2/24 | 217 µs ✓ | 205 µs ✓ | 205 µs ✓ | 216 µs ✓ |
| engines | wide-m2 | classical net j1 | wide-m2/256 | 9.3 ms ✓ | 9.4 ms ✓ | 9.0 ms ✓ | 9.0 ms ✓ |
| engines | wide-m2 | classical net j1 | wide-m2/32 | 199 µs ✓ | 179 µs ✓ | 178 µs ✓ | 188 µs ✓ |
| engines | wide-m2 | classical net j1 | wide-m2/8 | 44 µs ✓ | 43 µs ✓ | 55 µs ✓ | 63 µs ✓ |
| engines | wide-m3 | classical focus j1 | wide-m3/12 | 84 µs ✓ | 50 µs ✓ | 69 µs ✓ | 60 µs ✓ |
| engines | wide-m3 | classical focus j1 | wide-m3/2048 |  | 215.8 ms ? depth | 192.5 ms ✓ | 179.1 ms ✓ |
| engines | wide-m3 | classical focus j1 | wide-m3/24 | 168.6 ms ✓ | 94 µs ✓ | 96 µs ✓ | 79 µs ✓ |
| engines | wide-m3 | classical focus j1 | wide-m3/256 |  | 3.7 ms ✓ | 3.0 ms ✓ | 2.5 ms ✓ |
| engines | wide-m3 | classical focus j1 | wide-m3/30 | 10.85 s ✓ | 126 µs ✓ | 123 µs ✓ | 104 µs ✓ |
| engines | wide-m3 | classical net j1 | wide-m3/12 | 71 µs ✓ | 74 µs ✓ | 85 µs ✓ | 96 µs ✓ |
| engines | wide-m3 | classical net j1 | wide-m3/2048 |  | 649.4 ms ✓ | 628.1 ms ✓ | 630.5 ms ✓ |
| engines | wide-m3 | classical net j1 | wide-m3/24 | 217 µs ✓ | 213 µs ✓ | 211 µs ✓ | 226 µs ✓ |
| engines | wide-m3 | classical net j1 | wide-m3/256 |  | 9.7 ms ✓ | 9.4 ms ✓ | 9.3 ms ✓ |
| engines | wide-m3 | classical net j1 | wide-m3/30 | 182 µs ✓ | 166 µs ✓ | 176 µs ✓ | 177 µs ✓ |
| engines | wide-m4 | classical focus j1 | wide-m4/12 | 87 µs ✓ | 53 µs ✓ | 59 µs ✓ | 58 µs ✓ |
| engines | wide-m4 | classical focus j1 | wide-m4/2048 |  | 203.7 ms ? depth | 182.0 ms ✓ | 166.7 ms ✓ |
| engines | wide-m4 | classical focus j1 | wide-m4/24 | 171.8 ms ✓ | 92 µs ✓ | 93 µs ✓ | 83 µs ✓ |
| engines | wide-m4 | classical focus j1 | wide-m4/256 |  | 3.5 ms ✓ | 2.8 ms ✓ | 2.3 ms ✓ |
| engines | wide-m4 | classical focus j1 | wide-m4/28 | 2.75 s ✓ | 115 µs ✓ | 105 µs ✓ | 91 µs ✓ |
| engines | wide-m4 | classical net j1 | wide-m4/12 | 76 µs ✓ | 73 µs ✓ | 81 µs ✓ | 90 µs ✓ |
| engines | wide-m4 | classical net j1 | wide-m4/2048 |  | 673.2 ms ✓ | 651.3 ms ✓ | 633.1 ms ✓ |
| engines | wide-m4 | classical net j1 | wide-m4/24 | 223 µs ✓ | 215 µs ✓ | 218 µs ✓ | 224 µs ✓ |
| engines | wide-m4 | classical net j1 | wide-m4/256 |  | 10.1 ms ✓ | 9.8 ms ✓ | 9.4 ms ✓ |
| engines | wide-m4 | classical net j1 | wide-m4/28 | 290 µs ✓ | 278 µs ✓ | 258 µs ✓ | 290 µs ✓ |
| families | 3-partition-mll-no | classical auto j1 | 3-partition-mll-no/4 | 14 µs ✗ | 19 µs ✗ | 39 µs ✗ | 45 µs ✗ |
| families | 3-partition-mll-no | classical auto j1 | 3-partition-mll-no/5 | 15 µs ✗ | 18 µs ✗ | 41 µs ✗ | 47 µs ✗ |
| families | 3-partition-mll-no | classical auto j1 | 3-partition-mll-no/6 | 17 µs ✗ | 20 µs ✗ | 45 µs ✗ | 44 µs ✗ |
| families | 3-partition-mll-no | classical auto j1 | 3-partition-mll-no/8 | 16 µs ✗ | 20 µs ✗ | 40 µs ✗ | 45 µs ✗ |
| families | 3-partition-mll-yes | classical auto j1 | 3-partition-mll-yes/12 | 35 µs ✓ | 41 µs ✓ | 55 µs ✓ | 51 µs ✓ |
| families | 3-partition-mll-yes | classical auto j1 | 3-partition-mll-yes/4 | 27 µs ✓ | 36 µs ✓ | 54 µs ✓ | 51 µs ✓ |
| families | 3-partition-mll-yes | classical auto j1 | 3-partition-mll-yes/6 | 28 µs ✓ | 38 µs ✓ | 53 µs ✓ | 52 µs ✓ |
| families | 3-partition-mll-yes | classical auto j1 | 3-partition-mll-yes/8 | 31 µs ✓ | 39 µs ✓ | 56 µs ✓ | 55 µs ✓ |
| families | 3-partition-no | classical auto j1 | 3-partition-no/4 | 49.77 s ✗ | 299 µs ✗ | 319 µs ✗ | 286 µs ✗ |
| families | 3-partition-no | classical auto j1 | 3-partition-no/5 | > 300 s | 358 µs ✗ | 346 µs ✗ | 320 µs ✗ |
| families | 3-partition-yes | classical auto j1 | 3-partition-yes/12 | 30.1 ms ✓ | 48 µs ✓ | 64 µs ✓ | 63 µs ✓ |
| families | 3-partition-yes | classical auto j1 | 3-partition-yes/4 | 180 µs ✓ | 41 µs ✓ | 68 µs ✓ | 59 µs ✓ |
| families | 3-partition-yes | classical auto j1 | 3-partition-yes/6 | 555 µs ✓ | 43 µs ✓ | 61 µs ✓ | 59 µs ✓ |
| families | 3-partition-yes | classical auto j1 | 3-partition-yes/8 | 2.0 ms ✓ | 44 µs ✓ | 63 µs ✓ | 59 µs ✓ |
| families | additive | classical auto j1 | additive/12 | 13.6 ms ✓ | 13.8 ms ✓ | 10.8 ms ✓ | 11.4 ms ✓ |
| families | additive | classical auto j1 | additive/14 | 69.7 ms ✓ | 68.0 ms ✓ | 58.4 ms ✓ | 61.2 ms ✓ |
| families | additive | classical auto j1 | additive/16 | 527.9 ms ✓ | 209.5 ms ✓ | 181.6 ms ✓ | 192.2 ms ✓ |
| families | additive | classical auto j1 | additive/8 | 363 µs ✓ | 378 µs ✓ | 329 µs ✓ | 370 µs ✓ |
| families | chain | classical auto j1 | chain/128 | 269.7 ms ✓ | 40.2 ms ✓ | 33.7 ms ✓ | 34.0 ms ✓ |
| families | chain | classical auto j1 | chain/16 | 247 µs ✓ | 151 µs ✓ | 154 µs ✓ | 152 µs ✓ |
| families | chain | classical auto j1 | chain/256 | 2.82 s ✓ | 321.3 ms ✓ | 274.7 ms ✓ | 276.1 ms ✓ |
| families | chain | classical auto j1 | chain/64 | 22.1 ms ✓ | 5.1 ms ✓ | 4.4 ms ✓ | 4.4 ms ✓ |
| families | chain-fo | classical auto j1 | chain-fo/16#0 |  |  |  | 370 µs ✓ |
| families | chain-fo | classical auto j1 | chain-fo/16#1 |  |  |  | 345 µs ? bound |
| families | chain-fo | classical auto j1 | chain-fo/256#0 |  |  |  | 309.7 ms ✓ |
| families | chain-fo | classical auto j1 | chain-fo/256#1 |  |  |  | 275.7 ms ? bound |
| families | chain-fo | classical auto j1 | chain-fo/512#0 |  |  |  | 1.19 s ? depth |
| families | chain-fo | classical auto j1 | chain-fo/512#1 |  |  |  | 1.03 s ? depth |
| families | chain-fo | classical auto j1 | chain-fo/64#0 |  |  |  | 7.5 ms ✓ |
| families | chain-fo | classical auto j1 | chain-fo/64#1 |  |  |  | 6.9 ms ? bound |
| families | contraction | intuitionistic auto j1 | contraction/2#0 |  |  |  | 57 µs ✓ |
| families | contraction | intuitionistic auto j1 | contraction/2#1 |  |  |  | 73 µs ? bound |
| families | contraction | intuitionistic auto j1 | contraction/4#0 |  |  |  | 154 µs ✓ |
| families | contraction | intuitionistic auto j1 | contraction/4#1 |  |  |  | 488 µs ? bound |
| families | contraction | intuitionistic auto j1 | contraction/8#0 |  |  |  | 66.5 ms ✓ |
| families | contraction | intuitionistic auto j1 | contraction/8#1 |  |  |  | 74.8 ms ✗ |
| families | counter | classical auto j1 | counter/16 | > 300 s | 248 µs ✓ | 37 µs ✓ | 35 µs ✓ |
| families | counter | classical auto j1 | counter/2 | 11 µs ✓ | 51 µs ✓ | 27 µs ✓ | 35 µs ✓ |
| families | counter | classical auto j1 | counter/32 |  | 2.18 s ✓ | 43 µs ✓ | 40 µs ✓ |
| families | counter | classical auto j1 | counter/4 | 84 µs ✓ | 65 µs ✓ | 28 µs ✓ | 29 µs ✓ |
| families | counter | classical auto j1 | counter/64 |  | 80.68 s ✓ | 55 µs ✓ | 53 µs ✓ |
| families | counter | classical auto j1 | counter/8 | 124.0 ms ✓ | 87 µs ✓ | 34 µs ✓ | 32 µs ✓ |
| families | counter-over | classical auto j1 | counter-over/16 | > 300 s | 299 µs ✗ | 35 µs ✗ | 38 µs ✗ |
| families | counter-over | classical auto j1 | counter-over/2 | 24 µs ? bound | 55 µs ✗ | 24 µs ✗ | 28 µs ✗ |
| families | counter-over | classical auto j1 | counter-over/32 |  | 2.22 s ? bound | 44 µs ✗ | 42 µs ✗ |
| families | counter-over | classical auto j1 | counter-over/4 | 368 µs ? bound | 59 µs ✗ | 26 µs ✗ | 31 µs ✗ |
| families | counter-over | classical auto j1 | counter-over/64 |  | 85.03 s ? bound | 45 µs ✗ | 45 µs ✗ |
| families | counter-over | classical auto j1 | counter-over/8 | 492.7 ms ? bound | 110 µs ✗ | 33 µs ✗ | 36 µs ✗ |
| families | duplicate | classical auto j1 | duplicate/2#1 |  |  |  | 25 µs ✓ |
| families | duplicate | classical auto j1 | duplicate/4#1 |  |  |  | 29 µs ✓ |
| families | duplicate | classical auto j1 | duplicate/8#1 |  |  |  | 28 µs ✓ |
| families | duplicate | intuitionistic auto j1 | duplicate/2#0 |  |  |  | 28 µs ✓ |
| families | duplicate | intuitionistic auto j1 | duplicate/4#0 |  |  |  | 33 µs ✓ |
| families | duplicate | intuitionistic auto j1 | duplicate/8#0 |  |  |  | 30 µs ✓ |
| families | euler | intuitionistic auto j1 | euler/5#0 |  |  |  | 55 µs ✓ |
| families | euler | intuitionistic auto j1 | euler/5#1 |  |  |  | 151 µs ✗ |
| families | euler | intuitionistic auto j1 | euler/6#0 |  |  |  | 72 µs ✓ |
| families | euler | intuitionistic auto j1 | euler/6#1 |  |  |  | 344 µs ✗ |
| families | euler | intuitionistic auto j1 | euler/8#0 |  |  |  | 72 µs ✓ |
| families | euler | intuitionistic auto j1 | euler/8#1 |  |  |  | 1.6 ms ✗ |
| families | growing | classical auto j1 | growing/1024 | 567.8 ms ? depth | 256.8 ms ? depth | 114.7 ms ? depth | 113.3 ms ✗ |
| families | growing | classical auto j1 | growing/16 | 142 µs ? bound | 342 µs ? bound | 213 µs ? bound | 231 µs ✗ |
| families | growing | classical auto j1 | growing/256 | 79.3 ms ? bound | 41.8 ms ? bound | 18.4 ms ? bound | 18.2 ms ✗ |
| families | growing | classical auto j1 | growing/64 | 2.1 ms ? bound | 1.6 ms ? bound | 800 µs ? bound | 808 µs ✗ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#0 |  |  |  | 125 µs ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#1 |  |  |  | 83 µs ✗ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#2 |  |  |  | 247 µs ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#3 |  |  |  | 89 µs ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#4 |  |  |  | 86 µs ✗ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#0 |  |  |  | 4.0 ms ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#1 |  |  |  | 9.9 ms ✗ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#2 |  |  |  | 429.6 ms ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#3 |  |  |  | 4.5 ms ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#4 |  |  |  | 5.8 ms ✗ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#0 |  |  |  | 13.3 ms ? depth |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#1 |  |  |  | 46.1 ms ✗ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#2 |  |  |  | 3.87 s ? depth |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#3 |  |  |  | 18.3 ms ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#4 |  |  |  | 24.8 ms ✗ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#0 |  |  |  | 470 µs ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#1 |  |  |  | 552 µs ✗ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#2 |  |  |  | 6.7 ms ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#3 |  |  |  | 406 µs ✓ |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#4 |  |  |  | 415 µs ✗ |
| families | mix | mix auto j1 | mix/10 | 213.99 s ✗ | 145.85 s ✗ | 62.55 s ✗ | 258.7 ms ✗ |
| families | mix | mix auto j1 | mix/11 | > 300 s | > 300 s | > 300 s | 269.6 ms ✗ |
| families | mix | mix auto j1 | mix/4 | 275 µs ✗ | 302 µs ✗ | 365 µs ✗ | 330 µs ✗ |
| families | mix | mix auto j1 | mix/6 | 19.9 ms ✗ | 18.4 ms ✗ | 14.0 ms ✗ | 11.2 ms ✗ |
| families | mix | mix auto j1 | mix/8 | 1.91 s ✗ | 1.55 s ✗ | 858.5 ms ✗ | 256.8 ms ✗ |
| families | mix | mix auto j1 | mix/9 | 19.82 s ✗ | 14.10 s ✗ | 7.31 s ✗ | 257.3 ms ✗ |
| families | partition-no | classical auto j1 | partition-no/12 |  | 3.87 s ✗ | 3.82 s ✗ | 3.21 s ✗ |
| families | partition-no | classical auto j1 | partition-no/14 |  | > 300 s | > 300 s | > 300 s |
| families | partition-no | classical auto j1 | partition-no/15 |  | > 300 s | > 300 s | > 300 s |
| families | partition-no | classical auto j1 | partition-no/3 | 4.4 ms ✗ | 59 µs ✗ | 79 µs ✗ | 76 µs ✗ |
| families | partition-no | classical auto j1 | partition-no/4 | 113.8 ms ✗ | 141 µs ✗ | 144 µs ✗ | 136 µs ✗ |
| families | partition-no | classical auto j1 | partition-no/5 | > 300 s | 465 µs ✗ | 435 µs ✗ | 410 µs ✗ |
| families | partition-no | classical auto j1 | partition-no/9 |  | 84.9 ms ✗ | 85.5 ms ✗ | 72.4 ms ✗ |
| families | partition-yes | classical auto j1 | partition-yes/12 |  | 14.8 ms ✓ | 13.8 ms ✓ | 12.6 ms ✓ |
| families | partition-yes | classical auto j1 | partition-yes/16 |  | 186.8 ms ✓ | 147.8 ms ✓ | 134.4 ms ✓ |
| families | partition-yes | classical auto j1 | partition-yes/20 |  | 568.4 ms ✓ | 454.7 ms ✓ | 411.7 ms ✓ |
| families | partition-yes | classical auto j1 | partition-yes/24 |  | 117.66 s ✓ | 74.41 s ✓ | 71.59 s ✓ |
| families | partition-yes | classical auto j1 | partition-yes/28 |  | > 300 s | > 300 s | > 300 s |
| families | partition-yes | classical auto j1 | partition-yes/4 | 2.3 ms ✓ | 55 µs ✓ | 81 µs ✓ | 69 µs ✓ |
| families | partition-yes | classical auto j1 | partition-yes/5 | 186.5 ms ✓ | 76 µs ✓ | 93 µs ✓ | 86 µs ✓ |
| families | partition-yes | classical auto j1 | partition-yes/6 | 12.74 s ✓ | 130 µs ✓ | 136 µs ✓ | 131 µs ✓ |
| families | partition-yes | classical auto j1 | partition-yes/7 | > 300 s | 213 µs ✓ | 211 µs ✓ | 196 µs ✓ |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/2#0 |  |  |  | 71 µs ✓ |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/2#1 |  |  |  | 40 µs ✗ |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/3#0 |  |  |  | 951 µs ✓ |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/3#1 |  |  |  | 76 µs ✗ |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/4#0 |  |  |  | 38.4 ms ✓ |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/4#1 |  |  |  | 284 µs ✗ |
| families | qbf | classical auto j1 | qbf/12#0 | 121.0 ms ✓ | 1.8 ms ✓ | 1.2 ms ✓ | 1.2 ms ✓ |
| families | qbf | classical auto j1 | qbf/12#1 | 28.8 ms ✗ | 502 µs ✗ | 388 µs ✗ | 942 µs ✗ |
| families | qbf | classical auto j1 | qbf/12#2 | 45.0 ms ✓ | 925 µs ✓ | 625 µs ✓ | 597 µs ✓ |
| families | qbf | classical auto j1 | qbf/12#3 | 19.2 ms ✗ | 371 µs ✗ | 297 µs ✗ | 891 µs ✗ |
| families | qbf | classical auto j1 | qbf/16#0 | 7.87 s ✓ | 10.1 ms ✓ | 6.0 ms ✓ | 5.2 ms ✓ |
| families | qbf | classical auto j1 | qbf/16#1 | 1.45 s ✗ | 1.5 ms ✗ | 1.1 ms ✗ | 8.0 ms ✗ |
| families | qbf | classical auto j1 | qbf/16#2 | 2.09 s ✗ | 2.4 ms ✗ | 1.7 ms ✗ | 8.5 ms ✗ |
| families | qbf | classical auto j1 | qbf/16#3 | 6.56 s ✓ | 7.7 ms ✓ | 5.4 ms ✓ | 4.4 ms ✓ |
| families | qbf | classical auto j1 | qbf/20#0 | 87.23 s ✗ | 7.3 ms ✗ | 5.3 ms ✗ | 80.3 ms ✗ |
| families | qbf | classical auto j1 | qbf/20#1 | 123.61 s ✗ | 12.0 ms ✗ | 8.2 ms ✗ | 82.2 ms ✗ |
| families | qbf | classical auto j1 | qbf/20#2 | > 300 s | 49.8 ms ✓ | 30.0 ms ✓ | 26.4 ms ✓ |
| families | qbf | classical auto j1 | qbf/20#3 | 89.32 s ✗ | 7.1 ms ✗ | 5.1 ms ✗ | 80.5 ms ✗ |
| families | qbf | classical auto j1 | qbf/24#0 | > 300 s | 163.5 ms ✗ | 101.8 ms ✗ | 174.9 ms ✗ |
| families | qbf | classical auto j1 | qbf/24#1 | > 300 s | 58.0 ms ✗ | 37.0 ms ✗ | 120.5 ms ✗ |
| families | qbf | classical auto j1 | qbf/24#2 | > 300 s | 38.7 ms ✗ | 25.7 ms ✗ | 109.2 ms ✗ |
| families | qbf | classical auto j1 | qbf/24#3 | > 300 s | 57.2 ms ✗ | 36.1 ms ✗ | 120.3 ms ✗ |
| families | qbf | classical auto j1 | qbf/32#0 |  | 875.9 ms ✗ | 552.4 ms ✗ | 561.0 ms ✗ |
| families | qbf | classical auto j1 | qbf/32#1 |  | 943.7 ms ✗ | 626.7 ms ✗ | 593.4 ms ✗ |
| families | qbf | classical auto j1 | qbf/32#2 |  | 2.04 s ✗ | 1.14 s ✗ | 1.03 s ✗ |
| families | qbf | classical auto j1 | qbf/32#3 |  | 778.5 ms ✗ | 476.7 ms ✗ | 487.3 ms ✗ |
| families | qbf | classical auto j1 | qbf/40#0 |  | 17.53 s ✗ | 12.11 s ✗ | 10.00 s ✗ |
| families | qbf | classical auto j1 | qbf/40#1 |  | 20.18 s ✗ | 13.65 s ✗ | 11.42 s ✗ |
| families | qbf | classical auto j1 | qbf/40#2 |  | 23.34 s ✗ | 16.21 s ✗ | 13.33 s ✗ |
| families | qbf | classical auto j1 | qbf/40#3 |  | 18.29 s ✗ | 12.54 s ✗ | 10.46 s ✗ |
| families | qbf | classical auto j1 | qbf/44#0 |  | 77.60 s ✗ | 54.12 s ✗ | 44.75 s ✗ |
| families | qbf | classical auto j1 | qbf/44#1 |  | 79.01 s ✗ | 53.43 s ✗ | 45.37 s ✗ |
| families | qbf | classical auto j1 | qbf/44#2 |  | 115.85 s ✗ | 80.52 s ✗ | 66.52 s ✗ |
| families | qbf | classical auto j1 | qbf/44#3 |  | 218.72 s ✗ | 28.71 s ? memory | 23.23 s ? memory |
| families | qbf | classical auto j1 | qbf/48#0 |  | 290.27 s unknown | 84.57 s ? memory | 70.03 s ? memory |
| families | qbf | classical auto j1 | qbf/48#1 |  | 282.04 s ✗ | 207.33 s ✗ | 166.29 s ✗ |
| families | qbf | classical auto j1 | qbf/48#2 |  | 230.53 s ✗ | 161.73 s ✗ | 141.52 s ✗ |
| families | qbf | classical auto j1 | qbf/48#3 |  | > 300 s | 222.46 s ✗ | 185.15 s ✗ |
| families | qbf | classical auto j1 | qbf/8#0 | 551 µs ✗ | 123 µs ✗ | 131 µs ✗ | 176 µs ✗ |
| families | qbf | classical auto j1 | qbf/8#1 | 587 µs ✗ | 161 µs ✗ | 144 µs ✗ | 190 µs ✗ |
| families | qbf | classical auto j1 | qbf/8#2 | 830 µs ✓ | 184 µs ✓ | 156 µs ✓ | 152 µs ✓ |
| families | qbf | classical auto j1 | qbf/8#3 | 643 µs ✗ | 141 µs ✗ | 130 µs ✗ | 179 µs ✗ |
| families | reach-fo | intuitionistic auto j1 | reach-fo/2#0 |  |  |  | 110 µs ✓ |
| families | reach-fo | intuitionistic auto j1 | reach-fo/2#1 |  |  |  | 123 µs ? bound |
| families | reach-fo | intuitionistic auto j1 | reach-fo/4#0 |  |  |  | 1.2 ms ✓ |
| families | reach-fo | intuitionistic auto j1 | reach-fo/4#1 |  |  |  | 1.6 ms ? bound |
| families | wide-m1 | classical auto j1 | wide-m1/16 | 110 µs ✓ | 98 µs ✓ | 110 µs ✓ | 66 µs ✓ |
| families | wide-m1 | classical auto j1 | wide-m1/2048 | 610.2 ms ✓ | 581.5 ms ✓ | 523.0 ms ✓ | 202.6 ms ✓ |
| families | wide-m1 | classical auto j1 | wide-m1/24 | 198 µs ✓ | 199 µs ✓ | 192 µs ✓ | 88 µs ✓ |
| families | wide-m1 | classical auto j1 | wide-m1/256 | 8.8 ms ✓ | 8.2 ms ✓ | 7.7 ms ✓ | 3.4 ms ✓ |
| families | wide-m1 | classical auto j1 | wide-m1/32 | 178 µs ✓ | 166 µs ✓ | 174 µs ✓ | 118 µs ✓ |
| families | wide-m1 | classical auto j1 | wide-m1/8 | 40 µs ✓ | 40 µs ✓ | 54 µs ✓ | 54 µs ✓ |
| families | wide-m2 | classical auto j1 | wide-m2/16 | 103 µs ✓ | 97 µs ✓ | 107 µs ✓ | 65 µs ✓ |
| families | wide-m2 | classical auto j1 | wide-m2/2048 | 602.4 ms ✓ | 571.3 ms ✓ | 555.5 ms ✓ | 186.2 ms ✓ |
| families | wide-m2 | classical auto j1 | wide-m2/24 | 201 µs ✓ | 191 µs ✓ | 194 µs ✓ | 84 µs ✓ |
| families | wide-m2 | classical auto j1 | wide-m2/256 | 8.8 ms ✓ | 8.5 ms ✓ | 8.0 ms ✓ | 2.6 ms ✓ |
| families | wide-m2 | classical auto j1 | wide-m2/32 | 170 µs ✓ | 165 µs ✓ | 171 µs ✓ | 106 µs ✓ |
| families | wide-m2 | classical auto j1 | wide-m2/8 | 40 µs ✓ | 44 µs ✓ | 54 µs ✓ | 54 µs ✓ |
| families | wide-m3 | classical auto j1 | wide-m3/1024 |  | 55.2 ms ✓ | 42.8 ms ✓ | 41.0 ms ✓ |
| families | wide-m3 | classical auto j1 | wide-m3/12 | 77 µs ✓ | 44 µs ✓ | 59 µs ✓ | 60 µs ✓ |
| families | wide-m3 | classical auto j1 | wide-m3/2048 |  | 204.3 ms ? depth | 172.9 ms ✓ | 163.2 ms ✓ |
| families | wide-m3 | classical auto j1 | wide-m3/24 | 166.4 ms ✓ | 88 µs ✓ | 87 µs ✓ | 80 µs ✓ |
| families | wide-m3 | classical auto j1 | wide-m3/256 |  | 3.3 ms ✓ | 2.7 ms ✓ | 2.4 ms ✓ |
| families | wide-m3 | classical auto j1 | wide-m3/30 | 10.57 s ✓ | 112 µs ✓ | 104 µs ✓ | 100 µs ✓ |
| families | wide-m3 | classical auto j1 | wide-m3/36 | > 300 s | 139 µs ✓ | 128 µs ✓ | 114 µs ✓ |
| families | wide-m4 | classical auto j1 | wide-m4/1024 |  | 51.6 ms ✓ | 40.2 ms ✓ | 38.5 ms ✓ |
| families | wide-m4 | classical auto j1 | wide-m4/12 | 79 µs ✓ | 49 µs ✓ | 58 µs ✓ | 62 µs ✓ |
| families | wide-m4 | classical auto j1 | wide-m4/2048 |  | 191.8 ms ? depth | 167.4 ms ✓ | 154.0 ms ✓ |
| families | wide-m4 | classical auto j1 | wide-m4/24 | 170.2 ms ✓ | 85 µs ✓ | 92 µs ✓ | 82 µs ✓ |
| families | wide-m4 | classical auto j1 | wide-m4/256 |  | 3.2 ms ✓ | 2.5 ms ✓ | 2.2 ms ✓ |
| families | wide-m4 | classical auto j1 | wide-m4/28 | 2.72 s ✓ | 104 µs ✓ | 97 µs ✓ | 92 µs ✓ |
| families | wide-m4 | classical auto j1 | wide-m4/32 | 43.39 s ✓ | 119 µs ✓ | 108 µs ✓ | 104 µs ✓ |
| families | wide-m4 | classical auto j1 | wide-m4/36 | > 300 s | 137 µs ✓ | 134 µs ✓ | 117 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/10#0 |  |  |  | 62 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/10#1 |  |  |  | 56 µs ✗ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/10#2 |  |  |  | 62 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/10#3 |  |  |  | 2.57 s ✗ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/12#0 |  |  |  | 66 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/12#1 |  |  |  | 61 µs ✗ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/12#2 |  |  |  | 69 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/12#3 |  |  |  | > 300 s |
| families | witness-fo | intuitionistic auto j1 | witness-fo/4#0 |  |  |  | 56 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/4#1 |  |  |  | 50 µs ✗ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/4#2 |  |  |  | 55 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/4#3 |  |  |  | 68 µs ✗ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/8#0 |  |  |  | 59 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/8#1 |  |  |  | 55 µs ✗ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/8#2 |  |  |  | 58 µs ✓ |
| families | witness-fo | intuitionistic auto j1 | witness-fo/8#3 |  |  |  | 28.2 ms ✗ |
| intuitionistic | 3-partition-no | intuitionistic auto j1 | 3-partition-no/4 | 25.38 s ✗ | 293 µs ✗ | 273 µs ✗ | 275 µs ✗ |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 3-partition-yes/12 | 16.5 ms ✓ | 51 µs ✓ | 59 µs ✓ | 63 µs ✓ |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 3-partition-yes/4 | 116 µs ✓ | 40 µs ✓ | 60 µs ✓ | 58 µs ✓ |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 3-partition-yes/6 | 320 µs ✓ | 46 µs ✓ | 59 µs ✓ | 56 µs ✓ |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 3-partition-yes/8 | 1.1 ms ✓ | 47 µs ✓ | 58 µs ✓ | 55 µs ✓ |
| intuitionistic | chain | intuitionistic auto j1 | chain/128 | 275.9 ms ✓ | 42.4 ms ✓ | 35.6 ms ✓ | 37.2 ms ✓ |
| intuitionistic | chain | intuitionistic auto j1 | chain/16 | 273 µs ✓ | 159 µs ✓ | 149 µs ✓ | 159 µs ✓ |
| intuitionistic | chain | intuitionistic auto j1 | chain/256 | 2.89 s ✓ | 347.4 ms ✓ | 287.5 ms ✓ | 256.4 ms ✓ |
| intuitionistic | chain | intuitionistic auto j1 | chain/64 | 22.6 ms ✓ | 5.4 ms ✓ | 4.6 ms ✓ | 4.7 ms ✓ |
| intuitionistic | counter | intuitionistic auto j1 | counter/16 | > 300 s | 251 µs ✓ | 35 µs ✓ | 36 µs ✓ |
| intuitionistic | counter | intuitionistic auto j1 | counter/2 | 12 µs ✓ | 50 µs ✓ | 27 µs ✓ | 29 µs ✓ |
| intuitionistic | counter | intuitionistic auto j1 | counter/32 |  | 19.9 ms ✓ | 42 µs ✓ | 46 µs ✓ |
| intuitionistic | counter | intuitionistic auto j1 | counter/4 | 48 µs ✓ | 61 µs ✓ | 28 µs ✓ | 28 µs ✓ |
| intuitionistic | counter | intuitionistic auto j1 | counter/64 |  | 267.7 ms ✓ | 53 µs ✓ | 57 µs ✓ |
| intuitionistic | counter | intuitionistic auto j1 | counter/8 | 15.4 ms ✓ | 88 µs ✓ | 32 µs ✓ | 32 µs ✓ |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/16 | > 300 s | 306 µs ✗ | 35 µs ✗ | 39 µs ✗ |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/2 | 21 µs ? bound | 54 µs ✗ | 22 µs ✗ | 29 µs ✗ |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/32 |  | 20.3 ms ? bound | 44 µs ✗ | 40 µs ✗ |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/4 | 60 µs ? bound | 66 µs ✗ | 26 µs ✗ | 31 µs ✗ |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/64 |  | 268.5 ms ? bound | 54 µs ✗ | 48 µs ✗ |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/8 | 18.4 ms ? bound | 107 µs ✗ | 32 µs ✗ | 35 µs ✗ |
| long-1 | 3-partition-no | classical auto j1 | 3-partition-no/5 | 302.42 s ✗ | 393 µs ✗ | 400 µs ✗ | 514 µs ✗ |
| long-1 | counter | classical auto j1 | counter/16 | > 1200 s | 280 µs ✓ | 48 µs ✓ | 67 µs ✓ |
| long-1 | counter | classical auto j1 | counter/64 |  | 85.14 s ✓ | 60 µs ✓ | 76 µs ✓ |
| long-1 | partition-no | classical auto j1 | partition-no/15 |  | > 1200 s | > 1200 s | > 1200 s |
| long-1 | partition-no | classical auto j1 | partition-no/5 | 811.48 s ✗ | 477 µs ✗ | 441 µs ✗ | 558 µs ✗ |
| long-1 | partition-yes | classical auto j1 | partition-yes/28 |  | > 1200 s | > 1200 s | > 1200 s |
| long-1 | partition-yes | classical auto j1 | partition-yes/7 | > 1200 s | 230 µs ✓ | 214 µs ✓ | 289 µs ✓ |
| long-2 | mix | mix auto j1 | mix/11 | > 1200 s | > 1200 s | > 1200 s | 347.1 ms ✗ |
| long-2 | qbf | classical auto j1 | qbf/24#0 | > 1200 s | 161.2 ms ✗ | 99.6 ms ✗ | 258.7 ms ✗ |
| long-2 | qbf | classical auto j1 | qbf/48#0 |  | 301.01 s unknown | 83.81 s ? memory | 101.59 s ? memory |
| long-2 | wide-m3 | classical auto j1 | wide-m3/36 | 697.42 s ✓ | 157 µs ✓ | 154 µs ✓ | 173 µs ✓ |
| long-2 | wide-m4 | classical auto j1 | wide-m4/36 | 685.07 s ✓ | 144 µs ✓ | 135 µs ✓ | 161 µs ✓ |
| parallel | 3-partition-no | classical auto j1 | 3-partition-no/4 | 45.09 s ✗ | 395 µs ✗ | 379 µs ✗ | 391 µs ✗ |
| parallel | 3-partition-no | classical auto j1 | 3-partition-no/5 | > 120 s | 470 µs ✗ | 432 µs ✗ | 453 µs ✗ |
| parallel | 3-partition-no | classical auto j16 | 3-partition-no/4 | 5.80 s ✗ | 720 µs ✗ | 631 µs ✗ | 671 µs ✗ |
| parallel | 3-partition-no | classical auto j16 | 3-partition-no/5 | 37.07 s ✗ | 689 µs ✗ | 674 µs ✗ | 642 µs ✗ |
| parallel | 3-partition-no | classical auto j2 | 3-partition-no/4 | 23.49 s ✗ | 439 µs ✗ | 544 µs ✗ | 531 µs ✗ |
| parallel | 3-partition-no | classical auto j2 | 3-partition-no/5 | > 120 s | 483 µs ✗ | 567 µs ✗ | 525 µs ✗ |
| parallel | 3-partition-no | classical auto j4 | 3-partition-no/4 | 12.68 s ✗ | 342 µs ✗ | 428 µs ✗ | 425 µs ✗ |
| parallel | 3-partition-no | classical auto j4 | 3-partition-no/5 | 77.70 s ✗ | 354 µs ✗ | 461 µs ✗ | 481 µs ✗ |
| parallel | 3-partition-no | classical auto j8 | 3-partition-no/4 | 7.45 s ✗ | 467 µs ✗ | 550 µs ✗ | 471 µs ✗ |
| parallel | 3-partition-no | classical auto j8 | 3-partition-no/5 | 46.72 s ✗ | 431 µs ✗ | 562 µs ✗ | 496 µs ✗ |
| parallel | 3-partition-yes | classical auto j1 | 3-partition-yes/12 | 37.7 ms ✓ | 59 µs ✓ | 88 µs ✓ | 88 µs ✓ |
| parallel | 3-partition-yes | classical auto j1 | 3-partition-yes/4 | 217 µs ✓ | 48 µs ✓ | 103 µs ✓ | 93 µs ✓ |
| parallel | 3-partition-yes | classical auto j1 | 3-partition-yes/6 | 674 µs ✓ | 58 µs ✓ | 86 µs ✓ | 85 µs ✓ |
| parallel | 3-partition-yes | classical auto j1 | 3-partition-yes/8 | 2.4 ms ✓ | 54 µs ✓ | 86 µs ✓ | 88 µs ✓ |
| parallel | 3-partition-yes | classical auto j16 | 3-partition-yes/12 | 30.4 ms ✓ | 540 µs ✓ | 546 µs ✓ | 693 µs ✓ |
| parallel | 3-partition-yes | classical auto j16 | 3-partition-yes/4 | 706 µs ✓ | 506 µs ✓ | 591 µs ✓ | 518 µs ✓ |
| parallel | 3-partition-yes | classical auto j16 | 3-partition-yes/6 | 1.3 ms ✓ | 452 µs ✓ | 593 µs ✓ | 546 µs ✓ |
| parallel | 3-partition-yes | classical auto j16 | 3-partition-yes/8 | 2.8 ms ✓ | 601 µs ✓ | 649 µs ✓ | 727 µs ✓ |
| parallel | 3-partition-yes | classical auto j2 | 3-partition-yes/12 | 29.6 ms ✓ | 163 µs ✓ | 248 µs ✓ | 206 µs ✓ |
| parallel | 3-partition-yes | classical auto j2 | 3-partition-yes/4 | 489 µs ✓ | 209 µs ✓ | 215 µs ✓ | 211 µs ✓ |
| parallel | 3-partition-yes | classical auto j2 | 3-partition-yes/6 | 920 µs ✓ | 142 µs ✓ | 220 µs ✓ | 228 µs ✓ |
| parallel | 3-partition-yes | classical auto j2 | 3-partition-yes/8 | 2.4 ms ✓ | 164 µs ✓ | 263 µs ✓ | 256 µs ✓ |
| parallel | 3-partition-yes | classical auto j4 | 3-partition-yes/12 | 29.3 ms ✓ | 202 µs ✓ | 301 µs ✓ | 254 µs ✓ |
| parallel | 3-partition-yes | classical auto j4 | 3-partition-yes/4 | 425 µs ✓ | 198 µs ✓ | 247 µs ✓ | 294 µs ✓ |
| parallel | 3-partition-yes | classical auto j4 | 3-partition-yes/6 | 911 µs ✓ | 179 µs ✓ | 269 µs ✓ | 309 µs ✓ |
| parallel | 3-partition-yes | classical auto j4 | 3-partition-yes/8 | 2.3 ms ✓ | 183 µs ✓ | 278 µs ✓ | 291 µs ✓ |
| parallel | 3-partition-yes | classical auto j8 | 3-partition-yes/12 | 29.5 ms ✓ | 308 µs ✓ | 458 µs ✓ | 369 µs ✓ |
| parallel | 3-partition-yes | classical auto j8 | 3-partition-yes/4 | 433 µs ✓ | 328 µs ✓ | 416 µs ✓ | 391 µs ✓ |
| parallel | 3-partition-yes | classical auto j8 | 3-partition-yes/6 | 1.0 ms ✓ | 287 µs ✓ | 337 µs ✓ | 420 µs ✓ |
| parallel | 3-partition-yes | classical auto j8 | 3-partition-yes/8 | 2.5 ms ✓ | 352 µs ✓ | 423 µs ✓ | 426 µs ✓ |
| parallel | counter | classical auto j1 | counter/16 | > 120 s | 308 µs ✓ | 63 µs ✓ | 56 µs ✓ |
| parallel | counter | classical auto j1 | counter/2 | 13 µs ✓ | 65 µs ✓ | 38 µs ✓ | 53 µs ✓ |
| parallel | counter | classical auto j1 | counter/32 |  | 2.18 s ✓ | 72 µs ✓ | 71 µs ✓ |
| parallel | counter | classical auto j1 | counter/4 | 110 µs ✓ | 66 µs ✓ | 40 µs ✓ | 53 µs ✓ |
| parallel | counter | classical auto j1 | counter/64 |  | 78.45 s ✓ | 86 µs ✓ | 79 µs ✓ |
| parallel | counter | classical auto j1 | counter/8 | 129.6 ms ✓ | 98 µs ✓ | 51 µs ✓ | 78 µs ✓ |
| parallel | counter | classical auto j16 | counter/16 | > 120 s | 1.2 ms ✓ | 91 µs ✓ | 88 µs ✓ |
| parallel | counter | classical auto j16 | counter/2 | 479 µs ✓ | 465 µs ✓ | 80 µs ✓ | 92 µs ✓ |
| parallel | counter | classical auto j16 | counter/32 |  | 700.0 ms ✓ | 102 µs ✓ | 101 µs ✓ |
| parallel | counter | classical auto j16 | counter/4 | 645 µs ✓ | 559 µs ✓ | 71 µs ✓ | 82 µs ✓ |
| parallel | counter | classical auto j16 | counter/64 |  | 23.90 s ✓ | 116 µs ✓ | 117 µs ✓ |
| parallel | counter | classical auto j16 | counter/8 | 109.3 ms ✓ | 691 µs ✓ | 81 µs ✓ | 83 µs ✓ |
| parallel | counter | classical auto j2 | counter/16 | > 120 s | 389 µs ✓ | 83 µs ✓ | 88 µs ✓ |
| parallel | counter | classical auto j2 | counter/2 | 99 µs ✓ | 109 µs ✓ | 101 µs ✓ | 104 µs ✓ |
| parallel | counter | classical auto j2 | counter/32 |  | 2.70 s ✓ | 94 µs ✓ | 101 µs ✓ |
| parallel | counter | classical auto j2 | counter/4 | 225 µs ✓ | 99 µs ✓ | 73 µs ✓ | 85 µs ✓ |
| parallel | counter | classical auto j2 | counter/64 |  | 101.90 s ✓ | 106 µs ✓ | 124 µs ✓ |
| parallel | counter | classical auto j2 | counter/8 | 156.3 ms ✓ | 138 µs ✓ | 79 µs ✓ | 93 µs ✓ |
| parallel | counter | classical auto j4 | counter/16 | > 120 s | 648 µs ✓ | 93 µs ✓ | 100 µs ✓ |
| parallel | counter | classical auto j4 | counter/2 | 126 µs ✓ | 156 µs ✓ | 82 µs ✓ | 85 µs ✓ |
| parallel | counter | classical auto j4 | counter/32 |  | 2.02 s ✓ | 96 µs ✓ | 100 µs ✓ |
| parallel | counter | classical auto j4 | counter/4 | 258 µs ✓ | 161 µs ✓ | 83 µs ✓ | 105 µs ✓ |
| parallel | counter | classical auto j4 | counter/64 |  | 76.89 s ✓ | 102 µs ✓ | 113 µs ✓ |
| parallel | counter | classical auto j4 | counter/8 | 127.2 ms ✓ | 300 µs ✓ | 78 µs ✓ | 94 µs ✓ |
| parallel | counter | classical auto j8 | counter/16 | > 120 s | 747 µs ✓ | 93 µs ✓ | 107 µs ✓ |
| parallel | counter | classical auto j8 | counter/2 | 253 µs ✓ | 243 µs ✓ | 63 µs ✓ | 88 µs ✓ |
| parallel | counter | classical auto j8 | counter/32 |  | 629.7 ms ✓ | 100 µs ✓ | 108 µs ✓ |
| parallel | counter | classical auto j8 | counter/4 | 321 µs ✓ | 274 µs ✓ | 75 µs ✓ | 82 µs ✓ |
| parallel | counter | classical auto j8 | counter/64 |  | 22.55 s ✓ | 114 µs ✓ | 123 µs ✓ |
| parallel | counter | classical auto j8 | counter/8 | 105.3 ms ✓ | 419 µs ✓ | 83 µs ✓ | 97 µs ✓ |
| parallel | counter-over | classical auto j1 | counter-over/16 | > 120 s | 403 µs ✗ | 63 µs ✗ | 57 µs ✗ |
| parallel | counter-over | classical auto j1 | counter-over/2 | 27 µs ? bound | 65 µs ✗ | 49 µs ✗ | 51 µs ✗ |
| parallel | counter-over | classical auto j1 | counter-over/32 |  | 2.28 s ? bound | 73 µs ✗ | 69 µs ✗ |
| parallel | counter-over | classical auto j1 | counter-over/4 | 500 µs ? bound | 77 µs ✗ | 40 µs ✗ | 48 µs ✗ |
| parallel | counter-over | classical auto j1 | counter-over/64 |  | 82.99 s ? bound | 70 µs ✗ | 68 µs ✗ |
| parallel | counter-over | classical auto j1 | counter-over/8 | 549.9 ms ? bound | 116 µs ✗ | 55 µs ✗ | 51 µs ✗ |
| parallel | counter-over | classical auto j16 | counter-over/16 | > 120 s | 1.2 ms ✗ | 86 µs ✗ | 92 µs ✗ |
| parallel | counter-over | classical auto j16 | counter-over/2 | 446 µs ? bound | 510 µs ✗ | 68 µs ✗ | 77 µs ✗ |
| parallel | counter-over | classical auto j16 | counter-over/32 |  | 2.87 s ? bound | 95 µs ✗ | 96 µs ✗ |
| parallel | counter-over | classical auto j16 | counter-over/4 | 981 µs ? bound | 568 µs ✗ | 76 µs ✗ | 99 µs ✗ |
| parallel | counter-over | classical auto j16 | counter-over/64 |  | 103.02 s ? bound | 98 µs ✗ | 100 µs ✗ |
| parallel | counter-over | classical auto j16 | counter-over/8 | 323.2 ms ? bound | 759 µs ✗ | 73 µs ✗ | 92 µs ✗ |
| parallel | counter-over | classical auto j2 | counter-over/16 | > 120 s | 430 µs ✗ | 84 µs ✗ | 95 µs ✗ |
| parallel | counter-over | classical auto j2 | counter-over/2 | 102 µs ? bound | 93 µs ✗ | 72 µs ✗ | 85 µs ✗ |
| parallel | counter-over | classical auto j2 | counter-over/32 |  | 3.15 s ? bound | 96 µs ✗ | 92 µs ✗ |
| parallel | counter-over | classical auto j2 | counter-over/4 | 590 µs ? bound | 128 µs ✗ | 77 µs ✗ | 76 µs ✗ |
| parallel | counter-over | classical auto j2 | counter-over/64 |  | 112.73 s ? bound | 98 µs ✗ | 126 µs ✗ |
| parallel | counter-over | classical auto j2 | counter-over/8 | 502.6 ms ? bound | 165 µs ✗ | 76 µs ✗ | 90 µs ✗ |
| parallel | counter-over | classical auto j4 | counter-over/16 | > 120 s | 731 µs ✗ | 77 µs ✗ | 106 µs ✗ |
| parallel | counter-over | classical auto j4 | counter-over/2 | 153 µs ? bound | 150 µs ✗ | 72 µs ✗ | 83 µs ✗ |
| parallel | counter-over | classical auto j4 | counter-over/32 |  | 2.52 s ? bound | 94 µs ✗ | 108 µs ✗ |
| parallel | counter-over | classical auto j4 | counter-over/4 | 504 µs ? bound | 171 µs ✗ | 68 µs ✗ | 77 µs ✗ |
| parallel | counter-over | classical auto j4 | counter-over/64 |  | 86.58 s ? bound | 88 µs ✗ | 119 µs ✗ |
| parallel | counter-over | classical auto j4 | counter-over/8 | 389.9 ms ? bound | 274 µs ✗ | 73 µs ✗ | 80 µs ✗ |
| parallel | counter-over | classical auto j8 | counter-over/16 | > 120 s | 1.2 ms ✗ | 80 µs ✗ | 91 µs ✗ |
| parallel | counter-over | classical auto j8 | counter-over/2 | 216 µs ? bound | 240 µs ✗ | 78 µs ✗ | 94 µs ✗ |
| parallel | counter-over | classical auto j8 | counter-over/32 |  | 2.72 s ? bound | 94 µs ✗ | 98 µs ✗ |
| parallel | counter-over | classical auto j8 | counter-over/4 | 730 µs ? bound | 268 µs ✗ | 68 µs ✗ | 77 µs ✗ |
| parallel | counter-over | classical auto j8 | counter-over/64 |  | 95.97 s ? bound | 91 µs ✗ | 103 µs ✗ |
| parallel | counter-over | classical auto j8 | counter-over/8 | 355.2 ms ? bound | 454 µs ✗ | 87 µs ✗ | 75 µs ✗ |
| parallel | mix | mix auto j1 | mix/10 | > 120 s | > 120 s | 62.10 s ✗ | 271.8 ms ✗ |
| parallel | mix | mix auto j1 | mix/11 | > 120 s | > 120 s | > 120 s | 293.2 ms ✗ |
| parallel | mix | mix auto j1 | mix/8 | 1.80 s ✗ | 1.56 s ✗ | 857.8 ms ✗ | 267.9 ms ✗ |
| parallel | mix | mix auto j1 | mix/9 | 17.95 s ✗ | 13.94 s ✗ | 7.39 s ✗ | 271.5 ms ✗ |
| parallel | mix | mix auto j16 | mix/10 | > 120 s | > 120 s | > 120 s | 108.11 s ✗ |
| parallel | mix | mix auto j16 | mix/11 | > 120 s | > 120 s | > 120 s | 120.01 s ✗ |
| parallel | mix | mix auto j16 | mix/8 | 1.88 s ✗ | 2.43 s ✗ | 1.61 s ✗ | 1.40 s ✗ |
| parallel | mix | mix auto j16 | mix/9 | 16.93 s ✗ | 21.18 s ✗ | 11.51 s ✗ | 9.70 s ✗ |
| parallel | mix | mix auto j2 | mix/10 | > 120 s | > 120 s | 91.83 s ✗ | 69.67 s ✗ |
| parallel | mix | mix auto j2 | mix/11 | > 120 s | > 120 s | > 120 s | 120.00 s ✗ |
| parallel | mix | mix auto j2 | mix/8 | 1.86 s ✗ | 1.87 s ✗ | 965.2 ms ✗ | 806.9 ms ✗ |
| parallel | mix | mix auto j2 | mix/9 | 17.97 s ✗ | 17.40 s ✗ | 7.05 s ✗ | 5.95 s ✗ |
| parallel | mix | mix auto j4 | mix/10 | > 120 s | > 120 s | 95.53 s ✗ | 72.29 s ✗ |
| parallel | mix | mix auto j4 | mix/11 | > 120 s | > 120 s | > 120 s | 120.01 s ✗ |
| parallel | mix | mix auto j4 | mix/8 | 1.64 s ✗ | 1.91 s ✗ | 1.05 s ✗ | 872.7 ms ✗ |
| parallel | mix | mix auto j4 | mix/9 | 16.72 s ✗ | 17.88 s ✗ | 7.90 s ✗ | 6.17 s ✗ |
| parallel | mix | mix auto j8 | mix/10 | > 120 s | > 120 s | 109.66 s ✗ | 84.78 s ✗ |
| parallel | mix | mix auto j8 | mix/11 | > 120 s | > 120 s | > 120 s | 120.01 s ✗ |
| parallel | mix | mix auto j8 | mix/8 | 1.67 s ✗ | 2.07 s ✗ | 1.48 s ✗ | 1.08 s ✗ |
| parallel | mix | mix auto j8 | mix/9 | 16.01 s ✗ | 18.57 s ✗ | 9.50 s ✗ | 7.93 s ✗ |
| parallel | partition-no | classical auto j1 | partition-no/12 |  | 4.01 s ✗ | 3.59 s ✗ | 3.53 s ✗ |
| parallel | partition-no | classical auto j1 | partition-no/14 |  | > 120 s | > 120 s | > 120 s |
| parallel | partition-no | classical auto j1 | partition-no/4 | 127.3 ms ✗ | 170 µs ✗ | 192 µs ✗ | 198 µs ✗ |
| parallel | partition-no | classical auto j1 | partition-no/5 | > 120 s | 604 µs ✗ | 568 µs ✗ | 566 µs ✗ |
| parallel | partition-no | classical auto j16 | partition-no/12 |  | 847.6 ms ✗ | 581.6 ms ✗ | 579.3 ms ✗ |
| parallel | partition-no | classical auto j16 | partition-no/14 |  | > 120 s | > 120 s | > 120 s |
| parallel | partition-no | classical auto j16 | partition-no/4 | 26.9 ms ✗ | 721 µs ✗ | 600 µs ✗ | 666 µs ✗ |
| parallel | partition-no | classical auto j16 | partition-no/5 | > 120 s | 840 µs ✗ | 850 µs ✗ | 793 µs ✗ |
| parallel | partition-no | classical auto j2 | partition-no/12 |  | 2.83 s ✗ | 2.13 s ✗ | 2.07 s ✗ |
| parallel | partition-no | classical auto j2 | partition-no/14 |  | > 120 s | > 120 s | > 120 s |
| parallel | partition-no | classical auto j2 | partition-no/4 | 73.9 ms ✗ | 272 µs ✗ | 405 µs ✗ | 411 µs ✗ |
| parallel | partition-no | classical auto j2 | partition-no/5 | > 120 s | 681 µs ✗ | 745 µs ✗ | 815 µs ✗ |
| parallel | partition-no | classical auto j4 | partition-no/12 |  | 1.73 s ✗ | 1.24 s ✗ | 1.22 s ✗ |
| parallel | partition-no | classical auto j4 | partition-no/14 |  | > 120 s | > 120 s | > 120 s |
| parallel | partition-no | classical auto j4 | partition-no/4 | 45.9 ms ✗ | 242 µs ✗ | 347 µs ✗ | 351 µs ✗ |
| parallel | partition-no | classical auto j4 | partition-no/5 | > 120 s | 518 µs ✗ | 578 µs ✗ | 664 µs ✗ |
| parallel | partition-no | classical auto j8 | partition-no/12 |  | 1.20 s ✗ | 806.0 ms ✗ | 805.1 ms ✗ |
| parallel | partition-no | classical auto j8 | partition-no/14 |  | > 120 s | > 120 s | > 120 s |
| parallel | partition-no | classical auto j8 | partition-no/4 | 29.4 ms ✗ | 392 µs ✗ | 418 µs ✗ | 439 µs ✗ |
| parallel | partition-no | classical auto j8 | partition-no/5 | > 120 s | 501 µs ✗ | 614 µs ✗ | 644 µs ✗ |
| parallel | partition-yes | classical auto j1 | partition-yes/20 |  | 535.5 ms ✓ | 396.5 ms ✓ | 389.6 ms ✓ |
| parallel | partition-yes | classical auto j1 | partition-yes/24 |  | 105.53 s ✓ | 67.45 s ✓ | 67.11 s ✓ |
| parallel | partition-yes | classical auto j1 | partition-yes/5 | 191.6 ms ✓ | 97 µs ✓ | 117 µs ✓ | 128 µs ✓ |
| parallel | partition-yes | classical auto j1 | partition-yes/6 | 12.20 s ✓ | 167 µs ✓ | 187 µs ✓ | 173 µs ✓ |
| parallel | partition-yes | classical auto j1 | partition-yes/7 | > 120 s | 318 µs ✓ | 273 µs ✓ | 277 µs ✓ |
| parallel | partition-yes | classical auto j16 | partition-yes/20 |  | 651.0 ms ✓ | 445.7 ms ✓ | 437.1 ms ✓ |
| parallel | partition-yes | classical auto j16 | partition-yes/24 |  | > 120 s | 87.42 s ✓ | 83.58 s ✓ |
| parallel | partition-yes | classical auto j16 | partition-yes/5 | 17.5 ms ✓ | 703 µs ✓ | 851 µs ✓ | 845 µs ✓ |
| parallel | partition-yes | classical auto j16 | partition-yes/6 | 1.33 s ✓ | 743 µs ✓ | 906 µs ✓ | 621 µs ✓ |
| parallel | partition-yes | classical auto j16 | partition-yes/7 | > 120 s | 849 µs ✓ | 750 µs ✓ | 854 µs ✓ |
| parallel | partition-yes | classical auto j2 | partition-yes/20 |  | 586.4 ms ✓ | 418.5 ms ✓ | 416.1 ms ✓ |
| parallel | partition-yes | classical auto j2 | partition-yes/24 |  | > 120 s | 70.81 s ✓ | 69.59 s ✓ |
| parallel | partition-yes | classical auto j2 | partition-yes/5 | 192.0 ms ✓ | 207 µs ✓ | 311 µs ✓ | 321 µs ✓ |
| parallel | partition-yes | classical auto j2 | partition-yes/6 | 11.82 s ✓ | 280 µs ✓ | 461 µs ✓ | 404 µs ✓ |
| parallel | partition-yes | classical auto j2 | partition-yes/7 | > 120 s | 447 µs ✓ | 585 µs ✓ | 514 µs ✓ |
| parallel | partition-yes | classical auto j4 | partition-yes/20 |  | 608.0 ms ✓ | 429.9 ms ✓ | 426.4 ms ✓ |
| parallel | partition-yes | classical auto j4 | partition-yes/24 |  | > 120 s | 74.75 s ✓ | 72.58 s ✓ |
| parallel | partition-yes | classical auto j4 | partition-yes/5 | 195.6 ms ✓ | 260 µs ✓ | 334 µs ✓ | 361 µs ✓ |
| parallel | partition-yes | classical auto j4 | partition-yes/6 | 8.70 s ✓ | 287 µs ✓ | 413 µs ✓ | 377 µs ✓ |
| parallel | partition-yes | classical auto j4 | partition-yes/7 | > 120 s | 498 µs ✓ | 621 µs ✓ | 562 µs ✓ |
| parallel | partition-yes | classical auto j8 | partition-yes/20 |  | 634.9 ms ✓ | 439.9 ms ✓ | 419.2 ms ✓ |
| parallel | partition-yes | classical auto j8 | partition-yes/24 |  | > 120 s | 79.07 s ✓ | 76.81 s ✓ |
| parallel | partition-yes | classical auto j8 | partition-yes/5 | 30.5 ms ✓ | 399 µs ✓ | 461 µs ✓ | 409 µs ✓ |
| parallel | partition-yes | classical auto j8 | partition-yes/6 | 4.21 s ✓ | 450 µs ✓ | 546 µs ✓ | 539 µs ✓ |
| parallel | partition-yes | classical auto j8 | partition-yes/7 | > 120 s | 638 µs ✓ | 597 µs ✓ | 625 µs ✓ |
| parallel | qbf | classical auto j1 | qbf/16#0 | 8.28 s ✓ | 13.1 ms ✓ | 7.1 ms ✓ | 7.5 ms ✓ |
| parallel | qbf | classical auto j1 | qbf/16#1 | 1.34 s ✗ | 1.9 ms ✗ | 1.8 ms ✗ | 11.8 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/16#2 | 1.89 s ✗ | 3.3 ms ✗ | 2.6 ms ✗ | 12.7 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/16#3 | 5.99 s ✓ | 10.3 ms ✓ | 8.4 ms ✓ | 6.6 ms ✓ |
| parallel | qbf | classical auto j1 | qbf/20#0 | 78.42 s ✗ | 9.4 ms ✗ | 8.4 ms ✗ | 102.5 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/20#1 | 109.45 s ✗ | 16.1 ms ✗ | 14.3 ms ✗ | 101.5 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/20#2 | > 120 s | 60.2 ms ✓ | 53.3 ms ✓ | 34.9 ms ✓ |
| parallel | qbf | classical auto j1 | qbf/20#3 | 81.26 s ✗ | 9.2 ms ✗ | 8.1 ms ✗ | 97.5 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/24#0 | > 120 s | 169.3 ms ✗ | 142.0 ms ✗ | 203.8 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/24#1 | > 120 s | 62.6 ms ✗ | 68.0 ms ✗ | 139.4 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/24#2 | > 120 s | 44.7 ms ✗ | 47.6 ms ✗ | 125.1 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/24#3 | > 120 s | 62.2 ms ✗ | 57.9 ms ✗ | 139.9 ms ✗ |
| parallel | qbf | classical auto j1 | qbf/40#0 |  | 16.61 s ✗ | 13.75 s ✗ | 10.51 s ✗ |
| parallel | qbf | classical auto j1 | qbf/40#1 |  | 18.54 s ✗ | 13.41 s ✗ | 11.71 s ✗ |
| parallel | qbf | classical auto j1 | qbf/40#2 |  | 21.36 s ✗ | 15.46 s ✗ | 13.56 s ✗ |
| parallel | qbf | classical auto j1 | qbf/40#3 |  | 16.91 s ✗ | 11.55 s ✗ | 10.44 s ✗ |
| parallel | qbf | classical auto j1 | qbf/44#0 |  | 72.46 s ✗ | 50.45 s ✗ | 44.79 s ✗ |
| parallel | qbf | classical auto j1 | qbf/44#1 |  | 72.68 s ✗ | 49.78 s ✗ | 44.51 s ✗ |
| parallel | qbf | classical auto j1 | qbf/44#2 |  | 108.46 s ✗ | 73.42 s ✗ | 67.34 s ✗ |
| parallel | qbf | classical auto j1 | qbf/44#3 |  | > 120 s | 26.33 s ? memory | 23.28 s ? memory |
| parallel | qbf | classical auto j16 | qbf/16#0 | 6.33 s ✓ | 14.9 ms ✓ | 11.3 ms ✓ | 8.0 ms ✓ |
| parallel | qbf | classical auto j16 | qbf/16#1 | 709.2 ms ✗ | 2.4 ms ✗ | 2.0 ms ✗ | 12.2 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/16#2 | 1.19 s ✗ | 3.4 ms ✗ | 3.5 ms ✗ | 13.7 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/16#3 | 5.50 s ✓ | 13.2 ms ✓ | 9.5 ms ✓ | 7.4 ms ✓ |
| parallel | qbf | classical auto j16 | qbf/20#0 | 43.42 s ✗ | 7.8 ms ✗ | 6.7 ms ✗ | 101.7 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/20#1 | 61.09 s ✗ | 12.4 ms ✗ | 13.7 ms ✗ | 101.2 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/20#2 | > 120 s | 103.9 ms ✓ | 80.4 ms ✓ | 50.7 ms ✓ |
| parallel | qbf | classical auto j16 | qbf/20#3 | 41.57 s ✗ | 7.5 ms ✗ | 5.2 ms ✗ | 102.2 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/24#0 | > 120 s | 214.4 ms ✗ | 166.2 ms ✗ | 215.9 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/24#1 | > 120 s | 60.5 ms ✗ | 45.5 ms ✗ | 131.7 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/24#2 | > 120 s | 40.2 ms ✗ | 36.5 ms ✗ | 133.1 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/24#3 | > 120 s | 58.9 ms ✗ | 43.1 ms ✗ | 135.8 ms ✗ |
| parallel | qbf | classical auto j16 | qbf/40#0 |  | 14.33 s ✗ | 7.47 s ✗ | 6.49 s ✗ |
| parallel | qbf | classical auto j16 | qbf/40#1 |  | 15.24 s ✗ | 5.53 s ? memory | 4.85 s ? memory |
| parallel | qbf | classical auto j16 | qbf/40#2 |  | 17.72 s ✗ | 5.81 s ? memory | 5.14 s ? memory |
| parallel | qbf | classical auto j16 | qbf/40#3 |  | 15.47 s ✗ | 7.51 s ✗ | 6.87 s ✗ |
| parallel | qbf | classical auto j16 | qbf/44#0 |  | 54.98 s ✗ | 8.78 s ? memory | 7.90 s ? memory |
| parallel | qbf | classical auto j16 | qbf/44#1 |  | 59.98 s ✗ | 7.42 s ? memory | 6.62 s ? memory |
| parallel | qbf | classical auto j16 | qbf/44#2 |  | 96.80 s ✗ | 5.14 s ? memory | 4.67 s ? memory |
| parallel | qbf | classical auto j16 | qbf/44#3 |  | > 120 s | 8.73 s ? memory | 7.68 s ? memory |
| parallel | qbf | classical auto j2 | qbf/16#0 | 6.37 s ✓ | 14.7 ms ✓ | 10.4 ms ✓ | 7.9 ms ✓ |
| parallel | qbf | classical auto j2 | qbf/16#1 | 707.1 ms ✗ | 2.1 ms ✗ | 2.5 ms ✗ | 11.4 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/16#2 | 1.21 s ✗ | 3.6 ms ✗ | 2.7 ms ✗ | 12.5 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/16#3 | 5.53 s ✓ | 12.3 ms ✓ | 7.3 ms ✓ | 7.4 ms ✓ |
| parallel | qbf | classical auto j2 | qbf/20#0 | 43.03 s ✗ | 9.0 ms ✗ | 7.4 ms ✗ | 100.3 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/20#1 | 61.36 s ✗ | 14.7 ms ✗ | 11.0 ms ✗ | 100.4 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/20#2 | > 120 s | 103.0 ms ✓ | 64.0 ms ✓ | 50.9 ms ✓ |
| parallel | qbf | classical auto j2 | qbf/20#3 | 40.98 s ✗ | 7.7 ms ✗ | 7.8 ms ✗ | 96.5 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/24#0 | > 120 s | 201.0 ms ✗ | 126.2 ms ✗ | 209.9 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/24#1 | > 120 s | 58.6 ms ✗ | 37.2 ms ✗ | 138.0 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/24#2 | > 120 s | 43.7 ms ✗ | 30.8 ms ✗ | 128.9 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/24#3 | > 120 s | 58.2 ms ✗ | 39.2 ms ✗ | 136.9 ms ✗ |
| parallel | qbf | classical auto j2 | qbf/40#0 |  | 14.07 s ✗ | 8.61 s ✗ | 6.81 s ✗ |
| parallel | qbf | classical auto j2 | qbf/40#1 |  | 15.30 s ✗ | 5.56 s ? memory | 5.39 s ? memory |
| parallel | qbf | classical auto j2 | qbf/40#2 |  | 17.69 s ✗ | 5.80 s ? memory | 5.10 s ? memory |
| parallel | qbf | classical auto j2 | qbf/40#3 |  | 15.63 s ✗ | 7.46 s ✗ | 6.76 s ✗ |
| parallel | qbf | classical auto j2 | qbf/44#0 |  | 54.94 s ✗ | 8.79 s ? memory | 7.70 s ? memory |
| parallel | qbf | classical auto j2 | qbf/44#1 |  | 60.47 s ✗ | 7.31 s ? memory | 6.55 s ? memory |
| parallel | qbf | classical auto j2 | qbf/44#2 |  | 97.70 s ✗ | 5.14 s ? memory | 4.62 s ? memory |
| parallel | qbf | classical auto j2 | qbf/44#3 |  | > 120 s | 8.56 s ? memory | 7.68 s ? memory |
| parallel | qbf | classical auto j4 | qbf/16#0 | 6.26 s ✓ | 14.3 ms ✓ | 9.6 ms ✓ | 7.7 ms ✓ |
| parallel | qbf | classical auto j4 | qbf/16#1 | 709.6 ms ✗ | 1.7 ms ✗ | 1.6 ms ✗ | 11.8 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/16#2 | 1.19 s ✗ | 3.7 ms ✗ | 2.9 ms ✗ | 12.5 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/16#3 | 5.52 s ✓ | 12.3 ms ✓ | 8.5 ms ✓ | 7.0 ms ✓ |
| parallel | qbf | classical auto j4 | qbf/20#0 | 43.23 s ✗ | 8.7 ms ✗ | 7.5 ms ✗ | 100.2 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/20#1 | 61.74 s ✗ | 15.0 ms ✗ | 10.0 ms ✗ | 98.5 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/20#2 | > 120 s | 100.4 ms ✓ | 68.3 ms ✓ | 45.4 ms ✓ |
| parallel | qbf | classical auto j4 | qbf/20#3 | 41.17 s ✗ | 8.0 ms ✗ | 6.5 ms ✗ | 95.0 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/24#0 | > 120 s | 194.5 ms ✗ | 128.5 ms ✗ | 210.5 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/24#1 | > 120 s | 59.7 ms ✗ | 38.4 ms ✗ | 133.1 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/24#2 | > 120 s | 37.6 ms ✗ | 29.7 ms ✗ | 130.2 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/24#3 | > 120 s | 58.6 ms ✗ | 37.5 ms ✗ | 133.3 ms ✗ |
| parallel | qbf | classical auto j4 | qbf/40#0 |  | 13.95 s ✗ | 8.46 s ✗ | 6.78 s ✗ |
| parallel | qbf | classical auto j4 | qbf/40#1 |  | 15.25 s ✗ | 5.53 s ? memory | 5.40 s ? memory |
| parallel | qbf | classical auto j4 | qbf/40#2 |  | 17.64 s ✗ | 5.84 s ? memory | 5.14 s ? memory |
| parallel | qbf | classical auto j4 | qbf/40#3 |  | 15.46 s ✗ | 7.57 s ✗ | 6.97 s ✗ |
| parallel | qbf | classical auto j4 | qbf/44#0 |  | 55.96 s ✗ | 8.71 s ? memory | 7.72 s ? memory |
| parallel | qbf | classical auto j4 | qbf/44#1 |  | 60.23 s ✗ | 7.44 s ? memory | 6.52 s ? memory |
| parallel | qbf | classical auto j4 | qbf/44#2 |  | 97.99 s ✗ | 5.13 s ? memory | 4.60 s ? memory |
| parallel | qbf | classical auto j4 | qbf/44#3 |  | > 120 s | 8.59 s ? memory | 7.76 s ? memory |
| parallel | qbf | classical auto j8 | qbf/16#0 | 6.32 s ✓ | 14.0 ms ✓ | 10.3 ms ✓ | 7.6 ms ✓ |
| parallel | qbf | classical auto j8 | qbf/16#1 | 713.0 ms ✗ | 1.9 ms ✗ | 3.0 ms ✗ | 11.7 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/16#2 | 1.21 s ✗ | 3.3 ms ✗ | 2.7 ms ✗ | 13.2 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/16#3 | 5.76 s ✓ | 12.6 ms ✓ | 11.2 ms ✓ | 7.3 ms ✓ |
| parallel | qbf | classical auto j8 | qbf/20#0 | 43.89 s ✗ | 8.6 ms ✗ | 5.8 ms ✗ | 100.7 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/20#1 | 61.97 s ✗ | 13.8 ms ✗ | 11.2 ms ✗ | 100.7 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/20#2 | > 120 s | 100.1 ms ✓ | 71.9 ms ✓ | 51.6 ms ✓ |
| parallel | qbf | classical auto j8 | qbf/20#3 | 55.61 s ✗ | 7.0 ms ✗ | 8.6 ms ✗ | 97.8 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/24#0 | > 120 s | 196.9 ms ✗ | 135.1 ms ✗ | 209.4 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/24#1 | > 120 s | 58.5 ms ✗ | 43.4 ms ✗ | 135.2 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/24#2 | > 120 s | 39.0 ms ✗ | 28.1 ms ✗ | 131.6 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/24#3 | > 120 s | 58.8 ms ✗ | 38.6 ms ✗ | 133.7 ms ✗ |
| parallel | qbf | classical auto j8 | qbf/40#0 |  | 13.93 s ✗ | 7.84 s ✗ | 6.57 s ✗ |
| parallel | qbf | classical auto j8 | qbf/40#1 |  | 15.32 s ✗ | 5.52 s ? memory | 5.42 s ? memory |
| parallel | qbf | classical auto j8 | qbf/40#2 |  | 17.64 s ✗ | 5.82 s ? memory | 5.13 s ? memory |
| parallel | qbf | classical auto j8 | qbf/40#3 |  | 15.53 s ✗ | 7.44 s ✗ | 6.86 s ✗ |
| parallel | qbf | classical auto j8 | qbf/44#0 |  | 55.39 s ✗ | 8.78 s ? memory | 7.72 s ? memory |
| parallel | qbf | classical auto j8 | qbf/44#1 |  | 59.90 s ✗ | 7.34 s ? memory | 6.53 s ? memory |
| parallel | qbf | classical auto j8 | qbf/44#2 |  | 97.53 s ✗ | 5.14 s ? memory | 4.58 s ? memory |
| parallel | qbf | classical auto j8 | qbf/44#3 |  | > 120 s | 8.70 s ? memory | 7.70 s ? memory |
| parallel | wide-m3 | classical auto j1 | wide-m3/24 | 189.7 ms ✓ | 115 µs ✓ | 135 µs ✓ | 152 µs ✓ |
| parallel | wide-m3 | classical auto j1 | wide-m3/30 | 11.12 s ✓ | 152 µs ✓ | 182 µs ✓ | 153 µs ✓ |
| parallel | wide-m3 | classical auto j1 | wide-m3/36 | > 120 s | 182 µs ✓ | 192 µs ✓ | 174 µs ✓ |
| parallel | wide-m3 | classical auto j16 | wide-m3/24 | 131.0 ms ✓ | 666 µs ✓ | 714 µs ✓ | 679 µs ✓ |
| parallel | wide-m3 | classical auto j16 | wide-m3/30 | 8.24 s ✓ | 611 µs ✓ | 807 µs ✓ | 695 µs ✓ |
| parallel | wide-m3 | classical auto j16 | wide-m3/36 | > 120 s | 687 µs ✓ | 727 µs ✓ | 727 µs ✓ |
| parallel | wide-m3 | classical auto j2 | wide-m3/24 | 191.7 ms ✓ | 198 µs ✓ | 357 µs ✓ | 301 µs ✓ |
| parallel | wide-m3 | classical auto j2 | wide-m3/30 | 10.69 s ✓ | 269 µs ✓ | 316 µs ✓ | 337 µs ✓ |
| parallel | wide-m3 | classical auto j2 | wide-m3/36 | > 120 s | 274 µs ✓ | 415 µs ✓ | 350 µs ✓ |
| parallel | wide-m3 | classical auto j4 | wide-m3/24 | 149.5 ms ✓ | 254 µs ✓ | 315 µs ✓ | 328 µs ✓ |
| parallel | wide-m3 | classical auto j4 | wide-m3/30 | 8.34 s ✓ | 306 µs ✓ | 390 µs ✓ | 362 µs ✓ |
| parallel | wide-m3 | classical auto j4 | wide-m3/36 | > 120 s | 318 µs ✓ | 405 µs ✓ | 372 µs ✓ |
| parallel | wide-m3 | classical auto j8 | wide-m3/24 | 138.8 ms ✓ | 331 µs ✓ | 489 µs ✓ | 483 µs ✓ |
| parallel | wide-m3 | classical auto j8 | wide-m3/30 | 7.87 s ✓ | 424 µs ✓ | 503 µs ✓ | 459 µs ✓ |
| parallel | wide-m3 | classical auto j8 | wide-m3/36 | > 120 s | 424 µs ✓ | 453 µs ✓ | 519 µs ✓ |
| parallel | wide-m4 | classical auto j1 | wide-m4/28 | 2.97 s ✓ | 149 µs ✓ | 141 µs ✓ | 139 µs ✓ |
| parallel | wide-m4 | classical auto j1 | wide-m4/32 | 42.28 s ✓ | 179 µs ✓ | 186 µs ✓ | 152 µs ✓ |
| parallel | wide-m4 | classical auto j1 | wide-m4/36 | > 120 s | 175 µs ✓ | 190 µs ✓ | 196 µs ✓ |
| parallel | wide-m4 | classical auto j16 | wide-m4/28 | 1.98 s ✓ | 699 µs ✓ | 727 µs ✓ | 682 µs ✓ |
| parallel | wide-m4 | classical auto j16 | wide-m4/32 | 35.49 s ✓ | 620 µs ✓ | 807 µs ✓ | 679 µs ✓ |
| parallel | wide-m4 | classical auto j16 | wide-m4/36 | > 120 s | 638 µs ✓ | 722 µs ✓ | 702 µs ✓ |
| parallel | wide-m4 | classical auto j2 | wide-m4/28 | 2.92 s ✓ | 229 µs ✓ | 326 µs ✓ | 315 µs ✓ |
| parallel | wide-m4 | classical auto j2 | wide-m4/32 | 41.95 s ✓ | 266 µs ✓ | 369 µs ✓ | 306 µs ✓ |
| parallel | wide-m4 | classical auto j2 | wide-m4/36 | > 120 s | 270 µs ✓ | 430 µs ✓ | 335 µs ✓ |
| parallel | wide-m4 | classical auto j4 | wide-m4/28 | 2.19 s ✓ | 267 µs ✓ | 342 µs ✓ | 334 µs ✓ |
| parallel | wide-m4 | classical auto j4 | wide-m4/32 | 32.67 s ✓ | 284 µs ✓ | 368 µs ✓ | 381 µs ✓ |
| parallel | wide-m4 | classical auto j4 | wide-m4/36 | > 120 s | 329 µs ✓ | 435 µs ✓ | 403 µs ✓ |
| parallel | wide-m4 | classical auto j8 | wide-m4/28 | 2.12 s ✓ | 369 µs ✓ | 487 µs ✓ | 469 µs ✓ |
| parallel | wide-m4 | classical auto j8 | wide-m4/32 | 32.75 s ✓ | 393 µs ✓ | 538 µs ✓ | 507 µs ✓ |
| parallel | wide-m4 | classical auto j8 | wide-m4/36 | > 120 s | 412 µs ✓ | 535 µs ✓ | 489 µs ✓ |
| parallel-net | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/4 | 1.59 s ✗ | 1.70 s ✗ | 1.63 s ✗ | 368 µs ✗ |
| parallel-net | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/5 | 59.71 s ✗ | 51.56 s ✗ | 48.79 s ✗ | 994 µs ✗ |
| parallel-net | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/6 | > 60 s | > 60 s | > 60 s | 1.6 ms ✗ |
| parallel-net | 3-partition-mll-no | classical net j16 | 3-partition-mll-no/4 | 142.0 ms ✗ | 139.7 ms ✗ | 138.2 ms ✗ | 814 µs ✗ |
| parallel-net | 3-partition-mll-no | classical net j16 | 3-partition-mll-no/5 | 4.60 s ✗ | 4.51 s ✗ | 4.42 s ✗ | 1.4 ms ✗ |
| parallel-net | 3-partition-mll-no | classical net j16 | 3-partition-mll-no/6 | > 60 s | > 60 s | > 60 s | 2.1 ms ✗ |
| parallel-net | 3-partition-mll-no | classical net j2 | 3-partition-mll-no/4 | 820.1 ms ✗ | 886.3 ms ✗ | 829.1 ms ✗ | 502 µs ✗ |
| parallel-net | 3-partition-mll-no | classical net j2 | 3-partition-mll-no/5 | 26.06 s ✗ | 26.63 s ✗ | 25.09 s ✗ | 1.1 ms ✗ |
| parallel-net | 3-partition-mll-no | classical net j2 | 3-partition-mll-no/6 | > 60 s | > 60 s | > 60 s | 1.2 ms ✗ |
| parallel-net | 3-partition-mll-no | classical net j4 | 3-partition-mll-no/4 | 437.4 ms ✗ | 463.9 ms ✗ | 443.0 ms ✗ | 551 µs ✗ |
| parallel-net | 3-partition-mll-no | classical net j4 | 3-partition-mll-no/5 | 13.98 s ✗ | 14.46 s ✗ | 13.41 s ✗ | 1.1 ms ✗ |
| parallel-net | 3-partition-mll-no | classical net j4 | 3-partition-mll-no/6 | > 60 s | > 60 s | > 60 s | 1.8 ms ✗ |
| parallel-net | 3-partition-mll-no | classical net j8 | 3-partition-mll-no/4 | 249.9 ms ✗ | 254.6 ms ✗ | 246.5 ms ✗ | 583 µs ✗ |
| parallel-net | 3-partition-mll-no | classical net j8 | 3-partition-mll-no/5 | 8.01 s ✗ | 7.92 s ✗ | 7.58 s ✗ | 1.2 ms ✗ |
| parallel-net | 3-partition-mll-no | classical net j8 | 3-partition-mll-no/6 | > 60 s | > 60 s | > 60 s | 1.9 ms ✗ |
| parallel-net | partition-table | classical net j1 | partition-table/1-1 | 39 µs ✓ | 40 µs ✓ | 72 µs ✓ | 73 µs ✓ |
| parallel-net | partition-table | classical net j1 | partition-table/1-1-1-1-1-7 | > 60 s | > 60 s | > 60 s | 113.1 ms ✗ |
| parallel-net | partition-table | classical net j1 | partition-table/1-1-1-5 | 7.40 s ✗ | 7.11 s ✗ | 7.13 s ✗ | 4.2 ms ✗ |
| parallel-net | partition-table | classical net j1 | partition-table/1-1-2-4 | 1.07 s ✓ | 1.04 s ✓ | 1.01 s ✓ | 1.2 ms ✓ |
| parallel-net | partition-table | classical net j1 | partition-table/1-1-4 | 51.5 ms ✗ | 47.5 ms ✗ | 46.5 ms ✗ | 782 µs ✗ |
| parallel-net | partition-table | classical net j1 | partition-table/1-2-3-4-5-5 | > 60 s | > 60 s | > 60 s | 1.04 s ✓ |
| parallel-net | partition-table | classical net j1 | partition-table/1-2-5 | 4.09 s ✗ | 3.92 s ✗ | 3.66 s ✗ | 1.7 ms ✗ |
| parallel-net | partition-table | classical net j1 | partition-table/1-3 | 478 µs ✗ | 472 µs ✗ | 488 µs ✗ | 182 µs ✗ |
| parallel-net | partition-table | classical net j1 | partition-table/2-1-1 | 782 µs ✓ | 797 µs ✓ | 802 µs ✓ | 273 µs ✓ |
| parallel-net | partition-table | classical net j1 | partition-table/2-2-1-1 | 61.9 ms ✓ | 60.3 ms ✓ | 62.3 ms ✓ | 1.3 ms ✓ |
| parallel-net | partition-table | classical net j1 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | > 60 s | > 60 s | 3.91 s ✓ |
| parallel-net | partition-table | classical net j1 | partition-table/2-3-2-1 | 4.34 s ✓ | 4.38 s ✓ | 4.24 s ✓ | 3.4 ms ✓ |
| parallel-net | partition-table | classical net j1 | partition-table/3-3-3-1 | > 60 s | > 60 s | > 60 s | 10.5 ms ✗ |
| parallel-net | partition-table | classical net j16 | partition-table/1-1 | 482 µs ✓ | 505 µs ✓ | 508 µs ✓ | 513 µs ✓ |
| parallel-net | partition-table | classical net j16 | partition-table/1-1-1-1-1-7 | > 60 s | > 60 s | > 60 s | 12.1 ms ✗ |
| parallel-net | partition-table | classical net j16 | partition-table/1-1-1-5 | 611.3 ms ✗ | 598.5 ms ✗ | 649.9 ms ✗ | 4.3 ms ✗ |
| parallel-net | partition-table | classical net j16 | partition-table/1-1-2-4 | 76.5 ms ✓ | 72.5 ms ✓ | 74.5 ms ✓ | 5.8 ms ✓ |
| parallel-net | partition-table | classical net j16 | partition-table/1-1-4 | 5.2 ms ✗ | 5.3 ms ✗ | 4.7 ms ✗ | 1.2 ms ✗ |
| parallel-net | partition-table | classical net j16 | partition-table/1-2-3-4-5-5 | > 60 s | > 60 s | > 60 s | 6.7 ms ✓ |
| parallel-net | partition-table | classical net j16 | partition-table/1-2-5 | 309.5 ms ✗ | 297.9 ms ✗ | 291.2 ms ✗ | 2.2 ms ✗ |
| parallel-net | partition-table | classical net j16 | partition-table/1-3 | 1.6 ms ✗ | 1.6 ms ✗ | 965 µs ✗ | 587 µs ✗ |
| parallel-net | partition-table | classical net j16 | partition-table/2-1-1 | 4.4 ms ✓ | 4.4 ms ✓ | 1.4 ms ✓ | 767 µs ✓ |
| parallel-net | partition-table | classical net j16 | partition-table/2-2-1-1 | 7.7 ms ✓ | 7.1 ms ✓ | 6.0 ms ✓ | 2.5 ms ✓ |
| parallel-net | partition-table | classical net j16 | partition-table/2-2-2-2-2-2-9-1 | 955.4 ms ✓ | 835.8 ms ✓ | 977.2 ms ✓ | 3.1 ms ✓ |
| parallel-net | partition-table | classical net j16 | partition-table/2-3-2-1 | 337.5 ms ✓ | 316.1 ms ✓ | 336.8 ms ✓ | 5.3 ms ✓ |
| parallel-net | partition-table | classical net j16 | partition-table/3-3-3-1 | > 60 s | > 60 s | > 60 s | 4.4 ms ✗ |
| parallel-net | partition-table | classical net j2 | partition-table/1-1 | 203 µs ✓ | 245 µs ✓ | 180 µs ✓ | 205 µs ✓ |
| parallel-net | partition-table | classical net j2 | partition-table/1-1-1-1-1-7 | > 60 s | > 60 s | > 60 s | 61.4 ms ✗ |
| parallel-net | partition-table | classical net j2 | partition-table/1-1-1-5 | 3.91 s ✗ | 3.63 s ✗ | 3.49 s ✗ | 2.6 ms ✗ |
| parallel-net | partition-table | classical net j2 | partition-table/1-1-2-4 | 554.8 ms ✓ | 522.5 ms ✓ | 496.8 ms ✓ | 1.2 ms ✓ |
| parallel-net | partition-table | classical net j2 | partition-table/1-1-4 | 25.8 ms ✗ | 25.6 ms ✗ | 24.7 ms ✗ | 851 µs ✗ |
| parallel-net | partition-table | classical net j2 | partition-table/1-2-3-4-5-5 | > 60 s | > 60 s | > 60 s | 122.6 ms ✓ |
| parallel-net | partition-table | classical net j2 | partition-table/1-2-5 | 2.04 s ✗ | 1.92 s ✗ | 1.82 s ✗ | 1.2 ms ✗ |
| parallel-net | partition-table | classical net j2 | partition-table/1-3 | 557 µs ✗ | 554 µs ✗ | 584 µs ✗ | 292 µs ✗ |
| parallel-net | partition-table | classical net j2 | partition-table/2-1-1 | 930 µs ✓ | 931 µs ✓ | 726 µs ✓ | 471 µs ✓ |
| parallel-net | partition-table | classical net j2 | partition-table/2-2-1-1 | 31.6 ms ✓ | 30.7 ms ✓ | 30.6 ms ✓ | 1.0 ms ✓ |
| parallel-net | partition-table | classical net j2 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | > 60 s | > 60 s | 1.37 s ✓ |
| parallel-net | partition-table | classical net j2 | partition-table/2-3-2-1 | 2.27 s ✓ | 2.13 s ✓ | 2.18 s ✓ | 2.0 ms ✓ |
| parallel-net | partition-table | classical net j2 | partition-table/3-3-3-1 | > 60 s | > 60 s | > 60 s | 5.8 ms ✗ |
| parallel-net | partition-table | classical net j4 | partition-table/1-1 | 239 µs ✓ | 253 µs ✓ | 199 µs ✓ | 232 µs ✓ |
| parallel-net | partition-table | classical net j4 | partition-table/1-1-1-1-1-7 | > 60 s | > 60 s | > 60 s | 34.1 ms ✗ |
| parallel-net | partition-table | classical net j4 | partition-table/1-1-1-5 | 2.02 s ✗ | 1.92 s ✗ | 1.90 s ✗ | 1.7 ms ✗ |
| parallel-net | partition-table | classical net j4 | partition-table/1-1-2-4 | 281.9 ms ✓ | 265.7 ms ✓ | 251.8 ms ✓ | 802 µs ✓ |
| parallel-net | partition-table | classical net j4 | partition-table/1-1-4 | 13.5 ms ✗ | 13.3 ms ✗ | 12.9 ms ✗ | 919 µs ✗ |
| parallel-net | partition-table | classical net j4 | partition-table/1-2-3-4-5-5 | 3.5 ms ✓ | 3.4 ms ✓ | 2.0 ms ✓ | 1.6 ms ✓ |
| parallel-net | partition-table | classical net j4 | partition-table/1-2-5 | 1.02 s ✗ | 962.7 ms ✗ | 919.5 ms ✗ | 1.9 ms ✗ |
| parallel-net | partition-table | classical net j4 | partition-table/1-3 | 1.3 ms ✗ | 1.3 ms ✗ | 674 µs ✗ | 311 µs ✗ |
| parallel-net | partition-table | classical net j4 | partition-table/2-1-1 | 4.1 ms ✓ | 4.2 ms ✓ | 1.2 ms ✓ | 493 µs ✓ |
| parallel-net | partition-table | classical net j4 | partition-table/2-2-1-1 | 17.3 ms ✓ | 16.7 ms ✓ | 16.6 ms ✓ | 1.2 ms ✓ |
| parallel-net | partition-table | classical net j4 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | > 60 s | > 60 s | 1.20 s ✓ |
| parallel-net | partition-table | classical net j4 | partition-table/2-3-2-1 | 1.19 s ✓ | 1.06 s ✓ | 1.06 s ✓ | 1.4 ms ✓ |
| parallel-net | partition-table | classical net j4 | partition-table/3-3-3-1 | > 60 s | > 60 s | > 60 s | 3.4 ms ✗ |
| parallel-net | partition-table | classical net j8 | partition-table/1-1 | 384 µs ✓ | 312 µs ✓ | 346 µs ✓ | 273 µs ✓ |
| parallel-net | partition-table | classical net j8 | partition-table/1-1-1-1-1-7 | > 60 s | > 60 s | > 60 s | 20.3 ms ✗ |
| parallel-net | partition-table | classical net j8 | partition-table/1-1-1-5 | 1.11 s ✗ | 1.08 s ✗ | 1.08 s ✗ | 2.7 ms ✗ |
| parallel-net | partition-table | classical net j8 | partition-table/1-1-2-4 | 149.5 ms ✓ | 143.8 ms ✓ | 141.7 ms ✓ | 2.3 ms ✓ |
| parallel-net | partition-table | classical net j8 | partition-table/1-1-4 | 8.4 ms ✗ | 7.9 ms ✗ | 7.6 ms ✗ | 961 µs ✗ |
| parallel-net | partition-table | classical net j8 | partition-table/1-2-3-4-5-5 | > 60 s | > 60 s | > 60 s | 54.2 ms ✓ |
| parallel-net | partition-table | classical net j8 | partition-table/1-2-5 | 582.3 ms ✗ | 545.7 ms ✗ | 537.3 ms ✗ | 2.0 ms ✗ |
| parallel-net | partition-table | classical net j8 | partition-table/1-3 | 1.4 ms ✗ | 1.4 ms ✗ | 724 µs ✗ | 394 µs ✗ |
| parallel-net | partition-table | classical net j8 | partition-table/2-1-1 | 4.3 ms ✓ | 4.3 ms ✓ | 1.3 ms ✓ | 597 µs ✓ |
| parallel-net | partition-table | classical net j8 | partition-table/2-2-1-1 | 9.3 ms ✓ | 9.4 ms ✓ | 8.7 ms ✓ | 2.3 ms ✓ |
| parallel-net | partition-table | classical net j8 | partition-table/2-2-2-2-2-2-9-1 | 861.2 ms ✓ | 897.2 ms ✓ | 815.1 ms ✓ | 1.6 ms ✓ |
| parallel-net | partition-table | classical net j8 | partition-table/2-3-2-1 | 654.3 ms ✓ | 611.1 ms ✓ | 626.7 ms ✓ | 2.3 ms ✓ |
| parallel-net | partition-table | classical net j8 | partition-table/3-3-3-1 | > 60 s | > 60 s | > 60 s | 4.7 ms ✗ |
| period-1 | 3-partition-mll-no | classical net j1 period 1 | 3-partition-mll-no/4 | 1.80 s ✗ | 1.85 s ✗ | 1.73 s ✗ | 259 µs ✗ |
| period-1 | 3-partition-mll-no | classical net j1 period 1 | 3-partition-mll-no/5 | 56.27 s ✗ | 56.38 s ✗ | 55.13 s ✗ | 681 µs ✗ |
| period-1 | partition-no | classical net j1 period 1 | partition-no/3 | 3.49 s ✗ | 3.49 s ✗ | 3.32 s ✗ | 1.5 ms ✗ |
| period-1 | partition-no | classical net j1 period 1 | partition-no/4 | > 60 s | > 60 s | > 60 s | 9.3 ms ✗ |
| period-1 | partition-yes | classical net j1 period 1 | partition-yes/4 | 2.76 s ✓ | 2.75 s ✓ | 2.69 s ✓ | 1.7 ms ✓ |
| period-1 | wide-m1 | classical net j1 period 1 | wide-m1/2048 | 1.35 s ✓ | 1.35 s ✓ | 1.22 s ✓ | 1.26 s ✓ |
| period-1 | wide-m1 | classical net j1 period 1 | wide-m1/256 | 19.2 ms ✓ | 19.5 ms ✓ | 17.9 ms ✓ | 18.5 ms ✓ |
| period-1 | wide-m2 | classical net j1 period 1 | wide-m2/2048 | 1.35 s ✓ | 1.38 s ✓ | 1.23 s ✓ | 1.25 s ✓ |
| period-1 | wide-m2 | classical net j1 period 1 | wide-m2/256 | 19.3 ms ✓ | 19.6 ms ✓ | 18.5 ms ✓ | 18.3 ms ✓ |
| period-16 | 3-partition-mll-no | classical net j1 period 16 | 3-partition-mll-no/4 | 22.04 s ✗ | 20.98 s ✗ | 20.95 s ✗ | 259 µs ✗ |
| period-16 | 3-partition-mll-no | classical net j1 period 16 | 3-partition-mll-no/5 | > 60 s | > 60 s | > 60 s | 1.2 ms ✗ |
| period-16 | partition-no | classical net j1 period 16 | partition-no/3 | > 60 s | > 60 s | > 60 s | 2.2 ms ✗ |
| period-16 | partition-no | classical net j1 period 16 | partition-no/4 | > 60 s | > 60 s | > 60 s | 23.5 ms ✗ |
| period-16 | partition-yes | classical net j1 period 16 | partition-yes/4 | > 60 s | > 60 s | > 60 s | 2.7 ms ✓ |
| period-16 | wide-m1 | classical net j1 period 16 | wide-m1/2048 | 459.5 ms ✓ | 434.9 ms ✓ | 388.2 ms ✓ | 427.8 ms ✓ |
| period-16 | wide-m1 | classical net j1 period 16 | wide-m1/256 | 6.5 ms ✓ | 6.2 ms ✓ | 5.9 ms ✓ | 6.4 ms ✓ |
| period-16 | wide-m2 | classical net j1 period 16 | wide-m2/2048 | 442.5 ms ✓ | 429.6 ms ✓ | 405.9 ms ✓ | 420.0 ms ✓ |
| period-16 | wide-m2 | classical net j1 period 16 | wide-m2/256 | 6.6 ms ✓ | 6.6 ms ✓ | 6.2 ms ✓ | 6.5 ms ✓ |
| period-2 | 3-partition-mll-no | classical net j1 period 2 | 3-partition-mll-no/4 | 1.43 s ✗ | 1.45 s ✗ | 1.38 s ✗ | 235 µs ✗ |
| period-2 | 3-partition-mll-no | classical net j1 period 2 | 3-partition-mll-no/5 | 51.40 s ✗ | 51.04 s ✗ | 49.10 s ✗ | 695 µs ✗ |
| period-2 | partition-no | classical net j1 period 2 | partition-no/3 | 8.11 s ✗ | 7.82 s ✗ | 7.45 s ✗ | 1.2 ms ✗ |
| period-2 | partition-no | classical net j1 period 2 | partition-no/4 | > 60 s | > 60 s | > 60 s | 7.7 ms ✗ |
| period-2 | partition-yes | classical net j1 period 2 | partition-yes/4 | 6.11 s ✓ | 6.00 s ✓ | 5.72 s ✓ | 1.3 ms ✓ |
| period-2 | wide-m1 | classical net j1 period 2 | wide-m1/2048 | 856.4 ms ✓ | 860.9 ms ✓ | 818.1 ms ✓ | 809.0 ms ✓ |
| period-2 | wide-m1 | classical net j1 period 2 | wide-m1/256 | 12.2 ms ✓ | 12.4 ms ✓ | 12.2 ms ✓ | 12.0 ms ✓ |
| period-2 | wide-m2 | classical net j1 period 2 | wide-m2/2048 | 840.4 ms ✓ | 855.4 ms ✓ | 803.6 ms ✓ | 805.6 ms ✓ |
| period-2 | wide-m2 | classical net j1 period 2 | wide-m2/256 | 12.4 ms ✓ | 12.4 ms ✓ | 12.0 ms ✓ | 12.2 ms ✓ |
| period-8 | 3-partition-mll-no | classical net j1 period 8 | 3-partition-mll-no/4 | 5.01 s ✗ | 4.96 s ✗ | 4.78 s ✗ | 246 µs ✗ |
| period-8 | 3-partition-mll-no | classical net j1 period 8 | 3-partition-mll-no/5 | > 60 s | > 60 s | > 60 s | 709 µs ✗ |
| period-8 | partition-no | classical net j1 period 8 | partition-no/3 | > 60 s | > 60 s | > 60 s | 1.4 ms ✗ |
| period-8 | partition-no | classical net j1 period 8 | partition-no/4 | > 60 s | > 60 s | > 60 s | 22.4 ms ✗ |
| period-8 | partition-yes | classical net j1 period 8 | partition-yes/4 | > 60 s | > 60 s | > 60 s | 5.0 ms ✓ |
| period-8 | wide-m1 | classical net j1 period 8 | wide-m1/2048 | 520.8 ms ✓ | 494.5 ms ✓ | 448.2 ms ✓ | 481.0 ms ✓ |
| period-8 | wide-m1 | classical net j1 period 8 | wide-m1/256 | 7.3 ms ✓ | 7.2 ms ✓ | 7.0 ms ✓ | 7.3 ms ✓ |
| period-8 | wide-m2 | classical net j1 period 8 | wide-m2/2048 | 503.9 ms ✓ | 491.0 ms ✓ | 463.3 ms ✓ | 475.0 ms ✓ |
| period-8 | wide-m2 | classical net j1 period 8 | wide-m2/256 | 7.6 ms ✓ | 7.2 ms ✓ | 7.0 ms ✓ | 7.2 ms ✓ |

## Against 2026-10-17

### Verdicts that differ

None.

### Counters that differ

Of the 16733 problems both decide alike on one thread, 264 differ in nodes, splits, memo_hits, memo_entries, links, tests.

### Decided by before, not by after

The rows of `parallel`, `parallel-net`, `lltp-all-cores` and `lltp-default` in the fourth baseline are its parallel stage, 16 threads beside foreign load: not comparable with the earlier baselines' (above, "Where the fourth baseline ran differently").

| file | family | configuration | problem | before | after |
|---|---|---|---|--:|--:|
| lltp-default | ILL/ILLTP-SYJ-01 | intuitionistic auto j16 pool after 0.1s | SYJ205+1.008.p | 1.68 s ✓ | > 2 s |
| lltp-default | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 pool after 0.1s | SYJ212+1.012.p | 163.3 ms ✗ | > 2 s |
| lltp-default | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 pool after 0.1s | SYJ212+1.013.p | 532.2 ms ✗ | > 2 s |
| lltp-default | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 pool after 0.1s | SYJ212+1.014.p | 1.07 s ✗ | > 2 s |
| lltp-forward | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 bias factors | SYJ205+1.009.p | 3.46 s ✓ | > 5 s |

### By group

The rows of `parallel`, `parallel-net`, `lltp-all-cores` and `lltp-default` in the fourth baseline are its parallel stage, 16 threads beside foreign load: not comparable with the earlier baselines' (above, "Where the fourth baseline ran differently").

Problems in both; decided (late: after the time limit); decided by one and not the other; where the undecided ended (`time` the time limit or `killed` past it, `bound` the copy bound, `depth` the recursion limit, `memory` the memory limit, `index` an index past its width, `wide` a context too wide to split, `crash` out of memory); the median ratio of after's time to before's on the problems both decide within the limit (how many); and the problems only after has.

| file | family | configuration | problems | before decided | after decided | only before | only after | before ended | after ended | after/before | new |
|---|---|---|--:|--:|--:|--:|--:|---|---|--:|--:|
| engines | 3-partition-mll-no | classical focus j1 | 4 | 4 | 4 | 0 | 0 | – | – | 0.96× (4) | 0 |
| engines | 3-partition-mll-no | classical net j1 | 4 | 2 | 4 | 0 | 2 | 2 time | – | 1.4e-4× (2) | 0 |
| engines | 3-partition-mll-yes | classical focus j1 | 4 | 4 | 4 | 0 | 0 | – | – | 0.95× (4) | 0 |
| engines | 3-partition-mll-yes | classical net j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.23× (4) | 0 |
| engines | additive | classical focus j1 | 4 | 4 | 4 | 0 | 0 | – | – | 0.85× (4) | 0 |
| engines | cancellation | classical focus j1 | 1 | 1 | 1 | 0 | 0 | – | – | 0.93× (1) | 0 |
| engines | partition-no | classical focus j1 | 4 | 4 | 4 | 0 | 0 | – | – | 0.98× (4) | 0 |
| engines | partition-no | classical net j1 | 4 | 1 | 2 | 0 | 1 | 3 time | 2 time | 4.0e-4× (1) | 0 |
| engines | partition-table | classical focus j1 | 13 | 13 | 13 | 0 | 0 | – | – | 0.90× (13) | 0 |
| engines | partition-table | classical net j1 | 13 | 9 | 13 | 0 | 4 | 4 time | – | 1.4e-2× (9) | 0 |
| engines | partition-yes | classical focus j1 | 5 | 5 | 5 | 0 | 0 | – | – | 0.91× (5) | 0 |
| engines | partition-yes | classical net j1 | 5 | 1 | 3 | 0 | 2 | 4 time | 2 time | 6.2e-4× (1) | 0 |
| engines | wide-m1 | classical focus j1 | 6 | 6 | 6 | 0 | 0 | – | – | 0.96× (6) | 0 |
| engines | wide-m1 | classical net j1 | 6 | 6 | 6 | 0 | 0 | – | – | 1.03× (6) | 0 |
| engines | wide-m2 | classical focus j1 | 6 | 6 | 6 | 0 | 0 | – | – | 0.95× (6) | 0 |
| engines | wide-m2 | classical net j1 | 6 | 6 | 6 | 0 | 0 | – | – | 1.06× (6) | 0 |
| engines | wide-m3 | classical focus j1 | 5 | 5 | 5 | 0 | 0 | – | – | 0.85× (5) | 0 |
| engines | wide-m3 | classical net j1 | 5 | 5 | 5 | 0 | 0 | – | – | 1.01× (5) | 0 |
| engines | wide-m4 | classical focus j1 | 5 | 5 | 5 | 0 | 0 | – | – | 0.89× (5) | 0 |
| engines | wide-m4 | classical net j1 | 5 | 5 | 5 | 0 | 0 | – | – | 1.03× (5) | 0 |
| families | 3-partition-mll-no | classical auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.15× (4) | 0 |
| families | 3-partition-mll-yes | classical auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 0.98× (4) | 0 |
| families | 3-partition-no | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 0.92× (2) | 0 |
| families | 3-partition-yes | classical auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 0.97× (4) | 0 |
| families | additive | classical auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.06× (4) | 0 |
| families | chain | classical auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.00× (4) | 0 |
| families | chain-fo | classical auto j1 | 0 | | | | | | | | 8 |
| families | contraction | intuitionistic auto j1 | 0 | | | | | | | | 6 |
| families | counter | classical auto j1 | 6 | 6 | 6 | 0 | 0 | – | – | 0.96× (6) | 0 |
| families | counter-over | classical auto j1 | 6 | 6 | 6 | 0 | 0 | – | – | 1.09× (6) | 0 |
| families | duplicate | classical auto j1 | 0 | | | | | | | | 3 |
| families | duplicate | intuitionistic auto j1 | 0 | | | | | | | | 3 |
| families | euler | intuitionistic auto j1 | 0 | | | | | | | | 6 |
| families | growing | classical auto j1 | 4 | 0 | 4 | 0 | 4 | 3 bound, 1 depth | – | – | 0 |
| families | lambek-fo | intuitionistic auto j1 | 0 | | | | | | | | 20 |
| families | mix | mix auto j1 | 6 | 5 | 6 | 0 | 1 | 1 time | – | 0.30× (5) | 0 |
| families | partition-no | classical auto j1 | 7 | 5 | 5 | 0 | 0 | 2 time | 2 time | 0.94× (5) | 0 |
| families | partition-yes | classical auto j1 | 9 | 8 | 8 | 0 | 0 | 1 time | 1 time | 0.92× (8) | 0 |
| families | pigeonhole | intuitionistic-affine auto j1 | 0 | | | | | | | | 6 |
| families | qbf | classical auto j1 | 36 | 34 | 34 | 0 | 0 | 2 memory | 2 memory | 0.97× (34) | 0 |
| families | reach-fo | intuitionistic auto j1 | 0 | | | | | | | | 4 |
| families | wide-m1 | classical auto j1 | 6 | 6 | 6 | 0 | 0 | – | – | 0.60× (6) | 0 |
| families | wide-m2 | classical auto j1 | 6 | 6 | 6 | 0 | 0 | – | – | 0.61× (6) | 0 |
| families | wide-m3 | classical auto j1 | 7 | 7 | 7 | 0 | 0 | – | – | 0.94× (7) | 0 |
| families | wide-m4 | classical auto j1 | 8 | 8 | 8 | 0 | 0 | – | – | 0.95× (8) | 0 |
| families | witness-fo | intuitionistic auto j1 | 0 | | | | | | | | 16 |
| intuitionistic | 3-partition-no | intuitionistic auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 1.01× (1) | 0 |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 0.97× (4) | 0 |
| intuitionistic | chain | intuitionistic auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.04× (4) | 0 |
| intuitionistic | counter | intuitionistic auto j1 | 6 | 6 | 6 | 0 | 0 | – | – | 1.07× (6) | 0 |
| intuitionistic | counter-over | intuitionistic auto j1 | 6 | 6 | 6 | 0 | 0 | – | – | 1.11× (6) | 0 |
| lltp-all-cores | ILL/ILLTP-SYJ-01 | intuitionistic auto j16 | 31 | 0 | 0 | 0 | 0 | 15 bound, 12 depth, 4 memory | 15 bound, 12 depth, 4 memory | – | 0 |
| lltp-all-cores | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 | 31 | 0 | 0 | 0 | 0 | 16 bound, 12 depth, 3 memory | 16 bound, 12 depth, 3 memory | – | 0 |
| lltp-all-cores | ILL/ILLTP-SYJ-cbv | intuitionistic auto j16 | 31 | 2 | 2 | 0 | 0 | 2 time, 6 bound, 21 memory | 2 time, 6 bound, 21 memory | 1.11× (2) | 0 |
| lltp-all-cores | ILL/petri-nets/MCC | intuitionistic auto j16 | 1009 | 991 | 991 | 0 | 0 | 17 time, 1 memory | 17 time, 1 memory | 1.01× (991) | 0 |
| lltp-all-cores-generous | ILL/ILLTP-SYJ-01 | intuitionistic auto j16 | 5 | 0 | 0 | 0 | 0 | 1 bound, 4 memory | 1 bound, 4 memory | – | 0 |
| lltp-all-cores-generous | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 | 4 | 0 | 0 | 0 | 0 | 1 bound, 3 memory | 1 bound, 3 memory | – | 0 |
| lltp-all-cores-generous | ILL/ILLTP-SYJ-cbv | intuitionistic auto j16 | 3 | 0 | 0 | 0 | 0 | 3 memory | 3 memory | – | 0 |
| lltp-classical | CLL/Non-theorems | classical auto j1 | 3 | 3 | 3 | 0 | 0 | – | – | 0.65× (3) | 0 |
| lltp-classical | CLL/misc | classical auto j1 | 14 | 8 | 8 | 0 | 0 | 6 bound | 6 bound | 0.63× (8) | 0 |
| lltp-classical | ILL/ILLTP-LCL-01 | classical auto j1 | 2 | 0 | 1 | 0 | 1 | 2 bound | 1 bound | – | 0 |
| lltp-classical | ILL/ILLTP-LCL-cbn | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 0.76× (2) | 0 |
| lltp-classical | ILL/ILLTP-LCL-cbv | classical auto j1 | 2 | 0 | 1 | 0 | 1 | 2 bound | 1 bound | – | 0 |
| lltp-classical | ILL/ILLTP-SYJ-01 | classical auto j1 | 252 | 11 | 34 | 0 | 23 | 2 time, 215 bound, 22 depth, 2 memory | 2 time, 192 bound, 22 depth, 2 memory | 0.80× (11) | 0 |
| lltp-classical | ILL/ILLTP-SYJ-cbn | classical auto j1 | 252 | 35 | 38 | 0 | 3 | 3 time, 190 bound, 22 depth, 2 memory | 3 time, 187 bound, 22 depth, 2 memory | 0.88× (35) | 0 |
| lltp-classical | ILL/ILLTP-SYJ-cbv | classical auto j1 | 252 | 40 | 43 | 0 | 3 | 22 time, 178 bound, 10 depth, 2 memory | 21 time, 176 bound, 10 depth, 2 memory | 1.09× (40) | 0 |
| lltp-classical | ILL/ILLTP-SYN-01 | classical auto j1 | 19 | 13 | 14 | 0 | 1 | 6 bound | 5 bound | 1.16× (13) | 0 |
| lltp-classical | ILL/ILLTP-SYN-cbn | classical auto j1 | 19 | 18 | 18 | 0 | 0 | 1 bound | 1 bound | 1.07× (18) | 0 |
| lltp-classical | ILL/ILLTP-SYN-cbv | classical auto j1 | 19 | 18 | 18 | 0 | 0 | 1 bound | 1 bound | 1.11× (18) | 0 |
| lltp-classical | ILL/KLE-01 | classical auto j1 | 88 | 51 | 58 | 0 | 7 | 37 bound | 30 bound | 1.00× (51) | 0 |
| lltp-classical | ILL/KLE-IMP-CONJ | classical auto j1 | 222 | 162 | 162 | 0 | 0 | 60 bound | 60 bound | 1.06× (162) | 0 |
| lltp-classical | ILL/KLE-IMP-CONJ/ALT | classical auto j1 | 27 | 27 | 27 | 0 | 0 | – | – | 1.03× (27) | 0 |
| lltp-classical | ILL/KLE-IMP-CONJ/NON-THEOREMS | classical auto j1 | 22 | 22 | 22 | 0 | 0 | – | – | 1.39× (22) | 0 |
| lltp-classical | ILL/KLE-cbn | classical auto j1 | 88 | 69 | 76 | 0 | 7 | 19 bound | 12 bound | 1.02× (69) | 0 |
| lltp-classical | ILL/KLE-cbv | classical auto j1 | 88 | 68 | 75 | 0 | 7 | 20 bound | 13 bound | 1.02× (68) | 0 |
| lltp-classical | ILL/Non-theorems | classical auto j1 | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
| lltp-classical | ILL/misc | classical auto j1 | 3 | 3 | 3 | 0 | 0 | – | – | 0.94× (3) | 0 |
| lltp-classical | ILL/petri-nets/MCC | classical auto j1 | 3137 | 3071 | 3071 | 0 | 0 | 58 time, 8 memory | 58 time, 8 memory | 0.97× (3071) | 0 |
| lltp-classical-generous | ILL/ILLTP-SYJ-01 | classical auto j1 | 4 | 0 | 0 | 0 | 0 | 1 time, 1 bound, 2 memory | 1 time, 1 bound, 2 memory | – | 0 |
| lltp-classical-generous | ILL/ILLTP-SYJ-cbn | classical auto j1 | 2 | 0 | 0 | 0 | 0 | 2 memory | 2 memory | – | 0 |
| lltp-classical-generous | ILL/ILLTP-SYJ-cbv | classical auto j1 | 3 | 0 | 0 | 0 | 0 | 1 time, 2 memory | 1 time, 2 memory | – | 0 |
| lltp-classical-generous | ILL/petri-nets/MCC | classical auto j1 | 25 | 25 | 25 | 0 | 0 | – | – | 1.38× (25) | 0 |
| lltp-copies-10 | ILL/ILLTP-LCL-01 | intuitionistic auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.23× (2) | 0 |
| lltp-copies-10 | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.12× (2) | 0 |
| lltp-copies-10 | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 0.96× (2) | 0 |
| lltp-copies-10 | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 209 | 51 | 94 | 0 | 43 | 85 time, 73 bound | 61 time, 54 bound | 0.98× (51) | 0 |
| lltp-copies-10 | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 209 | 88 | 124 | 0 | 36 | 78 time, 43 bound | 49 time, 36 bound | 0.95× (88) | 0 |
| lltp-copies-10 | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 209 | 93 | 123 | 0 | 30 | 79 time, 37 bound | 56 time, 30 bound | 0.99× (93) | 0 |
| lltp-copies-10 | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | 8 | 7 | 7 | 0 | 0 | 1 bound | 1 bound | 1.00× (7) | 0 |
| lltp-copies-10 | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 | 8 | 8 | 8 | 0 | 0 | – | – | 0.98× (8) | 0 |
| lltp-copies-10 | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 | 8 | 8 | 8 | 0 | 0 | – | – | 1.04× (8) | 0 |
| lltp-copies-10 | ILL/KLE-01 | intuitionistic auto j1 | 38 | 31 | 37 | 0 | 6 | 7 bound | 1 bound | 1.00× (31) | 0 |
| lltp-copies-10 | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | 60 | 60 | 60 | 0 | 0 | – | – | 0.99× (60) | 0 |
| lltp-copies-10 | ILL/KLE-cbn | intuitionistic auto j1 | 38 | 36 | 38 | 0 | 2 | 2 bound | – | 0.93× (36) | 0 |
| lltp-copies-10 | ILL/KLE-cbv | intuitionistic auto j1 | 38 | 33 | 37 | 0 | 4 | 5 bound | 1 bound | 1.05× (33) | 0 |
| lltp-copies-10 | ILL/Non-theorems | intuitionistic auto j1 | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
| lltp-copies-10 | ILL/petri-nets/MCC | intuitionistic auto j1 | 171 | 171 | 171 | 0 | 0 | – | – | 0.99× (171) | 0 |
| lltp-default | ILL/ILLTP-LCL-01 | intuitionistic auto j16 pool after 0.1s | 2 | 2 | 2 | 0 | 0 | – | – | 1.35× (2) | 0 |
| lltp-default | ILL/ILLTP-LCL-cbn | intuitionistic auto j16 pool after 0.1s | 2 | 2 | 2 | 0 | 0 | – | – | 1.16× (2) | 0 |
| lltp-default | ILL/ILLTP-LCL-cbv | intuitionistic auto j16 pool after 0.1s | 2 | 2 | 2 | 0 | 0 | – | – | 1.17× (2) | 0 |
| lltp-default | ILL/ILLTP-SYJ-01 | intuitionistic auto j16 pool after 0.1s | 252 | 101 | 137 | 1 | 37 | 123 time, 22 depth, 6 memory | 87 time, 22 depth, 6 memory | 1.01× (100) | 0 |
| lltp-default | ILL/ILLTP-SYJ-cbn | intuitionistic auto j16 pool after 0.1s | 252 | 137 | 164 | 3 | 30 | 88 time, 22 depth, 5 memory | 61 time, 22 depth, 5 memory | 1.03× (134) | 0 |
| lltp-default | ILL/ILLTP-SYJ-cbv | intuitionistic auto j16 pool after 0.1s | 252 | 117 | 154 | 0 | 37 | 110 time, 10 depth, 15 memory | 77 time, 10 depth, 11 memory | 0.99× (117) | 0 |
| lltp-default | ILL/ILLTP-SYN-01 | intuitionistic auto j16 pool after 0.1s | 19 | 19 | 19 | 0 | 0 | – | – | 1.16× (19) | 0 |
| lltp-default | ILL/ILLTP-SYN-cbn | intuitionistic auto j16 pool after 0.1s | 19 | 19 | 19 | 0 | 0 | – | – | 1.32× (19) | 0 |
| lltp-default | ILL/ILLTP-SYN-cbv | intuitionistic auto j16 pool after 0.1s | 19 | 19 | 19 | 0 | 0 | – | – | 1.34× (19) | 0 |
| lltp-default | ILL/KLE-01 | intuitionistic auto j16 pool after 0.1s | 88 | 87 | 88 (1 late) | 0 | 1 | 1 time | – | 1.25× (87) | 0 |
| lltp-default | ILL/KLE-IMP-CONJ | intuitionistic auto j16 pool after 0.1s | 222 | 222 | 222 | 0 | 0 | – | – | 1.04× (222) | 0 |
| lltp-default | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j16 pool after 0.1s | 27 | 27 | 27 | 0 | 0 | – | – | 1.00× (27) | 0 |
| lltp-default | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j16 pool after 0.1s | 22 | 22 | 22 | 0 | 0 | – | – | 1.52× (22) | 0 |
| lltp-default | ILL/KLE-cbn | intuitionistic auto j16 pool after 0.1s | 88 | 88 | 88 | 0 | 0 | – | – | 1.08× (88) | 0 |
| lltp-default | ILL/KLE-cbv | intuitionistic auto j16 pool after 0.1s | 88 | 88 | 88 | 0 | 0 | – | – | 1.19× (88) | 0 |
| lltp-default | ILL/Non-theorems | intuitionistic auto j16 pool after 0.1s | 1 | 0 | 0 | 0 | 0 | 1 time | 1 time | – | 0 |
| lltp-default | ILL/misc | intuitionistic auto j16 pool after 0.1s | 3 | 3 | 3 | 0 | 0 | – | – | 1.12× (3) | 0 |
| lltp-default | ILL/petri-nets/MCC | intuitionistic auto j16 pool after 0.1s | 3137 | 3069 | 3069 | 0 | 0 | 66 time, 2 memory | 66 time, 2 memory | 1.03× (3069) | 0 |
| lltp-forward | ILL/ILLTP-LCL-01 | intuitionistic auto j1 bias factors | 2 | 2 | 2 | 0 | 0 | – | – | 1.49× (2) | 0 |
| lltp-forward | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 bias factors | 2 | 2 | 2 | 0 | 0 | – | – | 1.51× (2) | 0 |
| lltp-forward | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 bias factors | 2 | 2 | 2 | 0 | 0 | – | – | 1.51× (2) | 0 |
| lltp-forward | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 bias factors | 252 | 99 | 138 | 1 | 40 | 124 time, 4 bound, 22 depth, 3 memory | 90 time, 22 depth, 2 memory | 1.32× (98) | 0 |
| lltp-forward | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 bias factors | 252 | 142 | 170 | 0 | 28 | 87 time, 22 depth, 1 memory | 59 time, 22 depth, 1 memory | 1.34× (142) | 0 |
| lltp-forward | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 bias factors | 252 | 118 | 157 | 0 | 39 | 118 time, 4 bound, 10 depth, 2 memory | 83 time, 10 depth, 2 memory | 1.29× (118) | 0 |
| lltp-forward | ILL/ILLTP-SYN-01 | intuitionistic auto j1 bias factors | 19 | 19 | 19 | 0 | 0 | – | – | 1.33× (19) | 0 |
| lltp-forward | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 bias factors | 19 | 19 | 19 | 0 | 0 | – | – | 1.43× (19) | 0 |
| lltp-forward | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 bias factors | 19 | 19 | 19 | 0 | 0 | – | – | 1.43× (19) | 0 |
| lltp-forward | ILL/KLE-01 | intuitionistic auto j1 bias factors | 88 | 86 | 87 | 0 | 1 | 2 bound | 1 bound | 1.30× (86) | 0 |
| lltp-forward | ILL/KLE-IMP-CONJ | intuitionistic auto j1 bias factors | 222 | 222 | 222 | 0 | 0 | – | – | 1.32× (222) | 0 |
| lltp-forward | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j1 bias factors | 27 | 27 | 27 | 0 | 0 | – | – | 1.52× (27) | 0 |
| lltp-forward | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j1 bias factors | 22 | 22 | 22 | 0 | 0 | – | – | 1.89× (22) | 0 |
| lltp-forward | ILL/KLE-cbn | intuitionistic auto j1 bias factors | 88 | 87 | 88 | 0 | 1 | 1 bound | – | 1.33× (87) | 0 |
| lltp-forward | ILL/KLE-cbv | intuitionistic auto j1 bias factors | 88 | 86 | 87 | 0 | 1 | 2 bound | 1 bound | 1.33× (86) | 0 |
| lltp-forward | ILL/Non-theorems | intuitionistic auto j1 bias factors | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
| lltp-forward | ILL/misc | intuitionistic auto j1 bias factors | 3 | 3 | 3 | 0 | 0 | – | – | 1.33× (3) | 0 |
| lltp-forward | ILL/petri-nets/MCC | intuitionistic auto j1 bias factors | 3137 | 3071 | 3071 | 0 | 0 | 57 time, 9 memory | 63 time, 3 memory | 1.33× (3071) | 0 |
| lltp-intuitionistic | ILL/ILLTP-LCL-01 | intuitionistic auto j1 | 2 | 0 | 1 | 0 | 1 | 2 bound | 1 bound | – | 0 |
| lltp-intuitionistic | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.05× (2) | 0 |
| lltp-intuitionistic | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 | 2 | 0 | 1 | 0 | 1 | 2 bound | 1 bound | – | 0 |
| lltp-intuitionistic | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 252 | 11 | 34 | 0 | 23 | 1 time, 215 bound, 22 depth, 3 memory | 4 time, 189 bound, 22 depth, 3 memory | 1.06× (11) | 0 |
| lltp-intuitionistic | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 252 | 35 | 38 | 0 | 3 | 3 time, 190 bound, 22 depth, 2 memory | 4 time, 186 bound, 22 depth, 2 memory | 0.89× (35) | 0 |
| lltp-intuitionistic | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 252 | 40 | 43 | 0 | 3 | 22 time, 178 bound, 10 depth, 2 memory | 23 time, 174 bound, 10 depth, 2 memory | 1.05× (40) | 0 |
| lltp-intuitionistic | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | 19 | 13 | 14 | 0 | 1 | 6 bound | 5 bound | 0.97× (13) | 0 |
| lltp-intuitionistic | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 | 19 | 18 | 18 | 0 | 0 | 1 bound | 1 bound | 1.04× (18) | 0 |
| lltp-intuitionistic | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 | 19 | 18 | 18 | 0 | 0 | 1 bound | 1 bound | 1.04× (18) | 0 |
| lltp-intuitionistic | ILL/KLE-01 | intuitionistic auto j1 | 88 | 51 | 58 | 0 | 7 | 37 bound | 30 bound | 0.96× (51) | 0 |
| lltp-intuitionistic | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | 222 | 162 | 162 | 0 | 0 | 60 bound | 60 bound | 1.03× (162) | 0 |
| lltp-intuitionistic | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j1 | 27 | 27 | 27 | 0 | 0 | – | – | 0.98× (27) | 0 |
| lltp-intuitionistic | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j1 | 22 | 22 | 22 | 0 | 0 | – | – | 1.56× (22) | 0 |
| lltp-intuitionistic | ILL/KLE-cbn | intuitionistic auto j1 | 88 | 69 | 76 | 0 | 7 | 19 bound | 12 bound | 0.91× (69) | 0 |
| lltp-intuitionistic | ILL/KLE-cbv | intuitionistic auto j1 | 88 | 68 | 75 | 0 | 7 | 20 bound | 13 bound | 1.00× (68) | 0 |
| lltp-intuitionistic | ILL/Non-theorems | intuitionistic auto j1 | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
| lltp-intuitionistic | ILL/misc | intuitionistic auto j1 | 3 | 3 | 3 | 0 | 0 | – | – | 1.08× (3) | 0 |
| lltp-intuitionistic | ILL/petri-nets/MCC | intuitionistic auto j1 | 3137 | 3071 | 3071 | 0 | 0 | 58 time, 8 memory | 57 time, 9 memory | 0.97× (3071) | 0 |
| lltp-intuitionistic-generous | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 4 | 0 | 0 | 0 | 0 | 1 bound, 3 memory | 2 time, 2 memory | – | 0 |
| lltp-intuitionistic-generous | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 2 | 0 | 0 | 0 | 0 | 2 memory | 2 memory | – | 0 |
| lltp-intuitionistic-generous | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 3 | 0 | 0 | 0 | 0 | 1 time, 2 memory | 1 time, 2 memory | – | 0 |
| lltp-intuitionistic-generous | ILL/petri-nets/MCC | intuitionistic auto j1 | 19 | 19 | 19 | 0 | 0 | – | – | 1.32× (19) | 0 |
| lltp-rarer | ILL/ILLTP-LCL-01 | intuitionistic auto j1 bias rarer | 2 | 0 | 1 | 0 | 1 | 2 bound | 1 bound | – | 0 |
| lltp-rarer | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 bias rarer | 2 | 1 | 2 | 0 | 1 | 1 bound | – | 1.45× (1) | 0 |
| lltp-rarer | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 bias rarer | 2 | 0 | 1 | 0 | 1 | 2 bound | 1 bound | – | 0 |
| lltp-rarer | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 bias rarer | 252 | 11 | 34 | 0 | 23 | 1 time, 215 bound, 22 depth, 3 memory | 5 time, 189 bound, 22 depth, 2 memory | 1.43× (11) | 0 |
| lltp-rarer | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 bias rarer | 252 | 34 | 37 | 0 | 3 | 3 time, 192 bound, 22 depth, 1 memory | 7 time, 185 bound, 22 depth, 1 memory | 1.55× (34) | 0 |
| lltp-rarer | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 bias rarer | 252 | 40 | 43 | 0 | 3 | 22 time, 178 bound, 10 depth, 2 memory | 24 time, 173 bound, 10 depth, 2 memory | 1.58× (40) | 0 |
| lltp-rarer | ILL/ILLTP-SYN-01 | intuitionistic auto j1 bias rarer | 19 | 13 | 14 | 0 | 1 | 6 bound | 5 bound | 1.52× (13) | 0 |
| lltp-rarer | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 bias rarer | 19 | 18 | 18 | 0 | 0 | 1 bound | 1 bound | 1.53× (18) | 0 |
| lltp-rarer | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 bias rarer | 19 | 18 | 18 | 0 | 0 | 1 bound | 1 bound | 1.40× (18) | 0 |
| lltp-rarer | ILL/KLE-01 | intuitionistic auto j1 bias rarer | 88 | 51 | 58 | 0 | 7 | 37 bound | 30 bound | 1.36× (51) | 0 |
| lltp-rarer | ILL/KLE-IMP-CONJ | intuitionistic auto j1 bias rarer | 222 | 162 | 162 | 0 | 0 | 60 bound | 60 bound | 1.40× (162) | 0 |
| lltp-rarer | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j1 bias rarer | 27 | 27 | 27 | 0 | 0 | – | – | 1.39× (27) | 0 |
| lltp-rarer | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j1 bias rarer | 22 | 22 | 22 | 0 | 0 | – | – | 1.90× (22) | 0 |
| lltp-rarer | ILL/KLE-cbn | intuitionistic auto j1 bias rarer | 88 | 69 | 76 | 0 | 7 | 19 bound | 12 bound | 1.51× (69) | 0 |
| lltp-rarer | ILL/KLE-cbv | intuitionistic auto j1 bias rarer | 88 | 68 | 75 | 0 | 7 | 20 bound | 13 bound | 1.48× (68) | 0 |
| lltp-rarer | ILL/Non-theorems | intuitionistic auto j1 bias rarer | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
| lltp-rarer | ILL/misc | intuitionistic auto j1 bias rarer | 3 | 3 | 3 | 0 | 0 | – | – | 1.38× (3) | 0 |
| lltp-rarer | ILL/petri-nets/MCC | intuitionistic auto j1 bias rarer | 3137 | 3071 | 3071 | 0 | 0 | 58 time, 8 memory | 64 time, 2 memory | 1.31× (3071) | 0 |
| lltp-recursion | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 22 | 0 | 0 | 0 | 0 | 20 time, 2 depth | 20 time, 2 depth | – | 0 |
| lltp-recursion | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 22 | 0 | 0 | 0 | 0 | 20 time, 2 depth | 20 time, 2 depth | – | 0 |
| lltp-recursion | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 22 | 0 | 0 | 0 | 0 | 22 time | 22 time | – | 0 |
| lltp-recursion | ILL/petri-nets/MCC | intuitionistic auto j1 | 929 | 895 | 895 | 0 | 0 | 32 time, 2 memory | 32 time, 2 memory | 0.98× (895) | 0 |
| lltp-recursion-generous | ILL/petri-nets/MCC | intuitionistic auto j1 | 9 | 9 | 9 | 0 | 0 | – | – | 1.00× (9) | 0 |
| long-1 | 3-partition-no | classical auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 1.28× (1) | 0 |
| long-1 | counter | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.40× (2) | 0 |
| long-1 | partition-no | classical auto j1 | 2 | 1 | 1 | 0 | 0 | 1 time | 1 time | 1.27× (1) | 0 |
| long-1 | partition-yes | classical auto j1 | 2 | 1 | 1 | 0 | 0 | 1 time | 1 time | 1.35× (1) | 0 |
| long-2 | mix | mix auto j1 | 1 | 0 | 1 | 0 | 1 | 1 time | – | – | 0 |
| long-2 | qbf | classical auto j1 | 2 | 1 | 1 | 0 | 0 | 1 memory | 1 memory | 2.60× (1) | 0 |
| long-2 | wide-m3 | classical auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 1.12× (1) | 0 |
| long-2 | wide-m4 | classical auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 1.19× (1) | 0 |
| parallel | 3-partition-no | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.05× (2) | 0 |
| parallel | 3-partition-no | classical auto j16 | 2 | 2 | 2 | 0 | 0 | – | – | 1.06× (2) | 0 |
| parallel | 3-partition-no | classical auto j2 | 2 | 2 | 2 | 0 | 0 | – | – | 0.98× (2) | 0 |
| parallel | 3-partition-no | classical auto j4 | 2 | 2 | 2 | 0 | 0 | – | – | 1.04× (2) | 0 |
| parallel | 3-partition-no | classical auto j8 | 2 | 2 | 2 | 0 | 0 | – | – | 0.88× (2) | 0 |
| parallel | 3-partition-yes | classical auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.00× (4) | 0 |
| parallel | 3-partition-yes | classical auto j16 | 4 | 4 | 4 | 0 | 0 | – | – | 1.12× (4) | 0 |
| parallel | 3-partition-yes | classical auto j2 | 4 | 4 | 4 | 0 | 0 | – | – | 0.98× (4) | 0 |
| parallel | 3-partition-yes | classical auto j4 | 4 | 4 | 4 | 0 | 0 | – | – | 1.15× (4) | 0 |
| parallel | 3-partition-yes | classical auto j8 | 4 | 4 | 4 | 0 | 0 | – | – | 1.01× (4) | 0 |
| parallel | counter | classical auto j1 | 6 | 6 | 6 | 0 | 0 | – | – | 1.32× (6) | 0 |
| parallel | counter | classical auto j16 | 6 | 6 | 6 | 0 | 0 | – | – | 1.02× (6) | 0 |
| parallel | counter | classical auto j2 | 6 | 6 | 6 | 0 | 0 | – | – | 1.16× (6) | 0 |
| parallel | counter | classical auto j4 | 6 | 6 | 6 | 0 | 0 | – | – | 1.11× (6) | 0 |
| parallel | counter | classical auto j8 | 6 | 6 | 6 | 0 | 0 | – | – | 1.15× (6) | 0 |
| parallel | counter-over | classical auto j1 | 6 | 6 | 6 | 0 | 0 | – | – | 0.97× (6) | 0 |
| parallel | counter-over | classical auto j16 | 6 | 6 | 6 | 0 | 0 | – | – | 1.13× (6) | 0 |
| parallel | counter-over | classical auto j2 | 6 | 6 | 6 | 0 | 0 | – | – | 1.18× (6) | 0 |
| parallel | counter-over | classical auto j4 | 6 | 6 | 6 | 0 | 0 | – | – | 1.15× (6) | 0 |
| parallel | counter-over | classical auto j8 | 6 | 6 | 6 | 0 | 0 | – | – | 1.13× (6) | 0 |
| parallel | mix | mix auto j1 | 4 | 3 | 4 | 0 | 1 | 1 time | – | 3.7e-2× (3) | 0 |
| parallel | mix | mix auto j16 | 4 | 2 | 4 (1 late) | 0 | 2 | 2 time | – | 0.87× (2) | 0 |
| parallel | mix | mix auto j2 | 4 | 3 | 4 (1 late) | 0 | 1 | 1 time | – | 0.84× (3) | 0 |
| parallel | mix | mix auto j4 | 4 | 3 | 4 (1 late) | 0 | 1 | 1 time | – | 0.78× (3) | 0 |
| parallel | mix | mix auto j8 | 4 | 3 | 4 (1 late) | 0 | 1 | 1 time | – | 0.77× (3) | 0 |
| parallel | partition-no | classical auto j1 | 4 | 3 | 3 | 0 | 0 | 1 time | 1 time | 1.00× (3) | 0 |
| parallel | partition-no | classical auto j16 | 4 | 3 | 3 | 0 | 0 | 1 time | 1 time | 1.00× (3) | 0 |
| parallel | partition-no | classical auto j2 | 4 | 3 | 3 | 0 | 0 | 1 time | 1 time | 1.01× (3) | 0 |
| parallel | partition-no | classical auto j4 | 4 | 3 | 3 | 0 | 0 | 1 time | 1 time | 1.01× (3) | 0 |
| parallel | partition-no | classical auto j8 | 4 | 3 | 3 | 0 | 0 | 1 time | 1 time | 1.05× (3) | 0 |
| parallel | partition-yes | classical auto j1 | 5 | 5 | 5 | 0 | 0 | – | – | 0.99× (5) | 0 |
| parallel | partition-yes | classical auto j16 | 5 | 5 | 5 | 0 | 0 | – | – | 0.98× (5) | 0 |
| parallel | partition-yes | classical auto j2 | 5 | 5 | 5 | 0 | 0 | – | – | 0.98× (5) | 0 |
| parallel | partition-yes | classical auto j4 | 5 | 5 | 5 | 0 | 0 | – | – | 0.97× (5) | 0 |
| parallel | partition-yes | classical auto j8 | 5 | 5 | 5 | 0 | 0 | – | – | 0.97× (5) | 0 |
| parallel | qbf | classical auto j1 | 20 | 19 | 19 | 0 | 0 | 1 memory | 1 memory | 1.06× (19) | 0 |
| parallel | qbf | classical auto j16 | 20 | 14 | 14 | 0 | 0 | 6 memory | 6 memory | 3.15× (14) | 0 |
| parallel | qbf | classical auto j2 | 20 | 14 | 14 | 0 | 0 | 6 memory | 6 memory | 3.71× (14) | 0 |
| parallel | qbf | classical auto j4 | 20 | 14 | 14 | 0 | 0 | 6 memory | 6 memory | 3.55× (14) | 0 |
| parallel | qbf | classical auto j8 | 20 | 14 | 14 | 0 | 0 | 6 memory | 6 memory | 3.46× (14) | 0 |
| parallel | wide-m3 | classical auto j1 | 3 | 3 | 3 | 0 | 0 | – | – | 0.91× (3) | 0 |
| parallel | wide-m3 | classical auto j16 | 3 | 3 | 3 | 0 | 0 | – | – | 0.95× (3) | 0 |
| parallel | wide-m3 | classical auto j2 | 3 | 3 | 3 | 0 | 0 | – | – | 0.84× (3) | 0 |
| parallel | wide-m3 | classical auto j4 | 3 | 3 | 3 | 0 | 0 | – | – | 0.93× (3) | 0 |
| parallel | wide-m3 | classical auto j8 | 3 | 3 | 3 | 0 | 0 | – | – | 0.99× (3) | 0 |
| parallel | wide-m4 | classical auto j1 | 3 | 3 | 3 | 0 | 0 | – | – | 0.99× (3) | 0 |
| parallel | wide-m4 | classical auto j16 | 3 | 3 | 3 | 0 | 0 | – | – | 0.94× (3) | 0 |
| parallel | wide-m4 | classical auto j2 | 3 | 3 | 3 | 0 | 0 | – | – | 0.83× (3) | 0 |
| parallel | wide-m4 | classical auto j4 | 3 | 3 | 3 | 0 | 0 | – | – | 0.98× (3) | 0 |
| parallel | wide-m4 | classical auto j8 | 3 | 3 | 3 | 0 | 0 | – | – | 0.94× (3) | 0 |
| parallel-net | 3-partition-mll-no | classical net j1 | 3 | 2 | 3 | 0 | 1 | 1 time | – | 2.3e-4× (2) | 0 |
| parallel-net | 3-partition-mll-no | classical net j16 | 3 | 2 | 3 | 0 | 1 | 1 time | – | 5.9e-3× (2) | 0 |
| parallel-net | 3-partition-mll-no | classical net j2 | 3 | 2 | 3 | 0 | 1 | 1 time | – | 6.1e-4× (2) | 0 |
| parallel-net | 3-partition-mll-no | classical net j4 | 3 | 2 | 3 | 0 | 1 | 1 time | – | 1.2e-3× (2) | 0 |
| parallel-net | 3-partition-mll-no | classical net j8 | 3 | 2 | 3 | 0 | 1 | 1 time | – | 2.4e-3× (2) | 0 |
| parallel-net | partition-table | classical net j1 | 13 | 9 | 13 | 0 | 4 | 4 time | – | 1.7e-2× (9) | 0 |
| parallel-net | partition-table | classical net j16 | 13 | 10 | 13 | 0 | 3 | 3 time | – | 0.25× (10) | 0 |
| parallel-net | partition-table | classical net j2 | 13 | 9 | 13 | 0 | 4 | 4 time | – | 3.3e-2× (9) | 0 |
| parallel-net | partition-table | classical net j4 | 13 | 10 | 13 | 0 | 3 | 3 time | – | 7.4e-2× (10) | 0 |
| parallel-net | partition-table | classical net j8 | 13 | 10 | 13 | 0 | 3 | 3 time | – | 0.13× (10) | 0 |
| period-1 | 3-partition-mll-no | classical net j1 period 1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.5e-4× (2) | 0 |
| period-1 | partition-no | classical net j1 period 1 | 2 | 1 | 2 | 0 | 1 | 1 time | – | 4.4e-4× (1) | 0 |
| period-1 | partition-yes | classical net j1 period 1 | 1 | 1 | 1 | 0 | 0 | – | – | 6.5e-4× (1) | 0 |
| period-1 | wide-m1 | classical net j1 period 1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.03× (2) | 0 |
| period-1 | wide-m2 | classical net j1 period 1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.01× (2) | 0 |
| period-16 | 3-partition-mll-no | classical net j1 period 16 | 2 | 1 | 2 | 0 | 1 | 1 time | – | 1.2e-5× (1) | 0 |
| period-16 | partition-no | classical net j1 period 16 | 2 | 0 | 2 | 0 | 2 | 2 time | – | – | 0 |
| period-16 | partition-yes | classical net j1 period 16 | 1 | 0 | 1 | 0 | 1 | 1 time | – | – | 0 |
| period-16 | wide-m1 | classical net j1 period 16 | 2 | 2 | 2 | 0 | 0 | – | – | 1.10× (2) | 0 |
| period-16 | wide-m2 | classical net j1 period 16 | 2 | 2 | 2 | 0 | 0 | – | – | 1.05× (2) | 0 |
| period-2 | 3-partition-mll-no | classical net j1 period 2 | 2 | 2 | 2 | 0 | 0 | – | – | 1.7e-4× (2) | 0 |
| period-2 | partition-no | classical net j1 period 2 | 2 | 1 | 2 | 0 | 1 | 1 time | – | 1.6e-4× (1) | 0 |
| period-2 | partition-yes | classical net j1 period 2 | 1 | 1 | 1 | 0 | 0 | – | – | 2.3e-4× (1) | 0 |
| period-2 | wide-m1 | classical net j1 period 2 | 2 | 2 | 2 | 0 | 0 | – | – | 0.99× (2) | 0 |
| period-2 | wide-m2 | classical net j1 period 2 | 2 | 2 | 2 | 0 | 0 | – | – | 1.01× (2) | 0 |
| period-8 | 3-partition-mll-no | classical net j1 period 8 | 2 | 1 | 2 | 0 | 1 | 1 time | – | 5.2e-5× (1) | 0 |
| period-8 | partition-no | classical net j1 period 8 | 2 | 0 | 2 | 0 | 2 | 2 time | – | – | 0 |
| period-8 | partition-yes | classical net j1 period 8 | 1 | 0 | 1 | 0 | 1 | 1 time | – | – | 0 |
| period-8 | wide-m1 | classical net j1 period 8 | 2 | 2 | 2 | 0 | 0 | – | – | 1.07× (2) | 0 |
| period-8 | wide-m2 | classical net j1 period 8 | 2 | 2 | 2 | 0 | 0 | – | – | 1.03× (2) | 0 |

### Generated and listed problems

The rows of `parallel`, `parallel-net`, `lltp-all-cores` and `lltp-default` in the fourth baseline are its parallel stage, 16 threads beside foreign load: not comparable with the earlier baselines' (above, "Where the fourth baseline ran differently").

| file | family | configuration | problem | before | after | after/before |
|---|---|---|---|--:|--:|--:|
| engines | 3-partition-mll-no | classical focus j1 | 3-partition-mll-no/4 | 39 µs ✗ | 37 µs ✗ | 0.95× |
| engines | 3-partition-mll-no | classical focus j1 | 3-partition-mll-no/5 | 41 µs ✗ | 39 µs ✗ | 0.95× |
| engines | 3-partition-mll-no | classical focus j1 | 3-partition-mll-no/6 | 41 µs ✗ | 42 µs ✗ | 1.02× |
| engines | 3-partition-mll-no | classical focus j1 | 3-partition-mll-no/8 | 46 µs ✗ | 44 µs ✗ | 0.96× |
| engines | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/4 | 1.83 s ✗ | 250 µs ✗ | 1.4e-4× |
| engines | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/5 | 56.13 s ✗ | 689 µs ✗ | 1.2e-5× |
| engines | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/6 | > 60 s | 1.2 ms ✗ |  |
| engines | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/8 | > 60 s | 3.0 ms ✗ |  |
| engines | 3-partition-mll-yes | classical focus j1 | 3-partition-mll-yes/12 | 57 µs ✓ | 55 µs ✓ | 0.96× |
| engines | 3-partition-mll-yes | classical focus j1 | 3-partition-mll-yes/4 | 64 µs ✓ | 50 µs ✓ | 0.78× |
| engines | 3-partition-mll-yes | classical focus j1 | 3-partition-mll-yes/6 | 55 µs ✓ | 49 µs ✓ | 0.89× |
| engines | 3-partition-mll-yes | classical focus j1 | 3-partition-mll-yes/8 | 55 µs ✓ | 52 µs ✓ | 0.95× |
| engines | 3-partition-mll-yes | classical net j1 | 3-partition-mll-yes/12 | 170 µs ✓ | 222 µs ✓ | 1.31× |
| engines | 3-partition-mll-yes | classical net j1 | 3-partition-mll-yes/4 | 62 µs ✓ | 76 µs ✓ | 1.23× |
| engines | 3-partition-mll-yes | classical net j1 | 3-partition-mll-yes/6 | 88 µs ✓ | 102 µs ✓ | 1.16× |
| engines | 3-partition-mll-yes | classical net j1 | 3-partition-mll-yes/8 | 107 µs ✓ | 132 µs ✓ | 1.23× |
| engines | additive | classical focus j1 | additive/12 | 38.1 ms ✓ | 31.6 ms ✓ | 0.83× |
| engines | additive | classical focus j1 | additive/14 | 391.5 ms ✓ | 328.5 ms ✓ | 0.84× |
| engines | additive | classical focus j1 | additive/16 | 5.79 s ✓ | 5.28 s ✓ | 0.91× |
| engines | additive | classical focus j1 | additive/8 | 873 µs ✓ | 740 µs ✓ | 0.85× |
| engines | cancellation | classical focus j1 | cancellation/3-partition-4 | 300 µs ✓ | 278 µs ✓ | 0.93× |
| engines | partition-no | classical focus j1 | partition-no/12 | 3.81 s ✗ | 3.47 s ✗ | 0.91× |
| engines | partition-no | classical focus j1 | partition-no/3 | 71 µs ✗ | 82 µs ✗ | 1.15× |
| engines | partition-no | classical focus j1 | partition-no/4 | 150 µs ✗ | 147 µs ✗ | 0.98× |
| engines | partition-no | classical focus j1 | partition-no/9 | 89.0 ms ✗ | 80.2 ms ✗ | 0.90× |
| engines | partition-no | classical net j1 | partition-no/12 | > 60 s | > 60 s |  |
| engines | partition-no | classical net j1 | partition-no/3 | 3.68 s ✗ | 1.5 ms ✗ | 4.0e-4× |
| engines | partition-no | classical net j1 | partition-no/4 | > 60 s | 9.6 ms ✗ |  |
| engines | partition-no | classical net j1 | partition-no/9 | > 60 s | > 60 s |  |
| engines | partition-table | classical focus j1 | partition-table/1-1 | 51 µs ✓ | 50 µs ✓ | 0.98× |
| engines | partition-table | classical focus j1 | partition-table/1-1-1-1-1-7 | 1.2 ms ✗ | 1.1 ms ✗ | 0.96× |
| engines | partition-table | classical focus j1 | partition-table/1-1-1-5 | 141 µs ✗ | 132 µs ✗ | 0.94× |
| engines | partition-table | classical focus j1 | partition-table/1-1-2-4 | 70 µs ✓ | 58 µs ✓ | 0.83× |
| engines | partition-table | classical focus j1 | partition-table/1-1-4 | 81 µs ✗ | 72 µs ✗ | 0.89× |
| engines | partition-table | classical focus j1 | partition-table/1-2-3-4-5-5 | 97 µs ✓ | 86 µs ✓ | 0.89× |
| engines | partition-table | classical focus j1 | partition-table/1-2-5 | 83 µs ✗ | 74 µs ✗ | 0.89× |
| engines | partition-table | classical focus j1 | partition-table/1-3 | 50 µs ✗ | 50 µs ✗ | 1.00× |
| engines | partition-table | classical focus j1 | partition-table/2-1-1 | 67 µs ✓ | 60 µs ✓ | 0.90× |
| engines | partition-table | classical focus j1 | partition-table/2-2-1-1 | 82 µs ✓ | 74 µs ✓ | 0.90× |
| engines | partition-table | classical focus j1 | partition-table/2-2-2-2-2-2-9-1 | 129 µs ✓ | 106 µs ✓ | 0.82× |
| engines | partition-table | classical focus j1 | partition-table/2-3-2-1 | 75 µs ✓ | 76 µs ✓ | 1.01× |
| engines | partition-table | classical focus j1 | partition-table/3-3-3-1 | 137 µs ✗ | 138 µs ✗ | 1.01× |
| engines | partition-table | classical net j1 | partition-table/1-1 | 51 µs ✓ | 53 µs ✓ | 1.04× |
| engines | partition-table | classical net j1 | partition-table/1-1-1-1-1-7 | > 60 s | 106.5 ms ✗ |  |
| engines | partition-table | classical net j1 | partition-table/1-1-1-5 | 7.43 s ✗ | 3.1 ms ✗ | 4.2e-4× |
| engines | partition-table | classical net j1 | partition-table/1-1-2-4 | 1.04 s ✓ | 889 µs ✓ | 8.5e-4× |
| engines | partition-table | classical net j1 | partition-table/1-1-4 | 40.2 ms ✗ | 559 µs ✗ | 1.4e-2× |
| engines | partition-table | classical net j1 | partition-table/1-2-3-4-5-5 | > 60 s | 1.08 s ✓ |  |
| engines | partition-table | classical net j1 | partition-table/1-2-5 | 3.68 s ✗ | 1.3 ms ✗ | 3.4e-4× |
| engines | partition-table | classical net j1 | partition-table/1-3 | 388 µs ✗ | 118 µs ✗ | 0.30× |
| engines | partition-table | classical net j1 | partition-table/2-1-1 | 626 µs ✓ | 169 µs ✓ | 0.27× |
| engines | partition-table | classical net j1 | partition-table/2-2-1-1 | 52.2 ms ✓ | 928 µs ✓ | 1.8e-2× |
| engines | partition-table | classical net j1 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | 4.01 s ✓ |  |
| engines | partition-table | classical net j1 | partition-table/2-3-2-1 | 4.68 s ✓ | 2.5 ms ✓ | 5.3e-4× |
| engines | partition-table | classical net j1 | partition-table/3-3-3-1 | > 60 s | 8.3 ms ✗ |  |
| engines | partition-yes | classical focus j1 | partition-yes/12 | 13.3 ms ✓ | 12.8 ms ✓ | 0.96× |
| engines | partition-yes | classical focus j1 | partition-yes/20 | 471.1 ms ✓ | 426.4 ms ✓ | 0.91× |
| engines | partition-yes | classical focus j1 | partition-yes/4 | 104 µs ✓ | 68 µs ✓ | 0.65× |
| engines | partition-yes | classical focus j1 | partition-yes/5 | 99 µs ✓ | 79 µs ✓ | 0.80× |
| engines | partition-yes | classical focus j1 | partition-yes/6 | 140 µs ✓ | 130 µs ✓ | 0.93× |
| engines | partition-yes | classical net j1 | partition-yes/12 | > 60 s | > 60 s |  |
| engines | partition-yes | classical net j1 | partition-yes/20 | > 60 s | > 60 s |  |
| engines | partition-yes | classical net j1 | partition-yes/4 | 2.83 s ✓ | 1.8 ms ✓ | 6.2e-4× |
| engines | partition-yes | classical net j1 | partition-yes/5 | > 60 s | 28.5 ms ✓ |  |
| engines | partition-yes | classical net j1 | partition-yes/6 | > 60 s | 476.1 ms ✓ |  |
| engines | wide-m1 | classical focus j1 | wide-m1/16 | 77 µs ✓ | 75 µs ✓ | 0.97× |
| engines | wide-m1 | classical focus j1 | wide-m1/2048 | 206.6 ms ✓ | 209.2 ms ✓ | 1.01× |
| engines | wide-m1 | classical focus j1 | wide-m1/24 | 101 µs ✓ | 89 µs ✓ | 0.88× |
| engines | wide-m1 | classical focus j1 | wide-m1/256 | 4.0 ms ✓ | 3.3 ms ✓ | 0.83× |
| engines | wide-m1 | classical focus j1 | wide-m1/32 | 148 µs ✓ | 121 µs ✓ | 0.82× |
| engines | wide-m1 | classical focus j1 | wide-m1/8 | 52 µs ✓ | 50 µs ✓ | 0.96× |
| engines | wide-m1 | classical net j1 | wide-m1/16 | 120 µs ✓ | 129 µs ✓ | 1.08× |
| engines | wide-m1 | classical net j1 | wide-m1/2048 | 586.4 ms ✓ | 598.9 ms ✓ | 1.02× |
| engines | wide-m1 | classical net j1 | wide-m1/24 | 199 µs ✓ | 212 µs ✓ | 1.07× |
| engines | wide-m1 | classical net j1 | wide-m1/256 | 9.0 ms ✓ | 9.0 ms ✓ | 0.99× |
| engines | wide-m1 | classical net j1 | wide-m1/32 | 182 µs ✓ | 188 µs ✓ | 1.03× |
| engines | wide-m1 | classical net j1 | wide-m1/8 | 62 µs ✓ | 61 µs ✓ | 0.98× |
| engines | wide-m2 | classical focus j1 | wide-m2/16 | 69 µs ✓ | 67 µs ✓ | 0.97× |
| engines | wide-m2 | classical focus j1 | wide-m2/2048 | 209.9 ms ✓ | 200.5 ms ✓ | 0.96× |
| engines | wide-m2 | classical focus j1 | wide-m2/24 | 105 µs ✓ | 86 µs ✓ | 0.82× |
| engines | wide-m2 | classical focus j1 | wide-m2/256 | 3.2 ms ✓ | 2.8 ms ✓ | 0.86× |
| engines | wide-m2 | classical focus j1 | wide-m2/32 | 128 µs ✓ | 106 µs ✓ | 0.83× |
| engines | wide-m2 | classical focus j1 | wide-m2/8 | 55 µs ✓ | 52 µs ✓ | 0.95× |
| engines | wide-m2 | classical net j1 | wide-m2/16 | 113 µs ✓ | 124 µs ✓ | 1.10× |
| engines | wide-m2 | classical net j1 | wide-m2/2048 | 610.6 ms ✓ | 593.2 ms ✓ | 0.97× |
| engines | wide-m2 | classical net j1 | wide-m2/24 | 205 µs ✓ | 216 µs ✓ | 1.05× |
| engines | wide-m2 | classical net j1 | wide-m2/256 | 9.0 ms ✓ | 9.0 ms ✓ | 1.00× |
| engines | wide-m2 | classical net j1 | wide-m2/32 | 178 µs ✓ | 188 µs ✓ | 1.06× |
| engines | wide-m2 | classical net j1 | wide-m2/8 | 55 µs ✓ | 63 µs ✓ | 1.15× |
| engines | wide-m3 | classical focus j1 | wide-m3/12 | 69 µs ✓ | 60 µs ✓ | 0.87× |
| engines | wide-m3 | classical focus j1 | wide-m3/2048 | 192.5 ms ✓ | 179.1 ms ✓ | 0.93× |
| engines | wide-m3 | classical focus j1 | wide-m3/24 | 96 µs ✓ | 79 µs ✓ | 0.82× |
| engines | wide-m3 | classical focus j1 | wide-m3/256 | 3.0 ms ✓ | 2.5 ms ✓ | 0.83× |
| engines | wide-m3 | classical focus j1 | wide-m3/30 | 123 µs ✓ | 104 µs ✓ | 0.85× |
| engines | wide-m3 | classical net j1 | wide-m3/12 | 85 µs ✓ | 96 µs ✓ | 1.13× |
| engines | wide-m3 | classical net j1 | wide-m3/2048 | 628.1 ms ✓ | 630.5 ms ✓ | 1.00× |
| engines | wide-m3 | classical net j1 | wide-m3/24 | 211 µs ✓ | 226 µs ✓ | 1.07× |
| engines | wide-m3 | classical net j1 | wide-m3/256 | 9.4 ms ✓ | 9.3 ms ✓ | 1.00× |
| engines | wide-m3 | classical net j1 | wide-m3/30 | 176 µs ✓ | 177 µs ✓ | 1.01× |
| engines | wide-m4 | classical focus j1 | wide-m4/12 | 59 µs ✓ | 58 µs ✓ | 0.98× |
| engines | wide-m4 | classical focus j1 | wide-m4/2048 | 182.0 ms ✓ | 166.7 ms ✓ | 0.92× |
| engines | wide-m4 | classical focus j1 | wide-m4/24 | 93 µs ✓ | 83 µs ✓ | 0.89× |
| engines | wide-m4 | classical focus j1 | wide-m4/256 | 2.8 ms ✓ | 2.3 ms ✓ | 0.82× |
| engines | wide-m4 | classical focus j1 | wide-m4/28 | 105 µs ✓ | 91 µs ✓ | 0.87× |
| engines | wide-m4 | classical net j1 | wide-m4/12 | 81 µs ✓ | 90 µs ✓ | 1.11× |
| engines | wide-m4 | classical net j1 | wide-m4/2048 | 651.3 ms ✓ | 633.1 ms ✓ | 0.97× |
| engines | wide-m4 | classical net j1 | wide-m4/24 | 218 µs ✓ | 224 µs ✓ | 1.03× |
| engines | wide-m4 | classical net j1 | wide-m4/256 | 9.8 ms ✓ | 9.4 ms ✓ | 0.97× |
| engines | wide-m4 | classical net j1 | wide-m4/28 | 258 µs ✓ | 290 µs ✓ | 1.12× |
| families | 3-partition-mll-no | classical auto j1 | 3-partition-mll-no/4 | 39 µs ✗ | 45 µs ✗ | 1.15× |
| families | 3-partition-mll-no | classical auto j1 | 3-partition-mll-no/5 | 41 µs ✗ | 47 µs ✗ | 1.15× |
| families | 3-partition-mll-no | classical auto j1 | 3-partition-mll-no/6 | 45 µs ✗ | 44 µs ✗ | 0.98× |
| families | 3-partition-mll-no | classical auto j1 | 3-partition-mll-no/8 | 40 µs ✗ | 45 µs ✗ | 1.12× |
| families | 3-partition-mll-yes | classical auto j1 | 3-partition-mll-yes/12 | 55 µs ✓ | 51 µs ✓ | 0.93× |
| families | 3-partition-mll-yes | classical auto j1 | 3-partition-mll-yes/4 | 54 µs ✓ | 51 µs ✓ | 0.94× |
| families | 3-partition-mll-yes | classical auto j1 | 3-partition-mll-yes/6 | 53 µs ✓ | 52 µs ✓ | 0.98× |
| families | 3-partition-mll-yes | classical auto j1 | 3-partition-mll-yes/8 | 56 µs ✓ | 55 µs ✓ | 0.98× |
| families | 3-partition-no | classical auto j1 | 3-partition-no/4 | 319 µs ✗ | 286 µs ✗ | 0.90× |
| families | 3-partition-no | classical auto j1 | 3-partition-no/5 | 346 µs ✗ | 320 µs ✗ | 0.92× |
| families | 3-partition-yes | classical auto j1 | 3-partition-yes/12 | 64 µs ✓ | 63 µs ✓ | 0.98× |
| families | 3-partition-yes | classical auto j1 | 3-partition-yes/4 | 68 µs ✓ | 59 µs ✓ | 0.87× |
| families | 3-partition-yes | classical auto j1 | 3-partition-yes/6 | 61 µs ✓ | 59 µs ✓ | 0.97× |
| families | 3-partition-yes | classical auto j1 | 3-partition-yes/8 | 63 µs ✓ | 59 µs ✓ | 0.94× |
| families | additive | classical auto j1 | additive/12 | 10.8 ms ✓ | 11.4 ms ✓ | 1.06× |
| families | additive | classical auto j1 | additive/14 | 58.4 ms ✓ | 61.2 ms ✓ | 1.05× |
| families | additive | classical auto j1 | additive/16 | 181.6 ms ✓ | 192.2 ms ✓ | 1.06× |
| families | additive | classical auto j1 | additive/8 | 329 µs ✓ | 370 µs ✓ | 1.12× |
| families | chain | classical auto j1 | chain/128 | 33.7 ms ✓ | 34.0 ms ✓ | 1.01× |
| families | chain | classical auto j1 | chain/16 | 154 µs ✓ | 152 µs ✓ | 0.99× |
| families | chain | classical auto j1 | chain/256 | 274.7 ms ✓ | 276.1 ms ✓ | 1.00× |
| families | chain | classical auto j1 | chain/64 | 4.4 ms ✓ | 4.4 ms ✓ | 0.99× |
| families | chain-fo | classical auto j1 | chain-fo/16#0 |  | 370 µs ✓ |  |
| families | chain-fo | classical auto j1 | chain-fo/16#1 |  | 345 µs ? bound |  |
| families | chain-fo | classical auto j1 | chain-fo/256#0 |  | 309.7 ms ✓ |  |
| families | chain-fo | classical auto j1 | chain-fo/256#1 |  | 275.7 ms ? bound |  |
| families | chain-fo | classical auto j1 | chain-fo/512#0 |  | 1.19 s ? depth |  |
| families | chain-fo | classical auto j1 | chain-fo/512#1 |  | 1.03 s ? depth |  |
| families | chain-fo | classical auto j1 | chain-fo/64#0 |  | 7.5 ms ✓ |  |
| families | chain-fo | classical auto j1 | chain-fo/64#1 |  | 6.9 ms ? bound |  |
| families | contraction | intuitionistic auto j1 | contraction/2#0 |  | 57 µs ✓ |  |
| families | contraction | intuitionistic auto j1 | contraction/2#1 |  | 73 µs ? bound |  |
| families | contraction | intuitionistic auto j1 | contraction/4#0 |  | 154 µs ✓ |  |
| families | contraction | intuitionistic auto j1 | contraction/4#1 |  | 488 µs ? bound |  |
| families | contraction | intuitionistic auto j1 | contraction/8#0 |  | 66.5 ms ✓ |  |
| families | contraction | intuitionistic auto j1 | contraction/8#1 |  | 74.8 ms ✗ |  |
| families | counter | classical auto j1 | counter/16 | 37 µs ✓ | 35 µs ✓ | 0.95× |
| families | counter | classical auto j1 | counter/2 | 27 µs ✓ | 35 µs ✓ | 1.30× |
| families | counter | classical auto j1 | counter/32 | 43 µs ✓ | 40 µs ✓ | 0.93× |
| families | counter | classical auto j1 | counter/4 | 28 µs ✓ | 29 µs ✓ | 1.04× |
| families | counter | classical auto j1 | counter/64 | 55 µs ✓ | 53 µs ✓ | 0.96× |
| families | counter | classical auto j1 | counter/8 | 34 µs ✓ | 32 µs ✓ | 0.94× |
| families | counter-over | classical auto j1 | counter-over/16 | 35 µs ✗ | 38 µs ✗ | 1.09× |
| families | counter-over | classical auto j1 | counter-over/2 | 24 µs ✗ | 28 µs ✗ | 1.17× |
| families | counter-over | classical auto j1 | counter-over/32 | 44 µs ✗ | 42 µs ✗ | 0.95× |
| families | counter-over | classical auto j1 | counter-over/4 | 26 µs ✗ | 31 µs ✗ | 1.19× |
| families | counter-over | classical auto j1 | counter-over/64 | 45 µs ✗ | 45 µs ✗ | 1.00× |
| families | counter-over | classical auto j1 | counter-over/8 | 33 µs ✗ | 36 µs ✗ | 1.09× |
| families | duplicate | classical auto j1 | duplicate/2#1 |  | 25 µs ✓ |  |
| families | duplicate | classical auto j1 | duplicate/4#1 |  | 29 µs ✓ |  |
| families | duplicate | classical auto j1 | duplicate/8#1 |  | 28 µs ✓ |  |
| families | duplicate | intuitionistic auto j1 | duplicate/2#0 |  | 28 µs ✓ |  |
| families | duplicate | intuitionistic auto j1 | duplicate/4#0 |  | 33 µs ✓ |  |
| families | duplicate | intuitionistic auto j1 | duplicate/8#0 |  | 30 µs ✓ |  |
| families | euler | intuitionistic auto j1 | euler/5#0 |  | 55 µs ✓ |  |
| families | euler | intuitionistic auto j1 | euler/5#1 |  | 151 µs ✗ |  |
| families | euler | intuitionistic auto j1 | euler/6#0 |  | 72 µs ✓ |  |
| families | euler | intuitionistic auto j1 | euler/6#1 |  | 344 µs ✗ |  |
| families | euler | intuitionistic auto j1 | euler/8#0 |  | 72 µs ✓ |  |
| families | euler | intuitionistic auto j1 | euler/8#1 |  | 1.6 ms ✗ |  |
| families | growing | classical auto j1 | growing/1024 | 114.7 ms ? depth | 113.3 ms ✗ |  |
| families | growing | classical auto j1 | growing/16 | 213 µs ? bound | 231 µs ✗ |  |
| families | growing | classical auto j1 | growing/256 | 18.4 ms ? bound | 18.2 ms ✗ |  |
| families | growing | classical auto j1 | growing/64 | 800 µs ? bound | 808 µs ✗ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#0 |  | 125 µs ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#1 |  | 83 µs ✗ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#2 |  | 247 µs ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#3 |  | 89 µs ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/16#4 |  | 86 µs ✗ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#0 |  | 4.0 ms ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#1 |  | 9.9 ms ✗ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#2 |  | 429.6 ms ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#3 |  | 4.5 ms ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/256#4 |  | 5.8 ms ✗ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#0 |  | 13.3 ms ? depth |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#1 |  | 46.1 ms ✗ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#2 |  | 3.87 s ? depth |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#3 |  | 18.3 ms ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/512#4 |  | 24.8 ms ✗ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#0 |  | 470 µs ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#1 |  | 552 µs ✗ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#2 |  | 6.7 ms ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#3 |  | 406 µs ✓ |  |
| families | lambek-fo | intuitionistic auto j1 | lambek-fo/64#4 |  | 415 µs ✗ |  |
| families | mix | mix auto j1 | mix/10 | 62.55 s ✗ | 258.7 ms ✗ | 4.1e-3× |
| families | mix | mix auto j1 | mix/11 | > 300 s | 269.6 ms ✗ |  |
| families | mix | mix auto j1 | mix/4 | 365 µs ✗ | 330 µs ✗ | 0.90× |
| families | mix | mix auto j1 | mix/6 | 14.0 ms ✗ | 11.2 ms ✗ | 0.80× |
| families | mix | mix auto j1 | mix/8 | 858.5 ms ✗ | 256.8 ms ✗ | 0.30× |
| families | mix | mix auto j1 | mix/9 | 7.31 s ✗ | 257.3 ms ✗ | 3.5e-2× |
| families | partition-no | classical auto j1 | partition-no/12 | 3.82 s ✗ | 3.21 s ✗ | 0.84× |
| families | partition-no | classical auto j1 | partition-no/14 | > 300 s | > 300 s |  |
| families | partition-no | classical auto j1 | partition-no/15 | > 300 s | > 300 s |  |
| families | partition-no | classical auto j1 | partition-no/3 | 79 µs ✗ | 76 µs ✗ | 0.96× |
| families | partition-no | classical auto j1 | partition-no/4 | 144 µs ✗ | 136 µs ✗ | 0.94× |
| families | partition-no | classical auto j1 | partition-no/5 | 435 µs ✗ | 410 µs ✗ | 0.94× |
| families | partition-no | classical auto j1 | partition-no/9 | 85.5 ms ✗ | 72.4 ms ✗ | 0.85× |
| families | partition-yes | classical auto j1 | partition-yes/12 | 13.8 ms ✓ | 12.6 ms ✓ | 0.92× |
| families | partition-yes | classical auto j1 | partition-yes/16 | 147.8 ms ✓ | 134.4 ms ✓ | 0.91× |
| families | partition-yes | classical auto j1 | partition-yes/20 | 454.7 ms ✓ | 411.7 ms ✓ | 0.91× |
| families | partition-yes | classical auto j1 | partition-yes/24 | 74.41 s ✓ | 71.59 s ✓ | 0.96× |
| families | partition-yes | classical auto j1 | partition-yes/28 | > 300 s | > 300 s |  |
| families | partition-yes | classical auto j1 | partition-yes/4 | 81 µs ✓ | 69 µs ✓ | 0.85× |
| families | partition-yes | classical auto j1 | partition-yes/5 | 93 µs ✓ | 86 µs ✓ | 0.92× |
| families | partition-yes | classical auto j1 | partition-yes/6 | 136 µs ✓ | 131 µs ✓ | 0.96× |
| families | partition-yes | classical auto j1 | partition-yes/7 | 211 µs ✓ | 196 µs ✓ | 0.93× |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/2#0 |  | 71 µs ✓ |  |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/2#1 |  | 40 µs ✗ |  |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/3#0 |  | 951 µs ✓ |  |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/3#1 |  | 76 µs ✗ |  |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/4#0 |  | 38.4 ms ✓ |  |
| families | pigeonhole | intuitionistic-affine auto j1 | pigeonhole/4#1 |  | 284 µs ✗ |  |
| families | qbf | classical auto j1 | qbf/12#0 | 1.2 ms ✓ | 1.2 ms ✓ | 0.94× |
| families | qbf | classical auto j1 | qbf/12#1 | 388 µs ✗ | 942 µs ✗ | 2.43× |
| families | qbf | classical auto j1 | qbf/12#2 | 625 µs ✓ | 597 µs ✓ | 0.96× |
| families | qbf | classical auto j1 | qbf/12#3 | 297 µs ✗ | 891 µs ✗ | 3.00× |
| families | qbf | classical auto j1 | qbf/16#0 | 6.0 ms ✓ | 5.2 ms ✓ | 0.86× |
| families | qbf | classical auto j1 | qbf/16#1 | 1.1 ms ✗ | 8.0 ms ✗ | 7.13× |
| families | qbf | classical auto j1 | qbf/16#2 | 1.7 ms ✗ | 8.5 ms ✗ | 4.97× |
| families | qbf | classical auto j1 | qbf/16#3 | 5.4 ms ✓ | 4.4 ms ✓ | 0.82× |
| families | qbf | classical auto j1 | qbf/20#0 | 5.3 ms ✗ | 80.3 ms ✗ | 15.18× |
| families | qbf | classical auto j1 | qbf/20#1 | 8.2 ms ✗ | 82.2 ms ✗ | 10.07× |
| families | qbf | classical auto j1 | qbf/20#2 | 30.0 ms ✓ | 26.4 ms ✓ | 0.88× |
| families | qbf | classical auto j1 | qbf/20#3 | 5.1 ms ✗ | 80.5 ms ✗ | 15.78× |
| families | qbf | classical auto j1 | qbf/24#0 | 101.8 ms ✗ | 174.9 ms ✗ | 1.72× |
| families | qbf | classical auto j1 | qbf/24#1 | 37.0 ms ✗ | 120.5 ms ✗ | 3.26× |
| families | qbf | classical auto j1 | qbf/24#2 | 25.7 ms ✗ | 109.2 ms ✗ | 4.24× |
| families | qbf | classical auto j1 | qbf/24#3 | 36.1 ms ✗ | 120.3 ms ✗ | 3.33× |
| families | qbf | classical auto j1 | qbf/32#0 | 552.4 ms ✗ | 561.0 ms ✗ | 1.02× |
| families | qbf | classical auto j1 | qbf/32#1 | 626.7 ms ✗ | 593.4 ms ✗ | 0.95× |
| families | qbf | classical auto j1 | qbf/32#2 | 1.14 s ✗ | 1.03 s ✗ | 0.91× |
| families | qbf | classical auto j1 | qbf/32#3 | 476.7 ms ✗ | 487.3 ms ✗ | 1.02× |
| families | qbf | classical auto j1 | qbf/40#0 | 12.11 s ✗ | 10.00 s ✗ | 0.83× |
| families | qbf | classical auto j1 | qbf/40#1 | 13.65 s ✗ | 11.42 s ✗ | 0.84× |
| families | qbf | classical auto j1 | qbf/40#2 | 16.21 s ✗ | 13.33 s ✗ | 0.82× |
| families | qbf | classical auto j1 | qbf/40#3 | 12.54 s ✗ | 10.46 s ✗ | 0.83× |
| families | qbf | classical auto j1 | qbf/44#0 | 54.12 s ✗ | 44.75 s ✗ | 0.83× |
| families | qbf | classical auto j1 | qbf/44#1 | 53.43 s ✗ | 45.37 s ✗ | 0.85× |
| families | qbf | classical auto j1 | qbf/44#2 | 80.52 s ✗ | 66.52 s ✗ | 0.83× |
| families | qbf | classical auto j1 | qbf/44#3 | 28.71 s ? memory | 23.23 s ? memory |  |
| families | qbf | classical auto j1 | qbf/48#0 | 84.57 s ? memory | 70.03 s ? memory |  |
| families | qbf | classical auto j1 | qbf/48#1 | 207.33 s ✗ | 166.29 s ✗ | 0.80× |
| families | qbf | classical auto j1 | qbf/48#2 | 161.73 s ✗ | 141.52 s ✗ | 0.88× |
| families | qbf | classical auto j1 | qbf/48#3 | 222.46 s ✗ | 185.15 s ✗ | 0.83× |
| families | qbf | classical auto j1 | qbf/8#0 | 131 µs ✗ | 176 µs ✗ | 1.34× |
| families | qbf | classical auto j1 | qbf/8#1 | 144 µs ✗ | 190 µs ✗ | 1.32× |
| families | qbf | classical auto j1 | qbf/8#2 | 156 µs ✓ | 152 µs ✓ | 0.97× |
| families | qbf | classical auto j1 | qbf/8#3 | 130 µs ✗ | 179 µs ✗ | 1.38× |
| families | reach-fo | intuitionistic auto j1 | reach-fo/2#0 |  | 110 µs ✓ |  |
| families | reach-fo | intuitionistic auto j1 | reach-fo/2#1 |  | 123 µs ? bound |  |
| families | reach-fo | intuitionistic auto j1 | reach-fo/4#0 |  | 1.2 ms ✓ |  |
| families | reach-fo | intuitionistic auto j1 | reach-fo/4#1 |  | 1.6 ms ? bound |  |
| families | wide-m1 | classical auto j1 | wide-m1/16 | 110 µs ✓ | 66 µs ✓ | 0.60× |
| families | wide-m1 | classical auto j1 | wide-m1/2048 | 523.0 ms ✓ | 202.6 ms ✓ | 0.39× |
| families | wide-m1 | classical auto j1 | wide-m1/24 | 192 µs ✓ | 88 µs ✓ | 0.46× |
| families | wide-m1 | classical auto j1 | wide-m1/256 | 7.7 ms ✓ | 3.4 ms ✓ | 0.44× |
| families | wide-m1 | classical auto j1 | wide-m1/32 | 174 µs ✓ | 118 µs ✓ | 0.68× |
| families | wide-m1 | classical auto j1 | wide-m1/8 | 54 µs ✓ | 54 µs ✓ | 1.00× |
| families | wide-m2 | classical auto j1 | wide-m2/16 | 107 µs ✓ | 65 µs ✓ | 0.61× |
| families | wide-m2 | classical auto j1 | wide-m2/2048 | 555.5 ms ✓ | 186.2 ms ✓ | 0.34× |
| families | wide-m2 | classical auto j1 | wide-m2/24 | 194 µs ✓ | 84 µs ✓ | 0.43× |
| families | wide-m2 | classical auto j1 | wide-m2/256 | 8.0 ms ✓ | 2.6 ms ✓ | 0.33× |
| families | wide-m2 | classical auto j1 | wide-m2/32 | 171 µs ✓ | 106 µs ✓ | 0.62× |
| families | wide-m2 | classical auto j1 | wide-m2/8 | 54 µs ✓ | 54 µs ✓ | 1.00× |
| families | wide-m3 | classical auto j1 | wide-m3/1024 | 42.8 ms ✓ | 41.0 ms ✓ | 0.96× |
| families | wide-m3 | classical auto j1 | wide-m3/12 | 59 µs ✓ | 60 µs ✓ | 1.02× |
| families | wide-m3 | classical auto j1 | wide-m3/2048 | 172.9 ms ✓ | 163.2 ms ✓ | 0.94× |
| families | wide-m3 | classical auto j1 | wide-m3/24 | 87 µs ✓ | 80 µs ✓ | 0.92× |
| families | wide-m3 | classical auto j1 | wide-m3/256 | 2.7 ms ✓ | 2.4 ms ✓ | 0.86× |
| families | wide-m3 | classical auto j1 | wide-m3/30 | 104 µs ✓ | 100 µs ✓ | 0.96× |
| families | wide-m3 | classical auto j1 | wide-m3/36 | 128 µs ✓ | 114 µs ✓ | 0.89× |
| families | wide-m4 | classical auto j1 | wide-m4/1024 | 40.2 ms ✓ | 38.5 ms ✓ | 0.96× |
| families | wide-m4 | classical auto j1 | wide-m4/12 | 58 µs ✓ | 62 µs ✓ | 1.07× |
| families | wide-m4 | classical auto j1 | wide-m4/2048 | 167.4 ms ✓ | 154.0 ms ✓ | 0.92× |
| families | wide-m4 | classical auto j1 | wide-m4/24 | 92 µs ✓ | 82 µs ✓ | 0.89× |
| families | wide-m4 | classical auto j1 | wide-m4/256 | 2.5 ms ✓ | 2.2 ms ✓ | 0.89× |
| families | wide-m4 | classical auto j1 | wide-m4/28 | 97 µs ✓ | 92 µs ✓ | 0.95× |
| families | wide-m4 | classical auto j1 | wide-m4/32 | 108 µs ✓ | 104 µs ✓ | 0.96× |
| families | wide-m4 | classical auto j1 | wide-m4/36 | 134 µs ✓ | 117 µs ✓ | 0.87× |
| families | witness-fo | intuitionistic auto j1 | witness-fo/10#0 |  | 62 µs ✓ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/10#1 |  | 56 µs ✗ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/10#2 |  | 62 µs ✓ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/10#3 |  | 2.57 s ✗ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/12#0 |  | 66 µs ✓ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/12#1 |  | 61 µs ✗ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/12#2 |  | 69 µs ✓ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/12#3 |  | > 300 s |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/4#0 |  | 56 µs ✓ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/4#1 |  | 50 µs ✗ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/4#2 |  | 55 µs ✓ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/4#3 |  | 68 µs ✗ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/8#0 |  | 59 µs ✓ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/8#1 |  | 55 µs ✗ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/8#2 |  | 58 µs ✓ |  |
| families | witness-fo | intuitionistic auto j1 | witness-fo/8#3 |  | 28.2 ms ✗ |  |
| intuitionistic | 3-partition-no | intuitionistic auto j1 | 3-partition-no/4 | 273 µs ✗ | 275 µs ✗ | 1.01× |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 3-partition-yes/12 | 59 µs ✓ | 63 µs ✓ | 1.07× |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 3-partition-yes/4 | 60 µs ✓ | 58 µs ✓ | 0.97× |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 3-partition-yes/6 | 59 µs ✓ | 56 µs ✓ | 0.95× |
| intuitionistic | 3-partition-yes | intuitionistic auto j1 | 3-partition-yes/8 | 58 µs ✓ | 55 µs ✓ | 0.95× |
| intuitionistic | chain | intuitionistic auto j1 | chain/128 | 35.6 ms ✓ | 37.2 ms ✓ | 1.04× |
| intuitionistic | chain | intuitionistic auto j1 | chain/16 | 149 µs ✓ | 159 µs ✓ | 1.07× |
| intuitionistic | chain | intuitionistic auto j1 | chain/256 | 287.5 ms ✓ | 256.4 ms ✓ | 0.89× |
| intuitionistic | chain | intuitionistic auto j1 | chain/64 | 4.6 ms ✓ | 4.7 ms ✓ | 1.02× |
| intuitionistic | counter | intuitionistic auto j1 | counter/16 | 35 µs ✓ | 36 µs ✓ | 1.03× |
| intuitionistic | counter | intuitionistic auto j1 | counter/2 | 27 µs ✓ | 29 µs ✓ | 1.07× |
| intuitionistic | counter | intuitionistic auto j1 | counter/32 | 42 µs ✓ | 46 µs ✓ | 1.10× |
| intuitionistic | counter | intuitionistic auto j1 | counter/4 | 28 µs ✓ | 28 µs ✓ | 1.00× |
| intuitionistic | counter | intuitionistic auto j1 | counter/64 | 53 µs ✓ | 57 µs ✓ | 1.08× |
| intuitionistic | counter | intuitionistic auto j1 | counter/8 | 32 µs ✓ | 32 µs ✓ | 1.00× |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/16 | 35 µs ✗ | 39 µs ✗ | 1.11× |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/2 | 22 µs ✗ | 29 µs ✗ | 1.32× |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/32 | 44 µs ✗ | 40 µs ✗ | 0.91× |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/4 | 26 µs ✗ | 31 µs ✗ | 1.19× |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/64 | 54 µs ✗ | 48 µs ✗ | 0.89× |
| intuitionistic | counter-over | intuitionistic auto j1 | counter-over/8 | 32 µs ✗ | 35 µs ✗ | 1.09× |
| long-1 | 3-partition-no | classical auto j1 | 3-partition-no/5 | 400 µs ✗ | 514 µs ✗ | 1.28× |
| long-1 | counter | classical auto j1 | counter/16 | 48 µs ✓ | 67 µs ✓ | 1.40× |
| long-1 | counter | classical auto j1 | counter/64 | 60 µs ✓ | 76 µs ✓ | 1.27× |
| long-1 | partition-no | classical auto j1 | partition-no/15 | > 1200 s | > 1200 s |  |
| long-1 | partition-no | classical auto j1 | partition-no/5 | 441 µs ✗ | 558 µs ✗ | 1.27× |
| long-1 | partition-yes | classical auto j1 | partition-yes/28 | > 1200 s | > 1200 s |  |
| long-1 | partition-yes | classical auto j1 | partition-yes/7 | 214 µs ✓ | 289 µs ✓ | 1.35× |
| long-2 | mix | mix auto j1 | mix/11 | > 1200 s | 347.1 ms ✗ |  |
| long-2 | qbf | classical auto j1 | qbf/24#0 | 99.6 ms ✗ | 258.7 ms ✗ | 2.60× |
| long-2 | qbf | classical auto j1 | qbf/48#0 | 83.81 s ? memory | 101.59 s ? memory |  |
| long-2 | wide-m3 | classical auto j1 | wide-m3/36 | 154 µs ✓ | 173 µs ✓ | 1.12× |
| long-2 | wide-m4 | classical auto j1 | wide-m4/36 | 135 µs ✓ | 161 µs ✓ | 1.19× |
| parallel | 3-partition-no | classical auto j1 | 3-partition-no/4 | 379 µs ✗ | 391 µs ✗ | 1.03× |
| parallel | 3-partition-no | classical auto j1 | 3-partition-no/5 | 432 µs ✗ | 453 µs ✗ | 1.05× |
| parallel | 3-partition-no | classical auto j16 | 3-partition-no/4 | 631 µs ✗ | 671 µs ✗ | 1.06× |
| parallel | 3-partition-no | classical auto j16 | 3-partition-no/5 | 674 µs ✗ | 642 µs ✗ | 0.95× |
| parallel | 3-partition-no | classical auto j2 | 3-partition-no/4 | 544 µs ✗ | 531 µs ✗ | 0.98× |
| parallel | 3-partition-no | classical auto j2 | 3-partition-no/5 | 567 µs ✗ | 525 µs ✗ | 0.93× |
| parallel | 3-partition-no | classical auto j4 | 3-partition-no/4 | 428 µs ✗ | 425 µs ✗ | 0.99× |
| parallel | 3-partition-no | classical auto j4 | 3-partition-no/5 | 461 µs ✗ | 481 µs ✗ | 1.04× |
| parallel | 3-partition-no | classical auto j8 | 3-partition-no/4 | 550 µs ✗ | 471 µs ✗ | 0.86× |
| parallel | 3-partition-no | classical auto j8 | 3-partition-no/5 | 562 µs ✗ | 496 µs ✗ | 0.88× |
| parallel | 3-partition-yes | classical auto j1 | 3-partition-yes/12 | 88 µs ✓ | 88 µs ✓ | 1.00× |
| parallel | 3-partition-yes | classical auto j1 | 3-partition-yes/4 | 103 µs ✓ | 93 µs ✓ | 0.90× |
| parallel | 3-partition-yes | classical auto j1 | 3-partition-yes/6 | 86 µs ✓ | 85 µs ✓ | 0.99× |
| parallel | 3-partition-yes | classical auto j1 | 3-partition-yes/8 | 86 µs ✓ | 88 µs ✓ | 1.02× |
| parallel | 3-partition-yes | classical auto j16 | 3-partition-yes/12 | 546 µs ✓ | 693 µs ✓ | 1.27× |
| parallel | 3-partition-yes | classical auto j16 | 3-partition-yes/4 | 591 µs ✓ | 518 µs ✓ | 0.88× |
| parallel | 3-partition-yes | classical auto j16 | 3-partition-yes/6 | 593 µs ✓ | 546 µs ✓ | 0.92× |
| parallel | 3-partition-yes | classical auto j16 | 3-partition-yes/8 | 649 µs ✓ | 727 µs ✓ | 1.12× |
| parallel | 3-partition-yes | classical auto j2 | 3-partition-yes/12 | 248 µs ✓ | 206 µs ✓ | 0.83× |
| parallel | 3-partition-yes | classical auto j2 | 3-partition-yes/4 | 215 µs ✓ | 211 µs ✓ | 0.98× |
| parallel | 3-partition-yes | classical auto j2 | 3-partition-yes/6 | 220 µs ✓ | 228 µs ✓ | 1.04× |
| parallel | 3-partition-yes | classical auto j2 | 3-partition-yes/8 | 263 µs ✓ | 256 µs ✓ | 0.97× |
| parallel | 3-partition-yes | classical auto j4 | 3-partition-yes/12 | 301 µs ✓ | 254 µs ✓ | 0.84× |
| parallel | 3-partition-yes | classical auto j4 | 3-partition-yes/4 | 247 µs ✓ | 294 µs ✓ | 1.19× |
| parallel | 3-partition-yes | classical auto j4 | 3-partition-yes/6 | 269 µs ✓ | 309 µs ✓ | 1.15× |
| parallel | 3-partition-yes | classical auto j4 | 3-partition-yes/8 | 278 µs ✓ | 291 µs ✓ | 1.05× |
| parallel | 3-partition-yes | classical auto j8 | 3-partition-yes/12 | 458 µs ✓ | 369 µs ✓ | 0.81× |
| parallel | 3-partition-yes | classical auto j8 | 3-partition-yes/4 | 416 µs ✓ | 391 µs ✓ | 0.94× |
| parallel | 3-partition-yes | classical auto j8 | 3-partition-yes/6 | 337 µs ✓ | 420 µs ✓ | 1.25× |
| parallel | 3-partition-yes | classical auto j8 | 3-partition-yes/8 | 423 µs ✓ | 426 µs ✓ | 1.01× |
| parallel | counter | classical auto j1 | counter/16 | 63 µs ✓ | 56 µs ✓ | 0.89× |
| parallel | counter | classical auto j1 | counter/2 | 38 µs ✓ | 53 µs ✓ | 1.39× |
| parallel | counter | classical auto j1 | counter/32 | 72 µs ✓ | 71 µs ✓ | 0.99× |
| parallel | counter | classical auto j1 | counter/4 | 40 µs ✓ | 53 µs ✓ | 1.32× |
| parallel | counter | classical auto j1 | counter/64 | 86 µs ✓ | 79 µs ✓ | 0.92× |
| parallel | counter | classical auto j1 | counter/8 | 51 µs ✓ | 78 µs ✓ | 1.53× |
| parallel | counter | classical auto j16 | counter/16 | 91 µs ✓ | 88 µs ✓ | 0.97× |
| parallel | counter | classical auto j16 | counter/2 | 80 µs ✓ | 92 µs ✓ | 1.15× |
| parallel | counter | classical auto j16 | counter/32 | 102 µs ✓ | 101 µs ✓ | 0.99× |
| parallel | counter | classical auto j16 | counter/4 | 71 µs ✓ | 82 µs ✓ | 1.15× |
| parallel | counter | classical auto j16 | counter/64 | 116 µs ✓ | 117 µs ✓ | 1.01× |
| parallel | counter | classical auto j16 | counter/8 | 81 µs ✓ | 83 µs ✓ | 1.02× |
| parallel | counter | classical auto j2 | counter/16 | 83 µs ✓ | 88 µs ✓ | 1.06× |
| parallel | counter | classical auto j2 | counter/2 | 101 µs ✓ | 104 µs ✓ | 1.03× |
| parallel | counter | classical auto j2 | counter/32 | 94 µs ✓ | 101 µs ✓ | 1.07× |
| parallel | counter | classical auto j2 | counter/4 | 73 µs ✓ | 85 µs ✓ | 1.16× |
| parallel | counter | classical auto j2 | counter/64 | 106 µs ✓ | 124 µs ✓ | 1.17× |
| parallel | counter | classical auto j2 | counter/8 | 79 µs ✓ | 93 µs ✓ | 1.18× |
| parallel | counter | classical auto j4 | counter/16 | 93 µs ✓ | 100 µs ✓ | 1.08× |
| parallel | counter | classical auto j4 | counter/2 | 82 µs ✓ | 85 µs ✓ | 1.04× |
| parallel | counter | classical auto j4 | counter/32 | 96 µs ✓ | 100 µs ✓ | 1.04× |
| parallel | counter | classical auto j4 | counter/4 | 83 µs ✓ | 105 µs ✓ | 1.27× |
| parallel | counter | classical auto j4 | counter/64 | 102 µs ✓ | 113 µs ✓ | 1.11× |
| parallel | counter | classical auto j4 | counter/8 | 78 µs ✓ | 94 µs ✓ | 1.21× |
| parallel | counter | classical auto j8 | counter/16 | 93 µs ✓ | 107 µs ✓ | 1.15× |
| parallel | counter | classical auto j8 | counter/2 | 63 µs ✓ | 88 µs ✓ | 1.40× |
| parallel | counter | classical auto j8 | counter/32 | 100 µs ✓ | 108 µs ✓ | 1.08× |
| parallel | counter | classical auto j8 | counter/4 | 75 µs ✓ | 82 µs ✓ | 1.09× |
| parallel | counter | classical auto j8 | counter/64 | 114 µs ✓ | 123 µs ✓ | 1.08× |
| parallel | counter | classical auto j8 | counter/8 | 83 µs ✓ | 97 µs ✓ | 1.17× |
| parallel | counter-over | classical auto j1 | counter-over/16 | 63 µs ✗ | 57 µs ✗ | 0.90× |
| parallel | counter-over | classical auto j1 | counter-over/2 | 49 µs ✗ | 51 µs ✗ | 1.04× |
| parallel | counter-over | classical auto j1 | counter-over/32 | 73 µs ✗ | 69 µs ✗ | 0.95× |
| parallel | counter-over | classical auto j1 | counter-over/4 | 40 µs ✗ | 48 µs ✗ | 1.20× |
| parallel | counter-over | classical auto j1 | counter-over/64 | 70 µs ✗ | 68 µs ✗ | 0.97× |
| parallel | counter-over | classical auto j1 | counter-over/8 | 55 µs ✗ | 51 µs ✗ | 0.93× |
| parallel | counter-over | classical auto j16 | counter-over/16 | 86 µs ✗ | 92 µs ✗ | 1.07× |
| parallel | counter-over | classical auto j16 | counter-over/2 | 68 µs ✗ | 77 µs ✗ | 1.13× |
| parallel | counter-over | classical auto j16 | counter-over/32 | 95 µs ✗ | 96 µs ✗ | 1.01× |
| parallel | counter-over | classical auto j16 | counter-over/4 | 76 µs ✗ | 99 µs ✗ | 1.30× |
| parallel | counter-over | classical auto j16 | counter-over/64 | 98 µs ✗ | 100 µs ✗ | 1.02× |
| parallel | counter-over | classical auto j16 | counter-over/8 | 73 µs ✗ | 92 µs ✗ | 1.26× |
| parallel | counter-over | classical auto j2 | counter-over/16 | 84 µs ✗ | 95 µs ✗ | 1.13× |
| parallel | counter-over | classical auto j2 | counter-over/2 | 72 µs ✗ | 85 µs ✗ | 1.18× |
| parallel | counter-over | classical auto j2 | counter-over/32 | 96 µs ✗ | 92 µs ✗ | 0.96× |
| parallel | counter-over | classical auto j2 | counter-over/4 | 77 µs ✗ | 76 µs ✗ | 0.99× |
| parallel | counter-over | classical auto j2 | counter-over/64 | 98 µs ✗ | 126 µs ✗ | 1.29× |
| parallel | counter-over | classical auto j2 | counter-over/8 | 76 µs ✗ | 90 µs ✗ | 1.18× |
| parallel | counter-over | classical auto j4 | counter-over/16 | 77 µs ✗ | 106 µs ✗ | 1.38× |
| parallel | counter-over | classical auto j4 | counter-over/2 | 72 µs ✗ | 83 µs ✗ | 1.15× |
| parallel | counter-over | classical auto j4 | counter-over/32 | 94 µs ✗ | 108 µs ✗ | 1.15× |
| parallel | counter-over | classical auto j4 | counter-over/4 | 68 µs ✗ | 77 µs ✗ | 1.13× |
| parallel | counter-over | classical auto j4 | counter-over/64 | 88 µs ✗ | 119 µs ✗ | 1.35× |
| parallel | counter-over | classical auto j4 | counter-over/8 | 73 µs ✗ | 80 µs ✗ | 1.10× |
| parallel | counter-over | classical auto j8 | counter-over/16 | 80 µs ✗ | 91 µs ✗ | 1.14× |
| parallel | counter-over | classical auto j8 | counter-over/2 | 78 µs ✗ | 94 µs ✗ | 1.21× |
| parallel | counter-over | classical auto j8 | counter-over/32 | 94 µs ✗ | 98 µs ✗ | 1.04× |
| parallel | counter-over | classical auto j8 | counter-over/4 | 68 µs ✗ | 77 µs ✗ | 1.13× |
| parallel | counter-over | classical auto j8 | counter-over/64 | 91 µs ✗ | 103 µs ✗ | 1.13× |
| parallel | counter-over | classical auto j8 | counter-over/8 | 87 µs ✗ | 75 µs ✗ | 0.86× |
| parallel | mix | mix auto j1 | mix/10 | 62.10 s ✗ | 271.8 ms ✗ | 4.4e-3× |
| parallel | mix | mix auto j1 | mix/11 | > 120 s | 293.2 ms ✗ |  |
| parallel | mix | mix auto j1 | mix/8 | 857.8 ms ✗ | 267.9 ms ✗ | 0.31× |
| parallel | mix | mix auto j1 | mix/9 | 7.39 s ✗ | 271.5 ms ✗ | 3.7e-2× |
| parallel | mix | mix auto j16 | mix/10 | > 120 s | 108.11 s ✗ |  |
| parallel | mix | mix auto j16 | mix/11 | > 120 s | 120.01 s ✗ |  |
| parallel | mix | mix auto j16 | mix/8 | 1.61 s ✗ | 1.40 s ✗ | 0.87× |
| parallel | mix | mix auto j16 | mix/9 | 11.51 s ✗ | 9.70 s ✗ | 0.84× |
| parallel | mix | mix auto j2 | mix/10 | 91.83 s ✗ | 69.67 s ✗ | 0.76× |
| parallel | mix | mix auto j2 | mix/11 | > 120 s | 120.00 s ✗ |  |
| parallel | mix | mix auto j2 | mix/8 | 965.2 ms ✗ | 806.9 ms ✗ | 0.84× |
| parallel | mix | mix auto j2 | mix/9 | 7.05 s ✗ | 5.95 s ✗ | 0.84× |
| parallel | mix | mix auto j4 | mix/10 | 95.53 s ✗ | 72.29 s ✗ | 0.76× |
| parallel | mix | mix auto j4 | mix/11 | > 120 s | 120.01 s ✗ |  |
| parallel | mix | mix auto j4 | mix/8 | 1.05 s ✗ | 872.7 ms ✗ | 0.83× |
| parallel | mix | mix auto j4 | mix/9 | 7.90 s ✗ | 6.17 s ✗ | 0.78× |
| parallel | mix | mix auto j8 | mix/10 | 109.66 s ✗ | 84.78 s ✗ | 0.77× |
| parallel | mix | mix auto j8 | mix/11 | > 120 s | 120.01 s ✗ |  |
| parallel | mix | mix auto j8 | mix/8 | 1.48 s ✗ | 1.08 s ✗ | 0.73× |
| parallel | mix | mix auto j8 | mix/9 | 9.50 s ✗ | 7.93 s ✗ | 0.83× |
| parallel | partition-no | classical auto j1 | partition-no/12 | 3.59 s ✗ | 3.53 s ✗ | 0.98× |
| parallel | partition-no | classical auto j1 | partition-no/14 | > 120 s | > 120 s |  |
| parallel | partition-no | classical auto j1 | partition-no/4 | 192 µs ✗ | 198 µs ✗ | 1.03× |
| parallel | partition-no | classical auto j1 | partition-no/5 | 568 µs ✗ | 566 µs ✗ | 1.00× |
| parallel | partition-no | classical auto j16 | partition-no/12 | 581.6 ms ✗ | 579.3 ms ✗ | 1.00× |
| parallel | partition-no | classical auto j16 | partition-no/14 | > 120 s | > 120 s |  |
| parallel | partition-no | classical auto j16 | partition-no/4 | 600 µs ✗ | 666 µs ✗ | 1.11× |
| parallel | partition-no | classical auto j16 | partition-no/5 | 850 µs ✗ | 793 µs ✗ | 0.93× |
| parallel | partition-no | classical auto j2 | partition-no/12 | 2.13 s ✗ | 2.07 s ✗ | 0.97× |
| parallel | partition-no | classical auto j2 | partition-no/14 | > 120 s | > 120 s |  |
| parallel | partition-no | classical auto j2 | partition-no/4 | 405 µs ✗ | 411 µs ✗ | 1.01× |
| parallel | partition-no | classical auto j2 | partition-no/5 | 745 µs ✗ | 815 µs ✗ | 1.09× |
| parallel | partition-no | classical auto j4 | partition-no/12 | 1.24 s ✗ | 1.22 s ✗ | 0.98× |
| parallel | partition-no | classical auto j4 | partition-no/14 | > 120 s | > 120 s |  |
| parallel | partition-no | classical auto j4 | partition-no/4 | 347 µs ✗ | 351 µs ✗ | 1.01× |
| parallel | partition-no | classical auto j4 | partition-no/5 | 578 µs ✗ | 664 µs ✗ | 1.15× |
| parallel | partition-no | classical auto j8 | partition-no/12 | 806.0 ms ✗ | 805.1 ms ✗ | 1.00× |
| parallel | partition-no | classical auto j8 | partition-no/14 | > 120 s | > 120 s |  |
| parallel | partition-no | classical auto j8 | partition-no/4 | 418 µs ✗ | 439 µs ✗ | 1.05× |
| parallel | partition-no | classical auto j8 | partition-no/5 | 614 µs ✗ | 644 µs ✗ | 1.05× |
| parallel | partition-yes | classical auto j1 | partition-yes/20 | 396.5 ms ✓ | 389.6 ms ✓ | 0.98× |
| parallel | partition-yes | classical auto j1 | partition-yes/24 | 67.45 s ✓ | 67.11 s ✓ | 0.99× |
| parallel | partition-yes | classical auto j1 | partition-yes/5 | 117 µs ✓ | 128 µs ✓ | 1.09× |
| parallel | partition-yes | classical auto j1 | partition-yes/6 | 187 µs ✓ | 173 µs ✓ | 0.93× |
| parallel | partition-yes | classical auto j1 | partition-yes/7 | 273 µs ✓ | 277 µs ✓ | 1.01× |
| parallel | partition-yes | classical auto j16 | partition-yes/20 | 445.7 ms ✓ | 437.1 ms ✓ | 0.98× |
| parallel | partition-yes | classical auto j16 | partition-yes/24 | 87.42 s ✓ | 83.58 s ✓ | 0.96× |
| parallel | partition-yes | classical auto j16 | partition-yes/5 | 851 µs ✓ | 845 µs ✓ | 0.99× |
| parallel | partition-yes | classical auto j16 | partition-yes/6 | 906 µs ✓ | 621 µs ✓ | 0.69× |
| parallel | partition-yes | classical auto j16 | partition-yes/7 | 750 µs ✓ | 854 µs ✓ | 1.14× |
| parallel | partition-yes | classical auto j2 | partition-yes/20 | 418.5 ms ✓ | 416.1 ms ✓ | 0.99× |
| parallel | partition-yes | classical auto j2 | partition-yes/24 | 70.81 s ✓ | 69.59 s ✓ | 0.98× |
| parallel | partition-yes | classical auto j2 | partition-yes/5 | 311 µs ✓ | 321 µs ✓ | 1.03× |
| parallel | partition-yes | classical auto j2 | partition-yes/6 | 461 µs ✓ | 404 µs ✓ | 0.88× |
| parallel | partition-yes | classical auto j2 | partition-yes/7 | 585 µs ✓ | 514 µs ✓ | 0.88× |
| parallel | partition-yes | classical auto j4 | partition-yes/20 | 429.9 ms ✓ | 426.4 ms ✓ | 0.99× |
| parallel | partition-yes | classical auto j4 | partition-yes/24 | 74.75 s ✓ | 72.58 s ✓ | 0.97× |
| parallel | partition-yes | classical auto j4 | partition-yes/5 | 334 µs ✓ | 361 µs ✓ | 1.08× |
| parallel | partition-yes | classical auto j4 | partition-yes/6 | 413 µs ✓ | 377 µs ✓ | 0.91× |
| parallel | partition-yes | classical auto j4 | partition-yes/7 | 621 µs ✓ | 562 µs ✓ | 0.90× |
| parallel | partition-yes | classical auto j8 | partition-yes/20 | 439.9 ms ✓ | 419.2 ms ✓ | 0.95× |
| parallel | partition-yes | classical auto j8 | partition-yes/24 | 79.07 s ✓ | 76.81 s ✓ | 0.97× |
| parallel | partition-yes | classical auto j8 | partition-yes/5 | 461 µs ✓ | 409 µs ✓ | 0.89× |
| parallel | partition-yes | classical auto j8 | partition-yes/6 | 546 µs ✓ | 539 µs ✓ | 0.99× |
| parallel | partition-yes | classical auto j8 | partition-yes/7 | 597 µs ✓ | 625 µs ✓ | 1.05× |
| parallel | qbf | classical auto j1 | qbf/16#0 | 7.1 ms ✓ | 7.5 ms ✓ | 1.06× |
| parallel | qbf | classical auto j1 | qbf/16#1 | 1.8 ms ✗ | 11.8 ms ✗ | 6.48× |
| parallel | qbf | classical auto j1 | qbf/16#2 | 2.6 ms ✗ | 12.7 ms ✗ | 4.94× |
| parallel | qbf | classical auto j1 | qbf/16#3 | 8.4 ms ✓ | 6.6 ms ✓ | 0.79× |
| parallel | qbf | classical auto j1 | qbf/20#0 | 8.4 ms ✗ | 102.5 ms ✗ | 12.13× |
| parallel | qbf | classical auto j1 | qbf/20#1 | 14.3 ms ✗ | 101.5 ms ✗ | 7.10× |
| parallel | qbf | classical auto j1 | qbf/20#2 | 53.3 ms ✓ | 34.9 ms ✓ | 0.65× |
| parallel | qbf | classical auto j1 | qbf/20#3 | 8.1 ms ✗ | 97.5 ms ✗ | 12.06× |
| parallel | qbf | classical auto j1 | qbf/24#0 | 142.0 ms ✗ | 203.8 ms ✗ | 1.44× |
| parallel | qbf | classical auto j1 | qbf/24#1 | 68.0 ms ✗ | 139.4 ms ✗ | 2.05× |
| parallel | qbf | classical auto j1 | qbf/24#2 | 47.6 ms ✗ | 125.1 ms ✗ | 2.63× |
| parallel | qbf | classical auto j1 | qbf/24#3 | 57.9 ms ✗ | 139.9 ms ✗ | 2.42× |
| parallel | qbf | classical auto j1 | qbf/40#0 | 13.75 s ✗ | 10.51 s ✗ | 0.76× |
| parallel | qbf | classical auto j1 | qbf/40#1 | 13.41 s ✗ | 11.71 s ✗ | 0.87× |
| parallel | qbf | classical auto j1 | qbf/40#2 | 15.46 s ✗ | 13.56 s ✗ | 0.88× |
| parallel | qbf | classical auto j1 | qbf/40#3 | 11.55 s ✗ | 10.44 s ✗ | 0.90× |
| parallel | qbf | classical auto j1 | qbf/44#0 | 50.45 s ✗ | 44.79 s ✗ | 0.89× |
| parallel | qbf | classical auto j1 | qbf/44#1 | 49.78 s ✗ | 44.51 s ✗ | 0.89× |
| parallel | qbf | classical auto j1 | qbf/44#2 | 73.42 s ✗ | 67.34 s ✗ | 0.92× |
| parallel | qbf | classical auto j1 | qbf/44#3 | 26.33 s ? memory | 23.28 s ? memory |  |
| parallel | qbf | classical auto j16 | qbf/16#0 | 11.3 ms ✓ | 8.0 ms ✓ | 0.71× |
| parallel | qbf | classical auto j16 | qbf/16#1 | 2.0 ms ✗ | 12.2 ms ✗ | 6.26× |
| parallel | qbf | classical auto j16 | qbf/16#2 | 3.5 ms ✗ | 13.7 ms ✗ | 3.87× |
| parallel | qbf | classical auto j16 | qbf/16#3 | 9.5 ms ✓ | 7.4 ms ✓ | 0.78× |
| parallel | qbf | classical auto j16 | qbf/20#0 | 6.7 ms ✗ | 101.7 ms ✗ | 15.16× |
| parallel | qbf | classical auto j16 | qbf/20#1 | 13.7 ms ✗ | 101.2 ms ✗ | 7.37× |
| parallel | qbf | classical auto j16 | qbf/20#2 | 80.4 ms ✓ | 50.7 ms ✓ | 0.63× |
| parallel | qbf | classical auto j16 | qbf/20#3 | 5.2 ms ✗ | 102.2 ms ✗ | 19.75× |
| parallel | qbf | classical auto j16 | qbf/24#0 | 166.2 ms ✗ | 215.9 ms ✗ | 1.30× |
| parallel | qbf | classical auto j16 | qbf/24#1 | 45.5 ms ✗ | 131.7 ms ✗ | 2.90× |
| parallel | qbf | classical auto j16 | qbf/24#2 | 36.5 ms ✗ | 133.1 ms ✗ | 3.64× |
| parallel | qbf | classical auto j16 | qbf/24#3 | 43.1 ms ✗ | 135.8 ms ✗ | 3.15× |
| parallel | qbf | classical auto j16 | qbf/40#0 | 7.47 s ✗ | 6.49 s ✗ | 0.87× |
| parallel | qbf | classical auto j16 | qbf/40#1 | 5.53 s ? memory | 4.85 s ? memory |  |
| parallel | qbf | classical auto j16 | qbf/40#2 | 5.81 s ? memory | 5.14 s ? memory |  |
| parallel | qbf | classical auto j16 | qbf/40#3 | 7.51 s ✗ | 6.87 s ✗ | 0.91× |
| parallel | qbf | classical auto j16 | qbf/44#0 | 8.78 s ? memory | 7.90 s ? memory |  |
| parallel | qbf | classical auto j16 | qbf/44#1 | 7.42 s ? memory | 6.62 s ? memory |  |
| parallel | qbf | classical auto j16 | qbf/44#2 | 5.14 s ? memory | 4.67 s ? memory |  |
| parallel | qbf | classical auto j16 | qbf/44#3 | 8.73 s ? memory | 7.68 s ? memory |  |
| parallel | qbf | classical auto j2 | qbf/16#0 | 10.4 ms ✓ | 7.9 ms ✓ | 0.76× |
| parallel | qbf | classical auto j2 | qbf/16#1 | 2.5 ms ✗ | 11.4 ms ✗ | 4.56× |
| parallel | qbf | classical auto j2 | qbf/16#2 | 2.7 ms ✗ | 12.5 ms ✗ | 4.72× |
| parallel | qbf | classical auto j2 | qbf/16#3 | 7.3 ms ✓ | 7.4 ms ✓ | 1.01× |
| parallel | qbf | classical auto j2 | qbf/20#0 | 7.4 ms ✗ | 100.3 ms ✗ | 13.49× |
| parallel | qbf | classical auto j2 | qbf/20#1 | 11.0 ms ✗ | 100.4 ms ✗ | 9.11× |
| parallel | qbf | classical auto j2 | qbf/20#2 | 64.0 ms ✓ | 50.9 ms ✓ | 0.80× |
| parallel | qbf | classical auto j2 | qbf/20#3 | 7.8 ms ✗ | 96.5 ms ✗ | 12.34× |
| parallel | qbf | classical auto j2 | qbf/24#0 | 126.2 ms ✗ | 209.9 ms ✗ | 1.66× |
| parallel | qbf | classical auto j2 | qbf/24#1 | 37.2 ms ✗ | 138.0 ms ✗ | 3.71× |
| parallel | qbf | classical auto j2 | qbf/24#2 | 30.8 ms ✗ | 128.9 ms ✗ | 4.19× |
| parallel | qbf | classical auto j2 | qbf/24#3 | 39.2 ms ✗ | 136.9 ms ✗ | 3.49× |
| parallel | qbf | classical auto j2 | qbf/40#0 | 8.61 s ✗ | 6.81 s ✗ | 0.79× |
| parallel | qbf | classical auto j2 | qbf/40#1 | 5.56 s ? memory | 5.39 s ? memory |  |
| parallel | qbf | classical auto j2 | qbf/40#2 | 5.80 s ? memory | 5.10 s ? memory |  |
| parallel | qbf | classical auto j2 | qbf/40#3 | 7.46 s ✗ | 6.76 s ✗ | 0.91× |
| parallel | qbf | classical auto j2 | qbf/44#0 | 8.79 s ? memory | 7.70 s ? memory |  |
| parallel | qbf | classical auto j2 | qbf/44#1 | 7.31 s ? memory | 6.55 s ? memory |  |
| parallel | qbf | classical auto j2 | qbf/44#2 | 5.14 s ? memory | 4.62 s ? memory |  |
| parallel | qbf | classical auto j2 | qbf/44#3 | 8.56 s ? memory | 7.68 s ? memory |  |
| parallel | qbf | classical auto j4 | qbf/16#0 | 9.6 ms ✓ | 7.7 ms ✓ | 0.80× |
| parallel | qbf | classical auto j4 | qbf/16#1 | 1.6 ms ✗ | 11.8 ms ✗ | 7.31× |
| parallel | qbf | classical auto j4 | qbf/16#2 | 2.9 ms ✗ | 12.5 ms ✗ | 4.28× |
| parallel | qbf | classical auto j4 | qbf/16#3 | 8.5 ms ✓ | 7.0 ms ✓ | 0.83× |
| parallel | qbf | classical auto j4 | qbf/20#0 | 7.5 ms ✗ | 100.2 ms ✗ | 13.37× |
| parallel | qbf | classical auto j4 | qbf/20#1 | 10.0 ms ✗ | 98.5 ms ✗ | 9.86× |
| parallel | qbf | classical auto j4 | qbf/20#2 | 68.3 ms ✓ | 45.4 ms ✓ | 0.67× |
| parallel | qbf | classical auto j4 | qbf/20#3 | 6.5 ms ✗ | 95.0 ms ✗ | 14.57× |
| parallel | qbf | classical auto j4 | qbf/24#0 | 128.5 ms ✗ | 210.5 ms ✗ | 1.64× |
| parallel | qbf | classical auto j4 | qbf/24#1 | 38.4 ms ✗ | 133.1 ms ✗ | 3.47× |
| parallel | qbf | classical auto j4 | qbf/24#2 | 29.7 ms ✗ | 130.2 ms ✗ | 4.38× |
| parallel | qbf | classical auto j4 | qbf/24#3 | 37.5 ms ✗ | 133.3 ms ✗ | 3.55× |
| parallel | qbf | classical auto j4 | qbf/40#0 | 8.46 s ✗ | 6.78 s ✗ | 0.80× |
| parallel | qbf | classical auto j4 | qbf/40#1 | 5.53 s ? memory | 5.40 s ? memory |  |
| parallel | qbf | classical auto j4 | qbf/40#2 | 5.84 s ? memory | 5.14 s ? memory |  |
| parallel | qbf | classical auto j4 | qbf/40#3 | 7.57 s ✗ | 6.97 s ✗ | 0.92× |
| parallel | qbf | classical auto j4 | qbf/44#0 | 8.71 s ? memory | 7.72 s ? memory |  |
| parallel | qbf | classical auto j4 | qbf/44#1 | 7.44 s ? memory | 6.52 s ? memory |  |
| parallel | qbf | classical auto j4 | qbf/44#2 | 5.13 s ? memory | 4.60 s ? memory |  |
| parallel | qbf | classical auto j4 | qbf/44#3 | 8.59 s ? memory | 7.76 s ? memory |  |
| parallel | qbf | classical auto j8 | qbf/16#0 | 10.3 ms ✓ | 7.6 ms ✓ | 0.74× |
| parallel | qbf | classical auto j8 | qbf/16#1 | 3.0 ms ✗ | 11.7 ms ✗ | 3.91× |
| parallel | qbf | classical auto j8 | qbf/16#2 | 2.7 ms ✗ | 13.2 ms ✗ | 4.94× |
| parallel | qbf | classical auto j8 | qbf/16#3 | 11.2 ms ✓ | 7.3 ms ✓ | 0.66× |
| parallel | qbf | classical auto j8 | qbf/20#0 | 5.8 ms ✗ | 100.7 ms ✗ | 17.27× |
| parallel | qbf | classical auto j8 | qbf/20#1 | 11.2 ms ✗ | 100.7 ms ✗ | 8.99× |
| parallel | qbf | classical auto j8 | qbf/20#2 | 71.9 ms ✓ | 51.6 ms ✓ | 0.72× |
| parallel | qbf | classical auto j8 | qbf/20#3 | 8.6 ms ✗ | 97.8 ms ✗ | 11.36× |
| parallel | qbf | classical auto j8 | qbf/24#0 | 135.1 ms ✗ | 209.4 ms ✗ | 1.55× |
| parallel | qbf | classical auto j8 | qbf/24#1 | 43.4 ms ✗ | 135.2 ms ✗ | 3.12× |
| parallel | qbf | classical auto j8 | qbf/24#2 | 28.1 ms ✗ | 131.6 ms ✗ | 4.69× |
| parallel | qbf | classical auto j8 | qbf/24#3 | 38.6 ms ✗ | 133.7 ms ✗ | 3.46× |
| parallel | qbf | classical auto j8 | qbf/40#0 | 7.84 s ✗ | 6.57 s ✗ | 0.84× |
| parallel | qbf | classical auto j8 | qbf/40#1 | 5.52 s ? memory | 5.42 s ? memory |  |
| parallel | qbf | classical auto j8 | qbf/40#2 | 5.82 s ? memory | 5.13 s ? memory |  |
| parallel | qbf | classical auto j8 | qbf/40#3 | 7.44 s ✗ | 6.86 s ✗ | 0.92× |
| parallel | qbf | classical auto j8 | qbf/44#0 | 8.78 s ? memory | 7.72 s ? memory |  |
| parallel | qbf | classical auto j8 | qbf/44#1 | 7.34 s ? memory | 6.53 s ? memory |  |
| parallel | qbf | classical auto j8 | qbf/44#2 | 5.14 s ? memory | 4.58 s ? memory |  |
| parallel | qbf | classical auto j8 | qbf/44#3 | 8.70 s ? memory | 7.70 s ? memory |  |
| parallel | wide-m3 | classical auto j1 | wide-m3/24 | 135 µs ✓ | 152 µs ✓ | 1.13× |
| parallel | wide-m3 | classical auto j1 | wide-m3/30 | 182 µs ✓ | 153 µs ✓ | 0.84× |
| parallel | wide-m3 | classical auto j1 | wide-m3/36 | 192 µs ✓ | 174 µs ✓ | 0.91× |
| parallel | wide-m3 | classical auto j16 | wide-m3/24 | 714 µs ✓ | 679 µs ✓ | 0.95× |
| parallel | wide-m3 | classical auto j16 | wide-m3/30 | 807 µs ✓ | 695 µs ✓ | 0.86× |
| parallel | wide-m3 | classical auto j16 | wide-m3/36 | 727 µs ✓ | 727 µs ✓ | 1.00× |
| parallel | wide-m3 | classical auto j2 | wide-m3/24 | 357 µs ✓ | 301 µs ✓ | 0.84× |
| parallel | wide-m3 | classical auto j2 | wide-m3/30 | 316 µs ✓ | 337 µs ✓ | 1.07× |
| parallel | wide-m3 | classical auto j2 | wide-m3/36 | 415 µs ✓ | 350 µs ✓ | 0.84× |
| parallel | wide-m3 | classical auto j4 | wide-m3/24 | 315 µs ✓ | 328 µs ✓ | 1.04× |
| parallel | wide-m3 | classical auto j4 | wide-m3/30 | 390 µs ✓ | 362 µs ✓ | 0.93× |
| parallel | wide-m3 | classical auto j4 | wide-m3/36 | 405 µs ✓ | 372 µs ✓ | 0.92× |
| parallel | wide-m3 | classical auto j8 | wide-m3/24 | 489 µs ✓ | 483 µs ✓ | 0.99× |
| parallel | wide-m3 | classical auto j8 | wide-m3/30 | 503 µs ✓ | 459 µs ✓ | 0.91× |
| parallel | wide-m3 | classical auto j8 | wide-m3/36 | 453 µs ✓ | 519 µs ✓ | 1.15× |
| parallel | wide-m4 | classical auto j1 | wide-m4/28 | 141 µs ✓ | 139 µs ✓ | 0.99× |
| parallel | wide-m4 | classical auto j1 | wide-m4/32 | 186 µs ✓ | 152 µs ✓ | 0.82× |
| parallel | wide-m4 | classical auto j1 | wide-m4/36 | 190 µs ✓ | 196 µs ✓ | 1.03× |
| parallel | wide-m4 | classical auto j16 | wide-m4/28 | 727 µs ✓ | 682 µs ✓ | 0.94× |
| parallel | wide-m4 | classical auto j16 | wide-m4/32 | 807 µs ✓ | 679 µs ✓ | 0.84× |
| parallel | wide-m4 | classical auto j16 | wide-m4/36 | 722 µs ✓ | 702 µs ✓ | 0.97× |
| parallel | wide-m4 | classical auto j2 | wide-m4/28 | 326 µs ✓ | 315 µs ✓ | 0.97× |
| parallel | wide-m4 | classical auto j2 | wide-m4/32 | 369 µs ✓ | 306 µs ✓ | 0.83× |
| parallel | wide-m4 | classical auto j2 | wide-m4/36 | 430 µs ✓ | 335 µs ✓ | 0.78× |
| parallel | wide-m4 | classical auto j4 | wide-m4/28 | 342 µs ✓ | 334 µs ✓ | 0.98× |
| parallel | wide-m4 | classical auto j4 | wide-m4/32 | 368 µs ✓ | 381 µs ✓ | 1.04× |
| parallel | wide-m4 | classical auto j4 | wide-m4/36 | 435 µs ✓ | 403 µs ✓ | 0.93× |
| parallel | wide-m4 | classical auto j8 | wide-m4/28 | 487 µs ✓ | 469 µs ✓ | 0.96× |
| parallel | wide-m4 | classical auto j8 | wide-m4/32 | 538 µs ✓ | 507 µs ✓ | 0.94× |
| parallel | wide-m4 | classical auto j8 | wide-m4/36 | 535 µs ✓ | 489 µs ✓ | 0.91× |
| parallel-net | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/4 | 1.63 s ✗ | 368 µs ✗ | 2.3e-4× |
| parallel-net | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/5 | 48.79 s ✗ | 994 µs ✗ | 2.0e-5× |
| parallel-net | 3-partition-mll-no | classical net j1 | 3-partition-mll-no/6 | > 60 s | 1.6 ms ✗ |  |
| parallel-net | 3-partition-mll-no | classical net j16 | 3-partition-mll-no/4 | 138.2 ms ✗ | 814 µs ✗ | 5.9e-3× |
| parallel-net | 3-partition-mll-no | classical net j16 | 3-partition-mll-no/5 | 4.42 s ✗ | 1.4 ms ✗ | 3.1e-4× |
| parallel-net | 3-partition-mll-no | classical net j16 | 3-partition-mll-no/6 | > 60 s | 2.1 ms ✗ |  |
| parallel-net | 3-partition-mll-no | classical net j2 | 3-partition-mll-no/4 | 829.1 ms ✗ | 502 µs ✗ | 6.1e-4× |
| parallel-net | 3-partition-mll-no | classical net j2 | 3-partition-mll-no/5 | 25.09 s ✗ | 1.1 ms ✗ | 4.4e-5× |
| parallel-net | 3-partition-mll-no | classical net j2 | 3-partition-mll-no/6 | > 60 s | 1.2 ms ✗ |  |
| parallel-net | 3-partition-mll-no | classical net j4 | 3-partition-mll-no/4 | 443.0 ms ✗ | 551 µs ✗ | 1.2e-3× |
| parallel-net | 3-partition-mll-no | classical net j4 | 3-partition-mll-no/5 | 13.41 s ✗ | 1.1 ms ✗ | 8.4e-5× |
| parallel-net | 3-partition-mll-no | classical net j4 | 3-partition-mll-no/6 | > 60 s | 1.8 ms ✗ |  |
| parallel-net | 3-partition-mll-no | classical net j8 | 3-partition-mll-no/4 | 246.5 ms ✗ | 583 µs ✗ | 2.4e-3× |
| parallel-net | 3-partition-mll-no | classical net j8 | 3-partition-mll-no/5 | 7.58 s ✗ | 1.2 ms ✗ | 1.6e-4× |
| parallel-net | 3-partition-mll-no | classical net j8 | 3-partition-mll-no/6 | > 60 s | 1.9 ms ✗ |  |
| parallel-net | partition-table | classical net j1 | partition-table/1-1 | 72 µs ✓ | 73 µs ✓ | 1.01× |
| parallel-net | partition-table | classical net j1 | partition-table/1-1-1-1-1-7 | > 60 s | 113.1 ms ✗ |  |
| parallel-net | partition-table | classical net j1 | partition-table/1-1-1-5 | 7.13 s ✗ | 4.2 ms ✗ | 5.8e-4× |
| parallel-net | partition-table | classical net j1 | partition-table/1-1-2-4 | 1.01 s ✓ | 1.2 ms ✓ | 1.2e-3× |
| parallel-net | partition-table | classical net j1 | partition-table/1-1-4 | 46.5 ms ✗ | 782 µs ✗ | 1.7e-2× |
| parallel-net | partition-table | classical net j1 | partition-table/1-2-3-4-5-5 | > 60 s | 1.04 s ✓ |  |
| parallel-net | partition-table | classical net j1 | partition-table/1-2-5 | 3.66 s ✗ | 1.7 ms ✗ | 4.6e-4× |
| parallel-net | partition-table | classical net j1 | partition-table/1-3 | 488 µs ✗ | 182 µs ✗ | 0.37× |
| parallel-net | partition-table | classical net j1 | partition-table/2-1-1 | 802 µs ✓ | 273 µs ✓ | 0.34× |
| parallel-net | partition-table | classical net j1 | partition-table/2-2-1-1 | 62.3 ms ✓ | 1.3 ms ✓ | 2.1e-2× |
| parallel-net | partition-table | classical net j1 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | 3.91 s ✓ |  |
| parallel-net | partition-table | classical net j1 | partition-table/2-3-2-1 | 4.24 s ✓ | 3.4 ms ✓ | 7.9e-4× |
| parallel-net | partition-table | classical net j1 | partition-table/3-3-3-1 | > 60 s | 10.5 ms ✗ |  |
| parallel-net | partition-table | classical net j16 | partition-table/1-1 | 508 µs ✓ | 513 µs ✓ | 1.01× |
| parallel-net | partition-table | classical net j16 | partition-table/1-1-1-1-1-7 | > 60 s | 12.1 ms ✗ |  |
| parallel-net | partition-table | classical net j16 | partition-table/1-1-1-5 | 649.9 ms ✗ | 4.3 ms ✗ | 6.6e-3× |
| parallel-net | partition-table | classical net j16 | partition-table/1-1-2-4 | 74.5 ms ✓ | 5.8 ms ✓ | 7.8e-2× |
| parallel-net | partition-table | classical net j16 | partition-table/1-1-4 | 4.7 ms ✗ | 1.2 ms ✗ | 0.25× |
| parallel-net | partition-table | classical net j16 | partition-table/1-2-3-4-5-5 | > 60 s | 6.7 ms ✓ |  |
| parallel-net | partition-table | classical net j16 | partition-table/1-2-5 | 291.2 ms ✗ | 2.2 ms ✗ | 7.4e-3× |
| parallel-net | partition-table | classical net j16 | partition-table/1-3 | 965 µs ✗ | 587 µs ✗ | 0.61× |
| parallel-net | partition-table | classical net j16 | partition-table/2-1-1 | 1.4 ms ✓ | 767 µs ✓ | 0.55× |
| parallel-net | partition-table | classical net j16 | partition-table/2-2-1-1 | 6.0 ms ✓ | 2.5 ms ✓ | 0.42× |
| parallel-net | partition-table | classical net j16 | partition-table/2-2-2-2-2-2-9-1 | 977.2 ms ✓ | 3.1 ms ✓ | 3.2e-3× |
| parallel-net | partition-table | classical net j16 | partition-table/2-3-2-1 | 336.8 ms ✓ | 5.3 ms ✓ | 1.6e-2× |
| parallel-net | partition-table | classical net j16 | partition-table/3-3-3-1 | > 60 s | 4.4 ms ✗ |  |
| parallel-net | partition-table | classical net j2 | partition-table/1-1 | 180 µs ✓ | 205 µs ✓ | 1.14× |
| parallel-net | partition-table | classical net j2 | partition-table/1-1-1-1-1-7 | > 60 s | 61.4 ms ✗ |  |
| parallel-net | partition-table | classical net j2 | partition-table/1-1-1-5 | 3.49 s ✗ | 2.6 ms ✗ | 7.4e-4× |
| parallel-net | partition-table | classical net j2 | partition-table/1-1-2-4 | 496.8 ms ✓ | 1.2 ms ✓ | 2.4e-3× |
| parallel-net | partition-table | classical net j2 | partition-table/1-1-4 | 24.7 ms ✗ | 851 µs ✗ | 3.4e-2× |
| parallel-net | partition-table | classical net j2 | partition-table/1-2-3-4-5-5 | > 60 s | 122.6 ms ✓ |  |
| parallel-net | partition-table | classical net j2 | partition-table/1-2-5 | 1.82 s ✗ | 1.2 ms ✗ | 6.7e-4× |
| parallel-net | partition-table | classical net j2 | partition-table/1-3 | 584 µs ✗ | 292 µs ✗ | 0.50× |
| parallel-net | partition-table | classical net j2 | partition-table/2-1-1 | 726 µs ✓ | 471 µs ✓ | 0.65× |
| parallel-net | partition-table | classical net j2 | partition-table/2-2-1-1 | 30.6 ms ✓ | 1.0 ms ✓ | 3.3e-2× |
| parallel-net | partition-table | classical net j2 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | 1.37 s ✓ |  |
| parallel-net | partition-table | classical net j2 | partition-table/2-3-2-1 | 2.18 s ✓ | 2.0 ms ✓ | 9.4e-4× |
| parallel-net | partition-table | classical net j2 | partition-table/3-3-3-1 | > 60 s | 5.8 ms ✗ |  |
| parallel-net | partition-table | classical net j4 | partition-table/1-1 | 199 µs ✓ | 232 µs ✓ | 1.17× |
| parallel-net | partition-table | classical net j4 | partition-table/1-1-1-1-1-7 | > 60 s | 34.1 ms ✗ |  |
| parallel-net | partition-table | classical net j4 | partition-table/1-1-1-5 | 1.90 s ✗ | 1.7 ms ✗ | 8.9e-4× |
| parallel-net | partition-table | classical net j4 | partition-table/1-1-2-4 | 251.8 ms ✓ | 802 µs ✓ | 3.2e-3× |
| parallel-net | partition-table | classical net j4 | partition-table/1-1-4 | 12.9 ms ✗ | 919 µs ✗ | 7.1e-2× |
| parallel-net | partition-table | classical net j4 | partition-table/1-2-3-4-5-5 | 2.0 ms ✓ | 1.6 ms ✓ | 0.81× |
| parallel-net | partition-table | classical net j4 | partition-table/1-2-5 | 919.5 ms ✗ | 1.9 ms ✗ | 2.1e-3× |
| parallel-net | partition-table | classical net j4 | partition-table/1-3 | 674 µs ✗ | 311 µs ✗ | 0.46× |
| parallel-net | partition-table | classical net j4 | partition-table/2-1-1 | 1.2 ms ✓ | 493 µs ✓ | 0.41× |
| parallel-net | partition-table | classical net j4 | partition-table/2-2-1-1 | 16.6 ms ✓ | 1.2 ms ✓ | 7.4e-2× |
| parallel-net | partition-table | classical net j4 | partition-table/2-2-2-2-2-2-9-1 | > 60 s | 1.20 s ✓ |  |
| parallel-net | partition-table | classical net j4 | partition-table/2-3-2-1 | 1.06 s ✓ | 1.4 ms ✓ | 1.3e-3× |
| parallel-net | partition-table | classical net j4 | partition-table/3-3-3-1 | > 60 s | 3.4 ms ✗ |  |
| parallel-net | partition-table | classical net j8 | partition-table/1-1 | 346 µs ✓ | 273 µs ✓ | 0.79× |
| parallel-net | partition-table | classical net j8 | partition-table/1-1-1-1-1-7 | > 60 s | 20.3 ms ✗ |  |
| parallel-net | partition-table | classical net j8 | partition-table/1-1-1-5 | 1.08 s ✗ | 2.7 ms ✗ | 2.5e-3× |
| parallel-net | partition-table | classical net j8 | partition-table/1-1-2-4 | 141.7 ms ✓ | 2.3 ms ✓ | 1.7e-2× |
| parallel-net | partition-table | classical net j8 | partition-table/1-1-4 | 7.6 ms ✗ | 961 µs ✗ | 0.13× |
| parallel-net | partition-table | classical net j8 | partition-table/1-2-3-4-5-5 | > 60 s | 54.2 ms ✓ |  |
| parallel-net | partition-table | classical net j8 | partition-table/1-2-5 | 537.3 ms ✗ | 2.0 ms ✗ | 3.7e-3× |
| parallel-net | partition-table | classical net j8 | partition-table/1-3 | 724 µs ✗ | 394 µs ✗ | 0.54× |
| parallel-net | partition-table | classical net j8 | partition-table/2-1-1 | 1.3 ms ✓ | 597 µs ✓ | 0.47× |
| parallel-net | partition-table | classical net j8 | partition-table/2-2-1-1 | 8.7 ms ✓ | 2.3 ms ✓ | 0.26× |
| parallel-net | partition-table | classical net j8 | partition-table/2-2-2-2-2-2-9-1 | 815.1 ms ✓ | 1.6 ms ✓ | 2.0e-3× |
| parallel-net | partition-table | classical net j8 | partition-table/2-3-2-1 | 626.7 ms ✓ | 2.3 ms ✓ | 3.7e-3× |
| parallel-net | partition-table | classical net j8 | partition-table/3-3-3-1 | > 60 s | 4.7 ms ✗ |  |
| period-1 | 3-partition-mll-no | classical net j1 period 1 | 3-partition-mll-no/4 | 1.73 s ✗ | 259 µs ✗ | 1.5e-4× |
| period-1 | 3-partition-mll-no | classical net j1 period 1 | 3-partition-mll-no/5 | 55.13 s ✗ | 681 µs ✗ | 1.2e-5× |
| period-1 | partition-no | classical net j1 period 1 | partition-no/3 | 3.32 s ✗ | 1.5 ms ✗ | 4.4e-4× |
| period-1 | partition-no | classical net j1 period 1 | partition-no/4 | > 60 s | 9.3 ms ✗ |  |
| period-1 | partition-yes | classical net j1 period 1 | partition-yes/4 | 2.69 s ✓ | 1.7 ms ✓ | 6.5e-4× |
| period-1 | wide-m1 | classical net j1 period 1 | wide-m1/2048 | 1.22 s ✓ | 1.26 s ✓ | 1.03× |
| period-1 | wide-m1 | classical net j1 period 1 | wide-m1/256 | 17.9 ms ✓ | 18.5 ms ✓ | 1.03× |
| period-1 | wide-m2 | classical net j1 period 1 | wide-m2/2048 | 1.23 s ✓ | 1.25 s ✓ | 1.01× |
| period-1 | wide-m2 | classical net j1 period 1 | wide-m2/256 | 18.5 ms ✓ | 18.3 ms ✓ | 0.99× |
| period-16 | 3-partition-mll-no | classical net j1 period 16 | 3-partition-mll-no/4 | 20.95 s ✗ | 259 µs ✗ | 1.2e-5× |
| period-16 | 3-partition-mll-no | classical net j1 period 16 | 3-partition-mll-no/5 | > 60 s | 1.2 ms ✗ |  |
| period-16 | partition-no | classical net j1 period 16 | partition-no/3 | > 60 s | 2.2 ms ✗ |  |
| period-16 | partition-no | classical net j1 period 16 | partition-no/4 | > 60 s | 23.5 ms ✗ |  |
| period-16 | partition-yes | classical net j1 period 16 | partition-yes/4 | > 60 s | 2.7 ms ✓ |  |
| period-16 | wide-m1 | classical net j1 period 16 | wide-m1/2048 | 388.2 ms ✓ | 427.8 ms ✓ | 1.10× |
| period-16 | wide-m1 | classical net j1 period 16 | wide-m1/256 | 5.9 ms ✓ | 6.4 ms ✓ | 1.09× |
| period-16 | wide-m2 | classical net j1 period 16 | wide-m2/2048 | 405.9 ms ✓ | 420.0 ms ✓ | 1.03× |
| period-16 | wide-m2 | classical net j1 period 16 | wide-m2/256 | 6.2 ms ✓ | 6.5 ms ✓ | 1.05× |
| period-2 | 3-partition-mll-no | classical net j1 period 2 | 3-partition-mll-no/4 | 1.38 s ✗ | 235 µs ✗ | 1.7e-4× |
| period-2 | 3-partition-mll-no | classical net j1 period 2 | 3-partition-mll-no/5 | 49.10 s ✗ | 695 µs ✗ | 1.4e-5× |
| period-2 | partition-no | classical net j1 period 2 | partition-no/3 | 7.45 s ✗ | 1.2 ms ✗ | 1.6e-4× |
| period-2 | partition-no | classical net j1 period 2 | partition-no/4 | > 60 s | 7.7 ms ✗ |  |
| period-2 | partition-yes | classical net j1 period 2 | partition-yes/4 | 5.72 s ✓ | 1.3 ms ✓ | 2.3e-4× |
| period-2 | wide-m1 | classical net j1 period 2 | wide-m1/2048 | 818.1 ms ✓ | 809.0 ms ✓ | 0.99× |
| period-2 | wide-m1 | classical net j1 period 2 | wide-m1/256 | 12.2 ms ✓ | 12.0 ms ✓ | 0.99× |
| period-2 | wide-m2 | classical net j1 period 2 | wide-m2/2048 | 803.6 ms ✓ | 805.6 ms ✓ | 1.00× |
| period-2 | wide-m2 | classical net j1 period 2 | wide-m2/256 | 12.0 ms ✓ | 12.2 ms ✓ | 1.01× |
| period-8 | 3-partition-mll-no | classical net j1 period 8 | 3-partition-mll-no/4 | 4.78 s ✗ | 246 µs ✗ | 5.2e-5× |
| period-8 | 3-partition-mll-no | classical net j1 period 8 | 3-partition-mll-no/5 | > 60 s | 709 µs ✗ |  |
| period-8 | partition-no | classical net j1 period 8 | partition-no/3 | > 60 s | 1.4 ms ✗ |  |
| period-8 | partition-no | classical net j1 period 8 | partition-no/4 | > 60 s | 22.4 ms ✗ |  |
| period-8 | partition-yes | classical net j1 period 8 | partition-yes/4 | > 60 s | 5.0 ms ✓ |  |
| period-8 | wide-m1 | classical net j1 period 8 | wide-m1/2048 | 448.2 ms ✓ | 481.0 ms ✓ | 1.07× |
| period-8 | wide-m1 | classical net j1 period 8 | wide-m1/256 | 7.0 ms ✓ | 7.3 ms ✓ | 1.05× |
| period-8 | wide-m2 | classical net j1 period 8 | wide-m2/2048 | 463.3 ms ✓ | 475.0 ms ✓ | 1.03× |
| period-8 | wide-m2 | classical net j1 period 8 | wide-m2/256 | 7.0 ms ✓ | 7.2 ms ✓ | 1.02× |

## The passes under one bias against the default

In the fourth baseline the two passes under one bias ran on CPUs 4 and 5, efficiency cores, and the default pass on CPU 2, a performance core, so the ratios of their times measure the cores as much as the searches ("Where the fourth baseline ran differently").

### lltp-rarer against lltp-intuitionistic

#### Verdicts that differ

None.

#### Counters that differ

Of the 3662 problems both decide alike on one thread, 105 differ in nodes, splits, memo_hits, memo_entries, links, tests.

#### Decided by lltp-rarer, not by lltp-intuitionistic

None.

#### Decided by lltp-intuitionistic, not by lltp-rarer

| file | family | configuration | problem | lltp-rarer | lltp-intuitionistic |
|---|---|---|---|--:|--:|
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.002.p | 75 µs ? bound | 110 µs ✗ |

#### By group

Problems in both; decided (late: after the time limit); decided by one and not the other; where the undecided ended (`time` the time limit or `killed` past it, `bound` the copy bound, `depth` the recursion limit, `memory` the memory limit, `index` an index past its width, `wide` a context too wide to split, `crash` out of memory); the median ratio of lltp-intuitionistic's time to lltp-rarer's on the problems both decide within the limit (how many); and the problems only lltp-intuitionistic has.

| file | family | configuration | problems | lltp-rarer decided | lltp-intuitionistic decided | only lltp-rarer | only lltp-intuitionistic | lltp-rarer ended | lltp-intuitionistic ended | lltp-intuitionistic/lltp-rarer | new |
|---|---|---|--:|--:|--:|--:|--:|---|---|--:|--:|
|  | ILL/ILLTP-LCL-01 | intuitionistic auto j1 | 2 | 1 | 1 | 0 | 0 | 1 bound | 1 bound | 0.79× (1) | 0 |
|  | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.91× (2) | 0 |
|  | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 | 2 | 1 | 1 | 0 | 0 | 1 bound | 1 bound | 0.79× (1) | 0 |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 252 | 34 | 34 | 0 | 0 | 5 time, 189 bound, 22 depth, 2 memory | 4 time, 189 bound, 22 depth, 3 memory | 0.78× (34) | 0 |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 252 | 37 | 38 | 0 | 1 | 7 time, 185 bound, 22 depth, 1 memory | 4 time, 186 bound, 22 depth, 2 memory | 1.35× (37) | 0 |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 252 | 43 | 43 | 0 | 0 | 24 time, 173 bound, 10 depth, 2 memory | 23 time, 174 bound, 10 depth, 2 memory | 0.76× (43) | 0 |
|  | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | 19 | 14 | 14 | 0 | 0 | 5 bound | 5 bound | 0.71× (14) | 0 |
|  | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 | 19 | 18 | 18 | 0 | 0 | 1 bound | 1 bound | 0.86× (18) | 0 |
|  | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 | 19 | 18 | 18 | 0 | 0 | 1 bound | 1 bound | 0.78× (18) | 0 |
|  | ILL/KLE-01 | intuitionistic auto j1 | 88 | 58 | 58 | 0 | 0 | 30 bound | 30 bound | 0.77× (58) | 0 |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | 222 | 162 | 162 | 0 | 0 | 60 bound | 60 bound | 0.80× (162) | 0 |
|  | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j1 | 27 | 27 | 27 | 0 | 0 | – | – | 0.74× (27) | 0 |
|  | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j1 | 22 | 22 | 22 | 0 | 0 | – | – | 0.68× (22) | 0 |
|  | ILL/KLE-cbn | intuitionistic auto j1 | 88 | 76 | 76 | 0 | 0 | 12 bound | 12 bound | 0.84× (76) | 0 |
|  | ILL/KLE-cbv | intuitionistic auto j1 | 88 | 75 | 75 | 0 | 0 | 13 bound | 13 bound | 0.72× (75) | 0 |
|  | ILL/Non-theorems | intuitionistic auto j1 | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
|  | ILL/misc | intuitionistic auto j1 | 3 | 3 | 3 | 0 | 0 | – | – | 0.83× (3) | 0 |
|  | ILL/petri-nets/MCC | intuitionistic auto j1 | 3137 | 3071 | 3071 | 0 | 0 | 64 time, 2 memory | 57 time, 9 memory | 0.75× (3071) | 0 |

### lltp-forward against lltp-intuitionistic

#### Verdicts that differ

None.

#### Counters that differ

Of the 3663 problems both decide alike on one thread, 53 differ in nodes, splits, memo_hits, memo_entries, links, tests.

#### Decided by lltp-forward, not by lltp-intuitionistic

| file | family | configuration | problem | lltp-forward | lltp-intuitionistic |
|---|---|---|---|--:|--:|
|  | ILL/ILLTP-LCL-01 | intuitionistic auto j1 | LCL230+1.p | 101 µs ✗ | 75 µs ? bound |
|  | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 | LCL230+1.p | 110 µs ✗ | 75 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ106+1.p | 202 µs ✓ | 90 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ107+1.002.p | 737 µs ✓ | 107 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ107+1.003.p | 8.3 ms ✓ | 135 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ107+1.004.p | 91.5 ms ✓ | 171 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ201+1.001.p | 1.3 ms ✓ | 127 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ201+1.002.p | 101.7 ms ✓ | 242 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ202+1.002.p | 680 µs ✓ | 282 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ202+1.003.p | 179.7 ms ✓ | 1.6 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.002.p | 129 µs ✓ | 88 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.003.p | 236 µs ✓ | 120 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.004.p | 647 µs ✓ | 145 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.005.p | 2.2 ms ✓ | 200 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.006.p | 8.1 ms ✓ | 315 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.007.p | 30.4 ms ✓ | 411 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.008.p | 129.8 ms ✓ | 585 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.009.p | 179.7 ms ✓ | 854 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.010.p | 193.5 ms ✓ | 1.5 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.011.p | 200.4 ms ✓ | 2.5 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.012.p | 199.1 ms ✓ | 4.6 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.013.p | 201.1 ms ✓ | 9.1 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.014.p | 227.0 ms ✓ | 18.0 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.015.p | 235.8 ms ✓ | 36.5 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.016.p | 247.3 ms ✓ | 74.4 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.017.p | 235.2 ms ✓ | 113.6 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.018.p | 542.5 ms ✓ | 113.9 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.019.p | 547.0 ms ✓ | 112.5 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ203+1.020.p | 557.9 ms ✓ | 111.1 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.002.p | 86 µs ✓ | 52 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.003.p | 105 µs ✓ | 60 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.004.p | 354 µs ✓ | 61 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.005.p | 493 µs ✓ | 65 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.006.p | 633 µs ✓ | 71 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.007.p | 1.0 ms ✓ | 72 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.008.p | 15.8 ms ✓ | 71 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.009.p | 19.6 ms ✓ | 89 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.010.p | 23.9 ms ✓ | 85 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.011.p | 29.2 ms ✓ | 102 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.012.p | 34.4 ms ✓ | 97 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.013.p | 49.0 ms ✓ | 113 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.014.p | 57.4 ms ✓ | 122 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.015.p | 66.8 ms ✓ | 118 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.016.p | 108.3 ms ✓ | 132 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.017.p | 110.4 ms ✓ | 142 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.018.p | 107.8 ms ✓ | 159 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.019.p | 109.9 ms ✓ | 152 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ204+1.020.p | 107.5 ms ✓ | 163 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ205+1.001.p | 252 µs ✓ | 58 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ205+1.002.p | 997 µs ✓ | 66 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ205+1.003.p | 3.6 ms ✓ | 69 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ205+1.004.p | 17.7 ms ✓ | 68 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ205+1.005.p | 60.2 ms ✓ | 71 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ205+1.006.p | 191.3 ms ✓ | 74 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ205+1.007.p | 639.5 ms ✓ | 73 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ205+1.008.p | 2.41 s ✓ | 76 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ206+1.003.p | 702 µs ✓ | 137 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ206+1.004.p | 14.8 ms ✓ | 209 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ206+1.005.p | 517.5 ms ✓ | 329 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ207+1.001.p | 429 µs ✗ | 119 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ207+1.002.p | 89.3 ms ✗ | 192 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ208+1.001.p | 290 µs ✗ | 75 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ208+1.002.p | 4.5 ms ✗ | 321 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ208+1.003.p | 1.11 s ✗ | 1.7 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.002.p | 104 µs ✗ | 78 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.003.p | 265 µs ✗ | 98 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.004.p | 865 µs ✗ | 144 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.005.p | 3.6 ms ✗ | 184 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.006.p | 15.0 ms ✗ | 252 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.007.p | 65.3 ms ✗ | 375 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.008.p | 151.6 ms ✗ | 524 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.009.p | 162.4 ms ✗ | 867 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.010.p | 193.5 ms ✗ | 1.4 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.011.p | 191.8 ms ✗ | 2.5 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.012.p | 209.5 ms ✗ | 4.7 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.013.p | 222.5 ms ✗ | 9.5 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.014.p | 231.7 ms ✗ | 19.0 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.015.p | 270.4 ms ✗ | 38.7 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.016.p | 329.9 ms ✗ | 79.8 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.017.p | 671.8 ms ✗ | 116.9 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.018.p | 670.0 ms ✗ | 117.9 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.019.p | 680.9 ms ✗ | 115.8 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ209+1.020.p | 717.0 ms ✗ | 114.6 ms ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.001.p | 159 µs ✗ | 63 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.002.p | 849 µs ✗ | 58 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.003.p | 6.6 ms ✗ | 62 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.004.p | 42.1 ms ✗ | 67 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.005.p | 106.3 ms ✗ | 74 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.006.p | 122.4 ms ✗ | 75 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.007.p | 137.7 ms ✗ | 79 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.008.p | 283.6 ms ✗ | 84 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.009.p | 285.4 ms ✗ | 88 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.010.p | 561.3 ms ✗ | 88 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.011.p | 563.2 ms ✗ | 104 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.012.p | 1.08 s ✗ | 122 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.013.p | 1.07 s ✗ | 112 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.014.p | 2.19 s ✗ | 124 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.015.p | 2.18 s ✗ | 131 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.016.p | 2.18 s ✗ | 140 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.017.p | 4.20 s ✗ | 158 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.018.p | 4.16 s ✗ | 155 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ210+1.019.p | 4.27 s ✗ | 162 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ212+1.001.p | 143 µs ✓ MISMATCH | 54 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ212+1.002.p | 203 µs ✗ | 79 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ212+1.003.p | 11.3 ms ✗ | 127 µs ? bound |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | SYJ212+1.004.p | 770.1 ms ✗ | 210 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ106+1.p | 83 µs ✓ | 148 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ107+1.002.p | 359 µs ✓ | 88 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ107+1.003.p | 2.9 ms ✓ | 115 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ107+1.004.p | 27.0 ms ✓ | 143 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ201+1.001.p | 200 µs ✓ | 71 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ201+1.002.p | 1.6 ms ✓ | 119 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ201+1.003.p | 6.9 ms ✓ | 195 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ201+1.004.p | 249.7 ms ✓ | 333 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ201+1.005.p | 992.4 ms ✓ | 555 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ201+1.006.p | 4.87 s ✓ | 793 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ202+1.002.p | 482 µs ✓ | 235 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ202+1.003.p | 109.9 ms ✓ | 1.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.002.p | 99 µs ✓ | 75 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.003.p | 175 µs ✓ | 104 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.004.p | 406 µs ✓ | 137 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.005.p | 1.3 ms ✓ | 168 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.006.p | 4.6 ms ✓ | 238 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.007.p | 17.6 ms ✓ | 312 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.008.p | 67.5 ms ✓ | 468 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.009.p | 169.5 ms ✓ | 830 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.010.p | 177.5 ms ✓ | 1.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.011.p | 188.8 ms ✓ | 2.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.012.p | 199.5 ms ✓ | 4.0 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.013.p | 210.7 ms ✓ | 8.0 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.014.p | 206.6 ms ✓ | 16.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.015.p | 213.9 ms ✓ | 33.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.016.p | 221.2 ms ✓ | 67.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.017.p | 227.5 ms ✓ | 126.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.018.p | 220.7 ms ✓ | 125.9 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.019.p | 242.7 ms ✓ | 123.9 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ203+1.020.p | 252.9 ms ✓ | 121.6 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.003.p | 67 µs ✓ | 140 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.004.p | 71 µs ✓ | 129 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.005.p | 77 µs ✓ | 138 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.006.p | 85 µs ✓ | 138 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.007.p | 100 µs ✓ | 157 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.008.p | 113 µs ✓ | 168 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.009.p | 131 µs ✓ | 153 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.010.p | 148 µs ✓ | 205 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.011.p | 171 µs ✓ | 168 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.012.p | 223 µs ✓ | 172 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.013.p | 242 µs ✓ | 180 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.014.p | 271 µs ✓ | 205 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.015.p | 335 µs ✓ | 220 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.016.p | 379 µs ✓ | 249 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.017.p | 441 µs ✓ | 321 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.018.p | 489 µs ✓ | 230 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.019.p | 616 µs ✓ | 282 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ204+1.020.p | 640 µs ✓ | 241 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.001.p | 86 µs ✓ | 162 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.002.p | 137 µs ✓ | 198 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.003.p | 291 µs ✓ | 187 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.004.p | 767 µs ✓ | 223 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.005.p | 2.3 ms ✓ | 258 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.006.p | 6.5 ms ✓ | 383 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.007.p | 18.5 ms ✓ | 405 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.008.p | 53.6 ms ✓ | 571 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.009.p | 149.7 ms ✓ | 619 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.010.p | 151.4 ms ✓ | 738 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.011.p | 162.9 ms ✓ | 961 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.012.p | 167.5 ms ✓ | 1.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.013.p | 174.7 ms ✓ | 1.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.014.p | 179.8 ms ✓ | 1.7 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.015.p | 182.6 ms ✓ | 1.9 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.016.p | 189.9 ms ✓ | 2.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.017.p | 186.1 ms ✓ | 2.6 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.018.p | 188.6 ms ✓ | 3.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.019.p | 181.8 ms ✓ | 3.5 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ205+1.020.p | 195.0 ms ✓ | 4.0 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ206+1.004.p | 384 µs ✓ | 273 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ206+1.005.p | 2.1 ms ✓ | 455 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ206+1.006.p | 18.3 ms ✓ | 724 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ206+1.007.p | 333.7 ms ✓ | 1.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ207+1.001.p | 153 µs ✗ | 88 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ207+1.002.p | 6.3 ms ✗ | 112 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ207+1.003.p | 592.8 ms ✗ | 163 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ208+1.001.p | 135 µs ✗ | 65 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ208+1.002.p | 1.7 ms ✗ | 237 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ208+1.003.p | 418.7 ms ✗ | 1.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.002.p | 88 µs ✗ | 134 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.003.p | 183 µs ✗ | 83 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.004.p | 541 µs ✗ | 117 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.005.p | 2.2 ms ✗ | 144 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.006.p | 9.1 ms ✗ | 194 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.007.p | 39.3 ms ✗ | 282 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.008.p | 151.1 ms ✗ | 471 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.009.p | 151.9 ms ✗ | 658 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.010.p | 162.1 ms ✗ | 1.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.011.p | 162.6 ms ✗ | 2.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.012.p | 175.6 ms ✗ | 4.0 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.013.p | 194.2 ms ✗ | 8.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.014.p | 191.7 ms ✗ | 16.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.015.p | 230.6 ms ✗ | 33.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.016.p | 278.9 ms ✗ | 68.4 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.017.p | 344.3 ms ✗ | 123.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.018.p | 351.1 ms ✗ | 122.6 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.019.p | 352.6 ms ✗ | 120.5 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ209+1.020.p | 364.4 ms ✗ | 119.5 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.001.p | 73 µs ✗ | 142 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.002.p | 81 µs ✗ | 159 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.003.p | 85 µs ✗ | 135 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.004.p | 101 µs ✗ | 130 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.005.p | 126 µs ✗ | 149 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.006.p | 155 µs ✗ | 145 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.007.p | 183 µs ✗ | 141 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.008.p | 230 µs ✗ | 169 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.009.p | 281 µs ✗ | 176 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.010.p | 354 µs ✗ | 199 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.011.p | 420 µs ✗ | 211 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.012.p | 499 µs ✗ | 195 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.013.p | 797 µs ✗ | 227 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.014.p | 716 µs ✗ | 253 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.015.p | 836 µs ✗ | 244 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.016.p | 1.0 ms ✗ | 293 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.017.p | 1.2 ms ✗ | 292 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.018.p | 1.3 ms ✗ | 295 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.019.p | 1.6 ms ✗ | 340 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ210+1.020.p | 1.8 ms ✗ | 343 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.003.p | 100 µs ✗ | 193 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.004.p | 168 µs ✗ | 248 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.005.p | 288 µs ✗ | 403 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.006.p | 574 µs ✗ | 648 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.007.p | 1.1 ms ✗ | 964 µs ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.008.p | 2.2 ms ✗ | 1.7 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.009.p | 4.9 ms ✗ | 3.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.010.p | 12.7 ms ✗ | 6.8 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.011.p | 33.2 ms ✗ | 15.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.012.p | 101.5 ms ✗ | 38.4 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.013.p | 293.0 ms ✗ | 114.7 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.014.p | 744.8 ms ✗ | 322.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.015.p | 2.04 s ✗ | 838.9 ms ? bound |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | SYJ212+1.016.p | 4.37 s ✗ | 2.18 s ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ106+1.p | 154 µs ✓ | 83 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ107+1.004.p | 1.5 ms ✓ | 172 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ201+1.001.p | 176 µs ✓ | 129 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ201+1.002.p | 1.2 ms ✓ | 218 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ201+1.003.p | 5.1 ms ✓ | 439 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ201+1.004.p | 49.8 ms ✓ | 779 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ201+1.005.p | 166.9 ms ✓ | 1.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ201+1.006.p | 438.2 ms ✓ | 1.8 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ201+1.007.p | 935.9 ms ✓ | 2.7 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.002.p | 105 µs ✓ | 89 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.003.p | 200 µs ✓ | 103 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.004.p | 677 µs ✓ | 133 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.005.p | 2.7 ms ✓ | 189 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.006.p | 11.7 ms ✓ | 244 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.007.p | 49.8 ms ✓ | 374 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.008.p | 130.7 ms ✓ | 527 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.009.p | 143.1 ms ✓ | 803 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.010.p | 290.0 ms ✓ | 1.4 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.011.p | 300.1 ms ✓ | 2.4 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.012.p | 638.6 ms ✓ | 4.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.013.p | 631.6 ms ✓ | 8.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.014.p | 1.33 s ✓ | 16.5 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.015.p | 1.36 s ✓ | 33.5 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.016.p | 2.74 s ✓ | 68.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.017.p | 2.75 s ✓ | 128.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ203+1.018.p | 2.92 s ✓ | 125.0 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.002.p | 97 µs ✓ | 61 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.003.p | 112 µs ✓ | 70 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.004.p | 428 µs ✓ | 64 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.005.p | 556 µs ✓ | 74 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.006.p | 704 µs ✓ | 73 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.007.p | 1.1 ms ✓ | 84 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.008.p | 16.9 ms ✓ | 87 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.009.p | 20.8 ms ✓ | 96 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.010.p | 25.3 ms ✓ | 96 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.011.p | 30.4 ms ✓ | 107 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.012.p | 36.2 ms ✓ | 108 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.013.p | 51.2 ms ✓ | 126 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.014.p | 59.1 ms ✓ | 136 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.015.p | 69.0 ms ✓ | 130 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.016.p | 106.9 ms ✓ | 133 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.017.p | 106.9 ms ✓ | 157 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.018.p | 107.8 ms ✓ | 163 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.019.p | 110.6 ms ✓ | 175 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ204+1.020.p | 108.7 ms ✓ | 184 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.002.p | 225 µs ✓ | 96 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.003.p | 395 µs ✓ | 134 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.004.p | 1.7 ms ✓ | 172 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.005.p | 3.2 ms ✓ | 230 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.006.p | 5.9 ms ✓ | 381 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.007.p | 10.2 ms ✓ | 392 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.008.p | 53.0 ms ✓ | 481 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.009.p | 91.2 ms ✓ | 591 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.010.p | 157.1 ms ✓ | 744 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.011.p | 248.5 ms ✓ | 945 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.012.p | 394.3 ms ✓ | 1.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.013.p | 642.3 ms ✓ | 1.4 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.014.p | 953.5 ms ✓ | 1.8 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.015.p | 1.96 s ✓ | 2.0 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ205+1.016.p | 4.68 s ✓ | 2.4 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ206+1.004.p | 1.1 ms ✓ | 289 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ206+1.005.p | 11.3 ms ✓ | 510 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ206+1.006.p | 153.3 ms ✓ | 872 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ206+1.007.p | 2.62 s ✓ | 1.5 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ207+1.001.p | 231 µs ✗ | 104 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ207+1.002.p | 29.0 ms ✗ | 160 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ207+1.003.p | 4.26 s ✗ | 306 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ208+1.001.p | 132 µs ✗ | 64 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ208+1.002.p | 120 µs ✗ | 86 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ208+1.003.p | 247 µs ✗ | 171 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ208+1.004.p | 1.4 ms ✗ | 833 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ208+1.005.p | 13.2 ms ✗ | 9.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ208+1.006.p | 169.5 ms ✗ | 123.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ208+1.007.p | 1.17 s ✗ | 755.4 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.002.p | 95 µs ✗ | 73 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.003.p | 413 µs ✗ | 120 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.004.p | 4.3 ms ✗ | 110 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.005.p | 48.3 ms ✗ | 156 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.006.p | 146.9 ms ✗ | 205 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.007.p | 152.8 ms ✗ | 308 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.008.p | 151.2 ms ✗ | 476 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.009.p | 316.3 ms ✗ | 766 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.010.p | 305.6 ms ✗ | 1.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.011.p | 630.3 ms ✗ | 2.3 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.012.p | 609.7 ms ✗ | 4.2 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.013.p | 1.31 s ✗ | 8.4 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.014.p | 1.37 s ✗ | 16.7 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.015.p | 2.80 s ✗ | 33.9 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.016.p | 2.91 s ✗ | 69.9 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ209+1.017.p | 3.01 s ✗ | 125.1 ms ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.001.p | 99 µs ✗ | 61 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.002.p | 283 µs ✗ | 64 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.003.p | 1.8 ms ✗ | 60 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.004.p | 10.7 ms ✗ | 63 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.005.p | 59.2 ms ✗ | 70 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.006.p | 110.2 ms ✗ | 72 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.007.p | 126.0 ms ✗ | 76 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.008.p | 256.2 ms ✗ | 79 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.009.p | 258.3 ms ✗ | 87 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.010.p | 523.7 ms ✗ | 95 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.011.p | 520.9 ms ✗ | 102 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.012.p | 1.01 s ✗ | 106 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.013.p | 1.03 s ✗ | 112 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.014.p | 2.05 s ✗ | 121 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.015.p | 2.06 s ✗ | 140 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.016.p | 2.09 s ✗ | 136 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.017.p | 4.11 s ✗ | 151 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.018.p | 4.13 s ✗ | 157 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ210+1.019.p | 4.21 s ✗ | 167 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ212+1.001.p | 83 µs ✓ MISMATCH | 56 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ212+1.002.p | 153 µs ✗ | 83 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ212+1.003.p | 3.1 ms ✗ | 142 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ212+1.004.p | 77.9 ms ✗ | 282 µs ? bound |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | SYJ212+1.005.p | 1.61 s ✗ | 505 µs ? bound |
|  | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | SYN044+1.p | 138 µs ✓ | 74 µs ? bound |
|  | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | SYN045+1.p | 228 µs ✓ | 107 µs ? bound |
|  | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | SYN388+1.p | 65 µs ✗ | 53 µs ? bound |
|  | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | SYN391+1.p | 358 µs ✓ | 172 µs ? bound |
|  | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | SYN393+1.p | 3.4 ms ✗ | 138 µs ? bound |
|  | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 | SYN393+1.p | 116 µs ✗ | 159 µs ? bound |
|  | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 | SYN393+1.p | 275 µs ✗ | 134 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE004+1.p | 75 µs ✓ | 60 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE018+1.p | 155 µs ✓ | 107 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE026+1.p | 317 µs ✓ | 113 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE027+1.p | 333 µs ✓ | 95 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE028+1.p | 264 µs ✓ | 80 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE029+1.p | 272 µs ✓ | 97 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE030+1.p | 2.0 ms ✓ | 83 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE031+1.p | 10.8 ms ✓ | 89 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE032+1.p | 115 µs ✓ | 86 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE033+1.p | 130 µs ✓ | 89 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE034+1.p | 143 µs ✓ | 92 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE035+1.p | 150 µs ✓ | 110 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE036+1.p | 187 µs ✓ | 85 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE037+1.p | 197 µs ✓ | 104 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE038+1.p | 832 µs ✓ | 76 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE043+1.p | 161 µs ✓ | 84 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE044+1.p | 218 µs ✓ | 105 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE058+1.p | 171 µs ✓ | 55 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE061+1.p | 65 µs ✓ | 46 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE062+1.p | 106 µs ✓ | 55 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE066+1.p | 77 µs ✓ | 51 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE067+1.p | 125 µs ✓ | 83 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE068+1.p | 183 µs ✗ | 63 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE073+1.p | 149 µs ✓ | 59 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE076+1.p | 154 µs ✓ | 74 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE080+1.p | 100 µs ✓ | 68 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE081+1.p | 98 µs ✓ | 69 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE082+1.p | 145 µs ✓ | 59 µs ? bound |
|  | ILL/KLE-01 | intuitionistic auto j1 | KLE084+1.p | 6.6 ms ✓ | 155 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_14_01.p | 103 µs ✓ | 81 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_14_CBV.p | 82 µs ✓ | 66 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_22_01.p | 317 µs ✓ | 122 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_23_01.p | 335 µs ✓ | 92 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_23_CBN.p | 83 µs ✓ | 178 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_23_CBV.p | 206 µs ✓ | 83 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_24_01.p | 171 µs ✓ | 86 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_24_CBN.p | 72 µs ✓ | 129 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_24_CBV.p | 116 µs ✓ | 74 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_25_01.p | 280 µs ✓ | 93 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_25_CBN.p | 120 µs ✓ | 162 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_25_CBV.p | 180 µs ✓ | 83 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_26_01.p | 264 µs ✓ | 71 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_26_CBN.p | 92 µs ✓ | 65 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_26_CBV.p | 106 µs ✓ | 90 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_27_01.p | 611 µs ✓ | 94 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_27_CBN.p | 120 µs ✓ | 144 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_27_CBV.p | 319 µs ✓ | 75 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_28_01.p | 114 µs ✓ | 94 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_29_01.p | 131 µs ✓ | 96 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_30_01.p | 221 µs ✓ | 104 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_31_01.p | 156 µs ✓ | 104 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_32_01.p | 133 µs ✓ | 98 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_44_01.p | 64 µs ✓ | 59 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_45_01.p | 100 µs ✓ | 62 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_45_CBN.p | 60 µs ✓ | 159 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_45_CBV.p | 67 µs ✓ | 57 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_46_01.p | 78 µs ✓ | 55 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_46_CBN.p | 56 µs ✓ | 43 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_46_CBV.p | 58 µs ✓ | 55 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_47_01.p | 128 µs ✓ | 85 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_48_01.p | 81 µs ✓ | 67 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_49_01.p | 89 µs ✓ | 72 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_4_01.p | 75 µs ✓ | 61 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_50_01.p | 142 µs ✓ | 97 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_51_01.p | 200 µs ✓ | 109 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_51_CBN.p | 92 µs ✓ | 162 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_51_CBV.p | 123 µs ✓ | 83 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_52_01.p | 263 µs ✓ | 100 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_52_CBN.p | 82 µs ✓ | 161 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_52_CBV.p | 118 µs ✓ | 94 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_53_01.p | 119 µs ✓ | 76 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_54_01.p | 90 µs ✓ | 75 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_55_01.p | 90 µs ✓ | 64 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_56_01.p | 171 µs ✓ | 70 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_56_CBV.p | 81 µs ✓ | 61 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_57_01.p | 183 µs ✓ | 75 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_57_CBN.p | 67 µs ✓ | 117 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_57_CBV.p | 89 µs ✓ | 67 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_58_01.p | 186 µs ✓ | 73 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_58_CBN.p | 76 µs ✓ | 116 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_58_CBV.p | 92 µs ✓ | 72 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_59_01.p | 197 µs ✓ | 76 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_59_CBN.p | 79 µs ✓ | 145 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_59_CBV.p | 103 µs ✓ | 61 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_60_01.p | 173 µs ✓ | 96 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_60_CBV.p | 95 µs ✓ | 78 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_61_01.p | 202 µs ✓ | 78 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_61_CBN.p | 76 µs ✓ | 63 µs ? bound |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | KLE_61_CBV.p | 118 µs ✓ | 76 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE018+1.p | 72 µs ✓ | 54 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE027+1.p | 85 µs ✓ | 168 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE028+1.p | 70 µs ✓ | 156 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE029+1.p | 117 µs ✓ | 144 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE030+1.p | 236 µs ✓ | 54 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE031+1.p | 270 µs ✓ | 138 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE038+1.p | 62 µs ✓ | 140 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE058+1.p | 69 µs ✓ | 125 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE062+1.p | 56 µs ✓ | 109 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE068+1.p | 69 µs ✗ | 58 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE075+1.p | 311 µs ✗ MISMATCH | 214 µs ? bound |
|  | ILL/KLE-cbn | intuitionistic auto j1 | KLE084+1.p | 684 µs ✓ | 168 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE018+1.p | 112 µs ✓ | 81 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE027+1.p | 230 µs ✓ | 79 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE028+1.p | 114 µs ✓ | 71 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE029+1.p | 190 µs ✓ | 79 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE030+1.p | 302 µs ✓ | 76 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE031+1.p | 1.4 ms ✓ | 75 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE038+1.p | 158 µs ✓ | 57 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE058+1.p | 80 µs ✓ | 53 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE062+1.p | 63 µs ✓ | 49 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE076+1.p | 86 µs ✓ | 65 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE082+1.p | 80 µs ✓ | 64 µs ? bound |
|  | ILL/KLE-cbv | intuitionistic auto j1 | KLE084+1.p | 2.3 ms ✓ | 115 µs ? bound |

#### Decided by lltp-intuitionistic, not by lltp-forward

None.

#### By group

Problems in both; decided (late: after the time limit); decided by one and not the other; where the undecided ended (`time` the time limit or `killed` past it, `bound` the copy bound, `depth` the recursion limit, `memory` the memory limit, `index` an index past its width, `wide` a context too wide to split, `crash` out of memory); the median ratio of lltp-intuitionistic's time to lltp-forward's on the problems both decide within the limit (how many); and the problems only lltp-intuitionistic has.

| file | family | configuration | problems | lltp-forward decided | lltp-intuitionistic decided | only lltp-forward | only lltp-intuitionistic | lltp-forward ended | lltp-intuitionistic ended | lltp-intuitionistic/lltp-forward | new |
|---|---|---|--:|--:|--:|--:|--:|---|---|--:|--:|
|  | ILL/ILLTP-LCL-01 | intuitionistic auto j1 | 2 | 2 | 1 | 1 | 0 | – | 1 bound | 0.82× (1) | 0 |
|  | ILL/ILLTP-LCL-cbn | intuitionistic auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.82× (2) | 0 |
|  | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 | 2 | 2 | 1 | 1 | 0 | – | 1 bound | 0.66× (1) | 0 |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 252 | 138 | 34 | 104 | 0 | 90 time, 22 depth, 2 memory | 4 time, 189 bound, 22 depth, 3 memory | 0.31× (34) | 0 |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 252 | 170 | 38 | 132 | 0 | 59 time, 22 depth, 1 memory | 4 time, 186 bound, 22 depth, 2 memory | 1.57× (38) | 0 |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 252 | 157 | 43 | 114 | 0 | 83 time, 10 depth, 2 memory | 23 time, 174 bound, 10 depth, 2 memory | 0.79× (43) | 0 |
|  | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | 19 | 19 | 14 | 5 | 0 | – | 5 bound | 0.78× (14) | 0 |
|  | ILL/ILLTP-SYN-cbn | intuitionistic auto j1 | 19 | 19 | 18 | 1 | 0 | – | 1 bound | 0.91× (18) | 0 |
|  | ILL/ILLTP-SYN-cbv | intuitionistic auto j1 | 19 | 19 | 18 | 1 | 0 | – | 1 bound | 0.82× (18) | 0 |
|  | ILL/KLE-01 | intuitionistic auto j1 | 88 | 87 | 58 | 29 | 0 | 1 bound | 30 bound | 0.78× (58) | 0 |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | 222 | 222 | 162 | 60 | 0 | – | 60 bound | 0.87× (162) | 0 |
|  | ILL/KLE-IMP-CONJ/ALT | intuitionistic auto j1 | 27 | 27 | 27 | 0 | 0 | – | – | 0.79× (27) | 0 |
|  | ILL/KLE-IMP-CONJ/NON-THEOREMS | intuitionistic auto j1 | 22 | 22 | 22 | 0 | 0 | – | – | 0.82× (22) | 0 |
|  | ILL/KLE-cbn | intuitionistic auto j1 | 88 | 88 | 76 | 12 | 0 | – | 12 bound | 0.87× (76) | 0 |
|  | ILL/KLE-cbv | intuitionistic auto j1 | 88 | 87 | 75 | 12 | 0 | 1 bound | 13 bound | 0.79× (75) | 0 |
|  | ILL/Non-theorems | intuitionistic auto j1 | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
|  | ILL/misc | intuitionistic auto j1 | 3 | 3 | 3 | 0 | 0 | – | – | 0.96× (3) | 0 |
|  | ILL/petri-nets/MCC | intuitionistic auto j1 | 3137 | 3071 | 3071 | 0 | 0 | 63 time, 3 memory | 57 time, 9 memory | 0.76× (3071) | 0 |

## deduxit against LinearOne on the first-order families

Not a baseline: the first-order families (`lambek-fo`, `chain-fo`,
`witness-fo`) came after the first three baselines, and LinearOne, Moot's prover of
first-order MILL, is the prover their comparison is made with
(`bench/TOOLS.md`, "The first-order families", says how). The verdicts,
2026-10-30, each problem once with each tool under runexec at 10 s and
4 GiB (`comparison/first-order/verdicts.csv`):

| family (shapes) | problems | deduxit decides | LinearOne decides | both, verdicts equal | contradictions |
|---|---|---|---|---|---|
| lambek-fo (#0 to #4) | 20 | 18 | 20 | 18 | 0 |
| witness-fo, MILL1 (#2, #3) | 8 | 7 | 8 | 7 | 0 |
| witness-fo, MALL1 (#0, #1) | 8 | 8 | outside its fragment | – | – |

Every verdict of both equals the family's by construction. deduxit's three
undecided rows: `lambek-fo/512` #0 and #2 at its recursion limit (2 048),
`witness-fo/12#3` at the time limit. `chain-fo` (MELL1) is outside
LinearOne's fragment.

The wall times (`comparison/first-order/times-1.csv` and `times-2.csv`,
2026-10-30, the command of `bench/TOOLS.md` on cores 2 and 3 of an
otherwise quiet machine, one run per problem): on the 25 problems both
decide, deduxit takes 4.0 ms in the median and LinearOne 51 ms, deduxit
the faster on 24; LinearOne's floor of about 46 ms is SWI-Prolog's
start. Where the problems grow, `lambek-fo/512#3` takes deduxit 24 ms and
LinearOne 3.2 s, and the one row LinearOne wins of those both decide,
`witness-fo/10#3`, takes deduxit 2.5 s and LinearOne 46 ms; LinearOne
also decides the three deduxit leaves undecided.

