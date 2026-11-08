# Research: the simply typed lambda calculus, System F and the Curry-Howard side

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

## 1. What linlog has today

- No lambda-terms and no types in the type-theoretic sense: a grep of `docs/src/status.md`, `docs/src/first-order.md` and `plan/later.md` finds no lambda-term syntax, type checker, inhabitation entry point or term printer. What the manual calls "proofs as compact terms" (`docs/src/status.md`) are proof terms over subformula occurrences, not lambda-terms.
- Intuitionistic linear logic is built whole (`docs/src/status.md`): ILL and IMLL sequents are parsed, decided by the two-sided focused engine, the IMLL net embedding, the additive fast path and the Horn engine, checked, proved step by step, drawn two-sided, and cut elimination runs on proof terms and on nets. By Curry-Howard these proofs are linear lambda-terms, but nothing reads them as such.
- Ordinary classical, intuitionistic and minimal propositional logic are decided through embeddings, with the proof read back as LK or LJ and checked by a checker of its own (`docs/src/status.md`, `core/src/ordinary/`: `translate.rs`, `derivation.rs`, `decide.rs`, `rocq.rs`). So the STLC's types (implicational propositional formulas) are already decidable as formulas, and a proof of one is an LJ derivation.
- First-order quantifiers range over terms, not over propositions (`docs/src/first-order.md`: `forall x. A`, predicates applied to terms, function symbols). `core/src/sequents/term.rs` has `Term::Forall` and `Term::Exists` over an arena, locally nameless (bound variable of index 0, outer ones shifted). That is a binder for first-order variables inside formulas; there is no binder for proof or program variables, and no quantifier over atoms (no second-order propositional quantification).
- Cut elimination, step by step with named cases, exists for proof terms and nets (`docs/src/status.md`). It is the proof-theoretic twin of beta-reduction, but is shown on sequent proofs, not terms.
- The scope note `plan/notes/research/scope/types-higher.md` (written before steps 38 to 45) has seven candidates. Against `docs/src/status.md` as of 0.2.0 (the status page names none of them, so I treat none as built; (inference) from absence):
  - types-higher-1 (inhabitation: enumerate, count, decide uniqueness) still stands, unbuilt.
  - types-higher-2 (term printers for ILL proofs) still stands, unbuilt. Its premise changed in one way: step 40's recorded sides and the two-sided surface give the printer the data it needed.
  - types-higher-3 (sessions) and types-higher-4 (a builder and certificate back end for outside type checkers) still stand and belong to the session-types and applications notes, not here.
  - types-higher-5 (graded modalities), 6 (linear logic programming with terms) and 7 (relevant mode): not built; not in this area.
  This note takes types-higher-1 and 2 forward and generalises them past the linear case.

## 2. The field

### 2.1 The calculi

- **System F** was introduced by Girard (1972, thesis) and independently by Reynolds (1974); under Curry-Howard it is second-order propositional intuitionistic logic [1]. Its Church-style form has type checking that is straightforward because the types are in the terms [1].
- **The lambda cube** organises eight systems (STLC, System F, LF, the calculus of constructions among them) by what may depend on what: Barendregt, "Lambda Calculi with Types", Handbook of Logic in Computer Science II, OUP 1992 [2]; fuller treatment in Barendregt, Dekkers and Statman, *Lambda Calculus with Types*, CUP 2013 [2]. I read only the bibliographic records, not the chapter.
- **Strong normalisation** of STLC and System F is proved by Girard's reducibility candidates in Girard, Lafont and Taylor, *Proofs and Types*, CUP 1989, which has chapters on normalisation, strong normalisation and strong normalisation for System F, free on Paul Taylor's site [3]. I saw the table of contents and not the proofs.
- **Linear lambda-calculus.** Intuitionistic linear logic has term assignment systems: Abramsky, "Computational interpretations of linear logic", TCS 111 (1993) 3-57 [4], and Benton, Bierman, de Paiva and Hyland, "A term calculus for intuitionistic linear logic", TLCA 1993, LNCS 664, 75-90, which gives terms for both sequent calculus and natural deduction, with reduction from cut elimination and normalisation [5].

