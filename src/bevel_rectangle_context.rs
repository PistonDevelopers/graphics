use {
    AddColor,
    Borrowed,
    Field,
    BevelRectangleColorContext,
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

/// A bevel rectangle context.
pub struct BevelRectangleContext<'a> {
    /// Base/origin transform.
    pub base: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<'a, Radius>,
}

impl<'a> Clone for BevelRectangleContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleContext<'static> {
        BevelRectangleContext {
            base: Value(*self.base.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            bevel_radius: Value(*self.bevel_radius.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, BevelRectangleContext<'a>, Matrix2d> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.base.get()
    }
}

impl<'a> CanViewTransform<'a, BevelRectangleContext<'a>, Matrix2d> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, BevelRectangleContext<'a>, Rectangle> for BevelRectangleContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> AddColor<'a, BevelRectangleColorContext<'a>> for BevelRectangleContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> BevelRectangleColorContext<'a> {
        BevelRectangleColorContext {
            base: Borrowed(self.base.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

