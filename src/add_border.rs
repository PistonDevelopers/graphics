
/// Implemented by contexts that can add border.
///
/// This is used in cases where border style is implicit of the shape.
/// For example, `RectangleContext` gives a `RectangleBorderContext `.
pub trait AddBorder<'a, T> {
    /// Adds a border radius.
    fn border_radius(&'a self, radius: f64) -> T;

    /// Adds a border width.
    #[inline(always)]
    fn border_width(&'a self, width: f64) -> T {
        self.border_radius(0.5 * width)
    }
}

