# Unit 6a: notes

## core/src/lib.rs (437 lines, read in full)

- lib.rs:18 "A sequent of ordinary logic, here intuitionistic logic, the default:" — crate doc's first example leans on `ordinary::Options::default()` being intuitionistic (consumer test 6, D1).
- lib.rs:37-251 D4: after the first ordinary example, every walk-through section is linear: "A proof, its fragment and its JSON" (111), "Intuitionistic input" (143, linear ILL), "Limits and stops" (167, `linear::prove_within`), "The command's answers" (198, linear search options). Only protocol (227) uses ordinary. lib.rs:97 "Bound a search's time ... [`linear::prove_within`]", lib.rs:100 "Print or export a derivation: [`Proof::derivation`](linear::Proof::derivation)".
- lib.rs:107-109 "Proofs, nets and errors name the *occurrences* of a sequent ... the sequent's [`Forest`](linear::Forest)" — a linear notion stated as crate-wide.
- lib.rs:253-263 "# Errors" lists only linear-specific error types (CheckError, NetError, ShapeError), none of ordinary/modal/lambda/rules.
- lib.rs:274-282 "# Syntax and JSON": linear Sequent first, then a line for ordinary; modal, lambda, rules syntax not named.
- lib.rs:316-322 Stability: closed enums listed linear-first (`Term`, `Kind`, `Node`, `Rule`, `Verdict`, ...).
- No "write your own engine" entry (X8; consumer test 12).
- lib.rs:329-330 `mod errors;` private + re-exported: one path for Error. But lib.rs:342 `pub mod limits;` + lib.rs:436 `pub use limits::{Limits, Phase, Progress, Refusal};` and lib.rs:420 `pub mod settings;` + 437 `pub use settings::{Clock, Settings}` -> two paths each (surface.md: Clock, Limits, Phase, Progress, Refusal, Settings). limits also has Bound, Held, Space, WorkspaceLimits only under `limits::`. Inconsistent: errors private-with-reexports; limits/settings public-and-reexported.
- lib.rs:431-435 root exports `Backwards`, `StepFault` (no cfg) and `WitnessFault` (cfg interactive): step-errors of interactive sessions at the crate root, besides Error.
- lib.rs:353-416 sealed traits fine.
- Features in docs lines 284-309 match Cargo.toml.

## core/Cargo.toml

- features: serialize = ["dep:serde"]; C-SERDE asks the feature be named `serde`.
- default 9 features include protocol (serde_json), sat (batsat). No feature for the data model + checkers alone.
- core.md:52 `rules` is behind `serialize`; core.md:110-112 modal's `calculus`, `search`, `decide` behind `serialize`: a JSON feature gates a whole suite's decision and the rule-file calculi.
- `parse` gates the text syntax AND lltp, mist, families, tptp (benchmark inputs).
- keywords: "theorem-prover","logic","proof-search","sequent-calculus","linear-logic": the only suite named is linear (D4; 5 keyword max).
- no `authors` (C-METADATA; workspace.package has none either).
- timings.md: core alone dev default 40 s wall, no-default 16 s.

## core/src/errors/mod.rs (1820 lines, read in full)

