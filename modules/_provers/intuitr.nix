# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# intuitR (Fiorentini, "Efficient SAT-based proof search in
# intuitionistic propositional logic", CADE 2021; BSD-3-Clause): intuit
# with restarts, on MiniSat through the Haskell package `minisat'. Built
# with the GHC of nixpkgs. `intuitR FILE' reads a TPTP problem and prints
# `+++ RESULT: Valid' or `CounterSatisfiable'. nixpkgs marks its
# dependency `partial-order' broken for a failed build of its tests, so
# that is built without them.
{
  lib,
  haskellPackages,
  haskell,
  fetchFromGitHub,
}:
let
  inherit (haskell.lib.compose) markUnbroken dontCheck;
  src = fetchFromGitHub {
    owner = "cfiorentini";
    repo = "intuitR";
    rev = "0cbf0fd078567e12808ee006015d15a3deacab63";
    hash = "sha256-5oTZAwfnasDcp6b0GjqMDsnGdL/9IcpxGpBJX2kSgk4=";
  };
in
haskellPackages.mkDerivation {
  pname = "intuitR";
  version = "2026-10-27";
  inherit src;
  isLibrary = false;
  isExecutable = true;
  executableHaskellDepends = with haskellPackages; [
    array
    base
    containers
    directory
    filepath
    minisat
    mtl
    parsec
    (dontCheck (markUnbroken partial-order))
    random
    raw-strings-qq
    unix
  ];
  passthru.source = {
    url = "https://github.com/cfiorentini/intuitR";
    inherit (src) rev;
    date = "2026-10-27";
    licence = "BSD-3-Clause";
  };
  license = lib.licenses.bsd3;
  mainProgram = "intuitR";
}
