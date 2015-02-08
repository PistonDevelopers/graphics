use ImageSize;

/// Implemented by all graphics back-ends.
pub trait BackEnd {
    type Texture: ImageSize;

    /// Clears background with a color.
    fn clear(&mut self, color: [f32; 4]);

    /// Sets the texture.
    fn enable_texture(&mut self, _texture: &<Self as BackEnd>::Texture);

    /// Disables texture.
    fn disable_texture(&mut self);

    /// Sets the current color.
    fn color(&mut self, color: [f32; 4]);

    /// Renders list of 2d triangles.
    fn tri_list(&mut self, vertices: &[f32]);

    /// Renders list of 2d triangles.
    ///
    /// A texture coordinate is assigned per vertex.
    /// The texture coordinates refers to the current texture.
    fn tri_list_uv(&mut self, vertices: &[f32], texture_coords: &[f32]);
}

