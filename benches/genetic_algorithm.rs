#[macro_use]
extern crate criterion;

use std::ops::RangeInclusive;

use criterion::Criterion;

use allele::{
    operator::prelude::*,
    prelude::*,
    random::{Rng, RngExt},
};

const NUMBER_OF_QUEENS: i16 = 16;
const NUM_ROWS: i16 = NUMBER_OF_QUEENS;
const NUM_COLS: i16 = NUMBER_OF_QUEENS;
const POPULATION_SIZE: usize = 200;
const GENERATION_LIMIT: u64 = 2000;
const NUM_INDIVIDUALS_PER_PARENTS: usize = 3;
const SELECTION_RATIO: f64 = 0.7;
const MUTATION_RATE: f64 = 0.05;
const REINSERTION_RATIO: f64 = 0.7;

/// The genotype
#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Pos {
    x: i16,
    y: i16,
}

type Positions = Vec<Pos>;

fn count_collisions(positions: &Positions) -> i16 {
    let mut count = 0;
    for (i, i_pos) in positions.iter().enumerate() {
        for (j, j_pos) in positions.iter().enumerate() {
            if i != j
                && (i_pos.x == j_pos.x
                    || i_pos.y == j_pos.y
                    || i_pos.x + i_pos.y == j_pos.x + j_pos.y
                    || i_pos.x - i_pos.y == j_pos.x - j_pos.y)
            {
                count += 1;
            }
        }
    }

    count
}

/// The fitness function for `Positions`.
#[derive(Clone, Debug)]
struct FitnessCalc;

impl FitnessFunction<Positions, usize> for FitnessCalc {
    fn fitness_of(&self, positions: &Positions) -> usize {
        let collisions = count_collisions(positions);
        let max_collisions = (NUMBER_OF_QUEENS - 1) * (NUMBER_OF_QUEENS - 1);
        let score = (max_collisions - collisions) as f32 / (max_collisions + collisions) as f32;
        (score * score * 100. + 0.5).floor() as usize
    }

    fn average(&self, values: &[usize]) -> usize {
        (values.iter().sum::<usize>() as f32 / values.len() as f32 + 0.5).floor() as usize
    }

    fn fitness_bounds(&self) -> RangeInclusive<usize> {
        0..=NUMBER_OF_QUEENS as usize
    }
}

impl BreederValueMutation for Pos {
    fn breeder_mutated(value: Self, range: &Pos, adjustment: f64, sign: i8) -> Self {
        Pos {
            x: value.x,
            y: value.y + (range.y as f64 * adjustment * sign as f64) as i16,
        }
    }
}

impl RandomExclusiveValueMutation for Pos {
    fn random_mutated<R>(value: Self, min_value: &Pos, max_value: &Pos, rng: &mut R) -> Self
    where
        R: Rng + Sized,
    {
        Pos {
            x: value.x,
            y: rng.random_range(min_value.y..max_value.y),
        }
    }
}

/// Generate some random boards
struct QueensPositions;

impl GenomeBuilder<Positions> for QueensPositions {
    fn build_genome<R>(&self, _: usize, rng: &mut R) -> Positions
    where
        R: Rng + Sized,
    {
        (0..NUM_ROWS)
            .map(|row| Pos {
                x: row,
                y: rng.random_range(0..NUM_COLS),
            })
            .collect()
    }
}

fn bench_full_genetic_algorithm(c: &mut Criterion) {
    let seed = [42; 32];

    c.bench_function("full genetic algorithm (queens)", |b| {
        b.iter(|| {
            let initial_population: Population<Positions> = build_population()
                .with_genome_builder(QueensPositions)
                .of_size(POPULATION_SIZE)
                .using_seed(seed);

            let mut queens_sim = simulate(
                genetic_algorithm()
                    .with_evaluation(FitnessCalc)
                    .with_selection(
                        RouletteWheelSelector::new(SELECTION_RATIO, NUM_INDIVIDUALS_PER_PARENTS)
                            .unwrap(),
                    )
                    .with_crossover(UniformCrossBreeder::new())
                    .with_mutation(
                        BreederValueMutator::new(
                            MUTATION_RATE,
                            Pos { x: 0, y: 1 },
                            3,
                            Pos { x: 0, y: 0 },
                            Pos {
                                x: NUM_ROWS,
                                y: NUM_COLS,
                            },
                        )
                        .unwrap(),
                    )
                    .with_reinsertion(
                        ElitistReinserter::new(FitnessCalc, false, REINSERTION_RATIO).unwrap(),
                    )
                    .with_initial_population(initial_population)
                    .build(),
            )
            .until(or(
                FitnessLimit::new(*FitnessCalc.fitness_bounds().end()),
                GenerationLimit::new(GENERATION_LIMIT),
            ))
            .build_with_seed(seed);

            let result = queens_sim.run();
            assert!(result.is_ok());
        })
    });
}

criterion_group!(benches, bench_full_genetic_algorithm);
criterion_main!(benches);
