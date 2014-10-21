//! Traits implemented by contexts that has something.

/// Implemented by contexts that uses type `U` as current color.
pub trait HasColor<U> {
    /// Returns the current color.
    fn get_color(&self) -> U;
}

/// Implemented by contexts that uses type `U` as current rectangle.
pub trait HasRectangle<U> {
    /// Returns the current rectangle.
    fn get_rectangle(&self) -> U;
}

/// Implemented by contexts that uses type `U` as current source rectangle.
pub trait HasSourceRectangle<U> {
    /// Returns the current source rectangle.
    fn get_source_rectangle(&self) -> U;
}

/// Implemented by contexts that uses type `U` as current transform.
///
/// This helps to remove redundant code.
pub trait HasTransform<U> {
    /// Returns the current transform.
    fn get_transform(&self) -> U;
}

/// Implemented by contexts that uses type `U` as current view transform.
///
/// This helps to remove redundant code.
pub trait HasViewTransform<U> {
    /// Returns the current view transform.
    fn get_view_transform(&self) -> U;
}
