// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

use super::SequentForm;
use crate::Error;
use crate::limits::Limits;
#[cfg(feature = "interactive")]
use crate::ordinary::Rule;
use crate::ordinary::Sequent;
use crate::serialize::sequents::Extension;
use crate::wire::{self, Readable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The serialized form of a truth table: the sequent, a row per
/// assignment as a `T` or `F` per formula of the sequent's form, and the
/// rows that falsify the sequent.
#[derive(Serialize, Deserialize)]
pub(super) struct TableForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The sequent.
    sequent: SequentForm,
    /// The rows, from every atom true to every atom false.
    rows: Vec<String>,
    /// The rows that falsify the sequent.
    falsifying: Vec<u64>,
}

impl From<&crate::ordinary::TruthTable> for TableForm {
    /// Writes the table's form, nested: without its level.
    fn from(table: &crate::ordinary::TruthTable) -> Self {
        let sequent = table.sequent();
        Self {
            version: None,
            sequent: SequentForm::of_sequent(sequent),
            rows: (0..table.rows())
                .map(|row| {
                    table
                        .row(row)
                        .map(|value| if value { 'T' } else { 'F' })
                        .collect()
                })
                .collect(),
            falsifying: table.falsifying().to_vec(),
        }
    }
}

impl TableForm {
    /// Returns the table of the form's sequent, computed again within
    /// `limits`; refuses rows or falsifying rows that are not that
    /// table's.
    pub(super) fn within(self, limits: &Limits) -> Result<crate::ordinary::TruthTable, Error> {
        let (read, left, right) = self.sequent.within(limits, Extension::Sequent)?;
        let ids = read.ids.clone();
        let sequent = Sequent::new(read.formulas, left, right)?;
        let table = crate::ordinary::truth_table(&sequent, limits, |_| false)?;
        let matches = self.rows.len() as u64 == table.rows()
            && self.rows.iter().enumerate().all(|(row, text)| {
                text.chars().count() == ids.len()
                    && text.chars().zip(&ids).all(|(c, &id)| {
                        (c == 'T') == table.value(row as u64, id) && "TF".contains(c)
                    })
            })
            && self.falsifying == table.falsifying();
        if !matches {
            return Err(Error::Json {
                form: "truth table",
                message: "the rows are not the sequent's truth table".to_owned(),
            });
        }
        Ok(table)
    }
}

impl Serialize for crate::ordinary::TruthTable {
    /// Serializes the table as a document at level 14.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = TableForm::from(self);
        form.version = Some(wire::TABLES);
        form.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for crate::ordinary::TruthTable {
    /// Deserializes a table within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

impl crate::sealed::Readable for crate::ordinary::TruthTable {}

impl Readable for crate::ordinary::TruthTable {
    const FORM: &'static str = "truth table";

    /// Reads a truth table, computed again from its sequent.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        TableForm::deserialize(deserializer)?
            .within(limits)
            .map_err(wire::fail)
    }
}

/// A sign as a tableau's form writes it.
#[cfg(feature = "interactive")]
#[derive(Clone, Copy, Serialize, Deserialize)]
enum SignForm {
    /// True.
    T,
    /// False.
    F,
}

/// How a branch ends, as a tableau's form writes it, tagged by `kind`.
#[cfg(feature = "interactive")]
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum EndForm {
    /// Closed by a formula true at one line and false at another.
    Closed {
        /// The line where it is true.
        true_line: u32,
        /// The line where it is false.
        false_line: u32,
    },
    /// Closed by `⊥` true.
    Falsity {
        /// The line.
        line: u32,
    },
    /// Closed by `⊤` false.
    Truth {
        /// The line.
        line: u32,
    },
    /// Open and finished.
    Open,
    /// Open and not expanded.
    Unexpanded,
}

/// The serialized form of a tableau's node.
#[cfg(feature = "interactive")]
#[derive(Serialize, Deserialize)]
struct TableauNodeForm {
    /// The lines, each a sign and a formula's index.
    lines: Vec<(SignForm, u32)>,
    /// The line the children expand and the rule, absent for a leaf.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expands: Option<(u32, Rule)>,
    /// The children, by index; absent for a leaf.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<u32>,
    /// How the branch ends, for a leaf.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    end: Option<EndForm>,
}

/// The serialized form of a tableau: the sequent with every formula a
/// line names, and the nodes, each before its children.
#[cfg(feature = "interactive")]
#[derive(Serialize, Deserialize)]
pub(super) struct TableauForm {
    /// The wire level, written only at the top of a document.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "wire::version"
    )]
    version: Option<u32>,
    /// The sequent, its form holding every formula a line names.
    sequent: SequentForm,
    /// The nodes.
    nodes: Vec<TableauNodeForm>,
}

