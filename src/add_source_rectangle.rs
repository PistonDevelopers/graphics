
/// Implemented by all contexts that can add source rectangle.
pub trait AddSourceRectangle<'a, T> {
    /// Adds a source rectangle with coordinates in the center.
    #[inline(always)]
    fn src_rect_centered(&'a self, center_x: u32, center_y: u32, radius_width: u32, radius_height: u32) -> T {
        self.src_rect(
            center_x - radius_width,
            center_y - radius_height,
            2 * radius_width,
            2 * radius_height
       )
    }

    /// Adds a square with coordinates of upper left corner.
    #[inline(always)]
    fn src_square(&'a self, x: u32, y: u32, w: u32) -> T {
        self.src_rect(x, y, w, w)
    }

    /// Adds a square with coordinates in the center.
    #[inline(always)]
    fn src_square_centered(&'a self, center_x: u32, center_y: u32, radius: u32) -> T {
        self.src_rect(center_x - radius,
                  center_y - radius,
                  2 * radius,
                  2 * radius)
    }
}

