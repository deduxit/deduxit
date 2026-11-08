# Correctness verification of linlog, and its feasibility

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research sub-agent; sources read that day.

Nothing was built or run for this note. Sources marked [n] are listed in section 6; a claim without a number is read from the repository. "(inference)" marks a conclusion drawn from sources, not stated by them.

## 1. What linlog has today

What is trusted, and what is verified.

- **The Rust checker is the trusted base for every proof.** `core/src/proofs/check/` (`pass.rs` 1046 lines, `zones.rs` 365, `frames.rs` 378, `error.rs` 478, `mod.rs` whose first 46 lines are code and the rest tests) validates every proof any engine returns (`.claude/rules/core-proofs.md`). It shares no code with an engine, by rule. One bottom-up pass in arena order, no recursion; a node's `State` has `theta` (the least unrestricted zone), `gamma` (a multiset, a `HashMap<Member, u32>` over the crate's fixed-seed foldhash `hash.rs`), and `any` (a `⊤` above absorbs context). The rules file states the claim the review rests on: `State` characterises exactly the set of dyadic sequents the subterm proves.
- **Code shape of the checker** (read, not run): arena indices `NodeId` and `OccId` (`u32` newtypes) into `Box<[Node]>` and the forest; `u32` counts that saturate, `u64` sums of `u32` weights, `usize` lengths widened before summing; hash tables of members; a `dyn FnMut(Progress) -> bool` stop closure; generics (`examine<O: Observer>`, `impl Fn() -> I` with an `Instancing` trait for first-order frames); a second pass for error reports; memory and work accounting (`Pass::held`, `table_bytes` models hashbrown's layout); three `#[cfg]` sites, all in the tests. `unsafe_code = "forbid"` in the workspace `Cargo.toml`, so no `unsafe` anywhere in the library.
- **Why the integers are an argument and not a proof.** The rules file argues, from four facts (fewer than 2^32 nodes, two premises at most, fewer than 2^32 - 1 occurrences, a zone within `Bag::MOST`), that no integer wraps in any build. A real bug of that kind existed and was found by review: counters wrapped in a release build and a 131-node file was a "valid proof" of `⊢ !⊥, 1` (`refuses_a_zone_too_large_to_conclude`). This is the class of fault a tool that checks arithmetic exhaustively would find (inference).
- **Evidence for the checker today.** `proofs/oracle.rs` is the first implementation (bitset per node), kept for tests only; `agrees_with_the_first_implementation` requires the same verdict and the same error on engine proofs, families and mutants. Mutation testing (`mutants/run.sh`, `.claude/rules/evidence.md`) covers the checker, the readers, the search's front door, the Horn refutations and the ordinary layer's checker, with the survivors of each batch recorded under `mutants/baseline/`. Fuzz targets (`fuzz/fuzz_targets/`): `json_proof`, `json_disproof`, `json_sequent`, `json_session`, `json_structure`, `lltp`, `ordinary_*`, `proof_views`, `sequent_text`, `session_steps`, `spec`, `tptp`. The wasm32 tests of the checker's integers run under wasmtime (`web-core-wasm32`).
- **Refutations.** `Disproof::check` (`core/src/search/refutation.rs`, 1241 lines) verifies, without the search, the refutations of kind `Unbalanced`, `Equation`, `Classical` and their symbol variants; the kinds `Exhausted`, `OneSided`, `StateEquation` and `Saturated` have no check and no Rocq lemma (`docs/src/rocq.md`, `plan/later.md` "Follow-ups: certified refutations": "The one-sided and state-equation refutations have neither a check nor a Rocq lemma"). An unprovable verdict from the search alone is therefore trusted in the search engines.
- **The intuitionistic reading** (`occurrences/reading.rs`, `.claude/rules/core-forest.md`) reads a one-sided lowered sequent as a two-sided one, "what was written, and guesses nothing"; the checker, the derivation and the exports read it. It is trusted code; the Rocq side checks the two-sided term against the two-sided calculus, so the certificate does check the reading's result for the proofs it certifies (inference).
- **The parser** (`core/src/parse/`, about 1100 lines in `mod.rs` alone) and readers are trusted by definition: no proof can say that the text means what the user meant. They have mutation runs and fuzz targets.
- **The Rocq library `rocq-linlog`** [1] (own repository, pinned in `flake.lock`): the one-sided calculus with Mix, weakening and cut as parameters, the two-sided intuitionistic and affine calculi, proof terms as datatypes, checkers as functions, the theorem for every mode that an accepted term proves its sequent, the first-order development with de Bruijn indices and eigenvariables, invariants of derivations (the classical reading, an atom's balance, the count equation) from which "not derivable" lemmas follow, and bridges to NanoYalla's `nanoll.v` and Yalla's `nanoill.v` against pinned files. `nix flake check` requires every theorem's assumptions closed under the global context [1]. `--kernel linlog` is the default for Mix, affine, intuitionistic and first-order proofs; `--kernel nanoyalla` is the default for classical (`docs/src/rocq.md`).
- **What the certificate trusts.** A certificate is the sequent, a term and one `vm_compute`. Rocq's kernel and the library's definitions (`Formulas.v`, `Calculus.v`, `Intuitionistic.v` and the first-order files [1]) are trusted, plus the exporter that writes the sequent and term, the parser that made the sequent, and Rocq's own checking. The Rocq checker only runs on exported certificates, on the user's machine or in linlog's flake checks on test sequents; nothing connects it to the Rust checker's code except the flake checks and shared tests.
- **Stated precisely (inference).** The trusted base of a "proved" verdict printed by `linlog prove` is: the Rust checker (and Rust, LLVM, hashbrown, foldhash and the hardware), the parser, the reading. The base of a verdict a user takes to Rocq is: the parser, the exporter, the library's definitions and Rocq's kernel (the Rust checker drops out for that sequent). The base of "unprovable" is: the checker's `Disproof::check` for three kinds with the same extra Rocq route, and the search engines for every other kind.

## 2. The field

Each tool read in its own repository or documentation on 2026-11-08. Pages often showed no commit dates; where so, this is said.

**Deductive verifiers (the specification is in the source, solver-checked)**

- **Prusti** [2] (viperproject/prusti-dev, on Viper). Last tagged release 28 Feb 2024 (Rust `nightly-2023-09-15`) [3]; five automated nightly pre-releases between 26 Oct and 5 Nov 2026 [3]; `rust-toolchain` channel `nightly-2026-08-15` [4], so the head is kept current with a nightly. 277 open issues and 54 open pull requests [2]. Prebuilt binaries for Windows, macOS and Ubuntu [5]. Specifications: attributes (`requires`, `ensures`, loop `body_invariant!`), pure functions, predicates, external specifications, trusted functions, pledges; by default only panic and overflow freedom are checked, and it proves partial correctness (terminating runs only) [6]. The user guide page on closures says "NOT YET SUPPORTED" and "The syntax for attaching specifications to closures is currently not working" [7], and the feature list names closures and type models as supported or planned [6]. Whether `HashMap`, `Iterator` adapters and traits with associated types are specified was not found in sources read (unsourced).
- **Creusot** [8] (creusot-rs, LGPL-2.1, translates to Coma and Why3). Releases every one to three months: v0.14.0 on 2026-10-12 (toolchain 2026-08-03, newer Why3 and provers, constructor closures, aarch64), v0.13.0 2026-07-27, v0.12.0 2026-06-12, back to v0.5.0 in April 2025 [9]. The changelog shows continued iterator work (`Chain`, `FromIterator`, `DoubleEndedIterator`, `Iter`/`IterMut` specs), specs for slices and `NonZero`, `Wrapping`; its 0.4.0 entry of 2025-03-03 adds specifications for `HashMap` and `HashSet` iterators (#1306) and for `HashSet` (#1313) [9] (corrected at the review of 2026-11-08: the first reading said none). Installation: `rustup`, `opam`, `./INSTALL`; the repository has a `flake.nix` and a `nix` directory [8]. The guide at doc.creusot.rs could not be read (404 on both addresses tried).
- **Verus** [10] (verus-lang, SMT with Z3). Releases about weekly; latest 0.2026.11.02.8bf68ee (2 Nov) and a rolling one on 5 Nov [11]; pinned `rust-toolchain.toml` channel `1.99.1` [12], a stable compiler. States it supports "a subset of Rust (which we are working to expand)" and is "under active development. Features may be broken and/or missing" [10]. The guide's features page: closures supported as expressions (no mutable captures), traits with type arguments supported, `loop` and `while` supported, `HashMap`, `HashSet`, `Vec`, `VecDeque` supported, iterators and `for` loops only partially [13]. The solver does not usually do induction by itself; induction is written as recursive proof functions [14]. Code and specs are written inside Verus's macro in Rust syntax with `int` and `nat` and `forall`, `exists` [14].
- **Kani** [15] (model-checking, Apache-2.0 and MIT; bounded model checking with CBMC, bit-precise). `#[kani::proof]` harnesses with `kani::any()` inputs and `assert!`; checks panics, arithmetic overflow, undefined behaviour and user assertions or function contracts; loops need unwinding bounds or loop contracts [15][16]. Releases monthly, syncing with a Rust nightly: kani-0.68.0 built on nightly-2026-08-21 is the latest listed; loop contracts (`loop_decreases`, `loop_modifies`) and function contracts are being extended [17]. Concurrency out of scope; most core language features, traits, closures, `Box` supported; trait objects and function pointers partial [18]. Installation `cargo install --locked kani-verifier` and `cargo kani setup` [15]; a nixpkgs package was not found (unsourced).
- **Flux** [19] (flux-rs, MIT; refinement types for Rust, with an online demo). Specification and Rust version details were not on the repository page and the documentation site was not read (unsourced); no release listed [19].

**Translation to proof assistants (the proof is done in the assistant)**

- **Aeneas** [20] with **Charon** [21] (Apache-2.0). Charon extracts a crate's MIR as LLBC; Aeneas turns it into a pure functional program. "Our most mature backends are Lean and HOL4"; Coq and F* are listed as supported without a maturity note; no Isabelle [20]. Subset of safe Rust; not yet: `return` in nested loops and `break`/`continue` to outer loops, generics instantiated with a mutable reference, unsafe code and concurrency [20]. Charon calls itself "beta software" with open bugs [21]. The pages did not say whether closures, trait objects or iterators work, nor whether `HashMap` has a model (unsourced). Nix flake runs for both [20].
- **hax** [22] (cryspen, Apache-2.0): Rust to F* (marked stable), Rocq (experimental), Lean via Aeneas (recommended, active), ProVerif, SSProve and EasyCrypt (experimental); aims at full Rust except `&mut` in return types and aliasing [22]. Used for verified cryptography according to the interest group's page (HACL-Rust, Eurydice) [23].
- **rocq-of-rust** [24] (formal-land, formerly coq-of-rust): translates Rust to Rocq and lets one prove specifications there; "a work in progress in many parts", its main use is Revm against a functional specification [24].

**Foundational or unsafe-code oriented**

- **RefinedRust** [25]: refinement type system proved sound in Coq on Iris, for safe and unsafe Rust, "a prototype verification tool" evaluated on a `Vec` variant [25].
- **Gillian-Rust** [26]: hybrid verification of unsafe Rust (PLDI 2025), "unstable and will provide unclear error messages", no releases, 7 stars [26].
- The Rust Formal Methods Interest Group [23] lists, besides the above, crux-mir, Soteria Rust, MIRAI, KMIR (K semantics of Stable MIR), and monthly meetings; its page has no ranked list of tools [23].

**Extraction from Rocq (a checker run beside the Rust one)**

- Rocq's `Extraction` targets OCaml (default), Haskell and Scheme, and JSON for debugging; code supplied for realized constants is not checked; Rocq's refman page read did not state an extraction correctness theorem in the part read (the last 17 000 characters were not read) [27].
- MetaRocq [28] has a verified erasure ("the same as the type and proof erasure phase of the Extraction plugin") and cites a verified extraction pipeline from Rocq to OCaml/Malfunction (PLDI 2024, Forster, Sozeau, Tabareau), in a separate repository whose state was not read.
- CertiRocq [29] compiles to WebAssembly and to Clight; "large parts are verified, others still being verified"; the Wasm backend's maturity is not stated on the page [29].
- OCaml reaches the browser by js_of_ocaml and wasm_of_ocaml (same repository, needs binaryen 120 or later) [30]; size and speed of an extracted checker were not found in any source read (unsourced). There is no official Rust extraction from Rocq in the refman page [27] (inference from the three targets listed).

## 3. Candidates

The alternatives (a) to (e) of the brief are verification-4 (a), verification-5 (b), verification-3 (c), verification-2 (d) and verification-1 (e); two smaller steps, verification-6 and verification-7, come out of the evidence.

### verification-1: A second, differently shaped checker, and fuzzing the two against each other

- **What it brings, and to whom.** Research and the AI users of linlog trust a verdict the checker gives; a second checker whose design shares no data structure with `pass.rs` adds independent evidence. `oracle.rs` (803 lines, test-only) already is one, and `agrees_with_the_first_implementation` runs it on engine proofs and mutants; it differs in the state's data structure only and was written first by the same author as the rule, so it is not independent of the rule set (inference). A fresh implementation would be a plain recursive, tree-based checker over `Vec<Formula>` sequents with no sharing and no memory accounting: slow, small, readable (inference: about 300 to 500 lines).
- **In linlog's terms.** Decide and check only. A fuzz target (`fuzz/`) that feeds mutated `json_proof` inputs to both and compares the verdict, and a test over the generated families.
- **Cost.** One session. Adds nothing to the trusted base, adds a second implementation to maintain (every rule change is made in three places: the pass, the oracle, the new one).
- **Risk.** Agreement between two implementations by the same hand tests the transcription, not the rules; only a Rocq or paper-derived spec breaks that, which is verification-3 or 5.
- **Prerequisites.** None. **Sources.** The repository (`oracle.rs`, `core-proofs.md`).

### verification-2: Certify more verdicts with the Rocq library

- **What it brings.** To researchers who cite a "not provable" and to the web page's practice problems: the refutations `OneSided`, `StateEquation`, `Saturated`, `Exhausted` and the first-order ones have no certificate; the library covers only the classical reading, balance and the count equation [1]. Proofs of every mode, propositional and first-order, with cut, are already certified (`docs/src/rocq.md`, [1]) so there is nothing missing on that side except cyclic MLL, which the library does not cover [1], and the ordinary-logic readbacks (LK or LJ), whose certificates exist only as lemmas over `Prop` for the classical not-valid case (`status.md`).
- **In linlog's terms.** Check: add `Disproof::check` arms and a `linlog check` that reads a disproof (listed in `plan/later.md`). Export: `--format rocq` writes the lemma. Certify: the library proves each invariant once. Ordered by effort (`plan/later.md` "Certified refutations", inference on cost): (i) `OneSided` is the same balance argument with one sign below exponentials, a lemma of the same kind as the existing one; (ii) the state equation: a Farkas vector checked by computation after a lemma that a Horn program's proofs are firing sequences; (iii) `Saturated`, the inverse method's closed set, as a DRAT-like certificate with a checker proved sound once (the pattern `plan/later.md` names, with cake_lpr as the SAT example; the proof needs the completeness of the focused calculus for the fragment); (iv) finite phase models for MALL and affine fragments, whose soundness is short but whose finding is a search; (v) Kripke models for ordinary intuitionistic logic.
- **Cost.** (i) one session; (ii) one to two; (iii) a project of its own (three to five sessions, the largest proof in the list of `plan/later.md`); (iv) and (v) research. The Rust trusted base is unchanged; it moves "unprovable" from trusting the search engines to trusting Rocq for the covered kinds.
- **Risk.** With exponentials MELL's decidability is open and full linear logic is undecidable (`plan/later.md`), so no complete method exists; a certificate exists only where the refutation rests on a sufficient bound.
- **Prerequisites.** Work in `../rocq-linlog` first and then a lock move, per `CLAUDE.md`.
- **Sources.** [1], `plan/later.md`, `docs/src/rocq.md`.

### verification-3: Run a checker extracted from Rocq, in the command and in the browser

- **What it would prove.** The library's checkers are functions with soundness theorems; extracting one to OCaml gives an executable that is the verified function, up to extraction (Rocq's `Extraction` is not itself verified in the part of the manual read [27]; MetaRocq's pipeline is verified [28]) and up to the glue that reads a proof file into the Rocq datatype and the sequent's atoms (strings). The Rust checker would stay but its verdict would be compared with the extracted one.
- **Which part of the trusted base it removes.** None by itself: both checkers read a term the Rust side produced, so the reader of the term into Rocq's datatype becomes trusted instead (a JSON reader written in OCaml, or a printer in Rust, i.e. the exporter's existing role, inference). It gives a verified-by-construction check without Rocq installed, and in the browser a verdict the user need not take on faith. The same assurance as today is available to anyone with Rocq by compiling the certificate.
- **In linlog's terms.** Check only (and the interactive page's "proof is checked" badge). Prove, decide and export are unchanged.
- **Cost.** Native OCaml binary: two sessions (extraction of `check_classical` and the first-order checkers, a driver, a flake output, a differential test against the Rust checker on every family). Browser: add wasm_of_ocaml [30], whose output size and run time were not found in a source (unsourced); a further one to two sessions, and the client's size budget must be asked. The wasm bundle would hold the extracted code and the OCaml runtime next to linlog's own wasm.
- **Risk.** Atoms are strings and the extracted code is slower than the Rust pass (it has no per-node sharing accounting; inference). The memory and work bounds, which the Rust pass has so that a hostile file cannot exhaust a browser tab, would need rewriting in the extracted code's outside, which is unverified. The OCaml toolchain (OCaml, binaryen at 120 or later [30], opam) joins the flake. No official Rust extraction target exists [27]; rocq-of-rust [24] goes the other way (Rust into Rocq).
- **Prerequisites.** A stable proof-term wire format shared with the library (the JSON proof already exists).
- **Sources.** [27], [28], [29], [30], [1].

### verification-4: Verify the Rust checker's functional correctness with a deductive verifier

- **What it would prove.** That the Rust function `Proof::check` accepts exactly the proofs of the calculus, which needs the calculus stated in the tool's specification language (an inductive predicate over sequents, or a ghost model) and the `State` characterisation as the loop invariant, plus panic and overflow freedom. It removes the Rust checker from the trusted base, down to the verifier, its solver and rustc.
- **The comparison on linlog's own code.** The code is mostly the hard shape for every tool: an arena of `Copy` enum nodes indexed by `u32` newtypes (fine for all), a pass with `HashMap<Member, u32>`/`HashSet` of foldhash (a model is needed in each tool; Verus's guide lists `HashMap` as supported [13], Creusot's changelog shows no `HashMap` specs [9], Prusti not found), `u32` saturating arithmetic and `u64` sums (all handle bounded integers; Prusti checks overflow by default [6]), iterators and closures (`State::iter` returns `impl Iterator`; `&mut dyn FnMut` for the stop: closures are "NOT YET SUPPORTED" in Prusti [7], supported as expressions without mutable captures in Verus [13], and not listed in the Creusot changelog entries read besides constructor closures [9]), traits and generics (`Observer`, `Instancing`: Verus supports traits with type arguments [13]), the second pass for errors (functional correctness of error reports is not needed: prove the verdict only), and the memory accounting (`table_bytes` mirrors hashbrown's layout: not verifiable and not needed; specify `held` only as bounded).
- **Prusti against the others (inference, with the author's preference of 2026-10-23).** Prusti is not clearly worse on specification style, which is attributes on ordinary Rust, close to what the rules file says in prose. It is behind on three read facts: its closures are not supported [7]; its last tagged release is of Feb 2024 on a nightly of 2023 [3] (the head tracks a nightly from August 2026 [4] but the release channel is nightlies only); and the page read lists no std `HashMap` or iterator specification (unsourced either way). Prusti proves partial correctness only [6], so termination would be argued separately. Verus is the better fit on these facts: it has the closest support for the code shape (`HashMap`, traits, `while`, a stable compiler 1.99.1 [12], weekly releases [11]), and its limits are that it asks the verified code to be written inside its macro dialect, so `core/src/proofs/check` would be a verified copy kept in step with the rest (inference), and it pins one compiler, which linlog's `channel = "stable"` toolchain would have to be separate from. Creusot is the alternative where ghost sequences and `FMap` models make a multiset spec natural (v0.14.0 has `FMap` ghost specs [9]); it needs nightly-pinned `cargo creusot` and Why3 provers [8][9]. Prusti would be chosen only if its closure and `HashMap` limits were removed by restructuring the code (replace the stop closure by a trait object with a pure interface, the table by a `Vec`-based model): that restructuring is the cost. (inference) Verdict of the comparison: no tool is clearly better by what was read about Prusti alone; the sources say Verus and Creusot are more actively released and cover more of the code shape, so the preference clause "Prusti where as good a fit" is not satisfied on closures and release state.
- **Cost.** A project of its own: four to eight sessions for the pass's core (multiset state, the rules' transitions, the root test) after a refactor of `State` onto a verified-friendly table and the removal of the accounting from the verified path, then ongoing proof maintenance with every rule change. First-order frames, intuitionistic reading and cut triple it. The tool would join the flake (no nixpkgs package was found for any of Prusti, Verus, Kani; Creusot and Aeneas ship a flake in their repositories [8][20]; unsourced for nixpkgs).
- **Risk.** The specification is a second statement of the calculus: its correctness against the Rocq calculus is again a human check unless verification-5 closes it; the proof effort follows the `State` characterisation, whose invariant (least `theta`) is not simple.
- **Prerequisites.** A stable, small core of the checker with the accounting separated.
- **Sources.** [2] to [14].

