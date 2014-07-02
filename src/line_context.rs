
use {
    AddBevelBorder,
    AddColor,
    AddRoundBorder,
    AddSquareBorder,
    BevelBorderLineContext,
    Field,
    LineColorContext,
    RoundBorderLineContext,
    SquareBorderLineContext,
    Value,
};
use internal::{
    CanTransform,
    CanViewTransform,
    ColorComponent,
    HasTransform,
    HasViewTransform,
    Line,
    Matrix2d,
    Radius,
};

/// A line context.
pub struct LineContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current line.
    pub line: Field<Line>,
}

impl<'a> 
Clone for 
LineContext {
    #[inline(always)]
    fn clone(&self) -> LineContext {
        LineContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for LineContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<LineContext, Matrix2d> 
for LineContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> LineContext {
        LineContext {
            view: Value(self.view.get()),
            transform: Value(value),
            line: Value(self.line.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for LineContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<LineContext, Matrix2d> 
for LineContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> LineContext {
        LineContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
        }
    }
}

impl
AddColor<LineColorContext> 
for LineContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> LineColorContext {
        LineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl
AddRoundBorder<RoundBorderLineContext> 
for LineContext {
    #[inline(always)]
    fn round_border_radius(
        &self, 
        radius: Radius
    ) -> RoundBorderLineContext {
        RoundBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            round_border_radius: Value(radius),
        }
    }
}

impl
AddBevelBorder<BevelBorderLineContext> 
for LineContext {
    #[inline(always)]
    fn bevel_border_radius(
        &self, 
        radius: Radius
    ) -> BevelBorderLineContext {
        BevelBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            bevel_border_radius: Value(radius),
        }
    }
}

impl
AddSquareBorder<SquareBorderLineContext> 
for LineContext {
    #[inline(always)]
    fn square_border_radius(
        &self, 
        radius: Radius
    ) -> SquareBorderLineContext {
        SquareBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            square_border_radius: Value(radius),
        }
    }
}


