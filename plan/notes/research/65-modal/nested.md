# Research: nested sequents / prefixed tableaux for the modal cube (primary-source reading)

Notation used below: box = □ (pdftotext prints "2"), diamond = ◇ (pdftotext prints "3"), empty = ∅.
All papers were fetched as PDFs and converted with pdftotext; unless a line says otherwise,
"READ" means I read the statement in the full text of the source. Working copies are in
`.../scratchpad/nested-dl/` (dssml.txt, hab.txt, fitting.txt, massacci.txt, modal_prover.txt, readme.txt, gr.txt).

## Source access summary

| Source | Copy read | Status |
|---|---|---|
| Brünnler, "Deep sequent systems for modal logic", Arch. Math. Logic 48(6), 2009 | https://sirius.bfh.ch/~kai/Papers/2009dssml.pdf (author's manuscript, "will be inserted by the editor" header, 1394 lines of text) | READ in full text. Not the typeset journal version: numbering (Def./Lemma/Theorem/Fig.) is the manuscript's; pagination differs from the journal's pp. 551-577. |
| Brünnler, "Nested Sequents", Habilitationsschrift, arXiv:1004.1845 (v1, 2010) | https://arxiv.org/pdf/1004.1845 | READ Ch. 2 (systems, completeness, cut elimination, structural rules). Did not read Ch. 3 (common knowledge). |
| Fitting, "Prefixed tableaus and nested sequents", APAL 163(3):291-313, 2012 | author's posting version dated September 14, 2011: https://id144254.securedata.net/melvinfitting/bookspapers/pdf/papers/NestedSequentsPostingVersion/PrefixedDeep.pdf | READ Sections 1-7 in full text (not sections 8-16, which are quantification and justification logics; only skimmed headings). Posting version, may differ slightly from the published one. |
| Massacci, "Single step tableaux for modal logics...", J. Autom. Reasoning 24:319-364, 2000 | The logic.amu.edu.pl copy has a TLS certificate for the wrong host, so I did not fetch it from there (an attempt with certificate checks off was refused by the sandbox, and I did not retry). I read the same file through a Wayback Machine snapshot (https://web.archive.org/web/2023/https://logic.amu.edu.pl/images/a/a4/Singlestep.pdf), 46 pages, the journal typesetting (journal page numbers visible). | READ in full text (Sections 1-11 and most of the appendix headings; I rendered pages 326-327 as images to check Fig. 1c and Table II). |
| Girlando & Straßburger, "MOIN: a nested sequent theorem prover for intuitionistic modal logics (system description)", IJCAR 2020, LNCS 12167 | author copy https://www.lix.polytechnique.fr/~lutz/papers/modal_prover.pdf (HAL itself is behind an anti-bot wall) | READ in full text (8 pages). |
| MOIN page and `readme.txt` | https://www.lix.polytechnique.fr/~lutz/Software/Moin/MoinProver.html and .../readme.txt | READ (page via WebFetch summary, readme in full). I did NOT download or read the Prolog source (`moin.zip`). |
| Marin & Straßburger ESSLLI 2022 handout (nested sequents) | https://www.lix.polytechnique.fr/~lutz/orgs/ESSLLI22notes-5B-10-NestedSequents--Handout.pdf | Fetched, but it is a 2-page slide handout (202 lines of text); it only cites Brünnler for K and for the modal cube propagation rules. Nothing beyond Brünnler found there. Not used. |
| Goré & Ramanayake, "Labelled tree sequents, tree hypersequents and nested (deep) sequents", AiML 9 (2012) pp. 279-299 | http://www.aiml.net/volumes/volume9/Gore-Ramanayake.pdf | READ in full text. |

---

## A. Brünnler 2009 (and habilitation): the systems

### A1. The nested sequent system K (one-sided, negation normal form) — READ, manuscript Section 2, Fig. 3 (habilitation Fig. 2.3, "System K + {◇d_c, ◇t_c, ◇b_c, ◇4_c, ◇5_c}")

Formulas (Sec. 2): `A ::= p | p̄ | (A ∨ A) | (A ∧ A) | ◇A | □A`; negation is defined by De Morgan; ⊥ := p ∧ p̄, ⊤ := p ∨ p̄.
Nested sequent: finite **multiset** of formulas and boxed sequents `[Γ]`: `A1,...,Am,[Δ1],...,[Δn]`.
Corresponding formula: `A1 ∨ ... ∨ Am ∨ □(D1) ∨ ... ∨ □(Dn)` (⊥ for the empty sequent).
Contexts `Γ{ }` with holes; `Γ{ }{ }` is a binary context (used only by rule 5).

System K = {∧, ∨, □, k}, plus the axiom (a proof leaf is an instance of `Γ{p, p̄}`):

```
ax:  Γ{p, p̄}

∧:   Γ{A}   Γ{B}          ∨:   Γ{A, B}
     ------------              ----------
       Γ{A ∧ B}                 Γ{A ∨ B}

□:   Γ{[A]}               k:   Γ{◇A, [Δ, A]}
     --------                  ---------------
      Γ{□A}                     Γ{◇A, [Δ]}
```
(Read bottom-up: □ opens a fresh child containing A; k puts A into an existing child while keeping ◇A, i.e. the rule has built-in contraction. No eigenvariable-type side condition on □: the child is always new.)

Rules added for each axiom (Fig. 3 of the manuscript; READ):

```
d:  Γ{◇A, [A]}        t:  Γ{◇A, A}         b:  Γ{[Δ, ◇A], A}
    ------------          -----------          -----------------
    Γ{◇A}                 Γ{◇A}                Γ{[Δ, ◇A]}

4:  Γ{◇A, [Δ, ◇A]}              5:  Γ{◇A}{◇A}
    ---------------                 ------------   provided depth(Γ{ }{∅}) > 0
    Γ{◇A, [Δ]}                      Γ{◇A}{∅}
```
Reading bottom-up:
- d: a diamond formula gets a child containing A (so the node gets a child);
- t: the diamond's argument is added to the same node;
- b: a diamond formula inside a child gives A to the parent node;
- 4: a diamond formula is copied into each child (that is, ◇A moves down into existing children, keeping itself); the rule is conclusion `Γ{◇A,[Δ]}` -> premise `Γ{◇A,[Δ,◇A]}`;
- 5 (two-hole context): bottom-up, a formula ◇A that occurs at a node that is NOT the root (the proviso depth(Γ{ }{∅})>0 is the depth of the first hole, where ◇A sits in the conclusion) is copied to any other node of the sequent (including the root). Habilitation p. 9: "take some formula ◇A, which is not at the root, and copy it to any other place in the sequent". The manuscript, Lemma 2, says 5 is derivable from three one-hole rules 5a, 5b, 5c (with contraction):
  `5a: Γ{[Δ],◇A} / Γ{[Δ,◇A]}`, `5b: Γ{[Δ],[Λ,◇A]} / Γ{[Δ,◇A],[Λ]}`, `5c: Γ{[Δ,[Λ,◇A]]} / Γ{[Δ,◇A,[Λ]]}`.
  Remark in the K45 case of Claim 3: "(It is sufficient to have the 5a-rule instead of the 5-rule for all X which contain 4.)" (the habilitation names it "◇5^1_c").

Names: the manuscript calls these rules d, t, b, 4, 5; the habilitation names them ◇d_c, ◇t_c, ◇b_c, ◇4_c, ◇5_c ("c" = built-in contraction) and also gives the same rules without built-in contraction (Fig. 2.5, e.g. ◇4: `Γ{[Δ,◇A]} / Γ{◇A,[Δ]}`), related by Prop. 2.5.

Properties (Lemma 1 of the manuscript, habilitation Lemma 2.4; READ): for every X ⊆ {d,t,b,4,5}, in K+X the rules nec, wk, ctr are depth-preserving admissible and all rules are depth-preserving invertible. Soundness (Thm 1(iii)): K + X ⊢ Γ implies Γ X-valid, for **every** X (no restriction).

Interesting detail from the 5-rule soundness: for 5c a euclidean countermodel of the conclusion is one of the premise (uses t→t, u→t, t→w).

### A2. The structural variants — READ, manuscript Section 4 (Fig. 7) and Section 5; habilitation Fig. 2.7 / Fig. 2.8 and Sec. 2.2

Manuscript Fig. 7 "Modal structural rules" (used in cut elimination; they are not diamond rules):
```
ḋ:  Γ{[∅]} / Γ{∅}          ṫ:  Γ{[Δ]} / Γ{Δ}        ḃ:  Γ{[Δ,[Σ]]} / Γ{Σ,[Δ]}
4̇:  Γ{[Δ]} / Γ{[[Δ]]}      5̇:  Γ{[Δ]}{∅} / Γ{∅}{[Δ]}     provided depth(Γ{ }{[Δ]}) > 0
```
Habilitation Fig. 2.7/2.8 gives a different (equivalent-looking) form for [4]: `Γ{[Δ],[Σ]} / Γ{[[Δ],Σ]}`, [b]: `Γ{[Δ,[Σ]]} / Γ{[Δ],Σ}`, [t]: `Γ{[Δ]}/Γ{Δ}`, [d]: `Γ{[∅]}/Γ{∅}`, [5]: `Γ{[Δ]}{∅} / Γ{∅}{[Δ]}` (depth(Γ{ }{∅})>0), in the system `K_c + [X]` that has an explicit ctr rule and has box, k (without contraction: `Γ{[A,Δ]} / Γ{◇A,[Δ]}`), nec... 

Modularity status:
- Manuscript Sec. 5, Conjecture 1: for every X ⊆ {ḋ, ṫ, ḃ, 4̇, 5̇}, `K + X + {ctr, wk} ⊢ Γ iff Γ is X-valid` (no "45-complete" proviso). Manuscript Theorem 6 proves it only for X among {ḋ}, {ṫ}, {ḃ}, {4̇} (four singletons, four different proofs). Quote: "A general method to uniformly prove completeness for all combinations of the modal structural rules is subject of current research."
- Habilitation Theorem 2.38 (Completeness, READ): for **every** X ⊆ {d,t,b,4,5}, any formula provable in the Hilbert system K+X is provable in `K_c + [X]` (proof: Hilbert proof -> proof with cut -> cut elimination, Thm 2.37). So the structural-rule systems are modular and complete for arbitrary X; but nothing in the habilitation gives a terminating proof search for them (the proof-search/countermodel machinery in Ch. 2.1 is for the diamond-rule systems only; I found no termination result for `K_c + [X]` — I searched for "terminat" and found only Lemma 2.22 and the remark in the manuscript's Theorem 6 proof about the {4̇'} system).
- The manuscript's termination argument for {4̇} (Thm 6 proof): uses the variant `4̇': Γ{[Δ,[Λ]],[Λ]} / Γ{[Δ,[Λ]]}` and a **modal-depth** bound ("No rule, seen bottom-up, increases the modal depth of the sequent... the size of any sequent occurring in a proof is bounded").

### A3. Does the system work for an arbitrary subset X? — READ

No, only for 45-closed (manuscript: "45-complete") X.

Manuscript, Section 3, before Def. 9: "The current set of modal rules does not allow a modular completeness result of the form 'if Γ is X-valid then K + X ⊢ Γ'. In particular it is easy to check that we have Fact 2: For some formula A we have (i) K + {t, 5} ⊬ □A ⊃ □□A and (ii) K + {b, 4} ⊬ ◇A ⊃ □◇A."
(Habilitation Fact 2.10: for any propositional variable p, □p ⊃ □□p holds in every {t,5}-frame and ◇p ⊃ □◇p in every {b,4}-frame, but K+{◇t_c,◇5_c} ⊬ □p ⊃ □□p and K+{◇b_c,◇4_c} ⊬ ◇p ⊃ □◇p.)

Definition 9 (manuscript; "45-complete") = habilitation Definition 2.11 ("45-closed"): "Let X ⊆ {d, t, b, 4, 5}. The set X is 45-complete [45-closed] if for ρ ∈ {4, 5} we have that if all X-frames satisfy ρ then ρ ∈ X." Examples: {t,5} and {b,4} are not; {t,4,5} and {b,4,5} are.
Habilitation Def. 2.46 / Fact 2.47 gives the closure explicitly: `X+ = X ∪ {4}` if {t,5} ⊆ X or {b,5} ⊆ X; `X ∪ {5}` if {b,4} ⊆ X; `X` otherwise; "X+ is the least set which contains X and is 45-closed."
Statement in both texts: every class of frames characterisable by the five axioms is characterised by some 45-closed set ("The restriction to 45-complete sets ... is thus irrelevant"); "each modal logic has exactly one 45-complete axiomatisation, with the exception of S5, which has two" (S5 = {t,4,5} or {d,b,4,5}). (My remark, not in the source: strictly, {d,t} is also 45-closed and gives KT, so "exactly one" is true only if d is dropped when t is present; the printed case list in Claim 3 does not list {d,t,...} combinations separately.)

Resulting 45-closed rule sets for the fifteen logics, as listed in the proof of Thm 3 (Claim 3 cases):
K=∅, KD={d}, KT={t}, KB={b}, K4={4}, K5={5}, KDB={d,b}, KD4={d,4}, KD5={d,5}, KTB={b,t}, KB5(=KB4)={b,4,5}, K45={4,5}, KD45={d,4,5}, S4={t,4}, S5={t,4,5} or {d,b,4,5}.

### A4. The exact completeness theorem — READ

Manuscript Theorem 3 (Completeness) (habilitation Thm 2.23, with "45-closed" and a stronger (ii)):
"For all 45-complete sets X ⊆ {d,t,b,4,5} and for all sequents Γ the following hold: (i) If Γ is X-valid then K + X ⊢ Γ. (ii) If prove(Γ, X) fails then Γ is not X-valid."
Habilitation version: "(i) If Γ is X-valid then K + ◇X_c ⊢ Γ. (ii) If prove(Γ,X) fails then there is a **finite X-frame** in which Γ is not valid."
Corollaries: Cor. 1 (manuscript) / Cor. 2.25: for all X ⊆ {d,t,b,4,5} X-validity is decidable; habilitation Cor. 2.24: finite model property.
Cut elimination: manuscript Theorem 5 / hab. Thm 2.35: for 45-complete X, K+X+cut ⊢ Γ implies K+X ⊢ Γ (cut-rank-preserving structural rules; d needs the ḋ detour).
Both statements are about the rule sets K+X with X 45-complete.

---

## B. Brünnler: proof search, termination, countermodel — READ

### B1. The auxiliary systems (K+X)° (manuscript Sec. 3 before Def. 4; habilitation before Lemma 2.14)

Completeness is proved for systems S° obtained by replacing each rule ρ with ρ° which **keeps the main formula** in the premise. For most rules ρ° = ρ except:
```
∧°: Γ{A∧B, A}   Γ{A∧B, B}      ∨°: Γ{A∨B, A, B}
    -------------------------       --------------
          Γ{A∧B}                        Γ{A∨B}

