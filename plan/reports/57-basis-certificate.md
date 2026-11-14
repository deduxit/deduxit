# Step 57: a checked basis for non-coverability

## Session 57: the basis certificate

Session 1 of `plan/57-basis-certificate.md` (Opus 5.5 at `high`,
2026-11-14, cores 7-9), in the jj workspace
`../deduxit-wt-step-58a-610ed3`, started on `ympppsxt` and rebased onto
`main@origin` (`rvuqtyyl`, 54s-c3's classical default) at the end.
Paused once for the app's restart and resumed from the checklist this
report held.

The Horn engine's "not provable" in affine mode, where the backward
search saturates, now carries the basis it saturated
(`Refutation::Basis`). `search/basis.rs` checks it without the search,
and `Disproof::check` and the front door run that check. The search is
unchanged: both target sets equal 47's. Nothing is pushed.

| commit | what |
|---|---|
| `ytxszuuw` | `Refutation::Basis` (`Basis`, `Marking`), its check `search/basis.rs`, the engine's basis, `conclude`'s last resort, wire level 11, the JSON form, the Rocq refusal, tests, the manual, the guide, the changelog, the rules |
| `ozmvmkxm` | `later.md`: the Rocq lemma kept for later |
| `myrvqpyz` | `bench/targets/57.csv`, `net-57.csv` |
| (this one) | this report |

### The certificate's form

`Basis { body, classes, markings }` (both structs `#[non_exhaustive]`
with `new`):
- `body`, the sign of the goal's body literals. The check reads the goal
  as a net again under that sign.
- `classes`, the clauses used once grouped into classes of equal
  clauses, each class an occurrence list. A class is a place whose
  tickets are its unused clauses.
- `markings`, the basis: each marking is its atoms with counts, sorted
  by atom, and its tickets as (class index, count), sorted by class.

Its JSON form is `{"kind": "basis", "body": "+"|"-", "classes":
[[occ…]…], "markings": [{"atoms": [[atom, n]…], "tickets": [[class,
n]…]}…]}`, at wire level 11 (`wire::BASIS`). The reader refuses an
occurrence outside the forest or an atom outside the sequent. A reader of
level 10 refuses it by its level. `core/tests/wire/level-11.txt` holds
the disproof and outcome lines of `a, a -o b, !(a * b -o a * b * c) |-
c`, whose basis is `{c}`, `{a, b}` and `{2a + a ticket}`. That sequent
was chosen so that the state equation is feasible, the classical reading
valid and the counts silent, so only the backward search refutes it.

The engine gives the basis only where the state equation gave no
certificate (`equation.certificate().or_else(basis)`), and the front door
takes it as the refutation of last resort: only where the counts and the
classical refuter find none, as for the inverse method's `Saturated`.
It is checked before the verdict unless `Options::check` is off. The
stop or the work bound gives `Unknown`, and a failed check gives
`Error::Refutation`. The Rocq writer refuses it
(`Unsupported::Basis`): deduxit's library has no theorem for it yet.

### The check and its cost

`search/basis.rs` reads the goal as a net by its own code (`read`,
`counted`), with places = atoms + classes. It then checks:
1. the classes are exactly the clauses used once, with equal arcs in
   each class;
2. every marking is well formed (atoms and tickets strictly increasing,
   counts above zero, in range), checked before any place is computed;
3. some marking is at most the target (`Missed` otherwise);
4. no marking is at most the initial marking (`Reached` otherwise);
5. for each marking `m` and each live clause `t` with an output on `m`'s
   places, some marking is at most `max(m − out(t), 0) + in(t)`
   (`Open` otherwise). The live clauses are those whose inputs lie in
   the least set of places the initial marking reaches.

The argument is in the module's docs. A cut-free proof is a lossy firing
sequence by the weight count (no Mix, no goal weakened). Every marking
of that sequence, read from the end, lies in the basis's upward closure.
So the initial marking would too, which step 4 excludes. Ordered and
intuitionistic proofs are classical ones, Mix is admissible with
weakening, and the three commutative logics have cut elimination, so the
check does not read the mode.

Cost: for each marking and each live clause with an output on it, one
probe and one scan of the basis, a sum filter first. That is
O(|basis|² · |clauses| · places) at worst, quadratic in the basis as the
prompt asks. The check asks its stop once per probe. Inside the search it
runs under the caller's stop and `Limits::work`; `Disproof::check` has
no stop, as for `Saturated`. Measured by the integers member: a valid
redundant basis read from a file took 0.21 s at 0.69 MB, 0.85 s at
1.4 MB and 3.4 s at 2.8 MB, quadratic as stated. The check's time on
the engine's own bases (qcover's problems, say) was not measured
(open).

### The integers

- Counts are `u64`. A marking's count is a `u32`, and a clause's weight
  counts literals of one clause, below the forest's 2³² occurrences. So
  `max(m − out, 0) + in` stays below 2³³.
- The places, `atoms + classes`, are a `checked_add` in `usize`, refused
  as `Mismatch`.
- A marking's sum, used only as a filter, saturates.
- A ticket's class index is checked against the classes before any place
  is computed from it. The integers member found that it was not: on
  wasm32, `atoms + k as usize` wrapped for `k = u32::MAX` before the range
  check. That is fixed, and `(u32::MAX, 1)` is a test case giving
  `Mismatch`, passing on native and on wasm32-wasip1.

### The panel and the reviewer

- **panel-integers** (Sonnet 5.5, high): one finding, CONFIRMED and
  fixed, above. It also measured the quadratic cost on large redundant
  bases. That cost is documented (core-search.md) and is no fault.
