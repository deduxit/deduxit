# Unit 4a: verdicts

16 confirmed, 2 candidate, 2 rejected.

## F-4a-01: confirmed

Opened core/src/ordinary/derivation.rs:741-798: `check` documents and returns `Error::ReadBack` for every broken rule, and for an empty derivation. core/src/errors/mod.rs:672-681 says of that variant "This is a defect of this crate" and :1022 gives it kind Defect, while `InvalidDerivation` (:707-716, :986) is kind Invalid. cli/src/check.rs:736-779 prints "invalid derivation in LK … : the proof read back is not a derivation of LK" for a derivation a user wrote; export/latex.rs:656 and typst.rs:736 document `ReadBack` "for a derivation of LJ read from JSON". The modal suite already does what the change asks (modal/decide.rs:287 maps its checker's failure to `ReadBack` in `decide`). No rules file records the present form as a choice (grep of core-ordinary.md, core.md, cli.md for ReadBack finds only engine-side defects).

Correction: Cost is medium to large, not small: besides the checker, every crate-side caller that relies on `check` answering `ReadBack` must map it (decide.rs:493, :498, :618, :885, ljt, herbrand, interpolant, the connection read-back, interactive), cli/src/ordinary.rs:651, :656, :1298 match `Error::ReadBack` from these paths, and the lock files and docs/src/resolution.md:118 show the message. `InvalidDerivation::calculus` is a `Box<str>`, where `ReadBack`'s is `&'static str`.

## F-4a-02: confirmed

Opened core/src/ordinary/mod.rs:1115-1169 and :1236-1266: `function`, `variable` and `predicate` are `pub(crate)` behind `any(test, feature = "parse")`, `binder` is `pub(crate)`, `first_order()` (:1050) is `pub(crate)`. target/audit/public-api.txt:5323-5333 lists `Formulas`' public methods and none makes or reads a term or an atom's arguments; `Inference::term` returns an `FoTermId` nothing public reads. consumer-test.md:148 lists first-order sequents as not covered. No rules file records the privacy as a choice.

Correction: Two details of the claim are off. A binder can be built today through the public `Formulas::add(Node::Forall(body))` (mod.rs:888-897, named `x`); what cannot be built is a term or an atom with arguments (apart from `equals`). And `function` and `predicate` do not refuse foreign term ids as the change says: they pass `args` to the interner unchecked (mod.rs:1134, :1167), so a public form must add that check.

## F-4a-03: confirmed

Opened core/src/ordinary/mod.rs:162-171 (`#[default] Intuitionistic`), :590-596 (`Options` derives `Default`, is `#[non_exhaustive]` with `serde(default)`), :43-50 (the module example says "intuitionistic logic, the default" and calls `Options::default()`). consumer-test.md:92-102 is the cited point 6 and names D1. core/src/settings.rs:170 and web/src/calls.rs:87 read the default. criteria.md:39-44 words D1 for the command's flags; the consumer test extends it to the library call, which is the witness here. Since `Options` is non-exhaustive, a user outside the crate has no constructor but `default()`, so `Options::new(logic)` is needed for the change.

Correction: Cost is large, not medium: the change crosses core (settings.rs, the serde form of `Options` with `logic` required or defaulted elsewhere), web/src/calls.rs and web-tab, the docs, README and the guide's examples.

## F-4a-04: confirmed

Opened core/src/ordinary/mod.rs:546-562: the doc says the connection prover decides classical logic only and names neither grounding nor the model search, while the code answers true for `Connection` in intuitionistic logic and for `Grounding | Models` in every logic. :113-114 puts "The translations into linear logic." on `mod table;`, whose own header (table.rs:4) says truth table; translate.rs:4 carries the same sentence as its inner doc. :162 and :1611 say "propositional" although the module doc (:40) and the first-order API say otherwise.

Correction: `table` and `translate` are private modules, so the misplaced line shows only in the private docs; the `decides`, `Logic` and `Sequent` lines are the public part.

## F-4a-05: candidate

Opened core/src/ordinary/derivation.rs:783-787 (a `Vec<&Inference>` per inference), :894-908 (both sides cloned, then cloned again per premise in `with`), :1502-1526 (`members`). target/audit/dhat.md:9 gives ordinary-pigeons 60 124 blocks for 27 026 459 instructions (449 per block, the lowest of the table), and :82-86 the frames quoted. `check_inference(…, &premises)` is called with a collected vector at derivation.rs:793, interpolant.rs:837 and six sites of interactive.rs (1762, 2281, 2802, 3553, 3870, 5484: eight sites, the finder listed seven with slightly different lines). core-ordinary.md:1681-1682 already names "one allocation per inference" as the next step for `Inference`'s storage. Which share of the 27 910 `NodeId` blocks is the checker's and which the read-back's the frame names do not say.

Correction: Measure: DHAT blocks and callgrind instructions of the journey `ordinary-pigeons` before and after, with the DHAT frames split by caller (check_inference against read_back_with) first, so the change goes where the blocks are; the checker's mutation baseline after. The site list is eight, not seven (interactive.rs:3870 is missing from the finding).

## F-4a-06: candidate

Opened core/src/ordinary/mod.rs:926-938 (`atoms.push(name.to_owned())` then `names.insert(name.to_owned(), index)`), :1018-1022 and :1224-1228 (the key cloned). target/audit/dhat.md:148 and :150: 4 012 and 4 001 blocks of read-tptp's 18 418 at mod.rs:936 and :937. Reading supports the claim; the instruction saving is not measured.

Correction: The `Arc<str>` form is an API break and touches the wire form: `Formulas::atom_names(&self) -> &[String]` is public (public-api.txt:5327) and the arena serializes its names. The index-table form (a table of indices into `atoms`, hashed by name) keeps both; hashbrown is not a direct dependency of the core crate today, so it is the new-tool path. Measure DHAT blocks and callgrind instructions of `read-tptp`, `read-text` and `ordinary-pigeons` before and after.

## F-4a-07: confirmed

Opened core/src/ordinary/derivation.rs:188 (`pub(crate) const ALL`), :292-332 (`ASCII`, 40 spellings, private), cli/src/interact/ordinary.rs:622-668 (the command's `ASCII` of one shown spelling per rule and its `LK` list of 29 names with the logic filter). duplicates.md:74, :79, :84 list the pair. `Logic::ALL` and `Method::ALL` are public, `Rule::ALL` is not. The two tables are copies of the library's knowledge.

Correction: The claim that the copies "drift independently of the parser" is too strong: cli.md:482-488 records the lists and the test `listed_rules_read_back` (cli/src/interact/mod.rs:1224-1264), which reads every listed name and shown spelling through the library's `FromStr`, so a wrong spelling fails a test; only a rule missing from the command's list goes unseen. The command shows one spelling per rule on purpose (the word or `->L`, since its word splitter takes `\` as an escape), so `Rule::spellings` must let it choose that one, or the command keeps the choice of spelling and takes only the list of rules (`Rule::ALL` with the logic's `shape` filter).

## F-4a-08: confirmed

Opened core/src/ordinary/instance.rs:122-158 (`mentions`), :247-285 (`largest_eigenvariable`), :375-411 (`metas_in`) and herbrand.rs:344-377, :759-791: all five have the same node walk (two seen-sets, two stacks, the same `match` on `Node`) and then a term loop that differs only in what it does with each term once. One iterator over the reachable terms serves all five; `mentions` tests `u == term` before the seen-set, which a `find` over each-term-once reproduces. No rules file records the copies as a choice.

Correction: `mentions` is the checker's eigenvariable condition, so the change touches the checker (the finder says so); `mentions` returns early at the first hit, which a lazy iterator keeps and a collected one would not.

## F-4a-09: confirmed

Opened core/src/ordinary/derivation.rs:496-504 and :646-649: the derivation holds `formulas`, `left` and `right` as three fields and `sequent()` returns the two slices. cli/src/check.rs:743-755 rebuilds the text by hand; cli/src/grade.rs:366, :409 and several library callers (interpolant.rs:361, :801, herbrand.rs:2001, tableau.rs:280) take the pair apart again. `ordinary::Sequent` (mod.rs:1670-1723) is exactly an arena and two lists with a `Display`. No rules file records the pair as a choice.

Correction: `Sequent::new` refuses a conclusion with a loose bound variable (mod.rs:1691-1693), which `Derivation::from_parts` and the JSON reader do not check today, so the change must say where that is checked (the reader) or build the inner sequent without it. The pair `Interactive::sequent()` at cli/src/interact/ordinary.rs:611 is another type's method and would need its own change. A smaller, additive form is `Derivation::conclusion(&self) -> Sequent`.

## F-4a-10: rejected

Opened core/src/ordinary/decide.rs:618 and :885 and derivation.rs:1609, :1872. Both sides of the comparison use the one private constant `OPEN_WITNESS` (the read-back fails with `OPEN_WITNESS.to_owned()`, the callers compare with the same constant), so rewording the message cannot turn an unknown into a defect: the harm the finding states does not exist. The criterion does not cover it: AIP-193 is no identifier of criteria.md, and the rubric applies the AIPs "for the JSON wire forms only" (audit-rubric.md:95-110, a client branches on the reason of a wire error); this is control flow inside one module. core-ordinary.md:451-455 records the mechanism (the read-back answers `OPEN_WITNESS`, `within` and `Image::checked` turn it into `Reason::Incomplete`). A finding against it would need another criterion and a harm.

## F-4a-11: confirmed

Opened core/src/ordinary/bruijn.rs:23-37 (`Map`: Shift, Substitute, Replace, Abstract) and :230, and instance.rs:536-566 (`Task`, `Replace::{Variables, Metas}`, `Substitution` with the same two memo tables keyed by `(id, depth)`). Both walk the arena and the term tables per node and depth and put terms in for a removed binder's variable. core-ordinary.md:329-336 and :2383-2388 describe each walker and give no reason for keeping two.

Correction: The gain is tidiness (one walk to learn and to fix), and the cost is higher than a plain medium: the merged walker carries the checker's find-only lookup (`instance`), and `instance.rs` relies on closed terms needing no shift (core-ordinary.md:332) where `bruijn.rs` shifts open ones, so the merged form must keep the checker's path free of the shifting case or show by the mutation baseline that it is covered.

## F-4a-12: confirmed

Opened core/src/kripke.rs:36-39 (the struct's doc: "worlds directly above it, whose reflexive and transitive closure is the order"), :97-124 (`Display` writes "; below …") and cli/src/modal.rs:308-331 (`worlds_text`, the modal command's own printer with "sees"); cli/src/ordinary.rs:953, :970 print the ordinary model by the shared `Display`. core/src/modal/model.rs:493 has a `Countermodel` with no `Display`. cli.md:345 names `worlds_text` without a reason against the library printing it.

Correction: The module doc (kripke.rs:4-17) already gives both readings; what is in ordinary logic's words is the struct's doc, the JSON paragraph (:24-25, "directly above") and `Display`. `worlds_text` also marks a chosen world ("where the sequent fails", a `marked` argument used at modal.rs:526) and sorts and dedups the arrows, so the library's modal printer needs that parameter to replace it.

## F-4a-13: confirmed

Listed the crate's public `_within` and `_default` functions: fourteen `X`/`X_within` pairs or forms (`parse_within` 7, `derivation_within` 3, `decide_within` 2, `prove_within`, `check_within`) against four `X_default` (negative.rs:520, herbrand.rs:195, interpolant.rs:317, normal.rs:3178), where the plain name is the long form. core-ordinary.md:203-204 records that the suite follows "the linear suite's `prove`/`prove_within` shape" (the author's Q5), which the four break. C-WORD-ORDER (criteria.md:68) asks for one word order across the crate; its example is error names, and the finding applies it to this pair of suffixes, which is the same rule. No caller of the four `_default` names in cli/src, web/src, docs/src or skills.

## F-4a-14: confirmed

Opened core/src/ordinary/mod.rs:639-672: `method()` ignores the sequent, `method_for` answers grounding or the connection prover for a first-order sequent and otherwise calls `method()`. Callers of `Options::method()` are `method_for` (:672) and the test at decide.rs:950-969; every `.method()` in cli/src is `Outcome::method`. public-api.txt:5431 lists it.

Correction: The guide names it: skills/deduxit/reference.md:2059-2063 says both `decide` calls "decide by `options.method()`", which is wrong at first order and must change in the same commit (the readme test checks the names the guide uses). core-ordinary.md:205 names it too.

## F-4a-15: confirmed

Opened core/src/ordinary/decide.rs:403 (`pub const CONNECTION_WORK`) and :445 (`pub const EVIDENCE_WORK`), mod.rs:71 (`mod decide;`, private) and :125-127 (the re-export list, without `CONNECTION_WORK`). public-api.txt:5585 has `EVIDENCE_WORK` and no `CONNECTION_WORK`; the only uses are in decide.rs (:406, :423, :429, :1303).

## F-4a-16: confirmed

Opened core/src/ordinary/translate.rs:677-688 (a private hash-consing `Builder` of linear terms and atoms) and :587-595 (a struct expression of `linear::Sequent`'s private fields, then `optimize()`). plan/notes/api.md:354 has R59, the checked builder that waits for its first caller. The witness holds; the finding is conditional on F-arch-05 and is a follow-up of it, not a change by itself.

Correction: The translation sets more than the builder of R59 offers: the first-order tables `fo` with their binders (translate.rs:585-586, :593) and `antecedents`, and it relies on the terms' order (the read-back matches roots by position, the size estimate counts them). If the public builder cannot take those, the struct expression stays; the finding should be read as "use the builder if it covers this", and the value is one reader's, at a risk to the image's identity that the ratchet and the read-back tests guard.

## F-4a-17: confirmed

Opened core/src/ordinary/mod.rs:771 (`Formulas`: Clone, Debug, Default) and :1669 (`Sequent`: Clone, Debug), core/src/linear/sequents/mod.rs:186 (`linear::Sequent` derives PartialEq, Eq), core/src/modal/mod.rs:143, :388 (the modal pair lacks them too). No rules file gives a reason (grep for PartialEq in core-ordinary.md and core-modal.md finds none). C-COMMON-TRAITS (criteria.md:69) says what the finding says.

Correction: The equality is of the arena as stored, as in the linear suite: two sequents with the same formulas built in another order compare unequal, and the docs must say so; whether a binder's printing name counts (`∀x` against `∀y`) is a decision the change must make and state.

## F-4a-18: confirmed

Opened core/src/ordinary/mod.rs:217-227, :363-373, :577-587 and negative.rs:150-160: four identical `from_str` bodies but for the `what` text; the same body has 16 sites in core/src (grep for `ok_or_else(|| Error::UnknownName`). `Method::NAMES` (:454-462) restates `ALL`'s `cfg` list.

Correction: The "six places" overstate the second half: `name`, `title`, `logics` and `decides` are exhaustive matches the compiler checks under every feature set, so only `ALL` and `NAMES` are lists that can fall out of step. The helper would serve all 16 sites of the crate, beyond this unit. The gain is tidiness at small cost.

## F-4a-19: confirmed

Opened core/src/ordinary/instance.rs:770-772 and model.rs:299-303: both name `std::collections` tables with the default hasher; the crate's other tables come from `crate::hash`. A grep of core/src finds the same at modal/model.rs:273, linear/occurrences/instances.rs:947, linear/proofs/interactive/quantified.rs:959, :984 and others, and no `clippy.toml` exists at the root, so no lint settles it. P6 (criteria.md:198-201) names exactly this and `disallowed_types` as the means. Neither table is iterated, so order is not observed.

Correction: The finding says itself that neither site is hot: the gain is one hasher crate-wide and the lint, no measurable time.

## F-4a-20: rejected

Opened the seven places (mod.rs:720, derivation.rs:543, translate.rs:73, :415, :537, negative.rs:342, decide.rs:566). The writing rule (writing.md, rule 4) bars "refers to" and "stands as" where they stand in for "is" or "has". Here the verbs carry their own meaning: a node refers to its subformulas by arena index (a reference, which "is" does not say), the inferences refer to formulas of the arena, a negation refers to the node false, "atoms and their terms stand as they are" means they stay unchanged (not "stands as" at all), and "in ILL a hypothesis stands as its dual" says where it appears in the image; the proposed "a hypothesis is its dual" is false. Only decide.rs:566 ("the image's proof stands as `Evidence::Image`") is the barred pattern, and one site is no finding. target/audit/text.md is a pattern match, a pointer and not a finding (brief, "Where everything is").
