# Unit 9: verdicts

Places are relative to `/home/tux/Projects/own/rocq-deduxit` unless they
start with `core/` or `cli/` (deduxit's audit tree). Nothing was built or
run; every verdict is from reading and `grep`.

## F-9-01: rejected

The claim's second half is contradicted by the tests. Every one of the
exporter's preludes (`core/src/export/rocq/mod.rs:283-368`) is the import
block of a hand-written test (`tests/Certificates.v:15`,
`tests/TwoSided.v:11`, `tests/FirstOrder.v:14-15`,
`tests/FirstOrderTwoSided.v:12-13`, `tests/Weights.v:14`,
`tests/FirstOrderWeights.v:10-11`, `tests/Intuitionistic/Kripke.v:11`,
`tests/Intuitionistic/FirstOrder.v:12`, `tests/Modal/Certificates.v:9`,
`tests/Modal/Countermodels.v:8`), and every theorem name the exporter
writes is applied in a test: `grep -rlw` under `tests/` finds
`soundness_classical`, `soundness_mix`, `soundness_affine`,
`soundness_ill`, all the `*_refutation` names with their `i` forms,
`kripke_refutation`, `fo_classical_refutation`, `fo_kripke_refutation`,
`modal_soundness`, `labelled_soundness` and `modal_refutation`. The two
helpers the finding names as unchecked are used too: `weight_of` at
`tests/Weights.v:26` and fifteen more lines, `TermsCheck.terms_eqb` at
`tests/FirstOrderWeights.v:46,64`; the record fields are written
qualified at `tests/FirstOrder.v:56` (`Calculus.mix`, `Calculus.cut`) and
`tests/Intuitionistic/FirstOrder.v:27` (`Calculus.explosion`). A test
written by hand that applies a name fails when the name or its type
changes, so a rename fails this repository's own check first, and the
proposed `tests/Interface.v` would repeat what these tests do.
`tests/FromCommand.v` is the command's own output besides.

What is left of the finding is smaller and may come back alone: the
README lists modules and their main theorems (`README.md:151-164,
188-212, 280-292`) but has no sentence that says which names are the
certificates' contract (Q8 "each is listed", B9). That is a text finding
of small cost, not a missing check.

## F-9-02: confirmed

The witness is at `.github/workflows/release.yml:87-89`, and the job
uploads no asset (`gh release create` at `:94` passes none). Q1 says what
the finding says. No rules file gives a reason for the generated archive:
`CLAUDE.md:131-133` only describes it. Kind `build`, cost small. It
changes the next release only; 0.8.0's notes stay as they are.

## F-9-03: confirmed

Narrowed to its first part. `ci.yml` has one step, `nix flake check`, at
line 44 (the place says 45), and nothing in CI installs the package from
the opam file; `scripts/opam-install.sh` is run by hand. The recorded
reason (`.claude/rules/rocq.md:37-39`, `modules/opam.nix:10`: it needs the
network) holds for a flake check and not for a GitHub job, so it does not
settle the matter. The Rocq versions themselves are tested in CI, by the
flake's `rocq-deduxit-9_N` checks (`modules/rocq-deduxit.nix:34-38`): what
is untested is the opam file's `depends`, `build` and `install` per
version.

Corrections:
- `modules/rocq-bridges.nix` has 71 lines; the default Rocq is taken at
  `:20-21`, not `:163-164`.
- `--check-upstream` checks the archive a `url` section names, and this
  opam file has none, so the flag has nothing to check here; it belongs
  to the opam-repository copy. Drop that part.
- The bridges are not part of the package (`CLAUDE.md:50-52`) and the
  opam file claims nothing for them, so compiling them with one Rocq is
  no breach of Q7. A per-version bridge check also depends on the pinned
  `nanoll.v` and `nanoill.v` compiling on each version, which nobody has
  tried. Drop that part or raise it apart.

## F-9-04: confirmed

`theories/Linear/Calculus.v:38` holds the quoted line; `grep -rnE` for
`Notation IDENT :=` under `theories` counts 77, all `Local`. The README
states the warnings itself (`README.md:321-322`), and Q4 names this
deprecation. The recorded choice (`.claude/rules/rocq.md:35-37`) gives as
its reason that `Abbreviation` exists only from 9.2; the finder's answer
holds, since writing the application out needs no replacement command and
gives the same term on every version. The tests already avoid the form
for this reason (`rocq.md:656-658`).

Correction to the change: `Let` is not a safe substitute everywhere. A
`Let formula := …` cannot be declared in a file that defines a global of
that name (`Intuitionistic.v:70`, `FirstOrderIntuitionistic.v:91`), and a
`Let` is a section variable with a body, which `cbn`, `rewrite` and the
`perm`/`iperm` tactics' `match` on `eq_dec` see differently from a
notation. Write the applications out, or give the proof-side shorthands
(`eqb_spec`, `eq_dec`, `count`) other names as `Let` and check each proof.
Cost large as the finder says: about thirty files and every use.

## F-9-05: confirmed

Both places hold the same text: `Check.v:39-66,83-88` and
`FirstOrderCheck.v:125-152,171-176` are identical line for line
(`remove_one`, `remove_all`, `take_all`, `diff`, `take`), and the `let*`
notation is at `Check.v:25` and `FirstOrderCheck.v:32`. One section over
an element type and its Boolean equality would serve all four. The
recorded sentence (`rocq.md:165-166`, "changes none of its files") states
no reason that forbids it, and the checker files are outside the trusted
base (`rocq.md:75-83`).

Corrections:
- `CLAUDE.md:35-36` reserves `Deduxit.Base` for what two calculi share;
  these four checkers are all in `Deduxit.Linear`, so a module of that
  namespace fits the stated shape as well.
- The `formula_eqb_spec` scripts cannot become one lemma (each is an
  induction on its own inductive type); they can share a tactic only.
- `Soundness` exports `Check` (`rocq.md:419-423`), so `remove_one`,
  `take` and `diff` are visible to anyone who imports it; their types
  change, which is a small API change for a user of the library, not for
  a certificate.
- A generic function calls its equality through a parameter. Measure the
  certificates' `rocq compile` time before and after (Q10), with F-9-10.

## F-9-06: confirmed

`IntuitionisticCheck.v:389-428` and
`FirstOrderIntuitionisticCheck.v:536-575` differ only in `atom_eqb`
against `symbol_eqb` and the place of `Variable R` (checked by `diff`);
`same`, `unoc`, `cut_by` and `icut_conclusion` (`:140-183` against
`:171-212`) differ only in a comment. The propositional `iderive` writes
its matches out (`IntuitionisticCheck.v:193-215`). `let*` is a notation
for the same `match`, so the `steps` tactic keeps working when the
propositional `iderive` is rewritten with it. Cost medium.

## F-9-07: confirmed

All witnesses are in place: `mem` (`Intuitionistic/KripkeCheck.v:46`),
`kmem` (`Intuitionistic/FirstOrderCheck.v:207`), `among`
(`Modal/Countermodel.v:46`) are one definition; `existsb_false`
(`FirstOrderKripke.v:129`) and `existsb_false_in`
(`FirstOrderClassical.v:190`) have the same statement and proof;
`forallb_perm` and `existsb_perm` are at `Linear/Refutations.v:71,319`
and, generic, at `Intuitionistic/FirstOrderClassical.v:235,243`. The
rules' "shares nothing" (`rocq.md:431-432, 554-555`) describes the
namespaces and gives no reason against `Deduxit.Base`, which
`CLAUDE.md:35-36` foresees. All the files touched are proof-only, so no
trusted file gains an import. The gain is tidiness: each helper is three
or four lines, and the cost is medium.

## F-9-08: confirmed

`rocq-deduxit.opam:9-10` holds the sentence, which leaves modal logic
out although the same description goes on to describe `Deduxit.Modal`
(`:17-19`); `README.md:8-10` and `modules/rocq-deduxit.nix:70` are as
quoted. D4 gives the order. This repository's copy of `writing.md` lacks
the sentence on the suites that deduxit's rule 3 gained on 2026-11-26
(`grep -i suite .claude/rules/writing.md` finds nothing), and
`CLAUDE.md:205-207` says a change of the four rules belongs in both
repositories, so the copy is to be brought up to date in the same change.
Reordering the README's sections is medium work, not small; the opam and
Nix sentences are small.

## F-9-09: confirmed

No check reads `README.md` but `modules/conventions.nix:49` (the plan's
name). `no_contraction`, `excluded_middle'` and `four_needs_transitivity`
are in no file under `tests/`, `modules/` or `scripts/`. Correction: more
than `not_forall` has a twin. `tests/Intuitionistic/Kripke.v:20` states
the README's Kripke `excluded_middle`, and `tests/Modal/Certificates.v:40`
a lemma `four`, so three of the six blocks' lemmas are untested copies and
three have no copy; none is extracted from the README, which is the
finding. `deduxit`'s `cli/tests/readme.rs` reads its own README only.

## F-9-10: candidate

Reading supports it: `Linear/Formulas.v:78-81` joins the recursive calls
with `&&`, `andb` is an ordinary function, and the library's own note
(`Intuitionistic/FirstOrderClassical.v:56-58`) says `vm_compute`
evaluates both sides. Whether it matters is open: the right operand is
computed in vain only when two formulas share a head and differ in the
left part. Measure `rocq compile` of `tests/FromCommand.v` and of
deduxit's snapshot certificates on pinned cores before and after, and a
certificate with many formulas of one head in a zone. Note for the change:
`Formulas.v` is a trusted file, and the tactics that match `(_ && _) =
true` (`equalities`, `IntuitionisticCheck.v:391`) and the `_spec` proofs
must follow.

## F-9-11: confirmed

`_RocqProject` holds `-Q theories Deduxit` and the file list; `grep` for
`Default Goal Selector`, `Set Warnings` and `-arg` under `theories`,
`_RocqProject`, `Makefile` and `modules` finds nothing; only the tests
are required to print nothing (`modules/rocq-deduxit.nix:105-106`).

Corrections:
- The claim's last clause is wrong as worded. Without the setting a
  tactic acts on the first goal alone; `!` makes it fail when several
  goals are open. The risk is a tactic applied while more goals are open
  than the writer thought, not one that acts on all of them.
- `-w +deprecated` in the project file is what opam users build with, so
  it turns the present `Notation` deprecation into a failed install on
  9.2 and 9.3 until F-9-04 is done (the finder says "once F-9-04 is
  done"; it is a hard order). Within the opam bounds a new standard
  library could also add a warning that then breaks installs, which
  F-9-12's bound limits. Choose the warnings by name.

## F-9-12: confirmed

`rocq-deduxit.opam:45` is as quoted. The recorded reason
(`.claude/rules/rocq.md:29-32`: the standard library's versions do not
follow Rocq's) explains why the bound cannot copy `rocq-core`'s and gives
none for having no bound; Q1 asks for both bounds. The bound `< "9.3~"`
fits what the rules say was tested (9.0.0 with Rocq 9.0 and 9.1, 9.2.0
with 9.3); confirm the versions `scripts/opam-install.sh` installs before
writing it.

## F-9-13: confirmed

`theories/Linear/FirstOrderIntuitionisticCheck.v:658` is `21: {`, the
only numbered goal selection in the library (`grep` finds no other). The
block also uses the generated names `p1` and `p2` (`:662-663`). Adding a
rule at the end of `iproof`, where the quantifier rules went
(`rocq.md:183-185`), leaves 21 in place, so the proof breaks only for a
constructor placed before `ICut`; the finding says so. Small.

## F-9-14: confirmed

`diff` of `Weights.v:271-305` and `FirstOrderWeights.v:116-148` shows the
list lemmas equal but for one comment and one case; `Refutations.v:428-467`
and `FirstOrderWeights.v:436-475` differ in one qualified name. A section
over the relation `out` would hold `outs` and its list lemmas once.
Correction: the two `ill_ll` are inductions over different inductive
types (the propositional and the first-order `ill`), so no lemma serves
both; they can share a tactic at most, and the finder's change rightly
leaves them. Count them as witness of the repetition, not as something
the change removes. The gain is tidiness at medium cost.

## F-9-15: confirmed

`Mix.v:43` uses `IHll` and `H0`, `Refutations.v:230` `a0`,
`Soundness.v:102` `IHn`, and the inventory
(`target/audit/rocq.md:218-241`) lists 18 of 36 files refused, each with
an error such as "The variable A was not found". Each file stops at its
first error, so the number of sites is unknown and larger than 18: most
are `induction A` on a name the statement binds. The cost is large and
the gain is robustness against a naming change that the four tested
versions do not show; say so when it is weighed.

## F-9-16: confirmed

`FirstOrderWeights.v:403-413` proves `dual_dual`, `dual_subst` and
`occurs_dual`, `occurs_dual` is proved again at
`FirstOrderSoundness.v:150` with the same statement and script, and
`tsubst_closed` is at `FirstOrderWeights.v:194`. The change respects the
rule that the trusted first-order files hold definitions alone
(`rocq.md:80-82`): the lemmas go to `FirstOrderCheck.v` and
`TermsCheck.v`. `FirstOrderWeights.v` imports `TermsCheck` already
(`:27`) and would gain an import of `FirstOrderCheck`, whose names
(`check`, `state`, `take`) it must not clash with: `Require` without
`Import`, as it does for `Weights`. Small.

## F-9-17: confirmed

Narrowed, and the kind corrected. The Modal part holds:
`Modal/Check.v:46,74,85` prove one-way `formula_eqb_eq`, `item_eqb_eq`
and `mem_in` where `Linear/Formulas.v:91` proves a `reflect` named
`_spec` and `Intuitionistic/KripkeCheck.v:113` an iff. Adding `_spec`
lemmas and keeping the one-way forms as corollaries, as the change says,
breaks no name: that part is `refactor`, not `API break`, and its gain is
tidiness.

The `cut_r` part is rejected. `Intuitionistic/Calculus.v:34` declares the
record field `cut` in the same module, so a constructor named `cut`
cannot be declared there; and `cut_r` is the name the cut has in
`Linear/Calculus.v:74` and in the rules (`rocq.md:449`), so the present
name is the consistent one across namespaces. A rename would break users
of `lj` for no gain the finding names.

## F-9-18: candidate

`tests/FromCommand.v` is 44 317 bytes, and lines 61 and 64 (the statement
and the 192-node term of `from_command_6`) hold nearly all of it; each
`Tensor` node names both subformulas in full, which is the term's design
(`rocq.md:99-101`). Reading supports quadratic text. Whether a `let`
helps is for the measurement: file size and `rocq compile` time for `!A ⊢
A^⊗n` at n = 64, 128, 256, as written now and with bound subformulas, on
pinned cores. The change is in deduxit's exporter (unit 6b), it changes
every snapshot certificate with a repeated subformula, and the statement
repeats the formula too (line 61), which a `let` in the proof does not
shorten.

## Beside the findings

None.
