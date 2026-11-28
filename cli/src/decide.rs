// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The decision of one sequent, which `prove` and a batch's entry share:
//! ordinary logic by its own method or through the embedding, a linear
//! sequent by the search. The callers read the sequent and write the
//! answer; what differs between them is a [`Run`](crate::decide::Run).

use crate::limit::{Deadline, Notice};
use crate::ordinary::{self, Embedded, GaveWay, Native};
use crate::prove::{
    Ended, NOTICE_AFTER, Stop, describe, halting, notice_line, searched, verdict_line,
};
use crate::settings::Threads;
use crate::{Status, catch_interrupt, interrupted};
use anyhow::Result;
use deduxit::Limits;
use deduxit::linear::search::{Goal, Options, Outcome, prove_goal};
use deduxit::linear::{Forest, Mode, Written};
use deduxit::ordinary::{self as ordinary_logic, Image, Method};
use std::time::{Duration, Instant};

/// A sequent read, ready to be decided.
pub(crate) enum Read {
    /// A sequent of ordinary logic, decided as `options` say.
    Ordinary(ordinary_logic::Options, ordinary_logic::Sequent),
    /// A linear sequent's forest, how its formulas were written, and the
    /// mode it is searched in.
    Linear(Box<Forest>, Option<Box<[Written]>>, Mode),
}

/// What a caller of [`decide`] is: how the decision runs and stops, and
/// what it asks of the caller.
pub(crate) struct Run<'a> {
    /// The search settings.
    pub(crate) search: &'a Options,
    /// The bounds.
    pub(crate) limits: Limits,
    /// The threads of the linear search, or `None` for the calling thread
    /// alone with the settings' jobs (a batch's entry on one thread).
    pub(crate) threads: Option<Threads>,
    /// The time limit of this sequent.
    pub(crate) deadline: &'a Deadline,
    /// Whether a limit beyond the deadline passed: a batch's own.
    pub(crate) beyond: &'a (dyn Fn() -> bool + Sync),
    /// The time limit the search reports: the sequent's, or the batch's.
    pub(crate) limit: Option<Duration>,
    /// Why the stop fired, if it did.
    pub(crate) stopped: &'a (dyn Fn() -> Option<Stop> + Sync),
    /// Whether `--linear` answers for the image of an ordinary sequent.
    pub(crate) linear: bool,
    /// Whether `--net` asks for a proof net.
    pub(crate) net: bool,
    /// Whether this is a call of its own, `prove`, and not a batch's
    /// entry, which answers as `prove --deterministic` does. A call of its
    /// own says a wait on a terminal, catches Ctrl-C, translates within
    /// the time limit, and advises the flag a refusal's bound needs.
    pub(crate) own: bool,
    /// Asked before each step whose answer goes to the output, so that an
    /// output that cannot be made is found first: `prove` checks it.
    pub(crate) ready: &'a (dyn Fn(Before) -> Result<()> + Sync),
}

/// The step [`Run::ready`] is asked before.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Before {
    /// Ordinary logic's own method, or the library's default of a
    /// first-order sequent.
    Native,
    /// The linear search.
    Search,
}

impl Run<'_> {
    /// Whether to stop: Ctrl-C, the time limit or a limit beyond it.
    pub(crate) fn halted(&self) -> bool {
        interrupted() || self.deadline.passed() || (self.beyond)()
    }

    /// Returns how a step that began at `start` ended, now.
    pub(crate) fn ended(&self, start: Instant, after_search: bool) -> Ended {
        Ended {
            stop: (self.stopped)(),
            elapsed: start.elapsed(),
            after_search,
        }
    }

    /// Runs `f`, for a call of its own on a thread with the stack the
    /// limits size for the recursion depth, and returns its result.
    ///
    /// # Errors
    ///
    /// A thread that cannot be started.
    pub(crate) fn stacked<T: Send>(&self, f: impl FnOnce() -> T + Send) -> Result<T> {
        if self.own {
            crate::prove::on_large_stack(self.limits.stack_bytes(), f)
        } else {
            Ok(f())
        }
    }

    /// Starts the line a wait on a terminal is said by, for a call of its
    /// own.
    pub(crate) fn notice(&self, deepens: bool) -> Option<Notice> {
        self.own
            .then(|| Notice::start(NOTICE_AFTER, notice_line(self.deadline.limit(), deepens)))
    }
}

