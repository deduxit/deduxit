# Modal logics (various kinds): what a suite for them would be

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

## 1. What linlog has today

- No modal logic as such. A search of `core/src`, `docs/` and `README.md` for "modal" finds nothing; the only hits in `plan/` are research notes and scope notes (`plan/notes/research/scope/logics.md`, `types-higher.md`, `engines.md`, `interop.md`, `scope-extensions.md`), which discuss modal-like extensions of linear logic (subexponentials, graded modalities, Lambek with modalities) and the TPTP's non-classical syntax, which today covers modal logics only.
- The exponentials `!` and `?` are in every full-LL engine, the MELL proof nets with boxes, and the Horn and inverse engines (`docs/src/status.md`). They behave like an S4 box and diamond (Girard 1987; Martini and Masini 1994, [1], [2]). Nothing in linlog reads them as Kripke modalities.
- Ordinary classical, intuitionistic and minimal propositional logic, decided through embeddings into linear logic with the proof read back as LK or LJ, with a TPTP reader (`core/src/ordinary/`, `.claude/rules/core-ordinary.md`). This is the pattern a modal embedding would follow.
- First-order syntax, eigenvariables and unification (status.md), cyclic MLL and the Lambek calculus (order of formulas kept), interactive proving, the exports (LaTeX, Typst, SVG, PNG, PDF), Rocq certificates and the library `rocq-linlog`, and the wasm bindings with the web page.
- Refutations of unprovable sequents (`Disproof::check`): a falsifying assignment or the inverse method's saturated set. There is no Kripke-model counter-example type.

## 2. The field

### 2.1 Logics and complexity

