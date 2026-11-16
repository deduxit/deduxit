# Research notes: labelled / tableau decision procedures for the modal cube

Date read: 2026-11-16. All texts were fetched as PDF and converted with pdftotext (symbols for box/diamond/Gamma/Delta were partly lost in some extractions; where I rely on context to restore a symbol I say so).
Downloaded copies (scratch only): /tmp/claude-1000/-home-tux-Projects-own-deduxit-wt-step-58b-8c0bbc/5a76242c-67f4-4c76-b32d-dc9bed31f6da/scratchpad/src/

Legend for "how read": TEXT = read in the source's own text; ABSTRACT = abstract/search snippet only; SECONDARY = another paper describing it; MEMORY = from memory, not verified.

Sources actually obtained
- Negri 2005 (JPL 34:507-544): TEXT, Wayback copy of www.mv.helsinki.fi/home/negri/paml.pdf (live URL 404).
- Negri 2009 "Kripke completeness revisited": TEXT of the Helsinki preprint (Wayback copy of .../negri/gkcrev.pdf); I did not see the printed chapter in "Acts of Knowledge", so page numbers/wording may differ.
- Garg, Genovese, Negri LICS 2012: TEXT of the conference version (people.mpi-sws.org/~dg/papers/lics12.pdf) and of the full technical report with appendices (mpi-sws.org/~dg/papers/lics12-tr.pdf, 2276 lines). Theorem numbers below are the TR's.
- Heuerding/Seyfried/Zimmermann 1996: NOT obtained (Springer paywall). Only abstract (via search result) and descriptions in Goré 1999, Wu-Goré 2019 and Howe 1997.
- Goré, "Tableau methods for modal and temporal logics": TEXT of the draft TR-ARP-15-95 (last revised Sep 1997), www.inf.ufpr.br/marcos/ci311/Gore_Tableau_methods_for_modal_and_temporal_logics.pdf. Section numbers are the draft's (the printed handbook chapter is chapter 6 of the Handbook, the draft numbers its sections 1-6, "chapter 6" in the task text may mean the Handbook's own numbering).
- Fitting 1983 book and Handbook chapter "Modal proof theory": NOT obtained. Substitutes read in TEXT: Fitting, "A simple propositional S5 tableau system" (Ann. Pure Appl. Logic 2003; securedata copy of SimpleSfive.pdf), Open Logic Project "Simple tableaux for S5" (SECONDARY for Fitting-Mendelsohn rules), and Goré's account of [Fit83 pp. 201, 226, 229] (SECONDARY).
- Poggiolesi 2008: NOT obtained. HAL copy is behind a bot check (I did not try to bypass it); Cambridge page gave only the abstract.
- Avron 1996: NOT obtained.
- Restall 2007 "Proofnets for S5": TEXT of the Feb 2006 draft (consequently.org/papers/s5nets.pdf).
- Wu & Goré, ITP 2019 (LIPIcs 141, 31): TEXT (drops.dagstuhl.de PDF). NOTE: arXiv 1910.01697 is NOT this paper (it is Bentzen's Lean S5 completeness paper). The Wu-Goré paper is not on arXiv as far as I found.
- Horrocks, Sattler, Tobies, "Practical reasoning for very expressive description logics", Logic J. IGPL 8(3), 2000: TEXT (lat.inf.tu-dresden.de/research/papers/2000/HorrSattTob-IGPL.pdf).
- Extra: Brünnler, "Nested sequents" (Habilitationsschrift, arXiv 1004.1845), chapter 2: TEXT. Not on your list, but it is the best hit for question H (all 15 logics, termination, finite countermodel).

---------------------------------------------------------------------
## A. Negri 2005, "Proof analysis in modal logic" (TEXT)

### G3K (Table II, p. 512). Sequents are multisets Gamma => Delta of relational atoms xRy and labelled formulas x:A.
Initial sequents: `x:P, Γ ⇒ Δ, x:P` and `xRy, Γ ⇒ Δ, xRy` (the second is only needed for deriving frame axioms; p. 513 says it "can as well be left out").
Propositional rules are the G3c rules with a common label x (L&, R&, Lv, Rv, L⊃, R⊃, L⊥).
Modal rules (box = 2, diamond = 3 in the extraction):
- L□: `y:A, x:□A, xRy, Γ ⇒ Δ` / `x:□A, xRy, Γ ⇒ Δ`   (principal formula repeated; the premiss adds y:A)
- R□: `xRy, Γ ⇒ Δ, y:A` / `Γ ⇒ Δ, x:□A`   y not in the conclusion
- L◇: `xRy, y:A, Γ ⇒ Δ` / `x:◇A, Γ ⇒ Δ`   y not in the conclusion
- R◇: `xRy, Γ ⇒ Δ, x:◇A, y:A` / `xRy, Γ ⇒ Δ, x:◇A`   (principal formula repeated)

### Frame rules (p. 515, section 3)
- Ref: `xRx, Γ ⇒ Δ` / `Γ ⇒ Δ`
- Trans: `xRz, xRy, yRz, Γ ⇒ Δ` / `xRy, yRz, Γ ⇒ Δ`
- Sym: `yRx, xRy, Γ ⇒ Δ` / `xRy, Γ ⇒ Δ`
- Eucl: `yRz, xRy, xRz, Γ ⇒ Δ` / `xRy, xRz, Γ ⇒ Δ`
- Ser: `xRy, Γ ⇒ Δ` / `Γ ⇒ Δ`, side condition y not in Γ, Δ.
- Dir (directedness) also given. The 2005 paper lists the systems "G3T = G3K+Ref, G3K4 = G3K+Trans, G3KB = G3K+Sym, G3S4 = G3K+Ref+Trans, G3TB = G3K+Ref+Sym, G3S5 = G3K+Ref+Trans+Sym" and says "Thus, a Gentzen system for Deontic logic is obtained by adding rule Ser to G3K". Table III lists axioms T, 4, 5, B, 3, D, 2, W with frame properties (5 = euclideanness).
- Closure condition (Negri 2009, section 4.2, TEXT): instances with a duplicated atom must also be added when Ref is absent: Trans* `wRw, wRw, Γ ⇒ Δ / wRw, Γ ⇒ Δ` and Eucl* `oRo, wRo, Γ ⇒ Δ / wRo, Γ ⇒ Δ` ("Both of the contracted rules are instances of rule Ref"). Needed for contraction admissibility. A decision procedure that keeps relational atoms as a set does not need them.

### Decidability, section 6 (pp. 529-538)
Key statements (verbatim, short):
- Prop. 6.1: "All variables (worlds) in a minimal derivation of a sequent Γ ⇒ Δ in G3K and in its extensions with rules for R, G3T, G3K4, G3KB, G3S4, G3TB, G3S5, and in G3KGL, are either eigenvariables or else variables in Γ, Δ." (the "subterm property"; Lemma 6.2 handles Ref).
- Cor. 6.5: in a minimal derivation, L□ and R◇ are not applied more than once on the same pair of principal formulas (xRy, x:□A) / (xRy, x:◇A) on any branch.
- Thm 6.6: "The system G3K allows terminating proof search." Bounds given: number of L□ applications <= n(□)(p(□)+n(◇)) (no relational atoms in endsequent) or n(□)(p(□)+n(◇)+r) with r atoms; R◇ symmetric with p(◇).
- Thm 6.7: "The system G3T allows terminating proof search." (reflexivity restricted by the subterm property to worlds of the endsequent or eigenvariables: r replaced by r+w+p(□)+n(◇)).
- Thm 6.8: "The systems G3KB and G3TB allow terminating proof search." Bounds for G3KB: n(□)(2p(□)+2n(◇)+2r) for L□ and p(◇)(2p(□)+2n(◇)+2r) for R◇; for G3TB n(□)(3p(□)+3n(◇)+2r+w), p(◇)(same). The argument is a short paragraph: with Sym "whenever an atom xRy appears on the left, yRx has to be added".
- Prop. 6.9 and Cor. 6.10: "The systems G3S4 allows terminating proof search." Method: not a loop check but a bound; in a minimal derivation, for each positive x:□A there are at most n(□) iterated R□ applications on a chain xRx1, x1Rx2, ... Proof by height-preserving substitution x_{m+1}/x_m + height-preserving contraction + Ref gives a shorter derivation. Done only for the language with □ alone ("The results can be generalized by symmetry to the full language").
- Right after Cor. 6.10: "By the remarks before Theorem 6.8, the result above directly extends to G3S5." (one sentence; no separate bound).

### Coverage answer for A
- Proved: K, T, KB, TB (= KTB = B), S4, and (by the one-sentence remark) S5. 
- K4: the subterm property (Prop 6.1) is stated for G3K4 but there is NO termination theorem for G3K4. The S4 argument (R□ chain bound) is the only handle on transitivity and it uses Ref. 
- Euclideanness and seriality (so K5, K45, D, D4, D5, D45, DB): rules given, but no termination result at all in section 6 (the section lists no result for Eucl or Ser).
- Method is "minimal derivation" bounds (pure existence argument of a bounded search), not an implementable loop check; no countermodel extraction in this paper (that is in Negri 2009 / GGN 2012).
How read: TEXT.

---------------------------------------------------------------------
## B. Negri 2009 and Garg-Genovese-Negri 2012

### Negri 2009, "Kripke completeness revisited" (TEXT of Helsinki preprint)
- Section 4 gives G3K (Table 5) and the same frame rules (Ref, Trans, Sym, Eucl, Ser, Dir) plus the closure-condition contracted rules.
- Theorem 5.4: "Let Γ → ∆ be a sequent in the language of G3K*. Then either the sequent is derivable in G3K* or it has a Kripke countermodel with properties *." Method: "applying the rules of G3K* root first in all possible ways. If the construction terminates we obtain a proof, else the tree becomes infinite. By König's lemma an infinite tree has an infinite branch that is used to define a countermodel." 
- Countermodel: frame = all labels of relational atoms in the union Γ = ∪Γi, with relation = the atoms in the union; atoms w:P in Γ forced, o:Q in Δ not forced. The fair, cyclic ordering of 10+r stages (L&, R&, Lv, Rv, L⊃, R⊃, L□, R□, L◇, R◇, then one stage per frame rule). 
- NO blocking, no loop check, no finiteness: the model from an infinite branch can be infinite. Termination of search is not claimed here for any logic (it points to Negri 2005 section 6 for the subterm property).
- Covers every logic whose frame conditions are universal or geometric (including B, 5, 4, D, directedness). Does not claim finite models.

### GGN 2012 (TEXT). Statement of scope
Abstract: "multi-modal variants of several classical and intuitionistic logics including K, T, K4, S4 and their combinations with D are decidable and have the finite model property". Main text (Introduction, "Limitations") and Section 4 "Symmetry condition":
- "To the best of our knowledge, there is no SCR for the symmetry condition (∀x, y.(xNA y) → (yNA x)), the cornerstone of the modal logic S5. Consequently, our method cannot be used to prove decidability for this logic, in either the uni-modal or multi-modal setting." and they remark that "Baldoni has shown that, in general, multi-modal logics with symmetry conditions are undecidable [3]" (their reason it is unsurprising), adding "it is dissatisfying that our method is unable to handle even uni-modal S5".
- Also not covered: "label-producing" conditions like density or confluence (the only label-producing condition handled is seriality, section 3.6/B.8); intuitionistic possibility modalities (section 4).
- Related work (section 6): Negri [16] and Viganò [24] "provide sound, complete and terminating labeled sequent calculi for uni-modal classical logics K, T, K4 and S4"; they credit the containment condition to Gasquet et al. [10] (tableaux for classical uni-modal logics with transitivity, reflexivity, symmetry, euclideanness, seriality, confluence).
- Euclideanness is not discussed as a frame condition anywhere in the TR; the classical table (Fig. 8, TR p. 44) has: K (R(A) = {}), T (R = {}), K4 (R = N_A), S4 (R = N_A), "subsumption" ∀x,y.(xN_A y)→(xN_B y) (R = {}). With the D rule (B.8, Thm B.24) every entry also gets seriality. So of the 15 cube logics: K, KD, KT (D redundant), K4, KD4, S4 are covered (6 of 15). Not covered: B-containing, 5-containing logics (KB, KDB, KTB, K5, KD5, K45, KD45, KB4, S5).

### Classical procedure, exactly (TR Appendix B; the main text does the intuitionistic case and says in Section 5 that the classical case is the same with the countermodel changed)
Calculus Seq-MMχ (Fig. 5): rules init, ⊤R, ∧R, ∧L, ¬R, ¬L, necR, necL, and frame rules from χ (classical, no ≤).
History = sequent Σ; M; Γ; Δ with Σ the labels, M relational atoms (set, "tree-like": derived from a finite tree of N_A atoms and closure by frame rules), Γ and Δ sets of labelled formulas.
Signed formulas of a world x (classical, Def. B.5):
 Sfor(x) = {T(φ) | x:φ ∈ Γ} ∪ {F(φ) | x:φ ∈ Δ} ∪ {T(□_A φ) | ∃y. y (R(A))* x ∈ M and y:□_A φ ∈ Γ}.
 (R(A) is the logic's "suitable closure relation"; for K and T it is empty so only x's own formulas count; for K4 and S4 it is N_A, so a world also "inherits" the T(□φ) of every N_A-ancestor.)
 x ⊑ y iff Sfor(x) ⊆ Sfor(y).  (I write ⊑ for the relation the extraction shows as "4".)
Blocking condition (Fig. 7, rule necR, boxed applicability conditions): necR on x:□_A α is applicable only if
 (i) for all y in Σ: xN_A y ∈ M ⇒ y:α ∉ Δ, and
 (ii) for all y in Σ: (y is a proper ancestor of x in the tree underlying M, i.e. y ≼ x, y ≠ x) ⇒ NOT (x ⊑ y).
 So: x is blocked iff SOME PROPER ANCESTOR y has Sfor(x) ⊆ Sfor(y). It is SUBSET containment of the signed-formula sets (not equality) and compares x with its ancestors only.
Seriality rule D (B.8): applicable to x if no N_A successor exists in Σ and no proper ancestor y has x ⊑ y. Saturation clause (1) becomes: for each x and each A: (a) some y with xN_A y ∈ M, or (b) some y ≠ x, y ≼ x, x ⊑ y.
Saturated history (Def. B.8): M tree-like and closed under the frame rules; no x:p in both Γ and Δ; no x:⊤ in Δ; ∧/¬ clauses; □ left clause: x:□α ∈ Γ and xN_A y ∈ M ⇒ y:α ∈ Γ; □ right clause: x:□α ∈ Δ ⇒ (a) some y with xN_A y ∈ M and y:α ∈ Δ, or (b) some y ≠ x, y ≼ x, x ⊑ y.
Countermodel (Def. B.9): worlds Σ; relations M(C) with C = {(x,y) | x ⊑ y}; h(p) = {x | x:p ∈ Γ}. Here M(C) is the closure of M under the frame conditions χ AND under the extra condition ∀x,y,z. ((x,y) ∈ C ∧ yN_A z) → xN_A z. In words: whenever Sfor(x) ⊆ Sfor(y) (so x was blocked by y or is otherwise included), x gets all N_A-successors of y (this is what the "loop back" means: x inherits y's successors, so a blocked world behaves like its ancestor).
Correctness: Lemma B.10 (truth lemma by induction on formula, then on the tree order), Cor. B.11 (a saturated history has a finite countermodel), Thm B.16 (termination, by the infinite-path argument: along an infinite branch the Sfor values S_i at creation time satisfy S_i ⊉ S_j for i<j, but they all live in a finite poset so there is an infinite strictly ascending chain, contradiction; this uses that Sfor only grows backwards), Thm B.18/Cor B.19 decidability + finite model property + a finite set of finite countermodels, Thm B.22/Cor B.23 "comprehensiveness".
Frame rules and subset blocking depend on an SCR existing for the frame conditions (Def. B.6: (0) R(A) definable, (1) decidable, (2) (R(A)*∘N_A) ⊆ N_A derivable from χ, (3) for closed M and C a set of pairs, if xN_A y ∈ M(C) then x((R(A)∪C)*∘N_A)y with R(A), N_A in M). For symmetry there is none, hence no procedure.
Intuitionistic version (main text, Def. 3.1-3.3): same idea with ≤ and extra T(ϕ→ψ), T(p) inherited from ≤-ancestors; SCR for K, T (R = ≤), K4, S4 (R = N_A ∪ ≤), and I, unit, subsumption (Fig. 4).
How read: TEXT (TR and conference version agree on the statements I compared).

---------------------------------------------------------------------
## C. Heuerding, Seyfried, Zimmermann 1996 (full text NOT obtained)
- ABSTRACT only: "uniform sequent-based calculi with specifically tailored loop-checks" for the modal logics KT and S4, the tense logic Kt, and the fragment IPC(∧,→). K4 is not in the abstract. (via search result, not the Springer page)
- SECONDARY: Goré 1999 (section 4.14.5): "The tableau systems of Heuerding et al [HSZ] are further refinements of our tableau systems which allow for a more efficient check for cyclic branches. However, they are nonstandard in that the denominators and numerators carry extra sets to store the necessary information." and "one of their ideas addresses exactly this point" (avoids re-fulfilling the same eventuality at every level in the cut variant).
- SECONDARY: Wu & Goré 2019 say their tableaux "are variants of sequent calculi with histories given by Heuerding, Seyfried, and Zimmermann [16] (HSZ henceforth)", that for KT the tableau KTT_H keeps a history Σ (a formula set) with rule T: `Σ | φ, Γ` → `φ, Σ | φ, Γ` hmm see G below, and that "the measure given by HSZ does not always decrease" for KT and "could become negative in some cases" for S4. They claim to have given their own termination argument, so the termination claim of HSZ is as published but its measure is questioned (they say it does not necessarily mean HSZ is incorrect).
- SECONDARY: Howe 1997 describes HSZ's history for the intuitionistic case: the history holds goal formulas and is emptied whenever the context is extended.
- Rules for K, KT, S4, K4 as in HSZ: cannot quote; use the Wu-Goré reconstruction in section G. K4: NOT verified for HSZ.
How read: ABSTRACT + SECONDARY. Do not cite rules to HSZ from this note.

---------------------------------------------------------------------
## D. Goré 1999 (TEXT of draft TR-ARP-15-95)

### Systems (section 4.14, Fig. 6 and Fig. 7, drafts' pp. 28-29)
Implicit tableaux (no labels): rules (K), (T), (D), (KD), (K4), (S4), (45), (45D), (B), (T3), (5), (S5), plus analytic-cut variants (sfc2), (sfc3), (sfc_), (sfcT) and the modal cut (mc) (p. 47: "X / □P, X | ¬□P, X where □P ∈ Sf(X)").
Static and transitional rules, e.g. (K) `□X; ¬□P / X; ¬P`, (K4) `□X; ¬□P / X; □X; ¬P`, (S4) `□X; ¬□P / □X; ¬P`, (45) `□X; ¬□Y; ¬□P / X; □X; ¬□Y; ¬□P; ¬P`, (S5) `□X; ¬□Y; ¬□P / □X; ¬□Y; ¬□P; ¬P`, (B) `X; ¬□P / (X; ¬□P; P) | (X; ¬□P; ¬P; □¬□P)`, (T) `X; □P / X; □P; P`, (D) `X; □P / X; □P; ¬□¬P`.
Figure 7 table (verbatim facts; a dagger means analytic cut is used, otherwise cut-free; "?" = open question, no implicit tableau system known):
- Cut-free: C_K, C_T, C_D, C_D' (the KD version with (KD)), C_K4, C_K4D, C_K45, C_K45D, C_S4, and C_S5* (Fitting's destructive S5, only WEAKLY complete, see E).
- Analytic cut (dagger): C†_K45, C†_K45D, C†_K4B (= KB4), C†_S4 (alternative to C_S4 with an easier termination check), C†_B (= KTB), C†_S5, C†_S5'.
- Open ("?"): KB, K5, KDB, KD5 — "some of the basic logics have no known implicit tableau systems". In the draft: "For the logics with a symmetric R we seem to need analytic cut, either as (sfc) or as (sfcT)." and "a building up rule seems essential for C B".
- Therefore of the 15 cube logics, cut-free implicit tableaux exist for K, D, T, K4, D4, S4, K45, D45 and weakly for S5; analytic cut is needed for B(=KTB), KB4, S5 (strongly), and the K45/KD45 variants have both; KB, KDB, K5, KD5 have no implicit calculus (they do have the labelled one below).
- Bibliographic section 4.14.5: Fitting's S5 "essentially C_T + (S5) + (sac)"; semi-analytic cut (sac) "cannot give a decision procedure" because its superformulae are unbounded; Fitting's replacement building-up rule (*) is not "once off" so gives no decision procedure for S5 either.

### Countermodel construction, finite model proofs (section 4.14.4, "Completeness")
Model graphs: worlds = C_L-saturated sets, successor relation ≺, R derived from ≺.
- K, T, D, K4D etc.: tree by modal degree; K, T terminate by modal degree decreasing. 
- K4 (p. 39): successors X_i = {A : □A ∈ w} ∪ {□A : □A ∈ w} ∪ {¬Q_i}; "a sequence either terminates, or a node repeats. If in the latter case n > m are minimal with w_n = w_m we stop the construction and identify w_n and w_m in <W',≺> thus obtaining a circle instead of an infinite path." R = transitive closure of ≺ ("clusters in <W',R> form a tree"). So loop check = EQUALITY of saturated sets along a path (identify the repeat with the ancestor).
- S4 (cut-free C_S4) (p. 41): "proceed as for C_K4 except create a successor for eventuality ¬□P ∈ w only if ¬P ∉ w, and use (S4)". R = reflexive-transitive closure of ≺ (reflexivity from (T)). Termination condition used in the cut variant C†_S4 (p. 43): "(*) if w0≺w1≺…≺w_{i-1}≺w_i is a path and i≥1 is the least index such that □A ∈ w_i implies □A ∈ w_{i-1}; then put w_i ≺ w_{i-1} giving a cycle on this path and stop" = SUBSET check on the box-formulas of the node against its parent, justified because (S4) carries all boxed formulas down so □-formulas accumulate along a path. (Goré notes the termination check in C_S4 is harder: "we have to look at all predecessors in order to detect a cycle".)
- K45/K45D: sequence S = w0≺w1≺… choosing a successor "new to the current sequence" until none is new, then choose the successor wx nearest to the start that fulfils an eventuality and close the cycle w0 ≺ … ≺ wx ≺ … ≺ wm ≺ wx, discarding w1…w_{x-1}. Frame = degenerate cluster followed by one nondegenerate cluster, or a single nondegenerate cluster.
- Finite frames table (Fig. 13): K4 finite tree of finite clusters; K4D likewise with nondegenerate final clusters; K45: a single finite cluster, or a degenerate cluster followed by a finite nondegenerate cluster; K45D: a single nondegenerate cluster or degenerate followed by nondegenerate; S4: finite tree of finite nondegenerate clusters; KB4: single finite cluster; S5: single finite nondegenerate cluster.
- Models for S5, B, KB4 (analytic cut systems): one level only (w0 plus successors v_i for each ¬□Q_i, "put w0 ≺ v_i and stop!"), R = reflexive, symmetric, transitive closure; (5) / (S5) ensure all □/¬□ formulas agree across the cluster. For C†_B and C†_K4B similar single-step constructions (proof for C†_B at p. ~44, for K4B p. 42).
Section 4.12: "a finitely axiomatisable normal modal logic with the finite model property must be decidable" [Hughes-Cresswell p. 154].

### Section 6 (labelled tableaux, explicit accessibility; sections 6.2-6.6)
- Section 6.2 describes Massacci's labelled single-step tableau systems "for all the 15 distinct basic normal modal logics obtainable from K by the addition of any combination of the axioms T, D, 4, 5, and B in a modular way". Labels are dotted integer strings; rule (l◇): `σ :: ¬□P / σ.n :: ¬P` (σ.n new); (lK) `σ::□P / σ.n::P`; (lD) `σ::□P / σ::¬□¬P`; (lT) `σ::□P / σ::P`; (lB) `σ.n::□P / σ::P`; (l4) `σ::□P / σ.n::□P`; (l4r) `σ.n::□P / σ::□P`; (l4d) `σ.n::□P / σ.n.m::□P`; (l5) `σ.n::□P / σ::□□P`... (extraction of l5: "1.n :: □P / 1 :: □□P" with 1 = σ). Fig. 17: LC_K = (lK); LC_T adds (lT); LC_D (lD); LC_KB (lB); LC_K4 (l4); LC_K5 (l4d),(l4r),(l5); LC_KDB (lB),(lD); LC_KD5; LC_K4D; LC_K45 (l4),(l4r),(l5); LC_K45D adds (lD); LC_K4B (lB),(l4),(l4r); LC_B (lT),(lB); LC_S4 (lT),(l4); LC_S5 (lT),(l4),(l4r). Draft notes "[Mas94] contains some minor errors; for example, the system given there for K5 is incomplete" and Massacci's journal version [Mas95b].
- Section 6.4: "A chain is a sequence of labels … where each label is a simple extension of its predecessor. A chain of labels is periodic if there exist distinct labels σ_i and σ_j in the chain (i<j) such that σ_i :: A is on B iff σ_j :: A is on B; that is if {A | σ_i :: A on B} = {B | σ_j :: B on B}." Lemma 6.4.6: every infinite branch of a systematic tableau for finite X is periodic [Fit83].
- Section 6.6 "Cycles, termination and decidability": "once a chain of labels becomes periodic because σ_i and σ_j label identical sets of formulae, all formulae with the longer label are put to sleep. They are awakened only when periodicity for this chain is broken by the appearance of some new formula with a label σ_i or σ_j; see Massacci [Mas94]. … Thus these labelled tableaux can be used as decision procedures for the 15 basic logics. By keeping tabs on cycles we can also prove the finite model property … since the resulting L-frames are exactly the finite-L-frames of Figure 13." "The details are considerably more intricate than the preceding paragraph suggests … But there simply is no space." "a check for periodicity cannot be avoided for the transitive logics."
  So Goré only SKETCHES the labelled decision procedure; the proof is in Massacci [Mas94, Mas95b], not read. The blocking is EQUALITY of label-sets along a chain (not subset), and wakes up when something new arrives.
How read: TEXT (draft). I did not read the printed handbook chapter, nor Massacci.

---------------------------------------------------------------------
## E. Fitting: simplified S5 prefixed tableau and the "box prefix" completeness

### Simple prefixed S5 (SECONDARY: Open Logic Project, "Simple tableaux for S5", rev a216aa6 2026-11-14, based on the Fitting/Fitting-Mendelsohn presentation; I could not read Fitting-Mendelsohn itself)
Prefixed signed formulas `n T φ`, `n F φ`, with n a positive integer (prefix = world name). Rules (Table 1, TEXT of OLP):
- □T: `n T □φ` / `m T φ`, m is used (already on the branch).
- □F: `n F □φ` / `m F φ`, m is new.
- ◇T: `n T ◇φ` / `m T φ`, m is new.
- ◇F: `n F ◇φ` / `m F φ`, m is used.
Justification quoted: "S5 is sound and complete with respect to the class of universal models … In the general case, we take as prefixes sequences of positive integers … But in S5 any world is accessible from any world, so there is no need to so keep track. Instead, we can use positive integers as prefixes." (OLP gives an example derivation of 5 with 3 prefixes.) No termination/size argument in that note.
Termination/size (MEMORY, not verified): each new prefix comes from a diamond-like formula; each □T application is to an existing prefix, so with a once-per-(prefix, formula) restriction there are at most (#subformulas of the form ◇/¬□ + 1) prefixes, each with a set of subformulas, so the tableau has size polynomial in the number of subformulas times the number of branches; the set of worlds is bounded by the number of ◇-type subformulas plus one (they need not be nested because prefixes carry no accessibility). This is the standard S5 argument; I did not find it in the sources I could open.

### The "destructive" S5 tableau (TEXT: Fitting, "A simple propositional S5 tableau system")
- Tableau for X starts with ¬□X (Section 3 intro: "In order to prove the formula X, we do not construct a tableau for ¬X, but for ¬□X!" and Section 4: "a formula X is valid in all S5 models if and only if □X is valid in all S5 models").
- Rules: Smullyan alpha/beta rules; Table 4 T-rule `ν / ν0` ; the Destructive Rule (Table 6) `S, π / S#, π0` "if the set of formulas on a tableau branch matches the pattern … that entire branch is replaced with the set shown below the line" with S#= {X in S | X modalized} or S# = {X in S | X trivially modalized (a ν or π formula)} (two versions, Table 7). π0 = the diamond-less component.
- Soundness: Prop. 4.2 (satisfiable tableau stays satisfiable). Completeness: Lindenbaum argument; Prop. 5.4 "Suppose S is consistent and ¬□X ∈ S. Then S# ∪ {¬X} is also consistent"; Theorem 5.5 (Truth Lemma) builds the model G = all maximal consistent extensions of C*, "Since X is not provable … {¬□X} is consistent; it is also modalized" ⇒ countermodel. No backtracking needed for S5 ("no matter which of these two we work with, the other remains"), unlike S4 (section 6). Termination is not argued in the paper (it only remarks NP vs PSPACE).
- Weak completeness (box prefix / lifting) in Fitting 1983: SECONDARY via Goré (section 4.14.5, citing [Fit83, pp. 226, 229]): "Fitting then proves the curious fact that a single formula A is an S5-theorem if and only if a C_S5-tableau for {¬A} closes, and furthermore, that the (*) rule needs to be used only once at the beginning of the C_S5-tableau to lift ¬A to ¬□A [Fit83, page 229]. That is, the system C_S5* without the (*) rule is (weakly) complete for S5 in the sense that A is an S5-theorem if and only if a C_S5*-tableau for {¬□A} closes." Goré gives a constructive completeness proof (final cluster of the tree of clusters is symmetric; (S5) carries ¬□A into every node, so some member of the final cluster contains {¬□A, ¬A}) and notes "if ⊨_S5 A then ⊢_{S5*} □A". Fitting 1983 p. 201: tableaux for symmetric logics KB, KDB, KTB, S5 use the semi-analytic cut (sac) of Sonobe (SECONDARY).
How read: OLP (TEXT, secondary), Fitting 2003 paper (TEXT), Goré (TEXT) for the 1983 results. Fitting 1983, Fitting-Mendelsohn, Handbook chapter: NOT read.

---------------------------------------------------------------------
## F. Poggiolesi 2008 / Avron 1996 / Restall 2007

- Poggiolesi 2008: NOT read. ABSTRACT (Cambridge page): "theorem-equivalent to the Hilbert-style system S5, contraction-free, cut-free, decidable, all results proved syntactically". From a search-result snippet (not verified in the text): the calculus is a hypersequent calculus with "three modal rules", read by comparing a hypersequent with an S5 Kripke frame in which each sequent is a world; structural rules (including contraction) are height-preserving admissible, logical and modal rules height-preserving invertible. The explicit rules are NOT verified.
- Avron 1996: NOT read. (No PDF found.) From memory (not verified): hypersequent rules (□⇒) `G | Γ,A ⇒ Δ / G | Γ,□A ⇒ Δ`, (⇒□) `G | ⇒ A / G | ⇒ □A` for the S5 "splitting" version, plus external weakening and contraction, and a splitting/merging rule.
- Restall 2007 draft (TEXT; glyph □ lost in extraction, positions inferred from the surrounding discussion on p. 12-13 of the draft): hypersequent X1 ⊢ Y1 | … | Xn ⊢ Yn, a multiset of zones. Box rules (a single left/right pair, no separate "splitting" rule):
  - [□L]: `X, A ⊢ Y | Δ` / `□A ⊢ | X ⊢ Y | Δ`  (the box formula goes into a fresh zone of its own; the unboxed A joins the zone it came from) 
  - [□R]: `⊢ A | Δ` / `⊢ □A | Δ`  (only for a formula alone in an empty-antecedent zone)
  Structural rules: contraction inside zones, weakening only into a new zone ([KL], [KR]), a [merge] rule `X ⊢ Y | X' ⊢ Y' | Δ / X, X' ⊢ Y, Y' | Δ`, plus cut `X ⊢ A,Y | Δ , X',A ⊢ Y' | Δ' / X,X' ⊢ Y,Y' | Δ | Δ'`; cut-free completeness is by a search/countermodel argument ("the model … each zone describes a world"). Restall says a hypersequent corresponds to a model in which each zone is a world of the universal S5 frame ("Each zone partially describes a world").
- Equivalence with a labelled calculus without relational atoms: none of the three papers states it. It is an inference of mine (not in the sources) that a hypersequent X1⊢Y1|…|Xn⊢Yn is the labelled sequent with labels w1..wn and no R-atoms (R being universal), which is exactly the semantic reading that Restall gives and the reading of Fitting's integer prefixes in E. Brünnler (nested sequents, TEXT of the intro) states that nested sequents of depth one are hypersequents, and that "the only modal logic captured so far in the hypersequent calculus, that has not been captured in the ordinary sequent calculus, is the modal logic S5."

---------------------------------------------------------------------
## G. Wu & Goré, "Verified decision procedures for modal logics" (ITP 2019) (TEXT)

Calculi (all tableaux in negation normal form, formulas are sets, "AND-OR tableaux" with dotted-line transitional rules):
- K (Fig. 1): (id) `n, ¬n, Γ` unsat; (∧); (∨); (K) `◇φ, Σ, Γ / φ, Σ` (Σ = the □-formulas, with the equivalent form of Fig. 2 for all ◇ in the set: lower sequents `φ_i, Σ`).
- KT: Fig. 3, tableau KTT_H with a history Sigma (a formula set) written before the bar. The extraction shows (T) as `Σ | ϕ, Γ` over `ϕ, Σ | ϕ, Γ` and (K) as `Σ | ♦ϕ, Γ` over `∅ | ϕ, Σ`; the box and diamond glyphs are lost, so I do not reproduce the exact rules. What the text says: T is the loop-prone rule (the lower sequent is bigger, so no strict subformula property), the history records what T has already unboxed, and the transition rule starts a new world with an empty history. Termination for KT: "size(Σ | Γ) := (degree(Σ ∪ Γ), l(Γ))", lexicographic order (Def. 3.4, Thm 3.5); degree = maximal number of modal operators in a formula of the set. Soundness and completeness are direct semantic proofs (invertibility Thm 3.8), not a translation to HSZ.
- S4 (Fig. 4, S4T_H): sequent `A || S || H || Σ | Γ` with A ancestors (a list of non-empty signatures (ϕ, Σ)), S the signature of the sequent, H the history, Σ the boxed formulas. Rules (id), (∧), (∨), (□, new) `ϕ ∉ Σ`, (□, dup) `ϕ ∈ Σ`, and the transition rule (S4): `A || S || H || Σ | ♦ϕ, Γ` ↦ `(ϕ,Σ),A || (ϕ,Σ) || ϕ,H || Σ | ϕ,Σ` with side condition `ϕ ∉ H` — "The condition ϕ ∉ H in the transition rule is called a 'loop-check'". "H, S and A are called a history, a signature, and ancestors". H holds the diamond-formulas already used since the last time Σ changed, so a diamond-formula repeated with the same boxed set is cut off; Σ only grows.
- Termination of S4: Thm 3.11 (Σ, H are duplicate-free sublists of cl(Γ0); Γ ⊆ cl(Γ0)), Def. 3.12 size = (l(cl(Γ0)) − l(Σ), l(cl(Γ0)) − l(H), l(Γ)) ordered lexicographically, Thm 3.13 size(lower) <lex size(upper). They state that "the measure from HSZ could become negative in some cases" and so they use their own.
- Models: "All of these decision procedures return a concrete Kripke model in cases where the input set of formulas is satisfiable, and a proof constructed via the tableau rules witnessing unsatisfiability otherwise." For K and KT the model is a tree model (tmodel). For S4 a loop-check hit creates a "request" (backward edge) `request = {(ϕ,Σ) : ϕ ∈ H}` to an ancestor whose signature matches; the S4 model is `reach`, the reflexive-transitive closure of child and request edges (Defs. 3.15-3.18, Thms 3.19-3.21 "fulfillment"/"global invariant").
- Only K, KT, S4 are formalised ("K, KT and S4 in Lean"); backjumping only for K. No K4, no symmetric or euclidean logics. Formalisation at github.com/minchaowu/ModalTab (Lean 3). 
How read: TEXT; the rule glyphs ◇/□ were partly lost in extraction, my readings are marked.

---------------------------------------------------------------------
## H. A uniform terminating procedure with countermodels for all 15 cube logics

### Best primary source found: Brünnler, "Nested sequents", Habilitationsschrift (arXiv 1004.1845), Chapter 2 (TEXT)
- Covers every set X ⊆ {d, t, b, 4, 5} (the 15 cube logics: K, KD, KT, KB, K4, K5, KDB, KD4, KD5, KTB, KB4(=KB5), K45, KD45, S4, S5; Fig. 2.2). Statement: "systematic set of nested sequent systems for all normal modal logics formed by some combination of the axioms for seriality, reflexivity, symmetry, transitivity and euclideanness. We establish soundness and completeness … termination of proof-search".
- Calculus K + {◇d_c, ◇t_c, ◇b_c, ◇4_c, ◇5_c} (Fig. 2.3): rules ∧, ∨, □ `Γ{[A]}/Γ{□A}`, ◇k_c `Γ{◇A,[Δ,A]} / Γ{◇A,[Δ]}`, ◇d_c `Γ{◇A,[A]} / Γ{◇A}`, ◇t_c `Γ{◇A,A} / Γ{◇A}`, ◇b_c `Γ{[Δ,◇A],A} / Γ{[Δ,◇A]}`, ◇4_c `Γ{◇A,[Δ,◇A]} / Γ{◇A,[Δ]}`, ◇5_c `Γ{◇A}{◇A} / Γ{◇A}{∅}` (copy any ◇A not at the root to any other place, depth(Γ{}{∅})>0). One-sided, boxed sequents [..] are tree children; no labels. Incomplete for non-"45-closed" rule sets (Fact 2.10: {t,5} lacks □p ⊃ □□p, {b,4} lacks ◇p ⊃ □◇p), so one has to use a 45-closed set ({t,4,5} or {b,4,5} for S5).
- Procedure prove(Γ, X) (Fig. 2.6): Repeat (1) apply all propositional and "contraction-built-in" modal rules except □ and ◇d as long as possible, (2) apply □ and ◇d once wherever possible; until each non-axiomatic leaf is finished. Rules ρ° keep the principal formula and fire only if the SET sequent changes.
- Loop check (Def. 2.20): "A leaf of a sequent is cyclic if there is an inner node in the sequent that carries the same set of formulas. A node … is finished if no rule applies to a formula in this node. A sequent is finished if all its nodes are either finished or cyclic." This is EQUALITY of the underlying formula SET of a leaf with some inner node (not subset).
- Lemma 2.22 (Termination): at most 2^|sf(Γ)| iterations of the repeat-until loop (the number of different formula sets).
- Thm 2.23(ii): "If prove(Γ,X) fails then there is a finite X-frame in which Γ is not valid." Model construction: take the finished sequent Γ*, cyclic leaves Y, set S = subtrees minus Y, map f sends a cyclic leaf to the inner node with the same set of formulas, relation → on S: Δ → Λ iff Λ is an immediate subtree of Δ, or Δ has a cyclic child Σ with f(Σ) = Λ; then take →_X, the least relation containing → with the properties in X (reflexive, symmetric, transitive, euclidean, serial closures). Claim 3 does the case analysis on X; K4 uses ◇4_c, K5/K45/KB5 use Lemma 2.17 (the euclidean and transitive-euclidean closures are characterised by "euclidean connections" s ← s1 ↔ … ↔ sn → t) and ◇5_c, serial logics use the serial closure (adds s → s to dead ends; Lemma 2.19 shows serial closure preserves the other conditions).
- Cor. 2.24 (finite model property) and Cor. 2.25 (decidability) for all X.
- This is a nested-sequent procedure (and it is cyclic by equality of sets). It is the closest to "one uniform terminating procedure with finite countermodels for all fifteen logics". Caveats: no labels or relational atoms, ◇5_c copies formulas across the tree; the second S5 axiomatisation needs care (two 45-closed sets).

### Labelled alternative: Massacci [Mas94, Mas95b] as described by Goré (section D above). Periodicity by EQUALITY of label formula sets along a chain, with sleeping/waking of formulas; full proof not read.

### Horrocks, Sattler, Tobies, "Practical reasoning for very expressive description logics" (TEXT)
- Section 3.2 (logics without inverse roles): "For logics without inverse roles, the general procedure is to check the label of each new node y, and if it is a subset [2] of the label of an ancestor node x, then no further expansion of y is performed: y is said to block x" (the extraction swaps names; sense: the new node y is blocked by its ancestor x). "The resulting tree corresponds to a cyclical model in which y is identified with x." For a logic with transitive closure, subset blocking additionally needs a "good cycle" check (Fig. 2).
- Section 3.3 (inverse roles): "Besides using label equality instead of subset, this strategy [dynamic blocking] allowed blocks to be established, broken, and re-established. With inverse roles the blocking condition has to be considered more carefully because roles are now bi-directional, and additional concepts in x's label could invalidate the model with respect to y's predecessor. This problem can be overcome by allowing a node x to be blocked by one of its ancestors y if and only if they were labelled with the same sets of concepts."
- Section 3.4: pair-wise blocking for SHIF (functional restrictions, no finite model property).
- Section 4.1 definition (SHIF): "A node x is directly blocked iff none of its ancestors are blocked, and it has ancestors x', y and y' such that (1) x is a successor of x' and y is a successor of y' and (2) L(x) = L(y) and L(x') = L(y') and (3) L(<x',x>) = L(<y',y>). In this case we will say that y blocks x." Indirectly blocked: an ancestor is blocked, or it is a successor of x with an empty edge label. Expansion rules do not apply to blocked nodes (the ∃-rule only to non-blocked, ∀-rules to non-indirectly-blocked).
- Section 4.2 (SI, PSpace): "blocking no longer involves two pairs of nodes with identical labels but only two nodes with 'similar' labels" and a second label B per node is used for the PSpace bound (ideas of [46], K4 with tense).
- Why subset blocking is unsound with inverse/symmetric accessibility. What the paper says: only the sentence quoted above (extra concepts in the blocking ancestor's label can invalidate the model with respect to the blocked node's predecessor). My own explanation, in modal-logic terms (own reasoning, not quoted from any source, consistent with the sentence): when x is blocked by y and the model identifies x with y, y becomes a successor of x's parent p. With symmetric (or inverse) accessibility every box formula □D in L(y) constrains ALL neighbours of y, so after the identification D must hold at p. If L(x) ⊊ L(y), the formulas in L(y) \ L(x) were never propagated to p, so D can fail at p. With L(x) = L(y) every box formula of y was already applied to p when x was expanded, so the identification is safe. (For a pure forward-looking logic like K4 or S4 the extra formulas in y only constrain y's successors, which x's loop already satisfies because y ⊇ x, so subset blocking is fine; this is exactly why Goré's S4 uses subset on box formulas and GGN's blocking uses subset.) 
- Honest caveat: even equality of single labels is only enough for ALCI/SI-style logics without counting; for the SHIF/SHIQ fragments with functional restrictions the paper needs the pair-wise condition. For the modal cube (no counting) equality of the label (formula set) with an ancestor (plus the labels of the edge/neighbour relations, trivial in the monomodal case) is the relevant condition. I did not find a source proving this for the cube logics with euclideanness (K5, K45) other than Brünnler's equality-cyclic set check and Goré's/Massacci's periodicity.

---------------------------------------------------------------------
## What was NOT verified
1. Heuerding-Seyfried-Zimmermann 1996: no full text; rules, history contents and termination claim only through Goré 1999, Wu-Goré 2019 and Howe 1997; K4 coverage unknown.
2. Poggiolesi 2008 and Avron 1996: not read; rules for box not verified; equivalence with labelled calculi without relational atoms is not stated in anything I read.
3. Fitting 1983, Fitting-Mendelsohn 1998, Fitting's Handbook chapter "Modal proof theory": not obtained; their S5 rules via Open Logic Project and Goré; the S5 size/termination argument is from memory.
4. Massacci [Mas94, Mas95b] (proof of decidability of the labelled systems): not read; Goré only sketches the periodicity method.
5. Negri 2009: read the Helsinki preprint, not the printed chapter in "Acts of Knowledge".
6. Negri 2005 section 6 is a minimal-derivation counting argument; the K4 and Eucl/Ser cases are simply absent (not "false"): I did not search for other papers proving termination for G3K4 etc. (e.g. later work by Negri/Garg/Viganò/Andrikonis; a search snippet mentions Andrikonis 2011 and Viganò 2000, not read).
7. Some symbol glyphs (box, diamond, Γ, Δ, sequent bars) were lost in pdftotext for Negri 2005/2009, Goré, Wu-Goré, Restall; rules are restored from context.
8. GGN: the exact direction of the tree-order symbol was lost in the extraction; I take "y ≼ x" in the blocking condition to mean y is a proper ancestor of x (consistent with the introduction: "a world y (y ≠ x) such that y ≼ x and Sfor(x) ⊆ Sfor(y)").
