use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::ControlFlow,
    time::Instant,
};

use thiserror::Error;

use crate::{
    algorithm::Algorithm,
    random::{Prng, Seed, get_rng, random_seed},
    simulation::{
        Simulation, SimulationBuilder, SimulationControlFlow, SimulationResult, SimulationState,
    },
    statistic::{ProcessingTime, TrackProcessingTime},
    termination::{StopFlag, Termination},
};

/// The `simulate` function creates a new `Simulator` for the given
/// `algorithm::Algorithm`.
pub fn simulate<A>(algorithm: A) -> SimulatorBuilderWithAlgorithm<A>
where
    A: Algorithm,
{
    SimulatorBuilderWithAlgorithm { algorithm }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatorBuilder<A, T>
where
    A: Algorithm,
    T: Termination<A>,
{
    algorithm: A,
    termination: T,
}

impl<A, T> SimulationBuilder<Simulator<A, T>, A> for SimulatorBuilder<A, T>
where
    A: Algorithm + TrackProcessingTime + Debug,
    <A as Algorithm>::Error: Eq + Hash + Display + Send + Sync,
    T: Termination<A>,
{
    fn build(self) -> Simulator<A, T> {
        self.build_with_seed(random_seed())
    }

    fn build_with_seed(self, seed: Seed) -> Simulator<A, T> {
        Simulator {
            algorithm: self.algorithm,
            termination: self.termination,
            run_mode: RunMode::NotRunning,
            rng: get_rng(seed),
            iteration: 0,
            processing_time: ProcessingTime::zero(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatorBuilderWithAlgorithm<A>
where
    A: Algorithm,
{
    algorithm: A,
}

impl<A> SimulatorBuilderWithAlgorithm<A>
where
    A: Algorithm,
{
    pub fn until<T>(self, termination: T) -> SimulatorBuilder<A, T>
    where
        T: Termination<A>,
    {
        SimulatorBuilder {
            algorithm: self.algorithm,
            termination,
        }
    }
}

/// The `RunMode` identifies whether the simulation is running and how it has
/// been started.
#[derive(Clone, Debug, PartialEq)]
enum RunMode {
    /// The simulation is running in loop mode. i.e. it was started by calling
    /// the `run` function.
    Loop(Instant),
    /// The simulation is running in step mode. i.e. it was started by calling
    /// the `step` function.
    Step(Instant),
    /// The simulation is not running.
    NotRunning,
}

#[derive(Error, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SimError<A>
where
    A: Algorithm + Debug,
    <A as Algorithm>::Error: Eq + Hash + Debug,
{
    #[error("algorithm error: {0}")]
    AlgorithmError(#[source] <A as Algorithm>::Error),
    #[error("simulation already running {0}")]
    SimulationAlreadyRunning(String),
}

#[derive(Clone, Debug)]
pub struct Simulator<A, T>
where
    A: Algorithm,
    T: Termination<A>,
{
    algorithm: A,
    termination: T,
    run_mode: RunMode,
    rng: Prng,
    iteration: u64,
    processing_time: ProcessingTime,
}

impl<A, T> Simulator<A, T>
where
    A: Algorithm + TrackProcessingTime + Debug,
    <A as Algorithm>::Error: Eq + Hash + Display + Send + Sync,
    T: Termination<A>,
{
    pub fn termination(&self) -> &T {
        &self.termination
    }

    /// Processes one iteration of the algorithm used in this simulation.
    fn process_one_iteration(
        &mut self,
        started_at: Instant,
    ) -> Result<SimulationState<A>, <Self as Simulation<A>>::Error> {
        let loop_started_at = Instant::now();

        self.iteration += 1;
        let result = self.algorithm.next(self.iteration, &mut self.rng);
        self.processing_time += self.algorithm.processing_time();

        let loop_duration = loop_started_at.elapsed();
        match result {
            Ok(result) => Ok(SimulationState {
                started_at,
                iteration: self.iteration,
                duration: loop_duration,
                processing_time: self.algorithm.processing_time(),
                result,
            }),
            Err(error) => Err(SimError::AlgorithmError(error)),
        }
    }
}

impl<A, T> Simulation<A> for Simulator<A, T>
where
    A: Algorithm + TrackProcessingTime + Debug,
    <A as Algorithm>::Error: Eq + Hash + Display + Send + Sync,
    T: Termination<A>,
{
    type Error = SimError<A>;

    fn run(&mut self) -> Result<SimulationControlFlow<A>, Self::Error> {
        let started_at = match self.run_mode.clone() {
            RunMode::Loop(t) => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "in loop mode since {:?}",
                    t
                )));
            }
            RunMode::Step(t) => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "in step mode since {:?}",
                    t
                )));
            }
            RunMode::NotRunning => {
                let now = Instant::now();
                self.run_mode = RunMode::Loop(now);
                now
            }
        };
        let result = loop {
            match self.process_one_iteration(started_at) {
                Ok(state) => {
                    // Stage 5: Be aware of the termination:
                    match self.termination.evaluate(&state) {
                        StopFlag::Continue => {}
                        StopFlag::StopNow(reason) => {
                            let processing_time = self.processing_time;
                            let duration = started_at.elapsed();
                            break Ok(ControlFlow::Break(SimulationResult {
                                state,
                                processing_time,
                                duration,
                                stop_reason: reason,
                            }));
                        }
                    }
                }
                Err(error) => {
                    break Err(error);
                }
            }
        };
        self.run_mode = RunMode::NotRunning;
        result
    }

    fn step(&mut self) -> Result<SimulationControlFlow<A>, Self::Error> {
        let started_at = match self.run_mode.clone() {
            RunMode::Loop(t) => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "in loop mode since {:?}",
                    t
                )));
            }
            RunMode::Step(t) => t,
            RunMode::NotRunning => {
                let now = Instant::now();
                self.run_mode = RunMode::Step(now);
                now
            }
        };
        self.process_one_iteration(started_at).map(|state| {
            match self.termination.evaluate(&state) {
                StopFlag::Continue => ControlFlow::Continue(state),
                StopFlag::StopNow(reason) => {
                    let processing_time = self.processing_time;
                    let duration = started_at.elapsed();
                    self.run_mode = RunMode::NotRunning;
                    ControlFlow::Break(SimulationResult {
                        state,
                        processing_time,
                        duration,
                        stop_reason: reason,
                    })
                }
            }
        })
    }

    fn stop(&mut self) -> Result<bool, Self::Error> {
        match self.run_mode {
            RunMode::Loop(_) | RunMode::Step(_) => {
                self.run_mode = RunMode::NotRunning;
                Ok(true)
            }
            RunMode::NotRunning => Ok(false),
        }
    }

    fn reset(&mut self) -> Result<bool, Self::Error> {
        match &self.run_mode {
            RunMode::Loop(t) => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "Simulation still running in loop mode since {:?}. Wait for the \
                     simulation to finish or stop it before resetting it.",
                    t
                )));
            }
            RunMode::Step(t) => {
                return Err(SimError::SimulationAlreadyRunning(format!(
                    "Simulation still running in step mode since {:?}. Wait for the \
                     simulation to finish or stop it before resetting it.",
                    t
                )));
            }
            RunMode::NotRunning => (),
        }
        self.run_mode = RunMode::NotRunning;
        self.processing_time = ProcessingTime::zero();
        self.iteration = 0;
        self.algorithm.reset().map_err(SimError::AlgorithmError)
    }
}
