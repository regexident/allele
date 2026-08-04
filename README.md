# allele

[![Crates.io](https://img.shields.io/crates/v/allele)](https://crates.io/crates/allele)
[![Crates.io](https://img.shields.io/crates/d/allele)](https://crates.io/crates/allele)
[![Crates.io](https://img.shields.io/crates/l/allele)](https://crates.io/crates/allele)
[![docs.rs](https://docs.rs/allele/badge.svg)](https://docs.rs/allele/)

A modular framework for implementing and running genetic algorithm simulations.

---

## Usage

```rust
use allele::{operator::prelude::*, population::ValueEncodedGenomeBuilder, prelude::*};

const TARGET: &str = "Hello, world!";

// A candidate solution is encoded as a `Vec<u8>` (the genotype).
type Genome = Vec<u8>;

// The fitness function scores a genome against the target.
#[derive(Clone, Debug)]
struct FitnessCalc;

impl FitnessFunction<Genome, usize> for FitnessCalc {
    fn fitness_of(&self, genome: &Genome) -> usize {
        genome.iter().zip(TARGET.bytes()).filter(|(g, t)| *g == t).count()
    }

    fn average(&self, fitness_values: &[usize]) -> usize {
        fitness_values.iter().sum::<usize>() / fitness_values.len()
    }

    fn highest_possible_fitness(&self) -> usize {
        TARGET.len()
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

fn main() {
    let population = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(TARGET.len(), 32, 126))
        .of_size(200)
        .uniform_at_random();

    let mut sim = simulate(
        genetic_algorithm()
            .with_evaluation(FitnessCalc)
            .with_selection(MaximizeSelector::new(0.7, 2))
            .with_crossover(MultiPointCrossBreeder::new(2).expect("invalid num_cut_points"))
            .with_mutation(RandomValueMutator::new(0.1, 32, 126))
            .with_reinsertion(ElitistReinserter::new(FitnessCalc, true, 0.7))
            .with_initial_population(population)
            .build(),
    )
    .until(or(
        FitnessLimit::new(FitnessCalc.highest_possible_fitness()),
        GenerationLimit::new(1000),
    ))
    .build();

    loop {
        match sim.step() {
            Ok(ControlFlow::Break(result)) => {
                let genome = result.state.result.best_solution.solution.genome;
                println!("{}: {}", result.stop_reason, String::from_utf8(genome).unwrap());
                break;
            }
            Ok(ControlFlow::Continue(_)) => {}
            Err(error) => {
                eprintln!("{error}");
                break;
            }
        }
    }
}
```

## Overview

`allele` splits the genetic algorithm into building blocks, each modeled as a trait:

| Building block    | Responsibility                                                     |
| ----------------- | ------------------------------------------------------------------ |
| `Simulation`      | Runs an `Algorithm` in a loop, tracking iterations and time         |
| `Algorithm`       | Performs the steps of one evolution cycle (`GeneticAlgorithm`)      |
| `Termination`     | Decides when the simulation stops (limits, combinable via `or`/`and`) |
| `Operator`        | Stages of the algorithm: `SelectionOp`, `CrossoverOp`, `MutationOp`, `ReinsertionOp` |
| `Population`      | A set of individuals, built via `PopulationBuilder` / `GenomeBuilder` |
| `Genotype`/`Phenotype` | The candidate solution's encoding and its problem-domain form |
| `FitnessFunction` | Scores how good a candidate solution is                             |

Ready-made implementations are provided for every building block, so most problems
work out of the box. For problems that need something different, implement the
trait and plug it into the provided `GeneticAlgorithm`.

## Examples

* [knapsack](./examples/knapsack/main.rs): tries to solve the
  [0-1 knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem)
* [monkeys](./examples/monkeys/main.rs): explores the idea of Shakespeare's monkeys, also known
  as the [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem)
* [queens](./examples/queens/main.rs): searches for solutions of the
  [N Queens Problem](https://en.wikipedia.org/wiki/Eight_queens_puzzle)

## Crate features

| Feature        | Default | Description                                            |
| -------------- | ------- | ------------------------------------------------------ |
| `fixedbitset`  | no      | Provides `Fixedbitset` as a `Genotype`                 |
| `smallvec`     | no      | Provides `Smallvec` as a `Genotype`                    |
| `parallel`     | no      | Enables multithreading (via `rayon`)                   |
| `wasm-bindgen` | no      | Enables support for `wasm32-unknown-unknown` targets   |

Since version 0.7.0 `allele` supports wasm targets. To use `allele` for target
`wasm32-unknown-unknown` enable the `wasm-bindgen` feature. Note: multithreading
is not available on wasm32 targets!

## Documentation

Please refer to the documentation on [docs.rs](https://docs.rs/allele).

## Acknowledgments

`allele` is a renamed continuation of [genevo], a genetic algorithm framework
originally created by Harald Maida and the contributors at Innoave.com. The
project is dual-licensed under MIT/Apache-2.0; see [NOTICE](NOTICE) and
[COPYRIGHT.txt](COPYRIGHT.txt) for attribution details.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our
[code of conduct](https://www.rust-lang.org/conduct.html), and the process for
submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available,
see the [tags on this repository](https://github.com/regexident/allele/tags).

## License

This project is dual-licensed under the [**MIT**][mit-license] and
[**Apache-2.0**][apache-license] licenses – see the
[LICENSE_MIT.txt](LICENSE_MIT.txt)/[LICENSE_APACHE.txt](LICENSE_APACHE.txt) files
for details.

[mit-license]: https://www.tldrlegal.com/license/mit-license
[apache-license]: https://www.tldrlegal.com/l/apache-license-2-0-apache-2-0
[genevo]: https://github.com/innoave/genevo

Copyright &copy; 2017-2022, Innoave.com and contributors
Copyright &copy; 2026, The allele developers.
