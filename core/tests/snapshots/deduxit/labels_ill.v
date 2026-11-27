From Stdlib Require Import List String.
From Deduxit.Linear Require Import IntuitionisticCheck.
Import ListNotations.
Open Scope string_scope.

Lemma certificate : ill (atom := string) intuitionistic [] (iwith (iwith (iwith (ilmap (ivar "A") (ilmap (ilmap (ivar "A") (ivar "B")) (ivar "B"))) (ilmap (itens (ivar "A") (ivar "B")) (itens (ivar "B") (ivar "A")))) (iwith (ilmap (iwith (ivar "A") (ivar "B")) (ivar "A")) (ilmap (iwith (ivar "A") (ivar "B")) (ivar "B")))) (iwith (iwith (ilmap (iplus (ivar "A") (ivar "B")) (iplus (ivar "B") (ivar "A"))) (ilmap ione ione)) (iwith (iwith (ilmap izero (ivar "A")) itop) (iwith (ilmap (ioc (ivar "A")) (itens (ivar "A") (ivar "A"))) (iwith (ilmap (ioc (ivar "A")) (ioc (ivar "A"))) (ilmap (ioc (ivar "A")) ione)))))).
Proof.
  apply (soundness_ill String.eqb String.eqb_spec intuitionistic
    (let f'1 := ilmap (ivar "A") (ivar "B") in
     let f'2 := ilmap f'1 (ivar "B") in
     let f'3 := ilmap (ivar "A") f'2 in
     let f'4 := itens (ivar "A") (ivar "B") in
     let f'5 := itens (ivar "B") (ivar "A") in
     let f'6 := ilmap f'4 f'5 in
     let f'7 := iwith f'3 f'6 in
     let f'8 := iwith (ivar "A") (ivar "B") in
     let f'9 := ilmap f'8 (ivar "A") in
     let f'10 := ilmap f'8 (ivar "B") in
     let f'11 := iwith f'9 f'10 in
     let f'12 := iplus (ivar "A") (ivar "B") in
     let f'13 := iplus (ivar "B") (ivar "A") in
     let f'14 := ilmap f'12 f'13 in
     let f'15 := ilmap ione ione in
     let f'16 := iwith f'14 f'15 in
     let f'17 := ilmap izero (ivar "A") in
     let f'18 := iwith f'17 itop in
     let f'19 := ioc (ivar "A") in
     let f'20 := itens (ivar "A") (ivar "A") in
     let f'21 := ilmap f'19 f'20 in
     let f'22 := ilmap f'19 f'19 in
     let f'23 := ilmap f'19 ione in
     let f'24 := iwith f'22 f'23 in
     let f'25 := iwith f'21 f'24 in
     let f'26 := iwith f'18 f'25 in
     (IWithR (iwith f'7 f'11) (iwith f'16 f'26) (IWithR f'7 f'11 (IWithR f'3 f'6 (ILmapR (ivar "A") f'2 (ILmapR f'1 (ivar "B") (ILmapL (ivar "A") (ivar "B") (IAx "A") (IAx "B")))) (ILmapR f'4 f'5 (ITensL (ivar "A") (ivar "B") (ITensR (ivar "B") (ivar "A") (IAx "B") (IAx "A"))))) (IWithR f'9 f'10 (ILmapR f'8 (ivar "A") (IWithL1 (ivar "A") (ivar "B") (IAx "A"))) (ILmapR f'8 (ivar "B") (IWithL2 (ivar "A") (ivar "B") (IAx "B"))))) (IWithR f'16 f'26 (IWithR f'14 f'15 (ILmapR f'12 f'13 (IPlusL (ivar "A") (ivar "B") (IPlusR2 (ivar "B") (ivar "A") (IAx "A")) (IPlusR1 (ivar "B") (ivar "A") (IAx "B")))) (ILmapR ione ione (IOneL IOneR))) (IWithR f'18 f'25 (IWithR f'17 itop (ILmapR izero (ivar "A") (IZeroL [] (ivar "A"))) (ITopR [])) (IWithR f'21 f'24 (ILmapR f'19 f'20 (ICo (ivar "A") (ITensR (ivar "A") (ivar "A") (IDe (ivar "A") (IAx "A")) (IDe (ivar "A") (IAx "A"))))) (IWithR f'22 f'23 (ILmapR f'19 f'19 (IOcR (ivar "A") (IDe (ivar "A") (IAx "A")))) (ILmapR f'19 ione (IWk (ivar "A") IOneR))))))))).
  vm_compute. reflexivity.
Qed.
