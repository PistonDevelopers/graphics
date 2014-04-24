
/// Implemented by contexts who can add color.
pub trait AddColor<'a, T> {
    /// Add color with alpha channel.
    fn rgba(&'a self, r: f64, g: f64, b: f64, a: f64) -> T;

    /// Adds color with alpha channel set to 1.0.
    #[inline(always)]
    fn rgb(&'a self, r: f64, g: f64, b: f64) -> T {
        self.rgba(r, g, b, 1.0)
    }
}

