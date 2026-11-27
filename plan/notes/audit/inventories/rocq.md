# The Rocq library's inventory

The Rocq Prover, version 9.1.1
compiled with OCaml 4.14.4; 36 files of `_RocqProject`.

## The build, with every warning on (`-w all`)

| file | s | warnings |
|---|---:|---|
| `theories/Linear/Formulas.v` | 0.2 |  |
| `theories/Linear/Calculus.v` | 0.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/Proofs.v` | 0.1 |  |
| `theories/Linear/Check.v` | 0.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/Soundness.v` | 0.6 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/Intuitionistic.v` | 0.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/IntuitionisticCheck.v` | 0.6 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/Mix.v` | 0.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/Refutations.v` | 0.4 | warn-transitive-library-file-stdlib-vector (1), disj-pattern-notation (1) |
| `theories/Linear/Weights.v` | 0.6 | warn-transitive-library-file-stdlib-vector (1), disj-pattern-notation (1) |
| `theories/Linear/Terms.v` | 0.1 |  |
| `theories/Linear/TermsCheck.v` | 0.2 |  |
| `theories/Linear/FirstOrderFormulas.v` | 0.1 |  |
| `theories/Linear/FirstOrderCalculus.v` | 0.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/FirstOrderProofs.v` | 0.1 |  |
| `theories/Linear/FirstOrderCheck.v` | 0.3 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/FirstOrderSoundness.v` | 1.2 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/FirstOrderIntuitionistic.v` | 0.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/FirstOrderIntuitionisticCheck.v` | 1.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Linear/FirstOrderWeights.v` | 0.5 | warn-transitive-library-file-stdlib-vector (1), disj-pattern-notation (1) |
| `theories/Intuitionistic/Formulas.v` | 0.0 |  |
| `theories/Intuitionistic/Calculus.v` | 0.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Intuitionistic/Kripke.v` | 0.2 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Intuitionistic/KripkeCheck.v` | 0.3 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Intuitionistic/FirstOrderFormulas.v` | 0.1 |  |
| `theories/Intuitionistic/FirstOrderCalculus.v` | 0.1 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Intuitionistic/FirstOrderKripke.v` | 0.3 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Intuitionistic/FirstOrderClassical.v` | 0.3 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Intuitionistic/FirstOrderCheck.v` | 0.4 | warn-transitive-library-file-stdlib-vector (1) |
| `theories/Modal/Formulas.v` | 0.0 |  |
| `theories/Modal/Kripke.v` | 0.1 |  |
| `theories/Modal/Labelled.v` | 0.1 |  |
| `theories/Modal/Calculus.v` | 0.2 |  |
| `theories/Modal/Soundness.v` | 0.3 |  |
| `theories/Modal/Check.v` | 0.3 |  |
| `theories/Modal/Countermodel.v` | 0.3 |  |

Total: 10 s, one file after another.

- `tests/Assumptions.v`: 4.1 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/Certificates.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/Cuts.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/FirstOrder.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/FirstOrderTwoSided.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/FirstOrderWeights.v`: 0.2 s, disj-pattern-notation (1), warn-transitive-library-file-stdlib-vector (1)
- `tests/FromCommand.v`: 0.4 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/Modes.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/Refutations.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/TwoSided.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/Weights.v`: 0.2 s, disj-pattern-notation (1), warn-transitive-library-file-stdlib-vector (1)
- `tests/Intuitionistic/FirstOrder.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/Intuitionistic/Kripke.v`: 0.2 s, warn-transitive-library-file-stdlib-vector (1)
- `tests/Modal/Certificates.v`: 0.2 s
- `tests/Modal/Countermodels.v`: 0.2 s

Warnings by name:

- `warn-transitive-library-file-stdlib-vector`: 34
- `disj-pattern-notation`: 5

The first warnings of `theories/Linear/Calculus.v`:

```
File "./theories/Linear/Calculus.v", line 16, characters 0-44:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]

```

The first warnings of `theories/Linear/Check.v`:

```
File "./theories/Linear/Check.v", line 19, characters 0-51:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]

```

The first warnings of `theories/Linear/Soundness.v`:

```
File "./theories/Linear/Soundness.v", line 15, characters 0-53:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]
File "./theories/Linear/Soundness.v", line 15, characters 0-53:
Warning: Library File (transitively required) Stdlib.ZArith.ZArith_base is
deprecated since 9.0. use ZArith instead
[deprecated-transitive-library-file-since-9.0,deprecated-since-9.0,deprecated-transitive-library-file,deprecated]

