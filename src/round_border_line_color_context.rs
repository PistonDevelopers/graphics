use {
    Color,
    Field,
    Line,
    Matrix2d,
    RoundBorder,
};

/// A line context with round border information.
pub struct RoundBorderLineColorContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current round border.
    pub round_border: Field<'a, RoundBorder>,
}

