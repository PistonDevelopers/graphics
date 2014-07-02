use internal::{
    Radius,
    Width,
};

/// Implemented by contexts that can add round border.
pub trait AddBevelBorder<T> {
    /// Adds a bevel border radius.
    fn bevel_border_radius(&self, radius: Radius) -> T;

    /// Adds a bevel border width.
    #[inline(always)]
    fn bevel_border_width(&self, width: Width) -> T {
        self.bevel_border_radius(0.5 * width)
    }
}

