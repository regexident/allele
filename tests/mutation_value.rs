use proptest::prelude::ProptestConfig;
use test_strategy::proptest;

use allele::{
    mutation::value::RandomValueMutator,
    operator::MutationOp,
    random::{Prng, RngExt, SeedableRng},
};

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn random_value_mutator_preserves_genome_length(
    #[strategy(1usize..100)] genome_length: usize,
    #[strategy(0.0f64..=1.0)] mutation_rate: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let genome: Vec<i32> = (0..genome_length)
        .map(|_| rng.random_range(-1000i32..1000i32))
        .collect();
    let mutator = RandomValueMutator::new(mutation_rate, -1000i32, 1000i32);
    let mutated = mutator.mutate(genome, &mut rng);
    assert_eq!(mutated.len(), genome_length);
}

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn random_value_mutator_with_zero_rate_is_identity(
    #[strategy(1usize..100)] genome_length: usize,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let genome: Vec<i32> = (0..genome_length)
        .map(|_| rng.random_range(-1000i32..1000i32))
        .collect();
    let mutator = RandomValueMutator::new(0.0, -1000i32, 1000i32);
    let mutated = mutator.mutate(genome.clone(), &mut rng);
    assert_eq!(mutated, genome);
}

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn random_value_mutator_genes_stay_within_bounds(
    #[strategy(1usize..50)] genome_length: usize,
    #[strategy(-10_000i32..-1i32)] min_value: i32,
    #[strategy(1i32..10_001i32)] max_value: i32,
    #[strategy(0.0f64..=1.0)] mutation_rate: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let genome: Vec<i32> = (0..genome_length)
        .map(|_| rng.random_range(min_value..max_value))
        .collect();
    let mutator = RandomValueMutator::new(mutation_rate, min_value, max_value);
    let mutated = mutator.mutate(genome, &mut rng);
    for gene in mutated {
        assert!(
            gene >= min_value && gene < max_value,
            "gene {gene} out of bounds [{min_value}, {max_value})"
        );
    }
}
