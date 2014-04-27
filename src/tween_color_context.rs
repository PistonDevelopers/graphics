use {Field, Color, Matrix2d};

/// An animation inbetweening context with color.
pub struct TweenColorContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Animation inbetweening factor.
    pub tween_factor: Field<'a, f64>,
}

