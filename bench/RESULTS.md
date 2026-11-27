# Benchmark results

The baseline of 2026-11-27 on an Intel(R) Core(TM) Ultra X9 388H
(16 cores, 62 GB), release build, from
`bench/baseline.sh`, which writes this file, the CSV files beside it
(`bench/results/2026-11-27/`, one row per run) and a copy of this file as
`bench/RESULTS.md`, the latest baseline's. It started (once more for
every resumption), with the commit it measured, the LLTP problems of its
later stages chosen from `bench/results/2026-09-30/` and its reruns from
`bench/reruns.txt`:

- 2026-11-27 01:19, commit 9bcfe3743859 (Plan: unattended from 01:05), load average 1.00, sequential streams on CPUs 1 0 2 3

Its last part took 2 h 23 min, and the scheduled jobs that ran
meanwhile were: none. The package throttled
65452 times for 573 s in that part
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
summary bench/results/2026-11-27/*.csv` prints the tables again.

## Solved within the time limit

Sequential problems that waited for a CPU for over 1 % of their time (another process slowed them down; `†` in the tables below): 144.

| family | configuration | engines | problems | solved | proved | refuted | late | timeout | bound | other | mismatch | median solved | total solved |
|---|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| partition-yes | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 155 µs | 433.8 ms |
| partition-yes | engines: classical net j1 | net | 5 | 3 | 3 | 0 | 0 | 2 | 0 | 0 | 0 | 28.8 ms | 504.6 ms |
| partition-no | engines: classical focus j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 81.6 ms | 3.68 s |
| partition-no | engines: classical net j1 | net | 4 | 2 | 0 | 2 | 0 | 2 | 0 | 0 | 0 | 9.5 ms | 11.0 ms |
| 3-partition-mll-yes | engines: classical focus j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 55 µs | 222 µs |
| 3-partition-mll-yes | engines: classical net j1 | net | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 141 µs | 546 µs |
| 3-partition-mll-no | engines: classical focus j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 43 µs | 172 µs |
| 3-partition-mll-no | engines: classical net j1 | net | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 5.3 ms |
| wide-m1 | engines: classical focus j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 122 µs | 215.9 ms |
| wide-m1 | engines: classical net j1 | net | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 218 µs | 621.9 ms |
| wide-m2 | engines: classical focus j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 115 µs | 206.5 ms |
| wide-m2 | engines: classical net j1 | net | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 219 µs | 615.1 ms |
| wide-m3 | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 105 µs | 182.8 ms |
| wide-m3 | engines: classical net j1 | net | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 242 µs | 644.7 ms |
| wide-m4 | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 106 µs | 170.9 ms |
| wide-m4 | engines: classical net j1 | net | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 294 µs | 658.1 ms |
| additive | engines: classical focus j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 346.5 ms | 5.87 s |
| partition-table | engines: classical focus j1 | focus | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 83 µs | 2.3 ms |
| partition-table | engines: classical net j1 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.3 ms | 5.32 s |
| cancellation | engines: classical focus j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 308 µs | 308 µs |
| 3-partition-yes | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 67 µs | 272 µs |
| 3-partition-no | families: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 342 µs | 634 µs |
| 3-partition-mll-yes | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 63 µs | 247 µs |
| 3-partition-mll-no | families: classical auto j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 51 µs | 199 µs |
| partition-yes | families: classical auto j1 | focus | 9 | 8 | 8 | 0 | 0 | 1 | 0 | 0 | 0 | 12.8 ms | 72.49 s |
| partition-no | families: classical auto j1 | focus | 7 | 5 | 0 | 5 | 0 | 2 | 0 | 0 | 0 | 444 µs | 3.67 s |
| qbf | families: classical auto j1 | focus | 36 | 34 | 6 | 28 | 0 | 0 | 0 | 2 | 0 | 122.9 ms | 686.68 s |
| wide-m1 | families: classical auto j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 119 µs | 204.3 ms |
| wide-m2 | families: classical auto j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 109 µs | 189.1 ms |
| wide-m3 | families: classical auto j1 | focus | 7 | 7 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 121 µs | 206.8 ms |
| wide-m4 | families: classical auto j1 | focus | 8 | 8 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 118 µs | 192.4 ms |
| mix | families: mix auto j1 | focus, inverse | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 255.2 ms | 1.04 s |
| counter | families: classical auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 39 µs | 248 µs |
| counter-over | families: classical auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 45 µs | 252 µs |
| growing | families: classical auto j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 18.4 ms | 130.5 ms |
| chain | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 32.6 ms | 310.5 ms |
| additive | families: classical auto j1 | additive | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 58.6 ms | 251.3 ms |
| lambek-fo | families: intuitionistic auto j1 | two-sided | 20 | 18 | 10 | 8 | 0 | 0 | 0 | 2 | 0 | 4.0 ms | 548.6 ms |
| chain-fo | families: classical auto j1 | focus | 8 | 3 | 3 | 0 | 0 | 0 | 3 | 2 | 0 | 7.4 ms | 316.5 ms |
| witness-fo | families: intuitionistic auto j1 | two-sided | 16 | 15 | 8 | 7 | 0 | 1 | 0 | 0 | 0 | 67 µs | 2.55 s |
| contraction | families: intuitionistic auto j1 | inverse, two-sided | 6 | 4 | 3 | 1 | 0 | 0 | 2 | 0 | 0 | 64.7 ms | 139.7 ms |
| pigeonhole | families: intuitionistic-affine auto j1 | two-sided | 6 | 6 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 282 µs | 39.4 ms |
| euler | families: intuitionistic auto j1 | two-sided | 6 | 6 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 153 µs | 2.3 ms |
| duplicate | families: intuitionistic auto j1 | horn | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 32 µs | 94 µs |
| duplicate | families: classical auto j1 | horn | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 33 µs | 97 µs |
| reach-fo | families: intuitionistic auto j1 | two-sided | 4 | 2 | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 1.3 ms | 1.4 ms |
| Problems/LCL | iltp-classical: classical embedding j1 | focus | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 47 µs | 89 µs |
| Problems/LCL | iltp-classical: classical sat j1 | sat | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 33 µs | 62 µs |
| Problems/LCL | iltp-classical: classical auto j1 | focus | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 99 µs | 191 µs |
| Problems/LCL | iltp-classical: classical table j1 | table | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 13 µs | 25 µs |
| Problems/LCL | iltp-classical: classical tableau j1 | tableau | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 41 µs | 80 µs |
| Problems/SYJ | iltp-classical: classical embedding j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 37 µs | 196 µs |
| Problems/SYJ | iltp-classical: classical sat j1 | sat | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 25 µs | 131 µs |
| Problems/SYJ | iltp-classical: classical auto j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 72 µs | 448 µs |
| Problems/SYJ | iltp-classical: classical table j1 | table | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 13 µs | 66 µs |
| Problems/SYJ | iltp-classical: classical tableau j1 | tableau | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 26 µs | 181 µs |
| SYJ105 | iltp-classical: classical embedding j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 39 µs | 116 µs |
| SYJ105 | iltp-classical: classical sat j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 29 µs | 93 µs |
| SYJ105 | iltp-classical: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 80 µs | 253 µs |
| SYJ105 | iltp-classical: classical table j1 | table | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 12 µs | 38 µs |
| SYJ105 | iltp-classical: classical tableau j1 | tableau | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 34 µs | 114 µs |
| SYJ107 | iltp-classical: classical embedding j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 176 µs | 1.5 ms |
| SYJ107 | iltp-classical: classical sat j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 38 µs | 152 µs |
| SYJ107 | iltp-classical: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 1.6 ms | 16.9 ms |
| SYJ107 | iltp-classical: classical table j1 | table | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 21 µs | 99 µs |
| SYJ107 | iltp-classical: classical tableau j1 | tableau | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 344 µs | 2.1 ms |
| SYJ201 | iltp-classical: classical embedding j1 | focus | 20 | 4 | 4 | 0 | 0 | 3 | 0 | 13 | 0 | 14.0 ms | 246.6 ms |
| SYJ201 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 93 µs | 1.7 ms |
| SYJ201 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 17.6 ms | 329.4 ms |
| SYJ201 | iltp-classical: classical table j1 | table | 20 | 4 | 4 | 0 | 0 | 0 | 0 | 16 | 0 | 31 µs | 150 µs |
| SYJ201 | iltp-classical: classical tableau j1 | tableau | 20 | 12 | 12 | 0 | 0 | 0 | 0 | 8 | 0 | 5.9 ms | 106.1 ms |
| SYJ202 | iltp-classical: classical embedding j1 | focus | 20 | 2 | 2 | 0 | 0 | 2 | 0 | 16 | 0 | 286 µs | 323 µs |
| SYJ202 | iltp-classical: classical sat j1 | sat | 20 | 10 | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 2.2 ms | 1.53 s |
| SYJ202 | iltp-classical: classical auto j1 | focus, sat | 20 | 10 | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 110.7 ms | 2.17 s |
| SYJ202 | iltp-classical: classical table j1 | table | 20 | 2 | 2 | 0 | 0 | 0 | 0 | 18 | 0 | 19 µs | 33 µs |
| SYJ202 | iltp-classical: classical tableau j1 | tableau | 20 | 2 | 2 | 0 | 0 | 0 | 0 | 18 | 0 | 291 µs | 322 µs |
| SYJ203 | iltp-classical: classical embedding j1 | focus | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 58 µs | 1.2 ms |
| SYJ203 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 48 µs | 986 µs |
| SYJ203 | iltp-classical: classical auto j1 | focus | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 423 µs | 8.7 ms |
| SYJ203 | iltp-classical: classical table j1 | table | 20 | 9 | 9 | 0 | 0 | 0 | 0 | 11 | 0 | 20 µs | 365 µs |
| SYJ203 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 225 µs | 5.4 ms |
| SYJ204 | iltp-classical: classical embedding j1 | focus | 20 | 13 | 13 | 0 | 0 | 0 | 0 | 7 | 0 | 830 µs | 1.44 s |
| SYJ204 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 46 µs | 873 µs |
| SYJ204 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 15.1 ms | 232.6 ms |
| SYJ204 | iltp-classical: classical table j1 | table | 20 | 9 | 9 | 0 | 0 | 0 | 0 | 11 | 0 | 15 µs | 262 µs |
| SYJ204 | iltp-classical: classical tableau j1 | tableau | 20 | 12 | 12 | 0 | 0 | 0 | 0 | 8 | 0 | 1.1 ms | 113.6 ms |
| SYJ205 | iltp-classical: classical embedding j1 | focus | 20 | 17 | 17 | 0 | 0 | 0 | 0 | 3 | 0 | 2.5 ms | 2.27 s |
| SYJ205 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 84 µs | 1.6 ms |
| SYJ205 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 14.8 ms | 235.9 ms |
| SYJ205 | iltp-classical: classical table j1 | table | 20 | 3 | 3 | 0 | 0 | 0 | 0 | 17 | 0 | 25 µs | 101 µs |
| SYJ205 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 1.4 ms | 37.8 ms |
| SYJ206 | iltp-classical: classical embedding j1 | focus | 20 | 5 | 5 | 0 | 0 | 8 | 0 | 7 | 0 | 116 µs | 524.3 ms |
| SYJ206 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 291 µs | 5.3 ms |
| SYJ206 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 7.6 ms | 269.1 ms |
| SYJ206 | iltp-classical: classical table j1 | table | 20 | 10 | 10 | 0 | 0 | 0 | 0 | 10 | 0 | 18 µs | 242 µs |
| SYJ206 | iltp-classical: classical tableau j1 | tableau | 20 | 6 | 6 | 0 | 0 | 0 | 0 | 14 | 0 | 952 µs | 28.4 ms |
| SYJ207 | iltp-classical: classical embedding j1 | focus | 20 | 5 | 5 | 0 | 0 | 3 | 0 | 12 | 0 | 3.7 ms | 1.05 s |
| SYJ207 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 77 µs | 1.6 ms |
| SYJ207 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 18.0 ms | 319.0 ms |
| SYJ207 | iltp-classical: classical table j1 | table | 20 | 4 | 4 | 0 | 0 | 0 | 0 | 16 | 0 | 26 µs | 135 µs |
| SYJ207 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 54 µs | 1.1 ms |
| SYJ208 | iltp-classical: classical embedding j1 | focus | 20 | 2 | 2 | 0 | 0 | 2 | 0 | 16 | 0 | 283 µs | 320 µs |
| SYJ208 | iltp-classical: classical sat j1 | sat | 20 | 10 | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 2.3 ms | 1.61 s |
| SYJ208 | iltp-classical: classical auto j1 | focus, sat | 20 | 10 | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 111.3 ms | 2.26 s |
| SYJ208 | iltp-classical: classical table j1 | table | 20 | 2 | 2 | 0 | 0 | 0 | 0 | 18 | 0 | 19 µs | 34 µs |
| SYJ208 | iltp-classical: classical tableau j1 | tableau | 20 | 2 | 2 | 0 | 0 | 0 | 0 | 18 | 0 | 302 µs | 331 µs |
| SYJ209 | iltp-classical: classical embedding j1 | focus | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 64 µs | 1.3 ms |
| SYJ209 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 53 µs | 1.1 ms |
| SYJ209 | iltp-classical: classical auto j1 | focus | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 375 µs | 8.6 ms |
| SYJ209 | iltp-classical: classical table j1 | table | 20 | 9 | 9 | 0 | 0 | 0 | 0 | 11 | 0 | 23 µs | 397 µs |
| SYJ209 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 235 µs | 5.6 ms |
| SYJ210 | iltp-classical: classical embedding j1 | focus | 20 | 13 | 13 | 0 | 0 | 0 | 0 | 7 | 0 | 783 µs | 1.43 s |
| SYJ210 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 48 µs | 916 µs |
| SYJ210 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 15.3 ms | 234.4 ms |
| SYJ210 | iltp-classical: classical table j1 | table | 20 | 9 | 9 | 0 | 0 | 0 | 0 | 11 | 0 | 16 µs | 288 µs |
| SYJ210 | iltp-classical: classical tableau j1 | tableau | 20 | 12 | 12 | 0 | 0 | 0 | 0 | 8 | 0 | 1.1 ms | 112.9 ms |
| SYJ211 | iltp-classical: classical embedding j1 | focus | 20 | 18 | 18 | 0 | 0 | 0 | 0 | 2 | 0 | 2.5 ms | 1.99 s |
| SYJ211 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 62 µs | 1.2 ms |
| SYJ211 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 15.2 ms | 232.8 ms |
| SYJ211 | iltp-classical: classical table j1 | table | 20 | 3 | 3 | 0 | 0 | 0 | 0 | 17 | 0 | 21 µs | 83 µs |
| SYJ211 | iltp-classical: classical tableau j1 | tableau | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 285 µs | 6.1 ms |
| SYJ212 | iltp-classical: classical embedding j1 | focus | 20 | 5 | 5 | 0 | 0 | 8 | 0 | 7 | 0 | 122 µs | 525.4 ms |
| SYJ212 | iltp-classical: classical sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 251 µs | 5.5 ms |
| SYJ212 | iltp-classical: classical auto j1 | focus, sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 8.0 ms | 265.3 ms |
| SYJ212 | iltp-classical: classical table j1 | table | 20 | 10 | 10 | 0 | 0 | 0 | 0 | 10 | 0 | 19 µs | 261 µs |
| SYJ212 | iltp-classical: classical tableau j1 | tableau | 20 | 6 | 6 | 0 | 0 | 0 | 0 | 14 | 0 | 1.0 ms | 29.9 ms |
| Problems/SYN | iltp-classical: classical embedding j1 | focus | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 43 µs | 908 µs |
| Problems/SYN | iltp-classical: classical sat j1 | sat | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 32 µs | 647 µs |
| Problems/SYN | iltp-classical: classical auto j1 | focus | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 89 µs | 2.7 ms |
| Problems/SYN | iltp-classical: classical table j1 | table | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 14 µs | 275 µs |
| Problems/SYN | iltp-classical: classical tableau j1 | tableau | 19 | 19 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 39 µs | 1.0 ms |
| SYN007 | iltp-classical: classical embedding j1 |  | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| SYN007 | iltp-classical: classical sat j1 | sat | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 620 µs | 620 µs |
| SYN007 | iltp-classical: classical auto j1 | sat | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 659 µs | 659 µs |
| SYN007 | iltp-classical: classical table j1 | table | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| SYN007 | iltp-classical: classical tableau j1 | tableau | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| Problems/LCL | iltp: intuitionistic embedding j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 110 µs | 170 µs |
| Problems/LCL | iltp: intuitionistic ljt j1 | ljt | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 20 µs | 38 µs |
| Problems/LCL | iltp: intuitionistic sat j1 | sat | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 46 µs | 86 µs |
| Problems/LCL | iltp: intuitionistic auto j1 | sat | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 44 µs | 86 µs |
| Problems/LCL | iltp: minimal embedding j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 114 µs | 175 µs |
| Problems/LCL | iltp: minimal ljt j1 | ljt | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 18 µs | 36 µs |
| Problems/LCL | iltp: minimal sat j1 | sat | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 49 µs | 89 µs |
| Problems/LCL | iltp: minimal auto j1 | sat | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 52 µs | 93 µs |
| Problems/SYJ | iltp: intuitionistic embedding j1 | horn, two-sided | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 73 µs | 396 µs |
| Problems/SYJ | iltp: intuitionistic ljt j1 | ljt | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 21 µs | 124 µs |
| Problems/SYJ | iltp: intuitionistic sat j1 | sat | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 31 µs | 208 µs |
| Problems/SYJ | iltp: intuitionistic auto j1 | sat | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 41 µs | 286 µs |
| Problems/SYJ | iltp: minimal embedding j1 | horn, two-sided | 5 | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 107 µs | 552 µs |
| Problems/SYJ | iltp: minimal ljt j1 | ljt | 5 | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 21 µs | 117 µs |
| Problems/SYJ | iltp: minimal sat j1 | sat | 5 | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 33 µs | 174 µs |
| Problems/SYJ | iltp: minimal auto j1 | sat | 5 | 5 | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 45 µs | 221 µs |
| SYJ105 | iltp: intuitionistic embedding j1 | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 85 µs | 294 µs |
| SYJ105 | iltp: intuitionistic ljt j1 | ljt | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 52 µs | 145 µs |
| SYJ105 | iltp: intuitionistic sat j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 70 µs | 225 µs |
| SYJ105 | iltp: intuitionistic auto j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 104 µs | 322 µs |
| SYJ105 | iltp: minimal embedding j1 | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 85 µs | 285 µs |
| SYJ105 | iltp: minimal ljt j1 | ljt | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 49 µs | 144 µs |
| SYJ105 | iltp: minimal sat j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 77 µs | 238 µs |
| SYJ105 | iltp: minimal auto j1 | sat | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 123 µs | 349 µs |
| SYJ107 | iltp: intuitionistic embedding j1 | two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 2.3 ms | 24.6 ms |
| SYJ107 | iltp: intuitionistic ljt j1 | ljt | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 73 µs | 322 µs |
| SYJ107 | iltp: intuitionistic sat j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 95 µs | 383 µs |
| SYJ107 | iltp: intuitionistic auto j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 178 µs | 679 µs |
| SYJ107 | iltp: minimal embedding j1 | two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 2.3 ms | 25.0 ms |
| SYJ107 | iltp: minimal ljt j1 | ljt | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 67 µs | 318 µs |
| SYJ107 | iltp: minimal sat j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 95 µs | 375 µs |
| SYJ107 | iltp: minimal auto j1 | sat | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 169 µs | 685 µs |
| SYJ201 | iltp: intuitionistic embedding j1 | two-sided | 20 | 5 | 5 | 0 | 0 | 15 | 0 | 0 | 0 | 5.5 ms | 900.2 ms |
| SYJ201 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 87.0 ms | 3.48 s |
| SYJ201 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 889 µs | 20.5 ms |
| SYJ201 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 9.9 ms | 304.2 ms |
| SYJ201 | iltp: minimal embedding j1 | two-sided | 20 | 5 | 5 | 0 | 0 | 15 | 0 | 0 | 0 | 5.5 ms | 900.7 ms |
| SYJ201 | iltp: minimal ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 87.8 ms | 3.48 s |
| SYJ201 | iltp: minimal sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 916 µs | 20.3 ms |
| SYJ201 | iltp: minimal auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 9.8 ms | 304.0 ms |
| SYJ202 | iltp: intuitionistic embedding j1 | two-sided | 20 | 3 | 3 | 0 | 0 | 6 | 0 | 11 | 0 | 392 µs | 83.9 ms |
| SYJ202 | iltp: intuitionistic ljt j1 | ljt | 20 | 4 | 4 | 0 | 0 | 3 | 0 | 13 | 0 | 754 µs | 42.4 ms |
| SYJ202 | iltp: intuitionistic sat j1 | sat | 20 | 6 | 6 | 0 | 0 | 14 | 0 | 0 | 0 | 2.3 ms | 357.1 ms |
| SYJ202 | iltp: intuitionistic auto j1 | sat | 20 | 6 | 6 | 0 | 0 | 14 | 0 | 0 | 0 | 11.1 ms | 419.6 ms |
| SYJ202 | iltp: minimal embedding j1 | two-sided | 20 | 3 | 3 | 0 | 0 | 6 | 0 | 11 | 0 | 386 µs | 83.7 ms |
| SYJ202 | iltp: minimal ljt j1 | ljt | 20 | 4 | 4 | 0 | 0 | 3 | 0 | 13 | 0 | 805 µs | 42.6 ms |
| SYJ202 | iltp: minimal sat j1 | sat | 20 | 6 | 6 | 0 | 0 | 14 | 0 | 0 | 0 | 2.3 ms | 329.2 ms |
| SYJ202 | iltp: minimal auto j1 | sat | 20 | 6 | 6 | 0 | 0 | 14 | 0 | 0 | 0 | 11.2 ms | 462.4 ms |
| SYJ203 | iltp: intuitionistic embedding j1 | inverse, two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 130.3 ms | 1.77 s |
| SYJ203 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 237 µs | 4.9 ms |
| SYJ203 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 208 µs | 4.3 ms |
| SYJ203 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 480 µs | 9.6 ms |
| SYJ203 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 133.4 ms | 1.79 s |
| SYJ203 | iltp: minimal ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 235 µs | 4.8 ms |
| SYJ203 | iltp: minimal sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 233 µs | 4.5 ms |
| SYJ203 | iltp: minimal auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 466 µs | 9.8 ms |
| SYJ204 | iltp: intuitionistic embedding j1 | two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 215 µs | 6.2 ms |
| SYJ204 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 58 µs | 1.1 ms |
| SYJ204 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 52 µs | 1.0 ms |
| SYJ204 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 98 µs | 2.0 ms |
| SYJ204 | iltp: minimal embedding j1 | two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 208 µs | 6.1 ms |
| SYJ204 | iltp: minimal ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 56 µs | 1.1 ms |
| SYJ204 | iltp: minimal sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 54 µs | 1.1 ms |
| SYJ204 | iltp: minimal auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 98 µs | 2.0 ms |
| SYJ205 | iltp: intuitionistic embedding j1 | inverse, two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 122.6 ms | 1.69 s |
| SYJ205 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 468 µs | 10.8 ms |
| SYJ205 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 147 µs | 2.9 ms |
| SYJ205 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 616 µs | 14.0 ms |
| SYJ205 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 120.5 ms | 1.68 s |
| SYJ205 | iltp: minimal ljt j1 | ljt | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 462 µs | 10.8 ms |
| SYJ205 | iltp: minimal sat j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 154 µs | 2.9 ms |
| SYJ205 | iltp: minimal auto j1 | sat | 20 | 20 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 613 µs | 14.2 ms |
| SYJ206 | iltp: intuitionistic embedding j1 | two-sided | 20 | 7 | 7 | 0 | 0 | 12 | 0 | 1 | 0 | 325 µs | 577.7 ms |
| SYJ206 | iltp: intuitionistic ljt j1 | ljt | 20 | 16 | 16 | 0 | 0 | 3 | 0 | 1 | 0 | 6.7 ms | 3.32 s |
| SYJ206 | iltp: intuitionistic sat j1 | sat | 20 | 15 | 15 | 0 | 0 | 5 | 0 | 0 | 0 | 6.0 ms | 2.44 s |
| SYJ206 | iltp: intuitionistic auto j1 | sat | 20 | 15 | 15 | 0 | 0 | 5 | 0 | 0 | 0 | 9.8 ms | 2.56 s |
| SYJ206 | iltp: minimal embedding j1 | two-sided | 20 | 7 | 7 | 0 | 0 | 12 | 0 | 1 | 0 | 311 µs | 603.0 ms |
| SYJ206 | iltp: minimal ljt j1 | ljt | 20 | 16 | 16 | 0 | 0 | 3 | 0 | 1 | 0 | 6.6 ms | 3.32 s |
| SYJ206 | iltp: minimal sat j1 | sat | 20 | 15 | 15 | 0 | 0 | 5 | 0 | 0 | 0 | 7.1 ms | 2.03 s |
| SYJ206 | iltp: minimal auto j1 | sat | 20 | 15 | 15 | 0 | 0 | 5 | 0 | 0 | 0 | 10.9 ms | 2.15 s |
| SYJ207 | iltp: intuitionistic embedding j1 | two-sided | 20 | 3 | 0 | 3 | 0 | 17 | 0 | 0 | 0 | 5.1 ms | 484.9 ms |
| SYJ207 | iltp: intuitionistic ljt j1 | ljt | 20 | 5 | 0 | 5 | 0 | 15 | 0 | 0 | 0 | 3.8 ms | 822.6 ms |
| SYJ207 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 217 µs | 4.1 ms |
| SYJ207 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 207 µs | 4.1 ms |
| SYJ207 | iltp: minimal embedding j1 | two-sided | 20 | 3 | 0 | 3 | 0 | 17 | 0 | 0 | 0 | 5.2 ms | 493.4 ms |
| SYJ207 | iltp: minimal ljt j1 | ljt | 20 | 5 | 0 | 5 | 0 | 15 | 0 | 0 | 0 | 3.8 ms | 822.6 ms |
| SYJ207 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 199 µs | 4.1 ms |
| SYJ207 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 199 µs | 4.1 ms |
| SYJ208 | iltp: intuitionistic embedding j1 | two-sided | 20 | 3 | 0 | 3 | 0 | 6 | 0 | 11 | 0 | 1.3 ms | 345.4 ms |
| SYJ208 | iltp: intuitionistic ljt j1 | ljt | 20 | 6 | 0 | 6 | 0 | 4 | 0 | 10 | 0 | 667 µs | 434.0 ms |
| SYJ208 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 858 µs | 30.9 ms |
| SYJ208 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 828 µs | 31.3 ms |
| SYJ208 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 3 | 0 | 3 | 2 | 4 | 0 | 11 | 0 | 2.4 ms | 495.9 ms |
| SYJ208 | iltp: minimal ljt j1 | ljt | 20 | 6 | 0 | 6 | 0 | 4 | 0 | 10 | 0 | 683 µs | 381.7 ms |
| SYJ208 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 836 µs | 30.9 ms |
| SYJ208 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 834 µs | 31.2 ms |
| SYJ209 | iltp: intuitionistic embedding j1 | inverse, two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 122.6 ms | 2.39 s |
| SYJ209 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 119 µs | 2.8 ms |
| SYJ209 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 99 µs | 2.1 ms |
| SYJ209 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 102 µs | 2.1 ms |
| SYJ209 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 133.5 ms | 2.62 s |
| SYJ209 | iltp: minimal ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 112 µs | 2.6 ms |
| SYJ209 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 98 µs | 2.1 ms |
| SYJ209 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 101 µs | 2.1 ms |
| SYJ210 | iltp: intuitionistic embedding j1 | two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 383 µs | 21.5 ms |
| SYJ210 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 29 µs | 594 µs |
| SYJ210 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 47 µs | 930 µs |
| SYJ210 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 48 µs | 932 µs |
| SYJ210 | iltp: minimal embedding j1 | two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 410 µs | 14.8 ms |
| SYJ210 | iltp: minimal ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 17 µs | 335 µs |
| SYJ210 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 48 µs | 941 µs |
| SYJ210 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 49 µs | 954 µs |
| SYJ211 | iltp: intuitionistic embedding j1 | inverse, two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 92.4 ms | 1.74 s |
| SYJ211 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 5.4 ms | 295.1 ms |
| SYJ211 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 132 µs | 2.8 ms |
| SYJ211 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 133 µs | 2.7 ms |
| SYJ211 | iltp: minimal embedding j1 | inverse, two-sided | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 97.1 ms | 1.77 s |
| SYJ211 | iltp: minimal ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 5.6 ms | 326.4 ms |
| SYJ211 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 134 µs | 2.8 ms |
| SYJ211 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 132 µs | 2.8 ms |
| SYJ212 | iltp: intuitionistic embedding j1 | two-sided | 20 | 14 | 0 | 14 | 0 | 5 | 0 | 1 | 0 | 4.4 ms | 2.52 s |
| SYJ212 | iltp: intuitionistic ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 106 µs | 2.3 ms |
| SYJ212 | iltp: intuitionistic sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 341 µs | 113.9 ms |
| SYJ212 | iltp: intuitionistic auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 338 µs | 114.6 ms |
| SYJ212 | iltp: minimal embedding j1 | two-sided | 20 | 14 | 0 | 14 | 2 | 3 | 0 | 1 | 0 | 3.9 ms | 2.32 s |
| SYJ212 | iltp: minimal ljt j1 | ljt | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 77 µs | 1.5 ms |
| SYJ212 | iltp: minimal sat j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 154 µs | 3.2 ms |
| SYJ212 | iltp: minimal auto j1 | sat | 20 | 20 | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 160 µs | 3.2 ms |
| Problems/SYN | iltp: intuitionistic embedding j1 | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 0 | 53 µs | 1.6 ms |
| Problems/SYN | iltp: intuitionistic ljt j1 | ljt | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 0 | 19 µs | 420 µs |
| Problems/SYN | iltp: intuitionistic sat j1 | sat | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 0 | 42 µs | 911 µs |
| Problems/SYN | iltp: intuitionistic auto j1 | sat | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 0 | 44 µs | 1.1 ms |
| Problems/SYN | iltp: minimal embedding j1 | two-sided | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 0 | 102 µs | 1.8 ms |
| Problems/SYN | iltp: minimal ljt j1 | ljt | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 0 | 19 µs | 413 µs |
| Problems/SYN | iltp: minimal sat j1 | sat | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 0 | 40 µs | 873 µs |
| Problems/SYN | iltp: minimal auto j1 | sat | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 0 | 42 µs | 1.0 ms |
| SYN007 | iltp: intuitionistic embedding j1 |  | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| SYN007 | iltp: intuitionistic ljt j1 | ljt | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 192 µs | 192 µs |
| SYN007 | iltp: intuitionistic sat j1 | sat | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 355 µs | 355 µs |
| SYN007 | iltp: intuitionistic auto j1 | sat | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 388 µs | 388 µs |
| SYN007 | iltp: minimal embedding j1 |  | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | – | – |
| SYN007 | iltp: minimal ljt j1 | ljt | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 181 µs | 181 µs |
| SYN007 | iltp: minimal sat j1 | sat | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 363 µs | 363 µs |
| SYN007 | iltp: minimal auto j1 | sat | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 368 µs | 368 µs |
| counter | intuitionistic: intuitionistic auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 45 µs | 266 µs |
| counter-over | intuitionistic: intuitionistic auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 44 µs | 260 µs |
| chain | intuitionistic: intuitionistic auto j1 | inverse, two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 38.0 ms | 304.4 ms |
| 3-partition-yes | intuitionistic: intuitionistic auto j1 | two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 63 µs | 249 µs |
| 3-partition-no | intuitionistic: intuitionistic auto j1 | two-sided | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 324 µs | 324 µs |
| ILL/ILLTP-SYJ-01 | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 0 | 0 | 0 | 0 | 0 | 15 | 16 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 0 | 0 | 0 | 0 | 0 | 17 | 14 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 2 | 2 | 0 | 0 | 2 | 6 | 21 | 0 | 2.46 s | 2.56 s |
| ILL/petri-nets/MCC | lltp-all-cores: intuitionistic auto j16 | horn | 1009 | 991 | 991 | 0 | 0 | 17 | 0 | 1 | 0 | 258 µs | 12.03 s |
| ILL/ILLTP-SYJ-01 | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 5 | 0 | 0 | 0 | 0 | 0 | 1 | 4 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 4 | 0 | 0 | 0 | 0 | 0 | 1 | 3 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-classical: classical auto j1 | horn | 3137 | 3071 | 3071 | 0 | 0 | 58 | 0 | 8 | 0 | 403 µs | 39.25 s |
| ILL/misc | lltp-classical: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 57 µs | 239 µs |
| ILL/Non-theorems | lltp-classical: classical auto j1 | focus | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/KLE-cbv | lltp-classical: classical auto j1 | focus | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 0 | 52 µs | 4.1 ms |
| ILL/KLE-cbn | lltp-classical: classical auto j1 | focus | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 0 | 85 µs | 5.9 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-classical: classical auto j1 | focus | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 33 µs | 810 µs |
| ILL/KLE-IMP-CONJ | lltp-classical: classical auto j1 | focus | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 49 µs | 9.4 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-classical: classical auto j1 | focus | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 41 µs | 1.4 ms |
| ILL/KLE-01 | lltp-classical: classical auto j1 | focus | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 0 | 57 µs | 3.5 ms |
| ILL/ILLTP-SYN-cbv | lltp-classical: classical auto j1 | focus | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 0 | 44 µs | 853 µs |
| ILL/ILLTP-SYN-cbn | lltp-classical: classical auto j1 | focus | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 0 | 73 µs | 1.3 ms |
| ILL/ILLTP-SYN-01 | lltp-classical: classical auto j1 | focus | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 0 | 43 µs | 722 µs |
| ILL/ILLTP-SYJ-cbv | lltp-classical: classical auto j1 | focus | 252 | 43 | 19 | 24 | 0 | 22 | 175 | 12 | 0 | 66 µs | 3.64 s |
| ILL/ILLTP-SYJ-cbn | lltp-classical: classical auto j1 | focus, horn | 252 | 38 | 13 | 25 | 0 | 3 | 187 | 24 | 0 | 120 µs | 4.2 ms |
| ILL/ILLTP-SYJ-01 | lltp-classical: classical auto j1 | focus, horn | 252 | 34 | 10 | 24 | 0 | 2 | 192 | 24 | 0 | 82 µs | 3.5 ms |
| ILL/ILLTP-LCL-cbv | lltp-classical: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 63 µs | 63 µs |
| ILL/ILLTP-LCL-cbn | lltp-classical: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 111 µs | 175 µs |
| ILL/ILLTP-LCL-01 | lltp-classical: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 65 µs | 65 µs |
| CLL/misc | lltp-classical: classical auto j1 | focus | 14 | 8 | 8 | 0 | 0 | 0 | 6 | 0 | 0 | 38 µs | 370 µs |
| CLL/Non-theorems | lltp-classical: classical auto j1 | focus | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 43 µs | 159 µs |
| ILL/ILLTP-SYJ-01 | lltp-classical-generous: classical auto j1 | focus | 4 | 0 | 0 | 0 | 0 | 1 | 1 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-classical-generous: classical auto j1 | focus | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-classical-generous: classical auto j1 | focus | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-classical-generous: classical auto j1 | horn | 25 | 25 | 25 | 0 | 0 | 0 | 0 | 0 | 0 | 240 µs | 37.1 ms |
| ILL/ILLTP-LCL-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 125 µs | 210 µs |
| ILL/ILLTP-LCL-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 160 µs | 230 µs |
| ILL/ILLTP-LCL-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 90 µs | 174 µs |
| ILL/ILLTP-SYJ-01 | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 94 | 52 | 42 | 0 | 61 | 54 | 0 | 4 | 1.8 ms | 6.57 s |
| ILL/ILLTP-SYJ-cbn | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 124 | 64 | 60 | 0 | 49 | 36 | 0 | 4 | 1.2 ms | 15.82 s |
| ILL/ILLTP-SYJ-cbv | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 123 | 76 | 47 | 0 | 56 | 30 | 0 | 4 | 7.7 ms | 52.25 s |
| ILL/ILLTP-SYN-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 7 | 3 | 4 | 0 | 0 | 1 | 0 | 0 | 98 µs | 910 µs |
| ILL/ILLTP-SYN-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 8 | 3 | 5 | 0 | 0 | 0 | 0 | 0 | 104 µs | 790 µs |
| ILL/ILLTP-SYN-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 8 | 3 | 5 | 0 | 0 | 0 | 0 | 0 | 57 µs | 572 µs |
| ILL/KLE-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 37 | 28 | 9 | 0 | 0 | 1 | 0 | 8 | 134 µs | 38.0 ms |
| ILL/KLE-IMP-CONJ | lltp-copies-10: intuitionistic auto j1 | two-sided | 60 | 60 | 60 | 0 | 0 | 0 | 0 | 0 | 0 | 113 µs | 7.4 ms |
| ILL/KLE-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 38 | 28 | 10 | 0 | 0 | 0 | 0 | 9 | 107 µs | 9.4 ms |
| ILL/KLE-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 37 | 28 | 9 | 0 | 0 | 1 | 0 | 8 | 66 µs | 10.1 ms |
| ILL/Non-theorems | lltp-copies-10: intuitionistic auto j1 | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-copies-10: intuitionistic auto j1 | horn | 171 | 171 | 171 | 0 | 0 | 0 | 0 | 0 | 0 | 183 µs | 1.82 s |
| ILL/ILLTP-LCL-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 171 µs | 284 µs |
| ILL/ILLTP-LCL-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 130 µs | 248 µs |
| ILL/ILLTP-LCL-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 124 µs | 240 µs |
| ILL/ILLTP-SYJ-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | horn, inverse, two-sided | 252 | 137 | 67 | 70 | 0 | 87 | 0 | 28 | 4 | 17.2 ms | 19.08 s |
| ILL/ILLTP-SYJ-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | horn, inverse, two-sided | 252 | 164 | 85 | 79 | 0 | 61 | 0 | 27 | 4 | 731 µs | 8.68 s |
| ILL/ILLTP-SYJ-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | inverse, two-sided | 252 | 154 | 81 | 73 | 0 | 76 | 0 | 22 | 4 | 12.3 ms | 30.19 s |
| ILL/ILLTP-SYN-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 87 µs | 6.0 ms |
| ILL/ILLTP-SYN-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 3 | 97 µs | 2.0 ms |
| ILL/ILLTP-SYN-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 89 µs | 1.8 ms |
| ILL/KLE-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | inverse, two-sided | 88 | 87 | 76 | 11 | 1 | 0 | 0 | 0 | 11 | 124 µs | 634.9 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 81 µs | 2.4 ms |
| ILL/KLE-IMP-CONJ | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 222 | 222 | 222 | 0 | 0 | 0 | 0 | 0 | 0 | 105 µs | 26.7 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 68 µs | 1.6 ms |
| ILL/KLE-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 112 µs | 163.4 ms |
| ILL/KLE-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 95 µs | 213.0 ms |
| ILL/Non-theorems | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | – | – |
| ILL/misc | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 92 µs | 308 µs |
| ILL/petri-nets/MCC | lltp-default: intuitionistic auto j16 pool after 0.1s | horn | 3137 | 3069 | 3069 | 0 | 0 | 66 | 0 | 2 | 0 | 822 µs | 36.18 s |
| ILL/ILLTP-LCL-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 129 µs | 207 µs |
| ILL/ILLTP-LCL-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 63 µs | 125 µs |
| ILL/ILLTP-LCL-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 94 µs | 185 µs |
| ILL/ILLTP-SYJ-01 | lltp-forward: intuitionistic auto j1 bias factors | horn, inverse, two-sided | 252 | 139 | 69 | 70 | 0 | 87 | 1 | 25 | 4 | 12.2 ms | 33.03 s |
| ILL/ILLTP-SYJ-cbn | lltp-forward: intuitionistic auto j1 bias factors | horn, inverse, two-sided | 252 | 170 | 86 | 84 | 0 | 59 | 0 | 23 | 4 | 449 µs | 17.51 s |
| ILL/ILLTP-SYJ-cbv | lltp-forward: intuitionistic auto j1 bias factors | inverse, two-sided | 252 | 162 | 86 | 76 | 0 | 78 | 0 | 12 | 4 | 13.8 ms | 81.90 s |
| ILL/ILLTP-SYN-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 54 µs | 4.2 ms |
| ILL/ILLTP-SYN-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 3 | 46 µs | 1.0 ms |
| ILL/ILLTP-SYN-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 48 µs | 1.0 ms |
| ILL/KLE-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 87 | 76 | 11 | 0 | 0 | 1 | 0 | 10 | 65 µs | 1.05 s |
| ILL/KLE-IMP-CONJ/ALT | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 41 µs | 1.1 ms |
| ILL/KLE-IMP-CONJ | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 222 | 222 | 222 | 0 | 0 | 0 | 0 | 0 | 0 | 51 µs | 15.0 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 33 µs | 816 µs |
| ILL/KLE-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 48 µs | 40.6 ms |
| ILL/KLE-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 87 | 76 | 11 | 0 | 0 | 1 | 0 | 10 | 54 µs | 88.5 ms |
| ILL/Non-theorems | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 46 µs | 145 µs |
| ILL/petri-nets/MCC | lltp-forward: intuitionistic auto j1 bias factors | horn | 3137 | 3071 | 3071 | 0 | 0 | 57 | 0 | 9 | 0 | 418 µs | 37.34 s |
| ILL/ILLTP-LCL-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 102 µs | 102 µs |
| ILL/ILLTP-LCL-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 131 µs | 193 µs |
| ILL/ILLTP-LCL-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 68 µs | 68 µs |
| ILL/ILLTP-SYJ-01 | lltp-intuitionistic: intuitionistic auto j1 | horn, two-sided | 252 | 34 | 10 | 24 | 0 | 4 | 189 | 25 | 3 | 85 µs | 3.6 ms |
| ILL/ILLTP-SYJ-cbn | lltp-intuitionistic: intuitionistic auto j1 | horn, two-sided | 252 | 38 | 13 | 25 | 0 | 4 | 186 | 24 | 4 | 151 µs | 5.2 ms |
| ILL/ILLTP-SYJ-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 252 | 43 | 19 | 24 | 0 | 23 | 174 | 12 | 3 | 77 µs | 3.90 s |
| ILL/ILLTP-SYN-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 2 | 56 µs | 794 µs |
| ILL/ILLTP-SYN-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 3 | 96 µs | 1.5 ms |
| ILL/ILLTP-SYN-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 2 | 56 µs | 1.1 ms |
| ILL/KLE-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 10 | 67 µs | 4.1 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 53 µs | 1.8 ms |
| ILL/KLE-IMP-CONJ | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 56 µs | 10.5 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 37 µs | 899 µs |
| ILL/KLE-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 10 | 90 µs | 7.0 ms |
| ILL/KLE-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 10 | 63 µs | 5.1 ms |
| ILL/Non-theorems | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 53 µs | 236 µs |
| ILL/petri-nets/MCC | lltp-intuitionistic: intuitionistic auto j1 | horn | 3137 | 3071 | 3071 | 0 | 0 | 58 | 0 | 8 | 0 | 438 µs | 40.16 s |
| ILL/ILLTP-SYJ-01 | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-intuitionistic-generous: intuitionistic auto j1 | horn | 19 | 19 | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 282 µs | 36.7 ms |
| ILL/ILLTP-LCL-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 99 µs | 99 µs |
| ILL/ILLTP-LCL-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 72 µs | 136 µs |
| ILL/ILLTP-LCL-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 62 µs | 62 µs |
| ILL/ILLTP-SYJ-01 | lltp-rarer: intuitionistic auto j1 bias rarer | horn, two-sided | 252 | 34 | 10 | 24 | 0 | 4 | 189 | 25 | 3 | 80 µs | 3.5 ms |
| ILL/ILLTP-SYJ-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | horn, two-sided | 252 | 37 | 13 | 24 | 0 | 5 | 187 | 23 | 4 | 67 µs | 2.5 ms |
| ILL/ILLTP-SYJ-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 252 | 43 | 19 | 24 | 0 | 23 | 174 | 12 | 3 | 72 µs | 3.82 s |
| ILL/ILLTP-SYN-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 2 | 53 µs | 769 µs |
| ILL/ILLTP-SYN-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 3 | 43 µs | 921 µs |
| ILL/ILLTP-SYN-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 2 | 51 µs | 924 µs |
| ILL/KLE-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 10 | 59 µs | 3.6 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 45 µs | 1.3 ms |
| ILL/KLE-IMP-CONJ | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 53 µs | 8.9 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 39 µs | 950 µs |
| ILL/KLE-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 10 | 48 µs | 3.9 ms |
| ILL/KLE-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 10 | 51 µs | 4.1 ms |
| ILL/Non-theorems | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 44 µs | 151 µs |
| ILL/petri-nets/MCC | lltp-rarer: intuitionistic auto j1 bias rarer | horn | 3137 | 3071 | 3071 | 0 | 0 | 58 | 0 | 8 | 0 | 434 µs | 40.18 s |
| ILL/ILLTP-SYJ-01 | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 20 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 20 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 22 | 0 | 0 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-recursion: intuitionistic auto j1 | horn | 929 | 895 | 895 | 0 | 0 | 32 | 0 | 2 | 0 | 3.5 ms | 18.00 s |
| ILL/petri-nets/MCC | lltp-recursion-generous: intuitionistic auto j1 | horn | 9 | 9 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 2.4 ms | 29.7 ms |
| 3-partition-no | long-1: classical auto j1 | focus | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 367 µs | 367 µs |
| partition-no | long-1: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 441 µs | 441 µs |
| partition-yes | long-1: classical auto j1 | focus | 2 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 213 µs | 213 µs |
| counter | long-1: classical auto j1 | horn | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 61 µs | 120 µs |
| mix | long-2: mix auto j1 | inverse | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 273.1 ms | 273.1 ms |
| wide-m3 | long-2: classical auto j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 142 µs | 142 µs |
| wide-m4 | long-2: classical auto j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 128 µs | 128 µs |
| qbf | long-2: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 178.9 ms | 178.9 ms |
| 3-partition-yes | parallel: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 94 µs | 378 µs |
| 3-partition-yes | parallel: classical auto j2 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 251 µs | 989 µs |
| 3-partition-yes | parallel: classical auto j4 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 300 µs | 1.2 ms |
| 3-partition-yes | parallel: classical auto j8 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 440 µs | 1.6 ms |
| 3-partition-yes | parallel: classical auto j16 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 673 µs | 2.6 ms |
| 3-partition-no | parallel: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 438 µs | 818 µs |
| 3-partition-no | parallel: classical auto j2 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 537 µs | 1.1 ms |
| 3-partition-no | parallel: classical auto j4 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 501 µs | 994 µs |
| 3-partition-no | parallel: classical auto j8 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 645 µs | 1.1 ms |
| 3-partition-no | parallel: classical auto j16 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 840 µs | 1.5 ms |
| partition-yes | parallel: classical auto j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 275 µs | 64.42 s |
| partition-yes | parallel: classical auto j2 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 539 µs | 69.24 s |
| partition-yes | parallel: classical auto j4 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 601 µs | 73.34 s |
| partition-yes | parallel: classical auto j8 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 631 µs | 77.82 s |
| partition-yes | parallel: classical auto j16 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 849 µs | 83.98 s |
| partition-no | parallel: classical auto j1 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 580 µs | 3.53 s |
| partition-no | parallel: classical auto j2 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 780 µs | 2.05 s |
| partition-no | parallel: classical auto j4 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 584 µs | 1.21 s |
| partition-no | parallel: classical auto j8 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 684 µs | 803.7 ms |
| partition-no | parallel: classical auto j16 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 793 µs | 576.1 ms |
| qbf | parallel: classical auto j1 | focus | 20 | 19 | 3 | 16 | 0 | 0 | 0 | 1 | 0 | 137.3 ms | 196.77 s |
| qbf | parallel: classical auto j2 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 103.2 ms | 14.15 s |
| qbf | parallel: classical auto j4 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 102.3 ms | 14.34 s |
| qbf | parallel: classical auto j8 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 101.4 ms | 14.21 s |
| qbf | parallel: classical auto j16 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 102.6 ms | 14.29 s |
| mix | parallel: mix auto j1 | inverse | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 273.1 ms | 1.11 s |
| mix | parallel: mix auto j2 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 5.86 s | 75.75 s |
| mix | parallel: mix auto j4 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 6.27 s | 78.74 s |
| mix | parallel: mix auto j8 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 7.92 s | 94.85 s |
| mix | parallel: mix auto j16 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 9.59 s | 119.66 s |
| counter | parallel: classical auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 90 µs | 485 µs |
| counter | parallel: classical auto j2 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 112 µs | 688 µs |
| counter | parallel: classical auto j4 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 123 µs | 710 µs |
| counter | parallel: classical auto j8 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 121 µs | 712 µs |
| counter | parallel: classical auto j16 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 115 µs | 679 µs |
| counter-over | parallel: classical auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 90 µs | 527 µs |
| counter-over | parallel: classical auto j2 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 115 µs | 641 µs |
| counter-over | parallel: classical auto j4 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 117 µs | 698 µs |
| counter-over | parallel: classical auto j8 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 111 µs | 655 µs |
| counter-over | parallel: classical auto j16 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 116 µs | 675 µs |
| wide-m3 | parallel: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 168 µs | 507 µs |
| wide-m3 | parallel: classical auto j2 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 406 µs | 1.2 ms |
| wide-m3 | parallel: classical auto j4 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 400 µs | 1.2 ms |
| wide-m3 | parallel: classical auto j8 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 523 µs | 1.5 ms |
| wide-m3 | parallel: classical auto j16 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 663 µs | 2.0 ms |
| wide-m4 | parallel: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 152 µs | 474 µs |
| wide-m4 | parallel: classical auto j2 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 343 µs | 1.0 ms |
| wide-m4 | parallel: classical auto j4 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 429 µs | 1.2 ms |
| wide-m4 | parallel: classical auto j8 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 501 µs | 1.5 ms |
| wide-m4 | parallel: classical auto j16 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 651 µs | 2.1 ms |
| 3-partition-mll-no | parallel-net: classical net j1 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.0 ms | 3.1 ms |
| 3-partition-mll-no | parallel-net: classical net j2 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.1 ms | 2.9 ms |
| 3-partition-mll-no | parallel-net: classical net j4 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.1 ms | 3.5 ms |
| 3-partition-mll-no | parallel-net: classical net j8 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 3.7 ms |
| 3-partition-mll-no | parallel-net: classical net j16 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.5 ms | 4.4 ms |
| partition-table | parallel-net: classical net j1 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.8 ms | 5.05 s |
| partition-table | parallel-net: classical net j2 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 1.54 s |
| partition-table | parallel-net: classical net j4 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.4 ms | 1.25 s |
| partition-table | parallel-net: classical net j8 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 2.2 ms | 94.1 ms |
| partition-table | parallel-net: classical net j16 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 2.9 ms | 50.2 ms |
| wide-m1 | period-16: classical net j1 period 16 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 442.1 ms | 448.7 ms |
| wide-m2 | period-16: classical net j1 period 16 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 437.0 ms | 443.5 ms |
| 3-partition-mll-no | period-16: classical net j1 period 16 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 1.5 ms |
| partition-yes | period-16: classical net j1 period 16 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2.7 ms | 2.7 ms |
| partition-no | period-16: classical net j1 period 16 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 24.0 ms | 26.3 ms |
| wide-m1 | period-1: classical net j1 period 1 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 1.28 s | 1.30 s |
| wide-m2 | period-1: classical net j1 period 1 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 1.27 s | 1.29 s |
| 3-partition-mll-no | period-1: classical net j1 period 1 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 724 µs | 994 µs |
| partition-yes | period-1: classical net j1 period 1 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1.7 ms | 1.7 ms |
| partition-no | period-1: classical net j1 period 1 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 9.5 ms | 11.1 ms |
| wide-m1 | period-2: classical net j1 period 2 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 829.7 ms | 842.1 ms |
| wide-m2 | period-2: classical net j1 period 2 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 831.4 ms | 844.1 ms |
| 3-partition-mll-no | period-2: classical net j1 period 2 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 743 µs | 998 µs |
| partition-yes | period-2: classical net j1 period 2 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1.4 ms | 1.4 ms |
| partition-no | period-2: classical net j1 period 2 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 7.8 ms | 9.1 ms |
| wide-m1 | period-8: classical net j1 period 8 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 496.6 ms | 504.1 ms |
| wide-m2 | period-8: classical net j1 period 8 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 490.4 ms | 497.6 ms |
| 3-partition-mll-no | period-8: classical net j1 period 8 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 749 µs | 997 µs |
| partition-yes | period-8: classical net j1 period 8 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5.3 ms | 5.3 ms |
| partition-no | period-8: classical net j1 period 8 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 22.9 ms | 24.3 ms |

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
| SYJ201 | iltp: intuitionistic ljt j1 | 20 | 34420 | 0 | 840 | 34420 | 0 | 0 |
| SYJ201 | iltp: intuitionistic sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp: intuitionistic auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp: minimal embedding j1 | 5 | 4780088 | 1133939 | 3456735 | 1230227 | 0 | 0 |
| SYJ201 | iltp: minimal ljt j1 | 20 | 34420 | 0 | 840 | 34420 | 0 | 0 |
| SYJ201 | iltp: minimal sat j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ201 | iltp: minimal auto j1 | 20 | 0 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp: intuitionistic embedding j1 | 3 | 558958 | 0 | 378783 | 74853 | 0 | 0 |
| SYJ202 | iltp: intuitionistic ljt j1 | 4 | 44086 | 0 | 11013 | 44086 | 0 | 0 |
| SYJ202 | iltp: intuitionistic sat j1 | 6 | 988 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp: intuitionistic auto j1 | 6 | 988 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp: minimal embedding j1 | 3 | 558958 | 0 | 378783 | 74853 | 0 | 0 |
| SYJ202 | iltp: minimal ljt j1 | 4 | 44086 | 0 | 11013 | 44086 | 0 | 0 |
| SYJ202 | iltp: minimal sat j1 | 6 | 988 | 0 | 0 | 0 | 0 | 0 |
| SYJ202 | iltp: minimal auto j1 | 6 | 988 | 0 | 0 | 0 | 0 | 0 |
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
| SYJ206 | iltp: intuitionistic ljt j1 | 16 | 884465 | 0 | 5 | 884465 | 0 | 0 |
| SYJ206 | iltp: intuitionistic sat j1 | 15 | 16928 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp: intuitionistic auto j1 | 15 | 16928 | 0 | 0 | 0 | 0 | 0 |
| SYJ206 | iltp: minimal embedding j1 | 7 | 2228635 | 1290587 | 1621217 | 456167 | 0 | 0 |
| SYJ206 | iltp: minimal ljt j1 | 16 | 884465 | 0 | 5 | 884465 | 0 | 0 |
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
| ILL/ILLTP-SYJ-cbv | lltp-all-cores: intuitionistic auto j16 | 2 | 13451242 | 11663067 | 7995798 | 1281549 | 0 | 0 |
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
| ILL/ILLTP-SYJ-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 137 | 117340334 | 77266868 | 100814802 | 8183468 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 164 | 32252390 | 12946020 | 22819745 | 5373490 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 154 | 335347278 | 266069618 | 287631571 | 17594620 | 0 | 0 |
| ILL/ILLTP-SYN-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 27766 | 9512 | 16890 | 1189 | 0 | 0 |
| ILL/ILLTP-SYN-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 595 | 248 | 189 | 228 | 0 | 0 |
| ILL/ILLTP-SYN-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 934 | 807 | 352 | 257 | 0 | 0 |
| ILL/KLE-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 87 | 5572672 | 2666537 | 4699065 | 51239 | 0 | 0 |
| ILL/KLE-IMP-CONJ/ALT | lltp-default: intuitionistic auto j16 pool after 0.1s | 27 | 122 | 145 | 0 | 98 | 0 | 0 |
| ILL/KLE-IMP-CONJ | lltp-default: intuitionistic auto j16 pool after 0.1s | 222 | 26036 | 13152 | 12564 | 6583 | 0 | 0 |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-default: intuitionistic auto j16 pool after 0.1s | 22 | 36 | 34 | 0 | 36 | 0 | 0 |
| ILL/KLE-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 88 | 1281131 | 844754 | 1082969 | 9304 | 0 | 0 |
| ILL/KLE-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 88 | 1425291 | 1458651 | 1205931 | 10429 | 0 | 0 |
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
| 3-partition-yes | parallel: classical auto j4 | 4 | 97 | 533 | 2 | 63 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j8 | 4 | 235 | 1390 | 31 | 156 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j16 | 4 | 293 | 1599 | 43 | 176 | 0 | 0 |
| 3-partition-no | parallel: classical auto j1 | 2 | 1894 | 6088 | 1446 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j2 | 2 | 1907 | 6128 | 1456 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j4 | 2 | 2335 | 7564 | 1766 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j8 | 2 | 2500 | 8021 | 1916 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j16 | 2 | 3009 | 9723 | 2289 | 448 | 0 | 0 |
| partition-yes | parallel: classical auto j1 | 5 | 38546799 | 387076933 | 33786817 | 1098638 | 0 | 0 |
| partition-yes | parallel: classical auto j2 | 5 | 76935396 | 771542688 | 67443894 | 1098642 | 0 | 0 |
| partition-yes | parallel: classical auto j4 | 5 | 152154592 | 1519870612 | 133431156 | 1098737 | 0 | 0 |
| partition-yes | parallel: classical auto j8 | 5 | 278842715 | 2636400534 | 245742726 | 1098921 | 0 | 0 |
| partition-yes | parallel: classical auto j16 | 5 | 453040138 | 4089302980 | 400907735 | 1099133 | 0 | 0 |
| partition-no | parallel: classical auto j1 | 3 | 4252557 | 27627908 | 3720792 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j2 | 3 | 4634299 | 30221837 | 4046977 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j4 | 3 | 5137781 | 33379269 | 4481993 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j8 | 3 | 5454803 | 34640049 | 4775085 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j16 | 3 | 6470075 | 40263461 | 5663424 | 531765 | 0 | 0 |
| qbf | parallel: classical auto j1 | 19 | 258912034 | 1596527780 | 28181763 | 7813436 | 0 | 0 |
| qbf | parallel: classical auto j2 | 14 | 27115409 | 148796043 | 2702175 | 2631584 | 0 | 0 |
| qbf | parallel: classical auto j4 | 14 | 27115355 | 148795963 | 2702177 | 2631527 | 0 | 0 |
| qbf | parallel: classical auto j8 | 14 | 27102613 | 148776256 | 2702176 | 2618785 | 0 | 0 |
| qbf | parallel: classical auto j16 | 14 | 27115122 | 148795599 | 2702177 | 2631294 | 0 | 0 |
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
| partition-table | parallel-net: classical net j2 | 13 | 1282679 | 0 | 0 | 0 | 1389523 | 457820 |
| partition-table | parallel-net: classical net j4 | 13 | 1986458 | 0 | 0 | 0 | 2185603 | 662289 |
| partition-table | parallel-net: classical net j8 | 13 | 81053 | 0 | 0 | 0 | 153284 | 102669 |
| partition-table | parallel-net: classical net j16 | 13 | 29724 | 0 | 0 | 0 | 112584 | 45691 |
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
| partition-yes/4 | 71 µs ✓ | 1.8 ms ✓ | 72 µs ✓ |  |  |  |  |  |  | 2.7 ms ✓ | 1.7 ms ✓ | 1.4 ms ✓ | 5.3 ms ✓ |
| partition-yes/5 | 89 µs ✓ | 28.8 ms ✓ | 88 µs ✓ |  | 128 µs ✓ | 315 µs ✓ | 281 µs ✓ | 399 µs ✓ | 675 µs ✓ |  |  |  |  |
| partition-yes/6 | 155 µs ✓ | 474.0 ms ✓ | 137 µs ✓ |  | 192 µs ✓ | 428 µs ✓ | 395 µs ✓ | 631 µs ✓ | 667 µs ✓ |  |  |  |  |
| partition-yes/12 | 13.0 ms ✓ | > 60 s | 12.8 ms ✓ |  |  |  |  |  |  |  |  |  |  |
| partition-yes/20 | 420.5 ms ✓ | > 60 s | 415.7 ms ✓ |  | 388.9 ms ✓ | 405.7 ms ✓ | 418.0 ms ✓ | 432.6 ms ✓ | 247.8 ms ✓ |  |  |  |  |
| partition-yes/7 |  |  | 212 µs ✓ | 213 µs ✓ | 275 µs ✓ | 539 µs ✓ | 601 µs ✓ | 494 µs ✓ | 849 µs ✓ |  |  |  |  |
| partition-yes/16 |  |  | 136.5 ms ✓ |  |  |  |  |  |  |  |  |  |  |
| partition-yes/24 |  |  | 71.92 s ✓ |  | 64.03 s ✓ | 68.84 s ✓ | 72.92 s ✓ | 77.39 s ✓ | 83.73 s ✓ |  |  |  |  |
| partition-yes/28 |  |  | > 300 s | > 1200 s |  |  |  |  |  |  |  |  |  |

## partition-no

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| partition-no/3 | 81 µs ✗ | 1.5 ms ✗ | 85 µs ✗ |  |  |  |  |  |  | 2.3 ms ✗ | 1.5 ms ✗ | 1.3 ms ✗ | 1.5 ms ✗ |
| partition-no/4 | 151 µs ✗ | 9.5 ms ✗ | 163 µs ✗ |  | 204 µs ✗ | 416 µs ✗ | 363 µs ✗ | 403 µs ✗ | 685 µs ✗ | 24.0 ms ✗ | 9.5 ms ✗ | 7.8 ms ✗ | 22.9 ms ✗ |
| partition-no/9 | 81.6 ms ✗ | > 60 s | 79.7 ms ✗ |  |  |  |  |  |  |  |  |  |  |
| partition-no/12 | 3.60 s ✗ | > 60 s | 3.59 s ✗ |  | 3.53 s ✗ | 2.05 s ✗ | 1.21 s ✗ | 802.6 ms ✗ | 574.7 ms ✗ |  |  |  |  |
| partition-no/5 |  |  | 444 µs ✗ | 441 µs ✗ | 580 µs ✗ | 780 µs ✗ | 584 µs ✗ | 684 µs ✗ | 793 µs ✗ |  |  |  |  |
| partition-no/14 |  |  | > 300 s |  | > 120 s | > 120 s | > 120 s | > 120 s | > 120 s |  |  |  |  |
| partition-no/15 |  |  | > 300 s | > 1200 s |  |  |  |  |  |  |  |  |  |

## 3-partition-mll-yes

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 |
|---|--:|--:|--:|
| 3-partition-mll-yes/4 | 55 µs ✓ | 77 µs ✓ | 61 µs ✓ |
| 3-partition-mll-yes/6 | 53 µs ✓ | 101 µs ✓ | 64 µs ✓ |
| 3-partition-mll-yes/8 | 53 µs ✓ | 141 µs ✓ | 59 µs ✓ |
| 3-partition-mll-yes/12 | 61 µs ✓ | 227 µs ✓ | 63 µs ✓ |

## 3-partition-mll-no

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | parallel-net: classical net j1 | parallel-net: classical net j2 | parallel-net: classical net j4 | parallel-net: classical net j8 | parallel-net: classical net j16 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-mll-no/4 | 43 µs ✗ | 256 µs ✗ | 46 µs ✗ | 362 µs ✗ | 476 µs ✗ | 515 µs ✗ | 657 µs ✗ | 821 µs ✗ | 263 µs ✗ | 270 µs ✗ | 255 µs ✗ | 248 µs ✗ |
| 3-partition-mll-no/5 | 42 µs ✗ | 703 µs ✗ | 52 µs ✗ | 1.0 ms ✗ | 1.1 ms ✗ | 1.1 ms ✗ | 1.2 ms ✗ | 1.5 ms ✗ | 1.2 ms ✗ | 724 µs ✗ | 743 µs ✗ | 749 µs ✗ |
| 3-partition-mll-no/6 | 43 µs ✗ | 1.2 ms ✗ | 51 µs ✗ | 1.7 ms ✗ | 1.3 ms ✗ | 1.9 ms ✗ | 1.9 ms ✗ | 2.1 ms ✗ |  |  |  |  |
| 3-partition-mll-no/8 | 44 µs ✗ | 3.1 ms ✗ | 50 µs ✗ |  |  |  |  |  |  |  |  |  |

## wide-m1

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|
| wide-m1/8 | 50 µs ✓ | 63 µs ✓ | 52 µs ✓ |  |  |  |  |
| wide-m1/16 | 70 µs ✓ | 127 µs ✓ | 69 µs ✓ |  |  |  |  |
| wide-m1/24 | 98 µs ✓ | 218 µs ✓ | 93 µs ✓ |  |  |  |  |
| wide-m1/32 | 122 µs ✓ | 200 µs ✓ | 119 µs ✓ |  |  |  |  |
| wide-m1/256 | 3.3 ms ✓ | 9.3 ms ✓ | 3.2 ms ✓ | 6.6 ms ✓ | 19.1 ms ✓ | 12.4 ms ✓ | 7.6 ms ✓ |
| wide-m1/2048 | 212.2 ms ✓ | 612.0 ms ✓ | 200.7 ms ✓ | 442.1 ms ✓ | 1.28 s ✓ | 829.7 ms ✓ | 496.6 ms ✓ |

## wide-m2

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|
| wide-m2/8 | 58 µs ✓ | 68 µs ✓ | 52 µs ✓ |  |  |  |  |
| wide-m2/16 | 73 µs ✓ | 128 µs ✓ | 69 µs ✓ |  |  |  |  |
| wide-m2/24 | 90 µs ✓ | 219 µs ✓ | 86 µs ✓ |  |  |  |  |
| wide-m2/32 | 115 µs ✓ | 204 µs ✓ | 109 µs ✓ |  |  |  |  |
| wide-m2/256 | 3.0 ms ✓ | 9.1 ms ✓ | 2.6 ms ✓ | 6.5 ms ✓ | 19.1 ms ✓ | 12.7 ms ✓ | 7.3 ms ✓ |
| wide-m2/2048 | 203.1 ms ✓ | 605.4 ms ✓ | 186.2 ms ✓ | 437.0 ms ✓ | 1.27 s ✓ | 831.4 ms ✓ | 490.4 ms ✓ |

## wide-m3

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| wide-m3/12 | 62 µs ✓ | 95 µs ✓ | 62 µs ✓ |  |  |  |  |  |  |
| wide-m3/24 | 84 µs ✓ | 242 µs ✓ | 81 µs ✓ |  | 164 µs ✓ | 353 µs ✓ | 341 µs ✓ | 460 µs ✓ | 726 µs ✓ |
| wide-m3/30 | 105 µs ✓ | 198 µs ✓ | 101 µs ✓ |  | 168 µs ✓ | 406 µs ✓ | 400 µs ✓ | 523 µs ✓ | 649 µs ✓ |
| wide-m3/256 | 2.5 ms ✓ | 9.8 ms ✓ | 2.4 ms ✓ |  |  |  |  |  |  |
| wide-m3/2048 | 180.0 ms ✓ | 634.4 ms ✓ | 163.4 ms ✓ |  |  |  |  |  |  |
| wide-m3/36 |  |  | 121 µs ✓ | 142 µs ✓ | 175 µs ✓ | 434 µs ✓ | 491 µs ✓ | 534 µs ✓ | 663 µs ✓ |
| wide-m3/1024 |  |  | 40.6 ms ✓ |  |  |  |  |  |  |

## wide-m4

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| wide-m4/12 | 76 µs ✓ | 104 µs ✓ | 68 µs ✓ |  |  |  |  |  |  |
| wide-m4/24 | 98 µs ✓ | 261 µs ✓ | 83 µs ✓ |  |  |  |  |  |  |
| wide-m4/28 | 106 µs ✓ | 294 µs ✓ | 96 µs ✓ |  | 148 µs ✓ | 300 µs ✓ | 317 µs ✓ | 501 µs ✓ | 651 µs ✓ |
| wide-m4/256 | 2.4 ms ✓ | 9.6 ms ✓ | 2.2 ms ✓ |  |  |  |  |  |  |
| wide-m4/2048 | 168.1 ms ✓ | 647.9 ms ✓ | 151.6 ms ✓ |  |  |  |  |  |  |
| wide-m4/32 |  |  | 105 µs ✓ |  | 152 µs ✓ | 360 µs ✓ | 430 µs ✓ | 479 µs ✓ | 789 µs ✓ |
| wide-m4/36 |  |  | 118 µs ✓ | 128 µs ✓ | 174 µs ✓ | 343 µs ✓ | 429 µs ✓ | 550 µs ✓ | 638 µs ✓ |
| wide-m4/1024 |  |  | 38.0 ms ✓ |  |  |  |  |  |  |

## additive

| problem | engines: classical focus j1 | families: classical auto j1 |
|---|--:|--:|
| additive/8 | 754 µs ✓ | 338 µs ✓ |
| additive/12 | 32.3 ms ✓ | 10.8 ms ✓ |
| additive/14 | 346.5 ms ✓ | 58.6 ms ✓ |
| additive/16 | 5.49 s ✓ | 181.6 ms ✓ |

## partition-table

| problem | engines: classical focus j1 | engines: classical net j1 | parallel-net: classical net j1 | parallel-net: classical net j2 | parallel-net: classical net j4 | parallel-net: classical net j8 | parallel-net: classical net j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| partition-table/1-1 | 57 µs ✓ | 59 µs ✓ | 77 µs ✓ | 203 µs ✓ | 239 µs ✓ | 348 µs ✓ | 529 µs ✓ |
| partition-table/1-3 | 57 µs ✗ | 124 µs ✗ | 167 µs ✗ | 282 µs ✗ | 306 µs ✗ | 403 µs ✗ | 586 µs ✗ |
| partition-table/2-1-1 | 65 µs ✓ | 187 µs ✓ | 266 µs ✓ | 481 µs ✓ | 498 µs ✓ | 614 µs ✓ | 803 µs ✓ |
| partition-table/1-1-4 | 84 µs ✗ | 564 µs ✗ | 769 µs ✗ | 937 µs ✗ | 918 µs ✗ | 1.0 ms ✗ | 1.2 ms ✗ |
| partition-table/2-2-1-1 | 80 µs ✓ | 994 µs ✓ | 1.3 ms ✓ | 1.0 ms ✓ | 1.3 ms ✓ | 2.2 ms ✓ | 2.5 ms ✓ |
| partition-table/1-2-5 | 83 µs ✗ | 1.3 ms ✗ | 1.8 ms ✗ | 1.2 ms ✗ | 1.9 ms ✗ | 2.0 ms ✗ | 2.2 ms ✗ |
| partition-table/1-1-2-4 | 67 µs ✓ | 905 µs ✓ | 1.3 ms ✓ | 1.1 ms ✓ | 810 µs ✓ | 2.4 ms ✓ | 5.9 ms ✓ |
| partition-table/1-1-1-5 | 142 µs ✗ | 3.3 ms ✗ | 4.3 ms ✗ | 2.6 ms ✗ | 1.8 ms ✗ | 2.9 ms ✗ | 4.5 ms ✗ |
| partition-table/2-3-2-1 | 80 µs ✓ | 2.6 ms ✓ | 3.4 ms ✓ | 2.1 ms ✓ | 1.4 ms ✓ | 2.4 ms ✓ | 5.3 ms ✓ |
| partition-table/3-3-3-1 | 145 µs ✗ | 8.3 ms ✗ | 10.6 ms ✗ | 5.6 ms ✗ | 3.5 ms ✗ | 4.7 ms ✗ | 4.5 ms ✗ |
| partition-table/1-2-3-4-5-5 | 102 µs ✓ | 1.09 s ✓ | 1.03 s ✓ | 121.0 ms ✓ | 1.7 ms ✓ | 53.0 ms ✓ | 7.1 ms ✓ |
| partition-table/1-1-1-1-1-7 | 1.2 ms ✗ | 108.5 ms ✗ | 112.7 ms ✗ | 61.4 ms ✗ | 34.4 ms ✗ | 20.5 ms ✗ | 12.2 ms ✗ |
| partition-table/2-2-2-2-2-2-9-1 | 128 µs ✓ | 4.10 s ✓ | 3.88 s ✓ | 1.34 s ✓ | 1.20 s ✓ | 1.6 ms ✓ | 2.9 ms ✓ |

## cancellation

| problem | engines: classical focus j1 |
|---|--:|
| cancellation/3-partition-4 | 308 µs ✓ |

## 3-partition-yes

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-yes/4 | 75 µs ✓ | 61 µs ✓ | 94 µs ✓ | 259 µs ✓ | 285 µs ✓ | 322 µs ✓ | 667 µs ✓ |
| 3-partition-yes/6 | 65 µs ✓ | 62 µs ✓ | 93 µs ✓ | 251 µs ✓ | 255 µs ✓ | 410 µs ✓ | 701 µs ✓ |
| 3-partition-yes/8 | 65 µs ✓ | 63 µs ✓ | 92 µs ✓ | 248 µs ✓ | 300 µs ✓ | 473 µs ✓ | 673 µs ✓ |
| 3-partition-yes/12 | 67 µs ✓ | 63 µs ✓ | 99 µs ✓ | 231 µs ✓ | 330 µs ✓ | 440 µs ✓ | 564 µs ✓ |

## 3-partition-no

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-no/4 | 292 µs ✗ | 324 µs ✗ |  | 380 µs ✗ | 537 µs ✗ | 493 µs ✗ | 645 µs ✗ | 840 µs ✗ |
| 3-partition-no/5 | 342 µs ✗ |  | 367 µs ✗ | 438 µs ✗ | 530 µs ✗ | 501 µs ✗ | 477 µs ✗ | 669 µs ✗ |

## qbf

| problem | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| qbf/8#0 | 175 µs ✗ |  |  |  |  |  |  |
| qbf/8#1 | 191 µs ✗ |  |  |  |  |  |  |
| qbf/8#2 | 153 µs ✓ |  |  |  |  |  |  |
| qbf/8#3 | 186 µs ✗ |  |  |  |  |  |  |
| qbf/12#0 | 1.2 ms ✓ |  |  |  |  |  |  |
| qbf/12#1 | 963 µs ✗ |  |  |  |  |  |  |
| qbf/12#2 | 596 µs ✓ |  |  |  |  |  |  |
| qbf/12#3 | 951 µs ✗ |  |  |  |  |  |  |
| qbf/16#0 | 5.4 ms ✓ |  | 7.5 ms ✓ | 8.0 ms ✓ | 8.2 ms ✓ | 8.3 ms ✓ | 7.9 ms ✓ |
| qbf/16#1 | 8.1 ms ✗ |  | 11.3 ms ✗ | 11.3 ms ✗ | 10.9 ms ✗ | 11.4 ms ✗ | 11.7 ms ✗ |
| qbf/16#2 | 8.7 ms ✗ |  | 12.2 ms ✗ | 12.4 ms ✗ | 12.2 ms ✗ | 12.4 ms ✗ | 13.0 ms ✗ |
| qbf/16#3 | 4.7 ms ✓ |  | 6.5 ms ✓ | 6.8 ms ✓ | 7.2 ms ✓ | 7.3 ms ✓ | 7.6 ms ✓ |
| qbf/20#0 | 83.3 ms ✗ |  | 103.0 ms ✗ | 101.2 ms ✗ | 101.0 ms ✗ | 101.1 ms ✗ | 101.2 ms ✗ |
| qbf/20#1 | 84.9 ms ✗ |  | 101.9 ms ✗ | 103.2 ms ✗ | 102.3 ms ✗ | 101.4 ms ✗ | 102.6 ms ✗ |
| qbf/20#2 | 27.3 ms ✓ |  | 35.6 ms ✓ | 53.5 ms ✓ | 53.9 ms ✓ | 45.6 ms ✓ | 52.4 ms ✓ |
| qbf/20#3 | 82.4 ms ✗ |  | 96.9 ms ✗ | 95.1 ms ✗ | 96.2 ms ✗ | 96.6 ms ✗ | 97.5 ms ✗ |
| qbf/24#0 | 179.9 ms ✗ | 178.9 ms ✗ | 197.3 ms ✗ | 208.6 ms ✗ | 206.9 ms ✗ | 210.7 ms ✗ | 208.4 ms ✗ |
| qbf/24#1 | 122.9 ms ✗ |  | 137.9 ms ✗ | 135.7 ms ✗ | 136.2 ms ✗ | 140.8 ms ✗ | 136.6 ms ✗ |
| qbf/24#2 | 112.8 ms ✗ |  | 127.7 ms ✗ | 132.3 ms ✗ | 132.0 ms ✗ | 133.7 ms ✗ | 132.6 ms ✗ |
| qbf/24#3 | 125.8 ms ✗ |  | 137.3 ms ✗ | 138.1 ms ✗ | 136.9 ms ✗ | 137.1 ms ✗ | 133.6 ms ✗ |
| qbf/32#0 | 562.6 ms ✗ |  |  |  |  |  |  |
| qbf/32#1 | 614.5 ms ✗ |  |  |  |  |  |  |
| qbf/32#2 | 1.07 s ✗ |  |  |  |  |  |  |
| qbf/32#3 | 497.7 ms ✗ |  |  |  |  |  |  |
| qbf/40#0 | 10.40 s ✗ |  | 10.13 s ✗ | 6.44 s ✗ | 6.55 s ✗ | 6.44 s ✗ | 6.46 s ✗ |
| qbf/40#1 | 11.82 s ✗ |  | 11.47 s ✗ | 5.45 s ? memory | 5.46 s ? memory | 4.84 s ? memory | 4.85 s ? memory |
| qbf/40#2 | 13.79 s ✗ |  | 13.10 s ✗ | 5.17 s ? memory | 5.20 s ? memory | 5.17 s ? memory | 5.17 s ? memory |
| qbf/40#3 | 10.63 s ✗ |  | 10.29 s ✗ | 6.70 s ✗ | 6.78 s ✗ | 6.76 s ✗ | 6.82 s ✗ |
| qbf/44#0 | 44.21 s ✗ |  | 43.46 s ✗ | 7.88 s ? memory | 7.78 s ? memory | 7.82 s ? memory | 7.85 s ? memory |
| qbf/44#1 | 43.88 s ✗ |  | 43.46 s ✗ | 6.62 s ? memory | 6.60 s ? memory | 6.68 s ? memory | 6.60 s ? memory |
| qbf/44#2 | 64.50 s ✗ |  | 63.88 s ✗ | 4.58 s ? memory | 4.59 s ? memory | 4.65 s ? memory | 4.66 s ? memory |
| qbf/44#3 | 22.80 s ? memory |  | 22.78 s ? memory | 7.75 s ? memory | 7.75 s ? memory | 7.71 s ? memory | 7.84 s ? memory |
| qbf/48#0 | 69.37 s ? memory | 75.67 s ? memory |  |  |  |  |  |
| qbf/48#1 | 164.12 s ✗ |  |  |  |  |  |  |
| qbf/48#2 | 137.29 s ✗ |  |  |  |  |  |  |
| qbf/48#3 | 182.43 s ✗ |  |  |  |  |  |  |

## mix

| problem | families: mix auto j1 | long-2: mix auto j1 | parallel: mix auto j1 | parallel: mix auto j2 | parallel: mix auto j4 | parallel: mix auto j8 | parallel: mix auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| mix/4 | 319 µs ✗ |  |  |  |  |  |  |
| mix/6 | 11.1 ms ✗ |  |  |  |  |  |  |
| mix/8 | 254.0 ms ✗ |  | 270.0 ms ✗ | 806.1 ms ✗ | 885.1 ms ✗ | 1.16 s ✗ | 1.41 s ✗ |
| mix/9 | 255.2 ms ✗ |  | 270.1 ms ✗ | 5.86 s ✗ | 6.27 s ✗ | 7.92 s ✗ | 9.59 s ✗ |
| mix/10 | 255.9 ms ✗ |  | 273.1 ms ✗ | 69.08 s ✗ | 71.59 s ✗ | 85.77 s ✗ | 108.67 s ✗ |
| mix/11 | 267.0 ms ✗ | 273.1 ms ✗ | 292.0 ms ✗ | 120.00 s ✗ | 120.00 s ✗ | 120.00 s ✗ | 120.00 s ✗ |

## counter

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| counter/2 | 35 µs ✓ | 34 µs ✓ |  | 47 µs ✓ | 106 µs ✓ | 108 µs ✓ | 112 µs ✓ | 85 µs ✓ |
| counter/4 | 35 µs ✓ | 36 µs ✓ |  | 81 µs ✓ | 119 µs ✓ | 101 µs ✓ | 111 µs ✓ | 99 µs ✓ |
| counter/8 | 36 µs ✓ | 41 µs ✓ |  | 90 µs ✓ | 103 µs ✓ | 123 µs ✓ | 108 µs ✓ | 115 µs ✓ |
| counter/16 | 39 µs ✓ | 45 µs ✓ | 59 µs ✓ | 70 µs ✓ | 112 µs ✓ | 109 µs ✓ | 121 µs ✓ | 111 µs ✓ |
| counter/32 | 46 µs ✓ | 50 µs ✓ |  | 91 µs ✓ | 108 µs ✓ | 137 µs ✓ | 126 µs ✓ | 130 µs ✓ |
| counter/64 | 57 µs ✓ | 60 µs ✓ | 61 µs ✓ | 106 µs ✓ | 140 µs ✓ | 132 µs ✓ | 134 µs ✓ | 139 µs ✓ |

## counter-over

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| counter-over/2 | 30 µs ✗ | 33 µs ✗ | 80 µs ✗ | 100 µs ✗ | 117 µs ✗ | 90 µs ✗ | 99 µs ✗ |
| counter-over/4 | 37 µs ✗ | 37 µs ✗ | 82 µs ✗ | 115 µs ✗ | 108 µs ✗ | 106 µs ✗ | 93 µs ✗ |
| counter-over/8 | 45 µs ✗ | 41 µs ✗ | 75 µs ✗ | 84 µs ✗ | 109 µs ✗ | 110 µs ✗ | 114 µs ✗ |
| counter-over/16 | 42 µs ✗ | 44 µs ✗ | 90 µs ✗ | 115 µs ✗ | 108 µs ✗ | 113 µs ✗ | 116 µs ✗ |
| counter-over/32 | 49 µs ✗ | 50 µs ✗ | 101 µs ✗ | 122 µs ✗ | 139 µs ✗ | 125 µs ✗ | 125 µs ✗ |
| counter-over/64 | 49 µs ✗ | 55 µs ✗ | 99 µs ✗ | 105 µs ✗ | 117 µs ✗ | 111 µs ✗ | 128 µs ✗ |

## growing

| problem | families: classical auto j1 |
|---|--:|
| growing/16 | 230 µs ✗ |
| growing/64 | 857 µs ✗ |
| growing/256 | 18.4 ms ✗ † |
| growing/1024 | 111.0 ms ✗ |

## chain

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 |
|---|--:|--:|
| chain/16 | 154 µs ✓ | 160 µs ✓ |
| chain/64 | 4.3 ms ✓ | 4.8 ms ✓ |
| chain/128 | 32.6 ms ✓ | 38.0 ms ✓ |
| chain/256 | 273.5 ms ✓ | 261.4 ms ✓ |

## lambek-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| lambek-fo/16#0 | 126 µs ✓ |
| lambek-fo/16#1 | 87 µs ✗ |
| lambek-fo/16#2 | 257 µs ✓ |
| lambek-fo/16#3 | 96 µs ✓ |
| lambek-fo/16#4 | 90 µs ✗ |
| lambek-fo/64#0 | 475 µs ✓ |
| lambek-fo/64#1 | 548 µs ✗ |
| lambek-fo/64#2 | 6.7 ms ✓ |
| lambek-fo/64#3 | 409 µs ✓ |
| lambek-fo/64#4 | 413 µs ✗ |
| lambek-fo/256#0 | 4.0 ms ✓ |
| lambek-fo/256#1 | 9.8 ms ✗ |
| lambek-fo/256#2 | 427.2 ms ✓ |
| lambek-fo/256#3 | 4.3 ms ✓ |
| lambek-fo/256#4 | 5.3 ms ✗ |
| lambek-fo/512#0 | 12.9 ms ? depth |
| lambek-fo/512#1 | 46.2 ms ✗ |
| lambek-fo/512#2 | 3.85 s ? depth |
| lambek-fo/512#3 | 18.7 ms ✓ |
| lambek-fo/512#4 | 23.9 ms ✗ |

## chain-fo

| problem | families: classical auto j1 |
|---|--:|
| chain-fo/16#0 | 376 µs ✓ |
| chain-fo/16#1 | 351 µs ? bound |
| chain-fo/64#0 | 7.4 ms ✓ |
| chain-fo/64#1 | 7.0 ms ? bound |
| chain-fo/256#0 | 308.7 ms ✓ |
| chain-fo/256#1 | 276.7 ms ? bound |
| chain-fo/512#0 | 1.18 s ? depth |
| chain-fo/512#1 | 1.04 s ? depth |

## witness-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| witness-fo/4#0 | 63 µs ✓ |
| witness-fo/4#1 | 54 µs ✗ |
| witness-fo/4#2 | 54 µs ✓ |
| witness-fo/4#3 | 75 µs ✗ |
| witness-fo/8#0 | 65 µs ✓ |
| witness-fo/8#1 | 60 µs ✗ |
| witness-fo/8#2 | 60 µs ✓ |
| witness-fo/8#3 | 27.6 ms ✗ |
| witness-fo/10#0 | 71 µs ✓ |
| witness-fo/10#1 | 63 µs ✗ |
| witness-fo/10#2 | 67 µs ✓ |
| witness-fo/10#3 | 2.52 s ✗ |
| witness-fo/12#0 | 75 µs ✓ |
| witness-fo/12#1 | 69 µs ✗ |
| witness-fo/12#2 | 71 µs ✓ |
| witness-fo/12#3 | > 300 s |

## contraction

| problem | families: intuitionistic auto j1 |
|---|--:|
| contraction/2#0 | 57 µs ✓ |
| contraction/2#1 | 76 µs ? bound |
| contraction/4#0 | 153 µs ✓ |
| contraction/4#1 | 484 µs ? bound |
| contraction/8#0 | 64.7 ms ✓ |
| contraction/8#1 | 74.8 ms ✗ |

## pigeonhole

| problem | families: intuitionistic-affine auto j1 |
|---|--:|
| pigeonhole/2#0 | 79 µs ✓ |
| pigeonhole/2#1 | 43 µs ✗ |
| pigeonhole/3#0 | 934 µs ✓ |
| pigeonhole/3#1 | 82 µs ✗ |
| pigeonhole/4#0 | 38.0 ms ✓ |
| pigeonhole/4#1 | 282 µs ✗ |

## euler

| problem | families: intuitionistic auto j1 |
|---|--:|
| euler/5#0 | 58 µs ✓ |
| euler/5#1 | 153 µs ✗ |
| euler/6#0 | 66 µs ✓ |
| euler/6#1 | 317 µs ✗ |
| euler/8#0 | 79 µs ✓ |
| euler/8#1 | 1.6 ms ✗ |

## duplicate

| problem | families: intuitionistic auto j1 | families: classical auto j1 |
|---|--:|--:|
| duplicate/2#0 | 30 µs ✓ |  |
| duplicate/4#0 | 32 µs ✓ |  |
| duplicate/8#0 | 32 µs ✓ |  |
| duplicate/2#1 |  | 29 µs ✓ |
| duplicate/4#1 |  | 35 µs ✓ |
| duplicate/8#1 |  | 33 µs ✓ |

## reach-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| reach-fo/2#0 | 115 µs ✓ |
| reach-fo/2#1 | 129 µs ? bound |
| reach-fo/4#0 | 1.3 ms ✓ |
| reach-fo/4#1 | 1.7 ms ? bound |

## Problems/LCL

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| LCL181+1.p | 42 µs ✓ | 33 µs ✓ | 99 µs ✓ | 12 µs ✓ | 41 µs ✓ | 110 µs ✗ | 20 µs ✗ | 46 µs ✗ | 44 µs ✗ | 114 µs ✗ | 18 µs ✗ | 49 µs ✗ | 52 µs ✗ |
| LCL230+1.p | 47 µs ✓ | 29 µs ✓ | 92 µs ✓ | 13 µs ✓ | 39 µs ✓ | 60 µs ✗ | 18 µs ✗ | 40 µs ✗ | 42 µs ✗ | 61 µs ✗ | 18 µs ✗ | 40 µs ✗ | 41 µs ✗ |

## Problems/SYJ

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ101+1.p | 32 µs ✓ | 20 µs ✓ | 62 µs ✓ | 13 µs ✓ | 21 µs ✓ | 26 µs ✓ | 16 µs ✓ | 31 µs ✓ | 32 µs ✓ | 29 µs ✓ | 16 µs ✓ | 24 µs ✓ | 30 µs ✓ |
| SYJ102+1.p | 39 µs ✓ | 25 µs ✓ | 67 µs ✓ | 13 µs ✓ | 26 µs ✓ | 44 µs ✓ | 21 µs ✓ | 31 µs ✓ | 41 µs ✓ | 99 µs ✓ | 21 µs ✓ | 33 µs ✓ | 45 µs ✓ |
| SYJ103+1.p | 37 µs ✓ | 27 µs ✓ | 76 µs ✓ | 11 µs ✓ | 37 µs ✓ | 73 µs ✓ | 24 µs ✓ | 36 µs ✓ | 45 µs ✓ | 127 µs ✓ | 24 µs ✓ | 35 µs ✓ | 57 µs ✓ |
| SYJ104+1.p | 36 µs ✓ | 21 µs ✓ | 72 µs ✓ | 13 µs ✓ | 20 µs ✓ | 103 µs ✓ | 18 µs ✓ | 26 µs ✓ | 38 µs ✓ | 107 µs ✓ | 17 µs ✓ | 27 µs ✓ | 35 µs ✓ |
| SYJ106+1.p | 52 µs ✓ | 38 µs ✓ | 171 µs ✓ | 16 µs ✓ | 77 µs ✓ | 150 µs ✓ | 45 µs ✓ | 84 µs ✓ | 130 µs ✓ | 190 µs ✗ | 39 µs ✗ | 55 µs ✗ | 54 µs ✗ |

## SYJ105

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ105+1.002.p | 43 µs ✓ | 29 µs ✓ | 63 µs ✓ | 11 µs ✓ | 30 µs ✓ | 62 µs ✓ | 28 µs ✓ | 49 µs ✓ | 61 µs ✓ | 55 µs ✓ | 29 µs ✓ | 46 µs ✓ | 66 µs ✓ |
| SYJ105+1.003.p | 34 µs ✓ | 29 µs ✓ | 80 µs ✓ | 12 µs ✓ | 34 µs ✓ | 85 µs ✓ | 52 µs ✓ | 70 µs ✓ | 104 µs ✓ | 85 µs ✓ | 49 µs ✓ | 77 µs ✓ | 123 µs ✓ |
| SYJ105+1.004.p | 39 µs ✓ | 35 µs ✓ | 110 µs ✓ | 15 µs ✓ | 50 µs ✓ | 147 µs ✓ | 65 µs ✓ | 106 µs ✓ | 157 µs ✓ | 145 µs ✓ | 66 µs ✓ | 115 µs ✓ | 160 µs ✓ |

## SYJ107

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ107+1.001.p | 45 µs ✓ | 34 µs ✓ | 91 µs ✓ | 12 µs ✓ | 37 µs ✓ | 67 µs ✓ | 29 µs ✓ | 41 µs ✓ | 58 µs ✓ | 77 µs ✓ | 29 µs ✓ | 45 µs ✓ | 74 µs ✓ |
| SYJ107+1.002.p | 57 µs ✓ | 34 µs ✓ | 246 µs ✓ | 13 µs ✓ | 96 µs ✓ | 306 µs ✓ | 48 µs ✓ | 66 µs ✓ | 107 µs ✓ | 315 µs ✓ | 51 µs ✓ | 68 µs ✓ | 115 µs ✓ |
| SYJ107+1.003.p | 176 µs ✓ | 38 µs ✓ | 1.6 ms ✓ | 21 µs ✓ | 344 µs ✓ | 2.3 ms ✓ | 73 µs ✓ | 95 µs ✓ | 178 µs ✓ | 2.3 ms ✓ | 67 µs ✓ | 95 µs ✓ | 169 µs ✓ |
| SYJ107+1.004.p | 1.2 ms ✓ | 46 µs ✓ | 14.9 ms ✓ | 53 µs ✓ | 1.6 ms ✓ | 21.9 ms ✓ | 172 µs ✓ | 181 µs ✓ | 336 µs ✓ | 22.2 ms ✓ | 171 µs ✓ | 167 µs ✓ | 327 µs ✓ |

## SYJ201

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ201+1.001.p | 84 µs ✓ | 35 µs ✓ | 469 µs ✓ | 13 µs ✓ | 99 µs ✓ | 153 µs ✓ | 235 µs ✓ | 94 µs ✓ | 340 µs ✓ | 147 µs ✓ | 229 µs ✓ | 96 µs ✓ | 344 µs ✓ |
| SYJ201+1.002.p | 762 µs ✓ | 46 µs ✓ | 10.0 ms ✓ | 17 µs ✓ | 310 µs ✓ | 1.3 ms ✓ | 818 µs ✓ | 166 µs ✓ | 966 µs ✓ | 1.3 ms ✓ | 823 µs ✓ | 153 µs ✓ | 946 µs ✓ |
| SYJ201+1.003.p | 14.0 ms ✓ | 52 µs ✓ | 18.3 ms ✓ | 31 µs ✓ | 765 µs ✓ | 5.5 ms ✓ | 2.0 ms ✓ | 220 µs ✓ | 2.4 ms ✓ | 5.5 ms ✓ | 2.1 ms ✓ | 220 µs ✓ | 2.4 ms ✓ |
| SYJ201+1.004.p | 231.8 ms ✓ | 69 µs ✓ | 18.0 ms ✓ | 89 µs ✓ | 1.5 ms ✓ | 180.0 ms ✓ | 4.4 ms ✓ | 281 µs ✓ | 3.6 ms ✓ | 180.4 ms ✓ | 4.5 ms ✓ | 287 µs ✓ | 3.7 ms ✓ |
| SYJ201+1.005.p | 1.58 s ? memory | 67 µs ✓ | 18.5 ms ✓ | 12 µs unknown | 2.3 ms ✓ | 713.3 ms ✓ | 7.9 ms ✓ | 348 µs ✓ | 3.6 ms ✓ | 713.5 ms ✓ | 8.1 ms ✓ | 340 µs ✓ | 3.8 ms ✓ |
| SYJ201+1.006.p | 1.69 s ? memory | 62 µs ✓ | 18.4 ms ✓ | 13 µs unknown | 3.8 ms ✓ | > 2 s | 13.3 ms ✓ | 451 µs ✓ | 3.7 ms ✓ | > 2 s | 12.5 ms ✓ | 424 µs ✓ | 3.7 ms ✓ |
| SYJ201+1.007.p | > 2 s | 66 µs ✓ | 18.8 ms ✓ | 11 µs unknown | 5.9 ms ✓ | > 2 s | 21.4 ms ✓ | 571 µs ✓ | 5.0 ms ✓ | > 2 s | 21.4 ms ✓ | 568 µs ✓ | 4.9 ms ✓ |
| SYJ201+1.008.p | 1.07 s ? memory | 78 µs ✓ | 18.3 ms ✓ | 14 µs unknown | 8.8 ms ✓ | > 2 s | 31.7 ms ✓ | 607 µs ✓ | 5.6 ms ✓ | > 2 s | 31.2 ms ✓ | 633 µs ✓ | 5.6 ms ✓ |
| SYJ201+1.009.p | 1.15 s ? memory | 77 µs ✓ | 17.6 ms ✓ | 13 µs unknown | 12.1 ms ✓ | > 2 s | 45.4 ms ✓ | 699 µs ✓ | 6.8 ms ✓ | > 2 s | 45.1 ms ✓ | 752 µs ✓ | 6.8 ms ✓ |
| SYJ201+1.010.p | 1.27 s ? memory | 93 µs ✓ | 17.5 ms ✓ | 13 µs unknown | 16.9 ms ✓ | > 2 s | 63.6 ms ✓ | 782 µs ✓ | 8.1 ms ✓ | > 2 s | 64.0 ms ✓ | 770 µs ✓ | 8.3 ms ✓ |
| SYJ201+1.011.p | 1.48 s ? memory | 106 µs ✓ | 17.4 ms ✓ | 14 µs unknown | 22.8 ms ✓ | > 2 s | 87.0 ms ✓ | 889 µs ✓ | 9.9 ms ✓ | > 2 s | 87.8 ms ✓ | 916 µs ✓ | 9.8 ms ✓ |
| SYJ201+1.012.p | 1.55 s ? memory | 92 µs ✓ | 17.6 ms ✓ | 14 µs unknown | 30.7 ms ✓ | > 2 s | 116.4 ms ✓ | 1.1 ms ✓ | 12.1 ms ✓ | > 2 s | 116.0 ms ✓ | 1.1 ms ✓ | 12.1 ms ✓ |
| SYJ201+1.013.p | 1.67 s ? memory | 105 µs ✓ | 17.5 ms ✓ | 14 µs unknown | 14.5 ms unknown | > 2 s | 153.2 ms ✓ | 1.3 ms ✓ | 14.1 ms ✓ | > 2 s | 153.2 ms ✓ | 1.3 ms ✓ | 14.2 ms ✓ |
| SYJ201+1.014.p | > 2 s | 99 µs ✓ | 17.9 ms ✓ | 14 µs unknown | 19.4 ms unknown | > 2 s | 200.7 ms ✓ | 1.4 ms ✓ | 20.7 ms ✓ | > 2 s | 200.6 ms ✓ | 1.4 ms ✓ | 20.4 ms ✓ |
| SYJ201+1.015.p | > 2 s | 98 µs ✓ | 18.2 ms ✓ | 12 µs unknown | 23.1 ms unknown | > 2 s | 252.2 ms ✓ | 1.5 ms ✓ | 24.1 ms ✓ | > 2 s | 253.5 ms ✓ | 1.5 ms ✓ | 23.8 ms ✓ |
| SYJ201+1.016.p | 967.2 ms ? memory | 110 µs ✓ | 17.5 ms ✓ | 15 µs unknown | 29.1 ms unknown | > 2 s | 316.7 ms ✓ | 1.6 ms ✓ | 28.6 ms ✓ | > 2 s | 317.6 ms ✓ | 1.6 ms ✓ | 28.6 ms ✓ |
| SYJ201+1.017.p | 1.06 s ? memory | 109 µs ✓ | 17.2 ms ✓ | 15 µs unknown | 35.8 ms unknown | > 2 s | 392.0 ms ✓ | 1.8 ms ✓ | 32.2 ms ✓ | > 2 s | 393.2 ms ✓ | 1.8 ms ✓ | 32.2 ms ✓ |
| SYJ201+1.018.p | 1.09 s ? memory | 117 µs ✓ | 16.7 ms ✓ | 12 µs unknown | 41.8 ms unknown | > 2 s | 482.2 ms ✓ | 2.1 ms ✓ | 35.7 ms ✓ | > 2 s | 481.7 ms ✓ | 2.0 ms ✓ | 35.6 ms ✓ |
| SYJ201+1.019.p | 1.14 s ? memory | 118 µs ✓ | 16.8 ms ✓ | 13 µs unknown | 49.8 ms unknown | > 2 s | 583.6 ms ✓ | 2.2 ms ✓ | 40.3 ms ✓ | > 2 s | 586.2 ms ✓ | 2.2 ms ✓ | 40.0 ms ✓ |
| SYJ201+1.020.p | 1.21 s ? memory | 123 µs ✓ | 16.6 ms ✓ | 14 µs unknown | 58.8 ms unknown | > 2 s | 703.7 ms ✓ | 2.4 ms ✓ | 46.5 ms ✓ | > 2 s | 705.2 ms ✓ | 2.4 ms ✓ | 47.0 ms ✓ |

## SYJ202

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ202+1.001.p | 37 µs ✓ | 28 µs ✓ | 67 µs ✓ | 14 µs ✓ | 31 µs ✓ | 42 µs ✓ | 19 µs ✓ | 31 µs ✓ | 42 µs ✓ | 41 µs ✓ | 21 µs ✓ | 32 µs ✓ | 37 µs ✓ |
| SYJ202+1.002.p | 286 µs ✓ | 44 µs ✓ | 2.5 ms ✓ | 19 µs ✓ | 291 µs ✓ | 392 µs ✓ | 62 µs ✓ | 58 µs ✓ | 115 µs ✓ | 386 µs ✓ | 61 µs ✓ | 71 µs ✓ | 118 µs ✓ |
| SYJ202+1.003.p | 1.99 s ? memory | 75 µs ✓ | 16.6 ms ✓ | 13 µs unknown | 197.5 ms unknown | 83.5 ms ✓ | 754 µs ✓ | 263 µs ✓ | 1.1 ms ✓ | 83.3 ms ✓ | 805 µs ✓ | 256 µs ✓ | 1.1 ms ✓ |
| SYJ202+1.004.p | 1.57 s ? memory | 170 µs ✓ | 18.4 ms ✓ | 12 µs unknown | 117.5 ms ? memory | > 2 s | 41.6 ms ✓ | 2.3 ms ✓ | 11.1 ms ✓ | > 2 s | 41.7 ms ✓ | 2.3 ms ✓ | 11.2 ms ✓ |
| SYJ202+1.005.p | 1.39 s ? memory | 679 µs ✓ | 25.6 ms ✓ | 14 µs unknown | 66.9 ms ? memory | > 2 s | > 2 s | 25.4 ms ✓ | 35.0 ms ✓ | > 2 s | > 2 s | 26.1 ms ✓ | 35.5 ms ✓ |
| SYJ202+1.006.p | 1.09 s ? memory | 2.2 ms ✓ | 110.7 ms ✓ | 11 µs unknown | 27.9 ms ? memory | > 2 s | > 2 s | 329.1 ms ✓ | 372.3 ms ✓ | > 2 s | > 2 s | 300.4 ms ✓ | 414.5 ms ✓ |
| SYJ202+1.007.p | 1.71 s ? memory | 8.8 ms ✓ | 130.6 ms ✓ | 12 µs unknown | 18.4 ms ? memory | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s |
| SYJ202+1.008.p | 1.18 s ? memory | 42.2 ms ✓ | 160.0 ms ✓ | 14 µs unknown | 13.6 ms ? memory | > 2 s | 1.63 s ? memory | > 2 s | > 2 s | > 2 s | 1.69 s ? memory | > 2 s | > 2 s |
| SYJ202+1.009.p | 1.73 s ? memory | 183.4 ms ✓ | 300.4 ms ✓ | 12 µs unknown | 9.8 ms ? memory | > 2 s | 1.64 s ? memory | > 2 s | > 2 s | > 2 s | 1.52 s ? memory | > 2 s | > 2 s |
| SYJ202+1.010.p | 1.13 s ? memory | 1.29 s ✓ | 1.41 s ✓ | 12 µs unknown | 8.2 ms ? memory | 89.5 ms ? depth | 1.61 s ? memory | > 2 s | > 2 s | 89.6 ms ? depth | 1.62 s ? memory | > 2 s | > 2 s |
| SYJ202+1.011.p | 1.47 s ? memory | > 2 s | > 2 s | 14 µs unknown | 7.2 ms ? memory | 88.6 ms ? depth | 1.57 s ? memory | > 2 s | > 2 s | 88.3 ms ? depth | 1.56 s ? memory | > 2 s | > 2 s |
| SYJ202+1.012.p | > 2 s | > 2 s | > 2 s | 14 µs unknown | 7.7 ms ? memory | 87.2 ms ? depth | 1.54 s ? memory | > 2 s | > 2 s | 87.6 ms ? depth | 1.53 s ? memory | > 2 s | > 2 s |
| SYJ202+1.013.p | 1.26 s ? memory | > 2 s | > 2 s | 13 µs unknown | 9.0 ms ? memory | 86.4 ms ? depth | 1.50 s ? memory | > 2 s | > 2 s | 82.5 ms ? depth | 1.50 s ? memory | > 2 s | > 2 s |
| SYJ202+1.014.p | 1.50 s ? memory | > 2 s | > 2 s | 13 µs unknown | 11.4 ms ? memory | 84.3 ms ? depth | 1.53 s ? memory | > 2 s | > 2 s | 82.1 ms ? depth | 1.53 s ? memory | > 2 s | > 2 s |
| SYJ202+1.015.p | > 2 s | > 2 s | > 2 s | 14 µs unknown | 15.4 ms ? memory | 86.1 ms ? depth | 1.45 s ? memory | > 2 s | > 2 s | 82.7 ms ? depth | 1.44 s ? memory | > 2 s | > 2 s |
| SYJ202+1.016.p | 94.1 ms ? depth | > 2 s | > 2 s | 14 µs unknown | 21.7 ms ? memory | 82.3 ms ? depth | 1.49 s ? memory | > 2 s | > 2 s | 84.8 ms ? depth | 1.50 s ? memory | > 2 s | > 2 s |
| SYJ202+1.017.p | 95.8 ms ? depth † | > 2 s | > 2 s | 15 µs unknown | 30.4 ms ? memory | 83.2 ms ? depth | 1.47 s ? memory | > 2 s | > 2 s | 82.3 ms ? depth | 1.52 s ? memory | > 2 s | > 2 s |
| SYJ202+1.018.p | 94.9 ms ? depth | > 2 s | > 2 s | 15 µs unknown | 42.8 ms ? memory | 84.5 ms ? depth | 1.61 s ? memory | > 2 s | > 2 s | 82.0 ms ? depth | 1.58 s ? memory | > 2 s | > 2 s |
| SYJ202+1.019.p | 94.7 ms ? depth | > 2 s | > 2 s | 14 µs unknown | 59.2 ms ? memory | 82.8 ms ? depth | 1.54 s ? memory | > 2 s | > 2 s | 81.4 ms ? depth | 1.47 s ? memory | > 2 s | > 2 s |
| SYJ202+1.020.p | 92.9 ms ? depth | > 2 s | > 2 s | 15 µs unknown | 82.0 ms ? memory | 81.1 ms ? depth | 1.47 s ? memory | > 2 s | > 2 s | 83.8 ms ? depth | 1.47 s ? memory | > 2 s | > 2 s |

## SYJ203

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ203+1.001.p | 36 µs ✓ | 32 µs ✓ | 82 µs ✓ | 13 µs ✓ | 34 µs ✓ | 62 µs ✓ | 28 µs ✓ | 39 µs ✓ | 50 µs ✓ | 51 µs ✓ | 24 µs ✓ | 33 µs ✓ | 51 µs ✓ |
| SYJ203+1.002.p | 37 µs ✓ | 38 µs ✓ | 107 µs ✓ | 16 µs ✓ | 41 µs ✓ | 81 µs ✓ | 42 µs ✓ | 52 µs ✓ | 78 µs ✓ | 75 µs ✓ | 42 µs ✓ | 47 µs ✓ | 82 µs ✓ |
| SYJ203+1.003.p | 43 µs ✓ | 36 µs ✓ | 118 µs ✓ | 16 µs ✓ | 52 µs ✓ | 148 µs ✓ | 57 µs ✓ | 60 µs ✓ | 111 µs ✓ | 141 µs ✓ | 59 µs ✓ | 64 µs ✓ | 116 µs ✓ |
| SYJ203+1.004.p | 42 µs ✓ | 39 µs ✓ | 135 µs ✓ | 15 µs ✓ | 63 µs ✓ | 331 µs ✓ | 75 µs ✓ | 73 µs ✓ | 139 µs ✓ | 329 µs ✓ | 72 µs ✓ | 72 µs ✓ | 142 µs ✓ |
| SYJ203+1.005.p | 42 µs ✓ | 37 µs ✓ | 149 µs ✓ | 20 µs ✓ | 76 µs ✓ | 1.0 ms ✓ | 85 µs ✓ | 81 µs ✓ | 169 µs ✓ | 1.0 ms ✓ | 87 µs ✓ | 92 µs ✓ | 171 µs ✓ |
| SYJ203+1.006.p | 42 µs ✓ | 38 µs ✓ | 173 µs ✓ | 24 µs ✓ | 97 µs ✓ | 3.7 ms ✓ | 107 µs ✓ | 93 µs ✓ | 203 µs ✓ | 3.6 ms ✓ | 103 µs ✓ | 104 µs ✓ | 203 µs ✓ |
| SYJ203+1.007.p | 45 µs ✓ | 41 µs ✓ | 215 µs ✓ | 42 µs ✓ | 114 µs ✓ | 13.9 ms ✓ | 130 µs ✓ | 115 µs ✓ | 245 µs ✓ | 14.0 ms ✓ | 136 µs ✓ | 125 µs ✓ | 249 µs ✓ |
| SYJ203+1.008.p | 58 µs ✓ | 47 µs ✓ | 246 µs ✓ | 74 µs ✓ | 135 µs ✓ | 52.8 ms ✓ | 145 µs ✓ | 146 µs ✓ | 289 µs ✓ | 53.1 ms ✓ | 151 µs ✓ | 139 µs ✓ | 290 µs ✓ |
| SYJ203+1.009.p | 54 µs ✓ | 47 µs ✓ | 314 µs ✓ | 145 µs ✓ | 177 µs ✓ | 125.7 ms ✓ | 183 µs ✓ | 173 µs ✓ | 349 µs ✓ | 126.5 ms ✓ | 170 µs ✓ | 158 µs ✓ | 346 µs ✓ |
| SYJ203+1.010.p | 54 µs ✓ | 44 µs ✓ | 332 µs ✓ | 10 µs unknown | 189 µs ✓ | 127.8 ms ✓ | 222 µs ✓ | 182 µs ✓ | 391 µs ✓ | 133.4 ms ✓ | 211 µs ✓ | 187 µs ✓ | 428 µs ✓ |
| SYJ203+1.011.p | 55 µs ✓ | 48 µs ✓ | 438 µs ✓ | 9 µs unknown | 225 µs ✓ | 130.3 ms ✓ | 237 µs ✓ | 208 µs ✓ | 480 µs ✓ | 129.4 ms ✓ | 235 µs ✓ | 233 µs ✓ | 466 µs ✓ |
| SYJ203+1.012.p | 65 µs ✓ | 51 µs ✓ | 423 µs ✓ | 10 µs unknown | 280 µs ✓ | 137.8 ms ✓ | 270 µs ✓ | 223 µs ✓ | 520 µs ✓ | 138.5 ms ✓ | 250 µs ✓ | 239 µs ✓ | 519 µs ✓ |
| SYJ203+1.013.p | 76 µs ✓ | 58 µs ✓ | 538 µs ✓ | 10 µs unknown | 300 µs ✓ | 142.1 ms ✓ | 296 µs ✓ | 263 µs ✓ | 578 µs ✓ | 151.9 ms ✓ | 286 µs ✓ | 274 µs ✓ | 584 µs ✓ |
| SYJ203+1.014.p | 69 µs ✓ | 60 µs ✓ | 550 µs ✓ | 12 µs unknown | 349 µs ✓ | 137.5 ms ✓ | 306 µs ✓ | 267 µs ✓ | 628 µs ✓ | 140.3 ms ✓ | 307 µs ✓ | 305 µs ✓ | 613 µs ✓ |
| SYJ203+1.015.p | 71 µs ✓ | 59 µs ✓ | 612 µs ✓ | 9 µs unknown | 377 µs ✓ | 140.7 ms ✓ | 350 µs ✓ | 306 µs ✓ | 687 µs ✓ | 140.3 ms ✓ | 355 µs ✓ | 308 µs ✓ | 695 µs ✓ |
| SYJ203+1.016.p | 71 µs ✓ | 58 µs ✓ | 695 µs ✓ | 10 µs unknown | 431 µs ✓ | 141.7 ms ✓ | 421 µs ✓ | 326 µs ✓ | 775 µs ✓ | 148.1 ms ✓ | 402 µs ✓ | 352 µs ✓ | 792 µs ✓ |
| SYJ203+1.017.p | 84 µs ✓ | 60 µs ✓ | 756 µs ✓ | 9 µs unknown | 508 µs ✓ | 148.6 ms ✓ | 447 µs ✓ | 367 µs ✓ | 902 µs ✓ | 153.5 ms ✓ | 434 µs ✓ | 394 µs ✓ | 920 µs ✓ |
| SYJ203+1.018.p | 91 µs ✓ | 69 µs ✓ | 864 µs ✓ | 13 µs unknown | 592 µs ✓ | 164.6 ms ✓ | 462 µs ✓ | 406 µs ✓ | 916 µs ✓ | 147.9 ms ✓ | 458 µs ✓ | 427 µs ✓ | 943 µs ✓ |
| SYJ203+1.019.p | 95 µs ✓ | 60 µs ✓ | 896 µs ✓ | 8 µs unknown | 631 µs ✓ | 150.7 ms ✓ | 514 µs ✓ | 441 µs ✓ | 1.0 ms ✓ | 152.4 ms ✓ | 528 µs ✓ | 467 µs ✓ | 1.0 ms ✓ |
| SYJ203+1.020.p | 93 µs ✓ | 64 µs ✓ | 1.0 ms ✓ | 9 µs unknown | 698 µs ✓ | 150.2 ms ✓ | 561 µs ✓ | 478 µs ✓ | 1.1 ms ✓ | 151.7 ms ✓ | 535 µs ✓ | 503 µs ✓ | 1.2 ms ✓ |

## SYJ204

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ204+1.001.p | 37 µs ✓ | 21 µs ✓ | 70 µs ✓ | 11 µs ✓ | 26 µs ✓ | 103 µs ✓ | 19 µs ✓ | 28 µs ✓ | 42 µs ✓ | 106 µs ✓ | 19 µs ✓ | 28 µs ✓ | 38 µs ✓ |
| SYJ204+1.002.p | 38 µs ✓ | 23 µs ✓ | 95 µs ✓ | 11 µs ✓ | 40 µs ✓ | 102 µs ✓ | 23 µs ✓ | 31 µs ✓ | 42 µs ✓ | 106 µs ✓ | 25 µs ✓ | 32 µs ✓ | 48 µs ✓ |
| SYJ204+1.003.p | 44 µs ✓ | 28 µs ✓ | 150 µs ✓ | 10 µs ✓ | 64 µs ✓ | 106 µs ✓ | 26 µs ✓ | 34 µs ✓ | 52 µs ✓ | 110 µs ✓ | 28 µs ✓ | 35 µs ✓ | 54 µs ✓ |
| SYJ204+1.004.p | 62 µs ✓ | 30 µs ✓ | 333 µs ✓ | 15 µs ✓ | 123 µs ✓ | 130 µs ✓ | 30 µs ✓ | 36 µs ✓ | 59 µs ✓ | 113 µs ✓ | 30 µs ✓ | 37 µs ✓ | 61 µs ✓ |
| SYJ204+1.005.p | 121 µs ✓ | 30 µs ✓ | 848 µs ✓ | 14 µs ✓ | 241 µs ✓ | 135 µs ✓ | 35 µs ✓ | 41 µs ✓ | 68 µs ✓ | 118 µs ✓ | 32 µs ✓ | 40 µs ✓ | 66 µs ✓ |
| SYJ204+1.006.p | 272 µs ✓ | 32 µs ✓ | 2.8 ms ✓ | 20 µs ✓ | 485 µs ✓ | 144 µs ✓ | 35 µs ✓ | 43 µs ✓ | 71 µs ✓ | 127 µs ✓ | 36 µs ✓ | 40 µs ✓ | 70 µs ✓ |
| SYJ204+1.007.p | 830 µs ✓ | 38 µs ✓ | 9.1 ms ✓ | 31 µs ✓ | 1.1 ms ✓ | 148 µs ✓ | 38 µs ✓ | 46 µs ✓ | 76 µs ✓ | 143 µs ✓ | 41 µs ✓ | 46 µs ✓ | 78 µs ✓ |
| SYJ204+1.008.p | 2.4 ms ✓ | 40 µs ✓ | 33.3 ms ✓ | 56 µs ✓ | 2.4 ms ✓ | 165 µs ✓ | 41 µs ✓ | 45 µs ✓ | 78 µs ✓ | 149 µs ✓ | 42 µs ✓ | 46 µs ✓ | 78 µs ✓ |
| SYJ204+1.009.p | 8.2 ms ✓ | 44 µs ✓ | 14.6 ms ✓ | 94 µs ✓ | 5.4 ms ✓ | 164 µs ✓ | 47 µs ✓ | 49 µs ✓ | 84 µs ✓ | 172 µs ✓ | 43 µs ✓ | 47 µs ✓ | 82 µs ✓ |
| SYJ204+1.010.p | 25.6 ms ✓ | 46 µs ✓ | 15.3 ms ✓ | 14 µs unknown | 12.7 ms ✓ | 182 µs ✓ | 49 µs ✓ | 48 µs ✓ | 91 µs ✓ | 191 µs ✓ | 50 µs ✓ | 49 µs ✓ | 90 µs ✓ |
| SYJ204+1.011.p | 87.0 ms ✓ | 48 µs ✓ | 15.1 ms ✓ | 13 µs unknown | 27.8 ms ✓ | 215 µs ✓ | 58 µs ✓ | 52 µs ✓ | 98 µs ✓ | 208 µs ✓ | 56 µs ✓ | 54 µs ✓ | 98 µs ✓ |
| SYJ204+1.012.p | 300.7 ms ✓ | 46 µs ✓ | 15.2 ms ✓ | 14 µs unknown | 63.3 ms ✓ | 235 µs ✓ | 61 µs ✓ | 53 µs ✓ | 104 µs ✓ | 244 µs ✓ | 59 µs ✓ | 58 µs ✓ | 105 µs ✓ |
| SYJ204+1.013.p | 1.01 s ✓ | 52 µs ✓ | 15.6 ms ✓ | 13 µs unknown | 63.5 ms unknown | 271 µs ✓ | 61 µs ✓ | 57 µs ✓ | 110 µs ✓ | 270 µs ✓ | 66 µs ✓ | 56 µs ✓ | 113 µs ✓ |
| SYJ204+1.014.p | 1.90 s ? memory | 52 µs ✓ | 16.4 ms ✓ | 13 µs unknown | 136.9 ms unknown | 297 µs ✓ | 69 µs ✓ | 62 µs ✓ | 120 µs ✓ | 304 µs ✓ | 65 µs ✓ | 62 µs ✓ | 116 µs ✓ |
| SYJ204+1.015.p | 1.50 s ? memory | 52 µs ✓ | 16.8 ms ✓ | 12 µs unknown | 291.3 ms unknown | 338 µs ✓ | 74 µs ✓ | 63 µs ✓ | 128 µs ✓ | 341 µs ✓ | 69 µs ✓ | 66 µs ✓ | 132 µs ✓ |
| SYJ204+1.016.p | 1.59 s ? memory | 60 µs ✓ | 15.0 ms ✓ | 13 µs unknown | 621.1 ms unknown | 381 µs ✓ | 80 µs ✓ | 71 µs ✓ | 143 µs ✓ | 380 µs ✓ | 84 µs ✓ | 70 µs ✓ | 139 µs ✓ |
| SYJ204+1.017.p | 1.67 s ? memory | 55 µs ✓ | 14.7 ms ✓ | 13 µs unknown | 173.0 ms ? memory | 712 µs ✓ | 79 µs ✓ | 70 µs ✓ | 137 µs ✓ | 713 µs ✓ | 76 µs ✓ | 68 µs ✓ | 143 µs ✓ |
| SYJ204+1.018.p | 1.71 s ? memory | 56 µs ✓ | 15.6 ms ✓ | 12 µs unknown | 164.3 ms ? memory | 761 µs ✓ | 84 µs ✓ | 69 µs ✓ | 143 µs ✓ | 718 µs ✓ | 79 µs ✓ | 69 µs ✓ | 146 µs ✓ |
| SYJ204+1.019.p | 1.77 s ? memory | 55 µs ✓ | 16.2 ms ✓ | 14 µs unknown | 155.5 ms ? memory | 773 µs ✓ | 93 µs ✓ | 70 µs ✓ | 157 µs ✓ | 785 µs ✓ | 92 µs ✓ | 73 µs ✓ | 153 µs ✓ |
| SYJ204+1.020.p | 1.90 s ? memory | 65 µs ✓ | 15.4 ms ✓ | 14 µs unknown | 151.7 ms ? memory | 865 µs ✓ | 92 µs ✓ | 71 µs ✓ | 158 µs ✓ | 834 µs ✓ | 92 µs ✓ | 76 µs ✓ | 160 µs ✓ |

## SYJ205

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ205+1.001.p | 59 µs ✓ | 53 µs ✓ | 186 µs ✓ | 17 µs ✓ | 85 µs ✓ | 135 µs ✓ | 52 µs ✓ | 64 µs ✓ | 107 µs ✓ | 152 µs ✓ | 54 µs ✓ | 69 µs ✓ | 106 µs ✓ |
| SYJ205+1.002.p | 68 µs ✓ | 51 µs ✓ | 278 µs ✓ | 25 µs ✓ | 118 µs ✓ | 196 µs ✓ | 67 µs ✓ | 68 µs ✓ | 131 µs ✓ | 176 µs ✓ | 68 µs ✓ | 71 µs ✓ | 131 µs ✓ |
| SYJ205+1.003.p | 86 µs ✓ | 57 µs ✓ | 445 µs ✓ | 59 µs ✓ | 181 µs ✓ | 294 µs ✓ | 87 µs ✓ | 78 µs ✓ | 160 µs ✓ | 300 µs ✓ | 87 µs ✓ | 76 µs ✓ | 159 µs ✓ |
| SYJ205+1.004.p | 123 µs ✓ | 60 µs ✓ | 817 µs ✓ | 12 µs unknown | 240 µs ✓ | 1.6 ms ✓ | 117 µs ✓ | 87 µs ✓ | 203 µs ✓ | 1.6 ms ✓ | 119 µs ✓ | 86 µs ✓ | 201 µs ✓ |
| SYJ205+1.005.p | 174 µs ✓ | 58 µs ✓ | 1.7 ms ✓ | 13 µs unknown | 312 µs ✓ | 4.6 ms ✓ | 142 µs ✓ | 96 µs ✓ | 234 µs ✓ | 4.6 ms ✓ | 145 µs ✓ | 98 µs ✓ | 241 µs ✓ |
| SYJ205+1.006.p | 320 µs ✓ | 68 µs ✓ | 3.5 ms ✓ | 13 µs unknown | 408 µs ✓ | 15.2 ms ✓ | 185 µs ✓ | 105 µs ✓ | 282 µs ✓ | 15.3 ms ✓ | 192 µs ✓ | 100 µs ✓ | 278 µs ✓ |
| SYJ205+1.007.p | 609 µs ✓ | 66 µs ✓ | 7.6 ms ✓ | 13 µs unknown | 557 µs ✓ | 45.3 ms ✓ | 238 µs ✓ | 106 µs ✓ | 335 µs ✓ | 45.1 ms ✓ † | 241 µs ✓ | 119 µs ✓ | 337 µs ✓ |
| SYJ205+1.008.p | 1.2 ms ✓ | 74 µs ✓ | 18.3 ms ✓ | 12 µs unknown | 744 µs ✓ | 114.4 ms ✓ † | 276 µs ✓ | 122 µs ✓ | 390 µs ✓ | 114.1 ms ✓ † | 279 µs ✓ | 123 µs ✓ | 396 µs ✓ |
| SYJ205+1.009.p | 2.5 ms ✓ | 84 µs ✓ | 39.9 ms ✓ | 12 µs unknown | 873 µs ✓ | 113.2 ms ✓ † | 334 µs ✓ | 136 µs ✓ | 464 µs ✓ | 113.7 ms ✓ | 341 µs ✓ | 135 µs ✓ | 480 µs ✓ |
| SYJ205+1.010.p | 5.3 ms ✓ | 79 µs ✓ | 8.8 ms ✓ | 12 µs unknown | 1.1 ms ✓ | 116.4 ms ✓ † | 377 µs ✓ | 153 µs ✓ | 525 µs ✓ | 117.4 ms ✓ † | 404 µs ✓ | 154 µs ✓ | 554 µs ✓ |
| SYJ205+1.011.p | 11.7 ms ✓ | 78 µs ✓ | 16.2 ms ✓ | 12 µs unknown | 1.4 ms ✓ | 128.3 ms ✓ † | 468 µs ✓ | 146 µs ✓ | 616 µs ✓ | 120.5 ms ✓ † | 462 µs ✓ | 154 µs ✓ | 613 µs ✓ |
| SYJ205+1.012.p | 25.4 ms ✓ | 89 µs ✓ | 16.9 ms ✓ | 12 µs unknown | 1.7 ms ✓ | 124.5 ms ✓ † | 548 µs ✓ | 147 µs ✓ | 680 µs ✓ | 123.6 ms ✓ † | 528 µs ✓ | 150 µs ✓ | 693 µs ✓ |
| SYJ205+1.013.p | 53.4 ms ✓ | 85 µs ✓ | 15.7 ms ✓ | 14 µs unknown | 2.0 ms ✓ | 124.6 ms ✓ † | 617 µs ✓ | 161 µs ✓ | 775 µs ✓ | 125.1 ms ✓ † | 608 µs ✓ | 162 µs ✓ | 767 µs ✓ |
| SYJ205+1.014.p | 113.5 ms ✓ | 85 µs ✓ | 15.7 ms ✓ | 11 µs unknown | 2.4 ms ✓ | 126.0 ms ✓ † | 739 µs ✓ | 167 µs ✓ | 891 µs ✓ | 127.6 ms ✓ † | 690 µs ✓ | 182 µs ✓ | 891 µs ✓ |
| SYJ205+1.015.p | 248.6 ms ✓ | 98 µs ✓ | 14.6 ms ✓ | 11 µs unknown | 2.9 ms ✓ | 122.6 ms ✓ † | 783 µs ✓ | 179 µs ✓ | 998 µs ✓ | 123.1 ms ✓ † | 770 µs ✓ | 173 µs ✓ | 1.0 ms ✓ |
| SYJ205+1.016.p | 568.3 ms ✓ | 102 µs ✓ | 15.5 ms ✓ | 11 µs unknown | 3.4 ms ✓ | 124.3 ms ✓ † | 896 µs ✓ | 189 µs ✓ | 1.2 ms ✓ | 126.9 ms ✓ † | 891 µs ✓ | 188 µs ✓ | 1.2 ms ✓ |
| SYJ205+1.017.p | 1.24 s ✓ | 100 µs ✓ | 14.8 ms ✓ | 12 µs unknown | 3.9 ms ✓ | 128.0 ms ✓ † | 1.0 ms ✓ | 199 µs ✓ | 1.3 ms ✓ | 129.5 ms ✓ † | 1.1 ms ✓ | 212 µs ✓ | 1.3 ms ✓ |
| SYJ205+1.018.p | 1.49 s ? memory | 107 µs ✓ | 14.7 ms ✓ | 12 µs unknown | 4.4 ms ✓ | 133.4 ms ✓ † | 1.1 ms ✓ | 206 µs ✓ | 1.4 ms ✓ | 128.3 ms ✓ † | 1.1 ms ✓ | 207 µs ✓ | 1.4 ms ✓ |
| SYJ205+1.019.p | 1.51 s ? memory | 98 µs ✓ | 15.1 ms ✓ | 11 µs unknown | 5.2 ms ✓ | 132.2 ms ✓ † | 1.3 ms ✓ | 219 µs ✓ | 1.6 ms ✓ | 131.6 ms ✓ † | 1.3 ms ✓ | 218 µs ✓ | 1.6 ms ✓ |
| SYJ205+1.020.p | 1.52 s ? memory | 106 µs ✓ | 15.2 ms ✓ | 12 µs unknown | 5.9 ms ✓ | 131.0 ms ✓ † | 1.4 ms ✓ | 223 µs ✓ | 1.7 ms ✓ | 135.1 ms ✓ † | 1.4 ms ✓ | 237 µs ✓ | 1.8 ms ✓ |

## SYJ206

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ206+1.001.p | 33 µs ✓ | 24 µs ✓ | 68 µs ✓ | 12 µs ✓ | 26 µs ✓ | 38 µs ✓ | 21 µs ✓ | 32 µs ✓ | 40 µs ✓ | 39 µs ✓ | 21 µs ✓ | 31 µs ✓ | 39 µs ✓ |
| SYJ206+1.002.p | 44 µs ✓ | 30 µs ✓ | 133 µs ✓ | 12 µs ✓ | 66 µs ✓ | 115 µs ✓ | 27 µs ✓ | 46 µs ✓ | 65 µs ✓ | 119 µs ✓ | 28 µs ✓ | 46 µs ✓ | 63 µs ✓ |
| SYJ206+1.003.p | 116 µs ✓ | 39 µs ✓ | 802 µs ✓ | 13 µs ✓ | 228 µs ✓ | 150 µs ✓ | 56 µs ✓ | 121 µs ✓ | 168 µs ✓ | 149 µs ✓ | 58 µs ✓ | 117 µs ✓ | 169 µs ✓ |
| SYJ206+1.004.p | 2.5 ms ✓ | 55 µs ✓ | 27.8 ms ✓ | 14 µs ✓ | 952 µs ✓ | 325 µs ✓ | 140 µs ✓ | 340 µs ✓ | 489 µs ✓ | 311 µs ✓ | 144 µs ✓ | 378 µs ✓ | 525 µs ✓ |
| SYJ206+1.005.p | 521.6 ms ✓ | 84 µs ✓ | 16.7 ms ✓ | 18 µs ✓ | 4.4 ms ✓ | 3.7 ms ✓ † | 291 µs ✓ | 846 µs ✓ | 1.2 ms ✓ | 3.7 ms ✓ | 304 µs ✓ | 549 µs ✓ | 866 µs ✓ |
| SYJ206+1.006.p | 1.92 s ? memory | 101 µs ✓ | 15.5 ms ✓ | 16 µs ✓ | 22.7 ms ✓ | 34.9 ms ✓ † | 656 µs ✓ | 2.0 ms ✓ | 2.7 ms ✓ | 34.9 ms ✓ † | 662 µs ✓ | 1.2 ms ✓ | 1.9 ms ✓ |
| SYJ206+1.007.p | 1.86 s ? memory | 139 µs ✓ | 13.2 ms ✓ | 20 µs ✓ | 66.1 ms unknown | 538.5 ms ✓ | 1.5 ms ✓ | 3.9 ms ✓ | 5.7 ms ✓ | 563.8 ms ✓ † | 1.5 ms ✓ | 4.2 ms ✓ | 6.0 ms ✓ |
| SYJ206+1.008.p | 1.40 s ? memory | 168 µs ✓ | 11.0 ms ✓ | 26 µs ✓ | 424.3 ms unknown | > 2 s | 3.0 ms ✓ | 6.0 ms ✓ | 9.8 ms ✓ | > 2 s | 3.1 ms ✓ | 7.1 ms ✓ | 10.9 ms ✓ |
| SYJ206+1.009.p | 1.55 s ? memory | 185 µs ✓ | 7.6 ms ✓ | 38 µs ✓ | 343.6 ms ? memory | > 2 s | 6.7 ms ✓ | 8.0 ms ✓ | 16.3 ms ✓ | > 2 s | 6.6 ms ✓ | 13.8 ms ✓ | 21.6 ms ✓ |
| SYJ206+1.010.p | 1.71 s ? memory | 227 µs ✓ | 5.1 ms ✓ | 73 µs ✓ | 305.6 ms ? memory | > 2 s | 14.9 ms ✓ | 27.4 ms ✓ | 37.4 ms ✓ | > 2 s | 15.0 ms ✓ | 39.0 ms ✓ | 49.0 ms ✓ |
| SYJ206+1.011.p | 1.89 s ? memory | 297 µs ✓ | 4.7 ms ✓ | 12 µs unknown | 287.8 ms ? memory | > 2 s | 34.6 ms ✓ | 38.9 ms ✓ | 55.0 ms ✓ | > 2 s | 34.8 ms ✓ | 40.9 ms ✓ | 57.2 ms ✓ |
| SYJ206+1.012.p | 2.03 s ? memory | 291 µs ✓ | 5.6 ms ✓ | 10 µs unknown | 270.5 ms ? memory | > 2 s | 77.7 ms ✓ | 118.3 ms ✓ | 135.1 ms ✓ | > 2 s | 77.1 ms ✓ | 127.7 ms ✓ | 145.5 ms ✓ |
| SYJ206+1.013.p | > 2 s | 300 µs ✓ | 10.8 ms ✓ | 15 µs unknown | 262.2 ms ? memory | > 2 s | 168.5 ms ✓ | 268.1 ms ✓ | 286.0 ms ✓ | > 2 s | 171.1 ms ✓ | 223.3 ms ✓ | 240.1 ms ✓ |
| SYJ206+1.014.p | > 2 s | 488 µs ✓ | 22.5 ms ✓ | 13 µs unknown | 243.1 ms ? memory | > 2 s | 371.7 ms ✓ | 207.7 ms ✓ | 225.1 ms ✓ | > 2 s | 372.6 ms ✓ | 538.5 ms ✓ | 558.0 ms ✓ |
| SYJ206+1.015.p | > 2 s | 430 µs ✓ | 38.8 ms ✓ | 13 µs unknown | 239.6 ms ? memory | > 2 s | 827.5 ms ✓ | 1.76 s ✓ | 1.78 s ✓ | > 2 s | 827.4 ms ✓ | 1.04 s ✓ | 1.06 s ✓ |
| SYJ206+1.016.p | > 2 s | 446 µs ✓ | 86.7 ms ✓ | 13 µs unknown | 219.7 ms ? memory | > 2 s | 1.81 s ✓ | > 2 s | > 2 s | > 2 s | 1.81 s ✓ | > 2 s | > 2 s |
| SYJ206+1.017.p | > 2 s | 551 µs ✓ | 586 µs ✓ | 12 µs unknown | 218.6 ms ? memory | > 2 s | 1.92 s ? memory | > 2 s | > 2 s | > 2 s | 1.91 s ? memory | > 2 s | > 2 s |
| SYJ206+1.018.p | > 2 s | 457 µs ✓ | 498 µs ✓ | 10 µs unknown | 199.0 ms ? memory | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s |
| SYJ206+1.019.p | > 2 s | 522 µs ✓ | 547 µs ✓ | 10 µs unknown | 203.2 ms ? memory | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s | > 2 s |
| SYJ206+1.020.p | > 2 s | 513 µs ✓ | 539 µs ✓ | 14 µs unknown | 185.3 ms ? memory | 961.2 ms ? memory | > 2 s | > 2 s | > 2 s | 947.3 ms ? memory | > 2 s | > 2 s | > 2 s |

## SYJ207

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ207+1.001.p | 55 µs ✓ | 35 µs ✓ | 184 µs ✓ | 12 µs ✓ | 35 µs ✓ | 116 µs ✗ | 55 µs ✗ | 46 µs ✗ | 48 µs ✗ | 112 µs ✗ | 52 µs ✗ | 49 µs ✗ | 45 µs ✗ |
| SYJ207+1.002.p | 247 µs ✓ | 41 µs ✓ | 2.4 ms ✓ | 17 µs ✓ | 45 µs ✓ | 5.1 ms ✗ | 335 µs ✗ | 60 µs ✗ | 57 µs ✗ | 5.2 ms ✗ | 331 µs ✗ | 59 µs ✗ | 57 µs ✗ |
| SYJ207+1.003.p | 3.7 ms ✓ | 50 µs ✓ | 6.4 ms ✓ | 26 µs ✓ | 34 µs ✓ | 479.6 ms ✗ | 3.8 ms ✗ | 71 µs ✗ | 74 µs ✗ | 488.1 ms ✗ | 3.8 ms ✗ | 75 µs ✗ | 71 µs ✗ |
| SYJ207+1.004.p | 62.1 ms ✓ | 56 µs ✓ | 19.2 ms ✓ | 80 µs ✓ | 41 µs ✓ | > 2 s | 50.6 ms ✗ | 84 µs ✗ | 83 µs ✗ | > 2 s | 50.6 ms ✗ | 83 µs ✗ | 82 µs ✗ |
| SYJ207+1.005.p | 983.8 ms ✓ | 53 µs ✓ | 18.6 ms ✓ | 12 µs unknown | 39 µs ✓ | > 2 s | 767.8 ms ✗ | 110 µs ✗ | 99 µs ✗ | > 2 s | 767.9 ms ✗ | 102 µs ✗ | 103 µs ✗ |
| SYJ207+1.006.p | 1.79 s ? memory | 58 µs ✓ | 18.7 ms ✓ | 13 µs unknown | 42 µs ✓ | > 2 s | > 2 s | 107 µs ✗ | 115 µs ✗ | > 2 s | > 2 s | 111 µs ✗ | 106 µs ✗ |
| SYJ207+1.007.p | > 2 s | 64 µs ✓ | 19.1 ms ✓ | 13 µs unknown | 47 µs ✓ | > 2 s | > 2 s | 125 µs ✗ | 122 µs ✗ | > 2 s | > 2 s | 127 µs ✗ | 122 µs ✗ |
| SYJ207+1.008.p | 1.09 s ? memory | 69 µs ✓ | 18.9 ms ✓ | 12 µs unknown | 54 µs ✓ | > 2 s | > 2 s | 145 µs ✗ | 143 µs ✗ | > 2 s | > 2 s | 146 µs ✗ | 139 µs ✗ |
| SYJ207+1.009.p | 1.18 s ? memory | 76 µs ✓ | 17.8 ms ✓ | 13 µs unknown | 49 µs ✓ | > 2 s | > 2 s | 161 µs ✗ | 162 µs ✗ | > 2 s | > 2 s | 156 µs ✗ | 155 µs ✗ |
| SYJ207+1.010.p | 1.29 s ? memory | 77 µs ✓ | 18.2 ms ✓ | 12 µs unknown | 52 µs ✓ | > 2 s | > 2 s | 174 µs ✗ | 172 µs ✗ | > 2 s | > 2 s | 173 µs ✗ | 170 µs ✗ |
| SYJ207+1.011.p | 1.53 s ? memory | 75 µs ✓ | 18.1 ms ✓ | 12 µs unknown | 54 µs ✓ | > 2 s | > 2 s | 217 µs ✗ | 207 µs ✗ | > 2 s | > 2 s | 199 µs ✗ | 199 µs ✗ |
| SYJ207+1.012.p | 1.58 s ? memory | 83 µs ✓ | 17.9 ms ✓ | 13 µs unknown | 54 µs ✓ | > 2 s | > 2 s | 221 µs ✗ | 215 µs ✗ | > 2 s | > 2 s | 221 µs ✗ | 215 µs ✗ |
| SYJ207+1.013.p | 1.68 s ? memory | 87 µs ✓ | 18.0 ms ✓ | 11 µs unknown | 56 µs ✓ | > 2 s | > 2 s | 240 µs ✗ | 237 µs ✗ | > 2 s | > 2 s | 241 µs ✗ | 240 µs ✗ |
| SYJ207+1.014.p | > 2 s | 93 µs ✓ | 18.6 ms ✓ | 14 µs unknown | 59 µs ✓ | > 2 s | > 2 s | 261 µs ✗ | 262 µs ✗ | > 2 s | > 2 s | 255 µs ✗ | 269 µs ✗ |
| SYJ207+1.015.p | > 2 s | 92 µs ✓ | 19.2 ms ✓ | 13 µs unknown | 63 µs ✓ | > 2 s | > 2 s | 298 µs ✗ | 292 µs ✗ | > 2 s | > 2 s | 290 µs ✗ | 284 µs ✗ |
| SYJ207+1.016.p | 994.0 ms ? memory | 100 µs ✓ | 17.6 ms ✓ | 13 µs unknown | 63 µs ✓ | > 2 s | > 2 s | 306 µs ✗ | 310 µs ✗ | > 2 s | > 2 s | 310 µs ✗ | 323 µs ✗ |
| SYJ207+1.017.p | 1.08 s ? memory | 108 µs ✓ | 17.5 ms ✓ | 13 µs unknown | 64 µs ✓ | > 2 s | > 2 s | 332 µs ✗ | 342 µs ✗ | > 2 s | > 2 s | 334 µs ✗ | 356 µs ✗ |
| SYJ207+1.018.p | 1.11 s ? memory | 104 µs ✓ | 17.8 ms ✓ | 12 µs unknown | 70 µs ✓ | > 2 s | > 2 s | 371 µs ✗ | 364 µs ✗ | > 2 s | > 2 s | 369 µs ✗ | 367 µs ✗ |
| SYJ207+1.019.p | 1.17 s ? memory | 117 µs ✓ | 17.2 ms ✓ | 15 µs unknown | 72 µs ✓ | > 2 s | > 2 s | 396 µs ✗ | 396 µs ✗ | > 2 s | > 2 s | 391 µs ✗ | 399 µs ✗ |
| SYJ207+1.020.p | 1.23 s ? memory | 123 µs ✓ | 17.6 ms ✓ | 14 µs unknown | 80 µs ✓ | > 2 s | > 2 s | 420 µs ✗ | 423 µs ✗ | > 2 s | > 2 s | 427 µs ✗ | 422 µs ✗ |

## SYJ208

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ208+1.001.p | 37 µs ✓ | 31 µs ✓ | 78 µs ✓ | 15 µs ✓ | 29 µs ✓ | 96 µs ✗ | 23 µs ✗ | 38 µs ✗ | 38 µs ✗ | 155 µs ✗ | 20 µs ✗ | 41 µs ✗ | 38 µs ✗ |
| SYJ208+1.002.p | 283 µs ✓ | 47 µs ✓ | 2.6 ms ✓ | 19 µs ✓ | 302 µs ✓ | 1.3 ms ✗ | 44 µs ✗ | 49 µs ✗ | 55 µs ✗ | 2.4 ms ✗ | 42 µs ✗ | 50 µs ✗ | 50 µs ✗ |
| SYJ208+1.003.p | 1.99 s ? memory | 76 µs ✓ | 17.0 ms ✓ | 13 µs unknown | 201.6 ms unknown | 344.0 ms ✗ | 128 µs ✗ | 67 µs ✗ | 66 µs ✗ | 493.4 ms ✗ | 114 µs ✗ | 67 µs ✗ | 64 µs ✗ |
| SYJ208+1.004.p | 1.59 s ? memory | 202 µs ✓ | 18.2 ms ✓ | 13 µs unknown | 115.0 ms ? memory | > 2 s | 667 µs ✗ | 86 µs ✗ | 86 µs ✗ | 2.00 s ✗ † | 683 µs ✗ | 91 µs ✗ | 92 µs ✗ |
| SYJ208+1.005.p | 1.39 s ? memory | 867 µs ✓ | 26.1 ms ✓ | 15 µs unknown | 67.0 ms ? memory | > 2 s | 8.3 ms ✗ | 135 µs ✗ | 129 µs ✗ | 2.01 s ✗ | 8.0 ms ✗ | 128 µs ✗ | 127 µs ✗ |
| SYJ208+1.006.p | 1.08 s ? memory | 2.3 ms ✓ | 111.3 ms ✓ | 13 µs unknown | 31.9 ms ? memory | > 2 s | 424.9 ms ✗ | 180 µs ✗ | 179 µs ✗ | > 2 s | 372.8 ms ✗ | 192 µs ✗ | 179 µs ✗ |
| SYJ208+1.007.p | 1.71 s ? memory | 7.8 ms ✓ | 129.8 ms ✓ | 14 µs unknown | 21.2 ms ? memory | > 2 s | > 2 s | 265 µs ✗ | 253 µs ✗ | > 2 s | > 2 s | 247 µs ✗ | 249 µs ✗ |
| SYJ208+1.008.p | 1.18 s ? memory | 38.2 ms ✓ | 157.9 ms ✓ | 17 µs unknown | 14.8 ms ? memory | > 2 s | > 2 s | 359 µs ✗ | 370 µs ✗ | > 2 s | > 2 s | 337 µs ✗ | 335 µs ✗ |
| SYJ208+1.009.p | 1.72 s ? memory | 189.2 ms ✓ | 306.5 ms ✓ | 14 µs unknown | 10.6 ms ? memory | > 2 s | > 2 s | 502 µs ✗ | 474 µs ✗ | > 2 s | > 2 s | 476 µs ✗ | 464 µs ✗ |
| SYJ208+1.010.p | 1.12 s ? memory | 1.37 s ✓ | 1.49 s ✓ | 12 µs unknown | 8.7 ms ? memory | 82.8 ms ? depth | 2.45 s ? memory | 639 µs ✗ | 629 µs ✗ | 85.0 ms ? depth † | 2.45 s ? memory | 636 µs ✗ | 638 µs ✗ |
| SYJ208+1.011.p | 1.43 s ? memory | > 2 s | > 2 s | 13 µs unknown | 7.6 ms ? memory | 83.0 ms ? depth | 2.31 s ? memory | 858 µs ✗ | 828 µs ✗ | 83.5 ms ? depth † | 2.31 s ? memory | 836 µs ✗ | 834 µs ✗ |
| SYJ208+1.012.p | > 2 s | > 2 s | > 2 s | 15 µs unknown | 7.9 ms ? memory | 82.1 ms ? depth | > 2 s | 1.1 ms ✗ | 1.1 ms ✗ | 83.3 ms ? depth † | > 2 s | 1.2 ms ✗ | 1.1 ms ✗ |
| SYJ208+1.013.p | 1.26 s ? memory | > 2 s | > 2 s | 14 µs unknown | 9.1 ms ? memory | 81.3 ms ? depth | 2.32 s ? memory | 1.4 ms ✗ | 1.5 ms ✗ | 79.4 ms ? depth | 2.30 s ? memory | 1.5 ms ✗ | 1.4 ms ✗ |
| SYJ208+1.014.p | 1.51 s ? memory | > 2 s | > 2 s | 17 µs unknown | 11.7 ms ? memory | 80.3 ms ? depth | 2.10 s ? memory | 1.8 ms ✗ | 1.8 ms ✗ | 80.2 ms ? depth | 2.04 s ? memory | 1.8 ms ✗ | 1.8 ms ✗ |
| SYJ208+1.015.p | > 2 s | > 2 s | > 2 s | 15 µs unknown | 15.8 ms ? memory | 76.6 ms ? depth | 2.15 s ? memory | 2.2 ms ✗ | 2.2 ms ✗ | 81.4 ms ? depth | 2.15 s ? memory | 2.2 ms ✗ | 2.2 ms ✗ |
| SYJ208+1.016.p | 95.9 ms ? depth | > 2 s | > 2 s | 15 µs unknown | 21.8 ms ? memory | 79.5 ms ? depth | 2.16 s ? memory | 2.7 ms ✗ | 2.8 ms ✗ | 82.0 ms ? depth | 2.12 s ? memory | 2.8 ms ✗ | 2.8 ms ✗ |
| SYJ208+1.017.p | 93.7 ms ? depth | > 2 s | > 2 s | 14 µs unknown | 30.3 ms ? memory | 75.6 ms ? depth | 2.07 s ? memory | 3.3 ms ✗ | 3.4 ms ✗ | 79.2 ms ? depth | 2.06 s ? memory | 3.5 ms ✗ | 3.5 ms ✗ |
| SYJ208+1.018.p | 93.1 ms ? depth † | > 2 s | > 2 s | 14 µs unknown | 42.6 ms ? memory | 75.8 ms ? depth | 2.10 s ? memory | 4.1 ms ✗ | 4.2 ms ✗ | 80.2 ms ? depth | 2.08 s ? memory | 4.2 ms ✗ | 4.2 ms ✗ |
| SYJ208+1.019.p | 95.1 ms ? depth | > 2 s | > 2 s | 18 µs unknown | 59.3 ms ? memory | 78.0 ms ? depth | 2.27 s ? memory | 5.0 ms ✗ | 5.0 ms ✗ | 78.3 ms ? depth † | 2.26 s ? memory | 5.0 ms ✗ | 5.0 ms ✗ |
| SYJ208+1.020.p | 96.7 ms ? depth | > 2 s | > 2 s | 16 µs unknown | 81.6 ms ? memory | 77.8 ms ? depth | 2.15 s ? memory | 6.0 ms ✗ | 6.2 ms ✗ | 80.6 ms ? depth | 2.17 s ? memory | 5.8 ms ✗ | 6.1 ms ✗ |

## SYJ209

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ209+1.001.p | 44 µs ✓ | 31 µs ✓ | 103 µs ✓ | 15 µs ✓ | 42 µs ✓ | 73 µs ✗ | 24 µs ✗ | 41 µs ✗ | 47 µs ✗ | 145 µs ✗ | 20 µs ✗ | 47 µs ✗ | 44 µs ✗ |
| SYJ209+1.002.p | 49 µs ✓ | 33 µs ✓ | 108 µs ✓ | 15 µs ✓ | 53 µs ✓ | 147 µs ✗ | 29 µs ✗ | 45 µs ✗ | 46 µs ✗ | 220 µs ✗ | 26 µs ✗ | 46 µs ✗ | 47 µs ✗ |
| SYJ209+1.003.p | 43 µs ✓ | 37 µs ✓ | 116 µs ✓ | 20 µs ✓ | 56 µs ✓ | 405 µs ✗ | 35 µs ✗ | 55 µs ✗ | 54 µs ✗ | 457 µs ✗ | 32 µs ✗ | 54 µs ✗ | 53 µs ✗ |
| SYJ209+1.004.p | 44 µs ✓ | 40 µs ✓ | 137 µs ✓ | 17 µs ✓ | 72 µs ✓ | 1.4 ms ✗ | 41 µs ✗ | 61 µs ✗ | 60 µs ✗ | 2.5 ms ✗ | 41 µs ✗ | 59 µs ✗ | 60 µs ✗ |
| SYJ209+1.005.p | 48 µs ✓ | 47 µs ✓ | 163 µs ✓ | 23 µs ✓ | 88 µs ✓ | 5.5 ms ✗ | 51 µs ✗ | 69 µs ✗ | 66 µs ✗ | 9.1 ms ✗ † | 47 µs ✗ | 64 µs ✗ | 69 µs ✗ |
| SYJ209+1.006.p | 48 µs ✓ | 47 µs ✓ | 185 µs ✓ | 29 µs ✓ | 100 µs ✓ | 22.9 ms ✗ | 62 µs ✗ | 73 µs ✗ | 73 µs ✗ | 35.9 ms ✗ † | 53 µs ✗ | 70 µs ✗ | 72 µs ✗ |
| SYJ209+1.007.p | 47 µs ✓ | 47 µs ✓ | 216 µs ✓ | 46 µs ✓ | 129 µs ✓ | 95.3 ms ✗ | 72 µs ✗ | 76 µs ✗ | 79 µs ✗ | 118.0 ms ✗ † | 60 µs ✗ | 81 µs ✗ | 73 µs ✗ |
| SYJ209+1.008.p | 50 µs ✓ | 51 µs ✓ | 264 µs ✓ | 79 µs ✓ | 147 µs ✓ | 111.6 ms ✗ | 83 µs ✗ | 86 µs ✗ | 82 µs ✗ | 118.7 ms ✗ † | 75 µs ✗ | 87 µs ✗ | 82 µs ✗ |
| SYJ209+1.009.p | 52 µs ✓ | 49 µs ✓ | 294 µs ✓ | 153 µs ✓ | 179 µs ✓ | 117.0 ms ✗ | 93 µs ✗ | 86 µs ✗ | 87 µs ✗ | 122.9 ms ✗ † | 85 µs ✗ | 85 µs ✗ | 92 µs ✗ |
| SYJ209+1.010.p | 64 µs ✓ | 52 µs ✓ | 329 µs ✓ | 10 µs unknown | 196 µs ✓ | 119.0 ms ✗ | 109 µs ✗ | 96 µs ✗ | 91 µs ✗ | 126.5 ms ✗ † | 97 µs ✗ | 97 µs ✗ | 93 µs ✗ |
| SYJ209+1.011.p | 63 µs ✓ | 53 µs ✓ | 375 µs ✓ | 10 µs unknown | 235 µs ✓ | 122.6 ms ✗ | 119 µs ✗ | 99 µs ✗ | 102 µs ✗ | 133.5 ms ✗ † | 112 µs ✗ | 98 µs ✗ | 101 µs ✗ |
| SYJ209+1.012.p | 66 µs ✓ | 59 µs ✓ | 429 µs ✓ | 11 µs unknown | 278 µs ✓ | 130.3 ms ✗ | 140 µs ✗ | 115 µs ✗ | 108 µs ✗ | 138.8 ms ✗ † | 123 µs ✗ | 109 µs ✗ | 115 µs ✗ |
| SYJ209+1.013.p | 64 µs ✓ | 60 µs ✓ | 485 µs ✓ | 11 µs unknown | 310 µs ✓ | 137.8 ms ✗ | 159 µs ✗ | 119 µs ✗ | 114 µs ✗ | 147.8 ms ✗ † | 143 µs ✗ | 111 µs ✗ | 117 µs ✗ |
| SYJ209+1.014.p | 69 µs ✓ | 63 µs ✓ | 564 µs ✓ | 12 µs unknown | 369 µs ✓ | 143.7 ms ✗ | 165 µs ✗ | 124 µs ✗ | 123 µs ✗ | 158.1 ms ✗ † | 149 µs ✗ | 119 µs ✗ | 122 µs ✗ |
| SYJ209+1.015.p | 70 µs ✓ | 62 µs ✓ | 593 µs ✓ | 10 µs unknown | 390 µs ✓ | 161.8 ms ✗ | 199 µs ✗ | 131 µs ✗ | 134 µs ✗ | 188.6 ms ✗ † | 180 µs ✗ | 139 µs ✗ | 129 µs ✗ |
| SYJ209+1.016.p | 74 µs ✓ | 66 µs ✓ | 711 µs ✓ | 12 µs unknown | 486 µs ✓ | 197.8 ms ✗ | 237 µs ✗ | 143 µs ✗ | 143 µs ✗ | 245.3 ms ✗ † | 226 µs ✗ | 140 µs ✗ | 147 µs ✗ |
| SYJ209+1.017.p | 89 µs ✓ | 64 µs ✓ | 778 µs ✓ | 9 µs unknown | 507 µs ✓ | 249.5 ms ✗ | 247 µs ✗ | 147 µs ✗ | 148 µs ✗ | 262.8 ms ✗ † | 227 µs ✗ | 149 µs ✗ | 148 µs ✗ |
| SYJ209+1.018.p | 86 µs ✓ | 69 µs ✓ | 853 µs ✓ | 11 µs unknown | 572 µs ✓ | 253.9 ms ✗ | 279 µs ✗ | 162 µs ✗ | 157 µs ✗ | 274.5 ms ✗ † | 262 µs ✗ | 161 µs ✗ | 158 µs ✗ |
| SYJ209+1.019.p | 93 µs ✓ | 68 µs ✓ | 932 µs ✓ | 12 µs unknown | 660 µs ✓ | 259.0 ms ✗ | 309 µs ✗ | 174 µs ✗ | 181 µs ✗ | 269.2 ms ✗ † | 282 µs ✗ | 170 µs ✗ | 166 µs ✗ |
| SYJ209+1.020.p | 93 µs ✓ | 74 µs ✓ | 979 µs ✓ | 11 µs unknown | 717 µs ✓ | 256.7 ms ✗ | 343 µs ✗ | 175 µs ✗ | 177 µs ✗ | 270.3 ms ✗ † | 338 µs ✗ | 178 µs ✗ | 180 µs ✗ |

## SYJ210

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ210+1.001.p | 36 µs ✓ | 31 µs ✓ | 76 µs ✓ | 13 µs ✓ | 35 µs ✓ | 108 µs ✗ | 18 µs ✗ | 36 µs ✗ | 35 µs ✗ | 110 µs ✗ | 14 µs ✗ | 35 µs ✗ | 38 µs ✗ |
| SYJ210+1.002.p | 37 µs ✓ | 27 µs ✓ | 111 µs ✓ | 12 µs ✓ | 43 µs ✓ | 120 µs ✗ | 21 µs ✗ | 35 µs ✗ | 37 µs ✗ | 119 µs ✗ | 15 µs ✗ | 37 µs ✗ | 38 µs ✗ |
| SYJ210+1.003.p | 47 µs ✓ | 29 µs ✓ | 161 µs ✓ | 14 µs ✓ | 69 µs ✓ | 139 µs ✗ | 19 µs ✗ | 38 µs ✗ | 39 µs ✗ | 138 µs ✗ | 14 µs ✗ | 42 µs ✗ | 42 µs ✗ |
| SYJ210+1.004.p | 63 µs ✓ | 36 µs ✓ | 335 µs ✓ | 13 µs ✓ | 119 µs ✓ | 147 µs ✗ | 23 µs ✗ | 44 µs ✗ | 41 µs ✗ | 168 µs ✗ | 16 µs ✗ | 44 µs ✗ | 42 µs ✗ |
| SYJ210+1.005.p | 126 µs ✓ | 40 µs ✓ | 889 µs ✓ | 16 µs ✓ | 253 µs ✓ | 168 µs ✗ | 21 µs ✗ | 44 µs ✗ | 46 µs ✗ | 177 µs ✗ | 15 µs ✗ | 42 µs ✗ | 43 µs ✗ |
| SYJ210+1.006.p | 281 µs ✓ | 38 µs ✓ | 2.9 ms ✓ | 25 µs ✓ | 491 µs ✓ | 200 µs ✗ | 21 µs ✗ | 44 µs ✗ | 44 µs ✗ | 195 µs ✗ | 15 µs ✗ | 42 µs ✗ | 42 µs ✗ |
| SYJ210+1.007.p | 783 µs ✓ | 43 µs ✓ | 9.8 ms ✓ | 33 µs ✓ | 1.1 ms ✓ | 215 µs ✗ | 25 µs ✗ | 46 µs ✗ | 44 µs ✗ | 216 µs ✗ | 15 µs ✗ | 48 µs ✗ | 48 µs ✗ |
| SYJ210+1.008.p | 2.4 ms ✓ | 45 µs ✓ | 34.4 ms ✓ | 60 µs ✓ | 2.4 ms ✓ | 247 µs ✗ | 25 µs ✗ | 45 µs ✗ | 44 µs ✗ | 241 µs ✗ | 16 µs ✗ | 46 µs ✗ | 45 µs ✗ |
| SYJ210+1.009.p | 8.1 ms ✓ | 42 µs ✓ | 15.0 ms ✓ | 102 µs ✓ | 5.6 ms ✓ | 281 µs ✗ | 26 µs ✗ | 43 µs ✗ | 43 µs ✗ | 286 µs ✗ | 16 µs ✗ | 45 µs ✗ | 44 µs ✗ |
| SYJ210+1.010.p | 26.2 ms ✓ | 46 µs ✓ | 15.3 ms ✓ | 13 µs unknown | 12.3 ms ✓ | 351 µs ✗ | 29 µs ✗ | 45 µs ✗ | 48 µs ✗ | 372 µs ✗ | 18 µs ✗ | 46 µs ✗ | 49 µs ✗ |
| SYJ210+1.011.p | 86.0 ms ✓ | 48 µs ✓ | 15.0 ms ✓ | 13 µs unknown | 27.5 ms ✓ | 383 µs ✗ | 30 µs ✗ | 48 µs ✗ | 47 µs ✗ | 410 µs ✗ | 17 µs ✗ | 52 µs ✗ | 48 µs ✗ |
| SYJ210+1.012.p | 304.8 ms ✓ | 49 µs ✓ | 15.9 ms ✓ | 13 µs unknown | 63.0 ms ✓ | 1.2 ms ✗ | 29 µs ✗ | 47 µs ✗ | 50 µs ✗ | 779 µs ✗ | 15 µs ✗ | 47 µs ✗ | 51 µs ✗ |
| SYJ210+1.013.p | 1.00 s ✓ | 53 µs ✓ | 15.4 ms ✓ | 13 µs unknown | 63.6 ms unknown | 1.2 ms ✗ | 32 µs ✗ | 53 µs ✗ | 50 µs ✗ | 838 µs ✗ | 17 µs ✗ | 49 µs ✗ | 54 µs ✗ |
| SYJ210+1.014.p | 1.87 s ? memory | 53 µs ✓ | 16.2 ms ✓ | 12 µs unknown | 138.4 ms unknown | 1.2 ms ✗ | 33 µs ✗ | 49 µs ✗ | 52 µs ✗ | 933 µs ✗ | 19 µs ✗ | 51 µs ✗ | 51 µs ✗ |
| SYJ210+1.015.p | 1.50 s ? memory | 52 µs ✓ | 16.0 ms ✓ | 15 µs unknown | 290.8 ms unknown | 1.9 ms ✗ | 37 µs ✗ | 50 µs ✗ | 49 µs ✗ | 1.3 ms ✗ | 19 µs ✗ | 50 µs ✗ | 53 µs ✗ |
| SYJ210+1.016.p | 1.60 s ? memory | 58 µs ✓ | 15.0 ms ✓ | 12 µs unknown | 622.8 ms unknown | 1.9 ms ✗ | 37 µs ✗ | 51 µs ✗ | 51 µs ✗ | 1.3 ms ✗ | 20 µs ✗ | 53 µs ✗ | 53 µs ✗ |
| SYJ210+1.017.p | 1.66 s ? memory | 54 µs ✓ | 15.5 ms ✓ | 13 µs unknown | 173.3 ms ? memory | 2.0 ms ✗ | 39 µs ✗ | 56 µs ✗ | 53 µs ✗ | 1.4 ms ✗ | 19 µs ✗ | 51 µs ✗ | 51 µs ✗ |
| SYJ210+1.018.p | 1.70 s ? memory | 56 µs ✓ | 15.3 ms ✓ | 14 µs unknown | 167.0 ms ? memory | 2.7 ms ✗ | 41 µs ✗ | 50 µs ✗ | 53 µs ✗ | 1.6 ms ✗ | 19 µs ✗ | 52 µs ✗ | 54 µs ✗ |
| SYJ210+1.019.p | 1.79 s ? memory | 56 µs ✓ | 15.6 ms ✓ | 14 µs unknown | 159.2 ms ? memory | 3.5 ms ✗ | 44 µs ✗ | 53 µs ✗ | 53 µs ✗ | 2.0 ms ✗ | 17 µs ✗ | 53 µs ✗ | 52 µs ✗ |
| SYJ210+1.020.p | 1.83 s ? memory | 60 µs ✓ | 15.8 ms ✓ | 15 µs unknown | 151.5 ms ? memory | 3.6 ms ✗ | 44 µs ✗ | 53 µs ✗ | 53 µs ✗ | 2.1 ms ✗ | 19 µs ✗ | 56 µs ✗ | 56 µs ✗ |

## SYJ211

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ211+1.001.p | 42 µs ✓ | 36 µs ✓ | 121 µs ✓ | 16 µs ✓ | 52 µs ✓ | 516 µs ✗ | 36 µs ✗ | 51 µs ✗ | 51 µs ✗ | 356 µs ✗ | 34 µs ✗ | 48 µs ✗ | 47 µs ✗ |
| SYJ211+1.002.p | 47 µs ✓ | 42 µs ✓ | 163 µs ✓ | 21 µs ✓ | 66 µs ✓ | 39.8 ms ✗ | 78 µs ✗ | 59 µs ✗ | 54 µs ✗ | 10.4 ms ✗ | 60 µs ✗ | 56 µs ✗ | 57 µs ✗ |
| SYJ211+1.003.p | 62 µs ✓ | 41 µs ✓ | 252 µs ✓ | 46 µs ✓ | 84 µs ✓ | 107.7 ms ✗ | 123 µs ✗ | 61 µs ✗ | 62 µs ✗ | 106.9 ms ✗ | 110 µs ✗ | 62 µs ✗ | 62 µs ✗ |
| SYJ211+1.004.p | 76 µs ✓ | 45 µs ✓ | 452 µs ✓ | 10 µs unknown | 100 µs ✓ | 107.3 ms ✗ | 223 µs ✗ | 71 µs ✗ | 73 µs ✗ | 111.2 ms ✗ † | 198 µs ✗ | 73 µs ✗ | 75 µs ✗ |
| SYJ211+1.005.p | 104 µs ✓ | 42 µs ✓ | 820 µs ✓ | 10 µs unknown | 115 µs ✓ | 104.3 ms ✗ † | 386 µs ✗ | 78 µs ✗ | 74 µs ✗ | 108.1 ms ✗ † | 348 µs ✗ | 80 µs ✗ | 80 µs ✗ |
| SYJ211+1.006.p | 177 µs ✓ | 46 µs ✓ | 1.8 ms ✓ | 15 µs unknown | 141 µs ✓ | 102.6 ms ✗ † | 644 µs ✗ | 81 µs ✗ | 81 µs ✗ | 105.6 ms ✗ | 598 µs ✗ | 86 µs ✗ | 80 µs ✗ |
| SYJ211+1.007.p | 309 µs ✓ | 50 µs ✓ | 3.6 ms ✓ | 12 µs unknown | 159 µs ✓ | 101.5 ms ✗ | 1.0 ms ✗ | 96 µs ✗ | 96 µs ✗ | 105.9 ms ✗ | 997 µs ✗ | 102 µs ✗ | 97 µs ✗ |
| SYJ211+1.008.p | 585 µs ✓ | 56 µs ✓ | 8.6 ms ✓ | 12 µs unknown | 184 µs ✓ | 95.4 ms ✗ | 1.8 ms ✗ | 100 µs ✗ | 99 µs ✗ | 103.3 ms ✗ † | 1.7 ms ✗ | 111 µs ✗ | 105 µs ✗ |
| SYJ211+1.009.p | 1.1 ms ✓ | 61 µs ✓ | 19.7 ms ✓ | 13 µs unknown | 223 µs ✓ | 94.1 ms ✗ | 2.6 ms ✗ | 108 µs ✗ | 111 µs ✗ | 101.5 ms ✗ † | 2.5 ms ✗ | 115 µs ✗ | 110 µs ✗ |
| SYJ211+1.010.p | 2.5 ms ✓ | 61 µs ✓ | 46.1 ms ✓ | 16 µs unknown | 250 µs ✓ | 93.8 ms ✗ † | 3.8 ms ✗ | 128 µs ✗ | 129 µs ✗ | 99.1 ms ✗ † | 3.8 ms ✗ | 126 µs ✗ | 127 µs ✗ |
| SYJ211+1.011.p | 5.1 ms ✓ | 64 µs ✓ | 8.3 ms ✓ | 13 µs unknown | 285 µs ✓ | 92.4 ms ✗ † | 5.4 ms ✗ | 132 µs ✗ | 133 µs ✗ | 99.1 ms ✗ | 5.6 ms ✗ | 134 µs ✗ | 132 µs ✗ |
| SYJ211+1.012.p | 10.9 ms ✓ | 62 µs ✓ | 15.5 ms ✓ † | 13 µs unknown | 338 µs ✓ | 91.5 ms ✗ † | 7.3 ms ✗ | 153 µs ✗ | 148 µs ✗ | 95.6 ms ✗ † | 7.9 ms ✗ | 154 µs ✗ | 147 µs ✗ |
| SYJ211+1.013.p | 23.4 ms ✓ † | 71 µs ✓ | 16.2 ms ✓ | 14 µs unknown | 342 µs ✓ | 94.4 ms ✗ † | 10.4 ms ✗ | 163 µs ✗ | 152 µs ✗ | 97.1 ms ✗ † | 11.3 ms ✗ | 160 µs ✗ | 153 µs ✗ |
| SYJ211+1.014.p | 51.1 ms ✓ | 66 µs ✓ | 16.9 ms ✓ | 15 µs unknown | 387 µs ✓ | 91.6 ms ✗ † | 13.9 ms ✗ | 169 µs ✗ | 170 µs ✗ | 94.6 ms ✗ | 15.9 ms ✗ | 173 µs ✗ | 169 µs ✗ |
| SYJ211+1.015.p | 107.4 ms ✓ | 71 µs ✓ | 15.2 ms ✓ | 12 µs unknown | 429 µs ✓ | 91.1 ms ✗ † | 18.3 ms ✗ | 186 µs ✗ | 176 µs ✗ | 94.9 ms ✗ † | 21.4 ms ✗ | 192 µs ✗ | 185 µs ✗ |
| SYJ211+1.016.p | 232.1 ms ✓ | 74 µs ✓ | 15.9 ms ✓ | 13 µs unknown | 480 µs ✓ | 88.8 ms ✗ † | 24.7 ms ✗ | 200 µs ✗ | 194 µs ✗ | 92.1 ms ✗ † | 29.2 ms ✗ | 204 µs ✗ | 200 µs ✗ |
| SYJ211+1.017.p | 497.6 ms ✓ | 70 µs ✓ | 15.2 ms ✓ | 12 µs unknown | 534 µs ✓ | 87.4 ms ✗ † | 33.8 ms ✗ | 228 µs ✗ | 212 µs ✗ | 89.4 ms ✗ | 38.7 ms ✗ | 224 µs ✗ | 213 µs ✗ |
| SYJ211+1.018.p | 1.05 s ✓ | 79 µs ✓ | 15.7 ms ✓ | 13 µs unknown | 587 µs ✓ | 87.7 ms ✗ † | 43.2 ms ✗ | 231 µs ✗ | 222 µs ✗ | 86.8 ms ✗ † | 47.5 ms ✗ | 230 µs ✗ | 225 µs ✗ |
| SYJ211+1.019.p | 1.52 s ? memory | 77 µs ✓ | 16.5 ms ✓ | 16 µs unknown | 653 µs ✓ | 82.4 ms ✗ | 55.8 ms ✗ | 246 µs ✗ | 236 µs ✗ | 85.7 ms ✗ | 61.5 ms ✗ | 247 µs ✗ | 236 µs ✗ |
| SYJ211+1.020.p | 1.58 s ? memory | 83 µs ✓ | 15.7 ms ✓ | 13 µs unknown | 690 µs ✓ | 82.6 ms ✗ † | 71.7 ms ✗ | 268 µs ✗ | 263 µs ✗ | 85.3 ms ✗ † | 77.0 ms ✗ | 270 µs ✗ | 260 µs ✗ |

## SYJ212

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYJ212+1.001.p | 46 µs ✓ | 31 µs ✓ | 79 µs ✓ | 12 µs ✓ | 34 µs ✓ | 53 µs ✗ | 18 µs ✗ | 43 µs ✗ | 46 µs ✗ | 107 µs ✗ | 17 µs ✗ | 45 µs ✗ | 42 µs ✗ |
| SYJ212+1.002.p | 48 µs ✓ | 36 µs ✓ | 145 µs ✓ | 12 µs ✓ | 93 µs ✓ | 137 µs ✗ | 22 µs ✗ | 62 µs ✗ | 54 µs ✗ | 143 µs ✗ | 20 µs ✗ | 51 µs ✗ | 52 µs ✗ |
| SYJ212+1.003.p | 122 µs ✓ | 49 µs ✓ | 803 µs ✓ | 13 µs ✓ | 280 µs ✓ | 176 µs ✗ | 25 µs ✗ | 87 µs ✗ | 84 µs ✗ | 177 µs ✗ | 23 µs ✗ | 68 µs ✗ | 69 µs ✗ |
| SYJ212+1.004.p | 2.5 ms ✓ | 68 µs ✓ | 29.4 ms ✓ † | 17 µs ✓ | 1.0 ms ✓ | 249 µs ✗ | 40 µs ✗ | 116 µs ✗ | 120 µs ✗ | 242 µs ✗ | 30 µs ✗ | 79 µs ✗ | 83 µs ✗ |
| SYJ212+1.005.p | 522.7 ms ✓ | 94 µs ✓ | 16.8 ms ✓ | 17 µs ✓ | 4.5 ms ✓ | 402 µs ✗ | 47 µs ✗ | 277 µs ✗ | 290 µs ✗ | 390 µs ✗ | 35 µs ✗ | 87 µs ✗ | 94 µs ✗ |
| SYJ212+1.006.p | 1.94 s ? memory | 107 µs ✓ | 15.6 ms ✓ | 19 µs ✓ | 23.9 ms ✓ | 667 µs ✗ | 54 µs ✗ | 673 µs ✗ | 682 µs ✗ | 621 µs ✗ | 42 µs ✗ | 96 µs ✗ | 102 µs ✗ |
| SYJ212+1.007.p | 1.86 s ? memory | 128 µs ✓ | 13.7 ms ✓ † | 21 µs ✓ | 69.2 ms unknown | 2.2 ms ✗ | 67 µs ✗ | 319 µs ✗ | 323 µs ✗ | 2.0 ms ✗ | 46 µs ✗ | 111 µs ✗ | 108 µs ✗ |
| SYJ212+1.008.p | 1.40 s ? memory | 175 µs ✓ | 10.6 ms ✓ | 26 µs ✓ | 430.9 ms unknown | 4.4 ms ✗ | 77 µs ✗ | 257 µs ✗ | 257 µs ✗ | 3.9 ms ✗ | 56 µs ✗ | 128 µs ✗ | 124 µs ✗ |
| SYJ212+1.009.p | 1.56 s ? memory | 200 µs ✓ | 8.0 ms ✓ | 50 µs ✓ | 367.5 ms ? memory | 9.0 ms ✗ † | 87 µs ✗ | 276 µs ✗ | 280 µs ✗ | 8.2 ms ✗ | 67 µs ✗ | 142 µs ✗ | 133 µs ✗ |
| SYJ212+1.010.p | 1.68 s ? memory | 246 µs ✓ | 5.3 ms ✓ | 74 µs ✓ | 323.6 ms ? memory | 22.6 ms ✗ | 95 µs ✗ | 282 µs ✗ | 281 µs ✗ | 20.0 ms ✗ † | 69 µs ✗ | 145 µs ✗ | 148 µs ✗ |
| SYJ212+1.011.p | 1.87 s ? memory | 251 µs ✓ | 4.3 ms ✓ | 12 µs unknown | 307.2 ms ? memory | 67.7 ms ✗ † | 106 µs ✗ | 310 µs ✗ | 309 µs ✗ | 61.5 ms ✗ | 77 µs ✗ | 154 µs ✗ | 160 µs ✗ |
| SYJ212+1.012.p | 2.06 s ? memory | 271 µs ✓ | 5.5 ms ✓ | 14 µs unknown | 287.9 ms ? memory | 251.6 ms ✗ † | 125 µs ✗ | 350 µs ✗ | 352 µs ✗ | 232.2 ms ✗ | 90 µs ✗ | 180 µs ✗ | 181 µs ✗ |
| SYJ212+1.013.p | > 2 s | 340 µs ✓ | 9.4 ms ✓ | 11 µs unknown | 279.4 ms ? memory | 635.8 ms ✗ | 143 µs ✗ | 341 µs ✗ | 338 µs ✗ | 591.5 ms ✗ | 87 µs ✗ | 181 µs ✗ | 184 µs ✗ |
| SYJ212+1.014.p | > 2 s | 367 µs ✓ | 22.4 ms ✓ | 12 µs unknown | 262.7 ms ? memory | 1.53 s ✗ | 150 µs ✗ | 411 µs ✗ | 411 µs ✗ | 1.39 s ✗ | 95 µs ✗ | 187 µs ✗ | 201 µs ✗ |
| SYJ212+1.015.p | > 2 s | 531 µs ✓ | 43.2 ms ✓ | 15 µs unknown | 255.5 ms ? memory | > 2 s | 161 µs ✗ | 418 µs ✗ | 428 µs ✗ | 2.05 s ✗ | 103 µs ✗ | 223 µs ✗ | 217 µs ✗ |
| SYJ212+1.016.p | > 2 s | 465 µs ✓ | 77.7 ms ✓ | 13 µs unknown | 234.9 ms ? memory | > 2 s | 177 µs ✗ | 107.9 ms ✗ | 108.4 ms ✗ | 2.08 s ✗ | 112 µs ✗ | 235 µs ✗ | 229 µs ✗ |
| SYJ212+1.017.p | > 2 s | 439 µs ✓ | 484 µs ✓ | 12 µs unknown | 234.1 ms ? memory | > 2 s | 190 µs ✗ | 413 µs ✗ | 409 µs ✗ | > 2 s | 125 µs ✗ | 248 µs ✗ | 243 µs ✗ |
| SYJ212+1.018.p | > 2 s | 447 µs ✓ | 492 µs ✓ | 12 µs unknown | 212.2 ms ? memory | > 2 s | 201 µs ✗ | 461 µs ✗ | 456 µs ✗ | > 2 s | 125 µs ✗ | 263 µs ✗ | 261 µs ✗ |
| SYJ212+1.019.p | > 2 s | 625 µs ✓ | 687 µs ✓ | 12 µs unknown | 216.3 ms ? memory | > 2 s | 232 µs ✗ | 469 µs ✗ | 545 µs ✗ | > 2 s | 138 µs ✗ | 278 µs ✗ | 281 µs ✗ |
| SYJ212+1.020.p | > 2 s | 586 µs ✓ | 661 µs ✓ | 12 µs unknown | 198.1 ms ? memory | 654.6 ms ? memory | 253 µs ✗ | 490 µs ✗ | 501 µs ✗ | 661.0 ms ? memory | 150 µs ✗ | 305 µs ✗ | 317 µs ✗ |

## Problems/SYN

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYN001+1.p | 43 µs ✓ | 30 µs ✓ | 82 µs ✓ | 15 µs ✓ | 36 µs ✓ | 50 µs ✗ | 17 µs ✗ | 44 µs ✗ | 41 µs ✗ | 112 µs ✗ | 15 µs ✗ | 40 µs ✗ | 41 µs ✗ |
| SYN040+1.p | 52 µs ✓ | 43 µs ✓ | 114 µs ✓ | 18 µs ✓ | 52 µs ✓ | 113 µs ✗ | 20 µs ✗ | 46 µs ✗ | 46 µs ✗ | 118 µs ✗ | 19 µs ✗ | 44 µs ✗ | 47 µs ✗ |
| SYN041+1.p | 43 µs ✓ | 37 µs ✓ | 95 µs ✓ | 15 µs ✓ | 39 µs ✓ | 41 µs ✓ | 28 µs ✓ | 40 µs ✓ | 58 µs ✓ | 102 µs ✗ | 15 µs ✗ | 39 µs ✗ | 40 µs ✗ |
| SYN044+1.p | 59 µs ✓ | 39 µs ✓ | 197 µs ✓ | 19 µs ✓ | 84 µs ✓ | 122 µs ✓ | 33 µs ✓ | 48 µs ✓ | 73 µs ✓ | 124 µs ✓ | 33 µs ✓ | 47 µs ✓ | 71 µs ✓ |
| SYN045+1.p | 52 µs ✓ | 45 µs ✓ | 149 µs ✓ | 14 µs ✓ | 61 µs ✓ | 89 µs ✓ | 38 µs ✓ | 53 µs ✓ | 87 µs ✓ | 88 µs ✓ | 41 µs ✓ | 59 µs ✓ | 92 µs ✓ |
| SYN046+1.p | 46 µs ✓ | 41 µs ✓ | 110 µs ✓ | 16 µs ✓ | 44 µs ✓ | 116 µs ✗ | 18 µs ✗ | 43 µs ✗ | 44 µs ✗ | 106 µs ✗ | 18 µs ✗ | 46 µs ✗ | 45 µs ✗ |
| SYN047+1.p | 55 µs ✓ | 44 µs ✓ | 206 µs ✓ | 14 µs ✓ | 83 µs ✓ | 124 µs ✗ | 19 µs ✗ | 68 µs ✗ | 72 µs ✗ | 129 µs ✗ | 18 µs ✗ | 58 µs ✗ | 60 µs ✗ |
| SYN387+1.p | 36 µs ✓ | 26 µs ✓ | 70 µs ✓ | 12 µs ✓ | 23 µs ✓ | 40 µs ✗ | 13 µs ✗ | 35 µs ✗ | 37 µs ✗ | 42 µs ✗ | 14 µs ✗ | 35 µs ✗ | 36 µs ✗ |
| SYN388+1.p | 33 µs ✓ | 26 µs ✓ | 69 µs ✓ | 12 µs ✓ | 26 µs ✓ | 53 µs ✗ | 19 µs ✗ | 39 µs ✗ | 35 µs ✗ | 47 µs ✗ | 19 µs ✗ | 38 µs ✗ | 38 µs ✗ |
| SYN389+1.p | 37 µs ✓ | 28 µs ✓ | 81 µs ✓ | 13 µs ✓ | 33 µs ✓ | 96 µs ✗ | 17 µs ✗ | 39 µs ✗ | 39 µs ✗ | 108 µs ✗ | 15 µs ✗ | 39 µs ✗ | 42 µs ✗ |
| SYN390+1.p | 39 µs ✓ | 25 µs ✓ | 77 µs ✓ | 13 µs ✓ | 31 µs ✓ | 35 µs ✓ | 19 µs ✓ | 34 µs ✓ | 40 µs ✓ | 37 µs ✓ | 19 µs ✓ | 33 µs ✓ | 42 µs ✓ |
| SYN391+1.p | 52 µs ✓ | 40 µs ✓ | 135 µs ✓ | 13 µs ✓ | 63 µs ✓ | 257 µs ✓ | 46 µs ✓ | 59 µs ✓ | 97 µs ✓ | 299 µs ✓ | 45 µs ✓ | 62 µs ✓ | 98 µs ✓ |
| SYN392+1.p | 50 µs ✓ | 40 µs ✓ | 145 µs ✓ | 13 µs ✓ | 74 µs ✓ | 125 µs ✗ | 18 µs ✗ | 71 µs ✗ | 71 µs ✗ | 121 µs ✗ | 19 µs ✗ | 52 µs ✗ | 53 µs ✗ |
| SYN393+1.p | 126 µs ✓ | 43 µs ✓ | 802 µs ✓ | 16 µs ✓ | 229 µs ✓ | 155 µs ✗ | 32 µs ✗ | 124 µs ✗ | 123 µs ✗ | 155 µs ✗ | 32 µs ✗ | 115 µs ✗ | 109 µs ✗ |
| SYN416+1.p | 42 µs ✓ | 29 µs ✓ | 86 µs ✓ | 15 µs ✓ | 30 µs ✓ | 42 µs ✗ | 15 µs ✗ | 38 µs ✗ | 39 µs ✗ | 42 µs ✗ | 20 µs ✗ | 40 µs ✗ | 39 µs ✗ |
| SYN915+1.p | 27 µs ✓ | 23 µs ✓ | 58 µs ✓ | 12 µs ✓ | 21 µs ✓ | 23 µs ✓ | 15 µs ✓ | 23 µs ✓ | 30 µs ✓ | 22 µs ✓ | 16 µs ✓ | 23 µs ✓ | 32 µs ✓ |
| SYN916+1.p | 34 µs ✗ | 32 µs ✗ | 58 µs ✗ | 22 µs ✗ | 26 µs ✗ | 32 µs ✗ | 12 µs ✗ | 28 µs ✗ | 30 µs ✗ | 33 µs ✗ | 13 µs ✗ | 28 µs ✗ | 29 µs ✗ |
| SYN977+1.p | 44 µs ✓ | 29 µs ✓ | 89 µs ✓ | 11 µs ✓ | 41 µs ✓ | 47 µs ✗ | 16 µs ✗ | 37 µs ✗ | 42 µs ✗ | 45 µs ✗ | 16 µs ✗ | 37 µs ✗ | 39 µs ✗ |
| SYN978+1.p | 38 µs ✓ | 27 µs ✓ | 85 µs ✓ | 12 µs ✓ | 36 µs ✓ | 41 µs ✓ | 25 µs ✓ | 42 µs ✓ | 53 µs ✓ | 45 µs ✓ | 26 µs ✓ | 38 µs ✓ | 54 µs ✓ |

## SYN007

| problem | iltp-classical: classical embedding j1 | iltp-classical: classical sat j1 | iltp-classical: classical auto j1 | iltp-classical: classical table j1 | iltp-classical: classical tableau j1 | iltp: intuitionistic embedding j1 | iltp: intuitionistic ljt j1 | iltp: intuitionistic sat j1 | iltp: intuitionistic auto j1 | iltp: minimal embedding j1 | iltp: minimal ljt j1 | iltp: minimal sat j1 | iltp: minimal auto j1 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| SYN007+1.014.p | 0 µs error | 620 µs ✓ | 659 µs ✓ | 11 µs unknown | 234.4 ms ? memory | 0 µs error | 192 µs ✗ | 355 µs ✗ | 388 µs ✗ | 0 µs error | 181 µs ✗ | 363 µs ✗ | 368 µs ✗ |
