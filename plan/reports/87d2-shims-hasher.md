# Step 87d2: the stop shims, the hasher, the web module's first two mends (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the
workspace `../deduxit-wt-step-84-dc9a99`, from the planning session's
briefs `start-87-common.md` and `start-87d2.md`: `plan/87-refactor.md`'s
87d2, themes D3 and D4, P25's steps 1 and 2, F-8-03 and F-5a-07. Based
on 87d1's head (`tpwszxlz`), on cores 4 to 6, the target sets on 2 and 3
by the planning session's leave.

## Checklist

- [x] D3, the stop shims (one commit per area): the exports
  (`kusxqprp`), cut elimination (`lxqroowy`), the rule files' check and
  search and the workspace's runs (`swslqvrv`), the modal tableau
  (`ytzmrszk`), the interpolants and the normal form's check
  (`vslwqupo`), and `Pass::rule`'s observer/instancing pairs, F-1b-03
  (`kvwkslrq`).
- [x] F-8-03, one stop for every call of the bindings (`ysuxvyyo`).
- [x] D4, the crate's hasher everywhere and `core/clippy.toml`'s
  `disallowed-types` (`ollsrpxs`).
- [x] F-5a-07, P25 step 2, the lambda forms read field by field
  (`uxupvkpp`), with the planning session's leave for the tightening.
- [x] The target sets (`wykkvryo`), the web module's bytes, the bindings'
  tests under Node, the `budget` check.
- [x] The gate, `breaks 0.8.0` (nothing new).
- [x] The reviewer (two findings in F-5a-07, both fixed in its commit).
- The nets' shims (F-3-10) and `export/svg/mell.rs` wait for 87e6, which
  is not on `main` (`main@origin` is `zvkmksturpxo`, "87e4 reviewed, 87e3
  started"): left for 87d3, as the brief says.

## What landed

- **D3, the stop shims.** Each long body that was generic over the
  caller's stop now takes `&mut dyn FnMut(Progress) -> bool` behind a
  one-line public shim, the crate's rule:
  - the exports: `write_drawable`, the one match behind `latex::write`,
    `typst::write` and `svg::write` (F-6b-06), and `kripke::draw`, whose
    three callers' hand-written counting closures are now
    `limits::counting` inside it (F-6b-02);
  - cut elimination's `step` and `eliminate`, `eliminate`'s `each` too
    (F-1c-12);
  - the rule files' `check`, `check_open` and `check_node` (their one
    body `check_some`), the natural deduction session's `close` and
    `hint`, and the workspace's `compute` and `run` (F-5b-12);
  - `modal::decide_within`, the tableau and the derivation it emits
    (F-4d-04);
  - `interpolate`, `interpolant`, `checked` and `maehara`, and
    `NormalForm::check` (F-4b-02).
- **F-1b-03, `Pass::rule` compiled twelve times.** The size estimate and
  the derivation's record each choose an instancing (`Ground` or
  `Framed`) and then called `examine`, which chose again, so the pass
  was compiled for four observer/instancing pairs that never run
  together. They now call `examine_as::<_, I>` with their own choice,
  made fresh per pass by a crate-private trait `Fresh` (a failure runs
  the pass twice, and `dual_literals` keeps a memo). The frame pass still
  runs per pass, as before.
- **F-8-03, one stop in the bindings.** Five `never` functions, each a
  type of its own, compiled the library's generic functions again beside
  the protocol's copies. Now one `never` in `web/src/lib.rs` and
  `unstopped()`, which hands it as the protocol passes its stop, a
  `&'static mut dyn FnMut(Progress) -> bool` (a leaked box of a function
  item, which takes no memory), at all 87 calls.
