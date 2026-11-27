# Unit 1c: verdicts

11 confirmed, 3 candidate.

## F-1c-01: confirmed

Opened core/src/linear/search/refutation.rs:1122-1243: `check(&self)` takes neither limits nor stop, builds `Forest::new` (default limits), passes `&mut |_| false` to `saturated::check` and `basis::check` and has the two `unreachable!` arms; its own docs say "The check has no stop". search/mod.rs:1080-1104 wraps the same two checks with the stop and `limits.work`. cli/src/check.rs:454-456 runs it under `Deadline::within`, which (cli/src/limit.rs:101-126) leaves the thread behind at the limit; core/src/protocol/linear.rs:711-716 has `limits` and `stop` and uses them for the proof only; web/src/calls.rs:1767 calls it bare. The rubric's S1 (audit-rubric.md:51, must-fix) says every call that can run long takes a stop. core-search.md:282-288 and :313 record the missing stop as a fact (with the 2 491 bytes past 100 s the integers review found), not as a choice with a reason, so nothing settles it. The change is additive and keeps `check()`.

Correction: Cost: the new entry point is one module (refutation.rs, search/mod.rs), but the finding also changes the command, the protocol and the bindings, so by the brief's scale it is large (across crates), though each outside change is a line or two. `check()` must keep `work: None` and no stop (`Limits::default()` has to be checked for a work bound before it is used as the shim's argument), so that no disproof accepted today becomes a refusal; core-search.md:266-288 and :313 and the docs' Cost paragraph change in the same commit.

## F-1c-02: confirmed

Opened refutation.rs:96-228: `UnbalancedSymbol`, `OneSidedSymbol`, `Unbalanced`, `OneSided`, `Equation`, `StateEquation` and `Classical` are `#[non_exhaustive]` with public fields; a grep of core/src/linear for `impl` blocks of them finds none, so no outside crate can write their struct expressions. `Stable::new` (:285), `Saturated::new` (:297), `Basis::new` (:350) and `Marking::new` (:362) exist, and `Disproof::of_goal` is `pub(crate)` (:921) while `Proof::new_of_goal` is public (proofs/mod.rs:615). consumer-test.md:89 and :393 show the toy engine answering with `Refutation::Exhausted` only. The JSON reader (serialize/search.rs:654-663) builds the same values unchecked and validates only indices, so constructors add no path past `Disproof::check`; X3 is kept since a `Disproof` never reads as checked. core.md:383-398 marks these structs on purpose so that fields can be added; a `new` is the usual companion and three siblings have one.

Correction: Kind: additive API (no break), not a refactor in the strict sense; a later field added to one of these structs then means a new constructor, which is the cost the three existing `new`s already carry. The public `of_goal` should validate members as serialize/search.rs:660-662 does (the finding says so).

## F-1c-03: confirmed

Opened refutation.rs:13-29, :60-61, :908-917, :968-978 and :1060-1135. The summary of `Refutation` opens with "Most refutations rest on a search that tried every way to a proof" and never names `Disproof::check`; `Exhausted` and `Disproof::new` say nothing of what is or is not checked; that `Exhausted` has no check is said only at `RefutationFault::Uncertified` and in the check's Errors section. `check` (:1200-1240) does verify every kind but `Exhausted`. handovers.md:53-60 and :75 ask for exactly this. Docs only.

Correction: The Rocq sentence needs more care than the finding gives it: `write_disproof` (core/src/export/rocq/mod.rs:1134-1148) refuses, beside `Exhausted`, `Saturated` and `Basis`, every disproof under the NanoYalla kernel and in an ordered mode, and export/rocq/deduxit.rs:1020 refuses the count equation of a sequent with a quantifier. Better to link `export::rocq::write_disproof` than to restate the list.

## F-1c-04: confirmed

Opened weighted.rs:270-357 and basis.rs:95-252. The `head` closure (weighted.rs:284-289, basis.rs:124-129) is identical, the clause loop (weighted.rs:315-324, basis.rs:133-144) differs only in how the atom index is taken, and both propagate reached atoms by the same waiting/ready scheme (weighted.rs:331-355, basis.rs:218-252); basis.rs counts runs twice (:97-111, :145-152). The differences are the ones the finding names: `dead_atoms` accepts any number of headless clauses and headless clauses under `?`, `read` requires a head under `?` and exactly one goal. core-search.md:289-313 requires independence from the Horn engine (`Program::read_with`), not between the two checks, so one private reader for both keeps the recorded choice. The dead set is written into the Rocq certificate (deduxit.rs:1175), so the net the finding names is the right one.

