# The modal suite: its design (step 65, session 65a)

Written 2026-11-16 by session 65a (Fable 5.2, `high`) from
`plan/65-modal-logics.md`, `plan/notes/research/next/modal-logics.md`
and three research sub-agents' readings of the primary sources that day
(section 14 lists every source with how it was read). Sections 2 and 3
are built in this session; the rest is the design 65b to 65d build.
Where a statement is this session's own argument and not a source's, it
says "(ours)".

## 1. The design in one page

- **The arena** (`deduxit::modal`, built): a hash-consed arena of its
  own beside `ordinary::Formulas`, the ordinary connectives with `□` and
  `◇` both primitive, a two-sided `Sequent` read at one world (local
  consequence), the ordinary text syntax with the prefixes `[]`/`□` and
  `<>`/`◇`, the ordinary JSON form with two more tags.
- **The systems** (`modal::System`, built): a system is the set of frame
  conditions its frames meet (`Condition`: serial, reflexive, symmetric,
  transitive, euclidean), kept closed under what follows on every frame.
  The 32 sets close to the fifteen logics of the modal cube, so equal
  values are equal logics. All fifteen are values; K, D, T, B, K4, S4
  and S5 are the ones the step must decide.
- **The model** is step 55's `kripke::Model` unchanged. For the modal
  suite a world's arrows are the accessibility relation itself, nothing
  closed for the reader: the check tests the system's conditions on the
  arrows as written and names the first that fails with the worlds that
  show it.
- **The search** is one engine for every system: backward proof search
  in the labelled sequent calculus G3K with the frame rules of the
  system's conditions (Negri 2005), run as a tableau over a graph of
  worlds whose relation is kept closed under the frame rules. It ends
  without a loop check in twelve systems and needs one, blocking a world
  by an ancestor whose formulas include its own, in K4, KD4 and S4.
- **The proof** is the derivation the search made, a `rules::Derivation`
  in the system's labelled rule file, checked by the generic checker of
  step 60 (`rules::Derivation::check`). No new checker is trusted for
  "valid". A draft of the rule file and a derivation of `□p → □□p`
  already pass the checker on `main`, and a label that is not fresh is
  refused (section 7).
- **The countermodel** is the open saturated branch, read as a
  `kripke::Model` and checked by the modal model check (frame
  conditions, then truth at the model's world). The model check is the
  one new trusted piece; 65b's panel reviews it.
- **Trust**: the search is untrusted. "Valid" rests on the rule file and
  the generic checker, "not valid" on the model check. A search that
  goes wrong can only fail to give evidence, which is an error of kind
  `defect`, never a wrong verdict.

## 2. The arena and the sequent (built)

`core/src/modal/mod.rs`, `parse.rs`, `core/src/serialize/modal.rs`.

- **An arena of its own** (fixed by the prompt): `modal::Formulas`,
  `Node`, `NodeId`, a copy of `ordinary::Formulas` with
  `Node::Box(NodeId)` and `Node::Diamond(NodeId)`. Equal formulas are
  one id, every operand precedes its formula, an id of another arena is
  refused, a node is 12 bytes.
  - *Why both `□` and `◇` are nodes*: a formula prints as it was
    written, the labelled calculus has rules for both, and a student's
    `◇p` should not come back as `¬□¬p`. *Alternative*: `◇` as a parse
    abbreviation (Lean's FormalizedFormalLogic defines `dia φ := ∼□∼φ`):
    smaller rule table, worse output.
  - *Why not one arena generic over the node*: D25 says copy first and
    extract in step 67 from what two suites copied. The report lists the
    copies.
- **The sequent is read at a world**: `Γ ⊢ Δ` holds at `w` when some
  formula of `Δ` is true at `w` or some formula of `Γ` false, and is
  valid in a system when it holds at every world of every model of the
  system. So validity of `Γ ⊢ Δ` is validity of `⋀Γ → ⋁Δ` (local
  consequence), which is what the labelled root sequent `w:Γ ⊢ w:Δ`
  says. Global consequence (hypotheses true at every world) is another
  relation; TPTP tells them apart by role (`hypothesis` local, `axiom`
  global). It is not built; where it goes: a second list of the sequent
  or a flag of the options, and in the search the hypotheses added to
  every world created (termination then needs blocking in every
  system).
- **Text**: the ordinary syntax with two prefixes, `[]` or `□` and `<>`
  or `◇`, binding like `~`. `[]` and `<>` are what the Modal Logic
  Playground, CEGARBox and Carnap read; TPTP's short forms are `[.]` and
  `<.>`, MOIN's `!` and `?` (which are linear logic's exponentials here,
  so they are refused), LWB's `box` and `dia` (words would take two atom
  names). `<>` cannot be taken for `<->`: a prefix stands where a
  formula starts, a connective after one. Atom names follow the ordinary
  check, so a formula without modalities is the same text in both
  suites.
- **JSON**: the ordinary sequent's form with the tags `{"□": a}` and
  `{"◇": a}`. A sequent without either is written at level 8 with the
  bytes of the ordinary sequent it is (the wire rule: the lowest level
  whose reader understands it), and an ordinary sequent's document reads
  as a modal one. A sequent whose arena holds a modality is written at
  the modal level, `wire::MODAL`. The level is read from the whole
  arena, used or not, since the form writes the whole arena and a
  reader of level 8 refuses the tag `□`.
