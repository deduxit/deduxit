# Rules as data: the rule language, the generic derivation and its checker

The design of step 60 (`plan/60-rule-schemas.md`), written by session 60a
on 2026-11-15 before the code. It fixes what every later method builds
on: natural deduction (62), tableaux (56a's and 68's), the labelled modal
calculi (66) and the workspace graph (63g). Each decision has its reason
and the alternatives weighed. Sources are in the last section; "(note)"
marks what `plan/notes/research/next/custom-rules.md` read on
2026-11-08, "(read 60a)" what this session's source check opened, and
"(memory)" what is cited from memory and was not opened again.

The module is `deduxit::rules` (`core/src/rules/`), behind the feature
`serialize`: a calculus is data and enters as a document, so without
serde there is nothing to check against.

## 1. The three values

| value | what it is | JSON |
|---|---|---|
| `Calculus` | a rule file: sorts, connectives, the shape of a sequent, the schematic variables, the rules | wire level 15, strict (an unknown key is refused) |
| `Derivation` | terms, nodes that name their rules, scopes | wire level 15, a data form |
| the verdict | `Derivation::check(&calculus, &limits, stop)`: `Ok(())` reads "valid in the calculus given" | none |

Both values are immutable once made and are plain data with a JSON
form, so the workspace graph stores them as they are. A derivation
names its calculus (`"calculus": "LK"`) and holds every name as text; it
is read without the calculus and resolved against it by the check. The
alternative, a derivation that embeds its calculus, repeats the rule
file in every stored proof and gives the graph nothing to share.

## 2. The rule language

### 2.1 Terms: one many-sorted first-order signature

Everything a sequent holds is a term over a signature the calculus
declares:

- **sorts**: `formula` always exists; a calculus adds more (`label`,
  `judgement`, `signed`);
- **connectives**: a name, an arity and, where not every position is a
  formula, the sorts of its arguments and its result. `∧` is
  `{"name": "∧", "arity": 2}`; a labelled formula `x : A` is the
  connective `{"name": ":", "arity": 2, "sorts": ["label", "formula",
  "judgement"]}`, a relational atom `x R y` is `{"name": "R", "arity": 2,
  "sorts": ["label", "label", "judgement"]}`, a signed formula `T A` is
  `{"name": "T", "arity": 1, "sorts": ["formula", "signed"]}`;
- **atoms**: the names a derivation uses that the calculus does not
  declare, each of one sort: propositional atoms (`formula`), labels
  (`label`). Only a derivation has them.

Why one mechanism. D28 asks for "labels and relational atoms as a sort".
A fixed built-in set (formula, labelled formula, relational atom) would
serve Negri's calculi and nothing else: signed tableaux (Smullyan's `T`
and `F`), prefixed tableaux, multimodal relations `R_a` and agents each
need one more special case. A many-sorted signature is what the tools
built for this use: MetTeL2 declares sorts and gives every connective
its sorts; Jape declares syntactic classes; LF and Isabelle's Pure go
further (dependent or simple types with binders), which is the logical
framework the research note rejected as a different project. First-order
many-sorted terms without binders are the smallest language in which
all four methods of the plan are data. A label atom is a name, so a
modal method names its labels after the worlds of a `kripke::Model`
(`w0`, `w1`, …) and a relational atom `R(w0, w1)` names an arrow.

Print forms (`symbol`, `fixity`, `precedence`, `associativity`) are
declared with a connective and are all a printer and a later text
syntax need. The JSON does not depend on them: a term is written
structurally.

Where quantifiers go (D17): a connective gains `binds` (the sort of the
variable it binds), the term arena a bound-variable node (de Bruijn),
the pattern language a substitution `A[t/x]`. Because a node stores its
sequent (3.1), a witness is found by matching the premise, so the
derivation's nodes gain nothing. The freshness condition (2.4) is
already the eigenvariable condition.

### 2.2 The sequent: zones, each with a discipline

