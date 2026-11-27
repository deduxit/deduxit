# Unit 4d notes

## Read: modal/mod.rs, system.rs, model.rs, parse.rs, decide.rs, calculus.rs, search.rs

- modal/mod.rs:388 `#[derive(Clone, Debug)] pub struct Sequent` — no PartialEq/Eq; Formulas (143) also none.
  decide.rs:134 `if countermodel.sequent.to_string() != self.sequent.to_string()` compares by text (two allocs). Same as F-4a-17 on ordinary side.
- modal/mod.rs:186 `atom_name(&self, atom: u32)` bare u32 atom index (consumer point 8). Node::Atom(u32).
- modal/mod.rs:86 NodeId::new is pub(crate) behind svg only; NodeId has `index` only, no public constructor (fine — arena gives ids).
- modal/model.rs:18 `use crate::ordinary::MODEL_CELLS;` (F-arch-21). decide.rs:104, :238 link crate::ordinary::MODEL_CELLS in modal docs.
- modal/model.rs:273 `std::collections::HashMap<&str, u32>` SipHash in truth (every-unit handover). cold.
- modal/model.rs:176 impl Model { frame_fault, truth } on shared kripke::Model; Countermodel::check (523) wraps truth — two ways to check (consumer pt 11).
- modal/model.rs:374 `struct Meter` (work + stop every POLL) — compare ordinary/model.rs, structure.rs, kripke_structure.rs for copies.
- modal/model.rs:225 `truth` generic `impl FnMut` long body (~110 lines) — core.md says long bodies take &mut dyn. Check instantiation count.
- modal/decide.rs:14 `use crate::linear::search::Reason;` Verdict::Unknown(Reason). search.rs:65-66 Reason, memory::Account from linear.
- modal/decide.rs:40-48 Verdict::NotValid unit + Outcome.countermodel Option (F-arch-13).
- modal/decide.rs:255 decide_within(impl FnMut) -> search.run(&mut stop) generic: search.rs:458 `run(&mut self, mut stop: impl FnMut)`, all of tableau/saturate/... `&mut impl FnMut` -> whole tableau monomorphised per caller. Check llvm-lines/wasm.md.
- modal/decide.rs:23 Options { system } default K (D1 question for library defaults).
- modal/search.rs: bit-iteration over world words written 4x: 735-754 (refuted), 1061-1092 (edge), 1485-1496 (keep atoms), 1766-1778 (model atoms); `truth & boxes != 0 || falsity & diamonds != 0` 3x: 1296, 1338, 1544.
- modal/search.rs:1263-1290 (witness non-blocking path) repeats make() 1358-1383 (world, rule choice, linear, made).
- modal/search.rs:1934-1957 model builds Vec<String> atom names per world (kripke::World holds names) — model of many worlds clones names; cold.
- modal/calculus.rs:129 `name(system)` NAMES table parallel to System::ALL; could be `format!("G3{}")` but const &'static str needed by Error::ReadBack calculus field. ok.
- modal/calculus.rs:169 pub fn calculus(system, limits) — the rule file built as serde forms.

## Read: modal/rocq.rs, ordinary/rocq.rs, ordinary/rocq_models.rs (+ export/rocq/mod.rs:997-1066 caller)

