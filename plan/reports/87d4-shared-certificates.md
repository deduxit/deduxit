# Step 87d4: certificates with shared subterms, bounded (report)

Run on 2026-11-27 by one session on Opus 5.5 at `high`, in the workspace
`../deduxit-wt-fixup-30-3e23c4`, from `plan/87-refactor.md` (87d4) and
the planning session's briefs `start-87-common.md` and `start-87d4.md`,
on cores 13 to 15, on 87c3's head `loqwlmwu`. Paused at 20:28 for the
author's machine and resumed at the planning session's word; the panel
ran again from the start.

## Outcome

| commit | change | a user sees |
|---|---|---|
| `tyryyoly` | `ordinary/rocq.rs`: `falsified` takes a limit, `falsified_bytes` estimates its text bottom up, saturating; `write_countermodel` passes `derivation_bytes` | a classical countermodel's certificate past the bound is refused before anything is written (it was written in full, doubling per level of a shared formula) |
| `rylkosmn` | `export/rocq/shared.rs` (new), `deduxit.rs`: a first walk hash-conses the formulas the term names, the second writes `(let f'N := … in` and the term with the names | kernel certificates (`--kernel deduxit`) bind shared formulas inside the term; nine snapshots change; the statement is unchanged |
| `kzuxuvxw`, `msuqwlxp` | an estimate past `isize::MAX` (no text holds more) is refused under any limit | a limit of `u64::MAX` no longer lets an exabyte text through |
| `mttlplom` | `Shared::bind` binds only the largest shared pieces, as many as make the estimated Rocq cost least, at most 1 024 | no stack overflow and no slowdown on wide formulas (below) |
| `rlllvtpm` | `write_not_valid` holds to the default `derivation_bytes` | the image's refutation's certificate is refused past 64 MiB |
| `unwmpkwk`, `kmktkouz` | the rules file corrected | |

The manual (`docs/src/rocq.md`, a console example with two `let`s), the
guide (`skills/deduxit/reference.md`), the changelog (Changed: the
`let`s; Fixed: the bound) and `.claude/rules/core-export.md` (two
bullets) are in the commits that change what they describe.

**Why a cost estimate and a cap** (the argument member's finding at
`rylkosmn`): binding every shared formula nested one `let` per piece,
and Rocq's `apply` elaborates every constructor in a context that holds
every `let` around it, so its time grew with the constructors written
times the `let`s in scope, and its stack overflowed between 3 000 and
5 000 nested `let`s. The witness: `⊢` a balanced `⊗` tree of 4 096
leaves `(aᵢ ∨ ¬aᵢ)`, 8 188 `let`s, compiled by the base in 85 s, refused
with "Stack overflow" bound; at 1 024 leaves 7 s against 78 s. Now
`bind` takes the shared pieces largest first and, of the first 0, 1, 2,
4, … and all of them (at most `MOST_LETS`, 1 024), binds the number whose
estimate (constructors written × (100 + `let`s) / 100) is least, never
more costly than binding none.

## Measured

`rocq compile` on core 13, under the machine lock, against 87r4's
compiled library (`scratchpad/speed/libA`), medians of five runs
(three at 1 024 leaves, one at 4 096):

| input | base `loqwlmwu` | head | `let`s | bytes base / head |
|---|---:|---:|---:|---|
| `!A ⊢ A^64` | 0.641 s | 0.425 s | 61 | 38 KB / 7.2 KB |
| `!A ⊢ A^128` | 1.451 s | 0.569 s | 125 | 146 KB / 14 KB |
| `!A ⊢ A^256` | 4.724 s | 1.231 s | 253 | 570 KB / 29 KB |
| balanced, 256 leaves | 1.756 s | 1.530 s | 32 | 105 KB / 74 KB |
| balanced, 1 024 leaves | 7.113 s | 5.962 s | 16 | 513 KB / 418 KB |
| balanced, 4 096 leaves | 34.87 s | 29.89 s | 16 | 2.49 MB / 2.09 MB |

So 3.8 times faster at `!A ⊢ A^256`, as 87r4 measured by hand (its `let`
form 1.40 s in the same run), and 13 to 16 % faster on balanced trees.
Inputs: `scratchpad/speed/{p,c}{64,128,256}.v`, `{q,b}{256,1024,4096}.v`,
`time3.py`.

## Departures

