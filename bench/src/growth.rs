// linlog © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use anyhow::{Result, bail};
use std::collections::HashMap;
use std::path::Path;

/// The verdict of a size a growing set's configuration did not run, past
/// the limit its series met at a smaller size: no run, which every reader
/// of the rows drops.
pub const SKIPPED: &str = "skipped";

/// The columns of `run`'s rows that name a series: one shape of a family
/// under one configuration.
pub const ENGINE_SERIES: [&str; 5] = ["family", "index", "mode", "engine_requested", "jobs"];

/// The columns of `run --tools`'s rows that name a series: one shape of a
/// family under one tool.
pub const TOOL_SERIES: [&str; 4] = ["family", "index", "mode", "tool"];

/// The series of the growing sets that a run has ended, each with the size
/// it ended at: the shape's larger sizes are not run in that
/// configuration, since every growing set is a constructed family whose
/// larger instance of a shape is no easier.
#[derive(Debug, Default)]
pub struct Ended(HashMap<String, String>);

impl Ended {
    /// The series that the rows of a CSV file ended, read by the columns'
    /// names, so that a resumed run skips what the first run would have;
    /// none for a file that does not exist yet.
    pub fn read(path: &Path, series: &[&str]) -> Result<Self> {
        let Ok(text) = std::fs::read_to_string(path) else {
            return Ok(Self::default());
        };
        let mut lines = text.lines();
        let columns: Vec<&str> = lines.next().unwrap_or("").split(',').collect();
        let at = |name: &str| columns.iter().position(|c| *c == name);
        let (
            Some(key),
            Some(source),
            Some(size),
            Some(verdict),
            Some(reason),
            Some(time),
            Some(timeout),
        ) = (
            series
                .iter()
                .map(|name| at(name))
                .collect::<Option<Vec<usize>>>(),
            at("source"),
            at("size"),
            at("verdict"),
            at("reason"),
            at("time_ms"),
            at("timeout_s"),
        )
        else {
            bail!("{} is not a CSV file of `run`", path.display());
        };
        let mut ended = Self::default();
        for line in lines {
            let fields: Vec<&str> = line.split(',').collect();
            let field = |i: usize| fields.get(i).copied().unwrap_or("");
            if field(source) != "family" {
                continue;
            }
            let values: Vec<&str> = key.iter().map(|&i| field(i)).collect();
            ended.record(
                &values,
                field(size),
                field(verdict),
                field(reason),
                field(time).parse().unwrap_or(0.0),
                field(timeout).parse().unwrap_or(f64::INFINITY),
            );
        }
        Ok(ended)
    }

    /// The reason of a skipped row of the series, if a run ended it: the
    /// size it was past its limit at.
    pub fn skipped(&self, series: &[&str]) -> Option<String> {
        self.0
            .get(&series.join(","))
            .map(|size| format!("past the limit at size {size}"))
    }

    /// Ends the series if the run ended it: refused (the engine or tool
    /// does not take the shape at any size), or not decided within its
    /// time limit, at the memory bound, killed or crashed. A run that ended
    /// at a bound of the search's own (copies, recursion, witness depth)
    /// ends nothing: such a run is fast, and a family whose unprovable
    /// shape is searched to a bound ends there at every size.
    pub fn record(
        &mut self,
        series: &[&str],
        size: &str,
        verdict: &str,
        reason: &str,
        time_ms: f64,
        timeout_s: f64,
    ) {
        if verdict == SKIPPED {
            return;
        }
        let limited = matches!(
            reason,
            "timeout" | "killed" | "killed while loading" | "memory_limit" | "out_of_memory"
        ) || reason.starts_with("crash");
        let late = time_ms >= timeout_s * 1000.0;
        if verdict == "refused" || (verdict == "unknown" && limited) || late {
            self.0
                .entry(series.join(","))
                .or_insert_with(|| size.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A refusal, a run at the time limit and a late verdict end their
    /// series at their size, and a skipped row does not move that size; a
    /// run at a bound of the search's own ends nothing.
    #[test]
    fn what_ends_a_series() {
        let mut ended = Ended::default();
        for (series, verdict, reason, time) in [
            ("refused", "refused", "fragment", 0.0),
            ("timeout", "unknown", "timeout", 60_000.0),
            ("late", "unprovable", "", 60_200.0),
            ("bound", "unknown", "copy_bound", 10.0),
            ("decided", "proved", "", 5.0),
        ] {
            ended.record(&[series], "8", verdict, reason, time, 60.0);
        }
        ended.record(&["timeout"], "16", SKIPPED, "", 0.0, 60.0);
        for series in ["refused", "timeout", "late"] {
            assert_eq!(
                ended.skipped(&[series]).as_deref(),
                Some("past the limit at size 8")
            );
        }
        assert!(ended.skipped(&["bound"]).is_none() && ended.skipped(&["decided"]).is_none());
    }
}
