use {BackEnd};

/// Implemented by contexts that can draws something using a back-end.
pub trait Draw<'a> {
    /// Draw using back-end.
    fn draw<B: BackEnd>(&'a self, back_end: &mut B);
}

