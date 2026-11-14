# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# intuit (Claessen and Rosén, "SAT modulo intuitionistic implications",
# LPAR 2015; no licence stated: its LICENSE file is empty), a prover of
# intuitionistic propositional logic on MiniSat, through the Haskell
# package `minisat'. Built with the GHC of nixpkgs (it was written for 7.8
# and 7.10). `intuit FILE' reads a TPTP problem and prints `+++ RESULT:
# Valid' or `CounterSatisfiable'.
{
  haskellPackages,
  fetchFromGitHub,
}:
let
  src = fetchFromGitHub {
    owner = "koengit";
    repo = "intuit";
    rev = "dd965ce7c5ebfa22983bba98ced77be7fc1f0aa5";
    hash = "sha256-gf+5DG7oSKNyeFSfGFh1xVvW4lMUHX492G7JYzGbeWo=";
  };
in
haskellPackages.mkDerivation {
  pname = "intuit";
  version = "2020-09-21";
  inherit src;
  # MonadFailDesugaring is the only behaviour since GHC 8.8, and 9.10 no
  # longer knows the name.
  postPatch = ''
    sed -i '/LANGUAGE MonadFailDesugaring/d' ParseProblem.hs Parsek.hs
  '';
  isLibrary = false;
  isExecutable = true;
  executableHaskellDepends = with haskellPackages; [
    base
    containers
    minisat
  ];
  passthru.source = {
    url = "https://github.com/koengit/intuit";
    inherit (src) rev;
    date = "2020-09-21";
    licence = "none stated";
  };
  mainProgram = "intuit";
}
