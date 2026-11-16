# Benchmark results

The baseline of 2026-11-16 on an Intel(R) Core(TM) Ultra X9 388H
(16 cores, 62 GB), release build, from
`bench/baseline.sh`, which writes this file, the CSV files beside it
(`bench/results/2026-11-16/`, one row per run) and a copy of this file as
`bench/RESULTS.md`, the latest baseline's. It started (once more for
every resumption), with the commit it measured, the LLTP problems of its
later stages chosen from `bench/results/2026-09-30/` and its reruns from
`bench/reruns.txt`:

- 2026-11-16 02:04, commit 4093f02777ed (Plan: 62c reviewed and pushed), load average 4.04, sequential streams on CPUs 1 0 2 3

Its last part took 2 h 22 min, and the scheduled jobs that ran
meanwhile were: nix-optimise 03:52. The package throttled
55932 times for 479 s in that part
(platform profile performance, governor powersave, energy preference balance_performance, turbo on).
The journal of `deduxit-baseline` says every ten minutes what else used
a CPU. The sequential runs went in four
streams at once, each pinned to a CPU of its own (CPUs 1 0 2 3),
which other processes could share (the user's slices could not be kept off them: no cpuset controller); the parallel runs came after them with no stream of the run
beside them, on every core of the machine. A time is the median of up to
three runs; `✓` is proved, `✗` refuted, `?` unknown (`bound`: the copy
bound, `wide`: a context too wide to split, `depth`: the recursion
limit), `>` a time limit reached, `MISMATCH` a verdict against the
problem's known one (for LLTP, the one its header claims; for ILTP, its
intuitionistic status, which also settles a classical theorem and a
minimal non-theorem). `deduxit-bench
summary bench/results/2026-11-16/*.csv` prints the tables again.

## Solved within the time limit

Sequential problems that waited for a CPU for over 1 % of their time (another process slowed them down; `†` in the tables below): 138.

| family | configuration | engines | problems | solved | proved | refuted | late | timeout | bound | other | mismatch | median solved | total solved |
|---|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| partition-yes | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 150 µs | 433.9 ms |
| partition-yes | engines: classical net j1 | net | 5 | 3 | 3 | 0 | 0 | 2 | 0 | 0 | 0 | 28.8 ms | 503.0 ms |
| partition-no | engines: classical focus j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 80.5 ms | 3.64 s |
| partition-no | engines: classical net j1 | net | 4 | 2 | 0 | 2 | 0 | 2 | 0 | 0 | 0 | 9.4 ms | 10.9 ms |
| 3-partition-mll-yes | engines: classical focus j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 54 µs | 221 µs |
| 3-partition-mll-yes | engines: classical net j1 | net | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 135 µs | 539 µs |
| 3-partition-mll-no | engines: classical focus j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 44 µs | 175 µs |
| 3-partition-mll-no | engines: classical net j1 | net | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 5.3 ms |
| wide-m1 | engines: classical focus j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 126 µs | 216.4 ms |
| wide-m1 | engines: classical net j1 | net | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 225 µs | 615.9 ms |
| wide-m2 | engines: classical focus j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 112 µs | 206.2 ms |
| wide-m2 | engines: classical net j1 | net | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 223 µs | 611.2 ms |
| wide-m3 | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 104 µs | 182.2 ms |
| wide-m3 | engines: classical net j1 | net | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 249 µs | 648.2 ms |
| wide-m4 | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 98 µs | 170.5 ms |
| wide-m4 | engines: classical net j1 | net | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 299 µs | 658.5 ms |
| additive | engines: classical focus j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 347.2 ms | 5.88 s |
| partition-table | engines: classical focus j1 | focus | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 78 µs | 2.2 ms |
| partition-table | engines: classical net j1 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.3 ms | 5.29 s |
| cancellation | engines: classical focus j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 304 µs | 304 µs |
| 3-partition-yes | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 70 µs | 267 µs |
| 3-partition-no | families: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 344 µs | 637 µs |
| 3-partition-mll-yes | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 72 µs | 279 µs |
| 3-partition-mll-no | families: classical auto j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 48 µs | 190 µs |
| partition-yes | families: classical auto j1 | focus | 9 | 8 | 8 | 0 | 0 | 1 | 0 | 0 | 0 | 12.6 ms | 72.10 s |
| partition-no | families: classical auto j1 | focus | 7 | 5 | 0 | 5 | 0 | 2 | 0 | 0 | 0 | 438 µs | 3.61 s |
| qbf | families: classical auto j1 | focus | 36 | 34 | 6 | 28 | 0 | 0 | 0 | 2 | 0 | 119.5 ms | 693.57 s |
| wide-m1 | families: classical auto j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 122 µs | 204.1 ms |
| wide-m2 | families: classical auto j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 109 µs | 190.1 ms |
| wide-m3 | families: classical auto j1 | focus | 7 | 7 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 117 µs | 207.5 ms |
| wide-m4 | families: classical auto j1 | focus | 8 | 8 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 118 µs | 193.4 ms |
| mix | families: mix auto j1 | focus, inverse | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 252.8 ms | 1.03 s |
| counter | families: classical auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 43 µs | 244 µs |
| counter-over | families: classical auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 41 µs | 238 µs |
| growing | families: classical auto j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 18.1 ms | 130.2 ms |
| chain | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 33.0 ms | 309.6 ms |
| additive | families: classical auto j1 | additive | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 58.8 ms | 251.4 ms |
| lambek-fo | families: intuitionistic auto j1 | two-sided | 20 | 18 | 10 | 8 | 0 | 0 | 0 | 2 | 0 | 3.9 ms | 549.2 ms |
| chain-fo | families: classical auto j1 | focus | 8 | 3 | 3 | 0 | 0 | 0 | 3 | 2 | 0 | 7.3 ms | 316.6 ms |
| witness-fo | families: intuitionistic auto j1 | two-sided | 16 | 15 | 8 | 7 | 0 | 1 | 0 | 0 | 0 | 66 µs | 2.54 s |
| contraction | families: intuitionistic auto j1 | inverse, two-sided | 6 | 4 | 3 | 1 | 0 | 0 | 2 | 0 | 0 | 64.9 ms | 139.5 ms |
| pigeonhole | families: intuitionistic-affine auto j1 | two-sided | 6 | 6 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 279 µs | 40.1 ms |
| euler | families: intuitionistic auto j1 | two-sided | 6 | 6 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 153 µs | 2.3 ms |
| duplicate | families: intuitionistic auto j1 | horn | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 33 µs | 102 µs |
| duplicate | families: classical auto j1 | horn | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 31 µs | 89 µs |
| reach-fo | families: intuitionistic auto j1 | two-sided | 4 | 2 | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 1.3 ms | 1.4 ms |
| Problems/LCL | iltp-classical: classical embedding j1 | focus | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 40 µs | 73 µs |
| Problems/LCL | iltp-classical: classical sat j1 | sat | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 34 µs | 62 µs |
| Problems/LCL | iltp-classical: classical auto j1 | focus | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 114 µs | 201 µs |
| Problems/LCL | iltp-classical: classical table j1 | table | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 11 µs | 22 µs |
| Problems/LCL | iltp-classical: classical tableau j1 | tableau | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 40 µs | 73 µs |
| Problems/SYJ | iltp-classical: classical embedding j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 28 µs | 157 µs |
| Problems/SYJ | iltp-classical: classical sat j1 | sat | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 20 µs | 116 µs |
| Problems/SYJ | iltp-classical: classical auto j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 74 µs | 408 µs |
| Problems/SYJ | iltp-classical: classical table j1 | table | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 10 µs | 54 µs |
| Problems/SYJ | iltp-classical: classical tableau j1 | tableau | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 20 µs | 147 µs |
| SYJ105 | iltp-classical: classical embedding j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 32 µs | 94 µs |
| SYJ105 | iltp-classical: classical sat j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 27 µs | 83 µs |
| SYJ105 | iltp-classical: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 79 µs | 236 µs |
| SYJ105 | iltp-classical: classical table j1 | table | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 12 µs | 36 µs |
| SYJ105 | iltp-classical: classical tableau j1 | tableau | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 28 µs | 88 µs |
| SYJ107 | iltp-classical: classical embedding j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 192 µs | 1.5 ms |
| SYJ107 | iltp-classical: classical sat j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 41 µs | 150 µs |
| SYJ107 | iltp-classical: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 1.6 ms | 17.1 ms |
| SYJ107 | iltp-classical: classical table j1 | table | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 20 µs | 107 µs |
| SYJ107 | iltp-classical: classical tableau j1 | tableau | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 343 µs | 2.0 ms |
| SYJ201 | iltp-classical: classical embedding j1 | focus | 20 | 4 | 4 | 0 | 0 | 3 | 0 | 13 | 0 | 13.6 ms | 248.6 ms |
| SYJ201 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 88 µs | 1.7 ms |
| SYJ201 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 17.6 ms | 327.6 ms |
| SYJ201 | iltp-classical: classical table j1 | table | 20 | 4 | 4 | 0 | 0 | 0 | 0 | 16 | 0 | 27 µs | 140 µs |
| SYJ201 | iltp-classical: classical tableau j1 | tableau | 20 | 12 | 12 | 0 | 0 | 0 | 0 | 8 | 0 | 5.6 ms | 104.8 ms |
| SYJ202 | iltp-classical: classical embedding j1 | focus | 20 | 2 | 2 | 0 | 0 | 2 | 0 | 16 | 0 | 290 µs | 333 µs |
| SYJ202 | iltp-classical: classical sat j1 | sat | 20 | 10 | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 2.2 ms | 1.53 s |
| SYJ202 | iltp-classical: classical auto j1 | focus, sat | 20 | 10 | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 108.9 ms | 2.13 s |
| SYJ202 | iltp-classical: classical table j1 | table | 20 | 2 | 2 | 0 | 0 | 0 | 0 | 18 | 0 | 17 µs | 30 µs |
| SYJ202 | iltp-classical: classical tableau j1 | tableau | 20 | 2 | 2 | 0 | 0 | 0 | 0 | 18 | 0 | 280 µs | 311 µs |
| SYJ203 | iltp-classical: classical embedding j1 | focus | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 52 µs | 1.2 ms |
| SYJ203 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 47 µs | 940 µs |
| SYJ203 | iltp-classical: classical auto j1 | focus | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 361 µs | 8.4 ms |
| SYJ203 | iltp-classical: classical table j1 | table | 20 | 9 | 9 | 0 | 0 | 0 | 0 | 11 | 0 | 18 µs | 366 µs |
| SYJ203 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 224 µs | 5.0 ms |
| SYJ204 | iltp-classical: classical embedding j1 | focus | 20 | 13 | 13 | 0 | 0 | 0 | 0 | 7 | 0 | 767 µs | 1.39 s |
| SYJ204 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 44 µs | 827 µs |
| SYJ204 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 14.5 ms | 225.4 ms |
| SYJ204 | iltp-classical: classical table j1 | table | 20 | 9 | 9 | 0 | 0 | 0 | 0 | 11 | 0 | 16 µs | 262 µs |
| SYJ204 | iltp-classical: classical tableau j1 | tableau | 20 | 12 | 12 | 0 | 0 | 0 | 0 | 8 | 0 | 1.1 ms | 109.1 ms |
| SYJ205 | iltp-classical: classical embedding j1 | focus | 20 | 17 | 17 | 0 | 0 | 0 | 0 | 3 | 0 | 2.6 ms | 2.30 s |
| SYJ205 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 76 µs | 1.5 ms |
| SYJ205 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 14.8 ms | 236.0 ms |
| SYJ205 | iltp-classical: classical table j1 | table | 20 | 3 | 3 | 0 | 0 | 0 | 0 | 17 | 0 | 22 µs | 90 µs |
| SYJ205 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 1.4 ms | 37.2 ms |
| SYJ206 | iltp-classical: classical embedding j1 | focus | 20 | 5 | 5 | 0 | 0 | 8 | 0 | 7 | 0 | 119 µs | 520.1 ms |
| SYJ206 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 288 µs | 5.2 ms |
| SYJ206 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 7.2 ms | 269.0 ms |
| SYJ206 | iltp-classical: classical table j1 | table | 20 | 10 | 10 | 0 | 0 | 0 | 0 | 10 | 0 | 16 µs | 223 µs |
| SYJ206 | iltp-classical: classical tableau j1 | tableau | 20 | 6 | 6 | 0 | 0 | 0 | 0 | 14 | 0 | 931 µs | 27.0 ms |
| SYJ207 | iltp-classical: classical embedding j1 | focus | 20 | 5 | 5 | 0 | 0 | 3 | 0 | 12 | 0 | 3.4 ms | 1.05 s |
| SYJ207 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 79 µs | 1.6 ms |
| SYJ207 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 17.8 ms | 311.7 ms |
| SYJ207 | iltp-classical: classical table j1 | table | 20 | 4 | 4 | 0 | 0 | 0 | 0 | 16 | 0 | 28 µs | 128 µs |
| SYJ207 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 51 µs | 981 µs |
| SYJ208 | iltp-classical: classical embedding j1 | focus | 20 | 2 | 2 | 0 | 0 | 2 | 0 | 16 | 0 | 257 µs | 290 µs |
| SYJ208 | iltp-classical: classical sat j1 | sat | 20 | 10 | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 2.3 ms | 1.59 s |
| SYJ208 | iltp-classical: classical auto j1 | focus, sat | 20 | 10 | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 109.5 ms | 2.26 s |
| SYJ208 | iltp-classical: classical table j1 | table | 20 | 2 | 2 | 0 | 0 | 0 | 0 | 18 | 0 | 18 µs | 29 µs |
| SYJ208 | iltp-classical: classical tableau j1 | tableau | 20 | 2 | 2 | 0 | 0 | 0 | 0 | 18 | 0 | 294 µs | 322 µs |
| SYJ209 | iltp-classical: classical embedding j1 | focus | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 59 µs | 1.2 ms |
| SYJ209 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 51 µs | 969 µs |
| SYJ209 | iltp-classical: classical auto j1 | focus | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 368 µs | 8.5 ms |
| SYJ209 | iltp-classical: classical table j1 | table | 20 | 9 | 9 | 0 | 0 | 0 | 0 | 11 | 0 | 19 µs | 379 µs |
| SYJ209 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 215 µs | 5.2 ms |
| SYJ210 | iltp-classical: classical embedding j1 | focus | 20 | 13 | 13 | 0 | 0 | 0 | 0 | 7 | 0 | 752 µs | 1.43 s |
| SYJ210 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 45 µs | 839 µs |
| SYJ210 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 14.9 ms | 231.2 ms |
| SYJ210 | iltp-classical: classical table j1 | table | 20 | 9 | 9 | 0 | 0 | 0 | 0 | 11 | 0 | 14 µs | 271 µs |
| SYJ210 | iltp-classical: classical tableau j1 | tableau | 20 | 12 | 12 | 0 | 0 | 0 | 0 | 8 | 0 | 1.1 ms | 111.3 ms |
| SYJ211 | iltp-classical: classical embedding j1 | focus | 20 | 18 | 18 | 0 | 0 | 0 | 0 | 2 | 0 | 2.4 ms | 1.99 s |
| SYJ211 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 60 µs | 1.2 ms |
| SYJ211 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 15.2 ms | 224.2 ms |
| SYJ211 | iltp-classical: classical table j1 | table | 20 | 3 | 3 | 0 | 0 | 0 | 0 | 17 | 0 | 18 µs | 81 µs |
| SYJ211 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 279 µs | 5.9 ms |
| SYJ212 | iltp-classical: classical embedding j1 | focus | 20 | 5 | 5 | 0 | 0 | 8 | 0 | 7 | 0 | 119 µs | 511.5 ms |
| SYJ212 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 254 µs | 5.3 ms |
| SYJ212 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 7.2 ms | 255.3 ms |
| SYJ212 | iltp-classical: classical table j1 | table | 20 | 10 | 10 | 0 | 0 | 0 | 0 | 10 | 0 | 17 µs | 231 µs |
| SYJ212 | iltp-classical: classical tableau j1 | tableau | 20 | 6 | 6 | 0 | 0 | 0 | 0 | 14 | 0 | 1.1 ms | 29.3 ms |
| Problems/SYN | iltp-classical: classical embedding j1 | focus | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 34 µs | 783 µs |
| Problems/SYN | iltp-classical: classical sat j1 | sat | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 29 µs | 581 µs |
| Problems/SYN | iltp-classical: classical auto j1 | focus | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 92 µs | 2.6 ms |
| Problems/SYN | iltp-classical: classical table j1 | table | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 12 µs | 238 µs |
| Problems/SYN | iltp-classical: classical tableau j1 | tableau | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 36 µs | 941 µs |
| SYN007 | iltp-classical: classical embedding j1 |  | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| SYN007 | iltp-classical: classical sat j1 | sat | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 624 µs | 624 µs |
| SYN007 | iltp-classical: classical auto j1 | sat | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 658 µs | 658 µs |
| SYN007 | iltp-classical: classical table j1 | table | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| SYN007 | iltp-classical: classical tableau j1 | tableau | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| Problems/LCL | iltp: intuitionistic embedding j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 114 µs | 179 µs |
| Problems/LCL | iltp: intuitionistic ljt j1 | ljt | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 19 µs | 37 µs |
| Problems/LCL | iltp: intuitionistic sat j1 | sat | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 48 µs | 94 µs |
| Problems/LCL | iltp: intuitionistic auto j1 | sat | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 47 µs | 93 µs |
| Problems/LCL | iltp: minimal embedding j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 112 µs | 173 µs |
| Problems/LCL | iltp: minimal ljt j1 | ljt | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 18 µs | 36 µs |
| Problems/LCL | iltp: minimal sat j1 | sat | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 49 µs | 97 µs |
| Problems/LCL | iltp: minimal auto j1 | sat | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 50 µs | 96 µs |
| Problems/SYJ | iltp: intuitionistic embedding j1 | horn, two-sided | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 67 µs | 423 µs |
| Problems/SYJ | iltp: intuitionistic ljt j1 | ljt | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 23 µs | 127 µs |
| Problems/SYJ | iltp: intuitionistic sat j1 | sat | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 35 µs | 205 µs |
| Problems/SYJ | iltp: intuitionistic auto j1 | sat | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 47 µs | 273 µs |
| Problems/SYJ | iltp: minimal embedding j1 | horn, two-sided | 5 | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 97 µs | 498 µs |
| Problems/SYJ | iltp: minimal ljt j1 | ljt | 5 | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 21 µs | 107 µs |
| Problems/SYJ | iltp: minimal sat j1 | sat | 5 | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 33 µs | 171 µs |
| Problems/SYJ | iltp: minimal auto j1 | sat | 5 | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 41 µs | 210 µs |
| SYJ105 | iltp: intuitionistic embedding j1 | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 78 µs | 276 µs |
| SYJ105 | iltp: intuitionistic ljt j1 | ljt | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 44 µs | 131 µs |
| SYJ105 | iltp: intuitionistic sat j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 62 µs | 215 µs |
| SYJ105 | iltp: intuitionistic auto j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 98 µs | 318 µs |
| SYJ105 | iltp: minimal embedding j1 | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 75 µs | 264 µs |
| SYJ105 | iltp: minimal ljt j1 | ljt | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 42 µs | 134 µs |
| SYJ105 | iltp: minimal sat j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 66 µs | 220 µs |
| SYJ105 | iltp: minimal auto j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 106 µs | 323 µs |
| SYJ107 | iltp: intuitionistic embedding j1 | two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 2.2 ms | 24.5 ms |
| SYJ107 | iltp: intuitionistic ljt j1 | ljt | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 77 µs | 320 µs |
| SYJ107 | iltp: intuitionistic sat j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 95 µs | 364 µs |
| SYJ107 | iltp: intuitionistic auto j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 167 µs | 641 µs |
| SYJ107 | iltp: minimal embedding j1 | two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 2.3 ms | 24.6 ms |
| SYJ107 | iltp: minimal ljt j1 | ljt | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 81 µs | 304 µs |
| SYJ107 | iltp: minimal sat j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 99 µs | 369 µs |
| SYJ107 | iltp: minimal auto j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 171 µs | 659 µs |
| SYJ201 | iltp: intuitionistic embedding j1 | two-sided | 20 | 5 | 5 | 0 | 0 | 15 | 0 | 0 | 0 | 5.6 ms | 906.8 ms |
| SYJ201 | iltp: intuitionistic ljt j1 | ljt | 20 | 3 | 3 | 0 | 0 | 0 | 0 | 17 | 0 | 846 µs | 3.1 ms |
| SYJ201 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 878 µs | 19.9 ms |
| SYJ201 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 9.8 ms | 299.9 ms |
| SYJ201 | iltp: minimal embedding j1 | two-sided | 20 | 5 | 5 | 0 | 0 | 15 | 0 | 0 | 0 | 5.5 ms | 898.0 ms |
| SYJ201 | iltp: minimal ljt j1 | ljt | 20 | 3 | 3 | 0 | 0 | 0 | 0 | 17 | 0 | 825 µs | 3.1 ms |
| SYJ201 | iltp: minimal sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 870 µs | 20.1 ms |
| SYJ201 | iltp: minimal auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 9.8 ms | 299.0 ms |
| SYJ202 | iltp: intuitionistic embedding j1 | two-sided | 20 | 3 | 3 | 0 | 0 | 6 | 0 | 11 | 0 | 377 µs | 83.6 ms |
| SYJ202 | iltp: intuitionistic ljt j1 | ljt | 20 | 3 | 3 | 0 | 0 | 3 | 0 | 14 | 0 | 62 µs | 834 µs |
| SYJ202 | iltp: intuitionistic sat j1 | sat | 20 | 3 | 3 | 0 | 0 | 15 | 0 | 2 | 0 | 61 µs | 349 µs |
| SYJ202 | iltp: intuitionistic auto j1 | sat, two-sided | 20 | 3 | 3 | 0 | 0 | 17 | 0 | 0 | 0 | 119 µs | 1.2 ms |
| SYJ202 | iltp: minimal embedding j1 | two-sided | 20 | 3 | 3 | 0 | 0 | 6 | 0 | 11 | 0 | 385 µs | 84.2 ms |
| SYJ202 | iltp: minimal ljt j1 | ljt | 20 | 3 | 3 | 0 | 0 | 3 | 0 | 14 | 0 | 59 µs | 846 µs |
| SYJ202 | iltp: minimal sat j1 | sat | 20 | 3 | 3 | 0 | 0 | 15 | 0 | 2 | 0 | 59 µs | 363 µs |
| SYJ202 | iltp: minimal auto j1 | sat, two-sided | 20 | 3 | 3 | 0 | 0 | 17 | 0 | 0 | 0 | 119 µs | 1.2 ms |
| SYJ203 | iltp: intuitionistic embedding j1 | inverse, two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 138.1 ms | 1.80 s |
| SYJ203 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 229 µs | 4.9 ms |
| SYJ203 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 207 µs | 4.2 ms |
| SYJ203 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 443 µs | 9.2 ms |
| SYJ203 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 130.3 ms | 1.79 s |
| SYJ203 | iltp: minimal ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 225 µs | 4.9 ms |
| SYJ203 | iltp: minimal sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 216 µs | 4.4 ms |
| SYJ203 | iltp: minimal auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 440 µs | 9.5 ms |
| SYJ204 | iltp: intuitionistic embedding j1 | two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 209 µs | 6.0 ms |
| SYJ204 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 59 µs | 1.1 ms |
| SYJ204 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 54 µs | 1.0 ms |
| SYJ204 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 105 µs | 2.0 ms |
| SYJ204 | iltp: minimal embedding j1 | two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 210 µs | 6.0 ms |
| SYJ204 | iltp: minimal ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 55 µs | 1.1 ms |
| SYJ204 | iltp: minimal sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 53 µs | 1.0 ms |
| SYJ204 | iltp: minimal auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 103 µs | 2.0 ms |
| SYJ205 | iltp: intuitionistic embedding j1 | inverse, two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 119.2 ms | 1.68 s |
| SYJ205 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 477 µs | 11.1 ms |
| SYJ205 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 148 µs | 2.8 ms |
| SYJ205 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 598 µs | 13.9 ms |
| SYJ205 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 122.0 ms | 1.69 s |
| SYJ205 | iltp: minimal ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 460 µs | 11.1 ms |
| SYJ205 | iltp: minimal sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 145 µs | 2.7 ms |
| SYJ205 | iltp: minimal auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 596 µs | 13.7 ms |
| SYJ206 | iltp: intuitionistic embedding j1 | two-sided | 20 | 7 | 7 | 0 | 0 | 12 | 0 | 1 | 0 | 310 µs | 584.9 ms |
| SYJ206 | iltp: intuitionistic ljt j1 | ljt | 20 | 9 | 9 | 0 | 0 | 3 | 0 | 8 | 0 | 287 µs | 12.4 ms |
| SYJ206 | iltp: intuitionistic sat j1 | sat | 20 | 15 | 15 | 0 | 0 | 5 | 0 | 0 | 0 | 6.0 ms | 2.45 s |
| SYJ206 | iltp: intuitionistic auto j1 | sat | 20 | 14 | 14 | 0 | 1 | 5 | 0 | 0 | 0 | 9.6 ms | 943.1 ms |
| SYJ206 | iltp: minimal embedding j1 | two-sided | 20 | 7 | 7 | 0 | 0 | 12 | 0 | 1 | 0 | 310 µs | 578.4 ms |
| SYJ206 | iltp: minimal ljt j1 | ljt | 20 | 9 | 9 | 0 | 0 | 3 | 0 | 8 | 0 | 314 µs | 12.7 ms |
| SYJ206 | iltp: minimal sat j1 | sat | 20 | 15 | 15 | 0 | 0 | 5 | 0 | 0 | 0 | 7.0 ms | 2.03 s |
| SYJ206 | iltp: minimal auto j1 | sat | 20 | 15 | 15 | 0 | 0 | 5 | 0 | 0 | 0 | 10.8 ms | 2.59 s |
| SYJ207 | iltp: intuitionistic embedding j1 | two-sided | 20 | 3 | 0 | 3 | 0 | 17 | 0 | 0 | 0 | 5.1 ms | 486.4 ms |
| SYJ207 | iltp: intuitionistic ljt j1 | ljt | 20 | 5 | 0 | 5 | 0 | 15 | 0 | 0 | 0 | 3.7 ms | 771.6 ms |
| SYJ207 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 212 µs | 4.1 ms |
| SYJ207 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 195 µs | 4.0 ms |
| SYJ207 | iltp: minimal embedding j1 | two-sided | 20 | 3 | 0 | 3 | 0 | 17 | 0 | 0 | 0 | 5.2 ms | 487.7 ms |
| SYJ207 | iltp: minimal ljt j1 | ljt | 20 | 5 | 0 | 5 | 0 | 15 | 0 | 0 | 0 | 3.7 ms | 770.4 ms |
| SYJ207 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 195 µs | 4.1 ms |
| SYJ207 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 188 µs | 4.0 ms |
| SYJ208 | iltp: intuitionistic embedding j1 | two-sided | 20 | 3 | 0 | 3 | 0 | 6 | 0 | 11 | 0 | 1.3 ms | 346.0 ms |
| SYJ208 | iltp: intuitionistic ljt j1 | ljt | 20 | 6 | 0 | 6 | 0 | 4 | 0 | 10 | 0 | 680 µs | 402.6 ms |
| SYJ208 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 835 µs | 30.2 ms |
| SYJ208 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 808 µs | 29.9 ms |
| SYJ208 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 3 | 0 | 3 | 2 | 4 | 0 | 11 | 0 | 2.4 ms | 513.9 ms |
| SYJ208 | iltp: minimal ljt j1 | ljt | 20 | 6 | 0 | 6 | 0 | 4 | 0 | 10 | 0 | 640 µs | 354.3 ms |
| SYJ208 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 810 µs | 30.1 ms |
| SYJ208 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 793 µs | 29.9 ms |
| SYJ209 | iltp: intuitionistic embedding j1 | inverse, two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 122.0 ms | 2.37 s |
| SYJ209 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 121 µs | 2.7 ms |
| SYJ209 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 99 µs | 2.0 ms |
| SYJ209 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 104 µs | 2.0 ms |
| SYJ209 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 132.9 ms | 2.59 s |
| SYJ209 | iltp: minimal ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 109 µs | 2.5 ms |
| SYJ209 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 106 µs | 2.1 ms |
| SYJ209 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 98 µs | 2.0 ms |
| SYJ210 | iltp: intuitionistic embedding j1 | two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 400 µs | 21.6 ms |
| SYJ210 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 27 µs | 560 µs |
| SYJ210 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 52 µs | 1.0 ms |
| SYJ210 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 50 µs | 976 µs |
| SYJ210 | iltp: minimal embedding j1 | two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 405 µs | 14.9 ms |
| SYJ210 | iltp: minimal ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 15 µs | 304 µs |
| SYJ210 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 50 µs | 1.0 ms |
| SYJ210 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 50 µs | 973 µs |
| SYJ211 | iltp: intuitionistic embedding j1 | inverse, two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 91.4 ms | 1.73 s |
| SYJ211 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 5.2 ms | 285.8 ms |
| SYJ211 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 139 µs | 2.9 ms |
| SYJ211 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 128 µs | 2.6 ms |
| SYJ211 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 96.0 ms | 1.76 s |
| SYJ211 | iltp: minimal ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 5.5 ms | 317.0 ms |
| SYJ211 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 137 µs | 2.9 ms |
| SYJ211 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 130 µs | 2.7 ms |
| SYJ212 | iltp: intuitionistic embedding j1 | two-sided | 20 | 14 | 0 | 14 | 0 | 5 | 0 | 1 | 0 | 4.3 ms | 2.49 s |
| SYJ212 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 105 µs | 2.2 ms |
| SYJ212 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 351 µs | 119.1 ms |
| SYJ212 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 334 µs | 113.6 ms |
| SYJ212 | iltp: minimal embedding j1 | two-sided | 20 | 14 | 0 | 14 | 2 | 3 | 0 | 1 | 0 | 3.9 ms | 2.31 s |
| SYJ212 | iltp: minimal ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 69 µs | 1.4 ms |
| SYJ212 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 164 µs | 3.1 ms |
| SYJ212 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 147 µs | 3.1 ms |
| Problems/SYN | iltp: intuitionistic embedding j1 | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 0 | 47 µs | 1.6 ms |
| Problems/SYN | iltp: intuitionistic ljt j1 | ljt | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 0 | 19 µs | 415 µs |
| Problems/SYN | iltp: intuitionistic sat j1 | sat | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 0 | 42 µs | 908 µs |
| Problems/SYN | iltp: intuitionistic auto j1 | sat | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 0 | 42 µs | 1.0 ms |
| Problems/SYN | iltp: minimal embedding j1 | two-sided | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 0 | 101 µs | 1.7 ms |
| Problems/SYN | iltp: minimal ljt j1 | ljt | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 0 | 18 µs | 395 µs |
| Problems/SYN | iltp: minimal sat j1 | sat | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 0 | 40 µs | 872 µs |
| Problems/SYN | iltp: minimal auto j1 | sat | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 0 | 43 µs | 994 µs |
| SYN007 | iltp: intuitionistic embedding j1 |  | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| SYN007 | iltp: intuitionistic ljt j1 | ljt | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 187 µs | 187 µs |
| SYN007 | iltp: intuitionistic sat j1 | sat | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 372 µs | 372 µs |
| SYN007 | iltp: intuitionistic auto j1 | sat | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 359 µs | 359 µs |
| SYN007 | iltp: minimal embedding j1 |  | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| SYN007 | iltp: minimal ljt j1 | ljt | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 187 µs | 187 µs |
| SYN007 | iltp: minimal sat j1 | sat | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 371 µs | 371 µs |
| SYN007 | iltp: minimal auto j1 | sat | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 363 µs | 363 µs |
| counter | intuitionistic: intuitionistic auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 43 µs | 259 µs |
| counter-over | intuitionistic: intuitionistic auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 43 µs | 257 µs |
| chain | intuitionistic: intuitionistic auto j1 | inverse, two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 38.3 ms | 303.6 ms |
| 3-partition-yes | intuitionistic: intuitionistic auto j1 | two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 72 µs | 277 µs |
| 3-partition-no | intuitionistic: intuitionistic auto j1 | two-sided | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 296 µs | 296 µs |
| ILL/ILLTP-SYJ-01 | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 0 | 0 | 0 | 0 | 0 | 15 | 16 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 0 | 0 | 0 | 0 | 0 | 16 | 15 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 2 | 2 | 0 | 0 | 2 | 6 | 21 | 0 | 2.43 s | 2.53 s |
| ILL/petri-nets/MCC | lltp-all-cores: intuitionistic auto j16 | horn | 1009 | 991 | 991 | 0 | 0 | 17 | 0 | 1 | 0 | 260 µs | 11.93 s |
| ILL/ILLTP-SYJ-01 | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 5 | 0 | 0 | 0 | 0 | 0 | 1 | 4 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-classical: classical auto j1 | horn | 3137 | 3071 | 3071 | 0 | 0 | 58 | 0 | 8 | 0 | 401 µs | 39.23 s |
| ILL/misc | lltp-classical: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 56 µs | 247 µs |
| ILL/Non-theorems | lltp-classical: classical auto j1 | focus | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/KLE-cbv | lltp-classical: classical auto j1 | focus | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 0 | 52 µs | 4.4 ms |
| ILL/KLE-cbn | lltp-classical: classical auto j1 | focus | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 0 | 86 µs | 5.9 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-classical: classical auto j1 | focus | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 32 µs | 784 µs |
| ILL/KLE-IMP-CONJ | lltp-classical: classical auto j1 | focus | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 49 µs | 9.4 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-classical: classical auto j1 | focus | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 42 µs | 1.4 ms |
| ILL/KLE-01 | lltp-classical: classical auto j1 | focus | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 0 | 56 µs | 3.4 ms |
| ILL/ILLTP-SYN-cbv | lltp-classical: classical auto j1 | focus | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 0 | 49 µs | 881 µs |
| ILL/ILLTP-SYN-cbn | lltp-classical: classical auto j1 | focus | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 0 | 80 µs | 1.3 ms |
| ILL/ILLTP-SYN-01 | lltp-classical: classical auto j1 | focus | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 0 | 47 µs | 704 µs |
| ILL/ILLTP-SYJ-cbv | lltp-classical: classical auto j1 | focus | 252 | 43 | 19 | 24 | 0 | 22 | 175 | 12 | 0 | 71 µs | 3.62 s |
| ILL/ILLTP-SYJ-cbn | lltp-classical: classical auto j1 | focus, horn | 252 | 38 | 13 | 25 | 0 | 3 | 187 | 24 | 0 | 123 µs | 4.2 ms |
| ILL/ILLTP-SYJ-01 | lltp-classical: classical auto j1 | focus, horn | 252 | 34 | 10 | 24 | 0 | 2 | 192 | 24 | 0 | 82 µs | 3.4 ms |
| ILL/ILLTP-LCL-cbv | lltp-classical: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 63 µs | 63 µs |
| ILL/ILLTP-LCL-cbn | lltp-classical: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 117 µs | 174 µs |
| ILL/ILLTP-LCL-01 | lltp-classical: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 70 µs | 70 µs |
| CLL/misc | lltp-classical: classical auto j1 | focus | 14 | 8 | 8 | 0 | 0 | 0 | 6 | 0 | 0 | 39 µs | 379 µs |
| CLL/Non-theorems | lltp-classical: classical auto j1 | focus | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 40 µs | 155 µs |
| ILL/ILLTP-SYJ-01 | lltp-classical-generous: classical auto j1 | focus | 4 | 0 | 0 | 0 | 0 | 1 | 1 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-classical-generous: classical auto j1 | focus | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-classical-generous: classical auto j1 | focus | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-classical-generous: classical auto j1 | horn | 25 | 25 | 25 | 0 | 0 | 0 | 0 | 0 | 0 | 244 µs | 36.5 ms |
| ILL/ILLTP-LCL-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 127 µs | 220 µs |
| ILL/ILLTP-LCL-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 148 µs | 213 µs |
| ILL/ILLTP-LCL-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 86 µs | 166 µs |
| ILL/ILLTP-SYJ-01 | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 94 | 52 | 42 | 0 | 61 | 54 | 0 | 4 | 1.8 ms | 6.59 s |
| ILL/ILLTP-SYJ-cbn | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 124 | 64 | 60 | 0 | 49 | 36 | 0 | 4 | 1.2 ms | 15.80 s |
| ILL/ILLTP-SYJ-cbv | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 123 | 76 | 47 | 0 | 56 | 30 | 0 | 4 | 7.7 ms | 52.36 s |
| ILL/ILLTP-SYN-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 7 | 3 | 4 | 0 | 0 | 1 | 0 | 0 | 93 µs | 931 µs |
| ILL/ILLTP-SYN-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 8 | 3 | 5 | 0 | 0 | 0 | 0 | 0 | 102 µs | 761 µs |
| ILL/ILLTP-SYN-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 8 | 3 | 5 | 0 | 0 | 0 | 0 | 0 | 57 µs | 586 µs |
| ILL/KLE-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 37 | 28 | 9 | 0 | 0 | 1 | 0 | 8 | 135 µs | 37.9 ms |
| ILL/KLE-IMP-CONJ | lltp-copies-10: intuitionistic auto j1 | two-sided | 60 | 60 | 60 | 0 | 0 | 0 | 0 | 0 | 0 | 111 µs | 7.4 ms |
| ILL/KLE-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 38 | 28 | 10 | 0 | 0 | 0 | 0 | 9 | 108 µs | 9.3 ms |
| ILL/KLE-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 37 | 28 | 9 | 0 | 0 | 1 | 0 | 8 | 67 µs | 10.1 ms |
| ILL/Non-theorems | lltp-copies-10: intuitionistic auto j1 | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-copies-10: intuitionistic auto j1 | horn | 171 | 171 | 171 | 0 | 0 | 0 | 0 | 0 | 0 | 182 µs | 1.82 s |
| ILL/ILLTP-LCL-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 151 µs | 259 µs |
| ILL/ILLTP-LCL-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 158 µs | 273 µs |
| ILL/ILLTP-LCL-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 146 µs | 276 µs |
| ILL/ILLTP-SYJ-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | horn, inverse, two-sided | 252 | 137 | 67 | 70 | 0 | 87 | 0 | 28 | 4 | 17.2 ms | 19.08 s |
| ILL/ILLTP-SYJ-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | horn, inverse, two-sided | 252 | 164 | 85 | 79 | 0 | 61 | 0 | 27 | 4 | 726 µs | 8.76 s |
| ILL/ILLTP-SYJ-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | inverse, two-sided | 252 | 154 | 81 | 73 | 0 | 76 | 0 | 22 | 4 | 12.4 ms | 29.81 s |
| ILL/ILLTP-SYN-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 92 µs | 6.1 ms |
| ILL/ILLTP-SYN-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 3 | 110 µs | 1.9 ms |
| ILL/ILLTP-SYN-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 84 µs | 1.8 ms |
| ILL/KLE-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | inverse, two-sided | 88 | 87 | 76 | 11 | 1 | 0 | 0 | 0 | 11 | 115 µs | 621.9 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 108 µs | 3.2 ms |
| ILL/KLE-IMP-CONJ | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 222 | 222 | 222 | 0 | 0 | 0 | 0 | 0 | 0 | 131 µs | 32.3 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 95 µs | 2.1 ms |
| ILL/KLE-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 127 µs | 163.9 ms |
| ILL/KLE-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 87 µs | 212.7 ms |
| ILL/Non-theorems | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | – | – |
| ILL/misc | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 75 µs | 291 µs |
| ILL/petri-nets/MCC | lltp-default: intuitionistic auto j16 pool after 0.1s | horn | 3137 | 3069 | 3069 | 0 | 0 | 66 | 0 | 2 | 0 | 863 µs | 36.41 s |
| ILL/ILLTP-LCL-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 121 µs | 199 µs |
| ILL/ILLTP-LCL-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 67 µs | 132 µs |
| ILL/ILLTP-LCL-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 94 µs | 175 µs |
| ILL/ILLTP-SYJ-01 | lltp-forward: intuitionistic auto j1 bias factors | horn, inverse, two-sided | 252 | 139 | 69 | 70 | 0 | 87 | 1 | 25 | 4 | 12.1 ms | 33.09 s |
| ILL/ILLTP-SYJ-cbn | lltp-forward: intuitionistic auto j1 bias factors | horn, inverse, two-sided | 252 | 170 | 86 | 84 | 0 | 59 | 0 | 23 | 4 | 454 µs | 17.59 s |
| ILL/ILLTP-SYJ-cbv | lltp-forward: intuitionistic auto j1 bias factors | inverse, two-sided | 252 | 162 | 86 | 76 | 0 | 78 | 0 | 12 | 4 | 13.5 ms | 82.36 s |
| ILL/ILLTP-SYN-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 64 µs | 4.3 ms |
| ILL/ILLTP-SYN-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 3 | 52 µs | 1.1 ms |
| ILL/ILLTP-SYN-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 55 µs | 1.3 ms |
| ILL/KLE-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 87 | 76 | 11 | 0 | 0 | 1 | 0 | 10 | 66 µs | 1.03 s |
| ILL/KLE-IMP-CONJ/ALT | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 39 µs | 1.1 ms |
| ILL/KLE-IMP-CONJ | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 222 | 222 | 222 | 0 | 0 | 0 | 0 | 0 | 0 | 52 µs | 15.4 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 32 µs | 770 µs |
| ILL/KLE-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 47 µs | 40.8 ms |
| ILL/KLE-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 87 | 76 | 11 | 0 | 0 | 1 | 0 | 10 | 58 µs | 87.5 ms |
| ILL/Non-theorems | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 50 µs | 160 µs |
| ILL/petri-nets/MCC | lltp-forward: intuitionistic auto j1 bias factors | horn | 3137 | 3071 | 3071 | 0 | 0 | 57 | 0 | 9 | 0 | 412 µs | 37.03 s |
| ILL/ILLTP-LCL-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 102 µs | 102 µs |
| ILL/ILLTP-LCL-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 149 µs | 231 µs |
| ILL/ILLTP-LCL-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 78 µs | 78 µs |
| ILL/ILLTP-SYJ-01 | lltp-intuitionistic: intuitionistic auto j1 | horn, two-sided | 252 | 34 | 10 | 24 | 0 | 4 | 189 | 25 | 3 | 88 µs | 3.6 ms |
| ILL/ILLTP-SYJ-cbn | lltp-intuitionistic: intuitionistic auto j1 | horn, two-sided | 252 | 38 | 13 | 25 | 0 | 4 | 186 | 24 | 4 | 141 µs | 5.0 ms |
| ILL/ILLTP-SYJ-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 252 | 43 | 19 | 24 | 0 | 23 | 174 | 12 | 3 | 74 µs | 3.91 s |
| ILL/ILLTP-SYN-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 2 | 56 µs | 793 µs |
| ILL/ILLTP-SYN-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 3 | 87 µs | 1.6 ms |
| ILL/ILLTP-SYN-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 2 | 57 µs | 1.1 ms |
| ILL/KLE-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 10 | 68 µs | 4.0 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 54 µs | 1.7 ms |
| ILL/KLE-IMP-CONJ | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 55 µs | 10.3 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 35 µs | 858 µs |
| ILL/KLE-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 10 | 88 µs | 6.2 ms |
| ILL/KLE-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 10 | 58 µs | 4.9 ms |
| ILL/Non-theorems | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 62 µs | 263 µs |
| ILL/petri-nets/MCC | lltp-intuitionistic: intuitionistic auto j1 | horn | 3137 | 3071 | 3071 | 0 | 0 | 58 | 0 | 8 | 0 | 431 µs | 40.04 s |
| ILL/ILLTP-SYJ-01 | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-intuitionistic-generous: intuitionistic auto j1 | horn | 19 | 19 | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 277 µs | 35.8 ms |
| ILL/ILLTP-LCL-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 100 µs | 100 µs |
| ILL/ILLTP-LCL-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 66 µs | 123 µs |
| ILL/ILLTP-LCL-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 77 µs | 77 µs |
| ILL/ILLTP-SYJ-01 | lltp-rarer: intuitionistic auto j1 bias rarer | horn, two-sided | 252 | 34 | 10 | 24 | 0 | 4 | 189 | 25 | 3 | 84 µs | 3.6 ms |
| ILL/ILLTP-SYJ-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | horn, two-sided | 252 | 37 | 13 | 24 | 0 | 5 | 187 | 23 | 4 | 68 µs | 2.5 ms |
| ILL/ILLTP-SYJ-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 252 | 43 | 19 | 24 | 0 | 23 | 174 | 12 | 3 | 71 µs | 3.89 s |
| ILL/ILLTP-SYN-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 2 | 52 µs | 767 µs |
| ILL/ILLTP-SYN-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 3 | 48 µs | 958 µs |
| ILL/ILLTP-SYN-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 2 | 57 µs | 1.0 ms |
| ILL/KLE-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 10 | 62 µs | 3.8 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 46 µs | 1.2 ms |
| ILL/KLE-IMP-CONJ | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 46 µs | 7.8 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 35 µs | 816 µs |
| ILL/KLE-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 10 | 46 µs | 3.8 ms |
| ILL/KLE-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 10 | 50 µs | 4.2 ms |
| ILL/Non-theorems | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 44 µs | 148 µs |
| ILL/petri-nets/MCC | lltp-rarer: intuitionistic auto j1 bias rarer | horn | 3137 | 3071 | 3071 | 0 | 0 | 58 | 0 | 8 | 0 | 435 µs | 40.02 s |
| ILL/ILLTP-SYJ-01 | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 20 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 20 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 22 | 0 | 0 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-recursion: intuitionistic auto j1 | horn | 929 | 895 | 895 | 0 | 0 | 32 | 0 | 2 | 0 | 3.5 ms | 17.90 s |
| ILL/petri-nets/MCC | lltp-recursion-generous: intuitionistic auto j1 | horn | 9 | 9 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 2.5 ms | 29.6 ms |
| 3-partition-no | long-1: classical auto j1 | focus | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 375 µs | 375 µs |
| partition-no | long-1: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 448 µs | 448 µs |
| partition-yes | long-1: classical auto j1 | focus | 2 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 211 µs | 211 µs |
| counter | long-1: classical auto j1 | horn | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 57 µs | 112 µs |
| mix | long-2: mix auto j1 | inverse | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 272.2 ms | 272.2 ms |
| wide-m3 | long-2: classical auto j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 145 µs | 145 µs |
| wide-m4 | long-2: classical auto j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 130 µs | 130 µs |
| qbf | long-2: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 174.9 ms | 174.9 ms |
| 3-partition-yes | parallel: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 102 µs | 386 µs |
| 3-partition-yes | parallel: classical auto j2 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 262 µs | 1.0 ms |
| 3-partition-yes | parallel: classical auto j4 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 287 µs | 1.1 ms |
| 3-partition-yes | parallel: classical auto j8 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 414 µs | 1.6 ms |
| 3-partition-yes | parallel: classical auto j16 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 685 µs | 2.6 ms |
| 3-partition-no | parallel: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 432 µs | 815 µs |
| 3-partition-no | parallel: classical auto j2 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 586 µs | 1.1 ms |
| 3-partition-no | parallel: classical auto j4 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 484 µs | 918 µs |
| 3-partition-no | parallel: classical auto j8 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 708 µs | 1.4 ms |
| 3-partition-no | parallel: classical auto j16 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 813 µs | 1.5 ms |
| partition-yes | parallel: classical auto j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 267 µs | 64.25 s |
| partition-yes | parallel: classical auto j2 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 532 µs | 68.76 s |
| partition-yes | parallel: classical auto j4 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 612 µs | 73.29 s |
| partition-yes | parallel: classical auto j8 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 563 µs | 77.06 s |
| partition-yes | parallel: classical auto j16 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 878 µs | 84.09 s |
| partition-no | parallel: classical auto j1 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 584 µs | 3.53 s |
| partition-no | parallel: classical auto j2 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 775 µs | 2.05 s |
| partition-no | parallel: classical auto j4 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 591 µs | 1.20 s |
| partition-no | parallel: classical auto j8 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 763 µs | 750.5 ms |
| partition-no | parallel: classical auto j16 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 803 µs | 558.9 ms |
| qbf | parallel: classical auto j1 | focus | 20 | 19 | 3 | 16 | 0 | 0 | 0 | 1 | 0 | 132.1 ms | 200.36 s |
| qbf | parallel: classical auto j2 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 101.5 ms | 14.23 s |
| qbf | parallel: classical auto j4 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 101.6 ms | 14.29 s |
| qbf | parallel: classical auto j8 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 102.1 ms | 14.41 s |
| qbf | parallel: classical auto j16 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 100.2 ms | 14.19 s |
| mix | parallel: mix auto j1 | inverse | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 282.3 ms | 1.13 s |
| mix | parallel: mix auto j2 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 6.16 s | 76.47 s |
| mix | parallel: mix auto j4 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 6.48 s | 79.48 s |
| mix | parallel: mix auto j8 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 8.31 s | 93.31 s |
| mix | parallel: mix auto j16 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 9.57 s | 119.14 s |
| counter | parallel: classical auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 103 µs | 576 µs |
| counter | parallel: classical auto j2 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 124 µs | 756 µs |
| counter | parallel: classical auto j4 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 127 µs | 768 µs |
| counter | parallel: classical auto j8 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 130 µs | 752 µs |
| counter | parallel: classical auto j16 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 117 µs | 712 µs |
| counter-over | parallel: classical auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 94 µs | 555 µs |
| counter-over | parallel: classical auto j2 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 127 µs | 727 µs |
| counter-over | parallel: classical auto j4 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 123 µs | 711 µs |
| counter-over | parallel: classical auto j8 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 118 µs | 696 µs |
| counter-over | parallel: classical auto j16 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 129 µs | 729 µs |
| wide-m3 | parallel: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 143 µs | 446 µs |
| wide-m3 | parallel: classical auto j2 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 286 µs | 977 µs |
| wide-m3 | parallel: classical auto j4 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 340 µs | 1.0 ms |
| wide-m3 | parallel: classical auto j8 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 405 µs | 1.4 ms |
| wide-m3 | parallel: classical auto j16 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 625 µs | 1.9 ms |
| wide-m4 | parallel: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 162 µs | 489 µs |
| wide-m4 | parallel: classical auto j2 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 322 µs | 1.1 ms |
| wide-m4 | parallel: classical auto j4 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 360 µs | 1.1 ms |
| wide-m4 | parallel: classical auto j8 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 531 µs | 1.5 ms |
| wide-m4 | parallel: classical auto j16 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 636 µs | 1.9 ms |
| 3-partition-mll-no | parallel-net: classical net j1 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 971 µs | 3.0 ms |
| 3-partition-mll-no | parallel-net: classical net j2 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 2.9 ms |
| 3-partition-mll-no | parallel-net: classical net j4 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.1 ms | 3.5 ms |
| 3-partition-mll-no | parallel-net: classical net j8 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 3.8 ms |
| 3-partition-mll-no | parallel-net: classical net j16 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.4 ms | 4.3 ms |
| partition-table | parallel-net: classical net j1 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.8 ms | 4.98 s |
| partition-table | parallel-net: classical net j2 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 1.55 s |
| partition-table | parallel-net: classical net j4 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.4 ms | 1.23 s |
| partition-table | parallel-net: classical net j8 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 2.2 ms | 92.5 ms |
| partition-table | parallel-net: classical net j16 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 2.9 ms | 49.4 ms |
| wide-m1 | period-16: classical net j1 period 16 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 441.4 ms | 448.1 ms |
| wide-m2 | period-16: classical net j1 period 16 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 431.7 ms | 438.2 ms |
| 3-partition-mll-no | period-16: classical net j1 period 16 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 1.5 ms |
| partition-yes | period-16: classical net j1 period 16 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2.6 ms | 2.6 ms |
| partition-no | period-16: classical net j1 period 16 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 23.9 ms | 26.1 ms |
| wide-m1 | period-1: classical net j1 period 1 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 1.27 s | 1.29 s |
| wide-m2 | period-1: classical net j1 period 1 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 1.27 s | 1.29 s |
| 3-partition-mll-no | period-1: classical net j1 period 1 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 700 µs | 955 µs |
| partition-yes | period-1: classical net j1 period 1 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1.7 ms | 1.7 ms |
| partition-no | period-1: classical net j1 period 1 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 9.4 ms | 10.9 ms |
| wide-m1 | period-2: classical net j1 period 2 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 832.3 ms | 844.5 ms |
| wide-m2 | period-2: classical net j1 period 2 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 825.3 ms | 837.5 ms |
| 3-partition-mll-no | period-2: classical net j1 period 2 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 721 µs | 963 µs |
| partition-yes | period-2: classical net j1 period 2 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1.3 ms | 1.3 ms |
| partition-no | period-2: classical net j1 period 2 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 7.7 ms | 8.9 ms |
| wide-m1 | period-8: classical net j1 period 8 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 496.2 ms | 503.6 ms |
| wide-m2 | period-8: classical net j1 period 8 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 493.7 ms | 501.1 ms |
| 3-partition-mll-no | period-8: classical net j1 period 8 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 740 µs | 984 µs |
| partition-yes | period-8: classical net j1 period 8 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5.2 ms | 5.2 ms |
| partition-no | period-8: classical net j1 period 8 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 22.7 ms | 24.2 ms |

