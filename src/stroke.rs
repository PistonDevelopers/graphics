use {
    BackEnd,
    Image,
};

/// Strokes a shape using a back-end.
pub trait Stroke<'a, B: BackEnd<I>, I: Image> {
    /// Stroke shape using back-end.
    fn stroke(&'a self, back_end: &mut B);
}

