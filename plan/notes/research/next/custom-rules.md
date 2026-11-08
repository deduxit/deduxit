# Custom derivation rules: a rule language, a checker and an interactive builder

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

The author's words (2026-11-08): "maybe a way to add custom derivation rules and a way to interactively build derivations with them". This note settles what that can mean, what can be checked, searched and only stepped through under arbitrary user rules, what the field offers, and what it would ask of linlog's core.

## 1. What linlog has today

- **Fixed calculi, a closed rule enum.** `Rule` (`core/src/proofs/rule.rs`) is the one-sided standard calculus of linear logic plus Mix, affine weakening, cut, the quantifier rules and `Open`; its doc comment says "The type is closed: a new rule is a new release of the calculus." The ILL rules have no tags of their own: on the lowered one-sided sequent every one is a classical rule (`.claude/rules/core-proofs.md`). Cyclic MLL and Lambek are the same machinery with no exchange.
- **The checker is one pass over occurrence-based terms.** A `Proof` is an arena of 16-byte `Node`s over subformula occurrence ids of its own `Forest`; a node never records its sequent, the checker derives it in one pass and in memory proportional to the proof (`.claude/rules/core-proofs.md`, `docs/src/status.md`). Speed and a small trusted base are the point of this design; a rule's shape is compiled into the checker's code, not data.
- **Interactive proving applies the fixed rules.** `proofs/interactive/` holds a derivation with open goals as `Rule::Open` leaves; `rules(goal, position)` lists the `Applicable` rules, `apply` takes a `Step` (position, rule, split), `undo`, `close` (the search closes a goal), `invert`; the command is `linlog interact` (`docs/src/interactive.md`, `.claude/rules/core-derivations.md`, "Interactive proving").
- **Rule schemas exist only as content.** The web page's Learn lessons carry each rule as a front-matter entry (`premises`, `conclusion`, `twoSided`, `says`) in `linlog-prover.github.io/src/content/rules/*.md` (for instance `03-tensor.md`: premises `["⊢ Γ, A", "⊢ Δ, B"]`, conclusion `"⊢ Γ, Δ, A ⊗ B"`). These are strings for display, not data a program reads.
- **The nearest thing in the code is the inverse method's "rule as a term with holes"** (`.claude/rules/core-inverse.md`): a derived rule is the proof term of one focusing phase (`Step`, postorder, ending in `Hole`s), compiled from the forest, evaluated forward over premises by one function (`derive`), with subsumption of kept sequents. It is generated from the fixed calculus, with `take` and `join` mirroring the checker; it is not a user-facing rule language.
- **User theories as hypotheses are possible.** Horn programs with clauses under `!` are decided by the Horn engine (`docs/src/status.md`), so a Petri net or multiset-rewriting system is already a "custom rule set" at the object level: the rules are hypotheses of a sequent, not rules of the calculus.
- **A scope note rejected one form already.** `plan/notes/research/scope/education.md` (section 3) rejected "User-defined logics, as the Incredible Proof Machine's logic files", because "linlog's checker is the trusted base for a fixed calculus (D6); a rule restriction (education-4) gives the progression without that." This note reopens that decision only because the author has now asked for the feature by name; the reasoning (trusted base) is addressed in the candidates.
- **No other tool in `bench/TOOLS.md` offers custom rules**; all ten are for fixed linear logic fragments.

Nothing below re-proposes any of this. The proposals are about a second, data-driven rule layer.

## 2. The field

### 2.1 Theory: what a rule schema is, and what is decidable

A sequent-calculus rule schema is a set of premise sequents and one conclusion, over schematic variables of three kinds: formula variables (`A`, `B`), context variables (`Γ`, `Δ`, standing for multisets or sequences of formulas) and, with quantifiers, term variables with freshness conditions. Side conditions that matter in linear logic: promotion's context must consist only of `?` formulas (`!Γ`-style restrictions), eigenvariable freshness, modal rules that restrict the context to boxed formulas, and the structural discipline (exchange, weakening, contraction) of the calculus. The sources below give these ingredients in tools; the general statements that follow are mine.

