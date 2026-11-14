// deduxit © Fabian Lukas Grubmüller 2026
// Licensed under the EUPL

//! Families of problems with known verdicts, generated at any size.
//!
//! [`FAMILIES`] lists them, and [`find`] looks one up by name.
//! [`Family::instance`] builds a family's problem of a size as an
//! [`Instance`]: the sequent, the mode to prove it in, and whether it is
//! provable. A family with several problems per size numbers them by an
//! index. The verdict comes from the problem the family encodes or from
//! an argument at its generator, never from running an engine, so an
//! engine that disagrees is a finding. A random family seeds its
//! generator from the size and the index, so an instance is the same on
//! every run.
//!
//! The families are:
//! - hard problems of the literature: 3-Partition as a Horn program and
//!   as Lincoln and Winkler's two-literal sequent, Matsuoka's Partition,
//!   random QBF in a lock-and-key encoding into MALL, and Petri-net
//!   counters;
//! - problems on which one engine is known to be slow: wide contexts
//!   under Mix, wide sequents of distinct or shared atoms, and the slow
//!   cases of the exponentials;
//! - first-order problems: the Lambek calculus in first-order MILL, a
//!   chain of instances of a quantified clause under `!`, a witness
//!   chosen among many, and reachability through a cycle.
//!
//! Some families have a growing set of sizes ([`Family::growth`]) and a
//! claim ([`Family::claim`]): what the growth of the time to decide them
//! shows about an engine or another prover. Among them are Franzén's
//! contraction formulas and Cook's pigeonhole principle from Dyckhoff's
//! benchmarks, Euler tours from Chaudhuri's benchmarks of the inverse
//! method, and a clause used many times on one branch.
//!
//! The encodings are public functions too ([`three_partition`],
//! [`three_partition_mll`], [`partition`], [`qbf`], [`counter`],
//! [`wide`], [`mix`]), to build a problem from data of your own.
//!
//! # Examples
//!
//! List the families, then build an instance and prove it:
//!
//! ```
//! use deduxit::linear::families::{self, FAMILIES};
//! use deduxit::linear::search::{Options, Verdict, prove};
//!
//! for family in FAMILIES {
//!     println!("{}: {}", family.name, family.summary);
//! }
//!
//! let family = families::find("partition-no").unwrap();
//! let instance = family.instance(3, 0)?;
//! assert_eq!(instance.name, "partition-no/3");
//! assert!(!instance.provable);
//! let outcome = prove(&instance.sequent, instance.mode, &Options::default())?;
//! assert!(matches!(outcome.verdict, Verdict::Unprovable(_)));
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! A family with exponentials gives the copy bound to search with:
//!
//! ```
//! use deduxit::linear::families;
//! use deduxit::linear::search::{Options, prove};
//!
//! let instance = families::find("counter").unwrap().instance(8, 0)?;
//! let mut options = Options::default();
//! if let Some(copies) = instance.copies {
//!     options = options.with_copies(Some(copies));
//! }
//! let outcome = prove(&instance.sequent, instance.mode, &options)?;
//! assert!(outcome.verdict.proof().is_some());
//! # Ok::<(), deduxit::Error>(())
//! ```
//!
//! Needs the cargo feature `parse` (on by default).

use crate::{
    Error,
    linear::{Mode, Sequent},
};

/// One problem of a family, built by [`Family::instance`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Instance {
    /// The family's name, the size and, for a family of several
    /// instances per size, the index: `qbf/8#2`, `counter/8`.
    pub name: String,
    /// The sequent.
    pub sequent: Sequent,
    /// The mode to prove the sequent in.
    pub mode: Mode,
    /// Whether the sequent is provable in its mode.
    pub provable: bool,
    /// The per-branch copy bound to search with, for the families with
    /// exponentials: a proof exists within it when the instance is
    /// provable, and it bounds the search when the instance is not.
    /// `None` for the others.
    pub copies: Option<u32>,
    /// The witness depth to search with, for the families whose witnesses
    /// deepen with the size: a proof exists within it when the instance
    /// is provable. `None` for the others, which the default depth serves.
    pub witness_depth: Option<u32>,
}

/// A family of problems, indexed by a size.
#[derive(Debug)]
#[non_exhaustive]
pub struct Family {
    /// The name, as [`find`] takes it.
    pub name: &'static str,
    /// What the family is and what its size counts.
    pub summary: &'static str,
    /// The sizes a benchmark runs by default, from easy to past what the
    /// engines decide in minutes.
    pub sizes: &'static [u32],
    /// How many instances each size has: one for most constructed
    /// families, several for the random ones and for the constructed
    /// families of several shapes, whose summary says what each index is.
    pub instances: u32,
    /// The least size the family has an instance of.
    pub least: u32,
    /// Whether the family takes only powers of two as sizes.
    pub powers_of_two: bool,
    /// What the family shows: the strength or weakness of an engine or of
    /// another prover it is chosen for, and what a run is expected to
    /// show. Empty for a family that has no growing set.
    pub claim: &'static str,
    /// The sizes of its growing set, from easy to the ceiling where every
    /// engine that admits the family has met the time limit of a minute
    /// or a bound. They grow geometrically where the time grows
    /// polynomially, and by one where it grows exponentially. Empty for a
    /// family that has none.
    pub growth: &'static [u32],
    /// Builds the instance of a size and an index.
    generate: fn(u32, u32) -> Instance,
}

impl Family {
    /// Returns the instance of the given size and index, the same on
    /// every call.
    ///
    /// The index counts from 0 and must be below
    /// [`instances`](Self::instances). The call does not check it: past
    /// that, the instance may not be one of the family's, and its
    /// `provable` may be wrong. The instance is built without a bound on
    /// its occurrences, so a large size takes time and memory to match.
    ///
    /// # Errors
    ///
    /// [`Error::FamilySize`] for a size the family has no instance of:
    /// below [`least`](Self::least) (a 3-Partition bin smaller than four,
    /// a QBF of fewer than three variables, a solvable Partition of fewer
    /// than two items or an unsolvable one of fewer than three), or no
    /// power of two where [`powers_of_two`](Self::powers_of_two) asks one
    /// (a counter's tokens).
    pub fn instance(&self, size: u32, index: u32) -> Result<Instance, Error> {
        if size < self.least || (self.powers_of_two && !size.is_power_of_two()) {
            return Err(Error::FamilySize {
                family: self.name,
                size,
                least: self.least,
                powers_of_two: self.powers_of_two,
            });
        }
        let mut instance = (self.generate)(size, index);
        instance.name = if self.instances > 1 {
            format!("{}/{size}#{index}", self.name)
        } else {
            format!("{}/{size}", self.name)
        };
        Ok(instance)
    }
}

