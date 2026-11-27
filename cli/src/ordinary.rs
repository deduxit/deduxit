// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Ordinary logic on the command line: a formula or sequent of classical,
//! intuitionistic or minimal logic read under `--logic`, translated into
//! linear logic, decided there, and its proof read back as LK or LJ.

use crate::argument_parsing::{Format, InputFormat, ProveArgs, Tree};
use crate::io::{self, Most, admit, bound, refused};
use crate::limit::{Deadline, Notice};
use crate::prove::{
    Ended, NOTICE_AFTER, Stop, advice, notice_line, on_large_stack, stopped, unknown,
};
use crate::show::{Show, Shown, every_steps, lambda_json, lambda_line, not_built};
use crate::{Status, catch_interrupt, interrupted};
use anyhow::{Result, anyhow, bail};
use deduxit::linear::proofs::{Sides, Structural};
use deduxit::linear::search::{Outcome, Reason, Verdict};
use deduxit::ordinary::{self, Derivation, Image, Logic, Method, Translation, ljt, sat};
use deduxit::{
    Error, ErrorKind, Limits, Phase, Refusal, Settings,
    linear::{Forest, Proof, ViewOptions},
};
use std::fmt::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Returns the ordinary sequent `text` holds in `format`: text in the
/// syntax of ordinary logic (a parse error points into `text`), or a TPTP
/// problem, refused at the first subformula past `most` occurrences.
///
/// # Errors
///
/// Text that is no ordinary sequent, with a caret under the place, no
/// TPTP problem, or one past `most`.
pub(crate) fn sequent_in(
    text: &str,
    format: InputFormat,
    most: crate::io::Most,
) -> Result<ordinary::Sequent> {
    let within = bound(most);
    let sequent = match format {
        // The library's error says that the text is no TPTP problem.
        InputFormat::Tptp => {
            ordinary::read_tptp(text, &within)
                .map_err(|e| refused(e, most))?
                .sequent
        }
        InputFormat::Text | InputFormat::Auto | InputFormat::Lines => {
            ordinary::Sequent::parse_within(text, &within).map_err(|e| match e {
                Error::Parse(_) => crate::parse_error(text, e),
                e => refused(e, most),
            })?
        }
        _ => bail!(
            "--logic reads text (one sequent, or one per line with --input-format lines) or TPTP problems"
        ),
    };
    Ok(sequent)
}

/// Returns `sequent` with the equality axioms as its last hypotheses where
/// it uses `=`, within `most`: `prove`, its batch, `interact` and `grade`
/// decide, prove by hand and grade the one sequent so, and every
/// derivation shows them. The transformations and `resolve` take the
/// sequent as written.
///
/// # Errors
///
/// The axioms past `most`.
pub(crate) fn with_equality(
    sequent: ordinary::Sequent,
    most: crate::io::Most,
) -> Result<ordinary::Sequent> {
    Ok(ordinary::equality::with_axioms(&sequent, &bound(most))
        .map_err(|e| refused(e, most))?
        .unwrap_or(sequent))
}

/// Returns what a verdict line adds for a derivation whose conclusion's
/// last `axioms` hypotheses are the equality axioms: nothing for none.
pub(crate) fn axioms_note(axioms: usize) -> String {
    match axioms {
        0 => String::new(),
        n => format!(", its last {n} hypotheses the equality axioms"),
    }
}

/// Returns how many of the last hypotheses of the ordinary sequent an
/// image translates are equality axioms.
pub(crate) fn image_axioms(image: &Image) -> usize {
    let sequent = image.ordinary();
    ordinary::equality::axiom_count(sequent.formulas(), sequent.left())
}

/// Returns how many of the last hypotheses of the derivation of a valid
/// outcome are equality axioms: `0` without a derivation.
fn outcome_axioms(outcome: &ordinary::Outcome) -> usize {
    match &outcome.verdict {
        ordinary::Verdict::Valid(ordinary::Evidence::Derivation(derivation)) => {
            ordinary::equality::axiom_count(derivation.formulas(), derivation.sequent().0)
        }
        _ => 0,
    }
}

/// Returns the image of an ordinary sequent in the logic, under the
/// translation, that `options` give, refused past `most` occurrences.
///
/// # Errors
///
/// The translation's error: one that does not decide the logic, more
/// than one formula right of `⊢` in intuitionistic or minimal logic, or
/// an image past `most`.
pub(crate) fn image(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    most: crate::io::Most,
) -> Result<Image> {
    ordinary::translate(sequent, options.logic, options.translation(), &bound(most))
        .map_err(|e| refused(e, most))
}

/// Returns the image of an ordinary sequent under the translation
/// `options` choose, with the image's forest, refused past
/// `limits.occurrences`.
///
/// # Errors
///
/// As [`image`], and an image past the bound.
pub(crate) fn translated(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    limits: &Limits,
) -> Result<(Forest, Image)> {
    let most = Most::of(limits);
    let image = image(options, sequent, most)?;
    let sequent = admit(image.sequent().clone(), most)?;
    Ok((Forest::from_owned(sequent, &bound(most))?, image))
}

/// Refuses `--translation` and `--linear`, which name the embedding,
/// beside a choice of LJT.
///
/// # Errors
///
/// Either flag beside `--method ljt` or `sat`, or beside a settings
/// file's `ordinary.method` of either.
pub(crate) fn method_given(args: &ProveArgs, options: &ordinary::Options) -> Result<()> {
    if let Some(
        method @ (Method::Ljt
        | Method::Sat
        | Method::Connection
        | Method::Grounding
        | Method::Models),
    ) = options.method
        && (args.logic.translation.is_some() || args.linear)
    {
        bail!(
            "--translation and --linear name the embedding, and {} decides without one: leave \
             them out, or give --method embedding",
            by(method)
        );
    }
    Ok(())
}

/// The part of the time limit, in percent, that the default's first-order
/// method (grounding or the connection prover) gets before the embedding
/// takes over with the rest. On ILTP's first-order problems at 2 s the
/// connection prover proves all but three of its theorems within 0.6 s,
/// and five of the seven the embedding alone proves take it under 6 ms.
pub(crate) const FIRST_ORDER_SHARE: u32 = 90;

/// Returns the deadline of the default's first-order method:
/// [`FIRST_ORDER_SHARE`] of the time limit where no method is given and the
/// sequent's method is grounding or the connection prover, else one that
/// never passes.
///
/// # Errors
///
/// A thread for the deadline that cannot be started.
pub(crate) fn first_order_share(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    deadline: &Deadline,
) -> Result<Deadline> {
    let first_order = matches!(
        options.method_for(sequent),
        Method::Connection | Method::Grounding
    );
    if options.method.is_none() && first_order {
        deadline.share(FIRST_ORDER_SHARE)
    } else {
        Deadline::start(None, Instant::now())
    }
}

