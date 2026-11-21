// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The truth table of a sequent of classical logic: every assignment of
//! its atoms, the value of every formula under each, and the rows where
//! the sequent is false.

use super::{Logic, Node, NodeId, Sequent};
use crate::Error;
use crate::kripke::{Model, World};
use crate::limits::{Limits, Phase, Progress, Refusal};
use std::fmt::{self, Display, Formatter, Write};

/// The truth table of a sequent of classical logic.
///
/// A row is an assignment of the sequent's atoms, in the order of
/// [`Formulas::atom_names`](super::Formulas::atom_names): row 0 makes
/// every atom true, the last row every atom false, and the first atom
/// changes slowest, as textbooks write a table. The table holds the value
/// of every formula of the sequent's arena in every row. A row falsifies
/// the sequent where every formula left of `⊢` is true and every formula
/// right of it false.
///
/// [`truth_table`] makes one within [`Limits::table_atoms`], since the
/// rows double with each atom.
#[derive(Clone, Debug)]
pub struct TruthTable {
    /// The sequent the table is of.
    sequent: Sequent,
    /// A bit per row and formula of the arena, row by row.
    bits: Vec<u64>,
    /// The rows that falsify the sequent, in order.
    falsifying: Vec<u64>,
}

impl TruthTable {
    /// Returns the sequent the table is of.
    pub fn sequent(&self) -> &Sequent {
        &self.sequent
    }

    /// Returns how many atoms the table assigns.
    pub fn atoms(&self) -> usize {
        self.sequent.formulas().atom_names().len()
    }

    /// Returns how many rows the table has: two to the number of atoms.
    pub fn rows(&self) -> u64 {
        1 << self.atoms()
    }

    /// Returns the value row `row` gives the atom with index `atom` among
    /// the arena's atom names.
    ///
    /// # Panics
    ///
    /// For a row or an atom past the table's.
    pub fn assigns(&self, row: u64, atom: usize) -> bool {
        let atoms = self.atoms();
        assert!(
            row < self.rows() && atom < atoms,
            "a row and an atom of the table"
        );
        row >> (atoms - 1 - atom) & 1 == 0
    }

    /// Returns the value of the formula `id` of the sequent's arena in row
    /// `row`.
    ///
    /// # Panics
    ///
    /// For a row or a formula past the table's.
    pub fn value(&self, row: u64, id: NodeId) -> bool {
        let width = self.sequent.formulas().len();
        assert!(
            row < self.rows() && id.index() < width,
            "a row and a formula of the table"
        );
        // Below `rows · width`, which fits the vector's index.
        let cell = row as usize * width + id.index();
        self.bits[cell / 64] >> (cell % 64) & 1 == 1
    }

    /// Returns the values of every formula of the sequent's arena in row
    /// `row`, in the arena's order, each operand before its formula.
    ///
    /// # Panics
    ///
    /// For a row past the table's.
    pub fn row(&self, row: u64) -> impl Iterator<Item = bool> + '_ {
        let width = self.sequent.formulas().len();
        (0..width).map(move |id| self.value(row, NodeId(id as u32)))
    }

    /// Returns whether row `row` falsifies the sequent.
    pub fn falsifies(&self, row: u64) -> bool {
        self.falsifying.binary_search(&row).is_ok()
    }

    /// Returns the rows that falsify the sequent, in order. None means the
    /// sequent is valid.
    pub fn falsifying(&self) -> &[u64] {
        &self.falsifying
    }

    /// Returns row `row` as a Kripke model of one world, the atoms it
    /// makes true forced there.
    pub fn model(&self, row: u64) -> Model {
        let names = self.sequent.formulas().atom_names();
        let atoms = (0..names.len())
            .filter(|&a| self.assigns(row, a))
            .map(|a| names[a].clone())
            .collect();
        Model::new(vec![World::new(atoms, false, Vec::new())], 0)
    }
}

