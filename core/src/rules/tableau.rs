// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A signed tableau of classical logic written as a derivation in the
//! rule file `rules::TABLEAU`.

use super::derivation::{Derivation, Node, Step};
use super::ordinary::terms_of;
use super::terms::TermId;
use crate::Error;
use crate::ordinary::{End, Rule, Sign, Tableau};

/// Returns the name in `rules::TABLEAU` of the expansion of a line by
/// `rule` of LK, or `None` for a rule no expansion has.
const fn expansion(rule: Rule) -> Option<&'static str> {
    use Rule::*;
    Some(match rule {
        AndLeft => "T∧",
        AndLeft1 => "T∧₁",
        AndLeft2 => "T∧₂",
        AndRight => "F∧",
        OrLeft => "T∨",
        OrRight => "F∨",
        OrRight1 => "F∨₁",
        OrRight2 => "F∨₂",
        ImpliesLeft => "T→",
        ImpliesRight => "F→",
        NotLeft => "T¬",
        NotRight => "F¬",
        IffLeft => "T↔",
        IffLeft1 => "T↔₁",
        IffLeft2 => "T↔₂",
        IffRight => "F↔",
        TrueLeft => "T⊤",
        FalseRight => "F⊥",
        Axiom | WeakenLeft | WeakenRight | ContractLeft | ContractRight | FalseLeft | TrueRight
        | Cut | Open => return None,
    })
}

impl Derivation {
    /// Returns a signed tableau as a derivation in the rule file
    /// [`TABLEAU`](super::TABLEAU): a node per node of the tableau,
    /// holding the signed formulas of its branch as a set, the leaves
    /// first and the root last. An expansion is the rule of its sign and
    /// connective, a closed branch `×`, `T⊥` or `F⊤`, and an open branch
    /// an open goal, which the check refuses.
    ///
    /// A node holds its whole branch, so the derivation holds as many
    /// formulas as the tableau's nodes times their depth.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidDerivation`] for an expansion by a rule no signed
    /// tableau has (a weakening, an axiom); [`Error::Refused`] past what
    /// the derivation's indices can name. Needs the cargo feature
    /// `interactive`.
    pub fn from_tableau(tableau: &Tableau) -> Result<Self, Error> {
        let invalid = |reason: String| Error::InvalidDerivation {
            calculus: "tableau".into(),
            reason,
        };
        let mut out = Self::new("tableau", 1);
        let formulas = terms_of(tableau.formulas(), out.terms_mut())?;
        // The term of each line, signed.
        let mut lines: Vec<TermId> = Vec::new();
        for node in tableau.nodes() {
            for line in node.lines() {
                let sign = match line.sign {
                    Sign::True => "T",
                    Sign::False => "F",
                };
                let signed = out
                    .terms_mut()
                    .apply(sign, &[formulas[line.formula.index()]])?;
                lines.push(signed);
            }
        }
        let nodes = tableau.nodes();
        // Each node's parent: a child comes after its parent.
        let mut parents = vec![None; nodes.len()];
        for (index, node) in nodes.iter().enumerate() {
            for &child in node.children() {
                parents[child as usize] = Some(index);
            }
        }
        // The derivation's nodes are the tableau's in reverse, so a
        // premise (a child) comes before its conclusion.
        let ids = |index: usize| u32::try_from(nodes.len() - 1 - index);
        for index in (0..nodes.len()).rev() {
            // The branch: the lines of the node and of every node above
            // it, each signed formula once.
            let mut branch: Vec<TermId> = Vec::new();
            let mut line_terms: Vec<(u32, TermId)> = Vec::new();
            let mut at = Some(index);
            while let Some(k) = at {
                let node = &nodes[k];
                for (offset, _) in node.lines().iter().enumerate() {
                    let line = node.first_line() as usize + offset;
                    line_terms.push((line as u32, lines[line]));
                }
                at = parents[k];
            }
            line_terms.sort_unstable_by_key(|&(line, _)| line);
            for &(_, term) in &line_terms {
                if !branch.contains(&term) {
                    branch.push(term);
                }
            }
            let place = |line: u32| -> Result<u32, Error> {
                let term = lines[line as usize];
                let at = branch
                    .iter()
                    .position(|&t| t == term)
                    .ok_or_else(|| invalid(format!("line {line} is on no branch of its node")))?;
                u32::try_from(at).map_err(|_| invalid("the branch is too long".to_owned()))
            };
            let node = &nodes[index];
            let (step, positions) = match (node.expands(), node.end()) {
                (Some((line, rule)), _) => {
                    let name = expansion(rule).ok_or_else(|| {
                        invalid(format!(
                            "node {index} expands by {rule}, which no tableau has"
                        ))
                    })?;
                    (Step::Rule(name), vec![place(line)?])
                }
                (
                    None,
                    Some(End::Closed {
                        true_line,
                        false_line,
                    }),
                ) => (Step::Rule("×"), vec![place(true_line)?, place(false_line)?]),
                (None, Some(End::Falsity { line })) => (Step::Rule("T⊥"), vec![place(line)?]),
                (None, Some(End::Truth { line })) => (Step::Rule("F⊤"), vec![place(line)?]),
                (None, _) => (Step::Open, Vec::new()),
            };
            let premises = node
                .children()
                .iter()
                .map(|&child| ids(child as usize))
                .collect::<Result<Vec<u32>, _>>()
                .map_err(|_| invalid("the tableau is too large".to_owned()))?;
            let sequent = [branch];
            out.push(
                Node::new(&sequent, step)
                    .premises(&premises)
                    .positions(&positions),
            )?;
        }
        Ok(out)
    }
}