### verification-5: Prove that the Rust checker refines the Rocq checker, through Aeneas or hax

- **What it would prove.** Aeneas turns a safe-Rust crate into a pure functional program in Lean, HOL4, F* or Coq [20]; hax into F*, Rocq (experimental), Lean via Aeneas [22]. One would then prove that the translated function accepts a term iff the existing Rocq function `check` does, and the library's soundness theorem carries over: the Rust checker then is verified against the Rocq calculus, the strongest statement available, with the Rocq calculus as the single specification (inference).
- **State of the tools.** The mature Aeneas backends are Lean and HOL4, not Rocq [20]; the library and its theorems are Rocq, so either the equivalence goes through a Rocq-Lean bridge (none found) or the Rocq backend must be used, whose maturity is not stated [20]. hax's Rocq backend is "experimental" [22]. Known gaps: nested-loop `return`, generics over `&mut`, unsafe [20]; closures, iterators and `HashMap` were not stated by the pages read (unsourced). The checker's `dyn FnMut` stop and `impl Fn` constructors, the `HashMap` and the accounting would all be outside the supported subset or without a model (inference); the verified core would be a reduced pass.
- **Cost.** A project of its own, larger than verification-4 (six to twelve sessions, inference): a translatable core, a model of the table in the target assistant, the refinement proof, and a toolchain (Charon's pinned nightly, OCaml, Lean or Rocq) in the flake.
- **Risk.** Depends on two beta tools [21]; a fault in the translation is trusted. **Prerequisites.** verification-4's refactor of the core. **Sources.** [20] to [22].

### verification-6: Kani harnesses for the integer and bound facts

- **What it would prove.** Within a bound on the proof's size, for all inputs of a harness, that the pass never panics and no integer overflows or wraps: `Bag` saturation, `State` sums, `Proof::new`'s bound checks, `table_bytes`. These are the arguments the rules file makes in prose and the class of the one fault found. Kani checks panics, overflows and assertions, bit-precisely [15]; unbounded loops need unwinding or loop contracts [16][17].
- **Cost.** One to two sessions: harnesses under `#[cfg(kani)]` in the crate (the `unexpected_cfgs` lint needs the cfg declared), a CI job outside `nix flake check` unless nixpkgs packages Kani (unsourced). The pass's hash table makes large symbolic states costly for CBMC (inference); harness small zones.
- **Risk.** Bounded, not a proof for all sizes; the bound is the finding. Toolchain follows a nightly of Kani's choosing [17]; the repository's `stable` channel is not Kani's. **Prerequisites.** None. **Sources.** [15], [16], [17], [18].

### verification-7: A proof-carrying export of Rust verdicts for the web page

*Found built at the review of 2026-11-08*: the client's bottom band already exports the finished proof's and a refuted goal's Rocq certificate (`src/islands/BottomBand.tsx`, `api.certificate`), so this candidate is dropped; the paragraphs below are kept as written.

- **What it brings.** A cheap middle path using what exists: the web page's "checked" badge could offer "download certificate" with `--kernel linlog` text (the exporter is in `core/` under feature `rocq`, in the wasm build per `status.md` "every feature but parallel"), so a user verifies a displayed proof in their own Rocq with no change to the trusted base. Cost: half a session if the page does not already show it (the client was not read for this note; unsourced). It extends the certified part without new theory.
- **Sources.** `docs/src/rocq.md`.

## 4. Considered and rejected

- **Verifying the whole library or the search engines** with any tool: the search is not what soundness rests on, because every result is checked; a verified engine would verify performance code that changes monthly. The check-by-checker design (`core-proofs.md`) is what makes verifying the checker alone enough (inference).
- **RefinedRust and Gillian-Rust**: aimed at unsafe code [25][26]; linlog forbids `unsafe`, so their strength is unused and both are prototypes.
- **rocq-of-rust** [24]: translates Rust into Rocq with proofs by hand; "work in progress in many parts", a large Rocq model per construct, and used on one big codebase; weaker than Aeneas for this purpose by the maturity read (inference).
- **Flux**: refinement types would express index bounds well, but the specification of a multiset's functional correctness is outside refinement types' reach (inference), and nothing on its page read states the supported features.
- **A verified extraction to wasm through CertiRocq** [29] for the browser: the page gives no Wasm status; wasm_of_ocaml [30] on the unverified extraction is the more usual path, and the trust gained over the Rust checker is nil (verification-3).
- **Proof by reflection of the whole decision procedure** (`plan/later.md`): the largest development, the kernel runs the search; only small MALL instances.

## 5. What the structure must allow

- The checker is already separated from every engine and is a pure function of a proof and a mode: the right place for any of these. A tool wants this core small: the accounting (`Pass::held`, `table_bytes`, the stop polling, the second pass for reports) is orthogonal to the verdict and is what would be set aside. A split of `pass.rs` into the verdict transition function and the observer-and-bound wrapper would serve verification-4, 5 and 6 alike (inference).
- The proof terms are a first-class wire format (JSON, `goal`, `mode`), which makes verification-3 and the fuzz comparison of verification-1 possible without touching the engines.
- A generic kernel for other logics (steps in the other 46a notes) would need its own spec per calculus; the Rocq library is per calculus too. What carries over is the method (checker as function, soundness theorem, certificate by computation), not code.
- The linear-specific content is `State`'s two zones and the `any` flag; arena indices, saturating counters and the pass loop are reusable.

## 6. Sources

Read 2026-11-08 unless stated.

1. https://github.com/linlog-prover/rocq-linlog (README: calculi, theorems, certificates, refutations, closed assumptions, bridges).
2. https://github.com/viperproject/prusti-dev (repository page; 7 540 commits, 277 issues, 54 pull requests).
3. https://github.com/viperproject/prusti-dev/releases (last release 28 Feb 2024; nightlies to 5 Nov 2026).
4. https://github.com/viperproject/prusti-dev/blob/master/rust-toolchain (`nightly-2026-08-15`).
5. https://github.com/viperproject/prusti-dev/blob/master/docs/user-guide/src/install.md (binaries; VS Code assistant).
6. https://github.com/viperproject/prusti-dev/blob/master/docs/user-guide/src/verify/summary.md (verification features; partial correctness; default checks).
7. https://github.com/viperproject/prusti-dev/blob/master/docs/user-guide/src/verify/closure.md (closures not yet supported).
8. https://github.com/creusot-rs/creusot (Why3/Coma, LGPL-2.1, install, flake).
9. https://raw.githubusercontent.com/creusot-rs/creusot/master/CHANGELOG.md (v0.12.0 to v0.14.0) and https://github.com/creusot-rs/creusot/releases.
10. https://github.com/verus-lang/verus (repository page).
11. https://github.com/verus-lang/verus/releases (0.2026.11.02, rolling 5 Nov).
12. https://github.com/verus-lang/verus/blob/main/rust-toolchain.toml (channel 1.99.1).
13. https://verus-lang.github.io/verus/guide/features.html (supported and partial features).
14. https://verus-lang.github.io/verus/guide/ (overview; Z3; induction).
15. https://github.com/model-checking/kani (repository page).
16. https://model-checking.github.io/kani/ (what it proves; contracts; concurrency out of scope).
17. https://github.com/model-checking/kani/releases (kani-0.68.0 on nightly-2026-08-21; loop contracts; the page shows no years).
18. https://model-checking.github.io/kani/rust-feature-support.html.
19. https://github.com/flux-rs/flux.
20. https://github.com/AeneasVerif/aeneas (backends, limitations, Nix).
21. https://github.com/AeneasVerif/charon.
22. https://github.com/cryspen/hax.
23. https://rust-formal-methods.github.io/ (interest group; tools named in meetings).
24. https://github.com/formal-land/coq-of-rust (rocq-of-rust).
25. https://plv.mpi-sws.org/refinedrust/.
26. https://github.com/GillianPlatform/Gillian-Rust.
27. https://rocq-prover.org/doc/V9.0.0/refman/addendum/extraction.html (first 100 000 characters of 117 789).
28. https://github.com/MetaRocq/metarocq.
29. https://github.com/CertiCoq/certicoq.
30. https://github.com/ocsigen/js_of_ocaml (wasm_of_ocaml, binaryen 120).

## 7. Ranking (inference)

1. **verification-2 (i) and (ii)**: certify `OneSided` and the state equation. Best value per session, extends the certified part, uses what exists; (iii) is a project of its own.
2. **verification-6**: Kani harnesses for the integer facts. One to two sessions, targets the one class of fault actually found in the checker; bounded, not a proof.
3. **verification-1**: a second checker and a differential fuzz target. One session, adds evidence, adds maintenance.
4. **verification-7**: certificate download on the page, half a session, if absent.
5. **verification-3**: extracted checker, native (two sessions) then wasm (one to two more). Worth doing only if a no-Rocq user or the browser needs a verified verdict; it removes nothing from the Rust base.
6. **verification-4**: deductive verification of a refactored core, a project of its own; if started, compare Verus and Creusot first by a spike on the multiset state, since the sources read do not support Prusti on closures and release state.
7. **verification-5**: refinement to the Rocq checker via Aeneas or hax, the strongest statement and the largest project; the Rocq backends are the least mature of those listed.

## 8. Not verified

- Nothing was built or run. I would have run, to settle the open points: a Kani harness on `Bag` (to see CBMC's cost on hash tables); `cargo creusot` and Verus on a copy of `zones.rs` (to see whether `HashMap<Member, u32>` and the `impl Iterator` returns are accepted); `wasm_of_ocaml` on an extraction of `check_classical` (size and time). Each needs a capped scope on given cores.
- Unsourced: Prusti's std `HashMap`, iterator and trait support; Prusti's current maintainers; nixpkgs packages for any tool; Flux's feature list, toolchain and releases; Creusot's guide (doc.creusot.rs returned 404); Aeneas's closure, iterator and `HashMap` support and its Rocq backend's maturity; hax's last release date; Kani's release years (the page showed none; 0.68.0 is read as 2026 from its nightly's date); wasm_of_ocaml output size and speed; the status of verified extraction (coq-verified-extraction repository not read); CertiRocq's Wasm backend status; Gillian-Rust's and rocq-of-rust's last commit dates; the rest of the Rocq extraction manual page (17 789 characters unread). The web search budget ran out during the session, so a few lookups (Gillian-Rust, wasm_of_ocaml sizes) were done by fetching repository pages only.
- The "second checker" size, all cost estimates in sessions, and the rank order are my inference, not sourced. Whether the web client already offers a certificate download was not checked.
