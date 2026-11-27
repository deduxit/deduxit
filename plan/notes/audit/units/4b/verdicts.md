# Unit 4b: verdicts

12 confirmed, 4 candidate.

## F-4b-01: confirmed

Opened all six `push` bodies (ljt.rs:931, sat/read_back.rs:840, connection/intuitionistic/read_back.rs:1233, connection/read_back.rs:514, herbrand.rs:1176, derivation.rs:1779): each asks or is given the stop, charges memory by a formula of its own and refuses past u32 with the same `Refusal::Index`. `weaken_to`, `contract_to`, `in_order` and `difference` are the same code in ljt.rs:1032-1100 and connection/intuitionistic/read_back.rs:1283-1340. `Derivation::from_parts` is `pub(crate)` (derivation.rs:605). core-ordinary.md:183-190 records that every read-back charges `limits.memory_bytes`, not how much, so no recorded choice stands against one charge rule; but herbrand's charge was fitted to a measured peak (core-ordinary.md:2526-2535), so the shared rule must be at least as conservative, and the refusal points move as the finder says.

Correction: Only two builders hold all four helpers under those names (ljt, connection/intuitionistic). sat/read_back.rs has `weaken` (:892) and `without` (:782) with `in_order` (:954) and no `contract_to`; the line 893/782 given are those. LJT's `weaken_to` and `contract_to` carry the test's plant (`self.planted(Plant::Weakening)`), which a shared builder must keep for `a_planted_wrong_translation_is_refused`. derivation.rs:1779's `push` takes members with sides and no stop, so it fits the shared builder least. Kind: refactor for the shared builder; making it public is an API addition that belongs with the architecture reader's engine interface (F-arch-03 to 05). Cost large stands.

## F-4b-02: confirmed

target/audit/llvm-lines.txt lines 79-80, 109-110 and 192-193 show two copies each of `maehara` (2 089), `interpolant::checked` (1 822) and `NormalForm::check` (1 360), the second for the `_default` helper's closure. interpolant.rs:784-788 takes `stop: &mut impl FnMut`, :385-392 `mut stop: impl FnMut`, normal.rs:1074 is generic; normal.rs:2304/2314 already has the shim (`rewrite`/`rewrite_dyn`), and core.md:327 records the rule that a long body takes `&mut dyn FnMut`. Nothing public changes its signature.

## F-4b-03: confirmed

bench/src/journeys.rs:40-139 lists the journeys; the only ordinary one is `ordinary-pigeons`, whose body (:473-512) calls `ordinary::decide_within` with `Translation::Affine`, the embedding. bench/ceilings.csv has that one ordinary row. cli/src/ordinary.rs:399-401 sends the default method to `sat::decide_preferring_ljt`, which no journey calls. P1 asks for a measurement before a change, so the gap is what the criterion names.

Correction: The place :137 is inside the `ordinary-pigeons` entry (:135-139); the function quoted is at :473. Kind `build` is the nearest of the form's kinds for an addition to the harness; nothing in core changes.

## F-4b-04: confirmed

normal.rs:632-643 has five `pub` fields and `is_checked` (:648) reads only `checks`; interpolant.rs:176-190 has four `pub` fields and `is_checked` (:204) reads the two outcomes. Both are `#[non_exhaustive]`, so an outside caller cannot build one but can assign a field of one it got, and `is_checked()` then vouches for a value no check saw. The rules name the public fields as a fact (core-ordinary.md:322, :2424), not as a choice with a reason; the note on `Evidence` (:1589-1593) accepts a like weakness for another type. cli/src/transform.rs:136 and :281 and cli/src/resolve.rs:370 take exit statuses from `is_checked`. About 67 uses of the fields in cli, web, protocol and serialize: medium is honest.

