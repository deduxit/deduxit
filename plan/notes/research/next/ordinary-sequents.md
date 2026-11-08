# Ordinary sequents and substructural variants: what to add after 0.2.0

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

Claims about other tools and papers carry a bracketed number into section 6. "(inference)" marks this note's own conclusion; "(unsourced)" a claim that no page read in this session confirmed. Where a repository page showed no date of its last commit, the note says so rather than guess.

## 1. What linlog has today

All of this is built and nothing below re-proposes it.

- **Ordinary propositional logic by embeddings.** `--logic classical|intuitionistic|minimal` reads a formula or sequent in an ordinary syntax (`core/src/ordinary/`, `.claude/rules/core-ordinary.md`, manual chapter `docs/src/ordinary-logic.md`). Classical logic goes into affine MALL without exponentials; intuitionistic logic into ILL by Girard's call-by-name translation (default), call-by-value, or Liang and Miller's 0/1 translation; minimal logic is the same with false an atom. The linear proof is read back as a derivation of LK or LJ (G1c/G1i/G1m of Troelstra and Schwichtenberg with additive variants), which a checker of its own validates before it is shown (`Derivation::check`). Every output that draws a derivation draws LK and LJ (text, LaTeX, Typst, SVG, PNG, PDF), and `seq print --logic` shows the image.
- **Reader and benchmark.** A TPTP `fof` reader; the ILTP library's 274 propositional problems fetched by `nix build .#iltp`; the batch mode takes a directory of them (docs chapter above; `plan/reports/25-ordinary-logic.md`).
- **Certificates.** `--format rocq` writes a term over `Prop` that needs no library (classical ones use `NNPP`), and for a classical "not valid" sequent the lemma that the formula fails for some assignment, proved by instantiating the atoms with `True` and `False` (`docs/src/status.md`, "Certified refutations"; `docs/src/ordinary-logic.md`). For intuitionistic and minimal "not valid" there is a verdict ("the image is unprovable: the search was exhaustive") and no countermodel and no certificate.
- **What it decides, measured.** At 2 s on one thread per problem, call-by-name decides 96 of the 274 ILTP problems, call-by-value 88, 0/1 68, 108 together; classical logic decides 156 (`plan/reports/25-ordinary-logic.md`). Intuitionistic answers can be "unknown" (exit status 3) because the search deepens a copy bound. Of the 166 problems no translation decides, 35 (all non-theorems, SYJ207 to SYJ211, SYN392, SYN393) deepen past a bound of 30 with every level cheap, which a loop check would refute; the other 131 are too expensive per level (same report, and `plan/later.md`, "Ordinary logic through its embeddings" and "Follow-ups: ordinary logic").
- **Known gaps recorded in `plan/later.md`.** Termination via Dyckhoff's LJT or a loop check (deferred); Kripke countermodels (listed among the escalations of certified refutations: "a deep embedding of LJ, the soundness of Kripke semantics proved once, and a finite Kripke model as the certificate"); `interact --logic` (a session over the image whose display is read back, not built); a read-back that unfolds a shared proof exponentially (seven classical proofs of 8 to 32 million inferences could not be checked within 6 GiB; a read-back on the proof term as a DAG is proposed there).
- **First-order ordinary logic** is specified (`plan/notes/research/fo-embeddings.md`, `impact-fo-ordinary.md`, `corpus-fo-ordinary.md`) on top of the first-order linear engines, which exist; `docs/src/status.md` lists the ordinary layer as propositional only. This note does not re-propose it.
- **The web page.** The client has an "Ordinary" tab that is disabled ("comes later", `src/islands/TopBand.tsx` of linlog-prover.github.io); Learn lesson 11 teaches the translations (`src/content/lessons/11-ordinary.md`); design D57 makes linear versus ordinary a choice on the proof tab (`plan/web/design.md`).
- **Substructural logics natively.** Linear logic (MLL to full LL, Mix), affine mode (weakening, in every fragment), cyclic MLL, the Lambek calculus, proof nets, first-order linear logic (`docs/src/status.md`). Not built: relevant logic (contraction without weakening), bunched implications, display calculi, subexponentials (scoped in `plan/notes/research/scope/logics.md`, candidate logics-1, not assigned).

## 2. The field

### 2.1 The calculi

