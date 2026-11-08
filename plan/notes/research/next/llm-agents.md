# What AI models and agents need of linlog beyond step 44's guide

Written 2026-11-08 for step 46a (`plan/46-research-next.md`) by a research
sub-agent; sources read that day.

## 1. What linlog has today

- **The guide** (step 44, `plan/reports/44-llm-guide.md`): the Agent Skill
  `skills/linlog/` (`SKILL.md`, 99 lines, front matter plus an `llms.txt`-shaped
  body; `reference.md`, 964 lines, 19 sections), served on the rustdoc site as
  `llms.txt`, `llms-full.txt` and `reference.md`, and by the web page's own
  `llms.txt` that points to them. `cli/tests/readme.rs` runs every example of
  the guide and fails on a command, flag, session command or settings key that
  does not exist, and holds every stated default to `--help`. Trials with
  Haiku 5.5 and Sonnet 5.5 solved 7 of 7 tasks after two fixes.
- **Machine-readable output**: `prove --format json` (`verdict`, `checked`,
  `refutation` tagged by `kind`, `reason` tagged by `kind` with the flag that
  lifts the bound, `statistics`, the proof as nodes that `check` reads back);
  an error of the library as `{"code","kind","message","setting","details"}`
  (`reference.md` sections 5 and 8); exit statuses 0, 1, 2, 3, 130; the batch
  (`--input-format lines`, one JSON line per answer); `--settings` files;
  `seq json`; `interact` driven from a pipe with `goals`, `rules`, `apply`,
  `cut`, `undo`, `close`, `save`, `load`, `proof`.
- **The web bindings' JSON protocol** (`web/`, `reference.md` section 17):
  about thirty calls of JSON text in and JSON, SVG or bytes out that keep
  nothing between calls (`start`, `rules`, `apply`, `hint`, `close_all`,
  `prove`, `export`, `certificate`, the cut and net calls). The guide names them
  and says an agent with a shell uses the command instead.
- **Checking**: `check` verifies a proof file; the exported Rocq certificates
  (`--kernel linlog`) give a second, verified kernel.
- **The MCP server was judged "no, not now"** (step 44; `plan/later.md`
  "For the author: an MCP server for linlog"; `plan/handoff.md` "Open with the
  author" lists it). The sketch there is a subcommand `linlog mcp` over stdio
  with `prove`, `check`, `seq` and stateless session steps, the command's
  defaults as limits, cancellation and progress. Nothing of it is built. No
  `AGENTS.md`, no dataset, no Python package exists.

Nothing below re-proposes these.

## 2. The field

### 2.1 The Model Context Protocol today

- The current specification revision is **2026-07-28** [1]. Its changelog [2]
  against 2025-11-25 makes the protocol **stateless**: protocol-level sessions
  and the `initialize` handshake are removed, every request carries its protocol
  version and client capabilities in `_meta`, servers must implement
  `server/discover`, and "servers that need cross-call state use explicit,
  server-minted handles passed as ordinary tool arguments". Tasks moved out of
  the core into an extension (`io.modelcontextprotocol/tasks`); results carry
  `resultType`; `inputSchema` and `outputSchema` accept any JSON Schema 2020-12
  keywords and `structuredContent` any JSON value; `tools/list` results carry
  `ttlMs` and `cacheScope` and servers should list tools in a deterministic order
  "to improve LLM prompt cache hit rates". Roots, Sampling and Logging are
  deprecated; stdio servers log to stderr.
- The **Skills extension** (`io.modelcontextprotocol/skills`) reached Final as
  SEP-2640 on 2026-09-13 [3]: skills are discovered and read through MCP
  resources. The client-support matrix [4] lists it as partial for ChatGPT,
  fast-agent and MCP Inspector; the table's other cells were empty in the text
  read, so support by Claude, Cursor and VS Code could not be read (see section
  7).
- **Rust SDK** [5]: crate `rmcp` (`modelcontextprotocol/rust-sdk`, 4.1k stars,
  791 commits), releases rmcp 3.7.0 (22 Oct), 3.8.0 (26 Oct), 3.9.0 (5 Nov) [6],
  years not shown but the cadence is a release every few days. It states that it
  implements MCP 2026-07-28 and remains compatible with 2025-11-25 and earlier;
  stdio server transport behind the feature `transport-io`; macros `#[tool]`,
  `#[tool_router]`, `#[tool_handler]`; its `Cargo.toml` [7] depends on tokio,
  serde, serde_json, schemars (optional), futures, tracing and, for the HTTP
  transport, reqwest, hyper and others (optional). Licence [8]: new code is
  Apache-2.0, older contributions MIT, documentation CC-BY-4.0; both code
  licences are on `deny.toml`'s list (inference from CLAUDE.md's list). A
  stdio-only build would still pull tokio (inference); linlog's core has no
  async runtime today.
