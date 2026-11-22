// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Equality as axioms. A sequent that uses `=` is decided with the
//! equality axioms of its symbols as extra hypotheses: reflexivity,
//! symmetry, transitivity, and for each function and predicate symbol
//! that equal arguments give equal values or the same truth. Every method
//! then decides it as it decides any first-order sequent, and a
//! derivation shows the axioms as hypotheses.
//!
//! The axioms hold in every structure that reads `=` as identity, in
//! classical, intuitionistic and minimal logic alike, so a sequent valid
//! with them is valid. A structure that satisfies them reads `=` as an
//! equivalence that its functions and predicates respect; dividing its
//! domain by that equivalence gives one where `=` is identity and every
//! formula keeps its truth value: grounding's structures are so divided
//! before they are checked.

use super::{Formulas, Node, NodeId, Sequent};
use crate::Error;
use crate::first_order::{EQUALS, FoTerm, FoTermId, Symbol};
use crate::limits::{Limits, Phase, Refusal};

/// Returns `sequent` with the equality axioms of its symbols that its
/// hypotheses lack appended after them, or `None` where the sequent has no
/// `=` or holds every axiom already. The roots keep their places, so a
/// sequent made by this call gives `None` when called again.
///
/// The axioms, each a closed formula of the sequent's arena:
/// - `∀x. x = x`, `∀x y. x = y → y = x` and `∀x y z. x = y ∧ y = z → x = z`;
/// - for each function symbol `f` of arity n ≥ 1, `∀x₁ … xₙ y₁ … yₙ.
///   x₁ = y₁ ∧ … ∧ xₙ = yₙ → f(x₁, …, xₙ) = f(y₁, …, yₙ)`;
/// - for each predicate symbol `p` of arity n ≥ 1 other than `=`, `∀x₁ …
///   xₙ y₁ … yₙ. x₁ = y₁ ∧ … ∧ xₙ = yₙ ∧ p(x₁, …, xₙ) → p(y₁, …, yₙ)`.
///
/// # Errors
///
/// [`Refusal::Occurrences`] where the axioms would make the arena larger
/// than `limits.occurrences`, and [`Refusal::Index`] when it is full.
pub fn with_axioms(sequent: &Sequent, limits: &Limits) -> Result<Option<Sequent>, Error> {
    if !sequent.formulas().has_equality() {
        return Ok(None);
    }
    let mut formulas = sequent.formulas().clone();
    let axioms = axioms(&mut formulas, limits)?;
    let mut left = sequent.left().to_vec();
    let before = left.len();
    // A set, since a sequent may have as many hypotheses as symbols.
    let mut held: crate::hash::HashSet<NodeId> = left.iter().copied().collect();
    for axiom in axioms {
        if held.insert(axiom) {
            left.push(axiom);
        }
    }
    if left.len() == before {
        return Ok(None);
    }
    Sequent::new(formulas, left, sequent.right().to_vec()).map(Some)
}

/// Returns how many of the last of the hypotheses `left`, formulas of
/// `formulas`, are equality axioms of its symbols, as [`with_axioms`]
/// appends them: `0` for an arena without `=`. A derivation's conclusion
/// (`Derivation::sequent`) and a sequent's hypotheses are such lists.
pub fn axiom_count(formulas: &Formulas, left: &[NodeId]) -> usize {
    if !formulas.has_equality() {
        return 0;
    }
    // Built in a copy: an axiom the arena lacks is no hypothesis of it.
    let mut copy = formulas.clone();
    let Ok(axioms) = axioms(&mut copy, &Limits::default().with_occurrences(None)) else {
        return 0;
    };
    let axioms: crate::hash::HashSet<NodeId> = axioms.into_iter().collect();
    left.iter()
        .rev()
        .take_while(|id| axioms.contains(id))
        .count()
}

