# Step 87e5: the countermodels in a directory, the refutations' checks in a module (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-82-141614`, from `plan/87-refactor.md` (87e5, wave
E: moves only, public paths kept) and the planning session's briefs
`start-87-common.md` and `start-87e5.md`, on cores 10 to 12, the target
sets on cores 2 and 3 (the planning session's slot). The base was
`mwtrxynt` (87e6), on `main` since (`wwzwnkou`); the planning session
then stacked the chain on `xlzkwnly` (`main` and 87d2). Of the files
this chain touches, 87d2 changed two, each by one line putting a table
on the crate's hasher: `occurrences/instances.rs` (`eigen_names`, which
stays there; merged without a conflict) and `ordinary/model.rs`, whose
line went into `countermodel/mod.rs` in the first move's resolution.
Nothing else changed, so the target sets were not run again; `gate`
passed at the rebased head. No behaviour changes; nothing public moves. The three
moves are the work of one sub-agent (Opus, high) on briefs of this
session's, one move at a time, each checked here (clippy, the tests, the
docs) and committed by path before the next.

## What landed

**F-4d-10, ordinary logic's countermodels** (`kpqpqvqq`): seven flat
files of `ordinary/` into two directories, the countermodels' data and
checks apart from the searches that build them.

| old | new |
|---|---|
| `ordinary/model.rs` | `ordinary/countermodel/mod.rs` |
| `ordinary/structure.rs` | `ordinary/countermodel/finite.rs` |
| `ordinary/kripke_structure.rs` | `ordinary/countermodel/kripke.rs` |
| `ordinary/semantics.rs` | `ordinary/countermodel/semantics.rs` |
| `ordinary/models.rs` | `ordinary/models/mod.rs` (the driver) and `ordinary/models/finite.rs` (the signature, the expansion, the writer) |
| `ordinary/kripke_search.rs` | `ordinary/models/kripke.rs` |
| `ordinary/encode.rs` | `ordinary/models/encode.rs` (grounding reaches its `lookup` as before) |

Every public name stays at `ordinary::` by re-export, behind the same
features. The decision's code in `ordinary/mod.rs` (fixup-34's) is
untouched. `ordinary/rocq.rs` and `ordinary/rocq_models.rs` stay where
they were (departs from the plan: the planning session held them back,
since fixup-33 edits the classical Rocq certificate; the move is left,
below).

**F-1c-06, the refutations' checks** (`yrrrzkpz`): the linear suite's
second trust boundary, spread over four files beside the engines, is now
`search/refutation/`:

| old | new |
|---|---|
| `search/refutation.rs` | `refutation/mod.rs` (the refutations and `Disproof`, their tests), `refutation/phrase.rs` (their wording), `refutation/check.rs` (`RefutationFault`, `Failed`, every check) |
| `search/basis.rs`, `saturated.rs`, `weighted.rs` | `search/refutation/` |

`check` and `phrase` are private, every `check_*` private, `Disproof`'s
fields private; `basis` and `saturated` are `pub(super)` in `refutation`
(the reach they had as private modules of `search`), and `weighted` is
re-exported as `search::weighted` under the `rocq` feature for the Rocq
writer, its one user outside. No import of `refutation/` outside its tests
takes anything from `search`, `inverse` or `horn`.

**F-1a-09, `occurrences` free of `proofs`** (`kymlxkwv`): the proofs'
canonicalisation (`malformed`, `canonical`, `Canonical`) moved from
`occurrences/instances.rs` to `proofs/canonical.rs`, beside its caller.
`instances.rs` no longer imports `proofs`; to reach the numbering from
there, `Terms::lists` and `Numbering::list_number` are `pub(crate)` (as
their neighbours are) and `Numbering::eigen` is new (the eigenvariable's
term number, the expression the moved code wrote inline). Only test code
in `occurrences/ill.rs` still names `proofs`.

**Rules**, in the same commits: `core-ordinary.md`, `core-modal.md`,
`core-search.md` (its `paths:` and a bullet on the module's reach),
`core-inverse.md`, `core-horn.md`, `core-proofs.md`, `core-focus.md`,
`core.md`'s table and layout, `claude-infra.md`.

No changelog line: nothing a user sees changed.

## The net

- The focused target set (`bench/targets.sh`, cores 2 and 3) at the
  head: equal to `86.csv`, no verdict and no counter differs on the 145
  rows decided alike (`87e5.csv`). The net target set
  (`bench/net-targets.sh`): equal to `net-86.csv` on all 64 rows
  (`net-87e5.csv`). The refutations' `checked` column equals 86's in
  both (126 `ok` in the focused set, 31 in the net set, the rest empty
  as in 86).
- `gate` passed at the head (clippy, the tests, both `cargo hack` runs,
  the ratchet). The ratchet's journeys against their base (the ratchet
  run at `main@origin`, `wwzwnkou`, in a temporary workspace on the same
  cores): 36 of the 42 counts are equal to the instruction, among them
  every journey through the moved code (`ordinary-models`,
  `ordinary-kripke`, both `ordinary-sat-*`, `check-saturated`,
  `search-inverse`, `search-qbf-20-2`, `cut-elimination`,
  `session-linear`); the other six differ by at most 0.05 %
  (`read-spec` −0.05 %, `workspace-run` +0.01 %, the rest less).
- `breaks 0.8.0`: the same as at 87e6. cargo-semver-checks lists only
  what earlier steps changed (the witness faults' fields, System F's
  variants, `read_own` deprecated); the API diff has the same lines as
  87e6's (`Bias`'s path among them), and one header line more, `impl
  …::resolution::first_order::Refutation`, for the `check_within` that
  both diffs list (`krpknryx`, before 87e6; no commit of this chain
  touches `resolution`). Nothing of this chain is public.

## The reviewer

fresh-reviewer (Opus, high): nothing that breaks correctness or a stated
requirement in the three commits. It compared the old `refutation.rs`
with the new three files as line sets, as ordered blocks and byte for
byte: every block of the old file is byte-identical in its new place
(the checks are `check.rs:15-789`, old lines 945-1719), and only the
headers, the module docs, the `mod` and `use` lines and one `impl`
block split in two differ; `basis`, `saturated` and `weighted` differ in
their one `use` line (and one doc link). It found that no path inside
the moved code resolves to another item, that the engines reach no
check's internals (the reach of each module above), that every public
name is re-exported, that the renamed countermodel files differ only in
their imports, and that no file outside `plan/` names an old path. One
phrase it noted without counting it: `core-search.md` calls `weighted`
crate-wide for the Rocq writer, where the re-export exists only under
`rocq`, the writer's own feature.

## Left

- The Rocq move: `ordinary/rocq.rs` → `ordinary/rocq/mod.rs`,
  `ordinary/rocq_models.rs` → `ordinary/rocq/models.rs` (and
  `export/rocq/mod.rs`'s path to it), held back until fixup-33 lands.

## What the next steps must know

- A countermodel's data and its check go in `ordinary/countermodel/`,
  a search that builds one in `ordinary/models/` (`core-ordinary.md`).
- A refutation's check is in `search/refutation/check.rs`, which takes
  nothing from the engines; a new refutation kind keeps that
  (`core-search.md`).