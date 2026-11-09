From Stdlib Require Import Bool List String ZArith.
From Deduxit.Linear Require Calculus Intuitionistic TermsCheck.
From Deduxit.Linear Require Import FirstOrderCalculus FirstOrderIntuitionistic FirstOrderWeights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ill (symbol := string) Intuitionistic.intuitionistic [ivar "p" [tapp "a" []]; ioc (iex (ivar "p" [bound 0]))] ione.
Proof.
  apply (FirstOrderWeights.iweighted_refutation (fun p => existsb (String.eqb p) [])
    (fun p _ => if String.eqb p "p" then (-1)%Z else 0%Z)
    (fun p => if String.eqb p "p" then (-1)%Z else 0%Z) _).
  vm_compute. reflexivity.
Qed.
