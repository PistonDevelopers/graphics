
/// Implemented by all contexts that can add ellipse.
pub trait AddEllipse<T> {
    /// Adds an ellipse.
    fn ellipse(&self, x: f64, y: f64, w: f64, h: f64) -> T;

    /// Adds an ellipse with coordinates in the center.
    #[inline(always)]
    fn ellipse_centered(
        &self, 
        center_x: f64, 
        center_y: f64, 
        radius_width: f64, 
        radius_height: f64
    ) -> T {
        self.ellipse(
            center_x - radius_width,
            center_y - radius_height,
            2.0 * radius_width,
            2.0 * radius_height
       )
    }

    /// Adds a circle.
    #[inline(always)]
    fn circle(
        &self, 
        center_x: f64, 
        center_y: f64, 
        radius: f64
    ) -> T {
        self.ellipse(center_x - radius,
                  center_y - radius,
                  2.0 * radius,
                  2.0 * radius)
    }
}

