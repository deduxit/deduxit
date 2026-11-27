From Stdlib Require Import List String.
From Deduxit.Intuitionistic Require Import FirstOrderCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ~ lj (symbol := string) {| Calculus.explosion := true; Calculus.cut := true |} [fall (var "=" [bound 0; bound 0])] (fex (neg (var "=" [bound 0; (tapp "a" [])]))).
Proof.
  apply (fo_kripke_refutation String.eqb String.eqb_spec _
    {| later := [[1]; []];
       sizes := [1; 2];
       tables := [[("a", 0, [0])]; [("a", 0, [0])]];
       facts := [[("=", [0; 0])]; [("=", [0; 0]); ("=", [1; 1])]];
       absurd := [] |} 0).
  vm_compute. reflexivity.
Qed.
