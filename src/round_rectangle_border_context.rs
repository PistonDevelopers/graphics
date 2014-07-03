use {
    AddColor,
    RoundRectangleBorderColorContext,
};
use internal::{
    CanRectangle,
    CanTransform,
    CanViewTransform,
    ColorComponent,
    HasRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
};

/// A round rectangle border context.
pub struct RoundRectangleBorderContext {
    /// View transform.
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current roundness radius.
    pub round_radius: Radius,
    /// Curren tobrder.
    pub border: Radius,
}

impl Clone for RoundRectangleBorderContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
            border: self.border,
        }
    }
}

impl HasTransform<Matrix2d> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl CanTransform<RoundRectangleBorderContext, Matrix2d> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            round_radius: self.round_radius,
            border: self.border,
        }
    }
}

impl HasViewTransform<Matrix2d> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl CanViewTransform<RoundRectangleBorderContext, Matrix2d> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn view_transform(&self, value: Matrix2d) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
            border: self.border,
        }
    }
}

impl HasRectangle<Rectangle> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl CanRectangle<RoundRectangleBorderContext, Rectangle> 
for RoundRectangleBorderContext {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            round_radius: self.round_radius,
            border: self.border,
        }
    }
}

impl AddColor<RoundRectangleBorderColorContext> for RoundRectangleBorderContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RoundRectangleBorderColorContext {
        RoundRectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            color: [r, g, b, a],
            rect: self.rect,
            round_radius: self.round_radius,
            border: self.border,
        }
    }
}

