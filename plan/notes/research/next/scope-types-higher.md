# Higher types, linear types and sessions, updated for 0.2.0

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

This updates `plan/notes/research/scope/types-higher.md` (2026-10-09, seven candidates, sources S1 to S48). The simply typed λ-calculus and System F are the subject of a sibling note, `plan/notes/research/next/lambda-calculi.md` (candidates lambda-calculi-1 to 7); this note takes the old candidates' fate and the linear, session and graded side. The old note's own sources (S1 to S48) were checked by its author on 2026-10-09; I did not re-read them, and where a claim below leans on one it says "per the old note".

## 1. What linlog has today

- **Terms and binders now exist, but for first-order data only.** `core/src/sequents/term.rs` is the formula arena (`Term`, with the binders `Forall` and `Exists` last), and `core/src/sequents/fo.rs` holds a second arena of first-order terms, `FoTerm` (`Bound(n)`, a de Bruijn index with the innermost binder 0, and `App(Symbol, Args)`), hash-consed and topological, with binder names kept only for printing (`.claude/rules/core-sequents.md`). Atoms are predicates applied to such terms. There is no binder for proof or program variables, no λ-term syntax and no quantifier over propositions.
- **First-order linear logic is built** (step 42, `docs/src/status.md`): quantifiers, `∀` and `∃` steps with eigenvariables, search by unification with the occurs check in the classical modes and in the intuitionistic modes (affine included), certificates for the Rocq library's first-order development. This is what the old candidate 6 (logic programming with terms) asked as a prerequisite.
- **The two-sided intuitionistic surface is built** (step 40): ILL and IMLL are parsed, decided by the two-sided focused engine, the IMLL net embedding, the additive fast path and the Horn engine, drawn two-sided, checked, proved step by step; the recorded sides give a read-back the data it needs.
- **Cut and cut elimination are built** (step 41) on proof terms and on nets, in every mode with a cut. Cut is the composition of processes in candidate 3.
- **The Rocq library** `rocq-linlog` (steps 31 and 45a) states the one-sided calculus with Mix and weakening as parameters, the two-sided intuitionistic calculus and its affine variant, checkers as functions and soundness theorems; certificates are exported for classical, intuitionistic, affine, Mix, cut and first-order proofs.
- **Ordinary logic** (`core/src/ordinary/`): classical, intuitionistic and minimal propositional sequents decided by embedding, with LK or LJ read-back.
- `docs/src/status.md` names none of the old seven candidates. So: none is built (inference from absence), and there is no λ-term printer, no enumeration and no session layer.
- `bench/TOOLS.md` (read 2026-10-15) records that LinearOne prints λ-terms and LaTeX, and that lolli prints a tree or a λ-term. I could not confirm either on the tools' pages today (section 7).

## 2. The field, as it stands now

**Granule.** The repository is BSD-3-Clause, 3,922 commits, described as "a functional programming language with a linear type system" with fine-grained effects and coeffects through graded modal types; the README calls it a research project whose "interface is not stable"; it builds from source with Stack and Z3, has no GitHub release and binary builds only for macOS [1]. The project page lists its newest work: a CSL 2025 paper, "A Mixed Linear and Graded Logic: Proofs, Terms, and Models" (Vollmer, Marshall, Eades, Orchard), and an ICFP 2026 paper, "Same Coeffect, Different Base" (listed for August 2026; I read only the news line) [2]. The CSL paper extends Benton's linear/non-linear decomposition to graded modalities, with a sequent calculus, a natural deduction isomorphic to it, and a categorical model [3]. That is the proof-theoretic statement of the old candidate 5's "graded `!`" (inference). Synthesis from graded types (ESOP 2024) is still the newest synthesis paper the page lists [2]; I could not open the paper (Springer redirected).

**Graded sessions.** Marshall and Orchard's journal paper "Non-linear communication via graded modal session types" appeared in November 2024 [2][4]; it brings sharing back into linear session types with grades.

