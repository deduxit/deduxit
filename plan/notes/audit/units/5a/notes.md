# Unit 5a notes

## mod.rs (1404)
- mod.rs:212-268 four id newtypes TermId/VarId/TypeId/TypeVarId: only `index()`, no `new`/`get`; Type::Atom{atom: u32} bare u32 (no AtomId), C-NEWTYPE.
- mod.rs:298-432 Calculus: ALL, NAMES, name() match, serde rename_all, FromStr: names in 3 hand lists + serde; same for Notation 977-1018. Check.rs:274 a 4th table (sentence names).
- mod.rs:399-414 is_ordinary/is_linear/is_unrestricted pub(crate); is_typed/is_polymorphic pub. A user can't ask whether a calculus counts uses without matching 9 non_exhaustive variants. Calculus is a 3x3 product (typing x uses) flattened.
- mod.rs:809 Term: no public constructor (Builder private, from_parts pub(crate)); F-arch-05.
- mod.rs:874-881 Term::node(id) exists, no Term::ty(id)/type accessor; users index types()[id.index()].
- mod.rs:68-72 doc example uses expect/panic for typing lookup.
- mod.rs:1400 `stopped()` helper again (handover: four modules have one).
- mod.rs:1304 operands, 1352 renumber: same match twice (visitor + mut visitor).
- mod.rs:605 "The enum is exhaustive, as the calculus is fixed" - yet Node grew TypeLambda/TypeApply with System F.

## check.rs (1377)
- Fault pub(crate); users get Error::IllTyped{reason:String,node,span}; no programmatic kind.
- check.rs:762 Meter (work/stop/memory) - check for repeats in reduce/infer/inhabit.
- check.rs:1184 `written`; read.rs:107 `variables_of` - near walks per node.

## read.rs (354) / poly.rs (267)
- read.rs:52 type_operands duplicates poly.rs:94 operands (depth 0); read.rs:74 retyped duplicates poly.rs:55-83 rebuild Leave arm.
- read.rs:22 Parts pub(crate), from_parts pub(crate): only JSON makes a Term from parts.

## print.rs (1346)
- No public way to print a lone type: write_type pub(crate) (print.rs:348). Typing::types() (check.rs:44) holds types beyond the term's, and Term::principal_type etc.; a user with a TypeId from Typing::ty can't print it (X1 "a formula printed").
- print.rs:775 `pieces` allocates a Vec<Piece> per node at every place written (emit 1139) and again in lengths() 1018; length() 1004 writes a var name into a fresh String per piece. P2 candidate (no journey).
- print.rs:348 write_type 8 args, too_many_arguments expect; check.rs:210 Names holds the same arenas.
- print.rs:466 free_in: another hand type-operand match (5th: read.rs:52, poly.rs:94, print.rs:480, print.rs:689-716).
- print.rs:1329 Display: never fails, writes <refusal> recorded.
- print.rs:209/257 LATEX/TYPST tables duplicate per duplicates.md: data, not a finding.

## parse.rs (1544)
- Two grammars: typed Parser (Frame, operand/deliver 432-965, judgement 1232) and untyped (Open enum, untyped 1328-1437) each with own context reader (1234-1245 vs 1331-1344), own λ reader (577-597 vs 1352-1373), own application run (711-737 vs 1425-1437).
- Subscript-digit loop twice: parse.rs:330-337 and 1036-1043.
- parse returns an unchecked term (uses not checked) - documented.

## reduce.rs (946)
- reduce.rs:292 Budget{work,most,memory,next,stop} + tick, POLL 4096 vs check.rs:762 Meter{work,most,next,stop,held,memory} + tick, POLL 1024; parse.rs:235 count; inhabit? Three meters in one module.
- reduce.rs:766 fresh_name == print.rs:536 fresh (stem + least index): duplicate.
- reduce.rs:557 binders -> Vec per node (allocates in subterm Enter 493 for every node copied every step, needs_copy 928, rename 747). read.rs:107 variables_of = binders + Var; reduce.rs:585 rebind same match; check.rs:1184 written's variable part. Four per-node variable walks.
- reduce.rs:114-185 Strategy ALL/NAMES/name/FromStr: same 4-part pattern as Calculus, Notation.
- reduce.rs:850/888 each step copies whole term and re-checks it: recorded design (core-lambda.md "Every reduct is checked").
- End::Refused(Refusal) carries a refusal inside an Ok outcome; reduce returns Err only for the first check / ClaimFails.

