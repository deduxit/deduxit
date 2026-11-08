# Resolution and clausal methods: what to add after 0.2.0

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

## 1. What linlog has today

- No resolution or clausal method. A search of `core/src` and `docs/src` for "resolution" finds nothing, and `docs/src/status.md` lists none.
- Ordinary classical, intuitionistic and minimal propositional logic through embeddings into linear logic, the proof read back as LK or LJ (`docs/src/ordinary-logic.md`, `core/src/ordinary/`). Classical logic goes into affine MALL without exponentials; `plan/later.md` says it costs one branch per way through the conjunctions, is "not a SAT solver, and not to be offered as one".
- A TPTP reader for ordinary logic: `read_tptp` in `core/src/ordinary/parse.rs` reads propositional `fof` formulas only (TPTP connectives, roles axiom, hypothesis, conjecture), not `cnf`, `tff` or `thf`, and no first-order TPTP. Output in SZS form is not offered (not found in the docs).
- The LLTP reader (`core/src/lltp.rs`), the `.spec` reader of coverability problems, the inverse method (a forward saturation with subsumption, which is clausal-like in spirit but over sequents, `docs/src/status.md`), and the Horn engine (Petri-net markings).
- First-order terms exist: `core/src/sequents/fo.rs` has `FoTerm`, `Symbol`, `Eigen` and tables, and the focused engine unifies with the occurs check and levels (`docs/src/status.md`, "First-order proof search"). Whether that unifier is reusable on clauses was not read (see section 7).
- Proofs as terms with an independent checker, derivation views, exports (LaTeX, Typst, SVG, PNG, PDF), an interactive prover, and the Rocq library with certificates.

The scope note `plan/notes/research/scope/engines.md` has four candidates that touch this area. Status of each now:

- engines-1 (SAT-based intuitionistic engine for ordinary logic): stands. It needs a CDCL solver with assumptions, which resolution-3 would bring. No step 38 to 45 report is known to have built it (not checked against every report).
- engines-2 (lazy SAT over axiom linkings for MLL): stands as research, independent of resolution except for the solver dependency.
- engines-4 (encodings as exports, DIMACS and others): stands, and is the cheapest first move; resolution-3 starts from its DIMACS printer.
- engines-11 (linear logic in the TPTP World): stands as outreach. A 2025 paper says the TPTP World has supported non-classical logics since v9.0.0 and describes quantified normal multi-modal logic [14]; I could not read how a logic is specified there (section 7), so the scope note's premise that modal logics have a logic specification is not re-verified.
- The scope note's rejection "Classical ordinary logic through SAT" (trivial, and later.md says not to offer linlog as a SAT solver) is in tension with what the author now asks. See resolution-3.

## 2. The field

**The method.** Resolution derives from two clauses with complementary literals their resolvent; applied to a CNF together with the negated conjecture, deriving the empty clause shows the conjecture follows. It is sound and refutation-complete for propositional and first-order logic. Robinson's unification algorithm of 1965 made it practical; factoring is needed for completeness in the first-order case. Davis and Putnam had the idea in 1960 but instantiated over ground terms [1]. Robinson's paper is J. ACM 12(1):23-41, 1965 (DOI 10.1145/321250.321253 per an aggregator, not verified at the publisher) [2]. Non-clausal resolution (Murray, Traugott) works on formulas directly and is cited as helping readability in interactive proving [1]. Paramodulation handles equality [1].

**Refinements.** Ordered resolution, hyperresolution, set of support, linear and SLD resolution, and superposition for equality are standard (Wikipedia names paramodulation only [1]; the other names I read in no source in this session, so treat them as (unsourced) here, though they are textbook). Leo-III is "based on a paramodulation calculus" [10]; Zipperposition is "meant to be a superposition prover for full first order logic" [9].

