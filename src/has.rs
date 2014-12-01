//! Traits implemented by contexts that has something.

use internal::{
    Color,
    Rectangle,
    SourceRectangle,
};

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

