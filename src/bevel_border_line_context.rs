
use {
    AddColor,
    BevelBorderLineColorContext,
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
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current line.
    pub line: Line,
    /// Current bevel border.
    pub bevel_border_radius: Radius,
}

impl
Clone 
for BevelBorderLineContext {
    #[inline(always)]
    fn clone(&self) -> BevelBorderLineContext {
        BevelBorderLineContext {
            view: self.view,
            transform: self.transform,
            line: self.line,
            bevel_border_radius: self.bevel_border_radius,
        }
    }
}

impl
HasTransform<Matrix2d> 
for BevelBorderLineContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<BevelBorderLineContext, Matrix2d> 
for BevelBorderLineContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> BevelBorderLineContext {
        BevelBorderLineContext {
            view: self.view,
            transform: value,
            line: self.line,
            bevel_border_radius: self.bevel_border_radius,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for BevelBorderLineContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            line: self.line,
            bevel_border_radius: self.bevel_border_radius,
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
            view: self.view,
            transform: self.transform,
            line: self.line,
            color: [r, g, b, a],
            bevel_border_radius: self.bevel_border_radius,
        }
    }
}