/// The part of the time limit, in percent, that the default's connection
/// prover gets of a first-order classical sequent before the model search
/// takes the rest of [`FIRST_ORDER_SHARE`]. On ILTP's first-order problems
/// the model search finds each of its 210 structures within 0.4 s, a fifth
/// of a 2 s limit; at 70% the prover loses two of its 604 theorems at 2 s
/// and none at 5 s.
pub(crate) const CONNECTION_SHARE: u32 = 70;

/// The part of the time limit, in percent, the model search goes on for
/// after the connection prover gives way, before the embedding takes over:
/// on ILTP's first-order problems at 2 s, of the 30 structures it found
/// where the prover gave way first, it found every one within 2% of the
/// limit after, and the one theorem the embedding proves there took it
/// 2 ms.
pub(crate) const MODEL_GRACE: u32 = 5;

/// Returns how long the model search goes on after the connection prover
/// gives way: [`MODEL_GRACE`] of the time limit, or a tenth of a second
/// without one.
pub(crate) fn model_grace(deadline: &Deadline) -> Duration {
    deadline
        .limit()
        .map_or(Duration::from_millis(100), |limit| {
            limit * MODEL_GRACE / 100
        })
}

/// Returns the deadline of the default's connection prover on a
/// first-order classical sequent: [`CONNECTION_SHARE`] of the time limit
/// where no method is given and the sequent's method is grounding or the
/// connection prover in classical logic, else one that never passes.
///
/// # Errors
///
/// A thread for the deadline that cannot be started.
pub(crate) fn connection_share(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    deadline: &Deadline,
) -> Result<Deadline> {
    let first_order = matches!(
        options.method_for(sequent),
        Method::Connection | Method::Grounding
    );
    if options.method.is_none() && first_order && options.logic == Logic::Classical {
        deadline.share(CONNECTION_SHARE)
    } else {
        Deadline::start(None, Instant::now())
    }
}

/// Returns whether the command races the model search beside the
/// embedding: a first-order sequent in intuitionistic or minimal logic with
/// no method and no translation given (the embedding decides it, or takes
/// over from grounding), on more than one thread (`--jobs 1` and
/// `--deterministic` leave the embedding alone).
pub(crate) fn races_the_embedding(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    search: &deduxit::linear::search::Options,
) -> bool {
    models_beside(options, sequent) && search.jobs.count() > 1
}

/// Returns whether the default asks the model search beside the embedding:
/// a first-order sequent in intuitionistic or minimal logic with no method
/// and no translation given.
pub(crate) fn models_beside(options: &ordinary::Options, sequent: &ordinary::Sequent) -> bool {
    options.method.is_none()
        && options.translation.is_none()
        && options.logic != Logic::Classical
        && sequent.formulas().is_first_order()
}

/// Returns the model search's refutation of a sequent [`models_beside`]
/// names, searched first on one thread within its grace ([`model_grace`]),
/// with how long it took; `None` where it found none, and the embedding
/// takes the rest of the time. The command's race needs a second thread;
/// on one, a structure found in the grace is the answer the race would
/// give, at the cost of that grace to the embedding (`--jobs 1`,
/// `--deterministic` and a batch's entry).
pub(crate) fn model_first(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    search: &deduxit::linear::search::Options,
    limits: &deduxit::Limits,
    deadline: &Deadline,
    halt: &dyn Fn() -> bool,
) -> Option<(ordinary::Outcome, Duration)> {
    let start = Instant::now();
    let grace = model_grace(deadline);
    let modelled = ordinary::models::decide(sequent, options.logic, search, limits, |_| {
        halt() || start.elapsed() >= grace
    });
    match modelled {
        Ok(
            refuted @ ordinary::models::Outcome {
                verdict: ordinary::Verdict::NotValid,
                ..
            },
        ) => Some((ordinary::Outcome::from(refuted), start.elapsed())),
        Ok(_) | Err(_) => None,
    }
}

/// Returns whether the default's first-order method stopped at its share of
/// the time limit, not at the limit or Ctrl-C: the embedding then decides
/// with the time left.
pub(crate) fn stopped_at_share(
    decided: &Result<ordinary::Outcome, Error>,
    share: &Deadline,
    deadline: &Deadline,
) -> bool {
    let stopped = match decided {
        Ok(outcome) => matches!(
            outcome.verdict,
            ordinary::Verdict::Unknown(deduxit::linear::search::Reason::Stopped)
        ),
        Err(Error::Refused(Refusal::Stopped { .. })) => true,
        Err(_) => false,
    };
    stopped && share.passed() && !deadline.passed() && !interrupted()
}

/// Returns how the verdict line names a method that decides without the
/// embedding: `LJT` or `the SAT solver`.
pub(crate) const fn by(method: Method) -> &'static str {
    match method {
        Method::Sat => "the SAT solver",
        Method::Table => "its truth table",
        Method::Tableau => "a tableau",
        Method::Connection => "the connection prover",
        Method::Grounding => "grounding",
        Method::Models => "the model search",
        _ => "LJT",
    }
}

