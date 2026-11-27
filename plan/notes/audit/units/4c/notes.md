# Unit 4c notes

## Read so far: connection/mod.rs, matrix.rs, terms.rs, sizes.rs
- mod.rs:120 `connection::decide(sequent, logic, options: &search::Options, ...)` reads only path_length + restricted_backtracking of linear::search::Options (F-arch-02 from the engine side). mod.rs:171 builds prove::Config from them.
- mod.rs:52 `use crate::linear::search::{self, Reason}` — Reason from linear suite (handover).
- matrix.rs:46 `Budget` {stop, limit, done} + poll; compare prefix.rs:41 (duplicates.md 175 tokens).
- matrix.rs:259 `type Memo<'a> = (&mut [u32], &mut HashMap, &mut HashMap, &mut [u32])` 4-tuple of &mut: E3 (tuple with more than a few fields → named struct).
- matrix.rs:1048 `union` of sorted lists, duplicated in prefix.rs:488 (duplicates.md).
- terms.rs:266 unify / :318 equal: same pair walk (duplicates.md 184 tokens, 122 tokens).
- terms.rs:24 Seen<K> generic; good.
- sizes.rs TermBudget shared by both read-backs. fine.
- HashMap: crate::hash used everywhere so far (no std).

## prove.rs (classical) vs intuitionistic/prove.rs, prefix.rs, intuitionistic/mod.rs
- F-arch-24 CONFIRMED by reading: types PathNode/Item/Goal/Lemma/Cursor/Alt/Choice/Way identical (prove.rs:83-224 vs ip:61-203); Marks differs by `eqs`; Clock differs by `work`.
  Identical fns: cons, elements, try_start, try_clause, may_be_lemma, on_path, try_literal, next_way, inside, run's backtrack loop, poll (but eqs.len), held_bytes (+prefixes,eqs), marks/restore (+eqs).
  Differences (the hooks):
  1. run at g==NONE: classical returns Ok(true); intuit: prefixes_unify + accept(read-back), else rejected++ and backtrack.
  2. Solved: classical settles (restricted || ground && !bound_older); intuit only restricted (`let _ = mark;` ip:434).
  3. tick: intuit adds prefixes.take_read()/PREFIX_CHARS to clock.work.
  4. enter_literal `same`: intuit also compares of_literal prefixes (allocating).
  5. in_clause_of / region_for / extension_clauses: classical has is_ground positional shortcut + break on ground; intuit counts renames[clause].
  6. apply: intuit `|| !self.connect(...)` after unify both ways; fresh includes renames[clause].
  7. search(): classical End{Proved{used,terms},Exhausted,Unknown}; intuit End{Proved(Box<Derivation>),Unknown}, accept callback, rejected counters; no Exhausted.
