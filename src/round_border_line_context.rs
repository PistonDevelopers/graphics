
use {
    AddColor,
    RoundBorderLineColorContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current line.
    pub line: Line,
    /// Current round border.
    pub round_border_radius: Radius,
}

impl
Clone 
for RoundBorderLineContext {
    #[inline(always)]   
    fn clone(&self) -> RoundBorderLineContext {
        RoundBorderLineContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            round_border_radius: self.round_border_radius,
        }
    }
}

impl
HasTransform<Matrix2d> 
for RoundBorderLineContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
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
            view: self.view,
            transform: value,
            line: self.line,
            round_border_radius: self.round_border_radius,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RoundBorderLineContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            line: self.line,
            round_border_radius: self.round_border_radius,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: [r, g, b, a],
            round_border_radius: self.round_border_radius,
        }
    }
}

