# Step 87c1: one arena for ordinary and modal logic (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-87c1-64a1a5`, from `plan/87-refactor.md` (87c1, wave
C) and the planning session's briefs `start-87-common.md` and
`start-87c1.md`, on cores 7 to 9. The base is main at `qpuqvuxv`.
Theme C1 (F-arch-08, F-6a-11). No change of behaviour but one refusal's
message (below).

## What landed

- **`tlturtkl` Share one formula arena and text reader between ordinary
  and modal logic.**
  - `core/src/arena/mod.rs` (crate-private): `Arena<N: Shape>`, the
    nodes, the atom names and the two hash maps, with `find`, `admit`
    (the foreign operand, the atom past the names, the full arena, in
    that order), `push`, `add`, `holds`, `atom` (NFC, `check_ordinary`,
    then a hook for the suite), `keyed` (the `entry` path of a
    first-order atom), `keyed_index`, `rekey` and the occurrence bound.
    `Shape` is the node's `operands` and `atom`; `Id` makes and reads an
    index.
  - `ordinary::Formulas` is `{arena, fo}`; `add` is the arena's `find`,
    `admit`, `push` with the binder's first-order entries between, as
    before. `modal::Formulas` is `{arena}`. Both keep every public item
    and `MOST`; `Debug` writes the same fields as before (written by
    hand, `Arena::fields`).
  - `core/src/arena/parse.rs` (behind `parse`): `Text` (the input and
    the place, `rest` out of line, `eat`, `turnstile`, `next_at`,
    `identifier` `#[inline(always)]`, `end`), `End`, the two expected
    lists, the `Reads` trait and `sides`, the loop over a sequent's
    formulas that both `parse_within`s ran, and `count_within`. Both
    parsers hold a `Text`; the ordinary `word` is an out-of-line call of
    `Text::identifier` in place of its copy.
  - The other ordinary files read the arena through accessors
    (`nodes()`, `len()`, `find`): `bruijn.rs`, `derivation.rs`,
    `instance.rs`, `interactive.rs`, `interpolant.rs`, `negative.rs`,
    `rocq.rs`, `transfer.rs`, one line each but `rekey_atoms`.
- **`wvotywkz` Give modal logic's JSON sequent the ordinary formula
  form.** `serialize/ordinary.rs`'s `FormulaForm<I = u32>` holds the
  tags of both suites (`□`, `◇` beside `∀`, `∃`), `map` reads its
  operands, `read_forms` is the loop both readers ran, `ids_at` the
  sides, `forms_within` the occurrence check; `VerdictWord` is shared.
  `serialize/modal.rs` keeps its own `SequentForm` (its keys differ:
  no `symbols`, `fo`, `binders`, and its `atoms` are names), its `From`
  and its arms of the reader.

Lines: the six suite files lose 283 lines net, the two new modules add
480 (half of them docs), so the tree grows by about 200 with the rules.
The value is that the arena, the parser's text handling and the JSON
formula are written once.

## Departures

