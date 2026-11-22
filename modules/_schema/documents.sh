#!/usr/bin/env bash
# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# Writes the JSON documents the command writes on small inputs, one per
# file named FORM-CASE.json, FORM the entry of docs/src/wire.schema.json
# it must match. Usage: documents.sh DEDUXIT OUT-DIRECTORY. Each case
# states the exit status it expects; a case that exits otherwise, or
# writes nothing, fails the script after every case has run.
set -euo pipefail

deduxit=$(realpath "$1")
out=$(realpath "$2")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cd "$work"
failed=0

# run FILE STATUS COMMAND…: writes COMMAND's standard output to
# OUT/FILE.json, expecting the exit status STATUS.
run() {
  local file=$1 want=$2 status=0
  shift 2
  "$@" >"$out/$file.json" || status=$?
  if [ "$status" != "$want" ]; then
    echo "$file: exit $status, expected $want" >&2
    failed=1
  fi
}

# keep FILE STATUS NAME COMMAND…: runs COMMAND, which writes NAME in the
# working directory, and copies NAME to OUT/FILE.json.
keep() {
  local file=$1 want=$2 name=$3 status=0
  shift 3
  rm -f "$name"
  "$@" >/dev/null || status=$?
  if [ "$status" != "$want" ]; then
    echo "$file: exit $status, expected $want" >&2
    failed=1
  fi
  copy "$name" "$file"
}

# error FILE SOURCE KEY…: writes the error object found under the keys of
# OUT/SOURCE.json to OUT/FILE.json.
error() {
  local file=$1 source=$2
  shift 2
  python3 -c '
import json, sys
value = json.load(open(sys.argv[1]))
for key in sys.argv[3:]:
    value = value[key]
json.dump(value, open(sys.argv[2], "w"), ensure_ascii=False)
' "$out/$source.json" "$out/$file.json" "$@" || {
    echo "$file: no error under $* in $source.json" >&2
    failed=1
  }
}

# copy NAME FILE: copies NAME, which an earlier case wrote, to OUT/FILE.json.
copy() {
  cp "$1" "$out/$2.json" 2>/dev/null || {
    echo "$2: $1 was not written" >&2
    failed=1
  }
}

t=(--timeout 20s)

# Sequents.
run sequent-propositional 0 "$deduxit" seq json "A, A -o B |- B"
run sequent-first-order 0 "$deduxit" seq json "forall x. p(x) |- p(a)"
run sequent-cyclic 0 "$deduxit" seq json --cyclic "A * B |- B * A"
run ill_sequent-propositional 0 "$deduxit" seq json -i "A * B -o C |- A -o B -o C"
run ill_sequent-first-order 0 "$deduxit" seq json -i "forall x. p(x) |- p(a)"

# Outcomes of linear logic: each verdict, engine and refutation.
run outcome-proved 0 "$deduxit" prove "${t[@]}" --format json "A, A -o B |- B"
run outcome-proved-net 0 "$deduxit" prove "${t[@]}" --engine net --format json "A, A -o B |- B"
run outcome-proved-intuitionistic 0 "$deduxit" prove "${t[@]}" -i --format json "A, A -o B |- B"
run outcome-proved-additive 0 "$deduxit" prove "${t[@]}" --format json "A & B |- A"
run outcome-proved-exponential 0 "$deduxit" prove "${t[@]}" --format json "!A |- A * A"
run outcome-proved-horn 0 "$deduxit" prove "${t[@]}" --format json "!(a -o b), a |- b"
run outcome-proved-first-order 0 "$deduxit" prove "${t[@]}" --format json "forall x. p(x) |- p(a)"
run outcome-proved-cyclic 0 "$deduxit" prove "${t[@]}" --cyclic --format json "A * B |- A * B"
run outcome-proved-lambek 0 "$deduxit" prove "${t[@]}" --lambek --format json "A, B |- A * B"
run outcome-proved-mix 0 "$deduxit" prove "${t[@]}" --mix --format json "|- A, ~A, B, ~B"
run outcome-unknown 3 "$deduxit" prove "${t[@]}" --work-limit 1 --format json "!(a -o a * a), a |- a * a * a * a"
run outcome-exhausted 1 "$deduxit" prove "${t[@]}" --format json "A & B |- A * B"
run outcome-unbalanced 1 "$deduxit" prove "${t[@]}" --format json "A |- B"
run outcome-one-sided 1 "$deduxit" prove "${t[@]}" --format json "!A, A |- 1"
run outcome-equation 1 "$deduxit" prove "${t[@]}" --format json "|- A par B, ~A, ~B"
run outcome-state-equation 1 "$deduxit" prove "${t[@]}" --format json "!(a -o a * a), !(b * b -o c), a, b |- c"
run outcome-classical 1 "$deduxit" prove "${t[@]}" --format json "A + B |- A"
run outcome-saturated 1 "$deduxit" prove "${t[@]}" -i --engine inverse --format json "!(a -o b), a |- b * b"
run outcome-basis 1 "$deduxit" prove "${t[@]}" --affine --format json "!(a * a -o b * b * b), a, a, a |- b * b * b * b"
run outcome-unbalanced-symbol 1 "$deduxit" prove "${t[@]}" --format json "forall x. p(x) |- p(a) * p(b)"
run outcome-one-sided-symbol 1 "$deduxit" prove "${t[@]}" --format json "|- ~p(a), ?(exists x. ~p(x))"

