use {
    BackEnd,
    ImageSize,
};

/// Implemented by contexts that can clear the background.
pub trait Clear<B: BackEnd<I>, I: ImageSize> {
    /// Clears the background.
    fn clear(&self, back_end: &mut B);
}

