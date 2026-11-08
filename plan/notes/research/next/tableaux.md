# Tableaux: analytic tableaux, truth trees and the connection method as a suite beside linlog's sequent calculi

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

## 1. What linlog has today

- **No tableau method.** A grep (case-insensitive) for "tableau" over `core/src` finds only the simplex tableau of the Horn engine's state equation (`core/src/search/horn/equation.rs`, a linear-programming table, unrelated) and a test of a serde error for `{"kind":"tableau"}` in `core/src/serialize/search.rs`. `docs/src/` has no match. In the plan, tableaux appear only as the other prover linTAP in the comparison (`bench/TOOLS.md`, `plan/notes/research/29-comparison.md`, `plan/notes/research/42-first-order.md`) and as TABLEAUX-conference citations. `plan/later.md` (around line 610) says the classical-into-affine-MALL embedding costs "one branch per way through the conjunctions, as a tableau's".
- **Sequent calculi and their read-backs.** Proof terms and a checker for every linear mode; a derivation view that unfolds a term into the one- or two-sided sequent calculus; interactive step-by-step proving with open goals; export to LaTeX (ebproof), Typst (curryst), SVG, PNG, PDF; Rocq certificates (`docs/src/status.md`).
- **Ordinary logic.** Classical, intuitionistic and minimal propositional logic are decided through embeddings into affine MALL or ILL, and the linear proof is read back as LK or LJ with a checker of its own (`core/src/ordinary/`, `.claude/rules/core-ordinary.md`). The ordinary syntax is propositional only. A formula that is not valid gets `NotValid`; the linear side of a refutation gives a falsifying assignment looked for within a bound, the counts, or the saturated set (`docs/src/status.md`, "Automatic proof search").
- **Refutations.** `Disproof::check` verifies the falsifying-assignment, balance and count-equation refutations without the search; the Rocq library proves them sound (`status.md`, "Certified refutations"). No countermodel exists for any other logic ("Planned": "countermodels of the other logics").
- **Terms and first order.** `core/src/sequents/term.rs` holds `Term`, a 12-byte node in negation normal form with `Forall(TermId)` and `Exists(TermId)` as locally nameless binders, and the first-order engines unify with an occurs check and eigenvariable levels (`status.md`, "First-order proof search"). The ordinary layer has no quantifiers.
- **What this touches.** A closed analytic tableau is a sequent proof turned upside down (see section 2). So the LK read-back of `core/src/ordinary/derivation.rs`, the derivation view and the interactive proof state (a goal with open premises, a rule applied bottom-up to a chosen formula) are already the data of a one-sided tableau: a goal is a branch, its sequent is the set of formulas on the branch, applying a rule to a formula is expanding it. This is an inference from the sources of section 2 and from the status chapter, not something the code states.

Nothing below re-proposes those.

## 2. The field

### 2.1 The method

