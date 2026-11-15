#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#prooftree(
  rule(
    name: $upright(T)∨$,
    rule(
      name: $upright(F)∨$,
      rule(name: $"×"$, $upright("T")  (a ∨ b), upright("F")  (b ∨ a), upright("T")  a, upright("F")  b, upright("F")  a$),
      $upright("T")  (a ∨ b), upright("F")  (b ∨ a), upright("T")  a$,
    ),
    rule(
      name: $upright(F)∨$,
      rule(name: $"×"$, $upright("T")  (a ∨ b), upright("F")  (b ∨ a), upright("T")  b, upright("F")  b, upright("F")  a$),
      $upright("T")  (a ∨ b), upright("F")  (b ∨ a), upright("T")  b$,
    ),
    $upright("T")  (a ∨ b), upright("F")  (b ∨ a)$,
  ),
)