`"sequent"` is a list of zones in the order they are written. A zone
has a name, the sort of its members (`formula` by default), a
`discipline` (`set`, `multiset` or `sequence`), an optional `mark`
written before it (`⊢`), and optional bounds `least` and `most` on its
size.

| calculus | zones |
|---|---|
| LK | `left` multiset; `right` multiset, mark `⊢` |
| LJ | the same with `most: 1` on `right`; minimal logic `least: 1, most: 1` |
| one-sided linear logic | one zone, multiset, mark `⊢` |
| a Lambek calculus | `left` sequence; `right` with `least: 1, most: 1` |
| natural deduction | one zone `formula` with `least: 1, most: 1` |
| a signed tableau | one zone `branch` of sort `signed`, set |
| G3K | `left` and `right` of sort `judgement`, multiset |
| a dyadic or focused calculus | three or more zones |

Why zones and not "left and right". Sequoia lets a calculus have
several contexts on each side; dyadic and focused calculi have three or
four; a one-sided calculus and natural deduction have one. A list of
zones covers them with no special case, and a judgement of natural
deduction is then a sequent whose one zone holds one formula, so one
node type serves both.

Why bounds on a zone. "At most one formula right of `⊢`" is how LJ is
defined from LK (Gentzen), and `ordinary::Derivation::check` enforces
exactly that count per inference, with "exactly one" for minimal logic.
Stated as a bound of the zone, LJ's rule file is LK's with the bound
and the rules that differ, and the cross-check agrees with the ordinary
checker inference by inference.

A discipline says what a context is: a `set` has no multiplicity and no
order (contraction and exchange are built in), a `multiset` no order, a
`sequence` both. Weakening is never built in: a rule that allows a
context writes one.

### 2.3 Variables are declared; rules are premises and a conclusion

```json
"variables": {"formula": ["A", "B", "C"], "label": ["x", "y"]},
"contexts": ["Γ", "Δ", "Γ′", "Δ′"],
"rules": [
  {"name": "∧R",
   "premises": [{"left": ["Γ"], "right": ["A", "Δ"]},
                {"left": ["Γ"], "right": ["B", "Δ"]}],
   "conclusion": {"left": ["Γ"], "right": [["∧", "A", "B"], "Δ"]}}
]
```

- A sequent pattern is an object keyed by zone name (an absent zone is
  empty). A zone's entries are term patterns and context variables.
- A term pattern is a string (a declared variable or a nullary
  connective) or an array `[connective, pattern, …]`.
- A name that is neither declared nor a connective is refused, so a
  typo cannot turn `⊥` into a variable that matches anything, or a
  context into a single formula. This is Jape's way (classes of
  identifiers declared once) and the Incredible Proof Machine's (`free`
  lists a rule's variables); the alternatives were a convention on
  letters (Greek for contexts, as papers do: silent when mistyped) and
  a marker on every use (`{"context": "Γ"}`: noise in every rule).
- Rule names are unique (as in Isabelle, Jape, the Proof Machine and
  Sequoia). Where one textbook name covers two schemas, the file has
  two rules: LK's `∧R` shares its context and `∧R split` splits it.
  The alternative, several schemas under one name, would add a concept
  to the language for the sake of one cross-check.

JSON first, a text syntax later: schematic formulas are structured
arrays, not strings, so reading a rule file needs no parser and the
`parse` feature stays independent. `["∧", "A", "B"]` is the S-expression
idiom JSON tools use for expressions. The reader caps the nesting of a
pattern (`calculus::DEPTH`, 64), so no deserializer recurses deeper
whatever it is; patterns are then kept in a flat arena, operands before
their parents. A text syntax (`Γ ⊢ A ∧ B, Δ`) is a later reader that
produces the same document from the declared print forms.

### 2.4 The fixed menu of side conditions

1. **A context under a connective**, `["!", "Γ"]` in a zone: the
   members are `!B` for every `B` of `Γ`. It covers the prompt's "a
   context restricted to formulas of one connective" (promotion: `?Γ`
   in premise and conclusion) and, since `Γ` is the multiset of the
   bodies, also the modal rule `Γ ⊢ A / □Γ ⊢ □A`, which a restriction
   alone cannot state. Papers write it this way (`!Γ`). Isabelle's
   `Sequents/ILL` cannot: its contexts are plain list variables, so it
   states promotion through an auxiliary judgement `promaux` that moves
   the `!` formulas over one at a time (read 60a). A mapped context is
   the one addition that lets the rule stand as it is written.
