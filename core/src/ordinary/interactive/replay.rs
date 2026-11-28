// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! A session read from its document: its parts checked as a state built
//! step by step would be, and its history replayed.

use super::witness::{Made, Unbound, Witnesses, implications_from, unify_axiom};
use super::{Interactive, shown};
use crate::Error;
use crate::first_order::{FoTerm, FoTermId};
use crate::limits::{Limits, Refusal};
use crate::linear::proofs::InfId;
use crate::ordinary::bound;
use crate::ordinary::derivation::check_inference;
use crate::ordinary::instance::Resolution;
use crate::ordinary::translate::Extra;
use crate::ordinary::{Formulas, Inference, Logic, NodeId, Rule};
use crate::unify::Unifier;

/// What a state holds, as its JSON form writes and reads it.
pub(crate) struct Parts {
    /// The logic.
    pub(crate) logic: Logic,
    /// The formulas.
    pub(crate) formulas: Formulas,
    /// The inferences, the root first.
    pub(crate) inferences: Vec<Inference>,
    /// The steps, each the inference it closed.
    pub(crate) history: Vec<InfId>,
    /// The bindings of open witnesses in force, in the order made: the
    /// witness, its term, the step whose axiom bound it.
    pub(crate) bindings: Vec<(u32, FoTermId, u32)>,
}

impl Interactive {
    /// Returns what the state holds, for its JSON form: the logic, the
    /// formulas, the inferences as their steps made them, before any
    /// binding of an open witness changed them, the history and the
    /// bindings in force, each the witness, its term and the step.
    #[expect(clippy::type_complexity, reason = "the parts of one form, used once")]
    pub(crate) fn parts(
        &self,
    ) -> (
        Logic,
        &Formulas,
        std::borrow::Cow<'_, [Inference]>,
        &[InfId],
        Vec<(u32, FoTermId, u32)>,
    ) {
        let Some(w) = self.witnesses.as_deref() else {
            return (
                self.logic,
                &self.formulas,
                std::borrow::Cow::Borrowed(&self.inferences),
                &self.history,
                Vec::new(),
            );
        };
        let mut inferences = self.inferences.clone();
        for (inference, raw) in inferences.iter_mut().zip(&w.raw) {
            if let Some(made) = raw {
                inference.left.clone_from(&made.left);
                inference.right.clone_from(&made.right);
                inference.term = made.term;
            }
        }
        (
            self.logic,
            &self.formulas,
            std::borrow::Cow::Owned(inferences),
            &self.history,
            w.unifier.bindings().collect(),
        )
    }

