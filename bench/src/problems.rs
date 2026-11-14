// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The five sources of problems: the generated families, LLTP files,
//! `.spec` files, problem files and TPTP files.
//!
//! The parent lists [`Reference`]s without building a sequent from a file,
//! so that an input that crashes the parser crashes a child, not the run.
//! The child [`load`]s the one it is given.
//!
//! A problem file (`bench/problems/*.txt`) has one problem per line:
//! `name; mode; expected; copies; sequent`. A blank line is skipped, and so
//! is everything from a `#` on. The fields are:
//! - the name, whose part before a `/` is the family (else the family is
//!   the file's stem);
//! - the mode's name as the CSV writes it, such as `classical`,
//!   `intuitionistic-affine`, `cyclic` or `lambek`;
//! - `provable`, `unprovable` or `-`;
//! - a copy bound, or `-`;
//! - the sequent as the `deduxit` command reads it, in the mode's order.
//!
//! A TPTP problem (`--tptp`, as in the ILTP library) is ordinary logic.
//! `run` decides it in each of its `--logics` instead of in a mode. Where
//! the file is one size of a generated family, such as `SYJ201+1.005.p`,
//! its family is that one (`SYJ201`, size 5); else it is its directory.

use anyhow::{Context, Result, anyhow, bail};
use deduxit::linear::families::{FAMILIES, find};
use deduxit::linear::lltp::{self, Status};
use deduxit::linear::mist::{self, Safety};
use deduxit::ordinary;
use deduxit::{
    Limits,
    linear::{Mode, Sequent},
};
use std::path::{Path, PathBuf};

/// A problem as the parent names it.
#[derive(Clone, Debug)]
pub struct Reference {
    /// Where it comes from: `family`, `lltp`, `spec`, `file` or `tptp`.
    pub source: &'static str,
    /// Its family: a generated family, a problem file's group, or the
    /// directory of an LLTP, `.spec` or TPTP file relative to the parent of
    /// the path given. A TPTP file of a generated ILTP family is in that
    /// family.
    pub family: String,
    /// The size of a generated problem.
    pub size: Option<u32>,
    /// The index of a generated problem among those of its size.
    pub index: Option<u32>,
    /// Its name within the family.
    pub name: String,
    /// The mode it is meant for. A TPTP problem runs in logics, not in
    /// modes: its mode is intuitionistic, and no run reads it.
    pub mode: Mode,
    /// The name the child gets of the problem to load:
    /// `family:NAME:SIZE:INDEX`, `lltp:PATH`, `spec:PATH`, `file:PATH:LINE`
    /// or `tptp:PATH`.
    pub id: String,
}

impl Reference {
    /// Whether the problem is a TPTP problem of ordinary logic.
    pub fn is_ordinary(&self) -> bool {
        self.source == TPTP
    }
}

/// The source of the problems of ordinary logic.
pub const TPTP: &str = "tptp";

/// A problem the child runs.
pub struct Problem {
    /// The sequent.
    pub sequent: Sequent,
    /// The mode it is meant for.
    pub mode: Mode,
    /// Whether it is provable, when the source says.
    pub expected: Option<bool>,
    /// The copy bound the source names, if any.
    pub copies: Option<u32>,
    /// The witness depth the source names, if any.
    pub witness_depth: Option<u32>,
}

/// Lists the instances of the families named in `specs` (`NAME` or
/// `NAME=SIZES`). With `all` it adds every family at its default sizes, and
/// with `growing` every family's growing set.
pub fn families(specs: &[String], all: bool, growing: bool) -> Result<Vec<Reference>> {
    let mut chosen = Vec::new();
    if all {
        chosen.extend(FAMILIES.iter().map(|f| (f, f.sizes.to_vec())));
    }
    if growing {
        chosen.extend(
            FAMILIES
                .iter()
                .filter(|f| !f.growth.is_empty())
                .map(|f| (f, f.growth.to_vec())),
        );
    }
    for spec in specs {
        let (name, sizes) = spec.split_once('=').unwrap_or((spec, ""));
        let family = find(name).ok_or_else(|| anyhow!("no family `{name}`"))?;
        let sizes = if sizes.is_empty() {
            family.sizes.to_vec()
        } else {
            sizes
                .split(',')
                .map(|s| s.trim().parse().with_context(|| format!("size `{s}`")))
                .collect::<Result<_>>()?
        };
        chosen.push((family, sizes));
    }
    let mut references = Vec::new();
    for (family, sizes) in chosen {
        for size in sizes {
            for index in 0..family.instances {
                let instance = family.instance(size, index)?;
                references.push(Reference {
                    source: "family",
                    family: family.name.to_owned(),
                    size: Some(size),
                    index: Some(index),
                    name: instance.name,
                    mode: instance.mode,
                    id: format!("family:{}:{size}:{index}", family.name),
                });
            }
        }
    }
    Ok(references)
}