/// Decides an ordinary sequent by a method other than the embedding, as
/// the options' method for it says, until `halt` says to stop. With no
/// method given, the SAT solver takes LJT's derivation where LJT gives one,
/// as the library's default does, and a first-order classical sequent the
/// connection prover decides is searched for a finite structure beside it
/// ([`by_connection`]). `stop` is asked on the calling thread, the race's
/// `halt` on the model search's.
pub(crate) fn decide_natively(
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    search: &deduxit::linear::search::Options,
    limits: &deduxit::Limits,
    race: &Race<'_>,
    mut stop: impl FnMut(deduxit::Progress) -> bool,
) -> Result<ordinary::Outcome, Error> {
    let logic = options.logic;
    match options.method_for(sequent) {
        // The default keeps the connection prover's derivation of LK of a
        // valid classical sequent where it gives one within the bound of
        // the evidence, and where grounding gives way at a bound the
        // connection prover decides, as the library's default does.
        Method::Grounding => {
            let named = options.with_method(Some(Method::Grounding));
            // The default grounds within the library default's bound.
            let most = ordinary::grounding::DEFAULT_NODES;
            let grounding_limits = if options.method.is_none() {
                limits.with_occurrences(Some(limits.occurrences.map_or(most, |o| o.min(most))))
            } else {
                *limits
            };
            let grounded =
                ordinary::decide_within(sequent, &named, search, &grounding_limits, &mut stop);
            let classical = options.method.is_none() && logic == Logic::Classical;
            if classical && ordinary::gives_way(&grounded) {
                return by_connection(sequent, logic, search, limits, race, &mut stop);
            }
            let grounded = grounded?;
            if classical
                && matches!(grounded.verdict, ordinary::Verdict::Valid(_))
                && let Ok(connected) = ordinary::connection::decide(
                    sequent,
                    logic,
                    search,
                    &bounded(limits),
                    &mut stop,
                )
                && matches!(
                    connected.verdict,
                    ordinary::Verdict::Valid(ordinary::Evidence::Derivation(_))
                )
            {
                return Ok(ordinary::Outcome::from(connected));
            }
            Ok(grounded)
        }
        Method::Connection if options.method.is_none() && logic == Logic::Classical => {
            by_connection(sequent, logic, search, limits, race, &mut stop)
        }
        Method::Connection => ordinary::connection::decide(sequent, logic, search, limits, stop)
            .map(ordinary::Outcome::from),
        Method::Models => ordinary::models::decide(sequent, logic, search, limits, stop)
            .map(ordinary::Outcome::from),
        Method::Sat if options.method.is_none() => {
            sat::decide_preferring_ljt(sequent, logic, limits, stop).map(ordinary::Outcome::from)
        }
        Method::Sat => sat::decide(sequent, logic, limits, stop).map(ordinary::Outcome::from),
        Method::Table | Method::Tableau => {
            let search = deduxit::linear::search::Options::default();
            ordinary::decide_within(sequent, options, &search, limits, stop)
        }
        _ => ljt::decide(sequent, logic, limits, stop).map(ordinary::Outcome::from),
    }
}

/// What the default's race of a first-order classical sequent reads beside
/// the caller's stop: the connection prover's deadline on one thread, the
/// model search's grace after the prover gives way, and the stop of the
/// model search's thread.
#[derive(Clone, Copy)]
pub(crate) struct Race<'a> {
    /// The connection prover's deadline on one thread
    /// ([`connection_share`]).
    pub(crate) connection: &'a Deadline,
    /// How long the model search goes on after the prover gives way
    /// ([`model_grace`]).
    pub(crate) grace: Duration,
    /// Whether to stop, asked on the model search's thread.
    pub(crate) halt: &'a (dyn Fn() -> bool + Sync),
}

/// Decides a first-order classical sequent as the default does: the
/// connection prover, and beside it on a thread of its own the model
/// search, either verdict from the prover), each with half the memory
/// bound. Where the prover gives way (a bound other than the stop) the
/// model search goes on for `grace` at most, so that the embedding, which
/// the caller runs next, gets the time left: a theorem the embedding
/// proves at once is not kept waiting. Which of the two answers a sequent
/// both refute depends on which finds it first, as in the linear search's
/// race. On one thread (`--jobs 1`, `--deterministic`, or where no thread
/// starts) the prover searches until `connection` passes, and the model
/// search takes the rest of the time where the prover stopped there, or
/// `grace` where it gave way. The calling thread asks `stop`, the model
/// search's thread the race's `halt`.
///
/// On ILTP's first-order problems at 2 s, the model search found each of
/// its 210 structures within 0.4 s, while the prover proves two theorems
/// only after 1.4 s: run one after the other within the first-order share,
/// one of them would lose the other's answers.
fn by_connection(
    sequent: &ordinary::Sequent,
    logic: Logic,
    search: &deduxit::linear::search::Options,
    limits: &deduxit::Limits,
    race: &Race<'_>,
    stop: &mut dyn FnMut(deduxit::Progress) -> bool,
) -> Result<ordinary::Outcome, Error> {
    let Race {
        connection,
        grace,
        halt,
    } = *race;
    let stopped = |decided: &Result<ordinary::Outcome, Error>| {
        matches!(
            decided,
            Ok(ordinary::Outcome {
                verdict: ordinary::Verdict::Unknown(deduxit::linear::search::Reason::Stopped),
                ..
            }) | Err(Error::Refused(Refusal::Stopped { .. }))
        )
    };
    if search.jobs.count() > 1 {
        let half = limits.with_memory_bytes(limits.memory_bytes.map(|m| m / 2));
        let done = AtomicBool::new(false);
        // When the prover gave way: the model search ends `grace` after.
        let gave_way: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
        let raced = std::thread::scope(|scope| {
            let modelled = std::thread::Builder::new()
                .name("model search".into())
                .spawn_scoped(scope, || {
                    let modelled = ordinary::models::decide(sequent, logic, search, &half, |_| {
                        halt()
                            || done.load(Ordering::Relaxed)
                            || gave_way.get().is_some_and(|at| at.elapsed() >= grace)
                    });
                    if matches!(
                        &modelled,
                        Ok(ordinary::models::Outcome {
                            verdict: ordinary::Verdict::NotValid,
                            ..
                        })
                    ) {
                        done.store(true, Ordering::Relaxed);
                    }
                    modelled
                })
                .ok()?;
            let connected = ordinary::connection::decide(sequent, logic, search, &half, |p| {
                stop(p) || done.load(Ordering::Relaxed)
            });
            // The prover's verdict ends the model search, its giving way
            // leaves it `grace`, and its stop at the share leaves it to the
            // caller's stop.
            let at_stop = matches!(
                &connected,
                Ok(ordinary::connection::Outcome {
                    verdict: ordinary::Verdict::Unknown(deduxit::linear::search::Reason::Stopped),
                    ..
                }) | Err(Error::Refused(Refusal::Stopped { .. }))
            );
            let decided = matches!(
                &connected,
                Ok(ordinary::connection::Outcome {
                    verdict: ordinary::Verdict::Valid(_) | ordinary::Verdict::NotValid,
                    ..
                })
            );
            if decided {
                done.store(true, Ordering::Relaxed);
            } else if !at_stop {
                let _ = gave_way.set(Instant::now());
            }
            let modelled = match modelled.join() {
                Ok(modelled) => modelled,
                Err(panic) => std::panic::resume_unwind(panic),
            };
            Some((connected, modelled))
        });
        if let Some((connected, modelled)) = raced {
            let connected = connected.map(ordinary::Outcome::from);
            return match (connected, modelled) {
                (
                    connected @ Ok(ordinary::Outcome {
                        verdict: ordinary::Verdict::Valid(_) | ordinary::Verdict::NotValid,
                        ..
                    }),
                    _,
                ) => connected,
                (
                    _,
                    Ok(
                        modelled @ ordinary::models::Outcome {
                            verdict: ordinary::Verdict::NotValid,
                            ..
                        },
                    ),
                ) => Ok(ordinary::Outcome::from(modelled)),
                // A defect of either is the answer.
                (Err(error @ Error::Countermodel { .. }), _)
                | (_, Err(error @ Error::Countermodel { .. })) => Err(error),
                // The prover's unknown, which the embedding takes over from.
                (connected, _) => connected,
            };
        }
    }
    let connected = ordinary::connection::decide(sequent, logic, search, limits, |p| {
        stop(p) || connection.passed()
    })
    .map(ordinary::Outcome::from);
    if stopped(&connected) && connection.passed() && !halt() {
        return ordinary::models::decide(sequent, logic, search, limits, stop)
            .map(ordinary::Outcome::from);
    }
    if ordinary::gives_way(&connected) {
        let after = Instant::now();
        let modelled = ordinary::models::decide(sequent, logic, search, limits, |p| {
            stop(p) || after.elapsed() >= grace
        });
        if matches!(
            &modelled,
            Ok(ordinary::models::Outcome {
                verdict: ordinary::Verdict::NotValid,
                ..
            })
        ) {
            return modelled.map(ordinary::Outcome::from);
        }
    }
    connected
}

