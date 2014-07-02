use {
    AddColor,
    Field,
    SquareBorderLineColorContext,
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

/// A line context with square border information.
pub struct SquareBorderLineContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current line.
    pub line: Field<Line>,
    /// Current square border.
    pub square_border_radius: Field<Radius>,
}

impl
Clone 
for SquareBorderLineContext {
    #[inline(always)]
    fn clone(&self) -> SquareBorderLineContext {
        SquareBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            square_border_radius: Value(self.square_border_radius.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for SquareBorderLineContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<SquareBorderLineContext, Matrix2d> 
for SquareBorderLineContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> SquareBorderLineContext {
        SquareBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(value),
            line: Value(self.line.get()),
            square_border_radius: Value(self.square_border_radius.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for SquareBorderLineContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<SquareBorderLineContext, Matrix2d> 
for SquareBorderLineContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> SquareBorderLineContext {
        SquareBorderLineContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            square_border_radius: Value(self.square_border_radius.get()),
        }
    }
}


impl
AddColor<SquareBorderLineColorContext> 
for SquareBorderLineContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> SquareBorderLineColorContext {
        SquareBorderLineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value([r, g, b, a]),
            square_border_radius: Value(self.square_border_radius.get()),
        }
    }
}

