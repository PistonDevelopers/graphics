
/// Implemented by tweening contexts that can add polygons.
pub trait AddPolygons<'a, T> {
    /// Add polygons.
    fn polygons(&'a self, polygons: &'a [&'a [f64]]) -> T;
}