# Proof structures.
run proof_structure-mll 0 "$deduxit" prove "${t[@]}" --net --format json "A * B |- B * A"
run proof_structure-cyclic 0 "$deduxit" prove "${t[@]}" --net --cyclic --format json "A * B |- A * B"
run mell_structure-boxes 0 "$deduxit" prove "${t[@]}" --net --format json "!a, !b |- !a"

# Proofs and sessions of `interact`, and the cut elimination. A session
# that ends with a goal open exits with 1.
keep session-classical 0 session.json "$deduxit" interact "A, A -o B |- B" <<<$'apply 0 1 * 0\nsave session.json\nclose\nproof classical.json'
copy classical.json proof-classical
keep session-intuitionistic 1 session.json "$deduxit" interact -i "A, A -o B |- B" <<<$'apply 0 1 -oL 0\nsave session.json'
keep session-cut 0 session.json "$deduxit" interact -i "A, A -o B, B -o C |- C" <<<$'cut 0 B 0 1\nsave session.json\nclose\nproof cut.json'
copy cut.json proof-cut
keep session-first-order 0 session.json "$deduxit" interact "forall x. p(x) |- p(a)" <<<$'close\nsave session.json\nproof quantifier.json'
copy quantifier.json proof-first-order
keep proof-contraction 0 contraction.json "$deduxit" interact -i '!a |- a * a' <<<$'cut 0 !a 0\nclose\nproof contraction.json'
run elimination-cut-free 0 "$deduxit" eliminate "${t[@]}" --format json cut.json
run elimination-contraction 0 "$deduxit" eliminate "${t[@]}" --format json contraction.json
run elimination-refused 3 "$deduxit" eliminate "${t[@]}" --node-limit 10 --format json contraction.json
error error-elimination elimination-refused reason
echo '{"version":7,"sequent":{"terms":[{"D":0},{"∃":0},{"V":1},{"V":0},{"∀":3}],"roots":[1,2],"atoms":[{"P":[0,[0]]},{"P":[0,[1]]}],"symbols":[["p",1],["a",0]],"fo":[{"B":0},{"F":[1,[]]}],"binders":["x","x"],"antecedents":1,"cuts":[4]},"nodes":[{"ax":[7,8]},{"∃":[0,2,0]},{"∀":[3,0,1]},{"ax":[9,2]},{"∃":[5,1,3]},{"cut":[3,2,4]}],"instances":{"members":[[1,0],[4,0],[6,1]],"frames":[[null,2],[null,1]],"terms":[{"E":0}]}}' >quantified.json
run elimination-first-order 0 "$deduxit" eliminate "${t[@]}" --format json quantified.json
run net_elimination-mll 0 "$deduxit" eliminate "${t[@]}" --net --format json cut.json
run net_elimination-boxes 0 "$deduxit" eliminate "${t[@]}" --net --format json contraction.json

# `check`'s verdicts, and the errors they carry.
run checked-proof 0 "$deduxit" check "${t[@]}" --format json classical.json
run checked-refutation 0 "$deduxit" check "${t[@]}" --format json "$out/outcome-state-equation.json"
echo '{"version":1,"sequent":{"terms":[{"V":0},{"V":1}],"roots":[0,1],"atoms":["A","B"]},"nodes":[{"ax":[0,1]}],"mode":"classical"}' >invalid.json
run checked-invalid 1 "$deduxit" check "${t[@]}" --format json invalid.json
error error-invalid-proof checked-invalid error
run checked-limit 3 "$deduxit" check "${t[@]}" --work-limit 1 --format json contraction.json
error error-limit checked-limit error