#[cfg(feature = "interactive")]
impl From<&crate::ordinary::Tableau> for TableauForm {
    /// Writes the tableau's form, nested: without its level.
    fn from(tableau: &crate::ordinary::Tableau) -> Self {
        use crate::ordinary::{End, Sign};
        let (left, right) = tableau.sequent();
        Self {
            version: None,
            sequent: SequentForm::of(tableau.formulas(), left, right),
            nodes: tableau
                .nodes()
                .iter()
                .map(|node| TableauNodeForm {
                    lines: node
                        .lines()
                        .iter()
                        .map(|line| {
                            let sign = match line.sign {
                                Sign::True => SignForm::T,
                                Sign::False => SignForm::F,
                            };
                            (sign, line.formula.index() as u32)
                        })
                        .collect(),
                    expands: node.expands(),
                    children: node.children().to_vec(),
                    end: node.end().map(|end| match end {
                        End::Closed {
                            true_line,
                            false_line,
                        } => EndForm::Closed {
                            true_line,
                            false_line,
                        },
                        End::Falsity { line } => EndForm::Falsity { line },
                        End::Truth { line } => EndForm::Truth { line },
                        End::Open => EndForm::Open,
                        End::Unexpanded => EndForm::Unexpanded,
                    }),
                })
                .collect(),
        }
    }
}

#[cfg(feature = "interactive")]
impl TableauForm {
    /// Returns the tableau the form holds, its sequent within the limits.
    /// Refuses a form without the shape every tableau has: at least one
    /// node; every node but the first the child of exactly one node before
    /// it; a node with children expanding a line and without an end, a
    /// leaf with an end and expanding nothing; every line named in range.
    /// Whether its expansions follow the rules is not checked.
    pub(super) fn within(self, limits: &Limits) -> Result<crate::ordinary::Tableau, Error> {
        use crate::ordinary::{End, Sign, Signed, Tableau, TableauNode};
        let refuse = |message: String| Error::Json {
            form: "tableau",
            message,
        };
        let (read, left, right) = self.sequent.within(limits, Extension::Sequent)?;
        read.formulas.propositional("a tableau")?;
        let total: u64 = self.nodes.iter().map(|n| n.lines.len() as u64).sum();
        let total =
            u32::try_from(total).map_err(|_| refuse("more lines than a u32 numbers".to_owned()))?;
        let line = |n: u32| {
            if n < total {
                Ok(n)
            } else {
                Err(refuse(format!(
                    "line {n} is past the tableau's {total} lines"
                )))
            }
        };
        let len = self.nodes.len();
        let mut parents = vec![0u8; len];
        let mut nodes = Vec::with_capacity(len);
        let mut first = 0u32;
        for (n, form) in self.nodes.into_iter().enumerate() {
            let lines = form
                .lines
                .iter()
                .map(|&(sign, k)| {
                    let sign = match sign {
                        SignForm::T => Sign::True,
                        SignForm::F => Sign::False,
                    };
                    Ok(Signed::new(sign, read.ids(&[k])?[0]))
                })
                .collect::<Result<Vec<_>, Error>>()?;
            for &child in &form.children {
                if (child as usize) <= n || child as usize >= len {
                    return Err(refuse(format!("node {n} names child {child}")));
                }
                parents[child as usize] = parents[child as usize].saturating_add(1);
            }
            let expands = form
                .expands
                .map(|(l, rule)| line(l).map(|l| (l, rule)))
                .transpose()?;
            let end = form
                .end
                .map(|end| {
                    Ok::<_, Error>(match end {
                        EndForm::Closed {
                            true_line,
                            false_line,
                        } => End::Closed {
                            true_line: line(true_line)?,
                            false_line: line(false_line)?,
                        },
                        EndForm::Falsity { line: l } => End::Falsity { line: line(l)? },
                        EndForm::Truth { line: l } => End::Truth { line: line(l)? },
                        EndForm::Open => End::Open,
                        EndForm::Unexpanded => End::Unexpanded,
                    })
                })
                .transpose()?;
            let leaf = form.children.is_empty();
            if leaf != end.is_some() || leaf == expands.is_some() {
                return Err(refuse(format!(
                    "node {n} must have children and expand a line, or end its branch"
                )));
            }
            let count = lines.len() as u32;
            nodes.push(TableauNode::from_parts(
                first,
                lines,
                (expands, form.children, end),
            ));
            first += count;
        }
        let shaped = parents
            .split_first()
            .is_some_and(|(&root, rest)| root == 0 && rest.iter().all(|&p| p == 1));
        if !shaped {
            return Err(refuse(
                "the nodes are no tree with its root first".to_owned(),
            ));
        }
        Ok(Tableau::from_parts(read.formulas, (left, right), nodes))
    }
}

#[cfg(feature = "interactive")]
impl Serialize for crate::ordinary::Tableau {
    /// Serializes the tableau as a document at level 14.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut form = TableauForm::from(self);
        form.version = Some(wire::TABLES);
        form.serialize(serializer)
    }
}

#[cfg(feature = "interactive")]
impl<'a> Deserialize<'a> for crate::ordinary::Tableau {
    /// Deserializes a tableau within the default limits, as
    /// [`wire::upgrade`] does.
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Self::read(deserializer, &Limits::default())
    }
}

#[cfg(feature = "interactive")]
impl crate::sealed::Readable for crate::ordinary::Tableau {}

#[cfg(feature = "interactive")]
impl Readable for crate::ordinary::Tableau {
    const FORM: &'static str = "tableau";

    /// Reads a tableau: its shape and indices checked, not its
    /// expansions.
    fn read<'de, D: Deserializer<'de>>(deserializer: D, limits: &Limits) -> Result<Self, D::Error> {
        TableauForm::deserialize(deserializer)?
            .within(limits)
            .map_err(wire::fail)
    }
}
