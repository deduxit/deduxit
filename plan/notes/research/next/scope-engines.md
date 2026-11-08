# Engines after 0.2.0: the scope note "engines" brought up to date

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

This note continues `plan/notes/research/scope/engines.md` (written 2026-10-09 to 17, "the old note" below). Its candidates keep their names, engines-1 to engines-11; new ones continue at engines-12. "(inference)" marks a conclusion of this note's own. Numbers in square brackets are section 7 (the web sources). "[R]" is a file of this repository, named in the text.

## 1. What linlog has today

From `docs/src/status.md` and the reports it rests on; nothing below re-proposes these.

- **Engines and dispatch.** The focused engine (classical, one-sided), the two-sided engine for the intuitionistic modes, the net engine (MLL, cyclic, Lambek, IMLL essential nets), the additive recursion, the Horn engine for Petri-net programs (forward and backward search, the state equation by a floating-point simplex with an exact check), and since step 37 the focused inverse method (`Engine::Inverse`, `core/src/search/inverse/`), in every commutative mode, with a checked refutation `Refutation::Saturated` (`plan/reports/37-inverse.md`, "The certificate"). The dispatch is a table of eight rows (`DISPATCH` in `core/src/search/mod.rs`): Lambek to the essential-net engine, cyclic to the net engine, two additive formulas to the additive recursion, the Petri-net feature to the Horn engine, intuitionistic with exponentials and intuitionistic otherwise to the two-sided engine, Mix and classical to the focused engine. Two of those rows (intuitionistic with exponentials, and Mix) give the inverse method turns beside the engine: it searches alone for 2^24 units, then the inverse method takes a turn each time the engine's work has doubled, at most a sixteenth of that work in all (`PROBE_AFTER`, `TURN_SHARE`).
- **First order.** The focused and two-sided engines decide first-order sequents by unification with eigenvariables kept by levels, MLL1 and MALL1 decided, exponentials within the copy bound (`docs/src/status.md`, step 42). The inverse method has no quantifiers (`.claude/rules/core-inverse.md`, "Where the next steps plug in").
- **Refutations.** Counts (atom balance, count equation), the classical reading with a falsifying assignment looked for after any search that did not prove (step 31d, on by default), the state-equation weighting, and the saturated set; Rocq certificates for the first three kinds, not for the saturated set (`Unsupported::Saturated`).
- **Parallel and portfolio.** A pool behind the `parallel` feature (cube and conquer at the root, shared memo, `&` premises in parallel); the one-thread-first default; the portfolio was removed (step 19).
- **Measurement.** Four baselines (`docs/src/results/baseline-2026-11-02.md` is the fourth), the other provers on the libraries with the engines of 0.1.0 (`docs/src/results/provers-2026-10-15.md`), the families that grow (`growing-2026-10-31.md`, `growing-2026-11-02.md`) and the other provers on them (`provers-2026-11-03.md`), and the first-order comparison with LinearOne (`first-order-2026-10-30.md`). `status.md` lists "the comparison with the other provers on the libraries with the engines of 0.2.0" as planned, so no chapter shows Sympli against 0.2.0 on the ILTP-derived collections.
- **No SAT or SMT solver** anywhere in the workspace: a search of the repository's Markdown, TOML and Rust (outside the research notes and reports) for DIMACS, SMT-LIB, varisat, cadical and "SAT solver" finds nothing. No export of an encoding for an external solver. No search state kept between calls (`plan/later.md`, "Follow-ups: parallel search" still lists "a `Runtime` kept across calls"). The web bindings and `interact` have a goal closed by the search within a budget of work and a hint (`status.md`), not background marking of every open goal.

## 2. The field, as it bears on the engines

### 2.1 Where the other provers stand (read 2026-11-08)

