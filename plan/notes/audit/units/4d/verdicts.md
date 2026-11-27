# Unit 4d: verdicts

11 confirmed, 4 candidate.

## F-4d-01: confirmed

Opened core/src/ordinary/kripke_structure.rs:908-922 (atom looks the name up in `holds` only, no equality case; `grep EQUALS` finds nothing in the file) and structure.rs:169-173, :492-497 (the finite structure refuses a listed `=` and reads it as identity). core-ordinary.md:574-581 and :595-596 record that the search builds no Kripke model beside `=` and that a Kripke model must read `=` as identity or be refused beside it; the check does neither. I reproduced the witness with the tree's target/debug/deduxit: a full outcome document with the finder's `kripke_structure` gives `valid refutation of ⊢ a = a (intuitionistic logic): a Kripke model of 1 world refutes it`, status 0, while `prove --logic intuitionistic '|- a = a'` answers valid. cli/src/check.rs:960-981 calls `countermodel.check` with no guard of its own. This breaks the rubric's S4 (a checked refutation of a valid sequent). Both proposed mends are sound: identity at each world satisfies the equality axioms.

Correction: Kind: no signature changes, so it is not an API break in the usual sense; it is a soundness fix of a checker that makes it accept less (must-fix by S4). `From<Structure> for KripkeStructure` (kripke_structure.rs:127) changes the truth of `=` for the same reason and is mended by the same change.

## F-4d-02: confirmed

Opened structure.rs:125-260, :456-563, :603-632 and kripke_structure.rs:200-450, :884-979, :1019-1046. `tick`/`tick_by` are identical, `term` differs only in `self.functions.get` against `self.functions[w].get` and the size, `atom` differs only in the equality case (the drift of F-4d-01), the node walk of the up-front pass is verbatim, and both `check` wrappers branch on the string `side == "hypothesis"`. The inline stop polls at kripke_structure.rs:299, :360, :495 beside `poll` (:567) are there. No rules file records the two evaluators as a choice. The finder's caution on running the finite check as a one-world Kripke model (work counts at the bound) is right; the shared-core option keeps them.

Correction: The up-front terms passes are not identical after the node walk: the finite one checks each function table's size in the loop (structure.rs:236-250), the Kripke one only collects arities and checks the tables per world afterwards (kripke_structure.rs:440, :454-484). The shared part is the walk and the constant test; cost medium stands. It touches two checkers.

## F-4d-03: confirmed

Opened models.rs:515-680, :722-735, :815-835 and kripke_search.rs:165-270, :355-400, :955-970, grounding.rs:1278-1296, :1360-1398, :1490-1506. `junction` and `over_tuples` are verbatim in models.rs and kripke_search.rs, `tick` differs only in `poll_by(weight)` against `poll`, the atom maker differs in the prefix and the entry's weight, and `lookup` is a function at models.rs:726 and a closure of the same body at grounding.rs:1282. One writer core serves these. The rules file records the encodings, not the copies as a choice.

Correction: The core is smaller than the change says: `holds` and `values` are not shared as they stand (the Kripke writer makes an atom per world with growth axioms, kripke_search.rs:247-266), so the shared part is the bounds and tick, `junction`, `over_tuples`, `Denotes`, `lookup` and the numbered atom; I did not open the `symmetry` pair or compare the two task walks line by line. The statistics `atoms`, `nodes` and `size` are the net, as the finder says.

## F-4d-04: confirmed

