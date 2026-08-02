//! The `selection` module provides implementations of the
//! `operator::SelectionOp` genetic operator.
//!
//! The provided `operator::SelectionOp`s are organized in sub-modules
//! named after the utilized selection method:
//! * `truncation`
//! * `proportionate`
//! * `tournament`

pub mod proportionate;

pub mod tournament;

pub mod truncation;
