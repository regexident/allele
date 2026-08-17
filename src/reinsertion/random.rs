//! The `random` module provides `operator::ReinsertionOp` that combine the
//! individuals from the offspring and the old population without considering
//! the fitness or any other attribute of the individuals.

use std::borrow::Cow;
use std::marker::PhantomData;

use crate::{
    algorithm::EvaluatedPopulation,
    genetic::{Fitness, FitnessFunction, Genotype, Offspring},
    operator::{GeneticOperator, MultiObjective, ReinsertionOp, SingleObjective},
    random::{Rng, random_index},
};

/// The `UniformReinserter` takes n individuals from the offspring and
/// o individuals from the old population and combines them to the new
/// population. The sum of n and o is always equal to the size of the
/// old population. The individuals to be inserted in the new population
/// are picked uniformly at random. Both offspring and old-population
/// selection are done without replacement — each individual can appear at
/// most once in the resulting population.
///
/// The reinserter can be configured by the `replace_ratio` field. The
/// replace ratio is the fraction of the population size that is replaced by
/// individuals from the offspring. The remaining spots are filled with
/// individuals from the old population.
///
/// A replace ratio of 1.0 means that the new population is fist filled with
/// individuals from the offspring. if the offspring does not contain enough
/// individuals then the new population is filled up with individuals from the
/// old population. If the offspring contains more individuals than the size of
/// the population then the individuals are chosen uniformly at random.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct UniformReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
    /// The `FitnessFunction` to be used to calculate fitness values of
    /// individuals of the offspring.
    fitness_evaluator: E,
    /// The `replace_ratio` defines the fraction of the population size that
    /// is going to be replaced by individuals from the offspring.
    replace_ratio: f64,
    // phantom types
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<G, F, E> UniformReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
    /// Constructs a new instance of the `UniformReinserter` with the given
    /// parameters.
    ///
    /// Returns [`InvalidArgumentError`] if `replace_ratio` is not in [0.0, 1.0].
    pub fn new(
        fitness_evaluator: E,
        replace_ratio: f64,
    ) -> Result<Self, crate::InvalidArgumentError> {
        if !(0.0..=1.0).contains(&replace_ratio) {
            return Err(crate::InvalidArgumentError::new(
                "replace_ratio",
                format!("must be in [0.0, 1.0], got {}", replace_ratio),
            ));
        }
        Ok(UniformReinserter {
            fitness_evaluator,
            replace_ratio,
            _g: PhantomData,
            _f: PhantomData,
        })
    }

    /// Returns the `replace_ratio` of this `UniformReinserter`.
    pub fn replace_ratio(&self) -> f64 {
        self.replace_ratio
    }

    /// Set the `replace_ratio` of this `UniformReinserter` to the given
    /// value. The value must be between 0 and 1.0 (inclusive).
    ///
    /// # Errors
    ///
    /// Returns [`InvalidArgumentError`] if `value` is not in [0.0, 1.0].
    pub fn set_replace_ratio(&mut self, value: f64) -> Result<(), crate::InvalidArgumentError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(crate::InvalidArgumentError::new(
                "replace_ratio",
                format!("must be in [0.0, 1.0], got {}", value),
            ));
        }
        self.replace_ratio = value;
        Ok(())
    }
}

impl<G, F, E> GeneticOperator for UniformReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Uniform-Reinserter")
    }
}

/// Can be used for single-objective optimization
impl<G, F, E> SingleObjective for UniformReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
}
/// Can be used for multi-objective optimization
impl<G, F, E> MultiObjective for UniformReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
}

impl<G, F, E> ReinsertionOp<G, F> for UniformReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
    fn combine<R>(
        &self,
        offspring: &mut Offspring<G>,
        evaluated: &EvaluatedPopulation<G, F>,
        rng: &mut R,
    ) -> Vec<(G, F)>
    where
        R: Rng + Sized,
    {
        let old_individuals = evaluated.individuals();
        let old_fitness_values = evaluated.fitness_values();
        let population_size = old_individuals.len();
        let mut new_population = Vec::with_capacity(population_size);

        // How many individuals should we take from the offspring?
        let num_offspring = (population_size as f64 * self.replace_ratio + 0.5).floor() as usize;

        // first pick individuals from offspring
        if num_offspring < offspring.len() {
            // pick individuals from the offspring uniformly at random
            while num_offspring > new_population.len() {
                let index = random_index(rng, offspring.len());
                let genome = offspring.swap_remove(index);
                let fitness = self.fitness_evaluator.fitness_of(&genome);
                new_population.push((genome, fitness));
            }
        } else {
            // insert all individuals from offspring
            while let Some(genome) = offspring.pop() {
                let fitness = self.fitness_evaluator.fitness_of(&genome);
                new_population.push((genome, fitness));
            }
        }
        // finally fill up new population with individuals from old population
        // (as many as needed).
        let num_old_population = population_size - new_population.len();
        let mut available_indices: Vec<usize> = (0..old_individuals.len()).collect();
        for _ in 0..num_old_population {
            let index = random_index(rng, available_indices.len());
            let chosen = available_indices.swap_remove(index);
            new_population.push((
                old_individuals[chosen].clone(),
                old_fitness_values[chosen].clone(),
            ));
        }
        new_population
    }
}
