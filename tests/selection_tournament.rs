use std::rc::Rc;

use proptest::prelude::ProptestConfig;
use test_strategy::proptest;

use allele::{
    algorithm::EvaluatedPopulation,
    operator::{SelectionOp, prelude::*},
    random::{Prng, RngExt, SeedableRng},
};

#[proptest(ProptestConfig {
    cases: 20,
    failure_persistence: None,
    ..ProptestConfig::default()
})]
fn select_from_returns_expected_number_of_parent_tuples(
    #[strategy(2usize..6)] tournament_size: usize,
    #[strategy(0.2f64..0.9)] probability: f64,
    #[strategy(0.1f64..0.9)] selection_ratio: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let population_size = 25;

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
        Rc::new(individuals.clone()),
        fitness_values,
        highest,
        lowest,
        average,
    );

    let selector = TournamentSelector::new(selection_ratio, 2, tournament_size, probability, false);
    let parents = selector.select_from(&evaluated, &mut rng);

    let expected = (population_size as f64 * selection_ratio + 0.5).floor() as usize;
    assert_eq!(parents.len(), expected);

    for tuple in &parents {
        assert_eq!(tuple.len(), 2);
        for individual in tuple {
            assert!(individuals.contains(individual));
        }
    }
}
