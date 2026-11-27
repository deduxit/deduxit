# Unit 8: verdicts

16 confirmed.

## F-8-01: confirmed

Opened core/src/protocol/linear.rs:553-563 (no fallback), web/src/calls.rs:2290-2334 (`Net::of` with the `Placement::Cut` retry, `Net::read` with the `vertices` probe), cli/src/show.rs:1472-1496 (the same retry) and protocol/linear.rs:361-388 (the private `Net` and its probe). I ran the built witness again (target/agents/debug/unit8-net, capped scope): `check: Ok(())`, `linear.net: error code=invalid_net kind=Malformed ... vertex 5 is not the end of one cut link`, `linear.draw` and `linear.eliminate` ok, as quoted. web.md:230-234 records that the bindings follow the command's `net_of`; core-protocol.md records nothing that leaves the fallback out of `linear.net`. Unit 3's verifier confirmed the same reading (F-3-01) without the run. X5 says what the finding says. The bindings' `Net` is internal, so no call or JSON form of the client changes.

Correction: Kind: not a pure refactor: a fix of the protocol's answer (an error becomes a net with boxes, so a `linear.net` output may now have the `vertices` form for a sequent without exponentials) plus a new public type in `linear::nets`; one change with F-3-01. The place of the command's copy is cli/src/show.rs:1482 (the match), 1476 is the `has_exponentials` test.

## F-8-02: confirmed

Opened web/src/calls.rs:447-522: `View::of` calls `invertible_steps(state)` and `cuts_made(state)`, each of which builds `state.derivation_within(&ViewOptions::default(), &Limits::default().with_derivation_bytes(None), never)` and returns `Vec::new()` on an error; `changed` (:538-541) builds the view after every changing call. core/src/limits.rs:125, :262-268 give the default 1 GiB and no work bound; web-tab/src/lib.rs:24 the tab's 96 MiB. core/src/ordinary/interactive.rs:1190 reads the history without a derivation. web.md:171-176 records what the view's `invertible` lists, and web.md:47-49 says `{}` is never the command's gibibyte, so the default limits here go against the recorded intent; no rule gives a reason for them. The S1 part (the wrong bound, the silent empty lists) stands from reading; the time saved is unmeasured and the finding says so.

Correction: Kind: an addition to the library (two methods on the linear `Interactive`), not an API break; the view's JSON keeps its keys, so the client is not touched. The interim fix needs the settings passed into `View::of` and `changed` (they take only the state today, about a dozen call sites). The cost in time is a candidate: measure `view` on a session of a few hundred steps under callgrind.

## F-8-03: confirmed

Opened the five `never` at web/src/calls.rs:32, ordinary.rs:30, rules.rs:33, modal.rs:25, lambda.rs:27 and modal.rs:196-198 (the `&mut dyn` stop with its comment). I recomputed from target/audit/twiggy-top.json with crate hashes stripped: 202 003 bytes in items whose name holds a bindings `never` (calls 113 886, ordinary 83 710, rules 2 492, lambda 1 915), exactly the finding's figures; by my grouping 38 functions and 168 658 bytes have a bindings-stop copy beside another copy (the finding says 51 and 164 030; the grouping rule differs, the order of magnitude and the top rows `typst::write` 26 899, `write_text` 25 462, `interpolant` 16 821, `prove_goal` 16 505, `rocq::write` 15 732 agree). core.md:327-336 records the shim pattern and its 129 KB gain at gzip, so the change follows a recorded practice.

Correction: core.md:335-336 records that `prove_goal` was left generic on purpose ("a change there runs the target set"), so the library half must leave it out or run the focused target set; the bindings half alone (one `&mut dyn FnMut` stop for every call) already merges the bindings' copies with each other and with the protocol's where the types match, and touches no search code. Kind is a refactor of the bindings. The gain at gzip is unmeasured: build the module before and after and read `budget`.

## F-8-04: confirmed

Opened core/src/protocol/mod.rs:801-810 (the private `document`), core/src/wire.rs:444-467 (`upgrade`'s docs tell the caller to call `end()`), and the bindings' reads without `end()`: web/src/modal.rs:190-192 (two), ordinary.rs:636-638, :1005-1008, :1168-1170, workspace.rs:592-594; calls.rs:1708-1725 does call it. My counts of `wire::upgrade` / `.end()`: cli/src 14/13, web/src 20/12, bench/src 1/0, fuzz/fuzz_targets 8/2, as the finding says. No rules file records trailing text as accepted on purpose. The client sends whole documents, so a refusal of trailing text breaks none of its calls.

Correction: `wire` needs only `serialize`, and serde_json is an optional dependency behind `protocol` (core/Cargo.toml:42, :62), so a `wire::read(&str, …)` must sit behind `protocol` (or a feature that brings serde_json); it is an addition, and the behaviour change is the bindings' (six read sites in five functions: modal evaluate's two, `ordinary_model`, `refutation_of`, `countermodel_of`, `workspace_open`). Cost is across crates (cli, web, fuzz, bench) if all 43 sites move.

