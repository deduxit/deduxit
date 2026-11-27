# Unit 5a: verdicts

12 confirmed, 2 candidate.

## F-5a-01: confirmed

Opened core/src/lambda/reduce.rs:425-437, :495-500, :617-655 and mod.rs:1266-1300. `copied` starts each step's copy from `Types::of(&term.types, 0)` and `term.type_variables.clone()`, `fresh_type` pushes a name for every `ΛX` copied, and the made `Term` takes `copy.types.list` and `copy.type_variables` whole; `Builder::finish` (not used here) trims nodes only, and nothing after `copied` (`rename`, `rename_types`) drops a type. So every step adds one type variable and one `Type::Variable` per `ΛX` still in the term, which gives the finder's n² on its family (100, 400, 1600). I did not rerun the scratch measurement; the reading supports it. core-lambda.md records the fresh `TypeVarId` per `ΛX` copied (needed for `TypeTwice`), which the main change (compacting) keeps; a compaction that keeps the arena's order also keeps 'the first name interned wins'.

Correction: The measured numbers are arena sizes, not an instrument of P1, and the criterion fits loosely (P2 is allocation rate; this is growth of data kept and re-hashed per step by `Types::of` in check.rs:1086). Drop the optional 'reuse the source's TypeVarId for a ΛX copied once': the copy's argument for soundness (core-lambda.md, 'A type β step copies with a fresh TypeVarId per ΛX copied') rests on every copied ΛX being fresh. The change alters the reduct's JSON (type ids, `type_variables`), so it is not behaviour-kept for the wire: the manual's reduce examples with JSON and the wire-schema check are the net.

## F-5a-02: confirmed

Opened mod.rs:175-193 (`#[cfg(feature = "serialize")] pub(crate) mod read`), read.rs:139 (`pub(crate) fn from_parts`), mod.rs:212-265 (the four ids have `index` alone), mod.rs:464 (`Atom { atom: u32 }`) and listed every `pub fn` of mod.rs, print.rs, check.rs, parse.rs: a `Term` comes only from `Term::parse`, the JSON reader or a read-back. X1 asks for construction without the text parser in every suite. No rules file records the privacy as a choice; core-lambda.md says a term made another way must keep the types hash-consed, which `from_parts` does by interning. It repeats F-arch-05 (confirmed in arch/verdicts.md) for this module, as the handover asked.

Correction: Same matter as F-arch-05: count it once. Making `Parts`/`from_parts` public and giving the ids `new` is an addition, not a break; only the `AtomId` newtype for `Type::Atom` breaks.

## F-5a-03: confirmed

