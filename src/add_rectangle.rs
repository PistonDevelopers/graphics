
/// Implemented by all contexts that can add rectangle.
pub trait AddRectangle<'a, T> {
    /// Adds a rectangle.
    fn rect(&'a self, x: f64, y: f64, w: f64, h: f64) -> T;
}