impl TruthTable {
    /// Writes the table as text: a column per atom, then a column per
    /// formula left of `⊢` and right of it, a row per assignment with `T`
    /// and `F` under each column's name, and `×` after a row that
    /// falsifies the sequent.
    ///
    /// # Errors
    ///
    /// The writer's.
    pub fn write_text(&self, out: &mut impl Write) -> fmt::Result {
        let formulas = self.sequent.formulas();
        let names = formulas.atom_names();
        // Each column's header and, for a formula, its id.
        let mut columns: Vec<(String, Option<NodeId>)> =
            names.iter().map(|name| (name.clone(), None)).collect();
        let atoms = columns.len();
        for &id in self.sequent.left().iter().chain(self.sequent.right()) {
            columns.push((formulas.formula(id).to_string(), Some(id)));
        }
        let widths: Vec<usize> = columns.iter().map(|(h, _)| h.chars().count()).collect();
        let hypotheses = atoms + self.sequent.left().len();
        // What stands before column `c`: two spaces, or a separator.
        let gap = |c: usize| match c {
            0 => "",
            c if c == atoms => "  │  ",
            c if c == hypotheses => "  ⊢  ",
            _ => "  ",
        };
        let gap = |c: usize| {
            if c == hypotheses && c == atoms {
                "  │  ⊢  "
            } else {
                gap(c)
            }
        };
        for (c, (header, _)) in columns.iter().enumerate() {
            write!(out, "{}{header}", gap(c))?;
        }
        if atoms == columns.len() || hypotheses == columns.len() {
            write!(out, "{}", gap(columns.len()))?;
        }
        // A row's separators: `│` after the atoms, spaces under `⊢`.
        let under = |c: usize| if c == atoms { gap(c) } else { "     " };
        let mut line = String::new();
        for row in 0..self.rows() {
            line.clear();
            for (c, (_, id)) in columns.iter().enumerate() {
                let value = match id {
                    None => self.assigns(row, c),
                    Some(id) => self.value(row, *id),
                };
                let letter = if value { 'T' } else { 'F' };
                let before = (widths[c] - 1) / 2;
                let after = widths[c] - 1 - before;
                let gap = if c == hypotheses && c != atoms {
                    under(c)
                } else {
                    gap(c)
                };
                write!(line, "{gap}{:before$}{letter}{:after$}", "", "")?;
            }
            if self.falsifies(row) {
                line.push_str("  ×");
            }
            write!(out, "\n{}", line.trim_end())?;
        }
        Ok(())
    }
}

impl Display for TruthTable {
    /// Writes the table as [`write_text`](Self::write_text) does.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_text(f)
    }
}

