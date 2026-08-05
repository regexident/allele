use std::ops::RangeInclusive;
use std::sync::Arc;

use proptest::prelude::ProptestConfig;
use test_strategy::proptest;

use allele::{
    algorithm::EvaluatedPopulation,
    genetic::FitnessFunction,
    operator::{ReinsertionOp, prelude::*},
    random::{Prng, RngExt, SeedableRng},
};

#[derive(Clone, Debug, PartialEq)]
struct DummyEvaluator;

impl FitnessFunction<Vec<i8>, i32> for DummyEvaluator {
    fn fitness_of(&self, _: &Vec<i8>) -> i32 {
        0
    }

    fn average(&self, fitness_values: &[i32]) -> i32 {
        if fitness_values.is_empty() {
            0
        } else {
            fitness_values.iter().sum::<i32>() / fitness_values.len() as i32
        }
    }

    fn fitness_bounds(&self) -> RangeInclusive<i32> {
        -100..=100
    }
}

#[proptest(ProptestConfig {
    cases: 50,
    failure_persistence: None,
    ..ProptestConfig::default()
})]
fn combine_returns_population_of_original_size(
    #[strategy(10usize..100)] population_size: usize,
    #[strategy(0.0f64..1.0)] replace_ratio: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);

    let individuals: Vec<Vec<i8>> = (0..population_size).map(|i| vec![i as i8; 4]).collect();
    let fitness_values: Vec<i32> = (0..population_size)
        .map(|_| rng.random_range(-100i32..100i32))
        .collect();
    let highest = *fitness_values.iter().max().unwrap();
    let lowest = *fitness_values.iter().min().unwrap();
    let average = fitness_values.iter().sum::<i32>() / fitness_values.len() as i32;
    let evaluated = EvaluatedPopulation::new(
        Arc::new(individuals.clone()),
        fitness_values,
        highest,
        lowest,
        average,
    );

    let mut offspring_to_combine: Vec<Vec<i8>> = (0..population_size)
        .map(|_| (0..4).map(|_| rng.random_range(-10i8..=10i8)).collect())
        .collect();
    let original_offspring = offspring_to_combine.clone();

    let reinserter = UniformReinserter::new(DummyEvaluator, replace_ratio).unwrap();
    let new_population = reinserter.combine(&mut offspring_to_combine, &evaluated, &mut rng);

    assert_eq!(new_population.len(), population_size);
    for (individual, _) in &new_population {
        assert!(original_offspring.contains(individual) || individuals.contains(individual));
    }
}
