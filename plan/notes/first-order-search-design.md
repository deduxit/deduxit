# First-order proof search: the design (step 75)

Written 2026-11-21 by session 75a, before its code. It settles how
deduxit decides first-order classical, intuitionistic and minimal
sequents: through the embeddings first, then native connection provers,
the decidable fragments, the bounds and "unknown", the dispatch, and the
comparison with iLeanCoP and nanoCoP. Each decision names its sources
and the alternatives weighed. Where the code differs,
`plan/reports/75-first-order-search.md` says so.

Sources read for it (the papers and the provers' code were read by
the session's research sub-agents, whose notes and downloads are in the
session's scratch directory; the citations below are theirs, checked
where a decision rests on them):
- the plan: `plan/75-first-order-search.md`, `plan/README.md` (D17,
  D19, D24, D25), `plan/notes/first-order-design.md` (sections 3, 5, 10),
  the reports of steps 42, 54, 54s, 55 and 74, `more-logics.md`
  (more-logics-6), the spec's "First-order fragments";
- the code: `core/src/ordinary/translate.rs`, `derivation.rs`
  (`read_back_with`), `decide.rs`, `core/src/linear/search/` (the
  dispatch, `focus/framed.rs`), `linear/proofs/` (the first-order proof
  nodes, `Instances`, the derivation view's members);
- Otten, "leanCoP 2.0 and ileanCoP 1.2" (IJCAR 2008); "Restricting
  backtracking in connection calculi" (AI Communications 23, 2010);
  "nanoCoP: A Non-clausal Connection Prover" (IJCAR 2016); "The
  nanoCoP 2.0 connection provers" (TABLEAUX 2021, paywalled: its content
  read from the AReCCa 2023 overview and the shipped code); Otten,
  "ileanCoP: a connection calculus for intuitionistic logic"
  (TABLEAUX 2005); Otten and Kreitz, T-string unification (TABLEAUX
  1996); the provers' sources at leancop.de (leanCoP 2.1(b),
  ileanCoP 1.2, nanoCoP 2.0, nanoCoP-i 2.0, 2021-05-01);
- Kreitz and Schmitt, "A uniform procedure for converting matrix proofs
  into sequent-style systems" (Information and Computation 162, 2000);
  Reis, "Importing SMT and connection proofs as expansion trees"
  (PxTP 2015); Kunze (arXiv 1606.05948);
- the decision problem: TUM's "Decision Problem" slides (the finite
  model property of the monadic class, the Bernays–Schönfinkel class's
  finite Herbrand universe); Rybakov and Shkatov (arXiv 1706.05060:
  intuitionistic monadic undecidability, Maslov–Mints–Orevkov 1965,
  Gabbay 1981); Schubert, Urzyczyn and Zdanowski (arXiv 1610.02675:
  Mints' Π1 and Σ1 decidable, Orevkov's undecidable Π2, Σ2, Δ2); the
  ILTP paper (TABLEAUX 2005); CASC's EPR division.

## 1. What a user gets

- `prove --logic L` and `ordinary.decide` answer a first-order sequent:
  "valid" with a derivation of LK, LJ or LM that the checker accepts,
  "not valid" only with something checked (a finite structure, later a
  Kripke model), "unknown" at a bound with the bound named.
- First-order validity is undecidable (Church 1936, Turing 1936), so
  "unknown" is the honest answer outside the decidable fragments.
