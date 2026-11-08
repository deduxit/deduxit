# Neighbouring logics of linear logic, updated: research note for what to add after 0.2.0

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research
sub-agent; sources read that day. This updates
`plan/notes/research/scope/logics.md` (written 2026-10-09 to 17, "the
old note"), whose candidates keep their names `logics-1` to `logics-8`;
new ones continue at `logics-9`. The old note's section 1 (the field) and
its 36 source tags are not repeated: only what a re-read changed, and
what is new, is written here. "(inference)" marks this note's own
conclusions; "(unsourced)" a claim for which no page could be read.
The fixed-point candidate `logics-5` is treated in full by
`plan/notes/research/next/cyclic-proofs.md` (cyclic-proofs-3 restates it);
this note only cross-references it.

## 1. What linlog has today

From `docs/src/status.md` (the manual's "What exists and what is planned")
and the code, as of 0.2.0:

- **None of logics-1 to logics-8 is built.** `docs/src/status.md` names no
  subexponential, graded, relevant, light, adjoint, BV or fixed-point
  feature. Its "Planned" list holds only certified refutations, first-order
  proving on the web page and the comparison.
- **What changed in their prerequisites** since the old note:
  - First-order terms exist: quantifiers, predicates with arguments,
    function symbols, eigenvariables, unification in the focused and
    two-sided engines (`docs/src/first-order.md`, status). `Term` has
    `Forall`/`Exists` as locally nameless binders over first-order terms
    (`core/src/sequents/term.rs`). Binders over formulas (the need of
    logics-5) do not exist.
  - Cut and cut elimination exist, on terms and on nets, with boxes, in
    every commutative mode and at first order (`docs/src/cut.md`; status).
    Any new modality's rules now also need a cut-elimination case, a cost
    the old note did not count (inference).
  - Nets for MELL with boxes at every depth, with Mix, and for
    intuitionistic MELL, sequentialization through boxes and a drawing
    with every box a rectangle (`docs/src/proof-nets.md`; status). A box
    per subexponential label would hang on this (inference).
  - The Rocq library `rocq-linlog` is in its own repository, with the
    one-sided calculus, the two-sided ILL and affine calculi, the cut as an
    optional rule, proof terms, checkers, soundness theorems and a
    first-order development
    (<https://github.com/linlog-prover/rocq-linlog>, read 2026-11-08; status).
    Any new exponential rule set must be mirrored there to be certified
    (inference).
  - Cyclic MLL and the Lambek calculus exist, as an `Order` field of `Mode`
    (`core/src/fragment.rs`), planar nets, interactive proving, drawing
    (`docs/src/cyclic.md`, `docs/src/lambek.md`). They are multiplicative
    and without exponentials; an ordered mode refuses the cut (status).
    So the ordered side of logics-3 has its base, its modal side has none.
  - `Term` is 12 bytes, a tag and two arena indices
    (`const` assertion in `core/src/sequents/term.rs`); `Bang(TermId)` and
    `Quest(TermId)` use one index, so a label fits in the second (inference).
    `Mode` is a struct of booleans and the `Order` enum
    (`core/src/fragment.rs`), not matched by users outside the library.
  - Affine mode exists in every fragment; the unrestricted zone of the
    focused engine is a set of occurrence bits (`.claude/rules/core-inverse.md`
    on `theta`). One `theta`, not a zone per label.
- What the old note already said is built is unchanged: nothing of it is
  proposed again below.

## 2. The field: what the re-read of the old sources and the search changed

### 2.1 The old note's sources re-checked

- **Chaudhuri, undecidability of multiplicative subexponential logic**
  (one unrestricted and two incomparable linear subexponentials, two
  register machines): the arXiv page says exactly that, submitted
  2015-02-17 [1]. The old note's claim stands.
- **BI undecidable** (Galatos, Jipsen, Knudstorp, Ramanayake): arXiv
  2603.01595, submitted 2026-03-02, by Wang tilings, extending to GBI and
  BBI and pointing at gaps in the decidability claims [2]. The page shows
  no venue; the old note's "LICS 2026" is therefore not confirmed by the
  source read today (the arXiv listing is math.LO and names no
  conference). Rejection of BI stands, now with a primary source.
- **Granule** (<https://github.com/granule-project/granule>, read
  2026-11-08): BSD-3-Clause, "a statically-typed linear functional
  language with graded modal types", 3,922 commits, 41 open issues, 746
  stars, an empty Releases section, binaries "for macOS only"; the page
  shows no dates of commits [3]. The project's site lists as its newest
  papers one at CSL 2025 and, in its news, "Same Coeffect, Different Base:
  Connecting Two Dominant Approaches to Graded Types" (Liepelt, Marshall,
  Orchard) at ICFP 2026, dated August 2026 [4][5]. The research page lists
  nothing from 2026 [5], so the news item is the only 2026 evidence (the
  paper itself was not read). No release number could be sourced.
  The project is alive; its synthesis work (Hughes and Orchard, ESOP 2024)
  is the one the old note cited, and no later synthesis paper was found.
- **Adjoint logic's functional reading**: Jang, Roshal, Pfenning, Pientka,
  FSCD 2024, as the old note says. New: Boyland and Pfenning, "Proof-
  Theoretic Adjoint Compilation" (xlang 2026 draft, CMU), which
  translates adjoint natural deduction to the adjoint semi-axiomatic
  sequent calculus and reports a compiler from an adjoint functional
  language to C [6]. The TLLA 2025 programme lists the talk under the
  title "Compiling adjoint natural deduction to the semi-axiomatic
  sequent calculus" [7]; the draft read today carries the other title.
  Adjoint logic is described in that draft as "a generalization of
  Benton's linear/non-linear logic supporting arbitrary preorders of modes
  with varying substructural properties" and as integrating Nigam and
  Miller's subexponentials [6]: the old note's reading that logics-8 sits
  on logics-1's signature is confirmed.
- **Fixed points and circular proofs**: the old note's DDS22, NST19 and
  BDKS20 stand as written (not re-read here, they belong to
  cyclic-proofs.md). New in 2026: Curzi and Leigh, "Making progress:
  Reducibility Candidates and Cut Elimination in the Ill-founded Realm",
  two cut-elimination proofs for ill-founded μMALL, arXiv 2602.01299,
  2026-02-01, revised 2026-02-13 [8]. It is theory; no tool.
- **Light logics**: no re-read of GR09 and Terui's results (they are
  textbook, stable). New evidence of live interest in the typed side:
  "LFPL: Revisited and Mechanized" (Glover, Hoffmann), arXiv 2605.12893,
  submitted 2026-05-13, revised 2026-07-09, a mechanized account of an
  affine type system for polynomial time, in the Istari proof assistant
  [9]; and Atkey's invited talk "Implicit Polynomial Time and Linear
  Dependent Types" at TLLA 2025 [7] (the related paper, "Polynomial Time and
  Dependent Types", POPL 2024, arXiv 2307.09145, was only seen as a search
  result, not read). Both are type systems, not proof-search tools
  (inference).