### 2.2 Decision problems and their complexity

| problem | status | source |
|---|---|---|
| STLC type checking | easy, the types are in the terms (Church style) | [1] for System F, same for STLC (inference) |
| STLC type inference | can run in linear time (one source says so without proof) | [8] |
| System F type checking (Curry style) and typability | undecidable, and equivalent; Wells reduces from semi-unification | Wells, APAL 98 (1999) 111-156, [6]; LICS 1994 version [6] |
| System F type checking (Church style) | decidable | [1] |
| ML (Hindley-Milner with let) typability | DEXPTIME-complete (Kfoury, Tiuryn, Urzyczyn, CAAP 1990; Mairson, POPL 1990) | [8] |
| STLC inhabitation = implicational intuitionistic provability | PSPACE-complete (Statman, TCS 9(1) 1979, 67-72; the abstract says deciding whether a type is the type of a closed lambda-term is PSPACE-complete; the implicational fragment is PSPACE-complete too) | [7] |
| System F and Fomega inhabitation | undecidable (Urzyczyn, TLCA 1997, LNCS 1210, 373-389, syntactic proof; Löb 1976 for second-order intuitionistic propositional provability) | [9], [10] |
| second-order linear logic | undecidable even without exponentials: IMLL2 and MLL with first and second-order quantifiers (Lincoln, Shankar, Scedrov, LICS 1995); MALL2 (Lafont, JSL 1996); full propositional LL (Lincoln, Mitchell, Scedrov, Shankar, APAL 56, 1992) | [11] |

Notes. I could not read Statman's or Urzyczyn's full text (secondary abstracts only). The "linear time" for STLC inference rests on one forum-level mention in a search result and is not a sourced theorem here (see section 7).

### 2.3 Tools and teaching materials

Each entry: link, licence, last activity, use, as far as the page shows. Where a page showed no date I say so.

