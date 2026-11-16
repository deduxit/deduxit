# Step 66: the labelled modal calculus step by step, and its soundness in Rocq

## Session 66a: the calculus step by step

Opus 5.5 at `high`, a reused worker (the conversation of 54x6), in the
jj workspace `../deduxit-wt-step-58a-610ed3`, from the brief
`start-66a.md` and `plan/66-labelled-modal.md`, on 2026-11-16, cores 7
to 9. The chain on `main@origin`:

| change | what |
|---|---|
| `zqmrqlxq` | `[]` and `<>` read as `□` and `◇` in a calculus's text |
| `zuzplnsw` | `rules::Interactive::extend`, `rules::Calculus::contexts`, the command's `derive` in a sequent calculus, the random walks over the fifteen calculi, the rules files |
| `zlzplptt` | the behaviour lock: `interact-calculus-labelled` |
| `vyxlvxry` | the manual ("Prove step by step" in the modal chapter, `rules.md`), the guide, the status, the changelog |
| `zpmqyopq` | `plan/notes/labelled-certificate.md`, the certificate's design for 66b |

### The spellings

`rules::text`'s ASCII table gains `[]` for `□` and `<>` for `◇`, read
where the calculus declares the symbol and not the spelling, as `->`
is. Every place a labelled formula is read goes through that lexer: a
session's sequent and its `NAME=TEXT` values, `--assume`, and the
bindings' `rules_start`. `<>` and `<->` do not meet: the longest match
reads `<->` whole.

### The sessions, and what is refused

Backward steps worked already (`interact --calculus G3S4`, 65c). Forward
steps did not: 62b's lines are hypothesis and derived-line nodes, which
a sequent calculus never has (no rule discharges, no assumption is
given apart from the sequent), and its `derive` refuses every rule with
contexts, so every labelled rule was refused forwards.

