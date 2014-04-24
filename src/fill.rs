use {BackEnd};

/// Fills a shape using a back-end.
pub trait Fill<'a> {
    /// Fill shape using back-end.
    fn fill<B: BackEnd>(&'a self, back_end: &mut B);
}
