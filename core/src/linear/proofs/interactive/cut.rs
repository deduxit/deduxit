// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The cut in a session. The caller names the cut formula; it joins the
//! session's sequent with its dual, and the cut's two premises open as
//! goals.

use super::quantified::Quantified;
use super::{GoalId, Interactive, StepError};
use crate::Error;
use crate::limits::Limits;
#[cfg(feature = "parse")]
use crate::linear::fragment::Mode;
use crate::linear::occurrences::{
    ClassicalSequent, Forest, IllSequent, Member, OccId, Reading, Side, Written,
};
use crate::linear::proofs::Rule;
use crate::linear::proofs::derivation::{InfId, Inference};
use crate::linear::proofs::multiset::Multiset;
use crate::linear::sequents::Sequent;

/// A cut formula as it was written, for [`Interactive::cut`].
///
/// It holds the formula, one-sided, and what its syntax records of its
/// occurrences. The cut adds the formula twice: right of `⊢` in the cut's
/// first premise and left of it in the second. The session keeps the
/// record for both, so that every inference of the proof prints the
/// formula as written.
///
/// Read one from text with [`parse_within`](Self::parse_within), in the
/// syntax of the session's mode, or convert a sequent `⊢ C`: an
/// [`IllSequent`] (with its recorded sides), a [`ClassicalSequent`] (with
/// how it was written) or a one-sided [`Sequent`] (written one-sided).
///
/// Needs the cargo feature `interactive` (on by default).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CutFormula {
    /// The formula, as the one root of a sequent of its own.
    formula: Sequent,
    /// The intuitionistic sides of its occurrences, with the formula in
    /// output position, when it was written intuitionistically.
    sides: Option<Box<[Side]>>,
    /// How its occurrences were written, with the formula right of `⊢`,
    /// when it was written as classical text.
    writing: Option<Box<[Written]>>,
}

impl CutFormula {
    /// Reads a cut formula from text in the syntax of `mode`.
    ///
    /// In an intuitionistic mode the text is an intuitionistic formula,
    /// read strictly or one-sided as `input` says, as the goal of `⊢ C`.
    /// In any other mode it is a classical formula, kept as written. A
    /// parse error points into `text`.
    ///
    /// Needs the cargo feature `parse` (on by default).
    ///
    /// # Errors
    ///
    /// [`Error::Parse`] for text that is not a formula,
    /// [`Error::CutFormula`] for text with no formula or more than one,
    /// [`Error::NotIntuitionistic`] in an intuitionistic mode for a
    /// formula that cannot be a goal, and [`Error::Refused`] past
    /// `limits.occurrences`.
    #[cfg(feature = "parse")]
    pub fn parse_within(
        text: &str,
        mode: Mode,
        input: &crate::linear::InputOptions,
        limits: &Limits,
    ) -> Result<Self, Error> {
        /// What the text is read after, so that it is read as a goal.
        const TURNSTILE: &str = "|- ";
        let written = format!("{TURNSTILE}{text}");
        let shift = |e: Error| {
            let Error::Parse(mut e) = e else {
                return e;
            };
            let back = TURNSTILE.len();
            e.span = e.span.start.saturating_sub(back)..e.span.end.saturating_sub(back);
            e.span_utf16 =
                e.span_utf16.start.saturating_sub(back)..e.span_utf16.end.saturating_sub(back);
            if e.line == 1 {
                e.column = e.column.saturating_sub(back).max(1);
            }
            Error::Parse(e)
        };
        let formula = if mode.is_intuitionistic() {
            let ill = match input.intuitionistic {
                crate::linear::IllSyntax::OneSided => {
                    let sequent = Sequent::parse_within(&written, limits).map_err(shift)?;
                    IllSequent::read_one_sided(sequent, limits)?
                }
                crate::linear::IllSyntax::Strict => {
                    IllSequent::parse_within(&written, limits).map_err(shift)?
                }
            };
            Self::from(ill)
        } else {
            Self::from(ClassicalSequent::parse_within(&written, limits).map_err(shift)?)
        };
        match formula.formula.roots().len() {
            1 => Ok(formula),
            formulas => Err(Error::CutFormula { formulas }),
        }
    }

    /// Returns the formula, as the one root of a sequent of its own.
    pub fn sequent(&self) -> &Sequent {
        &self.formula
    }