## Counters of the decided problems

| family | configuration | decided | nodes | splits | memo_hits | memo_entries | links | tests |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| partition-yes | engines: classical focus j1 | 5 | 330930 | 2909630 | 277975 | 52955 | 0 | 0 |
| partition-yes | engines: classical net j1 | 3 | 98810 | 0 | 0 | 0 | 118133 | 118133 |
| partition-no | engines: classical focus j1 | 4 | 4369900 | 28361297 | 3818668 | 551232 | 0 | 0 |
| partition-no | engines: classical net j1 | 2 | 2486 | 0 | 0 | 0 | 3890 | 3890 |
| 3-partition-mll-yes | engines: classical focus j1 | 4 | 36 | 176 | 0 | 36 | 0 | 0 |
| 3-partition-mll-yes | engines: classical net j1 | 4 | 84 | 0 | 0 | 0 | 84 | 84 |
| 3-partition-mll-no | engines: classical focus j1 | 4 | 4 | 96 | 0 | 4 | 0 | 0 |
| 3-partition-mll-no | engines: classical net j1 | 4 | 1394 | 0 | 0 | 0 | 2615 | 2615 |
| wide-m1 | engines: classical focus j1 | 6 | 2390 | 4268900 | 0 | 2390 | 0 | 0 |
| wide-m1 | engines: classical net j1 | 6 | 4768 | 0 | 0 | 0 | 4768 | 1264 |
| wide-m2 | engines: classical focus j1 | 6 | 2390 | 3203460 | 0 | 2390 | 0 | 0 |
| wide-m2 | engines: classical net j1 | 6 | 4768 | 0 | 0 | 0 | 4768 | 1264 |
| wide-m3 | engines: classical focus j1 | 5 | 2375 | 2848074 | 0 | 2375 | 0 | 0 |
| wide-m3 | engines: classical net j1 | 5 | 4740 | 0 | 0 | 0 | 4740 | 1239 |
| wide-m4 | engines: classical focus j1 | 5 | 2373 | 2670434 | 0 | 2373 | 0 | 0 |
| wide-m4 | engines: classical net j1 | 5 | 4736 | 0 | 0 | 0 | 4736 | 1280 |
| additive | engines: classical focus j1 | 4 | 3036271 | 0 | 869899 | 1487175 | 0 | 0 |
| partition-table | engines: classical focus j1 | 13 | 3629 | 13830 | 2550 | 1079 | 0 | 0 |
| partition-table | engines: classical net j1 | 13 | 2080512 | 0 | 0 | 0 | 2203635 | 804647 |
| cancellation | engines: classical focus j1 | 1 | 925 | 2794 | 713 | 212 | 0 | 0 |
| 3-partition-yes | families: classical auto j1 | 4 | 28 | 140 | 0 | 28 | 0 | 0 |
| 3-partition-no | families: classical auto j1 | 2 | 1894 | 6088 | 1446 | 448 | 0 | 0 |
| 3-partition-mll-yes | families: classical auto j1 | 4 | 36 | 176 | 0 | 36 | 0 | 0 |
| 3-partition-mll-no | families: classical auto j1 | 4 | 4 | 96 | 0 | 4 | 0 | 0 |
| partition-yes | families: classical auto j1 | 8 | 38671826 | 388163209 | 33889921 | 1120561 | 0 | 0 |
| partition-no | families: classical auto j1 | 5 | 4370711 | 28365458 | 3819236 | 551475 | 0 | 0 |
| qbf | families: classical auto j1 | 34 | 825287750 | 5425130223 | 90299364 | 13899059 | 0 | 0 |
| wide-m1 | families: classical auto j1 | 6 | 2390 | 4268900 | 0 | 2390 | 0 | 0 |
| wide-m2 | families: classical auto j1 | 6 | 2390 | 3203460 | 0 | 2390 | 0 | 0 |
| wide-m3 | families: classical auto j1 | 7 | 3437 | 3551165 | 0 | 3437 | 0 | 0 |
| wide-m4 | families: classical auto j1 | 8 | 3468 | 3330514 | 0 | 3468 | 0 | 0 |
| mix | families: mix auto j1 | 6 | 25582 | 439720 | 21232 | 4350 | 0 | 0 |
| counter | families: classical auto j1 | 6 | 120 | 0 | 0 | 120 | 0 | 0 |
| counter-over | families: classical auto j1 | 6 | 171 | 0 | 23 | 148 | 0 | 0 |
| growing | families: classical auto j1 | 4 | 333891 | 166769 | 165396 | 1726 | 0 | 0 |
| chain | families: classical auto j1 | 4 | 86060 | 9484732 | 42220 | 928 | 0 | 0 |
| additive | families: classical auto j1 | 4 | 14404893 | 0 | 3788841 | 2442270 | 0 | 0 |
| lambek-fo | families: intuitionistic auto j1 | 18 | 684 | 13129420 | 0 | 8 | 0 | 0 |
| chain-fo | families: classical auto j1 | 3 | 35451 | 244782 | 0 | 336 | 0 | 0 |
| witness-fo | families: intuitionistic auto j1 | 15 | 83 | 56174203 | 0 | 71 | 0 | 0 |
| contraction | families: intuitionistic auto j1 | 4 | 432903 | 150221 | 310089 | 71663 | 0 | 0 |
| pigeonhole | families: intuitionistic-affine auto j1 | 6 | 547985 | 1002173 | 512974 | 35011 | 0 | 0 |
| euler | families: intuitionistic auto j1 | 6 | 4315 | 31118 | 1893 | 2422 | 0 | 0 |
| duplicate | families: intuitionistic auto j1 | 3 | 11 | 0 | 0 | 11 | 0 | 0 |
| duplicate | families: classical auto j1 | 3 | 11 | 0 | 0 | 11 | 0 | 0 |
| reach-fo | families: intuitionistic auto j1 | 2 | 378 | 4652 | 188 | 100 | 0 | 0 |
| Problems/LCL | iltp-classical: classical embedding j1 | 2 | 7 | 0 | 0 | 7 | 0 | 0 |
| Problems/LCL | iltp-classical: classical sat j1 | 2 | 4 | 0 | 0 | 0 | 0 | 0 |
| Problems/LCL | iltp-classical: classical auto j1 | 2 | 7 | 0 | 0 | 7 | 0 | 0 |
| Problems/LCL | iltp-classical: classical table j1 | 2 | 12 | 0 | 0 | 0 | 0 | 0 |
| Problems/LCL | iltp-classical: classical tableau j1 | 2 | 24 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYJ | iltp-classical: classical embedding j1 | 5 | 24 | 0 | 0 | 24 | 0 | 0 |
| Problems/SYJ | iltp-classical: classical sat j1 | 5 | 2 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYJ | iltp-classical: classical auto j1 | 5 | 24 | 0 | 0 | 24 | 0 | 0 |
| Problems/SYJ | iltp-classical: classical table j1 | 5 | 44 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYJ | iltp-classical: classical tableau j1 | 5 | 47 | 0 | 0 | 0 | 0 | 0 |
| SYJ105 | iltp-classical: classical embedding j1 | 3 | 6 | 0 | 0 | 6 | 0 | 0 |
| SYJ105 | iltp-classical: classical sat j1 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |
| SYJ105 | iltp-classical: classical auto j1 | 3 | 6 | 0 | 0 | 6 | 0 | 0 |
| SYJ105 | iltp-classical: classical table j1 | 3 | 14 | 0 | 0 | 0 | 0 | 0 |
| SYJ105 | iltp-classical: classical tableau j1 | 3 | 27 | 0 | 0 | 0 | 0 | 0 |
| SYJ107 | iltp-classical: classical embedding j1 | 4 | 3510 | 0 | 0 | 3510 | 0 | 0 |
| SYJ107 | iltp-classical: classical sat j1 | 4 | 10 | 0 | 0 | 0 | 0 | 0 |
| SYJ107 | iltp-classical: classical auto j1 | 4 | 3510 | 0 | 0 | 3510 | 0 | 0 |
| SYJ107 | iltp-classical: classical table j1 | 4 | 680 | 0 | 0 | 0 | 0 | 0 |
| SYJ107 | iltp-classical: classical tableau j1 | 4 | 832 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp-classical: classical embedding j1 | 4 | 193752 | 0 | 0 | 193752 | 0 | 0 |
| SYJ201 | iltp-classical: classical sat j1 | 20 | 40 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp-classical: classical auto j1 | 20 | 1332 | 0 | 0 | 1296 | 0 | 0 |
| SYJ201 | iltp-classical: classical table j1 | 4 | 680 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp-classical: classical tableau j1 | 12 | 17244 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp-classical: classical embedding j1 | 2 | 513 | 0 | 0 | 513 | 0 | 0 |
| SYJ202 | iltp-classical: classical sat j1 | 10 | 140543 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp-classical: classical auto j1 | 10 | 141054 | 0 | 0 | 513 | 0 | 0 |
| SYJ202 | iltp-classical: classical table j1 | 2 | 68 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp-classical: classical tableau j1 | 2 | 114 | 0 | 0 | 0 | 0 | 0 |
| SYJ203 | iltp-classical: classical embedding j1 | 20 | 230 | 0 | 0 | 230 | 0 | 0 |
| SYJ203 | iltp-classical: classical sat j1 | 20 | 20 | 0 | 0 | 0 | 0 | 0 |
| SYJ203 | iltp-classical: classical auto j1 | 20 | 230 | 0 | 0 | 230 | 0 | 0 |
| SYJ203 | iltp-classical: classical table j1 | 9 | 2044 | 0 | 0 | 0 | 0 | 0 |
| SYJ203 | iltp-classical: classical tableau j1 | 20 | 860 | 0 | 0 | 0 | 0 | 0 |
| SYJ204 | iltp-classical: classical embedding j1 | 13 | 2391483 | 0 | 0 | 1845736 | 0 | 0 |
| SYJ204 | iltp-classical: classical sat j1 | 20 | 19 | 0 | 0 | 0 | 0 | 0 |
| SYJ204 | iltp-classical: classical auto j1 | 20 | 9852 | 0 | 0 | 9840 | 0 | 0 |
| SYJ204 | iltp-classical: classical table j1 | 9 | 2044 | 0 | 0 | 0 | 0 | 0 |
| SYJ204 | iltp-classical: classical tableau j1 | 12 | 32724 | 0 | 0 | 0 | 0 | 0 |
| SYJ205 | iltp-classical: classical embedding j1 | 17 | 2097136 | 0 | 0 | 2097136 | 0 | 0 |
| SYJ205 | iltp-classical: classical sat j1 | 20 | 87 | 0 | 0 | 0 | 0 | 0 |
| SYJ205 | iltp-classical: classical auto j1 | 20 | 8221 | 0 | 0 | 8176 | 0 | 0 |
| SYJ205 | iltp-classical: classical table j1 | 3 | 672 | 0 | 0 | 0 | 0 | 0 |
| SYJ205 | iltp-classical: classical tableau j1 | 20 | 7120 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp-classical: classical embedding j1 | 5 | 666178 | 0 | 0 | 666178 | 0 | 0 |
| SYJ206 | iltp-classical: classical sat j1 | 20 | 2318 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp-classical: classical auto j1 | 20 | 7192 | 0 | 0 | 4902 | 0 | 0 |
| SYJ206 | iltp-classical: classical table j1 | 10 | 2046 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp-classical: classical tableau j1 | 6 | 11195 | 0 | 0 | 0 | 0 | 0 |
| SYJ207 | iltp-classical: classical embedding j1 | 5 | 647694 | 0 | 0 | 647694 | 0 | 0 |
| SYJ207 | iltp-classical: classical sat j1 | 20 | 20 | 0 | 0 | 0 | 0 | 0 |
| SYJ207 | iltp-classical: classical auto j1 | 20 | 360 | 0 | 0 | 342 | 0 | 0 |
| SYJ207 | iltp-classical: classical table j1 | 4 | 680 | 0 | 0 | 0 | 0 | 0 |
| SYJ207 | iltp-classical: classical tableau j1 | 20 | 80 | 0 | 0 | 0 | 0 | 0 |
| SYJ208 | iltp-classical: classical embedding j1 | 2 | 513 | 0 | 0 | 513 | 0 | 0 |
| SYJ208 | iltp-classical: classical sat j1 | 10 | 146967 | 0 | 0 | 0 | 0 | 0 |
| SYJ208 | iltp-classical: classical auto j1 | 10 | 147477 | 0 | 0 | 513 | 0 | 0 |
| SYJ208 | iltp-classical: classical table j1 | 2 | 68 | 0 | 0 | 0 | 0 | 0 |
| SYJ208 | iltp-classical: classical tableau j1 | 2 | 131 | 0 | 0 | 0 | 0 | 0 |
| SYJ209 | iltp-classical: classical embedding j1 | 20 | 230 | 0 | 0 | 230 | 0 | 0 |
| SYJ209 | iltp-classical: classical sat j1 | 20 | 20 | 0 | 0 | 0 | 0 | 0 |
| SYJ209 | iltp-classical: classical auto j1 | 20 | 230 | 0 | 0 | 230 | 0 | 0 |
| SYJ209 | iltp-classical: classical table j1 | 9 | 2044 | 0 | 0 | 0 | 0 | 0 |
| SYJ209 | iltp-classical: classical tableau j1 | 20 | 900 | 0 | 0 | 0 | 0 | 0 |
| SYJ210 | iltp-classical: classical embedding j1 | 13 | 2391483 | 0 | 0 | 1845736 | 0 | 0 |
| SYJ210 | iltp-classical: classical sat j1 | 20 | 20 | 0 | 0 | 0 | 0 | 0 |
| SYJ210 | iltp-classical: classical auto j1 | 20 | 9852 | 0 | 0 | 9840 | 0 | 0 |
| SYJ210 | iltp-classical: classical table j1 | 9 | 2044 | 0 | 0 | 0 | 0 | 0 |
| SYJ210 | iltp-classical: classical tableau j1 | 12 | 32748 | 0 | 0 | 0 | 0 | 0 |
| SYJ211 | iltp-classical: classical embedding j1 | 18 | 2097144 | 0 | 0 | 2097144 | 0 | 0 |
| SYJ211 | iltp-classical: classical sat j1 | 20 | 20 | 0 | 0 | 0 | 0 | 0 |
| SYJ211 | iltp-classical: classical auto j1 | 20 | 8194 | 0 | 0 | 8184 | 0 | 0 |
| SYJ211 | iltp-classical: classical table j1 | 3 | 672 | 0 | 0 | 0 | 0 | 0 |
| SYJ211 | iltp-classical: classical tableau j1 | 20 | 1270 | 0 | 0 | 0 | 0 | 0 |
| SYJ212 | iltp-classical: classical embedding j1 | 5 | 666178 | 0 | 0 | 666178 | 0 | 0 |
| SYJ212 | iltp-classical: classical sat j1 | 20 | 2325 | 0 | 0 | 0 | 0 | 0 |
| SYJ212 | iltp-classical: classical auto j1 | 20 | 7197 | 0 | 0 | 4902 | 0 | 0 |
| SYJ212 | iltp-classical: classical table j1 | 10 | 2046 | 0 | 0 | 0 | 0 | 0 |
| SYJ212 | iltp-classical: classical tableau j1 | 6 | 12759 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYN | iltp-classical: classical embedding j1 | 19 | 295 | 0 | 0 | 295 | 0 | 0 |
| Problems/SYN | iltp-classical: classical sat j1 | 19 | 38 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYN | iltp-classical: classical auto j1 | 19 | 295 | 0 | 0 | 295 | 0 | 0 |
| Problems/SYN | iltp-classical: classical table j1 | 19 | 86 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYN | iltp-classical: classical tableau j1 | 19 | 383 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp-classical: classical embedding j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp-classical: classical sat j1 | 1 | 292 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp-classical: classical auto j1 | 1 | 292 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp-classical: classical table j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp-classical: classical tableau j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Problems/LCL | iltp: intuitionistic embedding j1 | 2 | 72 | 33 | 24 | 20 | 0 | 0 |
| Problems/LCL | iltp: intuitionistic ljt j1 | 2 | 18 | 0 | 0 | 18 | 0 | 0 |
| Problems/LCL | iltp: intuitionistic sat j1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| Problems/LCL | iltp: intuitionistic auto j1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| Problems/LCL | iltp: minimal embedding j1 | 2 | 72 | 33 | 24 | 20 | 0 | 0 |
| Problems/LCL | iltp: minimal ljt j1 | 2 | 16 | 0 | 0 | 16 | 0 | 0 |
| Problems/LCL | iltp: minimal sat j1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| Problems/LCL | iltp: minimal auto j1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYJ | iltp: intuitionistic embedding j1 | 5 | 240 | 141 | 102 | 68 | 0 | 0 |
| Problems/SYJ | iltp: intuitionistic ljt j1 | 5 | 25 | 0 | 1 | 25 | 0 | 0 |
| Problems/SYJ | iltp: intuitionistic sat j1 | 5 | 1 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYJ | iltp: intuitionistic auto j1 | 5 | 1 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYJ | iltp: minimal embedding j1 | 5 | 176 | 99 | 66 | 46 | 0 | 0 |
| Problems/SYJ | iltp: minimal ljt j1 | 5 | 34 | 0 | 0 | 34 | 0 | 0 |
| Problems/SYJ | iltp: minimal sat j1 | 5 | 1 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYJ | iltp: minimal auto j1 | 5 | 1 | 0 | 0 | 0 | 0 | 0 |
| SYJ105 | iltp: intuitionistic embedding j1 | 3 | 488 | 153 | 160 | 156 | 0 | 0 |
| SYJ105 | iltp: intuitionistic ljt j1 | 3 | 30 | 0 | 0 | 30 | 0 | 0 |
| SYJ105 | iltp: intuitionistic sat j1 | 3 | 7 | 0 | 0 | 0 | 0 | 0 |
| SYJ105 | iltp: intuitionistic auto j1 | 3 | 7 | 0 | 0 | 0 | 0 | 0 |
| SYJ105 | iltp: minimal embedding j1 | 3 | 488 | 153 | 160 | 156 | 0 | 0 |
| SYJ105 | iltp: minimal ljt j1 | 3 | 30 | 0 | 0 | 30 | 0 | 0 |
| SYJ105 | iltp: minimal sat j1 | 3 | 4 | 0 | 0 | 0 | 0 | 0 |
| SYJ105 | iltp: minimal auto j1 | 3 | 4 | 0 | 0 | 0 | 0 | 0 |
| SYJ107 | iltp: intuitionistic embedding j1 | 4 | 185936 | 50904 | 108490 | 29779 | 0 | 0 |
| SYJ107 | iltp: intuitionistic ljt j1 | 4 | 130 | 0 | 10 | 130 | 0 | 0 |
| SYJ107 | iltp: intuitionistic sat j1 | 4 | 10 | 0 | 0 | 0 | 0 | 0 |
| SYJ107 | iltp: intuitionistic auto j1 | 4 | 10 | 0 | 0 | 0 | 0 | 0 |
| SYJ107 | iltp: minimal embedding j1 | 4 | 185936 | 50904 | 108490 | 29779 | 0 | 0 |
| SYJ107 | iltp: minimal ljt j1 | 4 | 130 | 0 | 10 | 130 | 0 | 0 |
| SYJ107 | iltp: minimal sat j1 | 4 | 10 | 0 | 0 | 0 | 0 | 0 |
| SYJ107 | iltp: minimal auto j1 | 4 | 10 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp: intuitionistic embedding j1 | 5 | 4780088 | 1133939 | 3456735 | 1230227 | 0 | 0 |
| SYJ201 | iltp: intuitionistic ljt j1 | 3 | 454 | 0 | 24 | 454 | 0 | 0 |
| SYJ201 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp: minimal embedding j1 | 5 | 4780088 | 1133939 | 3456735 | 1230227 | 0 | 0 |
| SYJ201 | iltp: minimal ljt j1 | 3 | 454 | 0 | 24 | 454 | 0 | 0 |
| SYJ201 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp: intuitionistic embedding j1 | 3 | 558958 | 0 | 378783 | 74853 | 0 | 0 |
| SYJ202 | iltp: intuitionistic ljt j1 | 3 | 1965 | 0 | 369 | 1965 | 0 | 0 |
| SYJ202 | iltp: intuitionistic sat j1 | 3 | 9 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp: intuitionistic auto j1 | 3 | 9 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp: minimal embedding j1 | 3 | 558958 | 0 | 378783 | 74853 | 0 | 0 |
| SYJ202 | iltp: minimal ljt j1 | 3 | 1965 | 0 | 369 | 1965 | 0 | 0 |
| SYJ202 | iltp: minimal sat j1 | 3 | 9 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp: minimal auto j1 | 3 | 9 | 0 | 0 | 0 | 0 | 0 |
| SYJ203 | iltp: intuitionistic embedding j1 | 20 | 495273 | 129992 | 280187 | 87782 | 0 | 0 |
| SYJ203 | iltp: intuitionistic ljt j1 | 20 | 1200 | 0 | 0 | 1200 | 0 | 0 |
| SYJ203 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ203 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ203 | iltp: minimal embedding j1 | 20 | 495273 | 129992 | 280187 | 87782 | 0 | 0 |
| SYJ203 | iltp: minimal ljt j1 | 20 | 1200 | 0 | 0 | 1200 | 0 | 0 |
| SYJ203 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ203 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ204 | iltp: intuitionistic embedding j1 | 20 | 45477 | 45169 | 42502 | 566 | 0 | 0 |
| SYJ204 | iltp: intuitionistic ljt j1 | 20 | 440 | 0 | 0 | 440 | 0 | 0 |
| SYJ204 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ204 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ204 | iltp: minimal embedding j1 | 20 | 45477 | 45169 | 42502 | 566 | 0 | 0 |
| SYJ204 | iltp: minimal ljt j1 | 20 | 440 | 0 | 0 | 440 | 0 | 0 |
| SYJ204 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ204 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ205 | iltp: intuitionistic embedding j1 | 20 | 527313 | 337656 | 403089 | 90750 | 0 | 0 |
| SYJ205 | iltp: intuitionistic ljt j1 | 20 | 1250 | 0 | 20 | 1250 | 0 | 0 |
| SYJ205 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ205 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ205 | iltp: minimal embedding j1 | 20 | 527313 | 337656 | 403089 | 90750 | 0 | 0 |
| SYJ205 | iltp: minimal ljt j1 | 20 | 1250 | 0 | 20 | 1250 | 0 | 0 |
| SYJ205 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ205 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp: intuitionistic embedding j1 | 7 | 2228635 | 1290587 | 1621217 | 456167 | 0 | 0 |
| SYJ206 | iltp: intuitionistic ljt j1 | 9 | 6753 | 0 | 5 | 6753 | 0 | 0 |
| SYJ206 | iltp: intuitionistic sat j1 | 15 | 16928 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp: intuitionistic auto j1 | 14 | 8714 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp: minimal embedding j1 | 7 | 2228635 | 1290587 | 1621217 | 456167 | 0 | 0 |
| SYJ206 | iltp: minimal ljt j1 | 9 | 6753 | 0 | 5 | 6753 | 0 | 0 |
| SYJ206 | iltp: minimal sat j1 | 15 | 16920 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp: minimal auto j1 | 15 | 16920 | 0 | 0 | 0 | 0 | 0 |
| SYJ207 | iltp: intuitionistic embedding j1 | 3 | 3679703 | 2054523 | 3103416 | 10892 | 0 | 0 |
| SYJ207 | iltp: intuitionistic ljt j1 | 5 | 186083 | 0 | 35158 | 186083 | 0 | 0 |
| SYJ207 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ207 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ207 | iltp: minimal embedding j1 | 3 | 3679703 | 2054523 | 3103416 | 10892 | 0 | 0 |
| SYJ207 | iltp: minimal ljt j1 | 5 | 186083 | 0 | 35158 | 186083 | 0 | 0 |
| SYJ207 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ207 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ208 | iltp: intuitionistic embedding j1 | 3 | 2321659 | 235035 | 1920446 | 82907 | 0 | 0 |
| SYJ208 | iltp: intuitionistic ljt j1 | 6 | 170694 | 0 | 43507 | 170694 | 0 | 0 |
| SYJ208 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ208 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ208 | iltp: minimal embedding j1 | 3 | 16104 | 3337 | 28543 | 1471 | 0 | 0 |
| SYJ208 | iltp: minimal ljt j1 | 6 | 161951 | 0 | 36845 | 161951 | 0 | 0 |
| SYJ208 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ208 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ209 | iltp: intuitionistic embedding j1 | 20 | 985920 | 407499 | 653489 | 48669 | 0 | 0 |
| SYJ209 | iltp: intuitionistic ljt j1 | 20 | 4380 | 0 | 0 | 4380 | 0 | 0 |
| SYJ209 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ209 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ209 | iltp: minimal embedding j1 | 20 | 361608 | 157724 | 226478 | 29830 | 0 | 0 |
| SYJ209 | iltp: minimal ljt j1 | 20 | 3749 | 0 | 0 | 3749 | 0 | 0 |
| SYJ209 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ209 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ210 | iltp: intuitionistic embedding j1 | 20 | 232421 | 230538 | 217861 | 1328 | 0 | 0 |
| SYJ210 | iltp: intuitionistic ljt j1 | 20 | 500 | 0 | 0 | 500 | 0 | 0 |
| SYJ210 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ210 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ210 | iltp: minimal embedding j1 | 20 | 152458 | 152007 | 141130 | 1089 | 0 | 0 |
| SYJ210 | iltp: minimal ljt j1 | 20 | 40 | 0 | 0 | 40 | 0 | 0 |
| SYJ210 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ210 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ211 | iltp: intuitionistic embedding j1 | 20 | 325839 | 219756 | 269952 | 14881 | 0 | 0 |
| SYJ211 | iltp: intuitionistic ljt j1 | 20 | 143315 | 0 | 14840 | 143315 | 0 | 0 |
| SYJ211 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ211 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ211 | iltp: minimal embedding j1 | 20 | 89569 | 57160 | 69366 | 9007 | 0 | 0 |
| SYJ211 | iltp: minimal ljt j1 | 20 | 186071 | 0 | 9216 | 186071 | 0 | 0 |
| SYJ211 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ211 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ212 | iltp: intuitionistic embedding j1 | 14 | 374018 | 116574 | 277939 | 89841 | 0 | 0 |
| SYJ212 | iltp: intuitionistic ljt j1 | 20 | 2330 | 0 | 0 | 2330 | 0 | 0 |
| SYJ212 | iltp: intuitionistic sat j1 | 20 | 1526 | 0 | 0 | 0 | 0 | 0 |
| SYJ212 | iltp: intuitionistic auto j1 | 20 | 1526 | 0 | 0 | 0 | 0 | 0 |
| SYJ212 | iltp: minimal embedding j1 | 14 | 373978 | 116572 | 278199 | 89603 | 0 | 0 |
| SYJ212 | iltp: minimal ljt j1 | 20 | 1372 | 0 | 0 | 1372 | 0 | 0 |
| SYJ212 | iltp: minimal sat j1 | 20 | 18 | 0 | 0 | 0 | 0 | 0 |
| SYJ212 | iltp: minimal auto j1 | 20 | 18 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYN | iltp: intuitionistic embedding j1 | 19 | 1705 | 401 | 695 | 586 | 0 | 0 |
| Problems/SYN | iltp: intuitionistic ljt j1 | 19 | 195 | 0 | 10 | 195 | 0 | 0 |
| Problems/SYN | iltp: intuitionistic sat j1 | 19 | 17 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYN | iltp: intuitionistic auto j1 | 19 | 17 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYN | iltp: minimal embedding j1 | 19 | 1565 | 408 | 580 | 558 | 0 | 0 |
| Problems/SYN | iltp: minimal ljt j1 | 19 | 189 | 0 | 10 | 189 | 0 | 0 |
| Problems/SYN | iltp: minimal sat j1 | 19 | 14 | 0 | 0 | 0 | 0 | 0 |
| Problems/SYN | iltp: minimal auto j1 | 19 | 14 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp: intuitionistic embedding j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp: intuitionistic ljt j1 | 1 | 402 | 0 | 11 | 402 | 0 | 0 |
| SYN007 | iltp: intuitionistic sat j1 | 1 | 7 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp: intuitionistic auto j1 | 1 | 7 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp: minimal embedding j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp: minimal ljt j1 | 1 | 402 | 0 | 11 | 402 | 0 | 0 |
| SYN007 | iltp: minimal sat j1 | 1 | 9 | 0 | 0 | 0 | 0 | 0 |
| SYN007 | iltp: minimal auto j1 | 1 | 9 | 0 | 0 | 0 | 0 | 0 |
| counter | intuitionistic: intuitionistic auto j1 | 6 | 120 | 0 | 0 | 120 | 0 | 0 |
| counter-over | intuitionistic: intuitionistic auto j1 | 6 | 171 | 0 | 23 | 148 | 0 | 0 |
| chain | intuitionistic: intuitionistic auto j1 | 4 | 21155 | 1161275 | 10465 | 927 | 0 | 0 |
| 3-partition-yes | intuitionistic: intuitionistic auto j1 | 4 | 28 | 140 | 0 | 28 | 0 | 0 |
| 3-partition-no | intuitionistic: intuitionistic auto j1 | 1 | 923 | 2794 | 713 | 210 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-all-cores: intuitionistic auto j16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores: intuitionistic auto j16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores: intuitionistic auto j16 | 2 | 13451238 | 11663063 | 7995795 | 1281563 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-all-cores: intuitionistic auto j16 | 991 | 23708490 | 0 | 16669051 | 7039439 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-all-cores-generous: intuitionistic auto j16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores-generous: intuitionistic auto j16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores-generous: intuitionistic auto j16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-classical: classical auto j1 | 3071 | 36664493 | 0 | 25351462 | 11313031 | 0 | 0 |
| ILL/misc | lltp-classical: classical auto j1 | 3 | 46 | 74 | 16 | 22 | 0 | 0 |
| ILL/Non-theorems | lltp-classical: classical auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/KLE-cbv | lltp-classical: classical auto j1 | 75 | 2896 | 1748 | 848 | 1200 | 0 | 0 |
| ILL/KLE-cbn | lltp-classical: classical auto j1 | 76 | 2515 | 1122 | 806 | 1036 | 0 | 0 |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-classical: classical auto j1 | 22 | 36 | 34 | 0 | 36 | 0 | 0 |
| ILL/KLE-IMP-CONJ | lltp-classical: classical auto j1 | 162 | 3736 | 1839 | 876 | 1677 | 0 | 0 |
| ILL/KLE-IMP-CONJ/ALT | lltp-classical: classical auto j1 | 27 | 128 | 186 | 0 | 103 | 0 | 0 |
| ILL/KLE-01 | lltp-classical: classical auto j1 | 58 | 3194 | 756 | 965 | 1277 | 0 | 0 |
| ILL/ILLTP-SYN-cbv | lltp-classical: classical auto j1 | 18 | 380 | 251 | 92 | 170 | 0 | 0 |
| ILL/ILLTP-SYN-cbn | lltp-classical: classical auto j1 | 18 | 436 | 104 | 121 | 209 | 0 | 0 |
| ILL/ILLTP-SYN-01 | lltp-classical: classical auto j1 | 14 | 326 | 59 | 112 | 103 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-classical: classical auto j1 | 43 | 13475998 | 11682066 | 8006996 | 1295014 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-classical: classical auto j1 | 38 | 1311 | 968 | 630 | 450 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-classical: classical auto j1 | 34 | 13726 | 12007 | 11496 | 739 | 0 | 0 |
| ILL/ILLTP-LCL-cbv | lltp-classical: classical auto j1 | 1 | 60 | 37 | 22 | 14 | 0 | 0 |
| ILL/ILLTP-LCL-cbn | lltp-classical: classical auto j1 | 2 | 72 | 33 | 24 | 21 | 0 | 0 |
| ILL/ILLTP-LCL-01 | lltp-classical: classical auto j1 | 1 | 66 | 33 | 22 | 17 | 0 | 0 |
| CLL/misc | lltp-classical: classical auto j1 | 8 | 58 | 68 | 7 | 43 | 0 | 0 |
| CLL/Non-theorems | lltp-classical: classical auto j1 | 3 | 104 | 223 | 45 | 59 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-classical-generous: classical auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-classical-generous: classical auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-classical-generous: classical auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-classical-generous: classical auto j1 | 25 | 76194 | 0 | 52744 | 23450 | 0 | 0 |
| ILL/ILLTP-LCL-01 | lltp-copies-10: intuitionistic auto j1 | 2 | 271 | 133 | 94 | 42 | 0 | 0 |
| ILL/ILLTP-LCL-cbn | lltp-copies-10: intuitionistic auto j1 | 2 | 72 | 33 | 24 | 21 | 0 | 0 |
| ILL/ILLTP-LCL-cbv | lltp-copies-10: intuitionistic auto j1 | 2 | 239 | 125 | 103 | 38 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-copies-10: intuitionistic auto j1 | 94 | 6801753 | 3096319 | 5174044 | 842062 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-copies-10: intuitionistic auto j1 | 124 | 27038419 | 6789155 | 17928460 | 3117512 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-copies-10: intuitionistic auto j1 | 123 | 47906887 | 44198089 | 33700893 | 8690262 | 0 | 0 |
| ILL/ILLTP-SYN-01 | lltp-copies-10: intuitionistic auto j1 | 7 | 3069 | 529 | 1692 | 685 | 0 | 0 |
| ILL/ILLTP-SYN-cbn | lltp-copies-10: intuitionistic auto j1 | 8 | 556 | 246 | 188 | 200 | 0 | 0 |
| ILL/ILLTP-SYN-cbv | lltp-copies-10: intuitionistic auto j1 | 8 | 849 | 779 | 344 | 211 | 0 | 0 |
| ILL/KLE-01 | lltp-copies-10: intuitionistic auto j1 | 37 | 272568 | 112694 | 194385 | 40849 | 0 | 0 |
| ILL/KLE-IMP-CONJ | lltp-copies-10: intuitionistic auto j1 | 60 | 22321 | 11382 | 11709 | 4906 | 0 | 0 |
| ILL/KLE-cbn | lltp-copies-10: intuitionistic auto j1 | 38 | 44242 | 31072 | 33734 | 4101 | 0 | 0 |
| ILL/KLE-cbv | lltp-copies-10: intuitionistic auto j1 | 37 | 50783 | 45451 | 35069 | 7825 | 0 | 0 |
| ILL/Non-theorems | lltp-copies-10: intuitionistic auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-copies-10: intuitionistic auto j1 | 171 | 5129049 | 0 | 3887187 | 1241862 | 0 | 0 |
| ILL/ILLTP-LCL-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 2 | 271 | 133 | 94 | 42 | 0 | 0 |
| ILL/ILLTP-LCL-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 2 | 72 | 33 | 24 | 21 | 0 | 0 |
| ILL/ILLTP-LCL-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 2 | 239 | 125 | 103 | 38 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 137 | 116837208 | 77161831 | 100410050 | 8135628 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 164 | 33353717 | 13877933 | 23808165 | 5250668 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 154 | 333023503 | 264763143 | 286503684 | 17541968 | 0 | 0 |
| ILL/ILLTP-SYN-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 27766 | 9512 | 16890 | 1189 | 0 | 0 |
| ILL/ILLTP-SYN-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 595 | 248 | 189 | 228 | 0 | 0 |
| ILL/ILLTP-SYN-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 934 | 807 | 352 | 257 | 0 | 0 |
| ILL/KLE-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 87 | 5612375 | 2685781 | 4732236 | 51239 | 0 | 0 |
| ILL/KLE-IMP-CONJ/ALT | lltp-default: intuitionistic auto j16 pool after 0.1s | 27 | 122 | 145 | 0 | 98 | 0 | 0 |
| ILL/KLE-IMP-CONJ | lltp-default: intuitionistic auto j16 pool after 0.1s | 222 | 26036 | 13152 | 12564 | 6583 | 0 | 0 |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-default: intuitionistic auto j16 pool after 0.1s | 22 | 36 | 34 | 0 | 36 | 0 | 0 |
| ILL/KLE-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 88 | 1303498 | 856447 | 1103923 | 9304 | 0 | 0 |
| ILL/KLE-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 88 | 1438020 | 1472399 | 1217326 | 10429 | 0 | 0 |
| ILL/Non-theorems | lltp-default: intuitionistic auto j16 pool after 0.1s | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/misc | lltp-default: intuitionistic auto j16 pool after 0.1s | 3 | 46 | 74 | 16 | 22 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-default: intuitionistic auto j16 pool after 0.1s | 3069 | 33140643 | 0 | 23242718 | 9897925 | 0 | 0 |
| ILL/ILLTP-LCL-01 | lltp-forward: intuitionistic auto j1 bias factors | 2 | 271 | 133 | 94 | 42 | 0 | 0 |
| ILL/ILLTP-LCL-cbn | lltp-forward: intuitionistic auto j1 bias factors | 2 | 72 | 33 | 24 | 21 | 0 | 0 |
| ILL/ILLTP-LCL-cbv | lltp-forward: intuitionistic auto j1 bias factors | 2 | 239 | 125 | 103 | 38 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-forward: intuitionistic auto j1 bias factors | 139 | 64136622 | 15199912 | 56590488 | 3733428 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-forward: intuitionistic auto j1 bias factors | 170 | 31611880 | 8535059 | 21761380 | 2930075 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-forward: intuitionistic auto j1 bias factors | 162 | 81453132 | 121632038 | 66637796 | 9219748 | 0 | 0 |
| ILL/ILLTP-SYN-01 | lltp-forward: intuitionistic auto j1 bias factors | 19 | 27766 | 9512 | 16890 | 1189 | 0 | 0 |
| ILL/ILLTP-SYN-cbn | lltp-forward: intuitionistic auto j1 bias factors | 19 | 595 | 248 | 189 | 228 | 0 | 0 |
| ILL/ILLTP-SYN-cbv | lltp-forward: intuitionistic auto j1 bias factors | 19 | 934 | 807 | 352 | 257 | 0 | 0 |
| ILL/KLE-01 | lltp-forward: intuitionistic auto j1 bias factors | 87 | 8405547 | 3610815 | 7263138 | 50822 | 0 | 0 |
| ILL/KLE-IMP-CONJ/ALT | lltp-forward: intuitionistic auto j1 bias factors | 27 | 122 | 145 | 0 | 98 | 0 | 0 |
| ILL/KLE-IMP-CONJ | lltp-forward: intuitionistic auto j1 bias factors | 222 | 26036 | 13152 | 12564 | 6583 | 0 | 0 |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-forward: intuitionistic auto j1 bias factors | 22 | 36 | 34 | 0 | 36 | 0 | 0 |
| ILL/KLE-cbn | lltp-forward: intuitionistic auto j1 bias factors | 88 | 301782 | 220722 | 239422 | 3368 | 0 | 0 |
| ILL/KLE-cbv | lltp-forward: intuitionistic auto j1 bias factors | 87 | 656195 | 743358 | 546957 | 8534 | 0 | 0 |
| ILL/Non-theorems | lltp-forward: intuitionistic auto j1 bias factors | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/misc | lltp-forward: intuitionistic auto j1 bias factors | 3 | 46 | 74 | 16 | 22 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-forward: intuitionistic auto j1 bias factors | 3071 | 36664493 | 0 | 25351462 | 11313031 | 0 | 0 |
| ILL/ILLTP-LCL-01 | lltp-intuitionistic: intuitionistic auto j1 | 1 | 66 | 33 | 22 | 17 | 0 | 0 |
| ILL/ILLTP-LCL-cbn | lltp-intuitionistic: intuitionistic auto j1 | 2 | 72 | 33 | 24 | 21 | 0 | 0 |
| ILL/ILLTP-LCL-cbv | lltp-intuitionistic: intuitionistic auto j1 | 1 | 60 | 37 | 22 | 14 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-intuitionistic: intuitionistic auto j1 | 34 | 13726 | 12007 | 11496 | 739 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-intuitionistic: intuitionistic auto j1 | 38 | 1311 | 968 | 630 | 450 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-intuitionistic: intuitionistic auto j1 | 43 | 13475910 | 11681716 | 8006923 | 1295014 | 0 | 0 |
| ILL/ILLTP-SYN-01 | lltp-intuitionistic: intuitionistic auto j1 | 14 | 326 | 59 | 112 | 103 | 0 | 0 |
| ILL/ILLTP-SYN-cbn | lltp-intuitionistic: intuitionistic auto j1 | 18 | 436 | 104 | 121 | 209 | 0 | 0 |
| ILL/ILLTP-SYN-cbv | lltp-intuitionistic: intuitionistic auto j1 | 18 | 376 | 192 | 88 | 170 | 0 | 0 |
| ILL/KLE-01 | lltp-intuitionistic: intuitionistic auto j1 | 58 | 3194 | 756 | 965 | 1277 | 0 | 0 |
| ILL/KLE-IMP-CONJ/ALT | lltp-intuitionistic: intuitionistic auto j1 | 27 | 122 | 145 | 0 | 98 | 0 | 0 |
| ILL/KLE-IMP-CONJ | lltp-intuitionistic: intuitionistic auto j1 | 162 | 3715 | 1770 | 855 | 1677 | 0 | 0 |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-intuitionistic: intuitionistic auto j1 | 22 | 36 | 34 | 0 | 36 | 0 | 0 |
| ILL/KLE-cbn | lltp-intuitionistic: intuitionistic auto j1 | 76 | 2515 | 1122 | 806 | 1036 | 0 | 0 |
| ILL/KLE-cbv | lltp-intuitionistic: intuitionistic auto j1 | 75 | 2853 | 1498 | 805 | 1200 | 0 | 0 |
| ILL/Non-theorems | lltp-intuitionistic: intuitionistic auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/misc | lltp-intuitionistic: intuitionistic auto j1 | 3 | 46 | 74 | 16 | 22 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-intuitionistic: intuitionistic auto j1 | 3071 | 36664493 | 0 | 25351462 | 11313031 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-intuitionistic-generous: intuitionistic auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-intuitionistic-generous: intuitionistic auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-intuitionistic-generous: intuitionistic auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-intuitionistic-generous: intuitionistic auto j1 | 19 | 76039 | 0 | 52719 | 23320 | 0 | 0 |
| ILL/ILLTP-LCL-01 | lltp-rarer: intuitionistic auto j1 bias rarer | 1 | 66 | 33 | 22 | 17 | 0 | 0 |
| ILL/ILLTP-LCL-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | 2 | 77 | 35 | 26 | 22 | 0 | 0 |
| ILL/ILLTP-LCL-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | 1 | 60 | 37 | 22 | 14 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-rarer: intuitionistic auto j1 bias rarer | 34 | 13726 | 12007 | 11496 | 739 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | 37 | 1392 | 989 | 658 | 507 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | 43 | 13475910 | 11681716 | 8006923 | 1295014 | 0 | 0 |
| ILL/ILLTP-SYN-01 | lltp-rarer: intuitionistic auto j1 bias rarer | 14 | 326 | 59 | 112 | 103 | 0 | 0 |
| ILL/ILLTP-SYN-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | 18 | 439 | 104 | 121 | 212 | 0 | 0 |
| ILL/ILLTP-SYN-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | 18 | 376 | 192 | 88 | 170 | 0 | 0 |
| ILL/KLE-01 | lltp-rarer: intuitionistic auto j1 bias rarer | 58 | 3194 | 756 | 965 | 1277 | 0 | 0 |
| ILL/KLE-IMP-CONJ/ALT | lltp-rarer: intuitionistic auto j1 bias rarer | 27 | 148 | 205 | 0 | 124 | 0 | 0 |
| ILL/KLE-IMP-CONJ | lltp-rarer: intuitionistic auto j1 bias rarer | 162 | 3988 | 1919 | 929 | 1870 | 0 | 0 |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-rarer: intuitionistic auto j1 bias rarer | 22 | 40 | 53 | 0 | 40 | 0 | 0 |
| ILL/KLE-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | 76 | 2553 | 1006 | 791 | 1128 | 0 | 0 |
| ILL/KLE-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | 75 | 2853 | 1498 | 805 | 1200 | 0 | 0 |
| ILL/Non-theorems | lltp-rarer: intuitionistic auto j1 bias rarer | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/misc | lltp-rarer: intuitionistic auto j1 bias rarer | 3 | 79 | 81 | 29 | 45 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-rarer: intuitionistic auto j1 bias rarer | 3071 | 36664493 | 0 | 25351462 | 11313031 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-recursion: intuitionistic auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-recursion: intuitionistic auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-recursion: intuitionistic auto j1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-recursion: intuitionistic auto j1 | 895 | 1959911 | 0 | 1041264 | 918647 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-recursion-generous: intuitionistic auto j1 | 9 | 3856 | 0 | 1169 | 2687 | 0 | 0 |
| 3-partition-no | long-1: classical auto j1 | 1 | 971 | 3294 | 733 | 238 | 0 | 0 |
| partition-no | long-1: classical auto j1 | 1 | 811 | 4161 | 568 | 243 | 0 | 0 |
| partition-yes | long-1: classical auto j1 | 1 | 178 | 1097 | 97 | 81 | 0 | 0 |
| counter | long-1: classical auto j1 | 2 | 78 | 0 | 0 | 78 | 0 | 0 |
| mix | long-2: mix auto j1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| wide-m3 | long-2: classical auto j1 | 1 | 37 | 970 | 0 | 37 | 0 | 0 |
| wide-m4 | long-2: classical auto j1 | 1 | 37 | 916 | 0 | 37 | 0 | 0 |
| qbf | long-2: classical auto j1 | 1 | 172882 | 511289 | 10646 | 162236 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j1 | 4 | 28 | 140 | 0 | 28 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j2 | 4 | 52 | 296 | 0 | 52 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j4 | 4 | 100 | 558 | 2 | 64 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j8 | 4 | 238 | 1330 | 28 | 159 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j16 | 4 | 298 | 1728 | 40 | 176 | 0 | 0 |
| 3-partition-no | parallel: classical auto j1 | 2 | 1894 | 6088 | 1446 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j2 | 2 | 1923 | 6166 | 1470 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j4 | 2 | 2246 | 7243 | 1707 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j8 | 2 | 2507 | 8039 | 1922 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j16 | 2 | 2690 | 8615 | 2066 | 448 | 0 | 0 |
| partition-yes | parallel: classical auto j1 | 5 | 38546799 | 387076933 | 33786817 | 1098638 | 0 | 0 |
| partition-yes | parallel: classical auto j2 | 5 | 76860945 | 770146121 | 67383302 | 1098642 | 0 | 0 |
| partition-yes | parallel: classical auto j4 | 5 | 152291674 | 1520793728 | 133553169 | 1098726 | 0 | 0 |
| partition-yes | parallel: classical auto j8 | 5 | 274841202 | 2582889097 | 242324250 | 1098922 | 0 | 0 |
| partition-yes | parallel: classical auto j16 | 5 | 448618112 | 4025478000 | 397070410 | 1099144 | 0 | 0 |
| partition-no | parallel: classical auto j1 | 3 | 4252557 | 27627908 | 3720792 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j2 | 3 | 4633945 | 30212971 | 4046781 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j4 | 3 | 5202097 | 33799972 | 4537465 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j8 | 3 | 5348083 | 34002561 | 4681556 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j16 | 3 | 5806259 | 36038699 | 5095717 | 531765 | 0 | 0 |
| qbf | parallel: classical auto j1 | 19 | 258912034 | 1596527780 | 28181763 | 7813436 | 0 | 0 |
| qbf | parallel: classical auto j2 | 14 | 27116864 | 148798265 | 2702175 | 2633036 | 0 | 0 |
| qbf | parallel: classical auto j4 | 14 | 27115048 | 148795486 | 2702177 | 2631220 | 0 | 0 |
| qbf | parallel: classical auto j8 | 14 | 27106832 | 148782956 | 2702175 | 2623004 | 0 | 0 |
| qbf | parallel: classical auto j16 | 14 | 27117509 | 148799235 | 2702176 | 2633681 | 0 | 0 |
| mix | parallel: mix auto j1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 |
| mix | parallel: mix auto j2 | 3 | 19231650 | 4434558676 | 17257885 | 1373331 | 0 | 0 |
| mix | parallel: mix auto j4 | 3 | 19231650 | 4434558676 | 17257885 | 1373331 | 0 | 0 |
| mix | parallel: mix auto j8 | 3 | 19231650 | 4434558676 | 17257885 | 1373331 | 0 | 0 |
| mix | parallel: mix auto j16 | 3 | 19231650 | 4434558676 | 17257885 | 1373331 | 0 | 0 |
| counter | parallel: classical auto j1 | 6 | 120 | 0 | 0 | 120 | 0 | 0 |
| counter | parallel: classical auto j2 | 6 | 120 | 0 | 0 | 120 | 0 | 0 |
| counter | parallel: classical auto j4 | 6 | 120 | 0 | 0 | 120 | 0 | 0 |
| counter | parallel: classical auto j8 | 6 | 120 | 0 | 0 | 120 | 0 | 0 |
| counter | parallel: classical auto j16 | 6 | 120 | 0 | 0 | 120 | 0 | 0 |
| counter-over | parallel: classical auto j1 | 6 | 171 | 0 | 23 | 148 | 0 | 0 |
| counter-over | parallel: classical auto j2 | 6 | 171 | 0 | 23 | 148 | 0 | 0 |
| counter-over | parallel: classical auto j4 | 6 | 171 | 0 | 23 | 148 | 0 | 0 |
| counter-over | parallel: classical auto j8 | 6 | 171 | 0 | 23 | 148 | 0 | 0 |
| counter-over | parallel: classical auto j16 | 6 | 171 | 0 | 23 | 148 | 0 | 0 |
| wide-m3 | parallel: classical auto j1 | 3 | 93 | 2112 | 0 | 93 | 0 | 0 |
| wide-m3 | parallel: classical auto j2 | 3 | 93 | 2109 | 0 | 93 | 0 | 0 |
| wide-m3 | parallel: classical auto j4 | 3 | 93 | 2109 | 0 | 93 | 0 | 0 |
| wide-m3 | parallel: classical auto j8 | 3 | 93 | 2115 | 0 | 93 | 0 | 0 |
| wide-m3 | parallel: classical auto j16 | 3 | 93 | 2124 | 0 | 93 | 0 | 0 |
| wide-m4 | parallel: classical auto j1 | 3 | 99 | 2222 | 0 | 99 | 0 | 0 |
| wide-m4 | parallel: classical auto j2 | 3 | 99 | 2219 | 0 | 99 | 0 | 0 |
| wide-m4 | parallel: classical auto j4 | 3 | 99 | 2219 | 0 | 99 | 0 | 0 |
| wide-m4 | parallel: classical auto j8 | 3 | 99 | 2219 | 0 | 99 | 0 | 0 |
| wide-m4 | parallel: classical auto j16 | 3 | 99 | 2228 | 0 | 99 | 0 | 0 |
| 3-partition-mll-no | parallel-net: classical net j1 | 3 | 665 | 0 | 0 | 0 | 1247 | 1247 |
| 3-partition-mll-no | parallel-net: classical net j2 | 3 | 665 | 0 | 0 | 0 | 4132 | 1247 |
| 3-partition-mll-no | parallel-net: classical net j4 | 3 | 665 | 0 | 0 | 0 | 6882 | 1247 |
| 3-partition-mll-no | parallel-net: classical net j8 | 3 | 665 | 0 | 0 | 0 | 6882 | 1247 |
| 3-partition-mll-no | parallel-net: classical net j16 | 3 | 665 | 0 | 0 | 0 | 6882 | 1247 |
| partition-table | parallel-net: classical net j1 | 13 | 2080512 | 0 | 0 | 0 | 2203635 | 804647 |
| partition-table | parallel-net: classical net j2 | 13 | 1285865 | 0 | 0 | 0 | 1392591 | 455113 |
| partition-table | parallel-net: classical net j4 | 13 | 1956208 | 0 | 0 | 0 | 2152740 | 652619 |
| partition-table | parallel-net: classical net j8 | 13 | 76402 | 0 | 0 | 0 | 148177 | 97567 |
| partition-table | parallel-net: classical net j16 | 13 | 28777 | 0 | 0 | 0 | 111748 | 44777 |
| wide-m1 | period-16: classical net j1 period 16 | 2 | 4608 | 0 | 0 | 0 | 4608 | 288 |
| wide-m2 | period-16: classical net j1 period 16 | 2 | 4608 | 0 | 0 | 0 | 4608 | 288 |
| 3-partition-mll-no | period-16: classical net j1 period 16 | 2 | 6609 | 0 | 0 | 0 | 6851 | 244 |
| partition-yes | period-16: classical net j1 period 16 | 1 | 6253 | 0 | 0 | 0 | 6656 | 474 |
| partition-no | period-16: classical net j1 period 16 | 2 | 46371 | 0 | 0 | 0 | 50650 | 4700 |
| wide-m1 | period-1: classical net j1 period 1 | 2 | 4608 | 0 | 0 | 0 | 4608 | 4608 |
| wide-m2 | period-1: classical net j1 period 1 | 2 | 4608 | 0 | 0 | 0 | 4608 | 4608 |
| 3-partition-mll-no | period-1: classical net j1 period 1 | 2 | 313 | 0 | 0 | 0 | 582 | 582 |
| partition-yes | period-1: classical net j1 period 1 | 1 | 485 | 0 | 0 | 0 | 711 | 711 |
| partition-no | period-1: classical net j1 period 1 | 2 | 2486 | 0 | 0 | 0 | 3890 | 3890 |
| wide-m1 | period-2: classical net j1 period 2 | 2 | 4608 | 0 | 0 | 0 | 4608 | 2304 |
| wide-m2 | period-2: classical net j1 period 2 | 2 | 4608 | 0 | 0 | 0 | 4608 | 2304 |
| 3-partition-mll-no | period-2: classical net j1 period 2 | 2 | 449 | 0 | 0 | 0 | 808 | 513 |
| partition-yes | period-2: classical net j1 period 2 | 1 | 513 | 0 | 0 | 0 | 765 | 511 |
| partition-no | period-2: classical net j1 period 2 | 2 | 2933 | 0 | 0 | 0 | 4462 | 2806 |
| wide-m1 | period-8: classical net j1 period 8 | 2 | 4608 | 0 | 0 | 0 | 4608 | 576 |
| wide-m2 | period-8: classical net j1 period 8 | 2 | 4608 | 0 | 0 | 0 | 4608 | 576 |
| 3-partition-mll-no | period-8: classical net j1 period 8 | 2 | 2689 | 0 | 0 | 0 | 2926 | 284 |
| partition-yes | period-8: classical net j1 period 8 | 1 | 5231 | 0 | 0 | 0 | 6307 | 1152 |
| partition-no | period-8: classical net j1 period 8 | 2 | 13140 | 0 | 0 | 0 | 18556 | 5885 |

