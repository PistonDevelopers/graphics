
/// Implemented by contexts that can make a shape rounder.
pub trait AddRound<'a, T> {
    /// Rounds the shape of the current context.
    fn round(&'a self, radius: f64) -> T;
}
