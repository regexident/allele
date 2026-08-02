use proptest::prelude::ProptestConfig;
use test_strategy::proptest;

use allele::{
    mutation::order::{InsertOrderMutator, SwapOrderMutator},
    operator::MutationOp,
    random::{Prng, SeedableRng, SliceRandom},
};

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn insert_order_mutator_preserves_permutation(
    #[strategy(4usize..50)] genome_length: usize,
    #[strategy(0.0f64..=1.0)] mutation_rate: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let mut genome: Vec<usize> = (0..genome_length).collect();
    genome.shuffle(&mut rng);

    let mutator = InsertOrderMutator::new(mutation_rate);
    let mutated = mutator.mutate(genome.clone(), &mut rng);

    let mut original_sorted = genome;
    original_sorted.sort_unstable();
    let mut mutated_sorted = mutated;
    mutated_sorted.sort_unstable();
    assert_eq!(mutated_sorted, original_sorted);
}

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn swap_order_mutator_preserves_permutation(
    #[strategy(4usize..50)] genome_length: usize,
    #[strategy(0.0f64..=1.0)] mutation_rate: f64,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let mut genome: Vec<usize> = (0..genome_length).collect();
    genome.shuffle(&mut rng);

    let mutator = SwapOrderMutator::new(mutation_rate);
    let mutated = mutator.mutate(genome.clone(), &mut rng);

    let mut original_sorted = genome;
    original_sorted.sort_unstable();
    let mut mutated_sorted = mutated;
    mutated_sorted.sort_unstable();
    assert_eq!(mutated_sorted, original_sorted);
}
