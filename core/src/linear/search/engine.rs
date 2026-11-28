// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! The proof-search engines: [`Engine`], which says which one decides
//! which goals, its [`Counter`]s, and why an engine forced by the options
//! is not taken ([`NotTaken`]).

#[cfg(doc)]
use super::{Bias, Options, Outcome, Refutation, Statistics};
use super::{Decide, additive, essential, focus, horn, inverse, net};
use crate::Error;
#[cfg(doc)]
use crate::limits::{Limits, Progress};
use crate::linear::fragment::{Fragment, Mode};
#[cfg(doc)]
use crate::linear::nets::ProofStructure;
#[cfg(doc)]
use crate::linear::occurrences::Reading;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The proof-search engines. An [`Outcome`] names the one that ran, and
/// [`Options::engine`] forces one.
///
/// Each variant describes its engine: how it searches, which options it
/// reads, and its unit of work.
///
/// # Which engine decides a goal
///
/// Unless [`Options::engine`] forces one, a goal goes to the engine of the
/// first row of this table that it fits. A row asks for a fragment the
/// goal lies in (detected, or asserted by [`Options::fragment`]), a mode,
/// and sometimes one more feature. Each row's engine is the one that
/// measured fastest there, or the only one that decides there at all.
///
/// | fragment | mode | feature | engine | measured (one thread) |
/// |---|---|---|---|---|
/// | unit-free MLL | Lambek, with or without empty antecedents | | [`Essential`](Engine::Essential) | the one engine of the Lambek calculus, by planar essential nets: chains `a₀, a₀ \ a₁, … ⊢ aₙ` of 1 024 and 4 096 links in 59 ms and 2.8 s, written with `/` in 23 and 393 ms, products of 4 096 atoms entailing themselves in 0.66 s, the exchanged and reversed forms refuted in 88 and 159 ms; 20 random derivations of the calculus over two atoms all proved in at most 4.7 ms (9.8 ms in all) and their exchanged forms decided in at most 0.4 ms, where the classical net search on the same one-sided sequents ran past 60 s on 8 of 20 and the focused engine took up to 3.5 s; without empty antecedents the products `x ⊢ x ⊗ ((a₀ \ a₀) ⊗ …)` of 16 to 256 factors refuted in 14 to 81 ms, with them proved in under 3 ms (`bench/problems/lambek.txt`) |
/// | unit-free MLL | cyclic | | [`Net`](Engine::Net) | the one engine that keeps the order, by planar linkings: chains `a₀, a₀ ⊸ a₁, … ⊢ aₙ` of 4 096 links in 0.46 s and products of 4 096 atoms entailing themselves in 0.89 s, as fast as the classical net search on the same sequents, which links them alike; their exchanged forms refuted in 0.08 and 0.16 s; random derivations of the cyclic calculus over two atoms decided in 23 µs to 0.4 ms, where the classical net search on the same sequents ran past 60 s on 2 of 20 (`bench/problems/cyclic.txt`) |
/// | additives only | any | two formulas, an additive connective or unit among them | [`Additive`](Engine::Additive) | `A ⊢ A` for `A` a complete tree of `&` and `⊕`: depth 8 in 0.5 ms against 1.4 ms on the focused engine, depth 14 in 0.1 to 0.2 s against 2.7 s, depth 16 in 0.3 s against over 20 s, in either mode |
/// | MELL | any, classical or intuitionistic, with or without Mix | a Horn program with a clause under `!`: a Petri net | [`Horn`](Engine::Horn) | in affine mode, the 176 coverability problems of the qcover suite at 5 s, intuitionistic: decides 162 (59 provable, 103 not, 155 within a second) where the two-sided engine decides 8, no verdict against another and every proof checked; 10 464 random affine Horn programs of one to four atoms and clauses, classical and intuitionistic: decides all, at most 0.2 ms each, where the focused engines leave 886 undecided at 1 s and are slower by more than a millisecond on 467. In linear mode, the library's 3 137 Petri nets at 5 s, intuitionistic, against the forward focused search (the factor bias within 30 copies, the better of the focused engine's two searches there): decides 3 026 nets against 1 628 (1 400 only by the Horn engine, 2 only by the forward search, no verdict against the other), in 0.23 ms against 1.2 ms in the median of the 1 626 both decide, faster on 1 013 of them; 2 670 within 10 ms against 1 103; the counter with 64 tokens proved in 0.07 ms and with the unreachable goal refuted in 0.6 ms, where the focused engine reaches its limit after 10 s; with the state equation, the dead transitions dropped and the backward search beside it, 3 071 of the nets; and of 7 000 random programs near the Horn shape it decides 4 303 of the 4 318 each linear mode sends it at 1 s, every one the focused engines decide and 548 to 580 more. Horn programs without `!` stay with the focused engines, whose counts decide the Partition encodings up to a hundred times faster (12 items: 15 ms against 2.0 s) |
/// | any, quantifiers too | intuitionistic | an exponential | [`TwoSided`](Engine::TwoSided), and a turn of [`Inverse`](Engine::Inverse) beside it | the inverse method refuses quantifiers, so a first-order goal is the two-sided engine's alone; where the two-sided engine alone ends at the time limit: on the library's intuitionistic problems at 60 s, one thread (the held-out sets, the rows no tool decided at 10 s, Sympli's ten proofs and six refutations; 1 113 rows), the turns decide 102 rows the two-sided engine alone does not and lose none, all ten proofs in 0.17 to 0.29 s and the six refutations in 0.15 to 22.8 s, with no verdict against another; the time with the turns over the time without is 1.00 in the median at every speed of the engine alone, at worst 1.24 from 10 ms to 1 s, 1.11 from 1 to 10 s and 1.42 over 10 s. The inverse method alone, which loses 20 rows the two-sided engine decides (the pigeonhole formulas `SYJ201` and `SYJ206`, `SYJ207`, `SYJ212`), gets no row of its own |
/// | any, quantifiers too | intuitionistic | | [`TwoSided`](Engine::TwoSided) | the general engine, and the one engine of intuitionistic first-order linear logic, by unification, on one thread or a pool; on unit-free IMLL faster than the net engine wherever both decide: wide (`a₀ ⊗ b₀, … ⊢ (bₙ ⊗ aₙ) ⊗ …`) and curried (`(a₀ ⊗ … ⊗ aₙ) ⊸ c ⊢ a₀ ⊸ (… ⊸ c)`) sequents of 1 024 and 4 096 atoms in 5 to 83 ms against 0.05 to 2.2 s, the Horn encodings of Partition 10 to 10⁴ times faster |
/// | any, quantifiers too | Mix, classical without weakening | | [`Focus`](Engine::Focus), and a turn of [`Inverse`](Engine::Inverse) beside it | the inverse method refuses quantifiers, so a first-order goal is the focused engine's alone; on the family `mix` the inverse method alone refutes every size measured (4 to 11) in at most 0.06 ms, where the focused engine takes 0.46 ms to 9.5 s and leaves sizes 10 and 11 to the classical refuter after 60 s; with the turns, sizes 8 to 11 are refuted in 0.42 to 0.47 s, and 4 and 6 within the head start, as without them. No other Mix sequent was measured, and some that the inverse method leaves undecided for seconds the focused engine decides in milliseconds, within the head start |
/// | any, quantifiers too | classical | | [`Focus`](Engine::Focus) | the general engine, the same on the one-sided sequent, and the one engine of first-order linear logic, by unification, on one thread or a pool; on unit-free MLL, with or without Mix, faster than the net engine wherever both decide: `wide` sequents of 8 to 4 096 literals, each once to four times, 1.07 to 3.6 times faster (`wide-m4` with 2 048: 166 ms against 594 ms), the Horn encodings of Partition and 3-Partition 1.6 to 32 000 times faster (`3-partition-mll-no/8` 37 µs against 2.8 ms) |
///
/// The bias of the focused engines, [`Bias::Auto`], is chosen per goal as
/// well, by the measurements its documentation names. Engines are ordered
/// as [`ALL`](Self::ALL) lists them.
///
/// # When another engine takes over
///
/// The focused engines recurse once per stable sequent and keep a memo.
/// So they may end at their recursion limit or memory bound. When that
/// happens on a unit-free MLL sequent in a mode without weakening, whose
/// atoms each occur once with each sign, the linking is forced, and
/// another engine takes the sequent over:
/// - After the recursion limit, the net engine. It keeps neither a
///   recursion nor a memo, and ends within one pass. On the chain
///   `a₀, a₀ ⊸ a₁, … ⊢ aₙ` of 1 024 and 4 096 links, the two-sided engine
///   meets the recursion limit after 11 and 46 ms. The net engine then
///   proves the chain in 26 ms and 0.43 s more, where the inverse method
///   took 51 ms and 1.0 s.
/// - After the memory bound in classical mode without Mix, the inverse
///   method, and the net engine after it where the inverse method meets a
///   bound of its own. The inverse method proves `wide-m1` with 4 096,
///   8 192 and 16 384 literals in 1.48, 3.05 and 11.8 s, against 3.09,
///   9.48 and 37.4 s by the net engine, and with 32 768 in 49.3 s, where
///   the net engine passes 60 s. Wide sequents shuffled or of triples that
///   reach the memory bound take it 0.017 to 0.33 of the net engine's
///   time.
/// - Under Mix and in intuitionistic mode, the net engine. Under Mix, the
///   inverse method ran past 60 s on `wide-m1` with 4 096 and 8 192
///   literals, which the net engine proved.
///
/// On every other goal measured, the focused engines decided first. Which
/// goals reach a limit depends on the limits, which the dispatch's
/// features (fragment, mode, the roots, the shape of a Horn program) do
/// not see.
///
/// # The inverse method's turns
///
/// Where a row names the inverse method beside its engine, the engine
/// first searches alone until it has done 2²⁴ units of its work (every
/// thread counted), some hundredths of a second. A goal it decides within
/// that never sees a turn. Then, and each time the engine's work has
/// doubled, the inverse method takes a turn on the same thread. Each turn
/// starts the inverse method afresh and does at most a thirty-second of
/// the engine's work, in the inverse method's own units. Then the engine
/// goes on where it stopped. So the turns together do at most a sixteenth
/// of the engine's work. A goal the inverse method decides in `w` of its
/// units is decided once the engine has done some thirty-two times `w`.
///
/// When a turn decides, its answer is the search's. Its proof is checked,
/// and so is its refutation ([`Refutation::Saturated`]). The outcome then
/// names [`Inverse`](Engine::Inverse) and carries its counters. Otherwise
/// the turn's sequents go, and the engine's answer is the search's.
/// [`Statistics::work`] counts the turns' work beside the engine's. After
/// a turn that met the memory bound or an index limit, no turn comes
/// again.
///
/// Turns come only to a search on one thread with no engine forced (a
/// race's pool searches beside the thread that takes them), and only at a
/// poll of the engine's own work. So where the default bias's second
/// search runs alone after the first ended, no turn comes. A turn's share
/// is counted in the inverse method's units, whose time can differ much
/// from the engine's.
///
/// Where the dispatch gives no turn, either the inverse method decided no
/// goal the engines did not, or its turns would slow or lose a goal the
/// engine decides:
/// - intuitionistic linear sequents without exponentials (`euler`, slower
///   at every size);
/// - classical sequents without exponentials or Mix (the generated
///   families);
/// - intuitionistic affine mode. There the turns decided `pigeonhole` #0
///   from 5 holes (2.1 s to 0.54 s, and 6 holes in 18 s where the
///   two-sided engine passes a minute). But they made #1 with 7 holes,
///   which the two-sided engine refutes in 1 s, 1.27 times slower. A share
///   small enough to spare it would have left 6 holes past the minute.
/// - classical goals with an exponential. There the turns decided the
///   family `chain` from 256 clauses on, and 100 classical readings of the
///   library's problems that the command leaves at its 2 s limit. But
///   where the inverse method's units ran some fifteen times slower than
///   the focused engine's, the turns took up to 2.8 times the engine's
///   time. On one thread they lost `SYJ212+1.015`, which the focused
///   engine refutes in 1.9 s, to the 2 s limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Engine {
    /// The focused sequent engine, one-sided: classical mode, every
    /// fragment.
    ///
    /// It searches backward over dyadic sequents `⊢ Θ ; Γ`. `Θ` holds the
    /// formulas that came under a `?` and may be copied, `Γ` the formulas
    /// each used once. The asynchronous phase decomposes the negative
    /// formulas, whose rules need no choice, until a stable sequent
    /// remains, one with no negative formula left. The memo of stable
    /// sequents may decide it. Otherwise the counts of each atom's
    /// literals may refute it without a search, or a focus proves it: the
    /// rules of a positive formula of `Γ`, or of a copy from `Θ`, applied
    /// in one go.
    ///
    /// The copies are bounded per branch, and the bound deepens.
    /// `Unprovable` comes only from a level that never met its bound. With
    /// Mix, a stable sequent may be split in two. In affine mode, a leaf
    /// weakens what is left.
    ///
    /// It reads every option but [`Options::test_period`], which is the
    /// net engine's. Its unit of work ([`Progress::work`],
    /// [`Limits::work`]) is a stable sequent, weighed by the size of its
    /// zones, members and copies, and a step of a split search. Forced
    /// splits count none.
    Focus,
    /// The proof-net engine, for unit-free MLL with or without Mix.
    ///
    /// In intuitionistic mode it decides IMLL through the embedding into
    /// MLL. In the cyclic mode it decides cyclic MLL, the only engine that
    /// does. In the commutative modes, the dispatch gives it a goal only
    /// where the focused engines end at their recursion limit or memory
    /// bound ("When another engine takes over", above). Forced, it decides
    /// any goal it admits.
    ///
    /// A cut-free proof of MLL is its axiom linking. So the engine links
    /// dual literals by backtracking over a [`ProofStructure`], and keeps
    /// the first linking the correctness criterion accepts. The count
    /// equation and each atom's balance refuse most unprovable sequents
    /// before any link. The net found is sequentialized into the proof
    /// returned.
    ///
    /// In the cyclic mode it keeps the linking planar. A link is admitted
    /// only when it crosses no link made and the literals between its ends
    /// are balanced in every atom. The symmetry breaks are off there,
    /// since an exchange of equal literals can make two links cross, and
    /// it runs on one thread.
    ///
    /// Of the options it reads [`Options::jobs`] and the pool,
    /// [`Options::test_period`], [`Options::check`] and the occurrence
    /// limit. A unit-free MLL sequent has no copies to bound, and the
    /// engine keeps no memo and does not recurse. So the copy bounds, the
    /// bias, the memo limit and the recursion limit have nothing to bound
    /// here. Its structure, scratch and tables of interchangeable literals
    /// are linear in the forest, and are not counted under
    /// [`Limits::memory_bytes`](crate::Limits::memory_bytes).
    ///
    /// Its unit of work is a literal chosen and an exact test that fails.
    Net,
    /// The focused sequent engine, two-sided: intuitionistic mode.
    ///
    /// It is the search of `Focus` on the one-sided sequent. That search
    /// keeps one goal on every branch by itself, except at the split of a
    /// hypothesis `A ⊸ B`, where the goal must go with `B`. That split is
    /// the one place it reads the sequent's [`Reading`]. It reads the
    /// options `Focus` does, and counts its work as `Focus` does.
    TwoSided,
    /// The fast path for a sequent of two additive-only formulas, in every
    /// mode.
    ///
    /// It is a recursion on pairs of subformula occurrences, one below
    /// each root, memoized on the pair. It takes time proportional to the
    /// product of the two formulas' sizes.
    ///
    /// It reads [`Options::memo_limit`] and [`Options::check`], and the
    /// limits' recursion depth and memory bound. It runs on the calling
    /// thread whatever [`Options::jobs`] says. Two additive formulas have
    /// no copies and no atoms to bias. Its unit of work is a pair decided,
    /// which it counts at its polls, every 1 024 pairs.
    Additive,
    /// The engine for Horn programs, which it reads as Petri nets.
    ///
    /// A Horn program here has clauses under `!` that may be used any
    /// number of times, implications used once, atoms, and one goal that
    /// is a tensor of atoms. It is a Petri net with a marking to reach. In
    /// `!(a ⊗ b ⊸ c), a, b ⊢ c`, the clause is a transition, the atoms on
    /// the left are the tokens, and the goal is the marking.
    ///
    /// It searches the markings instead of sequents. Each marking is
    /// reached once, and kept as a count of tokens per atom. The marking
    /// nearest to the goal is expanded first; the distance is the number of
    /// tokens by which the two differ. An implication used once is a
    /// transition that takes a token of its own.
    ///
    /// A firing sequence that reaches the goal is the proof. Every firing
    /// is a copy of its clause, whose body takes its tokens by axioms and
    /// whose head adds its atoms to the context. Once every marking reached
    /// has been expanded without reaching the goal, the goal is
    /// `Unprovable`, since a proof of a Horn program is a firing sequence
    /// read upward. So is, before any search, a goal whose atom counts
    /// cannot balance, as for every engine.
    ///
    /// Beside the search, a simplex solves the net's state equation: are
    /// the goal's tokens the start's plus some number of firings of each
    /// clause? It runs a slice at a time, and never does more work than the
    /// search has done. Where the equation has no solution, Farkas' lemma
    /// gives each atom a weight under which no clause raises the weighted
    /// count of the tokens, while the goal asks it raised. An exact check
    /// in integers confirms the weights. The goal is then `Unprovable` with
    /// that [`Refutation::StateEquation`], however many markings the net
    /// has.
    ///
    /// A net whose markings grow without end and whose equation has a
    /// solution is searched until the stop or
    /// [`Limits::memory_bytes`](crate::Limits::memory_bytes), which counts
    /// the markings kept and the simplex's basis. Two more refutations
    /// reach such nets:
    /// - a clause with an atom that no reachable marking holds never
    ///   fires, and is left out;
    /// - once the search has done some work, the same search runs backward
    ///   from the goal beside it, with a quarter of the work. If it
    ///   exhausts its markings, the goal is refuted. Its firing sequence,
    ///   read backward, is a proof.
    ///
    /// In affine mode, the tokens and the clauses a firing sequence leaves
    /// are weakened. So the question is whether a marking that covers the
    /// goal is reachable, and the engine decides it backward. From the
    /// goal, it computes the least markings from which a firing leads to
    /// one that covers it, and keeps each unless a smaller one is kept. It
    /// ends when one lies below the start (the proof), or when no new one
    /// comes (`Unprovable`), which Dickson's lemma says happens. The state
    /// equation runs beside it as in linear mode.
    ///
    /// It searches classical or intuitionistic, with or without Mix, which
    /// no proof of such a goal can use. It reads [`Options::check`] and the
    /// limits' memory bound, and runs on the calling thread whatever
    /// [`Options::jobs`] says. It needs no copy bound, memo limit or
    /// recursion limit: it keeps every marking once and does not recurse.
    /// Its unit of work is a marking taken from the frontier or an element
    /// from the queue, a marking computed backward, and a pivot of the
    /// state equation.
    Horn,
    /// The essential-net engine, for unit-free IMLL: intuitionistic mode
    /// without weakening, and the Lambek calculus.
    ///
    /// In the commutative modes the dispatch gives it no goal; forced, it
    /// decides any goal it admits. In the Lambek calculus it is the
    /// default and the only engine. There the criterion is that of planar
    /// essential nets (Lamarche and Retoré). A link is admitted only when
    /// it crosses no link made and the literals between its ends are
    /// balanced, and the symmetry breaks are off, as for the net engine's
    /// cyclic linking. Without empty antecedents, a complete linking is a
    /// proof only when, besides, the body of every implication but the
    /// goal has a free hypothesis other than the one it binds, so that no
    /// sequent of the proof is without one. A goal off the roots is
    /// searched as a sequent of its own.
    ///
    /// It is the net engine's search over the sequent's polarized
    /// structure, Lamarche's essential net, whose edges the [`Reading`]
    /// directs from the hypotheses toward the goal. A linking is a proof
    /// net when the directed graph has no cycle, and every implication
    /// `A ⊸ B` of output position dominates its antecedent: every path
    /// from `A` to the goal passes through it
    /// ([`ProofStructure::is_essential`]).
    ///
    /// It keeps the reflexive transitive closure of the graph as a bit
    /// matrix, quadratic in the sequent's size. The matrix and its undo
    /// log are counted under
    /// [`Limits::memory_bytes`](crate::Limits::memory_bytes). A link that
    /// would close a cycle, or lead an antecedent's literal to the goal
    /// around its implication, is rejected at once. The dominator
    /// condition is tested where the net engine runs its exact test. It
    /// counts, breaks symmetries and chooses literals as the net engine
    /// does, and sequentializes the net found into the proof returned.
    ///
    /// Of the options it reads [`Options::test_period`] and
    /// [`Options::check`]. It runs on the calling thread whatever
    /// [`Options::jobs`] says. Its unit of work is a literal chosen and a
    /// test that fails.
    ///
    /// Measured on one thread on the intuitionistic sequents the net
    /// engine was measured on, it is slower than the two-sided engine on
    /// all of them but the chain of 64 links (0.18 ms against 0.22 ms).
    /// Wide and curried sequents of 64 to 4 096 atoms take it 2.3 to 25
    /// times as long. The Horn encodings of Partition take it 1.3 to 4.5
    /// times as long, or run out of time at 60 s where the two-sided
    /// engine takes 13 and 75 ms. On the chains of 1 024 and 4 096 links,
    /// where the two-sided engine meets its recursion limit, it is 2.2 and
    /// 6 times slower than the net engine, which takes those over. It is
    /// faster than the net engine on the Partition encodings (16 to 660
    /// times) and on wide and curried sequents up to 1 024 atoms. So the
    /// dispatch gives it no goal.
    Essential,
    /// The focused inverse method, in every commutative mode and fragment.
    ///
    /// It is a forward search: it derives stable sequents from the axioms
    /// upward, instead of decomposing the goal. Its rules are whole
    /// focusing phases, compiled from the sequent before the search. A
    /// rule is a focus on a positive formula, or on a copy of a `?`
    /// formula's subformula, with the asynchronous phases of what it
    /// releases. There is one rule for each choice of `⊕` sides. A rule's
    /// premises are stable sequents derived before, and its axioms take a
    /// negative literal for each positive one.
    ///
    /// A derived sequent stands for itself, or, with a `⊤` above it or in
    /// affine mode, for every larger one. It is kept unless one kept
    /// already stands for all it does (subsumption, found through an index
    /// of the kept sequents by their members). It is also dropped when it
    /// cannot stand in a proof of the goal: when it has a member twice, or
    /// two members a `⊗`, `&` or `⊕` separates, outside the subformulas of
    /// a `?`, or, under the intuitionistic reading, other than one goal.
    ///
    /// The smallest sequent is taken up first. It is applied to every rule
    /// beside those taken up before it, and with Mix joined to each. The
    /// goal is proved once every stable sequent of its own asynchronous
    /// phase is. The proof is written from the record each sequent keeps
    /// of its rule and premises. Without exponentials the sequents are
    /// finitely many, and the search decides. With them, it proves every
    /// provable goal given time, needs no copy bound, and answers
    /// `Unprovable` only where nothing new comes.
    ///
    /// It reads [`Options::bias`], which says which atoms' literals are
    /// positive, and [`Options::check`]. It runs on the calling thread
    /// whatever [`Options::jobs`] says. The copy bounds, the memo limit and
    /// the recursion limit have nothing to bound here. Its rules, the
    /// sequents kept, their indices and records and the queue are counted
    /// under [`Limits::memory_bytes`](crate::Limits::memory_bytes). Over
    /// it, the search answers `Unknown`, since the sequents kept cannot be
    /// given back without losing what was derived.
    ///
    /// It refuses a goal within the sequent two of whose members no
    /// sequent of a proof holds together: one twice or below another, or
    /// two a `⊗`, `&` or `⊕` separates, outside a `?`. No derivation of the
    /// sequent has such a goal.
    ///
    /// Its units of work: a step of a rule written, a choice of `⊕` sides
    /// looked at, a literal looked at for an axiom's partner, and a member
    /// copied into the rules' tables; in the saturation, a member of a
    /// linear zone copied, compared or indexed, an entry looked up in the
    /// index of the kept sequents, and an application of a rule or Mix or
    /// a choice of premises given up. So its work grows with its time,
    /// whatever the size of its sequents.
    Inverse,
    /// An engine this version of the crate does not know: one a later
    /// version ran, named in an outcome read from its document. No search
    /// runs it, so forced through [`Options::engine`] it is refused
    /// ([`Error::UnknownName`]); it lists no counters, is not in
    /// [`ALL`](Self::ALL), and its name is `unknown`.
    Unknown,
}