What reads forwards in a labelled calculus is the left rules and the
frame rules: every rule of `modal::calculus` keeps its principal
formula and its contexts are the whole zones, so a one-premise rule
whose premise only adds before `⊢` is forward reasoning on the goal's
hypotheses. `L□` derives `y : A` from `x : □A` and `x R y`, `Trans`
derives `x R z` from `x R y` and `y R z`, the relational atoms being
hypotheses like the labelled formulas (the brief's "relational atoms as
lines too"). So:
- **`rules::Interactive::extend(goal, &Apply)`** plans the rule's
  backward step on the goal as `apply` does and takes it only where its
  one premise is the goal with members added to the first zone, every
  member kept; the node, the derivation and the JSON are `apply`'s, so
  no wire level changes. It answers the goal that holds the new members.
- **The command's `derive G RULE N…`** calls it in a calculus that
  declares contexts (`rules::Calculus::contexts`, new), reading N… as
  the goal's member numbers as `goals` prints them, and writes the new
  goal and an `added N: MEMBER` line per member added. In a calculus
  without contexts (natural deduction) `derive` is 62b's, unchanged.
- **The frame rules in both directions**: backwards with `apply` on any
  goal, forwards with `derive` from the relational atoms they need
  (`Sym`, `Trans`, `Eucl`) or from none (`Ref`, `Ser`, the label given
  as `x=w0`, a new one made for `Ser`'s `y` as backwards).
- **Refused forwards, each with its reason**: a rule with two premises
  (`R∧`, `L∨`, `L→`, `L↔`, `R↔`: "a forward step keeps one goal"), one
  with none (`ax`, `L⊥`, `R⊤`: it closes a goal), one that acts on a
  member after `⊢` (every right rule: "acts on members after ⊢"; `R¬`'s
  premise adds only before `⊢`, so without this test it passed, the
  reviewer's finding), one that adds after `⊢` (`L¬`), and a step that
  adds nothing the goal holds already. Each says "apply it backwards to
  the goal" but the last, whose step would change nothing.
  `forward_steps_are_the_rules_that_add_hypotheses` pins the set in each
  of the fifteen calculi (`L∧`, `L□`, `L◇` and the system's frame
  rules), on a goal where `R¬` and `R→` would add a new hypothesis; it
  fails without the test on the acted members.
- **What does not carry over from 62b**: lines with stable numbers.
  Members are numbered per goal, as `apply` numbers them, and a set
  zone's order can change between goals, so a number names a member of
  one goal only. `assume`/`cite` close by lines, which a sequent
  calculus has none of; its goals close by `ax` (`apply`).

The library's `derive` now names `extend` in its refusal of a rule with
contexts. Neither item was released: `extend` and `contexts` are
additions.

**Tests** (`core/tests/rules.rs`, `random_labelled_steps_keep_a_checked_session`):
random sessions in all fifteen calculi over seven sequents (the axioms
K, D, T, B, 4, 5 and one of every connective, written with `[]` and
`<>`): `apply` by what `rules` lists (closing rules first) or any rule
on any members, `extend` by the forward rules where they apply or any
rule on any members, `assume`, `undo`. Every state is read back through
`wire::upgrade`, which replays and checks every step, and written to the
same JSON; every finished state passes `finished`. Committed: two walks
of twenty steps per sequent and system (about 36 s in release). Run
once by hand at twelve walks of forty steps (15 × 7 × 12 sessions):
passed in 643 s, no state refused on reading, every finished state
checked.

### The certificate's design

`plan/notes/labelled-certificate.md`, for 66b. In short:
- The lemma states the sequent **valid over the system's frame class**
  (`valid C Γ Δ`, local consequence), the conditions as the closed set
  deduxit's `System` holds, so the trusted base is forcing and the frame
  conditions alone; a root that is no `w0 : Γ ⊢ w0 : Δ` is stated as
  labelled validity (`lvalid`).
- The proof term has a constructor per rule of the rule file, carrying
  the principal's parts and new labels only: the calculus keeps every
  principal formula, so a premise is the conclusion plus the rule's new
  items, and the checker `mcheck` needs no context matching. Theorems:
  `mcheck_sound` (to an inductive `g3`), `g3_sound` (to `lvalid`), `local`,
  and `modal_soundness`, the root's label a parameter, which a
  certificate applies with `vm_compute. reflexivity.`, as `soundness_ill`
  is.
- **It needs classical reasoning** as stated (`⊢ w0 : p ∨ ¬p` is
  derivable, and `valid` over `Prop` valuations is then excluded middle):
  the note recommends `Classical_Prop.classic` named in the assumptions,
  as the classical certificates over `Prop` already are, and gives the
  other way (a decidable or double-negation-stable forcing).
- The exporter (66c): `rocq::write` of a `Drawable::Rules` whose calculus
  is one of `modal::calculus`'s, the term read off the nodes' positions,
  a shared node under a `let`, the prelude `From Deduxit.Modal Require
  Import Check.`; a countermodel's certificate in the shape of
  `kripke_refutation`, if 66b takes it.
- **Where it lives**: the author chose `rocq-deduxit`, a module
  `Deduxit.Modal` (2026-11-16, told to this session through the plan);
  the note holds for a library of its own too.

### The page

65d (modal logic on the page) had landed when this chain was rebased
(`nqzpmrux`, "65d reviewed"), so the modal tab gains the builder here,
in the client (`../deduxit.github.io-66a`, change `pzvwxttr` on its `main@origin` `qtwlokpn`):
- Under a valid derivation, **"Prove it step by step"** (`modalSteps`)
  turns the tab into a calculus given as data: the rule file the
  bindings' `modal_calculus` gives (`G3S4`), the sequent the root of
  the valid derivation, every compound operand bracketed
  (`rootSequent`), started before the tab changes (a refusal is said in
  the modal panel, which stays as it was), then shown in the custom
  builder of 61b/62d, whose offered steps and value form take it from
  there.
- **No new binding**: in a sequent calculus a forward step is the same
  node as the backward step at the same members, and the builder's
  offered steps already list `L□` at its members, so the page needs no
  call over `extend`.
- Tests: `rootSequent` and real starts in G3T and G3K (a chain of `↔`,
  `∧` grouped left) in Vitest (`src/lib/modal.test.ts`), and the e2e test "a valid sequent is proved
  step by step in its system's labelled calculus" (`□p ⊢ p` in T: `Ref`
  with `x` given, `L□`, `ax`, checked whole).

### The checks

All on cores 7 to 9, capped.
- `cargo test --release -p deduxit --test rules`: 24 passed, the two
  new tests among them (the walks 34 s).
- The large sample of the walks (twelve walks of forty steps per
  sequent and system), once, by hand: passed in 643 s.
- `cargo test -p deduxit-cli --test readme`: the manual's three new
  sessions and the guide, 7 passed. `--test lock` with the new call
  `interact-calculus-labelled`, blessed in its own commit.
- `cargo clippy --workspace --all-targets -- --deny warnings`: clean;
  `-p deduxit --all-features` clean.
- The gate passed three times: before the reviewer's fixes, after the
  first round, and at the final chain.
- The client (`pzvwxttr`): Vitest 677 and lint, and its `nix flake
  check` under the machine lock at the final change: all checks passed,
  the e2e suite 314 passed.
- `nix flake check` of this chain under the machine lock: a first run
  failed on `treefmt` alone (a file changed from the shell, formatted
  since); at the final chain: all checks passed, the bindings' tests
  under Node among them. Only this report changed after it.

### The reviewer's findings

`fresh-reviewer` (Fable 5.2, medium), once, on the five changes. Five
findings, all answered:
1. **`derive` with a goal past the session's nodes panicked** in a
   calculus with contexts (`derive 7 L∧ 0`: index out of bounds, exit
   101): the command asked the goal's members before the library refused
   it. It now refuses an unknown goal first ("there is no open goal 7");
   the lock call starts with that step.
2. **`R¬` read forwards**, and `R→` where `B` stood right of `⊢`
   already: their premises add only before `⊢`. A forward step now also
   asks that the rule act on members before `⊢` alone, so every right
   rule is refused; the test above pins the set, and the manual, the
   guide, the changelog and the rules files say "acts on a member after
   ⊢".
3. **The note said soundness needs no classical reasoning**: false for a
   `Prop`-valued `valid` (`⊢ w0 : p ∨ ¬p` gives excluded middle). The
   note now says so and gives 66b the two ways, recommending `classic`
   named in the assumptions.
4. **The note's S4 record left out `serial`**, which the closed set has
   and `G3S4`'s `Ser` needs: fixed, with why.
5. **`modal_soundness` was fixed to the label `w0`**: the label is now
   its parameter (a session's root may be at any label), and the
   exporter's example passes it.

Its second round, on the fixes and the page: the five fixed. Four more
findings, answered in two fixes:
6. **The page's labelled text came from the typed text** (`labelled`):
   a chain of `<->` (the rule file's `↔` is non-associative) and an atom
   named `R` (the calculus's relation) were refused after the tab had
   already turned into a rules tab, the valid result lost; and `a /\ b
   /\ c` started a session on `a ∧ (b ∧ c)`, since the calculus groups
   `∧` right where the modal parser groups it left. The page now takes
   the valid derivation's root, fully bracketed, and starts the session
   before it changes the tab; a refusal (an atom named `R`) is said in
   the modal panel.
7. **The command's `derive` said no `added` line for a copy** in a
   multiset zone (LK's `CL`): it now counts members as multisets.

What it found sound: `extend` goes through `plan` and `commit`/`verify`
as `apply` does and answers the one premise; a refusal leaves nodes,
scopes and history as they were (an interned atom stays, documented);
members past the goal or in the other zone are refused; a given label
that is not new is refused by the checker; NJ's and NK's `derive` is
62b's, unchanged; the rule table matches Table II and 65's departures;
the note's account of the checker's freshness, the node positions and
DAG derivations is right.

### Departures

- **Forward steps cite the goal's members, not lines** (departs from the
  design, which said "62b's lines tried on the labelled rules"): tried,
  62b's lines do not exist in a sequent calculus, and a member is what a
  labelled hypothesis is. The relational atoms are cited as the
  labelled formulas are.
- **The certificate needs classical reasoning** (departs from the
  linear certificates' closed assumptions): recommended `classic`
  named, 66b decides.

### What the next sessions must know

- **66b (the Rocq proof)**: the note's sections 2 and 3 are the
  library's side, `Deduxit.Modal` in `rocq-deduxit` (the author's
  choice). Every rule keeps its principal formula and the contexts are
  whole zones (`modal::calculus`'s doc), which is what the checker's
  shape rests on; its departures from Table II (the axiom on any
  formula, `L¬`, `R¬`, `R⊤`, `L↔`, `R↔`, no initial sequent on a
  relational atom) each need their case in `g3_sound`. Negri's
  soundness theorem is to be read in her paper (its number is not in
  65's sources). The statement needs classical reasoning (the note's
  section 3): with `classic` named, the library's own assumptions check
  must let it through for the modal theorems, by name. The record a
  certificate states is the closed set (S4 marks `serial`), and the
  root's label is a parameter of `modal_soundness`.
- **66c (the exporter and the check)**: the note's section 4. The
  positions of a node follow the rule compiler's order
  (`core-modal.md`); labels are atoms of sort `label` with the names a
  search or a session gave (`w0`, `w1`, or `y`, `y1` in a session, since
  a session names a new label after the rule's variable). Derivations
  from a session are trees; a file may be a DAG.
- **The page**: the builder is in (above). A page that wanted forward
  steps worded as such ("from x : □p and x R y, derive y : p") would
  mark the offered steps whose rule reads forwards; no call is needed.
  The page writes the sequent in signs, so it does not wait for the
  client's lock to move past this chain.
- A refused step can still intern a new label's atom, so the next
  label made is numbered on (`y1` after a refused `R□`): cosmetic, noted
  in `core-modal.md`.

## Session 66b: the soundness proof

Fable 5.2 at `high` (a reused worker, its conversation cleared), from
the brief `/tmp/linlog-handoff/start-66b.md`, on 2026-11-16, cores 10 to
12. The work is in the Rocq library, in the jj workspace
`../rocq-deduxit-66b` (workspace `step66b`) on its `main@origin`
(`svrorwzs`, 0.4.0); deduxit has this report only.

### Outcome

- **`Deduxit.Modal` exists**, seven files under `theories/Modal/`, and
  every proof in it ends in `Qed` with its assumptions closed under the
  global context.
- **The soundness theorem holds for all fifteen systems at once**:
  `g3_sound : g3 C L Rt -> lvalid C L Rt`, for every set of frame
  conditions `C`, so for every system of the cube. The calculus `g3`
  is deduxit's rule file (`modal::calculus`) rule for rule, with its
  departures from Negri's Table II.
- **The checker and its theorem were reached**: `mcheck` is a function
  on proof terms, `mcheck_sound` gives `g3`, and `modal_soundness` is
  the theorem a certificate applies, concluding `valid C Γ Δ`, validity
  in the Kripke models of the system.
- **Finite countermodels too** (the design note left that to this
  session): `refutes` and `modal_refutation`, concluding
  `~ valid C Γ Δ`.
- **One finding about deduxit**: its checker accepts `Ser` with one new
  label on both ends of the edge (`w1 R w1`). The conclusion is still
  valid, so nothing deduxit answers is wrong, but the step reads as
  "seriality gives a world that sees itself". Below, under "For the
  author".

### The definitions

| file | holds | trusted by a certificate |
|---|---|---|
| `Formulas.v` | `formula atom`: `var`, `top`, `bot`, `neg`, `wedge`, `vee`, `imp`, `equiv`, `box`, `dia`, one constructor per node of `modal::Node` | yes |
| `Kripke.v` | `conditions` (the five frame conditions as Booleans), `Module System` with the fifteen systems as closed sets, `frame R C`, `forces R V w A`, `stable R V`, `valid C Γ Δ` | yes |
| `Labelled.v` | `label := string`, `item` (`lab x A`, `rel x y`), `holds R V ρ i`, `lvalid C L Rt` | for a labelled statement |
| `Calculus.v` | `mentions`, `fresh y L Rt`, the inductive `g3 C L Rt` with 22 rules | no |
| `Soundness.v` | `g3_sound`, `lvalid_valid`, `g3_valid` | no |
| `Check.v` | `formula_eqb`, `item_eqb`, `mem`, `freshb`, `proof`, `mcheck`, `mcheck_sound`, `labelled_soundness`, `modal_soundness` | no |
| `Countermodel.v` | `model` (`arrows`, `atoms`), `meets`, `forced`, `refutes`, `modal_refutation` | no |

- **A certificate states semantics, not derivability**, as 66a's design
  asks: `valid C Γ Δ` says that in every Kripke model whose relation
  meets `C` and whose forcing is stable, a world that forces all of `Γ`
  forces one of `Δ` (local consequence, the reading of `core-modal.md`).
  The rule table is not in the trusted base.
- **The systems** are `System.K`, `D`, `T`, `KB`, `KDB`, `B`, `K4`,
  `KD4`, `S4`, `K5`, `KD5`, `K45`, `KD45`, `KB5`, `S5`, each the closed
  set of `modal::System`'s constant of that name (T holds `serial`, S5
  all five). A generated certificate writes the record literal with
  `System::conditions()`; the two are convertible.
- **`g3` reads a sequent as two lists of items read as sets**: a rule
  asks membership of its principal items and gives each premise as the
  conclusion with the rule's new items added, which is what the rule
  file's kept principal formulas and whole-zone contexts make true of
  every instance. The two-premise rules keep the file's order.
- **A proof term holds no sequent**: one constructor per rule, with the
  labels and the parts of the principal formula, as in 66a's table
  (`Ax x A`, `LBot x`, `RTop x`, `LNeg x A p` … `LBox x y A p`, `RBox x
  y A p`, `Ref x p`, `Ser x y p`, `Sym x y p`, `Trans x y z p`, `Eucl x
  y z p`). `mcheck atom_eqb C p L Rt` starts from the sequent to prove.

### The theorems and their assumptions

- `g3_sound : forall C L Rt, g3 C L Rt -> lvalid C L Rt`.
- `lvalid_valid : lvalid C (map (lab x) Γ) (map (lab x) Δ) -> valid C Γ Δ`
  and `g3_valid`, their composition.
- `mcheck_sound : mcheck atom_eqb C p L Rt = true -> g3 C L Rt`.
- `labelled_soundness atom_eqb atom_eqb_spec C p : mcheck … L Rt = true
  -> lvalid C L Rt`.
- `modal_soundness atom_eqb atom_eqb_spec C x p : mcheck … (map (lab x)
  Γ) (map (lab x) Δ) = true -> valid C Γ Δ`.
- `modal_refutation atom_eqb m C w : refutes atom_eqb m C w Γ Δ = true
  -> ~ valid C Γ Δ`.

`Print Assumptions` answers "Closed under the global context" for each
of these seven and for `excluded_middle_stable` (below);
`tests/Assumptions.v` lists the eight and the library's check counts
the answers (45 with the 37 it had). The
only hypothesis in a statement is `atom_eqb_spec`, a `reflect` of the
atom equality, as in the other namespaces; `modal_refutation` needs
none.

- **How the proof goes.** By induction on the derivation, with the
  model taken inside the induction: no placing `ρ` of the labels makes
  every item on the left true and every item on the right false
  (`g3_model`). `box_r`, `dia_l` and `ser` move their new label to the
  world they need (`update`) and use `fresh` to leave every other item
  as it was.
- **A certificate of a valid sequent is three lines**, as 66a drew it:
  `apply (modal_soundness String.eqb String.eqb_spec C "w0" term).
  vm_compute. reflexivity.` A countermodel's is `apply
  (modal_refutation String.eqb {| arrows := …; atoms := … |} C w).
  vm_compute. reflexivity.`

### Departures from 66a's design note (`plan/notes/labelled-certificate.md`)

- **Forcing is stable by hypothesis, and no axiom is assumed (departs
  from the design's recommendation).** The calculus derives `⊢ p ∨ ¬p`,
  and with a valuation into `Prop` that needs every `V w p` decided, so
  classical reasoning is needed (the note's section 3, which leaves the
  choice to this session and recommends `Classical_Prop.classic`). The
  library's rule is that every theorem is closed under the global
  context, and the planning session named that rule as the one to keep.
  So `valid` and `lvalid` quantify over the models with `stable R V`
  (`~ ~ forces w A -> forces w A` for every world and formula), which
  is how Doczkal and Smolka define the models of classical modal logic
  in Coq ("Constructive completeness for modal logic with transitive
  closure", CPP 2012, section 2, read in this session).
  - With excluded middle every model is stable: `excluded_middle_stable
    : (forall P, P \/ ~ P) -> stable R V` is in `Soundness.v`, so a
    reader who assumes `classic` gets the textbook statement in one
    line, and one who does not sees exactly which models are covered.
    The statement with `classic` assumed follows from this one; the
    converse does not.
  - The proof uses stability in `wedge_r`, `imp_r`, `equiv_r` and
    `box_r`. The valuation stays `Prop`-valued and every forcing clause
    is the textbook's (`∨` is `\/`, `◇` is `exists`).
  - Weighed and not taken: `classic` as an axiom (the check would have
    to let an axiom through by name, and every modal certificate would
    rest on it); forcing clauses written negatively, which makes the
    trusted file harder to read; a Boolean valuation, which does not
    decide `□` over an infinite frame.
  - The certificate therefore imports no `Classical_Prop`, and its
    `Print Assumptions` is closed, as every other certificate's.
- **`lvalid` concludes "not all of `Rt` fail" (departs from the
  design)**, where `valid` concludes "one of `Δ` is forced". At one
  world the disjunction of `Δ` is a formula and stability gives the
  positive form; for items at several worlds stability of each does not.
- **The files** differ from the note's table in three names, none of
  which the exporter sees (it imports `Check` alone, which exports the
  rest): `label`, `item`, `holds` and `lvalid` are in `Labelled.v`, so
  that the trusted base of an ordinary certificate is two files without
  strings; `proof` is in `Check.v` beside its checker, as
  `IntuitionisticCheck.v` has it, so there is no `Proofs.v`; the note's
  `local` is `lvalid_valid`. `modal_soundness atom_eqb atom_eqb_spec C x
  p` has the note's arguments in the note's order.
- **`Ser` asks no more than the rule file** (below); the note's table
  is otherwise followed constructor for constructor.
- **The systems sit in `Module System`**, so importing the library does
  not define `K`, `T` and `B` at top level.
- **The calculus has both zones as lists of items**, as the note and
  the rule file have; a relational atom on the right is never principal.

### The Ser instance, and why the proof has a second model

deduxit's generic checker binds `x` and `y` of `Ser` from the premise's
new member and asks only that `y` occur nowhere in the conclusion. So it
accepts a premise `w1 R w1` for a new `w1`:

```
$ deduxit check --calculus G3D same.json
valid in the calculus given (G3D): w0 : p ⊢ w0 : p, a derivation of 2 nodes
  0. w0 : p, w1 R w1 ⊢ w0 : p    ax
  1. w0 : p ⊢ w0 : p    Ser 0
```

(the file written by hand in this session; `w0 R w1` passes too and
`w1 R w0` is refused, as it should be). That instance does not keep
validity inside one model: a serial frame may have no world that sees
itself. It is still sound for what the calculus derives, because a
world that sees itself alone can be put beside any model without
changing what the old worlds force, and all five frame conditions
survive that. `Soundness.v` proves exactly this (`beside_R`,
`beside_frame`, `beside_forces`, `beside_stable`), so `g3`'s `ser` has
the rule file's condition and no other, and every derivation deduxit's
checker accepts in a `G3…` calculus has a term `mcheck` accepts.

### Checks

- **The library's `nix flake check`**, under the machine lock on the
  final tree: "all checks passed", 14 checks, among them
  `rocq-deduxit` (Rocq 9.1) and `rocq-deduxit-9_0`, `-9_2`, `-9_3` (the
  library built, every test under `tests/` compiled in silence, the 45
  `Print Assumptions` lines each answered "Closed under the global
  context"), `rocq-bridges`, `conventions`, `typos`, `treefmt`.
  - The first full run failed on 9.2 and 9.3 alone: the tests named
    their atoms by `Notation p := (var "p")`, which warns there, and a
    test must print nothing. They are definitions now (a rules bullet
    says so).
  - `typos` read `Ser` as a misspelling; `ser` is a word of
    `typos.toml` now.
- **The tests** (`tests/Modal/`, compiled as `Deduxit.Modal.Tests` by a
  third loop of the check): `Certificates.v` has every one of the 22
  rules at least once, a certificate per frame rule (T, D, KB, S4, K5),
  the system once as a record literal, a labelled sequent, the Ser
  instance above, and `mcheck … = false` for a frame rule the system
  lacks, a label that is not new at `R□`, at `L◇` and at `Ser`, a
  missing edge, a missing principal formula and an axiom at two labels;
  `Countermodels.v` has a model for K, T, KD45 and S5 and one sequent
  with every connective, and `refutes … = false` for each frame
  condition, an arrow and a world out of the model, and a world that
  forces the wrong side.
- **The documentation** (`nix build .#doc`, `rocq doc` over every
  module): built, with a page for each of the seven modules of
  `Deduxit.Modal`.
- **deduxit against the new library**: `nix build
  .#checks.x86_64-linux.rocq-deduxit
  .#checks.x86_64-linux.rocq-deduxit-library --override-input
  rocq-deduxit git+file:///home/tux/Projects/own/rocq-deduxit-66b` in
  this workspace (on `main`, `tlzmomru`), under the machine lock: both
  built, "75 certificates, each closed under the global context". The
  exporter's existing certificates are untouched by the new namespace.
  (`git+file:`, not `path:`: the workspace holds the in-place build's
  `.vo` files, which `path:` would copy into the source.)
- **deduxit's checker on the Ser instance**: run by hand with the
  workspace's debug build, output above.
- Not run: `scripts/opam-install.sh` (network, about an hour; the opam
  file's dependencies did not change).

### The reviewer's findings

`fresh-reviewer` on Opus 5.5 at `high` (never the writer's model), once,
on the three changes, against the brief's criteria: every rule sound and
the rule file's, the statement of soundness the right one, no assumption
left open, the checker accepting only derivations, the tests and docs.

- **It found nothing that breaks correctness or a stated requirement.**
  It compared the 22 constructors of `g3` with `form` (principal items,
  new items, premise order, freshness, a frame rule exactly where the
  system has the condition), recomputed the fifteen records against
  `closed()`, checked the forcing clauses and the five conditions
  against the textbook, the quantifier order of `valid` and `lvalid`,
  that `stable` is no vacuous hypothesis (every finite model meets it),
  the `beside_R` argument, and traced each refusal test to the side
  condition it names. It compiled no probes; the proofs it takes from
  the kernel.
- **One remark, no change**: `valid` quantifies over worlds in one
  universe (`world : Type` at the definition's level). Finite models
  suffice for these logics, so nothing is lost.
- **One run it asked for** (deduxit's checker on `Ser w1 w1`) had been
  made already; its output is above.
- It read `excluded_middle_stable` in the working copy before it was
  committed and found it correct; the lemma and the tests' definitions
  came after its review of the three changes.

### The library's commits (change ids, in `../rocq-deduxit-66b`, on `svrorwzs`)

- `lmxkyrxm` Add Deduxit.Modal: the labelled calculus G3K, its soundness
  for Kripke semantics, proof terms with their checker, finite
  countermodels (the seven theories, `_RocqProject`, `typos.toml`).
- `mnvpnktw` Check the modal certificates, the countermodels and the
  modal theorems' assumptions (`tests/Modal/`, `tests/Assumptions.v`,
  `modules/rocq-deduxit.nix`).
- `ztysvovp` Describe Deduxit.Modal in README, the changelog, the
  package and CLAUDE.md (and the rules bullets of
  `.claude/rules/rocq.md`). The head.

Nothing is pushed. The workspace `step66b` (`../rocq-deduxit-66b`) is
left for the planning session to push from and remove.

### For the author, through the planning session

- **`Ser` with one new label for both ends.** Sound, as above, but a
  student can write the step "`w1 R w1` by seriality" in a session and
  the page will accept it. The rule file cannot say `x ≠ y`; a `fresh`
  on `x` would refuse the wanted instances (`x` in the conclusion). A
  fix would be a condition of the rule language ("these two variables
  differ") or a check in the modal session; neither is needed for
  soundness. Not changed in this session.
- **The stability hypothesis** is the one place where a certificate's
  statement differs from the textbook's on its face. It is documented
  at the top of `Kripke.v` and in README.

### What the next sessions must know

- **66c, the exporter.** Write `From Deduxit.Modal Require Import
  Check.` (and `Countermodel` for a model), the statement `valid (atom
  := string) {| serial := …; reflexive := …; symmetric := …; transitive
  := …; euclidean := … |} [Γ] [Δ]` from `System::conditions()`, and
  `apply (modal_soundness String.eqb String.eqb_spec {| … |} "w0"
  term).` A root that is not at one label, or has a relational atom,
  states `lvalid C L Rt` and applies `labelled_soundness String.eqb
  String.eqb_spec C term`.
  - The term's constructors take the principal label, then the edge's
    other label or the new label, then the formula's parts, then the
    premises in the rule file's order; `Trans x y z` adds `x R z` from
    `x R y` and `y R z`, `Eucl x y z` adds `y R z` from `x R y` and
    `x R z`, `Sym x y` adds `y R x`.
  - The checker walks the term as a tree: a `let` shares the text, not
    the work, and a shared node is checked once per use against that
    use's sequent, which is fine since a premise is determined by its
    conclusion.
  - A countermodel: `{| arrows := [[…]; …]; atoms := [[…]; …] |}` in
    the model's world order, the arrows as deduxit holds them
    (`core-modal.md`: nothing is closed), the refuting world's number
    last: `modal_refutation String.eqb m C w`.
  - The flake check of the certificates is 66c's: nothing in deduxit
    changed here, and the lock still pins `rocq-deduxit` at 0.4.0.
- **The planning session.** Push `rocq-deduxit` first
  (`plan/push-main.sh --rocq`, the head below), then move deduxit's
  lock (`nix flake update rocq-deduxit`). The library's version is
  still 0.4.0 with an `[Unreleased]` section in its changelog; deduxit's
  `export::rocq::DEDUXIT` must equal the opam file's version at the
  release.
- **The release.** `rocq-deduxit.opam`'s description and tags,
  `CITATION.cff` and README already describe `Deduxit.Modal`; the
  changelog's `[Unreleased]` section becomes the release's. `doc.nix`
  needs nothing (it builds every module of `_RocqProject`).
- **The library's rules** (`.claude/rules/rocq.md` there) hold the
  invariants: the trusted base, why forcing is stable, the two forms of
  the conclusion, `g3` against the rule file, the Ser instance, the
  proof term, the finite models over `{ w | w < size }`.

## Session 66c: the certificates

Opus 5.5 at `high`, the conversation of 54x6 and 66a, in the jj
workspace `../deduxit-wt-step-58a-610ed3`, from the brief
`start-66c.md`, on 2026-11-16, cores 7 to 9. The chain on `main@origin`
`uwnktrmt`:

| change | what |
|---|---|
| `utvppnwm` | the writer (`core/src/modal/rocq.rs`), `export::rocq::write_labelled` and `write_modal_countermodel`, two preludes, the snapshots `deduxit/modal_*.v` |
| `yuuznuly` | the command: `prove --system --format rocq`, `check --format rocq` of an outcome, `check --calculus G3… --format rocq`, a session's `proof --rocq [FILE]` |
| `uvxlklvu` | the behaviour lock: `modal-rocq`, `modal-rocq-countermodel` |
| `vurkosxr` | the flake: `rocq-deduxit` compiles the command's modal certificates, `check-certificate` checks one, both refuse forged ones |
| `rxvwtryy` | the Rocq chapter's "Certificates of modal logic", the modal chapter, the guide, the changelog |

### The exporter

- **A valid sequent** (`write_labelled`, from a `rules::Derivation` of a
  `G3…` calculus): the system is read from the calculus's name and the
  derivation checked in that system's calculus first (a user's file
  named `G3S4` is judged by the built-in one). A root whose items are
  all formulas at one label `x` is stated as `valid (atom := string) C
  [Γ] [Δ]` and proved by `modal_soundness String.eqb String.eqb_spec C
  "x" term`; any other root, a relational atom or two labels, as
  `lvalid C [items] [items]` by `labelled_soundness`. `C` is the record
  of `System::conditions()`, the closed set (T writes `serial := true`).
- **The term**: a constructor per rule, the 66b order (the principal
  label, then the edge's other label or the new label, then the
  principal formula's parts, then the premises in the rule file's
  order). Each node's members are read from its positions: `shape` says,
  per rule, which zone each position points into (the conclusion's left
  or right, or the first premise's left for the new edge of `R□`, `L◇`,
  `Ser` and `Ref`), in the rule compiler's order, and the item's
  structure gives the labels and parts. Every node's term is in
  brackets. The derivation is written as a tree, a node with two parents
  once per use (the reviewer's finding below: a `let` shared the text,
  not the checker's work).
- **An outcome** (`write_modal_outcome`, which the command calls for
  both verdicts): a valid one is checked (`Outcome::check`, the root is
  the sequent at `w0`) and stated as the sequent the outcome decided,
  repeated formulas included, which the derivation's root (a set) holds
  once.
- **A countermodel** (`write_modal_countermodel`): checked, then `~ valid
  C [Γ] [Δ]` from the modal arena, proved by `modal_refutation
  String.eqb {| arrows; atoms |} C w`, the arrows as the model holds
  them, the model's world.
- **Both** write into a text that refuses past `limits.derivation_bytes`
  before anything is written, ask the stop per node and per world, and
  walk terms and formulas on a stack (`Stops`), never by recursion. The
  kernel option is ignored, as for the ordinary certificates: only
  deduxit's library has modal logic. Preludes `DEDUXIT_MODAL_PRELUDE`
  (`Check`) and `DEDUXIT_MODAL_COUNTERMODEL_PRELUDE` (`Countermodel`).
- **Refused**: a derivation of another calculus (`Unsupported::Calculus`,
  whose message now names the modal calculi as the ones that have
  certificates), one with an open goal (`Unsupported::Open`), one that
  rests on an assumption (`Unsupported::Assumption`, new, last of the
  enum), one the checker refuses (its error), and a text past the bound
  (`Refusal::Output`).

### The command

- `prove --system S --format rocq` writes the certificate of either
  verdict after the verdict as a comment (`calculus::shown` and
  `countermodel_shown` gained a Rocq arm).
- `check --format rocq` of a modal outcome writes the certificate of
  evidence that holds, the verdict alone on standard error otherwise;
  `check --calculus G3S4 --format rocq` of a derivation file, through the
  drawing path, and "no certificate: …" for another calculus.
- A calculus session's `proof --rocq [FILE]` writes it, under the
  session's styles, which the session now reads as the linear one does
  (`style::read`: `--style rocq.form=standalone`, `--lemma`, `--prelude`;
  before, `--style` did not reach a calculus session).
- `check-certificate` already read `From Deduxit.Modal` as rocq-deduxit
  (a word match on `Deduxit`); its check now accepts a modal certificate
  and refuses one whose `Trans` names its edges wrongly.

### The checks

All on cores 7 to 9, capped.
- `cargo test -p deduxit --all-features --test export modal_certificates`:
  the thirteen snapshots (a valid sequent per frame rule, one of every
  connective, a shared node, a labelled root from a session,
  countermodels in K, T, S4, KD45 and S5), a derivation of another
  calculus and a bound refused.
- The snapshots compiled by hand against the pinned library, then the
  command's: `prove` in T and K, a session's `proof --rocq`, `check` of a
  valid T outcome and of an S5 countermodel; `Print Assumptions` closed;
  a `Trans` on the wrong edges and a record with `transitive := false`
  refused.
- `nix build .#checks.x86_64-linux.rocq-deduxit
  .#checks.x86_64-linux.check-certificate` under the machine lock: both
  passed, "100 certificates, each closed under the global context", the
  modal certificate accepted and the forged ones refused; again after the
  reviewer's fixes, 101 (`modal_repeated.v` new).
- `cargo test -p deduxit-cli --test readme` (the chapters' new
  examples) and `--test lock` (blessed in its own commit): passed.
- The gate passed before the reviewer's fixes and after them.
- `nix flake check` at the report's commit under the machine lock:
  all checks passed. Only this report changed after it.

### The reviewer's findings

`fresh-reviewer` (Fable 5.2, medium), once, on the five changes. It
compiled 285 certificates the command wrote (19 sequents in all fifteen
systems, 160 valid and 125 not valid), each in agreement with the exit
status and with the closed set of conditions, the thirteen snapshots,
and hand-written derivations (a shared node, permuted zones, labelled
roots, `Ser u u`, odd names); every forged term or model was refused.
Three findings, all answered:
1. **A `let` hid exponential work**: the library's `mcheck` walks a
   shared term once per use, so a chain of shared `L∨` nodes (both
   premises the conclusion itself, in a set zone) cost 2ⁿ in Rocq: 42 s
   at 20, past two minutes at 60, from a certificate the command wrote
   at once. The derivation is now written unfolded, so the text's bound
   (`limits.derivation_bytes`) is also the check's.
2. **A valid statement dropped repeated formulas**: `q, p, q ⊢ p, r, p`
   was stated `[q; p] [p; r]`, from the root's set zones; equivalent,
   but not the sequent as written. A modal outcome is now stated from
   its own sequent (`write_modal_outcome`).
3. **An assumption was refused as an open goal**: the new
   `Unsupported::Assumption` says what it is.

Its note: a formula nested 30 000 deep writes a certificate on which
Rocq's own parser overflows its stack (2 000 compiles); Rocq's limit,
not the writer's.

### Departures

- **The kernel option is ignored for modal certificates** (as for the
  ordinary ones): only deduxit's library has a modal calculus, so
  `--kernel nanoyalla` gets the library's certificate rather than a
  refusal.
- **`check --calculus G3… --format rocq`** goes beyond the brief (which
  named `prove` and the session): the drawing path took it for free.
- **No `let` for a shared node** (departs from the design, which wrote
  one): the review showed it hides the checker's work.

### What the next sessions must know

- **67**: modal certificates are written by `export::rocq::write_labelled`
  and `write_modal_countermodel`; a new frame condition is a field of the
  library's `conditions`, a row of `conditions()` in `modal/rocq.rs`, a
  rule of `shape` and a constructor of the library's `proof`, together.
- **The release of 0.6.0**: deduxit's lock pins `rocq-deduxit` with
  `Deduxit.Modal` at its version 0.4.0 (unreleased there). The library is
  released first and `export::rocq::DEDUXIT` moved to its version with
  the lock, as `rocq-deduxit.nix` asserts; the manual's Rocq chapter
  names no version. The changelog's "Certificates of modal logic" entry
  is under Unreleased.
- **The certificate of a large derivation** is held whole before it is
  written (within `limits.derivation_bytes`), unlike the linear writers,
  which flush per inference; a modal derivation's size is already
  estimated and bounded when it is made.
- **A new public item each**: `rocq::write_labelled`,
  `write_modal_countermodel`, `write_modal_outcome`, the preludes
  `DEDUXIT_MODAL_PRELUDE` and `DEDUXIT_MODAL_COUNTERMODEL_PRELUDE`, and
  the variant `Unsupported::Assumption` (last, as the rule for fieldless
  enums asks): the release's break check lists them as additions.
