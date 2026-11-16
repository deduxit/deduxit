#import "@preview/curryst:0.6.0": prooftree, rule
#set page(width: auto, height: auto, margin: 5pt)

#prooftree(
  rule(
    name: $upright(L)◇$,
    rule(
      name: $upright(R)◇$,
      rule(
        name: $upright(R)∧$,
        rule(
          name: $upright(R)□$,
          rule(
            name: $"Trans"$,
            rule(
              name: $upright(L)□$,
              rule(name: $"ax"$, $italic("w0") upright(":")  □p, italic("w0") upright(":")  ◇q, italic("w0") upright("R")  italic("w1"), italic("w1") upright(":")  q, italic("w1") upright("R")  italic("w2"), italic("w2") upright(":")  p, italic("w0") upright("R")  italic("w2") ⊢ italic("w0") upright(":")  ◇(□p ∧ q), italic("w1") upright(":")  □p ∧ q, italic("w1") upright(":")  □p, italic("w2") upright(":")  p$),
              $italic("w0") upright(":")  □p, italic("w0") upright(":")  ◇q, italic("w0") upright("R")  italic("w1"), italic("w1") upright(":")  q, italic("w1") upright("R")  italic("w2"), italic("w0") upright("R")  italic("w2") ⊢ italic("w0") upright(":")  ◇(□p ∧ q), italic("w1") upright(":")  □p ∧ q, italic("w1") upright(":")  □p, italic("w2") upright(":")  p$,
            ),
            $italic("w0") upright(":")  □p, italic("w0") upright(":")  ◇q, italic("w0") upright("R")  italic("w1"), italic("w1") upright(":")  q, italic("w1") upright("R")  italic("w2") ⊢ italic("w0") upright(":")  ◇(□p ∧ q), italic("w1") upright(":")  □p ∧ q, italic("w1") upright(":")  □p, italic("w2") upright(":")  p$,
          ),
          $italic("w0") upright(":")  □p, italic("w0") upright(":")  ◇q, italic("w0") upright("R")  italic("w1"), italic("w1") upright(":")  q ⊢ italic("w0") upright(":")  ◇(□p ∧ q), italic("w1") upright(":")  □p ∧ q, italic("w1") upright(":")  □p$,
        ),
        rule(name: $"ax"$, $italic("w0") upright(":")  □p, italic("w0") upright(":")  ◇q, italic("w0") upright("R")  italic("w1"), italic("w1") upright(":")  q ⊢ italic("w0") upright(":")  ◇(□p ∧ q), italic("w1") upright(":")  q, italic("w1") upright(":")  □p ∧ q$),
        $italic("w0") upright(":")  □p, italic("w0") upright(":")  ◇q, italic("w0") upright("R")  italic("w1"), italic("w1") upright(":")  q ⊢ italic("w0") upright(":")  ◇(□p ∧ q), italic("w1") upright(":")  □p ∧ q$,
      ),
      $italic("w0") upright(":")  □p, italic("w0") upright(":")  ◇q, italic("w0") upright("R")  italic("w1"), italic("w1") upright(":")  q ⊢ italic("w0") upright(":")  ◇(□p ∧ q)$,
    ),
    $italic("w0") upright(":")  □p, italic("w0") upright(":")  ◇q ⊢ italic("w0") upright(":")  ◇(□p ∧ q)$,
  ),
)
