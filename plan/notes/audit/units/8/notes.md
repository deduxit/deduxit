# Unit 8 notes

## protocol/mod.rs, shared.rs, linear.rs, rules.rs, ordinary.rs, modal.rs, lambda.rs (read in full)

- mod.rs:418 SUITES order linear, ordinary, rules, modal, lambda (F-arch-15). KINDS :505 lacks lambda (F-arch-11). statement :706 match, known list lacks "rules" (fine: no typed statement) but is a 4th registry.
- mod.rs:356 Suite unsealed, docs promise outside suites; Kind etc non_exhaustive w/o ctor (F-arch-10).
- `Limit { reason: reason.name(), setting: reason.setting() }` five times: linear.rs:529, ordinary.rs:682, ordinary.rs:1096, modal.rs:480, lambda.rs:502. Resolution's bound -> Limit mapping twice: ordinary.rs:1074-1084 and :1608-1616.
- Export in a format: `Format` struct four times (ordinary.rs:737 pub(super), rules.rs:201, modal.rs:397, lambda.rs:326), FORMATS three times (ordinary.rs:743, rules.rs:190, modal.rs:386) + lambda.rs:560 literal `&["latex","typst"]` without cfg. The match latex/typst/UnknownName five times: linear.rs:636, ordinary.rs:1200, rules.rs:328, modal.rs:519, lambda.rs:537.
- linear.rs:553-563 `linear.net`: exponentials -> Mell, else ProofStructure; no fallback on Placement::Cut. The command (cli/src/show.rs:1461-1496) and bindings (web/src/calls.rs:2290 Net::of) each choose with the fallback. Library has no `net_of`. Probe built at target/agents/unit8-net.
- ordinary.rs:1131-1168 and :1225-1266: `Input::Outcome(o) if o.countermodel.is_some()` then `.as_ref().unwrap_or_else(|| unreachable!(..))` six times: shape of F-arch-13.
- ordinary.rs:1563 and :1721 the same `texts` closure twice; `"version"` inserted into a Value and re-serialized to read a derivation: ordinary.rs:1558-1561, :1716-1720; ResolvedKeys json! rebuilt twice ordinary.rs:922-928 and :1775-1781.
- ordinary.rs:1632 `first_order_document` parses the whole output into serde_json::Value to find a key; called in summary (:913) and in check (:1301) before the typed read: output parsed 2-3 times.
- ordinary.rs:1700 `clauses.bindings_text(&inference)` per step of the check: unbounded text (F-4c-01 question).
- ordinary.rs:1022-1030: interpolate writes statement to String, reparses to Value, then writes again.
- linear.rs:717 `error.code() == "uncertified"`: matches by string code.
- protocol uses `ordinary::Options::default().with_logic(..)` — fine.

## RUN: linear.net on a cut-twice proof (target/agents/unit8-net, protocol::run)
Output:
  check: Ok(())
  linear.net: error code=invalid_net kind=Malformed: not a proof net: vertex 5 is not the end of one cut link between a cut formula's occurrence and its dual in one box
  linear.draw: ok, 3397 bytes
  linear.eliminate: ok, 201 bytes
=> F-3-01 confirmed: the protocol calls a checked proof's net malformed. Choice of net written 3x: cli/src/show.rs:1461-1496 (net_of), web/src/calls.rs:2290-2316 (Net::of), core/src/protocol/linear.rs:553-563 (no fallback).

