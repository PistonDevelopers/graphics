
use {
    AddBevel,
    AddColor,
    AddRound,
    BevelRectangleBorderContext,
    Borrowed,
    Field,
    RectangleBorderColorContext,
    RoundRectangleBorderContext,
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

/// A rectangle context.
pub struct RectangleBorderContext<'a> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
    /// Current border.
    pub border: Field<'a, Radius>,
}

impl<'a> Clone for RectangleBorderContext<'a> {
    #[inline(always)]
    fn clone(&self) -> RectangleBorderContext<'static> {
        RectangleBorderContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
            border: Value(*self.border.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for RectangleBorderContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RectangleBorderContext<'a>, Matrix2d> for RectangleBorderContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RectangleBorderContext<'a> {
        RectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for RectangleBorderContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, RectangleBorderContext<'a>, Matrix2d> 
for RectangleBorderContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> RectangleBorderContext<'a> {
        RectangleBorderContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for RectangleBorderContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, RectangleBorderContext<'a>, Rectangle> 
for RectangleBorderContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> RectangleBorderContext<'a> {
        RectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> AddColor<'a, RectangleBorderColorContext<'a>> for RectangleBorderContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RectangleBorderColorContext<'a> {
        RectangleBorderColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Borrowed(self.rect.get()),
            border: Borrowed(self.border.get()),
        }
    }
}

#[test]
fn test_rgba() {
    use {Context, AddRectangle};

    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 100.0);
    let e = d.rgba(1.0, 0.0, 0.0, 1.0);
    let color = e.color.get();
    assert_eq!(color[0], 1.0);
}

impl<'a> AddRound<'a, RoundRectangleBorderContext<'a>> for RectangleBorderContext<'a> {
    #[inline(always)]
    fn round(&'a self, radius: Radius) -> RoundRectangleBorderContext<'a> {
        RoundRectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Value(radius),
            border: Borrowed(self.border.get()),
        }
    }
}

impl<'a> AddBevel<'a, BevelRectangleBorderContext<'a>> for RectangleBorderContext<'a> {
    #[inline(always)]
    fn bevel(&'a self, radius: Radius) -> BevelRectangleBorderContext<'a> {
        BevelRectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Value(radius),
            border: Borrowed(self.border.get()),
        }
    }
}

