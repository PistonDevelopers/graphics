
/// Implemented by contexts that can color.
///
/// The context can color to type `T` by adding value `U`.
pub trait CanColor<'a, T, U> {
    /// Create a new context with color.
    fn color(&'a self, value: U) -> T;
}

/// Implemented by contexts that can set rectangle information.
pub trait CanRectangle<'a, T, U> {
    /// Create new context with recangle information.
    fn rectangle(&'a self, value: U) -> T;
}

/// Implemented by contexts that can transform.
///
/// The context can transform to type `T` by adding value `U`.
pub trait CanTransform<'a, T, U> {
    /// Create a new context with transformation.
    fn transform(&'a self, value: U) -> T;
}

/// Implemented by contexts that can view transform.
///
/// The context can view transform to type `T` by adding value `U`.
pub trait CanViewTransform<'a, T, U> {
    /// Create a new context with view transformation.
    fn view_transform(&'a self, value: U) -> T;
}

/// Implemented by contexts that uses type `U` as current color.
pub trait HasColor<'a, U> {
    /// Returns the current color.
    fn get_color(&'a self) -> &'a U;
}

/// Implemented by contexts that uses type `U` as current rectangle.
pub trait HasRectangle<'a, U> {
    /// Returns the current rectangle.
    fn get_rectangle(&'a self) -> &'a U;
}

/// Implemented by contexts that uses type `U` as current transform.
///
/// This helps to remove redundant code.
pub trait HasTransform<'a, U> {
    /// Returns the current transform.
    fn get_transform(&'a self) -> &'a U;
}

/// Implemented by contexts that uses type `U` as current view transform.
///
/// This helps to remove redundant code.
pub trait HasViewTransform<'a, U> {
    /// Returns the current view transform.
    fn get_view_transform(&'a self) -> &'a U;
}