```

The first warnings of `theories/Linear/Intuitionistic.v`:

```
File "./theories/Linear/Intuitionistic.v", line 26, characters 0-44:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]

```

The first warnings of `theories/Linear/IntuitionisticCheck.v`:

```
File "./theories/Linear/IntuitionisticCheck.v", line 26, characters 0-53:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]
File "./theories/Linear/IntuitionisticCheck.v", line 26, characters 0-53:
Warning: Library File (transitively required) Stdlib.ZArith.ZArith_base is
deprecated since 9.0. use ZArith instead
[deprecated-transitive-library-file-since-9.0,deprecated-since-9.0,deprecated-transitive-library-file,deprecated]

```

The first warnings of `theories/Linear/Mix.v`:

```
File "./theories/Linear/Mix.v", line 15, characters 0-44:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]

```

The first warnings of `theories/Linear/Refutations.v`:

```
File "./theories/Linear/Refutations.v", line 37, characters 0-60:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]
File "./theories/Linear/Refutations.v", line 37, characters 0-60:
Warning: Library File (transitively required) Stdlib.ZArith.ZArith_base is
deprecated since 9.0. use ZArith instead
[deprecated-transitive-library-file-since-9.0,deprecated-since-9.0,deprecated-transitive-library-file,deprecated]
File "./theories/Linear/Refutations.v", line 37, characters 0-60:
Warning:
Use of "( _ | _ )" Notation is deprecated as it is inconsistent with pattern syntax.
[disj-pattern-notation,syntax]

```

The first warnings of `theories/Linear/Weights.v`:

```
File "./theories/Linear/Weights.v", line 51, characters 0-60:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]
File "./theories/Linear/Weights.v", line 51, characters 0-60:
Warning: Library File (transitively required) Stdlib.ZArith.ZArith_base is
deprecated since 9.0. use ZArith instead
[deprecated-transitive-library-file-since-9.0,deprecated-since-9.0,deprecated-transitive-library-file,deprecated]
File "./theories/Linear/Weights.v", line 51, characters 0-60:
Warning:
Use of "( _ | _ )" Notation is deprecated as it is inconsistent with pattern syntax.
[disj-pattern-notation,syntax]

```

The first warnings of `theories/Linear/FirstOrderCalculus.v`:

```
File "./theories/Linear/FirstOrderCalculus.v", line 27, characters 0-44:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]

```

The first warnings of `theories/Linear/FirstOrderCheck.v`:

```
File "./theories/Linear/FirstOrderCheck.v", line 27, characters 0-82:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]

```

The first warnings of `theories/Linear/FirstOrderSoundness.v`:

```
File "./theories/Linear/FirstOrderSoundness.v", line 29, characters 0-53:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]
File "./theories/Linear/FirstOrderSoundness.v", line 29, characters 0-53:
Warning: Library File (transitively required) Stdlib.ZArith.ZArith_base is
deprecated since 9.0. use ZArith instead
[deprecated-transitive-library-file-since-9.0,deprecated-since-9.0,deprecated-transitive-library-file,deprecated]

```

The first warnings of `theories/Linear/FirstOrderIntuitionistic.v`:

```
File "./theories/Linear/FirstOrderIntuitionistic.v", line 28, characters 0-44:
Warning: Alternatives to Fin.t are available, see
<https://github.com/coq/stdlib/blob/master/theories/Vectors/Fin.v>.
[warn-transitive-library-file-stdlib-vector,stdlib-vector,warn-transitive-library-file,user-warn]

