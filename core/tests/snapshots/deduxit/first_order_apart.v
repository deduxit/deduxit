From Stdlib Require Import List String.
From Deduxit.Linear Require Calculus.
From Deduxit.Linear Require Import FirstOrderSoundness.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ll (symbol := string) {| Calculus.mix := false; Calculus.weakening := true; Calculus.cut := false |} [fall (parr (covar "p" [bound 0]) (var "p" [bound 0])); awith (var "q" []) (var "r" [])].
Proof.
  apply (soundness_affine String.eqb String.eqb_spec
    (let f'1 := parr (covar "p" [bound 0]) (var "p" [bound 0]) in
     (With (var "q" []) (var "r" []) (Weaken (var "q" []) (Forall f'1 0 (Par (covar "p" [eigen 0]) (var "p" [eigen 0]) (Ax "p" [eigen 0])))) (Weaken (var "r" []) (Forall f'1 1 (Par (covar "p" [eigen 1]) (var "p" [eigen 1]) (Ax "p" [eigen 1]))))))).
  vm_compute. reflexivity.
Qed.
