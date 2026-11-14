# Benchmarks and the comparison

deduxit's speed is measured with its own harness, `deduxit-bench`, and
set beside the other provers of linear logic. This chapter says what is
measured, how to read the results, and how to run the harness yourself.
["Beside the other provers"](#beside-the-other-provers) sums up the
comparison.

## The results

Each run has a chapter of its own, with its tables and figures:

- [The baseline of 2026-11-02](results/baseline-2026-11-02.md): the
  engines of 0.2.0 on the generated families and the LLTP library.
- [The baseline of 2026-10-17](results/baseline-2026-10-17.md): the
  engines of 0.1.0, on the same.
- [Families that grow, 2026-11-02](results/growing-2026-11-02.md) and
  [2026-10-31](results/growing-2026-10-31.md): time against size for
  families chosen to show each engine's and each other prover's
  strengths and weaknesses.
- The comparison with the other provers:
  [beside the other provers, 2026-10-15](results/provers-2026-10-15.md),
  [first order against LinearOne, 2026-10-30](results/first-order-2026-10-30.md)
  and [the other provers on the growing sets, 2026-11-03](results/provers-2026-11-03.md).

## How to read them

Every figure is drawn by `bench/plots.py` from the rows of the run's CSV
files when the manual is built (`docs/figures.sh`). None is drawn by
hand, and a figure always shows the rows it names.

- Time is the wall-clock time of the search alone: building the
  occurrence forest and starting a thread pool count, reading the
  problem, checking the proof and starting the process do not. A
  problem's time is the median of its runs. In a comparison with other
  provers the time is instead the whole process's, as BenchExec measures
  it.
- In the tables, `✓` is a proof, `✗` a refutation, and `> 10 s` a
  problem not decided within the time limit. `†` marks a run on one
  thread that waited for a CPU for over 1 % of its time, so another
  process slowed it down.
- In the figures of time against size, a hollow marker on the
  limit's line is a size not decided, and a cross above the limit is a
  size skipped because a smaller one already failed. Where a table
  follows the figure, it gives per line the better fit by least squares
  of the logarithm of the time, polynomial `n^d` or exponential `b^n`; "fewer
  than 3 sizes" means too few sizes took over a millisecond to fit.
- In the comparison, a tool solves a problem when it answers within
  the limit and is not wrong. A verdict is wrong when it refutes a
  problem some tool proved with a checked proof, or contradicts the
  verdict a generated problem has by construction. A verdict that only
  contradicts an LLTP file's header counts as solved, in a column of its
  own: about thirty of the library's headers are wrong. "Late" is an
  answer after the limit and before the kill.

## Run the harness

`deduxit-bench` is `cargo run --release -p deduxit-bench -- …` in a
checkout, or `nix build .#deduxit-bench`. It times the engines on these
kinds of problems:

- Generated families with known verdicts, each at any size.
  `deduxit-bench families` lists them: 3-Partition as a Horn program and
  as an MLL sequent, Matsuoka's Partition, random QBF, wide sequents,
  contexts under Mix, Petri-net counters, the Lambek calculus in
  first-order MILL, a chain of instances of a quantified clause under
  `!`, a witness among many, Franzén's contraction formulas and Cook's
  pigeonhole principle from Dyckhoff's intuitionistic benchmarks read in
  linear logic, Euler tours, a clause used many times on one branch,
  reachability through a cycle at first order, and more.
- The [LLTP library](https://github.com/meta-logic/lltp):
  `nix build .#lltp -o bench/lltp` fetches it at a pinned commit. Its
  problems under `ILL/` run intuitionistically.
- Coverability problems in the `.spec` format, with `--spec`: the
  176 of the [qcover](https://github.com/blondimi/qcover) suite, which
  `nix build .#qcover -o bench/qcover` fetches. They run in
  intuitionistic affine mode.
- Problem files of lines `name; mode; expected; copies; sequent`,
  such as `bench/problems/slow-tests.txt`.
- Problems of ordinary logic from the ILTP library, in TPTP format:
  `nix build .#iltp -o bench/iltp` fetches its 274 propositional
  problems, and `run --tptp PATH --logics …` runs them with the engines
  `embedding`, `ljt` and `auto`.

`run` runs every problem in every mode, engine and thread count asked
for. Each run is a child process of its own with a time limit, and
writes one CSV row with the verdict, the time and the engine's counters.
`summary` prints Markdown tables of CSV files: the problems solved
within the time limit per family and configuration, and a time per
problem and configuration.

```console
$ deduxit-bench run --family partition-no=3,4 --engines focus,net --jobs 1,4 --timeout 10 --output runs.csv
[1/8] partition-no/3 classical focus j1: unprovable  0.089 ms, about 0 min left
[2/8] partition-no/3 classical focus j4: unprovable  0.146 ms, about 0 min left
[3/8] partition-no/3 classical net j1: unprovable  3355.671 ms, about 0 min left
[4/8] partition-no/3 classical net j4: unprovable  1020.791 ms, about 0 min left
[5/8] partition-no/4 classical focus j1: unprovable  0.146 ms, about 0 min left
[6/8] partition-no/4 classical focus j4: unprovable  0.303 ms, about 0 min left
[7/8] partition-no/4 classical net j1: unknown timeout 10000.591 ms, about 0 min left
[8/8] partition-no/4 classical net j4: unknown timeout 10000.139 ms, about 0 min left

$ deduxit-bench summary runs.csv
…
## partition-no

| problem | runs: classical focus j1 | runs: classical focus j4 | runs: classical net j1 | runs: classical net j4 |
|---|--:|--:|--:|--:|
| partition-no/3 | 89 µs ✗ | 146 µs ✗ | 3.36 s ✗ | 1.02 s ✗ |
| partition-no/4 | 146 µs ✗ | 303 µs ✗ | > 10 s | > 10 s |
```

### Compare two runs

`summary --before DIR` compares every file with the file of the same
name in `DIR`, an earlier baseline, problem by problem. It lists the
verdicts that differ, the problems decided before and not after, one row
per family or LLTP collection and configuration (the problems decided
before and after, late verdicts apart, where the undecided ended, and
the median ratio of the times), and every generated problem side by
side. Given again for each earlier baseline, oldest first, it sets them
all side by side before it compares with the last.
`summary --against FILE` compares every other file with `FILE` in the
same way, whatever the files' names: for instance, the passes under one
bias with the default.

```console
$ deduxit-bench summary --before bench/results/2026-09-30 bench/results/2026-10-02/long-1.csv
…
| file | family | configuration | problem | before | after | after/before |
|---|---|---|---|--:|--:|--:|
| long-1 | 3-partition-no | classical auto j1 | 3-partition-no/5 | 302.42 s ✗ | 393 µs ✗ | 1.3e-6× |
| long-1 | counter | classical auto j1 | counter/16 | > 1200 s | 280 µs ✓ |  |
| long-1 | counter | classical auto j1 | counter/64 |  | 85.14 s ✓ |  |
| long-1 | partition-no | classical auto j1 | partition-no/15 |  | > 1200 s |  |
| long-1 | partition-no | classical auto j1 | partition-no/5 | 811.48 s ✗ | 477 µs ✗ | 5.9e-7× |
| long-1 | partition-yes | classical auto j1 | partition-yes/28 |  | > 1200 s |  |
| long-1 | partition-yes | classical auto j1 | partition-yes/7 | > 1200 s | 230 µs ✓ |  |
```

### Change how the search runs

Each of these options is written into the rows, so the summary keeps
the configurations apart:

- `run --bias rarer|factors` runs the focused engines under that bias.
- `run --forward-copies N` bounds the default's forward search.
- `run --copies none` deepens the copy bound until the time limit; the
  rows have the copy bound reached (`copies_reached`).
- `run --pool-after SECONDS` searches on one thread for that long
  before a pool of the other `--jobs` threads joins it, as the command
  does by default.
- `run --memo-limit 0` runs without the memo, to measure what it gains.

### The focused engine's target set

`bench/targets.sh LABEL` runs the instances the first baseline showed
the focused engine losing on: the hard families at the sizes that took
minutes or did not finish, and a fixed sample of 113 LLTP problems. It
runs on two pinned cores in a memory-capped user unit, into
`bench/targets/LABEL.csv`. On one thread the engine's counters are a
function of the input, so two such files tell whether a change altered
the search at all: `summary --against FILE` with `--counters` lists every
problem decided alike whose counters differ. `bench/TARGETS.md` sets the engine before the
performance work beside the engine after it.

### Growing sets

Some families have a growing set: sizes from easy to past what the
engines decide in a minute, chosen for a claim about the strength or
weakness of an engine or of another prover, which the growth of the time
shows. `deduxit-bench families` prints each family's sizes and claim
under its summary.

`run --growing` runs every growing set. Once a configuration's run on a
size is refused, or meets the time limit or the memory bound, the
configuration skips that shape's larger sizes: every growing set is a
constructed family whose larger instance is no easier. The skipped sizes
are rows `skipped` ("past the limit at size n") that no table counts.
With `--shard`, a shard takes whole shapes.

```console
$ deduxit-bench run --growing --only pigeonhole/ --engines auto,inverse --timeout 1 --output runs.csv
[1/28] pigeonhole/2#0 intuitionistic-affine auto j1: proved  0.144 ms, about 0 min left
[2/28] pigeonhole/2#0 intuitionistic-affine inverse j1: proved  0.078 ms, about 0 min left
…
[9/28] pigeonhole/4#0 intuitionistic-affine auto j1: proved  56.756 ms, about 0 min left
[10/28] pigeonhole/4#0 intuitionistic-affine inverse j1: proved  0.697 ms, about 0 min left
…
[13/28] pigeonhole/5#0 intuitionistic-affine auto j1: unknown timeout 1010.707 ms, about 0 min left
[14/28] pigeonhole/5#0 intuitionistic-affine inverse j1: proved  21.632 ms, about 0 min left
…
[18/28] pigeonhole/6#0 intuitionistic-affine inverse j1: proved  370.384 ms, about 0 min left
…
[22/28] pigeonhole/7#0 intuitionistic-affine inverse j1: unknown memory_limit 1038.479 ms, about 0 min left
```

`bench/growing.sh` runs the growing sets as the figures draw them, into
`bench/growing/DAY/`:

- every family under the default and with every engine forced
  (`engines-K.csv`, a stream per core);
- the passes that switch off the mechanism a claim is about: the memo on
  `chain` and `reach-fo` (`memo-off.csv`), the copy bound on `chain`
  (`deepening.csv`, `--copies none`), and the single thread on
  `witness-fo` (`pool.csv`, every core of `--pool-cores`).

`bench/comparison.sh --growing` runs the other provers on them.

## Baselines

`bench/baseline.sh --detach --fresh` takes the whole baseline (every
family, engine and thread count, and the whole LLTP library) at once, as
a systemd user unit on an otherwise idle machine, in about three hours.
`--arm` instead starts it unattended at night: at 20:00, or at once if
that has passed, once the machine is idle, and it stops at 07:00
whatever its state. `--slot=HH:MM-HH:MM` gives other times, and the
script run again without `--fresh` finishes a stopped baseline.
`journalctl --user -fu deduxit-baseline` follows it.

Every baseline keeps a directory named by the day it started,
`bench/results/DAY/`: the CSV files of its runs, `starts.txt` with the
commit measured, and its tables in `RESULTS.md`. `bench/RESULTS.md` is a
copy of the latest baseline's. A last stage runs again, with more time
before the kill and more memory, the runs that `bench/reruns.txt` lists:
those of an earlier baseline that were killed or crashed and that
measurement showed to finish with more room.

The baselines so far ran on a 16-core Intel Core Ultra X9 388H:

| baseline | engines | took |
|---|---|---|
| 2026-09-30 | before the performance work on the focused engine | 9 h 21 min, and 1 h 28 min the next night for the last stage |
| 2026-10-02 | after that work | 8 h 4 min |
| 2026-10-17 | the first release, 0.1.0, with the intuitionistic library under the command's default as a pass of its own | 2 h 58 min |
| 2026-11-02 | the second release, 0.2.0 | 2 h 23 min, by day beside other work, two of its four streams on efficiency cores (its chapter says what that changes) |

`bench/RESULTS.md` has the fourth's tables, and `bench/COMPARISON.md`
sets the four side by side. No verdict differs between them. The
intuitionistic LLTP pass decides 3 663 problems within 5 s in the
fourth, where the first decided 737, the second 2 047 and the third
3 610. Under the command's default (no copy bound, every core, 2 s) the
fourth decides 4 124 of the 4 495.

## The comparison with other provers

`run --tools` runs the same problems through other provers of linear
logic beside deduxit's command. The flake builds each at a pinned
version (`nix build .#comparison` has them all, with BenchExec). Every
problem is translated into each tool's syntax. Every run is a process
measured by BenchExec's `runexec` under the same time and memory limit
(`--timeout`, `--memory-limit`, 4 GiB by default). A problem outside a
tool's fragment or mode is a row of its own and is not run.

`ranking` prints the comparison: the problems each tool solves within
the limit, per group of problems; deduxit on one thread against each
tool; and every problem on which tools contradict each other or the
known verdict. `--shard K/N` runs every N-th problem from the K-th, so
that jobs on several machines split a set.

```console
$ deduxit-bench run --tools deduxit,maude-ill,sympli --lltp lltp/ILL/ILLTP-SYJ-01 --only SYJ101+1,SYJ105+1.003 --timeout 10 --output tools.csv
[1/6] ILLTP-SYJ-01/SYJ101+1.p deduxit: proved  3.4 ms, about 0 min left
[2/6] ILLTP-SYJ-01/SYJ101+1.p maude-ill: proved  89.8 ms, about 0 min left
[3/6] ILLTP-SYJ-01/SYJ101+1.p sympli: proved  11.7 ms, about 0 min left
[4/6] ILLTP-SYJ-01/SYJ105+1.003.p deduxit: unprovable  4.0 ms, about 0 min left
[5/6] ILLTP-SYJ-01/SYJ105+1.003.p maude-ill: unknown copy_bound 131.5 ms, about 0 min left
[6/6] ILLTP-SYJ-01/SYJ105+1.003.p sympli: unprovable  3.0 ms, about 0 min left

$ deduxit-bench ranking tools.csv
…
## Solved within the limit

| tool | solved | proved | unprovable | wrong | late | time | memory | gave up | error | outside | against the header |
|---|---|---|---|---|---|---|---|---|---|---|---|
| deduxit | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| sympli | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| maude-ill | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
…
```

deduxit's and Sympli's refutations of SYJ105+1.003 count as solved and
against the file's header, which says Theorem. The formula has a
classical countermodel, `a` and `b` false, so the header is wrong.

`bench/comparison.sh` runs the whole comparison on any machine with
Nix, and `bench/TOOLS.md` has the tools, how they were run and the
results.

## Beside the other provers

The comparison with the other provers, run by run:

| run | deduxit | problems | other provers | chapter |
|---|---|---|---|---|
| 2026-10-15 | the engines of 0.1.0 | the LLTP library, qcover's coverability problems and the generated families, 4 811 | the eight of the table below | [Beside the other provers, 2026-10-15](results/provers-2026-10-15.md) |
| 2026-10-30 | the first-order search of 0.2.0 | `lambek-fo` and `witness-fo`, 36 | LinearOne | [First order against LinearOne, 2026-10-30](results/first-order-2026-10-30.md) |
| 2026-10-31 | the engines of 0.2.0 | each growing set's first two sizes, a check of every tool's verdicts, not its curve | the eight of the table below | [Families that grow, 2026-10-31](results/growing-2026-10-31.md#the-other-provers) |
| 2026-11-03 | the engines of 0.2.0 | the growing sets to each tool's limit, 212 problems run | the eight of the table below | [The other provers on the growing sets, 2026-11-03](results/provers-2026-11-03.md) |

So the numbers on the libraries below are those of 0.1.0's engines;
the engines of 0.2.0 decide more of them ([the baseline of
2026-11-02](results/baseline-2026-11-02.md)), but the other provers have
not yet run on the libraries beside them. On the growing families the
other provers ran beside the engines of 0.2.0, to their limits.

The provers of linear logic we found, as their own sources state them
(read on 2026-10-15). `bench/TOOLS.md` has every feature with its source
and the results of running all of them on the same problems.

| prover | fragments | modes | refutes | checked proofs | licence |
|---|---|---|---|---|---|
| deduxit | MLL to LL, units | classical, intuitionistic, affine, Mix, cyclic and Lambek (MLL) | ✓ | ✓ (and Rocq certificates) | EUPL-1.2 |
| Maude LL prover | LL, ILL | classical, intuitionistic | without `!` | ✗ | none stated |
| llprover | first-order LL | classical, intuitionistic | without `!`, `?` | ✗ | none formal |
| linTAP | MLL with `?` | classical | without `?` | ✗ | GPL |
| Click & coLLecT | LL | classical | ✓ | Coq certificates | LGPL-2.1 |
| Sympli | ILL without `⅋ ?` | intuitionistic | ✓ | its own | BSD-2-Clause |
| LinearOne | first-order MILL | intuitionistic | ✓ | unverified | LGPL-2.1 |
| lolli | MLL, MALL, MELL | classical | ✗ (a depth limit) | ✗ | MIT |
| APLL | LL, ILL | classical, intuitionistic | unless its bound is met | Coq certificates | none stated |
| Proof Net Calculator | unit-free MLL nets | — | no prover | net criteria | none stated |

The 4 811 problems are the LLTP library, qcover's coverability problems
and the generated families. Each was run under 10 s and 4 GiB on one
core and measured by BenchExec, each tool on the problems in its own
fragment and modes. Solved:

| tool | solved |
|---|--:|
| deduxit, one thread | 4 181 |
| Sympli | 1 684 |
| the Maude prover, intuitionistic and classical | 768 and 31 |
| APLL, intuitionistic and classical | 612 and 31 |
| llprover, intuitionistic and classical | 479 and 11 |
| LinearOne | 69 |
| Click & coLLecT | 31 |
| linTAP | 15 |
| lolli | 14 |

Most of deduxit's lead is the library's 3 137 Petri nets, which it
decides with an engine of its own (3 072, against Sympli's 811). On the
other 1 358 intuitionistic problems Sympli solves the most, 873 against
deduxit's 829 (852 by its default), and 132 that deduxit does not,
mostly refutations of the SYJ collections.

No tool proves a sequent deduxit refutes. APLL and Click & coLLecT
refute provable sequents, 1 308 and 12 of them. Every one is an overflow
in the search the two share, from 63 formulas beside a `⊗` ([the growing
sets' comparison](results/provers-2026-11-03.md#the-wrong-refutations)).

On the first-order families in LinearOne's fragment (28 problems) the
two agree wherever both decide. LinearOne decides all 28 and deduxit
25; the other three end at deduxit's recursion and time limits
(`bench/TOOLS.md`, "The first-order families").