- mod.rs:58-783 `Error`: ~67 variants (cfg-gated by feature), one enum for every suite. Root error names linear types (Fragment, Mode, Engine, NotTaken, CheckError, ShapeError, NetError), ordinary (Translation, Logic, Method, negative::Translation, StepError), lambda (NoTerm). By design ("one error family", core.md:243).
- Variant name vs code divergence (mod.rs:1035-1133): `Model`->"invalid_model" (1116), `Structure`->"invalid_structure" (1117), `InvalidRefutation`->"invalid_clause_refutation" (1122) while `Refutation(_)`->"invalid_refutation" (1070), `Reduction`->"reduction_rejected" (1132), `Unsupported`(rocq)->"no_certificate" (1102), `Version`->"unsupported_version" (1054), `Check(Invalid)`->"invalid_proof". Variant naming mixes `Invalid*` prefix (InvalidCalculus, InvalidDerivation, InvalidOption, InvalidRefutation) with bare nouns for the same sense (Model, Structure, Refutation, Countermodel). Confusing: `Error::Refutation` (linear disproof check) vs `Error::InvalidRefutation` (clause refutation).
- String-typed reasons: `Model{reason:String}` 694, `Countermodel` 686, `Structure` 779, `InvalidCalculus` 703, `InvalidDerivation` 711, `Partition` 477, `ReadBack` 676, `InvalidRefutation` 731, `NotFitch` 761, `ClaimFails` 514, `NotApplicable` 493, `IllTyped` 625 — while the linear checker's fault is structured (`Check(CheckError)` with `Fault`, node; describe). Ordinary/modal model checks & rules derivation checks give prose only: a front end cannot point at the world/node at fault (except IllTyped has node/span).
- mod.rs:460-470 `GlivenkoFirstOrder` Display message is ~10 lines, an essay with Rocq lemma names; C-GOOD-ERR / T1 (messages concise). Doc 453-459 repeats.
- mod.rs:182-184 `Version{found:u64, supported:u32}` mixed widths; `IndexOutOfBounds{index:usize,len:usize}` vs `NoCut{node:Option<u32>}`, `NotGiven{node:u32}`.
- mod.rs:853-942 `CODES` hand list in sync with `code()` (test the_codes_are_listed); fine.
- mod.rs:1035 `code()` returns `&'static str`.
- mod.rs:1295 size assert 64; 1298 Send+Sync assert. good.
- mod.rs:1159 refusal_kind.

## core/src/errors/describe.rs (388, full)

- describe.rs:121 `pub trait Owner` (root re-export `deduxit::Owner`) and describe.rs:231 `Described` (root): every implementor is linear (Forest, Proof, Derivation, ProofStructure, MellStructure, linear::Interactive, lines 126-211); only linear errors (`Error`, CheckError, NetError, ShapeError) describe. Contradicts core.md:62 "What goes to the root is what ordinary or lambda uses besides the linear suite and is no part of the linear calculus". D4/C-HIDDEN: root holds a linear-only notion.
- Prefix literals duplicated: "not a proof net: " at describe.rs:306, describe.rs:374 and mod.rs:295 (`#[error("not a proof net: {0}")]`); "not an intuitionistic sequent: " describe.rs:378 and mod.rs:361. INVALID_PROOF and REJECTED are constants, these two not. E6 small.
- describe.rs:305-308 special-cases Subject::Error(Error::Net) duplicating Error::write's Net arm (373-376) to pass vertices.
- Serialize for Described (318-339) clones the inner error to wrap — cold.

## core/src/errors/parse.rs (114, full)

- parse.rs:31-38 `pub reserved: bool` documented "Always `false`: no word is reserved"; grep: only ever set false (parse.rs:73); read at lltp.rs:531, core/tests/parse.rs:289 asserts false; Display arm parse.rs:94-98 dead. X9 public by accident / dead field.
- ParseError has pub fields + #[non_exhaustive]; fine (passive data).
- Error::Parse(Box<ParseError>) — ParseError implements std::error::Error (114).

## core/src/errors/step.rs (563, full)

