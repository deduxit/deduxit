# Unit 3: findings

Read in full: 14 files. Not read in full: none.

The finder's remarks: Correctness, for the leader and unit 8: the protocol's `linear.net` (core/src/protocol/linear.rs:556-563) has no fallback to `MellStructure`. So it answers `NetError::Placement { fault: Cut }` (malformed) for a checked MLL proof that cuts one occurrence twice (core/src/linear/nets/cut/tests.rs:556), where the command and the bindings draw the net (F-3-01). I worked this out from the code and did not run it. On whether `ProofStructure` and `MellStructure` should share code: keep them two types, since vertex = occurrence is the net engine's hot path and `core-nets.md` records it. Share the graph kernel (F-3-02), the sequentialization's assembly (F-3-06) and a public sum type for the front ends (F-3-01). I could not judge from reading what the DHAT frames of `search-wide-m2-256` attribute to this unit (771 `Sequent::clone` blocks; `Switching::new` and `sequentialize` each clone the forest by decision). Unit 6b: `export/svg/mod.rs:1612` and `:1617` can run the criterion twice on one net, and both SVG writers sort links by hand (F-3-05). A low candidate left out: `mell/graph.rs:385` (`doors_in`, `cuts_in`) scans every door per box, O(boxes × doors) over a sequentialization. That is within the documented quadratic bound, and the stop is asked once per stage.

## F-3-01

- dimension: api
- criterion: X5
- place: core/src/protocol/linear.rs:556
- kind: API break
- cost: large
- candidate: no

**Claim.** Choosing the net of a proof (MELL with boxes for exponentials; for MLL `ProofStructure`, falling back to `MellStructure` on `Placement::Cut`) is left to every front end. Three front ends write it by hand, each with an either-net enum of its own, and the protocol's copy lacks the fallback.

**Witness.** The protocol, `core/src/protocol/linear.rs:556-563`, has no fallback:

```rust
                let criterion = Criterion::of(mode)?;
                if proof.sequent().fragment().has_exponentials() {
                    written(&MellStructure::from_proof(&proof, criterion, limits, stop)?)?
                } else {
                    written(&ProofStructure::from_proof(
                        &proof, criterion, limits, stop,
                    )?)?
                }
```

The command, `cli/src/show.rs:1482-1494`, has it, and the bindings repeat it at `web/src/calls.rs:2302-2313`:

```rust
        match ProofStructure::from_proof(proof, criterion, limits, |_| stop()) {
            Err(Error::Net(error))
                if matches!(
                    *error,
                    deduxit::linear::NetError::Placement {
                        fault: deduxit::linear::Placement::Cut,
                        ..
                    }
                ) =>
            {
                boxed(proof)
            }
```

Other copies:
- `nets_exist` is written twice, at `cli/src/show.rs:1590-1598` and `web/src/calls.rs:1490-1496`: `MellStructure::exist` for exponentials, else `nets::exist`.
- The either-net enums are `core/src/protocol/linear.rs:362` (`Plain`/`Boxes`), `cli/src/show.rs:1314` (`Mll`/`Mell`), and `web/src/calls.rs:2242` with `NetRef` at `:2251`.
- Reading a net document by probing `vertices` is done twice, at `protocol/linear.rs:380-386` and `web/src/calls.rs:2329-2331`.

The proof that needs the fallback is checked by the test `a_cut_occurrence_twice_needs_a_structure_with_boxes` (`core/src/linear/nets/cut/tests.rs:556`).

**Change.** Add to `linear::nets` a public `#[non_exhaustive] enum Net { Plain(ProofStructure), Boxes(MellStructure) }` with:
- `Net::exist(fragment, mode)`;
- `Net::of_proof(proof, mode, limits, stop)`, which takes the criterion from the mode and holds the `Placement::Cut` fallback;
- `is_correct`, `written_in`, `forest`, `cuts`, and a serde reader that probes `vertices`;
- `cut::step` and `cut::eliminate` taking it through the sealed `cut::Net` trait.

The protocol, the command and the bindings drop their copies. `nets::exist` stays MLL-only, since the net engine's `admits` uses it.

**Value.** A library user gets the net of any proof in one call, and the API answers the same as the command. Protocol clients get a net for a checked MLL proof that cuts one occurrence twice, where they now get an error. Three front ends lose about 60 lines each.

