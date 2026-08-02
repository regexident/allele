//! This module provides implementations of the `genetic::Fitness` trait for
//! some primitive types, such as `i32`, `i64` et cetera.
//! This is because Rust does not allow programmers to implement
//! a foreign trait for a foreign type, which would stop you as a library user
//! from using primitive types as fitness values.
//!
//! Implemented types:
//!
//! * `i8`
//! * `i16`
//! * `i32`
//! * `i64`
//! * `isize`
//! * `u8`
//! * `u16`
//! * `u32`
//! * `u64`
//! * `usize`

use crate::genetic::{AsScalar, Fitness};

macro_rules! implement_fitness_for_signed_integer {
    ( $($t:ty),* ) => {
        $(
            impl Fitness for $t {
                fn zero() -> $t {
                    0
                }

                fn abs_diff(&self, other: &$t) -> $t {
                    (*self as i128).abs_diff(*other as i128) as $t
                }
            }

            impl AsScalar for $t {
                #[inline]
                fn as_scalar(&self) -> f64 {
                    *self as f64
                }
            }
        )*
    }
}

implement_fitness_for_signed_integer!(i8, i16, i32, i64, isize);

macro_rules! implement_fitness_for_unsigned_integer {
    ( $($t:ty),* ) => {
        $(
            impl Fitness for $t {
                fn zero() -> $t {
                    0
                }

                fn abs_diff(&self, other: &$t) -> $t {
                    (*self).abs_diff(*other)
                }
            }

            impl AsScalar for $t {
                #[inline]
                fn as_scalar(&self) -> f64 {
                    *self as f64
                }
            }
        )*
    }
}

implement_fitness_for_unsigned_integer!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests;
