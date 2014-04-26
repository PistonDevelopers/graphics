
/// Should be implemented by contexts that have rectangle information.
pub trait RelativeRectangle<'a> {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(&'a self, m: f64) -> Self;
}

