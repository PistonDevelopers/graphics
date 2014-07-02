
use {
    AddBevelBorder,
    AddRoundBorder,
    AddSquareBorder,
    BevelBorderLineColorContext,
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
pub struct LineColorContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current line.
    pub line: Field<Line>,
    /// Current color.
    pub color: Field<Color>,
}

impl
Clone 
for LineColorContext {
    #[inline(always)]
    fn clone(&self) -> LineColorContext {
        LineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for LineColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<LineColorContext, Matrix2d> 
for LineColorContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> LineColorContext {
        LineColorContext {
            view: Value(self.view.get()),
            transform: Value(value),
            line: Value(self.line.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for LineColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<LineColorContext, Matrix2d> 
for LineColorContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> LineColorContext {
        LineColorContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value(self.color.get()),
        }
    }
}

impl
HasColor<Color> 
for LineColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color.get()
    }
}

impl
CanColor<LineColorContext, Color> 
for LineColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> LineColorContext {
        LineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value(value),
        }
    }
}

impl
AddRoundBorder<RoundBorderLineColorContext> 
for LineColorContext {
    #[inline(always)]
    fn round_border_radius(
        &self, 
        radius: Radius
    ) -> RoundBorderLineColorContext {
        RoundBorderLineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            round_border_radius: Value(radius),
            color: Value(self.color.get()),
        }
    }
}

impl
AddBevelBorder<BevelBorderLineColorContext> 
for LineColorContext {
    #[inline(always)]
    fn bevel_border_radius(
        &self, 
        radius: Radius
    ) -> BevelBorderLineColorContext {
        BevelBorderLineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            bevel_border_radius: Value(radius),
            color: Value(self.color.get()),
        }
    }
}

impl
AddSquareBorder<SquareBorderLineColorContext> 
for LineColorContext {
    #[inline(always)]
    fn square_border_radius(
        &self, 
        radius: Radius
    ) -> SquareBorderLineColorContext {
        SquareBorderLineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            square_border_radius: Value(radius),
            color: Value(self.color.get()),
        }
    }
}

