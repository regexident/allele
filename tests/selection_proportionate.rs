use std::rc::Rc;

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
fn roulette_wheel_selector_returns_expected_number_of_parent_tuples(
    #[strategy(0.1f64..0.9f64)] selection_ratio: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let population_size = 50usize;
    let num_individuals_per_parents = 2usize;

    let individuals: Vec<Vec<i8>> = (0..population_size)
        .map(|_| (0..4).map(|_| rng.random_range(-10i8..=10i8)).collect())
        .collect();
    let fitness_values: Vec<u32> = (0..population_size)
        .map(|_| rng.random_range(1u32..100u32))
        .collect();
    let highest = *fitness_values.iter().max().unwrap();
    let lowest = *fitness_values.iter().min().unwrap();
    let average = fitness_values.iter().copied().sum::<u32>() / fitness_values.len() as u32;
    let evaluated = EvaluatedPopulation::new(
        Rc::new(individuals.clone()),
        fitness_values,
        highest,
        lowest,
        average,
    );

    let selector = RouletteWheelSelector::new(selection_ratio, num_individuals_per_parents);
    let parents = selector.select_from(&evaluated, &mut rng);

    let expected = (population_size as f64 * selection_ratio + 0.5).floor() as usize;
    assert_eq!(parents.len(), expected);
    for tuple in &parents {
        assert_eq!(tuple.len(), num_individuals_per_parents);
        for individual in tuple {
            assert!(individuals.contains(individual));
        }
    }
}

#[proptest(ProptestConfig {
    cases: 50,
    failure_persistence: None,
    ..ProptestConfig::default()
})]
fn universal_sampling_selector_returns_expected_number_of_parent_tuples(
    #[strategy(0.1f64..0.9f64)] selection_ratio: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let population_size = 50usize;
    let num_individuals_per_parents = 2usize;

    let individuals: Vec<Vec<i8>> = (0..population_size)
        .map(|_| (0..4).map(|_| rng.random_range(-10i8..=10i8)).collect())
        .collect();
    let fitness_values: Vec<u32> = (0..population_size)
        .map(|_| rng.random_range(1u32..100u32))
        .collect();
    let highest = *fitness_values.iter().max().unwrap();
    let lowest = *fitness_values.iter().min().unwrap();
    let average = fitness_values.iter().copied().sum::<u32>() / fitness_values.len() as u32;
    let evaluated = EvaluatedPopulation::new(
        Rc::new(individuals.clone()),
        fitness_values,
        highest,
        lowest,
        average,
    );

    let selector = UniversalSamplingSelector::new(selection_ratio, num_individuals_per_parents);
    let parents = selector.select_from(&evaluated, &mut rng);

    let expected = (population_size as f64 * selection_ratio + 0.5).floor() as usize;
    assert_eq!(parents.len(), expected);
    for tuple in &parents {
        assert_eq!(tuple.len(), num_individuals_per_parents);
        for individual in tuple {
            assert!(individuals.contains(individual));
        }
    }
}
