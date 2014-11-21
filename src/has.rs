//! Traits implemented by contexts that has something.

use internal::{
    Color,
    Rectangle,
    SourceRectangle,
};
use vecmath::Matrix2d;

/// Implemented by contexts that uses type `U` as current color.
pub trait HasColor {
    /// Returns the current color.
    fn get_color(&self) -> Color;
}

/// Implemented by contexts that uses type `U` as current rectangle.
pub trait HasRectangle {
    /// Returns the current rectangle.
    fn get_rectangle(&self) -> Rectangle;
}

/// Implemented by contexts that uses type `U` as current source rectangle.
pub trait HasSourceRectangle {
    /// Returns the current source rectangle.
    fn get_source_rectangle(&self) -> SourceRectangle;
}

/// Implemented by contexts that uses type `U` as current transform.
///
/// This helps to remove redundant code.
pub trait HasTransform {
    /// Returns the current transform.
    fn get_transform(&self) -> Matrix2d;
}

/// Implemented by contexts that uses type `U` as current view transform.
///
/// This helps to remove redundant code.
pub trait HasViewTransform {
    /// Returns the current view transform.
    fn get_view_transform(&self) -> Matrix2d;
}

