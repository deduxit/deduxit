# Unit 9 notes (rocq-deduxit)

## Build, opam, CI, modules

- rocq-deduxit.opam:44 `"rocq-stdlib" {>= "9.0"}`: no upper bound (Q1). rules rocq.md:29-32 record "bounds rocq-stdlib below only, since the standard library's versions do not follow Rocq's"; that reason does not forbid an upper bound (e.g. `< "9.3~"` or what 9.3 takes, 9.2.0 → `< "9.3~"`).
- rocq-deduxit.opam:8-9 description: "deduxit is a prover of classical and intuitionistic linear logic and of ordinary logic." Omits modal logic, lambda-terms; linear first (D4 order: ordinary, linear, modal, lambda). Q11/T3. synopsis ok.
- rocq-deduxit.opam: description puts Linear first at length; order Linear, Intuitionistic, Modal.
- release.yml:83-86: SHA-512 of GitHub's generated archive `archive/refs/tags/$TAG.tar.gz`; Q1 says the release is an uploaded archive since generated tarballs keep no stable checksum. Finding Q1.
- ci.yml: only `nix flake check`; opam lint and the opam install per version are scripts/opam-install.sh, outside CI (needs network) -> Q7 "the installation from the opam file is tested per version" not in CI.
- _RocqProject: only `-Q theories Deduxit`, no `-arg -w +...` (Q5: warnings as errors through -arg). Library build does not fail on a warning; only tests are compiled in silence.
- rocq.md inventory: -mangle-names refused by 18 of 36 files (Q5).
- rocq.md inventory: Assumptions asks 53 of 275; need to check that every theorem the exporter calls is in tests/Assumptions.v (Q6).
- modules/rocq-bridges.nix:163 `rocq = pkgs.rocq-core;` vs rocq-deduxit.nix uses `rocqPackages.rocq-core` (package built with pkgs.rocqPackages). Mixed sources of Rocq; small.
- modules/doc.nix:18-25 and rocq-deduxit.nix:41-48 the same `library` fileset twice (minor duplication).
- rocq-deduxit.nix:97 and rocq-bridges.nix:185 the same Axiom grep twice.
- rocq-deduxit.nix:70 meta.description: "linear logic, proof terms, ..., Kripke models of intuitionistic logic, and the labelled calculus of modal logic" — linear first; differs from opam's synopsis. Text.
- Makefile: fine (reference manual wrapper).

## README
- README.md:9 "deduxit proves and refutes sequents of linear, intuitionistic, classical and modal logic." linear first (D4 order ordinary, linear, modal, lambda). Whole README orders Linear, Intuitionistic, Modal (sections 122-292, What it covers 300-312, repo 333). T3/D4.
- README.md:66 `deduxit prove --mix --format rocq --standalone --output mix.v "A, B |- A, B"` – D1 (no default logic in 0.9.0) will change the command; the README of this repository is not run by deduxit's readme test. Note for remarks.
- README.md:73-86,128-149,172-185,218-235,244-273: Rocq examples; no check compiles them (grep tests: only not_forall is in tests/Intuitionistic/FirstOrder.v). T3 "an example that runs" — Q11.
- README.md:117 "Every theorem rests on no axiom" — true by inventory (275/275) but check asks 53.
- README.md:321-322 deprecation warnings on 9.2/9.3 (Notation for abbreviation) — Q4 breach recorded by rules rocq.md:35-37.
- README.md:352-357 AI disclaimer — allowed exception.

