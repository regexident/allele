# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Please make sure to add your changes to the appropriate categories:

- `Added`: for new functionality
- `Changed`: for changes in existing functionality
- `Deprecated`: for soon-to-be removed functionality
- `Removed`: for removed functionality
- `Fixed`: for fixed bugs
- `Performance`: for performance-relevant changes
- `Security`: for security-relevant changes
- `Other`: for everything else

## [Unreleased]

### Added

- Added `InvalidArgumentError` type (re-exported from crate root) returned by constructors with out-of-range parameter values

### Changed

- `TournamentSelector::new` now returns `Result<Self, InvalidArgumentError>` instead of panicking on invalid `probability`
- `ElitistReinserter::new` and `UniformReinserter::new` now return `Result<Self, InvalidArgumentError>` instead of panicking on invalid parameters
- Replaced `Rc<Vec<G>>` with `Arc<Vec<G>>` in `EvaluatedPopulation` and `GeneticAlgorithm`, making both types `Send`; `EvaluatedPopulation::new` and `EvaluatedPopulation::individuals()` now use `Arc`
- `GeneticOperator::name()` now returns `Cow<'static, str>` instead of `String`, eliminating a heap allocation per call
- Bumped MSRV from "1.80.0" to "1.85.1"
- Bumped Rust edition from “2021” to “2024”
- Replaced the `chrono` dependency with `std::time`; `State::started_at` is now an `Instant`, and `State::duration`, `SimResult::Final`'s duration, `ProcessingTime`, and `TimeLimit` now use `std::time::Duration`
- Renamed `GeneticAlgorithm` intermediate builder structs to use full operator words for readability in compiler errors

### Deprecated

- n/a

### Removed

- Removed `BestSolution::found_at` field; timestamp-of-discovery is not part of the optimality result

### Fixed

- Eliminated dead `Instant::now()` initialization of `Simulator::started_at` at build time; start time is now recorded only when the simulation begins running
- Fixed `UniformReinserter` sampling old population with replacement; both offspring and old-population picks are now without replacement
- Replaced unbounded rejection-sampling loop in `random_cut_points_from_range` with O(1) direct construction
- Fixed `BreederValueMutation` on unsigned types producing asymmetric boundary behavior when the intermediate arithmetic result is negative; both bounds now clamp symmetrically via `i128` intermediate
- Fixed `random_n_cut_points` end-accumulation bug where the sampling range could exceed `length` for n > 2
- `FitnessLimit` now works with any `Algorithm` whose output implements `OptimizationResult`, not just the concrete 7-parameter `GeneticAlgorithm`; implemented `OptimizationResult` on `ga::State`

### Performance

- Replaced `Vec::remove(index)` with `swap_remove` in `UniformReinserter::combine`, removing O(n²) behavior
- Replaced `Vec::remove(0)` with an index cursor in `ElitistReinserter::combine`, removing O(n²) behavior in the `!offspring_has_precedence` path
- Replaced `Vec::remove(0)` with `VecDeque::pop_front()` and `swap_remove` in `TournamentSelector::select_from`, removing O(n²) behavior
- Optimized `order_one_crossover` with `HashSet` membership and `VecDeque`
- Used `HashMap` index for O(1) removal in `TournamentSelector`

### Security

- n/a

### Other

- Documented `par_evaluate_fitness` panic precondition for empty populations

## [0.8.0] - 2026-08-02 (https://crates.io/crates/allele/0.8.0)

### Changed

- Made `rayon` dependency optional behind "parallel" feature
- Renamed crate from `genevo` to `allele`
- Added validation that `replace_ratio`, `selection_ratio`, and `mutation_rate` are within `[0.0, 1.0]` and added the public `number_of_mutations` helper
- `ProcessingTime` now formats durations human-readably through `std::fmt::Display` via `humantime`

### Removed

- Removed the empty `selection::ranking` module
- Removed the custom `Display` trait and the `types::fmt` module

### Fixed

