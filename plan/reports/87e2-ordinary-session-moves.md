# Step 87e2: the ordinary session along the linear session's seams, and ordinary/mod.rs (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-fixup-30-3e23c4`, from `plan/87-refactor.md` (87e2, wave
E: moves only, public paths kept) and the planning session's briefs
`start-87-common.md` and `start-87e2.md`, on cores 13 to 15 (and 2 to 3
for the connection pin, with the planning session's word). The base is
`yysttsrx` (fixup-33 and fixup-34). F-4e-05 and F-arch-20. No change of
behaviour; nothing public moves. No panel (the plan marks none); the
reviewer on Opus at `high`.

## What landed

Eleven moves, one per commit, each building in every feature set the
module's code depends on:

- `onqnzmuz` Move the ordinary session into a directory of its own:
  `ordinary/interactive.rs` (5 907 lines) is `ordinary/interactive/mod.rs`,
  a rename.
- `snmunywr` the tests into `interactive/tests.rs` (1 946 lines), moved
  without re-indenting by hand (rustfmt re-indents code and leaves a
  literal's inside alone; the tokens are the same, white space aside).
- `trytqtmt` the reader behind `serialize` (`Parts`, `parts`,
  `from_parts`, `Replay`, `reached`, `Tree`) into `replay.rs`, declared
  under `#[cfg(feature = "serialize")]`.
- `wqurrrnl` the refusals (`StepError`, `BindingFault`, their text) into
  `error.rs`, re-exported.
- `tzumronn` the open witnesses, unification and the quantifier steps
  (`Made`, `Witnesses`, `Unbound`, `unify_axiom`, `arena_terms`,
  `unfolded`, `implications_from`, `bound_met`, `Reading`, and the twelve
  methods from `unifying_axiom` to `applied`) into `witness.rs`.
- `uuxutrlq` the steps and undo (`shape`, `apply`, `expand`,
  `run_within`, `checked`, `split`, `record`, `record_made`, `undo`,
  `reshow`, `subtree`, and `premise`, `contracted`, `axiom`) into
  `expand.rs`.
- `vrsqywus` the search's graft (`Decided`, `Carry`, `with_derivation`,
  `takes_open_witness`, `fit_root`, and `close`, `hint`, `decide`,
  `ids_of`, `graft`, `graft_first_order`, `carried`, `first_steps`,
  `step_at`) into `close.rs`.
- `nnynwkpq` the views (`derivation`, `derivation_within`, `finishing`,
  `derivation_ids`, `postorder`, `finished`, `estimate`) into
  `derivation.rs`.
- `zonynrwt` `ordinary/invert.rs` into `interactive/invert.rs`, a rename
  with its paths one level deeper.
- `zvzwywzp` the decision's vocabulary (`Logic`, `Translation`, `Target`,
  `Method`, `Options`, the default's class test `grounds` and its bounds)
  into `ordinary/options.rs`, re-exported.
- `notntsxx` the formula arena (`NodeId`, `Node`, `Formulas`, `Formula`,
  `Sequent`, `Symbols`, `OfTables`, `REFUSED`, `closed`, `bound` and the
  arena's helpers) into `ordinary/formulas.rs`, re-exported.

The session's files are now `mod.rs` 780 lines (the vocabulary, the
state, its queries, and the helpers several parts use: `open`,
`formula_at`, `needs_of`, `shown`, `terms_full`), `error.rs` 340,
`witness.rs` 872, `expand.rs` 641, `close.rs` 618, `derivation.rs` 222,
`invert.rs` 378, `replay.rs` 605 and `tests.rs` 1 946; `ordinary/mod.rs`
803 (the modules, the re-exports, the tests), `options.rs` 532 and
`formulas.rs` 1 035.

Each move keeps the code as it was. What a sibling now reads is
`pub(super)` (in the session: the witnesses' state, `Made`, the steps
other parts call; in `ordinary`: `NodeId`'s field, `Formulas`' and
`Sequent`'s fields and six of `Formulas`' methods, which every module of
`ordinary` read while they were private to `ordinary/mod.rs`); a
relative path one level deeper is written from the crate
(`crate::ordinary::…`): the moved code's inline `super::transfer`,
`super::herbrand` and `super::derivation::widths`, which in a child of
the session would have named the wrong module (`super::derivation` in
`interactive/derivation.rs` names that file); the doc links that
resolved through the old file's imports have full paths, each fixed in
the commit where it broke; the tests import by name what they took from
the old file's imports through `use super::*`, each behind the feature
its tests need. The imports were written from the parent's and trimmed
by `cargo fix` (unused imports only). The rules: `core-ordinary.md` (the
layout and the heads of two sections) and `core.md` (the module list).
No changelog line: nothing a user sees changed.

## Departures

- **Helpers several parts use stay in `mod.rs`** (`open`, `formula_at`,
  `needs_of`, `shown`, `terms_full`): the finding puts `premise`,
  `contracted` and `axiom` with expansion, which they are, and names no
  home for these.
- **`invert.rs` keeps `super::StepError`** in its signatures: in the
  session's directory it names the session's re-export of the same item.

## The net

- **The gate** at `notntsxx`: passed (clippy on the workspace and on the
  core crate without `parallel`, the tests, both `cargo hack` runs, the
  ratchet).
- **Every move built** with `cargo check` on the library and its tests
  in the feature sets the moved code depends on (none, `parse`, `sat`,
  `serialize`, `interactive`, `rocq` and their pairs with `interactive`
  or `parse`); two intermediate commits warned in one pair until the
  reviewer's fix (below), and every commit is clean now.
- **The rustdoc**, public and private, with warnings denied: clean at the
  head.
- **The ratchet** against the base: the gate's build of the head counts
  every ordinary journey and `session-ordinary` as a build of the base
  (`yysttsrx`, in a workspace of its own) does, to the instruction:
  `ordinary-pigeons` 31 854 796, `ordinary-connection-classical`
  9 187 453, `session-ordinary` 38 982 294, the others as at fixup-34's
  head. A second build of the head in another target directory counted
  those three 0.05 % below and 0.06 % and 0.04 % above, the spread
  between two builds of one tree (`bench.md`); the blocks are equal
  everywhere.
- **87a's connection pin** (`bench/connection-targets.sh 87e2`, cores 2
  and 3, the arena's methods being what both provers call): no verdict and
  no counter differs on any of the 1 312 problems against
  `connection-87a.csv`.
- **`breaks 0.8.0`**: nothing new against fixup-34's run. The public
  API's diff differs only in its presentation: `Interactive::close` and
  `hint`, defined in `close.rs`'s `impl` block, are listed under an
  `impl Interactive` header of their own, with the same signatures (and
  an `impl` header of `resolution::first_order::Refutation` comes from
  the restacked base).

## Stacked on the current top

The planning session stacked the chain on `uxotzoyo` (main with 87d2,
87e5, fixup-36, fixup-37, fixup-38 and the logs). The first eight moves
rebased without conflict; `ordinary/mod.rs` conflicted from `zonynrwt`
on, and `core.md` in `ynlomlqp`. Each was resolved in place, keeping
upstream's modules and paths in the new layout:

- `zonynrwt`: the `mod` list keeps 87e5's `countermodel/` and `models/`
  and fixup-37's `rocq/` (the old `kripke_search`, `kripke_structure`
  lines were already gone upstream), less `mod invert;`.
