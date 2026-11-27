# Step 87c4: the nets' graph kernel and sequentialization (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-step-82-141614`, from `plan/87-refactor.md` (87c4, wave C,
panel: the correctness criterion) and the planning session's briefs
`start-87-common.md` and `start-87c4.md`, on cores 10 to 12, the net
target sets on core 2 (the planning session's slot). Paused once
(20:35, the author's machine) and resumed from the checklist. The base
was `xyospxwn` (87c2), rebased at the end onto `main@origin`
(`tttrqxsq`, 87c7 and 87c8 among others, which touch no file of this
step) without a conflict. Behaviour is kept and nothing public changes.

## Outcome, per finding

| change | commit | lines (+/−) |
|---|---|---|
| `slorupzk` | Share the correctness criterion's graph kernel between the structures with and without boxes | 473 / 531 |
| `nvwmyuyz` | Share the sequentializations' proof assembly and Mix partition between the structures with and without boxes | 299 / 277 |
| `moytklwl` | Share the nets' verdict line, cut partner and axiom relinking between the structures | 57 / 50 |
| `mpyuqqyo` | Fill the essential engine's closure rows in one loop over the inputs then the outputs | 3 / 14 |
| `oxoouqzz` | Rules: the nets' shared graph kernel and proof assembly | 27 / 6 |

`graph.rs` went from 799 lines to 595, `mell/graph.rs` from 770 to 570,
`sequentialize.rs` from 507 to 416, `mell/sequentialize.rs` from 426 to
316; `kernel.rs` (344) and `assemble.rs` (220) are new, so the line count
is about even: the copies are one, with docs and the traits in place of
the second.

- **F-3-02** (the criterion's graph kernel twice): `nets/kernel.rs` holds
  the CSR search with Tarjan's bridges, Yeo's deletion rounds, the
  isolation of one switching cycle and the one public `Scratch`
  (bitsets, re-exported at `nets::Scratch` as before), generic over
  `Csr` and `Rounds::deciding`. MLL's view `Switched` reads the `⅋`s
  from the forest, so MLL's layout is unchanged (the verdict's point 1);
  MELL's graph reads its `switched` flags. `cycle` clears `removed`
  when the stop fires for both (point 2). `parts` stays per structure
  (point 3).
- **F-3-06** (the sequentializations' assembly twice): `nets/assemble.rs`
  holds the steps (`Rule`, `Open`), the loop (`Assemble::run`, each
  structure giving its `stage`) and the Mix partition (`Parts`). 87b1
  gave both partitions one shape, which the verdict's correction asked
  for first, so it is shared too.
- **F-3-07** (three small repeats): `write_verdict` for both `Display`s
  (MELL never reaches the planar arm: `from_parts` refuses an ordered
  criterion), `cut_partner_in`, and `relink`, the axiom case's four
  lines, generic over the link's id type; MELL's weakenings and erasures
  stay in `cut/mell.rs` (the verdict's correction).
- **F-3-13** (the essential set-up's two loops): one loop over
  `inputs.chain(outputs)`, the same rows in the same order and the same
  polls.

## The ratchet

`search-wide-m2-256` (the net engine's journey, which ends by
sequentializing the net it found) is the hot path the verdict named.
Measured on cores 10 to 12, `--only search-wide-m2-256`:

| build | instructions | against the base |
|---|--:|--:|
| base `xyospxwn` | 262 320 145 | |
| the kernel, first version | 266 100 560 | +1.44 % |
| rounds' search out of line (`search_every`) | 263 037 435 | +0.27 % |
| `discover` takes the graph, not its `start` slice | 262 722 814 | +0.15 % |
| with the shared assembly | 264 484 615 | +0.83 % |
| MLL's `stage` `#[inline(always)]` into the loop | 260 344 989 | −0.75 % |

The causes, from callgrind per function: inlined into the rounds, the
search cost 2.7 M more than the base's separate `Graph::search` and
`acyclic` together; `discover` passed two registers where it passed one;
and the stage, compiled apart from the shared loop, cost 1.7 M more than
inlined into the old loop. Each fix is commented at its site and in
`core-nets.md`. The ceiling was not touched; at the head every other
journey equals the base's (the base's own whole ratchet against the
head's gate). Trying a deletion test per structure over a shared
`bridged` made it worse (263.6 M) and was dropped.

## The net

- The net target set (`bench/net-targets.sh`, 64 problems, core 2):
  `net-87c4-final.csv` at the rebased head against `net-86.csv`: no
  verdict differs, and none of the 64 rows differs in `nodes`, `links`,
  `tests` or any other counter. Its times read slower than `net-86`'s
  at first; the base run back to back on the same core
  (`net-87c4-base.csv`) is as slow or slower on every group but three
  single rows, so that was the machine's state.
- `gate` passed at the rebased head (clippy, the tests, both `cargo
  hack` runs, the ratchet): `search-wide-m2-256` 260 344 989 (−0.02 %
  against its ceiling), every other journey within its tolerance and as
  at the base.
- `breaks 0.8.0`: only what earlier steps changed (the witness faults'
  fields, System F's variants, `read_own` deprecated); nothing of the
  nets, whose new items are crate-private and whose `Scratch` keeps its
  path.
- `nix flake check` at the commit of this report, under the machine
  lock: its result is the last section of this report.

## The panel

On cores 10 to 12, the brief `scratchpad/panel-87c4.md` of this session.

- **The argument** (Fable 5.2, high): no refutation. It checked the
  kernel against both copies line by line (the deletion test in each
  structure, `remaining` from the bitset, `cycle`'s edge order and its
  clearing, every caller's state of `removed` and `deleted`), the
  assembly's node order against both loops, `Parts` against both
  partitions, the helpers and the chained set-up. The differences it
  found are not observable: MELL's `removed` now cleared on the stop and
  at the end of `cycle` (its scratch is per call), and MLL's
  `debug_assert` in `is_bridge` now on MELL too, which holds at every
  MELL call (the debug-built tests pass).
- **The integers** (Sonnet 5.5, high): no refutation. A differential of
  12.2 MB of output per side (1 500 MLL and MELL structures across the
  bitset's word boundaries, 660 MELL proofs and their mutants, the stop
  at every poll) is byte for byte equal; the essential set-up's polls
  equal on chains across `SETUP_ROWS`; MELL's criterion now holds 27.2
  bytes per vertex where it held 29.0 (the bitsets), below its estimate
  `SCRATCH_BYTES` of 28, which the base's exceeded; the nets' tests pass
  on wasm32-wasip1.
- **The counterexamples** (Opus 5.5, high): no refutation. Base and head
  release builds of one test program, outputs compared byte for byte
  (about 750 MB a side): 136 464 MLL structures (with Mix, cyclic,
  incremental linking with `unlink`, a reused scratch, stops at every
  poll, sequentialization, the round trip), 112 167 of them as
  `MellStructure`s, 10 019 proofs with cuts and their mutants through
  elimination (stops swept), 28 000 MELL cases with 199 990 mutants,
  38 234 boxed structures for the in-box witnesses, and both net engines
  on 38 745 sequents with their statistics. Its own brute-force
  switching enumeration agrees with the head on 182 097 structures.

## The reviewer

fresh-reviewer (Fable 5.2, medium): nothing breaks behaviour, the public
API or the findings. Four text findings, all fixed: `core.md`'s module
list still put the Yeo test in `graph` and named neither new module;
`core-nets.md`'s sequentialization bullet still named the old `Step`;
its MLL graph bullet named `deletable` for the three-slot test, which
`Switched::deciding` reads now; and the stage's comment spoke of "the
old loop".

## What the next steps must know

- The net engine's hot path is shaped by three measured choices, each
  commented: the rounds' out-of-line `search_every`, `discover` taking
  the graph, and MLL's `stage` inlined into `Assemble::run`. A change
  there measures `search-wide-m2-256` against its base, not only the
  ceiling.
- `assemble::Rule` and `Open` are the steps both sequentializations
  leave; a new structure's sequentialization gives a `stage`.
- No changelog line: nothing a user sees changed.

## The full check

`nix flake check --keep-going` at `tuxxnwsr` on `xukolplx` (main with
87b3 and the rest of 87c7, whose changes to `nets/mod.rs`, `cut/mod.rs`,
`essential.rs` and `net.rs` rebased without a conflict), under the
machine lock: all checks passed (x86_64-linux). It passed at
`mrqtknsz` too. The net target set's last run was on `tttrqxsq`, before
those two rebases; 87c7 pinned its own changes to the same files. The run
before it found what the gate does not build: three intra-doc links in
`kernel.rs`'s module doc that the private docs could not resolve
(doc-private), fixed in `slorupzk`.
