# Step 84: untyped lambda calculi, a β stepper and principal types

## Outcome

deduxit reads, writes and checks untyped lambda-terms beside the typed
ones, reduces any term by β one step at a time, and gives an untyped
term its principal simple type.

- **Three untyped calculi**: `lambda::Calculus::Untyped`,
  `UntypedLinear` and `UntypedAffine` (JSON `untyped`,
  `untyped_linear`, `untyped_affine`), whose terms have variables,
  `λx. M` and `M N` and no types. `Term::parse` reads them (file
  `core/src/lambda/parse.rs`, `Parser::untyped`), the printer writes
  them, and `Term::check` checks how often they use their variables,
  with the refusal's reason. A document of one is at wire level 30.
- **A β stepper**: `Term::reduce` (file `core/src/lambda/reduce.rs`)
  takes steps by a named strategy (`Strategy::NormalOrder` by default,
  `ApplicativeOrder`), within `ReduceOptions`' bounds on steps and on a
  term's nodes, each step's redex named (`Step::redex`), every reduct
  checked, and ends in `End::NormalForm` or not finished (`End::Steps`,
  `End::Nodes`, `End::Refused`), never "no normal form".
  `Strategy::pick` names the next redex and `Term::write_marked` says
  where a subterm's text stands.
- **Principal types**: `Term::principal_type` (file
  `core/src/lambda/infer.rs`) by unification with the occurs check,
  returned as the term typed at it in the simple calculus and checked by
  the existing checker, or `Error::IllTyped` with the equation that
  cannot hold.
- **The command**: `term check --calculus untyped|untyped-linear|
  untyped-affine`, and the new `term reduce` and `term infer` (file
  `cli/src/term.rs`), in the manual's chapter "Proofs as programs"
  (`docs/src/terms.md`), the status chapter and the guide for AI models.
- **The protocol and the bindings**: the operations `lambda.reduce` and
  `lambda.infer` with the kind `lambda.reduction` and the promises
  `reduces` and `principal`; the bindings' calls `lambda_reduce` and
  `lambda_infer` (file `web/src/lambda.rs`), rows in
  `plan/web/README.md`.

The commits are listed at the end, after the review and the checks.

## Decisions

- **Typing is a second axis of one `Calculus`, as three new values**
  (`Untyped`, `UntypedLinear`, `UntypedAffine`, added last, so no
  discriminant moves), with `Calculus::is_typed`. Step 85's System F
  adds its values the same way. The literature names each calculus
  (the untyped, the linear, the affine lambda calculus), and a flat enum
  keeps `Term`, its JSON and every front end's calculus parameter one
  value. Two enums (typing and uses) would have broken `Calculus` and
  every caller for no gain a user sees.
- **An untyped term is the same `Term` with no types** (a break of the
  API, listed in the changelog): `Variable::ty`,
  `Term::goal` and `Typing::ty` return `Option<TypeId>`. The
  alternatives were a dummy type (a public accessor returning garbage)
  or a second term type (a second reader, printer, checker and JSON
  form). The invariant (a typed term has every type, an untyped one
  none) is kept by every builder and checked by the JSON reader. The
  changelog's breaking list says what to do.
- **The untyped calculi have variables, `λ` and application alone**, as
  the untyped lambda calculus of the textbooks (Barendregt 1984; Pierce,
  *Types and Programming Languages*, ch. 5). A free name is a free
  variable (the context, in the order the names first appear) unless a
  context is written before `⊢`, which then must declare every one.
  `term check` says `well formed`/`ill formed` for an untyped term,
  since "typed" says nothing of it; the library's error stays
  `Error::IllTyped` (code `ill_typed`), whose reason names the calculus.
