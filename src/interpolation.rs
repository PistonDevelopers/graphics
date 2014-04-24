//! Interpolation algorithms.
//!
//! Interpolation is used in animation, to describe smooth shapes and to make transitions.
//! Any object that fullfill certain mathematical properties can be interpolated.
//! A common technique is using one ore more 'numbers' controlling the mixture of states.
//! The choice of interpolation algorithm depends often on the circumstances where it used.

/// Performs linear interpolation.
/// A linear interpolation consists of two states 'a' and 'b'.
/// The 't' variable is a factor between 0 and 1 that gives weight to 'a' or 'b'.
/// When 't' is zero then 'a' has full weight.
/// When 't' is one then 'b' has full weight.
#[inline(always)]
pub fn lerp<T: Add<T, T> + Sub<T, T> + Mul<T, T>>(a: T, b: T, t: T) -> T {
    a + (b - a) * t
}

