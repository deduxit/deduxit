# Unit 4a notes

## mod.rs
- mod.rs:113-114 `/// The translations into linear logic.` sits on `mod table;` (the truth table); `mod translate;` (120) has no outer doc. Wrong doc on module.
- mod.rs:43 crate doc example "The excluded middle is not valid in intuitionistic logic, the default:"; Logic #[default] Intuitionistic at :171; Options derives Default (:593) (consumer pt 6, D1).
- Options: pub fields + with_logic/with_translation/with_method (pt 7 stale). method() vs method_for(sequent): method() wrong for first-order; two public answers.
- Formulas: public builders only atom/add; function/variable/predicate/binder pub(crate) under any(test,parse) (:1123,1140,1155,1245); tables first_order() pub(crate). Atom(u32) of a first-order atom gives only canonical text "p(f(a), #0)" via atom_name. => first-order formula can't be built or inspected via public API (X1).
- NodeId: index() only, no ids() iterator over arena (linear Forest has ids()).
- Logic/Translation/Method: same FromStr body x3 (:212,:358,:572) + negative.rs:154; Method's cfg list repeated in ALL, NAMES, name, title, logics, decides (6x cfg(sat)).
- between() (:1346) groups Iff with impossible arms; fine.
- Formulas/Sequent: Clone+Debug only; no PartialEq (C-COMMON-TRAITS), low value.
- Method doc :426 links linear search::Options::model_size (F-arch-02).

## parse.rs
- identifier/word copies: recorded on purpose (perf). skip.
- read_tptp retry loop reparses whole file per rename, `fresh` scans words(code) each: quadratic in renames; cold.
- Problem.status: linear::lltp::Status (F-arch-07).

## translate.rs
- translate.rs:677 private `Builder` (hash-consed linear arena + atoms) then struct expression `crate::linear::Sequent { terms, roots, atoms, antecedents, cuts, fo }` + optimize(): crate needs the linear builder F-arch-05/R59 proposes; evidence.
- :548 `_ => (0, 0)` wildcard over (Translation, Side).
- :504-520 Lollipop's real arm grouped with impossible cores.
- :638-644 and :667-673 identical "find or push empty Args list".
- translate clones sequent (:371) - needed.

## derivation.rs
- Derivation::check errors are Error::ReadBack (kind Defect, doc "This is a defect of this crate", message "the proof read back is not a derivation of LK: ...") for ANY broken derivation incl. one read from JSON / built outside. errors/mod.rs:672-681, :1022. protocol/ordinary.rs:758 maps Invalid|Defect both to Failed; cli check.rs:736-739 treats any non-refusal as No. X3 / one error family.
- check_inference allocs per inference: context clone :894, with() clone :901, rest() clones :930, premises Vec :783; doc of same() :1221 says "checker allocates nothing for short sequents". candidate (ordinary-pigeons journey runs check).
- or_else retries (:1011, :1042, :1048) format an Err String then drop.
- Inference fields pub(crate), from_parts pub(crate) (F-arch-05). trusted flag private, no getter (F-arch-23).
- Derivation::sequent() returns a tuple (&[NodeId],&[NodeId]) while Sequent has left()/right(): two shapes.
- Rule tables: LABELS :151, name() :226, ASCII :292; duplicates.md: cli/src/interact/mod.rs:936 and cli/src/interact/ordinary.rs:626 copy derivation.rs:151-160.
- rule_of :1699 `_ => return None` tuple wildcard (ok).

## decide.rs
- decide.rs:403 `pub const CONNECTION_WORK` in private module, not re-exported (mod.rs:125-127 re-exports EVIDENCE_WORK only); no use outside decide.rs. Unreachable pub; sibling EVIDENCE_WORK public.
- decide.rs:618 and :885 match `Error::ReadBack { reason, .. } if reason == super::derivation::OPEN_WITNESS` - control flow by comparing an error's message text (sentinel string, derivation.rs:1872).
- with_evidence :493/:498 match Error::ReadBack as "defect" marker.
- Outcome: three Option countermodel fields (F-arch-13 agree); Verdict::Unknown(linear::search::Reason) (pt10).
- Outcome built by From<ljt/sat/grounding/models/connection::Outcome> five near-identical impls (each fills countermodel/structure/kripke None).
- `ordinary::Outcome` non_exhaustive no ctor -> outside engine can't answer (X4, analog of F-arch-04): remarks.
- decide_within takes &search::Options (F-arch-02, agree: Method::Models doc points to search::Options::model_size).

## bruijn.rs / instance.rs
- Two substitution walkers over the arena: bruijn.rs Map/mapped_in (:222-291, memo HashMap<(NodeId,u32),NodeId>) and instance.rs Substitution::run/make/substitute (:563-765, same memo). Map::Substitute vs Replace::Variables. instance's Find maker is the checker's lookup.
- Five copies of "terms of formulas, each node/term once" two-stack walk: instance.rs:122 mentions, :247 largest_eigenvariable, :375 metas_in, herbrand.rs:345, herbrand.rs:760 (already returns Vec of terms). Also bruijn.rs:106 free_variables (with depth).
- SipHash: instance.rs:771 std HashSet in eigen_names; model.rs:299 std HashMap.

## equality / model / table / negative / kripke
- model.rs:299 std HashMap (SipHash) per model check; cold.
- Countermodel::check = one-line wrapper of Model::refutes (pt 11, F-arch-21 agree).
- Error::Model is kind Invalid (errors/mod.rs:984); Error::InvalidDerivation (Invalid, :986) is what rules::Derivation::check uses; ordinary checker uses ReadBack (Defect :1022). cli/check.rs:778 prints "invalid derivation in LK of …: the proof read back is not a derivation of LK: …".
- kripke.rs:36-39 Model doc "arrows ... directly above ... reflexive and transitive closure is the order" (ordinary reading); Display :118-121 "below"; modal CLI writes own worlds_text (cli/src/modal.rs:308) with "sees".
- negative.rs:468 compare(full) / :520 compare_default(short) vs decide/decide_within; also herbrand.rs:195, interpolant.rs:317, normal.rs:3178.
- negative::Translation FromStr :145 = 4th copy.

## measurements (dhat.md)
- ordinary-pigeons: 60 124 blocks / 27.0M instr = 449 instr/block, lowest of all journeys. with_capacity<NodeId> 27 910 (46 %), <InfId> 8 271 (13 %), <(NodeId,Side)> 4 647 (7 %), <&Inference> 4 135 (6 %) (= derivation.rs:783, only site on that journey).
- read-tptp: Formulas::atom mod.rs:936 4 012 blocks + :937 4 001 blocks = 43 % of 18 418 (name stored twice). read_tptp parse.rs:942 (lltp::clauses) 6 004 blocks 32 % -> unit 2c/1a remark.
- type sizes: ordinary::Outcome 400 B, Inference 104 B (3 Vecs), Formulas 136 B, Sequent 184 B.

## other
- Rule::ALL pub(crate) (:188), NAMES/ASCII private; cli/src/interact/ordinary.rs:625 ASCII copy, :657 LK name list copy.
- cli/src/interact/ordinary.rs:672 binders(): walk without seen set over hash-consed DAG -> exponential on a shared JSON arena (unit 7b remark).
- Options::method() (mod.rs:641): no caller outside mod.rs/decide tests; all front ends use method_for.
- Derivation::sequent tuple (derivation.rs:647); cli/check.rs:743-757 rebuilds "Γ ⊢ Δ" text by hand.
- ordinary/modal Sequent & Formulas: no PartialEq; linear Sequent has Eq.
