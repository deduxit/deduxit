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
run checked-ordinary 0 "$deduxit" check "${t[@]}" --format json "$out/ordinary_outcome-ljt-valid.json"
echo '{"worlds": [{"atoms": [], "arrows": [1]}, {"atoms": ["a"]}]}' >model.json
run checked-model 1 "$deduxit" check "${t[@]}" --format json --logic intuitionistic --sequent 'a \/ ~a' model.json
keep ordinary_session-intuitionistic 1 session.json "$deduxit" interact --logic intuitionistic 'a -> b, b -> c |- a -> c' <<<$'apply 0 R0 ->R\nsave session.json'
keep ordinary_derivation-intuitionistic 0 derivation.json "$deduxit" interact --logic intuitionistic 'a, a -> b |- b' <<<$'close\nproof derivation.json'
keep ordinary_derivation-classical 0 lk-derivation.json "$deduxit" interact --logic classical '((a -> b) -> a) -> a' <<<$'close\nproof lk-derivation.json'

# Negative translations and interpolants.
run comparison-kolmogorov 0 "$deduxit" seq translate "${t[@]}" --format json 'a \/ ~a'
run comparison-glivenko 0 "$deduxit" seq translate "${t[@]}" --by glivenko --format json '~~a -> a'
run comparison-not-valid 1 "$deduxit" seq translate "${t[@]}" --by godel-gentzen --format json 'a |- b'
run interpolation-classical 0 "$deduxit" interpolate "${t[@]}" --format json 'a /\ b |- b \/ c'
run interpolation-intuitionistic 0 "$deduxit" interpolate "${t[@]}" --logic intuitionistic --first L1 --format json 'a, a -> b |- b'
run interpolation-not-valid 1 "$deduxit" interpolate "${t[@]}" --format json 'a |- b'

# Calculi given as data: derivations, sessions and their check.
keep rules_session-backward 0 session.json "$deduxit" interact --calculus LK "a /\ b |- b /\ a" <<<$'apply 0 ∧R 1\napply 1 ∧L₂ 0\nsave session.json\napply 2 ∧L₁ 0\napply 3 ax\napply 4 ax\nproof swap.json'
copy swap.json rules_derivation-lk
run checked-calculus 0 "$deduxit" check "${t[@]}" --format json swap.json
keep rules_session-forward 0 session.json "$deduxit" interact --calculus NJ "(a -> b) -> (b -> c) -> a -> c" <<<$'apply 0 →I\napply 1 →I\napply 2 →I\nderive 3 →E 1 3\nsave session.json\nderive 3 →E 2 4\nassume 3\nproof chain.json'
copy chain.json rules_derivation-nj
keep rules_session-cites 1 session.json "$deduxit" interact --calculus NJ --assume "a \/ b" "b \/ a" <<<$'apply 0 ∨E A=a B=b\nassume 1\nderive 2 ∨I₂ 2 A=b\nassume 2 4\nsave session.json'

exit "$failed"
