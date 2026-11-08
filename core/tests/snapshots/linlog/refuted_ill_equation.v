From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ill (atom := string) intuitionistic [ilmap (ivar "A") (ivar "B")] (ivar "A").
Proof.
  apply (iequation_refutation _).
  vm_compute. reflexivity.
Qed.