    /// Returns the formula with its constants `names` bound by one `∀`
    /// each around it, the first outermost.
    ///
    /// The formula is read again from its one-sided text, which every
    /// printer writes so that it reads back; a name that an inner binder
    /// also takes is written apart. Its records of how it was written get
    /// one entry per binder in front, on the formula's side.
    ///
    /// # Errors
    ///
    /// [`Error::Refused`] past `limits.occurrences`.
    #[cfg(feature = "parse")]
    fn bound(&self, names: &[&str], limits: &Limits) -> Result<Self, Error> {
        let forest = Forest::within(&self.formula, &Limits::UNBOUNDED)?;
        let text = format!(
            "|- forall {}. ({})",
            names.join(" "),
            forest.formula(forest.roots()[0])
        );
        let formula = Sequent::parse_within(&text, limits)?;
        // The binders come first, each on the side the formula is on.
        let k = names.len();
        let sides = self.sides.as_ref().map(|sides| {
            let mut bound = vec![sides[0]; k];
            bound.extend_from_slice(sides);
            bound.into_boxed_slice()
        });
        let writing = self.writing.as_ref().map(|writing| {
            let mut bound = vec![Written::new(writing[0].side(), false); k];
            bound.extend_from_slice(writing);
            bound.into_boxed_slice()
        });
        Ok(Self {
            formula,
            sides,
            writing,
        })
    }

    /// Returns the formula unchanged: without the text reader, no name is
    /// taken for an eigenvariable, so there is nothing to bind.
    #[cfg(not(feature = "parse"))]
    fn bound(&self, _: &[&str], _: &Limits) -> Result<Self, Error> {
        Ok(self.clone())
    }
}

impl From<IllSequent> for CutFormula {
    /// Returns the cut formula of an intuitionistic sequent `⊢ C`, with its
    /// sides as written.
    fn from(sequent: IllSequent) -> Self {
        let reading = sequent.reading();
        let sides = sequent
            .forest()
            .ids()
            .map(|o| reading.position(o))
            .collect();
        Self {
            formula: sequent.into_sequent(),
            sides: Some(sides),
            writing: None,
        }
    }
}

impl From<ClassicalSequent> for CutFormula {
    /// Returns the cut formula of a classical sequent `⊢ C`, as it was
    /// written.
    fn from(sequent: ClassicalSequent) -> Self {
        let writing = sequent.writing().into();
        Self {
            formula: sequent.into_sequent(),
            sides: None,
            writing: Some(writing),
        }
    }
}

impl From<Sequent> for CutFormula {
    /// Returns the cut formula of a one-sided sequent `⊢ C`, written
    /// one-sided.
    fn from(formula: Sequent) -> Self {
        Self {
            formula,
            sides: None,
            writing: None,
        }
    }
}