**DPLL and CDCL.** DPLL is Davis, Putnam, Logemann, Loveland (1960 and 1962), CDCL its descendant, in which each learned clause is derived by resolution steps from earlier clauses [3]. Pipatsrisawat and Darwiche (Artif. Intell. 175(2), 2011, DOI 10.1016/j.artint.2010.10.002) show that CDCL solvers as used in practice can simulate general resolution, without extra assumptions [4]. So a CDCL refutation is a resolution refutation (inference from [3] and [4]).

**Complexity.** Propositional resolution proofs can be exponential: Haken (1985, "The intractability of resolution") gives an exp(Omega(n)) lower bound for the pigeonhole formulas with n+1 pigeons and n holes; Tseitin had it for regular resolution in 1970 [5]. Deciding propositional satisfiability is NP-complete (textbook, unsourced here). First-order resolution is only a semi-decision procedure, as first-order validity is undecidable (textbook, unsourced here).

**Non-classical logics.** Resolution is less natural. Mints gave a resolution method for intuitionistic logic by translation into a sentential counterpart, so that its completeness follows from Mints' theorem, and "Resolution Strategies for the Intuitionistic Logic" (NATO ASI, 1993) [6]. For modal logics there are Mints, "Resolution calculi for modal logics" (AMS Translations 143, 1989), Enjalbert and Fariñas del Cerro, "Modal resolution in clausal form" (Theoretical Computer Science 65(1), 1989), and prefixed resolution for modal and description logics [6]. Fitting's correspondence between S4 and intuitionistic logic is the usual bridge [6]. I read only bibliographic records of these, not the papers.

**Proof formats for SAT.** DRAT is checked by drat-trim (MIT licence; its README covers DRAT and a binary DRAT encoding) [11]. LRAT extends DRAT with hints so that checking is simple and fast; the paper (CADE-26, 2017, LNCS 10395, DOI 10.1007/978-3-319-63046-5_14) gives two verified checkers, in Coq and ACL2 [12]. cake_lpr (Tan, Heule, Myreen, TACAS 2021) is a verified checker in CakeML for LPR, which is LRAT-compatible plus propagation redundancy; it was used to validate the unsatisfiability proofs of the 2020 SAT Competition [13].

**The TPTP World.** SZS statuses are reported on one line "% SZS status ..."; with a conjecture the usual values are Theorem and CounterSatisfiable, without one Satisfiable and Unsatisfiable. Sutcliffe's 2008 paper notes that a clause-normal-form refutation prover can misleadingly say "unsatisfiable" for a problem that has a conjecture, which is why the exact labels exist [15]. GDV verifies proofs in TPTP format: structure (leaves trace to the problem formulae) and that a refutation's root is false, standalone and in SystemOnTSTP [14]. A 2025 FLAIRS paper chains GDV to the LambdaPi checker [16]. The TSTP library holds the outputs of many provers [14]. The TSTP derivation syntax itself I did not read (section 7).

**Provers.** All read TPTP.

| tool | licence | what it is | last activity |
|---|---|---|---|
| Vampire [7] | BSD-3-Clause | first-order prover; `vampire problem.p`, a portfolio mode `--mode casc` | repository page shows no date; a Vampire team news item of 2026-07-30 reports participation in CASC at FLoC 2026 [17] |
| E [8] | GPL (version not stated) | equational prover, README for version 3.5 | no date shown |
| Zipperposition [9] | BSD-2-Clause | OCaml, typed higher-order with equality and datatypes, TSTP traces, graphviz proofs | no date shown |
| Leo-III [10] | BSD-3-Clause | Scala, higher-order, paramodulation | no date shown |
| SPASS, Prover9/Otter | not read | Prover9's page could not be fetched (connection refused) | (unsourced) |

CASC is the annual competition; its index lists CASC-30 (2025) with Vampire 5.0 in most division columns, but I could not tell which column is which [18].

**SAT solvers.** CaDiCaL: MIT, "easy to understand and change" CDCL [19]. MiniSat, Kissat: not read (unsourced).

**Rust SAT crates** (versions and dates from crates.io, read today):

