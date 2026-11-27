# Step 87r4: the Rocq library's text, and two candidates of speed

Opus 5.5 at `high`, in `../rocq-deduxit-wt-87r3` on 87r3's head
(`looyoyxt`); theme H4 of the audit (F-9-08, F-9-10, F-9-18), with each
finding's verdict. One commit in rocq-deduxit, `lswpxwzm` "Name the
suites in the page's order, and record two speed measurements" (commit
`207d2c5f`); the rules file records both measurements, so a later
session finds them.

## Outcome

- **The texts in the suites' order (F-9-08).** The opam description,
  README, `CITATION.cff`'s abstract and keywords, and the Nix package's
  description name ordinary logic first, then linear logic, then modal
  logic, and the opam description names modal logic and the suites
  deduxit has. README moves its sections to that order (the trust list,
  "Prove with the library", "What it covers", the interface table, the
  repository list) and says that deduxit writes certificates for
  ordinary and modal logic too, beside the linear example it shows. The
  library's `.claude/rules/writing.md` gains the rule it lacked (the
  verdict: that is why the texts drifted): no suite is the main one, in
  the page's order, here by namespace.
- **`&&` in the structural equalities (F-9-10): measured, no change.**
  A variant of the library with every structural equality's `&&`
  written as `if … then … else false` (the formulas of the four
  calculi, the terms, the modal items; built in the scratchpad only)
  against the library as it is, `rocq compile` on core 12, five runs
  each, alternating, under the machine lock, medians:

  | input | as it is | short-circuit |
  |---|---:|---:|
  | deduxit's 71 snapshot certificates, in all | 18.75 s | 18.81 s |
  | `tests/FromCommand.v` | 0.687 s | 0.691 s |
  | synthetic: `take_all` over 200 formulas `aᵢ ⊗ B`, `B` of 200 nodes | 4.65 s | 0.49 s |

  The cost shows only where many formulas share a head and a large
  second operand and differ in the first, which no certificate deduxit
  writes for its tests has; the certificates take the same time. The
  change would rewrite the trusted `Formulas.v`'s `formula_eqb` and four
  reflection proofs for no measured gain, so the library keeps `&&`; the
  synthetic case is the evidence if a certificate of that shape ever
  appears.
- **A `let` for each `⊗` node's formula (F-9-18): measured, the time
  shows; the exporter's change is left to the planning session
  (departs from the plan).** The
  certificate of `!A ⊢ A ⊗ … ⊗ A` as deduxit writes it (checked byte
  for byte against `deduxit prove --kernel deduxit` at n = 4 and 64)
  and the same with the formulas bound by `let` inside the term handed
  to the theorem (the statement written out, as the verdict asks), on
  core 12, five runs each, medians:

  | n | written out | with `let` | file size |
  |---:|---:|---:|---|
  | 64 | 0.639 s | 0.427 s | 38 KB / 7 KB |
  | 128 | 1.471 s | 0.577 s | 146 KB / 13 KB |
  | 256 | 4.763 s | 1.261 s | 570 KB / 27 KB |

  The written-out form grows quadratically in size; the `let` form
  linearly in size, and its time grows more slowly (it is 3.8 times
  faster at n = 256, 1.5 times at 64), though not linearly either. The
  change is the exporter's (deduxit's `core/src/export/rocq/deduxit.rs`)
  and changes every certificate's text with a `⊗` under a rule, which
  87c3's rule ("the snapshots byte for byte; no certificate's text
  changes") forbids in that step, and it changes what deduxit writes,
  which the common brief's "behaviour is kept" leaves to the planning
  session; so it waits for that session's decision: a step of its own in
  part 2, or none (the gain is for formulas nested dozens deep).

## The net

- The library's `nix flake check` at `207d2c5f` (README's five Rocq
  examples compiled in their new places): passes.
- deduxit's `rocq-deduxit` and `rocq-deduxit-library` against it:
  pass.
- The measurements' inputs and timings are under the session's
  scratchpad, `speed/` (`gen.py`, `Same.v`, `time.py`, `times.txt`).

## The reviewer

`fresh-reviewer` on Fable, once; two findings and a question, all
answered:
1. README's interface table said the `Weights` row uses "the two rows
   of linear logic above it", which after the reorder are the
   first-order rows; it says "the first two rows of linear logic" now.
2. This report said the `let` form's time grows linearly; only its
   size does (0.43, 0.58, 1.26 s). Corrected.
3. The F-9-18 decision is marked "(departs from the plan)", with the
   brief's rule that leaves a change of deduxit's output to the
   planning session.
It checked the moved README blocks byte for byte, every file of the
trust list, the clause of the writing rule, and every number of the
report against `times.txt` and the files.

## For the planning session

- F-9-18: whether deduxit's exporter binds a `⊗` node's formulas with
  `let` (a change of every such certificate's text; the measurements
  above). Not started.
- The README's `console` example still runs `deduxit prove --mix`
  without a logic, which D1 changes in part 2.
