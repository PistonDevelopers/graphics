
/// Implemented by all contexts that can add rectangle.
pub trait AddRectangle<'a, T> {
    /// Adds a rectangle.
    fn rect(&'a self, x: f64, y: f64, w: f64, h: f64) -> T;

    /// Adds a rectangle with coordinates in the center.
    #[inline(always)]
    fn rect_centered(
        &'a self, 
        center_x: f64, 
        center_y: f64, 
        radius_width: f64, 
        radius_height: f64
    ) -> T {
        self.rect(
            center_x - radius_width,
            center_y - radius_height,
            2.0 * radius_width,
            2.0 * radius_height
       )
    }

    /// Adds a square with coordinates of upper left corner.
    #[inline(always)]
    fn square(&'a self, x: f64, y: f64, w: f64) -> T {
        self.rect(x, y, w, w)
    }

    /// Adds a square with coordinates in the center.
    #[inline(always)]
    fn square_centered(
        &'a self, 
        center_x: f64, 
        center_y: f64, 
        radius: f64
    ) -> T {
        self.rect(center_x - radius,
                  center_y - radius,
                  2.0 * radius,
                  2.0 * radius)
    }
}
