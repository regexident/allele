use std::borrow::Cow;
use std::fmt::Debug;

use rand::seq::IndexedRandom;

use crate::{
    genetic::Genotype,
    operator::{GeneticOperator, MutationOp},
    random::{Rng, RngExt, number_of_mutations, random_index},
};

#[derive(Clone, Debug, PartialEq)]
pub struct RandomValueMutator<G>
where
    G: Genotype + RandomGenomeMutation,
{
    mutation_rate: f64,
    min_value: <G as Genotype>::Dna,
    max_value: <G as Genotype>::Dna,
}

impl<G> RandomValueMutator<G>
where
    G: Genotype + RandomGenomeMutation,
{
    pub fn new(
        mutation_rate: f64,
        min_value: <G as Genotype>::Dna,
        max_value: <G as Genotype>::Dna,
    ) -> Self {
        assert!(
            (0.0..=1.0).contains(&mutation_rate),
            "mutation_rate must be in [0.0, 1.0], got {}",
            mutation_rate
        );
        RandomValueMutator {
            mutation_rate,
            min_value,
            max_value,
        }
    }
}

impl<G> GeneticOperator for RandomValueMutator<G>
where
    G: Genotype + RandomGenomeMutation,
{
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Random-Value-Mutator")
    }
}

impl<G> MutationOp<G> for RandomValueMutator<G>
where
    G: Genotype + RandomGenomeMutation,
{
    fn mutate<R>(&self, genome: G, rng: &mut R) -> G
    where
        R: Rng + Sized,
    {
        RandomGenomeMutation::mutate_genome(
            genome,
            self.mutation_rate,
            &self.min_value,
            &self.max_value,
            rng,
        )
    }
}

pub trait RandomGenomeMutation: Genotype {
    type Dna: Clone;

    fn mutate_genome<R>(
        genome: Self,
        mutation_rate: f64,
        min_value: &<Self as Genotype>::Dna,
        max_value: &<Self as Genotype>::Dna,
        rng: &mut R,
    ) -> Self
    where
        R: Rng + Sized;
}

impl<V> RandomGenomeMutation for Vec<V>
where
    V: Clone + Debug + PartialEq + Send + Sync + RandomValueMutation,
{
    type Dna = V;

    fn mutate_genome<R>(
        genome: Self,
        mutation_rate: f64,
        min_value: &V,
        max_value: &V,
        rng: &mut R,
    ) -> Self
    where
        R: Rng + Sized,
    {
        let genome_length = genome.len();
        let num_mutations = number_of_mutations(genome_length, mutation_rate, rng);
        let mut mutated = genome;
        for _ in 0..num_mutations {
            let index = random_index(rng, genome_length);
            mutated[index] = RandomValueMutation::random_mutated(
                mutated[index].clone(),
                min_value,
                max_value,
                rng,
            );
        }
        mutated
    }
}

#[cfg(feature = "fixedbitset")]
mod fixedbitset_random_genome_mutation {
    use fixedbitset::FixedBitSet;
    use rand::{Rng, RngExt};

    use crate::genetic::Genotype;

    use super::{RandomGenomeMutation, number_of_mutations, random_index};

    impl RandomGenomeMutation for FixedBitSet {
        type Dna = bool;

        fn mutate_genome<R>(
            genome: Self,
            mutation_rate: f64,
            _: &<Self as Genotype>::Dna,
            _: &<Self as Genotype>::Dna,
            rng: &mut R,
        ) -> Self
        where
            R: Rng + Sized,
        {
            let genome_length = genome.len();
            let num_mutations = number_of_mutations(genome_length, mutation_rate, rng);
            let mut mutated = genome;
            for _ in 0..num_mutations {
                let bit = random_index(rng, genome_length);
                let value = rng.random();
                mutated.set(bit, value);
            }
            mutated
        }
    }
}

#[cfg(feature = "smallvec")]
mod smallvec_random_genome_mutation {
    use std::fmt::Debug;

    use rand::Rng;
    use smallvec::{Array, SmallVec};

    use super::{RandomGenomeMutation, RandomValueMutation, number_of_mutations, random_index};

    impl<V, A> RandomGenomeMutation for SmallVec<A>
    where
        A: Array<Item = V> + Sync,
        V: Clone + Debug + PartialEq + Send + Sync + RandomValueMutation,
    {
        type Dna = V;

        fn mutate_genome<R>(
            genome: Self,
            mutation_rate: f64,
            min_value: &V,
            max_value: &V,
            rng: &mut R,
        ) -> Self
        where
            R: Rng + Sized,
        {
            let genome_length = genome.len();
            let num_mutations = number_of_mutations(genome_length, mutation_rate, rng);
            let mut mutated = genome;
            for _ in 0..num_mutations {
                let index = random_index(rng, genome_length);
                mutated[index] = RandomValueMutation::random_mutated(
                    mutated[index].clone(),
                    min_value,
                    max_value,
                    rng,
                );
            }
            mutated
        }
    }
}

pub trait RandomValueMutation {
    fn random_mutated<R>(value: Self, min_value: &Self, max_value: &Self, rng: &mut R) -> Self
    where
        R: Rng + Sized;
}