# Ordinary logic: each method and logic, valid, not valid and unknown.
run ordinary_outcome-sat-countermodel 1 "$deduxit" prove "${t[@]}" --logic intuitionistic --format json '~~a -> a'
run ordinary_outcome-sat-clausal 0 "$deduxit" prove "${t[@]}" --logic classical --method sat --format json 'a \/ ~a'
run ordinary_outcome-sat-derivation 0 "$deduxit" prove "${t[@]}" --logic intuitionistic --method sat --format json 'a \/ b -> b \/ a'
run ordinary_outcome-embedding-valid 0 "$deduxit" prove "${t[@]}" --logic intuitionistic --method embedding --format json 'a, a -> b |- b'
run ordinary_outcome-embedding-not-valid 1 "$deduxit" prove "${t[@]}" --logic classical --method embedding --format json 'a |- b'
run ordinary_outcome-ljt-valid 0 "$deduxit" prove "${t[@]}" --logic intuitionistic --method ljt --format json 'a \/ b -> b \/ a'
run ordinary_outcome-ljt-not-valid 1 "$deduxit" prove "${t[@]}" --logic intuitionistic --method ljt --format json '~~a -> a'
run ordinary_outcome-minimal 0 "$deduxit" prove "${t[@]}" --logic minimal --format json 'a, a -> b |- b'
run ordinary_outcome-minimal-countermodel 1 "$deduxit" prove "${t[@]}" --logic minimal --format json '~a |- a -> b'
run ordinary_outcome-table-valid 0 "$deduxit" prove "${t[@]}" --logic classical --method table --format json 'a |- a'
run ordinary_outcome-table-not-valid 1 "$deduxit" prove "${t[@]}" --logic classical --method table --format json 'a -> b |- b -> a'
run ordinary_outcome-tableau-valid 0 "$deduxit" prove "${t[@]}" --logic classical --method tableau --format json 'p \/ q, p -> r, q -> r |- r'
run ordinary_outcome-tableau-not-valid 1 "$deduxit" prove "${t[@]}" --logic classical --method tableau --format json 'a -> b |- b -> a'
run ordinary_outcome-unknown 3 "$deduxit" prove "${t[@]}" --logic intuitionistic --method ljt --work-limit 1 --format json '((a -> b) -> a) -> a'
run ordinary_outcome-connection 0 "$deduxit" prove "${t[@]}" --logic classical --method connection --format json 'forall x. (p(x) -> q(x)), p(a) |- q(a)'
run ordinary_outcome-grounding-valid 0 "$deduxit" prove "${t[@]}" --logic classical --method grounding --format json 'forall x. (p(x) -> q(x)), p(a) |- q(a)'
run ordinary_outcome-grounding-structure 1 "$deduxit" prove "${t[@]}" --logic classical --format json 'exists x. p(x), q |- forall x. p(x)'
run ordinary_outcome-models-structure 1 "$deduxit" prove "${t[@]}" --logic classical --method models --format json 'p(a) |- p(f(a))'
run ordinary_outcome-models-kripke 1 "$deduxit" prove "${t[@]}" --logic intuitionistic --method models --format json '(forall x. p(x)) -> q |- exists x. (p(x) -> q)'
run ordinary_outcome-models-kripke-unknown 3 "$deduxit" prove "${t[@]}" --logic intuitionistic --method models --model-size 2 --model-worlds 2 --format json 'forall x. p(x) |- p(f(a))'
run ordinary_outcome-models-unknown 3 "$deduxit" prove "${t[@]}" --logic classical --method models --model-size 2 --format json 'forall x. p(x) |- p(f(a))'
run checked-ordinary 0 "$deduxit" check "${t[@]}" --format json "$out/ordinary_outcome-ljt-valid.json"
run checked-kripke-structure 0 "$deduxit" check "${t[@]}" --format json "$out/ordinary_outcome-models-kripke.json"
run checked-structure-functions 0 "$deduxit" check "${t[@]}" --format json "$out/ordinary_outcome-models-structure.json"
run checked-structure 0 "$deduxit" check "${t[@]}" --format json "$out/ordinary_outcome-grounding-structure.json"
echo '{"worlds": [{"atoms": [], "arrows": [1]}, {"atoms": ["a"]}]}' >model.json
run checked-model 1 "$deduxit" check "${t[@]}" --format json --logic intuitionistic --sequent 'a \/ ~a' model.json
# Modal logic: outcomes, their check and a model checked by hand.
run modal_outcome-valid 0 "$deduxit" prove "${t[@]}" --system S4 --format json '[]p |- [][]p'
run modal_outcome-not-valid 1 "$deduxit" prove "${t[@]}" --system K --format json '[]p |- [][]p'
run modal_outcome-unknown 3 "$deduxit" prove "${t[@]}" --system S4 --work-limit 1 --format json '[](p -> []p) -> (p -> []p)'
run checked-modal 0 "$deduxit" check "${t[@]}" --format json "$out/modal_outcome-not-valid.json"
echo '{"worlds": [{"atoms": [], "arrows": [1]}, {"atoms": ["p"], "arrows": [2]}, {"atoms": []}]}' >modal-model.json
run checked-modal-model 1 "$deduxit" check "${t[@]}" --format json --system K --sequent '[]p |- [][]p' modal-model.json
keep ordinary_session-intuitionistic 1 session.json "$deduxit" interact --logic intuitionistic 'a -> b, b -> c |- a -> c' <<<$'apply 0 R0 ->R\nsave session.json'
keep ordinary_derivation-intuitionistic 0 derivation.json "$deduxit" interact --logic intuitionistic 'a, a -> b |- b' <<<$'close\nproof derivation.json'
keep ordinary_derivation-classical 0 lk-derivation.json "$deduxit" interact --logic classical '((a -> b) -> a) -> a' <<<$'close\nproof lk-derivation.json'
keep ordinary_session-first-order 1 fo-session.json "$deduxit" interact --logic intuitionistic 'forall x. (p(x) -> q(x)), exists x. p(x) |- exists y. q(y)' <<<$'apply 0 L1 existsL\napply 1 R0 existsR a\nsave fo-session.json'
keep ordinary_derivation-first-order 0 fo-derivation.json "$deduxit" interact --logic classical 'forall x. p(x) |- p(c)' <<<$'apply 0 L0 forallL c\napply 1 R0 ax\nproof fo-derivation.json'