- Rocq string quoting `fn string` x3: modal/rocq.rs:57, ordinary/rocq.rs:969 (pub(super), used by rocq_models), export/rocq/deduxit.rs:1301. Same body `format!("\"{}\"", name.replace('"', "\"\""))`.
- `struct Bounded` x2: modal/rocq.rs:24 and ordinary/rocq_models.rs:44 (latter adds weight). `fn stopped()` x2: modal/rocq.rs:77, rocq_models.rs:98; ordinary/rocq.rs builds Refusal::Stopped{Write} inline at :526, :736, :793, :903.
- Library constructor names of propositional formulas (var/top/bot/neg/wedge/vee/imp/equiv) written in 4 walks: ordinary/rocq.rs:928 `formula`, rocq_models.rs:442 `formula` (+fall/fex, terms), modal/rocq.rs:95 `constructor` (rules terms) and :522 `modal_formula` (+box/dia). Plus export/rocq/deduxit.rs (F-6b-05). Q8 contract names.
- Three bounding strategies for certificate text: ordinary/rocq.rs:726-761 estimate-then-write to out; modal/rocq.rs + rocq_models.rs Bounded buffer then write; ordinary/rocq.rs::write no derivation_bytes at all (streams).
- ordinary/rocq.rs:397 `write(..., mut stop: impl FnMut() -> bool)` — the only certificate writer with a progress-less stop and no `&Limits`; checks derivation at :410-414 with `work: None` regardless of caller. export/rocq/mod.rs:1034 adapts via limits::counting.
- modal/rocq.rs:401 `outcome.check(limits,&mut stop)?` (which ends derivation.check in calculus) then labelled() :434 `derivation.check(&super::calculus(system, limits)?, ...)` again: derivation checked twice and calculus built twice for one certificate of an outcome.
- modal/rocq.rs:481-491 and :600-609 write the sequent's two sides twice (duplicates.md 481↔599).
- ordinary/rocq.rs:166-167 computes `fa`, `fb` (String allocs) eagerly for every inference incl. Axiom/weakenings that never use them. cold.
- ordinary/rocq.rs:585-601 propositional truth by arena order — duplicates ordinary/mod.rs:2066 (duplicates.md).
- ordinary/rocq.rs:840 `Closure` (worlds above, bounded) shared with rocq_models; kripke_structure has closure lists too — check.

## Read: structure.rs, transfer.rs, kripke_structure.rs

- DEFECT (confirmed by run): kripke_structure.rs:908-922 `atom` reads `=` as an ordinary predicate (no EQUALS branch, no refusal of a listed "="), while structure.rs:169 refuses listing `=` and :493 reads identity. core-ordinary.md:595-596 says a Kripke model with domains "must read `=` as identity at each world, or be refused beside `=`". Witness: scratchpad eq.json (one world, no predicates, sequent ⊢ a = a) -> `target/debug/deduxit check eq.json` prints "valid refutation of ⊢ a = a (intuitionistic logic)" exit 0; `deduxit prove --logic intuitionistic '|- a = a'` proves it valid. X3/S sound verdicts. rocq_models::kripke_structure would certify it too (checks with the same check).
- Structure (structure.rs:110-564) and KripkeStructure (kripke_structure.rs:175-980) are two evaluators of one semantics; `From<Structure> for KripkeStructure` (:127) and the test at :1155-1175 asserts they agree on one world. Duplicates: term() structure.rs:510 ≈ kripke_structure.rs:927; atom 481≈908; tick_by 463≈890; memory estimate 136-157 ≈ 212-240; the reached-terms walk 192-260 ≈ 396-450; check() wrappers FiniteCountermodel::check 603 ≈ KripkeCountermodel::check 1019 (same "hypothesis"/"formula right of ⊢" string-compare dance). Structure::refutes could be KripkeStructure::from(..).evaluate(Classical) — but differs on `=` (identity) — which is exactly the defect.
- structure.rs:618 & :624 `if side == "hypothesis"` string compares to branch — side should be an enum/bool (same at kripke_structure.rs:1037).
- Reached-nodes-and-terms walk with Vec<bool> seen: structure.rs:192-260, kripke_structure.rs:396-450, rocq_models.rs:309-356 (`applied`), transfer.rs:290-326 (`reached`) — 4 more copies beyond F-4a-08's five.
- kripke_structure.rs:299-308, 360-369, 495-504: the work/stop poll written inline 3x in evaluate besides `poll` (:567) and Forcing::tick_by (:890) — 5 pollers in one file; structure.rs:463 another; modal/model.rs Meter another.
- KripkeWorld::new (110) positional 5 args incl bool `falsity` (C-CUSTOM-TYPE/C-BUILDER minor).
- transfer.rs:229 `renamed.values().any(|r| r == candidate)` linear scan per candidate in apart(); target.atom_names().iter().any linear: quadratic in names on Back. cold (interactive).

## Read: models.rs

