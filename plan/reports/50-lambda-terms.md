# Step 50: proofs read back as lambda-terms

## Session 50a: the read-back

### Outcome

Every intuitionistic proof linlog finds or checks now shows its program.
`Derivation::term` (file `core/src/lambda/linear.rs`) reads a two-sided
derivation of intuitionistic linear logic back as a term of the linear
lambda calculus of Benton, Bierman, de Paiva and Hyland, in the form
Bierman's thesis gives it with the additives. A proof that weakens a
formula of no `!` reads back to the affine calculus. A cut is the redex
`(λx:A. N) M`. `ordinary::Derivation::term` (file
`core/src/lambda/simple.rs`) reads an LJ derivation back as a simply
typed term.

The term (`lambda::Term`) is self-contained and has a JSON form, written
only. It is printed under one options value, `lambda::Options` (its
fields are `notation`, Unicode or ASCII, and `types`). The command maps
`--lambda-notation` and `--lambda-types` onto that value; the settings
key is `lambda`. The term appears in four places:

- as a text line: `prove --lambda`, `check --lambda`, and the session
  command `lambda` of both `interact`s;
- in JSON, under the key `lambda`;
- under the conclusion in LaTeX, Typst and SVG (and so in PNG and PDF),
  through the new `lambda` field of `latex::Options`, `typst::Options`
  and `svg::Style`.

What has no term is refused with `Error::NoTerm` and its reason
(`lambda::NoTerm`): a classical proof, Mix, the cyclic and Lambek modes,
a first-order proof, open goals, and LK.

A student sees `⊢ λx. λy. x ⊗ y : a ⊸ (b ⊸ (a ⊗ b))` and `⊢ λf. λg. λx.
g (f x) : (a → b) → ((b → c) → (a → c))`. The manual's new chapter
"Proofs as programs" (`docs/src/terms.md`) states the calculus with its
sources and shows every form with real output.

The commits, in the workspace `../linlog-step50a` on `main`:

| change | commit | subject |
|---|---|---|
| `zqloqwrl` | `a13b807891e3` | Read intuitionistic proofs back as lambda-terms |
| `lnmqrtlu` | `4489dc0454c4` | Print a proof's lambda-term with --lambda |
| `wwlnpspo` | `a4101f65d16c` | Lock the help of --lambda and the session command lambda |
| `wvrstvnx` | `4ecdcb5425dc` | Run the target sets at 50a: every decided row's counters equal 47's |
| `qkyvuulo` | `f5a9c2adb797` | Answer the review of the read-back, and mark linlog's own notations |

This report is the change after them. The first commit was made while
the signing key was locked; the hook signed it with the next jj command,
and the ids above are the signed ones.

### The calculus, chosen with its source

The research sub-agent (`linlog-research-50a`, Sonnet 5.5 at `medium`)
read both papers in the original:

- Benton, Bierman, de Paiva and Hyland, "A term calculus for
  intuitionistic linear logic", TLCA 1993, LNCS 664, pp. 75–90
  (<https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/tlca93.pdf>,
  DOI 10.1007/BFb0037099): its sequent calculus is figure 4, its natural
  deduction figure 3, its reductions figures 5 and 6, and its cut
  elimination section 5.