/// Every family, in the order a benchmark runs them.
pub static FAMILIES: &[Family] = &[
    Family {
        name: "3-partition-yes",
        summary: "3-Partition as a Horn program with a `&` over the bins per item (MALL), two bins of the size, solvable",
        sizes: &[4, 6, 8, 12],
        instances: 1,
        least: 4,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |b, _| {
            let items = solvable_triples(b);
            instance(three_partition(&items, 2, b), Mode::CLASSICAL, true)
        },
    },
    Family {
        name: "3-partition-no",
        summary: "3-Partition as a Horn program with a `&` over the bins per item (MALL), two bins of the size, unsolvable",
        sizes: &[4, 5],
        instances: 1,
        least: 4,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |b, _| {
            instance(
                three_partition(&unsolvable_triples(b), 2, b),
                Mode::CLASSICAL,
                false,
            )
        },
    },
    Family {
        name: "3-partition-mll-yes",
        summary: "3-Partition as Lincoln and Winkler's two-literal MLL sequent, two bins of the size, solvable",
        sizes: &[4, 6, 8, 12],
        instances: 1,
        least: 4,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |b, _| {
            let items = solvable_triples(b);
            instance(three_partition_mll(&items, 2, b), Mode::CLASSICAL, true)
        },
    },
    Family {
        name: "3-partition-mll-no",
        summary: "3-Partition as Lincoln and Winkler's two-literal MLL sequent, two bins of the size, unsolvable",
        sizes: &[4, 5, 6, 8],
        instances: 1,
        least: 4,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |b, _| {
            instance(
                three_partition_mll(&unsolvable_triples(b), 2, b),
                Mode::CLASSICAL,
                false,
            )
        },
    },
    Family {
        name: "partition-yes",
        summary: "Matsuoka's Horn encoding of Partition (MLL) with this many random items, solvable",
        sizes: &[4, 5, 6, 7, 12, 16, 20, 24, 28],
        instances: 1,
        least: 2,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |n, _| instance(partition(&partition_items(n, true)), Mode::CLASSICAL, true),
    },
    Family {
        name: "partition-no",
        summary: "Matsuoka's Horn encoding of Partition (MLL) with this many random items, unsolvable",
        sizes: &[3, 4, 5, 9, 12, 14, 15],
        instances: 1,
        least: 3,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |n, _| {
            instance(
                partition(&partition_items(n, false)),
                Mode::CLASSICAL,
                false,
            )
        },
    },
    Family {
        name: "qbf",
        summary: "random 3-QBF with this many alternating variables (∃ first), 1.5 clauses per variable with two existentials each, under a lock-and-key encoding into MALL",
        sizes: &[8, 12, 16, 20, 24, 32, 40, 44, 48],
        instances: 4,
        least: 3,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |n, index| {
            let (formula, valid) = qbf(n, index);
            instance(formula, Mode::CLASSICAL, valid)
        },
    },
    // Size 0 is the empty sequent, unprovable without Mix: the wide
    // families start at one pair.
    Family {
        name: "wide-m1",
        summary: "(a1 ⅋ b1) ⊗ … ⊗ (ak ⅋ bk), ~a1 ⊗ ~b1, …, ~ak ⊗ ~bk over distinct atoms (MLL), provable; the size is k",
        sizes: &[8, 16, 24, 32, 256, 2048],
        instances: 1,
        least: 1,
        powers_of_two: false,
        claim: "the inverse method past the focused engine's limits: with every atom once with each sign the linking is forced, so where the focused engine stops at its memory bound (4 096 pairs) the default hands the sequent over to the inverse method, which decides it in about quadratic time, three to four times as fast as the net engine, which takes it over under Mix",
        growth: &[256, 512, 1024, 2048, 4096, 8192, 16384, 32768],
        generate: |k, _| instance(wide(k, 1), Mode::CLASSICAL, true),
    },
    Family {
        name: "wide-m2",
        summary: "the wide sequent with every atom shared by two pairs: every literal twice",
        sizes: &[8, 16, 24, 32, 256, 2048],
        instances: 1,
        least: 1,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |k, _| instance(wide(k, 2), Mode::CLASSICAL, true),
    },
    Family {
        name: "wide-m3",
        summary: "the wide sequent with every atom shared by three pairs: every literal three times",
        sizes: &[12, 24, 30, 36, 256, 1024, 2048],
        instances: 1,
        least: 1,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |k, _| instance(wide(k, 3), Mode::CLASSICAL, true),
    },
    Family {
        name: "wide-m4",
        summary: "the wide sequent with every atom shared by four pairs: every literal four times",
        sizes: &[12, 24, 28, 32, 36, 256, 1024, 2048],
        instances: 1,
        least: 1,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |k, _| instance(wide(k, 4), Mode::CLASSICAL, true),
    },
    Family {
        name: "mix",
        summary: "(ai ⊗ bi) ⊕ 0, (~ai ⊗ ~bi) ⊕ 0 for this many i under Mix (MALL, where no count equation refutes it at once), unprovable",
        sizes: &[4, 6, 8, 9, 10, 11],
        instances: 1,
        least: 0,
        powers_of_two: false,
        claim: "the inverse method on Mix: it saturates in a few sequents at every size, where the focused engine alone grows about eightfold per pair as it meets the partitions of the context; the default gives the inverse method its turn after 2²⁴ units of the focused engine's work",
        growth: &[4, 6, 8, 10, 12, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096],
        generate: |k, _| instance(mix(k), Mode::CLASSICAL.with_mix(), false),
    },
    Family {
        name: "counter",
        summary: "the Petri-net counter !(c0 ⊗ c0 ⊸ c1), …, c0^n ⊢ cL with n tokens, n a power of two (MELL), provable within log2 n copies",
        sizes: &[2, 4, 8, 16, 32, 64],
        instances: 1,
        least: 0,
        powers_of_two: true,
        claim: "the Horn engine's strength: a counter decided as Petri-net reachability in about linear time, where the focused engine forced grows steeply, about as the cube of the tokens, and the inverse method forced does not finish from 16 tokens on",
        growth: &[
            2, 4, 8, 16, 32, 64, 128, 256, 1024, 4096, 16384, 65_536, 262_144,
        ],
        generate: |n, _| {
            let (sequent, levels) = counter(n, false);
            Instance {
                copies: Some(levels),
                ..instance(sequent, Mode::CLASSICAL, true)
            }
        },
    },
    Family {
        name: "counter-over",
        summary: "the counter with the unreachable goal cL ⊗ c0 (MELL), unprovable, searched to log2 n copies",
        sizes: &[2, 4, 8, 16, 32, 64],
        instances: 1,
        least: 0,
        powers_of_two: true,
        claim: "",
        growth: &[],
        generate: |n, _| {
            let (sequent, levels) = counter(n, true);
            Instance {
                copies: Some(levels),
                ..instance(sequent, Mode::CLASSICAL, false)
            }
        },
    },
    Family {
        name: "growing",
        summary: "!(a ⊸ a ⊗ a), a ⊢ ?b (MELL), unprovable, a context that grows with every copy; the size is the copy bound",
        sizes: &[16, 64, 256, 1024],
        instances: 1,
        least: 0,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |copies, _| Instance {
            copies: Some(copies),
            ..instance(parse("!(a -o a * a), a |- ?b"), Mode::CLASSICAL, false)
        },
    },
    Family {
        name: "chain",
        summary: "!(p0 ⊸ p1 & p2), …, !(p(k−1) ⊸ pk & p(k+1)), p0 ⊢ p(k+1) (MELL, ILL-shaped), provable within k + 2 copies; the size is k",
        sizes: &[16, 64, 128, 256],
        instances: 1,
        // Size 0 is `p0 ⊢ p1`, unprovable, and the family marks every
        // instance provable.
        least: 1,
        powers_of_two: false,
        claim: "the focused engine's memo on repeated subgoals: the `&` of each clause offers two ways on, so Fibonacci-many paths lead to the goal and meet the same subgoals, which the memo answers once (about half the stable sequents are hits), so the time grows polynomially where without the memo it passes a minute at 64; the default deepening (`--copies none`, about k/2 + 1 levels) costs nothing more, the memo answering each level's repeated subgoals; the inverse method forced is faster still, though no row of the dispatch gives it a turn on a classical sequent without Mix",
        growth: &[16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192],
        generate: |k, _| {
            let clauses: Vec<String> = (0..k)
                .map(|i| format!("!(p{i} -o p{} & p{})", i + 1, i + 2))
                .collect();
            let text = format!("{}, p0 |- p{}", clauses.join(", "), k + 1);
            Instance {
                copies: Some(k + 2),
                ..instance(parse(&text), Mode::CLASSICAL, true)
            }
        },
    },
    Family {
        name: "additive",
        summary: "A ⊢ A for A a complete tree of this depth alternating & and ⊕ over distinct atoms (the additive path), provable",
        sizes: &[8, 12, 14, 16],
        instances: 1,
        least: 0,
        powers_of_two: false,
        claim: "",
        growth: &[],
        generate: |depth, _| {
            let formula = additive_tree(depth, 0, &mut 0);
            instance(
                parse(&format!("{formula} |- {formula}")),
                Mode::CLASSICAL,
                true,
            )
        },
    },
    Family {
        name: "lambek-fo",
        summary: "Lambek sequents in Moot and Piazza's translation into first-order MILL, a formula at a pair of string positions; the size is k: #0 the chain a0, a0 \\ a1, …, a(k−1) \\ ak ⊢ ak, provable, #1 its first two formulas exchanged, unprovable, #2 the chain ak / a(k−1), …, a1 / a0, a0 ⊢ ak, provable, #3 a0, …, a(k−1) ⊢ a0 • … • a(k−1), provable, #4 its goal reversed, unprovable",
        sizes: &[16, 64, 256, 512],
        instances: 5,
        // At size 1 the reversed product of #4 is the product itself,
        // provable, and #4 is marked unprovable.
        least: 2,
        powers_of_two: false,
        claim: "LinearOne's home ground: the Lambek calculus in first-order MILL, the fragment LinearOne's proof-net search is written for; the two-sided first-order search is faster at small sizes and meets its recursion limit of 2 048 at 512 formulas on #0 and #2 and at 1 024 on #3 and #4",
        growth: &[8, 16, 32, 64, 128, 256, 512, 1024, 2048],
        generate: |k, index| {
            let (hypotheses, goal, provable) = lambek_shape(k, index);
            instance(
                parse(&lambek_first_order(&hypotheses, &goal)),
                Mode::INTUITIONISTIC,
                provable,
            )
        },
    },
    // #0 is proved by n copies of the clause, instantiated at z, s(z), …;
    // fewer do not reach sⁿ(z). #1 is unprovable: forgetting linearity
    // turns a proof into one of classical first-order logic, and the model
    // whose domain is the closed terms, with p true of the terms sⁱ(z)
    // alone, satisfies the hypotheses and not p(b). No refuter decides it
    // (MELL1 is undecidable), so a search ends at the copy bound.
    Family {
        name: "chain-fo",
        summary: "p(z), !∀x.(p(x) ⊸ p(s(x))) ⊢ p(sⁿ(z)) (MELL1), #0 provable within n copies, and #1 the goal p(b), unprovable, searched to n copies; the size is n",
        sizes: &[16, 64, 256, 512],
        instances: 2,
        least: 1,
        powers_of_two: false,
        claim: "the first-order search's instances of one clause: n instances on one branch, decided in polynomial time until the recursion limit of 2 048, met near 400 copies, while #1 ends at its copy bound; LinearOne refuses it, its fragment being MILL1",
        growth: &[16, 32, 64, 128, 256, 384, 512, 1024],
        generate: |n, index| {
            let goal = if index == 0 {
                format!("p({}z{})", "s(".repeat(n as usize), ")".repeat(n as usize))
            } else {
                "p(b)".to_owned()
            };
            let text = format!("p(z), !forall x. (p(x) -o p(s(x))) |- {goal}");
            // The last step binds x to sⁿ⁻¹(z).
            Instance {
                copies: Some(n),
                witness_depth: Some(n - 1),
                ..instance(parse(&text), Mode::CLASSICAL, index == 0)
            }
        },
    },
    // The provable instances take `an` as the witness, and #2 takes the
    // other p(ai) for the factors ∃x.p(x). In #1 and #3 a cut-free proof
    // links the goal's q(w) to the only q given, so w = b, and then p(b)
    // to some p(ai), which differs from it: unprovable.
    Family {
        name: "witness-fo",
        summary: "a witness among n: #0 p(a1) & … & p(an), q(an) ⊢ ∃x.(p(x) ⊗ q(x)) (MALL1), provable, #1 the same with q(b), unprovable, #2 p(a1), …, p(an), q(an) ⊢ ∃x.(p(x) ⊗ q(x)) ⊗ ∃x.p(x) ⊗ … with n − 1 factors ∃x.p(x) (MILL1), provable, #3 the same with q(b), unprovable",
        sizes: &[4, 8, 10, 12],
        instances: 4,
        least: 1,
        powers_of_two: false,
        claim: "a witness among n: the first-order search tries the instances in turn, and its split steps on the MILL1 refutation #3 grow exponentially (seconds at ten) where LinearOne's proof-net search stays near flat, a pool of threads dividing that work; LinearOne refuses the MALL1 shapes #0 and #1",
        growth: &[2, 4, 6, 8, 10, 12, 14, 16],
        generate: |n, index| {
            let alternatives: Vec<String> = (1..=n).map(|i| format!("p(a{i})")).collect();
            let q = if index % 2 == 0 {
                format!("q(a{n})")
            } else {
                "q(b)".to_owned()
            };
            let pair = "(exists x. (p(x) * q(x)))";
            let text = if index < 2 {
                format!("{}, {q} |- {pair}", alternatives.join(" & "))
            } else {
                let mut goal = vec![pair];
                goal.extend((1..n).map(|_| "(exists x. p(x))"));
                format!("{}, {q} |- {}", alternatives.join(", "), goal.join(" * "))
            };
            instance(parse(&text), Mode::INTUITIONISTIC, index % 2 == 0)
        },
    },
    // #0 is proved by using the clause n + 1 times on one branch: choosing
    // pi ⊸ f puts pi into the context for the next use, and once every pi
    // is there the tensor takes them. #1 is unprovable: forgetting
    // linearity and `!` gives ILTP's SYJ209, false in the Kripke model of
    // two worlds r ≤ u with p2, …, p(n) forced at both and p1 and f at u
    // alone (¬¬p1 is forced at r, the clause at both, f not at r).
    Family {
        name: "contraction",
        summary: "Franzén's formulas in ILL: !((p1 ⊗ … ⊗ p(n)) ⊕ (p1 ⊸ f) ⊕ … ⊕ (p(n) ⊸ f) ⊸ f) ⊢ f, #0 provable within n + 1 copies (ILTP's SYJ203), #1 with ((p1 ⊸ 0) ⊸ 0) ⊸ f for p1 ⊸ f, unprovable (SYJ209), searched to n + 1 copies; the size is n",
        sizes: &[2, 4, 8],
        instances: 2,
        least: 1,
        powers_of_two: false,
        claim: "the inverse method's strength on a clause used n + 1 times on one branch: the backward search finds the proof in exponentially many stable sequents and the inverse method in about n²/2 sequents, the default giving it its turn after 2²⁴ units of work; on the non-theorem #1 the backward search ends at its copy bound and the inverse method saturates, up to 16 atoms within a minute",
        growth: &[2, 4, 8, 16, 32, 64, 128, 256, 512],
        generate: |n, index| {
            let atoms: Vec<String> = (1..=n).map(|i| format!("p{i}")).collect();
            let mut alternatives = vec![format!("({})", atoms.join(" * "))];
            alternatives.extend(atoms.iter().enumerate().map(|(i, p)| {
                if i == 0 && index == 1 {
                    format!("((({p} -o 0) -o 0) -o f)")
                } else {
                    format!("({p} -o f)")
                }
            }));
            let text = format!("!(({}) -o f) |- f", alternatives.join(" + "));
            Instance {
                copies: Some(n + 1),
                ..instance(parse(&text), Mode::INTUITIONISTIC, index == 0)
            }
        },
    },
    // #0: each pigeon's ⊕ splits the proof, and on every branch two of the
    // n + 1 pigeons share one of the n holes, whose disjunct the goal
    // takes, the other atoms weakened. #1: with as many pigeons as holes
    // the classical reading is false where pigeon i sits in hole i alone,
    // and affine ILL proves nothing classically false.
    Family {
        name: "pigeonhole",
        summary: "Cook's pigeonhole principle in intuitionistic affine MALL: o(p,1) ⊕ … ⊕ o(p,n) for each pigeon p ⊢ the ⊕ over the holes h and the pairs p < q of o(p,h) ⊗ o(q,h), #0 with n + 1 pigeons, provable (ILTP's SYJ202), #1 with n pigeons, unprovable; the size is the holes n",
        sizes: &[2, 3, 4],
        instances: 2,
        least: 2,
        powers_of_two: false,
        claim: "the two-sided engine's weakness: the pigeons' ⊕ on the left split the search into nⁿ⁺¹ branches, which the inverse method forced avoids until its memory bound, while the default gives the inverse method no turn on a sequent without exponentials; #1, false classically, is refuted by its classical reading once the backward search has ended",
        growth: &[2, 3, 4, 5, 6, 7, 8],
        generate: |n, index| {
            let pigeons = if index == 0 { n + 1 } else { n };
            let hypotheses: Vec<String> = (0..pigeons)
                .map(|p| {
                    let holes: Vec<String> = (0..n).map(|h| format!("o{p}_{h}")).collect();
                    format!("({})", holes.join(" + "))
                })
                .collect();
            let mut pairs = Vec::new();
            for h in 0..n {
                for p in 0..pigeons {
                    for q in p + 1..pigeons {
                        pairs.push(format!("(o{p}_{h} * o{q}_{h})"));
                    }
                }
            }
            let text = format!("{} |- {}", hypotheses.join(", "), pairs.join(" + "));
            instance(parse(&text), Mode::INTUITIONISTIC.with_affine(), index == 0)
        },
    },
    // A cut-free proof of x, E ⊢ y, E edges between atoms, is a trail from
    // x to y using every edge of E once: an axiom is the empty trail, `&`
    // on the left picks a direction, and `⊸` on the left sends x to the
    // premise proving its argument (a premise without an atom proves
    // nothing) and its result to the other. So #0 is provable, the
    // square of a cycle being connected with every degree four (Euler),
    // and #1 is not: without the edge v1v2 those two vertices have degree
    // three.
    Family {
        name: "euler",
        summary: "Euler tours in IMALL: v0, (va ⊸ vb) & (vb ⊸ va) for every edge ab ⊢ v0, a closed trail from v0 through every edge once, on the square of a cycle of n vertices (vi adjacent to vi±1 and vi±2); #0 provable, #1 without the edge v1v2, unprovable; the size is n",
        sizes: &[5, 6, 8],
        instances: 2,
        least: 5,
        powers_of_two: false,
        claim: "the focused engines' context splits: each ⊸ divides the remaining edges between its premises and each & picks a direction, so the exhaustive search on a graph without an Euler circuit (#1) grows exponentially, about twofold per vertex, where a circuit (#0) is found at once; Chaudhuri's inverse method was measured on complete graphs, whose edges grow too fast for a curve",
        growth: &[5, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24],
        generate: |n, index| {
            let mut edges: Vec<(u32, u32)> = (0..n)
                .flat_map(|i| [1, 2].map(|d| (i.min((i + d) % n), i.max((i + d) % n))))
                .filter(|&edge| index == 0 || edge != (1, 2))
                .collect();
            edges.sort_unstable();
            let edges: Vec<String> = edges
                .iter()
                .map(|(a, b)| format!("((v{a} -o v{b}) & (v{b} -o v{a}))"))
                .collect();
            let text = format!("v0, {} |- v0", edges.join(", "));
            instance(parse(&text), Mode::INTUITIONISTIC, index == 0)
        },
    },
    // m − 1 uses of the clause turn the one `a` into m, all on the branch
    // before the goal's tensor splits them; a proof of ILL is one of
    // classical linear logic, so #1 is provable too.
    Family {
        name: "duplicate",
        summary: "!(a ⊸ a ⊗ a), a ⊢ a ⊗ … ⊗ a with m factors, provable within m − 1 copies on one branch, #0 intuitionistic (IMELL), #1 classical (MELL), so that the provers of either run it; the size is m",
        sizes: &[2, 4, 8],
        instances: 2,
        least: 1,
        powers_of_two: false,
        claim: "the other provers' caps on contraction: one clause used m − 1 times on one branch, past lolli's two uses of a clause, APLL's bound of 3, llprover's threshold of 5 and the Maude prover's copy bound of 5, while deduxit's Horn engine decides it in about linear time",
        growth: &[
            2, 4, 8, 16, 32, 64, 128, 256, 1024, 4096, 16384, 65_536, 262_144,
        ],
        generate: |m, index| {
            let text = format!("!(a -o a * a), a |- {}", power("a", m, " * "));
            let mode = if index == 0 {
                Mode::INTUITIONISTIC
            } else {
                Mode::CLASSICAL
            };
            Instance {
                copies: Some(m - 1),
                ..instance(parse(&text), mode, true)
            }
        },
    },
    // #0 is proved by n uses of the clause along the path n0, …, n(n−1),
    // z, nested on one branch, at whose end the first edge takes a copy
    // of its own: n + 1 copies. #1 is unprovable: forgetting linearity,
    // the model whose domain is the constants, with e true of the edges
    // given and r true of the cycle's nodes alone, satisfies the
    // hypotheses and not r(z). No refuter decides it in general, so a
    // search ends at a bound or the limit.
    Family {
        name: "reach-fo",
        summary: "reachability through a cycle (IMELL1): !e(n0,n1), …, !e(n(n−1),n0), r(n0), !∀x∀y.(e(x,y) ⊗ r(x) ⊸ r(y)) ⊢ r(z), #0 with the exit !e(n(n−1),z), provable within n + 1 copies, #1 without it, unprovable, searched to n + 1 copies; the size is the cycle's nodes n, one a loop",
        sizes: &[2, 4],
        instances: 2,
        least: 1,
        powers_of_two: false,
        claim: "the first-order memo's loop check: a search through a cycle comes back to an instance it has met, which the memo recognises, so the exit is found (without the memo the search passes a minute at 8 nodes, with it at 12); the time still grows steeply with the cycle; #1, the cycle without an exit, ends at its copy bound",
        growth: &[2, 3, 4, 5, 6, 8, 10, 12, 16],
        generate: |n, index| {
            let mut hypotheses: Vec<String> = (0..n)
                .map(|i| format!("!e(n{i}, n{})", (i + 1) % n))
                .collect();
            if index == 0 {
                hypotheses.push(format!("!e(n{}, z)", n - 1));
            }
            let text = format!(
                "{}, r(n0), !forall x. forall y. (e(x, y) * r(x) -o r(y)) |- r(z)",
                hypotheses.join(", ")
            );
            Instance {
                copies: Some(n + 1),
                ..instance(parse(&text), Mode::INTUITIONISTIC, index == 0)
            }
        },
    },
];

