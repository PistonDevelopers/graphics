//! A text character

use types::FontSize;
use ImageSize;
use math::Scalar;
use types::{ Point, Size };

/// Holds rendered character data.
#[derive(Clone)]
pub struct Character<T: ImageSize> {
    /// The offset of character.
    pub offset: Point,
    /// The size of character, including space.
    pub size: Size,
    /// The texture of the character.
    pub texture: T,
}

impl<T: ImageSize> Character<T> {
    /// The left offset.
    pub fn left(&self) -> Scalar {
        self.offset.x
    }

    /// The top offset.
    pub fn top(&self) -> Scalar {
        self.offset.y
    }

    /// Gets width of character, including space to the next one.
    pub fn width(&self) -> Scalar {
        self.size.w
    }

    /// Sets height of character, including space to the next one.
    pub fn height(&self) -> Scalar {
        self.size.h
    }
}

/// Stores characters in a buffer and loads them by demand.
pub trait CharacterCache {
    /// The textyre type associated with the character cache.
    type Texture: ImageSize;

    /// Get reference to character.
    fn character(
        &mut self,
        font_size: FontSize,
        ch: char
    ) -> &Character<<Self as CharacterCache>::Texture>;
}
