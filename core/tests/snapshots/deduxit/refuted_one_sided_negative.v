From Stdlib Require Import List String ZArith.
From Deduxit.Linear Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ll (atom := string) classical [tens (covar "a") (covar "a"); wn (tens (covar "a") (var "b")); covar "b"].
Proof.
  apply (weighted_refutation (weight_of String.eqb [("a", (-1)%Z)])
    (fun a => existsb (String.eqb a) []) true _).
  vm_compute. reflexivity.
Qed.
