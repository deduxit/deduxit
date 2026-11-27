# Step 87c8: the sessions' shared code, lambda-terms, rule files, the workspace, the wire forms (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-87c1-64a1a5` (87c1's), from `plan/87-refactor.md`
(87c8, wave C) and the planning session's briefs `start-87-common.md` and
`start-87c8.md`, on cores 7 to 9. The base is `wxwpxkyl` (87b1's head).
Themes C8, the library's half (F-4e-04, F-4e-11, F-4e-12, F-4e-13,
F-4f-08, F-1b-07), and C11 (F-5a-08, F-5a-12, F-5b-08, F-5b-09,
F-5b-17, F-5b-18, F-6a-18). No change of behaviour.

## What landed

- **`wluqkros` Name the wire levels, in their order** (F-6a-18): the
  levels of `wire.rs` stand in one table in their order (10, a clausal
  proof, has no constant: nothing writes it), levels 2 and 3
  named (`ORDERED`, `CLASSICAL`, `SATURATED`) where they were bare
  numbers (`level_in`, `level_of`, `serialize/nets.rs`), and `LEVEL` is
  `POLYMORPHIC`, the last.
- **`qstzlswx` Rebuild the workspace's record index in one place**
  (F-5b-17): `Workspace::reindex`, which both readers in `form.rs` and
  `collect` call; `collect` remaps its records first, then reindexes.
- **`zrqwolnx` Share the rule files' repeated lookups, defaults and
  checks** (F-5b-08, F-5b-09, F-5b-18, F-4e-13): natural deduction's
  translator calls `rules::ordinary::terms_of`; `Fixity::fitting` and
  `Fixity::default_precedence` (crate-private) are the defaults the
  printer and the reader share; `draw::drawn_root` is the zones and
  root check `Drawing::new` and `Fitch::new` both open with; the
  session's `rule` lookup, `From<Unplanned> for Error` and
  `check_refused` replace ten copies.
