# Step 87d1: the measured hot spots (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the
workspace `../deduxit-wt-step-84-dc9a99`, from the planning session's
briefs `start-87-common.md` and `start-87d1.md`: `plan/87-refactor.md`'s
87d1, theme D2. Based on `smkuzpyz` (87e1 and the mutation scope,
stacked on 87c6 and fixup-35), on cores 4 to 6. Wave D started before
wave C ended, the planning session's decision (departs from the plan):
only 87c5 still runs, and this step leaves its files alone.

## Checklist

- [x] F-2c-07, `mist::merged` in place (`wtykvxuu`).
- [x] F-2c-02, `lltp::clauses` by bytes (`mtxwrutv`).
- [x] F-6b-01, the SVG layout's one buffer (`lrpzopkk`).
- [x] F-1b-02, the checker's reuse of a dropped sequent for a leaf
  (`uvsymyys`).
- [x] F-1b-02's net: Kani's harnesses (17 of 17), `web-core-wasm32`
  (724 tests), both target sets (every counter equal), the panel (one
  finding, fixed; no counterexample to the fixed version).
- [x] The ceilings lowered in a commit of their own (`wxqwqqsx`).
- [x] The gate, `breaks 0.8.0` (nothing new).
- [x] The reviewer (no finding in the code; the changelog line it asked
  for added).
- F-4a-06, F-4a-05 and F-5a-01 are left for 87d3 (departs from the
  plan, as the brief allows): 87c5 is not on `main` (`main@origin` is
  `lkylvptn`, "87e1 reviewed, 87d1 started").

## What landed, measured

Every count is the ratchet's (`deduxit-bench ratchet --only …`): the
instructions callgrind counts in the journey's measured part, and the
heap blocks, which are DHAT's blocks of that part. Both are from this
session's release build of the harness, before (at `smkuzpyz`) and
after each commit; the ceilings are the shell's build's and differ from
these by up to 2 %.

| finding | journey | instructions before | after | change | blocks before | after | change |
|---|---|--:|--:|--:|--:|--:|--:|
| F-2c-07 | `read-spec` | 18 419 943 | 18 180 199 | −1.3 % | 12 097 | 10 096 | −16.5 % |
| F-2c-02 | `read-lltp` | 28 669 309 | 27 152 771 | −5.3 % | 10 124 | 4 118 | −59.3 % |
| F-2c-02 | `read-tptp` | 34 275 138 | 32 682 505 | −4.6 % | 18 077 | 12 074 | −33.2 % |
| F-6b-01 | `render-svg` | 79 958 264 | 63 633 337 | −20.4 % | 49 819 | 4 687 | −90.6 % |
| F-1b-02 | `check-qbf-20-2` | 139 075 800 | 125 719 028 | −9.6 % | 127 362 | 57 238 | −55.1 % |
| F-1b-02 | `check-wide-m1-2048` | | 7 489 936 | −12.7 % (against its ceiling) | 8 216 | 31 | −99.6 % |

- **F-2c-07**: `Vec::dedup_by` merges the sorted counts in place; the
  later element is handed first, so the kept one takes the larger
  count, as the loop did.
- **F-2c-02**: the lookahead around `-` and `.` reads bytes, since every
  character it tests is ASCII and no byte of a longer character is an
  ASCII name character; `glued` takes the line and a byte offset, both
  ends next to ASCII name bytes and so character boundaries. The
  readers' tests pass, the glued `a-ob` refusal among them.
- **F-6b-01**: `lay` hands `emit` a borrowed piece in one buffer that
  the next piece reuses; `run` copies a piece only where it keeps it;
  the tree's first pass lays a conclusion out for its width alone, and
  its second pass reuses one `Run` for every conclusion (`run_into`);
  `Escaping` writes the text between escaped characters as it is,
  `replacement` being `escape`'s one table. Every export snapshot is
  unchanged (49 tests).
- **F-1b-02**: `Bag::add` drains the smaller table instead of consuming
  it; `join` and `over` hand the premise they drop to the pass, which
  keeps up to 16, emptied, where the linear zone's table has room for
  three members at most (the four slots `table_bytes` counts for a
  leaf), and `just` takes one before it allocates. The table is drained
  before its capacity is read (`emptied_for_a_leaf`, the panel's
  finding below). No count of the pass moves. The checker's tests pass,
  the oracle's differential test and the two bound tests among them, and
  `a_table_emptied_by_removals_is_no_leaf_s` pins the finding.

The checker's reuse moved the other journeys that check a proof too:
`derivation-chain-64` 1 078 to 919 blocks, `cut-elimination` 16 053 to
14 140, `session-ordinary` 76 986 to 75 963. `ratchet --lower` wrote
every count that fell, the base's small falls among them (`wxqwqqsx`).

