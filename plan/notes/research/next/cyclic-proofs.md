# Cyclic proofs (infinite descent): research note for what to add after 0.2.0

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

**A warning about the name.** linlog's "cyclic linear logic" (`docs/src/cyclic.md`, `--cyclic`) is the non-commutative logic in which a sequent is a sequence of formulas read up to rotation. It has nothing to do with cyclic proofs. A cyclic proof is a finite derivation graph, with edges from open leaves back to earlier nodes, whose infinite paths must satisfy a progress condition; it is a way to reason about inductively defined predicates or fixed points. The two words share only "cycle". This note is about the second, which the author names as "cyclic proofs (infinite descent)" in `plan/46-research-next.md`.

## 1. What linlog has today

- No fixed points, no induction, no cyclic or non-wellfounded proofs. A search of `core/src`, `web/src` and `cli/src` for fixpoint, fixed point, induction, back-edge, circular proof and cyclic proof finds only comments that say "by induction" in proofs of lemmas, and the Rocq exporter's reserved words `Fixpoint` and `CoFixpoint` (`core/src/export/rocq/mod.rs`). `docs/src/status.md` lists none of it as built.
- First-order linear logic exists (quantifiers, eigenvariables, witnesses, unification; `docs/src/status.md`), without any inductive predicate. There is no first-order ordinary (LK/LJ) calculus with quantifiers in `core/src/ordinary/` that this note could rely on; the ordinary logics are propositional (task brief, `docs/src/status.md`).
- Proofs are compact terms: `Proof` owns an arena `Box<[Node]>` of 16-byte `Copy` nodes, premises as `NodeId`s, the last node the root; a subproof that two nodes share (a memo hit) is stored once, so the arena is a DAG but acyclic by construction (premises are earlier nodes), and a node never records its sequent, the checker derives it in one pass in memory proportional to the proof (`.claude/rules/core-proofs*.md`, `docs/src/status.md`).
- `plan/notes/research/scope/logics.md` (written 2026-10-14 to 17) has candidate **logics-5, Fixed points (μMALL)**: `μ` and `ν` binders over formula variables with Baelde's finitary rules, the focused system as discipline, bounded unfolding with a three-valued answer "since finitary μMALL is Σ⁰₁-complete", later circular proofs with a PSPACE validity check. Its cost line says binders in the arena, a node carrying an invariant, a checker for the invariant's premises, the interactive session asking for the invariant, a Rocq mirror with binders, and that "a circular proof is a graph, which the `Node` tree is not". Its section 4 says only logics-4 and circular logics-5 need "a certificate kind beside the `Node` tree". It lists non-wellfounded μMALL among the rejected ("Π⁰₁-hard, no finite proof object"). `plan/notes/research/scope/proof-theory.md` has no entry on cyclic proofs (a grep for cyclic, fixed, induct and circular finds only unrelated lines); `plan/later.md` has no entry either.
- `bench/TOOLS.md` names no cyclic-proof tool (grep).

Nothing below re-proposes what exists. Logics-5 is the one earlier record, and this note keeps it as cyclic-proofs-3 with what the sources now add.

## 2. The field

### 2.1 Calculi

