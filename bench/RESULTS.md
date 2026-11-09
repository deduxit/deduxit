# Benchmark results

The baseline of 2026-11-02 on an Intel(R) Core(TM) Ultra X9 388H
(16 cores, 62 GB), release build, from
`bench/baseline.sh`, which writes this file, the CSV files beside it
(`bench/results/2026-11-02/`, one row per run) and a copy of this file as
`bench/RESULTS.md`, the latest baseline's. It started (once more for
every resumption), with the commit it measured, the LLTP problems of its
later stages chosen from `bench/results/2026-09-30/` and its reruns from
`bench/reruns.txt`:

- 2026-11-02 14:34, commit a917eaf4772d (Let the baseline share the machine: its cores, CPUs and the others' as options), load average 1.23, sequential streams on CPUs 4 5 2 3, the run confined to CPUs 2-9, the other slices to 0-1,10-15

Its last part took 2 h 23 min, and the scheduled jobs that ran
meanwhile were: none. The package throttled
24521 times for 302 s in that part
(platform profile performance, governor powersave, energy preference balance_performance, turbo on).
The journal of `deduxit-baseline` says every ten minutes what else used
a CPU. The sequential runs went in four
streams at once, each pinned by taskset to a CPU of its own (CPUs 4 5 2
3), which other processes could share. The record above says the run
was confined to CPUs 2-9 and the other slices to 0-1,10-15, but neither
took effect: the cpuset controller is not delegated to the user's
systemd on this machine, so a unit's or a slice's AllowedCPUs does
nothing (the script now confines a run by its affinity and says what it
had). The parallel runs came after the sequential ones with no stream of
the run beside them, on 16 threads over every core of the machine (the
rows' `jobs`), while the machine's other work went on: their times and
speedups are not comparable with the third baseline's, taken at night.
A time is the median of up to
three runs; `✓` is proved, `✗` refuted, `?` unknown (`bound`: the copy
bound, `wide`: a context too wide to split, `depth`: the recursion
limit), `>` a time limit reached, `MISMATCH` a verdict against the
problem's known one (for LLTP, the one its header claims). `deduxit-bench
summary bench/results/2026-11-02/*.csv` prints the tables again.

## Solved within the time limit

Sequential problems that waited for a CPU for over 1 % of their time (another process slowed them down; `†` in the tables below): 46.

| family | configuration | engines | problems | solved | proved | refuted | late | timeout | bound | other | mismatch | median solved | total solved |
|---|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| partition-yes | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 130 µs | 439.5 ms |
| partition-yes | engines: classical net j1 | net | 5 | 3 | 3 | 0 | 0 | 2 | 0 | 0 | 0 | 28.5 ms | 506.3 ms |
| partition-no | engines: classical focus j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 80.2 ms | 3.55 s |
| partition-no | engines: classical net j1 | net | 4 | 2 | 0 | 2 | 0 | 2 | 0 | 0 | 0 | 9.6 ms | 11.1 ms |
| 3-partition-mll-yes | engines: classical focus j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 52 µs | 206 µs |
| 3-partition-mll-yes | engines: classical net j1 | net | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 132 µs | 532 µs |
| 3-partition-mll-no | engines: classical focus j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 42 µs | 162 µs |
| 3-partition-mll-no | engines: classical net j1 | net | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 5.1 ms |
| wide-m1 | engines: classical focus j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 121 µs | 212.8 ms |
| wide-m1 | engines: classical net j1 | net | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 212 µs | 608.5 ms |
| wide-m2 | engines: classical focus j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 106 µs | 203.6 ms |
| wide-m2 | engines: classical net j1 | net | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 216 µs | 602.8 ms |
| wide-m3 | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 104 µs | 181.8 ms |
| wide-m3 | engines: classical net j1 | net | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 226 µs | 640.3 ms |
| wide-m4 | engines: classical focus j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 91 µs | 169.2 ms |
| wide-m4 | engines: classical net j1 | net | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 290 µs | 643.1 ms |
| additive | engines: classical focus j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 328.5 ms | 5.64 s |
| partition-table | engines: classical focus j1 | focus | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 74 µs | 2.1 ms |
| partition-table | engines: classical net j1 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.3 ms | 5.21 s |
| cancellation | engines: classical focus j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 278 µs | 278 µs |
| 3-partition-yes | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 59 µs | 240 µs |
| 3-partition-no | families: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 320 µs | 606 µs |
| 3-partition-mll-yes | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 52 µs | 209 µs |
| 3-partition-mll-no | families: classical auto j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 45 µs | 181 µs |
| partition-yes | families: classical auto j1 | focus | 9 | 8 | 8 | 0 | 0 | 1 | 0 | 0 | 0 | 12.6 ms | 72.15 s |
| partition-no | families: classical auto j1 | focus | 7 | 5 | 0 | 5 | 0 | 2 | 0 | 0 | 0 | 410 µs | 3.28 s |
| qbf | families: classical auto j1 | focus | 36 | 34 | 6 | 28 | 0 | 0 | 0 | 2 | 0 | 120.3 ms | 698.30 s |
| wide-m1 | families: classical auto j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 118 µs | 206.3 ms |
| wide-m2 | families: classical auto j1 | focus | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 106 µs | 189.2 ms |
| wide-m3 | families: classical auto j1 | focus | 7 | 7 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 114 µs | 206.9 ms |
| wide-m4 | families: classical auto j1 | focus | 8 | 8 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 117 µs | 195.2 ms |
| mix | families: mix auto j1 | focus, inverse | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 257.3 ms | 1.05 s |
| counter | families: classical auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 35 µs | 224 µs |
| counter-over | families: classical auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 38 µs | 220 µs |
| growing | families: classical auto j1 | focus | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 18.2 ms | 132.5 ms |
| chain | families: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 34.0 ms | 314.6 ms |
| additive | families: classical auto j1 | additive | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 61.2 ms | 265.2 ms |
| lambek-fo | families: intuitionistic auto j1 | two-sided | 20 | 18 | 10 | 8 | 0 | 0 | 0 | 2 | 0 | 4.0 ms | 552.2 ms |
| chain-fo | families: classical auto j1 | focus | 8 | 3 | 3 | 0 | 0 | 0 | 3 | 2 | 0 | 7.5 ms | 317.6 ms |
| witness-fo | families: intuitionistic auto j1 | two-sided | 16 | 15 | 8 | 7 | 0 | 1 | 0 | 0 | 0 | 61 µs | 2.59 s |
| contraction | families: intuitionistic auto j1 | inverse, two-sided | 6 | 4 | 3 | 1 | 0 | 0 | 2 | 0 | 0 | 66.5 ms | 141.6 ms |
| pigeonhole | families: intuitionistic-affine auto j1 | two-sided | 6 | 6 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 284 µs | 39.8 ms |
| euler | families: intuitionistic auto j1 | two-sided | 6 | 6 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 151 µs | 2.3 ms |
| duplicate | families: intuitionistic auto j1 | horn | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 30 µs | 91 µs |
| duplicate | families: classical auto j1 | horn | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 28 µs | 82 µs |
| reach-fo | families: intuitionistic auto j1 | two-sided | 4 | 2 | 2 | 0 | 0 | 0 | 2 | 0 | 0 | 1.2 ms | 1.3 ms |
| counter | intuitionistic: intuitionistic auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 36 µs | 228 µs |
| counter-over | intuitionistic: intuitionistic auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 39 µs | 222 µs |
| chain | intuitionistic: intuitionistic auto j1 | inverse, two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 37.2 ms | 298.4 ms |
| 3-partition-yes | intuitionistic: intuitionistic auto j1 | two-sided | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 58 µs | 232 µs |
| 3-partition-no | intuitionistic: intuitionistic auto j1 | two-sided | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 275 µs | 275 µs |
| ILL/ILLTP-SYJ-01 | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 0 | 0 | 0 | 0 | 0 | 15 | 16 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 0 | 0 | 0 | 0 | 0 | 16 | 15 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores: intuitionistic auto j16 | two-sided | 31 | 2 | 2 | 0 | 0 | 2 | 6 | 21 | 0 | 2.48 s | 2.58 s |
| ILL/petri-nets/MCC | lltp-all-cores: intuitionistic auto j16 | horn | 1009 | 991 | 991 | 0 | 0 | 17 | 0 | 1 | 0 | 252 µs | 11.85 s |
| ILL/ILLTP-SYJ-01 | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 5 | 0 | 0 | 0 | 0 | 0 | 1 | 4 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 4 | 0 | 0 | 0 | 0 | 0 | 1 | 3 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores-generous: intuitionistic auto j16 | two-sided | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-classical: classical auto j1 | horn | 3137 | 3071 | 3071 | 0 | 0 | 58 | 0 | 8 | 0 | 386 µs | 37.17 s |
| ILL/misc | lltp-classical: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 51 µs | 213 µs |
| ILL/Non-theorems | lltp-classical: classical auto j1 | focus | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/KLE-cbv | lltp-classical: classical auto j1 | focus | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 0 | 46 µs | 3.8 ms |
| ILL/KLE-cbn | lltp-classical: classical auto j1 | focus | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 0 | 70 µs | 5.8 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-classical: classical auto j1 | focus | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 31 µs | 729 µs |
| ILL/KLE-IMP-CONJ | lltp-classical: classical auto j1 | focus | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 47 µs | 9.1 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-classical: classical auto j1 | focus | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 39 µs | 1.4 ms |
| ILL/KLE-01 | lltp-classical: classical auto j1 | focus | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 0 | 52 µs | 3.1 ms |
| ILL/ILLTP-SYN-cbv | lltp-classical: classical auto j1 | focus | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 0 | 43 µs | 806 µs |
| ILL/ILLTP-SYN-cbn | lltp-classical: classical auto j1 | focus | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 0 | 76 µs | 1.2 ms |
| ILL/ILLTP-SYN-01 | lltp-classical: classical auto j1 | focus | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 0 | 43 µs | 637 µs |
| ILL/ILLTP-SYJ-cbv | lltp-classical: classical auto j1 | focus | 252 | 43 | 19 | 24 | 0 | 21 | 176 | 12 | 0 | 61 µs | 3.42 s |
| ILL/ILLTP-SYJ-cbn | lltp-classical: classical auto j1 | focus, horn | 252 | 38 | 13 | 25 | 0 | 3 | 187 | 24 | 0 | 99 µs | 3.6 ms |
| ILL/ILLTP-SYJ-01 | lltp-classical: classical auto j1 | focus, horn | 252 | 34 | 10 | 24 | 0 | 2 | 192 | 24 | 0 | 78 µs | 3.3 ms |
| ILL/ILLTP-LCL-cbv | lltp-classical: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 57 µs | 57 µs |
| ILL/ILLTP-LCL-cbn | lltp-classical: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 95 µs | 153 µs |
| ILL/ILLTP-LCL-01 | lltp-classical: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 66 µs | 66 µs |
| CLL/misc | lltp-classical: classical auto j1 | focus | 14 | 8 | 8 | 0 | 0 | 0 | 6 | 0 | 0 | 35 µs | 330 µs |
| CLL/Non-theorems | lltp-classical: classical auto j1 | focus | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 37 µs | 144 µs |
| ILL/ILLTP-SYJ-01 | lltp-classical-generous: classical auto j1 | focus | 4 | 0 | 0 | 0 | 0 | 1 | 1 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-classical-generous: classical auto j1 | focus | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-classical-generous: classical auto j1 | focus | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-classical-generous: classical auto j1 | horn | 25 | 25 | 25 | 0 | 0 | 0 | 0 | 0 | 0 | 329 µs | 52.3 ms |
| ILL/ILLTP-LCL-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 111 µs | 196 µs |
| ILL/ILLTP-LCL-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 121 µs | 186 µs |
| ILL/ILLTP-LCL-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 78 µs | 150 µs |
| ILL/ILLTP-SYJ-01 | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 94 | 52 | 42 | 0 | 61 | 54 | 0 | 4 | 1.8 ms | 6.55 s |
| ILL/ILLTP-SYJ-cbn | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 124 | 64 | 60 | 0 | 49 | 36 | 0 | 4 | 1.2 ms | 15.73 s |
| ILL/ILLTP-SYJ-cbv | lltp-copies-10: intuitionistic auto j1 | inverse, two-sided | 209 | 123 | 76 | 47 | 0 | 56 | 30 | 0 | 4 | 7.6 ms | 52.96 s |
| ILL/ILLTP-SYN-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 7 | 3 | 4 | 0 | 0 | 1 | 0 | 0 | 95 µs | 849 µs |
| ILL/ILLTP-SYN-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 8 | 3 | 5 | 0 | 0 | 0 | 0 | 0 | 109 µs | 792 µs |
| ILL/ILLTP-SYN-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 8 | 8 | 3 | 5 | 0 | 0 | 0 | 0 | 0 | 53 µs | 544 µs |
| ILL/KLE-01 | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 37 | 28 | 9 | 0 | 0 | 1 | 0 | 8 | 136 µs | 37.9 ms |
| ILL/KLE-IMP-CONJ | lltp-copies-10: intuitionistic auto j1 | two-sided | 60 | 60 | 60 | 0 | 0 | 0 | 0 | 0 | 0 | 109 µs | 7.4 ms |
| ILL/KLE-cbn | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 38 | 28 | 10 | 0 | 0 | 0 | 0 | 9 | 105 µs | 9.2 ms |
| ILL/KLE-cbv | lltp-copies-10: intuitionistic auto j1 | two-sided | 38 | 37 | 28 | 9 | 0 | 0 | 1 | 0 | 8 | 69 µs | 10.3 ms |
| ILL/Non-theorems | lltp-copies-10: intuitionistic auto j1 | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-copies-10: intuitionistic auto j1 | horn | 171 | 171 | 171 | 0 | 0 | 0 | 0 | 0 | 0 | 191 µs | 1.86 s |
| ILL/ILLTP-LCL-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 168 µs | 276 µs |
| ILL/ILLTP-LCL-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 128 µs | 231 µs |
| ILL/ILLTP-LCL-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 124 µs | 245 µs |
| ILL/ILLTP-SYJ-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | horn, inverse, two-sided | 252 | 137 | 67 | 70 | 0 | 87 | 0 | 28 | 4 | 17.3 ms | 19.06 s |
| ILL/ILLTP-SYJ-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | horn, inverse, two-sided | 252 | 164 | 85 | 79 | 0 | 61 | 0 | 27 | 4 | 1.2 ms | 8.86 s |
| ILL/ILLTP-SYJ-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | inverse, two-sided | 252 | 154 | 81 | 73 | 0 | 77 | 0 | 21 | 4 | 12.4 ms | 30.57 s |
| ILL/ILLTP-SYN-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 94 µs | 6.3 ms |
| ILL/ILLTP-SYN-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 3 | 125 µs | 2.1 ms |
| ILL/ILLTP-SYN-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 88 µs | 1.9 ms |
| ILL/KLE-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | inverse, two-sided | 88 | 87 | 76 | 11 | 1 | 0 | 0 | 0 | 11 | 121 µs | 620.0 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 66 µs | 2.0 ms |
| ILL/KLE-IMP-CONJ | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 222 | 222 | 222 | 0 | 0 | 0 | 0 | 0 | 0 | 89 µs | 24.1 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 62 µs | 1.4 ms |
| ILL/KLE-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 105 µs | 161.4 ms |
| ILL/KLE-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 92 µs | 206.1 ms |
| ILL/Non-theorems | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | – | – |
| ILL/misc | lltp-default: intuitionistic auto j16 pool after 0.1s | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 89 µs | 307 µs |
| ILL/petri-nets/MCC | lltp-default: intuitionistic auto j16 pool after 0.1s | horn | 3137 | 3069 | 3069 | 0 | 0 | 66 | 0 | 2 | 0 | 836 µs | 35.91 s |
| ILL/ILLTP-LCL-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 119 µs | 220 µs |
| ILL/ILLTP-LCL-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 77 µs | 145 µs |
| ILL/ILLTP-LCL-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 110 µs | 207 µs |
| ILL/ILLTP-SYJ-01 | lltp-forward: intuitionistic auto j1 bias factors | horn, inverse, two-sided | 252 | 138 | 68 | 70 | 0 | 90 | 0 | 24 | 4 | 15.8 ms | 39.09 s |
| ILL/ILLTP-SYJ-cbn | lltp-forward: intuitionistic auto j1 bias factors | horn, inverse, two-sided | 252 | 170 | 86 | 84 | 0 | 59 | 0 | 23 | 4 | 616 µs | 23.23 s |
| ILL/ILLTP-SYJ-cbv | lltp-forward: intuitionistic auto j1 bias factors | inverse, two-sided | 252 | 157 | 84 | 73 | 0 | 83 | 0 | 12 | 4 | 11.3 ms | 75.98 s |
| ILL/ILLTP-SYN-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 61 µs | 5.0 ms |
| ILL/ILLTP-SYN-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 6 | 13 | 0 | 0 | 0 | 0 | 3 | 48 µs | 1.1 ms |
| ILL/ILLTP-SYN-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 19 | 19 | 7 | 12 | 0 | 0 | 0 | 0 | 2 | 55 µs | 1.3 ms |
| ILL/KLE-01 | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 87 | 76 | 11 | 0 | 0 | 1 | 0 | 10 | 75 µs | 1.32 s |
| ILL/KLE-IMP-CONJ/ALT | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 50 µs | 1.5 ms |
| ILL/KLE-IMP-CONJ | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 222 | 222 | 222 | 0 | 0 | 0 | 0 | 0 | 0 | 58 µs | 17.5 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 41 µs | 1.0 ms |
| ILL/KLE-cbn | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 88 | 76 | 12 | 0 | 0 | 0 | 0 | 11 | 55 µs | 51.6 ms |
| ILL/KLE-cbv | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 88 | 87 | 76 | 11 | 0 | 0 | 1 | 0 | 10 | 62 µs | 107.4 ms |
| ILL/Non-theorems | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-forward: intuitionistic auto j1 bias factors | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 69 µs | 220 µs |
| ILL/petri-nets/MCC | lltp-forward: intuitionistic auto j1 bias factors | horn | 3137 | 3071 | 3071 | 0 | 0 | 63 | 0 | 3 | 0 | 573 µs | 51.15 s |
| ILL/ILLTP-LCL-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 97 µs | 97 µs |
| ILL/ILLTP-LCL-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 124 µs | 185 µs |
| ILL/ILLTP-LCL-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 64 µs | 64 µs |
| ILL/ILLTP-SYJ-01 | lltp-intuitionistic: intuitionistic auto j1 | horn, two-sided | 252 | 34 | 10 | 24 | 0 | 4 | 189 | 25 | 3 | 79 µs | 3.3 ms |
| ILL/ILLTP-SYJ-cbn | lltp-intuitionistic: intuitionistic auto j1 | horn, two-sided | 252 | 38 | 13 | 25 | 0 | 4 | 186 | 24 | 4 | 115 µs | 4.1 ms |
| ILL/ILLTP-SYJ-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 252 | 43 | 19 | 24 | 0 | 23 | 174 | 12 | 3 | 66 µs | 3.75 s |
| ILL/ILLTP-SYN-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 2 | 42 µs | 630 µs |
| ILL/ILLTP-SYN-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 3 | 72 µs | 1.3 ms |
| ILL/ILLTP-SYN-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 2 | 47 µs | 843 µs |
| ILL/KLE-01 | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 10 | 53 µs | 3.2 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 41 µs | 1.3 ms |
| ILL/KLE-IMP-CONJ | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 49 µs | 9.3 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 34 µs | 785 µs |
| ILL/KLE-cbn | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 10 | 83 µs | 5.6 ms |
| ILL/KLE-cbv | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 10 | 47 µs | 3.8 ms |
| ILL/Non-theorems | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-intuitionistic: intuitionistic auto j1 | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 48 µs | 210 µs |
| ILL/petri-nets/MCC | lltp-intuitionistic: intuitionistic auto j1 | horn | 3137 | 3071 | 3071 | 0 | 0 | 57 | 0 | 9 | 0 | 415 µs | 37.88 s |
| ILL/ILLTP-SYJ-01 | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 4 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-intuitionistic-generous: intuitionistic auto j1 | two-sided | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-intuitionistic-generous: intuitionistic auto j1 | horn | 19 | 19 | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 398 µs | 52.1 ms |
| ILL/ILLTP-LCL-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 123 µs | 123 µs |
| ILL/ILLTP-LCL-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 74 µs | 139 µs |
| ILL/ILLTP-LCL-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 2 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 81 µs | 81 µs |
| ILL/ILLTP-SYJ-01 | lltp-rarer: intuitionistic auto j1 bias rarer | horn, two-sided | 252 | 34 | 10 | 24 | 0 | 5 | 189 | 24 | 3 | 101 µs | 4.2 ms |
| ILL/ILLTP-SYJ-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | horn, two-sided | 252 | 37 | 13 | 24 | 0 | 7 | 185 | 23 | 4 | 79 µs | 3.0 ms |
| ILL/ILLTP-SYJ-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 252 | 43 | 19 | 24 | 0 | 24 | 173 | 12 | 3 | 87 µs | 4.79 s |
| ILL/ILLTP-SYN-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 14 | 4 | 10 | 0 | 0 | 5 | 0 | 2 | 64 µs | 879 µs |
| ILL/ILLTP-SYN-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 18 | 6 | 12 | 0 | 0 | 1 | 0 | 3 | 52 µs | 1.1 ms |
| ILL/ILLTP-SYN-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 19 | 18 | 7 | 11 | 0 | 0 | 1 | 0 | 2 | 56 µs | 1.1 ms |
| ILL/KLE-01 | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 58 | 48 | 10 | 0 | 0 | 30 | 0 | 10 | 69 µs | 4.2 ms |
| ILL/KLE-IMP-CONJ/ALT | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 27 | 27 | 27 | 0 | 0 | 0 | 0 | 0 | 0 | 55 µs | 1.5 ms |
| ILL/KLE-IMP-CONJ | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 222 | 162 | 162 | 0 | 0 | 0 | 60 | 0 | 0 | 59 µs | 9.9 ms |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 22 | 22 | 0 | 22 | 0 | 0 | 0 | 0 | 0 | 49 µs | 1.2 ms |
| ILL/KLE-cbn | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 76 | 66 | 10 | 0 | 0 | 12 | 0 | 10 | 61 µs | 4.9 ms |
| ILL/KLE-cbv | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 88 | 75 | 64 | 11 | 0 | 0 | 13 | 0 | 10 | 68 µs | 5.3 ms |
| ILL/Non-theorems | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | – | – |
| ILL/misc | lltp-rarer: intuitionistic auto j1 bias rarer | two-sided | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 58 µs | 190 µs |
| ILL/petri-nets/MCC | lltp-rarer: intuitionistic auto j1 bias rarer | horn | 3137 | 3071 | 3071 | 0 | 0 | 64 | 0 | 2 | 0 | 572 µs | 54.19 s |
| ILL/ILLTP-SYJ-01 | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 20 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbn | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 20 | 0 | 2 | 0 | – | – |
| ILL/ILLTP-SYJ-cbv | lltp-recursion: intuitionistic auto j1 | two-sided | 22 | 0 | 0 | 0 | 0 | 22 | 0 | 0 | 0 | – | – |
| ILL/petri-nets/MCC | lltp-recursion: intuitionistic auto j1 | horn | 929 | 895 | 895 | 0 | 0 | 32 | 0 | 2 | 0 | 3.5 ms | 17.24 s |
| ILL/petri-nets/MCC | lltp-recursion-generous: intuitionistic auto j1 | horn | 9 | 9 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 2.4 ms | 28.5 ms |
| 3-partition-no | long-1: classical auto j1 | focus | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 514 µs | 514 µs |
| partition-no | long-1: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 558 µs | 558 µs |
| partition-yes | long-1: classical auto j1 | focus | 2 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 289 µs | 289 µs |
| counter | long-1: classical auto j1 | horn | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 76 µs | 143 µs |
| mix | long-2: mix auto j1 | inverse | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 347.1 ms | 347.1 ms |
| wide-m3 | long-2: classical auto j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 173 µs | 173 µs |
| wide-m4 | long-2: classical auto j1 | focus | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 161 µs | 161 µs |
| qbf | long-2: classical auto j1 | focus | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 258.7 ms | 258.7 ms |
| 3-partition-yes | parallel: classical auto j1 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 88 µs | 354 µs |
| 3-partition-yes | parallel: classical auto j2 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 228 µs | 901 µs |
| 3-partition-yes | parallel: classical auto j4 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 294 µs | 1.1 ms |
| 3-partition-yes | parallel: classical auto j8 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 420 µs | 1.6 ms |
| 3-partition-yes | parallel: classical auto j16 | focus | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 693 µs | 2.5 ms |
| 3-partition-no | parallel: classical auto j1 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 453 µs | 844 µs |
| 3-partition-no | parallel: classical auto j2 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 531 µs | 1.1 ms |
| 3-partition-no | parallel: classical auto j4 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 481 µs | 906 µs |
| 3-partition-no | parallel: classical auto j8 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 496 µs | 967 µs |
| 3-partition-no | parallel: classical auto j16 | focus | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 671 µs | 1.3 ms |
| partition-yes | parallel: classical auto j1 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 277 µs | 67.50 s |
| partition-yes | parallel: classical auto j2 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 514 µs | 70.00 s |
| partition-yes | parallel: classical auto j4 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 562 µs | 73.01 s |
| partition-yes | parallel: classical auto j8 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 625 µs | 77.23 s |
| partition-yes | parallel: classical auto j16 | focus | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 854 µs | 84.02 s |
| partition-no | parallel: classical auto j1 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 566 µs | 3.53 s |
| partition-no | parallel: classical auto j2 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 815 µs | 2.07 s |
| partition-no | parallel: classical auto j4 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 664 µs | 1.22 s |
| partition-no | parallel: classical auto j8 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 644 µs | 806.2 ms |
| partition-no | parallel: classical auto j16 | focus | 4 | 3 | 0 | 3 | 0 | 1 | 0 | 0 | 0 | 793 µs | 580.8 ms |
| qbf | parallel: classical auto j1 | focus | 20 | 19 | 3 | 16 | 0 | 0 | 0 | 1 | 0 | 139.4 ms | 203.84 s |
| qbf | parallel: classical auto j2 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 100.4 ms | 14.57 s |
| qbf | parallel: classical auto j4 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 100.2 ms | 14.73 s |
| qbf | parallel: classical auto j8 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 100.7 ms | 14.43 s |
| qbf | parallel: classical auto j16 | focus | 20 | 14 | 3 | 11 | 0 | 0 | 0 | 6 | 0 | 102.2 ms | 14.37 s |
| mix | parallel: mix auto j1 | inverse | 4 | 4 | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 271.8 ms | 1.10 s |
| mix | parallel: mix auto j2 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 5.95 s | 76.43 s |
| mix | parallel: mix auto j4 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 6.17 s | 79.33 s |
| mix | parallel: mix auto j8 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 7.93 s | 93.79 s |
| mix | parallel: mix auto j16 | focus | 4 | 3 | 0 | 3 | 1 | 0 | 0 | 0 | 0 | 9.70 s | 119.21 s |
| counter | parallel: classical auto j1 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 71 µs | 390 µs |
| counter | parallel: classical auto j2 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 101 µs | 595 µs |
| counter | parallel: classical auto j4 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 100 µs | 597 µs |
| counter | parallel: classical auto j8 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 107 µs | 605 µs |
| counter | parallel: classical auto j16 | horn | 6 | 6 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 92 µs | 563 µs |
| counter-over | parallel: classical auto j1 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 57 µs | 344 µs |
| counter-over | parallel: classical auto j2 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 92 µs | 564 µs |
| counter-over | parallel: classical auto j4 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 106 µs | 573 µs |
| counter-over | parallel: classical auto j8 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 94 µs | 538 µs |
| counter-over | parallel: classical auto j16 | horn | 6 | 6 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 96 µs | 556 µs |
| wide-m3 | parallel: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 153 µs | 479 µs |
| wide-m3 | parallel: classical auto j2 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 337 µs | 988 µs |
| wide-m3 | parallel: classical auto j4 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 362 µs | 1.1 ms |
| wide-m3 | parallel: classical auto j8 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 483 µs | 1.5 ms |
| wide-m3 | parallel: classical auto j16 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 695 µs | 2.1 ms |
| wide-m4 | parallel: classical auto j1 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 152 µs | 487 µs |
| wide-m4 | parallel: classical auto j2 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 315 µs | 956 µs |
| wide-m4 | parallel: classical auto j4 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 381 µs | 1.1 ms |
| wide-m4 | parallel: classical auto j8 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 489 µs | 1.5 ms |
| wide-m4 | parallel: classical auto j16 | focus | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 682 µs | 2.1 ms |
| 3-partition-mll-no | parallel-net: classical net j1 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 994 µs | 3.0 ms |
| 3-partition-mll-no | parallel-net: classical net j2 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.1 ms | 2.8 ms |
| 3-partition-mll-no | parallel-net: classical net j4 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.1 ms | 3.5 ms |
| 3-partition-mll-no | parallel-net: classical net j8 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 3.7 ms |
| 3-partition-mll-no | parallel-net: classical net j16 | net | 3 | 3 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.4 ms | 4.2 ms |
| partition-table | parallel-net: classical net j1 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.7 ms | 5.09 s |
| partition-table | parallel-net: classical net j2 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 1.57 s |
| partition-table | parallel-net: classical net j4 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 1.4 ms | 1.25 s |
| partition-table | parallel-net: classical net j8 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 2.3 ms | 94.6 ms |
| partition-table | parallel-net: classical net j16 | net | 13 | 13 | 7 | 6 | 0 | 0 | 0 | 0 | 0 | 3.1 ms | 49.4 ms |
| wide-m1 | period-16: classical net j1 period 16 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 427.8 ms | 434.2 ms |
| wide-m2 | period-16: classical net j1 period 16 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 420.0 ms | 426.5 ms |
| 3-partition-mll-no | period-16: classical net j1 period 16 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 1.2 ms | 1.5 ms |
| partition-yes | period-16: classical net j1 period 16 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2.7 ms | 2.7 ms |
| partition-no | period-16: classical net j1 period 16 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 23.5 ms | 25.7 ms |
| wide-m1 | period-1: classical net j1 period 1 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 1.26 s | 1.28 s |
| wide-m2 | period-1: classical net j1 period 1 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 1.25 s | 1.27 s |
| 3-partition-mll-no | period-1: classical net j1 period 1 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 681 µs | 940 µs |
| partition-yes | period-1: classical net j1 period 1 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1.7 ms | 1.7 ms |
| partition-no | period-1: classical net j1 period 1 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 9.3 ms | 10.8 ms |
| wide-m1 | period-2: classical net j1 period 2 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 809.0 ms | 821.1 ms |
| wide-m2 | period-2: classical net j1 period 2 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 805.6 ms | 817.8 ms |
| 3-partition-mll-no | period-2: classical net j1 period 2 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 695 µs | 930 µs |
| partition-yes | period-2: classical net j1 period 2 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1.3 ms | 1.3 ms |
| partition-no | period-2: classical net j1 period 2 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 7.7 ms | 8.9 ms |
| wide-m1 | period-8: classical net j1 period 8 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 481.0 ms | 488.3 ms |
| wide-m2 | period-8: classical net j1 period 8 | net | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 475.0 ms | 482.2 ms |
| 3-partition-mll-no | period-8: classical net j1 period 8 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 709 µs | 955 µs |
| partition-yes | period-8: classical net j1 period 8 | net | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5.0 ms | 5.0 ms |
| partition-no | period-8: classical net j1 period 8 | net | 2 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 22.4 ms | 23.8 ms |

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
| counter | intuitionistic: intuitionistic auto j1 | 6 | 120 | 0 | 0 | 120 | 0 | 0 |
| counter-over | intuitionistic: intuitionistic auto j1 | 6 | 171 | 0 | 23 | 148 | 0 | 0 |
| chain | intuitionistic: intuitionistic auto j1 | 4 | 21155 | 1161275 | 10465 | 927 | 0 | 0 |
| 3-partition-yes | intuitionistic: intuitionistic auto j1 | 4 | 28 | 140 | 0 | 28 | 0 | 0 |
| 3-partition-no | intuitionistic: intuitionistic auto j1 | 1 | 923 | 2794 | 713 | 210 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-all-cores: intuitionistic auto j16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-all-cores: intuitionistic auto j16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-all-cores: intuitionistic auto j16 | 2 | 13451229 | 11663054 | 7995797 | 1281561 | 0 | 0 |
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
| ILL/ILLTP-SYJ-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 137 | 116486522 | 77149600 | 100052853 | 8203261 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 164 | 33483497 | 13759351 | 23675009 | 5634417 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 154 | 329574139 | 260656598 | 282426328 | 17404036 | 0 | 0 |
| ILL/ILLTP-SYN-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 27766 | 9512 | 16890 | 1189 | 0 | 0 |
| ILL/ILLTP-SYN-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 595 | 248 | 189 | 228 | 0 | 0 |
| ILL/ILLTP-SYN-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 19 | 934 | 807 | 352 | 257 | 0 | 0 |
| ILL/KLE-01 | lltp-default: intuitionistic auto j16 pool after 0.1s | 87 | 5640790 | 2698489 | 4752801 | 51240 | 0 | 0 |
| ILL/KLE-IMP-CONJ/ALT | lltp-default: intuitionistic auto j16 pool after 0.1s | 27 | 122 | 145 | 0 | 98 | 0 | 0 |
| ILL/KLE-IMP-CONJ | lltp-default: intuitionistic auto j16 pool after 0.1s | 222 | 26036 | 13152 | 12564 | 6583 | 0 | 0 |
| ILL/KLE-IMP-CONJ/NON-THEOREMS | lltp-default: intuitionistic auto j16 pool after 0.1s | 22 | 36 | 34 | 0 | 36 | 0 | 0 |
| ILL/KLE-cbn | lltp-default: intuitionistic auto j16 pool after 0.1s | 88 | 1380424 | 907862 | 1168689 | 9304 | 0 | 0 |
| ILL/KLE-cbv | lltp-default: intuitionistic auto j16 pool after 0.1s | 88 | 1428712 | 1462288 | 1208995 | 10429 | 0 | 0 |
| ILL/Non-theorems | lltp-default: intuitionistic auto j16 pool after 0.1s | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| ILL/misc | lltp-default: intuitionistic auto j16 pool after 0.1s | 3 | 46 | 74 | 16 | 22 | 0 | 0 |
| ILL/petri-nets/MCC | lltp-default: intuitionistic auto j16 pool after 0.1s | 3069 | 33140643 | 0 | 23242718 | 9897925 | 0 | 0 |
| ILL/ILLTP-LCL-01 | lltp-forward: intuitionistic auto j1 bias factors | 2 | 271 | 133 | 94 | 42 | 0 | 0 |
| ILL/ILLTP-LCL-cbn | lltp-forward: intuitionistic auto j1 bias factors | 2 | 72 | 33 | 24 | 21 | 0 | 0 |
| ILL/ILLTP-LCL-cbv | lltp-forward: intuitionistic auto j1 bias factors | 2 | 239 | 125 | 103 | 38 | 0 | 0 |
| ILL/ILLTP-SYJ-01 | lltp-forward: intuitionistic auto j1 bias factors | 138 | 37652069 | 10004425 | 33454707 | 2684852 | 0 | 0 |
| ILL/ILLTP-SYJ-cbn | lltp-forward: intuitionistic auto j1 bias factors | 170 | 31611880 | 8535059 | 21761380 | 2930075 | 0 | 0 |
| ILL/ILLTP-SYJ-cbv | lltp-forward: intuitionistic auto j1 bias factors | 157 | 81449870 | 121632038 | 65907459 | 9215490 | 0 | 0 |
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
| 3-partition-yes | parallel: classical auto j2 | 4 | 48 | 275 | 0 | 46 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j4 | 4 | 96 | 536 | 3 | 68 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j8 | 4 | 244 | 1386 | 29 | 150 | 0 | 0 |
| 3-partition-yes | parallel: classical auto j16 | 4 | 310 | 1794 | 40 | 187 | 0 | 0 |
| 3-partition-no | parallel: classical auto j1 | 2 | 1894 | 6088 | 1446 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j2 | 2 | 1899 | 6108 | 1449 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j4 | 2 | 2362 | 7695 | 1777 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j8 | 2 | 2551 | 8231 | 1953 | 448 | 0 | 0 |
| 3-partition-no | parallel: classical auto j16 | 2 | 3031 | 9655 | 2334 | 448 | 0 | 0 |
| partition-yes | parallel: classical auto j1 | 5 | 38546799 | 387076933 | 33786817 | 1098638 | 0 | 0 |
| partition-yes | parallel: classical auto j2 | 5 | 76893405 | 771146641 | 67406528 | 1098642 | 0 | 0 |
| partition-yes | parallel: classical auto j4 | 5 | 152504069 | 1522903378 | 133745580 | 1098722 | 0 | 0 |
| partition-yes | parallel: classical auto j8 | 5 | 276833237 | 2607045732 | 244062535 | 1098886 | 0 | 0 |
| partition-yes | parallel: classical auto j16 | 5 | 424912975 | 3822332030 | 376142676 | 1087721 | 0 | 0 |
| partition-no | parallel: classical auto j1 | 3 | 4252557 | 27627908 | 3720792 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j2 | 3 | 4633039 | 30198924 | 4046094 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j4 | 3 | 5128497 | 33248710 | 4475201 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j8 | 3 | 5421737 | 34832256 | 4740698 | 531765 | 0 | 0 |
| partition-no | parallel: classical auto j16 | 3 | 5939493 | 36764093 | 5215323 | 531765 | 0 | 0 |
| qbf | parallel: classical auto j1 | 19 | 258912034 | 1596527780 | 28181763 | 7813436 | 0 | 0 |
| qbf | parallel: classical auto j2 | 14 | 27115983 | 148796907 | 2702178 | 2632155 | 0 | 0 |
| qbf | parallel: classical auto j4 | 14 | 27106779 | 148782871 | 2702177 | 2622952 | 0 | 0 |
| qbf | parallel: classical auto j8 | 14 | 27114024 | 148793932 | 2702176 | 2630196 | 0 | 0 |
| qbf | parallel: classical auto j16 | 14 | 27117793 | 148799667 | 2702176 | 2633965 | 0 | 0 |
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
| partition-table | parallel-net: classical net j2 | 13 | 1277271 | 0 | 0 | 0 | 1383871 | 455412 |
| partition-table | parallel-net: classical net j4 | 13 | 1986308 | 0 | 0 | 0 | 2184916 | 661925 |
| partition-table | parallel-net: classical net j8 | 13 | 82325 | 0 | 0 | 0 | 154770 | 104140 |
| partition-table | parallel-net: classical net j16 | 13 | 28675 | 0 | 0 | 0 | 111565 | 44602 |
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

## partition-yes

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| partition-yes/4 | 68 µs ✓ | 1.8 ms ✓ | 69 µs ✓ |  |  |  |  |  |  | 2.7 ms ✓ | 1.7 ms ✓ | 1.3 ms ✓ | 5.0 ms ✓ |
| partition-yes/5 | 79 µs ✓ | 28.5 ms ✓ | 86 µs ✓ |  | 128 µs ✓ | 321 µs ✓ | 361 µs ✓ | 409 µs ✓ | 845 µs ✓ |  |  |  |  |
| partition-yes/6 | 130 µs ✓ | 476.1 ms ✓ | 131 µs ✓ |  | 173 µs ✓ | 404 µs ✓ | 377 µs ✓ | 539 µs ✓ | 621 µs ✓ |  |  |  |  |
| partition-yes/12 | 12.8 ms ✓ | > 60 s | 12.6 ms ✓ |  |  |  |  |  |  |  |  |  |  |
| partition-yes/20 | 426.4 ms ✓ | > 60 s | 411.7 ms ✓ |  | 389.6 ms ✓ | 416.1 ms ✓ | 426.4 ms ✓ | 419.2 ms ✓ | 437.1 ms ✓ |  |  |  |  |
| partition-yes/7 |  |  | 196 µs ✓ | 289 µs ✓ | 277 µs ✓ | 514 µs ✓ | 562 µs ✓ | 625 µs ✓ | 854 µs ✓ |  |  |  |  |
| partition-yes/16 |  |  | 134.4 ms ✓ |  |  |  |  |  |  |  |  |  |  |
| partition-yes/24 |  |  | 71.59 s ✓ |  | 67.11 s ✓ | 69.59 s ✓ | 72.58 s ✓ | 76.81 s ✓ | 83.58 s ✓ |  |  |  |  |
| partition-yes/28 |  |  | > 300 s | > 1200 s |  |  |  |  |  |  |  |  |  |

## partition-no

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| partition-no/3 | 82 µs ✗ | 1.5 ms ✗ | 76 µs ✗ |  |  |  |  |  |  | 2.2 ms ✗ | 1.5 ms ✗ | 1.2 ms ✗ | 1.4 ms ✗ |
| partition-no/4 | 147 µs ✗ | 9.6 ms ✗ | 136 µs ✗ |  | 198 µs ✗ | 411 µs ✗ | 351 µs ✗ | 439 µs ✗ | 666 µs ✗ | 23.5 ms ✗ | 9.3 ms ✗ | 7.7 ms ✗ | 22.4 ms ✗ |
| partition-no/9 | 80.2 ms ✗ | > 60 s | 72.4 ms ✗ |  |  |  |  |  |  |  |  |  |  |
| partition-no/12 | 3.47 s ✗ | > 60 s | 3.21 s ✗ |  | 3.53 s ✗ | 2.07 s ✗ | 1.22 s ✗ | 805.1 ms ✗ | 579.3 ms ✗ |  |  |  |  |
| partition-no/5 |  |  | 410 µs ✗ | 558 µs ✗ | 566 µs ✗ | 815 µs ✗ | 664 µs ✗ | 644 µs ✗ | 793 µs ✗ |  |  |  |  |
| partition-no/14 |  |  | > 300 s |  | > 120 s | > 120 s | > 120 s | > 120 s | > 120 s |  |  |  |  |
| partition-no/15 |  |  | > 300 s | > 1200 s |  |  |  |  |  |  |  |  |  |

## 3-partition-mll-yes

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 |
|---|--:|--:|--:|
| 3-partition-mll-yes/4 | 50 µs ✓ | 76 µs ✓ | 51 µs ✓ |
| 3-partition-mll-yes/6 | 49 µs ✓ | 102 µs ✓ | 52 µs ✓ |
| 3-partition-mll-yes/8 | 52 µs ✓ | 132 µs ✓ | 55 µs ✓ |
| 3-partition-mll-yes/12 | 55 µs ✓ | 222 µs ✓ | 51 µs ✓ |

## 3-partition-mll-no

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | parallel-net: classical net j1 | parallel-net: classical net j2 | parallel-net: classical net j4 | parallel-net: classical net j8 | parallel-net: classical net j16 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-mll-no/4 | 37 µs ✗ | 250 µs ✗ | 45 µs ✗ | 368 µs ✗ | 502 µs ✗ | 551 µs ✗ | 583 µs ✗ | 814 µs ✗ | 259 µs ✗ | 259 µs ✗ | 235 µs ✗ | 246 µs ✗ |
| 3-partition-mll-no/5 | 39 µs ✗ | 689 µs ✗ | 47 µs ✗ | 994 µs ✗ | 1.1 ms ✗ | 1.1 ms ✗ | 1.2 ms ✗ | 1.4 ms ✗ | 1.2 ms ✗ | 681 µs ✗ | 695 µs ✗ | 709 µs ✗ |
| 3-partition-mll-no/6 | 42 µs ✗ | 1.2 ms ✗ | 44 µs ✗ | 1.6 ms ✗ | 1.2 ms ✗ | 1.8 ms ✗ | 1.9 ms ✗ | 2.1 ms ✗ |  |  |  |  |
| 3-partition-mll-no/8 | 44 µs ✗ | 3.0 ms ✗ | 45 µs ✗ |  |  |  |  |  |  |  |  |  |

## wide-m1

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|
| wide-m1/8 | 50 µs ✓ | 61 µs ✓ | 54 µs ✓ |  |  |  |  |
| wide-m1/16 | 75 µs ✓ | 129 µs ✓ | 66 µs ✓ |  |  |  |  |
| wide-m1/24 | 89 µs ✓ | 212 µs ✓ | 88 µs ✓ |  |  |  |  |
| wide-m1/32 | 121 µs ✓ | 188 µs ✓ | 118 µs ✓ |  |  |  |  |
| wide-m1/256 | 3.3 ms ✓ | 9.0 ms ✓ | 3.4 ms ✓ | 6.4 ms ✓ | 18.5 ms ✓ | 12.0 ms ✓ | 7.3 ms ✓ |
| wide-m1/2048 | 209.2 ms ✓ | 598.9 ms ✓ | 202.6 ms ✓ | 427.8 ms ✓ | 1.26 s ✓ | 809.0 ms ✓ | 481.0 ms ✓ |

## wide-m2

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | period-16: classical net j1 period 16 | period-1: classical net j1 period 1 | period-2: classical net j1 period 2 | period-8: classical net j1 period 8 |
|---|--:|--:|--:|--:|--:|--:|--:|
| wide-m2/8 | 52 µs ✓ | 63 µs ✓ | 54 µs ✓ |  |  |  |  |
| wide-m2/16 | 67 µs ✓ | 124 µs ✓ | 65 µs ✓ |  |  |  |  |
| wide-m2/24 | 86 µs ✓ | 216 µs ✓ | 84 µs ✓ |  |  |  |  |
| wide-m2/32 | 106 µs ✓ | 188 µs ✓ | 106 µs ✓ |  |  |  |  |
| wide-m2/256 | 2.8 ms ✓ | 9.0 ms ✓ | 2.6 ms ✓ | 6.5 ms ✓ | 18.3 ms ✓ | 12.2 ms ✓ | 7.2 ms ✓ |
| wide-m2/2048 | 200.5 ms ✓ | 593.2 ms ✓ | 186.2 ms ✓ | 420.0 ms ✓ | 1.25 s ✓ | 805.6 ms ✓ | 475.0 ms ✓ |

## wide-m3

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| wide-m3/12 | 60 µs ✓ | 96 µs ✓ | 60 µs ✓ |  |  |  |  |  |  |
| wide-m3/24 | 79 µs ✓ | 226 µs ✓ | 80 µs ✓ |  | 152 µs ✓ | 301 µs ✓ | 328 µs ✓ | 483 µs ✓ | 679 µs ✓ |
| wide-m3/30 | 104 µs ✓ | 177 µs ✓ | 100 µs ✓ |  | 153 µs ✓ | 337 µs ✓ | 362 µs ✓ | 459 µs ✓ | 695 µs ✓ |
| wide-m3/256 | 2.5 ms ✓ | 9.3 ms ✓ | 2.4 ms ✓ |  |  |  |  |  |  |
| wide-m3/2048 | 179.1 ms ✓ | 630.5 ms ✓ | 163.2 ms ✓ |  |  |  |  |  |  |
| wide-m3/36 |  |  | 114 µs ✓ | 173 µs ✓ | 174 µs ✓ | 350 µs ✓ | 372 µs ✓ | 519 µs ✓ | 727 µs ✓ |
| wide-m3/1024 |  |  | 41.0 ms ✓ |  |  |  |  |  |  |

## wide-m4

| problem | engines: classical focus j1 | engines: classical net j1 | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| wide-m4/12 | 58 µs ✓ | 90 µs ✓ | 62 µs ✓ |  |  |  |  |  |  |
| wide-m4/24 | 83 µs ✓ | 224 µs ✓ | 82 µs ✓ |  |  |  |  |  |  |
| wide-m4/28 | 91 µs ✓ | 290 µs ✓ | 92 µs ✓ |  | 139 µs ✓ | 315 µs ✓ | 334 µs ✓ | 469 µs ✓ | 682 µs ✓ |
| wide-m4/256 | 2.3 ms ✓ | 9.4 ms ✓ | 2.2 ms ✓ |  |  |  |  |  |  |
| wide-m4/2048 | 166.7 ms ✓ | 633.1 ms ✓ | 154.0 ms ✓ |  |  |  |  |  |  |
| wide-m4/32 |  |  | 104 µs ✓ |  | 152 µs ✓ | 306 µs ✓ | 381 µs ✓ | 507 µs ✓ | 679 µs ✓ |
| wide-m4/36 |  |  | 117 µs ✓ | 161 µs ✓ | 196 µs ✓ | 335 µs ✓ | 403 µs ✓ | 489 µs ✓ | 702 µs ✓ |
| wide-m4/1024 |  |  | 38.5 ms ✓ |  |  |  |  |  |  |

## additive

| problem | engines: classical focus j1 | families: classical auto j1 |
|---|--:|--:|
| additive/8 | 740 µs ✓ | 370 µs ✓ |
| additive/12 | 31.6 ms ✓ | 11.4 ms ✓ |
| additive/14 | 328.5 ms ✓ | 61.2 ms ✓ |
| additive/16 | 5.28 s ✓ | 192.2 ms ✓ |

## partition-table

| problem | engines: classical focus j1 | engines: classical net j1 | parallel-net: classical net j1 | parallel-net: classical net j2 | parallel-net: classical net j4 | parallel-net: classical net j8 | parallel-net: classical net j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| partition-table/1-1 | 50 µs ✓ | 53 µs ✓ | 73 µs ✓ | 205 µs ✓ | 232 µs ✓ | 273 µs ✓ | 513 µs ✓ |
| partition-table/1-3 | 50 µs ✗ | 118 µs ✗ | 182 µs ✗ | 292 µs ✗ | 311 µs ✗ | 394 µs ✗ | 587 µs ✗ |
| partition-table/2-1-1 | 60 µs ✓ | 169 µs ✓ | 273 µs ✓ | 471 µs ✓ | 493 µs ✓ | 597 µs ✓ | 767 µs ✓ |
| partition-table/1-1-4 | 72 µs ✗ | 559 µs ✗ | 782 µs ✗ | 851 µs ✗ | 919 µs ✗ | 961 µs ✗ | 1.2 ms ✗ |
| partition-table/2-2-1-1 | 74 µs ✓ | 928 µs ✓ | 1.3 ms ✓ | 1.0 ms ✓ | 1.2 ms ✓ | 2.3 ms ✓ | 2.5 ms ✓ |
| partition-table/1-2-5 | 74 µs ✗ | 1.3 ms ✗ | 1.7 ms ✗ | 1.2 ms ✗ | 1.9 ms ✗ | 2.0 ms ✗ | 2.2 ms ✗ |
| partition-table/1-1-2-4 | 58 µs ✓ | 889 µs ✓ | 1.2 ms ✓ | 1.2 ms ✓ | 802 µs ✓ | 2.3 ms ✓ | 5.8 ms ✓ |
| partition-table/1-1-1-5 | 132 µs ✗ | 3.1 ms ✗ | 4.2 ms ✗ | 2.6 ms ✗ | 1.7 ms ✗ | 2.7 ms ✗ | 4.3 ms ✗ |
| partition-table/2-3-2-1 | 76 µs ✓ | 2.5 ms ✓ | 3.4 ms ✓ | 2.0 ms ✓ | 1.4 ms ✓ | 2.3 ms ✓ | 5.3 ms ✓ |
| partition-table/3-3-3-1 | 138 µs ✗ | 8.3 ms ✗ | 10.5 ms ✗ | 5.8 ms ✗ | 3.4 ms ✗ | 4.7 ms ✗ | 4.4 ms ✗ |
| partition-table/1-2-3-4-5-5 | 86 µs ✓ | 1.08 s ✓ | 1.04 s ✓ | 122.6 ms ✓ | 1.6 ms ✓ | 54.2 ms ✓ | 6.7 ms ✓ |
| partition-table/1-1-1-1-1-7 | 1.1 ms ✗ | 106.5 ms ✗ | 113.1 ms ✗ | 61.4 ms ✗ | 34.1 ms ✗ | 20.3 ms ✗ | 12.1 ms ✗ |
| partition-table/2-2-2-2-2-2-9-1 | 106 µs ✓ | 4.01 s ✓ | 3.91 s ✓ | 1.37 s ✓ | 1.20 s ✓ | 1.6 ms ✓ | 3.1 ms ✓ |

## cancellation

| problem | engines: classical focus j1 |
|---|--:|
| cancellation/3-partition-4 | 278 µs ✓ |

## 3-partition-yes

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-yes/4 | 59 µs ✓ | 58 µs ✓ | 93 µs ✓ | 211 µs ✓ | 294 µs ✓ | 391 µs ✓ | 518 µs ✓ |
| 3-partition-yes/6 | 59 µs ✓ | 56 µs ✓ | 85 µs ✓ | 228 µs ✓ | 309 µs ✓ | 420 µs ✓ | 546 µs ✓ |
| 3-partition-yes/8 | 59 µs ✓ | 55 µs ✓ | 88 µs ✓ | 256 µs ✓ | 291 µs ✓ | 426 µs ✓ | 727 µs ✓ |
| 3-partition-yes/12 | 63 µs ✓ | 63 µs ✓ | 88 µs ✓ | 206 µs ✓ | 254 µs ✓ | 369 µs ✓ | 693 µs ✓ |

## 3-partition-no

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| 3-partition-no/4 | 286 µs ✗ | 275 µs ✗ |  | 391 µs ✗ | 531 µs ✗ | 425 µs ✗ | 471 µs ✗ | 671 µs ✗ |
| 3-partition-no/5 | 320 µs ✗ |  | 514 µs ✗ | 453 µs ✗ | 525 µs ✗ | 481 µs ✗ | 496 µs ✗ | 642 µs ✗ |

## qbf

| problem | families: classical auto j1 | long-2: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| qbf/8#0 | 176 µs ✗ |  |  |  |  |  |  |
| qbf/8#1 | 190 µs ✗ |  |  |  |  |  |  |
| qbf/8#2 | 152 µs ✓ |  |  |  |  |  |  |
| qbf/8#3 | 179 µs ✗ |  |  |  |  |  |  |
| qbf/12#0 | 1.2 ms ✓ |  |  |  |  |  |  |
| qbf/12#1 | 942 µs ✗ |  |  |  |  |  |  |
| qbf/12#2 | 597 µs ✓ |  |  |  |  |  |  |
| qbf/12#3 | 891 µs ✗ |  |  |  |  |  |  |
| qbf/16#0 | 5.2 ms ✓ |  | 7.5 ms ✓ | 7.9 ms ✓ | 7.7 ms ✓ | 7.6 ms ✓ | 8.0 ms ✓ |
| qbf/16#1 | 8.0 ms ✗ |  | 11.8 ms ✗ | 11.4 ms ✗ | 11.8 ms ✗ | 11.7 ms ✗ | 12.2 ms ✗ |
| qbf/16#2 | 8.5 ms ✗ |  | 12.7 ms ✗ | 12.5 ms ✗ | 12.5 ms ✗ | 13.2 ms ✗ | 13.7 ms ✗ |
| qbf/16#3 | 4.4 ms ✓ |  | 6.6 ms ✓ | 7.4 ms ✓ | 7.0 ms ✓ | 7.3 ms ✓ | 7.4 ms ✓ |
| qbf/20#0 | 80.3 ms ✗ |  | 102.5 ms ✗ | 100.3 ms ✗ | 100.2 ms ✗ | 100.7 ms ✗ | 101.7 ms ✗ |
| qbf/20#1 | 82.2 ms ✗ |  | 101.5 ms ✗ | 100.4 ms ✗ | 98.5 ms ✗ | 100.7 ms ✗ | 101.2 ms ✗ |
| qbf/20#2 | 26.4 ms ✓ |  | 34.9 ms ✓ | 50.9 ms ✓ | 45.4 ms ✓ | 51.6 ms ✓ | 50.7 ms ✓ |
| qbf/20#3 | 80.5 ms ✗ |  | 97.5 ms ✗ | 96.5 ms ✗ | 95.0 ms ✗ | 97.8 ms ✗ | 102.2 ms ✗ |
| qbf/24#0 | 174.9 ms ✗ | 258.7 ms ✗ | 203.8 ms ✗ | 209.9 ms ✗ | 210.5 ms ✗ | 209.4 ms ✗ | 215.9 ms ✗ |
| qbf/24#1 | 120.5 ms ✗ |  | 139.4 ms ✗ | 138.0 ms ✗ | 133.1 ms ✗ | 135.2 ms ✗ | 131.7 ms ✗ |
| qbf/24#2 | 109.2 ms ✗ |  | 125.1 ms ✗ | 128.9 ms ✗ | 130.2 ms ✗ | 131.6 ms ✗ | 133.1 ms ✗ |
| qbf/24#3 | 120.3 ms ✗ |  | 139.9 ms ✗ | 136.9 ms ✗ | 133.3 ms ✗ | 133.7 ms ✗ | 135.8 ms ✗ |
| qbf/32#0 | 561.0 ms ✗ |  |  |  |  |  |  |
| qbf/32#1 | 593.4 ms ✗ |  |  |  |  |  |  |
| qbf/32#2 | 1.03 s ✗ |  |  |  |  |  |  |
| qbf/32#3 | 487.3 ms ✗ |  |  |  |  |  |  |
| qbf/40#0 | 10.00 s ✗ |  | 10.51 s ✗ | 6.81 s ✗ | 6.78 s ✗ | 6.57 s ✗ | 6.49 s ✗ |
| qbf/40#1 | 11.42 s ✗ |  | 11.71 s ✗ | 5.39 s ? memory | 5.40 s ? memory | 5.42 s ? memory | 4.85 s ? memory |
| qbf/40#2 | 13.33 s ✗ |  | 13.56 s ✗ | 5.10 s ? memory | 5.14 s ? memory | 5.13 s ? memory | 5.14 s ? memory |
| qbf/40#3 | 10.46 s ✗ |  | 10.44 s ✗ | 6.76 s ✗ | 6.97 s ✗ | 6.86 s ✗ | 6.87 s ✗ |
| qbf/44#0 | 44.75 s ✗ |  | 44.79 s ✗ | 7.70 s ? memory | 7.72 s ? memory | 7.72 s ? memory | 7.90 s ? memory |
| qbf/44#1 | 45.37 s ✗ |  | 44.51 s ✗ | 6.55 s ? memory | 6.52 s ? memory | 6.53 s ? memory | 6.62 s ? memory |
| qbf/44#2 | 66.52 s ✗ |  | 67.34 s ✗ | 4.62 s ? memory | 4.60 s ? memory | 4.58 s ? memory | 4.67 s ? memory |
| qbf/44#3 | 23.23 s ? memory |  | 23.28 s ? memory | 7.68 s ? memory | 7.76 s ? memory | 7.70 s ? memory | 7.68 s ? memory |
| qbf/48#0 | 70.03 s ? memory | 101.59 s ? memory |  |  |  |  |  |
| qbf/48#1 | 166.29 s ✗ |  |  |  |  |  |  |
| qbf/48#2 | 141.52 s ✗ |  |  |  |  |  |  |
| qbf/48#3 | 185.15 s ✗ |  |  |  |  |  |  |

## mix

| problem | families: mix auto j1 | long-2: mix auto j1 | parallel: mix auto j1 | parallel: mix auto j2 | parallel: mix auto j4 | parallel: mix auto j8 | parallel: mix auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| mix/4 | 330 µs ✗ |  |  |  |  |  |  |
| mix/6 | 11.2 ms ✗ |  |  |  |  |  |  |
| mix/8 | 256.8 ms ✗ |  | 267.9 ms ✗ | 806.9 ms ✗ | 872.7 ms ✗ | 1.08 s ✗ | 1.40 s ✗ |
| mix/9 | 257.3 ms ✗ |  | 271.5 ms ✗ | 5.95 s ✗ | 6.17 s ✗ | 7.93 s ✗ | 9.70 s ✗ |
| mix/10 | 258.7 ms ✗ |  | 271.8 ms ✗ | 69.67 s ✗ | 72.29 s ✗ | 84.78 s ✗ | 108.11 s ✗ |
| mix/11 | 269.6 ms ✗ | 347.1 ms ✗ | 293.2 ms ✗ | 120.00 s ✗ | 120.01 s ✗ | 120.01 s ✗ | 120.01 s ✗ |

## counter

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | long-1: classical auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| counter/2 | 35 µs ✓ | 29 µs ✓ |  | 53 µs ✓ | 104 µs ✓ | 85 µs ✓ | 88 µs ✓ | 92 µs ✓ |
| counter/4 | 29 µs ✓ | 28 µs ✓ |  | 53 µs ✓ | 85 µs ✓ | 105 µs ✓ | 82 µs ✓ | 82 µs ✓ |
| counter/8 | 32 µs ✓ | 32 µs ✓ |  | 78 µs ✓ | 93 µs ✓ | 94 µs ✓ | 97 µs ✓ | 83 µs ✓ |
| counter/16 | 35 µs ✓ | 36 µs ✓ | 67 µs ✓ | 56 µs ✓ | 88 µs ✓ | 100 µs ✓ | 107 µs ✓ | 88 µs ✓ |
| counter/32 | 40 µs ✓ | 46 µs ✓ |  | 71 µs ✓ | 101 µs ✓ | 100 µs ✓ | 108 µs ✓ | 101 µs ✓ |
| counter/64 | 53 µs ✓ | 57 µs ✓ | 76 µs ✓ | 79 µs ✓ | 124 µs ✓ | 113 µs ✓ | 123 µs ✓ | 117 µs ✓ |

## counter-over

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 | parallel: classical auto j1 | parallel: classical auto j2 | parallel: classical auto j4 | parallel: classical auto j8 | parallel: classical auto j16 |
|---|--:|--:|--:|--:|--:|--:|--:|
| counter-over/2 | 28 µs ✗ | 29 µs ✗ | 51 µs ✗ | 85 µs ✗ | 83 µs ✗ | 94 µs ✗ | 77 µs ✗ |
| counter-over/4 | 31 µs ✗ | 31 µs ✗ | 48 µs ✗ | 76 µs ✗ | 77 µs ✗ | 77 µs ✗ | 99 µs ✗ |
| counter-over/8 | 36 µs ✗ | 35 µs ✗ | 51 µs ✗ | 90 µs ✗ | 80 µs ✗ | 75 µs ✗ | 92 µs ✗ |
| counter-over/16 | 38 µs ✗ | 39 µs ✗ | 57 µs ✗ | 95 µs ✗ | 106 µs ✗ | 91 µs ✗ | 92 µs ✗ |
| counter-over/32 | 42 µs ✗ | 40 µs ✗ | 69 µs ✗ | 92 µs ✗ | 108 µs ✗ | 98 µs ✗ | 96 µs ✗ |
| counter-over/64 | 45 µs ✗ | 48 µs ✗ | 68 µs ✗ | 126 µs ✗ | 119 µs ✗ | 103 µs ✗ | 100 µs ✗ |

## growing

| problem | families: classical auto j1 |
|---|--:|
| growing/16 | 231 µs ✗ |
| growing/64 | 808 µs ✗ |
| growing/256 | 18.2 ms ✗ |
| growing/1024 | 113.3 ms ✗ |

## chain

| problem | families: classical auto j1 | intuitionistic: intuitionistic auto j1 |
|---|--:|--:|
| chain/16 | 152 µs ✓ | 159 µs ✓ |
| chain/64 | 4.4 ms ✓ | 4.7 ms ✓ |
| chain/128 | 34.0 ms ✓ | 37.2 ms ✓ |
| chain/256 | 276.1 ms ✓ | 256.4 ms ✓ |

## lambek-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| lambek-fo/16#0 | 125 µs ✓ |
| lambek-fo/16#1 | 83 µs ✗ |
| lambek-fo/16#2 | 247 µs ✓ |
| lambek-fo/16#3 | 89 µs ✓ |
| lambek-fo/16#4 | 86 µs ✗ |
| lambek-fo/64#0 | 470 µs ✓ |
| lambek-fo/64#1 | 552 µs ✗ |
| lambek-fo/64#2 | 6.7 ms ✓ |
| lambek-fo/64#3 | 406 µs ✓ |
| lambek-fo/64#4 | 415 µs ✗ |
| lambek-fo/256#0 | 4.0 ms ✓ |
| lambek-fo/256#1 | 9.9 ms ✗ |
| lambek-fo/256#2 | 429.6 ms ✓ |
| lambek-fo/256#3 | 4.5 ms ✓ |
| lambek-fo/256#4 | 5.8 ms ✗ |
| lambek-fo/512#0 | 13.3 ms ? depth |
| lambek-fo/512#1 | 46.1 ms ✗ |
| lambek-fo/512#2 | 3.87 s ? depth |
| lambek-fo/512#3 | 18.3 ms ✓ |
| lambek-fo/512#4 | 24.8 ms ✗ |

## chain-fo

| problem | families: classical auto j1 |
|---|--:|
| chain-fo/16#0 | 370 µs ✓ |
| chain-fo/16#1 | 345 µs ? bound |
| chain-fo/64#0 | 7.5 ms ✓ |
| chain-fo/64#1 | 6.9 ms ? bound |
| chain-fo/256#0 | 309.7 ms ✓ |
| chain-fo/256#1 | 275.7 ms ? bound |
| chain-fo/512#0 | 1.19 s ? depth |
| chain-fo/512#1 | 1.03 s ? depth |

## witness-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| witness-fo/4#0 | 56 µs ✓ |
| witness-fo/4#1 | 50 µs ✗ |
| witness-fo/4#2 | 55 µs ✓ |
| witness-fo/4#3 | 68 µs ✗ |
| witness-fo/8#0 | 59 µs ✓ |
| witness-fo/8#1 | 55 µs ✗ |
| witness-fo/8#2 | 58 µs ✓ |
| witness-fo/8#3 | 28.2 ms ✗ |
| witness-fo/10#0 | 62 µs ✓ |
| witness-fo/10#1 | 56 µs ✗ |
| witness-fo/10#2 | 62 µs ✓ |
| witness-fo/10#3 | 2.57 s ✗ |
| witness-fo/12#0 | 66 µs ✓ |
| witness-fo/12#1 | 61 µs ✗ |
| witness-fo/12#2 | 69 µs ✓ |
| witness-fo/12#3 | > 300 s |

## contraction

| problem | families: intuitionistic auto j1 |
|---|--:|
| contraction/2#0 | 57 µs ✓ |
| contraction/2#1 | 73 µs ? bound |
| contraction/4#0 | 154 µs ✓ |
| contraction/4#1 | 488 µs ? bound |
| contraction/8#0 | 66.5 ms ✓ |
| contraction/8#1 | 74.8 ms ✗ |

## pigeonhole

| problem | families: intuitionistic-affine auto j1 |
|---|--:|
| pigeonhole/2#0 | 71 µs ✓ |
| pigeonhole/2#1 | 40 µs ✗ |
| pigeonhole/3#0 | 951 µs ✓ |
| pigeonhole/3#1 | 76 µs ✗ |
| pigeonhole/4#0 | 38.4 ms ✓ |
| pigeonhole/4#1 | 284 µs ✗ |

## euler

| problem | families: intuitionistic auto j1 |
|---|--:|
| euler/5#0 | 55 µs ✓ |
| euler/5#1 | 151 µs ✗ |
| euler/6#0 | 72 µs ✓ |
| euler/6#1 | 344 µs ✗ |
| euler/8#0 | 72 µs ✓ |
| euler/8#1 | 1.6 ms ✗ |

## duplicate

| problem | families: intuitionistic auto j1 | families: classical auto j1 |
|---|--:|--:|
| duplicate/2#0 | 28 µs ✓ |  |
| duplicate/4#0 | 33 µs ✓ |  |
| duplicate/8#0 | 30 µs ✓ |  |
| duplicate/2#1 |  | 25 µs ✓ |
| duplicate/4#1 |  | 29 µs ✓ |
| duplicate/8#1 |  | 28 µs ✓ |

## reach-fo

| problem | families: intuitionistic auto j1 |
|---|--:|
| reach-fo/2#0 | 110 µs ✓ |
| reach-fo/2#1 | 123 µs ? bound |
| reach-fo/4#0 | 1.2 ms ✓ |
| reach-fo/4#1 | 1.6 ms ? bound |
