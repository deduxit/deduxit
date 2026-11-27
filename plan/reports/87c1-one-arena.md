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

(filled at the end)

## The reviewer

(filled at the end)

## What the next steps must know

- `crate::arena` is where a third suite with formulas of this shape
  takes its arena from (a `Shape` for its node, an `Id` for its id).
- `Text::identifier` is `#[inline(always)]` on purpose
  (`core-ordinary.md`): the ordinary operand loop must keep its inlined
  copy now that the modal parser calls it too.
- 87e's moves and part 2 find the arena's fields behind accessors:
  `ordinary::Formulas::nodes()` is now in every build.