    /// Returns the state the parts describe, checked as a state built
    /// through [`apply`](Self::apply) is: the root first, every other
    /// inference the premise of exactly one before it, an open goal with
    /// neither rule nor premises and in intuitionistic and minimal logic
    /// one formula right of `⊢`, every other inference passing the
    /// checker's rules with its premises, the sequent within
    /// `limits.occurrences`, and the history distinct closed inferences
    /// each of whose steps added the suffix of the arena it then had.
    pub(crate) fn from_parts(parts: Parts, limits: &Limits) -> Result<Self, Error> {
        let Parts {
            logic,
            mut formulas,
            mut inferences,
            history,
            bindings,
        } = parts;
        let inconsistent = |reason| Error::InconsistentSession { reason };
        let root = inferences
            .first()
            .ok_or(inconsistent("the session has no inference"))?;
        bound(&formulas, &root.left, &root.right, limits)?;
        // Open witnesses make a session whose inferences are read as their
        // steps made them, each checked under the bindings in force then.
        if formulas.is_first_order()
            && !formulas
                .metas_in(
                    root.left.iter().chain(&root.right).copied(),
                    std::iter::empty(),
                )
                .is_empty()
        {
            return Err(inconsistent("the sequent holds an open witness"));
        }
        let open = !bindings.is_empty()
            || inferences
                .iter()
                .any(|i| i.term.is_some_and(|t| formulas.holds_meta(t)))
            || formulas.is_first_order()
                && !formulas
                    .metas_in(
                        inferences
                            .iter()
                            .flat_map(|i| i.left.iter().chain(&i.right))
                            .copied(),
                        std::iter::empty(),
                    )
                    .is_empty();
        let falsity = Extra::add(&mut formulas)?.falsity;
        let len = inferences.len();
        let mut parents = vec![0u8; len];
        for (n, inference) in inferences.iter().enumerate() {
            for id in inference.left.iter().chain(&inference.right) {
                formulas.holds(*id)?;
            }
            if let Some((side, at)) = inference.principal
                && at >= inference.side(side).len()
            {
                return Err(inconsistent(
                    "a principal position lies outside its sequent",
                ));
            }
            for p in &inference.premises {
                if p.index() <= n || p.index() >= len {
                    return Err(inconsistent("a premise does not follow its conclusion"));
                }
                parents[p.index()] = parents[p.index()].saturating_add(1);
            }
            if inference.rule == Rule::Open {
                let right = inference.right.len();
                if inference.principal.is_some()
                    || !inference.premises.is_empty()
                    || inference.term.is_some()
                {
                    return Err(inconsistent(
                        "an open goal has a principal, premises or a term",
                    ));
                }
                if logic != Logic::Classical && right != 1 {
                    return Err(inconsistent(
                        "an open goal of LJ must have exactly one formula right of ⊢",
                    ));
                }
                continue;
            }
            let premises: Vec<&Inference> = inference
                .premises
                .iter()
                .map(|p| &inferences[p.index()])
                .collect();
            if !open && check_inference(&formulas, logic, inference, &premises).is_err() {
                return Err(inconsistent("an inference breaks the rule it names"));
            }
        }
        if parents[0] != 0 || parents[1..].iter().any(|&n| n != 1) {
            return Err(inconsistent(
                "an inference is not the premise of exactly one",
            ));
        }
        // The history from the last step back: each step's goal closed,
        // named once, and what it added the suffix of the arena it left.
        let added = crate::goal_tree::added(
            &history,
            len,
            |i| &inferences[i.index()].premises[..],
            |i| inferences[i.index()].rule != Rule::Open,
        )
        .map_err(|fault| {
            inconsistent(match fault {
                crate::goal_tree::Fault::NotClosed => {
                    "a step names no inference it could have closed"
                }
                crate::goal_tree::Fault::NotSuffix => "a step did not add the suffix of the arena",
            })
        })?;
        // Its undo reopens each step's inference as a goal, which in LJ has
        // one formula right of `⊢`.
        if logic != Logic::Classical
            && history
                .iter()
                .any(|g| inferences[g.index()].right.len() != 1)
        {
            return Err(inconsistent(
                "a step of LJ closed a goal that has no formula or several right of ⊢",
            ));
        }
        let witnesses = if open {
            let read = Replay {
                logic,
                inferences: &inferences,
                history: &history,
                added: &added,
                most: limits.terms.unwrap_or(u64::MAX),
            };
            let (w, shown) = read.run(&mut formulas, &bindings)?;
            inferences = shown;
            Some(Box::new(w))
        } else {
            None
        };
        Ok(Self {
            logic,
            formulas,
            falsity,
            inferences,
            history,
            limit: limits.derivation_bytes,
            witnesses,
            terms: limits.terms.unwrap_or(u64::MAX),
        })
    }
}

/// A session document with open witnesses as its reader replays it: the
/// inferences as their steps made them, the steps and what each added.
struct Replay<'a> {
    /// The logic.
    logic: Logic,
    /// The inferences as made, the root first.
    inferences: &'a [Inference],
    /// The steps, each the inference it closed.
    history: &'a [InfId],
    /// The inferences each step added, a range per step.
    added: &'a [(usize, usize)],
    /// The bound on the symbols of a term an open witness stands for.
    most: u64,
}

