# Unit 4f: verdicts

11 confirmed, 1 candidate.

## F-4f-01: confirmed

web/src/calls.rs:478-500 (`cuts_made`) and :504-522 (`invertible_steps`) each call `derivation_within` with `with_derivation_bytes(None)`, and both run from the view at :455-456. `hint`, `first_order_hint`, `first_steps` and `read_steps` are at :1184, :1222, :1282, :1301, with the contract-and-weaken rule at :1176-1179. The library has `invertible_steps` and `hint` for ordinary logic (ordinary/interactive.rs:1190, :2542) and `hint` for rule files (rules/search.rs:126); the linear session's `inferences` is `pub(crate)` (mod.rs:877). web.md:91-103 and :270-278 record what the hint does, not a reason for keeping it in the bindings, and core-derivations.md names no such choice.

Correction: Cost large, not medium: the change adds three public items to the core crate and rewrites about 250 lines of the bindings (two crates). `rules::Interactive` has no `invertible_steps`, so 'the other suites have both' holds for ordinary logic only.

## F-4f-02: confirmed

expand.rs:659-662 returns `NotAlone` for an axiom on a goal that is not two formulas, and mod.rs:253-256 prints 'needs formula {position} without other formulas in the goal'. The same goal with a wrong partner gets `NoDual` (expand.rs:672-674), whose message at mod.rs:257-260 already says 'together with its dual literal and nothing else'. The variant's doc (mod.rs:127-128) says more than the message. Outside the module, nothing names `NotAlone` or the sentence (grep over core, cli, web, docs, skills); the tests at mod.rs:1266 and :1349 pin the variant.

Correction: Returning `NoDual` for the axiom changes the variant the test at mod.rs:1266 or :1349 expects, if that one is the axiom's case; rewording the `NotAlone` arm by rule keeps both tests.

## F-4f-03: confirmed

expand.rs:743-760 is as quoted: the rule is chosen from the formula's kind, and the doc says the step's rule is not read. cli/src/interact/mod.rs:524-536 repeats the choice before calling. The bindings' `split_passes` (web/src/calls.rs:815-821) passes any step through with no guard, so a Mix on a `⊗` formula (which `expand` allows, expand.rs:690-698) is judged by the `⊗`'s premises and can be answered `false` wrongly. core-derivations.md:489-492 and cli.md:489-492 record the behaviour, with no reason for ignoring the rule.

Correction: The signature can stay, so this is a behaviour change of a public function; 'API break' holds only for the `(goal, position, left)` variant. cli.md:489-492 and core-derivations.md:489-492 change with it.

## F-4f-04: confirmed

The two messages are at mod.rs:265-268 and :287-292 as quoted. The raising places each know the cause: expand.rs:620-625 (past the goal, or used, the principal marked at :621), mod.rs:430 (a cut point past the goal), cut.rs:261-265 (past or repeated), witness.rs:161-167 and cut.rs:525-528 (not in scope), expand.rs:387-392 (`Clash::Eigen`). Both variants are `#[non_exhaustive]`, so a field is additive for other crates.

Correction: Kind: the change adds public fields and enums, so it is an additive API change with new text, not text alone. The net misses cli/tests/cli.rs:1452, which pins the eigenvariable sentence word for word. The cut point of mod.rs:430 needs a sentence of its own: 'cannot go to the left premise' does not describe it.

## F-4f-05: confirmed

The `# JSON` section of `Interactive` (mod.rs:617-635) lists `sequent`, `rule`, `principal`, `premises`, `history` and `written`. The private forms write `cut` (serialize/interactive.rs:33-35) and `instances` (:61-64), whose keys are documented only on the private `InstancesForm` (:67-83). Neither word appears in mod.rs's docs (grep), and core-derivations.md records no choice to leave them out. S15 asks every JSON form on its type.

## F-4f-06: confirmed

The signatures are at the lines given: mod.rs:882, :896, :905, :911, :1022; expand.rs:781, :949; terms.rs:38; invert.rs:101, :108, :128. `finished` is at ordinary/interactive.rs:3188 and rules/interactive.rs:1504, and only the linear session calls it `proof`. The verifier of 4e confirmed F-4e-01 (4e/verdicts.md:5-9), so this is its third column.

Correction: Cost large, as 4e's verdict set for F-4e-01: three sessions, the command, the bindings and the protocol. `ordinary::Interactive` also says `is_complete` (ordinary/interactive.rs:1173); only the rule-file session says `is_finished` (rules/interactive.rs:430), so that rename follows F-4e-01's choice, not a two-to-one majority. C-WORD-ORDER fits loosely.

## F-4f-07: confirmed