**Net.** README's and the manual's examples of `--net` and `nets` (`cli/tests/readme.rs`), the bindings' tests under Node, and the protocol's tests. Add a protocol test with the cut-twice proof. No checker, search or certificate changes; `from_proof` and `is_correct` stay as they are.

## F-3-02

- dimension: architecture
- criterion: E6
- place: core/src/linear/nets/graph.rs:283; core/src/linear/nets/mell/graph.rs:502
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The graph kernel of the correctness criterion is written twice, once for `ProofStructure` and once for `MellStructure`: the CSR layout, `Scratch`, the iterative Tarjan bridge search, Yeo's deletion rounds and the isolation of a cycle witness. The copies differ only in how a switched vertex finds its parent edge.

**Witness.** The clone detector pairs `graph.rs:309` with `mell/graph.rs:523` (303 tokens, 39 lines) and `graph.rs:297` with `mell/graph.rs:511`. Line by line, the same code appears in both:
- `search` and `discover`: `graph.rs:283-342` and `mell/graph.rs:502-556`;
- `acyclic`: `:374-399` and `:580-600`;
- `twin`: `:241` and `:603`;
- `cycle`: `:409-470` and `:614-662`;
- `Scratch`: `:68-90` and `:708-729`. MLL keeps `deleted` and `stuck` as `OccSet` bitsets, MELL as `Vec<bool>`.

The only real difference is the deletion test. In `graph.rs:347-356`:

```rust
        if forest.kind(OccId::new(v)) == Kind::Par {
            // A `⅋` has two premises and no axiom slot, so a third slot is
            // its parent edge, to its parent or across its cut link.
            let slots = self.slots(v);
            slots.len() < 3 || !present(slots.start) || scratch.is_bridge(v, self.to[slots.start])
```

In `mell/graph.rs:568-573`:

```rust
    fn deletable(&self, scratch: &Scratch, v: u32) -> bool {
        let mut slots = self.slots(v);
        if self.switched[v as usize] {
            slots.end = slots.start + 1;
        }
        slots.all(|i| !self.present(scratch, i) || scratch.is_bridge(v, self.to[i]))
```

**Change.** Keep the two structure types, since `core-nets.md` records that the MLL vertex is its occurrence on the engine's hot path.

Move the kernel into one crate-private module of `nets`: CSR, `Scratch` with bitsets, `search`, `acyclic`, `cycle` and `twin`, non-generic, taking the stop as `&mut dyn`. It reads a `switched` bitset and slot 0 as the parent edge. For this, the MLL graph gives every vertex a slot 0, `NONE` for a root (still three slots at most), and a `switched` bit per `⅋`.

Each structure keeps its own builder and its own `parts`, which names vertices its own way.

**Value.** A reader of the code checks one Yeo test, not two. A later fix lands once: F-3-03 is a MELL fix the MLL copy never got. MELL's scratch shrinks from a byte to a bit per vertex for `deleted` and `stuck`.

**Net.** This touches a checker, the criterion of both structures. It also touches the net engine's hot path (`is_acyclic` from `Switching::test`). Instruments:
- `nets::graph`'s `agrees_with_enumeration` and `mell::tests`' `agrees_with_enumeration`, with the witnesses pinned in `classic_structures` and `classic_nets`;
- the ratchet's `search-wide-m2-256` (the rules record +4.8 % when `acyclic` was generic, so keep the kernel non-generic);
- `bench/net-targets.sh` against `net-47.csv` (`verdict`, `links` and `tests` must not move: the search is unchanged).

## F-3-03

- dimension: performance
- criterion: P1
- place: core/src/linear/nets/graph.rs:504; core/src/linear/nets/sequentialize.rs:261
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Two steps of the MLL criterion and sequentialization are quadratic in the number of parts, and neither asks the stop: the `Disconnected` witness and the Mix branch of a stage. The MELL copies removed both after the integers panel measured them.

**Witness.** In `graph.rs:503-507`, each conclusion-like vertex does a linear search over the parts found so far:

```rust
            let label = scratch.component[v.index()];
            match parts.iter_mut().find(|(l, _)| *l == label) {
                Some((_, part)) => part.push(VertexId::of(v)),
                None => parts.push((label, vec![VertexId::of(v)])),
            }
```

`mell/graph.rs:693-697` uses an index per root instead:

