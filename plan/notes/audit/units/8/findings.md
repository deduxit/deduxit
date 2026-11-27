# Unit 8: findings

Read in full: 21 files. Not read in full: none.

The finder's remarks: The client checkout (`deduxit.github.io/src/worker/deduxit.worker.ts:24-124`, files dated 2026-11-20/21) calls every export through one table and names neither `describe` nor the three `candidates` exports, `ordinary_structure`, `ordinary_certificate`, `rules_close`, `rules_hint`, `lambda_reduce`, `lambda_infer` or `modal_model_drawing`; it likely predates them, so whether the bindings' second protocol table (F-arch-12) has a reader is for the leader to check against the client's main. On the module's size (F-arch-18): `twiggy-top.json` lists 1 023 instantiations twice under identical names (1 420 840 raw bytes, e.g. `lambda::Node` deserialize 2 × 73 806), most likely one copy per crate, which wasm-opt's duplicate-function elimination may merge, so raw attribution overstates; measure the module after wasm-opt before acting on it (F-8-03's 164 030 bytes are distinct instantiations and are not affected). `about` publishes the library's defaults to the page (`ordinary::Options::default().logic`, intuitionistic, and `Mode::default()`), so D1's decision on library defaults reaches the page through it. Unit 2b/4f: the page calls `split_passes` per split choice (`worker.ts:32`, `islands/ui.tsx`), so F-2b-01's cost lands in the browser. Unit 3 or the leader should add a protocol test for F-8-01 when `Net::of` moves into the library. I agree with F-arch-10, -11, -12 and -15 from reading the code and did not raise them again; F-8-13 and F-8-16 follow F-arch-11, and F-8-10 follows F-arch-02.

## F-8-01

- dimension: api
- criterion: X5
- place: core/src/protocol/linear.rs:553; web/src/calls.rs:2295; cli/src/show.rs:1476
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The protocol's `linear.net` answers `invalid_net` (kind malformed, "not a proof net") for a proof the checker accepts when that proof cuts one occurrence twice, because the choice between `ProofStructure` and `MellStructure` is written three times and the protocol's copy lacks the fallback the command and the bindings have.

