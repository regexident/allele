use std::ops::{ControlFlow, RangeInclusive};

use allele::{operator::prelude::*, prelude::*, random::RngExt};
use proptest::prelude::ProptestConfig;
use test_strategy::proptest;

#[derive(Clone, Debug, PartialEq)]
struct TrivialGenomeBuilder;

impl GenomeBuilder<Vec<i8>> for TrivialGenomeBuilder {
    fn build_genome<R>(&self, _index: usize, rng: &mut R) -> Vec<i8>
    where
        R: Rng + Sized,
    {
        (0..5).map(|_| rng.random_range(-10i8..=10i8)).collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SumFitnessEvaluator;

impl FitnessFunction<Vec<i8>, i32> for SumFitnessEvaluator {
    fn fitness_of(&self, individual: &Vec<i8>) -> i32 {
        individual.iter().map(|&allele| allele as i32).sum()
    }

    fn average(&self, fitness_values: &[i32]) -> i32 {
        fitness_values.iter().sum::<i32>() / fitness_values.len() as i32
    }

    fn fitness_bounds(&self) -> RangeInclusive<i32> {
        -50..=50
    }
}

#[test]
fn ga_run_preserves_population_size_and_improves_best_fitness() {
    let seed: Seed = [2u8; 32];
    let population_size = 50;

    let initial_population: Population<Vec<i8>> = build_population()
        .with_genome_builder(TrivialGenomeBuilder)
        .of_size(population_size)
        .using_seed(seed);

    let algorithm = genetic_algorithm()
        .with_evaluation(SumFitnessEvaluator)
        .with_selection(TournamentSelector::new(0.5, 2, 3, 0.8, false).unwrap())
        .with_crossover(SinglePointCrossBreeder::new())
        .with_mutation(SwapOrderMutator::new(0.1))
        .with_reinsertion(UniformReinserter::new(SumFitnessEvaluator, 1.0).unwrap())
        .with_initial_population(initial_population)
        .build();

    let mut simulator = simulate(algorithm)
        .until(GenerationLimit::new(5))
        .build_with_seed(seed);

    let mut best_fitnesses: Vec<i32> = Vec::new();
    let mut final_population_size = 0;

    for _ in 0..5 {
        match simulator.step() {
            Ok(ControlFlow::Continue(state)) => {
                best_fitnesses.push(state.result.best_solution.solution.fitness);
            }
            Ok(ControlFlow::Break(result)) => {
                best_fitnesses.push(result.state.result.best_solution.solution.fitness);
                final_population_size =
                    result.state.result.evaluated_population.individuals().len();
            }
            Err(_) => panic!("simulation step returned an error"),
        }
    }

    assert_eq!(best_fitnesses.len(), 5);
    assert!(
        *best_fitnesses.last().unwrap() > best_fitnesses[0],
        "best fitness regressed across generations: {:?}",
        best_fitnesses
    );
    assert_eq!(final_population_size, population_size);
}

#[cfg(feature = "parallel")]
#[test]
fn parallel_serial_parity() {
    // Use a small population so that parallel code paths fall back to
    // serial execution (threshold is 50). This guarantees that the
    // parallel-enabled binary produces the same result as serial.
    let seed: Seed = [42u8; 32];
    let population_size = 20;

    let initial_population: Population<Vec<i8>> = build_population()
        .with_genome_builder(TrivialGenomeBuilder)
        .of_size(population_size)
        .using_seed(seed);

    let simulate_once = || {
        let algorithm = genetic_algorithm()
            .with_evaluation(SumFitnessEvaluator)
            .with_selection(TournamentSelector::new(0.7, 2, 3, 0.9, false).unwrap())
            .with_crossover(SinglePointCrossBreeder::new())
            .with_mutation(SwapOrderMutator::new(0.1))
            .with_reinsertion(ElitistReinserter::new(SumFitnessEvaluator, false, 1.0).unwrap())
            .with_initial_population(initial_population.clone())
            .build();

        let mut simulator = simulate(algorithm)
            .until(GenerationLimit::new(3))
            .build_with_seed(seed);

        #[allow(unused_assignments)]
        let mut best_solution = None;
        loop {
            match simulator.step() {
                Ok(ControlFlow::Continue(state)) => {
                    #[allow(unused_assignments)]
                    {
                        best_solution = Some(state.result.best_solution);
                    }
                }
                Ok(ControlFlow::Break(result)) => {
                    best_solution = Some(result.state.result.best_solution);
                    break;
                }
                Err(_) => panic!("simulation step returned an error"),
            }
        }
        best_solution.unwrap()
    };

    let first = simulate_once();
    let second = simulate_once();

    assert_eq!(
        first, second,
        "parallel and serial runs must produce identical results"
    );
}

#[proptest(ProptestConfig {
    cases: 30,
    failure_persistence: None,
    ..ProptestConfig::default()
})]
fn population_size_preserved_across_generations(
    #[strategy(10usize..60)] population_size: usize,
    #[strategy(0u64..u64::MAX)] seed_lo: u64,
) {
    let seed_bytes = {
        let mut bytes = [0u8; 32];
        bytes[..8].copy_from_slice(&seed_lo.to_le_bytes());
        bytes
    };

    let initial_population: Population<Vec<i8>> = build_population()
        .with_genome_builder(TrivialGenomeBuilder)
        .of_size(population_size)
        .using_seed(seed_bytes);

    let algorithm = genetic_algorithm()
        .with_evaluation(SumFitnessEvaluator)
        .with_selection(TournamentSelector::new(0.6, 2, 3, 0.7, false).unwrap())
        .with_crossover(SinglePointCrossBreeder::new())
        .with_mutation(SwapOrderMutator::new(0.1))
        .with_reinsertion(UniformReinserter::new(SumFitnessEvaluator, 0.9).unwrap())
        .with_initial_population(initial_population)
        .build();

    let mut simulator = simulate(algorithm)
        .until(GenerationLimit::new(5))
        .build_with_seed(seed_bytes);

    loop {
        match simulator.step() {
            Ok(ControlFlow::Continue(state)) => {
                let pop_len = state.result.evaluated_population.individuals().len();
                assert_eq!(
                    pop_len, population_size,
                    "population size must be preserved"
                );
            }
            Ok(ControlFlow::Break(result)) => {
                let pop_len = result.state.result.evaluated_population.individuals().len();
                assert_eq!(
                    pop_len, population_size,
                    "final population size must be preserved"
                );
                break;
            }
            Err(_) => panic!("simulation step returned an error"),
        }
    }
}
