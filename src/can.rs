//! Traits implemented by contexts that can something.

use internal::{
    Color,
    Rectangle,
    SourceRectangle,
};

/// Implemented by contexts that can color.
///
/// The context can color to type `T` by adding value `U`.
pub trait CanColor {
    /// Create a new context with color.
    fn color(&self, value: Color) -> Self;
}

/// Implemented by contexts that can set rectangle information.
pub trait CanRectangle {
    /// Create new context with rectangle information.
    fn rectangle(&self, value: Rectangle) -> Self;
}

/// Implemented by contexts that can set source rectangle information.
pub trait CanSourceRectangle {
    /// Create new context with source rectangle information.
    fn source_rectangle(&self, value: SourceRectangle) -> Self;
}

