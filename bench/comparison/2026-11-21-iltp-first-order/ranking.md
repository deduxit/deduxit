# Tools compared

1880 problems, 5 tools, limits 2 s and 4294967296 bytes.

## Solved within the limit

| tool | solved | proved | unprovable | wrong | late | time | memory | gave up | error | outside | against the header |
|---|---|---|---|---|---|---|---|---|---|---|---|
| deduxit | 1251 | 1065 | 186 | 0 | 1 | 474 | 0 | 146 | 8 | 0 | 0 |
| nanocop | 713 | 629 | 84 | 0 | 0 | 223 | 0 | 0 | 8 | 936 | 0 |
| leancop | 666 | 640 | 26 | 0 | 0 | 270 | 0 | 0 | 8 | 936 | 0 |
| nanocopi | 568 | 482 | 86 | 0 | 0 | 368 | 0 | 0 | 8 | 936 | 0 |
| ileancop | 559 | 485 | 74 | 0 | 0 | 377 | 0 | 0 | 8 | 936 | 0 |

`wrong`: a refutation of a problem that a tool proved with a checked proof, a refutation of a generated problem that is provable by construction, or an unchecked proof of one that is unprovable by construction. A wrong verdict never counts as solved. `late`: decided after the limit. `gave up`: the tool's own bound, or no answer. `outside`: not run, since the problem is outside the tool's fragment or mode. `against the header`: a verdict that contradicts an LLTP file's header. It counts as solved, since a few of the library's headers are wrong.

## By group: solved / in the fragment

| group | problems | leancop | nanocop | nanocopi | ileancop | deduxit |
|---|---|---|---|---|---|---|
| ILTP (ordinary logic) | 1880 | 666 / 944 | 713 / 944 | 568 / 944 | 559 / 944 | 1251 / 1880 |

## deduxit against each tool, on the problems both cover

| tool | both cover | both solve | deduxit alone | the tool alone | neither |
|---|---|---|---|---|---|
| leancop | 944 | 634 | 132 | 32 | 146 |
| nanocop | 944 | 634 | 132 | 79 | 99 |
| nanocopi | 944 | 384 | 101 | 184 | 275 |
| ileancop | 944 | 378 | 107 | 181 | 278 |

Solved by leancop and not by deduxit (32):

- Problems/GEJ/GEJ008+2.p (classical)
- Problems/GEJ/GEJ010+1.p (classical)
- Problems/GEJ/GEJ011+1.p (classical)
- Problems/GEJ/GEJ012+1.p (classical)
- Problems/GEJ/GEJ018+2.p (classical)
- Problems/GEJ/GEJ019+2.p (classical)
- Problems/GEJ/GEJ020+2.p (classical)
- Problems/GEJ/GEJ022+1.p (classical)
- Problems/GEJ/GEJ026+2.p (classical)
- Problems/GEJ/GEJ027+1.p (classical)
- Problems/GEJ/GEJ030+2.p (classical)
- Problems/GEJ/GEJ044+1.p (classical)
- Problems/GEJ/GEJ048+1.p (classical)
- Problems/GEJ/GEJ048+2.p (classical)
- Problems/KRS/KRS018+1.p (classical)
- Problems/KRS/KRS146+1.p (classical)
- Problems/KRS/KRS151+1.p (classical)
- Problems/KRS/KRS153+1.p (classical)
- Problems/KRS/KRS159+1.p (classical)
- Problems/KRS/KRS161+1.p (classical)
- Problems/KRS/KRS173+1.p (classical)
- Problems/PLA/PLA029+2.p (classical)
- Problems/PUZ/PUZ005+1.p (classical)
- Problems/SWV/SWV010+1.p (classical)
- Problems/SYJ/SYJ036+1.p (classical)
- Problems/SYJ/SYJ037+1.p (classical)
- Problems/SYN/SYN320+1.p (classical)
- Problems/SYN/SYN328+1.p (classical)
- Problems/SYN/SYN329+1.p (classical)
- Problems/SYN/SYN334+1.p (classical)
- Problems/SYN/SYN337+1.p (classical)
- Problems/SYN/SYN342+1.p (classical)

Solved by nanocop and not by deduxit (79):

