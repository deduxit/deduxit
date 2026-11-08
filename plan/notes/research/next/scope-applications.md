# Applications of linear logic, and where linlog's scope could grow: the scope note updated

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

This updates `plan/notes/research/scope/applications.md` (2026-10-09 to 17, "the old note"; its candidates applications-1 to 8 keep their names, new ones continue from 9). The old note's literature keys (for example [Mar15], [Sch19]) are not re-read here: they stay as the old note cites them, and a claim of this note rests only on a source of its own section 6 or says so. Where a candidate's old text is unchanged, this note says "as before".

## 1. What linlog has today

From `docs/src/status.md`, `.claude/rules/core-horn.md`, `bench/TOOLS.md` and the reports of steps 27 and 38 to 45:

- **The Horn engine** (`core/src/search/horn/`, `Engine::Horn`) decides propositional `!`-Horn programs, that is Petri nets and multiset rewriting: forward search of reachable markings plus a backward search from the goal in linear mode, backward coverability in affine mode, and the state equation's refutation (a weighting of the atoms found by a simplex and checked exactly). A proof is a firing sequence; an exhausted search is a refutation. It is the default for such programs. It takes atoms as places (`Program::place_of`); there are no labels on clauses and no multiplicity on a literal, so `a^k` is `k` copies.
- **First-order logic** (steps 42 to 45): quantifiers, unification with levels for the eigenvariable condition, witnesses retried, MLL1 and MALL1 decided, exponentials within the copy bound, checked first-order proofs, Rocq certificates, and the families `reach-fo` (reachability through a cycle in IMELL1, `core/src/families.rs`), `chain-fo`, `witness-fo`, `lambek-fo`. The first-order search is the focused engine's, not the Horn engine's: a first-order rule system is decided but not by the net-specific machinery (inference from `status.md`).
- **The batch**: many sequents in one call, with limits per sequent and for the whole batch (`status.md`, `docs/src/batch.md`).
- **Interactive proving**, the cut and cut elimination, the four exports, and certificates for `--kernel linlog`; the ordinary-logic layer (classical, intuitionistic, minimal through embeddings, with a TPTP reader).
- **Not built**: no answer streams (the engines stop at the first proof), no labels carried from input to output, no forward-stepping public API for a run, no program-as-term exports (lambda-term, process), no bounded exponential, no SBML, PNML or PDDL readers. `plan/later.md` ("Problems from practice") still lists planning domains as `!`-Horn clauses and real non-theorems on nets as candidate benchmark inputs.

Nothing below re-proposes any of this.

## 2. The field, as it stands today

### 2.1 Linear logic programming and rule-system tools

