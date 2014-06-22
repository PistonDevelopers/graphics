
use {
    AddBevel,
    AddBorder,
    AddColor,
    AddImage,
    AddRound,
    BevelRectangleContext,
    Borrowed,
    Field,
    ImageSize,
    ImageRectangleContext,
    RectangleBorderContext,
    RectangleColorContext,
    RoundRectangleContext,
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
pub struct RectangleContext<'a> {
    /// View transformation.
    pub view: Field<'a, Matrix2d>,
    /// Current transformation.
    pub transform: Field<'a, Matrix2d>,
    /// Current rectangle.
    pub rect: Field<'a, Rectangle>,
}

impl<'a> Clone for RectangleContext<'a> {
    #[inline(always)]
    fn clone(&self) -> RectangleContext<'static> {
        RectangleContext {
            view: Value(*self.view.get()),
            transform: Value(*self.transform.get()),
            rect: Value(*self.rect.get()),
        }
    }
}

impl<'a> HasTransform<'a, Matrix2d> for RectangleContext<'a> {
    #[inline(always)]
    fn get_transform(&'a self) -> &'a Matrix2d {
        self.transform.get()
    }
}

impl<'a> CanTransform<'a, RectangleContext<'a>, Matrix2d> for RectangleContext<'a> {
    #[inline(always)]
    fn transform(&'a self, value: Matrix2d) -> RectangleContext<'a> {
        RectangleContext {
            view: Borrowed(self.view.get()),
            transform: Value(value),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> HasViewTransform<'a, Matrix2d> for RectangleContext<'a> {
    #[inline(always)]
    fn get_view_transform(&'a self) -> &'a Matrix2d {
        self.view.get()
    }
}

impl<'a> CanViewTransform<'a, RectangleContext<'a>, Matrix2d> 
for RectangleContext<'a> {
    #[inline(always)]
    fn view_transform(&'a self, value: Matrix2d) -> RectangleContext<'a> {
        RectangleContext {
            view: Value(value),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
        }
    }
}

impl<'a> HasRectangle<'a, Rectangle> for RectangleContext<'a> {
    #[inline(always)]
    fn get_rectangle(&'a self) -> &'a Rectangle {
        self.rect.get()
    }
}

impl<'a> CanRectangle<'a, RectangleContext<'a>, Rectangle> for RectangleContext<'a> {
    #[inline(always)]
    fn rectangle(&'a self, rect: Rectangle) -> RectangleContext<'a> {
        RectangleContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Value(rect),
        }
    }
}

impl<'a> AddColor<'a, RectangleColorContext<'a>> for RectangleContext<'a> {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &'a self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RectangleColorContext<'a> {
        RectangleColorContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Borrowed(self.rect.get()),
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

impl<'a> AddRound<'a, RoundRectangleContext<'a>> for RectangleContext<'a> {
    #[inline(always)]
    fn round(&'a self, radius: Radius) -> RoundRectangleContext<'a> {
        RoundRectangleContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            round_radius: Value(radius),
        }
    }
}

impl<'a> AddBevel<'a, BevelRectangleContext<'a>> for RectangleContext<'a> {
    #[inline(always)]
    fn bevel(&'a self, radius: Radius) -> BevelRectangleContext<'a> {
        BevelRectangleContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            bevel_radius: Value(radius),
        }
    }
}

impl<'a, 'b, I: ImageSize> 
AddImage<'a, 'b, ImageRectangleContext<'a, 'b, I>, I> 
for RectangleContext<'a> {
    fn image(&'a self, image: &'b I) -> ImageRectangleContext<'a, 'b, I> {
        let (w, h) = image.get_size();
        ImageRectangleContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            image: Value(image),
            source_rect: Value([0, 0, w as i32, h as i32]),
        }
    }
}

impl<'a> AddBorder<'a, RectangleBorderContext<'a>> for RectangleContext<'a> {
    #[inline(always)]
    fn border_radius(&'a self, radius: f64) -> RectangleBorderContext<'a> {
        RectangleBorderContext {
            view: Borrowed(self.view.get()),
            transform: Borrowed(self.transform.get()),
            rect: Borrowed(self.rect.get()),
            border: Value(radius),
        }
    }
}

