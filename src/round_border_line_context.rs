
use {
    AddColor,
    Field,
    RoundBorderLineColorContext,
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

/// A line context with round border information.
pub struct RoundBorderLineContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current line.
    pub line: Field<Line>,
    /// Current round border.
    pub round_border_radius: Field<Radius>,
}

impl
Clone 
for RoundBorderLineContext {
    #[inline(always)]   
    fn clone(&self) -> RoundBorderLineContext {
        RoundBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            round_border_radius: Value(self.round_border_radius.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for RoundBorderLineContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<RoundBorderLineContext, Matrix2d> 
for RoundBorderLineContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RoundBorderLineContext {
        RoundBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(value),
            line: Value(self.line.get()),
            round_border_radius: Value(self.round_border_radius.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RoundBorderLineContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<RoundBorderLineContext, Matrix2d> 
for RoundBorderLineContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RoundBorderLineContext {
        RoundBorderLineContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            round_border_radius: Value(self.round_border_radius.get()),
        }
    }
}

impl
AddColor<RoundBorderLineColorContext> 
for RoundBorderLineContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RoundBorderLineColorContext {
        RoundBorderLineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value([r, g, b, a]),
            round_border_radius: Value(self.round_border_radius.get()),
        }
    }
}

