# Unit 1b: notes

## Inventories
- surface: items under several paths: Branch, CheckError (3 paths), ClassicalRule (3), ClassicalSides (3), Compact, Derivation (3), Division, Dyadic, Fault, InfId (3), Inference (3), Invalid, Labels, Layout, Named, Node, NodeId, OpenGoal, Proof, Refused, Rule, Sides, Size (3), Structural, TextOptions, TwoSided (3), ViewOptions (3).
- not named outside core/src: linear::Division (rule.rs:179), linear::proofs::Dyadic (check/error.rs:25); methods TextOptions::with_bar/with_gap (fmt.rs:86,92), ViewOptions::with_classical (view.rs:310).
- Proof: 22 methods, 20 without example; Derivation 11/11; Inference 7/7; TwoSided 12/12.
- duplicates: check/mod.rs:64 ~ derivation/mod.rs:56 (26 lines); check/mod.rs:308 ~ :997; check/mod.rs:999 ~ derivation/mod.rs:211; pass.rs:939 ~ :955; derivation/mod.rs:821 ~ :868; style.rs:62 ~ :69; classical.rs:792 ~ ordinary/derivation.rs:1081 (29 lines).
- clippy functions >100 lines: first_order.rs:137 (392, test), size.rs:757 (194), pass.rs:883 (188), check/mod.rs:418 (175 test), check/mod.rs:241 (142 test), classical.rs:1254 (122), derivation/mod.rs:453 (117 test), build.rs:621 (114), error.rs:391 (109), check/mod.rs:965 (105), derivation/mod.rs:284 (101).
- text.md: "rather than" sentences at fmt.rs:99, fmt.rs:188, size.rs:1060, classical.rs:365, view.rs:503, view.rs:637, view.rs:672.

## proofs/mod.rs (1091)
- :63-77 `Bytes` Display uses `std::fmt::Display/Formatter/Result` fully qualified though :59 imports them. Used by search/mod.rs:3682 too.
- :519-603 `reach`: `crate::limits::Refusal::…`, `crate::limits::Space::Node`, `crate::limits::Phase::Search` written out in full each time (also :572-575, :622-625, :733-737, :812-815): import `Space`, `Refusal`, `Phase` once. Idiom E3-ish (qualified paths) — minor.
- :679-705 `Proof::formula` returns `impl Display + '_` but always builds a `String` (both arms `.to_string()` / text). The `+ '_` borrows self for nothing; an eager String disguised as lazy Display. Computes `Reading::of_mode` (a pass over the forest) per call even for instance branch. C-CALLER-CONTROL / E1 rpit capture.
- :741-747 `onto`: `name` closure `f.sequent().atom_name(a).to_owned()` allocates a String per atom per pair compared in `alike` (each subtree node). Compare `&str` instead (Option<&str>). P2 candidate; onto is used by Interactive::fill (graft), not hot.
- :754-760 `onto` keys a HashMap by formula text String for every goal member (allocations), comment explains n² avoidance. OK.
- :830 `conclusion()` allocates Vec<OccId> per call; callers: pass.rs:139 (each check), size.rs:148, view.rs:585, derivation/mod.rs:537. One alloc per check — negligible.
- :350 Proof doc: "A search engine returns a proof. Whether it proves its sequent is a separate question, which check answers." -> consumer-test point 5: Proof's docs DO say the check is the boundary; Verdict/Disproof (search, unit 2a) are where it is missing.
- :414 `Proof` derives Clone, Debug only; no PartialEq (Forest?). Check.
- :443 `Proof::new` pub; `build` pub(crate) with stop `&mut dyn FnMut() -> bool` — a public `new` has no stop; an outside engine with a 10^8-node arena cannot stop `new` (X2). `new` polls `|| false`. Note: outside engine X2 (stop convention); `Proof::new` has no `_within`. Candidate finding X2.
- :1006 second `impl Node` block for map_premises after Proof's impl: split impl blocks of Node (194 and 1006). Minor idiom.
- :476 `with_instances`: `table != Instances::new(&proof.forest)` builds an empty table to compare for "is it empty". minor.

