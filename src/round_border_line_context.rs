
use {
    AddColor,
    Borrowed,
    Field,
    Line,
    Matrix2d,
    RoundBorderLineColorContext,
    Value,
    View,
};
use vecmath::{
    identity,
};

/// A line context with round border information.
pub struct RoundBorderLineContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current round border.
    pub round_border_radius: Field<'a, f64>,
}

impl<'a> AddColor<'a, RoundBorderLineColorContext<'a>> for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value([r, g, b, a]),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a> View<'a> for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> RoundBorderLineContext<'a> {
        RoundBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            line: Borrowed(self.line.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> RoundBorderLineContext<'a> {
        RoundBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            line: Borrowed(self.line.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> RoundBorderLineContext<'a> {
        RoundBorderLineContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

