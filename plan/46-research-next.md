# Step 46: research, what to add after 0.2.0

You are working in the linlog repository. CLAUDE.md applies throughout: jj
only (never git), no pushing. This step writes no code. It comes after
the release of 0.2.0 (step 45), so that it looks at what was released.
It has two parts with the author between them, as step 17's had: the
research, which ends in a ranked list of candidates and the questions
only the author can answer; and, once the author has chosen, the
prompts of the steps that follow. This prompt was written on 2026-10-23
at the author's word. It says what is fixed.

Read `plan/17-assessment.md` and its report (how the plan from step 18
on was made), `plan/later.md` (every candidate and follow-up still
unassigned), the scope notes of `plan/notes/research/scope/`
(applications, education, engines, interop, logics, proof theory,
semantics and decision, higher types), README (what linlog promises),
the reports of steps 38 to 45, `bench/TOOLS.md` and the comparison's
gap study (`plan/notes/research/comparison-gaps.md`).

## Goal

The candidates for linlog's next year, each with what it brings and to
whom (research, teaching, the AI models step 44 wrote for), what it
costs, what it risks and what it needs first, ranked; and the steps the
author then chooses, with their prompts.

## Fixed now (the author, 2026-10-23)

- **Read in sources, never from memory**: research sub-agents (Sonnet
  5.5 at `medium`), one per area, each reading the literature, the other
  provers and tools, and the standards it names, with links, and each
  briefed with what linlog has today (README and the reports), so that
  nothing it proposes is built already. The areas are at least the
  scope notes' eight, updated for what was built since they were
  written, and these beside them: the web page as students and teachers
  use it, the interfaces of other tools (proof assistants, TPTP and
  ILTP, editors), what models and agents need of linlog beyond step 44's
  guide (an MCP server, if step 44 left it to the author), and
  maintenance (dependencies, toolchains, wasm, accessibility).