- A propositional sequent goes the way it goes today: every first-order
  path starts behind `Formulas::is_first_order` (the author's "not
  overwhelming", D17).

## 2. Through the embeddings (75a)

**Decision: the quantifier rows copy the rows of the connective of
their polarity: `∀` takes `∧`'s row and `∃` takes `∨`'s, with one
operand.** Girard's translations treat `∀` as an infinite `&` and `∃`
as an infinite `⊕`: call by name `(∀x.A)* = ∀x.A*`, `(∃x.A)* =
∃x.!A*`, as `(A ∧ B)* = A* & B*`, `(A ∨ B)* = !A* ⊕ !B*` (Girard,
"Linear Logic", 1987, §5; Danos, Joinet and Schellinx's analysis of
the translations keeps the analogy). The table's other translations get
their rows by the same analogy:
- classical affine (MALL): `∀` as `∀` on the right and `∃` on the left,
  `∃` the dual, no `!`, the side's function passed to the body;
- call by value: `∀` with `∧`'s row (the operand's `!`s as `∧`'s), `∃`
  with `∨`'s;
- 0/1 (Liang and Miller): the rows of `∧` and `∨` under `T0` and `T1`.
  LLTP's translation, whose propositional rows the table copies, has no
  quantifier rows ("Propositional, for now", `tptpparser/
  translation.ml`), so here the analogy decides (a question the prompt
  does not settle, decided by Girard's analogy).
- A wrong row cannot give a wrong verdict: every proof found is read
  back and checked, and the read-back fails where an image's proof is no
  proof of the sequent. It could only lose proofs, which the
  differential tests against the native engines (75b, 75c) show.
- Alternatives weighed: Gödel–Gentzen's negative translation first and
  then the intuitionistic embedding for classical logic (one more layer
  for nothing the affine MALL image lacks); a translation of its own for
  quantifiers (no source).

**The image's first-order tables are the ordinary arena's, copied.** The
linear and the ordinary arenas hold the same `crate::first_order`
tables (74b's move), so the image's `fo` is the ordinary tables' symbols,
terms and argument lists verbatim, its atoms' predicates taken per
linear atom as the image names them, and its binders' names per linear
binder from the ordinary binder it images. Sequent-range term ids
then mean the same term in both arenas, which the read-back relies on.
The extra atom `false` of minimal logic is a nullary predicate of the
image.

**The search is step 42's.** The focused engine decides `LL1` one-sided
(the classical affine image) and two-sided (the intuitionistic images),
with its first-order bounds: the copy bound, `--witness-depth`,
`--instances`. Nothing in the engine changes.

**The read-back** maps the linear quantifier rules to the ordinary ones
by side: an output `∀` is `∀R` and an input `∀` is `∃L` (the eigenvariable
rules), an output `∃` is `∃R` and an input `∃` is `∀L` (the witness
rules), as the classical view already names them (`ClassicalRule`).
- A member of the linear derivation past the forest is an instance
  `(occurrence, frame)`; its ordinary formula is the occurrence's tag
  instantiated by the frame's terms, innermost binder first
  (`Formulas::instantiate`, 74c), so the derivation's sequents hold
  instances as nodes, which the checker compares by id.
- The witness of `∃R`/`∀L` and the eigenvariable of `∀R`/`∃L` is the
  term the premise's body member pushes on its frame. The proof's
  instance table's terms (sequent-range ids, and the table's own
  extension with `Eigen`) are interned into the ordinary arena, an
  eigenvariable as `FoTerm::Eigen` with its number.
- The checker then judges every inference, the eigenvariable condition
  included: a read-back that breaks it is a bug (`Error::ReadBack`),
  never an answer.

## 3. The native classical engine (75b)

**Decision: a non-clausal connection calculus, nanoCoP's (IJCAR 2016,
Fig. 2), with regularity, lemmata and iterative deepening on the
path's length; restricted backtracking (`cut`, `comp(n)`) an option,
off by default.**
- Why non-clausal: the proof stays in the formula's own structure, so
  its read-back needs no map from definition atoms back to subformulas.
  Reis imported only 1 224 of 3 043 leanCoP proofs through a
  definitional form (PxTP 2015), and Otten's own conclusion (IJCAR 2016)
  is that clausal and definitional forms make the translation back "not
  straightforward". nanoCoP also proves more than leanCoP's definitional
  form on TPTP 3.7.0 (1 232 and 1 485 with `[cut,comp(6)]` against
  1 065 at 100 s, IJCAR 2016 Table 3).
- Restricted backtracking is incomplete (AICOM 2010, Lemma 5), so the
  default keeps completeness and a "not valid" from an exhausted search
  of a ground problem (leanCoP's `pathlim` flag: a run that refused no
  extension for depth was exhaustive).
- Skolemization as the eigenvariable condition: δ-formulas get Skolem
  terms over the free variables of their subformula; the read-back turns
  each Skolem term into an eigenvariable.
- **Read-back to G3c**: the connections and the substitution are an
  oracle for a deterministic construction of a G3c derivation on the
  original sequent (the order of the rules from Kreitz and Schmitt's
  reduction ordering, the β-split pruning theirs), every inference then
  checked by the derivation's checker. A proof that does not read back is
  a bug, never an answer.
- Alternatives weighed: leanCoP's clausal calculus (simpler search,
  harder read-back, fewer proofs); a free-variable tableau (the same
  information, more branching); the inverse method (no read-back to G3
  without an extraction of its own).
