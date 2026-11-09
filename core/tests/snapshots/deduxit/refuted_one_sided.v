From Stdlib Require Import List String ZArith.
From Deduxit.Linear Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ll (atom := string) classical [wn (covar "d"); wn (tens (tens (tens (var "c") (var "d")) (var "d")) bot); tens (var "c") (var "c")].
Proof.
  apply (weighted_refutation (weight_of String.eqb [("c", 1%Z)])
    (fun a => existsb (String.eqb a) []) true _).
  vm_compute. reflexivity.
Qed.