- **`sytwuyqr` Share lambda-terms' naming code and the parsers'
  subscripts** (F-5a-12, F-5a-08's cheap part): `reduce.rs` calls
  `print::fresh` (at both call sites the name is taken, so the result is
  the same); `letter_class` and `letter_name` name a new variable for
  the read-back and the search; the parser's `subscripts` and
  `declares` serve the typed and the untyped reader.
- **`snzlptkl` Share the linear proofs' formula text and the session's
  repeated steps** (F-1b-07, F-4f-08): `proofs::member_text` writes a
  member for `Proof::formula` (its terms cut), `Derivation::formula`
  and `Interactive::formula`; the session's `open_goal`, `step_number`,
  `graft_ended`, `written_order`, `predicates` and `chosen` (the split's
  positions, the principal taken or not) replace the copies in
  `expand.rs`, `cut.rs`, `handover.rs`, `quantified.rs` and `witness.rs`;
  `cut` records its step through `record`.
- **`lslxruvv` Share the sessions' derivation bound and the ordinary
  session's repeated blocks** (F-4e-11, F-4e-12): `goal_tree::may_build`
  (the refusal past `derivation_bytes` and the stop) and
  `goal_tree::places` (the postorder renumbered) serve both sessions'
  `derivation_within` and the ordinary `derivation_ids`; in the ordinary
  session `fit_root`, `needs_of`, `next_eigenvariable` and `terms_full`.
- **`sqxznyus` Share the sessions' reader of a witness typed as text**
  (F-4e-04): `crate::witness` (behind `interactive`): the tokens, the
  stack of open applications and the refusals over a trait
  (`Witnesses`: `name`, `apply`) each session implements with its own
  tables. The one difference of the two loops, a `(` after a whole term
  (`f(a)(b)`: a stray bracket in the linear session, a missing comma in
  the ordinary one), is the trait's `BRACKET_AFTER_TERM`, so each keeps
  its faults. `a_witness_text_is_refused_for_its_first_fault` in each
  session pins twelve texts of one fault and three of several, where the
  sessions differ (`g(a`, `p(a`, `g(h)`: the linear session names the
  applied symbol's fault, the ordinary one the open bracket or the
  argument's), as the verdict on F-4e-04 asked; both tests pass on the
  code before the change too (run in a workspace at `lslxruvv`, then
  removed).
- The rules files: `core.md` (the root's `witness` and `goal_tree`'s two
  functions), `core-derivations.md`, `core-ordinary.md`, `core-rules.md`,
  `core-lambda.md`, `core-sequents.md` (the levels' order).

The tree grows by about 160 lines: the shared pieces carry their docs,
and the two witness tests are 70.

## Departures

- **F-5a-14 waits** (the brief's instruction): the tree layout that the
  typing derivation and a rule file's drawing both write goes to the
  next step that touches the exports after 87c3.
- **F-5a-08: the two lambda readers stay two** (departs from the plan,
  as its verdict advised): the untyped reader has its own semantics (free
  names become the context as they appear) and a merge puts an optional
  type through the typed parser's rule code; the cheap part, the
  subscript digits and the turnstile test, is shared.
- **F-4e-12: the eigenvariable sites are two pairs**, as the verdict
  corrected: one function for the pair over the inferences
  (`next_eigenvariable`), one value for the refusal built twice
  (`terms_full`); the two scans over the arena's terms keep their own
  errors.
- **F-5b-18: the checker keeps its own order** (as the verdict said):
  only the two layouts share `drawn_root`. `rules/draw.rs` is a drawing
  file; no other step of wave C touches it.
- No changelog line: nothing a user sees changed.

## The net

- **The gate's parts**, on cores 7 to 9: Clippy on the workspace and on
  the core crate alone, warnings denied, clean; `cargo test
  --workspace`, every suite passed (the sessions' tests in every suite,
  the behaviour lock, the wire levels and stability files among them),
  and the two witness tests again after their last change; both `cargo
  hack` runs and the powerset of depth 2 with `--tests`, warnings denied,
  clean after two imports were gated (`InfId` in `cut.rs` behind
  `serialize`, `TermId` in `natural.rs` behind `interactive` and
  `parse`).
- **The ratchet** (`--check`): every journey within its tolerance. Head
  against the base `wxwpxkyl`, built and counted the same way in a
  workspace of its own:

  | journey | base | head | change |
  |---|--:|--:|--:|
  | session-linear | 2 208 691 | 2 209 929 | +0.06 % |
  | session-ordinary | 39 299 826 | 39 282 457 | −0.04 % |
  | session-rules | 55 826 636 | 55 851 113 | +0.04 % |
  | lambda-check | 7 588 815 | 7 588 815 | 0 |
  | lambda-reduce | 30 802 732 | 30 817 089 | +0.05 % |
  | lambda-inhabit | 469 638 | 469 756 | +0.03 % |
  | rules-check | 23 672 639 | 23 672 639 | 0 |
  | workspace-run | 2 986 586 | 2 986 575 | −0.00 % |

  Heap blocks are equal but on `session-rules`, 141 296 against 141 302:
  natural deduction's terms no longer build a vector per formula
  (`terms_of` takes slices). `session-linear` stands 2.1 % below its
  ceiling at the base too; no ceiling was lowered (as at 87c1, for the
  planning session after wave C).
- **The target sets** on cores 2 and 3 (given by the planning session):
  `bench/targets/87c8.csv` against `86.csv`, no verdict differs and none
  of the 145 problems decided alike differs in a counter;
  `net-87c8.csv` against `net-86.csv`, likewise for its 64.
- **`breaks 0.8.0`**: nothing of this step; the same three failures and
  diff as at 87c1, all earlier steps' and in the changelog.
- **`nix flake check`** at `pwvpwkoq`, under the machine lock: "all
  checks passed!" on its second run. The first run was stopped by the
  planning session's pause (20:28), and the one after the resume failed
  `doc`, `doc-private` and `manual-links`: three links in
  `linear/proofs/interactive/expand.rs` named `CheckError`, whose import
  `snzlptkl` had dropped. They now give its path, folded into
  `snzlptkl`, and `cargo doc` with warnings denied, public and private,
  was clean before the second run. No other check failed.

## The reviewer

`fresh-reviewer` on Fable 5.2 at `medium`, over the seven commits, the
briefs, the verdicts and the code. No change of behaviour found; every
shared function compared equal to the copies it replaced, edge cases
included. Two findings, both answered:
- **The witness tests pinned no text with several faults**, which the
  verdict on F-4e-04 asked for: three such texts added to both tests
  (above), and both pass on the code before the change.
- **"Every level is a named constant"** was untrue of level 10: the
  rules sentence, the commit's subject and this report say what holds.

## What the next steps must know

- `crate::witness` is where a third session reads witnesses from text.
- The sessions' `derivation_within` share only the bound and the
  renumbering; their estimates stay their own.
- A new wire level goes at the end of `wire.rs`'s table, and `LEVEL`
  moves with it.
