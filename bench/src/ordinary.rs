// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The child's run of a problem of ordinary logic in one logic: through
//! the embedding, whose image is searched as a linear problem is and
//! whose proof is read back and checked as the check, or by LJT, which
//! checks its own derivation inside the time measured.

use crate::OneArgs;
use crate::problems;
use crate::run::{
    Clock, EngineChoice, LOADED, Searched, clean, forward_copies, memo_column, memory_column,
    memory_limit, name, pool_column, row, say,
};
use deduxit::linear::lltp::Status;
use deduxit::linear::search::Reason;
use deduxit::ordinary::{self, Logic, Method, Translation, Verdict, ljt};
use deduxit::{Error, Limits, Refusal};
use std::sync::atomic::Ordering;

/// Runs a TPTP problem in `logic` by the method the engine asked for
/// names (a linear engine's name the embedding, `auto` the logic's
/// default) and returns the tail of its row.
pub fn tail(args: &OneArgs, logic: Logic) -> String {
    let problem = match problems::load_ordinary(&args.problem) {
        Ok(problem) => problem,
        Err(error) => {
            return row(&[
                ("verdict", "error"),
                ("reason", &clean(&format!("{error:#}"))),
            ]);
        }
    };
    let expected = expected(problem.status, logic);
    let method = match args.engine {
        EngineChoice::Ordinary(method) => method,
        EngineChoice::Linear(_) => Method::Embedding,
        EngineChoice::Auto => ordinary::Options::default().with_logic(logic).method(),
    };
    if !method.decides(logic) {
        return row(&[
            ("verdict", "refused"),
            ("reason", &format!("{method} does not decide {logic} logic")),
        ]);
    }
    match method {
        Method::Embedding => embedded(args, logic, &problem.sequent, expected),
        Method::Ljt => by_ljt(args, logic, &problem.sequent, expected),
        other => row(&[
            ("verdict", "error"),
            ("reason", &format!("the harness does not run {other}")),
        ]),
    }
}

/// The `expected` column of a problem in `logic`. ILTP's statuses are
/// intuitionistic: a theorem is a classical one too, and a non-theorem a
/// minimal one.
fn expected(status: Option<Status>, logic: Logic) -> &'static str {
    match (status, logic) {
        (Some(Status::Theorem), Logic::Intuitionistic | Logic::Classical) => "provable",
        (Some(Status::NonTheorem), Logic::Intuitionistic | Logic::Minimal) => "unprovable",
        _ => "",
    }
}

/// Decides the sequent through its image under the logic's default
/// translation, searched as a linear problem is, and checks a proof by
/// reading the derivation of LK or LJ back and checking it.
fn embedded(args: &OneArgs, logic: Logic, sequent: &ordinary::Sequent, expected: &str) -> String {
    let translation = Translation::default_for(logic);
    let image = match ordinary::translate(sequent, logic, translation, &Limits::default()) {
        Ok(image) => image,
        Err(error) => {
            return row(&[
                ("expected", expected),
                ("verdict", "error"),
                ("reason", &clean(&error.to_string())),
            ]);
        }
    };
    let searched = Searched {
        sequent: image.sequent(),
        mode: image.mode(),
        expected,
        copies: None,
        witness_depth: None,
    };
    searched.run(args, |proof, limits| {
        match image
            .read_back(proof, limits, |_| false)
            .and_then(|derivation| derivation.check(limits, |_| false))
        {
            Ok(()) => "ok".to_owned(),
            Err(error) if error.is_refusal() => clean(&format!("unchecked: {error}")),
            Err(error) => clean(&format!("failed: {error}")),
        }
    })
}

/// Decides the sequent by LJT within the run's memory bound and time
/// limit. Its derivation is checked inside the time measured; a proof
/// whose derivation a bound refuses is `unknown`, with the bound as the
/// reason (`derivation_limit` for its size).
fn by_ljt(args: &OneArgs, logic: Logic, sequent: &ordinary::Sequent, expected: &str) -> String {
    let limits = Limits::default().with_memory_bytes(memory_limit(args.memory_limit));
    let (bias, forward) = (
        name(&args.bias),
        forward_copies(args.forward_copies).to_string(),
    );
    let (memory, pool, memo) = (
        memory_column(args.memory_limit),
        pool_column(args.pool_after),
        memo_column(args.memo_limit),
    );
    say(LOADED);
    let clock = Clock::start(args.timeout);
    let outcome = ljt::decide(sequent, logic, &limits, |_| {
        clock.expired.load(Ordering::Relaxed)
    });
    let (time, cpu, wait) = clock.read();
    let time = format!("{time:.3}");
    let mut fields = vec![
        ("expected", expected),
        ("engine", Method::Ljt.name()),
        ("fragment", "LJT"),
        ("time_ms", &time),
        ("cpu_ms", &cpu),
        ("wait_ms", &wait),
        ("bias", &bias),
        ("forward_copies", &forward),
        ("memory_limit", &memory),
        ("pool_after", &pool),
        ("memo_limit", &memo),
    ];
    let unknown = |reason: Reason| match reason {
        Reason::Stopped => "timeout",
        reason => reason.name(),
    };
    let message;
    let (sequents, hits, entries);
    match &outcome {
        Ok(outcome) => {
            let s = outcome.statistics;
            (sequents, hits, entries) = (
                s.sequents.to_string(),
                s.memo_hits.to_string(),
                s.memo_entries.to_string(),
            );
            fields.extend([
                ("nodes", sequents.as_str()),
                ("memo_hits", hits.as_str()),
                ("memo_entries", entries.as_str()),
                ("work", sequents.as_str()),
            ]);
            fields.extend(match &outcome.verdict {
                // Checked inside the search's time.
                Verdict::Valid(_) => [("verdict", "proved"), ("checked", "ok")],
                Verdict::NotValid => [("verdict", "unprovable"), ("checked", "")],
                Verdict::Unknown(reason) => [("verdict", "unknown"), ("reason", unknown(*reason))],
                _ => [
                    ("verdict", "error"),
                    ("reason", "a verdict the harness does not know"),
                ],
            });
        }
        // A proof found, but a bound refused its derivation: no checked
        // verdict.
        Err(Error::Refused(refusal)) => {
            let reason = match refusal {
                Refusal::Stopped { .. } => "timeout",
                Refusal::Output { .. } => "derivation_limit",
                Refusal::Memory { .. } => "memory_limit",
                Refusal::Work { .. } => "work_limit",
                _ => {
                    message = clean(&refusal.to_string());
                    &message
                }
            };
            fields.extend([("verdict", "unknown"), ("reason", reason)]);
        }
        Err(error) => {
            message = clean(&error.to_string());
            fields.extend([("verdict", "error"), ("reason", message.as_str())]);
        }
    }
    row(&fields)
}