invert.rs:182-191 catches every error of `framed_premises`, leaves the goal open and goes on; the run then returns `finished: Ok(())` (:199-203), which the doc at :79-82 says means no invertible rule applies to any open goal. `framed_premises` refuses with `WitnessFault::Table` at the session's bound (expand.rs:331-334). `held` is added at :181 before the step is tried. core-derivations.md:624-633 records the bound for a step, `close` and the reader, and says nothing of the invertible run. `Refusal::Terms` exists (handover.rs:472).

Correction: Kind: a behaviour change with a doc fix, no signature changes. The command's line is cli/src/interact/mod.rs:838, not :836. The catch also covers the eigenvariable count past `u32`, which needs a refusal as well.

## F-4f-08: confirmed

The open goal's `Inference` literal is at expand.rs:531-539, cut.rs:434-442 and mod.rs:719-727, and the bookkeeping around it at expand.rs:525-546 and cut.rs:431-450. The graft's error mapping is at expand.rs:858-863 and handover.rs:92-97, the same four arms. The goal's order is at handover.rs:110-115 and :317-322, the same sort. The predicates line is at witness.rs:42, :156, handover.rs:161 and quantified.rs:576. `record` sets `principal` from `has_principal`, so `cut` can call it and set `cut` after.

Correction: The quoted `let ids = [InfId::new(base), InfId::new(base + 1)];` is cut.rs:432; `record` has a `Vec` at expand.rs:527-529. The step number is a `let` at expand.rs:325, :368, :798 and handover.rs:268 and inline at cut.rs:396, under two wordings of the comment. The split checks differ: expand.rs:620-621 marks the principal as used first and cut.rs:261-265 has none, so a shared `chosen` takes an optional principal.

## F-4f-09: confirmed

core/src/settings.rs:11-12 imports `crate::linear::proofs::interactive` and :144-146 has the field of type `interactive::Options` (invert.rs:36-41). Ordinary logic reads it at cli/src/interact/ordinary.rs:129 and in the bindings. core-derivations.md:781-782 names the type and gives no reason for where it lives. It goes with F-4e-03, which 4e's verifier confirmed.

Correction: Depends on F-4e-03's shared module; alone it only moves the type into `settings`. A re-export from the linear session's module keeps the old path, so the break can be avoided.

## F-4f-10: candidate

quantified.rs:1006-1086 remakes the whole shown table (every term, every frame, a clone of the members, every size) per call, and it is called after every first-order step, failed step, undo and graft (expand.rs:347, :399, :435, :807; handover.rs:467). witness.rs:206-212 builds a parent per inference on each call (witness.rs:152, cut.rs:516). The reader replaced the same walk for the same reason (replay.rs:211-213, core-derivations.md:707-709). core-derivations.md:630 and :665-667 record that `show` exists and what it counts, not that it is whole per step. Reading supports quadratic time; nothing measures a session.

Correction: To measure: a callgrind journey of n `∀`/`∃` steps through `apply` at n = 1 000 and 10 000 (with open and with given witnesses), instructions against n. An incremental `show` must still recount `largest` and remake the terms a binding changes; a failed step's `show` (expand.rs:347) follows `back_to`.

## F-4f-11: confirmed

replay.rs:122-125, :129-132 and :347-350 and cut.rs:615-619 are as quoted, under the prefix 'not a proof in progress: ' (errors/mod.rs:280). The first three state the form's rule where the document breaks it; the reader's other faults name the defect (cut.rs:611, :623). No test, lock or manual page matches the five sentences outside the module (grep over core, cli, web, docs, skills). 4e's verifier confirmed the same pattern in part (F-4e-18).

Correction: The three sentences of replay.rs are the strong cases. 'an ordered mode has no cut' and 'a cut has two premises' are short and nearly say the fault, so they are the weaker half. The criterion is nearer S18 (an error says what went wrong) than T1, as for F-4e-18.

## F-4f-12: confirmed

quantified.rs:959-960 and :984 use `std::collections::HashSet` with `insert` alone, and :16 imports `crate::hash::HashMap`. No `clippy.toml` or `disallowed-types` settles it (4e's verdict on F-4e-14 found none and named these two sites).

Correction: As for F-4e-14: the lint can be turned on only once the other units' sites go too, and neither site is on a hot path, so the gain is one hasher in the file.

## Beside the findings

- core/src/linear/proofs/interactive/expand.rs:756-760 with web/src/calls.rs:815-821: the bindings' `split_passes` passes any step to the library, which judges a Mix on a `⊗` formula by the `⊗`'s premises. The answer `false` ('this split cannot be closed') can then be wrong for the Mix, and the page greys a split that may close. The command guards against it (cli/src/interact/mod.rs:526-536); the bindings do not. Read from the code, not run. It is F-4f-03's defect seen from the page.
- core/src/linear/proofs/interactive/invert.rs:187-203: when a first-order session is at its bound on terms, `invert` and `invert_all` skip the `∀` step and return `finished: Ok(())`, against the field's doc. It is F-4f-07's defect: a wrong report, no unsound step.
