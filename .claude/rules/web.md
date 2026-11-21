---
paths:
  - "web/**"
  - "web-render/**"
  - "web-tab/**"
  - "modules/web.nix"
---

# deduxit-web: the WebAssembly bindings

Loaded when a file of the bindings (`web/`) or their flake module is read
or changed. The plan of the web front end is `plan/web/README.md`; the
client is a repository of its own, which takes this repository's flake
as a pinned input and builds nothing of Rust.

## Layout

`web/src/calls.rs` the calls, plain Rust over JSON text, each reading
its documents and the settings and writing what the page shows;
`lib.rs` the `#[wasm_bindgen]` exports, one line each over `calls`,
named as in Rust but `export`, which JavaScript reserves (`export_as`);
`tab.rs` the tab's settings (`deduxit-web-tab`'s) and the module's
stack; `build.rs` the stack.
`web-tab/` (`deduxit-web-tab`, a library alone) the tab's defaults and
the overlay of the page's settings on them (`lib.rs`) and the error's
JSON a call throws (`failure.rs`), which both modules read.
`web-render/` (`deduxit-web-render`) the PNG and PDF renderers, a
module of their own with the one call `render`; `tests/render.rs` its
tests, drawing with the bindings (a dev-dependency).
`web/tests/calls.rs` one test per behaviour the page relies on, each
`#[wasm_bindgen_test(unsupported = test)]`, so the same test runs
natively (`cargo test`, the gate) and under Node (`web-wasm-tests`).
`about.toml` and `about.hbs` are cargo-about's configuration and template
for `THIRD-PARTY-NOTICES.txt`.

## What the code relies on

- **Nothing is kept between calls.** The page holds the session
  document and the settings and sends them with every call; a worker
  the page stops (its only cancel: the module reads no clock, and every
  call's stop is `never`) loses nothing. A handle kept in the module
  would save the read of the session per call (linear, and checked by
  `wire::upgrade` as a loaded session always is), and would have to be
  rebuilt after every stop.
- **The settings are read over the tab's** (`tab::read`): the page's
  JSON is laid over `tab::defaults()` key by key, objects merged and
  every other value replaced, then read as `Settings` with its
  `deny_unknown_fields`. So `{}` is the tab's settings, never the
  command's (whose memory bound is a gibibyte), and a misspelt key is
  refused as the command refuses it.
- **Errors are the library's JSON**, `Error`'s written form or
  `describe`'s with the session's formulas; the bindings' own refusals
  (`Failure::json`, `Failure::refused`) take the shape and a `code` the
  library uses (`json`, `unknown_name`). A new call returns `Failure`,
  never a message of its own.
- **A goal formula's `reading`** (intuitionistic mode alone, absent in
  classical mode) is `Reading::formula(o)`, the crate's two-sided printer
  for one occurrence in its position, and in intuitionistic mode `text`
  is the same (`Interactive::formula` writes under the session's
  reading): no text of an intuitionistic sequent is one-sided, and a
  typed intuitionistic sequent is read in ILL's syntax alone, so the
  one-sided text could not be read back there. It is a key of a call's
  answer, no stored document, so no wire level (`wire.rs`: a key an older
  reader may ignore raises none).
- **A typed sequent of an intuitionistic mode** (commutative; the
  Lambek calculus keeps its own reading) is read by `sequent_in`/`ill_of`
  as the settings' `input.intuitionistic` says, strict by default
  (`IllSequent::parse_within`); JSON may be the intuitionistic sequent's
  document (told by its `hypotheses` key) or a one-sided sequent read by
  `IllSequent::read_one_sided`. `read` has no mode and stays as it was;
  `read_as(input, mode, settings)` is `read` for a mode, its `text`
  two-sided, and is what the page calls for an intuitionistic proof.
  `start` and `prove` read their sequent the same way.
- **A typed sequent of a commutative classical mode is read as written**
  in `start` (`ClassicalSequent::parse_within`, `Interactive::of_classical`),
  so the session carries its record (`"written"`) from the first call and
  the page's "Two-sided" switch can turn at any point; `export` and
  `draw` draw it two-sided when `view.classical` is `"two"`, a session
  without a record (JSON) through `ClassicalSequent::read_one_sided`.
- **An intuitionistic net is drawn essential** (`draw_net` sets
  `styles.svg.essential` in an intuitionistic mode): the plain drawing of
  the one-sided net writes classical formulas.