## Verdicts that disagree

None: every problem decided in a mode has one verdict.

## partition-yes

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| partition-yes/4 | 72 µs ✓ | 1.8 ms ✓ | 72 µs ✓ |  |  |  |  |  |  | 2.6 ms ✓ | 1.7 ms ✓ | 1.3 ms ✓ | 5.2 ms ✓ |
| partition-yes/5 | 91 µs ✓ | 28.8 ms ✓ | 87 µs ✓ |  | 130 µs ✓ | 308 µs ✓ | 301 µs ✓ | 480 µs ✓ | 623 µs ✓ |  |  |  |  |
| partition-yes/6 | 150 µs ✓ | 472.5 ms ✓ | 142 µs ✓ |  | 178 µs ✓ | 437 µs ✓ | 369 µs ✓ | 563 µs ✓ | 670 µs ✓ |  |  |  |  |
| partition-yes/12 | 12.8 ms ✓ | > 60 s | 12.6 ms ✓ |  |  |  |  |  |  |  |  |  |  |
| partition-yes/20 | 420.8 ms ✓ | > 60 s | 420.6 ms ✓ |  | 385.1 ms ✓ | 407.3 ms ✓ | 423.3 ms ✓ | 421.0 ms ✓ | 439.8 ms ✓ |  |  |  |  |
| partition-yes/7 |  |  | 205 µs ✓ | 211 µs ✓ | 267 µs ✓ | 532 µs ✓ | 612 µs ✓ | 533 µs ✓ | 878 µs ✓ |  |  |  |  |
| partition-yes/16 |  |  | 136.0 ms ✓ |  |  |  |  |  |  |  |  |  |  |
| partition-yes/24 |  |  | 71.53 s ✓ |  | 63.86 s ✓ | 68.35 s ✓ | 72.87 s ✓ | 76.63 s ✓ | 83.65 s ✓ |  |  |  |  |
| partition-yes/28 |  |  | > 300 s | > 1200 s |  |  |  |  |  |  |  |  |  |

