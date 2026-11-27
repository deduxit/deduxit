From Stdlib Require Import List String.
From Deduxit.Linear Require Intuitionistic.
From Deduxit.Linear Require Import FirstOrderIntuitionisticCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ill (symbol := string) {| Intuitionistic.iweakening := false; Intuitionistic.icut := true |} [iall (itens (ivar "p" [bound 0]) (ivar "q" [bound 0]))] (iall (itens (ivar "p" [bound 0]) (ivar "q" [bound 0]))).
Proof.
  apply (soundness_ill String.eqb String.eqb_spec {| Intuitionistic.iweakening := false; Intuitionistic.icut := true |}
    (let f'1 := itens (ivar "p" [bound 0]) (ivar "q" [bound 0]) in
     (IAllR f'1 0 (ICut (itens (ivar "p" [eigen 0]) (ivar "q" [eigen 0])) (IAllL f'1 (eigen 0) (ITensL (ivar "p" [eigen 0]) (ivar "q" [eigen 0]) (ITensR (ivar "p" [eigen 0]) (ivar "q" [eigen 0]) (IAx "p" [eigen 0]) (IAx "q" [eigen 0])))) (ITensL (ivar "p" [eigen 0]) (ivar "q" [eigen 0]) (ITensR (ivar "p" [eigen 0]) (ivar "q" [eigen 0]) (IAx "p" [eigen 0]) (IAx "q" [eigen 0]))))))).
  vm_compute. reflexivity.
Qed.