- **D4, the hasher.** The 15 files that built `std::collections`
  tables with SipHash take `crate::hash` (foldhash, fixed seed), and
  `core/clippy.toml` disallows the two std types; the alias in `hash.rs`
  and the integration test `core/tests/natural.rs` (which cannot reach
  the crate's hasher) `#[expect]` the lint with their reasons. No order
  of a table reached an output (std's tables were seeded per process, so
  none could have); the two journeys whose counts moved between runs
  for that reason count the same now.
- **F-5a-07, the lambda forms.** `serialize/lambda.rs` reads a term's
  types and nodes through flat forms (`TypeForm`, `NodeForm`: the kind
  and every field any kind has, read where it stands, then one `match`)
  in place of `Type`'s and `Node`'s derived readers tagged by `kind`,
  which buffer each object in serde's `Content`. A name no kind has is
  ignored as before; a field of another kind of the wrong type is now
  refused, the planning session's decision, and so are that field given
  twice and the array form (see the reviewer's findings), pinned by
  `a_node_s_fields_are_read_where_they_stand` and said in the
  changelog. What is written is unchanged; the id types' fields became
  `pub(crate)` so that the reader can make them.

## Measured

**llvm-lines** of the core crate (`cargo llvm-lines --package deduxit
--lib --release`, the audit's command, the base in a workspace of its
own at `tpwszxlz`): 3 355 021 lines in 69 775 copies before, 3 262 585
in 68 442 after the shims, F-1b-03 and D4 (−92 436 lines, −2.8 %). By
function:

| function | lines before | copies | lines after | copies |
|---|--:|--:|--:|--:|
| `svg::kripke::draw` | 19 256 | 378 | 6 404 | 120 |
| `modal::decide::decide_within` | 989 | 4 | 15 | 2 |
| `interpolant::maehara` | 7 314 | 148 | 3 653 | 71 |
| `interpolant::checked` | 4 414 | 22 | 2 206 | 11 |
| `interpolant::interpolant` | 612 | 6 | 304 | 4 |
| `NormalForm::check` | 2 810 | 12 | 1 419 | 8 |
| `export::write_drawable` | 513 | 3 | 462 | 3 |
| the checker's `Pass` (every method) | 102 192 | 1 110 | 68 128 | 740 |

**The web module** (`nix build .#deduxit-web`, the bindings' module after
`wasm-opt`, under the machine lock): 2 972 623 bytes at `gzip -9` before,
2 894 172 after every change (−78 451, −2.6 %); 8 713 887 bytes raw
before, 8 374 681 after (−339 206). At the head, after the reviewer's
fix to the lambda reader: 2 892 627 at `gzip -9` (−79 996, −2.7 %),
8 366 090 raw. The `budget` check passes there, and so do the bindings'
tests under Node; its ceiling stays (the budget raises a ceiling when a
change passes it).

## The net

- **Target sets** (cores 2 and 3): focused, 145 rows decided alike, 0
  differ in any counter, no verdict differs; net, 64 rows, 0 differ
  (`wykkvryo`).
- **The bindings' tests under Node** (`web-wasm-tests`): pass. Clippy on
  the bindings for wasm32, `--deny warnings`: clean.
- **Clippy** on the core crate with every feature, the defaults and none,
  `--deny warnings`: clean. The lambda, serialize, proofs and checker
  tests pass.
- **The gate** at `wykkvryo`: passed (clippy, the 764 core tests and
  the others, both `cargo hack` runs, the ratchet within its ceilings).
  On the journeys the shims touch the counts are within a percent of the
  ceilings: `render-latex` +0.00 %, `render-typst` +1.16 % (inside the
  tolerance of 2 %: the stop is now a trait object in its writer;
  `render-latex`, which takes the same path, did not move), `cut-elimination`
  −0.16 %, `modal-tableau` −0.84 %, `rules-check` −0.85 %,
  `session-rules` +0.02 %, `workspace-run` +0.30 %, `check-qbf-20-2`
  −0.31 %, `ordinary-resolution-first-order` −1.48 %. No generic body was
  kept for a cost. The ceilings were not lowered: the falls are under 2 %.
- **`breaks 0.8.0`**: the same three checks of cargo-semver-checks and the
  same 9 removed lines as at 87d1, all the base's; nothing new (the id
  types' fields are `pub(crate)`, no public change).

## The reviewer's findings

`fresh-reviewer` on Opus at `high`, over `tpwszxlz..wykkvryo`. The shims,
F-1b-03, D4 and the bindings' stop hold: the stop and `each` are called
as before, `limits::counting` gives `kripke::draw`'s stop the closures'
sequence, `Framed::fresh`'s `expect` cannot fire, no changed table's
order reaches an output, and `unstopped()` allocates nothing. Two
findings, both in `uxupvkpp`, both fixed there:

1. **The new test took the next one's `#[cfg(feature = "serialize")]`**,
   so the core tests no longer built with `parse` alone
   (`cargo hack check --feature-powerset --depth 2 --tests`, CI's
   `features-tests`; the gate runs the powerset without `--tests`). Each
   test has its own cfg and doc again; the powerset with `--tests` is
   clean.
2. **The flat forms changed more than the decided tightening.** Restored
   exactly: `kind` is read as an identifier, as serde reads a tag
   (`"kind": {"var": null}` is refused again, a kind of the wrong type
   says "expected variant identifier" again), and a missing field is
   raised where its object ends, through `try_from`, with its line and
   column. Two narrowings remain, which reading every field in place
   brings and the planning session was told of: a node or type written
   as a JSON array (`["var", 0]`), which serde's tagged reader took and
   no writer makes, and a duplicate field of another kind, are refused.
   The changelog says all three; the test pins them.

## What the next steps must know

- The nets' shims (F-3-10) and `export/svg/mell.rs`'s are 87d3's: 87e6
  had not landed.
- `render-typst` counts 1.16 % above its ceiling since its writer's stop
  became a trait object; within the tolerance, but the next raise of
  that journey should look at it before blaming its own change.
- A new hash table in the library is `crate::hash`'s; clippy refuses
  std's.
- F-5a-07's flat forms list every field of every kind: a new `Node` or
  `Type` variant adds its kind to `NodeKind`/`TypeKind` (and `NAMES`,
  `ALL`) and its fields to the form, or a document of it is refused.
