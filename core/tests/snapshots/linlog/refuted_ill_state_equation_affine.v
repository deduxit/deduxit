From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ill (atom := string) affine [ioc (ilmap (ivar "a") (itens (ivar "a") (ivar "c"))); ioc (ilmap (ivar "a") (ivar "b")); ivar "a"] (itens (ivar "a") (ivar "b")).
Proof.
  apply (iaffine_refutation (weight_of String.eqb [("a", 1%Z); ("b", 1%Z)])
    (fun a => existsb (String.eqb a) []) true _).
  vm_compute. reflexivity.
Qed.