- **Sympli** (Chaudhuri's propositional linear inverse method, BSD-2-Clause, SML on MLton): the repository was archived by its owner on 2021-09-09, five commits, "written in 2003-2004 and only lightly maintained since" [1]. `bench/TOOLS.md` gives its last activity as 2018-07-27. It is still the tool linlog loses to on the intuitionistic collections, and the only one that passes 8 on `duplicate` (`provers-2026-11-03.md`).
- **LinearOne** (Moot, first-order multiplicative intuitionistic linear logic, LGPL-2.1): a prototype whose README carries copyright "2015-2026", 308 commits, 5 open issues [2]. The page shows no commit dates; `bench/TOOLS.md` records 2026-06-11. It decides 8 problems of the growing sets that linlog does not (`provers-2026-11-03.md`, "Solved within 60 s": 58 of 61 against linlog's 179 of 202 on a different, larger set).
- **TPTP and linear logic.** The TPTP non-classical page lists only the generic, alethic, deontic, epistemic and doxastic modal logics and the instant-based temporal one, and names no linear, substructural or intuitionistic logic [3]. The paper on the infrastructure says, in its further work, that the "obvious candidates include linear logics and intuitionistic logics" [4]. So the old note's engines-11 premise stands: the door is declared open, nothing is registered. CASC's divisions are classical; the 2025 competition (CASC-30) page I found lists THF, TFA, FOF, FNT and similar columns only [5].
- **ILTP** has two collections, propositional and first-order intuitionistic problems, about 2 800 together [6]. linlog uses the propositional 274 through the ILLTP translations; the first-order collection is not read by linlog (no mention in `status.md`; unverified beyond that).

### 2.2 SAT solvers for Rust (read 2026-11-08)

- **varisat**: MIT or Apache-2.0, README points at version 0.2.1, 244 commits, 24 open issues; the page shows no release date and no last-commit date [7]. The old note's "no release since 2020" is not confirmed by this page; I could not read crates.io or lib.rs (HTTP 403). Treat the maintenance state as unverified.
- **splr**: Mozilla Public License 2.0, version 0.19.0 released "22 August" (year not shown), benchmarked in the 2025 SAT Competition, a `platform_wasm` feature added 2024-02-03, incremental mode dropped in 0.18.0 [8][9]. The licence is not among those `deny.toml` allows (CLAUDE.md lists EUPL-1.2, MIT, Apache-2.0, Unicode-3.0, Zlib, BSD-2/3), and incremental solving with assumptions is what engines-1 needs (inference).
- **RustSAT** (Jabs, MIT, SAT 2025): one trait API over Kissat (non-incremental), CaDiCaL, MiniSat, Glucose and Batsat (incremental), any IPASIR library; Batsat is a Rust reimplementation of MiniSat, "so projects using it can be pure Rust and can, for example, compile to WebAssembly" [10][11]. This is the best-sourced route to a pure-Rust incremental solver with assumptions; licence of Batsat itself unsourced.
- **Complexity background** is unchanged from the old note (Lincoln and Winkler, Kanovich, Lincoln et al., Das); not re-read today, so cited there only.

### 2.3 Learning for proof search, 2025/26

My searches found nothing on machine-learned search for linear logic. The nearest sources are: Neural Proof Nets for MILL axiom links (old note, [20] there); "Learn from Failure" fine-tunes language models on trial-and-error data for intuitionistic propositional logic, with a benchmark in Lean, 2024 [12]; deep reinforcement learning for intuitionistic propositional proof search, 2018 [13]. I could not run the search for the 2025 Lean provers (DeepSeek-Prover-V2, Goedel-Prover, Seed-Prover) because the shared search budget ran out, so nothing is claimed about them here. A search for new work on the focused inverse method for linear logic returned only the 2005 to 2006 Chaudhuri and Pfenning papers and a student report [14].

## 3. Candidates

### 3.1 The old candidates, one by one

For each: status now, what the measurements since say, whether the cost estimate holds.

### engines-1: A SAT-based intuitionistic engine for ordinary logic

- **Status: untouched.** Neither the ordinary layer nor `status.md` mentions a SAT route; "countermodels of the other logics" stays under "Planned".
- **What the measurements say.** The inverse method now decides 113 of the 384 intuitionistic rows no tool decided at 10 s, the default 26 of them at 60 s, and its turns give the default 102 more rows (`plan/reports/37-inverse.md`, 37a and 37b outcomes). The need that engines-1 answered (166 ILTP problems undecided at the 0.1.0 layer) has therefore shrunk by an amount the comparison of 0.2.0 on the libraries would give; that comparison is not yet run, so the residue is unmeasured. The classical countermodel (31d) now refutes the rows that are false classically, which were the cheap half of Kripke refutations.
- **Cost.** Mostly as before: one session for the algorithm, one for the countermodel's check (inference). The dependency choice changed: RustSAT with Batsat is a pure-Rust incremental option [10][11], splr is MPL and without incrementality, varisat unverified [7][8]. A Rocq theorem for a Kripke countermodel would be the real cost (the library has none for the saturated set either).
- **Carries over.** Yes, strongly: a SAT-modulo-implications core serves intuitionistic and minimal ordinary logic and any suite built on them (inference).

### engines-2: Lazy SAT over axiom linkings for MLL

- **Status: untouched.**
- **What the measurements say.** On the families the net engine is now the claim of one family only (`growing-2026-11-02.md`), the default hands `wide-m1` to the net engine past the recursion limit, and the focused engine decides 4 096 pairs of `wide-m1` in 0.87 s (`provers-2026-11-03.md`). Partition encodings are still 10 to 100 times faster on the focused engine's counts than on nets (`plan/later.md`, "Follow-ups: the Horn engine", last items). Nothing in the baselines shows an MLL input the three engines leave and a linking search might take; the 4th baseline's open rows are `partition-yes` at 28 items and `partition-no` at 14 and 15 (`baseline-2026-11-02.md`, "The families"), where counts, not linkings, are the issue.
- **Cost.** Unchanged estimate; the case is weaker than in October (inference). Keep as pure research.
- **Carries over.** The encoding is multiplicative-specific; the conflict-clause loop is generic.

### engines-3: Integer and trap refutations for the Horn engine through a solver

- **Status: untouched.** The follow-ups in `plan/later.md` "Follow-ups: the Horn engine" are as in October: the integer state equation, traps, the 2^20 weight span, the dense simplex basis, a forward coverability search beside the backward one.
- **What the measurements say.** The Horn engine decides `counter` to 262 144 tokens in 0.25 s and `duplicate` #1 to 262 144 in 63 ms (`provers-2026-11-03.md`, `growing-2026-11-02.md`); the open rows are the 66 library nets of step 27 and the qcover residue. No comparison since shows another tool deciding a net linlog does not, except the 31 nets on which APLL answers against the header and linlog is silent (`comparison-gaps.md`, 1.1); APLL's refutations there are wrong by `bench/TOOLS.md`.
- **Cost.** Unchanged; value unchanged but now measured to be small for the headline numbers (inference). An exact simplex is the cheaper half than a solver dependency.
- **Carries over.** The state equation and traps serve any Petri-net or vector-addition suite (reachability, coverability), not other logics.

### engines-4: Encodings as exports (DIMACS, SMT-LIB, ASP, QBF)

- **Status: untouched** (no printer exists). Still low cost, no dependency. Its value is now clearer for the one reason in engines-1 and engines-2: it lets a researcher test a solver route on linlog's families before any engine is built. It also feeds suites for other logics a "encode into SAT" exercise (inference).

### engines-5: Learned dispatch and engine schedules

- **Status: partly built, by hand.** The time-sliced schedule the old note names exists: the bias search's backward and forward halves (`Bias::Auto`) and, since 37b, the inverse method's turns on two rows. The table has eight rows, not four.
- **What the measurements say.** 37b found that no static feature separates the inverse method's wins (intuitionistic sequents with `!`-hypotheses, the SYJ203 to SYJ210 families) from its losses (pigeonhole formulas, SYJ201, 206, 207, 212, and every generated family): "no static feature" (`plan/reports/37-inverse.md`, "The dispatch"). That is the case for a learned choice. Against it, the families show inverse wins with no row: `wide-m1` (32 768 pairs in 47 s forced, against 16 384 in 35 s by the default), classical `chain` (2 048 clauses in 4 ms, then 8 192 in 19 ms, where the default meets the recursion limit at 1 024), affine MALL `pigeonhole` (6 holes in 0.27 s against a minute) (`plan/later.md`, "Found by the growing families"; `growing-2026-11-02.md`). A learned rule would find those; a hand row for each is cheaper first (engines-12).
- **Cost.** Unchanged, but now the training data are larger (four baselines, the growing sets), and there are more engines to choose among (inference). Rules emitted as `DISPATCH` rows with their measurement keep the dispatch deterministic.
- **Carries over.** Entirely generic: features, a table of rows, a measured choice.

### engines-6: Learned choice ordering inside the focused engine

- **Status: untouched.** Step 39 measured the one pattern it looked for (two or more copies of a literal only a `⊤` can take) on 129 undecided held-out rows and found it in none (`plan/reports/33-top-cap.md`), so the undecided rows are not explained by that pattern; it says nothing about orderings. The old argument stands: an exhaustive failure explores every choice (inference). Research only; the lowest value of the old list for the three audiences.

### engines-7: A proof-search environment and dataset for machine learning

- **Status: partly built.** `interact` and the web bindings expose goals, rules and the session as JSON, with a hint (`status.md`); no protocol framed as an environment, no datasets. An MCP server for linlog is in `plan/later.md` ("For the author: an MCP server for linlog (asked by step 44, not built)"). The guide `skills/linlog/` serves models today.
- **What the field says.** Nothing in my 2025/26 reading contradicts the old note: ML on linear logic remains grammar-bound (Neural Proof Nets); the intuitionistic propositional work I found uses Lean (2024) or is from 2017 to 2018 [12][13]. A dataset of checked proofs and certified refutations is still without a counterpart (inference). A fresh search for 2025/26 provers was not possible (section 2.3).
- **Cost.** Low, as before; the MCP server is the same work seen by a different consumer.
- **Carries over.** The environment protocol is generic over calculi once the interactive state is.

### engines-8: Live provability feedback while proving by hand

- **Status: partly built.** A goal closed by the search within a budget, and hints, exist in the bindings (`status.md`); automatic marking of every open goal as proved, unprovable with its reason, or unknown is not listed. Still the highest value for teaching (inference), and the refutation kinds added since (saturated set, classical reading) make the "unprovable, because" side richer. Cost and prerequisites as in October, plus the point that a refuting engine now gives reasons that can be shown (a count, an assignment, a weighting).

### engines-9: Search state kept across calls

- **Status: untouched** (the follow-up is open in `plan/later.md`). The inverse method makes the case differently: a saturated set is itself a reusable state, valid for every goal in the same subformula closure (inference, unchecked; the saturation depends on the goal's forest, so sharing needs a shared forest). Cost and risk as before.

### engines-10: Competition inputs for the Horn engine (STRIPS planning, PNML nets)

- **Status: partly built in effect.** qcover's 176 `.spec` coverability problems and the Mist reader are now in the benchmarks (`status.md`: "coverability problems in the `.spec` format of the Mist tool"); STRIPS and PNML readers are not. The comparison shows no other prover of linear logic competes on nets except APLL (wrongly on 1 296 nets) and Click & coLLecT on tiny ones (`bench/TOOLS.md`, "Wrong answers"), so these inputs would compare linlog with Petri-net tools, not with provers of the logic. Value for research; low for teaching and for models (inference).

### engines-11: Linear logic in the TPTP World

- **Status: untouched, premise confirmed.** TPTP's registry has only modal and temporal logics [3]; the authors name linear and intuitionistic logic as obvious candidates for further standardisation [4]. No source shows anyone proposing it. Cost: mostly coordination, as before. The generic part, a second concrete syntax for the parser, helps the suites for other logics as well (inference).

### 3.2 New candidates

### engines-12: Dispatch rows for the inverse method

*What it brings and to whom.* The inverse method forced beats the default on `wide-m1`, classical `chain` and `pigeonhole` in affine intuitionistic MALL, where no row gives it a turn (`plan/later.md`, 2026-10-31 entry; `plan/README.md`, the 2026-11-02 review of 45b). Research and teaching see it only as speed; for models the default is what they get, so a missing row is a silent loss (inference).
*In linlog's terms.* Decide (and proofs and refutations by the same engine, with its certificate). A row needs the measurement 37b says it needs: it must win every row the default decides in its class, and "no static feature gave" that for the intuitionistic rows (`plan/reports/37-inverse.md`, "The dispatch"). The classical `chain` and the multiplicative `wide-m1` classes are different: the 4th baseline's generated families lose on the inverse method (45 of 123 against 111 decided, `37-inverse.md` 37a outcome), so a row by fragment alone would lose the Partition, QBF and wide sequents. The candidate row is by a feature (a chain of Horn-like clauses with no `!`, or the sequent's size against the recursion limit), found by measurement. A turn on the classical row, as on the Mix row, is the cheap form: its cost is bounded by `TURN_SHARE` (inference from the code's comments).
*Cost.* One session, mostly the measurement (the target sets and a held-out set). No trusted-base growth; the saturated set's check is already built.
*Risk.* The turns' time is "within a factor of 30" of the units (37b's open items), so a turn can take several times a sixteenth of the engine's time on some rows.
*Prerequisites.* None. *Sources.* The two repository notes above; `growing-2026-11-02.md`.

### engines-13: A search whose depth does not grow with the formula, for the first-order shapes LinearOne decides

*What it brings and to whom.* LinearOne solves 8 problems linlog does not: `lambek-fo` #0 and #2 at 512 and 1 024, #3 at 1 024, #4 at 1 024 and 2 048, where linlog's two-sided search meets its recursion limit of 2 048 (within 0.3 s on #0, #3 and #4, after 5.9 s and 44 s on #2), and `witness-fo` #3 at 12, where linlog reaches the 60 s limit (10 in 3.4 s, about 2.1^n) and LinearOne refutes it in 82 ms, flat from size 2 (`provers-2026-11-03.md`; `plan/later.md`, 2026-11-03 entry; `first-order-2026-10-30.md` for the earlier 3). Research (the Lambek calculus is linlog's one parsing use, via Moot and Piazza's translation), and for models a sentence-length limit that is a refusal rather than a verdict.
*What it would offer.* Decide; proofs and checks are built. Two separate defects: (a) the depth: recursion per formula or per link of a chain, which `plan/later.md` ("Follow-ups: the focused engine") already names as "a focused search that hands over to the net engine at the recursion limit", and `chain-fo` has the same shape; (b) the `witness-fo` refutation, which wants either the first-order counts or a better witness order.
*Cost.* Two sessions (inference): one to remove the recursion limit's reach at first order (a work stack in place of recursion, or a handover to the essential-net engine for MILL1, which has no first-order version today), one to measure the witness search. The first-order case of the net engines is a data-model change (inference).
*Risk.* The LinearOne fragment is MILL1 only; the effort serves one family unless the depth fix is general. The `chain-fo` finding (deepening past 140 000 without end) says the copy-bound deepening, not only the depth, is a problem.
*Prerequisites.* None for (a); for the net route, first-order essential nets, which no engine has.
*Sources.* As above, and LinearOne [2].

### engines-14: The copy bound's deepening, grown faster where the search used all its copies

*What it brings.* `chain-fo` is proved to 64 and never refuted at 16 under the command's own deepening, though under the family's stated bound the engines prove to 384 (`plan/later.md`, 2026-11-03 entry). The decisiveness defect of `comparison-gaps.md` (class A: 94 refutations whose every level was cut, copy levels in the millions) was fixed in part by R1 (cut entries read only within a generation, step 30b), R2 (classical countermodel) and the inverse method's turns, which refute 58 more rows; the 129 undecided held-out rows of step 39 are what is left of it (`plan/reports/33-top-cap.md`), and it is the same shape: a level counter that grows by one and never closes.
*What it would offer.* Decide, and an "unknown" that says why. A deepening that grows the bound by more than one copy a level only where the search used them all (the follow-up's own suggestion), or a saturation-based test that no level can close (the inverse method's `Exhausted` is such a test).
*Cost.* One session. *Risk.* The change moves every target set counter (the engine's counters are a function of the input); a behaviour-lock change, to be a commit of its own (CLAUDE.md).
*Prerequisites.* None. *Sources.* The entries above.

### engines-15: The inverse method at first order

*What it brings.* The one engine that beat Sympli's class C wins and refutes where saturation ends has no quantifiers (`core-inverse.md`, "Where the next steps plug in" names the place: the zone's members carry terms, unification at the partners and holes, subsumption as matching). First-order LL has no other forward prover in the comparison; LinearOne is backward and multiplicative. Research (a forward first-order linear prover is, as far as the comparison goes, absent: inference from `bench/TOOLS.md`).
*What it would offer.* Decide, with proofs checkable by the existing first-order checker; a refutation certificate would need a theorem about a saturated set with substitutions, which the Rocq library lacks at propositional level already.
*Cost.* Three or more sessions (inference): matching for subsumption is the hard part, and first-order saturation does not terminate in general, so "unknown" is the usual answer on unprovable goals.
*Risk.* High. Subsumption with terms can lose the efficiency that makes the method win; the Chaudhuri and Pfenning papers I found concern first-order intuitionistic linear logic and claim a practical prover [14], not a measured comparison linlog could repeat (the search snippet only). The refutation side is undecidable (first-order LL is undecidable; not re-sourced today).
*Prerequisites.* A decision on the propositional dispatch rows first (engines-12).
*Sources.* [14] and `core-inverse.md`.

### engines-16: A first-order comparison library

*What it brings.* The comparison has first-order families of linlog's own (`lambek-fo`, `witness-fo`, `reach-fo`, `chain-fo`) and one other tool, LinearOne, on one fragment; no first-order linear problem library exists, as far as my reading goes: LLTP and ILLTP are propositional; TPTP has no linear logic [3]. ILTP's first-order intuitionistic collection exists [6] and could be given through a translation into ILL as the propositional ones are, so linlog would run intuitionistic first-order problems through the Girard-style translations. QMLTP (modal) is not a candidate (a different logic). For research this is the only external first-order measure available (inference); for teaching and models, nothing.
*What a suite would offer.* A reader (the TPTP reader of the ordinary layer has the syntax), the translations (built for propositional), a harness collection. Check by the first-order checker; prove and refute by the existing engines, which would show where first-order search stands.
*Cost.* Two sessions (inference): the first-order translation and its soundness, then the measurement. *Risk.* A translation into ILL1 gives problems unlike what the engines were tuned for; the result could be bad and say little (inference). The licence of the first-order collection is not stated on the page I read; ILTP's own licence is unsourced (`bench/` notes list the propositional collection as having none stated).
*Prerequisites.* The first-order engines of step 42 (built). *Sources.* [6], [3].

### engines-17: A generic focused engine over rule schemas

*What it brings.* The author's wider question (`plan/46-research-next.md`, "The author's direction (2026-11-08)") is a collection of suites for many logics with custom rules. The focused engine and the inverse method are written for linear logic's connectives and zones, compiled from the forest into rules (`rules::compile` in `search/inverse/`); the rule-compiler in the inverse method already turns a calculus into "terms with holes", which is a schema. Whether the forward saturation can be parameterised by a calculus (rules as data) instead of the forest is the question that decides if one engine serves ordinary classical and intuitionistic sequent calculi, modal and cyclic suites, or only linear ones (inference).
*What it would offer.* Decide for any calculus given as schemas that are focusable; check, step and export through the same schemas. The checker, the derivation and the interactive layers would also have to be schema-driven; this note judges only the engine part.
*Cost.* Four or more sessions (inference), the largest of this note; plus an unknown number for the exports. It adds the schema reader and its soundness to the trusted base; a bug there would no longer be caught by an independent checker unless the checker is also parameterised.
*Risk.* The focusing completeness argument is per calculus (the engine's "focusing complete under any polarity" argument is for linear logic: `37-inverse.md`, "The certificate"), so a schema engine for a new calculus needs a proof per calculus or no completeness claim; and performance, which is linlog's stated goal, usually costs against generic code.
*Prerequisites.* The module layout decision of step 46 (a kernel split) and `plan/notes/research/next/custom-rules.md`, which owns the question of user-supplied rules (not repeated here).
*Sources.* Repository only.

## 4. Considered and rejected

Kept from the old note: a QBF engine for MALL, ASP as an engine, classical ordinary logic through SAT (still not offered as a classical prover: `plan/later.md`), more parallelism, an LLM inside the library, entering CASC. Changes and additions:

- **More parallelism** is rejected again with the new measurements: the pool gains nothing on the families beyond the second thread (QBF 1.2x) and is 2.9 times slower in the median on the LLTP problems both decide (`plan/later.md`, "Follow-ups: parallel search"). The default's four cores in the comparison decide exactly what one thread does (`provers-2026-11-03.md`: 179 against 179).
- **Chasing Sympli's remaining refutations by a new engine.** The class-C and the six refutations are decided; class A (94) is a decisiveness defect for which R1, R2 and the turns were the fix; the open count is unmeasured until the 0.2.0 library comparison (planned). Spend measurement first.
- **A solver-based search for the Horn nets** beyond engines-3's exact state equation: the headline results are dominated by the dispatch's own Horn engine, and the other prover's claims there are wrong answers (`bench/TOOLS.md`).
- **A third-party SAT dependency before a measurement** from engines-4's exports (inference): engines-1 and engines-2 each stand or fall on an encoding that no source shows works for linear logic.

## 5. Re-ranking (inference)

Order by what serves research, teaching and AI models, in this note's judgment.

**Research.** (1) engines-12 and engines-14, cheap and measured, and the open "unknown" rows are what a comparison paper would be asked about; (2) engines-13, because a named rival decides what linlog does not; (3) engines-16, the only external first-order measure; (4) engines-1 (SAT) with engines-4 (the exports) as the way to test it; (5) engines-15, high cost; (6) engines-3, engines-2, engines-5, engines-6.
**Teaching.** (1) engines-8, live feedback, which no other tool among those compared offers; (2) engines-7 as the MCP server of the follow-up for tutoring models; (3) engines-14, because "unknown, copy bound 140 000" teaches nothing; the rest of the list is invisible to a student.
**AI models.** (1) engines-12 and engines-14 (the default must decide, since a model calls the command and will not tune it), (2) engines-5, as schedules the model need not choose, (3) engines-7, the protocol.

**Linear-specific against carrying over.**

| candidate | linear-specific | carries to suites for other logics |
|---|---|---|
| engines-1 | no | yes: a SAT-modulo-implications core for intuitionistic and minimal logic; a CDCL core for classical propositional logic is its simpler cousin (inference) |
| engines-2, 3 | yes (linkings; the state equation of a Petri net) | no |
| engines-4 | encodings of linear problems | the printer skeleton |
| engines-5, 6 | the tables | features, tables, measurement: yes |
| engines-7, 8, 9 | partly | session, goals, background budgets: yes |
| engines-12, 14 | tied to the dispatch | the method of measuring a row: yes |
| engines-13, 15 | yes | the first-order unification and levels: yes in part |
| engines-16 | yes | any logic with a library |
| engines-17 | the question itself | yes, by design |

A CDCL or SAT core for classical propositional logic is not wanted as an engine: classical logic is decided through the embedding already and the project does not offer itself as a SAT solver (`plan/later.md`); a pure SAT back end earns its place only as engines-1's dependency.

## 6. What the structure must allow

From `.claude/rules/core.md` and `core-inverse.md` (quoted by reading, not re-derived here): the dispatch is a table of rows by fragment, mode and one feature, with `Engine` as data; engines sit behind `search::Decide`; refutations are named variants of `Refutation` that grow by field; the checker is independent of every engine. Reusable for other logics: the table and its measurement method, the stop and memory accounting, the memo, the checker's structure, the refutation variants, the wire levels. Linear-specific: the one-sided forest of occurrences, zones and counts, the focusing calculus, the net criteria. The first generic kernel the author's direction asks for (a rule-schema engine, engines-17) would have to split these two kinds of code first; this note does not design that.

## 7. Sources, read 2026-11-08

1. Sympli, github.com/chaudhuri/sympli: https://github.com/chaudhuri/sympli (archived 2021-09-09, BSD-2-Clause, 5 commits).
2. LinearOne, github.com/RichardMoot/LinearOne: https://github.com/RichardMoot/LinearOne (LGPL-2.1, copyright 2015-2026, 308 commits, no commit dates shown).
3. TPTP, non-classical logics: https://tptp.org/UserDocs/TPTPLanguage/NonClassicalLogics.html
4. A. Steen, G. Sutcliffe, "TPTP World Infrastructure for Non-classical Logics", arXiv 2508.09318, abstract https://arxiv.org/abs/2508.09318, further work read in the HTML version https://arxiv.org/html/2508.09318 (the last 13 000 characters not read).
5. CASC-30 design, found by search, not opened: https://tptp.org/CASC/30/DesignFull.html (only its snippet read; the divisions claim is thus weak).
6. ILTP library: https://www.iltp.de/
7. varisat: https://github.com/jix/varisat
8. splr: https://github.com/shnarazk/splr
9. splr releases: https://github.com/shnarazk/splr/releases
10. RustSAT repository: https://github.com/chrjabs/rustsat
11. C. Jabs, "RustSAT: A Library For SAT Solving in Rust", SAT 2025: https://drops.dagstuhl.de/storage/00lipics/lipics-vol341-sat2025/html/LIPIcs.SAT.2025.15/LIPIcs.SAT.2025.15.html
12. "Learn from Failure: Fine-Tuning LLMs with Trial-and-Error Data for Intuitionistic Propositional Logic Proving", arXiv 2404.07382: https://arxiv.org/pdf/2404.07382 (from a search snippet)
13. Kusumoto, Yahata, Sakai, deep reinforcement learning for intuitionistic propositional proof search, arXiv 1811.00796: https://arxiv.org/abs/1811.00796 (from a search snippet)
14. Chaudhuri and Pfenning on the focusing inverse method: https://link.springer.com/doi/10.1007/11538363_15 and https://researchportal.ip-paris.fr/en/publications/focusing-the-inverse-method-for-linear-logic/ (search results only, not opened)

Repository files read: `docs/src/status.md`; `docs/src/results/{baseline-2026-11-02,provers-2026-10-15,provers-2026-11-03,growing-2026-11-02,first-order-2026-10-30}.md`; `bench/TOOLS.md`; `plan/notes/research/{scope/engines,comparison-gaps}.md`; `plan/later.md` (the engines' follow-up sections); `plan/reports/{33-top-cap,37-inverse,42-first-order}.md`; `plan/README.md` (the 2026-11-02 entry); `core/src/search/mod.rs` (`DISPATCH`); `.claude/rules/core-inverse.md`.

## 8. Not verified

- The varisat release date and maintenance state, and the splr release year (pages show none; crates.io and lib.rs gave HTTP 403). Batsat's licence.
- Any 2025/26 work on machine-learned proof search (the Lean provers in particular): the search budget ran out; items 12 and 13 come from snippets, not read in full.
- Items 5 and 14 were seen only as search results. The claim that CASC-30 has no non-classical division rests on a snippet.
- How many of the 94 class-A rows of `comparison-gaps.md` the current engines decide: no comparison of 0.2.0 on the libraries exists (`status.md`), and I ran nothing. Likewise whether the inverse method decides `lambek-fo` or `witness-fo` rows LinearOne decides (it has no quantifiers, so it cannot).
- That ILTP's first-order collection is not read by linlog: inferred from `status.md` only. Its licence, and the problem counts of each collection.
- The old note's references (Lincoln and Winkler, Das, Kanovich, Neural Proof Nets, LeanDojo and others) were not re-opened.
- What I would have run: the 0.2.0 library comparison (`bench/comparison.sh`) with Sympli, to size engines-12 and engines-14 against the class-A rows; a probe on `lambek-fo` for how deep the recursion goes. Both need a capped scope on given cores, which I was not given.
