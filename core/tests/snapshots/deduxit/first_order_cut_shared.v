From Stdlib Require Import List String.
From Deduxit.Linear Require Calculus.
From Deduxit.Linear Require Import FirstOrderSoundness.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ll (symbol := string) {| Calculus.mix := false; Calculus.weakening := false; Calculus.cut := true |} [fex (fex (covar "p" [bound 1; bound 0])); awith (fall (var "p" [tapp "a" []; bound 0])) (fall (var "p" [tapp "a" []; bound 0]))].
Proof.
  apply (soundness String.eqb String.eqb_spec {| Calculus.mix := false; Calculus.weakening := false; Calculus.cut := true |}
    (let f'1 := fall (var "p" [bound 1; bound 0]) in
     let f'2 := fex (covar "p" [bound 1; bound 0]) in
     let f'3 := fall (var "p" [tapp "a" []; bound 0]) in
     (Cut (fall f'1) (Forall f'1 0 (Forall (var "p" [eigen 0; bound 0]) 1 (Exists f'2 (eigen 0) (Exists (covar "p" [eigen 0; bound 0]) (eigen 1) (Ax "p" [eigen 0; eigen 1]))))) (With f'3 f'3 (Forall (var "p" [tapp "a" []; bound 0]) 2 (Exists f'2 (tapp "a" []) (Exists (covar "p" [tapp "a" []; bound 0]) (eigen 2) (Ax "p" [tapp "a" []; eigen 2])))) (Forall (var "p" [tapp "a" []; bound 0]) 3 (Exists f'2 (tapp "a" []) (Exists (covar "p" [tapp "a" []; bound 0]) (eigen 3) (Ax "p" [tapp "a" []; eigen 3])))))))).
  vm_compute. reflexivity.
Qed.