- Problems/GEJ/GEJ008+2.p (classical)
- Problems/GEJ/GEJ010+1.p (classical)
- Problems/GEJ/GEJ011+1.p (classical)
- Problems/GEJ/GEJ012+1.p (classical)
- Problems/GEJ/GEJ018+2.p (classical)
- Problems/GEJ/GEJ019+2.p (classical)
- Problems/GEJ/GEJ020+2.p (classical)
- Problems/GEJ/GEJ022+1.p (classical)
- Problems/GEJ/GEJ026+2.p (classical)
- Problems/GEJ/GEJ027+1.p (classical)
- Problems/GEJ/GEJ030+2.p (classical)
- Problems/GEJ/GEJ044+1.p (classical)
- Problems/GEJ/GEJ048+1.p (classical)
- Problems/GEJ/GEJ048+2.p (classical)
- Problems/KRS/KRS018+1.p (classical)
- Problems/KRS/KRS019+1.p (classical)
- Problems/KRS/KRS020+1.p (classical)
- Problems/KRS/KRS116+1.p (classical)
- Problems/KRS/KRS146+1.p (classical)
- Problems/KRS/KRS153+1.p (classical)
- Problems/KRS/KRS173+1.p (classical)
- Problems/NLP/NLP043+1.p (classical)
- Problems/NLP/NLP044+1.p (classical)
- Problems/NLP/NLP045+1.p (classical)
- Problems/NLP/NLP046+1.p (classical)
- Problems/NLP/NLP047+1.p (classical)
- Problems/NLP/NLP048+1.p (classical)
- Problems/NLP/NLP059+1.p (classical)
- Problems/NLP/NLP062+1.p (classical)
- Problems/NLP/NLP066+1.p (classical)
- Problems/NLP/NLP067+1.p (classical)
- Problems/NLP/NLP068+1.p (classical)
- Problems/NLP/NLP097+1.p (classical)
- Problems/NLP/NLP098+1.p (classical)
- Problems/NLP/NLP099+1.p (classical)
- Problems/NLP/NLP100+1.p (classical)
- Problems/NLP/NLP101+1.p (classical)
- Problems/NLP/NLP102+1.p (classical)
- Problems/NLP/NLP131+1.p (classical)
- Problems/NLP/NLP132+1.p (classical)
- and 39 more

Solved by nanocopi and not by deduxit (184):

- Problems/ALG/ALG211+1.p (intuitionistic)
- Problems/GEJ/GEJ001+2.p (intuitionistic)
- Problems/GEJ/GEJ003+2.p (intuitionistic)
- Problems/GEJ/GEJ004+1.p (intuitionistic)
- Problems/GEJ/GEJ004+2.p (intuitionistic)
- Problems/GEJ/GEJ005+2.p (intuitionistic)
- Problems/GEJ/GEJ006+1.p (intuitionistic)
- Problems/GEJ/GEJ006+2.p (intuitionistic)
- Problems/GEJ/GEJ010+2.p (intuitionistic)
- Problems/GEJ/GEJ011+2.p (intuitionistic)
- Problems/GEJ/GEJ013+1.p (intuitionistic)
- Problems/GEJ/GEJ013+2.p (intuitionistic)
- Problems/GEJ/GEJ015+2.p (intuitionistic)
- Problems/GEJ/GEJ017+2.p (intuitionistic)
- Problems/GEJ/GEJ018+1.p (intuitionistic)
- Problems/GEJ/GEJ019+1.p (intuitionistic)
- Problems/GEJ/GEJ020+1.p (intuitionistic)
- Problems/GEJ/GEJ020+2.p (intuitionistic)
- Problems/GEJ/GEJ021+1.p (intuitionistic)
- Problems/GEJ/GEJ021+2.p (intuitionistic)
- Problems/GEJ/GEJ024+2.p (intuitionistic)
- Problems/GEJ/GEJ025+2.p (intuitionistic)
- Problems/GEJ/GEJ026+1.p (intuitionistic)
- Problems/GEJ/GEJ028+1.p (intuitionistic)
- Problems/GEJ/GEJ028+2.p (intuitionistic)
- Problems/GEJ/GEJ029+1.p (intuitionistic)
- Problems/GEJ/GEJ029+2.p (intuitionistic)
- Problems/GEJ/GEJ030+1.p (intuitionistic)
- Problems/GEJ/GEJ031+2.p (intuitionistic)
- Problems/GEJ/GEJ032+1.p (intuitionistic)
- Problems/GEJ/GEJ032+2.p (intuitionistic)
- Problems/GEJ/GEJ033+2.p (intuitionistic)
- Problems/GEJ/GEJ034+2.p (intuitionistic)
- Problems/GEJ/GEJ035+1.p (intuitionistic)
- Problems/GEJ/GEJ035+2.p (intuitionistic)
- Problems/GEJ/GEJ036+1.p (intuitionistic)
- Problems/GEJ/GEJ041+2.p (intuitionistic)
- Problems/GEJ/GEJ043+2.p (intuitionistic)
- Problems/GEJ/GEJ045+1.p (intuitionistic)
- Problems/GEJ/GEJ045+2.p (intuitionistic)
- and 144 more

