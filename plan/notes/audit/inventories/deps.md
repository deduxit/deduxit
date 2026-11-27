# Which module names which (`core/src`, test files apart)

A cell counts the files of the row's module that name a path of the column's, each path once per file.

| names → | `(root)` | `errors` | `export` | `first_order` | `goal_tree` | `hash` | `kripke` | `lambda` | `limits` | `linear` | `modal` | `name` | `ordinary` | `protocol` | `rules` | `sealed` | `serialize` | `settings` | `unify` | `wire` | `workspace` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `(root)` |  |  |  |  |  |  |  |  | 1 | 7 |  |  |  |  |  |  |  |  |  |  |  |
| `errors` |  |  | 2 |  |  |  |  | 2 | 6 | 41 |  | 1 | 11 |  |  | 1 | 1 |  |  |  |  |
| `export` | 28 |  |  |  |  | 6 | 3 | 11 | 33 | 129 | 8 |  | 34 |  | 10 |  |  |  |  |  |  |
| `first_order` | 1 |  |  |  |  | 1 |  |  | 1 |  |  | 1 |  |  |  |  |  |  |  |  |  |
| `goal_tree` |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |
| `hash` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `kripke` | 1 |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |
| `lambda` | 17 | 1 |  |  |  | 15 |  |  | 40 | 21 |  | 3 | 17 |  |  |  | 1 |  |  |  |  |
| `limits` |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |
| `linear` | 90 | 30 | 13 | 10 | 6 | 40 |  | 2 | 138 | 738 |  | 5 | 3 |  |  | 2 | 3 |  | 4 | 2 |  |
| `modal` | 8 | 1 | 5 |  |  | 4 | 4 |  | 20 | 7 |  | 3 | 1 |  | 13 |  | 7 |  |  |  |  |
| `name` | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `ordinary` | 86 | 3 | 16 | 86 | 6 | 59 | 16 | 4 | 173 | 129 |  | 8 | 116 |  | 10 |  | 1 |  | 3 | 7 |  |
| `protocol` | 24 |  | 32 |  |  |  | 1 | 8 |  | 21 | 8 |  | 23 |  | 10 |  | 1 |  |  | 2 |  |
| `rules` | 24 | 1 |  |  |  | 11 |  |  | 17 | 10 |  | 1 | 25 |  | 7 |  | 5 |  |  | 2 |  |
| `sealed` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `serialize` | 28 |  |  | 4 |  | 2 | 2 | 16 | 31 | 110 | 14 | 2 | 72 |  | 16 | 12 |  |  |  | 24 |  |
| `settings` | 3 |  | 1 |  |  |  |  | 2 |  | 8 |  |  | 1 |  |  |  | 1 |  |  |  |  |
| `unify` |  |  |  | 5 |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |
| `wire` | 1 |  |  | 1 |  |  |  |  | 1 | 6 |  |  |  |  |  | 1 |  |  |  |  |  |
| `workspace` | 9 |  |  |  |  | 1 |  |  | 6 | 1 |  |  |  | 14 |  | 1 |  |  |  | 1 |  |

## Between the suites


### `ordinary` names `linear`: 62 paths