- Normal modal logics built from K and the axioms D, T, B, 4, 5 form the "modal cube" of fifteen distinct logics; Brünnler's deep sequent systems cover all of them [3] (stated in the paper's abstract as seen in search results; I did not read the paper's body).
- Ladner 1977: provability in K, T and S4 is log-space complete in PSPACE; non-provability in S5 is NP-complete, so S5 validity is coNP, and S5 has the complexity of classical propositional logic [4]. A survey states the same as "validity for all logics between K and S4 is PSPACE-complete, S5 NP-complete (satisfiability)", and locates the gap in negative introspection [5].
- Gödel-Löb logic GL: PSPACE-complete [6] (secondary: a 2013 paper calls it easy to prove, and a 2020 paper says Ladner's construction gives hardness; I did not see Chagrov and Zakharyaschev's own statement).
- Epistemic: for S5 with many agents, satisfiability is PSPACE-complete; adding common knowledge makes it EXPTIME-complete (Halpern and Moses 1992, as reported by [7]; the paper's full text was not retrieved).
- Temporal: LTL satisfiability with the full operator set is PSPACE-complete (Sistla and Clarke 1985, as reported by [8]). CTL: EXPTIME-complete is the usual statement, (unsourced here: the search found nothing).
- Intuitionistic modal: IK, IS4 and IK4. Decidability of IS4 and IK4 was open; a paper submitted 21 September 2026 (v2 of 8 October 2026) claims a decision procedure that returns a proof or a finite countermodel, and corrects a mistake in the authors' LICS 2023 paper [9]. It is a preprint read as an abstract; treat it as unreviewed.
- First-order modal logic: the domain assumptions (constant, cumulative, varying) change the logic; the QMLTP library covers K, D, T, S4, S5 under these three [10].
- Deontic and temporal logics are normal modal logics with D resp. extra axioms (inference); no source was read for deontic logic as such.

### 2.2 Proof systems

- Labelled sequent calculi (Negri 2005): a general method that turns frame conditions of relational semantics into rules for the accessibility relation, extending contraction-free calculi; cut elimination is shown directly; it gives the first contraction- and cut-free calculus for GL; G3K is the reference system. They lack the full subformula property [11] (search-result summary; the PDF at the author's page returned 404 for the direct fetch).
- Nested sequents (Kashima 1994; Brünnler 2009; Poggiolesi 2009 as "tree hypersequents"): deep sequent systems for the whole cube, with terminating proof search and syntactic cut elimination [3], [12]. They are internal (formulas only, no worlds as syntax).
- Hypersequents (Avron 1996): S5 is the standard example of a modal logic with an analytic hypersequent calculus [13] (Avron's chapter itself was not retrieved; Wikipedia and a bibliographic record were). Poggiolesi 2008 gives a cut-free sequent calculus for S5 [13].
- Prefixed tableaux (Fitting; Fitting and Mendelsohn's book, 2nd ed. Springer 2023) [14]; display calculi (Belnap, Wansing): no source was found, left unsourced.
- Hilbert systems and translation to first-order logic (standard translation, functional translation; Hustadt and Schmidt's survey on first-order resolution for modal logics) [15].

### 2.3 Tools

| tool | what | licence | last activity | source |
|---|---|---|---|---|
| MleanCoP | connection prover for first-order modal logic; D, T, S4, S5; constant, cumulative, varying domains; Prolog (ECLiPSe, SWI, SICStus); heterogeneous multimodal in v1.3 | GPL | page updated 2022-05-05 | [16] |
| nanoCoP-M | non-clausal connection prover for first-order multimodal logics, with prefixes encoding Kripke semantics; part of nanoCoP 2.0 | GPL, per the paper | not seen | [17] |
| MOIN | Prolog prover on nested sequents for the 15 classical modal logics of the cube and the decidable intuitionistic ones (IK and others), prototype for IK4, ID5, IS4; outputs a derivation (LaTeX) or a Kripke or bi-relational countermodel | no licence stated | page "2020" | [18] |
| MOILab | Prolog prover for IK on a labelled calculus | not seen | not seen | [18] (named in the search result) |
| CEGARBox | Haskell, K, KT, S4 satisfiability by SAT-based tableaux with clause learning; reported best on standard benchmarks | BSD-3-Clause | no date shown; 14 commits | [19], [20] |
| InKreSAT | OCaml reduction of K, T, K4, S4 satisfiability to SAT (MiniSat) | no licence stated | CADE-24, 2013; v1.0 | [21] |
| MSPASS | modal reasoner built on SPASS by translation to first-order logic and resolution | not seen | 2000 paper | [15] |
| LoTREC | generic tableau prover for modal and description logics in Java, with a graph-based rule language; accompanies the book "Kripke's Worlds" (2014) | not seen | not seen | [22] |
| MetTeL2 | generator of tableau provers from a calculus specification, Java output, web interface, models for satisfiable input | not seen | 2012 | [23] |
| Logics Workbench | automated K, KT, KT4, KT45, KW; Hilbert-style references (LWBtheory) | not seen | availability unconfirmed | [24] |
| InKreSAT, Spartacus, BDDtab | benchmarked together in later comparisons; Spartacus is a hybrid-logic tableau prover from a Saarland thesis | not seen | not seen | [21], [25] |
| Modal Logic Playground | web page: draw a Kripke model, evaluate a formula at each state; no proving; D3, MathJax | MIT | 63 commits, no date | [26] |
| Carnap | web framework with natural deduction for K5 and KTB after Hardegree, a naively quantified system, sequent calculi LK and LJ | not read | not read | [27] |
| QMLTP | 600 problems (580 unimodal, 20 multimodal; 421 first-order, 179 propositional) for K, D, T, S4, S5; IJCAR 2012; last updated about 2012 | not seen | ~2012 | [10] |
| TPTP World | supports non-classical (modal) logics since v9.0.0 through a logic specification `$modal` with box and diamond written `{$box}`, `{$dia}` | TPTP's | 2025 paper | [28], [29] |
| Isabelle AFP "Epistemic Logic: Completeness of Modal Logics" (From) | K to S5 completeness; Kripke models as a datatype | AFP | 2018 | [30] |
| Lean 3: Bentzen's S5 completeness; Wu and Goré's tableaux for K, KT, S4 with decision procedures proved sound and complete | see [31] |

"Molle" and "KSP" were named in the brief; no source was found for either (unsourced, not described).

The TPTP embedding route of Steen, Sutcliffe and Benzmüller solves QMLTP problems by translation to classical first- and higher-order logic, strong at disproving, and covers more modal logics than native modal provers [32].

### 2.4 The embedding analogy

- Girard's linear logic (1987) presents the exponentials as controlled access to weakening and contraction, with rules resembling those of S4's modalities [33]. The Girard translation sends an intuitionistic implication to a linear implication guarded by `!`.
- Martini and Masini (JSL 59(3), 1994): a sequent calculus for S4 leads to a translation into full propositional linear logic that extends the Grishin-Ono translation; linear modalities appear only where S4 modalities occur; they study the complexity of the decision problem on the resulting classes of linear formulas [2].
- Fukuda and Yoshimizu (FSCD 2019): the naive combination of the S4 box and `!` interacts badly; they merge them into one "bangbox" and get a sound translation of IS4 [34] (abstract level).

## 3. Candidates

### modal-logics-1: A decision procedure for K to S5 with Kripke countermodels

- *Brings, to whom*: teaching (a validity checker with a drawn countermodel is the missing half of the Kripke-model playground [26], which evaluates but does not prove) and research; for AI models using the guide, a command that answers valid or invalid with a certificate. Evidence of use: MOIN gives a derivation or a countermodel [18]; CEGARBox and InKreSAT show that K, T, S4 is a benchmark field with working solvers [19], [21]; Carnap shows classroom use of modal natural deduction [27]. No source was found for how many users these tools have.
- *In linlog's terms*: decide `⊨_L A` for L in K, T, D, B, K4, S4, S5 (and further cube logics) by a terminating search; return a proof in a named calculus or a finite Kripke model that falsifies the formula, checked apart from the search (as `Disproof::check` is for linear logic); the model drawn as SVG (worlds as nodes, accessibility as arrows, valuation inside; a hooking point in the existing SVG export). Checkable: both certificates. Searchable: the cube by a tableau or a labelled/nested search with loop checks for 4 (blocking or loop checking, the cause of PSPACE) [4]. Export: LaTeX/Typst of the derivation; the model as SVG and as TPTP-like text (inference). Certify: see modal-logics-3.
- *Cost*: 3 to 4 sessions for a first engine on K, T, S4, S5 with a countermodel and a model checker, 1 for the rest of the cube; adds to the trusted base only the model checker (a small evaluator of a formula in a finite frame) if the search is not trusted.
- *Risk*: performance against SAT-based solvers [19], [21] is not a goal here (inference: a teaching size, formulas of a few dozen subformulas, is enough); GL needs a different loop treatment (Negri's calculus for GL is the first cut-free one [11]); for logics with 4 and 5 the termination arguments differ per logic.
- *Prerequisites*: a formula arena with box and diamond (`sequents/` has binary and unary connectives, modal ones are new variants, in the line of `scope-extensions.md`'s "`Term`'s modal variants"); a model type; no use of the linear forest.
- *Sources*: [3], [4], [11], [18], [19], [26].

### modal-logics-2: Step-by-step proving in a labelled sequent calculus (G3K plus frame rules)

- *Brings, to whom*: teaching. Labels make the frame conditions visible: reflexivity, transitivity, symmetry, Euclideanness and seriality become rules on `xRy` that a student sees, and one calculus covers the cube by adding rules [11] (inference for "visible to students"; the uniformity is Negri's claim [11]). Research: the calculus is the base of the labelled provers like MOILab for IK [18] and of the general proof-certification work (the search results list "A general proof certification framework for modal logic" [35], abstract not read).
- *In linlog's terms*: interactive proving (like `interact`) of sequents `Γ ⊢ Δ` over labelled formulas `x:A` and relational atoms `xRy`; every rule offered applies to a chosen formula; the checker is a one-pass derivation checker (as for first-order eigenvariable conditions: the `□R` rule's fresh world is an eigenvariable, and linlog already checks eigenvariable conditions with levels). Checkable: yes, trivially a rule-by-rule check. Searchable: a proof search over labelled sequents terminates for K and with care (blocking) for 4 and S4; the frame-rule closure is what makes the search infinite without a loop check, as the 2013 implicit-contraction remark shows [11].
- *Cost*: 3 sessions beyond modal-logics-1 (data model for labels and relational atoms, checker, interactive rules, exports).
- *Risk*: the subformula property fails [11], so a "proof search by subformulas" with bitsets, as in the focused engine, does not carry over (inference); the search needs labels as growing state.
- *Prerequisites*: the data model change below (section 5): a sequent holds labelled formulas and relational atoms, not only a forest of formulas.
- *Sources*: [11], [18], [35].

### modal-logics-3: Certificates for modal derivations in `rocq-linlog`

- *Brings, to whom*: research and trust. linlog's identity is checked proofs with Rocq soundness theorems for the checkers; a modal derivation checker with a soundness theorem against Kripke semantics would extend the line. Evidence that the formalisation is feasible: Isabelle AFP has completeness for K to S5 [30], Lean has soundness and completeness of tableaux for K, KT, S4 [31], Bentzen's S5 proof [31].
- *In linlog's terms*: export a derivation as a Rocq term, with a small checker and a theorem: an accepted derivation implies validity on every frame of the logic; for refutations, a Kripke model and a proof that it falsifies the formula (the model is small and its check is computation).
- *Cost*: 2 sessions for soundness of labelled calculus for K, T, S4, S5; completeness is not needed for certificates (inference); the Rocq library is its own repository (`CLAUDE.md`). Adds to the trusted base nothing beyond the Rocq kernel.
- *Risk*: the first-order part of Rocq library is the reuse; labels need a new development.
- *Prerequisites*: modal-logics-1 or 2.
- *Sources*: [30], [31].

### modal-logics-4: Nested sequents as the second calculus (internal), compared with labelled

- *Brings, to whom*: research and tools. Nested sequents need no labels: a sequent is a tree of multisets, `□` introduces a new child, and the same calculus exists for the whole cube and for intuitionistic modal logics [3], [12], [18]. MOIN decides by it and extracts countermodels, and it is the calculus of a prover on SWI-Prolog [18].
- *Compared*: labelled calculi are uniform and keep the frame conditions as visible rules but lose the subformula property and need relational atoms [11]; nested ones keep the subformula property and have terminating proof search with syntactic cut elimination [3], but the structural rules for 4, 5 and B are rules that move formulas between nodes and are less transparent for students (inference). Poggiolesi's tree hypersequents are the same objects [3].
- *Verdict for a step-by-step tool (inference)*: labelled for the step-by-step tool, nested for the engine that searches, with the labelled sequent read off a nested one. Both for the same logics gives a cross-check, the way linlog already has two checkers or engines.
- *In linlog's terms*: a tree data type; the existing text tree and SVG export draw a tree; a search that is a variant of the focused engine over a tree.
- *Cost*: 3 sessions; no new trusted base if each derivation is translated to a labelled one and checked there (inference).
- *Risk*: the translation between them is a research-level piece of work; do only if modal-logics-1 shows the need.
- *Prerequisites*: modal-logics-1 or 2.
- *Sources*: [3], [11], [12], [18].

### modal-logics-5: A Kripke model editor and evaluator in the web page

- *Brings, to whom*: teaching. The Modal Logic Playground is the existing precedent: draw states and arrows, set valuations, evaluate [26]. In linlog's page this would be a tab whose stage draws a model and marks the worlds where a formula holds; with modal-logics-1 it turns a countermodel into an editable model.
- *In linlog's terms*: model checking (polynomial in model and formula, trivially checkable), a model JSON form, SVG export, frame-property detection (reflexive, transitive, symmetric, Euclidean: so "this model is an S4 frame").
- *Cost*: 1 session for the library part, 1 for the island in the client (not read: the client was not part of this note's reading).
- *Risk*: little; it duplicates a small MIT tool [26], so the gain is in integration with proving.
- *Prerequisites*: the formula arena of modal-logics-1.
- *Sources*: [26].

### modal-logics-6: S4 and IS4 into linear logic (the embedding as reuse)

- *Brings*: research. Martini and Masini give a translation of S4 into full propositional LL where the exponentials appear only at S4 modalities [2]; intuitionistic S4 has the "bangbox" translation [34]. linlog could decide the image with its MELL engines and read the proof back to an S4 sequent calculus, as `ordinary/` does for LK and LJ.
- *In linlog's terms*: translation table, read-back, checker over the read-back derivation; decide, check, prove step by step, certify as in the ordinary suite.
- *Cost*: 2 sessions after `core/src/ordinary/`'s pattern is read for reuse (inference); 1 more for IS4.
- *Risk*: the image is in full LL, whose propositional decision problem is undecidable, so completeness of the decision depends on the image falling in a decidable fragment (inference: Martini and Masini study the complexity of classes of the image [2]; the abstract does not say which is decidable). An S4 decided by Ladner's PSPACE bound [4] would be far faster directly (inference). The value is pedagogical and in the cross-check, not in speed.
- *Prerequisites*: the propositional modal syntax of modal-logics-1.
- *Sources*: [2], [33], [34], [4].

### modal-logics-7: Multimodal and epistemic logics (S5 with n agents, common knowledge)

- *Brings*: teaching and applications of "muddy children", the epistemic puzzles (inference, no source read for usage). Complexity: S5n PSPACE-complete for many agents, EXPTIME-complete with common knowledge [7]. MleanCoP v1.3 and nanoCoP-M handle multimodal logics [16], [17], and LoTREC is the teaching tool whose book is on these logics [22].
- *In linlog's terms*: agent-indexed boxes in the arena; models with one relation per agent; common knowledge needs fixpoint rules (cyclic or infinitary proofs, which relates to the author's "cyclic proofs" area, not read here). Decide and countermodel by the same engine as modal-logics-1; common knowledge only by a tableau with loop check (a separate engine).
- *Cost*: 1 session for n agents without common knowledge; 3 for common knowledge.
- *Risk*: EXPTIME makes teaching examples hard to scale; no sourced proof calculus for common knowledge was read.
- *Prerequisites*: modal-logics-1.
- *Sources*: [7], [16], [17], [22].

### modal-logics-8: First-order modal logic with a QMLTP and TPTP reader

- *Brings*: research and benchmarking. QMLTP has 421 first-order problems [10] and the TPTP World has accepted modal logics since v9.0.0 [28]; linlog's TPTP reader exists for ordinary logic (`core-ordinary.md`), and its first-order machinery (eigenvariables, unification) could be reused.
- *In linlog's terms*: a reader of TPTP `$modal` logic specifications [29]; a prover for K to S5 with constant, cumulative or varying domains [10], [16]; prefixed connection search is MleanCoP's way [16].
- *Cost*: 4 sessions or more; domain conditions change the quantifier rules.
- *Risk*: first-order modal logics are undecidable (inference from FOL); the Barcan and domain variants multiply the logics (Fitting and Mendelsohn [14]); the other provers are stronger (inference; no benchmark was run).
- *Prerequisites*: modal-logics-1 and -2.
- *Sources*: [10], [14], [16], [28], [29].

## 4. Considered and rejected

- A SAT-based modal solver for speed (CEGARBox, InKreSAT [19], [21]): a competitive field with solvers that stay ahead; linlog's stated goals are teaching and checked proofs, not modal SAT (inference).
- A generic tableau-prover generator like MetTeL2 or LoTREC [22], [23] as the first step: this is the "custom rules" question of the step's prompt and belongs to that note.
- Model checking of LTL or CTL: a different method (automata, model checkers), not proof sequents; LTL is PSPACE-complete [8]. Only a pointer to the cyclic-proofs area.
- Display calculi (Belnap, Wansing) and hypersequents for S5 alone [13]: no tool found that teaches them, no source for display calculi read; hypersequents help S5 only, where labelled calculi already work.
- Natural-deduction and Hilbert systems for modal logic as proof objects: Carnap already serves modal natural deduction in the browser [27]; a Hilbert checker adds little to teaching (inference).
- Deontic logic as a separate suite: it is D with a different reading (inference); a gloss in the modal suite is enough.

## 5. What the structure must allow

- Reusable (judging from `.claude/rules/core.md` and its module files): the arena and printer idea of `sequents/`, parsing and serialization, first-order eigenvariables and levels in the checker, the one-signature export machinery and the SVG, the interactive proving shell, the CLI and bindings pattern, the Rocq export route, and the pattern of `ordinary/` (a syntax, translations, read-back, checker).
- Linear-specific and not reusable: the occurrence forest with bitset occurrence sets and its linear counts, the focused engine and its memo, the inverse method, Horn engine, nets and the linear proof terms with the contexts split by `⊗`. A modal calculus needs weakening, contraction and sharing of contexts, so none of the engines serves it (inference).
- The data model asks for: formulas with box and diamond (agent-indexed for multimodal); sequents with labels and relational atoms (for labelled calculi) or trees of multisets (for nested); a Kripke model type with a checker, drawn and serialised; rule schemas parameterised by the logic's frame conditions, which is where the "custom rules" generic kernel would plug in (inference).

## 6. Sources (all read 2026-11-08)

1. Girard, "Linear logic", TCS 50 (1987): scan at <https://curien.galene.org/ECI2023/Girard-LL.pdf> (found in search results, not opened).
2. Martini, Masini, "A modal view of linear logic", JSL 59(3), 1994: <https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/modal-view-of-linear-logic/AA2232D491291E188ACBBE17BB8C6DC4> (abstract through search).
3. Brünnler, "Deep sequent systems for modal logic", Arch. Math. Logic 48(6), 2009: <https://sirius.bfh.ch/~kai/Papers/2009dssml.pdf>, DOI 10.1007/s00153-009-0137-3 (through search summary).
4. Ladner, "The computational complexity of provability in systems of modal propositional logic", SIAM J. Comput. 6(3), 1977: <https://core.ac.uk/display/103544924> (abstract).
5. Halpern, Rêgo, "Characterizing the NP-PSPACE gap in the satisfiability problem for modal logic": <https://arxiv.org/abs/cs/0603019>.
6. GL PSPACE: <https://arxiv.org/pdf/2012.15347> and <https://arxiv.org/pdf/1305.6065> (secondary).
7. Halpern, Moses, Artificial Intelligence 54(3), 1992, as reported in <https://arxiv.org/pdf/2302.04218>; bibliographic record <https://portal.mardi4nfdi.de/wiki/Publication:1193476>.
8. LTL satisfiability: <https://eccc.weizmann.ac.il/report/2006/153> (reporting Sistla and Clarke 1985).
9. Girlando, Kuznets, Marin, Straßburger, "A decision procedure for intuitionistic modal logic IS4 (and IK4)": <https://arxiv.org/abs/2609.24922>.
10. Raths, Otten, QMLTP, IJCAR 2012: DOI <https://www.doi.org/10.1007/978-3-642-31365-3_35>; description in <https://arxiv.org/pdf/2212.09570>; library <https://iltp.de/qmltp> (named in the search result).
11. Negri, "Proof analysis in modal logic", J. Phil. Logic 34 (2005): <https://www.mv.helsinki.fi/home/negri/paml.pdf> (direct fetch 404; read through search results), and the implicit-contraction paper <https://portal.mardi4nfdi.de/entity/Q377468>.
12. Nested sequents: <https://ar5iv.arxiv.org/html/1004.1845>, <https://www.lix.polytechnique.fr/~lutz/orgs/ESSLLI22notes-5B-10-NestedSequents--Handout.pdf>.
13. Avron's hypersequents: <https://philpapers.org/rec/AVRTMO>, <https://en.wikipedia.org/wiki/Hypersequent>; Poggiolesi S5: <https://cambridge.org/core/journals/review-of-symbolic-logic/article/cutfree-simple-sequent-calculus-for-modal-logic-s5/53D259C8484E461F94D48163910ED44E>.
14. Fitting, Mendelsohn: <https://www.springer.com/us/book/9780792353348>; 2nd ed. <https://www.schweitzer-online.de/en/buch/Fitting/First-Order-Modal-Logic/9783031407130/A68235765/>.
15. Hustadt, Schmidt, MSPASS: <https://www.cs.man.ac.uk/~schmidt/publications/HustadtSchmidt00b.html>; survey <https://www.cs.man.ac.uk/~schmidt/publications/SchmidtHustadt13.html>.
16. MleanCoP: <https://www.leancop.de/mleancop/>.
17. nanoCoP-M: <http://leancop.de/nanocop-m/> (named in a search result), <https://link.springer.com/chapter/10.1007/978-3-030-86059-2_14>.
18. MOIN: <https://www.lix.polytechnique.fr/~lutz/Software/Moin/MoinProver.html>, <https://hal-univ-tlse3.archives-ouvertes.fr/INRIA/hal-02457240>, MOILab <https://hal.archives-ouvertes.fr/hal-03048966>.
19. CEGARBox repository: <https://github.com/cormackikkert/CEGARBox>.
20. CEGAR-Tableaux: <https://researchportalplus.anu.edu.au/en/publications/cegar-tableaux-improved-modal-satisfiability-via-modal-clause-lea/>.
21. InKreSAT: <https://www.ps.uni-saarland.de/~kaminski/inkresat/>, <https://www.ps.uni-saarland.de/Publications/documents/KaminskiTebbi_2013_InKreSAT.pdf>.
22. LoTREC: <https://www.irit.fr/~Andreas.Herzig/P/ijcar01.html>, <https://link.springer.com/doi/10.1007/978-3-7643-8504-0>.
23. MetTeL2: <https://www.cs.man.ac.uk/~schmidt/publications/TishkovskySchmidtKhodadadi12b.html>.
24. LWB: <https://home.inf.unibe.ch/ltg/publications/2000/bhs00.pdf>, <https://arxiv.org/pdf/2112.14605>.
25. Spartacus: <https://core.ac.uk/display/81946362>.
26. Modal Logic Playground: <https://rkirsling.github.io/modallogic/>, <https://github.com/rkirsling/modallogic>.
27. Carnap: <https://carnap.philosophy.ubc.ca/about>, <https://carnap.github.io/Carnap-Manual/systems.html>, <https://arxiv.org/pdf/1803.03092>.
28. Steen et al. on the TPTP's non-classical logics: <https://arxiv.org/html/2508.09318v1>, <https://tptp.org/Seminars/NonClassical>.
29. TPTP logic specification slides: <https://tptp.org/TPTPTParty/2024/SpeakersSlides/MelanieTaprogge.pdf>; LET tool <https://ar5iv.labs.arxiv.org/html/2203.12352>.
30. AFP entry by From (via search results, page not opened): <https://arxiv.org/pdf/2404.14919>.
31. Lean formalisations: <https://arxiv.org/pdf/1910.01697>, <https://link.springer.com/10.1007/978-3-030-89391-0_25>.
32. Steen, Sutcliffe, Benzmüller: <https://arxiv.org/html/2212.09570v3>.
33. See 1; resource reading as summarised at <https://episteme.tllm.fr/wiki/⅋>.
34. Fukuda, Yoshimizu: <https://arxiv.org/pdf/1904.10605>.
35. A general proof certification framework for modal logic: <https://arxiv.org/pdf/1810.10257> (title only).

## 7. Not verified

- Licence and last activity of nanoCoP-M, MOILab, MSPASS, LoTREC, MetTeL2, LWB, Spartacus; last commit dates of CEGARBox and the Playground; the licence of MOIN and InKreSAT (stated as not given on their pages).
- "Molle" and "KSP": no source found. Display calculi (Belnap, Wansing): no source found. CTL EXPTIME-completeness: not sourced.
- Full texts not read: Ladner, Negri (404), Brünnler's body, Avron, Halpern and Moses, Girard, Martini and Masini (abstract only); GL's PSPACE-completeness only through secondary papers; the AFP entry page itself.
- The IS4 decision procedure is a September 2026 preprint, abstract only.
- Whether a modal tab fits the client's stage was not assessed (the client's repository was not read for this note).
- Cost figures are estimates (inference); no program was run. I would have run nothing.
