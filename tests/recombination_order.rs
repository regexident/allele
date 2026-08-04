use proptest::prelude::ProptestConfig;
use test_strategy::proptest;

use allele::{
    operator::CrossoverOp,
    random::{Prng, SeedableRng, SliceRandom},
    recombination::order::{OrderOneCrossover, PartiallyMappedCrossover},
};

fn is_valid_permutation(genome: &[usize]) -> bool {
    let n = genome.len();
    let mut seen = vec![false; n];
    genome.iter().all(|&v| {
        v < n && {
            let was = seen[v];
            seen[v] = true;
            !was
        }
    })
}

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn order_one_crossover_children_are_valid_permutations(
    #[strategy(5usize..20)] genome_length: usize,
    #[strategy(2usize..5)] num_parents: usize,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let parents: Vec<Vec<usize>> = (0..num_parents)
        .map(|_| {
            let mut perm: Vec<usize> = (0..genome_length).collect();
            perm.shuffle(&mut rng);
            perm
        })
        .collect();

    let op = OrderOneCrossover::new();
    let children = op.crossover(parents, &mut rng);

    assert_eq!(children.len(), num_parents);
    for child in &children {
        assert_eq!(child.len(), genome_length);
        assert!(
            is_valid_permutation(child),
            "child is not a valid permutation: {child:?}"
        );
    }
}

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn partially_mapped_crossover_children_are_valid_permutations(
    #[strategy(5usize..20)] genome_length: usize,
    #[strategy(2usize..5)] num_parents: usize,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let parents: Vec<Vec<usize>> = (0..num_parents)
        .map(|_| {
            let mut perm: Vec<usize> = (0..genome_length).collect();
            perm.shuffle(&mut rng);
            perm
        })
        .collect();

    let op = PartiallyMappedCrossover::new();
    let children = op.crossover(parents, &mut rng);

    assert_eq!(children.len(), num_parents);
    for child in &children {
        assert_eq!(child.len(), genome_length);
        assert!(
            is_valid_permutation(child),
            "child is not a valid permutation: {child:?}"
        );
    }
}
