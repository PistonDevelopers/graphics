//! Draw image

use types::{ Color, Rect, SourceRectangle };
use triangulation;
use Graphics;
use ImageSize;
use DrawState;
use math::Matrix2d;

/// An image
#[derive(Copy, Clone)]
pub struct Image {
    /// The color
    pub color: Option<Color>,
    /// The rectangle to draw image inside
    pub rectangle: Option<Rect>,
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
    pub fn new_colored(color: Color) -> Image {
        Image {
            color: Some(color),
            source_rectangle: None,
            rectangle: None
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
    pub fn rect<T: Into<Rect>>(mut self, value: T) -> Self {
        self.rectangle = Some(value.into());
        self
    }

    /// Sets optional rectangle.
    pub fn maybe_rect<T: Into<Rect>>(mut self, value: Option<T>) -> Self {
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

    /// Draws the image.
    pub fn draw<G>(
        &self,
        texture: &<G as Graphics>::Texture,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where G: Graphics
    {
        use math::Scalar;

        let color = self.color.unwrap_or([1.0; 4]);
        let source_rectangle = self.source_rectangle.unwrap_or({
            let (w, h) = texture.get_size();
            [0, 0, w as i32, h as i32]
        });
        let rectangle = self.rectangle.unwrap_or([
            0.0,
            0.0,
            source_rectangle[2] as Scalar,
            source_rectangle[3] as Scalar
        ].into());
        g.tri_list_uv(
            draw_state,
            &color,
            texture,
            |f| f(
                &triangulation::rect_tri_list_xy(transform, rectangle),
                &triangulation::rect_tri_list_uv(texture, source_rectangle)
            )
        );
    }
}

#[cfg(test)]
mod test {
    use super::Image;

    #[test]
    fn test_image() {
        let _img = Image::new()
            .color([1.0; 4])
            .rect([0.0, 0.0, 100.0, 100.0])
            .src_rect([0, 0, 32, 32]);
    }
}
