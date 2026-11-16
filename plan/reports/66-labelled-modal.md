# Step 66: the labelled modal calculus step by step, and its soundness in Rocq

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