- `linear::search::Reason`: 12 files
- `linear::search::generate::Rng`: 12 files
- `linear::search`: 11 files
- `linear::proofs::InfId`: 9 files
- `linear::search::memory::Account`: 7 files
- `linear::proofs::ViewOptions`: 3 files
- `linear::proofs::Structural`: 3 files
- `linear::search::Options::default`: 3 files
- `linear::occurrences::instances::meta_name`: 3 files
- `linear::proofs::GoalId`: 3 files
- `linear::sequents::Stop`: 3 files
- `linear::sequents::Stops`: 3 files
- `linear::Proof`: 2 files
- `linear::prove`: 2 files
- `linear::proofs::Size`: 2 files
- `linear::Verdict`: 2 files
- `linear::Options`: 2 files
- `linear::sequents::notation::TEXT`: 2 files
- `linear::search::generate::Rng::new`: 2 files
- `linear::Refutation::Classical`: 1 files
- `linear::search::Reason::WorkLimit`: 1 files
- `linear::proofs::style::plain`: 1 files
- `linear::occurrences::instances::Terms`: 1 files
- `linear::proofs::Sides`: 1 files
- `linear::Member::from`: 1 files
- `linear::proofs::fmt::write_steps`: 1 files
- `linear::proofs::TextOptions::default`: 1 files
- `linear::proofs::fmt::write_text`: 1 files
- `linear::occurrences::Member`: 1 files
- `linear::proofs::Compact`: 1 files
- `linear::search::Goal::new`: 1 files
- `linear::proofs::Labels`: 1 files
- `linear::proofs::Rule`: 1 files
- `linear::proofs::style::Drawn`: 1 files
- `linear::search::prove_goal`: 1 files
- `linear::proofs::fmt::text_size`: 1 files
- `linear::Derivation`: 1 files
- `linear::Instances`: 1 files
- `linear::Forest::new`: 1 files
- `linear::sequents::notation::Notation`: 1 files
- `linear::proofs::TextOptions`: 1 files
- `linear::occurrences::OccId`: 1 files
- `linear::search::Reason::Incomplete`: 1 files
- `linear::proofs::interactive::Inverted`: 1 files
- `linear::Options::default`: 1 files
- `linear::fragment::Mode`: 1 files
- `linear::fragment::Fragment`: 1 files
- `linear::lltp::clauses`: 1 files
- `linear::lltp::Status`: 1 files
- `linear::lltp::Clauses`: 1 files
- `linear::search::Disproof`: 1 files
- `linear::search::Classical`: 1 files
- `linear::search::Refutation`: 1 files
- `linear::Forest::MOST`: 1 files
- `linear::sequents::fo::FirstOrder`: 1 files
- `linear::sequents::TermId`: 1 files
- `linear::Sequent`: 1 files
- `linear::Mode`: 1 files
- `linear::sequents::Atom`: 1 files
- `linear::sequents::fo::FirstOrder::new`: 1 files

### `ordinary` names `lambda`: 4 paths

- `lambda::Term`: 1 files
- `lambda::simple::read`: 1 files
- `lambda::InhabitOptions::default`: 1 files
- `lambda::enumerate`: 1 files

### `ordinary` names `rules`: 8 paths

- `rules::Derivation::from_ordinary`: 2 files
- `rules::Calculus`: 2 files
- `rules::TABLEAU`: 1 files
- `rules::Step`: 1 files
- `rules::Node`: 1 files
- `rules::TermId`: 1 files
- `rules::terms_of`: 1 files
- `rules::Derivation`: 1 files

### `linear` names `ordinary`: 3 paths

- `ordinary::Symbols`: 1 files
- `ordinary::NodeId`: 1 files
- `ordinary::Formulas`: 1 files

### `linear` names `lambda`: 2 paths

- `lambda::Term`: 1 files
- `lambda::linear::read`: 1 files

### `modal` names `ordinary`: 1 paths

- `ordinary::MODEL_CELLS`: 1 files

### `modal` names `linear`: 4 paths

- `linear::search::Reason`: 2 files
- `linear::sequents::Stops`: 2 files
- `linear::sequents::Stop`: 2 files
- `linear::search::memory::Account`: 1 files

### `modal` names `rules`: 11 paths

- `rules`: 2 files
- `rules::TermId`: 2 files
- `rules::Fixity`: 1 files
- `rules::Associativity`: 1 files
- `rules::Discipline`: 1 files
- `rules::Step`: 1 files
- `rules::Term`: 1 files
- `rules::NodeId`: 1 files
- `rules::NodeRef`: 1 files
- `rules::Terms`: 1 files
- `rules::Derivation`: 1 files

### `lambda` names `ordinary`: 14 paths

- `ordinary::Formulas`: 2 files
- `ordinary::NodeId`: 2 files
- `ordinary::Logic`: 2 files
- `ordinary::Method`: 1 files
- `ordinary::Sequent`: 1 files
- `ordinary`: 1 files
- `ordinary::Verdict`: 1 files
- `ordinary::Node`: 1 files
- `ordinary::Side`: 1 files
- `ordinary::REFUSED`: 1 files
- `ordinary::Derivation`: 1 files
- `ordinary::Node::False`: 1 files
- `ordinary::Node::Implies`: 1 files
- `ordinary::Rule`: 1 files

### `lambda` names `linear`: 19 paths

