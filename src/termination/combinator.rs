//! The `combinator` module provides combinators that logically combine two
//! `termination::Termination` conditions.
//!
//! Provided combinators are:
//! * `And` - stops the simulation when both of the combined termination
//!   conditions are met.
//! * `Or` - stops the simulation when any of the combined termination
//!   conditions is met.
//!
//! The functions `and` and `or` are provided for convenience and are
//! re-exported by the `termination` module.

use std::marker::PhantomData;

use crate::{
    algorithm::Algorithm,
    simulation::SimulationState,
    termination::{StopFlag, Termination},
};

/// Combines two `Termination` conditions with a logical AND. The simulation
/// stops only when both conditions are met.
pub fn and<T1, T2, A>(condition1: T1, condition2: T2) -> And<T1, T2, A>
where
    T1: Termination<A>,
    T2: Termination<A>,
    A: Algorithm,
{
    And::new(condition1, condition2)
}

/// A `Termination` condition that stops the simulation when both of the two
/// combined `Termination` conditions are met.
#[derive(Clone, Debug, PartialEq)]
pub struct And<T1, T2, A>
where
    T1: Termination<A>,
    T2: Termination<A>,
    A: Algorithm,
{
    condition1: T1,
    condition2: T2,
    _a: PhantomData<A>,
}

impl<T1, T2, A> And<T1, T2, A>
where
    T1: Termination<A>,
    T2: Termination<A>,
    A: Algorithm,
{
    /// Creates a new `And` combinator from two `Termination` conditions.
    pub fn new(condition1: T1, condition2: T2) -> Self {
        And {
            condition1,
            condition2,
            _a: PhantomData,
        }
    }

    /// Returns the first combined `Termination` condition.
    pub fn condition1(&self) -> &T1 {
        &self.condition1
    }

    /// Returns the second combined `Termination` condition.
    pub fn condition2(&self) -> &T2 {
        &self.condition2
    }
}

impl<T1, T2, A> Termination<A> for And<T1, T2, A>
where
    T1: Termination<A>,
    T2: Termination<A>,
    A: Algorithm,
{
    fn evaluate(&mut self, state: &SimulationState<A>) -> StopFlag {
        let mut reasons = Vec::with_capacity(2);
        match self.condition1.evaluate(state) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match self.condition2.evaluate(state) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match reasons.len() {
            0 | 1 => StopFlag::Continue,
            _ => StopFlag::StopNow(reasons.join(" AND ")),
        }
    }
}

/// Combines two `Termination` conditions with a logical OR. The simulation
/// stops when any of the two conditions is met.
pub fn or<T1, T2, A>(condition1: T1, condition2: T2) -> Or<T1, T2, A>
where
    T1: Termination<A>,
    T2: Termination<A>,
    A: Algorithm,
{
    Or::new(condition1, condition2)
}

/// A `Termination` condition that stops the simulation when any of the two
/// combined `Termination` conditions is met.
#[derive(Clone, Debug, PartialEq)]
pub struct Or<T1, T2, A>
where
    T1: Termination<A>,
    T2: Termination<A>,
    A: Algorithm,
{
    condition1: T1,
    condition2: T2,
    _a: PhantomData<A>,
}

impl<T1, T2, A> Or<T1, T2, A>
where
    T1: Termination<A>,
    T2: Termination<A>,
    A: Algorithm,
{
    /// Creates a new `Or` combinator from two `Termination` conditions.
    pub fn new(condition1: T1, condition2: T2) -> Self {
        Or {
            condition1,
            condition2,
            _a: PhantomData,
        }
    }

    /// Returns the first combined `Termination` condition.
    pub fn condition1(&self) -> &T1 {
        &self.condition1
    }

    /// Returns the second combined `Termination` condition.
    pub fn condition2(&self) -> &T2 {
        &self.condition2
    }
}

impl<T1, T2, A> Termination<A> for Or<T1, T2, A>
where
    T1: Termination<A>,
    T2: Termination<A>,
    A: Algorithm,
{
    fn evaluate(&mut self, state: &SimulationState<A>) -> StopFlag {
        let mut reasons = Vec::with_capacity(2);
        match self.condition1.evaluate(state) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match self.condition2.evaluate(state) {
            StopFlag::StopNow(reason) => reasons.push(reason),
            StopFlag::Continue => (),
        }
        match reasons.len() {
            0 => StopFlag::Continue,
            _ => StopFlag::StopNow(reasons[0].clone()),
        }
    }
}
