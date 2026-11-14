// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Running problems. The parent (`run`) starts this program again as a
//! child (`one`) for every run. So a crash, a stack overflow or a run that
//! ignores its time limit costs that run only.
//!
//! The child times the search alone, not its start or the parsing. It
//! prints the tail of the run's CSV row: once with the search's verdict,
//! before it checks a proof, and once more with the check's result.
//!
//! The parent kills a child that takes too long to load its problem, or
//! that outlives its time limit by more than the grace period, counted
//! from the end of the load. A child that dies in its check leaves the row
//! its verdict.

use crate::growth::{ENGINE_SERIES, Ended, SKIPPED};
use crate::problems::{self, Reference, mode_name};
use crate::{OneArgs, RunArgs};
use anyhow::{Context, Result, anyhow};
use clap::ValueEnum;
use deduxit::linear::search::{Engine, Options, Reason, Verdict, prove_within};
use deduxit::ordinary::{Logic, Method};
use deduxit::{
    Error, Limits,
    linear::{Atom, Bias, Forest, Mode, Sign},
};
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

/// The columns the parent writes, then those the child prints. Older
/// files have a column `portfolio` after `jobs`, which the summaries
/// read.
pub const HEADER: &str = "source,family,size,index,problem,mode,engine_requested,jobs,\
                          test_period,timeout_s,run,copies,expected,verdict,reason,checked,engine,\
                          fragment,occurrences,multiplicity,time_ms,nodes,memo_hits,memo_entries,\
                          splits,links,tests,recursion_limit,cpu_ms,wait_ms,bias,forward_copies,\
                          check_ms,memory_limit,copies_reached,pool_after,forward_copies_reached,work,\
                          tool,tool_version,translate_ms,peak_bytes,raw,unifications,bindings,\
                          memo_limit";

/// The number of columns the child prints, the last of [`HEADER`].
const TAIL: usize = 35;

/// The line the child prints when its problem is loaded and its search
/// starts, from which the parent counts the time limit.
pub(crate) const LOADED: &str = "loaded";

/// Returns the index in the tail of the column `name`.
fn column(name: &str) -> usize {
    let columns: Vec<&str> = HEADER.split(',').collect();
    columns[columns.len() - TAIL..]
        .iter()
        .position(|c| *c == name)
        .expect("a column of the tail")
}

/// Which mode to run a problem in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum ModeChoice {
    /// The problem's own
    Given,
    /// The problem's own made classical
    Classical,
    /// The problem's own made intuitionistic (without Mix)
    Intuitionistic,
}

impl ModeChoice {
    /// Returns the mode to run a problem meant for `given` in.
    fn apply(self, given: Mode) -> Mode {
        match self {
            Self::Given => given,
            Self::Classical => {
                let mut mode = Mode::CLASSICAL;
                if given.is_affine() {
                    mode = mode.with_affine();
                }
                if given.has_mix() {
                    mode = mode.with_mix();
                }
                mode
            }
            Self::Intuitionistic if given.is_affine() => Mode::INTUITIONISTIC.with_affine(),
            Self::Intuitionistic => Mode::INTUITIONISTIC,
        }
    }
}

/// How the focused engines pick the positive literal of every atom.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum BiasChoice {
    /// The rarer literal in affine mode, by tensor factors in linear mode
    /// without exponentials, and in linear mode with exponentials both,
    /// each in a search of its own
    Auto,
    /// The rarer literal
    Rarer,
    /// By tensor factors
    Factors,
}

impl BiasChoice {
    /// Returns the rule of the library.
    fn bias(self) -> Bias {
        match self {
            Self::Auto => Bias::Auto,
            Self::Rarer => Bias::Rarer,
            Self::Factors => Bias::Factors,
        }
    }
}

/// One of N shards of the problems. The K-th takes the problems at the
/// places K − 1, K − 1 + N, K − 1 + 2N, … of the list, counted from 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shard {
    /// Which shard, from 1.
    k: usize,
    /// How many.
    n: usize,
}

impl std::str::FromStr for Shard {
    type Err = String;

    /// Reads `K/N` with 1 ≤ K ≤ N.
    fn from_str(text: &str) -> Result<Self, String> {
        let (k, n) = text
            .split_once('/')
            .and_then(|(k, n)| Some((k.parse().ok()?, n.parse().ok()?)))
            .ok_or_else(|| format!("{text:?} is not a shard K/N"))?;
        if k == 0 || k > n {
            return Err(format!("shard {text}: K must be 1 to N"));
        }
        Ok(Self { k, n })
    }
}

impl Shard {
    /// Returns this shard's problems of the list, in their order.
    pub fn of<T>(self, list: Vec<T>) -> Vec<T> {
        list.into_iter().skip(self.k - 1).step_by(self.n).collect()
    }

    /// Returns this shard's groups of the list, in their order: every
    /// N-th group from the K-th, each with all its problems. The groups
    /// are numbered in the order of their first problems.
    pub fn of_groups<T, K: Eq + std::hash::Hash>(
        self,
        list: Vec<T>,
        group: impl Fn(&T) -> K,
    ) -> Vec<T> {
        let mut numbers = std::collections::HashMap::new();
        list.into_iter()
            .filter(|item| {
                let next = numbers.len();
                let number = *numbers.entry(group(item)).or_insert(next);
                number % self.n == self.k - 1
            })
            .collect()
    }
}

