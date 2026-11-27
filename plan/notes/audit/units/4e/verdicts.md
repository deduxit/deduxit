# Unit 4e: verdicts

16 confirmed, 2 candidate.

## F-4e-01: confirmed

Every signature in the table is at the line given: ordinary/interactive.rs:1144, 1159, 1173, 1179, 1274, 1326, 1600, 2394, 2508, 2542, 3115, 3188; rules/interactive.rs:416, 425, 430, 439, 787, 820, 923, 1389, 1484, 1504; rules/search.rs:84, 126; linear/proofs/interactive/mod.rs:882, 896, 905, 1022 and expand.rs:781, 837. core-rules.md records no reason for `is_finished`, `goals()` as a `Vec` or `derivation_goals`, and the arch unit's verdict on F-arch-12 already confirmed the disagreement. One part of the change does not work as written: `rules::Undone` (rules/interactive.rs:197-205) has `Removed(LineId)` for a forward step taken back, which reopens no goal, so `Option<GoalId>` cannot be the one return type of `undo`.

Correction: Cost large, not medium: the renames touch three sessions in three modules, the command (cli/src/interact/), the bindings and the protocol. `undo` cannot take one return type unless the other two sessions adopt an enum like `Undone`; keep `Option<Undone>` for the rule-file session or say so. C-WORD-ORDER fits only loosely (the matter is one name per operation across sibling types); the place of the second `derivation_ids` is as given.

## F-4e-02: confirmed

rules/interactive.rs:434-441, 447-456, 462-465 and 557-568 index `self.nodes[goal.index()]` and document the panic; `open` (:595-600) already builds `StepFault::NoGoal`. cli/src/interact/rules.rs:247-252 writes its own copy of the sentence, against core.md:394-396 (a front end never builds an error of the library). core-rules.md records no reason for the panic. But the change as written breaks two callers: cli/src/interact/rules.rs:486 and web/src/rules.rs:257 read `state.goal(GoalId::new(0))`, the root's sequent, which is closed once the first step is taken; and `in_force` (:582) and `search.rs:312` call `lines` on goals already checked. The accessors today answer for any node of the session, open or closed.

Correction: Refuse only an id past the session's nodes (or keep a separate accessor for the root's sequent), not every goal that is not open; otherwise the root read at cli/src/interact/rules.rs:486 and web/src/rules.rs:257 fails after the first step. The command has one guard of its own (:247), not one per call. Cost small to medium (core, cli, web callers).

## F-4e-03: confirmed

ordinary/interactive.rs:28-29, ordinary/invert.rs:15-16, goal_tree.rs:13 and web/src/ordinary.rs:25 are as quoted; ordinary/mod.rs re-exports no `GoalId` or `Inverted` (grep), while rules/mod.rs:82 exports its own `GoalId` (rules/interactive.rs:17-33, `new` and `index`, no `get`). core-ordinary.md:1749-1750 only states that `Inverted` is the linear session's type, with no reason; core-derivations.md:407 gives none for where `GoalId` lives. The arch unit's architecture.md asks for the same move. Type moves only; the JSON stays a number (`serde(transparent)`).

Correction: The ordinary `GoalId` has `get` and the rule-file one `index` returning usize and derives `Serialize`, `PartialOrd`, `Ord`: the shared type needs the union. `Reason::Incomplete` at :2636 belongs to F-arch-02, not to this move.

## F-4e-04: confirmed

The tokenizer at ordinary/interactive.rs:2061-2075 equals linear/proofs/interactive/witness.rs:43-56 line for line, and both parsers keep `open` and `done` and answer `WitnessFault` (errors/step.rs:32, already at the root). They are not the same beyond that: the linear reader resolves a name when it meets it (unknown symbol, predicate and arity-without-brackets refused at the name), the ordinary one at the closing bracket through `applied` (:2142), and the ordinary one counts names against `self.terms` and checks an eigenvariable's scope. A shared reader over two closures can serve both, but which fault a text with two errors gets may change for one session.

Correction: Behaviour is kept only for well-formed text and single faults; for a text with several faults the first one reported can differ, so the net needs a test per session on such texts (or the shared reader keeps a hook at the name as well as at the application). The witness tests named exist only for the ordinary side by those names.

## F-4e-05: confirmed

The file has 5 900 lines (`wc`), the `impl Interactive` starts at :1069, the reader behind `serialize` runs :3407-3999 and the tests from :4000; the item list read by `grep` matches the seams given (errors :39-372, vocabulary :380-572, `Made`/`Witnesses`/`unify_axiom` :660-960, `Decided`/`Carry` :963-1056, close/hint/graft :2508-2987, views :2995-3201). The linear session is a directory of nine files (13 540 lines). The arch unit's verdict on F-arch-20 confirmed the same split; this adds the seams. Gain is for readers alone.

## F-4e-06: confirmed

