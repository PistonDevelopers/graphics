
/// Implemented by all graphics back-ends.
/// This trait uses default methods to simplify implementation.
///
/// ## Alpha blending
/// The default behavior for alpha blending is assumed to be turned off.
/// A back-end might choose to turn it on by default and ignore enable/disable.
/// The default behavior for not supporting alpha blending is ignoring enable/disable.
/// Alpha blending is assumed to be expensive and turned off when not needed.
/// For cases when alpha blending is explicitly not wanted there will be own methods,
/// in case the back-end needs to restore to its own default afterwards.
pub trait BackEnd {
    /// Returns true if feature is supported.
    #[inline(always)]
    fn supports_clear_rgba(&self) -> bool { false }

    /// Clears background with a color.
    fn clear_rgba(&mut self, _r: f32, _g: f32, _b: f32, _a: f32) {}

    /// Turns on alpha blending.
    fn enable_alpha_blend(&mut self) {}

    /// Turns off alpha blending.
    fn disable_alpha_blend(&mut self) {}

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