/// Lists the LLTP problems under the paths, in path order.
pub fn lltp(paths: &[PathBuf]) -> Result<Vec<Reference>> {
    listed(paths, "lltp", "p", lltp_mode)
}

/// Lists the coverability problems under the paths, in path order, each in
/// intuitionistic affine mode. A coverability problem asks whether a
/// marking that covers the target is reachable.
pub fn specs(paths: &[PathBuf]) -> Result<Vec<Reference>> {
    listed(paths, "spec", "spec", |_| {
        Mode::INTUITIONISTIC.with_affine()
    })
}

/// Lists the TPTP problems of ordinary logic under the paths, in path
/// order. A problem that is one size of a generated family is in the
/// family its name gives.
pub fn tptp(paths: &[PathBuf]) -> Result<Vec<Reference>> {
    let mut references = listed(paths, TPTP, "p", |_| Mode::INTUITIONISTIC)?;
    for reference in &mut references {
        if let Some((family, size)) = sized(&reference.name) {
            reference.family = family.to_owned();
            reference.size = Some(size);
        }
    }
    Ok(references)
}

/// Returns the family and size in a TPTP file name of one size of a
/// generated family, `NAME+VERSION.SIZE.p` with a size of three digits:
/// `SYJ201+1.005.p` is `SYJ201` at 5. Any other name gives `None`.
fn sized(name: &str) -> Option<(&str, u32)> {
    let (family, rest) = name.strip_suffix(".p")?.split_once('+')?;
    let (_, size) = rest.split_once('.')?;
    (size.len() == 3)
        .then(|| size.parse().ok())
        .flatten()
        .map(|size| (family, size))
}

/// Loads a TPTP problem of ordinary logic that a [`Reference::id`] names:
/// its sequent, and the status its header claims.
pub fn load_ordinary(id: &str) -> Result<ordinary::Problem> {
    let path = id
        .strip_prefix("tptp:")
        .ok_or_else(|| anyhow!("not a TPTP problem: `{id}`"))?;
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
    Ok(ordinary::read_tptp(&text, &Limits::default())?)
}

/// Lists the files of a source under the paths, in path order. Each path
/// is a file, or a directory searched for files with the extension. The
/// family of a file is its directory relative to the parent of the path
/// given.
fn listed(
    paths: &[PathBuf],
    source: &'static str,
    extension: &str,
    mode: fn(&Path) -> Mode,
) -> Result<Vec<Reference>> {
    let mut references = Vec::new();
    for root in paths {
        let mut files = Vec::new();
        collect(root, extension, &mut files)
            .with_context(|| format!("reading {}", root.display()))?;
        files.sort();
        let base = root.parent().unwrap_or(Path::new(""));
        for file in files {
            let directory = file.parent().unwrap_or(Path::new(""));
            let family = directory.strip_prefix(base).unwrap_or(directory);
            references.push(Reference {
                source,
                family: family.display().to_string(),
                size: None,
                index: None,
                name: file
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                mode: mode(&file),
                id: format!("{source}:{}", file.display()),
            });
        }
    }
    Ok(references)
}

/// Adds `path` if it is a file, and the files with the extension below it
/// if it is a directory.
fn collect(path: &Path, extension: &str, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if !path.is_dir() {
        files.push(path.to_owned());
        return Ok(());
    }
    for entry in std::fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(&path, extension, files)?;
        } else if path.extension().is_some_and(|e| e == extension) {
            files.push(path);
        }
    }
    Ok(())
}

/// Returns the mode of an LLTP problem: intuitionistic under the library's
/// `ILL` directory, classical otherwise.
fn lltp_mode(path: &Path) -> Mode {
    if path.components().any(|c| c.as_os_str() == "ILL") {
        Mode::INTUITIONISTIC
    } else {
        Mode::CLASSICAL
    }
}

/// Lists the problems of problem files.
pub fn files(paths: &[PathBuf]) -> Result<Vec<Reference>> {
    let mut references = Vec::new();
    for path in paths {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        for (number, line) in text.lines().enumerate() {
            let Some([name, mode, ..]) = fields(line) else {
                continue;
            };
            let place = format!("{}:{}", path.display(), number + 1);
            let (family, _) = name.split_once('/').unwrap_or((&stem, name));
            references.push(Reference {
                source: "file",
                family: family.to_owned(),
                size: None,
                index: None,
                name: name.to_owned(),
                mode: parse_mode(mode).with_context(|| place.clone())?,
                id: format!("file:{place}"),
            });
        }
    }
    Ok(references)
}