/// Returns the family of that name, or `None` if there is none.
pub fn find(name: &str) -> Option<&'static Family> {
    FAMILIES.iter().find(|family| family.name == name)
}

/// Returns a 3-Partition problem as a linear Horn program, in the style of
/// Kanovich's encodings: can `items` be split into `bins` triples, each of
/// total `size`?
///
/// - Bin `j` offers `size` units `bj` and three slots `tj`.
/// - Item `i` is the `&`, over the bins, of a clause that takes the
///   item's units and a slot from that bin and gives `di`.
/// - The goal is the tensor of every `di`.
///
/// Every resource must be used exactly once, so the sequent is provable
/// in classical mode if and only if the items split into triples of total
/// `size`, one per bin.
pub fn three_partition(items: &[u32], bins: u32, size: u32) -> Sequent {
    let mut hypotheses = Vec::new();
    for j in 1..=bins {
        hypotheses.extend((0..size).map(|_| format!("b{j}")));
        hypotheses.extend((0..3).map(|_| format!("t{j}")));
    }
    for (i, &a) in items.iter().enumerate() {
        let clauses: Vec<String> = (1..=bins)
            .map(|j| format!("({} * t{j} -o d{i})", power(&format!("b{j}"), a, " * ")))
            .collect();
        hypotheses.push(clauses.join(" & "));
    }
    let goal: Vec<String> = (0..items.len()).map(|i| format!("d{i}")).collect();
    parse(&format!(
        "{} |- {}",
        hypotheses.join(", "),
        goal.join(" * ")
    ))
}

