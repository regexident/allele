//! The `tournament` module.
//!
//! The provided `SelectionOp` implementations are:
//! * `TournamentSelector`

use std::borrow::Cow;
use std::collections::VecDeque;

use crate::{
    algorithm::EvaluatedPopulation,
    genetic::{Fitness, Genotype, ParentIndices},
    operator::{GeneticOperator, MultiObjective, SelectionOp, SingleObjective},
    random::{Rng, random_index, random_probability},
};

/// The `TournamentSelector` implements the tournament selection method.
/// It runs tournaments with a small size of participants and pick the best
/// performing individuals from each tournament.
///
/// The number of participants in each tournament is configurable by the
/// `tournament_size` field. A tournament size of 1 is called 1-way tournament
/// and is equivalent to random selection.
///
/// The final selection is picked from the best performing participants in each
/// tournament but with a probability. The probability gives also chances to
/// the second best, third best and so on. The probability is configurable by
/// the `probability` field. A probability of 1.0 means the tournament is
/// deterministic. The best and only the best individual of each tournament is
/// selected.
///
/// To avoid that candidates chosen once are selected again they are removed
/// from the list of candidates. Though this can be configured as well. The
/// field `remove_selected_individuals` controls whether selected candidates
/// are removed or not.
///
/// This `TournamentSelector` can be used for single-objective fitness values
/// as well as multi-objective fitness values.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TournamentSelector {
    /// The fraction of number of parents to select in relation to the
    /// number of individuals in the population.
    selection_ratio: f64,
    /// The number of individuals per parents.
    num_individuals_per_parents: usize,
    /// The number of participants on each tournament.
    tournament_size: usize,
    /// The probability to pick candidates from one tournament.
    /// Values must be between 0 and 1.0 (inclusive).
    probability: f64,
    /// Remove chosen individuals from the list of candidates to avoid that
    /// they can be picked again.
    remove_selected_individuals: bool,
}

impl TournamentSelector {
    /// Constructs a new instance of the `TournamentSelector`.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidSelectionRatio` if `selection_ratio` is not in
    /// `[0.0, 1.0]`.
    ///
    /// Returns `Error::InvalidTournamentSize` if `tournament_size` is less than
    /// 1.
    ///
    /// Returns `Error::InvalidProbability` if `probability` is not in
    /// `(0.0, 1.0]`.
    pub fn new(
        selection_ratio: f64,
        num_individuals_per_parents: usize,
        tournament_size: usize,
        probability: f64,
        remove_selected_individuals: bool,
    ) -> Result<Self, crate::error::Error> {
        if !(0.0..=1.0).contains(&selection_ratio) {
            return Err(crate::error::Error::InvalidSelectionRatio {
                value: selection_ratio,
            });
        }
        if tournament_size < 1 {
            return Err(crate::error::Error::InvalidTournamentSize {
                value: tournament_size,
            });
        }
        if !(probability > 0.0 && probability <= 1.0) {
            return Err(crate::error::Error::InvalidProbability { value: probability });
        }
        Ok(TournamentSelector {
            selection_ratio,
            num_individuals_per_parents,
            tournament_size,
            probability,
            remove_selected_individuals,
        })
    }

    /// Returns the selection ratio.
    ///
    /// The selection ratio is the fraction of number of parents that are
    /// selected on every call of the `select_from` function and the number
    /// of individuals in the population.
    pub fn selection_ratio(&self) -> f64 {
        self.selection_ratio
    }

    /// Sets the selection ratio to a new value.
    ///
    /// The selection ratio is the fraction of number of parents that are
    /// selected on every call of the `select_from` function and the number
    /// of individuals in the population.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidSelectionRatio` if `value` is not in
    /// `[0.0, 1.0]`.
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

    /// Returns the size of one tournament.
    pub fn tournament_size(&self) -> usize {
        self.tournament_size
    }

    /// Sets the size of one tournament to a given value. The value must be
    /// a positive integer greater 0.
    ///
    /// A tournament size of 1 is called 1-way tournament and is
    /// equivalent to random selection.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidTournamentSize` if `value` is less than 1.
    pub fn set_tournament_size(&mut self, value: usize) -> Result<(), crate::error::Error> {
        if value < 1 {
            return Err(crate::error::Error::InvalidTournamentSize { value });
        }
        self.tournament_size = value;
        Ok(())
    }

    /// Returns the probability to pick candidates from one tournament.
    pub fn probability(&self) -> f64 {
        self.probability
    }

