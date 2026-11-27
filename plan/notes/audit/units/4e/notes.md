# Unit 4e notes

## goal_tree.rs, unify.rs, invert.rs
- goal_tree.rs:13 `use crate::linear::proofs::InfId;` root module shared by linear+ordinary names linear's InfId (deps.md: goal_tree names linear 1 path). Shared id belongs at root (F-arch-01 style).
- unify.rs:15 `use std::collections::HashSet;` SipHash; used in unify_pairs:208 `met` and bind:251 `seen` — per unification call, on every axiom step w/ witnesses (hot-ish for a session, not search).
- invert.rs:15-16 ordinary session uses `linear::proofs::GoalId` and `linear::proofs::interactive::Inverted` -> ordinary public API leans on linear suite (D4). rules session has its own `rules::GoalId` (rules/interactive.rs:19) -> two GoalId types; ordinary borrows linear's.
- invert.rs:68 `is_invertible(&self, (left, right): (&[NodeId], &[NodeId]), step: &Step)` tuple param.
- invert.rs:109 `invert(goal, &limits)` no stop; bounded by memory only.

## rules/interactive.rs
- :416 goals() -> Vec<GoalId>; :430 is_finished; :439 goal() panics on foreign id (returns slice, no Result); :450 members, :465 hypotheses, :560 lines panic too.
- :750 verify(): every step builds the whole derivation `self.derived()` (clones Terms: derived_with :665 `out.adopt_terms(self.terms.clone())`) then check_node -> O(session) per step, O(n^2) session. verify_all likewise (assume, cite, derive).
- :759-762 and :771-774 identical map_err closure (InvalidDerivation -> StepFault::Check).
- :820 candidates: per rule, per choice (<=4096) `self.clone()` (whole session: calculus+terms+nodes) then plan (clones Calculus :1516) + give (clones calculus :1783) + fresh (:1882 clone) + commit->verify (derived, clones terms). No stop parameter; no work bound other than RULE_CHOICES.
- :925-935, :1032-1037/:1059-1063, :1136-1141/:1186-1190, :1860-1871: rule lookup by name + Unplanned->Error mapping repeated 4x.
- :1389 undo -> Option<Undone>; :1504 finished(stop) -> Result<Derivation>.
- from_parts :2209-2213 per scope counts scopes with same owner -> O(scopes^2); :2179 per node scans scopes; :2319-2323 per history entry scans all scopes -> O(history*scopes). Rules files say the linear/ordinary readers are linear; this one isn't.
- :2342 remove(): Vec::remove in loop.
- :1393 undo of a line: line_places() whole sort per call. :572 in_force sorts all lines per cited line.
- InconsistentSession reasons: "a session has a node and a scope", "the nodes are no tree with its root first" (T1 wording).

## ordinary/interactive.rs 1-3120
- :28 `use crate::linear::proofs::{GoalId, InfId, Size, Structural, ViewOptions}; use crate::linear::search;` — ordinary session's public API: GoalId (linear), close/hint take `&search::Options` (linear, F-arch-02), Structural/ViewOptions linear. StepError::NoGoal{goal: GoalId} :50.
- three step error types: ordinary::StepError (own enum :45), rules uses Error::StepRefused{StepFault} + Error::StepNeeds, linear has linear StepError. close() errors with Error::OrdinaryStep.
- Display texts :235-245, :258-264 say "every goal of LJ has exactly one" also in minimal logic (LM) (StepError::Logic message for Logic::Minimal + WR etc.; Succedents in LM). T1 inaccuracy.
- doc example :596 `state.goals().next().unwrap()` (C-QUESTION-MARK) minor.
- :1241-1243 widths(): doc "No step adds a formula." but quantifier steps add instances (invert.rs:144 "A quantifier step adds its instance to the arena" recomputes). stale doc.
- :1274 rules(goal, side, position) & :1326 candidates(...) duplicate the shape->Needs match (:1290-1302 vs :1334-1344).
- :1144 goals() iterator; :1159 goal(id)->Result<(&[NodeId],&[NodeId]),StepError>; :1173 is_complete; :1179 steps; :2394 undo()->Option<GoalId>; :2508 close(goal, translation, &search::Options, &Limits, stop)->Result<Closed,Error>; :2542 hint(&self, same)->Result<Hint,Error>; finished(&Limits, stop) (see later); :1600 apply(&mut, GoalId, &Step)->Result<Vec<GoalId>,StepError>.
- :1190 invertible_steps: closed_invertibly does `self.history.iter().position(|&h| h == id)` per history entry -> O(h^2).
- :2025 scope(): builds a parent vector over all inferences per call (O(n)); called by unifies (per partner y loop calls unify_axiom with arena=true: arena_terms + holds over all terms -> O(side*arena)), held, open_witness, witness.
- :2719-2729 vs :2784-2795 duplicate (duplicates.md 133 tokens): GoalMismatch check + principal remap + clone_from of sides.
- largest-eigenvariable+1-or-refusal pattern 4x: :1953, :1975, :2833-2838, :3088-3096; Refusal::Index{Space::Term, u32::MAX+1} built twice (:2834, :3082).
- :2636 ordinary code writes `crate::linear::search::Reason::Incomplete`; docs :2490 link it.