impl Display for Engine {
    /// Writes the engine's name: `focus`, `net`, `two-sided`, `additive`,
    /// `horn`, `essential` or `inverse`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

/// A counter of [`Statistics`] as an engine fills it
/// ([`Engine::counters`]).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Counter {
    /// The field's name in the statistics' JSON form.
    pub key: &'static str,
    /// A short label, for a line of statistics.
    pub label: &'static str,
    /// The label in affine mode, where an engine may count something
    /// else: the Horn engine searches backward there.
    pub affine_label: &'static str,
    /// What it counts for the engine.
    pub meaning: &'static str,
}

impl Counter {
    /// Returns a counter with the key, the label (in every mode) and the
    /// meaning given.
    const fn new(key: &'static str, label: &'static str, meaning: &'static str) -> Self {
        Self {
            key,
            label,
            affine_label: label,
            meaning,
        }
    }

    /// Returns the counter with another label in affine mode.
    const fn affine(self, affine_label: &'static str) -> Self {
        Self {
            affine_label,
            ..self
        }
    }

    /// Returns the label for a search in the mode given.
    #[must_use]
    pub const fn label_in(&self, mode: Mode) -> &'static str {
        if mode.is_affine() {
            self.affine_label
        } else {
            self.label
        }
    }
}

/// Why an engine the options force does not take a goal.
///
/// Each engine checks, in this order: the largest fragment it decides,
/// its modes, whether the goal is the sequent itself, and the goal's
/// shape.
///
/// In JSON (feature `serialize`), as the `because` of an
/// `engine_refused` error's `details` beside its `engine`, tagged by
/// `kind`: `{"kind": "fragment", "decides": "MLL", "goal": "MELL"}`,
/// `{"kind": "mode", "mode": "affine"}`, `{"kind": "goal"}` or
/// `{"kind": "shape"}`; written, never read.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotTaken {
    /// The goal lies beyond the largest fragment the engine decides.
    #[non_exhaustive]
    Fragment {
        /// The largest fragment the engine decides.
        decides: Fragment,
        /// The fragment the goal was searched in.
        goal: Fragment,
    },
    /// The engine does not search in the mode.
    #[non_exhaustive]
    Mode {
        /// The mode asked for.
        mode: Mode,
    },
    /// The goal is not the sequent itself, the only goal the engine takes.
    Goal,
    /// The goal is not of the engine's shape: two additive-only formulas
    /// for the additive engine, a Horn program for the Horn engine, and
    /// for the inverse method a goal within the sequent whose members can
    /// stand together in one sequent of a proof.
    Shape,
}

