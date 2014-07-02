
/// Implemented by contexts that can make a shape rounder.
pub trait AddRound<T> {
    /// Rounds the shape of the current context.
    fn round(&self, radius: f64) -> T;
}