- **The wire level's number is provisional.** `wire::MODAL` and
  `wire::LEVEL` are 20 in this chain, since 63g-b takes 19 and had not
  landed. The number is named in `core/src/wire.rs` (the constant,
  `LEVEL`, the table's row, the forms' entry), in the example on
  `modal::Sequent`, in `CHANGELOG.md` and in `web/tests/ordinary.rs`
  (the level `about` reports); the library's tests read the constant.
  The review sets it. No
  `core/tests/wire/level-N.txt` is written yet for that reason: 65b adds
  it once the number stands (those files are never rewritten).
- **Bounds**: every reader refuses past `limits.occurrences` as the
  ordinary readers do (the arena's formulas, the sequent's formulas, its
  unfolding, the operators and parentheses that wait). Nothing recurses:
  the parser has its own stack, the printer walks `Stops`
  (`core/tests/depth.rs` has a modal case).
- **D17, where quantifiers and agents go**: `Node` gains `Forall` and
  `Exists` with a bound variable and `Atom` gains terms, as the ordinary
  arena will; the labelled calculus takes the quantifier rules with
  eigenvariables beside its fresh labels, and the domain condition
  (constant, cumulative, varying) becomes a field of the options beside
  the system, a model gaining a domain per world. An agent index is a
  second `u32` in `Box` and `Diamond` (the node stays 12 bytes), a third
  argument of the relation in the rule file, and a relation per agent in
  the model.

## 3. The systems as values (built)

`core/src/modal/system.rs`.

- **`Condition`** (`#[non_exhaustive]`): `Serial`, `Reflexive`,
  `Symmetric`, `Transitive`, `Euclidean`, each with `name()` and
  `axiom()` (`D`, `T`, `B`, `4`, `5`). These are the five conditions the
  prompt lists, one per axiom of the cube (Brünnler 2009 Fig. 2,
  Halpern and Rêgo 2007 section 2, Negri 2005 Table III).
- **`System`** is a `Copy` value holding a set of conditions closed
  under five implications: reflexive gives serial; reflexive and
  euclidean give symmetric (and with it transitive); symmetric and
  transitive give euclidean; symmetric and euclidean give transitive;
  serial, symmetric and transitive give reflexive. Halpern and Rêgo
  state the first two; the others are two-line proofs written beside
  each rule in the code (ours). `a_system_holds_what_follows_on_every_frame`
  checks all of it against every relation on up to three worlds: a
  system has a condition exactly when every such frame that meets the
  given conditions meets it, and the 32 sets give fifteen systems. A
  research agent's enumeration up to four worlds found the same fifteen
  classes, which are the fifteen logics of Brünnler's Fig. 1 and of
  MOIN, TPTP and FormalizedFormalLogic.
  - *Why closed*: equality of values is then equality of frame classes,
    `System::T.with(Condition::Euclidean) == System::S5`, and the search
    and the model check read the conditions without asking which
    presentation the user chose. Brünnler needs the same closure for his
    rules to be complete ("45-complete" sets; K+{t,5} does not prove
    `□A → □□A` without it, his Fact 2).
  - *Alternative*: an enum of the named systems. It would need the
    conditions as a table anyway, and a sixteenth system would be a new
    variant at every `match`; a set of conditions is what D2 made of
    fragments.
- **Names**: one per system, `K, D, T, KB, KDB, B, K4, KD4, S4, K5, KD5,
  K45, KD45, KB5, S5`: the classic short names where there is one (D, T,
  B, S4, S5), `K` with the axioms' letters otherwise, as Halpern and
  Moses write KD45. `FromStr` reads those in either case and every code
  `K` plus letters (`KT4`, `KTB`, `KB4`, `KDB5`, `K4B`), a leading `D`
  as `KD`. TPTP's `M` for T is not read (a letter more to explain;
  interop-11 is later).
- **Default**: K (`System::default()`), the logic of all frames: what is
  valid there is valid in every system, and a newcomer who names no
  system gets the textbook's first logic.
- **`extends`**: a system extends another when it has its conditions;
  the tests of 65b use it (valid in a system, valid in every extension).
- **JSON**: the name, read by `FromStr`.

## 4. The Kripke model in the modal suite

`kripke::Model` is unchanged (worlds with `atoms`, `falsity`, `arrows`;
`world`), wire level 13, the strict reader for a document written by
hand.

- **The arrows are the relation.** `w.arrows` lists exactly the worlds
  `w` sees. Nothing is closed when a model is read or checked: an S4
  model lists its reflexive arrows, or the check says "not reflexive:
  world 0 does not see itself".
  - *Why*: the prompt asks that the frame conditions be checked on the
    model and the failing one named. A student who draws a model for S4
    and forgets an arrow should be told, and a model means the same in
    every system. The ordinary suite reads the same key as a generating
    relation (its order is the closure); that reading stays the
    ordinary suite's.
  - *Alternative*: arrows as generators closed under the system. Models
    get shorter (an S5 model of n worlds lists n² arrows otherwise), but
    one document then denotes different frames in different systems and
    no condition can ever fail. Rejected for the data; the *drawing* may
    leave implied arrows out (an option of the drawing, 65d: reflexive
    loops and composites hidden where the system has the condition).
- **`falsity`** must be false: a modal model is classical at each world.
  An atom the sequent does not name is ignored. An arrow written twice
  is one arrow.
