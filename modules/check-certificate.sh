# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
# shellcheck shell=bash

# The program of the app `check-certificate' (check-certificate.nix), which
# sets ROCQ_LOAD_PATH, ROCQ_VERSION, NANOYALLA_VERSION and DEDUXIT_VERSION.
# Rocq type-checks every proof it compiles, so a file that compiles proves
# its lemmas; `Print Assumptions' then shows whether a lemma rests on an
# axiom or on a proof left `Admitted', which compile without complaint.

usage="Usage: check-certificate FILE

Checks a Rocq certificate that deduxit wrote with --format rocq --standalone.
It compiles FILE with Rocq $ROCQ_VERSION against the kernel its import lines
name: NanoYalla $NANOYALLA_VERSION, rocq-deduxit $DEDUXIT_VERSION, or Rocq's
standard library alone for ordinary logic. Then it asks Rocq which axioms
each lemma of the file rests on.

Exit status: 0 if every lemma is proved without an axiom (a classical
certificate over Prop may use the excluded middle), 1 if Rocq refuses the
file or a lemma rests on another axiom, 2 if FILE cannot be read."

case "${1-}" in
-h | --help)
  echo "$usage"
  exit 0
  ;;
esac
if [ "$#" -ne 1 ]; then
  echo "$usage" >&2
  exit 2
fi
file=$1
if [ ! -f "$file" ] || [ ! -r "$file" ]; then
  echo "check-certificate: cannot read $file" >&2
  exit 2
fi

# The kernel, from the import lines deduxit writes with --standalone.
requires=$(grep -E '^[[:space:]]*(From|Require)[[:space:]]' "$file" || true)
if grep -qw NanoYalla <<<"$requires"; then
  kernel=" and NanoYalla $NANOYALLA_VERSION"
elif grep -qw Deduxit <<<"$requires"; then
  kernel=" and rocq-deduxit $DEDUXIT_VERSION"
elif [ -n "$requires" ]; then
  kernel=" and its standard library"
else
  kernel=" alone, since the file imports nothing"
fi

# Rocq names a module after its file, so the copy keeps the file's name
# where that is a Rocq identifier and is called `certificate' otherwise.
name=$(basename -- "$file")
name=${name%.v}
if ! [[ $name =~ ^[A-Za-z_][A-Za-z0-9_\']*$ ]]; then
  name=certificate
fi

# Compiled in a directory of its own, so nothing is written beside FILE and
# no other file there is loaded.
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
mkdir "$work/checked"
cp -- "$file" "$work/checked/$name.v"
cd "$work" || exit 2
export ROCQPATH=$ROCQ_LOAD_PATH
unset COQPATH

echo "Checking $file with Rocq $ROCQ_VERSION$kernel."
if ! (cd checked && rocq compile -Q . Checked "$name.v") >compile.log 2>&1; then
  cat compile.log >&2
  echo "Rocq refuses $file." >&2
  if [ -z "$requires" ]; then
    echo "The file has no import lines: write it with deduxit's --standalone." >&2
  fi
  exit 1
fi
# Warnings, if any: they do not make a proof wrong.
cat compile.log

mapfile -t lemmas < <(
  sed -nE "s/^[[:space:]]*(Lemma|Theorem|Corollary|Proposition|Fact|Remark)[[:space:]]+([A-Za-z_][A-Za-z0-9_']*).*/\2/p" \
    "checked/$name.v"
)
if [ "${#lemmas[@]}" -eq 0 ]; then
  echo "$file states no lemma, so it certifies nothing." >&2
  exit 1
fi

# The standard library's excluded middle, as Print Assumptions names it. An
# axiom of another module prints with a longer name, and one that printed
# the same would state the excluded middle all the same.
classic='Axioms:
Classical_Prop.classic : forall P : Prop, P \/ ~ P'
holds=true
for lemma in "${lemmas[@]}"; do
  printf 'From Checked Require %s.\nPrint Assumptions Checked.%s.%s.\n' \
    "$name" "$name" "$lemma" >assumptions.v
  if ! rocq compile -Q checked Checked assumptions.v >assumptions.log 2>&1; then
    cat assumptions.log >&2
    exit 1
  fi
  assumptions=$(<assumptions.log)
  if [ "$assumptions" = 'Closed under the global context' ]; then
    echo "Lemma $lemma: proved, with no axiom."
  elif [ "$assumptions" = "$classic" ] && grep -qw Classical_Prop <<<"$requires"; then
    echo "Lemma $lemma: proved, with the excluded middle (classic) as its one axiom."
  else
    echo "Lemma $lemma rests on assumptions that no certificate of deduxit needs:" >&2
    echo "$assumptions" >&2
    holds=false
  fi
done
if [ "$holds" = true ]; then
  echo "The certificate holds."
else
  echo "Rocq refuses $file as a certificate." >&2
  exit 1
fi
