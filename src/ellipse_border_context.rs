use {
    AddColor,
    Borrowed,
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
pub struct EllipseBorderContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle enclosing the ellipse.
    pub rect: Field<'a, Rectangle>,
    /// Current border.
    pub border: Field<'a, Radius>,
}

impl<'a> 
Clone 
for EllipseBorderContext<'a> {
    #[inline(always)]
    fn clone(&self) -> EllipseBorderContext<'static> {
        EllipseBorderContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            border: Value(*self.border.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for EllipseBorderContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, EllipseBorderContext<'a>, Matrix2d> 
for EllipseBorderContext<'a> {
    #[inline(always)]
    fn transform(
        &'a self, 
        value: Matrix2d
    ) -> EllipseBorderContext<'a> {
        EllipseBorderContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for EllipseBorderContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, EllipseBorderContext<'a>, Matrix2d> 
for EllipseBorderContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> EllipseBorderContext<'a> {
        EllipseBorderContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> 
AddColor<'a, EllipseBorderColorContext<'a>> 
for EllipseBorderContext<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> EllipseBorderColorContext<'a> {
        EllipseBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Value([r, g, b, a]),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> 
HasRectangle<'a, Rectangle> 
for EllipseBorderContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> 
CanRectangle<'a, EllipseBorderContext<'a>, Rectangle> 
for EllipseBorderContext<'a> {
    #[inline(always)]
    fn rectangle(
        &'a self, 
        rect: Rectangle
    ) -> EllipseBorderContext<'a> {
        EllipseBorderContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            border: Borrowed(self.border.get()),
        }
    }
}

