use {
    AddColor,
    SquareBorderLineColorContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current line.
    pub line: Line,
    /// Current square border.
    pub square_border_radius: Radius,
}

impl
Clone 
for SquareBorderLineContext {
    #[inline(always)]
    fn clone(&self) -> SquareBorderLineContext {
        SquareBorderLineContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            square_border_radius: self.square_border_radius,
        }
    }
}

impl
HasTransform<Matrix2d> 
for SquareBorderLineContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            line: self.line,
            square_border_radius: self.square_border_radius,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for SquareBorderLineContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            line: self.line,
            square_border_radius: self.square_border_radius,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: [r, g, b, a],
            square_border_radius: self.square_border_radius,
        }
    }
}