- `linear::sequents::notation::Atoms`: 2 files
- `linear::proofs::InfId`: 2 files
- `linear::sequents::notation::Atoms::Plain`: 1 files
- `linear::sequents::notation::Notation`: 1 files
- `linear::InfId`: 1 files
- `linear::proofs::Labels`: 1 files
- `linear::proofs::style::Drawn`: 1 files
- `linear::search::Reason`: 1 files
- `linear::Options::default`: 1 files
- `linear::occurrences::Member`: 1 files
- `linear::occurrences::OccId`: 1 files
- `linear::Mode::is_ordered`: 1 files
- `linear::proofs::Inference`: 1 files
- `linear::Mode::has_mix`: 1 files
- `linear::proofs::Derivation`: 1 files
- `linear::occurrences::IllConnective`: 1 files
- `linear::occurrences::Reading`: 1 files
- `linear::occurrences::Side`: 1 files
- `linear::proofs::Rule`: 1 files

### `rules` names `ordinary`: 17 paths

- `ordinary`: 3 files
- `ordinary::Rule`: 3 files
- `ordinary::Formulas`: 2 files
- `ordinary::Side`: 2 files
- `ordinary::Logic`: 2 files
- `ordinary::Verdict`: 2 files
- `ordinary::Node`: 1 files
- `ordinary::NodeId`: 1 files
- `ordinary::Options`: 1 files
- `ordinary::Sequent`: 1 files
- `ordinary::REFUSED`: 1 files
- `ordinary::Inference`: 1 files
- `ordinary::Method`: 1 files
- `ordinary::Evidence`: 1 files
- `ordinary::Tableau`: 1 files
- `ordinary::Sign`: 1 files
- `ordinary::End`: 1 files

### `rules` names `linear`: 8 paths

- `linear::sequents::notation::Notation`: 3 files
- `linear::proofs::style::LITERAL`: 1 files
- `linear::InfId`: 1 files
- `linear::proofs::style::superscript`: 1 files
- `linear::proofs::style::RAISED`: 1 files
- `linear::proofs::Labels`: 1 files
- `linear::proofs::style::Drawn`: 1 files
- `linear::search`: 1 files

## What the shared modules name of a suite


### `(root)` names `linear`: 7 paths

- `linear::nets::NetError`: 1 files
- `linear::occurrences::OccId`: 1 files
- `linear::proofs::cut::Options`: 1 files
- `linear::nets::VertexId`: 1 files
- `linear::Mode`: 1 files
- `linear::Instances`: 1 files
- `linear::nets::cut::Step`: 1 files

### `errors` names `ordinary`: 11 paths

- `ordinary::Logic::Intuitionistic`: 1 files
- `ordinary::Logic`: 1 files
- `ordinary::negative::Translation::Kuroda`: 1 files
- `ordinary::Logic::Classical`: 1 files
- `ordinary::Method`: 1 files
- `ordinary::Method::Ljt`: 1 files
- `ordinary::Translation`: 1 files
- `ordinary::negative::Translation`: 1 files
- `ordinary::Logic::Minimal`: 1 files
- `ordinary::Translation::Affine`: 1 files
- `ordinary::StepError`: 1 files

### `errors` names `linear`: 36 paths

- `linear::nets::NetError`: 2 files
- `linear::proofs::CheckError`: 2 files
- `linear::occurrences::Member`: 2 files
- `linear::occurrences::ShapeError`: 2 files
- `linear::occurrences::OccId`: 2 files
- `linear::Interactive::mode`: 1 files
- `linear::occurrences::instances::Framing`: 1 files
- `linear::nets::MellStructure`: 1 files
- `linear::occurrences::Instances`: 1 files
- `linear::nets::ProofStructure`: 1 files
- `linear::Interactive::forest`: 1 files
- `linear::proofs::Derivation`: 1 files
- `linear::occurrences::Forest`: 1 files
- `linear::proofs::Proof`: 1 files
- `linear::occurrences::Reading`: 1 files
- `linear::Mode`: 1 files
- `linear::Interactive`: 1 files
- `linear::sequents::notation::TEXT`: 1 files
- `linear::proofs::Node`: 1 files
- `linear::fragment::Mode`: 1 files
- `linear::proofs::Fault`: 1 files
- `linear::search::Engine`: 1 files
- `linear::search::RefutationFault::Uncertified`: 1 files
- `linear::sequents::fo::SymbolFault`: 1 files
- `linear::proofs::StepError`: 1 files
- `linear::proofs::Refused`: 1 files
- `linear::fragment::Fragment`: 1 files
- `linear::search::NotTaken`: 1 files
- `linear::proofs::interactive::GoalId::new`: 1 files
- `linear::proofs::StepError::NoGoal`: 1 files
- `linear::proofs::NodeId`: 1 files
- `linear::search::RefutationFault::Satisfied`: 1 files
- `linear::search::RefutationFault`: 1 files
- `linear::nets::Placement::Premise`: 1 files
- `linear::nets::VertexId::new`: 1 files
- `linear::proofs::Invalid`: 1 files

