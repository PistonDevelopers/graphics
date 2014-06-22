use {
    BackEnd,
    ImageSize,
};

/// Implemented by contexts that can draws something using a back-end.
pub trait Draw<'a, B: BackEnd<I>, I: ImageSize> {
    /// Draw using back-end.
    fn draw(&'a self, back_end: &mut B);
}