errors/mod.rs:302, :309, :722-725, :1073 and :1121 are as quoted; `StepFault::NoGoal { goal: u32 }` (errors/step.rs:211-215) against `ordinary::StepError::NoGoal { goal: GoalId }` (ordinary/interactive.rs:48-51). core-rules.md:335 and :449 record that the code is `step_refused` but give no reason for it differing from `step`, and `step` already covers two shapes (serialize/errors.rs:113, :115). The finder missed the reach: `Error::StepRefused` is also the refusal of a resolution step by hand (ordinary/resolution/refutation.rs:143-146), and the code is in the manual's schema (docs/src/wire.schema.json:622), the guide (skills/deduxit/reference.md:704, :2350), six assertions of web/tests/rules.rs and ordinary.rs:583, and the docs of web/src/rules.rs.

Correction: Cost large, not medium: the wire schema, the guide for AI models, the bindings' tests and docs and the client page that reads the code all move, and a stored or scripted reader of `step_refused` breaks. The code is not only the rule-file session's: resolution by hand answers it too. The gain is modest (should-fix under S18, no refusal read as a verdict).

## F-4e-07: confirmed

ordinary/invert.rs:109 and :116 take `&Limits` alone and the only early exit is the memory bound at :149-151; linear/proofs/interactive/invert.rs:101 and :108 have the same signatures. core-derivations.md:829-839 and core-ordinary.md:1728-1741 record the memory bound as the answer to the 2^n goals, and say the web bindings pass the smaller of the memory and derivation bounds, but give no reason against a stop; S1 asks a stop of every call that can run long. So the run is bounded, not unbounded, yet under the command's default of 1 GiB it can take long with no way to end it but the bound.

Correction: Cost medium, not small: two sessions, the command (cli/src/interact/mod.rs:834, :863; ordinary.rs:420, :436) and the bindings (web/src/calls.rs:582, :891, :940, :960; web/src/ordinary.rs:189, :410, :432) all pass the new argument, and both rules files' descriptions of the call change. The claim's 'no stop' holds; 'bounded only by memory' is the recorded design, so the finding is the missing stop alone.

## F-4e-08: confirmed

ordinary/interactive.rs:233-245 and :258-263 say 'every goal of LJ', and :1515-1517 refuses `WeakenRight | ContractRight | OrRight | FalseRight` whenever the logic is not classical, so minimal logic prints it: docs/src/ordinary-logic.md:1216 shows 'the rule ⊥R is not available in minimal logic: … every goal of LJ has exactly one'. `Logic::Intuitionistic` is documented as 'whose derivations are LJ's' and `Minimal` as intuitionistic logic without ex falso (ordinary/mod.rs:170-174), so the sentence is not wrong, but a learner in minimal logic was never told that. core-ordinary.md uses 'LJ' for both without recording the wording as a choice.

Correction: The sentence is also pinned at cli/tests/cli.rs:1070 and shown at docs/src/ordinary-logic.md:1216; both change with it, the manual in the same commit. The reader's message is at :3543.

## F-4e-09: candidate

`let mut trial = self.clone();` is at rules/interactive.rs:868 inside the loop over choices (up to `RULE_CHOICES`); `calculus` is an owned `Calculus` field (:301, a `#[derive(Clone)]` struct of maps and vectors, calculus.rs:264), cloned whole at :1516, :1783 and :1882; `verify` (:750-751) calls `derived`, whose `derived_with` starts with `self.terms.clone()` (:665). core-rules.md:316-334 describes the clone and the whole-session conversion as how the step is checked but gives no reason that a measurement could not overturn. Reading supports the cost growing with the session times the choices; how much it is in practice is not known.

Correction: To measure: a callgrind and a DHAT journey that starts a session in `rules::LK` on a goal of about ten members and calls `candidates` once, and one that applies about a hundred steps in `rules::NK`, before and after. The ordinary session's cheaper trial (one lazy clone, then apply and undo, ordinary/interactive.rs:1357-1360) is the model; an undo in place of the clone must still leave `self` untouched (`candidates` takes `&self`), so one clone per call remains.

## F-4e-10: confirmed

ordinary/interactive.rs:2654-2672 and ordinary/written/mod.rs:771-786 are the same loop and match. `transfer::copy` (transfer.rs:74-197) with `roots: None` and `Parameters::Constants` on a source without binders or terms takes the arm `Node::Atom(a) => target.atom(&name)` with `apart` returning the name unchanged (:209-211), and adds every other node as the two loops do; `Copied::node` reads the id. `transfer` is `pub(super)` and behind `interactive`, which `written/` also needs, so both callers reach it. core-ordinary.md:1892 mentions `ids_of` without a reason to keep it apart.

Correction: `copy` also builds `Copied::names` and runs `reached` over the source, a small extra cost per graft and per tableau review; negligible, but not nothing. Cost small, though it is two files.

## F-4e-11: confirmed

ordinary/interactive.rs:3017-3035 and linear/proofs/interactive/mod.rs:972-989 are the same refusal block, the same stop and the same renumbering; the renumbering is written again at ordinary/interactive.rs:3123-3126 (there filled with `InfId::new(0)`, not `u32::MAX`). `goal_tree.rs` already holds the shared `postorder` (:29). The estimates before the refusal differ by session, so only the refusal and the renumbering are shared. Tidiness at small cost.

