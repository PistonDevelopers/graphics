use {Field, Matrix2d, Color};

/// A polygon color context.
pub struct PolygonColorContext<'a> {
    /// Base/origin transform.
    base: Field<'a, Matrix2d>,
    /// Current transform.
    transform: Field<'a, Matrix2d>,
    /// Current color.
    color: Field<'a, Color>,
    /// Current polygon.
    polygon: Field<'a, &'a [f64]>,
}


