# Use the library

The library is the crate `deduxit`. Add it, with `serde_json` for the
JSON forms:

```toml
[dependencies]
deduxit = "0.3"
serde_json = "1"
```

The API's documentation is on [docs.rs/deduxit](https://docs.rs/deduxit)
for the releases and on
[deduxit.github.io/deduxit](https://deduxit.github.io/deduxit/) for
`main`. Its crate root has the examples below, which its tests run.
[Install deduxit](installation.md#the-library) lists the crate's
features.

## Decide a sequent and print the proof

Parse the sequent, prove it, print the derivation, and write the proof
as JSON:

```rust
use deduxit::linear::{Fragment, Mode, Options, Sequent, Verdict, prove};

// Two-sided input becomes one-sided: ⊢ ~A, A ⊗ ~B, B.
let sequent: Sequent = "A, A -o B |- B".parse()?;
assert_eq!(sequent.fragment(), Fragment::MLL);

let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default())?;
let Verdict::Proved(proof) = &outcome.verdict else {
    panic!("the sequent is provable");
};
assert_eq!(
    proof.derivation()?.to_string(),
    "─────── ax   ─────── ax\n\
     ⊢ ~A, A      ⊢ ~B, B\n\
     ──────────────────── ⊗\n\
    \x20 ⊢ ~A, A ⊗ ~B, B"
);

let json = serde_json::to_string(&proof)?;
let back: deduxit::linear::Proof = serde_json::from_str(&json)?;
back.check(Mode::CLASSICAL)?;
```

`Mode` is the logic (`Mode::CLASSICAL` or `Mode::INTUITIONISTIC`, with
`.with_mix()` or `.with_affine()`), and `Options` the search's settings.
An unprovable verdict, `Verdict::Unprovable(disproof)`, prints its reason
with `{}`. For an intuitionistic sequent written with sides, parse it
as an `IllSequent`.

## Decide as the command does

`Options::default()` bounds the copies of a `?` formula at three, since
a call without a stop must end. The command instead deepens the bound
until its time limit of two seconds. `Settings::default()` holds the
command's settings, and `Settings::stop` builds its stop from the time
since the start. It fires at the time limit and lets the classical
reading after the search run past it, as the command does. On one
thread this gives the command's `--deterministic` answer:

```rust
use deduxit::{Settings, linear::{Mode, Verdict, prove_within}};
use std::time::Instant;

let settings = Settings::default();
let search = settings.search.clone().with_jobs(1);
let start = Instant::now();
let sequent = "!A |- !A * !A".parse()?;
let stop = settings.stop(|| start.elapsed());
let outcome = prove_within(&sequent, Mode::CLASSICAL, &search, &settings.limits, stop)?;
assert!(matches!(outcome.verdict, Verdict::Proved(_)));
```

The crate reads no clock, so the caller passes the time. In a browser,
where there is no clock to read, a stop can count the `work` that each
call to it reports instead.

## Export a proof

Each export has a `to_string` beside its `write`:
`export::latex::to_string(derivation, &options)`, and the same for
`typst`, `svg` and `rocq`. It returns the whole text. `write` writes into
a stream and takes a stop, for a large derivation.
[Export a proof](output-formats.md) shows the formats and their options.

A `lambda::Term` prints with `{}` under the default options
([Read a proof as a program](terms.md)).

## Read the command's JSON

A file that `deduxit prove --format json` wrote reads back as an
`Outcome`: `let outcome: Outcome = serde_json::from_str(&text)?;` gives
the verdict with its proof or disproof, the mode, the engine and the
statistics. Check the proof with `proof.check(outcome.mode)`. An engine
that a later version names reads as `Engine::Unknown`.

## Read an error

Every call returns the one error type, `deduxit::Error`. Its `kind` says
what sort of failure it is, so that a call that a bound or a stop refused
is never read as a fault of its input, and its `code` is a stable reason
to branch on. Its `Display` writes formulas where it can: given a
sequent with no intuitionistic reading, `prove` in an intuitionistic
mode returns `Error::NotIntuitionistic`, whose message names the
subformula.