### `errors` names `lambda`: 2 paths

- `lambda::NoTerm::Classical`: 1 files
- `lambda::NoTerm`: 1 files

### `export` names `ordinary`: 17 paths

- `ordinary::FiniteCountermodel`: 5 files
- `ordinary::KripkeCountermodel`: 5 files
- `ordinary::Logic`: 5 files
- `ordinary::Symbols`: 3 files
- `ordinary::Node`: 2 files
- `ordinary::NodeId`: 2 files
- `ordinary::Countermodel`: 2 files
- `ordinary::Derivation`: 1 files
- `ordinary::Sequent`: 1 files
- `ordinary::KripkeStructure`: 1 files
- `ordinary::rocq::not_valid`: 1 files
- `ordinary::rocq::countermodel`: 1 files
- `ordinary::rocq_models::structure`: 1 files
- `ordinary::Image`: 1 files
- `ordinary::rocq::write`: 1 files
- `ordinary::rocq_models::kripke_structure`: 1 files
- `ordinary::REFUSED`: 1 files

### `export` names `linear`: 79 paths

- `linear::Mode`: 5 files
- `linear::proofs::OpenGoal`: 4 files
- `linear::sequents::Sequent`: 4 files
- `linear::proofs::style::Drawn`: 4 files
- `linear::sequents::notation::Notation`: 4 files
- `linear::proofs::InfId`: 4 files
- `linear::occurrences::Reading`: 4 files
- `linear::occurrences::OccId`: 4 files
- `linear::sequents::notation::Atoms`: 3 files
- `linear::proofs::Labels`: 3 files
- `linear::proofs::style::parts`: 3 files
- `linear::proofs::style::Part`: 3 files
- `linear::occurrences::Side`: 3 files
- `linear::nets::NetError`: 3 files
- `linear::nets::VertexId`: 3 files
- `linear::Forest`: 2 files
- `linear::sequents::Stop`: 2 files
- `linear::proofs::Derivation`: 2 files
- `linear::proofs::Rule`: 2 files
- `linear::occurrences::Forest`: 2 files
- `linear::sequents::Stops`: 2 files
- `linear::sequents::Term`: 2 files
- `linear::Mode::INTUITIONISTIC`: 2 files
- `linear::nets::MellStructure`: 2 files
- `linear::sequents::Kind`: 2 files
- `linear::nets::ProofStructure`: 2 files
- `linear::TwoSided`: 1 files
- `linear::Sequent`: 1 files
- `linear::occurrences::Reading::of_mode`: 1 files
- `linear::Derivation`: 1 files
- `linear::Forest::within`: 1 files
- `linear::proofs::TextOptions`: 1 files
- `linear::InfId::new`: 1 files
- `linear::occurrences::Sign`: 1 files
- `linear::occurrences::instances::Framing`: 1 files
- `linear::sequents::FoTermId`: 1 files
- `linear::search::Disproof`: 1 files
- `linear::occurrences::FrameId`: 1 files
- `linear::occurrences::Member`: 1 files
- `linear::sequents::FoTerm`: 1 files

### `export` names `modal`: 7 paths

- `modal::Countermodel`: 2 files
- `modal::Outcome`: 1 files
- `modal::rocq::countermodel`: 1 files
- `modal::rocq::outcome`: 1 files
- `modal::rocq::derivation`: 1 files
- `modal::NodeId`: 1 files
- `modal::Node`: 1 files

### `export` names `lambda`: 8 paths

- `lambda::Options`: 4 files
- `lambda::print::LATEX`: 1 files
- `lambda::NoTerm::Calculus`: 1 files
- `lambda::Drawing`: 1 files
- `lambda::NoTerm::Classical`: 1 files
- `lambda::print::Spelling`: 1 files
- `lambda::print::TYPST`: 1 files
- `lambda::print::SVG`: 1 files

### `export` names `rules`: 6 paths