- Abramsky, "Computational interpretations of linear logic", TCS 111
  (1993) 3–57, from a scan of the Elsevier original
  (<https://categorytheory.zulipchat.com/user_uploads/21317/Uc_tofs_6EC3g0kIEmPtf_gl/computational_linear.pdf>):
  its intuitionistic linear logic terms are on pages 14–16.

What the two papers cover:

- **BBdPH** is multiplicative and exponential only. It has `⊗`, `⊸`, `1`
  and `!` with `promote`, `derelict`, `discard` and `copy`. It shows that
  the earlier promotion `!Γ ⊢ !t` is not closed under substitution, and
  proves substitution and subject reduction for its own.
- **Abramsky** has the additives `&` and `⊕`, with pattern `let`s for
  `&L`, but no `⊤` or `0`. Its promotion is the one BBdPH call flawed.

A follow-up search found the calculus that completes BBdPH: G. M.
Bierman, *On Intuitionistic Linear Logic*, PhD thesis, Cambridge 1993,
published as technical report UCAM-CL-TR-346 (1994). It was read in a
copy with a text layer at
<https://ncatlab.org/nlab/files/Bierman-LinearLogic.pdf>. Its chapter 3
has the term grammar (page 67), figure 3.1 for natural deduction
(page 68) and figure 3.2 for the sequent calculus (page 72). Figure 3.2
adds these terms to BBdPH's:

- `⟨M,N⟩`, `fst`, `snd`, `inl`, `inr` and `case`;
- `true(x⃗)` for `⊤R`;
- `false_A(x⃗; z)` for `0L`.

Figure 3.6 gives the reductions, and figures 3.9 and 3.10 the commuting
conversions, among them `true` absorbing its arguments' free variables.

**Chosen: BBdPH's calculus as Bierman's thesis completes it, its
figure 3.2.** It is one published calculus that covers every connective
of linlog's ILL, and it is the one with the proved substitution property
and subject reduction. It gives a sequent-calculus term per rule, which
is what the read-back follows.

**The LJ side:** Girard, Lafont and Taylor, *Proofs and Types* (1989),
sections 3.1.2 and 10.6
(<https://www.paultaylor.eu/stable/Proofs+Types>). Their `π₁`, `π₂`,
`ι₁`, `ι₂`, `δ` and `ε` are written `fst`, `snd`, `inl`, `inr`, `case`
and `abort`, and `⟨⟩` is the unit, which the book does not name. The
manual states all of this (`docs/src/terms.md`, "The linear lambda
calculus" and "The simply typed lambda calculus").

### Decisions

- **The read-back takes a derivation, not the proof term.** The
  two-sided derivation already names every rule by its side, instantiates
  what `⊤` absorbs, and is built within `Limits`. A term read from it
  inherits those bounds. Structural rules are read by the difference of
  the premise's and the conclusion's hypotheses, so a compact derivation
  (the default view, which the exports draw) reads back like an explicit
  one.
- **Left rules substitute** (Bierman's `N[x := f M]`). The term of a
  cut-free proof is then in normal form, which is what students expect
  (`g (f x)`, not `let y = f x in …`).
  - The substituted term is one node that every place the hypothesis
    goes names. The term is therefore a DAG, and its text can grow by any
    factor over the derivation: a pair doubles it.
  - The printer bounds the text by an upper estimate over the DAG and
    refuses past `limits.derivation_bytes` before writing anything.
- **A cut is the redex `(λx:A. N) M`**, a term of the calculus's own
  grammar whose β-reduct is the thesis's cut. The thesis's figure 3.3
  writes `cut M for x in N`, but that figure assigns one constructor per
  rule, with `let` patterns for `&L`, and mixing it with figure 3.2 would
  mix two term assignments.
- **One term type for both calculi.** The simple calculus's types and
  nodes are a subset of the linear one's. `With` prints as `∧`, `Plus` as
  `∨` and `Lollipop` as `→` when the term's `Calculus` is `Simple`, and
  `true()` and `false(; z)` print as `⟨⟩` and `abort(z)` there. So one
  printer, one JSON form and one checker serve both.
- **Types belong to the term** (its own arena of `Type`, hash-consed, its
  own atom names), not ids into the proof's forest or the ordinary arena.
  The term is printed, serialised and checked without its owner, and a
  later checker takes it as it is.
- **Variable names are part of the data** (a letter and an index):
  - `f`, `g`, `h` for a variable of an implication;
  - `x`, `y`, `z`, `u`, `v`, `w` for any other;
  - then the same letters with an index: `x₁` in Unicode, `x1` in ASCII,
    `x_{1}` in LaTeX.
- **`abort` into `⊥` is not written in LJ.** `WR` and `⊥L` whose goal is
  `⊥` pass the term through, since `abort_⊥(M)` is `M`. Without this,
  `a, ¬a ⊢` read `abort(f x)`.
- **The flag is `--lambda`, not `--term`**, because `--term-limit`
  already bounds the first-order terms. The export field and the settings
  key are `lambda` for the same reason.
- **In text, the term is a line after the derivation**, written as
  `--stats` writes its lines (`Show::after`), so that the tree's output
  is unchanged and a script finds the line at the end. In the drawn
  formats it is in the drawing. In JSON it is a key beside the outcome.
- **Where the term stands in the exports.** It is the typing judgement
  under the conclusion, which is the first of the plan's two readings
  ("a term beside the conclusion, or at each node as an option"). A term
  at each node is not built: it would mean a judgement at every
  inference, whose terms repeat the substituted subterms. (departs from
  the design)
  - In LaTeX it is ebproof's `\infer[no rule]1{…}`.
  - In Typst it is a `stack` of the tree and the judgement, for both
    layouts.
  - In SVG it is a row under the root, with the id `term`.
- **The exports print the term within `Limits::default()`**, since their
  `write` takes no limits: a term over the default 64 MiB is refused
  there.
- **The notation option.** It has Unicode, the calculus's own, as the
  default, and ASCII as the second notation, the types spelt as the
  parsers read them. Haskell's `LinearTypes` was not built: `⊗` as
  tuples, `&` and `!` need types and encodings of their own, so it is no
  cheap second notation.

- **What is linlog's own, not the sources'.** The base is one published
  calculus, BBdPH's as Bierman's thesis completes it (fig. 3.2), which
  has every connective, `⊤` and `0` included. Nothing is taken from
  Abramsky. Four things are linlog's own, each marked so in the manual
  (`docs/src/terms.md`) and the module docs. (departs from the design)
  - **The cut** is written as the redex `(λx:A. N) M`, where the thesis
    writes its substitution `N[x := M]`. The plan asks for a redex, and
    this one uses only the calculus's own syntax.
  - **The affine reading**: a `wk` has no term, and a variable may go
    unused. No source gives an affine calculus, and the plan fixes this
    reading.
  - **`⟨⟩` for LJ's unit**: *Proofs and Types* names no unit term, and
    the empty pair is the usual choice.
  - **The ASCII notation** (`\x. M`, `*`, `()`, `<M, N>`, `->` and `|`
    in a case): no source has one, and it is the plan's second notation.

  The round-trip test covers every construct. The linear test asserts
  that all 19 kinds of node appear in round-tripped terms, and its cut
  test has the redexes. The LJ test has `⟨⟩` (`|- a -> true`) and
  `abort` (`false |- a`). The ASCII notation is pinned by
  `the_examples_read_back_as_documented`.

How each front end sets the options (D15, D16):

- **The library:** `lambda::Options` (`DEFAULT_NOTATION`,
  `DEFAULT_TYPES`) with serde, and the field `lambda` on the three
  export options.
- **The command:** the flags `--lambda`, `--lambda-notation` and
  `--lambda-types`, the key `lambda` of `--settings`, and the styles'
  `lambda` field (`--style latex.lambda=…`).
- **The web bindings (50b):** the same JSON, `{"notation": "unicode",
  "types": false}`, held in the tab's settings.
- **Quantifiers (D17):** their place is a `Node` variant for type
  abstraction and application and a `Type` variant with a type variable.
  `NoTerm::Quantified` stays for first-order quantifiers.

### What was checked, and by what

- **The tests.** `core/src/lambda/tests.rs` has ten tests, all passing.
  - **The round trip.** A type checker of the tests' own (`Checker`)
    infers each term's type and counts every variable's uses: exactly once
    in the linear calculus, with additive branches sharing; at most once
    in the affine one; any number in the simple one. For every term it
    checks three things: the type is the goal; the free variables are the
    context; and the hypotheses' and the goal's types print as the
    reading's own `Display` prints the sequent's formulas (in LJ, as a
    converter written apart from the read-back types them). It runs on:
    - 18 ILL sequents, covering every rule and all 19 kinds of node;
    - an affine proof;
    - five proofs with cuts;
    - 11 LJ sequents in intuitionistic and minimal logic, each in the
      explicit view and the admissible one.
  - **Cut elimination.**
    `cut_elimination_keeps_the_normal_form` takes nine proofs with a cut,
    in the `⊸`, `&`, `⊤` fragment. For each, the normal form of its term
    equals the term of the proof `proofs::cut::eliminate` makes, up to
    the names of bound variables, and that term has no redex.
  - **Bounds and refusals.** Further tests pin:
    - the refusals and their reasons (`what_has_no_term_is_refused`);
    - the output bound and the stop (`a_term_past_the_bound_is_refused`);
    - the type bound over the arena
      (`a_type_past_the_bound_is_refused_unwritten`);
    - the phase of a stop (`a_stopped_read_back_names_its_phase`);
    - 120 000 inferences read back and printed on a 256 KiB stack
      (`any_height_reads_back_on_a_small_stack`).

  `core/tests/depth.rs` reads back and prints, with types, a term whose
  type is nested 50 000 deep. `core/tests/export.rs`'s `lambda_terms`
  pins the term under the conclusion in LaTeX, Typst (both layouts) and
  SVG (the snapshots `term*`), for an ILL proof, a proof with a cut and
  an LJ derivation, and the refusal of a classical one.
- **The command.** `cargo test -p linlog-cli` passed: `--lambda`'s
  settings keys are in `each_flag_overrides_its_key_in_a_file`, and the
  readme test runs every example of the new chapter and of the guide,
  7 of 7 passing. The behaviour lock changed only in the help texts (its
  own commit, `wwlnpspo`).
- **The gate** passed at `wwlnpspo` (exit status 0, ending `gate:
  passed`), and again at `qkyvuulo` (exit status 0, `gate: passed`). The ratchet held every
  journey within its ceiling. It noted that cache misses moved on
  `search-qbf-20-2` and `check-qbf-20-2`; the gate says such a note is
  never a failure, and the search's code did not change.
- **The target sets** were run at `wwlnpspo`, then compared with
  `linlog-bench summary --against … --counters`:
  - `bench/targets.sh 50a` against `bench/targets/47.csv`: no verdict
    differs, nothing is decided by one and not the other, and of the 145
    problems both decide alike on one thread, 0 differ in a counter.
  - `bench/net-targets.sh 50a` against `net-47.csv`: likewise, with 0
    of 64.

  Their times ran 1.0 to 1.3 times 47's. Two other sessions ran beside
  them: 51a in the Rocq library on cores 10 to 13, and a design session
  on cores 14 and 15. The times are not a measurement; the counters are
  what compares.
- **The flake checks.** `nix build .#checks.x86_64-linux.manual
  .#checks.x86_64-linux.manual-links .#checks.x86_64-linux.export`
  passed (exit 0, all three output paths printed). The export check
  compiled the term snapshots with pdfLaTeX and Typst and rendered the
  SVG. `nix flake check` runs at the commit that holds this report, so
  its exit status cannot be in it: it is in the session's last message to
  `linlog-orchestrator`.
- **Checked by hand.** The reviewer's case (an `↔` nested 27 deep in an
  LJ session, then `lambda`) is now refused at once ("estimated at
  3221225476 bytes, more than the limit of 67108864"), where it was
  OOM-killed under 2 GiB before the fix.

### The reviewer's round

The fresh-context reviewer (`linlog-review-50a`, Opus 5.5 at `high`)
checked the read-back rule by rule against Bierman's figure 3.2 and the
round trip, with a type checker of its own over the JSON form. It ran:

- about 70 ILL sequents: every rule, `⊤` absorbing at `⊗`, at `&` and
  under promotion, duplicate members from copies of `!` formulas;
- compact runs and affine `wk`;
- cuts on every connective;
- about 40 LJ sequents;
- every refusal.

It found every term well typed with the right context and goal. It
reported three findings, and all three hold:

1. **The printer could run out of memory before it refused.** It
   measured each type by writing its text, and types share their
   operands, so an LJ `↔` nested `k` deep writes `2ᵏ` atoms; at `k = 27`
   the process was OOM-killed under 2 GiB. **Fixed at the cause**:
   `Printer::new` bounds every type's length over the arena, each type
   once, saturating, and writes nothing to measure it.
   `a_type_past_the_bound_is_refused_unwritten` pins it with a type
   nested 64 deep. The reviewer's case is now refused at once.
2. **A stopped read-back said `Phase::View`.** Its polls and the
   documentation say `Phase::ReadBack`. **Fixed**, and pinned by
   `a_stopped_read_back_names_its_phase`.
3. **The stated bound on a term's nodes was wrong for compact
   derivations.** A run of `!w` is one inference and a `Discard` per
   formula. **Fixed in the documentation** (`linear.rs`,
   `core-lambda.md`): the nodes are at most two per inference, one per
   formula a run weakens or copies, and one per variable, so never more
   than the derivation's members. The memory was linear all along.

It also raised a suspicion it did not report as a finding: the
admissible view might lose an `abort` when a hidden `WR` stands above a
one-premise `¬L`. No public path builds such a derivation (sessions
refuse `WR` in LJ, and `decide` never makes one), so it needs no change.

### Checklist

- [x] Research sub-agent's report on both calculi, with links; the
  calculus chosen and stated with its source in the manual
  (`docs/src/terms.md`).
- [x] Read-back of ILL proofs (linear, affine) and LJ derivations; cuts
  as redexes; refusals with reasons (classical, Mix, cyclic and Lambek,
  quantified, open goals, LK).
- [x] Printer with one options value (`lambda::Options`, serde, the
  `--lambda-notation` and `--lambda-types` flags, the settings key
  `lambda`).
- [x] Term in the JSON forms and in the LaTeX, Typst and SVG exports.
- [x] Bounds: no recursion, `Limits` (the output bound over the DAG and
  the type arena), the stop polled.
- [x] Round-trip test and the cut-elimination normal-form test.
- [x] `prove`, `check` and `interact` print the term; real examples in
  the manual and `skills/linlog/reference.md`; `cargo test -p linlog-cli
  --test readme` passing.
- [x] Gate passed; target sets 50a and net-50a against 47 with every
  decided row's counters equal; manual, manual-links and export checks
  passed.
- [x] Fresh-context reviewer's round, each finding answered.
- [x] `nix flake check` at the last commit (its exit status in the last
  message to `linlog-orchestrator`).
- [x] Last message to `linlog-orchestrator`.

### What is left open

- The test that equal normal forms follow cut elimination keeps to `⊸`,
  `&` and `⊤`. With `⊗`, `⊕`, `1` or `!`, the term of the eliminated
  proof differs from the reduct of the term with the cut by commuting
  conversions, which the test's normaliser does not implement
  (`core-lambda.md` says so).
- The term at each node of an exported derivation is not built (see
  Decisions).
- `CHANGELOG.md` has no section for the coming release yet; the release
  adds these items.

### What the next sessions must know

- **50b (the page).** The bindings need one call that reads a finished
  proof's term back and returns `lambda_json`'s shape:
  - the term's JSON form, with `text` and `judgement` added;
  - or `{"error": …}` with the refusal.

  For the linear suite, the call builds the two-sided derivation of the
  proof (`Proof::derivation_within` with `Sides::Two`) and calls
  `Derivation::term`. For the ordinary suite, it takes the session's
  `ordinary::Interactive::derivation_within` or the derivation `decide`
  returns, and calls `ordinary::Derivation::term`.

  The term's print options are the settings' `lambda` key. A row of both
  suites' calls belongs in `web.md`. Say the web module's growth against
  step 47's 6 913 020 bytes and 49b's 7 148 412.
- **The term data's shape (step 71's checker takes it).** A `Term` has:
  - `calculus`: linear, affine or simple;
  - `atoms`: names;
  - `types`: an arena of `Type` with operands first, hash-consed so that
    equal types are one id;
  - `variables`: each a type plus a printed letter and index;
  - `context`: the hypotheses' variables in the order the sequent writes
    them;
  - `goal`: the term's type;
  - `nodes`: an arena of `Node`, operands first, a DAG;
  - `root`.

  Binders carry their variables, and a variable's type is its
  annotation, so `λx:A` is `Lambda { variable }` with `A` the variable's
  type. `Left` and `Right` carry their sum type, and `False` its goal, so
  checking needs no inference of a missing annotation: the terms are
  Church-style.

  The test-only checker in `core/src/lambda/tests.rs` is the reference
  for the typing and the use discipline, as step 71 must implement them:
  - `Pair` and `Case` share their context: equal uses in the linear
    calculus, the larger of each otherwise;
  - `True` and `False` consume their arguments;
  - `Promote`'s body uses only its own variables;
  - `Discard` and `Copy` take a `!`.
- **Matching premises by member** (in `linear.rs`): equal members are
  copies of one formula, so which copy's term goes where does not
  matter. The premises are in the one-sided rule's order. A change to the
  derivation view that reorders premises breaks `⊗R`, `&R`, `⊕L`, `⊸L`
  and the cut.