- `zvzwywzp`: the re-exports keep 87e5's `pub use countermodel::{…}` and
  add `pub use options::{…}` (the old `kripke_structure` and `model`
  re-exports were gone upstream). `notntsxx` resolved with it.
- `ynlomlqp`: `core.md`'s module list is 87e5's, with the session's
  entry as this step wrote it.

The resolutions changed only `mod` lines, re-exports and a rules line,
so the connection pin was not run again. Every commit builds on the new
base (the library and its tests with every feature, and the tests with
`parse` and `interactive` alone, no warning); both rustdocs are clean at
the head; the gate passed at `poutolpy` (the ratchet's ordinary
journeys within their ceilings, `ordinary-pigeons` and `session-ordinary`
0.34 % and 0.25 % below them with upstream's changes beneath).

## The reviewer

fresh-reviewer (Opus 5.5, high) checked each move with a diff that
detects moves (removed against added lines as multisets, then each new
file tiled by the longest runs of its parent's tokens): every
difference that is no moved line is an import, a module declaration or
re-export, a `pub(super)` with a user outside its file, a `super::` path
rewritten to `crate::ordinary::` that names the same item, a doc link
given its target with its text unchanged, a `cfg` moved onto a module or
an import, or a test import. No public path changed (the re-exports at
base and head differ only by the two added for `options` and
`formulas`); the 227 tests of `ordinary/` are the same names at base and
head. It rebuilt each of the eleven trees in several feature sets and
found two things, both fixed:

- `tzumronn` and `uuxutrlq` built the session's tests with an unused
  import (`crate::WitnessFault`, which only tests behind `serialize`
  use) in the pair {`parse`, `interactive`}, which the flake's
  `features-tests` denies; the import is behind `serialize` from
  `tzumronn` on now, and both commits build clean there.
- `core-ordinary.md` still put `Target` in `translate.rs`, a line stale
  since before this step and now naming two homes: `Target` and
  `Translation::target` are described under `options.rs` alone.

## What the next steps must know

- The session's parts are along the linear session's seams; a new step
  goes in `expand.rs`, a new quantifier rule's witness handling in
  `witness.rs`, a new view in `derivation.rs`.
- fixup-37's countermodel directory changes `ordinary/mod.rs`'s `mod`
  lines; this step left the countermodel modules' lines as they were, so
  the conflict is in the `mod` list alone.
