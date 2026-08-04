//! The `discrete` module provides `operator::CrossoverOp`s that recombine
//! `genetic::Genotype`s by exchanging variable values between the parent
//! individuals. Discrete recombination can be applied for binary encoded
//! genotypes and value encoded genotypes.
//!
//! The provided `operator::CrossoverOp`s are:
//! * `UniformCrossBreeder` for `fixedbitset::FixedBitSet`,
//!   `smallvec::SmallVec` and `Vec` of any type.
//! * `SinglePointCrossBreeder` for `fixedbitset::FixedBitSet`,
//!   `smallvec::SmallVec` and `Vec` of any type.
//! * `MultiPointCrossBreeder` for `fixedbitset::FixedBitSet`,
//!   `smallvec::SmallVec` and `Vec` of any type.

use std::borrow::Cow;
use std::fmt::Debug;

use crate::{
    genetic::{Children, Genotype, Parents},
    operator::{CrossoverOp, GeneticOperator},
    random::{Rng, RngExt, random_n_cut_points},
};

/// The `UniformCrossBreeder` operator combines binary encoded or value encoded
/// `genetic::Genotype`s by walking through the bits/values of the parents one
/// by one and randomly selecting the bit/value of one partner that is copied to
/// the resulting child.
///
/// This crossover operator always creates as many child individuals as there
/// are individuals in the given `genetic::Parents` parameter.
#[allow(missing_copy_implementations)]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct UniformCrossBreeder {}

impl UniformCrossBreeder {
    pub fn new() -> Self {
        UniformCrossBreeder {}
    }
}

impl GeneticOperator for UniformCrossBreeder {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Uniform-Cross-Breeder")
    }
}

impl<V> CrossoverOp<Vec<V>> for UniformCrossBreeder
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    fn crossover<R>(&self, parents: Parents<Vec<V>>, rng: &mut R) -> Children<Vec<V>>
    where
        R: Rng + Sized,
    {
        let genome_length = parents[0].len();
        let num_parents = parents.len();
        // breed one child for each partner in parents
        let mut offspring: Vec<Vec<V>> = Vec::with_capacity(num_parents);
        while num_parents > offspring.len() {
            // for each value in the genotype pick the value of a randomly chosen parent
            let genome: Vec<V> = (0..genome_length)
                .map(|locus| parents[rng.random_range(0..num_parents)][locus].clone())
                .collect();
            offspring.push(genome);
        }
        offspring
    }
}

#[cfg(feature = "fixedbitset")]
mod fixedbitset_uniform_cross_breeder {
    use fixedbitset::FixedBitSet;
    use rand::{Rng, RngExt};

    use crate::{
        genetic::{Children, Parents},
        operator::CrossoverOp,
    };

    use super::UniformCrossBreeder;

    impl CrossoverOp<FixedBitSet> for UniformCrossBreeder {
        fn crossover<R>(&self, parents: Parents<FixedBitSet>, rng: &mut R) -> Children<FixedBitSet>
        where
            R: Rng + Sized,
        {
            let genome_length = parents[0].len();
            let num_parents = parents.len();
            // breed one child for each partner in parents
            let mut offspring: Vec<FixedBitSet> = Vec::with_capacity(num_parents);
            while num_parents > offspring.len() {
                // for each bit in the genotype pick the bit of a randomly chosen parent
                let genome: FixedBitSet = (0..genome_length)
                    .filter(|&locus| parents[rng.random_range(0..num_parents)][locus])
                    .collect();
                offspring.push(genome);
            }
            offspring
        }
    }
}

#[cfg(feature = "smallvec")]
mod smallvec_uniform_cross_breeder {
    use std::fmt::Debug;

    use rand::{Rng, RngExt};
    use smallvec::{Array, SmallVec};

    use crate::operator::CrossoverOp;

    use super::UniformCrossBreeder;