/// What deciding an ordinary sequent by LJT or the SAT solver leaves to
/// its caller.
pub(crate) enum Native<T> {
    /// The answer, written: `prove`'s exit status, or a batch's entry.
    Answered(T),
    /// The logic's default ended at a bound, and the embedding decides in
    /// its place.
    GivesWay(GaveWay),
    /// The SAT solver decided a classical sequent by default, and the
    /// embedding is asked for its evidence within [`bounded`]; where it
    /// gives none, this outcome is written. The instant is the search's
    /// start.
    Solved(Box<ordinary::Outcome>, Instant),
}

/// Why the logic's default gave way to the embedding: the method and its
/// reason, or nothing where it did not run.
pub(crate) type GaveWay = Option<String>;

/// Returns whether the options' method is the classical default, the SAT
/// solver that keeps the embedding's evidence where the embedding gives
/// it within [`bounded`].
pub(crate) fn keeps_evidence(options: &ordinary::Options, sequent: &ordinary::Sequent) -> bool {
    options.method.is_none()
        && options.method_for(sequent) == Method::Sat
        && options.logic == Logic::Classical
}

/// Returns the limits of the embedding's translation and search for the
/// evidence of a sequent the SAT solver decided: at most
/// [`ordinary::EVIDENCE_WORK`] occurrences and units of work, or the
/// caller's bounds where they are lower, as the library's default has them.
pub(crate) fn bounded(limits: &Limits) -> Limits {
    let most = ordinary::EVIDENCE_WORK;
    limits
        .with_work(Some(limits.work.map_or(most, |w| w.min(most))))
        .with_occurrences(Some(limits.occurrences.map_or(most, |o| o.min(most))))
}

/// What the embedding gives for a sequent the SAT solver decided.
pub(crate) enum Embedded {
    /// No evidence: no verdict, or a proof whose derivation of LK a bound
    /// or the stop refused. The solver's outcome is written.
    Nothing,
    /// The derivation of LK of its proof, read back and checked.
    Derivation(Box<Derivation>),
    /// A refutation of the image.
    Refutation,
}

/// Gives the embedding's ordinary outcome the countermodel of the SAT
/// solver that decided first, where its refutation gave none (one by
/// exhaustion or by a count), as the library's default does.
pub(crate) fn solver_model(outcome: &mut ordinary::Outcome, solved: Option<&ordinary::Outcome>) {
    if let (ordinary::Verdict::NotValid, None, Some(solved)) =
        (&outcome.verdict, &outcome.countermodel, solved)
    {
        outcome.countermodel.clone_from(&solved.countermodel);
    }
}

/// Returns the checker's refusal of a derivation the library read back
/// from the command's own proof as what it is: a defect of deduxit, not a
/// fault of the input (the library maps its own read-backs so; the command
/// reads back itself).
fn read_back_fault(error: Error) -> anyhow::Error {
    anyhow!(error)
        .context("the derivation read back fails its check, a defect of deduxit to report")
}

/// Returns what the embedding's `outcome` on the image gives for a
/// sequent the SAT solver `solved`, as the library's default asks.
pub(crate) fn evidence(
    outcome: &Outcome,
    image: &Image,
    solved: &ordinary::Outcome,
    limits: &Limits,
    halt: impl Fn() -> bool,
) -> Result<Embedded> {
    match (&outcome.verdict, &solved.verdict) {
        (Verdict::Proved(proof), ordinary::Verdict::Valid(_)) => {
            let derivation = match image.read_back(proof, limits, |_| halt()) {
                Ok(derivation) => derivation,
                Err(error @ Error::ReadBack { .. }) => return Err(error.into()),
                Err(_) => return Ok(Embedded::Nothing),
            };
            // A read-back the checker refuses is a defect, said as such.
            match derivation.check(limits, |_| halt()) {
                Ok(()) => Ok(Embedded::Derivation(Box::new(derivation))),
                Err(error @ Error::ReadBack { .. }) => Err(error.into()),
                Err(error @ Error::InvalidDerivation { .. }) => Err(read_back_fault(error)),
                Err(_) => Ok(Embedded::Nothing),
            }
        }
        (Verdict::Unprovable(_), ordinary::Verdict::NotValid) => Ok(Embedded::Refutation),
        (Verdict::Unknown(_), _) => Ok(Embedded::Nothing),
        (Verdict::Proved(_) | Verdict::Unprovable(_), _) => {
            bail!("the embedding contradicts the SAT solver's verdict; this is a defect")
        }
    }
}

/// Returns why a method's answer gave way to the embedding: the method,
/// and the bound it ended at.
fn gave_way(method: Method, decided: &Result<ordinary::Outcome, Error>) -> GaveWay {
    let why = match decided {
        Ok(outcome) => match &outcome.verdict {
            // A method that passes a sequent outside its reach on (grounding
            // outside its classes) says nothing a user can act on.
            ordinary::Verdict::Unknown(Reason::Incomplete) => return None,
            ordinary::Verdict::Unknown(reason) => reason.to_string(),
            _ => return None,
        },
        Err(error) => error.to_string(),
    };
    Some(format!("{} gave no verdict: {why}", by(method)))
}