- **panel-argument** (Fable 5.2, high): no refutation. Its notes were
  that the docs did not state the ordered modes, did not argue the appeal
  to cut elimination, and did not restate the weight count for the
  check's own reading, and that the shape reader follows
  `Program::read_with` arm for arm, so it is not logically independent
  there. The module's docs and core-search.md now say each of these.
- **panel-counterexamples** (Opus 5.5, high): no counterexample. It
  wrote two references of its own, a net decider and a one-sided sequent
  prover in affine Mix, which agree with each other. It generated 2 650
  Horn goals (both body signs, `?` and once clauses, `⊥` and `1` factors,
  goals off the roots, two-sided texts) and ran 42 400 engine runs in
  seven modes. Every one of the 720 engine bases and 9 468 bases of its
  own in nine modes was accepted, each for an uncoverable, unproved goal.
  Of 59 901 mutants, 14 831 were accepted, all for uncoverable, unproved
  goals. Of 540 285 transplants to coverable goals, none was accepted.
  The JSON path with the mode changed to seven names was accepted, and
  with a member appended it was refused.
- **fresh-reviewer** (Fable 5.2, medium), two findings, both fixed in
  `ytxszuuw`:
  - A goal may name a member twice (`Goal::new`, "with repeats"). The
    check keyed the clauses used once by occurrence, as a set, so a goal
    naming `a -o b` twice got its sound basis refused, and `conclude`
    answered `Error::Refutation(Mismatch)` where `Unprovable` was the
    answer before. The reviewer marked it PLAUSIBLE. The new test
    `a_member_named_twice_is_two_clauses` failed so before the fix. Now
    the classes are matched as a multiset. Soundness was not at stake,
    since a sound basis was refused and no unsound one accepted.
  - `check`'s doc said it reports one unit per count compared, but it
    counted one per marking looked at. It now counts what the doc says:
    one for the clause, one per marking looked at, and one per count of
    each marking compared, so `Limits::work` bounds the comparisons too.
  - Nothing found against soundness, independence, the integers, the
    search unchanged, the JSON form or the text.

### The target sets and the families

- `bench/targets/57.csv` against `47.csv`: 0 verdicts and 0 counters
  differ over 145 rows (`summary --against … --counters`).
- `bench/targets/net-57.csv` against `net-47.csv`: 0 differ over 64
  rows.
- Measured before the rebase. What the rebase brought is 54s-c3's
  ordinary logic, whose own target sets equal 47's, and this change does
  not touch the search.
- Run on cores 7-9 by copies of the two scripts that differ only in the
  cores, the unit's name and the starting directory (the scripts pin
  2-3, which timed runs keep).
- Every family at 5 s: no `MISMATCH`, no verdicts disagreeing (195 runs).
  The families have no affine mode, so they do not reach the basis.

### The checks

- Workspace Clippy clean and every test binary passing before the
  commit; after the panel's fix, `search::basis`'s tests on native and
  on wasm32-wasip1, and the gate below for the rest.
- The ratchet: `read-json` came out 2.03 % over its ceiling, the only
  journey past the tolerance. It reads a sequent's JSON, code this change
  does not touch. From binaries at paths of one length (the path alone
  moved the count 0.5 %), the base counted 12 997 858 and the head
  13 059 169. Under `callgrind_annotate` every deduxit function ran the
  same instructions; only libc's `memcmp` and `memcpy` moved. The two
  hottest functions keep their sizes and moved from 16 and 32 bytes past
  a 64-byte line to 48, and the head was 4 % slower pinned to core 9
  (686 against 660 µs, the same with a padded heap): a layout move, not a
  regression by itself (bench.md, "What a move means"). 54s-c3 had raised
  the same ceiling to 13 059 079 for the same reason (`wrnltouo`, on
  `main@origin`), so after the rebase my own raise was abandoned.
- The gate, on the rebased chain: passed (Clippy, the tests, both `cargo hack` runs, the ratchet with `read-json` 0.37 % under its ceiling).
- `nix flake check` at the last code commit, under the machine lock: all checks passed (Kani's harnesses, the ratchet and the Rocq certificates among them), 2026-11-15 00:06.

### Departures

None from the prompt. Two choices it leaves open:
- The state equation's certificate is preferred over the basis where
  both exist. It is smaller, and the Rocq library certifies it.
- The check reads no mode (the argument above), where the inverse
  method's `Saturated` is checked per mode.

### What the next sessions must know

- **Wire level 11 is claimed twice.** 54s-c3's chain not yet on
  `main@origin` (`rpotllmp..vumlvmvs`) takes level 11 for its `SEARCH`
  form, with its own `core/tests/wire/level-11.txt`, as this step takes
  it for `BASIS`. Whichever lands second moves to 12: `wire.rs`'s table,
  `LEVEL` and its constant, `level_of`, the stability test's case and
  file, the newer version in the doc test, `web/tests/ordinary.rs`'s
  about level, the changelog's line.

- The Rocq lemma for the basis is in `later.md` ("Follow-ups: certified
  refutations"). Until it exists, `--kernel deduxit` refuses a basis
  (`Unsupported::Basis`).
- `basis.rs`'s shape reader follows `Program::read_with` arm for arm. A
  change to what the Horn engine reads as a program must change both.
  A misreading both share would pass the check, so the independent part
  is the closure test.
- A basis read from a file can make `Disproof::check` take seconds
  (quadratic, no stop), as a saturated set can. A front end that checks
  untrusted disproofs needs a size limit of its own.
- The check's time on qcover's engine bases is not measured.
