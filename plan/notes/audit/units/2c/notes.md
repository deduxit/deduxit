# Unit 2c notes

## horn/ (mod, proof, reach, cover, equation) read in full

- E6: merge of two sorted arc lists written 3x with the same 5-arm match ladder:
  reach.rs:686-705 (`moved`), equation.rs:304-323 (Tableau::new effect),
  reach.rs:526-547 (`successor`, marked vs outputs, same shape). mod.rs:241-257 `caps`
  does the same merge with peekable. One `merge(inputs, outputs) -> impl Iterator<(p, taken, given)>`.
- E6: mod.rs:620-635 `Reader::clause` repeats `clause_head` (853-866) loop body (Tensor/One/body/head arms)
  and `head_of` (639) calls clause_head again on a clause just read. proof.rs:134,196 call
  clause_head per firing again (twice per firing: replay and build).
- P2 (setup, not hot): mod.rs:699 `seen.get(&(inputs.clone(), outputs.clone()))` clones both
  arc lists to look up; mod.rs:712-713 `key.clone()` in entry + classes.push(key). equation.rs:328
  `seen.insert(effect.clone())` keeps a second copy of every effect (uncharged).
- DHAT search-spec-chain: horn read closure 4015 blocks 7% (Program::read find_map).
- Charging growth of a BinaryHeap written twice: reach.rs:479-493 (frontier) and
  cover.rs:503-519 (room_in_queue); reach::room for Vec (741) is used by cover and mod (Parents::room)
  but lives in reach.rs.
- mod.rs:115 expect("the engine admitted the goal") fine.
- Decide::decide ignores _options,_limits,_work (horn).
- mod.rs:88 reaches into super::focus::counts::refutation (F-2b-03): needs refutation(forest, goal,
  fragment, mode, &account, stop) only.
- reach.rs:343 `u32::MAX - key as u32` parent from key.
- cover.rs:532 `load(..., clear: bool)` bool param (internal).
- No std HashMap: horn uses crate::hash.

## inverse/ (mod, rules, eval, index) read in full

- No ratchet/DHAT journey runs Engine::Inverse (journeys: search-* are focus/horn/additive). Every
  perf claim here is a candidate; instrument: target rows forced `--engine inverse` or a new journey.
- P2 index.rs:77 `walk` allocates `vec![(start, 0usize)]` per query; `within` (135-139) calls
  ends_within per node, each a new Vec. Two subsumption queries per derived sequent (mod.rs:458-459,
  keep and run). horn/cover.rs:127 keeps `stack: RefCell<Vec<(u32,u32)>>` for the same trie walk.
- P2 mod.rs:538 `self.goal_index.get(o).cloned().unwrap_or_default()` clones a hole list per keep;
  mod.rs:539 collects 0..holes when gamma empty.
- P2 mod.rs:865 `fits` under a reading builds `most: Vec` per call, in the inner loop per premise choice.
- P2 mod.rs:468 `coherent` collects+sorts a new Vec per derived sequent; mod.rs:598 Zone::default per pop.
- P2 eval.rs:67-85 `sum` allocates a merged Vec per Tensor/Mix step instead of merging in place.
- mod.rs:98 as_set (F-2a-06 leftover, confirmed).
- mod.rs:144 `super::focus::bias::signs(forest, bias)` (F-2b-03): needs only signs -> Box<[Sign]>.
- E6 rules.rs:768-780 and 818-830: identical 12-field `Writer { .. }` literal in compile and goal.
- rules.rs:216 `copied` walks to the root per coherent() call (after lca, also O(depth)); a per-occ
  flag "under a ?" computed once would make it O(1). Candidate.
- duplicates.md eval.rs:98 (excess) vs saturated.rs:519: recorded on purpose (core-inverse.md:219
  "shares no code with inverse/"). Not a finding.
- text.md:79 index.rs:42 "making it where it is missing" (trailing -ing?).
- rules.rs:137,155 `Box<[crate::linear::occurrences::Sign]>` full path where `Sign` could be imported.
- No std HashMap in inverse (crate::hash).

## lltp.rs, mist.rs, families.rs read in full

- PROBE (scratch crate /tmp/.../scratchpad/probe2c, default features, cores 13-15):
  three_partition(&[0,1,2],1,3), three_partition(&[1,1,1],0,3), three_partition_mll(&[0,1,2],1,3),
  three_partition_mll(&[1,1,1],0,3), partition(&[0,2,2]) all PANIC at families.rs:1111
  "a generated sequent parses: ...". Docs' # Panics name only odd total / power of two / n<3 / shared 0.
  mist: "not a .spec problem: the counter `c` is not declared" and "expected `>=` at `=`" -- no line.
  lltp: lemma and theorem accepted (Ok "⊢ ~a, ~b, a ⊗ b") though # Errors (lltp.rs:108-111) says a role
  other than axiom, hypothesis or conjecture is refused (description at 90-92 says lemma/theorem ok).
- P2 lltp.rs:300 `let chars: Vec<char> = before.chars().collect();` per line: DHAT read-lltp 6007 of 10465
  blocks (57%) at lltp::read (lltp.rs:120, clauses inlined); read-tptp 6004 of 18418 (32%) at
  ordinary::parse::read_tptp (parse.rs:942). LINES=2000 -> ~3 allocs/line (collect from Chars grows).
  Lookahead is ASCII only: byte indices into `before` give the same answers.
- E2/P2 mist.rs:419-430 `merged` allocates a second Vec; `dedup_by` in place. DHAT read-spec 2001 of 12440
  blocks (16%) at mist::merged (mist.rs:421); search-spec-chain too.
- mist.rs:630-661 another sorted merge (guards vs changes) like horn's.
- mist Arena (mist.rs:308-405) is a private builder over pub(crate) Sequent::from_parts (consumer test pt 1).
- families build every sequent as text then parse (families.rs:1104); DHAT: wide closures 2x3096 blocks
  = 40% of read-text journey's blocks; additive_tree 52090+32758 of 167253 (51%) of search-additive-14;
  check-wide-m1-2048 8%. Generation pollutes the journeys' DHAT tables. format! per piece + join.
- Examples use `deduxit::linear::search::{Options, prove}` (lltp.rs:27, mist.rs:46, families.rs:45,63)
  where core.md says users write `deduxit::linear::prove` (re-exported linear/mod.rs:43).
- C-COMMON-TRAITS: lltp::Problem, mist::Problem, families::Instance derive Clone, Debug only; Sequent is Eq.
- inverse/eval.rs insert/remove/sum/excess/within/union = proofs::multiset::Multiset ops (crate-private,
  used by derivation/cut/interactive, not by check/). saturated.rs keeps its own on purpose.
