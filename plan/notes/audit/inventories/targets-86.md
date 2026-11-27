## 86 against 47

### Verdicts that differ

None.

### Counters that differ

Of the 145 problems both decide alike on one thread, 0 differ in nodes, splits, memo_hits, memo_entries, links, tests.

### Decided by 86, not by 47

None.

### Decided by 47, not by 86

None.

### By group

Per group: the problems in both; how many each decided (late: after the time limit); how many one decided and the other did not; where the undecided runs ended; the median ratio of 47's time to 86's on the problems both decide within the limit (how many); and the problems only 47 has. An undecided run ended at `time` (the time limit), `killed` (past it, or while loading), `bound` (the copy bound), `depth` (the recursion limit), `memory` (the memory limit), `index` (an index past its width), `wide` (a context too wide to split), `crash` (the run crashed) or `error` (an error, or another reason such as the work limit).

| file | family | configuration | problems | 86 decided | 47 decided | only 86 | only 47 | 86 ended | 47 ended | 47/86 | new |
|---|---|---|--:|--:|--:|--:|--:|---|---|--:|--:|
|  | 3-partition-no | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.08× (2) | 0 |
|  | ILL/ILLTP-LCL-cbv | intuitionistic auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 0.71× (1) | 0 |
|  | ILL/ILLTP-SYJ-01 | intuitionistic auto j1 | 7 | 0 | 0 | 0 | 0 | 6 bound, 1 depth | 6 bound, 1 depth | – | 0 |
|  | ILL/ILLTP-SYJ-cbn | intuitionistic auto j1 | 6 | 1 | 1 | 0 | 0 | 5 bound | 5 bound | 0.75× (1) | 0 |
|  | ILL/ILLTP-SYJ-cbv | intuitionistic auto j1 | 6 | 0 | 0 | 0 | 0 | 5 bound, 1 depth | 5 bound, 1 depth | – | 0 |
|  | ILL/ILLTP-SYN-01 | intuitionistic auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 0.55× (1) | 0 |
|  | ILL/KLE-01 | intuitionistic auto j1 | 2 | 1 | 1 | 0 | 0 | 1 bound | 1 bound | 0.90× (1) | 0 |
|  | ILL/KLE-IMP-CONJ | intuitionistic auto j1 | 2 | 0 | 0 | 0 | 0 | 2 bound | 2 bound | – | 0 |
|  | ILL/KLE-cbn | intuitionistic auto j1 | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
|  | ILL/KLE-cbv | intuitionistic auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 0.80× (1) | 0 |
|  | ILL/petri-nets/MCC | intuitionistic auto j1 | 91 | 89 | 89 | 0 | 0 | 2 time | 1 time, 1 memory | 0.96× (89) | 0 |
|  | cancellation | classical auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 1.06× (1) | 0 |
|  | chain | classical auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.10× (4) | 0 |
|  | chain-over | intuitionistic auto j1 | 1 | 0 | 0 | 0 | 0 | 1 bound | 1 bound | – | 0 |
|  | counter | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.12× (2) | 0 |
|  | counter | intuitionistic auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 0.97× (2) | 0 |
|  | counter-over | classical auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 0.97× (1) | 0 |
|  | counter-over | intuitionistic auto j1 | 1 | 1 | 1 | 0 | 0 | – | – | 1.14× (1) | 0 |
|  | growing | classical auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.10× (4) | 0 |
|  | mix | mix auto j1 | 4 | 4 | 4 | 0 | 0 | – | – | 1.11× (4) | 0 |
|  | partition-no | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.05× (2) | 0 |
|  | partition-table | classical auto j1 | 13 | 13 | 13 | 0 | 0 | – | – | 1.04× (13) | 0 |
|  | partition-yes | classical auto j1 | 3 | 3 | 3 | 0 | 0 | – | – | 1.11× (3) | 0 |
|  | qbf | classical auto j1 | 8 | 8 | 8 | 0 | 0 | – | – | 1.10× (8) | 0 |
|  | wide-m3 | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.11× (2) | 0 |
|  | wide-m4 | classical auto j1 | 2 | 2 | 2 | 0 | 0 | – | – | 1.06× (2) | 0 |

### Generated and listed problems