impl Interactive {
    /// Cuts an open goal `Γ, Δ` on the formula `A` that `formula` names,
    /// and returns the two goals the cut opens.
    ///
    /// The goals are the premises `Γ, A` and `Δ, A⊥`, in that order: the
    /// formulas at the positions `left` go to the first, the others to the
    /// second. Intuitionistically they are `Γ ⊢ A` and `Δ, A ⊢ C`, so the
    /// goal's succedent `C` goes to the second premise.
    ///
    /// `A` and its dual join the session's sequent as a cut formula
    /// ([`Sequent::cuts`]). Their occurrences are recorded as `formula` was
    /// written, `A` right of `⊢` and `A⊥` left of it, so that the two-sided
    /// views print them as written. [`undo`](Self::undo) takes the cut
    /// back, and its formula with it.
    ///
    /// An ordered mode has no cut, since there the dual of a formula turns
    /// it round. In a session with quantifiers, `A` is a closed formula of
    /// the sequent's symbols and the goal's eigenvariables, and may have
    /// quantifiers of its own; the goals' instances keep their terms. A cut
    /// formula with a quantifier makes a session without one a session with
    /// quantifiers.
    ///
    /// # Errors
    ///
    /// Nothing changes on an error. [`Error::Step`] with:
    /// - [`StepError::NoGoal`] for a goal that is not open;
    /// - [`StepError::Mode`] in an ordered mode;
    /// - [`StepError::Split`] for a position past the goal or named twice;
    /// - [`StepError::Succedents`] in intuitionistic mode where a premise
    ///   would not have exactly one succedent (the formulas going left must
    ///   all be hypotheses);
    /// - [`StepError::CutSymbol`] for a cut formula naming a symbol the
    ///   sequent has with another arity or role, or, in a session with
    ///   quantifiers, one the sequent does not have and that is not an
    ///   eigenvariable's letter used as a constant;
    /// - [`StepError::Eigenvariable`] for an eigenvariable outside the
    ///   goal's scope.
    ///
    /// Also [`Error::CutFormula`] for a cut formula with no formula or more
    /// than one, [`Error::NotIntuitionistic`] in intuitionistic mode
    /// for a one-sided formula that cannot be a goal, and
    /// [`Error::Refused`] when the sequent with the cut formula has more
    /// occurrences than `limits.occurrences`.
    pub fn cut(
        &mut self,
        goal: GoalId,
        formula: &CutFormula,
        left: &[usize],
        limits: &Limits,
    ) -> Result<[GoalId; 2], Error> {
        let sequent = self.goal(goal)?.to_vec();
        if self.mode.is_ordered() {
            return Err(StepError::Mode {
                rule: Rule::Cut.into(),
                mode: self.mode,
            }
            .into());
        }
        let mut used = vec![false; sequent.len()];
        for &p in left {
            if p >= sequent.len() || std::mem::replace(&mut used[p], true) {
                return Err(StepError::Split { position: p }.into());
            }
        }
        let named = if self.forest.sequent().is_first_order() || formula.formula.is_first_order() {
            self.cut_symbols(goal, &formula.formula)?
        } else {
            Vec::new()
        };
        // A cut formula naming eigenvariables stays closed: it is bound by
        // one `∀` per eigenvariable, and the cut is on its body's instance.
        let bound;
        let formula = if named.is_empty() {
            formula
        } else {
            let names: Vec<&str> = named.iter().map(|(name, _)| name.as_str()).collect();
            bound = formula.bound(&names, limits)?;
            &bound
        };
        let depth = named.len() as u32;
        let mut extended = self.forest.sequent().clone();
        extended.add_cut(formula.formula.clone())?;
        let forest = Forest::within(&extended, limits)?;
        // The new trees follow every occurrence the session had. The cast
        // is exact: a forest has fewer than `u32::MAX` occurrences.
        let a = OccId::new(self.forest.len() as u32);
        let size = forest.size(a) as usize;
        let dual = OccId::new(a.get() + forest.size(a));
        debug_assert_eq!(forest.cut_pairs().last(), Some((a, dual)));
        let reading = match &self.reading {
            Some((positions, goal)) => {
                let sides = match &formula.sides {
                    Some(sides) => sides.clone(),
                    None => {
                        let ill = IllSequent::read_one_sided(formula.formula.clone(), limits)?;
                        let reading = ill.reading();
                        ill.forest().ids().map(|o| reading.position(o)).collect()
                    }
                };
                let mut extended = positions.to_vec();
                extended.extend_from_slice(&sides);
                extended.extend(sides.iter().map(|&side| !side));
                debug_assert!(
                    Reading::of_mode(&forest, self.mode)
                        .is_ok_and(|r| forest.ids().all(|o| r.position(o) == extended[o.index()])),
                    "the sides recorded as written are the reading's"
                );
                Some((extended.into_boxed_slice(), *goal))
            }
            None => None,
        };
        let writing = match &self.writing {
            None => None,
            Some(writing) => Some({
                let record = match &formula.writing {
                    Some(record) => record.clone(),
                    // Unbounded is safe: the formula has fewer occurrences
                    // than the forest just built within the limits.
                    None => crate::linear::occurrences::written::one_sided(&Forest::within(
                        &formula.formula,
                        &Limits::UNBOUNDED,
                    )?),
                };
                let mut extended = writing.to_vec();
                extended.extend_from_slice(&record);
                // The dual is written left of `⊢`, so each of its
                // occurrences takes the other side.
                extended.extend(
                    record
                        .iter()
                        .map(|w| Written::new(!w.side(), w.is_implication())),
                );
                extended.into_boxed_slice()
            }),
        };
        debug_assert_eq!(forest.len(), self.forest.len() + 2 * size);
        // The members past the forest move past the cut formulas' trees.
        let (from, to) = (self.forest.len() as u32, forest.len() as u32);
        let shift = |m: Member| {
            if m.get() < from {
                m
            } else {
                Member::new(m.get() - from + to)
            }
        };
        let premises = {
            let positions = reading.as_ref().map(|(p, _)| &**p);
            let shifted: Vec<Member> = sequent.iter().map(|&m| shift(m)).collect();
            // A member below `to` is an occurrence of the new forest, the
            // cut formula's among them; one past it is a moved instance.
            let occurrence = |m: Member| {
                if m.get() < to {
                    m.occ()
                } else {
                    self.occurrence(Member::new(m.get() - to + from))
                }
            };
            // The bodies' occurrences stand for their instances here.
            cut_premises(
                positions,
                &shifted,
                occurrence,
                Member::new(a.get() + depth),
                Member::new(dual.get() + depth),
                &used,
            )?
        };
        // Pairs of a body's occurrence and the instance the cut is on.
        let mut instances: Vec<(Member, Member)> = Vec::new();
        match self.fo.as_mut() {
            Some(q) => {
                let mark = q.mark();
                q.rebase(&self.forest, &forest);
                for inference in &mut self.inferences {
                    for m in &mut inference.sequent {
                        *m = shift(*m);
                    }
                    // An earlier cut on an instance names a member past
                    // the forest too.
                    inference.cut = inference.cut.map(shift);
                }
                if !named.is_empty() {
                    let mut frame = None;
                    for &(_, e) in &named {
                        let t = q.eigen_term(e);
                        frame = Some(q.frame(frame, t));
                    }
                    for o in [a, dual] {
                        let o = OccId::new(o.get() + depth);
                        instances.push((Member::from(o), q.member(o, frame)));
                    }
                    // The cast is exact: there are fewer steps than
                    // inferences, which an `InfId` numbers.
                    q.made(self.history.len() as u32, mark);
                }
            }
            // A cut formula with a quantifier makes the session one with
            // quantifiers. It had no member past its forest, so nothing
            // moves.
            None if forest.has_binders() => {
                let mut q = Quantified::new(&forest);
                q.limit = limits.terms;
                self.fo = Some(Box::new(q));
            }
            None => {}
        }
        self.forest = forest;
        self.reading = reading;
        self.writing = writing;
        let premises = premises.map(|premise| {
            let mut premise = premise.into_members();
            if !instances.is_empty() {
                for m in &mut premise {
                    if let Some(&(_, instance)) = instances.iter().find(|(o, _)| o == m) {
                        *m = instance;
                    }
                }
                premise.sort_unstable();
            }
            premise
        });
        let cut = match instances.first() {
            Some(&(_, instance)) => {
                self.show();
                instance
            }
            None => Member::from(a),
        };
        let base = self.inferences.len() as u32;
        let ids = [InfId::new(base), InfId::new(base + 1)];
        for premise in premises {
            self.inferences.push(Inference {
                sequent: premise,
                rule: Rule::Open.into(),
                principal: None,
                premises: vec![],
                times: 1,
                cut: None,
                node: None,
            });
        }
        let inference = &mut self.inferences[goal.index()];
        inference.rule = Rule::Cut.into();
        inference.principal = None;
        inference.premises = ids.to_vec();
        inference.cut = Some(cut);
        self.history.push(goal.inf());
        Ok(ids.map(GoalId::of))
    }

