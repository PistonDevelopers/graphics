use {Field, Matrix2d, RoundRectangle};

/// A round rectangle context.
pub struct RoundRectangleContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current round rectangle.
    pub round_rect: Field<'a, RoundRectangle>,
}
