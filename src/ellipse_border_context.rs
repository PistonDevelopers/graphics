use {
    AddColor,
    EllipseBorderColorContext,
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

/// An ellipse border context.
pub struct EllipseBorderContext {
    /// View transform.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle enclosing the ellipse.
    pub rect: Rectangle,
    /// Current border.
    pub border: Radius,
}

impl
Clone 
for EllipseBorderContext {
    #[inline(always)]
    fn clone(&self) -> EllipseBorderContext {
        EllipseBorderContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            border: self.border,
        }
    }
}

impl
HasTransform<Matrix2d> 
for EllipseBorderContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<EllipseBorderContext, Matrix2d> 
for EllipseBorderContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> EllipseBorderContext {
        EllipseBorderContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            border: self.border,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for EllipseBorderContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<EllipseBorderContext, Matrix2d> 
for EllipseBorderContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> EllipseBorderContext {
        EllipseBorderContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            border: self.border,
        }
    }
}

impl
AddColor<EllipseBorderColorContext> 
for EllipseBorderContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> EllipseBorderColorContext {
        EllipseBorderColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            color: [r, g, b, a],
            border: self.border,
        }
    }
}

impl 
HasRectangle<Rectangle> 
for EllipseBorderContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<EllipseBorderContext, Rectangle> 
for EllipseBorderContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> EllipseBorderContext {
        EllipseBorderContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            border: self.border,
        }
    }
}