    /// Returns `cut` and its dual's member, or `None` when `cut` is not a
    /// member a cut can be on.
    ///
    /// A plain cut is on a cut formula's root, whose dual is the root of
    /// the dual formula. A cut on an instance is on the body below a cut
    /// formula's `∀`s, under a frame of one term per `∀`; its dual is the
    /// dual's body under the same frame.
    pub(super) fn cut_pair(&self, cut: Member) -> Option<(Member, Member)> {
        let (o, frame) = match &self.fo {
            Some(q) => q.instance(cut),
            None => (cut.occurrence(&self.forest)?, None),
        };
        let root = self.forest.root(o);
        let (_, dual) = self.forest.cut_pairs().find(|&(a, _)| a == root)?;
        let Some(frame) = frame else {
            return (o == root).then_some((cut, Member::from(dual)));
        };
        let q = self.fo.as_ref()?;
        // A frame is as long as its occurrence's binders.
        let depth = q.frame_terms(Some(frame)).len() as u32;
        let binders = (root.get()..root.get() + depth)
            .all(|b| self.forest.kind(OccId::new(b)) == crate::linear::sequents::Kind::Forall);
        (o.get() == root.get() + depth && binders)
            .then(|| q.find(OccId::new(dual.get() + depth), Some(frame)))
            .flatten()
            .map(|d| (cut, d))
    }