## proofs/rule.rs (532)
- Three tables of the same 41+4 names: `Named::name` match (:305-340), `KNOWN` array (:498-502), `FromStr` match (:438-479); plus `Rule::name` (:83-106) and `Node::TAGS`/`Node::name` (mod.rs:205,295) spelling "ax","⊗",... again. The test `names_round_trip` pins them; KNOWN could be a const computed from `Named::ALL`/`DIVISIONS` via const fn `name` (E6). FromStr's Unicode arm could be `ALL.iter().find(|n| n.name()==text)` like DIVISIONS at :435 is, leaving only ASCII aliases in a match.
- :190 `Named::ALL: [Self; 41]`; core-derivations.md:297 says "a public array of 34" — stale rules text (not code).
- :349 `Named::index` linear search in a while loop (const fn) — 41 entries; per label lookup. negligible.
- :160 Named non_exhaustive with pub fields; no public constructor with division — fine (applied is pub(crate)).

## check/mod.rs (1425; 52 lines of code, rest tests)
- tests helpers `o`, `n`, `proof` duplicate derivation/mod.rs:56 (duplicates.md 26 lines) — test helpers; could share a `#[cfg(test)]` module. Low value.

## check/zones.rs (434)
- clean. `table_bytes` hard-codes hashbrown's layout (16-byte group) — documented.
- `State` 2 hash tables per derived sequent, boxed: every leaf (`just`) allocates a Box and a HashMap (and later HashSet on first copy). Candidate P2/P5: DHAT on check-* journeys.

## check/pass.rs (1116)
- :790 `l.unrestricted += self.quest_weight(a)`, :1014 `d.unrestricted -= u64::from(self.observer.weight(m, o))`, :1027 `d.unrestricted += self.quest_weight(a)`: plain (non-saturating, unchecked in release) arithmetic on the Θ weight sum, inline in `Pass`, not in the `State` functions the Kani harnesses call. core-proofs.md:260-263 claims "The arithmetic of the pass lives in functions the harnesses call (State::put, remove, add_linear, recount, pass::room, table_bytes)". Check harnesses.rs for `unrestricted`.
- :883 `rule` 188 lines: one match arm per Node; arms repeat `let o = self.inst.occ(m); self.expect(m, o, Kind::X)?; let mut d = self.premise(p)?; facts.absent[0] = self.take(...)?; self.put(&mut d, m); Ok(d)` (Bot, Par, Plus, Bang, Quest, Forall, Exists). duplicates.md pass.rs:939~:955 (Tensor/With heads). A helper `unary(m, kind, p)` would shorten; but checker code readability — modest.
- :55 `Allowance` pub(crate) fields + `new`; second pass builds it by literal (:240). fine.
- :195 `examine_with` generic over O, I and `impl Fn() -> I` — instances per observer × instancing (llvm-lines check).
- :278 `arcs` fine.

## check/harnesses.rs (487)
- Harnesses set `unrestricted: 0` (:74, :197) and never call `Pass::unite` or the Quest/Copy arms: the Θ weight sum (`State::unrestricted`) arithmetic at pass.rs:790/1014/1027 is unchecked by Kani, and is plain `+=`/`-=` (wraps in release). Its argument is in zones.rs:258-262 doc only. Finding candidate S3: move it into `State` methods (add_unrestricted/remove_unrestricted) and a harness, as the rules claim the arithmetic lives in harnessed functions (core-proofs.md:260-263). Not verdict-relevant for `check` (observer `()` has weight 0) but feeds size estimate -> derivation bound (S1).

