
/// Implemented by contexts that uses type `U` as current rectangle.
pub trait HasRectangle<'a, U> {
    /// Returns the current rectangle.
    fn get_rectangle(&'a self) -> &'a U;
}