- **The check** (new, `core/src/modal/model.rs`, 65b, under the panel):
  1. Shape: at least one world, `world` among them, every arrow to a
     world of the model, no `falsity`.
  2. The frame conditions of the system, in `Condition::ALL`'s order,
     the first that fails reported with its witness: serial (a world
     without arrows), reflexive (a world), symmetric (two worlds),
     transitive and euclidean (three worlds). Sorted, deduplicated
     adjacency; transitive is "for every arrow x→y, the arrows of y are
     among those of x", euclidean "for every arrow x→y, the arrows of x
     are among those of y", each a merge of two sorted lists, so the
     cost is the sum over the arrows of two degrees: counted as work,
     polled, bounded by `limits.work`.
  3. Truth: a bit per world and formula, formulas in arena order,
     `□A` at `w` the conjunction over `w`'s arrows, `◇A` the
     disjunction. Cells are (worlds + arrows) × formulas, refused past
     `MODEL_CELLS` and `limits.memory_bytes` as step 55's check
     (`MODEL_CELLS` moves to `kripke` then, re-exported from
     `ordinary`).
- **Its API** (the names are proposals; `refutes` and `forcing` are
  taken on `Model` by the ordinary suite, and Rust has no overloading):
  - `Model::meets(system, &limits, stop) -> Result<(), Error>`: steps 1
    and 2.
  - `Model::truth(&sequent, system, &limits, stop) -> Result<modal::Truth, Error>`:
    all three. `Truth::holds(world, id)`, `at(world)` (every formula's
    value, as `Forcing::at`), `sequent_holds(world)`, `failing() ->
    Option<u32>` (a world where the sequent fails, the model's own
    first), `witness(world, id) -> Option<u32>` (for a false `□A` or a
    true `◇A`, a world it sees that shows it: the "why" of a modal
    formula).
  - `modal::Countermodel { sequent, system, model }` with `check`, and
    the crate-private `found` of step 55 (a model too large to check
    drops the model, never the verdict).
- **Errors**: a model that is none of the system is kind `invalid`. The
  failing condition should be data, not only text (the page marks the
  missing arrow): a variant `Error::Frame { condition, worlds }` with
  code `frame_condition`, beside `Error::Model` for the shape. Within 64
  bytes: a `Condition` and a boxed or three-element list.
- **"At a world or at every world"** (the author, 2026-11-08): with a
  world given, `sequent_holds(world)`; without, `failing()` over all
  worlds. "Which world fails and why": the world, the values of the
  sequent's formulas there (`at`), and `witness` for each modal one.

## 5. The search: one labelled tableau for the cube

### 5.1 The calculus

G3K (Negri 2005, Table II, p. 512) over labelled formulas `x:A` and
relational atoms `xRy`, with the frame rules of p. 515:

```
L□   y:A, x:□A, xRy, Γ ⊢ Δ  /  x:□A, xRy, Γ ⊢ Δ
R□   xRy, Γ ⊢ Δ, y:A        /  Γ ⊢ Δ, x:□A          (y fresh)
L◇   xRy, y:A, Γ ⊢ Δ        /  x:◇A, Γ ⊢ Δ          (y fresh)
R◇   xRy, Γ ⊢ Δ, x:◇A, y:A  /  xRy, Γ ⊢ Δ, x:◇A
Ref  xRx, Γ ⊢ Δ             /  Γ ⊢ Δ
Sym  yRx, xRy, Γ ⊢ Δ        /  xRy, Γ ⊢ Δ
Trans xRz, xRy, yRz, Γ ⊢ Δ  /  xRy, yRz, Γ ⊢ Δ
Eucl yRz, xRy, xRz, Γ ⊢ Δ   /  xRy, xRz, Γ ⊢ Δ
Ser  xRy, Γ ⊢ Δ             /  Γ ⊢ Δ                (y fresh)
```

and the propositional rules of G3c at one label. A system's calculus is
G3K with the frame rule of each of its conditions (all of them, the
implied ones too: closed sets make the rule set complete for the frame
class). Read upside down it is a prefixed tableau: left of `⊢` is "true
at", right "false at". The search works on sets (a formula at a world
once, an edge once), so Negri's contracted instances of Trans and Eucl
(2009, section 4.2) are not needed.

### 5.2 The state and the rules of the search

One branch is: worlds numbered in the order made, each with the set of
formulas true there and the set false there (bitsets over the arena);
the tree of who made whom; the relation `R` as edges, **kept closed
under the system's frame rules but Ser** (whenever an edge is added, Ref
on a new world, Sym, Trans and Eucl to a fixpoint; each derived edge
remembers the rule and the edges it came from, for the proof). The root
world holds `Γ` true and `Δ` false.

Rules, in the order the strategy takes them:
1. **Close** the branch on a formula true and false at one world, `⊥`
   true, `⊤` false.
2. **Without choice**: the one-premise propositional rules; `□A` true
   (or `◇A` false) at `x` puts `A` true (false) at every `y` with
   `xRy`, also for edges made later. To a fixpoint over all worlds.
3. **Split** on a two-premise propositional rule, the oldest world
   first. The search is depth first over the splits, with a trail to
   undo, or a copy per split: 65b measures.
4. **Make a world**: `□A` false at `x` (or `◇A` true) with no `y`,
   `xRy`, where `A` is false (true): a fresh world `y`, the edge, `A`
   there; then the closure of `R`. In a serial system, a world with a
   `□A` true or `◇A` false of its own and no edge out at all gets a
   fresh world by Ser. The oldest world that can, first; a blocked
   world never (5.3). Then back to 1.
5. Nothing applies: the branch is open and saturated, and gives the
   countermodel (section 6).

Every step is one inference of the calculus on the branch's labelled
sequent, so a closed tableau is a derivation (section 7).

