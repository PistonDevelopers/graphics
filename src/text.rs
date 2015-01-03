//! Draw text

use color;
use internal;
use Image;
use Context;
use ImageSize;
use BackEnd;
use character::CharacterCache;
use RelativeTransform;

/// Renders text
#[derive(Copy, Clone)]
pub struct Text {
    /// The color
    pub color: internal::Color,
    /// The font size
    pub font_size: internal::FontSize,
}

impl Text {
    /// Creates a new text with black color
    pub fn new(font_size: internal::FontSize) -> Text {
        Text {
            color: color::BLACK,
            font_size: font_size,
        }
    }

    /// Creates a new colored text
    pub fn colored(
        color: internal::Color, 
        font_size: internal::FontSize
    ) -> Text {
        Text {
            color: color,
            font_size: font_size,
        }
    }

    /// Draws text with a character cache
    pub fn draw<
        C: CharacterCache<I>,
        B: BackEnd<I>,
        I: ImageSize
    >(&self, text: &str, cache: &mut C, c: &Context, back_end: &mut B) {
        let image = Image::colored(self.color);
        let mut x = 0;
        let mut y = 0;
        for ch in text.chars() {
            let character = cache.character(self.font_size, ch);
            image.draw(&character.texture,
                &c.trans(
                    x as f64 + character.left(),
                    y as f64 - character.top()
                ),
                back_end
            );
            x += character.width() as i32;
            y += character.height() as i32;
        }
    }
}

