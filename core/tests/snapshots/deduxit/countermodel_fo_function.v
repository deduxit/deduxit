From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import FirstOrderCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (symbol := string) {| Calculus.explosion := true; Calculus.cut := true |} [] (imp (neg (neg (var "p" [(tapp "f" [(tapp "a" [])])]))) (var "p" [(tapp "f" [(tapp "a" [])])])).
Proof.
  apply (fo_kripke_refutation String.eqb String.eqb_spec _
    {| later := [[1]; []];
       sizes := [1; 2];
       tables := [[("a", 0, [0]); ("f", 1, [0])]; [("a", 0, [0]); ("f", 1, [0; 1])]];
       facts := [[]; [("p", [0])]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
