From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ll (atom := string) {| mix := false; weakening := true; cut := false |} [awith (covar "A") (covar "B"); var "A"].
Proof.
  apply (classical_refutation _ (fun a => existsb (String.eqb a) ["B"])).
  vm_compute. reflexivity.
Qed.
