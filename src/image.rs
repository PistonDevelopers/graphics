use internal;
use triangulation;
use BackEnd;
use Context;
use ImageSize;

/// An image with source rectangle
pub struct Image<'a, I: 'a + ImageSize> {
    /// The texture to draw with
    pub texture: &'a I,
    /// The image source rectangle
    pub source_rectangle: internal::SourceRectangle,
}

impl<'a, I: ImageSize> Image<'a, I> {
    /// Draws the image.
    pub fn draw<B: BackEnd<I>>(&self, c: &Context, back_end: &mut B) {
        use internal::Scalar;

        let color: [f32, ..4] = [1.0, 1.0, 1.0, 1.0];
        let rect = [
            0.0,
            0.0,
            self.source_rectangle[2] as Scalar,
            self.source_rectangle[3] as Scalar
        ];
        // Complete transparency does not need to be rendered.
        if color[3] == 0.0 { return; }
        back_end.enable_texture(self.texture);
        back_end.color(color);
        back_end.tri_list_uv(
            &triangulation::rect_tri_list_xy(c.transform, rect),
            &triangulation::rect_tri_list_uv(self.texture, self.source_rectangle)
        );
        back_end.disable_texture();
    }
}

