# Step 85: System F, checked

Opus 5.5 at `high`, in `../deduxit-wt-fixup-30-3e23c4` (the fixup-30
session given this step), on `main@origin` with step 84's library part
(`lvzsyywy`), rebased onto step 83's (`yntvykyq`) without conflicts;
83 took no wire level, so System F's is 31.

## Outcome

System F is in deduxit (its library, command, manual, guide and
bindings, on `main` since the planning session's push, with its docs
fix `okrptpvw`) and on the page: the lambda tab's typing toggle reads
"Simply typed", "System F", "Untyped", and a System F term is checked
and its typing derivation drawn with `∀I` and `∀E`. The client's work
is in `../deduxit.github.io-85`.

## Outcome (the deduxit part)

System F is the third value of the lambda-terms' typing axis: three
calculi, `lambda::Calculus::SystemF`, `SystemFLinear` and
`SystemFAffine`, the simply typed, the linear and the affine calculus
with types `∀X. A`, the type abstraction `ΛX. M` and the type
application `M [A]`. Terms are read (`Term::parse`, Unicode and ASCII:
`forall X. A`, `/\X. M`, `M [A]`), written, checked in Church's style
(`Term::check`), drawn as typing derivations with the rules `∀I` and
`∀E`, exported as LaTeX, Typst and SVG through the existing drawing,
reduced by type β in 84's stepper (`(ΛX. M) [A]`), and read and written
as JSON at wire level 31. The command takes `--calculus system-f`,
`system-f-linear` and `system-f-affine` in `term check` and `term
reduce`; the bindings' `lambda_read` takes `system_f`, `system_f_linear`
and `system_f_affine`. No type inference and no inhabitation search: a
`λ` without its type is refused with the reason that type inference is
undecidable there.

