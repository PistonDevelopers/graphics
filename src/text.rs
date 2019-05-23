//! Draw text

use types::{Color, FontSize};
use {color, Image, Graphics, Transformed, DrawState};
use character::CharacterCache;
use math::Matrix2d;

/// Renders text
#[derive(Copy, Clone)]
pub struct Text {
    /// The color
    pub color: Color,
    /// The font size
    pub font_size: FontSize,
    /// Whether or not the text's position should be rounded (to a signed distance field).
    pub round: bool,
}

impl Text {
    /// Creates a new text with black color
    pub fn new(font_size: FontSize) -> Text {
        Text {
            color: color::BLACK,
            font_size: font_size,
            round: false,
        }
    }

    /// Creates a new colored text
    pub fn new_color(color: Color, font_size: FontSize) -> Text {
        Text {
            color: color,
            font_size: font_size,
            round: false,
        }
    }

    /// A builder method indicating that the Text's position should be rounded upon drawing.
    pub fn round(mut self) -> Text {
        self.round = true;
        self
    }

    /// Draws text with a character cache
    pub fn draw<C, G>(&self,
                      text: &str,
                      cache: &mut C,
                      draw_state: &DrawState,
                      transform: Matrix2d,
                      g: &mut G)
                      -> Result<(), C::Error>
        where C: CharacterCache,
              G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        let mut image = Image::new_color(self.color);

        let mut x = 0.0;
        let mut y = 0.0;
        for ch in text.chars() {
            let character = cache.character(self.font_size, ch)?;
            let mut ch_x = x + character.left();
            let mut ch_y = y - character.top();
            if self.round {
                ch_x = ch_x.round();
                ch_y = ch_y.round();
            }
            image = image.src_rect([
                character.atlas_offset[0], character.atlas_offset[1],
                character.atlas_size[0], character.atlas_size[1]
            ]);
            image.draw(character.texture,
                       draw_state,
                       transform.trans(ch_x, ch_y),
                       g);
            x += character.advance_width();
            y += character.advance_height();
        }

        Ok(())
    }
}