## The net

- **Kani's harnesses** (`nix build .#kani-harnesses`, under the machine
  lock): 17 of 17 verified. The build was of the commit before the
  panel's fix, which changes `recycle` alone, which no harness calls;
  `Bag::add`'s drain is the code the harness `join` ran.
- **`web-core-wasm32`** (under the lock, after the fix): 724 passed, 2
  ignored, the new test among them.
- **Target sets** (cores 2 and 3, the planning session's leave):
  focused, 145 rows decided alike, 0 differ in any counter, no verdict
  differs; net, 64 rows, 0 differ. `xnqszyny`.
- **The gate** at `wxqwqqsx`: passed, the ratchet within the lowered
  ceilings (the cache model's misses moved on `search-chain-128`,
  `check-qbf-20-2` and `render-svg`, which it reports and does not fail
  on; the last two are this step's).
- **`breaks 0.8.0`**: the same three checks of cargo-semver-checks and
  the same 9 removed lines as at 87e1, all the base's; nothing new.
- **Clippy**, with and without every feature, `--deny warnings`: clean
  (the free list takes an `#[expect(clippy::vec_box)]`: its boxes are
  what a leaf reuses).

## The panel on F-1b-02

On the commit as first made (Opus at `high` for the argument and the
counterexamples, Sonnet at `high` for the integers):

- **The integers member and the argument member found one hole, each
  with an input**: `recycle` read `capacity()` before resetting the
  table. A removal leaves a tombstone that `capacity()` does not see,
  and `clear` resets nothing on a table with no member, so a `&`'s
  dropped premise whose zone copies had consumed read as capacity 0
  with 512 or 4 096 slots, and a leaf counted at four slots held them.
  The counts of the pass were unchanged, the memory they bound was not:
  144 MiB through a bound of about 7 MiB (the integers member), 376 MB
  against 16.9 MB before the change at an equal count (the argument
  member). Fixed in `uvsymyys`: `emptied_for_a_leaf` drains the table,
  which resets every slot, before it reads the capacity; the argument
  member's probe on the fix peaks at 18.4 MB, and its random search
  found no leaf above four slots in 4 000 trials, where the first form
  gave two.
- The integers member compared the two versions on 45 962 cases (the
  oracle's proofs in every mode, mutants, shared premises, Mix chains),
  each at the least memory and work bounds that do not refuse: every
  line identical, `held`, `done` and every error, also on wasm32.
- The argument member checked every reader of a zone after `add_linear`,
  the observers (all sort or sum), the `&` rule's arms and the error
  pass, and could not refute the fixed version. It also found the new
  test's doc comment run into `the_bound_is_exact`'s; fixed.

- **The counterexamples member found no difference** between the
  fixed commit and its parent on 1.2 million records. The inputs:
  - the oracle's 2 874 proofs in 8 modes and 206 640 mutants;
  - 1 820 eta-expanded identity proofs up to 15 746 nodes, and 10 500
    random terms with shared premises;
  - 1 407 larger generated and family proofs up to 65 533 nodes;
  - the first-order proofs.

  Each record holds the full `CheckError`, every progress the stop saw,
  the exact memory profile (every bound at which the result changes,
  found by bisection, so a refusal at any node), the work profile, the
  derivation and its size in three side modes, and cut elimination's
  steps. Its second reference, a direct test of a drained table against
  a fresh one, agreed on 157 304 cases and refused 24 928 large tables.

## The reviewer's findings

`fresh-reviewer` on Opus at `high`, over `smkuzpyz..wxqwqqsx`. No
finding in the code: it ran the old and new forms side by side on 2
million count lists (`mist::merged`), 9.4 million strings with
multi-byte characters around `-`, `.` and `-o` (`lltp::clauses` and
`glued`), and 2 million strings of every escaped and control character
(`Escaping`), all identical; checked that `run_into` overwrites every
field, that `recycle` resets every field and a reused table has the
bucket count and order of a fresh one, that the new test fails on the
first form, and that the ceilings commit raises nothing (25 rows). It
asked for the changelog's line, which the step had not written; added
under Changed.

## What the next steps must know

- F-4a-06, F-4a-05 and F-5a-01 (ordinary logic and lambda-terms) are
  87d3's: 87c5 had not landed when this step ended.
- A table reused across states must be drained before its capacity is
  read: `capacity()` does not see tombstones (`core-proofs.md`).
- `check-qbf-20-2` still makes 57 238 blocks: the rest are the clones of
  shared premises and the leaves that find no spare box. A larger spare
  list or a reused table past four slots would change what the pass
  counts, which this step kept.