/// Returns a 3-Partition problem as Lincoln and Winkler's MLL sequent of
/// two literals, `⊢ k ⊗ (~c ⅋ … ⅋ ~c), …, ((~k ⅋ ~k ⅋ ~k) ⅋ (c ⊗ … ⊗
/// c)) ⊗ …`.
///
/// An item of size `s` is `k ⊗` the `⅋` of `s` literals `~c`. A bin is
/// three `~k` beside the tensor of `size` literals `c`. The bins are
/// joined by `⊗`, so each bin's premise takes three items of total
/// `size`.
pub fn three_partition_mll(items: &[u32], bins: u32, size: u32) -> Sequent {
    let mut roots: Vec<String> = items
        .iter()
        .map(|&s| format!("k * ({})", power("~c", s, " | ")))
        .collect();
    let bin = format!("((~k | ~k | ~k) | ({}))", power("c", size, " * "));
    roots.push(power(&bin, bins, " * "));
    parse(&format!("|- {}", roots.join(", ")))
}

/// Returns a Partition problem as a Horn sequent, as Matsuoka encodes it:
/// can `sizes` be split into two halves of equal total?
///
/// The items `a1`, …, `an` start out as one tensor. Item `ai` of size
/// `si` can be turned into `si` units `b` or into `si` units `c`. Half the
/// total in `b` and half in `c` buy all the items back once, and the same
/// again buys the goal `e`. Every clause is used exactly once, so the
/// sequent is provable in classical mode if and only if the sizes split
/// into two halves of equal total.
///
/// # Panics
///
/// If the sizes add up to an odd total.
pub fn partition(sizes: &[u32]) -> Sequent {
    let total: u32 = sizes.iter().sum();
    assert_eq!(total % 2, 0, "the total is even");
    let items: Vec<String> = (1..=sizes.len()).map(|i| format!("a{i}")).collect();
    let mut hypotheses = vec![items.join(" * ")];
    for unit in ["b", "c"] {
        for (i, &size) in sizes.iter().enumerate() {
            hypotheses.push(format!("(a{} -o {})", i + 1, power(unit, size, " * ")));
        }
    }
    let half = format!(
        "({} * {})",
        power("b", total / 2, " * "),
        power("c", total / 2, " * ")
    );
    hypotheses.push(format!("({half} -o {})", items.join(" * ")));
    hypotheses.push(format!("({half} -o e)"));
    parse(&format!("{} |- e", hypotheses.join(", ")))
}

