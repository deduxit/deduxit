# Tools compared

866 problems, 4 tools, limits 2 s and 4294967296 bytes.

## Solved within the limit

| tool | solved | proved | unprovable | wrong | late | time | memory | gave up | error | outside | against the header |
|---|---|---|---|---|---|---|---|---|---|---|---|
| leancop | 135 | 134 | 1 | 0 | 0 | 298 | 0 | 0 | 0 | 433 | 0 |
| nanocop | 116 | 115 | 1 | 0 | 0 | 317 | 0 | 0 | 0 | 433 | 0 |
| ileancop | 91 | 90 | 1 | 0 | 0 | 341 | 0 | 0 | 1 | 433 | 0 |
| nanocopi | 88 | 87 | 1 | 0 | 0 | 345 | 0 | 0 | 0 | 433 | 0 |

`wrong`: a refutation of a problem that a tool proved with a checked proof, a refutation of a generated problem that is provable by construction, or an unchecked proof of one that is unprovable by construction. A wrong verdict never counts as solved. `late`: decided after the limit. `gave up`: the tool's own bound, or no answer. `outside`: not run, since the problem is outside the tool's fragment or mode. `against the header`: a verdict that contradicts an LLTP file's header. It counts as solved, since a few of the library's headers are wrong.

## By group: solved / in the fragment

| group | problems | leancop | nanocop | nanocopi | ileancop |
|---|---|---|---|---|---|
| ILTP (ordinary logic) | 866 | 135 / 433 | 116 / 433 | 88 / 433 | 91 / 433 |

## Translation and memory

| tool | translated | translation, median | translation, most | peak memory of solved runs, median | most |
|---|---|---|---|---|---|
| leancop | 433 | 0 µs | 0 µs | 9.3 MiB | 84.3 MiB |
| nanocop | 433 | 0 µs | 0 µs | 9.9 MiB | 15.8 MiB |
| nanocopi | 433 | 0 µs | 0 µs | 11.3 MiB | 39.1 MiB |
| ileancop | 433 | 0 µs | 0 µs | 9.9 MiB | 13.9 MiB |

deduxit reads LLTP and `.spec` files itself and the rest as JSON, which the harness writes; the translation time is the harness's, outside the measured run.

## Answered after the limit

None.

## Contradictions

A problem on which one tool proves what another refutes, or a tool decides against the known verdict. Each verdict says whether it was checked. deduxit's proofs and refutations are checked, but for an exhausted search, which has no certificate; Sympli checks its own proofs. No other tool's verdict is checked.

None.
