use {
    AddBorder,
    AddColor,
    EllipseBorderContext,
    EllipseColorContext,
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
    Rectangle,
};

/// An ellipse context.
pub struct EllipseContext {
    /// View transform.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle enclosing the ellipse.
    pub rect: Rectangle,
}

impl
Clone 
for EllipseContext {
    #[inline(always)]
    fn clone(&self) -> EllipseContext {
        EllipseContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
        }
    }
}

impl
HasTransform<Matrix2d> 
for EllipseContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<EllipseContext, Matrix2d> 
for EllipseContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> EllipseContext {
        EllipseContext {
            view: self.view,
            transform: value,
            rect: self.rect,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for EllipseContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<EllipseContext, Matrix2d> 
for EllipseContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> EllipseContext {
        EllipseContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
        }
    }
}

impl
AddColor<EllipseColorContext> 
for EllipseContext {
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> EllipseColorContext {
        EllipseColorContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            color: [r, g, b, a],
        }
    }
}

impl
HasRectangle<Rectangle> 
for EllipseContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<EllipseContext, Rectangle> 
for EllipseContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> EllipseContext {
        EllipseContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
        }
    }
}

impl
AddBorder<EllipseBorderContext> 
for EllipseContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> EllipseBorderContext {
        EllipseBorderContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            border: radius,
        }
    }
}