- **Celf** [1]: github.com/clf/celf, Standard ML, GPL-3.0, 264 commits, 67 stars; the last commit shown on the commits page is 2019-06-24 (a merged pull request), the commits before it 2013; no release. The README says only how to build. The CLF implementation named in the old note is thus still available but has not moved since 2019 (read 2026-11-08).
- **LolliMon** [2][3]: github.com/clf/lollimon, described as a prototype in OCaml; the README warns that `;` and `zero` in the monad are unstable and the saturation check for open terms experimental. A search result showed the same warning on a mirror. Last activity not shown (unsourced). The repository path the old note's tooling might have used (`jcreedcmu/lollimon`) returned 404.
- **Lolli** [4]: Dale Miller's page, written by Hodas in 1995 and updated by Miller recently; version 0.701 offered as a download with the interpreter's source; no licence stated; the page says the "most active implementation effort now is LLP, a language and compiler at Kobe University" (the effort's repository was not read: unsourced beyond that sentence).
- **Ceptre** [5][6]: github.com/chrisamaphone/interactive-lp, MLton, 409 commits, 293 stars; the most recent commits are 2024-08-25 (two merged pull requests) and 2024-06-16 (tutorial); prebuilt binaries for Windows, Mac and Linux under Releases; the README says it "runs on a Unix-based command line (for now)". The LICENSE file is the **Anti-Capitalist Software License 1.4**, copyright 2023 Chris Martens: use is allowed only to individuals working for themselves, non-profits, educational institutions and certain worker-owned organisations. The old note said "Ceptre is not open source"; the sourced statement is sharper: it is source-available under a licence that is not OSI-approved and excludes commercial studios (the OSI status is my inference, since the licence's own text restricts users). Ceptre is thus alive (2024 commits) and the licence is the reason for room beside it (inference).
- Martens's later use of Ceptre: a search found no 2025 publication on Ceptre [7]; a 2016 talk describes it as forward-chaining linear logic for game mechanics and interactive storytelling, and a murder-mystery generator cites Ceptre as inspiration for its linear-logic rule evaluator (search summaries, not read at the source: unsourced beyond the search listing [7]).

### 2.2 Planning, Petri nets and chemistry

- **Linear logic for planning** is old and continues: Küngas's planner (Acta Cybernetica 15(4), 2002) writes actions as sequents of a linear-logic fragment, handles nondeterminism with additive disjunction as a two-player game and searches proofs by Petri-net reachability tree analysis [8]. That is the same shape as linlog's Horn engine, without the state equation or certificates (inference).
- **Resource exchange**: Ceragioli, Degano, Galletta and Viganò (arXiv:2408.09516, August 2024) propose "a decidable extension of the computational fragment of linear logic" for agents' exchange policies and study their dynamics through provability; the abstract names no tool or proof search [9].
- **Deep-learning experiments**: DeepLL (Papoulias, arXiv:2501.00169, 30 Dec 2024) models an experiment's control flow, resources and consumption rules in linear logic, with outputs "linear logic proofs that off-the-shelf reasoners can verify"; the abstract names no reasoner [10]. This is a new kind of user: linear logic as a specification of a workflow's resources (inference).
- **Chemical networks**: reachability in restricted chemical reaction networks (Alaniz et al., arXiv:2211.12603, revised 2025-02-27): PSPACE-complete for bimolecular reactions with at most two creating and two consuming rules per species, polynomial for some feed-forward cases, NP-complete with void or autogenesis rules [11]. Pathway realisability (Andersen et al., arXiv:2309.10629, revised 2024-12-20) asks whether a pathway can be ordered so that every reaction fires, by Petri-net reachability, which "provides an ordering certificate"; linear logic is not mentioned [12]. Both are untimed questions of the Horn engine's kind (inference). The search found no tool of 2024 or 2025 for coverability on SBML models; CRNT4SBML (PNNL, 2020) reads SBML but is for bistability, and the SynKit documentation lists a CRN module with reachability and realisability, with no mention of SBML or coverability [13][14].

### 2.3 Authorisation

A search for 2024 and 2025 work on linear authorisation logics found only the line the old note cites (Garg et al. 2006, Bowers et al. 2007, the proof-carrying file system, eta-logic with explicit time, Garg's 2009 thesis) [15]. The search summary also says Nigam reports undecidability for the multiplicative fragment of linear authorisation logics and PSPACE-completeness for propositional versions; that is a talk's claim seen in a search summary only (unsourced beyond [15]).

### 2.4 Session types, ownership, quantum

- **Session types**: `par` is a Rust crate, "session types, as an implementation of linear logic with MIX", latest listed version 0.3.10 of 2025-07-23, source github.com/faiface/par (licence not seen) [16]. Ferrite embeds session types from judgemental (linear-logic based) typing in Rust, with both linear and shared channels (arXiv:2009.13619) [17]. A paper on Rust sessions notes Rust is affine, so a session may be dropped: the fit with strictly linear sessions is weaker, which suits linlog's affine mode as the right reading (arXiv:1909.05970, search summary; inference) [18].
- **Ownership**: a June 2025 Glasgow talk compares linearity, uniqueness and Rust's ownership and presents an ownership-and-borrowing type system in Granule [19]. A 2025 Toninho paper on lazy linearity (arXiv abs 2511.10361) appeared in a listing; its content was not read [20] (unsourced beyond the title).
- **Quantum**: "Imperative Quantum Programming with Ownership and Borrowing in Guppy" (arXiv:2510.13082, 15 Oct 2025) adds a linear discipline with a borrow checker to Quantinuum's Guppy, forbidding use of a qubit after measurement or discard and aliasing of owned qubits; the paper credits Selinger and Valiron and notes linear types have found little traction in imperative quantum languages except Silq [21].
- **Theory of linear logic itself, 2026**: Das and Pshenitsyn, "Wider systems for linear logic with fixed points: proof theory and complexity" (arXiv:2602.10280); Tabatabai and Jalali, "Proof Complexity of Linear Logics" (arXiv:2601.22393) with exponential lower bounds on proof size in ALL, MALL and CLL [22][23]. Neither is an application; the second bounds what any prover's proofs can be (inference), and bears on the certificates' size.

In all of these linearity is checked by a type system or a borrow checker, as the old note said: no search problem for linlog arises except synthesis. The 2025 evidence confirms the old rejection of "quantum programs" and "smart contracts" as search applications, and adds that Rust-like ownership is affine, not linear (inference).

## 3. Candidates

The old numbering is kept. Each old candidate gets its state, then what is new.

### applications-1: Plans and their certificates from the Horn engine

- **State**: untouched. The firing sequence exists as the proof (`core-horn.md`, `proof.rs`); the state equation's weighting exists and is checked exactly (step 27). What is missing is the presentation: action names, the partial order, a PDDL writer.
- **Changed**: the Horn engine now has a refutation certificate beside it (the weights), and the batch makes "many planning problems in one call" a plain use. Küngas [8] shows the Petri-net route to plans from linear-logic proofs is an established one (inference). Quantifiers do not help here: the Horn engine is propositional, and a PDDL domain with parameters must be grounded before it, which can blow up (inference from `status.md`).
- **To whom**: research (planning, certificates of unsolvability) and AI models, which would read a plan as steps with names.
- **Offer in linlog's terms**: decide, prove (the plan), check (the checker already does), export the plan as text and JSON, certify by Rocq. Searchable.
- **Cost**: 1 to 2 sessions. Trusted base: none if the names are labels outside the checked term.
- **Risk**: grounding; PDDL features outside STRIPS (costs, conditional effects) have no sequent.
- **Prerequisite**: labels from input to output (the old "room now" item 1, still not built).

### applications-2: A rule-system front end: run, explore, play

- **State**: untouched. No `run`, `explore` or forward `interact`.
- **Changed**: (a) Ceptre is alive but its licence excludes commercial use [6], so the room the old note saw remains and is better sourced; (b) the author's direction of 2026-11-08 asks for custom rules, and "a rule system with a state to run" is the object-level form of it (the custom-rules note describes the meta-level form: rules of the calculus). This candidate is **not** a custom calculus: rules here are hypotheses of a sequent, already within the Horn engine (`custom-rules.md` section 1 says the same).
- **To whom**: teaching (Petri nets, concurrency, linear logic as state change), designers of games and narratives; for AI models, a deterministic seeded run is a testable oracle.
- **Offer**: prove (reachable or not: exists), steppable (forward `interact`, new), explore (a bounded state-space listing, new). Check: the run's trace is a firing sequence the checker validates.
- **Cost**: 3 sessions (reader and format, forward API with seeded choice and no clock, command and web stage). Trusted base: none added.
- **Risk**: state spaces explode; `explore` needs limits (the batch's limits fit).
- **Prerequisite**: public `Program`, marking, `enabled` and `fire` types; labels.
- **Generic or linear-specific** (inference): the *forward-chaining mode over user rules* is generic: a state is a multiset or a set, a rule consumes and produces. Only the multiset case is linear; a set-based forward chaining is Datalog-like. See section 5.

### applications-3: Many answers: enumerate and count proofs

- **State**: untouched. Engines stop at the first proof.
- **Changed**: nothing sourced. The batch is not the same: it is many sequents, not many proofs of one. Counting reachable markings (`explore`) is the more useful half and does not need proof identity.
- **Cost**: as before, medium to high; the memo tables assume one answer. For first-order search the answer-substitution view adds a second reason (all witnesses), not a cheap path.
- **Verdict**: stay low; do it only through applications-2's `explore` and through the interactive session's alternatives.

### applications-4: Proofs as programs and processes

- **State**: untouched.
- **Changed**: sources move in two ways. `par` [16] is a working Rust session-type library built on linear logic with Mix, a natural target a CLL proof could be printed as: a linlog proof in Mix mode maps to a `par` term (inference; the mapping was not checked against its documentation). Quantifiers now exist, which the old note named as the gate to useful synthesis (needs data types beyond that). Ownership languages such as Guppy [21] type-check linearity; they do not ask for synthesis.
- **To whom**: teaching (Curry-Howard for linear logic; session types) and AI models, which can ask for a term for a type.
- **Offer**: export (lambda-term, CP process), check by the existing checker plus term typing outside the trusted base. Not certified.
- **Cost**: 2 sessions per target (term assignment and printer, fresh names). Trusted base unchanged if the term is only printed.
- **Risk**: a printed term whose typing was not independently checked is an unsound claim; it must be typed by a second small checker.
- **Prerequisite**: the export entry open for new targets (`core-exports.md` records the checklist).

### applications-5: A linear logic programming mode after quantifiers

- **State**: partly. Its prerequisite (step 38, quantifiers and unification with a witness arena) is built as steps 42 to 45: the focused engine returns witnesses and decides first-order sequents. What is not built is the Lolli-style goal-directed reading with a predictable operational order and answer substitutions as output.
- **Changed**: much cheaper than the old "High". The old note asked for all of step 38 first; now what remains is an answers view (substitutions read from the checked proof's witnesses) and a program/query surface. Completeness is the obstacle the old note named: a complete search does not promise the goal-directed order Lolli's users expect [4]. Lolli's state: available as a download, no licence stated, with the Kobe effort the more active [4]; Celf is GPL and unchanged since 2019 [1]; LolliMon is a prototype with unstable features [2]. A maintained tool with a stated licence has room (inference).
- **To whom**: research and teaching of logic programming; AI models, which could pose queries and read bindings.
- **Offer**: prove (answers by the focused engine), check (the first-order checker), export (the proof as before). Answer substitutions are searchable but only the first one; all of them is applications-3.
- **Cost**: 2 to 3 sessions (surface, answer extraction, a documented "this is not Prolog's order" stance). Trusted base: none added; substitutions are read from a checked proof.
- **Risk**: users compare with Lolli's results where the order matters (output choice); termination on programs that loop (the first-order memo's loop check helps, `reach-fo`, but it is not complete: `core/src/families.rs` says no refuter decides unprovable `reach-fo` shapes in general).
- **Prerequisite**: answer stream of applications-3 for more than one answer.

### applications-6: Reaction networks and conservation laws

- **State**: untouched (no SBML or PNML reader), though the engine exists.
- **Changed**: new sources show the questions are asked as Petri-net reachability with certificates by chemists [12], and with sharp complexity classifications for small restricted classes [11] (PSPACE-complete in the bimolecular case; so a worst case that the engine can hit). The search found no maintained coverability tool reading SBML [13][14], so the old "no tool" claim stands for that narrow case. Pathway realisability [12] gives a concrete question: does an ordering of this pathway's reactions fire? In linlog's terms that is a reachability goal with a pinned multiset (inference).
- **To whom**: research (cheminformatics, systems biology); AI models for a stoichiometry question.
- **Offer**: decide, prove (the ordering), certify (the weights are a conservation-law certificate, as before; an exact simplex checks them).
- **Cost**: 2 sessions (an SBML subset reader and a PNML reader; multiplicity on literals). Trusted base: none.
- **Risk**: readers for formats with rich semantics (rate laws, compartments, events) must refuse what they cannot map; the 2^53 rule on counts.
- **Prerequisite**: multiplicity on a literal in the Horn program.

### applications-7: Consumable credentials and proof-carrying authorisation

- **State**: untouched; `Kind` and `Rule` growth needed only for the full logic.
- **Changed**: the search found no new work since 2013 on the linear authorisation line [15]; the field is quiet, which lowers value and raises the risk that a faithful logic has no users to ask. The propositional version (principals folded into atom names) stays as cheap as before.
- **Verdict**: stay low; a documentation example in the guide, not a feature (inference).
- **Cost**: 0.5 session for an example; full modalities as before, new theory.

### applications-8: Bounded exponentials and budgets

- **State**: untouched. No `!n`.
- **Changed**: nothing sourced since the old note; with multiplicity on a literal (applications-6's prerequisite) the budget atom `c^n` is the same work. Keep as a follow-on of applications-6, not a candidate of its own (inference).

### applications-9: First-order rule systems by grounding, and parametrised nets (new)

- **What it brings**: rules with variables (`!∀x. at(x) ⊗ door(x,y) ⊸ at(y)`), as Celf and Ceptre write them, with a finite domain, compiled to the propositional Horn engine by grounding, and decided by the focused first-order engine where the domain is open (`reach-fo`'s shape). Whether grounding or first-order search decides is a choice the front end can take by size (inference).
- **To whom**: the same users as applications-2, who write rules with variables, and AI models, for whom a parametrised rule is shorter than hundreds of ground ones.
- **Offer**: decide, prove (grounded run: Horn engine; open: focused engine), check; searchable. Exploration requires the grounding.
- **Cost**: 1 to 2 sessions beside applications-2. No trusted base added if grounding is outside the checked term (a ground instance is a first-order proof's instance).
- **Risk**: grounding explodes; the open first-order search is incomplete for unprovable shapes (see `reach-fo`).
- **Prerequisite**: applications-2.

### applications-10: Resource specifications of workflows (new, weak)

- **What**: DeepLL [10] and the resource-exchange logic [9] are users of linear logic as a specification language, expecting "off-the-shelf reasoners" to verify proofs. linlog is such a reasoner: a documented translation recipe (guide page, examples) plus the existing `check`/`prove`, no new feature.
- **Cost**: 0.5 session of documentation for the guide. Value: reach to a new group of users (inference). Risk: none beyond the guide's budget.

## 4. Considered and rejected

As the old note (smart-contract verification, quantum programs, protocol analysis, scheduling with time, hybrid and subexponential logics, web-service composition), with these changes:

- **Quantum programs**: the 2025 Guppy paper [21] adds ownership and borrowing to a linear discipline, a type system; the rejection stands.
- **Ownership and session types as "tool fits"**: a tool for linearity checking in Rust or in Haskell is a type checker. A prover fits only for the synthesis of terms (applications-4), and for explaining the connection in teaching. Rust's affine nature [18] matches affine mode, which linlog has.
- **Web-service composition and security protocol analysis**: unchanged.
- **Move and Nomos**: not re-read; the old rejection stands, its Boogie claim still unsourced.

## 5. What the structure must allow

- **Reusable as it is**: the checker over proof terms and the Horn engine's firing sequences; the batch (a rule system's `explore` and many-planning-problems both fit its limits); the exports; the first-order machinery (witnesses, unification) for applications-5 and 9; interactive proving's undo and step model for a forward `interact` (inference from `core-derivations.md`).
- **Linear-specific**: the Horn engine's state equation and backward coverability (multiset semantics, so Petri nets); the focused engine's counts; proof nets. A suite of other logics would not reuse these.
- **Could stand as a suite feature of its own**: "run a rule system" as forward chaining over a user's rules, with a seeded choice, a trace, bounded exploration and a stage in the web page. For multiset rules it is linear logic; for set rules it is Datalog-like forward chaining; for rules of a calculus it is the custom-rules note's interactive builder (`plan/notes/research/next/custom-rules.md`). The forward-stepping public API is the shared piece (inference). Also generic: labels from input to output, which any front end needs.
- **Two-line summary of the API room needed** (as the old note's section 4): labels; forward stepping as public API; an answer stream; multiplicity on literals; export entry open for terms and processes. Of these, the Horn engine's `Program` as a public serialisable type is still the first.

## 6. Re-rank for research, teaching and AI models

Marked (inference) throughout; no source ranks these.

| rank | candidate | research | teaching | AI models | cost (sessions) |
|---|---|---|---|---|---|
| 1 | applications-2 with 9 (rule systems, run/explore/play) | medium | high | medium | 3 + 1 to 2 |
| 2 | applications-1 (plans, certificates) | high | medium | medium | 1 to 2 |
| 3 | applications-6 (reaction networks) | medium | low | low | 2 |
| 4 | applications-5 (Lolli-style mode) | medium | medium | medium | 2 to 3 |
| 5 | applications-4 (programs, processes) | medium | high | medium | 2 per target |
| 6 | applications-10, 7, 8 (documentation or follow-on) | low | low | medium | 0.5 each |
| 7 | applications-3 (enumerate proofs) | low | medium | low | 3+ |

Reasons: applications-2 serves teaching best (state change, Petri nets) and rests on built parts; applications-1 is the cheapest and gives researchers a certificate no planner of the old note's sources prints (the weights, [12] and the old note's [ERH17] show interest in certificates); applications-4 teaches best but costs per target. Almost all of this list is linear-specific except the forward-chaining front end of applications-2 and the label and stepping APIs; given the author's direction of 2026-11-08, applications-2 is the one to design as a generic mode first (inference).

## 7. Sources

1. Celf, github.com/clf/celf and its commits page. https://github.com/clf/celf, https://github.com/clf/celf/commits/master (2026-11-08)
2. LolliMon prototype, github.com/clf/lollimon (via search results; README warnings as reported by the search summary). https://github.com/clf/lollimon (2026-11-08, not fetched directly)
3. The `github.com/clf/` organisation page, as listed by the same search. https://github.com/clf (2026-11-08, not fetched directly)
4. Lolli, Dale Miller's page. https://www.lix.polytechnique.fr/~dale/lolli (2026-11-08)
5. Ceptre repository. https://github.com/chrisamaphone/interactive-lp and https://github.com/chrisamaphone/interactive-lp/commits/master (2026-11-08)
6. Ceptre LICENSE. https://raw.githubusercontent.com/chrisamaphone/interactive-lp/master/LICENSE (2026-11-08)
7. Search results on Ceptre and Martens, listing Martens, AIIDE 2015 https://ojs.aaai.org/index.php/AIIDE/article/view/12784, her 2016 talk https://speakerdeck.com/chrisamaphone/programming-interactive-worlds-with-linear-logic (2026-11-08; the talk and the murder-mystery generator seen in search listings only)
8. P. Küngas, "Resource-conscious AI planning with conjunctions and disjunctions", Acta Cybernetica 15(4), 2002. https://cyber.bibl.u-szeged.hu/index.php/actcybern/article/view/3599 (2026-11-08)
9. Ceragioli, Degano, Galletta, Viganò, "A Logic for Policy Based Resource Exchanges in Multiagent Systems", arXiv:2408.09516. https://arxiv.org/abs/2408.09516 (2026-11-08)
10. N. Papoulias, "DeepLL", arXiv:2501.00169. https://arxiv.org/abs/2501.00169 (2026-11-08)
11. Alaniz et al., "Reachability in Restricted Chemical Reaction Networks", arXiv:2211.12603. https://arxiv.org/abs/2211.12603 (2026-11-08)
12. Andersen et al., "Pathway Realisability in Chemical Networks", arXiv:2309.10629. https://arxiv.org/abs/2309.10629 (2026-11-08)
13. CRNT4SBML at PNNL, as listed in search. https://www.pnnl.gov/publications/crnt4sbml-python-package-detection-bistability-biochemical-reaction-networks (2026-11-08, search listing only)
14. SynKit CRN documentation. https://synkit.readthedocs.io/en/stable/crn.html (2026-11-08, search listing only)
15. Linear authorisation logic papers (Garg et al. 2006, Bowers et al. 2007, Garg's thesis) as listed by search. https://www.mpi-sws.org/~dg/papers/aff-know.pdf, https://www.mpi-sws.org/~dg/papers/NDSS07Final.pdf, https://people.mpi-sws.org/~dg/papers/thesis.pdf (2026-11-08, search summaries only)
16. `par` crate. https://docs.rs/crate/par/0.3.2 (2026-11-08)
17. Ferrite, arXiv:2009.13619. https://arxiv.org/pdf/2009.13619v5 (2026-11-08, search summary only)
18. "Rusty Variation: Deadlock-free Sessions with Failure in Rust", arXiv:1909.05970. https://arxiv.org/pdf/1909.05970 (2026-11-08, search summary only)
19. D. Marshall, "Linearity, Uniqueness, Ownership: An Entente Cordiale", seminar listing, 2025-06-26. https://chocola.ens-lyon.fr/events/meeting-2025-06-26/talks/-1/ (2026-11-08, search listing only)
20. Toninho's paper listing, DBLP. https://dblp.dagstuhl.de/pid/46/9220.html (2026-11-08, listing only)
21. Koch et al., "Imperative Quantum Programming with Ownership and Borrowing in Guppy", arXiv:2510.13082. https://arxiv.org/abs/2510.13082v1 (2026-11-08, search summary only)
22. Das, Pshenitsyn, arXiv:2602.10280. https://arxiv.org/abs/2602.10280 (2026-11-08, search listing only)
23. Tabatabai, Jalali, "Proof Complexity of Linear Logics", arXiv:2601.22393. https://arxiv.org/abs/2601.22393 (2026-11-08, search listing only)

## 8. Not verified

- The old note's literature (keys such as [Mar15], [Sch19], [HKP20], [Gar06]) was not re-read; no check for follow-up work to proof-carrying plans (2020) was made, so "no new work since" is claimed only for the authorisation line, and only from one search.
- Entries marked "search summary only" or "listing only" in section 6 (17 to 23, 7, 13, 14, 15) were not opened at the source; their claims stay as hedged where used.
- LolliMon's last activity date, the Kobe LLP repository, the Ceptre releases' dates and `par`'s licence were not found.
- The Move Prover's use of Boogie stays unsourced, as in the old note.
- Whether a linlog proof in Mix mode maps to a `par` term is an inference; `par`'s documentation was not read beyond its one-line description.
- Nothing was run.
