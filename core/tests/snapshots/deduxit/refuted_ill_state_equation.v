From Stdlib Require Import List String ZArith.
From Deduxit.Linear Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ill (atom := string) intuitionistic [ivar "a"; ilmap (ivar "a") (ivar "b"); ioc (ilmap (ivar "b") (itens (ivar "b") (ivar "c"))); ioc (ilmap (ivar "b") (ivar "d"))] (itens (ivar "b") (ivar "d")).
Proof.
  apply (iweighted_refutation (weight_of String.eqb [("a", 2%Z); ("b", 1%Z); ("d", 1%Z)])
    (fun a => existsb (String.eqb a) []) true _).
  vm_compute. reflexivity.
Qed.