Opened modal/decide.rs:255-263 and the grep of `impl FnMut` over core/src/modal: search.rs:399, :458, :481, :500, :899, :1159, :1240, :1328, :1457, :1659, :1689, :1735, :1752 and decide.rs:421 all take `&mut impl FnMut(Progress) -> bool`. target/audit/llvm-lines.txt:69-70, :121-122, :152-153 show `Search::model`, `decide::derivation` and `Search::keep` twice each (the `&mut dyn` of the protocol and `decide`'s closure). core.md:327-336 states the rule and the 129 KB. The shims keep the public signatures, so nothing breaks; the count is from the inventory, so this is a finding of fact on code size, and the bytes saved in the web module are for the budget check to show.

Correction: Places: `Model::truth` and `frame_fault` take the stop at modal/model.rs:200 and :230 (the finder's :196 and :225 are their doc lines); `Countermodel::check` is :523 and :565.

## F-4d-05: candidate

Opened models.rs:539-552 (`format!("m_{number}")`) and :826-830 (`strip_prefix("m_")` and parse), kripke_search.rs:194-203 and :962-966, grounding.rs:1394 and :1496-1500, :565-569 (the solver's world's atom names as strings). Each writer starts from `Formulas::default()` (models.rs:751, kripke_search.rs:570) and makes atoms at one call site only, so the atom index is the entry number. core-ordinary.md:1078-1081 and :1226-1227 record the naming, with a reason for the flat tuples only, not for the round trip through text. Reading supports the claim; no journey runs these searches, so the gain is unmeasured.

Correction: Measure DHAT blocks and callgrind instructions of `prove --logic classical --method grounding` on ILTP's largest decided grounding and of `--method models` on a refuted problem, before and after. For grounding the change reaches `sat::decide`'s result (indices beside names) and an unnamed atom in the arena, so cost medium is right and it meets F-4a-06 (the index table of names).

## F-4d-06: candidate

Opened models.rs:555-582 (`holds.get(&(predicate, tuple.to_vec()))`, then `to_vec` twice more on a miss; `values` clones the row on a hit), kripke_search.rs:247-266 (the same shape) and grounding.rs:220, :236, :1195, :1365 (`(symbol, args.clone())`, `(n, values.clone())`). The allocations per probe are there as read. Whether they matter is unmeasured: `over_tuples` also allocates the conjunct vectors per tuple, so the key is one allocation among several.

Correction: Measure with DHAT on the same two runs as F-4d-05. A borrowed probe of a tuple-keyed map is not available with std's `Borrow`, so the single-`Vec<u32>` key the finder proposes (or hashbrown's raw entry, a new dependency) is the form; the statistics and checked structures are the net.

## F-4d-07: confirmed

Opened structure.rs:192-219, kripke_structure.rs:396-423 and transfer.rs:290-328: the node walk (seen-set by `mem::replace`, the same `match` on `Node`) is verbatim in the two checks, and `transfer::reached` is the same walk returning the node and term bitmaps. `transfer` is behind `interactive` (ordinary/mod.rs:117-119), as the finder says. F-4a-08 (confirmed) found the same walk five times in instance.rs and herbrand.rs; one iterator serves all.

Correction: The two checks use the term loop to do different work per term (structure.rs:226-259 checks tables, kripke_structure.rs:430-449 collects arities), so only the walk to the reached terms is shared, not the loop bodies; `reached` returns bitmaps over the whole arena, where the checks want an iterator. I did not open rocq_models.rs:309-336. It touches two checkers' up-front pass.

## F-4d-08: confirmed

Opened ordinary/rocq.rs:931-957, rocq_models.rs:466-479, modal/rocq.rs:93-106 and :544-554: the names `top`, `bot`, `neg`, `wedge`, `vee`, `imp`, `equiv` (and `box`, `dia`, `fall`, `fex`) are spelt in each. F-9-01 is confirmed in units/9/verdicts.md. The witness holds, but the value is smaller than said.

Correction: The four walks write three different inductives of the library: `Intuitionistic/Formulas.v:24`, `Intuitionistic/FirstOrderFormulas.v:33-36` (where `equiv` is a definition, :49) and `Modal/Formulas.v:19-24`. A rename in one of them does not touch the others, so 'one table' is three lists that happen to agree, and only modal/rocq.rs:95 and :522 write the same inductive twice. `export/rocq/deduxit.rs` has none of these names (`grep wedge` finds nothing; it writes linear logic's constructors), so it would not read the table. The mend that carries value is F-9-01's list with these names per library module in it; the shared constants are tidiness at small cost.

## F-4d-09: confirmed

Opened modal/rocq.rs:22-54, :57-59, :77-82 and ordinary/rocq_models.rs:44-102: `Bounded::push` is the same body in both (the second adds `weight`), `stopped` is the same function, and `string` is the same line at modal/rocq.rs:58, ordinary/rocq.rs:969-971 and export/rocq/deduxit.rs:1301-1303. The inline `Refusal::Stopped` at ordinary/rocq.rs:526, :736, :793, :903 are there. Moving the three helpers into `export::rocq` is a refactor with byte-identical text.

Correction: The last part of the change is not a refactor. `ordinary::rocq::write` (ordinary/rocq.rs:393-397) takes `impl FnMut() -> bool` because the public `export::rocq::write` (export/rocq/mod.rs:1029-1038) takes no `Limits` and wraps its stop with `limits::counting`, as it does for the linear writer beside it: that is the exporter's convention, not a stray one. Giving it `&Limits` changes the public `export::rocq::write` and its callers (cli/src/show.rs:1065, web/src/ordinary.rs:1138, protocol/linear.rs:666, protocol/ordinary.rs:1270), an API break across modules that belongs to the exporter's unit. Keep the finding to the three helpers.

## F-4d-10: confirmed

Opened ordinary/mod.rs:84-122: `kripke_search`, `kripke_structure`, `model`, `models`, `rocq`, `rocq_models`, `structure` are flat siblings among 30 entries of the directory; `wc -l` gives the sizes the finder states. The handover for unit 4d asks this question, and E5 asks for a seam, which the finding names (data and checks, searches, certificates). No public path moves: `models` is the only public one and keeps its name, the countermodel types are re-exported at mod.rs:136.

Correction: The gain is orientation alone, at medium cost. The move also renames every path the rules files cite: core-ordinary.md and core.md name these files throughout, and the rules files' `paths` and CLAUDE.md's table must change in the same commit, which the finding does not count. It is best done together with F-4d-02 and F-4d-03, which create the shared modules the new directories would hold.

## F-4d-11: candidate

Opened modal/rocq.rs:390-434 and modal/decide.rs:128-203: `outcome` calls `outcome.check(limits, &mut stop)`, which ends in `calculus::calculus(self.system, limits)?` and `derivation.check(&calculus, limits, stop)`, and then `labelled`, which begins with `derivation.check(&super::calculus(system, limits)?, limits, &mut stop)?`. The double check and the calculus built twice are there; core-modal.md records no reason for it. The split is sound: the root comparison (decide.rs:164-200) does not depend on the check. What it saves is unmeasured.

Correction: Measure pinned time, or instructions under callgrind, of writing the Rocq certificate of a large valid modal outcome before and after. `Outcome::check` also checks a countermodel carried beside a valid verdict (decide.rs:133-151); the split must keep that in the public check.

## F-4d-12: candidate

Opened modal/search.rs:1158-1220 (`retain` calls `beta`, which builds two `Vec<Member>` per pending formula, then `beta` again for the chosen one), :996-1016 (`sees.clone()` per box or diamond), :1100-1134 (`sees` and `seen_by` cloned) and :826-835 (`linear` receives owned `written` and `premise` and returns at once where the premise holds). The allocations are as read. No journey runs modal logic, so nothing is measured.

Correction: A modal journey under DHAT is needed first, as the finder says. Iterating `sees` by index in place of the clone is not plainly behaviour-keeping: `linear` takes `&mut Branch` and its premises are edges (`Member::Edge(x, z)` at :1111-1117), so the list may grow under the loop; the mend must fix the length before the loop, and the statistics `worlds`, `splits`, `blocked`, `work` must stay equal.

## F-4d-13: confirmed

Opened modal/model.rs:494-508, structure.rs:566-589 and kripke_structure.rs:982-1005. Each field doc states the property in the indicative ('Its relation meets the system's frame conditions…', 'It makes every formula left of the sequent's ⊢ true…', 'Its world forces every formula…') with 'as check checks' appended, the fields are public and `new` is const and unchecked. X3 asks that no public path make a value that reads as checked without the check.

Correction: The value is small: each `new` already says 'unchecked', and the two ordinary type docs already say 'said to refute'. The mend is three field docs and a sentence on each type.

## F-4d-14: confirmed

Opened modal/search.rs:65-66 (both imports), modal/mod.rs:62-66 (no re-export of `Reason`, so a modal user matching `Verdict::Unknown` names `deduxit::linear::search::Reason`), core-modal.md:268-270 (records that modal took the account and `Reason` 'as it is', with no reason against moving them). F-arch-01 is confirmed in units/arch/verdicts.md. The witness holds.

Correction: It adds little to the architecture reader's findings and should be folded into them: F-arch-01 moves `Reason`, and units/arch/findings.md:224-230 already proposes moving `memory::Account` and names modal/search.rs among its users. `Account` is `pub(crate)` (linear/search/memory.rs:21, `pub(crate) mod memory` at linear/search/mod.rs:39), so its move is a refactor, not an API break; the only break is F-arch-01's. I did not check the list of seven `Account` methods one by one.

## F-4d-15: confirmed

Opened modal/search.rs:728-752 (two bit loops), :1056-1084 (two more), :1483-1497 and :1764-1779 (the same iteration as `flat_map` over `from_fn`), and :1296-1298, :1338-1340, :1544-1545 (the test `w.truth[i] & self.boxes[i] != 0 || w.falsity[i] & self.diamonds[i] != 0` three times). Two helpers serve all of them and the statistics do not move.

Correction: Tidiness at small cost. At :1060-1062 both masks of a word are read before the loops, inside which `self.linear` takes `&mut Branch`; an iterator that borrows the world would not compile there, so the helper takes the masked word by value (`ones(word, bits)`), not a borrow of the set.
