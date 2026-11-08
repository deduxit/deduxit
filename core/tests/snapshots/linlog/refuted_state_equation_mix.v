From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ll (atom := string) {| mix := true; weakening := false; cut := false |} [wn (tens (var "a") (parr (covar "a") (covar "a"))); wn (tens (tens (var "a") (var "c")) (parr (covar "b") (covar "c"))); covar "a"; var "b"].
Proof.
  apply (weighted_refutation (weight_of String.eqb [("a", (-1)%Z); ("b", 1%Z)])
    (fun a => existsb (String.eqb a) ["c"; "b"]) true _).
  vm_compute. reflexivity.
Qed.