```rust
            let label = scratch.component[v as usize];
            if index[label as usize] == NONE {
                index[label as usize] = parts.len() as u32;
                parts.push((label, Vec::new()));
            }
```

In `sequentialize.rs:261-267`, the Mix branch builds its labels with `contains` per conclusion. Then `of(&gamma)` filters every conclusion once per label (`:274-279`):

```rust
            let mut labels: Vec<OccId> = Vec::new();
            for &c in gamma.iter().chain(&cuts) {
                let label = self.scratch.component(c);
                if !labels.contains(&label) {
                    labels.push(label);
                }
            }
```

`mell/sequentialize.rs:317-339` sorts the labels and binary-searches them. `core-nets.md:428-433` records the MELL fixes: a witness of 40 000 parts took 3.4 s, and a stage of 320 000 roots 3.7 s, neither asking the stop.

**Change.** Port the two MELL fixes: an index per component root in `Graph::parts`, and sorted labels with `binary_search` in the Mix branch of `stage`. F-3-02 and F-3-06 would share them outright.

**Value.** A user's stop or time limit holds on large disconnected or Mix nets. Two places reach these steps: `is_correct` on a structure read from JSON or from the protocol, and the net engine's own `sequentialize` of a Mix proof (`search::net::answer`).

**Net.** To measure: `is_correct` on `|- a1, ~a1, …, ak, ~ak` linked pairwise under `Criterion::MLL`, and `sequentialize` of the same under `Criterion::MLL.with_mix()`, for k = 20 000 and 80 000. Use callgrind instructions or pinned time, and expect quadratic growth today.

Kept behaviour: the witnesses' order is pinned in `classic_structures` and `classic_nets`, the round trips, and the net target set's counters. This touches the criterion's witness, not its verdict.

## F-3-04

- dimension: text
- criterion: T1
- place: core/src/linear/nets/mod.rs:627
- kind: text
- cost: small
- candidate: no

**Claim.** `NetError::Mode` says proof nets exist "in linear mode only, with or without Mix". It also answers for a cyclic criterion with Mix, for cut links under the cyclic order, and for a structure with boxes under the cyclic order. The cyclic mode is linear, so in those three cases the message names the wrong reason.

**Witness.** In `nets/mod.rs:627-630`:

```rust
            Mode { mode } => write!(
                f,
                "proof nets exist in linear mode only, with or without Mix, not in {mode} mode"
            ),
```

In `nets/mod.rs:877-885`, a cut link under the cyclic criterion prints "… not in cyclic mode", and Mix under it prints "… with or without Mix, not in cyclic with Mix mode":

```rust
        // No calculus of the cyclic order has Mix or a cut.
        if criterion.order != Order::Commutative && (criterion.mix || !cuts.is_empty()) {
            let mode = if criterion.mix {
                Mode::CYCLIC.with_mix()
            } else {
                Mode::CYCLIC
            };
            return Err(NetError::Mode { mode }.into());
```

`mell/mod.rs:402-404` returns `NetError::Mode { mode: Mode::CYCLIC }` for a structure with boxes, whose real reason is that the ordered modes refuse exponentials.

**Change.** Say the real reason in each case, either by branching the message on the mode or with a field that names it:
- "the cyclic order has no Mix";
- "the cyclic calculus has no cut, so a structure of the cyclic order has no cut link";
- "proof nets with boxes need exponentials, which the ordered modes refuse".

Keep the affine wording, which `docs/src/proof-nets.md:133` and `cli/tests/cli.rs:1606` pin.

**Value.** A library user who builds a structure of the cyclic order with Mix or cut links learns what to change, where today the message contradicts itself.

**Net.** `cli/tests/cli.rs` and README's and the manual's examples (the affine message stays). No checker, search or certificate is touched; the code stays `no_nets`.

## F-3-05

- dimension: api
- criterion: C-COMMON-TRAITS
- place: core/src/linear/nets/mod.rs:778
- kind: API break
- cost: small
- candidate: no

**Claim.** The docs promise that two proofs differing in rule order give "the same net". Yet `ProofStructure` has no equality and no canonical link list, so the library's printers and exports and four tests each normalise and sort the links by hand.

**Witness.** In `nets/mod.rs:778-779`:

```rust
#[derive(Clone, Debug)]
pub struct ProofStructure {
```

