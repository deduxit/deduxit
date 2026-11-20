# Step 72: problems with verified answers for evaluating models

One session (72) on Opus 5.5 at `high`, keeping the conversation of 54x7
and 54x8; a research sub-agent on Sonnet 5.5 at `high`; `fresh-reviewer`
on Fable 5.2. Cores 10 to 12.

## Outcome

- **`deduxit grade SEQUENT`** (`rnulnstu`; the lock `xwrotovo`): one
  call that reads a model's or a student's answer to a problem and
  answers with the command's exit statuses, 0 correct, 1 incorrect (an
  answer that does not read too), 3 unknown, 2 for the problem's or the
  flags' faults. The problem is the sequent and its logic as `prove` and
  `interact` take them (mode flags, `--logic`, `--system`, `--calculus`
  with `--assume`); the answer comes from standard input or `--answer
  FILE`, its kind read from its shape:
  - a JSON document as `check` reads one: a proof or a refutation of
    linear logic, an outcome or a bare derivation of ordinary logic, a
    modal outcome, a Kripke model (correct where it refutes the
    sequent), a derivation in a calculus given as data;
  - a signed tableau (first line a number), for classical logic;
  - otherwise a session script of `interact`, which must finish the
    proof by its own steps: `close`, `invertible`, `save` and `load`
    are refused, the
    commands that only show are passed over, the first line that failed
    is named.
  A document must be of the problem's sequent in its logic before its
  check runs. `--format json` writes `{"version", "verdict", "answer",
  "message", "line"?}` (the schema's new form `graded`).
- **The dataset** (`lrkppzkz`, with `deduxit-bench instances`,
  `vnnwskvx`): `dataset/problems.jsonl.gz`, 843 problems in every suite
  that has a source whose licence allows it, each with the verdict
  deduxit decided under a bound of work and the answer `grade` accepted;
  the card `dataset/README.md`, `croissant.json`, `SHA256SUMS` and the
  script `dataset/build.py` that builds it from the command. Nothing is
  uploaded.
- **The flake check `dataset`**: the checksum, then `build.py verify`
  decides every tenth problem again with the built command and grades
  its stored answer.

## The research

A sub-agent on Sonnet 5.5 at `high` read the current harnesses' and
cards' documentation (2026-11-20). What it found, with its sources:

- **Record format.** JSON Lines is the common denominator: Inspect
  (`inspect_ai` v0.3.291, 2026-11-17) reads `json_dataset()` with the
  fields `id`, `input`, `target`, `metadata` and no field mapping
  (https://inspect.aisi.org.uk/datasets.html); lm-evaluation-harness
  (v0.4.13) reads local JSON through `dataset_path: json` with
  `data_files`, `doc_to_text`, `doc_to_target`
  (https://github.com/EleutherAI/lm-evaluation-harness/blob/main/docs/new_task_guide.md);
  OpenCompass, promptfoo and Hugging Face's `datasets` read JSONL too;
  OpenAI's `evals` takes `input` and `ideal` and accepts no custom code;
  `simple-evals` is deprecated since July 2025.
- **External grading.** Inspect's custom `@scorer` returns a `Score`
  (https://inspect.aisi.org.uk/custom-scorers.html); lm-eval calls a
  callable `process_results(doc, results)` from the task's config
  (`lm_eval/api/task.py`). Neither has a standard for "a dataset with a
  verifier command"; the provers' benchmarks (PutnamBench, miniF2F)
  ship the verifier beside the data, not in the records.
- **Cards.** Hugging Face's card is `README.md` with YAML front matter
  (`license`, `pretty_name`, `task_categories`, `size_categories`,
  `annotations_creators`, `configs` with `data_files`), which works from
  any git repository (https://huggingface.co/docs/hub/datasets-cards,
  https://huggingface.co/docs/hub/datasets-manual-configuration); its
  body follows the questions of Datasheets for Datasets (Gebru et al.,
  arXiv 1803.09010). Croissant 1.1 (2026-01-29,
  https://docs.mlcommons.org/croissant/docs/croissant-spec-1.1.html) is
  cheap and NeurIPS 2026 asks it of self-hosted datasets
  (https://neurips.cc/Conferences/2026/EvaluationsDatasetsHosting).
- **Contamination and versions.** BIG-bench's canary convention
  (https://github.com/google/BIG-bench/blob/main/docs/doc.md): a GUID of
  one's own in the card and the data; a stated version and checksums.
- **Licences.** ILTP (https://iltp.de) states no licence; Creative
  Commons asks that third-party material be marked
  (https://creativecommons.org/faq/). No source gives guidance for
  problems whose source states no licence.

What the dataset took from it: Inspect's field names (so Inspect reads
it with no mapping and lm-eval with two lines of YAML), JSON Lines, a
Hugging Face card whose body follows the datasheet's questions,
Croissant 1.1 beside it, a canary GUID of its own in the card and in
every record's `metadata` (a canary line of its own would be read as a
problem), a version and `SHA256SUMS`, and the grading command named in
the card with the two harnesses' hooks. The research's own report is
kept in the session's transcript; its wording came through a
summarising fetch tool, so a field list is to be checked against the
linked page before it goes into a spec.

## The dataset and its card

- **Sources**: the generated families, each at its smallest default
  size (41 instances, `deduxit-bench instances`); the harness's problem
  files (`bench/problems`: cyclic linear logic, the Lambek calculus, the
  Partition tables); the web client's stores (452 linear and 62
  ordinary problems, CC BY 4.0, read from the client's repository with
  `--client`); sixteen modal schemas in each of the fifteen systems.
  LLTP stays out (GPL-3.0), and so does ILTP (no licence): `--iltp DIR`
  builds `iltp.jsonl` beside the data for local use, never committed.
- **What a record holds**: `id`, `input` (the question, naming the
  logic, asking for one word), `target` (`provable`/`unprovable`,
  `valid`/`not valid`), and `metadata`: suite, logic, the command's
  flags, the sequent, the verdict, the evidence (deduxit's own JSON
  answer) and its kind, family, source, source licence and URL, the
  version of deduxit, the canary.
- **How it is made**: `prove --format json --deterministic --timeout
  none --work-limit 2000000`, a bound of work in place of a time limit,
  so the answers are the machine's no more than the file's; then `grade`
  of that answer, the problem kept only where it says `correct`; a
  verdict against the family's construction or the source's stated one
  stops the build. Of 938 problems, 843 are in: 7 were unknown at the
  bound, 88 unprovable sequents whose refutation is an exhausted search,
  which has no check (29 in the Lambek calculus, 25 classical, 18
  cyclic, 16 intuitionistic). Two builds of the same commit give the
  same bytes.
- **Where it lives**: in the repository, `dataset/`, compressed (2.5 MB
  of JSON Lines, 369 KB as gzip, as OpenAI's HumanEval ships its data;
  Hugging Face's loader and lm-eval through it read `.jsonl.gz`, and the
  card says to `gunzip` it for Inspect). jj refuses a new file over
  1 MiB by default, which the plain file is; a release asset would have
  left the flake check nothing to verify against. The evidence is three
  fifths of the plain bytes (1.4 MB), which a proof's JSON makes.
- **Licence**: the card puts the dataset under CC BY 4.0, the licence of
  the client's stores, and says the code (the script) is EUPL-1.2.

## The grading mode

`grade` builds on `check` and `interact` rather than beside them:

- a check's output is kept in memory (`io::Sink::Held`): `check`'s own
  functions run as they do, and their verdict line is the message, so a
  check that `check` refines later grades the same way;
- `check_ordinary` takes a sink in place of its path; `check --tableau`'s
  review is a function of its own (`tableau::review`); `interact`'s loop
  takes `Input::Stdin` or `Input::Script` and returns `Ran` (finished,
  commands run, the first failure), and `interact::session` starts any
  of the three sessions from arguments, which `grade` builds from
  `interact`'s defaults through clap;
- the sequent comparisons are by what each document can say: a linear
  sequent by its `Display` and mode; an ordinary one by each side's
  formulas as a multiset of texts (outside classical logic `Γ ⊢` is
  `Γ ⊢ ⊥`, as the logic decides it); a modal one by text and system; a
  rules derivation by its root's text in the calculus and its
  assumptions among `--assume`'s, each written by the calculus.

## Checks

- As the work went: clippy on the command and the harness with
  `--deny warnings`; the command's tests (the new
  `grade_reads_each_kind_of_answer`, the settings' tests with `grade`
  added, the behaviour lock with its new calls, README's, the manual's
  and the guide's examples and names, the summary's budget, which
  `grade` fits by a shorter library paragraph); the harness's tests; the
  schema check's documents with the two new `graded` ones, validated
  with `jsonschema` (0 failures).
- The dataset: built twice from the same commit, byte for byte the same
  (`problems.jsonl.gz` and `croissant.json`); `build.py verify` on it,
  85 of 843 decided and graded again, 0 failures; the reviewer's own
  sample of every 17th, 50 problems, 0 failures, and a planted flipped
  target and swapped evidence reported.
- The gate (`nix develop -c gate`, cores 10 to 12) at `puvyozky`, after
  the review's fixes: passed. The ratchet's note on the cache model's
  misses of `search-chain-128` and `check-qbf-20-2` is no failure (the
  journeys' code is unchanged by this step).
- `nix flake check --keep-going` under the machine lock (`--max-jobs 4
  --cores 8`) at `puvyozky` with this report in the working copy: all
  checks passed, `dataset` among them ("85 of 843 problems decided and
  graded again, 0 failures"), `wire-schema` with the form `graded`.

## The reviewer

`fresh-reviewer` on Fable 5.2 read the chain to `lrkppzkz` against the
brief and the prompt, and reproduced each finding with the built
command. Every finding is fixed in `ynxxsstx` (the help's lock in
`puvyozky`):

1. **A derivation was graded in a logic it does not prove** (unsound):
   an LK derivation of `⊢ a ∨ ¬a`, bare or inside an outcome whose
   top-level `logic` was changed, was correct for an intuitionistic or a
   minimal problem, and an LJ derivation of ex falso for a minimal one.
   Only the sides were compared and the derivation checked in its own
   calculus. Now a derivation proves only in its own logic and the
   stronger ones (minimal, intuitionistic, classical, `proves_in`), and
   the CLI test pins it. No row of the dataset was affected.
2. **`--file` lost the file's format**: the problem was put back as
   text, so a `.json` sequent or a `.p` problem did not read, and a
   right answer was graded incorrect once. Now a file stays a file (its
   format from its name or `--input-format`), only standard input is
   read once as text, and a model's check reads the parsed sequent's
   text (the ordinary and modal parsers read their own printing).
3. **`invertible on` took every step**: a one-line script finished any
   LK proof. It is refused as `close` is, and the setting
   `interactive.invertible` is off in a graded session whatever the
   settings file says.
4. **Texts**: exit 3 also means an answer with nothing to check (a
   refutation only the search vouches for), now said in the help, the
   manual, the guide and the card; the card's shell line broke on flags
   of none or two words, so it gives a Python call with the flags as
   separate arguments; the card named the problem files as cyclic and
   Lambek only (eight rows are Partition's tables in classical linear
   logic). Its note that `build --iltp` into `dataset/` would leave
   `iltp.jsonl` for jj to snapshot is met by an ignore rule.

It found sound: every other sequent comparison (linear by mode and
text, modal by system and text, rules by root, label and assumptions,
atom names holding a connective refused on reading), the Kripke models'
checks (persistence, cycles, one world classically, the frame
conditions per system), the tableau path, `show` and `proof` skipped in
a script, the exit statuses, the refactors of `check`, `check --tableau`
and `interact` (the lock and README's examples pass), and the dataset's
rows (unique ids, `target` equal to the evidence's verdict, no ILTP or
LLTP row, the counts and checksum; `verify` catches a flipped target and
a swapped evidence).

## Departures

- The grading mode is a subcommand of its own, `grade`, rather than a
  mode of `check` or of `interact` (departs from the design: the prompt
  says it "builds on `interact`'s session script and `check`"). It does
  build on them, running their code; one call that takes every kind of
  answer, the problem given as `prove` takes it, is the current CLI
  idiom (a verb per task), and `check`'s FILE and `interact`'s standard
  input each mean one kind of document.
- The data file is committed compressed (`problems.jsonl.gz`), not as
  plain JSON Lines (departs from the design, which named neither): jj
  refuses a new file over 1 MiB by default, and the plain file is
  2.5 MB. Inspect reads it unpacked.
- The calculi given as data have no problems of their own in the
  dataset (departs from the design: "problems in every suite"):
  deduxit writes no derivation in a rule file by itself, so no answer
  of that suite can be verified without a hand-written one. `grade
  --calculus` grades a model's derivation or session in one.

## For the author

- **ILTP's problems**: the library states no licence (its site, checked
  by the research), so the dataset leaves them out and `build.py --iltp`
  builds them for local use. If the author wants them in, the way is a
  word from ILTP's maintainers recorded in the card.
- **The licence of the dataset**: CC BY 4.0, chosen as the client's
  stores' (the author's own content); CC0 would also do for the
  generated part.
- **A hub upload** (Hugging Face, Zenodo) is the author's act; the card
  is ready for Hugging Face as it is, `croissant.json` for NeurIPS's
  self-hosted route.