impl NotTaken {
    /// Returns the sentence that says why `engine` does not take the goal.
    pub fn explained(self, engine: Engine) -> impl Display {
        Explained(engine, self)
    }
}

/// The sentence of a refusal of a forced engine.
struct Explained(Engine, NotTaken);

impl Display for Explained {
    /// Writes why the engine does not take the goal, in the engine's
    /// words where it has them.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Explained(engine, because) = *self;
        match (engine, because) {
            (Engine::Net, NotTaken::Fragment { goal, .. }) => {
                write!(
                    f,
                    "proof nets exist for MLL without units only, not for {goal}"
                )
            }
            (Engine::Essential, NotTaken::Fragment { goal, .. }) => {
                write!(
                    f,
                    "essential nets exist for IMLL without units only, not for {goal}"
                )
            }
            (Engine::Net, NotTaken::Mode { mode }) if mode.is_ordered() => write!(
                f,
                "the net engine does not search in {mode} mode, whose planar essential nets the \
                 essential engine searches"
            ),
            (Engine::Net, NotTaken::Mode { mode }) => write!(
                f,
                "proof nets exist in linear mode only, with or without Mix, not in {mode} mode"
            ),
            (_, NotTaken::Goal) => write!(
                f,
                "the {engine} engine decides the whole sequent only, not a goal within it"
            ),
            (_, NotTaken::Mode { mode }) => {
                write!(f, "the {engine} engine does not search in {mode} mode")
            }
            (Engine::Additive, NotTaken::Fragment { goal, .. }) => write!(
                f,
                "the additive engine decides a sequent of two additive-only formulas, not one of \
                 {goal}"
            ),
            (Engine::Additive, NotTaken::Shape) => f.write_str(
                "the additive engine decides a sequent of two additive-only formulas only",
            ),
            (Engine::Horn, NotTaken::Fragment { .. } | NotTaken::Shape) => f.write_str(
                "the horn engine decides a Horn program only: atoms, implications between \
                 tensors of atoms, such implications under !, and one goal that is a tensor of \
                 atoms",
            ),
            (_, NotTaken::Fragment { decides, goal }) => {
                write!(
                    f,
                    "the {engine} engine decides {decides} at most, not {goal}"
                )
            }
            (Engine::Inverse, NotTaken::Shape) => f.write_str(
                "the inverse engine does not take a goal two of whose members no sequent of a \
                 proof holds together: one twice or below another, or two that a ⊗, & or ⊕ \
                 separates, outside the formula under a ?",
            ),
            (_, NotTaken::Shape) => {
                write!(f, "the {engine} engine does not take a goal of this shape")
            }
        }
    }
}