- `rules::Fitch`: 4 files
- `rules::Fitch::write_cites`: 2 files
- `rules::Drawing`: 1 files
- `rules::FitchOptions`: 1 files
- `rules::Derivation`: 1 files
- `rules::fitch::Cite`: 1 files

### `goal_tree` names `linear`: 1 paths

- `linear::proofs::InfId`: 1 files

### `protocol` names `ordinary`: 19 paths

- `ordinary::Logic`: 3 files
- `ordinary::Sequent`: 2 files
- `ordinary::Derivation`: 2 files
- `ordinary::Sequent::parse_within`: 1 files
- `ordinary::negative`: 1 files
- `ordinary::normal`: 1 files
- `ordinary::interpolant::Partition`: 1 files
- `ordinary::resolution`: 1 files
- `ordinary::Method`: 1 files
- `ordinary::herbrand::Options`: 1 files
- `ordinary`: 1 files
- `ordinary::resolution::first_order`: 1 files
- `ordinary::Translation`: 1 files
- `ordinary::interpolant`: 1 files
- `ordinary::resolution::first_order::Step`: 1 files
- `ordinary::Outcome`: 1 files
- `ordinary::herbrand::herbrand`: 1 files
- `ordinary::resolution::first_order::Answer`: 1 files
- `ordinary::Interactive`: 1 files

### `protocol` names `linear`: 20 paths

- `linear::Mode`: 2 files
- `linear::search::Outcome`: 1 files
- `linear::nets::MellStructure`: 1 files
- `linear::GoalId`: 1 files
- `linear::proofs::derivation::Compact::Never`: 1 files
- `linear::nets::ProofStructure`: 1 files
- `linear::Sequent`: 1 files
- `linear::occurrences::reading::Side`: 1 files
- `linear::proofs::cut::eliminate`: 1 files
- `linear::nets::Criterion`: 1 files
- `linear::Interactive`: 1 files
- `linear::search::Verdict`: 1 files
- `linear::prove_within`: 1 files
- `linear::IllSequent`: 1 files
- `linear::Proof`: 1 files
- `linear::IllSequent::parse_within`: 1 files
- `linear::Sequent::parse_within`: 1 files
- `linear::Sequent::parse_ordered`: 1 files
- `linear::GoalId::new`: 1 files
- `linear::proofs::derivation::Layout::Fitch`: 1 files

### `protocol` names `modal`: 7 paths

- `modal::System`: 2 files
- `modal::Sequent::parse_within`: 1 files
- `modal::Evidence`: 1 files
- `modal::Sequent`: 1 files
- `modal`: 1 files
- `modal::Outcome`: 1 files
- `modal::Verdict`: 1 files

### `protocol` names `lambda`: 7 paths

- `lambda::Term`: 2 files
- `lambda::Inhabitation`: 1 files
- `lambda::Typing`: 1 files
- `lambda`: 1 files
- `lambda::ReduceOptions`: 1 files
- `lambda::Calculus`: 1 files
- `lambda::InhabitOptions`: 1 files

### `protocol` names `rules`: 8 paths

- `rules::Drawing::new`: 2 files
- `rules::Derivation`: 2 files
- `rules::Interactive`: 1 files
- `rules::Calculus`: 1 files
- `rules::LK`: 1 files
- `rules::LM`: 1 files
- `rules::Fitch::new`: 1 files
- `rules::LJ`: 1 files

### `serialize` names `ordinary`: 69 paths

- `ordinary::Sequent`: 2 files
- `ordinary::resolution::Method`: 2 files
- `ordinary::resolution::Bound`: 2 files
- `ordinary::Side`: 1 files
- `ordinary::Tableau`: 1 files
- `ordinary::FiniteCountermodel`: 1 files
- `ordinary::sat::Statistics`: 1 files
- `ordinary::TruthTable`: 1 files
- `ordinary::truth_table`: 1 files
- `ordinary::interpolant::Partition`: 1 files
- `ordinary::Search`: 1 files
- `ordinary::KripkeCountermodel`: 1 files
- `ordinary::sat::ClausalProof`: 1 files
- `ordinary::FiniteCountermodel::new`: 1 files
- `ordinary::Logic`: 1 files
- `ordinary::Signed`: 1 files
- `ordinary::Countermodel`: 1 files
- `ordinary::resolution::Method::Saturation`: 1 files
- `ordinary::NodeId`: 1 files
- `ordinary::Inference`: 1 files
- `ordinary::negative::Agreement`: 1 files
- `ordinary::closed`: 1 files
- `ordinary::normal::Claim`: 1 files
- `ordinary::normal::Form`: 1 files
- `ordinary::bound`: 1 files
- `ordinary::Structure`: 1 files
- `ordinary::negative::Comparison`: 1 files
- `ordinary::models::Statistics`: 1 files
- `ordinary::Verdict`: 1 files
- `ordinary::TableauReview`: 1 files
- `ordinary::TableauNode`: 1 files
- `ordinary::connection::Statistics`: 1 files
- `ordinary::Translation`: 1 files
- `ordinary::KripkeStructure`: 1 files
- `ordinary::Formulas`: 1 files
- `ordinary::Node`: 1 files
- `ordinary::normal::Promise`: 1 files
- `ordinary::End`: 1 files
- `ordinary::negative::Translation`: 1 files
- `ordinary::Evidence`: 1 files

