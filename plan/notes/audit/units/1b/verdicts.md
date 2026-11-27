# Unit 1b: verdicts

Verifier on Fable 5.2, medium effort. Every place was opened in
`/home/tux/Projects/own/deduxit-wt-audit`. Nothing was built or run.

## F-1b-01: confirmed

The witness is at the place: `ViewOptions` (view.rs:130-160) holds
`structural` ("A linear derivation ignores it") and `layout` ("The other
suites' derivations are trees whatever this says"). The imports quoted are
at ordinary/derivation.rs:9-12, :1088, :1108, :1131, lambda/draw.rs:10-12,
rules/draw.rs:10-12, protocol/rules.rs:226 and settings.rs:16. `Drawn` has
the five implementors named. No rules file records a reason to keep the
drawing layer in the linear suite; core.md:75-77 says the opposite ("an
item that a second suite comes to share without being linear logic moves
to the root then"). D4 says what the finding says. The cost `large` is
honest. One addition: `ViewOptions` also holds three fields only the linear
suite reads (`compact`, `sides`, `classical`), so the move takes the whole
struct or splits it, and the change should say which.

## F-1b-02: candidate

The witness is at pass.rs:764-778, and `just` is called at pass.rs:912,
:916 and :920 (three sites, not "912 and the other"). `dhat.md` for
`check-qbf-20-2` shows 62 463 blocks from `Box<State>::new` and 64 896
from a `reserve_rehash` that the linker names by another table of the same
layout (`(Eigen, FoTermId)`), out of 129 111: the share is as claimed, the
split by site I did not open in the JSON. Reading supports the claim; the
gain is unmeasured. To measure: `check-qbf-20-2` and `check-wide-m1-2048`
under DHAT (blocks) and callgrind (the ratchet's counts) before and after.
What the change must keep: the size estimate counts each zone's table by
`table_bytes(most, …)` (zones.rs:29, :84, :212), `the_bound_is_exact` and
`holds_no_more_than_its_bound` pin it, and Kani's harnesses build `Bag`
directly, so an inline small zone changes both the accounting and the
harnesses. The cost is `medium` at least.

## F-1b-03: candidate

The witness holds: `examine` (pass.rs:176-190) chooses `Ground` or
`Framed`, and so do `size::measured` (size.rs:1003-1009, which then calls
`check::examine` at :1040) and `derivation::build` (build.rs:104-107, then
:144). `llvm-lines.txt:20-31` lists `Pass::rule` twelve times at 3 422 or
3 428 lines, four of them for pairs whose observer and pass disagree
(`Record<Ground>` with `Framed` and so on). The `match` is on a run-time
value, so the compiler cannot drop those four. The count is of LLVM IR
lines before optimisation, not bytes, and the first-order cost is from
reading. To measure: `cargo llvm-lines` and the web module's gzip bytes
before and after, and a first-order derivation under callgrind (no journey
has one). `examine_with` takes a closure because a failure runs the pass a
second time (pass.rs:192-194), so the change must still give the second
pass a fresh instancing.

## F-1b-04: confirmed

The witness is at style.rs:53, classical.rs:837, ordinary/derivation.rs:1162,
lambda/draw.rs:609 and rules/draw.rs:288. Corrections:
- "Silently" is too strong: the gap is documented at
  ordinary/derivation.rs:1095, export/latex.rs:647, export/typst.rs:726 and
  export/svg/mod.rs:1441, and the rules files state it as a fact
  (core-export.md:144-145, :361; core-derivations.md:348-349). They give no
  reason for it, so no recorded choice stands against the finding.
- A table keyed by `String` loses the check of the key: today a rule name
  that `Named::from_str` does not know fails when the JSON or
  `--style labels.table.RULE=LABEL` is read. The change needs a check of
  its own (against the names of the derivation drawn), or a typo passes
  unnoticed. `Named::from_str` also takes ASCII spellings (`-oL`), which a
  string key would have to keep.
- `Drawn::markup` takes a rule's number, not its name (style.rs:247), so
  every implementor needs a table from number to name. With the three
  export targets, the command's `--style` and the manual
  (docs/src/output-formats.md:291-315), the cost is `large`, not `medium`.

## F-1b-05: confirmed

The three updates are at pass.rs:790, :1014 and :1027, a fourth site swaps
the sum (pass.rs:786), and both harness states set `unrestricted: 0`
(harnesses.rs:74, :197). core-proofs.md:260-263 reads as quoted and lists
no function that changes this sum, so the rule does not hold for it. The
change is sound; `quest_weight` reads the observer and the instancing
(pass.rs:697-703), so the `State` methods take the weight as an argument.
I could not build a proof on which the `-=` at :1014 underflows, and the
finder shows none: the remark on a release build that wraps is a
possibility, not a witness.

## F-1b-06: confirmed

The witness holds: 37 variants (`ALL: [Self; 37]`, classical.rs:111),
`UPRIGHT`, `SUBSCRIPT`, `name`, `rule`, `side` and `applied`
(classical.rs:155-260). The decision is at core-proofs.md:671-674 as
quoted, and no rules file gives a reason for the flat enum. Two limits on
the value:
- Half of the decision's reason does not carry over. `Named` lost
  unreachable arms; every one of the 37 variants here is reachable.
- The struct proposed admits values no derivation has (a negation step
  without a side, an implication flag on `1`), which the enum does not. It
  needs a constructor that refuses them, or private fields.
Outside the unit only web/src/calls.rs:1891 matches on the type, on two
variants. The gain is mostly tidiness for a reader, at `medium` cost (the
label tables are indexed by declaration order and `every_label` pins them).

## F-1b-07: confirmed

The three bodies are at proofs/mod.rs:679-705, derivation/view.rs:720-742
and interactive/mod.rs:824-850, the same but for where the framing comes
from and the cut (`Some(describe::TERM)` in the first, `None` in the other
two). Each returns a `String` as `impl Display`. Corrections:
- The Clippy sites were not checked by a run; the callers that copy the
  string again are there (classical.rs:1289, size.rs:1120,
  first_order.rs:972, cli/src/eliminate.rs:748, web/src/calls.rs:1906).
- The cut must stay a parameter, as the change says, and
  `Derivation::formula` must stay uncut: first_order.rs:972 counts its
  characters against the size estimate. The value's "the same cut wherever
  it asks" would break that.
- Returning `String` changes three public signatures; with the signatures
  kept, the kind `refactor` holds.

## F-1b-08: confirmed

The witness is at proofs/mod.rs:21-48. In `public-api.txt` every item
under `proofs::check`, `derivation`, `size` and `style` is one of the 19
types `proofs` re-exports, so making the four modules private removes no
item. No rules file records a reason for the public modules; core.md:230
only asks that no public module is left without public items. Outside
`core/src` only web/src/calls.rs:1254 and :1304-1305 write a nested path.
Notes: the change leaves two paths (`linear::X` and `linear::proofs::X`),
so it halves the duplication without ending it; `cut` and `interactive`
have the same shape and are outside the finding; the module docs of
`check`, `derivation` and `size`, not only `style`'s, need a new home.

## F-1b-09: confirmed

The witness is at proofs/mod.rs:443-449 and :490-507: `new` passes
`&mut || false`, `with_instances` does the same to `reach` (:477), and
`build` is `pub(crate)` with one caller (search/mod.rs:1453). No rules file
gives a reason. Corrections: the kind is an addition, not a break (new
`_within` methods beside the old ones). `build`'s stop is `FnMut() -> bool`
and its refusal names `Phase::Search` (mod.rs:519-523), so the public form
must settle the phase. `instances::canonical` in `with_instances` takes no
stop either. The value is real but modest: the two passes are linear in the
nodes.

## F-1b-10: confirmed

`Proof` (proofs/mod.rs:413-414) and `Forest` (occurrences/mod.rs:251)
derive `Clone, Debug` only, while `Node`, `Sequent` and `Instances` have
`PartialEq, Eq`. The trait fits and the change is additive (the kind is
not a break). The witness does not show what the claim draws from it,
though: first_order.rs:1140-1141 says "The reader numbers the argument
lists in another order than the parser, so the sequents compare by their
text." A derived equality would call that very round trip unequal, so the
test compares field by field for a reason an `Eq` impl does not remove,
and the value "a user checks a round trip through JSON in one
`assert_eq!`" does not hold for a first-order proof. What remains is
structural equality of two proofs built over the same sequent value; the
doc of the impl must say that it follows the arena's numbering.

## F-1b-11: confirmed

`with_instances` (proofs/mod.rs:470-484) only passes `&instances` to
`instances::canonical`. The value is smaller than stated: of its callers
(cut/mod.rs:639, search/mod.rs:1450, interactive/terms.rs:59,
serialize/proofs.rs:318, first_order.rs:122, cut/tests.rs:711) every one
owns its table and drops it; only the test loop at first_order.rs:696-701
clones, and it clones the forest too. The break touches the doc example at
occurrences/instances.rs:85. Small gain at small cost.

## F-1b-12: confirmed

`KNOWN` (rule.rs:498-502) repeats `Named::name` over `Named::ALL` and the
divisions, and the Unicode half of `FromStr` (rule.rs:438-479) repeats it
again; `name` is a `const fn` (rule.rs:305), so the change is possible.
The gain is tidiness only: `names_round_trip` (rule.rs:512-531) already
fails when the lists drift, and core-export.md:137-138 records the
procedure ("A new rule is a new entry in `Rule::ALL`, its named forms in
`Named::ALL`, both tables, `name` and `from_str`"), which the change must
amend. The other tables the witness lists (`Rule::name`, `Node::TAGS`,
`style.rs`'s tables) are not touched by the change proposed.

## F-1b-13: confirmed

size.rs:30 imports the standard tables, and the twelve other places are
as listed (thirteen with size.rs; the heading's count of the others is
off by one). There is no `clippy.toml`. hash.rs:16 calls its hasher "The
hasher every table in this crate uses", which these places contradict. P6
names `disallowed_types` as the means. Notes: none of the places is shown
to be hot, so the gain is the rule a machine checks and reproducible
iteration order, not speed; hash.rs's own two aliases name the standard
types and need an `#[expect]`; `clippy.toml` holds for the whole
workspace, so `cli/`, `bench/` and `web/` need the alias or an exemption,
which makes the cost `medium`.

## F-1b-14: confirmed

style.rs:21-23, style.rs:227-230 and fmt.rs:13-14 say "the linear
derivations and those of LK and LJ"; five types implement `Drawn`
(style.rs:318, classical.rs:823, ordinary/derivation.rs:1149,
lambda/draw.rs:597, rules/draw.rs:276). `style` is a public module, so its
doc names a crate-private trait to public readers. S11 covers a stale
claim. The place for fmt.rs is line 13-14, not 10-11.

## F-1b-15: candidate

The witness holds: `Inference` is 88 bytes with `.principal` 16 and
`.premises` 24 (type-sizes.txt:42611-42619), `Rule::premises` returns 0, 1
or 2 (rule.rs:121-129), and `split_off` at build.rs:458 makes each list.
`dhat.md` gives 130 of 2 873 blocks to `Vec<InfId>` on
`derivation-chain-64`, 4.5 %; the inference's `sequent: Vec<Member>`
allocates 555 and stays. So the gain is small and unmeasured. To measure:
DHAT and callgrind on `derivation-chain-64` and `render-latex` before and
after. What the change must keep: `Size::BYTES_PER_INFERENCE` (128, public,
size.rs:76) is the estimate's bytes per inference; left as it is the bound
still holds, and lowering it moves where `limits.derivation_bytes` refuses,
which the behaviour lock sees. `TwoSided` keeps its own `Vec<InfId>`
premises (classical.rs:338), and mod.rs:885-889 builds inferences from
slices.

## Beside the findings

None.