/// Returns the counter, a problem of Petri-net reachability, and the
/// copies it needs.
///
/// The sequent has `tokens` tokens `c0` and the reusable clauses `!(ci ⊗
/// ci ⊸ c(i+1))`, which double up to the goal `cL`, `L` the base-2
/// logarithm of `tokens`. With `over`, the goal is `cL ⊗ c0`, which no
/// firing reaches. The number returned is `L`, the copies a branch to one
/// token of the goal takes.
///
/// # Panics
///
/// If `tokens` is not a power of two.
pub fn counter(tokens: u32, over: bool) -> (Sequent, u32) {
    assert!(tokens.is_power_of_two(), "{tokens} tokens");
    let levels = tokens.trailing_zeros();
    let mut hypotheses: Vec<String> = (0..levels)
        .map(|i| format!("!(c{i} * c{i} -o c{})", i + 1))
        .collect();
    hypotheses.extend((0..tokens).map(|_| "c0".to_owned()));
    let extra = if over { " * c0" } else { "" };
    let text = format!("{} |- c{levels}{extra}", hypotheses.join(", "));
    (parse(&text), levels)
}

/// Returns the wide sequent `⊢ (a1 ⅋ b1) ⊗ … ⊗ (ak ⅋ bk), ~a1 ⊗ ~b1, …,
/// ~ak ⊗ ~bk` of `k` pairs, where pair `i` uses the atoms of index `i /
/// shared`.
///
/// Every literal occurs `shared` times, the last atoms fewer when
/// `shared` does not divide `k`. The sequent is provable for `k` of 1 or
/// more, since identifying atoms keeps a proof a proof. `wide(0, _)` is
/// the empty sequent, which is unprovable.
///
/// # Panics
///
/// If `shared` is 0 and `k` is not.
pub fn wide(k: u32, shared: u32) -> Sequent {
    let pairs: Vec<String> = (0..k)
        .map(|i| format!("(a{0} | b{0})", i / shared))
        .collect();
    let mut roots = vec![pairs.join(" * ")];
    roots.extend((0..k).map(|i| format!("~a{0} * ~b{0}", i / shared)));
    parse(&format!("|- {}", roots.join(", ")))
}

