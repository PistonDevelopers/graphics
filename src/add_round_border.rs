
/// Implemented by contexts that can add round border.
pub trait AddRoundBorder<'a, T> {
    /// Adds a round radius and a width.
    fn round_border_radius(&'a self, radius: f64) -> T;
}

