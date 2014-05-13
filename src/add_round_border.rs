
/// Implemented by contexts that can add round border.
pub trait AddRoundBorder<'a, T> {
    /// Adds a round radius and a width.
    fn round_border(&'a self, radius: f64, width: f64) -> T;
}

