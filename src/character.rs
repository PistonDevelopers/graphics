//! A text character

use types::{FontSize, Scalar};
use ImageSize;

/// Holds rendered character data.
#[derive(Clone)]
pub struct Character<'a, T: 'a + ImageSize> {
    /// The offset of character.
    pub offset: [Scalar; 2],
    /// The size of character, including space.
    pub size: [Scalar; 2],
    /// The texture of the character.
    pub texture: &'a T,
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
    pub fn width(&self) -> Scalar {
        self.size[0]
    }

    /// Sets height of character, including space to the next one.
    pub fn height(&self) -> Scalar {
        self.size[1]
    }
}

/// Stores characters in a buffer and loads them by demand.
pub trait CharacterCache {
    /// The textyre type associated with the character cache.
    type Texture: ImageSize;

    /// Get reference to character.
    fn character<'a>(&'a mut self,
                     font_size: FontSize,
                     ch: char)
                     -> Character<'a, <Self as CharacterCache>::Texture>;

    /// Return the width for some given text.
    fn width(&mut self, size: FontSize, text: &str) -> ::math::Scalar {
        text.chars().fold(0.0, |a, ch| {
            let character = self.character(size, ch);
            a + character.width()
        })
    }
}
