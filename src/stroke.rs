use {
    BackEnd,
    ImageSize,
};

/// Strokes a shape using a back-end.
pub trait Stroke<'a, B: BackEnd<I>, I: ImageSize> {
    /// Stroke shape using back-end.
    fn stroke(&'a self, back_end: &mut B);
}

