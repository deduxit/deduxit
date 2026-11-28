# Step 87e3: the normal forms, first-order resolution and Herbrand along their seams (report)

Run on 2026-11-28 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-87c1-64a1a5`, from `plan/87-refactor.md` (87e3,
wave E: moves only, public paths kept) and the planning session's briefs
`start-87-common.md` and `start-87e3.md`, on cores 7 to 9. The base is
`zvkmkstu` (87c5 and 87e4 with their logs). F-4b-08 and F-4c-11. No
change of behaviour; nothing public moves.

## What landed

- **`slktosuu` Split the normal forms into their vocabulary, rewriting,
  first-order part, encodings and form test** (F-4b-08):
  `ordinary/normal.rs` (3 574 lines) is `normal/mod.rs` (the public
  types, `rewrite`, `normalize`, the claims and their check, the claims'
  helpers), `laws.rs` (the stages, the laws and their application, the
  passes, the bound, the run), `first_order.rs` (the quantifier laws,
  Skolem's and Herbrand's terms, `negation_normal`, `fresh_prefix`),
  `encode.rs` (Tseitin's and Plaisted and Greenbaum's encodings) and
  `form.rs` (`in_form`). `fresh_prefix` and `negation_normal`, which the
  rest of `ordinary` calls, are `pub(in crate::ordinary)` and re-exported
  by `normal` as before.
- **`krpknryx` Split first-order resolution into its inference, check,
  search, read-back and text** (F-4c-11): `resolution/first_order.rs`
  (2 787 lines) is `first_order/mod.rs` (the public types, `Clauses`'
  public methods, `resolve`, `clauses_of` and the entry points),
  `clauses.rs` (the inference function and what it rests on, a second
  `impl Clauses`), `check.rs` (`Refutation::check` and `check_within`, a
  second `impl Refutation`), `search.rs` (the given-clause search),
  `read_back.rs` and `text.rs`. The module is public, so every public
  item stays in `mod.rs`; the two public methods in `check.rs` are
  inherent and stay public.
- **`qwtxnrtl` Split Herbrand's instances into their search, matching and
  derivation builder** (F-4c-11): `herbrand.rs` (2 040 lines) is
  `herbrand/mod.rs` (the public types, `herbrand` and `deskolemized`),
  `search.rs`, `matching.rs` and `build.rs`.

Each move keeps the code as it was. What a sibling now reads is
`pub(super)`, a relative path one level deeper is written from the crate
(`crate::ordinary::…`), and the doc links that resolved through the old
file's imports or neighbours have full paths. The rules: `core-ordinary.md`
(the module list and the heads of the three sections) and `bench.md`
(which files seed std's `HashMap`).

## Departures

- **The private module of the laws is `laws`, not `rewrite`** (departs
  from the plan, which names it `rewrite.rs`): a module `rewrite` beside
  the public function `rewrite` makes every doc link to the function
  ambiguous, the public module doc's among them.
- **`law_claim` and the other claims' helpers stay in `normal/mod.rs`**
  (departs from the finding, which puts `law_claim` with the first-order
  part): the verdict groups them with the claims they serve, which are
  not first-order.
- **Herbrand's builder stays in `herbrand/build.rs`**: the finding moves
  it to F-4c-03's shared module, and since 87c5 it builds on the shared
  `ordinary::branch`; what is left is Herbrand's own oracle.

## The net

- **The gate** at the chain's head, on cores 7 to 9: Clippy on the
  workspace and on the core crate alone, warnings denied, clean; `cargo
  test --workspace`, all 39 suites passed; both `cargo hack` runs and the
  powerset of depth 2 with `--tests`, warnings denied, clean; the ratchet
  (`--check`) within its tolerance ("gate: passed").
- **The docs**, which the gate does not build: `cargo doc` of both
  crates, public and with private items, with and without every feature,
  warnings denied, clean (the reviewer found them failing first; above).
- **The ratchet, head against base** (`zvkmkstu`, counted the same way in
  a workspace of its own): the journeys that run moved code within
  0.06 %:

  | journey | base | head | change |
  |---|--:|--:|--:|
  | read-tptp | 34 278 159 | 34 275 138 | −0.01 % |
  | ordinary-pigeons | 31 818 931 | 31 825 277 | +0.02 % |
  | ordinary-sat-classical | 21 345 597 | 21 346 844 | +0.01 % |
  | ordinary-sat-intuitionistic | 28 187 815 | 28 188 833 | +0.00 % |
  | ordinary-resolution | 9 443 086 | 9 448 633 | +0.06 % |
  | ordinary-resolution-first-order | 5 943 037 | 5 943 852 | +0.01 % |
  | ordinary-ljt | 26 132 900 | 26 130 987 | −0.01 % |
  | ordinary-connection-classical | 9 187 536 | 9 187 455 | −0.00 % |

  No journey's heap blocks changed. The journeys that moved more run
  none of this code (`check-qbf-20-2` +0.51 %, `check-saturated`
  +0.40 %, `lambda-inhabit` +0.44 %, `rules-check` −0.71 %,
  `ordinary-models` −1.21 %, `ordinary-kripke` −0.52 %,
  `ordinary-connection-intuitionistic` −0.39 %, `session-linear`
  −0.30 %): the same journeys by the same amounts as at 87c6 and 87e4,
  against a base built in another directory.
- **87a's connection pin**: not run. No function the connection provers
  call moved (they take nothing from `normal`, `herbrand` or first-order
  resolution).
- **`breaks 0.8.0`**: nothing of this step. cargo-semver-checks fails
  the base's three checks. cargo-public-api's diff, with the names of
  87e1's moved types made one, removes the base's 9 known lines, as at
  87e4; it adds one line more than at 87e4, a second `impl
  ordinary::resolution::first_order::Refutation`: `check` and
  `check_within` are now in an impl block of their own (`check.rs`),
  which the tool lists apart; the methods and their signatures are as
  they were.

## The reviewer

`fresh-reviewer` (Opus 5.5 at `high`), with a multiset and a bigram
comparison of each old file against its new files: the moves clean, no
statement moved, every public item where it was, `cargo hack
--each-feature` clean, 48 unit tests and 4 doctests of the three modules
passed, the records right. One finding: the docs failed (`doc` and
`doc-private` with warnings denied): links to `Refusal`'s variants, to
`law` and to `Clauses::bindings_text` lost their targets in the moved
doc comments, and the new module `rewrite` collided with the function.
Answered in the three commits (full link paths, the module renamed to
`laws`).

## What the next steps must know

- `normal/`, `resolution/first_order/` and `herbrand/` are directories;
  a doc comment moved between their files resolves its links in its new
  module, so a link to an item the file does not import needs its path.
- The gate does not build the docs: a move should build them
  (`cargo doc --document-private-items`, warnings denied).