/// Decides an ordinary sequent by LJT or the SAT solver, as the options'
/// method says, and writes what `prove` writes for it ([`write_native`]).
/// Writes nothing where the logic's default gives way to the embedding
/// (it ended at a bound other than the time limit or Ctrl-C, or a bound
/// refused the derivation of the proof it found), or where the classical
/// default decided and the embedding is asked for its evidence; with
/// `--net`, the classical default leaves the sequent to the embedding at
/// once.
///
/// # Errors
///
/// `--net` beside any other method, which draws a linear proof; a refusal
/// that is no reason of an unknown (the tableau's steps past the
/// derivation's bound, a first-order read-back past the terms' bound); an
/// output that cannot be written.
pub(crate) fn by_native(
    args: &ProveArgs,
    settings: &Settings,
    options: &ordinary::Options,
    sequent: &ordinary::Sequent,
    deadline: &Deadline,
) -> Result<Native<Status>> {
    let method = options.method_for(sequent);
    let keeps = keeps_evidence(options, sequent);
    if args.output.net {
        if keeps {
            return Ok(Native::GivesWay(None));
        }
        bail!(
            "--net draws the proof net of a linear proof, and {} makes none: add --method \
             embedding",
            by(method)
        );
    }
    let limits = settings.limits;
    // The logic's default, which the embedding takes over from.
    let gives_way = options.method.is_none();
    catch_interrupt();
    on_large_stack(limits.stack_bytes(), || {
        let start = Instant::now();
        let notice = Notice::start(NOTICE_AFTER, notice_line(deadline.limit(), false));
        let share = first_order_share(options, sequent, deadline)?;
        let connection = connection_share(options, sequent, deadline)?;
        let halt = || interrupted() || deadline.passed() || share.passed();
        // Whether the read-back or a check began, which the verdict line of
        // a stop says (LJT's and the SAT solver's follow their search).
        let after_search = std::cell::Cell::new(false);
        let decided = decide_natively(
            options,
            sequent,
            &settings.search,
            &limits,
            &Race {
                connection: &connection,
                grace: model_grace(deadline),
                halt: &halt,
            },
            |p| {
                if p.phase != Phase::Search {
                    after_search.set(true);
                }
                halt()
            },
        );
        drop(notice);
        // The method that ran last: the model search after the connection
        // prover, where it took over.
        let last = decided.as_ref().map_or(method, ordinary::Outcome::method);
        if gives_way && stopped_at_share(&decided, &share, deadline) {
            return Ok(Native::GivesWay(Some(format!(
                "{} gave no verdict within {FIRST_ORDER_SHARE}% of the time limit",
                by(last)
            ))));
        }
        if gives_way && ordinary::gives_way(&decided) {
            return Ok(Native::GivesWay(gave_way(last, &decided)));
        }
        if keeps
            && let Ok(solved) = &decided
            && matches!(
                solved.verdict,
                ordinary::Verdict::Valid(_) | ordinary::Verdict::NotValid
            )
        {
            return Ok(Native::Solved(Box::new(solved.clone()), start));
        }
        let ended = Ended {
            stop: stopped(deadline),
            elapsed: start.elapsed(),
            after_search: after_search.get() && matches!(method, Method::Ljt | Method::Sat),
        };
        let outcome = match decided {
            Ok(outcome) => outcome,
            // Only the table and the tableau answer the stop with no
            // outcome, since theirs holds the table or the tableau the stop
            // left unbuilt; every other method answers it as an unknown.
            Err(Error::Refused(Refusal::Stopped { .. })) => {
                let line = format!(
                    "unknown ({} logic by {}): {}",
                    options.logic,
                    by(method),
                    Stop::why(ended.stop)
                );
                match (args.output.format(), &args.output.output) {
                    (Format::Text, None) => io::write(None, &format!("{line}\n"))?,
                    _ => eprintln!("{line}"),
                }
                return Ok(Native::Answered(Status::Unknown));
            }
            Err(error @ Error::Refused(Refusal::Atoms { .. })) => bail!(
                "{error}; raise the limit with --table-atoms, or decide the sequent by another \
                 method (--method tableau)"
            ),
            Err(error @ Error::Refused(_)) => {
                // Only the tableau's steps are held to the drawing's bound.
                let flags = if matches!(error, Error::Refused(Refusal::Output { .. })) {
                    "--derivation-limit"
                } else {
                    "--memory-limit, --work-limit"
                };
                bail!(
                    "{} gave no verdict: {error}; raise the limit ({flags}), or decide through \
                     the embedding with --method embedding",
                    by(method)
                )
            }
            Err(error) => return Err(error.into()),
        };
        write_native(args, settings, &outcome, &ended, deadline).map(Native::Answered)
    })?
}

