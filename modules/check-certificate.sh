# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL
# shellcheck shell=bash

# The program of the app `check-certificate' (check-certificate.nix), which
# sets ROCQ_LOAD_PATH, ROCQ_VERSION, NANOYALLA_VERSION and DEDUXIT_VERSION.
# Rocq type-checks every proof it compiles, so a file that compiles proves
# its lemmas as Rocq reads them. The .glob file Rocq writes lists the
# lemmas; `Print Assumptions' then shows whether one rests on an axiom or on
# a proof left `Admitted', which compile without complaint. What no check
# here sees is a file that gives the statement's own names another meaning
# (a `Definition ll'): the statement is read from the file.

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
requires=$(grep -E -e '^[[:space:]]*(From|Require)[[:space:]]' -- "$file" || true)
if grep -qw NanoYalla <<<"$requires"; then
  kernel=" and NanoYalla $NANOYALLA_VERSION"
elif grep -qw Deduxit <<<"$requires"; then
  kernel=" and rocq-deduxit $DEDUXIT_VERSION"
elif [ -n "$requires" ]; then
  kernel=" and its standard library"
else
  kernel=" alone, since the file imports nothing"
fi

# Compiled in a directory of its own, so nothing is written beside FILE and
# no other file there is loaded. Rocq names a module after its file, so the
# copy has a name that is always a module's, and Rocq's messages are given
# FILE's name back.
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
mkdir "$work/checked"
cp -- "$file" "$work/checked/certificate.v"
cd "$work" || exit 2
export ROCQPATH=$ROCQ_LOAD_PATH
unset COQPATH

echo "Checking $file with Rocq $ROCQ_VERSION$kernel."
compiled=true
log=$(cd checked && rocq compile -Q . Checked certificate.v 2>&1) || compiled=false
log=${log//'"./certificate.v"'/"\"$file\""}
if [ "$compiled" = false ]; then
  echo "$log" >&2
  echo "Rocq refuses $file." >&2
  if [ -z "$requires" ]; then
    echo "The file has no import lines: write it with deduxit's --standalone." >&2
  fi
  exit 1
fi
# Warnings, if any: they do not make a proof wrong.
if [ -n "$log" ]; then
  echo "$log"
fi

# What the file defines, from the .glob file Rocq writes beside the module:
# a line `KIND START:END SECTION NAME' per definition, so a lemma is found
# however the file lays it out. A certificate defines lemmas alone: in a
# section, a hypothesis becomes a premise of every lemma that uses it, and
# a module or a declaration states what the file does not show.
lemmas=()
while read -r kind _ section entry _; do
  case $kind in
  prf | def)
    if [ "$section" != '<>' ]; then
      echo "$file defines $entry inside $section, which no certificate of deduxit does." >&2
      echo "The certificate does not hold." >&2
      exit 1
    fi
    lemmas+=("$entry")
    ;;
  binder | DIGEST | F* | R[0-9]*) ;;
  *)
    case $kind in
    sec) what="a section" ;;
    mod) what="a module" ;;
    modtype) what="a module type" ;;
    var) what="a variable" ;;
    ax) what="an axiom" ;;
    *) what="a definition of the kind Rocq calls $kind" ;;
    esac
    echo "$file has $what named $entry, which no certificate of deduxit has." >&2
    echo "The certificate does not hold." >&2
    exit 1
    ;;
  esac
done <checked/certificate.glob
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
  printf 'From Checked Require certificate.\nPrint Assumptions Checked.certificate.%s.\n' \
    "$lemma" >assumptions.v
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
  echo "The certificate does not hold." >&2
  exit 1
fi
