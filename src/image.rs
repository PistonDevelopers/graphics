//! Draw image

use internal;
use triangulation;
use Graphics;
use Color;
use Context;
use ImageSize;
use Rect;
use SrcRect;

/// An image
#[derive(Copy, Clone)]
pub struct Image {
    /// The color
    pub color: Option<internal::Color>,
    /// The rectangle to draw image inside
    pub rectangle: Option<internal::Rectangle>,
    /// The image source rectangle
    pub source_rectangle: Option<internal::SourceRectangle>,
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
    pub fn colored(color: internal::Color) -> Image {
        Image {
            color: Some(color),
            source_rectangle: None,
            rectangle: None
        }
    }

    /// Draws the image.
    pub fn draw<G>(
        &self,
        texture: &<G as Graphics>::Texture,
        c: &Context,
        g: &mut G
    )
        where G: Graphics
    {
        use internal::Scalar;

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
        ]);
        g.tri_list_uv(
            &c.draw_state,
            &color,
            texture,
            |f| f(
                &triangulation::rect_tri_list_xy(c.transform, rectangle),
                &triangulation::rect_tri_list_uv(texture, source_rectangle)
            )
        );
    }
}

quack! {
    img: Image[]
    get:
    set:
        fn (val: Color) [] { img.color = Some(val.0) }
        fn (val: Rect) [] { img.rectangle = Some(val.0) }
        fn (val: SrcRect) [] { img.source_rectangle = Some(val.0) }
    action:
}

#[cfg(test)]
mod test {
    use super::Image;
    use Color;
    use Rect;
    use SrcRect;
    use quack::Set;

    #[test]
    fn test_image() {
        let _img = Image::new()
            .set(Color([1.0; 4]))
            .set(Rect([0.0, 0.0, 100.0, 100.0]))
            .set(SrcRect([0, 0, 32, 32]));
    }
}
