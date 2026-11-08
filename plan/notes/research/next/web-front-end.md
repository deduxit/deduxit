# The unified web front end: one stage, each tab choosing its logic and method

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

The author's words (2026-11-08, `plan/46-research-next.md`): "a unified view with the center stage and for each stage (per tab) one could choose the calculus and proof method that one wants to work on". This note says how far the client is from that, what the precedents do, what the architecture options cost, and what a unified front end asks of the libraries. Repository facts are cited by file; everything else carries a numbered source (section 6).

## 1. What linlog has today

All of this is read from `/home/tux/Projects/own/linlog-prover.github.io` (the client, at the working copy of 2026-11-08) and from `plan/web/design.md` and `plan/web/README.md`.

- **Tabs.** A row of open proofs in the top band, each a session document in browser storage with its own link, exports and step tree (D50, `plan/web/design.md`). A goal can be opened in a new tab as a proof of its own, in the same calculus, and filled back from another tab's finished proof (client `README.md`, `state.tsx`). Every tab keeps its own calculus (client `README.md`).
- **The per-tab choice of calculus.** The tab's calculus mark opens a popover (`src/islands/TopBand.tsx`, `Calculus.tsx`): a segmented control Linear or Ordinary (D57), then, for linear logic, classical or intuitionistic, Mix, affine weakening and, in classical mode, a two-sided presentation (D58). The type behind it is `Mode`, six strings: `classical`, `affine`, `mix`, `affine-mix`, `intuitionistic`, `intuitionistic-affine` (`src/lib/linlog.ts`, `src/content.config.ts`). A change of mode restarts the proof, with a warning first (D2, `Calculus.tsx`).
- **"Ordinary" is greyed.** The button in `TopBand.tsx` is `aria-disabled` with the tip "Ordinary propositional logic, through linlog's translations, comes later". `plan/web/README.md` ("Left out of version 1") and `plan/later.md` ("Ordinary logic's JSON forms") say why: the bindings lack the JSON forms of `ordinary::Outcome`, the ordinary sequent and its derivation. So the web page offers one logic family, linear logic, in six modes. It offers neither cyclic MLL nor the Lambek calculus (no mode names them), although the library has both (`docs/src/status.md`). Learn's lesson 11 teaches ordinary logic through the translations inside the linear tab (`src/content/lessons/11-ordinary.md`).
- **The stage.** D3: the derivation or the net fills the page, camera zooms and pans. D4: derivation and net are two views of one stage; the net tab is greyed with a reason unless the sequent has a net (MLL, essential nets of IMLL). `Stage.tsx` asks the bindings for an SVG (`draw` or `draw_net`), sets it with `host.innerHTML`, and decorates it. The decoration relies on an id scheme the library writes: `i<n>` on every conclusion and `i<n>-<p>` on every formula (`plan/web/README.md`, table of calls; setting `styles.svg.ids`). The page's clickable formulas are buttons laid over those ids.
- **The worker and its protocol.** One module worker loads one wasm module, `linlog-web` (`src/worker/linlog.worker.ts`). The protocol in `src/lib/protocol.ts` is generic: a request `{id, call, args}` answered by `{id, ok, value}` or `{id, ok:false, error}`; `answer()` looks the call name up in a table and the table is written out by hand in the worker (about 30 names: `start`, `view`, `rules`, `apply`, `undo`, `close`, `hint`, `draw`, `draw_net`, `export_as`, `certificate`, ...). Every call takes JSON text and returns JSON text, an SVG or bytes, and nothing is kept in the module between calls (`.claude/rules/web.md`), so the page holds the session and a worker can be terminated to cancel a search. Settings are JSON laid over the tab's defaults (`web/src/tab.rs`).
- **Module size.** 5.2 MB of wasm, 2.0 MB with `gzip -9` at step 34 (`plan/reports/34-web.md`); 6.9 MB after step 42's first-order logic (`plan/reports/42-first-order.md`, "The module's load", which also records that Chromium sometimes fails to write the module to its HTTP cache, and the worker then loads it past the cache). The compressed size at 6.9 MB is not recorded in the reports I read.
- **Budgets.** D35: the critical path under web.dev's 170 KB compressed, the wasm loading after first paint (the client's JS and CSS were 43 KB compressed at step 34, 55 KB at its last report, `plan/reports/34-web.md`). D49: a service worker caches the client's own files so the page works offline after one visit; the module is part of that cache.
- **Learn and Practice.** Learn is a frame around the stage (D47) with a tutorial of lessons (the stage plays a worked proof one inference at a time), practice and a reference of rules drawn as schemas (`src/content/lessons/`, `src/content/rules/`, 29+ rule files, one-sided and two-sided names). Practice draws from one store of 425 problems, each decided by linlog's command in a flake check, with level, categories and a session file (`src/content.config.ts`, client `README.md`). The content collections are written for linear logic only: `MODES` is the six modes and `CATEGORIES` includes `classical` and `intuitionistic` as linear modes.
- **The link and autosave.** `#v=1&m=<mode>&s=<sequent>[&d=<session>]`, the session deflated and base64url (`src/lib/link.ts`); every open proof is autosaved as `{id, input, mode, session, redo}` (`src/lib/shelf.ts`). Both are keyed by `mode`, a closed set of linear strings.

