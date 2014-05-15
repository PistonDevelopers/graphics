
/// Implemented by contexts that can add round border.
pub trait AddBevelBorder<'a, T> {
    /// Adds a bevel border radius.
    fn bevel_border_radius(&'a self, radius: f64) -> T;

    /// Adds a bevel border width.
    #[inline(always)]
    fn bevel_border_width(&'a self, width: f64) -> T {
        self.bevel_border_radius(0.5 * width)
    }
}