- Licences: leanCoP, nanoCoP and ileanCoP are GPL; deduxit is EUPL. The
  engine is written from the papers, never from their code.

## 4. The native intuitionistic engine (75c)

**Decision: nanoCoP-i's prefixed non-clausal connection calculus**
(Otten, AReCCa 2023 §3 and §5; the shipped `nanocopi20.pl`): the
classical connection search with a prefix per literal, prefix equations
collected per connection, the domain (admissibility) condition, and
T-string unification (Otten and Kreitz 1996, ten rules) checked after
each connection, which prunes earlier than nanoCoP-i's check at the end.
- ileanCoP 1.2 is clausal and prints no proof (its ReadMe; Kunze §3.1),
  so it is a comparison, not a model.
- **Read-back to G3i**: Kreitz and Schmitt's procedure produces LJ from
  the connections, `σQ` and `σJ`; as for LK, every inference is checked.
  Its worst case is exponential (their §3.2, §5): the read-back keeps the
  derivation's size bound and answers a refusal past it, as every
  read-back does.
- **Minimal logic**: no connection calculus for minimal logic was found
  (the research's searches of the leanCoP papers and Wallen, Kreitz and
  Otten 1999). The plan: `⊥` an ordinary atom and `¬A` as `A → ⊥`, with
  no `$false` clause, as the propositional suite treats minimal logic;
  untested in the literature, so it stays behind a differential test
  against the embedding's minimal translation until it agrees on the
  generated and the ILTP problems, and minimal logic goes through the
  embedding until then.

## 5. The decidable fragments (75d)