- **The classical certificate's statement has no `let`s** (departs from
  the plan, which expected the `let`s to shrink its text): it is the
  sequent over `Prop` and `tauto`, all statement, and a statement is
  written out (the audit's verdict). So it gets the bound alone.
- **`write_not_valid` keeps to the default bound, not the command's**
  (departs from the plan): it takes no limits, and a bounded form is an
  API addition (part 2). So `prove --logic classical --format rocq
  --derivation-limit 1000` on a doubling sequent still writes up to
  64 MiB (the counterexamples member's witness `d18.txt`: 1.8 MB), and
  `--derivation-limit none` cannot lift the 64 MiB. Recorded in the
  changelog and the rules file.

## The panel (certificates)

The three members on the brief `side-session-notes/…/87d4-panel-brief.md`,
from the start after the pause, and each again on the new `bind`:

- **The argument** (Fable 5.2, `high`): the nested `let`s overflowed and
  slowed Rocq (above): answered by `mttlplom`, then checked again: every
  bound piece written at least twice under any prefix (writes are
  monotone in the bound set), the `let`s name only earlier ones, the
  expansion is the base's for any set; six certificates expanded equal
  to the base, the 4 096-leaf witness compiles (36.5 s), a chain at the
  cap (1 024 `let`s) compiles. Its note: past the cap a chain of 1 500
  copies binds nothing and is the base's text (recorded in the rules
  file; a spread choice would bind it). The brief's "a text within the
  bound is never refused" overclaims: the estimate is above the text
  (95 bytes refused below 126); the repository says "estimated".
- **The counterexamples** (Opus 5.5, `high`): no difference. At
  `rylkosmn` 28 266 inputs (19 450 certificates, 11 140 with `let`s, two
  expanders of its own, one a tree-based second reference), 459
  certificates compiled; at `rlllvtpm` 7 698 inputs and 336 compiled.
  Its findings: the default classical path ignores `--derivation-limit`
  (the departure above), and outside the change a first-order embedding
  "unknown" writes a classical certificate Rocq refuses (below).
- **The integers** (Sonnet 5.5, `high`): no wrap, panic or truncation;
  `falsified_bytes` never below the text over 60 000 random cases (worst
  ratio 2.24), the refusal before any byte. Found: a saturated estimate
  passed a limit of `u64::MAX` (`kzuxuvxw`), and a doubling of sixty
  levels, estimated just past 2^63 but below `u64::MAX`, did too
  (`msuqwlxp`: refused past `isize::MAX`; the test builds sixty levels
  under both limits). `bind` is linear (58 to 139 ns a piece up to 6.4
  million pieces). The writer's memory is now of the order of the
  output (the pieces' texts), where the base streamed: bounded by
  `derivation_bytes` as the derivation is.

## The reviewer

`fresh-reviewer` on Fable, once, at `msuqwlxp`: one finding, the rules
file's test described as forty levels (fixed in `kmktkouz`), and a
question: the `isize::MAX` refusal is under any limit, not with
`derivation_bytes` `None`, where nothing is bounded by design; the two
subjects say "under any limit" now. It expanded the nine changed
snapshots against the base (equal byte for byte) and checked the
estimate against every `write!` of `falsified`.

## The net

- `gate` at `rlllvtpm`: passes (the ratchet's `render-typst` +1.16 %,
  as at 87c3; `msuqwlxp` and the rules commits came during and after,
  covered by the flake check below).
- `breaks 0.8.0`: the earlier steps' items only, nothing of the exports.
- The nix checks `export`, `rocq`, `rocq-deduxit` (the new certificates
  against the pinned library) and `check-certificate` at `xtrrokpr`:
  pass; the snapshots did not change after.
- `cargo test -p deduxit-cli --test readme` (the manual's example), the
  export snapshots, clippy.
- `nix flake check` at this report's head (`knrtoroq`, before this line
  was added): all checks passed, the gate's parts, `features`,
  `rocq-deduxit`, `check-certificate` and `ratchet` among them.

## For the planning session

- A bounded `write_not_valid` (or the command checking its own limit):
  an addition, part 2.
- `falsified` has no door for a first-order arena: `prove --logic
  classical --method embedding --format rocq 'p(a) |- q(b)'` answers
  unknown and writes `~ (forall p_28_a_29_ … : Prop, p(a) -> q(b))`,
  which Rocq refuses (the base does the same).
- A spread choice of pieces for chains past the cap, if a certificate
  of that size matters.