    impl<A, V> CrossoverOp<SmallVec<A>> for UniformCrossBreeder
    where
        A: Array<Item = V> + Sync,
        V: Clone + Debug + PartialEq + Send + Sync,
    {
        fn crossover<R>(&self, parents: Vec<SmallVec<A>>, rng: &mut R) -> Vec<SmallVec<A>>
        where
            R: Rng + Sized,
        {
            let genome_length = parents[0].len();
            let num_parents = parents.len();
            // breed one child for each partner in parents
            let mut offspring: Vec<SmallVec<A>> = Vec::with_capacity(num_parents);
            while num_parents > offspring.len() {
                // for each value in the genotype pick the value of a randomly chosen parent
                let genome: SmallVec<A> = (0..genome_length)
                    .map(|locus| parents[rng.random_range(0..num_parents)][locus].clone())
                    .collect();
                offspring.push(genome);
            }
            offspring
        }
    }
}

/// The `SinglePointCrossBreeder` operator combines binary encoded or value
/// encoded `genetic::Genotype`s by splitting the vector of bits/values into 2
/// slices and combining the slices from randomly picked parents into the new
/// `genetic:Genotype`.
///
/// This crossover operator always creates as many child individuals as there
/// are individuals in the given `genetic::Parents` parameter.
#[allow(missing_copy_implementations)]
#[derive(Default, Clone, Debug, PartialEq)]
pub struct SinglePointCrossBreeder {}

impl SinglePointCrossBreeder {
    pub fn new() -> Self {
        SinglePointCrossBreeder {}
    }
}

impl GeneticOperator for SinglePointCrossBreeder {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Single-Point-Cross-Breeder")
    }
}

impl<G> CrossoverOp<G> for SinglePointCrossBreeder
where
    G: Genotype + MultiPointCrossover,
{
    fn crossover<R>(&self, parents: Parents<G>, rng: &mut R) -> Children<G>
    where
        R: Rng + Sized,
    {
        MultiPointCrossover::crossover(parents, 1, rng)
    }
}

/// The `MultiPointCrossBreeder` operator combines binary or value encoded
/// `genetic:Genotype`s by splitting the vector of values into 2 or several
/// slices and combining the slices from randomly picked parents into the new
/// `genetic:Genotype`.
///
/// This crossover operator always creates as many child individuals as there
/// are individuals in the given `genetic::Parents` parameter.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct MultiPointCrossBreeder {
    /// The number of cut points used by this operator.
    num_cut_points: usize,
}

impl MultiPointCrossBreeder {
    pub fn new(num_cut_points: usize) -> Result<Self, crate::error::Error> {
        if num_cut_points < 1 {
            return Err(crate::error::Error::InvalidNumCutPoints {
                value: num_cut_points,
            });
        }
        Ok(MultiPointCrossBreeder { num_cut_points })
    }

    /// Returns the number of cut points used by this operator.
    pub fn num_cut_points(&self) -> usize {
        self.num_cut_points
    }

    /// Sets the number of cut points used by this operator to the given value.
    pub fn set_num_cut_points(&mut self, value: usize) {
        self.num_cut_points = value;
    }
}

impl GeneticOperator for MultiPointCrossBreeder {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Multi-Point-Cross-Breeder")
    }
}

impl<G> CrossoverOp<G> for MultiPointCrossBreeder
where
    G: Genotype + MultiPointCrossover,
{
    fn crossover<R>(&self, parents: Parents<G>, rng: &mut R) -> Children<G>
    where
        R: Rng + Sized,
    {
        MultiPointCrossover::crossover(parents, self.num_cut_points, rng)
    }
}

pub trait MultiPointCrossover: Genotype {
    type Dna;

    fn crossover<R>(parents: Parents<Self>, num_cut_points: usize, rng: &mut R) -> Children<Self>
    where
        R: Rng + Sized;
}

fn crossover_one_child<V, R>(
    parents: &[impl std::ops::Deref<Target = [V]>],
    num_cut_points: usize,
    rng: &mut R,
) -> Vec<V>
where
    V: Clone,
    R: Rng + Sized,
{
    let genome_length = parents[0].len();
    let num_parents = parents.len();
    let mut genome = Vec::with_capacity(genome_length);
    let mut cutpoints = random_n_cut_points(rng, num_cut_points, genome_length);
    cutpoints.push(genome_length);
    let mut cutpoints_iter = cutpoints.into_iter();
    let mut start = 0;
    let mut end = cutpoints_iter.next().unwrap();
    let mut p_index = num_parents;
    loop {
        loop {
            let index = rng.random_range(0..num_parents);
            if index != p_index {
                p_index = index;
                break;
            }
        }
        let partner = &parents[p_index];
        for partner in partner.iter().take(end).skip(start) {
            genome.push(partner.clone())
        }
        match cutpoints_iter.next() {
            None => break,
            Some(next) => {
                start = end;
                end = next;
            }
        }
    }
    genome
}

