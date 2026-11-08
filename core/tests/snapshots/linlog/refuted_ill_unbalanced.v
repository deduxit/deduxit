From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ill (atom := string) intuitionistic [ivar "A"] (ivar "B").
Proof.
  apply (iunbalanced_refutation String.eqb "A" _).
  vm_compute. reflexivity.
Qed.
