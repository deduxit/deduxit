% deduxit © Fabian Lukas Grubmüller 2026
% Licensed under the EUPL

% A driver for the connection provers leanCoP 2.1, nanoCoP 2.0, nanoCoP-i
% 2.0 and iLeanCoP 1.2 on SWI-Prolog. Each release ships a shell script
% that runs its prover once per strategy, each in a Prolog process of its
% own, and polls the process's CPU time with `ps' once a second, so that
% no answer comes before a second has passed. This driver runs the same
% strategies in the same order with the same shares of the time limit, in
% one process, each under `call_with_time_limit'. A strategy's "valid" is
% the answer; its "not valid" only where the script trusts it (the
% complete strategies). The last line it prints is the verdict: `theorem',
% `non-theorem' or `timeout'.
%
% Run as: swipl -q DRIVER -- PROVER FILE SECONDS, after the prover's own
% files are loaded by `load/1' from the directory of this file.

:- use_module(library(time)).

:- initialization(main, main).

main :-
    current_prolog_flag(argv, [Prover, File, Seconds | _]),
    atom_number(Seconds, Limit),
    load(Prover),
    get_time(Start),
    schedule(Prover, Strategies),
    run(Strategies, Prover, File, Start, Limit).

% The directory this file is in, where the prover's files are.
:- prolog_load_context(directory, Dir), assertz(home(Dir)).

% Loads the prover's main file from the directory of this file.
load(Prover) :-
    home(Dir),
    main_file(Prover, Name),
    working_directory(Old, Dir),
    consult(Name),
    working_directory(_, Old).

main_file(leancop, 'leancop_main.pl').
main_file(nanocop, 'nanocop_main.pl').
main_file(nanocopi, 'nanocopi_main.pl').
main_file(ileancop, 'ileancop_main.pl').

% Runs the strategies in turn until one answers. A strategy's share of the
% limit is its percentage of it divided by 111, as the scripts compute it,
% but never past the time left; the last takes all that is left.
run([], _, _, _, _) :-
    verdict(timeout).
run([Strategy | Rest], Prover, File, Start, Limit) :-
    Strategy = s(Settings, Percent, Complete),
    get_time(Now),
    Left is Limit - (Now - Start),
    (   Left =< 0
    ->  verdict(timeout)
    ;   (   Rest == []
        ->  Slice = Left
        ;   Slice is min(Left, Limit * Percent / 111)
        ),
        (   Slice > 0,
            catch(call_with_time_limit(Slice, solve(Prover, File, Settings, Result)),
                  time_limit_exceeded, fail)
        ->  answer(Result, Complete, Rest, Prover, File, Start, Limit)
        ;   run(Rest, Prover, File, Start, Limit)
        )
    ).

answer(theorem, _, _, _, _, _, _) :-
    !,
    verdict(theorem).
answer(non_theorem, true, _, _, _, _, _) :-
    !,
    verdict('non-theorem').
answer(_, _, Rest, Prover, File, Start, Limit) :-
    run(Rest, Prover, File, Start, Limit).

verdict(Word) :-
    nl,
    write(verdict(Word)),
    nl,
    halt(0).

% One strategy of the prover on the file, its result read from the main
% predicate of the prover's release.
solve(leancop, File, Settings, Result) :-
    leancop_main(File, Settings, Answer),
    result(Answer, Result).
solve(nanocop, File, Settings, Result) :-
    nanocop_main(File, Settings, Answer),
    result(Answer, Result).
solve(nanocopi, File, Settings, Result) :-
    nanocopi_main(File, Settings, Answer),
    result(Answer, Result).
solve(ileancop, File, Settings, Result) :-
    ileancop_main(File, Settings, Answer),
    result(Answer, Result).

result('Theorem', theorem).
result('Unsatisfiable', theorem).
result('Non-Theorem', non_theorem).
result('Satisfiable', non_theorem).

% The strategies of each release's script, in its order: the settings, the
% percentage of the time limit, and whether the script trusts a "not
% valid" of it.
schedule(leancop, [
    s([cut,comp(7)], 10, true),
    s([conj,def,cut], 15, false),
    s([nodef,scut,cut], 15, false),
    s([scut], 10, false),
    s([def,cut], 5, false),
    s([conj,nodef,cut], 4, false),
    s([def,scut,cut], 2, false),
    s([scut,cut], 2, false),
    s([conj,def], 1, false),
    s([reo(40),conj,nodef,scut,cut], 4, false),
    s([reo(42),def,scut,cut], 4, false),
    s([reo(12),def,scut,cut], 2, false),
    s([reo(72),def,scut,cut], 2, false),
    s([reo(39),nodef,cut], 2, false),
    s([reo(38),conj,def,cut], 2, false),
    s([reo(15),conj,def,cut], 2, false),
    s([reo(73),conj,def,cut], 1, false),
    s([reo(57),conj,def,cut], 1, false),
    s([reo(13),conj,nodef,scut,cut], 1, false),
    s([reo(59),conj,nodef,scut,cut], 1, false),
    s([reo(75),conj,nodef,scut,cut], 1, false),
    s([reo(36),conj,nodef,scut], 1, false),
    s([reo(16),conj,nodef,scut], 1, false),
    s([reo(71),def,scut], 1, false),
    s([reo(58),def,scut,cut], 1, false),
    s([reo(76),def,scut,cut], 1, false),
    s([reo(74),nodef,cut], 1, false),
    s([reo(14),nodef,cut], 1, false),
    s([reo(37),nodef,scut], 1, false),
    s([def], 99, true)
]).
schedule(nanocop, [
    s([cut,comp(7)], 15, true),
    s([reo(22),conj,cut], 20, false),
    s([scut], 10, false),
    s([scut,cut], 5, false),
    s([noeq,scut], 5, false),
    s([reo(20),conj,cut], 10, false),
    s([reo(30),conj,scut,cut], 10, false),
    s([reo(34),conj,scut,cut], 5, false),
    s([reo(33),conj,scut,cut], 3, false),
    s([noeq,reo(64),cut], 3, false),
    s([reo(12),scut,cut], 3, false),
    s([reo(16),scut,cut], 2, false),
    s([reo(2),cut], 2, false),
    s([reo(5),cut], 1, false),
    s([reo(6),scut], 1, false),
    s([reo(8),scut], 1, false),
    s([reo(42),scut], 1, false),
    s([reo(48),cut], 1, false),
    s([reo(64),scut], 1, false),
    s([], 99, true)
]).
schedule(nanocopi, [
    s([cut,comp(6)], 15, true),
    s([scut], 10, false),
    s([scut,cut], 5, false),
    s([noeq,scut], 5, false),
    s([reo(20),conj,cut], 10, false),
    s([reo(22),conj,cut], 10, false),
    s([reo(33),conj,scut,cut], 10, false),
    s([reo(2),cut], 10, false),
    s([reo(5),cut], 10, false),
    s([reo(6),scut], 5, false),
    s([noeq,reo(64),cut], 5, false),
    s([], 99, true)
]).
% iLeanCoP's script gives its strategies 1/50, 30/50, 10/50 and 5/50 of
% the limit and the last all of it; written here as percentages of 111.
schedule(ileancop, [
    s([def,scut,cut,comp(7)], 2.22, true),
    s([def,scut,cut], 66.6, false),
    s([conj,scut,cut], 22.2, false),
    s([def,conj,cut], 11.1, false),
    s([def], 111, true)
]).
