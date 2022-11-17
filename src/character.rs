//! A text character

use crate::{
    math,
    types::{FontSize, Scalar},
    ImageSize,
};

/// Holds rendered character data.
#[derive(Clone)]
pub struct Character<'a, T: ImageSize> {
    /// The offset of character.
    pub offset: [Scalar; 2],
    /// The advance size of character, including space.
    pub advance_size: [Scalar; 2],
    /// The offset of character within texture atlas.
    pub atlas_offset: [Scalar; 2],
    /// The size of character within texture atlas.
    pub atlas_size: [Scalar; 2],
    /// The texture of the character.
    pub texture: &'a T,
    /// if this is an "invalid character" character
    pub is_invalid: bool
}

impl<'a, T: ImageSize> Character<'a, T> {
    /// The left offset.
    pub fn left(&self) -> Scalar {
        self.offset[0]
    }

    /// The top offset.
    pub fn top(&self) -> Scalar {
        self.offset[1]
    }

    /// Gets width of character, including space to the next one.
    pub fn advance_width(&self) -> Scalar {
        self.advance_size[0]
    }

    /// Sets height of character, including space to the next one.
    pub fn advance_height(&self) -> Scalar {
        self.advance_size[1]
    }
}

/// Stores characters in a buffer and loads them by demand.
pub trait CharacterCache {
    /// The texture type associated with the character cache.
    type Texture: ImageSize;
    /// The error type associated with the character cache.
    type Error: core::fmt::Debug;

    /// Get reference to character.
    fn character(
        &mut self,
        font_size: FontSize,
        ch: char,
    ) -> Result<Character<'_, Self::Texture>, Self::Error>;

    /// Return the width for some given text.
    fn width(&mut self, size: FontSize, text: &str) -> Result<math::Scalar, Self::Error> {
        let mut width = 0.0;
        for ch in text.chars() {
            let character = self.character(size, ch)?;
            width += character.advance_width();
        }
        Ok(width)
    }
}
