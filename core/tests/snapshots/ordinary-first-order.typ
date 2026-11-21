#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#prooftree(
  rule(
    name: $forall upright(R)$,
    rule(
      name: $exists upright(L)$,
      rule(
        name: $forall upright(L)$,
        rule(
          name: $exists upright(R)$,
          rule(
            name: $"WL"$,
            rule(name: $"ax"$, $r(b, a) ⊢ r(b, a)$),
            $r(b, a), ∀y. r(b, y) ⊢ r(b, a)$,
          ),
          $r(b, a), ∀y. r(b, y) ⊢ ∃x. r(x, a)$,
        ),
        $∀y. r(b, y) ⊢ ∃x. r(x, a)$,
      ),
      $∃x. ∀y. r(x, y) ⊢ ∃x. r(x, a)$,
    ),
    $∃x. ∀y. r(x, y) ⊢ ∀y. ∃x. r(x, y)$,
  ),
)