    /// Returns the eigenvariables a cut formula on `goal` names by their
    /// letters, each with its name, in the order of the formula's symbols.
    ///
    /// It refuses:
    /// - an eigenvariable outside the goal's scope, as for a witness;
    /// - a symbol the sequent has with another arity or role, which
    ///   joining the two sequents would rename apart;
    /// - in a session with quantifiers, any other symbol the sequent does
    ///   not have. The eigenvariables take the letters no symbol takes, so
    ///   a new symbol could take an eigenvariable's letter.
    fn cut_symbols(
        &self,
        goal: GoalId,
        formula: &Sequent,
    ) -> Result<Vec<(String, crate::linear::sequents::Eigen)>, StepError> {
        let ours: crate::hash::HashMap<&str, (u32, bool)> =
            symbol_roles(self.forest.sequent()).into_iter().collect();
        // An eigenvariable's letter reads as a constant: a nullary symbol
        // that is not a predicate. The text is read without the session,
        // so the letters exist only with the text reader.
        let letters: crate::hash::HashMap<String, crate::linear::sequents::Eigen> = match &self.fo {
            Some(q) if cfg!(feature = "parse") => {
                crate::linear::occurrences::instances::eigen_names(
                    &self.forest.sequent().fo,
                    &q.shown.eigenvariables(),
                )
                .into_iter()
                .map(|(e, name)| (name, e))
                .collect()
            }
            _ => crate::hash::HashMap::default(),
        };
        let scope = if letters.is_empty() {
            Vec::new()
        } else {
            self.scope(goal.inf())
        };
        let mut named = Vec::new();
        for (name, role) in symbol_roles(formula) {
            let known = ours.get(name);
            if known.is_none()
                && role == (0, false)
                && let Some(&e) = letters.get(name)
            {
                if !scope.contains(&e) {
                    return Err(StepError::Eigenvariable {
                        name: name.to_owned(),
                    });
                }
                named.push((name.to_owned(), e));
            } else if known.is_some_and(|&r| r != role) || (known.is_none() && self.fo.is_some()) {
                return Err(StepError::CutSymbol {
                    name: name.to_owned(),
                });
            }
        }
        Ok(named)
    }

    /// Takes back the cut formula of the cut that [`undo`](Self::undo)
    /// takes back. It is the last one added, since the steps after the cut
    /// were taken back first.
    pub(super) fn uncut(&mut self) {
        let mut sequent = self.forest.sequent().clone();
        // The arena keeps every term the table still names. A session with
        // quantifiers also keeps every symbol: its cuts add none, and a
        // symbol no atom uses still takes a letter from the eigenvariables.
        let reach = self.fo.as_ref().map_or(0, |q| q.arena_reach());
        sequent.pop_cut_keeping(reach, self.fo.is_some());
        // Unbounded is safe: the forest is smaller than the one the
        // session had.
        let forest = Forest::within(&sequent, &Limits::UNBOUNDED).expect("a smaller forest");
        if !forest.has_binders() {
            self.fo = None;
        }
        if let Some(q) = self.fo.as_mut() {
            q.rebase(&self.forest, &forest);
            // The members past the forest move back over the cut formula's
            // trees, which no inference left names.
            let (from, to) = (self.forest.len() as u32, forest.len() as u32);
            let back = |m: Member| {
                if m.get() >= from {
                    Member::new(m.get() - from + to)
                } else {
                    m
                }
            };
            for inference in &mut self.inferences {
                for m in &mut inference.sequent {
                    *m = back(*m);
                }
                inference.cut = inference.cut.map(back);
            }
        }
        self.forest = forest;
        let len = self.forest.len();
        if let Some((positions, goal)) = &mut self.reading {
            *positions = positions[..len].into();
            debug_assert!(goal.index() < len);
        }
        if let Some(writing) = &mut self.writing {
            *writing = writing[..len].into();
        }
    }