```

## The build with `-mangle-names`

Each file alone, against the library as built above. A file that fails names a hypothesis or variable Rocq generated.

- `theories/Linear/Formulas.v`: File "./theories/Linear/Formulas.v", line 15, characters 0-17: Warning: Trying to mask the absolute name "Formulas"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Linear/Formulas.v", line 53, characters 17-18: Error: The variable A was not found in the current environment.
- `theories/Linear/Soundness.v`: File "./theories/Linear/Soundness.v", line 19, characters 0-18: Warning: Trying to mask the absolute name "Soundness"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Linear/Soundness.v", line 33, characters 12-13: Error: The variable l was not found in the current environment.
- `theories/Linear/IntuitionisticCheck.v`: File "./theories/Linear/IntuitionisticCheck.v", line 61, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Linear/Mix.v`: File "./theories/Linear/Mix.v", line 19, characters 0-12: Warning: Trying to mask the absolute name "Mix"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Linear/Mix.v", line 43, characters 24-28: Error: The variable IHll was not found in the current environment.
- `theories/Linear/Refutations.v`: File "./theories/Linear/Refutations.v", line 42, characters 0-20: Warning: Trying to mask the absolute name "Refutations"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Linear/Refutations.v", line 66, characters 63-74: Error: In environment atom : Type
- `theories/Linear/Weights.v`: File "./theories/Linear/Weights.v", line 199, characters 0-16: Warning: Trying to mask the absolute name "Weights"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Linear/Weights.v", line 261, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Linear/TermsCheck.v`: File "./theories/Linear/TermsCheck.v", line 60, characters 12-13: Error: The variable t was not found in the current environment.
- `theories/Linear/FirstOrderCheck.v`: File "./theories/Linear/FirstOrderCheck.v", line 68, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Linear/FirstOrderSoundness.v`: File "./theories/Linear/FirstOrderSoundness.v", line 50, characters 12-13: Error: The variable l was not found in the current environment.
- `theories/Linear/FirstOrderIntuitionisticCheck.v`: File "./theories/Linear/FirstOrderIntuitionisticCheck.v", line 64, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Linear/FirstOrderWeights.v`: File "./theories/Linear/FirstOrderWeights.v", line 90, characters 17-18: Error: The variable A was not found in the current environment.
- `theories/Intuitionistic/Kripke.v`: File "./theories/Intuitionistic/Kripke.v", line 25, characters 0-15: Warning: Trying to mask the absolute name "Kripke"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Intuitionistic/Kripke.v", line 60, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Intuitionistic/FirstOrderKripke.v`: File "./theories/Intuitionistic/FirstOrderKripke.v", line 204, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Intuitionistic/FirstOrderClassical.v`: File "./theories/Intuitionistic/FirstOrderClassical.v", line 150, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Intuitionistic/FirstOrderCheck.v`: File "./theories/Intuitionistic/FirstOrderCheck.v", line 92, characters 12-13: Error: The variable l was not found in the current environment.
- `theories/Modal/Soundness.v`: File "./theories/Modal/Soundness.v", line 29, characters 0-18: Warning: Trying to mask the absolute name "Soundness"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Modal/Soundness.v", line 131, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Modal/Check.v`: File "./theories/Modal/Check.v", line 21, characters 0-14: Warning: Trying to mask the absolute name "Check"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Modal/Check.v", line 48, characters 12-13: Error: The variable A was not found in the current environment.
- `theories/Modal/Countermodel.v`: File "./theories/Modal/Countermodel.v", line 20, characters 0-21: Warning: Trying to mask the absolute name "Countermodel"! [masking-absolute-name,deprecated-since-8.8,deprecated,default] File "./theories/Modal/Countermodel.v", line 149, characters 12-13: Error: The variable n was not found in the current environment.

18 of 36 files refused.

## The assumptions of every theorem and lemma

275 theorems and lemmas in the library; `tests/Assumptions.v` asks about 53. Asked here of all: 275 closed under the global context, 0 with assumptions, 0 not resolved.


| file | theorems and lemmas | in `tests/Assumptions.v` |
|---|---:|---:|
| `theories/Linear/Formulas.v` | 2 | 0 |
| `theories/Linear/Calculus.v` | 0 | 0 |
| `theories/Linear/Proofs.v` | 0 | 0 |
| `theories/Linear/Check.v` | 0 | 0 |
| `theories/Linear/Soundness.v` | 16 | 4 |
| `theories/Linear/Intuitionistic.v` | 0 | 0 |
| `theories/Linear/IntuitionisticCheck.v` | 8 | 1 |
| `theories/Linear/Mix.v` | 3 | 2 |
| `theories/Linear/Refutations.v` | 27 | 11 |
| `theories/Linear/Weights.v` | 35 | 6 |
| `theories/Linear/Terms.v` | 0 | 0 |
| `theories/Linear/TermsCheck.v` | 6 | 0 |
| `theories/Linear/FirstOrderFormulas.v` | 0 | 0 |
| `theories/Linear/FirstOrderCalculus.v` | 0 | 0 |
| `theories/Linear/FirstOrderProofs.v` | 0 | 0 |
| `theories/Linear/FirstOrderCheck.v` | 1 | 0 |
| `theories/Linear/FirstOrderSoundness.v` | 30 | 4 |
| `theories/Linear/FirstOrderIntuitionistic.v` | 0 | 0 |
| `theories/Linear/FirstOrderIntuitionisticCheck.v` | 21 | 1 |
| `theories/Linear/FirstOrderWeights.v` | 23 | 4 |
| `theories/Intuitionistic/Formulas.v` | 0 | 0 |
| `theories/Intuitionistic/Calculus.v` | 0 | 0 |
| `theories/Intuitionistic/Kripke.v` | 5 | 3 |
| `theories/Intuitionistic/KripkeCheck.v` | 8 | 1 |
| `theories/Intuitionistic/FirstOrderFormulas.v` | 0 | 0 |
| `theories/Intuitionistic/FirstOrderCalculus.v` | 0 | 0 |
| `theories/Intuitionistic/FirstOrderKripke.v` | 17 | 4 |
| `theories/Intuitionistic/FirstOrderClassical.v` | 20 | 2 |
| `theories/Intuitionistic/FirstOrderCheck.v` | 19 | 2 |
| `theories/Modal/Formulas.v` | 0 | 0 |
| `theories/Modal/Kripke.v` | 0 | 0 |
| `theories/Modal/Labelled.v` | 0 | 0 |
| `theories/Modal/Calculus.v` | 0 | 0 |
| `theories/Modal/Soundness.v` | 16 | 4 |
| `theories/Modal/Check.v` | 7 | 3 |
| `theories/Modal/Countermodel.v` | 11 | 1 |

