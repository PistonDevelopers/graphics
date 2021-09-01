//! Draw an image

use crate::{
    math::Matrix2d,
    triangulation,
    types::{Color, Rectangle, SourceRectangle},
    DrawState, Graphics, ImageSize,
};

/// An image
///
/// # Example
///
/// ```ignore
/// extern crate piston;
/// extern crate graphics;
/// extern crate glutin_window;
/// extern crate opengl_graphics;
///
/// use piston::window::WindowSettings;
/// use piston::event::*;
/// use glutin_window::GlutinWindow as Window;
/// use opengl_graphics::{GlGraphics, OpenGL, Texture};
/// use graphics::{Image, clear, default_draw_state};
/// use graphics::rectangle::square;
/// use std::path::Path;
///
/// fn main() {
///     let opengl  = OpenGL::_3_2;
///     let mut gl  = GlGraphics::new(opengl);
///     let window  = Window::new(
/// 			opengl,
/// 			WindowSettings::new(
/// 				"Example",
/// 				[600, 400]
/// 			).exit_on_esc(true));
///
///     // Create the image object and attach a square Rectangle object inside.
///     let image   = Image::new().rect(square(0.0, 0.0, 200.0));
///     // A texture to use with the image
///     let texture = Texture::from_path(Path::new("Example.png")).unwrap();
///
///     // Main loop
///     for e in window.events() {
/// 		if let Some(r) = e.render_args() {
/// 			gl.draw(r.viewport(), |c, gl| {
/// 				//Clear the screen
/// 				clear([0.0, 0.0, 0.0, 1.0], gl);
/// 				//Draw the image with the texture
/// 				image.draw(&texture, default_draw_state(), c.transform, gl);
/// 			});
/// 		}
/// 	}
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Image {
    /// The color
    pub color: Option<Color>,
    /// The rectangle to draw image inside
    pub rectangle: Option<Rectangle>,
    /// The image source rectangle
    pub source_rectangle: Option<SourceRectangle>,
}

impl Image {
    /// Creates a new image
    pub fn new() -> Image {
        Image {
            color: None,
            source_rectangle: None,
            rectangle: None,
        }
    }

    /// Creates a new colored image
    pub fn new_color(color: Color) -> Image {
        Image {
            color: Some(color),
            source_rectangle: None,
            rectangle: None,
        }
    }

    /// Sets color.
    pub fn color(mut self, value: Color) -> Self {
        self.color = Some(value);
        self
    }

    /// Sets optional color.
    pub fn maybe_color(mut self, value: Option<Color>) -> Self {
        self.color = value;
        self
    }

    /// Sets rectangle.
    pub fn rect<R: Into<Rectangle>>(mut self, value: R) -> Self {
        self.rectangle = Some(value.into());
        self
    }

    /// Sets optional rectangle.
    pub fn maybe_rect<R: Into<Rectangle>>(mut self, value: Option<R>) -> Self {
        self.rectangle = value.map(|v| v.into());
        self
    }

    /// Sets source rectangle.
    pub fn src_rect(mut self, value: SourceRectangle) -> Self {
        self.source_rectangle = Some(value);
        self
    }

    /// Sets optional source rectangle.
    pub fn maybe_src_rect(mut self, value: Option<SourceRectangle>) -> Self {
        self.source_rectangle = value;
        self
    }

    /// Draws image using default method.
    #[inline(always)]
    pub fn draw<G>(
        &self,
        texture: &<G as Graphics>::Texture,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        g.image(self, texture, draw_state, transform);
    }

    /// Draws image using triangulation.
    pub fn draw_tri<G>(
        &self,
        texture: &<G as Graphics>::Texture,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) where
        G: Graphics,
    {
        use crate::math::Scalar;

        let color = self.color.unwrap_or([1.0; 4]);
        let source_rectangle = self.source_rectangle.unwrap_or({
            let (w, h) = texture.get_size();
            [0.0, 0.0, w as Scalar, h as Scalar]
        });
        let rectangle = self.rectangle.unwrap_or([
            0.0,
            0.0,
            source_rectangle[2] as Scalar,
            source_rectangle[3] as Scalar,
        ]);
        g.tri_list_uv(draw_state, &color, texture, |f| {
            f(
                &triangulation::rect_tri_list_xy(transform, rectangle),
                &triangulation::rect_tri_list_uv(texture, source_rectangle),
            )
        });
    }
}

/// Draws many images.
pub fn draw_many<G>(
    rects: &[(Rectangle, SourceRectangle)],
    color: Color,
    texture: &<G as Graphics>::Texture,
    draw_state: &DrawState,
    transform: Matrix2d,
    g: &mut G,
) where
    G: Graphics,
{
    g.tri_list_uv(draw_state, &color, texture, |f| {
        for r in rects {
            f(
                &triangulation::rect_tri_list_xy(transform, r.0),
                &triangulation::rect_tri_list_uv(texture, r.1),
            )
        }
    });
}

#[cfg(test)]
mod test {
    use super::Image;

    #[test]
    fn test_image() {
        let _img = Image::new()
            .color([1.0; 4])
            .rect([0.0, 0.0, 100.0, 100.0])
            .src_rect([0.0, 0.0, 32.0, 32.0]);
    }
}
