use {BackEnd};

/// Strokes a shape using a back-end.
pub trait Stroke<'a> {
    /// Stroke shape using back-end.
    fn stroke<B: BackEnd>(&'a self, back_end: &mut B);
}

