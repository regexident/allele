//! The `truncation` module provides selection methods that are merely based
//! on the fitness values of the individuals but don't have any stochastic
//! component.
//!
//! The provided `SelectionOp` implementations are:
//! * `MaximizeSelector`

use std::borrow::Cow;

use crate::{
    algorithm::EvaluatedPopulation,
    genetic::{Fitness, Genotype, ParentIndices},
    operator::{GeneticOperator, MultiObjective, SelectionOp, SingleObjective},
    random::Rng,
};

/// The `MaximizeSelector` selects the best performing `genetic::Genotype`s
/// from the population.
///
/// This `MaximizeSelector` can be used for single-objective fitness values
/// as well as multi-objective fitness values.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MaximizeSelector {
    /// The truncation threshold is the ratio between the number of parents
    /// to be selected and the size of the population:
    /// threshold = number of parents / size of population
    selection_ratio: f64,
    /// The number of individuals per parents.
    num_individuals_per_parents: usize,
    // phantom types
}

impl MaximizeSelector {
    /// Constructs a new instance of the `MaximizeSelector`.
    pub fn new(
        selection_ratio: f64,
        num_individuals_per_parents: usize,
    ) -> Result<Self, crate::error::Error> {
        if !(0.0..=1.0).contains(&selection_ratio) {
            return Err(crate::error::Error::InvalidSelectionRatio {
                value: selection_ratio,
            });
        }
        Ok(MaximizeSelector {
            selection_ratio,
            num_individuals_per_parents,
        })
    }

    /// Returns the selection ratio.
    ///
    /// The selection ratio is the fraction of number of parents that are
    /// selected on every call of the `selection` function and the number
    /// of individuals in the population.
    pub fn selection_ratio(&self) -> f64 {
        self.selection_ratio
    }

    /// Sets the selection ratio to a new value.
    ///
    /// The selection ratio is the fraction of number of parents that are
    /// selected on every call of the `selection` function and the number
    /// of individuals in the population.
    pub fn set_selection_ratio(&mut self, value: f64) -> Result<(), crate::error::Error> {
        if !(0.0..=1.0).contains(&value) {
            return Err(crate::error::Error::InvalidSelectionRatio { value });
        }
        self.selection_ratio = value;
        Ok(())
    }

    /// Returns the number of individuals per parents use by this selector.
    pub fn num_individuals_per_parents(&self) -> usize {
        self.num_individuals_per_parents
    }

    /// Sets the number of individuals per parents to the given value.
    pub fn set_num_individuals_per_parents(&mut self, value: usize) {
        self.num_individuals_per_parents = value;
    }
}

/// Can be used for single-objective optimization
impl SingleObjective for MaximizeSelector {}
/// Can be used for multi-objective optimization
impl MultiObjective for MaximizeSelector {}

impl GeneticOperator for MaximizeSelector {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Maximizing-Truncation-Selection")
    }
}

impl<G, F> SelectionOp<G, F> for MaximizeSelector
where
    G: Genotype,
    F: Fitness,
{
    fn select_from<R>(
        &self,
        evaluated: &EvaluatedPopulation<G, F>,
        _: &mut R,
    ) -> Result<Vec<ParentIndices>, crate::error::Error>
    where
        R: Rng + Sized,
    {
        let individuals = evaluated.individuals();
        let fitness_values = evaluated.fitness_values();

        let mut mating_pool: Vec<usize> = (0..fitness_values.len()).collect();
        mating_pool.sort_by(|x, y| fitness_values[*y].cmp(&fitness_values[*x]));

        let num_parents_to_select =
            (individuals.len() as f64 * self.selection_ratio + 0.5).floor() as usize;
        let pool_size =
            (num_parents_to_select * self.num_individuals_per_parents).min(mating_pool.len());
        let mut selected: Vec<ParentIndices> = Vec::with_capacity(num_parents_to_select);

        let mut index_m = 0;
        for _ in 0..num_parents_to_select {
            let mut tuple = Vec::with_capacity(self.num_individuals_per_parents);
            for _ in 0..self.num_individuals_per_parents {
                index_m %= pool_size;
                let index_i = mating_pool[index_m];
                tuple.push(index_i);
                index_m += 1;
            }
            selected.push(tuple);
        }
        Ok(selected)
    }
}