/// Returns the five fields of a problem line, or `None` for a comment or a
/// blank line. A missing field is empty.
fn fields(line: &str) -> Option<[&str; 5]> {
    let line = line.split_once('#').map_or(line, |(code, _)| code).trim();
    if line.is_empty() {
        return None;
    }
    let mut fields = line.splitn(5, ';').map(str::trim);
    Some(std::array::from_fn(|_| fields.next().unwrap_or("")))
}

/// Reads a mode's name as problem files and the CSV write it.
fn parse_mode(name: &str) -> Result<Mode> {
    Ok(name.parse()?)
}

/// Returns a mode's name as problem files and the CSV write it.
pub fn mode_name(mode: Mode) -> &'static str {
    mode.name()
}

/// Loads the problem a [`Reference::id`] names.
pub fn load(id: &str) -> Result<Problem> {
    let (source, rest) = id.split_once(':').unwrap_or((id, ""));
    match source {
        "family" => {
            let mut parts = rest.split(':');
            let (Some(name), Some(size), Some(index)) = (parts.next(), parts.next(), parts.next())
            else {
                bail!("not a family problem: `{id}`");
            };
            let family = find(name).ok_or_else(|| anyhow!("no family `{name}`"))?;
            let instance = family.instance(size.parse()?, index.parse()?)?;
            Ok(Problem {
                sequent: instance.sequent,
                mode: instance.mode,
                expected: Some(instance.provable),
                copies: instance.copies,
                witness_depth: instance.witness_depth,
            })
        }
        "lltp" => {
            let text = std::fs::read_to_string(rest).with_context(|| format!("reading {rest}"))?;
            let problem = lltp::read(&text, &Limits::default())?;
            Ok(Problem {
                sequent: problem.sequent,
                mode: lltp_mode(Path::new(rest)),
                expected: problem.status.map(|s| s == Status::Theorem),
                copies: None,
                witness_depth: None,
            })
        }
        "spec" => {
            let text = std::fs::read_to_string(rest).with_context(|| format!("reading {rest}"))?;
            let problem = mist::read(&text, &Limits::default())?;
            Ok(Problem {
                sequent: problem.sequent,
                mode: Mode::INTUITIONISTIC.with_affine(),
                expected: problem.expected.map(|s| s == Safety::Unsafe),
                copies: None,
                witness_depth: None,
            })
        }
        "file" => {
            let (path, line) = rest
                .rsplit_once(':')
                .ok_or_else(|| anyhow!("not a file problem: `{id}`"))?;
            let text = std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
            let line: usize = line.parse()?;
            let [_, mode, expected, copies, sequent] = text
                .lines()
                .nth(line - 1)
                .and_then(fields)
                .ok_or_else(|| anyhow!("no problem on line {line} of {path}"))?;
            // The sequent of an ordered mode, such as `cyclic`, is read in
            // that mode's order.
            let mode = parse_mode(mode)?;
            Ok(Problem {
                sequent: Sequent::parse_ordered(sequent, mode.order(), &Limits::default())?,
                mode,
                expected: match expected {
                    "provable" => Some(true),
                    "unprovable" => Some(false),
                    _ => None,
                },
                copies: copies.parse().ok(),
                witness_depth: None,
            })
        }
        TPTP => bail!("`{id}` is ordinary logic, which runs in a logic, not a mode"),
        _ => bail!("unknown problem source in `{id}`"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every commutative mode reads back from its name, with Mix tried in
    /// the classical modes alone. A word that names no mode is an error.
    #[test]
    fn mode_names_read_back() {
        for intuitionistic in [false, true] {
            for affine in [false, true] {
                for mix in [false, !intuitionistic] {
                    let mut mode = if intuitionistic {
                        Mode::INTUITIONISTIC
                    } else {
                        Mode::CLASSICAL
                    };
                    if affine {
                        mode = mode.with_affine();
                    }
                    if mix {
                        mode = mode.with_mix();
                    }
                    assert_eq!(parse_mode(mode_name(mode)).unwrap(), mode);
                }
            }
        }
        assert!(parse_mode("classicl").is_err());
    }

    /// The name of a TPTP problem of one size of a generated family gives
    /// that family and size; any other name gives none.
    #[test]
    fn tptp_families_are_read_from_names() {
        assert_eq!(sized("SYJ201+1.005.p"), Some(("SYJ201", 5)));
        assert_eq!(sized("SYJ105+1.2.p"), None);
        assert_eq!(sized("LCL181+1.p"), None);
    }
}