□°: Γ{□A, [A]}  where in the conclusion the node of the active formula does not have a child node which contains A
d°: Γ{◇A, [A]}  where in the conclusion the node of the active formula does not have a child node.
```
"In addition, each rule ρ° carries the proviso that for all of its premises the set sequent is different from the set sequent of the conclusion." Set sequent (Def. 3): the underlying set of the sequent, recursively (multisets collapsed to sets at every node). Lemma 4: (K+X)° ⊢ Γ implies K+X ⊢ Γ (by contraction and weakening admissibility).

### B2. The algorithm (Fig. 5) and the loop check (Def. 7), READ verbatim

```
Repeat
  (step 1) Keep applying the rules in ((K + X) \ {□, d})° as long as possible.
  (step 2) Wherever possible, apply the rules in ((K + X) ∩ {□, d})° once.
Until each non-axiomatic derivation leaf is finished.
```
Definition 7: "A leaf of a sequent is **cyclic** if there is an inner node in the sequent that carries the same set of formulas. A node in a sequent is **finished** for a system S if no rule from S applies to a formula in this node. A sequent is finished for a system S if all its nodes are either finished for S or cyclic."
Note the wording: the comparison is with an *inner node of the sequent* (any non-leaf node of the same nested sequent tree, not only an ancestor on the path; for a tree the natural reading is "an inner node" is any node with a child; the text does not say "ancestor"). A *leaf* of the sequent (tree) = a node without children. prove(Γ,X) "terminates and all derivation leaves are axiomatic then it succeeds; if it terminates and there is a non-axiomatic derivation leaf then it fails."

Lemma 7 (Termination): "For all sets X ⊆ {d,t,b,4,5} and for all sequents Γ the procedure prove(Γ,X) terminates after at most 2^{|sf(Γ)|} iterations (of the repeat-until-loop)." (pdftotext prints the exponent flat as "2|sf(Γ)|"; this is the number of different sets of formulas per node.) Proof: step 1 grows some node's formula set strictly and creates no nodes; step 2 strictly grows the sequent's size; only 2^|sf| different node-sets exist, so every unfinished leaf eventually becomes cyclic.

### B3. For which of the fifteen logics is the cyclic check needed?

The text does not say. The definition and Lemma 7 are uniform in X: the same loop check is used for all X ⊆ {d,t,b,4,5}, K included. I found no sentence in the manuscript or habilitation that says "K and T need no loop check". Related statements elsewhere:
- Brünnler, manuscript Thm 6 proof (only for the structural {4̇'} system): termination via modal depth, no loop check.
- Massacci 2000 (Sec. 8.2, Table IV): for K, D, T, KB, KDB, B the longest prefix needed is `1 + d` (d = modal depth), so a pure depth bound (no loop check) terminates; the loop check / pruned-copy argument is needed for K4, KD4, S4 (bound `2 + dp + p·n`); K45/KD45/S5 need none beyond "reduce each π-formula once" (Sec. 9.2). Brünnler's nested sequents are the same tree structure, so this suggests (my inference, not stated by Brünnler) that without 4 and 5 the depth bound makes the cyclic check unnecessary. NOT verified in Brünnler.
- MOIN paper, Sec. 5: "All the classical modal logics in the S5-cube are decidable, and a simple loop check in the rules that create a new nesting is enough to ensure termination (see [3] for details)." (the loop check there is on the rules that create a new nesting: □, d.)

### B4. Countermodel construction (proof of Thm 3(ii)), READ verbatim (manuscript; the habilitation's version is identical)

"For (ii) we define a model M on an X-frame for which we prove that it is a countermodel for Γ. Let Γ* be the set sequent of the non-axiomatic finished sequent obtained. Let Y be the set of all cyclic leaves in Γ*. Let S = st(Γ*) \ Y. Let f : Y → S be some function which maps a cyclic leaf to a sequent in S whose root carries the same set of formulas and extend f to st(Γ*) by the identity on S. Define a binary relation → on S such that Δ → Λ iff either 1) Λ is an immediate subtree of Δ or 2) Δ has an immediate subtree Σ ∈ Y and f(Σ) = Λ. Let V(p) = {Δ ∈ S | p̄ ∈ Δ}. Let M = (S, →_X, V)."
- Worlds = the subtrees (nodes) of the final non-axiomatic finished leaf, set-ified, minus the cyclic leaves. Each cyclic leaf is redirected: the edge from its parent goes to a node of S carrying the same set of formulas ("f"), so the model is a tree plus back/forward edges.
- Valuation: p true at a node iff p̄ is in that node (the failed sequent is a disjunction, countermodel makes every formula in it false).
- Accessibility: →_X "denotes the smallest relation that includes → and has the properties in X" (Def. 4: for X ⊆ {t,b,4,5} →_X is the smallest relation including → with those properties: reflexive, symmetric, transitive, euclidean closure; Lemma 5: → ∪ →5 is the least euclidean relation containing →; →45 is the least transitive and euclidean one; Lemma 5(i) shows well-definedness). Euclidean connection: `s ← s1 ↔ s2 ↔ ... ↔ sn → t`; transitive-euclidean connection: `s = s1 ↔ s2 ↔ ... ↔ sn → t`.
- Seriality (Def. 6): "Its serial closure, denoted →^d, is obtained from → by adding s → s for each s ∈ S which violates seriality. For X ⊆ {t,b,4,5} the relation →_{X∪{d}} is defined as (→_X)^d." (Lemma 6: serial closure preserves t, b, 4, 5.) So dead-end worlds get a self-loop *after* the closure under the other conditions.
- Three claims: Claim 1: for each Δ in st(Γ*), M, f(Δ) ⊭ Δ. Claim 2: for each Δ ∈ S and each A ∈ Δ, M,Δ ⊭ A. Claim 3: if Δ →_X Λ and ◇A ∈ Δ then A ∈ Λ (proved case by case over the 45-complete X listed in A3: this is where each rule's role appears: k for immediate children, t for Δ=Λ, b for Λ→Δ, 4 for paths, 5/5a for euclidean connections via a common parent, d for the serial loop).
- Finite model: S is finite; habilitation Cor. 2.24 (finite model property) states "a finite X-frame".

Not in the text: what the model looks like as a diagram for the cyclic leaf beyond the above; no minimisation; the choice of f is "some function" (any node of S with same set of formulas).

---

## C. Fitting 2012 — READ (posting version, Sections 1-7)

Claim (abstract): "We show that modal nested sequents and prefixed modal tableaus are notational variants of each other, roughly in the same way that Gentzen sequent calculi and tableaus are notational variants. This immediately gives rise to new modal nested sequent systems which may be of independent interest."

What is proved, with locators:
- Sec. 4 ("Turning tableaus over"): prefixed tableau -> "prefix sequents" (a backward/forward inversion of tableau rules).
- Sec. 5, Def. 5.1: translation N(S) of a set S of prefixed formulas to a nested sequent: formulas with the same prefix go in one nested sequent; the nested sequent of formulas with prefix σ.n is a member of that of σ. Each prefix sequent rule translates to the matching nested sequent rule (ν -> `Γ([ν0])/Γ(ν)` because "σ.n does not occur in S" forces a boxed sequent with nothing else in it; π -> `Γ(π,[π0,...])/Γ(π,[...])`). Fitting says the translation is "quite transparent, so we do not give a formal proof of correctness"; a worked example (Fig. 2 -> Fig. 3).
- Sec. 6 ("Going the other way"): nested proofs -> prefixed tableau proofs, harder because "both {1 X, 1.1 Y, 1.2 Z} and {1 X, 1.1 Z, 1.2 Y} translate to the nested sequent {X,{Y},{Z}}"; solved by annotating subsequent occurrences with prefixes over the whole proof (an "annotation": mapping from subsequent occurrences to prefixes meeting conditions) and translating whole proofs, not lines.
- Soundness/completeness of the nested systems is transferred from prefixed tableaux (or by Brünnler's arguments, sketched in Sec. 3); no separate cut-elimination: "Fitting does not prove syntactic cut-admissibility for his prefixed tableaux" (Goré-Ramanayake Related Work).
- Differences from Brünnler's system (Sec. 3): sets instead of multisets, no structural rules, arbitrary formulas (not NNF), and **empty nested sequents are not allowed** ("The connection with prefixed tableaus is simpler if we do not do so").

Logics covered (Sec. 7, chart on p. 15, READ): prefixed tableau rules T, D, B, 4, 4r (Fig. on p. 15; B here is "σ.n ν / σ ν0", 4r is "σ.n ν / σ ν"), with the table:
KD = D; KD4 = D,4; T = T; K4 = 4; KB = B; KDB = D,B; "B" (= KTB) = B,T; S4 = T,4; S5 = T,4,4r.
That is: K, KD, KT, K4, KB, KDB, KTB, S4, S5 only. **K5, KD5, K45, KD45, KB5(KB4) are not treated.** The text notes: "though one might expect that Prefixed Tableau Rules T, 4, and B would combine to give a system for S5, they do not. A similar thing happens with nested sequents, and was noted in [4, 5]."
Nested sequent rules given (from Brünnler, without NNF): t `Γ(π,π0)/Γ(π)`, d `Γ(π,[π0])/Γ(π)`, b `Γ(π0,[π,...])/Γ([π,...])`, 4 `Γ(π,[π,...])/Γ(π,[...])` (with [...] a *non-empty* nested sequent), and for S5 the rule **4r**: `Γ(π,[π,...]) / Γ([π,...])` ("which allows us to eliminate a π occurrence outside a boxed sequent... translates the Euclidean condition"): "Adding Nested Sequent Rules t, 4, and 4r to the basic system for K produces a system for S5. The exact relationship between 4r and 5 might be interesting to explore." (Brünnler's 5-rule has two holes; Fitting notes it is derivable from 5a-5c).
Also: Fitting's d-rule variant: the tableau rule `σ ν / σ.n ν0` with the side condition "σ.n already occurs" dropped ("it may also be new") translates back from the nested d rule.
No termination/complexity claims in Sec. 1-7 (only soundness/completeness; the "systematic tableau construction" is cited to his earlier books).

---

## D. Massacci 2000 — READ (journal version, via Wayback)

### D1. The rules (Fig. 1a-1c, p. 326; Table II, p. 327; images checked)

Language: ∧, ¬, □; prefixes are nonempty integer sequences; "L-present", "new". Rules:
```
Propositional (Fig 1a):  α: σ:A∧B / σ:A , σ:B     β: σ:¬(A∧B) / σ:¬A | σ:¬B     dneg: σ:¬¬A / σ:A
Fig 1b (all logics):
  Loc:  ... / 1:B   if B ∈ U (local assumptions)
  Glob: ... / σ:B   if σ present and B ∈ G (global assumptions)
  π:    σ:¬□A / σ.n:¬A   with σ.n new in the branch