### How far this is already "a stage with tabs choosing the calculus"

(inference, from the files above.) Three of the author's four ingredients exist. The tab with its own calculus exists and is stored, linked and replayed. The stage is already generic at one boundary: it shows whatever SVG it is given, with the `i<n>` id convention for hit targets. The worker protocol is generic: a name-to-function table over JSON text. What is missing for "calculus and method":

1. **The mode is a flat closed enum of linear logics.** There is no notion of a *suite* (a logic family with its own sequent syntax and rules) above the mode and none of a *method* (sequent calculus, natural deduction, tableau, resolution, net, term) beside the calculus. D57's linear-or-ordinary segment is the one place the idea exists, and it is a stub. The Calculus popover mixes both ideas: its switches (Mix, affine, two-sided) are the parameters of one suite.
2. **The stage assumes a derivation tree grown bottom-up with one active goal** (D5), clickable formulas that list rules (D9), splits of a context, and a net view of a finished tree (D4). Those are properties of sequent-style proofs. A Fitch proof, a tableau, a resolution graph and a typed term are other structures (section 2.3).
3. **The wasm module is one and holds linear logic.** The call table is linear logic's. A second suite needs either more names in the same table or a second module.
4. **The written content is per logic, not per suite.** Lessons, rule schemas, problems, tour and the tutorial opening are all linear (`src/content.config.ts`); Practice's filters are connectives and the linear modes.
5. **No single-sequent "judgement" type.** `read`, `start`, `view` return linear-logic JSON (`fragment`, `goals`, `formulas`); there is no common shape that another suite's sequents or terms could fill.

## 2. The field

### 2.1 Precedents: how multi-logic and multi-method front ends choose

None of the pages below was run in a browser in this session; what follows is what their repositories, papers or documentation say.

