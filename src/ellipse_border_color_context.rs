
use {
    BackEnd,
    Clear,
    Borrowed,
    Field,
    Image,
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

/// An ellipse border color context.
pub struct EllipseBorderColorContext<'a> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current color.
    pub color: Field<'a, Color>,
    /// Current border.
    pub border: Field<'a, Radius>,
}

impl<'a> Clone for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn clone(&self) -> EllipseBorderColorContext<'static> {
        EllipseBorderColorContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            color: Value(*self.color.get()),
            border: Value(*self.border.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, EllipseBorderColorContext<'a>, Matrix2d> for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> EllipseBorderColorContext<'a> {
        EllipseBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, EllipseBorderColorContext<'a>, Matrix2d> 
for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> EllipseBorderColorContext<'a> {
        EllipseBorderColorContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            color: Borrowed(self.color.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasColor<'a, Color> for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn get_color(&'a self) -> &'a Color {
        self.color.get()
    }
}

impl<'a> CanColor<'a, EllipseBorderColorContext<'a>, Color> for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn color(&'a self, value: Color) -> EllipseBorderColorContext<'a> {
        EllipseBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value(value),
            rect: Borrowed(self.rect.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, EllipseBorderColorContext<'a>, Rectangle> for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> EllipseBorderColorContext<'a> {
        EllipseBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            color: Borrowed(self.color.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a, B: BackEnd<I>, I: Image> Clear<B, I> for EllipseBorderColorContext<'a> {
    #[inline(always)]
    fn clear(&self, back_end: &mut B) {
        if back_end.supports_clear_rgba() {
            let color = self.color.get();
            back_end.clear_rgba(color[0], color[1], color[2], color[3]);
        } else {
            unimplemented!();
        }
    }
}