- models.rs:151 `pub fn decide(sequent, logic, options: &search::Options, ...)` — public entry of ordinary suite takes linear::search::Options for model_size/model_worlds (F-arch-02 at another door).
- models.rs:106-122 Outcome { verdict, structure: Option, kripke: Option } (F-arch-13 shape).
- models.rs:555-562 `Writer::holds`: `self.holds.get(&(predicate, tuple.to_vec()))` allocs a key per probe, then 2 more `to_vec` on insert; :566-581 `values` same (`args.to_vec()` x3) and clones the Vec<NodeId> on a hit (:568); :668-676 `key(b)` allocs per probe; :703, :721 clone denotes tables. Expansion hot loop; candidate P2 (no journey runs the model search: DHAT a `prove --method models` run).
- models.rs:541 `self.formulas.atom(&format!("m_{number}"))` + :827-829 parse names back `strip_prefix("m_")...parse` — the atom index is the entry number channel through strings (name normalisation, check_ordinary, 2 String allocs per atom in Formulas). kripke_search similar? check.
- models.rs:226-231 ≡ :254-259 (same match on kripke()).
- models.rs:362 unknown_for — check grounding for a twin.

## Read: grounding.rs

- Atom index through a made-up name: grounding.rs:1394 `formulas.atom(&format!("g_{index}"))`, structure_of :1496-1502 `atom.strip_prefix("g_").and_then(|n| n.parse::<usize>().ok())` over `true_atoms: &[String]` (the SAT model's world atom names, :569); models.rs:541 `m_{number}` / :827-829 parse back (there true_atoms are already indices: names[a] then parse!). Atom index == entry number in both arenas (atoms made only there). Each atom: format! + NFC normalize + check_ordinary + 2 Strings (F-4a-06) + model's names. Candidate P2; no journey runs grounding/models (DHAT a `prove --method grounding` on ILTP SYJ212+1.010 or similar).
- The memoised walk of the normal form under bindings (Task Enter/Expand/Join/Store, lookup over a bindings chain, memo HashMap<(u32, Vec<u32>), NodeId>, held/tasks/occurrence bound per step) written twice: grounding.rs:1282-1460 (ground) and models.rs:726-1027 (lookup + expand). duplicates.md: grounding.rs:1354↔models.rs:967 (21 lines), :1413↔:1002. Differences: literal handling (ground atom table vs over_tuples), ∀ (Skolem binding vs size-way conjunction). One generic walk with a trait for Lit/Forall.
- grounding.rs:1365 `memo.get(&(n, values.clone()))` clone per probe; :1195 `visited.insert((n, values.clone()))`; :220 `app_index.get(&(symbol, args.clone()))`. models.rs:978 same. P2 candidate (same as models Writer).
- grounding.rs:452 `decide(sequent, logic, limits, stop)` vs models.rs:148 `decide(sequent, logic, &search::Options, limits, stop)` — sibling public engines, models takes linear options for two bounds.
- grounding.rs:509-526 + 527-552: refusal→Reason mapping as closures; models.rs:362 unknown_for; decide::ended (4a). three maps.

## Read: kripke_search.rs (all 16 files now read)

- kripke_search.rs:196 `format!("k_{number}")` + :963-965 parse back — third copy of the name channel.
- models.rs ↔ kripke_search.rs Writer twins: junction 585≈208, over_tuples 605-652≈363-409 (duplicates.md 399 tokens, 58 lines), tick 508≈172, symmetry 787-798≈739-759, function table building 857-873≈1011-1036, Denotes enum 452≈69, Entry, lookup. holds/values/constant key `to_vec()` per probe (248, 253, 264, 319, 352, 354).
- kripke_search.rs:597 `let mut seen = vec![false; formulas.len()]` inside the per-root loop: one arena-sized vec per hypothesis.

## Checks done
- llvm-lines: modal::search::Search::model/keep and decide::derivation each 2 instantiations in the lib (decide's closure, &mut dyn from protocol); CLI cli/src/modal.rs:115 adds a third closure. wasm.md: deduxit::modal 128 298 bytes raw.
- No bench/ratchet/DHAT journey runs modal, grounding, models or the Kripke search (grep bench/src modal: none).
- One SipHash site: modal/model.rs:273 (cold).
- Plan of findings: 01 = defect; 02 Structure/Kripke evaluators; 03 encoder twins; 04 modal generic body; 05 name channel; 06 key to_vec; 07 reached walk; 08 Rocq constructor tables; 09 Rocq helpers; 10 directory; 11 double check; 12 modal split allocs; 13 countermodel docs; 14 modal needs of Reason/Account; 15 modal bitset loops.