### `serialize` names `linear`: 87 paths

- `linear::fragment::Mode`: 3 files
- `linear::occurrences::Member`: 3 files
- `linear::occurrences::Forest`: 3 files
- `linear::Sequent::try_from`: 3 files
- `linear::occurrences::OccId`: 3 files
- `linear::nets::VertexId`: 2 files
- `linear::sequents::Sequent`: 2 files
- `linear::sequents::Atom`: 2 files
- `linear::sequents::Term`: 2 files
- `linear::occurrences::Side`: 2 files
- `linear::sequents::TermId`: 2 files
- `linear::proofs::Named`: 2 files
- `linear::proofs::InfId`: 2 files
- `linear::search::Reason`: 2 files
- `linear::proofs::cut::Case`: 2 files
- `linear::proofs::cut::End`: 2 files
- `linear::search::Verdict`: 2 files
- `linear::sequents::Symbol`: 2 files
- `linear::nets::NetError`: 1 files
- `linear::proofs::Dyadic`: 1 files
- `linear::search::NotTaken`: 1 files
- `linear::nets::Placement`: 1 files
- `linear::proofs::Fault`: 1 files
- `linear::occurrences::ShapeError`: 1 files
- `linear::proofs::CheckError`: 1 files
- `linear::sequents::Visit`: 1 files
- `linear::occurrences::IllConnective`: 1 files
- `linear::occurrences::IllSequent`: 1 files
- `linear::proofs::interactive::Parts`: 1 files
- `linear::proofs::interactive::PartTerm`: 1 files
- `linear::proofs::interactive::Interactive`: 1 files
- `linear::proofs::Rule`: 1 files
- `linear::occurrences::Written`: 1 files
- `linear::proofs::Inference`: 1 files
- `linear::nets::MellStructure`: 1 files
- `linear::nets::cut::Step`: 1 files
- `linear::nets::ProofStructure`: 1 files
- `linear::nets::Placed`: 1 files
- `linear::nets::Criterion`: 1 files
- `linear::nets::cut::Elimination`: 1 files

### `serialize` names `modal`: 14 paths

- `modal::Evidence`: 1 files
- `modal::Sequent`: 1 files
- `modal::Outcome`: 1 files
- `modal::System`: 1 files
- `modal::Statistics`: 1 files
- `modal::Countermodel`: 1 files
- `modal::NodeId`: 1 files
- `modal::bound`: 1 files
- `modal::Formulas`: 1 files
- `modal::Node`: 1 files
- `modal::Verdict`: 1 files
- `modal::calculus`: 1 files
- `modal::decide_within`: 1 files
- `modal::Options`: 1 files

### `serialize` names `lambda`: 16 paths

- `lambda::Term`: 1 files
- `lambda::Inhabitation`: 1 files
- `lambda::Variable`: 1 files
- `lambda::Strategy`: 1 files
- `lambda::Node`: 1 files
- `lambda::End`: 1 files
- `lambda::read::FORM`: 1 files
- `lambda::Calculus`: 1 files
- `lambda::TermId`: 1 files
- `lambda::VarId`: 1 files
- `lambda::read::Parts`: 1 files
- `lambda::Reduction`: 1 files
- `lambda::Inhabitants`: 1 files
- `lambda::Type`: 1 files
- `lambda::TypeId`: 1 files
- `lambda::Count`: 1 files

### `serialize` names `rules`: 16 paths

