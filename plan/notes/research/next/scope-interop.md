# Interoperability and the interfaces of other tools, updated

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

This note updates `plan/notes/research/scope/interop.md` (interop-1 to interop-10, written 2026-10-09 to 17) and adds what the author named an area of its own on 2026-10-23: the interfaces of other tools (proof assistants, the TPTP World, ILTP, editors). Candidates keep their old names; new ones continue from interop-11. "(inference)" marks a conclusion of this note's own; "(unsourced)" a claim for which no source was found. Searching the web stopped working during the session (the shared budget was spent), so some claims rest on page fetches alone.

## 1. What linlog has today

- **Wire forms with versions.** Every JSON document carries `version`, its wire level: the sequent (level 1 classical, 5 intuitionistic in the examples), the proof (`check` reads it), the proof net, the interactive session, the refutation payloads (`docs/src/output-formats.md`, `skills/linlog/reference.md` section 8). There is no JSON Schema, and the rule names and preorder numbering are documented in the manual and the guide, not as a standalone specification (a search of `docs/src` and `skills/` finds no schema; (inference) interop-1 is therefore partly built).
- **Certificates.** `--format rocq` writes a lemma for the NanoYalla kernel or, with `--kernel linlog` (the default for Mix, affine, intuitionistic and first-order proofs), for the library `rocq-linlog` in its own repository, with soundness theorems for the checkers; first-order certificates and certificates of non-derivability exist (`docs/src/status.md`, `docs/src/rocq.md`). For ordinary logic a certificate over `Prop` needs no library (`docs/src/ordinary-logic.md`). The opam submission of `rocq-linlog` 0.2.0 to the Rocq opam repository was prepared in step 45e (`plan/reports/45-release-0.2.0.md`, "Session 45e"); whether it has been opened is the author's act and is not shown in the report. The library has no tactic or plugin (`../rocq-linlog` holds `theories/`, `bridges/`, `tests/`, no `Ltac` or plugin code).
- **TPTP and its neighbours.** A reader for TPTP `fof` problems under `--logic` ("as the ILTP library writes them", `nix build .#iltp`, 274 propositional problems), the LLTP library's reader, `.spec` for coverability, the batch with directories and JSON Lines (`docs/src/ordinary-logic.md`, `docs/src/batch.md`, `docs/src/status.md`). A search of `core/src`, `cli/src`, `docs/src` and `skills/` for "SZS" finds nothing: no SZS status lines, no `% SZS output` blocks.
- **Bindings.** `linlog-web` (unpublished): wasm-bindgen, JSON in and JSON, SVG or bytes out, tested under Node; the library reads no clock and uses threads only behind `parallel` (`docs/src/status.md`, `CLAUDE.md`). No Python package, no WIT component, no language server, no tree-sitter grammar (searches of `plan/later.md` and `docs/src` find only the planned "editor plug-in or notebook reuses the same JSON", `plan/later.md` lines 134 to 135).
- **For models.** The guide `skills/linlog/` (step 44) describes the command for AI models, its examples run by `cli/tests/readme.rs`.
- **Exports for typesetters.** LaTeX (ebproof, cmll) and Typst (curryst) fragments and standalone documents, which need no editor support beyond the packages (`docs/src/output-formats.md`).

Nothing below re-proposes these.

## 2. The field, as read today

### 2.1 Lean and cslib

- cslib, "The Lean Computer Science Library", Apache-2.0, 775 commits, no releases listed on its page ([1], read 2026-11-08). `Cslib/Logics/` holds `BTL`, `HML`, `LinearLogic/CLL`, `Modal` and `Propositional` ([2]); `CLL` holds `Basic.lean`, `CutElimination.lean`, `EtaExpansion.lean`, `MLL.lean` and `PhaseSemantics/` ([3]). The page shows no commit dates ((unsourced) for activity).
- The Logics README says proof systems instantiate an `InferenceSystem` class, default to `Type` rather than `Prop` so that computations over derivations are possible, fragments are given by a predicate (MLL is the example), and the authors plan "shared interfaces for manipulating proofs" over `InferenceSystem` ([4]). (Inference) cslib's modal and propositional logics beside linear logic make it the one Lean library where a multi-logic exchange of derivations could land.
- Lean's `IO.Process.output` runs a child process synchronously and returns exit code, stdout and stderr; `spawn` gives handles ([5]).
- `lean-smt`: the `smt` tactic translates the goal, sends it to cvc5, replays the proof in Lean and returns holes as goals; Apache-2.0, beta, no releases ([6]).
- `bv_decide` (Lean 4.12.0 release notes, read in the earlier note) checks an LRAT proof with verified algorithms and so depends on `Lean.ofReduceBool` (unchanged since 2026-10-09, not re-read).