- **CLI against MCP, the evidence**: Anthropic's post on code execution with MCP
  [9] says loading every tool definition up front and passing intermediate
  results through the model wastes context, reports 150 000 to 2 000 tokens for
  its one example, and itself warns that the figure is a single illustration and
  that sandboxing is needed. The step 44 report already holds the vendor
  measurements (1 365 against 44 026 tokens, since narrowed by lazy loading).
  Nothing read today changes that balance for a tool a model can call from a shell.

### 2.2 Skills and llms.txt

- OpenAI's Codex documentation [10] says its skills "build on the open agent
  skills standard", a folder with `SKILL.md`, name and description read first and
  the file loaded when chosen, in the CLI, the IDE extension and the app. Claims
  that Gemini CLI, Cursor and Copilot read the same files come from third-party
  blogs in the same search and were not confirmed from those vendors' pages
  (unsourced as first party). Anthropic's own account (step 44, read 2026-10-24)
  is that the format became an open standard on 2025-12-18. (Inference: the
  existing `skills/linlog/` is therefore already usable by Claude Code and
  Codex without a change; whether it is discoverable from the repository is not
  verified.)
- The llms.txt adoption evidence is unchanged from step 44's report: most
  `llms.txt` files are never requested (Ahrefs, May 2026); it helps an agent
  already pointed at the site. Not re-read today.

### 2.3 How proof assistants and provers expose themselves to agents

- **Lean.** `lean-lsp-mcp` [11] (MIT, 23 stars on the fork read, project-numina
  organisation) gives agents Lean through the language server: goal inspection,
  diagnostics, running code, trying several tactics at once (`lean_multi_attempt`),
  local and external premise search; installed with `uvx` and, for Claude Code,
  one `claude mcp add` line. LeanInteract [12] (MIT, 137 stars) is a Python
  package over the Lean REPL that extracts declarations and info trees for
  dataset building. LeanDojo [13][14] (MIT; NeurIPS 2023) is a toolkit, a
  benchmark of 98 734 theorems and proofs from Lean's mathlib, programmatic
  interaction with the proof state, and the retrieval prover ReProver.
- **Rocq.** `rocq-mcp` [15] (LLM4Rocq, Apache-2.0, 50 stars, 13 tools in three
  groups: compile and verify, interactive ones that need coq-lsp's `pet`
  server, diagnostics, among them `rocq_assumptions`), built on Pytanque [16]
  (Python client of the Petanque server). A paper [17] (arXiv 2603.20405,
  March 2026, revised May) reports Claude Opus 4.6 with this MCP proving 10 of
  the 12 Putnam 2025 problems in Rocq, 141 subagents, about 1.9 billion tokens, on
  a VM without internet, with tools designed "compile-first, interactive
  fallback". (Inference: for large, open-ended proof tasks the tool is a
  stateful stepping interface plus whole-file verification; linlog's problems
  are decided by a search, so the agent's job is smaller.) A separate OCaml
  server, `boulodrome` [18], also exists (found through search only).
- **First-order provers.** SystemOnTPTP [19] is a web form for running about 90
  systems (E, Vampire, iProver, Prover9, cvc5, Z3, Leo-III among them) on a TPTP
  problem, with SZS statuses; the page documents no programmatic interface
  (the page read says so; whether a script interface exists elsewhere on the
  TPTP site was not checked). A short paper at IWIL 2026 [20] proposes the TPTP
  World as a benchmark for LLM reasoning in which success is a derivation
  verified by independent verifiers, with an interface "PAFA" for an agent to
  state claims and justify them; it leaves the proof system open and says
  sequent calculus in TPTP is still complicated (the last from a search summary,
  not read in the paper).
- **Datasets for linear logic and models.** The search found no benchmark that
  evaluates language models on linear logic or sequent provability [21]
  (search summary; a negative result from one search, not proof of absence).
  Related: Tarau and de Paiva generate theorems and proof terms of implicational
  linear logic in Prolog, offered for testing provers and as training data for
  neural networks [22]; Kogkalidis, Moortgat and Moot's Neural Proof Nets [23]
  train a Sinkhorn network to build linear proof nets from sentences on the
  AEthel dataset of 72 192 derivations. Other recent logic benchmarks for models
  (a Hilbert-system propositional task checked in Lean, PROOFGRID with proof
  writing, checking, masking and gap filling) were seen as search titles only
  [21]; PROOFGRID's page returned 403.
