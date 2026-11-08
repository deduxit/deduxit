# Natural deduction: what to add after 0.2.0

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

## 1. What linlog has today

- Nothing of natural deduction as such. A grep of `core/src` and `docs/src` for "natural deduction" finds no hit; the phrase occurs only in plan notes (`plan/notes/comparison.md` row for LinearOne, which outputs natural deduction; `plan/notes/research/scope/education.md`; `plan/notes/research/web-precedents.md`; `plan/web/design.md` D-notes about the Incredible Proof Machine as a precedent).
- Sequent-calculus derivations, one- and two-sided, of every linear fragment, printed, drawn and exported (`docs/src/status.md`, "Proofs as compact terms", "Export of sequents and derivations").
- Interactive proving of sequents with open goals, undo, cut, translation of a finished derivation to a checked term (`docs/src/interactive.md`, status.md "Interactive proving").
- Ordinary classical, intuitionistic and minimal propositional logic through embeddings, with the linear proof read back as an LK or LJ derivation that `core/src/ordinary/derivation.rs` checks (`.claude/rules/core-ordinary.md`).
- Proof nets and cut elimination on terms and on nets.
- The proof term is an arena of rule instances over subformula occurrence ids, a node never records its sequent, and the checker derives it (`.claude/rules/core-proofs.md`). There is no hypothesis-discharge data model and no lambda-term printer. `plan/notes/research/scope/types-higher.md` proposed one as candidate types-higher-2 (printers for ILL proofs as lambda terms, two-sided only); it is unassigned, and this note's candidate 2 builds on it rather than repeating it.

None of the above may be re-proposed.

## 2. The field

### 2.1 The systems

- Gentzen's trees, Jaskowski's boxes and Fitch's indented subproofs are the three formats. The Stanford Encyclopedia entry says Gentzen's N calculi use trees with numerals on leaves marking subproofs that are crossed out when closed, Jaskowski used boxes around suppositions, Fitch uses vertical lines with each supposition above a short horizontal line; converting between formats is mostly rearranging the proof; proof theorists prefer trees, textbooks Fitch [1].
- "Sequent natural deduction" keeps elimination rules but writes the open assumptions as an antecedent; textbooks often call that natural deduction [1, section 4]. This is the form closest to linlog's two-sided sequents (inference).
- Normalisation: Gentzen's normal form theorem removes introduction-elimination detours (immediate reductions) and, for disjunction and existential elimination, permutative ones; normal intuitionistic proofs have the subformula property; Prawitz proved normalisation for natural deduction in 1965 and von Plato and Siders gave a general classical result in 2012 [1, section 5]. Prawitz's book (Dover reprint 2006) has chapters on Gentzen-type natural deduction, the inversion principle and normal deductions in classical logic [2].
- Modal logic: Fitch (1952) defined systems for S4 and T whose box introduction uses a strict subproof with restricted reiteration, and diamond elimination mirrors existential elimination; normalisation is generally weaker than in first-order logic [1, section 6]. Later Fitch-style work: Borghuis's calculus for intuitionistic K (a structural connective opens the strict subproof; it lacks the subformula property), Hawthorn 1990, Cerrato 1994, and Miranda-Perea et al. 2023, a sequent-style system that enforces the Fitch diagram's features by its rules [3]. Labelled "Kripke-style" systems attach a world to each formula instead [3].
- Linear logic: Benton, Bierman, de Paiva and Hyland gave a term assignment for intuitionistic linear logic for both sequent calculus and natural deduction, with a substitution property and typed reduction; the technical report states that the reduction from cut elimination and from normalisation are related to each other and to the categorical model [4]. The search summary of the paper states that a naive natural deduction formulation fails the substitution lemma (secondary reading, the full text was not read) [5]. Strong normalisation for the linear term calculus is by Benton, via translation into a second-order polymorphic lambda calculus [5].
- Sequent calculus and natural deduction: Zucker (1974) showed a close correspondence between cut elimination and normalisation for the fragment with conjunction, implication and universal quantifier, and claimed it fails with disjunction and existential; Urban's later analysis disputes the reason [6]. Negri and von Plato, "Structural Proof Theory" (Cambridge 2001), chapter 8 "Back to Natural Deduction" has natural deduction with general elimination rules and translations in both directions; their conclusion says cut-free proofs and normal proofs become essentially the same once weakening and contraction are explicit [7]. Their statements could not be read (paywall, see section 7).
- Focusing: Simmons argues focused sequent derivations correspond to normal natural deduction proofs for restricted fragments [8]; Liang and Miller's LJF is a focused intuitionistic system derived through linear logic [9]. Linlog's focused engines already search the sequent side (`.claude/rules/core-focus.md`).
- Logical frameworks: the Edinburgh Logical Framework (Harper, Honsell, Plotkin, J. ACM 40(1), 1993, DOI 10.1145/138027.138060) represents logics by "judgments as types" in a dependent typed lambda calculus [10]. That a natural deduction rule with discharge is an LF binder is standard but was not confirmed from the page read (unsourced here).