## infer.rs (451)
- infer.rs:208 Solution::write: third type printer (own, for unification shapes); fine.
- infer.rs:256 subterm clones the whole term for an error message (error path only).
- infer.rs:155 letter(n) a..z,a1 vs mod.rs:1178 LETTERS f,g,h/x..w: two naming schemes (fine, different purpose).

## simple.rs (545)
- simple.rs:91-95 read(derivation, _limits, stop): limits unused ("bounds nothing here").
- simple.rs:485 type_of: ordinary formula -> Type; inhabit.rs uses it.

## linear.rs (641)
- linear.rs:107-111 read(derivation, _limits, stop) limits unused, like simple.rs.
- linear.rs:154-227 ty(): occurrence -> Type walk; simple.rs:485 type_of formula -> Type: two converters (different sources, fine).
- linear.rs:161,179 Vec allocs per formula (operands, missing); simple.rs:499 same. minor.
- many expect()s: documented "never panics" only for LJ read-back (core-lambda.md); linear read-back relies on derivation being checked. Not mine to judge correctness.

## draw.rs (675)
- draw.rs:27 LABELS and draw.rs:52 NAMES: same 21x2 table twice, differing only in `_1` vs `₁` (duplicates.md rows 85,115).
- draw.rs:606-613 markup: Labels::Table(_) and Labels::Subscript treated as Upright: a user's own labels (Table keyed by linear Named) ignored; Subscript same as Upright. Handover question: lambda labels by its own const table, no user table.
- draw.rs:610 vs rules/draw.rs:289 duplicate (136 tokens) per duplicates.md.
- draw.rs:173 premises(node) Vec per node, called twice per node visit (457 and 469).
- draw.rs:255 free_variables: another free-variable pass (check Pass, reduce rename, draw free_variables).
- draw.rs:382 Drawing::new checks the term itself (term.check) - fine.

## inhabit.rs (1486)
- inhabit.rs:222/303 inhabit/enumerate take ordinary::Sequent, answer Unknown(linear::search::Reason), Empty{countermodel: Option<ordinary::Countermodel>}; inhabit.rs:237 & 815 pass crate::linear::Options::default() into ordinary::decide_within (F-arch-02 from the lambda side).
- inhabit.rs:714-724 extend: copy of Builder::variable naming (mod.rs:1224-1233).
- inhabit.rs:777 Search::tick: 3rd/4th meter (work, memory via held(), stop POLL 1024).
- inhabit.rs:813 decided(): Sequent::new(self.formulas.clone(), ...) clones the arena per shape decided. candidate.
- inhabit.rs:368 `terms.into_iter().filter(|t| !best.contains(t))`: O(max_terms^2) whole-Term comparisons. candidate.
- inhabit.rs:1223 term(): self.base.clone() (Builder with hash tables) per listed term. candidate.
- inhabit.rs:950 listed recomputed by sum over results per task.

## Cross-checks
- MEASURED: System F reduce keeps every type/type var of earlier steps (reduce.rs:627-628 Types::of(&term.types,0), type_variables clone + push per ΛX copied). Scratch crate /tmp/.../scratchpad/sysf, n nested (ΛX.λx:X.x)[a](...) over y:a:
  n=10: start types 25 tv 10 -> NF nodes 1 types 115 tv 100; n=20: 425/400; n=40: 1645/1600. Quadratic.
- Progress units: check Meter polls counting() once per 1024 units -> Progress{work:1, done:#polls}; reduce Budget Progress::new(Reduce,1,done=nodes of this copy); inhabit Progress::new(Search,POLL,work). limits.rs:408-412 says work = units since previous poll, done = units since call began.
- Calculus/Strategy: CLI kebab (system-f, untyped-linear; clap ValueEnum), library/JSON/web snake; mod.rs:345 doc says "as the JSON form and the command write them". No test ties ALL/NAMES/name()/serde.
- wasm.md:119,126: Node Deserialize 73806 B x2, Type 32492 B x2 (internally tagged serde(tag="kind")): ~213 KB of lambda's 671 KB.
- Drawn impl root/premises CSR duplicated lambda/draw.rs:619-630 and rules/draw.rs:298-309.
- What lambda needs from linear: Atoms (name writer), InfId, Labels, Drawn (exports), search::Reason (inhabit Unknown), linear::Options (to call ordinary::decide_within).