Correction: The gain is small: about fifteen lines in two places and four in a third. The helper for the refusal takes the estimate as an argument, since each session computes its own.

## F-4e-12: confirmed

The root's fitting is twice, ordinary/interactive.rs:2720-2729 and :2786-2795, word for word; the shape-to-needs match twice, :1294-1300 and :1336-1342. The eigenvariable part is overstated: :1953 and :3095 take the maximum over the arena's term table, :1975 and :2833 over `largest_eigenvariable` of the inferences, and the refusals are `WitnessFault::Eigenvariables` at :1954 and :1976 but `Refusal::Index` at :2834-2838 and :3081-3086. So they are two pairs with two sources and two errors, not four copies of one block.

Correction: One helper cannot serve the four eigenvariable sites: at most one for the pair over `largest_eigenvariable` and one for the `Refusal::Index` value built twice. The other two blocks are as claimed.

## F-4e-13: confirmed

The lookup by name with `StepFault::NoRule` is at rules/interactive.rs:925-929, :1032-1036, :1136-1140 and :1860-1864; the `Unplanned` match at :931-935, :1059-1063, :1186-1190 and :1867-1871; the checker's refusal mapped to `StepFault::Check` at :759-762 and :771-774. `apply`, `derive` and `extend` clone the rule after the lookup (they go on to borrow `self` mutably), `premise_bindings` borrows it, so a helper returning `&Rule` serves all four only if the three callers clone its result. core-rules.md records nothing against it.

Correction: Ten places, not seven (four lookups, four mappings, two closures).

## F-4e-14: confirmed

unify.rs:15 imports `std::collections::HashSet`, used at :208 and :251 with `insert` alone; the ordinary session uses `crate::hash`. No `clippy.toml` exists and `Cargo.toml` has no `disallowed` entry, so no lint settles it; core.md names `hash` as the crate's tables without excepting the unifier. The same two sets stand with SipHash in the linear session's copy (linear/proofs/interactive/quantified.rs:959, :984), and other sites remain across the crate (first_order.rs:598, ordinary/instance.rs:771, ordinary/model.rs:299, modal/model.rs:273, linear/proofs/size.rs:30), so this change alone does not let the lint be turned on.

Correction: The value as stated ('the lint cannot be turned on while this site remains') holds only together with the other units' sites; here it is one hasher for the unifier, not on a hot path.

## F-4e-15: confirmed

ordinary/interactive.rs:1241-1242 says 'No step adds a formula.'; ordinary/invert.rs:144-147 recomputes the widths because 'A quantifier step adds its instance to the arena', and `instance_step` calls `self.formulas.instantiate(body, term)` at :1833-1835. core-ordinary.md:2038 also says the invertible run recomputes them. The comment is false since first-order steps exist.

## F-4e-16: candidate

The three scans are there: rules/interactive.rs:2179-2182 (`position` over the scopes per node with a discharging parent), :2208-2212 (a `filter().count()` over every scope inside the loop over scopes) and :2319-2323 (the same per history entry of a rule node). Each discharging step opens a scope, so a document of n such steps is read in about n² comparisons. core-rules.md records fixes of other quadratic scans (:87, :122, :254) and nothing in favour of these. The work is bounded by the document's size, so this is no hang under S5; whether n² matters at sizes the limits admit (10 000 scopes is about 10⁸ comparisons) is for a measurement.

Correction: Criterion: P (the reader's cost), not S5: the work is bounded. To measure: instructions under callgrind for reading a session of 1 000 and of 10 000 discharging steps in `rules::NJ`, and whether the limits admit a document that large at all.

## F-4e-17: confirmed

ordinary/interactive.rs:4912-4915 is as quoted: lines 1 and 4 are one sentence ('undone and taken / again, it names `a` again'), lines 2 and 3 another about `∃y. p(y)`. The test below (:4916-4950) starts on `(exists x. p(x)) \/ (exists x. p(x)) |- exists y. p(y)` and looks formulas up by `∃x. p(x)` on the right, which is what the second sentence explains, so it still applies and should be kept as its own comment.

Correction: Keep the second sentence (it explains why the test finds the right side's formula as `∃x. p(x)`); do not drop it.

## F-4e-18: confirmed

rules/interactive.rs:2077-2078 answers 'a session has a node and a scope' when the document has none, which under the prefix at errors/mod.rs:280 reads as a statement of the rule, where the reader's other faults state the defect ('a premise of a node without a rule', :2173). That half holds. The second half is weaker: 'the nodes are no tree with its root first' (:2151, :2162) says what is wrong, and 'is no' is the wording the project uses throughout (its rules files and messages); no criterion of the writing rules names it. The same sentence stands at core/src/serialize/ordinary.rs:1558. No test or lock matches either sentence outside the file (grep over core, cli, web, docs, skills).

Correction: Confirmed for :2078 alone. The 'no tree' sentence is taste; if it is reworded, core/src/serialize/ordinary.rs:1558 has the same text and changes with it. The criterion is nearer S18 (an error says what went wrong) than T1.
