
use {
    AddBevelBorder,
    AddColor,
    AddRoundBorder,
    AddSquareBorder,
    BevelBorderLineContext,
    Borrowed,
    Color,
    Field,
    Line,
    LineColorContext,
    Matrix2d,
    RoundBorderLineContext,
    SquareBorderLineContext,
    Value,
};
use internal::{
    CanTransform,
    CanViewTransform,
    HasTransform,
    HasViewTransform,
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

impl<'a> Clone for LineContext<'a> {
    #[inline(always)]
    fn clone(&self) -> LineContext<'static> {
        LineContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            line: self.line.clone(),
        }
    }
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

impl<'a> HasViewTransform<'a, Matrix2d> for LineContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, LineContext<'a>, Matrix2d> 
for LineContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> LineContext<'a> {
        LineContext {
            base: Value(value),
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
            color: Value(Color([r, g, b, a])),
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


