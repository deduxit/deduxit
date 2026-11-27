# Unit 4c: verdicts

14 confirmed, 3 candidate.

## F-4c-01: confirmed

Opened resolution/first_order.rs:702-721 (`mapped_atom`: no size test before `predicate_atom`, which writes the key's text, ordinary/mod.rs:1215-1234), :788-819 (`applied_atom` has the `most_terms` test), :1997-2027 (the read-back's instance loop: no stop asked, `mapped_atom` per literal) and :216-239 (`bindings_text` writes every binding whole). I built the sequent family and ran the tree's debug binary in a capped scope on cores 13-15: `resolve --timeout 1s` took 1.10 s at n = 16 (69 MB) and 1.47 s at n = 20 (135 MB), the output's last binding list growing as 2^n, so the run does pass the time limit and the growth is the finder's; I did not rerun n = 22 or 24. The resolvent is the empty clause, so `applied_atom`'s bound never sees these terms. core-ordinary.md:2681-2689 records the bound on the DAG for `applied_atom` only; nothing records the read-back's instances or the bindings' text as left out on purpose. S1 of the rubric says what the finding says.

Correction: Kind: not a pure refactor. A refusal in `mapped_atom` and a stop in the loop keep every result within the bound, but abbreviating or refusing `bindings_text` changes what `resolve` prints and the level-29 wire form (each inference's unifier as text), which is an output change for the author to decide; the manual's resolve chapter and wire.schema.json go with it. `mapped_atom` is also `shifted`'s and `renumbered`'s helper (:724-734), so the size test there runs on every search step: put it in the read-back's call or measure it.

## F-4c-02: confirmed

Diffed connection/prove.rs against intuitionistic/prove.rs: 449 differing lines of 937 and 1078, and outside comments they are the finder's seven hook points (the `g == NONE` acceptance with `prefixes_unify` and `accept`, `Solved` settling only under restricted backtracking with `let _ = mark;`, `tick` adding prefix characters, `same` comparing prefixes, the `is_ground` shortcuts against `renames[clause]`, `connect` in the two connection tests, `End::Exhausted` only classical) plus the fields `prefixes`, `renames`, `eqs`, `rejected`, `Marks.eqs`, `Clock.work`. duplicates.md has the 13 rows. The `RefCell` around the stop is at intuitionistic/mod.rs:101. The tests count 10 and 3. `grep` of bench/ceilings.csv and bench/RESULTS.md for the connection prover finds nothing, so no instrument pins the counters; the harness has the engine (bench/src/ordinary.rs:182, :258). core-ordinary.md records the intuitionistic search as a copy changed, with no reason to keep two; the arch verifier confirmed F-arch-24 on the same ground.

Correction: Kind stays refactor only under the pin the finding names: the oracle rows must be recorded at the base commit before the merge, and a moved `steps`, `extensions`, `reductions`, `lemmata` or `path_length` on a decided row makes it a search change. The intuitionistic clock counts `1 + prefix characters / 64` per tick where the classical one counts steps, so the poll points and `limits.work` differ between the two and the trait must keep each (`extra_work`). Cost medium is honest for the search; with the harness columns and the oracle it reaches bench/ too.

## F-4c-03: confirmed

Opened connection/read_back.rs:508-545 and :626-709 beside herbrand.rs:1171-1222 and :1271-1352: `close` is the same code line for line up to how the two early returns are written (the `⊤R` and `⊥L` cases, the axiom, then the weakenings in order), and `expand` pushes the same `Frame::Emit` and `Frame::Expand`. core-ordinary.md:2471-2484 says `Build` "follows the connection read-back's construction ... with its own oracle", which names the seam and gives no reason for the copy. The `push` charges do differ (read_back.rs:525-527: one `Inference` plus plain lengths, then `account`; herbrand.rs:1194-1205: twice the `Inference` plus rounded blocks, and the stop asked in `push`), as the finding says.

Correction: Cost: between medium and large; it spans connection/ and herbrand.rs and adds a module, and the brief's `medium` is one module. The net is not only the checked derivations: one `push` gives one memory charge, so the point where `Refusal::Memory` fires moves in one of the two read-backs, and `expand`'s stop and work test (herbrand.rs:1235-1245, recorded at core-ordinary.md:2505-2509) and the connection side's `account` must both survive; the tests that pin a memory or stop refusal (`the_read_back_asks_the_stop_and_charges_its_arena`, core-ordinary.md:853) are the net for that. The intuitionistic read-back is a third construction and stays out.

## F-4c-04: confirmed

Opened connection/mod.rs:93-133 and :171-174: `decide` takes `&search::Options` and reads `path_length` and `restricted_backtracking` only, as its doc says; intuitionistic/mod.rs:93-96 does the same. resolution/mod.rs:842-862: `clauses_of` takes `&crate::linear::search::Options` and only passes it to `normal::normalize`. core-ordinary.md:909-912 records that the bound is `search::Options::path_length` and that `Reason::PathLength` names `search.path_length`, a description of the code with no reason against options of the suite's own. This is F-arch-02 from the engines' side, which the arch verifier confirmed; D4 says what the finding says.

Correction: Claim: `resolution::clauses_of` at resolution/mod.rs:842 is the propositional resolution's (it refuses `Form::Clausal`, :864-872), not first-order resolution's. Not an independent finding: it is one part of F-arch-02 and should be decided with it. Cost: medium for the two signatures; large once the settings keys `search.path_length` and `search.restricted_backtracking`, the reason's text, the protocol's `reads` lists and the manual follow, as the arch verdict on F-arch-02 says.

## F-4c-05: confirmed

Opened first_order.rs:1193-1213 (the pair `derivation: Option<Box<Derivation>>`, `not_read_back: Option<String>`), :1342-1346 (`Err(error) => (None, Some(error.to_string()))`), cli/src/resolve.rs:200-221 (the match on the pair with the `(None, None)` arm that no value built by `resolve` reaches) and the uses in serialize/ordinary.rs:1958-2026, wire.rs:285, docs/src/resolution.md:274 and wire.schema.json:4477. `Error` derives `Clone, Debug, PartialEq, Eq` (errors/mod.rs:57), so `Result<Box<Derivation>, Error>` keeps `Answer`'s derives. core-ordinary.md:2679-2680 records that a failed read-back leaves the refutation as evidence, "said so (`not_read_back`)": it records that the reason is given, not that it is text. The gain is real: a stop, a memory refusal, the refusal of paramodulation and an `Error::ReadBack` defect are one string today.

Correction: Criterion: C-CUSTOM-TYPE speaks of arguments; this is a result's fields, so the nearer criterion is the error criteria of the C list (a meaningful error type) or X; the substance holds. Cost: medium to large, since the wire form changes (level 29's `not_read_back` string becomes an error object: a level bump, wire.schema.json, docs/src/resolution.md, the guide if it names the field, and the web page if it reads it).