- **Checking a derivation by matching** is the easy case when each rule application names where it applies. Under a rule with a single principal formula the match is first-order matching of a formula against a pattern plus a split of the remaining multiset among the premises' context variables; the split is data in the derivation (linlog's own `Step` carries a `Split` today). (inference)
- **Matching without that information is hard.** Associative-commutative matching is NP-complete (Benanav, Kapur and Narendran 1987, "Complexity of Matching Problems", J. Symbolic Computation 3:203-216; abstract also states that associative and commutative matching alone are NP-complete) [S1]. It stays NP-complete when each pattern variable occurs at most twice, and linear patterns are polynomial (same search result, from abstracts and mirrors, not the primary text) [S1]. So a checker that must find the principal formula and the split by itself, for arbitrary patterns, is a search problem; a checker given them is linear. (inference)
- **Searching with user rules is undecidable in general and need not terminate.** The properties that would make a rule set safe to search with (admissibility, invertibility, cut elimination, identity expansion) are "generally undecidable", but semi-decision procedures exist: Olarte, Pimentel and Rocha's L-Framework proves them by rewriting and narrowing in Maude for propositional systems, applied to intuitionistic and classical logic, classical and intuitionistic linear logic, a dyadic system and normal modal logics (arXiv:2101.03113) [S2].
- **Focusing can be synthesised for well-behaved rule sets.** Marin, Miller, Pimentel and Volpe (Annals of Pure and Applied Logic 173(5), 2022, DOI 10.1016/j.apal.2022.103091) turn theories of bipolar formulas into synthetic inference rules using the focused systems LJF and LKF, with polarity assignment on atoms and some connectives deciding which rules come out; they apply it to labelled sequent systems of several propositional modal logics [S3][S4]. The paper's own introduction explains the point: focusing structures the choices that "are in desperate need of being structured somehow" (from the PDF, read) [S4].
- **Soundness is the user's responsibility.** A tool that accepts any rule set checks only that a derivation follows the rules given; it says nothing about whether the rules are sound for any semantics. In the Incredible Proof Machine the formalised result is that proof graphs correspond to natural-deduction trees of the logic given, in Isabelle's AFP; no claim is made about the user's logic being sound [S5]. Linlog could give a certificate only if the user supplied a semantics, which is out of scope for a rule language. (inference)
- **Linear logic as the framework.** Cervesato and Pfenning's LLF (LICS 1996; Information and Computation 2002, DOI 10.1006/inco.2001.2951) is a linear type theory that conservatively extends LF, built to encode stateful deductive systems; its examples include a sequent calculus for classical linear logic with its cut elimination theorem [S6]. This is the field's existing answer to "describe a calculus in linear logic itself". (Celf's design was not read; unsourced.)

### 2.2 Tools

Each line: what it is, licence, last activity as shown, use. Where a page did not show a date or a licence, it says so.

- **Twelf / LF** [S7][S8]. LF is the Edinburgh Logical Framework, a typed lambda-calculus with dependent types for defining logics (Harper, Honsell, Plotkin, "A framework for defining logics", J. ACM 40(1):143-184, 1993, DOI 10.1145/138027.138060) [S9]. Twelf implements LF, the Elf constraint logic programming language and a meta-theorem prover (README, copyright Pfenning and Schuermann 1997-2011); the Twelf site says it is "no longer under active development" but has a user community and is used in research; large projects include TALT and a type-safety proof for Standard ML [S7][S8]. The pages read did not state the licence or a release date.
- **Isabelle's meta-logic Pure.** Isabelle is a "generic proof assistant", Isabelle2026 of October 2026, BSD-style main licence [S10]. Paulson's "The foundation of a generic theorem prover" (J. Automated Reasoning 5:363-397, 1989; arXiv cs/9301105) introduces a higher-order meta-logic with Huet's higher-order unification, in which object logics are rule sets [S11]. Isabelle's distribution also ships a `Sequents` library for sequent calculi, including LK, ILL and modal logics, as object logics [S12].
- **ELPI / λProlog** [S13]. ELPI is an embeddable λProlog interpreter with Constraint Handling Rules, in OCaml, LGPL-2.1 or later, aimed at the elaborators of proof assistants; the README names Coq-elpi as its largest embedding. Page shows 3,395 commits and no date.
- **Abella** [S14]. A prover for reasoning about specifications in a hereditary Harrop subset of λProlog (executable, Teyjus an implementation) with lambda-tree syntax; version 2.0.8 of 31 October 2023; among its example applications, cut-admissibility for a sequent calculus. Licence not stated on the page read.
- **Dedukti / Lambdapi** [S15][S16]. Dedukti implements the λΠ-calculus modulo rewriting, a framework where a logic is a signature plus rewrite rules; when the rewrite system is confluent and strongly normalising, conversion and type-checking are decidable [S15]. Lambdapi (Deducteam/lambdapi on GitHub, 4,113 commits, 411 stars) is a proof assistant on the same calculus; the page does not show licence, release or date [S16].
- **Sequoia** [S17][S18]. "Sequoia: A Playground for Logicians (System Description)", IJCAR 2020, pp. 480-488, by Giselle Reis, Zan Naeem and Mohammed Hashim (the PMC record; Elaine Pimentel and Vivek Nigam are not among its authors): a web application, hosted at logic.qatar.cmu.edu/sequoia, in which the user specifies a sequent calculus (several contexts on the left and right, each rule acting on one connective), builds ground and schematic proof trees, and checks weakening admissibility, identity expansion and permutability. Cut admissibility and invertibility are not listed among its checks (a snippet of the paper; the paper itself was behind a login redirect, so this is from a search summary). Repository `meta-logic/sequoia-v0`, GPL-3.0, 697 commits, no date shown; setup needs Node.js, MongoDB, Standard ML of New Jersey and Python 3, i.e. a server [S17].
- **L-Framework** [S2]. Specification language and semi-decision procedures in Maude (see 2.1); a research tool, not a teaching page. Licence and date not read.
- **MetTeL2** [S19]. A generator of tableau provers: the user gives the syntax and the tableau rules, with rule priorities, and gets Java code that decides satisfiability and outputs models; JELIA 2012 (LNCS 7519, pp. 492-495), usable through a web interface. Licence and date not read.
- **LoTREC** [S20]. A Java tool in which the user writes a tableau system in "a simple, graph-based, interactive language"; accompanies the book *Kripke's Worlds* (Gasquet, Herzig, Said, Schwarzentruber; Birkhauser 2014) and was used in an ESSLLI 2010 course and by lecturers to compare K4, S4 and S5. Licence and date not read.
- **Jape** [S21][S22]. "A configurable proof calculator that supports the interactive discovery of formal proofs in inference systems"; ships natural deduction, several sequent calculi, Hindley-Milner typing and syllogisms; a manual "Roll your own Jape logic" explains how to encode a new logic. GPL-2.0; 3,612 commits; the page carries a note of 15 October 2026 by Bernard Sufrin that he will keep supporting users, and says Richard Bornat died earlier in the year and that his students are building a successor. Jape makes a proof move "if and only if" the loaded logic justifies it [S22]. Bornat's book *Proof and Disproof in Formal Logic* (Oxford) is the course text (title from a search result).
- **Carnap** [S23][S24]. Haskell framework for teaching logic in the browser; checking runs in the student's browser; more than fifty formal systems; a rule-builder widget lets students save derived rules; the 2026 paper "Carnap Ten Years Later" (arXiv 2607.07722) reports use by over 45,000 students over a decade; each system is added by the maintainers, not described by the instructor in a data file (Daily Nous snippet) [S23][S24]. Proof checking uses Huet-style unification (conference slides) [S25]. Licence not read.
- **The Incredible Proof Machine** [S5][S26][S27]. A browser prover where a proof is a graph of rule blocks (Breitner, LFMTP 2016 invited talk, DOI 10.1145/2966268.2966273; ITP 2016). Logics are YAML files in a `logics` directory: a rule has a name, "ports" (a conclusion, assumptions, and local hypotheses with the input they may be used for) and a list of schematic variables that "may unify with any proposition"; a `sessions.yaml` lists tasks (assumptions and conclusions) and which rules each session shows; `make` turns the YAML into JSON; the page is static files, so it needs no server [S5]. The logic core is Haskell compiled with GHCJS, with Nipkow's functional higher-order unification [S5]. Repository `nomeata/incredible`, MIT, 708 commits, no date shown [S26]. The meta-theory (proof graphs against natural deduction trees) is in Isabelle's AFP [S5][S27]. The talk says the result has user-defined logics, Hilbert-style proofs and typing derivations (search summary).
- **Metamath** [S28]. The extreme case: no logic built in, axioms and theorems in text files, a verifier of about 300 lines (a 2014 talk by Megill), substitution the only rule; many independent verifiers check set.mm. The book is in the public domain; Norman Megill died in 2021 [S28].
- **Rust crates.** One crates.io/docs.rs search found no crate that is a general sequent-rule framework: `tensorlogic-ir` has a fixed sequent module with a proof-tree validity check, `lolli-core` a fixed `Rule` enum of linear logic, `pumpkin-checking` an inference-checker trait from a constraint solver's context [S29] (search-snippet level only; their docs were not opened).
- **Not found:** a "Calculus Toolbox" by Ciabattoni or others (search returned only papers by Ciabattoni on hypersequents and rule systems, none a tool) [S30]; a TINC project page (search found nothing). Both are left out as unsourced.

### 2.3 What the tools share (inference from 2.2)

Every tool that lets the user add rules keeps the rule as data (YAML in the Proof Machine, a theory file in Jape, a specification in Sequoia, MetTeL2 and LoTREC), runs a generic unification or matching routine against it, and has the user carry soundness. The tools aimed at teaching (Proof Machine, Jape, Carnap) check derivations and step through them; none of them promises to search with arbitrary rules, and the ones that search (MetTeL2, LoTREC) do so for tableaux with termination left to the specification. The research tools (Sequoia, L-Framework) analyse the rule set rather than proving in it.

## 3. Candidates

Costs are in sessions as the plan counts them, my estimates (inference). Every candidate shares one decision, stated first.

**The kernel decision (inference).** Two designs serve custom rules.
- *Kernel A, generic kernel with the fixed calculi as instances:* rewrite the checker so that the standard rules are rule schemas interpreted by a generic matcher. It would make one trusted base, but the checker's speed rests on rules being compiled code over occurrence ids in one pass and on the dyadic `Θ; Γ` encoding with counts (`core-proofs.md`), the search engines, the net engine and the Rocq certificates (checked against the same rules in `rocq-linlog`) all read that representation. The cost is a rewrite of the crate's most tested part and a performance loss on the ratchet's journeys, against a gain only the custom-rules user sees. Not recommended.
- *Kernel B, a second, generic derivation type beside the fast checker:* a `rules` module (own feature, default off like `png`) with a schema type, a derivation type over formula trees and sequents of multisets or sequences, and a checker by matching. The fixed calculi stay as they are. The generic checker is slower (matching, not an index pass) and is a second trusted base, but it touches nothing that exists; and linlog's calculi can later be written as data in it and cross-checked against the fast checker, which is a test of the generic one, not a dependency. Recommended (inference).

### custom-rules-1: A rule language and a checker for derivations under user rules

*What it brings and to whom.* Teaching and research: an instructor defines a calculus (a sequent system for a modal logic, a variant of LL with another exponential, Gentzen's system with a changed rule) and students check derivations; the Proof Machine's YAML logic files and Jape's theory files are the evidence that teachers do this [S5][S21]. For the AI models using `skills/linlog/` it gives a way to test a rule set stated by a user, but only as far as the user's rules go.
*What a suite offers.* A calculus file (JSON first, since linlog already writes JSON everywhere; a text form with the parser's style second) holding: connectives with arity and print forms, structural discipline per sequent side (multiset, sequence, with or without weakening and contraction), and rules as premises, conclusion and side conditions over `A`, `B`, `Γ`, `Δ`, with side conditions limited to a fixed menu: context members restricted to a connective's formulas (`?Γ`), freshness of an eigenvariable, and no others. Checkable: a derivation whose steps name the principal formula and the context split (as `Step` does now), by first-order matching plus multiset or sequence splitting, in time linear in the derivation (inference, see 2.1). Output: the same derivation printing and exports (LaTeX, Typst, SVG) fed by a generic `Inference`, so the existing writers reuse; no Rocq certificate. Decide and prove: not in this candidate.
*Cost.* Two to three sessions: schema type and file format with a manual chapter, the matcher and checker with a differential test against the fixed calculus (write LL's rules as a file and compare verdicts with the fast checker on the benchmark families' proofs), the exports. The trusted base grows by one generic checker that the user's rules sit under, flagged in output as "valid in the calculus given", never "valid" unqualified.
*Risk.* Rule sets that are incoherent (no cut elimination) pass the checker, which is correct but may mislead a student; the output wording matters. Context restrictions beyond the menu (modal side conditions, labelled systems) are not expressible; each addition is a design decision. Soundness is the user's.
*Prerequisites.* None in core; the derivation printing's `Inference` is private with `pub(crate)` fields (`core-derivations.md`), so a generic one means a trait or a second type (inference, to be settled in the design session).
*Sources.* [S1][S5][S9][S21][S23].

### custom-rules-2: Interactive building of derivations under user rules, in the command and the web stage

*What it brings and to whom.* The author's second half: "interactively build derivations with them". The Proof Machine, Jape, Carnap and Sequoia all give exactly this, with the rule set as data [S5][S17][S21][S23]. For teaching it is the main use; for research a playground for a new calculus (Sequoia's own stated aim) [S17].
*What a suite offers.* A session state like `Interactive` over the generic derivation: open goals, `rules` listing the rules whose conclusion matches the selected formula or sequent (matching is the "applicable" test, with the split asked of the user as `Split` is today), `apply`, `undo`, `show`, `save` and `load` as JSON including the calculus, `proof` re-checking with candidate 1's checker. In the web stage a calculus is a tab setting, loaded from a file or pasted JSON; the page already has a stage with tabs and rule schemas displayed, so a calculus file drives the same panels. Stepped only: no search, no `close`, no `invert` (an invertibility flag would be asserted by the user, not derived: see custom-rules-4).
*Cost.* Two to three sessions after candidate 1: the session type, command `linlog interact --calculus FILE`, bindings in `web/` (JSON in and out, nothing kept between calls: `web.md`'s rule), the stage's tab and a calculus editor in the client (a client-repository step). The trusted base adds only what candidate 1 added.
*Risk.* Without search, a student with no rule at hand is stuck; a "show all matches" listing mitigates it. The client work is the larger half and sits in another repository.
*Prerequisites.* custom-rules-1.
*Sources.* [S5][S17][S21][S23].

### custom-rules-3: Bounded backward search with user rules

*What it brings and to whom.* Research and teaching: "is there a derivation in my new calculus within depth n" turns a stepper into a tool for testing a design. MetTeL2 and LoTREC search with specified rules; the Proof Machine, Jape and Carnap do not [S19][S20].
*What a suite offers.* Backward proof search by unifying a goal's formulas with rule conclusions, with the user's rule priorities (as MetTeL2 allows), a depth or node limit through the existing `Limits`, a loop check by memo of sequents, and the verdict "found" with a derivation, or "unknown" with the reason (depth reached). Never "unprovable", since the search is incomplete for arbitrary rules and termination is not guaranteed (2.1). Searchable only in this bounded sense; the checker of candidate 1 re-checks every result, as linlog's engines do today.
*Cost.* Two to four sessions: the multiset matching with a choice of the principal formula (NP-complete in general, 2.1, so a work bound is a must), the memo, the limits, the tests; efficiency work is open-ended.
*Risk.* Search quality for unfamiliar rule sets is poor without the focusing the fixed engines rely on; users may expect it to match the speed of the focused engine, which it will not. Anything more than a bounded verdict would be a misleading claim.
*Prerequisites.* custom-rules-1. Linlog's `Limits`, `stop` and `Progress` conventions (`core.md`) carry over.
*Sources.* [S1][S2][S19][S20].

### custom-rules-4: Static analysis of a rule set (invertibility, identity expansion, cut admissibility as bounded checks)

*What it brings and to whom.* Research: Sequoia checks weakening admissibility, identity expansion and permutability, and the L-Framework adds invertibility, admissibility and cut elimination by semi-decision procedures [S2][S17][S18]. For linlog's own use, an invertibility test would let candidate 3's search apply invertible rules eagerly and candidate 2's `invert` work.
*What a suite offers.* A command `linlog calculus check FILE` that reports, per rule, permutability with every other rule (a finite check on schematic instances), whether the rule is invertible (the schematic reverse derivation within a bound), and whether identity expansion holds for each connective; each answer is yes, no with a witness, or unknown.
*Cost.* Three or more sessions, most of it research: these procedures are the content of papers [S2]; the L-Framework's algorithms are implemented by rewriting in Maude with narrowing, which has no equivalent in linlog (inference). A bounded, schematic version (permutation of two rules on the schematic conclusion) is the smaller subset and could be one session.
*Risk.* High. The properties are undecidable in general; the procedures are semi-decision; an unknown on most inputs is likely for realistic calculi.
*Prerequisites.* custom-rules-1; candidate 3 if invertibility is to feed a search.
*Sources.* [S2][S17][S18].

### custom-rules-5: Synthetic rules from axioms (a front end for modal and other axiom-defined logics)

*What it brings and to whom.* Research, and the author's modal-logic wish: given a logic by axioms (for example the geometric or bipolar axioms of a modal logic's frame conditions), generate its sequent rules by the focused method rather than the user writing them. Marin, Miller, Pimentel and Volpe show how bipolar theories give synthetic rules and apply it to labelled sequent systems of several propositional modal logics [S3][S4].
*What a suite offers.* A function from a bipolar axiom and a polarity assignment to a rule in candidate 1's schema type, with the choice of polarity exposed (the paper's point is that different assignments give different rules for one formula) [S4]. Checkable and steppable as candidates 1 and 2; searchable as candidate 3.
*Cost.* Two sessions after candidate 1, plus the modal-logic area's own decisions (labelled sequents need labels as a term sort, which candidate 1's schema must then allow); this note leaves modal logic to its area's note (inference).
*Risk.* Limited to the formula class the paper treats (bipolars); many axioms fall outside it.
*Prerequisites.* custom-rules-1, with labels as a sort.
*Sources.* [S3][S4].

