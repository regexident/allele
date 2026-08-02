use proptest::prelude::ProptestConfig;
use test_strategy::proptest;

use allele::{
    operator::CrossoverOp,
    random::{Prng, Rng, SeedableRng},
    recombination::discrete::{
        MultiPointCrossBreeder, SinglePointCrossBreeder, UniformCrossBreeder,
    },
};

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn uniform_crossbreeder_child_count_and_length(
    #[strategy(2usize..20)] genome_length: usize,
    #[strategy(2usize..6)] num_parents: usize,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let parents: Vec<Vec<u8>> = (0..num_parents)
        .map(|_| (0..genome_length).map(|_| rng.r#gen::<u8>()).collect())
        .collect();
    let op = UniformCrossBreeder::new();
    let children = op.crossover(parents, &mut rng);
    assert_eq!(children.len(), num_parents);
    for child in &children {
        assert_eq!(child.len(), genome_length);
    }
}

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn uniform_crossbreeder_genes_come_from_parents_at_same_locus(
    #[strategy(2usize..20)] genome_length: usize,
    #[strategy(2usize..6)] num_parents: usize,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let parents: Vec<Vec<u8>> = (0..num_parents)
        .map(|_| (0..genome_length).map(|_| rng.r#gen::<u8>()).collect())
        .collect();
    let op = UniformCrossBreeder::new();
    let children = op.crossover(parents.clone(), &mut rng);
    for child in &children {
        for (locus, gene) in child.iter().enumerate() {
            assert!(
                parents.iter().any(|p| p[locus] == *gene),
                "gene {gene} at locus {locus} not from any parent"
            );
        }
    }
}

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn single_point_crossbreeder_child_count_and_length(
    #[strategy(4usize..30)] genome_length: usize,
    #[strategy(2usize..6)] num_parents: usize,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let parents: Vec<Vec<u8>> = (0..num_parents)
        .map(|_| (0..genome_length).map(|_| rng.r#gen::<u8>()).collect())
        .collect();
    let op = SinglePointCrossBreeder::new();
    let children = op.crossover(parents, &mut rng);
    assert_eq!(children.len(), num_parents);
    for child in &children {
        assert_eq!(child.len(), genome_length);
    }
}

#[proptest(ProptestConfig { cases: 100, failure_persistence: None, ..ProptestConfig::default() })]
fn multi_point_crossbreeder_child_count_and_length(
    #[strategy(8usize..50)] genome_length: usize,
    #[strategy(2usize..6)] num_parents: usize,
    #[strategy(1usize..4)] num_cut_points: usize,
    #[strategy(0u64..u64::MAX)] seed: u64,
) {
    let mut rng = Prng::seed_from_u64(seed);
    let genome_length = genome_length.max(num_cut_points * 2 + 1);
    let parents: Vec<Vec<u8>> = (0..num_parents)
        .map(|_| (0..genome_length).map(|_| rng.r#gen::<u8>()).collect())
        .collect();
    let op = MultiPointCrossBreeder::new(num_cut_points);
    let children = op.crossover(parents, &mut rng);
    assert_eq!(children.len(), num_parents);
    for child in &children {
        assert_eq!(child.len(), genome_length);
    }
}
