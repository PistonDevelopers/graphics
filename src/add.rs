//! Add traits

use {
};
use internal::{
    Radius,
    Width,
};
use vecmath::{
    Scalar
};

/// Implemented by contexts that can add round border.
pub trait AddRoundBorder<T> {
    /// Adds a round border radius.
    fn round_border_radius(&self, radius: Radius) -> T;

    /// Adds a round border width.
    #[inline(always)]
    fn round_border_width(&self, width: Width) -> T {
        self.round_border_radius(0.5 * width)
    }
}


/// Implemented by contexts that can make a shape rounder.
pub trait AddRound<T> {
    /// Rounds the shape of the current context.
    fn round(&self, radius: Radius) -> T;
}

/// Implemented by contexts that can add square border.
pub trait AddSquareBorder<T> {
    /// Adds a square border radius.
    fn square_border_radius(&self, radius: Radius) -> T;

    /// Adds a square border width.
    #[inline(always)]
    fn square_border_width(&self, width: Width) -> T {
        self.square_border_radius(0.5 * width)
    }
}


/// Implemented by contexts who can give an animated inbetweening context.
pub trait AddTween<T> {
    /// Do linear interpolation.
    fn lerp(&self, tween_factor: Scalar) -> T;
}