| crate | licence | latest | updated | note |
|---|---|---|---|---|
| varisat [20] | MIT/Apache-2.0 | 0.2.2 | 2020-09-09 | pure Rust CDCL, has `varisat-lrat` and checker crates [21]; 7.8 million downloads; no release since 2020 |
| rustsat [22] | MIT | 0.7.5 | 2026-01-30 | framework with interfaces to CaDiCaL, Kissat, Glucose, MiniSat and any IPASIR solver [23] |
| batsat [24] | MIT | 0.6.0 | 2025-01-20 | "Extensible SAT solver in Rust"; rustsat-batsat says "BatSat is fully implemented in Rust" [25] |
| cadical (bindings) [26] | MIT | 0.1.16 | 2025-04-24 | bindings to the C++ solver |
| splr [27] | MPL-2.0 | 0.19.0 | 2026-08-22 | pure Rust, writes DRAT with `--certify`, verified with drat-trim; MPL-2.0 is not on `deny.toml`'s list, so it is out |

Whether batsat or varisat emits DRAT or LRAT, and whether the C++ solvers build for wasm32-unknown-unknown, I did not source.

**Teaching tools.** An Edinburgh applet turns a propositional sentence into clause form and runs a simple resolution prover [28]. "Resolution Editor" is an npm package, a client-only interactive editor for first-order resolution proofs, described as a teaching tool [29]. ProverSLD (CEUR-WS, 2025) visualizes SLD resolution with a table linked to the search tree [30]. LAP (arXiv, 2026) has command-line tools for teaching logic including DPLL and natural deduction checking [31]. LogicLearner (Columbia) guides propositional proofs by equivalence laws, not resolution [32]. Carnap and the other course systems were not searched (unsourced). That the search finds only small tools for resolution is a weak sign that it is not well served on the web (inference, from one search).

## 3. Candidates

### resolution-1: Propositional clausal suite (CNF, resolution, DPLL), teaching-scale

*What it brings and to whom.* Teaching: resolution is the method taught after truth tables and natural deduction in most logic and AI courses, and the web page has no clausal tab (inference from `status.md`). Showing a refutation as a DAG of resolvents, with the choice of clauses to resolve left to the student, is what the tools in section 2 offer on a small scale [28][29]. For AI models using the guide: a deterministic, checkable refutation as an answer. Research: little; this competes with SAT solvers and must not claim to.

*A suite in linlog's terms.*
- Decide: CNF conversion (naive and Tseitin), then saturation by resolution with subsumption and tautology deletion, and a DPLL search with unit propagation. Exhaustive saturation is complete and ends for propositional input; it may blow up (Haken [5]), so a bound and an "unknown" answer as elsewhere in linlog.
- Check: a refutation is a list of clauses each a resolvent of two earlier ones; the checker is a few dozen lines and one pass, the same shape as the proof checker (inference). Checking the CNF conversion is the harder part: its correctness (equisatisfiability for Tseitin) is the trusted piece.
- Prove step by step: the student picks two clauses and a literal; the interactive layer would show the resolvent, flag tautologies and subsumed clauses, and offer hints. This is steppable only, since search is the machine's.
- Export: the DAG as LaTeX, Typst and SVG (a resolution tree with the clause under each node, as `core/src/export` draws derivations, but as a DAG, not a tree).
- Certify: a Rocq checker over clause lists with a soundness theorem, small (inference); the CNF side needs its own theorem.

*Cost.* 3 to 4 sessions: the clause data model and CNF (1), resolution and DPLL with bounds (1), checker, interactive mode, exports (1 to 2); one more for a Rocq checker. Trusted base grows by the clause checker and, until certified, the CNF conversion.

*Risk.* Scope creep toward competing with SAT solvers; the author's `later.md` line says linlog is not to be offered as one [see section 1]. The mitigation is the framing "teaching and certified small instances". Exponential blow-up is real but harmless with a bound.

*Prerequisites.* A clause type (literals as atoms with sign, a clause as a sorted set), a DAG of inferences; neither exists. No library needed. A web tab depends on the web plan's generic stage, which this note does not read.

*Sources.* [1][2][3][5][28][29].

### resolution-2: First-order resolution at teaching scale

