# Unit 9: findings

Read in full: 65 files. Not read in full: none.

Places are relative to the Rocq library's root (`/home/tux/Projects/own/rocq-deduxit`), except where a place starts with `core/`, which is deduxit's (`/home/tux/Projects/own/deduxit-wt-audit`).

## F-9-01

- dimension: api
- criterion: Q8
- place: rocq-deduxit/README.md:151
- kind: build
- cost: medium
- candidate: no

**Claim.** The certificates' interface (the modules, theorems, record fields, constructors and helpers deduxit's exporter writes) is listed nowhere in the library and checked by none of its tests, so a rename here fails only in deduxit's flake once its lock moves.

**Witness.** The exporter picks theorem names as strings (`core/src/export/rocq/deduxit.rs:166-169`):

```rust
            (false, false, false) => ("classical".to_owned(), "soundness_classical"),
            (true, false, false) => (record(mix, weakening, cut), "soundness_mix"),
            (false, true, false) => (record(mix, weakening, cut), "soundness_affine"),
            _ => (record(mix, weakening, cut), "soundness"),
```

It also writes `weight_of` and `TermsCheck.terms_eqb` (`deduxit.rs:1202`, `:1282`), the fields `Calculus.explosion`/`Calculus.cut` (`core/src/ordinary/rocq_models.rs`), `kripke_refutation` (`core/src/ordinary/rocq.rs:786`), `fo_classical_refutation` and `fo_kripke_refutation` (`rocq_models.rs:207`, `:235`), and eleven preludes (`core/src/export/rocq/mod.rs:283-368`). In this repository the README lists modules and some theorems in prose (`README.md:151-292`), with no record fields, constructors or helpers. `grep -in interface README.md CHANGELOG.md` finds nothing. Every test under `tests/` is written by hand, so none fails when a name the exporter uses changes.

**Change.** Add `tests/Interface.v`, which `Check`s every name the exporter writes with its type (for example `Check soundness_mix : forall atom (atom_eqb : …) …`, `Check Build_finite_model`, `Check (fun r : Calculus.rules => Calculus.cut r)`). Add a README section that lists the same names as the library's stable surface, and a changelog line for each rename. The existing check compiles the file, so a rename fails here first.

**Value.** Users of certificates and the authors of both repositories: the contract becomes explicit (B9), and a breaking change shows in the commit that makes it, not one lock bump later.

**Net.** The new test file and deduxit's `rocq-deduxit` check. No checker, search or certificate changes.

## F-9-02

- dimension: idiom
- criterion: Q1
- place: rocq-deduxit/.github/workflows/release.yml:87
- kind: build
- cost: small
- candidate: no

**Claim.** The release publishes the SHA-512 of GitHub's generated tag archive, which the Rocq opam archive's guidance warns keeps no stable checksum, instead of an archive the release uploads.

**Witness.**

```sh
          url="https://github.com/$REPOSITORY/archive/refs/tags/$TAG.tar.gz"
          curl --silent --fail --location --output source.tar.gz "$url"
          sum=$(sha512sum source.tar.gz | cut -d ' ' -f 1)
```

Q1 (from rocq-prover.org/docs/opam-packaging): "the release is an uploaded archive with a SHA-512, since GitHub's generated tarballs keep no stable checksum".