The same normalisation, `(x.min(y), x.max(y))` followed by a sort, appears at:
- `nets/mod.rs:1532-1537` (`Display`);
- `nets/mell/mod.rs:803-808` (`Display`);
- `export/svg/net.rs:337-342`;
- `export/svg/mell.rs:281-286`;
- the tests' helpers at `nets/mod.rs:1987`, `search/net.rs:1098`, `search/essential.rs:702` and `nets/cut/tests.rs:298`.

```rust
        let mut links: Vec<(VertexId, VertexId)> = self
            .links
            .iter()
            .map(|&(x, y)| (x.min(y), x.max(y)))
            .collect();
        links.sort_unstable();
```

The promise is in `nets/mod.rs:35-36`: "two derivations that differ only in the order of their rules give the same net".

**Change.** Add `sorted_links()` to both structures: the links, smaller id first, ordered. Also give `ProofStructure` a `PartialEq` and `Eq` by forest, criterion, cut links and that sorted list. The links' history order is no part of the net, so a derive would be wrong. Replace the hand-written copies with these.

**Value.** A library user can check the promised canonicity in one comparison (`a == b`), as the tests need to. The exports and printers lose four copies.

**Net.** The text forms are pinned in `text_form` and `mell::tests`, and the SVG snapshots in the export tests stay byte-identical. The change is additive. No checker, search or certificate is touched.

## F-3-06

- dimension: architecture
- criterion: E6
- place: core/src/linear/nets/sequentialize.rs:177; core/src/linear/nets/mell/sequentialize.rs:187
- kind: refactor
- cost: medium
- candidate: no

**Claim.** The two sequentializations repeat their machinery for assembling a proof: the explicit step stack, `push`, `premise`, and the `Mix`, `Tensor` and `Cut` arms of the loop.

**Witness.** The clone detector pairs `mell/sequentialize.rs:199` with `sequentialize.rs:190` (138 tokens). Both have:
- `struct Sequentialization { net, scratch, nodes, steps, proved }`;
- `fn push`;
- `fn premise`, with the same `expect("a sub-net proved for every premise")`.

The loop is the same in both. From `sequentialize.rs:193-207`:

```rust
                Step::Mix => {
                    let other = self.premise();
                    let node = self.premise();
                    self.push(Node::Mix(node, other))
                }
                Step::Tensor(t) => {
                    let right = self.premise();
                    let left = self.premise();
                    self.push(Node::Tensor(Member::from(t), left, right))
                }
                Step::Cut(a) => {
                    let right = self.premise();
                    let left = self.premise();
                    self.push(Node::Cut(Member::from(a), left, right))
                }
```

MLL's `Step::Pars(Vec<OccId>)` is MELL's `Step::Opened(Vec<Open>)` with only `Open::Par`.

**Change.** One crate-private assembly in `nets`: the arena and the proved stack, with `push` and `premise`. Its step enum is generic over a sub-net's conclusions (`Prove(G)`, `Mix`, `Tensor`, `Cut`, `Bang`, `Opened(Vec<Open>)`), so that each sequentialization writes only its `stage`. The Mix branch's partition by label (F-3-03) belongs there once.

**Value.** A reader of the code sees each sequentialization as its stage alone. A fix to how sub-nets are joined lands once.

**Net.** The term the sequentialization writes must stay node for node the same:
- `classic_nets` in `sequentialize.rs` and `mell/tests.rs` pin node lists;
- the round trips;
- the behaviour lock;
- the net target set, since the net engine's proofs come from `sequentialize`.

The `debug_assert` that checks each proof stays. No checker is touched.

## F-3-07

- dimension: architecture
- criterion: E6
- place: core/src/linear/nets/mod.rs:1562; core/src/linear/nets/mell/mod.rs:815
- kind: refactor
- cost: small
- candidate: no

**Claim.** The two structures repeat three small pieces: the criterion's verdict line, the cut-link accessors, and the axiom case of cut elimination.

**Witness.** The verdict line in `nets/mod.rs:1562-1565`, written again in `mell/mod.rs:815-818`:

```rust
        match self.is_correct(|_| false) {
            Ok(()) if self.criterion.mix => {
                f.write_str("proof net with Mix (the correctness criterion with Mix holds)")
            }
```

