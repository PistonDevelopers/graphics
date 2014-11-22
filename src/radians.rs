//! Helper methods for working with radians (angular measurement).

/// Useful constants for radians.
pub trait Radians {
    /// Returns radians corresponding to 90 degrees.
    fn _90() -> Self;

    /// Returns radians corresponding to 180 degrees.
    fn _180() -> Self;

    /// Returns radians corresponding to 360 degrees.
    fn _360() -> Self;
}

impl Radians for f32 {
    #[inline(always)]
    fn _90() -> f32 {
        ::std::f32::consts::FRAC_PI_2
    }

    #[inline(always)]
    fn _180() -> f32 {
        ::std::f32::consts::PI
    }

    #[inline(always)]
    fn _360() -> f32 {
        ::std::f32::consts::PI_2
    }
}

impl Radians for f64 {
    #[inline(always)]
    fn _90() -> f64 {
        ::std::f64::consts::FRAC_PI_2
    }

    #[inline(always)]
    fn _180() -> f64 {
        ::std::f64::consts::PI
    }

    #[inline(always)]
    fn _360() -> f64 {
        ::std::f64::consts::PI_2
    }
}