### 2.2 Rocq

- CoqHammer: `hammer` translates problems for external provers (Vampire, CVC4, E, Z3) and reconstructs proofs with `sauto`; LGPL 2.1; the dev branch is for Rocq 9.2 ([7]). The page does not say how the provers are invoked.
- `rocq-lsp` (formerly `coq-lsp`): LSP 3.17 with Rocq extensions, the `petanque` protocol "aimed at AI and software engineering uses", clients for VS Code, Emacs, experimental Vim and Neovim, Python (`coqpyt`), LGPL 2.1; citation version 0.2.5 of 2025-11-30 ([8]).
- Ltac2 declares OCaml functions with `Ltac2 @ external` ([9], only the first 100 000 of 164 417 characters read). I found no Ltac2 primitive to run a process in the part read (the rest unread); a Rocq tactic that shells out to `linlog` would therefore be an OCaml plugin, or, with no plugin, a script that runs the command and lets `rocq` compile the certificate (inference). The second way is what the existing flake checks already do.
- Isabelle: Sledgehammer registers remote provers by name through SystemOnTPTP and raises an error when a system is not offered there (Isabelle's `atp_systems.ML` and `system_on_tptp.scala`, seen only as search results, not opened; (unsourced) in detail).

### 2.3 Dedukti and Lambdapi

- Lambdapi: 411 stars, 4 113 commits ([10]); `CHANGES.md` lists 4.0.0 on **2026-11-05** with "Export to Lean", 3.0.0 on 2025-07-16, 2.5.1 on 2024-07-22 with `raw_dk` ([11]). The licence is not stated on the page read.
- Dedukti: CeCILL-B, 2 280 commits; "only the emacs mode is maintained currently" refers to editor modes ([12]). (Inference) the new Lean export is the first route from a Lambdapi theory into Lean; whether it covers a theory written for sequent calculi is unsourced.
- I found no linear logic encoding in either (the search budget ended before a second search; the earlier note's one search found none; absence not proved).

### 2.4 The TPTP World

- The TPTP is at **v9.3.1** (Fri Aug 21 2026), its home moved to `tptp.tptp.org`; it links QMLTP (quantified modal) and ILTP ([13]).
- Since v9.0.0 it supports non-classical logics: a `logic` role annotated formula (`tff(semantics,logic,$modal == [...])`), one per file, normally first; a file without one is classical; the properties have no defaults. Five modal families are standardised (`$modal`, `$alethic_modal`, `$deontic_modal`, `$epistemic_modal`, `$doxastic_modal`), `$temporal_instant` is in a footnote ([14]). NXF and NHF are the non-classical typed extended first-order and higher-order languages (NHF curried application, NXF first-order application) ([15], from a search result summary of the 2024 talk and the paper).
- The paper says the framework "can represent" the binary connectives of linear logic, and lists "obvious candidates include linear logics and intuitionistic logics" among the logics to standardise next, plus combinations of logics and a non-classical division of CASC ([14]). This is the primary source for interop-5(c): the TPTP World itself names linear logic as a candidate.
- NTF provers integrated into the TPTP World: KSP (propositional), nanoCoP-M and MleanCoP (first order, restricted modalities), Leo-III; the ATFLET tool embeds NXF/NHF into TFF/THF so that E and Vampire can be used ([14]). The repository `TPTPWorld/NonClassicalLogic` (84 commits, 2 stars, folders `LogicSpecifications`, `Logics`, `SampleProblems`, `Tooling`) holds the specifications and sample problems; no licence or dates on the page ([16]).
- SZS: a status is reported once on a line `% SZS status <value> for <problem>`; a justification between `% SZS output start <DataForm> for <problem>` and `% SZS output end ...`; values: Theorem, ContradictoryAxioms, CounterSatisfiable (with conjectures), Satisfiable, Unsatisfiable (without), Timeout, GaveUp, Error (no success); data forms Proof, CNFRefutation, Model, FiniteModel, InfiniteModel, Saturation; extra information after a colon ([17]).
- SystemOnTPTP: submits a problem to chosen systems, checks syntax, converts to the system's syntax under a time limit, extracts the result, and with `-S` asks for a TPTP-format proof; derivations can be viewed with IDV ([18]). How a new system is registered: **not found**. The TPTP page gives no submission process ([13]); a Springer chapter's abstract says users "provide ATP systems/tools for SystemOnTPTP and StarExec" ([19], from a search snippet). The practical route is to write to the TPTP's maintainer (inference); the requirements of the "guidelines for ATP system evaluation" were not read.

### 2.5 ILTP

- iltp.de: "About 2800 propositional and first-order benchmark problems in a standardized syntax", release v1.1, footer "01.03.2017", maintainers Thomas Raths and Jens Otten, no news ([20]). So the library is stable and unmaintained in the sense of no update since 2017 (inference from the footer; the page names no syntax; linlog's own reader treats the files as TPTP `fof`, `docs/src/ordinary-logic.md`). TPTP v9.3.1 still links it ([13]). linlog reads the 274 propositional problems already; the first-order problems need first-order intuitionistic logic, which linlog does not have as such (its first-order linear logic is not LJ with quantifiers).

### 2.6 Editors, bindings and components

- LSP: the spec's navigation shows 3.18 current and 3.19 upcoming; hover, publish and pull diagnostics, code actions, inlay hints and semantic tokens exist ([21]). `lsp-server` 0.10.0 of 2026-07-16, "a language server scaffold, exposing a synchronous crossbeam-channel based API" ([22]).
- Tree-sitter: a grammar is a `grammar.js`, a test corpus and highlight queries; the docs warn of a "difficult learning curve" ([23]). (Inference) a grammar of sequents is small because the syntax is an expression grammar with a binding table.
- WASI 0.3.0 was released 2026-06-11 with async in the component model (`async func`, `stream<T>`, `future<T>`), 0.3.1 on 2026-08-11; Wasmtime 46 is the first to implement the final spec; a 0.2-capable setup need not migrate; toolchain and runtime must target the same WIT version ([24]). Rust support for 0.3 is not described on the page.
- PyO3: current examples pin 0.30.0, CPython 3.9 or later, PyPy and GraalPy, Rust 1.85 or later; free-threaded support not stated on the page ([25]). The earlier note's PyO3 0.29.2 and maturin 1.15.0 are not re-read.

## 3. Candidates

### interop-1: the proof term as a specified exchange format (status: partly built)

*Today.* Wire levels exist everywhere (section 1); a specification apart from the manual's examples, with a JSON Schema and stable rule names, does not. *Brings.* To research: an independent checker for a foreign prover's output, and the shared form on which interop-2 to 6 lean. To models: a schema is what a tool-calling model validates its output against (inference). To teaching: little. *Suite.* Check only (the checker already exists); decide, prove and export unchanged. *Cost.* 0.5 to 1 session: the schema generated from the serde types (a `schemars`-style derive is a new dependency, to be justified by `new-tool`), a test that the schema accepts every example of the manual, stable rule-name strings. Trusted base: none added. *Risk.* A schema that lags the wire level; mitigated by a test per level. *Prerequisites.* None. *Per suite or shared.* Shared in a collection: one envelope (version, logic, sequent, derivation as rule-schema instances) with per-logic payloads is the one exchange format a multi-logic project could have (inference); the proof term over occurrences is linear-specific in its details. *Sources.* Section 1; [1] to [4] for what a Lean consumer would need.

### interop-2: Lean 4 certificates over cslib (status: untouched; prerequisites changed)

*Changed.* cslib still has classical CLL with `CutElimination` and `MLL` ([3]) and still no release ([1]); its README now states its design (inference systems in `Type`, fragments by predicate) ([4]), which fits a reflective checker. rocq-linlog is now a library in its own repository, so the Lean one would be a second repository beside it (inference). Lambdapi 4.0.0 adds an export to Lean ([11]), so interop-3 now has a second road to Lean. *Brings.* Research: certificates for the second assistant of the community (inference); teaching little. *Suite.* Certify only. *Cost.* 3 to 4 sessions: a Lean port of the checker and its soundness theorem against cslib's inductive for classical LL, nothing for ILL, affine or Mix, which would be definitions of linlog's own; the trusted base is Lean's kernel plus the port. *Risk.* cslib moves without releases, so a pinned commit and a lake dependency are needed (Lean's `lake` plugin practice not read: (unsourced)). *Prerequisites.* interop-1's stable terms; the exporter's targets table (`Kernel`). *Per suite.* Per suite, unless cslib's `InferenceSystem` interface becomes the shared target for the other logics' suites (inference). *Rank:* see section 6.

### interop-3: a Dedukti/Lambdapi theory of linear logic (status: untouched; prerequisites changed)

*Changed.* Lambdapi's 4.0.0 of 2026-11-05 adds Lean export ([11]); the Dedukti and Lambdapi repositories show no encoding of linear logic in what was read ([10], [12]). *Brings.* Research: one small checker and a route to several assistants (inference); the first linear theory there would be new if none exists (absence not proved). *Suite.* Certify and export only. *Cost.* 2 to 3 sessions: the theory (short), the exporter, a flake check running `lambdapi`/`dk` (a new tool for `new-tool`); trusted base: Dedukti's checker plus the theory's faithfulness, which needs review by Deducteam (inference). *Risk.* The exchange rule in a deep encoding; unknown how Lambdapi's translations treat a theory with multiset sequents. *Prerequisites.* The exporter's target table. *Per suite or shared.* Shared in a collection: Dedukti was built for many logics at once (the earlier note, from the Dedukti paper [26], read 2026-10-09); a single exporter to Lambdapi over a generic derivation could serve every suite (inference). This is the strongest candidate for "one exchange format for derivations across logics" on the proof-checking side.

### interop-4: focused certificates with holes (status: untouched)

Unchanged from the old note: certificates proportional to the choices, in the style of foundational proof certificates ([27], [28], read 2026-10-09). Nothing built since touches it; step 45's certificates of non-derivability are the cheap refutation part. Research only; cost 4 or more sessions with the largest proof (completeness of focusing for the fragment) ; not recommended before the collection's kernel question is settled (inference).

### interop-5: TPTP conventions in and out (status: untouched; the primary source now supports it)

Three parts, re-assessed.

- **(a) SZS lines.** `--szs` printing `% SZS status Theorem|CounterSatisfiable|Timeout|GaveUp|Error for NAME` and the proof between `% SZS output start Proof` and `end` ([17]). Mapping (inference): provable to Theorem (with a conjecture), unprovable to CounterSatisfiable only when the refutation is a countermodel in the classical sense, otherwise to a status the ontology has for "no proof but complete" which I did not find in the subset read ([17]); unknown to GaveUp, a time limit to Timeout, a parse error to Error. For linear logic the words `Theorem`/`CounterSatisfiable` mean theoremhood in linear logic, which the TPTP does not define (inference): hence (c). *Cost:* 0.5 session; the batch's problem names already exist. *Brings:* TPTP-based drivers (Isabelle's Sledgehammer registry, StarExec, SystemOnTPTP) can run linlog on ordinary-logic problems (inference; for ordinary logic classical `fof` the verdicts are the standard ones). For ILTP, intuitionistic problems get `Theorem` and `CounterSatisfiable` as ILTP intends (inference).
- **(b) TFF/THF input.** Only when quantifiers need typed terms; no demand found. Keep rejected unless first-order ordinary logic comes (it is in the candidates of the ordinary-logic notes).
- **(c) A logic specification for linear logic in the non-classical syntax.** The TPTP authors name linear logic an "obvious candidate" for standardisation and say the framework can represent its binary connectives ([14]); a proposal would be a `logic` annotated formula such as `$linear` with properties (exchange, weakening, contraction, which connective set) plus linlog and the LLTP library as the first problems and system. LLTP's files are the existing practice (`docs/src/commands.md`). *Cost:* 1 session to write the proposal, the specification and sample problems in TPTP's format, then months of correspondence (the author's, an outward-facing act); the repository `TPTPWorld/NonClassicalLogic` has `LogicSpecifications` and `SampleProblems` ([16]). *Risk:* the specification of a substructural logic's properties is research the paper does not do; linlog's modes (affine, Mix, cyclic) multiply properties.
- *Per suite or shared:* TPTP is where classical and modal suites would meet the community's infrastructure (inference): modal logic is standardised ([14]) with NTF provers and a CASC division planned; a modal suite of the collection could read and write NXF natively and be the first system with a native interactive proof reader, which is an interface linlog would build once for the collection. Linear logic would be the proposal.

### interop-6: tactics that call linlog from a proof assistant (status: untouched; shape clarified)

*Lean.* A tactic is plain Lean using `IO.Process.output` with `args` and reading `exitCode` and `stdout` ([5]); with `linlog prove --format json` or `--format lean` (interop-2) the result is re-checked in Lean, as `lean-smt` replays cvc5's proof ([6]). Cost 1 session after interop-2. *Rocq.* No Ltac2 primitive to run a process was found ([9], partly read); a plugin in OCaml, or an `rocq` file generated by the command and compiled, as the flake does today. A plugin adds an OCaml build to `rocq-linlog` and a dependence on the Rocq plugin API (inference); the lighter route is a Python or shell wrapper that writes the certificate (already `--format rocq`). *Isabelle.* Not recommended: the only logic found there is the old `Sequents` object logic (earlier note). *Brings.* Research, mostly; for models, `rocq-lsp`'s `petanque` ([8]) is the interface an AI agent uses to drive Rocq, so a model proving a Rocq goal could call `linlog` through the guide (inference). *Prerequisites.* interop-1, interop-2.

### interop-7: a Python package (status: untouched)

PyO3 0.30.0 with CPython 3.9 or later ([25]); wheels via maturin (earlier note). *Brings.* Teaching notebooks and models' tool code (the guide's command suffices for models already, spawning the binary: inference). *Cost.* 2 sessions (crate, CI wheels per platform, a stop callback, the GIL released). *Per suite.* One package for a collection with a module per suite (inference). *Rank.* Below the web and below the guide; see section 6.

### interop-8: a WebAssembly component with WIT (status: untouched; the tooling matured)

WASI 0.3.0 (2026-06-11), 0.3.1 (2026-08-11), Wasmtime 46 implements the final spec, and 0.2 setups need not migrate ([24]). The earlier objection ("still landing") is weaker; the Rust side is still not described on the page (unsourced). *Brings.* One sandboxed artefact for hosts other than browsers (inference); the web page needs none. *Cost.* 1.5 to 2 sessions (WIT over the JSON forms, a `wasm32-wasip2` build beside `linlog-web`, tests under Wasmtime). *Rank.* Low until a host asks.

### interop-9: a language server and a tree-sitter grammar (status: untouched)

*Brings.* People who keep files of sequents (teaching sets, the LLTP-style problems) get diagnostics, hover on the fragment, "prove" as a code action and verdicts as inlay hints (LSP 3.17 has all four, [21]); `lsp-server` 0.10.0 is the maintained scaffold ([22]). A tree-sitter grammar is small ([23]). *Evidence of use.* `rocq-lsp` is how Rocq users edit (clients in VS Code, Emacs, Neovim, [8]); for a language as small as a sequent file no evidence of demand was found (unsourced). *Cost.* Grammar 0.5 session; server 1.5 sessions; a VS Code extension 0.5. Needs parser spans and error recovery (core-sequents rules). *Typst and LaTeX editors.* Nothing to build: the export is a fragment for `ebproof`/`cmll` and `curryst`; Tinymist and LaTeX Workshop work on the file as any other (inference, not sourced). *Per suite.* A language server over a generic parser trait would serve a collection; the grammar per syntax.

### interop-10: notebooks (status: untouched)

Unchanged; needs interop-7 or the web bindings. The web page already offers the stage in a browser; a notebook widget is a second client of the same JSON (inference). Low.

### interop-11: SZS and TPTP for the collection, native NXF reading (new)

*What.* After interop-5(a), a reader and writer of the TPTP's non-classical syntax for the modal suite (when it exists), so that the modal suite is a system in the TPTP World's tools: `$modal` specifications with the axiom systems K to S5 ([14]), SZS status and proofs in TSTP form. *Brings.* Research: a comparison with Leo-III, nanoCoP-M and MleanCoP on the TPTP's own sample problems ([14], [16]). *Cost.* 1.5 sessions on top of a modal suite. *Risk.* The NTF paper's interpretation format for Kripke models is for countermodels; its exact syntax was not read here. *Prerequisite.* A modal suite (`modal-logics.md`).

### interop-12: a version-checked "linlog serve" for tools (new, small)

*What.* A line-oriented or JSON-RPC mode (`linlog serve`) reading requests (sequent, logic, options) and answering the JSON form, so that a Lean or Rocq tactic, a Python wrapper, an editor or a model keeps one process instead of spawning per call; the batch already reads JSON Lines and standard input (`docs/src/batch.md`), so the first step is nearly built (inference). *Cost.* 0.5 session. *Brings.* The base that interop-6, 7 and 9 would share. *Risk.* Duplicates the web bindings' calls; the contract must be one JSON vocabulary (interop-1).

## 4. Considered and rejected

- **OpenTheory.** Not re-read; the earlier note (from FPC13) had no source for HOL-only; a Dedukti route (interop-3) covers the same role (inference).
- **Alethe, LFSC.** As before: SMT proof formats whose rules are SMT reasoning.
- **Isabelle export.** As before; Sledgehammer's SZS interface ([2.2]) is reached through interop-5(a) for ordinary logic, not through a certificate.
- **Agda export.** As before.
- **A Jupyter kernel for linlog's syntax.** As before.
- **tower-lsp.** As before.
- **ILTP first-order problems.** They need intuitionistic first-order logic proper (inference); not through linear logic's quantifiers without a translation not built.
- **Joining SystemOnTPTP now.** No registration process was found; with no SZS output and a linear logic the TPTP does not define, a listing would have no problems to run (inference). Do (a) and (c) first.

## 5. What the structure must allow

- Reusable, from `.claude/rules/core.md` and its module files (read as the brief says): the wire-level discipline, the serde forms, the stop interface, the batch, the exports' targets, the checker's one-pass shape. Linear-specific: the occurrence-numbered proof term and its rule set, the preorder promise, the certificates' library.
- For a collection: an envelope (version, logic name, payload) for the JSON forms; an exporter that takes a generic derivation (rule name, premises, conclusion) so that Lambdapi, LaTeX, Typst and SVG are written once; a `Verdict` with its cause as data so that SZS words map; a parser trait with spans. These are the same four items the old note's section 4 asked for (inference).
- Which interfaces are per suite: certificate kernels (Rocq, Lean) and logic specifications for TPTP. Shared: the JSON envelope, the Lambdapi export, SZS, LSP, Python, the component, the tactics' process call.

## 6. Re-ranking (inference)

The order inside each audience, with the reason in the shortest form.

- **For research.** (1) interop-5(a) and (c): the TPTP World's own paper names linear logic a candidate and the work is mostly a specification and sample problems ([14]); highest visibility for least code. (2) interop-3 Lambdapi: now with a Lean export ([11]), the one route to several assistants. (3) interop-1 schema (prerequisite of both). (4) interop-2 Lean: valuable but a second proof effort on a moving library without releases ([1]). (5) interop-6, 4, 11.
- **For teaching.** (1) What exists: the web page, LaTeX and Typst fragments. (2) interop-9's grammar and server only for course repositories of problem files; (3) interop-10 notebooks. interop-8 and 7 do not serve teaching directly (inference).
- **For AI models.** (1) interop-1 (a schema models validate against) and interop-12 (one process per session); (2) interop-6 tactics, because `rocq-lsp`'s `petanque` and Lean's `IO.Process` are how agents already reach assistants ([5], [8]); (3) interop-7. The guide `skills/linlog/` already covers the binary.

Overall order (inference): interop-1, interop-5(a), interop-12, interop-5(c) (the author's act), interop-3, then interop-2 and interop-6 as one step, then interop-9, interop-7, interop-8, interop-10, interop-4.

## 7. Sources (all read 2026-11-08 unless marked)

1. https://github.com/leanprover/cslib (Apache-2.0, 775 commits, no releases).
2. https://github.com/leanprover/cslib/tree/main/Cslib/Logics
3. https://github.com/leanprover/cslib/tree/main/Cslib/Logics/LinearLogic/CLL
4. https://github.com/leanprover/cslib/blob/main/Cslib/Logics/README.md
5. https://lean-lang.org/doc/reference/latest/IO/Processes/
6. https://github.com/ufmg-smite/lean-smt
7. https://github.com/lukaszcz/coqhammer
8. https://github.com/ejgallego/coq-lsp
9. https://rocq-prover.org/doc/V9.0.0/refman/proof-engine/ltac2.html (first 100 000 characters)
10. https://github.com/Deducteam/lambdapi
11. https://github.com/Deducteam/lambdapi/blob/master/CHANGES.md
12. https://github.com/Deducteam/Dedukti
13. https://tptp.tptp.org (the old address tptp.org/TPTP/ redirects there)
14. https://arxiv.org/html/2508.09318v1, A. Steen, G. Sutcliffe, "TPTP World Infrastructure for Non-classical Logics" (first 100 000 characters; abstract at https://arxiv.org/abs/2508.09318)
15. Web search summary naming NXF and NHF (https://arxiv.org/pdf/2508.09318 and https://tptp.org/TPTPTParty/2024/SpeakersSlides/AlexSteen.pdf, not opened)
16. https://github.com/TPTPWorld/NonClassicalLogic
17. https://szs.tptp.org (the SZS ontology; the old page tptp.org/UserDocs/SZSOntology/ redirects)
18. https://www.cs.miami.edu/home/geoff/Courses/TPTPSYS/Practicum/UsingATPSystems.shtml
19. https://link.springer.com/chapter/10.1007/978-3-031-63498-7_3 (search snippet only)
20. https://www.iltp.de/
21. https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/ (first 100 000 characters)
22. https://docs.rs/crate/lsp-server/latest
23. https://tree-sitter.github.io/tree-sitter/creating-parsers/
24. https://wasi.dev/releases/wasi-p3
25. https://github.com/PyO3/pyo3
26. https://arxiv.org/abs/2311.07185 (Dedukti; read 2026-10-09 in the earlier note, not re-read)
27. FPC17: https://hal.archives-ouvertes.fr/hal-01390912 (not re-read)
28. LKU: https://www.lix.polytechnique.fr/~dale/papers/lku.pdf (not re-read)
- Repository: `docs/src/status.md`, `docs/src/ordinary-logic.md`, `docs/src/output-formats.md`, `docs/src/batch.md`, `skills/linlog/SKILL.md`, `plan/reports/45-release-0.2.0.md`, `plan/later.md`, `plan/notes/research/scope/interop.md`.

## 8. Not verified

- cslib's activity dates, and whether any file of `CLL` covers ILL or affine linear logic (the earlier note: classical only).
- Whether Lambdapi's Lean export handles a theory with sequent calculi, and Lambdapi's licence; whether any linear logic encoding exists in Dedukti or Lambdapi (one search, then the budget ended).
- How a system is added to SystemOnTPTP; the "guidelines for ATP system evaluation"; the SZS status for "unprovable but complete" in a logic that has no models (only a subset of the ontology was shown); the NTF interpretation format's syntax.
- The syntax of ILTP's files (the page does not name it; linlog reads them as `fof`), and ILTP's maintenance beyond its footer date of 2017-03-01.
- How CoqHammer and Sledgehammer invoke provers in detail; whether Ltac2 or a Rocq plugin can run processes (part of the page unread); Lean `lake` plugin practice.
- Rust's support for WASI 0.3 and jco's and componentize-py's status in 2026; PyO3's free-threaded support; maturin's current version.
- Demand for a sequent-file language server (no evidence found), and how Tinymist or LaTeX editors treat the exports.
- The year of Lambdapi 4.0.0 is from `CHANGES.md` (2026-11-05); the releases page showed no year.
- Not run: nothing needed running.