- **Carnap** [1][2][3][4]. A Haskell framework with a GHCJS client in the browser and a GHC server; GPL-3.0; 2,469 commits on master (no date shown) [1]. The user does not switch logic in the UI: the instructor writes an exercise as a code block whose `system="..."` attribute "selects both the language and the derivation system (its rules and derivation style)", with names such as `LogicBookSD`, `openLogicNK`, `openLogicLK`, `montagueSC`, `thomasBolducAndZachTFL2019` [3]. Exercise *types* are separate: derivations, truth tables, translation, model checking, Gentzen-Prawitz natural deduction, sequent calculus, qualitative problems, truth trees and syntax checking [4]; the About page names truth tables, translation and parsing exercises and deductions for five textbooks [2]. It supports propositional and first-order LK and LJ [4]. The site lists about 31 institutions, noting the list is incomplete [2]. The paper describes an "open framework for formal reasoning" aimed at "ease of use for students and adaptability for instructors" [5]. Lesson for linlog (inference): in Carnap the *content author* binds an exercise to a system, and the checkers are separate widgets per exercise type, sharing a document and a course shell, not one stage. The student never has a tab where logic and method are picked.
- **LogicProof** (TEAL 2026 at FLoC) [6][7]. The closest recent precedent for this question. A single-page client-side application in vanilla JavaScript, with ANTLR4 parsers, D3 proof trees, Monaco editor, MIT licence; its three proof systems are Gentzen-style tree natural deduction, Fitch-style natural deduction and sequent calculus, each in classical and intuitionistic variants, propositional and first-order [6][7]. The paper says the choice is made "on the welcome screen": the user can "select the logic, its paradigm, and the proof system" there, and the Back button of the control panel returns to it [6]. So: one UI shell, a choice at entry, three views of three methods behind it. The repository shows 144 commits, 1 star, no date on the page I read [7]; the paper reports a study with 35 students, self-reported benefits [8]. (The authors are not named on the pages I read.)
- **The Incredible Proof Machine** [9]. Haskell logic core compiled with GHCJS plus a plain JavaScript UI, MIT licence; the repository has a `logics` folder, a `logic` folder and a `sessions.yaml` file, which suggests that logics and sessions are data files beside one engine (inference from the file names; I could not read their contents) [9]. Its site says it handles "various logics (e.g. propositional, predicate logic)" and does not describe a selector; the start dialog asks what to prove [10]. Its proofs are graphical blocks, a different view from a sequent tree.
- **Jape** [11][12]. "A configurable proof calculator": it has "no built-in knowledge of any object logic" and only makes a move if a rule of the loaded logic allows it; the distribution ships encodings of natural deduction, several sequent calculi, a protocol logic, Hindley-Milner typing and syllogisms; a manual explains how to write one's own [11][12]. GPL-2.0, 3,612 commits [11]. A note on the repository, signed by Bernard Sufrin and dated 15 October 2026, says Richard Bornat has died and that a successor to Jape is in development [11]. Jape is the standard precedent for "custom rules"; it is a desktop program, not a web page, so it informs the library side more than the stage (see the custom-rules note, if the supervisor commissions one).
- **Logitext** [13]. Sequent calculus on the web, Ur/Web over Coq, BSD-3-Clause, last push 2021-03-10 [13]. One logic. Click & coLLecT, the linear logic page, credits it as inspiration [14].
- **Click & coLLecT** (Callies and Laurent, TLLA 2021) [14][15]. Classical propositional linear logic, one-sided sequents, in a browser at click-and-collect.linear-logic.org; an OCaml backend and LGPL-2.1 per an aggregator listing that I could not confirm on GitHub [15]; used at the Linear Logic Winter School 2022 [15]. One logic, one method; it is the page linlog's own is compared with.
- **ProofWeb** (Radboud, 2006 to 2010 sources) [16]. A browser client uncoupled from a proof assistant on the server (Coq, later Isabelle), natural deduction following Huth and Ryan's textbook; instructors host courses on the Radboud server [16]. Choosing among assistants is a server-side configuration. I could not confirm that the service is still running.
- **Tree Proof Generator** (Seville, `cs.us.es`) [17]. Generates tableaux for classical propositional and predicate logic, with a free-variable tableau search and a countermodel [17]. One logic, one method.
- **Proof Tree Builder** (Princeton, arXiv 2303.05865) [18]. A web proof assistant for sequent calculus and Hoare logic of a small imperative language, with Z3; two systems, the abstract does not say how one is picked and no repository is linked [18].
- **Sequent Calculus Trainer** [19]. A trainer for sequent calculus with feedback on wrong rule applications (named in `plan/notes/research/scope/education.md`; I did not read the paper in this session).
- **jsCoq** [20][21]. "A port of Coq to Javascript" through js_of_ocaml, "no servers or external programs"; the README does not mention WebAssembly [20]. Libraries come as packages, ZIP archives of compiled `.vo` files with a JSON manifest, and `init_pkgs` lists those loaded at start (default `init`) [21]. Used by Coq Platform docs, EPIT 2020 and several winter schools [20]. This is the precedent for "the engine is one blob and the libraries are lazy data"; it does not show a module per logic.
- **Lean web editor (lean4web)** [22]. Runs Lean *server-side* behind a web page, Apache-2.0, with projects configured by the maintainers [22]. A precedent for the opposite choice (server), excluded by linlog's no-server goal.
- **Open Logic Project** (via Carnap). Its two proof systems, a sequent calculus and natural deduction (`openLogicLK`, `openLogicNK`), are named in Carnap's systems page [3]; I did not read the project's own site.

What the precedents show together (inference): the pattern that works is *one shell, many checkers or views* chosen either by the content author (Carnap) or at entry by the user (LogicProof). A per-tab choice of both logic and method, with several tabs open on different suites at once, has no precedent in what I read. It is a natural extension of linlog's existing tabs rather than a copy of anything.

### 2.2 The architecture options and what the sources say

