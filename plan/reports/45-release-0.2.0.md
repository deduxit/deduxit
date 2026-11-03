# Step 45: the fourth baseline, and the second release, 0.2.0

The step's sessions each add a section here: 45b (the families that
grow), 45a (the Rocq library in its own repository), 45c (the fourth
baseline), 45d (the release prepared), 45e (the package archives) and
45f (the other provers on the growing sets).

## Session 45f: the other provers on the growing sets

### Outcome

The other provers' rows on the growing sets of 2026-11-03 are committed
as `bench/comparison/2026-11-03-growing/` (`runs.csv`, 3 068 rows;
`ranking.md`; each stream's `env.json`; the script's light `figures/`),
as `bench/comparison/2026-10-15/` keeps them. The manual has a chapter
of their own, `docs/src/results/provers-2026-11-03.md` ("The other
provers on the growing sets, 2026-11-03"), in `SUMMARY.md` beside the
growing sets of 2026-11-02: what ran where and when, the solved table,
each family per tool from the rows, the growth figure with its fitted
growth, both cactus plots and twelve scatter plots, all drawn at build
time by `bench/plots.py` from `runs.csv` through two lines of
`docs/figures.sh`. The section "The other provers" of
`growing-2026-11-02.md`, README's clause on the comparison, the
benchmarks chapter's table of comparison runs and the status chapter
point to it. The probe of the wrong refutations found their cause, an
integer overflow in the search Click & coLLecT and APLL share, and
cleared our translation: no translator changed, no row is the
harness's fault. No code changed.

### The probe of the wrong refutations

Run by hand on cores 2 and 3 in capped scopes, each run under a
timeout, with the binaries of `.#comparison` at this commit. The
harness's own translations were captured by putting a shim named like
each tool first on `PATH` (it copied the file it was given and answered
"timeout") and running `linlog-bench run --tools click-and-collect,apll-ll
--family wide-m1=1,2,3,4,8,32,36,64,128,200,255,256,512`; a hand
generator of the same text matched the captured files byte for byte at
3 and 256 pairs.

- **The threshold**: both tools prove `wide-m1` at 1 to 4 pairs, reach
  their limits (10 s for Click & coLLecT, 12 s for APLL by hand) at 8,
  32, 36, 40, 48, 56, 60 and 62 pairs (APLL tried at 8, 32, 60 and 62),
  and refute it at 63 pairs (64 formulas) in 4 ms and 5 ms, and at 64,
  128, 200, 255, 256 and 512.
- **Formulas, not atoms or depth**: one atom, `⊢ p0 ⊗ ⋯ ⊗ p0, p0⊥, …,
  p0⊥`, is refuted by both with 63 factors (6 ms and 9 ms) and 64, and
  reaches both limits with 61 and 62; linlog proves the 63-factor one.
  A balanced tensor of 63 atoms beside their 63 negations (depth 6, 64
  formulas) is refuted by both, of 62 not decided; `⊢ ⊥ ×63, 1` is
  proved by both (the `⊥` leave the context first). The two-formula
  shape `⊢ T, P` (the wide tensor and the `⅋` of its duals, 126 atoms)
  is refuted by APLL and proved by Click & coLLecT, whose `auto_weak`
  closes `⊢ A⊥, A` by an axiom (`proof.ml`, line 356): `P` is `T`'s
  dual; written as one formula `T ⅋ P` both refute it.
- **The cause, from the sources at the pinned revisions**: the `Tensor`
  case of the backward search counts the context's splits down from
  `fast_exp_2 (List.length gamma) - 1` (Click & coLLecT
  `focused_proof.ml` line 280; APLL `foc_ll_bwd.ml` line 157 and
  `foc_ill_bwd.ml` line 72, and the ILL search's left `⊸` at line 163;
  `fast_exp_aux` multiplies native ints). In 63-bit arithmetic `fast_exp_2 63` is 2⁶³ ≡ 0, so the count
  is −1 and `split_gamma` returns at once; at 62 it is 2⁶² = `min_int`,
  so the count is `max_int` and the search runs (simulated in Python
  with 63-bit wrapping: −1 for 63, 64 and 100). The search then fails
  everywhere and reports an exhaustive failure.
- **Both runs' wrong rows are this overflow**: in 2026-10-15's rows,
  the classical wrong refutations are `wide-m*` at 256 and above and
  `partition-yes` at 24 and 28, whose context after the `⅋` are taken
  apart holds 75 and 87 formulas (63 at 20, where both timed out); every
  APLL ILL refutation of a Petri net there has an initial marking of 63
  tokens or more (1 327: the 1 296 counted wrong and 31 that nothing
  proved; the other 13 such nets, of 401 to 20 000 tokens, ended in a
  stack overflow), and none with fewer is refuted (derived from the
  rows and the library's files, not run again).

So the translation is cleared, and nothing in either run is marked the
harness's fault; `provers-2026-10-15.md` is unchanged.

### The chapter, from the rows

- linlog 179 of the 202 problems it ran (both columns equal in every
  count), LinearOne 58 of 61, Sympli 23 of 28, the rest 2 to 13; only
  LinearOne solves problems linlog does not (8).
- The supervisor's first look held but for two points, corrected from
  the rows: `witness-fo/12` #3 ends at linlog's time limit, not its
  recursion limit (that is `lambek-fo`, from 512 on #0 and #2 and from
  1 024 on #3 and #4); and the default's 3 GiB peak is `pigeonhole` #1
  at 8, not `wide-m1`, whose default peaked at 2.0 GiB at 4 096 and was
  killed by the cgroup at 4 GiB at 8 192.
- Found in writing it: APLL's LL search is the fastest tool on `chain`
  (512 clauses in 9 ms against linlog's 3.7 s); linlog is slower than
  LinearOne on `lambek-fo` #1 at 1 024 and 2 048; in the comparison
  linlog gets no copy bound, so `chain-fo` decides less there than in
  the engines' run (#0 to 64, #1 never refuted); and the harness reads
  "copy bound" before "recursion" in linlog's verdict line, so `chain`
  at 1 024 is `copy_bound` where the line says the recursion limit
  (no count changes). The chapter says each; the last three are
  follow-ups in `plan/later.md`.
- `bench/TOOLS.md`'s "Wrong answers" gained the cause (it is what the
  drafts attach); `bench/COMPARISON.md` does not speak of these rows and
  is unchanged; the guide for AI models does not name the comparison
  and is unchanged.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-review-45f`),
read only, on cores 2 and 3, judged six criteria: every number of the
chapter, README and the benchmarks chapter against the rows; every
figure drawn from the rows through `docs/figures.sh` (it drew all 16
files of both themes); the threshold claim against the sources and the
probe's files (8 runs by hand of its own, which agreed); the rows
against the precedent and the run's files (all 25 equal by `cmp`); no
mention of the plan in the manual or README; the follow-ups against the
rows. Five findings, each answered:

1. *The Petri-net claim says APLL refuted every net of 63 tokens or
   more, but 13 such nets (401 to 20 000 tokens) ended in a stack
   overflow.* Holds; the chapter and APLL's draft say "every one on
   which it did not crash" and name the 13 (`bench/TOOLS.md`'s wording,
   about the refuted nets, was already exact).
2. *`foc_ill_bwd.ml` line 163 is not a `⊗` case but the left `⊸`
   (`Impl`, line 146).* Holds (checked: `Tensor` at 41, `Impl` at 146);
   the chapter, TOOLS.md, APLL's draft, `bench.md` and this report say
   line 72 for the `⊗` and 163 for a `⊸` focused on the left.
3. *`lambek-fo` #2 at 2 048 ends at the time limit, not the recursion
   limit.* Holds (60.0 s, `timeout`, in both columns); the chapter says
   so. The count of LinearOne's 7 problems there is unchanged.
4. *Against Sympli, `duplicate` #0 at 2 to 8 and `contraction` #0 at 2
   also lie above the diagonal*, by under a millisecond. Holds; the
   chapter names them.
5. *The rows equal the run's files and keep what the precedent keeps.*
   Holds; no change.

The rest held: every other number, both themes' figures, the arithmetic
of the overflow, the partition counts, README, the status chapter and
the follow-ups.

### Departures and assumptions

- **The chapter is named `provers-2026-11-03.md`**, by its date beside
  `provers-2026-10-15.md`, and the run's directory
  `bench/comparison/2026-11-03-growing/` as the brief named it.
- **The stream logs are not committed**: the precedent keeps none
  (`.gitignore` drops `bench/comparison/*/*.log` and the stream CSVs);
  the brief's "the streams' logs as there" read as that. The env
  records keep their scratch paths, as the precedent's do.
- **Beyond "rewrite nothing else"**, kept true where the run changes
  them: the benchmarks chapter (its list of chapters, a row of its
  comparison table, the sentence that said the growing comparison was
  not run, and a clause on the cause after the 1 308 and 12 wrong
  refutations), the status chapter (the growing comparison moved from
  planned to built), `bench/TOOLS.md`'s wrong answers, and two bullets
  in `.claude/rules/bench.md`. The design asked for these joins
  (45d2's "What the next sessions must know"), and conduct.md for a
  true status chapter.
- **The Petri-net link is derived, not run**: from 2026-10-15's rows and
  the library's files, with the counter in APLL's ILL search read in
  the source; said so in the chapter, the drafts and TOOLS.md.

### What the next sessions must know

- **Step 46** inherits a comparison whose wrong verdicts all have one
  cause; a future run on larger contexts draws more of them, which the
  judge already counts wrong. When the authors confirm or fix it, the
  drafts' paragraphs of 2026-11-03 are the ones to update.
- **The growing comparison runs linlog without a family's copy bound
  and with the cgroup's 4 GiB as `--memory-limit`**: rows on
  `chain-fo` and `wide-m1` differ from the engines' run for that, not
  for the engines; `bench.md` says so.
- **The probe's captured files and scripts are scratch** (this
  session's scratchpad), not in the repository; the shim technique is
  described above for a rerun.
- **The follow-ups** in `plan/later.md` ("Found by the other provers on
  the growing sets"): LinearOne's 8 problems (the recursion limit on
  `lambek-fo`, `witness-fo` #3 at 12), linlog's memory account on
  `wide-m1` at 8 192, `chain-fo` without its copy bound, the harness's
  reading of linlog's reason.
- **The other provers on the libraries beside 0.2.0's engines** are
  still not run (the status chapter's planned item).

### What is left open

- The messages to Click & coLLecT's and APLL's authors, the author's to
  send.
- The follow-ups above.

### Checklist

- [x] Workspace `../linlog-step45f` on `qrxzmunm`.
- [x] Read the brief, the step's prompt, `conduct.md`, 45b's and 45d2's
  sections, the Status entries of 2026-11-02 and 03, `bench.md`,
  `docs/figures.sh`, the results chapters, README's comparison clause,
  `plan/notes/comparison-messages.md`.
- [x] The rows committed (`ymqzpnqt`), each file equal to the run's.
- [x] The probe by hand: threshold 63 formulas beside a `⊗`, the cause
  in both sources, the translation cleared; sent to the supervisor.
- [x] The chapter, its figures in `docs/figures.sh` (drawn in both
  themes, the light growth, cactus and three scatter figures read
  rendered), `SUMMARY.md`, the pointers (`muqtnmow`); the finding in
  TOOLS.md, both drafts and `bench.md` (`mwxpoxzv`); the follow-ups
  (`mwpvrlqu`).
- [x] `nix build .#checks.x86_64-linux.manual
  .#checks.x86_64-linux.manual-links`: exit 0, 0 link errors; the new
  page holds its 32 figures and the growth table.
- [x] `cargo test -p linlog-cli --test readme`: 7 passed; `cargo test -p
  linlog-bench --test manual`: passed after the chapter named the
  command's call (`linlog prove --deterministic`) instead of a bare
  flag the test reads as the harness's.
- [x] No code changed, so no gate.
- [x] The fresh-context reviewer's round: five points, four corrections
  (the Petri nets' crashes, line 163's `⊸`, `lambek-fo` #2's time
  limit, two more points above Sympli's diagonal), one confirmation;
  the manual and its links built again and both tests passed after
  them.
- [x] `nix flake check -L --keep-going` under the machine lock with
  `--max-jobs 2 --cores 6`: exit 0 over the tree before the review's
  corrections; again at this report's commit, its exit status in the
  final message.
- [x] `jj st` clean; the final message to `linlog-orchestrator`.

## Session 45e: the Rocq library's opam submission

### Outcome

The Rocq library's submission to the Rocq opam repository is prepared
and checked, ready for the author to submit. It is one commit in a
local jj clone of `rocq-prover/opam`,
`/tmp/claude-1000/-home-tux-Projects-own-linlog/2e75f3b2-37e3-4af6-a93a-6dbd202461ad/scratchpad/rocq-opam`:
`40954aee` ("Add rocq-linlog.0.2.0", change `omkspqll`, signed by the
author's key), on the repository's `master` of 2026-11-02 (`80ee9978`,
the merge of pull request #3901), adding the file
`released/packages/rocq-linlog/rocq-linlog.0.2.0/opam`. Its `url` is
the `v0.2.0` tag's archive, its SHA-512 computed from that archive and
equal to the release notes'. `opam lint --check-upstream` passes, and
the package installs from the clone into a fresh switch for each of
the six `rocq-core` versions its bounds admit (9.0.0 to 9.3.0). Zenodo
archived linlog 0.2.0 under its concept DOI and gave `rocq-linlog`
0.2.0 a DOI of its own. The session ran 2026-11-03 00:15 to 00:23
(paused at the supervisor's word, the machine being needed) and 09:17
to the end (after the reboot, light work first, the installs at the
supervisor's word). nixpkgs was skipped at the author's word
(2026-11-03): any Nix user gets linlog and the library through their
own flakes, so nothing of nixpkgs was assessed, packaged or cloned.

### What was read

- **The contribution guide** is the repository's `README.md` at
  `master` (`80ee9978`): its section "Adding packages" sends a
  contributor to opam's general packaging manual and to the existing
  pull requests, and its section "Website and opam metadata" says how
  the `tags` field is read (`keyword:`, `category:`, `logpath:`, and
  `date:`, "the date the software was last updated (not the package
  definition)"). `released/README.md` says to
  add a package by a pull request to `released`. The repository has no
  pull-request template and no CONTRIBUTING file.
- **What its CI runs** (`.gitlab-ci.yml` and `scripts/`, read at the
  same commit): `scripts/opam-coq-lint` runs `opam lint --warn=-21
  --check-upstream` on every `opam` file the pull request touches (21
  is the warning on the `opam-version` field), and
  `scripts/opam-coq-install-remove` runs `opam install PKG -y
  --with-test` with the `released` repository added, under OCaml
  4.09.0, 4.14.2 and 5.3.0, the solver choosing the Rocq; a package
  that does not fit a compiler is reported as untested, not failed.
- **Packages accepted in the last months**, as models: `rocq-smt-lib.1.0`
  (commit `b03408285f`, pull request #3903, merged 2026-10-29),
  `rocq-sulfur.0.1` (#3891), `rocq-partial` (#3881) and
  `rocq-pi-leibniz.1.0.0` (#3884). Only `rocq-pi-leibniz.1.0.0` keeps
  `name:` and `version:`; the others leave them out, the directory
  naming the version. Each
  ends with `url { src: "…/archive/refs/tags/vX.tar.gz" checksum: … }`,
  the archive GitHub serves for the tag, with a `sha512=` or `sha256=`
  checksum; their commits are named "Add rocq-NAME.VERSION" (also
  "Package rocq-NAME.VERSION"); their pull requests' bodies are one line
  to a paragraph, and neither #3903 nor #3884 had a maintainer's review
  comment (their public pages).

### The package file

- **From the library's opam file at its tag**: the file in the clone is
  `rocq-linlog.opam` from the `v0.2.0` archive with four lines fewer
  (the licence header's two lines and the blank line after it, and
  `version:`) and the `url` section added; the rest is byte for byte the
  tag's (shown by `diff` in the session). The `version:` line goes, as
  most models leave it out (the directory names the version). The
  licence header goes too: it says the file is under the EUPL, while
  the opam repository is under the LGPL-2.1 (its `LICENSE`), and none
  of the repository's 3 671 package files carries a licence or
  copyright line (60 keep a generator's comment, "This file is
  generated by dune" and the like).
- **The url and the checksum**: `src` is
  `https://github.com/linlog-prover/rocq-linlog/archive/refs/tags/v0.2.0.tar.gz`,
  the archive the release workflow names in the release's notes; the
  checksum `sha512=14b5d239…39094b6e` was computed here with `sha512sum`
  over the archive downloaded from that URL (88 883 bytes, its pax
  header naming the tagged commit `b895ecd15b5d`) and equals the one the
  release's notes state (read on the release's public page, the GitHub
  API being rate-limited for unauthenticated calls that night).

### Zenodo

- **linlog 0.2.0 is archived** under the concept DOI
  10.5281/zenodo.23448588 (which `doi.org` resolves to the newest
  version, record 23899551): its own DOI is 10.5281/zenodo.23899551,
  titled "linlog", version "v0.2.0", the second version of the concept
  (index 1), software, EUPL-1.2, the author as the only creator, the
  description saying how the code was written, related to the tree at
  `v0.2.0`. Its publication date is 2026-11-03, a day after the
  `date-released` of `CITATION.cff` (2026-11-02), since Zenodo dates the
  record by its creation (2026-11-02 23:09 UTC, the release published
  after midnight in Zürich); nothing to change.
- **rocq-linlog 0.2.0 has a DOI**: 10.5281/zenodo.23899110 (record
  23899110, "rocq-linlog", version "v0.2.0", 2026-11-02, EUPL-1.2), and
  a concept DOI of its own, 10.5281/zenodo.23899109, which resolves to
  the latest version. Both records' public pages were read
  (`zenodo.org/records/23899551` and `/23899110`, HTTP 200, each naming
  its two DOIs).
- **Where the library should cite it**, for a later commit there (not
  made): `CITATION.cff` a top-level `doi: 10.5281/zenodo.23899109` after
  `date-released`, with the comment linlog's has ("The DOI of every
  version, which resolves to the latest."); README a DOI badge under
  the title and a short "Citing" paragraph, as linlog's README has
  (its line 3 and lines 115 to 118).

### The lint and the installs

- **`opam lint --check-upstream` passes** on the file in the clone,
  with opam 2.6.1 from the library's opam shell (`nix develop
  …/rocq-linlog#opam`), both as given and in the CI's form
  `--warn=-21`: "Passed.", exit 0 for each (run by itself on cores 2
  and 3 at about 09:20, shown in the session; the install log has the plain
  form again at its start). `--check-upstream` downloads the archive and
  checks it against the file's checksum.
- **The Rocq versions the file admits**: `rocq-core {>= "9.0" &
  < "9.4~"}` admits every `rocq-core` opam.ocaml.org publishes today,
  9.0.0, 9.0.1, 9.1.0, 9.1.1, 9.2.0 and 9.3.0 (its package page, read
  2026-11-03); `rocq-stdlib {>= "9.0"}` admits 9.0.0, 9.1.0 and 9.2.0,
  the solver taking the one each Rocq accepts. 45d installed four (the
  latest patch of each minor); this session installs all six, since a
  patch release is a version the file admits.
- **A first run proved nothing for Rocq 9.0.0** and was stopped: it
  installed `rocq-core.9.0.0` alone and then the package, and the
  solver upgraded Rocq to 9.3.0 for the newest `rocq-stdlib` (9.2.0,
  which needs a newer Rocq; `rocq-stdlib {>= "9.0"}` admits it). The
  script now asks for `rocq-core.V` and the package in one request and
  fails unless `opam list` shows `rocq-core` at V and the package from
  the clone's repository; every switch was made afresh after the fix.
  45d's script did not have this fault (it asked for `rocq-core.V
  rocq-stdlib` together).
- **The six installs pass** (2026-11-03 09:54 to 10:15, a
  detached unit `linlog-45e-installs` on cores 4 to 7, 8 GB; exit 0;
  the log `installs.log` beside the clone, the script
  `bin/install-from-clone.sh`): a fresh switch per Rocq version on OCaml
  4.14.4 (`ocaml-system`, the shell's; the CI uses 4.09.0, 4.14.2 and
  5.3.0), the default repository and the clone's `released` tree added
  as a repository, as the CI adds it, `opam install rocq-core.V
  rocq-linlog.0.2.0 --with-test`, the archive fetched from the file's
  `url` and its checksum checked by opam. Each switch then had:

  | rocq-core | rocq-stdlib | rocq-linlog | `.vo` files installed |
  |---|---|---|---|
  | 9.0.0 | 9.1.0 | 0.2.0, from the clone | 18 |
  | 9.0.1 | 9.1.0 | 0.2.0, from the clone | 18 |
  | 9.1.0 | 9.2.0 | 0.2.0, from the clone | 18 |
  | 9.1.1 | 9.2.0 | 0.2.0, from the clone | 18 |
  | 9.2.0 | 9.2.0 | 0.2.0, from the clone | 18 |
  | 9.3.0 | 9.2.0 | 0.2.0, from the clone | 18 |

  18 is the number of modules `_RocqProject` lists; a file requiring
  every installed module compiled in each switch, and the package was
  removed again.

### The pull request's text

Title: `Add rocq-linlog.0.2.0` (the commit's subject, as #3903's and
#3884's titles are theirs). The branch, in the author's fork:
`rocq-linlog.0.2.0`. Body:

> Adds `rocq-linlog.0.2.0`, the first release of the Rocq library of
> [linlog](https://github.com/linlog-prover/linlog), a prover for
> classical and intuitionistic linear logic whose proofs export as Rocq
> scripts checked against this library. It holds the one-sided sequent
> calculus of linear logic (with Mix, weakening and cut as parameters),
> the two-sided calculus of intuitionistic linear logic and its affine
> variant, their first-order versions, checkers for linlog's proof terms
> with the theorems that an accepted term proves its sequent, and
> certificates that a sequent is not derivable. No theorem rests on an
> axiom: each one's assumptions are closed under the global context.
>
> - Source: the GitHub archive of the tag `v0.2.0`, with the SHA-512
>   its release notes state.
> - Depends on `rocq-core` (>= 9.0, < 9.4) and `rocq-stdlib`; built
>   with `make`.
> - Checked: `opam lint --check-upstream`, and installed from this
>   branch into a fresh switch for each of `rocq-core` 9.0.0, 9.0.1,
>   9.1.0, 9.1.1, 9.2.0 and 9.3.0 (OCaml 4.14).
> - Documentation: https://linlog-prover.github.io/rocq-linlog/ ;
>   archived on Zenodo, https://doi.org/10.5281/zenodo.23899109.
> - The library, like linlog, was written with substantial AI
>   assistance (Anthropic's Claude models in Claude Code), as its
>   description says, in steps I planned, directed and accepted.

Every claim of the text rests on this section: the "Checked" line on
the lint and the six installs above, "no theorem rests on an axiom" on
the library's `CITATION.cff` and its check `rocq-linlog`, which prints
each theorem's assumptions closed (45d's report), and the DOI on the
Zenodo record read above.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-review-45e`),
read only, judged the file against the guide and the accepted packages,
the checksum against the release archive and the release notes, the
lint and install log, the pull request's text and the report's claims,
told to report only what breaks a stated requirement. It found the
file, the commit, the lint, the six installs and the pull request's
text sound (the checksum equal in three places: the archive, the
release notes and opam's download cache; each switch at its Rocq, the
package from the clone; "no theorem rests on an axiom" against
`tests/Assumptions.v` and a search of `theories/`), and two wrong
statements in this report, both of which held:

1. **"None has a `version:` field"** of the models was false:
   `rocq-pi-leibniz.1.0.0` keeps `name:` and `version:`. Answered: the
   sentence and the one in "The package file" now say only that one
   keeps them and the others leave them out; dropping the line stands,
   as most models and 45d's note have it. Checking that paragraph again
   here also corrected "two lines fewer" (four: the header's two lines,
   the blank after it, and `version:`) and "no file of the repository
   carries a header" (none carries a licence line; 60 keep a
   generator's comment).
2. **The `tags` rules were put in the guide's section "Adding
   packages"**; they are in its section "Website and opam metadata".
   Answered: "What was read" names both sections for what each says.

It noted, as no finding, that the install log holds the lint in its
plain form only; the CI's form `--warn=-21` was run separately in the
session, as "The lint and the installs" now says.

### Departures and assumptions

- **The licence header left out of the package file** (departs from
  the design, which takes the file from the library's opam file at its
  tag with the `url` section added): the header says the file is under
  the EUPL, the opam repository is under the LGPL-2.1, and none of its
  package files carries a licence line; the rest of the file is the
  tag's, byte for byte, but for `version:`, which 45d foresaw dropping.
- **The pull request's text has no template to fill** (departs from the
  design, which says "filled in from its template"): the repository has
  none (no `.github/PULL_REQUEST_TEMPLATE*`, no CONTRIBUTING file at
  `master` `80ee9978`), so the text follows the accepted pull requests
  #3903 and #3884, with the checks run and the AI disclosure that the
  library's own description makes.
- **Six installs, not 45d's four**: the file admits six published
  `rocq-core` versions, and the goal asks for each; agreed with the
  supervisor on 2026-11-03.
- **One compiler**: every switch is on OCaml 4.14 (`ocaml-system`, the
  shell's), one of the three the repository's CI uses (4.09.0, 4.14.2,
  5.3.0). The CI's 4.09.0 job reports the package as not compatible
  with that compiler if no admitted Rocq accepts 4.09, which its script
  counts as untested, not failed (`scripts/opam-coq-install-remove`);
  5.3.0 is left to the CI. An assumption, read from the script, not
  run.
- **The release notes were read on the release's public page**, the
  GitHub API having no unauthenticated calls left that night; `gh` was
  not used, as the brief says.

### What the next sessions must know

- **The author submits**: fork
  `rocq-prover/opam` on GitHub, push the clone's commit `40954aee` to a
  branch `rocq-linlog.0.2.0` of the fork, and open the pull request
  against `master` with the title and body above. The clone is
  `/tmp/claude-1000/-home-tux-Projects-own-linlog/2e75f3b2-37e3-4af6-a93a-6dbd202461ad/scratchpad/rocq-opam`
  (a scratch directory: copy it out, or redo the one file from the
  report, if the author submits after a clean of `/tmp`). From the
  clone, with jj (0.46.0 here): `jj git remote add fork
  git@github.com:USER/opam.git`, then `jj git push --remote fork
  --named "rocq-linlog.0.2.0=40954aee"`, which creates the bookmark,
  pushes it and tracks it (the form `plan/push-main.sh` uses for a new
  bookmark; `--remote` and `jj git remote add` as jj's CLI reference at
  docs.jj-vcs.dev gives them). Not run here: pushing is the author's.
- **After the pull request**, coqbot runs the GitLab CI (lint, then an
  install per compiler); a maintainer merges. A later release of the
  library adds a directory `rocq-linlog.X.Y.Z` the same way, the
  checksum from the release notes the library's release workflow
  writes. `opam publish` (which recent submissions also used, the
  branches `opam-publish-…`) would do the fork and push in one step,
  under the author's GitHub account.
- **The library's DOI** (10.5281/zenodo.23899109, all versions) belongs
  in its `CITATION.cff` and README, in a commit in its repository (the
  Zenodo section above says where); not made here.
- **A rocq-core release after 9.3** (9.4) is outside the file's bounds:
  a new version of the library, or a pull request widening the bound
  after the library's flake checks it, admits it.

### What is left open

- The submission itself, the author's.
- The DOI in the library's `CITATION.cff` and README.

### Checklist

- [x] Read the brief (`start-45e.md`), the step's prompt (45e's first
  part; nixpkgs skipped), `conduct.md`, the Status entries of
  2026-11-02 and 03, 45d's report, `plan/notes/distribution.md`, and
  the library's CLAUDE.md, opam file, release workflow and install
  script.
- [x] Workspace `../linlog-step45e` on `qrxzmunm`.
- [x] The Rocq opam repository's guide read at `master` `80ee9978`
  (README.md's "Adding packages" and "Website and opam metadata",
  `released/README.md`, `.gitlab-ci.yml`, `scripts/`), and recently
  accepted packages and pull requests as models.
- [x] A local clone through `jj git clone`, never git.
- [x] The release's notes read and the `v0.2.0` archive downloaded; its
  SHA-512 computed and equal to the notes'; the archive's commit
  `b895ecd1`.
- [x] `released/packages/rocq-linlog/rocq-linlog.0.2.0/opam` written
  from the tag's file, committed as "Add rocq-linlog.0.2.0"
  (`40954aee`, signed).
- [x] `opam lint --check-upstream` passing, also in the CI's form.
- [x] An install from the clone into a fresh switch for each of the six
  `rocq-core` versions the file admits, each at its version, the
  package from the clone (after a first run's fault was fixed).
- [x] Zenodo's records for linlog 0.2.0 and rocq-linlog 0.2.0 read on
  their public pages; where the library should cite its DOI noted.
- [x] The fresh-context reviewer's round; its two findings answered.
- [x] The pull request's text, the departures and what the next
  sessions must know, in this section.
- [x] Nothing pushed, forked or opened; no `gh`, no git.
- [x] This section committed in the workspace; the last message to
  `linlog-orchestrator` is sent after the commit, with the outcome, the
  clone's path and commit, the pull request's text, the Zenodo
  findings, the reviewer's findings and answers, the departures and the
  author's steps.

## Session 45d, part 2: the results and the date

### Outcome

The fourth baseline and the growing sets of 2026-11-02 are in the
comparison and the manual, README and the manual speak for the released
0.2.0, and both repositories are dated 2026-11-03. `bench/COMPARISON.md`
sets the four baselines side by side (the tables printed by `summary
--before` over the three earlier ones), with a section "Where the fourth
baseline ran differently" and "At a glance" written from the rows: no
decided verdict differs from the third's, every proof was checked, the
gains are the inverse method's turns, the refutation by the classical
reading and the net engine's pruning, the one slowdown a classical
assignment search after an exhausted refutation (read from the code,
not traced, under the author's rule of no timed runs). The manual has
two new chapters, `results/baseline-2026-11-02.md` (its parallel stage
in a section of its own that a quiet rerun replaces) and
`results/growing-2026-11-02.md`, whose three figures `docs/figures.sh`
draws at build time from the rows (`baseline-2026-11-02/families.svg`
and `engines.svg`, `growing-2026-11-02/engines/growth.svg` with its
`growth.txt`), beside the older chapters. The fourth baseline's
boilerplate was false (its confinement to CPUs 2–9 never took effect),
so `bench/baseline.sh` now confines by affinity and writes what the run
had, and this run's `RESULTS.md` says what it had. README, the status
chapter and the guide's description say what 0.2.0 adds; "Beside the
other provers" says the comparison is of 0.1.0's engines on the
libraries and of the first-order search against LinearOne, in a table a
later run joins as a row. The Rocq library's commit was checked, sent as
"rocq ready" and pushed by the supervisor, and linlog's lock follows it.
The library's dated commit is `6f5028f8728f` (pushed by the supervisor);
linlog's lock is on it (`flake.lock: Update`), its two Rocq checks built
against it (exit 0).

### The cores, and what the fourth baseline had

- **Found at the start, confirmed by the supervisor**: the fourth
  baseline's `--cpus=2-9` and `--others=0-1,10-15` did nothing. A user
  unit's or slice's `AllowedCPUs` needs the cpuset controller, and the
  user manager's cgroup has `cpu io memory pids` only
  (`user@1000.service/cgroup.controllers`); `systemd-run --user --scope
  -p AllowedCPUs=4-5 nproc` printed 16. The rows agree: the parallel
  stage's `jobs` are 16. So its parallel stage ran 16 threads over the
  whole machine beside two other projects' sessions, and the sequential
  streams, pinned by taskset to CPUs 4, 5, 2 and 3, were not shielded
  (46 problems `†`). The same holds of every earlier baseline's "kept
  off" slices, which `bench.md` claimed; it now says so.
- **The fix at its cause** (`Confine a baseline by its affinity and say
  what it had`): `--cpus` is `taskset -cp` on the script's own PID
  before anything starts, which every child inherits and the streams'
  taskset narrows; the slices are shielded only where the cpuset
  controller exists; RESULTS.md's paragraph and the record line say
  which CPUs the streams had, whether other processes could share them,
  and where the parallel runs ran (confined, with the count of CPUs, or
  every core); the header counts the machine's cores by `nproc --all`.
  Not run (no timed runs); `bash -n` and shellcheck through `nix fmt`.
- **The two bias passes and the long runs** ran on CPUs 4 and 5,
  efficiency cores, where the third's ran on 1 and 0: their times are
  1.3 to 1.6 times the third's for that (median ratios per collection
  1.29 to 1.58 but KLE-IMP-CONJ's non-theorems, against 0.89 to 1.08
  for `lltp-intuitionistic` on CPU 2 in both), and the default's time over theirs is 0.75 and 0.76 where it
  was 1.01 and 1.04. The comparison and the page say so.

### What changed against the third baseline, from the rows

- No decided verdict differs; 264 of 16 733 problems decided alike on
  one thread differ in their counters (the engines changed).
- Five problems decided before are not: four of `lltp-default` (the
  parallel stage, at the 2 s limit; `SYJ212+1.012` had 13 s of CPU in
  2 s, about 6.5 of 16 threads), and `SYJ205+1.009` of `lltp-forward`
  (3.46 s on CPU 0, past 5 s on CPU 5).
- Decided now, by the rows' engine and earlier reason: the inverse
  method's turns 104 of `lltp-default`, 107 of `lltp-forward`, 94 of
  `lltp-copies-10`, and `mix` (62.5 s → 259 ms at 10); the classical
  reading 53 in the default and classical copy-bound-3 passes, 54
  under `--bias rarer`, 27 of `lltp-copies-10`, 3 of
  `lltp-forward`, and `growing` at every size; the net engine
  `3-partition-mll-no`, `partition-table` and two sizes of the
  Partition families. The one new verdict against an LLTP header,
  `KLE083+1` of `KLE-01`, is a classical refutation, so the header is
  wrong.
- Slower: refuted QBF instances of 16 to 24 variables (1.1 → 8 ms,
  5 → 80 ms) with equal nodes, splits and work. In the code, a search
  whose refutation is `Exhausted` then looks for a classical assignment
  within `Options::classical_work` (2²⁶) for the disproof
  (`core/src/search/mod.rs`); on these instances it finds none. Derived
  from the code and the rows, not traced (a follow-up below).
- Holds whatever the load: on `mix`, one thread decides 8 to 11 pairs
  in about 0.27 s by the inverse method's turn, which only a one-thread
  search gets, while 2 to 16 threads race the focused engine alone (0.8
  s at 8 pairs, past 120 s at 11).
- The growing sets of 2026-11-02 reproduce 2026-10-31: every line both
  runs drew ends at the same size, the largest decided size within a
  quarter of the earlier time where over a millisecond, apart from the
  sizes extended since and `duplicate`'s classical shape, new. So no
  claim in `core/src/families.rs` changes, and no code did.

### The pages, the figures and the words

- `bench/COMPARISON.md`: the four baselines; the hand-written head, then
  `summary --before … --before … --before …` with a note beside each
  table that the parallel stage's rows are not comparable, then the
  bias passes by `summary --against` with the cores said, then the
  LinearOne section kept.
- `docs/src/results/baseline-2026-11-02.md`: the families table beside
  the third's numbers, the two figures with what each shows (both read
  rendered), the LLTP table, and "The parallel stage, not comparable",
  which names the four files a rerun replaces.
- `docs/src/results/growing-2026-11-02.md`: the engines' figure (read
  rendered), what is new, the fitted growth, and "The other provers",
  which says they have not run to their limits and where their rows
  join; `docs/figures.sh` has a comment where their `growth` line goes.
  The chapter of 2026-10-31 stays as the run that placed the sizes.
- `docs/src/benchmarks.md`: the list of chapters, the fourth baseline's
  paragraph, and under "Beside the other provers" a table of what each
  comparison run covers (0.1.0's engines on the libraries, 2026-10-15;
  the first-order search against LinearOne, 2026-10-30; the check of
  2026-10-31 at the first two sizes) that a later run joins as a row.
- README: a paragraph of what 0.2.0 adds and a clause on what the
  comparison covers (162 lines). `status.md`: four baselines; the
  release's packages; the web page moved to built; "Planned" now
  first-order proving on the web page and the other provers beside
  0.2.0's engines (the first-order memo, its parallel search and the
  times against LinearOne were built and listed as planned). The guide's
  description names first-order logic and cut elimination; the rest of
  the guide was already true for 0.2.0. The installation chapter's URLs
  were already v0.2.0's (part 1).
- Dates: linlog's `## [0.2.0] - 2026-11-03` and `date-released:
  "2026-11-03"`; the library's the same and `"date:2026-11-03"` among
  the opam file's tags.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-review-45d2`),
read only, on cores 4–7, no timed run, judged seven requirements: the
comparison's columns and cores, every number on the pages against the
rows, every figure drawn from the rows and listed, what the comparison
with the other provers covers, README, the manual and the guide true for
0.2.0, the dates and the release workflows' checks, and the script's
change. It drew every figure and checked every count of the comparison
and the pages (the LLTP counts of all four baselines, the inverse and
classical counts, the five lost problems, all 200 growing lines' ends);
those held. Seven findings, each answered:

1. *The cores' ratios were misquoted* ("1.31 to 1.58 in every
   collection", `lltp-intuitionistic` "0.89 to 1.06"; KLE-IMP-CONJ's
   non-theorems are 1.56 on CPU 2 in both; the long runs differ).
   Holds: 1.29 to 1.58 but the non-theorems, `lltp-intuitionistic` 0.89
   to 1.08, the non-theorems 1.56 there and 1.89 and 1.90 in the passes,
   the long runs' ratios given, in the comparison and the page.
2. *Mix's default is flat from 8 pairs, not 6.* Holds; corrected.
3. *`lltp-rarer` gained 54 by the classical reading, not 53.* Holds; the
   page and the comparison say 54.
4. *The page said every other pass ran on the third's cores*, but two
   of the last stage's reruns ran on CPUs 4 and 5. Holds; the page names
   them.
5. *The guide's summary said `prove` decides quantified sequents "not
   yet with `-i`"*, false for 0.2.0. Holds; it now says with `-i` too
   (the readme test passes, the summary within its budget).
6. *The script blamed the missing cpuset controller also when a
   foreground run is simply not in a slice of its own; and a stream's
   taskset outside `--cpus` would widen the affinity unchecked.* Holds:
   the paragraph names the cause it has, and the script refuses
   `--cores` outside `--cpus` (its membership test tried on `2-9` and
   `2-9,12`).
7. *The manual's introduction named the comparison without saying what
   it covers.* Holds; it says so as README does.


### Departures and assumptions

- **The fourth baseline's parallel stage is reported, not rerun**
  (departs from the design's "comparable with the third"): the author's
  rule of 2026-11-02 forbids timed runs until further notice; the page
  and the comparison say it is not comparable, and the supervisor puts a
  quiet rerun to the author.
- **The bias passes' times are not comparable with the third's
  either**, for the cores (departs from the design's comparability); the
  verdict counts are.
- **The growing sets of 2026-11-02 ran by day**, on CPUs 2 and 3 beside
  other work, not on an idle night (departs from the design's "the
  nights"); they reproduce the placing run, and the page says how they
  ran.
- **The QBF slowdown is explained from the code, not traced** (no timed
  run allowed); the comparison says so.
- **The 2026-10-31 growing chapter is kept** beside the new one as the
  placing run, with its check of the other provers at the first two
  sizes, rather than replaced.

### What the next sessions must know

- **A baseline by day**: `bench/baseline.sh --cpus=LIST` now confines
  by affinity; the user's slices are never shielded on this machine (no
  cpuset controller), so a run beside other work is marked by its `†`
  count and its RESULTS.md says so. A rerun of the fourth's parallel
  stage on a quiet machine replaces `parallel.csv`, `parallel-net.csv`,
  `lltp-all-cores*.csv` and `lltp-default.csv` in
  `bench/results/2026-11-02/` and the section "The parallel stage, not
  comparable" of its chapter, and the sentences on them in
  `bench/COMPARISON.md`; the rest of the tables are regenerated by the
  two `summary` commands at its head.
- **The other provers' rows join** as a row of the table in
  `benchmarks.md`'s "Beside the other provers", a `growth` line in
  `docs/figures.sh` (the comment says where) and the section "The other
  provers" of `results/growing-2026-11-02.md` (or a chapter of their
  night's own date), from `bench/comparison/DAY-growing/runs.csv`.
- **45e** starts after the author's tags: linlog's `v0.2.0` and the
  library's `v0.2.0`, both dated 2026-11-03.
- Follow-ups: the classical assignment search after an exhausted
  refutation costs up to 80 ms on small QBFs for a certificate the
  search does not need; the inverse method's turns are off under the
  pool, so `--jobs` above 1 on Mix is hundreds of times slower than one
  thread.

### What is left open

- The parallel stage's rerun on a quiet machine, and the other provers'
  night, both the author's to give.
- The two follow-ups above, after 0.2.0.

### Checklist

- [x] Workspaces `../linlog-step45d2` (on `sxprrrvl`) and
  `../rocq-linlog-45d2` (on `rlunttwn`).
- [x] Read the brief, the step's prompt, `conduct.md`, 45d part 1's and
  45b's sections, the Status entries of 2026-11-02, `bench.md`, the
  benchmark chapter and its results pages, `docs/figures.sh`,
  `bench/COMPARISON.md`, the fourth baseline's `RESULTS.md`, README,
  both changelogs and citation files, `bench/baseline.sh`.
- [x] Paused at 17:36 for the author (graceful stop, nothing running),
  resumed at 21:44 under the author's new rule: no timed run of any
  kind.
- [x] The cpuset finding sent and confirmed; `bench/baseline.sh` fixed;
  `bench.md` corrected; this run's `RESULTS.md` corrected.
- [x] `bench/COMPARISON.md` with the fourth baseline's column, the
  cores stated, the parallel caveat beside every table.
- [x] The manual's pages for the baseline and the growing sets of
  2026-11-02, figures in `docs/figures.sh`, drawn (48 SVGs) and the new
  light ones read rendered.
- [x] README, `benchmarks.md`, `status.md` and the guide's description
  for 0.2.0 and the other provers.
- [x] `cargo test -p linlog-cli --test readme` (7 passed) and `cargo
  test -p linlog-bench --test manual` (1 passed); `nix build
  .#checks.x86_64-linux.manual .#checks.x86_64-linux.manual-links`:
  exit 0, 0 link errors.
- [x] The dates in both repositories; the library's `nix flake check`
  at `6f5028f8728f`: exit 0; "rocq ready" sent.
- [x] Linlog's lock moved to `6f5028f8728f` (`flake.lock: Update`);
  `nix build .#checks.x86_64-linux.rocq-linlog
  .#checks.x86_64-linux.rocq-linlog-library` against it: exit 0.
- [x] The fresh-context reviewer's round: seven findings, each held and
  fixed (three commits); the readme and manual tests passed again.
- [x] `nix flake check` at this report's commit, under the machine lock
  with `--max-jobs 2 --cores 6`: its exit status in the final message.
- [x] `jj st` clean in both workspaces; the final message to
  `linlog-orchestrator`.

## Session 45d, part 1: the release prepared

### Outcome

The 0.2.0 release's mechanics are prepared in both repositories, all
but the fourth baseline's results, the final words of README and the
manual, and the date, which part 2 does. In linlog
(`../linlog-step45d1`, on `qknxxpoo`): what 0.2.0 breaks was found by
two tools and is listed in the changelog's `## [0.2.0] - unreleased`
with what to do instead; `core.md`'s lists of closed and marked types
were checked against the rustdoc JSON and corrected (15 closed, 119
marked); `Error::Quantifiers`, raised nowhere and never released, is
retired; the version is 0.2.0 in the manifests, both locks,
`CITATION.cff` and through them the web bindings' package; `cargo
publish --dry-run --locked --workspace` packages and verifies both
crates (exit 0). The command has prebuilt files for every system: the
flake builds Linux's (a static musl binary with mimalloc, in an archive,
a `.deb`, an `.rpm` and an Arch Linux package, `nix build .#dist`), the
release workflow builds macOS's and Windows' archives on their runners
and attaches everything with `SHA256SUMS` and a build provenance
attestation; every package carries the licences, the crates' notices
among them. The manual's Installation chapter covers Debian and Ubuntu,
Fedora, Arch Linux, any Linux, NixOS and Nix, macOS and Windows, with
what each needs, how to update and remove, and what an unsigned file
meets; every Linux instruction ran in that distribution's root
filesystem here, the Nix ones in a temporary profile, and a new workflow
(`install.yml`) runs them all on GitHub's runners and containers. The
Rocq library (`../rocq-linlog-45d1`) is ready for its own 0.2.0: checked
with Rocq 9.0 to 9.3 by its flake, installed from its opam file into a
fresh opam switch for each, `opam lint` passing, with a changelog,
`CITATION.cff`, README, documentation and a release workflow; the
supervisor pushed it (`main` = `2ad6487e8388`, after the review's README
fix on `52e23df76340`) and linlog's lock is on it, every one of the 49
certificates linlog's check compiles closed under the global context.

### The break, found by tools

- **Tools**: cargo-semver-checks 0.51.0 and cargo-public-api 0.52.0 from
  the pinned nixpkgs, adopted through `new-tool` as the devshell's
  `breaks VERSION` (`modules/devshell.nix`; `core.md`, "The break against
  the last release", says how to read them). cargo-semver-checks has no
  lint for a changed field or signature type, so the second tool diffs
  both public APIs, each built as rustdoc JSON with every feature (the
  published 0.1.0 from its crates.io package). cargo-semver-checks'
  own JSON documents private items and its baseline overwrites
  `target/doc/linlog.json`, which a first attempt at the diff ran into.
- **Its findings** (`enum_variant_added`, `enum_no_repr_variant_discriminant_changed`,
  `partial_ord_enum_variants_reordered`; the diff's "Changed"): the new
  variants `Term::{Forall, Exists}`, `Kind::{Forall, Exists}`,
  `Node::{Cut, Forall, Exists}`, `Rule::{Cut, Forall, Exists}`;
  `Rule::ALL` `[Rule; 16]` → `[Rule; 19]` and `Named::ALL` `[Named; 34]`
  → `[Named; 41]`; `Rule::Open` 15 → 18 and after the new rules in the
  derived order; `export::rocq::Unsupported::AffineWeakening` 2 → 3 and
  `Compact` 3 → 4. A test crate showed that a downstream `as` cast of
  the `#[non_exhaustive]` fieldless `Unsupported` compiles, so its moved
  numbers are a break, if a small one. The diff's other lines are none:
  `svg::Font`'s derived `Deserialize` became a hand-written one (removed
  and added again, the `where` bound alone differing), and
  `view::Inference`'s hand-written `PartialEq` loses
  `StructuralPartialEq`, which matters only to a constant pattern, which
  its private fields already forbid.
- **The plan's list corrected** (departs from the design's accounting):
  `Placed::parent` is no break, since `Placed` came with the MELL nets on
  2026-10-23, after 0.1.0; nor are the Rocq certificates with a record
  literal, since 0.1.0's exporter wrote NanoYalla's alone (its crate has
  no `Kernel`); `FoTerm`, `Fault`, `Refusal` and `Space` are new or
  marked. The changelog said otherwise for the first two and now says
  this.
- **The command**: 0.1.0's binary (built from crates.io) and this one's
  `--help` of every subcommand compared flag by flag and value by value:
  nothing removed, 29 flags and the subcommand `eliminate` added, the
  engines `essential` and `inverse` among `--engine`'s values. Three
  changes of behaviour a script sees are in the break list with what
  to do instead: `-i` reads ILL's syntax alone (`--ill-syntax
  one-sided`), exit 1 instead of 3 after the classical refutation
  (`--classical-work 0`), `--format rocq` without `--kernel` for linlog's
  library (`--kernel nanoyalla`).
- **`core.md`'s lists** were checked by a script over the head's JSON:
  0.1.0 had 13 closed and 81 marked types; now 15 closed (`Division`
  and `FoTerm` added; the list lacked `FoTerm`) and 119 marked (23 were
  missing: the net looks, `Order`, the MELL and net-cut types,
  `IllSyntax`, `InputOptions`, `TermNode`, `Witness`, the refutation
  types, `SymbolFault`). `Named::ALL` stays an array: `Named` is a
  struct over the closed `Rule`, `Side` and `Division`, so its length
  changes only where a closed enum's does.
- **`Error::Quantifiers`** is retired: not in 0.1.0, constructed by
  the tests alone, its code `quantifiers` named by no document, the
  guide or the web client. No item is `#[deprecated]`.

### The versions

0.2.0 in `[workspace.package]` and the workspace dependency, both
`Cargo.lock`s (`cargo update --workspace`, `-p linlog` in `fuzz/`),
`CITATION.cff` (its `date-released` removed until part 2 dates it),
README's dependency line; the web bindings take the workspace's version.
Every outcome writes the crate's version, so the behaviour lock
(`core/tests/lock/json.txt`) and `core/tests/serialize.rs` change by it
alone, in the version's commit (checked: masked, the files are equal).
docs.rs builds `linlog` with every feature on x86_64 Linux and
`linlog-cli`'s library, as the flake's `doc` check does.

### Installing on each system

- **Research** (three sub-agents on Sonnet 5.5 at `medium`, sources
  read that day): ripgrep, fd, bat, hyperfine, typst, just, starship,
  zoxide, Kani and Verus install pages; every one offers prebuilt
  archives and a source build, most lead with distribution channels and
  Homebrew, only typst says how to update and none how to remove.
  rustc per system (packages.debian.org, packages.ubuntu.com,
  packages.fedoraproject.org, archlinux.org, formulae.brew.sh): Debian
  13 1.85.1 (backports 1.96.1), Ubuntu 24.04 1.75 (1.91 versioned),
  Ubuntu 26.04 1.93.1, Fedora 43/44 1.99.1, Arch 1.99.1, Homebrew
  1.99.1; MSRV 1.92. rustup's commands and Windows' MSVC need
  (rust-lang.org/tools/install, the rustup book). `nix profile install`
  became `add` in Nix 2.30 (its release notes); flakes are off on the
  official installer (nix.conf reference). Release tooling: dist 0.34.0
  owns its release workflow and builds `.msi` with WiX v3 only
  (axodotdev's book); ripgrep, fd and typst build by hand-written
  matrices, ripgrep a static musl binary and `.deb`, with jemalloc on
  musl because musl's allocator is slow (its `main.rs`); Flathub takes
  no console software ("Console softwares will not be accepted",
  docs.flathub.org requirements); AppImage's documents say nothing for
  a single static binary to gain; `actions/attest` is the current
  provenance action. The Rocq practice is in its own bullet below.
- **Offered for 0.2.0** (needs no account): prebuilt archives for
  `x86_64`/`aarch64-unknown-linux-musl`, `aarch64`/`x86_64-apple-darwin`,
  `x86_64`/`aarch64-pc-windows-msvc`; `.deb`, `.rpm` and an Arch Linux
  package for both Linux architectures (one more than the plan named,
  free with nfpm); `cargo install`; the flake. Not offered: Flatpak and
  AppImage (above); an `.msi` (WiX v7 asks its user to accept a EULA
  with the Open Source Maintenance Fee, for organisations above $10 000
  of revenue, docs.firegiant.com/wix/osmf: the author's to accept;
  departs from the design); a macOS `.pkg` (no surveyed tool ships one,
  and an unsigned one meets Gatekeeper where a `curl`-fetched archive
  does not); `cargo binstall` (it would need `[package.metadata.binstall]`
  to find archives named `linlog-…`, untested, a follow-up).
- **How they are made**: `modules/dist.nix` (`linlog-static`: crane's
  musl example, `pkgsStatic`'s C compiler for mimalloc; `dist`: a
  deterministic `.tar.gz` and nfpm 2.47.0's `.deb`, `.rpm` and
  `.pkg.tar.zst` from one list of files; `notices`: cargo-about over
  `cli/about.toml`'s eight targets); nfpm instead of cargo-deb and
  cargo-generate-rpm (departs from the design: cargo-generate-rpm is not
  in nixpkgs, and nfpm, GoReleaser's packager, makes all three formats
  in the sandbox without running cargo). `.github/workflows/packages.yml`
  builds them on `ubuntu-24.04` and `ubuntu-24.04-arm` with Nix, and
  macOS's and Windows' archives with rustup at the flake's compiler
  version on `macos-26`, `macos-26-intel`, `windows-2025` and
  `windows-11-arm` (ci.md's exception, written down). `release.yml`
  calls it after `check`, writes `SHA256SUMS`, attests every file
  (`actions/attest` v4.2.2 by SHA) and attaches them with the crates.
  Every package holds the binary, README, `LICENSE`, the five fonts'
  licences and `THIRD-PARTY-NOTICES.txt` (143 crates' entries); the
  flake's default package now installs the notices too.
- **musl and speed** (the supervisor's request, two timed runs it
  cleared, cores 4–7, medians of three, the flake's glibc build against
  the static one of the same commit): with musl's allocator `chain` at
  512 took 3.63 s against 4.70 s on one thread and 3.73 against 5.46 on
  four; `witness-fo` #3 at 10 3.80 against 6.81 and 2.67 against 9.83.
  With mimalloc (`#[global_allocator]` under `cfg(target_env =
  "musl")`, MIT, `cargo deny check` ok): 3.62/4.67, 3.74/4.73,
  3.82/5.50, 2.65/3.84, so 1.26 to 1.45 times on one thread and four
  alike, the rest musl's libc. The supervisor decided: the static binary
  for 0.2.0, the chapter states the numbers and recommends `cargo
  install` or Nix for long searches, a glibc-linked prebuilt binary is a
  follow-up.
- **Unsigned, what a user meets** (in the chapter): apt and pacman say
  nothing for a file; dnf5 installs it with "Warning: skipped OpenPGP
  checks for 1 package from repository: @commandline" (seen); macOS
  runs a `curl`-fetched binary and refuses a browser-fetched one until
  `xattr -d com.apple.quarantine` or "Open Anyway" (Apple's support page
  102445; not seen here); Windows may show SmartScreen's "Windows
  protected your PC" (Microsoft's SmartScreen page; `Unblock-File`
  clears the mark; not seen here).

### Every instruction, and how it was tried

| instruction | tried how | result |
|---|---|---|
| Debian 13: `curl -LO …deb`, `apt install ./…deb`, `apt remove linlog` | LXC root filesystem under bubblewrap (a user namespace, root mapped, capabilities inside it; apt's sandbox user set to root for the trial), the file served at 127.0.0.1 for the URL | installed, ran, removed |
| Ubuntu 24.04: the same | the same | installed, ran, removed |
| Fedora 44: `dnf install URL`, `dnf remove linlog` | the same; also `./file` | installed (the OpenPGP warning), ran, removed |
| Arch: `curl -LO`, `pacman -U ./…`, `pacman -R linlog` | the same (pacman's `DownloadUser` off for the trial) | installed, ran, removed |
| any Linux: `curl -LO` the archive and `SHA256SUMS`, `sha256sum --check --ignore-missing`, `tar -xzf`, `install -D … ~/.local/bin/linlog`, `rm` | in all four | OK, ran, removed |
| `cargo install --locked linlog-cli`, `cargo uninstall` with Fedora's Rust (`dnf install cargo`) | the same (crates.io serves 0.1.0 until the author publishes) | built in 56 s, ran, removed |
| the same with Arch's Rust (`pacman -S rust`) | the same, after `pacman-key --init --populate`, which the image skips (the first run failed for that; the rerun's output is in the session and appended to the trial log) | built, ran, removed |
| rustup on Debian 13 (`curl … sh.rustup.rs \| sh`), `rustup self uninstall` | the same | built, ran, removed; rustup removed |
| Debian 13's and Ubuntu 24.04's own rustc too old | `apt install rustc` there | 1.85.1 and 1.75.0, as the chapter says |
| Nix: `nix run github:…`, `nix profile add`, `remove linlog` | a temporary profile, at origin's `main` (`ceed014d`) for the tag | ran; the entry is named `linlog`; added, removed. `nix profile upgrade linlog` on that locked entry only warned that it "can't be upgraded" (a first record here read it as done; the reviewer found it): the chapter now updates a tagged entry by adding the next tag, and `install.yml` tries `upgrade` on an untagged one |
| Ubuntu 26.04's `apt install cargo build-essential`, macOS's archive and `shasum`, Windows' PowerShell steps, the arm64 `.deb` and `.rpm`, `cargo install` on macOS and Windows, the C compilers the source routes now name | not here (no root filesystem fetched for 26.04; no macOS or Windows) | by `install.yml` after the supervisor's push: all passed (below) |
| the Arch package on ARM | nowhere (Arch Linux ARM has no container image on GitHub) | not offered by the chapter: ARM takes the archive; the file is still built |
| the chapter's downloads from the release (`v0.2.0` URLs, `dnf install URL`) | after the tag only: `install.yml`'s `release` jobs, skipped while the latest release has no `SHA256SUMS` | after the tag |
| the Rocq library: `opam install` of the package, per Rocq version | `nix develop .#opam -c scripts/opam-install.sh 9.0.1 9.1.1 9.2.0 9.3.0` in a temporary opam root (OCaml 4.14 as `ocaml-system`) | lint Passed; installed, every module imported, removed: 9.0.1 with stdlib 9.1.0, 9.1.1/9.2.0, 9.2.0/9.2.0, 9.3.0/9.2.0 |
| the library by Nix (`nix build`, a shell with the package) | a shell of Rocq, its stdlib and the package | `From Linlog Require Import Soundness` compiled without `ROCQPATH` |

### The Rocq library, released

- **Complete for 0.2.0**: linlog's check `rocq-linlog` now prints
  `Print Assumptions` of every certificate's lemma: 49 certificates
  (the 34 exporter snapshots: one-sided classical, Mix, affine, cut,
  MALL, MELL, two-sided, first-order with cut, instances, witnesses and
  eigenvariables, every refutation kind; and 15 the command writes),
  each `Closed under the global context`. What it lacks, in its README:
  theorems for linlog's count of one sign under an exponential, a state
  equation, a saturated set and any first-order refutation, so linlog
  writes no certificate for them; the ordered calculus; cut elimination.
- **By the Rocq community's practice** (a sub-agent on Sonnet 5.5 at
  `medium`, read that day): rocq-prover/opam's README (the `released`
  repository, tags `logpath:`, `category:`, `keyword:`, `date:` "the date
  the software was last updated"), its CI (`opam lint --check-upstream`,
  one Rocq picked by the solver), rocq-partial, rocq-pi-leibniz,
  rocq-smt-lib, rocq-sulfur and rocq-laproof (tag archives with their
  checksums, open or bounded `rocq-core`); rocq-stdlib's versions do
  not follow Rocq's (9.3 takes stdlib 9.2.0), so the file bounds
  `rocq-stdlib` below only; `rocq doc` is the current name and `make
  html` its target; the rocq-community templates emit `doc:` for a
  Pages site.
- **What it has now** (commits `zllvwomt`, `pxzymvww`, `oquqzrlr`,
  `opkuqzzo`): the flake checks it with every admitted Rocq
  (`rocq-linlog-9_0`, `-9_2`, `-9_3` beside the default 9.1; 9.2 and 9.3
  print the deprecation of `Notation` for abbreviations, whose
  replacement 9.0 and 9.1 lack); `nix build .#doc`; the opam shell and
  `scripts/opam-install.sh`; the opam file at 0.2.0 with `rocq-core
  {>= "9.0" & < "9.4~"}`, `rocq-stdlib {>= "9.0"}` and `doc:`; a
  changelog section `## [0.2.0] - unreleased`; `CITATION.cff`; README's
  install by opam and Nix and what it covers; `.github/workflows/release.yml`
  (the tag against the opam file, changelog and citation file, `nix
  flake check`, a GitHub release with the changelog's section and the
  tag archive's SHA-512 for the opam file, the documentation on Pages);
  CLAUDE.md and `rocq.md`, with the line on linlog's
  `rocq-linlog-library` that 45a left. Its `nix flake check` exited 0 at
  `52e23df7`; "rocq ready" was sent and answered (pushed), and linlog's
  `nix flake update rocq-linlog` moved the lock there with `LINLOG =
  "0.2.0"`.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-review-45d1`),
read only, judged four areas: the break list against the tools'
outputs and `core.md`'s lists, every instruction against its trial and
`install.yml`, the release's packages, and the Rocq library's
readiness. The break list, the type lists, the packages' names,
artifacts, pins and permissions, and the library held; seven findings:

1. *A Nix entry added by the tag is never upgraded, and the trial
   record claimed it was; the same for an opam pin.* Holds: the
   chapter, `rocq.md`, the library's README (`2ad6487e`, pushed, the
   lock moved) and `install.yml` now say and try what updates each, and
   this report corrects the record.
2. *macOS's and Windows' archives lack `THIRD-PARTY-NOTICES.txt`.*
   Holds: `packages.yml` builds the notices once with Nix and puts them
   into every archive.
3. *Windows' binary needs the Visual C++ runtime, yet the table says it
   needs nothing.* Holds as a risk the runners cannot show (they have
   Visual Studio): the binaries link the C runtime in (`+crt-static`),
   as ripgrep's do.
4. *The rustup route omits the C compiler.* Holds: every source route
   names it (`build-essential`, `gcc`, `xcode-select --install`), and
   `install.yml` installs what the chapter says.
5. *Arch on ARM is untried and no job tries it.* Holds: the chapter
   offers the Arch package for x86_64 alone.
6. *The Arch `cargo install` trial is unlogged.* Partly: it ran in the
   session after the keyring setup (its output is in the session's
   record), and its log now holds it; `install.yml` tries it too.
7. *`core-sequents.md` still names the retired variants.* Holds: the
   sentence says what refuses a binder today.

Its side note, that `nix build .#dist` on the ARM runner may build
`pkgsStatic`'s compiler when no cache has it, is in "What is left
open".

### Departures and assumptions

- No `.msi` (departs from the design): WiX v7's EULA is the author's to
  accept; a WiX v5 build (MS-RL, no EULA) or v7 is the proposal.
- nfpm instead of cargo-deb and cargo-generate-rpm (departs from the
  design), and an Arch Linux package beside them.
- The plan's break list was corrected by the tools (departs from the
  design's accounting): `Placed::parent` and the record-literal
  certificates are no breaks against 0.1.0; the moved discriminants and
  the arrays' lengths are.
- `cargo binstall` is not offered, though the research recommended it.
- Linux trials ran under bubblewrap, not proot (both need no system
  change; bubblewrap was on the machine), with three trial-only
  settings a real system does not need: apt's sandbox user, pacman's
  download user, and the keyring an LXC Arch image ships empty.
- The README test's list of other programs (`TOOLS`) gained `sudo`,
  `pacman`, `gh` and `xattr`, the programs the chapter's prose shows
  with flags: the list is the test's place for them, and every flag of
  `linlog` is still checked.
- The library's `doc:` field names its Pages site before the author
  switches Pages on; its release workflow deploys the documentation, so
  the switch belongs before its tag.

### What the next sessions must know

- **Part 2 (tomorrow)**: the baseline's results and figures, README's
  and the manual's final words, and the dates. Dating means, in linlog,
  `CHANGELOG.md`'s `## [0.2.0] - unreleased` becoming `## [0.2.0] -
  YYYY-MM-DD` and `CITATION.cff`'s `date-released: "YYYY-MM-DD"` (both
  workflows refuse a tag without them); in rocq-linlog the same two, and
  the opam file's `"date:YYYY-MM-DD"` tag, which the Rocq opam archive
  asks for (it is also what 45e's file states). Both repositories' heads
  then need their checks again and a push.
- **The chapter's commands name 0.2.0's files**, so a later release
  changes the version there; `packages.yml` and `dist.nix` name the
  files by the workspace's version, and `install.yml` finds them by
  pattern or reads the latest tag, so neither needs a change.
- **`install.yml` is the instructions' test**: a change of a chapter's
  command changes it in the same commit (ci.md). Its first runs come
  with the supervisor's push of this session's commits; its `release`
  jobs and the release workflow's packaging run first at the tag.
- **Nothing of the release workflow's new jobs has run**: actionlint
  passes; the first real run is at the tag. If `packages` fails at the
  tag, the GitHub release waits (the `release` job needs it), while the
  crates are already published: fix and run the release job again (the
  publish job skips versions already on crates.io).
- **The prebuilt Linux binary is 1.26 to 1.45 times slower** than a
  glibc build (`cli.md`), stated in the chapter; a glibc-linked prebuilt
  binary built on an old-glibc runner is the follow-up.
- **`breaks VERSION`** in the devshell lists a release's breaks; every
  line is a changelog bullet or explained as none (core.md).
- **45e** takes the library's opam file from its repository at the tag,
  adds `url { src … checksum }` from the release notes' SHA-512 (the
  release workflow writes it), drops `version:` as recent submissions
  do, and runs `opam lint --check-upstream`; `nix develop .#opam` in the
  library's repository has opam and an OCaml all admitted Rocq versions
  accept.

### What is left open

- The runs of `install.yml` and `packages.yml` on GitHub after the
  supervisor's push (read below, when they have run); the release
  workflow's new jobs and `install.yml`'s `release` jobs first run at the
  tag.
- `nix build .#dist` on `ubuntu-24.04-arm` may build `pkgsStatic`'s
  aarch64 C compiler when no cache holds it, which costs time on the
  runner (the reviewer's note).
- Follow-ups: a glibc-linked prebuilt Linux binary (built on an
  old-glibc runner) for long searches; `[package.metadata.binstall]` for
  `cargo binstall`; an `.msi` once the author decides on WiX; signing.
- The channels that need an account, proposed to the author in the
  last message: a Homebrew tap (`linlog-prover/homebrew-tap`), the AUR
  (`linlog-bin`), winget (a manifest in `microsoft/winget-pkgs`, the
  `.zip` as a portable installer), a Scoop bucket, Fedora COPR; Homebrew
  core and Debian or Fedora proper later (Homebrew core asks 225 stars
  of a self-submission, Debian packages every crate).

### Checklist

- [x] Read the brief, the step's prompt, conduct.md, step 30's prompt,
  `distribution.md`, 45a's and 45b's reports, `ci.md`, `cli.md`,
  `flake.md`, `core.md`, the changelog, `CITATION.cff`, `release.yml`,
  the installation chapter, the library's CLAUDE.md and `rocq.md`, the
  `new-tool` skill; research by three sub-agents on Sonnet 5.5 at
  `medium` (install channels, release packaging, Rocq practice) and the
  author's reminders by a fourth.
- [x] The break by cargo-semver-checks and cargo-public-api
  (`breaks 0.1.0`), the command's flags by both binaries' help; the
  changelog's `## [0.2.0] - unreleased` with the break list;
  `core.md`'s lists checked (15 closed, 119 marked);
  `Error::Quantifiers` retired.
- [x] 0.2.0 in the manifests, both locks, `CITATION.cff`, the bindings
  by the workspace; the behaviour lock by the version alone; `cargo
  publish --dry-run --locked --workspace`: exit 0.
- [x] The Linux files from the flake (`linlog-static`, `dist`,
  `notices`), mimalloc on musl after two timed runs the supervisor
  cleared, `cargo deny check` ok; `packages.yml`, `release.yml` with
  `SHA256SUMS` and attestations; the chapter; `install.yml`.
- [x] Every Linux instruction in Debian 13, Ubuntu 24.04, Fedora 44 and
  Arch under bubblewrap; the cargo route with Fedora's and Arch's Rust
  and rustup on Debian; Nix in a temporary profile; the rest by
  `install.yml` (its runs below).
- [x] The Rocq library: four commits and the review's one (`2ad6487e`),
  checked with Rocq 9.0 to 9.3, the opam install per version, `nix flake
  check` exit 0 at `52e23df7` and at `2ad6487e`; "rocq ready" sent and
  answered twice (pushed); linlog's lock on `2ad6487e`; 49 certificates
  closed under the global context.
- [x] D17: `45d.csv` against `45b.csv`, 0 of 145 decided rows differ;
  `net-45d.csv` against `net-45b.csv`, 0 of 64; the ratchet within its
  ceilings in every gate, none raised.
- [x] The gate at every code commit: `tznykwpm`, `zrsxssrs`,
  `ytyxokvy`, `slwypkqv`, `knmskrro`, and `xxtlmlst` (the head, with
  `ktntzwks`'s test change), each `gate: passed`; linlog's `nix flake
  check` exit 0 at `mykmzxwl` and again at this report's commit (in
  the final message).
- [x] The fresh-context reviewer's round, each finding answered and
  fixed.
- [x] `install.yml`'s and `packages.yml`'s runs after the supervisor's
  push, read through GitHub's API: run 41446104804, every job a success
  (below).

### The runs on GitHub

The supervisor pushed linlog's `main` = `0592d5f358ef` (2026-11-02);
GitHub's public API (`/repos/linlog-prover/linlog/actions/runs/41446104804/jobs`,
read without `gh`) reported the Install run `completed success`, its 27
jobs:

- **The packages** (`packages.yml`, called): the flake's Rust version and
  the notices; Linux x86_64 and aarch64 by `nix build .#dist`;
  `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`
  and `aarch64-pc-windows-msvc` by rustup on their runners; `sums`: all
  success.
- **The chapter's packages and archive** in containers: `debian:13`
  (x86_64 and aarch64), `ubuntu:24.04`, `ubuntu:26.04`, `fedora:44`
  (x86_64 and aarch64), `archlinux:latest`: installed, ran, removed, the
  archive checked by `sha256sum` and installed and removed: success.
- **macOS and Windows**: the archives checked (`shasum`, `Get-FileHash`),
  installed, run and removed by the chapter's commands on `macos-26`,
  `macos-26-intel`, `windows-2025` and `windows-11-arm`: success.
- **Nix**: `nix run`, `nix profile add`/`remove` at the commit, `upgrade`
  of an untagged entry: success.
- **`cargo install`**: Fedora's, Arch's and Ubuntu 26.04's Rust, rustup
  on Debian 13, the runners' rustup on macOS and Windows: success.
- **From the release**: skipped on a push, as written; they run after
  the tag.

The push's Docs run succeeded; its CI run (`nix flake check` on GitHub)
was still running when this was written.

## Session 45b: families that grow

### Outcome

Twelve families with a growing set and a claim, five of them new
(`contraction`, `pigeonhole`, `euler`, `duplicate`, `reach-fo`) and seven
reused (`wide-m1`, `counter`, `mix`, `chain`, `chain-fo`, `witness-fo`,
`lambek-fo`). Each claim is written from measured rows, a fresh-context
reviewer checked every verdict by construction and every claim, and its
four findings that held are fixed. The library gained `Family::growth`
and `Family::claim` (public fields of a `#[non_exhaustive]` struct, an
addition; no enum variant). The harness gained `run --growing`, which
runs every growing set and skips a configuration's larger sizes after a
run refused or at its limit, written as `skipped` rows that no table
counts, and `run --memo-limit N`, a configuration axis (column
`memo_limit`), so that the memo's claim has its comparison line.
`bench/growing.sh` is the nights' run with each claim's comparison
passes, and `bench/comparison.sh --growing` runs the other provers on
the growing sets. `bench/plots.py growth` draws a line per engine,
configuration or prover and writes the fitted growth beside each figure;
the manual's chapter `results/growing-2026-10-31.md` shows both figures,
drawn at build time from the rows of this session's timed runs (by day,
cores 2 and 3, the runs that placed each family's sizes); the nights'
rows replace them. `--all-families` shows no mismatch; both target sets
equal 42k's counters; every code commit passed the gate, and `nix flake
check` passed at the session's last commit. The signing key lapsed
during the session and the last commits waited for the author (below,
"What is left open"); the session was paused on 2026-10-31 at 07:25 and
finished on 2026-11-02.

### The research, and what each source gave

Three sub-agents on Sonnet 5.5 at `medium` read the sources below (the
literature's hard families for linear and intuitionistic logic, ILTP's
and TPTP's parametrised classes, the other provers' own sets and
documented limits); what they could not open they marked, and nothing
below rests on what was only recalled.

- **Chaudhuri, *The Focused Inverse Method for Linear Logic*, PhD thesis,
  CMU-CS-06-162 (2006), chapter 7** (read in full): Euler tours on the
  complete graph `K_n` as `edge₁ ⊗ … ⊗ edgeₘ ⊸ a ⊸ a` with an edge `(a ⊸
  b) & (b ⊸ a)` (§7.2.2, pp. 195–196; the inverse method took 9 s, 152 s
  and 3 043 s on its three sets, the unfocused prover failing on all),
  Hamiltonian tours, blocks world, QBF in MALL, affine encodings and the
  SICS problems under two embeddings. It gave `euler`; its verdict (a
  circuit exists iff every degree is even) is this session's derivation
  from Euler's theorem, and the graph is the square of a cycle instead
  of `K_n` (below).
- **Raths, Otten and Kreitz, "The ILTP problem library for
  intuitionistic logic", JAR 38 (2007), §2.1**, and the ILTP v1.1.2
  files themselves (`nix build .#iltp`: the headers of SYJ201 to SYJ212,
  each citing Dyckhoff, "Some benchmark formulas for intuitionistic
  propositional logic", 1997, and for SYJ203 and SYJ209 Franzén's SICS
  reports R87010B and R-89/89006), and the library's comparison table
  `ILTP-v1.1.2-prop-comparison.txt` (600 s per problem, 2007: SYJ202
  fails every prover from size 7 to 10, SYJ209 every prover from 11).
  It gave `contraction` (SYJ203 and SYJ209: "proof in LJ needs n
  contractions") and `pigeonhole` (SYJ202, Cook's pigeonhole principle);
  their linear readings are this session's, argued at the generators.
- **Olarte, de Paiva, Pimentel and Reis, "The ILLTP library for
  intuitionistic linear logic", EPTCS 292 (2019), arXiv:1904.06850,
  §3–4**: the SYJ classes kept at all 20 sizes in three translations,
  none of the non-theorems SYJ207 to SYJ212 refuted by the Maude prover
  in 5 minutes; Petri-net reachability as `⊗ !(•t ⊸ t•) ⊢ M ⊸ M'`. It
  confirmed that the intuitionistic non-theorems are where refutation
  is least explored, which `contraction` #1 takes up.
- **Hodas and Miller, "Logic programming in a fragment of intuitionistic
  linear logic", I&C 110 (1994), introduction**: the exponentially many
  splits of a context under `⊗`, delayed as long as possible; the
  claim of `euler`.
- **Lincoln and Winkler's announcement of 3-Partition in constant-only
  MLL (Types list, 1992)** and the abstracts of Kanovich's Horn
  fragments: linlog's families already have both.
- **The other provers' own sources**, read at the revisions the flake
  pins (`modules/_provers/`), and runs of the comparison's binaries by
  the agent (cores 4–7, 10 to 60 s): llprover (`llprover.pl` lines
  335–347 and 421–432, and its guide: a per-path threshold of 5
  contractions and cuts, "initial value is 5"), the Maude ILL prover
  (`ILL-Prover-flat.maude` lines 142, 286–310, and the LLTP script's
  copy limit 5), APLL (`prover.ml:10`, a bound of 3; `foc_ill_bwd.ml`,
  copies refilled once per round, `2^|Γ|` splits), Click & coLLecT
  (`focused_proof.ml` lines 460–476), linTAP (`lintap.pl` lines 56–58),
  lolli (`lolli-prove/src/search.rs` at `cb09efc`, lines about 293–352:
  a clause of the unrestricted zone used at most twice on a branch),
  LinearOne (`mill1.pl`, MILL1 only) and Sympli (its README). It gave
  `duplicate`, `!(a ⊸ a ⊗ a), a ⊢ a ⊗ … ⊗ a`, which the agent ran: lolli
  fails from 4 factors, APLL from 5, llprover and the Maude prover from
  7, Click & coLLecT times out from 8, Sympli proves 6 and 12.
- Not used: Tarau's generators of implicational formulas
  (arXiv:2009.10241, 1910.01775), which enumerate rather than grow;
  TPTP's SYN007 (Pelletier 71), classically valid and intuitionistically
  not, which needs the additive reading of `⇔`; Heijltjes and Houston's
  MLL with units, a question of proof equivalence, not provability.

### The families and their claims

Each family's verdicts are argued at its generator; "comparison" is the
line its claim is set against in the figure. Sizes are the growing set
(`Family::growth`), the ceiling the size where every line that admits
the family has met the 60 s limit or a bound in the timed run, or a
stated ceiling.

| family | claim (the line) | comparison | verdicts by construction | growing set, ceiling |
|---|---|---|---|---|
| `wide-m1` (reused) | the net engine past the focused engine's memory bound, to which the default hands over: 16 384 pairs in 35 s, quadratic | the focused engine (memory bound at 4 096); the inverse method forced is faster still (32 768 in 47 s), and no row gives it a turn | provable: a proof's identification of atoms keeps it a proof | 256 to 32 768, doubling; every line at the limit or memory by 32 768 |
| `counter` (reused) | the Horn engine, linear (262 144 tokens in 0.09 s) | the focused engine forced, about cubic (256 in 0.8 s, past a minute at 1 024); the inverse method forced, 16 in 16 s | provable within log₂ n copies (doubling clauses) | 2 to 262 144; stated ceiling 262 144 tokens, where the Horn engine takes 0.1 s and the sequent's text is megabytes |
| `mix` (reused) | the inverse method's turn on Mix (1 024 pairs in 34 ms forced; the default flat at 0.3 s after its turn) | the focused engine forced, eightfold per pair (10 in 51 s) | unprovable: every linking of a pair closes a cycle through its tensors, Mix too | 4 to 4 096 (extended after the run, where the inverse method took 34 ms at 1 024) |
| `chain` (reused) | the focused engine's memo on repeated subgoals, about cubic (512 in 2.3 s) | the memo off (`--memo-limit 0`: 32 in 0.23 s, past a minute at 64); no copy bound (no extra cost); the inverse method forced (2 048 in 4 ms), which no row gives a turn | provable within k + 2 copies | 16 to 8 192 (extended); the default and the focused engine meet the recursion limit from 1 024 |
| `contraction` (new) | the inverse method on a clause used n + 1 times on a branch (256 atoms in 1.8 s forced, about n³; the default proves 16 to 128 by its turn) | the two-sided engine forced (8 in 0.07 s, past a minute at 16) | #0 provable within n + 1 copies (Franzén's derivation); #1 unprovable: its image without linearity and `!` is ILTP's SYJ209, false in a two-world Kripke model (derived here, checked by the reviewer) | 2 to 512 (extended) |
| `pigeonhole` (new) | the two-sided engine's ⊕-left case splits, about fortyfold per hole (5 holes in 2.2 s, past a minute at 6) | the inverse method forced (6 in 0.27 s), which the default gives no turn without exponentials | #0 (n + 1 pigeons) provable in affine ILL by the pigeonhole principle on every branch; #1 (n pigeons) classically false | 2 to 8, by one; #0's lines at the limit or memory by 7, #1's by 8 |
| `euler` (new) | the focused engines' context splits: on a graph without an Euler circuit (#1) about 2.4-fold per vertex (18 in 11 s) | a circuit (#0) found in under a millisecond; the inverse method forced, slower | a cut-free proof of `x, E ⊢ y` is a trail using every edge once (the reviewer checked the lemma); the square of a cycle is connected and 4-regular, without `v1v2` two vertices have degree 3 | 5 to 24 vertices, by two; #1's lines at the limit or memory by 20 |
| `duplicate` (new) | the other provers' caps on contraction: lolli gives up at 4 factors, llprover's classical system passes 10 s at 4 (the research's runs: APLL from 5, llprover and the Maude prover from 7) | linlog's Horn engine, linear (262 144 in 0.07 s); the two-sided engine forced, 256 in 0.2 s | provable within m − 1 copies on one branch, #0 in ILL and #1 the same sequent classically | 2 to 262 144; the same stated ceiling as `counter` |
| `reach-fo` (new) | the first-order memo's loop check: a cycle of 10 nodes to its exit in 20 s, about fivefold per node | the memo off: 6 nodes in 1.1 s, past a minute at 8 | #0 provable within n + 1 copies (the path; the first edge's copy ends the branch); #1 unprovable: a classical first-order model, `r` true on the cycle alone | 1 to 16; at the limit by 12 |
| `chain-fo` (reused) | n instances of one clause, about n^2.6 (384 in 1.0 s), the recursion limit at 512 | LinearOne refuses it (MILL1 only); the resumption has no switch, so no comparison line | #0 provable within n copies; #1 unprovable by a classical countermodel, ends at its copy bound | 16 to 1 024; recursion limit from 512 |
| `witness-fo` (reused) | the pool of threads on the refutation #3: 10 in 1.7 s on two threads against 2.7 s on one | one thread; LinearOne decides #3 near flat (its own run of 2026-10-30), refuses #0 and #1 | as argued at its generator (42h) | 2 to 16, by two; #3 at the limit by 12 |
| `lambek-fo` (reused) | LinearOne's fragment: linlog's two-sided search within 0.5 s to 256 formulas | the recursion limit at 512 (#0, #2) and 1 024 (#3, #4) | as argued at its generator (42h) | 8 to 2 048 (extended) |

What was measured to settle the claims, beyond the timed run: the
verdict run of every growing set under the default and every engine at
5 s (cores 4–7, 570 rows, no verdict against its construction, no error,
no crash), and probes of the candidates with the command (Franzén's
formulas, the pigeonhole, Euler tours on complete graphs and on the
square of a cycle, reachability, the duplication). The `euler` family
moved from complete graphs to the square of a cycle when the complete
graphs went from 0.19 s to past the limit in one size, too few points
for a curve.

### The harness

- `run --growing` (`bench/src/growth.rs`): the growing sets, and a
  series (family, index, mode, engine, threads; for a tool, family,
  index, mode, tool) ends at a run refused, unknown at the time limit,
  the memory, killed or crashed, or decided past the limit. Its larger
  sizes are rows of verdict `skipped` with the reason "past the limit at
  size N" or "refused at size N" and the configuration's columns, which
  `summary::read_all` drops, so no table counts them, as a timeout or at
  all (the orchestrator's conditions, 2026-10-31). A bound of the
  search's own ends nothing. Every growing set is a constructed family,
  whose larger instance is no easier; a random family gets none. Under
  `--shard`, a shard takes whole shapes (`Shard::of_groups`), so that a
  series sees its end; `--resume` rebuilds the ends from the file.
- `run --memo-limit N` (column `memo_limit`, the last of `HEADER`; the
  resume key and the summary's label `memo off`): the memo's comparison.
  The resume key's test gained the field (its commit says why).
- `bench/growing.sh`: engines-K.csv (a stream per core, by shape),
  memo-off.csv, deepening.csv and pool.csv (`--jobs all` on
  `--pool-cores`, run alone after the streams). The first timed run
  failed its pool pass at once on `--jobs 4` with two cores; the script
  now takes every pool core, and the pass was rerun (2 minutes).
- `bench/comparison.sh --growing` (and `--quick`, the first two sizes of
  each set), building only the comparison package.
- `batch-families` (the ratchet's journey) takes a list of the
  seventeen families it was set on, so that its count stays the same
  batch (`+0.33 %` at the first gate, within the 2 % tolerance).
- `plots.py growth`: rows of `run` or `run --tools`; a line per engine
  (the default, each forced), per configuration (the memo off, threads,
  no copy bound, dashed in the engine's colour) or per tool; skipped
  sizes past the limit a cross above the limit's line; a refused line
  not drawn; `growth.txt` beside the SVG, per line the sizes over 1 ms
  and the better least-squares fit of the logarithm of the time, `n^d`
  or `b^n` (`.txt`, since a `.md` under `docs/src/figures/` would read as
  an unlisted chapter to the manual's test after a local preview). The
  header above the panels is sized by the legend's rows, and the title
  pinned in inches from the top: on the tall growth figure the title had
  run into the legend.

Options a front end would set (D15): none of the library's; `growth` and
`claim` are data of the families, read by the harness and by anyone
listing them.

### The graphs

Two figures in `docs/src/results/growing-2026-10-31.md`, both listed in
`docs/figures.sh` and drawn at build time in both themes by
`plots.py growth`: the engines' (`bench/growing/2026-10-31/*.csv`, the
timed run) and the other provers' (`bench/growing/2026-10-31/provers/
runs.csv`, the first two sizes of each set), each with its fitted growth
included from `growth.txt`. `modules/manual.nix` copies `bench/growing`
into the manual's build. The chapter says what each line shows, every
number read from the rows. The light and the dark engines figure were
rendered and read here. Step 29's cactus and scatter plots were already
shown by 44b's chapter "Beside the other provers, 2026-10-15".

### The nights' machine time, by arithmetic

**The first night (45c), the engines on the growing sets**, as
`bench/growing.sh` at 60 s. Measured by day: the searches took 2 227 s in
all (engines-1 1 107 s, engines-2 833 s, memo-off 185 s, deepening 40 s,
pool 62 s); the two streams ran 22 min 20 s (stream 1, 1 332 s of search,
so process starts and generation add about 1 %), and the pool pass 1.5
min. The sizes added since (the verdict check at 5 s placed them): up to
240 s for `contraction` at 512 (the default and the inverse method on
both shapes, a minute each at most), 2 s for `mix` at 2 048 and 4 096, up
to 180 s for `chain` at 4 096 (the default, the focused engine and the
deepening pass a minute each, 8 192 then skipped), up to 125 s for
`lambek-fo` at 2 048 (#2 a minute for each of its two lines), up to 170 s
for `duplicate`'s classical shape (the focused engine to its limit near
1 024, the inverse method to its limit at 16), about 0 for `reach-fo` at
1. So about 2 227 + 720 ≈ 2 950 s, 49 minutes of one core: **about 27
minutes on two streams plus 2 for the pool, about 14 on four streams
(cores 0 to 3, the baseline's sequential cores) plus 1 for the pool on
four cores**, beside the baseline's three hours.

**What the skipping saves**: 86 sizes past a line's limit were skipped,
each at least the full minute had it run, so at least 86 minutes of one
core, 1.75 times the run itself; and 1 002 sizes after a refusal, a
child start each (a few seconds in all).

**The second night, the other provers on the growing sets**, as
`bench/comparison.sh --growing` at 60 s: 75 series (a tool on a shape
in its fragment: linlog 25, LinearOne 7, the Maude prover's ILL system,
llprover's ILL, Sympli and APLL's ILL 5 each, the Maude prover's CLL,
Click & coLLecT, llprover's CLL, lolli and APLL's LL 4 each, linTAP 3,
from the check's rows). A series costs at most its decided sizes plus
one run at the limit, and on these families a line's times grow at
least geometrically with its sizes (the fits), so its decided sizes sum
to less than about one limit: at most 120 s a series, 75 × 120 s =
9 000 s, **2.5 hours of one core, about 25 minutes on comparison.sh's
six sequential streams (cores 6 to 11 as on 2026-10-15)**; linlog's
default stream (25 series, at most 50 minutes on its cores) bounds the
night's part at **about 50 minutes**. The check by day (two sizes, 10 s,
136 runs of 12 tools in 4 minutes on two cores) agrees in scale. Step
29's library comparison with 0.2.0's engines, if the night has room, is
its own length, as on 2026-10-15.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-reviewer-45b`)
checked every growing family's verdicts by construction, `copies` and
`least`, and every claim against the engines' documented behaviour and
the verdict rows; one round, six probes on cores 6–7. It found every
argument sound (contraction's Kripke model, the pigeonhole, the trail
lemma of `euler`, the copies of `duplicate` and `reach-fo`, the reused
families) and five findings:

- **`wide-m1` at size 0 is the empty sequent, marked provable** (and
  `wide-m2` to `wide-m4`): holds. Outside every size list, older than
  this session, but a wrong verdict of a family whose claim it reuses;
  fixed: the wide families start at one pair, the refusal test pins it,
  the changelog says so.
- **`reach-fo`'s claim "#1 is refuted by the loop check on two nodes"**
  is false at the family's own copy bound (it refutes only without one,
  at level 10): holds; the claim says #1 ends at its copy bound.
- **`counter`'s claim "the inverse method forced fills its memory"**:
  holds (it runs out of time at 16 tokens, the kept set no longer
  growing); the claim says it does not finish from 16 on.
- **`chain`'s claim "the deepening repeats k + 2 levels"**: holds (the
  proof is at about k/2 + 1); the timed run showed moreover that the
  deepening costs nothing, and the claim says so.
- **`reach-fo`'s `least` is 2 where size 1, a loop, has correct
  instances**: holds; `least` is 1 and the summary says one is a loop.

The reviewer also noted the `lambek-fo` claim leaves out #3 and #4's
recursion limit at 1 024 (incomplete, not false); it now names it.

### Departures and assumptions

- **Sizes by one where the time grows exponentially** (departs from the
  design, which asks for geometric sizes): `mix`'s first sizes,
  `pigeonhole`, `euler`, `reach-fo`, `witness-fo`; one doubling there
  jumps from milliseconds past the limit. The polynomial families double.
- **`euler` on the square of a cycle, not Chaudhuri's complete graphs**
  (departs from the source): the complete graphs' edges grow
  quadratically, and the unprovable shape went from 0.19 s to past the
  limit in one size; the square of a cycle grows by two edges a vertex.
- **A series skips its larger sizes after a refusal or its limit**, as
  rows `skipped` (departs from the design's "each family up to the size
  where every engine reaches the limit", which reads as running every
  size): the orchestrator's conditions of 2026-10-31 (monotone families
  only, the rows marked, never a timeout) are met; it saves about 86
  core-minutes of the first night.
- **No family shows the copy bound's deepening as a weakness** (departs
  from the design's expectation for "a long `!` chain"): on `chain`, the
  deepening without a bound costs nothing beside the family's own bound,
  the memo answering the repeated levels; the figure shows that line
  ("no copy bound") as measured.
- **The net engine's strength is shown past the focused engine's
  limits, beside a faster inverse method** (departs from the design's
  claim "the net engine on wide unit-free MLL"): the rows show the
  inverse method forced faster on `wide-m1` at every size; the claim
  says both.
- **The resumption of answers (42j) has no comparison line**: it has no
  switch, the option never built after the author's decision of
  2026-10-30; `chain-fo`'s claim no longer names it.
- **The first-order memo's family is new** (`reach-fo`, 42i's chosen
  reachability sequents made a family) and 42i's "lifted" rows are not
  reused: they are propositional problems under a vacuous quantifier,
  whose claim would be about the first-order zone's overhead.
- **The plan names llprover's "contraction cap of three"**: llprover's
  guide says its threshold is 5 (per path, contractions and cuts) and
  APLL's bound is 3; the claim of `duplicate` follows the sources.
- **The other provers ran on the first two sizes only** this session,
  at 10 s: their curves are the second night's; the chapter says so.
- **The figures are drawn at build time, not committed as SVG**: the
  brief's correction of the plan, 44b's practice.
- The public API: `Family::claim` and `Family::growth` added to a
  `#[non_exhaustive]` struct (an addition); `Family::least` of the wide
  families 0 → 1 (a wrong verdict fixed). No variant of a closed enum.
  The harness's `HEADER` gained `memo_limit` (an older file is resumed
  into a new one, as for every column added).

### What the next sessions must know

- **45c, the first night**: `bench/growing.sh --out bench/growing/DAY
  --cores 0,1,2,3 --pool-cores 0-3 --timeout 60` beside or after the
  baseline (about 15 minutes so, arithmetic above), from a build of the
  release's commit; then a chapter `results/growing-DAY.md` and two lines
  in `docs/figures.sh`, as this session's, and this session's chapter
  kept beside it as the placing run, or replaced, as 45d decides.
- **The second night**: `bench/comparison.sh --growing --cores
  6,7,8,9,10,11 --default-cores 12-15 --timeout 60 --out
  bench/comparison/DAY-growing` (about 50 minutes, bound by linlog's
  default stream); `ranking` and `plots.py comparison` and `growth`
  run inside it; its `runs.csv` is what the provers' figure of a night's
  chapter draws.
- **A claim is read from rows, never from expectation**: six first
  drafts were wrong (`.claude/rules/core-inputs.md`). When a night's
  rows change a claim's numbers or direction, the claim changes in
  `core/src/families.rs` and in the chapter.
- **New families add no rows to the target sets**, which name their
  families; `batch-families` names its seventeen families.
- **Findings for the dispatch** (in `plan/later.md`): the inverse method
  forced beats the default on `wide-m1`, classical `chain` and
  `pigeonhole`, where no row gives it a turn; `pigeonhole` #1's classical
  refutation comes only after the backward search ends.
- **Every `nix flake check` and large `nix build` takes `--max-jobs 2
  --cores 6`** (the author, through the orchestrator, 2026-11-02): the
  daemon's builds otherwise take every core, the desktop's too.

### What is left open

- **The signing key lapsed at 06:40 on 2026-10-31**, and the permission
  classifier of this session refused `jj --config
  signing.behavior=drop`, which the brief names for that case; the last
  commits waited until the author unlocked the key (about 07:10), and
  every commit of the session is signed.
- The curves of the other provers and the nights' curves of the engines.
- The dispatch findings above, as follow-ups.

### Checklist

- [x] Research by three sub-agents on Sonnet 5.5 at `medium`, the
  sources named above.
- [x] Twelve families with a claim and a growing set, five new, each
  verdict argued at its generator; `verdicts_as_constructed` covers the
  smallest sizes.
- [x] `run --growing` under the default and every engine (the verdict
  run, 570 rows; the timed run, 1 762 rows), the comparison passes
  (`--memo-limit 0`, `--copies none`, the pool), the other provers
  through `bench/comparison.sh --growing --quick` (600 rows, 12 tools).
- [x] `--all-families --timeout 5`: no mismatch.
- [x] The graphs drawn at build time by `plots.py` in both themes,
  listed in `docs/figures.sh`, the fitted growth beside each, the
  chapter `results/growing-2026-10-31.md`.
- [x] The nights' machine time by arithmetic.
- [x] The fresh-context reviewer's round, each finding answered.
- [x] D17: `45b.csv` against `42k.csv`, 0 of 145 decided rows differ;
  `net-45b.csv` against `net-42k.csv`, 0 of 64; the ratchet within its
  ceilings in every gate, `bench/ceilings.csv` untouched.
- [x] The gate passed at each code commit (`lksmzxlu`, its rustfmt-only
  amend after; `wyvuxykp`, `kwrssksr`, `ttktylyk`, `vukmwwzr`).
- [x] `.claude/rules/bench.md` and `core-inputs.md` amended; the
  changelog, the manual's benchmark chapter and status chapter true.
- [x] The rows, the chapter, the figure layout fix and this report
  committed after the author unlocked the key (`oklltymu`, `mknqmnqy`,
  `orttrvur`, `vuvykpvu`, all signed); the layout fix's tree gated
  before (`gate: passed`, 07:05).
- [x] `nix flake check` at `vuvykpvu`: exit 1, two checks failing at
  their cause in this session's changes, fixed in `lpxnmypo`: `bench`
  (the check made every line of `linlog-bench families` an argument,
  and a growing set's indented line broke it; its `sed` now prints only
  the families' lines, all 25 families as before; its script run here
  in bash: passed) and `typos` (the atom `pn` of `contraction`'s summary,
  now `p(n)`). The run stopped at the first failure, so the other checks
  were not all seen.
- [x] Paused by the orchestrator on 2026-10-31 at 07:25 (the author
  needed the machine) and resumed on 2026-11-02 from this checklist.
- [x] The gate of `lpxnmypo`'s tree (`gate: passed`, 2026-11-02).
- [x] `nix flake check -L --keep-going` under the machine lock at
  `kpomtxsx`: exit 0; and again at the report's last commit with
  `--max-jobs 2 --cores 6` (below, in the final message).
- [x] The final message to `linlog-orchestrator`.

## Session 45a: the Rocq library in its own repository

### Outcome

The Rocq library `rocq-linlog` (logical path `Linlog`) is a repository
of its own, `linlog-prover/rocq-linlog` (`../rocq-linlog`), started with
one signed commit, `vqnnvkms` (`ce2ea757`), of the tree that stood
under `rocq/` at linlog's `pqyxnxkk` (`31f36a5d`), with no history
carried over; the planning session pushed it as that repository's
`main`. Its `nix flake check` passed (exit 0) under the machine lock:
the library built as nixpkgs builds a Rocq library, every certificate
under `tests/` compiled in silence, `tests/Assumptions.v` printed
`Closed under the global context` 23 times (once per `Print
Assumptions` line), and the bridges' check, rebuilt to show its
output, printed it 4 times for `bridges/Assumptions.v`. linlog no
longer holds the library: it takes it as the flake input `rocq-linlog`
(`github:linlog-prover/rocq-linlog`, locked to `ce2ea757`, following
linlog's `nixpkgs`, `flake-parts`, `import-tree`, `treefmt-nix` and
`nanoyalla`), and its check `rocq-linlog` compiles every certificate
the exporter writes for `--kernel linlog` against that package. The
NanoYalla export check `rocq` is the very derivation it was at
`pqyxnxkk` (`w8qprsl4…-check-rocq.drv` in both), and the library
linlog now compiles against is the very derivation it built from
`rocq/` before (`z1ngc80s…-rocq9.1-linlog-0.1.0.drv` at `pqyxnxkk`, in
the new repository and through the input). linlog's commits:
`xrrqvktq` (the `--kernel linlog` help names the new repository; the
behaviour lock's three help texts change with it, a commit of its own
as CLAUDE.md asks), `lnmmlutv` (the move), `luwqknxt` (D20 and D22's
notes), `qkpvrkpz` (the update-deps skill, the reviewer's finding) and
`mrumwypt` (the check `rocq-linlog-library`, the planning session's
request, below). The gate passed at the move's tree (`gate: passed`);
linlog's `nix flake check` at the report's commit is in the final
message to the planning session.

### What moved and what stayed

| what | before | after |
|---|---|---|
| the library `theories/` (18 files), `_RocqProject`, `Makefile` | linlog `rocq/` | rocq-linlog, byte for byte |
| `rocq-linlog.opam` | linlog `rocq/` | rocq-linlog: `homepage`, `bug-reports`, `dev-repo` name the new repository, `build`/`install` without `-C rocq`, the AI note worded for the library; version still 0.1.0 |
| `README.md` with the logo | linlog `rocq/` | rocq-linlog: the build from the root and `nix develop`, the load path `path/to/rocq-linlog/theories`, where the library came from |
| the tests `tests/*.v` (9 files) | linlog's check `rocq-linlog` | rocq-linlog's check `rocq-linlog`, byte for byte |
| the bridges `bridges/*.v` (3 files), inputs `nanoyalla` and `yalla` | linlog's check `rocq-bridges` | rocq-linlog's check `rocq-bridges` (the same derivation as linlog's was), inputs pinned at linlog's revisions |
| the grep for `Admitted`, `admit`, `Axiom`, `Parameter`, `Conjecture` | linlog's two checks, over theories, tests, bridges | rocq-linlog's two checks, the same greps over the same files |
| `Print Assumptions` closed, counted per line | linlog's `rocq-linlog` and `rocq-bridges` | rocq-linlog's `rocq-linlog` and `rocq-bridges` |
| the package `rocq-linlog` (mkRocqDerivation) | linlog `modules/rocq-linlog.nix` | rocq-linlog `modules/rocq-linlog.nix` (`packages.rocq-linlog` and `default`, `meta.homepage` added); linlog builds it through the input |
| `.claude/rules/rocq.md` | linlog | rocq-linlog, its `paths:` the library's files, the references to `rocq/` and linlog's paths adapted |
| the 34 exporter snapshots `core/tests/snapshots/linlog/*.v` | linlog's `rocq-linlog` | linlog's `rocq-linlog`, against the pinned input |
| the 15 certificates the command writes (Mix, affine, intuitionistic, intuitionistic affine, the `⊤`/`0` goal, classical with `--kernel linlog`, four refutations, five first-order with the session's) and the planted wrong witness that must be refused | linlog's `rocq-linlog` | linlog's `rocq-linlog`, unchanged lines |
| the library's own check (tests, assumptions closed), seen from linlog | none: the library's checks were linlog's own | linlog's `rocq-linlog-library`, the input's `checks.SYSTEM.rocq-linlog` re-exported, the same derivation as the library's repository builds |
| `export::rocq::LINLOG` equal to the library's version | asserted against `rocq/rocq-linlog.opam` | asserted against the pinned package's `version` (read by its flake from its opam file) |
| the NanoYalla export check `rocq` (`modules/rocq.nix`) | linlog | linlog, the same derivation |

No certificate test is lost: each of the old checks' steps is in one of
the two repositories' checks, as the table says. What the two `rocq-linlog`
checks no longer share is one directory: before, the tests and the
exporter's certificates were compiled in one derivation; now each
repository compiles its own.

### The new repository

- **Form**, by the Rocq community's current practice as read on
  2026-11-02 by a research sub-agent (Sonnet 5.5, `medium`):
  `rocq-community/aac-tactics` (updated 2026-11-01) keeps
  `rocq-NAME.opam`, `_RocqProject`, `theories/`, `tests/`, a thin
  `Makefile` over `rocq makefile`, `CHANGELOG.md` and `LICENSE` at the
  root, its opam file `build: [make "-j%{jobs}%"]` and `install: [make
  "install"]`, which the library now has; the rocq-community templates
  offer a Nix CI beside the Docker one, so a flake-only CI is within the
  practice. The 9.0 and 9.1 reference manuals still name
  `_CoqProject`; the Makefile passes `-f _RocqProject` explicitly, so the
  name works with both (the check builds with nixpkgs' Rocq 9.1).
- **Flake**, in linlog's dendritic style: `flake.nix` declares the
  inputs (nixpkgs' channel tarball, flake-parts, import-tree,
  treefmt-nix, `nanoyalla`, `yalla`), `modules/` holds
  `rocq-linlog.nix` (package, check, a devshell with Rocq and the
  standard library and `ROCQPATH`), `rocq-bridges.nix`,
  `conventions.nix` (`conventions`: the licence header on every `.v`,
  `.nix`, `.toml`, `.sh`, `.py`, `.yml`, the Makefile, the opam file and
  `.envrc`, and no comment or README line naming the plan; `typos`,
  `typos.toml` with `falso`, `implicits`, `itens`; `deadnix`;
  `actionlint`), `treefmt.nix` (nixfmt, taplo, shfmt, shellcheck,
  ruff-format), `claude-hooks.nix` (linlog's, byte for byte) and
  `systems.nix`. Its `flake.lock` holds linlog's locked nodes for those
  six inputs, copied from linlog's lock by a script and accepted by
  `nix flake lock` unchanged, so both repositories build one Rocq.
- **CI** (`.github/workflows/ci.yml`): `nix flake check` on pushes to
  `main`, pull requests, weekly and by dispatch, with linlog's pinned
  actions (`actions/checkout`, `cachix/install-nix-action`,
  `cachix/cachix-action` by SHA), pulling from Cachix `linlog-prover`
  and pushing from `main` only once the author adds the secret
  `CACHIX_AUTH_TOKEN` (CLAUDE.md says so); `.github/dependabot.yml` as
  linlog's.
- **`.claude/`**: `rocq.md` as its rules, linlog's jj-only guards and
  their shared module byte for byte, `format.sh` with the formatters'
  suffixes of this repository, `session-jj-state.sh`, and
  `settings.json` with linlog's env, the read-only allow rules that
  apply, `ask` on `jj git push` and `gh`, `deny` on git and hand edits of
  `LICENSE` and `flake.lock`. **CLAUDE.md** says what the library is,
  that linlog depends on it and what a change of a statement breaks
  there, the commands, the flake, CI, jj, the conventions and the setup.
- **The changelog** (`CHANGELOG.md`, Keep a Changelog) has an
  "Unreleased" section that says where the library came from (linlog's
  `rocq/` at `31f36a5d` on 2026-11-02, its history linlog's) and what
  changed in the move; linlog's changelog says where it went.

### Decisions

- **The version stays 0.1.0** in the opam file and in
  `export::rocq::LINLOG`: the step's prompt leaves the raise to 45a and
  45d, and the brief gives the 0.2.0 release to 45d, which raises both
  together when it releases the library (linlog's flake asserts that
  they agree); a version raised here would name a release that does not
  exist yet.
- **linlog's check keeps the name `rocq-linlog`**, since it still
  compiles the `--kernel linlog` certificates; CLAUDE.md, the devshell's
  `check` help and the verification table name it so. linlog no longer
  has the package `rocq-linlog` (`nix build
  github:linlog-prover/rocq-linlog` builds it), nor the check
  `rocq-bridges`.
- **The input follows linlog's `nanoyalla`**, so one pin of NanoYalla
  serves the export check and the library's bridges; linlog no longer
  declares `yalla`, which stays in the lock as the library's own input
  (never fetched by linlog's evaluation, which reads only the package).
- **The library's flake has no devshell input**: a `mkShell` with Rocq
  serves `make`; linlog's numtide devshell is for its many commands.

### Departures and assumptions

- **The library's rules do not mention linlog's `rocq-linlog-library`**
  (departs from the design: the planning session asked, after the push,
  to say so in `rocq.md` too). `rocq.md` moved to the library's
  repository, whose one initial commit is pushed; a second commit there
  was not this session's to make once the goal named one. linlog's
  `flake.md` and `core-export.md` say it, and the library's CLAUDE.md
  already says that linlog compiles against its package. 45d adds the
  line to `rocq.md` with its first change there.
- **The new lock was written from linlog's**: the six locked nodes
  copied by a script from linlog's `flake.lock`, then `nix flake lock`
  accepted the file unchanged and `nix flake metadata` showed every
  input at linlog's revision. Locking from scratch would have taken the
  channel's newest nixpkgs and another Rocq build than linlog's.
- **The library keeps the licence header `linlog © …`** on its files,
  unchanged, since the library is part of the linlog project; its
  conventions check requires that header.
- **`.claude/` of the new repository has no `claude-infra.md`**: its
  CLAUDE.md carries the setup's few points (the guards are copies of
  linlog's, the hooks fail open), since one rules file serves a
  repository of one area.

### The reviewer

A fresh-context reviewer on Opus 5.5 at `high` (`linlog-review-45a`),
told to report only what breaks a stated requirement, judged five:
nothing of the library lost (every file, every certificate compiled
somewhere, the greps, the assumptions counted), the trusted base as
`rocq.md` lists it and unchanged, linlog's input and checks, no stale
reference to `rocq/`, the new repository's flake and CI. It compared
the 34 files with `cmp`, the old and new modules by `diff`, the locks'
nodes by a script, and the package's derivation in all three places.

1. *`.claude/skills/update-deps/SKILL.md` still said that `nanoyalla`
   and `yalla` are pinned in `flake.nix`* (requirement 4, outside the
   listed paths). Holds: fixed in `qkpvrkpz`, which says the library's
   `yalla` moves with that repository's lock and that `nix flake update`
   moves `rocq-linlog` to its `main`.

Its side notes, answered: the passing build of linlog's `rocq-linlog`
in `linlog-rocq.log` was against the command before the `--kernel`
help's commit; the final `nix flake check` builds it against the
command of the last commit. It counted sixteen certificates the command
writes; the check has fifteen `linlog prove`/`linlog interact` lines
(the sixteenth file is the planted fault). Requirements 1, 2, 3 and 5
held in its reading.

### What the next sessions must know

- The library is `../rocq-linlog` (`linlog-prover/rocq-linlog`), with
  its own CLAUDE.md and `.claude/rules/rocq.md`; linlog's
  `core-export.md` and `flake.md` say how linlog takes it.
- A change of the library's statements, rules, record fields or proof
  terms is made, checked (`nix flake check` there) and pushed in its
  repository first (`plan/push-main.sh --rocq REV`); then
  `nix flake update rocq-linlog` in linlog moves the pin, with the
  exporter's change in the same commit. `--override-input rocq-linlog
  path:../rocq-linlog` tries an unpushed change.
- 45d: the version is 0.1.0 in `rocq-linlog.opam` and in
  `export::rocq::LINLOG`; raise both to 0.2.0 together (linlog's
  flake asserts they agree, through the pinned package's `version`).
  The opam file's bounds, `doc`, `opam lint`, a release workflow, its
  own `CITATION.cff` and the README's install instructions are 45d's.
- The new repository's push to Cachix waits for the author's secret
  `CACHIX_AUTH_TOKEN`; until then its CI downloads only.
- The guards under `.claude/hooks/` are copies in both repositories; a
  change to one belongs in both (each `claude-infra.md`/CLAUDE.md says
  so).
- The rocq-linlog repository's lock pins linlog's revisions (nixpkgs
  of 2026-10-20) and moves on its own with "flake.lock: Update" commits;
  linlog's input follows linlog's nixpkgs whatever that lock says.
- linlog's check `rocq-linlog-library` is the library's own check
  re-exported: a pin whose library check fails fails linlog's `nix flake
  check`. 45d adds a line saying so to the library's `rocq.md`.

### What is left open

- The client's About does not link the library (checked: the client's
  `src/` names no `tree/main/rocq` and no `rocq-linlog`), so nothing
  changed there.
- 45d's work on the library's release, listed above.

### Checklist

- [x] Read the brief, the step's prompt, conduct.md, `rocq.md`,
  `flake.md`, `ci.md`, `core-export.md`, `claude-infra.md`, the three
  Rocq modules, the CI workflow, the changelog, `CITATION.cff`, the
  manual's Rocq chapter, the guide, the `new-tool` skill; the Rocq
  community's practice through a research sub-agent (Sonnet 5.5,
  `medium`).
- [x] `../rocq-linlog`: `jj git init --colocate`, `origin` =
  `git@github.com:linlog-prover/rocq-linlog.git`, one signed commit
  `vqnnvkms` (`ce2ea757`): the library, tests, bridges, opam file,
  README, flake modules, lock, CI, Dependabot, `.claude/`, CLAUDE.md,
  LICENSE, CHANGELOG.md, typos.toml, `.gitignore`, `.envrc`.
- [x] Its `nix flake check` under the machine lock: exit 0 (and again
  at the commit: exit 0); assumptions closed, 23 lines and 4.
- [x] "rocq ready" to `linlog-orchestrator`; its answer: pushed,
  `main` = `ce2ea75761a0`.
- [x] linlog without `rocq/`, `modules/rocq-bridges.nix`, the input
  `yalla` and `.claude/rules/rocq.md`; the input `rocq-linlog` locked
  at `ce2ea757` (`nix flake lock`, nothing else moved); the check
  `rocq-linlog` against it built (exit 0); `rocq` the same derivation.
- [x] Every certificate test placed (the table above).
- [x] CLAUDE.md, README, the manual (`rocq.md`, `status.md`,
  `installation.md`), `CITATION.cff` (the abstract and a `references`
  entry), the rules (`flake.md`, `core-export.md`, `core.md`,
  `claude-infra.md`), `plan/README.md`'s D20 and D22 with dated notes,
  the guide (`SKILL.md`, `reference.md`; `cargo test -p linlog-cli
  --test readme`: 7 passed), both changelogs.
- [x] The `--kernel` help and the behaviour lock in a commit of their
  own (`xrrqvktq`; `cargo test --test lock` passed after `BLESS=1`).
- [x] The gate at the move's tree: `gate: passed`.
- [x] The fresh-context reviewer's round, its one finding fixed.
- [x] The planning session's request: `rocq-linlog-library` (`mrumwypt`).
- [x] This report committed; linlog's `nix flake check` at its commit
  and the final message to `linlog-orchestrator` follow.
