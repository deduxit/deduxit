#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#prooftree(
  rule(
    name: $⊗ upright(E)$,
    rule(name: $"ax"$, $x : a ⊗ !b ⊢ x : a ⊗ !b$),
    rule(
      name: $⊗ upright(I)$,
      rule(
        name: $⊸ upright(E)$,
        rule(name: $"ax"$, $f : b ⊸ c ⊢ f : b ⊸ c$),
        rule(
          name: $! upright(E)$,
          rule(name: $"ax"$, $z : !b ⊢ z : !b$),
          $z : !b ⊢ "derelict"(z) : b$,
        ),
        $f : b ⊸ c, z : !b ⊢ f space "derelict"(z) : c$,
      ),
      rule(name: $"ax"$, $y : a ⊢ y : a$),
      $f : b ⊸ c, y : a, z : !b ⊢ f space "derelict"(z) ⊗ y : c ⊗ a$,
    ),
    $x : a ⊗ !b, f : b ⊸ c ⊢ "let" space x space "be" space y ⊗ z space "in" space f space "derelict"(z) ⊗ y : c ⊗ a$,
  ),
)