A unit of `limits.work` is a formula placed or an edge added; the stop
is polled on LJT's cadence; worlds, edges and trail are charged to the
memory account (`linear::search::memory::Account`, as LJT uses it).

### 5.3 Termination, system by system

Let `d` be the sequent's modal depth, `p` the number of its subformulas
`□A` and `◇A`, `n` the number of its subformulas.

| systems | why the search ends | loop check | bound per branch | sources |
|---|---|---|---|---|
| K, D, T | a world made by `x` holds formulas of smaller modal depth than `x`'s | none | depth `d`, at most `(p+1)^d` worlds | Negri 2005 Thm 6.6 (K), 6.7 (T); Massacci 2000 Table IV (K, D, T: prefixes of length at most `1+d`) |
| KB, KDB, B | the same: a formula sent back to the parent has depth two less | none | as above | Negri 2005 Thm 6.8 (KB, TB); Massacci Table IV |
| K4, KD4, S4 | blocking, below | a world is blocked by a proper ancestor whose formulas include its own | finite; chains up to `2^n` in the worst case (ours: no polynomial bound claimed for subset blocking) | Garg, Genovese and Negri 2012 (TR Def. B.5, Fig. 7, Thm B.16; K, T, K4, S4 and each with D, Thm B.24); Goré 1999 section 4.14.4; Wu and Goré 2019 |
| K5, KD5, K45, KD45, KB5, S5 | every world but the root lies in one cluster whose worlds all see each other, so a witness made once serves every world of the cluster | none | at most `2p + 2` worlds | shape: Halpern and Rêgo 2007 Lemma 3.2 (K5, KD5, K45, KD45, S5), Halpern and Moses 1992 Prop 6.2 (S5, citing Ladner), Hemaspaandra and Schnoor Thm 4.15 (KB5 as K4B); the search's argument is ours; Massacci Thm 9.6 has it for K45, KD45, S5 |

- **Built in 65b with blocking by any earlier unblocked world**, not
  ancestors only: ancestor blocking let sibling worlds of equal content
  each expand (KD4 sequents of fourteen nodes at 40 million units; the
  panel of 65b). The argument below carries over: every existing
  world's signature is final when a world is first tested. The report's
  65b section has the rest of what was built differently.
- **K4, KD4, S4, the loop check.** Let `S(x)` be the formulas true and
  false at `x` together with every `□A` true and `◇A` false at a world
  that sees `x` (the modal formulas `x` inherits). `x` is *blocked*
  when a proper ancestor `a` in the tree has `S(x) ⊆ S(a)`; rule 4 does
  not apply at a blocked world. This is Garg, Genovese and Negri's
  condition (their `Sfor`, subset against proper ancestors).
  - In these systems formulas move only to worlds made later (no
    symmetric and no euclidean edge leads back), and rules 1 to 3 are
    at a fixpoint before rule 4 runs, so `S` of every ancestor is final
    when `x` is tested. The test is made once, when `x` is about to make
    a world. (ours; the research note on GGN's Thm B.16 describes its
    termination argument as resting on the same fact, in its own
    words, not theirs.)
  - Why subset is enough here and not with symmetry: a blocked world
    only needs what its blocker sees; with a symmetric edge the blocker
    would also be seen from the blocked world's parent and its box
    formulas would have to hold there, which only equal sets give
    (Horrocks, Sattler and Tobies 2000, section 3.3; GGN say there is
    no closure relation for symmetry). The symmetric systems of the
    cube need no blocking, so the question does not arise.
- **The euclidean systems** (ours, checked against Halpern and Rêgo's
  shape): closing `R` under Eucl makes every world but the root see
  every other such world and itself, as soon as it exists (from `xRy`
  and `xRz`: `yRz`, `zRy`, `yRy`). Rule 4 fires for `□A` false at a
  world only when no world it sees has `A` false, so each of the `p`
  modal subformulas makes at most one world from the root and one from
  the cluster (the root of K5 and KD5 does not see the worlds the
  cluster made; in K45, KD45, KB5 and S5 it does, by Trans or Sym, and
  the bound is `p + 2`). Brünnler 2009 (Thm 3, Lemma 7) covers these six
  systems with a uniform cyclic check and a `2^n` bound instead; it is
  the fallback if 65b's panel finds a hole in the argument.
- **Seriality** never makes a chain: Ser fires only at a world with a
  box formula of its own and no edge out, and outside K4-like systems
  the new world's own formulas are of smaller depth; in KD4 the new
  world is subject to blocking. A world left without edges gets a
  reflexive arrow when the model is read (section 6).

### 5.4 What the search costs, and the complexity of the problems

| systems | validity | read in |
|---|---|---|
| K, T, S4 | PSPACE-complete | Ladner 1977 through Halpern and Moses 1992 Thm 6.6(1), 6.11, 6.12 and section 6.3 |
| K4 | PSPACE-complete | Hemaspaandra and Schnoor 2008 Thm 2.4(1), attributed to Ladner |
| D, KD4 | PSPACE-complete | Halpern and Rêgo 2007 Thm 3.1 (the upper bound by "a straightforward modification of Ladner", no proof read) |
| KB, KDB, B | PSPACE-complete | hardness Rybakov and Shkatov 2019 Thm 2.1; membership cited there to Goré 1999 (not read), Hemaspaandra and Schnoor Cor 4.36, 4.37 for KB and B |
| K5, KD5, K45, KD45, S5 | coNP-complete | Halpern and Rêgo Thm 3.1; S5 Halpern and Moses Thm 6.1, KD45 Thm 6.4 |
| KB5 | coNP-complete | Hemaspaandra and Schnoor Thm 4.15 (as K4B); no source read names KB5 |