/// Returns the sequent `⊢ (a1 ⊗ b1) ⊕ 0, (~a1 ⊗ ~b1) ⊕ 0, …` of `k`
/// pairs, unprovable with or without Mix.
///
/// Every axiom linking of a pair closes a cycle through its two `⊗`. The
/// `⊕ 0` makes the sequent MALL, where the count equation that refutes
/// the bare tensors at once does not hold, so a search under Mix meets
/// the partitions of the context.
pub fn mix(k: u32) -> Sequent {
    let roots: Vec<String> = (0..k)
        .map(|i| format!("(a{i} * b{i}) + 0, (~a{i} * ~b{i}) + 0"))
        .collect();
    parse(&format!("|- {}", roots.join(", ")))
}

/// Returns a random closed 3-QBF encoded into MALL, and its truth value:
/// the sequent is provable in classical mode exactly when the QBF is
/// true.
///
/// The QBF has `n` variables quantified alternately, `∃` first, and
/// `⌈1.5 n⌉` clauses of three distinct variables with random signs. At
/// least two variables of each clause are existential (Gent and Walsh's
/// model A), which keeps random instances from being false outright. The
/// generator is seeded by `n` and `index`.
///
/// The encoding orders the quantifiers by locks and keys, `S` being the
/// rest of the formula:
/// - `∀x` is `(~tx & ~fx) ⅋ S`: each premise of the `&` puts one of the
///   two values into the context;
/// - `∃x` is `((~tx ⅋ ~kx) ⊕ (~fx ⅋ ~kx)) ⅋ (kx ⊗ S)`: `S` needs the key
///   `~kx`, which only the choice of a value releases, so the choice
///   comes before every later quantifier;
/// - the matrix is the `&` of the clauses, and a clause the `⊕` of `(tx ⊗
///   ⊤)` or `(fx ⊗ ⊤)` over its literals, provable when the context holds
///   a value that satisfies one of them.
///
/// # Panics
///
/// For fewer than three variables, which leave no clause to draw.
pub fn qbf(n: u32, index: u32) -> (Sequent, bool) {
    assert!(n >= 3, "a QBF of {n} variables has no clause of three");
    let mut rng = Rng((u64::from(n) << 32) | u64::from(index));
    let existential = |v: u32| v.is_multiple_of(2);
    let mut clauses: Vec<[(u32, bool); 3]> = Vec::new();
    while clauses.len() < (3 * n).div_ceil(2) as usize {
        let x = rng.below(n);
        let y = rng.below(n);
        let z = rng.below(n);
        let vars = [x, y, z];
        if x == y || y == z || x == z || vars.iter().filter(|&&v| existential(v)).count() < 2 {
            continue;
        }
        clauses.push(vars.map(|v| (v, rng.below(2) == 0)));
    }
    let valid = qbf_value(n, &clauses, &mut Vec::new());

    let literal =
        |(v, positive): (u32, bool)| format!("({}{v} * top)", if positive { "t" } else { "f" });
    let matrix: Vec<String> = clauses
        .iter()
        .map(|clause| format!("({})", clause.map(literal).join(" + ")))
        .collect();
    let mut formula = format!("({})", matrix.join(" & "));
    for v in (0..n).rev() {
        formula = if existential(v) {
            format!("(((~t{v} | ~k{v}) + (~f{v} | ~k{v})) | (k{v} * {formula}))")
        } else {
            format!("((~t{v} & ~f{v}) | {formula})")
        };
    }
    (parse(&format!("|- {formula}")), valid)
}

/// Returns the truth value of [`qbf`]'s formula with its first variables
/// set to `values`.
fn qbf_value(n: u32, clauses: &[[(u32, bool); 3]], values: &mut Vec<bool>) -> bool {
    let v = values.len() as u32;
    if v == n {
        return clauses
            .iter()
            .all(|clause| clause.iter().any(|&(x, sign)| values[x as usize] == sign));
    }
    let mut outcomes = [true, false].into_iter().map(|value| {
        values.push(value);
        let result = qbf_value(n, clauses, values);
        values.pop();
        result
    });
    if v.is_multiple_of(2) {
        outcomes.any(|b| b)
    } else {
        outcomes.all(|b| b)
    }
}

/// Returns six item sizes for two bins of size `b`, at least four, that
/// split into the triples `1, 2, b − 3`.
fn solvable_triples(b: u32) -> [u32; 6] {
    assert!(b >= 4, "bins of {b}");
    [1, 2, b - 3, 1, 2, b - 3]
}