## partition-no

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| partition-no/3 | 83 µs ✗ | 1.4 ms ✗ | 78 µs ✗ |  |  |  |  |  |  | 2.3 ms ✗ | 1.5 ms ✗ | 1.2 ms ✗ | 1.5 ms ✗ |
| partition-no/4 | 148 µs ✗ | 9.4 ms ✗ | 149 µs ✗ |  | 218 µs ✗ | 417 µs ✗ | 332 µs ✗ | 465 µs ✗ | 637 µs ✗ | 23.9 ms ✗ | 9.4 ms ✗ | 7.7 ms ✗ | 22.7 ms ✗ |
| partition-no/9 | 80.5 ms ✗ | > 60 s | 78.7 ms ✗ |  |  |  |  |  |  |  |  |  |  |
| partition-no/12 | 3.56 s ✗ | > 60 s | 3.53 s ✗ |  | 3.53 s ✗ | 2.04 s ✗ | 1.20 s ✗ | 749.2 ms ✗ | 557.4 ms ✗ |  |  |  |  |
| partition-no/5 |  |  | 438 µs ✗ | 448 µs ✗ | 584 µs ✗ | 775 µs ✗ | 591 µs ✗ | 763 µs ✗ | 803 µs ✗ |  |  |  |  |
| partition-no/14 |  |  | > 300 s |  | > 120 s | > 120 s | > 120 s | > 120 s | > 120 s |  |  |  |  |
| partition-no/15 |  |  | > 300 s | > 1200 s |  |  |  |  |  |  |  |  |  |

## 3-partition-mll-yes

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 |
|---|--:|--:|--:|
| 3-partition-mll-yes/4 | 54 µs ✓ | 78 µs ✓ | 75 µs ✓ |
| 3-partition-mll-yes/6 | 54 µs ✓ | 105 µs ✓ | 72 µs ✓ |
| 3-partition-mll-yes/8 | 59 µs ✓ | 135 µs ✓ | 66 µs ✓ |
| 3-partition-mll-yes/12 | 54 µs ✓ | 221 µs ✓ | 66 µs ✓ |

## 3-partition-mll-no

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | parallel-net: classical net j1 | parallel-net: classical net j2 | parallel-net: classical net j4 | parallel-net: classical net j8 | parallel-net: classical net j16 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-mll-no/4 | 44 µs ✗ | 263 µs ✗ | 52 µs ✗ | 359 µs ✗ | 494 µs ✗ | 551 µs ✗ | 623 µs ✗ | 802 µs ✗ | 267 µs ✗ | 255 µs ✗ | 242 µs ✗ | 244 µs ✗ |
| 3-partition-mll-no/5 | 42 µs ✗ | 694 µs ✗ | 45 µs ✗ | 971 µs ✗ | 1.2 ms ✗ | 1.1 ms ✗ | 1.2 ms ✗ | 1.4 ms ✗ | 1.2 ms ✗ | 700 µs ✗ | 721 µs ✗ | 740 µs ✗ |
| 3-partition-mll-no/6 | 45 µs ✗ | 1.2 ms ✗ | 48 µs ✗ | 1.7 ms ✗ | 1.2 ms ✗ | 1.9 ms ✗ | 1.9 ms ✗ | 2.1 ms ✗ |  |  |  |  |
| 3-partition-mll-no/8 | 44 µs ✗ | 3.1 ms ✗ | 45 µs ✗ |  |  |  |  |  |  |  |  |  |

## wide-m1

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|
| wide-m1/8 | 55 µs ✓ | 65 µs ✓ | 56 µs ✓ |  |  |  |  |
| wide-m1/16 | 72 µs ✓ | 126 µs ✓ | 67 µs ✓ |  |  |  |  |
| wide-m1/24 | 90 µs ✓ | 225 µs ✓ | 86 µs ✓ |  |  |  |  |
| wide-m1/32 | 126 µs ✓ | 205 µs ✓ | 122 µs ✓ |  |  |  |  |
| wide-m1/256 | 3.3 ms ✓ | 8.9 ms ✓ | 3.1 ms ✓ | 6.7 ms ✓ | 18.7 ms ✓ | 12.2 ms ✓ | 7.4 ms ✓ |
| wide-m1/2048 | 212.7 ms ✓ | 606.4 ms ✓ | 200.7 ms ✓ | 441.4 ms ✓ | 1.27 s ✓ | 832.3 ms ✓ | 496.2 ms ✓ |

## wide-m2

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|
| wide-m2/8 | 56 µs ✓ | 62 µs ✓ | 53 µs ✓ |  |  |  |  |
| wide-m2/16 | 66 µs ✓ | 127 µs ✓ | 68 µs ✓ |  |  |  |  |
| wide-m2/24 | 87 µs ✓ | 223 µs ✓ | 86 µs ✓ |  |  |  |  |
| wide-m2/32 | 112 µs ✓ | 200 µs ✓ | 109 µs ✓ |  |  |  |  |
| wide-m2/256 | 2.8 ms ✓ | 8.9 ms ✓ | 2.6 ms ✓ | 6.5 ms ✓ | 18.9 ms ✓ | 12.1 ms ✓ | 7.5 ms ✓ |
| wide-m2/2048 | 203.0 ms ✓ | 601.7 ms ✓ | 187.2 ms ✓ | 431.7 ms ✓ | 1.27 s ✓ | 825.3 ms ✓ | 493.7 ms ✓ |

## wide-m3

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| wide-m3/12 | 58 µs ✓ | 94 µs ✓ | 59 µs ✓ |  |  |  |  |  |  |
| wide-m3/24 | 83 µs ✓ | 249 µs ✓ | 83 µs ✓ |  | 143 µs ✓ | 284 µs ✓ | 340 µs ✓ | 375 µs ✓ | 538 µs ✓ |
| wide-m3/30 | 104 µs ✓ | 185 µs ✓ | 106 µs ✓ |  | 132 µs ✓ | 286 µs ✓ | 336 µs ✓ | 405 µs ✓ | 625 µs ✓ |
| wide-m3/256 | 2.5 ms ✓ | 9.4 ms ✓ | 2.4 ms ✓ |  |  |  |  |  |  |
| wide-m3/2048 | 179.5 ms ✓ | 638.3 ms ✓ | 163.9 ms ✓ |  |  |  |  |  |  |
| wide-m3/36 |  |  | 117 µs ✓ | 145 µs ✓ | 171 µs ✓ | 407 µs ✓ | 359 µs ✓ | 588 µs ✓ | 770 µs ✓ |
| wide-m3/1024 |  |  | 40.9 ms ✓ |  |  |  |  |  |  |