Commits: `kvtrwrrz` (the library, with the reviewer's two fixes),
`oqlwslow` (the command, the manual's chapter, the guide), `pvyqxqlq`
(the bindings and their row), `zvnwmyor` (the module's size with System
F in the budget's note: 2 987 284 bytes at gzip, 2 954 598 before it,
under 83's ceiling of 3 020 000, so no raise).

## Decisions

- **Representation.** A `∀`'s bound variable is a de Bruijn index
  (`Type::Bound`), and the `∀` keeps the name it was written with, an
  entry of the new `Term::type_variables`, for printing alone. The type
  arena keys every type by `Type::key`, which erases that name, so types
  equal up to bound names are one id and every existing rule still
  compares types as ids (`core-lambda.md`'s invariant). A type abstraction
  binds a `TypeVarId` of its own, written `Type::Variable` in the types
  inside its body; its type is the body's closed over it
  (`poly::close`). Chosen over named binders with α-equivalence tests,
  which would have turned every `==` of the rules into a walk, and over
  names without hints, which would print every `∀` with a generated
  name. The cost: a type prints with the names of the first equal type
  the term made (`∀Y. ∀X. Y → (X → Y)` for a term written with `X`
  first), which the manual says.
- **Capture-avoiding substitution.** Every type a term writes is closed
  (`from_parts` refuses a loose index), so a type application opens a
  `∀` with a closed argument (`poly::open`) and nothing can be captured
  in the data. The capture left is the printed one: `print::write_type`
  renames a `∀` whose name is free in its body (an atom, a type variable,
  or the printed name of an outer `∀` the body refers to) to the least
  free index, `∀Y₁`. Tests: `a_type_application_substitutes_without_capture`
  (an atom `Y` and a `Λ`'s `Y` substituted under `∀Y`, the text read
  back and checked), and the type β test below.
- **The side condition of `∀I`** (Girard, Lafont and Taylor's §11.1: `X`
  "not free in the type of a free variable of v") is `check::scope`: a
  node's free type variables are those of the types it writes and of its
  subterms, less its own at a `ΛX`, and the root's must be none. A term
  variable's type is written at its binder, so one bound outside `ΛX`
  with `X` in its type leaves `X` free at the root. The check also
  refuses a `ΛX` with an atom's name or one that would take in another
  type variable of its name, so a checked term's text reads back.
  Test: `a_type_variable_free_in_a_hypothesis_is_not_generalized`
  (`x : X ⊢ ΛX. x`, made by hand since the parser cannot write it).
- **Type β copies with fresh type variables**: the stepper's copy gives
  each copied `ΛX` a `TypeVarId` of its own and rewrites the types under
  it, as term binders get fresh variables, so no `ΛX` is bound twice;
  `rename_types` renames a copied `ΛX` whose name would capture.
- **Which nodes and types.** System F over the simple calculus takes
  its nodes and types (`∧`, `∨`, `⊤`, `⊥`) besides `∀`; over the linear
  and affine calculi theirs (`⊗`, `&`, `⊕`, `!`, units), checking the
  uses of term variables as those calculi do. Girard defines the
  connectives by encodings; keeping the calculi's own is what the
  typing axis beside the use axis asks.
- **Notation**, from the sources read this session:
  - Girard, Lafont and Taylor, *Proofs and Types*, ch. 11, §11.1 (the
    PDF at paultaylor.eu, its text extracted): types `ΠX. V`,
    "universal abstraction" `ΛX. v`, "universal application" `t U`, the
    conversion `(ΛX. v) U` to `v[U/X]`.
  - Wikipedia, "System F" (revision of 8 May 2026): `Λα. M`, `∀α. σ`,
    application by juxtaposition.
  - Taken: `ΛX. M` (ASCII `/\X. M`), `∀X. A` (ASCII `forall X. A`, the
    keyword the first-order syntax already has), and the type argument
    in brackets, `M [A]`: juxtaposition would read a type argument as a
    term, since both are identifiers. Pierce's *Types and Programming
    Languages* could not be read this session (no fetchable text), so
    the manual cites only the book above; that TAPL brackets type
    arguments is from memory and is not cited.
- **Printing with the types off**: `ΛX.` and `[A]` are shown whatever
  `lambda::Options::types` says, since they are part of the term in
  Church style; the `λ`s' types follow the option as before.
- **Wire level 31** (`wire::POLYMORPHIC`) for a term of System F and a
  reduction of one; 84's terms stay at 23 and 30. The manual's JSON
  Schema has the new calculi, types, nodes and `type_variables`.
- **Labels**: `∀I` and `∀E`, in the natural-deduction style of the
  existing `→I`, `→E` (Girard's book has no rule names for the typing
  rules).

## Options for front ends (D15)

No new option: System F is a value of the calculus the term is read in
(`--calculus`, the bindings' `calculus` argument, the protocol's
statement), and printing takes the existing `lambda::Options`. The
stepper's options are 84's (`ReduceOptions`).

## Where quantifiers go (D17)

This step is the type quantifier. First-order quantifiers stay outside
the lambda-terms (`NoTerm::Quantified`).

## Checks (the deduxit part)

- `cargo test -p deduxit --lib lambda` (44 passed, the eight new System
  F tests among them) and `cargo test -p deduxit-web --test lambda` (4
  passed) during the work; `cargo clippy --workspace --all-targets --
  --deny warnings` clean; `cargo test -p deduxit-cli --test readme` (7
  passed) and `--test lock` (passed, nothing reblessed).
- On the rebased head (`pvyqxqlq` over `yntvykyq`): the gate (capped,
  cores 10-12) passed; `nix build .#checks.x86_64-linux.web-wasm-tests
  .#checks.x86_64-linux.budget` under the machine lock passed (175
  packages, the module 2 987 284 bytes, the renderers' 1 647 397);
  `breaks 0.8.0` lists System F's new variants of the closed
  `lambda::Type` and `lambda::Node` (in the changelog's breaking list),
  `Calculus`'s new variants and `TypeVarId`, `Term::type_variables` and
  `Calculus::is_polymorphic` as additions, beside step 81's renamed
  fields.
- Before the rebase, after the reviewer's fixes: both `cargo hack` runs
  (`--each-feature`, and `--feature-powerset --depth 2 --tests`) clean;
  the first gate failed on `Builder::type_variable`, dead without
  `parse`, then gated (and `poly::needs` behind `serialize`).
- Not run here: the full flake check (the planning session's), the
  `export` check's compile of a System F drawing (no snapshot pins one).

## The reviewer

`fresh-reviewer` on Fable 5.2 at `medium`, on the deduxit part, before
the client's. Two findings, both held and fixed in `kvtrwrrz`:

1. **Soundness: one type variable bound by two nested `ΛX`.** A JSON
   document (never the parser, which gives each `ΛX` its own id) could
   write `ΛX. λx:X. ΛX. x` under one `TypeVarId`: the inner `ΛX`
   generalized the outer `x`, the outer took `X` out of the root's set,
   and the check accepted `∀X. X → ∀X. X`; the reviewer's closed witness
   had the type `a → ⊥`, and its reduction failed with `ClaimFails`.
   Fixed: `check::scope` refuses a `TypeVarId` bound by two `ΛX` nodes
   (`Fault::TypeTwice`), which the side-condition argument needs (the
   rules file says why); the parser and the stepper's copy never make
   one. Test: `a_type_variable_bound_twice_is_refused`.
2. **Quadratic memory before any limit.** `poly::free` made a list of
   free type variables for every type of the arena, Θ(n²) for a chain of
   arrows over n variables, before the meter was asked: 344 MB for
   218 KB of text under a 50 MB bound, and the reader with no bound at
   all. Fixed: `poly::Walker` finds a written type's free variables on
   demand (a mark per type), each walk ticked and each result held
   against the limits; the reader's loose-index check is `poly::needs`,
   linear. The witness at n = 4 000 now peaks at 58 MB (91 MB before) and
   is refused at the memory bound in 4 s.

Checked and found sound by it: `close`, `open` and `substitute` under
nested `∀` and `∀`-typed arguments (about 60 terms through parse, check,
print in both notations, reparse and recheck), type β in both strategies
(about 25 reductions), linear and affine use through `ΛX` and `M [A]`,
the JSON reader's refusals, the refusal without types, the drawing and
export (read, not compiled). Not a finding: the parser's work on `n`
nested `ΛX` over one long arrow is quadratic, within the occurrence
bound it promises.

## The client part

Commits in the client's workspace `../deduxit.github.io-85`, on its
`main@origin` (`klmtowxs`, steps 82, 83 and 84's page work):
- `vsxvxznv` "flake.lock: Update deduxit": the input `deduxit` moved to
  deduxit's `main` (`f59ca480`, `vqwqlqkl`), which holds System F.
- `rsmzxsry` "Check and draw terms of System F":
  - `src/lib/lambda.ts`: the calculi `system_f`, `system_f_linear`,
    `system_f_affine`; the typing axis as `LambdaTyping` (`simple`,
    `system_f`, `untyped`) with `typingOf`, `useOf` and `calculusWith`
    joining and parting it with the use; `eraseTypes` drops
    `forall X.`, `/\X.`/`ΛX.` and `[A]` for the switch to Untyped (the
    binders' types first, since `/\` is also a type's ∧);
    `SYSTEM_F_EXAMPLE`, `TYPING_WORDS`, `TYPING_TIPS`.
  - `Toggles.tsx`: the typing `Seg` of three values. Its first value,
    "Typed" before, is "Simply typed" (the page's existing name of that
    calculus), since "Typed" beside "System F" would not say which.
  - `Lambda.tsx`: `SystemFSetup`, the sentences and a term to try before
    one is read. A System F term takes the typed path (`lambda_check`,
    `lambda_draw`); a `λ` without its type is deduxit's parse error,
    whose reason ("System F infers no types…") the field's note shows.
  - `suites.ts`: the mark `λ2` (Barendregt's name in the λ-cube),
    `λ2⊸` and `λ2⊸+aff`.
  - The design guide's D12 rows (`design/redesign.md`), which the rules
    make the spec, name the third typing; `.claude/rules/web.md` has a
    bullet for it.
  - Tests: `lambda.test.ts` (a System F term read and checked against
    the module, `eraseTypes` on a polymorphic text and on `∧` in a
    binder's type, the round trip of the nine calculi) and
    `tests/lambda.spec.ts`'s "a term of System F is checked and drawn,
    and loses its types for an untyped calculus" (the drawing's `∀I` and
    `∀E`, the mark, the link's `m=system_f`, the refusal's reason, the
    switch to Untyped); the first test now asks for "Simply typed".
- Checks: `vitest` (the lambda and suites files, 12 passed), `biome lint`
  and `astro check` clean, `playwright test tests/lambda.spec.ts` (9
  passed); the full `nix flake check` under the machine lock passed
  ("all checks passed!") before the reviewer's text fix and again at the
  head `rsmzxsry` after it, nothing flaky.
- The reviewer, the same agent continued on the client: nothing breaks
  correctness; one low finding held, three texts still describing the
  toggle as "Typed"/"Untyped" (the rules bullet, the design guide's D12
  rows, `Toggles.tsx`'s header), fixed and folded into `rsmzxsry`. It ran
  `eraseTypes`' regexes on 15 System F texts and read every result back
  with deduxit's untyped reader.
- Not shown: the panel's "Try" buttons do not show before a term is read
  in the current layout (no test presses them for any calculus), so the
  System F test types its term.

## Left open

- No export snapshot pins a System F drawing, so the `export` check
  compiles none (the reviewer read the LaTeX, Typst and SVG output).
- The parser's work on `n` nested `ΛX` over one long arrow is quadratic,
  within the occurrence bound it promises.
- A type prints with the names of the first equal type the term made
  (the manual says it); a front end that wants the names as typed would
  need the hint kept per place.
- The client's `flake.nix` description still reads "deduxit in the
  browser: linear logic proved by clicking", a suite named as the main
  one (fixup-30's matter, in the client).
