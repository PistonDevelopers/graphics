
use {Image};

/// Implemented by contexts that can add image.
pub trait AddImage<'a, 'b, T, I: Image> {
    /// Add image to context.
    fn image(&'a self, image: &'b I) -> T;
}

