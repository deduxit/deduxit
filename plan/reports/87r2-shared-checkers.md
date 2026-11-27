# Step 87r2: one copy of what the Rocq library's checkers share

Opus 5.5 at `high`, in `../rocq-deduxit-wt-87r` on 87r1's head
(`xrzlqylk`, now on main), theme H3 of the audit but F-9-17 (F-9-05,
F-9-06, F-9-07, F-9-14, F-9-16), with each finding's verdict. One
commit in rocq-deduxit, `oprrnszl` "Write what the checkers share once"
(commit `fce3cbe4`, after the panel's and the reviewer's mends).

## Outcome

Every proof the checkers shared is written once; every name the library
had keeps its statement and its path.

- **Multisets (F-9-05).** `Linear/Multiset.v`: `remove_one`,
  `remove_all`, `take_all`, `diff`, `take`, `same` over any type with a
  Boolean equality, and `remove_one_perm`, `remove_one_absent`,
  `take_all_perm`, `remove_all_perm`, `diff_count`, `diff_in`,
  `diff_incl`, `take_spec`, `same_perm` under its reflection lemma and a
  decision. The four checkers' copies (`Check`, `FirstOrderCheck`, the
  two `pick`s and `same`s) and their lemmas (`Soundness`,
  `FirstOrderSoundness`, the two-sided files) are instances: a
  `Definition` that applies the generic function, a lemma `exact` of
  the generic one. The `let*` notation is `Multiset.Bind`'s, one
  declaration for the three, imported by each checker and exported by
  none (the verdict's point 4). The tactic `perm` is `Multiset.perm`,
  given the decision and the instance of `diff_count`.
- **The two-sided checkers (F-9-06).** `Linear/TwoSided.v`: `cut_by`,
  `icut_conclusion`, their soundness over any two-sided relation with
  exchange and cut (the verdict: the relation and the two rules as
  hypotheses), `cut_by_in` and `icut_conclusion_in` (the first-order
  file's), and the tactics `equalities`, `steps`, `iperm` and `premise`,
  each taking the equality, the decision or the calculus as an argument
  (the verdict: the tactics named each file's equality). An argument of
  an `Ltac` is elaborated where it is written, so a notation cannot be
  passed and an implicit argument is given (`@ill atom R`). The propositional `iderive` is written
  with `let*`, as the first-order one; its soundness proof is unchanged.
- **Lists of two namespaces (F-9-07).** `Base/Lists.v`
  (`Deduxit.Base.Lists`, the library's stated place, created): `mem`
  with `mem_In`, `existsb_false`, `forallb_perm`, `existsb_perm`.
  `KripkeCheck.mem`, `FirstOrderCheck.kmem` and `Countermodel.among`
  are `Lists.mem`; `mem_In`, `kmem_In`, `among_in`,
  `FirstOrderKripke.existsb_false`, `FirstOrderClassical.existsb_false_in`
  and the two pairs of `forallb_perm`/`existsb_perm` (`Refutations`, over
  linear formulas, and `FirstOrderClassical`) its instances.
- **First-order facts (F-9-16).** `Linear/FirstOrderFacts.v`
  (`dual_dual`, `dual_subst`, `occurs_dual`, beside the trusted
  `FirstOrderFormulas.v`, required by both users, as the verdict
  placed it) and `TermsCheck.tsubst_closed`; the copies in
  `FirstOrderWeights` and `FirstOrderSoundness` are instances.
- **Folded instances, in the proofs only.** The soundness proofs set
  `Local Arguments … : simpl never` on the instances (`Soundness.v`,
  `FirstOrderSoundness.v`, the two two-sided files): there `cbn`
  unfolded the new `Definition`s where it had left the stuck fixpoints,
  and `perm`'s `rewrite diff_count` stopped matching in
  `derive_sound`. One proof (`FirstOrderSoundness.take_in`, which
  unfolded `take` to its match) goes through `take_spec` now. The flag
  was global first; the panel showed that a user's `cbn` then no longer
  computed `check` on a closed term, and as `Local` it leaves `cbn`
  everywhere else as it was (`simpl nomatch` was tried and does not
  help: the unfolding exposes a stuck fixpoint, not a match).

Kept apart, each with its reason (departs from the plan where marked):

- `unoc` and `unoc_map`: sharing them needs a function that takes `!`
  apart in each file, which costs the lines it saves.
- F-9-14, the weighted counts' `outs` layer (departs from the plan):
  `outs` is an inductive in each file, and one shared inductive moves its
  constructors `outs_nil`, `outs_cons` (the step keeps every name); the
  verdict valued it at 20 to 35 lines and "value low".
- `wk_list`, `wn_contract_cons`, `wn_contract`: they name `ll` and `wn`
  (the verdict's point 2).
- The term evaluations of Kripke models and finite structures (`teval`,
  `value`, F-9-07's second half): the verdict counts it apart, as a
  design of its own.
- `allb`, `anyb`, `forallb_ext_in`, `existsb_ext_in`: one file uses
  them, so `Deduxit.Base` does not take them (its rule: two calculi).
- The reflection proofs of the four formula equalities
  (`formula_eqb_spec` and its kin, F-9-05's last witness): one is in the
  trusted `Formulas.v`, which would then require a module of tactics,
  and the first-order ones add the terms' case.

## The statements' identity

`Search _ inside M` for every module of main's library, printed with
both builds at `Printing Width 4000` and sorted: the 1 199 lines are
equal, and the new build has one more, `TermsCheck.tsubst_closed`.
`tests/Assumptions.v` unchanged and closed (the check).

## The net

- The library builds with every file of 87r1's strict build; the
  library's `nix flake check` at `fce3cbe4` (pinned by `?rev=`): passes
  (the four Rocq versions under the strict build, the tests, the
  interface, README's examples, the bridges, treefmt, conventions,
  typos, deadnix, actionlint).
- deduxit's `rocq-deduxit` and `rocq-deduxit-library` against it: pass.
- The statements: `Search` on every module of main's library and of
  this one, equal but for `TermsCheck.tsubst_closed` (above); the
  panel's stronger comparison below.
- A probe of `cbn` (`Cbn.v` in the session's scratchpad): `cbn` turns
  `check` and `icheck` of a closed term into `true`, as on main, and
  leaves `take` of an open list folded.

## The panel

- `panel-argument` (Fable, high), on the statements' identity: holds.
  It reran the comparison with `Set Printing All` and `About` on all
  1 071 old names (equal but for the flags), proved the old and new
  checker functions convertible (`Goal @Old.X = @New.X. reflexivity`
  for every definition of the four checker files, with negative
  controls), found `let*`, the keywords and the scopes unchanged after
  each prelude, and compiled the 71 snapshots and every test against the
  new build. One finding (low): the global `simpl never` stopped `cbn`
  from computing `check` on a closed term in a user's proof. Mended:
  the flag is `Local` to the proofs that need it (above). It also noted
  that the session's `Search` comparison could not see bodies, implicit
  arguments, flags or universe constraints; it closed the first three,
  and universes were compared by neither.
- `panel-integers` (Sonnet, high): nothing breaks. Old and new
  multiset operations convertible, `diff_count`'s truncated subtraction
  the same statement, edge inputs (empty lists, a count of zero, a world
  past the model) agreeing.
- `panel-counterexamples` (Opus, high): no counterexample. Old and
  new libraries gave the same answer on every input: the 71 snapshots
  and the library's 26 test files; 5 100 random two-sided terms under
  four rule sets and 2 400 cut-heavy ones (byte-identical `iderive` and
  `icheck` results, and 0 mismatches against a Python reference of its
  own); 5 100 random one-sided terms under four calculi; 2 595
  certificates `deduxit prove` wrote in 25 modes and 6 870 copies with
  faults planted in the term; 2 602 faults planted in the snapshots;
  5 312 faults planted in statements. It also proved the old definitions
  equal to the new ones for every input (`Equiv.v`, closed). Its runs
  were against `f9b945d7`; `fce3cbe4` changes tactics and comments
  only.

## The reviewer

`fresh-reviewer` on Fable, once; nothing breaks correctness, three
findings, all mended in the commit:
1. The changelog said `cbn` computes the instances "as before"; true on
   closed terms only (on an open one it may show `Multiset.diff …`). The
   sentence says so now.
2. `perm`, `premise` and the four reflection scripts were still twice or
   four times with no reason written: `perm` and `premise` are shared
   now, the reflection scripts are kept apart with their reason.
3. Two doc comments carried history (`Lists.v`, `Multiset.v`); reworded.