### 2.2 Decision problems

- Intuitionistic propositional validity is PSPACE-complete, and so is the implicational fragment, hence minimal implicational logic (Statman, TCS 9(1), 1979, pp. 67-72; a simpler semantic proof by Svejdar) [11]. Classical propositional is co-NP-complete (textbook fact, unsourced here).
- Checking a given natural deduction proof is a single pass over the proof (inference: each line or node is checked against its rule and the scope of its assumptions; linlog's checker is already one pass in memory proportional to the proof, status.md). I found no source stating a bound; it is not hard.
- Search in natural deduction is not subformula-free without normal forms; the practical route is to search in a sequent calculus or focused calculus and translate (inference from [7], [8]).

### 2.3 Tools

Each row: what, licence, activity, use.

- **Carnap**, https://carnap.io/, repository https://github.com/Carnap/Carnap [12][13]. Haskell framework, client compiled to JavaScript, GPL-3.0, 2,469 commits, 349 stars, 103 open issues (the page shows no commit dates; activity by date not confirmed). The site says it is used at "dozens of colleges and universities"; the 2025 experience report says over 45,000 students used it in the last decade and describes a bottom-up redesign with a Metamath Zero verifier kernel (mm0-zig) and a proof compiler "abc" [14]. The 2018 paper reports it was the main tool of three Kansas State courses in fall 2017, with anecdotal evidence only [15]. The README names no proof system; the 2018 paper is the place for the list (not read in full).
- **Incredible Proof Machine**, http://incredible.pm/, https://github.com/nomeata/incredible [16]. Haskell (GHCJS) core and JavaScript UI, MIT, 708 commits, pushes to master deploy automatically. Natural deduction as wired blocks (port graph, not a tree); the earlier scope note records ITP 2016 and a course evaluation (`plan/notes/research/web-precedents.md`, not re-read from the paper here).
- **Jape**, https://github.com/RBornat/jape [17], Bornat and Sufrin. GPL-2.0, 3,612 commits, "configurable proof editor, best suited to natural deduction and sequent calculus". The user describes the logic as inference rules and Jape builds proofs in it without built-in knowledge of a logic; a 1999 study used it with about 170 first-year students at Queen Mary; a stable release 9.1.8 of October 2023 is reported by a search summary [18] (secondary).
- **ProofWeb**, proofweb.cs.ru.nl, Radboud and VU, 2006-2007, main developer Cezary Kaliszyk [19]. Web interface over Coq (and Isabelle) with Gentzen tree and Fitch box views, a database of graded exercises, automatic grading, a parser that tells whether Coq's automation was used, free hosting for courses, following Huth and Ryan. Last activity not found.
- **Panda** (IRIT, Gasquet, Schwarzentruber, Strecker, TICTTL 2011) is a Java, Gentzen-tree tool; **Pandora** (Imperial, Broda et al., from 1996) is Fitch-box style, https://www.doc.ic.ac.uk/pandora [20]. Licences and activity not found.
- **fitch-checker / forall x proof checker**, https://proofs.openlogicproject.org/, https://github.com/OpenLogicProject/fitch-checker [21][22]. PHP and JavaScript, GPL-3.0, Fitch-style for truth-functional and first-order logic of forall x: Calgary and Cambridge; the repository says "NO LONGER MAINTAINED" and names LogicPenguin (github.com/frabjous/logicpenguin) as successor; only the successor's name was read [22][23].
- **NaDeA** (DTU: Villadsen, From, Schlichtkrull), https://nadea.compute.dtu.dk/, arXiv 1904.00618 [24]: a click-driven natural deduction assistant based on an Isabelle formalisation, used in the authors' bachelor course; the earlier scope note records that it shows only the rules that apply.
- **Holbert** (O'Connor and Amjad), arXiv 2210.11411 [25]: a pedagogical proof assistant and textbook platform with rule definitions and proof exercises embedded in the text, graphical proof manipulation; work in progress.
- **LogicProof, OnlineProver, Hazel Deriver, Logic Daemon**: evaluations of the first three (students, SUS scores) are recorded in `plan/notes/research/web-precedents.md`; not re-read here.
- **LinearOne** outputs natural deduction for first-order MILL (`plan/notes/comparison.md`); I did not read its output format.

The pattern in the evidence (inference from [14], [18], [19]): natural deduction tools are course tools, graded per exercise, with Fitch as the dominant student format and trees as the theorist's; the maintained ones with large use are Carnap and Jape; several are unmaintained.

## 3. Candidates

### natural-deduction-1: Checker and builder for Gentzen-tree and Fitch-style natural deduction (ordinary logic first)

- *What it brings and to whom.* Teaching above all (section 2.3: every tool in the field serves courses), and the AI models that use linlog (they would get a checker for a proof they wrote, as with `linlog check` today). linlog's own pitch is linear logic; ordinary propositional natural deduction is the widest audience and the one the author named. Evidence of use: [14], [18], [19].
- *What a suite would offer.*
  - Check: a text format for a Fitch proof (lines with formula, rule, line references, nested subproofs) and a tree format; the checker verifies rule instances, assumption scope and, at first order, the eigenvariable conditions. Linear time (inference).
  - Step by step: open goals as in `interactive.md`, but with the natural-deduction rules (introduction backwards, elimination forwards), assumptions opened by implication introduction and discharged on closing; hints from the existing deciders ("this goal is provable / is not").
  - Export: Gentzen trees reuse the ebproof and curryst writers (leaf labels for discharge, the numerals of [1]); Fitch needs a new layout (indentation bars) in LaTeX, Typst and SVG, with fitch.sty or an own drawing (the LaTeX package choice was not researched).
  - Decide: through the existing ordinary-logic deciders, so an unprovable goal is refuted with a countermodel.
  - Certify: a checked natural deduction proof into the Rocq library's `Prop` certificate of `core/src/ordinary/rocq.rs` (inference: the statement is the same formula).
- *Cost.* About 4 to 6 sessions: the data model and checker (2), interactive building with discharge (1 to 2), the two exports and SVG (1 to 2). Adds a second small checker to the trusted base for ordinary logic (comparable to the LK and LJ checker).
- *Risk.* The Fitch layout is new drawing work. Format choice is the teaching risk: courses use different rule sets and notations (Carnap supports several systems per its site, not enumerated here). Rule-set variation points to candidate 4.
- *Prerequisites.* A hypothesis-discharge data model (section 5); the ordinary syntax's formulas reused, no change of the linear core.
- *Sources.* [1], [12], [14], [19], [22].

### natural-deduction-2: Translation between sequent proofs and natural deduction, and the lambda-term read-back

- *What it brings and to whom.* Research and teaching: shows what the found proof is in the notation courses use, and ties the existing LK and LJ read-back to natural deduction. Researchers get Curry-Howard terms (the term assignment of [4]) for intuitionistic linear logic; programmers get types-higher-2's printers.
- *What a suite would offer.* Translate a checked LJ or LK derivation (already produced by `ordinary/`) to a normal natural deduction proof, and an ILL sequent proof to a linear lambda term; normalise a natural deduction proof step by step (named reductions: immediate and permutative, as cut elimination steps are named today) and show the correspondence to the cut-elimination steps linlog already has. Translating back is easier. Checkable throughout; the translation is not a search. For classical logic a natural deduction proof needs a rule for classical reasoning (double negation or reductio) or multiple conclusions; the translation of LK is therefore a design choice (unsourced; Prawitz covers normal deductions in classical logic [2]).
- *Cost.* 3 to 4 sessions after candidate 1 (translation LJ to ND, ILL to terms, normalisation steps with their checks). The trusted base grows only if the translation's output is not rechecked; it is rechecked by candidate 1's checker, so the translation need not be trusted (inference).
- *Risk.* Zucker's result shows the correspondence is not one to one with disjunction and existential, and the correct statement is disputed [6]; that bounds what "the same proof" may claim. The naive ND for linear logic fails the substitution lemma [5], so the linear case must follow [4] with care. Negri and von Plato's translations could not be read [7].
- *Prerequisites.* Candidate 1; for the linear case, types-higher-2 (a decision about it belongs there).
- *Sources.* [2], [4], [5], [6], [7], [8].

### natural-deduction-3: Linear natural deduction as a new calculus (tree style, ILL and IMLL)

- *What it brings and to whom.* Research and the AI users: Benton et al.'s natural deduction for ILL is the standard Curry-Howard reading [4]; none of the other linear provers in `bench/TOOLS.md` is described as offering it (LinearOne is the only one that prints natural deduction, `plan/notes/comparison.md`; its output form unread).
- *What a suite would offer.* A checker for ILL and IMLL natural deduction proofs with linear discharge (each assumption used exactly once, a count rather than a scope), an export of found ILL proofs as natural deduction trees (this is candidate 2's translation), and building step by step. Linear discharge is a use count, which linlog's occurrence counting resembles (inference).
- *Cost.* 2 to 3 sessions after 1 and 2. A third checker for a different mode set.
- *Risk.* The substitution property is exactly what a naive formulation lacks [5]; the rules of [4] must be taken from the paper (full text not read here). Classical linear logic has no standard natural deduction I could source (Troelstra's lectures were not read: unsourced). That half is better served by proof nets, which linlog has.
- *Prerequisites.* Candidates 1 and 2.
- *Sources.* [4], [5], [3].

### natural-deduction-4: Rule files, so that a course can define its own natural deduction rules

- *What it brings and to whom.* Teaching: Jape's model is rules as data, with the tool knowing no logic in advance [18]; Holbert embeds rule definitions in a textbook [25]; the Incredible Proof Machine takes logics as files (earlier scope note). Natural deduction tools differ chiefly in their rule sets (Fitch's, Gentzen's, textbook variants), so rule data covers variation that code does not. This overlaps another agent's area (custom rules); only the natural-deduction side is stated here.
- *What a suite would offer.* A rule schema (premises with local assumptions and eigenvariable side conditions, conclusion), used by the checker of candidate 1 and by step-by-step building; checkable and steppable, searchable only in the sense of the existing deciders where the rules coincide with a known logic (otherwise stepping only).
- *Cost.* 3 to 5 sessions, but best made as part of the shared kernel, not alone. It enlarges the trusted base to the schema interpreter.
- *Risk.* A user's rules may be unsound; the checker only checks derivability in the stated system (inference). Side conditions of quantifier rules are the hard part of any schema language [1, section 3].
- *Prerequisites.* A generic judgment and derivation model (section 5); candidate 1 as the first instance.
- *Sources.* [10], [18], [25].

### natural-deduction-5: Fitch-style modal natural deduction

- *What it brings and to whom.* Teaching and research: Fitch's strict subproofs for K, T, S4 [1, section 6], the intuitionistic K of Borghuis [3].
- *What a suite would offer.* A checker and step-by-step building with strict subproofs and restricted reiteration; no decision here (belongs to the modal suite). Export reuses candidate 1's Fitch layout.
- *Cost.* 2 sessions after 1, if the modal suite exists; otherwise only checking.
- *Risk.* Normalisation is weak for modal systems [1], rule sets vary by author [3]; without a modal suite there is no decider to hint with.
- *Prerequisites.* Candidate 1, and the modal area's semantics.
- *Sources.* [1], [3].

Order (inference): 1, then 2, then 3, with 4 designed alongside 1 so the schema is not retrofitted; 5 only with a modal suite.

## 4. Considered and rejected

- Graphical natural deduction in the style of the Incredible Proof Machine: a third representation no checker or export reads (earlier scope note), and wiring proofs are better left to the existing tool [16].
- A server with accounts, grade book and LTI as in Carnap and ProofWeb [14][19]: step 32's rule that nothing is sent to a server (earlier scope note). A course pack graded in the browser covers the core.
- Proof search directly in natural deduction: search is cleaner in sequent calculus or focused calculus, then translate [7][8]; linlog has the engines.
- Natural deduction for classical linear logic as a new calculus: no source found, and proof nets already serve (section 3, candidate 3).
- Competing with Carnap's catalogue of textbook systems: Carnap has over 45,000 users and years of work on it [14]; linlog's niche is linear and ordinary logic with certificates.

## 5. What the structure must allow

Reusable: the formula and ordinary syntax with the parser; the deciders as hint and refutation sources; the checker's one-pass style and its limits (time, memory, occurrence limit, no recursion on depth); the export pipeline (ebproof, curryst, SVG layout, options values); the interactive session shape (open goals, undo, JSON, a bound on size); the Rocq certificate path for ordinary logic.

Different: linlog's proof term is an arena of rule instances over occurrence ids with the sequent derived by the checker (`core-proofs.md`). Natural deduction has hypotheses discharged at a distance: a leaf names the rule application that discharges it, and the same formula can be discharged at several leaves. A data model for it needs hypothesis nodes with an identity, a discharge link from the closing rule to the leaves, and for Fitch a scope tree. It cannot be the occurrence-based term and should be a separate arena with the same discipline (premises precede conclusions, no recursion, size estimate before building). The checker for it is a scope check (a hypothesis may only be used below its binder and not outside the closing rule) and, at first order, an eigenvariable check; it does not need counts, except in the linear case (candidate 3). The exports' tree writers apply to the tree form; the Fitch layout is new. Rule schemas (candidate 4) are the place where a generic kernel would first be needed.

## 6. Sources

Read 2026-11-08.

1. Stanford Encyclopedia of Philosophy, "Natural Deduction Systems in Logic": https://plato.stanford.edu/entries/natural-deduction/ (the last 1,052 characters unread).
2. Prawitz, "Natural Deduction: A Proof-Theoretical Study", Dover 2006 reprint, catalogue record: https://nantilus.univ-nantes.fr/vufind/Record/PPN102137935/Description (bibliographic records only; book not read).
3. Search results for Fitch-style modal systems (Fitch 1952, Borghuis, Hawthorn, Cerrato, Miranda-Perea et al. 2023): https://www.cys.cic.ipn.mx/ojs/index.php/CyS/article/view/4542 and https://typeset.io/pdf/fitch-style-rules-for-many-modal-logics-15zxluq9si.pdf (search summaries, papers not read).
4. Benton, Bierman, de Paiva, Hyland, "A Term Calculus for Intuitionistic Linear Logic" for ILL, TLCA 1993, https://www.doi.org/10.1007/BFB0037099, and the technical report UCAM-CL-TR-262 abstract: https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-262.html.
5. Search results on the term calculus and its strong normalisation: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/tlca93.pdf and https://www.cambridge.org/core/services/aop-cambridge-core/content/view/DDBBD2A74BC4F0D5F9DDB1326DCD70E9/S0956796800001246a.pdf/strong-normalisation-for-the-linear-term-calculus.pdf (summaries; neither read).
6. Zucker, Annals of Mathematical Logic 7 (1974), DOI 10.1016/0003-4843(74)90010-2, and Urban's 2014 revisit: https://rd.springer.com/chapter/10.1007/978-94-007-7548-0_2 (search summary).
7. Negri and von Plato, Structural Proof Theory, CUP 2001: https://www.mv.helsinki.fi/home/negri/cont.html and https://www.cambridge.org/core/books/structural-proof-theory/conclusion-diversity-and-unity-in-structural-proof-theory/1D302570BA2ABD73FEBA5C2FA56D5DF8 (contents and conclusion only).
8. Simmons, focused sequent derivations and normal natural deduction: https://arxiv.org/abs/1109.6273 (search summary).
9. Liang and Miller, LJF: https://www.lix.polytechnique.fr/~dale/papers/tcs09fixed.pdf (search summary).
10. Harper, Honsell, Plotkin, "A framework for defining logics", J. ACM 40(1), 1993: https://projects.csail.mit.edu/jacm/References/harperhp1993:143.html (bibliographic).
11. Statman 1979: https://deepblue.lib.umich.edu/handle/2027.42/23534?show=full ; Svejdar: https://www1.cuni.cz/~svejdar/papers/sv_pspin.pdf (search summaries).
12. https://carnap.io/
13. https://github.com/Carnap/Carnap
14. "Carnap Ten Years Later: Lessons Learned and Next Steps": https://arxiv.org/abs/2607.07722
15. Leach-Krouse, "Carnap: An Open Framework for Formal Reasoning in the Browser": https://arxiv.org/pdf/1803.03092 (search summary).
16. https://github.com/nomeata/incredible
17. https://github.com/RBornat/jape
18. Jape background (search summary): https://en.wikipedia.org/wiki/Jape_(software) and https://www.ppig.org/files/1999-PPIG-11th-aczel.pdf
19. ProofWeb (search summary): https://core.ac.uk/download/16156617.pdf and https://doaj.org/article/7f94b26d134b46f7ada5c91730198ed8
20. Panda and Pandora (search summary): https://www.irit.fr/~Martin.Strecker/Publications/ticttl_panda2011.html and https://www.doc.ic.ac.uk/pandora
21. https://proofs.openlogicproject.org/
22. https://github.com/OpenLogicProject/fitch-checker
23. https://github.com/frabjous/logicpenguin (named by [22] only, not opened).
24. NaDeA: https://arxiv.org/abs/1904.00618
25. Holbert: https://arxiv.org/abs/2210.11411
Also read: linlog's own `docs/src/status.md`, `.claude/rules/core-proofs.md`, `plan/notes/research/scope/types-higher.md`, `education.md`, `plan/notes/research/web-precedents.md`.

## 7. Not verified

- Last-activity dates of every repository (the GitHub pages showed commit counts, not dates).
- The list of systems Carnap supports; licences and activity of Panda, Pandora, ProofWeb, NaDeA, Holbert.
- Jape's latest release date (secondary summary only).
- Full texts of [3] to [9]: claims about Zucker, Negri and von Plato, Simmons, LJF and the BBdPH substitution failure rest on abstracts or search summaries.
- Classical linear natural deduction, Troelstra's lectures on linear logic, the complexity of classical propositional validity and the claim that LF binders encode discharge: no source read.
- LinearOne's natural deduction output format.
- Which LaTeX package suits Fitch proofs.
- What I would have run: nothing.
