
use {
    AddColor,
    Borrowed,
    Color,
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
use internal::{
    CanTransform,
    HasTransform,
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

impl<'a> Clone for RoundBorderLineContext<'a> {
    #[inline(always)]   
    fn clone(&self) -> RoundBorderLineContext<'static> {
        RoundBorderLineContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            line: self.line.clone(),
            round_border_radius: self.round_border_radius.clone(),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RoundBorderLineContext<'a>, Matrix2d> for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RoundBorderLineContext<'a> {
        RoundBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            round_border_radius: Borrowed(self.round_border_radius.get()),
        }
    }
}

impl<'a> AddColor<'a, RoundBorderLineColorContext<'a>> for RoundBorderLineContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value(Color([r, g, b, a])),
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

