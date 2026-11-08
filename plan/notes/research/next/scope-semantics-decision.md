# Semantics and decision problems, updated: certified non-derivability and the Horn engine's promises

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research
sub-agent; sources read that day. This note updates
`plan/notes/research/scope/semantics-decision.md` (written 2026-10-09 to 17,
before steps 37 to 45; below "the scope note"), keeps its candidates'
names (semantics-decision-1 to 9) and continues the numbering from 10.
Repository files are cited by path, outside sources by the numbered list
of section 6 (S). "(inference)" marks a conclusion of this note's own;
"(unsourced)" a claim for which I found no page.

## 1. What linlog has today

From `docs/src/status.md`, `.claude/rules/core-inverse.md`,
`core/src/search/refutation.rs`, `plan/later.md`:

- **Decision.** A Horn engine for programs whose clauses sit under `!`
  (Petri nets): linear mode searches reachable markings forward and
  backward; affine mode runs the backward coverability algorithm, which
  always ends. It answers 3 072 of the LLTP's 3 137 nets and 162 of
  qcover's 176 problems (`docs/src/results/provers-2026-10-15.md`;
  later baselines are in `docs/src/results/`).
- **The inverse method (step 37)** is built, in every commutative mode,
  affine and intuitionistic included: forward saturation with whole
  focusing phases as rules and subsumption over tries; it "decides
  without exponentials and with them proves without a copy bound and
  refutes where its saturation ends" (`status.md`;
  `plan/reports/37-inverse.md`).
- **Refutation kinds** in `Refutation`: `Exhausted`, `Unbalanced`,
  `OneSided`, `Equation`, `StateEquation`, `Classical`, `Saturated`,
  and per-predicate `UnbalancedSymbol`, `OneSidedSymbol` for first order
  (`core/src/search/refutation.rs`).