## theories/Linear propositional
- Formulas.v:91-104 formula_eqb_spec proof == IntuitionisticCheck.v:59-72 iformula_eqb_spec (same tactic script).
- Check.v:39-43 remove_one == IntuitionisticCheck.v:140-144 pick; Soundness.v:30-38 remove_one_perm == IntuitionisticCheck.v:357-365 pick_perm (identical proof). Q9 multiset module generic in the element type.
- Check.v:25-27 local `let*` notation; IntuitionisticCheck.v:189-335 iderive spells every option match out (nested 3-deep) — inconsistent; a shared option bind would serve both (Q3 consistency).
- Soundness.v:82 `Ltac perm` (global, not Local) vs FirstOrderSoundness.v:118 `Ltac perm` again; IntuitionisticCheck.v:389-428 equalities/steps/iperm/premise == FirstOrderIntuitionisticCheck.v:536-575 (to check).
- No `Set Default Goal Selector "!"` anywhere (grep) — Q5.
- Generated names relied on: Soundness.v:102 `IHn`; Mix.v:43 `IHll`, `H0`; Refutations.v:86 `H0`, 231 `a0`; Weights.v:592 `H0`. Inventory: 18/36 files refuse -mangle-names (Q5).
- Soundness.v:320-342 three instance theorems repeat the record literal of Mix/affine (no named `mix_rules`/`affine_rules` definition in Calculus.v, though Intuitionistic.v:65 defines `affine : irules`). Inconsistent: two-sided has `affine`, one-sided has none; certificates write record literals. Q3 consistency / API.
- Refutations.v:71-79 existsb_perm and 319-327 forallb_perm identical shape (minor).
- Proofs.v:60-61 doc comment of Cut after the final period (dangling); same IntuitionisticCheck.v:133-135 (minor).
- Weights.v:602-653 `ltac:(cbn; rewrite ?andb_true_iff; tauto)` 9 times; weighted_balance (435-480) and affine_balance (589-663) repeat the rule cases (tens, parr, with, plus, oc, de, co) near verbatim.

## theories/Linear first-order
- FirstOrderCheck.v:32-34 let* notation again (3rd copy; FirstOrderIntuitionisticCheck.v:99-101 4th).
- FirstOrderCheck.v:125-152 remove_one/remove_all/take_all/diff == Check.v:39-66 verbatim; take 171-176 == Check.v:83-88; join_with 183-200 ~ Check.v:95-111 (+eigens).
- FirstOrderCheck.v:66-80 formula_eqb_spec, same script as Formulas.v:91-104 (+terms_eqb case). 104-110 is_wn duplicate of Formulas.v:57-61.
- FirstOrderSoundness.v:47-126 remove_one_perm, remove_one_absent, take_all_perm, remove_all_perm, diff_count, Ltac perm == Soundness.v:30-90 verbatim (atom_eqb->symbol_eqb); 235-247 wk_list, wn_contract_cons, wn_contract == Soundness.v:97-109; 277-285 take_spec == Soundness.v:124-132.
- FirstOrderSoundness.v:118,212,223,304 `Ltac perm/facts/fresh_split/delta` not Local; IntuitionisticCheck uses Local Ltac — inconsistent.
- FirstOrderIntuitionisticCheck.v:171-212 pick/same/unoc/cut_by/icut_conclusion == IntuitionisticCheck.v:140-183; 374-401 pick_perm/same_perm/unoc_map == IntuitionisticCheck.v:357-384; 488-507 cut_by_sound/icut_conclusion_sound == IntuitionisticCheck.v:431-450; 536-573 Ltacs equalities/steps/iperm/premise == IntuitionisticCheck.v:389-428.
- FirstOrderIntuitionisticCheck.v:438-462 ifresh_cons_iff/app_iff/perm/nil == FirstOrderSoundness.v:155-190 fresh_* (other formula type); 403-421 ieigens_occurs/ioccurs_subst == FirstOrderSoundness.v:130-148; 472-477 not_reintroduced vs FirstOrderSoundness.v:622-625 inline; 763-764 == FirstOrderSoundness.v:694-695.
- FirstOrderIntuitionisticCheck.v:658 `21: {` selects the ICut goal by its number (order of constructors) — Q3/Q5 anti-pattern (depends on goal order).
- FirstOrderIntuitionisticCheck.v:248 uses `let* (l, C, e) :=` pattern; propositional IntuitionisticCheck.v:193-335 spells all matches out — the same checker written two ways.
- FirstOrderWeights.v:436-475 ill_ll cases == Refutations.v:428-467 verbatim (22 cases); +4 quantifier cases. 116-148 outs_app/outs_perm/outs_wn/out_wn_merge == Weights.v:271-305; 299-347 weighted_balance cases == Weights.v:432-480.
- FirstOrderWeights.v:403 dual_dual for first-order formulas lives in the weights file (propositional one is Formulas.v:52); 412 occurs_dual == FirstOrderSoundness.v:150; 194 tsubst_closed is a term lemma outside TermsCheck. Files split by topic (Q3 anti-patterns).
- FirstOrderWeights.v:29-31 long Import list of Weights names (name clash workaround, recorded rocq.md:353-356).

