# Tools compared

212 problems, 13 tools, limits 60 s and 4294967296 bytes.

## Solved within the limit

| tool | solved | proved | unprovable | wrong | late | time | memory | gave up | error | outside | against the header |
|---|---|---|---|---|---|---|---|---|---|---|---|
| linlog | 179 | 117 | 62 | 0 | 0 | 11 | 1 | 11 | 0 | 0 | 0 |
| linlog-default | 179 | 117 | 62 | 0 | 0 | 11 | 1 | 11 | 0 | 0 | 0 |
| linearone | 58 | 32 | 26 | 0 | 0 | 3 | 0 | 0 | 0 | 18 | 0 |
| sympli | 23 | 16 | 7 | 0 | 0 | 4 | 1 | 0 | 0 | 20 | 0 |
| maude-cll | 13 | 13 | 0 | 0 | 0 | 1 | 3 | 0 | 0 | 21 | 0 |
| apll-ll | 10 | 10 | 0 | 8 | 0 | 3 | 0 | 4 | 0 | 21 | 0 |
| maude-ill | 10 | 7 | 3 | 0 | 0 | 4 | 0 | 13 | 2 | 20 | 0 |
| click-and-collect | 6 | 6 | 0 | 8 | 0 | 3 | 0 | 0 | 0 | 21 | 0 |
| apll-ill | 3 | 3 | 0 | 0 | 0 | 4 | 0 | 24 | 2 | 20 | 0 |
| lintap | 3 | 3 | 0 | 0 | 0 | 2 | 1 | 0 | 0 | 22 | 0 |
| llprover-ill | 3 | 3 | 0 | 0 | 0 | 5 | 0 | 0 | 0 | 20 | 0 |
| lolli | 3 | 3 | 0 | 0 | 1 | 2 | 0 | 7 | 2 | 22 | 0 |
| llprover-cll | 2 | 2 | 0 | 0 | 0 | 4 | 0 | 0 | 0 | 21 | 0 |

`wrong`: a refutation of a problem some tool proved with a checked proof, or a verdict against a generated problem's by construction, never counted as solved; `late`: decided after the limit; `gave up`: the tool's own bound or no answer; `outside`: not run, outside the tool's fragment or mode; `against the header`: a verdict, counted as solved, that contradicts an LLTP file's header, which a few of the library's files have wrong.

## By group: solved / in the fragment

| group | problems | linlog | maude-ill | maude-cll | click-and-collect | llprover-cll | llprover-ill | lintap | linearone | sympli | lolli | apll-ll | apll-ill | linlog-default |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| generated families | 212 | 179 / 202 | 10 / 29 | 13 / 17 | 6 / 17 | 2 / 6 | 3 / 8 | 3 / 6 | 58 / 61 | 23 / 28 | 3 / 15 | 10 / 25 | 3 / 33 | 179 / 202 |

## linlog against each tool, on the problems both cover

| tool | both cover | both solve | linlog alone | the tool alone | neither |
|---|---|---|---|---|---|
| maude-ill | 29 | 10 | 18 | 0 | 1 |
| maude-cll | 17 | 13 | 3 | 0 | 1 |
| click-and-collect | 15 | 6 | 8 | 0 | 1 |
| llprover-cll | 6 | 2 | 4 | 0 | 0 |
| llprover-ill | 8 | 3 | 5 | 0 | 0 |
| lintap | 6 | 3 | 3 | 0 | 0 |
| linearone | 59 | 48 | 0 | 8 | 3 |
| sympli | 28 | 23 | 4 | 0 | 1 |
| lolli | 15 | 3 | 12 | 0 | 0 |
| apll-ll | 23 | 10 | 11 | 0 | 2 |
| apll-ill | 27 | 3 | 22 | 0 | 2 |
| linlog-default | 202 | 179 | 0 | 0 | 23 |

Solved by linearone and not by linlog (8):