- **Origins.** Beth, Hintikka and Kanger developed semantic tableaux independently; Smullyan simplified them to one-sided signed tableaux for classical logic; Dodgson's truth trees in *Symbolic Logic II* are an early form; "tableaux can be seen as sequent systems built upside down" (Carnielli 1991) ([1], Wikipedia, which cites Beth 1955, Smullyan 1968, Carnielli 1991).
- **Standard textbooks.** Smullyan, *First-Order Logic* (Dover); Fitting, *First-Order Logic and Automated Theorem Proving*, 2nd ed. (Springer, 1996), whose contents list free-variable semantic tableaux (chapter 7) and a free-variable tableau system with equality (chapter 9) ([2], a library record, so a table of contents and no more); Howson, *Logic with Trees* (Routledge) and Girle, *Modal Logics and Philosophy* for teaching; the *Handbook of Tableau Methods* (D'Agostino, Gabbay, Hähnle, Posegga, Kluwer 1999), with Goré's chapter "Tableau Methods for Modal and Temporal Logics" ([1], bibliography list only; I read none of these books).
- **Signed and unsigned.** Open Logic Project's tableaux chapter uses prefixed (signed) tableaux, "which label true and false explicitly", because that makes comparison with the sequent calculus easier and extends to many-valued logics; Zach says the method "works very nicely and uniformly" for modal logics, which often lack a convenient natural deduction ([3]).
- **First order, free variables.** In free-variable tableaux the γ rule introduces a fresh variable instead of a ground instance and branches are closed by a most general unifier; the variables are rigid, so a substitution changes the whole tableau; delayed instantiation is a remedy for the resulting branch-closing problems ([1]). This is the computational core of leanTAP and of connection provers.
- **Modal.** World-labelled, set-labelled and formula-deleting tableaux, plus global assumptions ([1]). Prefixed modal tableaux are what Open Logic teaches ([3], [4]). The Tree Proof Generator draws modal trees for K with the accessibility relation constrained as universal (S5), reflexive, symmetric, transitive, euclidean, serial, and, for quantified modal formulas, constant domains and rigid terms ([5]).
- **Intuitionistic.** Fitting's 1969 *Intuitionistic Logic, Model Theory and Forcing* (North-Holland) is the standard source; I could confirm only its bibliographic record, not that it presents prefixed tableaux (unsourced). ileanCoP is "an extension of leanCoP for first-order intuitionistic logic" ([6]); nanoCoP-i is its non-clausal successor ([7]).
- **Connection method.** Clause tableaux restricted by connectedness give complete first-order calculi ([1]). leanCoP is a compact Prolog prover for classical first-order logic with equality based on the connection calculus, GPL, versions 1.0 to 2.1, page dated 2024-12-12; ileanCoP and MleanCoP (v1.3, "several first-order multimodal logics") are its extensions ([6]). nanoCoP works on the non-clausal form without translation to clauses ([7]).
- **Linear logic.** linTAP is "a tableau prover for the multiplicative and exponential fragment" that checks classical validity with an analytic tableau and linear validity by prefix unification; sound and complete; Prolog for ECLiPSe, SWI and SICStus; no licence stated; page dated 2008-03-03 ([8]). linlog already runs it in the comparison (`bench/TOOLS.md`: 15 problems solved, none on LLTP in 29-comparison), and the plan's note on first order says it "leaves the additive fragment and the quantifiers out of consideration" (`plan/notes/research/42-first-order.md`, citing Mantel and Otten 1999). Otten's group also has linear connection methods (Kreitz, Mantel, Otten, Schmitt 1997, `42-first-order.md`; not read by me).
- **Resource tableaux.** Galmiche, Méry and Pym give resource tableaux for BI (`plan/notes/research/scope/logics.md`, [GMP05]; not read by me).

### 2.2 Complexity

- Deciding classical propositional validity is co-NP-complete, so any complete tableau method is exponential in the worst case (standard; I did not source it from a page read today, so treat it as textbook knowledge, unsourced).
- Arai, Pitassi and Urquhart, "The complexity of analytic tableaux", JSL 71(3), 2006: for three variants (clausal, generalized clausal, binary tableaux) they compare the variants and tableaux with resolution; the main result is a quasi-polynomial simulation of tree resolution by analytic tableaux with a lower bound tight up to a polynomial factor ([9], abstract only).
- A search result states that D'Agostino's work shows a class of problems harder for tableaux than for truth tables; I did not read it (unsourced beyond a search summary).
- First order is semi-decidable, modal K is PSPACE-complete (standard; unsourced here). Linear-logic fragments' complexities are already in `plan/notes/research/42-first-order.md`.

### 2.3 Tools

| tool | what | licence | last activity | how used |
|---|---|---|---|---|
| Tree Proof Generator, [5] [10] | Wolfgang Schwarz's browser tableau prover for classical propositional, first-order (with functions and identity) and normal modal logic; finds a countermodel or a tree proof; searches a free-variable tableau and translates it back into a sentence tableau (the help page of a mirror, [11]) | GPL-3.0 ([10]) | most recent commit 2026-09-11, "speed up modelfinder", 252 commits ([12]) | the page invites bug reports; use in courses is not stated on the page (unsourced) |
| Carnap truth trees, [13] | `TruthTree` exercises: students right-click a line and choose "continue branch with 2 formulas", "split branch", "mark as resolved", "mark branch as contradiction", and enter row, rule and the contradicting pair of lines; default system Ichikawa-Jenkins SL; `checkIchikawaJenkinsQLTableau` for QL | the licence of Carnap was not read | the manual page ([13]); used in logic courses with automatic grading (Zach's post, [14], title only) | a *checker* of student trees, not a prover |
| LoTREC, [15] [16] | generic tableau prover for modal and description logics; the user writes a tableau system in a graph-rewriting language; builds models or countermodels; companion to *Kripke's Worlds* (Gasquet, Herzig, Said, Schwarzentruber, Birkhäuser 2014); TABLEAUX 2005 | not stated on the page | download page dated 2019-02-07, LoTREC 2.0 | teaching modal logic |
| MetTeL2, [17] | Tishkovsky, Schmidt, Khodadadi 2012: from a tableau calculus specification (no connective predefined) generates a Java prover that outputs models for satisfiable input, with dynamic backtracking and backjumping; also a web interface | not read | 2012 papers; the site was not reachable in this session | research on new calculi |
| Spartacus, [18] | Götzmann, Kaminski, Smolka: tableau prover for hybrid logic with pattern-based blocking; Standard ML, MLton; online demo | not read | ENTCS 2010 | research |
| leanTAP, [19] | Beckert and Posegga, J. Autom. Reasoning 15(3), 1995: a complete and sound first-order prover from free-variable tableaux in a few lines of Prolog | not read | 1995 | the standard teaching example of lean provers |
| leanCoP family, [6] | see 2.1 | GPL (leanCoP) | page 2024-12-12 | CASC entries (leanCoP 2.0 best newcomer at CASC-21; third in proof class at CASC-22) |
| prooftrees, [20] | LaTeX package for Forest-based proof trees in intro logic courses: line numbers, justifications, branch-closure explanations, cross-references; built on forest and TikZ | LPPL 1.3c | version 0.9.3, 2026-02-21 | Open Logic's tableaux ([3]) |
| Jape, [21] | configurable proof editor for natural deduction and sequent calculus; no tableau support found | GPL-2.0 | version 9.1.8 (2023-10-10) per Wikipedia | teaching |
| KSP | a resolution prover for multimodal K, not a tableau tool (as a search summary of the MaRDI listing states) | n/a | n/a | not a candidate |

## 3. Candidates

### tableaux-1: One-sided classical propositional tableaux as a view of the existing derivation, with countermodels

- *What it brings and to whom.* Teaching: truth trees are the standard exercise of philosophy logic courses (Carnap has them as a graded exercise type, [13]; Open Logic teaches prefixed tableaux, [3]; the Tree Proof Generator is the free tool, [5]). The payoff is that an open branch is a countermodel. linlog today shows a falsifying assignment for the refuted ordinary formula but no tree of how the search found it. For the AI guide `skills/linlog/`, a tree is a readable explanation of "not valid".
- *What a suite would offer.* **Decide**: nothing new; the verdict stays the ordinary layer's. **Prove step by step**: the interactive state already has goals with a set of formulas, so a tableau is a different drawing and different rule names (α, β, closing by a complementary pair of literals) of the same LK derivation read-back; the student's moves are the invertible rules applied to a formula of the branch, which `interact` validates today. **Check**: a closed tableau is checkable by a one-pass checker (each node is a subformula of its parent, each branch holds a complementary pair); for a tableau read from the LK derivation the existing `Derivation::check` is the checker (inference). **Export**: a tree in LaTeX (prooftrees, [20]; or forest), Typst, SVG. **Certify**: the existing certificate over `Prop` already proves validity; the tableau is a presentation of the same proof. **Countermodel**: the open branch of a saturated tableau is a valuation (Smullyan; [1]); it can be checked by evaluation, which `Disproof::check` already does for assignments.
- *Cost.* Two to three sessions: a tableau data type over the `ordinary::Formulas` arena (hash-consed, so a tree shares nodes), a conversion from the LK derivation (inference: one-sided Gentzen-Schütte form, the rule shapes are the same), a text and an SVG or `prooftrees` export, a `linlog seq tableau`-style output and an interactive mode. Trusted base: a small checker for tableaux, or none if the tree is always converted back to the LK derivation and that is checked.
- *Risk.* Duplication: two views of one proof that students will not distinguish; the real gain is notation (signed `T`/`F` against the sequent's sides) and the countermodel. Tableau expansion order is not the search order of the linear engine, so a tree "found by linlog" is a re-presentation, not what the engine did.
- *Prerequisites.* None; the ordinary layer and the derivation view exist. A choice between signed and unsigned notation (Open Logic uses signed, [3]).
- *Sources.* [1] [3] [5] [13] [20], `core/src/ordinary/`.

### tableaux-2: A tableau checker for student trees (the Carnap model)

- *What it brings and to whom.* Teaching, where the student builds the whole tree, not a sequent proof: Carnap's interface checks each step by row, rule and the contradicting pair ([13]). linlog's `interact` already validates rule applications; a tableau mode would let a lecturer reuse an Open Logic or Howson exercise.
- *What a suite would offer.* A JSON form of a tableau (nodes with a formula, a parent, a rule and its row number; closure marks) for the web page; a `check` of it with a message naming the faulty node; a hint ("this branch is open, expand row 4") from the engine. Step by step: yes, this is the point.
- *Cost.* Two sessions after tableaux-1, one more for the web island if the web page wants it (outside this note's scope; the web area decides). Trusted base: the tableau checker if its verdicts are shown as "correct".
- *Risk.* A grading tool competes with Carnap's established one; the audience is a niche of the page's learners. Rule sets vary by textbook (Ichikawa-Jenkins versus Smullyan versus Open Logic), so the checker needs a rule-set parameter, as Carnap's `checkFunction` has ([13]).
- *Prerequisites.* tableaux-1's data type.
- *Sources.* [13] [3] [14].

### tableaux-3: First-order tableaux with free variables and unification

- *What it brings and to whom.* Research and teaching of first-order classical logic through the same machinery as the linear first-order engines: the Tree Proof Generator does predicate logic with identity ([5]), leanTAP is its ten-line ancestor ([19]), and linlog already has unification with an occurs check and eigenvariable levels for linear first-order (`status.md`).
- *What a suite would offer.* Ordinary first-order classical logic (a new `Logic` over quantified formulas) decided by free-variable tableaux, or by embedding into first-order affine MALL, whose engines exist (inference: the propositional embedding is in `core/src/ordinary/translate.rs`; first-order is not there today, and the embedding of quantifiers into `∀`/`∃` of MALL1 would need checking against the first-order engine's modes). Check: a free-variable tableau is checkable by applying the substitution and checking ground closure; **semi-decidable** only, so "unknown" at a bound.
- *Cost.* Three to five sessions: quantified ordinary syntax and TPTP `fof` with quantifiers, the translation, the read-back to LK with quantifier rules, a first-order Rocq certificate (the library's first-order development exists, `status.md`). Trusted base grows by the quantifier rules' checker.
- *Risk.* Competing with provers that are far stronger (the leanCoP family at CASC, [6]); the value is the integration and the certified read-back, not the speed. Free-variable tableaux are destructive and may need backtracking over instantiations; linlog's search engines are built around linear sequents.
- *Prerequisites.* tableaux-1; first-order ordinary syntax, which does not exist.
- *Sources.* [1] [2] [5] [19] [6], `status.md`.

### tableaux-4: Normal modal logic (K, T, S4, S5 and friends) by prefixed tableaux

- *What it brings and to whom.* Teaching and research: modal logic is a large teaching audience (Open Logic added prefixed modal tableaux because modal logics lack convenient natural deduction, [3] [4]; *Kripke's Worlds* teaches modal logic via tableaux with LoTREC, [15]). The Tree Proof Generator does K with frame conditions ([5]). Countermodels are Kripke models read off an open branch.
- *What a suite would offer.* A new object, not a view: modal formulas (`□`, `◇`), prefixes (world labels), frame conditions as parameters, a decision procedure (termination needs loop checks for K4/S4, which blocking provides, as Spartacus does for hybrid logic, [18]), a checker for tableaux with prefixes, a Kripke countermodel with a check by evaluation; export of the tree and of the countermodel graph; interactive proving.
- *Cost.* Four to six sessions; `Term` needs modal nodes (the scope note `scope/logics.md` already reasons about modal variants of `Term` carrying payloads) or a separate formula arena as `ordinary` has. Trusted base: a new checker and a new Rocq development for the modal calculus (soundness of the tableau rules against Kripke semantics).
- *Risk.* A different logic from linear logic: linlog's name and promise are linear logic (README), and the linear modalities `!` and `?` are not S4's `□`/`◇` in the sense students need (the classical embedding of S4 into linear logic exists in the literature but I did not source it). The area is served by free tools, LoTREC and the Tree Proof Generator ([5] [15]).
- *Prerequisites.* tableaux-1; a modal syntax.
- *Sources.* [3] [4] [5] [15] [18].

### tableaux-5: A tableau or connection prover for linear logic with prefixes, as an engine beside linTAP

- *What it brings and to whom.* Research: linTAP decides propositional multiplicative-exponential linear logic by a classical tableau plus prefix unification and ignores additives and quantifiers ([8], `42-first-order.md`). linlog's engines already beat or match it in the comparison (`bench/TOOLS.md`).
- *What a suite would offer.* A new engine, for diversity only; nothing a user cannot do with the focus, net and inverse engines.
- *Cost.* Three or more sessions plus a prefix unifier.
- *Risk.* No evidence of use: linTAP's page is dated 2008 and no problem library was solved by it (`bench/TOOLS.md`).
- *Prerequisites.* None.
- *Sources.* [8].

## 4. Considered and rejected

- **tableaux-5 (a linear tableau engine).** Rejected: the comparison has linTAP at the bottom of the problems solved and no user asks for another engine; the net engine covers the same multiplicative fragment with a correctness criterion.
- **Description-logic tableaux.** Rejected: their audience is ontology reasoners (LoTREC and MetTeL2 serve research, [15] [17]), far from linlog's logics and from its learners. I read no source that shows a demand.
- **A tableau prover generator (MetTeL2-style).** Rejected: a different product, and linlog's specific advantage, a checked linear kernel, does not carry over ([17]).
- **A connection-method prover for classical logic.** Rejected: leanCoP, nanoCoP and their intuitionistic and modal variants exist, are small and maintained ([6] [7]); the proof of a classical formula by linlog is already certified.
- **Many-valued tableaux.** Rejected: no source of demand read; outside the logics linlog states.

## 5. What the structure must allow

- **Reusable.** The ordinary layer's hash-consed `Formulas` arena and its LK read-back and checker (`core-ordinary.md`); the derivation and interactive machinery (`core-derivations.md`), whose goals and rule applications are what a tableau step is; the export crates (LaTeX ebproof, Typst, SVG) as templates for a tree drawer; the term arena and unification of the first-order engines for free-variable tableaux; the Rocq library's pattern (term, checker as a function, soundness theorem) for certifying a tableau.
- **Linear-specific, not reusable.** The focused engine, the counts, the nets, the Horn engine: none apply to a tableau for ordinary logic. A tableau's branch is a set where linear sequents are multisets; its closing rule is an axiom with weakening (affine-like); modal prefixes and frame conditions have no counterpart in the linear core.
- **The data-model question.** A tableau is a tree whose nodes are formulas, so it fits an arena of nodes with parent indices; whether it belongs beside `Derivation` (as another view of it) or as its own proof object is the decision that sets the cost of tableaux-2 to tableaux-4.

## 6. Sources

All read 2026-11-08 unless noted.

1. Wikipedia, Method of analytic tableaux. https://en.wikipedia.org/wiki/Method_of_analytic_tableaux (bibliography as listed there)
2. Library records of Fitting, *First-Order Logic and Automated Theorem Proving* (contents), via search: https://library.holycross.edu/Record/b3192539/TOC
3. Richard Zach, Open Logic Project tableaux, blog post of 2018-02. https://richardzach.org/2018/02/
4. Open Logic Project, modal logic and propositional tableaux. https://openlogicproject.org/2018/02/22/modal-logic-propositional-logic-tableaux/ (search result only)
5. Tree Proof Generator. https://www.umsu.de/trees/
6. leanCoP, ileanCoP, MleanCoP. http://www.leancop.de
7. Otten, nanoCoP: Natural Non-clausal Theorem Proving, IJCAI 2017. https://www.ijcai.org/proceedings/2017/695 (search result; nanoCoP-i and nanoCoP-M in https://link.springer.com/chapter/10.1007/978-3-030-86059-2_14)
8. linTAP page. http://www.leancop.de/lintap/
9. Arai, Pitassi, Urquhart, The complexity of analytic tableaux, JSL 71(3), 2006. https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/complexity-of-analytic-tableaux/AE435EF2B52D71655CDF3F26F19E1470
10. wo/tpg repository. https://github.com/wo/tpg
11. Tree Proof Generator help page (mirror), via search. https://www.cs.us.es/~jalonso/cursos/li-06/sistemas/tableros/info.html
12. wo/tpg commits. https://github.com/wo/tpg/commits/master/
13. Carnap manual, truth trees. https://carnap.github.io/Carnap-Manual/truth-tree.html
14. Zach, adding online exercises with Carnap (title in a search result only). https://richardzach.org/2020/01/adding-online-exercises-with-automated-grading-to-any-logic-course-with-carnap/
15. LoTREC page. http://www.irit.fr/Lotrec/ ; TABLEAUX 2005 record https://link.springer.com/doi/10.1007/11554554_25 (search result)
16. Gasquet, Herzig, Said, Schwarzentruber, *Kripke's Worlds*, Birkhäuser 2014, via search. https://link.springer.com/doi/10.1007/978-3-7643-8504-0
17. MetTeL2 papers. https://www.cs.man.ac.uk/~schmidt/publications/TishkovskySchmidtKhodadadi12b.html (page timed out; content from the search summary); PAAR-2012 DOI 10.29007/1c73
18. Spartacus. https://www.ps.uni-saarland.de/spartacus
19. Beckert, Posegga, leanTAP, JAR 15(3), 1995. https://dblp.uni-trier.de/rec/journals/jar/BeckertP95.html
20. prooftrees on CTAN. https://ctan.org/pkg/prooftrees
21. Jape. https://github.com/RBornat/jape
22. rocq-linlog. https://github.com/linlog-prover/rocq-linlog (what it does not certify: first-order refutations, ordered calculus)

## 7. Not verified

- No book was read (Smullyan, Fitting, Handbook of Tableau Methods, Goré's chapter): their content is as listed in Wikipedia's bibliography and library tables of contents.
- Fitting's intuitionistic tableaux: the 1969 book's existence only; whether it uses prefixes is not confirmed.
- The complexity of K (PSPACE) and of classical validity (co-NP) are textbook facts I did not source today; the D'Agostino results are a search summary.
- Licences of Carnap, LoTREC, MetTeL2, Spartacus, leanTAP, ileanCoP, MleanCoP, nanoCoP were not found; MetTeL2's own site did not load; the Tree Proof Generator's use in courses is not stated.
- No source for a translation of S4 into linear logic, no survey of tableaux in description logics, no check of KSP beyond a search summary.
- Whether the embedding of first-order classical logic into first-order affine MALL works with linlog's current first-order engines was not checked in code (`core/src/search`, first-order modes).
- One search result (a mirror of a Wikipedia article on cavac.at) contained an embedded instruction unrelated to the task; it was ignored.
- Nothing was run; I would have run nothing.
