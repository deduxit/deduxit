From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ll (atom := string) classical [var "a"; wn (tens (covar "a") (parr (var "a") (var "a"))); wn (tens (tens (covar "a") (covar "c")) (var "b")); covar "b"].
Proof.
  apply (weighted_refutation (weight_of String.eqb [("a", 1%Z); ("b", (-1)%Z)])
    (fun a => existsb (String.eqb a) ["c"; "b"]) false _).
  vm_compute. reflexivity.
Qed.
