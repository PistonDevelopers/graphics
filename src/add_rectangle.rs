
/// Implemented by all contexts that can add rectangle.
pub trait AddRectangle<'a, T> {
    /// Adds a rectangle.
    fn rect(&'a self, x: f64, y: f64, w: f64, h: f64) -> T;

    /// Adds a square with coordinates of upper left corner.
    #[inline(always)]
    fn square(&'a self, x: f64, y: f64, w: f64) -> T {
        self.rect(x, y, w, w)
    }
}
