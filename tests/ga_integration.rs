use allele::{operator::prelude::*, prelude::*, random::RngExt};

#[derive(Clone, Debug, PartialEq)]
struct TrivialGenomeBuilder;

impl GenomeBuilder<Vec<i8>> for TrivialGenomeBuilder {
    fn build_genome<R>(&self, _index: usize, rng: &mut R) -> Vec<i8>
    where
        R: Rng + Sized,
    {
        (0..4).map(|_| rng.random_range(-10i8..=10i8)).collect()
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

    fn highest_possible_fitness(&self) -> i32 {
        40
    }

    fn lowest_possible_fitness(&self) -> i32 {
        -40
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
        .with_reinsertion(UniformReinserter::new(1.0).unwrap())
        .with_initial_population(initial_population)
        .build();

    let mut simulator = simulate(algorithm)
        .until(GenerationLimit::new(5))
        .build_with_seed(seed);

    let mut best_fitnesses: Vec<i32> = Vec::new();
    let mut final_population_size = 0;

    for _ in 0..5 {
        match simulator.step() {
            Ok(SimResult::Intermediate(state)) => {
                best_fitnesses.push(state.result.best_solution.solution.fitness);
            }
            Ok(SimResult::Final(state, _, _, _)) => {
                best_fitnesses.push(state.result.best_solution.solution.fitness);
                final_population_size = state.result.evaluated_population.individuals().len();
            }
            Err(_) => panic!("simulation step returned an error"),
        }
    }

    assert_eq!(best_fitnesses.len(), 5);
    assert!(
        *best_fitnesses.last().unwrap() >= best_fitnesses[0],
        "best fitness regressed across generations: {:?}",
        best_fitnesses
    );
    assert_eq!(final_population_size, population_size);
}