- prove.rs:532 / :563 / ip:582 / :613 `self.matrix.kids(clause).to_vec()` allocation per start/decomposition: unneeded since matrix is `&'m Matrix` (copy the reference, slice lives 'm). P2 candidate.
- prove.rs:815 / ip:862 region_for allocates `copies` Vec per call (per clause per target). P2 candidate.
- ip:646 `same` closure calls prefixes.of_literal per path node & lemma: two Vec allocs (chain + read, prefix.rs:200,405) per comparison. P2 candidate.
- ip:915 `let mut eqs = self.eqs.clone();` deep clone per candidate proof.
- prefix.rs:44 Budget == matrix.rs:46 Budget word for word; prefix.rs:492 union == matrix.rs:1048 union word for word. free_vars (prefix.rs:218) mirrors free_slots (matrix.rs:920) with own Tally vs closure account.
- prefix.rs:447 `let _ = matrix;` unused param of of_occurrence (called ip:967,969).
- prefix.rs:634 unify returns HashMap built and discarded by caller (ip:979 `Ok(_)`).
- intuitionistic/mod.rs:101 RefCell around stop so accept and search share it.
- intuitionistic/mod.rs:132-141 and connection/mod.rs:196-206 both build Statistics from Counts field by field: a `From<Counts>` would do (small).
- Both decide fns: identical "unknown" closure + match on Matrix::of errors (mod.rs:158-170, imod.rs:76-92).

## read_back.rs (classical) and intuitionistic/read_back.rs
- rb.rs:514 push (stop+memory+u32 index refusal) ≈ irb.rs:1233 Emit::push ≈ herbrand Build? (check). Three inference pushers.
- rb.rs:397 `self.skolems.get(&(k, args.clone()))` clones a Vec to look up; key (u32, Vec<FoTermId>).
- rb.rs:176-187 / irb.rs:173-185 the "first eigen number past the arena's" scan duplicated (and herbrand?).
- irb.rs:1079 `instance_term` is a one-line alias of `term`.
- irb.rs:569 `vec![value].into_boxed_slice()` key per child lookup (alloc per lookup).
- irb.rs:734 `prove` recursive, bounded by limits.recursion_depth/2 (rules file records). Not a finding.
- intuitionistic/mod.rs:101 RefCell<stop> shared by accept + search: consequence of the accept-callback shape.
- rb.rs close(): weakening loop clones l,r per weakening (quadratic in branch width) — inherent to explicit weakenings.

## herbrand.rs
- Build (h:1065) duplicates classical connection read-back's ReadBack: Frame (h:1025 == rb:92), Step (h:1013 == rb:1086 minus doc), position (h:998 == rb:242), replaced (h:1005 == rb:939), close (h:1273-1352 == rb:628-709 word for word but `return self.push(..).map(Some)` vs `let id = ...; Ok(Some(id))`), closes_a_premise (h:1567 == rb:904), push (h:1176 ≈ rb:514 ≈ irb:1233), derive (h:1102 ≈ rb:477), expand (h:1226 ≈ rb:587), step's order (h:1360 ≈ rb:713), unary/binary (h:1587/1625 ≈ rb:947/993 minus occ), instance (h:1659 ≈ rb:1039). rules: core-ordinary.md:2471 "follows the connection read-back's construction".
- base eigen number: h:1858-1868 == rb:178-187 == irb:176-185 (3 copies).
- h:1824 skolem_terms and h:1840 nesting walk terms as trees with no Seen; `!found.contains(&u)` linear. connection's rb:423 `available` uses Seen. herbrand available() (h:1817) calls skolem_terms per witness per item per step. Perf candidate on shared terms (resolution read-back hands herbrand unifier terms that share parts).
- h:350 Universe::of walk == h:763 witnesses_of walk (dup 185 tokens) and ~ instance.rs.
- h:195 herbrand_default only in docs/tests (surface.md: named nowhere outside core/src). Module fn `herbrand::herbrand` stutters; other modules use decide/decide_within.
- h:1490 given(): returns cloned Option<Vec> from cache each call.
- h:611 cut(): `instances.clone()` per try — up to MOST_CUT=256 SAT calls each with clone: fine (bounded).
- h:1048 Frame::bytes estimate.

## resolution mod.rs, refutation.rs, saturate.rs, dpll.rs, outside.rs
- mod.rs:842 public `clauses_of(sequent, form, search: &crate::linear::search::Options, ...)` - linear search options in ordinary resolution's public signature (F-arch-02 pattern; check what normalize reads).
- refutation.rs:248-251 checker clones two clauses per resolvent via clause_unchecked (refutation.rs:286 `.cloned()`), where `self.clause(k)` already gives &Clause. P3 in the checker; behaviour kept.
- refutation.rs:228-231, saturate.rs:103-108,115-120, dpll.rs:139-141: `x.is_some_and(..)` then `x.unwrap_or(u64::MAX)` for the same Option: let-chain `if let Some(w) = .. && ..` (E1).
- outside.rs:19 `use crate::linear::search::memory::Account` (F-arch-06): needs holding/charge/over/limit.
- mod.rs:589-620 / 661-691: Method and Equality each hand-write ALL, NAMES, name(), FromStr: crate pattern (check elsewhere), C? E6.
- saturate.rs:85 clones the given clause per pop.
- dpll: every scan over all clauses per step (teaching scale; rules say so).
- mod.rs:77 Literal(u32) with `new`->Option and pub(crate) `of`; public `Clause::new` but no Clause from slice. fine.
- mod.rs:527/559 ClauseText, LiteralText public, named nowhere outside core (surface.md) - but returned by public Clauses::text/literal; fine (display adaptors). Not a finding.

## first_order.rs and the measurement
- MEASURED S1: `forall x1..xn. ~p(x1..xn, x2..xn), forall y2..yn. p(f(y2,y2)..f(yn,yn), a, y2..yn) |-` (generator in scratchpad gen.py): `deduxit resolve --timeout 1s` (target/debug/deduxit, debug build): n=18 1.12 s, n=20 1.68 s, n=22 4.64 s (37 MB of text), n=24 21.99 s (--format json, output discarded); "the read-back was stopped". Unbounded writes: first_order.rs:2004 read_back -> mapped_atom (:702) -> predicate_atom key, no size check (applied_atom :806-816 has one); bindings_text (:216) writes every binding in full; the read-back's instance loop (:1998-2027) asks no stop.
- first_order.rs:1197-1206 Answer::Refuted { derivation: Option<Box<Derivation>>, not_read_back: Option<String> }; :1342-1346 error.to_string(); consumers: serialize/ordinary.rs:1958, cli/src/resolve.rs:204-220.
- first_order.rs:1136 Refutation::check(stop) with no &Limits vs refutation.rs:191 check(limits, stop).
- first_order.rs:1692-1702 pushes a Resolution step for every opposite-sign pair whatever the predicate; made() clones both parents (:298-300) and interns shifted atoms of b (:318-330) before pairs() finds the predicates differ (:336). Failed steps not counted (:1727-1730).
- first_order.rs:918 subsumes clones `values` per candidate literal.
- Seams of first_order.rs (2563): Clauses+inference (:173-945), text (:1030-1104), Refutation (:1106-1188), resolve/search (:1190-1875), read-back (:1877-2180), tests (:2182-2563).
- herbrand.rs:1870 `made.sort_by_key(|&s| self.nesting(s))` - nesting walks the term each comparison: sort_by_cached_key (E2).
- Pins of connection search today: tests pin verdicts only (connection/tests.rs 10 tests, intuitionistic/tests.rs 3, agrees_with_ljt holds rejected-but-unified to zero); one bound on steps (tests.rs:310). bench harness writes nodes=extensions, work=steps (bench/src/ordinary.rs:180-190); bench/defaults/connection-*.csv are 2 s timed runs. RESULTS.md has no connection rows.
- walk "terms of the atoms of formulas" repeated: herbrand.rs:350, :759, first_order.rs:1396, instance.rs:132,261,389.