/// Returns the memory bound of a run in bytes: the one asked for, none
/// for 0, or the library's default.
pub(crate) fn memory_limit(asked: Option<u64>) -> Option<u64> {
    match asked {
        None => Some(Limits::DEFAULT_MEMORY_BYTES),
        Some(0) => None,
        bound => bound,
    }
}

/// Returns the `memory_limit` column of a run: the bytes, or 0 for no
/// bound.
pub(crate) fn memory_column(asked: Option<u64>) -> String {
    memory_limit(asked).unwrap_or(0).to_string()
}

/// A copy bound as `--copies` reads it: a number, or `none` for a search
/// that deepens until it decides or its time limit passes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bound(pub Option<u32>);

impl std::str::FromStr for Bound {
    type Err = String;

    /// Reads a number, or `none`.
    fn from_str(text: &str) -> Result<Self, String> {
        match text {
            "none" => Ok(Self(None)),
            n => n
                .parse()
                .map(|n| Self(Some(n)))
                .map_err(|_| format!("{n:?} is not a number of copies, or `none`")),
        }
    }
}

impl std::fmt::Display for Bound {
    /// Writes the bound as `--copies` reads it and its column has it.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(n) => write!(f, "{n}"),
            None => f.write_str("none"),
        }
    }
}

/// Returns the `memo_limit` column of a run: the most entries the
/// search's memo may hold, as asked for or the library's default. 0 is no
/// memo.
pub(crate) fn memo_column(asked: Option<u32>) -> String {
    asked.unwrap_or(Options::DEFAULT_MEMO_LIMIT).to_string()
}

/// Returns the `pool_after` column of a run: the seconds one thread
/// searched alone before the pool, or empty for every thread from the
/// start.
pub(crate) fn pool_column(asked: Option<f64>) -> String {
    asked.map_or(String::new(), |t| t.to_string())
}

/// Returns the forward search's copy bound of a run: the one asked for,
/// or the library's default.
pub(crate) fn forward_copies(asked: Option<u32>) -> u32 {
    asked.unwrap_or(Options::DEFAULT_FORWARD_COPIES)
}

/// Which engine to run: the one the library picks for the fragment and
/// the mode (on a problem of ordinary logic, the logic's default method),
/// a linear engine by name, or a method of ordinary logic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineChoice {
    /// The automatic choice.
    Auto,
    /// A linear engine, which on a problem of ordinary logic searches the
    /// image.
    Linear(Engine),
    /// A method of ordinary logic, which a linear problem refuses.
    Ordinary(Method),
}

impl EngineChoice {
    /// Reads an engine's or a method's name, as the library spells it, or
    /// `auto`.
    pub fn parser() -> impl clap::builder::TypedValueParser<Value = Self> {
        use clap::builder::TypedValueParser as _;
        clap::builder::PossibleValuesParser::new(
            std::iter::once("auto")
                .chain(Engine::NAMES.iter().copied())
                .chain(Method::NAMES.iter().copied()),
        )
        .map(|name| match (name.parse(), name.parse()) {
            (Ok(engine), _) => Self::Linear(engine),
            (_, Ok(method)) => Self::Ordinary(method),
            _ => Self::Auto,
        })
    }

    /// Returns the name, `auto` for the automatic choice.
    pub fn name(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Linear(engine) => engine.name(),
            Self::Ordinary(method) => method.name(),
        }
    }

    /// Returns the linear engine forced, `None` for the automatic choice.
    pub const fn engine(self) -> Option<Engine> {
        match self {
            Self::Linear(engine) => Some(engine),
            Self::Auto | Self::Ordinary(_) => None,
        }
    }
}

/// What a problem runs in: a mode, or for ordinary logic a logic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Setting {
    /// The mode asked for, and the mode it gives the problem.
    Mode(ModeChoice, Mode),
    /// A logic.
    Logic(Logic),
}

impl Setting {
    /// Returns the name the `mode` column has for it.
    fn name(self) -> &'static str {
        match self {
            Self::Mode(_, mode) => mode_name(mode),
            Self::Logic(logic) => logic.name(),
        }
    }
}

/// Returns the name of a value as its argument spells it.
pub(crate) fn name(value: &impl ValueEnum) -> String {
    value
        .to_possible_value()
        .map_or_else(String::new, |v| v.get_name().to_owned())
}