| file | family | configuration | problem | 86 | 47 | 47/86 |
|---|---|---|---|--:|--:|--:|
|  | 3-partition-no | classical auto j1 | 3-partition-no/4 | 287 µs ✗ | 290 µs ✗ | 1.01× |
|  | 3-partition-no | classical auto j1 | 3-partition-no/5 | 309 µs ✗ | 334 µs ✗ | 1.08× |
|  | cancellation | classical auto j1 | cancellation/3-partition-4 | 263 µs ✓ | 280 µs ✓ | 1.06× |
|  | chain | classical auto j1 | chain/128 | 33.5 ms ✓ | 37.5 ms ✓ | 1.12× |
|  | chain | classical auto j1 | chain/16 | 147 µs ✓ | 161 µs ✓ | 1.10× |
|  | chain | classical auto j1 | chain/256 | 277.9 ms ✓ | 305.4 ms ✓ | 1.10× |
|  | chain | classical auto j1 | chain/64 | 4.3 ms ✓ | 4.8 ms ✓ | 1.09× |
|  | chain-over | intuitionistic auto j1 | chain-over/12 | 154 µs ? bound | 164 µs ? bound |  |
|  | counter | classical auto j1 | counter/16 | 32 µs ✓ | 36 µs ✓ | 1.12× |
|  | counter | classical auto j1 | counter/8 | 32 µs ✓ | 34 µs ✓ | 1.06× |
|  | counter | intuitionistic auto j1 | counter/16 | 35 µs ✓ | 34 µs ✓ | 0.97× |
|  | counter | intuitionistic auto j1 | counter/8 | 32 µs ✓ | 30 µs ✓ | 0.94× |
|  | counter-over | classical auto j1 | counter-over/8 | 32 µs ✗ | 31 µs ✗ | 0.97× |
|  | counter-over | intuitionistic auto j1 | counter-over/8 | 35 µs ✗ | 40 µs ✗ | 1.14× |
|  | growing | classical auto j1 | growing/1024 | 114.0 ms ✗ | 124.9 ms ✗ | 1.10× |
|  | growing | classical auto j1 | growing/16 | 224 µs ✗ | 251 µs ✗ | 1.12× |
|  | growing | classical auto j1 | growing/256 | 18.4 ms ✗ | 20.3 ms ✗ | 1.10× |
|  | growing | classical auto j1 | growing/64 | 799 µs ✗ | 832 µs ✗ | 1.04× |
|  | mix | mix auto j1 | mix/10 | 257.1 ms ✗ | 287.7 ms ✗ | 1.12× |
|  | mix | mix auto j1 | mix/11 | 267.6 ms ✗ | 296.8 ms ✗ | 1.11× |
|  | mix | mix auto j1 | mix/8 | 253.5 ms ✗ | 281.7 ms ✗ | 1.11× |
|  | mix | mix auto j1 | mix/9 | 255.9 ms ✗ | 284.9 ms ✗ | 1.11× |
|  | partition-no | classical auto j1 | partition-no/4 | 135 µs ✗ | 138 µs ✗ | 1.02× |
|  | partition-no | classical auto j1 | partition-no/5 | 412 µs ✗ | 431 µs ✗ | 1.05× |
|  | partition-table | classical auto j1 | partition-table/1-1 | 40 µs ✓ | 42 µs ✓ | 1.05× |
|  | partition-table | classical auto j1 | partition-table/1-1-1-1-1-7 | 1.0 ms ✗ | 1.1 ms ✗ | 1.06× |
|  | partition-table | classical auto j1 | partition-table/1-1-1-5 | 127 µs ✗ | 128 µs ✗ | 1.01× |
|  | partition-table | classical auto j1 | partition-table/1-1-2-4 | 59 µs ✓ | 56 µs ✓ | 0.95× |
|  | partition-table | classical auto j1 | partition-table/1-1-4 | 63 µs ✗ | 69 µs ✗ | 1.10× |
|  | partition-table | classical auto j1 | partition-table/1-2-3-4-5-5 | 78 µs ✓ | 83 µs ✓ | 1.06× |
|  | partition-table | classical auto j1 | partition-table/1-2-5 | 69 µs ✗ | 71 µs ✗ | 1.03× |
|  | partition-table | classical auto j1 | partition-table/1-3 | 45 µs ✗ | 45 µs ✗ | 1.00× |
|  | partition-table | classical auto j1 | partition-table/2-1-1 | 52 µs ✓ | 53 µs ✓ | 1.02× |
|  | partition-table | classical auto j1 | partition-table/2-2-1-1 | 63 µs ✓ | 65 µs ✓ | 1.03× |
|  | partition-table | classical auto j1 | partition-table/2-2-2-2-2-2-9-1 | 100 µs ✓ | 106 µs ✓ | 1.06× |
|  | partition-table | classical auto j1 | partition-table/2-3-2-1 | 63 µs ✓ | 71 µs ✓ | 1.13× |
|  | partition-table | classical auto j1 | partition-table/3-3-3-1 | 127 µs ✗ | 132 µs ✗ | 1.04× |
|  | partition-yes | classical auto j1 | partition-yes/5 | 79 µs ✓ | 88 µs ✓ | 1.11× |
|  | partition-yes | classical auto j1 | partition-yes/6 | 117 µs ✓ | 127 µs ✓ | 1.09× |
|  | partition-yes | classical auto j1 | partition-yes/7 | 186 µs ✓ | 215 µs ✓ | 1.16× |
|  | qbf | classical auto j1 | qbf/16#0 | 5.2 ms ✓ | 5.8 ms ✓ | 1.11× |
|  | qbf | classical auto j1 | qbf/16#1 | 7.8 ms ✗ | 8.5 ms ✗ | 1.09× |
|  | qbf | classical auto j1 | qbf/16#2 | 8.2 ms ✗ | 9.0 ms ✗ | 1.10× |
|  | qbf | classical auto j1 | qbf/16#3 | 4.6 ms ✓ | 4.9 ms ✓ | 1.06× |
|  | qbf | classical auto j1 | qbf/20#0 | 80.1 ms ✗ | 86.5 ms ✗ | 1.08× |
|  | qbf | classical auto j1 | qbf/20#1 | 80.4 ms ✗ | 89.5 ms ✗ | 1.11× |
|  | qbf | classical auto j1 | qbf/20#2 | 26.0 ms ✓ | 28.9 ms ✓ | 1.11× |
|  | qbf | classical auto j1 | qbf/20#3 | 80.2 ms ✗ | 85.7 ms ✗ | 1.07× |
|  | wide-m3 | classical auto j1 | wide-m3/24 | 79 µs ✓ | 88 µs ✓ | 1.11× |
|  | wide-m3 | classical auto j1 | wide-m3/30 | 95 µs ✓ | 97 µs ✓ | 1.02× |
|  | wide-m4 | classical auto j1 | wide-m4/24 | 73 µs ✓ | 76 µs ✓ | 1.04× |
|  | wide-m4 | classical auto j1 | wide-m4/28 | 84 µs ✓ | 89 µs ✓ | 1.06× |