**Change.** Build the archive in the job (`git archive --format=tar.gz --prefix=rocq-deduxit-$version/ $TAG`, or `nix`'s source), upload it as a release asset with `gh release create … rocq-deduxit-$version.tar.gz`, and print that asset's URL and SHA-512 in the notes for the opam file's `url` section.

**Value.** opam users and the opam-repository maintainers: a checksum that GitHub's regenerations cannot break, which an archive submission needs.

**Net.** `actionlint` (the flake's check) and one tag in a fork. No checker, search or certificate changes.

## F-9-03

- dimension: idiom
- criterion: Q7
- place: rocq-deduxit/.github/workflows/ci.yml:45
- kind: build
- cost: medium
- candidate: no

**Claim.** CI never installs the package from its opam file: the per-version opam install and `opam lint` run only by hand (`scripts/opam-install.sh`), `opam lint` runs without `--check-upstream`, and the bridges compile with one Rocq only.

**Witness.** The CI's one step:

```yaml
      - run: nix flake check --print-build-logs --keep-going
```

`scripts/opam-install.sh:33`: `opam lint "$repo/rocq-deduxit.opam"`, outside every check, since "the switches are built from the network, so none of this runs in a check" (`modules/opam.nix:173-174`). `modules/rocq-bridges.nix:163-164` takes `pkgs.rocq-core` and `pkgs.rocqPackages.stdlib` alone, while the library's check runs on 9.0 to 9.3. The rocq-community template's CI is a matrix of images, one per Rocq version, that installs the opam file (Q2).

**Change.** Add a GitHub Actions job with a matrix over the four admitted Rocq versions that runs the template's `docker-coq-action` (or `scripts/opam-install.sh VERSION`, which needs the network GitHub's runners have), and `opam lint --check-upstream`. Run the bridges' check for each admitted version as the library's check is.

**Value.** opam users: the bounds the opam file claims are what is tested, per version (Q7), before an archive submission.

**Net.** The new CI job itself. No checker, search or certificate changes.

## F-9-04

- dimension: idiom
- criterion: Q4
- place: rocq-deduxit/theories/Linear/Calculus.v:38
- kind: refactor
- cost: large
- candidate: no

**Claim.** The theories declare 77 abbreviations with `Notation`, deprecated in Rocq 9.2, so the library compiles with deprecation warnings on two of the four versions the opam file admits.

**Witness.**

```coq
Local Notation formula := (formula atom).
```

`grep -rhn "Local Notation [a-z_]* :=" theories` counts 29 for `formula`, 15 `term`, 12 `iformula`, then `eqb_spec`, `eqb`, `eq_dec` (4 each), `item` (3), `proof`, `count` (2 each), `state`, `span`. README.md:321-322 states the result: "On Rocq 9.2 and 9.3 it compiles with deprecation warnings (`Notation` for an abbreviation, whose replacement 9.0 and 9.1 lack)." The rules record the choice (`.claude/rules/rocq.md:35-37`): `Abbreviation` exists only from 9.2. That reason assumes the replacement must be `Abbreviation`. Each of these abbreviations shortens one application (`formula atom`, `formula_eqb_spec atom_eqb atom_eqb_spec`), which can be written out, or bound with `Let` in the section, on every version.

**Change.** Write the applied type in place of the type abbreviations (`formula atom`, `iformula symbol`, …), and bind the proof abbreviations (`eqb_spec`, `eq_dec`, `count`) with `Let` inside their sections. Keep the three `let*` notations, which are real notations.

**Value.** Every user who builds on 9.2 or 9.3 sees a silent build. The deprecation can become an error without breaking the library.

**Net.** The library's check on all four versions and deduxit's `rocq-deduxit` check. No statement changes (an abbreviation unfolds to the same term). No checker, search or certificate changes.

## F-9-05

- dimension: architecture
- criterion: Q9
- place: rocq-deduxit/theories/Linear/Check.v:39 and rocq-deduxit/theories/Linear/FirstOrderCheck.v:125
- kind: refactor
- cost: large
- candidate: no

**Claim.** The multiset core of the checkers (`remove_one`/`pick`, `remove_all`, `take_all`, `diff`, `take`, the option bind and their permutation lemmas and tactics) is written out once per checker, four times in all, though only the formula type and its equality differ.

**Witness.** `Check.v:39-43` and `FirstOrderCheck.v:125-129` are the same text:

```coq
Fixpoint remove_one (A : formula) (l : list formula) : option (list formula) :=
  match l with
  | [] => None
  | B :: l' => if eqb A B then Some l' else option_map (cons B) (remove_one A l')
  end.
```

The same function is `pick` at `IntuitionisticCheck.v:140-144` and `FirstOrderIntuitionisticCheck.v:171-175`. `remove_all`, `take_all`, `diff` and `take` repeat at `Check.v:46-66,83-88` and `FirstOrderCheck.v:132-152,171-176`. Their lemmas `remove_one_perm`, `remove_one_absent`, `take_all_perm`, `remove_all_perm`, `diff_count`, the tactic `perm`, `wk_list`, `wn_contract_cons`, `wn_contract` and `take_spec` are `Soundness.v:30-132` and again `FirstOrderSoundness.v:47-126,235-285`, with `atom_eqb` renamed `symbol_eqb`. `pick_perm` and `same_perm` are a third and fourth copy (`IntuitionisticCheck.v:357-374`, `FirstOrderIntuitionisticCheck.v:374-391`). The `let*` notation is declared three times (`Check.v:25`, `FirstOrderCheck.v:32`, `FirstOrderIntuitionisticCheck.v:99`). The reflection proof of `formula_eqb_spec` repeats its `repeat match goal` script four times (`Formulas.v:91-104`, `IntuitionisticCheck.v:59-72`, `FirstOrderCheck.v:66-80`, `FirstOrderIntuitionisticCheck.v:62-77`).

**Change.** One module (`Deduxit.Base.Multiset` under the library's stated shape, `CLAUDE.md:35-36`) with a section over `{A : Type} (eqb : A -> A -> bool)`, the functions, the option bind, and the lemmas under `forall x y, reflect (x = y) (eqb x y)`, with `perm` as a tactic over that section. The four checkers instantiate it. The rules record that the first-order development "changes none of" the propositional files (`.claude/rules/rocq.md:165-166`). That protected the trusted statements while the development was new; the checker files are not trusted (`rocq.md:75-83`), and no statement changes.

**Value.** Readers who check the checkers' soundness read one multiset proof instead of four. A fix or speed-up (F-9-10) lands once.

**Net.** The library's check (every certificate under `tests/`, including the `check … = false` lemmas) and deduxit's `rocq-deduxit` check. Touches the checkers, not the statements: certificates stay as written.

## F-9-06

- dimension: architecture
- criterion: Q9
- place: rocq-deduxit/theories/Linear/IntuitionisticCheck.v:389 and rocq-deduxit/theories/Linear/FirstOrderIntuitionisticCheck.v:536
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The two-sided checker's cut and promotion machinery and the four tactics of its soundness proof are copied verbatim from the propositional file into the first-order one, and the same pass `iderive` is written in two styles.

**Witness.** `IntuitionisticCheck.v:389-394` and `FirstOrderIntuitionisticCheck.v:536-541` differ only in the equality's name:

```coq
Local Ltac equalities :=
  repeat match goal with
  | H : (_ && _) = true |- _ => apply andb_prop in H; destruct H
  | H : iformula_eqb atom_eqb ?A ?B = true |- _ =>
      destruct (eqb_spec A B); [subst | discriminate]
  end.
```

The same holds for `steps`, `iperm` and `premise` (`IntuitionisticCheck.v:398-428`, `FirstOrderIntuitionisticCheck.v:545-573`), `same`, `unoc`, `cut_by`, `icut_conclusion` (`:147-183` and `:178-212`), `unoc_map`, `cut_by_sound`, `icut_conclusion_sound` (`:376-450` and `:393-507`). The propositional `iderive` spells every option match out, three levels deep (`IntuitionisticCheck.v:204-216`), while the first-order one uses `let*` (`FirstOrderIntuitionisticCheck.v:254-258`).

**Change.** Put the two-sided helpers and tactics in a section parameterised by the formula type, its equality and its `!` constructor, shared by both files (with F-9-05's multiset module under it). Write the propositional `iderive` with the shared option bind, as the first-order one is.

**Value.** Readers of the two-sided soundness proofs read one set of tactics. A new two-sided rule (the plug-in path `rocq.md:659-671` describes) changes one place.

**Net.** The library's check and deduxit's `rocq-deduxit` check (the two-sided snapshots `ill.v`, `cut_ill.v`, `first_order_*_ill.v`). Touches the checker, not its statements.

## F-9-07

- dimension: architecture
- criterion: Q9
- place: rocq-deduxit/theories/Intuitionistic/KripkeCheck.v:46 and rocq-deduxit/theories/Modal/Countermodel.v:46
- kind: refactor
- cost: medium
- candidate: no

**Claim.** List helpers that two namespaces need are written once per file under different names, and `Deduxit.Base`, the library's stated place for them, does not exist.

**Witness.** Three names for one function:

```coq
Definition mem (v : nat) (l : list nat) : bool := existsb (Nat.eqb v) l.
```

(`KripkeCheck.v:46`), `Definition kmem (v : nat) (l : list nat) : bool := existsb (Nat.eqb v) l.` (`Intuitionistic/FirstOrderCheck.v:207`), `Definition among (w : nat) (l : list nat) : bool := existsb (Nat.eqb w) l.` (`Modal/Countermodel.v:46`), each with its own `mem_In` / `kmem_In` / `among_in` (`:113`, `:319`, `:130`). One lemma under two names in one namespace: `existsb_false` (`Intuitionistic/FirstOrderKripke.v:129`) and `existsb_false_in` (`Intuitionistic/FirstOrderClassical.v:190`). `forallb_perm` and `existsb_perm` are proved over linear formulas (`Linear/Refutations.v:71,319`) and again for any type (`Intuitionistic/FirstOrderClassical.v:235,243`). `teval_tsubst`, `closed_teval`, `teval_update`, `update` (`FirstOrderKripke.v:89-145`) repeat as `value_tsubst`, `closed_value`, `value_update`, `cupdate` (`FirstOrderClassical.v:100-206`) with the same proofs. `CLAUDE.md:35-36`: "`Deduxit.Base` only once two calculi share a lemma".

**Change.** Create `theories/Base/` (`Deduxit.Base.Lists`: `mem` with its `reflect` lemma, `allb`, `anyb`, `forallb_perm`, `existsb_perm`, `existsb_false`, `forallb_ext_in`, `existsb_ext_in`). Prove the term-evaluation lemmas once over a valuation of function symbols, and derive the Kripke and structure forms from it.

**Value.** Readers of the countermodel checkers of two suites (ordinary and modal logic) meet one set of names. The library keeps the shape it states.

**Net.** The library's check and deduxit's `rocq-deduxit` check. The names that change are not in any certificate (F-9-01's list says which are). No statement changes.

## F-9-08

- dimension: text
- criterion: D4
- place: rocq-deduxit/rocq-deduxit.opam:9
- kind: text
- cost: small
- candidate: no

**Claim.** The opam description leaves modal logic out of what deduxit does and names linear logic first; the README and the Nix package description put linear logic first too, against the suites' order.

**Witness.** `rocq-deduxit.opam:8-10`:

```
proofs and refutations it finds. deduxit is a prover of classical and
intuitionistic linear logic and of ordinary logic. The library defines
the sequent calculi of linear logic under Deduxit.Linear, a checker for
```

The same description then describes `Deduxit.Modal`, which the first sentence omits. `README.md:8-10`: "deduxit proves and refutes sequents of linear, intuitionistic, classical and modal logic." `modules/rocq-deduxit.nix:70`: "The Rocq library of deduxit: linear logic, proof terms, …". The README's sections run Linear, Intuitionistic, Modal (`README.md:122-292`, `:300-312`, `:333-335`). D4 and `writing.md` rule 3 set the order: ordinary logic, linear logic, modal logic, lambda-terms.

**Change.** opam: "The Rocq library that checks the certificates deduxit writes. deduxit proves and refutes sequents of ordinary logic, linear logic and modal logic." README line 8: "deduxit proves and refutes sequents of ordinary (classical, intuitionistic and minimal), linear and modal logic." Put the `Deduxit.Intuitionistic` sections before `Deduxit.Linear` in the README and in the opam description, and give the Nix `meta.description` the same sentence.

**Value.** Readers on the opam archive and GitHub see every suite the library covers, and no suite is the main one.

**Net.** `typos` and `conventions` (flake checks). No checker, search or certificate changes.

## F-9-09

- dimension: text
- criterion: Q11
- place: rocq-deduxit/README.md:128
- kind: build
- cost: small
- candidate: no

**Claim.** No check compiles the README's six Rocq examples or runs its command line, so an example can drift from the library and the command without anything failing.

**Witness.** `README.md:146-148`:

```coq
(* Not derivable: one more A than ~A. *)
Lemma no_contraction : ~ ll (atom := string) classical [covar "A"; tens (var "A") (var "A")].
Proof. apply (unbalanced_refutation String.eqb "A" _). vm_compute. reflexivity. Qed.
```

`grep -rn "no_contraction\|excluded_middle'\|four_needs_transitivity" tests modules scripts` finds nothing; of the README's lemmas only `not_forall` has a copy (`tests/Intuitionistic/FirstOrder.v:26`). `README.md:66`, `deduxit prove --mix --format rocq --standalone --output mix.v "A, B |- A, B"`, is run by neither repository's README test. In 0.9.0 no logic is the default (D1), so this command will need a logic flag.

**Change.** Add a check to `modules/rocq-deduxit.nix` that extracts each ```` ```coq ```` block of README.md into its own file and compiles it against the package, as deduxit's `cli/tests/readme.rs` does for its README. Update `README.md:66` with deduxit's 0.9.0 flags in the same pair of changes as D1.

**Value.** Users who copy the README's examples get examples that compile (T3: "an example that runs").

**Net.** The new check. No checker, search or certificate changes.

## F-9-10

- dimension: performance
- criterion: Q10
- place: rocq-deduxit/theories/Linear/Formulas.v:78
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** Every structural equality the checkers compute with joins its recursive calls with `&&`, which `vm_compute` evaluates in full even when the first side is already false, though the library itself avoids `&&` elsewhere for that reason.

**Witness.** `Formulas.v:78`:

```coq
  | tens A1 A2, tens B1 B2 => formula_eqb A1 B1 && formula_eqb A2 B2
```

The same at `IntuitionisticCheck.v:48-51`, `FirstOrderCheck.v:45-58`, `FirstOrderIntuitionisticCheck.v:45-55`, `TermsCheck.v:27,31,41` and `Modal/Check.v:37-40,69-70`. `remove_one`/`pick`/`mem` call these once per list member. The library's own note (`Intuitionistic/FirstOrderClassical.v:56-58`): "Under [vm_compute] the standard [forallb] and [&&] evaluate every member, which a quantifier decided at its first element need not."

**Change.** Write each case as `if formula_eqb A1 B1 then formula_eqb A2 B2 else false` (and `if symbol_eqb f g then … else false`), and adapt the `_spec` proofs' patterns. Do it once in F-9-05's shared module where possible.

**Value.** Users who check large certificates: less kernel time per certificate, if the measurement confirms it.

**Net.** Measure first: `rocq compile` time of `tests/FromCommand.v` and of deduxit's snapshot certificates on pinned cores, before and after, plus a synthetic certificate with many same-headed formulas in one zone (for example `?(A ⊗ B_i)` for 50 different `B_i`). Touches the checkers' equality, and `Formulas.v`, a file of the trusted base, though not a definition a statement uses. Behaviour: the library's check and deduxit's `rocq-deduxit` check.

## F-9-11

- dimension: idiom
- criterion: Q5
- place: rocq-deduxit/_RocqProject:1
- kind: build
- cost: small
- candidate: no

**Claim.** The project file sets no warnings as errors and no `Default Goal Selector "!"`, so the library's own build accepts any new warning or deprecation, and a tactic may act on every open goal unnoticed.

**Witness.** `_RocqProject` holds `-Q theories Deduxit` and the list of files, nothing else. `grep -rn "Goal Selector\|Set Warnings" theories` finds nothing. The tests compile in silence (`modules/rocq-deduxit.nix:105-106`), but the theories go through `mkRocqDerivation`, which does not. On 9.2 and 9.3 the theories already print deprecations (F-9-04). Q5: "warnings turned into errors by `+name` in the `Warnings` option or `-w`, passed through `-arg` in the project file".

**Change.** Add `-arg -w -arg +deprecated,+default` to `_RocqProject` (once F-9-04 is done), minus the standard library's transitive warnings (`-warn-transitive-library-file`) that the library cannot fix. Add `#[global] Set Default Goal Selector "!".` through `-arg`, or one `Set` per file, and fix the scripts that then fail.

**Value.** Maintainers: a deprecation or a tactic that hits more goals than meant fails the build of the version that introduces it.

**Net.** The library's check on all four versions. No statement or certificate changes.

## F-9-12

- dimension: idiom
- criterion: Q1
- place: rocq-deduxit/rocq-deduxit.opam:45
- kind: build
- cost: small
- candidate: no

**Claim.** `rocq-stdlib` has no upper bound, so a future standard library is admitted untested.

**Witness.**

```
  "rocq-stdlib" {>= "9.0"}
```

Q1: "`depends` with a lower and an upper bound on `rocq-core` and `rocq-stdlib` so that every installable version compiles". The rules record the choice (`.claude/rules/rocq.md:29-33`) because the standard library's versions do not follow Rocq's (9.3 takes stdlib 9.2.0). That is a reason for a bound of its own, not for none.

**Change.** `"rocq-stdlib" {>= "9.0" & < "9.3~"}`, the versions the admitted Rocqs take, raised with the `rocq-core` bound.

**Value.** opam users: no switch that installs a standard library the library was never checked against.

**Net.** `scripts/opam-install.sh` (or F-9-03's job). No checker, search or certificate changes.

## F-9-13

- dimension: idiom
- criterion: Q3
- place: rocq-deduxit/theories/Linear/FirstOrderIntuitionisticCheck.v:658
- kind: refactor
- cost: small
- candidate: no

**Claim.** The soundness proof of the first-order two-sided checker selects the cut's case by its goal number, which shifts when a rule is added before `ICut`.

**Witness.**

```coq
  induction p; intros i lc Cc ec I H G.
  21: {
    (* ICut: the cut formula's eigenvariables are in the set, and every
```

The rules' plug-in path adds a rule as a constructor of `iproof` and a case of the proof (`.claude/rules/rocq.md:659-671`). The CoqStyle anti-patterns rule out a script that depends on the order of goals (Q3).

**Change.** Select the case by its shape: `all: try match goal with I : iintroduced (ICut _ _ _) = _ |- _ => … end`, or state the cut case as a lemma and apply it there.

**Value.** Maintainers who add a two-sided rule: the proof breaks at the new case, not at an unrelated one.

**Net.** The library's check. Touches only a proof.

## F-9-14

- dimension: architecture
- criterion: Q9
- place: rocq-deduxit/theories/Linear/Weights.v:271 and rocq-deduxit/theories/Linear/FirstOrderWeights.v:116
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The first-order weighted count repeats the propositional one's list lemmas and its invariant's case analysis, and the first-order `ill_ll` repeats the propositional one's 22 cases verbatim.

**Witness.** `outs_app`, `outs_perm`, `outs_wn` and `out_wn_merge` are the same text in `Weights.v:271-305` and `FirstOrderWeights.v:116-148`, for example:

```coq
Lemma outs_app : forall l1 l2 o1 o2,
  outs l1 o1 -> outs l2 o2 -> outs (l1 ++ l2) (opar o1 o2).
```

`weighted_balance`'s cases are `Weights.v:435-480` and again `FirstOrderWeights.v:303-346`. `ill_ll`'s propositional cases are `Refutations.v:428-467` and again `FirstOrderWeights.v:436-475`.

**Change.** State `outs` and its list lemmas in a section over the relation `out` (any `out : F -> outcome -> Prop` with `opar`), shared by both files. Leave the calculus-specific inductives where they are.

**Value.** Readers of the refutation proofs read one list layer. A change to `opar` is made once.

**Net.** The library's check (`tests/Weights.v`, `tests/FirstOrderWeights.v`) and deduxit's `rocq-deduxit` check (the refutation snapshots). No statement changes.

## F-9-15

- dimension: idiom
- criterion: Q5
- place: rocq-deduxit/theories/Linear/Mix.v:43
- kind: refactor
- cost: large
- candidate: no

**Claim.** Half the library's files (18 of 36) rely on names Rocq generates, so they fail the manual's `-mangle-names` linter and can break when a Rocq release changes its naming.

**Witness.** `Mix.v:41-43`:

```coq
  induction H.
  - apply wk_r, ax_r.
  - eapply ex_r; [exact IHll | apply perm_skip, H0].
```

Also `Refutations.v:230` (`destruct (atom_eqb a a0)`), `Soundness.v:102` (`apply IHn`), `Weights.v:592` (`H0`). The inventory (`target/audit/rocq.md`, "The build with `-mangle-names`") lists the 18 files and their first failing lines.

**Change.** Name what each `induction`, `destruct` and `intros` introduces (`induction H as [a | l1 l2 H IH HP | …]`, `intros * H`), file by file, and add `-mangle-names` to a check once all pass.

**Value.** Maintainers: proofs that survive Rocq's naming changes over the four admitted versions and the next ones.

**Net.** The library's check on all four versions, and `-mangle-names` per file. Touches only proofs.

## F-9-16

- dimension: architecture
- criterion: Q3
- place: rocq-deduxit/theories/Linear/FirstOrderWeights.v:403
- kind: refactor
- cost: small
- candidate: no

**Claim.** Basic lemmas about first-order formulas and terms live in the file that first needed them, one of them twice.

**Witness.** `FirstOrderWeights.v:403-413` proves facts of `FirstOrderFormulas` in the weights file:

```coq
Lemma dual_dual : forall A : formula, dual (dual A) = A.
Proof. induction A; cbn; congruence. Qed.
```

with `dual_subst` (`:406`) and `occurs_dual` (`:412`); `occurs_dual` is also `FirstOrderSoundness.v:150`. `tsubst_closed`, a lemma about terms, is `FirstOrderWeights.v:194`, not in `TermsCheck.v`. The propositional `dual_dual` sits with its definition (`Formulas.v:52`). Q3's anti-patterns: files split by topic.

**Change.** Move `dual_dual`, `dual_subst` and `occurs_dual` next to `feigens` in `FirstOrderCheck.v` (the trusted `FirstOrderFormulas.v` holds definitions alone), and `tsubst_closed` to `TermsCheck.v`. Delete the second `occurs_dual`.

**Value.** Readers find a lemma where its definition's other facts are.

**Net.** The library's check. Only proofs move.

## F-9-17

- dimension: idiom
- criterion: Q3
- place: rocq-deduxit/theories/Modal/Check.v:85
- kind: API break
- cost: medium
- candidate: no

**Claim.** The same notions carry different lemma names and shapes across the namespaces: one-way `_eq` and `_in` lemmas beside `reflect`-valued `_spec` and iff-valued `_In` ones.

**Witness.** `Modal/Check.v:85`:

```coq
Lemma mem_in : forall i l, mem i l = true -> In i l.
```

beside `Lemma mem_In : forall v l, mem v l = true <-> In v l.` (`Intuitionistic/KripkeCheck.v:113`), `kmem_In` (`Intuitionistic/FirstOrderCheck.v:319`) and `among_in` (`Modal/Countermodel.v:130`). `formula_eqb_eq` and `item_eqb_eq` (`Modal/Check.v:46,74`) are one-way where the other namespaces prove `formula_eqb_spec : … reflect …` (`Linear/Formulas.v:91`). `Intuitionistic/Calculus.v:75` names the cut `cut_r` among `wedge_r`, `imp_r`, where `_r` means a right rule. Q3: one naming scheme, the standard library's suffixes (`_spec` for `reflect`, `_iff`) where in doubt.

**Change.** Prove `reflect` lemmas named `_spec` for every Boolean equality and membership test, and keep one-way forms only as corollaries. Name the cut rule of `lj` `cut` (or `cut_l`/`cut_r` by the side it acts on), with the changelog's list of renamings. None of these names is in a certificate.

**Value.** Users who prove with more than one namespace meet one scheme.

**Net.** The library's check, and F-9-01's interface test to show no certificate name moved. Touches no checker's computation.

## F-9-18

- dimension: performance
- criterion: Q10
- place: rocq-deduxit/tests/FromCommand.v:64
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** A certificate's proof term repeats each `⊗` node's whole subformulas, so its text grows quadratically with the depth of a formula: a 192-node term for `!A ⊢ A ⊗ … ⊗ A` (64 copies) makes a 44 KB file.

**Witness.** `tests/FromCommand.v:59-60`:

```coq
(** !A |- A ⊗ … ⊗ A, 64 times *)
(** 192 nodes *)
```

`wc -c tests/FromCommand.v` gives 44 317 bytes, nearly all in `from_command_6`, whose `Tensor` nodes each restate the left operand down to the leaves. The term names formulas, not occurrences, by design (`.claude/rules/rocq.md:99-101`).

**Change.** On the exporter's side, bind each subformula the term repeats once with `let A3 := tens A2 (var "A") in …` inside the proof, or with a local `Definition` before the lemma. The library's statements and checker stay as they are.

**Value.** Users of the command whose proofs have deep formulas: smaller certificates that compile faster, if the measurement confirms it.

**Net.** Measure first: file size and `rocq compile` time on pinned cores for `!A ⊢ A^⊗n` with n = 64, 128, 256, as the exporter writes it now and with let-bound subformulas. Touches the certificates' text (the exporter, unit 6b), not the library. Instrument for behaviour: deduxit's `rocq-deduxit` check.