- lambek-fo/lambek-fo/1024#0 (intuitionistic)
- lambek-fo/lambek-fo/1024#2 (intuitionistic)
- lambek-fo/lambek-fo/1024#3 (intuitionistic)
- lambek-fo/lambek-fo/1024#4 (intuitionistic)
- lambek-fo/lambek-fo/2048#4 (intuitionistic)
- lambek-fo/lambek-fo/512#0 (intuitionistic)
- lambek-fo/lambek-fo/512#2 (intuitionistic)
- witness-fo/witness-fo/12#3 (intuitionistic)

## Translation and memory

| tool | translated | translation, median | translation, most | peak memory of solved runs, median | most |
|---|---|---|---|---|---|
| linlog | 202 | 0 µs | 4.8 ms | 1.2 MiB | 1616.0 MiB |
| maude-ill | 29 | 0 µs | 25.0 ms | 43.6 MiB | 223.6 MiB |
| maude-cll | 17 | 0 µs | 200 µs | 27.7 MiB | 1934.8 MiB |
| click-and-collect | 17 | 0 µs | 6.8 ms | 2.0 MiB | 64.2 MiB |
| llprover-cll | 6 | 0 µs | 100 µs | 8.0 MiB | 8.0 MiB |
| llprover-ill | 8 | 0 µs | 0 µs | 8.4 MiB | 10.0 MiB |
| lintap | 6 | 0 µs | 100 µs | 8.8 MiB | 9.0 MiB |
| linearone | 61 | 100 µs | 1.7 ms | 10.1 MiB | 365.2 MiB |
| sympli | 28 | 0 µs | 400 µs | 5.6 MiB | 522.0 MiB |
| lolli | 15 | 0 µs | 200 µs | 1.9 MiB | 1.9 MiB |
| apll-ll | 25 | 0 µs | 7.1 ms | 1.8 MiB | 4.0 MiB |
| apll-ill | 33 | 0 µs | 23.6 ms | 1.0 MiB | 1.0 MiB |
| linlog-default | 202 | 0 µs | 4.8 ms | 1.8 MiB | 3072.4 MiB |

linlog reads LLTP and `.spec` files itself and the rest as JSON, which the harness writes; the translation time is the harness's, outside the measured run.

## Answered after the limit

| problem | tool | verdict | time |
|---|---|---|---|
| counter/counter/8 (classical) | lolli | proved | 60.78 s |

## Contradictions

A problem on which one tool proves what another refutes, or a tool decides against the known verdict. A proof is checked where the row says so (linlog's command checks every proof it reports, Sympli validates its own); no tool's `unprovable` is checked, linlog's included.

| problem | expected | linlog | maude-ill | maude-cll | click-and-collect | llprover-cll | llprover-ill | lintap | linearone | sympli | lolli | apll-ll | apll-ill | linlog-default |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| wide-m1/wide-m1/1024 (classical) | provable | proved (checked) | – | – | unprovable (unchecked) | – | – | – | – | – | – | unprovable (unchecked) | – | proved (checked) |
| wide-m1/wide-m1/16384 (classical) | provable | – | – | – | unprovable (unchecked) | – | – | – | – | – | – | unprovable (unchecked) | – | – |
| wide-m1/wide-m1/2048 (classical) | provable | proved (checked) | – | – | unprovable (unchecked) | – | – | – | – | – | – | unprovable (unchecked) | – | proved (checked) |
| wide-m1/wide-m1/256 (classical) | provable | proved (checked) | outside | memory | unprovable (unchecked) | time | outside | memory | outside | outside | time | unprovable (unchecked) | outside | proved (checked) |
| wide-m1/wide-m1/32768 (classical) | provable | – | – | – | unprovable (unchecked) | – | – | – | – | – | – | unprovable (unchecked) | – | – |
| wide-m1/wide-m1/4096 (classical) | provable | proved (checked) | – | – | unprovable (unchecked) | – | – | – | – | – | – | unprovable (unchecked) | – | proved (checked) |
| wide-m1/wide-m1/512 (classical) | provable | proved (checked) | – | – | unprovable (unchecked) | – | – | – | – | – | – | unprovable (unchecked) | – | proved (checked) |
| wide-m1/wide-m1/8192 (classical) | provable | memory | – | – | unprovable (unchecked) | – | – | – | – | – | – | unprovable (unchecked) | – | memory |
