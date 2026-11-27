# Step 87c2: one connection search, shared builders of LK and LJ (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-82-141614`, from `plan/87-refactor.md` (87c2, wave C,
panel) and the planning session's briefs `start-87-common.md` and
`start-87c2.md`, on cores 10 to 12 (the panel's on 2 and 3, the
planning session's, which it lent), beside 87b1, 87c1 and 87c3. The
base was `qpuqvuxv`, rebased at the end onto `nskzrsnk` (87c1 on
`main`, without conflicts; the pin and the net ran again there).
The panel and the reviewer read the chain before that rebase. Behaviour is kept: no verdict, counter, refusal or
derivation changes, and nothing public changes.

## Outcome, per finding

The chain, oldest first:

| change | commit | lines (+/−) |
|---|---|---|
| `xzpwnpsy` | Share one connection search between the classical and the intuitionistic prover | 454 / 1 050 |
| `plkpznuz` | Keep one Budget and one union for the connection module's builds | 9 / 64 |
| `wnqrnvwv` | Share the connection provers' small helpers: the next eigenvariable, the statistics, a build's unknown | 57 / 67 |
| `ykvzxnkp` | Share the mechanics of the builders of LK and LJ, each keeping its charge | 295 / 300 |
| `trnmqnmv` | Share the branches of the connection read-back and Herbrand's construction | 336 / 340 |

In all, 15 files, 1 151 lines added and 1 821 removed (the rules
included).

- **F-arch-24, F-4c-02** (the intuitionistic search a copy of the
  classical one): `connection::prove::search<C: Calculus>` is the one
  search, monomorphised for `Classical` and for `Prefixed`
  (`intuitionistic/prove.rs`, 1 078 lines before, 302 now). The hooks
  sit where the copy had its changes, each called in the same order:
  `work`, `key`, `connect`, `renames`, `mark`/`restore` (an associated
  type, `()` for classical logic: see the panel), `longest`,
  `held_bytes`, `round`, `proved`, and the constant `CLASSICAL` for the
  ground shortcuts, the settled cut and `Exhausted`.
- **F-4c-08** (small helpers two to four times): `matrix::Budget` and
  `matrix::union` serve the intuitionistic prefixes too;
  `derivation::next_eigen` replaces the scans of both read-backs and
  Herbrand; `Counts::statistics` and `prove::build_ended` (a stopped or
  memory-bound build as an unknown) replace the two provers' copies.
- **F-4b-01** (six builders of a derivation of LK or LJ): `derivation.rs`
  has `ask_read_back` (the stop before an inference), `add` (the `u32`
  refusal and the push), `difference`, and the trait `Builder`, whose
  default `weaken_to`, `contract_to` and `in_order` go through each
  builder's `step` (LJT's planted faults a hook). LJT, the SAT read-back
  and the intuitionistic connection read-back implement it; Herbrand
  uses `add` and `ask_read_back`, the classical read-back and the
  embedding's `Out` `add` alone (the first asks its stop in `account`
  beside its arena's charge, the second asks none per push). **The one charge rule is left for wave D (87d3,
  measured)**, as the planning session decided: each builder keeps its
  own charge, so every refusal stays where it was. The six charges as
  they are, also in `core-ordinary.md`:
  - LJT (`ljt.rs`, `Lj::push`) and the SAT read-back
    (`sat/read_back.rs`, `Lj::push`): 4 bytes per formula and premise
    id to their `Account`, the list's growth by capacity after the push;
    LJT also its drawn estimate against `derivation_bytes` where
    preferred;
  - the intuitionistic connection read-back (`Emit::push`): the
    `Inference` and 4 bytes per id against `limits.memory_bytes`;
  - the classical connection read-back (`ReadBack::push`): the same,
    plus its arena (`account`, `arena_bytes`);
  - Herbrand (`Build::push`): twice the `Inference` and each vector's
    allocator block (rounded to 16 bytes, plus 16), plus its arena;
  - the embedding's read-back (`Out::push`): the `Inference` and 4
    bytes per member and premise.
