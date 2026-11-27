# Unit 5b: findings

Read in full: 15 files. Not read in full: none.

The finder's remarks: I could not judge F-5b-12 to F-5b-16 by measurement: no ratchet or DHAT journey runs rules, the workspace or a session, so each needs a journey added first (P1). The checker is sound as far as I read it (positions order, scopes, freshness, sets). Read-in-JSON scopes keep parent < child (`serialize/rules.rs:578`, `Derivation::scope`), and no SipHash map is in this unit. On the handovers: I agree with F-arch-09, since `natural.rs`, `ordinary.rs` and `tableau.rs` use only public `rules` API and could move to `ordinary`, while `search.rs` needs `Interactive`'s crate-private parts. Rule files name the linear suite's `InfId`, `Labels`, `Drawn`, `LITERAL`, `RAISED`, `superscript` (`draw.rs:10-12`) and `Notation` (`print.rs:10`); `search.rs` needs `linear::search::Options` only to pass on to `ordinary::decide_within` (F-arch-02); the workspace needs `Reason::NAMES`. `Drawing::markup` (`draw.rs:285`) ignores `Labels::Table`, so a user's labels never reach a rule file's drawing (F-1b-04): a rule is labelled by its file name alone. Unit 6a should look at `serialize/rules.rs:154-223`, where `PremiseForm` and `Once` write the same "key written twice" map visitor twice, and unit 8 at `protocol/rules.rs:207`, whose `built_in` knows only LK, LJ and LM (F-5b-02).

## F-5b-01

- dimension: architecture
- criterion: E6
- place: core/src/workspace/mod.rs:1541
- kind: refactor
- cost: small
- candidate: no

**Claim.** The workspace's hand-written list of settings keys that a limit names lacks the three keys of ordinary logic's searches, so a record read back from a file or accepted from a worker loses the setting that lifts its bound.

**Witness.** `workspace/mod.rs:1541-1557` (`limit_named`) keeps only the keys it lists:
```rust
    let setting = setting.and_then(|setting| {
        [
            "limits.memory_bytes",
            "limits.occurrences",
            "limits.terms",
            "limits.derivation_bytes",
            "limits.work",
            "limits.table_atoms",
            "limits.form_occurrences",
            "limits.recursion_depth",
            "search.copies",
            "search.witness_depth",
            "search.instances",
        ]
        .into_iter()
        .find(|&key| key == setting)
    });
```
while `linear/search/mod.rs:3659-3661` (`Reason::setting`, which `protocol/ordinary.rs:680` `limit_of` writes into every `Limit`) names three more:
```rust
            Self::PathLength { .. } => Some("search.path_length"),
            Self::ModelSize { .. } => Some("search.model_size"),
            Self::KripkeSize { .. } => Some("search.model_worlds"),
```
`limit_named` is called on `accept` (`mod.rs:1195`) and on reading (`form.rs:435`). Then `cli/src/workspace.rs:512-517` prints "stopped at model_size" where the run it made printed "stopped at its bound search.model_size".

**Change.** Derive the known keys from the reasons' and refusals' own `setting()` (a const slice next to `Reason::NAMES`, tested against every variant), or let `protocol::Limit` own its strings so that no mapping back to `&'static str` is needed.

**Value.** Users of the page and of `deduxit workspace` see which setting to raise after a file is opened or a worker's run is accepted. A new bound cannot be dropped silently again.

**Net.** The workspace's tests (`workspace/tests.rs`) plus a test that round-trips a record ended by `model_size` through `wire::upgrade`. This is a fix of a read-back, not a pure refactor. It touches no checker, search or certificate.

## F-5b-02

- dimension: api
- criterion: E6
- place: cli/src/calculus.rs:28; web/src/rules.rs:38; web/src/rules.rs:55; core/src/protocol/rules.rs:207; core/src/rules/natural.rs:84
- kind: API break
- cost: large
- candidate: no

