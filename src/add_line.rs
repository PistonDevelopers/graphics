
/// Implemented by all contexts that can add rectangle.
pub trait AddLine<'a, T> {
    /// Adds a line.
    fn line(
        &'a self, 
        x1: f64, 
        y1: f64, 
        x2: f64, 
        y2: f64
    ) -> T;
}