- The search is exponential in time in every system, as a decision of a
  coNP-hard problem by backtracking is.
- **Space**: in the euclidean systems a branch has at most `2p + 2`
  worlds, so the search holds a polynomial state (the NP bound's small
  model). In the other nine a branch is a tree of worlds. For K, T and
  S4 there are formulas of size `O(m²)`, `O(m)` and `O(m)` whose every
  model has `2^m` worlds (Halpern and Moses Prop 6.5); no smaller bound
  is claimed here for the other six. The search
  keeps the whole tree of a branch, so it is **not** a PSPACE procedure,
  where Ladner's and Massacci's (Thm 9.8) forget a finished subtree. It
  keeps it because the countermodel and the labelled proof are that
  tree, and both are exponential on those formulas whatever the search
  does. The limits bound it (`work`, `memory_bytes`,
  `derivation_bytes`), and the answer past them is "unknown" with the
  bound's name. Not a modal SAT competitor (the prompt): CEGARBox and
  InKreSAT are.
- **One optimisation worth its code** (ours; 65b measures before it
  keeps it): when the first premise of a split closes by a derivation
  that never used the formula the split added, the conclusion closes by
  that derivation and the second premise is not searched. In K, D, T,
  K4, KD4 and S4 the subtrees of two worlds made by one world do not
  see each other, so without this the splits inside one subtree
  multiply with those inside the next; with it the search is the and-or
  search of the Gentzen calculi. The same use marks say which frame
  steps and which propagations the proof needs, so the derivation
  emitted holds only those.

## 6. The countermodel

From an open saturated branch:
- **Worlds**: the branch's worlds; the model's world is the root.
- **Arrows**: the branch's `R`, already closed under the system's
  rules. In K4, KD4 and S4, a blocked world gets the arrows of its
  blocker, and the relation is closed again (Garg, Genovese and Negri,
  Def. B.9 and Lemma B.10). 65b may instead drop a blocked world and
  send the arrows into it to its blocker, which gives smaller models
  (ours: `S(x) ⊆ S(a)` carries every box formula that reaches `x` on to
  `a`, and what `x` was made to witness is in `a`); either way the
  model check decides each model.
- **Serial systems**: a world without arrows gets one to itself. It has
  no box formula of its own (Ser would have fired), so nothing true
  there changes, and the new arrow adds no pair that Trans, Sym or Eucl
  would not already demand (ours; Brünnler closes his countermodels the
  same way, Def. 6 and Lemma 6).
- **Atoms**: `p` at `x` exactly when `p` is true at `x` on the branch.
- **Checked** before it is returned (`Countermodel::found`): the frame
  conditions, then the sequent false at the model's world. A model the
  check refuses is `Error::Countermodel`, a defect of the search; one
  too large to check is dropped and the verdict stays, as in step 55.
- Nothing in sections 5.3 and 6 is trusted for a verdict. A wrong blocking argument shows as a defect error on some
  input, which the differential tests of section 12 look for.

## 7. The proof and its check

- **The proof is a `rules::Derivation` in the system's labelled rule
  file**, checked by `rules::Derivation::check` before `Valid` is
  answered. `Evidence::Derivation(Box<rules::Derivation>)`, the enum
  `#[non_exhaustive]`.
  - *Why*: D28 designed the generic derivation for labelled modal
    calculi (sorts, `fresh` for a new label), its checker went through
    the panel of 60b, its exports, text, JSON and protocol kinds exist,
    and step 66's "labelled calculus step by step" is then
    `rules::Interactive` on the same files. Labels make the frame
    condition a proof uses a visible step (`Trans` in the proof of
    axiom 4), which is the research note's reason for labelled calculi
    in teaching.
  - *Tried in this session*: a draft rule file for S4 and a hand-written
    derivation of `⊢ x:□p → □□p` (R→, R□, R□, Trans, L□, ax) are
    "valid in the calculus given" for `deduxit check --calculus` on
    `main`; the same derivation with a label that is not fresh at R□ is
    refused ("a name the rule asks to be new occurs in the conclusion").
    No change to `rules` was needed.
- **The rule file's shape** (as tested): sorts `label` and `item`; the
  formula connectives with `□` and `◇` (prefix, precedence 5); `:` with
  sorts `[label, formula, item]` and `R` with `[label, label, item]`,
  both infix at precedence 0; two zones `left` and `right` (mark `⊢`) of
  sort `item` and discipline `set`, so that a sequent prints as Negri
  writes it, `x : □p, x R y ⊢ y : □p`; variables `A`, `B` of sort
  `formula` and `x`, `y`, `z` of sort `label`; contexts `Γ`, `Δ`; the
  rules of 5.1 with `fresh: ["y"]` on R□, L◇ and Ser; `ax` on any
  formula at one label (Negri's is atomic; the general one is sound and
  shorter), `L⊥`, `R⊤`, and G3c's rules for `¬ ∧ ∨ → ↔`.
  - A node gives positions for the conclusion's written members and for
    a premise's member that binds a new variable (the checker says how
    many: R□ takes two, the boxed formula and the new edge).
  - Atoms of sort `label` are written `{"atom": "w0", "sort": "label"}`
    in a derivation's terms.
  - Set zones are what a tableau branch is, and make the kept principal
    formula of L□ and the dropped one of the propositional rules both
    instances.