/// Writes what `prove` writes for an ordinary sequent LJT or the SAT
/// solver decided: the verdict line, the checked derivation of LJ or LK,
/// its lambda-term and the statistics, or the outcome as JSON. Returns
/// the exit status.
///
/// # Errors
///
/// An output that cannot be written.
pub(crate) fn write_native(
    args: &ProveArgs,
    settings: &Settings,
    outcome: &ordinary::Outcome,
    ended: &Ended,
    deadline: &Deadline,
) -> Result<Status> {
    let format = args.output.format();
    let show = Show::new(&args.output, settings)?;
    let status = match &outcome.verdict {
        ordinary::Verdict::Valid(_) => Status::Yes,
        ordinary::Verdict::NotValid => Status::No,
        _ => Status::Unknown,
    };
    let over = || interrupted() || deadline.passed() || crate::io::closed();
    let mut out = io::Output::open(args.output.output.as_deref(), format.is_binary())?;
    if format == Format::Json {
        write_json(outcome, &show, over, &mut out)?;
        out.finish()?;
        return Ok(status);
    }
    let line = show.verdict.then(|| native_verdict_line(outcome, ended));
    let prefix = show.open(&mut out, line.as_deref())?;
    let why = || Stop::why(stopped(deadline));
    // A truth table or a tableau is the text in place of a derivation.
    let laid_out = match &outcome.search {
        ordinary::Search::Table(table) => Some(table.to_string()),
        ordinary::Search::Tableau(tableau) => Some(tableau.to_string()),
        _ => None,
    };
    let shown = match (&outcome.verdict, args.output.quiet) {
        (_, false) if laid_out.is_some() && format == Format::Text => match (show.tree, laid_out) {
            (Tree::Never, _) | (_, None) => Shown::Nothing,
            (_, Some(text)) => {
                show.after(&mut out, &text, false)?;
                Shown::Written
            }
        },
        (ordinary::Verdict::Valid(_), false) if show.tree == Tree::Never => Shown::Nothing,
        (ordinary::Verdict::Valid(_), false) if over() => {
            Shown::LeftOut(format!("the derivation is not written: {}", why()))
        }
        (ordinary::Verdict::Valid(evidence), false) => match evidence.derivation() {
            Ok(d) => crate::show::shown(
                shown(d.clone(), &show),
                show.fit(),
                &show,
                every_steps(&over),
                why,
                prefix.as_deref(),
                &mut out,
            )?,
            // A clausal proof, which the verdict line names.
            Err(error) => Shown::LeftOut(error.to_string()),
        },
        (ordinary::Verdict::NotValid, false)
            if matches!(
                format,
                Format::Svg | Format::Png | Format::Pdf | Format::Latex | Format::Typst
            ) && show.tree != Tree::Never =>
        {
            match &outcome.countermodel {
                Some(countermodel) => crate::show::model_drawn(
                    countermodel,
                    outcome.logic,
                    &show,
                    prefix.as_deref(),
                    &mut out,
                    &over,
                    &why,
                )?,
                None => match crate::show::FirstOrderModel::of(outcome) {
                    Some(model) => crate::show::first_order_model_drawn(
                        model,
                        outcome.logic,
                        &show,
                        prefix.as_deref(),
                        &mut out,
                        &over,
                        &why,
                    )?,
                    None => Shown::Nothing,
                },
            }
        }
        (ordinary::Verdict::NotValid, false) if format == Format::Rocq => {
            match &outcome.countermodel {
                Some(countermodel) => crate::show::countermodel_into(
                    countermodel,
                    outcome.logic,
                    &show,
                    prefix.as_deref(),
                    &mut out,
                    over,
                ),
                None => match crate::show::FirstOrderModel::of(outcome) {
                    Some(model) => crate::show::first_order_certificate_into(
                        model,
                        outcome.logic,
                        &show,
                        prefix.as_deref(),
                        &mut out,
                        over,
                    ),
                    None => Shown::Nothing,
                },
            }
        }
        _ => Shown::Nothing,
    };
    let drawn = matches!(shown, Shown::Written | Shown::Rendered(_));
    let keep = show.close(&mut out, shown, line.as_deref())?;
    if let (ordinary::Verdict::Valid(evidence), Format::Text, Some(_)) =
        (&outcome.verdict, format, show.lambda)
    {
        let term = evidence
            .derivation()
            .and_then(|d| d.term(&show.limits, |_| over()));
        show.after(&mut out, &lambda_line(term, &show, over), drawn)?;
    }
    // The countermodel's worlds, after the verdict, as text; the drawing
    // formats show none.
    if let (ordinary::Verdict::NotValid, Some(countermodel), Format::Text, false) = (
        &outcome.verdict,
        &outcome.countermodel,
        format,
        args.output.quiet,
    ) && show.tree != Tree::Never
    {
        show.after(&mut out, &countermodel.model.to_string(), drawn)?;
    }
    // Grounding's finite structure, likewise.
    if let (ordinary::Verdict::NotValid, Some(structure), Format::Text, false) = (
        &outcome.verdict,
        &outcome.structure,
        format,
        args.output.quiet,
    ) && show.tree != Tree::Never
    {
        show.after(&mut out, &structure.structure.to_string(), drawn)?;
    }
    // The model search's Kripke model, likewise.
    if let (ordinary::Verdict::NotValid, Some(kripke), Format::Text, false) =
        (&outcome.verdict, &outcome.kripke, format, args.output.quiet)
        && show.tree != Tree::Never
    {
        show.after(&mut out, &kripke.model.to_string(), drawn)?;
    }
    if args.stats {
        show.after(&mut out, &native_statistics(outcome, ended.elapsed), drawn)?;
    }
    if keep {
        out.finish()?;
    }
    Ok(status)
}

/// Writes an ordinary outcome as JSON, with the lambda-term of its
/// derivation under the key `lambda` where `--lambda` asks for one.
///
/// # Errors
///
/// An output that cannot be written.
pub(crate) fn write_json(
    outcome: &ordinary::Outcome,
    show: &Show,
    over: impl Fn() -> bool,
    out: &mut io::Output,
) -> Result<()> {
    match &outcome.verdict {
        ordinary::Verdict::Valid(evidence) if show.lambda.is_some() => {
            let term = evidence
                .derivation()
                .and_then(|d| d.term(&show.limits, |_| over()));
            let mut value = serde_json::to_value(outcome)?;
            if let Some(object) = value.as_object_mut() {
                object.insert("lambda".into(), lambda_json(term, show, over)?);
            }
            serde_json::to_writer(out.stream(), &value)?;
        }
        _ => serde_json::to_writer(out.stream(), outcome)?,
    }
    Ok(())
}

/// Returns the verdict line of an ordinary sequent decided by LJT or the
/// SAT solver: the verdict, the logic and the method, for a sequent valid
/// by a clausal proof that proof, and for an undecided sequent why.
pub(crate) fn native_verdict_line(outcome: &ordinary::Outcome, ended: &Ended) -> String {
    let method = outcome.method();
    let context = format!("{} logic by {}", outcome.logic, by(method));
    match &outcome.verdict {
        ordinary::Verdict::Valid(ordinary::Evidence::Clausal(proof)) => {
            format!("valid ({context}): {proof}")
        }
        ordinary::Verdict::Valid(_) if method == Method::Table => {
            format!("valid ({context}): no row falsifies it")
        }
        ordinary::Verdict::Valid(_) if method == Method::Tableau => {
            format!("valid ({context}): every branch closes, a checked derivation of LK")
        }
        ordinary::Verdict::Valid(_) => {
            format!("valid ({context}){}", axioms_note(outcome_axioms(outcome)))
        }
        ordinary::Verdict::NotValid => match &outcome.search {
            ordinary::Search::Table(table) => {
                let rows = table.rows();
                let falsifying = table.falsifying().len();
                let checked = outcome.countermodel.is_some();
                let rows = match (falsifying, checked) {
                    (1, true) => {
                        format!("1 of its {rows} rows falsifies it, a checked countermodel")
                    }
                    (1, false) => format!("1 of its {rows} rows falsifies it"),
                    (_, true) => format!(
                        "{falsifying} of its {rows} rows falsify it, the first a checked \
                         countermodel"
                    ),
                    (_, false) => format!("{falsifying} of its {rows} rows falsify it"),
                };
                format!("not valid ({context}): {rows}")
            }
            ordinary::Search::Tableau(_) if outcome.countermodel.is_some() => {
                format!("not valid ({context}): an open branch gives a checked countermodel")
            }
            ordinary::Search::Tableau(_) => format!(
                "not valid ({context}): an open branch gives a countermodel, too large to check"
            ),
            _ => not_valid(outcome, method, &context),
        },
        // Grounding grounds nothing outside its classes, and outside
        // classical logic a valid ground sequent says nothing.
        ordinary::Verdict::Unknown(deduxit::linear::search::Reason::Incomplete) if matches!(&outcome.search, ordinary::Search::Grounding(s) if s.domain == 0) =>
        {
            format!(
                "unknown ({context}): the sequent lies outside the classes grounding decides: \
                 no function symbols, and either no ∀ under an ∃ or no predicate of two or more \
                 arguments"
            )
        }
        ordinary::Verdict::Unknown(deduxit::linear::search::Reason::Incomplete)
            if matches!(&outcome.search, ordinary::Search::Grounding(_)) =>
        {
            format!(
                "unknown ({context}): the sequent is valid in classical logic, which says nothing \
                 of {} logic",
                outcome.logic
            )
        }
        ordinary::Verdict::Unknown(reason) => {
            format!("unknown ({context}): {}", advice(*reason, "", ended))
        }
        _ => format!("unknown ({context})"),
    }
}