- **LK and LJ, G3 and G4.** Troelstra and Schwichtenberg's *Basic Proof Theory* presents G3c and G3i, in which weakening and contraction are admissible and the rules are invertible where possible [1]. A later paper cites the book for G3c and G3i and lecture notes show weakening and contraction depth-preserving admissible in both [2][3]. Negri and von Plato's *Structural Proof Theory* is the companion text (unsourced: not fetched in this session). linlog's checker already uses the G1 variants with explicit structure, "all derivable" from these, per `core-ordinary.md`.
- **Dyckhoff's LJT (also called G4ip).** A contraction-free calculus for intuitionistic propositional logic, in which proof search on a sequent terminates because every premise is smaller in a well-founded measure; the Journal of Symbolic Logic paper is volume 57 issue 3 (1992), pages 795 to 807, and Dyckhoff published a 2018 correction with a shortened proof of Vorob'ev's result (JSL 83(4), 1680 to 1682) [4]. Vorob'ev, Hudelmaier and Dyckhoff are credited with its completeness [5]. Coq's and Rocq's `tauto` tactic is documented as "a decision procedure for intuitionistic propositional calculus based on the contraction-free sequent calculi LJT* of Roy Dyckhoff" [6], so LJT is what working proof assistants use as their propositional decision procedure.
- **Formalisations.** Férée and van Gool formalised, in Coq, uniform interpolation by propositional quantifiers, defined "by mutual induction on Dyckhoff's calculus LJT" with correctness by induction on LJT proof size (CPP 2023) [5]. This is the paper `plan/later.md` cites for G4ip in Coq. (The paper's abstract in the search result was cut off; the statement about LJT comes from the IRIF seminar abstract and slides listed in [5].)
- **Multi-succedent intuitionistic calculi and Kleene's G3.** Named in the task; not sourced in this session (unsourced). The checker's LJ has at most one formula right (`core-ordinary.md`).
- **Labelled calculi for countermodels.** Negri shows, for G3I (a labelled calculus for intuitionistic logic) and others with geometric rules, that failed proof search with invertible rules yields a countermodel to the end-sequent (Logica Universalis 8, 2014; companion Tableaux 2013 paper) [7]. I could read abstracts only, not the rules or the termination argument.

### 2.2 Decision procedures and complexity

- **Classical propositional.** Validity (the tautology problem) is coNP-complete, by the Cook-Levin theorem for SAT [8]. Modern solvers use DPLL and conflict-driven clause learning [8].
- **Intuitionistic propositional.** PSPACE-complete (Statman, *Theoretical Computer Science* 9(1), 1979, pages 67 to 72), for validity in Brouwer's, Kripke's or Heyting's sense, and the implicational fragment is PSPACE-complete too [9]. Hudelmaier gave an O(n log n)-space decision procedure from a sequent calculus (JLC 3(1), 1993, 63 to 75), improving Ladner's PSPACE bound [10].
- **First-order.** Undecidable (standard; unsourced in this session). linlog's first-order linear engines use bounds and a three-valued answer.
- **SAT-based intuitionistic provers.** Claessen and Rosén's *SAT modulo intuitionistic implications* (LPAR 2015, LNCS 9450, 622 to 637) uses an incremental SAT solver, and claims scaling to very large problems and fitting an SMT framework [11]. Its implementation, `intuit`, is a Haskell program on GitHub whose README mentions TPTP syntax and conversion of benchmarks to the formats of IntHistGC and fCube [12]; the page shows 47 commits and no dates, and names its licence only as "a LICENSE file" (not stated on the page). Fiorentini and Ferrari's *General clauses for SAT-based proof search in intuitionistic propositional logic* (JAR 68(3), article 13, 2024, DOI 10.1007/s10817-024-09703-8) extends the clauses so that their semantics relates to Kripke semantics, reports that the implementation "in general outperforms the state-of-the-art provers for IPL" and lists countermodel construction among its keywords [13].
- **fCube.** An "efficient prover for intuitionistic propositional logic" by Ferrari, Fiorentini and Fiorino (copyright 2012), version 5:0:1 of 2024-03-14 in the Logtalk port by Paulo Moura, GPL-2.0-or-later; its interface prints or returns a counter-model when the formula is not provable [14]. The page does not name its method or complexity (I did not source that it is a tableau method).
- **ILTP.** The library has about 2800 propositional and first-order problems in 24 domains, in TPTP syntax with a status `(intuit.)` line; the page shows a last update of 2017-03-01; the licence is not stated on the page [15]. The 2005 paper by Raths, Otten and Kreitz describes the library and a test of provers (ft, JProver and others) [16]; I could not read its PDF, so I give no result figures from it. LLTP's translated files descend from these problems (`plan/later.md`).

### 2.3 Substructural variants

- **Relevant logics.** Urquhart proved the propositional logics E, R and T undecidable (JSL 49(4), 1984, 1059 to 1073); the pure implicational fragments of R and E are decidable (Kripke) [17]. LR (R without distribution) is decidable and its implication-conjunction fragment has no primitive recursive decision procedure (Urquhart, JSL 64(4), 1999, 1774 to 1802: the abstract says Meyer's thesis translates LR into R-implication-conjunction, and the lower bound combines Urquhart's 1990 technique with the method of Lincoln, Mitchell, Scedrov and Shankar) [18]. `plan/notes/research/scope/logics.md` states the KRIPKE program for LR; I did not re-read that source.
- **Affine and linear.** Propositional linear logic is undecidable; the full propositional linear affine logic, with multiplicatives, additives, exponentials and constants, is decidable (Kopylov, LICS 1995) [19]. linlog already decides affine mode with a copy bound and a three-valued answer.
- **Bunched implications.** BI was believed decidable; Galatos, Jipsen, Knudstorp and Ramanayake show it undecidable by encoding Wang tilings (arXiv 2603.01595, March 2026, LICS 2026), and the LICS abstract points out gaps in earlier decidability claims [20]. A decision procedure for BI therefore cannot exist; what exists is fragments (unsourced here: the symbolic-heap fragment of separation logic is in `scope/logics.md`).
- **Lambek, ordered and cyclic.** Built (section 1); `scope/logics.md` and step 35's report cover them.
- **Display calculi.** Not researched (unsourced); see section 4.

### 2.4 The tools

Date read 2026-11-08 for each. "Last activity": none of the GitHub pages showed a date of the last commit through the fetcher, so only commit counts are given.

| Tool | What it does | Licence | Link, activity |
|---|---|---|---|
| Carnap | Haskell framework and the carnap.io service for teaching formal logic; its sequent calculus mode (ProofJS) has propositional and first-order LK and LJ among its systems, with interchange and contraction applied automatically; natural deduction in many textbook systems; the Open Logic Project's sequent calculus `openLogicLK` and `openLogicFOLLK` | GPL-3.0 | [21][22][23] 2,469 commits; "dozens of colleges and universities" per its home page. The systems page mentions no intuitionistic, minimal or substructural system. |
| Logitext | interactive visualisation of LK and LJ-style inference, validating each rule application with Coq (needs a Coq with PGIP support and Ur/Web) | BSD-3-Clause | [24] 171 commits; one search result gives the last push as March 2021 (not seen on the page). A client-side reimplementation of LK and LJ exists on a personal page [25]. |
| Sequent Calculus Trainer | Java application of Kassel University: sequent calculus for propositional and first-order logic with equality, feedback on each step; a later version asks Z3 whether a sequent is valid and hints the next rule; the papers say it gives no help with finding strategies beyond that | BSD-3 | [26][27] no version or date on the page; papers 2015 and 2018 |
| LogicProof | interactive web tool for natural deduction and sequent calculus, propositional and first-order, classical and constructive; a study of 35 students (TEAL 2026 at FLoC) | not stated | [28] arXiv 2026-07-05; no tool URL on the page |
| Jape | "configurable proof calculator" with example encodings of natural deduction and several sequent calculi, and a manual "Roll your own Jape logic" | GPL-2.0 | [29] 3,612 commits; its creator, Richard Bornat, has died and the maintainers say they will support those who still use it |
| ProofWeb | Coq-based web system for practising natural deduction, tree and Fitch views, tracks use of automation; used in type theory and logic undergraduate courses (2010 paper) | not stated | [30] sources from 2006 to 2010; live site not checked |
| Incredible Proof Machine | visual, graph-based proofs, configurable by YAML, with an Isabelle model in the Archive of Formal Proofs showing it is as powerful as natural deduction; for high-school students | MIT (per an index page) | [31] repository URL read: the first guess (nomeata/incredible-proof-machine) returned 404; the search result names nomeata/incredible |
| Pandora | Imperial College Fitch-style natural deduction tool, feedback on rule application | not stated | [32] |
| AProS / Carnegie Proof Lab | automated proof search engine, proof tutor and the Logic and Proofs course, in natural deduction | not stated | [33] |
| Sequent Calculus Calculator | 2019 bachelor thesis at OST Rapperswil by Matteo Kamm and Mike Marti | all rights reserved | [34] |
| Rocq `tauto`, `intuition`, `firstorder` | LJT-based decision procedure for intuitionistic propositional goals inside the proof assistant; `rtauto` builds its proof by reflection from a sequent-calculus proof | Rocq's own | [6] |

What the table shows (inference): the teaching tools for ordinary sequent calculi check or hint rule applications but, from what their pages say, decide with an external solver at most (the Trainer with Z3) and none of the pages I read mentions Kripke countermodels, step-by-step proving with an independent checker, proof certificates for a proof assistant, or a choice among several embeddings. The provers that produce countermodels (fCube, `intuit`, Fiorentini and Ferrari's) are command-line research tools with no interactive proving. The combination "decide, check, step, export, certify, countermodel" for the ordinary calculi is not in any tool read.

## 3. Candidates

Costs are in sessions of the plan's kind (one model's working day producing reviewed, tested code).

### ordinary-sequents-1: A native intuitionistic engine on Dyckhoff's LJT, with a loop-free guarantee

*What it brings and to whom.* Today intuitionistic and minimal logic can answer "unknown" and decides 108 of the 274 ILTP problems at 2 s (section 1). Dyckhoff's calculus makes proof search terminate [4], is what Rocq's `tauto` uses [6], and its decision procedure is within PSPACE, the optimum for the logic [9][10] (inference: no implementation of it can be asymptotically better in space than a PSPACE procedure unless PSPACE collapses; the point is that termination is built in). Teachers get an answer for every propositional formula; researchers get a second engine to cross-check the embeddings (the benchmark's contradiction check, as for the other engines); the AI models that read `skills/linlog/` get "valid" or "not valid" instead of exit 3 on small intuitionistic problems.

*What a suite offers.* Decide: every propositional intuitionistic and minimal sequent, no copy bound. Check: the LJT derivation is checked by rules of its own (the checker's LJ guards plus the LJT rules; the LJT implication-left rules are not derivable by single LJ inferences, so either the checker learns the five G4ip rules or the engine's proof is rewritten to LJ by derived rules; which of the two is an open design point, inference). Prove step by step: the rules are invertible-first and the goal state is small. Export: the exporters draw a `Derivation`; a new rule set is names in the same `Rule` enum. Certify: a Rocq term over `Prop` from the LJT proof, as `rocq.rs` does from LJ. Searchable: yes, fully.

*Cost.* Two sessions: the engine (a few hundred lines; memo on the sequent as a multiset; the loop-free measure), then the derivation form, checker rules, exports and Rocq term, with the differential test against the embeddings on the 274 ILTP problems and the generated families. The trusted base grows by the LJT rule table of the checker; the engine is untrusted, as now.

*Risk.* Proof search in LJT is PSPACE-complete in the worst case [9], so hard ILTP problems (the large SYJ2xx) may still time out, as they do for every prover; the 131 problems "too expensive per level" may stay expensive (inference). Redundant proofs: LJT proofs are not LJ's shortest ones (unsourced).

*Prerequisites.* None in the kernel: the ordinary arena, parser and checker exist. A `Method` choice on the ordinary journey (embedding or native) is an API addition; the web page's ordinary tab (disabled) would offer the same choice.

*Sources.* [4][5][6][9][10]; `plan/reports/25-ordinary-logic.md`.

### ordinary-sequents-2: Kripke countermodels as checkable certificates

*What it brings and to whom.* For an intuitionistic or minimal "not valid" sequent linlog today says only "unprovable" with no reason, where classical "not valid" gets an assignment and a Rocq lemma. A finite Kripke model that forces the antecedents and refutes the succedent is the standard explanation (fCube's interface prints a counter-model [14]; Negri's labelled calculi give one from a failed search [7]; Fiorentini and Ferrari list countermodel construction among the points of their prover [13]). Teachers show it, students check it by hand; researchers get a certificate. `plan/later.md` already lists it.

*What a suite offers.* Decide: as candidate 1. Check: a model checker for finite Kripke models (forcing by a recursion over the formula, polynomial, a few dozen lines) that is trivially independent of the search. Prove step by step: a lesson "find the step that went wrong, then the refutation" (design D56 in `plan/web/design.md`) can show the model. Export: LaTeX and Typst via TikZ or Typst's tools, SVG as a graph (the Euler-font drawing path exists); the JSON form for the web page. Certify: the Rocq library proves the soundness of Kripke semantics for LJ once (a deep embedding of LJ, as later.md says) and the model is the certificate; `Disproof::check`'s pattern exists for the other refutations. Searchable: yes, as a by-product of candidate 1's search or of a countermodel search of its own.

*Cost.* Two sessions in linlog (model type and checker; extraction from failed search or a dedicated search, exports, JSON) and about one in the Rocq repository (the deep embedding of LJ and soundness). The Rocq-checked part enlarges the trusted base with the Kripke soundness theorem; the model checker alone adds a few dozen lines.

*Risk.* How to extract a model from a failed LJT (G4ip) search is not settled by what I read: the labelled-calculus route [7] yields models from invertible-rule failure but I could not read its termination argument, and the SAT-based provers produce models in their own way [13] (full text not reachable). A research half-session is needed to choose the route (inference). Countermodel size can be exponential, since the logic is PSPACE-complete (inference, no source for a size bound). Minimal logic needs the model to give false no special status (an atom that may be forced anywhere), which is simple.

*Prerequisites.* Candidate 1 for the cheap route; otherwise a dedicated search.

*Sources.* [7][13][14]; `plan/later.md` (escalations, Kripke countermodels).

### ordinary-sequents-3: Interactive proving in LK, LJ and their variants, with the ordinary tab

*What it brings and to whom.* The teaching tools in section 2.4 are mostly about applying rules by hand with feedback; linlog has this for linear logic (`interact`, the web stage, Learn lessons) and not for ordinary logic, where `interact --logic` is a recorded follow-up and the web "Ordinary" tab is disabled. Students of a first logic course meet LK and LJ, not linear logic (inference from the courses the tools target: Carnap, the Trainer and ProofWeb [21][26][30]). This candidate brings the existing step-by-step machinery to that audience, with the existing hints and "close this goal" by search, and the exports.

*What a suite offers.* Prove step by step: a goal state of ordinary sequents, rules chosen by connective and side, G1 or G3-style rule sets as a display option (the same proof shown with explicit structure or with admissible structure, a teaching point of [1][3]); the hint is the search's next move. Decide and certify: as today, or candidates 1 and 2 if built. Check: the existing `Derivation::check`. Export: existing. Two designs differ: (a) a session over the image whose display is read back, as later.md proposes, which reuses the linear session and the proof net and read-back but shows ordinary goals only after an inverse mapping; (b) a native goal state of ordinary sequents whose rules are the checker's rules, simpler to explain and independent of the translations (inference). I recommend (b) because the checker's rule table is already the specification, and the session needs only a goal list and rule application, and because design (a) carries the translations' `!` bookkeeping into every goal (inference from `core-ordinary.md`).

*Cost.* Two sessions (the native state and its JSON; the web tab with a lesson), after candidate 1 optionally. Trusted base: none added, the checker validates the finished derivation.

*Risk.* Duplicates the linear session's machinery for a second goal type unless the kernel is generic (see section 5). The web page's design (D57) already allows the choice on a tab.

*Prerequisites.* A decision on whether the session state is generic over the calculus (section 5).

*Sources.* [1][3][21][26][30]; `plan/later.md` ("Follow-ups: ordinary logic"), `plan/web/design.md` D56, D57; `TopBand.tsx`.

### ordinary-sequents-4: A read-back on the proof DAG, so that large classical proofs can be checked

*What it brings and to whom.* Seven classical proofs (8 to 32 million inferences when unfolded) could not be checked within 6 GiB (`plan/later.md`); this blocks "valid" for large classical problems at the checker, not at the search. A checker on the shared proof (a DAG of subproofs) removes the blow-up. It serves anyone batching large classical problems (ILTP's SYJ2xx are classical-valid in part; the report says 116 of the 274 classical problems are unknown at 2 s for cost of search, not of the checker).

*What a suite offers.* Check and export at scale; no new logic. Prove, decide: unchanged.

*Cost.* One session; adds a second checker form to the trusted base (the DAG checker) unless it replaces the tree checker, which should happen in the same change with the mutation run extended.

*Risk.* Low in principle; the tree view for display remains.

*Prerequisites.* None. It is an engineering item, listed here because it concerns the ordinary layer and is not assigned.

*Sources.* `plan/later.md`, "Follow-ups: ordinary logic".

### ordinary-sequents-5: A classical engine of its own on G3c (and a SAT hand-off), only if classical decision matters

*What it brings and to whom.* Classical logic through affine MALL "costs one branch per way through the conjunctions" (later.md) and decides 156 of 274 at 2 s. A dedicated G3c prover with invertible rules and memoisation, or a clause-based method with a SAT solver (CDCL is the standard technique [8]), would decide more and give a falsifying assignment directly (linlog already has one from the linear refutation's classical reading). The later.md judgement is "not a SAT solver, and not to be offered as one", and no source read in this session shows teachers or models needing speed on large classical problems (inference).

*What a suite offers.* Decide: more problems; the proof read back or a resolution refutation. A SAT dependency would need a new certified path (DRAT or LRAT, which `plan/later.md` names as the pattern).

*Cost.* One session for G3c with memo; two to three for a SAT back-end with a certified refutation.

*Risk.* A SAT crate is a dependency that must earn its place (CLAUDE.md conventions); WebAssembly builds constrain it. Lowest priority of the five.

*Prerequisites.* None.

*Sources.* [8]; `plan/later.md`; `plan/reports/25-ordinary-logic.md`.

### ordinary-sequents-6: A structural-rule dial for the ordinary calculi (teaching the substructural cube)

*What it brings and to whom.* Teachers of substructural logic show the same implication-conjunction connectives with weakening, contraction and exchange switched on or off (linear, affine, relevant, BCK, ordered). linlog has linear, affine and Lambek/cyclic natively; relevant (contraction without weakening) is the one missing corner, and R is undecidable [17] and its decidable fragment LR has no primitive recursive procedure [18], so a decider must be bounded like the linear exponentials, with the three-valued answer (inference). Evidence of demand: none found in the tools read; Carnap's systems page lists no substructural system [23] (inference: little teaching supply, perhaps little demand).

*What a suite offers.* A signature of structural flags on a sequent (`scope/logics.md` logics-1, SELL, is the general form); checking and stepping are straightforward, deciding is bounded only.

*Cost.* One to two sessions as a mode of the existing engines after the SELL candidate; not assigned.

*Risk.* Undecidable in general [17]; the decidable fragments are exotic.

*Prerequisites.* The structural-rule signature of `scope/logics.md` logics-1.

*Sources.* [17][18][19]; `plan/notes/research/scope/logics.md`.

## 4. Considered and rejected

- **Bunched implications and BI provers.** Undecidable (LICS 2026) [20]; the earlier decidability results had gaps, so a decision procedure would be unsound. Not worth a suite; only fragments (the symbolic-heap fragment of separation logic) could be, and `scope/logics.md` covers that.
- **Relevant logic R, E, T as decision targets.** Undecidable [17]. Only as a bounded mode (candidate 6).
- **Display calculi.** I did not find a source in this session that shows a working teaching or proving tool for them (unsourced); they serve proof-theory research on modal and substructural logics more than a first-course audience. Left to the modal-logic note if it asks.
- **Replacing the embeddings by native calculi.** Rejected, see the next paragraph.
- **A natural-deduction teaching tool for the ordinary logics inside this area.** Natural deduction is its own area of the author's list; Pandora, ProofWeb, Carnap, AProS and the Incredible Proof Machine are in the table for completeness [21][30][31][32][33].
- **An LLM-facing "hint" service.** Not an ordinary-logic question; the guide for AI models exists.

**Replace or sit beside?** Beside (inference). What the read-back of the embeddings already gives: a checked LK or LJ derivation, in every output, with the independent checker; Rocq certificates of validity over `Prop`; for classical non-validity an assignment and a lemma; one engine family, so that every improvement of the focused engine (the parallel runtime, the memo) serves ordinary logic as well; and the translations themselves, which Learn lesson 11 teaches. What it does not give: a verdict for every intuitionistic formula (the copy bound), a countermodel for intuitionistic non-validity, and speed on large problems (116 classical and 166 intuitionistic ILTP problems undecided at 2 s). What a native LJT engine gives and the embedding cannot: termination, a countermodel route, and a second, independent implementation to compare verdicts with (the differential check that found bugs earlier, `core-ordinary.md` on minimal logic and G4ip). The embeddings' value as the unified view of "ordinary logic inside linear logic" is lost if they are removed. So the plan is a `--method` choice: `embedding` (today's) and `native`.

## 5. What the structure must allow

From `.claude/rules/core.md` and `core-ordinary.md` (read), judged here:

- **Reusable as is:** the hash-consed formula arena `Formulas` and the two-sided `Sequent` of `core/src/ordinary/`; the parser in both dialects and the TPTP reader; `Derivation`, `Inference`, `Rule` and the checker, which already carry LK, LJ and minimal logic with a position-given principal formula; the exports' derivation drawers (`latex::ordinary`, `typst::ordinary`, `svg::ordinary`) and the Rocq writer over `Prop`; `Outcome` and the batch; limits, stop and statistics conventions.
- **Linear-specific:** the forest and occurrences (`OccId`), the focused and net engines, the proof terms over subformula occurrences, the interactive session's state (a forest with positions), and the translations' pattern table. A native ordinary engine and a native ordinary goal state would not use them.
- **What must allow:** a rule table that is data (so LJT's rules, G1 and G3 sets are entries, not new code paths); an engine choice on the ordinary journey; a `Derivation` whose rules are an open set per calculus (the Rocq library's deep embeddings already work this way for LK and LJ); and a session state generic over goals, or two parallel session types, which the author's custom-rules direction (another note) will decide. A countermodel is a new kind of certificate beside `Disproof`; it needs a type and a JSON form, both new.

## 6. Sources

Read 2026-11-08 unless noted. Where the fetcher returned only a search summary, the line says so.

1. Troelstra and Schwichtenberg, *Basic Proof Theory*, Cambridge Tracts 43 (1996, 2nd ed. 2000): https://www.cambridge.org/core/books/basic-proof-theory/gentzen-systems/B273AB679C77FBDA4A534EC6A616A372 (via search result summary).
2. Absorbing the structural rules in the sequent calculus with additional atomic rules: https://arxiv.org/pdf/1810.11407 (search summary).
3. Universal Proof Theory, TACL 2022 lecture notes: https://arxiv.org/pdf/2305.10888 (search summary).
4. Dyckhoff, Contraction-free sequent calculi for intuitionistic logic, JSL 57(3), 1992: https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/contractionfree-sequent-calculi-for-intuitionistic-logic/C7ABBFC63E74878E52FF4E624029503B; correction: DOI 10.1017/jsl.2018.38 (search summary).
5. Férée and van Gool, Formalizing and computing propositional quantifiers, CPP 2023: https://ipqcoq.github.io/cpp2023fereevangool-prepub.pdf, https://www.irif.fr/seminaires/types/types2022, https://www.irif.fr/_media/gt-scalp/journees-2022/sam-scalp-cirm.pdf (search summaries only; the arXiv 2402.10494 abstract is about modal logics K, GL and iSL and does not mention LJT).
6. Rocq reference manual, solvers for logic and equality: https://rocq-prover.org/doc/v9.1/refman/proofs/automatic-tactics/logic.html (search summary of the page).
7. Negri, Proofs and countermodels in non-classical logics, Logica Universalis 8, 2014: https://www.mv.helsinki.fi/home/negri/procou_lu_rev.pdf; Tableaux 2013: https://www.mv.helsinki.fi/home/negri/negri_tableaux_2013.pdf (abstracts via search; PDFs not read).
8. Boolean satisfiability problem: https://en.wikipedia.org/wiki/Boolean_satisfiability_problem
9. Statman, Intuitionistic propositional logic is polynomial-space complete, TCS 9(1), 1979: https://deepblue.lib.umich.edu/handle/2027.42/23534 and https://portal.mardi4nfdi.de/wiki/Publication:1259589 (search summary; full text not read).
10. Hudelmaier, An O(n log n)-space decision procedure for intuitionistic propositional logic, JLC 3(1), 1993, DOI 10.1093/logcom/3.1.63 (search summary): https://www.bohrium.com/paper-details/an-o-n-log-n-space-decision-procedure-for-intuitionistic-propositional-logic/812031645379985408-2722
11. Claessen and Rosén, SAT modulo intuitionistic implications, LPAR 2015, DOI 10.1007/978-3-662-48899-7_43: https://research.chalmers.se/en/publication/529757
12. intuit: https://github.com/koengit/intuit
13. Fiorentini and Ferrari, General clauses for SAT-based proof search in intuitionistic propositional logic, JAR 68(3), 2024, DOI 10.1007/s10817-024-09703-8: https://irinsubria.uninsubria.it/handle/11383/2178111 (search summary of the abstract).
14. fCube (Logtalk port): https://logtalk.org/apis/fcube_0.html
15. ILTP: https://www.iltp.de/
16. Raths, Otten, Kreitz, The ILTP library, 2005: https://www.cs.cornell.edu/info/people/kreitz/PDF/05tableaux-iltp.pdf (PDF unreadable by the fetcher; known only from search summaries).
17. Urquhart, The undecidability of entailment and relevant implication, JSL 49(4), 1984: https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/undecidability-of-entailment-and-relevant-implication/FC9AE61D959740AB0356AEE1F5BC70E8 (search summary of the abstract).
18. Urquhart, The complexity of decision procedures in relevance logic II, JSL 64(4), 1999: https://resolve.cambridge.org/core/journals/journal-of-symbolic-logic/article/complexity-of-decision-procedures-in-relevance-logic-ii/C2C773998D8D629630ACEA0E64203A0F (search summary).
19. Kopylov, Decidability of linear affine logic, LICS 1995: https://lics.siglog.org/archive/1995/Kopylov-DecidabilityofLinea.html (search summary).
20. Galatos, Jipsen, Knudstorp, Ramanayake, The logic of bunched implications is undecidable, arXiv 2603.01595, LICS 2026: https://arxiv.org/abs/2603.01595 and https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.LICS.2026.46 (search summary).
21. Carnap repository: https://github.com/Carnap/Carnap
22. Carnap home: https://carnap.io/ and about: https://carnap.philosophy.ubc.ca/about (search summary).
23. Carnap manual, supported systems: https://carnap.github.io/Carnap-Manual/systems.html and sequent calculus chapter https://carnap.github.io/Carnap-Manual/sequent-calculus.html (search summary).
24. Logitext: https://github.com/ezyang/logitext
25. Client-side Logitext reimplementation: https://www.hedonisticlearning.com/logic
26. Sequent Calculus Trainer, Kassel: https://uni-kassel.de/eecs/en/tifm/software/sequent-calculus-trainer
27. Ehle, Hundeshagen, Lange, The Sequent Calculus Trainer, https://arxiv.org/pdf/1507.03666 and with automated reasoning https://arxiv.org/pdf/1803.01467 (search summaries).
28. Perhac, Khashcha, Novotny, LogicProof, arXiv 2607.04321: https://arxiv.org/abs/2607.04321
29. Jape: https://github.com/RBornat/jape
30. ProofWeb: search summaries of Hendriks, Kaliszyk, van Raamsdonk, Wiedijk, Teaching logic using a state-of-the-art proof assistant, Acta Didactica Napocensia 2010: https://doaj.org/article/7f94b26d134b46f7ada5c91730198ed8
31. Incredible Proof Machine: https://www.joachim-breitner.de/publications/Incredible_ITP2016_preprint.pdf, https://isa-afp.org/entries/Incredible_Proof_Machine.html (search summaries; the GitHub URL guessed first returned 404).
32. Pandora: https://www.doc.ic.ac.uk/pandora (search summary).
33. AProS: https://www.andrew.cmu.edu/user/dawnm/landp/index.html (search summary).
34. Sequent Calculus Calculator: https://orix.ost.ch/id/eprint/798/

Repository files read: `docs/src/status.md`, `docs/src/ordinary-logic.md`, `.claude/rules/core-ordinary.md`, `plan/46-research-next.md`, `plan/later.md`, `plan/reports/25-ordinary-logic.md`, `plan/notes/research/scope/logics.md`, `plan/web/design.md`, the client's `TopBand.tsx` and `src/content/lessons/11-ordinary.md`, `skills/linlog/SKILL.md`.

## 7. Not verified

- Last-commit dates of every GitHub repository (the fetcher showed commit counts only); licences of intuit, ProofWeb, Pandora, AProS and LogicProof.
- fCube's method and complexity; how fCube, `intuit` and the labelled calculi construct countermodels, and any bound on countermodel size.
- Full texts of Statman, Hudelmaier, Urquhart, Kopylov, Negri and Férée and van Gool (abstracts and secondary summaries only); the Raths, Otten and Kreitz results on provers (PDF unreadable); `iltp.de/provers.html` returned 404, so I give no ILTP prover results.
- Negri and von Plato's book, Kleene's G3, multi-succedent calculi, display calculi, first-order undecidability and the symbolic-heap decidability claim: not sourced here.
- Whether any tool other than those listed offers countermodels with an independent checker and proof certificates; I searched only for the tools named in the task.
- The claims that LJT proofs are not LJ's shortest and that a native LJT checker needs the five G4ip rules rest on my reading of the calculus, not on a source read here (inference).
- Nothing was run; the one thing I would have run is `linlog prove --logic intuitionistic --quiet` on the ILTP set at a larger time limit to see how many of the 131 "too expensive" problems a native engine would need to beat, which the supervisor can order if the ranking needs it.