- **One calculus per system**, named `G3` and the system's name (`G3K`,
  `G3S4`): `modal::calculus(system) -> rules::Calculus` builds the
  `CalculusForm` in code from one table (the base rules, and a frame
  rule per condition). The core crate has no JSON parser outside the
  feature `protocol`, so it cannot read a checked-in file there;
  `Calculus` writes itself back as a rule file, which is how the
  command (`--calculus G3S4`), the manual and step 66 get the text. A
  test pins the written file of each system.
- **What the trust is**: the generic checker (reviewed in 60b) and the
  rule table. Each rule is Negri's (2005), whose soundness for Kripke
  semantics this session did not read in detail; 65b's
  panel-argument reads the table against Table II and p. 515, and the
  differential tests check that no sequent is ever both proved and
  refuted.
- **Cost of the choice**: `rules` needs the feature `serialize`, so
  `modal::decide` does too (the arena, the systems and the model check
  are in every build). A node stores its whole sequent as term ids, so
  a derivation's bytes are steps times sequent size; it is estimated
  against `limits.derivation_bytes` before it is built, and past it the
  refusal is `Refusal::Output`, never a verdict lost: where the proof
  is refused the outcome is that refusal, as `ordinary::decide` does.
- **The root** is `w0 : Γ ⊢ w0 : Δ`; labels are `w0`, `w1`, … in the
  order the search made them.