**Graded dependent types.** Abel, Danielsson and Eriksson, "A Graded Modal Dependent Type Theory with Erasure, Formalized" (arXiv 2603.29716, March to April 2026): Π, Σ, naturals, a universe, grades from a parameterised modality, formalised in Agda with subject reduction, normalisation and decidable conversion, and an erasure extraction proved sound for natural-number programs [5]. This is a checker and a metatheory, not a prover (inference), and confirms the old note's rejection of a graded dependent core.

**Linear Haskell.** The GHC User's Guide (version 9.15.20260306) still gives `LinearTypes` as "Status: Experimental", available since 9.0.1, with multiplicity polymorphism "incomplete and experimental" and multiplicity multiplication not supported; non-top-level, non-recursive `let` and `where` bindings can be linear with `%1` [6]. `linear-base` (Tweag) is MIT, "a standard library for developing applications with linear types", on Hackage and Stackage; the page shows no release date or maturity label [7].

**Idris 2.** The tutorial offers the multiplicities 0, 1 and unrestricted; `duplicate x = (x, x)` is rejected with "There are 2 uses of linear name x"; richer counts "as in Granule" and multiplicity polymorphism are called open questions [8]. The repository page shows 4,065 commits and no licence text or release in what I could read [9].

**Session types.**
- **FreeST**: BSD-3-Clause, "a functional programming language for safe concurrency powered by context-free session types", 790 commits on `dev`, built with Stack; the site links an online editor, a tutorial and publications [10][11]. Context-free session types go past what linear logic gives (inference: they are not propositions of CLL; the pages do not say so).
- **SILL**: an OCaml interpreter, opam package `sill` by Griffith and Pfenning, GPL-3 with the OCaml linking exception, latest listed version 1.4, homepage the ISANobody/sill repository (from a search result; the opam page itself returned 404 for me) [12]; Griffith's thesis reports an OCaml, a Haskell and an Idris implementation [12].
- **Rast** (Das and Pfenning, LMCS 18(1), 2022, DOI 10.46298/lmcs-18(1:9)2022): an open-source implementation of session-typed programs with arithmetic refinements, ergometric and temporal types [13]. I could not find its repository URL (both guessed URLs returned 404).
- **Links**: OCaml, tagged `session-types`, the project site says development resumed in 2014 with a focus on adding session types; I could not read a licence or a release date [14][15].
- Wadler's CP and GV, HCP, Priority CP, multiparty coherence, Ferrite and `sesh`: per the old note (S28 to S35); not re-read.

**Inhabitation and counting.** Espírito Santo, Matthes and Pinto, "Inhabitation in simply typed lambda-calculus through a lambda-calculus for proof search" (MSCS, arXiv 1604.02086): decision and counting procedures from a proof-search calculus [16]; this is the STLC method, the sibling note's ground. For linear terms, Brent Yorgey's 2011 blog posts count linear λ-terms of polymorphic types (Mersenne numbers) and enumerate linear inhabitants [17]. A search for a 2025 tool that counts or enumerates linear inhabitants found none (section 7).

**λ-term printers of proof tools.** Click & coLLecT (LGPL-2.1, 479 commits, "a web interactive tool for building proofs in the sequent calculus of Linear Logic", proof sharing in the URL, LaTeX and Coq export) states no λ-term output on its page [18]; its `bench/TOOLS.md` row has none either, only "LaTeX, PDF, PNG, ASCII, Coq" (`bench/TOOLS.md`). LinearOne (LGPL-2.1, 308 commits, "a prototype theorem prover/parser for first-order (multiplicative, intuitionistic) linear logic", its tree has `sem_utils.pl` and `semantic_terms.pdf`) does not say on the page whether it prints λ-terms [19]; `bench/TOOLS.md` records that it does, read at commit `d9379a5`, and its last activity as 2026-06-11. The reason for LinearOne's term output is the categorial grammar use: linear λ-terms are the meaning of a derivation (inference from the README's mention of type-logical grammars [19]).

## 3. Candidates

Numbering continues the old note. Costs are in sessions, as in the plan.

