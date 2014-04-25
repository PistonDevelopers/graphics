use {Matrix2d, Field};

/// A polygon context.
pub struct PolygonContext<'a> {
    /// Base/origin transform.
    base: Field<'a, Matrix2d>,
    /// Current transform.
    transform: Field<'a, Matrix2d>,
    /// Current polygon.
    polygon: Field<'a, &'a [f64]>
}