## F-8-05: confirmed

Opened web-tab/src/failure.rs:42 by its uses: `Failure::refused` at web/src/calls.rs:1621, :1752, ordinary.rs:1150, :1173, :1298, :1314, rules.rs:85, :784, workspace.rs:56, :141, :375, web-render/src/lib.rs:39. core/src/serialize/errors.rs:70-71 writes `what`, `name`, `known` for `UnknownName`; core/src/errors/mod.rs:110-111 marks the variant `#[non_exhaustive]`. `no_countermodel` is no code of the library (grep of core/src finds it nowhere; `no_certificate` is at errors/mod.rs:1102), which breaks web.md:51-55 ("a `code` the library uses"). core/src/protocol/modal.rs:404-413 answers `InvalidOption` key `inputs` for the same condition. The client reads none of `no_countermodel`, `no_certificate`, `unknown_name` or `details.known` by name (grep of src), so the change breaks no use there.

Correction: core-protocol.md ("No new error code") records `InvalidOption` key `inputs` for `modal.countermodel` on purpose, so a new `NoCountermodel` variant goes against a recorded choice: either the bindings take the protocol's code, or the rule is changed with the author. The constructors are additions; the break is the bindings' `no_countermodel` code going away.

## F-8-06: confirmed

Opened web/src/calls.rs:63-160 (the five suites' `name` and hand-written parameters), the five `label()` in core/src/protocol/{linear,ordinary,rules,modal,lambda}.rs ("Linear", "Ordinary", "Custom calculus", "Modal logic", "Lambda-terms": four agree with `about`, modal does not), web/src/protocol.rs:482-503 (`describe` parses `about()` back, merges `SUITES` and adds `catalogue`), and core/src/protocol/mod.rs:700-735 (`statement` parses each suite's parameter with no data naming it). twiggy-top.json gives `about` 24 725 and `describe` 11 376 bytes. web.md:455-459 records what `describe` answers, not a reason for three tables.

