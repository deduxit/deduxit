# Step 46: research, what to add after 0.2.0

The step's two sessions each add a section here: 46a (the research and
its synthesis, below) and 46b (the prompts of the chosen steps, after
the author's answers).

## Session 46a: the research and its synthesis

### Outcome

Twenty-five research notes under `plan/notes/research/next/`, one per
area, each written from sources read on 2026-11-08 with a link on every
claim about another tool, paper or service, and briefed with what linlog
has today so that nothing proposed is built already: the eight logics
and methods the author named on 2026-11-08 (ordinary sequents and
substructural variants, natural deduction, resolution, tableaux, modal
logics, cyclic proofs, the typed lambda calculi, custom rules), what
else is worth adding, the structure of a collection of suites, the
unified web front end, the name and the organisation; the prompt's own
areas (correctness verification and its feasibility, what users say,
what AI models and agents need, maintenance, the web page as students
and teachers use it); and the eight scope notes of 2026-10 brought up
to date with what steps 38 to 45 built. This report ranks every
candidate (section "The ranked list"), answers the author's direction
point by point (the structure with the options compared, what a rename
touches with name candidates checked against public registries, an
order in which each step leaves a released, useful tool), records the
fresh-context reviewer's round with an answer to each finding, and ends
with the decisions for the author, each with a recommendation, and what
46b must know. No code was written, nothing was run beyond reading,
nothing was pushed, registered or renamed.

The short answer to the author's question. Yes, linlog can grow into a
collection of suites, and the research says how, but the collection
should grow one suite at a time from the seam linlog already has: the
`ordinary/` module, which decides ordinary logic through the embeddings
and reads the proof back as LK or LJ, is already a second suite built
beside the linear core, and its web tab is the one greyed button on the
page. The recommended first year makes that tab real with a native
intuitionistic engine and Kripke countermodels, then adds the generic
layer every further method needs (a derivation type over rules given as
data, with a checker and an interactive builder), and natural deduction
in Fitch style, tableaux with countermodels, modal logics with Kripke
countermodels, lambda-terms read off proofs, and resolution at teaching
scale, each a release of its own. Cyclic proofs and System F stay
research items: provability is undecidable in every variant the notes
found, and only checking a given proof is decidable. The structure is
one repository and one workspace with `linlog` kept as the name and
crate of the linear suite, a small kernel crate extracted only once a
second suite has copied from the first, and the web page's one module
until a suite from another repository arrives. The name should change
by creating an umbrella organisation and transferring the repositories,
keeping `linlog-prover` as a held stub, before the opam submission is
sent.

### How the research was made

- Read first: the brief, `plan/46-research-next.md` whole,
  `plan/conduct.md`, `plan/17-assessment.md` and its report (sections
  6, 7 and 9), `plan/later.md` (the candidate table and the open
  sections), the eight scope notes' structure and the scope synthesis
  `plan/notes/research/scope-extensions.md`, README, `docs/src/status.md`,
  the reports of steps 38 to 45 (their outcomes and what each left
  open), `bench/TOOLS.md`, `plan/notes/research/comparison-gaps.md`,
  `plan/handoff.md`'s "Open with the author", the Status log of
  `plan/README.md` from 2026-10-30, `plan/web/design.md`'s decisions,
  `.claude/rules/core.md` and `core-ordinary.md`, and the web client's
  `CLAUDE.md`, `src/islands/` and `src/content/`.
- Twenty-five research sub-agents (Sonnet 5.5 at `medium`), one per
  area, each with a shared brief (what linlog has today, the sources
  rule, the note's form, the machine rules: no build, no run, no
  `api.github.com` but for the users agent's five calls, no `gh`) and
  an area brief naming the questions the prompt fixes. Each wrote its
  note into the workspace and reported what it could not source; those
  lists are each note's section "Not verified".
- The synthesis (this session, Fable 5.1 at `high`) read every note's
  candidates, costs, risks and prerequisites, checked the claims that
  carry the recommendations against their sources where they could be
  read again in a call (GitHub's pages on renaming an organisation and
  transferring a repository, the 2026 undecidability result for BI, the
  client's greyed "Ordinary" button, that `core/src/ordinary/` has no
  interactive proving), and ranked.
- A fresh-context reviewer (Opus 5.5 at `high`), one round, on a sample
  of each note: do the claims about other tools say what the cited
  sources say, is anything proposed as new built already, does the
  ranking follow from the notes. Its findings and the answers are in
  the section of that name.

### The notes

| note | area | candidates |
|---|---|---|
| `ordinary-sequents.md` | LK, LJ, minimal, G3, Dyckhoff's LJT, substructural variants, countermodels, the tools | ordinary-sequents-1 to 6 |
| `natural-deduction.md` | Gentzen and Fitch styles, linear natural deduction, translation from sequent proofs, the teaching tools | natural-deduction-1 to 5 |
| `resolution.md` | clausal methods, DPLL and CDCL, DRAT and LRAT, TPTP's `cnf` and SZS, Rust SAT crates | resolution-1 to 4 |
| `tableaux.md` | analytic tableaux, free-variable and prefixed tableaux, countermodels from open branches | tableaux-1 to 5 |
| `modal-logics.md` | the modal cube, labelled and nested calculi, Kripke countermodels, the provers, QMLTP and TPTP's `$modal` | modal-logics-1 to 8 |
| `cyclic-proofs.md` | cyclic pre-proofs and the trace condition, CLKIDω, μMALL, Cyclist, certificates | cyclic-proofs-1 to 5 |
| `lambda-calculi.md` | STLC, System F, inhabitation, inference, normalisation, Curry–Howard read-back | lambda-calculi-1 to 7 |
| `custom-rules.md` | rule languages and frameworks (LF, Pure, Sequoia, MetTeL2, Jape, Carnap), what is checkable, searchable, steppable | custom-rules-1 to 6 |
| `more-logics.md` | first-order classical logic with equality, QBF, description logics, Hoare logic, many-valued, temporal, BI, dependent types | more-logics-1 to 14 |
| `structure.md` | one workspace or many repositories, the kernel's boundary, what in core is generic | structure-1 to 5 |
| `web-front-end.md` | the client as built, a suite registry, one module or one per suite, a written protocol, views per method | web-front-end-1 to 7 |
| `name-and-organisation.md` | what a rename touches at GitHub, crates.io, Zenodo, opam, Cachix, Nix; eleven name candidates checked | name-and-organisation-1 to 4 |
| `verification.md` | the trusted base, Prusti, Creusot, Verus, Kani, Aeneas, hax, extraction from Rocq, more certificates | verification-1 to … |
| `users.md` | the public counters two weeks after 0.2.0 | users-1 to 3 |
| `llm-agents.md` | MCP today, datasets, bindings, what the guide lacks | llm-agents-1 to … |
| `maintenance.md` | MSRV and cadence, wasm baseline, actions, accessibility audit, signing, channels | maintenance-1 to 9 |
| `web-teaching.md` | teachers' and students' needs, LTI, grading, progress, the evidence of use | web-teaching-1 to … |
| `scope-logics.md`, `scope-proof-theory.md`, `scope-engines.md`, `scope-interop.md`, `scope-applications.md`, `scope-education.md`, `scope-semantics-decision.md`, `scope-types-higher.md` | the eight scope notes of 2026-10, each candidate's state after steps 38 to 45, its sources re-read, new work of 2025 and 2026 | the old numbering continued |

A session is one model's working day of reviewed, tested code, as the
plan counts them; every cost below is an estimate marked as such in its
note.

### The author's direction, answered

#### Each logic and method the author named

The table says, for each, what linlog has today, what the field's
decision problem is, what a suite would offer in linlog's terms, and
the note's cost. The verdict column is this session's (inference from
the notes), and the ranked list below places each candidate.

| logic or method | linlog today | the decision problem | a suite would offer | cost (sessions) | verdict |
|---|---|---|---|---|---|
| ordinary classical, intuitionistic, minimal sequents | decided through the embeddings, the proof read back as LK or LJ with a checker, a TPTP `fof` reader, a certificate over `Prop`; no native engine, no Kripke countermodel for intuitionistic or minimal logic (a classical "not valid" gets a falsifying assignment and a lemma over `Prop`), no interactive proving, the web tab greyed (`core/src/ordinary/`, `TopBand.tsx`) | classical coNP-complete; intuitionistic PSPACE-complete (Statman 1979); first order undecidable | a native LJT (Dyckhoff's G4ip) engine that always terminates (today 108 of ILTP's 274 problems at 2 s, step 25's measurement), Kripke countermodels as checked certificates with a Rocq lemma, interactive LK and LJ with the web tab made real, a structural-rule dial for the substructural corners | 2 + 2 (+1 Rocq) + 2 | **first**: the second suite already half exists |
| substructural variants | linear, affine, Mix, cyclic MLL and the Lambek calculus natively | R undecidable (Urquhart 1984); LR decidable but not primitive recursive (Urquhart 1999); BI undecidable (Galatos, Jipsen, Knudstorp, Ramanayake, LICS 2026, arXiv 2603.01595, read here) | relevant logic as a bounded mode of the ordinary suite (contraction without weakening); BI only as fragments | 1 to 2 | a mode, not a suite; BI rejected |
| natural deduction | nothing; sequent derivations one- and two-sided, interactive proving of sequents | checking linear; search is the logic's (via sequent calculus) | a checker and step-by-step builder for Gentzen trees and Fitch proofs with discharge at a distance, ordinary logic first; translation from LK and LJ proofs; linear natural deduction (Benton, Bierman, de Paiva, Hyland) after | 4 to 6, then 3 to 4, then 2 to 3 | **second method**: the widest teaching audience, a new data model |
| resolution | nothing clausal; `read_tptp` reads propositional `fof` only; the inverse method is a forward method for linear logic | propositional resolution complete, refutations may be exponential (Haken 1985); CDCL in practice; first order semi-decidable | a propositional clausal suite at teaching scale (CNF, resolution as a DAG, DPLL with bounds), first-order resolution over the existing `FoTerm`s, a certified SAT back end (DRAT or LRAT) only through a crate with an allowed licence, a TPTP `cnf` reader with SZS output | 3 to 4, 4 to 6, 2 to 3, 1 to 2 | teaching scale only; never offered as a SAT solver |
| tableaux | nothing; a closed classical tableau is an LK proof upside down | the logic's (the note cites a quasi-polynomial simulation of tree resolution by tableaux, Arai, Pitassi and Urquhart) | classical propositional tableaux as a second drawing of the LK read-back with countermodels from open branches, a checker for student trees, first-order free-variable tableaux over the term arena, prefixed modal tableaux | 2 to 3, 2, 3 to 5, 4 to 6 | cheap first step; the beginner's method with truth tables |
| modal logics | nothing; `!` and `?` are S4-like (Martini and Masini 1994) but read as exponentials | K, T and S4 PSPACE-complete, S5 satisfiability NP-complete so validity coNP (Ladner 1977, read at abstract level); GL PSPACE; common knowledge EXPTIME | a decision procedure for K to S5 with a checked Kripke countermodel drawn as SVG, step-by-step proving in Negri's labelled calculus (frame rules visible), Rocq soundness for the labelled derivations, nested sequents as the search calculus, a model editor on the page, multimodal and epistemic, first-order modal with QMLTP | 3 to 4, 3, 2, 3, 2, 1 to 3, 4+ | **third suite**: a new judgement (labels and relational atoms), well-trodden theory |
| cyclic proofs (infinite descent) | nothing (linlog's "cyclic linear logic" is the order of formulas, a different thing); proof terms are trees | provability undecidable in every variant found (CLKIDω contains arithmetic; μMALL Σ⁰₁-complete); checking a pre-proof is decidable (the trace condition, Büchi inclusion, PSPACE) | a trace-condition checker (the kernel of the area), cyclic LK with inductive definitions step by step, μMALL finitary and circular (the nearest to linlog), Cyclist-style bounded search, certificates (only an Isabelle mechanisation of the abstract condition exists) | 2 to 3, 4 to 6, 6 to 9, 4+, 4 to 8 | research item; no teaching use found; after the others |
| simply typed lambda calculus, System F | nothing; ILL proofs are linear lambda-terms by Curry–Howard, no printer; first-order binders are over terms, not propositions | STLC checking and inference decidable, inhabitation PSPACE-complete (Statman); System F checking decidable, inference undecidable (Wells 1999), inhabitation undecidable (Löb 1976) | read ILL and LJ proofs back as lambda-terms (the cheapest, highest-value item of the area), a Church-style STLC checker with typing derivations drawn as trees, inhabitation (enumerate, count, uniqueness) over the existing engines, inference by unification, a beta-reduction stepper tied to cut elimination, System F checking only, a Rocq checker for typing derivations | 1 to 2, 3 to 4, 2 to 3, 2, 2, 3, 2 | the read-back early; the checker as the fourth suite; System F as checking only |
| custom rules | fixed calculi: `Rule` is an enum, the checker one pass over occurrence terms, the inverse method keeps rules as terms with holes | checking under user rules decidable by matching; search undecidable in general; soundness is the user's | a rule language (JSON first) and a generic checker for derivations under user rules, interactive building in the command and the web stage, bounded backward search without an "unprovable" verdict, static checks of a rule set (Sequoia-style, research), synthetic rules from axioms, and a manual chapter on theories as hypotheses (the Horn engine already runs rule systems written as `!(a ⊗ b ⊸ c)`) | 2 to 3, 2 to 3, 2 to 4, 3+, 2, 1 | **the generic layer**: a second derivation type beside the fast kernel, never a rewrite of it |

Three points the notes make that change the shape of the plan:

- **The ordinary suite is the seam.** `core/src/ordinary/` already has its
  own hash-consed formula arena, its own derivation type and checker,
  and reuses the linear engines by translation; the web client's design
  already puts "Linear or Ordinary" on the tab (D57) and the button is
  greyed for want of the bindings' JSON forms (`plan/later.md`,
  "Ordinary logic's JSON forms"). So the first new suite costs no
  restructuring, and it teaches what a generic kernel must hold before
  one is written (`structure.md`, section 1; `web-front-end.md`,
  section 1).
- **Natural deduction, tableaux and labelled modal sequents share one
  need**: a derivation whose nodes are rule instances given as data,
  with contexts as sets or multisets, hypotheses discharged at a
  distance (natural deduction), branches closed by complementary
  literals (tableaux) and labels with relational atoms (modal). That is
  the same object the custom-rules note designs as "a second, generic
  derivation type and checker beside the fast occurrence-based one"
  (`custom-rules.md`, recommendation; `natural-deduction.md` section 5;
  `tableaux.md` section 5; `modal-logics.md` section 5). Building it
  once, with the first method that needs it, is the plan's largest
  saving.
- **Three things got cheaper because of what 0.2.0 built**: lambda-terms
  read off ILL and LJ proofs (binders and a derivation walk exist; 1 to
  2 sessions), first-order tableaux and resolution (the term arena and
  unification with the occurs check exist, `core/src/sequents/term.rs`,
  though both still need a first-order ordinary syntax and cost 3 to 6
  sessions each), and a
  first-order logic-programming mode (`scope-applications.md`,
  applications-5, its quantifier prerequisite met).

#### What else is worth adding

`more-logics.md` weighs fourteen further logics and methods with their
evidence of use. Its five to add first, which this session accepts with
one change of order (the first two are folded into the ordinary suite
and the tableau step, so they cost no step of their own): truth tables
and semantic tableaux as the beginner's entry (the method every first
course and every teaching tool starts with); classical first-order logic
with equality as a mode of an ordinary first-order step (the TPTP
mainstream; `read_tptp` reads propositional `fof` today); intuitionistic
first-order logic with Kripke countermodels (ILTP's first-order half);
a QBF reader (QDIMACS) with a game view over the QBF-to-MALL encoding
the benchmarks already use; and Hoare logic over first-order assertions,
reached through custom rules rather than as a suite. Rejected as suites
of their own, with the reasons in the note: temporal model checking (a
different method and tool class), BI and separation logic (BI
undecidable, LICS 2026; the symbolic-heap fragment is a verification
tool's), Martin-Löf type theory (the proof assistants' own ground),
equational completion, categorical tools. Description logic ALC and the
epistemic logics fall under the modal suite as modes.

#### Custom rules

What the frameworks and rule languages do, from `custom-rules.md`: LF
and Twelf, Isabelle's Pure, λProlog, Abella, Dedukti and Lambdapi are
logical frameworks where a calculus is a signature and checking is type
checking; Sequoia specifies a sequent calculus and checks meta-properties
(weakening admissibility, identity expansion, permutability) with
semi-decision procedures; MetTeL2 and LoTREC generate tableau provers
from rule specifications and search with them; Jape, Carnap and the
Incredible Proof Machine take rules as data for teaching and step
through derivations without search. What can be done under arbitrary
user rules: checking a derivation (decidable, by first-order matching
plus multiset or sequence splitting of the context; the NP-hard part is
choosing the principal formula in search, not in checking), bounded
search that answers "found" or "unknown" and never "unprovable",
stepping through with the applicable rules listed by matching; what
cannot: a certificate, since the user gives no semantics, so every
verdict reads "valid in the calculus given". Static analysis of a rule
set (invertibility, cut admissibility) is research-sized and mostly
"unknown" on realistic calculi.

What it asks of linlog, and the design this session recommends
(inference, with the note): a rule schema type (connectives with arity
and print forms, the structural discipline per side, rules as premises
and conclusion over schematic formulas and contexts with a fixed menu
of side conditions: a context restricted to one connective's formulas
as `!Γ`, freshness of an eigenvariable, later labels for modal
calculi), a derivation type whose nodes name a rule by reference rather
than by the closed `Rule` enum, a generic checker by matching, and an
interactive session over it with `rules`, `apply`, `undo`, `show`,
`save` and `load`, in the command as `linlog interact --calculus FILE`
and on the page as a tab whose calculus is a file. The fast linear
checker, the engines and the Rocq library stay as they are; the
generic layer is a second kind of derivation, flagged in every output
as valid in the given calculus. The same type is what natural deduction
(with a discharge link from the closing rule to its leaves and a scope
tree for Fitch), tableaux and labelled modal sequents need, so the
first of those methods to be built should build it. Cost: 2 to 3
sessions for the language and checker, 2 to 3 for the interactive
builder with its client work, 1 for the chapter on theories as
hypotheses, which needs no code.

#### The structure of a collection of suites

The options, compared in `structure.md` with the sources (the Cargo
book on workspaces, matklad on large workspaces, the manifests of typst,
polars, wasmtime and rust-analyzer, Carnap's cabal file, Isabelle's
object logics on Pure, Jape's rule files, MathComp's packages, the
semver rules, Vite's and wasm-bindgen's documentation):

| option | brings | costs and risks |
|---|---|---|
| structure-1: one repository, one workspace, a kernel crate plus a crate per logic family | one lock, one flake, one CI, one set of conventions, refactoring across kernel and suites in one commit; one guide and one command for the AI models; one `linlog-web` crate | build time grows with the suites unless `default-members` restrains it (typst, wasmtime do so); a kernel written before a second logic needs it is a guess; lockstep versions bump every crate |
| structure-2: a repository per suite, each a workspace, a kernel published on crates.io | independence of release; the shape the author's "incorporate the new repos" suggests | 2 to 3 sessions of infrastructure per repository (step 45a's split of the Rocq library is the measure); the web module must link every suite, so some repository is the integration point anyway; version skew, and a kernel API that cannot be retracted once published |
| structure-3: one crate, features per logic | no restructuring | the closed enums make every variant a 0.y bump for all users; `cargo hack`'s feature powerset grows combinatorially; release coupling total; a third party's suite must be merged in |
| structure-4: one engine, calculi as data (the Jape and Isabelle end) | the custom-rules layer | the largest addition to the trusted base; context handling for substructural, modal and binder rules makes the schema language large; it cannot be the only checker without rewriting the trusted core |

What in the core is generic and what is linear-specific (judged from
the rules files, `structure.md` section structure-5): generic are the
error family, `Limits`, `Progress`, `Refusal`, `Settings` and `Clock`,
the wire policy and `Within`, the formula walk, the checker's shape (one
pass, a memory bound, a typed fault) and its observers (the derivation
view, the size estimate, the text tree, interactive proving's goals and
undo), the export layouts (trees by subtree widths for LaTeX, Typst and
SVG, the options values, PNG and PDF over SVG, the fonts), the batch,
the front door's `Outcome`, `Verdict` and `Reason`, the parallel
runtime, the harness and the bindings' JSON-in, JSON-out shape; linear-
specific are negation normal form and the one-sided dual, the fragments
and modes, the occurrence forest and `OccSet` with the intuitionistic
reading as a lowering, the focused engine's dyadic zones, copy bound and
memo, the Horn engine's Petri reading, the inverse method's rules, the
nets and their criteria, the counts-based refutations and the Rocq
invariants behind them, the NanoYalla certificate, the LLTP and `.spec`
readers and the families. The export layouts and the interactive shape
are generic in their code but take the linear `Derivation` and
`Member`, so making them generic means a small trait for a derivation
tree, not new layout code.

**Recommendation (this session's, agreeing with the note):**
structure-1, with structure-5's boundary and the order "second suite
first, kernel second". One repository and one workspace; `linlog` stays
the linear suite and keeps its crate name, so that 0.2.0's users and
the two concept DOIs keep their meaning; a kernel crate holds only the
generic list, starts as an unpublished path crate at version 0.0.0 and
is published when a second suite uses it; the arena, forest and proof
term stay in `linlog` to keep the propositional speed (D17: the
engines call the kernel only for errors, limits and the stop poll, so
the inner loops do not cross the crate boundary). The layout moves to a
flat `crates/` only when a second crate beyond `linlog` exists;
`default-members` stays the command and the library. The web bindings
stay one crate behind per-suite features; a module per suite waits for
a measured size. The Rocq library `rocq-linlog` stays the linear
library in its repository; a generic checker library for the rule-schema
layer, if it is certified at all, is a new opam package. structure-2
stays open for a suite a third party writes. The first steps, in order:
measure (the wasm size, `cargo build --timings`, the ratchet and the
target set on the current commit: one session with the reports), make
the ordinary suite whole beside `linlog` the way `ordinary/` was built
and write down what it copied, extract the kernel from what the two
copied (3 to 5 sessions, `linlog` re-exporting at the old paths and
`breaks 0.2.0` as the check), then the rule-schema derivation (2 to 3).

The risk the author should weigh: a kernel extracted too early fits
one logic; one extracted too late means a third copy of the arena and
the checker shape. The note's evidence for "second suite first" is that
`ordinary/` built its own arena, derivation and checker beside the
linear ones rather than on them, and works; the second copy is the one
that shows what is common.

#### The unified web front end

What the client has today (`web-front-end.md`, section 1, read from the
client's sources): a row of open proofs in the top band, each a session
document with its own link, exports and step tree (D50); each tab's
calculus mark opens a popover with Linear or Ordinary (D57, the
Ordinary button `aria-disabled` with the tip "comes later"), then
classical or intuitionistic, Mix, affine and the two-sided presentation
(D58), behind a closed `Mode` of six linear strings; a stage that draws
whatever SVG the library returns and lays click targets over the
library's `i<n>` ids; one module worker loading one wasm module (6.9 MB
after step 42) behind a generic request table of about thirty call
names, every call JSON in and JSON, SVG or bytes out with nothing kept
between calls; Learn as a frame with eleven lessons and the rule
schemas; Practice over a store of 452 problems (the client's README still says 425) decided by the command
in a flake check; a link and autosave keyed by `mode`. Neither cyclic
MLL nor the Lambek calculus is offered on the page though the library
has both. Three of the author's four ingredients exist: the tab with
its own calculus, the generic stage, the generic protocol. Missing is
any notion of a suite (a logic family with its own syntax and rules)
above the mode and of a method (sequent calculus, natural deduction,
tableau, resolution, net, term) beside it; the stage assumes a
bottom-up tree with one active goal; the content is linear only.

The precedents read: Carnap (one page, many systems chosen by a
`system` attribute of the exercise, Fitch, sequent, truth tables and
trees, with instructor accounts), LogicProof (a 2026 web tool for
classical and constructive natural deduction and sequent calculus with
three views, chosen on a welcome screen), the Incredible Proof Machine
(logics as sessions), Jape (one engine, logics as text files), the
Tree Proof Generator, Click & coLLecT, jsCoq (Coq in the browser with
packages loaded on demand, the precedent for a module per suite loaded
lazily).

The candidates and this session's recommendation (with the note):
web-front-end-2, making the Ordinary segment real (LK and LJ on the
stage, with interactive proving added to the library since the ordinary
layer today decides and reads back but has no step-by-step state),
comes first and writes the suite registry (web-front-end-1: `Mode`
becomes suite, method and parameters; the link gains a suite field with
old links read as the linear suite) as part of that work rather than
before it, at two to three sessions. Then the written suite protocol
(web-front-end-5: `read`, `start`, `view`, `rules`, `apply`, `undo`,
`close`, `hint`, `draw`, `export_as`, `certificate`, `describe`, every
call stateless JSON with a level, and a `layout` field of `tree`,
`list`, `graph` or `text` that says which host the stage draws into),
frozen only after the second suite works. One module (web-front-end-3)
until a suite from another repository arrives, then a module per suite
loaded lazily by the tab (web-front-end-4) with the offline cache
filled per suite after first use. Views for list, graph and text proofs
(web-front-end-6: Fitch, tableau, resolution, terms) each built with
the suite that needs it, Fitch the likeliest first. Learn and Practice
keyed by suite (web-front-end-7), with the warning the note states: a
suite without lessons is a tool, not a course, and the linear store of
452 problems and the lessons are the measure of each suite's content
cost. The frame is about four sessions; each suite then adds its
library work, one client session and its content. The budgets to hold
are D35 (the critical path under about 170 KB compressed, the module
after first paint) and D49 (offline).

#### The name and the organisation

What a rename touches, from `name-and-organisation.md` with each
service's documentation read today (GitHub's pages on renaming an
organisation and transferring a repository were read again by this
session and say what the note says):

- **GitHub.** Renaming an organisation redirects web links to its
  repositories and git operations, not the old profile page, not API
  requests under the old name, and the old name "becomes available for
  someone else to claim", whose repositories then override the
  redirects. Transferring a repository redirects its web and git links
  until a repository is created at the old location, and "we don't
  redirect GitHub Pages associated with the repository". The
  organisation site `linlog-prover.github.io` is named after the
  organisation, so under a new name the site repository must be renamed
  `<new>.github.io` and every published URL changes: the manual at
  `/linlog/manual/`, the rustdoc at `/linlog/`, the library's docs at
  `/rocq-linlog/`, the logo URLs that README, the Rocq library's README
  and the rustdoc of every published version embed (docs.rs's 0.2.0
  pages are frozen with them). A custom domain on Pages makes the
  `github.io` address secondary: GitHub's page on custom domains describes
  redirects only between the apex and `www` forms of the custom domain
  and says nothing about the `github.io` address (read here), so the
  gain is that every URL published from then on names the domain, not
  the organisation (inference).
  The repository counts 164 occurrences of `linlog-prover` outside
  `plan/`, the Rocq library 25, the web client 36.
- **crates.io.** No page read offers a rename of a crate, only a transfer of
  ownership (inference from the pages that could be read); `linlog` and `linlog-cli` keep their names. Trusted
  Publishing is configured per crate for a repository owner and name,
  so a transfer needs the configuration edited by hand before the next
  release; whether crates.io matches on a numeric repository id could
  not be read (its pages are an application the fetch could not
  render). docs.rs follows the crate.
- **Zenodo.** The GitHub integration must be enabled again for the new
  owner; the records' metadata (the repository URL) can be edited; the
  DOIs 10.5281/zenodo.23448588 and 10.5281/zenodo.23899109 and every
  version DOI stay what they are; no Zenodo page read describes a
  rename, only users' anecdotes.
- **opam.** The prepared pull request's `homepage`, `dev-repo`,
  `bug-reports` and `doc` fields can be edited freely before it is
  submitted; after a merge no page read says how a released package's
  fields are changed; that a renamed package is a new package is the
  convention the note infers, not a page it read. The
  package name `rocq-linlog` need not change.
- **Cachix.** The cache `linlog-prover` can keep its name whatever the
  organisation is called; no page read says a cache can be renamed.
- **Nix.** `github:linlog-prover/linlog` in README, the manual and the
  client's flake follow GitHub's redirect for a transferred repository
  (inferred, not read); the locks are refreshed on the next update.

The candidates, with the eleven names checked on 2026-11-08 against
GitHub (`github.com/NAME`), crates.io's API, opam's package pages, PyPI
and the RDAP services of `.org` and `.dev` (the full table is
name-and-organisation-4):

| name | free everywhere checked? | note |
|---|---|---|
| `sequentium` | yes | the shortest free one; "sequents" in a made-up Latin |
| `gentzenlab` | yes | names the lineage |
| `turnstilelab` | yes | the turnstile is the sequent arrow |
| `prooflings` | yes | nothing found under it |
| `syllogos` | all but `.org` | |
| `gentzen`, `organon`, `turnstile`, `apodeixis`, `sequentia` | no: taken on GitHub as users or organisations | `turnstile` is a Racket framework (POPL 2017) |
| `linlog` | keep for the linear suite | `linlog` the GitHub user is someone else's; the crates are ours |

One web search per free name found nothing notable; that is a weak
check, not a trademark search, and a free name stays free only until
someone registers it.

**Recommendation (this session's, with the note):** not
name-and-organisation-2 (rename in place: the Pages URLs break and the
old name is free to squat at once) but name-and-organisation-3: create
the umbrella organisation under the chosen name, transfer `linlog`,
`rocq-linlog` and the site repository (renamed `<new>.github.io`), keep
`linlog-prover` as a held organisation with a stub site that redirects
the published URLs (the manual, the rustdoc, the logo) to the new ones,
and keep `linlog` as the name of the linear suite, its repository and
its crates. Do it before the opam pull request is submitted. Put a
custom domain on the new Pages site at the same time if the author
wants to own one (the free `.org` and `.dev` names are in the table),
so that no later rename touches a published URL again. By hand after
the transfer, each the author's act asked for before: the Trusted
Publishing configuration of both crates, Zenodo's integration and the
records' metadata, the `crates-io` environment and the Cachix secret
checked, the two lock files refreshed. The next release proves the
chain. Cost about 1.5 sessions of repository work plus the author's
browser tasks; no change to the trusted base. If the author prefers not
to rename, name-and-organisation-1 (keep `linlog-prover` and add the
new repositories to it) costs nothing today, and the cost of a later
rename grows with every release and citation.

#### The order: each step leaves a released, useful tool

The principle (inference from the notes, above all `structure.md`'s
"second suite first" and `web-front-end.md`'s "registry with the
second suite, protocol after it"): generalise only what two suites have
copied, release after every suite, and put the cheap, high-value items
of the linear suite (lambda-terms, certified refutations, the inverse
method's dispatch rows) into the same releases so that linear-logic
users gain at every step too. Each release below is a step or a group
of steps of the plan's kind; the session counts are the notes'
estimates summed.

| release | what it adds | why it is useful on its own | sessions |
|---|---|---|---|
| 0.3.0, ordinary logic on the page | the organisation and name settled first if the author chooses so (name-and-organisation-3, before the opam submission); the client's suite registry written with the Ordinary tab made real (web-front-end-1 and 2) over interactive LK and LJ in the library (ordinary-sequents-3) and the ordinary JSON forms; lambda-terms read off ILL and LJ proofs (lambda-calculi-1, types-higher-2); the one-sided and state-equation refutations certified (verification-2 i and ii); the inverse method's dispatch rows (engines-12); a "who uses linlog" channel (users-1) | a first logic course can use the page for LK and LJ; every linear proof shows its program; two more refutation kinds carry a Rocq lemma; measured gains on three families | 9 to 11 |
| 0.4.0, intuitionistic logic decided, with countermodels | a native engine on Dyckhoff's LJT whose search always terminates (ordinary-sequents-1), Kripke countermodels as checked certificates with their Rocq lemma and a shared countermodel output kind (ordinary-sequents-2, semantics-decision-11), classical tableaux as a view of the LK read-back with countermodels from open branches and truth tables as the beginner's mode (tableaux-1, more-logics-2), a basis certificate for non-coverability (semantics-decision-1), the maintenance cadence, wasm baseline and accessibility audit (maintenance-1, 3, 4) | the copy-bound "unknown" goes for the 35 ILTP problems a loop check settles and the search never runs without end, though hard problems may still time out (today 166 of ILTP's 274 are undecided at 2 s by step 25's measurement); every invalid ordinary sequent gets a model; the beginner's two methods exist | 9 to 11 |
| 0.5.0, custom rules and natural deduction | the generic derivation type over rules given as data, its checker and the rule language (custom-rules-1), interactive building in the command and on the page (custom-rules-2), natural deduction in Gentzen and Fitch style for ordinary logic on that type with the client's list view (natural-deduction-1, web-front-end-6), the written suite protocol and content keyed by suite (web-front-end-5, 7), the chapter on theories as hypotheses (custom-rules-6); the kernel crate extracted here if the two suites' copies say so (structure-5) | the author's custom-rules wish met; the method most courses teach first; the collection's frame written from two working suites | 13 to 16 |
| 0.6.0, modal logics | K to S5 decided with checked Kripke countermodels drawn as SVG (modal-logics-1), step-by-step proving in the labelled calculus (modal-logics-2) on the generic derivation, Rocq soundness for the labelled derivations (modal-logics-3), the model editor on the page (modal-logics-5), a tableau checker for student trees (tableaux-2), course packs with a checkable handed-in link (education-4) | a third logic family with the certificate story linlog is known for; teachers can set and check exercises | 11 to 13 |
| 0.7.0, resolution and the typed lambda calculus | a propositional clausal suite at teaching scale (resolution-1), a Church-style STLC checker with typing derivations and inhabitation over the existing engines (lambda-calculi-2, 3), a DRAT or LRAT reader if an allowed crate exists (resolution-3, reduced), the dataset of problems with verified answers for models (llm-agents-2) | the remaining methods of a first course; the Curry–Howard side as a tool | 9 to 11 |
| after, by the author's choice | classical first-order logic with equality and intuitionistic first-order logic with Kripke countermodels (more-logics-1, 6), first-order tableaux and resolution (tableaux-3, resolution-2), exponentials by signature (logics-1 with 6a and 2), explaining unprovability (proof-theory-9), the rule-system front end (applications-2 with 9), the MCP server when a client without a shell asks (llm-agents-1), Lambdapi export (interop-3), the TPTP conventions (interop-5), the install channels (maintenance-7); as research projects: μMALL and the trace-condition checker (cyclic-proofs-1, 3), System F checking (lambda-calculi-6), static analysis of rule sets (custom-rules-4), deductive verification or refinement of the Rust checker (verification-4, 5) | | |

Five releases of about 55 sessions in all; at the pace of steps 18 to
45 (twenty-eight steps between 2026-10-03 and 2026-11-03) that is
within the year the prompt asks about, with room for the baselines,
the comparisons and the author's own requests.

### The ranked list

Every candidate of the notes, ranked by what it brings to the author's
three audiences against its cost, risk and prerequisites. The rank
follows the order above; within a release the cheaper item with the
wider audience comes first. "Brings" names the audience: research (R),
teaching (T), the AI models that use linlog through its guide (M). The
costs are the notes' estimates in sessions. Each row's sources are its
note's.

| rank | candidate | brings, to whom | cost | risk | prerequisites | sources (the note; the index below gives each candidate's references in it) |
|---|---|---|---|---|---|---|
| 1 | name-and-organisation-3: the umbrella organisation, repositories transferred, `linlog-prover` kept as a stub | the collection has a home before anything else is published (R, T, M) | 1.5 + the author's browser tasks | the Pages URLs of 0.2.0's docs break unless the stub redirects; a window for squatting | the author's choice of name | name-and-organisation |
| 2 | web-front-end-2 with web-front-end-1 and ordinary-sequents-3: the Ordinary tab real, interactive LK and LJ, the suite registry | T first: a first logic course's calculi on the stage; M: a second suite through the guide | 3 to 4 | old links and stored proofs must read as the linear suite; the session state duplicates the linear one unless generic | the ordinary JSON forms (`plan/later.md`) | web-front-end, ordinary-sequents |
| 3 | lambda-calculi-1 (types-higher-2): proofs read back as lambda-terms | T: the Curry–Howard exercise; R: proof and term pairs; M: a program as an answer | 1 to 2 | the calculus for `!` and additives must be named (Abramsky or Benton et al.); no term for classical one-sided proofs | none | lambda-calculi, scope-types-higher |
| 4 | verification-2 (i, ii): the one-sided and state-equation refutations certified | R: two uncertified refutation kinds get a Rocq lemma | 2 to 3 | the invariants must be stated and proved in the library | none | verification, scope-semantics-decision |
| 5 | engines-12: dispatch rows for the inverse method on `wide-m1`, classical `chain`, affine `pigeonhole` | R, M: the default decides where a measured engine already wins | 1 | a row is earned by measurement only (D19) | a timed run the author gives | scope-engines |
| 6 | users-1: Discussions on, a "who uses linlog" request, a citation request | the author: evidence where there is none | 0.5 | an empty thread | the author's `gh` yes | users |
| 7 | ordinary-sequents-1: a native LJT engine | T, M: a verdict for every propositional intuitionistic formula | 2 | duplicates the embedding's verdicts, cross-checked by the harness | none | ordinary-sequents |
| 8 | ordinary-sequents-2 with semantics-decision-11: Kripke countermodels as certificates and a shared countermodel kind | T: why a formula is not valid; R: certified non-validity | 2 + 1 (Rocq) | model size; the Rocq lemma is new | rank 7 | ordinary-sequents, scope-semantics-decision |
| 9 | tableaux-1 with more-logics-2: classical tableaux as a view with countermodels, truth tables | T: the beginner's two methods | 2 to 3 (+1) | a view, not a search; the drawing is new | the LK read-back (exists) | tableaux, more-logics |
| 10 | semantics-decision-1: a basis certificate for non-coverability | R: certified refutations of the Horn engine's affine answers | 1 to 2 | a quadratic checker; the Rocq lemma later | none | scope-semantics-decision |
| 11 | maintenance-1, maintenance-3, maintenance-4: MSRV and update cadence, a wasm feature baseline, an accessibility audit with a screen-reader pass | every user; the page's WCAG floor kept | 2 to 3 | none | none | maintenance |
| 12 | custom-rules-1 and custom-rules-2: the rule language, the generic checker, interactive building | the author's wish; T: a course's own rules; R: a playground for a calculus | 4 to 6 | unsound rule sets pass as "valid in the calculus given"; the client half is in another repository | none in core; a second derivation type | custom-rules |
| 13 | natural-deduction-1 with web-front-end-6: Gentzen and Fitch natural deduction for ordinary logic, the list view | T: the widest audience of all methods; M: a checker for a written proof | 4 to 6 + 1 | the Fitch layout is new drawing work; rule sets vary by textbook | rank 12's derivation type | natural-deduction, web-front-end |
| 14 | web-front-end-5 and web-front-end-7: the written suite protocol, content keyed by suite | every later suite; M: one protocol | 2 | designed too early it fits only trees | two working suites | web-front-end |
| 15 | structure-5: the kernel crate extracted | the maintainer: one copy of the generic parts | 3 to 5 | propositional speed if the arena or proof term moved (they do not) | two suites' copies | structure |
| 16 | custom-rules-6: theories as hypotheses, a chapter | M, T: what the Horn engine already runs | 1 | none | none | custom-rules |
| 17 | modal-logics-1: K to S5 decided with Kripke countermodels | T, R, M: the third logic family | 3 to 4 | not a modal SAT competitor; loop checks per logic | a modal formula arena | modal-logics |
| 18 | modal-logics-2 and modal-logics-3: the labelled calculus step by step, Rocq soundness | T: frame conditions visible; R: certificates | 3 + 2 | labels and relational atoms are a new judgement | rank 12's type, rank 17 | modal-logics |
| 19 | modal-logics-5: a Kripke model editor on the page | T | 2 | duplicates a small MIT tool | rank 17 | modal-logics |
| 20 | tableaux-2: a tableau checker for student trees | T, M | 2 | format variation | rank 9 | tableaux |
| 21 | education-4 with education-8's embed (web-teaching-2, web-teaching-9): course packs and a checkable handed-in link | T: a course can set, restrict and check | 2 to 3 | no server, so no grade book (D36) | none | scope-education |
| 22 | resolution-1: a propositional clausal suite at teaching scale | T; M: a checkable refutation | 3 to 4 | scope creep toward a SAT solver | none | resolution |
| 23 | lambda-calculi-2 and lambda-calculi-3: the STLC checker, inhabitation | T: typing derivations; R: counting inhabitants | 3 to 4 + 2 to 3 | a term data model beside formulas | rank 3 | lambda-calculi |
| 24 | resolution-3: a certified SAT back end (DRAT, LRAT) | R, M | 2 to 3 | no Rust solver with an allowed licence emits DRAT (splr is MPL-2.0) | an allowed crate or a reader only | resolution |
| 25 | llm-agents-2 and llm-agents-6: a dataset with verified answers, `check` as a grader | M | 1 + 0.5 | LLTP is GPL-3.0, so the families and ILTP carry it | none | llm-agents |
| 26 | verification-6: Kani harnesses for the checker's integer and bound facts | R: the one fault class found so far, checked exhaustively within a bound | 1 to 2 | a nightly toolchain in CI | none | verification |
| 27 | engines-14 and engines-13: faster copy-bound deepening, a depth that does not grow with the formula | R, M: the "unknown" rows | 2 to 3 | measured only | a timed run | scope-engines, comparison-gaps |
| 28 | proof-theory-9: explaining unprovability (a failing slice, missing resources, the stuck leaf drawn) | T, M | 1 to 2 | a bound on the attempt kept | none | scope-proof-theory |
| 29 | interop-1 and interop-5(a): a JSON Schema of the wire forms, TPTP conventions in and out | M, R | 2 | none | none | scope-interop |
| 30 | more-logics-1 and more-logics-6: classical first-order logic with equality, intuitionistic first-order logic with Kripke countermodels | R, T: the TPTP mainstream and ILTP's first-order half | 4 to 6 | equality and undecidability; bounded answers | the ordinary suite | more-logics |
| 31 | tableaux-3, resolution-4 and resolution-2: first-order tableaux, the TPTP `cnf` and first-order `fof` reader with SZS output, first-order resolution over the term arena | R, T | 3 to 5 + 1 to 2 + 4 to 6 | a first-order ordinary syntax does not exist yet; no library to measure by but TPTP | ranks 9 and 22; resolution-2 needs resolution-4 first | tableaux, resolution |
| 32 | logics-1 with logics-6 (its exponential-only half) and logics-2: exponentials by signature (SELL, a weakening-free exponential, counting grades) | R: the live literature of 2025 | 3 to 4 | every exponential case of cut elimination and the Rocq mirror per label | none | scope-logics |
| 33 | applications-2 with applications-9: the rule-system front end (run, explore, play), first-order rule systems by grounding | T, R: Ceptre's and LLF's use | 3 + 1 to 2 | a forward-chaining mode beside the engines | rank 12 | scope-applications |
| 34 | natural-deduction-2 and natural-deduction-3: translation from LK and LJ, normalisation, linear natural deduction | R, T | 3 to 4 + 2 to 3 | Zucker's non-bijection bounds the claim | rank 13 | natural-deduction |
| 35 | lambda-calculi-4, lambda-calculi-5, lambda-calculi-7: inference, a reduction stepper tied to cut elimination, a Rocq checker for typing derivations | T, R | 2 + 2 + 2 | crowded by small tools | rank 23 | lambda-calculi |
| 36 | modal-logics-4, modal-logics-7, modal-logics-8: nested sequents, multimodal and epistemic, first-order modal with QMLTP | R | 3 + 1 to 3 + 4 | the first-order case undecidable; common knowledge EXPTIME | rank 17 | modal-logics |
| 37 | llm-agents-1: an MCP server | M: clients without a shell | 1.5 | `rmcp` is at major version 3 with breaking releases; chat clients without a shell may need a remote HTTP server, so hosting; the specification itself is stateless since revision 2026-07-28 | a client that asks | llm-agents |
| 38 | interop-3: a Lambdapi theory with its Lean export | R | 2 to 3 | no linear logic encoding found there yet | none | scope-interop |
| 39 | maintenance-7, maintenance-5, maintenance-8: install channels in order (winget, a Homebrew tap, Scoop, Chocolatey), cosign, a glibc binary | users | 1 each | accounts the author must hold | none | maintenance |
| 40 | custom-rules-3: bounded search under user rules | R | 2 to 4 | never "unprovable" | rank 12 | custom-rules |
| 41 | semantics-decision-2, semantics-decision-3: Presburger invariants, Karp–Miller | R | 2 + 2 | measured only | none | scope-semantics-decision |
| 42 | proof-theory-1, proof-theory-12, proof-theory-13: proof equivalence, interpolants by cut introduction, MALL isomorphisms | R | 2 to 3 each | research | none | scope-proof-theory |
| 43 | verification-1 and verification-3: a second checker fuzzed against the first, a checker extracted from Rocq | R | 1; 2 + 2 | the extracted checker removes nothing from the Rust base | none | verification |
| 44 | cyclic-proofs-1 and cyclic-proofs-3: the trace-condition checker, μMALL | R: the nearest extension of linlog's own logic | 2 to 3 + 6 to 9 | provability undecidable; no tool to compare with; a graph proof type | binders over formulas | cyclic-proofs, scope-logics |
| 45 | lambda-calculi-6: System F, checking only | R, T | 3 | inference and inhabitation undecidable | rank 23 | lambda-calculi |
| 46 | custom-rules-4 and custom-rules-5: static analysis of rule sets, synthetic rules from axioms | R | 3+ and 2 | mostly "unknown" | rank 12 | custom-rules |
| 47 | verification-4 and verification-5: deductive verification of the checker, refinement to the Rocq checker | R: the strongest statements | 4 to 8; more | Prusti does not fit (closures unsupported, no tagged release since 2024-02); Verus or Creusot after a spike; Aeneas's and hax's Rocq back ends are the least mature | a refactored checker core | verification |
| 48 | logics-4, logics-5 beyond μMALL, cyclic-proofs-2, cyclic-proofs-4, cyclic-proofs-5: BV and pomset nets, cyclic LK with inductive definitions, Cyclist-style search, certificates for the trace condition | R | 4 to 8 each | research projects | none now (research projects) | scope-logics, cyclic-proofs |
| dropped | BI and separation logic as a suite (BI undecidable, LICS 2026), temporal model checking, Martin-Löf type theory, equational completion, categorical tools, a hosted service with accounts, a SAT solver as a goal, natural deduction for classical linear logic (no standard calculus found), nixpkgs for now (the author's word) | nothing: each is rejected for the reason named | 0 | none | none | more-logics, custom-rules, resolution, natural-deduction |

The remaining candidates of the notes, each placed after the list above
unless a row says otherwise; the costs are the notes' lines:

| candidate | brings, to whom | cost | risk or reason for its place | prerequisites | sources (the note; the index below gives each candidate's references in it) |
|---|---|---|---|---|---|
| web-front-end-3 and web-front-end-4: one module, or a module per suite loaded lazily | every suite's users; decided when a suite from another repository arrives | 0 now; 1 + 1 then | duplicated code per module, a heavier offline cache | web-front-end-5 | web-front-end |
| web-teaching-1, web-teaching-6, web-teaching-9: a learner's local record exported as a file, its views, a teacher's reading of it | T | 1 + 1 + little code | a self-reported file | none | web-teaching |
| web-teaching-3 (education-1): a provability light behind the Learn toggle | T | 1 | deliberately on request only today (D11) | none | web-teaching, scope-education |
| web-teaching-4 (education-7): a screen-reader path with a structural twin of the drawing | T, every user | 1 to 2 plus a review by a screen-reader user | none found to review | none | web-teaching, scope-education |
| web-teaching-5 (education-10): Learn and Practice keyed by suite, a lesson format as data | T | 1 for the mechanism, then content per suite | content is the largest cost | the registry | web-teaching, scope-education |
| web-teaching-7, web-teaching-8, web-teaching-10: a print stylesheet, a classroom setting, localisation | T | 1, under 1, 2 or more | English only today | none | web-teaching |
| education-2: proof-net exercises, a linking drawn and checked | T (linear-specific, the one thing no tool found offers) | 2 | bindings for `link` and `unlink` | none | scope-education |
| education-3 (auto-reverse), education-5 (generated exercises in the library), education-9 (resource puzzles over the Horn engine), education-11 (a learner's record) | T | under 1, 2, 2, 1 | auto-reverse needs the library's invertibility per rule (exists); generated problems need a measure of difficulty people confirm; puzzles need a Petri-net drawing | the Learn frame (built) | scope-education |
| education-6: interactive LK and LJ on the page | T | 2 to 3 | the same item as ordinary-sequents-3 and web-front-end-2 above | the ordinary JSON forms and a native goal state (ordinary-sequents-3) | scope-education |
| ordinary-sequents-4: the read-back on the proof DAG so large classical proofs check | R: seven classical proofs of 8 to 32 million inferences | 1 | checker work | none | ordinary-sequents |
| ordinary-sequents-5: a classical G3c engine or SAT hand-off | R | 2 to 3 | "not a SAT solver" | none | ordinary-sequents |
| ordinary-sequents-6: a structural-rule dial (relevant, BCK, ordered) for the ordinary calculi | T | 1 to 2 | R undecidable, so bounded | logics-1 | ordinary-sequents |
| tableaux-4: prefixed modal tableaux | T | 4 to 6 | the labelled calculus (modal-logics-2) covers the same ground | modal-logics-1 | tableaux |
| tableaux-5: a tableau or connection prover for linear logic beside linTAP | R | 3 or more | rejected by its note: the focused engines are stronger | a prefix unifier | tableaux |
| resolution-4: a TPTP `cnf` reader with SZS output | R, M | 1 to 2 (+1 for TSTP) | none | resolution-1 | resolution |
| more-logics-3: a QBF reader (QDIMACS) with a game view over the QBF-to-MALL encoding | R, T | 1 (+1 for certificates) | the encoding exists | none | more-logics |
| more-logics-4: description logic ALC | T | 3 alone, about 1 over a generic modal K | three sessions alone against one over a generic modal K: build after the modal suite or not at all | modal-logics-1 | more-logics |
| more-logics-5: Hoare logic over first-order assertions | T | 3 | through custom rules | custom-rules-1 | more-logics |
| more-logics-7: a RUP or DRAT checker and a toy CDCL | T, M | 2 | the same ground as resolution-3 | resolution-1 | more-logics |
| more-logics-8: equational logic and term rewriting | T | 3 to 4 | not for this project (its note) | a term rewriting data model (none) | more-logics |
| more-logics-9: temporal logics | R | 3 (LTL), double for CTL | a model checker's method; rejected as a suite | none | more-logics |
| more-logics-10: many-valued and paraconsistent logics as tables and tableaux | T (philosophy courses) | 1 to 2 | a mode of the truth-table and tableau step | more-logics-2 | more-logics |
| more-logics-11: epistemic, dynamic epistemic, deontic, conditional logics | T | 2 | modes of the modal suite | modal-logics-1 | more-logics |
| more-logics-12, more-logics-13, more-logics-14: BI and separation logic, dependent type theory, categorical tools | none for this project (the notes reject each) | 3, large, none | rejected (BI undecidable; the proof assistants' ground; no proof suite fits) | none | more-logics |
| modal-logics-6: S4 and IS4 into linear logic through the MELL engines, as `ordinary/` does for LK and LJ | R: a cross-check, pedagogical | 2 (+1 for IS4) | the image lies in full LL, decidability of the image class unread | modal-logics-1's syntax | modal-logics |
| natural-deduction-4: rule files for a course's own natural deduction rules | T | 3 to 5 | the same item as custom-rules-1 applied to natural deduction | custom-rules-1 | natural-deduction |
| natural-deduction-5: Fitch-style modal natural deduction | T, R | 2 | only with a modal suite | natural-deduction-1, modal-logics-1 | natural-deduction |
| applications-1: plans and their certificates from the Horn engine | R | 1 to 2 | none | none | scope-applications |
| applications-3: enumerate and count proofs | R, T | medium to high | the memo assumes one answer; see lambda-calculi-3 | lambda-calculi-3's enumeration front door | scope-applications |
| applications-4: proofs as programs and processes (term assignment per target) | T, R | 2 per target | the same ground as lambda-calculi-1 and types-higher-3 | lambda-calculi-1 (the read-back) | scope-applications |
| applications-5: a linear logic programming mode | R, T, M | 2 to 3 | quantifiers exist now | none | scope-applications |
| applications-6: reaction networks and conservation laws (SBML and PNML readers) | R | 2 | the SBML subset must be chosen; multiplicities on literals are new | none | scope-applications |
| applications-7, applications-8, applications-10: credentials, bounded exponentials, workflow specifications | M (documentation) | 0.5 each | folded or demoted by the note | none | scope-applications |
| engines-1 with engines-4: a SAT-based intuitionistic engine, encodings as exports | R | 2 + under 1 | a dependency with an allowed licence; the pure-Rust choice changed | none | scope-engines |
| engines-2, engines-3: lazy SAT over linkings, integer and trap refutations through a solver | R | unchanged, research | measured small gains | a solver crate with an allowed licence, or an exact simplex (exists for the state equation) | scope-engines |
| engines-5, engines-6: learned dispatch and schedules, learned ordering | R, M | unchanged | more data now (four baselines) | the baselines' rows (exist) | scope-engines |
| engines-7 with llm-agents-2: a proof-search environment and dataset for machine learning | M | low | the MCP server is the same work seen by a different consumer | none | scope-engines, llm-agents |
| engines-8: live provability feedback while proving by hand | T | small | partly built (hints, close within a budget); the light is D11's choice | none | scope-engines |
| engines-9: search state kept across calls | R | medium | a saturated set is a reusable state | the inverse method's saturated set (built) | scope-engines |
| engines-10, engines-11: competition inputs for the Horn engine, linear logic in the TPTP World | R | 2 each | TPTP's authors name linear logic a candidate | interop-5 | scope-engines, scope-interop |
| engines-15, engines-16: the inverse method at first order, a first-order comparison library | R | 3 or more, 2 | saturation does not terminate at first order; no linear first-order library exists | the inverse method and first-order terms (built) | scope-engines |
| engines-17: a generic focused engine over rule schemas | R | 4 or more | adds the schema reader to the trusted base; the engine-side counterpart of custom-rules-3 | custom-rules-1 | scope-engines |
| interop-2: Lean 4 certificates over cslib | R | 3 to 4 | cslib has no release | none | scope-interop |
| interop-4: focused certificates with holes | R | 2 | certificates proportional to the choices; nothing built since the 2026-10 note | none | scope-interop |
| interop-6 with interop-12: tactics that call linlog from Rocq or Lean, a `linlog serve` mode | R, M | 2 + 1 | a tactic that shells out is only as portable as the command's installation; whether Rocq's Ltac2 can run a process was not read | none | scope-interop |
| interop-7: a Python package (PyO3, maturin) | M, T | 2 to 3 | not recommended yet by the agents note | none | scope-interop, llm-agents |
| interop-8: a WebAssembly component with WIT | R | 2 | WASI 0.3 released; the objection weakened | none | scope-interop |
| interop-9, interop-10: a language server and tree-sitter grammar, notebooks | T | 2, 1 | for course repositories of problem files | none | scope-interop |
| interop-11: native reading of TPTP's non-classical syntax for the modal suite | R | 1 to 2 | the NXF syntax details come from a search summary only | interop-5, modal-logics-1 | scope-interop |
| logics-3: ordered cut, then Lambek with modalities | R, T | the old estimate | the ordered mode refuses cut today | logics-1 for the modalities | scope-logics |
| logics-6 (LR proper), logics-7 (light logics as a stratification checker), logics-8 (adjoint logic) | R | 1 over a `Mode` flag; small; 1 to 2 after logics-1 | no tool and no audience seen for LR | logics-1 | scope-logics |
| logics-9: first-order and net-side follow-ups of 2025 and 2026 (nets for first-order MALL, Yeo's theorem for coloured graphs) | R | per item | each item a paper read at abstract level | first-order nets (proof-theory-3), MALL nets (proof-theory-2) | scope-logics |
| proof-theory-2: MALL nets as conflict nets or linking sets | R: a drawing of the one core fragment with none; the input of proof-theory-13 | 1 | linear-specific; `plan/later.md` dropped MALL nets "only if a use case appears"; conflict nets not re-read | boxes and cut links (built) | scope-proof-theory, sources S5, S6, S7, N1, N5 |
| proof-theory-3: unification nets for first-order MLL | R: a canonical first-order net that no witness annotates | 1 (a first-order net data model, nets are propositional today) | the witness slot of `Node` must allow an implicit witness, to be checked in the code | first-order proofs with witnesses and eigenvariables (built) | scope-proof-theory, sources S8, S9 |
| proof-theory-4: subformula linking in the interactive session | T: fewer clicks on wide sequents, "why these meet"; generalises to every sequent suite | small to 1 | the chain is not unique under `⊗` | none | scope-proof-theory, sources S24, S25, N13, N16 |
| proof-theory-5: a calculus-of-structures view, and BV | R | 1 for an export; 2 or more and a new data model for BV | research grade; no cut-free sequent calculus for BV | none for the export | scope-proof-theory, sources S14 to S23, N12 |
| proof-theory-6: proof compression and minimal proofs (redundancy removal, cut introduction) | R, T | small for (i), 1 for (iii) | (ii) has no default notion of minimal | cut elimination (built); proof-theory-12 gives (iii) its construction | scope-proof-theory, sources S26 to S29, N10, N13 |
| proof-theory-7: a token machine on nets | T: a picture of the geometry of interaction beside the reduction animation | 1 | linear-specific; no visualiser found to compare with | MELL nets with boxes and cut links (built) | scope-proof-theory, sources S30 to S35 |
| proof-theory-8: a dialogue mode | T: Lorenzen games; a suite-level feature | 1 plus a soundness panel | the game rules per logic must be sourced | none | scope-proof-theory, sources S37 to S41 |
| proof-theory-10: combinatorial proofs for the ordinary layer (and the constructive modal logics) | T, R: one graph-over-formula drawing for classical, intuitionistic and modal logic | 1 | the modal papers were not read | the ordinary layer (built) | scope-proof-theory, sources S11, S12, N18 |
| proof-theory-11: a Curry–Howard export of linear proofs (linear lambda-terms, session processes) | T, R: the bridge to the lambda-calculi suite | 0.5 | the same item as lambda-calculi-1 for the linear half | none | scope-proof-theory, sources S47, S48 |
| proof-theory-14: a certified net criterion in the Rocq library | R: the criterion out of the trusted base | 2 to 3 in the library's repository | the sequentialization theorem is the hard part; the reference development's licence and reusability unread | the library's term checkers, a data model of nets in Rocq (new) | scope-proof-theory, sources N1, N6, N15 |
| semantics-decision-4: directed and accelerated reachability for the undecided nets | R: speed on the 66 undecided LLTP nets | 1 to 2 per heuristic | changes speed, not what is certifiable; each heuristic earned on `bench/targets.sh` | none | scope-semantics-decision, no new source beyond the 2026-10 note's |
| semantics-decision-5: a Petri-net front door (a PNML reader, reachability and coverability queries) | R: the Model Checking Contest's conversation with checkable certificates | 3 | the contest's formulas are richer than a sequent's goal; verifypn is GPL-3.0, callable not linkable | none | scope-semantics-decision, sources S20, S22, S23 |
| semantics-decision-6: affine decision by saturation with subsumption (the termination claim and its measurement) | R: a documented decision for affine fragments with exponentials | 1 | partly built in the affine inverse method; termination in every affine fragment is an inference by Dickson's lemma | the inverse method (built) | scope-semantics-decision, source S8 and the 2026-10 note's S2 |
| semantics-decision-7: finite phase countermodels | R | a prototype first | Lafont gives the theorem for MALL and the negative for MELL; no tool, no size bound; Yalla's phase semantics is ongoing work | none | scope-semantics-decision, sources S8, S9 |
| semantics-decision-8: Petri nets with data (first-order Horn with fresh names) | R: protocol models | 4 to 6 | the WSTS algorithm and the data ordering are separate work; the focused engine, not the Horn row, decides `reach-fo` today | terms and eigenvariables (built) | scope-semantics-decision, the 2026-10 note's S45, S46 |
| semantics-decision-9: Horn shapes beyond `!`-Horn | R | medium | the gain is unknown until the LLTP rows missing the Horn row for shape are counted | none | scope-semantics-decision, no new source |
| semantics-decision-10 and semantics-decision-12: Rocq certificates for saturated refutations and for the backward search's failures | R | 5 to 8; 6 to 10 | a project of its own: focusing completeness in Rocq | semantics-decision-1 and -11 first, by the note's order | scope-semantics-decision |
| types-higher-1 (with lambda-calculi-3), types-higher-3 (sessions), types-higher-4, types-higher-5, types-higher-6, types-higher-7, types-higher-8 (a linear lambda-term checker) | R, T | 2 to 3; 3 to 4; 1; 1 or 5+; 2 to 3; a flag; small | sessions lost their blocker (cut exists) | types-higher-2 | scope-types-higher |
| users-2, users-3: a monthly reading of the public counters, one announcement | the author | fractions of a session | download counts include mirrors and scanners; one post that draws no one | none | users |
| llm-agents-3, llm-agents-4, llm-agents-5: a Python package, structured hints for models, `--explain` | M | 2 to 3, after a trial, 1 | not recommended yet | a trial with a small model for -4 | llm-agents |
| verification-7: a certificate download on the page | built already: the client's bottom band exports the finished proof's and a refuted goal's Rocq certificate (`BottomBand.tsx`, found by the reviewer) | 0 | dropped | none | verification |
| maintenance-6, maintenance-9: platform code signing, a crate and module budget; maintenance-2 is built already (`.github/dependabot.yml` of 2026-09-29 watches the actions weekly, found by the reviewer), so only advisories remain of it | users, the maintainer | accounts and fees; under 1 | accounts, fees and a certificate the author must hold; the budget is a number to keep, not code | none | maintenance |
| structure-2, structure-3, structure-4: a repository per suite, features per logic, one engine with calculi as data | the maintainer: the shape of the collection | 2 to 3 of infrastructure per repository for structure-2; none for 3; 2 to 3 on top of a kernel for 4 | compared above; structure-2 kept open for a third party's suite | structure-1's kernel for 2 and 4 | structure |
| name-and-organisation-1, name-and-organisation-2, name-and-organisation-4: keep the organisation, rename in place, the name table | the author: the organisation's home and name | 0 today for 1; 1.5 plus the author's browser tasks for 2; 0 for the table | compared above | the author's choice | name-and-organisation |




The sources of each candidate: the note that holds it, and the links its
section cites, resolved from the note's source list (for a scope update,
from the 2026-10 note's list where the label is the old one, or from that
note's own section for the candidate where the update only states its
status) or taken from the section's text; at most six here, the note has
the rest. "From the note's field section" marks a candidate whose section
points at the note's survey rather than citing; a candidate that rests on
the repository's own files or on another note alone says so.

| candidate | note | links |
|---|---|---|
| custom-rules-1 | `custom-rules.md` | <https://www.joachim-breitner.de/publications/Incredible_LFMTP_2016-06-23.pdf>, <https://github.com/RBornat/jape>, <https://www.bohrium.com/paper-details/complexity-of-matching-problems/811926861788479489-885>, <https://uio.no/studier/emner/matnat/ifi/nedlagte-emner/INF5160/v10/undervisningsmateriale/kapur.narendran87Matching_unification_and_complexity.pdf>, <https://twelf.org/wiki/template-bibtex-hhp93lf/>, <https://arxiv.org/abs/2607.07722> |
| custom-rules-2 | `custom-rules.md` | <https://www.joachim-breitner.de/publications/Incredible_LFMTP_2016-06-23.pdf>, <https://github.com/meta-logic/sequoia>, <https://github.com/RBornat/jape>, <https://arxiv.org/abs/2607.07722> |
| custom-rules-3 | `custom-rules.md` | <https://www.cs.man.ac.uk/~schmidt/publications/TishkovskySchmidtKhodadadi12b.html>, <https://www.irit.fr/~Andreas.Herzig/Cours/CTableaux>, <https://www.bohrium.com/paper-details/complexity-of-matching-problems/811926861788479489-885>, <https://uio.no/studier/emner/matnat/ifi/nedlagte-emner/INF5160/v10/undervisningsmateriale/kapur.narendran87Matching_unification_and_complexity.pdf>, <https://arxiv.org/abs/2101.03113> |
| custom-rules-4 | `custom-rules.md` | <https://arxiv.org/abs/2101.03113>, <https://github.com/meta-logic/sequoia>, <https://link.springer.com/10.1007/978-3-030-51054-1_32> |
| custom-rules-5 | `custom-rules.md` | <https://figshare.le.ac.uk/articles/journal_contribution/From_axioms_to_synthetic_inference_rules_via_focusing/21708164>, <https://www.lix.polytechnique.fr/~dale/papers/synthetic-rules-via-focusing.pdf> |
| custom-rules-6 | `custom-rules.md` | <https://lics.siglog.org/archive/1996/CervesatoPfenning-ALinearLogicalFrame.html>, <https://twelf.org/>, <https://twelf.org/wiki/template-bibtex-hhp93lf/>, <https://isabelle.in.tum.de/>, <https://arxiv.org/pdf/1507.08719>, <https://github.com/meta-logic/sequoia>, and 26 more in the note |
| cyclic-proofs-1 | `cyclic-proofs.md` | <https://era.ed.ac.uk/items/f3318657-8d5e-42e4-810b-9eba4235a450/full>, <https://arxiv.org/abs/2109.03235>, <https://ar5iv.labs.arxiv.org/html/2109.03235>, <https://arxiv.org/abs/1807.10248>, <https://www.irif.fr/~doumane/these.pdf>, <https://arxiv.org/abs/2005.08257>, and 3 more in the note |
| cyclic-proofs-2 | `cyclic-proofs.md` | <https://www.research.ed.ac.uk/en/publications/sequent-calculi-for-induction-and-infinite-descent/>, <https://era.ed.ac.uk/items/f3318657-8d5e-42e4-810b-9eba4235a450/full>, <https://github.com/cyclist-org/cyclist>, <https://arxiv.org/abs/2109.03235>, <https://ar5iv.labs.arxiv.org/html/2109.03235>, <https://arxiv.org/abs/1712.03502>, and 2 more in the note |
| cyclic-proofs-3 | `cyclic-proofs.md` | <https://drops.dagstuhl.de/opus/volltexte/2016/6582>, <https://www.irif.fr/~doumane/these.pdf>, <https://arxiv.org/abs/2005.08257>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2018.35>, <https://hal-univ-tlse3.archives-ouvertes.fr/INRIA/hal-02173207>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2022.20>, and 1 more in the note |
| cyclic-proofs-4 | `cyclic-proofs.md` | <https://github.com/cyclist-org/cyclist>, <https://link.springer.com/doi/10.1007/978-3-642-35182-2_25>, <https://discovery-pp.ucl.ac.uk/10140071/1/cyclist_APLAS-new.pdf> |
| cyclic-proofs-5 | `cyclic-proofs.md` | <https://arxiv.org/abs/1712.03502>, <https://lics.siglog.org/2017/BerardiTatsuta-Equivalenceofinduct.html>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2018.35>, <https://devel.isa-afp.org/entries/Infinite_Descent_Criteria.html>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITP.2026.14>, <https://www.research.ed.ac.uk/en/publications/sequent-calculi-for-induction-and-infinite-descent/>, and 24 more in the note |
| lambda-calculi-1 | `lambda-calculi.md` | <https://github.com/plfa/plfa.github.io>, <https://plfa.github.io/>, <https://flint.cs.yale.edu/cs428/notes/L17.pdf>, <https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/tlca93.pdf>, <https://www.doi.org/10.1007/BFB0037099> |
| lambda-calculi-2 | `lambda-calculi.md` | <https://pages.github.khoury.northeastern.edu/sholtzen/cs4400-spr24/lecture-notes/lec10-stlc>, <https://ilyasergey.net/CS4212/_static/lectures/PLDI-Week-10-more-typing.pdf>, <https://rick00ne.github.io/LambdaCalculus_TypeChecker>, <https://en.wikipedia.org/wiki/System_F>, <https://github.com/plfa/plfa.github.io>, <https://plfa.github.io/> |
| lambda-calculi-3 | `lambda-calculi.md` | <https://deepblue.lib.umich.edu/handle/2027.42/23534?show=full>, <https://portal.mardi4nfdi.de/wiki/Publication:1259589>, <https://www1.cuni.cz/~svejdar/papers/sv_pspin.pdf>, <https://unpaywall.org/10.1007%2F3-540-62688-3_47>, <https://cis.upenn.edu/~bcpierce/types/archives/1996/msg00387.html>, <https://core.ac.uk/download/248561704.pdf> |
| lambda-calculi-4 | `lambda-calculi.md` | <https://plfa.github.io/>, <https://github.com/hmac/lc>, <https://seas.upenn.edu/~sweirich/types/archive/1989/msg00092.html>, <https://gallium.inria.fr/~fpottier/publis/emlti-final.pdf>, <https://researchportal.hw.ac.uk/en/publications/typability-and-type-checking-in-system-f-are-equivalent-and-undec/>, <https://lics.siglog.org/1994/Wells-Typabilityandtypech.html> |
| lambda-calculi-5 | `lambda-calculi.md` | <https://www.paultaylor.eu/stable/Proofs+Types>, <https://logic.amu.edu.pl/images/5/54/Proofsandtypes.pdf>, <https://github.com/bntre/visual-lambda>, <https://github.com/thmsmlr/lambda-playground>, <https://tromp.github.io/cl/diagrams.html> |
| lambda-calculi-6 | `lambda-calculi.md` | <https://plfa.github.io/>, <https://en.wikipedia.org/wiki/System_F>, <https://researchportal.hw.ac.uk/en/publications/typability-and-type-checking-in-system-f-are-equivalent-and-undec/>, <https://lics.siglog.org/1994/Wells-Typabilityandtypech.html>, <https://unpaywall.org/10.1007%2F3-540-62688-3_47>, <https://cis.upenn.edu/~bcpierce/types/archives/1996/msg00387.html>, and 4 more in the note |
| lambda-calculi-7 | `lambda-calculi.md` | <https://github.com/plfa/plfa.github.io>, <https://github.com/DeepSpec/sf>, <https://github.com/the-little-typer/pie>, <https://researchportal.hw.ac.uk/en/publications/typability-and-type-checking-in-system-f-are-equivalent-and-undec/>, <https://lics.siglog.org/1994/Wells-Typabilityandtypech.html>, <https://unpaywall.org/10.1007%2F3-540-62688-3_47>, and 27 more in the note |
| llm-agents-1 | `llm-agents.md` | <https://github.com/project-numina/lean-lsp-mcp>, <https://github.com/LLM4Rocq/rocq-mcp>, <https://modelcontextprotocol.io/specification/latest>, <https://modelcontextprotocol.io/specification/2026-07-28/changelog>, <https://modelcontextprotocol.io/community/working-groups/skills-over-mcp.md>, <https://modelcontextprotocol.io/extensions/client-matrix.md>, and 5 more in the note |
| llm-agents-2 | `llm-agents.md` | <https://arxiv.org/pdf/2602.20973.pdf>, <https://www.preprints.org/manuscript/202604.1702>, <https://arxiv.org/abs/2306.15626>, <https://arxiv.org/abs/2009.10241>, <https://arxiv.org/abs/2009.12702> |
| llm-agents-3 | `llm-agents.md` | <https://github.com/augustepoiroux/LeanInteract>, <https://github.com/LLM4Coq/pytanque> |
| llm-agents-4 | `llm-agents.md` | <https://www.anthropic.com/engineering/code-execution-with-mcp> |
| llm-agents-5 | `llm-agents.md` | the repository's own files and other notes only |
| llm-agents-6 | `llm-agents.md` | <https://arxiv.org/abs/2603.20405>, <https://easychair.org/publications/paper/PMbN>, <https://tptp.org/cgi-bin/SystemOnTPTP>, <https://developers.openai.com/codex/skills>, <https://modelcontextprotocol.io/specification/latest>, <https://modelcontextprotocol.io/specification/2026-07-28/changelog>, and 18 more in the note |
| maintenance-1 | `maintenance.md` | <https://blog.rust-lang.org/releases/>, <https://doc.rust-lang.org/cargo/reference/rust-version.html> |
| maintenance-2 | `maintenance.md` | <https://docs.github.com/en/code-security/dependabot/working-with-dependabot/keeping-your-actions-up-to-date-with-dependabot> |
| maintenance-3 | `maintenance.md` | <https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html>, <https://caniuse.com/wasm-bulk-memory>, <https://github.com/wasm-bindgen/wasm-bindgen/releases>, <https://webassembly.org/features/> |
| maintenance-4 | `maintenance.md` | <https://www.w3.org/TR/WCAG22/>, <https://developer.mozilla.org/en-US/docs/Web/SVG/Guides/SVG_in_HTML> |
| maintenance-5 | `maintenance.md` | <https://docs.sigstore.dev/cosign/signing/overview/>, <https://developer.apple.com/support/compare-memberships/> |
| maintenance-6 | `maintenance.md` | <https://developer.apple.com/support/compare-memberships/> |
| maintenance-7 | `maintenance.md` | <https://learn.microsoft.com/en-us/windows/package-manager/package/manifest>, <https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap>, <https://github.com/ScoopInstaller/Scoop/wiki/Buckets>, <https://docs.chocolatey.org/en-us/community-repository/moderation/> |
| maintenance-8 | `maintenance.md` | the repository's own files and other notes only |
| maintenance-9 | `maintenance.md` | <https://developer.mozilla.org/en-US/docs/Web/MathML>, <https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html>, <https://github.com/rust-lang/rust/issues/43781>, <https://blog.rust-lang.org/releases/>, <https://doc.rust-lang.org/cargo/reference/rust-version.html>, <https://github.com/obi1kenobi/cargo-semver-checks>, and 14 more in the note |
| modal-logics-1 | `modal-logics.md` | <https://rkirsling.github.io/modallogic/>, <https://github.com/rkirsling/modallogic>, <https://www.lix.polytechnique.fr/~lutz/Software/Moin/MoinProver.html>, <https://hal-univ-tlse3.archives-ouvertes.fr/INRIA/hal-02457240>, <https://hal.archives-ouvertes.fr/hal-03048966>, <https://github.com/cormackikkert/CEGARBox>, and 9 more in the note |
| modal-logics-2 | `modal-logics.md` | <https://www.mv.helsinki.fi/home/negri/paml.pdf>, <https://portal.mardi4nfdi.de/entity/Q377468>, <https://www.lix.polytechnique.fr/~lutz/Software/Moin/MoinProver.html>, <https://hal-univ-tlse3.archives-ouvertes.fr/INRIA/hal-02457240>, <https://hal.archives-ouvertes.fr/hal-03048966>, <https://arxiv.org/pdf/1810.10257> |
| modal-logics-3 | `modal-logics.md` | <https://arxiv.org/pdf/2404.14919>, <https://arxiv.org/pdf/1910.01697>, <https://link.springer.com/10.1007/978-3-030-89391-0_25> |
| modal-logics-4 | `modal-logics.md` | <https://sirius.bfh.ch/~kai/Papers/2009dssml.pdf>, <https://ar5iv.arxiv.org/html/1004.1845>, <https://www.lix.polytechnique.fr/~lutz/orgs/ESSLLI22notes-5B-10-NestedSequents--Handout.pdf>, <https://www.lix.polytechnique.fr/~lutz/Software/Moin/MoinProver.html>, <https://hal-univ-tlse3.archives-ouvertes.fr/INRIA/hal-02457240>, <https://hal.archives-ouvertes.fr/hal-03048966>, and 2 more in the note |
| modal-logics-5 | `modal-logics.md` | <https://rkirsling.github.io/modallogic/>, <https://github.com/rkirsling/modallogic> |
| modal-logics-6 | `modal-logics.md` | <https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/modal-view-of-linear-logic/AA2232D491291E188ACBBE17BB8C6DC4>, <https://arxiv.org/pdf/1904.10605>, <https://core.ac.uk/display/103544924>, <https://episteme.tllm.fr/wiki/⅋> |
| modal-logics-7 | `modal-logics.md` | <https://arxiv.org/pdf/2302.04218>, <https://portal.mardi4nfdi.de/wiki/Publication:1193476>, <https://www.leancop.de/mleancop/>, <http://leancop.de/nanocop-m/>, <https://link.springer.com/chapter/10.1007/978-3-030-86059-2_14>, <https://www.irit.fr/~Andreas.Herzig/P/ijcar01.html>, and 1 more in the note |
| modal-logics-8 | `modal-logics.md` | <https://www.doi.org/10.1007/978-3-642-31365-3_35>, <https://arxiv.org/pdf/2212.09570>, <https://iltp.de/qmltp>, <https://arxiv.org/html/2508.09318v1>, <https://tptp.org/Seminars/NonClassical>, <https://tptp.org/TPTPTParty/2024/SpeakersSlides/MelanieTaprogge.pdf>, and 51 more in the note |
| more-logics-1 | `more-logics.md` | <https://www.tptp.org/>, <https://carnap.io/about>, <https://forallx.openlogicproject.org/>, <https://www.cs.us.es/~jalonso/cursos/li-06/sistemas/tableros/info.html>, <https://www.umsu.de/wo/2002/3>, <https://carnap.io> |
| more-logics-2 | `more-logics.md` | <https://carnap.io>, <https://carnap.io/about>, <https://github.com/Carnap/Carnap>, <https://forallx.openlogicproject.org/>, <https://www.cs.us.es/~jalonso/cursos/li-06/sistemas/tableros/info.html>, <https://www.umsu.de/wo/2002/3> |
| more-logics-3 | `more-logics.md` | <https://en.wikipedia.org/wiki/True_quantified_Boolean_formula>, <https://www.qbflib.org/qbfeval22.php> |
| more-logics-4 | `more-logics.md` | <https://en.wikipedia.org/wiki/Description_logic> |
| more-logics-5 | `more-logics.md` | <https://softwarefoundations.cis.upenn.edu/plf-current/Hoare.html>, <https://arxiv.org/abs/2303.05865>, <https://arxiv.org/abs/2603.01595> |
| more-logics-6 | `more-logics.md` | <https://link.springer.com/chapter/10.1007/11554554_28>, <https://www.cs.cornell.edu/info/people/kreitz/PDF/05tableaux-iltp.pdf> |
| more-logics-7 | `more-logics.md` | <https://github.com/marijnheule/drat-trim> |
| more-logics-8 | `more-logics.md` | <https://arxiv.org/pdf/1505.01338>, <https://cl-informatik.uibk.ac.at/research/software> |
| more-logics-9 | `more-logics.md` | <https://en.wikipedia.org/wiki/Linear_temporal_logic> |
| more-logics-10 | `more-logics.md` | <https://plato.stanford.edu/entries/logic-manyvalued/>, <https://plato.stanford.edu/entries/logic-paraconsistent/> |
| more-logics-11 | `more-logics.md` | <https://people.irisa.fr/Francois.Schwarzentruber/software.html>, <https://www.ijcai.org/proceedings/2018/862>, <https://github.com/jrclogic/SMCDEL>, <https://hackage.haskell.org/package/smcdel> |
| more-logics-12 | `more-logics.md` | <https://fbinfer.com/docs/checker-pulse/>, <https://arxiv.org/abs/2603.01595> |
| more-logics-13 | `more-logics.md` | <https://softwarefoundations.cis.upenn.edu/plf-current/Hoare.html> |
| more-logics-14 | `more-logics.md` | <https://en.wikipedia.org/wiki/Linear_temporal_logic>, <https://arxiv.org/abs/2603.01595>, <https://arxiv.org/pdf/1505.01338>, <https://cl-informatik.uibk.ac.at/research/software>, <https://www.tptp.org/>, <https://carnap.io>, and 20 more in the note |
| name-and-organisation-1 | `name-and-organisation.md` | the repository's own files and other notes only |
| name-and-organisation-2 | `name-and-organisation.md` | <https://linlog-prover.github.io/logo/> |
| name-and-organisation-3 | `name-and-organisation.md` | from the note's field section: <https://docs.github.com/en/organizations/managing-organization-settings/renaming-an-organization>, <https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository>, <https://docs.github.com/en/repositories/creating-and-managing-repositories/renaming-a-repository>, <https://docs.github.com/en/pages/getting-started-with-github-pages/about-github-pages> |
| name-and-organisation-4 | `name-and-organisation.md` | <https://github.com/NAME>, <https://crates.io/api/v1/crates/NAME>, <https://opam.ocaml.org/packages/NAME/>, <https://pypi.org/pypi/NAME/json>, <https://github.com/racket-tw/macrotypes>, <https://docs.github.com/en/organizations/managing-organization-settings/renaming-an-organization>, and 18 more in the note |
| natural-deduction-1 | `natural-deduction.md` | <https://arxiv.org/abs/2607.07722>, <https://en.wikipedia.org/wiki/Jape_>, <https://www.ppig.org/files/1999-PPIG-11th-aczel.pdf>, <https://core.ac.uk/download/16156617.pdf>, <https://doaj.org/article/7f94b26d134b46f7ada5c91730198ed8>, <https://plato.stanford.edu/entries/natural-deduction/>, and 2 more in the note |
| natural-deduction-2 | `natural-deduction.md` | <https://www.doi.org/10.1007/BFB0037099>, <https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-262.html>, <https://nantilus.univ-nantes.fr/vufind/Record/PPN102137935/Description>, <https://rd.springer.com/chapter/10.1007/978-94-007-7548-0_2>, <https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/tlca93.pdf>, <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/DDBBD2A74BC4F0D5F9DDB1326DCD70E9/S0956796800001246a.pdf/strong-normalisation-for-the-linear-term-calculus.pdf>, and 3 more in the note |
| natural-deduction-3 | `natural-deduction.md` | <https://www.doi.org/10.1007/BFB0037099>, <https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-262.html>, <https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/tlca93.pdf>, <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/DDBBD2A74BC4F0D5F9DDB1326DCD70E9/S0956796800001246a.pdf/strong-normalisation-for-the-linear-term-calculus.pdf>, <https://www.cys.cic.ipn.mx/ojs/index.php/CyS/article/view/4542>, <https://typeset.io/pdf/fitch-style-rules-for-many-modal-logics-15zxluq9si.pdf> |
| natural-deduction-4 | `natural-deduction.md` | <https://en.wikipedia.org/wiki/Jape_>, <https://www.ppig.org/files/1999-PPIG-11th-aczel.pdf>, <https://arxiv.org/abs/2210.11411>, <https://plato.stanford.edu/entries/natural-deduction/>, <https://projects.csail.mit.edu/jacm/References/harperhp1993:143.html> |
| natural-deduction-5 | `natural-deduction.md` | <https://plato.stanford.edu/entries/natural-deduction/>, <https://www.cys.cic.ipn.mx/ojs/index.php/CyS/article/view/4542>, <https://typeset.io/pdf/fitch-style-rules-for-many-modal-logics-15zxluq9si.pdf>, <https://github.com/nomeata/incredible>, <https://arxiv.org/abs/2607.07722>, <https://core.ac.uk/download/16156617.pdf>, and 27 more in the note |
| ordinary-sequents-1 | `ordinary-sequents.md` | <https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/contractionfree-sequent-calculi-for-intuitionistic-logic/C7ABBFC63E74878E52FF4E624029503B>, <https://rocq-prover.org/doc/v9.1/refman/proofs/automatic-tactics/logic.html>, <https://deepblue.lib.umich.edu/handle/2027.42/23534>, <https://portal.mardi4nfdi.de/wiki/Publication:1259589>, <https://www.bohrium.com/paper-details/an-o-n-log-n-space-decision-procedure-for-intuitionistic-propositional-logic/812031645379985408-2722>, <https://ipqcoq.github.io/cpp2023fereevangool-prepub.pdf>, and 2 more in the note |
| ordinary-sequents-2 | `ordinary-sequents.md` | <https://logtalk.org/apis/fcube_0.html>, <https://www.mv.helsinki.fi/home/negri/procou_lu_rev.pdf>, <https://www.mv.helsinki.fi/home/negri/negri_tableaux_2013.pdf>, <https://irinsubria.uninsubria.it/handle/11383/2178111> |
| ordinary-sequents-3 | `ordinary-sequents.md` | <https://github.com/Carnap/Carnap>, <https://uni-kassel.de/eecs/en/tifm/software/sequent-calculus-trainer>, <https://doaj.org/article/7f94b26d134b46f7ada5c91730198ed8>, <https://www.cambridge.org/core/books/basic-proof-theory/gentzen-systems/B273AB679C77FBDA4A534EC6A616A372>, <https://arxiv.org/pdf/2305.10888> |
| ordinary-sequents-4 | `ordinary-sequents.md` | the repository's own files and other notes only |
| ordinary-sequents-5 | `ordinary-sequents.md` | <https://en.wikipedia.org/wiki/Boolean_satisfiability_problem> |
| ordinary-sequents-6 | `ordinary-sequents.md` | <https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/undecidability-of-entailment-and-relevant-implication/FC9AE61D959740AB0356AEE1F5BC70E8>, <https://resolve.cambridge.org/core/journals/journal-of-symbolic-logic/article/complexity-of-decision-procedures-in-relevance-logic-ii/C2C773998D8D629630ACEA0E64203A0F>, <https://carnap.github.io/Carnap-Manual/systems.html>, <https://carnap.github.io/Carnap-Manual/sequent-calculus.html>, <https://lics.siglog.org/archive/1995/Kopylov-DecidabilityofLinea.html>, <https://arxiv.org/abs/2603.01595>, and 37 more in the note |
| resolution-1 | `resolution.md` | <https://www.aiai.ed.ac.uk/~gwickler/proposition-resolution.html>, <https://www.npmjs.com/package/@4zzz/resolution-editor>, <https://portal.mardi4nfdi.de/wiki/Item:Q1071750>, <https://arxiv.org/pdf/1912.00534>, <https://en.wikipedia.org/wiki/Resolution_>, <https://www.bohrium.com/paper-details/a-machine-oriented-logic-based-on-the-resolution-principle/813217272737824768-2464>, and 1 more in the note |
| resolution-2 | `resolution.md` | <https://github.com/eprover/eprover>, <https://github.com/sneeuwballen/zipperposition>, <https://github.com/leoprover/Leo-III>, <https://github.com/vprover/vampire>, <https://tptp.org/CASC/index.html>, <https://en.wikipedia.org/wiki/Resolution_>, and 1 more in the note |
| resolution-3 | `resolution.md` | <https://github.com/marijnheule/drat-trim>, <https://arxiv.org/pdf/1612.02353>, <https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7984575/>, <https://en.wikipedia.org/wiki/Conflict-driven_clause_learning>, <https://crates.io/api/v1/crates/rustsat>, <https://github.com/chrjabs/rustsat>, and 9 more in the note |
| resolution-4 | `resolution.md` | <https://arxiv.org/abs/2508.09318>, <https://arxiv.org/pdf/2508.09318>, <https://tptp.org/UserDocs/SZSOntology/>, <https://www.cs.miami.edu/home/geoff/Papers/Conference/2008_Sut08_KEAPPA-38-49.pdf>, <https://github.com/sneeuwballen/zipperposition>, <https://journals.flvc.org/FLAIRS/article/view/138642>, and 33 more in the note |
| applications-1 | `scope-applications.md` | <https://cyber.bibl.u-szeged.hu/index.php/actcybern/article/view/3599> |
| applications-2 | `scope-applications.md` | <https://raw.githubusercontent.com/chrisamaphone/interactive-lp/master/LICENSE> |
| applications-3 | `scope-applications.md` | the repository's own files and other notes only |
| applications-4 | `scope-applications.md` | <https://docs.rs/crate/par/0.3.2>, <https://arxiv.org/abs/2510.13082v1> |
| applications-5 | `scope-applications.md` | <https://www.lix.polytechnique.fr/~dale/lolli>, <https://github.com/clf/celf>, <https://github.com/clf/celf/commits/master>, <https://github.com/clf/lollimon> |
| applications-6 | `scope-applications.md` | <https://arxiv.org/abs/2309.10629>, <https://arxiv.org/abs/2211.12603>, <https://www.pnnl.gov/publications/crnt4sbml-python-package-detection-bistability-biochemical-reaction-networks>, <https://synkit.readthedocs.io/en/stable/crn.html> |
| applications-7 | `scope-applications.md` | <https://www.mpi-sws.org/~dg/papers/aff-know.pdf>, <https://www.mpi-sws.org/~dg/papers/NDSS07Final.pdf>, <https://people.mpi-sws.org/~dg/papers/thesis.pdf> |
| applications-8 | `scope-applications.md` | from the 2026-10 note's section: <https://arxiv.org/abs/0910.2401>, <https://arxiv.org/abs/q-bio/0608019>, <https://research.tees.ac.uk/en/publications/linear-logic-for-non-linear-storytelling/>, <https://arxiv.org/abs/1710.09756>, <https://pldi20.sigplan.org/details/pldi-2020-papers/47/Silq-A-High-Level-Quantum-Language-with-Safe-Uncomputation-and-Intuitive-Semantics>, <https://arxiv.org/abs/2004.05106>, and 37 more in the note |
| applications-9 | `scope-applications.md` | the repository's own files and other notes only |
| applications-10 | `scope-applications.md` | <https://arxiv.org/abs/2501.00169>, <https://arxiv.org/abs/2408.09516>, <https://arxiv.org/abs/2510.13082v1>, <https://arxiv.org/pdf/1909.05970>, <https://arxiv.org/abs/2309.10629>, <https://github.com/clf/celf>, and 22 more in the note |
| education-1 | `scope-education.md` | <https://arxiv.org/abs/1803.01467>, <https://arxiv.org/pdf/1803.01467>, <https://click-and-collect.linear-logic.org/> |
| education-2 | `scope-education.md` | <https://ll-handbook.pages.math.cnrs.fr/>, <https://ll-handbook.pages.math.cnrs.fr/book/ll-handbook-draft.pdf>, <https://staff.aist.go.jp/s-matsuoka/PNCalculator/index.html> |
| education-3 | `scope-education.md` | <https://click-and-collect.linear-logic.org/> |
| education-4 | `scope-education.md` | <https://iltis.rub.de/>, <https://arxiv.org/abs/2607.07722> |
| education-5 | `scope-education.md` | <https://www.ijcai.org/Proceedings/13/Papers/291.pdf> |
| education-6 | `scope-education.md` | <https://arxiv.org/abs/2607.04321>, <https://arxiv.org/abs/2505.05988>, <https://proof.compute.dtu.dk/>, <https://ankushdas.github.io/courses/CS599-S26.html> |
| education-7 | `scope-education.md` | <https://forallx.openlogicproject.org/html/A4.html> |
| education-8 | `scope-education.md` | <https://click-and-collect.linear-logic.org/>, <https://github.com/ComputerAidedLL/click-and-collect> |
| education-9 | `scope-education.md` | <https://ojs.aaai.org/index.php/AIIDE/article/view/12784> |
| education-10 | `scope-education.md` | the repository's own files and other notes only |
| education-11 | `scope-education.md` | <https://iltis.rub.de/>, <https://arxiv.org/abs/2607.07722>, <https://arxiv.org/abs/1803.01467>, <https://arxiv.org/pdf/1803.01467>, <https://click-and-collect.linear-logic.org/>, <https://ll-handbook.pages.math.cnrs.fr/>, and 18 more in the note |
| engines-1 | `scope-engines.md` | <https://github.com/chrjabs/rustsat>, <https://drops.dagstuhl.de/storage/00lipics/lipics-vol341-sat2025/html/LIPIcs.SAT.2025.15/LIPIcs.SAT.2025.15.html>, <https://github.com/jix/varisat>, <https://github.com/shnarazk/splr> |
| engines-2 | `scope-engines.md` | from the 2026-10 note's section: <https://ftp.math.utah.edu/pub/tex/bib/idx/tcs1990/135/1/155_169.html>, <https://software.imdea.org/Conferences/hcvs17/papers/paper_1.pdf> |
| engines-3 | `scope-engines.md` | from the 2026-10 note's section: <https://archive.model.in.tum.de/um/courses/petri/SS2017/material/cav2014-paper.pdf>, <https://arxiv.org/abs/1510.05724> |
| engines-4 | `scope-engines.md` | from the 2026-10 note's section: <https://arxiv.org/abs/1306.3542>, <https://arxiv.org/abs/1906.03611> |
| engines-5 | `scope-engines.md` | from the 2026-10 note's section: <https://arxiv.org/abs/1111.2249>, <https://arxiv.org/abs/2403.12869> |
| engines-6 | `scope-engines.md` | from the 2026-10 note's section: <https://arxiv.org/abs/1805.07563> |
| engines-7 | `scope-engines.md` | <https://arxiv.org/pdf/2404.07382>, <https://arxiv.org/abs/1811.00796> |
| engines-8 | `scope-engines.md` | from the 2026-10 note's section: <https://click-and-collect.linear-logic.org/>, <https://www21.in.tum.de/~wenzelm/papers/itp-pide.pdf> |
| engines-9 | `scope-engines.md` | from the 2026-10 note's section: <https://research.chalmers.se/en/publication/74330> |
| engines-10 | `scope-engines.md` | from the 2026-10 note's section: <https://hal.archives-ouvertes.fr/hal-00003530>, <https://dblp.org/pid/34/462>, <https://mcc.lip6.fr/2025/>, <https://unsolve-ipc.eng.unimelb.edu.au> |
| engines-11 | `scope-engines.md` | <https://tptp.org/UserDocs/TPTPLanguage/NonClassicalLogics.html>, <https://arxiv.org/abs/2508.09318>, <https://arxiv.org/html/2508.09318> |
| engines-12 | `scope-engines.md` | the repository's own files and other notes only |
| engines-13 | `scope-engines.md` | <https://github.com/RichardMoot/LinearOne> |
| engines-14 | `scope-engines.md` | the repository's own files and other notes only |
| engines-15 | `scope-engines.md` | <https://link.springer.com/doi/10.1007/11538363_15>, <https://researchportal.ip-paris.fr/en/publications/focusing-the-inverse-method-for-linear-logic/> |
| engines-16 | `scope-engines.md` | <https://tptp.org/UserDocs/TPTPLanguage/NonClassicalLogics.html>, <https://www.iltp.de/> |
| engines-17 | `scope-engines.md` | <https://github.com/chaudhuri/sympli>, <https://github.com/RichardMoot/LinearOne>, <https://tptp.org/UserDocs/TPTPLanguage/NonClassicalLogics.html>, <https://arxiv.org/abs/2508.09318>, <https://arxiv.org/html/2508.09318>, <https://tptp.org/CASC/30/DesignFull.html>, and 10 more in the note |
| interop-1 | `scope-interop.md` | <https://github.com/leanprover/cslib>, <https://github.com/leanprover/cslib/blob/main/Cslib/Logics/README.md> |
| interop-2 | `scope-interop.md` | <https://github.com/leanprover/cslib/tree/main/Cslib/Logics/LinearLogic/CLL>, <https://github.com/leanprover/cslib>, <https://github.com/leanprover/cslib/blob/main/Cslib/Logics/README.md>, <https://github.com/Deducteam/lambdapi/blob/master/CHANGES.md> |
| interop-3 | `scope-interop.md` | <https://github.com/Deducteam/lambdapi/blob/master/CHANGES.md>, <https://github.com/Deducteam/lambdapi>, <https://github.com/Deducteam/Dedukti>, <https://arxiv.org/abs/2311.07185> |
| interop-4 | `scope-interop.md` | <https://hal.archives-ouvertes.fr/hal-01390912>, <https://www.lix.polytechnique.fr/~dale/papers/lku.pdf> |
| interop-5 | `scope-interop.md` | <https://szs.tptp.org>, <https://arxiv.org/html/2508.09318v1>, <https://arxiv.org/abs/2508.09318>, <https://github.com/TPTPWorld/NonClassicalLogic> |
| interop-6 | `scope-interop.md` | <https://lean-lang.org/doc/reference/latest/IO/Processes/>, <https://github.com/ufmg-smite/lean-smt>, <https://rocq-prover.org/doc/V9.0.0/refman/proof-engine/ltac2.html>, <https://github.com/ejgallego/coq-lsp> |
| interop-7 | `scope-interop.md` | <https://github.com/PyO3/pyo3> |
| interop-8 | `scope-interop.md` | <https://wasi.dev/releases/wasi-p3> |
| interop-9 | `scope-interop.md` | <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/>, <https://docs.rs/crate/lsp-server/latest>, <https://tree-sitter.github.io/tree-sitter/creating-parsers/>, <https://github.com/ejgallego/coq-lsp> |
| interop-10 | `scope-interop.md` | from the 2026-10 note's section: <https://arxiv.org/abs/2311.07185>, <https://ocaml.org/p/lambdapi/3.0.0/lambdapi.export/Export/index.html>, <https://github.com/Deducteam/hol2dk/blob/main/README.md>, <https://ecosystem.code.gouv.fr/hosts/GitHub/owners/Deducteam>, <https://www.lix.polytechnique.fr/~dale/papers/checking-fpc.pdf>, <https://hal.archives-ouvertes.fr/hal-01390912>, and 29 more in the note |
| interop-11 | `scope-interop.md` | <https://arxiv.org/html/2508.09318v1>, <https://arxiv.org/abs/2508.09318>, <https://github.com/TPTPWorld/NonClassicalLogic> |
| interop-12 | `scope-interop.md` | <https://arxiv.org/html/2508.09318v1>, <https://arxiv.org/abs/2508.09318>, <https://github.com/Deducteam/lambdapi/blob/master/CHANGES.md>, <https://github.com/leanprover/cslib>, <https://lean-lang.org/doc/reference/latest/IO/Processes/>, <https://github.com/ejgallego/coq-lsp>, and 24 more in the note |
| logics-1 | `scope-logics.md` | <https://www.cs.cmu.edu/~fp/papers/xlang26.pdf>, <https://arxiv.org/abs/2507.12360>, <https://arxiv.org/abs/2509.00644>, <https://arxiv.org/abs/1502.04769> |
| logics-2 | `scope-logics.md` | <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2025.32>, <https://github.com/granule-project/granule>, <https://granule-project.github.io>, <https://granule-project.github.io/research.html> |
| logics-3 | `scope-logics.md` | <https://arxiv.org/abs/2505.13284> |
| logics-4 | `scope-logics.md` | <https://arxiv.org/abs/2505.13284>, <https://lipn.univ-paris13.fr/TLLA/2025> |
| logics-5 | `scope-logics.md` | <https://arxiv.org/abs/2602.01299>, <https://lipn.univ-paris13.fr/TLLA/2025> |
| logics-6 | `scope-logics.md` | <https://arxiv.org/abs/2509.00644> |
| logics-7 | `scope-logics.md` | <https://arxiv.org/abs/2605.12893>, <https://lipn.univ-paris13.fr/TLLA/2025> |
| logics-8 | `scope-logics.md` | <https://www.cs.cmu.edu/~fp/papers/xlang26.pdf>, <https://lipn.univ-paris13.fr/TLLA/2025>, <https://arxiv.org/abs/2605.14476>, <https://arxiv.org/abs/2603.03262>, <https://arxiv.org/abs/2603.01595>, <https://arxiv.org/abs/2505.13284>, and 11 more in the note |
| proof-theory-1 | `scope-proof-theory.md` | <https://www.lix.polytechnique.fr/~dale/papers/tcs08trackb.pdf>, <https://lmcs.episciences.org/1625>, <https://arxiv.org/abs/2110.02630>, <https://dc2.philarchive.org/rec/VELMMP>, <https://www.mathstat.dal.ca/wollic2023/papers/veltri.pdf>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.TLCA.2015.60>, and 4 more in the note |
| proof-theory-2 | `scope-proof-theory.md` | <https://www.irif.fr/~diguardia/papers/thesis_manuscript.pdf>, <https://www.ens-lyon.fr/evenement/recherche/identity-proofs-and-formulas-using-proof-nets-multiplicative-additive-linear>, <https://arxiv.org/abs/2603.03262>, <https://easychair.org/smart-program/FSCD2025/2025-07-18.html>, <https://doi.org/10.1145/2933575.2934559>, <https://people.bath.ac.uk/wbh22/pdf/2016-hughes-heijltjes.pdf>, and 2 more in the note |
| proof-theory-3 | `scope-proof-theory.md` | <https://arxiv.org/abs/1802.03224>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2019.22> |
| proof-theory-4 | `scope-proof-theory.md` | <https://github.com/ComputerAidedLL/click-and-collect>, <https://github.com/chaudhuri/profound>, <https://researchportal.ip-paris.fr/en/publications/subformula-linking-as-an-interaction-method/>, <https://arxiv.org/abs/2210.11820> |
| proof-theory-5 | `scope-proof-theory.md` | <https://www.arxiv.org/abs/2505.13284>, <https://arxiv.org/abs/0709.1201>, <https://lmcs.episciences.org/1081>, <https://arxiv.org/abs/0903.5392>, <https://arxiv.org/abs/cs/9910023>, <https://www.lix.polytechnique.fr/~lutz/papers/dissvonlutz.pdf>, and 6 more in the note |
| proof-theory-6 | `scope-proof-theory.md` | <https://github.com/ComputerAidedLL/click-and-collect>, <https://dmg.tuwien.ac.at/hetzl/research/towards.pdf>, <https://doi.org/10.1007/BF00247711>, <https://www.kurims.kyoto-u.ac.jp/~terui/pub_bib.html>, <https://www.springerprofessional.de/en/compression-of-propositional-resolution-proofs-by-lowering-subpr/4164474>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2025.32> |
| proof-theory-7 | `scope-proof-theory.md` | <https://en.wikipedia.org/wiki/Geometry_of_interaction>, <https://www.cambridge.org/core/books/abs/advances-in-linear-logic/proofnets-and-the-hilbert-space/4C074845DBBF5BA14529DE8FFFD8DDF3>, <https://spiral.imperial.ac.uk/handle/10044/1/46072>, <https://perso.ens-lyon.fr/olivier.laurent/goi.bib>, <https://lmcs.episciences.org/5882>, <https://doi.org/10.1145/96709.96718>, and 1 more in the note |
| proof-theory-8 | `scope-proof-theory.md` | <https://easychair.org/publications/paper/8r>, <https://deepblue.lib.umich.edu/handle/2027.42/30097>, <https://arxiv.org/abs/1311.6057>, <https://www.irif.fr/~mellies/papers/AbramskyMellies99.pdf>, <https://doi.org/10.1007/978-3-030-65824-3_8> |
| proof-theory-9 | `scope-proof-theory.md` | <https://sun.iwu.edu/~mliffito/publications/jar_liffiton_CAMUS.pdf>, <https://www.cs.ox.ac.uk/publications/publication4972-abstract.html>, <https://www.joachim-breitner.de/publications/Incredible_ITP2016_preprint.pdf>, <http://logitext.mit.edu>, <https://arxiv.org/abs/1803.01467>, <https://click-and-collect.linear-logic.org/> |
| proof-theory-10 | `scope-proof-theory.md` | <https://arxiv.org/pdf/2104.09115.pdf>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2022.19>, <https://annals.math.princeton.edu/2006/164-3/p09>, <https://people.bath.ac.uk/wbh22/pdf/2019-heijltjes-hughes-strassburger-LICS.pdf> |
| proof-theory-11 | `scope-proof-theory.md` | <https://www.springerprofessional.de/session-types-as-intuitionistic-linear-propositions/3352308>, <https://www.cambridge.org/core/journals/journal-of-functional-programming/article/propositions-as-sessions/0985539E5D607AC00FB00FF900BA1C86> |
| proof-theory-12 | `scope-proof-theory.md` | <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FSCD.2025.32> |
| proof-theory-13 | `scope-proof-theory.md` | <https://arxiv.org/abs/2402.11987>, <https://www.irif.fr/~diguardia/papers/thesis_manuscript.pdf>, <https://www.ens-lyon.fr/evenement/recherche/identity-proofs-and-formulas-using-proof-nets-multiplicative-additive-linear> |
| proof-theory-14 | `scope-proof-theory.md` | <https://perso.ens-lyon.fr/olivier.laurent/rocqpn.pdf>, <https://arxiv.org/abs/2603.03262>, <https://easychair.org/smart-program/FSCD2025/2025-07-18.html>, <https://arxiv.org/html/2506.21678v1>, <https://doi.org/10.1145/2933575.2934559>, <https://people.bath.ac.uk/wbh22/pdf/2016-hughes-heijltjes.pdf>, and 21 more in the note |
| semantics-decision-1 | `scope-semantics-decision.md` | <https://github.com/blondimi/qcover>, <https://arxiv.org/pdf/2302.14741>, <https://people.irisa.fr/Nicolas.Markey/biblio/tcs256> |
| semantics-decision-2 | `scope-semantics-decision.md` | <https://arxiv.org/pdf/1009.1076>, <https://arxiv.org/pdf/2302.14741>, <https://arxiv.org/pdf/2208.03043>, <https://lmcs.episciences.org/13141> |
| semantics-decision-3 | `scope-semantics-decision.md` | <https://link.springer.com/chapter/10.1007/978-3-031-57246-3_21>, <https://rocq-prover.org/p/coq-karp-miller> |
| semantics-decision-4 | `scope-semantics-decision.md` | from the 2026-10 note's section: <https://arxiv.org/abs/2010.07912>, <https://hal.archives-ouvertes.fr/hal-00346310> |
| semantics-decision-5 | `scope-semantics-decision.md` | <https://mcc.lip6.fr/2026/results.php>, <https://mcc.lip6.fr/2025/results.php>, <https://github.com/TAPAAL/verifypn> |
| semantics-decision-6 | `scope-semantics-decision.md` | <https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/abs/finite-model-property-for-various-fragments-of-linear-logic/F75D11ED6907E37EA0424226108DDA66> |
| semantics-decision-7 | `scope-semantics-decision.md` | <https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/abs/finite-model-property-for-various-fragments-of-linear-logic/F75D11ED6907E37EA0424226108DDA66>, <https://perso.ens-lyon.fr/olivier.laurent/yalla/> |
| semantics-decision-8 | `scope-semantics-decision.md` | from the 2026-10 note's section: <https://wrap.warwick.ac.uk/75834/>, <https://wrap.warwick.ac.uk/79162/>, <https://www.cs.cmu.edu/~iliano/papers/csfw99.pdf>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CONCUR.2024.31>, <https://eprints.ucm.es/20624/>, <https://kilthub.cmu.edu/articles/journal_contribution/Relating_State-Based_and_Process-Based_Concurrency_through_Linear_Logic/6608957> |
| semantics-decision-9 | `scope-semantics-decision.md` | from the 2026-10 note's section: <https://www.engineering.upenn.edu/~sweirich/types/archive/1991/msg00101.html>, <https://api.openalex.org/works/doi:10.1016%2F0168-0072%2894%2990085-X>, <https://lics.siglog.org/archive/2004/deGrooteGuillaumeSa-VectorAdditionTreeA.html>, <https://dmtcs.episciences.org/350>, <https://arxiv.org/abs/1401.6785>, <https://arxiv.org/abs/1402.0705>, and 45 more in the note |
| semantics-decision-10 | `scope-semantics-decision.md` | the repository's own files and other notes only |
| semantics-decision-11 | `scope-semantics-decision.md` | <https://ijcai.org/proceedings/2019/232.md>, <https://arxiv.org/abs/2603.29424>, <https://www.cambridge.org/core/journals/journal-of-symbolic-logic/article/abs/finite-model-property-for-various-fragments-of-linear-logic/F75D11ED6907E37EA0424226108DDA66> |
| semantics-decision-12 | `scope-semantics-decision.md` | <https://link.springer.com/chapter/10.1007/978-3-031-57246-3_21>, <https://rocq-prover.org/p/coq-karp-miller>, <https://www.lix.polytechnique.fr/Labo/Lutz.Strassburger/papers/OnDeciMELL.pdf>, <https://arxiv.org/abs/2502.13916>, <https://arxiv.org/html/2603.05221v1>, <https://mcc.lip6.fr/2025/results.php>, and 17 more in the note |
| types-higher-1 | `scope-types-higher.md` | <https://byorgey.github.io/blog/posts/2011/01/26/counting-linear-lambda-terms.html>, <https://arxiv.org/abs/1604.02086> |
| types-higher-2 | `scope-types-higher.md` | <https://github.com/ComputerAidedLL/click-and-collect>, <https://www.arxiv.org/abs/2401.17199>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2025.32>, <https://github.com/RichardMoot/LinearOne> |
| types-higher-3 | `scope-types-higher.md` | <https://www.sciencedirect.com/science/article/pii/S0890540124000993>, <https://arxiv.org/html/2203.12875>, <https://github.com/freest-lang/freest>, <https://opam.ocaml.org/packages/sill>, <https://www.ideals.illinois.edu/items/92895>, <https://lmcs.episciences.org/8954>, and 1 more in the note |
| types-higher-4 | `scope-types-higher.md` | <https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/linear_types.html>, <https://idris2.readthedocs.io/en/latest/tutorial/multiplicities.html>, <https://github.com/granule-project/granule>, <https://arxiv.org/abs/1903.00982>, <https://www.mpi-sws.org/news/programming-languages-and-verification/2018/> |
| types-higher-5 | `scope-types-higher.md` | <https://www.arxiv.org/abs/2401.17199>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2025.32>, <https://idris2.readthedocs.io/en/latest/tutorial/multiplicities.html>, <https://github.com/granule-project/granule> |
| types-higher-6 | `scope-types-higher.md` | <https://www.itu.dk/~carsten/papers/ijcar08.pdf>, <https://www.cs.cmu.edu/~fp/courses/15816-s12/software.html>, <https://ojs.aaai.org/index.php/AIIDE/article/view/12784> |
| types-higher-7 | `scope-types-higher.md` | <https://idris2.readthedocs.io/en/latest/tutorial/multiplicities.html>, <https://arxiv.org/abs/2603.29716>, <https://github.com/granule-project/granule>, <https://github.com/freest-lang/freest>, <https://www.arxiv.org/abs/2401.17199>, <https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.CSL.2025.32>, and 16 more in the note |
| structure-1 | `structure.md` | <https://doc.rust-lang.org/cargo/reference/workspaces.html>, <https://matklad.github.io/2021/08/22/large-rust-workspaces.html>, <https://github.com/Carnap/Carnap/blob/master/Carnap/Carnap.cabal> |
| structure-2 | `structure.md` | <https://doc.rust-lang.org/cargo/reference/publishing.html> |
| structure-3 | `structure.md` | <https://doc.rust-lang.org/cargo/reference/features.html>, <https://doc.rust-lang.org/cargo/reference/semver.html> |
| structure-4 | `structure.md` | from the note's field section: <https://github.com/isabelle-prover/mirror-isabelle/tree/master/src>, <http://dists.isabelle.cit.tum.de/dist/library/?token=Isabelle>, <https://isabelle.in.tum.de/overview.html>, <https://github.com/isabelle-prover/mirror-isabelle/blob/master/src/Sequents/LK.thy> |
| structure-5 | `structure.md` | <https://github.com/ezyang/logitext>, <https://github.com/rust-lang/rfcs/pull/3243>, <https://doc.rust-lang.org/cargo/reference/workspaces.html>, <https://doc.rust-lang.org/cargo/reference/features.html>, <https://doc.rust-lang.org/cargo/reference/semver.html>, <https://doc.rust-lang.org/cargo/reference/publishing.html>, and 31 more in the note |
| tableaux-1 | `tableaux.md` | <https://carnap.github.io/Carnap-Manual/truth-tree.html>, <https://richardzach.org/2018/02/>, <https://www.umsu.de/trees/>, <https://ctan.org/pkg/prooftrees>, <https://en.wikipedia.org/wiki/Method_of_analytic_tableaux> |
| tableaux-2 | `tableaux.md` | <https://carnap.github.io/Carnap-Manual/truth-tree.html>, <https://richardzach.org/2018/02/>, <https://richardzach.org/2020/01/adding-online-exercises-with-automated-grading-to-any-logic-course-with-carnap/> |
| tableaux-3 | `tableaux.md` | <https://www.umsu.de/trees/>, <https://dblp.uni-trier.de/rec/journals/jar/BeckertP95.html>, <http://www.leancop.de>, <https://en.wikipedia.org/wiki/Method_of_analytic_tableaux>, <https://library.holycross.edu/Record/b3192539/TOC> |
| tableaux-4 | `tableaux.md` | <https://richardzach.org/2018/02/>, <https://openlogicproject.org/2018/02/22/modal-logic-propositional-logic-tableaux/>, <http://www.irit.fr/Lotrec/>, <https://link.springer.com/doi/10.1007/11554554_25>, <https://www.umsu.de/trees/>, <https://www.ps.uni-saarland.de/spartacus> |
| tableaux-5 | `tableaux.md` | <http://www.leancop.de/lintap/>, <http://www.irit.fr/Lotrec/>, <https://link.springer.com/doi/10.1007/11554554_25>, <https://www.cs.man.ac.uk/~schmidt/publications/TishkovskySchmidtKhodadadi12b.html>, <http://www.leancop.de>, <https://www.ijcai.org/proceedings/2017/695>, and 18 more in the note |
| users-1 | `users.md` | <https://api.github.com/repos/linlog-prover/linlog>, <https://github.com/linlog-prover/linlog/discussions> |
| users-2 | `users.md` | <https://api.github.com/repos/linlog-prover/linlog/releases>, <https://api.github.com/repos/linlog-prover/linlog>, <https://crates.io/api/v1/crates/linlog>, <https://crates.io/api/v1/crates/linlog-cli>, <https://zenodo.org/records/23448588> |
| users-3 | `users.md` | <https://awesome.ecosyste.ms/projects/github.com%2Fibrahimcesar%2Flolli>, <https://codeberg.org/flgrubm/linlog>, <https://api.github.com/repos/linlog-prover/linlog/releases>, <https://api.github.com/repos/linlog-prover/linlog>, <https://github.com/linlog-prover/linlog>, <https://github.com/linlog-prover/linlog/issues?q=is%3Aissue>, and 13 more in the note |
| verification-1 | `verification.md` | the repository's own files and other notes only |
| verification-2 | `verification.md` | <https://github.com/linlog-prover/rocq-linlog> |
| verification-3 | `verification.md` | <https://rocq-prover.org/doc/V9.0.0/refman/addendum/extraction.html>, <https://github.com/MetaRocq/metarocq>, <https://github.com/ocsigen/js_of_ocaml>, <https://github.com/formal-land/coq-of-rust>, <https://github.com/CertiCoq/certicoq>, <https://github.com/linlog-prover/rocq-linlog> |
| verification-4 | `verification.md` | <https://verus-lang.github.io/verus/guide/features.html>, <https://raw.githubusercontent.com/creusot-rs/creusot/master/CHANGELOG.md>, <https://github.com/creusot-rs/creusot/releases>, <https://github.com/viperproject/prusti-dev/blob/master/docs/user-guide/src/verify/summary.md>, <https://github.com/viperproject/prusti-dev/blob/master/docs/user-guide/src/verify/closure.md>, <https://github.com/viperproject/prusti-dev/releases>, and 9 more in the note |
| verification-5 | `verification.md` | <https://github.com/AeneasVerif/aeneas>, <https://github.com/cryspen/hax>, <https://github.com/AeneasVerif/charon> |
| verification-6 | `verification.md` | <https://github.com/model-checking/kani>, <https://model-checking.github.io/kani/>, <https://github.com/model-checking/kani/releases>, <https://model-checking.github.io/kani/rust-feature-support.html> |
| verification-7 | `verification.md` | <https://plv.mpi-sws.org/refinedrust/>, <https://github.com/GillianPlatform/Gillian-Rust>, <https://github.com/formal-land/coq-of-rust>, <https://github.com/CertiCoq/certicoq>, <https://github.com/ocsigen/js_of_ocaml>, <https://github.com/linlog-prover/rocq-linlog>, and 25 more in the note |
| web-front-end-1 | `web-front-end.md` | <https://carnap.github.io/Carnap-Manual/systems.html>, <https://arxiv.org/html/2607.04321v1> |
| web-front-end-2 | `web-front-end.md` | <https://carnap.github.io/Carnap-Manual/systems.html>, <https://arxiv.org/html/2607.04321v1>, <https://github.com/ezyang/logitext>, <https://core.ac.uk/download/16156617.pdf>, <https://ar5iv.labs.arxiv.org/html/1005.2672>, <https://www.cs.us.es/~jalonso/cursos/li-06/sistemas/tableros/info.html> |
| web-front-end-3 | `web-front-end.md` | <https://vite.dev/guide/features.html> |
| web-front-end-4 | `web-front-end.md` | <https://vite.dev/guide/features.html>, <https://github.com/wasm-bindgen/wasm-bindgen>, <https://cdn.jsdelivr.net/npm/jscoq@0.17.1/docs/embedding.md>, <https://github.com/jscoq/jscoq/blob/master/docs/embedding.md> |
| web-front-end-5 | `web-front-end.md` | the repository's own files and other notes only |
| web-front-end-6 | `web-front-end.md` | <https://carnap.github.io/Carnap-Manual/systems.html>, <https://arxiv.org/html/2607.04321v1>, <https://www.cs.us.es/~jalonso/cursos/li-06/sistemas/tableros/info.html>, <https://github.com/6ximik9/naturalDeduction> |
| web-front-end-7 | `web-front-end.md` | <https://github.com/leanprover-community/lean4web>, <https://core.ac.uk/download/16156617.pdf>, <https://ar5iv.labs.arxiv.org/html/1005.2672>, <https://www.goodfirstissue.org/ComputerAidedLL/click-and-collect>, <https://perso.ens-lyon.fr/olivier.laurent/ll2022/>, <https://arxiv.org/html/2607.04321v1>, and 23 more in the note |
| web-teaching-1 | `web-teaching.md` | <https://www.joachim-breitner.de/publications/Incredible_ITP2016_preprint.pdf> |
| web-teaching-2 | `web-teaching.md` | <https://arxiv.org/abs/2607.07722>, <https://www.joachim-breitner.de/publications/Incredible_ITP2016_preprint.pdf>, <https://iltis.rub.de/> |
| web-teaching-3 | `web-teaching.md` | <https://arxiv.org/abs/1803.01467>, <https://arxiv.org/html/1803.01467v1> |
| web-teaching-4 | `web-teaching.md` | <https://www.w3.org/TR/WCAG22/> |
| web-teaching-5 | `web-teaching.md` | <https://arxiv.org/abs/2607.07722>, <https://carnap.io/book>, <https://arxiv.org/abs/2303.05865> |
| web-teaching-6 | `web-teaching.md` | the repository's own files and other notes only |
| web-teaching-7 | `web-teaching.md` | <https://github.com/ComputerAidedLL/click-and-collect> |
| web-teaching-8 | `web-teaching.md` | <https://www.joachim-breitner.de/publications/Incredible_ITP2016_preprint.pdf> |
| web-teaching-9 | `web-teaching.md` | <https://www.joachim-breitner.de/publications/Incredible_ITP2016_preprint.pdf>, <https://arxiv.org/abs/1803.01467>, <https://arxiv.org/html/1803.01467v1> |
| web-teaching-10 | `web-teaching.md` | <https://www.joachim-breitner.de/publications/Incredible_ITP2016_preprint.pdf>, <https://iltis.rub.de/>, <https://www.imsglobal.org/spec/lti/v1p3/>, <https://arxiv.org/abs/2607.07722>, <https://arxiv.org/abs/2308.06970>, <https://github.com/RBornat/jape>, and 15 more in the note |

### Correctness verification, and its feasibility

What linlog trusts today (`verification.md`, section 1, read from the
code and the rules files): the Rust checker (`core/src/proofs/check/`,
about 3 700 lines with its frames, zones and errors), which validates
every proof the search returns and whose shape is arena indices as
`u32` newtypes into boxed slices, saturating `u32` counts and `u64`
sums, hash tables of members, a `dyn FnMut(Progress) -> bool` stop
closure, generics over an observer and over first-order instancing, no
`unsafe` anywhere (`unsafe_code = "forbid"`); `Disproof::check` for the
refutations that have a check; the intuitionistic reading; the parser;
the exporter that writes certificates. Verified: the Rocq library's
checkers with their soundness theorems, which run only on exported
certificates, and the invariants behind the certified refutations.
Tested: differential tests against the test-only `oracle.rs`, mutation
runs and fuzzing.

The tools, each read in its repository and documentation today: Prusti
(Viper; attributes for pre- and postconditions and loop invariants; by
default panic and overflow freedom; partial correctness; the user guide
says closures are "NOT YET SUPPORTED"; the last tagged release is of
2024-02-28 and only automated nightly pre-releases since, which this
session confirmed on its releases page), Creusot (LGPL-2.1, to Why3;
monthly releases, v0.14.0 on 2026-10-12, iterator and slice
specifications growing, specifications for `HashMap` and `HashSet`
iterators since 0.4.0 of 2025-03-03 (the note said none; corrected at
the review), a
`flake.nix`), Verus (SMT with Z3; releases about weekly on a pinned
stable compiler 1.99.1; closures as expressions, traits, `HashMap`,
`Vec` supported, iterators partially; "features may be broken and/or
missing"), Kani (bounded model checking with CBMC; harnesses with
`kani::any()`; monthly releases on a nightly; most language features
including closures and traits), Aeneas (Rust to Lean, HOL4, and less
mature Coq and F* back ends), hax (Rust to F*, with Coq and Lean back
ends marked experimental), Flux (refinement types; not read in depth),
and for an extracted checker Rocq's extraction to OCaml, Haskell and
Scheme with `wasm_of_ocaml` as the way into a browser, CertiCoq to C
and MetaRocq's verified extraction as directions, none of which gives
Rust.

The alternatives, with what each would prove, what it would remove from
the trusted base, and its cost:

| alternative | proves | removes from the trusted base | cost | verdict |
|---|---|---|---|---|
| verification-2: more certificates checked by the Rocq library (the one-sided and state-equation refutations have neither a check nor a lemma today; intuitionistic and first-order proofs are already certified) | each new kind of verdict has a machine-checked witness | nothing from the Rust base; extends the certified part | 2 to 3 for (i) and (ii); the failure certificates of the backward search and the saturated sets (iii) need focusing completeness in Rocq, a project of its own | **first** |
| verification-6: Kani harnesses for the integer and bound facts of the checker | no wrap, no out-of-bounds index, within a bound | nothing, but it targets the one fault class the project has met (a wrapped counter, `plan/conduct.md`) | 1 to 2 | second |
| verification-1: a second checker of a different shape, fuzzed against the first | agreement of two transcriptions | nothing | 1 | third |
| verification-3: a checker extracted from Rocq beside or instead of the Rust one, native then in the browser | the extracted function is the proved one | nothing: the reader that feeds it and the extraction become trusted instead; in the browser it would be a second module | 2 + 1 to 2 | only if a user without Rocq needs a verified verdict |
| verification-4: functional correctness of the Rust checker with a deductive verifier | the checker accepts exactly the derivable sequents, given the calculus in the tool's specification language | the checker | 4 to 8 on a refactored core | a project of its own; **Prusti does not meet the author's "as good a fit" clause** on the sources read (closures unsupported, release state); Verus and Creusot both fit the code shape on the sources read (Verus: a stable pinned compiler, closures as expressions, `HashMap` supported; Creusot: monthly releases, `HashMap` and `HashSet` iterator specifications since 0.4.0); a spike on `zones.rs` in both would decide |
| verification-5: the Rust checker refines the Rocq checker through Aeneas or hax | the strongest statement: the Rust checker agrees with the proved one | the checker | larger than 4 | the Rocq back ends of both tools are their least mature; a project of its own |

Feasible in a step or two: verification-2 (i) and (ii), verification-6,
verification-1. Projects of their own: verification-3's browser half,
verification-4, verification-5. This session's recommendation
(inference, with the note): take verification-2 (i, ii) into 0.3.0 and
verification-6 into a maintenance step, and defer the deductive
verification until the checker core is refactored for it, choosing the
tool then by a spike rather than now.

### What users say, from what is public

Read on 2026-11-08 (`users.md`): the three repositories have 0 stars, 0
forks and 0 watchers; `linlog` has no issues, no pull requests and no
Discussions tab; the v0.2.0 release assets total 25 downloads; the
crate `linlog` has 67 downloads (26 of 0.2.0) and `linlog-cli` 43 (21);
docs.rs built 0.2.0 with every item documented; the Zenodo record of
linlog has 9 views and 2 downloads, the library's 1 and 1; OpenAlex
indexes both DOIs and lists no citing work; no blog, forum or chat
mention was found. For scale, Click & coLLecT has 27 stars and 19 open
issues, Carnap 349 stars and 103 open issues. The finding is the
absence: two weeks after a release, with a page that tracks no one,
there is no public statement of use beyond the author's. What follows
(users-1 to 3, all without tracking): switch on Discussions with a "who
uses linlog" thread and a citation request in README and the page's
About pane, read the public counters once a month into a table under
`plan/notes/`, and one announcement where users of this kind are (the
author's act), pointing at the page's lessons, which need no install.
For the plan: no candidate below can claim a user's demand; the
evidence of use in the notes is of the kind of tool (courses using
Carnap, the Incredible Proof Machine, Click & coLLecT), not of linlog.

### What AI models and agents need beyond the guide

`llm-agents.md` re-read the sources of step 44's "no, not now" on an
MCP server: the Model Context Protocol's revision of 2026-07-28 is
stateless (sessions and the handshake removed), which matches linlog's
stateless calls; the Rust SDK `rmcp` is at 3.9.0 (2026-11-05) and
implements it; Skills over MCP reached Final on 2026-09-13. So `linlog
mcp` is about 1.5 sessions and the verdict moves from "no" to "when a
client without a shell asks" (llm-agents-1). Cheap now: a dataset of
problems with verified answers and checked proofs for evaluating models
(llm-agents-2, about 1 session; one search found no linear-logic
benchmark for models; the LLTP's GPL-3.0 keeps it out, the generated
families and ILTP carry it), and a one-call grading mode over the
existing `interact` session script (llm-agents-6). Structured hints and
errors for models (llm-agents-4) wait for a trial with a small model,
since refutation kinds and error codes exist already; the Python
package (llm-agents-3) and a narrated `--explain` (llm-agents-5) are
not recommended yet. How other provers reach agents (Lean's
lean-lsp-mcp and LeanInteract, Rocq's petanque in `coq-lsp`, Isabelle's
client) is in the note with links.

### Maintenance

`maintenance.md` reads the toolchain and the ecosystem today: stable
Rust is 1.99.1 and linlog's MSRV 1.92 is seven minor releases behind,
with no written policy; the oldest flake input is of 2026-04-13; the
actions are SHA-pinned, watched weekly by `.github/dependabot.yml`
(grouped, since 2026-09-29) and moved by the `update-deps` skill, with
no notification of a new advisory between runs; the wasm module is built with the
toolchain's default feature set and no check records which features
the module uses; the page's WCAG 2.2 AA floor was swept by hand (step
38) and the SVGs' reading for screen readers was never tried with a
screen reader. Recommended, in order (maintenance-1 to 4): a written
cadence and MSRV policy (for example "stable minus two, moved at each
`update-deps`"), a scheduled advisory check beside the skill (the
actions are watched already), a wasm feature baseline checked on the built module
with a bindgen policy, and an accessibility audit with axe or
Lighthouse and a screen-reader pass over the exports. Then the install
channels by users reached against cost (maintenance-7: winget, a
Homebrew tap, Scoop, Chocolatey; the AUR's rules and COPR's pages could
not be read), keyless cosign signatures beside the provenance
attestation (maintenance-5), platform code signing only with the
accounts and fees the author decides (maintenance-6; Apple's page gave
no fee, Windows costs were not found), the glibc binary (maintenance-8),
and a budget for crates and modules so the collection stays cheap to
build (maintenance-9). Rejected: nixpkgs for now (the author's word)
and MathML for the formulas.

### The scope notes, brought up to date

Each of the eight notes of 2026-10 was re-read against what steps 38
to 45 built, its sources re-checked where a page could be read, and
2025 and 2026 work added. In one line each:

- **logics** (`scope-logics.md`): none of logics-1 to 8 is built;
  logics-3 is partly (Lambek and cyclic MLL exist, no modalities, the
  ordered mode refuses cut); cut, boxes and the Rocq library make every
  exponential candidate cost more than the old note said (a
  cut-elimination case and a library mirror per label); new in 2025 and
  2026: Suzuki and Sano's undecidability of linear logic without
  exponential weakening, Granule's mixed linear and graded calculus (CSL
  2025), Intuitionistic BV, Curzi and Leigh on μMALL cut elimination;
  the ranking puts one "exponentials by signature" step first as a mode
  of the linear suite, and counts μMALL, BV and pomset logic as new
  suites.
- **proof theory** (`scope-proof-theory.md`): cut elimination built,
  cut introduction and redundancy removal not; certified refutations
  built, their reader-facing explanations not; new: interpolants by cut
  introduction (Saurin, FSCD 2025), MALL type isomorphisms (Di Guardia
  and Laurent, LMCS 2025), a certified net criterion in the Rocq
  library, Yeo's theorem generalised (FSCD 2025); explaining
  unprovability, subformula linking and proof equivalence generalise
  beyond linear logic.
- **engines** (`scope-engines.md`): nine of eleven untouched or partly
  built; the inverse method beats the default on three families with no
  dispatch row (engines-12); LinearOne's eight first-order problems
  (engines-13); the decisiveness defect of `comparison-gaps.md` stands
  unmeasured against 0.2.0's engines (no library comparison with them
  exists); a generic focused engine over rule schemas (engines-17) is
  the engine-side counterpart of the custom-rules layer.
- **interop** (`scope-interop.md`): JSON wire levels exist, no schema;
  the TPTP World's 2025 paper names linear and intuitionistic logics as
  "obvious candidates" to standardise next (TPTP 9.3.1); Lambdapi 4.0.0
  (2026-11-05) exports to Lean; ILTP's page is dated 2017; WASI 0.3.0
  was released 2026-06-11; a `linlog serve` mode (interop-12) and native
  reading of TPTP's non-classical syntax for a modal suite (interop-11)
  are new.
- **applications** (`scope-applications.md`): applications-5 (a
  logic-programming mode) is now cheap since quantifiers exist (2 to 3
  sessions); the rule-system front end (applications-2 with new
  applications-9, first-order rule systems by grounding) is the one to
  design as a generic forward-chaining mode beside the custom-rules
  layer; Ceptre's licence (the Anti-Capitalist Software License) and
  Celf's inactivity since 2019 noted.
- **education** (`scope-education.md`): the hint ladder (education-3)
  and the link (education-8's share) are built; the provability light
  (education-1) is deliberately on request only (D11); net exercises
  (education-2), the Ordinary tab (education-6) and auto-reverse are
  untouched; course packs with a checkable handed-in link and an embed
  rank first for teaching; new: a lesson format as data (education-10)
  and a local learner record (education-11); LogicProof (2026) and
  MiniCalc are new tools, both classical and constructive only.
- **semantics and decision** (`scope-semantics-decision.md`): the
  inverse method's `Saturated` refutation partly covers
  semantics-decision-6 and is the template for a basis certificate
  (-1, now cheaper); the state equation exists uncertified; MELL
  decidability is still open; Ackermann-completeness of VASS
  reachability is the ground truth for the Horn engine's promises; new:
  a Rocq certificate for saturated refutations (-10), the shared
  countermodel kind (-11), failure certificates of the backward search
  (-12); the manual's first planned item should be split: basis and
  state-equation certificates and Kripke countermodels first, the
  Rocq-side completeness work as a project of its own.
- **higher types** (`scope-types-higher.md`): none built;
  logic programming with terms is partly there (terms and unification,
  no forward chaining over them); sessions lost their blocker (cut
  exists); Granule's CSL 2025 calculus; a linear lambda-term checker
  (types-higher-8) closes the loop on the term printer; the ground is
  divided with the lambda-calculi suite: linear-side features are stated
  on an ILL proof, the suite takes terms that exist without a proof.

### The web page as students and teachers use it

`web-teaching.md` counts what the client has on 2026-11-08 (452
problems in the store, 407 provable, where the client's README still
says 425 and 392: a line for the client's next session), no lesson
progress key, no print stylesheet, English only, no automated
accessibility audit and no structural twin of the drawings for screen
readers. What teachers need, from the tools and the sources read:
Carnap's model is instructor accounts, assignments and autograding on
a server, which the page's design rules out (D31 hosting, D36 no
tracking), and LTI 1.3 needs a server and signed launches, so both are
rejected; what fits the page is a pack or assignment link with a check
view a teacher opens on a handed-in link (web-teaching-2, the same item
as education-4 with education-8's embed), a learner's local record that
can be exported as a file (web-teaching-1), a provability light behind
the Learn toggle (web-teaching-3, education-1), a screen-reader path
with a structural twin of the drawing reviewed with a real user
(web-teaching-4), and, when the multi-logic stage exists, Learn and
Practice keyed by suite (web-teaching-5, the largest cost and mostly
content). The education research the page already follows (worked
examples and fading, immediate feedback, hints on request, erroneous
examples) was read at abstract level only, so the note claims no
effect sizes; the evidence of course use is of the kind of tool, not of
this page (no syllabus citing the Incredible Proof Machine, Click &
coLLecT or Logitext was found before the search budget ran out).

### The decisions for the author

Each can be answered in a line. The recommendation comes first.

1. **The collection.** (a) Grow linlog into a collection of suites one
   suite at a time, in the order above, with `linlog` kept as the linear
   suite. (b) Stay a linear logic suite and take only the linear items
   (lambda-terms, certified refutations, the dispatch rows, exponentials
   by signature, explaining unprovability) plus the Ordinary tab as D57
   planned. I recommend (a): the second suite already half exists, the
   notes found the generic layer every method shares, and the linear
   suite gains at every release either way.
2. **The structure.** (a) One repository and one workspace: `linlog`
   keeps its name and crate; a small kernel crate is extracted once the
   second non-embedded suite and `ordinary/` have shown what they copy,
   unpublished until a second suite uses it; the arena, forest and proof
   term stay in `linlog`; one web module until a suite from another
   repository arrives; `rocq-linlog` stays the linear library in its
   repository. (b) A repository per suite from the start, with a
   published kernel. (c) One crate with a feature per logic. I recommend
   (a); (b) costs 2 to 3 sessions of infrastructure per repository
   before any logic and freezes the kernel's API on crates.io too early;
   (c) makes every variant a 0.y bump for every user.
3. **The name and the organisation.** (a) Create an umbrella
   organisation under a new name, transfer the three repositories, rename
   the site repository, keep `linlog-prover` as a held organisation with
   a stub site that redirects the published URLs, keep `linlog` as the
   linear suite's name and crates, do it before the opam pull request is
   submitted, and consider a custom domain so no later rename touches a
   published URL. (b) Rename `linlog-prover` in place (the Pages URLs
   break, the old name is free to squat). (c) Keep `linlog-prover` and
   add the new repositories to it. I recommend (a); if you would rather
   not decide a name now, (c) costs nothing today. The names free on
   every registry checked on 2026-11-08 are `sequentium`, `gentzenlab`,
   `turnstilelab` and `prooflings`; `syllogos` is free but for the
   `.org`; the short classical names are taken on GitHub. The choice is
   yours; nothing has been registered.
4. **The first logics and their order.** (a) As the release table:
   ordinary logic on the page (0.3.0), intuitionistic logic decided with
   Kripke countermodels and the beginner's tableaux (0.4.0), custom rules
   and natural deduction (0.5.0), modal logics (0.6.0), resolution and
   the typed lambda calculus (0.7.0). (b) Modal logic before custom
   rules and natural deduction (its engine needs nothing generic; its
   step-by-step calculus does). (c) Another order you name. I recommend
   (a): the generic layer is built with the method that needs it most
   (natural deduction) and modal logic then gets it for free.
5. **Custom rules: the scope.** (a) A second, generic derivation type
   beside the fast linear checker, with a rule language (JSON first), a
   checker by matching, interactive building in the command and on the
   page, every output flagged "valid in the calculus given", no
   certificate, and bounded search only if asked later. (b) The same
   plus bounded backward search from the start. (c) A rewrite of the
   kernel so that the fixed calculi become instances of the generic one.
   I recommend (a); (c) rewrites the trusted, measured, Rocq-linked core
   for a feature that adds nothing to the fixed calculi.
6. **The web front end's shape.** (a) One stage; the tab chooses the
   suite, the method and the parameters; the registry written with the
   Ordinary tab; one module until a suite from another repository
   arrives, then a module per suite loaded lazily; views for list, graph
   and text proofs built each with the suite that needs it; Learn and
   Practice keyed by suite. (b) A page per suite under one site. I
   recommend (a), which is your "unified view with the center stage".
7. **Cyclic proofs and System F.** (a) Research items after the
   decidable suites: the trace-condition checker and μMALL first if any,
   System F as checking only. (b) μMALL in the first year as the linear
   suite's own extension (6 to 9 sessions). I recommend (a): provability
   is undecidable in every variant the notes found, no teaching use was
   found, and the data model (binders over formulas, a graph proof type)
   is the largest change of all.
8. **Correctness verification.** (a) Certify the one-sided and
   state-equation refutations in the Rocq library now, Kani harnesses for
   the checker's integers in a maintenance step, and defer deductive
   verification until the checker core is refactored for it, choosing
   the tool then by a spike on `zones.rs` in Verus and Creusot. (b)
   Start a verification project now with Prusti. I recommend (a); on the
   sources read Prusti does not meet your "as good a fit" clause
   (closures unsupported, no tagged release since 2024-02).
9. **Three small acts of yours**, each a yes or no, each recommended
   yes: switch on GitHub Discussions for `linlog` with a "who uses
   linlog" thread (a `gh` administrative task; I recommend yes, since
   there is no channel today and an issue tracker invites bugs, not
   use); one announcement where users of this kind are (the text
   prepared by a session, the posting yours; I recommend yes, pointing
   at the page's lessons, after decision 3 so that the links last); the
   opam pull request after, not before, decision 3 (I recommend that
   order, since the fields are free to edit before submission).

### What the next sessions must know

- **46b writes the prompts** from decisions 1 to 8, numbering the
  chosen steps from 47 in the plan's table with their models, sessions
  and dependencies, and makes `plan/later.md` true: every candidate of
  the ranked list assigned, kept with its reason, or dropped with it.
  The release table above is the roadmap the prompt asks for if the
  author chooses the collection.
- **The first step of any order is measurement**, since this session
  ran nothing: the wasm module's size compressed and uncompressed,
  `cargo build --timings` of the workspace, and the ratchet and the
  target set on the current commit (`structure.md`'s first step), so
  that the kernel extraction and the module question have numbers.
- **The ordinary JSON forms and interactive LK and LJ** are the
  library's half of the Ordinary tab (`plan/later.md`, "Ordinary logic's
  JSON forms", and "Follow-ups: ordinary logic" with `interact
  --logic`); `ordinary-sequents.md` recommends a native goal state of
  ordinary sequents (design b) over a session on the image (design a).
- **The generic derivation type** (custom-rules-1) must carry, from its
  first design, hypothesis nodes with identity and a discharge link
  (natural deduction), a scope tree (Fitch), closure by complementary
  literals (tableaux), labels and relational atoms as a sort (modal),
  and the same bounds as the linear checker (a size estimate before
  building, no recursion on depth, `Limits` and `stop`).
- **Claims this report rests on that were read again here**: GitHub's
  pages on renaming an organisation and transferring a repository (the
  transfer page says "we don't redirect GitHub Pages associated with
  the repository"; the rename page is silent on Pages and says the old
  name becomes available); the BI undecidability paper (arXiv
  2603.01595, Galatos, Jipsen, Knudstorp, Ramanayake); Prusti's releases
  page (tagged 2024-02-28, nightlies since); the client's greyed
  Ordinary button (`TopBand.tsx`); that `core/src/ordinary/` has no
  interactive proving; the client's store of 452 problems.
- **What no note could source** is listed in each note's last section;
  the most consequential gaps: crates.io's pages on Trusted Publishing
  and name policy could not be rendered (the policy text is from a
  search snippet), no Zenodo page on renames, no measurement of the
  module's compressed size after step 42, last-activity dates of most
  GitHub repositories (the pages showed none), and the education
  research read at abstract level only.
- **The client's README** says 425 problems and 392 provable; the store
  holds 452 and 407. A client session corrects it.
- **The comparison on the libraries with 0.2.0's engines** (the manual's
  third planned item) is still the only way to measure
  `comparison-gaps.md`'s decisiveness defect against today's engines
  (`scope-engines.md`); it needs a night the author gives.

### Departures from the design, and assumptions

- **The order note was written by the synthesis session, not by a
  research sub-agent** (departs from the design): `order.md` is a
  synthesis of the other notes, so it carries their sources rather than
  its own, and says so in its first paragraph.
- **The scope notes were not edited in place.** Each has an update
  beside it under `next/` (`scope-*.md`) that states every old
  candidate's state and continues its numbering; the notes of 2026-10
  stay as the record of what step 17's follow-up research found.
- **The sub-agents' brief of "what linlog has today"** pointed at
  `docs/src/status.md`, README, `bench/TOOLS.md`, the area's scope note
  and `plan/later.md`, with the step reports named where an area leaned
  on one, rather than at every report of steps 38 to 45.
- **Several notes run over the brief's 400 lines** (the name note at
  516, education, web-teaching, logics and the agents note); none was
  cut, since the overrun is tables of checks and sources.
- **The users agent made two calls to `api.github.com`** of the five it
  was allowed; every other agent read `github.com` pages only.
- **Nothing was measured.** The module's size after step 42 is the
  report's figure (6.9 MB uncompressed); its compressed size and the
  build timings are the first step of any order.
- **Cost estimates** are the notes' and this session's judgement in
  sessions, marked "(inference)" in each note; none rests on a
  measurement.

### The reviewer's round

A fresh-context reviewer (Opus 5.5 at `high`), one round, on the
notes and this report at change `olvzzyyt`: it checked 41 claims, 28
against web sources and 13 against the repository or the client; 37
held (among them GitHub's rename and transfer pages, the BI paper,
Prusti's closures page and releases, the MCP revision's statelessness
and `rmcp` 3.9.0, TPTP's "obvious candidates", Lambdapi 4.0.0's Lean
export, Rocq's `tauto` on LJT, the μMALL complexity results, Cyclist
and Sequoia, Carnap's `system` attribute, LogicProof, crates.io's
download counts, three name checks, Suzuki and Sano, Ceptre's licence,
Verus's toolchain, Creusot's v0.14.0, typst's `default-members`,
MiniCalc, the client's 452 and 407 problems, the 6.9 MB module, the
checker's size, the uncertified refutation kinds, the occurrence
counts). Its sixteen findings and what was done with each:

1. **Creusot's changelog does list `HashMap`** (0.4.0, 2025-03-03,
   specifications for `HashMap` and `HashSet` iterators), where the
   verification note said none and this report repeated it to rank
   Verus above Creusot. Confirmed against the changelog here. Corrected
   in the note (marked) and in the report: both tools fit the code
   shape on the sources read; the spike on `zones.rs` decides between
   them. The ranking of verification-4 does not move.
2. **The custom-domain redirect**: the report said a custom domain
   makes the `github.io` address redirect to it; GitHub's page describes
   redirects only between the apex and `www` forms of the custom domain,
   as the note said. Confirmed here. The report now says so and keeps
   the custom domain as a way to publish URLs that name a domain rather
   than the organisation, marked inference. Decision 3's recommendation
   does not rest on the redirect.
3. **"A renamed opam package is a new package" and "a crate is never
   renamed"** were stated as fact where the note marks them inferred or
   unsourced. Both now read as inferences from the pages that could be
   read.
4. **The name note paraphrased GitHub's transfer page** as "The Pages
   site itself is not redirected" inside quotation marks; the page says
   "we don't redirect GitHub Pages associated with the repository". The
   note now quotes it exactly.
5. **Ladner's results**: the report added K4 to the PSPACE list and
   wrote "S5 NP-complete" without saying satisfiability, where the note
   says K, T and S4, and S5 non-provability NP-complete (validity coNP).
   Corrected to the note's statement, marked as read at abstract level.
6. **"Tableaux can be exponentially worse than resolution"** is not what
   the tableaux note says (it cites a quasi-polynomial simulation of
   tree resolution by tableaux). Replaced by the note's statement.
7. **verification-7, a certificate download on the page, is built**:
   the client's bottom band exports the finished proof's and a refuted
   goal's Rocq certificate. Confirmed in `BottomBand.tsx`. Dropped from
   the ranked list and marked in the note.
8. **maintenance-2's actions half is built**: `.github/dependabot.yml`
   watches the actions weekly since 2026-09-29, where the maintenance
   note said there was none. Confirmed here. The note and the report's
   maintenance section and ranked list now say so; what remains of the
   candidate is an advisory check between `update-deps` runs.
9. **"No countermodel" for ordinary logic** was too broad: a classical
   "not valid" gets a falsifying assignment and a lemma over `Prop`.
   The table now says no Kripke countermodel for intuitionistic or
   minimal logic.
10. **Costs that differed from the notes'**: tableaux-3 (3 to 5, not
    3), resolution-2 (4 to 6, not 3), resolution-4 (1 to 2, not 1),
    tableaux-4 (4 to 6, not 3 to 4), ordinary-sequents-4 (1, not 2),
    and the sentence calling first-order tableaux and resolution
    "cheap" where the notes cost them at 3 to 6 sessions each and
    tableaux-3 needs a first-order ordinary syntax. All corrected to the
    notes' figures; the sentence now says "got cheaper".
11. **A prerequisite after its dependant**: resolution-2 needs the
    first-order TPTP reader of resolution-4, which sat in the unranked
    tail. resolution-4 now shares rank 31 and the row says it comes
    first.
12. **llm-agents-1's risk** read "none" where the note names two
    (`rmcp`'s breaking major releases; chat clients may need a remote
    HTTP server, so hosting). The row now carries both.
13. **0.4.0's promise that "unknown" disappears** overstated the note,
    which says hard ILTP problems may still time out. The release row
    now says the search always terminates and the 35 loop-check cases
    are settled, while hard problems may still time out.
14. **Decision 9 gave a recommendation only for the opam order.** It
    now recommends yes on all three, with the reason each.
15. **`users.md` lacked "What the structure must allow"** and the
    brief's "The field" heading. A short section was added and the
    mapping of its sections to the brief's stated.
16. **The reviewer's section did not exist yet** when it read the
    report (pending, not a finding). It is this section.

Findings 1, 2, 7 and 8 are the four of its 41 checks that failed; the
rest are statements of this report that its notes did not support, now
aligned with the notes. No ranking changed place except that
verification-7 left the list and resolution-4 moved to rank 31.

### Checklist

- [x] Workspace `../linlog-step46a` on `rwsoxnoz`.
- [x] Read the brief, `plan/46-research-next.md` whole,
  `plan/conduct.md`, step 17's prompt and report (sections 6, 7, 9),
  `plan/later.md`, the scope notes and their synthesis, README,
  `docs/src/status.md`, the reports of steps 38 to 45 (outcomes and
  open items), `bench/TOOLS.md`, `comparison-gaps.md`, the handoff's
  open items, the Status log from 2026-10-30, the web design, the core
  rules, the client's `CLAUDE.md`, islands and content.
- [x] Twenty-five research sub-agents on Sonnet 5.5 at `medium`, one
  per area, each with the shared brief and an area brief; their notes
  under `plan/notes/research/next/`, each with its "Not verified"
  section; `order.md` by this session.
- [x] The claims the recommendations lean on read again here: GitHub's
  rename and transfer pages, the BI undecidability paper, Prusti's
  releases, Creusot's changelog, GitHub's custom-domain page, the
  client's greyed Ordinary button, `core/src/ordinary/` without
  interactive proving, the client's 452 problems, `.github/dependabot.yml`,
  the client's certificate export.
- [x] The report: the author's direction answered (each logic and
  method, what else, custom rules, the structure with the options
  compared and a recommendation, the web front end, the name with its
  touch points and the candidates checked, the order with releases),
  the ranked list naming every candidate of the notes with what it
  brings and to whom, cost, risk, prerequisites and its note,
  verification's feasibility, users, agents, maintenance, the scope
  updates, the decisions with a recommendation each, what the next
  sessions must know, the departures.
- [x] The fresh-context reviewer's round (Opus 5.5 at `high`): sixteen
  findings, each answered above; the notes and the report corrected.
- [x] No code changed, nothing built or run, no benchmark; nothing
  pushed, registered or renamed; no `gh`, no `git`. The plan's files
  are outside every automated check (`typos.toml` excludes `plan/`), so
  no check applies to this change.
- [x] Committed in the workspace (`rqwluyuy`, `olvzzyyt`, `kposwtwu`, and
  the ranked list's completion after the stop check: every candidate a
  row of its own, no blank cell, a sources index per candidate), `jj st`
  clean; the final message to
  `linlog-orchestrator`.
