
use ImageSize;

/// Implemented by contexts that can add image.
pub trait AddImage<'b, T, I: ImageSize> {
    /// Add image to context.
    fn image(&self, image: &'b I) -> T;
}

