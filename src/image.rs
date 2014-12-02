//! Draw image

use current::{ Modifier };
use internal;
use triangulation;
use BackEnd;
use Color;
use Context;
use ImageSize;
use Rect;
use SrcRect;

/// An image
#[deriving(Copy)]
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
    pub fn draw<B: BackEnd<I>, I: ImageSize>(
        &self, 
        texture: &I, 
        c: &Context, 
        back_end: &mut B
    ) {
        use internal::Scalar;

        let color = self.color.unwrap_or([1.0, ..4]);
        if color[3] == 0.0 { return; }
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
        back_end.enable_texture(texture);
        back_end.color(color);
        back_end.tri_list_uv(
            &triangulation::rect_tri_list_xy(c.transform, rectangle),
            &triangulation::rect_tri_list_uv(texture, source_rectangle)
        );
        back_end.disable_texture();
    }
}

impl Modifier<Image> for Color {
    fn modify(self, img: &mut Image) {
        let Color(val) = self;
        img.color = Some(val);
    }
}

impl Modifier<Image> for Rect {
    fn modify(self, img: &mut Image) {
        let Rect(val) = self;
        img.rectangle = Some(val);
    }
}

impl Modifier<Image> for SrcRect {
    fn modify(self, img: &mut Image) {
        let SrcRect(val) = self;
        img.source_rectangle = Some(val);
    }
}

#[cfg(test)]
mod test {
    use super::Image;
    use Color;
    use Rect;
    use SrcRect;
    use current::Set;

    #[test]
    fn test_image() {
        let _img = Image::new()
            .set(Color([1.0, ..4]))
            .set(Rect([0.0, 0.0, 100.0, 100.0]))
            .set(SrcRect([0, 0, 32, 32]));
    }
}

