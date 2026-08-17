use std::ops::RangeInclusive;

use allele::{operator::prelude::*, population::ValueEncodedGenomeBuilder, prelude::*};

#[test]
fn create_new_genetic_algorithm_application() {
    type MyGenome = Vec<f64>;

    #[derive(Clone, Debug, PartialEq)]
    struct MyFitnessEvaluator;

    impl FitnessFunction<MyGenome, u32> for MyFitnessEvaluator {
        fn fitness_of(&self, individual: &MyGenome) -> u32 {
            (individual.iter().sum::<f64>() * 10000. + 0.5).floor() as u32
        }

        fn average(&self, fitness_values: &[u32]) -> u32 {
            (fitness_values.iter().sum::<u32>() as f64 / fitness_values.len() as f64 + 0.5).floor()
                as u32
        }

        fn fitness_bounds(&self) -> RangeInclusive<u32> {
            0..=10000
        }
    }

    let initial_population: Population<Vec<f64>> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(4, -2., 2.))
        .of_size(200)
        .uniform_at_random();

    let algorithm = genetic_algorithm()
        .with_evaluation(MyFitnessEvaluator)
        .with_selection(RouletteWheelSelector::new(0.7, 2).unwrap())
        .with_crossover(MultiPointCrossBreeder::new(3).expect("invalid num_cut_points"))
        .with_mutation(RandomValueMutator::new(0.015, -2.0, 2.0).unwrap())
        .with_reinsertion(ElitistReinserter::new(MyFitnessEvaluator, false, 0.7).unwrap())
        .with_initial_population(initial_population)
        .build();

    assert_eq!(algorithm.selector().selection_ratio(), 0.7);
    assert_eq!(algorithm.selector().num_individuals_per_parents(), 2);
    assert_eq!(algorithm.breeder().num_cut_points(), 3);
}