- `rules::Step`: 1 files
- `rules::Interactive`: 1 files
- `rules::Interactive::from_parts`: 1 files
- `rules::interactive::Goal`: 1 files
- `rules::Term`: 1 files
- `rules::interactive::Scope`: 1 files
- `rules::FORMULA`: 1 files
- `rules::Node`: 1 files
- `rules::Discipline`: 1 files
- `rules::Calculus`: 1 files
- `rules::Fixity`: 1 files
- `rules::Associativity`: 1 files
- `rules::TermId`: 1 files
- `rules::interactive::Done`: 1 files
- `rules::Terms`: 1 files
- `rules::Derivation`: 1 files

### `settings` names `ordinary`: 1 paths

- `ordinary`: 1 files

### `settings` names `linear`: 8 paths

- `linear::search::batch`: 1 files
- `linear::ViewOptions`: 1 files
- `linear::proofs::interactive`: 1 files
- `linear::search::Schedule`: 1 files
- `linear::InputOptions`: 1 files
- `linear::proofs::cut`: 1 files
- `linear::search::Jobs`: 1 files
- `linear::search::Options`: 1 files

### `settings` names `lambda`: 2 paths

- `lambda::Options`: 1 files
- `lambda::Options::default`: 1 files

### `unify` names `linear`: 1 paths

- `linear::occurrences::instances::Terms`: 1 files

### `wire` names `linear`: 6 paths

- `linear::search::Refutation::Classical`: 1 files
- `linear::search::Refutation`: 1 files
- `linear::Mode`: 1 files
- `linear::Sequent`: 1 files
- `linear::search::Refutation::Basis`: 1 files
- `linear::search::Refutation::Saturated`: 1 files

### `workspace` names `linear`: 1 paths

- `linear::search::Reason::NAMES`: 1 files

## Inside the linear and the ordinary suite


### `linear`

| names → | `families` | `fragment` | `lltp` | `mist` | `nets` | `occurrences` | `parse` | `proofs` | `prove` | `prove_within` | `search` | `sequents` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `families` |  |  |  |  |  |  |  |  |  |  | 3 |  |
| `fragment` |  |  |  |  |  |  |  |  |  |  |  | 4 |
| `lltp` |  |  |  |  |  |  |  |  |  |  |  |  |
| `mist` |  |  |  |  |  | 1 |  |  |  |  |  | 3 |
| `nets` |  | 11 |  |  |  | 29 |  | 18 |  |  | 11 | 20 |
| `occurrences` |  |  |  |  | 4 |  | 2 | 3 |  |  | 9 | 21 |
| `parse` |  | 1 |  |  |  | 3 |  |  |  |  |  | 9 |
| `proofs` |  | 13 |  |  |  | 132 |  |  | 3 |  | 33 | 71 |
| `prove` |  |  |  |  |  |  |  |  |  |  |  |  |
| `prove_within` |  |  |  |  |  |  |  |  |  |  |  |  |
| `search` | 9 | 23 |  |  | 17 | 94 |  | 39 |  |  |  | 46 |
| `sequents` |  |  |  |  |  | 9 |  | 1 |  |  |  |  |

### `ordinary`

| names → | `bound` | `bruijn` | `closed` | `connection` | `decide` | `decide_within` | `derivation` | `equality` | `grounding` | `herbrand` | `instance` | `interactive` | `interpolant` | `invert` | `kripke_search` | `kripke_structure` | `ljt` | `model` | `models` | `negative` | `normal` | `parse` | `read_tptp` | `resolution` | `rocq` | `rocq_models` | `sat` | `structure` | `table` | `tableau` | `tests` | `transfer` | `translate` | `truth_table` | `written` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `bound` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `bruijn` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `closed` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `connection` |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |
| `decide` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |
| `decide_within` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `derivation` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `equality` |  |  |  | 1 |  | 1 |  |  | 1 |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `grounding` |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `herbrand` |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `instance` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `interactive` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `interpolant` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `invert` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `kripke_search` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `kripke_structure` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `ljt` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `model` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `models` |  |  |  | 1 | 1 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `negative` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `normal` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `parse` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `read_tptp` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `resolution` |  |  |  |  |  | 1 |  | 2 |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  | 6 |  |  |  |  |  |  |  |  |
| `rocq` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `rocq_models` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `sat` |  |  |  |  |  | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 |  |  |  |  |  |  |  | 2 |  |  |  |  |
| `structure` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `table` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `tableau` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `tests` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `transfer` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `translate` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `truth_table` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| `written` |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