*What it brings and to whom.* Teaching and research-lite: first-order resolution with unification is the standard method in courses on automated reasoning, and the author names resolution as a suite. It is also where linlog's first-order terms and unifier could be reused (section 5).

*A suite in linlog's terms.*
- Decide: only semi-decide. Clausification with Skolemization, then given-clause saturation with factoring, ordered resolution or set of support as options, under time and clause bounds. "Not valid" is only available when the saturation ends, which it does for some decidable fragments only; otherwise "unknown" (exit status 3 fits).
- Check: each step is a resolvent under a most general unifier, or a factor; checking applies the substitution and compares clauses up to renaming. Easy, and the substitution must be given in the proof.
- Prove step by step: choose two clauses, two literals, and see the unifier computed; hints on failing unification (occurs check). This is the teaching value.
- Export: DAG with the unifier on each edge.
- Certify: Rocq checker over first-order clauses, a larger development than resolution-1 because of substitution and renaming lemmas (inference). Skolemization and clausification are not equivalence-preserving (only equisatisfiable), which the certificate must cover.

*Cost.* 4 to 6 sessions after resolution-1: clause terms and unification over `FoTerm` (1), clausification (1), saturation with one or two strategies (1 to 2), checker and interactive (1 to 2). Equality (paramodulation, superposition) is a project of its own and out of this candidate [8][9][10]. Trusted base: the unifier and the clausifier.

*Risk.* Performance cannot approach Vampire or E [7][8], which are tuned over decades (a claim from the tools' standing at CASC [18], not measured); the suite must say it is for small problems. Equality problems are the common ones and are not covered. Undecidability gives only "unknown".

*Prerequisites.* resolution-1's clause DAG; a first-order TPTP reader (`fof` with quantifiers and `cnf`) beyond today's propositional `read_tptp`.

*Sources.* [1][2][7][8][9][10].

### resolution-3: A SAT backend with certified refutations (CDCL, DRAT, LRAT)

*What it brings and to whom.* Research and AI use: practical decision of propositional classical logic, and the route to engines-1 (intuitionistic through SAT) and engines-2 (lazy SAT for MLL) of the scope note. Certification is the field's own: SAT solvers certify UNSAT by DRAT or LRAT, checked by drat-trim or the verified cake_lpr [11][12][13], the pattern `plan/later.md` already cites for failure certificates of the focused engine.

*A suite in linlog's terms.*
- Decide: DIMACS export first (engines-4, no dependency); then a solver call returning a model or an UNSAT proof. A satisfying assignment is checkable at once; an UNSAT proof is LRAT.
- Check: an LRAT checker is simple by design [12]; a Rust one inside linlog, and the Rocq one beside it as the certificate (inference). Importing a DRAT proof from an external solver and checking it with a linlog checker needs a DRAT to LRAT conversion, which drat-trim does [11][12]; linlog would take LRAT.
- Prove step by step: not a fit; LRAT proofs can be millions of lines. Only a replay of small proofs as resolution DAGs (resolution-1) is meaningful, since learned clauses are resolution derivations [3].
- Export: DIMACS, and the LRAT proof as text; drawing is for small cases only.
- Certify: the Rocq checker with soundness; cake_lpr shows what the verified end looks like [13].

*Dependency choice.* The dependency rules (`CLAUDE.md`) allow MIT, Apache-2.0, BSD, Zlib, EUPL and Unicode licences. Options from section 2:
- RustSAT (MIT, 2026-01) is a framework; it needs a backend [22][23].
- batsat (MIT, 2025-01) is pure Rust, so it is a candidate for wasm, which is a stated goal; its proof output is unsourced [24][25].
- varisat (MIT/Apache-2.0) is pure Rust with LRAT crates, but unreleased since 2020 [20][21].
- cadical bindings (MIT) compile C++, which works against the library's wasm goal (the scope note's own inference, `engines.md`, and not re-verified here) [26].
- splr is MPL-2.0 and not allowed [27].
- Writing a small CDCL in-house is the alternative: a few thousand lines for a decent solver (estimate, unsourced), with no proof-format risk and the search owned. It would be slower than the above and CPU-bound on large inputs.
A short measurement session (build each for wasm32-unknown-unknown in a capped scope, on a few DIMACS inputs; I ran nothing) would decide.