/// Returns the verdict line of a sequent LJT or the SAT solver found not
/// valid: with its countermodel, too large a one, or none.
fn not_valid(outcome: &ordinary::Outcome, method: Method, context: &str) -> String {
    if let Some(structure) = &outcome.structure {
        let elements = match structure.structure.size {
            1 => "1 element".to_owned(),
            n => format!("{n} elements"),
        };
        return format!(
            "not valid ({context}): a checked finite structure of {elements} refutes it"
        );
    }
    if let Some(kripke) = &outcome.kripke {
        let worlds = match kripke.model.worlds.len() {
            1 => "1 world".to_owned(),
            n => format!("{n} worlds"),
        };
        return format!(
            "not valid ({context}): a checked Kripke model of {worlds} with growing domains \
             refutes it"
        );
    }
    if outcome.countermodel.is_some() {
        return format!("not valid ({context}): a checked countermodel refutes it");
    }
    if let ordinary::Search::Grounding(s) = &outcome.search {
        return format!(
            "not valid ({context}): a finite structure of {} elements refutes it, too large to \
             check",
            s.domain
        );
    }
    if method == Method::Sat {
        let worlds = match &outcome.search {
            ordinary::Search::Sat(s) => s.worlds,
            _ => 0,
        };
        return format!(
            "not valid ({context}): a countermodel of {worlds} worlds refutes it, too large to \
             show and check"
        );
    }
    format!("not valid ({context}): the search was exhaustive")
}

/// Returns the lines `--stats` prints for LJT or the SAT solver: its
/// counters, a line each, and the time.
pub(crate) fn native_statistics(outcome: &ordinary::Outcome, elapsed: Duration) -> String {
    let mut lines = String::new();
    let counters: &[(&str, u64)] = match &outcome.search {
        ordinary::Search::Ljt(s) => &[
            ("sequents expanded", s.sequents),
            ("memo hits", s.memo_hits),
            ("memo entries", s.memo_entries),
            ("refuted by an earlier failure", s.subsumed),
        ],
        ordinary::Search::Sat(s) => &[
            ("solver calls", s.calls),
            ("conflicts", s.conflicts),
            ("lemmas", s.lemmas),
            ("worlds", s.worlds),
            ("facts derived", s.facts),
            ("clauses", s.clauses),
            ("proof steps", s.proof_steps),
        ],
        ordinary::Search::Table(table) => &[
            ("rows", table.rows()),
            ("falsifying rows", table.falsifying().len() as u64),
        ],
        ordinary::Search::Connection(s) => &[
            ("steps", s.steps),
            ("extension steps", s.extensions),
            ("reduction steps", s.reductions),
            ("literals closed by a lemma", s.lemmata),
            ("bound on the active path", u64::from(s.path_length)),
        ],
        ordinary::Search::Grounding(s) => &[
            ("elements", s.domain),
            ("ground atoms", s.atoms),
            ("formulas of the ground sequent", s.nodes),
            ("solver calls", s.sat.calls),
            ("conflicts", s.sat.conflicts),
            ("proof steps", s.sat.proof_steps),
        ],
        ordinary::Search::Models(s) => &[
            ("largest structure tried, elements", s.size),
            ("worlds of the last Kripke model tried", s.worlds),
            ("atoms", s.atoms),
            ("formulas", s.nodes),
            ("solver calls", s.sat.calls),
            ("conflicts", s.sat.conflicts),
        ],
        ordinary::Search::Tableau(tableau) => &[
            ("lines", u64::from(tableau.line_count())),
            (
                "branches",
                tableau
                    .nodes()
                    .iter()
                    .filter(|n| n.children().is_empty())
                    .count() as u64,
            ),
        ],
        _ => &[],
    };
    // A finite structure's search tried no Kripke model.
    for (label, value) in counters
        .iter()
        .filter(|&&(label, value)| value > 0 || !label.starts_with("worlds of"))
    {
        writeln!(lines, "{label}: {value}").expect("a string takes it");
    }
    // LJT's search, where it gave the SAT solver's derivation.
    if let ordinary::Search::Sat(sat::Statistics { ljt: Some(s), .. }) = &outcome.search {
        writeln!(lines, "derivation by LJT: {} sequents expanded", s.sequents)
            .expect("a string takes it");
    }
    write!(lines, "time: {elapsed:.2?}").expect("a string takes it");
    lines
}

/// Returns the first line of the text output for an ordinary sequent
/// decided in `logic` through its image under `translation`: the verdict,
/// the logic and the translation, the engine, and for an undecided
/// sequent why. A valid one says that the last `axioms` hypotheses are the
/// equality axioms ([`image_axioms`]).
pub(crate) fn verdict_line(
    outcome: &Outcome,
    logic: Logic,
    translation: Translation,
    incomplete: bool,
    axioms: usize,
    ended: &Ended,
) -> String {
    let context = format!(
        "{logic} logic by the {translation} translation into {}, {} engine",
        translation.target(),
        outcome.engine
    );
    match &outcome.verdict {
        Verdict::Proved(_) => format!("valid ({context}){}", axioms_note(axioms)),
        Verdict::Unprovable(_) if incomplete => format!(
            "unknown ({context}): the translated sequent is unprovable, which leaves a \
             first-order sequent open, since the translation may lose its proofs"
        ),
        Verdict::Unprovable(refutation) => {
            format!("not valid ({context}): the translated sequent is unprovable: {refutation}")
        }
        Verdict::Unknown(reason) => {
            format!("unknown ({context}): {}", unknown(*reason, outcome, ended))
        }
    }
}

/// Returns whether the image's search found its sequent unprovable where
/// that leaves the ordinary sequent unknown: at first order, where the
/// translation may lose proofs.
pub(crate) fn incomplete(outcome: &Outcome, image: &Image) -> bool {
    matches!(outcome.verdict, Verdict::Unprovable(_))
        && image.ordinary().formulas().is_first_order()
}