impl Replay<'_> {
    /// Replays the steps in order, as a session took them: each step's
    /// open witness made at the level its scope gives, each axiom that
    /// unifies its formulas binding them as the session binds them, and
    /// every inference a step closed checked by the checker's rules under
    /// the bindings in force after that step, holding no witness made
    /// later. Then every inference is checked again under the bindings in
    /// force at the end, which can only bring eigenvariables in, and the
    /// bindings found must be the document's. Returns the witnesses and
    /// the inferences as they show the bindings.
    fn run(
        &self,
        formulas: &mut Formulas,
        bindings: &[(u32, FoTermId, u32)],
    ) -> Result<(Witnesses, Vec<Inference>), Error> {
        let bad = |reason| Error::InconsistentSession { reason };
        let inferences = self.inferences;
        let len = inferences.len();
        // The step that closed each inference: its goal, or one it added
        // that no later step closed.
        let mut closer: Vec<Option<usize>> = vec![None; len];
        for (s, (g, &(start, end))) in self.history.iter().zip(self.added).enumerate() {
            closer[start..end].fill(Some(s));
            closer[g.index()] = Some(s);
        }
        // Per step, the inferences it closed; those no step closed (a reader
        // without witnesses takes them) come first, before any witness.
        let mut by_step: Vec<Vec<usize>> = vec![Vec::new(); self.history.len() + 1];
        for (i, inference) in inferences.iter().enumerate() {
            if inference.rule != Rule::Open {
                by_step[closer[i].map_or(0, |s| s + 1)].push(i);
            }
        }
        let tree = Tree::of(formulas, inferences);
        let mut w = Witnesses::new(len);
        for (at, closed) in by_step.iter().enumerate() {
            let Some(s) = at.checked_sub(1) else {
                if formulas
                    .metas_in(
                        closed.iter().flat_map(|&i| {
                            let inference = &inferences[i];
                            inference.left.iter().chain(&inference.right).copied()
                        }),
                        closed.iter().filter_map(|&i| inferences[i].term),
                    )
                    .last()
                    .is_some()
                {
                    return Err(bad("an inference no step closed holds an open witness"));
                }
                self.check(formulas, &w.unifier, closed, false)?;
                continue;
            };
            // Fewer steps than inferences, which a `u32` counts.
            let number = s as u32;
            for &i in closed {
                let inference = &inferences[i];
                let Some(t) = inference.term else { continue };
                if !formulas.holds_meta(t) {
                    continue;
                }
                let opens = matches!(inference.rule, Rule::ForallLeft | Rule::ExistsRight)
                    && formulas.first_order().terms[t.index()]
                        == FoTerm::Meta(w.unifier.next_meta());
                if !opens {
                    return Err(bad(
                        "an open witness is not the witness of the one ∀L or ∃R that left it open",
                    ));
                }
                let level = tree.level[i].ok_or(bad("an eigenvariable of the last number"))?;
                w.push(t, level, number);
            }
            // No inference of the step, nor a premise's sequent, holds a
            // witness made later. A premise's term is its own step's.
            let made = w.unifier.next_meta();
            let sequents = closed.iter().flat_map(|&i| {
                let inference = &inferences[i];
                let premises = inference.premises.iter().map(|p| &inferences[p.index()]);
                std::iter::once(inference).chain(premises)
            });
            let roots = sequents.flat_map(|i| i.left.iter().chain(&i.right).copied());
            let terms = closed.iter().filter_map(|&i| inferences[i].term);
            if formulas
                .metas_in(roots, terms)
                .last()
                .is_some_and(|&k| k >= made)
            {
                return Err(bad(
                    "an inference holds an open witness no step left open before",
                ));
            }
            if let Some((left, right)) = self.axiom(s, &closer) {
                // The formulas as the session showed them when it took the
                // step: those it unified.
                let built = formulas.len();
                let now = w.unifier.metas().iter().map(|m| m.bound).collect();
                let mut resolution = Resolution::new(now);
                let pair = (
                    resolution.node(formulas, left)?,
                    resolution.node(formulas, right)?,
                );
                implications_from(formulas, built)?;
                let g = self.history[s].index();
                let in_scope = |e| tree.in_scope(e, g);
                match unify_axiom(
                    formulas,
                    &mut w.unifier,
                    pair,
                    number,
                    &in_scope,
                    self.most,
                    false,
                ) {
                    Ok(()) => {}
                    Err(Unbound::Large(size)) => {
                        return Err(Error::Refused(Refusal::Terms {
                            terms: size,
                            limit: self.most,
                        }));
                    }
                    Err(Unbound::Differ | Unbound::Eigen(_)) => {
                        return Err(bad(
                            "an axiom's formulas do not unify as the session's would",
                        ));
                    }
                }
            }
            self.check(formulas, &w.unifier, closed, false)?;
        }
        if !w.unifier.bindings().eq(bindings.iter().copied()) {
            return Err(bad("the bindings are not those the axioms make"));
        }
        let all: Vec<usize> = (0..len)
            .filter(|&i| inferences[i].rule != Rule::Open)
            .collect();
        let shown = self.check(formulas, &w.unifier, &all, true)?;
        let mut shown = shown.unwrap_or_else(|| inferences.to_vec());
        for ((inference, raw), made) in shown.iter_mut().zip(&mut w.raw).zip(inferences) {
            if inference.left != made.left
                || inference.right != made.right
                || inference.term != made.term
            {
                *raw = Some(Made::of(made));
            }
        }
        // Each inference under the witnesses it shows and those its form as
        // made reaches through the bindings: an undo of any binding in force
        // changes no other.
        let reach = reached(formulas, &w.unifier);
        for (i, (inference, made)) in shown.iter().zip(inferences).enumerate() {
            let mut metas = formulas.metas_in(
                made.left.iter().chain(&made.right).copied(),
                made.term.into_iter(),
            );
            metas = metas
                .iter()
                .flat_map(|&k| reach.get(k as usize).into_iter().flatten().copied())
                .chain(formulas.metas_in(
                    inference.left.iter().chain(&inference.right).copied(),
                    inference.term.into_iter(),
                ))
                .collect();
            metas.sort_unstable();
            metas.dedup();
            w.register(i, &metas);
        }
        Ok((w, shown))
    }

    /// Returns the formulas left and right of `⊢` of the axiom of step `s`,
    /// as made, where the step is an axiom's (its goal's run of
    /// weakenings, each closed by the step, ends in the axiom) and the two
    /// differ; `closer` is the step that closed each inference.
    fn axiom(&self, s: usize, closer: &[Option<usize>]) -> Option<(NodeId, NodeId)> {
        let mut at = &self.inferences[self.history[s].index()];
        while matches!(at.rule, Rule::WeakenLeft | Rule::WeakenRight)
            && let [p] = at.premises[..]
            && closer[p.index()] == Some(s)
            && self.inferences[p.index()].rule != Rule::Open
        {
            at = &self.inferences[p.index()];
        }
        match (at.rule, &at.left[..], &at.right[..]) {
            (Rule::Axiom, &[l], &[r]) if l != r => Some((l, r)),
            _ => None,
        }
    }

    /// Checks the inferences `closed` by the checker's rules as they show
    /// the bindings `unifier` holds, with their premises likewise. With
    /// `all`, returns every inference as it shows them, `None` where
    /// nothing is bound or `all` is not asked.
    fn check(
        &self,
        formulas: &mut Formulas,
        unifier: &Unifier,
        closed: &[usize],
        all: bool,
    ) -> Result<Option<Vec<Inference>>, Error> {
        let bindings: Vec<Option<FoTermId>> = unifier.metas().iter().map(|m| m.bound).collect();
        let bound = bindings.iter().any(Option::is_some);
        let built = formulas.len();
        let mut resolution = Resolution::new(bindings);
        let mut cache: crate::hash::HashMap<usize, Inference> = crate::hash::HashMap::default();
        let mut get = |i: usize, formulas: &mut Formulas| -> Result<Inference, Error> {
            if !bound {
                return Ok(self.inferences[i].clone());
            }
            if let Some(found) = cache.get(&i) {
                return Ok(found.clone());
            }
            let made = shown(&mut resolution, formulas, &self.inferences[i])?;
            cache.insert(i, made.clone());
            Ok(made)
        };
        let mut checked = Vec::with_capacity(closed.len());
        for &i in closed {
            let inference = get(i, formulas)?;
            let premises = inference
                .premises
                .iter()
                .map(|p| get(p.index(), formulas))
                .collect::<Result<Vec<_>, _>>()?;
            checked.push((inference, premises));
        }
        let every = if all && bound {
            Some(
                (0..self.inferences.len())
                    .map(|i| get(i, formulas))
                    .collect::<Result<Vec<_>, _>>()?,
            )
        } else {
            None
        };
        implications_from(formulas, built)?;
        for (inference, premises) in &checked {
            let premises: Vec<&Inference> = premises.iter().collect();
            if check_inference(formulas, self.logic, inference, &premises).is_err() {
                return Err(Error::InconsistentSession {
                    reason: "an inference breaks the rule it names under the bindings in force",
                });
            }
        }
        Ok(every)
    }
}

