use {
    AddColor,
    Borrowed,
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
pub struct EllipseContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle enclosing the ellipse.
    pub rect: Field<'a, Rectangle>,
}

impl<'a> Clone for EllipseContext<'a> {
    #[inline(always)]
    fn clone(&self) -> EllipseContext<'static> {
        EllipseContext {
            base: Value(*self.base.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for EllipseContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, EllipseContext<'a>, Matrix2d> for EllipseContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for EllipseContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, EllipseContext<'a>, Matrix2d> 
for EllipseContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> EllipseContext<'a> {
        EllipseContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> AddColor<'a, EllipseColorContext<'a>> for EllipseContext<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> EllipseColorContext<'a> {
        EllipseColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Value([r, g, b, a]),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for EllipseContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, EllipseContext<'a>, Rectangle> for EllipseContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> EllipseContext<'a> {
        EllipseContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
        }
    }
}

