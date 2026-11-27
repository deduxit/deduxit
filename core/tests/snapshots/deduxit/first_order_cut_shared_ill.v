From Stdlib Require Import List String.
From Deduxit.Linear Require Intuitionistic.
From Deduxit.Linear Require Import FirstOrderIntuitionisticCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ill (symbol := string) {| Intuitionistic.iweakening := false; Intuitionistic.icut := true |} [iall (iall (ivar "p" [bound 1; bound 0]))] (iwith (iall (ivar "p" [tapp "a" []; bound 0])) (iall (ivar "p" [tapp "a" []; bound 0]))).
Proof.
  apply (soundness_ill String.eqb String.eqb_spec {| Intuitionistic.iweakening := false; Intuitionistic.icut := true |}
    (let f'1 := iall (ivar "p" [bound 1; bound 0]) in
     let f'2 := iall (ivar "p" [tapp "a" []; bound 0]) in
     (ICut (iall f'1) (IAllR f'1 0 (IAllR (ivar "p" [eigen 0; bound 0]) 1 (IAllL f'1 (eigen 0) (IAllL (ivar "p" [eigen 0; bound 0]) (eigen 1) (IAx "p" [eigen 0; eigen 1]))))) (IWithR f'2 f'2 (IAllR (ivar "p" [tapp "a" []; bound 0]) 2 (IAllL f'1 (tapp "a" []) (IAllL (ivar "p" [tapp "a" []; bound 0]) (eigen 2) (IAx "p" [tapp "a" []; eigen 2])))) (IAllR (ivar "p" [tapp "a" []; bound 0]) 3 (IAllL f'1 (tapp "a" []) (IAllL (ivar "p" [tapp "a" []; bound 0]) (eigen 3) (IAx "p" [tapp "a" []; eigen 3])))))))).
  vm_compute. reflexivity.
Qed.