## check/frames.rs (391)
- :328-337 `dual_literals` closure `args` builds `Vec<u64>` from `table.frame_terms(f)` (itself a Vec) per literal per axiom node — two allocations per framed axiom. P2 candidate, first-order checks only (no journey? check journeys are propositional).
- :364-389 `bring` allocates `stack` and `seen` HashSet per ∃/cut node. first-order only.
- :237 `eigen_terms[&a]` index panics if a ∀'s eigenvariable has no term in table — relies on canonical closure. ok.

## check/error.rs (549)
- :428-487 five arms repeat `if names.is_none() { f.write_str("formula ")?; } cut(f, names, *o, limit)?` (Kind, Missing, NotACut, NotUnderQuest, Instance): a helper `member(f, names, o, limit)`. E6 small.
- :509-540 `name(node, reading, occ)`: a Node→Rule map (Tensor→Tensor, Quest→Dereliction, Bang→Promotion…). Check whether derivation/build.rs has another Node→Rule map (E6).
- :25 `Dyadic` pub, `#[non_exhaustive]`, pub Vec fields; not named outside core (surface). fine as report data.
- :507 `impl std::error::Error for CheckError {}` — C-GOOD-ERR ok; Display lower case.
- Consumer point 3 confirmed: `Refusal` (core/src/limits.rs:593) has only `code`, `setting`, `is_stop` (:708-752), every variant non_exhaustive: no constructor. Same for `Refused` here (error.rs:191, non_exhaustive, no ctor) — correct for a checker's error.
- Consumer point 5: `Proof` docs say it (proofs/mod.rs:350-351: "Whether it proves its sequent is a separate question, which check answers"); `Verdict` (search/mod.rs:3411-3433) does not; `Outcome::checked` (search/mod.rs:3404-3408) does. So: correct the point to Verdict/Disproof::new, not Proof.
- Consumer point 11 confirmed with a nuance: `ordinary::Countermodel::check` (ordinary/model.rs:55) is a one-line wrapper over `Model::refutes` (model.rs:449) and its doc says "as Model::refutes does"; `refutes` doesn't point back. Two names for one check.

## derivation/view.rs (775)
- :130-160 `ViewOptions` (linear::proofs) holds `structural` (LK/LJ only), `layout` (rule files only), `classical` (front ends only). Users: ordinary/derivation.rs:10-11, ordinary/interactive.rs:28, ordinary/tableau.rs:28, protocol/rules.rs:226, settings.rs:16,137. D4: shared option value lives in linear suite.
- :720-741 `Derivation::formula` duplicates `Proof::formula` (mod.rs:679-705) and `Interactive::formula` (interactive/mod.rs:824-849): same instance branch + reading/forest arms, each a String returned as `impl Display`; Proof's cuts terms at TERM, the other two pass None. clippy redundant_clone at classical.rs:1289, size.rs:1120, first_order.rs:972 on `.formula(m).to_string()`.
- Derivation has no `ids()` (Proof has); export/typst.rs:890 builds `InfId::new(i as u32)`.
- `InfId` used by ordinary, lambda/draw.rs:10, rules/draw.rs:10, serialize/ordinary.rs:373.

## derivation/build.rs (920)
- :104-107 `made.unfold(proof, &Framed::new(proof, table), stop)` then :144 `check::examine(...)` which re-dispatches on `proof.instances()` (pass.rs:184-189) and builds another `Framed::new` — two frame passes; and `Record<I>` × pass `I'` instantiated for all 4 pairs, 2 unreachable. Same in size.rs:1003-1009 + 1040.
- llvm-lines: `Pass::rule` 12 copies × ~3 425 lines; observers (), cut::Record, build::Record<Ground|Framed>, size::Measure<Ground|Framed> × pass Ground|Framed; 4 mixed pairs unreachable (~13 700 lines of `rule` alone).
- :411 `standard(id)` clones a Multiset per read; :750 clones the node's own just for `difference`. minor P3.
- :458 `self.done.split_off(...)` allocates a premises Vec per inference (DHAT derivation-chain-64: 130 of 2 873 blocks). Inference 88 bytes (type-sizes 42611): principal Option<usize> 16, premises Vec 24.

