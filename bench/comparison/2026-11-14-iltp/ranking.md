# Tools compared

274 problems, 3 tools, limits 10 s and 4294967296 bytes.

## Solved within the limit

| tool | solved | proved | unprovable | wrong | late | time | memory | gave up | error | outside | against the header |
|---|---|---|---|---|---|---|---|---|---|---|---|
| intuit | 263 | 128 | 135 | 0 | 0 | 11 | 0 | 0 | 0 | 0 | 0 |
| intuitr | 263 | 128 | 135 | 0 | 0 | 11 | 0 | 0 | 0 | 0 | 0 |
| deduxit-default | 254 | 119 | 135 | 0 | 1 | 18 | 0 | 1 | 0 | 0 | 0 |

`wrong`: a refutation of a problem some tool proved with a checked proof, or a verdict against a generated problem's by construction, never counted as solved; `late`: decided after the limit; `gave up`: the tool's own bound or no answer; `outside`: not run, outside the tool's fragment or mode; `against the header`: a verdict, counted as solved, that contradicts an LLTP file's header, which a few of the library's files have wrong.

## By group: solved / in the fragment

| group | problems | deduxit-default | intuit | intuitr |
|---|---|---|---|---|
| ILTP (ordinary logic) | 274 | 254 / 274 | 263 / 274 | 263 / 274 |

## Translation and memory

| tool | translated | translation, median | translation, most | peak memory of solved runs, median | most |
|---|---|---|---|---|---|
| deduxit-default | 274 | 0 µs | 0 µs | 1.2 MiB | 342.9 MiB |
| intuit | 274 | 0 µs | 0 µs | 4.5 MiB | 14.0 MiB |
| intuitr | 274 | 0 µs | 0 µs | 5.0 MiB | 16.0 MiB |

deduxit reads LLTP and `.spec` files itself and the rest as JSON, which the harness writes; the translation time is the harness's, outside the measured run.

## Answered after the limit

| problem | tool | verdict | time |
|---|---|---|---|
| SYJ206/SYJ206+1.018.p (intuitionistic) | deduxit-default | proved | 10.14 s |

## Contradictions

A problem on which one tool proves what another refutes, or a tool decides against the known verdict. A proof is checked where the row says so (deduxit's command checks every proof it reports, Sympli validates its own); no tool's `unprovable` is checked, deduxit's included.

None.
