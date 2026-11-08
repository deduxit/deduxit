From Stdlib Require Import List String ZArith.
From Linlog Require Import Weights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ill (atom := string) intuitionistic [ioc (ivar "d"); ioc (ilmap (itens (itens (ivar "c") (ivar "d")) (ivar "d")) ione)] (itens (ivar "c") (ivar "c")).
Proof.
  apply (iweighted_refutation (weight_of String.eqb [("c", 1%Z)])
    (fun a => existsb (String.eqb a) []) true _).
  vm_compute. reflexivity.
Qed.