- **F-4c-03** (Herbrand's builder repeats the classical read-back's):
  `ordinary/branch.rs` holds the branch, the step, the frames, `sides`,
  `position`, `replaced`, `closes_a_premise` and `close` (`⊤R`, `⊥L`,
  or an axiom with its weakenings), through the traits `Placed` (an
  item) and `Emits` (the builder's own push). The audit's file split of
  `herbrand.rs` (F-4c-11, the builder moving into the shared module)
  is not this step's.

## Departures

- **Each builder keeps its derive loop, its rule order and its oracle**
  (departs from the plan's "one derivation builder"): the classical
  read-back and Herbrand charge the frames waiting differently (by
  sides, by `Frame::bytes`), and their orders of rules and witnesses
  differ (the classical read-back's `nesting` rule, Herbrand's Skolem
  terms). Merging them would move refusals; what is shared is the
  mechanics both run through.
- **The SAT read-back keeps its own `weaken`** (`without` keeps the
  order of the remaining formulas, which the derivation drawn shows);
  it shares `in_order`.
- **The unary and binary rules of the classical read-back and Herbrand
  stay apart**: the first makes an item per occurrence of the matrix
  (`child`), the second a plain item.
- **No changelog line**: nothing a user sees changed.

## The net

- The connection pin (`bench/connection-targets.sh`, on cores 10 to 12
  with the planning session's leave): `connection-87c2-before.csv` at
  the base, `connection-87c2-search.csv` after the shared search,
  `connection-87c2-final.csv` at the head after the panel's fix and the
  rebase onto 87c1 (the shared arena, `nskzrsnk`). Each
  against `connection-87a.csv`: no verdict differs, and of the 1 312
  problems decided alike none differs in `nodes` or `work`.
- `gate` passed at the head before the rebase (clippy, the tests, both
  `cargo hack` runs, the ratchet). The ordinary journeys:
  `ordinary-connection-classical` 9 183 153 instructions (−1.38 %
  against the ceiling), `ordinary-connection-intuitionistic` 7 232 271
  (+0.34 %), `ordinary-ljt` 26 130 958 (+0.30 %),
  `ordinary-sat-intuitionistic` 28 188 656 (+1.21 %), `ordinary-pigeons`
  −0.27 %, the others within 0.7 %; every one under its ceiling.
- `breaks 0.8.0`: only what earlier steps changed (the witness faults'
  fields of step 81, System F's variants of step 84, `read_own`
  deprecated); nothing of this step, whose items are all private.
- `nix flake check` at the commit of this report, under the machine
  lock (`--max-jobs 2 --cores 3`): its result is the last line of this
  report.

## The panel

On cores 2 and 3, the brief and the chain as above.

- **The argument** (Fable 5.2, high): one refutation. The classical
  search's choice point had grown 4 bytes (`Marks::calculus: u32`,
  stored in every `Choice`), and the memory poll counts the choice
  points by capacity, so the classical search refused earlier:
  pigeonhole 4 into 4 at a memory bound of 16 464 bytes was "not valid"
  on the base and `MemoryLimit` at 2 048 steps on the head; the
  transitivity sequent at path length 5 and 13 584 bytes likewise.
  Fixed in `xzpwnpsy`: the mark is `Calculus::Mark`, `()` for classical
  logic, so `Choice` has its old 68 bytes (the intuitionistic one its
  old 72), and a `const` assertion holds the classical marks to the
  terms' mark and six counts. The member's probe rebuilt on the fix gives the base's
  results at both thresholds and one byte below them. Everything else
  it checked line by line equivalent: both instantiations of the search
  against the base's two searches, the builders' stop, charge and
  refusal order, the default methods against each copy, `branch.rs`
  against both copies.
- **The integers** (Sonnet 5.5, high): the same refutation by another
  input (Pelletier 34 at `--memory-limit 20480`: 92 160 steps against
  the base's 177 152), the only line that differed in a scan of 97
  bounds, all equal after the fix. The thresholds of 16 builder and
  sequent pairs (LJT in two logics, the SAT read-back, both connection
  read-backs, the embedding, Herbrand) equal to the byte; 6 454
  differential runs over memory and work bounds with no line differing;
  the ordinary tests on wasm32-wasip1 pass. `add` refuses at the same
  count as each copy; `next_eigen` and each caller's handling at
  `u32::MAX` are as before.
- **The counterexamples** (Opus 5.5, high): the same refutation a third
  time, with verdicts lost (`|- (((c -> b) -> ~a) <-> ((d /\ d) <-> b))
  \/ (((c -> b) -> ~a) <-> ~((d /\ d) <-> b))` at `--memory-limit 27000`:
  valid on the base, unknown at the memory bound on the first head); all
  four of its witnesses byte for byte equal after the fix. Against the
  fixed head, built from the tree beside a base build, outputs compared
  byte for byte (standard output, standard error, exit status, with
  `--deterministic --timeout none`):
  - 21 502 propositional runs (827 inputs: random, valid shapes, edge
    cases of `close`) over both connection provers, LJT and the SAT
    method in intuitionistic and minimal logic, the embedding in two
    logics and the defaults, with work, memory and path bounds drawn per
    case: none differs (5 timed out in the base);
  - 9 306 first-order runs (517 inputs) over both connection provers,
    `herbrand`, `resolve`, grounding, the embedding and the default:
    none differs (15 timed out);
  - 2 844 runs on ILTP (the 274 propositional problems, 300 first-order
    ones): none differs; three that met the 10 s bound on the head alone
    are equal at 60 to 90 s;
  - threshold sweeps (the base's least passing bound, then about 30
    points around and below it) for every engine above, memory and
    work: 0 of about 9 400 points differ; verdict sweeps every 250 bytes
    from 6 000 to 40 000 on 57 inputs: none differs;
  - its own truth-table reference on 680 inputs (4 076 runs): no
    classical verdict against the table, no intuitionistic or minimal
    "valid" that is classically invalid, no two methods at odds.

## The reviewer

fresh-reviewer (Fable 5.2, medium) read the chain against the brief and
the findings: nothing breaks behaviour or the public API; the three
bullets of the rules file are true of the code; the pin's three
comparisons and the line counts check out. Four text findings, all
fixed: two doc comments told history (`Calculus`'s "where its copy of
the search had it", `Builder`'s "as it did … waits for a measurement"),
now saying why; this report had the diffstat's figure for
`intuitionistic/prove.rs`'s length (1 078 lines before, 302 now), and
said the classical read-back and `Out` use `ask_read_back`, which only
Herbrand does.

## What the next steps must know

- 87d3 decides the one charge rule, measured: the six charges are
  listed above and in `core-ordinary.md`. Unifying them moves refusals,
  which the integers member's threshold scans
  (`scratchpad/panel-integers/thresh.py` of this session; the method,
  not the file, is what lasts) would show.
- A type stored in the search's choice points changes the memory poll:
  `Calculus::Mark` is `()` for classical logic for that reason.
- The pin covers the searches at the default bounds only; a change of
  either search should also compare memory thresholds on a few inputs.

## The full check

`nix flake check --keep-going` at `xyospxwn` on `nskzrsnk`, under the
machine lock: all checks passed (x86_64-linux). Two earlier runs at
that head found what the gate does not run: `mod branch;` out of
rustfmt's order (treefmt), and two modules' doc links to `Refusal`
whose import the chain had dropped (doc, doc-private); both fixed in
the commits that caused them. `main` has since gained 87b1 and 87c3,
which touch none of this chain's files but the changelog, which this
step does not change.
