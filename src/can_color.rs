
/// Implemented by contexts that can color.
///
/// The context can color to type `T` by adding value `U`.
pub trait CanColor<'a, T, U> {
    /// Create a new context with color.
    fn color(&'a self, value: U) -> T;
}