/// The answer of one sequent, which the caller writes.
pub(crate) enum Answer {
    /// An outcome of ordinary logic that the command's own linear search
    /// did not give: its own method's, the library's default of a
    /// first-order sequent, or the classical default's where the embedding
    /// gave no evidence.
    Native {
        /// The outcome.
        outcome: Box<ordinary_logic::Outcome>,
        /// When its search began.
        start: Instant,
        /// Whether the search had decided and its read-back or a check had
        /// begun, where a stop ended it.
        after_search: bool,
        /// Whether the step that found it ran on the large stack
        /// ([`Run::stacked`]), so that it is written there too.
        on_stack: bool,
    },
    /// An unknown verdict with no outcome, said by its line: the table or
    /// the tableau stopped.
    Stopped(String),
    /// The time limit passed while the embedding translated the sequent,
    /// said by its line (a call of its own).
    Untranslated(String),
    /// The linear search's outcome, on the sequent or on the image of an
    /// ordinary one.
    Searched(Box<Searched>),
}

/// The linear search's answer, with what its writer needs.
pub(crate) struct Searched {
    /// The outcome.
    pub(crate) outcome: Outcome,
    /// The image the search ran on, for an ordinary sequent.
    pub(crate) image: Option<Image>,
    /// How the linear sequent's formulas were written.
    pub(crate) writing: Option<Box<[Written]>>,
    /// The mode searched in.
    pub(crate) mode: Mode,
    /// The derivation the embedding's proof was read back to, where the
    /// classical default asked for its evidence.
    pub(crate) read: Option<ordinary_logic::Derivation>,
    /// The SAT solver's outcome, where the classical default decided first.
    pub(crate) solved: Option<Box<ordinary_logic::Outcome>>,
    /// Why the logic's default gave way to the embedding.
    pub(crate) gave_way: GaveWay,
    /// The exit status.
    pub(crate) status: Status,
    /// How the search ended.
    pub(crate) ended: Ended,
    /// The verdict line.
    pub(crate) line: String,
}

/// Returns the [`Answer::Native`] of an outcome whose search began at
/// `start`, found on the large stack where `on_stack` says so.
fn native(outcome: Box<ordinary_logic::Outcome>, start: Instant, on_stack: bool) -> Answer {
    Answer::Native {
        outcome,
        start,
        after_search: false,
        on_stack,
    }
}