*Cost.* The DIMACS printer: 1 session or less (engines-4). A dependency-based solver with LRAT: 2 to 3 sessions including the checker; the Rocq checker 1 to 2 more. In-house CDCL with proof logging: 3 to 4 sessions. Trusted base: the LRAT checker (Rust and Rocq); the solver itself is untrusted, which is the point of certificates.

*Risk.* The later.md position against being a SAT solver would need a decision from the author; the framing "certified, one backend among others" fits. Solver maintenance (varisat dormant). Wasm size and threads.

*Prerequisites.* The clause type of resolution-1; a decision on the dependency.

*Sources.* [3][4][11][12][13][19]-[27].

### resolution-4: TPTP reader for clauses and SZS output

*What it brings and to whom.* Interop: linlog reads `fof` propositional only today. Reading `cnf` and first-order `fof`, and answering with an SZS status line [14][15] would let linlog be called from the TPTP tools (the scope note's engines-11 goes further and proposes linear logic as a TPTP logic). For resolution suites it supplies the problems: a few thousand propositional problems exist in the library (unsourced count).

*A suite in linlog's terms.* Reader and printer only: decide (through resolution-1 to 3), check (GDV-style structural check of a TSTP derivation: leaves in the problem, root false [14]), export (a refutation as a TSTP derivation, as Zipperposition writes TSTP traces [9]). The TSTP derivation syntax was not read; a session would begin there.

*Cost.* 1 to 2 sessions for the reader and SZS lines; 1 more for TSTP derivation output. Trusted base: the parser.

*Risk.* The TPTP's rules about roles and conjectures give a non-trivial semantics (a conjecture is negated, a status without conjecture differs [15]); getting SZS statuses wrong is easy.

*Prerequisites.* resolution-1 at least, for something to answer with.

*Sources.* [14][15][16][9].

## 4. Considered and rejected

- **Resolution for intuitionistic or modal logic as a suite now.** The methods exist but each is a translation or a prefixed calculus with its own literature [6], I read only their references, and linlog's embeddings into linear logic already decide the ordinary intuitionistic logic. Later, if the author's modal-logic suite is chosen (a separate area).
- **Equality (paramodulation, superposition).** A project of its own; E, Vampire and Zipperposition spend decades on it [7][8][9].
- **Competing in CASC.** Rejected as in the scope note: a 2 to 5 order-of-magnitude gap is unmeasured here, but the field's tools are of a different class [18].
- **Splr as the solver.** MPL-2.0 [27].
- **A resolution engine inside the Horn engine or the inverse method.** The inverse method already saturates forward with subsumption; a clause-level reuse would be an adaptation of its tries, not a new feature (inference from `docs/src/status.md`; the module itself was not read).

## 5. What the structure must allow

- Reusable from the core: the first-order term arena and symbol tables (`sequents/fo.rs`), the unifier with the occurs check, the checker's pattern (one pass, memory proportional to the proof), the interactive layer's shape (a state, a step, a hint), the exports' single signature and options values, the batch, limits and `Stop`, the Rocq pipeline for certificates (`.claude/rules/core.md`, `core-proofs.md`, `core-export.md`; table rows read, module files not read in full).
- Linear-specific, not reusable: the forest of occurrences and `OccSet`, the focused engine's dyadic sequents, the counts, nets, `Mode`, `Fragment`, and the derivation view (trees of sequent rules). A resolution proof is a DAG of clauses, not a tree of sequents, so the proof term and derivation view need a sibling type rather than an extension (inference).
- Therefore a clause type (literals, clauses as sorted sets), an inference DAG with shared nodes, and a generic "decide, check, step, export" interface per method are what the structure should provide; whether `Decide` can be that interface was not read.

## 6. Sources

All read 2026-11-08.

1. https://en.wikipedia.org/wiki/Resolution_(logic)
2. https://www.bohrium.com/paper-details/a-machine-oriented-logic-based-on-the-resolution-principle/813217272737824768-2464 (search result for Robinson 1965; see also Cambridge JSL review in the same search)
3. https://en.wikipedia.org/wiki/Conflict-driven_clause_learning
4. Pipatsrisawat and Darwiche 2011, search result: https://portal.mardi4nfdi.de/wiki/Publication:543613
5. Haken 1985, search results: https://portal.mardi4nfdi.de/wiki/Item:Q1071750 and https://arxiv.org/pdf/1912.00534
6. Mints, modal and Enjalbert/Fariñas del Cerro references, search results: https://arxiv.org/html/2503.05360v2, https://ouci.dntb.gov.ua/works/42WDpzwl, https://ftp.math.utah.edu/pub/tex/bib/idx/tcs1985/65/1/1_33.html, https://arxiv.org/pdf/1304.1628
7. https://github.com/vprover/vampire
8. https://github.com/eprover/eprover
9. https://github.com/sneeuwballen/zipperposition
10. https://github.com/leoprover/Leo-III
11. https://github.com/marijnheule/drat-trim
12. https://arxiv.org/pdf/1612.02353
13. https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7984575/
14. https://arxiv.org/abs/2508.09318 (and its PDF, https://arxiv.org/pdf/2508.09318, via search snippets)
15. https://tptp.org/UserDocs/SZSOntology/ and https://www.cs.miami.edu/home/geoff/Papers/Conference/2008_Sut08_KEAPPA-38-49.pdf (search snippets only)
16. https://journals.flvc.org/FLAIRS/article/view/138642
17. https://forsyte.at/news/2026-07-30/ (title and date from a search result)
18. https://tptp.org/CASC/index.html (search snippet)
19. https://github.com/arminbiere/cadical (fetched through a redirect-style URL that resolved to the CaDiCaL page)
20. https://crates.io/api/v1/crates/varisat
21. https://github.com/jix/varisat
22. https://crates.io/api/v1/crates/rustsat
23. https://github.com/chrjabs/rustsat
24. https://crates.io/api/v1/crates/batsat
25. https://crates.io/api/v1/crates/rustsat-batsat
26. https://crates.io/api/v1/crates/cadical
27. https://crates.io/api/v1/crates/splr and https://github.com/shnarazk/splr
28. https://www.aiai.ed.ac.uk/~gwickler/proposition-resolution.html (search result)
29. https://www.npmjs.com/package/@4zzz/resolution-editor (search result; page itself returned 403)
30. https://ceur-ws.org/Vol-4311/PEG_regular_06.pdf (search result)
31. https://arxiv.org/pdf/2607.08000 (search result)
32. https://arxiv.org/pdf/2503.19280 (search result)

## 7. Not verified

- Last-commit dates of Vampire, E, Zipperposition, Leo-III, CaDiCaL, drat-trim, varisat, rustsat, batsat, cake_lpr (the repository pages showed none; crates.io dates are given where read).
- Prover9, Otter, SPASS, MiniSat, Kissat, Carnap and course systems: not read. The Prover9 page refused the connection.
- The TSTP derivation syntax, the GDV command and the exact `deny.toml` list (taken from `CLAUDE.md`).
- The names ordered resolution, hyperresolution and set of support are textbook but unsourced here; so are NP-completeness of SAT and undecidability of first-order validity.
- Whether batsat, varisat, cadical emit DRAT or LRAT, and whether the C++ solver builds for wasm32-unknown-unknown.
- Whether the focused engine's unifier is reusable on clauses (`core/src/search/focus` not read), and whether any step 38 to 45 report built engines-1 or engines-4.
- The CASC-30 division columns; the TPTP's way of specifying a non-classical logic; the number of propositional TPTP problems.
- Costs in sessions are estimates (inference), not measured.
- What I would have run, for the supervisor to decide: building each candidate solver crate for wasm32-unknown-unknown in a capped scope.
