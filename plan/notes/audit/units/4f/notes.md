# Unit 4f notes

## mod.rs (read in full, 2117 lines; tests from 1027)
- mod.rs:954-960 derivation_within takes `view: &ViewOptions` and does `let _ = view;` — the param is ignored (doc 947-948 says "whatever view says"). Ordinary's derivation_within takes the view and uses Structural? check.
- mod.rs:963-1008 estimate + stop + renumber: duplicate with ordinary/interactive.rs:3016 (F-4e-11).
- mod.rs:824-849 `formula` returns `impl Display + '_` but builds a String every time; has `# Panics` with `.expect`. Third copy (F-1b-07).
- mod.rs:882 goals() iterator; 896 goal() Result<&[Member], StepError>; 905 is_complete; 911 steps; 1022 derivation_ids() -> Vec<GoalId> (no Structural param unlike ordinary).
- mod.rs:327-356 GoalId new/get/index; ordinary uses this type (F-4e-03).
- mod.rs:77 StepError: Display messages. "the rule {rule} is not available in {mode} mode" — Mode Display? e.g. "classical affine mode".
- mod.rs:289-292 Eigenvariable message long: "either ... or ..." two causes.
- mod.rs:270-273 Succedents: "a premise would have {n} formulas right of ⊢ instead of one" — n can be 0 ("0 formulas") — reads oddly; and intuitionistic-only, not naming logic.
- mod.rs:267 Split message: "position {position} cannot go to the left premise: it is out of range, repeated, or the formula the rule acts on" — three causes, user can't tell which. Also used by left_positions for At cut point outside goal (mod.rs:430) where "left premise" wording partly fits.
- mod.rs:1028+ tests: close/close_all signatures carry 5 args.

## expand.rs (read in full, 991 lines)
- expand.rs:127-159 candidates: first-order clones whole session once (trial), fine. Propositional uses expand.
- expand.rs:261-304 apply: calls self.open(goal) then self.goal(goal)? again (271) — minor.
- expand.rs:740-775 split_passes doc "It is a cheap test" -> calls focus::counts::split_passes (F-2b-01, check counts.rs:1254); reaches into search::focus (F-2b-03).
- expand.rs:781 undo -> Option<GoalId>.
- expand.rs:837-866 close(goal, options, view, limits, stop); close_framed for fo.
- expand.rs:889 close_with; 930 fill; 949 close_all -> Vec<(GoalId, Result<Closed, Error>)>.
- expand.rs:298 apply: fo path calls framed_premises; witness given on non-Exists -> NoWitness only checked AFTER expand (295) fine.
- expand.rs:972 graft: found.pop().unwrap() — found nonempty.

## invert.rs (558)
- invert.rs:101 invert(goal, &Limits) -> Result<Inverted, StepError>; 108 invert_all(&Limits) -> Inverted. No stop (F-4e-07 other half). Memory bound only.
- invert.rs:182-191: fo framed_premises error -> goal pushed to open and `continue`, but `held` already counted. Also the error is swallowed silently: Inverted.finished is Ok(()) even though a step was refused by bound on terms (WitnessFault::Table) -> doc of `finished` says Ok "when no invertible rule applies to any goal of open" — false then.
- invert.rs:128 is_invertible(&self, sequent: &[Member], step) public; web computes invertible_steps itself (web/src/calls.rs:455).
- invert.rs:36 Options{invertible} — a front-end setting struct in the library; named `Options` beside search::Options (ambiguous name in `linear::proofs::interactive::Options`).

## terms.rs (238)
- proof(limits, stop) -> Result<Proof, Error>; ordinary is `finished`. Third name.

