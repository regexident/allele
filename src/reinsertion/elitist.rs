//! The `elitist` module provides `operator::ReinsertionOp` that combine the
//! individuals from the offspring and the old population by choosing the best
//! individuals from both.

use std::borrow::Cow;
use std::marker::PhantomData;

use crate::{
    algorithm::EvaluatedPopulation,
    genetic::{Fitness, FitnessFunction, Genotype, Offspring},
    operator::{GeneticOperator, MultiObjective, ReinsertionOp, SingleObjective},
    random::Rng,
};

/// The `ElitistReinserter` combines the best individuals from the offspring and
/// the old population. When there are more individuals in the offspring than
/// necessary, either because the offspring is larger than the population size
/// or a replace ratio smaller then 1.0 is specified, only those individuals
/// with the best fitness are taken over into the new population.
///
/// The reinserter can be configured by the `replace_ratio` field. The
/// replace ratio is the fraction of the population size that is replaced by
/// individuals from the offspring. The remaining spots are filled with
/// individuals from the old population.
///
/// A replace ratio of 1.0 means that the new population is fist filled with
/// individuals from the offspring. If the offspring does not contain enough
/// individuals then the new population is filled up with individuals from the
/// old population. If the offspring contains more individuals than the size of
/// the population then the individuals are chosen from the offspring based on
/// their fitness and the `offspring_has_precedence` setting.
#[derive(Clone, Debug, PartialEq)]
pub struct ElitistReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
    /// The `FitnessFunction` to be used to calculate fitness values of
    /// individuals of the offspring.
    fitness_evaluator: E,
    /// `offspring_has_precedence` defines whether individuals from offspring
    /// with lower fitness should possible replace better performing ones from
    /// the old population.
    offspring_has_precedence: bool,
    /// The `replace_ratio` defines the fraction of the population size that
    /// is going to be replaced by individuals from the offspring.
    replace_ratio: f64,
    // phantom types
    _g: PhantomData<G>,
    _f: PhantomData<F>,
}

impl<G, F, E> ElitistReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
    /// Constructs a new instance of the `ElitistReinserter`.
    ///
    /// Returns [`InvalidArgumentError`] if `replace_ratio` is not in [0.0, 1.0].
    pub fn new(
        fitness_evaluator: E,
        offspring_has_precedence: bool,
        replace_ratio: f64,
    ) -> Result<Self, crate::InvalidArgumentError> {
        if !(0.0..=1.0).contains(&replace_ratio) {
            return Err(crate::InvalidArgumentError::new(
                "replace_ratio",
                format!("must be in [0.0, 1.0], got {}", replace_ratio),
            ));
        }
        Ok(ElitistReinserter {
            fitness_evaluator,
            offspring_has_precedence,
            replace_ratio,
            _g: PhantomData,
            _f: PhantomData,
        })
    }

    /// Returns true if the offspring should take precedence over better
    /// performing individuals from the old population.
    pub fn offspring_has_precedence(&self) -> bool {
        self.offspring_has_precedence
    }

    /// Sets whether the offspring should have precedence over better
    /// performing individuals from the old population.
    pub fn set_offspring_has_precedence(&mut self, value: bool) {
        self.offspring_has_precedence = value;
    }

    /// Returns the `replace_ratio` of this `ElitistReinserter`.
    pub fn replace_ratio(&self) -> f64 {
        self.replace_ratio
    }

    /// Set the `replace_ratio` of this `ElitistReinserter` to the given
    /// value. The value must be between 0 and 1.0 (inclusive).
    ///
    /// # Panics
    ///
    /// Panics if `value` is not in [0.0, 1.0].
    pub fn set_replace_ratio(&mut self, value: f64) {
        assert!(
            (0.0..=1.0).contains(&value),
            "replace_ratio must be in [0.0, 1.0], got {}",
            value
        );
        self.replace_ratio = value;
    }
}

impl<G, F, E> GeneticOperator for ElitistReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Elitist-Reinserter")
    }
}

/// Can be used for single-objective optimization
impl<G, F, E> SingleObjective for ElitistReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
}
/// Can be used for multi-objective optimization
impl<G, F, E> MultiObjective for ElitistReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
}