/// Returns the truth table of a sequent of classical logic, asking `stop`
/// once per row.
///
/// # Errors
///
/// [`Refusal::Atoms`] for more atoms than [`Limits::table_atoms`];
/// [`Refusal::Memory`] for a table past `limits.memory_bytes` (a bit per
/// row and formula); [`Refusal::Work`] for more rows than `limits.work`;
/// [`Refusal::Index`] for a table whose cells the machine cannot index;
/// [`Refusal::Stopped`] when `stop` fired.
pub fn truth_table(
    sequent: &Sequent,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<TruthTable, Error> {
    let formulas = sequent.formulas();
    formulas.propositional(super::Method::Table.title())?;
    let atoms = formulas.atom_names().len() as u64;
    let width = formulas.len() as u64;
    if let Some(limit) = limits.table_atoms
        && atoms > u64::from(limit)
    {
        return Err(Error::Refused(Refusal::Atoms {
            atoms,
            limit: u64::from(limit),
        }));
    }
    let rows = 1u64.checked_shl(atoms as u32).filter(|_| atoms < 64);
    let cells = rows.and_then(|rows| rows.checked_mul(width));
    // The bits, and the list of falsifying rows, which every row may join.
    let needed = cells.zip(rows).map_or(u64::MAX, |(cells, rows)| {
        cells.div_ceil(8).saturating_add(rows.saturating_mul(8))
    });
    if let Some(limit) = limits.memory_bytes
        && needed > limit
    {
        return Err(Error::Refused(Refusal::Memory {
            phase: Phase::Search,
            limit_bytes: limit,
            needed_bytes: Some(needed),
        }));
    }
    let (Some(rows), Some(cells)) = (rows, cells) else {
        return Err(index(atoms));
    };
    if let Some(limit) = limits.work
        && rows > limit
    {
        return Err(Error::Refused(Refusal::Work { limit }));
    }
    let Ok(cells) = usize::try_from(cells) else {
        return Err(index(atoms));
    };
    let width = width as usize;
    let mut bits = vec![0u64; cells.div_ceil(64)];
    let mut falsifying = Vec::new();
    let mut values = vec![false; width];
    for row in 0..rows {
        if stop(Progress::new(Phase::Search, 1, row + 1)) {
            return Err(Error::Refused(Refusal::Stopped {
                phase: Phase::Search,
            }));
        }
        // Every operand stands before its formula in the arena.
        for id in 0..width {
            let at = |k: NodeId| values[k.index()];
            values[id] = match formulas.node(NodeId(id as u32)) {
                Node::Atom(a) => row >> (atoms - 1 - u64::from(a)) & 1 == 0,
                Node::True => true,
                Node::False => false,
                Node::Not(a) => !at(a),
                Node::And(a, b) => at(a) && at(b),
                Node::Or(a, b) => at(a) || at(b),
                Node::Implies(a, b) => !at(a) || at(b),
                Node::Iff(a, b) => at(a) == at(b),
                Node::Forall(_) | Node::Exists(_) => unreachable!("{}", crate::ordinary::REFUSED),
            };
        }
        // Below `cells`, which is a `usize`.
        let start = row as usize * width;
        for (id, &value) in values.iter().enumerate() {
            let cell = start + id;
            bits[cell / 64] |= u64::from(value) << (cell % 64);
        }
        if sequent.left().iter().all(|h| values[h.index()])
            && sequent.right().iter().all(|c| !values[c.index()])
        {
            falsifying.push(row);
        }
    }
    Ok(TruthTable {
        sequent: sequent.clone(),
        bits,
        falsifying,
    })
}

/// Returns the refusal of a table whose cells no index of the machine
/// names.
fn index(atoms: u64) -> Error {
    Error::Refused(Refusal::Index {
        what: crate::limits::Space::Atom,
        count: atoms,
        most: 63,
    })
}

/// Decides a sequent of classical logic by its truth table: valid where
/// no row falsifies it, else not valid with the first falsifying row as
/// a checked countermodel of one world.
pub(super) fn decide(
    sequent: &Sequent,
    logic: Logic,
    limits: &Limits,
    mut stop: impl FnMut(Progress) -> bool,
) -> Result<super::Outcome, Error> {
    if logic != Logic::Classical {
        return Err(Error::Logic {
            logic,
            method: super::Method::Table,
        });
    }
    let table = truth_table(sequent, limits, &mut stop)?;
    let (verdict, countermodel) = match table.falsifying().first() {
        None => (super::Verdict::Valid(super::Evidence::Table), None),
        Some(&row) => {
            let model = table.model(row);
            let countermodel =
                super::Countermodel::found(sequent.clone(), model, logic, limits, &mut stop)?;
            (super::Verdict::NotValid, countermodel)
        }
    };
    Ok(super::Outcome {
        verdict,
        logic,
        search: super::Search::Table(Box::new(table)),
        countermodel,
        structure: None,
    })
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::{Logic, Sequent, truth_table};
    use crate::{Error, Limits, Refusal};

    /// The rows run from every atom true to every atom false, the first
    /// atom slowest; a valid sequent has no falsifying row, and one that
    /// is not valid has each of its falsifying rows marked; more atoms
    /// than the bound are refused, naming it.
    #[test]
    fn a_table_marks_the_rows_that_falsify_the_sequent() {
        let limits = Limits::default();
        let sequent: Sequent = "a -> b |- b -> a".parse().unwrap();
        let table = truth_table(&sequent, &limits, |_| false).unwrap();
        assert_eq!(table.rows(), 4);
        assert!(table.assigns(0, 0) && table.assigns(0, 1));
        assert!(table.assigns(1, 0) && !table.assigns(1, 1));
        // a false, b true: a → b true, b → a false.
        assert_eq!(table.falsifying(), [2]);
        assert!(table.falsifies(2) && !table.falsifies(1));
        assert_eq!(
            table
                .model(2)
                .refutes(&sequent, Logic::Classical, &limits, |_| false),
            Ok(())
        );
        let valid: Sequent = "|- a \\/ ~a".parse().unwrap();
        assert_eq!(
            table.to_string(),
            "a  b  │  a → b  ⊢  b → a\n\
             T  T  │    T         T\n\
             T  F  │    F         T\n\
             F  T  │    T         F    ×\n\
             F  F  │    T         T"
        );
        let valid = truth_table(&valid, &limits, |_| false).unwrap();
        assert_eq!(valid.falsifying(), [0u64; 0]);
        let three: Sequent = "a, b |- c".parse().unwrap();
        assert_eq!(
            truth_table(&three, &limits.with_table_atoms(Some(2)), |_| false).err(),
            Some(Error::Refused(Refusal::Atoms { atoms: 3, limit: 2 }))
        );
    }
}
