//! Interpolation algorithms.
//!
//! Interpolation is used in animation, 
//! to describe smooth shapes and to make transitions.
//! Any object that fullfill certain mathematical 
//! properties can be interpolated.
//! A common technique is using one ore more 'numbers' 
//! controlling the mixture of states.
//! The choice of interpolation algorithm depends often 
//! on the circumstances where it used.

/// Performs linear interpolation.
/// A linear interpolation consists of two states 'a' and 'b'.
/// The 't' variable is a factor between 0 and 1 that 
/// gives weight to 'a' or 'b'.
/// When 't' is zero then 'a' has full weight.
/// When 't' is one then 'b' has full weight.
#[inline(always)]
pub fn lerp<T: Add<T, T> + Sub<T, T> + Mul<U, T>, U>(
    a: &T, 
    b: &T, 
    t: &U
) -> T {
    *a + (*b - *a) * *t
}

/// Performs linear interpolation on array of size 3.
#[inline(always)]
pub fn lerp_2<T: Add<T, T> + Sub<T, T> + Mul<U, T>, U>(
    a: &[T, ..2], 
    b: &[T, ..2], 
    t: &U
) -> [T, ..2] {
    [lerp(&a[0], &b[0], t),
    lerp(&a[1], &b[1], t)]
}

/// Performs linear interpolation on array of size 3.
#[inline(always)]
pub fn lerp_3<T: Add<T, T> + Sub<T, T> + Mul<U, T>, U>(
    a: &[T, ..3], 
    b: &[T, ..3], 
    t: &U
) -> [T, ..3] {
    [lerp(&a[0], &b[0], t),
    lerp(&a[1], &b[1], t),
    lerp(&a[2], &b[2], t)]
}

/// Performs linear interpolation on array of size 4.
#[inline(always)]
pub fn lerp_4<T: Add<T, T> + Sub<T, T> + Mul<U, T>, U>(
    a: &[T, ..4], 
    b: &[T, ..4], 
    t: &U
) -> [T, ..4] {
    [lerp(&a[0], &b[0], t),
    lerp(&a[1], &b[1], t),
    lerp(&a[2], &b[2], t),
    lerp(&a[3], &b[3], t)]
}

/// Performs linear interpolation on array of size 5.
#[inline(always)]
pub fn lerp_5<T: Add<T, T> + Sub<T, T> + Mul<U, T>, U>(
    a: &[T, ..5], 
    b: &[T, ..5], 
    t: &U
) -> [T, ..5] {
    [lerp(&a[0], &b[0], t),
    lerp(&a[1], &b[1], t),
    lerp(&a[2], &b[2], t),
    lerp(&a[3], &b[3], t),
    lerp(&a[4], &b[4], t)]
}

/// Performs quadratic beziér interpolation.
/// This is done by nesting linear interpolations.
/// For more information, see:
///
/// <a href="http://en.wikipedia.org/wiki/B%C3%A9zier_curve">Beziér Curve at Wikipedia</a>
#[inline(always)]
pub fn quad_bez<T: Add<T, T> + Sub<T, T> + Mul<U, T>, U>(
    x0: &T, 
    x1: &T, 
    x2: &T, 
    t: &U
) -> T {
    let x_0_1 = lerp(x0, x1, t);
    let x_1_2 = lerp(x1, x2, t);
    lerp(&x_0_1, &x_1_2, t)
}

/// Performs cubic beziér interpolation.
/// This is done by interpolation between two quadratic beziér.
/// For more information, see:
///
/// <a href="http://en.wikipedia.org/wiki/B%C3%A9zier_curve">Beziér Curve at Wikipedia</a>
#[inline(always)]
pub fn cub_bez<T: Add<T, T> + Sub<T, T> + Mul<U, T>, U>(
    x0: &T, 
    x1: &T, 
    x2: &T, 
    x3: &T, 
    t: &U
) -> T {
    let x_0_2 = quad_bez(x0, x1, x2, t);
    let x_1_3 = quad_bez(x1, x2, x3, t);
    lerp(&x_0_2, &x_1_3, t)
}

