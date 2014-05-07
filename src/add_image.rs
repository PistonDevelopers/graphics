
use {Image};

/// Implemented by contexts that can add image.
pub trait AddImage<'a, T> {
    /// Add image to context.
    fn image(&'a self, image: Image) -> T;
}