/// Runs every problem in every configuration asked for and writes the rows.
pub fn run(args: &RunArgs) -> Result<()> {
    let mut references = problems::families(&args.family, args.all_families, args.growing)?;
    references.extend(problems::lltp(&args.lltp)?);
    references.extend(problems::specs(&args.spec)?);
    references.extend(problems::files(&args.problems)?);
    references.extend(problems::tptp(&args.tptp)?);
    if !args.only.is_empty() {
        references.retain(|r| {
            let path = format!("{}/{}", r.family, r.name);
            args.only.iter().any(|only| path.contains(only.as_str()))
        });
    }
    if let Some(shard) = args.shard {
        // A growing set's series stays in one shard, so the shard that runs
        // its end skips its larger sizes.
        references = if args.growing {
            shard.of_groups(references, |r| {
                (
                    r.family.clone(),
                    r.index,
                    (r.source != "family").then(|| r.name.clone()),
                )
            })
        } else {
            shard.of(references)
        };
    }
    if !args.tools.is_empty() {
        return crate::tools::main(args, references);
    }
    let jobs = args
        .jobs
        .iter()
        .map(|j| match j.as_str() {
            "all" => Ok(thread::available_parallelism().map_or(1, |n| n.get())),
            n => n.parse().with_context(|| format!("jobs `{n}`")),
        })
        .collect::<Result<Vec<usize>>>()?;
    // A search uses no more threads than the machine runs at once, so a
    // row with a larger count would name threads that never ran.
    if let Ok(machine) = thread::available_parallelism()
        && let Some(n) = jobs.iter().find(|&&n| n > machine.get())
    {
        anyhow::bail!(
            "--jobs {n}: this process may run {machine} threads at once, and a search uses no more"
        );
    }

    let done_before = match &args.output {
        Some(path) if args.resume => finished(path)?,
        _ => HashSet::new(),
    };
    let mut ended = match &args.output {
        Some(path) if args.resume && args.growing => Ended::read(path, &ENGINE_SERIES)?,
        _ => Ended::default(),
    };
    let mut out: Box<dyn Write> = match &args.output {
        Some(path) => {
            let file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(args.append)
                .truncate(!args.append)
                .open(path)
                .with_context(|| format!("opening {}", path.display()))?;
            let empty = file.metadata()?.len() == 0;
            let mut file = Box::new(std::io::BufWriter::new(file));
            if empty {
                writeln!(file, "{HEADER}")?;
            }
            file
        }
        None => {
            println!("{HEADER}");
            Box::new(std::io::stdout())
        }
    };

    let exe = std::env::current_exe()?;
    if args.reverse {
        references.reverse();
    }
    let settings = |reference: &Reference| -> Vec<Setting> {
        if reference.is_ordinary() {
            return args
                .logics
                .iter()
                .map(|&logic| Setting::Logic(logic))
                .collect();
        }
        let mut settings: Vec<Setting> = Vec::new();
        for &choice in &args.modes {
            let mode = choice.apply(reference.mode);
            if !settings
                .iter()
                .any(|s| matches!(s, Setting::Mode(_, m) if *m == mode))
            {
                settings.push(Setting::Mode(choice, mode));
            }
        }
        settings
    };
    let total = references.iter().map(|r| settings(r).len()).sum::<usize>()
        * args.engines.len()
        * jobs.len();
    let mut done = 0;
    // The configurations this invocation ran, for the estimate of the time
    // left. Those a resumed run skips cost nothing, so they do not count.
    let (start, mut ran) = (Instant::now(), 0);
    for reference in &references {
        for setting in settings(reference) {
            for &engine in &args.engines {
                for &threads in &jobs {
                    done += 1;
                    let key = [
                        reference.source,
                        &reference.family,
                        &reference.name,
                        setting.name(),
                        engine.name(),
                        &threads.to_string(),
                        &args.test_period.map_or(String::new(), |p| p.to_string()),
                        &name(&args.bias),
                        &forward_copies(args.forward_copies).to_string(),
                        &memory_column(args.memory_limit),
                        &pool_column(args.pool_after),
                        &memo_column(args.memo_limit),
                    ]
                    .join(",");
                    let index = reference.index.map_or(String::new(), |i| i.to_string());
                    let threads_text = threads.to_string();
                    let series = [
                        reference.family.as_str(),
                        &index,
                        setting.name(),
                        engine.name(),
                        &threads_text,
                    ];
                    let growing = args.growing && reference.source == "family";
                    if done_before.contains(&key) {
                        continue;
                    }
                    if let Some(reason) = ended.skipped(&series).filter(|_| growing) {
                        // The configuration's columns, as a run's row has them.
                        let tail = row(&[
                            (
                                "copies",
                                &args.copies.map_or(String::new(), |c| c.to_string()),
                            ),
                            ("verdict", SKIPPED),
                            ("reason", &reason),
                            ("bias", &name(&args.bias)),
                            (
                                "forward_copies",
                                &forward_copies(args.forward_copies).to_string(),
                            ),
                            ("memory_limit", &memory_column(args.memory_limit)),
                            ("pool_after", &pool_column(args.pool_after)),
                            ("memo_limit", &memo_column(args.memo_limit)),
                        ]);
                        writeln!(
                            out,
                            "{},{},{},{index},{},{},{},{threads},{},{},0,{tail}",
                            reference.source,
                            reference.family,
                            reference.size.map_or(String::new(), |s| s.to_string()),
                            reference.name,
                            setting.name(),
                            engine.name().to_owned(),
                            args.test_period.map_or(String::new(), |p| p.to_string()),
                            args.timeout,
                        )?;
                        continue;
                    }
                    ran += 1;
                    for run in 0..args.repeat {
                        let tail = child(&exe, reference, setting, engine, threads, args)?;
                        let fields: Vec<&str> = tail.split(',').collect();
                        let (verdict, time) = (
                            fields[column("verdict")],
                            fields[column("time_ms")].parse().unwrap_or(0.0),
                        );
                        writeln!(
                            out,
                            "{},{},{},{},{},{},{},{threads},{},{},{run},{tail}",
                            reference.source,
                            reference.family,
                            reference.size.map_or(String::new(), |s| s.to_string()),
                            reference.index.map_or(String::new(), |i| i.to_string()),
                            reference.name,
                            setting.name(),
                            engine.name().to_owned(),
                            args.test_period.map_or(String::new(), |p| p.to_string()),
                            args.timeout,
                        )?;
                        out.flush()?;
                        if run == 0 && growing {
                            ended.record(
                                &series,
                                &reference.size.map_or(String::new(), |s| s.to_string()),
                                verdict,
                                fields[column("reason")],
                                time,
                                args.timeout,
                            );
                        }
                        if run == 0 {
                            // Minutes left: the configurations left, at the
                            // mean time of those run so far.
                            let left = start.elapsed().as_secs_f64() / f64::from(ran)
                                * (total - done) as f64
                                / 60.0;
                            eprintln!(
                                "[{done}/{total}] {} {} {} j{threads}: {verdict} {} {time:.3} ms, \
                                 about {left:.0} min left",
                                reference.name,
                                setting.name(),
                                engine.name().to_owned(),
                                fields[column("reason")],
                            );
                        }
                        if time >= args.repeat_under * 1000.0 || verdict == "refused" {
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Returns the problems and configurations a CSV file has a row for, none
/// for a file that does not exist. Each is a key of the row's source,
/// family, problem, mode, requested engine, jobs, test period, bias,
/// forward copy bound, memory limit, pool delay and memo limit, joined by
/// commas, as `run` builds it.
///
/// An empty bias reads as `auto`, an empty memory limit as no bound (`0`),
/// and an empty memo limit as the library's default. A file with other
/// columns than [`HEADER`] is refused, since the rows this run appends
/// would not fit it.
fn finished(path: &Path) -> Result<HashSet<String>> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Ok(HashSet::new());
    };
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    if header != HEADER {
        anyhow::bail!(
            "{} has other columns than this version writes; resume into a new file",
            path.display()
        );
    }
    let columns: Vec<&str> = header.split(',').collect();
    let at = |column: &str| columns.iter().position(|c| *c == column);
    let key = [
        "source",
        "family",
        "problem",
        "mode",
        "engine_requested",
        "jobs",
        "test_period",
    ]
    .map(at);
    let Some(key) = key.into_iter().collect::<Option<Vec<usize>>>() else {
        anyhow::bail!("{} is not a CSV file of `run`", path.display());
    };
    let bias = at("bias");
    let forward = at("forward_copies");
    // An empty field reads as no bound.
    let memory = at("memory_limit");
    // An empty field is every thread from the start, as `pool_column` writes it.
    let pool = at("pool_after");
    // An empty field reads as the default memo.
    let memo = at("memo_limit");
    let default_memo = memo_column(None);
    Ok(lines
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            let field = |i: usize| fields.get(i).copied().unwrap_or("");
            let bias = bias.map(field).filter(|b| !b.is_empty()).unwrap_or("auto");
            key.iter()
                .map(|&i| field(i))
                .chain([
                    bias,
                    forward.map_or("", field),
                    memory.map(field).filter(|m| !m.is_empty()).unwrap_or("0"),
                    pool.map_or("", field),
                    memo.map(field)
                        .filter(|m| !m.is_empty())
                        .unwrap_or(&default_memo),
                ])
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect())
}

/// Runs one problem in a child process and returns the tail of its row:
/// the child's own, or one that says how it died.
fn child(
    exe: &Path,
    reference: &Reference,
    setting: Setting,
    engine: EngineChoice,
    jobs: usize,
    args: &RunArgs,
) -> Result<String> {
    let mut command = Command::new(exe);
    command.args(["one", "--problem", &reference.id]);
    match setting {
        Setting::Mode(choice, _) => command.args(["--mode", &name(&choice)]),
        Setting::Logic(logic) => command.args(["--logic", logic.name()]),
    };
    command.args(["--engine", engine.name()]).args([
        "--jobs",
        &jobs.to_string(),
        "--timeout",
        &args.timeout.to_string(),
    ]);
    if let Some(copies) = args.copies {
        command.args(["--copies", &copies.to_string()]);
    }
    command.args(["--bias", &name(&args.bias)]);
    if let Some(copies) = args.forward_copies {
        command.args(["--forward-copies", &copies.to_string()]);
    }
    if let Some(work) = args.classical_work {
        command.args(["--classical-work", &work.to_string()]);
    }
    if let Some(bytes) = args.memory_limit {
        command.args(["--memory-limit", &bytes.to_string()]);
    }
    if let Some(limit) = args.recursion_limit {
        command.args(["--recursion-limit", &limit.to_string()]);
    }
    if let Some(period) = args.test_period {
        command.args(["--test-period", &period.to_string()]);
    }
    if let Some(alone) = args.pool_after {
        command.args(["--pool-after", &alone.to_string()]);
    }
    if let Some(entries) = args.memo_limit {
        command.args(["--memo-limit", &entries.to_string()]);
    }
    let mut process = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("starting a child")?;
    // The child's rows, and the moment it said its problem was loaded.
    let (loaded_at, loaded) = mpsc::channel();
    let pipe = process.stdout.take().expect("piped");
    let stdout = thread::spawn(move || {
        let mut rows = Vec::new();
        for line in BufReader::new(pipe).lines().map_while(Result::ok) {
            if line == LOADED {
                let _ = loaded_at.send(Instant::now());
            } else {
                rows.push(line);
            }
        }
        rows
    });
    let mut pipe = process.stderr.take().expect("piped");
    let stderr = thread::spawn(move || {
        let mut text = String::new();
        let _ = pipe.read_to_string(&mut text);
        text
    });

    let spawned = Instant::now();
    let mut searching = None;
    // Time for the pool's teardown and the proof check.
    let grace = args.grace.unwrap_or(args.timeout * 0.1 + 5.0);
    // A limit and grace past what a duration holds never kill.
    let limit = Duration::try_from_secs_f64(args.timeout + grace).unwrap_or(Duration::MAX);
    let load_limit = Duration::from_secs_f64(args.load_limit);
    let status = loop {
        if searching.is_none() {
            searching = loaded.try_recv().ok();
        }
        if let Some(status) = process.try_wait()? {
            break Some(status);
        }
        let over = match searching {
            Some(since) => since.elapsed() > limit,
            None => spawned.elapsed() > load_limit,
        };
        if over {
            process.kill()?;
            process.wait()?;
            break None;
        }
        thread::sleep(Duration::from_millis(2));
    };
    let rows = stdout.join().map_err(|_| anyhow!("reading a child"))?;
    let stderr = stderr.join().map_err(|_| anyhow!("reading a child"))?;
    let row = rows.iter().rev().find(|row| row.split(',').count() == TAIL);
    // A child that died after it printed its verdict died in the check of
    // its proof. Its row keeps the verdict and says what became of the
    // check. A child that died before gets a row of the parent's.
    let died = |reason: String| match row {
        Some(row) => {
            let mut fields: Vec<&str> = row.split(',').collect();
            let checked = format!("failed: {reason}");
            fields[column("checked")] = &checked;
            fields.join(",")
        }
        None => {
            let mut fields = vec![String::new(); TAIL];
            fields[column("verdict")] = "unknown".to_owned();
            fields[column("reason")] = reason;
            fields[column("bias")] = name(&args.bias);
            fields[column("forward_copies")] = forward_copies(args.forward_copies).to_string();
            fields[column("memory_limit")] = memory_column(args.memory_limit);
            fields[column("pool_after")] = pool_column(args.pool_after);
            fields[column("memo_limit")] = memo_column(args.memo_limit);
            let since = searching.unwrap_or(spawned);
            fields[column("time_ms")] = format!("{:.3}", since.elapsed().as_secs_f64() * 1000.0);
            fields.join(",")
        }
    };
    Ok(match (status, row) {
        (Some(status), Some(row)) if status.success() => row.clone(),
        (None, _) if searching.is_none() => died("killed while loading".to_owned()),
        (None, _) => died("killed".to_owned()),
        (Some(status), _) => {
            // The row keeps the line that says why. The log gets all of
            // the error output, with a panic's place and backtrace.
            eprintln!(
                "{} crashed ({status}); its error output:\n{}",
                reference.name,
                stderr.trim_end()
            );
            died(clean(&format!("crash ({status}): {}", cause(&stderr))))
        }
    })
}

/// Returns the line of a dead child's error output that says why it died:
/// the last one that is neither empty nor a note. A note is such as the
/// hint about backtraces that follows a panic's message.
fn cause(stderr: &str) -> &str {
    stderr
        .lines()
        .rev()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("note:"))
        .unwrap_or("")
}

/// Returns the `checked` column of a disproof: `ok` where
/// `Disproof::check` accepts its refutation, `failed: …` where the
/// refutation does not hold, and empty for an exhausted search, which has
/// nothing to check.
pub(crate) fn refutation_checked(disproof: &deduxit::linear::Disproof) -> String {
    match disproof.check() {
        Ok(()) => "ok".to_owned(),
        Err(error) if error.kind() == deduxit::ErrorKind::Unsupported => String::new(),
        Err(error) => clean(&format!("failed: {error}")),
    }
}

/// Returns text as a CSV field: commas become `;` and line breaks spaces,
/// since the files are split on commas without quoting.
pub(crate) fn clean(text: &str) -> String {
    text.replace(',', ";").replace(['\n', '\r'], " ")
}

/// Runs one problem and prints the tail of its row. It runs on a thread
/// whose stack holds the search at its recursion limit and the parser on
/// a huge input.
pub fn one(args: OneArgs) -> Result<()> {
    let stack = Limits::default().stack_bytes().max(1 << 30);
    let line = thread::Builder::new()
        .stack_size(stack)
        .spawn(move || tail(&args))?
        .join()
        .map_err(|_| anyhow!("the child panicked"))?;
    say(&line);
    Ok(())
}

/// Prints a line for the parent at once.
pub(crate) fn say(line: &str) {
    let mut out = std::io::stdout().lock();
    let _ = writeln!(out, "{line}");
    let _ = out.flush();
}

/// Loads and runs one problem and returns the tail of its row. On the way
/// it says when the problem is loaded, and prints the tail of a proved
/// problem once before the proof is checked.
fn tail(args: &OneArgs) -> String {
    if let Some(logic) = args.logic {
        return crate::ordinary::tail(args, logic);
    }
    let problem = match problems::load(&args.problem) {
        Ok(problem) => problem,
        Err(error) => {
            return row(&[
                ("verdict", "error"),
                ("reason", &clean(&format!("{error:#}"))),
            ]);
        }
    };
    if let EngineChoice::Ordinary(method) = args.engine {
        return row(&[
            ("verdict", "refused"),
            ("reason", &format!("{method} decides ordinary logic")),
        ]);
    }
    let mode = args.mode.apply(problem.mode);
    // A known verdict holds in the problem's own mode only. Classical
    // linear logic proves more than intuitionistic, affine more than linear.
    let expected = match problem.expected.filter(|_| mode == problem.mode) {
        Some(true) => "provable",
        Some(false) => "unprovable",
        None => "",
    };
    let searched = Searched {
        sequent: &problem.sequent,
        mode,
        expected,
        copies: problem.copies,
        witness_depth: problem.witness_depth,
    };
    searched.run(args, |proof, limits| {
        match proof.check_within(mode, limits, |_| false) {
            Ok(()) => "ok".to_owned(),
            // The check reached the run's memory bound: no verdict on the proof.
            Err(deduxit::linear::CheckError::Refused(_)) => "unchecked: memory limit".to_owned(),
            Err(error) => clean(&format!("failed: {error}")),
        }
    })
}

/// A linear sequent to search, as a problem gives it.
pub(crate) struct Searched<'a> {
    /// The sequent.
    pub(crate) sequent: &'a deduxit::linear::Sequent,
    /// The mode to search it in.
    pub(crate) mode: Mode,
    /// The `expected` column: `provable`, `unprovable` or empty.
    pub(crate) expected: &'a str,
    /// The copy bound the problem names.
    pub(crate) copies: Option<u32>,
    /// The witness depth the problem names.
    pub(crate) witness_depth: Option<u32>,
}

impl Searched<'_> {
    /// Searches the sequent as `args` say and returns the tail of its row.
    /// On the way it says when the search starts, and prints the tail of a
    /// proved sequent once before the proof is checked. `check` checks it
    /// within the run's limits and returns the `checked` column.
    pub(crate) fn run(
        &self,
        args: &OneArgs,
        check: impl FnOnce(&deduxit::linear::Proof, &Limits) -> String,
    ) -> String {
        let Self {
            sequent,
            mode,
            expected,
            copies,
            witness_depth,
        } = *self;
        let copies = match args.copies {
            Some(Bound(bound)) => bound,
            None => Some(copies.unwrap_or(Options::DEFAULT_COPIES)),
        };
        let bound = Bound(copies).to_string();
        let recursion = args
            .recursion_limit
            .unwrap_or(Limits::DEFAULT_RECURSION_DEPTH);
        let (occurrences, multiplicity) = match Forest::new(sequent) {
            Ok(forest) => (forest.len(), multiplicity(&forest)),
            Err(_) => (0, 0),
        };
        // The check is the child's own, outside the time measured.
        let options = search_options(args, copies, witness_depth).with_check(false);
        let limits = Limits::default()
            .with_memory_bytes(memory_limit(args.memory_limit))
            .with_recursion_depth(recursion);
        say(LOADED);

        // The time limit is a flag that a thread of its own raises, so a poll
        // costs one load whatever the engine's cadence. A clock read only
        // every so many polls is late by that many polls. On a large net,
        // where polls come many milliseconds apart, that came to seconds.
        let clock = Clock::start(args.timeout);
        let outcome = decide(sequent, mode, &options, &limits, args, &clock.expired);
        let (time, cpu, wait) = clock.read();

        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(error) => {
                let verdict = match error {
                    Error::EngineRefused { .. } | Error::IntuitionisticMix => "refused",
                    _ => "error",
                };
                return row(&[
                    ("copies", &bound),
                    ("expected", expected),
                    ("verdict", verdict),
                    ("reason", &clean(&error.to_string())),
                    ("occurrences", &occurrences.to_string()),
                    ("multiplicity", &multiplicity.to_string()),
                    ("recursion_limit", &recursion.to_string()),
                    ("bias", &name(&args.bias)),
                    (
                        "forward_copies",
                        &forward_copies(args.forward_copies).to_string(),
                    ),
                    ("memory_limit", &memory_column(args.memory_limit)),
                    ("pool_after", &pool_column(args.pool_after)),
                    ("memo_limit", &memo_column(args.memo_limit)),
                ]);
            }
        };
        let (verdict, reason) = match &outcome.verdict {
            Verdict::Proved(_) => ("proved", ""),
            Verdict::Unprovable(_) => ("unprovable", ""),
            Verdict::Unknown(reason) => {
                // The harness's stop is its time limit.
                let reason = match reason {
                    Reason::Stopped => "timeout",
                    reason => reason.name(),
                };
                ("unknown", reason)
            }
        };
        let s = outcome.statistics;
        let tail = |checked: &str, check_ms: &str| {
            [
                bound.clone(),
                expected.to_owned(),
                verdict.to_owned(),
                reason.to_owned(),
                checked.to_owned(),
                outcome.engine.to_string(),
                outcome.fragment.name_in(mode).to_owned(),
                occurrences.to_string(),
                multiplicity.to_string(),
                format!("{time:.3}"),
                s.nodes.to_string(),
                s.memo_hits.to_string(),
                s.memo_entries.to_string(),
                s.splits.to_string(),
                s.links.to_string(),
                s.tests.to_string(),
                recursion.to_string(),
                cpu.clone(),
                wait.clone(),
                name(&args.bias),
                forward_copies(args.forward_copies).to_string(),
                check_ms.to_owned(),
                memory_column(args.memory_limit),
                s.copies.to_string(),
                pool_column(args.pool_after),
                s.forward_copies.to_string(),
                s.work.to_string(),
            ]
            .into_iter()
            // The columns of a row of `run --tools`: tool, tool_version,
            // translate_ms, peak_bytes and raw.
            .chain(std::iter::repeat_n(String::new(), 5))
            .chain([
                s.unifications.to_string(),
                s.bindings.to_string(),
                memo_column(args.memo_limit),
            ])
            .collect::<Vec<_>>()
            .join(",")
        };
        let proof = match &outcome.verdict {
            Verdict::Proved(proof) => proof,
            // A refutation with a certificate is checked, as a proof is. One
            // that rests on the search alone has nothing to check.
            Verdict::Unprovable(disproof) => return tail(&refutation_checked(disproof), ""),
            Verdict::Unknown(_) => return tail("", ""),
        };
        // The verdict goes out before the check, so that it stands when the
        // check of a large proof runs out of memory or is killed.
        say(&tail("", ""));
        let start = Instant::now();
        let checked = check(proof, &limits);
        let check = start.elapsed().as_secs_f64() * 1000.0;
        tail(&checked, &format!("{check:.3}"))
    }
}

/// Returns the search's options a run asks for, with the copy bound and
/// witness depth given.
pub(crate) fn search_options(
    args: &OneArgs,
    copies: Option<u32>,
    witness_depth: Option<u32>,
) -> Options {
    Options::default()
        .with_engine(args.engine.engine())
        .with_jobs(args.jobs)
        .with_copies(copies)
        .with_witness_depth(witness_depth.unwrap_or(Options::DEFAULT_WITNESS_DEPTH))
        .with_bias(args.bias.bias())
        .with_forward_copies(forward_copies(args.forward_copies))
        .with_test_period(args.test_period)
        .with_memo_limit(args.memo_limit.unwrap_or(Options::DEFAULT_MEMO_LIMIT))
        .with_classical_work(
            args.classical_work
                .unwrap_or(Options::DEFAULT_CLASSICAL_WORK),
        )
}

/// The clock of a run. A thread of its own raises a flag at the time
/// limit. The clock reads the wall-clock time, the CPU time and the time
/// waited for a CPU since the start.
pub(crate) struct Clock {
    /// Raised when the time limit has passed.
    pub(crate) expired: Arc<AtomicBool>,
    /// When the run started.
    start: Instant,
    /// The process's CPU time at the start.
    cpu: Option<f64>,
    /// The calling thread's time waited for a CPU at the start.
    wait: Option<f64>,
}

impl Clock {
    /// Starts the clock of a run with a time limit of `timeout` seconds.
    pub(crate) fn start(timeout: f64) -> Self {
        let expired = Arc::new(AtomicBool::new(false));
        let limit = Duration::from_secs_f64(timeout);
        let (cpu, wait) = (cpu_ms(), wait_ms());
        let start = Instant::now();
        {
            let expired = Arc::clone(&expired);
            thread::spawn(move || {
                thread::sleep(limit);
                expired.store(true, Ordering::Relaxed);
            });
        }
        Self {
            expired,
            start,
            cpu,
            wait,
        }
    }

    /// Returns the `time_ms`, `cpu_ms` and `wait_ms` columns since the
    /// start: the wall-clock time, the CPU time of the process's threads,
    /// and the time the calling thread was ready but waited for a CPU. The
    /// last is well above zero when another process slowed the run down.
    /// The last two are empty where `/proc` cannot tell.
    pub(crate) fn read(&self) -> (f64, String, String) {
        let time = self.start.elapsed().as_secs_f64() * 1000.0;
        let since = |after: Option<f64>, before: Option<f64>| {
            after
                .zip(before)
                .map_or(String::new(), |(a, b)| format!("{:.3}", a - b))
        };
        (time, since(cpu_ms(), self.cpu), since(wait_ms(), self.wait))
    }
}

/// Decides a problem under the options until the flag is raised.
///
/// Without `--pool-after`, or on one thread, the options' threads search
/// from the start. With it, the library's race runs, as in the command's
/// default: one thread searches alone, and once the delay has passed a
/// pool of the other threads searches beside it. The first to decide
/// answers. Both draw on one memory bound, and the outcome has the
/// counters of both.
fn decide(
    sequent: &deduxit::linear::Sequent,
    mode: deduxit::linear::Mode,
    options: &Options,
    limits: &Limits,
    args: &OneArgs,
    expired: &AtomicBool,
) -> Result<deduxit::linear::search::Outcome, Error> {
    // The refutation after the search is bounded by its own work, not by
    // the time limit, so it still answers a search the limit ended, as in
    // the command.
    let stop = |progress: deduxit::Progress| {
        progress.phase != deduxit::Phase::Refute && expired.load(Ordering::Relaxed)
    };
    let Some(alone) = args.pool_after.filter(|_| args.jobs > 1) else {
        return prove_within(sequent, mode, options, limits, stop);
    };
    let forest = Forest::within(sequent, limits)?;
    let alone = Duration::from_secs_f64(alone);
    let start = Instant::now();
    deduxit::linear::search::race(
        deduxit::linear::Goal::conclusion(&forest),
        mode,
        options,
        limits,
        args.jobs,
        |_| start.elapsed() >= alone,
        stop,
    )
}

/// Returns a tail with the given columns filled in and the others empty.
pub(crate) fn row(filled: &[(&str, &str)]) -> String {
    let mut fields = vec![""; TAIL];
    for &(name, value) in filled {
        fields[column(name)] = value;
    }
    fields.join(",")
}

/// Returns the CPU time of the process so far, in milliseconds: the user
/// and system time of all its threads, ended ones included, from
/// `/proc/self/stat`. It counts in the kernel's ticks of 10 ms. `None`
/// where there is no such file.
pub(crate) fn cpu_ms() -> Option<f64> {
    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    // After the command in parentheses, utime and stime are the 12th and
    // 13th fields.
    let mut fields = stat.rsplit_once(')')?.1.split_whitespace().skip(11);
    let user: f64 = fields.next()?.parse().ok()?;
    let system: f64 = fields.next()?.parse().ok()?;
    Some((user + system) * 10.0)
}

/// Returns how long the calling thread has been ready to run but waiting
/// for a CPU, in milliseconds, from `/proc/thread-self/schedstat`. `None`
/// where there is no such file.
pub(crate) fn wait_ms() -> Option<f64> {
    let stat = std::fs::read_to_string("/proc/thread-self/schedstat").ok()?;
    let nanoseconds: f64 = stat.split_whitespace().nth(1)?.parse().ok()?;
    Some(nanoseconds / 1e6)
}

/// Returns the most occurrences of one literal, `a` or `~a`, in the
/// forest: the `multiplicity` column.
fn multiplicity(forest: &Forest) -> usize {
    (0..forest.sequent().atom_names().len() as u32)
        .map(|a| {
            let atom = Atom::new(a);
            forest
                .literals(atom, Sign::Atom)
                .len()
                .max(forest.literals(atom, Sign::Dual).len())
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{HEADER, Options, Shard, finished};

    /// The N shards of a list take every problem once, each shard in the
    /// list's order, and by groups every group whole. A shard outside 1 to
    /// N is refused.
    #[test]
    fn shards_partition_the_problems_in_order() {
        let problems: Vec<u32> = (0..23).collect();
        for n in [1, 2, 5, 23, 30] {
            let mut joined: Vec<u32> = (1..=n)
                .flat_map(|k| {
                    format!("{k}/{n}")
                        .parse::<Shard>()
                        .unwrap()
                        .of(problems.clone())
                })
                .collect();
            let first = format!("1/{n}")
                .parse::<Shard>()
                .unwrap()
                .of(problems.clone());
            assert!(
                first.windows(2).all(|w| w[0] < w[1]),
                "a shard keeps the order"
            );
            joined.sort_unstable();
            assert_eq!(joined, problems, "{n} shards take every problem once");
            // By groups of three, a shard takes each group whole.
            let mut joined: Vec<u32> = (1..=n)
                .flat_map(|k| {
                    let shard = format!("{k}/{n}").parse::<Shard>().unwrap();
                    let taken = shard.of_groups(problems.clone(), |p| p / 3);
                    assert!(taken.iter().all(|p| taken.contains(&(p / 3 * 3))));
                    taken
                })
                .collect();
            joined.sort_unstable();
            assert_eq!(joined, problems, "{n} shards take every group once");
        }
        for bad in ["0/3", "4/3", "3", "a/b"] {
            assert!(bad.parse::<Shard>().is_err(), "{bad}");
        }
    }

    /// `--resume` refuses a file of other columns. From a file of its own
    /// it reads each row's key: what the row ran and its options, an empty
    /// option read as `finished` reads it.
    #[test]
    fn resume_reads_its_own_files_only() {
        let dir = std::env::temp_dir().join(format!("deduxit-resume-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert!(finished(&dir.join("absent.csv")).unwrap().is_empty());
        let foreign = dir.join("foreign.csv");
        std::fs::write(&foreign, "source,family,problem\nf,a,b\n").unwrap();
        let error = finished(&foreign).unwrap_err().to_string();
        assert!(error.contains("other columns"), "{error}");
        let row: Vec<&str> = HEADER
            .split(',')
            .map(|column| match column {
                "source" => "family",
                "family" => "chain",
                "problem" => "chain/3",
                "mode" => "classical",
                "engine_requested" => "auto",
                "jobs" => "1",
                _ => "",
            })
            .collect();
        let own = dir.join("own.csv");
        std::fs::write(&own, format!("{HEADER}\n{}\n", row.join(","))).unwrap();
        let keys = finished(&own).unwrap();
        assert_eq!(
            keys.into_iter().collect::<Vec<_>>(),
            [format!(
                "family,chain,chain/3,classical,auto,1,,auto,,0,,{}",
                Options::DEFAULT_MEMO_LIMIT
            )]
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