## wide-m4

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| wide-m4/12 | 62 µs ✓ | 97 µs ✓ | 60 µs ✓ |  |  |  |  |  |  |
| wide-m4/24 | 85 µs ✓ | 249 µs ✓ | 84 µs ✓ |  |  |  |  |  |  |
| wide-m4/28 | 98 µs ✓ | 299 µs ✓ | 96 µs ✓ |  | 137 µs ✓ | 318 µs ✓ | 358 µs ✓ | 475 µs ✓ | 565 µs ✓ |
| wide-m4/256 | 2.3 ms ✓ | 9.6 ms ✓ | 2.2 ms ✓ |  |  |  |  |  |  |
| wide-m4/2048 | 168.0 ms ✓ | 648.2 ms ✓ | 152.9 ms ✓ |  |  |  |  |  |  |
| wide-m4/32 |  |  | 104 µs ✓ |  | 162 µs ✓ | 322 µs ✓ | 360 µs ✓ | 532 µs ✓ | 722 µs ✓ |
| wide-m4/36 |  |  | 118 µs ✓ | 130 µs ✓ | 190 µs ✓ | 446 µs ✓ | 427 µs ✓ | 531 µs ✓ | 636 µs ✓ |
| wide-m4/1024 |  |  | 37.8 ms ✓ |  |  |  |  |  |  |

## additive

| problem | engines: classical focus j1 | families: classical auto j1 |
|---|--:|--:|
| additive/8 | 727 µs ✓ | 343 µs ✓ |
| additive/12 | 32.3 ms ✓ | 10.6 ms ✓ |
| additive/14 | 347.2 ms ✓ | 58.8 ms ✓ |
| additive/16 | 5.50 s ✓ | 181.7 ms ✓ |

## partition-table

| problem | engines: classical focus j1 | engines: classical net j1 | parallel-net: classical net j1 | parallel-net: classical net j2 | parallel-net: classical net j4 | parallel-net: classical net j8 | parallel-net: classical net j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| partition-table/1-1 | 54 µs ✓ | 57 µs ✓ | 85 µs ✓ | 194 µs ✓ | 240 µs ✓ | 307 µs ✓ | 486 µs ✓ |
| partition-table/1-3 | 53 µs ✗ | 122 µs ✗ | 163 µs ✗ | 286 µs ✗ | 314 µs ✗ | 383 µs ✗ | 572 µs ✗ |
| partition-table/2-1-1 | 66 µs ✓ | 192 µs ✓ | 261 µs ✓ | 427 µs ✓ | 529 µs ✓ | 580 µs ✓ | 788 µs ✓ |
| partition-table/1-1-4 | 76 µs ✗ | 541 µs ✗ | 745 µs ✗ | 851 µs ✗ | 921 µs ✗ | 960 µs ✗ | 1.1 ms ✗ |
| partition-table/2-2-1-1 | 74 µs ✓ | 940 µs ✓ | 1.3 ms ✓ | 1.0 ms ✓ | 1.2 ms ✓ | 2.2 ms ✓ | 2.5 ms ✓ |
| partition-table/1-2-5 | 78 µs ✗ | 1.3 ms ✗ | 1.8 ms ✗ | 1.2 ms ✗ | 1.9 ms ✗ | 2.0 ms ✗ | 2.1 ms ✗ |
| partition-table/1-1-2-4 | 63 µs ✓ | 890 µs ✓ | 1.2 ms ✓ | 1.2 ms ✓ | 813 µs ✓ | 2.4 ms ✓ | 5.7 ms ✓ |
| partition-table/1-1-1-5 | 139 µs ✗ | 3.2 ms ✗ | 4.2 ms ✗ | 2.6 ms ✗ | 1.7 ms ✗ | 2.8 ms ✗ | 4.4 ms ✗ |
| partition-table/2-3-2-1 | 78 µs ✓ | 2.5 ms ✓ | 3.4 ms ✓ | 2.0 ms ✓ | 1.4 ms ✓ | 2.3 ms ✓ | 5.3 ms ✓ |
| partition-table/3-3-3-1 | 144 µs ✗ | 8.2 ms ✗ | 10.2 ms ✗ | 5.8 ms ✗ | 3.5 ms ✗ | 4.7 ms ✗ | 4.5 ms ✗ |
| partition-table/1-2-3-4-5-5 | 87 µs ✓ | 1.10 s ✓ | 1.02 s ✓ | 120.2 ms ✓ | 1.5 ms ✓ | 51.4 ms ✓ | 7.0 ms ✓ |
| partition-table/1-1-1-1-1-7 | 1.1 ms ✗ | 107.3 ms ✗ | 108.2 ms ✗ | 61.5 ms ✗ | 34.2 ms ✗ | 20.7 ms ✗ | 12.2 ms ✗ |
| partition-table/2-2-2-2-2-2-9-1 | 130 µs ✓ | 4.07 s ✓ | 3.83 s ✓ | 1.35 s ✓ | 1.19 s ✓ | 1.6 ms ✓ | 2.9 ms ✓ |

## cancellation

| problem | engines: classical focus j1 |
|---|--:|
| cancellation/3-partition-4 | 304 µs ✓ |

## 3-partition-yes

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-yes/4 | 70 µs ✓ | 61 µs ✓ | 107 µs ✓ | 297 µs ✓ | 270 µs ✓ | 385 µs ✓ | 597 µs ✓ |
| 3-partition-yes/6 | 63 µs ✓ | 69 µs ✓ | 89 µs ✓ | 238 µs ✓ | 287 µs ✓ | 414 µs ✓ | 690 µs ✓ |
| 3-partition-yes/8 | 63 µs ✓ | 72 µs ✓ | 88 µs ✓ | 248 µs ✓ | 282 µs ✓ | 421 µs ✓ | 685 µs ✓ |
| 3-partition-yes/12 | 71 µs ✓ | 75 µs ✓ | 102 µs ✓ | 262 µs ✓ | 308 µs ✓ | 381 µs ✓ | 614 µs ✓ |

## 3-partition-no

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-no/4 | 293 µs ✗ | 296 µs ✗ |  | 383 µs ✗ | 540 µs ✗ | 484 µs ✗ | 681 µs ✗ | 697 µs ✗ |
| 3-partition-no/5 | 344 µs ✗ |  | 375 µs ✗ | 432 µs ✗ | 586 µs ✗ | 434 µs ✗ | 708 µs ✗ | 813 µs ✗ |

## qbf

| problem | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| qbf/8#0 | 177 µs ✗ |  |  |  |  |  |  |
| qbf/8#1 | 199 µs ✗ |  |  |  |  |  |  |
| qbf/8#2 | 147 µs ✓ |  |  |  |  |  |  |
| qbf/8#3 | 178 µs ✗ |  |  |  |  |  |  |
| qbf/12#0 | 1.1 ms ✓ |  |  |  |  |  |  |
| qbf/12#1 | 978 µs ✗ |  |  |  |  |  |  |
| qbf/12#2 | 594 µs ✓ |  |  |  |  |  |  |
| qbf/12#3 | 907 µs ✗ |  |  |  |  |  |  |
| qbf/16#0 | 5.4 ms ✓ |  | 7.4 ms ✓ | 7.7 ms ✓ | 7.8 ms ✓ | 8.9 ms ✓ | 8.5 ms ✓ |
| qbf/16#1 | 8.1 ms ✗ |  | 11.4 ms ✗ | 11.2 ms ✗ | 11.2 ms ✗ | 11.5 ms ✗ | 11.5 ms ✗ |
| qbf/16#2 | 8.5 ms ✗ |  | 12.3 ms ✗ | 12.4 ms ✗ | 12.5 ms ✗ | 12.2 ms ✗ | 14.1 ms ✗ |
| qbf/16#3 | 4.8 ms ✓ |  | 6.8 ms ✓ | 7.3 ms ✓ | 7.1 ms ✓ | 7.2 ms ✓ | 7.7 ms ✓ |
| qbf/20#0 | 82.9 ms ✗ |  | 101.2 ms ✗ | 101.5 ms ✗ | 100.4 ms ✗ | 101.1 ms ✗ | 100.2 ms ✗ |
| qbf/20#1 | 83.0 ms ✗ |  | 98.9 ms ✗ | 99.9 ms ✗ | 101.6 ms ✗ | 102.1 ms ✗ | 97.3 ms ✗ |
| qbf/20#2 | 26.7 ms ✓ |  | 34.9 ms ✓ | 53.6 ms ✓ | 46.7 ms ✓ | 52.3 ms ✓ | 53.6 ms ✓ |
| qbf/20#3 | 81.2 ms ✗ |  | 97.9 ms ✗ | 97.8 ms ✗ | 99.2 ms ✗ | 96.8 ms ✗ | 98.4 ms ✗ |
| qbf/24#0 | 178.2 ms ✗ | 174.9 ms ✗ | 193.1 ms ✗ | 206.0 ms ✗ | 202.0 ms ✗ | 203.8 ms ✗ | 209.5 ms ✗ |
| qbf/24#1 | 119.5 ms ✗ |  | 132.1 ms ✗ | 134.1 ms ✗ | 134.7 ms ✗ | 136.6 ms ✗ | 131.8 ms ✗ |
| qbf/24#2 | 111.8 ms ✗ |  | 128.5 ms ✗ | 132.7 ms ✗ | 133.6 ms ✗ | 131.7 ms ✗ | 130.4 ms ✗ |
| qbf/24#3 | 122.5 ms ✗ |  | 136.3 ms ✗ | 137.0 ms ✗ | 139.9 ms ✗ | 133.2 ms ✗ | 133.3 ms ✗ |
| qbf/32#0 | 562.1 ms ✗ |  |  |  |  |  |  |
| qbf/32#1 | 615.4 ms ✗ |  |  |  |  |  |  |
| qbf/32#2 | 1.06 s ✗ |  |  |  |  |  |  |
| qbf/32#3 | 497.3 ms ✗ |  |  |  |  |  |  |
| qbf/40#0 | 10.41 s ✗ |  | 10.26 s ✗ | 6.47 s ✗ | 6.52 s ✗ | 6.58 s ✗ | 6.41 s ✗ |
| qbf/40#1 | 11.81 s ✗ |  | 11.37 s ✗ | 5.40 s ? memory | 5.49 s ? memory | 4.83 s ? memory | 5.42 s ? memory |
| qbf/40#2 | 13.58 s ✗ |  | 13.03 s ✗ | 5.20 s ? memory | 5.16 s ? memory | 5.25 s ? memory | 5.21 s ? memory |
| qbf/40#3 | 10.68 s ✗ |  | 10.38 s ✗ | 6.76 s ✗ | 6.77 s ✗ | 6.83 s ✗ | 6.78 s ✗ |
| qbf/44#0 | 45.06 s ✗ |  | 43.63 s ✗ | 7.83 s ? memory | 7.89 s ? memory | 7.83 s ? memory | 8.04 s ? memory |
| qbf/44#1 | 45.69 s ✗ |  | 43.57 s ✗ | 6.62 s ? memory | 6.66 s ? memory | 6.73 s ? memory | 6.66 s ? memory |
| qbf/44#2 | 64.84 s ✗ |  | 67.16 s ✗ | 4.88 s ? memory | 4.81 s ? memory | 4.91 s ? memory | 4.87 s ? memory |
| qbf/44#3 | 22.94 s ? memory |  | 24.31 s ? memory | 8.05 s ? memory | 8.02 s ? memory | 8.13 s ? memory | 8.02 s ? memory |
| qbf/48#0 | 69.69 s ? memory | 76.16 s ? memory |  |  |  |  |  |
| qbf/48#1 | 166.66 s ✗ |  |  |  |  |  |  |
| qbf/48#2 | 138.70 s ✗ |  |  |  |  |  |  |
| qbf/48#3 | 182.58 s ✗ |  |  |  |  |  |  |

## mix

| problem | families: mix auto j1 | long-2: mix auto j1 | parallel: mix auto j1 | parallel: mix auto j2 | parallel: mix auto j4 | parallel: mix auto j8 | parallel: mix auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| mix/4 | 297 µs ✗ |  |  |  |  |  |  |
| mix/6 | 11.1 ms ✗ |  |  |  |  |  |  |
| mix/8 | 252.4 ms ✗ |  | 280.5 ms ✗ | 835.9 ms ✗ | 907.4 ms ✗ | 1.25 s ✗ | 1.62 s ✗ |
| mix/9 | 252.8 ms ✗ |  | 282.3 ms ✗ | 6.16 s ✗ | 6.48 s ✗ | 8.31 s ✗ | 9.57 s ✗ |
| mix/10 | 253.7 ms ✗ |  | 275.4 ms ✗ | 69.47 s ✗ | 72.09 s ✗ | 83.76 s ✗ | 107.94 s ✗ |
| mix/11 | 264.7 ms ✗ | 272.2 ms ✗ | 294.8 ms ✗ | 120.00 s ✗ | 120.00 s ✗ | 120.00 s ✗ | 120.00 s ✗ |

## counter

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| counter/2 | 31 µs ✓ | 32 µs ✓ |  | 68 µs ✓ | 104 µs ✓ | 122 µs ✓ | 117 µs ✓ | 117 µs ✓ |
| counter/4 | 31 µs ✓ | 35 µs ✓ |  | 82 µs ✓ | 123 µs ✓ | 127 µs ✓ | 114 µs ✓ | 111 µs ✓ |
| counter/8 | 38 µs ✓ | 40 µs ✓ |  | 91 µs ✓ | 124 µs ✓ | 124 µs ✓ | 113 µs ✓ | 117 µs ✓ |
| counter/16 | 43 µs ✓ | 43 µs ✓ | 55 µs ✓ | 106 µs ✓ | 110 µs ✓ | 111 µs ✓ | 130 µs ✓ | 126 µs ✓ |
| counter/32 | 46 µs ✓ | 51 µs ✓ |  | 103 µs ✓ | 145 µs ✓ | 143 µs ✓ | 135 µs ✓ | 100 µs ✓ |
| counter/64 | 55 µs ✓ | 58 µs ✓ | 57 µs ✓ | 126 µs ✓ | 150 µs ✓ | 141 µs ✓ | 143 µs ✓ | 141 µs ✓ |

## counter-over

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| counter-over/2 | 29 µs ✗ | 32 µs ✗ | 91 µs ✗ | 119 µs ✗ | 122 µs ✗ | 111 µs ✗ | 104 µs ✗ |
| counter-over/4 | 33 µs ✗ | 36 µs ✗ | 90 µs ✗ | 114 µs ✗ | 81 µs ✗ | 96 µs ✗ | 106 µs ✗ |
| counter-over/8 | 39 µs ✗ | 39 µs ✗ | 94 µs ✗ | 128 µs ✗ | 122 µs ✗ | 124 µs ✗ | 125 µs ✗ |
| counter-over/16 | 41 µs ✗ | 43 µs ✗ | 100 µs ✗ | 127 µs ✗ | 123 µs ✗ | 132 µs ✗ | 129 µs ✗ |
| counter-over/32 | 44 µs ✗ | 53 µs ✗ | 72 µs ✗ | 101 µs ✗ | 134 µs ✗ | 118 µs ✗ | 130 µs ✗ |
| counter-over/64 | 52 µs ✗ | 54 µs ✗ | 108 µs ✗ | 138 µs ✗ | 129 µs ✗ | 115 µs ✗ | 135 µs ✗ |

## growing

| problem | families: classical auto j1 |
|---|--:|
| growing/16 | 230 µs ✗ |
| growing/64 | 807 µs ✗ |
| growing/256 | 18.1 ms ✗ |
| growing/1024 | 111.1 ms ✗ |

## chain

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 |
|---|--:|--:|
| chain/16 | 171 µs ✓ | 163 µs ✓ |
| chain/64 | 4.3 ms ✓ | 4.8 ms ✓ |
| chain/128 | 33.0 ms ✓ | 38.3 ms ✓ |
| chain/256 | 272.1 ms ✓ | 260.3 ms ✓ |

## lambek-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| lambek-fo/16#0 | 120 µs ✓ |
| lambek-fo/16#1 | 89 µs ✗ |
| lambek-fo/16#2 | 253 µs ✓ |
| lambek-fo/16#3 | 97 µs ✓ |
| lambek-fo/16#4 | 88 µs ✗ |
| lambek-fo/64#0 | 458 µs ✓ |
| lambek-fo/64#1 | 553 µs ✗ |
| lambek-fo/64#2 | 6.7 ms ✓ |
| lambek-fo/64#3 | 401 µs ✓ |
| lambek-fo/64#4 | 409 µs ✗ |
| lambek-fo/256#0 | 3.9 ms ✓ |
| lambek-fo/256#1 | 9.8 ms ✗ |
| lambek-fo/256#2 | 428.1 ms ✓ |
| lambek-fo/256#3 | 4.2 ms ✓ |
| lambek-fo/256#4 | 5.3 ms ✗ |
| lambek-fo/512#0 | 13.1 ms ? depth |
| lambek-fo/512#1 | 46.3 ms ✗ |
| lambek-fo/512#2 | 3.85 s ? depth |
| lambek-fo/512#3 | 18.5 ms ✓ |
| lambek-fo/512#4 | 23.8 ms ✗ |

## chain-fo

| problem | families: classical auto j1 |
|---|--:|
| chain-fo/16#0 | 392 µs ✓ |
| chain-fo/16#1 | 348 µs ? bound |
| chain-fo/64#0 | 7.3 ms ✓ |
| chain-fo/64#1 | 6.9 ms ? bound |
| chain-fo/256#0 | 308.9 ms ✓ |
| chain-fo/256#1 | 274.6 ms ? bound |
| chain-fo/512#0 | 1.18 s ? depth |
| chain-fo/512#1 | 1.03 s ? depth |

## witness-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| witness-fo/4#0 | 62 µs ✓ |
| witness-fo/4#1 | 56 µs ✗ |
| witness-fo/4#2 | 53 µs ✓ |
| witness-fo/4#3 | 73 µs ✗ |
| witness-fo/8#0 | 63 µs ✓ |
| witness-fo/8#1 | 58 µs ✗ |
| witness-fo/8#2 | 62 µs ✓ |
| witness-fo/8#3 | 27.6 ms ✗ |
| witness-fo/10#0 | 67 µs ✓ |
| witness-fo/10#1 | 66 µs ✗ |
| witness-fo/10#2 | 68 µs ✓ |
| witness-fo/10#3 | 2.51 s ✗ |
| witness-fo/12#0 | 72 µs ✓ |
| witness-fo/12#1 | 65 µs ✗ |
| witness-fo/12#2 | 71 µs ✓ |
| witness-fo/12#3 | > 300 s |

## contraction

| problem | families: intuitionistic auto j1 |
|---|--:|
| contraction/2#0 | 55 µs ✓ |
| contraction/2#1 | 73 µs ? bound |
| contraction/4#0 | 152 µs ✓ |
| contraction/4#1 | 476 µs ? bound |
| contraction/8#0 | 64.9 ms ✓ |
| contraction/8#1 | 74.4 ms ✗ |

## pigeonhole

| problem | families: intuitionistic-affine auto j1 |
|---|--:|
| pigeonhole/2#0 | 77 µs ✓ |
| pigeonhole/2#1 | 40 µs ✗ |
| pigeonhole/3#0 | 956 µs ✓ |
| pigeonhole/3#1 | 77 µs ✗ |
| pigeonhole/4#0 | 38.7 ms ✓ |
| pigeonhole/4#1 | 279 µs ✗ |

## euler

| problem | families: intuitionistic auto j1 |
|---|--:|
| euler/5#0 | 59 µs ✓ |
| euler/5#1 | 153 µs ✗ |
| euler/6#0 | 65 µs ✓ |
| euler/6#1 | 320 µs ✗ |
| euler/8#0 | 73 µs ✓ |
| euler/8#1 | 1.6 ms ✗ |

## duplicate

| problem | families: intuitionistic auto j1 | families: classical auto j1 |
|---|--:|--:|
| duplicate/2#0 | 37 µs ✓ |  |
| duplicate/4#0 | 32 µs ✓ |  |
| duplicate/8#0 | 33 µs ✓ |  |
| duplicate/2#1 |  | 27 µs ✓ |
| duplicate/4#1 |  | 31 µs ✓ |
| duplicate/8#1 |  | 31 µs ✓ |

## reach-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| reach-fo/2#0 | 114 µs ✓ |
| reach-fo/2#1 | 136 µs ? bound |
| reach-fo/4#0 | 1.3 ms ✓ |
| reach-fo/4#1 | 1.6 ms ? bound |

## Problems/LCL

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| LCL181+1.p | 40 µs ✓ | 34 µs ✓ | 114 µs ✓ | 11 µs ✓ | 40 µs ✓ | 114 µs ✗ | 19 µs ✗ | 46 µs ✗ | 47 µs ✗ | 112 µs ✗ | 18 µs ✗ | 49 µs ✗ | 50 µs ✗ |
| LCL230+1.p | 33 µs ✓ | 28 µs ✓ | 87 µs ✓ | 11 µs ✓ | 33 µs ✓ | 65 µs ✗ | 18 µs ✗ | 48 µs ✗ | 46 µs ✗ | 61 µs ✗ | 18 µs ✗ | 48 µs ✗ | 46 µs ✗ |

## Problems/SYJ

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ101+1.p | 28 µs ✓ | 18 µs ✓ | 54 µs ✓ | 10 µs ✓ | 16 µs ✓ | 25 µs ✓ | 18 µs ✓ | 26 µs ✓ | 35 µs ✓ | 25 µs ✓ | 14 µs ✓ | 20 µs ✓ | 30 µs ✓ |
| SYJ102+1.p | 27 µs ✓ | 20 µs ✓ | 55 µs ✓ | 10 µs ✓ | 20 µs ✓ | 39 µs ✓ | 23 µs ✓ | 33 µs ✓ | 47 µs ✓ | 93 µs ✓ | 21 µs ✓ | 33 µs ✓ | 41 µs ✓ |
| SYJ103+1.p | 27 µs ✓ | 24 µs ✓ | 78 µs ✓ | 10 µs ✓ | 28 µs ✓ | 67 µs ✓ | 24 µs ✓ | 35 µs ✓ | 48 µs ✓ | 135 µs ✓ | 29 µs ✓ | 34 µs ✓ | 55 µs ✓ |
| SYJ104+1.p | 30 µs ✓ | 20 µs ✓ | 74 µs ✓ | 11 µs ✓ | 16 µs ✓ | 140 µs ✓ | 19 µs ✓ | 36 µs ✓ | 37 µs ✓ | 97 µs ✓ | 15 µs ✓ | 27 µs ✓ | 35 µs ✓ |
| SYJ106+1.p | 45 µs ✓ | 34 µs ✓ | 147 µs ✓ | 13 µs ✓ | 67 µs ✓ | 152 µs ✓ | 43 µs ✓ | 75 µs ✓ | 106 µs ✓ | 148 µs ✗ | 28 µs ✗ | 57 µs ✗ | 49 µs ✗ |

## SYJ105

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ105+1.002.p | 28 µs ✓ | 25 µs ✓ | 68 µs ✓ | 12 µs ✓ | 23 µs ✓ | 47 µs ✓ | 25 µs ✓ | 45 µs ✓ | 58 µs ✓ | 47 µs ✓ | 29 µs ✓ | 47 µs ✓ | 63 µs ✓ |
| SYJ105+1.003.p | 34 µs ✓ | 27 µs ✓ | 79 µs ✓ | 13 µs ✓ | 28 µs ✓ | 78 µs ✓ | 44 µs ✓ | 62 µs ✓ | 98 µs ✓ | 75 µs ✓ | 42 µs ✓ | 66 µs ✓ | 106 µs ✓ |
| SYJ105+1.004.p | 32 µs ✓ | 31 µs ✓ | 89 µs ✓ | 11 µs ✓ | 37 µs ✓ | 151 µs ✓ | 62 µs ✓ | 108 µs ✓ | 162 µs ✓ | 142 µs ✓ | 63 µs ✓ | 107 µs ✓ | 154 µs ✓ |

## SYJ107

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ107+1.001.p | 35 µs ✓ | 25 µs ✓ | 102 µs ✓ | 13 µs ✓ | 40 µs ✓ | 70 µs ✓ | 32 µs ✓ | 41 µs ✓ | 60 µs ✓ | 73 µs ✓ | 28 µs ✓ | 42 µs ✓ | 64 µs ✓ |
| SYJ107+1.002.p | 64 µs ✓ | 36 µs ✓ | 271 µs ✓ | 17 µs ✓ | 97 µs ✓ | 297 µs ✓ | 44 µs ✓ | 57 µs ✓ | 96 µs ✓ | 297 µs ✓ | 41 µs ✓ | 62 µs ✓ | 99 µs ✓ |
| SYJ107+1.003.p | 192 µs ✓ | 41 µs ✓ | 1.6 ms ✓ | 20 µs ✓ | 343 µs ✓ | 2.2 ms ✓ | 77 µs ✓ | 95 µs ✓ | 167 µs ✓ | 2.3 ms ✓ | 81 µs ✓ | 99 µs ✓ | 171 µs ✓ |
| SYJ107+1.004.p | 1.2 ms ✓ | 48 µs ✓ | 15.1 ms ✓ | 57 µs ✓ | 1.5 ms ✓ | 22.0 ms ✓ | 167 µs ✓ | 171 µs ✓ | 318 µs ✓ | 21.9 ms ✓ | 154 µs ✓ | 166 µs ✓ | 325 µs ✓ |

## SYJ201

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ201+1.001.p | 77 µs ✓ | 32 µs ✓ | 429 µs ✓ | 13 µs ✓ | 92 µs ✓ | 146 µs ✓ | 224 µs ✓ | 95 µs ✓ | 335 µs ✓ | 155 µs ✓ | 242 µs ✓ | 99 µs ✓ | 347 µs ✓ |
| SYJ201+1.002.p | 753 µs ✓ | 39 µs ✓ | 10.1 ms ✓ | 19 µs ✓ | 296 µs ✓ | 1.3 ms ✓ | 846 µs ✓ | 156 µs ✓ | 1.0 ms ✓ | 1.3 ms ✓ | 825 µs ✓ | 149 µs ✓ | 996 µs ✓ |
| SYJ201+1.003.p | 13.6 ms ✓ | 51 µs ✓ | 17.6 ms ✓ | 27 µs ✓ | 720 µs ✓ | 5.6 ms ✓ | 2.0 ms ✓ | 197 µs ✓ | 2.3 ms ✓ | 5.5 ms ✓ | 2.0 ms ✓ | 208 µs ✓ | 2.3 ms ✓ |
| SYJ201+1.004.p | 234.2 ms ✓ | 51 µs ✓ | 17.7 ms ✓ | 81 µs ✓ | 1.3 ms ✓ | 185.1 ms ✓ | 3.1 ms unknown | 253 µs ✓ | 3.7 ms ✓ | 178.6 ms ✓ | 3.1 ms unknown | 288 µs ✓ | 3.6 ms ✓ |
| SYJ201+1.005.p | 1.57 s ? memory | 66 µs ✓ | 18.6 ms ✓ | 12 µs unknown | 2.5 ms ✓ | 714.7 ms ✓ | 2.9 ms unknown | 341 µs ✓ | 3.6 ms ✓ | 712.5 ms ✓ | 3.0 ms unknown | 331 µs ✓ | 3.6 ms ✓ |
| SYJ201+1.006.p | 1.72 s ? memory | 65 µs ✓ | 18.3 ms ✓ | 15 µs unknown | 3.7 ms ✓ | > 2 s | 3.1 ms unknown | 414 µs ✓ | 3.5 ms ✓ | > 2 s | 3.2 ms unknown | 427 µs ✓ | 3.5 ms ✓ |
| SYJ201+1.007.p | > 2 s | 68 µs ✓ | 19.3 ms ✓ | 13 µs unknown | 5.6 ms ✓ | > 2 s | 4.1 ms unknown | 482 µs ✓ | 4.6 ms ✓ | > 2 s | 4.2 ms unknown | 522 µs ✓ | 4.7 ms ✓ |
| SYJ201+1.008.p | 1.04 s ? memory | 78 µs ✓ | 17.6 ms ✓ | 13 µs unknown | 8.2 ms ✓ | > 2 s | 5.1 ms unknown | 620 µs ✓ | 5.7 ms ✓ | > 2 s | 4.9 ms unknown | 612 µs ✓ | 5.5 ms ✓ |
| SYJ201+1.009.p | 1.17 s ? memory | 82 µs ✓ | 17.6 ms ✓ | 14 µs unknown | 11.8 ms ✓ | > 2 s | 6.1 ms unknown | 690 µs ✓ | 6.7 ms ✓ | > 2 s | 5.9 ms unknown | 690 µs ✓ | 6.8 ms ✓ |
| SYJ201+1.010.p | 1.25 s ? memory | 91 µs ✓ | 17.6 ms ✓ | 13 µs unknown | 17.0 ms ✓ | > 2 s | 7.5 ms unknown | 777 µs ✓ | 8.1 ms ✓ | > 2 s | 7.4 ms unknown | 772 µs ✓ | 8.1 ms ✓ |
| SYJ201+1.011.p | 1.50 s ? memory | 85 µs ✓ | 17.2 ms ✓ | 12 µs unknown | 23.6 ms ✓ | > 2 s | 8.7 ms unknown | 878 µs ✓ | 9.8 ms ✓ | > 2 s | 8.8 ms unknown | 870 µs ✓ | 9.8 ms ✓ |
| SYJ201+1.012.p | 1.53 s ? memory | 94 µs ✓ | 18.1 ms ✓ | 12 µs unknown | 30.2 ms ✓ | > 2 s | 10.7 ms unknown | 1.0 ms ✓ | 11.8 ms ✓ | > 2 s | 10.8 ms unknown | 1.1 ms ✓ | 11.9 ms ✓ |
| SYJ201+1.013.p | 1.69 s ? memory | 88 µs ✓ | 17.1 ms ✓ | 13 µs unknown | 15.2 ms unknown | > 2 s | 12.6 ms unknown | 1.2 ms ✓ | 13.9 ms ✓ | > 2 s | 12.7 ms unknown | 1.2 ms ✓ | 14.0 ms ✓ |
| SYJ201+1.014.p | > 2 s | 103 µs ✓ | 18.0 ms ✓ | 14 µs unknown | 19.1 ms unknown | > 2 s | 18.9 ms unknown | 1.3 ms ✓ | 20.4 ms ✓ | > 2 s | 18.9 ms unknown | 1.4 ms ✓ | 20.2 ms ✓ |
| SYJ201+1.015.p | > 2 s | 98 µs ✓ | 17.7 ms ✓ | 12 µs unknown | 22.4 ms unknown | > 2 s | 22.1 ms unknown | 1.5 ms ✓ | 23.5 ms ✓ | > 2 s | 22.0 ms unknown | 1.5 ms ✓ | 23.5 ms ✓ |
| SYJ201+1.016.p | 966.2 ms ? memory | 121 µs ✓ | 17.3 ms ✓ | 14 µs unknown | 28.9 ms unknown | > 2 s | 26.2 ms unknown | 1.6 ms ✓ | 27.7 ms ✓ | > 2 s | 26.2 ms unknown | 1.6 ms ✓ | 27.8 ms ✓ |
| SYJ201+1.017.p | 1.03 s ? memory | 109 µs ✓ | 17.4 ms ✓ | 13 µs unknown | 34.3 ms unknown | > 2 s | 30.5 ms unknown | 1.7 ms ✓ | 31.8 ms ✓ | > 2 s | 30.3 ms unknown | 1.7 ms ✓ | 31.9 ms ✓ |
| SYJ201+1.018.p | 1.10 s ? memory | 125 µs ✓ | 16.7 ms ✓ | 10 µs unknown | 40.8 ms unknown | > 2 s | 33.1 ms unknown | 2.1 ms ✓ | 35.6 ms ✓ | > 2 s | 33.0 ms unknown | 2.1 ms ✓ | 35.2 ms ✓ |
| SYJ201+1.019.p | 1.14 s ? memory | 127 µs ✓ | 16.7 ms ✓ | 11 µs unknown | 48.5 ms unknown | > 2 s | 37.4 ms unknown | 2.2 ms ✓ | 40.3 ms ✓ | > 2 s | 37.6 ms unknown | 2.2 ms ✓ | 40.4 ms ✓ |
| SYJ201+1.020.p | 1.21 s ? memory | 127 µs ✓ | 16.6 ms ✓ | 15 µs unknown | 56.9 ms unknown | > 2 s | 42.3 ms unknown | 2.4 ms ✓ | 45.6 ms ✓ | > 2 s | 42.1 ms unknown | 2.4 ms ✓ | 44.7 ms ✓ |