    /// Checks a cut read back: it names a cut formula of the sequent, and
    /// its premises are what the cut yields on its conclusion, with the
    /// split recovered from the first premise.
    #[cfg(feature = "serialize")]
    pub(super) fn replay_cut(&self, id: InfId) -> Result<(), Error> {
        let inconsistent = |reason| Error::InconsistentSession { reason };
        let inference = &self.inferences[id.index()];
        let sequent = &inference.sequent;
        let Some((a, dual)) = inference.cut.and_then(|a| self.cut_pair(a)) else {
            return Err(inconsistent(
                "a cut names no cut formula, nor the instance of one's body that a session cuts on",
            ));
        };
        // A cut on an instance holds only eigenvariables in the goal's
        // scope, as a given witness does.
        if let Some(q) = self.fo.as_ref()
            && let (_, Some(frame)) = q.instance(a)
        {
            let scope = self.scope(id);
            let terms = q.terms(&self.forest);
            let held = q.frame_terms(Some(frame)).into_iter().all(|t| {
                matches!(terms.term(t), crate::linear::sequents::FoTerm::Eigen(e) if scope.contains(&e))
            });
            if !held {
                return Err(inconsistent(
                    "a cut's instance holds a term that is no eigenvariable in its goal's scope",
                ));
            }
        }
        if self.mode.is_ordered() {
            return Err(inconsistent("an ordered mode has no cut"));
        }
        let &[first, _] = inference.premises.as_slice() else {
            return Err(inconsistent("a cut has two premises"));
        };
        let mut context = Multiset::of(self.inferences[first.index()].sequent.iter().copied());
        if !context.remove(a) {
            return Err(inconsistent("a cut's first premise lacks its cut formula"));
        }
        // Both are sorted, so each member of the context is matched with
        // the next equal one of the sequent, in linear time.
        let mut used = vec![false; sequent.len()];
        let mut at = 0;
        for &o in context.as_slice() {
            while at < sequent.len() && sequent[at] < o {
                at += 1;
            }
            if at == sequent.len() || sequent[at] != o {
                return Err(inconsistent(
                    "a premise holds a formula its conclusion lacks",
                ));
            }
            used[at] = true;
            at += 1;
        }
        let positions = self.reading.as_ref().map(|(p, _)| &**p);
        let expected = cut_premises(positions, sequent, |m| self.occurrence(m), a, dual, &used)?;
        let same = expected.iter().zip(&inference.premises).all(|(e, p)| {
            e.as_slice()
                .iter()
                .copied()
                .eq(self.inferences[p.index()].sequent.iter().copied())
        });
        if same {
            Ok(())
        } else {
            Err(inconsistent(
                "an inference's premises are not what its rule yields",
            ))
        }
    }
}

/// Returns every symbol of a sequent in the order of its table, with its
/// arity and whether it is a predicate. A sequent without first-order
/// atoms gives its atoms as nullary predicates.
fn symbol_roles(sequent: &Sequent) -> Vec<(&str, (u32, bool))> {
    let fo = &sequent.fo;
    let mut roles: Vec<(&str, (u32, bool))> = fo
        .symbols
        .iter()
        .map(|(name, arity)| (name.as_str(), (*arity, false)))
        .collect();
    for &(p, _) in &fo.atoms {
        roles[p.index()].1.1 = true;
    }
    if fo.atoms.is_empty() {
        roles.extend(
            sequent
                .atom_names()
                .iter()
                .map(|name| (name.as_str(), (0, true))),
        );
    }
    roles
}

/// Returns the premises of a cut on `a`, whose dual is `dual`, of the goal
/// `sequent`. The formulas marked in `left` go to the first premise.
///
/// In intuitionistic mode, `positions` gives the side of each member's
/// occurrence, and each premise must have exactly one succedent.
fn cut_premises(
    positions: Option<&[Side]>,
    sequent: &[Member],
    occurrence: impl Fn(Member) -> OccId,
    a: Member,
    dual: Member,
    left: &[bool],
) -> Result<[Multiset<Member>; 2], StepError> {
    let (mut first, mut second) = (Multiset::new(), Multiset::new());
    for (&o, &goes_left) in sequent.iter().zip(left) {
        if goes_left {
            first.insert(o);
        } else {
            second.insert(o);
        }
    }
    first.insert(a);
    second.insert(dual);
    if let Some(positions) = positions {
        for premise in [&first, &second] {
            let outputs = premise
                .as_slice()
                .iter()
                .filter(|&&m| positions[occurrence(m).index()] == Side::Output)
                .count();
            if outputs != 1 {
                return Err(StepError::Succedents { count: outputs });
            }
        }
    }
    Ok([first, second])
}