- **The stepper's API mirrors cut elimination's** (`proofs::cut`):
  `ReduceOptions` (`strategy`, `steps`, `nodes`, defaults as named
  constants, serde, flags `--strategy`, `--steps`, `--node-limit`),
  `Strategy` with `pick`, `Step { redex }`, `End`, `Reduction`, and a
  callback `each(before, step)` for the front ends that print every
  step. A bound is an `End` variant (`Steps`, `Nodes`), not a
  `Refusal`, because "not finished at the bound" is the answer the
  prompt asks for, as `InhabitOptions`' depth gives "at least"; the
  limits' refusals and the stop are `End::Refused`. No settings key
  holds the options: like `InhabitOptions`, they are options of the
  call (the command's flags, the protocol's parameters, the bindings'
  `options` argument).
- **Only `(λx. M) N` is a redex.** β proper, which is the cut's redex
  in the read-back (`core-lambda.md`); the projections, `case` of an
  injection and the linear constructs' detours are not reduced, and the
  manual says so. Adding them is a follow-up (below).
- **Strategies by their current names**: normal order (leftmost
  outermost, the default, which finds a normal form when one exists)
  and applicative order (leftmost innermost). Read on 2026-11-27:
  Wikipedia's "Reduction strategy (lambda calculus)" ("normal-order
  reduction refers to leftmost-outermost reduction"; "applicative order
  reduction refers to leftmost-innermost reduction"), and the lamcal
  REPL's documentation on docs.rs, which names the same two `nor` and
  `app`. Burch's Lambda Calculator calls them "Normal" and "Eager", so
  the textbook names were taken.
- **Each step is a fresh tree** (`Copier` in `reduce.rs`): every binder
  of the copy is a new variable, so capture by id is impossible, and a
  binder that would take in a free variable of its name is renamed to
  its letters with the least free index (`y₁`), the textbook
  capture-avoiding substitution, so the text reads back. Every reduct
  is checked by `Term::check` (subject reduction for typed terms; the
  uses for linear and affine ones), a fault being a defect
  (`Error::ClaimFails`). The redex is marked in the text by
  `Term::write_marked`, which the command uses after `β:` and the
  bindings as a UTF-16 range.