### 2.2 New work, 2025 to 2026

Read today: the TLLA 2025 programme (Birmingham, 2025-07-19 and 20, with
FSCD 2025) [7]; the call for TLLA 2026 (the 10th, Lisbon, 2026-07-18 and
19, with FLoC 2026; deadlines 2026-05-15 and 2026-05-25) [10]; no 2026
programme could be found (the site `tlla.linear-logic.org/2026` was not
fetched; see section 7). The web search budget ran out before the
LICS, CSL and FSCD 2026 programmes could be searched; coverage of
those venues is therefore partial (section 7).

- **Intuitionistic BV (IBV)**, Acclavio and Straßburger, arXiv 2505.13284,
  submitted 2025-05-19, revised 2026-04-24, TABLEAUX 2025 [11][7]: a
  deep-inference system whose MLL fragment is IMLL, with cut elimination,
  and its non-associative variant INML with a cut-free sequent calculus
  [11]. Barrett gave a tutorial "Linearity and Deep Inference" at the same
  workshop [7]. This changes logics-4: the intuitionistic reading is the
  one linlog's two-sided surface already has (inference).
- **Undecidability of linear logics without weakening**, Suzuki and Sano,
  arXiv 2509.00644, submitted 2025-08-31, TLLA 2025 [12][7]: classical
  linear logic with the exponential weakening rule removed, with units
  (CLLR) and without units (CLLRR), is undecidable; CLLR had been studied
  by Melliès and Tabareau, its undecidability was open; the authors note it
  as a special case of subexponential linear logic [12]. This is the
  "contraction without weakening" exponential, the fourth corner for the
  exponentials alone. It says that a relevant exponential mode (the
  exponential half of logics-6) is undecidable like full LL, so linlog's
  copy bound and three-valued answer apply unchanged (inference).
