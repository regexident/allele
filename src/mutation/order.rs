//! The `order` module provides `operator::MutationOp`s for permutation encoded
//! `genetic::Genotype`s.

use std::fmt::Debug;

use crate::{
    operator::{GeneticOperator, MutationOp},
    random::{Rng, number_of_mutations, random_cut_points},
};

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct InsertOrderMutator {
    mutation_rate: f64,
}

impl InsertOrderMutator {
    pub fn new(mutation_rate: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&mutation_rate),
            "mutation_rate must be in [0.0, 1.0], got {}",
            mutation_rate
        );
        InsertOrderMutator { mutation_rate }
    }

    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    pub fn set_mutation_rate(&mut self, value: f64) {
        assert!(
            (0.0..=1.0).contains(&value),
            "mutation_rate must be in [0.0, 1.0], got {}",
            value
        );
        self.mutation_rate = value;
    }
}

impl GeneticOperator for InsertOrderMutator {
    fn name() -> String {
        "Order-Insert-Mutation".to_string()
    }
}

impl<V> MutationOp<Vec<V>> for InsertOrderMutator
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    fn mutate<R>(&self, genome: Vec<V>, rng: &mut R) -> Vec<V>
    where
        R: Rng + Sized,
    {
        let genome_length = genome.len();
        let num_mutations = number_of_mutations(genome_length, self.mutation_rate, rng);
        let mut mutated = genome;
        for _ in 0..num_mutations {
            let (locus1, locus2) = random_cut_points(rng, genome_length);
            let value2 = mutated.remove(locus2);
            mutated.insert(locus1 + 1, value2);
        }
        mutated
    }
}

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct SwapOrderMutator {
    mutation_rate: f64,
}

impl SwapOrderMutator {
    pub fn new(mutation_rate: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&mutation_rate),
            "mutation_rate must be in [0.0, 1.0], got {}",
            mutation_rate
        );
        SwapOrderMutator { mutation_rate }
    }

    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    pub fn set_mutation_rate(&mut self, value: f64) {
        assert!(
            (0.0..=1.0).contains(&value),
            "mutation_rate must be in [0.0, 1.0], got {}",
            value
        );
        self.mutation_rate = value;
    }
}

impl GeneticOperator for SwapOrderMutator {
    fn name() -> String {
        "Order-Swap-Mutation".to_string()
    }
}

impl<V> MutationOp<Vec<V>> for SwapOrderMutator
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    fn mutate<R>(&self, genome: Vec<V>, rng: &mut R) -> Vec<V>
    where
        R: Rng + Sized,
    {
        let genome_length = genome.len();
        let num_mutations = number_of_mutations(genome_length, self.mutation_rate, rng);
        let mut mutated = genome;
        for _ in 0..num_mutations {
            let (locus1, locus2) = random_cut_points(rng, genome_length);
            mutated.swap(locus1, locus2);
        }
        mutated
    }
}
