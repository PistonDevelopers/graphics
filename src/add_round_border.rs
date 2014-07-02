
/// Implemented by contexts that can add round border.
pub trait AddRoundBorder<T> {
    /// Adds a round border radius.
    fn round_border_radius(&self, radius: f64) -> T;

    /// Adds a round border width.
    #[inline(always)]
    fn round_border_width(&self, width: f64) -> T {
        self.round_border_radius(0.5 * width)
    }
}