## SYJ202

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ202+1.001.p | 43 µs ✓ | 33 µs ✓ | 90 µs ✓ | 13 µs ✓ | 31 µs ✓ | 37 µs ✓ | 17 µs ✓ | 33 µs ✓ | 38 µs ✓ | 36 µs ✓ | 17 µs ✓ | 34 µs ✓ | 39 µs ✓ |
| SYJ202+1.002.p | 290 µs ✓ | 40 µs ✓ | 2.5 ms ✓ | 17 µs ✓ | 280 µs ✓ | 377 µs ✓ | 62 µs ✓ | 61 µs ✓ | 119 µs ✓ | 385 µs ✓ | 59 µs ✓ | 59 µs ✓ | 119 µs ✓ |
| SYJ202+1.003.p | 1.97 s ? memory | 62 µs ✓ | 16.4 ms ✓ | 13 µs unknown | 194.9 ms unknown | 83.2 ms ✓ | 755 µs ✓ | 255 µs ✓ | 1.1 ms ✓ | 83.8 ms ✓ | 770 µs ✓ | 270 µs ✓ | 1.1 ms ✓ |
| SYJ202+1.004.p | 1.56 s ? memory | 156 µs ✓ | 17.9 ms ✓ | 13 µs unknown | 115.5 ms ? memory | > 2 s | 38.0 ms unknown | 39.1 ms unknown | > 2 s | > 2 s | 38.5 ms unknown | 39.3 ms unknown | > 2 s |
| SYJ202+1.005.p | 1.39 s ? memory | 679 µs ✓ | 25.7 ms ✓ | 13 µs unknown | 68.3 ms ? memory | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s |
| SYJ202+1.006.p | 1.09 s ? memory | 2.2 ms ✓ | 108.9 ms ✓ | 12 µs unknown | 27.5 ms ? memory | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s |
| SYJ202+1.007.p | 1.71 s ? memory | 8.9 ms ✓ | 129.5 ms ✓ | 13 µs unknown | 18.8 ms ? memory | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s |
| SYJ202+1.008.p | 1.18 s ? memory | 42.5 ms ✓ | 160.1 ms ✓ | 13 µs unknown | 13.4 ms ? memory | > 2 s | 1.59 s ? memory | 1.76 s unknown | > 2 s | > 2 s | 1.62 s ? memory | 2.13 s unknown | > 2 s |
| SYJ202+1.009.p | 1.74 s ? memory | 181.5 ms ✓ | 297.0 ms ✓ | 12 µs unknown | 9.3 ms ? memory | > 2 s | 1.59 s ? memory | > 2 s | > 2 s | > 2 s | 1.66 s ? memory | > 2 s | > 2 s |
| SYJ202+1.010.p | 1.13 s ? memory | 1.29 s ✓ | 1.37 s ✓ | 11 µs unknown | 7.9 ms ? memory | 86.4 ms ? depth | 1.62 s ? memory | > 2 s | > 2 s | 87.3 ms ? depth | 1.61 s ? memory | > 2 s | > 2 s |
| SYJ202+1.011.p | 1.45 s ? memory | > 2 s | > 2 s | 13 µs unknown | 7.0 ms ? memory | 82.8 ms ? depth | 1.57 s ? memory | > 2 s | > 2 s | 82.4 ms ? depth | 1.55 s ? memory | > 2 s | > 2 s |
| SYJ202+1.012.p | > 2 s | > 2 s | > 2 s | 10 µs unknown | 7.2 ms ? memory | 80.4 ms ? depth | 1.52 s ? memory | > 2 s | > 2 s | 81.0 ms ? depth | 1.53 s ? memory | > 2 s | > 2 s |
| SYJ202+1.013.p | 1.25 s ? memory | > 2 s | > 2 s | 12 µs unknown | 8.6 ms ? memory | 79.0 ms ? depth | 1.50 s ? memory | > 2 s | > 2 s | 79.1 ms ? depth | 1.50 s ? memory | > 2 s | > 2 s |
| SYJ202+1.014.p | 1.48 s ? memory | > 2 s | > 2 s | 10 µs unknown | 11.1 ms ? memory | 77.5 ms ? depth | 1.53 s ? memory | > 2 s | > 2 s | 78.6 ms ? depth | 1.51 s ? memory | > 2 s | > 2 s |
| SYJ202+1.015.p | > 2 s | > 2 s | > 2 s | 13 µs unknown | 15.3 ms ? memory | 78.1 ms ? depth | 1.48 s ? memory | > 2 s | > 2 s | 78.3 ms ? depth | 1.45 s ? memory | > 2 s | > 2 s |
| SYJ202+1.016.p | 90.6 ms ? depth | > 2 s | > 2 s | 11 µs unknown | 21.8 ms ? memory | 78.3 ms ? depth | 1.46 s ? memory | > 2 s | > 2 s | 78.0 ms ? depth | 1.48 s ? memory | > 2 s | > 2 s |
| SYJ202+1.017.p | 91.9 ms ? depth | > 2 s | > 2 s | 14 µs unknown | 30.3 ms ? memory | 77.7 ms ? depth | 1.47 s ? memory | > 2 s | > 2 s | 77.4 ms ? depth | 1.47 s ? memory | > 2 s | > 2 s |
| SYJ202+1.018.p | 91.5 ms ? depth | > 2 s | > 2 s | 16 µs unknown | 43.1 ms ? memory | 77.6 ms ? depth | 1.56 s ? memory | > 2 s | > 2 s | 77.3 ms ? depth | 1.57 s ? memory | > 2 s | > 2 s |
| SYJ202+1.019.p | 89.8 ms ? depth | > 2 s | > 2 s | 12 µs unknown | 59.5 ms ? memory | 77.2 ms ? depth | 1.54 s ? memory | > 2 s | > 2 s | 77.9 ms ? depth | 1.59 s ? memory | > 2 s | > 2 s |
| SYJ202+1.020.p | 91.8 ms ? depth | > 2 s | > 2 s | 13 µs unknown | 82.4 ms ? memory | 79.7 ms ? depth | 1.48 s ? memory | > 2 s | > 2 s | 80.1 ms ? depth | 1.47 s ? memory | > 2 s | > 2 s |

## SYJ203

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ203+1.001.p | 41 µs ✓ | 39 µs ✓ | 89 µs ✓ | 14 µs ✓ | 31 µs ✓ | 51 µs ✓ | 25 µs ✓ | 32 µs ✓ | 51 µs ✓ | 44 µs ✓ | 25 µs ✓ | 33 µs ✓ | 51 µs ✓ |
| SYJ203+1.002.p | 46 µs ✓ | 31 µs ✓ | 113 µs ✓ | 13 µs ✓ | 43 µs ✓ | 75 µs ✓ | 43 µs ✓ | 49 µs ✓ | 93 µs ✓ | 77 µs ✓ | 43 µs ✓ | 48 µs ✓ | 82 µs ✓ |
| SYJ203+1.003.p | 52 µs ✓ | 45 µs ✓ | 163 µs ✓ | 16 µs ✓ | 53 µs ✓ | 135 µs ✓ | 54 µs ✓ | 56 µs ✓ | 107 µs ✓ | 135 µs ✓ | 56 µs ✓ | 56 µs ✓ | 111 µs ✓ |
| SYJ203+1.004.p | 45 µs ✓ | 39 µs ✓ | 148 µs ✓ | 17 µs ✓ | 60 µs ✓ | 324 µs ✓ | 73 µs ✓ | 73 µs ✓ | 149 µs ✓ | 336 µs ✓ | 78 µs ✓ | 74 µs ✓ | 140 µs ✓ |
| SYJ203+1.005.p | 45 µs ✓ | 38 µs ✓ | 165 µs ✓ | 18 µs ✓ | 75 µs ✓ | 1.0 ms ✓ | 88 µs ✓ | 85 µs ✓ | 172 µs ✓ | 1.0 ms ✓ | 87 µs ✓ | 101 µs ✓ | 191 µs ✓ |
| SYJ203+1.006.p | 43 µs ✓ | 39 µs ✓ | 193 µs ✓ | 26 µs ✓ | 85 µs ✓ | 3.7 ms ✓ | 107 µs ✓ | 100 µs ✓ | 199 µs ✓ | 3.7 ms ✓ | 109 µs ✓ | 104 µs ✓ | 204 µs ✓ |
| SYJ203+1.007.p | 42 µs ✓ | 43 µs ✓ | 239 µs ✓ | 42 µs ✓ | 108 µs ✓ | 13.9 ms ✓ | 135 µs ✓ | 119 µs ✓ | 247 µs ✓ | 13.8 ms ✓ | 127 µs ✓ | 120 µs ✓ | 240 µs ✓ |
| SYJ203+1.008.p | 47 µs ✓ | 49 µs ✓ | 288 µs ✓ | 74 µs ✓ | 131 µs ✓ | 52.7 ms ✓ | 153 µs ✓ | 131 µs ✓ | 291 µs ✓ | 52.3 ms ✓ | 161 µs ✓ | 133 µs ✓ | 284 µs ✓ |
| SYJ203+1.009.p | 50 µs ✓ | 45 µs ✓ | 290 µs ✓ | 146 µs ✓ | 147 µs ✓ | 128.2 ms ✓ | 191 µs ✓ | 166 µs ✓ | 343 µs ✓ | 125.8 ms ✓ | 169 µs ✓ | 157 µs ✓ | 340 µs ✓ |
| SYJ203+1.010.p | 50 µs ✓ | 43 µs ✓ | 320 µs ✓ | 7 µs unknown | 184 µs ✓ | 126.6 ms ✓ | 203 µs ✓ | 167 µs ✓ | 380 µs ✓ | 125.3 ms ✓ | 222 µs ✓ | 181 µs ✓ | 403 µs ✓ |
| SYJ203+1.011.p | 50 µs ✓ | 47 µs ✓ | 361 µs ✓ | 8 µs unknown | 224 µs ✓ | 140.7 ms ✓ | 229 µs ✓ | 207 µs ✓ | 443 µs ✓ | 130.3 ms ✓ | 225 µs ✓ | 216 µs ✓ | 440 µs ✓ |
| SYJ203+1.012.p | 58 µs ✓ | 46 µs ✓ | 417 µs ✓ | 9 µs unknown | 243 µs ✓ | 138.1 ms ✓ | 256 µs ✓ | 218 µs ✓ | 488 µs ✓ | 138.6 ms ✓ | 277 µs ✓ | 237 µs ✓ | 502 µs ✓ |
| SYJ203+1.013.p | 61 µs ✓ | 47 µs ✓ | 476 µs ✓ | 9 µs unknown | 283 µs ✓ | 147.7 ms ✓ | 285 µs ✓ | 241 µs ✓ | 520 µs ✓ | 147.7 ms ✓ | 291 µs ✓ | 255 µs ✓ | 538 µs ✓ |
| SYJ203+1.014.p | 61 µs ✓ | 50 µs ✓ | 522 µs ✓ | 8 µs unknown | 330 µs ✓ | 140.3 ms ✓ | 315 µs ✓ | 265 µs ✓ | 592 µs ✓ | 137.8 ms ✓ | 317 µs ✓ | 285 µs ✓ | 600 µs ✓ |
| SYJ203+1.015.p | 60 µs ✓ | 51 µs ✓ | 564 µs ✓ | 10 µs unknown | 355 µs ✓ | 156.1 ms ✓ | 364 µs ✓ | 312 µs ✓ | 656 µs ✓ | 143.9 ms ✓ | 347 µs ✓ | 313 µs ✓ | 729 µs ✓ |
| SYJ203+1.016.p | 69 µs ✓ | 57 µs ✓ | 672 µs ✓ | 10 µs unknown | 409 µs ✓ | 144.5 ms ✓ | 394 µs ✓ | 329 µs ✓ | 747 µs ✓ | 148.7 ms ✓ | 437 µs ✓ | 355 µs ✓ | 802 µs ✓ |
| SYJ203+1.017.p | 77 µs ✓ | 57 µs ✓ | 752 µs ✓ | 9 µs unknown | 466 µs ✓ | 150.6 ms ✓ | 429 µs ✓ | 350 µs ✓ | 824 µs ✓ | 160.8 ms ✓ | 444 µs ✓ | 389 µs ✓ | 844 µs ✓ |
| SYJ203+1.018.p | 87 µs ✓ | 56 µs ✓ | 810 µs ✓ | 8 µs unknown | 530 µs ✓ | 148.0 ms ✓ | 459 µs ✓ | 386 µs ✓ | 871 µs ✓ | 149.8 ms ✓ | 475 µs ✓ | 414 µs ✓ | 914 µs ✓ |
| SYJ203+1.019.p | 88 µs ✓ | 59 µs ✓ | 888 µs ✓ | 8 µs unknown | 595 µs ✓ | 149.6 ms ✓ | 501 µs ✓ | 414 µs ✓ | 981 µs ✓ | 150.8 ms ✓ | 505 µs ✓ | 483 µs ✓ | 1.0 ms ✓ |
| SYJ203+1.020.p | 89 µs ✓ | 59 µs ✓ | 962 µs ✓ | 9 µs unknown | 657 µs ✓ | 155.2 ms ✓ | 564 µs ✓ | 455 µs ✓ | 1.0 ms ✓ | 156.1 ms ✓ | 549 µs ✓ | 493 µs ✓ | 1.1 ms ✓ |

## SYJ204

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ204+1.001.p | 28 µs ✓ | 20 µs ✓ | 74 µs ✓ | 11 µs ✓ | 25 µs ✓ | 101 µs ✓ | 23 µs ✓ | 31 µs ✓ | 44 µs ✓ | 98 µs ✓ | 20 µs ✓ | 31 µs ✓ | 41 µs ✓ |
| SYJ204+1.002.p | 41 µs ✓ | 28 µs ✓ | 104 µs ✓ | 11 µs ✓ | 39 µs ✓ | 105 µs ✓ | 25 µs ✓ | 31 µs ✓ | 52 µs ✓ | 107 µs ✓ | 25 µs ✓ | 34 µs ✓ | 50 µs ✓ |
| SYJ204+1.003.p | 45 µs ✓ | 29 µs ✓ | 164 µs ✓ | 12 µs ✓ | 64 µs ✓ | 121 µs ✓ | 26 µs ✓ | 35 µs ✓ | 53 µs ✓ | 126 µs ✓ | 29 µs ✓ | 36 µs ✓ | 53 µs ✓ |
| SYJ204+1.004.p | 65 µs ✓ | 29 µs ✓ | 322 µs ✓ | 13 µs ✓ | 123 µs ✓ | 117 µs ✓ | 30 µs ✓ | 40 µs ✓ | 61 µs ✓ | 121 µs ✓ | 29 µs ✓ | 35 µs ✓ | 60 µs ✓ |
| SYJ204+1.005.p | 118 µs ✓ | 31 µs ✓ | 842 µs ✓ | 16 µs ✓ | 239 µs ✓ | 118 µs ✓ | 38 µs ✓ | 40 µs ✓ | 66 µs ✓ | 124 µs ✓ | 36 µs ✓ | 40 µs ✓ | 66 µs ✓ |
| SYJ204+1.006.p | 268 µs ✓ | 33 µs ✓ | 2.7 ms ✓ | 21 µs ✓ | 483 µs ✓ | 129 µs ✓ | 39 µs ✓ | 43 µs ✓ | 72 µs ✓ | 135 µs ✓ | 41 µs ✓ | 45 µs ✓ | 72 µs ✓ |
| SYJ204+1.007.p | 767 µs ✓ | 38 µs ✓ | 9.0 ms ✓ | 30 µs ✓ | 1.1 ms ✓ | 155 µs ✓ | 42 µs ✓ | 46 µs ✓ | 77 µs ✓ | 142 µs ✓ | 37 µs ✓ | 44 µs ✓ | 74 µs ✓ |
| SYJ204+1.008.p | 2.4 ms ✓ | 38 µs ✓ | 31.8 ms ✓ | 53 µs ✓ | 2.3 ms ✓ | 143 µs ✓ | 43 µs ✓ | 49 µs ✓ | 82 µs ✓ | 162 µs ✓ | 41 µs ✓ | 46 µs ✓ | 79 µs ✓ |
| SYJ204+1.009.p | 8.0 ms ✓ | 44 µs ✓ | 14.3 ms ✓ | 95 µs ✓ | 5.3 ms ✓ | 173 µs ✓ | 48 µs ✓ | 46 µs ✓ | 82 µs ✓ | 166 µs ✓ | 46 µs ✓ | 49 µs ✓ | 85 µs ✓ |
| SYJ204+1.010.p | 25.6 ms ✓ | 37 µs ✓ | 14.0 ms ✓ | 10 µs unknown | 11.7 ms ✓ | 187 µs ✓ | 50 µs ✓ | 49 µs ✓ | 93 µs ✓ | 179 µs ✓ | 54 µs ✓ | 52 µs ✓ | 100 µs ✓ |
| SYJ204+1.011.p | 84.5 ms ✓ | 38 µs ✓ | 14.1 ms ✓ | 10 µs unknown | 26.2 ms ✓ | 209 µs ✓ | 59 µs ✓ | 54 µs ✓ | 110 µs ✓ | 210 µs ✓ | 55 µs ✓ | 53 µs ✓ | 103 µs ✓ |
| SYJ204+1.012.p | 284.6 ms ✓ | 44 µs ✓ | 15.3 ms ✓ | 12 µs unknown | 61.6 ms ✓ | 240 µs ✓ | 60 µs ✓ | 56 µs ✓ | 105 µs ✓ | 236 µs ✓ | 60 µs ✓ | 54 µs ✓ | 105 µs ✓ |
| SYJ204+1.013.p | 984.9 ms ✓ | 44 µs ✓ | 14.7 ms ✓ | 11 µs unknown | 61.7 ms unknown | 268 µs ✓ | 66 µs ✓ | 59 µs ✓ | 113 µs ✓ | 259 µs ✓ | 64 µs ✓ | 57 µs ✓ | 109 µs ✓ |
| SYJ204+1.014.p | 1.81 s ? memory | 48 µs ✓ | 15.7 ms ✓ | 11 µs unknown | 134.0 ms unknown | 289 µs ✓ | 63 µs ✓ | 57 µs ✓ | 113 µs ✓ | 273 µs ✓ | 63 µs ✓ | 57 µs ✓ | 125 µs ✓ |
| SYJ204+1.015.p | 1.50 s ? memory | 56 µs ✓ | 15.9 ms ✓ | 12 µs unknown | 284.3 ms unknown | 320 µs ✓ | 65 µs ✓ | 58 µs ✓ | 128 µs ✓ | 316 µs ✓ | 68 µs ✓ | 61 µs ✓ | 118 µs ✓ |
| SYJ204+1.016.p | 1.58 s ? memory | 51 µs ✓ | 14.5 ms ✓ | 12 µs unknown | 605.2 ms unknown | 341 µs ✓ | 69 µs ✓ | 65 µs ✓ | 131 µs ✓ | 364 µs ✓ | 74 µs ✓ | 69 µs ✓ | 128 µs ✓ |
| SYJ204+1.017.p | 1.65 s ? memory | 55 µs ✓ | 14.8 ms ✓ | 12 µs unknown | 167.7 ms ? memory | 672 µs ✓ | 75 µs ✓ | 66 µs ✓ | 141 µs ✓ | 669 µs ✓ | 72 µs ✓ | 64 µs ✓ | 140 µs ✓ |
| SYJ204+1.018.p | 1.71 s ? memory | 54 µs ✓ | 16.1 ms ✓ | 14 µs unknown | 162.1 ms ? memory | 747 µs ✓ | 80 µs ✓ | 66 µs ✓ | 140 µs ✓ | 730 µs ✓ | 82 µs ✓ | 67 µs ✓ | 144 µs ✓ |
| SYJ204+1.019.p | 1.79 s ? memory | 56 µs ✓ | 15.6 ms ✓ | 11 µs unknown | 153.3 ms ? memory | 766 µs ✓ | 87 µs ✓ | 67 µs ✓ | 148 µs ✓ | 774 µs ✓ | 87 µs ✓ | 68 µs ✓ | 153 µs ✓ |
| SYJ204+1.020.p | 1.83 s ? memory | 54 µs ✓ | 15.6 ms ✓ | 13 µs unknown | 143.6 ms ? memory | 841 µs ✓ | 98 µs ✓ | 79 µs ✓ | 159 µs ✓ | 845 µs ✓ | 93 µs ✓ | 71 µs ✓ | 169 µs ✓ |

## SYJ205

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ205+1.001.p | 56 µs ✓ | 44 µs ✓ | 173 µs ✓ | 15 µs ✓ | 73 µs ✓ | 125 µs ✓ | 52 µs ✓ | 60 µs ✓ | 105 µs ✓ | 129 µs ✓ | 52 µs ✓ | 70 µs ✓ | 102 µs ✓ |
| SYJ205+1.002.p | 68 µs ✓ | 47 µs ✓ | 261 µs ✓ | 22 µs ✓ | 105 µs ✓ | 173 µs ✓ | 72 µs ✓ | 68 µs ✓ | 137 µs ✓ | 185 µs ✓ | 74 µs ✓ | 68 µs ✓ | 132 µs ✓ |
| SYJ205+1.003.p | 81 µs ✓ | 53 µs ✓ | 450 µs ✓ | 53 µs ✓ | 158 µs ✓ | 291 µs ✓ | 97 µs ✓ | 76 µs ✓ | 154 µs ✓ | 306 µs ✓ | 87 µs ✓ | 76 µs ✓ | 155 µs ✓ |
| SYJ205+1.004.p | 116 µs ✓ | 58 µs ✓ | 774 µs ✓ | 11 µs unknown | 223 µs ✓ | 1.6 ms ✓ | 122 µs ✓ | 88 µs ✓ | 195 µs ✓ | 1.6 ms ✓ | 121 µs ✓ | 80 µs ✓ | 193 µs ✓ |
| SYJ205+1.005.p | 178 µs ✓ | 55 µs ✓ | 1.7 ms ✓ | 13 µs unknown | 303 µs ✓ | 4.6 ms ✓ | 154 µs ✓ | 87 µs ✓ | 243 µs ✓ | 4.7 ms ✓ | 142 µs ✓ | 89 µs ✓ | 227 µs ✓ |
| SYJ205+1.006.p | 313 µs ✓ | 63 µs ✓ | 3.4 ms ✓ | 10 µs unknown | 397 µs ✓ | 15.2 ms ✓ | 179 µs ✓ | 98 µs ✓ | 270 µs ✓ | 15.3 ms ✓ | 185 µs ✓ | 97 µs ✓ | 271 µs ✓ |
| SYJ205+1.007.p | 637 µs ✓ | 66 µs ✓ | 7.5 ms ✓ | 13 µs unknown | 511 µs ✓ | 45.0 ms ✓ † | 232 µs ✓ | 107 µs ✓ | 342 µs ✓ | 45.2 ms ✓ † | 244 µs ✓ | 108 µs ✓ | 333 µs ✓ |
| SYJ205+1.008.p | 1.2 ms ✓ | 70 µs ✓ | 18.3 ms ✓ | 14 µs unknown | 727 µs ✓ | 113.1 ms ✓ † | 272 µs ✓ | 116 µs ✓ | 384 µs ✓ | 114.9 ms ✓ † | 282 µs ✓ | 118 µs ✓ | 378 µs ✓ |
| SYJ205+1.009.p | 2.6 ms ✓ | 71 µs ✓ | 41.2 ms ✓ | 14 µs unknown | 869 µs ✓ | 113.7 ms ✓ † | 357 µs ✓ | 149 µs ✓ | 494 µs ✓ | 115.8 ms ✓ † | 349 µs ✓ | 129 µs ✓ | 470 µs ✓ |
| SYJ205+1.010.p | 5.3 ms ✓ | 82 µs ✓ | 9.0 ms ✓ | 12 µs unknown | 1.1 ms ✓ | 113.6 ms ✓ † | 390 µs ✓ | 133 µs ✓ | 510 µs ✓ | 115.1 ms ✓ † | 391 µs ✓ | 145 µs ✓ | 529 µs ✓ |
| SYJ205+1.011.p | 11.6 ms ✓ | 81 µs ✓ | 16.3 ms ✓ | 14 µs unknown | 1.4 ms ✓ | 119.2 ms ✓ † | 477 µs ✓ | 136 µs ✓ | 598 µs ✓ | 122.0 ms ✓ † | 460 µs ✓ | 136 µs ✓ | 596 µs ✓ |
| SYJ205+1.012.p | 25.0 ms ✓ | 76 µs ✓ | 16.2 ms ✓ | 11 µs unknown | 1.6 ms ✓ | 122.8 ms ✓ † | 530 µs ✓ | 148 µs ✓ | 698 µs ✓ | 124.3 ms ✓ † | 531 µs ✓ | 147 µs ✓ | 676 µs ✓ |
| SYJ205+1.013.p | 53.1 ms ✓ | 76 µs ✓ | 15.5 ms ✓ | 12 µs unknown | 1.9 ms ✓ | 123.2 ms ✓ † | 631 µs ✓ | 159 µs ✓ | 767 µs ✓ | 124.1 ms ✓ † | 618 µs ✓ | 153 µs ✓ | 788 µs ✓ |
| SYJ205+1.014.p | 117.1 ms ✓ | 85 µs ✓ | 15.9 ms ✓ | 11 µs unknown | 2.2 ms ✓ | 127.3 ms ✓ † | 709 µs ✓ | 159 µs ✓ | 869 µs ✓ | 124.8 ms ✓ † | 726 µs ✓ | 153 µs ✓ | 856 µs ✓ |
| SYJ205+1.015.p | 254.4 ms ✓ | 88 µs ✓ | 14.7 ms ✓ | 12 µs unknown | 2.8 ms ✓ | 123.4 ms ✓ † | 803 µs ✓ | 163 µs ✓ | 961 µs ✓ | 124.9 ms ✓ † | 801 µs ✓ | 171 µs ✓ | 966 µs ✓ |
| SYJ205+1.016.p | 577.1 ms ✓ | 97 µs ✓ | 15.1 ms ✓ | 14 µs unknown | 3.4 ms ✓ | 133.9 ms ✓ † | 947 µs ✓ | 187 µs ✓ | 1.1 ms ✓ | 136.5 ms ✓ † | 951 µs ✓ | 175 µs ✓ | 1.1 ms ✓ |
| SYJ205+1.017.p | 1.25 s ✓ | 104 µs ✓ | 15.0 ms ✓ | 12 µs unknown | 3.8 ms ✓ | 134.9 ms ✓ † | 1.0 ms ✓ | 195 µs ✓ | 1.3 ms ✓ | 128.8 ms ✓ † | 1.1 ms ✓ | 186 µs ✓ | 1.2 ms ✓ |
| SYJ205+1.018.p | 1.53 s ? memory | 95 µs ✓ | 14.8 ms ✓ | 13 µs unknown | 4.5 ms ✓ | 127.3 ms ✓ † | 1.2 ms ✓ | 199 µs ✓ | 1.4 ms ✓ | 130.2 ms ✓ † | 1.2 ms ✓ | 204 µs ✓ | 1.4 ms ✓ |
| SYJ205+1.019.p | 1.54 s ? memory | 101 µs ✓ | 15.7 ms ✓ | 11 µs unknown | 5.2 ms ✓ | 128.5 ms ✓ † | 1.4 ms ✓ | 212 µs ✓ | 1.6 ms ✓ | 130.8 ms ✓ † | 1.4 ms ✓ | 211 µs ✓ | 1.6 ms ✓ |
| SYJ205+1.020.p | 1.52 s ? memory | 106 µs ✓ | 14.2 ms ✓ | 12 µs unknown | 6.0 ms ✓ | 133.2 ms ✓ † | 1.5 ms ✓ | 235 µs ✓ | 1.7 ms ✓ | 130.1 ms ✓ † | 1.5 ms ✓ | 221 µs ✓ | 1.7 ms ✓ |

