
/// Implemented by contexts that uses type `U` as current color.
pub trait HasColor<'a, U> {
    /// Returns the current color.
    fn get_color(&'a self) -> &'a U;
}