- The accessors `cuts`, `cut_partner` and `is_cut_free`: `nets/mod.rs:1160-1179` and `mell/mod.rs:617-636`, word for word (the clone detector's `mell/mod.rs:615` against `nets/mod.rs:1158`, 110 tokens).
- The axiom step: `nets/cut/mod.rs:368-383` and `nets/cut/mell.rs:229-255` (the clone detector's `cut/mell.rs:200` against `cut/mod.rs:344`, 108 tokens). Both write `links[first] = (px, py); links.retain(|link| !ends(link));`.

**Change.** In `nets`, add:
- one private function that writes the verdict line from `is_correct`'s result, the criterion and the describer;
- one private struct `CutLinks` holding the list, with `cuts`, `partner` and `is_cut_free`, which both structures keep and delegate to;
- one helper, generic over the id type, that merges the two axiom links of an axiom cut.

**Value.** A reader of the code finds one wording of the verdict, which the manual quotes, and one cut-link list. A new verdict form, such as planar, is added once.

**Net.** `text_form` in both modules and README's and the manual's `--net` output (`cli/tests/readme.rs`) keep the bytes, and the cut elimination tests (`nets/cut/tests.rs`) keep every step. This touches the cut elimination's steps, which are checked after each step.

## F-3-08

- dimension: api
- criterion: C-HIDDEN
- place: core/src/linear/mod.rs:30
- kind: API break
- cost: small
- candidate: no

**Claim.** The eight public types of proof nets are reachable as both `linear::X` and `linear::nets::X`. The rest of the nets API (`nets::exist`, `nets::Scratch`, `nets::cut`) exists only under `nets`, so a user writes both paths for one task.

**Witness.** In `core/src/linear/mod.rs:30-32`:

```rust
pub use nets::{
    Criterion, MellStructure, NetError, Parts, Placed, Placement, ProofStructure, VertexId,
};
```

`surface.md` lists all eight under two paths (lines 23, 44, 48, 56-58, 60 and 77). The command uses `deduxit::linear::{…ProofStructure…}` beside `deduxit::linear::nets::cut::eliminate` (`cli/src/eliminate.rs:494`) and `deduxit::linear::nets::exist` (`cli/src/show.rs:1597`). The crate's doc example of `eliminate` (`nets/cut/mod.rs:233-236`) imports from both.

**Change.** Give each type one path, `linear::nets::…`: `nets` is a public module anyway and holds `exist`, `Scratch` and `cut`. Drop the re-export from `linear`, and say in the changelog's migration note which path to write.

**Value.** A library user and a reader of rustdoc or of an error message see one name for each net type, and all of the nets API under one module.

**Net.** `cargo test --workspace` (the doc examples), the bindings' build, `breaks 0.8.0`, and the manual's and the guide's names (`cli/tests/readme.rs` checks every name the guide uses). No behaviour changes.

## F-3-09

- dimension: api
- criterion: C-NEWTYPE
- place: core/src/linear/nets/mod.rs:275
- kind: API break
- cost: small
- candidate: no

**Claim.** `NetError::NoVertex` names its vertex as a bare `u32`, while every other variant names vertices as `VertexId`.

**Witness.** In `nets/mod.rs:275-280`:

```rust
    NoVertex {
        /// The vertex named.
        vertex: u32,
        /// How many vertices the structure has.
        vertices: u32,
    },
```

The other variants use `VertexId`: `NotLiteral { vertex: VertexId }`, `LinkedTwice { vertex: VertexId }` and `Unlinked { vertex: VertexId }`. The two constructors convert back with `v.get()` (`nets/mod.rs:1086`, `mell/mod.rs:470`).

**Change.** Make `vertex` a `VertexId`. A vertex outside the structure is still a vertex id, which `describe` already prints without a formula. Keep `vertices` as a count.

**Value.** A library user who matches on the error gets the same id type everywhere and can pass it back to `partner` or `occurrence` without wrapping it again.

**Net.** The `validation` test in `nets/mod.rs` and the JSON form of `NetError` (`serialize/errors.rs` writes the number either way, so the wire form is unchanged; check `wire-schema`). No checker is touched.

## F-3-10

- dimension: performance
- criterion: P4
- place: core/src/linear/nets/essential.rs:131
- kind: refactor
- cost: small
- candidate: yes

**Claim.** Several long net entry points keep their whole body generic over the caller's stop closure, against the crate's rule that a long body takes `&mut dyn FnMut(Progress) -> bool` behind a one-line shim.

**Witness.** `nets/essential.rs:131`, a body of 133 lines (`clippy.md` lists it among the functions over 100 lines):

```rust
    pub fn is_essential(&self, mut stop: impl FnMut(Progress) -> bool) -> Result<(), NetError> {
```

The same holds for:
- `ProofStructure::from_proof` (`nets/mod.rs:1011`, about 60 lines);
- `ProofStructure::is_correct` (`:1384`, 44 lines);
- `ProofStructure::sequentialize` (`sequentialize.rs:80`);
- `MellStructure::from_proof` (`mell/desequentialize.rs:97`);
- `mell::graph::Graph::correct` (`mell/graph.rs:451`), which `MellStructure::is_correct` passes its generic stop to;
- `cut::eliminate` (`cut/mod.rs:259`).

They are called with distinct closures: `|_| stop()` (`cli/src/show.rs:1474`), `never` (`web/src/calls.rs:1543`), `&mut stop` (`export/svg/mod.rs:1611`) and `|_| false` throughout. The rule is in `core.md:327-336`: such shims took 129 KB off the web module.

**Change.** Make each public `impl FnMut` entry a one-line shim that passes `&mut stop` to a private body taking `&mut dyn FnMut(Progress) -> bool`, as `ordinary::decide_within` does.

**Value.** Every visitor of the web page downloads fewer bytes, if measurement shows copies.

**Net.** To measure: `twiggy` or `cargo llvm-lines` on the `deduxit-web` module for these symbols' copies, and the budget check's gzip bytes before and after. Nothing on the net engine's hot path changes: `is_acyclic` and `acyclic` already take `dyn`. The criterion's tests keep the verdicts.

## F-3-11

- dimension: architecture
- criterion: E5
- place: core/src/linear/nets/cut/mell.rs:209
- kind: refactor
- cost: medium
- candidate: no

**Claim.** Seven functions of the unit run past 100 lines, each with visible seams. The longest are the MELL step of cut elimination (154 lines, one arm per case) and the MELL sequentialization's stage (158 lines).

**Witness.** `clippy.md` lists these over 100 lines:
- `cut/mell.rs:209` (154);
- `mell/sequentialize.rs:256` (158);
- `nets/mod.rs:530` (136);
- `mell/graph.rs:188` (127);
- `search/net.rs:418` (123);
- `search/essential.rs:285` (118);
- `nets/essential.rs:131` (118).

The seams of `reduce` (`cut/mell.rs:228-357`) are the arms `Kind::Atom | Kind::DualAtom`, `Kind::Tensor | Kind::Par` and `Kind::Bang | Kind::Quest`, the last with the sub-arms `[]` (weakening), `[c] if …` (dereliction), `[c]` (box into box) and `[c, ..]` (contraction). `stage` (`mell/sequentialize.rs:256-425`) opens conclusions (`:265-312`), then joins parts with Mix (`:315-340`), then an axiom, a promotion or a split.

**Change.** Split along those seams:
- in `reduce`, one method of `Work` per `Case` (`axiom`, `tensor_par`, `weakening`, `dereliction`, `box_into_box`, `contraction`);
- in `stage`, `open` and `split`, with the Mix partition shared (F-3-06);
- in `Symmetry::new`, its anchors, literal classes and compound classes;
- in `is_essential`, its cycle search and its dominator search.

**Value.** A reader of the code can hold each cut case or each phase of a stage in mind, and match it to the case list in `nets/cut/mod.rs`'s docs.

**Net.** The cut elimination tests (every step's net sequentialized and checked), `classic_nets`, the round trips, and for `Symmetry::new` the net target set's counters (`nodes`, `links` and `tests` against `net-47.csv`, which must not move). This touches cut elimination and a search; both are checked after each step.

## F-3-12

- dimension: idiom
- criterion: E7
- place: core/src/linear/nets/mod.rs:1278
- kind: refactor
- cost: small
- candidate: no

**Claim.** The nets module holds 36 of the crate's `must_use_candidate` sites, among them predicates such as `same_component`, `is_complete` and `is_cut_free`. A dropped call to any of them is a silent bug.

**Witness.** `clippy.md` gives the `clippy::must_use_candidate` count for the `linear::nets` area as 36. One of them, at `nets/mod.rs:1278`:

```rust
    pub fn same_component(&self, x: VertexId, y: VertexId) -> bool {
```

Others: `is_complete` (`:1207`), `is_cut_free` (`:1177`), `partner` (`:1183`), `vertex` (`:1141`), and `MellStructure::{len, is_empty, partner, hangs_on}`.

**Change.** Add `#[must_use]` to the 36 sites in `linear::nets`, as `core.md` asks (pedantic cleared area by area).

**Value.** A library user who writes `net.same_component(x, y);` as a statement gets a warning instead of a silent no-op.

**Net.** Clippy with `--deny warnings` and `cargo test --workspace`; no behaviour changes.

## F-3-13

- dimension: idiom
- criterion: E6
- place: core/src/linear/search/essential.rs:337
- kind: refactor
- cost: small
- candidate: no

**Claim.** The set-up of the essential engine's closure writes the same row-filling loop twice, once for input occurrences from the last up and once for outputs from the first down.

**Witness.** From `search/essential.rs:337-349`, which `:350-362` repeats with only the first line changed to `for o in forest.ids().filter(|&o| position(o) == Side::Output) {` (the clone detector's `:337` against `:350`):

```rust
        for o in forest.ids().rev().filter(|&o| position(o) == Side::Input) {
            if poll(stop) {
                return Err(Reason::Stopped);
            }
            let (u, row) = (o.index(), o.index() * words);
            set(&mut reach[row..row + words], u);
            for s in fixed_successors(forest, reading, o) {
                let from = s.index() * words;
                for w in 0..words {
                    reach[row + w] |= reach[from + w];
                }
            }
        }
```

**Change.** Use one loop over `forest.ids().rev().filter(input).chain(forest.ids().filter(output))`, keeping the comment that explains the order.

**Value.** A reader of the code sees the one fill and why its order works.

**Net.** `search/essential.rs`' tests (`agrees_with_the_two_sided_engine`, `unlinking_restores_the_closure`, `the_closure_is_charged`). The essential engine is not in the net target set, so check its counters on the tests' sequents. The search order is unchanged.

## F-3-14

- dimension: idiom
- criterion: E3
- place: core/src/linear/nets/mod.rs:2171
- kind: refactor
- cost: small
- candidate: no

**Claim.** In `nets/mod.rs` a production `impl` comes after the test module. The test module, already behind `cfg(all(test, feature = "parse"))`, repeats `#[cfg(feature = "parse")]` on 11 of its items, and `sequentialize.rs`' test module does the same on 4.

**Witness.** From `nets/mod.rs:1598-1603`:

```rust
#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    #[cfg(feature = "parse")]
    use crate::linear::fragment::Mode;
```

The inner attribute repeats at `nets/mod.rs` lines 1601, 1603, 1605, 1645, 1653, 1682, 1787, 1942, 2000, 2059 and 2099, and at `nets/sequentialize.rs` lines 357, 362, 368 and 437. After the test module, `nets/mod.rs:2171` adds `impl Display for In<'_, crate::linear::MellStructure>`, away from its sibling at `:1590`.

**Change.** Remove the 15 redundant attributes, and move the `In<'_, MellStructure>` impl next to the `In<'_, ProofStructure>` one, before the test module.

**Value.** A reader of the code finds both `In` impls together, and is not left wondering why a test needs a feature its module already requires.

**Net.** Both `cargo hack` runs (each feature, and pairs) and `cargo test --workspace`; no behaviour changes.

## F-3-15

- dimension: idiom
- criterion: E3
- place: core/src/linear/search/net.rs:274
- kind: refactor
- cost: small
- candidate: no

**Claim.** `counts_admit` uses an `if` expression as the condition of another `if`, which reads as a typo.

**Witness.** From `search/net.rs:274-280`:

```rust
    if if mix {
        conclusions < needed
    } else {
        conclusions != needed
    } {
        return false;
    }
```

**Change.** Bind it first: `let admitted = if mix { conclusions >= needed } else { conclusions == needed }; if !admitted { return false; }`.

**Value.** A reader of the code sees the count equation, which `core-nets.md` says must not be relaxed, at a glance.

**Net.** The net engine's tests and the net target set's counters: the preprocessing is unchanged, so no row moves.