- **Principal types by Hindley's algorithm**: one type variable per
  variable, `λ` giving `α → τ`, an application unifying its function's
  type with `argument → β`, unification with the occurs check
  (J. R. Hindley, "The principal type-scheme of an object in
  combinatory logic", *Trans. AMS* 146, 1969, pp. 29-60; Pierce, ch. 22).
  The answer is the term typed at the principal type, in the simple
  calculus, checked by the existing checker, which is how the prompt's
  "checked" is met, and which `term check`, `lambda.check` and
  `lambda.draw` then take. Type variables are written `a`, `b`, `c`,
  the atoms of deduxit's own type syntax (TAPL writes `X`, `Y`); the
  term is computed on a tree, since a DAG would force two places of one
  subterm to one type. A term with no type is `Error::IllTyped` with the
  application and the equation (`in \`x x\`, … a = a → b`). The tests:
  every typing the checker accepts over six types is an instance of the
  principal one (the prompt's test), and the result agrees with a
  recursive textbook unifier on 3 000 random terms.
- **Wire level 30** (`wire::UNTYPED`) for an untyped term and for a
  reduction: a reader of 0.8.0 refuses both by level.
- **The protocol**: `lambda.reduce` (a `lambda.term` to the new kind
  `lambda.reduction`, promise `reduces`: the reduction made again and
  compared) and `lambda.infer` (a `lambda.term` to a `lambda.term`,
  promise `principal`: the output checked, the type inferred again and
  compared). The bindings' `lambda_reduce` and `lambda_infer` run them,
  `lambda_reduce` adding the next redex's place for the page's
  highlight; the page's "Next step" is `{"steps": 1}`, "To the end"
  the default, and its undo keeps the terms before, as nothing is kept
  between calls.

D15 (one options value for every front end): `ReduceOptions` with
serde is the one value; the command maps `--strategy`, `--steps` and
`--node-limit` onto it, the protocol takes it as parameters, the
bindings pass it as JSON. The printing of a term stays `lambda::Options`
(the settings' `lambda`). D16: every bound is a named constant
(`DEFAULT_STEPS` 1 000, `DEFAULT_NODES` 100 000) with a flag. D17:
quantifiers over types (step 85's System F) go where `core-lambda.md`
says; the stepper copies any node, so type abstraction and application
need a binder entry in `binders` and `rebind` and a redex in `pick`.

## The review

`fresh-reviewer` (Fable 5.2 at medium) read the first three commits
against the prompt. One finding, which held: the principal type's
unification linked two function types in the union-find before their
operands were unified, so the occurs check no longer saw the operands
of the one linked away; `\z. \x. \k. k (z x) (z z)` then got a cyclic
type, and the solution's walk ran without bound (killed at 1 GB, past
the work limit, which that walk did not tick); `\z. \x. z x (z z)`
named the wrong application. Answered in `przrnxvt`: only variables are
linked now, a pair of function types is unified once (a set of pairs),
both witnesses are tests, and a new test compares the principal type
with a recursive textbook unifier on 3 000 random terms; I checked that
this test kills the old unifier (it no longer finishes) and passes the
new one. Everything else the reviewer probed held: captures by name and
by id, documents binding a variable twice or sharing a `λ`, typed
reductions with `let` and `copy`, the strategies, the bounds' wording,
the readers, the JSON reader's refusals of mixed documents, the
`Option` change's callers, the exit statuses, the protocol's checks and
the bindings' UTF-16 range.

## The checks

At the head `qqpmrwwk`:
- `gate` passed (Clippy on the workspace and on the core crate without
  `parallel`, every test, both `cargo hack` runs, the ratchet), and
  `cargo hack check --feature-powerset --depth 2 --tests -p deduxit`
  with warnings denied (the test-only module behind `parse`);
- `nix build` of `web-wasm-tests` (the bindings under Node),
  `wire-schema` (130 documents, among them the new untyped, principal
  and reduction ones), `manual`, `manual-links` and `budget` (the
  module 2 915 974 bytes at gzip, under the new ceiling);
- `breaks 0.8.0`: the step's breaks are the three signatures that now
  return `Option` (`Term::goal`, `Typing::ty`, `Variable::ty`), in the
  changelog; its additions are `Calculus`' three values, `is_typed`,
  `Phase::Reduce` (each last), the five new types, `write_marked`,
  `reduce`, `principal_type` and `Strategy::pick`. The one failure of
  cargo-semver-checks is step 81's change of `StepError::Witness`'s
  field, listed before.

## What is left open

- **Other detours are not reduced**: a pair taken apart, `case` of an
  injection, and the linear calculus's `let` of a pair, `derelict` of a
  `promote` and the like. With them the stepper would reach the normal
  forms of cut elimination for every connective, the tie to proofs the
  research saw as deduxit's own; it needs the commuting conversions too
  (`core-lambda.md`, the cut-elimination test).
- **The protocol's `KINDS` list in `core/src/protocol/mod.rs` names no
  lambda kind**, so an unknown kind's error lists none of them; a gap
  from before this step (the test holds `OPERATIONS` alone), left as it
  was.
- **The budget with step 82**: the chain is rebased onto step 82
  (`orukyztp`, the planning session resolved the changelog's
  `[Unreleased]`); step 82 took no wire level, so 30 stands, and the
  combined module is checked against the new ceiling by the full check
  on `ci/step-84`.
- **The client part** (D12's "Untyped" toggle, the stepper and the
  principal type on the page) waits for the planning session's go; its
  section follows.

## Commits (deduxit)

| change | subject |
|---|---|
| `puwpslqt` | Read, reduce and type untyped lambda-terms |
| `lztrtlsy` | Lock the term subcommands for untyped terms |
| `wqlzxzlw` | Reduce and type untyped terms in the web bindings |
| `przrnxvt` | Keep function types apart in the principal type's unification |
| `yxnuytup` | Compile the β stepper and the principal type once |
| `qqpmrwwk` | Raise the bindings' budget for untyped lambda-terms |

This report is the change after them.