**Classical logic** (sources: TUM's slides; the CASC EPR division):
- **The class**: a sequent `Γ ⊢ Δ` is valid iff `Γ ∧ ¬Δ` is unsatisfiable;
  Skolemizing it turns the strong quantifiers (`∀` on the right, `∃` on
  the left) into Skolem symbols. The sequent is **EPR** when no strong
  quantifier stands in the scope of a weak one and there is no function
  symbol of arity above 0: every Skolem symbol is then a constant, the
  Herbrand universe is finite, and grounding over the constants (one if
  there are none) gives a propositional problem for the SAT solver.
- **The monadic class** (no function symbols, no equality, every
  predicate unary) has the finite model property with at most 2^k
  elements for k predicates (Löwenheim, Behmann 1922); miniscoping turns
  it into the EPR form, so it is decided the same way.
- **"Not valid"** is a finite structure (a domain `0..n`, each constant's
  element, each predicate's true tuples) under which `Γ` holds and `Δ`
  fails, checked by an evaluator over the sequent (cost `|φ|·n^q`, `q`
  the quantifier depth). Step 76 owns the countermodel's type and its
  check; 75d writes its fragments' models in that type.
- "Valid" in the fragment is a derivation as everywhere: the SAT
  solver's refutation of the ground problem is a clausal proof, and the
  verdict stands with the clausal proof checked, as the propositional
  classical SAT path's does; a derivation of LK comes from the
  connection prover or the embedding where they find one.
- Ackermann's and Gödel's classes are later: they need a finite-model
  search, not a grounding.

**Intuitionistic logic**:
- The monadic fragment is undecidable intuitionistically
  (Maslov–Mints–Orevkov 1965; Gabbay 1981), and so are Orevkov's prenex
  Π2, Σ2 and Δ2 classes; Mints' Π1 (no negative quantifiers) and Σ1 (no
  positive ones) are decidable but costly (co-2-NEXPTIME-hard,
  EXPSPACE-complete; Schubert, Urzyczyn, Zdanowski).
- **Decision**: a classical "not valid" is an intuitionistic and a
  minimal one (every intuitionistic theorem is a classical one), so the
  classical fragments answer for all three logics; a classically valid
  sequent in an undecidable fragment stays "unknown" until step 76's
  bounded Kripke search finds a checked model (a model found is
  evidence; an exhausted bounded search is no answer). Mints' deciders
  are no part of step 75.

## 6. Bounds and "unknown"

- The embedding's search ends as step 42's does: `Reason::CopyBound`,
  `WitnessDepth`, `Instances`, the memory and work limits, the stop; its
  unknown keeps the linear reason, so the command's advice names the flag
  (`--copies`, `--witness-depth`, `--instances`).
- The connection provers deepen the path's length; their unknown names
  the deepest length searched (a new reason, `PathLength { length }`,
  for 75b), or the time limit.
- A refused read-back (the derivation's size) is the refusal it is
  today, the verdict "unknown" with the bound named, never "valid"
  without a checked derivation.

## 7. The dispatch

- `decide` at first order: the method given, else `Method::default_for`'s
  first-order default. In 75a that default is the embedding (the only
  first-order method); from 75b the connection prover for classical
  logic, from 75c for intuitionistic logic, each where a measurement on
  ILTP's first-order problems shows it ahead (D19), the embedding the
  fallback where the native one gives way at a bound.
- The SAT solver, LJT, the truth table and the tableau keep refusing a
  first-order sequent (`Error::FirstOrder`), except the SAT solver's
  grounding inside the EPR decider (75d).
- The command's `--method` and the protocol's `ordinary.decide` take the
  same names; a new method (`connection`) is a `Method` variant.
- **The embedding answers "valid" or "unknown" at first order, never
  "not valid".** The classical affine image has no contraction, and
  G3c needs one at first order (`∃R` and `∀L` keep their principal: the
  drinker's `⊢ ∃x. (p(x) → ∀y. p(y))` uses its witness twice), so that
  image loses proofs, and an unprovable image says nothing about the
  sequent. The intuitionistic images have `!` where the copies go, but
  the linear refutations do not apply at first order (`Disproof::check`
  answers `NotApplicable` there), so no refutation of the image can be
  checked either. A linear "unprovable" at first order is therefore the
  ordinary "unknown", its reason saying the image was refuted within the
  engine's bounds; "not valid" comes from the fragments (section 5) and
  step 76's models.
- Classical first-order logic through the affine image is incomplete
  for the same reason; the default for classical logic is the embedding
  only until 75b's connection prover, and the report measures what the
  affine image proves of ILTP's classically read problems. (A classical
  embedding with exponentials, `?!`-style, was weighed: no row of the
  table has one, and the connection prover is the planned answer.)

## 8. The comparison

- iLeanCoP 1.2 and nanoCoP-i 2.0 (and leanCoP 2.1 for classical logic)
  are fetched by the flake at their published tarballs, pinned by hash,
  never vendored (GPL), run on SWI-Prolog as the comparison's other
  Prolog provers are (`modules/_provers/`), each a `tools::Driver` with
  its ILTP reading (`--tptp` with `--tools`). Their verdicts are read
  from their output lines ("is an Intuitionistic Theorem", "Non-Theorem",
  `Timeout`) and their exit statuses (0, 1, 2).
- Their published ILTP figures (ileanCoP 1.0: 690 of 2 550 at 600 s;
  nanoCoP-im 2.0: 788 proved and 88 refuted at 10 s) are context, not a
  baseline: the measure is the same machine and limit for every tool.

## 9. What 75a builds

The embedding's quantifier rows and its first-order tables, the
read-back of the focused engine's first-order proofs to LK and LJ with
the witnesses and eigenvariables, and `decide` answering a first-order
sequent through the embedding: "valid" with a checked derivation,
"unknown" at a bound. 75b to 75d build the rest of sections 3 to 8.