- `web/tests/calls.rs`'s `no_intuitionistic_text_writes_a_classical_symbol`
  is the bindings' share of the property that no intuitionistic output
  has a classical symbol (the library's and the command's are their
  own); a new call that writes text of a sequent joins it.
- **`Step` and `Applicable` have no wire form in the library**: `StepForm`
  (`{"position", "rule", "left"}`) and `Offered` (`{"rule", "split"}`)
  are the bindings', the rule by its `Named` name. When the library
  gives them one (at level 1), the bindings take it.
- **A hint is the first steps of the goal's proof**: `hint` runs
  `prove_goal` on the goal (the session untouched) and reads the
  derivation of that proof alone (never the session's, which a large tree
  would have refused), from its root up to and including the first
  inference that is not structural (`first_steps`): a contraction at the
  root comes with the rule that uses both copies, since a search on the
  contracted goal may weaken the copy again, and hints that stop at the
  contraction contract and weaken in turn forever (a reviewer's finding).
  Each step is the principal's position, and for `⊗` the left premise's
  members less the left subformula, for Mix the left premise's members,
  matched to positions of its goal; the goal of a later step is the one
  the step before opened, whose members are the premise's in the same
  order. `following_the_hints_proves_a_sequent` pins it.
- **`close_all`'s budget is the searches' work**: each goal is closed
  with what the searches before it left of `limits.work` as its own, so
  the library itself says `work_limit` where the budget ran out (a stop
  that fires would say `stopped`, which the page cannot tell from a
  cancel). What a search spent is its `statistics.work`; the polls are
  no measure, since work between the last poll and the end is not
  reported. The check of each proof has the same remainder as its own
  bound, and the refuter its `classical_work`.
- **The integers at the boundary**: a goal and a position come from
  JavaScript as numbers (`f64`), which `calls::index` takes only as an
  integer from 0 to `u32::MAX`: as a `u32` parameter wasm-bindgen would
  take any number modulo 2³² (`2 ** 32 + 3` as 3, `1.5` as 1, `NaN` as
  0). A `u32` widens to `usize` on every target. A position inside a
  step's JSON is a `usize` read by serde_json, which refuses a number past
  `usize::MAX` on wasm32 rather than wrapping it
  (`a_position_past_32_bits_is_refused`). Every count the library writes
  is a `u64`, exact in JavaScript to 2⁵³ (`wire`'s rule).
- **The settings' overlay refuses a non-object where the tab has an
  object, but for a string**: serde reads an array as a struct's fields
  in order and fills the rest from `Default`, the command's values, so
  `[]` would have been the command's settings, a gibibyte of memory and
  a recursion depth the host's stack does not hold. A string is a name
  an option reads in place of its fields (`styles.svg.font`:
  `"stix-two"`, `a_font_is_chosen_by_its_name`), or is refused by serde.
- **`render` takes one font's file**: the page passes the file of the
  font its export's `styles.svg.font` names, and no other font sets the
  drawing's text (`an_export_is_set_in_the_font_the_settings_name`).
- **The renderers are a module of their own** (`deduxit-web-render`),
  which the page loads on its first PNG or PDF export: resvg, krilla and
  their fonts', images' and filters' code were 1.48 MB of the 3.91 MB
  every visitor's worker loaded (at gzip, 2026-11-21). The boundary is
  narrow: an SVG text, a font's bytes and the page's settings in, bytes
  out. The settings are read over the same tab's defaults
  (`deduxit_web_tab::read`, so `limits.memory_bytes` is the tab's 96 MiB
  there too) and a failure is the same JSON. `Settings` refuses an
  unknown key, and some of its fields and variants are behind a feature
  (`interactive`, `ordinary.method` `sat`), so `deduxit-web-tab` enables
  every feature of the bindings' but `png` and `pdf`: both modules accept
  the same keys (`a_feature_s_keys_are_read`, which only a run of that
  crate alone can fail, since a joint test build unifies the features:
  `web-wasm-tests` runs it so, in a cargo of its own).
  The tab and the error are a
  crate both depend on: a dependency on `deduxit-web` would link every
  `#[wasm_bindgen]` export of its into the renderers' cdylib, and the
  renderers in the bindings would put resvg and krilla back. The
  library's features of each module are its own (`png` and `pdf` only in
  the renderers'), so the package builds each by a cargo of its own: the
  library's `Styles` and `Error` grow with the features, and a joint
  build would compile both modules with all of them. Its stack is the
  bindings' 8 MiB (`web-render/build.rs`), which it had inside them.
- **The stack**: `build.rs` links the module with an 8 MiB stack
  (`-zstack-size`, read back as `tab::STACK_BYTES`), but the host's own
  stack, which no page can set (about 1 MiB in Node and in a browser),
  runs out first: each wasm frame is a native frame too. Measured in Node
  on the deepest levels, a Mix of pairs: the optimized module reached
  768 levels and not 1 024, the debug one 512 and not 768. Hence
  `tab::RECURSION_DEPTH` 384, half the optimized depth, and
  `a_search_at_the_recursion_limit_stops_there` runs the three paths
  with the largest levels (copies of a `!`, nested `&`, a Mix of pairs)
  at the limit under Node, in the debug build the check uses. A change
  that grows a level's frames shows there first.
- **The invertible rules** (`interactive.invertible`, off in the tab's
  settings as in the library's): `start` applies them to the root and
  `apply` to the goals its step opened, `opened` then the goals they
  left open and `inverted` how many steps they took; `invert_all`
  applies them whatever the settings say, which is what turning the
  toggle on mid-session needs. A run is bounded by the smaller of
  `limits.memory_bytes` and `limits.derivation_bytes` (`inverting`), so
  that it never makes a session the page could not draw. The view's
  `invertible` lists every closed inference whose rule is invertible on
  its conclusion, whoever took it (the session does not record who):
  the page's undo with the toggle on goes past them to the nearest step
  that is not one. A replay must run with the setting off, or every
  replayed step would apply the invertible rules again and the recorded
  ones would find their goals closed. A hint's `invertible` is the
  library's judgement of its last step, for the ladder's first rung.
- **`unsafe_code` stays forbidden**: the lint does not see the unsafe
  code `#[wasm_bindgen]` generates, so the workspace's `forbid` holds for
  the bindings too.

- **Nets**: `view`'s `net` and `draw_net` gate on `nets_exist`, which
  takes `MellStructure::exist` for a sequent with exponentials, so a
  finished MELL proof, classical or intuitionistic, draws its net with
  boxes (`svg::mell_net`, ids `o<n>`, `v<k>`, `b<k>`, `l<x>-<y>`; with
  `styles.svg.essential` an intuitionistic one as its essential net with
  boxes, the same ids); an affine one is refused with `no_nets`.
- **A net's look is `styles.svg.net_look`**, read over the tab's like
  every setting: the name `"textbook"` or `"deduxit"` (a string, which the
  overlay lets through where the tab has an object, as for the font), or
  an object of `premises`, `conclusions` and `why_not`, each left out
  keeping deduxit's (`a_proof_with_exponentials_draws_its_net_with_boxes`
  checks both forms). The tab's default is deduxit's look; nothing in
  `tab::defaults` sets it. Under the rules a compact `?` vertex whose
  one premise is a wire has no circle, so no element carries its id.

- **`cut(session, goal, formula, left, settings)`** reads the cut
  formula's text with `CutFormula::parse_within` in the session's mode
  and the settings' `input`, `left` a JSON array of positions, and
  answers as `apply` does (the invertible steps after it under the
  setting); `no_intuitionistic_text_writes_a_classical_symbol` cuts every
  session on its goal before closing it.

- **A session's cuts replay from the view's `cuts`**: each cut's
  inference and its formula as typed in the mode's syntax (the reading's
  text in an intuitionistic mode, `Interactive::formula_as_written` in a
  classical session that keeps its record, else one-sided), which `cut`
  reads again; `start` on the session's sequent leaves its cut formulas
  out (`Interactive::within`), so the replayed cut adds the same terms
  and the same occurrences (`a_cut_opens_two_goals`).
- **Cut elimination** (`cuts`, `cut_step`, `eliminate`; `net_cuts`,
  `net_cut_step`, `net_eliminate`) takes the finished session and the
  proof or net the page holds, `""` for the session's own: the session
  gives the mode, and for a classical proof drawn as written its
  record, which every proof of the elimination shares since a step
  keeps the forest (`proof_in` refuses a proof of another sequent with
  `foreign_proof`, as the record is indexed by its occurrences). Undo is
  the page's: it keeps the proof or net before each step, and a step of
  `eliminate` is taken again by `eliminate` with `cut.steps` one fewer,
  the strategy being deterministic.
- **A cut's ids in a derivation's drawing** come from
  `Inference::node`, the proof node an inference shows, matched against
  the cut's node (`Drawn::showing`); in a two-sided classical drawing
  through `TwoSided::shows`, a negation step showing no node. A shared
  cut node is several inferences. Node ids do not carry over from a
  proof to the next, so the ids of a step are those of the drawing of
  the proof it was taken on.
- **The net of an elimination** follows the command's `net_of`
  (`Net::of`): with boxes for exponentials and where `ProofStructure`
  refuses with `Placement::Cut`, which a session's own proof never
  makes (each cut adds a cut formula) but a proof an elimination on
  terms made may: hence the net calls take a proof's document too. In
  an intuitionistic mode a net with cut links is drawn under the
  reading as `draw_net` draws a cut-free one (the library draws a cut
  link's two trees as an output and an input one), so no step writes a
  classical symbol: `no_intuitionistic_text_writes_a_classical_symbol`
  checks every step's drawing on the net too. A MELL step renumbers the vertices, so a net's ids are those of the net
  the step was taken on, and the vertex ids of `id` follow
  `svg::mell_net`'s rule (`o<n>` for an occurrence's only vertex, `v<k>`
  otherwise, `b<k>` for the box a box entered).
- **The drawings of `eliminate` and `net_eliminate`** are bounded by
  `cut.drawn_bytes` together (`Drawings`), the command's rule: none
  after the first that would pass it, `undrawn` naming that step. A
  drawing its own bound refuses is `null` with `refused`, and the steps
  go on drawn; the drawing of the proof or net reached is the view's,
  outside the bound, as the command writes `FILE` anyway.

- **First-order sessions**: a step's `witness` is text the session
  reads (`Interactive::witness`, the session's letters for its
  eigenvariables), left out for an open one; `rules` writes `"witness":
  true` only where a rule takes one, so propositional answers are as
  before. A goal formula's `reading` is the session's text (the reading
  of the occurrence alone would print an instance's open formula,
  `p(#0)`), and a cut's formula of the elimination calls an instance's
  text (`member_in`, `Proof::formula`).
- **The view's `opened`** lists every closed quantifier step with the
  term it took, as taken (`Interactive::opened_term` reads the table,
  not the shown one: an open witness an axiom filled is still `null`),
  because the page rebuilds each step from the session and replays: a
  witness replayed as the term that filled it would make other members
  than the open one did. Eigenvariables are numbered in the order the
  `∀` steps are taken, so a replay that leaves one out renumbers the
  later ones; the page maps a recorded witness's letters by the `∀`
  inferences' entries here. The view's `forest` (a session with
  quantifiers alone) lets the page read a member's occurrence off the
  session's `instances`, since a replay renumbers the members past the
  forest and only the occurrences stay.
- **`hint` at first order** closes the goal in a clone of the session
  (`Interactive::close`, which takes a goal of instances through its
  written sequent; `Goal::new` refuses members past the forest) and
  reads the steps off the clone's derivation above the goal
  (`read_steps`, as `first_steps` does for a propositional proof). An
  `∃R`/`∀L` step carries the witness the search chose
  (`Interactive::opened_term`) only where the session reads it: a
  witness nothing constrained is a constant the search made in the copy,
  so it stays open (`following_first_order_hints_proves_a_sequent`).
- **The tab's `limits.terms` is its occurrences** (`tab::TERMS`): the
  library's fifty million would pass the tab's memory before a session's
  table refused a step. `search.witness_depth` and `search.instances`
  are the library's.

- **An ordinary tab's calls** are `ordinary.rs`'s, exported as
  `ordinary_start`, `ordinary_view`, `ordinary_rules`, `ordinary_apply`,
  `ordinary_undo`, `ordinary_close`, `ordinary_hint`, `ordinary_draw` and
  `ordinary_export`; a module of their own rather than a branch in each
  linear call, since the session documents, the step's form and the
  addressing (a side and a position) differ and nothing of the linear
  calls' view applies. The session is `ordinary::Interactive` at wire
  level 8; a linear session given to them is refused by its reader. The
  step's JSON is the bindings' `StepForm` (`{"side", "position", "rule",
  "split"}`, the rule by its exact name, which `Rule`'s serde form
  reads), since the library's `Step` has no wire form; `split` left out
  shares the context. `close` and `hint` take `ordinary.translation`
  from the settings and answer the library's ordinary outcome as it
  writes it, at wire level 12: `search` tagged `{"method":
  "embedding", "translation", "image": {…}}`, the goal's image's outcome
  nested (the goal state closes through the embedding alone); `export` and `draw` draw with `view.structural`,
  the view's `inferences` (`derivation_ids`) under the same setting so
  that `i<n>` maps to the goal, and `rocq` is the finished derivation,
  checked, always explicit. `ordinary_model(model, sequent, logic,
  world, settings)` checks a Kripke model the page gives (the library's
  `Model::forcing`): `{"holds", "failing": {"world", "forcing":
  [{"formula", "forced"}]}}`, a model of another logic a failure of
  code `invalid_model` (kind invalid). `ordinary_table(sequent,
  settings)` and `ordinary_tableau(sequent, settings)` answer the
  outcome of those methods (`ordinary::decide_within` with the method
  named, classical logic), which 56b's page draws; `about` lists them
  under the ordinary suite's `methods` (`method`, `call`, `logics`) and
  `limits.table_atoms` among its settings. "Every world" is
  `Forcing::failing_anywhere`, linear. `ordinary_tableau_review(text,
  sequent, settings)` reviews a tableau written by hand
  (`ordinary::WrittenTableau`, the built-in rule file `tableau` read per
  call), `sequent` `""` for none, answering the library's review (level
  14, written only); a text that is no tableau is the library's parse
  error, its place in the text (`details.line`, `span_utf16`) for the
  page's editor; `describe` lists it among the ordinary suite's extras.
  The invertible rules (`interactive.invertible`) apply in an ordinary
  session as in a linear one: `ordinary_start` and `ordinary_apply`
  answer `inverted`, `ordinary_invert_all` applies them whatever the
  settings say, and the view's `invertible` is the library's
  `Interactive::invertible_steps`, the goals the steps closed by an
  invertible rule (the ids `ordinary_undo` answers in `reopened`), which the
  page's undo goes past as in a linear tab.
  `tests/ordinary.rs` holds their tests.
