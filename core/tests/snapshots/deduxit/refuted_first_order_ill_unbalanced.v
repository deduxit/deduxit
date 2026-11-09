From Stdlib Require Import Bool List String ZArith.
From Deduxit.Linear Require Calculus Intuitionistic TermsCheck.
From Deduxit.Linear Require Import FirstOrderCalculus FirstOrderIntuitionistic FirstOrderWeights.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ ill (symbol := string) Intuitionistic.intuitionistic [ivar "p" [tapp "a" []]; ioc (iall (ilmap (ivar "p" [bound 0]) (itens (ivar "q" [bound 0]) (ivar "q" [bound 0]))))] (ivar "r" [tapp "a" []]).
Proof.
  apply (FirstOrderWeights.iweighted_refutation (fun p => existsb (String.eqb p) ["r"])
    (fun p ts => if String.eqb p "r" && TermsCheck.terms_eqb String.eqb ts [tapp "a" []] then 1%Z else 0%Z)
    (fun _ => 0%Z) _).
  vm_compute. reflexivity.
Qed.
