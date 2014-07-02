use {
    AddBorder,
    AddColor,
    EllipseBorderContext,
    EllipseColorContext,
    Field,
    Value,
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
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle enclosing the ellipse.
    pub rect: Field<Rectangle>,
}

impl
Clone 
for EllipseContext {
    #[inline(always)]
    fn clone(&self) -> EllipseContext {
        EllipseContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for EllipseContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<EllipseContext, Matrix2d> 
for EllipseContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> EllipseContext {
        EllipseContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for EllipseContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl
HasRectangle<Rectangle> 
for EllipseContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            border: Value(radius),
        }
    }
}

