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
pub trait BackEnd<I> {
    /// Clears background with a color.
    fn clear(&mut self, color: [f32, ..4]);

    /// Turns on alpha blending.
    fn enable_alpha_blend(&mut self);

    /// Turns off alpha blending.
    fn disable_alpha_blend(&mut self);

    /// Sets the texture.
    fn enable_texture(&mut self, _texture: &I) {}

    /// Disables texture.
    fn disable_texture(&mut self) {}

    /// Should return true if texture has alpha channel.
    ///
    /// This will enable alpha blending.
    /// Alpha blending might be enabled if color has alpha.
    #[inline(always)]
    fn has_texture_alpha(&self, _texture: &I) -> bool;

    /// Sets the current color.
    fn color(&mut self, color: [f32, ..4]);

    /// Renders list of 2d triangles.
    fn tri_list(&mut self, vertices: &[f32]);

    /// Renders list of 2d triangles.
    ///
    /// A texture coordinate is assigned per vertex.
    /// The texture coordinates refers to the current texture.
    fn tri_list_uv(&mut self, vertices: &[f32], texture_coords: &[f32]);
}

