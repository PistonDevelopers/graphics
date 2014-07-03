use {
    BackEnd,
    ImageSize,
};

/// Implemented by contexts that can draws something using a back-end.
pub trait Draw<B: BackEnd<I>, I: ImageSize> {
    /// Draw using back-end.
    fn draw(&self, back_end: &mut B);
}