- **Vite and wasm.** Vite loads a `.wasm` import with `?init` (a function that resolves to the instance) or `?url` (the client's choice, so the worker can compile it itself); dynamic `import()` is split into separate chunks, with automatic preload of shared chunks and with the CSS of an async chunk loaded before the chunk runs [23]. So a second suite's JavaScript glue and wasm can be lazily loaded by a dynamic import in the worker the first time a tab asks for it, using only documented features.
- **wasm-bindgen.** Each crate built with the CLI is a module with its own glue; the README says it "generates bindings only for the JavaScript imports and Rust exports you actually use" [24]. It does not describe sharing Rust's standard library or allocator across several modules, and I found no source saying two bindgen modules can share code. (inference) Two suites as two wasm modules therefore each carry their own copy of the code they share (the formula arena, the search runtime, the SVG layout), unless they are linked into one module.
- **One big module versus one per suite** (inference). The module is 6.9 MB today (about 2 MB compressed at 5.2 MB, step 34). A module for each of N suites costs the first-load budget only when the suite is used (D35 keeps the module off the critical path already), but the offline cache (D49) must hold every suite's module if every suite is to work offline after one visit; N modules of several MB each is the cost. One module holding every suite is simplest for the protocol and for sharing code, and costs every visitor the full download even if they use one suite. A middle path, one shared kernel module plus per-suite rule data (jsCoq's package model [21], and the Incredible Proof Machine's `logics` data folder [9]) works only where a suite is *data* for a generic kernel, which is the custom-rules question, not the question of a prover for tableaux or resolution.
- **What I could not source**: the compressed size of today's 6.9 MB module, the real size cost of a second Rust suite in wasm (no measurement exists), and whether `wasm-opt` plus shared allocators could trim duplicates.

### 2.3 Method-specific views against the generic stage

(inference, from the structure of each proof object; each view needs the library to give it.)

| method | shape | what the generic stage gives | what is method-specific |
|---|---|---|---|
| sequent calculus (LK, LJ, LL) | tree upward, goals, rule at a formula | all of it (D3, D5, D9, camera, ids) | nothing |
| natural deduction, Gentzen style | tree downward from assumptions, discharge marks | camera, ids, export | rules attached to a connective's introduction or elimination, discharge labels |
| natural deduction, Fitch style | vertical list with nested scope bars and line numbers | the shell, export | a list view, line numbers and justifications, scope bars (LogicProof draws both [6][7]) |
| tableau | tree growing downward, branches close with a mark | the camera, the shell | expansion of a chosen formula, closing a branch, an open branch's countermodel |
| resolution | clause set and a DAG of resolvents | the shell | a clause list and an inference graph, not a tree |
| proof nets | graph, links, switchings | the existing net view (D4) | the net editor (D4's "later") |
| simply typed lambda terms, system F | text with typing derivation and reduction steps | the shell, the camera for the typing tree | a term editor, a reduction stepper |

The generic part is therefore: the camera, the tab row, the shell, the status light, undo and redo as a step list, the export menu, the hint ladder, Learn's frame. The method part is what is drawn and what a click means. The existing seam is the SVG string with ids: for tree-shaped methods another suite can reuse the stage unchanged if it writes the same `i<n>` / `i<n>-<p>` ids and answers `rules`, `apply`, `undo`, `close`. For list, graph and text views the stage needs a second host element and a view per kind, chosen by the suite.

Accessibility (inference, from D33 and the existing stage): each view needs its own keyboard model (D17: every formula is a button, which holds for trees); a Fitch list is naturally a list with levels and is easier for a screen reader than a tree; a tableau and a DAG are as hard as the derivation tree is and need the same `description` text (`styles.svg.description`, `plan/web/README.md`). Nothing in the sources read measures this.

## 3. Candidates

### web-front-end-1: A suite registry and a per-tab suite choice, with linear logic as the one suite

- **What it brings and to whom.** The refactoring that every later suite needs, with no new logic. The client's `Mode` enum becomes `{suite, method, parameters}`; the tab's calculus mark becomes a choice among registered suites; the popover shows the suite's own switches (for linear logic today's classical/intuitionistic, Mix, affine, two-sided). Brings nothing new to a student today; it removes the single hard-coded assumption listed in section 1 so that web-front-end-2 and later suites are additions, not rewrites. Evidence of need: the greyed "Ordinary" stub (D57) is this seam half built.
- **What it offers in linlog's terms.** Decide, check, step, export as today. A *suite descriptor* in the client holds: the name, the parameters and their switches, the sequent field's syntax and example, which views the suite has (derivation, net, list), the worker module and its calls, the content collections (lessons, rules, problems) and the link and autosave keys. The link becomes `#v=2&u=<suite>&m=<parameters>&s=...`, and `shelf.ts` stores `{suite, ...}`. Old links and stored proofs read as the linear suite (a migration the link codec's version mark was made for, `link.ts`).
- **Cost (inference).** One session of the client (`state.tsx` is 1,398 lines and `Stage.tsx` 1,871, both would be touched; the link, shelf and content-config tests are many), plus half a session in linlog for a `suites` description (names and parameters) in the bindings' `about`. No new trusted code.
- **Risk.** Breaking links and stored proofs of existing users (mitigated by the version mark and a test that old forms read); designing the descriptor around one example (a second suite would then bend it). Mitigation: write the descriptor together with the first ordinary-logic suite (web-front-end-2), not before.
- **Prerequisites.** None in the library; the client's design D50, D57, D58 already say one tab one choice.
- **Sources.** Repository files in section 1; precedents Carnap's `system` attribute [3] and LogicProof's welcome-screen choice [6].

### web-front-end-2: Switch on "Ordinary" (classical, intuitionistic, minimal; LK and LJ) in the tabs

- **What it brings and to whom.** Students: the same stage for ordinary propositional logic, with the proof read back as LK or LJ, which is what a first logic course teaches (this is the most requested kind of tool in the precedents: Carnap, LogicProof, Logitext, Tree Proof Generator, ProofWeb all start with ordinary logic [3][6][13][16][17]). The existing stub, lesson 11 and the 35 ILTP problems that a loop check would settle (`plan/later.md`) are the evidence of need in this repository. AI models: a second suite in the same protocol, usable through the guide.
- **What it offers.** Already in the library (`core/src/ordinary/`, `docs/src/status.md`): decide, the proof read back as LK or LJ with a checker of its own, drawings, and a Rocq certificate over `Prop`. What is missing (`plan/web/README.md`, `plan/later.md`): the bindings' JSON forms for the ordinary sequent, outcome and derivation, and a step-by-step interactive session over LK or LJ (today the ordinary layer decides and reads back; *interactive* proving of LK or LJ rules by clicking is not stated as built, so I count it as missing, unsourced against `core-ordinary` rules which I did not read for this note). Without step-by-step, this suite would be "decide and show the derivation", which is half a student's use.
- **Cost (inference).** Two to three sessions: one in linlog for the wire forms and the bindings' calls (`start`, `view`, `rules`, `apply`, `draw`, `export_as`, `certificate` for ordinary sequents), one in the client for the suite, its sequent field syntax and its Learn content; a third if interactive LK or LJ proving is added to the library. Trusted base grows by the bindings' JSON only; the checker is already in the library.
- **Risk.** Termination on the image of a translation (`plan/later.md` states 35 of 274 ILTP problems would be refuted by a loop check, so the verdict "unknown" must be shown honestly); a student seeing the linear image rather than the ordinary proof (the page must show LK and LJ, never the image, by default).
- **Prerequisites.** web-front-end-1 (the suite descriptor) or, as a shortcut, filling the existing greyed segment; and `plan/later.md`'s "Ordinary logic's JSON forms".
- **Sources.** `plan/web/design.md` D2 and D57; `plan/later.md`; the precedents above.

### web-front-end-3: One wasm module, one protocol, suite as a namespace of calls

- **What it brings and to whom.** The simplest architecture for the next one to three suites that live in the `linlog` crate family: a single `linlog-web` module whose call table grows a prefix per suite (`ord_start`, `ord_apply`, ...) or takes a `suite` argument. The client's worker, protocol and cache need no change (`answer()` is already a name table, `protocol.ts`).
- **What it offers.** Every suite works offline once the module is cached (D49) and shares the kernel's code in one binary, so the size grows by each suite's own code only. (inference.)
- **Cost.** Per suite: its calls and a test each under Node (`web-wasm-tests`). The module grows: 6.9 MB today; the added size per suite is unmeasured and should be measured with the first one (a session's probe: build with the ordinary calls, compare `wasm-opt`'d sizes). The worker's cold start (compile of 6.9 MB, with the cache write failure noted in `plan/reports/42-first-order.md`) grows with it.
- **Risk.** A module of many suites cannot stay under a budget forever: the first suite that arrives from another repository (say a tableau prover with its own dependencies) forces a split. And one module means one crash domain and one release train.
- **Prerequisites.** None.
- **Sources.** `src/worker/linlog.worker.ts`, `src/lib/protocol.ts`, `plan/reports/34-web.md`, `plan/reports/42-first-order.md`; Vite wasm docs [23].

### web-front-end-4: A module per suite, loaded lazily by the tab

- **What it brings and to whom.** The only option that scales to a collection of suites from several repositories (the author's "incorporate the new repos"): each suite publishes its own wasm package with the same protocol; the client's registry maps suite names to a dynamic import; the worker loads a module the first time a tab of that suite makes a call. Visitors pay only for what they open.
- **What it offers.** Independence of release: a suite can be fixed and rebuilt without touching the others. The client's flake already takes `linlog-web` as a pinned input and builds no Rust (`CLAUDE.md` of the client), so N inputs is the same mechanism N times.
- **Cost (inference).** The registry and the lazy load: one session of the client; the second package's build (the flake module `modules/web.nix` copied per suite or generalised): about one session in whatever repository holds it. Offline: the service worker precache grows by every module the visitor may open, unless it caches a module only after the first use (a service-worker policy the client does not have today; D49 only says the client's own files are cached).
- **Risk.** Duplicated code in each module (section 2.2: no source says bindgen modules share anything); a heavier total offline cache; versions of the protocol drifting between suites (needs a protocol level in each module's `about`, as the session document already has a level, `src/lib/link.ts`).
- **Prerequisites.** web-front-end-1 and a written protocol (web-front-end-5).
- **Sources.** Vite [23], wasm-bindgen [24], jsCoq's lazy packages [21] as the precedent for load on demand, the client's `CLAUDE.md`.

### web-front-end-5: A written suite protocol (judgement, goals, steps, drawing) as the interface between libraries and client

- **What it brings and to whom.** The thing every suite and the client agree on, written once: the AI models that use linlog through `skills/linlog/` and the authors of future suites benefit as much as the page. It is the generalisation of `plan/web/README.md`'s "The bindings' API".
- **What it asks of a suite (inference, from the calls the stage uses).** (a) `read(text, settings)`: parse a judgement to a document with its fragment or class and its errors in the shared error JSON (`failure.rs`). (b) `start(judgement, parameters, settings)`: a session document with a level. (c) `view(session)`: goals with their formulas (positions that the page can click) and the inferences that the drawing's ids refer to. (d) `rules(session, goal, position)`: what a click offers, each with whether it splits and whether it needs a witness (`Offered` in `src/lib/linlog.ts`). (e) `apply(session, goal, step)`, `undo`, `close`, `close_all`, `hint`. (f) `draw(session, settings)` as SVG with the id scheme. (g) `export_as` and `certificate`. (h) a `describe()` of the suite: parameters, views, example sequents, syntax help, rule schemas for Learn. Everything stays JSON text in and out and stateless between calls (`.claude/rules/web.md`). Methods other than trees add a `layout` field to `view` (`tree`, `list`, `graph`, `text`) which says which host the stage draws into.
- **Cost (inference).** One session, mostly writing and a test that runs a suite against the protocol (a conformance test, in the style of `web/tests/calls.rs`), then each suite implements it. The existing linear calls already satisfy most of it; the gain is the rewriting of linear logic's `view` to the neutral shape.
- **Risk.** Fitting the neutral shape to tableaux, resolution and terms before one has been built: designed too early, it fits only trees. The sound order is to freeze it after the second suite works (web-front-end-2), and to treat it as a draft until the third.
- **Prerequisites.** web-front-end-2 as the second sample.
- **Sources.** `plan/web/README.md` ("The bindings' API"), `web/src/calls.rs`, `src/lib/linlog.ts`.

### web-front-end-6: Stage views for list, graph and text proofs (Fitch, tableau, resolution, terms)

- **What it brings and to whom.** The views that the sequent-tree stage lacks, needed by any method that is not a bottom-up tree (section 2.3). Teaching: natural deduction in Fitch style and tableaux are what Carnap, LogicProof and the Tree Proof Generator offer [3][6][17]; research: none.
- **What it offers.** In the client: a list host (rows with level, line number, justification; one active hole; rule choice at a row), a text host (a term, a reduction stepper), a graph host (a DAG or tableau tree growing downward, which can reuse the camera). The camera, the step list, undo, hint, export and Learn's frame are shared.
- **Cost (inference).** One session per kind of host, in the client, after the corresponding suite exists in a library; each also needs its keyboard model and a screen-reader description (D17, D33).
- **Risk.** Building views for suites that do not exist yet; keep each view coupled to its first suite.
- **Prerequisites.** web-front-end-5 and the suite whose view it is.
- **Sources.** LogicProof's three views [6][7]; the table in section 2.3 (inference).

### web-front-end-7: Learn and Practice keyed by suite

- **What it brings and to whom.** Teachers and students of each added suite. The lessons, rule schemas, tour and store are collections today with `MODES` and `CATEGORIES` fixed (`src/content.config.ts`); keying them by `suite` (a field, and a folder per suite under `src/content/`) lets each suite carry its curriculum, with the learning-progression rule of D60 per suite. Practice's filters become suite-specific, with a suite filter on top.
- **What it offers.** Each suite's problems still decided by its own command in a flake check, as the store is for linear logic (`problems.test.ts`, the flake's `problems` check).
- **Cost (inference).** One session of mechanism plus each suite's content as its own sessions (a first course for ordinary logic: five to eight lessons and 60 to 100 problems: the linear store of 425 problems and 13 lessons is the measure of the work). Content is CC BY 4.0 (client `README.md`).
- **Risk.** Content is the largest cost of a suite and the part that is easiest to neglect; a suite without lessons is a tool, not a course, and Learn's expertise-reversal rule (D47) means it can simply be off.
- **Prerequisites.** web-front-end-1.
- **Sources.** Client `src/content.config.ts`, `plan/web/design.md` D21, D47, D56, D60.

## 4. Considered and rejected

- **A server per suite** (as Lean 4 web does [22], ProofWeb [16] and Click & coLLecT's OCaml backend [15]): rejected by the stated goals (nothing is sent to a server; WebAssembly).
- **One tab hosting several methods of the same proof at once** (a sequent proof beside its Fitch form): attractive for teaching, but it needs a conversion between derivation forms for each pair; not sourced; keep as a later idea after two methods exist.
- **Rewriting the stage in a generic framework before a second suite exists:** the stage is 1,871 lines tuned to trees (camera, ids, popovers); (inference) a generic stage designed from one example is a worse guess than splitting it when the second host kind is real.
- **A JavaScript reimplementation of suites in the client** (as LogicProof does in vanilla JS [6]): it would give a second trusted base outside the Rust checker and the Rocq library; rejected against linlog's checking story.
- **Letting an author choose the suite per exercise only, as Carnap does** [3]: right for a course platform, wrong for the author's wish of a student-chosen tab.

## 5. What the structure must allow

- **Reusable as is** (from section 1): the tab row, the autosave and link machinery once keyed by suite, the camera and pan/zoom, the SVG-with-ids seam, the request-id protocol and the stateless call model, the export menu, the status light and verdicts three-valued, Learn's frame, the content collections' mechanism.
- **Linear-specific today:** `Mode`, the Calculus popover, `View`'s `fragment`, the `Offered` rules (one-sided names, `split`, witnesses), the net view, Learn's content and rule schemas, the problem store's categories.
- **What the client must gain:** a suite registry (descriptor in section 3, web-front-end-1), a per-suite module loader in the worker (web-front-end-4 or the namespace of web-front-end-3), a view kind per layout (web-front-end-6), content collections keyed by suite (web-front-end-7), a link and shelf format with a suite field.
- **What the libraries must gain:** the protocol of web-front-end-5, stateless JSON sessions with a level each, the shared error JSON, the SVG id scheme, a `describe()` for each suite. These are cheap for a suite built as a library with the call shape of `linlog-web` from its first day.
- **Budgets to hold** (D35, D49): critical path under about 170 KB compressed; a suite's module and content loaded on first use of that suite, never on the first load; the offline cache decision (all suites at install versus each suite after its first use) made explicitly.

## Recommendation (inference)

Do web-front-end-2 first, with web-front-end-1's registry written as part of it, not before; it gives the visible result (a second choice in the tab, the stub made real) at two to three sessions and teaches what the descriptor must hold. Then write the protocol (web-front-end-5) from the two suites that exist, and choose between one module (3) and a module per suite (4) at the moment a suite from another repository arrives, not earlier: until then one module costs nothing extra in the protocol, and the first-load budget is already protected because the module loads after first paint. Keep Learn and Practice per suite (7) as the content cost of each suite, and build non-tree views (6) only with the suite that needs one (natural deduction in Fitch style is the likeliest first). Total for the frame: about four sessions (1+2 together, 5, 7's mechanism), with each further suite adding its own library work, one client session for its suite entry and views, and its content.

## 6. Sources

Read on 2026-11-08 unless noted.

1. https://github.com/Carnap/Carnap (GPL-3.0, Haskell with GHCJS client; last commit date not shown on the page)
2. https://carnap.io and https://carnap.io/about (exercise types, about 31 institutions)
3. https://carnap.github.io/Carnap-Manual/systems.html (the `system` attribute and system names)
4. Search result summary of the Carnap manual, https://carnap.github.io/Carnap-Manual/quickstart.html and https://carnap.philosophy.ubc.ca/about (exercise types, LK and LJ); only the search summary was read, not the pages
5. https://arxiv.org/abs/1803.03092 (Carnap: an open framework for formal reasoning in the browser)
6. https://arxiv.org/html/2607.04321v1 (LogicProof, TEAL 2026: choice of logic, paradigm and proof system on a welcome screen, client-side)
7. https://github.com/6ximik9/naturalDeduction (MIT, three proof systems, 144 commits; the repository link is inferred from the paper's link annotations)
8. https://arxiv.org/abs/2607.04321 (abstract: 35-student study, accepted at TEAL 2026 at FLoC 2026)
9. https://github.com/nomeata/incredible (MIT, Haskell and JavaScript, `logic`, `logics`, `sessions.yaml`)
10. https://incredible.pm
11. https://github.com/RBornat/jape (GPL-2.0, note of 15 October 2026 on Bornat and a successor)
12. Search summary of Jape's papers and manuals (https://www.cs.ox.ac.uk/people/bernard.sufrin/jape.org.uk/DOCUMENTS/CURRENT/JapeForIFP.pdf and the natural deduction manual); search results only, not the PDFs
13. https://github.com/ezyang/logitext (BSD-3-Clause, last push 2021-03-10 per the search listing; the repository page itself was not fetched, a fetch of another URL had failed)
14. https://hal-lirmm.ccsd.cnrs.fr/lirmm-03271501 (Click & coLLecT, TLLA 2021; seen through a search listing)
15. https://www.goodfirstissue.org/ComputerAidedLL/click-and-collect (aggregator: OCaml backend, LGPL-2.1; unconfirmed) and https://perso.ens-lyon.fr/olivier.laurent/ll2022/ (winter school, via the search listing)
16. https://core.ac.uk/download/16156617.pdf and https://ar5iv.labs.arxiv.org/html/1005.2672 (ProofWeb, via search summary)
17. https://www.cs.us.es/~jalonso/cursos/li-06/sistemas/tableros/info.html (Tree Proof Generator help, via search summary)
18. https://arxiv.org/abs/2303.05865 (Proof Tree Builder)
19. https://arxiv.org/pdf/1507.03666 (Sequent Calculus Trainer; title seen in a search, not read)
20. https://github.com/jscoq/jscoq (js_of_ocaml, uses)
21. https://cdn.jsdelivr.net/npm/jscoq@0.17.1/docs/embedding.md and https://github.com/jscoq/jscoq/blob/master/docs/embedding.md (packages, `init_pkgs`; via search summary)
22. https://github.com/leanprover-community/lean4web (Apache-2.0, server-side Lean)
23. https://vite.dev/guide/features.html (WebAssembly `?init` and `?url`, dynamic import, code splitting, preload)
24. https://github.com/wasm-bindgen/wasm-bindgen (Apache-2.0 or MIT; generates bindings only for what is used)

Repository files: `/home/tux/Projects/own/linlog-prover.github.io/{README.md,CLAUDE.md,src/islands/{Calculus,TopBand,Stage,state}.tsx,src/lib/{protocol,linlog,link,shelf}.ts,src/worker/linlog.worker.ts,src/content.config.ts,src/content/lessons/11-ordinary.md}`; linlog `plan/web/design.md`, `plan/web/README.md`, `plan/later.md`, `plan/reports/34-web.md`, `plan/reports/42-first-order.md`, `.claude/rules/web.md`, `docs/src/status.md`, `plan/notes/research/scope/education.md`.

## 7. Not verified

- The compressed size of today's 6.9 MB module, and the size a second suite would add (no measurement; nothing was built).
- Whether two wasm-bindgen modules can share code, and whether a service worker can precache lazily per module: no source read says either.
- Carnap's current activity and its systems page's full list (the page lists names under several headings); the Carnap and Jape PDFs were read only through search summaries.
- ProofWeb's and Logitext's hosted services still running; Click & coLLecT's repository, licence and activity (aggregator only, unconfirmed on GitHub); the Tree Proof Generator's licence and activity.
- Whether the LogicProof authors' repository is the one in the paper (the link is inferred from annotations); its activity dates.
- Whether linlog has an interactive LK or LJ stepper for ordinary logic: I did not read `.claude/rules/core-ordinary*.md` for this note; the claim in web-front-end-2 that it is missing should be checked there by the supervisor.
- Lean's own browser editor, Open Logic's site, Alloy, modal-logic playgrounds and the Sequent Calculus Trainer's current page were not read.
- Nothing was run; the one measurement worth taking is the wasm size of the library with the ordinary calls added, under a capped scope.