- Fixed `UniformReinserter` silently dropping every other offspring when combining the new population
- Fixed `TournamentSelector` panicking on invalid probabilities and looping indefinitely on some configurations
- Fixed `UniversalSamplingSelector` pointer exceeding the roulette wheel sum and documented the non-negative fitness requirement
- Fixed `MultiPointCrossBreeder` hanging indefinitely when breeding single-parent tuples
- Fixed parallel `PopulationBuilder` passing local indices to `build_genome` instead of global ones
- Fixed `GeneticAlgorithm` panicking on empty parents and missing best fitness values; `determine_best_solution` now returns a `Result`
- Fixed signed `Fitness::abs_diff` overflow and restricted the blanket encoding marker implementations

## [0.7.1] - 2022-03-13 (https://crates.io/crates/genevo/0.7.1)

### Fixed

- Fix issue #23 Future compile error on Rust beta 1.60 in types/tests.rs

## [0.7.0] - 2021-11-07 (https://crates.io/crates/genevo/0.7.0)

### Added

- Add support for wasm32 targets

## [0.6.0] - 2021-11-07 (https://crates.io/crates/genevo/0.6.0)

### Changed

- Bump `fixedbitset` optional dependency to version 0.4
- Bump `rand` crate dependency to version 0.8
- Bump `rand_xoshiro` crate dependency to version 0.6
- Bump `proptest` crate dependency to version 1
- Replace deprecated method with new one in Criterion benchmark tests

### Fixed

- Fix index out of bounds exception in `OrderOneCrossover` and `PartiallyMappedCrossover` operations.
- Fix typos in docs.

## [0.5.0] - 2019-11-10 (https://crates.io/crates/genevo/0.5.0)

### Changed

- Bump `rand` crate dependency to version 0.7
- Bump `rand_xoshiro` crate dependency to version 0.3

## [0.4.0] - 2019-06-25 (https://crates.io/crates/genevo/0.4.0)

### Removed

- Remove method `step_with_seed` from `Simulation` trait.
- Remove field `seed` from `State` struct of the simulation.
- Remove variant `Unexpected` from `SimError` enum.

### Changed

- The `SimulationBuilder` trait requires an additional method `build_with_seed`.
- Method `processing_time` on `TrackProcessingTime` trait now returns owned 
  `ProcessingTime` instead of a reference to `ProcessingTime`. 

### Fixed

- Accumulate processing time for final simulation result

## [0.3.0] - 2019-06-25 (https://crates.io/crates/genevo/0.3.0)

### Added

- Add support for `SmallVec` as optional crate feature
- Implement std `Error` trait for `SimError` and `GeneticAlgorithmError`.
  This implicitly provides support for the `failure` crate.

### Changed

- Make support for `FixedBitSet` an optional crate feature
- Replace `DiscreteCrossBreeder` by integrating it into `UniformCrossBreeder`

### Fixed

- Make support for `Vec<bool>` consistent through all building blocks
- Tracking of accumulated processing time not correct 

### Other

- Minor internal changes to ease development

## [0.2.0] - 2019-06-24 (https://crates.io/crates/genevo/0.2.0)

### Added

- Implement `RandomValueMutation` for `bool`

### Changed

- Use `rand_xoshiro` crate for pseudo random number generation
- Migrate `rand` crate to version 0.6.x
- Do not use references to primitive types in function parameters or return types 
- Migrate to Rust 2018 edition
- Use `criterion` for benchmarking on stable Rust

## [0.1.2] - 2017-11-07 (https://crates.io/crates/genevo/0.1.2)

### Fixed

- Fix some mistakes in the documentation

## [0.1.1] - 2017-11-06 (https://crates.io/crates/genevo/0.1.1)

### Added

- Describe the basic building blocks (traits) defined in this crate.<br/>
  (documentation only, no code changes)

## [0.1.0] - 2017-10-26 (https://crates.io/crates/genevo/0.1.0)

### Added

- First release
