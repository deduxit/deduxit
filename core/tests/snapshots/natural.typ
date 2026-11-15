#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#prooftree(
  rule(
    name: $→ upright(I)^("3")$,
    rule(
      name: $∨ upright(E)^("1,2")$,
      $[a ∨ b]^(3)$,
      rule(
        name: $∨ upright(I)_2$,
        $[a]^(1)$,
        $b ∨ a$,
      ),
      rule(
        name: $∨ upright(I)_1$,
        $[b]^(2)$,
        $b ∨ a$,
      ),
      $b ∨ a$,
    ),
    $a ∨ b → b ∨ a$,
  ),
)