## web-tab, web-render, web/build.rs, tab.rs, lib.rs, protocol.rs, calls.rs (read in full)
- web/src/tab.rs:13-22 hand-written const decimal parser; `usize::from_str_radix` is const since 1.82 (rust-version 1.92). STACK_BYTES 8<<20 in web/build.rs:9 and web-render/build.rs:8 (dup const).
- web-tab/src/failure.rs:26-44 builds error JSON by hand (json, refused). Failure::refused("unknown_name", ...) at web/src/calls.rs:1621 and web-render/src/lib.rs:39 where library has Error::UnknownName (variant #[non_exhaustive] at errors/mod.rs:111 -> not constructible outside). calls.rs:1752 index() refuses a number as code `json`.
- calls.rs:63-164 `about()` hand-written JSON (24.7 KB in the module, wasm.md); suites' names: modal "Modal" (calls.rs:135) vs library label "Modal logic" (protocol/modal.rs:421). describe() (protocol.rs:482) parses about() back to Value and merges SUITES + catalogue: three descriptions of the suites in one answer.
- web/src/protocol.rs:151 SUITES hand table; library has modal.decide op but web lists modal_decide as extra; lambda check/reduce/infer extras; order linear first (D4).
- calls.rs:478-522 cuts_made and invertible_steps each build the session's derivation with `Limits::default().with_derivation_bytes(None)` (1 GiB memory, no work bound) on EVERY View::of (every call that changes a session); errors silently -> empty lists. Tab's memory bound 96 MiB bypassed. Ordinary has library Interactive::invertible_steps.
- calls.rs:1588-1611 and :1853-1879: the "as written / writing() or read_one_sided" two-sided logic twice (export and drawn).
- calls.rs:2290-2316 Net::of (fallback) = cli show.rs net_of; nets_exist (calls.rs:1491) picks MellStructure::exist vs nets::exist.
- `wire::upgrade(&mut json, limits)` + `json.end()` 3-line read: 15 sites in web/src; protocol has private `document()` (mod.rs:802) doing exactly this. Library lacks a public "read one document from text".
- Probing documents by key: web Probe{inferences} (calls.rs:215) tells a session by `inferences`; protocol linear Probe{history} (protocol/linear.rs:329) tells it by `history`; Boxes{vertices} twice (calls.rs:2322, protocol/linear.rs:371); Nodes (calls.rs:2460), Ill{hypotheses} (calls.rs:1673) vs protocol Claim.hypotheses. Each parses the text twice.
- calls.rs:885-895 and :936-944 the invert-after-step loop twice (duplicates.md row 150).
- StepForm/Judged/Offered are bindings' wire forms (web.md records it, waiting for the library).

## web/src/ordinary.rs, rules.rs, modal.rs, lambda.rs, workspace.rs (read in full)
- ordinary.rs:841,876,911,968,998,1076: `deduxit::linear::search::Options::default()` built only to call ordinary fns (F-arch-02 from the consumer side); protocol passes `&settings.search` to the same fns (protocol/ordinary.rs:969,:1016,:1071): bindings ignore the page's `search` settings there. web.md:373-378 describes the defaults, gives no reason.
- ordinary.rs:514-537 with_countermodel: SAT model patched onto session outcome (recorded web.md:405-425).
- ordinary.rs:1173 refused("no_countermodel","unsupported") vs protocol/modal.rs:409 InvalidOption key inputs for the same condition; ordinary.rs:1298 refused("no_certificate","unsupported") with no details, library has no_certificate = Error::Unsupported (errors/mod.rs:545,1102). Hand-built unknown_name (calls.rs:1621, ordinary.rs:1150, rules.rs:85, :784, workspace.rs:375, web-render lib.rs:39) lacks details {what,name,known} that the library's UnknownName writes (serialize/errors.rs:70-71); UnknownName variant #[non_exhaustive] (errors/mod.rs:111), so not constructible outside.
- wire::upgrade without `.end()` in web: modal.rs:191,:192; ordinary.rs:638 (model), :1007 (refutation_of), :1170 (countermodel_of); workspace.rs:594 (open). Others call `.end()`. Counts: cli 14 upgrade/13 end, web 20/12, fuzz 8/2, bench 1/0, workspace 8/0. protocol/mod.rs:802 `document` private does exactly read+end.
- rules.rs:38-69 BUILT_IN (13) + NAMES (13) by hand = cli/src/calculus.rs:28-42 (duplicates.md row 113); protocol/rules.rs:207 built_in LK/LJ/LM third. Library has constants, no lookup by name.
- modal.rs:31 ran = lambda.rs:33 ran; Drawing/Text structs twice (modal.rs:109,:130; lambda.rs:79,:99): bindings parse the protocol's {"svg"} / {"text"} value back to unwrap the string -> SVG escaped into JSON then unescaped per call.
- lambda.rs:143-153 printed: term Value -> to_string -> wire::upgrade -> write text+judgement; reduce reads reached term twice (:178 printed, :180 read_term). Protocol's lambda.inhabit/enumerate/reduce/infer write no text/judgement, while linear.term/ordinary.term do (protocol/ordinary.rs:1877 term_value) and lambda.check writes judgement only.
- workspace.rs:461-493 state_of/end_of map State/Cause/End by hand (no Serialize on them: workspace/mod.rs:206,:263,:278); :520-530 map protocol::Cost/Class by hand though they derive Serialize snake_case (protocol/mod.rs:98,:113). cli/src/workspace.rs:523-535 maps State/Cause again (prose).
- never x5 in web (calls.rs:32, ordinary.rs:30, rules.rs:33, modal.rs:25, lambda.rs:27); written x3; session_of x3; sequent_of x2.
- Two-sided "writing or read_one_sided(unbounded) then TwoSided::new": cli/src/show.rs:849-863, web/src/calls.rs:1588-1606, :1853-1879.

## Client (deduxit.github.io, read-only)
- src/worker/deduxit.worker.ts:24-124 one table of exports; not named anywhere in src: describe, candidates/ordinary_candidates/rules_candidates, ordinary_structure, ordinary_certificate, rules_close, rules_hint, lambda_reduce, lambda_infer, modal_model_drawing. Files dated 2026-11-20/21 -> checkout may predate those exports. `about` is called (islands/Panels.tsx:1010, App.tsx:334).

## bench/fuzz skim
- fuzz targets: wire::upgrade 8 sites, end 2. bench/src/tools.rs:370 one.

## Measurements from target/audit/twiggy-top.json (python, stdlib)
- items generic over a bindings `never` (deduxit_web::<module>::never): 202 003 raw bytes (calls 113 886, ordinary 83 710, rules 2 492, lambda 1 915).
- 51 functions have a copy for a bindings stop AND another copy (other stop type): 164 030 raw bytes in the bindings-stop copies; top: export::typst::write 26 899 (2 web copies, 4 others), linear::proofs::fmt::write_text 25 462 (3 web), ordinary::interpolant::interpolant 16 821, linear::search::prove_goal 16 505, export::rocq::write 15 732, export::latex::write 10 653, Formulas::write_first_order 10 144.
- 1 023 names appear twice or more with the identical instantiation: 1 420 840 raw bytes beyond the largest copy (lambda::Node deserialize 2x73 806 etc.): emitted in two crates; wasm-opt's duplicate function elimination likely merges them -> raw attribution overstates; measure after wasm-opt.
- calls::about 24 725, protocol::describe 11 376, web_tab::read 17 217 raw bytes.

## Protocol first-order resolution (F-4c-01 follow)
- serialize/ordinary.rs:1995 `unifier: set.bindings_text(i)`; protocol/ordinary.rs:1700 recomputes per step; bindings_text (resolution/first_order.rs:216-239) writes each term in full, no stop, no bound. Reached from the page through workspace connect (ordinary.resolve).
