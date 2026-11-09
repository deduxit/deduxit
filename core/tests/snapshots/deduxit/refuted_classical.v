From Stdlib Require Import List String ZArith.
From Deduxit.Linear Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ll (atom := string) classical [awith (covar "A") (covar "B"); var "A"].
Proof.
  apply (classical_refutation _ (fun a => existsb (String.eqb a) ["B"])).
  vm_compute. reflexivity.
Qed.