**Claim.** The library exports thirteen rule-file constants but no table of them by name, so the command, the bindings (twice) and the protocol each write their own, and the protocol's knows only three.

**Witness.** `cli/src/calculus.rs:28` and `web/src/rules.rs:38` are the same table (duplicates.md, 122 tokens):
```rust
const BUILT_IN: [(&str, &str); 13] = [
    ("LK", rules::LK),
    ("LJ", rules::LJ),
```
`web/src/rules.rs:55` repeats the names: `pub const NAMES: [&str; 13] = [`. `core/src/protocol/rules.rs:207-213`:
```rust
fn built_in(name: &str) -> Option<&'static str> {
    match name {
        "LK" => Some(crate::rules::LK),
        "LJ" => Some(crate::rules::LJ),
        "LM" => Some(crate::rules::LM),
        _ => None,
    }
}
```
`rules/natural.rs:84` lists the eight natural deduction files by name a fifth time (`const FILES: [File; 8]`). `cli/src/calculus.rs:50-56` alone also resolves `G3…` to `modal::calculus`.

**Change.** Add `rules::BUILT_IN: &[(&str, &str)]` (a slice, per core.md's rule on open lists) and `rules::built_in(name, &limits) -> Option<Result<Calculus, Error>>`, which also reads `G3<system>`. The command, the bindings, the protocol and `natural::FILES` take their names from it.

**Value.** Library users get the shipped calculi by name without copying a table. A new rule file is added in one place, and the protocol, the page and the command stop disagreeing on which names resolve.

**Net.** The examples of README, the manual (`docs/src/rules.md`) and the guide (`cli/tests/readme.rs`), the bindings' tests under Node, and `protocol/tests.rs`. Additive in the library; no checker is touched.

## F-5b-03

- dimension: api
- criterion: X3
- place: core/src/rules/calculus.rs:362
- kind: text
- cost: small
- candidate: no

**Claim.** Which written members of a rule need a position in a node's `at`, and in what order, is decided by the compiler and said nowhere public, so a rule-file author or an outside engine cannot write a node's positions without trial and error.

**Witness.** The only public statement, `calculus.rs:362-365`:
```rust
    /// Returns how many positions a node of the rule named `rule` gives:
    /// one for each member the rule writes out that the checker cannot
    /// find otherwise, the conclusion's first, then each premise's.
```
The rule that decides it, `calculus.rs:846-860`:
```rust
                    let find = if contexts <= 1 && (sequence || entries.len() == 1) {
                        if seen_context {
                            Find::End((entries.len() - 1 - at) as u32)
                        } else {
                            Find::Start(place)
                        }
                    } else if sequence || unbound || index == 0 {
```
The manual (`docs/src/rules.md:49-51`) says only "A rule names the formulas it needs positions for, and `check` says when a node gives too many or too few."

**Change.** State the rule on `Calculus::positions` and in the manual's "The rule file". Every written member of the conclusion takes a position, unless its zone is a sequence with at most one context or holds it alone. A premise's member takes one where its zone is such a sequence with more contexts, or where it has a variable not yet bound. Positions run zone by zone in the order written. Better still, add `Calculus::needs(rule)`, an iterator of (sequent, zone, written member) in position order.

**Value.** Anyone who writes a calculus, or an engine that emits `rules::Derivation`s for one (X4's second way), knows which `at` each node takes before the checker refuses it.

**Net.** The manual's examples (`cli/tests/readme.rs`) and rustdoc. With the iterator, `core/tests/rules.rs` can compare it with `positions`. The checker is not touched.

## F-5b-04

- dimension: api
- criterion: C-NEWTYPE
- place: core/src/rules/derivation.rs:66
- kind: API break
- cost: medium
- candidate: no

**Claim.** Building a derivation by hand mixes the ids `push` and `scope` return with raw `u32` indices that `Node` and `scope` take, so every builder in the crate converts back and forth.

**Witness.** `derivation.rs:65-71` and `:395`:
```rust
    /// Its premises, as indices of earlier nodes, in the rule's order.
    pub premises: &'a [u32],
    /// The positions of the members its rule finds by position
    /// ([`Calculus::positions`](super::Calculus::positions)).
    pub positions: &'a [u32],
    /// The index of its scope.
    pub scope: u32,
```
```rust
    pub fn scope(&mut self, parent: u32) -> Result<ScopeId, Error> {
```
while `push` returns `NodeId` and the builder `Node::scope` takes `ScopeId`. The crate's own builders convert: `natural.rs:906` `results.push(node.index() as u32);`, `natural.rs:927` `let inner = derivation.scope(scope.index() as u32)?;`, `rules/ordinary.rs:203-207` `.map(|p| p.index() as u32)`. `Derivation::new("LK", 2)` (`mod.rs:31`) also repeats the calculus's zone count by hand.

**Change.** Have `Node::premises` take `&[NodeId]`, the field `scope` hold a `ScopeId`, and `Derivation::scope` take a `ScopeId` parent. Add `Derivation::of(&Calculus)`, which takes the name and zone count from the calculus. Make `NodeId::new` public as `ScopeId::ROOT` is.

**Value.** An outside engine writes a derivation without casts and cannot confuse a scope with a node. A derivation made with `of` cannot name the wrong zone count.

**Net.** `core/tests/rules.rs`, `core/tests/natural.rs`, the doc example of `rules`, the modal suite's builders (`modal/`), the JSON reader (`serialize/rules.rs:577`); `breaks 0.8.0`. The checker reads the same storage; no search is touched.

## F-5b-05

- dimension: api
- criterion: C-INTERMEDIATE
- place: core/src/rules/check.rs:347
- kind: refactor
- cost: small
- candidate: no

**Claim.** The checker knows which node fails but writes it only into the message, so a front end or engine that wants to point at the node must parse text.

**Witness.** `check.rs:347-355`:
```rust
                    invalid(match self.nodes[index].kind {
                        Kind::Rule(rule) => format!(
                            "node {index} ({}, concluding {sequent}): {reason}",
                            self.rules[rule as usize]
                        ),
```
and the error, `errors/mod.rs:709-716`:
```rust
    #[error("not a derivation in the calculus {calculus}: {reason}")]
    #[non_exhaustive]
    InvalidDerivation {
        /// The calculus's name.
        calculus: Box<str>,
        /// What fails, and at which node.
        reason: String,
    },
```

**Change.** Add `node: Option<u32>` to `Error::InvalidDerivation`: the failing node, `None` for a fault of the whole (wrong calculus, no node). The variant is `#[non_exhaustive]`, so the field is additive. Write it into the error's JSON.

**Value.** The web page and `check --format json` can mark the failing node of a hand-written derivation, and an outside engine can repair or report it, without parsing the message.

**Net.** The behaviour lock's JSON forms (a new key in the error), `wire-schema`, `core/tests/rules.rs`. The checker's verdicts are unchanged.

## F-5b-06

- dimension: text
- criterion: T3
- place: core/src/rules/calculus.rs:882
- kind: text
- cost: small
- candidate: no

**Claim.** Several errors a rule-file author meets leave out the name that is wrong, and one names the wrong fault.

**Witness.** `calculus.rs:882-886`; the variable's `name` is in scope at `:889` but unused:
```rust
        let unbound = |what: &str| {
            invalid(format!(
                "a variable of {what} occurs in no sequent of the rule"
            ))
        };
```
`calculus.rs:437-438`, an empty sort reported as declared twice:
```rust
            if sort.is_empty() || sort_ids.insert(sort.clone(), index).is_some() {
                return Err(invalid(format!("the sort {sort:?} is declared twice")));
```
`:770` `"a context stands twice in the zone {name:?}"`, `:779` `"a context of the zone {name:?} is used elsewhere with another discipline or sort"` and `:997` `"its contexts cannot be told apart: …"` name no context. `:673` `"an argument of {name:?} has another sort than the connective takes"` names neither sort. `text.rs:460-464` refuses an unknown sort with `known: &[]`, so the message lists none of the calculus's sorts.

**Change.** Name what is wrong: the variable (`the variable "x" of `fresh` occurs in no sequent of the rule`), the context, the sorts expected and found, the contexts that cannot be told apart. Say "a sort has an empty name" for an empty sort. Pass the calculus's sort names as `known`.

**Value.** A person writing a calculus by hand finds the faulty name from the message, without bisecting the file.

**Net.** `core/tests/rules.rs` (the reader's refusals), the manual's examples. No verdict changes.

## F-5b-07

- dimension: api
- criterion: C-VALIDATE
- place: core/src/workspace/form.rs:381
- kind: API break
- cost: small
- candidate: no

**Claim.** `Workspace::read_own` is a public reader that trusts every record and check of whatever document it is given, but its name does not mark it as the unchecked path.

**Witness.** `form.rs:381` `pub fn read_own(document: &str, limits: &Limits) -> Result<Self, Error> {`. Then `form.rs:454` and `:581-583`:
```rust
            trusted: trust == Trust::Own && !(foreign || record.foreign),
```
```rust
    if trust == Trust::Claim {
        verify(&mut workspace)?;
    }
```
A document passed to it can claim a `held` check or any search output. That output then answers a check's own runs (`asked`, `mod.rs:1136-1139`, which takes `trusted` records). core-workspace.md ("Two readers") records why the page needs such a reader; its one caller is `web/src/workspace.rs:40`.

**Change.** Keep both readers, as the rules file decides, but name this one for what it skips (`read_own_unchecked`, or `read_trusted`) and say under `# Security` or the summary that a document from anywhere else must go through `wire::upgrade`.

**Value.** A library user who keeps workspaces on disk does not read a file someone else edited through the path that trusts it. The checks then stay the trust boundary (X3).

**Net.** The bindings' tests under Node (the one caller), `workspace/tests.rs`; `breaks 0.8.0`. Behaviour is unchanged.

## F-5b-08

- dimension: architecture
- criterion: E6
- place: core/src/rules/ordinary.rs:24; core/src/rules/natural.rs:858; core/src/rules/natural.rs:1084
- kind: refactor
- cost: small
- candidate: no

**Claim.** The mapping between ordinary logic's formula nodes and a rule file's connective names is written three times in `rules`, twice in the same direction.

**Witness.** `rules/ordinary.rs:31-39` (`terms_of`):
```rust
            Atom(atom) => terms.atom(formulas.atom_name(atom), FORMULA)?,
            True => terms.apply("⊤", &[])?,
            False => terms.apply("⊥", &[])?,
            Not(a) => terms.apply("¬", &[ids[a.index()]])?,
            And(a, b) => terms.apply("∧", &[ids[a.index()], ids[b.index()]])?,
```
`natural.rs:868-874` (`Translator::derivation`), the same loop again:
```rust
                Node::True => ("⊤", vec![]),
                Node::False => ("⊥", vec![]),
                Node::Not(a) => ("¬", vec![at(a)]),
                Node::And(a, b) => ("∧", vec![at(a), at(b)]),
```
and the inverse at `natural.rs:1085-1091` (`ordinary_of`): `("⊤", []) => Node::True, … ("↔", &[a, b]) => Node::Iff(a, b),`.

**Change.** Have `Translator::derivation` call `terms_of(&self.formulas, derivation.terms_mut())`. Keep one table of (connective name, arity, node constructor) that both `terms_of` and `ordinary_of` read.

**Value.** A reader finds one place where ordinary formulas meet rule files. A connective added to ordinary logic (or renamed in a rule file) changes one table, not three.

**Net.** `core/tests/rules.rs` (the cross-check with the ordinary checker) and `core/tests/natural.rs`. These are the conversions, not the checker.

## F-5b-09

- dimension: architecture
- criterion: E6
- place: core/src/rules/print.rs:79; core/src/rules/text.rs:196
- kind: refactor
- cost: small
- candidate: no

**Claim.** How a connective without a declared fixity or precedence is written and how it is read are computed by two copies of the same defaults, and the round trip "what `show` writes reads back" rests on them staying equal.

**Witness.** `print.rs:79-88`:
```rust
        let fixity = match (declared.and_then(|c| c.fixity), arity) {
            (Some(Fixity::Prefix) | None, 1) => Fixity::Prefix,
            (Some(Fixity::Postfix), 1) => Fixity::Postfix,
            (Some(Fixity::Infix) | None, 2) => Fixity::Infix,
            _ => Fixity::Function,
        };
        let precedence = declared.and_then(|c| c.precedence).unwrap_or(match fixity {
            Fixity::Infix => 0,
            Fixity::Prefix | Fixity::Postfix | Fixity::Function => u32::MAX,
        });
```
`text.rs:196-206`:
```rust
        let fixity = match (form.fixity, arity) {
            (_, 0) => Fixity::Function,
            (Some(Fixity::Prefix) | None, 1) => Fixity::Prefix,
            (Some(Fixity::Postfix), 1) => Fixity::Postfix,
            (Some(Fixity::Infix) | None, 2) => Fixity::Infix,
            _ => Fixity::Function,
        };
        let precedence = form.precedence.unwrap_or(match fixity {
            Fixity::Infix => 0,
            Fixity::Prefix | Fixity::Postfix | Fixity::Function => u32::MAX,
        });
```

**Change.** Compute each connective's print form once when the calculus is read (a `Vec<Reading>` beside `connectives`), and let the printer and the parser both use it. The printer then does no hash lookup per term either.

**Value.** A reader changes the defaults in one place, and the text syntax cannot drift from the printer for rule files that leave fixity out.

**Net.** The text syntax's round-trip tests in `core/tests/rules.rs`, the manual's `interact --calculus` examples. No checker is touched.

## F-5b-10

- dimension: text
- criterion: T3
- place: core/src/workspace/mod.rs:1565
- kind: text
- cost: small
- candidate: no

**Claim.** A node number the workspace lacks is reported as a proof's node out of an empty list, which is false and points to the wrong thing.

**Witness.** `workspace/mod.rs:1565-1571`:
```rust
fn no_node(node: NodeId) -> Error {
    Error::IndexOutOfBounds {
        space: crate::limits::Space::Node,
        index: node.0 as usize,
        len: 0,
    }
}
```
which `errors/mod.rs:1196-1198` writes as:
```rust
        Space::Node => format!(
            "a proof refers to node {index}, but it has {}",
            counted(len, "node", "nodes")
```
so a workspace of five nodes asked for node 9 answers "a proof refers to node 9, but it has 0 nodes".

**Change.** Refuse a missing workspace node with its own message (`Error::NotApplicable` with "the workspace has no node 9", or `IndexOutOfBounds` with a `Space::WorkspaceNode` and the real count of nodes alive).

**Value.** A user of `deduxit workspace` and of the page reads which node is missing, not a claim about a proof.

**Net.** `workspace/tests.rs`; the error's code stays or changes with the variant (the JSON forms). No checker is touched.

## F-5b-11

- dimension: api
- criterion: C-VALIDATE
- place: core/src/rules/text.rs:503
- kind: refactor
- cost: small
- candidate: no

**Claim.** `parse_sequent` and `parse_list` check `limits.occurrences` per member, not per text as their docs say, so a sequent of many short members passes the bound.

**Witness.** The bound is applied to the slice `term_of` gets, `text.rs:232-239`:
```rust
        if let Some(limit) = limits.occurrences
            && tokens.len() as u64 > limit
```
and `members` hands it one member at a time, `text.rs:503-504`:
```rust
                Token::Comma if depth == 0 => {
                    members.push(self.term_of(terms, text, &tokens[from..index], sort, limits)?);
```
while `parse_sequent`'s docs (`text.rs:522-524`) promise "[`Refusal::Occurrences`] for a text of more tokens than `limits.occurrences`".

**Change.** Count the whole text's tokens once in `parse_sequent` and `parse_list` (after `Lexer::tokens`), as `parse_term` does in effect, or say in the docs that the bound is per member.

**Value.** The session's start (`Interactive::within`, from the command and the page) keeps to the bound the caller set, as core-rules.md says ("A text's tokens count against `limits.occurrences`").

**Net.** A test of `parse_sequent` with members totalling past the bound; the manual's `interact --calculus` examples. No checker is touched.

## F-5b-12

- dimension: performance
- criterion: P4
- place: core/src/rules/check.rs:221; core/src/rules/search.rs:126; core/src/workspace/mod.rs:1003; core/src/workspace/mod.rs:1303
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Four long bodies of this unit are generic over the stop, against the crate's rule that a long body takes `&mut dyn FnMut` behind a one-line shim, so each is compiled once per caller.

**Witness.** `check.rs:221-227`, a 150-line body behind `check`, `check_open` and `check_node`:
```rust
    fn check_some(
        &self,
        calculus: &Calculus,
        limits: &Limits,
        mut stop: impl FnMut(Progress) -> bool,
        which: Which,
    ) -> Result<(), Error> {
```
Its callers pass different closure types: `rules/interactive.rs:753, 770, 1510, 2272`, `modal/decide.rs:202, 284`, `modal/rocq.rs:434`, `protocol/rules.rs:379`, `natural.rs:1046`, the command and the bindings. Likewise `search.rs:126-132` `pub fn hint(… mut stop: impl FnMut(Progress) -> bool,` (125 lines), `workspace/mod.rs:1003-1008` `pub fn compute(… mut stop: impl FnMut(Progress) -> bool,` (108 lines; callers `run`, `Edit::ready`, `web/src/workspace.rs:400`), and `:1303-1309` `run`. core.md:327-336 states the rule, and `natural.rs:981-988` follows it.

**Change.** Make each a one-line shim over a body that takes `&mut dyn FnMut(Progress) -> bool`, as `Derivation::natural` does with `natural_dyn`.

**Value.** The web module and the command carry one copy of each body. The size is to be measured: twiggy on the module before and after, counting copies of `check_some`, `hint`, `compute` and `run`.

**Net.** `cargo test --workspace`, the bindings' tests under Node, the budget check. The checker's logic is unchanged (only its stop's type), so no verdict moves.

## F-5b-13

- dimension: performance
- criterion: P11
- place: core/src/rules/check.rs:544
- kind: refactor
- cost: small
- candidate: yes

**Claim.** The checker builds a message string for every member it matches, whether or not the match fails, and allocates several vectors per node and zone.

**Witness.** Inside the loop over every written member of every sequent of every node, `check.rs:544-548`:
```rust
                    let sequent = match s {
                        0 => "the conclusion".to_owned(),
                        _ if sequents.len() == 2 => "the premise".to_owned(),
                        _ => format!("premise {s}"),
                    };
```
used only in the two error paths below it. Per node and zone, `check.rs:512-514` collects the sequents into a `Vec`, `:539-540` partitions the items into two new `Vec`s, `:859-863` collects `rest`, and `mapped` (`:781`) clones a context's members on every call. In `sets`, `:912` and `:937` call `unmapped(&[m], under)`, which allocates a `Vec` per member.

**Change.** Build the location text only in the error path (a small `Copy` enum or a closure). Keep the item order (placed, then by value) in the compiled rule, and reuse `Checker`'s buffers for `rest` and the contexts. Give `unmapped` a one-member form that returns `Option<TermId>`.

**Value.** Checking large derivations (a rule file's check from the command or the page, the cross-checks of `core/tests/rules.rs`, every step of a session, which re-checks) gets faster. Measure with a journey that checks `from_ordinary` of a family's LK derivation under DHAT (allocations) and callgrind.

**Net.** `core/tests/rules.rs` and `core/tests/natural.rs` (the cross-checks hold every verdict), the mutation baseline if the checker is in it. This touches the checker (the panel), but not what it decides.

## F-5b-14

- dimension: performance
- criterion: P1
- place: core/src/workspace/mod.rs:700
- kind: refactor
- cost: medium
- candidate: yes

**Claim.** `Workspace::state` of one node computes every node's state, and `states` clones each recipe's parameters and settings to build a key, so `run` and the front ends' loops are quadratic in the nodes.

**Witness.** `mod.rs:700-704`:
```rust
    pub fn state(&self, node: NodeId) -> State {
        self.nodes
            .binary_search_by_key(&node, |n| n.id)
            .map_or(State::NotRun, |at| self.states()[at])
    }
```
`key_of` (`mod.rs:607-612`) clones both strings per node: `parameters: recipe.parameters.clone(), settings: recipe.settings.clone(),`. `run` calls `state` twice per planned node (`:1313`, `:1325`), and `cli/src/workspace.rs:421` calls `workspace.nodes()` (all states) once per node run.

**Change.** Compare a record's key with the recipe without building a `Key` (borrowed fields, or a `Key` that holds `Rc<str>`), compute one node's state from its inputs' states only, and let `run` compute the states once per run.

**Value.** The page's workspace and `deduxit workspace` stay fast as a workspace grows toward `limits.workspace.nodes`. To be measured: a journey that builds a chain of a few hundred nodes and runs it, under callgrind.

**Net.** `workspace/tests.rs`, above all `a_random_walk_keeps_current_nodes_fresh_and_undo_exact`, which compares every state with a fresh run. No search is touched.

## F-5b-15

- dimension: performance
- criterion: P2
- place: core/src/workspace/mod.rs:572
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Interning a value copies its whole text to look it up, keeps every text twice, and sums all values on each insert, so the bound on bytes covers half of what is held.

**Witness.** `mod.rs:572` allocates a copy for the lookup:
```rust
        if let Some(&id) = self.interned.get(&(kind.name, Box::from(text))) {
```
`mod.rs:586-590` stores the text in `values` and again as the map's key:
```rust
        self.values.push(Value {
            kind,
            text: text.into(),
        });
        self.interned.insert((kind.name, text.into()), id);
```
while `bytes` (`mod.rs:595-597`), which the bound `limits.workspace.bytes` reads, counts each text once and walks every value per call: `self.values.iter().map(|value| value.text.len()).sum()`. `asked` (`mod.rs:1127`) copies each input text the same way.

**Change.** Key the map by a hash of (kind, text) to `ValueId`s and compare against `values` (or use `hashbrown`'s `raw_entry`/`HashTable`), so that each text is held once and looked up without a copy. Keep a running byte count.

**Value.** The page holds half the memory for large values (sessions, derivations) and stays within the bound it shows. Measure DHAT bytes for a workspace with a few large values.

**Net.** `workspace/tests.rs` (one test per bound, the reference walk). No search or checker is touched.

## F-5b-16

- dimension: performance
- criterion: P3
- place: core/src/rules/search.rs:135
- kind: refactor
- cost: small
- candidate: yes

**Claim.** `Interactive::hint` clones the whole session, its calculus included, up to three times per call, the same pattern F-4e-09 found in `candidates`.

**Witness.** `search.rs:135`, `:203` and `:220`:
```rust
        let mut state = self.clone();
```
```rust
        if state.clone().assume(goal).is_ok() {
```
```rust
                let needs = match state.clone().apply(goal, &Apply::new(rule)) {
```
`rules::Interactive` is 1 024 bytes inline (type-sizes.txt) and owns its `Calculus` and term arena.

**Change.** Ask whether a goal is a line in force, and what `apply` would ask for, through read-only queries (the `plan` half of `apply`, a `closes_by_line(goal)`). Keep one scratch clone for `carried`, or carry the terms into a side arena.

**Value.** Each hint on the page and in `interact` gets cheaper. Measure with a session journey of a natural deduction proof with hints, under DHAT and callgrind, which the ratchet lacks (handovers: no journey runs a session).

**Net.** The tests in `search.rs` (`following_the_hints_proves_a_goal`, `following_classical_hints_proves_a_goal`). The search's result is unchanged.

## F-5b-17

- dimension: architecture
- criterion: E6
- place: core/src/workspace/mod.rs:1508; core/src/workspace/form.rs:568; core/src/workspace/form.rs:760
- kind: refactor
- cost: small
- candidate: no

**Claim.** The record index is rebuilt from the records by the same loop in three places.

**Witness.** `form.rs:568-580`, the same as `form.rs:760-772`:
```rust
    workspace.index = workspace
        .records
        .iter()
        .enumerate()
        .filter_map(|(at, r)| {
            r.as_ref().map(|r| {
                (
                    r.key.clone(),
                    RecordId(u32::try_from(at).unwrap_or(u32::MAX)),
                )
            })
        })
        .collect();
```
and `mod.rs:1508-1518` (`collect`), the same loop written with `insert`.

**Change.** Add `Workspace::reindex(&mut self)` and call it at the three places.

**Value.** A reader of the workspace sees one rule for what the index holds. A change to `Key` changes one place.

**Net.** `workspace/tests.rs`. No checker or search is touched.

## F-5b-18

- dimension: architecture
- criterion: E6
- place: core/src/rules/draw.rs:90; core/src/rules/fitch.rs:190
- kind: refactor
- cost: small
- candidate: no

**Claim.** The tree drawing and the Fitch proof open with the same checks of zones and root, written twice.

**Witness.** `draw.rs:90-103` and `fitch.rs:190-203` (duplicates.md, 118 tokens):
```rust
        let invalid = |reason: String| Error::InvalidDerivation {
            calculus: calculus.name().into(),
            reason,
        };
        if derivation.zones() != calculus.zones.len() {
            return Err(invalid(format!(
                "its sequents have {} zones, the calculus's {}",
                derivation.zones(),
                calculus.zones.len()
            )));
        }
        let Some(root) = derivation.root() else {
            return Err(invalid("it has no node".to_owned()));
        };
```
`check.rs:238-244` writes the zones check a third time.

**Change.** Add a crate-private `Derivation::root_in(&Calculus) -> Result<NodeRef, Error>` that checks the zones and returns the root, and call it from `Drawing::new`, `Fitch::new` and the checker.

**Value.** A reader finds one statement of what a derivation must be before it is laid out or checked, and the three messages stay alike.

**Net.** The exports' checks (`export`), `core/tests/rules.rs`. The checker's verdicts are unchanged.

## F-5b-19

- dimension: api
- criterion: C-CALLER-CONTROL
- place: core/src/rules/fitch.rs:187
- kind: API break
- cost: small
- candidate: no

**Claim.** `Fitch::new` borrows its options and then clones them, where taking them by value would let a caller that owns them avoid the copy.

**Witness.** `fitch.rs:184-189` and `:481`:
```rust
    pub fn new(
        calculus: &'a Calculus,
        derivation: &'a Derivation,
        options: &FitchOptions,
        limits: &Limits,
    ) -> Result<Self, Error> {
```
```rust
            options: options.clone(),
```
`FitchOptions` holds two `String`s.

**Change.** Take `options: FitchOptions` by value, or borrow it for `'a` as the calculus and the derivation are (`options: &'a FitchOptions`), which needs no clone at all.

**Value.** Callers that build options for one drawing (the command, the bindings) pass them without a copy, and the signature says what the function keeps.

**Net.** The exports' tests and the manual's Fitch examples (`docs/src/natural-deduction.md`); `breaks 0.8.0`. No checker is touched.