/// Returns, per open witness, the witnesses it reaches through the
/// bindings `unifier` holds, itself included, ascending: the open witnesses
/// of its term, of theirs, and so on. The bindings hold no cycle. It walks
/// on a stack of its own, each witness once.
fn reached(formulas: &Formulas, unifier: &Unifier) -> Vec<Vec<u32>> {
    let metas = unifier.metas();
    let mut reach: Vec<Option<Vec<u32>>> = vec![None; metas.len()];
    for start in 0..metas.len() {
        let mut stack = vec![(start, false)];
        while let Some((k, done)) = stack.pop() {
            if reach[k].is_some() {
                continue;
            }
            let next: Vec<u32> = match metas[k].bound {
                Some(t) => formulas.metas_in(std::iter::empty(), std::iter::once(t)),
                None => Vec::new(),
            };
            if done {
                let mut all = vec![k as u32];
                for &j in &next {
                    all.extend(
                        reach
                            .get(j as usize)
                            .and_then(Option::as_ref)
                            .into_iter()
                            .flatten(),
                    );
                }
                all.sort_unstable();
                all.dedup();
                reach[k] = Some(all);
            } else {
                stack.push((k, true));
                stack.extend(
                    next.iter()
                        .map(|&j| j as usize)
                        .filter(|&j| j < metas.len())
                        .map(|j| (j, false)),
                );
            }
        }
    }
    reach.into_iter().map(Option::unwrap_or_default).collect()
}