- **ILL with subexponentials as a type theory**, Rogozin, arXiv 2507.12360,
  submitted 2025-07-16, v9 of 2026-10-09: the calculus SILL(λ)_Σ for
  multiplicative intuitionistic linear logic with subexponentials,
  models (Σ-assemblages), a 2-categorical characterisation and
  realisability [13]. Theory; confirms a living research line on SELL
  as a signature (inference).
- **A mixed linear and graded logic**, Vollmer, Marshall, Eades, Orchard,
  CSL 2025: graded modalities split into a graded logic and a graded
  linear logic, with a sequent calculus, proof theory, a categorical
  model and a natural deduction system; the abstract mentions neither
  proof search nor an implementation [14]. It is the nearest thing to a
  sequent calculus for linlog's logics-2 (inference).
- **Proof nets for first-order MALL** (Acclavio and Manara, TLLA 2025) and
  **Proof nets for PiL**, a first-order MALL extension to encode
  π-calculus processes (Acclavio and Manara, arXiv 2605.14476, 2026-05-14;
  correctness criterion, sequentialization, no implementation) [7][15];
  **Yeo's theorem for locally coloured graphs** (Di Guardia, Laurent,
  Tortora de Falco, Vaux Auclair, arXiv 2603.03262, 2026-03-03), a
  generalisation of the tool behind Danos-Regnier sequentialization,
  covering Mix and unit-free MALL [16]; Di Guardia and Laurent, "A
  Formalization of Multiplicative Proof-Nets in Rocq" (TLLA 2025) [7].
  These concern the nets area, not logics; they bear on `logics-9` below.
- **Adjoint logic in practice**: Boyland and Pfenning above [6][7].
- **Circular proofs in linear logic**: Saurin and Osorio-Valencia, "On a
  Proof-Relevant Interpolation Theorem for Circular Proofs in Linear
  Logic", and Fiorillo, Osorio Valencia and Saurin, "On Correctness,
  Sequentialization and Interpolation" (TLLA 2025) [7]; Grellois and
  Clairambault on fixed points in the infinitary relational semantics [7];
  Miller's invited talk "A system of inference based on proof search" [7].
  Cross-reference: `cyclic-proofs.md`.
- **Relevant logic provers**: a search surfaced a Coq-mechanized
  constructive decision procedure for the implication fragments LR1 and
  LR2 of relevant logic, with a free licence on GitHub (seen only as a
  search snippet; its page, repository and date could not be read, and
  its link is unsourced). No maintained LR prover was found; the old
  note's "no maintained decider is known" is unchanged but now rests on
  one search only.
- **What was not found**: no 2025 to 2026 prover for SELL, BV or MAV
  search, no new tool for light logics, no new μMALL prover (the 2026
  μMALL paper is theory), and no new benchmark. The TLLA 2025 programme
  has no title naming relevant logic, light logics or subexponential
  provers [7] (inference: the field is theory-heavy; a tool here would
  meet no competing tool).

## 3. Candidates

For each: status today, what changed, cost re-estimate, then (where it
changed) the audience reading. Sessions are as the brief defines them.

### logics-1: Subexponential signatures (SELL)

- *Status*: untouched. *Changed*: the signature view is now well supported
  as the common frame of adjoint logic [6], of ILL type theory [13] and of
  the weakening-free variants [12]. The undecidability results [1][12]
  keep the copy bound and the three-valued answer as the only honest
  discipline (inference).
- *What it brings*: the old note's value stands. New: the CLLR result [12]
  gives a concrete instance that is easy to explain (an exponential with
  contraction only), and teaches why weakening matters.
- *What a suite would offer*: decide within the copy bound, check, prove
  step by step (promotion checks the preorder), export, certify through
  the library's checker mirror; search for the commutative labelled
  fragment only.
- *Cost, re-estimated*: the old estimate of one to two sessions after step
  28 left out what 38 to 45 added. Engine, checker, parser and printers as
  before (one to two sessions); plus the cut-elimination case per label
  on terms and on nets (a box kind per label), plus the library mirror of
  the rules and of the soundness theorems in both calculi and at first
  order. Full stack: three to four sessions, of which the Rocq part is one
  to two and is done in the other repository (inference). The trusted
  base grows by the promotion rule per label in the Rust checker, and in
  the library by its mirror.
- *Risk*: undecidable in general [1][12]; a signature with a preorder
  invites user error, so the parser must validate it.
- *Prerequisites*: the label in `Bang`/`Quest`'s spare index; a zone per
  label in the focused engine (today one set `theta`, core-inverse.md);
  `Mode` carrying a signature (a value, not a boolean). The two-sided
  engine and the net engine must each split by label.
