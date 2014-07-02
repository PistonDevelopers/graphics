
use {
    AddColor,
    Field,
    BevelBorderLineColorContext,
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

/// A line context with bevel border information.
pub struct BevelBorderLineContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transform.
    pub transform: Field<Matrix2d>,
    /// Current line.
    pub line: Field<Line>,
    /// Current bevel border.
    pub bevel_border_radius: Field<Radius>,
}

impl
Clone 
for BevelBorderLineContext {
    #[inline(always)]
    fn clone(&self) -> BevelBorderLineContext {
        BevelBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            bevel_border_radius: Value(self.bevel_border_radius.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelBorderLineContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<BevelBorderLineContext, Matrix2d> 
for BevelBorderLineContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> BevelBorderLineContext {
        BevelBorderLineContext {
            view: Value(self.view.get()),
            transform: Value(value),
            line: Value(self.line.get()),
            bevel_border_radius: Value(self.bevel_border_radius.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelBorderLineContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<BevelBorderLineContext, Matrix2d> 
for BevelBorderLineContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> BevelBorderLineContext {
        BevelBorderLineContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            bevel_border_radius: Value(self.bevel_border_radius.get()),
        }
    }
}

impl
AddColor<BevelBorderLineColorContext> 
for BevelBorderLineContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> BevelBorderLineColorContext {
        BevelBorderLineColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            line: Value(self.line.get()),
            color: Value([r, g, b, a]),
            bevel_border_radius: Value(self.bevel_border_radius.get()),
        }
    }
}