## F-4c-06: candidate

Opened connection/prove.rs:229 (`matrix: &'m Matrix`), :532 and :563 (`self.matrix.kids(clause).to_vec()` only to pass `&nodes` to `self.elements`, which a copy of the `&'m` reference would let borrow), :813-824 (`region_for` collects the path's copies into a fresh `Vec` and only iterates it, after the ground shortcut at :810), matrix.rs:905 (`kids` returns a slice), and intuitionistic/prefix.rs:200 and :405 (`chain` and `read` each build a `Vec`; `of_literal` is called in `same` per path element, visible in the diff of the two prove.rs files). P2 says this. Reading supports it; no journey or DHAT run covers the prover, so the gain is unmeasured.

Correction: Measure as the finding says: DHAT allocation counts and pinned time on connection rows of the ILTP first-order set, one thread, with `steps`, `extensions`, `reductions` and `lemmata` equal on every decided row. The prefix comparison is the delicate part: `read` also interns contexts and counts `self.read`, which feeds the intuitionistic clock (`take_read`), so a position-by-position comparison that stops early would read fewer characters and move the poll points and the work count; keep `read`'s count or call it a search change.

## F-4c-07: candidate

Opened first_order.rs:1690-1703 (a `Step::Resolution` queued for every pair of opposite sign, no test of the predicates), :298-338 (`made` clones both parents, shifts every literal of the second through `shifted`, which interns, and only then `pairs` answers "the two literals are of different predicates"), :574-579 (paramodulation's head test before queuing) and :1718-1730 (a failed step is skipped before `statistics.steps` is counted). No `sort`, `dedup` or order comparison exists in the file, so arena ids from the skipped interning decide nothing. The claim holds on reading; how much time it is of a search has not been measured.

Correction: Two things to keep: `made` is also the checker's and the step-by-step session's one inference function, and it tests the signs (:311-315) before the predicates, so a predicate test moved before the clones must keep that order of reasons or a session's message changes; and each queued step asks the stop once (:1719), so fewer queued steps ask it less often (the `Progress` they give is the unchanged step count). Measure with DHAT and callgrind on `resolve` of a first-order problem with many predicates, paramodulation off.

## F-4c-08: confirmed

Diffed the 30 lines from `struct Budget` in connection/matrix.rs and intuitionistic/prefix.rs: identical, `poll` included. Diffed `fn union` in both: identical. The eigenvariable base is the same eight lines at connection/read_back.rs:178-187, intuitionistic/read_back.rs:176-185, herbrand.rs:1858-1868 and derivation.rs:1907-1915. The statistics copy is at connection/mod.rs:199-205 and intuitionistic/mod.rs:135-141, the `unknown` closure with the match on the build's refusals at mod.rs:158-170 and intuitionistic/mod.rs:76-92. No rules entry gives a reason for the copies (core-ordinary.md:783 names `matrix::Budget` only). E6 says this.

Correction: The intuitionistic `unknown` path returns a pair with a count (`Ok((unknown(..), 0))`) and builds the prefixes inside the same `and_then`, so the shared helper maps a refusal to a `Reason`, not a whole `Outcome`.

## F-4c-09: candidate

Opened herbrand.rs:1817-1852: `skolem_terms` and `nesting` push every argument with no visited set (`found.contains` only keeps the result free of repeats), so a term that shares its parts is walked as a tree; they are called per witness tried at :1418, :1439, :1461 and :1468, and `made.sort_by_key(|&s| self.nesting(s))` at :1870 calls the walk at every comparison. connection/read_back.rs:423-460 does the same two jobs with `Seen`, and core-ordinary.md:854-868 records why. Whether shared terms reach `Build` at a size that matters is not shown: `Tables::read` and `limits.terms` bound a term's unfolded size on most paths (core-ordinary.md:353-357), and the path that does not is F-4c-01's `mapped_atom`.

Correction: Kind and criterion: with F-4c-01 unfixed these walks are exponential on that family and ask no stop inside, which is S1, not E2 or speed alone; once F-4c-01 bounds the instances' terms, what is left is a constant factor. Measure callgrind on `resolve` and `herbrand` of the F-4c-01 family at small n, before and after F-4c-01. `nesting` with a visited set must keep the deepest count per term (a memo of the depth below each term, not a plain seen set, since the count depends on the path above).

## F-4c-10: confirmed

Opened first_order.rs:1127-1187: `check(&self, stop)` with no limits, the arena cloned at :1142, each step made again by `made`; refutation.rs:191 has `check(&self, limits, stop)` and counts `limits.work` at :228. core.md:482-486 gives the rule that every long call takes `&Limits` and `stop`. The caller at :1332 has the limits at hand. core-ordinary.md:457 mentions the calls of `check` with `&mut stop` and gives no reason for leaving the limits out.

Correction: The witness overstates what is unbounded: the check carries the term bound already (`most_terms: self.clauses.most_terms` at :1145, the set's own), asks the stop once per step, and its work is linear in the inferences the refutation holds. What is missing is `limits.work`, the memory account for the cloned arena and its growth, and a term bound the caller of the check sets instead of the one stored in the refutation. The gain is one signature for both checkers and those bounds; modest, at the price of an API break with callers in the CLI, the protocol and the serialize module to follow.

## F-4c-11: confirmed

Line counts are right (first_order.rs 2 563, herbrand.rs 2 211). Opened the seams the finding names where other findings took me: `Clauses` and `made` (:173-945), `Refutation::check` (:1127-1187), `Answer` and `resolve` (:1190-1360), the search loop (:1660-1740), the read-back (:1940-2040), and in herbrand.rs the universe walk (:345), `witnesses_of` (:757) and `Build` (:1147-1894). E5 asks for the seam and the finding names them. No rules entry asks for one file.

Correction: Cost is honest only for the move itself; the herbrand half depends on F-4c-03 (the builder's shared module), so the two are one piece of work or this one waits. The checker needs `Clauses::made` from the inference file, so `check.rs` is a file of its own but not a trust boundary on its own: the checker's trusted code is `made`, `paramodulant` and `concluded` too.

## F-4c-12: confirmed

surface.md:673-675 lists `ordinary::connection::Outcome`, `Statistics` and `decide` without an example; connection/mod.rs has no code fence at all (`grep` for the fence finds resolution/mod.rs:42-51 and none in connection/mod.rs), and herbrand.rs:29-39 has the doctest with the `cfg_attr(feature = "parse", ...)` pattern. The manual and the guide do not name `connection::decide` either. C-EXAMPLE says every public module and function has one within reason.

Correction: The example must build a `linear::search::Options` today (F-4c-04); if that finding lands first, the example is written against the new options, so order the two.

## F-4c-13: confirmed

Opened refutation.rs:228-232, saturate.rs:103-109 and :115-121, dpll.rs:138-141: each tests `is_some_and` and then reads the same option with `unwrap_or(u64::MAX)`, a default no run reaches; first_order.rs:1722-1726 writes the `if let Some(limit) = ... && ...` form. E1 names `let` chains where they flatten; this is the same form put to a near use, and the rust-version has it. No lint settles it.

Correction: Tidiness at small cost. In dpll.rs `past` is built once before the loop and `full` is a closure used at several places; the change there is to build the verdict where it is returned, which touches each such place.

## F-4c-14: confirmed

Opened refutation.rs:123-128 (`clause` returns `Option<&Clause>`), :248-251 (`&self.clause_unchecked(..)` twice, borrowed at once) and :286-290 (`.cloned()` then `unwrap_or_else(unreachable!)`). `a` and `b` are only read (`contains`, `resolvent(a, b, r.atom)`), so returning `&Clause` compiles and changes nothing observable. P3 says a clone of heap data on a path is justified or removed.

Correction: Whether the checker is on a measured journey is not shown (no DHAT row cited), so the gain is two allocations per resolvent by reading, small either way; the change is sound without a measurement because it removes work and nothing else. `grep` for other callers of `clause_unchecked` before changing its return type.

## F-4c-15: confirmed

Opened intuitionistic/prefix.rs:440-450 (`let _ = matrix;`, the parameter unused; both callers at intuitionistic/prove.rs:967 and :969 pass it), intuitionistic/prove.rs:427-438 (`mark` bound in the pattern, then `let _ = mark;`), intuitionistic/read_back.rs:1079 (`instance_term` forwards to `term`; two callers at :1001 and :1059) and matrix.rs:259 with its use at :666 (a four-tuple of `&mut` destructured in a parameter). E3 names a tuple of more than a few fields becoming a struct with names; the other three are leftovers no lint catches (the `let _ =` silences them).

Correction: Tidiness at small cost. If F-4c-02 merges the two searches, the `mark` leftover goes with it, so do not do that part twice.

## F-4c-16: confirmed

Opened herbrand.rs:345-369 and :759-791: the same walk over nodes and then terms, each seen once, with the same `match formulas.node(n)` arms; first_order.rs:1396-1424 writes it with `node.operands()`; duplicates.md has the rows for herbrand.rs:350/:763 and herbrand.rs:356/:769 against instance.rs:132, :261, :389. `witnesses_of` sorts by `(depth, id)` at :793, so its result does not depend on the walk's order. E6 says this.

Correction: instance.rs belongs to another unit; the shared walk's home is `Formulas` (ordinary/mod.rs), so the cost is small per site but the change crosses units 4a and 4c. Herbrand's universe at :345-369 collects constants and function symbols in the order met: check that nothing reads that order (the universe's levels, hence the instances' order and the SAT solver's atom numbers) before changing the walk's order there, or keep the order the walk has today.

## F-4c-17: confirmed

Opened connection/mod.rs:29-37: the quote is exact, one sentence from "Where the matrix has no variable at all" to "Lemmas 4 and 5).", with a parenthesis inside a parenthesis and an odd line break after "clause is the clause, the". T1 asks for short sentences. No rules entry fixes this wording.

Correction: The proposed text drops three things the present one says and a user needs: that "not valid" is classical logic's answer only (:6, :100), the lemmas that the completeness with positive start clauses and regularity rests on (the search uses both, so Theorem 4 alone does not cover it), and that a sequent with `=` or a run with restricted backtracking never answers it (:38-39, :105-107). Keep those in the rewrite.

## Beside the findings

- core/src/ordinary/herbrand.rs:1824-1852 and :1870: `skolem_terms` and `nesting` walk a term as a tree with no visited set and ask no stop; on the F-4c-01 family the read-back's instances reach `Build` through `mapped_atom` (resolution/first_order.rs:702, no `limits.terms` test), so these walks are exponential in the input and outside the time limit. Not a wrong verdict, a bound that does not hold (S1); it is F-4c-01 and F-4c-09 taken together, and fixing F-4c-01 at `mapped_atom` closes it.