macro_rules! impl_random_value_mutation {
    ($($t:ty),*) => {
        $(
            impl RandomValueMutation for $t {
                #[inline]
                fn random_mutated<R>(_: $t, min_value: &$t, max_value: &$t, rng: &mut R) -> $t
                    where R: Rng + Sized
                {
                    rng.random_range(*min_value..*max_value)
                }
            }
        )*
    }
}

impl_random_value_mutation!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl RandomValueMutation for usize {
    #[inline]
    fn random_mutated<R>(_: usize, min_value: &usize, max_value: &usize, rng: &mut R) -> usize
    where
        R: Rng + Sized,
    {
        rng.random_range(*min_value as u64..*max_value as u64) as usize
    }
}

impl RandomValueMutation for isize {
    #[inline]
    fn random_mutated<R>(_: isize, min_value: &isize, max_value: &isize, rng: &mut R) -> isize
    where
        R: Rng + Sized,
    {
        rng.random_range(*min_value as i64..*max_value as i64) as isize
    }
}

impl RandomValueMutation for bool {
    #[inline]
    fn random_mutated<R>(_value: bool, _min_value: &bool, _max_value: &bool, rng: &mut R) -> bool
    where
        R: Rng + Sized,
    {
        rng.random_bool(0.5)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BreederValueMutator<G>
where
    G: Genotype + BreederGenomeMutation,
{
    mutation_rate: f64,
    mutation_range: <G as Genotype>::Dna,
    mutation_precision: u8,
    min_value: <G as Genotype>::Dna,
    max_value: <G as Genotype>::Dna,
}

impl<G> BreederValueMutator<G>
where
    G: Genotype + BreederGenomeMutation,
{
    pub fn new(
        mutation_rate: f64,
        mutation_range: <G as Genotype>::Dna,
        mutation_precision: u8,
        min_value: <G as Genotype>::Dna,
        max_value: <G as Genotype>::Dna,
    ) -> Self {
        assert!(
            (0.0..=1.0).contains(&mutation_rate),
            "mutation_rate must be in [0.0, 1.0], got {}",
            mutation_rate
        );
        BreederValueMutator {
            mutation_rate,
            mutation_range,
            mutation_precision,
            min_value,
            max_value,
        }
    }
}

impl<G> GeneticOperator for BreederValueMutator<G>
where
    G: Genotype + BreederGenomeMutation,
{
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Breeder-Value-Mutator")
    }
}

impl<G> MutationOp<G> for BreederValueMutator<G>
where
    G: Genotype + BreederGenomeMutation,
{
    fn mutate<R>(&self, genome: G, rng: &mut R) -> G
    where
        R: Rng + Sized,
    {
        BreederGenomeMutation::mutate_genome(
            genome,
            self.mutation_rate,
            &self.mutation_range,
            self.mutation_precision,
            &self.min_value,
            &self.max_value,
            rng,
        )
    }
}

pub trait BreederGenomeMutation: Genotype {
    type Dna: Clone;

    fn mutate_genome<R>(
        genome: Self,
        mutation_rate: f64,
        range: &<Self as Genotype>::Dna,
        precision: u8,
        min_value: &<Self as Genotype>::Dna,
        max_value: &<Self as Genotype>::Dna,
        rng: &mut R,
    ) -> Self
    where
        R: Rng + Sized;
}

impl<V> BreederGenomeMutation for Vec<V>
where
    V: Clone
        + Debug
        + PartialEq
        + PartialOrd
        + Send
        + Sync
        + BreederValueMutation
        + RandomValueMutation,
{
    type Dna = V;

    fn mutate_genome<R>(
        genome: Vec<V>,
        mutation_rate: f64,
        range: &<Self as Genotype>::Dna,
        precision: u8,
        min_value: &<Self as Genotype>::Dna,
        max_value: &<Self as Genotype>::Dna,
        rng: &mut R,
    ) -> Vec<V>
    where
        R: Rng + Sized,
    {
        let genome_length = genome.len();
        let num_mutations = number_of_mutations(genome_length, mutation_rate, rng);
        let mut mutated = genome;
        for _ in 0..num_mutations {
            let index = random_index(rng, genome_length);
            let sign = *[-1, 1].choose(rng).unwrap();
            let adjustment = if *[true, false].choose(rng).unwrap() {
                1. / (1i64 << precision) as f64
            } else {
                1.
            };
            let value_mut = BreederValueMutation::breeder_mutated(
                mutated[index].clone(),
                range,
                adjustment,
                sign,
            );
            if value_mut < *min_value {
                mutated[index] =
                    RandomValueMutation::random_mutated(value_mut, min_value, max_value, rng)
            } else if value_mut > *max_value {
                mutated[index] = max_value.clone();
            } else {
                mutated[index] = value_mut;
            }
        }
        mutated
    }
}

pub trait BreederValueMutation {
    fn breeder_mutated(value: Self, range: &Self, adjustment: f64, sign: i8) -> Self;
}

macro_rules! impl_breeder_mutation {
    ($($t:ty),*) => {
        $(
            #[allow(trivial_numeric_casts)]
            impl BreederValueMutation for $t {
                #[inline]
                fn breeder_mutated(value: $t, range: &$t, adjustment: f64, sign: i8) -> $t {
                    (value as f64 + *range as f64 * adjustment * sign as f64) as $t
                }
            }
        )*
    }
}

impl_breeder_mutation!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