- **A refusal's message changes** (departs from the plan's "no change of
  behaviour"; the brief's "the modal wire form is the ordinary one with
  two more tags" needs it, and 6a's verdict on F-6a-11 foresaw it). An
  ordinary JSON sequent with `□` or `◇`, and a modal one with `∀` or `∃`,
  are refused by the reader with `Error::Json` ("not an ordinary
  sequent of deduxit's wire form: `□` and `◇` are modal logic's, which an
  ordinary sequent does not hold"), where serde's "unknown variant …"
  refused them; a tag of neither suite gets serde's message, whose list
  of expected tags is now the twelve. Kind and code stay (`malformed`,
  `json`); the place in the document is no longer in the message.
  `each_suite_refuses_the_other_s_tags` pins both; the changelog says so
  under "Changed".
- **The sequents stay two** (F-arch-08 proposed one sequent generic over
  the formula's form): `ordinary::Sequent` and `modal::Sequent` are
  public, their fields read across each suite's files, and only `new`'s
  loop of `holds` was shared code. Part 2 may revisit.
- **The printers stay two**: the ordinary one is generic over the
  target's symbols and first-order; the modal one is twenty lines. No
  audit pair named them.
- **The parsers' operand loops stay two**: a shared loop would need the
  ordinary first-order paths (binders, predicates, equality, TPTP) behind
  a trait the modal parser refuses, with each refusal's error and place
  kept; the ordinary loop is the `read-tptp` journey's hot path, which a
  generic loop puts at risk for little gain.

## The net

- **The gate's parts**, on cores 7 to 9: Clippy on the workspace and on
  the core crate alone, warnings denied, clean; `cargo test
  --workspace`, every one of the 39 suites passed (the parsers' and
  serializers' tests of both suites, the lock, the wire levels, the
  stability files among them), and the new
  `each_suite_refuses_the_other_s_tags`; both `cargo hack` runs, and
  the powerset of depth 2 with `--tests`, warnings denied, clean after
  `Arena::rekey` was gated.
- **The ratchet** (`--check`): every journey within its tolerance. Head
  against this step's base (`qpuqvuxv`, built and counted the same way
  in a workspace of its own), instructions:

  | journey | base | head | change |
  |---|--:|--:|--:|
  | read-tptp | 34 338 231 | 34 275 138 | −0.18 % |
  | read-text | 12 594 372 | 12 598 468 | +0.03 % |
  | read-json | 12 279 683 | 12 279 683 | 0 |
  | read-lltp | 28 679 329 | 28 669 309 | −0.03 % |
  | ordinary-pigeons | 32 150 074 | 32 150 136 | +0.00 % |
  | ordinary-resolution-first-order | 5 962 012 | 5 969 375 | +0.12 % |
  | ordinary-connection-classical | 9 313 156 | 9 318 382 | +0.06 % |
  | session-ordinary | 39 300 404 | 39 300 326 | −0.00 % |
  | modal-tableau | 9 655 475 | 9 640 527 | −0.15 % |

  Heap blocks are equal on every journey. The journeys that stand
  above their ceilings by 1 to 1.5 % (`search-qbf-20-2`,
  `search-spec-chain`, `check-qbf-20-2`, `read-tptp`, `render-typst`)
  and `ordinary-resolution-first-order`'s 64 blocks over its ceiling do
  so at the base too. No ceiling was lowered (departs from the plan,
  which lowers a count that falls): `modal-tableau`'s fall is 0.15 %,
  and `ratchet --lower` would also write the counts that were below
  their ceilings before this step, in a file the steps beside this one
  may touch; the planning session lowers them once, after wave C.
- **The target sets**: not run. The step touches no file of
  `core/src/linear` and nothing the linear search calls.
- **`breaks 0.8.0`**: nothing of this step. cargo-semver-checks fails
  three checks, all earlier steps' and in the changelog's breaking list
  (the witness and binding faults in place of `reason`, the lambda
  calculus's new `Node` and `Type` variants, `Workspace::read_own`
  deprecated); cargo-public-api's diff names no item of `ordinary` or
  `modal`'s arenas, parsers or forms.
- **`nix flake check`** at `uruprwvn` (this report's first commit),
  under the machine lock: "all checks passed!", `wire-schema` (the
  forms byte for byte), `doc` and `doc-private` (every link resolves),
  `features`, `features-tests`, the `ratchet` in the sandbox, `wasm` and
  the web checks among them. Only this report changed after it.

## The reviewer

`fresh-reviewer` on Fable 5.2 at `medium`, over both commits, the briefs
and the code:
- **Three doc links unresolved** (`ordinary/parse.rs`: `parse_within`,
  `formula_in`, `read_tptp` linked `Refusal::Occurrences`, whose import
  the commit dropped): fixed in `tlturtkl`, the links given their path.
- **Borderline, kept**: the shared reader reads a form's operands
  before the suite refuses its tag, so `{"□": 0}` as the first formula of
  an ordinary document is refused for its index
  (`Error::IndexOutOfBounds`), and a document past the occurrence bound
  for the bound, before the tag. Only documents with a second fault meet
  it, and each is still refused; the changelog's sentence holds for the
  rest.
- Everything else it read holds: the refusals' order, the atoms' path,
  the push order, `Debug`, the parsers' errors and spans in both
  dialects, the inlining of `rest` and `identifier`, the bytes written.

Found by the gate, not the reviewer: `Arena::rekey` was dead without
`interactive` (`cargo hack`'s `--no-default-features`); gated in
`tlturtkl`.

## What the next steps must know

- `crate::arena` is where a third suite with formulas of this shape
  takes its arena from (a `Shape` for its node, an `Id` for its id).
- `Text::identifier` is `#[inline(always)]` on purpose
  (`core-ordinary.md`): the ordinary operand loop must keep its inlined
  copy now that the modal parser calls it too.
- 87e's moves and part 2 find the arena's fields behind accessors:
  `ordinary::Formulas::nodes()` is now in every build.