### types-higher-1: Inhabitation: enumerate, count, decide uniqueness (still open, unchanged)

- *Fate.* Untouched. The prerequisite the old note listed (the engines, the permutations the focused calculus quotients) is the same; step 40 adds the two-sided data the term of each inhabitant needs.
- *Brings.* Research: coherence questions (is this inhabitant unique, which BCK types have one proof, per the old note S40); teaching: why `a ⊸ a ⊸ a` has two proofs, and the classic count of linear inhabitants [17]; AI models: "all programs of this type", with a bound. Evidence of demand is thin: the linear counting I found is a blog post [17]; the STLC side has a paper [16].
- *In linlog's terms.* Decide: built; enumerate and count: new control flow (continue after a proof, share failures but not successes, re-expand "proved" memo entries); `unique` verdict; step by step: the existing interact session with a "next proof" move; export: the list; certify: each inhabitant is a checked, certifiable proof. Under the exponentials it is bounded, not decided (old note S39).
- *Cost.* 2 to 3 sessions, as before; no new trusted base. The notion of "the same inhabitant" is the design question (nets for MLL, the focused calculus's identifications otherwise).
- *Division with the λ-calculi suite.* This is the same problem as lambda-calculi-3. One implementation should serve both: the enumeration front door over the existing engines belongs to the linear side because the engines are linear (the ordinary logic goes through them); the suite adds only the STLC notion of equality (βη-normal forms) as another quotient (inference).
- *Prerequisites.* types-higher-2 for printing the inhabitants as terms.
- *Sources.* [16][17], old note S40.

### types-higher-2: Proofs as programs: term printers (still open; now the first thing to build)

- *Fate.* Untouched, but its premises improved: step 40 gives the recorded two-sided sides, step 41 the cut (a `let`/application redex), step 42 the first-order binders (a term printer must bind at `⊸R` and `∀R`). No printer reads these yet.
- *Brings.* Teaching: the Curry-Howard exercise in a linear setting, which Click & coLLecT's page does not show [18] and LinearOne and lolli are recorded to show only in batch tools (`bench/TOOLS.md`). Research: category-theoretic readings pair terms with proofs (the CSL 2025 paper's natural deduction with terms is the model for the graded case [3]). AI models: a program as the answer to "give me a term of type T", the most direct use of the guide.
- *In linlog's terms.* A read-back pass (ILL proof to a linear λ-term with `λ`, application, pairs, `let` for `⊗L` and `!`, `case` for `⊕L`, `inl` and `inr`, `⟨,⟩`/`fst`/`snd` for `&`), printed in a neutral syntax first and GHC `LinearTypes` and Granule syntax after; shown beside each derivation node; step by step: the partial term beside the goals; cut elimination as β-reduction made visible. The old note's choice stands: two-sided proofs only, a one-sided classical proof has no λ-term.
- *Cost.* 1 to 2 sessions for the read-back and one printer, 1 more for two further syntaxes and the web display. No trusted base if the printer is a display; a claim that the term "is" the proof needs types-higher-8.
- *Risk.* Which calculus for `!` and `⊕` (Abramsky's or Benton et al.'s, as the sibling note says: it cites them without having read them); the Mix and cyclic cases have no term; first-order and exponential rules need a decision on type-level quantifiers.
- *Prerequisites.* None.
- *Division.* The sibling's lambda-calculi-1 is the same item with LJ added. They are one session's work, not two: one read-back module with two sources (ILL proofs, LJ derivations).
- *Sources.* [3][18][19], `bench/TOOLS.md`.

### types-higher-3: Sessions: types, duality, composition, a process (still open; cut now exists)

