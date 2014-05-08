
/// Implemented by contexts that can transform.
///
/// The context can transform to type `T` by adding value `U`.
pub trait CanTransform<'a, T, U> {
    /// Create a new context with transformation.
    fn transform(&'a self, value: U) -> T;
}