- *Alternatives*:
  - A native `modal::Derivation` with a checker of its own: no feature
    dependency and tighter storage, but a second trusted checker, new
    drawing code, and step 66 would then write the rule files and a
    converter anyway (as 60 did for LK). More code, more trust.
  - Gentzen calculi per system with histories (Heuerding, Seyfried and
    Zimmermann 1996; Goré 1999): they fit rule files too (a context
    under `□`, as LL's promotion), but only K, D, T, K4, KD4, S4, K45
    and KD45 have cut-free ones; B, KB4 and S5 need analytic cut, and
    for KB, K5, KDB, KD5 Goré knows none (his Fig. 7 and the "?"
    entries). S5 would need a hypersequent calculus (Avron, Poggiolesi,
    Restall), which is the labelled calculus with every world seeing
    every world.
  - Nested sequents (Brünnler 2009, MOIN): one system for all fifteen
    with a proved search, and the research note's first idea for the
    engine. A nested sequent is close to a labelled sequent whose
    relation is a tree: Goré and Ramanayake 2012 prove it for
    tree-hypersequents (Lemma 3.9, Cor. 3.10) and only sketch it for
    nested sequents; Fitting 2012 calls prefixed tableaux and nested
    sequents notational variants. So the search here is close to
    MOIN's with the relation
    explicit; but a nested sequent is a tree of multisets, which a rule
    file's fixed zones cannot hold, so it would need its own derivation
    type and checker.

## 8. The outcome, the options and the errors

- `modal::decide(&sequent, &modal::Options) -> Result<Outcome, Error>`
  and `decide_within(&sequent, &options, &limits, stop)`, the shape of
  `ordinary::decide` (the author's Q5).
- `Outcome` (`#[non_exhaustive]`): `verdict` (`Valid(Evidence)`,
  `NotValid`, `Unknown(Reason)`), `system`, `countermodel:
  Option<Countermodel>`, `statistics` (`#[non_exhaustive]`: worlds made,
  splits, units of work, worlds blocked).
- **Options** (D15, D16): `modal::Options { system: System }`,
  `#[non_exhaustive]`, serde with `deny_unknown_fields`, default K. The
  command maps `--system` onto it, the page holds it as JSON in the
  tab's settings, the protocol reads it from the statement. The bounds
  are the shared `Limits` (`work`, `memory_bytes`, `occurrences`,
  `derivation_bytes`, and the time limit through the stop). There is no
  method to choose yet: one engine. The drawing of a model gets its own
  options value in 65d (implied arrows shown or hidden, default shown
  in the data, hidden loops in S4 and S5 drawings only if the legend
  says so).
- **Errors**: `Error::Frame` (section 4); `Error::Countermodel` and
  `Error::ReadBack`-like defects reuse the ordinary suite's variants
  where the wording fits, else a variant each; every new code joins
  `Error::CODES`. An unknown system name is `Error::UnknownName` (built).

## 9. The JSON forms and the wire level

- Sequent (built) and system (built, its name).
- **Statement** (63a): the sequent's document with `"system": "S4"`
  beside its keys, which the sequent's reader ignores.
- **Outcome**: `{version, deduxit, verdict, reason?, system,
  statistics, sequent, derivation?, countermodel?}`, the derivation the
  `rules` derivation's form nested (it names its calculus), the
  countermodel `{"model": {…}}` as the ordinary outcome nests it. It
  reads back; its reader checks nothing but shape, and `check` is the
  caller's.
- **Evaluation** of a user's model (written only): `{version, system,
  holds, world?, failing?, frame?: {condition, worlds}, values?}`.
- All modal forms share the one level `wire::MODAL` (they are released
  together in 0.6.0); the model stays at 13. Each gets its entry in
  `docs/src/wire.schema.json` and its case in the `wire-schema` check
  when the command writes it (65c); the sequent's entry goes in with
  the command too, since the check wants a document of every form.

## 10. A model the user gives

The library call is `Model::truth` (section 4). The command and the
bindings are step 55's with the system named (65c):
- `deduxit check MODEL.json --system S4 --sequent "[]p -> p" [--world
  n]`, as `check --logic … --sequent …` checks an ordinary model; exit
  0 holds, 1 fails or the model is none of the system (the message
  says which), 2 an error.
- It answers, in this order: the frame condition that fails and where;
  else whether the sequent holds at the world given or at every world;
  where not, the world and the values of the sequent's formulas there.
- The page's model editor is step 68's; it needs `Error::Frame`'s data
  and `Truth::at`.

## 11. The suite in the protocol (63a)

Suite `modal`, label "Modal logic". Registered when its first operation
runs (65c), not in this session: a suite without operations would show
empty in `describe()`, which the client reads.

| kind | role (`Role`) | what it is | form |
|---|---|---|---|
| `modal.sequent` | `Statement` | the sequent with its `system` | modal sequent |
| `modal.outcome` | `Evidence` | a sequent decided: verdict with a derivation or a countermodel | modal outcome |
| `modal.model` | `Evidence` | a Kripke model, given or taken from an outcome | Kripke model (level 13) |
| `modal.evaluation` | `Evidence` | a model checked against a statement | modal evaluation |

| operation | from → to | class, cost | promise |
|---|---|---|---|
| `modal.decide` | sequent → outcome | derived, search | `decides`: the derivation passes the checker, or the model refutes the sequent in the system |
| `modal.countermodel` | outcome → model | derived, instant | `exact` |
| `modal.evaluate` | sequent, model → evaluation (parameter `world`) | derived, instant | `exact`: the evaluation is the check's own answer |
| `modal.derivation` | outcome → `rules.derivation` | derived, instant | `proves`, checked in the system's calculus |
| `modal.calculus` | sequent → `rules.calculus` | derived, instant | `exact` (step 66 starts sessions from it) |
| `modal.draw` | outcome → `drawing` | derived, instant | `renders` |
| `modal.export` | outcome → `text` (text, latex, typst) | derived, instant | `renders` |
| `modal.duplicate` | sequent → sequent | copy, instant | `exact` |

- The classes, costs and promises are 63a's (`Class`, `Cost`, the
  seven promises of `protocol::promises`); no new promise is needed. A
  model the user wrote is `Evidence` by role for want of a better one:
  63a's roles have none for a value the user gives that is no
  statement, which 65c raises if the workspace needs to tell them
  apart.
- `reads`: `modal.decide` reads `limits.work`, `limits.memory_bytes`,
  `limits.occurrences`, `limits.derivation_bytes`; `modal.evaluate`
  `limits.occurrences`, `limits.memory_bytes`, `limits.work`.
- The model is a kind here, where 63a kept it inside the outcome,
  because `modal.evaluate` takes one as an input. If the ordinary suite
  gains the same input (step 68), the kind moves to the shared ones.
- No `modal.certificate`: Rocq certificates of modal derivations are
  the research note's modal-logics-3, not this step.
- A later operation with a promise between suites: Gödel's translation
  `ordinary.sequent → modal.sequent` (intuitionistic valid exactly when
  the image is S4-valid), `same_verdict`. In this step it is a test
  only (section 12).

## 12. What 65b checks the search against

- **Every answer's evidence**: each derivation passes
  `rules::Derivation::check`, each model `Countermodel::check`, on
  every test input; no input gets both.
- **The axioms' table**: K's axiom is valid in all fifteen; each of D,
  T, B, 4, 5 exactly in the systems that have its condition (75 cases
  from `Condition::axiom` and `System::has`).
- **The lattice**: valid in a system, valid in every system that
  `extends` it; a countermodel of an extension refutes in the system.
- **Small frames**: every model of up to three worlds over the
  sequent's atoms (512 relations, bounded atoms) that meets the system
  must satisfy what the search calls valid, and a sequent one of them
  refutes must not be called valid. A second reference of the
  panel-counterexamples member.
- **Classical agreement**: a sequent without modalities gets the truth
  table's verdict in every system.
- **Gödel's translation**: for generated ordinary sequents, the
  intuitionistic verdict of the ordinary suite equals the S4 verdict of
  the image (`p` to `□p`, `A → B` to `□(A' → B')`, `¬A` to `□¬A'`, `∧`
  and `∨` unchanged; McKinsey and Tarski 1948, read through the
  encyclopedia's statement). Two suites, two engines, one answer.
- **Forged evidence**: a derivation broken once per rule is refused; a
  model with an arrow removed is refused for the condition by name.
- **Bounds**: Halpern and Moses's formulas `φ_m` for K, T and S4 at
  small `m` give models of `2^m` worlds, and at a larger `m` "unknown"
  at the memory bound within the time limit; the integers member
  drives the world count, the edge count and the cell count to their
  limits.

## 13. For the next sessions, and what is open

- **65b** (the search and its panel): `modal::calculus`, the search of
  section 5, the model's reading and check of sections 4 and 6,
  `decide`, the outcome's JSON, the tests of section 12, the panel on
  the model check, the rule table and the blocking argument. The seven
  systems the prompt names are required. The other eight run through
  the same code (KB and KDB as B without Ref; KD4 as K4 with Ser; K5,
  KD5, K45, KD45 and KB5 as S5 with fewer rules) and are offered if
  they pass the same tests; one that does not is refused by name
  (`unsupported`) and listed for the author, not shipped.
- **65c** (the command): `--system`, `prove`, `check` of a model, the
  outputs (text of a labelled derivation, the model's lines: `Display`
  of `kripke::Model` says "below" for an arrow, which is the ordinary
  reading; the modal lines need "sees"), the exports (the symbols `□`
  and `◇` per target; the SVG font's advances for them need checking),
  the manual's chapter, the guide, the protocol's suite, the schema.
- **65d** (the page): the suite in the registry, the countermodel drawn
  (step 55's drawing with arrows that are no order: cycles and loops),
  the labelled derivation as a tree.
- **Open, for the author through the planning session**:
  1. Whether the eight systems beyond the prompt's seven are offered in
     0.6.0 when they pass (recommended: yes, KD45 above all).
  2. The command's spelling for the modal suite (`--system` selecting
     it, as `--logic` selects the ordinary one) is a new flag, not a
     change; 65c proposes it with its help text.
- **Not done here, on purpose**: the model check (it needs the panel),
  the suite's registration, any manual chapter beyond one line of
  "what exists".

## 14. Sources (read 2026-11-16 by three sub-agents; "text" means the source's own text)

- Negri, "Proof analysis in modal logic", J. Philos. Logic 34 (2005)
  507–544: text (archived copy of the author's PDF). Table II, p. 515,
  Thms 6.6 to 6.8, Cor. 6.10. No termination result for K4, Eucl or
  Ser.
- Negri, "Kripke completeness revisited" (2009): text of the preprint.
  Countermodels from a fair search, possibly infinite; section 4.2.
- Garg, Genovese, Negri, "Countermodels from sequent calculi in
  multi-modal logics", LICS 2012, and its technical report
  (`mpi-sws.org/~dg/papers/lics12-tr.pdf`): text. Appendix B: Def.
  B.5, B.8, B.9, Lemma B.10, Thm B.16, Cor. B.19, Thm B.24, Fig. 7 and
  8. No symmetry, no euclideanness.
- Brünnler, "Deep sequent systems for modal logic", Arch. Math. Logic
  48 (2009), author's manuscript, and "Nested sequents", arXiv
  1004.1845, chapter 2: text. Figs. 1 to 3, Def. 7 and 9, Fact 2,
  Lemmas 5 to 7, Thm 3 (Thm 2.23 in the habilitation).
- Massacci, "Single step tableaux for modal logics", J. Autom.
  Reasoning 24 (2000) 319–364: text (archived copy). Fig. 1c, Tables
  II and IV, Thms 7.1, 8.4, 9.6, 9.8. Its depth bound for K4, KD4, S4
  is printed two ways. No termination theorem for K5, KD5, KB4.
- Fitting, "Prefixed tableaus and nested sequents", Ann. Pure Appl.
  Logic 163 (2012): text of the 2011 posting, sections 1 to 7.
- Goré and Ramanayake, "Labelled tree sequents, tree hypersequents and
  nested (deep) sequents", AiML 9 (2012): text. Lemma 3.9, Cor. 3.10.
- Goré, "Tableau methods for modal and temporal logics", draft
  TR-ARP-15-95 of the Handbook chapter (1999): text. Figs. 6, 7, 17,
  sections 4.14.4, 6.2, 6.6.
- Wu and Goré, "Verified decision procedures for modal logics", ITP
  2019 (LIPIcs 141): text. K, KT, S4 in Lean, models returned.
- Horrocks, Sattler, Tobies, "Practical reasoning for very expressive
  description logics", Logic J. IGPL 8(3) (2000): text, section 3.3.
- Halpern and Moses, "A guide to completeness and complexity for modal
  logics of knowledge and belief", Artif. Intell. 54 (1992): text of
  the author's 1996 PDF. Thms 6.1, 6.4, 6.6, 6.11, 6.12, Props 6.2,
  6.3, 6.5.
- Halpern and Rêgo, "Characterizing the NP-PSPACE gap in the
  satisfiability problem for modal logic", arXiv cs/0603019: text. Thm
  3.1, Lemma 3.2.
- Hemaspaandra and Schnoor, arXiv 0802.1884: text of the parts cited.
  Thm 2.4, 4.15, Cor. 4.36, 4.37.
- Rybakov and Shkatov, arXiv 1910.02257: text. Thm 2.1.
- Girlando and Straßburger, "MOIN", IJCAR 2020, and MOIN's readme:
  text. All fifteen logics, derivation or countermodel as LaTeX,
  `!`/`?`, axioms as a list.
- Steen and others on TPTP's non-classical forms, arXiv 2508.09318,
  and the TPTP language page: text. `{$box}`, `[.]`, the logic
  specification, the system names.
- The Modal Logic Playground's source, CEGARBox's readme, Carnap's
  parser, FormalizedFormalLogic's `Formula`: text, for the syntax.
- Gödel's translation: the encyclopedia articles on modal companions
  and on Gödel; McKinsey and Tarski 1948 not read.
- **Not obtained**: Ladner 1977 (its theorems through Halpern and
  Moses, Halpern and Rêgo, Hemaspaandra and Schnoor); Heuerding,
  Seyfried and Zimmermann 1996 (abstract only; its rules through Goré
  and Wu and Goré); Fitting 1983, Fitting and Mendelsohn; Poggiolesi
  2008 and Avron 1996 (abstracts); Nagle and Thomason 1985; Blackburn,
  de Rijke and Venema chapter 6; a proof of PSPACE membership for D,
  KD4 and KDB; that the fifteen logics are pairwise distinct as logics
  (their frame classes are: the enumeration).
- The agents' full notes, with quotations, locators and what each
  could not verify, are `plan/notes/research/65-modal/nested.md`,
  `labelled.md` and `complexity.md` (sub-agents' text: data to weigh).
  Beside them, `g3s4-draft.json` is the rule file tried in section 7
  and `four-draft.json` its derivation of axiom 4 (`deduxit check
  --calculus g3s4-draft.json four-draft.json`).