impl<G, F, E> ReinsertionOp<G, F> for ElitistReinserter<G, F, E>
where
    G: Genotype,
    F: Fitness,
    E: FitnessFunction<G, F>,
{
    fn combine<R>(
        &self,
        offspring: &mut Offspring<G>,
        evaluated: &EvaluatedPopulation<G, F>,
        _: &mut R,
    ) -> Vec<(G, F)>
    where
        R: Rng + Sized,
    {
        let old_individuals = evaluated.individuals();
        let old_fitness_values = evaluated.fitness_values();
        // holds indices to the individuals and fitness_values slices
        let mut old_population_indices: Vec<usize> = (0..old_fitness_values.len()).collect();
        // sort fitness indices from best performing to worst performing index
        old_population_indices.sort_by(|x, y| old_fitness_values[*y].cmp(&old_fitness_values[*x]));

        let population_size = old_individuals.len();
        let mut new_population: Vec<(G, F)> = Vec::with_capacity(population_size);

        // How many individuals should we take from the offspring?
        let num_offspring = (population_size as f64 * self.replace_ratio + 0.5).floor() as usize;

        if self.offspring_has_precedence {
            // first pick individuals from offspring
            if num_offspring < offspring.len() {
                // evaluate fitness of the offspring individuals
                let mut offspring_fitness: Vec<(G, F)> = Vec::with_capacity(offspring.len());
                while let Some(child) = offspring.pop() {
                    let fitness = self.fitness_evaluator.fitness_of(&child);
                    offspring_fitness.push((child, fitness));
                }
                // sort offspring from worst to best performing performing
                offspring_fitness.sort_by(|x, y| x.1.cmp(&y.1));
                // pick only the best individuals from the offspring
                for _ in 0..num_offspring {
                    new_population.push(offspring_fitness.pop().unwrap());
                }
            } else {
                // insert all individuals from offspring
                let mut count = 0;
                while count < num_offspring && !offspring.is_empty() {
                    let child = offspring.remove(0);
                    let fitness = self.fitness_evaluator.fitness_of(&child);
                    new_population.push((child, fitness));
                    count += 1;
                }
            }
            // finally fill up new population with individuals from old population
            let num_old_population = population_size - new_population.len();
            for index_old in old_population_indices.iter().take(num_old_population) {
                // pick only the best individuals from old population
                new_population.push((
                    old_individuals[*index_old].clone(),
                    old_fitness_values[*index_old].clone(),
                ));
            }
        } else {
            // evaluate fitness of the offspring individuals
            let mut offspring_fitness: Vec<(G, F)> = Vec::with_capacity(offspring.len());
            while let Some(child) = offspring.pop() {
                let fitness = self.fitness_evaluator.fitness_of(&child);
                offspring_fitness.push((child, fitness));
            }
            // sort offspring from worst to best performing performing
            offspring_fitness.sort_by(|x, y| x.1.cmp(&y.1));
            let mut old_idx = 0;
            // Invariant: we iterate population_size times and there are at least
            // that many old individuals, so old_idx never exceeds the indices slice.
            for _ in 0..population_size {
                debug_assert!(
                    old_idx < old_population_indices.len(),
                    "old_idx out of bounds: old_idx = {}, len = {}",
                    old_idx,
                    old_population_indices.len()
                );
                let index_old = old_population_indices[old_idx];
                if !offspring_fitness.is_empty()
                    && offspring_fitness[offspring_fitness.len() - 1].1
                        > old_fitness_values[index_old]
                {
                    new_population.push(offspring_fitness.pop().unwrap());
                } else {
                    new_population.push((
                        old_individuals[index_old].clone(),
                        old_fitness_values[index_old].clone(),
                    ));
                    old_idx += 1;
                }
            }
        }
        new_population
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::EvaluatedPopulation;
    use crate::operator::ReinsertionOp;
    use crate::random::Prng;
    use rand::SeedableRng;
    use std::sync::Arc;

    #[derive(Clone, Debug, PartialEq)]
    struct DummyEvaluator;

    impl FitnessFunction<Vec<i32>, i32> for DummyEvaluator {
        fn fitness_of(&self, individual: &Vec<i32>) -> i32 {
            individual.iter().sum()
        }

        fn average(&self, fitness_values: &[i32]) -> i32 {
            if fitness_values.is_empty() {
                0
            } else {
                fitness_values.iter().sum::<i32>() / fitness_values.len() as i32
            }
        }

        fn highest_possible_fitness(&self) -> i32 {
            100
        }

        fn lowest_possible_fitness(&self) -> i32 {
            -100
        }
    }

    fn make_population(
        population_size: usize,
        fitness_values: Vec<i32>,
    ) -> EvaluatedPopulation<Vec<i32>, i32> {
        let individuals: Vec<Vec<i32>> = (0..population_size).map(|i| vec![i as i32]).collect();
        let highest = *fitness_values.iter().max().unwrap();
        let lowest = *fitness_values.iter().min().unwrap();
        let sum: i32 = fitness_values.iter().sum();
        let average = sum / fitness_values.len() as i32;
        EvaluatedPopulation::new(
            Arc::new(individuals),
            fitness_values,
            highest,
            lowest,
            average,
        )
    }

    #[test]
    fn elitist_offspring_has_precedence_true() {
        let population_size = 5;
        let fitness_values = vec![1, 2, 3, 4, 5];
        let evaluated = make_population(population_size, fitness_values);

        let mut offspring: Vec<Vec<i32>> = vec![vec![6], vec![7], vec![8], vec![9], vec![10]];

        let reinserter = ElitistReinserter::new(DummyEvaluator, true, 1.0).unwrap();
        let mut rng = Prng::seed_from_u64(42);
        let result = reinserter.combine(&mut offspring, &evaluated, &mut rng);

        assert_eq!(result.len(), population_size);
        for (genome, _) in &result {
            assert!(
                genome[0] >= 6,
                "expected offspring individual, got genome {:?}",
                genome
            );
        }
    }

    #[test]
    fn elitist_offspring_has_precedence_false() {
        let population_size = 5;
        let fitness_values = vec![10, 9, 8, 7, 6];
        let evaluated = make_population(population_size, fitness_values);

        let mut offspring: Vec<Vec<i32>> = vec![vec![5], vec![4], vec![3], vec![2], vec![1]];

        let reinserter = ElitistReinserter::new(DummyEvaluator, false, 1.0).unwrap();
        let mut rng = Prng::seed_from_u64(42);
        let result = reinserter.combine(&mut offspring, &evaluated, &mut rng);

        assert_eq!(result.len(), population_size);
        let has_old_best = result.iter().any(|(g, _)| g[0] == 0);
        assert!(
            has_old_best,
            "best old individual should be kept when precedence=false"
        );
    }

    #[test]
    fn elitist_replace_ratio_zero() {
        let population_size = 5;
        let fitness_values = vec![1, 2, 3, 4, 5];
        let evaluated = make_population(population_size, fitness_values);

        let mut offspring: Vec<Vec<i32>> = vec![vec![100], vec![200]];

        let reinserter = ElitistReinserter::new(DummyEvaluator, true, 0.0).unwrap();
        let mut rng = Prng::seed_from_u64(42);
        let result = reinserter.combine(&mut offspring, &evaluated, &mut rng);

        assert_eq!(result.len(), population_size);
        for (genome, _) in &result {
            assert!(
                genome[0] < 100,
                "only old individuals should survive when ratio=0.0, got {:?}",
                genome
            );
        }
    }

    #[test]
    fn elitist_replace_ratio_one() {
        let population_size = 5;
        let fitness_values = vec![1, 2, 3, 4, 5];
        let evaluated = make_population(population_size, fitness_values);

        let mut offspring: Vec<Vec<i32>> = vec![vec![10], vec![20], vec![30], vec![40], vec![50]];

        let reinserter = ElitistReinserter::new(DummyEvaluator, true, 1.0).unwrap();
        let mut rng = Prng::seed_from_u64(42);
        let result = reinserter.combine(&mut offspring, &evaluated, &mut rng);

        assert_eq!(result.len(), population_size);
        for (genome, _) in &result {
            assert!(
                genome[0] >= 10,
                "only offspring should survive when ratio=1.0, got {:?}",
                genome
            );
        }
    }

    #[test]
    fn elitist_offspring_larger_than_population() {
        let population_size = 3;
        let fitness_values = vec![1, 2, 3];
        let evaluated = make_population(population_size, fitness_values);

        let mut offspring: Vec<Vec<i32>> = (0..10).map(|i| vec![i]).collect();

        let reinserter = ElitistReinserter::new(DummyEvaluator, true, 1.0).unwrap();
        let mut rng = Prng::seed_from_u64(42);
        let result = reinserter.combine(&mut offspring, &evaluated, &mut rng);

        assert_eq!(result.len(), population_size);
        for (genome, _) in &result {
            assert!(
                genome[0] >= 7,
                "only best {} offspring should be in result, got {:?}",
                population_size,
                genome
            );
        }
    }
}