## SYJ206

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ206+1.001.p | 38 µs ✓ | 20 µs ✓ | 63 µs ✓ | 11 µs ✓ | 23 µs ✓ | 36 µs ✓ | 18 µs ✓ | 31 µs ✓ | 43 µs ✓ | 33 µs ✓ | 18 µs ✓ | 30 µs ✓ | 42 µs ✓ |
| SYJ206+1.002.p | 41 µs ✓ | 27 µs ✓ | 134 µs ✓ | 10 µs ✓ | 61 µs ✓ | 112 µs ✓ | 25 µs ✓ | 43 µs ✓ | 65 µs ✓ | 126 µs ✓ | 28 µs ✓ | 47 µs ✓ | 66 µs ✓ |
| SYJ206+1.003.p | 119 µs ✓ | 37 µs ✓ | 764 µs ✓ | 10 µs ✓ | 216 µs ✓ | 160 µs ✓ | 56 µs ✓ | 118 µs ✓ | 177 µs ✓ | 149 µs ✓ | 57 µs ✓ | 116 µs ✓ | 167 µs ✓ |
| SYJ206+1.004.p | 2.4 ms ✓ | 57 µs ✓ | 26.5 ms ✓ | 13 µs ✓ | 931 µs ✓ | 310 µs ✓ | 143 µs ✓ | 336 µs ✓ | 484 µs ✓ | 310 µs ✓ | 142 µs ✓ | 374 µs ✓ | 520 µs ✓ |
| SYJ206+1.005.p | 517.5 ms ✓ | 83 µs ✓ | 16.5 ms ✓ | 16 µs ✓ | 4.1 ms ✓ | 3.6 ms ✓ † | 287 µs ✓ | 851 µs ✓ | 1.1 ms ✓ | 3.6 ms ✓ | 314 µs ✓ | 546 µs ✓ | 856 µs ✓ |
| SYJ206+1.006.p | 1.88 s ? memory | 95 µs ✓ | 15.3 ms ✓ | 15 µs ✓ | 21.7 ms ✓ | 34.3 ms ✓ | 664 µs ✓ | 2.0 ms ✓ | 2.6 ms ✓ | 34.2 ms ✓ † | 640 µs ✓ | 1.1 ms ✓ | 1.8 ms ✓ |
| SYJ206+1.007.p | 1.83 s ? memory | 136 µs ✓ | 12.9 ms ✓ | 18 µs ✓ | 63.8 ms unknown | 546.4 ms ✓ | 1.5 ms ✓ | 4.0 ms ✓ | 5.7 ms ✓ | 539.9 ms ✓ † | 1.4 ms ✓ | 4.2 ms ✓ | 5.7 ms ✓ |
| SYJ206+1.008.p | 1.36 s ? memory | 174 µs ✓ | 10.5 ms ✓ | 24 µs ✓ | 408.5 ms unknown | > 2 s | 3.1 ms ✓ | 6.0 ms ✓ | 9.6 ms ✓ | > 2 s | 3.3 ms ✓ | 7.0 ms ✓ | 10.8 ms ✓ |
| SYJ206+1.009.p | 1.51 s ? memory | 178 µs ✓ | 7.2 ms ✓ | 39 µs ✓ | 335.1 ms ? memory | > 2 s | 6.6 ms ✓ | 8.0 ms ✓ | 15.7 ms ✓ | > 2 s | 6.8 ms ✓ | 13.5 ms ✓ | 21.4 ms ✓ |
| SYJ206+1.010.p | 1.67 s ? memory | 228 µs ✓ | 5.0 ms ✓ | 67 µs ✓ | 299.0 ms ? memory | > 2 s | 8.2 ms unknown | 27.4 ms ✓ | 36.9 ms ✓ | > 2 s | 8.2 ms unknown | 38.6 ms ✓ | 48.1 ms ✓ |
| SYJ206+1.011.p | 1.85 s ? memory | 299 µs ✓ | 4.4 ms ✓ | 10 µs unknown | 283.9 ms ? memory | > 2 s | 14.4 ms unknown | 38.6 ms ✓ | 54.4 ms ✓ | > 2 s | 14.6 ms unknown | 40.9 ms ✓ | 56.2 ms ✓ |
| SYJ206+1.012.p | 1.99 s ? memory | 288 µs ✓ | 5.6 ms ✓ | 11 µs unknown | 265.1 ms ? memory | > 2 s | 29.4 ms unknown | 117.3 ms ✓ | 147.3 ms ✓ | > 2 s | 29.5 ms unknown | 127.5 ms ✓ | 157.9 ms ✓ |
| SYJ206+1.013.p | > 2 s | 304 µs ✓ | 10.3 ms ✓ | 12 µs unknown | 256.8 ms ? memory | > 2 s | 62.6 ms unknown | 270.0 ms ✓ | 330.0 ms ✓ | > 2 s | 62.7 ms unknown | 223.7 ms ✓ | 283.6 ms ✓ |
| SYJ206+1.014.p | > 2 s | 472 µs ✓ | 22.5 ms ✓ | 12 µs unknown | 240.7 ms ? memory | > 2 s | 136.1 ms unknown | 208.7 ms ✓ | 338.9 ms ✓ | > 2 s | 134.1 ms unknown | 536.8 ms ✓ | 670.4 ms ✓ |
| SYJ206+1.015.p | > 2 s | 428 µs ✓ | 42.8 ms ✓ | 13 µs unknown | 234.8 ms ? memory | > 2 s | 303.2 ms unknown | 1.76 s ✓ | 2.04 s ✓ | > 2 s | 308.5 ms unknown | 1.04 s ✓ | 1.34 s ✓ |
| SYJ206+1.016.p | > 2 s | 417 µs ✓ | 86.4 ms ✓ | 11 µs unknown | 216.1 ms ? memory | > 2 s | 683.0 ms unknown | > 2 s | > 2 s | > 2 s | 685.8 ms unknown | > 2 s | > 2 s |
| SYJ206+1.017.p | > 2 s | 541 µs ✓ | 567 µs ✓ | 10 µs unknown | 215.8 ms ? memory | > 2 s | 1.46 s unknown | > 2 s | > 2 s | > 2 s | 1.45 s unknown | > 2 s | > 2 s |
| SYJ206+1.018.p | > 2 s | 458 µs ✓ | 495 µs ✓ | 10 µs unknown | 196.3 ms ? memory | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s |
| SYJ206+1.019.p | > 2 s | 498 µs ✓ | 535 µs ✓ | 9 µs unknown | 200.1 ms ? memory | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s |
| SYJ206+1.020.p | > 2 s | 478 µs ✓ | 530 µs ✓ | 10 µs unknown | 181.7 ms ? memory | 947.4 ms ? memory | > 2 s | > 2 s | > 2 s | 942.9 ms ? memory | > 2 s | > 2 s | > 2 s |

## SYJ207

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ207+1.001.p | 54 µs ✓ | 35 µs ✓ | 182 µs ✓ | 13 µs ✓ | 29 µs ✓ | 112 µs ✗ | 49 µs ✗ | 46 µs ✗ | 46 µs ✗ | 101 µs ✗ | 47 µs ✗ | 49 µs ✗ | 45 µs ✗ |
| SYJ207+1.002.p | 246 µs ✓ | 41 µs ✓ | 2.4 ms ✓ | 14 µs ✓ | 33 µs ✓ | 5.1 ms ✗ | 348 µs ✗ | 62 µs ✗ | 59 µs ✗ | 5.2 ms ✗ | 321 µs ✗ | 58 µs ✗ | 57 µs ✗ |
| SYJ207+1.003.p | 3.4 ms ✓ | 47 µs ✓ | 6.0 ms ✓ | 28 µs ✓ | 35 µs ✓ | 481.2 ms ✗ | 3.7 ms ✗ | 70 µs ✗ | 69 µs ✗ | 482.4 ms ✗ | 3.7 ms ✗ | 75 µs ✗ | 75 µs ✗ |
| SYJ207+1.004.p | 61.2 ms ✓ | 56 µs ✓ | 18.4 ms ✓ | 73 µs ✓ | 39 µs ✓ | > 2 s | 48.8 ms ✗ | 82 µs ✗ | 83 µs ✗ | > 2 s | 48.3 ms ✗ | 82 µs ✗ | 83 µs ✗ |
| SYJ207+1.005.p | 983.4 ms ✓ | 61 µs ✓ | 18.1 ms ✓ | 13 µs unknown | 36 µs ✓ | > 2 s | 718.7 ms ✗ | 94 µs ✗ | 95 µs ✗ | > 2 s | 718.0 ms ✗ | 94 µs ✗ | 91 µs ✗ |
| SYJ207+1.006.p | 1.76 s ? memory | 60 µs ✓ | 18.7 ms ✓ | 11 µs unknown | 41 µs ✓ | > 2 s | > 2 s | 112 µs ✗ | 110 µs ✗ | > 2 s | > 2 s | 120 µs ✗ | 110 µs ✗ |
| SYJ207+1.007.p | > 2 s | 64 µs ✓ | 18.5 ms ✓ | 12 µs unknown | 39 µs ✓ | > 2 s | > 2 s | 134 µs ✗ | 126 µs ✗ | > 2 s | > 2 s | 136 µs ✗ | 128 µs ✗ |
| SYJ207+1.008.p | 1.07 s ? memory | 68 µs ✓ | 17.7 ms ✓ | 11 µs unknown | 40 µs ✓ | > 2 s | > 2 s | 149 µs ✗ | 140 µs ✗ | > 2 s | > 2 s | 143 µs ✗ | 137 µs ✗ |
| SYJ207+1.009.p | 1.19 s ? memory | 73 µs ✓ | 18.1 ms ✓ | 13 µs unknown | 49 µs ✓ | > 2 s | > 2 s | 156 µs ✗ | 162 µs ✗ | > 2 s | > 2 s | 152 µs ✗ | 151 µs ✗ |
| SYJ207+1.010.p | 1.30 s ? memory | 79 µs ✓ | 17.6 ms ✓ | 11 µs unknown | 48 µs ✓ | > 2 s | > 2 s | 172 µs ✗ | 169 µs ✗ | > 2 s | > 2 s | 182 µs ✗ | 175 µs ✗ |
| SYJ207+1.011.p | 1.51 s ? memory | 79 µs ✓ | 18.0 ms ✓ | 11 µs unknown | 52 µs ✓ | > 2 s | > 2 s | 212 µs ✗ | 195 µs ✗ | > 2 s | > 2 s | 195 µs ✗ | 188 µs ✗ |
| SYJ207+1.012.p | 1.59 s ? memory | 88 µs ✓ | 18.3 ms ✓ | 15 µs unknown | 51 µs ✓ | > 2 s | > 2 s | 213 µs ✗ | 211 µs ✗ | > 2 s | > 2 s | 215 µs ✗ | 209 µs ✗ |
| SYJ207+1.013.p | 1.68 s ? memory | 81 µs ✓ | 17.8 ms ✓ | 11 µs unknown | 51 µs ✓ | > 2 s | > 2 s | 235 µs ✗ | 231 µs ✗ | > 2 s | > 2 s | 241 µs ✗ | 236 µs ✗ |
| SYJ207+1.014.p | > 2 s | 90 µs ✓ | 18.0 ms ✓ | 13 µs unknown | 53 µs ✓ | > 2 s | > 2 s | 260 µs ✗ | 254 µs ✗ | > 2 s | > 2 s | 266 µs ✗ | 265 µs ✗ |
| SYJ207+1.015.p | > 2 s | 89 µs ✓ | 18.3 ms ✓ | 12 µs unknown | 62 µs ✓ | > 2 s | > 2 s | 283 µs ✗ | 280 µs ✗ | > 2 s | > 2 s | 283 µs ✗ | 275 µs ✗ |
| SYJ207+1.016.p | 1.00 s ? memory | 102 µs ✓ | 17.1 ms ✓ | 13 µs unknown | 58 µs ✓ | > 2 s | > 2 s | 313 µs ✗ | 308 µs ✗ | > 2 s | > 2 s | 306 µs ✗ | 306 µs ✗ |
| SYJ207+1.017.p | 1.06 s ? memory | 100 µs ✓ | 17.0 ms ✓ | 10 µs unknown | 59 µs ✓ | > 2 s | > 2 s | 337 µs ✗ | 324 µs ✗ | > 2 s | > 2 s | 334 µs ✗ | 331 µs ✗ |
| SYJ207+1.018.p | 1.09 s ? memory | 107 µs ✓ | 17.1 ms ✓ | 12 µs unknown | 62 µs ✓ | > 2 s | > 2 s | 357 µs ✗ | 354 µs ✗ | > 2 s | > 2 s | 363 µs ✗ | 363 µs ✗ |
| SYJ207+1.019.p | 1.16 s ? memory | 117 µs ✓ | 17.3 ms ✓ | 12 µs unknown | 72 µs ✓ | > 2 s | > 2 s | 391 µs ✗ | 391 µs ✗ | > 2 s | > 2 s | 389 µs ✗ | 385 µs ✗ |
| SYJ207+1.020.p | 1.24 s ? memory | 122 µs ✓ | 17.1 ms ✓ | 12 µs unknown | 72 µs ✓ | > 2 s | > 2 s | 436 µs ✗ | 431 µs ✗ | > 2 s | > 2 s | 420 µs ✗ | 420 µs ✗ |

## SYJ208

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ208+1.001.p | 33 µs ✓ | 27 µs ✓ | 72 µs ✓ | 11 µs ✓ | 28 µs ✓ | 101 µs ✗ | 22 µs ✗ | 46 µs ✗ | 42 µs ✗ | 169 µs ✗ | 19 µs ✗ | 41 µs ✗ | 41 µs ✗ |
| SYJ208+1.002.p | 257 µs ✓ | 40 µs ✓ | 2.4 ms ✓ | 18 µs ✓ | 294 µs ✓ | 1.3 ms ✗ | 42 µs ✗ | 55 µs ✗ | 54 µs ✗ | 2.4 ms ✗ | 42 µs ✗ | 56 µs ✗ | 54 µs ✗ |
| SYJ208+1.003.p | 1.97 s ? memory | 75 µs ✓ | 16.9 ms ✓ | 11 µs unknown | 195.9 ms unknown | 344.6 ms ✗ | 127 µs ✗ | 77 µs ✗ | 80 µs ✗ | 511.4 ms ✗ | 115 µs ✗ | 73 µs ✗ | 64 µs ✗ |
| SYJ208+1.004.p | 1.57 s ? memory | 202 µs ✓ | 18.4 ms ✓ | 11 µs unknown | 116.1 ms ? memory | > 2 s | 680 µs ✗ | 89 µs ✗ | 88 µs ✗ | 2.00 s ✗ † | 640 µs ✗ | 88 µs ✗ | 87 µs ✗ |
| SYJ208+1.005.p | 1.39 s ? memory | 857 µs ✓ | 25.7 ms ✓ | 12 µs unknown | 68.0 ms ? memory | > 2 s | 8.1 ms ✗ | 128 µs ✗ | 125 µs ✗ | 2.01 s ✗ | 7.7 ms ✗ | 128 µs ✗ | 122 µs ✗ |
| SYJ208+1.006.p | 1.08 s ? memory | 2.3 ms ✓ | 109.5 ms ✓ | 11 µs unknown | 30.9 ms ? memory | > 2 s | 393.6 ms ✗ | 179 µs ✗ | 171 µs ✗ | > 2 s | 345.8 ms ✗ | 179 µs ✗ | 175 µs ✗ |
| SYJ208+1.007.p | 1.71 s ? memory | 7.6 ms ✓ | 128.7 ms ✓ | 12 µs unknown | 20.2 ms ? memory | > 2 s | > 2 s | 252 µs ✗ | 245 µs ✗ | > 2 s | > 2 s | 254 µs ✗ | 246 µs ✗ |
| SYJ208+1.008.p | 1.17 s ? memory | 37.6 ms ✓ | 155.3 ms ✓ | 12 µs unknown | 14.3 ms ? memory | > 2 s | > 2 s | 352 µs ✗ | 333 µs ✗ | > 2 s | > 2 s | 340 µs ✗ | 342 µs ✗ |
| SYJ208+1.009.p | 1.75 s ? memory | 185.9 ms ✓ | 301.2 ms ✓ | 12 µs unknown | 9.9 ms ? memory | > 2 s | > 2 s | 475 µs ✗ | 456 µs ✗ | > 2 s | > 2 s | 481 µs ✗ | 470 µs ✗ |
| SYJ208+1.010.p | 1.13 s ? memory | 1.36 s ✓ | 1.50 s ✓ | 12 µs unknown | 8.2 ms ? memory | 81.2 ms ? depth | 2.45 s ? memory | 615 µs ✗ | 608 µs ✗ | 82.0 ms ? depth † | > 2 s | 629 µs ✗ | 608 µs ✗ |
| SYJ208+1.011.p | 1.45 s ? memory | > 2 s | > 2 s | 13 µs unknown | 7.4 ms ? memory | 75.9 ms ? depth | 2.38 s ? memory | 835 µs ✗ | 808 µs ✗ | 76.9 ms ? depth † | 2.31 s ? memory | 810 µs ✗ | 793 µs ✗ |
| SYJ208+1.012.p | > 2 s | > 2 s | > 2 s | 11 µs unknown | 7.3 ms ? memory | 74.2 ms ? depth | > 2 s | 1.1 ms ✗ | 1.1 ms ✗ | 76.3 ms ? depth † | 2.44 s ? memory | 1.1 ms ✗ | 1.1 ms ✗ |
| SYJ208+1.013.p | 1.23 s ? memory | > 2 s | > 2 s | 11 µs unknown | 8.8 ms ? memory | 73.3 ms ? depth | 2.26 s ? memory | 1.4 ms ✗ | 1.4 ms ✗ | 74.7 ms ? depth † | 2.26 s ? memory | 1.4 ms ✗ | 1.4 ms ✗ |
| SYJ208+1.014.p | 1.47 s ? memory | > 2 s | > 2 s | 15 µs unknown | 11.5 ms ? memory | 73.3 ms ? depth | 1.99 s ? memory | 1.8 ms ✗ | 1.7 ms ✗ | 74.9 ms ? depth | 2.00 s ? memory | 1.7 ms ✗ | 1.7 ms ✗ |
| SYJ208+1.015.p | > 2 s | > 2 s | > 2 s | 13 µs unknown | 15.3 ms ? memory | 72.8 ms ? depth | 2.12 s ? memory | 2.2 ms ✗ | 2.1 ms ✗ | 74.3 ms ? depth | 2.11 s ? memory | 2.2 ms ✗ | 2.2 ms ✗ |
| SYJ208+1.016.p | 90.5 ms ? depth | > 2 s | > 2 s | 13 µs unknown | 21.8 ms ? memory | 72.2 ms ? depth | 2.09 s ? memory | 2.8 ms ✗ | 2.7 ms ✗ | 74.3 ms ? depth | 2.08 s ? memory | 2.8 ms ✗ | 2.7 ms ✗ |
| SYJ208+1.017.p | 92.0 ms ? depth | > 2 s | > 2 s | 14 µs unknown | 30.6 ms ? memory | 72.1 ms ? depth | 2.00 s ? memory | 3.3 ms ✗ | 3.2 ms ✗ | 74.2 ms ? depth | 2.00 s ? memory | 3.3 ms ✗ | 3.3 ms ✗ |
| SYJ208+1.018.p | 92.9 ms ? depth | > 2 s | > 2 s | 14 µs unknown | 42.6 ms ? memory | 73.4 ms ? depth | 2.03 s ? memory | 4.0 ms ✗ | 4.0 ms ✗ | 74.0 ms ? depth | 2.03 s ? memory | 4.0 ms ✗ | 4.0 ms ✗ |
| SYJ208+1.019.p | 90.9 ms ? depth | > 2 s | > 2 s | 13 µs unknown | 59.4 ms ? memory | 72.3 ms ? depth | 2.18 s ? memory | 4.8 ms ✗ | 4.8 ms ✗ | 74.4 ms ? depth | 2.20 s ? memory | 4.8 ms ✗ | 4.8 ms ✗ |
| SYJ208+1.020.p | 92.1 ms ? depth | > 2 s | > 2 s | 13 µs unknown | 82.2 ms ? memory | 75.5 ms ? depth | 2.09 s ? memory | 5.8 ms ✗ | 5.8 ms ✗ | 77.8 ms ? depth | 2.06 s ? memory | 5.8 ms ✗ | 5.8 ms ✗ |

## SYJ209

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ209+1.001.p | 40 µs ✓ | 30 µs ✓ | 83 µs ✓ | 12 µs ✓ | 29 µs ✓ | 66 µs ✗ | 18 µs ✗ | 39 µs ✗ | 42 µs ✗ | 116 µs ✗ | 15 µs ✗ | 43 µs ✗ | 39 µs ✗ |
| SYJ209+1.002.p | 35 µs ✓ | 30 µs ✓ | 94 µs ✓ | 12 µs ✓ | 40 µs ✓ | 138 µs ✗ | 27 µs ✗ | 45 µs ✗ | 44 µs ✗ | 191 µs ✗ | 22 µs ✗ | 43 µs ✗ | 44 µs ✗ |
| SYJ209+1.003.p | 41 µs ✓ | 30 µs ✓ | 111 µs ✓ | 14 µs ✓ | 51 µs ✓ | 369 µs ✗ | 32 µs ✗ | 52 µs ✗ | 51 µs ✗ | 455 µs ✗ | 32 µs ✗ | 55 µs ✗ | 53 µs ✗ |
| SYJ209+1.004.p | 42 µs ✓ | 37 µs ✓ | 138 µs ✓ | 14 µs ✓ | 64 µs ✓ | 1.4 ms ✗ | 38 µs ✗ | 56 µs ✗ | 57 µs ✗ | 2.4 ms ✗ | 35 µs ✗ | 58 µs ✗ | 58 µs ✗ |
| SYJ209+1.005.p | 41 µs ✓ | 38 µs ✓ | 165 µs ✓ | 19 µs ✓ | 78 µs ✓ | 5.5 ms ✗ | 46 µs ✗ | 66 µs ✗ | 63 µs ✗ | 8.8 ms ✗ | 45 µs ✗ | 62 µs ✗ | 62 µs ✗ |
| SYJ209+1.006.p | 47 µs ✓ | 40 µs ✓ | 184 µs ✓ | 24 µs ✓ | 91 µs ✓ | 22.6 ms ✗ | 56 µs ✗ | 74 µs ✗ | 73 µs ✗ | 35.5 ms ✗ † | 48 µs ✗ | 67 µs ✗ | 71 µs ✗ |
| SYJ209+1.007.p | 43 µs ✓ | 41 µs ✓ | 221 µs ✓ | 43 µs ✓ | 110 µs ✓ | 95.3 ms ✗ | 77 µs ✗ | 78 µs ✗ | 78 µs ✗ | 116.2 ms ✗ † | 61 µs ✗ | 79 µs ✗ | 74 µs ✗ |
| SYJ209+1.008.p | 47 µs ✓ | 45 µs ✓ | 249 µs ✓ | 75 µs ✓ | 128 µs ✓ | 110.1 ms ✗ | 84 µs ✗ | 85 µs ✗ | 81 µs ✗ | 116.9 ms ✗ † | 75 µs ✗ | 84 µs ✗ | 81 µs ✗ |
| SYJ209+1.009.p | 49 µs ✓ | 48 µs ✓ | 318 µs ✓ | 166 µs ✓ | 166 µs ✓ | 116.8 ms ✗ | 98 µs ✗ | 88 µs ✗ | 88 µs ✗ | 126.1 ms ✗ † | 83 µs ✗ | 87 µs ✗ | 83 µs ✗ |
| SYJ209+1.010.p | 73 µs ✓ | 52 µs ✓ | 317 µs ✓ | 10 µs unknown | 177 µs ✓ | 117.8 ms ✗ | 106 µs ✗ | 94 µs ✗ | 109 µs ✗ | 126.8 ms ✗ † | 90 µs ✗ | 95 µs ✗ | 92 µs ✗ |
| SYJ209+1.011.p | 52 µs ✓ | 48 µs ✓ | 368 µs ✓ | 9 µs unknown | 215 µs ✓ | 122.0 ms ✗ | 121 µs ✗ | 99 µs ✗ | 98 µs ✗ | 132.9 ms ✗ † | 109 µs ✗ | 106 µs ✗ | 98 µs ✗ |
| SYJ209+1.012.p | 59 µs ✓ | 51 µs ✓ | 416 µs ✓ | 8 µs unknown | 253 µs ✓ | 130.2 ms ✗ | 131 µs ✗ | 107 µs ✗ | 104 µs ✗ | 142.6 ms ✗ † | 118 µs ✗ | 112 µs ✗ | 111 µs ✗ |
| SYJ209+1.013.p | 70 µs ✓ | 55 µs ✓ | 482 µs ✓ | 11 µs unknown | 294 µs ✓ | 135.9 ms ✗ | 144 µs ✗ | 112 µs ✗ | 111 µs ✗ | 142.7 ms ✗ † | 127 µs ✗ | 115 µs ✗ | 108 µs ✗ |
| SYJ209+1.014.p | 64 µs ✓ | 53 µs ✓ | 517 µs ✓ | 11 µs unknown | 348 µs ✓ | 140.0 ms ✗ | 172 µs ✗ | 123 µs ✗ | 121 µs ✗ | 151.8 ms ✗ † | 150 µs ✗ | 121 µs ✗ | 116 µs ✗ |
| SYJ209+1.015.p | 65 µs ✓ | 59 µs ✓ | 647 µs ✓ | 9 µs unknown | 364 µs ✓ | 158.0 ms ✗ | 202 µs ✗ | 127 µs ✗ | 131 µs ✗ | 187.8 ms ✗ † | 176 µs ✗ | 139 µs ✗ | 126 µs ✗ |
| SYJ209+1.016.p | 72 µs ✓ | 57 µs ✓ | 680 µs ✓ | 9 µs unknown | 412 µs ✓ | 193.9 ms ✗ | 224 µs ✗ | 139 µs ✗ | 142 µs ✗ | 237.8 ms ✗ † | 208 µs ✗ | 141 µs ✗ | 136 µs ✗ |
| SYJ209+1.017.p | 81 µs ✓ | 60 µs ✓ | 769 µs ✓ | 11 µs unknown | 485 µs ✓ | 253.2 ms ✗ | 240 µs ✗ | 147 µs ✗ | 153 µs ✗ | 273.2 ms ✗ † | 221 µs ✗ | 148 µs ✗ | 147 µs ✗ |
| SYJ209+1.018.p | 91 µs ✓ | 66 µs ✓ | 828 µs ✓ | 9 µs unknown | 549 µs ✓ | 251.0 ms ✗ | 278 µs ✗ | 169 µs ✗ | 155 µs ✗ | 262.5 ms ✗ † | 249 µs ✗ | 165 µs ✗ | 153 µs ✗ |
| SYJ209+1.019.p | 97 µs ✓ | 61 µs ✓ | 914 µs ✓ | 10 µs unknown | 612 µs ✓ | 263.0 ms ✗ | 309 µs ✗ | 165 µs ✗ | 166 µs ✗ | 266.1 ms ✗ † | 286 µs ✗ | 178 µs ✗ | 164 µs ✗ |
| SYJ209+1.020.p | 94 µs ✓ | 68 µs ✓ | 956 µs ✓ | 10 µs unknown | 689 µs ✓ | 250.3 ms ✗ | 337 µs ✗ | 175 µs ✗ | 171 µs ✗ | 262.4 ms ✗ † | 337 µs ✗ | 180 µs ✗ | 178 µs ✗ |