/// What the reader of a session asks of its tree of inferences: each
/// inference's place in a walk, the eigenvariables the inferences
/// introduce, and the level an open witness made there takes.
struct Tree {
    /// When a walk of the tree from the root enters each inference.
    enter: Vec<u32>,
    /// When it leaves it, after every inference above it.
    exit: Vec<u32>,
    /// The inferences that introduce each eigenvariable, by its term.
    introduced: crate::hash::HashMap<FoTermId, Vec<usize>>,
    /// Per inference, one more than the largest eigenvariable a `∀R` or
    /// `∃L` below it introduces, `Some(0)` for none and `None` past the
    /// last number.
    level: Vec<Option<u32>>,
}

impl Tree {
    /// Returns the tree of `inferences`, every premise after its
    /// conclusion and each the premise of one inference.
    fn of(formulas: &Formulas, inferences: &[Inference]) -> Self {
        let len = inferences.len();
        let mut introduced: crate::hash::HashMap<FoTermId, Vec<usize>> =
            crate::hash::HashMap::default();
        let mut level = vec![Some(0); len];
        for (i, inference) in inferences.iter().enumerate() {
            let above = match inference.eigenvariable() {
                Some(t) => {
                    introduced.entry(t).or_default().push(i);
                    match formulas.first_order().terms[t.index()] {
                        FoTerm::Eigen(e) => {
                            level[i].zip(e.get().checked_add(1)).map(|(a, b)| a.max(b))
                        }
                        FoTerm::Bound(_) | FoTerm::App(..) | FoTerm::Meta(_) => level[i],
                    }
                }
                None => level[i],
            };
            for p in &inference.premises {
                level[p.index()] = above;
            }
        }
        let (mut enter, mut exit) = (vec![0; len], vec![0; len]);
        let mut clock = 0u32;
        let mut stack = vec![(0usize, false)];
        while let Some((i, done)) = stack.pop() {
            clock += 1;
            if done {
                exit[i] = clock;
            } else {
                enter[i] = clock;
                stack.push((i, true));
                stack.extend(inferences[i].premises.iter().map(|p| (p.index(), false)));
            }
        }
        Self {
            enter,
            exit,
            introduced,
            level,
        }
    }

    /// Returns whether a `∀R` or `∃L` below inference `g` introduces the
    /// eigenvariable `e`.
    fn in_scope(&self, e: FoTermId, g: usize) -> bool {
        self.introduced.get(&e).is_some_and(|by| {
            by.iter()
                .any(|&b| self.enter[b] < self.enter[g] && self.exit[g] < self.exit[b])
        })
    }
}