/// Decides a sequent read: ordinary logic by its own method first where
/// it has one, or a first-order sequent with no method given by the
/// library's default, then the linear search on the image or on the
/// linear sequent; returns the answer for the caller to write.
///
/// # Errors
///
/// A sequent the search refuses (an engine or a fragment that does not
/// apply), an output that cannot be made ([`Run::ready`]), a refusal that
/// is no reason of an unknown, and a defect of a read-back.
pub(crate) fn decide(read: Read, run: &Run<'_>) -> Result<Answer> {
    let (mut gave_way, mut solved) = (None, None);
    let (forest, image, writing, mode) = match read {
        Read::Linear(forest, writing, mode) => (*forest, None, writing, mode),
        Read::Ordinary(options, sequent) => {
            if options.method_for(&sequent) != Method::Embedding
                || ordinary::by_library(&options, &sequent, run)
            {
                (run.ready)(Before::Native)?;
                match run.stacked(|| ordinary::natively(&options, &sequent, run))?? {
                    Native::Decided(outcome, start, after_search) => {
                        return Ok(Answer::Native {
                            outcome,
                            start,
                            after_search,
                            on_stack: run.own,
                        });
                    }
                    Native::Stopped(line) => return Ok(Answer::Stopped(line)),
                    Native::GivesWay(why) => gave_way = why,
                    Native::Solved(outcome, start) => solved = Some((outcome, start)),
                }
            }
            // The classical default's evidence is translated within a
            // bound: an image past it is refused, and the solver's answer
            // written.
            let translating = match &solved {
                Some(_) => ordinary::bounded(&run.limits),
                None => run.limits,
            };
            let translated = if run.own {
                let within = run
                    .deadline
                    .within(move || ordinary::translated(&options, &sequent, &translating))?;
                let Some(translated) = within else {
                    let limit = run.deadline.limit().expect("only a limit passes");
                    return Ok(match solved {
                        Some((outcome, start)) => native(outcome, start, false),
                        None => Answer::Untranslated(crate::prove::untranslated(
                            limit,
                            gave_way.as_deref(),
                        )),
                    });
                };
                translated
            } else {
                ordinary::translated(&options, &sequent, &translating)
            };
            match (translated, solved.take()) {
                (Ok((forest, image)), kept) => {
                    solved = kept;
                    let mode = image.mode();
                    (forest, Some(image), None, mode)
                }
                (Err(_), Some((outcome, start))) => return Ok(native(outcome, start, false)),
                (Err(error), None) => return Err(error),
            }
        }
    };
    let sequent = forest.sequent();
    if run.net {
        crate::show::nets_exist(sequent, mode)?;
    }
    (run.ready)(Before::Search)?;
    if run.own {
        catch_interrupt();
    }
    // The search, and the evidence read back from its proof, recurse as
    // deep as the limits allow.
    run.stacked(move || -> Result<Answer> {
        let sequent = forest.sequent();
        let options = run.search;
        let limits = run.limits;
        let start = solved
            .as_ref()
            .map_or_else(Instant::now, |(_, start)| *start);
        let deepens = options.copies.is_none() && sequent.fragment().has_exponentials();
        let notice = run.notice(deepens);
        // Every poll asks each condition, all flags.
        let passed = || run.deadline.passed() || (run.beyond)();
        // The classical default's evidence is searched for within a bound.
        let within = match &solved {
            Some(_) => ordinary::bounded(&limits),
            None => limits,
        };
        let goal = Goal::conclusion(&forest);
        let outcome = match run.threads {
            Some(threads) => searched(goal, mode, options, &within, threads, (run.limit, &passed)),
            None => prove_goal(goal, mode, options, &within, halting(run.limit, &passed)),
        };
        drop(notice);
        match (outcome, solved) {
            (Ok(outcome), solved) => {
                finish(outcome, image, writing, mode, solved, gave_way, start, run)
            }
            (Err(_), Some((solved, start))) => Ok(native(solved, start, run.own)),
            (Err(error), None) => Err(describe(error, sequent)),
        }
    })?
}

/// Takes the evidence the classical default asked for from the linear
/// search's `outcome`, and returns the answer: the solver's outcome where
/// the embedding gave none, else the search's with its status and line.
///
/// # Errors
///
/// A read-back the checker refuses, a defect.
#[expect(clippy::too_many_arguments, reason = "the parts of one answer")]
fn finish(
    outcome: Outcome,
    image: Option<Image>,
    writing: Option<Box<[Written]>>,
    mode: Mode,
    solved: Option<(Box<ordinary_logic::Outcome>, Instant)>,
    gave_way: GaveWay,
    start: Instant,
    run: &Run<'_>,
) -> Result<Answer> {
    // A derivation read back to decide whether the embedding gives the
    // evidence, drawn as it is.
    let mut read = None;
    if let (Some((solved, begun)), Some(image)) = (&solved, &image) {
        match ordinary::evidence(&outcome, image, solved, &run.limits, || run.halted())? {
            Embedded::Derivation(derivation) => read = Some(*derivation),
            Embedded::Refutation => {}
            Embedded::Nothing => return Ok(native(solved.clone(), *begun, run.own)),
        }
    }
    let ended = run.ended(start, false);
    // Under --logic, an unprovable image of a first-order sequent is
    // unknown; --linear answers for the image.
    let status = if run.linear {
        Status::from(&outcome.verdict)
    } else {
        ordinary::status(&outcome, image.as_ref())
    };
    let line = match &image {
        Some(image) => ordinary::verdict_line(
            &outcome,
            image.logic(),
            image.translation(),
            ordinary::incomplete(&outcome, image),
            ordinary::image_axioms(image),
            &ended,
        ),
        None => verdict_line(&outcome, run.search.fragment.is_some(), &ended),
    };
    Ok(Answer::Searched(Box::new(Searched {
        outcome,
        image,
        writing,
        mode,
        read,
        solved: solved.map(|(solved, _)| solved),
        gave_way,
        status,
        ended,
        line,
    })))
}