## SYJ210

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ210+1.001.p | 32 µs ✓ | 27 µs ✓ | 85 µs ✓ | 12 µs ✓ | 30 µs ✓ | 109 µs ✗ | 17 µs ✗ | 38 µs ✗ | 38 µs ✗ | 113 µs ✗ | 14 µs ✗ | 40 µs ✗ | 36 µs ✗ |
| SYJ210+1.002.p | 39 µs ✓ | 27 µs ✓ | 112 µs ✓ | 11 µs ✓ | 42 µs ✓ | 120 µs ✗ | 15 µs ✗ | 41 µs ✗ | 37 µs ✗ | 137 µs ✗ | 12 µs ✗ | 38 µs ✗ | 40 µs ✗ |
| SYJ210+1.003.p | 50 µs ✓ | 35 µs ✓ | 163 µs ✓ | 13 µs ✓ | 69 µs ✓ | 126 µs ✗ | 18 µs ✗ | 45 µs ✗ | 42 µs ✗ | 147 µs ✗ | 13 µs ✗ | 41 µs ✗ | 42 µs ✗ |
| SYJ210+1.004.p | 66 µs ✓ | 30 µs ✓ | 330 µs ✓ | 14 µs ✓ | 126 µs ✓ | 152 µs ✗ | 20 µs ✗ | 44 µs ✗ | 42 µs ✗ | 143 µs ✗ | 16 µs ✗ | 42 µs ✗ | 41 µs ✗ |
| SYJ210+1.005.p | 119 µs ✓ | 33 µs ✓ | 877 µs ✓ | 14 µs ✓ | 232 µs ✓ | 165 µs ✗ | 20 µs ✗ | 44 µs ✗ | 45 µs ✗ | 175 µs ✗ | 13 µs ✗ | 50 µs ✗ | 47 µs ✗ |
| SYJ210+1.006.p | 267 µs ✓ | 37 µs ✓ | 2.7 ms ✓ | 21 µs ✓ | 482 µs ✓ | 191 µs ✗ | 22 µs ✗ | 44 µs ✗ | 43 µs ✗ | 185 µs ✗ | 14 µs ✗ | 48 µs ✗ | 46 µs ✗ |
| SYJ210+1.007.p | 752 µs ✓ | 40 µs ✓ | 9.3 ms ✓ | 33 µs ✓ | 1.1 ms ✓ | 239 µs ✗ | 22 µs ✗ | 48 µs ✗ | 45 µs ✗ | 209 µs ✗ | 12 µs ✗ | 49 µs ✗ | 45 µs ✗ |
| SYJ210+1.008.p | 2.4 ms ✓ | 39 µs ✓ | 33.3 ms ✓ | 54 µs ✓ | 2.4 ms ✓ | 251 µs ✗ | 26 µs ✗ | 46 µs ✗ | 45 µs ✗ | 265 µs ✗ | 15 µs ✗ | 52 µs ✗ | 47 µs ✗ |
| SYJ210+1.009.p | 8.0 ms ✓ | 39 µs ✓ | 14.8 ms ✓ | 99 µs ✓ | 5.5 ms ✓ | 285 µs ✗ | 24 µs ✗ | 50 µs ✗ | 45 µs ✗ | 297 µs ✗ | 15 µs ✗ | 49 µs ✗ | 44 µs ✗ |
| SYJ210+1.010.p | 25.9 ms ✓ | 43 µs ✓ | 14.4 ms ✓ | 10 µs unknown | 12.0 ms ✓ | 350 µs ✗ | 27 µs ✗ | 51 µs ✗ | 50 µs ✗ | 348 µs ✗ | 14 µs ✗ | 49 µs ✗ | 48 µs ✗ |
| SYJ210+1.011.p | 87.2 ms ✓ | 50 µs ✓ | 15.1 ms ✓ | 13 µs unknown | 27.6 ms ✓ | 400 µs ✗ | 27 µs ✗ | 53 µs ✗ | 54 µs ✗ | 405 µs ✗ | 17 µs ✗ | 54 µs ✗ | 50 µs ✗ |
| SYJ210+1.012.p | 300.7 ms ✓ | 45 µs ✓ | 15.2 ms ✓ | 10 µs unknown | 61.8 ms ✓ | 1.1 ms ✗ | 29 µs ✗ | 57 µs ✗ | 52 µs ✗ | 769 µs ✗ | 15 µs ✗ | 53 µs ✗ | 52 µs ✗ |
| SYJ210+1.013.p | 1.01 s ✓ | 48 µs ✓ | 15.6 ms ✓ | 10 µs unknown | 61.7 ms unknown | 1.2 ms ✗ | 26 µs ✗ | 52 µs ✗ | 50 µs ✗ | 856 µs ✗ | 15 µs ✗ | 49 µs ✗ | 52 µs ✗ |
| SYJ210+1.014.p | 1.86 s ? memory | 46 µs ✓ | 16.1 ms ✓ | 13 µs unknown | 133.9 ms unknown | 1.2 ms ✗ | 29 µs ✗ | 54 µs ✗ | 54 µs ✗ | 913 µs ✗ | 17 µs ✗ | 52 µs ✗ | 53 µs ✗ |
| SYJ210+1.015.p | 1.51 s ? memory | 47 µs ✓ | 16.7 ms ✓ | 10 µs unknown | 285.8 ms unknown | 1.8 ms ✗ | 33 µs ✗ | 58 µs ✗ | 56 µs ✗ | 1.2 ms ✗ | 16 µs ✗ | 50 µs ✗ | 56 µs ✗ |
| SYJ210+1.016.p | 1.60 s ? memory | 48 µs ✓ | 15.4 ms ✓ | 11 µs unknown | 611.5 ms unknown | 1.9 ms ✗ | 37 µs ✗ | 56 µs ✗ | 55 µs ✗ | 1.4 ms ✗ | 16 µs ✗ | 59 µs ✗ | 53 µs ✗ |
| SYJ210+1.017.p | 1.65 s ? memory | 50 µs ✓ | 14.9 ms ✓ | 11 µs unknown | 171.4 ms ? memory | 2.1 ms ✗ | 36 µs ✗ | 56 µs ✗ | 53 µs ✗ | 1.5 ms ✗ | 17 µs ✗ | 57 µs ✗ | 57 µs ✗ |
| SYJ210+1.018.p | 1.70 s ? memory | 52 µs ✓ | 15.7 ms ✓ | 12 µs unknown | 161.1 ms ? memory | 2.8 ms ✗ | 39 µs ✗ | 54 µs ✗ | 56 µs ✗ | 1.6 ms ✗ | 18 µs ✗ | 59 µs ✗ | 54 µs ✗ |
| SYJ210+1.019.p | 1.77 s ? memory | 51 µs ✓ | 14.9 ms ✓ | 11 µs unknown | 155.7 ms ? memory | 3.5 ms ✗ | 46 µs ✗ | 55 µs ✗ | 58 µs ✗ | 2.1 ms ✗ | 17 µs ✗ | 60 µs ✗ | 55 µs ✗ |
| SYJ210+1.020.p | 1.84 s ? memory | 52 µs ✓ | 15.6 ms ✓ | 11 µs unknown | 148.3 ms ? memory † | 3.6 ms ✗ | 47 µs ✗ | 58 µs ✗ | 56 µs ✗ | 2.2 ms ✗ | 18 µs ✗ | 58 µs ✗ | 55 µs ✗ |

## SYJ211

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ211+1.001.p | 38 µs ✓ | 31 µs ✓ | 113 µs ✓ | 15 µs ✓ | 48 µs ✓ | 557 µs ✗ | 34 µs ✗ | 58 µs ✗ | 47 µs ✗ | 349 µs ✗ | 29 µs ✗ | 53 µs ✗ | 51 µs ✗ |
| SYJ211+1.002.p | 42 µs ✓ | 36 µs ✓ | 156 µs ✓ | 18 µs ✓ | 60 µs ✓ | 39.7 ms ✗ | 69 µs ✗ | 61 µs ✗ | 59 µs ✗ | 10.4 ms ✗ † | 61 µs ✗ | 63 µs ✗ | 62 µs ✗ |
| SYJ211+1.003.p | 51 µs ✓ | 38 µs ✓ | 295 µs ✓ | 48 µs ✓ | 83 µs ✓ | 108.1 ms ✗ | 126 µs ✗ | 66 µs ✗ | 67 µs ✗ | 106.8 ms ✗ | 107 µs ✗ | 62 µs ✗ | 61 µs ✗ |
| SYJ211+1.004.p | 82 µs ✓ | 43 µs ✓ | 426 µs ✓ | 11 µs unknown | 92 µs ✓ | 107.4 ms ✗ † | 217 µs ✗ | 74 µs ✗ | 72 µs ✗ | 111.6 ms ✗ † | 205 µs ✗ | 82 µs ✗ | 73 µs ✗ |
| SYJ211+1.005.p | 96 µs ✓ | 42 µs ✓ | 799 µs ✓ | 10 µs unknown | 116 µs ✓ | 105.4 ms ✗ | 375 µs ✗ | 80 µs ✗ | 75 µs ✗ | 107.4 ms ✗ † | 353 µs ✗ | 85 µs ✗ | 75 µs ✗ |
| SYJ211+1.006.p | 171 µs ✓ | 43 µs ✓ | 1.7 ms ✓ | 9 µs unknown | 129 µs ✓ | 102.0 ms ✗ | 636 µs ✗ | 84 µs ✗ | 84 µs ✗ | 105.5 ms ✗ † | 576 µs ✗ | 86 µs ✗ | 87 µs ✗ |
| SYJ211+1.007.p | 295 µs ✓ | 55 µs ✓ | 3.5 ms ✓ | 11 µs unknown | 151 µs ✓ | 100.8 ms ✗ † | 986 µs ✗ | 101 µs ✗ | 90 µs ✗ | 106.3 ms ✗ | 957 µs ✗ | 97 µs ✗ | 92 µs ✗ |
| SYJ211+1.008.p | 552 µs ✓ | 50 µs ✓ | 8.3 ms ✓ | 9 µs unknown | 191 µs ✓ | 97.1 ms ✗ | 1.7 ms ✗ | 107 µs ✗ | 101 µs ✗ | 101.5 ms ✗ | 1.7 ms ✗ | 107 µs ✗ | 97 µs ✗ |
| SYJ211+1.009.p | 1.1 ms ✓ | 53 µs ✓ | 18.5 ms ✓ | 11 µs unknown | 207 µs ✓ | 93.0 ms ✗ | 2.5 ms ✗ | 118 µs ✗ | 108 µs ✗ | 99.5 ms ✗ | 2.5 ms ✗ | 121 µs ✗ | 118 µs ✗ |
| SYJ211+1.010.p | 2.4 ms ✓ | 60 µs ✓ | 42.4 ms ✓ | 10 µs unknown | 234 µs ✓ | 93.4 ms ✗ † | 3.7 ms ✗ | 122 µs ✗ | 121 µs ✗ | 97.7 ms ✗ † | 3.8 ms ✗ | 126 µs ✗ | 122 µs ✗ |
| SYJ211+1.011.p | 5.0 ms ✓ | 62 µs ✓ | 8.2 ms ✓ | 11 µs unknown | 279 µs ✓ | 91.2 ms ✗ † | 5.2 ms ✗ | 139 µs ✗ | 128 µs ✗ | 97.5 ms ✗ † | 5.5 ms ✗ | 137 µs ✗ | 130 µs ✗ |
| SYJ211+1.012.p | 10.7 ms ✓ | 60 µs ✓ | 14.7 ms ✓ | 10 µs unknown | 295 µs ✓ | 90.7 ms ✗ † | 7.0 ms ✗ | 148 µs ✗ | 138 µs ✗ | 94.9 ms ✗ † | 7.9 ms ✗ | 143 µs ✗ | 143 µs ✗ |
| SYJ211+1.013.p | 23.1 ms ✓ | 63 µs ✓ | 15.4 ms ✓ | 11 µs unknown | 341 µs ✓ | 92.6 ms ✗ † | 10.0 ms ✗ | 158 µs ✗ | 149 µs ✗ | 96.0 ms ✗ † | 11.1 ms ✗ | 160 µs ✗ | 159 µs ✗ |
| SYJ211+1.014.p | 51.0 ms ✓ | 66 µs ✓ | 16.9 ms ✓ | 11 µs unknown | 379 µs ✓ | 91.4 ms ✗ † | 13.5 ms ✗ | 173 µs ✗ | 162 µs ✗ | 95.2 ms ✗ † | 15.6 ms ✗ | 179 µs ✗ | 167 µs ✗ |
| SYJ211+1.015.p | 107.9 ms ✓ | 66 µs ✓ | 15.3 ms ✓ | 10 µs unknown | 421 µs ✓ | 91.3 ms ✗ † | 17.8 ms ✗ | 183 µs ✗ | 171 µs ✗ | 94.9 ms ✗ † | 21.1 ms ✗ | 188 µs ✗ | 177 µs ✗ |
| SYJ211+1.016.p | 226.4 ms ✓ | 68 µs ✓ | 15.2 ms ✓ | 12 µs unknown | 469 µs ✓ | 89.5 ms ✗ † | 24.2 ms ✗ | 227 µs ✗ | 185 µs ✗ | 92.3 ms ✗ † | 28.7 ms ✗ | 215 µs ✗ | 186 µs ✗ |
| SYJ211+1.017.p | 499.8 ms ✓ | 71 µs ✓ | 16.0 ms ✓ | 11 µs unknown | 514 µs ✓ | 86.0 ms ✗ † | 32.8 ms ✗ | 215 µs ✗ | 198 µs ✗ | 87.4 ms ✗ † | 37.2 ms ✗ | 213 µs ✗ | 209 µs ✗ |
| SYJ211+1.018.p | 1.06 s ✓ | 80 µs ✓ | 15.4 ms ✓ | 11 µs unknown | 567 µs ✓ | 85.6 ms ✗ | 42.3 ms ✗ | 236 µs ✗ | 213 µs ✗ | 86.5 ms ✗ † | 46.2 ms ✗ | 239 µs ✗ | 222 µs ✗ |
| SYJ211+1.019.p | 1.52 s ? memory | 80 µs ✓ | 15.2 ms ✓ | 12 µs unknown | 628 µs ✓ | 82.0 ms ✗ † | 53.6 ms ✗ | 245 µs ✗ | 227 µs ✗ | 85.0 ms ✗ | 59.7 ms ✗ | 251 µs ✗ | 230 µs ✗ |
| SYJ211+1.020.p | 1.56 s ? memory | 85 µs ✓ | 15.6 ms ✓ | 12 µs unknown | 675 µs ✓ | 81.7 ms ✗ | 69.1 ms ✗ | 258 µs ✗ | 248 µs ✗ | 83.9 ms ✗ † | 73.9 ms ✗ | 269 µs ✗ | 253 µs ✗ |

## SYJ212

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ212+1.001.p | 30 µs ✓ | 27 µs ✓ | 73 µs ✓ | 11 µs ✓ | 29 µs ✓ | 51 µs ✗ | 17 µs ✗ | 40 µs ✗ | 42 µs ✗ | 107 µs ✗ | 14 µs ✗ | 40 µs ✗ | 40 µs ✗ |
| SYJ212+1.002.p | 45 µs ✓ | 34 µs ✓ | 138 µs ✓ | 11 µs ✓ | 70 µs ✓ | 129 µs ✗ | 20 µs ✗ | 50 µs ✗ | 54 µs ✗ | 135 µs ✗ | 18 µs ✗ | 57 µs ✗ | 49 µs ✗ |
| SYJ212+1.003.p | 119 µs ✓ | 47 µs ✓ | 801 µs ✓ | 10 µs ✓ | 250 µs ✓ | 159 µs ✗ | 23 µs ✗ | 83 µs ✗ | 85 µs ✗ | 161 µs ✗ | 23 µs ✗ | 62 µs ✗ | 62 µs ✗ |
| SYJ212+1.004.p | 2.5 ms ✓ | 61 µs ✓ | 27.4 ms ✓ | 17 µs ✓ | 1.1 ms ✓ | 236 µs ✗ | 33 µs ✗ | 112 µs ✗ | 118 µs ✗ | 243 µs ✗ | 27 µs ✗ | 78 µs ✗ | 81 µs ✗ |
| SYJ212+1.005.p | 508.9 ms ✓ | 85 µs ✓ | 15.9 ms ✓ | 15 µs ✓ | 4.4 ms ✓ | 370 µs ✗ | 41 µs ✗ | 269 µs ✗ | 264 µs ✗ | 404 µs ✗ | 35 µs ✗ | 89 µs ✗ | 88 µs ✗ |
| SYJ212+1.006.p | 1.90 s ? memory | 103 µs ✓ | 15.0 ms ✓ | 15 µs ✓ | 23.5 ms ✓ | 633 µs ✗ | 52 µs ✗ | 673 µs ✗ | 670 µs ✗ | 629 µs ✗ | 37 µs ✗ | 97 µs ✗ | 95 µs ✗ |
| SYJ212+1.007.p | 1.86 s ? memory | 125 µs ✓ | 13.4 ms ✓ | 18 µs ✓ | 67.2 ms unknown | 2.2 ms ✗ | 58 µs ✗ | 306 µs ✗ | 305 µs ✗ | 2.0 ms ✗ | 45 µs ✗ | 104 µs ✗ | 104 µs ✗ |
| SYJ212+1.008.p | 1.39 s ? memory | 164 µs ✓ | 10.8 ms ✓ | 25 µs ✓ | 420.6 ms unknown | 4.3 ms ✗ | 70 µs ✗ | 253 µs ✗ | 251 µs ✗ | 3.9 ms ✗ | 50 µs ✗ | 117 µs ✗ | 122 µs ✗ |
| SYJ212+1.009.p | 1.52 s ? memory | 186 µs ✓ | 7.2 ms ✓ | 38 µs ✓ | 360.9 ms ? memory | 8.7 ms ✗ | 78 µs ✗ | 262 µs ✗ | 262 µs ✗ | 7.9 ms ✗ | 56 µs ✗ | 129 µs ✗ | 127 µs ✗ |
| SYJ212+1.010.p | 1.71 s ? memory | 223 µs ✓ | 5.2 ms ✓ | 71 µs ✓ | 316.5 ms ? memory | 21.4 ms ✗ † | 96 µs ✗ | 274 µs ✗ | 272 µs ✗ | 18.8 ms ✗ † | 62 µs ✗ | 141 µs ✗ | 140 µs ✗ |
| SYJ212+1.011.p | 1.87 s ? memory | 254 µs ✓ | 4.3 ms ✓ | 9 µs unknown | 302.2 ms ? memory | 65.9 ms ✗ | 105 µs ✗ | 317 µs ✗ | 300 µs ✗ | 60.0 ms ✗ | 69 µs ✗ | 165 µs ✗ | 147 µs ✗ |
| SYJ212+1.012.p | 2.00 s ? memory | 272 µs ✓ | 5.6 ms ✓ | 11 µs unknown | 284.3 ms ? memory | 250.0 ms ✗ † | 116 µs ✗ | 351 µs ✗ | 347 µs ✗ | 234.8 ms ✗ | 80 µs ✗ | 164 µs ✗ | 170 µs ✗ |
| SYJ212+1.013.p | > 2 s | 331 µs ✓ | 10.4 ms ✓ | 11 µs unknown | 273.5 ms ? memory | 632.9 ms ✗ | 137 µs ✗ | 365 µs ✗ | 334 µs ✗ | 589.8 ms ✗ | 89 µs ✗ | 178 µs ✗ | 182 µs ✗ |
| SYJ212+1.014.p | > 2 s | 351 µs ✓ | 20.2 ms ✓ | 10 µs unknown | 258.1 ms ? memory | 1.50 s ✗ | 145 µs ✗ | 397 µs ✗ | 399 µs ✗ | 1.39 s ✗ | 90 µs ✗ | 191 µs ✗ | 196 µs ✗ |
| SYJ212+1.015.p | > 2 s | 515 µs ✓ | 38.9 ms ✓ | 10 µs unknown | 251.5 ms ? memory | > 2 s | 155 µs ✗ | 410 µs ✗ | 410 µs ✗ | 2.05 s ✗ | 96 µs ✗ | 210 µs ✗ | 209 µs ✗ |
| SYJ212+1.016.p | > 2 s | 468 µs ✓ | 77.8 ms ✓ | 12 µs unknown | 231.0 ms ? memory | > 2 s | 169 µs ✗ | 113.1 ms ✗ | 107.7 ms ✗ | 2.08 s ✗ | 104 µs ✗ | 223 µs ✗ | 220 µs ✗ |
| SYJ212+1.017.p | > 2 s | 427 µs ✓ | 459 µs ✓ | 10 µs unknown | 230.2 ms ? memory | > 2 s | 183 µs ✗ | 402 µs ✗ | 400 µs ✗ | > 2 s | 114 µs ✗ | 241 µs ✗ | 236 µs ✗ |
| SYJ212+1.018.p | > 2 s | 444 µs ✓ | 478 µs ✓ | 9 µs unknown | 209.1 ms ? memory | > 2 s | 207 µs ✗ | 470 µs ✗ | 456 µs ✗ | > 2 s | 126 µs ✗ | 264 µs ✗ | 257 µs ✗ |
| SYJ212+1.019.p | > 2 s | 619 µs ✓ | 660 µs ✓ | 9 µs unknown | 213.1 ms ? memory | > 2 s | 229 µs ✗ | 466 µs ✗ | 514 µs ✗ | > 2 s | 136 µs ✗ | 273 µs ✗ | 274 µs ✗ |
| SYJ212+1.020.p | > 2 s | 585 µs ✓ | 610 µs ✓ | 10 µs unknown | 193.7 ms ? memory | 652.3 ms ? memory | 264 µs ✗ | 489 µs ✗ | 484 µs ✗ | 651.7 ms ? memory | 143 µs ✗ | 304 µs ✗ | 305 µs ✗ |

## Problems/SYN

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYN001+1.p | 44 µs ✓ | 30 µs ✓ | 97 µs ✓ | 15 µs ✓ | 36 µs ✓ | 47 µs ✗ | 17 µs ✗ | 41 µs ✗ | 42 µs ✗ | 106 µs ✗ | 15 µs ✗ | 39 µs ✗ | 43 µs ✗ |
| SYN040+1.p | 41 µs ✓ | 34 µs ✓ | 96 µs ✓ | 11 µs ✓ | 43 µs ✓ | 152 µs ✗ | 20 µs ✗ | 49 µs ✗ | 49 µs ✗ | 113 µs ✗ | 20 µs ✗ | 48 µs ✗ | 46 µs ✗ |
| SYN041+1.p | 31 µs ✓ | 28 µs ✓ | 88 µs ✓ | 11 µs ✓ | 34 µs ✓ | 40 µs ✓ | 31 µs ✓ | 42 µs ✓ | 60 µs ✓ | 149 µs ✗ | 19 µs ✗ | 40 µs ✗ | 40 µs ✗ |
| SYN044+1.p | 49 µs ✓ | 30 µs ✓ | 171 µs ✓ | 12 µs ✓ | 65 µs ✓ | 122 µs ✓ | 34 µs ✓ | 45 µs ✓ | 69 µs ✓ | 127 µs ✓ | 31 µs ✓ | 45 µs ✓ | 70 µs ✓ |
| SYN045+1.p | 40 µs ✓ | 34 µs ✓ | 120 µs ✓ | 11 µs ✓ | 51 µs ✓ | 89 µs ✓ | 37 µs ✓ | 52 µs ✓ | 85 µs ✓ | 86 µs ✓ | 39 µs ✓ | 58 µs ✓ | 86 µs ✓ |
| SYN046+1.p | 35 µs ✓ | 30 µs ✓ | 98 µs ✓ | 14 µs ✓ | 39 µs ✓ | 103 µs ✗ | 16 µs ✗ | 46 µs ✗ | 40 µs ✗ | 109 µs ✗ | 16 µs ✗ | 45 µs ✗ | 45 µs ✗ |
| SYN047+1.p | 54 µs ✓ | 45 µs ✓ | 186 µs ✓ | 12 µs ✓ | 78 µs ✓ | 115 µs ✗ | 20 µs ✗ | 68 µs ✗ | 69 µs ✗ | 136 µs ✗ | 18 µs ✗ | 59 µs ✗ | 59 µs ✗ |
| SYN387+1.p | 30 µs ✓ | 24 µs ✓ | 66 µs ✓ | 12 µs ✓ | 22 µs ✓ | 36 µs ✗ | 14 µs ✗ | 35 µs ✗ | 36 µs ✗ | 36 µs ✗ | 12 µs ✗ | 33 µs ✗ | 34 µs ✗ |
| SYN388+1.p | 29 µs ✓ | 29 µs ✓ | 79 µs ✓ | 12 µs ✓ | 28 µs ✓ | 47 µs ✗ | 15 µs ✗ | 38 µs ✗ | 36 µs ✗ | 46 µs ✗ | 15 µs ✗ | 41 µs ✗ | 40 µs ✗ |
| SYN389+1.p | 31 µs ✓ | 28 µs ✓ | 77 µs ✓ | 12 µs ✓ | 28 µs ✓ | 96 µs ✗ | 15 µs ✗ | 42 µs ✗ | 39 µs ✗ | 101 µs ✗ | 14 µs ✗ | 40 µs ✗ | 40 µs ✗ |
| SYN390+1.p | 32 µs ✓ | 22 µs ✓ | 72 µs ✓ | 10 µs ✓ | 28 µs ✓ | 35 µs ✓ | 20 µs ✓ | 31 µs ✓ | 40 µs ✓ | 33 µs ✓ | 19 µs ✓ | 32 µs ✓ | 42 µs ✓ |
| SYN391+1.p | 43 µs ✓ | 29 µs ✓ | 125 µs ✓ | 11 µs ✓ | 57 µs ✓ | 255 µs ✓ | 47 µs ✓ | 62 µs ✓ | 97 µs ✓ | 279 µs ✓ | 47 µs ✓ | 63 µs ✓ | 98 µs ✓ |
| SYN392+1.p | 45 µs ✓ | 39 µs ✓ | 141 µs ✓ | 11 µs ✓ | 74 µs ✓ | 118 µs ✗ | 19 µs ✗ | 70 µs ✗ | 67 µs ✗ | 117 µs ✗ | 18 µs ✗ | 52 µs ✗ | 54 µs ✗ |
| SYN393+1.p | 124 µs ✓ | 40 µs ✓ | 769 µs ✓ | 10 µs ✓ | 225 µs ✓ | 161 µs ✗ | 31 µs ✗ | 123 µs ✗ | 122 µs ✗ | 147 µs ✗ | 30 µs ✗ | 109 µs ✗ | 104 µs ✗ |
| SYN416+1.p | 32 µs ✓ | 27 µs ✓ | 76 µs ✓ | 13 µs ✓ | 25 µs ✓ | 38 µs ✗ | 14 µs ✗ | 39 µs ✗ | 37 µs ✗ | 36 µs ✗ | 13 µs ✗ | 38 µs ✗ | 37 µs ✗ |
| SYN915+1.p | 28 µs ✓ | 25 µs ✓ | 60 µs ✓ | 14 µs ✓ | 17 µs ✓ | 24 µs ✓ | 16 µs ✓ | 23 µs ✓ | 30 µs ✓ | 21 µs ✓ | 14 µs ✓ | 24 µs ✓ | 31 µs ✓ |
| SYN916+1.p | 31 µs ✗ | 33 µs ✗ | 58 µs ✗ | 24 µs ✗ | 25 µs ✗ | 29 µs ✗ | 10 µs ✗ | 31 µs ✗ | 31 µs ✗ | 28 µs ✗ | 11 µs ✗ | 29 µs ✗ | 31 µs ✗ |
| SYN977+1.p | 30 µs ✓ | 25 µs ✓ | 79 µs ✓ | 10 µs ✓ | 29 µs ✓ | 40 µs ✗ | 15 µs ✗ | 36 µs ✗ | 41 µs ✗ | 41 µs ✗ | 17 µs ✗ | 38 µs ✗ | 40 µs ✗ |
| SYN978+1.p | 34 µs ✓ | 29 µs ✓ | 92 µs ✓ | 13 µs ✓ | 37 µs ✓ | 37 µs ✓ | 24 µs ✓ | 35 µs ✓ | 59 µs ✓ | 38 µs ✓ | 27 µs ✓ | 39 µs ✓ | 54 µs ✓ |

## SYN007

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYN007+1.014.p | 0 µs error | 624 µs ✓ | 658 µs ✓ | 11 µs unknown | 229.5 ms ? memory | 0 µs error | 187 µs ✗ | 372 µs ✗ | 359 µs ✗ | 0 µs error | 187 µs ✗ | 371 µs ✗ | 363 µs ✗ |