# Negative translations and interpolants.
run comparison-kolmogorov 0 "$deduxit" seq translate "${t[@]}" --format json 'a \/ ~a'
run comparison-glivenko 0 "$deduxit" seq translate "${t[@]}" --by glivenko --format json '~~a -> a'
run comparison-not-valid 1 "$deduxit" seq translate "${t[@]}" --by godel-gentzen --format json 'a |- b'
run interpolation-classical 0 "$deduxit" interpolate "${t[@]}" --format json 'a /\ b |- b \/ c'
run interpolation-intuitionistic 0 "$deduxit" interpolate "${t[@]}" --logic intuitionistic --first L1 --format json 'a, a -> b |- b'
run interpolation-not-valid 1 "$deduxit" interpolate "${t[@]}" --format json 'a |- b'
# Clause sets decided: refuted by saturation and by DPLL, satisfiable,
# and unknown at a bound.
run clause_resolution-saturation 0 "$deduxit" resolve "${t[@]}" --format json 'a -> b, a |- b'
run clause_resolution-dpll 0 "$deduxit" resolve "${t[@]}" --method dpll --format json 'a \/ b, ~a |- b'
run clause_resolution-satisfiable 1 "$deduxit" resolve "${t[@]}" --method dpll --format json 'a \/ b |- a'
run clause_resolution-unknown 3 "$deduxit" resolve "${t[@]}" --step-limit 1 --format json 'a -> b, b -> c |- a -> c'
run clause_resolution-first-order 0 "$deduxit" resolve "${t[@]}" --format json 'forall x y. (p(x) \/ p(y)), forall u v. (~p(u) \/ ~p(v)) |-'
run clause_resolution-first-order-saturated 3 "$deduxit" resolve "${t[@]}" --format json 'exists x. p(x) |- forall y. p(y)'
run clause_resolution-paramodulation 0 "$deduxit" resolve "${t[@]}" --format json 'a = b, p(a) |- p(b)'
run herbrand-valid 0 "$deduxit" herbrand "${t[@]}" --format json '|- exists x. (p(x) -> forall y. p(y))'
run herbrand-unknown 3 "$deduxit" herbrand "${t[@]}" --depth 1 --format json 'forall x. (p(x) -> p(f(x))) |- p(a)'
# Lambda-terms: a term checked, well typed and not; a type with a term
# and one without; a type's terms counted exactly, infinitely many, and
# at least as many as a shallow search met.
run lambda_typing-check 0 "$deduxit" term check --format json '\f:a -> b. \x:a. f x'
run lambda_ill_typed-check 1 "$deduxit" term check --format json '\x:a. x x'
run lambda_inhabitation-inhabited 0 "$deduxit" term inhabit "${t[@]}" --format json 'a /\ b -> b /\ a'
run lambda_inhabitation-empty 1 "$deduxit" term inhabit "${t[@]}" --format json '((a -> b) -> a) -> a'
run lambda_inhabitants-exactly 0 "$deduxit" term enumerate "${t[@]}" --format json 'a -> a -> a'
run lambda_inhabitants-infinite 0 "$deduxit" term enumerate "${t[@]}" --format json --depth 3 '(a -> a) -> a -> a'
run lambda_inhabitants-at-least 3 "$deduxit" term enumerate "${t[@]}" --format json --depth 1 '(a -> b) -> (b -> c) -> a -> c'
# Normal forms: an equivalent one and an encoding.
run normal_form-cnf 0 "$deduxit" seq normalize "${t[@]}" --format json '~(a -> b /\ c)'
run normal_form-tseitin 0 "$deduxit" seq normalize "${t[@]}" --to tseitin --format json 'a /\ b |- a'
run normal_form-prenex 0 "$deduxit" seq normalize "${t[@]}" --to prenex --format json '(forall x. p(x)) -> exists y. q(y)'
run normal_form-clausal 0 "$deduxit" seq normalize "${t[@]}" --to clausal --format json 'forall x. exists y. r(x, y) |- exists z. r(a, z)'
run normal_form-herbrand 0 "$deduxit" seq normalize "${t[@]}" --to herbrand --format json 'exists x. p(x) |- exists y. forall z. q(y, z)'
# A tableau written by hand, reviewed: closed, open with the countermodel
# its branch gives, and wrong at a line.
printf '1 T p \\/ q\n2 F q \\/ p\n3 F q 2\n4 F p 2\n- 5 T p 1\n  x 4, 5\n- 6 T q 1\n  x 3, 6\n' >closed.txt
run tableau_review-closed 0 "$deduxit" check "${t[@]}" --tableau --format json closed.txt
printf '1 T a -> b\n2 F b -> a\n3 T b 2\n4 F a 2\n- 5 F a 1\n  open\n- 6 T b 1\n  open\n' >open.txt
run tableau_review-open 0 "$deduxit" check "${t[@]}" --tableau --format json open.txt
printf '1 T p /\\ q\n2 F p\n3 T r 1\n' >wrong.txt
run tableau_review-wrong 1 "$deduxit" check "${t[@]}" --tableau --format json wrong.txt
# An answer graded: a script that finishes the proof, and one with a step
# that fails.
printf 'apply 0 2 * 0\napply 1 1 ax\napply 2 1 ax\n' >script.txt
run graded-correct 0 "$deduxit" grade "${t[@]}" --format json --answer script.txt 'A, B |- A * B'
printf 'apply 0 0 par\n' >failing.txt
run graded-incorrect 1 "$deduxit" grade "${t[@]}" --format json --answer failing.txt 'A, B |- A * B'

# Calculi given as data: derivations, sessions and their check.
keep rules_session-backward 0 session.json "$deduxit" interact --calculus LK "a /\ b |- b /\ a" <<<$'apply 0 ∧R 1\napply 1 ∧L₂ 0\nsave session.json\napply 2 ∧L₁ 0\napply 3 ax\napply 4 ax\nproof swap.json'
copy swap.json rules_derivation-lk
run checked-calculus 0 "$deduxit" check "${t[@]}" --format json swap.json
keep rules_session-forward 0 session.json "$deduxit" interact --calculus NJ "(a -> b) -> (b -> c) -> a -> c" <<<$'apply 0 →I\napply 1 →I\napply 2 →I\nderive 3 →E 1 3\nsave session.json\nderive 3 →E 2 4\nassume 3\nproof chain.json'
copy chain.json rules_derivation-nj
keep rules_session-cites 1 session.json "$deduxit" interact --calculus NJ --assume "a \/ b" "b \/ a" <<<$'apply 0 ∨E A=a B=b\nassume 1\nderive 2 ∨I₂ 2 A=b\nassume 2 4\nsave session.json'

exit "$failed"