- **Checks.** `Disproof::check` verifies `Unbalanced`, `OneSided`,
  `Equation`, `Classical`, `Saturated` and the symbol kinds without the
  search. The saturated-set check shares no code with the engine and
  accepted all 213 measured refutations (`core-inverse.md`, "The
  certificate"). `Exhausted` and `StateEquation` answer
  `RefutationFault::Uncertified` (`refutation.rs` line 662 on).
- **Rocq.** The library proves derivable implies classically true,
  atom balance and the count equation, so `--format rocq` writes a
  non-derivability lemma for those refutations, and for ordinary
  invalid sequents over `Prop` (`status.md`, "Certified refutations").
  Not certified in Rocq: `Saturated`, `StateEquation`, `OneSided`
  with copies, first-order refutations (`plan/later.md`, "Follow-ups:
  certified refutations"; `plan/reports/42-first-order.md` line 2251:
  the disproof writer answers `Unsupported::Quantifiers`).
- **First order (step 42).** Terms, eigenvariables by levels,
  unification: decided in the classical modes and the two-sided
  intuitionistic ones, with the families `reach-fo` and `chain-fo`
  (`docs/src/results/provers-2026-11-03.md`: `reach-fo` #0 to 10 nodes,
  #1 at 2; no other tool runs them).
- **Planned (manual, `status.md`, first item):** "Certified refutations
  beyond the cheap ones and the saturated sets: countermodels of the other
  logics, failure certificates of the backward search." Section 3 weighs it.
- **Not built:** any countermodel output (Kripke or phase), any ideal or
  Presburger certificate, forward coverability (Karp-Miller), a PNML reader,
  data nets, Horn shapes beyond `!`-Horn, a `Refutation` read back by
  `linlog check` (`plan/later.md`, same follow-ups).

## 2. The field, re-checked

**Complexity ground truth (what a Horn engine can promise).**

- Reachability in VASS is Ackermann-complete (Czerwiński and Orlikowski,
  FOCS 2021 [S1]; Leroux independently [S2]); F_k-hard in dimension 6k
  [S1]. Fixed-dimension and 2025/26 work: 3-VASS reachability is elementary
  (ICALP 2025) [S3]; for d-VASS extended with integer counters, reachability
  lies in F_{d+2}, and the abstract says Tower-hardness is known only from
  dimension 8 (arXiv 2603.05221, March 2026; read as a search snippet, not
  opened) [S4]. Coverability stays EXPSPACE-complete (the scope note's S25,
  Künnemann et al., not re-read today).
- (inference) A Horn engine that is complete for linear mode (reachability)
  can never promise a bound better than Ackermann in the worst case, so
  its promises are three: sound, complete in affine mode (coverability,
  EXPSPACE), and in linear mode a verdict when either a run or an
  invariant is found. Leroux's theorem is what makes the last honest:
  unreachability always has a Presburger inductive invariant as witness
  [S5; S6].
- MELL provability is equivalent to reachability in branching VASS;
  decidability is still open. Straßburger's paper says so and shows that
  a claimed proof (Bimbó 2018) has a mistake with no obvious fix [S7].
  A search for 2025/26 results found none claiming to settle it
  (search of 2026-11-08, no source to cite beyond [S7]; this absence is
  not a proof). Lazić and Schmitz's lower bound (Tower-hard for affine MELL
  and extensions) is from the scope note's S6, not re-read.

**Finite models.** Lafont's paper states finite phase semantics is complete
for MALL and for affine LL (LLW), not for MELL [S8]. Okada and Terui for the
intuitionistic fragments (scope note S12, not re-read). No tool searching
for finite phase countermodels was found (the scope note's finding;
re-searched today with no new hit). Yalla 2.0.7 (released 2025/03/26, the
page says so) lists phase semantics as ongoing work [S9];
the same page lists Rocq 9.0 with 9.1 and 9.2 compatible. For
intuitionistic propositional logic, finite countermodel work is current:
Fiorentini's ASP-based minimal countermodels (IJCAI 2019) [S10], and
Lyon's nested-sequent proof search with finite countermodel extraction
for intuitionistic tense logics (arXiv 2603.29424, March 2026, in review)
[S11]. A Lean-formalised terminating IPL search with refutation
construction is listed in a 2026 seminar abstract I could not read (PDF
not extractable); (unsourced) beyond that.

**Certificates for coverability and reachability, in tools.**

- SMPT returns an inductive invariant that an independent SMT solver
  checks in three queries (initial, inductive, implies the property);
  Amat and Dal Zilio's PDR learns linear invariants, e.g.
  `p0 + p1 = p2 + 2` [S12; S13].
- Chini and Furbach's inductive half-spaces (linear inequalities that
  disprove reachability) with an inductivity check that is coNP-complete
  (reported by a search result summary; the paper itself not opened,
  link not recorded) (unsourced beyond that).
- Blondin and Esparza's separators in continuous Petri nets, checkable
  in polynomial time or better [S14].
- Formal verification in Coq: Hilaire, Ilcinkas and Leroux certified the
  MinCov algorithm for the minimal coverability set (TACAS 2024) [S15];
  Larchey-Wendling's `coq-karp-miller` decides coverability and builds the
  Karp-Miller tree, correct by construction, MPL-2.0, listed active
  [S16]. No Isabelle or Lean formalisation was found. (inference) A
  Rocq-side coverability decision is therefore within precedent, and
  `rocq-linlog` could take a reflected checker without inventing the
  mathematics.

**Tools (state read today).**

| tool | what | licence | activity | source |
|---|---|---|---|---|
| qcover | backward coverability pruned by continuous reachability, Z3; Python 2.7; `.spec` input; prints only "Safe", no certificate | Apache-2.0 | 57 commits on master, last date not shown | [S17] |
| mist | safety checker for Petri nets, backward, ic4pn, tsi, eec, eec-cegar | GPL-3.0 | 281 commits, last date not shown | [S18] |
| KReach (`dixonary/kosaraju`) | Haskell, Kosaraju's procedure | BSD-3-Clause, copyright 2019 | 25 commits, last date not shown; the scope note found it unchanged since 2020 | [S19] |
| verifypn | TAPAAL's engine, reachability queries on PNML | GPL-3.0 | 3 671 commits, no release listed, last date not shown | [S20] |
| ICover | variant of QCover with state equation and sign analysis | (unsourced) | paper only | [S21] |
| LoLA | (not re-read; licence and state unsourced) | | | |