/// Returns six item sizes for two bins of size `b`, at least four, that
/// admit no 3-Partition: the item `b − 1` needs two more items of total
/// one.
fn unsolvable_triples(b: u32) -> [u32; 6] {
    assert!(b >= 4, "bins of {b}");
    [1, 1, 1, b - 1, 1, b - 3]
}

/// Returns `n` random item sizes from 1 to `n`, with an even total, that
/// split into two halves of equal total exactly when `solvable` is true.
/// The generator, seeded from `n` and `solvable`, draws until the sizes
/// answer so.
fn partition_items(n: u32, solvable: bool) -> Vec<u32> {
    // One item never splits, so a solvable instance needs two. Two items
    // of size one or two with an even total always split, so an
    // unsolvable one needs three.
    assert!(n >= 2 + u32::from(!solvable), "{n} items");
    let mut rng = Rng((u64::from(n) << 1) | u64::from(solvable));
    loop {
        let sizes: Vec<u32> = (0..n).map(|_| 1 + rng.below(n)).collect();
        let total: u32 = sizes.iter().sum();
        if total % 2 == 1 {
            continue;
        }
        // Subset sums, as a bit per reachable sum.
        let mut reachable = vec![false; total as usize + 1];
        reachable[0] = true;
        for &s in &sizes {
            for sum in (s..=total).rev() {
                reachable[sum as usize] |= reachable[(sum - s) as usize];
            }
        }
        if reachable[total as usize / 2] == solvable {
            return sizes;
        }
    }
}

/// Returns a complete tree of depth `depth` from `level` down, over the
/// atoms `x<next>` numbered from `next` on, with `&` at even levels and
/// `⊕` at odd ones.
fn additive_tree(depth: u32, level: u32, next: &mut u32) -> String {
    if depth == level {
        *next += 1;
        return format!("x{}", *next - 1);
    }
    let left = additive_tree(depth, level + 1, next);
    let right = additive_tree(depth, level + 1, next);
    let connective = if level.is_multiple_of(2) { "&" } else { "+" };
    format!("({left} {connective} {right})")
}

/// A formula of the Lambek calculus, which [`lambek_first_order`]
/// translates.
#[derive(Clone, Debug)]
enum Lambek {
    /// An atom by its name.
    Atom(String),
    /// `B \ A`: the argument `B`, wanted on the left, and the result `A`.
    Under(Box<Lambek>, Box<Lambek>),
    /// `A / B`: the result `A` and the argument `B`, wanted on the right.
    Over(Box<Lambek>, Box<Lambek>),
    /// `A • B`.
    Product(Box<Lambek>, Box<Lambek>),
}

impl Lambek {
    /// Returns the atom `a<i>`.
    fn atom(i: u32) -> Self {
        Self::Atom(format!("a{i}"))
    }

    /// Returns the formula in the syntax of the Lambek calculus that this
    /// crate's ordered parse reads, every binary formula in parentheses.
    #[cfg(test)]
    fn text(&self) -> String {
        match self {
            Self::Atom(name) => name.clone(),
            Self::Under(b, a) => format!("({} \\ {})", b.text(), a.text()),
            Self::Over(a, b) => format!("({} / {})", a.text(), b.text()),
            Self::Product(a, b) => format!("({} * {})", a.text(), b.text()),
        }
    }

    /// Returns Moot and Piazza's translation of the formula at the string
    /// positions `x` to `y` into first-order MILL: an atom `p` is the
    /// predicate `p(x, y)`, `A / B` is `∀z.(B(y, z) ⊸ A(x, z))`, `B \ A` is
    /// `∀z.(B(z, x) ⊸ A(z, y))` and `A • B` is `∃z.(A(x, z) ⊗ B(z, y))`,
    /// each `z` a variable of its own (`v<n>`, numbered by `fresh`).
    fn first_order(&self, x: &str, y: &str, fresh: &mut u32) -> String {
        if let Self::Atom(name) = self {
            return format!("{name}({x}, {y})");
        }
        *fresh += 1;
        let z = format!("v{fresh}");
        match self {
            Self::Atom(_) => unreachable!("handled above"),
            Self::Over(a, b) => format!(
                "(forall {z}. ({} -o {}))",
                b.first_order(y, &z, fresh),
                a.first_order(x, &z, fresh)
            ),
            Self::Under(b, a) => format!(
                "(forall {z}. ({} -o {}))",
                b.first_order(&z, x, fresh),
                a.first_order(&z, y, fresh)
            ),
            Self::Product(a, b) => format!(
                "(exists {z}. ({} * {}))",
                a.first_order(x, &z, fresh),
                b.first_order(&z, y, fresh)
            ),
        }
    }
}

/// Returns the Lambek sequent of shape `index` at size `k`, at least two,
/// of the family `lambek-fo`, and whether it is provable.
///
/// The verdicts hold of the translations directly, by the positions
/// alone. The positions `i0`, …, `in` are distinct constants, and a
/// cut-free proof of multiplicative linear logic without units links
/// every atom of its conclusion's instance to one of the same predicate,
/// the same arguments and the opposite side:
///
/// - the chain is proved with every `∀` instantiated by `i0`, the chain
///   of `/` with every `∀` instantiated by the last position;
/// - the exchanged chain is not: `a0 \ a1` at `i0, i1` asks for `a0(z, i0)`,
///   and the only `a0` given stands at `i1, i2`;
/// - the product is proved with the positions as the witnesses;
/// - the reversed product is not: its leftmost atom `a(k−1)` stands at
///   `i0` and the only one given at `i(k−1)`.
fn lambek_shape(k: u32, index: u32) -> (Vec<Lambek>, Lambek, bool) {
    let under = |i: u32| Lambek::Under(Box::new(Lambek::atom(i)), Box::new(Lambek::atom(i + 1)));
    let product = |order: &mut dyn Iterator<Item = u32>| {
        let first = Lambek::atom(order.next().expect("two atoms"));
        order.fold(first, |left, i| {
            Lambek::Product(Box::new(left), Box::new(Lambek::atom(i)))
        })
    };
    match index {
        0 | 1 => {
            let mut hypotheses = vec![Lambek::atom(0)];
            hypotheses.extend((0..k).map(under));
            if index == 1 {
                hypotheses.swap(0, 1);
            }
            (hypotheses, Lambek::atom(k), index == 0)
        }
        2 => {
            let mut hypotheses: Vec<Lambek> = (1..=k)
                .rev()
                .map(|i| Lambek::Over(Box::new(Lambek::atom(i)), Box::new(Lambek::atom(i - 1))))
                .collect();
            hypotheses.push(Lambek::atom(0));
            (hypotheses, Lambek::atom(k), true)
        }
        3 => (
            (0..k).map(Lambek::atom).collect(),
            product(&mut (0..k)),
            true,
        ),
        _ => (
            (0..k).map(Lambek::atom).collect(),
            product(&mut (0..k).rev()),
            false,
        ),
    }
}

