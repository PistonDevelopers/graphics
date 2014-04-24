
/// Implemented by all graphics back-ends.
/// This trait uses default methods to simplify implementation.
pub trait BackEnd {
    /// Returns true if feature is supported.
    #[inline(always)]
    fn supports_clear_rgba(&self) -> bool { false }

    /// Clears background with a color.
    fn clear_rgba(&mut self, _r: f64, _g: f64, _b: f64, _a: f64) {}

    /// Returns true if feature is supported.
    #[inline(always)]
    fn supports_tri_list_xy_rgba_f64(&self) -> bool { false }

    /// Renders list of 2d triangles with color assigned per vertex.
    fn tri_list_xy_rgba_f64(&mut self, _vertices: &[f64], _colors: &[f64]) {}

    /// Returns true if feature is supported.
    #[inline(always)]
    fn supports_tri_list_xy_rgba_f32(&self) -> bool { false }

    /// Renders list of 2d triangles with color assigned per vertex.
    fn tri_list_xy_rgba_f32(&mut self, _vertices: &[f32], _colors: &[f32]) {}
}