Correction: One loss the finding does not name: today a caller can set `result` by hand and call `check` to have another normal form judged (the crate's test at normal.rs:3347 does it, and `replay` was hardened for it, core-ordinary.md:2424). With private fields that use needs a constructor, or it goes; the author should choose.

## F-4b-05: confirmed

written/mod.rs:602-641: `review` takes `&Calculus` and its docs say it is `TABLEAU` read; `review_dyn` hands it to the rule file's checker unexamined. Both front ends read `TABLEAU` themselves (cli/src/tableau.rs:85, web/src/ordinary.rs:811-813). core-ordinary.md:1758-1759 records why the caller passes it (serde_json only under `protocol`), and the finding answers that record. C-VALIDATE says what the finding says.

Correction: The value is smaller than claimed. The review reads the rules itself first and answers the first fault before the calculus is consulted, and the derivation it writes names `TABLEAU`'s rules, so another rule file passed by mistake gives an error, not `Closed`; only a file with the same rule names and laxer rules would pass, and a check of names does not catch that one. `Calculus` derives only `Clone` and `Debug` (rules/calculus.rs:264), so a real validation needs a comparison written for it. Building `TABLEAU` inside `review` is the change that settles it, and it needs serde_json under the features that gate `written` (parse, serialize, interactive) or a constructor without JSON. Kind: the validation alone is no API break (a new error); taking no calculus is one.

## F-4b-06: confirmed

sat/classical.rs:57-91 and normal.rs:2872-2901 apply the same rule (flip under `¬` and left of `→`, both under `↔`), with the same constants `TRUE`/`FALSE` defined twice (classical.rs:36-39, normal.rs:2680-2682); normal's uses a `HashMap<NodeId, u8>` over the reached nodes where a `Vec<u8>` by index serves. sat/mod.rs:539-580 differs in `⊥` under a negation and the implications of an equivalence. core-ordinary.md:2349-2351 calls normal's "the classical encoder's rule". `normal` has no feature gate and `sat` has one (ordinary/mod.rs), so the shared function must live outside `sat`, as the finding says.

Correction: Merging the two equal copies is the sound part; the intuitionistic one is better left as it is (its extra rules are inside the loop). One cost to say in the commit: the Plaisted-Greenbaum form's promise is checked by the classical SAT method (`CLAIMS`, normal.rs:1511), whose encoder would then share the function under check, so a fault in it would be in both; today the two copies are a weak independence. Cost: small to medium.

## F-4b-07: confirmed

transfer.rs:74 `copy` takes `Some(roots)` or `None` and copies atoms and symbols by name, a bound variable by index and a binder by name. normal.rs:1338-1430 (`copy_formulas`, three std `HashMap`s) and :1437-1505 (`transfer`, the whole arena in index order) do the same for the cases without eigenvariables, refusing those with `ClaimFails`; written/mod.rs:769-786 copies a propositional arena node by node. `reached` (transfer.rs:290) handles open roots, which `copy_formulas` needs.

Correction: `transfer.rs` is behind the feature `interactive` (ordinary/mod.rs: `#[cfg(feature = "interactive")] mod transfer;`) and `normal` has no gate, so the module must lose its gate (or the part `Parameters::Constants` needs must), and both `cargo hack` runs are part of the net. `Parameters::Constants` turns an eigenvariable into a constant where normal refuses; the refusal must be kept by testing `Copied::constants` and `metas`, with the two `ClaimFails` messages ("a step holds…", "the input holds…") kept apart.

## F-4b-08: confirmed

normal.rs is 3 688 lines (target/audit/lines.md). The item list bears the seams out: the types and `impl NormalForm` at :79-1200, the claims' helpers at :1205-1545, stages, laws and passes at :1550-2531 with `quantifier_law` (:1624) and `apply_quantifier` (:1981) inside, Skolem's helpers at :2536-2677, the encodings at :2686-3000, the form test at :3011-3150, the tests from :3189. E5 asks for the seam, which is given. No rules file records the single file as a choice (core-ordinary.md:2303 only heads its section). A move only; the gain is for readers.

Correction: core-ordinary.md:2303 names the section by the file (`normal.rs`), so the rules file changes in the same commit.

## F-4b-09: candidate

tableau.rs:620-627 scans `self.nodes[..index]` backwards for the node whose children hold `index`, and `write_text` (:568) calls it once per node; the walk pushes each child from its parent (:585-597), so carrying `expands` on the stack is a local change that keeps the text. Reading supports a cost above linear; a measurement decides how much.

Correction: The claim's figure is too high in general: the scan runs backwards from `index` and stops at the parent, so it costs the distance to the parent. With nodes numbered depth first (core-ordinary.md:1750-1752) a first child's parent is the node before it; only a split's second branch scans back over the first branch's subtree, which is n log n for a balanced tree and quadratic for a comb. `write_text` also clones the prefix string per node, which may weigh as much. The 1.7 million inferences cited (core-ordinary.md:1681) are inferences of the derivation, not tableau nodes. Measure: pinned time of `deduxit prove --method tableau` with the text written, on ILTP's SYJ206+1.012 and on a one-sided chain of splits, before and after.

## F-4b-10: confirmed

sat/classical.rs:256-272 (`decide`) and :379-392 (`falsify`) are the same lines from `new_var` to `solver.conflicts()`, differing in `Batsat::recording` against `Batsat::new` and in the arms on the answer; `assignment` already takes `&impl Solver`, so a shared helper over the solver compiles. No rules record keeps them apart. The gain is tidiness (about 15 lines) on the path whose clauses the DRUP check reads; the checker is untouched.

## F-4b-11: confirmed

normal.rs:2710-2723 `prefix` is `fresh_prefix` (:2592-2609) with the base `t` and without the symbols' names, which a propositional arena has none of (`encoded` refuses first-order input at `rewrite_dyn`'s door). The `Negate` loops at :2625-2640 and :2732-2747 are the same lines. core-ordinary.md:2346 names `prefix` in its text, so the rules file changes with it.

