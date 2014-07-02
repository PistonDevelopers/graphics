use {
    AddColor,
    EllipseBorderColorContext,
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
    Radius,
    Rectangle,
};

/// An ellipse border context.
pub struct EllipseBorderContext {
    /// View transform.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle enclosing the ellipse.
    pub rect: Field<Rectangle>,
    /// Current border.
    pub border: Field<Radius>,
}

impl
Clone 
for EllipseBorderContext {
    #[inline(always)]
    fn clone(&self) -> EllipseBorderContext {
        EllipseBorderContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for EllipseBorderContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
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
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
            border: Value(self.border.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for EllipseBorderContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
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
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            border: Value(self.border.get()),
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            color: Value([r, g, b, a]),
            border: Value(self.border.get()),
        }
    }
}

impl 
HasRectangle<Rectangle> 
for EllipseBorderContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
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
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
            border: Value(self.border.get()),
        }
    }
}

