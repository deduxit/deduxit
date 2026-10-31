# Tools compared

50 problems, 12 tools, limits 10 s and 4294967296 bytes.

## Solved within the limit

| tool | solved | proved | unprovable | wrong | late | time | memory | gave up | error | outside | against the header |
|---|---|---|---|---|---|---|---|---|---|---|---|
| linlog | 47 | 30 | 17 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 0 |
| linearone | 14 | 8 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 36 | 0 |
| sympli | 10 | 6 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 40 | 0 |
| maude-ill | 8 | 6 | 2 | 0 | 0 | 0 | 0 | 2 | 0 | 40 | 0 |
| apll-ll | 6 | 6 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 42 | 0 |
| maude-cll | 6 | 6 | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 42 | 0 |
| click-and-collect | 5 | 5 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 42 | 0 |
| apll-ill | 3 | 3 | 0 | 0 | 0 | 4 | 0 | 3 | 0 | 40 | 0 |
| lintap | 3 | 3 | 0 | 0 | 0 | 1 | 2 | 0 | 0 | 44 | 0 |
| llprover-ill | 3 | 3 | 0 | 0 | 0 | 7 | 0 | 0 | 0 | 40 | 0 |
| lolli | 3 | 3 | 0 | 0 | 0 | 3 | 1 | 1 | 0 | 42 | 0 |
| llprover-cll | 2 | 2 | 0 | 0 | 0 | 6 | 0 | 0 | 0 | 42 | 0 |

`wrong`: a refutation of a problem some tool proved with a checked proof, or a verdict against a generated problem's by construction, never counted as solved; `late`: decided after the limit; `gave up`: the tool's own bound or no answer; `outside`: not run, outside the tool's fragment or mode; `against the header`: a verdict, counted as solved, that contradicts an LLTP file's header, which a few of the library's files have wrong.

## By group: solved / in the fragment

| group | problems | linlog | maude-ill | maude-cll | click-and-collect | llprover-cll | llprover-ill | lintap | linearone | sympli | lolli | apll-ll | apll-ill |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| generated families | 50 | 47 / 50 | 8 / 10 | 6 / 8 | 5 / 8 | 2 / 8 | 3 / 10 | 3 / 6 | 14 / 14 | 10 / 10 | 3 / 8 | 6 / 8 | 3 / 10 |

## linlog against each tool, on the problems both cover

| tool | both cover | both solve | linlog alone | the tool alone | neither |
|---|---|---|---|---|---|
| maude-ill | 10 | 8 | 2 | 0 | 0 |
| maude-cll | 8 | 6 | 2 | 0 | 0 |
| click-and-collect | 8 | 5 | 3 | 0 | 0 |
| llprover-cll | 8 | 2 | 6 | 0 | 0 |
| llprover-ill | 10 | 3 | 7 | 0 | 0 |
| lintap | 6 | 3 | 3 | 0 | 0 |
| linearone | 14 | 14 | 0 | 0 | 0 |
| sympli | 10 | 10 | 0 | 0 | 0 |
| lolli | 8 | 3 | 5 | 0 | 0 |
| apll-ll | 8 | 6 | 2 | 0 | 0 |
| apll-ill | 10 | 3 | 7 | 0 | 0 |

## Translation and memory

| tool | translated | translation, median | translation, most | peak memory of solved runs, median | most |
|---|---|---|---|---|---|
| linlog | 50 | 0 µs | 100 µs | 1.2 MiB | 19.5 MiB |
| maude-ill | 10 | 0 µs | 0 µs | 33.8 MiB | 50.0 MiB |
| maude-cll | 8 | 0 µs | 100 µs | 16.1 MiB | 23.2 MiB |
| click-and-collect | 8 | 0 µs | 100 µs | 1.5 MiB | 62.2 MiB |
| llprover-cll | 8 | 0 µs | 100 µs | 8.0 MiB | 8.0 MiB |
| llprover-ill | 10 | 0 µs | 0 µs | 8.4 MiB | 10.0 MiB |
| lintap | 6 | 0 µs | 100 µs | 8.8 MiB | 9.0 MiB |
| linearone | 14 | 0 µs | 0 µs | 9.9 MiB | 9.9 MiB |
| sympli | 10 | 0 µs | 0 µs | 1.2 MiB | 10.5 MiB |
| lolli | 8 | 0 µs | 100 µs | 1.9 MiB | 1.9 MiB |
| apll-ll | 8 | 0 µs | 100 µs | 1.0 MiB | 1.2 MiB |
| apll-ill | 10 | 0 µs | 0 µs | 1.0 MiB | 1.2 MiB |

linlog reads LLTP and `.spec` files itself and the rest as JSON, which the harness writes; the translation time is the harness's, outside the measured run.

## Answered after the limit

None.

## Contradictions

A problem on which one tool proves what another refutes, or a tool decides against the known verdict. A proof is checked where the row says so (linlog's command checks every proof it reports, Sympli validates its own); no tool's `unprovable` is checked, linlog's included.

| problem | expected | linlog | maude-ill | maude-cll | click-and-collect | llprover-cll | llprover-ill | lintap | linearone | sympli | lolli | apll-ll | apll-ill |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| wide-m1/wide-m1/256 (classical) | provable | proved (checked) | outside | time | unprovable (unchecked) | time | outside | memory | outside | outside | time | unprovable (unchecked) | outside |
| wide-m1/wide-m1/512 (classical) | provable | proved (checked) | outside | memory | unprovable (unchecked) | time | outside | memory | outside | outside | time | unprovable (unchecked) | outside |
