
/// Implemented by contexts that can make a shape bevel.
pub trait AddBevel<'a, T> {
    /// Bevels the shape of the current context.
    fn bevel(&'a self, radius: f64) -> T;
}