- The earlier scope note `plan/notes/research/scope/engines.md` (engines-7)
  proposes the same as candidate (b) below plus an environment protocol; it
  cites ML on linear logic in grammar [engines refs 20, 21], which matches [23].

## 3. Candidates

### llm-agents-1: an MCP server (`linlog mcp`)

- *What it brings and to whom.* Use of linlog from clients that have no shell
  (chat applications and IDEs that speak MCP but run no commands), typed input
  and output schemas, a read-only flag per tool, cancellation, and a place to
  put the Skills extension's `SKILL.md` as a resource. Evidence of use by
  models: the Rocq and Lean servers above are used by agents with a shell
  (Claude Code is the install example in [11] and [15]), so for them the server
  is the interface even where a command exists; for linlog the same agents read
  the guide and call the command at 7 of 7 tasks (step 44).
- *What changed since 2026-10-24 that moves the verdict.* (1) The specification
  of 2026-07-28 removed sessions and the handshake: linlog's stateless design
  (web bindings, calls that take and return a session's JSON) is now the
  protocol's own model, with "server-minted handles passed as arguments" as the
  stated way to keep state, so the sketch in `plan/later.md` needs no state
  machine, which was its main implementation cost (inference). (2) The Rust SDK
  is at 3.9.0 with a release a few days apart and implements the current
  revision, so a dependency would be maintained, not a risk of being stranded.
  (3) Skills can travel over MCP (SEP-2640 Final), which lets one server carry
  tools and the guide together. What did not change: for a model with a shell
  the command plus skill costs fewer tokens than tool schemas on every turn;
  Claude Code defers MCP tool schemas by tool search (step 44's source), which
  narrows but does not close that gap.
- *What a suite would offer in linlog's terms.* Tools `prove`, `check`, `seq`
  and `session` (start, rules, apply, undo, close) as one-shot calls whose
  `structuredContent` is the outcome JSON that exists today, the verdict line as
  text, the settings' `limits` as optional arguments with the command's defaults,
  `outputSchema` from the library's serde types through `schemars` (a new
  optional dependency; the serde feature exists). All tools read-only (the
  sketch's choice: drawings returned as SVG text). Decide, check and step are
  covered; export comes back as text (LaTeX, Typst, SVG) and as base64 for PNG
  and PDF; certificates (Rocq) return as text, not compiled. The web bindings'
  `hint` and `close_all` could be tools for models that step.
- *Cost.* About 1.5 sessions: a `mcp` module in the CLI crate behind a feature,
  tool definitions and schemas, cancellation wired to the stop flag, a test that
  runs a session over a pipe against the current revision, the guide's and
  manual's chapter (the names test in `readme.rs` must learn the tool names), a
  `server/discover` check. It adds a dependency tree (tokio, schemars, rmcp's
  macros) to the CLI, not to the library, and nothing to the trusted base: the
  server calls the same checked outcomes.
- *Risk.* The revision is four months old and the ecosystem is moving
  (deprecations of Sampling and Logging, the Tasks extension redesign): a server
  built on rmcp inherits its breaking releases (rmcp is at major version 3).
  Tool-count and token-cost: four to six tools is small. A server that spawns
  searches needs the work limit and memory cap as defaults (they exist). Few
  clients are known to support the Skills extension. Local stdio servers are
  not usable from browser-hosted chat clients, which expect remote servers
  (unsourced; the search that would have checked it was refused by the budget),
  so the stated benefit "chat apps without a shell" may need a remote HTTP
  transport, which is hosting, a different undertaking (auth, abuse limits, a
  server the author must run).
- *Prerequisites.* None in the core. The author's decision on a dependency
  (`rmcp` with `transport-io`, `macros`, `server`, `schemars`), and on whether a
  remote deployment is in scope.
- *Sources.* [1][2][3][4][5][6][7][8][9] and `plan/later.md`.
- *Verdict (inference).* Still not demanded by evidence of users; now cheap
  enough (a session and a half) that the remaining reason not to build it is the
  absence of a client that needs it. Build when someone asks, or if the author
  wants linlog listed in MCP registries. The new reason to reconsider is that
  statelessness removes the design cost.

### llm-agents-2: a dataset of linear-logic problems with verified answers

- *What it brings and to whom.* Researchers evaluating or training models on
  sequent provability, and anyone building tutoring tools. No linear-logic
  benchmark for models turned up [21], so this is a gap (inference from one
  search). Linear logic is decidable in its multiplicative-additive fragment and
  gives labelled negatives with checkable reasons (the `refutation` kinds), which
  LeanDojo's data [14] cannot give; that was engines-7's argument.
- *What it would offer.* Rows of sequent text (with the logic flag), verdict,
  reason kind, a checked proof (JSON) for the provable ones, difficulty
  measures (the counters, the work count), and a split by fragment, from linlog's
  own generated families (`core/src/families.rs`), whose sizes and verdicts are
  known by construction, and, separately, LLTP problems. Decide, prove and check
  are all already there; the dataset needs a generator command or script and a
  release pipeline. Tasks: decide, prove (the model writes the proof, `check`
  grades), explain a refutation, repair a wrong rule application.
- *Cost.* About 1 session for the generator, schema, a dataset card, and a
  Zenodo or Hugging Face release (Zenodo is already used by the project, DOI
  10.5281/zenodo.23448588); a second session if a baseline run of one or two
  models is wanted for the card. No trusted base added; labels are as trusted as
  the checker and the Rocq-checked certificates, and the card should say so.
- *Risk.* Licence: LLTP is GPL-3.0 (CLAUDE.md) and the families are linlog's
  own (EUPL-1.2); redistributing LLTP problems with labels is a derivative
  question the author decides; the safe form ships the generator and the
  families and, for LLTP, only identifiers and labels (inference, not a legal
  reading). The families are regular, so a model can learn the pattern, not the
  logic: the card must say which problems are contamination-prone. Hugging Face
  licence and hosting terms were not read (unsourced).
- *Prerequisites.* None; the harness (`bench/`) and the JSON forms exist.
- *Sources.* [14][21][22][23], `plan/notes/research/scope/engines.md` engines-7.

### llm-agents-3: a Python package

- *What it brings and to whom.* Agents whose sandbox is Python (notebooks, data
  pipelines, evaluation harnesses) and dataset users; LeanInteract [12], Pytanque
  [16] and the Rocq and Lean servers show that Python is where the proof-assistant
  agent tooling lives. A Python caller can already spawn the binary and read its
  JSON, which the guide supports.
- *What a suite would offer.* The interop-7 scope (`plan/notes/research/scope/interop.md`):
  PyO3 and maturin wheels with prove, check, the session and the exports.
  Everything the command does would be reachable without a subprocess.
- *Cost.* 2 to 3 sessions (a bindings crate, wheel builds for several
  platforms in CI, GIL release during a search for Ctrl-C, tests). It adds PyO3
  and a Python toolchain to the flake and CI, and nothing to the trusted base.
- *Risk.* Wheel builds on three platforms are the usual failure; a second API
  surface that must stay equal to the command's (the guide's names test would
  need a Python twin). For an agent in a sandbox, `pip install` of a wheel is
  easier than `cargo install`, which compiles for minutes (inference): that is
  the real benefit, and `cargo install --locked linlog-cli` is the only install
  line in the guide's front matter today. A cheaper step toward the same
  benefit is a prebuilt binary release (the release workflow produces
  installers, per `plan/handoff.md`), not verified here.
- *Prerequisites.* None in the core (interop-7's "room now": plain-data entry
  points and a `Stop` that is object-safe).
- *Sources.* [12][16], interop scope note.

### llm-agents-4: structured hints and error messages for models

- *What it brings and to whom.* Models that retry after a failure. Step 44's
  "What is left open" lists three things (flag values not checked by name, the
  web bindings described by call names only, little evidence crawlers fetch
  llms.txt), not message quality. The state today is better than that list
  suggests: unknown verdicts name the bound and the flag, refutations are tagged
  by `kind`, library errors have `code`, `kind` and `setting`, `-i` given a
  classical symbol is refused with a hint (`docs/src/status.md`). Anthropic's
  guidance on tools [step 44, "Writing tools for agents"] asks for errors that say
  what to do with a correct input.
- *What remains.* Unverified: whether a parse error carries a position and the
  expected token, and whether a refused flag combination shows a corrected
  command. A model needs these in JSON on the command (`--format json` for
  errors, not only in batch records and `check`). Worth a test with a small model on
  twenty deliberately wrong inputs before building anything (the trial method of
  step 44).
- *Cost.* 0.5 to 1 session for improvements found by the trial; the trial itself
  half a session.
- *Risk.* Message wording is a public interface the guide's tests hold; every
  change touches the manual and the guide in the same commit (CLAUDE.md).
- *Prerequisites.* None.
- *Sources.* step 44's report; `skills/linlog/reference.md` sections 5 and 8; [9].

### llm-agents-5: `--explain`, a narrated proof for students

- *What it brings and to whom.* Students and teachers who ask a model to explain
  a derivation, and models that otherwise invent the narration. The risk of that
  is the actual reason to supply it: a model narrating a proof tree from its
  picture can misstate a rule. A narration generated by the checker, one sentence
  per rule instance ("the sequent splits the context between the two premises of
  the tensor"), would be right by construction and a model would relay and
  expand it.
- *What it would offer.* Text or JSON steps beside the proof, in the vocabulary
  the web page's Learn lessons use (step 43b; the education scope note covers
  teaching output). Prove-step-by-step already exists; this adds natural-language
  rendering of rule instances and of refutations (`kind` to a paragraph).
- *Cost.* 1 session for a sentence table per rule in the supported fragments, the
  tests, and the manual; more if localised (the web page has its own copy, so a
  single source with the page would need a decision).
- *Risk.* Wording that teaches wrongly; duplication of the web page's lesson text;
  a natural-language interface wants an owner who maintains it for every new
  logic the suite gains (each custom rule, each calculus). No source was read on
  whether students benefit from narration (unsourced); the education scope note
  and `plan/notes/research/next/web-front-end.md` should decide this, not this
  note.
- *Prerequisites.* The rule set as data with names and a short description each
  (partly there in `rules` output).
- *Sources.* step 44's report; scope note `education.md` (not re-read here).

### llm-agents-6: `check` as a verifier of model-written proofs

- *What it brings and to whom.* Evaluators and tutors who want to grade a
  proof a model (or a student) wrote. The existing `check` takes the proof JSON
  that `prove --format json` writes and reports valid or invalid. The mechanism
  that proof-assistant work relies on is the same: the agent writes, the kernel
  verifies (the Putnam run [17] used a "compile-first" strategy with whole-file
  verification and an assumptions query). The IWIL paper [20] states the same
  principle for TPTP: success is a derivation that independent verifiers confirm.
- *What a suite would offer.* A friendlier input: today the model must write the
  node-array JSON (`{"⊗": [o, l, r]}` with subformula numbers), which is hard
  for a model to emit correctly; a derivation in the plain text form the command
  prints (the tree with rule names) or as a list of `apply G P RULE` lines (the
  session script, which exists) would be a natural proof format. The session
  script is already a proof language that `interact` replays and checks: `printf
  'apply ...' | linlog interact SEQUENT` ends with `valid proof`; the guide
  teaches it. What is missing is a documented one-call grading mode: the script in,
  verdict and the first step that failed out (with the reason: which position
  was out of range, which rule did not apply), plus a score of progress (how many
  goals remain open). Whether `interact` reports the failing line today is
  not verified here.
- *Cost.* 0.5 to 1 session: a `--check-script`-like mode or a documented use
  of `interact`'s exit status with a JSON transcript; manual and guide text.
  Adds nothing to the trusted base: the checker is the same.
- *Risk.* A grader that gives partial credit gives a gradient to cheat on;
  answers must be graded by the final checked proof only, as now. An LLM-judge
  replaced by a kernel is the point, so the cost per grading is a command call
  (milliseconds, no model tokens), against an LLM-judge's tokens; no source
  measured it, but the step 44 trials show the call is within a 2 s limit by default.
- *Prerequisites.* None.
- *Sources.* [17][20], step 44's report (trials).

## 4. Considered and rejected

- **A hosted remote MCP or HTTP API for linlog.** It makes linlog a service the
  author runs: abuse limits, availability and privacy statements. The web page
  states it has no API (its `llms.txt`), and the project's WebAssembly goal serves
  the same users in their own browser. Reconsider only with evidence of demand.
- **An LLM inside the library or the command.** The core must stay offline and
  deterministic (CLAUDE.md; also rejected in `scope/engines.md`).
- **A SystemOnTPTP-style web service.** The TPTP service page documents only a
  web form and a limited number of concurrent users [19]; the same reasoning
  as for a hosted API.
- **A separate fine-tuned model or a training run.** No evidence of need and a
  large cost; the dataset (llm-agents-2) is the cheap precursor and anyone may
  train on it.
- **A thin `AGENTS.md`** is left to the author by step 44 and is about agents
  that work on the repository, not users. No new evidence read today.
- **Programmatic guidance in more formats (a `.cursorrules`, a Copilot
  instruction file).** (Inference) The Skills format is read by Codex per its own
  docs [10] and by Claude Code; per-vendor files would duplicate a text that
  tests keep true.

## 5. What the structure must allow

Reusable for all candidates: the outcome and error JSON forms with their
`code`, `kind` and `setting`, the stateless session (the web bindings keep
nothing between calls, which fits MCP 2026-07-28 and a Python wrapper alike),
the stop flag and work count (cancellation and progress), the serde types
(schema generation), the names test that holds the guide to the command. A
server, a package and a dataset are thin layers over the same calls; they should
live in the CLI crate or beside it, behind features, so that the library keeps
no clock, no async runtime and no network. What is linear-specific is only the
vocabulary of rules and refutations; a narration or a grading mode would need
that vocabulary as data per calculus, which any new logic (step 47 onward)
must add.

## 6. Sources

All read 2026-11-08 unless noted.

1. https://modelcontextprotocol.io/specification/latest (revision 2026-07-28).
2. https://modelcontextprotocol.io/specification/2026-07-28/changelog
3. https://modelcontextprotocol.io/community/working-groups/skills-over-mcp.md
4. https://modelcontextprotocol.io/extensions/client-matrix.md
5. https://github.com/modelcontextprotocol/rust-sdk
6. https://github.com/modelcontextprotocol/rust-sdk/releases
7. https://github.com/modelcontextprotocol/rust-sdk/blob/main/crates/rmcp/Cargo.toml
8. https://github.com/modelcontextprotocol/rust-sdk/blob/main/LICENSE
9. https://www.anthropic.com/engineering/code-execution-with-mcp
10. https://developers.openai.com/codex/skills (via search result, summary read)
11. https://github.com/project-numina/lean-lsp-mcp
12. https://github.com/augustepoiroux/LeanInteract
13. https://leandojo.org/
14. https://arxiv.org/abs/2306.15626 (LeanDojo)
15. https://github.com/LLM4Rocq/rocq-mcp
16. https://github.com/LLM4Coq/pytanque (found by search; page not fetched)
17. https://arxiv.org/abs/2603.20405 (Putnam 2025 in Rocq with Rocq-MCP)
18. https://ocaml.org/p/boulodrome (search result only)
19. https://tptp.org/cgi-bin/SystemOnTPTP
20. https://easychair.org/publications/paper/PMbN (IWIL 2026, Urosevic et al.)
21. Web search "large language models linear logic proof benchmark ..." and "linear logic language model ..." (results only; includes https://arxiv.org/pdf/2602.20973.pdf and https://www.preprints.org/manuscript/202604.1702, the latter 403)
22. https://arxiv.org/abs/2009.10241 (Tarau, de Paiva; search summary)
23. https://arxiv.org/abs/2009.12702 (Neural Proof Nets; via search and `scope/engines.md`)
- Repository: `plan/reports/44-llm-guide.md`, `plan/later.md`, `plan/handoff.md`,
  `skills/linlog/SKILL.md`, `skills/linlog/reference.md`, `docs/src/status.md`,
  `plan/notes/research/scope/engines.md`, `scope/interop.md`, `bench/TOOLS.md`.

## 7. Not verified

- Which MCP clients (Claude, Cursor, VS Code, Gemini CLI) support the Skills
  extension or stdio servers: the matrix's cells were empty in the text read.
  Whether browser chat clients accept only remote servers: unsourced (the
  search budget ran out).
- rmcp's licence per crate (the LICENSE file says Apache-2.0 for new code, MIT for
  older), its release years, download counts and its tokio footprint in a
  stdio-only build; crates.io and lib.rs refused or returned no data.
- Whether Gemini CLI, Cursor and Copilot load `SKILL.md` (third-party claims only).
- Last-commit dates of lean-lsp-mcp, LeanInteract, rocq-mcp (pages showed none).
- The Hugging Face and Zenodo dataset terms; the legal status of labelled LLTP
  data under GPL-3.0.
- Whether linlog's parse errors carry positions and whether `interact`
  reports a failing line in JSON: not read in the code (reading the CLI source
  would settle it; no command was run).
- That no linear-logic benchmark for language models exists: one search, from
  summaries, with PROOFGRID not read.
- Anything on MCP behaviour in Claude Code's tool search beyond step 44's source.
- Nothing was run; the one thing that would help is a trial of a small model on
  twenty wrong inputs (llm-agents-4), under the supervisor's capped scope.
