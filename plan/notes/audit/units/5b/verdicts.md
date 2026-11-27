# Unit 5b: verdicts

13 confirmed, 5 candidate, 1 rejected.

## F-5b-01: confirmed

Opened core/src/workspace/mod.rs:1534-1562 (limit_named, eleven keys), core/src/linear/search/mod.rs:3645-3662 (Reason::setting, with search.path_length, search.model_size, search.model_worlds) and :3587-3601 (the three reasons are in Reason::NAMES, so the reason survives and only the setting is dropped), core/src/protocol/ordinary.rs:680-688 and modal.rs:482, lambda.rs:504, linear.rs:531 (all write reason.setting()), and cli/src/workspace.rs:512-517. The list is a hand copy that lacks three keys; no rules file records it as a choice. The loss is wider than the claim says: Workspace::run (mod.rs:1321-1322) goes compute then accept, and accept (recorded, :1195) calls limit_named, so every workspace run ended by one of the three bounds loses the setting, not only a record read back or accepted from a worker.

Correction: The witness's last sentence is off: the run the workspace made itself also prints "stopped at model_size", since run goes through accept. The kind is a fix of behaviour (the finder says so in the net), not a refactor. Place and cost stand.

## F-5b-02: confirmed

Opened cli/src/calculus.rs:28 and web/src/rules.rs:38, :55 (the same 13-entry table, and its names again), core/src/rules/mod.rs:102-163 (thirteen pub consts, no table), core/src/protocol/rules.rs:207-214 and its one caller :369, core/src/rules/natural.rs:84. cli.md:298 and web.md:346 only name each front end's BUILT_IN, with no reason against a shared table. Two of the five places are weaker than claimed: the protocol's built_in is called only with from.logic().calculus(), which is LK, LJ or LM, so its three names are all it needs and it disagrees with nothing; natural::FILES is a table of profiles (logic, classical rule, rule list) keyed by name, not a name-to-text table. The real duplicate is the command's and the bindings' table.

Correction: Kind: an additive library API (new const and function) plus refactors in cli and web, not an API break. The protocol's and natural.rs's places are not duplicates of the table; they could read it but need not. Cost large stands (three crates).

## F-5b-03: confirmed

Opened core/src/rules/calculus.rs:362-369 (the doc on positions) and :823-879 (the rule that assigns Find::Start/End/Position/Value), and docs/src/rules.md:49-51. The public texts give only the count; the rule is stated in .claude/rules/core-rules.md ("How a written member is found"), which no user reads. The finder's wording matches the code: a member is forced when contexts <= 1 and (sequence or the zone has one entry); otherwise the conclusion's members, a sequence's and a premise's with an unbound variable take a position; positions are consumed conclusion first, zone by zone, in written order (check.rs:519-559).

Correction: The criterion fits T3 or X4 better than X3 (X3 is about no unchecked path); the finding stands under either. An iterator `needs` would be an additive API, beyond kind `text`.

## F-5b-04: confirmed