## ordinary/interactive.rs 3120-5900, comparisons
- code ends at :3999; :4000-5900 tests (1 900 lines, a third of the file). Seams for F-arch-20: errors :34-372; vocabulary :374-572; witnesses/unify :658-960 + quantifier steps :1621-2162; state & queries :1069-1470; expansion :1472-1619,:2164-2389,+free fns :3204-3391; undo :2391-2473; search close/hint/graft :962-1056,:2475-2987; views :2989-3201,:3393-3404; reader :3406-3999 (serialize); tests.
- test comment :4912-4915 scrambled (two comments interleaved).
- :3030-3035 and :3123-3126 postorder renumber twice; linear mod.rs:954-990 same estimate/stop/renumber (duplicates.md 169 tokens).
- witness text parser: ordinary :2061-2137 == linear witness.rs:44-120 (tokenizer verbatim, same stack parser, same WitnessFault variants) (duplicates.md 115 tokens).
- ids_of :2654-2672 == ordinary/written/mod.rs:776-788 and does what transfer::copy(target, source, None, &Parameters::Constants) does for propositional arenas (transfer.rs:74, "otherwise every atom is copied by its name alone").
- Error shapes: no open goal -> linear Error::Step(StepError::NoGoal) code "step"; ordinary Error::OrdinaryStep(StepError::NoGoal{goal: GoalId}) code "step"; rules Error::StepRefused{StepFault::NoGoal{goal: u32}} code "step_refused" (errors/mod.rs:302,309,722,1073,1121).
- rules goal()/members()/hypotheses()/lines() panic on foreign id; cli/src/interact/rules.rs:247-252 writes its own guard + copy of "there is no open goal N".
- web/src/ordinary.rs:25 `use deduxit::{..., linear::GoalId, ...}` to drive the ordinary session; ordinary/mod.rs:132 re-exports no GoalId; invert.rs:16 returns linear Inverted whose docs link the linear Interactive::invert/undo.
- names: is_complete (ordinary :1173, linear mod.rs:905) vs is_finished (rules :430); derivation_ids(structural) (ordinary :3115) / derivation_ids() (linear mod.rs:1022) vs derivation_goals() -> Result (rules :1484); finished(&Limits, stop) (ordinary :3188) / finished(stop) (rules :1504) / proof(&Limits, stop) (linear terms.rs:38); undo -> Option<GoalId> vs Option<Undone>; goals iterator vs Vec; linear has no hint (bindings do it); ordinary has invertible_steps in core, linear's in web/src/calls.rs:455.
- invert/invert_all (ordinary invert.rs:109,116 and linear invert.rs:101,108) take no stop; bounded only by memory_bytes (default 1 GiB); LK `|- a1/\b1,...,an/\bn` doubles goals per step. S1.
- rules candidates no stop; per choice self.clone() + plan/give/fresh each clone Calculus (472 B + form + maps) + verify->derived clones Terms. No journey covers a session (ratchet.out) -> candidate.

## Chosen findings (18)
01 op names table; 02 rules goal panics; 03 linear ids in ordinary/rules sessions; 04 witness parser dup; 05 seams; 06 three error shapes; 07 invert no stop; 08 LJ in LM text; 09 rules per-step cost (cand.); 10 ids_of vs transfer::copy; 11 derivation_within dup; 12 ordinary local dups; 13 rules local dups; 14 unify SipHash; 15 widths doc; 16 rules reader quadratic (cand.); 17 scrambled comment; 18 rules reader messages.
