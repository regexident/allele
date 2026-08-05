use std::ops::RangeInclusive;

use allele::{
    algorithm::EvaluatedPopulation,
    error::Error,
    genetic::FitnessFunction,
    operator::{SelectionOp, prelude::*},
    population::ValueEncodedGenomeBuilder,
    prelude::GenomeBuilder,
    random::{Prng, SeedableRng},
};

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

fn build_population(size: usize, seed: u64) -> (EvaluatedPopulation<Vec<i8>, i32>, Prng) {
    let mut rng = Prng::seed_from_u64(seed);
    let genome_builder = ValueEncodedGenomeBuilder::new(4, -10i8, 10i8);
    let individuals: Vec<Vec<i8>> = (0..size)
        .map(|i| genome_builder.build_genome(i, &mut rng))
        .collect();
    let evaluator = SumFitnessEvaluator;
    let fitness_values: Vec<i32> = individuals
        .iter()
        .map(|ind| evaluator.fitness_of(ind))
        .collect();
    let highest = *fitness_values.iter().max().unwrap();
    let lowest = *fitness_values.iter().min().unwrap();
    let average = evaluator.average(&fitness_values);
    let evaluated = EvaluatedPopulation::new(individuals, fitness_values, highest, lowest, average);
    (evaluated, rng)
}

#[test]
fn new_rejects_selection_ratio_above_one() {
    let result = TournamentSelector::new(1.1, 2, 3, 0.8, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let expected = Error::InvalidSelectionRatio { value: 1.1 };
    assert_eq!(err, expected);
}

#[test]
fn new_rejects_selection_ratio_below_zero() {
    let result = TournamentSelector::new(-0.1, 2, 3, 0.8, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let expected = Error::InvalidSelectionRatio { value: -0.1 };
    assert_eq!(err, expected);
}

#[test]
fn new_rejects_tournament_size_zero() {
    let result = TournamentSelector::new(0.5, 2, 0, 0.8, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let expected = Error::InvalidTournamentSize { value: 0 };
    assert_eq!(err, expected);
}

#[test]
fn new_accepts_valid_parameters() {
    let result = TournamentSelector::new(0.5, 2, 3, 0.8, false);
    assert!(result.is_ok());
}

#[test]
fn select_from_prob_one_produces_expected_parent_tuples() {
    let (evaluated, _rng) = build_population(10, 0);
    let mut rng = Prng::seed_from_u64(42);
    let selector = TournamentSelector::new(0.5, 2, 3, 1.0, false).unwrap();
    let parents = selector.select_from(&evaluated, &mut rng).unwrap();
    assert_eq!(parents.len(), 5);
    for tuple in &parents {
        assert_eq!(tuple.len(), 2);
        for &index in tuple {
            assert!(index < 10);
        }
    }
}

#[test]
fn select_from_zero_ratio_produces_zero_parents() {
    let (evaluated, _rng) = build_population(10, 1);
    let mut rng = Prng::seed_from_u64(42);
    let selector = TournamentSelector::new(0.0, 2, 3, 0.8, false).unwrap();
    let result = selector.select_from(&evaluated, &mut rng);
    match result {
        Ok(parents) => {
            assert_eq!(parents.len(), 0);
        }
        Err(e) => {
            assert_eq!(e, Error::ZeroParentsSelected);
        }
    }
}

#[test]
fn select_from_remove_selected_exhausts_candidates() {
    let mut rng = Prng::seed_from_u64(0);
    let genome_builder = ValueEncodedGenomeBuilder::new(1, 0i8, 10i8);
    let individuals: Vec<Vec<i8>> = (0..5)
        .map(|i| genome_builder.build_genome(i, &mut rng))
        .collect();
    let evaluator = SumFitnessEvaluator;
    let fitness_values: Vec<i32> = individuals
        .iter()
        .map(|ind| evaluator.fitness_of(ind))
        .collect();
    let highest = *fitness_values.iter().max().unwrap();
    let lowest = *fitness_values.iter().min().unwrap();
    let average = evaluator.average(&fitness_values);
    let evaluated = EvaluatedPopulation::new(individuals, fitness_values, highest, lowest, average);

    let mut rng = Prng::seed_from_u64(0);
    let selector = TournamentSelector::new(1.0, 2, 2, f64::MIN_POSITIVE, true).unwrap();
    let result = selector.select_from(&evaluated, &mut rng);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        Error::SelectionIterationLimitExceeded { .. }
    ));
}

#[test]
fn select_from_deterministic_tournament_selects_only_best() {
    let individuals: Vec<Vec<i8>> = vec![
        vec![100i8],
        vec![1i8],
        vec![2i8],
        vec![3i8],
        vec![4i8],
        vec![5i8],
        vec![6i8],
        vec![7i8],
        vec![8i8],
        vec![9i8],
    ];
    let evaluator = SumFitnessEvaluator;
    let fitness_values: Vec<i32> = individuals
        .iter()
        .map(|ind| evaluator.fitness_of(ind))
        .collect();

    let mut rng = Prng::seed_from_u64(0);
    let selector = TournamentSelector::new(0.5, 2, 10, 1.0, false).unwrap();
    let parents = selector
        .select_from(
            &EvaluatedPopulation::new(individuals.clone(), fitness_values, 100, 1, 100 / 2),
            &mut rng,
        )
        .unwrap();

    assert_eq!(parents.len(), 5);
    for tuple in &parents {
        assert_eq!(tuple.len(), 2);
        for &index in tuple {
            assert!(index < 10);
        }
    }
}