- step.rs:209 `StepFault` (rules + resolution by hand), step.rs:407 `Backwards` (only StepFault::Backwards' payload), step.rs:32 `WitnessFault` (linear + ordinary sessions) re-exported at the root (lib.rs:434-435) as `deduxit::Backwards` etc. `Backwards` at root is an opaque name.
- StepFault and Backwards are not cfg-gated though `rules` is behind serialize; WitnessFault is behind interactive.
- `Check { reason: String }` 246 again prose.
- Fine otherwise; Display phrases.

## core/src/limits.rs (889, full)

- limits.rs:403-424 `Progress` #[non_exhaustive], pub fields, derive Default (405); `Progress::new` pub(crate) (429). An outside crate CAN build one: `let mut p = Progress::default(); p.phase = Phase::Search; p.work = n;` (non_exhaustive forbids only struct expressions). So consumer test point 2 ("only the crate can call a stop") is half-wrong: possible but undocumented, no ctor (C-CTOR).
- limits.rs:583-706 `Refusal`: every variant #[non_exhaustive] struct variant, no constructor, no Default → an outside engine cannot make a refusal (consumer test 3; X2). core.md:394 "a front end never builds an error of the library: it asks the call that does" — reason is adding fields without a break; associated constructor fns keep that reason intact.
- limits.rs:15-39 the table "The long calls, the fields of Limits that bound each, and how often each asks its stop": lists linear and ordinary calls and exports only; no modal (`modal::decide_within`), lambda (`Term::check`, `inhabit`, `enumerate`, `Term::reduce`, `principal_type`), rules (`rules::Derivation::check`), protocol (`protocol::run`), workspace, ordinary engines (`ljt::decide`, `sat::decide`, `resolution::decide`, herbrand, normal::rewrite). Reference incomplete; D4 (linear first, rows 20-29).
- limits.rs:56-121 Limits holds suite-specific bounds: table_atoms (ordinary truth table), form_occurrences (ordinary normal forms), workspace (protocol feature) — WorkspaceLimits/Held/Refusal::Workspace not gated though `workspace` is behind `protocol`.
- limits.rs:448 Phase: has `name()` but no Display (ErrorKind has Display at errors/mod.rs:834). C-COMMON-TRAITS small.
- limits.rs:532 Space::singular pub(crate) vs 549 plural pub: inconsistent, small.
- limits.rs:101 table_atoms Option<u32> while Refusal::Atoms limit u64 (671-673).
- limits.rs:880 `counting(stop: impl FnMut...)` generic per caller — P4 candidate; check callers.
- Limits: pub fields + with_* both: consistent internally.

## core/src/settings.rs (359, full)

- settings.rs:129-153 `Settings` fields: `search: Options` (= linear::search::Options), `view: ViewOptions` (linear), `input: InputOptions` (linear ILL reading), `cut: cut::Options` (linear proofs::cut), `interactive: interactive::Options` (linear proofs::interactive) — unqualified generic names for one suite's options — beside `ordinary: ordinary::Options` and `lambda: lambda::Options` named by suite. D4. Also ordinary engines' bounds are set through `search` (161-166: with_path_length = connection prover's active path, with_model_size/with_model_worlds = finite model search) and protocol passes `&settings.search` to ordinary::decide_within (protocol/ordinary.rs:969,1015,1071). No modal field (modal::Options holds only `system`, chosen per call: protocol/modal.rs:477).
- settings.rs:180-243 with_* for every field but `lambda` (no with_lambda). small.
- settings.rs:272 `stop<E: Fn() -> Duration>(&self, elapsed: E) -> impl Fn(Progress) -> bool + use<E>` fine (E1).
- doc settings.rs:112 "[`search::Options::default()`](Options)" link text names `search::Options` (path deduxit::linear::search::Options).

## core/src/wire.rs (630, full)

- wire.rs:500-526 `CAUSE` thread-local: `fail` stores the library's typed error; only `upgrade` (476-498) reads it back. `Within` (417-441, public seed) returns `T::read`'s D::Error, i.e. the text only; the typed error (Refusal, Version, IndexOutOfBounds) is lost and stays in the thread-local until the next `upgrade`.
- WITNESS: cli/src/io.rs:402 `Within::<IllSequent>::new(&within).deserialize(&mut document)` then `.context("not an intuitionistic sequent in JSON")`. Run (target/debug/deduxit, scratchpad ill.json `{"version":5,...}` 5 occurrences): `deduxit prove --file ill.json --intuitionistic --occurrence-limit 2` → "error: not an intuitionistic sequent in JSON: the sequent unfolds to at least 5 subformula occurrences, more than the limit of 2", exit 2; the one-sided JSON path (`upgrade`-like, ll.json) says "the sequent unfolds to 5 subformula occurrences, more than the limit of 2; raise it with --occurrence-limit". So Within's text-only error makes the CLI call a valid over-limit document "not an intuitionistic sequent" and lose the hint. Only outside user of Within: cli/src/io.rs:7.
- wire.rs:350-587 level constants out of order (LEVEL 31, LAMBDA 23, UNTYPED 30, POLYMORPHIC 31, INHABITANTS 24, RULES 15, SESSION 17, FORWARD 18, WORKSPACE 19, MODAL 20, BASE 1, BOXES 4, ILL 5, ..., then after fns CUTS 6, TERMS 7, ORDINARY 8, ORDINARY_CUT 9, ORDINARY_TERMS 25, MODELS 26, KRIPKE_STRUCTURES 27, SEARCH 12, KRIPKE 13, TABLES 14, TRANSFORMS 16, FORMS 21, CLAUSES 22, HERBRAND 28, FIRST_ORDER_CLAUSES 29, BASIS 11); levels 2, 3, 10 have no constant (wire.rs:612 `=> 2`, 626 `=> 3`, 627 `=> 2`). Nothing ties `LEVEL` to the highest constant (POLYMORPHIC = 31): a new level must change LEVEL by hand.
- wire.rs:4-341 module doc: the forms list leads with the linear forms (Sequent, Proof, Disproof, structures, ILL, FO, Session: 87-153), then ordinary, modal, rules, lambda; examples use `A, A -o B |- B` (85). D4 mild (a reference table ordered by level is fine; the forms list is not ordered by level: Outcome (level 1) last at 336).
- wire.rs:180,241,248,256 doc examples hard-code `"deduxit": "0.8.0"` — will be stale at 0.9.0 (it's illustrative).

## core/src/hash.rs (23, full) — fine. foldhash FixedState; P6 ok.

## core/src/serialize/mod.rs (105, full)

- mod.rs:5 deny wildcard_enum_match_arm. `auto`, `exact` helpers. ok.
- All suites' serde lives here (ill, interactive, kripke, lambda, modal, nets, ordinary, proofs, resolution, rules, search, sequents) while some types derive serde in their own module (modal/decide.rs:20 Options, Member, lambda ids). deps.md: serialize names linear 110 paths, ordinary 72, modal 14, lambda 16, rules 16, wire 24. Architecture: the format boundary cuts across suites.

## core/src/serialize/errors.rs (385, full)

- errors.rs:198-385 hand-written `Serialize` for NotTaken, CheckError, Dyadic, Fault, ShapeError, NetError, Placement (~190 lines) because the ids have no serde: `"member" => &member.get()`, `"vertex" => &vertex.get()`, `vertices(cycle)` (383) allocating Vecs. Meanwhile Refusal (limits.rs:588-592), StepFault (step.rs:203-207), WitnessFault, Backwards derive with `serde(tag=..., rename_all="snake_case")`.
- Id newtypes' serde (C-SERDE): Member (occurrences/mod.rs:58-62), Eigen (first_order.rs:97), lambda TermId etc. (lambda/mod.rs:209..259) Serialize+Deserialize transparent; GoalId (proofs/interactive/mod.rs:328), InfId (derivation/view.rs:18), rules GoalId (rules/interactive.rs:17) Serialize only; OccId (occurrences/mod.rs:28), VertexId (nets/mod.rs:168), proofs NodeId (proofs/mod.rs:80), FrameId (instances.rs:29), linear TermId and Atom (sequents/term.rs:8,36), ordinary NodeId (ordinary/mod.rs:710), modal NodeId (modal/mod.rs:75), rules TermId (rules/terms.rs:14), rules NodeId (rules/derivation.rs:13) none.
- errors.rs:24-52 `Details` big match: fine by rule (no wildcard).
- `entries!` local macro.

## core/src/serialize/ill.rs (364, full)
- ill.rs:322-330 hand-written occurrences check (one of ~30 `if let Some(limit) = limits.X && n > limit { Err(Refused(..)) }` sites across core/src: mist.rs:220, occurrences/mod.rs:358, lambda/read.rs:161, modal/mod.rs:477, serialize/kripke.rs:124, ordinary/mod.rs:1764, ordinary/translate.rs:602, resolution/outside.rs:146, serialize/resolution.rs:130, serialize/proofs.rs:133, serialize/sequents.rs:373,559, rules/text.rs:232, serialize/rules.rs:531, serialize/interactive.rs:282, serialize/modal.rs:131, ordinary/interactive.rs:1102, serialize/ordinary.rs:162, ordinary/derivation.rs:749, ordinary/table.rs:240, ordinary/tableau.rs:816 …). E6 + X2: `Limits::admit_occurrences(n)`-style helpers.
- otherwise fine.

## core/src/serialize/interactive.rs (329, full)
- interactive.rs:105 `marks: Vec<(u32, u32, u32, u32, u32, u32)>` 6-tuple (wire array; internal tuple; E3 tuple struct with many fields). small.
- interactive.rs:226-251 Serialize builds a full owned proxy (Vec<u32> per inference, Sequent proxy) to write a session; web client round-trips sessions on every request. P2 candidate (measure web/interact journey with DHAT).
- interactive.rs:110-169 TermForm mirrors PartTerm with two From impls; fine.

## core/src/serialize/kripke.rs (175, full)
- kripke.rs:95-111 From<&Model> clones every world's atoms (Vec<String>) and arrows to write; P3 cold.
- kripke.rs:45-93 StrictWorld/StrictModel duplicate WorldForm/ModelForm for deny_unknown_fields; recorded on purpose (core-sequents.md:383-386).

## core/src/serialize/lambda.rs (293, full)
- Written<'a> borrows (good pattern, no clone) — contrast to modal/kripke/interactive proxies that clone.
- lambda.rs:14 `use crate::linear::search::Reason` — Inhabitation::Unknown carries linear's Reason (D4, consumer test 10 analogue for lambda).
- lambda.rs:241,285 `deduxit: env!(..)` &'static str; modal.rs:306 `Some(env!(..).to_owned())` String.

## core/src/serialize/modal.rs (531, full)
- modal.rs:36-68 FormulaForm, 70-87 From<Node>, 92-108 SequentForm, 110-187 within, 238-248 VerdictWord: duplicates of serialize/ordinary.rs (duplicates.md: modal.rs:29↔ordinary.rs:33 144 tok; modal.rs:67↔ordinary.rs:71 187 tok/17 lines; modal.rs:139↔ordinary.rs:190; modal.rs:158↔ordinary.rs:215 183 tok; modal.rs:212↔ordinary.rs:293; modal.rs:317↔ordinary.rs:1071). The modal form is "an ordinary sequent's form with □ and ◇" (wire.rs:159-163).
- modal.rs:6 `use super::search::Why` — modal Unknown reason is linear's Reason written by linear's Why (D4).

## core/src/serialize/ordinary.rs (2065, full)
- ordinary.rs:37-91 FormulaForm/From<Node>; 93-245 SequentForm; duplicated in modal.rs (see above).
- ordinary.rs:709-832 outcome_form: level computed by a chain of `version.max(..)` across 744-809 (clippy too_many_lines 110 at 709).
- ordinary.rs:580-629 KripkeStructureForm::of clones `c.model` (586), StructureForm::of clones `c.structure` (626) to serialize; CountermodelForm built inline again at 1660-1663 (dup of 603-608).
- `SequentForm::of(s.formulas(), s.left(), s.right())` repeated at 585, 605, 625, 1278, 1656, 1661, 1672(closure), 1733, 1891, 1982, 2037: wants `SequentForm::of_sequent(&Sequent)`. small E6.
- ordinary.rs:471-473 vs 1553-1555 tree-shape check by parents count repeated (derivation root last / tableau root first).
- E5 seam: sequent/derivation/outcome/session read-back (1-1250), tables & tableaux (1252-1598), written-only transforms (comparison, interpolation, review, normal form, Herbrand, first-order resolution: 1600-2065).
- ordinary.rs:4 `use super::search::Why` and 766, 787, 804 `crate::linear::search::Reason::{Incomplete, PathLength, ModelSize, KripkeSize}`: ordinary's unknown reasons (path length = connection prover, model size = model search, kripke size) are variants of linear::search::Reason (D4, consumer test 10). Written through linear's `Why`.

## core/src/serialize/proofs.rs (448, full)
- proofs.rs:260-269 `Proof::keys` builds owned proxies (Sequent::from(p.sequent()), Vec<Step>, goal Vec) to write a proof; Elimination (398-433) too. P2 candidate together with kripke/ordinary/interactive proxies; no ratchet journey writes JSON (bench/ceilings.csv: read-json only).
- From<Node> for Step and back: fine.

## core/src/serialize/resolution.rs (253, full)
- resolution.rs:229 `RefutationForm::of(r).resolvents` builds the whole form (atoms.to_vec(), all clauses) to keep the resolvents. small P2 cold.
- resolution.rs:155-159 a resolvent's parent clause out of range is reported as `Space::Node`; errors/mod.rs:1196-1199 writes Space::Node as "a proof refers to node {index}, but it has {len} nodes" — wrong words for a refutation's clause. Same: Space::Term message (errors/mod.rs:1192-1195) "a root formula is term {index}, but the arena has N terms" used for an ordinary document's first-order term index (serialize/ordinary.rs:263-267, Read::term) and for an ill arena operand? (ill.rs:254 roots: correct). out_of_bounds assumes linear contexts.

## core/src/serialize/rules.rs (856, full)
- rules.rs:23-51 `CalculusForm` (pub(crate)) is what `rules::Calculus` stores (rules/calculus.rs:12 `use crate::serialize::rules::{CalculusForm,..}`, :267 `form: CalculusForm`, :383 `from_form`): the rule-file suite is built on the serde proxy, which is why `rules` is behind `serialize` (lib.rs:350). And modal: modal/mod.rs:42-63 `calculus`, `decide`, `search` and `pub use decide::{..., decide, decide_within}` behind `#[cfg(feature = "serialize")]` because modal::decide emits a `rules::Derivation`. So the JSON feature gates the modal decision and calculi-as-data: `default-features = false, features = ["parse"]` has modal sequents but no `modal::decide`.
- rules.rs:310 Serialize for Calculus clones the whole form; 728 session clones calculus form. cold.
- rules.rs:526-538 occurrences check again (hand-written).
- rules.rs:561-565 `term_at` → Space::Term → "a root formula is term N" message for a rule derivation term index (wrong words).
- PatternForm recursion bounded by DEPTH 64: ok (S rule nothing recurses... patterns nested 64 max).

## core/src/serialize/search.rs (1156, full)
- search.rs:53-160 `Why` mirrors `linear::search::Reason` variant for variant with two From impls (~110 lines); Reason is Copy with plain u32/u64 fields: `#[cfg_attr(feature="serialize", derive(Serialize, Deserialize), serde(tag="kind", rename_all="snake_case"))]` on Reason gives the same JSON. search.rs:211-546 `WhyNot` mirrors `Refutation` (~330 lines with converters) because Atom, OccId, Symbol lack serde and Sign is written "atom"/"dual" via a `Literal` mirror (179-206). E6, with the ids finding.
- search.rs:784-828 outcome form: proof's keys flattened (Proof::keys owned clone, 803).
- search.rs:845-853 EngineRead unknown engine → Engine::Unknown: fine.
- search.rs:997-1059 AutoOrNumber helpers; `serialize::auto` (mod.rs:14-42) is another "auto" helper: two ways of "auto" (auto::serialize for Option<T: Display>, AutoOrNumber for Jobs/Cadence). small.

## core/src/serialize/sequents.rs (664, full)
- sequents.rs:571-581 `impl TryFrom<Sequent> for Seq` reads with `&Limits::default()` (578-580: `Self::read_within(s, &Limits::default())`), so `limits.terms` of the caller is NOT honoured for the nested sequent of a proof (proofs.rs:303), a proof structure (nets.rs:123), a structure with boxes (nets.rs:313), a session (interactive.rs:259). Only the sequent document (Sequent::within, 556) and the disproof (search.rs:592 `self.sequent.within(limits)`) use the caller's limits.
- WITNESS (target/debug/deduxit, scratchpad): `prove 'p(f(f(a))) |- p(f(f(a)))' --term-limit 1` → "error: the first-order terms unfold to at least 3 terms, more than the limit of 1; raise it with --term-limit", exit 2. `prove ... --format json > fo-outcome.json`; `check --file fo-outcome.json --term-limit 1` → "valid proof ..." exit 0 (cli/src/check.rs:1279 `wire::upgrade::<Proof, _>(&mut document, limits)`). The proof reader ignored the caller's bound of 1. S1 (bound the caller sets).
- (CLI side, unit 7a: cli/src/io.rs:411 reads a JSON sequent `Within::<Sequent>::new(&unbounded)` so `prove --file fo-seq.json --term-limit 1` also accepts: exit 0.)
- sequents.rs:533-550 From<&Seq> owned proxy (names.to_vec(), binders clone) — P2 candidate with others.

## core/src/serialize/nets.rs (339, full)
- nets.rs:85 `.max(2)` level of cyclic order as a bare 2 (no wire constant), like wire.rs:612,627.
- nets.rs:123, 313 `Sequent::try_from` default limits (above).
