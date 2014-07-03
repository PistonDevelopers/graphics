use {
    AddBorder,
    AddColor,
    RoundRectangleBorderContext,
    RoundRectangleColorContext,
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

/// A round rectangle context.
pub struct RoundRectangleContext {
    /// View transform.
    pub view: Matrix2d,
    /// Current transform.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current roundness radius.
    pub round_radius: Radius,
}

impl
Clone 
for RoundRectangleContext {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleContext {
        RoundRectangleContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
        }
    }
}

impl
HasTransform<Matrix2d> 
for RoundRectangleContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<RoundRectangleContext, Matrix2d> 
for RoundRectangleContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RoundRectangleContext {
        RoundRectangleContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            round_radius: self.round_radius,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RoundRectangleContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<RoundRectangleContext, Matrix2d> 
for RoundRectangleContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RoundRectangleContext {
        RoundRectangleContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
        }
    }
}

impl
HasRectangle<Rectangle> 
for RoundRectangleContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<RoundRectangleContext, Rectangle> 
for RoundRectangleContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> RoundRectangleContext {
        RoundRectangleContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            round_radius: self.round_radius,
        }
    }
}

impl
AddColor<RoundRectangleColorContext> 
for RoundRectangleContext {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RoundRectangleColorContext {
        RoundRectangleColorContext {
            view: self.view,
            transform: self.transform,
            color: [r, g, b, a],
            rect: self.rect,
            round_radius: self.round_radius,
        }
    }
}

impl
AddBorder<RoundRectangleBorderContext> 
for RoundRectangleContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            round_radius: self.round_radius,
            border: radius,
        }
    }
}

