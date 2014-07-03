
use {
    AddBevelBorder,
    AddColor,
    AddRoundBorder,
    AddSquareBorder,
    BevelBorderLineContext,
    LineColorContext,
    RoundBorderLineContext,
    SquareBorderLineContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current line.
    pub line: Line,
}

impl<'a> 
Clone for 
LineContext {
    #[inline(always)]
    fn clone(&self) -> LineContext {
        LineContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
        }
    }
}

impl
HasTransform<Matrix2d> 
for LineContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<LineContext, Matrix2d> 
for LineContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> LineContext {
        LineContext {
            view: self.view,
            transform: value,
            line: self.line,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for LineContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            line: self.line,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: [r, g, b, a],
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            round_border_radius: radius,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            bevel_border_radius: radius,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            square_border_radius: radius,
        }
    }
}


