From Stdlib Require Import List String ZArith.
From Deduxit.Linear Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ll (atom := string) {| mix := false; weakening := true; cut := false |} [var "a"; wn (tens (covar "a") (parr (var "a") (var "c"))); wn (tens (covar "a") (var "b")); tens (covar "a") (covar "b")].
Proof.
  apply (affine_refutation (weight_of String.eqb [("a", (-1)%Z); ("b", (-1)%Z)])
    (fun a => existsb (String.eqb a) []) false _).
  vm_compute. reflexivity.
Qed.