impl<V> MultiPointCrossover for Vec<V>
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    type Dna = V;

    fn crossover<R>(parents: Parents<Self>, num_cut_points: usize, rng: &mut R) -> Children<Self>
    where
        R: Rng + Sized,
    {
        let num_parents = parents.len();
        let mut offspring: Vec<Vec<V>> = Vec::with_capacity(num_parents);
        while num_parents > offspring.len() {
            if num_parents <= 1 {
                for _ in 0..num_parents {
                    offspring.push(parents[0].clone());
                }
                continue;
            }
            offspring.push(crossover_one_child(&parents, num_cut_points, rng));
        }
        offspring
    }
}

#[cfg(feature = "smallvec")]
mod smallvec_multipoint_crossover {
    use std::fmt::Debug;

    use rand::Rng;
    use smallvec::{Array, SmallVec};

    use crate::genetic::{Children, Parents};

    use super::{MultiPointCrossover, crossover_one_child};

    impl<A, V> MultiPointCrossover for SmallVec<A>
    where
        A: Array<Item = V> + Sync,
        V: Clone + Debug + PartialEq + Send + Sync,
    {
        type Dna = V;

        fn crossover<R>(
            parents: Parents<Self>,
            num_cut_points: usize,
            rng: &mut R,
        ) -> Children<Self>
        where
            R: Rng + Sized,
        {
            let num_parents = parents.len();
            let mut offspring: Vec<SmallVec<A>> = Vec::with_capacity(num_parents);
            while num_parents > offspring.len() {
                if num_parents <= 1 {
                    for _ in 0..num_parents {
                        offspring.push(parents[0].clone());
                    }
                    continue;
                }
                offspring.push(SmallVec::from_vec(crossover_one_child(
                    &parents,
                    num_cut_points,
                    rng,
                )));
            }
            offspring
        }
    }
}

#[cfg(feature = "fixedbitset")]
mod fixedbitset_multipoint_crossover {
    use fixedbitset::FixedBitSet;
    use rand::{Rng, RngExt};

    use crate::genetic::{Children, Parents};

    use super::{MultiPointCrossover, random_n_cut_points};

    impl MultiPointCrossover for FixedBitSet {
        type Dna = bool;

        fn crossover<R>(
            parents: Parents<FixedBitSet>,
            num_cut_points: usize,
            rng: &mut R,
        ) -> Children<FixedBitSet>
        where
            R: Rng + Sized,
        {
            let genome_length = parents[0].len();
            let num_parents = parents.len();
            // breed one child for each partner in parents
            let mut offspring: Vec<FixedBitSet> = Vec::with_capacity(num_parents);
            while num_parents > offspring.len() {
                if num_parents <= 1 {
                    for _ in 0..num_parents {
                        offspring.push(parents[0].clone());
                    }
                    continue;
                }
                let mut genome = FixedBitSet::with_capacity(genome_length);
                let mut cutpoints = random_n_cut_points(rng, num_cut_points, genome_length);
                cutpoints.push(genome_length);
                let mut cutpoints_iter = cutpoints.into_iter();
                let mut start = 0;
                let mut end = cutpoints_iter.next().unwrap();
                let mut p_index = num_parents;
                loop {
                    loop {
                        let index = rng.random_range(0..num_parents);
                        if index != p_index {
                            p_index = index;
                            break;
                        }
                    }
                    let partner = &parents[p_index];
                    for bit in start..end {
                        genome.set(bit, partner[bit])
                    }
                    match cutpoints_iter.next() {
                        None => break,
                        Some(next) => {
                            start = end;
                            end = next;
                        }
                    }
                }
                offspring.push(genome);
            }
            offspring
        }
    }
}
