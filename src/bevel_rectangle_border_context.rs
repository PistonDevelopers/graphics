use {
    AddColor,
    Borrowed,
    Field,
    BevelRectangleBorderColorContext,
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

/// A bevel rectangle border context.
pub struct BevelRectangleBorderContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<'a, Radius>,
    /// Current border.
    pub border: Field<'a, Radius>,
}

impl<'a> Clone for BevelRectangleBorderContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleBorderContext<'static> {
        BevelRectangleBorderContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            bevel_radius: Value(*self.bevel_radius.get()),
            border: Value(*self.border.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for BevelRectangleBorderContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, BevelRectangleBorderContext<'a>, Matrix2d> 
for BevelRectangleBorderContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> BevelRectangleBorderContext<'a> {
        BevelRectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for BevelRectangleBorderContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, BevelRectangleBorderContext<'a>, Matrix2d> 
for BevelRectangleBorderContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> BevelRectangleBorderContext<'a> {
        BevelRectangleBorderContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for BevelRectangleBorderContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, BevelRectangleBorderContext<'a>, Rectangle> 
for BevelRectangleBorderContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> BevelRectangleBorderContext<'a> {
        BevelRectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> AddColor<'a, BevelRectangleBorderColorContext<'a>> 
for BevelRectangleBorderContext<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> BevelRectangleBorderColorContext<'a> {
        BevelRectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

