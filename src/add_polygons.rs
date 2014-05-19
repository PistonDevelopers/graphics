use {
    Polygons,
};

/// Implemented by tweening contexts that can add polygons.
pub trait AddPolygons<'a, T> {
    /// Add polygons.
    fn polygons(&'a self, polygons: Polygons<'a>) -> T;
}

