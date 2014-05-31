
/// Must be implemented by all images.
///
/// Rust-Graphics only needs to know the size.
pub trait Image {
    /// Get the image size.
    fn get_size(&self) -> (u32, u32);
}

