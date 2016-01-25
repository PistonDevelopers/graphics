use DrawState;
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
/// [An example back-end using raw OpenGL](https://github.com/PistonDevelopers/opengl_graphics)
///
/// By default, this design uses triangles as graphics primitives.
/// This is supported by all GPUs and easy to implement in shader languages.
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
    ///
    /// In generic code, this type is often unknown.
    /// This might lead to more boilerplate code:
    ///
    /// ```ignore
    /// fn draw_texture<G, T>(c: &Context, g: &mut G)
    ///     where G: Graphcis<Texture = T>, T: ImageSize { ... }
    /// ```
    ///
    /// Code written specifically for one back-end can be easier to write.
    /// Later, when the code is done, it can be refactored into generic code.
    type Texture: ImageSize;

    /// Clears background with a color.
    ///
    /// The color should replace the values in the buffer.
    fn clear_color(&mut self, color: types::Color);

    /// Clears stencil buffer with a value, usually 0.
    ///
    /// A stencil buffer contains values that are not visible on the screen.
    /// These values are used to test against the pixel to paint.
    ///
    /// If you are drawing a shape for clipping and forgot to clear the
    /// stencil buffer, then the clipping shape will carry over in next frame
    /// and cause artifacts.
    fn clear_stencil(&mut self, value: u8);

    /// Renders list of 2d triangles using a solid color.
    ///
    /// All vertices share the same color.
    ///
    /// The back-end calls the closure with a closure to receive vertices.
    /// First, the back-end sets up shaders and such to prepare.
    /// Then it calls the closure, which calls back with chunks of vertices.
    /// The number of vertices per chunk never exceeds
    /// `BACK_END_MAX_VERTEX_COUNT`.
    /// Vertex positions are encoded `[x0, y0, x1, y1, ...]`.
    fn tri_list<F>(&mut self, draw_state: &DrawState, color: &[f32; 4], f: F)
        where F: FnMut(&mut FnMut(&[f32]));

    /// Renders list of 2d triangles using a color and a texture.
    ///
    /// All vertices share the same color.
    ///
    /// Tip: For objects of different colors, use grayscale textures.
    /// The texture color gets multiplied with the color.
    ///
    /// A texture coordinate is assigned per vertex (from [0, 0] to [1, 1]).
    ///
    /// The back-end calls the closure with a closure to receive vertices.
    /// First, the back-end sets up shaders and such to prepare.
    /// Then it calls the closure, which calls back with chunks of vertices.
    /// The number of vertices per chunk never exceeds
    /// `BACK_END_MAX_VERTEX_COUNT`.
    /// Vertex positions are encoded `[x0, y0, x1, y1, ...]`.
    /// Texture coordinates are encoded `[u0, v0, u1, v1, ...]`.
    ///
    /// Chunks uses separate buffer for vertex positions and texture coordinates.
    /// Arguments are `|vertices: &[f32], texture_coords: &[f32]`.
    fn tri_list_uv<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        texture: &<Self as Graphics>::Texture,
        f: F
    ) where F: FnMut(&mut FnMut(&[f32], &[f32]));

    /// Draws a rectangle.
    ///
    /// Can be overriden in the back-end for higher performance.
    ///
    /// Instead of calling this directly, use `Rectangle::draw`.
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
    ///
    /// Can be overridden in the back-end for higher performance.
    ///
    /// Instead of calling this directly, use `Polygon::draw`.
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
    ///
    /// Can be overridden in the back-end for higher performance.
    ///
    /// Instead of calling this directly, use `Polygon::draw_tween_lerp`.
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
    ///
    /// Can be overridden in the back-end for higher performance.
    ///
    /// Instead of calling this directly, use `Image::draw`.
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
    ///
    /// Can be overridden in the back-end for higher performance.
    ///
    /// Instead of calling this directly, use `Ellipse::draw`.
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
    ///
    /// Can be overridden in the back-end for higher performance.
    ///
    /// Instead of calling this directly, use `Line::draw`.
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
    ///
    /// Can be overriden in the back-end for higher performance.
    ///
    /// Instead of calling this directly, use `CircleArc::draw`.
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
    ///
    /// Can be overriden in the back-end for higher performance.
    ///
    /// Instead of calling this directly, use `DeformGrid::draw_image`.
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
