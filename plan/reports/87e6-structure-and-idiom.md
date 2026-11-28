# Step 87e6: the MELL drawing, the nets' long functions, small matters of idiom (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-82-141614`, from `plan/87-refactor.md` (87e6, wave
E: moves only, public paths kept) and the planning session's briefs
`start-87-common.md` and `start-87e6.md`, on cores 10 to 12, the net
target set on core 2 (the planning session's slot). The base was
`xvytsmxt` (87c6 and fixup-35), rebased onto `main` (`zvkmkstu`, with
87c5 and 87e4) without a conflict once both had landed. No behaviour
changes; nothing public moves.

## What landed

**F-3-11, the nets' long functions** (none in the nets past 100 lines
now; clippy's `too_many_lines` run for the area lists none):

| change | what |
|---|---|
| `sytrkqml` | each case of the MELL cut step a method of `Work` (`axiom`, `tensor_par`, `weakening`, `dereliction`, `box_into_box`, `contraction`), the exponential cases over an `Exponential` |
| `ynxsqvsq` | the MELL sequentialization's stage into `open` and `split` |
| `mvmksvqy` | the MLL stage the same, both parts `#[inline(always)]` into the stage, as the stage is into the shared loop (87c4's measured choice) |
| `oprmnpmo` | `Symmetry::new` into `anchors`, `literal_classes`, `compound_classes` |
| `zptmmmur` | `is_essential` into `directed_cycle` and `dominators` |
| `xkrtkzwt` | the essential engine's set-up into `closure` (the charge and the allocation), `fill` and `Nonempty::of` |
| `umplssyp` | the MELL graph into `Builder::gather` and `Builder::layout` |
| `nspstolt` | `NetError::write_over`'s vertex text (`VertexText`) and the MELL fragment message (`write_not_mell`) out of the match |

**F-6b-03, `svg::mell::draw`** (about 1 390 lines, the crate's largest
function): `pluwqlro` reads the vertices through a `View` (occurrence,
kind, the formula as the look writes it, the id); `zvvknwms` makes the
drawing a `Layout` built by its layout passes (`build_tree`,
`count_levels`, `place_rows`, `measure_extents`, `place_anchors`,
`raise_links`, `raise_line_height`, `fix_heights`, `place_doors`,
`find_cycle`), each emitting pass a method into `Groups`, the seven
strings (`vertices`, `rule_nodes`, `wires`, `legs`, `door_wires`,
`boxes`, `links`, `cut_bars`). `draw` is 40 lines; the longest function
of the file is `Layout::of` (94, mostly its fields), and the
`#[expect(too_many_lines)]` is gone. The second commit is the work of
a sub-agent (Opus, high) on a brief of this session's, checked here and
by the reviewer; it is one commit where the brief asked one move per
commit (departs from the plan: the layout's passes and the emitting
passes landed together, each pass still a method of its own).

**Theme E3, small matters of idiom**:
- `rvskuzxz` F-3-15: the count equation bound before it is tested.
- `okpykpyn` F-3-14: the nets' production code before their tests, the
  tests' items under the module's one attribute (11 in `nets/mod.rs`, 4
  in `sequentialize.rs`).
- `suyyzpxq` F-1a-15: the linear parser denies `wildcard_enum_match_arm`,
  its one wildcard over `Term` (the ordered dual) naming every variant;
  `core.md`'s list of such modules has `parse`.
- `lrtnslmn` F-6b-15: every `manual_midpoint` site (six in the SVG
  layouts, one in `linear/proofs/first_order.rs`, one in
  `ordinary/derivation.rs`, 87c5's area, one line) by `midpoint`, which
  rounds toward zero as `/ 2` does (checked on this toolchain over
  10 000 pairs of each sign), and the lint turned on.
- `yzzrmyny` F-7b-20: the command's net drawing's `#[allow]` an
  `#[expect]` with its reason.
- `ppopqops` F-8-15: the bindings' stack size read with
  `from_str_radix`; the verdict kept the finding to that, and each build
  script now names the other.
- After 87c5 and 87e4 landed (the chain rebased onto `main` without a
  conflict), the items in their files, by a sub-agent (Opus, high) on a
  brief of this session's, checked here:
  - `ysszmqys` F-6a-19: `serialize/ordinary.rs` into `ordinary/mod.rs`
    (the forms read back), `tables.rs` (the truth table and the
    tableau) and `transforms.rs` (the written-only forms), with
    `SequentForm::of_sequent` for the eleven whole-sequent calls and
    `ReviewForm` through `CountermodelForm::of`, as the finding asks;
    the crate's `self_named_module_files` lint puts the parent in a
    directory. The JSON forms are pinned byte for byte (`serialize`,
    `stability`, `lock`, and the command's examples).
  - `lvkxwupz` F-4c-13: the propositional resolution's bounds bound once
    by `let` chains (`refutation.rs`, `saturate.rs`, `dpll.rs`).
  - `ovszzlnk` F-4c-15: the connection prover's leftovers (an unused
    parameter, a forwarding method, the four-tuple alias now a struct).
  - `tylyxtys` F-2b-04, F-2b-08, F-2b-11 in one commit (departs from the
    plan's one move per commit: two files carry hunks of two items,
    which jj splits only with an editor): the four `#[allow]`s of the
    focused engine `#[expect]`s with their reasons, and with no
    `#[allow(` left in the workspace `allow_attributes` and
    `allow_attributes_without_reason` turned on; the threaded schedule a
    module behind one attribute (`schedule/threaded.rs`, not named
    `alternate` as the finding suggests, since a module and its function
    of one name make the doc link ambiguous); the choice point's
    sentinel a named test (`Choice::tries_none`, `PAST`) and the Mix
    answer a `bool`.

**Rules**: `zssxnxzn` (`core-export.md`: the drawing's passes;
`core-nets.md`: the cut step's cases, the symmetry classes' parts).

No changelog line: nothing a user sees changed.

## The net

- The net target set (`bench/net-targets.sh`, 64 problems, core 2, built
  at `pluwqlro` in a temporary workspace while the drawing's sub-agent
  edited the working copy): equal to `net-86.csv` on every row, verdicts
  and every counter (`net-87e6.csv`).
- `gate` passed at the head before the rebase: `search-wide-m2-256`
  260 097 212 instructions (−0.11 % against its ceiling; 87c4 left
  260 344 989), `render-svg` +0.10 %, `render-typst` +1.15 % (the base's
  own), every journey within its tolerance. After the deferred items, `gate`
  passed again at the head (`search-qbf-20-2` +1.31 %, `check-qbf-20-2`
  +1.71 %, `read-tptp` +1.37 %, all within the 2 % tolerance and near
  main's own counts).
- The bindings' tests under Node (`web-wasm-tests`, under the machine
  lock): passed.
- The focused target set (`bench/targets.sh`, cores 2 and 3) at the
  head: equal to `86.csv`, no verdict and no counter differs on the 145
  rows decided alike (`87e6.csv`). The connection prover's pin
  (`bench/connection-targets.sh`, cores 10 to 12): equal to
  `connection-87a.csv` on all 1 312 rows (`connection-87e6.csv`).
- `breaks 0.8.0`: cargo-semver-checks lists only what earlier steps
  changed (the witness faults' fields, System F's variants, `read_own`
  deprecated). The API diff also shows `Bias`'s serde impls and items
  removed under `deduxit::linear::search::Bias` and present under
  `deduxit::linear::Bias`: a path change already on `main` (no commit of
  this chain touches `Bias`, `search/options.rs` or `search/mod.rs`),
  for the planning session to judge against part 1's public paths.
  Nothing of this chain is public.

## The reviewer

fresh-reviewer (Opus, high): no line whose meaning changed in any of the
17 commits, no public change. It diffed each commit's moves with a
script that ignores indentation and the added prefixes, read every
hunk, and ran a differential of the CLI built at the base and at the
head: 912 runs of `prove --net --format svg` over 19 MELL sequents in
three modes and six looks (696 drawings), 378 of `check --net` and
`eliminate --net` and 294 step drawings of seven proofs with cut links
(three inside a box): every drawing and message identical (24 runs
stopped at a 5 s limit differed only in the copy bound reached, which is
timing). Its findings: a rules line naming `Symmetry::new`'s `chained`,
which `literal_classes` holds now (fixed); and the E3 items and F-6a-19
not yet done, since their files were 87c5's and 87e4's (done after both
landed, below).

The four commits of the deferred items came after this review and were
checked by the sub-agent's runs and the net above (clippy in every
feature pair, the area's tests, the JSON locks, both target sets), not
by the reviewer, who runs once.

## What the next steps must know

- The MELL drawing's tables are empty until their pass runs, and only
  `Layout::of`'s order checks it: a new pass goes where its inputs are
  made (`core-export.md`).
- The MLL stage and its two parts are inlined into the shared loop on
  purpose (`search-wide-m2-256`).