Fig 1c ("SST-rules characterizing logics"; σ, σ.n, σ.n.m must be present in the branch):
  K:    σ:□A / σ.n:A          D:   σ:□A / σ:¬□¬A          T:   σ:□A / σ:A
  4:    σ:□A / σ.n:□A         4^R: σ.n:□A / σ:□A         B:   σ.n:□A / σ:A
  4^π:  σ.n:¬□A / σ:¬□A       4^D: σ.n:□A / σ.n.m:□A     Cxt: σ.n:□A / σ:□□A
```
(4^R is the backward 4; B in Massacci's Table I is the axiom ◇□A ⊃ A, i.e. the usual b; 4^π encodes ◇◇A ⊃ ◇A; Cxt is McCarthy's contextual logic.)

Table II "Modal logics and SST-rules" (READ, image-checked):
```
K:    K                    KD:   K, D              T:    K, T
KB:   K, B                 KDB:  K, B, D           B:    K, T, B         (B = KTB)
K4:   K, 4                 KD4:  K, D, 4           S4:   [K], T, 4
K5:   K, 4^D, 4^R, Cxt     KD5:  K, D, 4^D, 4^R, Cxt     KB4:  K, B, 4^π, 4^R
K45:  K, 4, 4^R            KD45: K, D, 4, 4^R      S5:   [K], T, 4, 4^R
K45:  K, 4^π, 4^R          KD45: K, D, 4^π, 4^R    S5:   K, T, 4^π, 4^R
```
([K] = derivable, can be dropped: (K) simulated by (T) then (4).) Remark under the table: "For the logics K5 and KD5 we only need to apply rule (Cxt) to prefixes σ.n with σ = 1." The fifteen logics of the question are exactly the ones in the text of Sec. 3: "K, KB, K4, K5, K45, KD, KDB, KD4, KD5, KD45, KB4, T, B, S4, S5" (KB4 = KB5). Cxt itself is a sixteenth logic.
Modularity remarks (Sec. 6): no 1-1 axiom/rule mapping; "(4^R) alone cannot simulate the power of axiom 5"; "(K)+(T)+(4)+(B) is not enough to get completeness for S5. Overlooking this fact leads to incomplete formalizations. For instance the rules for K5 are not complete in [32], and the rules for S5 are not complete in [29]."
Theorems 4.1/4.2 (strong soundness/completeness, proofs in Sec. 10): for L among K, KB, K4, K5, K45, KD, KDB, KD4, KD5, KD45, KB4, T, B, S4, S5, Cxt: A has an L-tableau proof (closed tableau from 1:¬A, with local assumptions U, global G) iff G |=_L U => A. Sec. 10 conditions: Table V (rules and model properties), Table VI (syntactic relation conditions).

### D2. Proof confluence (Sec. 7, Thm 7.1)
"If ν-formulae can be reduced more than once, SST rules are strongly confluent modulo isomorphic renaming of prefixes." So no backtracking is needed over the choice of rule (unlike Gentzen-type calculi); for K45/KD45/S5 with 4^π the π-formulae must also be reducible more than once. Backtracking is only needed in the proof search over the branching of β.

### D3. Termination (Sec. 8) — what loop check, per logic
- Technique 4.1: apply a rule to σ:A only if the formula is not already "reduced" (Def. 4.3).
- Pure analyticity alone suffices for K (with empty global assumptions): "If every analytic rule introduces in the consequents only proper subformulae..., then technique 4.1 is the only thing we need to terminate. This is the case for Gentzen-type calculi ... for the logic K, if we restrict ourselves to validity". SST are analytic except for the logics with rule Cxt; for K5 "once-off" application of Cxt (restricted to prefixes σ.n with σ=1) keeps it super-analytic with S* = S ∪ □S.
- Loop check ("copy"): Def. 8.1: a prefix σ is a copy of σ0 in B if for every A, σ:A ∈ B iff σ0:A ∈ B. "A branch B is π-completed if (i) all prefixes are π-reduced in B, and (ii) for every σ that is not fully reduced there is a fully reduced copy σ0 shorter than σ." Technique 8.1: select formulas with the shortest prefix; Technique 8.2: before reducing a π-formula check that its prefix is not a copy of a shorter prefix (= Demri-type loop check). Thm 8.1: a π-completed branch gives satisfiability. For K4, S4: modal copies (same ν-formulae) and π-modal-completed (Def. 8.2); "This is not the case for logics KB and B" (footnote with counterexample G={◇(□B ∧ ◇◇¬B)}, A=⊤).
- **Loop check replaced by a depth bound (Technique 8.3, Table IV, Lemma 8.3, Thm 8.4)**, for validity / local consequence: "the application of the single step tableau π-rule is limited to prefixes whose length is less than hb_L":

| Logic L | bound hb_L |
|---|---|
| K, D, T, KB, KDB, B | 1 + d |
| K4, KD4, S4 | 2 + dp + p × n (the table) / "hb_L = 2 + d + n × p" (the sentence at the end of the sketch of the proof of Thm 8.4: the source is inconsistent between "dp" and "d") |

(d = max modal nesting in nnf(U ∪ {¬A}); n = number of ν-subformulae; p = number of π-subformulae; dp = max nesting of ◇ not under □; Table III.) Thm 8.4: every strategy with techniques 4.1 and 8.3 terminates in (1) a proof, or (2) every branch has an applicable rule left, or (3) at least one branch can be pruned (Pruning Lemma 8.2) into a π-(modal-)completed branch. Only these nine logics get Table IV bounds. **No bound or termination theorem is given for K5, KD5 or KB4 (KB5)**; K45, KD45, S5 are in Sec. 9.2. For global consequence: Thm 9.4 gives hb_L = 2 + (dp+pG) + (p+pG)(n+nG+fG) for K4 and S4; Thm 9.1/Cor 9.3: for K, KD, T, KB, KDB, B no polynomial bound is possible (else PSPACE = EXPTIME).
- K45, KD45, S5 (Sec. 9.2): Technique 9.1 (with rule 4) "apply rule (π) once to each π-formula ¬□A (no matter its prefix)"; Technique 9.2 (with rule 4^π) "apply rule (π) only to the prefix σ = 1". Lemma 9.5 (termination), Thm 9.6: branches have size polynomial in G, U, A ("at most p + pG + 1 different prefixes, which have length at most 1 + p + pG + 1").

### D4. Complexity results proved (Sec. 9)
Background (cited, not proved here): validity is PSPACE-complete for the modal logics between K and S4 [Ladner]; logical consequence (global assumptions) is EXPTIME-complete [Halpern-Moses]; K45/KD45/S5: validity is co-NP-complete, satisfiability NP-complete.
Proved/claimed in this paper:
- NP (co-NP validity) search for **K45, KD45, S5** with SST: Technique 9.1/9.2 + Thm 9.6 (polynomial-size branch); abstract: "Minimal conditions on SST search strategies are proven to yield PSPACE (and NPTIME for S5 and KD45) decision procedures." (the abstract lists S5 and KD45 only; the body covers K45, KD45, S5.)
- **PSPACE** strategy for **K, KD, T, K4, KD4, S4** (Thm 9.8: with techniques 4.1, 8.3 and the width bound T.9.4 the number of unreduced prefixes in a branch is polynomial; Technique 9.5 deletes confined, reduced prefixes; "Theorem 9.1 [Prop 9.1 / Local stability Lemma 9.7] guarantees such gaps are harmless"). Extended to **KB, KDB, B** by modified deletion Techniques 9.6/9.7 (reduce again ◇B formulas whose child was deleted when a new □A arrives; "reckless deletion of prefixes may lead to incomplete search strategies"); no separate theorem, only the techniques and the argument.
- Fact 9.6: "Any complete strategy for prefixed (single step) tableaux that never deletes formulae requires worst-case exponential space." Remark: depth-first with deletion gives polynomial space; breadth-first needs exponential space.
- Not covered by complexity statements: K5, KD5, KB4 (KB5).
- Global consequence section: decidable for K4, S4 (Thm 9.4), and general impossibility of polynomial local checks for K, KD, T, KB, KDB, B.

---

## E. MOIN — READ (paper + page + readme.txt)

Logics (readme "Axioms of the implemented logics"; paper Sec. 1, 2): "Classical modal logics: MOIN treats all the 15 logics in the S5-cube." (nested sequents of Brünnler [3], "we implemented his method in Moin"; the paper says classical details "follow straightforwardly" and does not describe them.) Intuitionistic: "all the decidable modal logics in the IS5-cube" via single-conclusion (Straßburger) and multi-conclusion (Kuznets-Straßburger) nested calculi; for IS4, IK4, ID4 decidability is open and termination is not ensured (readme warns; the page says only "prototype implementation").
Output: "MOIN implements a backwards proof search on the formula, yielding a derivation or, for proof systems 1 and 3, a countermodel." (1 = classical, 2 = single-conclusion intuitionistic: no countermodel, 3 = multi-conclusion intuitionistic.) Files: derivation `derivationNK.tex` (classical), `derivationNKs.tex`, `derivationNKm.tex`; countermodel `counterNK.tex` (classical), `counterNKm.tex`; to get a PDF compile `NK+X-result.tex` etc. with the Virginia Lake macros (virginialake.sty). Paper Sec. 5/6: "we also implemented the multi-conclusion systems... We follow the construction of [12] for obtaining a countermodel", and "In case of failure for classical and intuitionistic multi-conclusion sequents, Moin prints out a countermodel in a LaTeX file". Termination for the classical cube: "a simple loop check in the rules that create a new nesting is enough"; for intuitionistic logics with 5 the depth of the sequent is restricted to 1.
Input syntax (readme "Syntax"): atoms are Prolog atoms `a, b, c,...` and constants `false`, `true`; connectives `~A | A ^ A | A v A | A -> A | !A | ?A` = negation, conjunction, disjunction, (implication), **box = `!A`, diamond = `?A`**; "The usual conventions for parentheses apply." Paper Sec. 6: "the connectives ¬, ∧, ∨, ⊃, □ and ♦ are respectively represented by ~, v ^, ->, ! and ?." (the paper lists `v` and `^` for ∨, ∧.)
Naming the logic: in Prolog, `?- [moin].` then `?- derive(System, Axioms, F).` with System = `k` (classical nested), `i` (intuitionistic single-conclusion), `m` (intuitionistic multi-conclusion); Axioms = a list: `[]` = K/IK, `[d]`, `[t]`, `[b]`, `[4]`, `[5]`, and combinations such as `[t,b]` = KTB, "and so on". There is also `[d1]` for the intuitionistic rule d^[] (only intuitionistic single-conclusion). Example: `derive(i,[],!(a->b)-> ((!a)->(!b)) ).` and, from the paper, `derive(i,[b,5], (?a -> !?a) ^ (?!a -> !a))`. The shell script `./moin.sh System 'F'` takes System = `K`, `IKs`, `IKm`, and appends the axiom letters directly: "Add D, T, B, 4, 5 right after K to derive F in the corresponding extension of K or IK", example `./moin.sh IS5m '((?((!a -> !b) -> false) ^ ?(!c -> false)) -> !?!a)'` (readme's example says IS5 where the text said K/IKs/IKm; the readme is not consistent about the S5 spelling).
**Not verified:** (a) whether MOIN's classical mode takes the axiom list literally (so `[t,5]` would be the incomplete system of Brünnler's Fact 2) or closes it under 45; the readme and paper say nothing about 45-closure, and I did not read the Prolog source. (b) The exact output/format of the LaTeX countermodel. (c) Complexity or performance for classical logics (Sec. 7 benchmarks are for the intuitionistic variants only; Fig. 6 of the paper shows IS5 timings, tens of seconds for some formulas with ">150 s" entries).

---

## F. Goré & Ramanayake 2012 — READ

Abstract: "We identify a subclass of labelled sequents called 'labelled tree sequents' and show that these are notational variants of tree-hypersequents in the sense that a sequent of one type can be represented naturally as a sequent of the other type. This relationship can be extended to nested (deep) sequents using the relationship between tree-hypersequents and nested (deep) sequents, which we also show."
Definitions (Sec. 2): Def. 2.2 tree-hypersequent (THS): a sequent S, or `S/(G1;...;Gn)`. Def. 2.5-2.6: a relation set R is treelike if the frame it defines is a tree (or R empty); a labelled tree sequent (LTS) is `R, X ⇒ Y` with R treelike, if R = ∅ all labelled formulas carry one label, and if R ≠ ∅ every state variable occurring in X or Y also occurs in R.
Maps (Sec. 3): Def. 3.1 `TL_x(X⇒Y) = x:X ⇒ x:Y`; `TL_x(X⇒Y/G1;...;Gn) = ⊗_j TL_{xj}(Gj) ⊗ (Rxx1,...,Rxxn, x:X ⇒ x:Y)`; Def. 3.3 LT in the other direction. Lemma 3.5: (i) TL G is an LTS, (ii) LT S is a THS, (iii) LT(TL G) = G and TL(LT S) = S_f for some renaming f of S.
Statements (Sec. 3): **Lemma 3.9**: for a THS calculus C, for any THS G, ⊢_C G iff ⊢_{TL C} TL G; for any LTS S, ⊢_{TL C} S iff ⊢_C LT S; "In each case, the respective derivations in C and TLC have identical height." **Corollary 3.10**: for any THS calculus C and formula A: ⊢_C ⇒ A iff ⊢_{TL C} ⇒ x:A. Application: Theorem 4.4 (TLCSGL and G3GL prove the same formulas), giving cut admissibility for Poggiolesi's CSGL (provability logic GL).
Nested sequents: the correspondence with nested (deep) sequents is **not given as a numbered theorem**; it is in Section 5 (Conclusion): "It is straightforward to construct mappings between THS and the nested (deep) sequents that Fitting [5] refers to... (adapting to Brünnler's formulation of nested sequents is analogous)." Nested sequent = `Γ, [N1],...,[Nk]`; maps `TN(X⇒Y/(G1;...;Gk)) = (X ⇒_1S Y, [TN G1],...,[TN Gk])` and `NT(Γ,[N1],...,[Nk]) = (Γ_s/(NT N1;...;NT Nk))`; "In this way we can show that THS and nested sequents are notational variants." The proof is only asserted. The restriction: nested sequent rules operate at any depth of nesting but "not inside a formula". Related-work paragraph: "Fitting's work shows a to-and-fro correspondence between prefixed tableaux and Brünnler's deep sequents ... uses exactly the same term 'notational variants'". No decision procedure, termination, or model construction.

---

## Items not verified / caveats (summary)
1. Brünnler's manuscript is not the typeset journal text: numbering may differ slightly (Definition 9 "45-complete" in the manuscript and "Definition 2.11 (45-closed)" in the habilitation; same content). Exact journal Fig./Thm numbers not checked.
2. Brünnler never states in the text which of the fifteen logics need the cyclic-leaf check; my "K, T, KB, ... do not need it" statements are inferences from Massacci's depth bounds, marked as such. In the algorithm the loop check is used uniformly.
3. The definition of "cyclic" compares with "an inner node in the sequent" (any inner node), not specifically ancestors; I did not test whether a stricter ancestor-only check is also complete.
4. Fitting: only the posting version (2011); Sections 8-16 (quantification, justification logics) not read; no termination or complexity content in the part read.
5. Massacci: Table IV's bound for K4/KD4/S4 is printed as `2 + dp + p×n` but the proof sketch states `2 + d + n×p` (source inconsistency). No loop-check/bounds for K5, KD5, KB4 beyond the once-off Cxt remark; PSPACE for KB/KDB/B only by technique, not a numbered theorem. The paper's A-axiom "B" is the usual axiom b.
6. MOIN: Prolog source not read; classical output formats and handling of non-45-closed axiom lists unknown. The HAL record of the paper was blocked, I used the authors' LIX copy (same title, IJCAR 2020 LNCS 12167).
7. Goré-Ramanayake: nested sequent correspondence appears only as a sketched claim in Section 5; the THS/LTS part is Lemma 3.9 and Corollary 3.10.
8. Marin-Straßburger ESSLLI handout: contains nothing beyond citations to Brünnler (2 slides worth).
9. Other files in the scratchpad `dl/` directory (guide.pdf, hm.pdf, hr.pdf, hs.pdf, ladner.pdf) were written by something else, not by me; I did not read them.