/// Returns the exit status of an image's outcome: unknown where
/// [`incomplete`] says so, else the image's verdict's.
pub(crate) fn status(outcome: &Outcome, image: Option<&Image>) -> crate::Status {
    match image {
        Some(image) if incomplete(outcome, image) => crate::Status::Unknown,
        _ => crate::Status::from(&outcome.verdict),
    }
}

/// Makes the derivation of LK or LJ a proof of the image reads back as
/// and writes it in the format `show` names, as
/// [`derivation`](crate::show::derivation) does for a linear one: within
/// the view's bounds, the time limit and Ctrl-C, fitted to a terminal.
/// The read-back is checked before it is written; one that fails the
/// check is an error, a defect to report. A derivation `read` back and
/// checked already is written as it is.
#[expect(
    clippy::too_many_arguments,
    reason = "the output's parts, as show::derivation takes them"
)]
pub(crate) fn derivation(
    image: &Image,
    proof: &Proof,
    read: Option<Derivation>,
    show: &Show,
    mut halt: impl FnMut() -> bool,
    why: impl Fn() -> String,
    prefix: Option<&str>,
    out: &mut impl Write,
) -> Result<Shown> {
    if show.tree == Tree::Never {
        return Ok(Shown::Nothing);
    }
    let stopped = || Shown::LeftOut(format!("the derivation is not written: {}", why()));
    if let Some(d) = read {
        return crate::show::shown(shown(d, show), show.fit(), show, halt, why, prefix, out);
    }
    let d = match image.read_back(proof, &show.limits, |_| halt()) {
        Ok(d) => d,
        // The stop, whether the unfolding or the checker's pass in it
        // noticed it first.
        Err(error) if error.kind() == ErrorKind::Stopped => return Ok(stopped()),
        Err(error) if error.is_refusal() => {
            let sides = if image.mode().is_intuitionistic() {
                Sides::Two
            } else {
                Sides::One
            };
            let view = ViewOptions::default().with_sides(sides);
            let size = || {
                proof
                    .derivation_size(&view, &Limits::default(), |_| false)
                    .ok()
            };
            return Ok(Shown::LeftOut(not_built(&error, size)));
        }
        Err(error @ Error::ReadBack { .. }) => return Err(error.into()),
        Err(error) => return Err(anyhow!(error).context("the proof cannot be unfolded")),
    };
    match d.check(&show.limits, |_| halt()) {
        Err(Error::Refused(Refusal::Stopped { .. })) => return Ok(stopped()),
        Err(error @ Error::InvalidDerivation { .. }) => return Err(read_back_fault(error)),
        checked => checked?,
    }
    crate::show::shown(shown(d, show), show.fit(), show, halt, why, prefix, out)
}

/// Returns a checked derivation of LK or LJ with its structural rules as
/// the view shows them, every one explicit in a certificate, which names
/// each formula it weakens.
pub(crate) fn shown(d: Derivation, show: &Show) -> Derivation {
    match (show.format, show.view.structural) {
        (Format::Rocq, _) | (_, Structural::Explicit) => d,
        (_, structural) => d.shown(structural).into_owned(),
    }
}

/// Returns the lambda-term of the LJ derivation read back from the proof
/// of an image, within the show's limits, or why it has none (a
/// derivation of LK).
pub(crate) fn term(
    image: &Image,
    proof: &deduxit::linear::Proof,
    show: &Show,
    halt: &dyn Fn() -> bool,
) -> Result<deduxit::lambda::Term, deduxit::Error> {
    let derivation = image.read_back(proof, &show.limits, |_| halt())?;
    derivation.term(&show.limits, |_| halt())
}

#[cfg(test)]
mod tests {
    use super::derivation;
    use crate::argument_parsing::{Cli, Command};
    use crate::settings::Given;
    use crate::show::{Show, Shown};
    use clap::Parser;
    use deduxit::linear::search::{Options, Verdict};
    use deduxit::ordinary::{self, Logic, Translation};

    /// A read-back the stop ends, in the unfolding or in the checker's
    /// pass inside it, is left out with the stop's reason.
    #[test]
    fn a_stopped_read_back_says_why() {
        let line = ["deduxit", "check", "--format", "latex", "--output", "x.tex"];
        let Command::Check(check) = Cli::try_parse_from(line).unwrap().command else {
            unreachable!("a check command")
        };
        let settings = check.settings(&Given::default()).unwrap();
        let show = Show::new(&check.output, &settings).unwrap();
        // Past the nodes the checker passes between two polls of its stop.
        let atoms: Vec<String> = (0..300).map(|i| format!("a{i}")).collect();
        let text = format!("{} |- {}", atoms.join(", "), atoms.join(" /\\ "));
        let sequent: ordinary::Sequent = text.parse().unwrap();
        let image = ordinary::translate(
            &sequent,
            Logic::Classical,
            Translation::Affine,
            &deduxit::Limits::default(),
        )
        .unwrap();
        let outcome =
            deduxit::linear::prove(image.sequent(), image.mode(), &Options::default()).unwrap();
        let Verdict::Proved(proof) = outcome.verdict else {
            panic!("the image is provable")
        };
        let mut out = String::new();
        let shown = derivation(
            &image,
            &proof,
            None,
            &show,
            || true,
            || "the time limit of 1s was reached".to_owned(),
            None,
            &mut out,
        )
        .unwrap();
        let Shown::LeftOut(line) = shown else {
            panic!("the derivation is left out")
        };
        assert_eq!(
            line,
            "the derivation is not written: the time limit of 1s was reached"
        );
    }

    /// A sequent valid by a clausal proof says so on its verdict line, by
    /// the SAT solver, and has no derivation to draw: the refusal names
    /// the method that gives one.
    #[test]
    fn a_clausal_proof_is_named_on_the_verdict_line() {
        let sequent: ordinary::Sequent = "|- ((a -> b) -> a) -> a".parse().unwrap();
        let outcome = ordinary::sat::decide(
            &sequent,
            Logic::Classical,
            &deduxit::Limits::default(),
            |_| false,
        )
        .unwrap();
        let outcome = ordinary::Outcome::from(outcome);
        let ended = super::Ended {
            stop: None,
            elapsed: std::time::Duration::ZERO,
            after_search: false,
        };
        let line = super::native_verdict_line(&outcome, &ended);
        assert!(
            line.starts_with(
                "valid (classical logic by the SAT solver): a checked clausal proof of "
            ),
            "{line}"
        );
        let ordinary::Verdict::Valid(evidence) = &outcome.verdict else {
            panic!("valid")
        };
        let refused = evidence.derivation().unwrap_err().to_string();
        assert!(refused.contains("the method embedding"), "{refused}");
    }
}
