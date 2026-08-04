pub mod simulator;

use std::{
    ops::ControlFlow,
    time::{Duration, Instant},
};

use crate::{
    algorithm::Algorithm, random::Seed, statistic::ProcessingTime, termination::StopReason,
};

/// A `Simulation` is the execution of an algorithm.
pub trait Simulation<A>
where
    A: Algorithm,
{
    type Error;

    /// Runs this simulation completely. The simulation ends when the
    /// termination criteria are met.
    ///
    /// Returns a [`std::ops::ControlFlow`]: `Break` with the
    /// [`SimulationResult`] once the simulation has finished, and `Continue`
    /// with the [`State`] of the last processed iteration while the simulation
    /// is still running.
    fn run(&mut self) -> Result<SimulationControlFlow<A>, Self::Error>;

    /// Makes one step in this simulation. One step in the simulation performs
    /// one time the complete loop of the genetic algorithm.
    ///
    /// Returns a [`std::ops::ControlFlow`]: `Break` with the
    /// [`SimulationResult`] once the simulation has finished, and `Continue`
    /// with the [`State`] of the last processed iteration while the simulation
    /// is still running.
    fn step(&mut self) -> Result<SimulationControlFlow<A>, Self::Error>;

    /// Stops the simulation after the current loop is finished.
    fn stop(&mut self) -> Result<bool, Self::Error>;

    /// Resets the simulation in order to be able to rerun it again. This
    /// method resets the simulation in its initial state, as if it's just
    /// newly created.
    fn reset(&mut self) -> Result<bool, Self::Error>;
}

/// The `SimulationBuilder` creates a new `Simulation` with given parameters
/// and options. It forms the initialization stage of the algorithm.
pub trait SimulationBuilder<S, A>
where
    S: Simulation<A>,
    A: Algorithm,
{
    /// Finally build the simulation.
    fn build(self) -> S;

    /// Finally build the simulation and initialize the RNG with the given seed.
    ///
    /// A simulation run can be repeated with the exact same sequence of
    /// iterations/generations, by providing the same seed as for a previous
    /// run.
    fn build_with_seed(self, seed: Seed) -> S;
}

/// The `State` struct holds the state of the `Simulation`.
#[derive(Clone, PartialEq, Debug)]
pub struct SimulationState<A>
where
    A: Algorithm,
{
    /// The point in time when this simulation started.
    pub started_at: Instant,
    /// The number of the iteration that this state represents. Iterations
    /// are counted from 1 and increased by 1 on each iteration of the
    /// simulation loop.
    pub iteration: u64,
    /// Duration of processing the current iteration. This is the time it
    /// took to process one iteration of the algorithm.
    pub duration: Duration,
    /// Accumulated time spent by each thread in case of parallel processing.
    /// In case of sequential processing this time is nearly the same as the
    /// `duration` value. In case of parallel processing this time is usually
    /// a multitude of the `duration`.
    pub processing_time: ProcessingTime,
    /// The result of this iteration.
    pub result: <A as Algorithm>::Output,
}

/// The result of a finished `Simulation`.
#[derive(Clone, PartialEq, Debug)]
pub struct SimulationResult<A>
where
    A: Algorithm,
{
    /// The `State` of the last processed generation.
    pub state: SimulationState<A>,
    /// The total processing time of the simulation.
    pub processing_time: ProcessingTime,
    /// The total duration of the simulation.
    pub duration: Duration,
    /// The `StopReason` is the matching criteria why the simulation stopped.
    pub stop_reason: StopReason,
}

pub type SimulationControlFlow<A> = ControlFlow<SimulationResult<A>, SimulationState<A>>;
