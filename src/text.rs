//! Draw text

use color;
use internal;
use Image;
use Graphics;
use character::CharacterCache;
use Transformed;
use math::Matrix2d;
use DrawState;

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
    pub fn draw<C, G>(
        &self,
        text: &str,
        cache: &mut C,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G
    )
        where
            C: CharacterCache,
            G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        let image = Image::new_colored(self.color);
        let mut x = 0;
        let mut y = 0;
        for ch in text.chars() {
            let character = cache.character(self.font_size, ch);
            image.draw(&character.texture,
                draw_state,
                transform.trans(
                    x as f64 + character.left(),
                    y as f64 - character.top()
                ),
                g
            );
            x += character.width() as i32;
            y += character.height() as i32;
        }
    }
}