Correction: The client's test src/lib/suites.test.ts:33 holds `about`'s `name` to its own `suites.ts` name, "Modal" (suites.ts:116): unify on "Modal" (change the library's label) or the client's table and test change with it. Adding a required method to the public trait `protocol::Suite` is a break of the library (no outside implementor can exist today, F-arch-10). The byte gain is a guess: `about` shrinks only by what the library's data replaces.

## F-8-07: confirmed

Opened web/src/rules.rs:37-90 (the 13 rows, `NAMES` with the same 13 names, and the message listing them as prose), cli/src/calculus.rs:28-42 (the same 13 rows), core/src/protocol/rules.rs:206-214 (`built_in` for LK, LJ, LM) and core/src/rules/mod.rs:102-163 (13 constants, no table). No rules file records a reason for keeping the table out of the library.

Correction: Kind: an addition to the library, not an API break. The protocol's table is three on purpose: it is looked up by a logic's calculus (protocol/rules.rs:369, `from.logic().calculus()`), so it can use the shared lookup without a change of behaviour. The command matches names in any case (calculus.rs:26-27), so the lookup needs that or the command keeps its fold.

## F-8-08: confirmed

Opened web/src/calls.rs:1585-1606 and :1853-1880 and cli/src/show.rs:849-863: the same fallback (`Limits::default().with_occurrences(None)`, `ClassicalSequent::read_one_sided(sequent.clone(), …)`, `.writing()`, then `TwoSided::new`) three times, and the `as_written` test twice in the bindings (the command has `show.as_written(mode)`). core/src/linear/proofs/derivation/classical.rs:425-429 takes `writing: &'a [Written]`. No rule records the fallback as each front end's.

Correction: `TwoSided<'a>` borrows the writing for `'a` (classical.rs:427, :436), and the fallback's writing is owned by a local `ClassicalSequent`: a `TwoSided::of(&derivation, Option<…>)` cannot build it inside without the view owning the record (a `Cow<'a, [Written]>` field) or the caller holding the one-sided sequent. That makes it a change of the type, medium cost, and an addition, not a break.

## F-8-09: confirmed

Opened web/src/lambda.rs:76-110 (`Drawing`, `Text` parsed back from the protocol's JSON), :140-152 (`printed`), :164-198 (`reduce` reads the term twice), :225-231 (`read_term` writes the `Value` to text and parses it), :256-268 (once per listed term), web/src/modal.rs:107-133 (the same `Drawing` and `Text`), core/src/protocol/lambda.rs:494-563 (the lambda arms write the bare documents) and protocol/ordinary.rs:1877-1887 (`term_value`). The round trips are as the finding says. Half of the change is not sound as written, see the correction; the other half (a call that gives a rendering's string) is.

Correction: `linear.term` and `ordinary.term` answer the rendering kind `term` (`{text, judgement}`, never stored, core-protocol.md "Layout"), while `lambda.inhabitation`, `lambda.inhabitants` and `lambda.reduction` are stored wire forms (levels 24 and 30, the command's documents). Putting `text` and `judgement` under `settings.lambda` into them (1) makes a stored value depend on `lambda`, which `INHABIT_READS` and `CHECK_READS` must then list or the workspace's cache answers wrongly (core-protocol.md, "`reads` is a cache key"); (2) breaks the check of `reduces`, which compares the documents; (3) parts the protocol's form from the command's; and (4) does not serve `lambda_infer`, which prints with `with_types(true)` (web/src/lambda.rs:218-219). The sound form is a rendering operation from a lambda term to the kind `term`, plus the typed rendering. The time gained is a candidate (callgrind on `lambda_enumerate` with 32 terms, `modal_draw`). The bindings' answers keep their keys, so the client is not touched.

## F-8-10: confirmed

Opened web/src/ordinary.rs:841, :876, :911, :968, :998, :1076: each builds `deduxit::linear::search::Options::default()` and passes it, while :494 and :568 of the same file pass `&settings.search`; core/src/protocol/ordinary.rs passes `&settings.search` to the same functions. web-tab/src/lib.rs:96-99 sets `copies` none, one job and `classical_work` a sixteenth of the default, so the six calls run under other options than the page's settings. web.md:373-378 records the fact ("the linear search's defaults (one thread, the copy bound 3)") for the transformations with no reason, and says nothing for the normal forms, the clauses or `decide_within` at :1076.

Correction: Two changes of different size: passing `&settings.search` at the six sites is small, in one file, and changes what those calls may answer at a bound (a behaviour change of the page, no JSON form changes, web.md:373-378 to amend); dropping the parameter from the ordinary functions is F-arch-02's break and belongs there.

## F-8-11: confirmed

Opened web/src/workspace.rs:458-493 (`state_of`, `end_of` with `"other"` arms) and :520-530 (the hand maps of `Cost` and `Class`), core/src/protocol/mod.rs:96-119 (both derive `Serialize` with `rename_all = "snake_case"`) and core/src/workspace/mod.rs:203-290 (`End`, `State`, `Cause` derive no serde). The hand maps of `Cost` and `Class` are plain duplicates of the derives. The client reads `state`, `cause` and `node` (src/lib/workspace.ts:19-21, src/islands/Workspace.tsx:224).

Correction: A plain derive does not give the page's form: `State::OutOfDate(Cause)` with `Cause::Input(NodeId)` must write `{"state": "out_of_date", "cause": "input", "node": n}` with `node: null` for `recipe` and `version`, which an internally tagged newtype variant over an integer cannot write, and `End::Refused` embeds the stored error text as raw JSON (`raw(error)`), which needs `RawValue` or a hand-written impl. So `State`, `Cause` and `End` need a small hand-written `Serialize` (or struct variants), with a test that holds the form the client reads; `json!(operation.cost)` and `json!(operation.class)` work as said. An addition, not a break.

## F-8-12: confirmed

Opened core/src/protocol/ordinary.rs:1668-1710 (the check's loop: `clauses.bindings_text(&inference) == step.unifier` per step, the stop not asked inside the loop), core/src/serialize/ordinary.rs:1990-1997 (`unifier: set.bindings_text(i)` on writing), core/src/ordinary/resolution/first_order.rs:214-239 (each binding written whole by `write_term`, no bound, no stop) and unit 4c's verdict on F-4c-01 (confirmed by a run). The protocol's `ordinary.resolve` writes the same document as the command, so the growth reaches it; web.md:38-41 says the tab's only cancel is stopping the worker. I did not run the family through the protocol; the code path is the one F-4c-01 measured.

Correction: It is F-4c-01's defect seen from the protocol, and one fix (the bound in the read-back and in `bindings_text`) serves both; what is the protocol's own is the stop in the check's loop and the bound on the check's texts. Kind: not a refactor (a refusal or an abbreviated unifier changes the level-29 form, as F-4c-01's verifier said). The interim "compare unifiers as terms" is not available: the document holds each unifier as text only (`step.unifier`), so the check has nothing but texts to compare unless it parses them. web/src/workspace.rs:270 only connects a node; the run is the workspace's run call.

## F-8-13: confirmed

Opened core/src/protocol/linear.rs:617-655 (the format match), lambda.rs:526-563 (the same match, its `known: &["latex", "typst"]` without cfg although each arm is behind its feature), modal.rs:385-399 and rules.rs:190-201 (cfg-gated `FORMATS` and a `Format` each), ordinary.rs:737-743 (`Format`, `FORMATS`), lambda.rs:326 (a fourth `Format`), and the `Limit { reason: reason.name(), setting: reason.setting() }` sites at linear.rs:529, ordinary.rs:683, :1097, modal.rs:481, lambda.rs:503, with the resolution bound's mapping at ordinary.rs:1074-1081 and :1607-1614. Counts are as the finding says. No rule records a reason for the copies.

Correction: The default format differs by suite ("text" for linear, "latex" for lambda and modal, which write no text), which the proposed `default` parameter covers; the known list must stay per suite for the same reason, so one cfg-gated list of all formats is not enough: a list per set of formats a suite writes.

## F-8-14: confirmed

Opened web/src/calls.rs:933-946 and web/src/ordinary.rs:403-416: the same loop over `std::mem::take(&mut opened)` with `state.invert(goal, &limits)` and `automatic.add`, differing only in the error's mapping (`Failure::described(&Error::from(e), &state)` against `Failure::from(…)`) and in `changed`'s arguments; `fn written` at calls.rs:1778, ordinary.rs:1343, rules.rs:829; `session_of` at calls.rs:1720, ordinary.rs:1335, rules.rs:821; `ran`, `Drawing`, `Text` in both lambda.rs and modal.rs. The gain is tidiness in the bindings at small cost, as the finding says.

Correction: The two loops are over two session types with two error mappings and two `Automatic` types, so the shared form needs a small trait or a closure for `invert` and for the error; it saves fewer lines than the count of the copies suggests. The helpers `written`, `ran`, `Drawing`, `Text` move to one module without that. Overlaps F-8-03 (`never`) and F-8-04 (`session_of`, `sequent_of`).

## F-8-15: confirmed

Opened web/src/tab.rs:9-22 (the const parser), web/build.rs:9-16 and web-render/build.rs:9 (`STACK_BYTES = 8 << 20` in both), Cargo.toml:36 (`rust-version = "1.92"`; `usize::from_str_radix` is const since 1.82). The witness holds. The value is small: the text parsed is written by the crate's own build script two lines from its constant, so a malformed value cannot reach it; the gain is ten lines less.

Correction: The second half is weaker than stated. web-tab depends on the library (web-tab/Cargo.toml:19), so a build script that reads the size from it would compile the library for the host as a build dependency; and web.md:152-155 already says the renderers' stack is the bindings' 8 MiB. What is left is a comment in each build.rs naming the other, or a constant-only crate. Keep the finding to `from_str_radix`.

## F-8-16: confirmed

Opened web/src/calls.rs:212-218 (`Probe { inferences }`), :1671-1684 (`Ill { hypotheses }`), :2319-2334 (`Boxes { vertices }`), :2457-2470 (`Nodes { nodes }`), core/src/protocol/linear.rs:275-281 (`Claim { hypotheses }`), :328-336 (`Probe { history, links, verdict }`) and :369-388 (`Boxes`). A session's document has both `inferences` and `history` (core/src/wire.rs:140), so the two probes agree today while reading different keys; each site parses the text once for the probe and once for the value. core-protocol.md ("An input of several kinds is told by its keys") records why the protocol probes, not why the bindings probe again by other keys.

Correction: Kind: an addition to the library (a public reader and enum), not an API break; the bindings' `read` keeps its answer, so the client is not touched. It is one change with F-8-01 and F-3-01 (the net with or without boxes is one of the enum's cases). An outcome's document is also what the proof's reader takes (core-protocol.md, `proof_of`), so the order of the key tests is part of the reader's contract and needs a test per form.
