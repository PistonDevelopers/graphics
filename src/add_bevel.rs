use internal::{
    Radius,
};

/// Implemented by contexts that can make a shape bevel.
pub trait AddBevel<T> {
    /// Bevels the shape of the current context.
    fn bevel(&self, radius: Radius) -> T;
}