## cut.rs (955), witness.rs (327), handover.rs (810), quantified.rs (1161), replay.rs (483) read in full
- cut.rs:246 cut(goal, &CutFormula, left: &[usize], &Limits) -> Result<[GoalId;2], Error>; uses Error not StepError (because forest rebuilt) fine.
- cut.rs:261-266 split validation duplicate of expand.rs:620-628 (used vec, Split error) small.
- cut.rs:431-449 push premises + record step: duplicates expand.rs:526-546 `record` (base, ids, push Open inference, set rule/principal/premises, history push). cut could call record then set .cut.
- cut.rs:713-715 cut_premises Succedents check duplicates expand.rs:726-731 reading.outputs.
- witness.rs:43-57 tokenizer dup w/ ordinary (F-4e-04).
- witness.rs:206-228 scope() builds a parent vec of all inferences per call: O(n) per ∃ step with given witness and per cut; reader uses intervals (replay_binders). candidate.
- handover.rs:517 `struct Copy<'a>` shadows prelude trait `Copy`; `#[derive(Clone, Copy...)]` at :39, :60 in the same file. Also `struct Written` (:49) beside the public `occurrences::Written`; `enum Bound` (:40) beside public `limits::Bound`.
- handover.rs:761,807 `Back::term(&mut self, forest: &Forest, ..)` then `let _ = forest;` — unused param. Same pattern mod.rs:960 `let _ = view;`.
- handover.rs:90-97 grafted match == expand.rs:858-863 (dup 7 lines).
- handover.rs:374-399 members_of cache returns ms.clone() Vec per member per inference; Place::Prefix => order.clone(). P2 candidate (no journey).
- quantified.rs:959, :984 std::collections::HashSet (SipHash) in same_atom and variables. For "every unit" SipHash list. Not on search hot path (per axiom check in invert; per close write).
- quantified.rs:1006 show() rebuilds the whole shown table after every first-order change (called from make_premises, undo, framed failure, carry_back): O(table) per step → O(n^2) per session; recorded in core-derivations.md:665-667 ("made again after every change") without a cost reason. Candidate.
- quantified.rs:64 constants: Vec<Eigen> with contains in unify closure.
- replay.rs:37 from_parts fine; messages InconsistentSession reasons are full sentences like "a premise must be a later inference and the premise of one inference only" (text ok).
- replay.rs:349 reason "every rule but the axiom and Mix names its principal formula, and those two name none" — rule-as-fact form (like F-4e-18). Message prefix "not a proof in progress: ".

## Outside the unit, checked
- web/src/calls.rs:478-484 cuts_made and :504-521 invertible_steps each build derivation_within with derivation_bytes None per view; :1184-1348 hint/first_order_hint/first_steps/read_steps: the linear hint lives in the bindings; ordinary hint ordinary/interactive.rs:2542, rules hint rules/search.rs:126; ordinary invertible_steps ordinary/interactive.rs:1190.
- cli/src/interact/mod.rs:524-536 replicates "which rule split_passes reads" (at_tensor) because split_passes ignores step.rule (expand.rs:744-760). F-2b-01 confirmed (cost), F-2b-03 (reach into focus).
- settings.rs:12,146 Settings.interactive is linear::proofs::interactive::Options; read by ordinary front ends too (web/src/ordinary.rs:188, cli/src/interact/ordinary.rs:129) -> D4.
- serialize/interactive.rs:33-35 `cut`, :61-64 `instances` keys written but the public rustdoc JSON section (mod.rs:617-635) omits them (S15).
- F-4e-07 (invert stop) confirmed incl. linear: do not re-raise.
- NotAlone message wrong for ax: goal |- a, ax -> "the rule ax needs formula 0 without other formulas in the goal" (expand.rs:660-661, mod.rs:253-256).
- Split/Eigenvariable messages list all causes though call sites know which (expand.rs:623-624, mod.rs:430, cut.rs:263-264; witness.rs:161-167, expand.rs:387-392, cut.rs:525-528).
- invert.rs:182-191 fo framed failure swallowed, finished Ok — doc :79-82 false.
- text.md flags "this session" (invert.rs:113, mod.rs:1018) as narration: false positive (the type is a session).