- *Fate.* Untouched. The prerequisite that was missing in October, cut and cut elimination (step 41), is built, so composition of two endpoints is a cut and its elimination a communication step.
- *Brings.* Teaching propositions as sessions (Wadler, per the old note S28) with a tool that checks duality and shows the communication as cut elimination. Research: the active line is graded sessions [4] and context-free sessions [10], neither of which linlog's calculus can state (inference: CLL has no grades and no recursion on types beyond the exponentials). Tools exist that run programs (SILL [12], Rast [13], FreeST [10], Links [15]); I found none that is a teaching prover of the logic itself (unsourced as a negative: only searched).
- *In linlog's terms.* A `sessions` layer like `ordinary`: a syntax parsed into CLL formulas, `dual` as negation, composition by cut, and a process read off a proof (needs a CP term printer, the classical counterpart of types-higher-2). Decide: provability of a session type; check: a proof; step by step: communication as cut-elimination steps; export: derivation and process; certify: the cut calculus in Rocq.
- *Cost.* 3 to 4 sessions, as before, now with the cut part done; the CP term read-back is the new work. Multiparty coherence stays a separate checker or out of scope.
- *Risk.* Recursion and the infinite behaviours of real session types are outside cyclic MLL as built (inference); Mix corresponds to HCP's hyperenvironments by the old note's (inferred) reading.
- *Prerequisites.* types-higher-2 for the CP side.
- *Division.* Not in the λ-calculi suite's ground (it covers STLC and System F, not processes).
- *Sources.* [4][10][12][13][15], old note S28 to S35.

### types-higher-4: A certificate back end for linear type checkers (still open; the premise weakened)

