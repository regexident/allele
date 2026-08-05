use proptest::prelude::ProptestConfig;
use test_strategy::proptest;

use allele::{
    algorithm::EvaluatedPopulation,
    operator::{SelectionOp, prelude::*},
    random::{Prng, RngExt, SeedableRng},
};

#[proptest(ProptestConfig {
    cases: 50,
    failure_persistence: None,
    ..ProptestConfig::default()
})]
fn maximize_selector_returns_expected_number_of_parent_tuples(
    #[strategy(0.1f64..0.9f64)] selection_ratio: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let population_size = 50usize;

    let individuals: Vec<Vec<i8>> = (0..population_size)
        .map(|_| (0..4).map(|_| rng.random_range(-10i8..=10i8)).collect())
        .collect();
    let fitness_values: Vec<i32> = (0..population_size)
        .map(|_| rng.random_range(-100i32..100i32))
        .collect();
    let highest = *fitness_values.iter().max().unwrap();
    let lowest = *fitness_values.iter().min().unwrap();
    let average = fitness_values.iter().sum::<i32>() / fitness_values.len() as i32;
    let evaluated = EvaluatedPopulation::new(
        individuals.clone(),
        fitness_values,
        highest,
        lowest,
        average,
    );

    let selector = MaximizeSelector::new(selection_ratio, 2).unwrap();
    let parents = selector.select_from(&evaluated, &mut rng).unwrap();

    let expected = (population_size as f64 * selection_ratio + 0.5).floor() as usize;
    assert_eq!(parents.len(), expected);
    for tuple in &parents {
        for &index in tuple {
            assert!(index < individuals.len());
        }
    }
}

#[proptest(ProptestConfig {
    cases: 50,
    failure_persistence: None,
    ..ProptestConfig::default()
})]
fn maximize_selector_selects_from_top_performers(
    #[strategy(0.1f64..0.5f64)] selection_ratio: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let population_size = 50usize;

    let individuals: Vec<Vec<i8>> = (0..population_size)
        .map(|_| (0..8).map(|_| rng.random_range(-100i8..=100i8)).collect())
        .collect();
    let fitness_values: Vec<i32> = (0..population_size)
        .map(|_| rng.random_range(-100i32..100i32))
        .collect();
    let highest = *fitness_values.iter().max().unwrap();
    let lowest = *fitness_values.iter().min().unwrap();
    let average = fitness_values.iter().sum::<i32>() / fitness_values.len() as i32;
    let evaluated = EvaluatedPopulation::new(
        individuals.clone(),
        fitness_values.clone(),
        highest,
        lowest,
        average,
    );

    let num_individuals_per_parents = 2usize;
    let selector = MaximizeSelector::new(selection_ratio, num_individuals_per_parents).unwrap();
    let parents = selector.select_from(&evaluated, &mut rng).unwrap();

    let num_parents_to_select = (population_size as f64 * selection_ratio + 0.5).floor() as usize;
    let pool_size = num_parents_to_select * num_individuals_per_parents;
    let mut sorted_fitness = fitness_values.clone();
    sorted_fitness.sort_unstable_by(|a, b| b.cmp(a));
    let threshold = sorted_fitness[pool_size.min(population_size) - 1];

    for tuple in &parents {
        for &index in tuple {
            assert!(
                fitness_values[index] >= threshold,
                "selected individual has fitness {} below threshold {}",
                fitness_values[index],
                threshold
            );
        }
    }
}