impl Engine {
    /// Every engine, in the order of [`NAMES`](Self::NAMES).
    pub const ALL: &'static [Self] = &[
        Self::Focus,
        Self::Net,
        Self::TwoSided,
        Self::Additive,
        Self::Horn,
        Self::Essential,
        Self::Inverse,
    ];

    /// The engines' names, as [`name`](Self::name) writes them and
    /// [`FromStr`] reads them.
    pub const NAMES: &'static [&'static str] = &[
        "focus",
        "net",
        "two-sided",
        "additive",
        "horn",
        "essential",
        "inverse",
    ];

    /// Returns the counters of [`Statistics`] the engine fills, each with
    /// its key in the statistics' JSON form, a label and what it counts
    /// for this engine. The other counters stay zero. A front end shows
    /// these.
    pub const fn counters(self) -> &'static [Counter] {
        /// The unit of work, which every engine counts.
        const WORK: Counter = Counter::new("work", "units of work", "the engine's unit of work");
        /// The focused engines' counters.
        const FOCUSED: &[Counter] = &[
            Counter::new(
                "nodes",
                "stable sequents visited",
                "stable sequents visited, memo hits included",
            ),
            Counter::new(
                "memo_hits",
                "from the memo",
                "stable sequents the memo answered",
            ),
            Counter::new(
                "memo_entries",
                "memo entries at most",
                "the most entries the memo held at once",
            ),
            Counter::new(
                "splits",
                "splits examined",
                "steps of the split searches and forced splits",
            ),
            Counter::new(
                "copies",
                "copy bound reached",
                "the level of the search that decided, else of the backward one",
            ),
            Counter::new(
                "forward_copies",
                "forward search's copy bound reached",
                "the forward search's level under the default bias",
            ),
            Counter::new(
                "unifications",
                "unifications tried",
                "unifications of two literals tried, on a first-order sequent",
            ),
            Counter::new(
                "bindings",
                "metavariables bound",
                "metavariables bound, on a first-order sequent",
            ),
            WORK,
        ];
        /// The net engine's counters.
        const NET: &[Counter] = &[
            Counter::new(
                "nodes",
                "literals chosen",
                "literals the search chose a partner for",
            ),
            Counter::new("links", "links tried", "axiom links made"),
            Counter::new("tests", "exact tests run", "exact acyclicity tests"),
            WORK,
        ];
        /// The essential-net engine's counters.
        const ESSENTIAL: &[Counter] = &[
            Counter::new(
                "nodes",
                "literals chosen",
                "literals the search chose a partner for",
            ),
            Counter::new("links", "links tried", "axiom links made"),
            Counter::new(
                "tests",
                "dominator tests run",
                "tests of the dominator condition",
            ),
            WORK,
        ];
        /// The additive path's counters.
        const ADDITIVE: &[Counter] = &[
            Counter::new(
                "nodes",
                "pairs of subformulas visited",
                "pairs decided, memo hits included",
            ),
            Counter::new("memo_hits", "from the memo", "pairs the memo answered"),
            Counter::new(
                "memo_entries",
                "memo entries",
                "the most pairs the memo held at once",
            ),
            WORK,
        ];
        /// The Horn engine's counters.
        const HORN: &[Counter] = &[
            Counter::new(
                "nodes",
                "markings reached",
                "markings reached from either end, or in affine mode computed backward",
            )
            .affine("markings computed backward"),
            Counter::new(
                "memo_hits",
                "markings met again",
                "markings kept already, or covered already",
            )
            .affine("markings covered already"),
            Counter::new("memo_entries", "markings kept", "markings kept"),
            WORK,
        ];
        /// The inverse method's counters.
        const INVERSE: &[Counter] = &[
            Counter::new(
                "nodes",
                "sequents taken up",
                "derived sequents taken up and applied to the rules",
            ),
            Counter::new(
                "memo_hits",
                "sequents subsumed",
                "derived sequents a kept one subsumed",
            ),
            Counter::new("memo_entries", "sequents kept", "derived sequents kept"),
            WORK,
        ];
        match self {
            Self::Focus | Self::TwoSided => FOCUSED,
            Self::Net => NET,
            Self::Essential => ESSENTIAL,
            Self::Additive => ADDITIVE,
            Self::Horn => HORN,
            Self::Inverse => INVERSE,
            Self::Unknown => &[],
        }
    }

    /// Returns the engine's name: `focus`, `net`, `two-sided`, `additive`,
    /// `horn`, `essential` or `inverse`, and `unknown` for
    /// [`Unknown`](Self::Unknown).
    pub const fn name(self) -> &'static str {
        match self {
            Self::Focus => "focus",
            Self::Net => "net",
            Self::TwoSided => "two-sided",
            Self::Additive => "additive",
            Self::Horn => "horn",
            Self::Essential => "essential",
            Self::Inverse => "inverse",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the engine searches on several threads when
    /// [`Options::jobs`] asks for them.
    ///
    /// The focused engines do, and so does the net engine in a commutative
    /// mode. The additive, Horn, essential-net and inverse engines run on
    /// the calling thread whatever `jobs` says.
    pub const fn parallel(self) -> bool {
        match self {
            Engine::Focus | Engine::Net | Engine::TwoSided => true,
            Engine::Additive
            | Engine::Horn
            | Engine::Essential
            | Engine::Inverse
            | Engine::Unknown => false,
        }
    }

    /// Returns the implementation of the engine.
    pub(super) fn implementation(self) -> &'static dyn Decide {
        match self {
            Engine::Focus => &focus::ONE_SIDED,
            Engine::TwoSided => &focus::TWO_SIDED,
            Engine::Net => &net::Nets,
            Engine::Additive => &additive::Additive,
            Engine::Horn => &horn::Horn,
            Engine::Essential => &essential::Essentials,
            Engine::Inverse => &inverse::Inverse,
            Engine::Unknown => unreachable!("the front door refuses an engine it does not know"),
        }
    }
}

