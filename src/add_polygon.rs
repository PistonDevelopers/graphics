use {
    Polygon,
};

/// Implemented by contexts who can add polygon.
pub trait AddPolygon<'a, T> {
    /// Add polygon.
    fn polygon(&'a self, polygon: Polygon<'a>) -> T;
}

