# The web front end's plan

Written by step 32's first session (2026-10-19) from `plan/34-web.md`
and the design (`plan/web/design.md`, D1 to D60, fixed). It says which
session builds what, what version 1 is and leaves out, the bindings'
API as `linlog-web` offers it, and what the library lacks for the page.
The decisions are the design's; where this plan settles something the
design left open, it says so.

Names as of their day (2026-11-09): this plan was written when the
program was linlog. The bindings are the crate `deduxit-web` now, the
command `deduxit`, the rustdoc at `deduxit.github.io/deduxit/`, the
library's items under `deduxit::linear` (`deduxit::Settings` at the
root), and `about()` answers `{"deduxit", …}`.

## The sessions

Three sessions, each on Opus 5.5 at `high` (`plan/34-web.md`, "How the
step runs").

**32a, the plan and the bindings** (this repository alone): this
plan; the crate `linlog-web` (`web/`), its tests under wasm32 and its
flake checks; item 3's measurement. It builds, of the design:

- D28's half in this repository: the module built with the
  `wasm-bindgen` CLI at the version `Cargo.lock` pins (the flake check
  `web-bindgen` compares the two), `--target web`, `wasm-opt` after it,
  the stack raised with `-zstack-size`, the memory bound's default set
  by a tab's 256 MiB; the package `linlog-web` that the client's flake
  takes through its pinned input.
- D32's half in this repository: the notices of the Rust crates inside
  the module, collected by `cargo-about` into the package
  (`THIRD-PARTY-NOTICES.txt`).
- The values the page words itself: D11 and D15 (every refusal and
  verdict as the library's JSON), D12 (the hint's search on a copy of
  the goal), D14 (a goal's close and all goals' within one work
  budget), D16 (every export), D20 and D51 (a session and a sequent read
  back at their level), D59 (the fragment), D26 (the drawing's `Style`
  takes the theme's colours), D4 (the reason a sequent has no net, for
  the greyed tab).

**32b, the client's repository and version 1's core**: sets up
`/home/tux/Projects/own/linlog-prover.github.io` with jj (the default
of `plan/34-web.md` item 1, unless that session finds a reason against
it; creating it on GitHub and pushing are the planning session's). It
reads `~/Projects/own/website/docs/stack-spec.md` and `website-learn`'s
`CLAUDE.md`, `.claude/rules/` and `modules/` first (D37) and builds:

- the stack and its tooling: D27 (Astro 7, TypeScript 6, Solid 1.9, the
  app one island), D38 (Tailwind v4, the mockup's tokens in CSS), D39
  (Biome, Vitest with fast-check, Playwright), D40 (npm under Nix), D41
  (the dendritic flake, taking this repository's flake as a pinned
  input), D43's procedure for updates;
- D28's other half: the module in a module Worker, the request-id
  protocol, cancellation by `terminate()` and a fresh worker;
- the page of `linlog interact` with a mouse: D1, D2 (the default
  calculus, the labelled switches, the restart warning), D3, D5, D9,
  D10, D11, D13 (undo per branch, by the client's own step tree and
  replay), D14, D15, D16, D17, D18, D19, D22, D24 to D26, D33, D34,
  D35, D44 to D48 (the one-row top band, the dock at the goal, the
  bottom band by intent, the light and dark toggle, the dotted grid),
  D53 to D55, D58 (the cluster of switches; the two-sided presentation
  of classical derivations greyed with its reason), D59;
- what holds the work: D20 (the link), D29 (autosave), D50 (several
  proofs at once), D51 (import).

**32c, the finish**: hosting and the licence check (`plan/34-web.md`
items 5 and 6) and the rest of version 1:

- D31 (GitHub Pages at `linlog-prover.github.io`, the rustdoc staying
  at `/linlog`, a `404.html`, the CI of the sites), D42 (the security
  baseline as far as Pages allows, the CSP as a `<meta>` tag with
  `'wasm-unsafe-eval'`), D36 (no tracking, the line on GitHub's logs);
- D32's other half (Vite's `build.license` for the npm bundle, served
  with the package's `THIRD-PARTY-NOTICES.txt` and the font's
  `OFL.txt`), item 6's CC BY 4.0 for the written content, said in the
  client's README and on the pages that carry it;
- D49 (offline and installable: the service worker, scoped away from
  `/linlog/`, and the manifest), D52 (About), D7 (the examples list),
  D8 (the tour), D6 and D47 (the Learn toggle and its frame, with the
  place for the tutorial, practice and reference), D21 and D23 (the
  content's shape and where it lives);
- the repository's website field pointing to the client, on the
  author's word.

Learn's lessons (D56, D60) are step 43's, every sequent in them proved
or refuted by the command first.

## Version 1

`linlog interact` with a mouse (`plan/34-web.md` item 4), widened by the
design: a sequent in the field, the calculus's switches, the goals with
clickable formulas, the rules a formula takes, the split, undo per
branch and redo, a goal's close and all goals', the hint ladder, the
derivation drawn and, for a finished MLL or IMLL proof, its net, every
export, the link, autosave, several proofs, import, About, the examples,
the tour and Learn's frame.

Left out of version 1, each for a later step:

- the net editor (D4): linking literals pair by pair; the library's
  `ProofStructure::link` and `unlink` exist, the bindings do not offer
  them yet;
- the two-sided presentation of classical derivations (D58): a library
  feature (`plan/later.md`), its toggle greyed;
- ordinary propositional logic (D2, D57): the tab's choice between
  linear and ordinary is greyed; the bindings have its calls since
  step 49 (below), the page takes them in 49c;
- Learn's lessons (D56, D60) and the content of D21 beyond the examples
  list (the reasons, the prompts, the fading, the dead ends);
- a progress display during a long search: a call answers when it
  ends; the page shows that it is searching and can stop it.

## The bindings' API

`linlog-web` (`web/`, `publish = false`) is the library compiled to
`wasm32-unknown-unknown` without `parallel`, with every other feature.
Every call takes JSON text and returns JSON text, an SVG document or
bytes, and keeps nothing between calls: the page holds the session
document and the settings, so a worker stopped mid-call loses nothing.
A failed call throws a string, the error's JSON as the library writes
it (`{"code", "kind", "message", "setting", "details"}`, with formulas
written as text where a session has them); the page branches on `code`,
`kind` and `details.kind` and words the message itself.

`settings` is always the JSON of `linlog::Settings` read over the tab's
defaults: an empty text is the defaults, and a key the page gives
replaces the default there, an object key by key; a value that is not
an object where the defaults have one is refused. The calls (exported
under these names; `goal` and `position` are numbers, integers from 0
to 2³² − 1, anything else refused):

| call | answers |
|---|---|
| `about()` | `{"linlog", "level", "stack_bytes", "suites"}`: the library's version, the highest wire level it reads, and the suites, each `{"suite", "name", "calls", "parameters": [{"name", "values", "default"}], "settings", "term"}` (`linear` with the parameter `mode` and calls without prefix, `ordinary` with `logic` and the calls `ordinary_…`, `rules` with `calculus` and the calls `rules_…` and no `term`; `term` the name of the suite's call for the lambda-term) |
| `describe()` | the protocol every suite speaks: `about`'s answer with `calls` (the protocol's twelve names: `read`, `start`, `view`, `rules`, `candidates`, `apply`, `undo`, `close`, `hint`, `draw`, `export_as`, `certificate`), and each suite with `calls` (each `{"call", "export", "takes"}`, `export` `null` where the suite lacks the call), `extra` (its calls beyond the protocol's), `layouts` and `builds` (its methods, `natural` running the `rules` suite in NK, NJ or NM); `catalogue` is the library's `protocol::catalogue()`, every suite's kinds and operations |
| `ordinary_read(input, logic, settings)` | `{"kind": "sequent", "sequent", "text", "logic"}`, as `read_as` for the linear suite; refuses what `ordinary_start` refuses |
| `ordinary_certificate(outcome, settings)` | the Rocq certificate of an ordinary outcome: its derivation where valid, its countermodel where not |
| `defaults()` | the tab's settings, for the Settings pane |
| `read(input, settings)` | a typed sequent, a sequent's JSON (`seq json`) or a session (`save`): `{"kind": "sequent" or "session", "sequent", "text", "fragment": {"classical", "intuitionistic"}, "mode", "session"}`; a parse error carries the caret's place, a level above the module's is `unsupported_version` |
| `start(sequent, mode, settings)` | a new session: `{"session", "view", "inverted"}`; `mode` one of `classical`, `affine`, `mix`, `affine-mix`, `intuitionistic`, `intuitionistic-affine`; with `interactive.invertible` on, every invertible rule applied first, `inverted` the steps that took |
| `view(session, settings)` | `{"mode", "fragment", "complete", "steps", "goals": [{"goal", "formulas": [{"text", "occurrence", "side", "reading"}]}], "inferences", "invertible", "cuts", "net"}`: `reading`, in intuitionistic mode alone, the formula as the reading writes it in its position (a hypothesis `A ⊸ B`, whose `text` is `A ⊗ ~B`); `inferences[n]` is the goal the drawing's `i<n>` is, `invertible` the inferences a step of an invertible rule closed (`Interactive::is_invertible`), `cuts` each cut's `inference` and its `formula` as typed in the mode's syntax (what a replay cuts with: `start` leaves a sequent's cut formulas out), `net` the refusal that says why the sequent has no net, or `null`; in a session with quantifiers `opened`, each closed quantifier step's `inference` and the `term` it took (a `∀` step's eigenvariable by its letter, a `∃` step's witness as given, `null` for one left open, whatever filled it since): what a replay takes the step with, and by which a replay that renumbers eigenvariables maps their letters; and `forest`, the number of occurrences, below which a member is an occurrence and past which it is the entry `[occurrence, frame]` of the session's `instances.members` at its offset (a replay renumbers those) |
| `rules(session, goal, position, settings)` | `[{"rule", "split"}]`: the rules the formula takes by its connective and the mode, with `"witness": true` for `∃R` and `∀L` (`∃` one-sided), which take a witness |
| `candidates(session, goal, position, settings)` | `[{"rule", "status", "needs", "refusal"}]`: the same rules, each `applies`, `needs_input` with `needs` (`split` for `⊗` and Mix where some split is legal, `witness` for `∃R` and `∀L`), or `refused` with `refusal`, the error `apply` throws for the step without input (every other formula to the right premise, a witness open); legal steps only, never whether a goal can close; at most one `apply` per rule |
| `split_passes(session, goal, step, settings)` | `false` when the split cannot close |
| `apply(session, goal, step, settings)` | `{"session", "view", "opened", "inverted"}`, or the refusal with its reason; a step is `{"position", "rule", "left", "witness"}`, `left` the positions going to the left premise of a `⊗` or Mix, `witness` the term of a `∃R`/`∀L` step as text (the sequent's symbols, the session's eigenvariables by their letters), left out for a witness left open as `$1`; the eigenvariable condition refuses with `details.kind` `eigenvariable`, naming it; with `interactive.invertible` on, every invertible rule then applied to the goals it opened, `opened` those left open and `inverted` the steps taken (`inverted_refused` the bound that stopped them, if one did) |
| `invert_all(session, settings)` | `{"session", "view", "inverted"}`: every invertible rule applied to every open goal, whatever the settings say, as turning the setting on mid-session does |
| `undo(session, settings)` | `{"session", "view", "reopened"}` |
| `close(session, goal, settings)` | `{"session", "view", "outcome", "refused", "refutation_checked"}`: the search within `limits.work`, its proof grafted; `refused` the graft's refusal |
| `fill(session, goal, source, settings)` | `{"session", "view", "filled"}`: the checked proof of `source`, another finished session of an equal sequent, carried over to the goal (its formulas matched up to order) and grafted, checked in this session's mode; `sequent_mismatch` with `details.proof` and `details.goal` (the formulas only on each side), `open_goals` for an unfinished source, `invalid_proof` for a rule this mode does not allow |
| `close_all(session, settings)` | `{"session", "view", "results": [{"goal", "outcome", "refused", "error", "work"}]}`: every goal's search within one budget of `limits.work` for all of them, a goal it runs out on `work_limit` |
| `hint(session, goal, settings)` | `{"outcome", "steps", "invertible"}`: the search on the goal, the session unchanged, and the first steps of its proof (the structural rules at its root and the first rule that is not one), each for `apply` on the goal the one before opened, and whether the last applies an invertible rule; following the hints proves the goal. In a session with quantifiers the goal is closed in a copy of the session (`Interactive::close`, the goal written as a sequent of its own) and the steps are read off the copy, an `∃R`/`∀L` step with the witness the search chose where the session reads it, else open |
| `proof(session, settings)` | the checked proof of a finished session |
| `term(session, settings)` | the lambda-term of the session's proof, read back from its derivation: the term's JSON (`calculus`, `atoms`, `types`, `variables`, `context`, `goal`, `nodes`, `root`) with its `text` and `judgement` written as the settings' `lambda` says (`{"notation": "unicode" or "ascii", "types"}`), as `prove --lambda --format json` writes it; or `{"error": …}`, the library's error: `no_term` with the reason (a classical proof, Mix, the cyclic and Lambek modes, quantifiers, open goals), or the refusal of `limits.derivation_bytes` |
| `prove(sequent, mode, settings)` | `{"outcome", "refutation_checked"}`: the verdict for the status light, `refutation_checked` what `Disproof::check` said |
| `draw(session, settings)` | the session's derivation as SVG, open goals as leaves, `i<n>` and `i<n>-<p>` on every conclusion and formula |
| `draw_net(session, settings)` | the net of a finished session's proof as SVG (`styles.svg.essential` for an essential net) |
| `cuts(session, proof, settings)` | `{"proof", "cuts": [{"node", "formula", "inferences"}], "next", "drawing", "refused"}`: a proof of the finished session's sequent (`""` for the session's own) with its cuts, the drawing's `i<n>` that show each, the strategy's next cut, and its derivation drawn as `draw` draws the session's (`null` with `refused` past the bound) |
| `cut_step(session, proof, at, settings)` | `{"step": {"cut", "formula", "case", "name", "above", "highlight": {"cut", "above"}}, …}` and the proof made as `cuts` answers: one step on the cut `at`, or the strategy's for `undefined`; `highlight` the ids in the drawing of the proof given; `no_cut` for a node that is no cut |
| `eliminate(session, proof, settings)` | `{"steps": [{…, "drawing"}], "end", "reason", "undrawn", …}` and the proof reached: every step by the strategy within `cut.steps` and `cut.nodes`, each with the drawing of the proof it was taken on while `cut.drawn_bytes` holds them (`undrawn` the first step left out), `end` `cut_free` or `refused` with the bound as `reason` |
| `net_cuts(session, net, settings)` | `{"net", "boxes", "cuts": [{"ends", "id", "formula"}], "next", "drawing"}`: a net as these calls write it, or the net of a proof (`""` for the session's), with boxes for exponentials and for a proof that cuts an occurrence twice; `id` the cut link's bar `c<x>-<y>`; a net with cut links drawn plain |
| `net_cut_step(session, net, at, settings)` | as `cut_step` on a net, `at` a vertex at either end of a cut link, with `marked`, the net before drawn with the step's marks, and `highlight` the cut link and the vertex (`o<n>`, `v<k>`) or box (`b<k>`) it acted on |
| `net_eliminate(session, net, settings)` | as `eliminate` on a net, each step's drawing the net it was taken on with its marks |
| `export_as(session, format, settings)` (`export` is reserved in JavaScript) | `text`, `latex`, `typst`, `svg` (the session's tree, open goals included) or `rocq` (the certificate of the finished proof) |
| `render(svg, format, font, settings)` | `png` or `pdf` bytes of a drawing, rendered with the font's bytes the page passes; a PDF needs `styles.pdf.date` from the page's clock. The one call of the renderers' module (`deduxit_web_render.js`, the crate `deduxit-web-render`), which the page loads on its first PNG or PDF export |
| `certificate(document, settings)` | the Rocq certificate of non-derivability of a disproof, or of an unprovable outcome |
| `ordinary_start(sequent, logic, settings)` | a session of LK or LJ: `{"session", "view"}`; `logic` one of `classical`, `intuitionistic`, `minimal`; the sequent typed in ordinary syntax or as the ordinary sequent's JSON; `Γ ⊢` in LJ as `Γ ⊢ ⊥` |
| `ordinary_view(session, settings)` | `{"logic", "calculus", "complete", "steps", "goals": [{"goal", "left", "right"}], "inferences", "opened"}`: each goal's formulas as text by side, in the order of their positions; `inferences` maps a drawing's `i<n>` to its goal under `view.structural`; `opened`, where a quantifier step was taken, each such step's `inference` and the `term` it took as text the session's `witness` reads (`null` for a witness left open), so that a replay gives each witness again |
| `ordinary_rules(session, goal, side, position, settings)` | `[{"rule", "split"}]`: the rules of LK or LJ the formula takes, `side` `left` or `right`; `split` for a rule with two premises, which share the context unless a step splits it |
| `ordinary_candidates(session, goal, side, position, settings)` | `[{"rule", "status", "needs", "refusal"}]`: the axiom and every rule of the logic that acts on the formula, each `applies`, `needs_input` with `needs` (`context` for two premises, `witness` for `∀L` and `∃R`) or `refused` with `refusal`, the error `apply` throws for the step with a shared context and a witness left open |
| `ordinary_apply(session, goal, step, settings)` | `{"session", "view", "opened"}`, or the refusal (`code` `step`, its `kind` and reason); a step is `{"side", "position", "rule", "split": {"left", "right"}}`, the rule by its name (`→L`), `split` the positions going to the first premise, left out to share the context |
| `ordinary_undo(session, settings)` | `{"session", "view", "reopened"}`: a step or a graft retracted whole |
| `ordinary_close(session, goal, settings)` | `{"session", "view", "outcome", "refused"}`: the goal decided through `ordinary.translation`, its derivation grafted; `outcome` the ordinary outcome (`valid`, `not_valid`, `unknown` with its `reason`); a `not_valid` goal's `countermodel` (`{"sequent", "model"}`), the SAT solver's model of the goal; a first-order goal decided by the first-order methods (a proof search, then a finite countermodel), its eigenvariables constants there: a derivation found grafted, each inference checked first, or a `not_valid` outcome's `structure` (`{"sequent", "structure"}`) checked; a goal holding an open witness never `not_valid` |
| `ordinary_hint(session, goal, settings)` | `{"outcome", "steps"}`: the goal decided, the session unchanged, `steps` the first steps of the derivation found (its weakenings and contractions at the root and the first rule that is not one), each on the goal the one before opened, as `ordinary_apply` takes them, a `∀L` or `∃R` with its `witness` as text where the goal holds the term; the outcome's `countermodel` and `structure` as `ordinary_close` gives them |
| `ordinary_term(session, settings)` | as `term`, the derivation of LJ read back as a simply typed term (`calculus` `simple`); `no_term` for a derivation of LK or with open goals |
| `ordinary_draw(session, settings)` | the derivation so far as SVG, open goals as bare leaves, the structural rules as `view.structural` draws them, `i<n>` on every conclusion |
| `ordinary_export(session, format, settings)` | `text`, `latex`, `typst`, `svg` (the derivation so far) or `rocq` (the certificate of the finished derivation over `Prop`, checked first); PNG and PDF by `render` |
| `rules_calculus(name)` | the rule file of a built-in calculus, by the command's names (`LK`, `LJ`, `LM`, `LL`, `tableau`, `NJ`, `NK`, `NM` and their variants), as JSON text to start one's own from |
| `rules_read(calculus, settings)` | a rule file (JSON, or a built-in name) checked: `{"name", "description", "zones": [{"name", "mark"}], "rules"}`; `invalid_calculus` with the reason |
| `rules_start(calculus, goal, assumptions, settings)` | a session in the calculus: `{"session", "view"}`; the goal in the calculus's syntax, `assumptions` a JSON array of sequents or `""` |
| `rules_view(session, settings)` | a saved session read, every step checked again: `{"calculus", "sequent", "assumptions", "finished", "steps", "goals": [{"goal", "sequent", "zones", "hypotheses", "lines"}], "inferences"}`, `zones` each zone's members as text in the order of their numbers, `hypotheses` those in force, `lines` the lines in force `[{"line", "fitch", "sequent"}]`, `line` the number the calls take (from 1, fixed while the line stands) and `fitch` its number in the session's Fitch proof (`null` where none writes it), `inferences[n]` the session node the tree's `i<n>` is (a line cited twice is two inferences; empty when the bounds refuse the drawing) |
| `rules_rules(session, goal, settings)` | `[{"rule", "at", "needs"}]`: each rule at each choice of the members it acts on that the checker takes, or what it needs given |
| `rules_candidates(session, goal, settings)` | `[{"rule", "at", "status", "needs", "refusal"}]`: every rule of the calculus, at each choice of members where it is legal (`applies`, or `needs_input` with the names to give), and a rule no choice makes legal once with `at` empty, `refused` and `refusal` (`step_refused`, its `details.fault`) |
| `rules_apply(session, goal, step, settings)` | `{"session", "view", "opened"}`; a step is `{"rule", "at", "given": {NAME: TEXT}}`; `step_needs` with `details.needs`, `step_refused` with the reason |
| `rules_derive(session, goal, step, settings)` | `{"session", "view", "line"}`: a forward step at the goal, `{"rule", "lines": [N…], "given": {NAME: TEXT}}`, from lines in force to a new line numbered `line`; a rule that discharges is `step_refused` (it applies backwards) |
| `rules_cite(session, goal, line, settings)` | `{"session", "view"}`: the goal closed by the line numbered `line`, in force there and equal to it; `step_refused` with the reason otherwise |
| `rules_assume(session, goal, settings)` | `{"session", "view"}`: the goal closed by a hypothesis in force |
| `rules_undo(session, settings)` | `{"session", "view", "reopened", "removed"}`: the goal reopened, or the line a forward step had derived |
| `rules_check(session, settings)` | `{"calculus", "verdict": "valid in the calculus given", "derivation"}` for a finished session; `open_goals` otherwise |
| `rules_draw(session, settings)`, `rules_export(session, format, settings)` | the derivation so far as SVG (`i<n>` on every inference), or in `latex` or `typst`; with `view.layout` `fitch` its Fitch proof (`l<n>` on line `n`'s formula, `styles.fitch` its text), `not_fitch` with the reason where none writes it |
| `ordinary_model_drawing(outcome, settings)` | the countermodel of an ordinary outcome (`ordinary_close`'s or `ordinary_hint`'s, or `prove --format json`'s) drawn as SVG, checked first: worlds as boxes with their atoms, arrows up, the model's world outlined, ids `w<n>` on each world's box; `no_countermodel` for an outcome without one |
| `ordinary_structure(structure, sequent, logic, settings)` | a finite structure the page gives (`{"size", "constants", "functions", "predicates"}`, the library's `Structure`) checked against a first-order sequent: `{"refutes", "reason", "left": [{"formula", "true"}], "right": […]}`, `refutes` by `FiniteCountermodel::check` with its `reason` (the first formula wrong) where it does not, and each formula's text and truth, evaluated as a sequent of that formula alone (`Structure::refutes`); `invalid_structure` for a structure that cannot interpret the sequent |
| `ordinary_model_certificate(outcome, settings)` | the Rocq certificate of that countermodel, in `styles.rocq` (`form` `standalone` for a file): `~ lj` for deduxit's Rocq library in intuitionistic and minimal logic, over `Prop` in classical logic |
| `ordinary_tableau_review(text, sequent, settings)` | a signed tableau of classical logic written by hand, as text, reviewed step by step against the built-in rule file `tableau` (and against `sequent` unless it is `""`): `{"standing": "closed"|"open"|"unfinished"|"wrong", "sequent", "text_line", "line", "message", "countermodel"}`; a text that is no tableau is a parse failure placed in it |
| `ordinary_normalize(sequent, options, settings)` | a classical sequent rewritten into a normal form (`options` `{"form"}`: `nnf`, `cnf`, `dnf`, `tseitin`, `plaisted-greenbaum`), every step with its law, the promise checked: the level-21 document |
| `ordinary_resolve(sequent, options, settings)` | the clauses of a classical sequent's negation decided by saturation or DPLL (`options` `{"form", "method", "clauses", "steps"}`): the level-22 document, its refutation or model checked, DPLL's `trace` |
| `ordinary_clauses(sequent, form, settings)` | a refutation by hand started: the clauses of the sequent's negation, no resolvent |
| `ordinary_resolve_step(refutation, first, second, atom, settings)` | a resolvent added to a refutation by hand after its steps are checked: `{"refutation", "added", "finished"}`, a step the clauses do not allow refused (`step_refused`) |
| `ordinary_refutation_check(refutation, settings)` | a refutation by hand checked step by step: `{"finished"}` |
| `modal_read(text, system, settings)` | a modal statement: the sequent's document with `system` (one of `about`'s fifteen names, or K and the axioms' letters) and `text` as typed |
| `modal_decide(statement, settings)` | the modal outcome (wire level 20): `verdict`, `system`, `statistics`, `sequent`, and `derivation` (a `rules` derivation in `G3` and the system's name) or `countermodel.model`; `unknown` with `reason` at a bound |
| `modal_countermodel(outcome, settings)` | a not-valid outcome's Kripke model (level 13), each world's `arrows` the worlds it sees, `world` where the sequent fails; `invalid_option` for an outcome without one |
| `modal_evaluate(statement, model, world, settings)` | `{"system", "world"?, "frame"?: {"condition", "worlds"}, "holds"?, "failing"?, "values"?: [{"formula", "true", "witness"?}]}`: the frame condition the model fails, or whether the sequent holds at `world` (`undefined`: every world) and each compound formula's value where it fails |
| `modal_draw(outcome, settings)`, `modal_export(outcome, format, settings)` | a valid outcome's derivation as SVG, or in `latex` or `typst`; a not-valid outcome's countermodel as SVG (`w<n>` on each world's box, an arrow to each world it sees, loops and both ways drawn) |
| `modal_model_drawing(statement, model, world, settings)` | a model the visitor gave, drawn as the countermodel of the statement at `world` after checking it is one; `invalid_model` naming the frame condition or the formula otherwise |
| `modal_calculus(statement, settings)` | the system's labelled calculus as a rule file, which `rules_start` takes |
| `lambda_read(text, calculus, settings)` | a lambda-term's statement: the term's document (wire level 23) with `text` as typed; `calculus` `simple`, `linear` or `affine`; a parse error or `ill_typed` thrown with its place (`span_utf16`) |
| `lambda_check(term, settings)` | the term checked: its document with `typing` (a type per node) and `judgement`; `ill_typed` thrown with the reason and the node |
| `lambda_draw(typing, settings)`, `lambda_export(typing, format, settings)` | the checked term's typing derivation as SVG (`i<n>` per inference, a judgement `Γ ⊢ M : A` each), or in `latex` or `typst` |
| `lambda_read_type(text, settings)` | a type: an ordinary statement in intuitionistic logic, its formula the type and the formulas before `|-` the free variables' types |
| `lambda_inhabit(type, settings)`, `lambda_enumerate(type, options, settings)` | a term of the type or that it has none (the protocol's `lambda.inhabitation`); the type's terms listed and counted (`lambda.inhabitants`), `options` `{"depth", "max_terms"}`; every term with its `text` and `judgement` added, written as the settings' `lambda` says |
| `workspace_statement(suite, text, parameter, settings)` | the value a typed sequent is in the workspace: its document with `logic` (`ordinary`) or `mode` (`linear`) and the text as typed |
| `workspace_open(document, settings)` | a workspace from a link or a file, read as a claim (instant runs made again, a search's check `recorded`) and given the tab's bounds (`rebound`); the document the page then holds |
| `workspace_edit(document, action, work, settings)` | `{"workspace", "made"}` (in every answer `workspace` is the document's text as a JSON string, to hand back as it is): one action `{"name", "edits"}`, one entry of the log, each edit `add`, `set`, `connect`, `parameters`, `settings`, `restart`, `work`, `detach` or `remove`, a node its number or `{"made": K}`; a proof's or a copy's instant inputs are made in the same action; `made` per edit the node made, or the nodes removed |
| `workspace_undo(document, work, settings)`, `workspace_redo(…)` | `{"workspace", "name", "sessions"}`: the action taken back or brought back (`null` for none) and each proof's session it changed, by node |
| `workspace_compute(document, node, what, work, settings)`, `workspace_accept(document, computed, work, settings)` | a node's run (`what`: `output` or `check`) made apart, for the worker, and put in where the node still asks for exactly it: `{"workspace", "accepted"}`; a run the stop ended is never made |
| `workspace_run(document, nodes, what, work, settings)` | `{"workspace", "ran"}`: what the nodes need, in order |
| `workspace_view(document, work, settings)` | `{"nodes": [{"node", "kind", "value", "summary", "operation", "label", "cost", "class", "from", "parameters", "state": {"state", "cause", "node"}, "end", "check", "column"}], "undo", "redo"}` |
| `workspace_pack(document, nodes, work, settings)` | what a link holds: the nodes and what they are made from, their recipes and sessions, no runs and no log |

The session's JSON itself is the session export (D16) and what the link
carries (D20).

**The tab's defaults** (`web/src/tab.rs`, `tab::defaults()`), the
command's but for:

| key | tab | command | why |
|---|---|---|---|
| `limits.memory_bytes` | 96 MiB | 1 GiB | a tab on iOS gets about 256 MiB of wasm memory; the rest holds what the bound does not count |
| `limits.occurrences` | 1 000 000 | 50 000 000 | a forest of 25 MB, copied by a session, a proof and a net |
| `limits.terms` | 1 000 000 | 50 000 000 | as many first-order terms as occurrences, the library's rule: a session's table no larger than its forest |
| `search.witness_depth`, `search.instances` | 64, 2¹⁶ | 64, 2¹⁶ | the library's: a search's witnesses and instances, charged against `limits.memory_bytes` |
| `limits.derivation_bytes` | 4 MiB | 64 MiB | a drawing peaks at several times its estimate |
| `limits.work` | 2²⁶ | none | the page's budget, about a second (68 million units a second under wasm): no clock in the module |
| `limits.recursion_depth` | 384 | 2 048 | the host's stack, not the module's, runs out (below) |
| `search.copies` | none | none | the work bound ends the deepening |
| `search.jobs` | 1 | auto | one thread |
| `search.classical_work` | 2²² | 2²⁶ | within a tab's budget |
| `clock` | no limits | 2 s | the page applies a time by stopping its worker |
| `styles.svg.ids`, `.description` | on | off | the page's buttons over the formulas, the screen reader's description |
| `cut.drawn_bytes` | 16 MiB | 256 MiB | the page holds every step's drawing of an elimination: four at the tab's bound on one, as the command's is four at its own |
| `limits.workspace.bytes` | 2 MiB | 64 MiB | the page keeps the workspace in the browser's storage; three linked tabs are about 17 KB |

## What the library lacks for the page

Each item of `plan/34-web.md`'s "What the bindings must offer the page",
checked against the library on 2026-10-19:

| the page needs | offered by | missing, and where it goes |
|---|---|---|
| the rules a formula takes, `split_passes` | `rules`, `split_passes` (`Interactive::rules`, `split_passes`) | nothing |
| each refusal's reason as a value | `apply`'s thrown error: `StepError`'s JSON in `details`, by `kind` with its positions | nothing |
| a search on a copy of the goal for the hint ladder | `hint` | the library has no "first steps of a proof of a goal"; the bindings read them off the derivation of the goal's proof (`calls::first_steps`); a library call would serve other front ends, a later addition to `Interactive` |
| a goal's close and all goals' within a work budget | `close`, `close_all` (`limits.work`, one budget for all) | nothing |
| the verdicts as `Outcome` and `Refutation` with `Disproof::check` | `prove`, `close` (`refutation_checked`) | nothing |
| every export: SVG, PNG and PDF from the page's font, LaTeX, Typst, Rocq, the certificates of non-derivability, the session as JSON | `draw`, `draw_net`, `export`, `render`, `certificate`, the session document | the certificate of non-validity of ordinary logic (`rocq::write_not_valid`) waits for ordinary logic on the page |
| the session document read back at its level, a sequent as `seq json` writes it | `read`, every call's `session` | nothing |
| the detected fragment | `read`, `view` | nothing |
| the drawing's `Style` with the theme's colours | `styles.svg` in the settings (`text`, `line`, `par`, `link`, `highlight`, `background`) | nothing |
| a step and a rule offered, as JSON | the bindings' own forms, `{"position", "rule", "left"}` and `{"rule", "split"}` | `Step` and `Applicable` have no wire form in the library; when one comes it is at level 1, and the bindings take it |
| ordinary logic (D2, D57, after version 1) | the `ordinary_…` calls over `ordinary::Interactive`, the forms at wire level 8 | the certificate of non-validity (`rocq::write_not_valid`) for a goal that is not valid |
| the two-sided presentation of a classical derivation (D58) | none | a presentation pass over `Derivation` (`plan/later.md`), after step 32 |
| the net editor (D4, after version 1) | none | the bindings' calls over `ProofStructure::link` and `unlink`; nothing in the library |

## Item 3: the two searches of the default bias under wasm

See `plan/reports/34-web.md`, "Session 32a", for the measurement and
its answer.
