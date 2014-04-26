
/// Should be implemented by contexts that have rectangle information.
pub trait RelativeRectangle<'a> {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(&'a self, m: f64) -> Self;

    /// Moves to a relative rectangle using the current rectangle as tile.
    fn rel(&'a self, x: f64, y: f64) -> Self;
}