2. **Freshness**, `"fresh": ["y"]`: the variable is matched by an atom
   that occurs neither in the conclusion nor in a hypothesis in force
   at the node (3.3). In a sequent calculus this is the eigenvariable
   condition (Negri's `R□`: the label "must not occur in the
   conclusion"); in natural deduction it is the condition of forall x:
   Calgary's `∀I`, "the name must not occur in a premise or an
   undischarged assumption" (read 60a), the hypotheses in force being
   the undischarged ones. Gentzen's condition counts only the
   assumptions the premise depends on (Carnap checks those alone, its
   manual says); ours is stricter and equal up to renaming, and the
   dependencies can be computed later from the same data. Two fresh
   variables of a rule are different atoms.
3. **Atomicity**, `"atomic": ["A"]`: the variable is matched by an
   atom. (60a's addition to the prompt's list; decided here.) G3's
   axiom `P, Γ ⊢ Δ, P` and a tableau's closure "by complementary
   literals" both restrict a schematic formula to atoms, and nothing
   else in the menu can say so.
4. **Labels and relational atoms** are terms of their sorts (2.1), not
   a condition.

Nothing else. A condition outside the menu (a label different from
another, a formula's polarity) is a design decision of the step that
needs it, added as a key of the rule at a new wire level.

### 2.5 Discharge

A premise may name hypotheses it discharges:

```json
{"name": "→I",
 "premises": [{"formula": ["B"], "discharges": [{"formula": ["A"]}]}],
 "conclusion": {"formula": [["→", "A", "B"]]}}
```

`discharges` is a list of sequent patterns without contexts. This is
the Proof Machine's "local hypothesis" port of a rule and Prawitz's
`[A]` over a premise. `∨E` has two such premises, a general elimination
rule one premise with two hypotheses.

### 2.6 What is refused when a rule file is read

Beyond names and sorts: a context variable used in zones of two
disciplines or two sorts, or twice in one zone; a variable of a
discharged hypothesis or of `fresh`/`atomic` that no premise or
conclusion binds; and a rule whose contexts cannot be told apart
(4.2). The file is the trusted base of every verdict, so what the
checker could only guess at is refused before any derivation is read.

## 3. The derivation

### 3.1 Nodes store their sequents

A derivation is an arena: hash-consed terms (equal terms are one id, as
`ordinary::Formulas`), then nodes, premises before conclusions. A node
has its sequent (per zone the ids of its members), what justifies it,
its premises, its scope, and the positions the check needs (4.1). All
of it lies in flat vectors of the derivation; a node is a few indices.

A node is justified by a rule (by name), or is a **hypothesis**, or is
**open** (a goal of a proof in progress, which the checker refuses, as
`ordinary::Rule::Open`; step 61's session needs no new node kind).

Why stored sequents. The linear `Proof` stores none and its checker
derives them, which is the fast design for one compiled calculus. Under
rules given as data, deriving a sequent means instantiating a schema,
which needs every variable the conclusion does not determine (a cut
formula, a weakened formula, a witness) as an explicit argument, and a
fault then shows only where a derived sequent stops matching, far from
its cause. With stored sequents each node is checked alone against its
premises, "trusting nothing of whatever made the derivation" as the
ordinary checker does; a drawing needs no check; and a stored value is
self-describing for the graph. The price is size: a tableau of `n`
lines on one branch stores `n²/2` ids. That is 4 bytes an id against a
bound the reader enforces, and the same trade `ordinary::Derivation`
made.

The derivation is a DAG: a node may be the premise of several nodes (a
Fitch line cited twice) or of none (an unused line). Every node is
checked, used or not.

### 3.2 What each method needs, and where it is

| need (D28, 46a) | in the type |
|---|---|
| hypothesis nodes with an identity | a node justified as a hypothesis; its id is its identity |
| a discharge link from the closing rule to its leaves | the rule node's discharging premise lies in a scope; the hypothesis nodes of that scope are the leaves (`Derivation::hypotheses(scope)`) |
| a scope tree (Fitch) | `scopes`: each scope names its parent; each node its scope |
| closure of a branch by complementary literals | a rule without premises whose conclusion writes both formulas, with `atomic`; the node's positions cite the two lines |
| labels and relational atoms as a sort | sorts and atoms of the term arena |

### 3.3 Scopes

Scope 0 is the root; every other scope names a parent with a smaller
index. A node lies in one scope. The rules, for a node `n` and a
premise `p` of it:

- a premise that discharges nothing lies in `n`'s scope or an ancestor
  of it (a line may cite what is still in force);
- a premise of a rule that discharges lies there too (it then
  discharges nothing: Gentzen's `→I` over a formula that does not use
  the hypothesis), or in a scope `c` whose parent is `n`'s scope or an
  ancestor of it, and then every hypothesis node of `c` is an instance
  of one of the premise's `discharges` patterns.

A hypothesis in the root scope is an assumption of the whole
derivation, and the root lies in the root scope: the derivation shows
the root's sequent from the assumptions (`Derivation::assumptions()`).
A last node inside a scope would rest on hypotheses that are neither
discharged nor assumptions (the review's first finding), so the check
refuses it.

Why this is sound: by induction on the node order, every node's sequent
follows from the hypotheses of its scope and of that scope's ancestors.
A hypothesis is one of them. A rule's plain premises hold under a
subset of the node's hypotheses; a discharging premise in `c` holds
under those of `c`'s parent chain and `c`'s own, which all match what
the rule discharges, so the conclusion holds under the parent chain
alone, which is part of `n`'s.

One model for Gentzen's trees and Fitch's lists. In a Gentzen tree the
subderivation over a discharging premise is a scope, its discharged
leaves the hypothesis nodes of it, and leaves that stay open are nodes
of the root scope inside the same subtree. In a Fitch proof a subproof
is a scope, its assumption line a hypothesis node, a citation of the
subproof a discharging premise on its last line. The same check serves
both; the drawing differs (62). It is the Proof Machine's proof graph
with local hypotheses, whose equivalence with natural deduction trees
is formalised in Isabelle's AFP (note). The alternative, an explicit
list of discharged leaves on the rule node, would state the same thing
twice (the list and the scope) and could disagree with itself.

What the scope check does not enforce, on purpose: that a Fitch
subproof's lines are contiguous, that its assumption is its first line
and that a citation names its last line. Those are layout rules of one
presentation; 62's Fitch view checks them where it draws.

Linear natural deduction, not planned, would count how often a
hypothesis node is a premise; the identity is there.

## 4. The checker

### 4.1 Matching without search

Matching a multiset against a pattern with several formulas and
contexts is associative-commutative matching, NP-complete in general
(Benanav, Kapur and Narendran 1987, note). The checker therefore never
searches: a node says where the rule applies, as the linear `Step` and
`ordinary::Inference::principal` do.

The members a rule writes out are met in one order: the conclusion's
zones in order, then each premise's. A written member is found

- as the zone's only member, where it is the zone's only entry;
- by its place in a `sequence` zone with at most one context, counting
  from the ends;
- **by position** in the conclusion otherwise: the node says which
  members of its conclusion the rule acts on, even where their value
  would find them (a drawing marks them, and `⊥L` names its `⊥` as
  `ordinary`'s `principal` does);
- **by value** in a premise's set or multiset zone, where every
  variable of the member is bound by then: the instance is looked up in
  the term arena (no term is built) and one occurrence of it is taken;
- **by position** in a premise otherwise (a cut formula, a relational
  atom with a new label, any member of a sequence with two contexts).

The node's `at` list gives the positions in that order.

Which members take a position is fixed by the rule, so a front end asks
the calculus (`Calculus::located(rule)`). For LK: one position for a
rule with a principal formula (`ordinary`'s `principal`), none for the
axiom `A ⊢ A`, none for LJ's cut (the cut formula is the first
premise's one formula right of `⊢`). For natural deduction: none at
all. For a tableau closure: the positions of `T A` and `F A`, the two
lines 56a's `End::Closed` cites.

Then term matching is first-order and linear in the pattern, with an
explicit stack. The alternatives: positions for every written member
(redundant, and every hand-written proof carries them), explicit
bindings of the variables the conclusion leaves open (a second
mechanism, and a position in a premise gives the same), and search over
candidates with a work bound (a checker that may answer "unknown").

### 4.2 Contexts by discipline

What the written members leave of each zone is the rest. Each zone of
each sequent gives one equation between its rest and its contexts.

- **multiset**: rest = the sum of the zone's contexts. A zone with one
  unbound context defines it; with none it is compared. The order in
  which the zones are solved is found once per rule when the file is
  read; a rule in which every remaining zone has two unbound contexts
  is refused ("its contexts cannot be told apart"). Every usual rule
  passes: a split rule's contexts each stand alone in a premise.
- **sequence**: the written members cut the zone into segments, each a
  concatenation of contexts, solved the same way from both ends.
- **set**: a context is at most the zone wherever it occurs, so its
  largest value is the intersection of those zones; the rule applies
  exactly if, with every context at its largest, each zone is covered
  by its written members and contexts. No order and no positions for
  contexts are needed, and `Γ, A` matches whether or not `A` is in `Γ`,
  as sets are read.
- a context under a connective is mapped on the way in and out.

Given the positions, the check accepts a rule node exactly when it is
an instance of the rule. Its cost is linear in the node's sequents and
its rule, with a sort for each multiset comparison.

### 4.3 Bounds, as the linear checker's

- a size estimate before anything is allocated, refused past
  `limits.memory_bytes` (`Refusal::Memory` with what was needed);
- no recursion: terms, patterns, scopes and nodes are walked in arena
  order or with a stack of the checker's own;
- `limits.work`: a node, a member handled, a term visited by a
  freshness walk and a scope stepped through are each a unit, counted
  in a saturating `u64`; past the bound `Refusal::Work`;
- the stop is asked at every node and every `POLL` units within one;
- every index of a derivation is checked when the derivation is built
  or read, so the check indexes without a panic; positions are checked
  against their zone as they are read; no counter of the check can
  wrap, since each is bounded by the size of the document in memory.

A refusal is never a verdict. The verdict itself is `Ok(())` or
`Error::InvalidDerivation` naming the first node that fails and why.
No certificate (the user gives no semantics) and no search (D28).

## 5. The calculi shipped, and the cross-checks

- `rules::LK`, `rules::LJ`, `rules::LM` are the rule files' text. They
  are the rules of `ordinary::Derivation::check` (`core-ordinary.md`):
  Gentzen's with explicit weakening and contraction, each binary rule
  in its sharing form (`∧R`) and its splitting form (`∧R split`), `¬`
  in LK's forms and, in LJ and LM, also as `A → ⊥` (`¬R ⊥`, `¬L ⊥`),
  cut in LJ and LM only (`cut` split as Gentzen's, `cut shared` for a
  conclusion with nothing right of `⊢`), no `⊥L` in LM.
- `rules::Derivation::from_ordinary` writes an ordinary derivation in
  the generic form; the test compares both checkers on derivations from
  every method and on each of them broken once per inference.
- A signed tableau as a rule file (56a's rules) and `Tableau`'s trees
  written in the generic form: 60b.
- The linear calculus as a file, compared with the linear checker on a
  sample: 60b.

## 6. What it copies (for step 67)

The hash-consed arena with a chain per hash (`ordinary::Formulas`'s
idea, n-ary), the poll cadence and the saturating work count of the
ordinary checkers, the wire reader's pattern (`wire::Readable`, a form
struct, `within`), and the shape "premises before conclusions, flat
vectors" of `linear::Proof`. Nothing of `linear` is used.

## 7. Sources

Opened by this session's source check on 2026-11-15 (a research
sub-agent; most pages came through a summarising fetcher, so the
wording quoted is close, not verbatim):

- **The Incredible Proof Machine**, `logics/predicate.yaml` and
  `SPEC.md` of github.com/nomeata/incredible: a rule has `id`, `free`
  (its schematic variables, per rule), `ports` each of `type`
  `assumption`, `conclusion` or `local hypothesis`, the last with
  `consumedBy` naming the port that discharges it; propositions are
  strings a parser reads. (2.3, 2.5, 3.3)
- **Jape**, `distrib/examples/sequent_calculus/` of
  github.com/RBornat/jape: `CLASS BAG Γ ∆`, `CLASS FORMULA A B C …`,
  `SEQUENT IS BAG ⊢ BAG` (and `BAG ⊢ FORMULA`), `RULE "⊢∧" FROM Γ ⊢ A,∆
  AND Γ ⊢ B,∆ INFER Γ ⊢ A∧B,∆`, `RULE "⊢∀"(OBJECT m) WHERE FRESH m …`.
  (2.2, 2.3, 2.4)
- **MetTeL2**, Tishkovsky, Schmidt and Khodadadi, PAAR 2012
  (easychair.org/publications/paper/XhJM): the specification declares
  sorts (`sort formula, element, list;`) and gives every connective its
  sorts; a tableau rule is premises `/` conclusions, branches separated
  by `$|`. (2.1)
- **Negri's G3K**, through Bilotta's slides "Sara Negri's labelled
  sequent calculi for modal logics" (math.lmu.de/~schwicht/PC/25/…) and
  arXiv 2203.14147 §2.1: formulas `x : A` and `xRy`, sequents of
  multisets of them, the initial sequent on atoms, `L□` repeating its
  principal formula, `R□` with a label that "must not occur in the
  conclusion", frame conditions as rules on relational atoms (Ref,
  Trans, Sym, Eucl, Ser). Negri's paper (J. Philos. Logic 34, 2005) and
  Negri and von Plato's book were not opened. (2.1, 2.4, the test's
  calculus)
- **Isabelle's `Sequents`**, `LK0.thy` and `ILL.thy` at
  cl.cam.ac.uk/research/hvg/Isabelle/dist/library/Misc/Sequents: context
  variables `$H`, `$G`; `conjR: ⟦$H ⊢ $E, P, $F; $H ⊢ $E, Q, $F⟧ ⟹
  $H ⊢ $E, P ∧ Q, $F`; promotion through `promaux`; context splitting
  as a judgement `$F, $J :=: $G`. (2.4)
- **G1 and G3**, through Iemhoff and Jalali's lecture notes (arXiv
  2305.10888 §6.5–6.6), which cite Troelstra and Schwichtenberg 2000:
  G3c has no structural rules, weakening and contraction are admissible
  in it, G1c and G3c prove the same sequents. The book itself was not
  opened; that G1's two-premise rules split the context and a sharing
  style gives an equivalent system rests on a search snippet of
  Brünnler (arXiv 1203.4754). So this note calls the shipped rules
  "the ordinary checker's" and claims no more. (5)
- **forall x: Calgary** (forallx.openlogicproject.org, chapters 17, 36,
  37) and Carnap's manual for it: a cited line must not lie in a
  subproof closed before the citing line; a subproof is cited as
  `m–n`; `∀I`'s name "must not occur in a premise or an undischarged
  assumption"; Carnap checks only the assumptions used. (2.4, 3.3)

From `custom-rules.md` (read 2026-11-08, not opened again): AC
matching's NP-completeness (Benanav, Kapur and Narendran 1987), the
Proof Machine's formalisation in the AFP, Sequoia's several contexts,
LF and Pure as the frameworks not to copy.

From memory, not opened: Gentzen's definition of LJ from LK by the
bound on the succedent; Prawitz's `[A]` over a discharging premise;
Smullyan's signed formulas; the S-expression idiom of JSON expression
languages.
