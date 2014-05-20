use internal::{
    PixelRectangle,
};

/// Represents an image.
///
/// Images are often packed together in sprite sheets.
/// For this reason it refers to a rectangle within a texture.
///
/// The texture is a unique identifier recognized by the back-end.
/// An image contains the size of a texture to be able to
/// compute normalized coordinates.
///
/// There is no garbage collection of textures,
/// this responsibility is given to the back-end.
pub struct Image {
    /// A unique identifier of the texture, recognizable by back-end.
    pub texture_id: uint,
    /// The pixel width of the texture.
    pub texture_width: u32,
    /// The pixel height of the texture.
    pub texture_height: u32,
    /// The source rectangle in the texture.
    pub source_rect: PixelRectangle,
}

impl Clone for Image {
    #[inline(always)]
    fn clone(&self) -> Image {
        Image {
            texture_id: self.texture_id,
            texture_width: self.texture_width,
            texture_height: self.texture_height,
            source_rect: self.source_rect,
        }
    }
}

