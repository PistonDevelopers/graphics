
/// Implemented by contexts that uses type `U` as current transform.
///
/// This helps to remove redundant code.
pub trait HasTransform<'a, U> {
    /// Returns the current transform.
    fn get_transform(&'a self) -> &'a U;
}

