# deduxit © Fabian Lukas Grubmüller 2026
# Licensed under the EUPL

# What the collection may cost as it grows: the crates Cargo.lock holds and
# the web module's download, each under a ceiling. A change that passes one
# raises it here, in a commit that says what earned the room; the budget
# says when a cost grew, not whether it may.
{
  perSystem =
    {
      config,
      pkgs,
      ...
    }:
    let
      # The lock's packages, the workspace's own members among them: 173
      # on 2026-11-14.
      mostPackages = 180;
      packages = builtins.length (builtins.fromTOML (builtins.readFile ../Cargo.lock)).package;

      # The modules the web page loads, in bytes at `gzip -9', what a
      # browser downloads. The bindings' module, which every visitor's
      # worker loads: 2 661 758 on 2026-11-22 with the finite model search
      # (the expansion over a domain, the function tables read off and
      # checked, the normal form's function terms); 2 631 105 that day
      # with grounding (the
      # decidable classes' normal form, miniscoping, the ground walk) and
      # the finite structure's evaluator and reader beside both connection
      # provers; 2 590 533 on 2026-11-21 with both connection provers
      # (the classical one's matrix and search, the intuitionistic one's
      # prefixes, their unification and its read-back to LJ); 2 502 108
      # that day with the ordinary session's
      # open witnesses (their fillings and the reader's replay of them,
      # the inferences each witness shows, the bound on terms for every
      # step) and first-order derivations drawn; 2 438 116 that day, once the PNG and PDF
      # renderers left it for a module of their own (3 913 893 that day
      # before, with the modal search's reuse of worlds); 3 903 905 on
      # 2026-11-20 with first-order ordinary formulas (the arena's binders
      # and instances, the native and TPTP parsers' and the JSON reader's
      # first-order paths, the printer's renaming, every method's
      # refusal); 3 831 827 that day with the suite of lambda-terms (the
      # term's parser and checker and the enumeration of a type's terms,
      # which the workspace's runs link; 3 741 399 before it, with normal
      # forms and resolution); 3 622 968 on 2026-11-16 with the modal
      # suite (3 541 980 before it, once the ordinary searches and the
      # SVG drawing were compiled once whatever closure stops them;
      # 3 670 553 before that, when the bindings took up the workspace,
      # whose runs and checks link every suite's operations; 3 166 704
      # before that; 3 111 006 on 2026-11-15, with calculi given as data;
      # 2 842 389 on 2026-11-14, 2 603 759 at 0.2.0).
      mostModuleBytes = 2700000;
      # The renderers' module, loaded on a visitor's first PNG or PDF
      # export: 1 639 598 on 2026-11-21 (resvg and krilla with their
      # fonts', images' and filters' code, and the reader of every key of
      # the tab's settings).
      mostRenderBytes = 1700000;
    in
    {
      checks.budget =
        pkgs.runCommand "check-budget"
          {
            nativeBuildInputs = [ pkgs.gzip ];
          }
          ''
            if [ ${toString packages} -gt ${toString mostPackages} ]; then
              echo "Cargo.lock holds ${toString packages} packages, over the budget of ${toString mostPackages}"
              exit 1
            fi
            gzipped() {
              gzip -9 -c ${config.packages.deduxit-web}/$1 | wc -c
            }
            bytes=$(gzipped deduxit_web_bg.wasm)
            if [ "$bytes" -gt ${toString mostModuleBytes} ]; then
              echo "the web module is $bytes bytes at gzip -9, over the budget of ${toString mostModuleBytes}"
              exit 1
            fi
            render=$(gzipped deduxit_web_render_bg.wasm)
            if [ "$render" -gt ${toString mostRenderBytes} ]; then
              echo "the renderers' module is $render bytes at gzip -9, over the budget of ${toString mostRenderBytes}"
              exit 1
            fi
            echo "${toString packages} packages, the module $bytes bytes and the renderers' $render at gzip -9" > $out
          '';
    };
}
