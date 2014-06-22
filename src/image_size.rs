
/// Must be implemented by all images to be used with graphics back-end.
///
/// Rust-Graphics only needs to know the size.
pub trait ImageSize {
    /// Get the image size.
    fn get_size(&self) -> (u32, u32);
}