Opened mod.rs:1304-1395, read.rs:52-134, reduce.rs:557-612, poly.rs:53-112, print.rs:466-492. All quotes are at their lines. The duplication is real but smaller than 'ten times one walk': there are three different node projections (subterms: `operands` and its mutable twin `renumber`; variables: `variables_of` in read.rs and its mutable twin `rebind` in reduce.rs, with `binders` a different set, binders only; written types: `check::written`, single), and one type walk written four to five times (`read::type_operands`, `poly::operands` with a depth, the inline match of `print::free_in`, and `read::retyped` against `poly::rebuild`'s Leave arm, which is `retyped` with a depth). `Node` and `Type` are public and not `non_exhaustive`, so a user can match them; no public walk exists.

Correction: Claim: say 'three node walks, two of them written twice (shared and mutable), and the type's operands written four to five times', not ten copies of one walk. The clear wins are `Type::operands`/`map_operands` (replaces read::type_operands, poly::operands, free_in's match, retyped and rebuild's arm) and one variables walk for read.rs and reduce.rs; a shared/mutable pair over `Box<[TermId]>` lists needs a closure form (as now) or an iterator type, so `operands`/`renumber` merge less cheaply. `binders` and `written` are not duplicates of anything.

## F-5a-04: confirmed

Opened print.rs:348 (`pub(crate) fn write_type`, eight arguments), check.rs:42-57 (`Typing::types`, `Typing::ty`) and the public functions of print.rs (:1243 `write`, :1268 `write_judgement`, :1310 `write_marked`, `Display for Term`): nothing public prints a `TypeId`, and `Typing::types` holds types the term's own arena lacks, so even `write_judgement` cannot show a subterm's type. X1 names 'a formula printed'. No rules file records this as a choice. The length bound of core-lambda.md ('The types are a DAG too') must hold for the new entry point, as the finding says.

Correction: Kind: an addition to the API, not a break.

## F-5a-05: confirmed

Opened check.rs:758-793 and :1374 (the checker's `Meter` asks a stop wrapped by `limits::counting` once per 1024 units, and `counting`, limits.rs:877-889, then reports work 1 and done = number of polls), reduce.rs:292-329 and :392 (`Progress::new(Phase::Reduce, 1, done)` once per 4096 units, `done` the copy's node count or a loop index), inhabit.rs:777-799 (`Progress::new(Phase::Search, POLL, self.work)`), and limits.rs:408-412 (`work` is units since the previous poll in the unit of `Limits::work`, `done` units since the call began). Only the enumerator reports what `Progress` documents. No rules file records the units as a choice.

Correction: Overlaps F-2a's one-meter finding (units/2a/findings.md:387) and F-arch-03: one crate-private meter is a cross-module change, so the cost is large if done crate-wide and medium for the lambda suite alone. The `Progress` numbers a caller's stop sees are public behaviour, so the kind is nearer an API change than a refactor; the bindings' progress display should be checked against it.

## F-5a-06: confirmed

Opened mod.rs:335-372 and :417-431 (`NAMES` documented 'as the JSON form and the command write them', snake case; `FromStr` compares with `name()` only), reduce.rs:114-127 (the same for `Strategy`), cli/src/argument_parsing.rs:1961, :2093-2132, :2150-2155 (the command's own `CalculusArg` and `StrategyArg` value enums, kebab case). The command reads `system-f` and the JSON writes `system_f`; `ALL`, `NAMES`, `name()` and serde's `rename_all` (mod.rs:297, reduce.rs:101) are four hand-kept lists, and I found no test in the lambda tests or the command that ties them. The handover ('One spelling for the names of values') asked this unit exactly this.

Correction: Criterion: L8 is about subcommands and flags; the witness is a naming matter between the library, the JSON and the flags, the same as F-7a-03, and should be settled with it (criteria.md:43 names L6 and L8 together). Cost is small for the doc line, `name()` from `NAMES` and the test; the one spelling crate-wide is unit 7a's decision and large.

## F-5a-07: candidate

Opened mod.rs:459-462 and :613-616 (both enums derive `Deserialize` with `tag = "kind"`), target/audit/wasm.md:13, :119, :126, :150-156 (the two functions, 73 806 and 32 492 bytes, two copies each with the same instantiation over serde_json's `StrRead`), and core/src/serialize/lambda.rs:158-202 (`TermForm` reads `Vec<Type>` and `Vec<Node>`). Internally tagged enums do go through serde's `Content`. Whether a flat form with optional fields and one `match` is smaller is not known from reading: a derived struct of some fifteen optional fields plus the conversion is code too. Measure the module's raw and gzip bytes before and after with twiggy and the budget check.

Correction: `Node` and `Type` have no `deny_unknown_fields` today, so the proposed flat form with `deny_unknown_fields` would refuse documents now accepted, and a field of another kind (a `body` on a `var`) must be refused or ignored by the `match` as the tagged enum does; 'the JSON accepted stays the same' needs a test of both. The public `Deserialize` impls of `Node` and `Type` stay (removing them would be a break), so the saving exists only if nothing else in the module instantiates them.

## F-5a-08: confirmed

Opened parse.rs:55-90 (`Frame`), :1234, :1267-1437 (`Open`, `untyped_variable`, `untyped_use`, `untyped`, `untyped_apply`), :332-338 and :1038-1044. The untyped reader is about 170 lines with its own stack (`Open::Run`, `Bracket`, `Lambda`), context loop and application run beside the typed parser's, and the subscript-digit loop is written twice word for word. core-lambda.md lists `Parser::untyped` in the layout and records its rule for free names, but gives no reason for a second reader. The two already share the cursor, `name`, `word`, `scope`, `count` and the errors with spans.

Correction: The gain is tidiness: the untyped reader has its own semantics (free names become the context as they appear, a hint for a typed `λ`), and merging means an optional type through `Value` and `make` in a typed parser whose rule code may `expect` a type (core-lambda.md, 'Typed or untyped is the calculus's'). 'Untyped terms get the typed parser's errors and spans' is no gain: they have spans now. The cheap, certain part is one helper for the subscript digits (:332 and :1038) and for the turnstile test (:1234 and :1331); the merge of the two readers is medium cost for a modest gain.

## F-5a-09: confirmed

Opened lambda/draw.rs:27-75 (`LABELS` and `NAMES`, two tables of 21 rows that differ in the subscript markup of six rows) and :606-613 (`Subscript` and `Table(_)` take the `Upright` arm), rules/draw.rs:285-291 (the same), and linear/proofs/style.rs:32-54 (`Labels::Table(BTreeMap<Named, String>)`, whose docs do not say it applies to linear proofs alone). core-export.md:124-126 states the table is keyed by `Named` as a fact, with no reason. This is F-1b-04 seen from this unit (confirmed in units/1b/verdicts.md, which lists lambda/draw.rs:609), as the handover asked.

Correction: Count the `Labels::Table` part with F-1b-04, not as a new finding. New here: `Labels::Subscript` is ignored too, and `NAMES` repeats `LABELS`. Deriving `NAMES` from `LABELS` cannot be a `const` table without a const conversion of `_1` to `₁`; either compute the plain name on demand or keep one table of pairs.

## F-5a-10: confirmed

Opened mod.rs:1-64. The summary line is 'A proof as a program: an intuitionistic proof read back as a lambda-term', the first section 'Get a term' proves a linear sequent with `deduxit::linear::{Mode, Options, Sequent, Verdict, prove}`, and 'Write and check a term' comes second; reduction, principal types and inhabitation follow. The module also holds the untyped calculi and System F, which no proof reads back to. T3 and D4 say what the finding says; core-lambda.md records no choice for the order. The first example stays valid as a later section, so the doctests keep running.

## F-5a-11: confirmed

Opened parse.rs:1438-1500: the syntax table and the paragraphs after it have no row for `ΛX. M`, `M [A]` or `∀X. A`, although the parser reads them (parse.rs:85, :452, :628, :1127), and 'in the other two' at :1477 dates from three calculi (there are nine). Opened reduce.rs:774-786: the doc names the type redex `(ΛX. M) [A]` at :777-778 and then says 'Only the application of a `λ` is a redex' at :782-783.

## F-5a-12: confirmed

Opened reduce.rs:701, :753, :766-773 and print.rs:536-546: `fresh_name` is `print::fresh` without the first test, and at both call sites the name is in `taken` (`taken` is every name of the term, or every atom's and type variable's), so `print::fresh(name, |c| taken.contains(c))` gives the same name. Opened mod.rs:1222-1235 and inhabit.rs:714-724: the letter and index of a new variable are computed by the same eight lines. core-lambda.md ('Variable names follow the read-back's letters per kind of type') wants the two to agree, which one function ensures.

Correction: Place also core/src/lambda/mod.rs:1224 and core/src/lambda/inhabit.rs:716 for the second duplicate.

## F-5a-13: candidate

The fact holds: bench/ceilings.csv lists twenty journeys (search, read, check, derivation, render, batch, ordinary-pigeons) and none of the lambda suite, target/audit/ratchet.out has no line with 'lambda', and dhat.md's matches for 'term' are the linear and first-order `TermId` only. The reading candidates are at their lines (print.rs:775 `pieces` returns a `Vec` per node, called at :1018 and :1139; inhabit.rs:368 `best.contains` over whole terms; :813 `self.formulas.clone()`; :1223 `self.base.clone()`), but whether any is hot is not known. Measure: instruction counts under callgrind and allocations under DHAT for a reduction (untyped Church numerals and System F), a print of a shared DAG near the bound, and an enumeration cut at a few thousand terms.

Correction: Place: the missing journeys are in bench (bench/ceilings.csv and the ratchet's journey list), not print.rs:775. Adding journeys raises the ratchet's run time (B7); three small ones are enough.

## F-5a-14: confirmed

Opened lambda/draw.rs:615-630 and rules/draw.rs:294-309: `len`, `root` and `premises` are the same lines over the same three fields (`nodes`, `ends`, `premises`), down to the comment. One small post-order tree type would serve both. The gain is a reader's; the cost is small.