/// Builds the equality axioms of the symbols of `formulas` into it and
/// returns them, reflexivity, symmetry and transitivity first, then one
/// per symbol in the order of the tables.
fn axioms(formulas: &mut Formulas, limits: &Limits) -> Result<Vec<NodeId>, Error> {
    // The symbols a term or an atom uses, each with its role: a JSON
    // sequent may declare others, which no formula needs an axiom for.
    let symbols: Vec<(Symbol, u32, bool)> = {
        let fo = formulas.interner();
        let tables = &fo.tables;
        let mut used = vec![None; tables.symbols.len()];
        for term in &tables.terms {
            if let FoTerm::App(f, _) = *term {
                used[f.index()] = Some(false);
            }
        }
        for &(p, _) in &tables.atoms {
            used[p.index()] = Some(true);
        }
        tables
            .symbols
            .iter()
            .zip(used)
            .enumerate()
            .filter_map(|(i, ((name, arity), predicate))| {
                let predicate = predicate?;
                (*arity > 0 && name != EQUALS).then_some((Symbol::new(i as u32), *arity, predicate))
            })
            .collect()
    };
    // A symbol of arity n makes about 4n + 4 nodes and 2n variables: the
    // whole is held to `u32::MAX` nodes before anything is built, so
    // `2 * arity` below cannot wrap. `limits.occurrences` bounds the
    // arena as it grows (`Build::within`), so axioms the arena holds
    // already cost nothing against it and a second call adds none.
    let most = u64::from(u32::MAX);
    let needed = symbols
        .iter()
        .fold(formulas.len() as u64 + 16, |sum, &(_, arity, _)| {
            sum.saturating_add(u64::from(arity).saturating_mul(4).saturating_add(4))
        });
    if needed > most {
        return Err(Error::Refused(Refusal::Occurrences {
            occurrences: needed,
            limit: most,
        }));
    }
    let start = formulas.len() as u64;
    let mut build = Build {
        formulas,
        limits,
        start,
    };
    let mut axioms = Vec::with_capacity(3 + symbols.len());
    // ∀x. x = x
    let x = build.variable(0);
    let body = build.equal(x, x)?;
    axioms.push(build.close(body, &["x"])?);
    // ∀x y. x = y → y = x
    let (x, y) = (build.variable(1), build.variable(0));
    let xy = build.equal(x, y)?;
    let yx = build.equal(y, x)?;
    let body = build.node(Node::Implies(xy, yx))?;
    axioms.push(build.close(body, &["x", "y"])?);
    // ∀x y z. x = y ∧ y = z → x = z
    let (x, y, z) = (build.variable(2), build.variable(1), build.variable(0));
    let xy = build.equal(x, y)?;
    let yz = build.equal(y, z)?;
    let xz = build.equal(x, z)?;
    let both = build.node(Node::And(xy, yz))?;
    let body = build.node(Node::Implies(both, xz))?;
    axioms.push(build.close(body, &["x", "y", "z"])?);
    for (symbol, arity, predicate) in symbols {
        axioms.push(build.congruence(symbol, arity, predicate)?);
    }
    Ok(axioms)
}

/// What one node the axioms add holds, its keys, names and table entries
/// with it: an axiom of a symbol of a million arguments held 967 MB for
/// its four million nodes.
const NODE_BYTES: u64 = 256;

/// The arena the axioms are built into, and the bounds on its size.
struct Build<'a> {
    /// The arena.
    formulas: &'a mut Formulas,
    /// The bounds on the arena's nodes and on the memory the axioms add.
    limits: &'a Limits,
    /// The arena's length before the axioms: only what they add is
    /// charged to the memory bound, so axioms it holds already cost
    /// nothing.
    start: u64,
}

