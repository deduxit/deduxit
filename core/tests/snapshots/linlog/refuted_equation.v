From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ll (atom := string) classical [tens (var "A") (covar "A")].
Proof.
  apply (equation_refutation _).
  vm_compute. reflexivity.
Qed.
