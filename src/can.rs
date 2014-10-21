//! Traits implemented by contexts that can something.

/// Implemented by contexts that can color.
///
/// The context can color to type `T` by adding value `U`.
pub trait CanColor<T, U> {
    /// Create a new context with color.
    fn color(&self, value: U) -> T;
}

/// Implemented by contexts that can set rectangle information.
pub trait CanRectangle<T, U> {
    /// Create new context with rectangle information.
    fn rectangle(&self, value: U) -> T;
}

/// Implemented by contexts that can set source rectangle information.
pub trait CanSourceRectangle<T, U> {
    /// Create new context with source rectangle information.
    fn source_rectangle(&self, value: U) -> T;
}

/// Implemented by contexts that can transform.
///
/// The context can transform to type `T` by adding value `U`.
pub trait CanTransform<T, U> {
    /// Create a new context with transformation.
    fn transform(&self, value: U) -> T;
}

/// Implemented by contexts that can view transform.
///
/// The context can view transform to type `T` by adding value `U`.
pub trait CanViewTransform<T, U> {
    /// Create a new context with view transformation.
    fn view_transform(&self, value: U) -> T;
}
