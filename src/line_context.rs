
use {
    AddBevelBorder,
    AddColor,
    AddRoundBorder,
    AddSquareBorder,
    BevelBorderLineContext,
    Borrowed,
    Field,
    Line,
    LineColorContext,
    Matrix2d,
    RoundBorderLineContext,
    SquareBorderLineContext,
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

/// A line context.
pub struct LineContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
}

impl<'a> HasTransform<'a, Matrix2d> for LineContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, LineContext<'a>, Matrix2d> for LineContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
        }
    }
}

impl<'a> View<'a> for LineContext<'a> {
    #[inline(always)]
    fn view(&'a self) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.base.get()),
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn reset(&'a self) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.base.get()),
            transform: Value(identity()),
            line: Borrowed(self.line.get()),
        }
    }

    #[inline(always)]
    fn store_view(&'a self) -> LineContext<'a> {
        LineContext {
            base: Borrowed(self.transform.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
        }
    }
}

impl<'a> AddColor<'a, LineColorContext<'a>> for LineContext<'a> {
    #[inline(always)]
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> LineColorContext<'a> {
        LineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl<'a> AddRoundBorder<'a, RoundBorderLineContext<'a>> for LineContext<'a> {
    #[inline(always)]
    fn round_border_radius(&'a self, radius: f64) -> RoundBorderLineContext<'a> {
        RoundBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            round_border_radius: Value(radius),
        }
    }
}

impl<'a> AddBevelBorder<'a, BevelBorderLineContext<'a>> for LineContext<'a> {
    #[inline(always)]
    fn bevel_border_radius(&'a self, radius: f64) -> BevelBorderLineContext<'a> {
        BevelBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            bevel_border_radius: Value(radius),
        }
    }
}

impl<'a> AddSquareBorder<'a, SquareBorderLineContext<'a>> for LineContext<'a> {
    #[inline(always)]
    fn square_border_radius(&'a self, radius: f64) -> SquareBorderLineContext<'a> {
        SquareBorderLineContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            square_border_radius: Value(radius),
        }
    }
}


