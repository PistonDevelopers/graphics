use {
    AddBorder,
    AddColor,
    Borrowed,
    Field,
    BevelRectangleBorderContext,
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
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<'a, Radius>,
}

impl<'a> 
Clone 
for BevelRectangleContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleContext<'static> {
        BevelRectangleContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            bevel_radius: Value(*self.bevel_radius.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for BevelRectangleContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, BevelRectangleContext<'a>, Matrix2d> 
for BevelRectangleContext<'a> {
    #[inline(always)]
    fn transform(
        &'a self, 
        value: Matrix2d
    ) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for BevelRectangleContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, BevelRectangleContext<'a>, Matrix2d> 
for BevelRectangleContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> 
HasRectangle<'a, Rectangle> 
for BevelRectangleContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> 
CanRectangle<'a, BevelRectangleContext<'a>, Rectangle> 
for BevelRectangleContext<'a> {
    #[inline(always)]
    fn rectangle(
        &'a self, 
        rect: Rectangle
    ) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> 
AddColor<'a, BevelRectangleColorContext<'a>> 
for BevelRectangleContext<'a> {
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
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
        }
    }
}

impl<'a> 
AddBorder<'a, BevelRectangleBorderContext<'a>> 
for BevelRectangleContext<'a> {
    #[inline(always)]
    fn border_radius(
        &'a self, 
        radius: f64
    ) -> BevelRectangleBorderContext<'a> {
        BevelRectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            border: Value(radius),
        }
    }
}

