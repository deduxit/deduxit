# Proof theory, updated: what to add after 0.2.0

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

This note updates `plan/notes/research/scope/proof-theory.md` (written 2026-10-09 to 17, cited here as "the old note", its candidates keeping their names `proof-theory-1` to `proof-theory-11`; its source numbers `S1` to `S48` are kept for the old sources and listed in its section 4). New sources are numbered `N1` and on, in section 6. A claim without a bracket is marked "(inference)".

## 1. What linlog has today

From `docs/src/status.md`, `docs/src/proof-nets.md`, `docs/src/cut.md` and `plan/later.md`; nothing below may re-propose these.

- Proof nets for MLL (with or without Mix) and MELL without units (with boxes, `?` nodes with any number of copies), with an independent Danos-Regnier criterion decided by Yeo's deletion test (so the 2025 Yeo-theorem work, N1, is about the very test linlog uses), sequentialization and desequentialization, SVG drawing, text and JSON forms; IMLL and intuitionistic MELL nets as essential nets. The status chapter says the net of a proof is canonical only up to which axiom each weakening hangs on, and may differ for proofs that hold a `?` formula twice. So the net is the canonical object for unit-free MLL only; there is no `equiv` command or `Proof::canonical`.
- Cut and cut elimination on proof terms (every commutative mode, first order too, one step by name or to the end within a bound), and on nets (with and without boxes, each step named by its case, checked by the criterion, drawn). The search stays cut-free.
- First-order proofs with eigenvariables and explicit witnesses, checked, drawn, certified in the Rocq library's first-order development; first-order search by unification. No first-order nets.
- Interactive proving: rules applied at a formula of a goal, undo, invertible rules on request, the cut by a named formula. No subformula linking, no dialogue.
- Certified refutations (counts, classical reading, phase-type invariants via the Rocq library; `Disproof::check`), and an "unprovable" that says why where the counts tell. No human-facing explanation (failing slice, missing resources, best attempt).
- The ordinary layer (`core/src/ordinary/`): classical, intuitionistic and minimal propositional logic through embeddings, proofs read back as LK or LJ, a Rocq certificate over `Prop`. No combinatorial proofs.
- Exports: LaTeX, Typst, SVG, PNG, PDF, Rocq (NanoYalla and the library's own kernel). No Curry-Howard, session-type or calculus-of-structures export.
- The proof size is computed from the term without building the derivation (status chapter); `plan/later.md` records a proof DAG. I found no proof compression or cut introduction in the status chapter.

## 2. The field, as it stands today (what changed since the old note)

**Proof equivalence and canonical forms.** The old note's account stands where re-read: maximally multi-focused proofs are canonical for MALL and in bijection with nets on unit-free MLL [S2], MALL proof equivalence is Logspace-complete [S4; re-read in N2, the journal version of Bagnol's paper], and Hughes and van Glabbeek's nets identify exactly the proofs equal modulo rule commutation [S6; the search result of 2026-11-08 states its main result as that kernel statement, N3]. Nothing in my searches of 2025 and 2026 changed the complexity picture: I found no 2025 or 2026 paper on proof equivalence with units, on MALL net canonicity or on conflict nets (negative result of two searches; see section 7). What is new around it:

- Di Guardia's thesis (ENS Lyon, defended 2024-09-23) and the LMCS paper by Di Guardia and Laurent, "Type isomorphisms for multiplicative-additive linear logic" (LMCS 21(4), 2025-11-21, arXiv 2402.11987): an equational system that generates exactly the isomorphic MALL formulas, extending Balat and Di Cosmo's multiplicative result; the unit-free case uses Hughes and van Glabbeek's nets, full MALL with units goes by the sequent calculus, cut elimination and rule commutations [N4, N5]. The thesis also gives a simpler sequentialization proof for Hughes and van Glabbeek's nets [N5].
- "Yeo's theorem for locally colored graphs: the path to sequentialization in linear logic" (Di Guardia, Laurent, Tortora de Falco, Vaux Auclair; FSCD 2025, LIPIcs 337 paper 16; arXiv 2603.03262, 2026-03-03): sequentialization under the Danos-Regnier criterion from a generalised Yeo theorem, and per the arXiv abstract it works with Mix and unit-free MALL [N1].
- Di Donna and Tortora de Falco, "On the role of connectivity in Linear Logic proofs" (arXiv 2506.21678, 2025-06-26): the Danos-Regnier acyclicity plus "number of components equals bottoms and weakenings plus one" is necessary but not sufficient for MELL proofs; they give a geometric condition on untyped structures that makes it sufficient, stable under one cut-elimination step, and name fragments where it is a correctness criterion [N6]. This bears on linlog's own account of weakenings hung on axiom links (status chapter): the paper studies exactly the weakening and bottom nodes that make MELL nets non-canonical (inference). I read the first 100 000 characters of 155 683 only.
- Pistone, "A new conjecture about identity of proofs" (arXiv 2110.02630, 2021): rule permutations in propositional, first-order and second-order logic as naturality conditions [N7]. Not linear-specific.
- Veltri, "Maximally multi-focused proofs for skew non-commutative MILL" (WoLLIC 2023): multi-focusing as the canonical form beyond commutative linear logic [N8; seen as a search result only].

