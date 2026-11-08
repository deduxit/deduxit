#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#stack(spacing: 0.8em, align(center, prooftree(
  rule(
    name: $"cut"$,
    rule(
      name: $! upright(R)$,
      rule(
        name: $! upright(L)$,
        rule(name: $"ax"$, $a ⊢ a$),
        $!a ⊢ a$,
      ),
      $!a ⊢ !a$,
    ),
    rule(
      name: $! upright(c)$,
      rule(
        name: $! upright(L)$,
        rule(
          name: $! upright(L)$,
          rule(
            name: $⊗ upright(R)$,
            rule(name: $"ax"$, $a ⊢ a$),
            rule(name: $"ax"$, $a ⊢ a$),
            $a, a ⊢ a ⊗ a$,
          ),
          $!a, a ⊢ a ⊗ a$,
        ),
        $!a, !a ⊢ a ⊗ a$,
      ),
      $!a ⊢ a ⊗ a$,
    ),
    $!a ⊢ a ⊗ a$,
  ),
)), align(center, $x : !a ⊢ (lambda y:!a. "copy" space y space "as" space u, v space "in" space "derelict"(v) ⊗ "derelict"(u)) space ("promote" space x space "for" space z space "in" space "derelict"(z)) : a ⊗ a$))
