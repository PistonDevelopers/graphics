
use {
    AddBevelBorder,
    AddRoundBorder,
    AddSquareBorder,
    BackEnd,
    BevelBorderLineColorContext,
    Borrowed,
    Clear,
    Color,
    Field,
    Line,
    Matrix2d,
    RoundBorderLineColorContext,
    SquareBorderLineColorContext,
    Value,
};
use internal::{
    CanColor,
    CanTransform,
    CanViewTransform,
    HasColor,
    HasTransform,
    HasViewTransform,
};

/// A line context.
pub struct LineColorContext<'a> {
    /// Base/original transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current line.
    pub line: Field<'a, Line>,
    /// Current color.
    pub color: Field<'a, Color>,
}

impl<'a> Clone for LineColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> LineColorContext<'static> {
        LineColorContext {
            base: self.base.clone(),
            transform: self.transform.clone(),
            line: self.line.clone(),
            color: self.color.clone(),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for LineColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, LineColorContext<'a>, Matrix2d> for LineColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> LineColorContext<'a> {
        LineColorContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for LineColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, LineColorContext<'a>, Matrix2d> 
for LineColorContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> LineColorContext<'a> {
        LineColorContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for LineColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, LineColorContext<'a>, Color> for LineColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> LineColorContext<'a> {
        LineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value(value),
        }
    }
}

impl<'a> AddRoundBorder<'a, RoundBorderLineColorContext<'a>> for LineColorContext<'a> {
    #[inline(always)]
    fn round_border_radius(&'a self, radius: f64) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            round_border_radius: Value(radius),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> AddBevelBorder<'a, BevelBorderLineColorContext<'a>> for LineColorContext<'a> {
    #[inline(always)]
    fn bevel_border_radius(&'a self, radius: f64) -> BevelBorderLineColorContext<'a> {
        BevelBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            bevel_border_radius: Value(radius),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> AddSquareBorder<'a, SquareBorderLineColorContext<'a>> for LineColorContext<'a> {
    #[inline(always)]
    fn square_border_radius(&'a self, radius: f64) -> SquareBorderLineColorContext<'a> {
        SquareBorderLineColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            square_border_radius: Value(radius),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> Clear for LineColorContext<'a> {
    fn clear<B: BackEnd>(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let &Color(color) = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