Opened core/src/rules/derivation.rs:14-41 (NodeId has no public constructor, ScopeId only ROOT), :59-107 (Node's premises: &[u32], scope: u32, while the builder Node::scope takes ScopeId), :395 (scope(parent: u32) returns ScopeId), :418 (push returns NodeId). The casts are there: natural.rs:906, :927, :948, rules/ordinary.rs:206, modal/decide.rs:573. Derivation::new(name, zones) is called with a hand-written zone count at mod.rs:31, natural.rs:855, modal/decide.rs:456. No rules file records the raw indices as a choice. push validates every index, so this is type hygiene, not soundness.

Correction: Cost is large by the brief's scale: rules/, modal/, serialize/rules.rs, two test files and the doc example change together.

## F-5b-05: confirmed

Opened core/src/rules/check.rs:341-357 (the node index goes only into the formatted reason) and core/src/errors/mod.rs:709-716 (calculus and reason only; the variant is #[non_exhaustive]). Error::InvalidRefutation already carries `step: Some(1)` (errors/mod.rs:1506-1508), so a structured place is the crate's own pattern. core-rules.md ("Errors") says only that the error names the first node that fails.

Correction: Kind: an additive public field and a new JSON key, not a pure refactor. Cost is medium: the variant is built in about a dozen files (rules/check, draw, fitch, ordinary, tableau, interactive, modal/decide, serialize/modal, serialize/errors, ordinary/written), each of which gains `node: None` or the index.

## F-5b-06: confirmed

Opened core/src/rules/calculus.rs:437-438 (an empty sort is reported as declared twice), :673, :770-771, :778-781, :882-893 (`name` is in scope at :889 and unused in the message), :996-1000, and core/src/rules/text.rs:459-464 (known: &[]). Each quote is in the file as given. The messages name the zone but not the variable, context or sorts.

## F-5b-07: confirmed

Opened core/src/workspace/form.rs:359-387, :454, :581-583, mod.rs:1136-1139 and the one caller web/src/workspace.rs:40. The witness holds: a document passed to read_own keeps its checks as written, and its records are trusted and answer a check's own runs. core-workspace.md ("Two readers") records why the reader exists, and the finding keeps both. The value is small: the doc comment at form.rs:368-375 already says a document from elsewhere goes through wire::upgrade, so only the rename and a `# Security` heading remain, at the price of an API break with one caller.

Correction: `read_own_unchecked` would mislead: the reader does check numbers, order, log, bounds and values. `read_trusted` fits. The second half of the change (say where a foreign document goes) is already in the doc.

## F-5b-08: confirmed

Opened core/src/rules/ordinary.rs:24-47 (terms_of, pub(crate), no feature gate), core/src/rules/natural.rs:854-881 (the same loop over formulas.nodes() with the same seven connective names, in the same arena order) and :1084-1098 (the inverse match). Translator::derivation could call terms_of(&self.formulas, derivation.terms_mut()) and get the same Vec<TermId>. No rules file records the copy as wanted; core-rules.md's independent second converter is the one in core/tests/natural.rs, which stays.

Correction: The first half (call terms_of) is the clear gain. One table for both directions saves less, since the two matches have different shapes.

## F-5b-09: confirmed

Opened core/src/rules/print.rs:70-95 and core/src/rules/text.rs:191-213. The fixity and precedence defaults are the same code twice, and core-rules.md states both the defaults and "what `show` writes reads back", which rests on their staying equal. One difference the finder missed: the printer takes the arity of the term it prints and writes a name it does not know as a function, while the reader takes the declared arity (and has an explicit (_, 0) arm that the printer reaches through its fallback).

Correction: A table computed at read time serves the printer only where the term's arity is the declared one; the printer needs its fallback for other terms. A shared function of (declared fixity, declared precedence, arity) removes the duplicate with no such case. text.rs is behind `parse`, so the shared piece lives in print.rs or calculus.rs.

## F-5b-10: confirmed

Opened core/src/workspace/mod.rs:1564-1571 (len: 0 hard-coded, Space::Node) and core/src/errors/mod.rs:1196-1199 ("a proof refers to node {index}, but it has {} nodes"). no_node is returned at mod.rs:945, :1026 and :1610 (plan, compute, Edit). The command guards one path with its own message (cli/src/workspace.rs:106, "the workspace has no node N"), so the false sentence reaches library users, the bindings and the command's other paths.

## F-5b-11: confirmed

Opened core/src/rules/text.rs:220-239 (term_of bounds the slice it is given), :485-512 (members calls term_of per member), :473-482 and :514-531 (parse_list and parse_sequent lex the whole text and never count it), and the docs at :520-524. The docs and core-rules.md ("A text's tokens count against limits.occurrences") say per text; the code bounds each member. The lexer has already built every token by then, so the gap is in the terms built, not in an unbounded read.

Correction: Kind: with the code changed this is a small behaviour change (more texts refused), not a refactor; with the docs changed it is `text`.

## F-5b-12: candidate

Opened core/src/rules/check.rs:221-227 (check_some generic over the stop), core/src/rules/search.rs:126-132, core/src/workspace/mod.rs:1003-1008 and :1303-1309, core.md:327-336 (the rule) and natural.rs:981-1007 (the pattern followed). All four are generic. For the checker the gain is less than the 150 lines suggest: Meter holds the stop as `&mut dyn FnMut` (check.rs:38), so Checker's methods are compiled once (llvm-lines.txt lists Checker::equations and Checker::rule with one copy each); only check_some's own body (the estimate and the node loop) repeats per caller. target/audit/wasm.md's table of functions compiled more than once does not list any of the four among its top rows.

Correction: To measure: twiggy on the web module (or cargo llvm-lines on the cli) before and after, counting copies and bytes of check_some, Interactive::hint, Workspace::compute and Workspace::run. Expect a small number for check_some.

## F-5b-13: candidate

Opened core/src/rules/check.rs:511-597. The String at :544-548 is built for every written member and read only in the two error paths; :512-514 collects the sequents, :539-540 partitions into two Vecs per zone; mapped and unmapped (:778, :798) return Vecs, and :912 and :937 call unmapped with a one-member slice. Reading supports the claim. The change touches the checker's code (not what it decides), so it goes to the panel.

Correction: To measure: DHAT allocations and callgrind instructions for a journey that checks from_ordinary of a large LK derivation and one that checks a long natural deduction derivation, before and after. No journey of the ratchet checks a rule-file derivation today, so one must be added first.

## F-5b-14: candidate

Opened core/src/workspace/mod.rs:599-613 (key_of clones parameters and settings, both Box<str>), :616-683 (states walks every node), :700-704 (state computes all states and indexes one), :1313, :1325, :1345 (run asks state per planned node) and cli/src/workspace.rs:421 (nodes() per node run). The quadratic shape is as described. key_of runs in states only for a node with a shown record and no waiting input. Each run's own cost (an operation, a search) is likely far above this at the bound on nodes, so only a measurement says whether it shows.

Correction: To measure: callgrind on a journey that builds a chain of a few hundred derived nodes with instant operations and runs it, and the same near limits.workspace.nodes; compare the share of states and key_of.

## F-5b-15: candidate

Opened core/src/workspace/mod.rs:373 (interned: HashMap<(&'static str, Box<str>), ValueId>), :566-597 (the lookup boxes a copy of the text, the text is stored in values and again as the key, bytes sums every value on each call) and :1127 (asked copies each input for the lookup), :1479-1489 (collect clones every text into the map again). The witness holds, and limits.workspace.bytes counts each text once while two copies are held.

Correction: To measure: DHAT peak bytes and allocation count for a workspace holding a few large values (a session, a derivation of some hundred KB) through an edit and a run. That the bound covers half the bytes held is a fact from reading, not a measurement.

## F-5b-16: candidate

Opened core/src/rules/search.rs:126-250: `self.clone()` at :135, `state.clone().assume(goal)` at :203 and `state.clone().apply(goal, …)` at :220, as quoted. Each hint also runs a full decision search from the root (found_at, :141) and a translation, which is likely to dominate three clones of the session; unit 4e's F-4e-09 on the same pattern is a candidate too (target/audit/units/4e/verdicts.md).

Correction: To measure: DHAT and callgrind on a session journey in NK that asks for a hint at each of some twenty steps, with the clones' share of bytes and instructions set against found_at's.

## F-5b-17: confirmed

Opened core/src/workspace/form.rs:568-580 and :760-772 (the same expression) and core/src/workspace/mod.rs:1508-1518. The two in form.rs are identical. The one in collect is not the same loop: it remaps each record's key inputs and output in the same pass (iter_mut with `map`) before inserting, so it would call reindex after its remap. Tidiness at small cost.

Correction: Two true duplicates (form.rs); collect's loop does more and would be split in two to share. Place for the claim: form.rs:568 and form.rs:760.

## F-5b-18: confirmed

Opened core/src/rules/draw.rs:85-103 and core/src/rules/fitch.rs:184-203 (the same fourteen lines) and core/src/rules/check.rs:228-244, :361-362. The two layouts open alike and one helper serves both. The checker differs: it checks the calculus's name first, then the zones, estimates memory, resolves the terms, and says "it has no node" only after the loop, so a shared root_in would change which message an empty derivation with a bad term gets.

Correction: Share the helper between Drawing::new and Fitch::new; for the checker share at most the zones test, and leave its order alone (a change there goes to the panel for no gain).

## F-5b-19: rejected

The quote is at core/src/rules/fitch.rs:184-189 and :481, but neither variant of the change gains a user anything. Every caller outside tests borrows the options from styles it keeps (cli/src/calculus.rs:254 and :359, both `&show.styles.fitch`, the first two calls on one value; web/src/rules.rs:798 `&styles.fitch`), so by value each would clone at the call site instead. Borrowing for 'a breaks the documented call `Fitch::new(&calculus, &derivation, &FitchOptions::default(), &limits)` (skills/deduxit/reference.md:2261, core/tests/rules.rs:1057), whose temporary ends with the statement. What is saved is two short Strings (fitch.rs:38-46) per drawing. Under D2 a break needs a gain a user can name, and there is none.

## Beside the findings

- core/src/workspace/mod.rs:1195 with :1541-1557: every workspace run goes compute then accept (Workspace::run, :1321-1322), and accept maps the limit's setting through limit_named's eleven keys, so a run ended by search.path_length, search.model_size or search.model_worlds is recorded with setting None at once, not only after a read-back. `deduxit workspace` then prints "stopped at model_size" (cli/src/workspace.rs:517) and never names the setting to raise. This is F-5b-01's defect at its full extent.
- core/src/rules/text.rs:503-510: parse_sequent and parse_list apply limits.occurrences to each member's tokens, not to the text, so a sequent of many short members builds more terms than the bound the docs (text.rs:520-524) promise. This is F-5b-11; noted here because it is a bound not kept.