    /// Set the probability to pick candidates from one tournament to the given
    /// value. The value must be between 0 and 1.0 (inclusive).
    ///
    /// A probability of 1.0 means the tournament is deterministic. The best
    /// and only the best individual of each tournament is selected.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidProbability` if `value` is not in `(0.0, 1.0]`.
    pub fn set_probability(&mut self, value: f64) -> Result<(), crate::error::Error> {
        if !(value > 0.0 && value <= 1.0) {
            return Err(crate::error::Error::InvalidProbability { value });
        }
        self.probability = value;
        Ok(())
    }

    /// Returns whether individuals are removed from the list of candidates
    /// after they have been picked once.
    pub fn removes_selected_individuals(&self) -> bool {
        self.remove_selected_individuals
    }

    /// Sets whether individuals shall be removed from the list of candidates
    /// after they have been picked once.
    pub fn set_remove_selected_individuals(&mut self, value: bool) {
        self.remove_selected_individuals = value;
    }
}

/// Can be used for single-objective optimization
impl SingleObjective for TournamentSelector {}
/// Can be used for multi-objective optimization
impl MultiObjective for TournamentSelector {}

impl GeneticOperator for TournamentSelector {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Tournament-Selection")
    }
}

impl<G, F> SelectionOp<G, F> for TournamentSelector
where
    G: Genotype,
    F: Fitness,
{
    fn select_from<R>(
        &self,
        evaluated: &EvaluatedPopulation<G, F>,
        rng: &mut R,
    ) -> Result<Vec<ParentIndices>, crate::error::Error>
    where
        R: Rng + Sized,
    {
        let individuals = evaluated.individuals();
        let fitness_values = evaluated.fitness_values();

        // mating pool holds indices to the individuals and fitness_values slices
        let mut mating_pool: Vec<usize> = (0..fitness_values.len()).collect();

        let num_parents_to_select =
            (individuals.len() as f64 * self.selection_ratio + 0.5).floor() as usize;
        let target_num_candidates = num_parents_to_select * self.num_individuals_per_parents;

        // select candidates for parents
        let max_iterations = target_num_candidates
            .saturating_mul(self.tournament_size)
            .saturating_mul(100);
        let mut iterations: usize = 0;
        let mut picked_candidates: VecDeque<usize> = VecDeque::with_capacity(target_num_candidates);
        let mut count_candidates = 0;
        while count_candidates < target_num_candidates && !mating_pool.is_empty() {
            if iterations >= max_iterations {
                return Err(crate::error::Error::SelectionIterationLimitExceeded {
                    limit: max_iterations,
                });
            }
            iterations += 1;
            // fill up tournament with candidates
            let mut tournament: Vec<(usize, usize)> = Vec::with_capacity(self.tournament_size);
            let mut count_participants = 0;
            while count_participants < self.tournament_size {
                let pool_pos = random_index(rng, mating_pool.len());
                tournament.push((mating_pool[pool_pos], pool_pos));
                count_participants += 1;
            }
            tournament.sort_by(|a, b| fitness_values[a.0].cmp(&fitness_values[b.0]));
            let mut prob = self.probability;
            while let Some((picked, pool_pos)) = tournament.pop() {
                if random_probability(rng) <= prob {
                    if self.remove_selected_individuals {
                        let old_last = mating_pool.len() - 1;
                        mating_pool.swap_remove(pool_pos);
                        for entry in &mut tournament {
                            if entry.1 == old_last {
                                entry.1 = pool_pos;
                            }
                        }
                    }
                    picked_candidates.push_back(picked);
                    count_candidates += 1;
                }
                prob *= 1.0 - self.probability;
            }
        }
        // convert selected candidate indices to parents of individuals
        picked_candidates.truncate(target_num_candidates);
        let usable = (picked_candidates.len() / self.num_individuals_per_parents)
            * self.num_individuals_per_parents;
        picked_candidates.truncate(usable);
        let mut selected: Vec<ParentIndices> = Vec::with_capacity(num_parents_to_select);
        while !picked_candidates.is_empty() {
            let mut tuple = Vec::with_capacity(self.num_individuals_per_parents);
            for _ in 0..self.num_individuals_per_parents {
                // index into individuals slice
                let index_i = picked_candidates.pop_front().unwrap();
                tuple.push(index_i);
            }
            selected.push(tuple);
        }
        Ok(selected)
    }
}