- *Sources*: [1][12][13][6]; old note's NOP17, KKNS19.

### logics-2: Graded and bounded exponentials

- *Status*: untouched. *Changed*: Granule is alive and the CSL 2025 sequent
  calculus of a mixed linear and graded logic [14] exists, giving a
  published sequent calculus to implement; Granule's repository still has
  no releases entry [3]. No newer synthesis work than ESOP 2024 was found.
- *Cost*: unchanged in kind. The natural-number and interval semirings are
  counting, which the copy bound already does per branch; the old
  estimate (two sessions) holds for the engine, plus the library mirror as
  for logics-1. A trait for general semirings stays advised (inference).
- *Risk*: the Granule community's real programs use data types and
  recursion, beyond propositional sequents (`plan/later.md` row 29 already
  dropped Granule's benchmarks for this reason).
- *Prerequisites*: as logics-1; a grade is a label with arithmetic.
- *Sources*: [3][4][5][14]; old note's GSS92, HO20, HO24, EO20.

### logics-3: Ordered logic and Lambek with modalities

- *Status*: partly built. The Lambek calculus and cyclic MLL are built
  (`docs/src/lambek.md`, `docs/src/cyclic.md`); the ordered side has planar
  nets and two-sided derivations. Not built: any modality, an ordered
  zone beside an unrestricted one, cut in an ordered mode (refused,
  status).
- *Changed*: the prerequisite "depends on 36" is met. Left: the exponentials
  on the ordered side, which the net engine does not have in an ordered
  mode (the planar nets are multiplicative, no boxes) (inference from
  status). IBV and INML [11] are an ordered intuitionistic neighbour with a
  cut-free sequent calculus.
- *Cost*: the old note's estimate (depends on logics-1) stands; the
  decidable fragments of the old note are the default. Two to three
  sessions after logics-1 (inference). The ordered cut is a separate
  small step (one session) that also serves the Lambek teaching case.
- *Audience*: linguistics and teaching (the Lambek page); not shown to have
  a user among linlog's today.
- *Sources*: old note's KKNS19, KKNS20, MWS21, Pol01; [11].

### logics-4: Deep inference (BV, MAV, pomset nets)

- *Status*: untouched. *Changed*: IBV is new [11], and BV's tutorial
  presence at TLLA 2025 [7] shows an active line, but no search tool or
  prover for BV or MAV was found in 2025 to 2026 [section 2.2].
- *Cost*: the old estimate (two to three sessions, research in part) is
  unchanged and still the highest in this note: a derivation that is not a
  `Node` tree. The cheaper entry (a checker and a drawing for pomset nets)
  is unchanged.
- *Audience*: research only; no teaching tool in linlog's reach reads
  deep inference (inference).
- *Sources*: [11][7]; old note's Gug07, Hor15, NS23.

### logics-5: Fixed points (μMALL)

- *Status*: untouched. *Changed*: a 2026 paper on ill-founded cut
  elimination [8] and TLLA 2025 talks [7]; no tool. First-order terms exist
  but binders over formulas do not. Cross-reference: `cyclic-proofs.md`
  (cyclic-proofs-3 restates this candidate; its author's estimates
  prevail). Not re-estimated here.

### logics-6: Relevant logic LR as a mode

- *Status*: untouched; affine mode is the dual corner and exists.
- *Changed*: the exponential half is now known undecidable [12], so the
  mode splits in two: (a) the exponential-only variant (CLLR style: `!`/`?`
  with contraction but no weakening), a cheap mode on the existing copy
  bound, in the spirit of logics-1; (b) LR proper (Kripke-Meyer, the
  old note's source), whose procedure has an Ackermann-primitive-recursive
  or worse bound and which no maintained tool implements. A Coq-mechanized
  decision procedure for implication fragments exists (unsourced beyond a
  search snippet).
- *Cost*: (a) one session on top of a `Mode` flag, with the library mirror
  one more; (b) as the old note: two to three sessions including the
  dominance memo, and a loop check that `plan/later.md` wants anyway.
- *Sources*: [12]; old note's Urq16, Urq99.

### logics-7: Light logics

- *Status*: untouched. *Changed*: boxes exist at every depth in nets
  (status), so the "judgement on proofs" needs only the depth conditions
  on top. The cut elimination that the light logics bound now exists to
  be bounded (the bound is a number to display) (inference).
- *Cost*: small, as the old note, and now the cheapest of the
  modal candidates for teaching: one session for the stratification
  judgement on a proof net, plus the display of the cut-elimination step
  count. The soft rule set as a mode is not re-estimated.
- *Evidence of use*: LFPL's 2026 mechanization [9] and Atkey's talk [7]
  show an active implicit-complexity community, on the type side.
- *Sources*: [9][7]; old note's GR09, Ter, DM04.

### logics-8: Adjoint logic's functional reading

- *Status*: untouched; two-sided surface and ILL are built.
- *Changed*: an adjoint compiler exists [6], so a reader who wants adjoint
  logic has a partner tool for programs; a linlog playground for the
  logic itself (modes, shifts, derivations) remains uncovered (inference).
  Not re-costed: after logics-1, one to two sessions for shifts and
  modes on the two-sided engine, a term printer one more.
- *Sources*: [6][7]; old note's PCPR18, JFSCD24.

### logics-9 (new): First-order and net-side follow-ups from 2025 to 2026

- *What*: proof nets for first-order MALL and a generalised sequentialization
  proof [7][15][16] bear on linlog's nets. linlog decides MALL1 by search
  (status) but draws no first-order nets; its nets are propositional.
- *Why here*: this is not a neighbouring logic but a way the existing logic
  could grow with a source; the nets area's own note should judge it.
  (inference: not costed.) The theorem of [16] covers what linlog's
  Yeo-based correctness test already uses (status), so a check against it
  would only confirm the implementation.
- *Sources*: [7][15][16].

## 4. Considered and rejected

The old note's list stands (BI, pomset and BV as search, NEL, non-wellfounded
μMALL, differential LL, full R, SMT in the library). New:

- **BI as a decided logic**: now undecidable with a primary source [2],
  the rejection is stronger.
- **PiL nets and choreography encodings** [15]: theory with no implementation
  and no user among linlog's audiences (inference).
- **A prover for IBV or INML now** [11]: a sequent calculus for INML is cut-
  free, but the logic is two papers old and no user is known (inference).
- **Linear dependent types and light type assignment** [7][9]: type
  systems, outside a sequent prover (inference).

## 5. Re-ranking for the three audiences (inference)

Research, teaching, AI models through `skills/linlog/`. "Mode" means a mode
of the linear suite; "suite" a new suite.

| rank | candidate | kind | research | teaching | AI models | why |
|---|---|---|---|---|---|---|
| 1 | logics-1 with logics-6a and logics-2 (counting grades), as one "exponentials by signature" step | modes | high | medium | medium | one data change serves SELL, CLLR, grades, and the weakening-free variants; the only candidate whose 2025 literature [12][13][14] is live and tool-free |
| 2 | logics-7 stratification judgement | mode (a checker) | medium | high | low | cheapest, uses boxes and cut elimination that now exist |
| 3 | logics-3 ordered cut, then modal Lambek | modes | medium | medium | low | Lambek exists; the ordered cut closes a refusal in status |
| 4 | logics-8 adjoint | mode on two-sided surface | medium | medium | low | needs logics-1; a live research line [6][7] |
| 5 | logics-6b LR | mode | low | low | low | no tool exists, no audience seen |
| 6 | logics-5 (μMALL) | new suite | high | medium | low | see `cyclic-proofs.md`; needs a certificate kind beside `Node` |
| 7 | logics-4 (BV, MAV, pomset) | new suite | high | low | none | a different proof object, no live tool |

For the collection-of-suites direction (inference): subexponentials,
graded, relevant exponentials, light and the ordered modalities are
**modes** of the linear suite (they change which structural and exponential
rules a `Mode` permits and reuse the forest, checker, interactive and
exports). μMALL with cyclic proofs, BV and pomset logic are **new suites**:
their proof objects are graphs or rewrite sequences, not `Node` trees
(the old note's section 4). Adjoint logic is a mode with a larger signature,
but its audience is programming languages, which is the likeliest candidate
for a suite of its own if a web playground is wanted.

Against AI models: none of these helps an agent more than a prover that
already answers; the exponents-by-signature mode adds only one flag family
to the guide (inference).

## 6. What the structure must allow

Unchanged from the old note's section 4, with the checks of this re-read
(from `.claude/rules/core.md` and the module rules, and the code):

- The label in `Bang` and `Quest` fits `Term`'s spare index (12 bytes
  asserted); `Mode` is a struct, so a signature is a new field, not an
  exhaustive match.
- A per-label zone replaces the single `theta`; this touches the focused
  engine, the two-sided engine, the inverse method's subsumption (it
  compares `theta_T ⊆ theta_S`, `core-inverse.md`), the net engine's
  boxes and the memo keys.