Solved by ileancop and not by deduxit (181):

- Problems/ALG/ALG211+1.p (intuitionistic)
- Problems/GEJ/GEJ001+2.p (intuitionistic)
- Problems/GEJ/GEJ003+2.p (intuitionistic)
- Problems/GEJ/GEJ004+1.p (intuitionistic)
- Problems/GEJ/GEJ004+2.p (intuitionistic)
- Problems/GEJ/GEJ005+2.p (intuitionistic)
- Problems/GEJ/GEJ006+1.p (intuitionistic)
- Problems/GEJ/GEJ006+2.p (intuitionistic)
- Problems/GEJ/GEJ008+1.p (intuitionistic)
- Problems/GEJ/GEJ008+2.p (intuitionistic)
- Problems/GEJ/GEJ010+2.p (intuitionistic)
- Problems/GEJ/GEJ011+2.p (intuitionistic)
- Problems/GEJ/GEJ012+2.p (intuitionistic)
- Problems/GEJ/GEJ013+1.p (intuitionistic)
- Problems/GEJ/GEJ013+2.p (intuitionistic)
- Problems/GEJ/GEJ015+2.p (intuitionistic)
- Problems/GEJ/GEJ017+2.p (intuitionistic)
- Problems/GEJ/GEJ019+1.p (intuitionistic)
- Problems/GEJ/GEJ019+2.p (intuitionistic)
- Problems/GEJ/GEJ020+1.p (intuitionistic)
- Problems/GEJ/GEJ020+2.p (intuitionistic)
- Problems/GEJ/GEJ021+1.p (intuitionistic)
- Problems/GEJ/GEJ021+2.p (intuitionistic)
- Problems/GEJ/GEJ022+1.p (intuitionistic)
- Problems/GEJ/GEJ022+2.p (intuitionistic)
- Problems/GEJ/GEJ026+2.p (intuitionistic)
- Problems/GEJ/GEJ027+2.p (intuitionistic)
- Problems/GEJ/GEJ029+1.p (intuitionistic)
- Problems/GEJ/GEJ030+2.p (intuitionistic)
- Problems/GEJ/GEJ031+2.p (intuitionistic)
- Problems/GEJ/GEJ032+1.p (intuitionistic)
- Problems/GEJ/GEJ032+2.p (intuitionistic)
- Problems/GEJ/GEJ033+2.p (intuitionistic)
- Problems/GEJ/GEJ035+1.p (intuitionistic)
- Problems/GEJ/GEJ035+2.p (intuitionistic)
- Problems/GEJ/GEJ036+1.p (intuitionistic)
- Problems/GEJ/GEJ041+2.p (intuitionistic)
- Problems/GEJ/GEJ043+1.p (intuitionistic)
- Problems/GEJ/GEJ043+2.p (intuitionistic)
- Problems/GEJ/GEJ045+1.p (intuitionistic)
- and 141 more

## Translation and memory

| tool | translated | translation, median | translation, most | peak memory of solved runs, median | most |
|---|---|---|---|---|---|
| leancop | 936 | 0 µs | 0 µs | 8.8 MiB | 23.3 MiB |
| nanocop | 936 | 0 µs | 0 µs | 8.8 MiB | 24.3 MiB |
| nanocopi | 936 | 0 µs | 0 µs | 9.1 MiB | 143.8 MiB |
| ileancop | 936 | 0 µs | 0 µs | 8.8 MiB | 377.9 MiB |
| deduxit | 1872 | 0 µs | 0 µs | 1.8 MiB | 381.0 MiB |

deduxit reads LLTP and `.spec` files itself and the rest as JSON, which the harness writes; the translation time is the harness's, outside the measured run.

## Answered after the limit

| problem | tool | verdict | time |
|---|---|---|---|
| Problems/GEJ/GEJ020+1.p (intuitionistic) | deduxit | proved | 2.00 s |

## Contradictions

A problem on which one tool proves what another refutes, or a tool decides against the known verdict. Each verdict says whether it was checked. deduxit's proofs and refutations are checked, but for an exhausted search, which has no certificate; Sympli checks its own proofs. No other tool's verdict is checked.

None.
