From Stdlib Require Import List String.
From Deduxit.Linear Require Import Soundness.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ll (atom := string) classical [awith (awith (awith (parr (parr (covar "A") (covar "B")) (tens (var "A") (var "B"))) one) (awith (parr bot one) top)) (awith (awith (aplus one zero) (aplus zero one)) (awith (awith (oc one) (wn one)) (awith (parr (wn (var "A")) one) (parr (wn (covar "A")) (tens (var "A") (var "A"))))))].
Proof.
  apply (soundness_classical String.eqb String.eqb_spec
    (let f'1 := parr (covar "A") (covar "B") in
     let f'2 := tens (var "A") (var "B") in
     let f'3 := parr f'1 f'2 in
     let f'4 := awith f'3 one in
     let f'5 := parr bot one in
     let f'6 := awith f'5 top in
     let f'7 := aplus one zero in
     let f'8 := aplus zero one in
     let f'9 := awith f'7 f'8 in
     let f'10 := oc one in
     let f'11 := wn one in
     let f'12 := awith f'10 f'11 in
     let f'13 := wn (var "A") in
     let f'14 := parr f'13 one in
     let f'15 := wn (covar "A") in
     let f'16 := tens (var "A") (var "A") in
     let f'17 := parr f'15 f'16 in
     let f'18 := awith f'14 f'17 in
     let f'19 := awith f'12 f'18 in
     (With (awith f'4 f'6) (awith f'9 f'19) (With f'4 f'6 (With f'3 one (Par f'1 f'2 (Par (covar "A") (covar "B") (Tensor (var "A") (var "B") (Ax "A") (Ax "B")))) One) (With f'5 top (Par bot one (Bot One)) Top)) (With f'9 f'19 (With f'7 f'8 (Plus one zero Left One) (Plus zero one Right One)) (With f'12 f'18 (With f'10 f'11 (Bang one One) (Quest one (Copy one One))) (With f'14 f'17 (Par f'13 one (Quest (var "A") One)) (Par f'15 f'16 (Quest (covar "A") (Tensor (var "A") (var "A") (Copy (covar "A") (Ax "A")) (Copy (covar "A") (Ax "A"))))))))))).
  vm_compute. reflexivity.
Qed.