**Witness.** Run (settled by a run, the handover's F-3-01 question): a program at `target/agents/unit8-net` builds the proof of `core/src/linear/nets/cut/tests.rs:570` (`|- a * a, ~a, ~a` with the cut formula `a` cut under both premises of the ⊗) through the public API and calls `deduxit::protocol::run`:

```text
check: Ok(())
linear.net: error code=invalid_net kind=Malformed: not a proof net: vertex 5 is not the end of one cut link between a cut formula's occurrence and its dual in one box
linear.draw: ok, 3397 bytes
linear.eliminate: ok, 201 bytes
```

The protocol's copy, `core/src/protocol/linear.rs:553-563`:

```rust
            "linear.net" => {
                let proof = proof_of(call.input(0), limits)?;
                let mode = mode_of(&proof);
                let criterion = Criterion::of(mode)?;
                if proof.sequent().fragment().has_exponentials() {
                    written(&MellStructure::from_proof(&proof, criterion, limits, stop)?)?
                } else {
                    written(&ProofStructure::from_proof(
                        &proof, criterion, limits, stop,
                    )?)?
                }
```

The bindings' copy with the fallback, `web/src/calls.rs:2302-2314` (`Net::of`), and the command's, `cli/src/show.rs:1482-1494` (`net_of`), both match `NetError::Placement { fault: Placement::Cut, .. }` and retry with `MellStructure`. The protocol's private `enum Net { Plain, Boxes }` and its reader by the key `vertices` (`protocol/linear.rs:362-388`) are written again as the bindings' `enum Net { Mll, Mell }` and `Net::read` (`web/src/calls.rs:2242`, `:2319`).

**Change.** Give the library one net type and constructor, `linear::nets::Net { Plain(ProofStructure), Boxes(MellStructure) }` with `Net::of(proof, mode, limits, stop)` (the fallback inside), a reader, `is_correct`, `cuts` and the drawing dispatch; the protocol's `linear.net`, the command's `net_of` and the bindings' `Net` call it.

**Value.** A user of the protocol or the workspace gets the net of every checked proof instead of a malformed error that blames the proof; three front ends stop keeping one choice in step by hand.

**Net.** A fix: the protocol's answer changes from an error to a net for proofs that cut an occurrence twice (a test with the proof above in `protocol/tests.rs`); `core/src/linear/nets/cut/tests.rs:556`, `web/tests/calls.rs`, the behaviour lock for the command. No checker or search is touched; the net criterion's check stays the trust boundary.

## F-8-02

- dimension: performance
- criterion: S1
- place: web/src/calls.rs:479; web/src/calls.rs:505
- kind: API break
- cost: medium
- candidate: no

**Claim.** Every bindings call that answers a linear session's view builds the session's whole derivation twice under the library's default limits (1 GiB of memory, no work bound, no size bound) instead of the tab's (96 MiB), and drops `cuts` and `invertible` silently when that fails, because the linear session has no `invertible_steps` or cut list in the library while the ordinary session has one.

**Witness.** ```rust
fn cuts_made(state: &Interactive) -> Vec<CutMade> {
    let Ok(derivation) = state.derivation_within(
        &deduxit::linear::ViewOptions::default(),
        &deduxit::Limits::default().with_derivation_bytes(None),
        never,
    ) else {
        return Vec::new();
    };
```

and again at `web/src/calls.rs:504-511` (`invertible_steps`), both called from `View::of` (`:455-456`), which `changed` runs after `start`, `apply`, `cut`, `undo`, `close`, `close_all`, `fill` and `invert_all`. `Limits::default()` has `memory_bytes: Some(Self::DEFAULT_MEMORY_BYTES)` = `1 << 30` and `work: None` (`core/src/limits.rs:125`, `:264-268`); the tab's bound is `MEMORY_BYTES: u64 = 96 << 20` (`web-tab/src/lib.rs:24`). The ordinary session's library method reads its own history without a derivation: `pub fn invertible_steps(&self) -> Vec<GoalId>` (`core/src/ordinary/interactive.rs:1190`).

**Change.** Add to the linear `Interactive` `invertible_steps()` and a cut list (inference, cut member) read off its own inferences as the ordinary session does, and have the bindings use them; until then build one derivation per view under `settings.limits` with `derivation_bytes` lifted.

**Value.** A visitor's tab keeps to the memory bound it was given on every step of a large session; the page stops losing its replay data silently; the bindings stop computing what the ordinary suite's library already gives (unit 4e's remark).

**Net.** `web/tests/calls.rs` (the view's `invertible` and `cuts`, `a_cut_opens_two_goals`); the time saved per call is a candidate to measure (callgrind on `view` of a session of a few hundred steps; no journey runs a session). No checker or search is touched.

## F-8-03

- dimension: performance
- criterion: P4
- place: web/src/calls.rs:32; web/src/ordinary.rs:30; web/src/rules.rs:33; web/src/modal.rs:25; web/src/lambda.rs:27
- kind: build
- cost: small
- candidate: no

**Claim.** The bindings define five `never` stops, each a distinct fn-item type, so 51 library functions generic over the stop are compiled again for the bindings beside the copy the protocol's `&mut dyn FnMut` stop makes: 164 030 raw bytes in the bindings-stop copies.

**Witness.** Measured on `target/audit/twiggy-top.json` (the module as cargo builds it, names kept) with a short Python script: group items by name with the generic arguments removed, keep groups that hold a copy whose name contains `deduxit_web::<module>::never` and another copy.

```text
items generic over a bindings never: 202 003 bytes (calls 113 886, ordinary 83 710, rules 2 492, lambda 1 915)
functions with a copy for a bindings stop and another copy: 51, bytes of the bindings-stop copies: 164 030
26899  deduxit::export::typst::write          (2 bindings copies, 4 others)
25462  deduxit::linear::proofs::fmt::write_text (3 bindings copies)
16821  deduxit::ordinary::interpolant::interpolant
16505  deduxit::linear::search::prove_goal
15732  deduxit::export::rocq::write
10653  deduxit::export::latex::write
10144  <deduxit::ordinary::Formulas>::write_first_order
```

The bindings already know the fix in one place, `web/src/modal.rs:196-198`:

```rust
    // The stop as the protocol's `modal.draw` passes it, so the drawing is
    // compiled once for both.
    let stop: &mut dyn FnMut(Progress) -> bool = &mut never;
```

**Change.** Give the bindings one stop of type `&mut dyn FnMut(Progress) -> bool` used by every call (one helper in `calls`), and move the library bodies named above behind the crate's one-line generic shim over `&mut dyn` (the handover's list of bodies generic over the stop).

**Value.** Every visitor downloads less: up to 164 030 raw bytes, the share at gzip to be measured; the five copies of `never` go.

**Net.** `budget` (gzip bytes before and after), `web-wasm-tests`; the ratchet for the library's shims. No behaviour changes; no checker, search or certificate logic is touched.

## F-8-04

- dimension: api
- criterion: E6
- place: core/src/protocol/mod.rs:802; web/src/calls.rs:1720; web/src/modal.rs:191; web/src/ordinary.rs:1007
- kind: API break
- cost: medium
- candidate: no

**Claim.** The library has no public call that reads one document from text and refuses what follows it, so its consumers write the three-line `Deserializer` + `wire::upgrade` + `end()` 45 times and eight bindings sites forget `end()`, accepting trailing text that the other calls refuse; the protocol keeps exactly that call private.

**Witness.** The private helper, `core/src/protocol/mod.rs:802-810`:

```rust
fn document<T: wire::Readable>(text: &str, limits: &Limits) -> Result<T, Error> {
    let mut json = serde_json::Deserializer::from_str(text);
    let value = wire::upgrade(&mut json, limits)?;
    json.end().map_err(|error| Error::Json {
```

Counts of `wire::upgrade` / `.end()` sites: cli/src 14/13, web/src 20/12, fuzz targets 8/2, bench/src 1/0. Bindings reads without `end()`: `web/src/modal.rs:191`, `:192`, `web/src/ordinary.rs:638` (`ordinary_model`), `:1007` (`refutation_of`, so `ordinary_resolve_step` and `ordinary_refutation_check`), `:1170` (`countermodel_of`), `web/src/workspace.rs:594` (`workspace_open`), e.g.

```rust
fn refutation_of(refutation: &str, settings: &Settings) -> Result<resolution::Refutation, Failure> {
    let mut json = serde_json::Deserializer::from_str(refutation);
    wire::upgrade(&mut json, &settings.limits).map_err(Failure::from)
}
```

The same `session_of`/`sequent_of` pair is copied in `web/src/calls.rs:1708-1725`, `web/src/ordinary.rs:1323-1340` and `web/src/rules.rs:821-826` (`duplicates.md`: `calls.rs:1709` ↔ `ordinary.rs:1324`, `ordinary.rs:1332` ↔ `rules.rs:818`). `wire::upgrade`'s own example ends with `document.end().expect(…)` (`core/src/wire.rs:467`).

**Change.** Make the protocol's `document` public as `wire::read::<T>(text: &str, limits: &Limits) -> Result<T, Error>` (the `Json` error for trailing text), keep `upgrade` for readers that are not text, and replace the 45 sites.

**Value.** A library user reads a wire form in one call that cannot forget the end check; the page gets one answer for a document with trailing text whatever call it sends it to.

**Net.** Additive in the library; the bindings' seven calls named now refuse trailing text (a behaviour change, one test in `web/tests/calls.rs`); `cargo test --workspace`, the fuzz targets' seeds. No checker is touched.

## F-8-05

- dimension: api
- criterion: S18
- place: web-tab/src/failure.rs:42; web/src/calls.rs:1621; web/src/ordinary.rs:1173; web/src/ordinary.rs:1298
- kind: API break
- cost: medium
- candidate: no

**Claim.** The bindings write errors of the library's codes by hand without the details the library writes for them, and refuse the same condition with another code than the protocol does, because `Error::UnknownName` and the other variants cannot be built outside the crate.

**Witness.** `web-tab/src/failure.rs:42-44`:

```rust
    pub fn refused(code: &str, kind: &str, message: &str) -> Self {
        Self(serde_json::json!({"code": code, "kind": kind, "message": message}).to_string())
    }
```

used as `Failure::refused("unknown_name", "malformed", "the formats are text, latex, typst, svg and rocq")` (`web/src/calls.rs:1621`; also `web/src/ordinary.rs:1150`, `web/src/rules.rs:85`, `:784`, `web/src/workspace.rs:375`, `web-render/src/lib.rs:39`), while the library's `unknown_name` writes `entries!(s; "what" => what; "name" => name; "known" => known)` (`core/src/serialize/errors.rs:70-71`) and the variant is `#[non_exhaustive]` (`core/src/errors/mod.rs:111`). `Failure::refused("no_certificate", "unsupported", …)` (`web/src/ordinary.rs:1298`) copies the code of the library's `Error::Unsupported` (`errors/mod.rs:1102`) without its details. An outcome with no countermodel is `no_countermodel`, kind `unsupported`, in the bindings (`web/src/ordinary.rs:1173`) and `invalid_option`, key `inputs`, kind malformed, in the protocol (`core/src/protocol/modal.rs:409-412`), which `modal_countermodel` throws on the same page.

**Change.** Give `Error` constructors for the refusals a front end makes (`Error::unknown_name(what, name, known)`, a `NoCountermodel` refusal used by the protocol's `modal.countermodel` and by `ordinary_model_drawing`), and drop `Failure::refused`.

**Value.** The page reads one error form per code (`details.known` for every `unknown_name`), and an outside front end refuses in the crate's error family (the consumer test's point 3).

**Net.** `web/tests/*.rs` (codes and kinds), the manual's schema of the error form (`wire-schema`), the behaviour lock for the protocol's `modal.countermodel` if its code changes. No checker is touched.

## F-8-06

- dimension: architecture
- criterion: E6
- place: web/src/calls.rs:63; web/src/protocol.rs:482; core/src/protocol/modal.rs:420
- kind: API break
- cost: medium
- candidate: no

**Claim.** `describe` answers three hand-kept descriptions of the suites in one document (the bindings' `about`, their `SUITES` table and the library's catalogue), which already disagree on a suite's name, and `about`, the bindings' largest function, hand-writes each suite's start parameter because `protocol::Suite` does not give it.

**Witness.** `web/src/calls.rs:133-136`:

```rust
            {
                "suite": "modal",
                "name": "Modal",
                "calls": "modal_",
```

against `core/src/protocol/modal.rs:420-422`:

```rust
    fn label(&self) -> &'static str {
        "Modal logic"
    }
```

`describe` (`web/src/protocol.rs:482-503`) parses `about()`'s text back into a `Value`, inserts `SUITES`' rows and adds `catalogue`, so both names reach the page. The parameters are hand-written per suite (`"values": Mode::NAMES, "default": Mode::default()`, `:74`; `"default": "LJ"`, `:128`), while `protocol::statement` (`core/src/protocol/mod.rs:700-735`) parses the same parameters with no data saying which they are. Sizes in `target/audit/twiggy-top.json`: `deduxit_web::calls::about` 24 725 raw bytes, `deduxit_web::protocol::describe` 11 376.

**Change.** Add the statement's parameter to `protocol::Suite` (name, values, default; `None` for `rules`), build `about`'s suites from `protocol::suites()` with `name()`/`label()` and that parameter, and keep in the bindings only what maps a protocol operation to an export.

**Value.** The page and any other front end read one name and one parameter list per suite from the library; a new suite or mode appears without a hand edit; about 36 KB raw of hand-built JSON code shrink.

**Net.** `web/tests/protocol.rs` (every export `describe` names), `protocol/tests.rs`, the client's suites test (`src/lib/suites.test.ts` reads `about`). The page's labels may change ("Modal" or "Modal logic": one to choose). No checker is touched.

## F-8-07

- dimension: architecture
- criterion: E6
- place: web/src/rules.rs:38; cli/src/calculus.rs:28; core/src/protocol/rules.rs:207
- kind: API break
- cost: small
- candidate: no

**Claim.** The table of built-in rule files by name is written three times, twice in full with thirteen rows and a hand-kept list of names beside one of them, because the library exports the files as constants without a lookup by name.

**Witness.** `web/src/rules.rs:38-42` and `:55-58`:

```rust
const BUILT_IN: [(&str, &str); 13] = [
    ("LK", rules::LK),
    ("LJ", rules::LJ),
    ("LM", rules::LM),
…
pub const NAMES: [&str; 13] = [
    "LK",
    "LJ",
    "LM",
```

The same thirteen rows at `cli/src/calculus.rs:28-42` (`duplicates.md`, 122 tokens), and a third table of three at `core/src/protocol/rules.rs:207-214` (`fn built_in(name: &str) -> Option<&'static str>`). The bindings' error message lists the thirteen names a fourth time as prose (`web/src/rules.rs:88`).

**Change.** Give the library `rules::BUILT_IN: [(&str, &str); N]` (or `rules::built_in(name) -> Option<&'static str>` and `rules::BUILT_IN_NAMES`), used by the protocol, the command and the bindings.

**Value.** A rule file added to the library reaches the command, the page and the protocol at once; the names in error messages come from one list.

**Net.** `cli/tests/readme.rs` (the manual's calculus examples), `web/tests/rules.rs`, `protocol/tests.rs`. Additive in the library; no checker is touched.

## F-8-08

- dimension: architecture
- criterion: E6
- place: web/src/calls.rs:1588; web/src/calls.rs:1853; cli/src/show.rs:849
- kind: API break
- cost: small
- candidate: no

**Claim.** Drawing a classical proof two-sided needs the sequent's writing, and a session or proof read from JSON has none, so the same fallback (read the one-sided sequent again with unbounded limits, then `TwoSided::new`) is written three times in two front ends.

**Witness.** `web/src/calls.rs:1592-1604`:

```rust
    let writing = match state.writing() {
        Some(writing) => writing,
        None if as_written => {
            // The session's sequent passed the bounds already.
            let unbounded = deduxit::Limits::default().with_occurrences(None);
            one_sided = ClassicalSequent::read_one_sided(state.sequent().clone(), &unbounded)
                .map_err(Failure::from)?;
            one_sided.writing()
        }
```

again at `web/src/calls.rs:1870-1879` and `cli/src/show.rs:851-860`, each followed by `TwoSided::new(&derivation, writing, limits)`. The `as_written` test (`view.classical == ClassicalSides::Two && !mode.is_intuitionistic() && !mode.is_ordered()`) is repeated at `web/src/calls.rs:1588-1590` and `:1853-1855`.

**Change.** Let the library build the view from what the derivation has: `TwoSided::of(&derivation, writing: Option<&Writing>, limits)` falling back to the one-sided writing itself, and a `ViewOptions` predicate for "drawn as written in this mode".

**Value.** A library user draws any classical proof two-sided in one call; the bindings and the command lose about 40 lines of copies.

**Net.** The behaviour lock (`show --classical two`), `web/tests/calls.rs` (`export` with `view.classical` `two`), `core/src/linear/proofs/derivation/classical.rs` tests. No checker is touched.

## F-8-09

- dimension: api
- criterion: C-INTERMEDIATE
- place: web/src/lambda.rs:143; web/src/modal.rs:122; core/src/protocol/lambda.rs:498
- kind: API break
- cost: medium
- candidate: no

**Claim.** The protocol's lambda operations answer terms without the `text` and `judgement` that its `linear.term` and `ordinary.term` write, and its drawings and texts come wrapped in JSON, so the bindings parse each answer, re-read every term from a re-serialized `Value` to print it, and unwrap every SVG from its JSON string.

**Witness.** `web/src/lambda.rs:143-151`:

```rust
fn printed(term: &mut Value, settings: &Settings) -> Result<(), Failure> {
    let read = read_term(term, settings)?;
    let (mut written, mut judgement) = (String::new(), String::new());
    read.write(&mut written, &settings.lambda, &settings.limits, never)?;
    read.write_judgement(&mut judgement, &settings.lambda, &settings.limits, never)?;
    if let Value::Object(keys) = term {
        keys.insert("text".to_owned(), Value::String(written));
```

with `read_term` writing the `Value` to text and parsing it again (`:225-230`), once per listed term in `enumerate` (`:264-267`) and twice for the reached term in `reduce` (`:178`, `:180`). The protocol has the printer it needs, `term_value` (`core/src/protocol/ordinary.rs:1877-1887`), used by `linear.term` and `ordinary.term` but not by `lambda.inhabit`, `lambda.enumerate`, `lambda.reduce` or `lambda.infer` (`core/src/protocol/lambda.rs:498-522`). Drawings: `web/src/modal.rs:123-125` and `web/src/lambda.rs:92-94` parse `{"svg": …}` back (`let drawing: Drawing = serde_json::from_str(&drawn)`), so each SVG is escaped into JSON by `written(&super::shared::Drawing { svg })` and unescaped again; `ran`, `Drawing` and `Text` are copied in both files (`modal.rs:31`, `:109`, `:130`; `lambda.rs:33`, `:79`, `:99`).

**Change.** Have the protocol's lambda operations write each term with `text` and `judgement` under the settings' `lambda` (as `term_value` does), and give `protocol` a call that returns a rendering's string (or `Produced` a typed rendering) so a front end need not parse its own library's JSON.

**Value.** Every protocol user (the workspace, the page) gets printable terms; the bindings drop two JSON round trips per term and one escape/unescape of every modal and lambda drawing (time to measure: instructions of `lambda_enumerate` with 32 terms and of `modal_draw` on a large countermodel).

**Net.** The wire form of `lambda.inhabitation`, `lambda.inhabitants`, `lambda.reduction` gains two keys (additive; `wire-schema`, the manual's schema), `web/tests/lambda.rs`, `protocol/tests.rs`. The checks of `well_typed`, `decides` and `reduces` read the term as now. No checker is touched.

## F-8-10

- dimension: api
- criterion: D4
- place: web/src/ordinary.rs:841; web/src/ordinary.rs:1076; core/src/protocol/ordinary.rs:969
- kind: API break
- cost: medium
- candidate: no

**Claim.** Six ordinary-logic calls of the bindings build a default `linear::search::Options` only because the ordinary functions take one, so the page's `search` settings are ignored there while the protocol passes them to the same functions: the same decision runs under other options by front end.

**Witness.** `web/src/ordinary.rs:1073-1079`:

```rust
    let options = deduxit::ordinary::Options::default()
        .with_logic(Logic::Classical)
        .with_method(Some(method));
    let search = deduxit::linear::search::Options::default();
    let outcome =
        deduxit::ordinary::decide_within(&sequent, &options, &search, &settings.limits, never)
```

The same `let search = deduxit::linear::search::Options::default();` at `:841` (`negative::compare`), `:876` (`interpolate`), `:911` (`normalize`), `:968` and `:998` (`resolution::clauses_of`). The protocol's `ordinary.decide` passes `&settings.search` (`core/src/protocol/ordinary.rs:969`), as do `ordinary.interpolate` (`:1015`) and `ordinary.resolve` (`:1071`). The tab's search options differ from the default (`with_copies(None)`, `with_jobs(Jobs::Count(1))`, `with_classical_work(CLASSICAL_WORK)`, `web-tab/src/lib.rs:96-99`). `web.md:376-378` describes the defaults and gives no reason.

**Change.** With F-arch-02: move the options only ordinary logic's engines read into `ordinary::Options` and drop the `&linear::search::Options` parameter from the ordinary functions; until then pass `&settings.search` in the bindings as the protocol does.

**Value.** A user of ordinary logic configures it with its own options value (the consumer test's point 10); the page and the workspace decide a sequent alike.

**Net.** `breaks 0.8.0` for the signatures; `web/tests/ordinary.rs`; the behaviour lock for the command. The searches' counters do not move with the default features (the option values stay); no checker is touched.

## F-8-11

- dimension: api
- criterion: C-SERDE
- place: web/src/workspace.rs:461; web/src/workspace.rs:520; core/src/workspace/mod.rs:263
- kind: API break
- cost: small
- candidate: no

**Claim.** The workspace's `State`, `Cause` and `End` have no serde form, so the bindings spell each variant by hand with an `"other"` fallback, and they spell `protocol::Cost` and `Class` by hand too although both derive `Serialize` in snake case.

**Witness.** `web/src/workspace.rs:520-530`:

```rust
            let cost = operation.map(|operation| match operation.cost {
                protocol::Cost::Instant => "instant",
                protocol::Cost::Search => "search",
                _ => "other",
            });
            let class = operation.map(|operation| match operation.class {
                protocol::Class::Derived => "derived",
                protocol::Class::Work => "work",
                protocol::Class::Copy => "copy",
                _ => "other",
            });
```

while `core/src/protocol/mod.rs:98-99` and `:113-114` derive `Serialize` with `rename_all = "snake_case"` for both. `state_of` and `end_of` (`web/src/workspace.rs:461-493`) map `State::Given`, `Current`, `NotRun`, `OutOfDate(Cause::…)` and `End::…` to JSON by hand; `core/src/workspace/mod.rs:205-206`, `:262-263`, `:277-278` derive only `Clone, Copy, Debug, PartialEq, Eq`.

**Change.** Derive `Serialize` (snake case, `state` and `cause` tags as the page reads them now) on `State`, `Cause` and `End`, write `json!(operation.cost)` and `json!(operation.class)`, and drop the hand maps.

**Value.** A new state or cause reaches the page without a bindings edit (today it reads `"other"`); a library user writes a workspace's view as JSON without a table of names.

**Net.** `web/tests/workspace.rs` (the view's states), the manual's schema if the form is documented there. Additive in the library; no checker is touched.

## F-8-12

- dimension: performance
- criterion: S1
- place: core/src/protocol/ordinary.rs:1700; core/src/serialize/ordinary.rs:1995
- kind: refactor
- cost: small
- candidate: no

**Claim.** The protocol's `ordinary.resolve` on a first-order sequent writes each inference's unifier in full, and its check writes every unifier again per step, with no bound and no stop, and the page reaches it through the workspace, where no clock ends a call.

**Witness.** `core/src/protocol/ordinary.rs:1696-1700`:

```rust
    for step in keys.inferences.unwrap_or_default() {
        match clauses.infer(step.step) {
            Ok(inference) => {
                let same = clauses.text(inference.clause) == step.clause
                    && clauses.bindings_text(&inference) == step.unifier;
```

The run's output is written by `unifier: set.bindings_text(i)` (`core/src/serialize/ordinary.rs:1995`); `bindings_text` (`core/src/ordinary/resolution/first_order.rs:216-239`) writes each bound term with `write_term`, unbounded and without a stop. F-4c-01 (confirmed) shows `resolve --timeout 1s` running 22 s on a family whose unifier doubles a term per variable. The bindings' `workspace_edit` runs `connect` with any protocol operation (`web/src/workspace.rs:270`), and the tab's only cancel is stopping the worker (`web.md:38-44`).

**Change.** Carry F-4c-01's bound into the protocol: write unifiers through the bounded read-back (a refusal at `limits.derivation_bytes` or the memory account) and poll the stop in the check's loop; until then compare unifiers as terms, not texts.

**Value.** A workspace in a browser tab cannot hang on a first-order resolution whose unifiers grow, and its check keeps to the settings' bounds.

**Net.** A fix (a refusal where the run hung): `protocol/tests.rs` with F-4c-01's family; the check of `decides` keeps comparing the same clauses and unifiers. The check is a checker of evidence: route it through the panel.

## F-8-13

- dimension: architecture
- criterion: E6
- place: core/src/protocol/linear.rs:636; core/src/protocol/modal.rs:519; core/src/protocol/lambda.rs:537; core/src/protocol/linear.rs:529
- kind: refactor
- cost: small
- candidate: no

**Claim.** Following F-arch-11: the protocol writes "export a drawable in a format" five times with four `Format` structs and three `FORMATS` lists (the lambda copy lists formats the build may lack), and builds a `Limit` from a `Reason` five times.

**Witness.** `core/src/protocol/lambda.rs:556-561`:

```rust
                    other => {
                        return Err(Error::UnknownName {
                            what: "format",
                            name: other.into(),
                            known: &["latex", "typst"],
                        });
```

where `core/src/protocol/modal.rs:386-391` and `rules.rs:190-195` gate each entry with `#[cfg(feature = …)]`. The match over `"text"`/`"latex"`/`"typst"` is at `linear.rs:636-653`, `ordinary.rs:1200-1217`, `rules.rs:328-342`, `modal.rs:519-545`, `lambda.rs:537-563`; `struct Format { format: Option<String> }` at `ordinary.rs:737`, `rules.rs:201`, `modal.rs:397`, `lambda.rs:326`. `Limit { reason: reason.name(), setting: reason.setting() }` at `linear.rs:529-532`, `ordinary.rs:682-685`, `:1096-1099`, `modal.rs:480-483`, `lambda.rs:502-505`; the resolution bound's mapping twice (`ordinary.rs:1074-1084`, `:1608-1616`).

**Change.** In `protocol/shared.rs`: `fn exported(drawable: Drawable, call: &Call, default: &str, stop) -> Result<String, Error>` with one `Format` and one cfg-gated `FORMATS`; `impl From<&Reason> for Limit` (and one for `resolution::Bound`).

**Value.** A new format or suite is added in one place; a build without `typst` names only the formats it writes in the error.

**Net.** `protocol/tests.rs`, `wire-schema` (the outputs' forms unchanged), `cargo hack` (each feature). The bytes of the protocol (4.5 % raw, `wasm.md`) to compare before and after. No checker is touched.

## F-8-14

- dimension: architecture
- criterion: E6
- place: web/src/calls.rs:885; web/src/calls.rs:936; web/src/ordinary.rs:406
- kind: refactor
- cost: small
- candidate: no

**Claim.** The bindings' modules repeat their glue: the loop that applies the invertible rules after a step three times, the `written`/`session_of`/`sequent_of` helpers in each module, and per-suite copies of `never`, `ran`, `Drawing` and `Text`.

**Witness.** `web/src/calls.rs:886-895`:

```rust
    let mut automatic = Automatic::default();
    if settings.interactive.invertible {
        let limits = inverting(&settings);
        for goal in std::mem::take(&mut opened) {
            let inverted = state
                .invert(goal, &limits)
                .map_err(|e| Failure::described(&Error::from(e), &state))?;
            opened.extend(automatic.add(inverted));
        }
    }
```

again at `web/src/calls.rs:935-944` (`duplicates.md`, 106 tokens) and `web/src/ordinary.rs:405-414`; `fn written` at `calls.rs:1778`, `ordinary.rs:1343`, `rules.rs:829`; `fn session_of` at `calls.rs:1720`, `ordinary.rs:1335`, `rules.rs:821`.

**Change.** One generic `invert_opened(state, opened, settings)` over a small trait both sessions implement (or a library `Interactive::apply_then_invert`), and one module of shared helpers in the bindings.

**Value.** A change to how the invertible rules follow a step is made once for both sessions; the bindings shrink by about 80 lines.

**Net.** `web/tests/calls.rs` and `web/tests/ordinary.rs` (`inverted`, `opened`); no library change; no checker is touched.

## F-8-15

- dimension: idiom
- criterion: E2
- place: web/src/tab.rs:13
- kind: refactor
- cost: small
- candidate: no

**Claim.** The bindings parse the stack size from an environment string with a hand-written const decimal parser, where `usize::from_str_radix` has been usable in const context since Rust 1.82 (the workspace needs 1.92), and the size itself is written in both `build.rs` files.

**Witness.** `web/src/tab.rs:13-22`:

```rust
const fn parse(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut value = 0;
    let mut i = 0;
    while i < bytes.len() {
        value = value * 10 + (bytes[i] - b'0') as usize;
        i += 1;
    }
    value
}
```

`const STACK_BYTES: usize = 8 << 20;` at `web/build.rs:9` and `web-render/build.rs:9`.

**Change.** `match usize::from_str_radix(env!("DEDUXIT_WEB_STACK_BYTES"), 10) { Ok(n) => n, Err(_) => panic!(…) }` in the const; let `deduxit-web-tab` (which both modules depend on) hold the stack size and both build scripts read it, or say in one place that they must agree.

**Value.** A malformed value fails the build instead of giving a wrong number; one stack size for both modules.

**Net.** `web-wasm-tests` (`a_search_at_the_recursion_limit_stops_there`), `about`'s `stack_bytes`. No library change.

## F-8-16

- dimension: api
- criterion: E6
- place: web/src/calls.rs:215; core/src/protocol/linear.rs:329; web/src/calls.rs:2322; core/src/protocol/linear.rs:371
- kind: API break
- cost: medium
- candidate: no

**Claim.** Which linear document a text is (a session, a proof, a net with or without boxes, an outcome, an intuitionistic sequent) is told by probing keys in six places across the bindings and the protocol, by different keys (the bindings tell a session by `inferences`, the protocol by `history`), each parsing the text twice.

**Witness.** `web/src/calls.rs:214-218`:

```rust
#[derive(Deserialize)]
struct Probe {
    /// A session's inferences.
    inferences: Option<serde::de::IgnoredAny>,
}
```

against `core/src/protocol/linear.rs:328-336`:

```rust
#[derive(Deserialize)]
struct Probe {
    /// A session's history.
    history: Option<serde::de::IgnoredAny>,
    /// A proof structure's links.
    links: Option<serde::de::IgnoredAny>,
```

`struct Boxes { vertices: … }` at both `web/src/calls.rs:2322` and `core/src/protocol/linear.rs:371`; `Nodes` at `web/src/calls.rs:2460`; `Ill { hypotheses }` at `web/src/calls.rs:1673` and the protocol's `Claim::hypotheses` (`protocol/linear.rs:280`).

**Change.** Give the library one reader of "any linear document": an enum (`Session`, `Proof`, `Net`, `MellNet`, `Outcome`, `Sequent`, `IllSequent`) read by the probe once, used by the protocol's `input` and the bindings' `read`, `net_in` and `ill_of`.

**Value.** A front end opens a file or a link of any linear form in one call, and the keys that identify a form are decided in one place next to the forms.

**Net.** `web/tests/calls.rs` (`read` on each form), `protocol/tests.rs` (`input`'s kinds), `wire-schema`. No checker is touched.