impl Build<'_> {
    /// Returns the bound variable `index` binders out, as a term.
    fn variable(&mut self, index: u32) -> FoTermId {
        self.formulas.interner().term(FoTerm::Bound(index))
    }

    /// Returns `node`, added unless the arena has it, within the bound.
    fn node(&mut self, node: Node) -> Result<NodeId, Error> {
        let id = self.formulas.add(node)?;
        self.within(id)
    }

    /// Returns `id` where the arena keeps within the bound on occurrences,
    /// and what the axioms added within the memory bound, at
    /// [`NODE_BYTES`] a node.
    fn within(&self, id: NodeId) -> Result<NodeId, Error> {
        let len = self.formulas.len() as u64;
        if let Some(limit) = self.limits.occurrences
            && len > limit
        {
            return Err(Error::Refused(Refusal::Occurrences {
                occurrences: limit.saturating_add(1),
                limit,
            }));
        }
        let held = len.saturating_sub(self.start).saturating_mul(NODE_BYTES);
        if let Some(limit) = self.limits.memory_bytes
            && held > limit
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Read,
                limit_bytes: limit,
                needed_bytes: Some(held),
            }));
        }
        Ok(id)
    }

    /// Returns the atom `left = right`.
    fn equal(&mut self, left: FoTermId, right: FoTermId) -> Result<NodeId, Error> {
        let id = self.formulas.equality(left, right)?;
        self.within(id)
    }

    /// Returns `∀names. body`, the last name the innermost binder's.
    fn close(&mut self, mut body: NodeId, names: &[&str]) -> Result<NodeId, Error> {
        for name in names.iter().rev() {
            body = self.formulas.binder(true, body, name)?;
            body = self.within(body)?;
        }
        Ok(body)
    }

    /// Returns the congruence axiom of `symbol`, a predicate where
    /// `predicate` is set, else a function symbol, of `arity` ≥ 1.
    fn congruence(&mut self, symbol: Symbol, arity: u32, predicate: bool) -> Result<NodeId, Error> {
        // The axiom adds at least its 2n binders, unless the arena has it,
        // in which case a first call added them within the bound: so the
        // variables are not made where the binders cannot fit.
        let binders = u64::from(arity) * 2;
        if let Some(limit) = self.limits.memory_bytes
            && binders.saturating_mul(NODE_BYTES) > limit
        {
            return Err(Error::Refused(Refusal::Memory {
                phase: Phase::Read,
                limit_bytes: limit,
                needed_bytes: Some(binders.saturating_mul(NODE_BYTES)),
            }));
        }
        // Under the binders x₁ … xₙ y₁ … yₙ, yₙ innermost: xᵢ is the bound
        // variable 2n − i binders out, yᵢ the variable n − i.
        let xs: Vec<FoTermId> = (1..=arity).map(|i| self.variable(2 * arity - i)).collect();
        let ys: Vec<FoTermId> = (1..=arity).map(|i| self.variable(arity - i)).collect();
        let mut premise = None;
        for (&x, &y) in xs.iter().zip(&ys) {
            let equal = self.equal(x, y)?;
            premise = Some(match premise {
                None => equal,
                Some(before) => self.node(Node::And(before, equal))?,
            });
        }
        let premise = premise.expect("an arity of at least one");
        let (premise, conclusion) = if predicate {
            let before = self.formulas.applied_atom(symbol, &xs)?;
            let after = self.formulas.applied_atom(symbol, &ys)?;
            (self.node(Node::And(premise, before))?, after)
        } else {
            let fo = self.formulas.interner();
            let (lx, ly) = (fo.list(&xs), fo.list(&ys));
            let (fx, fy) = (
                fo.term(FoTerm::App(symbol, lx)),
                fo.term(FoTerm::App(symbol, ly)),
            );
            (premise, self.equal(fx, fy)?)
        };
        let body = self.node(Node::Implies(premise, conclusion))?;
        let names: Vec<String> = (1..=arity)
            .map(|i| format!("x{i}"))
            .chain((1..=arity).map(|i| format!("y{i}")))
            .collect();
        let names: Vec<&str> = names.iter().map(String::as_str).collect();
        self.close(body, &names)
    }
}