- **Programming Language Foundations in Agda (PLFA)**, Wadler, Kokke, Siek: github.com/plfa/plfa.github.io [12], CC-BY-4.0, 3,084 commits on `dev`, no date visible. Its table of contents has chapters on the simply typed lambda calculus (Lambda, Properties, DeBruijn, More), bidirectional type inference, and normalisation of the untyped calculus; System F is not listed in the chapters I saw [13]. The site lists courses taught from it from 2018 to 2026, among them Siek at Indiana, Thiemann at Freiburg and Wadler at Edinburgh [13].
- **Software Foundations** (Pierce et al.), mirror github.com/DeepSpec/sf [14]: volumes lf, plf (Programming Language Foundations), qc, secf, slf, vc, vfa; licence not shown on the page, 45 commits, "mainly for CI". The page does not say that plf covers STLC or System F (unsourced; (inference) from the volume's name only).
- **Pie** (The Little Typer), github.com/the-little-typer/pie [15]: AGPL-3.0, Racket, "finished and will not change"; a dependently typed teaching language, installed with `raco pkg install pie` or in DrRacket. Not STLC-specific.
- **PLT Redex**, github.com/racket/redex [16]: a Racket DSL for operational semantics with interactive exploration and randomised testing, Apache-2.0 and MIT contributions, 1,717 commits, no date visible. The page does not describe its reduction visualiser or typesetting; I did not read the documentation.
- **TAPL** (Pierce 2002): chapters on System F (23, 25) and bounded quantification (26) are cited by [1]. I could not fetch the book's own page (frameset), so its contents beyond that are unsourced.
- **hmac/lc**, github.com/hmac/lc [17]: "a playground for exploring different type systems" (untyped, STLC, System T, System F, Hindley-Milner, a dependent calculus), PureScript (inference), demo at hmac.dev/lc/index.html; licence not stated, 70 commits, 2 stars. A small hobby tool, not a maintained suite.
- **rick00ne.github.io/LambdaCalculus_TypeChecker** [18]: per a search summary, a browser STLC checker that shows the typing derivation tree being built and exports it to LaTeX. The page itself I could fetch showed only its title, so the description rests on the search result; licence and repository not seen.
- **visual-lambda**, github.com/bntre/visual-lambda [19]: Python and pygame, LGPL-3.0, 47 commits, no releases; untyped lambda calculus drawn as bubble notation, beta-reduction animated, per-term undo and redo, selectable strategy.
- **lambda-playground**, github.com/thmsmlr/lambda-playground [20]: JavaScript step reducer with several strategies, one commit, no licence shown.
- **Tromp's Lambda Diagrams and playground**, tromp.github.io/cl/diagrams.html [21]: a graphical notation for closed lambda-terms; no licence found. Not verified further.
- Not found in this session: lambdacalc.io, "Lambda Viewer", Gabriel Lebec's visualiser (the repository URL I guessed returned 404), the "Lambda Calculus Calculator". I leave them out rather than describe them (see section 7).

What the survey shows (inference): the typed side is served by textbooks and proof-assistant courses (PLFA, Software Foundations) and by a few small web tools; the web tool that builds a derivation tree interactively is a hobby project. None of the tools found connects a typing derivation to a sequent calculus proof, a proof net or a certified checker, which is where linlog already has machinery.

## 3. Candidates

A common remark on cost: one session is one working day of reviewed, tested code. All candidates below stay in the implicational fragment of intuitionistic logic and its linear counterpart first, since that is where linlog has engines and Curry-Howard is exact (Church-style).

### lambda-calculi-1: Read proofs back as lambda-terms (linear and ordinary)

- *What it brings and to whom.* Students see that a proof of `A ⊸ B ⊸ A ⊗ B` is `λx.λy.(x, y)` and that an LJ proof of `(a -> b) -> (b -> c) -> a -> c` is function composition; this is the standard Curry-Howard exercise of the sources in section 2.3 [12][13] and is the first thing a type theory course shows. It also makes linlog's cut elimination legible as beta-reduction. For AI models using the guide, a term is a compact, readable answer to "give me a program of this type". Evidence of the need: the scope note's candidate 2 and its practice rows (`plan/notes/research/scope/types-higher.md`).
- *In linlog's terms.* Decide and prove: already built; the new thing is a read-back pass from a checked proof term (ILL, step 40's two-sided data) or an LJ derivation (`core/src/ordinary/derivation.rs`) to a term, printed in a neutral syntax (`λ`, application, pairs, `let` for `!`, `case` for `⊕`) and optionally in GHC `LinearTypes` and Granule syntax. Check: a type checker for the printed term (candidate 2) closes the loop, so a round trip is a test of the read-back. Step by step: the existing interactive derivation can show the partial term beside its goals. Export: the term in LaTeX, Typst, SVG labels; the derivation drawing can carry the term at each node (as a decoration of the existing tree). Certify: the term is a different presentation of an already certified proof, so no new trust.
- *Cost.* About 1 to 2 sessions for ILL and LJ read-back with one printer; plus about 1 for the second and third syntaxes and the web display. Adds no trusted base if the term is only printed; adds the printer to the trusted path only when it is claimed to correspond to the proof (then candidate 2's checker is what makes it checkable).
- *Risk.* Classical one-sided proofs have no lambda-term (the scope note says so); modes with Mix or the cyclic calculi need other reductions (CP for classical, not covered here). The mapping of `!` and `?` and of additives to terms depends on which calculus is chosen (Abramsky's or Benton et al.'s [4][5]); the choice must be stated. Exponential and first-order proofs need a decision on type quantifiers.
- *Prerequisites.* None beyond what is built; the derivation walk with binder stops (the scope note's D-10) helps.
- *Sources.* [4][5][12][13], scope note.

### lambda-calculi-2: A Church-style type checker with derivation trees (STLC, then linear)

- *What it brings and to whom.* Teaching: the exercise "build the typing derivation of this term" is the central STLC exercise (Northeastern CS4400 lecture on drawing typing derivations by hand, and the Sergey lecture, per a search summary [22]); the only web tool found that builds the tree interactively is a small hobby project [18]. Research: little. AI models: a checker that says why a term is ill-typed.
- *In linlog's terms.* A term syntax (variables, application, abstraction with annotated binder, pairs, units, sums), a bidirectional checker (types as implicational formulas: the ordinary formula arena is reusable), and the derivation as a proof tree in the same drawing machinery (rules `var`, `abs`, `app` instead of `ax`, `⊸R`, `⊸L`). Decide: type checking, linear time in the term for Church style (inference from [1]'s remark for System F). Check: the independent checker is the type checker itself; the correspondence to LJ or ILL derivations makes the term a second checker for proofs (inference). Step by step: build the derivation rule by rule, as in `interact`. Export: LaTeX and Typst trees, SVG. Certify: a Rocq checker for typing derivations is candidate 7.
- *Cost.* About 3 to 4 sessions: syntax and parser (locally nameless as `term.rs` already does for first-order binders, here for term variables), the checker, the derivation view and exports (large reuse), the web tab and a chapter of the manual and the guide. Adds a second checker to the trusted base.
- *Risk.* A new data model (terms with binders and a context with usage counts for the linear case) beside formulas; the plan's "no item is dead" and doc conventions apply to a new module family. The risk of scope creep toward a programming language (let, recursion, data types): stay with the pure calculus.
- *Prerequisites.* The ordinary logic's implicational syntax; ideally a generic derivation type (see section 5), since the derivation view is now sequent-specific.
- *Sources.* [1][12][13][18][22].

### lambda-calculi-3: Inhabitation: find, enumerate and count terms of a type

- *What it brings and to whom.* "Find a term of type T" is the Curry-Howard counterpart of proof search and what students and program synthesis tools ask for (the scope note's types-higher-1 cites synthesis tools wanting all inhabitants [S42 of that note]; I did not re-read that source). Research: counting and uniqueness of inhabitants (coherence) are live questions; the scope note cites a source for BCK.
- *In linlog's terms.* Decide: STLC inhabitation is PSPACE-complete [7], the same problem linlog's ordinary-intuitionistic path solves through ILL or the LJ search, so the decision is built; what is new is enumeration (continue after a proof, share failures but not successes) and counting. Linear inhabitation (IMLL, ILL) is already decided by the engines; enumeration is the same new control flow. Prove step by step: built (interactive proving); the term shows beside it (candidate 1). Export: lists of terms. Certify: each inhabitant is a checked proof, so certified through the existing certificates. System F inhabitation is undecidable [9][10]: a bounded search can return a term or "unknown", never "uninhabited".
- *Cost.* About 2 to 3 sessions: an enumeration front door with a `max_proofs` bound (the scope note's D-8 room), distinctness up to a stated equivalence (beta-eta normal forms for STLC, the focused calculus's identifications for linear, nets for MLL), a count and a `unique` verdict, tests against known counts. No new trusted base.
- *Risk.* The notion of "the same inhabitant" is subtle (focused proofs identify some but not all permutations: scope note); the number of inhabitants grows fast, so counts need a bound; PSPACE-completeness means some types blow up.
- *Prerequisites.* Candidate 1.
- *Sources.* [7][9][10], scope note.

### lambda-calculi-4: Type inference: principal types by unification, and Hindley-Milner

- *What it brings and to whom.* Teaching: "what is the type of `λx.λy.x y`" with the unifier shown step by step is a standard lecture (PLFA has a bidirectional inference chapter [13]; hmac/lc offers Hindley-Milner in a playground [17]). Research: none new.
- *In linlog's terms.* Decide: STLC type inference by first-order unification of type terms; the search engines already unify first-order terms with an occurs check and eigenvariable levels (`docs/src/status.md`), but over formula terms, not type terms with variables, so some code is reusable, not all (inference). Hindley-Milner with let-polymorphism is DEXPTIME-complete in the worst case [8] though fast in practice (inference; the source gives the worst case only). Step by step: the constraint set and substitution shown after each unification step. System F type inference is undecidable [6] and must not be offered; only checking.
- *Cost.* About 2 sessions for STLC inference with a trace, 1 more for let-polymorphism. Adds an inference procedure (trusted, but its output can be checked by candidate 2's checker, so not trusted).
- *Risk.* Small. The risk is a tool that duplicates the many small HM demonstrators (section 2.3).
- *Prerequisites.* Candidate 2.
- *Sources.* [6][8][13][17].

### lambda-calculi-5: A beta-reduction stepper and normalisation (typed terms)

- *What it brings and to whom.* Teaching: strategy comparison (normal order, applicative order) and the guarantee that typed terms normalise (strong normalisation of STLC and System F [3]); many small tools exist for the untyped case ([19][20]; Tromp's diagrams [21]). The link to linlog is the correspondence of reduction and cut elimination, which no tool found draws.
- *In linlog's terms.* Step by step: each beta step, with the redex marked, the strategy chosen, and for a typed term the typing derivation of each reduct (subject reduction as a check, candidate 2). Export: the sequence of terms. Linking: show the cut-elimination step on the proof beside the beta step on the term; the existing step-by-step cut elimination (built) is the proof-side half. Decide: normal forms are computable for typed terms (strong normalisation) but can have huge size (the non-elementary bound for STLC is a known result I did not source here, see section 7), so a step and size bound is needed, as `cut elimination` has today.
- *Cost.* About 2 sessions for terms and a stepper, 1 for the tie to cut elimination and the web display. No certification path unless a Rocq proof of subject reduction is added (a project of its own).
- *Risk.* Crowded space of untyped steppers; value lies only in the typed link. Alpha-equivalence and capture are the usual bugs (mitigated by locally nameless).
- *Prerequisites.* Candidate 2; candidate 1 for the cut-elimination tie.
- *Sources.* [3][19][20][21].

### lambda-calculi-6: System F: Church-style checking, and what stays out

- *What it brings and to whom.* System F is the second step of any type course (PLFA's table of contents does not list it [13]; TAPL has it [1]); the Curry-Howard partner of second-order propositional logic [1]. For linlog users: the second-order quantifier is the one thing missing between first-order linear logic and Girard's second-order linear logic.
- *In linlog's terms.* Decide: only Church-style type checking is decidable [1]; type inference [6] and inhabitation [9][10] are undecidable, as is second-order linear logic provability, even for fragments without exponentials (MALL2, IMLL2 [11]). So the offer is a checker and a derivation view, plus bounded proof search that can answer "found" or "unknown". Step by step: building a derivation with type abstraction and application; rules for `∀` over types. Export: derivation trees. Certify: a checker in Rocq (candidate 7) and the strong-normalisation proof stays outside.
- *Cost.* About 3 sessions after candidate 2: a binder over types (a second binder kind in the term and type syntaxes), substitution of types, the rules and views, and a decision whether the linear second-order quantifier is in scope. Trusted base grows by the substitution function.
- *Risk.* A tool that cannot decide anything interesting is only a checker; a student may expect inference to work. The undecidability results mean the interface must say "unknown" honestly.
- *Prerequisites.* Candidates 2 and, for the guide, 1.
- *Sources.* [1][6][9][10][11][13].

### lambda-calculi-7: A Rocq checker for typing derivations

- *What it brings and to whom.* The same story as for sequent proofs: the typing derivation or inferred type is accepted by a checker with a soundness theorem, which is what distinguishes linlog from the hobby tools of section 2.3 (inference). To research and teaching with Rocq, and to AI models that want a verdict they need not trust.
- *In linlog's terms.* Certify: terms and types as datatypes in the `rocq-linlog` library, the checker as a function, and the theorem that an accepted term has the type; the exporter writes the instance. Relation to what exists: the library already has linlog's proof terms as datatypes with checkers and soundness theorems for sequents (`CLAUDE.md`, section "Project"), and an `ILL` calculus; the typing judgement of STLC is a relabelling of the intuitionistic implicational fragment, so the proof of equivalence with the sequent calculus is a lemma, not a new development (inference).
- *Cost.* About 2 sessions in the Rocq repository after candidate 2, plus a session to wire the certificates through the exporter and the flake checks (`modules/rocq-linlog.nix`). Adds to the trusted base only Rocq's kernel and the statement of the theorem.
- *Risk.* Binder representation in Rocq (locally nameless or de Bruijn) must match the Rust checker's; each side's substitution lemmas are real work.
- *Prerequisites.* Candidate 2.
- *Sources.* the `CLAUDE.md` description of `rocq-linlog`; [12][14] as courses that do this in Agda and Rocq (Software Foundations' PLF volume, unverified as to content).

### Ranking of this area's candidates (inference)

1 first: cheapest, highest teaching value, uses only what is built. Then 2 (the base for the rest), 3 (the natural payoff of 1, needs only the engines), then 7, 4, 5, and 6 last. Candidates 1 and 3 are the part of this area that is genuinely Curry-Howard for linlog; candidates 2, 4, 5 and 6 build a small typed-lambda-calculus tool beside the prover and compete with course tools.

## 4. Considered and rejected

- **System Fomega and the full lambda cube, dependent types.** Type checking becomes normalisation and conversion; this is the territory of Agda, Lean, Rocq and Pie, which exist [12][14][15]. The scope note rejected the same for linear dependent frameworks.
- **System F type inference and a general inhabitation decision.** Undecidable [6][9][10]; there is nothing to implement beyond a bounded search.
- **A general untyped lambda-calculus playground or combinator reducer.** Many exist [19][20][21]; nothing in linlog's machinery improves on them except the typed link of candidate 5.
- **Re-implementing PLT Redex or a proof assistant for STLC.** Not linlog's role; PLFA and Software Foundations already serve it [12][13][14].
- **A strong-normalisation proof of STLC or System F in Rocq as part of linlog.** A project of its own; the library's scope is checkers and soundness for linlog's calculi (inference).
- **CP / classical process terms for one-sided classical proofs.** Belongs with the session-types area (scope note types-higher-3).

## 5. What the structure must allow

- Reusable (judged from `core/src/sequents/` and the module list in `.claude/rules/core.md`, which I did not read in full): the formula arena for types (implicational formulas), the locally nameless binder technique of `Term::Forall`, the derivation tree view, rule labels and the LaTeX, Typst and SVG exports, the interactive session (goals, rules, undo), the JSON wire forms, the Rocq exporter, the focused two-sided and LJ search as the inhabitation oracle, and cut elimination as the proof-side reduction.
- Linear-specific: occurrence bitsets, counts and memo keyed by multisets of occurrences; a type context for the linear lambda calculus needs usage counts (multisets) while the ordinary STLC wants sets with duplication and weakening; the ordinary logic path sidesteps this by embedding into linear logic, but a term checker cannot (inference).
- New for this area: a term syntax with binders, substitution and renaming, a typing context as a first-class thing (not an occurrence set), and a derivation type whose nodes are labelled by terms. A generic "derivation against a rule table with a context discipline" (the same need as for custom rules and other logics) would serve candidates 2, 5 and 6 directly; building it in this area first would repeat the work.
- The second-order quantifier (candidate 6) needs a binder over atoms, absent in `term.rs` and in the first-order tables.

## 6. Sources (all read 2026-11-08)

1. https://en.wikipedia.org/wiki/System_F (Girard 1972, Reynolds 1974; Church-style checking; Curry-Howard with second-order logic; its reference list: Girard 1986 doi:10.1016/0304-3975(86)90044-7; Wells 1999 doi:10.1016/S0168-0072(98)00047-5; Pierce TAPL chapters 23, 25, 26)
2. https://academic.oup.com/book/52965/chapter/421961562 and https://aslonline.org/books/perspectives-in-logic/available-volumes/lambda-calculus-with-types/ (Barendregt 1992; Barendregt, Dekkers, Statman 2013), via a search listing; the chapters themselves not read.
3. https://www.paultaylor.eu/stable/Proofs+Types (Proofs and Types, free online; chapters), seen via a search listing; also https://logic.amu.edu.pl/images/5/54/Proofsandtypes.pdf (PDF copy, rights status unclear)
4. Abramsky, Computational interpretations of linear logic, TCS 111 (1993) 3-57, via the Yale note https://flint.cs.yale.edu/cs428/notes/L17.pdf and search result; the paper not read.
5. https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/tlca93.pdf and https://www.doi.org/10.1007/BFB0037099 (Benton, Bierman, de Paiva, Hyland, TLCA 1993), via search listing.
6. https://researchportal.hw.ac.uk/en/publications/typability-and-type-checking-in-system-f-are-equivalent-and-undec/ and https://lics.siglog.org/1994/Wells-Typabilityandtypech.html (Wells), via search listing.
7. https://deepblue.lib.umich.edu/handle/2027.42/23534?show=full and https://portal.mardi4nfdi.de/wiki/Publication:1259589 (Statman 1979, abstract); https://www1.cuni.cz/~svejdar/papers/sv_pspin.pdf (Švejdar, semantic proof) via search listing.
8. https://seas.upenn.edu/~sweirich/types/archive/1989/msg00092.html and https://gallium.inria.fr/~fpottier/publis/emlti-final.pdf (ML typability DEXPTIME-complete; Kfoury, Tiuryn, Urzyczyn; Mairson), via search listing; the linear-time remark for STLC inference is in a search-tool summary only.
9. https://unpaywall.org/10.1007%2F3-540-62688-3_47 and https://cis.upenn.edu/~bcpierce/types/archives/1996/msg00387.html (Urzyczyn 1997, abstract)
10. https://core.ac.uk/download/248561704.pdf (Rehof, TYPES 2018, on Löb and a simpler undecidability proof), via search listing.
11. https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/undecidability-of-second-order-linear-logic-without-exponentials/A3A41FD275BEF08BD15331BD32FC89B3 (Lafont 1996); https://lics.siglog.org/1995/LincolnShankarScedr-DecisionProblemsFor.html; https://www.csl.sri.com/papers/lmss90/lmss90.pdf, via search listing.
12. https://github.com/plfa/plfa.github.io
13. https://plfa.github.io/
14. https://github.com/DeepSpec/sf
15. https://github.com/the-little-typer/pie
16. https://github.com/racket/redex
17. https://github.com/hmac/lc
18. https://rick00ne.github.io/LambdaCalculus_TypeChecker (title only fetched; description from search)
19. https://github.com/bntre/visual-lambda
20. https://github.com/thmsmlr/lambda-playground
21. https://tromp.github.io/cl/diagrams.html (via search result; not fetched)
22. https://pages.github.khoury.northeastern.edu/sholtzen/cs4400-spr24/lecture-notes/lec10-stlc and https://ilyasergey.net/CS4212/_static/lectures/PLDI-Week-10-more-typing.pdf (search listing only)

## 7. Not verified

- Statman's and Urzyczyn's papers were not read, only abstracts and secondary descriptions; the Löb 1976 original not seen.
- STLC inference in linear time: not sourced as a theorem. The non-elementary cost of normalising STLC terms is a known result I did not source.
- That Software Foundations' PLF covers STLC and System F, and what TAPL's book page offers (fetch failed): not confirmed.
- PLT Redex's reduction visualiser and typesetting: not read. Licences and last activity of PLFA, Software Foundations, Pie, Redex, hmac/lc and lambda-playground: pages showed no dates; the Redex licence is only "Apache-2.0 and MIT for contributions".
- lambdacalc.io, Lambda Viewer, Gabriel Lebec's visualiser, Lambda Calculus Calculator: not found or not read.
- Benton et al. and Abramsky: bibliographic details only, not the term syntaxes; which calculus suits linlog's `!` and `⊕` is open.
- That the scope note's S-numbered sources (S42 etc.) say what it claims: not re-read.
- Whether step 44's guide or later steps already handle any of this: I read `docs/src/status.md`, not the reports of steps 38 to 45.
- I did not read `.claude/rules/core*.md` in full; section 5's reuse claims are inferences from the status page and `term.rs`.
