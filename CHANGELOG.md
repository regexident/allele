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

- n/a

### Changed

- Made `rayon` dependency optional behind "parallel" feature

### Deprecated

- n/a

### Removed

- n/a

### Fixed

- n/a

### Performance

- n/a

### Security

- n/a

### Other

- n/a

## [0.7.1] - 2022-03-13

### Fixed

- Fix issue #23 Future compile error on Rust beta 1.60 in types/tests.rs

## [0.7.0] - 2021-11-07

### Added

- Add support for wasm32 targets

## [0.6.0] - 2021-11-07

### Changed

- Bump `fixedbitset` optional dependency to version 0.4
- Bump `rand` crate dependency to version 0.8
- Bump `rand_xoshiro` crate dependency to version 0.6
- Bump `proptest` crate dependency to version 1
- Replace deprecated method with new one in Criterion benchmark tests

### Fixed

- Fix index out of bounds exception in `OrderOneCrossover` and `PartiallyMappedCrossover` operations.
- Fix typos in docs.

## [0.5.0] - 2019-11-10

### Changed

- Bump `rand` crate dependency to version 0.7
- Bump `rand_xoshiro` crate dependency to version 0.3

## [0.4.0] - 2019-06-25

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

## [0.3.0] - 2019-06-25

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

## [0.2.0] - 2019-06-24

### Added

- Implement `RandomValueMutation` for `bool`

### Changed

- Use `rand_xoshiro` crate for pseudo random number generation
- Migrate `rand` crate to version 0.6.x
- Do not use references to primitive types in function parameters or return types 
- Migrate to Rust 2018 edition
- Use `criterion` for benchmarking on stable Rust

## [0.1.2] - 2017-11-07

### Fixed

- Fix some mistakes in the documentation

## [0.1.1] - 2017-11-06 : First words

### Added

- Describe the basic building blocks (traits) defined in this crate.<br/>
  (documentation only, no code changes)

## [0.1.0] - 2017-10-26 : Newborn

### Added

- First release