/// Returns proofs with cuts, each with its mode, made from `proofs`.
///
/// For each proof in a commutative mode, a session of its sequent is cut
/// on one of its formulas `C`. The other formulas go to the premise that
/// proves `C`; intuitionistically, where `C` is a hypothesis, the first
/// premise is `C ⊢ C`. Then one of the two goals is cut again on one of
/// its formulas, which may be the first cut's, and the search closes every
/// goal.
#[cfg(all(test, feature = "parse"))]
pub(crate) fn with_cuts(
    proofs: &[(crate::linear::Proof, Mode)],
    rng: &mut crate::linear::search::generate::Rng,
) -> Vec<(crate::linear::Proof, Mode)> {
    let limits = Limits::default();
    let options = crate::linear::Options::default();
    let view = crate::linear::ViewOptions::default();
    let mut made = Vec::new();
    for (proof, mode) in proofs {
        if mode.is_ordered() || proof.goal().is_some() {
            continue;
        }
        let Ok(mut session) = Interactive::new(proof.sequent(), *mode) else {
            continue;
        };
        let mut goal = session.goals().next().expect("the root is open");
        for _ in 0..2 {
            let members = session.goal(goal).expect("open").to_vec();
            let p = rng.below(members.len());
            let o = members[p].occ();
            let (text, left): (String, Vec<usize>) = match session.reading() {
                Some(reading) if reading.position(o) == Side::Input => {
                    (reading.formula(o).to_string(), vec![p])
                }
                reading => {
                    let text = match reading {
                        Some(reading) => reading.formula(o).to_string(),
                        None => session.forest().formula(o).to_string(),
                    };
                    (text, (0..members.len()).filter(|&q| q != p).collect())
                }
            };
            let formula = CutFormula::parse_within(
                &text,
                *mode,
                &crate::linear::InputOptions::default(),
                &limits,
            )
            .expect("a formula as the session prints it");
            let premises = session
                .cut(goal, &formula, &left, &limits)
                .expect("a cut that leaves one goal per premise");
            goal = premises[rng.below(2)];
        }
        let mut polls = 0;
        let closed = session.close_all(&options, &view, &limits, |_| {
            polls += 1;
            polls > 20_000
        });
        if closed.iter().all(|(_, c)| {
            c.as_ref()
                .is_ok_and(|c| c.grafted.is_ok() && c.outcome.verdict.proof().is_some())
        }) && let Ok(proof) = session.proof(&limits, |_| false)
        {
            made.push((proof, *mode));
        }
    }
    made
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::*;
    use crate::linear::proofs::{Named, Node};
    use crate::linear::{InputOptions, Options, TwoSided, ViewOptions};

    /// Reads a cut formula of `mode` with the default input options.
    fn formula(text: &str, mode: Mode) -> Result<CutFormula, Error> {
        CutFormula::parse_within(text, mode, &InputOptions::default(), &Limits::default())
    }

    /// A cut opens its two premises as goals, the cut formula right of `⊢`
    /// in the first and left of it in the second, as typed. Then:
    /// - the finished proof has the cut and checks;
    /// - the two-sided view names it `cut`;
    /// - the session writes its formula as typed;
    /// - a session started on its sequent starts from the conclusion, and
    ///   the same cut made again gives the same sequent;
    /// - `undo` takes the cut back with its formula.
    #[test]
    fn a_cut_opens_two_goals_and_undo_takes_it_back() {
        let limits = Limits::default();
        let written: ClassicalSequent = "A, A -o B |- B".parse().unwrap();
        let mut session = Interactive::of_classical(&written, Mode::CLASSICAL, &limits).unwrap();
        let before = state(&session);
        let root = session.goals().next().unwrap();
        let [first, second] = session
            .cut(
                root,
                &formula("A -o B", Mode::CLASSICAL).unwrap(),
                &[1],
                &limits,
            )
            .unwrap();
        let shown = |g| -> Vec<String> {
            let goal = session.goal(g).unwrap();
            goal.iter()
                .map(|&m| session.forest().formula(m.occ()).to_string())
                .collect()
        };
        assert_eq!(shown(first), ["A ⊗ ~B", "~A ⅋ B"]);
        assert_eq!(shown(second), ["~A", "B", "A ⊗ ~B"]);
        session.close_all(
            &Options::default(),
            &ViewOptions::default(),
            &limits,
            |_| false,
        );
        let proof = session.proof(&limits, |_| false).unwrap();
        assert_eq!(
            proof
                .nodes()
                .iter()
                .filter(|n| matches!(n, Node::Cut(..)))
                .count(),
            1
        );
        assert_eq!(proof.check(Mode::CLASSICAL), Ok(()));
        let derivation = session.derivation().unwrap();
        let cut = derivation.inference(derivation.root());
        assert_eq!(cut.rule(), Named::from(Rule::Cut));
        let view = TwoSided::new(&derivation, session.writing().unwrap(), &limits).unwrap();
        let root = view.root();
        assert_eq!(view.rule(root).name(), "cut");
        let premises: Vec<String> = view
            .premises(root)
            .iter()
            .map(|&p| view.sequent(p).to_string())
            .collect();
        assert_eq!(premises, ["A ⊸ B ⊢ A ⊸ B", "A, A ⊸ B ⊢ B"]);
        let typed = session
            .formula_as_written(cut.cut().unwrap())
            .unwrap()
            .to_string();
        assert_eq!(typed, "A ⊸ B");
        let mut again = Interactive::within(session.sequent(), Mode::CLASSICAL, &limits).unwrap();
        assert!(!again.forest().has_cuts());
        assert_eq!(state(&again), before);
        let root = again.goals().next().unwrap();
        let typed = formula(&typed, Mode::CLASSICAL).unwrap();
        again.cut(root, &typed, &[1], &limits).unwrap();
        assert_eq!(again.sequent(), session.sequent());
        while session.undo().is_some() {}
        assert_eq!(state(&session), before);
        assert!(!session.forest().has_cuts());
    }

    /// What a test compares sessions by: the sequent, the forest's size
    /// and the goals.
    fn state(session: &Interactive) -> String {
        let goals: Vec<_> = session
            .goals()
            .map(|g| session.goal(g).unwrap().to_vec())
            .collect();
        format!(
            "{:?} {} {goals:?}",
            session.sequent(),
            session.forest().len()
        )
    }

    /// A cut is refused, and nothing changes, for a goal that is not
    /// open, in an ordered mode, for a split past the goal or naming a
    /// formula twice, where a premise would have two succedents or none,
    /// and for text with no formula or more than one. A parse error points
    /// into the text typed.
    #[test]
    fn refusals() {
        let limits = Limits::default();
        let classical = formula("A", Mode::CLASSICAL).unwrap();
        let sequent: Sequent = "A |- A".parse().unwrap();
        let mut session = Interactive::new(&sequent, Mode::INTUITIONISTIC).unwrap();
        let root = session.goals().next().unwrap();
        let ill = formula("A", Mode::INTUITIONISTIC).unwrap();
        let step = |e: Result<[GoalId; 2], Error>| match e {
            Err(Error::Step(e)) => e,
            other => panic!("{other:?}"),
        };
        // The succedent A going left with the cut formula: the first
        // premise would have two succedents.
        assert_eq!(
            step(session.cut(root, &ill, &[1], &limits)),
            StepError::Succedents { count: 2 }
        );
        assert_eq!(
            step(session.cut(root, &ill, &[2], &limits)),
            StepError::Split { position: 2 }
        );
        assert_eq!(
            step(session.cut(root, &ill, &[0, 0], &limits)),
            StepError::Split { position: 0 }
        );
        assert!(!session.forest().has_cuts());
        let [first, _] = session.cut(root, &ill, &[0], &limits).unwrap();
        assert!(matches!(
            step(session.cut(root, &ill, &[], &limits)),
            StepError::NoGoal { .. }
        ));
        // A classical formula typed in intuitionistic mode is refused by
        // the strict syntax; one-sided, ~A cannot be a goal.
        assert!(matches!(
            formula("A par B", Mode::INTUITIONISTIC),
            Err(Error::Parse(_))
        ));
        let one_sided = CutFormula::from("|- ~A".parse::<Sequent>().unwrap());
        assert!(matches!(
            session.cut(first, &one_sided, &[], &limits),
            Err(Error::NotIntuitionistic(_))
        ));
        assert!(matches!(
            formula("A, B", Mode::CLASSICAL),
            Err(Error::CutFormula { formulas: 2 })
        ));
        let Err(Error::Parse(e)) = formula("A * * B", Mode::CLASSICAL) else {
            panic!("no formula");
        };
        assert_eq!((e.span, e.column), (4..5, 5));
        let cyclic: Sequent = "|- A, ~A".parse().unwrap();
        let mut ordered = Interactive::new(&cyclic, Mode::CYCLIC).unwrap();
        let root = ordered.goals().next().unwrap();
        assert!(matches!(
            step(ordered.cut(root, &classical, &[], &limits)),
            StepError::Mode { .. }
        ));
    }
}
