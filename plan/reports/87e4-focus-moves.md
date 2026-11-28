# Step 87e4: the focused engine's first-order zone, its counts and its bias (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-87c1-64a1a5`, from `plan/87-refactor.md` (87e4,
wave E: moves only, public paths kept) and the planning session's briefs
`start-87-common.md` and `start-87e4.md`, on cores 7 to 9. Begun on
`mntkktmm` (87c6) and rebased, as the planning session asked, onto
`lkylvptn` (main with 87c6, fixup-35 and 87e1), so that the three lines
of `linear/search/mod.rs` the move needs are made on 87e1's split file.
F-2b-06, F-2b-13 and F-2b-03. No change of behaviour; nothing public
moves.

## What landed

- **`nrzqsont` Split the first-order zone into its variables, keys, Zone
  impl and closing** (F-2b-06): `focus/framed.rs` (2 123 lines) is
  `focus/framed/mod.rs` (the zone's types, variables, unification and
  answers, and `Members` and `Set`), `keys.rs` (the keys of first-order
  stable sequents, with the comment that explains them as its module
  doc), `zone.rs` (the `impl Zone`) and `closing.rs` (the closing of a
  proof). The code moves as it was; what a sibling now reads is
  `pub(super)` (`KeyScratch` and its `bytes`, `write_keys`, `apply`,
  `Closing` with its fields, `term` and `member`).
- **`mutlozkn` Write the counts' set-up as one function per pass**
  (F-2b-13): `new_polled` keeps the checks, the fixed charge and the
  assembly, and calls `exponential_atoms`, `ranks`,
  `unrestricted_literals` and `write_rows` (with `Seen`, what the last
  pass reads of the others, and `Written`, what it gives back). One
  `stopped` closure and one account go through them, so the polls and
  the charges fall where they did.
- **`xnmuvmkp` Move what a factor forces to the counts that compute it**:
  `Forced` from `split.rs` to `counts.rs`, which computes it
  (`Counts::forced`), so that the counts need nothing of the engine.
- **`owpyrmwk` Give the counts the prunes they read, apart from the
  engine's switches** (F-2b-03, the verdict's correction): `Prunes`
  (`mix`, `equation`, `intervals`) with `Prunes::new`, the three
  expressions `Switches::new` had; `Switches::new` takes them from it and
  keeps `exponentials`, `affine` and `stack`, the engine's own.
  `counts::refutation` and `split_passes` build `Prunes`.
- **`lrwmwymp` Move the counts and the atom bias out of the focused
  engine** (F-2b-03): `search/counts.rs` and `search/bias.rs`, beside the
  focused engine, the Horn engine, the inverse method, the front door and
  the interactive session that read them; their paths updated, and
  `bias.rs`'s link to `Bias` made plain (rustdoc called the explicit
  target redundant once the module sat beside it).
- **`otsvtvln` Pin how often the counts' set-up polls**: a nest of
  2 000 tensors, whose rows are quadratic, polls 30 times; the same test
  run on the base gave 30. The verdict of F-2b-13 asked for such a test,
  since the target set's counters do not see the polls.

The rules: `core-focus.md` (its `paths:` now take `search/counts.rs` and
`search/bias.rs`, its file list, the bias and counts bullets, `Prunes`),
`core.md` (the table and the module list), `core-search.md`,
`core-horn.md`, `core-derivations.md`, `core-forest.md` and
`claude-infra.md`, and the comments of `first_order.rs`, `keyed.rs` and
`focus/mod.rs` that named the moved files.

## The net

- **The gate** at the chain's head, on cores 7 to 9: Clippy on the
  workspace and on the core crate alone, warnings denied, clean; `cargo
  test --workspace`, all 39 suites passed; both `cargo hack` runs and the
  powerset of depth 2 with `--tests`, warnings denied, clean; the ratchet
  (`--check`) within its tolerance ("gate: passed"). The private docs
  (`cargo doc --document-private-items`, warnings denied, as the flake's
  `doc-private` builds them) clean.
- **The target sets** on cores 2 and 3 (given by the planning session):
  `bench/targets/87e4.csv` against `86.csv`, no verdict differs and none
  of the 145 problems decided alike differs in a counter, nothing lost or
  gained, the `checked` column the oracle's row for row; `net-87e4.csv`
  against `net-86.csv`, likewise for its 64.
- **The ratchet, head against base** (`lkylvptn`, counted the same way
  in a workspace of its own): the focused engine's journeys within
  0.26 %:

  | journey | base | head | change |
  |---|--:|--:|--:|
  | search-qbf-20-2 | 487 739 825 | 487 744 200 | +0.00 % |
  | search-chain-128 | 962 851 139 | 962 858 514 | +0.00 % |
  | search-partition-no-5 | 7 064 157 | 7 064 986 | +0.01 % |
  | search-wide-m2-256 | 260 349 305 | 260 344 968 | −0.00 % |
  | search-spec-chain | 22 767 267 | 22 826 369 | +0.26 % |
  | search-chain-64-intuitionistic | 122 292 418 | 122 296 296 | +0.00 % |
  | search-chain-fo-32 | 24 888 842 | 24 893 956 | +0.02 % |
  | search-inverse | 4 461 267 | 4 459 203 | −0.05 % |

  No journey's heap blocks changed, and nothing needed inlining. Five
  journeys that run no code of this chain moved by 0.4 to 1.2 %
  (`check-qbf-20-2` +0.51 %, `check-saturated` +0.40 %, `lambda-inhabit`
  +0.44 %, `ordinary-connection-intuitionistic` −0.39 %,
  `ordinary-models` −1.24 %, `rules-check` −0.71 %): the head's counts of
  those are exactly 87c6's head's, and the base, built in another
  directory, gives them differently both times, so the build, not the
  code, moves them.
- **`breaks 0.8.0`**: nothing of this step. cargo-semver-checks fails
  the base's three checks (`enum_struct_variant_field_missing`,
  `enum_variant_added`, `type_method_marked_deprecated`).
  cargo-public-api's diff names the twelve types 87e1 moved by their
  other public path, as at 87e1; with those names made one, 9 lines are
  removed, exactly the base's known breaks (the step errors' `reason`
  fields, `lambda::Term::goal`, `Typing::ty`, `Variable::ty`), and no
  line names a counts, bias or first-order zone item, all crate-private.

## The reviewer

`fresh-reviewer` (Opus 5.5 at `high`), with a diff of each old file
against its new parts: the code moves sound, no logic changed, nothing
public added, every commit building clean on its own (Clippy with all
features at each), 205 tests of `linear::search` passed. One finding:
`core-search.md` still named `Switches` and "the focused engine's
`Counts`" in the refutation's bullet the chain had edited; corrected in
`owpyrmwk`. Its open question, the test of the set-up's polls, is
`otsvtvln`.

## What the next steps must know

- The counts and the bias are `crate::linear::search::{counts, bias}`;
  their rules stay in `core-focus.md`, which now loads for them.
- `Prunes` is what the count tests read; the engine's `Switches` copies
  its three fields.
- `focus/framed/`'s parts reach each other's private fields as children
  of `framed`; an item a sibling calls is `pub(super)`.
