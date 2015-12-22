use draw_state::DrawState;
use types::{ self, Matrix2d, Scalar };
use deform::DeformGrid;
use {
    CircleArc,
    Ellipse,
    Image,
    ImageSize,
    Line,
    Polygon,
    Rectangle,
};

/// Implemented by all graphics back-ends.
///
/// Default trait methods can be overridden for better performance or higher
/// quality.
///
/// When drawing, use this trait as generic constraint:
///
/// ```ignore
/// fn draw<G: Graphics>(c: &Context, g: &mut G) { ... }
/// ```
pub trait Graphics: Sized {
    /// The texture type associated with the back-end.
    type Texture: ImageSize;

    /// Clears background with a color.
    fn clear_color(&mut self, color: [f32; 4]);

    /// Clears stencil buffer with a value.
    fn clear_stencil(&mut self, value: u8);

    /// Renders list of 2d triangles.
    fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], f: F)
        where F: FnMut(&mut FnMut(&[f32]));

    /// Renders list of 2d triangles.
    ///
    /// A texture coordinate is assigned per vertex.
    /// The texture coordinates refers to the current texture.
    fn tri_list_uv<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        texture: &<Self as Graphics>::Texture,
        f: F
    ) where F: FnMut(&mut FnMut(&[f32], &[f32]));

    /// Draws a rectangle.
    #[inline(always)]
    fn rectangle<R: Into<types::Rectangle>>(
        &mut self,
        r: &Rectangle,
        rectangle: R,
        draw_state: &DrawState,
        transform: Matrix2d
    ) {
        r.draw_tri(rectangle, draw_state, transform, self);
    }

    /// Draws a polygon.
    #[inline(always)]
    fn polygon(
        &mut self,
        p: &Polygon,
        polygon: types::Polygon,
        draw_state: &DrawState,
        transform: Matrix2d
    ) {
        p.draw_tri(polygon, draw_state, transform, self);
    }

    /// Draws a tweened polygon using linear interpolation.
    #[inline(always)]
    fn polygon_tween_lerp(
        &mut self,
        p: &Polygon,
        polygons: types::Polygons,
        tween_factor: Scalar,
        draw_state: &DrawState,
        transform: Matrix2d
    ) {
        p.draw_tween_lerp_tri(polygons, tween_factor,
            draw_state, transform, self);
    }

    /// Draws image.
    #[inline(always)]
    fn image(
        &mut self,
        image: &Image,
        texture: &Self::Texture,
        draw_state: &DrawState,
        transform: Matrix2d
    ) {
        image.draw_tri(texture, draw_state, transform, self);
    }

    /// Draws ellipse.
    #[inline(always)]
    fn ellipse<R: Into<types::Rectangle>>(
        &mut self,
        e: &Ellipse,
        rectangle: R,
        draw_state: &DrawState,
        transform: Matrix2d
    ) {
        e.draw_tri(rectangle, draw_state, transform, self);
    }

    /// Draws line.
    #[inline(always)]
    fn line<L: Into<types::Line>>(
        &mut self,
        l: &Line,
        line: L,
        draw_state: &DrawState,
        transform: Matrix2d
    ) {
        l.draw_tri(line, draw_state, transform, self);
    }

    /// Draws circle arc.
    #[inline(always)]
    fn circle_arc<R: Into<types::Rectangle>>(
        &mut self,
        c: &CircleArc,
        rectangle: R,
        draw_state: &DrawState,
        transform: Matrix2d
    ) {
        c.draw_tri(rectangle, draw_state, transform, self);
    }

    /// Draws deformed image.
    #[inline(always)]
    fn deform_image(
        &mut self,
        d: &DeformGrid,
        texture: &Self::Texture,
        draw_state: &DrawState,
        transform: Matrix2d
    ) {
        d.draw_image_tri(texture, draw_state, transform, self);
    }
}