**Cyclic proofs for first-order logic with inductive definitions.** Brotherston and Simpson define three systems over FO with inductive predicates (Edinburgh Research Explorer record of the journal paper, source 1; Brotherston's thesis record, source 2). LKID is sequent calculus with explicit induction, as left rules for inductive predicates; it is cut-free complete for Henkin models. LKIDω replaces induction by plain case-split left rules and a global soundness condition on infinite branches ("some inductive definition is unfolded infinitely often on each branch", by traces); it is cut-free complete for standard models. CLKIDω restricts LKIDω to regular proofs, those representable by finite graphs. The thesis abstract says that because such proofs have finite representations, "the soundness condition on proofs is thus decidable". They show CLKIDω subsumes LKID and conjecture the two are equivalent. The thesis adds a general framework for cyclic proof systems and a "cycle normal form" with a finitary, localised soundness condition.

**Shape of a cyclic proof, in the words of a tool paper.** A pre-proof is a finite derivation in which some terminal nodes, the buds, are labelled with sequents that already occur in the derivation, their companions; each bud-companion pair is a back-link; a bud has one companion, a companion may serve many buds. Traces are built from inductive atoms on the left of sequents; a step that strictly decreases is a progress point; "A pre-proof is a proof if every infinite path has an infinitely progressing trace starting from some point" (E-Cyclist, source 7).

**The open conjecture and its partial answers.** The equivalence of induction and regular infinite descent is the Brotherston-Simpson conjecture. Berardi and Tatsuta prove it for classical systems under arithmetic (LICS 2017) and show the intuitionistic version holds when both systems include Heyting arithmetic, while it fails in general (arXiv 1712.03502, source 8). Their LICS proof route is described only by a seminar abstract in the search result (source 8; not the paper).

**Cyclic arithmetic.** Simpson (FoSSaCS 2017, "Cyclic arithmetic is equivalent to Peano arithmetic") showed PA simulates cyclic reasoning and conversely (as reported by source 9; Simpson's paper itself was not read). Das (LMCS 16(1), 2020, arXiv 1807.10248, source 9): cyclic proofs using only Σₙ formulas prove the same Π(n+1) theorems as IΣ(n+1); CA and PA proofs of one theorem differ at most exponentially in size; consistency of CΣₙ is provable in IΣ(n+2) but not in IΣ(n+1).

**Linear logic with fixed points, finitary and circular (μMALL).** Baelde's μMALL (ACM TOCL 13(1), 2012, arXiv 0910.3383) has cut elimination and focusing; this is cited from `scope/logics.md` and was not re-read here. Baelde, Doumane and Saurin (CSL 2016, source 10) study the infinitary μMALLi, prove cut elimination and focalization, and aim to show it a "computational proof system in itself". Validity: a thread criterion; Doumane's thesis shows that it is PSPACE decidable by Büchi automata (as a search summary, source 11, of the thesis at irif.fr; the thesis was not opened). Nollet, Saurin and Tasson (CSL 2018, source 12) give a stricter, local, labelling-based criterion, decidable efficiently, for a fragment that still contains every circular embedding of Baelde's finitary proofs with invariants, and show the Brotherston-Simpson conjecture holds for that fragment (every labelled circular proof translates to a finitary one). The TABLEAUX 2019 paper by the same authors, cited in `scope/logics.md`, is the PSPACE-completeness of the thread criterion; its HAL page refused the fetch (source 13), so that claim rests on the scope note only. Bouncing threads (Baelde, Doumane, Kuperberg, Saurin, 2020, arXiv 2005.08257) generalise validity to account for cuts and axioms; the validity condition is undecidable in general but splits into a hierarchy of decidable ones indexed by a height (a search summary, source 11).

**Decision problems.** Das, De and Saurin (FSCD 2022, source 14): provability in the non-wellfounded μMALL is Π⁰₁-hard (reduction from non-halting of Minsky machines), strictly harder than the circular system, which is in Σ⁰₁; the least-fixed-point-only fragment is Σ⁰₁-complete (reachability of alternating vector addition systems with states), and so is the finitary system. So provability is undecidable in all variants; only checking a given proof is decidable (inference from the above and source 2).

**Other uses of the same idea.** Kleene algebra: Das and Pous (CSL 2018, source 15) give a cut-eliminating non-wellfounded system complete for the equational theory of Kleene algebra, with regular proofs and cut recovering the theory of action lattices. System T: Kuperberg, Pinault and Pous (POPL 2021, source 16) show a cyclic system over regular expression types computes the same functions on naturals as system T, and exactly the primitive recursive ones when contraction is dropped. Session types: Derakhshan and Pfenning (LMCS 18(2), 2022, arXiv 1908.01909, source 17) read circular proofs of linear logic with fixed points as session-typed processes with a decidable local criterion, stricter than Fortier and Santocanale's guard condition. I found no implementation linked from that page. Equational reasoning about functional programs: CycleQ (Jones, Ong, Ramsay, PLDI 2022, arXiv 2111.12553, source 18), a GHC plugin that uses the size-change principle for incremental global correctness; cyclegg (Polikarpova's group) redoes it on e-graphs and its README warns some CycleQ proofs it does not reproduce are believed unsound (source 18).

### 2.2 Deciding the trace condition

- The standard check is decidable but relies on complementing Büchi automata, exponential (source 7). I did not find a source that states the PSPACE bound for FOLID's condition in so many words; it is stated for μMALL's thread criterion (above).
- Stratulat's E-Cyclist (EPTCS 342, 2021, arXiv 2109.03235, source 7) validates cyclic FOLID proofs in polynomial time with a method whose costly steps resemble comparisons in multiset path orderings, implemented in Cyclist; it validated all proofs of Cyclist's original distribution; heuristics derive the ordering measures from the derivation. This is a sufficient criterion (inference from the "Validating back-links" line in the search summary, source 19).
- The abstract notion: the Isabelle entry below defines Infinite Descent on "sloped graphs", independent of any encoding, with cyclic-proof soundness and size-change termination as instances (source 20).

### 2.3 Tools

| tool | what | licence | activity | use |
|---|---|---|---|---|
| Cyclist (source 3, 4, 5, 6) | OCaml and C++ framework for cyclic provers; subcommands `sl`, `while`, `asl-while`, `proc`, `fo` (prove), `ltl`, `checkproof`; a C++ model checker checks the soundness (trace) condition; the generic prover (APLAS 2012, Brotherston, Gorogiannis, Petersen) lets a leaf link to any matching node and has three instances: FOL with inductive definitions, pure separation logic entailments, termination proofs for pointer programs | BSD-3-Clause (LICENSE.md, copyright 2012 to 2025) | 996 commits, 34 stars, 1 open issue; the page showed no commit date, but the licence runs to 2025 | research: verification of heap programs, separation-logic entailment (competition use: SL-COMP; source 21) |
| Cyclist-SL (source 3) | separation-logic entailment prover on the same theory | as Cyclist (inference) | SMT-COMP 2014 system description | research |
| Songbird (source 22, 23) | OCaml entailment prover for separation logic with user-defined inductive predicates, by mutual explicit induction (not a cyclic proof; the paper contrasts it with cyclic proofs); ships Z3 | not stated on its page | newest dated news item 2018-07-13 | research, SL-COMP |
| CycleQ (source 18) | GHC plugin, cyclic equational reasoning | not read | PLDI 2022 | research |
| Isabelle/HOL "Certified Infinite Descent Criteria" (source 20) | locale-based framework of sloped graphs; complete criteria shown equivalent to the abstract predicate; soundness of tool-facing sufficient criteria; verified counterexamples to completeness; Büchi lemmas; depends on Büchi Complementation; no proof-reconstruction pipeline from tool output | BSD | AFP development version dated 2026-06-11, ITP 2026 | research (certification) |
| Bedwyr (source 24) | OCaml model checker over syntactic expressions with bindings, built on μMALL-like proof search (CADE 2007; `scope/logics.md` says it is built on this proof theory) | not found | not found | research |
| Abella (source 24) | interactive prover on lambda-tree syntax | GPL-3.0-only | not read | research and some teaching (unsourced) |

I found no teaching tool for cyclic proofs: no course page, no web tool for drawing and checking cyclic derivations, and no Coq or Lean formalisation of the global trace condition (the search that found the Isabelle entry turned up none; absence of a hit is not proof of absence). I did not find a cyclic-proof tool for μMALL, circular proofs of linear logic or the session-type language of source 17 in any search. A tool for μMALL circular proofs would thus be new (inference, from a negative result).

**Cyclic proof search in practice (Cyclist).** The prover builds a derivation by backward rule application, and whenever a new open leaf matches an ancestor (up to substitution) it tries to close it as a bud with a back-link; the trace condition is checked on the finished or partial graph (source 6 as summarised by the search result, which says "leaves may be linked to any other matching node"; the details of the search strategy were not read, the APLAS PDF refused the fetch).

## 3. Candidates

### cyclic-proofs-1: a checker for the trace condition on cyclic pre-proofs (the kernel of the area)

*What it brings and to whom.* Research and teaching: the one decidable, checkable core of every system in 2.1. Without it no other candidate has a checker. For the AI guide: a proof with a back-link that a model builds can be checked with a verdict and a witness (the bad path). Evidence of use: Cyclist's `checkproof` subcommand and its C++ model checker exist for exactly this (source 3); Isabelle's framework treats the condition abstractly (source 20).

*What a suite would offer.* Check: yes, decided. A pre-proof is a finite graph of sequents, rules and back-links; an edge-labelled graph of "slopes" (a trace step is decreasing or progressing between a premise's and a conclusion's atoms) is the abstract object; the condition is an omega-regular property, so a Büchi-automaton inclusion test is the textbook procedure (exponential in the plain method, source 7), the thread criterion of μMALL is PSPACE (above), and the polynomial sufficient criteria exist (sources 7, 12). Decide, search: not here. Prove step by step: not here. Export: the graph can be drawn with the back-edge (see 5). Certify: depends on cyclic-proofs-5.

*Cost.* Two to three sessions (inference): the graph type and a reader (JSON), a slope-graph trace checker with one exact method (for example the automata-free "size-change"-style closure under composition, which decides the condition on slope graphs, from the size-change literature, not read here; or the Büchi route) and one fast sufficient criterion, a counterexample path as the refusal, tests with cyclic proofs from the papers. Adds the trace checker to the trusted base: a bug accepts an unsound proof.

*Risk.* Which exact algorithm to implement: the Büchi complement is costly and notoriously subtle to implement correctly; a polynomial sufficient criterion is cheaper but rejects some valid proofs (sources 7, 12, 20 on incompleteness). No library yet for omega-automata in the dependency list (inference).

*Prerequisites.* A graph proof object (see section 5). Nothing else, if the checker is given the rules as data (a generic checker); with the rules built in, one of 2 or 3.

*Sources.* 2, 7, 9, 11, 12, 20.

### cyclic-proofs-2: cyclic LK for first-order logic with inductive definitions (CLKIDω), check and step by step

*What it brings and to whom.* Teaching and research: the standard textbook-level setting of "infinite descent", the one the author's phrase most directly names (the Fermat-style method, thesis abstract, source 2). Examples such as the associativity of addition on `nat` or `even(x) or odd(x)` are the usual ones (unsourced here: I did not read an example list). To the guide's users: a model can build a descent proof and be told which step breaks the trace condition.

*What a suite would offer.* Check: the finite graph plus the trace condition of 1, decidable. Prove step by step: yes, this is the natural interactive feature: the user applies rules (left-unfold a predicate, case split, cut, quantifier rules), and may close an open goal against an ancestor with a substitution, the session reporting at once whether some trace progresses (Cyclist's `fo` prover does the same automatically, source 3). Decide or search: only bounded and heuristic: provability is undecidable (the first-order setting contains arithmetic; source 9 shows cyclic arithmetic is as strong as PA, hence undecidable by Gödel; inference), so the engine would be Cyclist-style (see cyclic-proofs-4). Export: the derivation drawn in LaTeX, Typst and SVG with back-edges. Certify: a Rocq mirror is a research task (cyclic-proofs-5).

*Cost.* Four to six sessions (inference): a first-order ordinary classical sequent calculus with equality and inductive predicate definitions in the data model, which linlog does not have today (its first-order logic is linear and its ordinary logic is propositional); the rules for unfolding; the graph proof; interaction; exports; a hand-chosen set of examples. Trusted base: the first-order rules and the trace checker.

*Risk.* The calculus is large: equality, substitution, definitional unfolding with case-split rules whose premises need freshness, atoms that are traced through substitution. The hardest test is soundness, which is a theorem about the whole system (conjecture of equivalence with induction is open in general, source 2, 8). Proof search is unbounded.

*Prerequisites.* cyclic-proofs-1; first-order ordinary classical logic (a separate candidate area of the ordinary-logic notes of this step, not decided here); a generic derivation kernel with rule schemas, if the author takes the collection of suites (see 5).

*Sources.* 1, 2, 3, 7, 8, 9.

### cyclic-proofs-3: μMALL, finitary and circular (logics-5 restated)

*What it brings and to whom.* Research and teaching, and the nearest to what exists: μMALL is linear logic with least and greatest fixed points (source 10), the only candidate here that extends linlog's own logic rather than adding another one. Value: inductive and coinductive types and definitions in a resource-sensitive setting; the session-types reading (source 17) gives it a programming meaning; Bedwyr is a model checker built on the proof theory (`scope/logics.md`, source 24). The evidence of tools is thin: I found no tool that checks or builds μMALL circular proofs (see 2.3).

*What a suite would offer.* Check: yes, in two ways. Finitary proofs with explicit induction rules and invariants: finite tree, local rules, decided in one pass like today's proofs. Circular proofs: a graph with the thread criterion, PSPACE by Büchi automata (source 11, 13 via scope note), or the local criterion of Nollet, Saurin and Tasson in a fragment (source 12), or the session-types criterion (source 17), the last two efficient. Prove step by step: yes, finitary (the session asks for the invariant of an induction) and circular (close against an ancestor, check the thread). Decide: no, provability is Σ⁰₁-complete finitary and circular and Π⁰₁-hard non-wellfounded (source 14), and Σ⁰₁-complete already for least fixed points alone; a bounded unfolding search can answer proved, or unknown with the bound as the reason, never "unprovable" except by the false-read-classically refutations, which carry over only if a classical reading of fixed points is defined (not sourced). Export: yes, the derivation with its back-edges. Certify: finitary proofs through a Rocq mirror of the rules (the library `rocq-linlog` is for the calculus without binders); circular ones need cyclic-proofs-5.

*Cost.* Six to nine sessions (inference): formula-level binders and substitution in the arena, the walk and the printers (the scope note's D-10 binder stop), the invariant node, the finitary checker, the focused search with unfolding bounds, the graph proof and trace checker (cyclic-proofs-1), the interactive session, exports, Rocq mirror. Trusted base: the binder machinery, the invariant rule, the trace checker.

*Risk.* High. Everything in the focused engines assumes a formula with no binders over formula variables; first-order quantifiers bind term variables, not formulas (inference from `docs/src/status.md`). The cut and focusing theory is in the sources but the engines would be new. Search is undecidable and the benchmark corpus is small (none found).

*Prerequisites.* A graph proof object; binders over formula variables (the scope note says "after 38", and step 42 has built first-order, which gives substitution and levels for terms); cyclic-proofs-1.

*Sources.* 10, 11, 12, 13, 14, 17, `scope/logics.md` (Bae12, BDKS20, DDS22, NST18, NST19 as cited there).

### cyclic-proofs-4: Cyclist-style search with back-link closing, bounded, for one fragment

*What it brings and to whom.* Research: automation of descent proofs for verification (Cyclist `sl`, `while`, `fo`, source 3; SL-COMP, source 21). For linlog's users and the guide a decision answer would be proved, with the checked graph, or unknown, with the bound.

*What a suite would offer.* Search: backward search with unfolding and ancestor-matching, trace-check on each closing; decide: only inside fragments where the number of distinct sequents is bounded (not sourced; I did not find a decidable fragment of cyclic FOLID proofs in what I read, other than the decidability of checking a given proof).

*Cost.* Four or more sessions after 2 or 3 (inference), including the memoisation of sequents modulo substitution that the existing memo (an exact match on bitsets) does not do.

*Risk.* High for little visible result: the problems of interest in the sources are separation-logic entailments, whose heap model linlog lacks, and its first-order linear provers are fed by unification rather than by matching up to substitution of inductive atoms. No benchmark set in the area came to light for a comparison (Cyclist's SL-COMP problems are separation-logic).

*Prerequisites.* 2 or 3.

*Sources.* 3, 6, 21.

### cyclic-proofs-5: certificates for the trace condition (Rocq)

*What it brings and to whom.* Research: soundness of the trace condition is "the hard theorem"; for linlog, whose point is that every verdict has an independent certificate (`rocq-linlog`, `docs/src/status.md`), a cyclic proof is the one case where the checker alone is hard to trust.

*What a suite would offer.* Certify: an exported proof that Rocq checks through a mechanised soundness theorem for the trace condition. The mechanisation exists in Isabelle/HOL for the abstract sloped-graph condition with complete criteria, sufficient criteria and Büchi lemmas, BSD licence, but not as soundness for any concrete cyclic proof system and with no reconstruction from tool output (source 20). I found no Coq or Lean one (2.3).

*Cost.* Large: four to eight sessions in the Rocq library's repository (inference), a research-sized task: the infinite Ramsey theorem or an equivalent appears in Berardi and Tatsuta's route (source 8, from a seminar abstract) and a Büchi complementation (source 20 depends on one).

*Risk.* Very high. Alternatively, certify the finitary μMALL proofs only (cyclic-proofs-3) and use a local criterion that translates a circular proof to a finitary one (source 12, in its fragment), which sidesteps the omega-regular theory: the certificate is then the translated finitary proof (inference).

*Prerequisites.* 1 and one of 2 or 3.

*Sources.* 8, 12, 20.

## 4. Considered and rejected

- **Non-wellfounded (infinite, non-regular) proofs as objects.** No finite object; Π⁰₁-hard for μMALL (source 14); `scope/logics.md` rejects them too.
- **Cyclic arithmetic as a suite.** An interesting equivalence with PA (source 9), but a suite would only add a numeral language and a trace on arithmetic atoms; a model would sooner use cyclic-proofs-2 with `nat` as an inductive predicate (inference).
- **Cyclic Kleene algebra proofs (Das and Pous).** Its decision problem is decidable by other means, regular-expression equivalence, and cyclic proofs are not the tool one would use (inference from source 15); and the regular fragment of the cut-free system is not complete (source 15's background in the search summary).
- **Separation logic with inductive heap predicates.** The setting of Cyclist and Songbird's results, but it needs a heap model and an SMT back end (Songbird ships Z3, source 22) and a separate area of the author's list ("a collection of suites"); not part of this one.
- **Cyclic equational reasoning for functional programs (CycleQ).** Needs a language of programs and rewriting; no link with linlog (inference).
- **Size-change termination as a feature of its own.** It is an instance of the abstract condition of source 20, and its use belongs to a tool for program termination, not to a logic suite.
- **Cyclic proofs for the modal mu-calculus (Fortier and Santocanale).** Named in sources only in passing (as a comparison in source 17); not read; leave to the modal-logic note.

## 5. What the structure must allow

- **Reusable from core.** Parsing and printing, the arena representation with `NodeId`s (the DAG that already shares subproofs is one step from a graph: only the rule that premises are earlier nodes must be relaxed for a back-link), the interactive session's tree of goals, the derivation views and the SVG, LaTeX and Typst drawing, the JSON reader with its refusals, first-order eigenvariable and unification machinery (for CLKIDω's substitutions), the Rocq export's layout.
- **Not reusable or to be rebuilt.** The checker's "one pass in memory proportional to the proof" is a postorder over a DAG: a back-link makes the premise a later node, so the pass becomes a graph traversal with a fixed point or a trace analysis over the whole graph (a Büchi or a size-change closure), which is not one pass and not linear. The "node never records its sequent, the checker derives it" convention fails for a bud: the bud's sequent must equal its companion's up to the substitution, so derivation of one node's sequent depends on the others' (inference from `.claude/rules/core-proofs*.md`). Fixed-point formulas need formula-level binders and substitution (`scope/logics.md`, D-10 binder stop). The focused engines and the memo are specific to binder-free linear sequents and do not carry over (inference); the thing that does carry over to a cyclic search is the idea of a memo of seen sequents, which is exactly the ancestor-matching of a back-link (inference).
- **A graph proof type beside `Node`.** `scope/logics.md` already says so for logics-5 and for BV. A generic kernel (rule schemas, judgements, a checker against schemas) as in the author's "collection of suites" direction would give cyclic-proofs-1 its natural home: a back-link is a cycle in a graph of rule instances, and a generic checker could check the local rules and delegate the global condition to the trace module.

## 6. Sources

1. Brotherston, Simpson, Sequent calculi for induction and infinite descent, J. Logic Comput. 21(2), 2010. <https://www.research.ed.ac.uk/en/publications/sequent-calculi-for-induction-and-infinite-descent/> (read 2026-11-08; DOI 10.1093/logcom/exq052; LICS 2007 version DOI 10.1109/LICS.2007.16 from a search result)
2. Brotherston, Sequent calculus proof systems for inductive definitions, PhD thesis, Edinburgh, 2006. <https://era.ed.ac.uk/items/f3318657-8d5e-42e4-810b-9eba4235a450/full> (abstract only; 2026-11-08)
3. Cyclist repository. <https://github.com/cyclist-org/cyclist> (2026-11-08)
4. Cyclist site. <https://www.cyclist-prover.org> (2026-11-08)
5. Cyclist LICENSE. <https://raw.githubusercontent.com/cyclist-org/cyclist/master/LICENSE.md> (2026-11-08)
6. Brotherston, Gorogiannis, Petersen, A generic cyclic theorem prover, APLAS 2012, <https://link.springer.com/doi/10.1007/978-3-642-35182-2_25>; PDF <https://discovery-pp.ucl.ac.uk/10140071/1/cyclist_APLAS-new.pdf> (not fetched: 403; known only from a search summary, 2026-11-08)
7. Stratulat, E-Cyclist, EPTCS 342, 2021. <https://arxiv.org/abs/2109.03235> and <https://ar5iv.labs.arxiv.org/html/2109.03235> (2026-11-08)
8. Berardi, Tatsuta, Equivalence of intuitionistic inductive definitions and intuitionistic cyclic proofs under arithmetic. <https://arxiv.org/abs/1712.03502>; LICS 2017 page <https://lics.siglog.org/2017/BerardiTatsuta-Equivalenceofinduct.html> (search results only, 2026-11-08)
9. Das, On the logical complexity of cyclic arithmetic, LMCS 2020. <https://arxiv.org/abs/1807.10248> (abstract; the PDF was binary; Simpson's FoSSaCS 2017 paper as reported there, 2026-11-08)
10. Baelde, Doumane, Saurin, Infinitary proof theory: the multiplicative additive case, CSL 2016. <https://drops.dagstuhl.de/opus/volltexte/2016/6582> (2026-11-08)
11. Search summaries of Doumane's thesis and of bouncing threads. <https://www.irif.fr/~doumane/these.pdf>, <https://arxiv.org/abs/2005.08257> (not opened; 2026-11-08)
12. Nollet, Saurin, Tasson, Local validity for circular proofs in linear logic with fixed points, CSL 2018. <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2018.35> (via search summary, 2026-11-08)
13. Nollet, Saurin, Tasson, PSPACE-completeness of a thread criterion for cyclic proofs in linear logic with least and greatest fixed points, TABLEAUX 2019. <https://hal-univ-tlse3.archives-ouvertes.fr/INRIA/hal-02173207> (fetch refused; claim from `plan/notes/research/scope/logics.md`)
14. Das, De, Saurin, Decision problems for linear logic with least and greatest fixed points, FSCD 2022. <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2022.20> (2026-11-08)
15. Das, Pous, Non-wellfounded proof theory for (Kleene+action)(algebras+lattices), CSL 2018. <https://drops.dagstuhl.de/opus/volltexte/2018/9686> (search summary, 2026-11-08)
16. Kuperberg, Pinault, Pous, Cyclic proofs, system T, and the power of contraction, POPL 2021. <https://arxiv.org/pdf/2012.14421> (search summary, 2026-11-08)
17. Derakhshan, Pfenning, Circular proofs as session-typed processes: a local validity condition, LMCS 2022. <https://arxiv.org/abs/1908.01909> (2026-11-08)
18. Jones, Ong, Ramsay, CycleQ, PLDI 2022. <https://arxiv.org/pdf/2111.12553>; cyclegg <https://www.github.com/nadia-polikarpova/cyclegg> (search summary, 2026-11-08)
19. Stratulat, Validating back-links of FOLID cyclic pre-proofs, CL&C 2018. <https://core.ac.uk/download/163007613.pdf> (search summary only, 2026-11-08)
20. Wright, Cohen, Rowe, Popescu, Certified Infinite Descent Criteria, AFP. <https://devel.isa-afp.org/entries/Infinite_Descent_Criteria.html>; ITP 2026 <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITP.2026.14> (2026-11-08)
21. SL-COMP 2018 mention on the Songbird page (source 23).
22. Ta, Le, Khoo, Chin, Songbird, arXiv 1609.00919. <https://www.arxiv.org/pdf/1609.00919> (search summary, 2026-11-08)
23. Songbird site. <https://songbird-prover.github.io> (2026-11-08)
24. Bedwyr (Baelde, Gacek, Miller, Nadathur, Tiu, CADE 2007) <https://arxiv.org/abs/cs/0702116>; Abella <https://opam.ocaml.org/packages/abella> (search results only, 2026-11-08)
25. Repository: `docs/src/status.md`, `docs/src/cyclic.md`, `plan/46-research-next.md`, `plan/notes/research/scope/logics.md`, `.claude/rules/core-proofs*.md`, `core/src/export/rocq/mod.rs`.

## 7. Not verified

- The exact definition of the trace condition in Brotherston-Simpson (only the E-Cyclist paraphrase was read) and the PSPACE bound for FOLID's condition; the PSPACE bound for μMALL's thread criterion is from a search summary of Doumane's thesis and from the scope note's citation of the TABLEAUX 2019 paper, whose page refused the fetch.
- Cyclist's search strategy (the APLAS PDF refused), its last commit date, and the licence of Bedwyr and Songbird; Abella's GPL-3.0-only is from its opam page.
- Simpson's FoSSaCS 2017 paper and Baelde's μMALL paper were not read; Fortier and Santocanale's circular proofs for the mu-calculus, and the equivalence results beyond the cited ones.
- Any teaching use of cyclic proofs, any decidable fragment of cyclic FOLID provability, and any Coq or Lean formalisation of the trace condition: none found, which does not prove none exists.
- The "exact decision of the trace condition by size-change closure" in cyclic-proofs-1 is background knowledge not read in this session.
- All cost estimates in sessions are inferences. Nothing was run.
