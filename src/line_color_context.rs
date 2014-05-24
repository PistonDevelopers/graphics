
use {
    AddBevelBorder,
    AddRoundBorder,
    AddSquareBorder,
    BackEnd,
    BevelBorderLineColorContext,
    Borrowed,
    Clear,
    Field,
    RoundBorderLineColorContext,
    SquareBorderLineColorContext,
    Value,
};
use internal::{
    CanColor,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasTransform,
    HasViewTransform,
    Line,
    Matrix2d,
    Radius,
};

/// A line context.
pub struct LineColorContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
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
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            line: Value(*self.line.get()),
            color: Value(*self.color.get()),
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
            view: Borrowed(self.view.get()),
            transform: Value(value),
            line: Borrowed(self.line.get()),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for LineColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, LineColorContext<'a>, Matrix2d> 
for LineColorContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> LineColorContext<'a> {
        LineColorContext {
            view: Value(value),
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
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            color: Value(value),
        }
    }
}

impl<'a> AddRoundBorder<'a, RoundBorderLineColorContext<'a>> for LineColorContext<'a> {
    #[inline(always)]
    fn round_border_radius(&'a self, radius: Radius) -> RoundBorderLineColorContext<'a> {
        RoundBorderLineColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            round_border_radius: Value(radius),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> AddBevelBorder<'a, BevelBorderLineColorContext<'a>> for LineColorContext<'a> {
    #[inline(always)]
    fn bevel_border_radius(&'a self, radius: Radius) -> BevelBorderLineColorContext<'a> {
        BevelBorderLineColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            line: Borrowed(self.line.get()),
            bevel_border_radius: Value(radius),
            color: Borrowed(self.color.get()),
        }
    }
}

impl<'a> AddSquareBorder<'a, SquareBorderLineColorContext<'a>> for LineColorContext<'a> {
    #[inline(always)]
    fn square_border_radius(&'a self, radius: Radius) -> SquareBorderLineColorContext<'a> {
        SquareBorderLineColorContext {
            view: Borrowed(self.view.get()),
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
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        }
    }
}

