use {
    AddColor,
    Borrowed,
    Field,
    RoundRectangleBorderColorContext,
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

/// A round rectangle border context.
pub struct RoundRectangleBorderContext<'a> {
    /// View transform.
    pub view: Field<'a, Matrix2d>,
    /// Current transform.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current roundness radius.
    pub round_radius: Field<'a, Radius>,
    /// Curren tobrder.
    pub border: Field<'a, Radius>,
}

impl<'a> Clone for RoundRectangleBorderContext<'a> {
    #[inline(always)]
    fn clone(&self) -> RoundRectangleBorderContext<'static> {
        RoundRectangleBorderContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            round_radius: Value(*self.round_radius.get()),
            border: Value(*self.border.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> 
for RoundRectangleBorderContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RoundRectangleBorderContext<'a>, Matrix2d> 
for RoundRectangleBorderContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RoundRectangleBorderContext<'a> {
        RoundRectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> 
for RoundRectangleBorderContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, RoundRectangleBorderContext<'a>, Matrix2d> 
for RoundRectangleBorderContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> RoundRectangleBorderContext<'a> {
        RoundRectangleBorderContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> 
for RoundRectangleBorderContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, RoundRectangleBorderContext<'a>, Rectangle> 
for RoundRectangleBorderContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> RoundRectangleBorderContext<'a> {
        RoundRectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            round_radius: Borrowed(self.round_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> AddColor<'a, RoundRectangleBorderColorContext<'a>> for RoundRectangleBorderContext<'a> {
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RoundRectangleBorderColorContext<'a> {
        RoundRectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Borrowed(self.rect.get()),
            round_radius: Borrowed(self.round_radius.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