/// Returns `structure` with its domain divided by the predicate `=` it
/// lists: each class of elements its tuples make equal one element, in
/// the order of each class's least element; constants, function tables
/// and tuples mapped to their classes, and `=` dropped, since a structure
/// reads `=` as identity. A structure without `=` comes back as it is.
///
/// Where the structure satisfies the equality axioms of a sequent, its
/// `=` is an equivalence that every function and predicate respects, and
/// the quotient makes every formula of the sequent as true as the
/// structure does. Grounding's structures and the model search's are so
/// divided before they are checked.
#[cfg(feature = "sat")]
pub(super) fn identified(mut structure: super::Structure) -> super::Structure {
    let Some(at) = structure
        .predicates
        .iter()
        .position(|(name, _)| name == EQUALS)
    else {
        return structure;
    };
    let (_, pairs) = structure.predicates.remove(at);
    let size = structure.size as usize;
    // Union-find over the elements, each class rooted at its least.
    let mut parent: Vec<u32> = (0..structure.size).collect();
    let find = |parent: &mut Vec<u32>, mut e: u32| {
        while parent[e as usize] != e {
            let up = parent[parent[e as usize] as usize];
            parent[e as usize] = up;
            e = up;
        }
        e
    };
    for pair in &pairs {
        if let [a, b] = pair[..]
            && (a as usize) < size
            && (b as usize) < size
        {
            let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
            let (low, high) = (ra.min(rb), ra.max(rb));
            parent[high as usize] = low;
        }
    }
    // Each class's number, in the order of its root, its least element,
    // which stands for it.
    let mut class = vec![u32::MAX; size];
    let mut roots: Vec<u32> = Vec::new();
    for e in 0..structure.size {
        let root = find(&mut parent, e);
        if class[root as usize] == u32::MAX {
            class[root as usize] = roots.len() as u32;
            roots.push(root);
        }
        class[e as usize] = class[root as usize];
    }
    let classes = roots.len() as u32;
    let map = |e: u32| class.get(e as usize).copied().unwrap_or(e);
    // A function's table over the classes: each tuple of classes looked up
    // at the tuple of their roots, which a congruence makes the value of
    // every tuple of members. The table has `size^arity` entries, so the
    // arity is found by dividing its length; the new one has
    // `classes^arity`, no more.
    for (_, table) in &mut structure.functions {
        let (mut arity, mut rest) = (0u32, table.len());
        while rest > 1 && size > 1 && rest % size == 0 {
            rest /= size;
            arity += 1;
        }
        let entries = (classes as usize).pow(arity);
        let mut quotient = Vec::with_capacity(entries);
        for index in 0..entries {
            // The tuple of classes `index` names, the last digit least,
            // read at their roots in the old table's numbering.
            let (mut at, mut digits, mut place) = (0usize, index, 1usize);
            for _ in 0..arity {
                let c = digits % classes as usize;
                digits /= classes as usize;
                at += roots[c] as usize * place;
                place *= size;
            }
            quotient.push(map(table.get(at).copied().unwrap_or(0)));
        }
        *table = quotient;
    }
    for (_, element) in &mut structure.constants {
        *element = map(*element);
    }
    for (_, tuples) in &mut structure.predicates {
        for tuple in tuples.iter_mut() {
            for e in tuple.iter_mut() {
                *e = map(*e);
            }
        }
        tuples.sort_unstable();
        tuples.dedup();
    }
    structure.size = classes.max(1);
    structure
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::{axiom_count, with_axioms};
    use crate::Limits;
    use crate::ordinary::Sequent;

    /// The axioms of a sequent's symbols are appended once, read back as
    /// they print, and a second call adds none.
    #[test]
    fn the_axioms_are_appended_once() {
        let sequent: Sequent = "p(a), a = b |- p(f(b))".parse().unwrap();
        let limits = Limits::default();
        let with = with_axioms(&sequent, &limits).unwrap().unwrap();
        assert_eq!(
            with.to_string(),
            "p(a), a = b, ∀x. x = x, ∀x. ∀y. (x = y → y = x), \
             ∀x. ∀y. ∀z. ((x = y ∧ y = z) → x = z), ∀x1. ∀y1. ((x1 = y1 ∧ p(x1)) → p(y1)), \
             ∀x1. ∀y1. (x1 = y1 → f(x1) = f(y1)) ⊢ p(f(b))",
        );
        assert_eq!(axiom_count(with.formulas(), with.left()), 5);
        assert!(with_axioms(&with, &limits).unwrap().is_none());
        assert!(
            with_axioms(&"p(a) |- p(a)".parse().unwrap(), &limits)
                .unwrap()
                .is_none()
        );
    }

    /// In each logic a valid sequent with `=` gets a checked derivation
    /// whose conclusion holds the axioms as its last hypotheses, and one
    /// that is not valid a checked structure of identity, which lists no
    /// `=`.
    #[cfg(feature = "sat")]
    #[test]
    fn equality_is_decided_by_its_axioms() {
        use crate::linear::search;
        use crate::ordinary::{Evidence, Logic, Options, Verdict, decide_within};
        let (search, limits) = (search::Options::default(), Limits::default());
        for logic in Logic::ALL.iter().copied() {
            let options = Options::default().with_logic(logic);
            for valid in ["a = b, p(a) |- p(b)", "a = b |- f(b) = f(a)", "|- a = a"] {
                let sequent: Sequent = valid.parse().unwrap();
                let outcome = decide_within(&sequent, &options, &search, &limits, |_| false);
                let Ok(Verdict::Valid(Evidence::Derivation(derivation))) =
                    outcome.map(|o| o.verdict)
                else {
                    panic!("{valid} in {logic:?}");
                };
                derivation.check(&limits, |_| false).unwrap();
                let (left, _) = derivation.sequent();
                let count = axiom_count(derivation.formulas(), left);
                assert!(count >= 3, "{valid} in {logic:?}: {count} axioms");
            }
            for not_valid in ["a = b, p(a) |- p(c)", "|- a = b"] {
                let sequent: Sequent = not_valid.parse().unwrap();
                let outcome = decide_within(&sequent, &options, &search, &limits, |_| false)
                    .unwrap_or_else(|e| panic!("{not_valid}: {e}"));
                assert!(matches!(outcome.verdict, Verdict::NotValid), "{not_valid}");
                let structure = outcome.structure.expect("a structure");
                assert!(structure.structure.predicates.iter().all(|(p, _)| p != "="));
                structure.check(logic, &limits, |_| false).unwrap();
            }
        }
        // The model search's structure interprets `f`, divided by `=` too.
        // (The default without a time limit tries the connection prover
        // first, whose deepening with the axioms runs for minutes.)
        let sequent: Sequent = "f(a) = b |- f(b) = a".parse().unwrap();
        let found =
            crate::ordinary::models::decide(&sequent, Logic::Classical, &search, &limits, |_| {
                false
            });
        let structure = found.unwrap().structure.expect("a structure");
        assert!(
            matches!(structure.structure.functions[..], [(ref f, _)] if f == "f"),
            "f is interpreted"
        );
        structure
            .check(Logic::Classical, &limits, |_| false)
            .unwrap();
        // The methods called directly add the axioms themselves.
        let sequent: Sequent = "|- a = a".parse().unwrap();
        let grounded =
            crate::ordinary::grounding::decide(&sequent, Logic::Classical, &limits, |_| false);
        assert!(matches!(
            grounded.unwrap().unwrap().verdict,
            Verdict::Valid(_)
        ));
        let connected = crate::ordinary::connection::decide(
            &sequent,
            Logic::Classical,
            &search,
            &limits,
            |_| false,
        );
        assert!(matches!(connected.unwrap().verdict, Verdict::Valid(_)));
        // A sequent augmented already, as the command hands it over,
        // whose structure a bound leaves unchecked, is unknown.
        let sequent: Sequent = "a = b, p(a) |- p(c)".parse().unwrap();
        let with = with_axioms(&sequent, &limits).unwrap().unwrap();
        let small = limits.with_work(Some(20));
        let options = Options::default().with_logic(Logic::Classical);
        let outcome = decide_within(&with, &options, &search, &small, |_| false).unwrap();
        assert!(!matches!(outcome.verdict, Verdict::NotValid) || outcome.structure.is_some());
    }
}