## theories/Intuitionistic
- Formulas.v, Calculus.v fine. Rule names `ax`, `ex`, `wk`, `co`, `wedge_r` ..., `cut_r`: `_r` means right here but `cut_r` is not a right rule; in Linear `_r` is every rule. Naming (Q3) minor.
- Kripke.v:77-95 kripke_soundness propositional cases == FirstOrderKripke.v:281-300 (with eta). persistence repeated.
- KripkeCheck.v:46 mem / 55 ups / 113 mem_In / 121 ups_past / 156 le_past == FirstOrderCheck.v:207 kmem / 216 kups / 319 kmem_In / 352 kups_past / 358 kle_past (same proofs, renamed with k).
- FirstOrderKripke.v:128-133 `existsb_false` == FirstOrderClassical.v:189-194 `existsb_false_in` (same statement and proof, two names, same namespace).
- FirstOrderKripke.v:89 update == FirstOrderClassical.v:100 cupdate; 99-107 closed_teval == FirstOrderClassical.v:104-111 closed_value; 112-126 teval_tsubst == 115-129 value_tsubst (same proof); 137-145 teval_update == 198-206 value_update.
- FirstOrderClassical.v:235-249 forallb_perm/existsb_perm (generic over X) vs Refutations.v:71-79/319-327 (over formula) — a generic list lemma written three times.
- FirstOrderClassical.v:56-70 allb/anyb with comment "Under vm_compute the standard forallb and && evaluate every member" — the library's own evidence that `&&` is strict under vm_compute. But every formula_eqb (Formulas.v:78-83, IntuitionisticCheck.v:48-52, FirstOrderCheck.v:45-58, FOICheck.v:45-55, TermsCheck.v:27,31) and KripkeCheck.v:69-101 (well_formed, forced, refutes) use `&&`/forallb. Q10 candidate: short-circuit equality in the checkers' hot path (remove_one calls eqb per list member).
- allb/anyb live in a semantics file (FirstOrderClassical) but are list utilities used by the checker.

## theories/Modal
- Labelled.v:26 `Definition label : Type := string.` labels fixed to string while atoms are a parameter; trusted file requires Stdlib String. (minor API)
- Check.v:46 formula_eqb_eq / 74 item_eqb_eq / 85 mem_in one-way, where other namespaces give reflect `_spec` (Formulas.v:91) or iff `mem_In` (KripkeCheck.v:113). Naming of the same notion: mem_in / mem_In / kmem_In / among_in.
- Countermodel.v:46 `among w l := existsb (Nat.eqb w) l` == KripkeCheck.v:46 `mem` == FirstOrderCheck.v:207 `kmem`; among_in (130) == mem_In (one dir) == kmem_In. Shared by Intuitionistic and Modal: by the library's stated shape (CLAUDE.md:35-36 "Deduxit.Base only once two calculi share a lemma") these belong in Deduxit.Base, which does not exist.
- Countermodel.v:146 `Lemma table` name (a lemma called `table`) vs nth_worlds in KripkeCheck.v:127 (same lemma: nth of map over seq). 
- Check.v:157-211 mcheck chains `&&` (strict under vm_compute) — only matters on rejection; formula_eqb && inside mem matters on acceptance.
- Soundness.v fine; g3_model long but structured with bullets & comments.

## tests, bridges, exporter
- tests/Assumptions.v: 53 theorems; every theorem the exporter applies is there (soundness*, soundness_ill x2, *_refutation incl. i-forms, FirstOrderWeights.*, kripke_refutation, fo_*, modal_soundness, labelled_soundness, modal_refutation). Q6 holds; no finding.
- tests/FromCommand.v:61-64: 192-node term is 44 317 bytes for the file; Tensor nodes repeat the whole left subformula → quadratic text in ⊗ depth. Q10 candidate (exporter could let-bind subformulas).
- Exporter (core/src/export/rocq/mod.rs:283-368 preludes; deduxit.rs:154-169,1128-1294 theorem names; weight_of, TermsCheck.terms_eqb, Calculus.* fields; ordinary/rocq.rs:786 kripke_refutation; rocq_models.rs:207,235; modal/rocq.rs). No list of these names in rocq-deduxit (README lists modules partly; grep "interface" none). Q8/B9.
- bridges fine; rocq-bridges check only default Rocq (modules/rocq-bridges.nix:163-164), not per admitted version.
- scripts/opam-install.sh:33 `opam lint` without --check-upstream; not in CI.
- opam line numbers: rocq-core :44, rocq-stdlib :45; description :9.
- README examples: README.md:128-149 (excluded_middle, excluded_middle', no_contraction) compiled nowhere.
