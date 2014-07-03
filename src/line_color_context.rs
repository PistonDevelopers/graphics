
use {
    AddBevelBorder,
    AddRoundBorder,
    AddSquareBorder,
    BevelBorderLineColorContext,
    RoundBorderLineColorContext,
    SquareBorderLineColorContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current line.
    pub line: Line,
    /// Current color.
    pub color: Color,
}

impl
Clone 
for LineColorContext {
    #[inline(always)]
    fn clone(&self) -> LineColorContext {
        LineColorContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: self.color,
        }
    }
}

impl
HasTransform<Matrix2d> 
for LineColorContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            line: self.line,
            color: self.color,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for LineColorContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            line: self.line,
            color: self.color,
        }
    }
}

impl
HasColor<Color> 
for LineColorContext {
    #[inline(always)]
    fn get_color(&self) -> Color {
        self.color
    }
}

impl
CanColor<LineColorContext, Color> 
for LineColorContext {
    #[inline(always)]
    fn color(&self, value: Color) -> LineColorContext {
        LineColorContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: value,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            round_border_radius: radius,
            color: self.color,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            bevel_border_radius: radius,
            color: self.color,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            square_border_radius: radius,
            color: self.color,
        }
    }
}