- **A tab in a calculus given as data** has `rules.rs`'s calls,
  exported as `rules_calculus`, `rules_read`, `rules_start`,
  `rules_view`, `rules_rules`, `rules_apply`, `rules_derive`,
  `rules_cite`, `rules_assume`, `rules_undo`, `rules_check`,
  `rules_draw` and `rules_export`. The
  session is `rules::Interactive` at level 17 (18 once it holds a line
  a forward step derived; `rules_undo` answers `reopened: null` and
  `removed`, the line's number, for a forward step taken back) and holds the rule file,
  so the page needs nothing else to go on and a saved session is the
  whole state; reading it (`from_parts`) checks every step again, which
  is what lets the page trust a session from its link or storage. A
  calculus is a rule file's JSON or a built-in's name (`BUILT_IN`, as
  the command's). The step is the bindings' `StepForm` (`{"rule",
  "at", "given"}`, `given` a map so that a name is given once); a
  forward step is `DeriveForm` (`{"rule", "lines", "given"}`), its lines
  by their numbers from 1, which never change while a line stands, so the
  page cites what the view's `lines` lists. Each listed line also has
  `fitch`, its number in the session's Fitch proof (built from
  `Interactive::fitch_derivation`, which opens every subproof with its
  hypotheses, and mapped through `Fitch::nodes`): the session numbers
  lines in the order the steps made them, the drawing top to bottom, so
  the page shows and reads `fitch` and sends `line`, one numbering on
  the screen. The view's `inferences` maps
  each of the tree's `i<n>` to its session node through
  `Drawing::nodes()` and `Interactive::derivation_goals` (a line cited
  twice is one node and two inferences), so the view builds the drawing
  within the settings' bounds and is empty where they refuse it. `draw`
  and `export` write the Fitch proof when `view.layout` is `fitch`
  (`rules::Fitch`, `styles.fitch` its text, ids `l<n>`), its `not_fitch`
  refusal passed on as the library writes it. `about` lists the suite
  as `rules` with the parameter `calculus` and `term` `null` (a rule
  file's rules have no lambda-terms). `tests/rules.rs` holds the tests.
- **The transformations** (`ordinary_translate(sequent, options,
  settings)`, `ordinary_interpolate(sequent, logic, partition,
  settings)`): the library's `negative::compare` and
  `interpolant::interpolate` under the tab's limits, no stop and the
  linear search's defaults (one thread, the copy bound 3), answering
  their level-16 documents; `options` and `partition` are their serde
  forms or `""` for the defaults (Kolmogorov's translation into minimal
  logic; every hypothesis in the first half). `about` lists both under
  the ordinary suite's `transformations` (`transformation`, `call`, the
  `translations` with each `target`, or the `logics`), which the page
  and the workspace graph read. A disagreement or an interpolant that
  fails its check is the library's `claim_fails` (kind defect), thrown.
- **Normal forms and resolution** (`ordinary_normalize(sequent,
  options, settings)`, `ordinary_resolve(sequent, options, settings)`):
  the library's `normal::normalize` and `resolution::clauses_of` with
  `decide`, answering their level-21 and level-22 documents; `options`
  `{"form"}` and `{"form", "method", "clauses", "steps"}` (each key
  optional, `""` for CNF and saturation within the library's bounds).
  `about` lists them under `transformations` as `normal` (its `forms`)
  and `resolution` (its `methods` and `clauses`), and the tab's
  `settings` names `limits.form_occurrences`. **A refutation by hand**:
  `ordinary_clauses(sequent, form, settings)` starts one (the level-22
  refutation with no resolvent), `ordinary_resolve_step(refutation,
  first, second, atom, settings)` adds a resolvent (clauses from 0, the
  atom from 1 as the document numbers atoms, each through
  `calls::index`; an atom 0 is no atom, which `resolve` refuses) after
  checking every step the page sent (`Refutation::check_steps`, the
  library's: the bindings build no error of their own), answering
  `{"refutation", "added", "finished"}`, and
  `ordinary_refutation_check(refutation, settings)` checks one,
  answering `{"finished"}`. They add about 20 KB at gzip (3 741 399
  bytes on 2026-11-20, 8.6 KB under the ceiling).
- **A countermodel on the page** (`ordinary_close`, `ordinary_hint`):
  a session decides a goal through the embedding, which has no Kripke
  model in intuitionistic and minimal logic, so a `not_valid` outcome
  without one gets the SAT solver's model of the goal (`with_countermodel`,
  `sat::decide` on the goal as a sequent, within the settings' limits;
  none where it refuses, the outcome as it was). `ordinary_model_drawing`
  and `ordinary_model_certificate` read that outcome (`wire::upgrade` of
  `ordinary::Outcome`, which carries the countermodel and the logic)
  and answer `svg::countermodel` and `rocq::write_countermodel`; an
  outcome without one is the bindings' refusal `no_countermodel` (kind
  `unsupported`), which the library has no code for.
- **The lambda-term** (`term`, `ordinary_term`) is the session's own
  derivation read back, as the command's session command `lambda` reads
  it, written with the settings' `lambda` (the command's key). What has no
  term, and a refusal of the bounds on the way (the derivation's, the
  term's text, both `limits.derivation_bytes`), is the answer
  `{"error": …}` with the library's error, the command's JSON form under
  `lambda`, never thrown: a refusal is what the page shows in place of the
  term, and only an unreadable session throws. The judgement writes types
  and joins `no_intuitionistic_text_writes_a_classical_symbol`.
- **The protocol** (`protocol.rs`, exported `describe`): which of each
  suite's calls is which of the protocol's eleven (`CALLS`), with its
  arguments (`takes`), `null` where the suite lacks one (the `rules`
  suite has no search, so no `close`, `hint` or `certificate`), its
  calls beyond them (`extra`), the layouts its views draw into and how
  its proofs are built (`builds`: the ordinary suite's `natural` runs the
  `rules` suite's calls in NK, NJ or NM). It is `about`'s answer with
  those keys added (`about`'s own `methods`, the ordinary suite's table
  and tableau, stay), and `catalogue`, the library's
  `protocol::catalogue()`, which the workspace runs. Every `view`
  answers `layout`: `tree`, or `list` for a Fitch proof (`rules`, under
  `view.layout` `fitch`); a key of an answer, so no wire level.
  `tests/protocol.rs` holds every export `describe` names to `lib.rs`
  and every kind's form to the manual's schema
  (`docs/src/wire.schema.json`, which the `web-tests` part keeps). A new
  call of a suite is a row there; a new suite a row of `SUITES`.
- **`ordinary_read(input, logic, settings)`** reads a sequent as
  `ordinary_start` would (a session made and dropped, so that two
  formulas right of ⊢ outside classical logic is the same
  `succedents`), answering `{"kind", "sequent", "text", "logic"}`;
  **`ordinary_certificate(outcome, settings)`** writes an outcome's Rocq
  certificate, its derivation's or its countermodel's.
- **`about` lists the suites** (`suite`, `name`, `calls` the prefix of
  the suite's calls, `parameters` the argument its start takes by name
  with every value and the default, `settings` the keys that change how
  it proves or draws, `term` the name of its call for the lambda-term). The client's registry reads it; a new suite is a
  row there with its calls.

- **A tab of modal logic** has `modal.rs`'s calls, exported as
  `modal_…`, each the protocol's operation of its name run under the
  tab's settings (`modal.decide`, `modal.countermodel`,
  `modal.evaluate`, `modal.draw`, `modal.export`, `modal.calculus`), so
  the page and the workspace answer alike; `modal_read` is
  `protocol::statement("modal", …)`. There is no session: a tab holds
  a statement and its outcome. `modal_model_drawing` builds a
  `modal::Countermodel` of the visitor's model at the world given and
  draws it with `svg::modal_countermodel`, which checks it first, so a
  model where the sequent holds is `invalid_model`, never a drawing.
  `about`'s row names the parameter `system`, default `K`; `SUITES`
  (`protocol.rs`) offers `read`, `draw`, `export_as` and the rest as
  extras, the method `labelled`.
- **A tab of lambda-terms** has `lambda.rs`'s calls, exported as
  `lambda_…`, each the protocol's operation of its name under the tab's
  settings, as the modal tab's are: `lambda_read` (`protocol::statement
  ("lambda", …)`), `lambda_check`, `lambda_draw` and `lambda_export`
  (`lambda.draw`, `lambda.export`, which take the checked term),
  `lambda_read_type` (an ordinary statement claimed in intuitionistic
  logic), `lambda_inhabit` and `lambda_enumerate`, which add to every
  term they answer its `text` and `judgement` (`printed`, the settings'
  `lambda`), as `term` does, so the page prints no term. There is no session:
  the derivation is a function of the term, so a tab holds the term or
  the type and what the calls answered. An ill-typed term and text that
  is no term are thrown, as every reader's error is; the page branches
  on `code` (`ill_typed`, `parse`) and shows the place. `about`'s row
  names the parameter `calculus`, default `simple`, and the setting
  `lambda`; `SUITES` offers `read`, `draw` and `export_as`, the method
  `typing`, and the rest as extras. `tests/lambda.rs` holds the tests.
- **The workspace** (`workspace.rs`, exported `workspace_…`): the
  linked tabs a page keeps, over the library's `workspace`
  (`core-workspace.md`). The page holds the document and sends it with
  every call, read by `Workspace::read_own`, its runs and checks as
  written: read as a claim each call, a search's check would turn
  `recorded` and run again. A link or a file comes in through
  `workspace_open` alone, the claim reader, which also gives its recipes
  the tab's bounds. An action (`workspace_edit`) is read whole before any
  edit, since a refusal inside `Workspace::edit` can only be the
  library's (its error's variants are not built outside it); a node is
  its number or `{"made": K}`, an earlier edit's. `work` lays the open
  tabs' sessions in before a call without an entry of the log, and
  leaves out a node the document no longer holds: after an undo that
  took a tab's node away the page still sends that tab's session, and a
  refusal there left the page's document undone and its view not. An
  answer carries its document as a JSON string of its text (`answer`),
  which the page hands back as it is: read as an object and written
  again (by `serde_json::Value`, or by JavaScript, which puts keys that
  look like numbers first), a value's keys could come out in another
  order, which is another value to the workspace, and every record would
  drop on the next read. The tab's `limits.workspace.bytes` is 2 MiB
  (`tab::WORKSPACE_BYTES`).

## The build (`modules/web.nix`)

- The `wasm-bindgen` CLI is nixpkgs' `wasm-bindgen-cli_X_Y_Z` for the
  version of the `wasm-bindgen` crate in `Cargo.lock`, which the
  manifest pins with `=` (and `wasm-bindgen-test` with it, which pins
  the crate exactly): a module the CLI of another version writes does
  not load. `web-bindgen` compares the CLI's `--version` with the lock.
  A bump of the crate needs the nixpkgs in `flake.lock` to have its CLI;
  otherwise the flake fails at evaluation, naming it.
- The package is `cargo build --release` for wasm32 of each module's
  crate (`modules`, `deduxit-web` and `deduxit-web-render`), `wasm-bindgen
  --target web`, then `wasm-opt -O3`, giving `deduxit_web.js`,
  `deduxit_web_bg.wasm`, `deduxit_web_render.js` and
  `deduxit_web_render_bg.wasm`, with `LICENSE` and one
  `THIRD-PARTY-NOTICES.txt`, a section per module (cargo-about,
  `--frozen`, on each crate's manifest with `web/about.toml` and
  `about.hbs`, the target's crates only, from `about.toml`'s accepted
  licences, which are `deny.toml`'s; the unpublished workspace crates
  are left out). A crate whose licence is not listed fails the package.
- `web-wasm-tests` runs `cargo test --target wasm32-unknown-unknown`
  of the three crates (`selections.web`) with `wasm-bindgen-test-runner`
  in Node, which only finds
  `#[wasm_bindgen_test]` functions; `web-core-wasm32` runs the library's
  whole unit suite on `wasm32-wasip1` under wasmtime, since plain
  `#[test]` functions have no runner on the web target, but the ten
  `core32Skipped` names: eight start a thread, which WASI cannot, and
  two pin byte counts of 64-bit layouts (the Horn engine's least memory
  for a search; the bound where a proof's check, not its search, runs
  out, which on 32 bits the check fits). A new test that starts a
  thread or pins a 64-bit size joins that list with its reason. On WASI
  a panic aborts the run: a failing test there is found by running each
  alone (`--exact NAME`), about 3 minutes for the 403 in a debug build.
- **Clippy for wasm32**: `wasm` (`checks.nix`) on the library and its
  tests, `web-clippy-wasm32` on the bindings and theirs, warnings
  denied: a lint that fires on one width only takes a `cfg_attr` on
  `target_pointer_width` (`flake.md`).
- **The feature baseline**: the module may use the WebAssembly features
  Rust enables for wasm32 by default since 1.87 (LLVM's `generic` CPU:
  mutable globals, sign extension, non-trapping float-to-int, multi-value,
  reference types, bulk memory), which every browser of the last years
  runs; `web-features` validates both of the package's modules against exactly
  those with `wasm-tools`. A toolchain bump that adds one to the default,
  or a crate that enables one (SIMD by `#[target_feature]`, threads),
  fails it: taking the feature is a decision, recorded here with the
  browsers that run it, and the list in `web.nix` moves with it. A page
  that must run on an MVP engine needs `-Ctarget-cpu=mvp` and a nightly
  `-Zbuild-std` (the rustc book's wasm32 page), which no build here does.
- **The wasm-bindgen policy**: the crate is pinned with `=` and moves
  only at an `update-deps` run whose nixpkgs has the matching
  `wasm-bindgen-cli_X_Y_Z` (above), the test crate with it; never ahead
  of nixpkgs, and never alone (the client loads the module this CLI
  writes, so the client's flake update follows the push).
- **The budget** (`modules/budget.nix`): each module at `gzip -9` under a
  ceiling of its own (the bindings' 2 500 000 bytes, 2 438 116 on
  2026-11-21 once the renderers left; the renderers' 1 650 000, 1 609 148;
  before, the one module had 3 913 893 that day under 3 950 000;
  3 622 968 on 2026-11-16 with the modal
  suite, 3 541 980 before it once the generic bodies took the stop as a
  trait object, `core.md`; 3 670 553 before, with the workspace's calls,
  which link every suite's operations through the protocol's run and
  check; 3 111 006 on 2026-11-15 with the calls of a
  calculus given as data, 2 842 389 on 2026-11-14, 2 603 759 at 0.2.0,
  the PNG and PDF renderers' crates two fifths of it) and Cargo.lock's
  packages under another (180; 175). A change that passes one raises it
  in a commit that says what earned the room. The page loads the module
  after first paint, in its worker, so the module is not part of the
  page's first-load budget (about 170 KB of the client's own).
- Locally: `nix develop` has the CLI, `wasm-opt`, Node and wasmtime;
  `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
  cargo test -p deduxit-web --target wasm32-unknown-unknown` and
  `CARGO_TARGET_WASM32_WASIP1_RUNNER="wasmtime run --dir=$PWD::$PWD --"
  cargo test -p deduxit --lib --target wasm32-wasip1 -- proofs::check`
  run the two.

## Extension points

- A new call that needs resvg or krilla belongs in `web-render`, so
  that the bindings' module stays without them; one that needs the
  bindings' calls does not belong there, since the renderers' module
  would then carry them.
- A new call: a function of `calls` over `&str` returning
  `Result<String, Failure>`, one export in `lib.rs`, a row in
  `plan/web/README.md`'s table, and a test in `web/tests/calls.rs` if
  the page relies on a behaviour of it.
- A new default for a tab: a constant of `tab` with the reason, set in
  `tab::defaults()`, and the table of `plan/web/README.md`.