### custom-rules-6: Theories as hypotheses (documentation and a guide chapter, no new engine)

*What it brings and to whom.* Users and the AI models that use the guide: much of what "custom rules" means to users of a linear logic prover is a resource system (a Petri net, a game, a protocol) whose rules are linear implications, the LLF and Ceptre-style use [S6]. Linlog's Horn engine and the full-LL engines already decide these sequents; what is missing is a chapter that says so, shows how to write rules as hypotheses (`!(a ⊗ b ⊸ c)`) and read the proof as a derivation in that system.
*What a suite offers.* Manual chapter and guide section with real invocations (the manual's examples are tests), a few examples; decide, check, step through, export, certify all work as today, since the rules are formulas.
*Cost.* One short session; no addition to the trusted base.
*Risk.* None beyond overclaiming: this is a theory of the sequent, not a new calculus.
*Prerequisites.* None.
*Sources.* [S6]; linlog's own `docs/src/status.md` for the Horn engine.

### Ranking (inference)

custom-rules-6 first (small, fits today). Then custom-rules-1 and -2 together as one step pair: they are the author's request, and they make linlog the one tool with a fast fixed core and a data-driven second layer, which no tool read here combines. custom-rules-3 only if users ask for search; custom-rules-4 and -5 are research and depend on the author's interest in modal logic and calculus analysis.

## 4. Considered and rejected

- **A generic kernel as the only checker (Kernel A).** Rewrites the trusted, optimised, Rocq-linked core for a feature that adds nothing to the fixed calculi; see the kernel decision.
- **A logical framework in the LF or Dedukti sense (dependent types, rewriting).** A different, much larger project, with mature tools (Twelf, Dedukti, Lambdapi, Isabelle Pure) [S7][S9][S10][S15]; linlog's gain would be a worse copy. Export of a calculus to one of them is possible later (inference) but not proposed here.
- **User-defined logics as unrestricted trusted checking with a certificate.** No semantics is given by the user, so no certificate can be issued; the output must say "valid in the calculus given".
- **Graphical block proofs in the Proof Machine's style.** Already rejected in `plan/notes/research/scope/education.md` as a third representation no linlog plan or export reads.
- **A hosted service with accounts for calculi (Sequoia's architecture: Node.js, MongoDB, SML)** [S17]. Conflicts with the web page's no-server goal; a file or pasted JSON covers sharing.
- **A Rust crate dependency for the rule framework.** The one search found none that fits (S29, snippet level); nothing to adopt.
- **Automatic search with a completeness claim for arbitrary rules.** Undecidable in general (2.1).

## 5. What the structure must allow

Reusable for custom rules (judged from `.claude/rules/core.md` and the module rules):
- the formula and sequent syntax and the parser's style, the JSON conventions, `Limits`, `stop` and `Progress` for long calls, and the error family with its `describe`;
- the derivation printing, its LaTeX, Typst and SVG writers (once their input is a trait or a generic inference type), the interactive session's command grammar, and the web bindings' pattern of JSON in and out with nothing kept between calls;
- the manual-as-test mechanism (`cli/tests/readme.rs`) for a new chapter and the guide.

Linear-specific, not reusable as they are:
- the occurrence-id proof terms, the dyadic `Θ; Γ` encoding, counts and bitset sequents of the focused engines, the net and Horn engines, the inverse method's compiled rules, and the Rocq certificates. A generic kernel (Kernel B) therefore shares nothing with them but the syntax and the output layer, which is the reason it can sit beside them without a cost to their speed.
- Leave room now: a derivation type whose `Inference` is not tied to the `Rule` enum, or a trait the writers read, so the generic checker's derivations print through the same code. (inference)

## 6. Sources

All read 2026-11-08. "Search result" means the claim comes from a search summary and the primary page was not opened or was not readable.

- [S1] Benanav, Kapur, Narendran, "Complexity of Matching Problems", J. Symbolic Computation 3 (1987) 203-216. Search results and mirrors: https://www.bohrium.com/paper-details/complexity-of-matching-problems/811926861788479489-885 ; https://uio.no/studier/emner/matnat/ifi/nedlagte-emner/INF5160/v10/undervisningsmateriale/kapur.narendran87Matching_unification_and_complexity.pdf
- [S2] Olarte, Pimentel, Rocha, "A Rewriting Logic Approach to Specification, Proof-search, and Meta-proofs in Sequent Systems", arXiv:2101.03113 (2021). https://arxiv.org/abs/2101.03113
- [S3] Marin, Miller, Pimentel, Volpe, "From axioms to synthetic inference rules via focusing", APAL 173(5), 2022, DOI 10.1016/j.apal.2022.103091 (search result). https://figshare.le.ac.uk/articles/journal_contribution/From_axioms_to_synthetic_inference_rules_via_focusing/21708164
- [S4] The same paper, preprint, abstract and introduction read. https://www.lix.polytechnique.fr/~dale/papers/synthetic-rules-via-focusing.pdf
- [S5] Breitner, "The Incredible Proof Machine", LFMTP 2016 talk paper, DOI 10.1145/2966268.2966273, text read. https://www.joachim-breitner.de/publications/Incredible_LFMTP_2016-06-23.pdf
- [S6] Cervesato, Pfenning, "A Linear Logical Framework", LICS 1996 (search result: abstract page, journal version DOI 10.1006/inco.2001.2951). https://lics.siglog.org/archive/1996/CervesatoPfenning-ALinearLogicalFrame.html
- [S7] Twelf. https://twelf.org/
- [S8] Twelf repository README. https://github.com/standardml/twelf
- [S9] Harper, Honsell, Plotkin, "A framework for defining logics", J. ACM 40(1) 1993, DOI 10.1145/138027.138060 (search result). https://twelf.org/wiki/template-bibtex-hhp93lf/
- [S10] Isabelle. https://isabelle.in.tum.de/
- [S11] Paulson, "The foundation of a generic theorem prover", J. Autom. Reasoning 5 (1989), arXiv:cs/9301105 (search result). https://arxiv.org/abs/cs/9301105
- [S12] Isabelle `Sequents` library README (search result). https://isabelle.in.tum.de/website-Isabelle2011/dist/library/Sequents/README.html
- [S13] ELPI. https://github.com/LPCIC/elpi
- [S14] Abella. https://abella-prover.org/
- [S15] Assaf and Burel, "Translating HOL to Dedukti" and Burel et al., "Checking Zenon Modulo Proofs in Dedukti" (search result). https://arxiv.org/pdf/1507.08719
- [S16] Lambdapi. https://github.com/Deducteam/lambdapi
- [S17] Sequoia repository. https://github.com/meta-logic/sequoia
- [S18] Reis, Naeem, Hashim, "Sequoia: A Playground for Logicians (System Description)", IJCAR 2020 (search result; the Springer page redirected to a login). https://link.springer.com/10.1007/978-3-030-51054-1_32
- [S19] Tishkovsky, Schmidt, Khodadadi, "The Tableau Prover Generator MetTeL2", JELIA 2012 (search result). https://www.cs.man.ac.uk/~schmidt/publications/TishkovskySchmidtKhodadadi12b.html
- [S20] LoTREC and *Kripke's Worlds* (search result). https://www.irit.fr/~Andreas.Herzig/Cours/CTableaux
- [S21] Jape repository. https://github.com/RBornat/jape
- [S22] Jape (software) and manuals (search result). https://en.wikipedia.org/wiki/Jape_(software)
- [S23] "Carnap Ten Years Later: Lessons Learned and Next Steps", arXiv:2607.07722 (search result). https://arxiv.org/abs/2607.07722
- [S24] Leach-Krouse, "Carnap: An Open Framework for Formal Reasoning in the Browser" (search result). https://philarchive.org/archive/LEACAO-2
- [S25] Carnap conference slides (search result). https://gleachkr.github.io/CADEslides
- [S26] Incredible Proof Machine repository. https://github.com/nomeata/incredible
- [S27] Incredible Proof Machine site. https://incredible.pm/
- [S28] Metamath (search result). https://en.wikipedia.org/wiki/Metamath ; https://us.metamath.org/downloads/metamath.pdf
- [S29] docs.rs pages for `tensorlogic-ir`, `lolli-core`, `pumpkin-checking` (search result only). https://docs.rs/tensorlogic-ir/latest/tensorlogic_ir/sequent/index.html
- [S30] Ciabattoni and Genco, rule systems and hypersequents, arXiv:1805.04852 (search result; no tool found). https://www.arxiv.org/abs/1805.04852

Repository files read: `docs/src/status.md`, `docs/src/interactive.md`, `plan/46-research-next.md`, `bench/TOOLS.md`, `core/src/proofs/rule.rs`, `.claude/rules/core-proofs.md`, `.claude/rules/core-derivations.md`, `.claude/rules/core-inverse.md`, `plan/notes/research/scope/education.md`, and `linlog-prover.github.io/src/content/rules/03-tensor.md`.

## 7. Not verified

- Beluga, Celf, Teyjus, Logitext and the Logics Workbench were not read; no claim about them is made. Pandora, "Gentzen web tools" and Ask-Elle were not looked up.
- Licences, release dates or last-activity dates for Twelf (licence file), Lambdapi, Abella (licence), L-Framework, MetTeL2, LoTREC, Carnap and Isabelle's exact licence terms were not shown by the pages read.
- Sequoia's exact checks come from a search summary of the IJCAR paper (the Springer page redirected to a login); whether a later version checks cut admissibility or invertibility is unknown. The repository README says nothing on rule specification.
- Jape's theory-file syntax and the "Roll your own Jape logic" manual were not read; how Jape represents contexts and side conditions is unknown.
- Whether Carnap lets an instructor define a system in data, or only the maintainers add one, rests on a Daily Nous snippet and the 2026 abstract.
- The Proof Machine's YAML schema was read only as described in the 2016 talk paper, not in the repository files; whether later versions changed it is unknown.
- The AC-matching complexity results come from abstracts and mirrors, not the primary paper's text.
- The cost estimates, the kernel recommendation, the linear-time claim for checking a derivation that names its principal formula and split, and the ranking are inferences; no code was read or run to test them, and the generic checker's actual speed against the fast checker is unmeasured.
- Nothing was run. A later session designing candidate 1 would want to measure matching cost on the benchmark families' proofs rewritten as schema instances; that needs a capped run on given cores.