- **Correctness verification, and its feasibility** (the author,
  2026-10-23), an area of its own. What linlog trusts today: the Rust
  checker and the checks of refutations (`Disproof::check`), the
  intuitionistic reading (and step 40's recorded sides), the parser,
  tested by differential tests, mutation runs and fuzzing; what is
  verified: the Rocq library's checkers with their soundness theorems,
  which the exported certificates of classical proofs rest on, and the
  invariants behind some refutations. The research weighs, with what
  each would cost here and what it would prove: verifying the Rust
  checker itself (Verus, Creusot, Prusti, Kani's bounded model
  checking, Aeneas or hax translating Rust to Lean, Rocq or F*), proving
  it refines the Rocq checker, running a checker extracted from Rocq
  beside or instead of it (in the command, in the browser's wasm),
  certificates for more verdicts (intuitionistic proofs, every kind of
  refutation) checked by the library, and which part of the trusted base
  each would remove. It says what is feasible in a step or two and what
  would be a project of its own, and ranks it with the rest. The
  author's preference among the tools (2026-10-23): where verifying the
  Rust code is feasible and Prusti is as good a fit as the others,
  Prusti; another tool only where it is clearly better suited, with the
  reasons said. The comparison is made on linlog's own
  code (the checker's loops, its arena indices and integers, the
  features of Rust it uses), each tool's state and maintenance read in
  its sources.
- **What users say, from what is public**: the repositories' issues and
  discussions, citations of the Zenodo DOI, crates.io's download counts,
  read through the public web and GitHub's public API (no `gh` task,
  which asks the author first), and nothing else, since the page tracks
  no one.
- **The synthesis** on Fable 5.1 at `high`, the plan's model for a
  session whose decisions every later one builds on: the research notes
  under `plan/notes/research/next/`, one per area, and the ranked list
  in the report, every candidate with its sources; the decisions for
  the author in one message through the planning session, each with a
  recommendation, as the author asked on 2026-10-23 ("concise plain
  language, a recommendation each, and ask me to decide").
- **Then the prompts**, after the author's answers: the chosen steps,
  numbered from 47, in the plan's table with their models, sessions and
  dependencies, and `plan/later.md` true (each candidate assigned, kept
  for later with the reason, or dropped with it).

## The author's direction (2026-11-08)

At the go for this step the author widened its question, in these words:

> I actually thought that maybe it would be nice to have not only a
> linear logic suite, but maybe a collection of suites for many
> different kinds of logic. The web-app frontend could then maybe have
> a unified view with the center stage and for each stage (per tab) one
> could choose the calculus and proof method that one wants to work on.
> I'm thinking about the following (but feel free to suggest more if
> you think they're a good idea to add):
> - ordinary classical, intuitionistic and minimal sequents, some
>   substructural variants
> - natural deduction, resolution
> - tableaus
> - modal logics (various kinds)
> - cyclic proofs (infinite descent)
> - simply typed lambda calculus, system F
> - maybe a way to add custom derivation rules and a way to
>   interactively build derivations with them
>
> I have no idea about how to structure a project like this. I might
> need/want to rename the linlog-prover org to something else and
> incorporate the new repos for the new features.

So the research answers that direction first and ranks the rest beside
it. What it must settle for the author, each from sources:

- **Each logic and method the author names**: what linlog has of it
  today (it already decides ordinary classical, intuitionistic and
  minimal propositional logic through its embeddings and reads the
  proofs back to LK and LJ, `core/src/ordinary/`; it has cyclic MLL and
  the Lambek calculus, interactive proving, the exports and the Rocq
  library); the calculi, decision procedures and their complexity; the
  other tools (provers, proof assistants, teaching tools) and how they
  are used; and what a suite for it would offer in linlog's terms
  (decide, check, prove step by step, export, certify), at what cost.
- **What else is worth adding**, as the author invites: further logics
  or methods, each with the reason and the evidence of use.
- **Custom rules**: how a user could define a calculus and build
  derivations in it interactively, what logical frameworks and
  rule-description languages do (from LF and Isabelle's meta-logic to
  tableau-prover generators and the rule languages of teaching tools),
  what can be checked, searched or only stepped through under such
  rules, and what that would ask of linlog's derivations and checker.
- **The structure of a collection of suites**: one repository with
  many crates or a repository per suite; a shared kernel (formulas and
  judgements, derivations, a checker against rule schemas, interactive
  proving, the exports, the web bindings) or engines of their own per
  logic; what in linlog's core is generic and what is linear-specific;
  how other multi-logic projects are laid out and maintained. Speed
  and WebAssembly stay goals.
- **The unified web front end**: one stage with tabs, each choosing its
  calculus and proof method; how the client's stage, Learn and Practice
  would carry it, with precedents.
- **The name and the organisation**: whether `linlog-prover` and the
  name `linlog` should change, and what a rename touches: GitHub's
  redirects for an organisation and its repositories and their limits,
  the Pages site (the client's repository is named
  `linlog-prover.github.io`), crates.io's names and Trusted Publishing's
  binding to a repository, Zenodo's GitHub integration and the DOIs
  already cited, the opam package `rocq-linlog`, the Cachix cache
  `linlog-prover`, the published manual's URLs. Name candidates are
  checked for clashes (GitHub, crates.io, opam, domains) from public
  sources. Nothing is registered or renamed: that is the author's.
- **The order**: what to generalise first and what to add in which
  order, so that each step leaves a released, useful tool.

The decisions for the author then include the structure, the name, the
first logics and their order, the scope of custom rules and the web
front end's shape, each with a recommendation.

## What step 45 left you

- **0.2.0 is released**, linlog's and the Rocq library's (the Status
  entries of 2026-11-02 and 2026-11-03): the two-sided surface, MELL
  nets, cut and its elimination, first-order logic, the Rocq library in
  its own repository `linlog-prover/rocq-linlog`, the logo; `linlog` and
  `linlog-cli` on crates.io, packages for Linux, macOS and Windows, both
  archived on Zenodo (linlog's concept DOI 10.5281/zenodo.23448588, the
  library's 10.5281/zenodo.23899109).
- **The comparison**: the fourth baseline (its parallel stage marked
  not comparable, a rerun waiting for a night), the growing families,
  and the other provers on them, where Click & coLLecT's and APLL's
  wrong refutations turned out to be an integer overflow in their
  shared search (`bench/TOOLS.md`). LinearOne decides eight first-order
  problems linlog does not (`plan/later.md`).
- **Prepared, the author's to do**: the Rocq library's opam pull
  request (the report's section 45e), the messages to the other
  provers' authors, the LLTP header report. nixpkgs was skipped at the
  author's word.
- **Open with the author** (`plan/handoff.md`): code signing, install
  channels that need an account, an MCP server, a Zenodo community, a
  desktop app.

## How the step runs

Two sessions with the author between them, no code and no benchmark
run:

- **46a, the research and its synthesis**: Fable 5.1 at `high`, its
  research sub-agents on Sonnet 5.5 at `medium`, one per area (the
  areas above and the author's direction's, which may split by logic),
  each briefed with what linlog has today. A fresh-context reviewer on
  Opus 5.5 at `high` checks, on a sample of each note, that its claims
  about other tools say what their cited sources say, that nothing it
  proposes as new is built already, and that the ranking follows from
  the notes. The decisions go to the author in one message through the
  planning session.
- **46b, the prompts**, after the author's answers: Opus 5.5 at
  `high`. The chosen steps from 47 on, in the plan's table, and, if the
  author chooses the collection of suites, the roadmap that sets their
  order, as step 17 set the steps from 18.

Deliverables: `plan/notes/research/next/`, `plan/reports/46-research-next.md`,
the prompts of the chosen steps.
