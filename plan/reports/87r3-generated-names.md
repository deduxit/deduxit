# Step 87r3: proofs that do not rely on generated names

Opus 5.5 at `high`, in a second rocq-deduxit workspace,
`../rocq-deduxit-wt-87r3`, on 87r2's commit (`oprrnszl`) while 87r2's
panel used the first one's build; theme H2 of the audit (F-9-15,
F-9-04, F-9-13), with each finding's verdict. Two commits:
`ozzxysrk` "Write out the abbreviations" and `looyoyxt` "Name what
every proof introduces" (commit `dd57f5a3`), rebased onto 87r2's
amended commit; the rebase's two conflicts (the `perm` tactic, now
`Multiset.perm`, in both soundness files) were resolved to 87r2's form.

## Outcome

- **The abbreviations (F-9-04).** No file declares `Local Notation
  NAME := (…)` any more: the 79 (77 at the audit, 2 more from 87r2's
  new files) are written out where they were used, by a script that
  replaced each use up to the end of the declaring section, outside
  comments and strings, in parentheses where the use is an argument or
  is applied. The verdict's warning is kept: no `Let`, whose discharge
  would put a `let` in front of the statements. The check's build now
  makes `notation-for-abbreviation` an error like every default
  warning; only `register-all` stays exempt (87r1).
- **Generated names (F-9-15).** Every file compiles with
  `-mangle-names M_`, and the check's build passes the flag, so the
  check fails on any proof that uses a name Rocq generated. The audit's
  inventory gave the first failure of 18 files; the fixes:
  - 47 sites by a script from each lemma's statement: `intros *`
    becomes the statement's binders, and a first `induction X` or
    `destruct X` on a binder not yet introduced gets the `intros` up to
    `X` (the normal build checked each);
  - `as` patterns that give the hypotheses the names Rocq generated
    before (`IHA1`, `IHll`, `H0`, `IHPermutation`), so the proof text
    below is unchanged: the formulas of the four calculi, `ll`
    (propositional and first-order), `ill` (both), `Permutation`;
  - `intros B` before the second induction variable of the equality
    proofs, `intros u` in `term_eqb_iff`;
  - `iperm` and `good_tac` bind the variable they introduce with
    `fresh` (`intros ?x` gives no name the tactic can use);
  - `out_wn_merge` (both weights files) takes the `?` premise's
    hypothesis by its shape;
  - three weakening premises renamed where the proof named them.
- **The goal picked by number (F-9-13).** The cut case of the
  first-order two-sided `iderive_sound` is the lemma
  `iderive_sound_cut` (the verdict's preferred form), applied to every
  goal by the shape of its hypotheses (`try (eapply (iderive_sound_cut
  … I H G); eassumption)`), which only the cut's goal matches. One new
  name; no statement changes.
- **Lines.** Writing the types out lengthened 54 lines past 100
  characters (46 were longer before). A mechanical wrap at spaces
  outside comments and strings (which changes nothing Rocq reads) split
  types, patterns and applications badly; the reviewer listed them, and
  those 38 are joined again. Neither repository sets a line width, so a
  long line stays where a break would read worse; the instance lemmas of
  `diff_count` and `remove_all_perm` are laid out by hand.

## The statements' identity

`Search _ inside M` for every module of the library, printed with
87r2's build and with this one at `Printing Width 4000` and sorted:
the 1 227 lines are equal; this build has one more,
`iderive_sound_cut`. The certificates' interface (`tests/Interface/`)
and `tests/Assumptions.v` compile and are closed (the check).

## The net

- Every file compiles with `-mangle-names M_`; the whole library builds
  (Rocq 9.1 here) after each change.
- The library's `nix flake check` at `dd57f5a3` (pinned by `?rev=`):
  passes, the four Rocq versions under the strict build with
  `-mangle-names` and no abbreviation exemption.
- deduxit's `rocq-deduxit` and `rocq-deduxit-library` against it: pass.
- The `cbn` probe of 87r2 passes here too.

## The reviewer

`fresh-reviewer` on Fable, once; nothing breaks correctness (it
checked the abbreviations' expansions for shadowing and section ends,
the remaining fresh-name patterns, that `iderive_sound_cut`'s `eapply`
can only close the cut's goal, and that the flags reach the check's
build alone). Two findings, both mended in `looyoyxt`:
1. `iderive_sound`'s doc comment sat on the new `iderive_sound_cut`;
   moved back to its lemma.
2. The mechanical wrap left about 40 lines hard to read (above); the bad
   splits are joined again.