## derivation/mod.rs (940): tests only.

## derivation/classical.rs (1486)
- :28-106 `ClassicalRule` 37 variants = (one-sided Rule × side × implication) + 2 negation steps + Open; tables: name (171), UPRIGHT (155), SUBSCRIPT (163), ALL (111), rule() (217), side() (246), applied() (263). Contradicts core-proofs.md Decision (:671-674) "Rule is a one-sided rule, and Named adds the side... a new connective adds one rule, not three".
- :739-757 `sides` uses `hidden.contains`/`pending.contains` (Vec linear scans) per member: O(n·k) per sequent written; `negations` (:601-628) clones a growing `pending` per step. Not on a journey. low.
- :823-898 Drawn impl; :837 `Labels::Table(_)` ignored (upright).
- TwoSided has len/rule/premises/shows/sides but no ids iterator; test loops `0..view.len()` with `InfId::new(k as u32)` (:1271-1272).

## size.rs (1329)
- :30 `use std::collections::{HashMap, HashSet};` — SipHash beside the crate's foldhash `hash` module; 12 more stray std tables in core/src (first_order.rs:598, unify.rs:15, occurrences/instances.rs:947, interactive/quantified.rs:959,984, search/reference.rs:44, sequents/notation.rs:236, modal/model.rs:273, ordinary/model.rs:299, ordinary/instance.rs:771, ordinary/normal.rs:71, lambda/tests.rs:503); no clippy.toml disallowed_types.
- :1003-1009 + :1040 double dispatch/double Framed::new (see build.rs).
- otherwise saturating arithmetic consistent.

## fmt.rs (522)
- TextOptions in linear::proofs, used by ordinary/derivation.rs:1104, export/styles.rs:4; `fmt::write_text/text_size/write_steps` called by ordinary/derivation.rs:1088,1108,1131. D4 with ViewOptions.

## style.rs (453)
- :21-23 public module doc: "Every output draws a derivation through the crate-private trait `Drawn`, which the linear derivations and those of LK and LJ implement." and :227-230 same; five implementors: style.rs:318, classical.rs:823, ordinary/derivation.rs:1149, lambda/draw.rs:597, rules/draw.rs:276. Stale.
- :53 `Labels::Table(BTreeMap<Named, String>)` keyed by linear `Named`: ignored by TwoSided (classical.rs:837), ordinary (ordinary/derivation.rs:1162), lambda (lambda/draw.rs:609), rules (rules/draw.rs:288); docs say so in export/latex.rs:647, typst.rs:726, svg/mod.rs:1441. Users of 4 of 5 drawings get no custom labels.
- :62-74 UPRIGHT/SUBSCRIPT: two more copies of the 45 names (rule.rs).
- Drawn used by lambda/draw.rs:12, rules/draw.rs:12, export/*.

## multiset.rs (172)
- :127 `is_subset` allocates the difference; used in debug_assert only (build.rs:893). fine.

## first_order.rs (1151, test only)
- :696-701 mutant loop clones `table` to call `Proof::with_instances` (takes Instances by value, reads only; clippy needless_pass_by_value mod.rs:472).
- :1137-1147 compares proofs field by field: `Proof` has no PartialEq (mod.rs:414 derives Clone, Debug; Forest occurrences/mod.rs:251 Clone, Debug only).
- :1005 println! in a test.
- Instances builder is public (occurrences/instances.rs:115-162): X3 at first order works.

## DHAT
- check-qbf-20-2: 129 111 blocks total; `Pass::just` Box<State> at pass.rs:912 (Ax) 31 743 + 30 720 (other just site), and the Bag table's first reserve 31 743 + 30 720: ~124 926 blocks (97 %). 138.1 M instructions (baselines.md:56).
- check-wide-m1-2048: 4 096 Box<State> from just.

## Proof::new and stop
- mod.rs:443-449: public `new` has no stop; crate engines call pub(crate) `build` (:500) with their stop; outside engine cannot (X2).