Correction: The shared loop also sets what is left right of `⊢`, which `skolemized` keeps in `right` and clears after; `encoded` has no such variable. Small.

## F-4b-12: candidate

interpolant.rs:260-262 clones the derivation out of the outcome (`Ok(Some(d.clone()))`), :295 passes it by reference and :306 moves the outcome, which holds the same derivation, into the result. `Derivation` (derivation.rs:495-506) owns its arena and its inferences, so the clone is deep and the derivation is held twice until the call returns. A borrow from the chosen outcome followed by the move compiles in both branches.

Correction: P3 is about clones on hot paths and this is one clone per call, so the gain is peak memory under the bound, not time; the result's own memory is unchanged. Measure: DHAT peak bytes of `interpolate` on a valid sequent whose derivation has a few thousand inferences, before and after.

## F-4b-13: candidate

ljt.rs:518-520 makes a `HashSet` and a `Vec` per call of `heads`; `levels` (:539-570) calls it twice per hypothesis per round for up to one round per hypothesis, and `rank` (:493-496) once per implication with a `Vec` of its own. It runs only where more than one `→→L` alternative exists (:491). core-ordinary.md:639 records that this orders the alternatives and never prunes, and scratch storage keeps the order, so the kind is refactor: `ljt::Statistics` must be equal before and after.

Correction: `heads` and `levels` take `&self`, so scratch in `Search` needs `&mut self` up the call chain or a `RefCell`; still one file. Measure: DHAT blocks and callgrind instructions of LJT on Korn and Kreitz's family at size 20 (the journey F-4b-03 proposes), with the sequents and memo hits compared.

## F-4b-14: candidate

sat/drup.rs:211 keys the index by `Box<[u32]>`, :342 clones the literals into it, and :332-333 charges `2 * literals.len() * 4 + CLAUSE_BYTES`; the comment at :179-184 says the 8 bytes per literal are the clause and the key. Only `delete` (:354-372) reads the index. So each kept clause's literals are held and charged twice. No rules record chooses the copy (core-ordinary.md:1525-1540 describes the checker without it).

Correction: The saving is 4 bytes per literal beside 224 fixed bytes per clause (`CLAUSE_BYTES`), so for short clauses it is a few percent, not half; "about half the proof" holds only for long clauses. The clause's own literals are reordered by the watches, so a deletion must compare as sets after a hash hit. The change adds a case to the checker a classical "valid" rests on; a wrong match stays sound (a deletion only weakens) but must not happen silently. Measure: DHAT peak bytes and the account's charge of the check on `pigeons(7)`, before and after, to see whether the gain is worth that.

## F-4b-15: confirmed

normal.rs:3160-3166 (`normalize`), :1074 (`NormalForm::check`), interpolant.rs:249-255 (`interpolate`) and :350-355 (`interpolant`) each take `search: &search::Options` of `crate::linear::search`. F-arch-02, which this follows, is confirmed (target/audit/units/arch/verdicts.md:21). It adds four places to that finding and depends on it: alone it changes nothing.

Correction: "Reads none of it" holds for the propositional claims with the feature `sat` only (`CLAIMS`, normal.rs:1511). First-order claims go to the classical default (core-ordinary.md:2429-2431), which reads the options, as does a build without `sat`; and `interpolate` passes them to `decide_within`. So the parameter cannot simply go: it becomes the ordinary options F-arch-02 makes. Best folded into F-arch-02's change as four more places.

## F-4b-16: confirmed

sat/mod.rs:458-493 (`prefer_ljt`) maps `Evidence::Clausal`, `Image` and `Table` to `Ok(derivation)` and :498-531 (`by_ljt`) maps the same three to `NoDerivation` and `NotBuilt`, with the same message "LJT refutes a sequent the SAT search proved" at :484 and :530. At :374-379 a `Refusal::Output` of zeros is built on every valid decision and reaches `Err(refused)` (:393) only where `derive` is `Derive::Ljt`, a variant under `#[cfg(all(test, feature = "parse"))]` (:264). The gain is tidiness at small cost.

Correction: The two functions treat an unknown differently on purpose (`prefer_ljt` keeps the derivation read off, `by_ljt` answers the read-back's refusal), so the shared helper can cover only the valid and refuted arms. `unreachable!` in place of an error is a choice for the author: one arm that maps the three impossible evidences to the same error in both functions mends the inconsistency without a new panic.