- Cut and cut elimination must be extended per label; the Rocq library
  must be extended in its own repository first (CLAUDE.md: library
  changed there, then the lock moves).
- Reusable by every candidate: forest, checker framework, interactive
  session, exports. Linear-specific: the counts (`core-search`), the
  Horn engine, the nets' correctness criteria.

## 7. Sources

1. K. Chaudhuri, Undecidability of multiplicative subexponential logic, arXiv:1502.04769. <https://arxiv.org/abs/1502.04769>, read 2026-11-08.
2. N. Galatos, P. Jipsen, S. B. Knudstorp, R. Ramanayake, The logic of bunched implications is undecidable, arXiv:2603.01595. <https://arxiv.org/abs/2603.01595>, read 2026-11-08.
3. Granule repository. <https://github.com/granule-project/granule>, read 2026-11-08.
4. The Granule Project site. <https://granule-project.github.io>, read 2026-11-08.
5. Granule research page. <https://granule-project.github.io/research.html>, read 2026-11-08.
6. J. Boyland, F. Pfenning, Proof-Theoretic Adjoint Compilation (draft, xlang 2026). <https://www.cs.cmu.edu/~fp/papers/xlang26.pdf>, read 2026-11-08 (abstract and introduction only).
7. TLLA 2025 programme, Birmingham, 2025-07-19/20. <https://lipn.univ-paris13.fr/TLLA/2025>, read 2026-11-08.
8. G. Curzi, G. E. Leigh, Making progress: Reducibility Candidates and Cut Elimination in the Ill-founded Realm, arXiv:2602.01299. <https://arxiv.org/abs/2602.01299>, read 2026-11-08.
9. N. Glover, J. Hoffmann, LFPL: Revisited and Mechanized, arXiv:2605.12893. <https://arxiv.org/abs/2605.12893>, read 2026-11-08.
10. TLLA 2026 call, 2nd cfp. <https://lists.seas.upenn.edu/pipermail/types-announce/2026/012480.html>, read 2026-11-08.
11. M. Acclavio, L. Straßburger, Intuitionistic BV (extended version), arXiv:2505.13284. <https://arxiv.org/abs/2505.13284>, read 2026-11-08.
12. J. Suzuki, K. Sano, Undecidability of Linear Logics without Weakening, arXiv:2509.00644. <https://arxiv.org/abs/2509.00644>, read 2026-11-08.
13. D. Rogozin, Intuitionistic Linear Logic with Subexponentials: Type Theory, Categorical Models and Realisability, arXiv:2507.12360. <https://arxiv.org/abs/2507.12360>, read 2026-11-08.
14. V. Vollmer, D. Marshall, H. Eades III, D. Orchard, A Mixed Linear and Graded Logic: Proofs, Terms, and Models, CSL 2025. <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2025.32>, read 2026-11-08.
15. M. Acclavio, G. Manara, Proof Nets for PiL, arXiv:2605.14476. <https://arxiv.org/abs/2605.14476>, read 2026-11-08.
16. R. Di Guardia, O. Laurent, L. Tortora de Falco, L. Vaux Auclair, Yeo's Theorem for Locally Colored Graphs, arXiv:2603.03262. <https://arxiv.org/abs/2603.03262>, read 2026-11-08.
17. rocq-linlog repository. <https://github.com/linlog-prover/rocq-linlog>, read 2026-11-08.
18. The old note's other sources (36 tags) were not re-read; see `plan/notes/research/scope/logics.md` section 5.

## 8. Not verified

- The Coq decision procedure for relevant implication fragments: known only
  from a search snippet; its page returned 403 and no repository was read.
- TLLA 2026's programme and the CSL, LICS and FSCD 2026 accepted papers: not
  read (the web search budget ran out); section 2.2 is therefore partial for
  2026. Also not searched: 2025 to 2026 work on SELL provers beyond one
  search, on light logics beyond LFPL, on relevant logic.
- Granule's current version and last commit date: the repository page shows
  neither; the ICFP 2026 paper was seen only as a news item.
- That the arXiv listing of BI's undecidability is later published at LICS
  2026 (the old note's claim): not confirmed.
- The remaining sources of the old note (Bae12, NST19, DDS22, GSS92, Urq16
  and the rest) were not re-read today. Their claims are the old note's.
- Cost figures are estimates by inference from the module rules and the
  code, not from any prior step's report.
- Whether the net engine in an ordered mode can take boxes: judged only
  from status.md's text.