Correction: The shared reader must return `None` only where both do today and leave `read`'s two extra conditions to basis.rs; `dead_atoms` counts a body literal once per occurrence (`missing`), `live` once per place, which give the same set. core-search.md:306-308 notes that the shape tests follow `Program::read_with` arm for arm: that note then speaks of one reader, and should say so.

## F-1c-05: confirmed

Opened refutation.rs:1295-1324, :1359-1412, :1419-1448 and :1489-1588. The bottom-up walk at :1382-1395 and :1527-1540 is the same but for the exponential's arm (`l.filter(|&s| s == (0, 0))` against `l.map(|_| (0, 0))`), and the open-predicate test at :1306-1316 and :1430-1440 is the same block with another invariant's name. One `excess` and one `predicate_open` would serve both.

Correction: The two callers add the members' intervals differently: `check_balance` adds plainly (:1403, with the comment at :1374-1378 on why an `i64` cannot be left), `check_copies` with `checked_add` and `Mismatch` (:1548-1550). The shared walk should return one member's interval and leave the sum to each caller, or take the checked form for both. The sign scan below the exponentials (:1516-1526) stays in `check_copies`. Value is a reader's; cost small.

## F-1c-06: confirmed

Opened refutation.rs (2 072 lines: the types to :365, the phrases around :637-846, `Disproof` :896-966, the faults and `check` :968-1243, five kinds' checks from :1245) and search/mod.rs:19-62, where `basis`, `classical`, `refutation`, `saturated` and `weighted` are declared among the engines. E5 asks for the seam, and the finding names it (types, phrases, checks; the three other checks beside them). No rules file records the layout as a choice: core.md:22, core-search.md:9-10 and claude-infra.md:61 only list the paths. `Refutation` and `Disproof` are re-exported, so no public path moves.

Correction: The move also changes the `paths:` front matter of core-search.md (:9-10), the table of core.md:22, claude-infra.md:61, and the paths that core-horn.md:300 and core-inverse.md:219 cite, and whatever the mutation run's file lists name (`mutants/`); with those the cost is medium as stated. The gain is a reader's only, and the tie to the architecture reader's feature split is that finding's to argue.

## F-1c-07: confirmed

Opened framed.rs:284-449 and :594-678. The postorder walk with `(id, expanded)` pairs, the `base` test and a memo is written at :320-341 (`names_term`), :419-448 (`substituted_term`) and :615-640 (`Transfer::term`); the walk up a frame chain to the first memoised cell and back down at :291-305, :384-406 and :646-663. The quoted chain snippet is `Transfer::frame`'s (:646-656), not `names_member`'s, which keeps a `bool` in place of `below`; the three are still one shape.

Correction: The helper is less simple than the finding's signature: `substituted_term` adds terms to the table it reads (`self.table().instances.apply`, :441, which is why it copies the argument list at :428), so a fold that holds `terms` borrowed across the `app` callback does not compile for it. The helper has to take the state as one context argument (or two callbacks on a small private trait: the arguments of `t`, and the value made from the folded arguments). Cost stays small; the value is a reader's.

## F-1c-08: confirmed

Opened cut/mod.rs:332-369, :426-503 and :519-556. The choice of the cut is written at :341-352 and :464-472 with the same `at` test and `NoCut` error. `step` checks the made proof with `check::reading` and `check::examine` and `&mut ()` (:355-368); `eliminate` by the next round's `Record::of` (:446-460), which runs the same `examine` with `Phase::Eliminate` (:546-554). Both map an invalid made proof to `Error::Reduction`.

Correction: Two limits on the change. `Record::of` on the made proof is not quite the same pass: the record's zones count against the memory bound (`Observer::bytes`, :565-569) and are built for nothing, so `step` would be refused a little earlier under a tight `memory_bytes`; a small private helper that maps `examine`'s result is the form that keeps behaviour. And the mapping is only partly shared: `step` returns a refused check as `Err(Error::Check)`, `eliminate` as `Ok` with `End::Refused` and the step dropped (:448-457); only the `Invalid` to `Reduction` arm can live once. `chosen` returns `Option`, with `step` turning `None` into `NoCut` and `eliminate` into `CutFree`. Tidiness at small cost.

## F-1c-09: confirmed

Opened saturated.rs:78-93 and basis.rs:56-69: the same two-variant `Failed` with the same `From<RefutationFault>` (saturated's also derives `PartialEq, Eq`), matched alike at search/mod.rs:1087-1104 and refutation.rs:1210-1235. One type beside `RefutationFault` serves both and no rules file asks for two.

Correction: Small and tidiness only; it falls out of F-1c-01 if that is done, so the two should be one change.

## F-1c-10: candidate

Opened saturated.rs:606-700 and :1028-1125. `derive` is called once per choice of premises and partners (:1104-1105) and `Step::Hole(h) => set[premises[h]].clone()` (:640) clones both zones each time, though later steps (`take`, `insert`, `retain`) do change the zone they pop; `closed` allocates `sum(&c.linear, &c.unrestricted)` per derived sequent (:1048), and `Tensor`/`With` allocate new zones too. core-inverse.md:260-266 gives the check's cost (17 to 90 % of the search, 37 s on `SYJ207+1.002` in cbv). Reading supports the claim; no instrument has counted these allocations, and how much of the check's time they are is unknown (the candidate lists and the subsumption test may dominate).

Correction: Measure with DHAT and pinned time on the check alone of saved disproofs of `SYJ207+1.002` (cbv) and `SYJ208+1.005` (cbn), as the finding says; the check's units (core-inverse.md:266-268) must stay the same so that the search's work bound decides alike. Touches a checker.

## F-1c-11: confirmed

Opened saturated.rs:488-548 and proofs/multiset.rs:52-129. `insert` is the same line for line; `sum`, `excess` and `within` are `Multiset::sum`, `difference` and `is_subset` on sorted vectors. `Multiset` is used only under `proofs/` and `rules/` (grep: no file of `search/`), so using it does not join the check to `inverse/`, whose independence core-inverse.md:219 records. One implementation could serve both.

Correction: The value is a reader's, and the check is hot (core-inverse.md:260-266), so the net is not only the tests: `Multiset::merge` (:89-107) counts runs per id through a closure where the local `sum` is a plain merge, and `is_subset` allocates a difference where `within` allocates nothing. The change needs the allocation-free `is_subset` first and the same pinned-time measurement of the saturated check as F-1c-10 before and after; if the generic form is slower, keep the local functions. `Stable`'s public fields stay `Vec<OccId>`, so only the private `Zone` changes.

## F-1c-12: candidate

Opened cut/mod.rs:332-369 and :426-503: both bodies are generic over `stop` (and `eliminate` over `each`), where core.md:327-336 states the rule of a one-line shim over a `&mut dyn` body; everything they call (`Record::of` :525, `reduce` :592, `framed::apart`) already takes `&mut dyn`. Callers with their own closure types: cli/src/eliminate.rs:174 and :821, web/src/calls.rs:2163 and :2200, core/src/protocol/linear.rs:568. The bodies are 38 and 78 lines that mostly call non-generic code, and neither appears in target/audit/wasm.md's list of generics compiled more than once nor in llvm-lines.txt, so the bytes saved may be few.

Correction: Measure the copies with `cargo llvm-lines` for the web module and the command, and the web module's bytes at gzip before and after; as a matter of the stated rule alone the shim is cheap and behaviour-keeping either way.

## F-1c-13: candidate

Opened classical.rs:59-198: `cost` is the number of terms reached (:116), each node of the backtracking pays it (:128-131) and `evaluate` walks all of `order` (:180-194), with no propagation and atoms tried by index. core-search.md:218-244 records the design and the bound (`DEFAULT_CLASSICAL_WORK`, 2^26) as facts, with no reason against a stronger refuter, and says that only an exhausted search's refutation and an undecided verdict can change. Whether any row stops at the bound is not known from reading.

Correction: Measure first, as the finding says: count on the target sets and the baseline's rows the searches that end `Unknown` or `Exhausted` with the refuter stopped at `classical_work`. If none, drop it. If the work unit changes (incremental evaluation), `--classical-work` means something else and the manual says so; handing the reading to `ordinary::resolution` or `ordinary::sat` would make the linear suite name the ordinary one (the `sat` engine behind a feature), which the layering findings weigh. A search change; the check of the assignment is untouched.

## F-1c-14: confirmed

Opened refutation.rs:637-660 and :733-817. The `Words::Two` branch (:746-782) and the `Words::Intuitionistic` branch (:783-817) compute the same `(more, fewer)`, the same `(place, copies)` and the same two templates; they differ in where `fewer_absent` comes from and in the closing note (`crossed`, empty or "a negated formula or an implication's antecedent…", against the fixed "an implication's antecedent counts on the other side"). core-search.md:256-262 says the intuitionistic phrase uses the same words as the two-sided one on purpose, which one branch would enforce.

Correction: Small; the merged branch must keep both notes' exact words, which the README's and the manual's examples pin (cli/tests/readme.rs).