## Habits of the sources (`theories/`)

| what | lines |
|---|---:|
| `From Stdlib Require` | 32 |
| `From Coq Require` | 0 |
| `Require` without `From` | 0 |
| `Require Import` or `Export` (opens a module) | 66 |
| hint declared with a locality attribute | 0 |
| hint declared without one | 0 |
| `Qed.` | 275 |
| `Defined.` | 0 |
| `Set Implicit Arguments` and the like | 0 |
| `Ltac` definitions | 19 |
| `Notation` or `Infix` | 80 |
| `intuition`, `firstorder` or `tauto` | 16 |
| `lia` | 133 |
| `auto` or `eauto` | 100 |
| lines over 80 characters | 580 |

Modules of other libraries required, by the files that do:

- `Stdlib.List`: 32
- `Stdlib.Bool`: 19
- `Stdlib.Permutation`: 17
- `Stdlib.Lia`: 11
- `Stdlib.Arith`: 5
- `Stdlib.String`: 4
- `Stdlib.ZArith`: 3

## The modules the exporter's certificates import (`core/src`)

- `Deduxit.Intuitionistic.FirstOrderCheck`: 1 places
- `Deduxit.Intuitionistic.KripkeCheck`: 1 places
- `Deduxit.Linear.Calculus`: 2 places
- `Deduxit.Linear.FirstOrderCalculus`: 1 places
- `Deduxit.Linear.FirstOrderIntuitionistic`: 1 places
- `Deduxit.Linear.FirstOrderIntuitionisticCheck`: 1 places
- `Deduxit.Linear.FirstOrderSoundness`: 1 places
- `Deduxit.Linear.FirstOrderWeights`: 1 places
- `Deduxit.Linear.Intuitionistic`: 2 places
- `Deduxit.Linear.IntuitionisticCheck`: 1 places
- `Deduxit.Linear.Soundness`: 1 places
- `Deduxit.Linear.TermsCheck`: 1 places
- `Deduxit.Linear.Weights`: 1 places
- `Deduxit.Modal.Check`: 1 places
- `Deduxit.Modal.Countermodel`: 1 places

The library's 21 other modules:

- `Deduxit.Intuitionistic.Calculus`
- `Deduxit.Intuitionistic.FirstOrderCalculus`
- `Deduxit.Intuitionistic.FirstOrderClassical`
- `Deduxit.Intuitionistic.FirstOrderFormulas`
- `Deduxit.Intuitionistic.FirstOrderKripke`
- `Deduxit.Intuitionistic.Formulas`
- `Deduxit.Intuitionistic.Kripke`
- `Deduxit.Linear.Check`
- `Deduxit.Linear.FirstOrderCheck`
- `Deduxit.Linear.FirstOrderFormulas`
- `Deduxit.Linear.FirstOrderProofs`
- `Deduxit.Linear.Formulas`
- `Deduxit.Linear.Mix`
- `Deduxit.Linear.Proofs`
- `Deduxit.Linear.Refutations`
- `Deduxit.Linear.Terms`
- `Deduxit.Modal.Calculus`
- `Deduxit.Modal.Formulas`
- `Deduxit.Modal.Kripke`
- `Deduxit.Modal.Labelled`
- `Deduxit.Modal.Soundness`