impl FromStr for Engine {
    type Err = Error;

    /// Reads an engine's name, one of [`NAMES`](Self::NAMES).
    fn from_str(name: &str) -> Result<Self, Error> {
        Self::ALL
            .iter()
            .copied()
            .find(|engine| engine.name() == name)
            .ok_or_else(|| Error::UnknownName {
                what: "engine",
                name: name.into(),
                known: Self::NAMES,
            })
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use super::super::dispatch::{DISPATCH, Feature, Modes};
    use super::*;

    /// The table "Which engine decides a goal" of `Engine`'s
    /// documentation lists the rows of `DISPATCH` in their order. Each line
    /// has the row's fragment, its modes by the cell's first word, a
    /// feature exactly where the row asks one, its engine, and the inverse
    /// method beside it exactly where the row gives it turns.
    #[test]
    fn the_documented_dispatch_is_the_dispatch() {
        let section = include_str!("engine.rs")
            .split("/// # Which engine decides a goal")
            .nth(1)
            .expect("the section");
        let rows: Vec<Vec<&str>> = section
            .lines()
            .skip(1)
            .map_while(|line| line.strip_prefix("///"))
            .map(str::trim)
            .filter(|line| line.starts_with('|'))
            .skip(2)
            .map(|line| line.trim_matches('|').split('|').map(str::trim).collect())
            .collect();
        assert_eq!(rows.len(), DISPATCH.len(), "{rows:?}");
        for (cells, row) in rows.iter().zip(&DISPATCH) {
            let fragment = match cells[0] {
                "additives only" => Fragment::ADDITIVE,
                "unit-free MLL" => Fragment::MLL,
                "MELL" => Fragment::MELL,
                "any" => Fragment::LL,
                "any, quantifiers too" => Fragment::LL1,
                other => panic!("a fragment the test does not know: {other:?}"),
            };
            let modes = match cells[1].split([',', ' ']).next() {
                Some("any") => Modes::Any,
                Some("classical") => Modes::Classical,
                Some("intuitionistic") => Modes::Intuitionistic,
                Some("cyclic") => Modes::Cyclic,
                Some("Lambek") => Modes::Lambek,
                Some("Mix") => Modes::Mix,
                other => panic!("modes the test does not know: {other:?}"),
            };
            let engine = cells[3]
                .split_once("(Engine::")
                .and_then(|(_, rest)| rest.split_once(')'))
                .map(|(name, _)| name);
            let probe = cells[3].contains("(Engine::Inverse)");
            assert_eq!(
                (fragment, modes, cells[2].is_empty(), engine, probe),
                (
                    row.fragment,
                    row.modes,
                    row.feature == Feature::Any,
                    Some(format!("{:?}", row.engine).as_str()),
                    row.probe
                ),
                "{cells:?}"
            );
        }
    }
}