**Focusing and polarities.** Dale Miller, "Linear logic using negative connectives" (FSCD 2025, LIPIcs 337 paper 29): defines polarity by invertibility of the right rule, gives a calculus with negative connectives only, multi-focused proofs, and a split of linear logic into L0, L1, L2 [N9, from the FSCD 2025 programme's abstract; the PDF itself was unreadable to my tool]. It supports candidate 1's "multi-focused normal form" route and shows the multi-focus literature is still active.

**Cut elimination, cut introduction.** Saurin, "Interpolation as cut-introduction: on the computational content of Craig-Lyndon interpolation" (FSCD 2025, LIPIcs 337 paper 32): from a cut-free proof of A ⊢ B in first-order linear logic it constructs an interpolant C over the shared vocabulary and proofs of A ⊢ C and C ⊢ B whose composition cut-reduces to the original; the same for LJ and LK through linear translations; the construction read as cut introduction [N10]. A seminar on cut expansion in MLL nets announces the same direction for nets (a proof with many cuts to one cut at the root) [N11; abstract only]. I found nothing new on the complexity of cut elimination in linear logic since [S27] (not searched beyond one query that failed; see section 7).

**Deep inference.** Acclavio and Straßburger, "Intuitionistic BV" (TABLEAUX 2025, arXiv 2505.13284, 2025-05-19): a deep-inference system for an intuitionistic BV with cut elimination, whose restriction to the MLL connectives is IMLL; the search result repeats the earlier fact that BV has no cut-free sequent calculus [N12]. This changes candidate 5 only in that the non-commutative operator now has an intuitionistic variant.

**Tools.**
- Click & coLLecT (github.com/ComputerAidedLL/click-and-collect, LGPL-2.1, 479 commits, 19 open issues; the page shows no last-commit date): interactive sequent proofs in LL, auto-prove and auto-reverse, a cut mode, a "proof transformation" mode, provability checks, proof sharing by compressed URL, LaTeX and Coq export (via nanoyalla), and, by its module names, proof simplification and compression. The page does not mention proof nets or MALL nets [N13]. Of the old candidates, 6 (compression) and the cut half of candidate 7's teaching use overlap with what it already does; read its modules before specifying either (inference).
- Yalla: the Rocq package `coq-yalla` 2.0.6 (2024-09-16) requires Coq 8.20 and below 8.21; source at github.com/olaure01/yalla (named by the package page; I did not open the repository) [N14]. linlog's library bridges to NanoYalla and Yalla's `nanoill.v` (status chapter), so a Rocq 9 gap in Yalla is a maintenance fact for linlog's bridge checks (inference); the state of Yalla itself under Rocq 9 I did not establish.
- A formalisation of unit-free MLL nets with sequentialization and cut elimination in Rocq by Di Guardia and Laurent (TLLA 2025, `perso.ens-lyon.fr/olivier.laurent/rocqpn.pdf`) [N15; known from a search snippet, the PDF was unreadable].
- Profound (github.com/chaudhuri/profound): "an experiment in formula linking as an interaction method", 0.3 alpha, copyright 2013 INRIA, 91 commits, 4 stars, no activity date shown [N16]. Actema (Donato, Strub, Werner [S25]): I could not open its repository or find its present state (section 7).

**Venues.** TLLA 2025 was the 9th workshop, Birmingham, 19 to 20 July 2025, with FSCD 2025; the 10th is announced for Lisbon, 18 to 19 July 2026 [N17]. Its programme page was empty when read, so the talks of TLLA are only known through authors' own pages.

## 3. Candidates

Status of each old candidate, then the new ones. Costs are in sessions as the brief defines them.

### proof-theory-1: Proof equivalence and canonical proofs

*Status: untouched as a feature; its prerequisite is half built.* Nothing in `docs/src/` offers `equiv`. Nets exist for MLL and MELL and are the canonical object for unit-free MLL; the status chapter states the MELL caveat (weakening links, repeated `?` formulas), so for MELL the net comparison is sound but incomplete: equal nets mean equal proofs, different nets may still be equivalent (inference from the status chapter). Cost as before for MLL: small, one session shared with a `Proof::canonical` for MALL by the multi-focused form [S2]; a fraction of a session for "equal up to permutation" on MLL by desequentialising both sides and comparing nets. The cost of the MALL half is unchanged; the old note's cost is still right, but the first deliverable is now cheaper because the net comparison exists as a function once `from_proof` and a graph-isomorphism test are used (inference).
*Brings:* a grading and regression oracle (two engines' proofs equal up to permutation), research on identity. *Unit-free MLL only is canonical; with units PSPACE-complete [S3]* (N-sources did not change that).
*Generalises:* strongly. The question "are two derivations the same proof up to permutation of independent rules" exists for LK and LJ (Pistone [N7] frames it for propositional, first-order and second-order logic; Veltri [N8] for a non-commutative linear logic), for natural deduction and for sequent suites. A permutation normaliser over a rule-schema derivation type (see section 5) would serve all of them (inference).
*Sources:* S2, S3, S4, S6, N2, N7, N8, N9.

### proof-theory-2: MALL nets as conflict nets or linking sets

*Status: untouched; `plan/later.md` still lists MALL nets as dropped ("only if a use case appears").* Prerequisites changed in two ways: the net machinery now has boxes, cut links and step-by-step reduction with highlighting, so a second net kind has real code to share; and the correctness-by-colouring argument for Hughes-van Glabbeek nets now has a simpler published sequentialization proof (Di Guardia [N5], Yeo for locally coloured graphs [N1], which per its abstract covers unit-free MALL). Cost of one session still looks right for display plus criterion; sequentialization is easier than the old note assumed (inference). Conflict nets [S7] I did not re-read in 2026; the search found no later work on them (section 7).
*Brings:* a drawing of MALL, the one core fragment with none; the input of candidate 13. *Generalises:* no, linear-specific (`&` and `⊕` slices).
*Sources:* S5, S6, S7, N1, N5.

### proof-theory-3: Unification nets for first-order MLL

*Status: untouched.* The prerequisite exists now: step 38 and the first-order proofs of steps 40 to 45 carry explicit witnesses and eigenvariables (status chapter), so the "witness slot must allow implicit" remark of the old note needs a check against the code before any plan (the old note said so; I did not read `core/src/` here). Cost: half a session after a first-order net data model, which is a data-model change of its own (nets are propositional today); with MELL boxes already built, a first-order net is more than half a session and I would now say one session (inference). *Brings:* a canonical first-order net that no witness annotates. *Generalises:* partly; the canonicity-of-quantifiers idea is the first-order version of candidate 1 for any sequent suite (inference). *Sources:* S8, S9.

### proof-theory-4: Subformula linking in the interactive session

*Status: untouched.* `Interactive` applies rules at a formula of a goal (status chapter). Click & coLLecT, the model for the web page, has no linking in the features its README lists [N13]. Profound's last public state is a 2013 alpha [N16]; Actema's I could not read. The gesture remains unserved in the sources I read, which supports the old note's reading that it is the main web gesture (inference). Cost unchanged (small to one session; the chain is non-unique under `⊗`).
*Brings:* fewer clicks on wide sequents, teaching of "why these meet". *Generalises:* fully; linking works on any sequent calculus with an invertibility discipline, and Chaudhuri's and Actema's work was on classical first-order logic and Coq [S24, S25] (inference for a suite of suites).
*Sources:* S24, S25, N13, N16.

### proof-theory-5: Calculus-of-structures view, and BV

*Status: untouched.* New: intuitionistic BV with cut elimination [N12]; the old note's (b) now has an intuitionistic target, still without a cut-free sequent calculus. Cost and risk unchanged: (a) one session for an export, (b) research-grade, two or more sessions and a new data model. Rank low. *Generalises:* deep inference is the one proof-theoretic family that reaches classical and intuitionistic logic and modal logics together (inference from [S20, S21, N12]).
*Sources:* S14 to S23, N12.

### proof-theory-6: Proof compression and minimal proofs

*Status: partly.* Cut and cut elimination exist (so cut introduction as compression has its inverse), the proof size is computable without unfolding, and Click & coLLecT already has proof simplification and compression modules [N13] (I did not read them). Item (i), redundancy removal on the term, and item (iii), cut introduction, are untouched in the status chapter. Cost: (i) small, (ii) no default, (iii) one session; Saurin's cut-introduction (N10, candidate 12 below) gives a concrete construction for (iii) that the old note lacked (inference).
*Generalises:* yes for (i) and (iii) on LK and LJ through the ordinary layer, since the layer reads linear proofs back; cut introduction is logic-independent as a theme (Hetzl et al. [S28], N10).
*Sources:* S26 to S29, N10, N13.

### proof-theory-7: A token machine on nets

*Status: untouched; prerequisites built.* MELL nets with boxes, cut links and cut-elimination steps with highlights are in the manual (`cut.md`), which is what the old note required (candidate 33 and 34). I did not find a visualiser of the GoI machines in 2026 either, but my search for that was not made (section 7). Cost: a session, as before, for MLL and MELL with boxes. *Brings:* a teaching picture of GoI beside the existing reduction animation. *Generalises:* no. *Sources:* S30 to S35.

### proof-theory-8: A dialogue mode

*Status: untouched.* No source read in this update changed its premises. Cost: a session plus soundness panel. *Generalises:* strongly (Lorenzen games for classical and intuitionistic logic, the Alama-Uckelman tool [S41] is for non-linear rule sets), so it is a suite-level feature in the author's collection (inference). *Sources:* S37 to S41.

### proof-theory-9: Explanations of unprovability for a reader

*Status: partly.* The machine-checkable half was built: certified refutations (counts, classical reading, atom balance) and an "unprovable" that says why where the counts tell (`status.md`). The reader-facing views (failing slice, missing and surplus resources, best attempt as a partial derivation) are not in the status chapter. Cost: (i) small, (ii) a bounded search of its own, (iii) memory in the engine, one session in all; unchanged. Rank up: step 45's certified refutations give (i) and (ii) a checked core to explain (inference).
*Brings:* the teaching audience and the AI models (a structured reason is more useful to a model than a bare "unprovable" (inference)). *Generalises:* strongly: the open branch of a failed tableau, the stuck leaf of a failed LK search and the countermodel are the same reader-facing object in every suite.
*Sources:* S44 to S46.

### proof-theory-10: Combinatorial proofs for the ordinary layer

*Status: untouched.* The ordinary layer exists (status chapter). New: Acclavio and Straßburger's combinatorial proofs for constructive modal logics CK and CD (FSCD 2022 search result), and the combinatorial semantics of constructive modal proofs (arXiv 2104.09115, 2021) [N18]; I did not read either. They extend the family to the modal logics the author names. Cost unchanged (one session); more value for a suite of suites since one graph-over-formula drawing then serves classical, intuitionistic and modal logic (inference). *Sources:* S11, S12, N18.

### proof-theory-11: Proofs as programs: a Curry-Howard export

*Status: untouched as an export; the reduction half is built* (cut elimination on terms is cut elimination of the typed term, inference). The author's list names simply typed lambda calculus and System F, which `plan/notes/research/next/lambda-calculi.md` covers (another note); this candidate remains the linear case: linear lambda terms and session processes [S47, S48]. Cost half a session as before. *Generalises:* it is the bridge to the lambda-calculus suite. *Sources:* S47, S48.

### proof-theory-12: Interpolants by cut introduction (new)

*What it brings and to whom.* For a proved A ⊢ B the command would return an interpolant C over the shared vocabulary, with checked proofs of A ⊢ C and C ⊢ B, whose cut (linlog has the cut rule and its elimination) reduces back to the input proof, following Saurin [N10]. Research: a proof-relevant interpolation tool for first-order linear logic and, through the translations, for LK and LJ, which N10 states. Teaching: an interpolant is a concrete, checkable lemma. AI models: a verifiable artefact with a clear success condition (inference). I found no other tool that computes proof-relevant interpolants for linear logic (not searched for exhaustively; section 7).
*What a suite would offer.* Decide: not applicable (it consumes a proof). Check: the three proofs by the existing checker, and the vocabulary condition by a walk over formulas; the cut reduction by the existing elimination. Prove: the construction on a cut-free term. Export: all existing. Certify: the three proofs are ordinary linlog proofs, so the existing Rocq certificates apply, and the "reduces back" statement can be checked by running elimination (inference).
*Cost.* One to two sessions: a recursion on the cut-free term (Maehara's method [N10]) over linlog's term forms and modes; first order adds the quantifier cases, and the exponential cases I have not verified from the paper (I read the abstract only). Adds nothing to the trusted base, since the checker validates its outputs.
*Risk.* Which modes and fragments the construction covers (the abstract says "first-order linear logic", with Mix, affine or intuitionistic variants not stated); interpolant size may be exponential (unsourced).
*Prerequisites.* Cut and cut elimination on terms in every mode, first-order proofs: both built (status chapter). Uses the ordinary layer for LK and LJ.
*Generalises:* yes: Craig-Lyndon interpolation is a property of every logic with cut elimination; the construction transfers to LK and LJ through linlog's embeddings (N10) and in principle to a natural-deduction or sequent suite with cut admissibility (inference).
*Sources:* N10, N11 (the nets side, abstract only).

### proof-theory-13: Type isomorphisms of formulas (new)

*What it brings.* `linlog iso A B`: decide whether two MALL formulas are isomorphic, with, if yes, the two proofs A ⊢ B and B ⊢ A and the check that their cut reduces to the identity (axiom) both ways; if no, say so. Di Guardia and Laurent give an equational system that generates exactly the isomorphisms of MALL, including units, extending the multiplicative case [N4, N5]. Research: a runnable form of the theorem. Teaching: isomorphisms such as distributivity of `⊗` over `⊕` and the cancellation laws. AI models: a yes/no with a proof pair (inference).
*What a suite would offer.* Decide by normalising with the equations (the decision procedure's complexity I did not read); check by linlog's existing cut elimination on the composition; prove step by step as two ordinary proofs; certify through the existing certificate for each proof plus the cut-elimination step trace (the library has the cut calculus). A check of "isomorphic" by search alone is also possible: search both sequents, compose, eliminate, compare with the axiom (inference).
*Cost.* One session for MLL and MALL; adds nothing to the trusted base when the check is by elimination. *Risk.* Units and the exact complete system must be read from the paper; exponentials excluded (the paper treats MALL only).
*Prerequisites.* Cut elimination on terms (built). MALL nets (candidate 2) are optional, used by the paper only for the unit-free proof.
*Generalises:* yes in idea: isomorphism of types is a topic of the simply typed lambda calculus (Balat and Di Cosmo are named by N4 for the multiplicative case; the equational theory for STLC with products and sums is classical, unsourced here), so the lambda-calculus suite could carry an `iso` command (inference).
*Sources:* N4, N5.

### proof-theory-14: A certified net criterion in the Rocq library (new)

*What it brings.* The Rocq library `rocq-linlog` checks proof terms; the net criterion (Yeo's deletion test, sequentialization) is trusted Rust today (status chapter). Di Guardia and Laurent formalised unit-free MLL nets with sequentialization and cut elimination in Rocq (TLLA 2025) [N15], and a simpler sequentialization proof via the Yeo generalisation exists [N1]. A formalised criterion would let `--format rocq` certify "this structure is correct" for nets, and relate net sequentialization to the library's term checker. For research and for the trusted-base story (the step-46 area on correctness verification will weigh it; this note only flags it) (inference).
*Cost.* Two to three sessions in the library's own repository (the sequentialization theorem is the hard part; N15's development is the reference and its licence and reusability I could not read). Adds a Rocq theorem and removes the net criterion from the unverified Rust base for the certified cases. *Risk.* MELL with boxes is harder than MLL; the connectivity paper shows the basic criterion is not enough for MELL with weakening [N6], which is exactly the case the status chapter flags.
*Prerequisites.* The library's term checkers and a data model of nets in Rocq (new).
*Generalises:* no, linear-specific, though Yeo's theorem lemma is graph theory.
*Sources:* N1, N6, N15.

## 4. Considered and rejected

The old note's rejections (coherence-space and relational pictures, ludics, atomic flows, interaction-net evaluators, deep inference as the main engine, Skeptik compression) stand; no source of this update contradicts one. New:

- **A MELL criterion from the connectivity paper** [N6] as a replacement for Danos-Regnier: it is a necessary condition made sufficient under an extra geometric condition on untyped structures, and linlog's criterion with boxes already decides the cases it handles; no gain for a user (inference).
- **TLLA and Linearity programme mining as a source of candidates**: the TLLA 2025 page was empty and no 2025 accepted-paper list was readable, so no claim rests on it.
- **Differential linear logic models and Ohana trees** (the other FSCD 2025 linear-logic talks [N19]): semantics, no tool consequence.

## 5. What the structure must allow

(Judged from `.claude/rules/core.md` and the module rules only as the brief names them; I did not read them in this update, so this is from the status chapter and the old note.)

- Reusable across suites: the derivation as a DAG over occurrences with a stable node order (equivalence up to permutation, compression, cut introduction and interpolation all act on it); the cut rule and the cut-elimination driver with named cases; the checker's one-pass shape; the interactive session as goal plus rule application (linking and a dialogue wrap it); the export entry over `&Proof` with a target enum; refutations as named payloads, extendable by variants (explanations).
- Linear-specific: proof nets, the coloured structure graph and boxes, the token machine, MALL slices and conflict relations, the count-based refutations. In a collection of suites these stay in the linear suite; a natural-deduction or sequent suite would share the permutation normaliser's generic part (a rule-schema derivation type with "independent rule instances commute") and the interpolation and compression constructions, not the nets (inference).
- For candidates 1, 12 and 13 the one thing to keep open now is that a derivation carries its mode and its rule schema, so that a permutation relation or an interpolation recursion can be written over schemas rather than over linlog's fixed rule list (inference).

## 6. Sources

Old sources S1 to S48 are those of the old note's section 4 (not all re-read; re-read today: S4 by N2, S6 and S2 by search results, S24 by N16, S46 by N13). New, all read 2026-11-08:

- N1. R. Di Guardia, O. Laurent, L. Tortora de Falco, L. Vaux Auclair, "Yeo's theorem for locally colored graphs: the path to sequentialization in linear logic", FSCD 2025 (LIPIcs.FSCD.2025.16), arXiv 2603.03262. https://arxiv.org/abs/2603.03262 ; programme https://easychair.org/smart-program/FSCD2025/2025-07-18.html
- N2. M. Bagnol, "Multiplicative-additive proof equivalence is Logspace-complete, via binary decision trees", journal version. https://arxiv.org/pdf/1707.00991
- N3. R. van Glabbeek, D. Hughes, "MALL proof nets identify proofs modulo rule commutation" as summarised by the search result of 2026-11-08 (primary: S6). https://cgi.cse.unsw.edu.au/~rvg/pub/kernel.pdf
- N4. R. Di Guardia, O. Laurent, "Type isomorphisms for multiplicative-additive linear logic", LMCS 21(4), 2025, arXiv 2402.11987. https://arxiv.org/abs/2402.11987
- N5. R. Di Guardia, thesis "Identity of proofs and formulas using proof-nets in multiplicative-additive linear logic", ENS Lyon, 2024-09-23. https://www.irif.fr/~diguardia/papers/thesis_manuscript.pdf (read via the event page https://www.ens-lyon.fr/evenement/recherche/identity-proofs-and-formulas-using-proof-nets-multiplicative-additive-linear and search results; the manuscript itself not opened)
- N6. R. Di Donna, L. Tortora de Falco, "On the role of connectivity in Linear Logic proofs", arXiv 2506.21678. https://arxiv.org/html/2506.21678v1
- N7. P. Pistone, "A new conjecture about identity of proofs", arXiv 2110.02630. https://arxiv.org/abs/2110.02630
- N8. F. Veltri, "Maximally multi-focused proofs for skew non-commutative MILL", WoLLIC 2023. https://dc2.philarchive.org/rec/VELMMP (page returned 403; known from the search listing) ; https://www.mathstat.dal.ca/wollic2023/papers/veltri.pdf
- N9. D. Miller, "Linear logic using negative connectives", FSCD 2025 (LIPIcs.FSCD.2025.29). https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2025.29 ; https://www.lix.polytechnique.fr/~dale/papers/negll.pdf (PDF unreadable to the tool; gist from the programme page in N1)
- N10. A. Saurin, "Interpolation as cut-introduction", FSCD 2025 (LIPIcs.FSCD.2025.32). https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2025.32
- N11. Seminar "Cut expansion in proof nets of multiplicative linear logic", I2M Marseille. https://www.i2m.univ-amu.fr/evenements/cut-expansion-in-proof-nets-of-multiplicative-linear-logic/ (search result; abstract only)
- N12. M. Acclavio, L. Straßburger, "Intuitionistic BV", TABLEAUX 2025, arXiv 2505.13284. https://www.arxiv.org/abs/2505.13284
- N13. Click & coLLecT. https://github.com/ComputerAidedLL/click-and-collect
- N14. `coq-yalla` 2.0.6. https://rocq-prover.org/p/coq-yalla
- N15. R. Di Guardia, O. Laurent, TLLA 2025 formalisation of MLL proof nets in Rocq. https://perso.ens-lyon.fr/olivier.laurent/rocqpn.pdf (PDF unreadable; known from a search snippet)
- N16. Profound. https://github.com/chaudhuri/profound
- N17. TLLA 2025 site https://tlla2025.sciencesconf.org ; TLLA 2026 calls https://lists.seas.upenn.edu/pipermail/types-announce/2026/012400.html (search results only; the 2026 call was not opened)
- N18. M. Acclavio, G. Catta, L. Straßburger, combinatorial semantics for constructive modal proofs, arXiv 2104.09115; Acclavio, Straßburger, combinatorial proofs for CK and CD, FSCD 2022. https://arxiv.org/pdf/2104.09115.pdf ; https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2022.19 (search results only)
- N19. FSCD 2025 programme, session 14. https://easychair.org/smart-program/FSCD2025/2025-07-18.html

## 7. Not verified

- No 2025 or 2026 paper on proof equivalence with units, on MALL net canonicity or on conflict nets was found in two searches; that is a negative result of those searches, not proof of absence. Conflict nets [S7] and Hughes's unification nets [S8] were not re-read.
- Nothing was found or searched on a 2025 or 2026 complexity result for cut elimination, on proof compression or certificate-size work (one such search failed because the shared search budget ran out). The ranking of candidate 6 therefore rests on the old note and on Click & coLLecT's README.
- Linearity 2025 and 2026 workshops were not searched; TLLA's accepted lists were unreadable. FSCD 2025 was read only for session 14 and Saurin's paper; LICS and CSL 2025 and 2026 were not searched for proof-net papers.
- Actema's repository and present state, Yalla's GitHub repository and Rocq 9 compatibility, the licence of Di Guardia and Laurent's Rocq formalisation, and Click & coLLecT's last commit date could not be read. Miller's, N15's and several PDFs were unreadable to the fetch tool; claims about them rest on programme pages and search snippets, and are marked.
- The coverage of Saurin's construction (exponentials, Mix, affine, intuitionistic variants) and the complexity of deciding MALL isomorphism (N4) were not read in full.
- Whether `core/src/` already has something under candidates 1, 6 or 9 beyond what the status chapter says was not checked in code; statuses are from the manual chapters and `plan/later.md`.
- Ranking (inference), the three audiences. Teaching: 9, 4, 1 (MLL net comparison as a grader), 7, 2, 13. Research: 1 with 2, 12, 13, 14, 3. AI models using the guide: 1 (a deterministic yes/no), 9 (structured reasons), 12 and 13 (artefacts with checkable success conditions), 6. Generalising beyond linear logic, in order: 9, 4, 1, 12, 6 (i and iii), 8, 11, 10; linear-specific: 2, 3, 5(b), 7, 14.
