% deduxit © Fabian Lukas Grubmüller 2026
% SPDX-License-Identifier: GPL-3.0-or-later
% It follows leanCoP 2.1's main (`leancop_main.pl', © 2009 Jens Otten),
% which states the GNU General Public License without a version, so this
% file is under the GPL too. It is benchmark glue, run against provers
% the flake fetches, and no part of any crate.

% iLeanCoP 1.2's main on SWI-Prolog: the release runs on ECLiPSe and reads
% problems in leanCoP's own syntax, made by the TPTP2X tool. Here a TPTP
% problem is read by leanCoP 2.1's TPTP reader, its includes found under
% the directory `TPTP' names, as leanCoP's own main reads one, and handed
% to iLeanCoP's `prove2/2'.

:- dynamic(axiom_path/1).
:- ( getenv('TPTP', Path) -> true ; Path = '' ),
   retractall(axiom_path(_)),
   assertz(axiom_path(Path)).

% ECLiPSe's name of `retractall/1', which the prover calls.
retract_all(Head) :- retractall(Head).

:- [ileancop12].
:- [leancop_tptp2].

ileancop_main(File, Settings, Result) :-
    axiom_path(AxPath),
    ( AxPath = '' -> AxDir = '' ; atom_concat(AxPath, '/', AxDir) ),
    leancop_tptp2(File, AxDir, [_], F, Conj),
    ( Conj \= [] -> Problem = F ; Problem = (~F) ),
    leancop_equal(Problem, Problem1),
    (   prove2(Problem1, Settings)
    ->  ( Conj \= [] -> Result = 'Theorem' ; Result = 'Unsatisfiable' )
    ;   ( Conj \= [] -> Result = 'Non-Theorem' ; Result = 'Satisfiable' )
    ).
