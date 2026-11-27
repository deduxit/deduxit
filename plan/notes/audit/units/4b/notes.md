# Unit 4b notes

Instruments: ordinary-pigeons journey runs the EMBEDDING (bench/src/journeys.rs:473-499), not LJT/SAT/normal/tableau/written. No ratchet/DHAT journey covers this unit's engines -> every perf claim a candidate.
llvm-lines: interpolant::maehara and interpolant::checked compiled TWICE (once &mut &mut dyn FnMut, once interpolate_default::{closure#0}) -> P4 vs core.md "long body takes stop as &mut dyn". Tableau::of::<&mut &mut &mut dyn>; normal::NormalForm::check::<&mut&mut&mut dyn>. Check generics.
duplicates.md: ljt.rs:958 <-> sat/read_back.rs:855 (227 tok, push); ljt.rs:926 <-> read_back.rs:836 (112 tok); normal.rs:2625<->2732; normal.rs:363<->403; classical.rs:256<->379; herbrand.rs:1770<->interpolant.rs:611; interactive.rs:2660<->written/mod.rs:776; tests ljt.rs:1346/1453 <-> sat/mod.rs:1025/1082.
surface: normal::normalize_default nothing names (0 elsewhere in core); ljt::POLL public const.
text.md: normal.rs:2448 "stand as hypotheses"; solver.rs:25 "rather than"; drup.rs:625; solver.rs:238.

## ljt.rs
- :585-589, :633-637, :1180-1184 three copies of operands->(a,b) with falsity default.
- :367-387 implication / implication_of build the same node twice.
- :518-534 heads() allocates a HashSet per call; levels() (:539) calls heads twice per hypothesis per round (up to left.len() rounds) -> O(n^2) HashSet allocs per moves() with >1 ->->L. Candidate P2; no instrument.
- :1328 difference() O(n*m) position scans; weaken_to clones `have` per weakening (quadratic by design of derivation).
- :85 `pub const POLL` public; also sat's POLL private const same value 1<<8 (sat/mod.rs:139).
- run() uses &mut dyn (good). Lj generic S instantiated once.
## sat/mod.rs
- :374-394 builds a dummy Refusal::Output{0,0} in non-test builds where Derive::Ljt (cfg test) is the only user.
- :482-485 and :528-531 same ReadBack "LJT refutes a sequent the SAT search proved" twice.
- :517-523 by_ljt maps impossible evidences to fake errors; prefer_ljt treats them as Ok(derivation) -> inconsistent unreachable handling.
- :539 polarities (NEGATIVE/POSITIVE u8 bits) vs classical.rs encode polarity vs normal.rs polarities: three polarity passes? check.
- :936 sort_by_key(|w| w.members().count()) — members() bit-by-bit filter; count_ones; sort_by_cached_key. :1008 formulas.atom_names() inside per-world closure (check cost).
- :620 Bits::members iterates 64 bits/word with filter.
## sat/solver.rs
- :259 add_clause allocs Vec per clause; :277 solve allocs assumptions per call. minor.
- Lit {u32,bool} 8 bytes; drup uses 2v+s codes.
## sat/classical.rs
- :235-287 decide and :360-401 falsify duplicate encode -> new_var loop -> add_clause+poll loop -> solve -> statistics (duplicates.md classical.rs:256<->379). One helper `solve(encoding, recording)`.
- :57-92 polarities() = sat/mod.rs:539 polarities() with TRUE/FALSE renamed NEGATIVE/POSITIVE; differences: mod.rs Not also marks falsity, Iff marks the implications. normal.rs has another (check).
- :162 `definition` Vec<Vec<Lit>> allocated per formula; fine (setup).
## sat/drup.rs
- :342 keep() clones literals into index key: every kept clause's literals stored twice (charged 2x). Index only serves deletions; could key by clause id with hash of literals. Candidate P5; no instrument.
- :313 normal() allocs Vec per step. ok.
- ClausalProof Copy, private fields, checked pub(crate): matches X3 trust boundary.
## sat/read_back.rs
- Lj at :794-969 vs ljt.rs Lj :879-1099 vs connection/intuitionistic/read_back.rs Emit :1207-1339 vs connection/read_back.rs:514 push vs herbrand.rs:1176 push vs derivation.rs:1779 Out::push: SIX builders of a derivation with push(stop, account, u32 index refusal), weaken_to, contract_to/tidy, in_order, difference/without. `Space::Inference` refusal written at ljt.rs:969, sat/read_back.rs:866, connection/read_back.rs:531, connection/intuitionistic/read_back.rs:1265, herbrand.rs:1208, derivation.rs:1799.
- :782 without() uses remove (O(n)) vs ljt difference uses swap_remove — same function, different order semantics.
- :836 uses() linear contains; unwind calls uses per step per k -> O(steps*|left|). fine.
- :363 self.truths.clone() per prove call.
- :47 too_many_arguments expect on derivation() and Tableau::new (8 args each).
## normal.rs (normal forms, NOT natural deduction)
- :71 `use std::collections::HashMap;` SipHash, beside crate::hash::HashSet at :798,:811,:1207,:1223,:3118. HashMap sites: :1344-1347 copy_formulas (3 maps), :2757 encoded `name`, :2872-2873 polarities. Cold-ish (per rewrite/claim), not a hot search.
- :1338 copy_formulas and :1437 transfer duplicate ordinary/transfer.rs:74 `copy(target, source, roots: Option<&[NodeId]>, &Parameters)` (roots Some / None). Differences: normal errors on Eigen/Meta (ClaimFails), copy makes constants.
- :2872 polarities (HashMap over `order`) = third copy of the polarity pass (sat/mod.rs:539, sat/classical.rs:57).
- :2592 fresh_prefix(f, base) vs :2710 prefix(f) ("t", atom names only) — prefix is fresh_prefix(f,"t") but ignores symbols (propositional: symbols empty).
- :2625-2640 (skolemized) and :2732-2747 (encoded): same Negate-step loop (duplicates.md normal.rs:2625<->2732).
- NormalForm (:632) public fields form/sequent/steps/result/checks; is_checked() (:648) reads `checks` only, so mutating `result` or `steps` after check() leaves is_checked() true (claims() comment :675 admits public fields). Test :3348 mutates result then calls check. X3/C-STRUCT-PRIVATE.
- `normalize_default` :3178 (nothing outside core names it; only doc example) and interpolant::interpolate_default: two `_default` convenience fns.
- E5: file is 3688 lines: seams = (a) vocabulary Form/Promise/Rule/Step/Claim/Checked/NormalForm + check (:74-1183), (b) propositional rewriting Passes/Run/law/apply (:1548-2531), (c) first-order: quantifier_law/apply_quantifier/skolemized/strong/skolem_term/skolem_fresh/replay/law_claim/copy_formulas/holds_symbol (+first_order_tests), (d) encodings encoded/definition/polarities/constant (:2679-3007), (e) in_form checks (:3009-3151).
- llvm-lines: NormalForm::claims 2545, encoded 2171, replay 1721, copy_formulas 1561, definition 1416, transfer 1400, check::<&mut&mut&mut dyn> 1360.
- check() is `impl FnMut` generic, big body (:1074-1182), unlike rewrite which shims to rewrite_dyn. Instantiated per caller.
## interpolant.rs
- :350 interpolant() pub generic impl FnMut, calls maehara(:784, `stop: &mut impl FnMut`) and checked(:385, impl FnMut) -> no &mut dyn shim. llvm-lines.txt:79-80 maehara 2089 lines x2 (`&mut &mut dyn FnMut` and `&mut interpolate_default::{closure#0}`), :109-110 checked 1822 x2. core.md "long body takes stop as &mut dyn". MEASURED (llvm-lines).
- :176 Interpolant pub fields first/second/first_outcome/second_outcome; is_checked (:204) from pub outcomes -> forgeable, same as NormalForm. X3.
- :262 cut_free closure clones whole derivation `d.clone()` while `decided` (holding same derivation) is kept in Interpolation.outcome -> derivation held twice; could compute interpolant by reference before moving outcome. P3 candidate.
- :317 interpolate_default; normal::normalize_default :3178 (pattern _default; ordinary::decide vs decide_within uses other naming).
- :476 atoms_of pub(crate), used by normal.rs:1016,:1051 (arena walk living in interpolant).
- duplicates herbrand.rs:1770<->interpolant.rs:611 (rebuild nodes after operands loop).
- :280 another "Error::ReadBack ... embedding finds not valid" string.
## tableau.rs
- :620-627 parent_rule scans nodes[..index] with children.contains per node; called from write_text (:568) once per node -> O(n^2) to print a tableau (Display). Tableaux reach 1.7M inferences (rules: SYJ206+1.020). Fix: carry the parent's `expands` on the write_text stack. Candidate (no journey prints a tableau).
- :716 too_many() refuses with Space::Inference for tableau nodes/lines.
- :889-896, :929-936: `countermodel: None, structure: None, kripke: None` (F-arch-13 pattern).
- :264 Tableau::of generic impl FnMut pub; one instance in llvm-lines (&mut&mut&mut dyn).
- :859 decide pub(super) generic impl FnMut, body ~80 lines + expand generic.
## written/mod.rs
- :633 review(&self, calculus: &Calculus, ...) docs (:602-604) say calculus "is the rule file TABLEAU read"; nothing checks it. Callers parse TABLEAU themselves: cli/src/tableau.rs:86 (calculus::named("tableau")), web/src/ordinary.rs:811-813 (serde_json + wire::upgrade). WrittenTableau has no doc example (surface.md:672). X3/C-VALIDATE.
- :769-786 differs() copies expected's arena by atom names: another arena copy (duplicates.md interactive.rs:2660<->written/mod.rs:776).
- :870 too_many == tableau.rs:716 too_many (Space::Inference for lines/branches).
- :912 shapes() allocates Vec<Shape> of Vec<Vec<Signed>> per call; called per line (cited, expansion) and per path line in unexpanded(). teaching scale, skip.
## cross
- llvm-lines.txt:192-193 NormalForm::check also twice (1360 x2): second copy for normalize_default closure. Total duplicate: maehara 2089 + checked 1822 + check 1360 = 5271 lines.
- 1a already listed normal.rs:71 SipHash sites (1a findings :490). Fold into copy/polarity findings.
- F-4a-13 covers _default naming; F-4a-19 SipHash of 4a; F-arch-06 Account move; F-arch-02 two options.
- Linear-suite items this unit names: linear::search::Reason (ljt, sat, normal), memory::Account (ljt, sat/*), linear::proofs::InfId (ordinary derivation ids!), linear::proofs::{GoalId, Structural, ViewOptions} (tableau), linear::search::Options as a public parameter of normalize/check/interpolate/interpolant.
- Derivation::from_parts pub(crate) (derivation.rs:605), Inference fields pub(crate) (:400-416): no public way to build an LK/LJ derivation.
- ratchet journeys (bench/src/journeys.rs:42-137): none runs LJT, SAT, normal, interpolant, tableau, written. RESULTS.md has ILTP verdict rows for sat/table/tableau methods.
