From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ill (atom := string) intuitionistic [iplus (ivar "A") (ivar "B")] (ivar "A").
Proof.
  apply (iclassical_refutation _ (fun a => existsb (String.eqb a) ["B"])).
  vm_compute. reflexivity.
Qed.