Contest: MCC 2025 had five tools and 1 855 model instances, Tapaal ahead
on reachability formulas (52 819 points), ITS-Tools and smpt next [S22];
MCC 2026 had six tools (tedd, smpt, Tapaal, petrivet, TY, ITS-Tools) and
1 953 instances, Tapaal first again (53 680) [S23]. The MCC measures
verdicts, not certificates (inference: the pages show points for verdicts
only). Theory background for the scope note's other papers (WSTS, ideal
Karp-Miller, ν-nets, data nets) was not re-read; the claims below that rest
on them cite the scope note's numbers and say so.

## 3. Candidates

The scope note's nine, then the new ones. Status: built, partly, untouched.

### semantics-decision-1. Ideal certificates for non-coverability

*Status: untouched; prerequisites changed (cheaper).* The affine Horn
engine still ends with a finite basis of `Pre*(up goal)` and exposes it to
nobody. Step 37 added a precedent that was not there: `Refutation::Saturated`
is a closed set of sequents, checked by a checker that shares no code with
the engine. (inference) The basis is the Horn row's version of the same
idea, and the payload, the independent checker and the `Uncertified`
vocabulary now exist to copy.
- *Brings.* Research and AI models: a checked "unprovable" for every
  refuted qcover problem, none of whose tools prints a certificate
  (qcover prints "Safe" only [S17]; SMPT's invariant is for its own
  reachability setting [S12]). The Rocq lemma lands the follow-up in
  `plan/later.md` ("The one-sided and state-equation refutations have
  neither a check nor a Rocq lemma").
- *In linlog's terms.* Decide: as today. Check: new `Refutation::Basis`
  checked in `Disproof::check` (`|B|^2 * |T|` vector comparisons).
  Export: Rocq lemma that a firing sequence yields a proof (the converse of
  `core-horn.md`'s induction). Searchable: yes; the search already holds it.
- *Cost.* 2 to 3 sessions (Rust payload and checker 1, Rocq lemma and
  writer 1 to 2). Trusted base: the checker only, until the lemma lands.
- *Risk.* Basis size on the large qcover nets (66 950 places);
  (inference) a certificate larger than the sequent is still a certificate
  but costs memory, which the memory account must charge.
- *Prerequisites.* The Petri translation public with a numbering stated
  (as the scope note asked); `Disproof` readable by `linlog check`
  (`plan/later.md`).
- *Sources.* [S17] [S12]; `core-horn.md`; scope note S36, S37.

### semantics-decision-2. Presburger inductive invariants for non-reachability

*Status: partly (the Farkas state equation, uncertified); the integer gap
untouched.* `Refutation::StateEquation` exists and is found by a simplex but
its check is `Uncertified`; the odd-counter nets are still unrefuted by the
Horn row (`plan/later.md`, "What no refutation reaches yet").
- *Brings.* Research: Leroux's theorem makes invariants the universal
  witness [S5; S6]; SMPT, PDR and the half-space work show invariants
  of linear and modular shape are what tools produce and check [S12; S13].
  Teaching: "the counter stays odd" is the explanation a student wants.
- *In linlog's terms.* Check: two kinds of payload, a Farkas weighting over
  every transition (rational, exact) and a modulo-k invariant `y,k` with
  `y*C_t = 0 mod k` and `y*(M - M0) != 0 mod k`; each checked by integer
  arithmetic in one pass. Rocq: lemma by computation (the refutations note,
  group E). Search: modulo-k by solving over Z/k for small k, or from the
  backward markings; semilinear in general is research.
- *Cost.* Farkas certification: 1 session (an exact checker and the
  Rocq lemma; the weights beyond 2^20 need an exact rational read-off,
  already a follow-up). Modulo-k: 1 to 2 sessions. General Presburger
  invariants: a project of its own.
- *Risk.* A third of the value is in the state equation, whose checking is
  cheap; the rest is a search for which no bound is known (inference).
- *Prerequisites.* The class places' weights and dead-transition argument
  stated as data (`plan/notes/research/refutations.md`, group E).
- *Sources.* [S5] [S6] [S12] [S13] [S14].

### semantics-decision-3. Ideal Karp-Miller: coverability sets and boundedness

*Status: untouched.* The forward search of the Horn engine is not
Karp-Miller; no omega-markings exist. Precedent changed: a Rocq
`coq-karp-miller` and a certified MinCov exist [S15; S16], so a
reflection-based checker for coverability sets is mathematically available.
- *Brings.* Teaching (is this resource bounded?) and models
  (the answer "unbounded" with a witness). Questions beyond provability
  need a question kind in the front door; no other candidate here does.
- *Cost.* 3 sessions (omega type, acceleration, verdict kind, JSON); a
  Rocq side is a project of its own. Trusted base: the omega-marking
  logic, unless the Rocq side follows.
- *Risk.* Karp-Miller trees blow up; the minimal coverability set (MinCov
  [S15]) is the practical form. (inference)
- *Prerequisites.* A question kind beside provability in `Verdict` and
  JSON; the guide `skills/linlog/` updated.
- *Sources.* [S15] [S16]; scope note S38.

### semantics-decision-4. Directed and accelerated reachability

*Status: untouched; the 66 undecided nets (as of the scope note) are the
measure.* Unchanged from the scope note, no new source. Cost 1 to 2
sessions per heuristic, earned by `bench/targets.sh`. Rank low for the
suite's mission: it changes speed, not what is certifiable (inference).

### semantics-decision-5. A Petri-net front door

*Status: untouched.* MCC 2026 ran with six tools on 1 953 instances [S23]
and every entrant is in the verdict business [S22]. A PNML reader plus
reachability and coverability queries would let linlog enter the same
conversation with checkable certificates, but the MCC's reachability
formulas are boolean combinations on markings, richer than a sequent's
goal (inference). Cost 3 sessions; value for research; low for teaching.
verifypn is GPL-3.0 (callable, never linkable) [S20].

### semantics-decision-6. Affine decision by saturation with subsumption

*Status: partly built, in the affine inverse method.* Step 37's engine runs
in affine and intuitionistic affine mode with weakening-subsumption, and
`Refutation::Saturated` is the antichain as certificate, checked apart from
the engine (`core-inverse.md`). What remains of the candidate is not the
engine but the claim: whether the method terminates in every affine
fragment with exponentials (the scope note's inference by Dickson's lemma;
Lazić and Schmitz's Tower-hardness bounds the size, not the termination)
and a measurement on the affine MELL family. The fragment's decidability
rests on Kopylov (scope note S2) and Lafont [S8]. Cost: 1 session to test
and document; the Rocq side is the failure-certificate project of
semantics-decision-12.

### semantics-decision-7. Finite phase countermodels

*Status: untouched.* Lafont [S8] gives the theorem for MALL and LLW and
the negative for MELL. Still no tool and no size bound read. Yalla lists
phase semantics as ongoing work [S9], so the Rocq reference theory is
not there to lean on; `rocq-linlog` would prove soundness itself (a short
induction, the scope note's R3, inference). Research, measure first on the
refuted MALL targets: a prototype enumerating small commutative monoids
under a size bound and reporting how often one is found. Rank: research
interest high, practical value uncertain (inference).

### semantics-decision-8. Petri nets with data: first-order Horn with fresh names

*Status: prerequisite met (step 42), candidate untouched.* Step 42 gave
terms, unification and eigenvariables by levels. (inference) That makes
the Horn clause with variables expressible but not the Horn engine's
markings: the data ordering and the WSTS algorithm are separate work;
what step 42 saves is the reader, the term arena and the guards, not the
decision procedure. Cost stays high (4 to 6 sessions) and the benefit
narrow (MSR-style protocol models, scope note S45, S46). The
`reach-fo` family (decided to 10 nodes, `provers-2026-11-03.md`) is the
closest built relative and shows the focused engine, not the Horn row,
decides first-order reachability today. Rank low.

### semantics-decision-9. Horn shapes beyond !-Horn

*Status: untouched.* No source changed. Cost medium; the measured gain is
unknown. Rank low until a count of the LLTP rows that miss the Horn row
for shape reasons exists (inference).

### semantics-decision-10. Rocq certificates for saturated refutations

*What.* `Refutation::Saturated` has a Rust check and no Rocq lemma. A
Rocq checker over closed sets, and the theorem that a closed set under the
focused rules means no proof, would take the largest piece of the
inverse method out of the trusted base.
- *Brings.* Research and teaching: the saturated set is a readable "why
  not" (every rule applied, nothing new), better than a truth table.
- *In linlog's terms.* Check (done), export (new), certify (new). It
  needs completeness of the focused calculus for the fragment: mechanised
  for other presentations (the refutations note's S23, S25: `coq-ll` and
  Felty et al.), not for linlog's own system (inference).
- *Cost.* 5 to 8 sessions: it is the "largest proof in this list" of
  `plan/later.md`. Trusted base afterwards: Rocq's kernel plus the
  statement.
- *Risk.* The focusing-completeness proof, and certificate size (the
  check costs 17 to 90 % of the search, `core-inverse.md`).
- *Prerequisites.* Decide the library's focused calculus first; MALL and
  affine first, as `later.md` says.
- *Sources.* `core-inverse.md`; `plan/later.md`; refutations note.

### semantics-decision-11. A shared "countermodel" output kind across suites

*What.* A `Refutation::Model` payload family: a finite Kripke model for
ordinary intuitionistic and minimal logic (the ordinary layer), a finite
phase model for MALL and affine later, with one serde shape, one
`Disproof::check` entry and one Rocq statement shape (a forcing
checker over a finite table). For intuitionistic propositional logic
finite countermodels are the mature case: intuitionistic logic has the
finite model property and tools extract small models [S10; S11].
- *Brings.* Teaching, first: a student asking why `((a -> b) -> a) -> a`
  is not provable wants a model, and no step of the plan shows one. The AI
  models in `skills/linlog/` get a uniform "why not". Research: a
  second semantic witness beside the proof-theoretic one.
- *Terms.* Search: a terminating calculus (G4ip, Dyckhoff) with
  extraction, as `plan/later.md` names; check: a forcing evaluation,
  `O(worlds^2 * formula)` (refutations note, group C); export: Rocq over
  `Prop` by instantiation, as the classical case does today; draw: the
  model as SVG (a small Hasse diagram).
- *Cost.* Kripke for LJ through G4ip: 3 to 4 sessions (calculus, model
  extraction, checker, Rocq statement and soundness, drawing). Phase
  models: research (candidate 7). Trusted base: the forcing checker (small).
- *Risk.* G4ip is for propositional logic: the first-order ordinary
  layer (`plan/46` notes first order) has no finite model property
  (undecidable), so the kind must refuse quantifiers (inference). ILL
  has no Kripke search anywhere found (scope note, re-searched today:
  no hit).
- *Prerequisites.* The refutation payload as versioned data and the
  `Disproof` JSON readable by `check`; one generic table checker
  shared with candidate 7 (inference).
- *Sources.* [S10] [S11] [S8]; refutations note, group B and C.

### semantics-decision-12. Failure certificates of the backward search

*What.* The second half of the manual's planned item: the focused
engine records its exhaustive failure as a DAG of its choices, memo
sharing kept, checked by a verified checker (the DRAT/LRAT pattern in
`plan/later.md`).
- *Brings.* Research: certificates for the `Exhausted` verdicts that
  remain after the counts, classical, saturated and basis kinds have taken
  theirs. How many remain is not measured (open).
- *Cost.* 6 to 10 sessions for MALL and affine; a project of its
  own. It shares its hardest part, focusing completeness in Rocq, with
  candidate 10 (inference): do them as one project or not at all.
- *Risk.* Certificate size is the search's work; with exponentials only
  where a copy bound is proved sufficient.
- *Prerequisites.* The Rust checker first, no Rocq (refutations note,
  group D recommendation); candidates 1 and 10.
- *Sources.* `plan/later.md`; refutations note.

## 4. Considered and rejected

- **Rocq-side reflection of the whole decision procedure** for the Horn row
  (a verified Karp-Miller in the library): precedent exists [S15; S16],
  but the kernel would run the search and the library would take a
  development that tracks Larchey-Wendling's; the certificate route (1, 2)
  gives the same trust for far less (inference).
- **A MELL decision procedure** and anything resting on MELL
  decidability: open [S7].
- **Contractive logic, bunched implications, fixed-dimension
  specialisations, external net tools, ordered-data nets**: as the scope
  note, no new source reverses them. Fixed dimension is now less
  attractive still: the nets of practice are not of fixed small dimension
  (the scope note's reasoning; [S3, S4] stay about d-VASS).
- **Kripke countermodels for ILL**: unchanged, no completeness result
  for a searchable form found (scope note; no new hit today).
- **Presenting an MCC entry as a goal**: the MCC measures verdicts [S22; S23];
  a certified linlog would answer fewer queries in the same time
  (inference). Keep the PNML reader (5) as an interface, not a target.

## 5. What the structure must allow

- Reusable (judged from `core-inverse.md` and the refutation module):
  the payload shape of `Refutation` with an independent `Disproof::check`,
  the memory account, the stop with `Phase::Check`, Rocq statements over
  a checked computation. A shared countermodel kind needs one more
  thing: `Disproof` JSON that `linlog check` reads (still a follow-up in
  `plan/later.md`).
- Linear-specific: the basis certificate (Horn programs), the Farkas and
  modulo-k weightings (net structure), the phase semantics. Kripke
  countermodels belong to the ordinary layer and to any future
  intuitionistic suite (modal, more-logics): there the kind is the
  suite's first export, not a linear one.
- Certified non-derivability as a kernel feature: each suite supplies
  `refute -> payload` and `check(payload, sequent)`; the library supplies
  the serde, the JSON, the `check` command and the Rocq statement shape.
  (inference) A suite that cannot give a payload says `Uncertified`,
  as today.

## 5b. Re-rank (inference)

Ranks for research, teaching, AI models, one to five (five best), and the
order I would take them in. Everything here is judgement.

| candidate | research | teaching | AI models | order |
|---|---|---|---|---|
| 1 basis certificate (with Farkas check from 2) | 4 | 2 | 4 | first |
| 11 countermodel kind, Kripke for ordinary LJ | 3 | 5 | 5 | second |
| 2 modulo-k and Presburger | 4 | 3 | 3 | third |
| 6 affine termination measure | 3 | 1 | 2 | small, with 1 |
| 10 and 12 Rocq for saturated and failure certificates | 5 | 2 | 2 | a project of its own, after 1 and 11 |
| 3 Karp-Miller question kind | 3 | 4 | 3 | later |
| 7 phase countermodels | 4 | 2 | 1 | prototype only |
| 5 PNML front door | 3 | 1 | 2 | on demand |
| 4, 8, 9 | 2 | 1 | 1 | not now |

On the manual's first planned item: split it. (inference) The cheap
and useful half is the certificates the engines already hold (1, the
state equation's check and Rocq lemma from 2) and Kripke countermodels
for the ordinary layer (11), 5 to 8 sessions together; the
failure-certificate half (10, 12) is a project of its own resting on a
focusing-completeness proof that nobody has done for linlog's calculus.
The status page's wording ("countermodels of the other logics, failure
certificates of the backward search") is two items.

## 6. Sources

Read 2026-11-08.

- S1. Czerwiński, Orlikowski, "Reachability in Vector Addition Systems is Ackermann-complete", FOCS 2021. https://arxiv.org/abs/2104.13866
- S2. Leroux, "The reachability problem for Petri nets is not primitive recursive", FOCS 2021 (named in the scope note, https://arxiv.org/abs/2104.12695; not re-fetched today).
- S3. Czerwiński, Jecker, Lasota, Orlikowski, "Reachability in 3-VASS is elementary", ICALP 2025 (named in the scope note, https://arxiv.org/abs/2502.13916; not re-fetched today).
- S4. arXiv 2603.05221, March 2026, VASS with integer counters in F_{d+2}; seen as a search result only. https://arxiv.org/html/2603.05221v1
- S5. Leroux, "The general vector addition system reachability problem by Presburger inductive invariants", LMCS 6(3), 2010. https://arxiv.org/pdf/1009.1076 (listed by search, summary read).
- S6. Same, as summarised by the search result of 2026-11-08.
- S7. Straßburger, "On the decision problem for MELL", TCS 2019. https://www.lix.polytechnique.fr/Labo/Lutz.Strassburger/papers/OnDeciMELL.pdf (search summary).
- S8. Lafont, "The finite model property for various fragments of linear logic", JSL 62(4), 1997. https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/abs/finite-model-property-for-various-fragments-of-linear-logic/F75D11ED6907E37EA0424226108DDA66
- S9. Laurent, YALLA project page, release 2.0.7 of 2025/03/26. https://perso.ens-lyon.fr/olivier.laurent/yalla/
- S10. Fiorentini, countermodels for intuitionistic propositional logic by ASP, IJCAI 2019. https://ijcai.org/proceedings/2019/232.md (search summary).
- S11. Lyon, "Loop-Checking and Counter-Model Extraction for Intuitionistic Tense Logics via Nested Sequents", arXiv 2603.29424, 2026-03-31. https://arxiv.org/abs/2603.29424
- S12. Amat, Dal Zilio, "SMPT", FM 2023. https://arxiv.org/pdf/2302.14741
- S13. Amat, Dal Zilio, "Property Directed Reachability for Generalized Petri Nets". https://arxiv.org/pdf/2208.03043
- S14. Blondin, Esparza, "Separators in Continuous Petri Nets", LMCS. https://lmcs.episciences.org/13141
- S15. Hilaire, Ilcinkas, Leroux, MinCov certified in Coq, TACAS 2024. https://link.springer.com/chapter/10.1007/978-3-031-57246-3_21
- S16. `coq-karp-miller` (Larchey-Wendling), MPL-2.0. https://rocq-prover.org/p/coq-karp-miller
- S17. qcover. https://github.com/blondimi/qcover (the page links `michaelblondin/qcover`)
- S18. mist. https://github.com/pierreganty/mist
- S19. KReach's repository. https://github.com/dixonary/kosaraju (named by Dixon and Lazić's slides; paper at https://link.springer.com/chapter/10.1007/978-3-030-45190-5_22)
- S20. verifypn. https://github.com/TAPAAL/verifypn
- S21. Geffroy, Leroux, Sutre, ICover. https://arxiv.org/abs/1607.05956 (search summary)
- S22. MCC 2025 results. https://mcc.lip6.fr/2025/results.php
- S23. MCC 2026 results. https://mcc.lip6.fr/2026/results.php

## 7. Not verified

- Last-activity dates of every repository (the GitHub pages did not show
  them to the fetcher), the licences of ICover and LoLA, LoLA's state.
- Whether any 2025/26 paper settles MELL decidability: only absence in
  searches; no 2026 survey was found.
- The claims taken over from the scope note without re-reading: WSTS and
  ideal Karp-Miller (S36 to S38 of it), ν-nets and data nets (S41 to S44),
  Kopylov, Okada and Terui, Lazić and Schmitz, Kanovich's Horn fragments,
  Künnemann et al. on coverability.
- Chini and Furbach's inductive half-spaces: seen in a search summary only.
- The 2026 seminar abstract on a Lean-formalised IPL refutation search:
  PDF unreadable; not used.
- arXiv 2511.19212 (parameterised coverability) and Sinclair-Banks's 2026
  talk on coverability complexity: seen as search results only; not cited
  for any claim.
- The session estimates are mine and unmeasured. The fraction of
  `Exhausted` verdicts left after all cheap kinds is not measured; a count
  over the baselines would decide candidates 10 and 12. I would have run
  `linlog-bench summary` on the 2026-11-02 baseline CSVs for it; I ran
  nothing.
