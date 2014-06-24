use {
    Borrowed,
    Field,
    Value,
};
use triangulation::{
};
use internal::{
    CanColor,
    CanRectangle,
    CanTransform,
    CanViewTransform,
    Color,
    HasColor,
    HasRectangle,
    HasTransform,
    HasViewTransform,
    Matrix2d,
    Radius,
    Rectangle,
};

/// A bevel rectangle border color context.
pub struct BevelRectangleBorderColorContext<'a> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current bevel radius.
    pub bevel_radius: Field<'a, Radius>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current border.
    pub border: Field<'a, Radius>,
}

impl<'a> 
Clone 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> BevelRectangleBorderColorContext<'static> {
        BevelRectangleBorderColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            bevel_radius: Value(*self.bevel_radius.get()),
            color: Value(*self.color.get()),
            border: Value(*self.border.get()),
        }
    }
}

impl<'a> 
HasTransform<'a, Matrix2d> 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> 
CanTransform<'a, BevelRectangleBorderColorContext<'a>, Matrix2d> 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn transform(
        &'a self, 
        value: Matrix2d
    ) -> BevelRectangleBorderColorContext<'a> {
        BevelRectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> 
HasViewTransform<'a, Matrix2d> 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> 
CanViewTransform<'a, BevelRectangleBorderColorContext<'a>, Matrix2d> 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn view_transform(
        &'a self, 
        value: Matrix2d
    ) -> BevelRectangleBorderColorContext<'a> {
        BevelRectangleBorderColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> 
HasColor<'a, Color> 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> 
CanColor<'a, BevelRectangleBorderColorContext<'a>, Color> 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn color(
        &'a self, 
        value: Color
    ) -> BevelRectangleBorderColorContext<'a> {
        BevelRectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> 
HasRectangle<'a, Rectangle> 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> 
CanRectangle<'a, BevelRectangleBorderColorContext<'a>, Rectangle> 
for BevelRectangleBorderColorContext<'a> {
    #[inline(always)]
    fn rectangle(
        &'a self, 
        rect: Rectangle
    ) -> BevelRectangleBorderColorContext<'a> {
        BevelRectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            bevel_radius: Borrowed(self.bevel_radius.get()),
            color: Borrowed(self.color.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

