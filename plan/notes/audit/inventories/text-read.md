# The pattern matches, read

| pattern | matches | of which break the rule |
|---|---:|---:|
| says more than is or has | 26 | 7 |
| inflated word | 1 | 0 |
| negative parallelism | 0 | 0 |
| rather than | 17 | 0 |
| trailing -ing commentary | 6 | 0 |
| filler to the reader | 0 | 0 |
| vague attribution | 0 | 0 |
| narration of how it was made | 26 | 0 |
| first person | 2 | 0 |
| curly quote | 0 | 0 |
| emoji or dingbat | 9 | 0 |

## Break the rule

- `core/src/modal/search.rs:1467`, says more than is or has: "a part that stands as its entry"; "is its entry".
- `core/src/modal/search.rs:1505`, same: "A kept part stands as its entry"; "is its entry".
- `core/src/ordinary/decide.rs:566`, same: "stands as [`Evidence::Image`]"; "is".
- `core/src/ordinary/rocq.rs:90`, same: "so that it stands as an argument"; "so that it is an argument".
- `core/src/ordinary/normal.rs:2448`, same: "the clauses stand as hypotheses"; "are hypotheses".
- `core/src/rules/natural.rs:964`, same: "which stand as assumptions"; "which are assumptions".
- `core/src/ordinary/translate.rs:537`, same: "a hypothesis stands as its dual"; "is its dual".

## False matches

- "refers to" / "refer to" for what a pointer, index or name denotes (arena indices, atoms, subterms): 18 (first_order.rs, export/mod.rs, instances.rs, arena.rs x3, focus/mod.rs, sequents/mod.rs x2, term.rs, modal/mod.rs, derivation.rs, ordinary/mod.rs, translate.rs x2, reference.md).
- "stand(s) as they are / as written", "wherever it stands" (stay unchanged, or position): 4 (negative.rs:342, form.rs:362 and 371, translate.rs:415).
- "underscores" as the character: 1 (export/rocq/mod.rs:143).
- "rather than" stating a real choice or contrast the reader needs (bar or conclusion, goal or whole proof, refused or mis-numbered, written net or derivation, JSON or sequent): 17.
- "making it where ..." states what the function does when the item is missing, not commentary: 6.
- "no longer" describing run-time state (goal no longer open, memory no longer held, node no longer read, item can no longer help close): 21 of the 26 narration matches.
- "this session" meaning the interactive proving session of the program: 5 (invert.rs x2, interactive/mod.rs, calls.rs x2).
- CHANGELOG history ("no longer"): 3 (CHANGELOG.md:433, 585, 2384).
- README "identify this project, and may not be used": a licence term, not process: 1 (README.md:198).
- First person: "our literal" (deduxit's, against batsat's) 1; "provers we found" is a method statement of a survey 1.
- Dingbats that are check and cross marks in a table's cells and legend: 9 (benchmarks.md).

Number of true violations: 7