- *Fate.* The Rocq library and the certificates exist (31, 45a) and the JSON proof import is what the old note meant; no builder API for foreign derivations was announced. The demand side weakened: the checkers named are all still experimental or research-grade (GHC `LinearTypes` experimental [6], Idris 2's linearity admitted incomplete [8], Granule unstable [1]), and none ships a hook for outside certificates, as far as the pages show (inference).
- *Brings.* To a research language author: a trusted check of usage rules. To nobody else; I found no request for it.
- *In linlog's terms.* Check and certify only; no search. The mapping from a language's derivation to an ILL proof is the user's.
- *Cost.* 1 session for a documented builder and wire form; no new trust. Risk: the builder's shape becomes public API.
- *Prerequisites.* A stable proof wire form.
- *Division.* Linear suite only. It would also serve a λ-calculus type checker's output (lambda-calculi-7), which is the same Rocq work seen from the other side: if the suite builds a typing-derivation checker in Rocq, this candidate's usage part is an instance of it for the linear case (inference).
- *Sources.* [1][6][8], old note S18, S22, S23.

### types-higher-5: Graded and sub-exponential modalities (still open; the theory moved)

- *Fate.* Untouched. The CSL 2025 sequent calculus and natural deduction for a mixed linear and graded logic [3] give the proof system a prover would implement; before the paper the old note had only Granule's type-level presentation. Graded logic is also what Idris 2 calls an open question beyond 0 and 1 [8].
- *Brings.* Research: a prover for a graded `!`. Teaching: `A [n]` as at most `n` copies, which a hand expansion loses. AI models: very little.
- *In linlog's terms.* First stage as before: a per-hypothesis copy bound as an engine option (`--copies` exists as a global bound; per-hypothesis is new). Second stage: a new `Kind`, checker rule, Rocq mirror, exports and parser (every layer). Decidability changes with the flags (old note S39).
- *Cost.* 1 session for per-hypothesis bounds, 5 or more for a full graded modality (inference from the old note's list of layers).
- *Risk.* Grades come from a semiring; Granule solves grade equations with Z3 [1], which is not proof search and which linlog would not add (inference).
- *Prerequisites.* None for stage one.
- *Sources.* [1][3][8].

### types-higher-6: Linear logic programming with terms (partly built)

- *Fate.* Partly. Step 42 built the data model this candidate named: first-order terms, symbols, unification with the occurs check and eigenvariable conditions, in the focused and the two-sided engine, so Lolli-style queries over first-order sequents are decided today. Not built: forward chaining with data (Celf, Ceptre style), a Horn engine over terms (the Horn engine is propositional, from `docs/src/status.md`; I did not check whether step 42 extended it), a query syntax with clause databases and answer substitutions.
- *Brings.* As before: planners, protocol modellers; no new evidence of demand found this session.
- *Cost.* 2 to 3 sessions for unification in the Horn engine and answer substitutions; the reachability problem with data has its own bound (inference).
- *Prerequisites.* Built (42).
- *Sources.* Old note S1, S5, S8; `docs/src/status.md`.

### types-higher-7: Relevant mode (untouched)

- *Fate.* Untouched; the cost description of the old note stands (a `Mode` flag, search assumptions broken by contraction without weakening). Nothing found in 2025 or 2026 changes demand. Low priority: a teaching contrast to the affine mode.
- *Cost.* 1 to 2 sessions, bounded not decided.

### types-higher-8 (new): A linear λ-term checker (round trip of types-higher-2)

- *What it brings.* A type checker for the printed linear λ-calculus: a term and a type go in, a derivation (or the first use that breaks linearity) comes out. Teaching: the linear typing exercise, with the error "x is used twice", the message Idris 2 gives [8]. It closes types-higher-2's loop: a printed term is checked by a second implementation, the cheapest differential test of the read-back.
- *In linlog's terms.* The context holds usage counts (a multiset), not a set as in the STLC, so the checker is linear-specific; its derivation is an ILL derivation with terms at the nodes. Decide: linear in the term (Church style, inference); certify: only via the ILL proof it corresponds to.
- *Cost.* 2 sessions (the term parser with binders is the work); adds a second checker to the trusted path.
- *Risk.* Scope creep into a language (let, recursion, data types).
- *Division.* This is the linear case of lambda-calculi-2, which builds the STLC checker with a derivation view. The term syntax with binders, substitution and a typing context as a first-class object is shared and should be built once, by whichever suite comes first, with the usage-count variant as a parameter of the context (inference).
- *Prerequisites.* types-higher-2; a term arena (below).

## 4. Considered and rejected

Unchanged from the old note unless said.
- **An LLF/CLF framework of linlog's own**: higher-order terms, HOAS and unification stay out; step 42 took the first-order part only.
- **A QTT or graded dependent core**: the 2026 Agda formalisation [5] confirms that the substance is normalisation and conversion; a checker, not a prover.
- **Rust borrow checking, Move's verifier, separation logic, multiparty global types, amortised bounds, full Forum**: as in the old note.
- **Granule's grade constraints by SMT** [1]: not proof search, and a Z3 dependency against the wasm goal (inference).
- **Context-free session types** [10]: not linear logic; they go beyond CLL.
- **A competitor to Granule, Idris 2 or Linear Haskell**: all three are programming languages with compilers; a prover of their usage rules is candidate 4 only.

## 5. Ranking for research, teaching and AI models (inference)

1. **types-higher-2**, with the sibling's lambda-calculi-1 as one item: cheapest (1 to 2 sessions), no new trust, uses only what is built, and the one candidate all three audiences use: teaching (Curry-Howard exercise), research (proof/term pairs for a given calculus), AI models (a program as an answer).
2. **types-higher-1** (with lambda-calculi-3): the natural payoff of 2.
3. **types-higher-8**: small, and what makes 2 checkable.
4. **types-higher-3** (sessions): the strongest teaching story beyond 2, now with cut built, but 3 to 4 sessions and a CP read-back.
5. **types-higher-6** (the unbuilt part): useful to practitioners, no fresh evidence.
6. **types-higher-5** (graded): research interest is real (CSL 2025 [3]), but the work touches every layer.
7. **types-higher-4** and **types-higher-7**: lowest; no demand found.

## 6. How this and the λ-calculi suite divide the ground

- **Linear suite (this note, and linlog's core)**: read-back of ILL proofs as linear λ-terms and their printers (types-higher-2); enumeration and counting by the existing engines (1); the linear checker with usage counts (8); sessions, graded and relevant modes (3, 5, 7); the certificate back end (4).
- **λ-calculi suite (sibling note)**: STLC and System F as calculi with their own terms and checkers (lambda-calculi-2, 6), type inference by unification (4), the β-stepper (5), a Rocq checker for typing derivations (7). The STLC term read back from LJ is a second source for the same read-back module.
- **Shared and built once** (inference): (a) a term syntax with binders, substitution and a typing context, de Bruijn or locally nameless as `fo.rs` already does for first-order terms, but separate from them (a proof variable is not a first-order variable); (b) a derivation type labelled with terms, whose node view the existing derivation drawing can take; (c) normalisation, which is β-reduction for the suite and cut elimination for linlog: show both together, since step 41's cut elimination is the proof side; (d) the enumeration front door. The usage count context is the only part specific to the linear case.
- **Rule of thumb**: if the feature is stated on an ILL or LJ proof, it belongs to the linear suite; if it is stated on a term that exists without a proof (typing a given term, inferring, reducing), it belongs to the λ-calculi suite.

## 7. What the structure must allow

- Reusable: the `Term`/`FoTerm` arena technique (topological, hash-consed, indices); the two-sided derivation view and its exports (LaTeX, Typst, SVG); the interactive session; the engines as inhabitation oracles; cut elimination; the Rocq exporter and library; JSON wire forms.
- Linear-specific: occurrence sets, multiset-keyed memo and counts, the focused calculus's identification of proofs (so what "same inhabitant" means for linear terms differs from βη for the STLC).
- New: a proof-variable binder and a term arena of its own; a derivation type that carries a term at each node; a "next proof" control in the search (enumeration) and in `interact`; a CP read-back for sessions.

## 8. Sources (read 2026-11-08)

1. https://github.com/granule-project/granule
2. https://granule-project.github.io/
3. https://www.arxiv.org/abs/2401.17199 (found by search; abstract summary only; also https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2025.32)
4. https://www.sciencedirect.com/science/article/pii/S0890540124000993 (Marshall, Orchard; search result only) and https://arxiv.org/html/2203.12875
5. https://arxiv.org/abs/2603.29716
6. https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/linear_types.html
7. https://github.com/tweag/linear-base
8. https://idris2.readthedocs.io/en/latest/tutorial/multiplicities.html
9. https://github.com/idris-lang/Idris2
10. https://github.com/freest-lang/freest
11. https://freest-lang.github.io/
12. https://opam.ocaml.org/packages/sill (404 on fetch; content from a search result) and https://www.ideals.illinois.edu/items/92895 (Griffith thesis, search result)
13. https://lmcs.episciences.org/8954 (search result; abstract text only)
14. https://github.com/links-lang/links
15. https://links-lang.org/
16. https://arxiv.org/abs/1604.02086
17. https://byorgey.github.io/blog/posts/2011/01/26/counting-linear-lambda-terms.html (search result)
18. https://github.com/ComputerAidedLL/click-and-collect
19. https://github.com/RichardMoot/LinearOne
- The old note's sources S1 to S48 (`plan/notes/research/scope/types-higher.md`), not re-read.
- The sibling note `plan/notes/research/next/lambda-calculi.md`, read in the workspace.

## 9. Not verified

- Whether LinearOne, lolli or Click & coLLecT print λ-terms: the pages do not say; `bench/TOOLS.md` says LinearOne and lolli do (read 2026-10-15).
- Last-activity dates of Granule, FreeST, Idris 2, linear-base and Links: the fetched pages showed none. Licences of Idris 2 and Links: not shown.
- The Rast repository's URL, and SILL's current maintenance: not found. SILL's opam page returned 404; its licence and version come from a search snippet.
- The ESOP 2024 synthesis paper and the ICFP 2026 paper: not opened; only the project page's titles.
- Contextual metaprogramming for session types (a January 2026 follow-up a search summary mentioned) and TYPES 2025 abstracts on linear types in dependent type theory: seen only in a secondary listing, left out.
- Whether step 42 extended the Horn engine to terms: not checked in the reports.
- Any 2025 or 2026 tool that enumerates or counts linear inhabitants: a search found none (a negative from one search).
- That no linear teaching prover covers sessions: only searched, not exhaustively.
- Nothing was run.