/// Returns the two-sided text of the Lambek sequent `hypotheses ⊢ goal`
/// in Moot and Piazza's translation: the `i`th hypothesis at the
/// positions `i` to `i + 1`, the goal from the first position to the
/// last, each position a constant `i<n>`.
fn lambek_first_order(hypotheses: &[Lambek], goal: &Lambek) -> String {
    let mut fresh = 0;
    let translated: Vec<String> = hypotheses
        .iter()
        .enumerate()
        .map(|(i, h)| h.first_order(&format!("i{i}"), &format!("i{}", i + 1), &mut fresh))
        .collect();
    let goal = goal.first_order("i0", &format!("i{}", hypotheses.len()), &mut fresh);
    format!("{} |- {goal}", translated.join(", "))
}

/// Returns `n` copies of `formula` joined by `separator`.
fn power(formula: &str, n: u32, separator: &str) -> String {
    vec![formula; n as usize].join(separator)
}

/// Parses a sequent a generator wrote, without a bound on its size.
fn parse(text: &str) -> Sequent {
    // A large size is the caller's to ask for: no bound on occurrences or
    // terms.
    let unbounded = crate::Limits::default()
        .with_occurrences(None)
        .with_terms(None);
    Sequent::parse_within(text, &unbounded)
        .unwrap_or_else(|e| panic!("a generated sequent parses: {e}"))
}

/// Returns an instance without a copy bound or a witness depth, its name
/// left for [`Family::instance`] to fill in.
fn instance(sequent: Sequent, mode: Mode, provable: bool) -> Instance {
    Instance {
        name: String::new(),
        sequent,
        mode,
        provable,
        copies: None,
        witness_depth: None,
    }
}

/// SplitMix64: a small seeded generator, so that a random instance is a
/// function of its seed.
struct Rng(u64);

impl Rng {
    /// Returns a number below `n`.
    fn below(&mut self, n: u32) -> u32 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        ((z ^ (z >> 31)) % u64::from(n)) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear::search::{Options, Verdict, prove};

    /// The smallest instances of every family are decided as the family
    /// says, and every proof checks; the random families are the same on
    /// every call. The unsolvable Horn 3-Partition is left out: refuting
    /// its smallest instance takes a minute, which is what the benchmarks
    /// measure.
    #[test]
    fn verdicts_as_constructed() {
        for family in FAMILIES.iter().filter(|f| f.name != "3-partition-no") {
            for index in 0..family.instances {
                let instance = family.instance(family.sizes[0], index).unwrap();
                let options = Options::default()
                    .with_copies(Some(instance.copies.unwrap_or(Options::DEFAULT_COPIES)))
                    .with_witness_depth(
                        instance
                            .witness_depth
                            .unwrap_or(Options::DEFAULT_WITNESS_DEPTH),
                    );
                let outcome = prove(&instance.sequent, instance.mode, &options).unwrap();
                match &outcome.verdict {
                    Verdict::Proved(proof) => {
                        assert!(instance.provable, "{} proved", instance.name);
                        proof.check(instance.mode).unwrap();
                    }
                    Verdict::Unprovable(_) => {
                        assert!(!instance.provable, "{} refuted", instance.name)
                    }
                    // An unprovable instance with a copy bound may end at
                    // the bound.
                    Verdict::Unknown(_) => assert!(
                        !instance.provable && instance.copies.is_some(),
                        "{}: {:?}",
                        instance.name,
                        outcome.verdict
                    ),
                }
                assert_eq!(
                    instance.sequent,
                    family.instance(family.sizes[0], index).unwrap().sequent
                );
            }
        }
    }

    /// The QBF encoding agrees with the formula's truth value on many
    /// small random instances, true and false ones alike.
    #[test]
    fn qbf_encoding() {
        let mut valid = 0;
        for n in 3..=7 {
            for index in 0..12 {
                let (sequent, expected) = qbf(n, index);
                let outcome = prove(&sequent, Mode::CLASSICAL, &Options::default()).unwrap();
                assert_eq!(
                    outcome.verdict.proof().is_some(),
                    expected,
                    "qbf({n}, {index})"
                );
                assert!(expected || matches!(outcome.verdict, Verdict::Unprovable(_)));
                valid += usize::from(expected);
            }
        }
        assert!((10..50).contains(&valid), "{valid} of 60 valid");
    }

    /// The Lambek sequents behind `lambek-fo` have the verdicts the
    /// family claims for their translations, as the planar search of the
    /// Lambek calculus decides them, and the translations are decided so
    /// by the first-order search: the translation keeps provability.
    #[test]
    fn lambek_translations_keep_the_planar_verdicts() {
        let family = find("lambek-fo").unwrap();
        for k in 2..=5 {
            for index in 0..family.instances {
                let (hypotheses, goal, provable) = lambek_shape(k, index);
                let written: Vec<String> = hypotheses.iter().map(Lambek::text).collect();
                let text = format!("{} |- {}", written.join(", "), goal.text());
                let original = Sequent::parse_ordered(
                    &text,
                    crate::linear::Order::Cyclic,
                    &crate::Limits::default(),
                )
                .unwrap();
                let planar = prove(&original, Mode::LAMBEK, &Options::default()).unwrap();
                assert_eq!(planar.verdict.proof().is_some(), provable, "{text}");
                let translated = family.instance(k, index).unwrap();
                let first_order =
                    prove(&translated.sequent, translated.mode, &Options::default()).unwrap();
                assert_eq!(
                    first_order.verdict.proof().is_some(),
                    provable,
                    "{}",
                    translated.sequent
                );
            }
        }
    }

    /// A size a family has no instance of is refused, never a panic.
    #[test]
    fn refuses_sizes_without_instances() {
        for (name, size, least, powers_of_two) in [
            ("qbf", 2, 3, false),
            ("wide-m1", 0, 1, false),
            ("3-partition-no", 3, 4, false),
            ("partition-no", 2, 3, false),
            ("counter", 6, 0, true),
        ] {
            let family = find(name).unwrap();
            assert_eq!(
                family.instance(size, 0).map(|i| i.name),
                Err(Error::FamilySize {
                    family: family.name,
                    size,
                    least,
                    powers_of_two
                }),
                "{name}"
            );
        }
        // Every family answers every size from 0 up past its least with an
        // instance or this refusal, never a panic: a wrong `least` was one.
        for family in FAMILIES {
            for size in 0..=family.least + 2 {
                let instance = family.instance(size, 0);
                assert!(
                    matches!(instance, Ok(_) | Err(Error::FamilySize { .. })),
                    "{} {size}",
                    family.name
                );
            }
        }
    }
}
