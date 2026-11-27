# Unit 1c notes

## cut/mod.rs (read in full)
- :332 `step` and :426 `eliminate` generic over `stop: impl FnMut` (and `each`), bodies ~37 and ~75 lines, not shims (core.md rule: long body takes &mut dyn). They pass &mut stop to Record::of (dyn) and reduce (dyn) already.
- :341-352 vs :464-472: the "at must be a Cut, else NoCut; else strategy.pick" match written twice (step, eliminate).
- :357-368 step checks the made proof with check::examine by hand; eliminate checks via Record::of next round. Two ways.
- :602 reduce calls `check::reading(proof, mode).unwrap_or_default()` again though Record::of (:546) computed it for the same proof; step :355 computes reading of made; eliminate's next Record::of computes it again then reduce again -> reading computed 2x per proof per step. Candidate (perf).
- :514 Record::zones HashMap<NodeId, Multiset> keyed by dense ids where `kept: Vec<bool>` already indexes: Vec<Option<Multiset>> would do (minor).
- :1024, :1116 `node.premises().collect::<Vec<_>>()[..]` to destructure one premise: allocation; idiom minor.
- :1502 `witness` builds a HashMap for one pair.
- :234 `Case::name` but no Display impl for Case (C-COMMON-TRAITS minor). Strategy has one variant.
- :643 unreachable! on Proof::new error other than Refused.
- Step::formula is a Member of proof taken on (rules say so).
## cut/framed.rs (read in full)
- Three copies of the memoised postorder walk over FoTerm (stack of (id, expanded)): names_term :311-342, substituted_term :412-449, Transfer::term :613-641.
- Three copies of frame-chain walk (collect chain up to memoised, then rebuild downwards): names_member :291-305, substituted_frame :384-406, Transfer::frame :645-664.
- :140 `OccId::new(o.get() + 1)` = a binder's body is the next occurrence: forest numbering invariant used by arithmetic.
- :substitute uses HashMap<NodeId,NodeId> `made` where a Vec indexed by id would do (changes is a Vec already).
- apart: up to 4 `reached` passes O(n) per round; stop per round. OK.

## search/refutation.rs (read in full)
- :1173 `Disproof::check(&self)` takes no limits and no stop; builds `Forest::new` (default limits) :1175; calls saturated::check and basis::check with `&mut |_| false` :1210, :1229, then `unreachable!` on Stopped. Doc :1131-1135 says "The check has no stop ... So a caller checks such a set within a time limit of its own." Callers: cli/src/check.rs:454-456 runs it on a thread via `deadline.within` (comment "A saturated set's check has no stop of its own"; the thread is left running), core/src/protocol/linear.rs:716 `disproof.check()` with `limits` and `stop` in hand (it checks the proof at :712 with check_within(…, limits, stop)), web/src/calls.rs:1767 (wasm, no thread). Proof has check_within (proofs/mod.rs:927). -> FINDING (top): add Disproof::check_within(&limits, stop).
- Constructors: Saturated::new :297, Stable::new :285, Basis::new :350, Marking::new :362 exist; Classical, Unbalanced, OneSided, Equation, StateEquation, UnbalancedSymbol, OneSidedSymbol are #[non_exhaustive] structs w/ pub fields and NO constructor (grep: no impl). So outside engine can build only Exhausted (uncertified), Saturated, Basis. `Disproof::of_goal` :921 pub(crate): outside can't refute a goal. -> FINDING (X4).
- Handover: what each promises. Exhausted: rests on search; check answers Uncertified (kind unsupported). Others: certificates checked by Disproof::check. Docs on `Refutation::Exhausted` :60 say "Every way to prove the sequent was tried and failed" — no word that it is uncheckable/unverified or that anyone can build it; `Disproof::new` :909 doc "Returns the disproof of a sequent in a mode." says nothing that the value is unchecked.
- Duplicate: check_unbalanced :1301-1316 vs check_one_sided :1425-1440 (range + open predicate block verbatim). check_balance :1379-1404 vs check_copies :1508-1541 (the span walk; differs at Bang|Quest arm and checked_add). duplicates.md lists both pairs.
- write_unbalanced :746-782 (Two) vs :783-817 (Intuitionistic): same (more,fewer), (place,copies) code twice; only the parenthesis suffix differs.
- `Refutation::NAMES` :370 and `name()` :384 are two lists of the same words.
- `crate::linear::occurrences::Forest` spelled in full in 8 signatures (no `use`).
- check_equation :1628 `as i64` casts.
- write_unbalanced takes `(copied, predicate): (Option<Sign>, bool)` tuple of a bool – C-CUSTOM-TYPE (private, minor).

## saturated.rs, weighted.rs, basis.rs, classical.rs (read in full)
- saturated.rs:81 and basis.rs:58: the same `Failed { Fault(RefutationFault), Stopped }` + From impl twice; matched twice at search/mod.rs:1087-1104 and refutation.rs:1210-1236.
- Both checks take `stop: &mut dyn FnMut(u64) -> bool` (units); search/mod.rs:1081-1086 wraps it into Progress + limits.work — the logic a Disproof::check_within would hold.
- Horn reading twice: weighted.rs:284-289 `head` == basis.rs:124-129; factor loop weighted.rs:315-324 == basis.rs:133-144; reachability weighted.rs:331-355 ~ basis.rs:218-252 (`live`). basis.rs also has the sort+run-length count twice (:97-111 and :145-152).
- saturated.rs:489-548 insert/sum/excess/within duplicate proofs::multiset::Multiset (insert, sum, difference, is_subset) and inverse/eval.rs (the latter on purpose, core-inverse.md:219). union differs (set union).
- saturated.rs:640 `Step::Hole(h) => set[premises[h]].clone()` in the innermost loop; :1048 `sum(...)` alloc per derived sequent: candidate (check costs 17-90% of search, core-inverse.md:260-266). No ratchet/DHAT journey covers any refutation check or cut elimination.
- saturated::check is 246 lines (:944-1190): setup, rules closure, Mix, goal-phase holes.
- classical.rs:132 evaluates every reached term at each backtracking node; no propagation; ordinary has DPLL and SAT. Candidate (search change: Unknown -> Unprovable).
- Rocq: Refutations.v has classical/unbalanced/equation (+ ill variants), Weights.v weighted/affine; no theorem for Saturated or Basis (export/rocq/deduxit.rs:1208-1210 refuses them). Rust-side argument only.
- ordinary::resolution::Refutation::check(&limits, stop) (refutation.rs:191) and Proof::check_within(mode, &limits, stop): siblings that take both.
