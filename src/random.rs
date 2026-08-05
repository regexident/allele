//! The `random` module defines functions that are used to generate random
//! values for specific purposes.

pub use rand::{
    Rng, RngExt, SeedableRng,
    distr::{Open01, uniform::SampleUniform},
    seq::{IndexedRandom, SliceRandom},
};

use rand_xoshiro::Xoshiro256Plus;

use crate::error::Error;
use crate::genetic::AsScalar;

/// The `Prng` is the pseudo random number generator used through out this
/// library.
pub type Prng = Xoshiro256Plus;

/// The `Seed` as used through out this library to seed the `Prng`.
pub type Seed = <Prng as SeedableRng>::Seed;

/// Generates a random seed to initialize the `Prng`.
pub fn random_seed() -> Seed {
    let mut rng = Prng::try_from_rng(&mut rand::rngs::SysRng).expect("failed to seed PRNG from OS");
    rng.random()
}

/// Returns a new `Prng` initialized with the given seed.
pub fn get_rng(seed: Seed) -> Prng {
    Prng::from_seed(seed)
}

/// Generates a random index into a slice of given length using the given
/// `Prng`.
pub fn random_index<R>(rng: &mut R, length: usize) -> usize
where
    R: Rng + Sized,
{
    random_index_from_range(rng, 0, length)
}

/// Generates a random index in the given range using the given `Prng`.
pub fn random_index_from_range<R>(rng: &mut R, min: usize, max: usize) -> usize
where
    R: Rng + Sized,
{
    rng.random_range(min..max)
}

/// Generates two cut points for a slice of given length using the given `Prng`.
/// The cut points are non-edge (at least one position from the ends) with a
/// minimum delta between them, producing non-degenerate crossover. The first
/// of the two returned cut points is always smaller than the second one.
pub fn random_cut_points<R>(rng: &mut R, length: usize) -> (usize, usize)
where
    R: Rng + Sized,
{
    random_cut_points_from_range(rng, 0, length)
}

/// Generates two cut points within the given range using the given `Prng`. The
/// first of the two returned cut points is always smaller than the second one.
pub fn random_cut_points_from_range<R>(rng: &mut R, min: usize, max: usize) -> (usize, usize)
where
    R: Rng + Sized,
{
    // delta must be drawn from [1, max-min-3], so the range must span at least 5 positions
    assert!(
        max >= min + 5,
        "range must span at least 5 elements: max={max}, min={min}"
    );
    let max_slice = max - min - 2;
    let delta = rng.random_range(1..max_slice);
    let cutpoint1 = rng.random_range(min..(max - delta));
    (cutpoint1, cutpoint1 + delta)
}

/// Generates `n` cut points for a slice of given length using the given `Prng`.
/// The returned cut points are ordered in ascending order.
///
/// For n==2: cut points are non-edge with minimum delta (via [`random_cut_points`]).
/// For n==1: edge-positions are possible, representing degenerate crossover
/// (via [`random_index`]).
/// For n>=3: cut points are distributed across proportional segments.
pub fn random_n_cut_points<R>(rng: &mut R, n: usize, length: usize) -> Vec<usize>
where
    R: Rng + Sized,
{
    assert!(n > 0);
    assert!(length >= 2 * n);
    let mut cutpoints = Vec::with_capacity(n);
    match n {
        1 => {
            cutpoints.push(random_index(rng, length));
        }
        2 => {
            let (cp1, cp2) = random_cut_points(rng, length);
            cutpoints.push(cp1);
            cutpoints.push(cp2);
        }
        _ => {
            for i in 1..=n {
                let start = ((i - 1) * length) / n + 1;
                let end = if i == n { length } else { (i * length) / n };
                let cutpoint = random_index_from_range(rng, start, end);
                cutpoints.push(cutpoint);
            }
        }
    }
    cutpoints
}

/// Generates a random probability between 0 and 1 using the given `Prng`.
///
/// The generated probabilities are in the open range (0,1), excluding 0 and
/// excluding 1.
pub fn random_probability<R>(rng: &mut R) -> f64
where
    R: Rng + Sized,
{
    rng.sample(Open01)
}

/// Returns the number of mutations to be performed on a genome of the given
/// length for the given mutation rate.
#[inline]
pub fn number_of_mutations<R>(genome_length: usize, mutation_rate: f64, rng: &mut R) -> usize
where
    R: Rng + Sized,
{
    ((genome_length as f64 * mutation_rate) + rng.random::<f64>()).floor() as usize
}

/// The `WeightedDistribution` is used to select values proportional to their
/// weighted values.
///
/// The values in a `WeightedDistribution` must have a scalar representation.
/// Thus their types must implement the `genetic::AsScalar` trait. The weights
/// of the values are calculated from their scalar representation.
#[derive(Clone, Debug, PartialEq)]
pub struct WeightedDistribution<'a, T>
where
    T: 'a + AsScalar,
{
    values: &'a [T],
    sum: f64,
    weights: Vec<f64>,
}

impl<'a, T> WeightedDistribution<'a, T>
where
    T: 'a + AsScalar,
{
    /// Constructs a new instance of `WeightedDistribution` for the given slice
    /// of values.
    pub fn from_scalar_values(values: &'a [T]) -> Result<Self, Error> {
        let (weights, weight_sum) = calc_weights_and_sum(values);
        if weight_sum == 0.0 {
            return Err(Error::ZeroWeightSum);
        }
        Ok(WeightedDistribution {
            values,
            weights,
            sum: weight_sum,
        })
    }

    /// Selects a value proportional to its weight and returns its index.
    ///
    /// The pointer must be a float between 0 und the sum of the weights of all
    /// values. Usually the pointer is chosen uniformly at random.
    pub fn select(&self, pointer: f64) -> usize {
        weighted_select(pointer.clamp(0.0, self.sum), &self.weights)
    }

    /// Returns the sum of the weights of all values in this
    /// `WeightedDistribution` instance.
    ///
    /// The sum is calculated from the scalar values of the slice that was used
    /// to create this `WeightedDistribution` instance.
    pub fn sum(&self) -> f64 {
        self.sum
    }

    /// Returns a reference to the value at the given index.
    pub fn value(&self, index: usize) -> &T {
        &self.values[index]
    }

    /// Returns the prefix-sum array used by this distribution.
    pub fn prefix_sums(&self) -> &[f64] {
        &self.weights
    }
}

/// Calculates weights and the sum for the given values.
fn calc_weights_and_sum<'a, T>(values: &'a [T]) -> (Vec<f64>, f64)
where
    T: 'a + AsScalar,
{
    let mut prefix_sum = 0.0f64;
    let weights: Vec<f64> = values
        .iter()
        .map(|v| {
            prefix_sum += v.as_scalar();
            prefix_sum
        })
        .collect();
    let sum = weights.last().copied().unwrap_or(0.0);

    (weights, sum)
}

/// Selects one index proportional to their weights.
fn weighted_select(pointer: f64, prefix_sums: &[f64]) -> usize {
    prefix_sums
        .partition_point(|&w| w < pointer)
        .min(prefix_sums.len() - 1)
}

#[cfg(test)]
mod tests;
