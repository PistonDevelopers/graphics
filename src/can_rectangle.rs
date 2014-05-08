
/// Implemented by contexts that can set rectangle information.
pub trait CanRectangle<'a, T, U> {
    /// Create new context with recangle information.
    fn rect(&'a self, value: U) -> T;
}

