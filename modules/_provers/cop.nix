# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# Jens Otten's connection provers for first-order logic, each its release
# from leancop.de run by nixpkgs' SWI-Prolog: leanCoP 2.1 (classical,
# clausal), nanoCoP 2.0 (classical, non-clausal), nanoCoP-i 2.0
# (intuitionistic, non-clausal) and iLeanCoP 1.2 (intuitionistic, clausal).
# Each is a command on one TPTP problem: `NAME FILE SECONDS BYTES', the
# time limit and the limit of SWI-Prolog's stacks. The driver `cop.pl'
# runs the strategies of the release's script in one process, and
# `ileancop_main.pl' reads TPTP for iLeanCoP, whose release runs on ECLiPSe
# (not in nixpkgs): its two lines that set ECLiPSe's flags are replaced by
# SWI-Prolog's global occurs check. All four are GPL (no version stated),
# fetched, never copied into this repository.
{
  fetchurl,
  swi-prolog,
  writeShellApplication,
  runCommand,
  coreutils,
}:
let
  fetch =
    path: hash:
    fetchurl {
      url = "https://www.leancop.de/${path}";
      inherit hash;
    };
  leancop-src = fetch "programs/leancop21.tar.gz" "sha256-zkMsX5NowJPwjfMSAhhGPYvbhBLnHsmAq5yFLBPO8wA=";
  ileancop-src = fetch "programs/ileancop12.tar.gz" "sha256-W572e0QLSx+W/TnobGbzUfA10GXYn4AsG1UI1JxadvA=";
  nanocop-src = fetch "nanocop/programs/nanoCoP20.tar.gz" "sha256-kwBtnHMI5oRu7EQo8jOjoVNQUYWzHL5vL+QXkh9vQ7E=";
  nanocopi-src = fetch "nanocop-i/programs/nanoCoP-i20.tar.gz" "sha256-lZDWRA2ZLL1enrOjtLVdFsUAcIARqSzmxUFdqLvpIwg=";
  # A command `name' that runs the driver on `prover' with the release's
  # files in `share'. A TPTP problem's includes name files under the
  # library's root, two directories above the problem (`Problems/SYN/').
  tool =
    {
      name,
      prover,
      version,
      share,
      source,
    }:
    (writeShellApplication {
      inherit name;
      runtimeInputs = [
        swi-prolog
        coreutils
      ];
      text = ''
        file=$(realpath "$1")
        TPTP=$(dirname "$(dirname "$(dirname "$file")")")
        export TPTP
        exec swipl --stack-limit="$3" -q ${share}/cop.pl -- ${prover} "$file" "$2"
      '';
    }).overrideAttrs
      {
        inherit version;
        passthru.source = source;
      };
  # The release's files, unpacked from `src' (its directory `dir', or the
  # top), with the driver beside them.
  share =
    name: src: dir:
    runCommand "${name}-share" { } ''
      mkdir unpacked $out
      tar -xzf ${src} -C unpacked
      cp unpacked/${dir}*.pl $out/
      cp ${./cop.pl} $out/cop.pl
    '';
  licence = "GPL (no version stated)";
in
{
  leancop = tool {
    name = "leancop";
    prover = "leancop";
    version = "2.1";
    share = share "leancop" leancop-src "leancop21/";
    source = {
      url = "https://www.leancop.de/programs/leancop21.tar.gz";
      date = "2009-07-03";
      inherit licence;
    };
  };
  nanocop = tool {
    name = "nanocop";
    prover = "nanocop";
    version = "2.0";
    share = share "nanocop" nanocop-src "nanoCoP20/";
    source = {
      url = "https://www.leancop.de/nanocop/programs/nanoCoP20.tar.gz";
      inherit licence;
    };
  };
  nanocopi = tool {
    name = "nanocopi";
    prover = "nanocopi";
    version = "2.0";
    share = share "nanocopi" nanocopi-src "nanoCoP-i20/";
    source = {
      url = "https://www.leancop.de/nanocop-i/programs/nanoCoP-i20.tar.gz";
      inherit licence;
    };
  };
  ileancop = tool {
    name = "ileancop";
    prover = "ileancop";
    version = "1.2";
    share = runCommand "ileancop-share" { } ''
      mkdir unpacked $out
      tar -xzf ${ileancop-src} -C unpacked
      tar -xzf ${leancop-src} -C unpacked
      cp unpacked/ileancop12.pl unpacked/def_mm_intu.pl $out/
      cp unpacked/leancop21/leancop_tptp2.pl $out/
      cp ${./ileancop_main.pl} $out/ileancop_main.pl
      cp ${./cop.pl} $out/cop.pl
      substituteInPlace $out/ileancop12.pl \
        --replace-fail ':- lib(iso).' '%' \
        --replace-fail ':- set_flag(occur_check,on).' ':- set_prolog_flag(occurs_check,true).'
    '';
    source = {
      url = "https://www.leancop.de/programs/ileancop12.tar.gz";
      date = "2007-06-07";
      inherit licence;
      changed = "runs on SWI-Prolog: lib(iso) left out, ECLiPSe's occur_check as SWI-Prolog's occurs_check; TPTP read by leanCoP 2.1's reader";
    };
  };
}
