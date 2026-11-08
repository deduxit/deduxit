# Research note: what else is worth adding (logics and methods beyond the author's list)

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

The note covers the logics and methods that the author's list (ordinary sequents and substructural variants, natural deduction, resolution, tableaux, modal logics, cyclic proofs, lambda calculi, custom rules) and the linear-logic scope notes (`plan/notes/research/scope/logics.md`) do not. Claims marked "(inference)" are this note's own conclusions; "(unsourced)" marks a claim with no source read.

## 1. What linlog has today

- Classical, intuitionistic and minimal propositional logic through embeddings into linear logic, a TPTP reader (`fof` and the ILTP library's `.p` files), derivations in LK and LJ, a Rocq certificate over `Prop`, and, for a non-valid sequent, a falsifying assignment or a countermodel lemma (`docs/src/status.md`, `docs/src/ordinary-logic.md`). The ordinary logic is propositional; ordinary first-order logic is planned after step 42 (`plan/notes/research/impact-fo-ordinary.md`, `fo-embeddings.md`).
- First-order linear logic in every classical and intuitionistic mode, with unification, eigenvariable levels, witnesses, proofs, certificates and cut elimination (`docs/src/first-order.md`). No equality, no function-symbol theory, no sorts.
- Randomly generated QBF as a benchmark family under a lock-and-key encoding into MALL (`core/src/families.rs`, family `qbf`; `bench/TARGETS.md`). There is no QBF reader, no QBF as an input logic.
- The truth-value reading: every unprovable classical sequent gets a falsifying assignment (`status.md`), so a truth-table view exists inside the refutations.
- Cyclic MLL and the Lambek calculus, nets, Petri-net decisions (coverability), the web page with Learn and Practice.
- Nothing for description, temporal, bunched, many-valued, epistemic or equational logic, for Hoare logic, or for SAT proofs.

## 2. The field

### 2.1 Where logic is taught and what the tools are

- **Carnap** is "a free and open software framework written in Haskell for teaching and studying formal logic", used at "dozens of colleges and universities"; its about page lists 31 institutions (incomplete) and names these systems: Kalish and Montague, Hardegree's variants, Bergmann and Moore's Logic Book, Goldfarb's Deductive Logic, forall x (QL, SL, and the Calgary remix FOL), Hardegree's modal logic (K5, KTB, MPL), polyadic second-order logic, elementary set theory, and propositional and first-order LK and LJ via ProofJS; exercises are truth tables, translation and parsing, deductions, with a JSON API. Repository: GPL-3.0, 2,469 commits, a truth-tree component (Rudolf). [1] [2] [3] Its footer reads copyright 2015 to 2026, so it is maintained (inference from [1]).
- **forall x: Calgary** (Thomas-Bolduc and Zach), a free textbook (CC BY 4.0): consequence, truth-functional logic, first-order logic with identity, Fitch natural deduction, modal logic as an advanced topic; latest print edition Fall 2025; it recommends Carnap for exercises and ndprover.org as a checker that also does modal proofs. [4] The page names no adopting courses.
- **Tree Proof Generator** (Wolfgang Schwarz): generates tableaux for classical propositional and predicate logic in JavaScript, by a free-variable tableau search converted to an ordinary sentence tableau; its own help page says it still does badly on some formulas. [5] Truth trees are the classical beginner's method in philosophy courses (inference, from [2] and [5] together).
- **Proof Tree Builder** (Korkut, Princeton): a web pedagogical assistant for sequent calculus and Hoare logic over a simple imperative language, with basic automation and Z3; students of an automated-reasoning course used it. [6]
- **Software Foundations, Programming Language Foundations, chapter Hoare**: Floyd-Hoare logic of Imp programs in Rocq, assertions as `state -> Prop`, triples, skip, sequencing, assignment and consequence rules, then conditionals and loops (the page's later part was not read). [7]
- **Hintikka's World** (Schwarzentruber and others): a pedagogical tool for epistemic modal logic, presented at IJCAI 2018 and 2019 and used in tutorials at IJCAI 2018, 2019 and AAMAS 2019, with muddy children and Russian cards. [8]
- **SMCDEL** (Gattinger): symbolic model checking of dynamic epistemic logic in Haskell, GPL-2.0, 249 commits, command line only for S5 with public announcements, K and richer updates as a library, BDD backends, a web interface; theory in Gattinger's 2018 PhD thesis; the repository's "Used by" is empty. [9] [10]

### 2.2 Automated reasoning and its infrastructure

- **TPTP** is "the core of the TPTP World, the infrastructure the ATP community uses"; CASC is its annual competition; SystemOnTPTP runs provers online. [11] The first-order mainstream there is classical, with equality (the page read does not list the sub-languages FOF, TFF, THF, so their counts are unsourced here).
- **ILTP** (Raths, Otten, Kreitz): a benchmark library for intuitionistic first-order and propositional provers, status Theorem, Non-Theorem, Unsolved or Open per problem, v1.0 2005, v1.1 2006, J. Autom. Reasoning 2007. [12] linlog already reads its propositional part; the first-order part has the same format (inference).
- **QBF**: TQBF is the canonical PSPACE-complete problem; solvers are QDPLL or CEGAR based (DepQBF, CAQE, CADET); QBFEVAL has run roughly yearly since 2004 on QDIMACS, QCIR or QAIGER; QBFEVAL'22 gave an award for partial certificates in QDIMACS 1.1. [13] [14] A 2025 edition could not be confirmed. Applications: planning, bounded model checking, synthesis, equivalence checking [13].
- **SAT proofs**: DRAT-trim (Heule, MIT licence) checks a DRAT clausal proof of a DIMACS CNF's unsatisfiability; each step adds or deletes a clause, additions checked in polynomial time, DRAT generalises extended resolution. [15] Teaching CDCL as such is not covered by anything read (unsourced).
- **Vampire** was to be the first-order prover example; its repository page could not be read (404), so nothing is claimed about it.

### 2.3 Other logics

- **Description logics.** ALC is the base language of many DLs; OWL 2 is based on SROIQ(D); tableau reasoners (FaCT++, RACER, Pellet, HermiT) implement analytic tableaux or hypertableaux; HermiT is open source (Oxford), Pellet AGPL or proprietary. [16] The complexity of ALC with a TBox is not stated on the page read (the Description Logic Complexity Navigator is the pointer), so it is unsourced here.
- **Temporal logics.** LTL model checking and satisfiability are PSPACE-complete, synthesis 2EXPTIME-complete; the standard method is a Buechi automaton for the model and for the negated property. [17] This is verification of a model, not a derivation (inference).
- **Bunched implications.** BI is undecidable (Galatos, Jipsen, Knudstorp, Ramanayake, arXiv 2603.01595, March 2026), and the result extends to GBI and Boolean BI, with gaps pointed out in the earlier decidability claims. [18] Separation logic's practical tools are symbolic-heap checkers; Infer's page read says only that Pulse "replaces the original biabduction analysis", for C, C++, Objective-C, Hack and Java, Erlang and Rust experimental. [19] The scope note already holds that BI is rejected as a decided logic (`scope/logics.md`, section 3).
- **Many-valued and paraconsistent.** Finite-valued logics are decidable by enumeration; Lukasiewicz's infinite-valued logic and Goedel logic are coNP-complete, with fusion max(x+y-1,0) and implication min(1-x+y,1); Lukasiewicz logic is one of the main fuzzy logics. [20] FDE has four-valued relational semantics (Dunn), is a fragment of relevant logics; LP is Priest's three-valued logic, invalidating explosion; the SEP article says nothing of tableaux or decidability. [21]
- **Equational logic and rewriting.** Knuth-Bendix completion is a semi-decision procedure; KBCV (LGPL-3) is an interactive completion visualiser that can record equational proof trees; CSI proves confluence, TTT2 termination, and CeTA certifies the tools' proofs. [22] Which courses use them was not found.

## 3. Candidates

### more-logics-1: Classical first-order logic with equality (a `fof` mode)

*What it brings and to whom.* It is the TPTP mainstream: the CASC competition and the problem library are built on it [11], and the textbooks and teaching systems read (forall x with identity, Carnap's QL and FOL, Tree Proof Generator) are classical first-order [2] [4] [5]. A student or an AI model that meets a `fof` file with `=` gets "unsupported" today (inference: linlog has no equality, section 1). Ordinary first-order logic without equality is already planned; equality is the missing half.

*What a suite would offer.* Decide (semi-decide: first-order validity is only semi-decidable, so "unknown" at a bound is a normal answer), check and prove step by step in LK with `=` rules, export, and certify through Rocq (`=` is native there). Equality enters either as axioms in the linear image (reflexivity, symmetry, transitivity, congruence per symbol: sound and checkable, but a poor search, inference) or as a rule with unification modulo the rules (paramodulation-like, a new engine). The honest scope is a teaching and checking suite, not a competitor of Vampire or E (inference, since those are built for CASC [11]).

*Cost.* Two to three sessions for the axiom route (reader, checker rule, derivation, export, certificate); four to six for a native equality rule in the focused engine. The trusted base grows by one checker rule.

*Risk.* The axiom route explodes on congruence instances and times out on the problems that teach equality (inference). Sorts and arithmetic (TFF) are out of scope.

*Prerequisites.* The ordinary first-order step (planned) and step 42's unification.

*Sources.* [1] [2] [4] [5] [11].

*Verdict:* worth a mode of another suite (the ordinary first-order suite) (inference).

### more-logics-2: Truth tables and semantic tableaux (truth trees) as the beginner's entry

*What it brings and to whom.* Truth tables and truth trees are what logic courses teach first: Carnap's exercise types begin with truth tables and the repository has a truth-tree component [1] [2] [3]; forall x begins with truth-functional logic and defers proofs [4]; Tree Proof Generator exists for the tree method [5]. The tableau method itself belongs to the author's tableaux item, covered by another agent; what this note adds is the beginner's use. linlog already computes falsifying assignments (section 1), so a truth-table view of any propositional sequent, with the rows that falsify it marked, costs almost nothing and gives the Learn lessons an entry before sequents (inference).

*What a suite would offer.* Decide (exhaustive over atoms, bounded by 2^n, with a limit), check (a table is checked by evaluation), prove step by step (the student fills a row or picks the next rule of a tree), export (LaTeX, Typst, SVG as for derivations), no certificate needed beyond the existing Rocq `tauto` lemma. Tableau trees: checked, steppable, with an open branch read as a countermodel.

*Cost.* One session for tables and the reading of countermodels, two for trees as a proof method with a closed-tree checker; trusted base: the evaluator and the tree checker, both small.

*Risk.* Teaching value depends on matching the notation of the chosen textbooks (Carnap and forall x use their own symbols); that is a syntax option, not a logic change (inference).

*Prerequisites.* None for propositional; first-order trees need the first-order ordinary step.

*Sources.* [1] [2] [3] [4] [5].

*Verdict:* worth a mode of another suite (tableaux or ordinary logic) (inference).

### more-logics-3: Quantified Boolean formulas

*What it brings and to whom.* TQBF is PSPACE-complete, QBF solvers are an active research area with a yearly competition and a certificate award [13] [14]. linlog has already a QBF family whose encoding into MALL shows linear logic's PSPACE-hardness result in practice; its generator makes random 3-QBF and the focused engine's target set times it (`bench/TARGETS.md`). What is missing is an input format: a QDIMACS reader would let linlog read the community's instances [13] and answer them as MALL sequents. Users: researchers who compare provers on QBF (the audience of the comparison in `bench/TOOLS.md`); teachers of complexity (inference).

*What a suite would offer.* Decide through the existing encoding (competing with DepQBF or CAQE is not the goal: those are dedicated QDPLL and CEGAR solvers [13], inference), check a proof (a QBF proof in the MALL image is already checked by linlog's checker; Skolem-function or Herbrand certificates are the QBF community's own form, QDIMACS 1.1 partial certificates [14]), steps in the interactive prover (the game reading, existential player chooses, universal player attacks, is a natural teaching view, inference).

*Cost.* One session for the reader, the encoding on input and the game view; certificates in the community's format are a further session. Trusted base: the encoding (already tested by verdicts of the family).

*Risk.* Instances of the competition are large; the encoding's blowup (the family's 20-variable instances take 80 to 113 seconds in `bench/TARGETS.md`) means linlog solves only toy instances there.

*Prerequisites.* None.

*Sources.* [13] [14].

*Verdict:* worth a mode of another suite (an input format of the linear suite) (inference).

### more-logics-4: Description logic ALC

*What it brings and to whom.* ALC is the base of OWL's logics [16], so a semantic-web or knowledge-representation course meets it first; the textbook method is the tableau [16]. Whether linlog's own engines suit it is a stretch: ALC's quantifiers are modal operators over roles (inference), so it overlaps the modal-logics item (K with several modalities, a TBox as global assumptions). Evidence of course use beyond [16] was not found.

*What a suite would offer.* Satisfiability and subsumption by tableau, a derivation as a completion tree, a countermodel as an interpretation drawn as a graph. Not checkable by the linear kernel; it needs its own checker.

*Cost.* Three sessions as a stand-alone suite; about one if modal logic K with multiple modalities is built generically (inference).

*Risk.* The TBox case's complexity is not sourced here; blocking and termination are real work. Competes with Protege and HermiT, which students already use [16].

*Prerequisites.* A generic tableau kernel with modal rules (the author's tableaux and modal items).

*Sources.* [16].

*Verdict:* worth a mode of another suite (the modal one), not a suite of its own (inference).

### more-logics-5: Hoare logic and program logics

*What it brings and to whom.* Hoare logic is taught in programming-language and verification courses: Software Foundations has it as a chapter [7], and a purpose-built web tool exists, the Proof Tree Builder, used in a Princeton course [6]. That a research tool of this kind was written for one course is evidence of the demand and of the gap in tools (inference).

*What a suite would offer.* Check a Hoare derivation of a small imperative language (rules of [7]: skip, sequence, assignment, consequence, then conditional and loop), prove step by step with weakest preconditions computed, verification conditions handed to the first-order engine, export the tree. The side conditions (arithmetic) need a decision procedure; linlog has none for integers (inference), so the assertion logic would be uninterpreted predicates or a small fragment, while [6] uses Z3.

*Cost.* Three sessions with a toy language and assertions in first-order logic; trusted base: the rule checker and the substitution of assignments.

*Risk.* Arithmetic is the whole difficulty; without it, the tool teaches the rules but proves few real programs. Separation logic is out of reach (undecidable core [18], inference for the extension).

*Prerequisites.* Ordinary first-order logic, custom rules (the author's item: Hoare rules are a user rule set).

*Sources.* [6] [7] [18].

*Verdict:* worth a mode of another suite (custom rules, over first-order ordinary logic) (inference).

### more-logics-6: Intuitionistic first-order logic with Kripke countermodels

*What it brings and to whom.* ILTP covers first-order intuitionistic problems with a status per problem [12]; the propositional half is already read. The Kripke countermodel is the intuitionistic analogue of the falsifying assignment and is what a student needs to see why `~~a -> a` fails (inference). linlog's first-order two-sided engine already decides intuitionistic first-order linear sequents; the ordinary intuitionistic first-order logic is a planned reading on top.

*What a suite would offer.* Decide (semi), prove in LJ with quantifier rules, refute by a drawn Kripke model for the propositional part (the model comes out of the exhaustive failed search, an extraction no engine does today, inference), export, certify in Rocq with the intuitionistic `Prop`.

*Cost.* One session beyond ordinary first-order for the first-order ILTP reader and refutations; two more for Kripke model extraction. Trusted base: a model checker that evaluates the forcing relation, small.

*Risk.* Model extraction from the inverse method's saturation is new research-grade engineering; first-order countermodels may be infinite.

*Prerequisites.* Ordinary first-order (planned).

*Sources.* [12].

*Verdict:* worth a mode of another suite (ordinary logic) (inference).

### more-logics-7: SAT proofs, resolution refutations and DRAT

*What it brings and to whom.* DRAT is the format by which SAT solvers' UNSAT answers are checked [15]. Resolution is on the author's list (another agent); the addition here is a checker for clausal proofs and a small CDCL that emits them, as the teaching device for conflict-driven learning. No source read shows a course or a tool for teaching CDCL (unsourced), so the evidence of use is only that DRAT-trim is the community's checker [15].

*What a suite would offer.* Check a DRAT proof (a small RUP checker is easy, full RAT with deletions is more), decide small CNF by a CDCL with proof output, step through decisions and conflicts. The linear link is thin: linlog already refutes classical sequents through MALL and a clausal proof would be a different object (inference).

*Cost.* Two sessions for a RUP/DRAT checker and a toy CDCL; a Rust SAT library exists as an option but was not examined here. Trusted base: the DRAT checker.

*Risk.* It competes with industrial tools (DRAT-trim and verified checkers) on their own ground [15]; only teaching justifies it.

*Prerequisites.* The resolution item (clause data model).

*Sources.* [15].

*Verdict:* worth a mode of another suite (resolution) (inference).

### more-logics-8: Equational logic, Boolean algebra and term rewriting

*What it brings and to whom.* Completion, confluence and termination are the theory of equational reasoning; KBCV is the teaching tool, CSI and TTT2 the research tools, CeTA certifies their answers [22]. Equational proofs (chains of rewrites, `a = b` by symmetric and congruence closure) are also how algebra courses prove identities (inference). A suite would offer a rewriting stepper and a Knuth-Bendix completion with a checked equational proof tree. The overlap with linlog's kernel is nil: no sequents, no linear resources (inference), except that the equality of more-logics-1 shares the term data model.

*Cost.* Three to four sessions (term data model, orders, completion, proof tree checker). Trusted base: a rewriting checker, small.

*Risk.* Completion may not terminate; termination proofs need their own machinery [22]. KBCV and CeTA already serve this audience.

*Prerequisites.* First-order terms (step 42 has them).

*Sources.* [22].

*Verdict:* not for this project at present; revisit with more-logics-1 (inference).

### more-logics-9: Temporal logics (LTL, CTL)

*What it brings and to whom.* The standard problems, satisfiability and model checking, are PSPACE-complete for LTL [17], solved by automata, not by deriving sequents. A proof suite would need either an LTL tableau with a derivation (loops closed as in cyclic proofs, which links to the cyclic-proof item and to `Cyclic MLL`, inference) or a model checker, which is a different product (inference). Evidence of course use is real (verification courses) but no source on it was read here (unsourced).

*What a suite would offer.* Satisfiability by tableau with the open branch read as a lasso countermodel; no certificate in the linear kernel. Steppable: yes, as a tableau.

*Cost.* Three sessions for LTL tableau; CTL double.

*Risk.* Duplicating NuSMV and SPIN (not read, unsourced); termination via eventuality checks.

*Prerequisites.* Tableau kernel with loop detection.

*Sources.* [17].

*Verdict:* worth a mode of another suite (tableaux, with the modal logics), the model checker not for this project (inference).

### more-logics-10: Many-valued and paraconsistent logics (Lukasiewicz, Goedel, FDE, LP)

*What it brings and to whom.* Philosophy and fuzzy-logic courses; the decision problem of the infinite-valued logics is coNP-complete, the same as classical [20]. Lukasiewicz logic is also the logic where linear logic's connectives appear (fusion and its dual, the SEP page's fusion and sum), so a suite would cost little at the connective level (inference: the SEP page does not mention linear logic in the part read). FDE and LP are four- and three-valued, truth-table based [21]. Tableau or sequent calculi for them were not found in the sources read (unsourced).

*What a suite would offer.* Decide by valuation search (finite-valued), a many-valued table as an exercise; for Lukasiewicz, a decision through its piecewise-linear semantics would need a linear-programming style solver (inference, not sourced). Check: evaluating a valuation. Prove: none beyond tables unless a calculus is chosen.

*Cost.* One session for finite-valued truth tables with designated values (FDE, LP, K3, Lukasiewicz L3); more for L-infinity.

*Risk.* No agreed teaching proof system, so only the semantics can be taught well.

*Prerequisites.* The truth-table mode of more-logics-2.

*Sources.* [20] [21].

*Verdict:* worth a mode of another suite (the truth-table mode) for the finite-valued case; not for this project beyond that (inference).

### more-logics-11: Epistemic, dynamic epistemic, deontic and conditional logics

*What it brings and to whom.* Hintikka's World is a teaching and tutorial tool at IJCAI and AAMAS [8]; SMCDEL is the research model checker [9] [10]. Basic epistemic logic (S5, K with agents) is a modal logic and falls under the modal-logics agent. Public announcement and muddy-children style updates are model-transforming, a model checking task, not a derivation (inference). Deontic and conditional logics: no tools or courses were read (unsourced).

*What a suite would offer.* A Kripke model editor and evaluator with announcements, drawn as SVG; no proof system.

*Cost.* Two sessions for models and announcements.

*Risk.* The existing tools [8] [9] cover it; the product is not a proof suite.

*Prerequisites.* The modal suite's Kripke model drawing.

*Sources.* [8] [9] [10].

*Verdict:* not for this project beyond modal logic with several agents (inference).

### more-logics-12: Bunched implications and separation logic

*What it brings and to whom.* It shares linear logic's resource reading (a bunch separates, as `⊗` does), and the verification tools use it [19]. But BI is undecidable [18], as the scope note recorded, so a suite could be a checker and a bounded searcher only.

*What a suite would offer.* A checker of BI sequent proofs (bunches as trees) and an interactive prover; no decision. A symbolic-heap entailment checker is another product.

*Cost.* Three sessions; the bunch-shaped context contradicts the multiset zones of linlog's core (`scope/logics.md`, section 3, inference).

*Risk.* Undecidable, as sourced; nobody asking.

*Prerequisites.* A bunch data model.

*Sources.* [18] [19].

*Verdict:* not for this project (inference), as `scope/logics.md` concluded.

### more-logics-13: Dependent type theory (Martin-Loef)

*What it brings and to whom.* Rocq, Lean and Agda own this; Software Foundations teaches through Rocq [7]. A suite would duplicate a proof assistant with a smaller trusted base at best, and checking type theory with universes and inductive types is a project of its own (inference). The simply typed lambda calculus and System F are the author's own items, covered elsewhere.

*Verdict:* not for this project (inference).

### more-logics-14: Proof-theoretic string diagrams and categorical tools

*Verdict:* not for this project (inference): the task said to skip unless a proof suite fits, and no source read describes a proof checker of that kind (unsourced).

## 4. Considered and rejected

- **A temporal model checker** (more-logics-9, the checking half): different product, automata not derivations [17].
- **Separation logic provers** (more-logics-12): undecidable core [18], different engineering world.
- **Martin-Loef type theory** (more-logics-13): the proof assistants own it.
- **Deontic and conditional logics**: no tool or course evidence read.
- **Equational completion as a suite** (more-logics-8): well served by KBCV, CSI, TTT2 and CeTA [22]; revisit with equality.
- **A competitive first-order prover** (more-logics-1): CASC's tools do that [11].

## 5. What the structure must allow

Reusable from linlog's core (judged from `CLAUDE.md`'s description of the modules; the rules files `.claude/rules/core*.md` were not read for this note): the arena of subformula occurrences, the parser and printer with the two spellings, the term and unification layer of step 42, the derivation and interactive layers, the exports (LaTeX, Typst, SVG), the Rocq certificate path, the benchmark harness and the TPTP reader. Linear-specific: the one-sided dyadic sequent with occurrence bitsets, the focused engines, the counts, the nets, the Petri-net decisions. A truth-table mode, a tableau and a clause-based proof need an evaluator or a tree kernel next to the sequent kernel and share nothing with the engines (inference). So the sensible shape is one library of syntax, terms, derivation display and export, with each logic's kernel beside the linear one (inference).

## 6. Ranking: the five to add first (inference)

1. **Truth tables and trees as the beginner's mode** (more-logics-2): the cheapest, closest to the evidence of course use [1] [4] [5], and it feeds the Learn lessons.
2. **Classical first-order logic with equality** (more-logics-1): the TPTP mainstream [11], and the gap in the planned ordinary first-order step.
3. **Intuitionistic first-order with Kripke countermodels** (more-logics-6): extends the ordinary suite with ILTP's first-order half [12].
4. **QBF input and the game view** (more-logics-3): a reader plus the game reading, over an encoding that already exists.
5. **Hoare logic over first-order assertions** (more-logics-5): a live teaching demand [6] [7], reached through custom rules.

Close behind: DRAT beside resolution (more-logics-7), finite-valued tables (more-logics-10).

*Overlaps with the author's list:* 2 and 6 sit with ordinary sequents and tableaux; 1 with ordinary classical sequents; 5 with custom rules; 7 with resolution; 4 and 9 with cyclic proofs and modal logics only in kernel shape; 4 (ALC) and 11 with the modal item.

## 7. Sources

All read 2026-11-08.

1. https://carnap.io (page read: "free and open software framework", dozens of institutions, copyright 2015 to 2026)
2. https://carnap.io/about (systems, exercise types, 31 institutions)
3. https://github.com/Carnap/Carnap (GPL-3.0, 2,469 commits; no activity date shown)
4. https://forallx.openlogicproject.org/ (forall x: Calgary, CC BY 4.0)
5. https://www.cs.us.es/~jalonso/cursos/li-06/sistemas/tableros/info.html (Tree Proof Generator help, via a search result; the page itself was not fetched) and https://www.umsu.de/wo/2002/3 (its author's update, search result only)
6. https://arxiv.org/abs/2303.05865 (Korkut, A Proof Tree Builder for Sequent Calculus and Hoare Logic)
7. https://softwarefoundations.cis.upenn.edu/plf-current/Hoare.html (first 100,000 of 316,689 characters)
8. https://people.irisa.fr/Francois.Schwarzentruber/software.html and https://www.ijcai.org/proceedings/2018/862 (Hintikka's World, via a search result)
9. https://github.com/jrclogic/SMCDEL
10. https://hackage.haskell.org/package/smcdel (licence GPL-2.0-only, via a search result)
11. https://www.tptp.org/
12. https://link.springer.com/chapter/10.1007/11554554_28 and https://www.cs.cornell.edu/info/people/kreitz/PDF/05tableaux-iltp.pdf (ILTP, via a search result)
13. https://en.wikipedia.org/wiki/True_quantified_Boolean_formula (the page flags several claims as needing citations)
14. https://www.qbflib.org/qbfeval22.php (QBFEVAL'22, via a search result)
15. https://github.com/marijnheule/drat-trim
16. https://en.wikipedia.org/wiki/Description_logic
17. https://en.wikipedia.org/wiki/Linear_temporal_logic
18. https://arxiv.org/abs/2603.01595
19. https://fbinfer.com/docs/checker-pulse/
20. https://plato.stanford.edu/entries/logic-manyvalued/ (first 100,000 of 247,955 characters)
21. https://plato.stanford.edu/entries/logic-paraconsistent/
22. https://arxiv.org/pdf/1505.01338 (KBCV 2.0) and https://cl-informatik.uibk.ac.at/research/software (CSI, TTT2, CeTA), via a search result

## 8. Not verified

- Vampire's and E's pages (404 or not read); the TPTP's sub-languages and problem counts; any CASC result.
- The ILTP library's first-order problem count and the live site's state; ILTP via search snippets only.
- The complexity of ALC with a TBox (the page read refers to the Complexity Navigator); the course use of ALC.
- Course use of Hoare logic tools beyond [6] and [7]; the rules for loops and conditionals in [7].
- Tableau or sequent calculi for FDE, LP and Lukasiewicz logic; the relation of Lukasiewicz logic to linear logic (not in the part read).
- Course use of LTL, CTL, SMCDEL and equational completion tools; NuSMV and SPIN (not read).
- Activity dates of Carnap, SMCDEL and DRAT-trim (the pages show none); whether a QBFEVAL 2025 edition exists.
- The rules files `.claude/rules/core*.md` were not read; section 5 rests on CLAUDE.md and the status chapter.
- Teaching of CDCL; deontic and conditional logics; categorical tools (no source).
- Cost estimates are this note's own judgement, with no source.
