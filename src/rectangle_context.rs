
use {
    AddBevel,
    AddBorder,
    AddColor,
    AddImage,
    AddRound,
    BevelRectangleContext,
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
pub struct RectangleContext {
    /// View transformation.
    pub view: Field<Matrix2d>,
    /// Current transformation.
    pub transform: Field<Matrix2d>,
    /// Current rectangle.
    pub rect: Field<Rectangle>,
}

impl
Clone 
for RectangleContext {
    #[inline(always)]
    fn clone(&self) -> RectangleContext {
        RectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
        }
    }
}

impl
HasTransform<Matrix2d> 
for RectangleContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform.get()
    }
}

impl
CanTransform<RectangleContext, Matrix2d> 
for RectangleContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> RectangleContext {
        RectangleContext {
            view: Value(self.view.get()),
            transform: Value(value),
            rect: Value(self.rect.get()),
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RectangleContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view.get()
    }
}

impl
CanViewTransform<RectangleContext, Matrix2d> 
for RectangleContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RectangleContext {
        RectangleContext {
            view: Value(value),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
        }
    }
}

impl
HasRectangle<Rectangle> 
for RectangleContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect.get()
    }
}

impl
CanRectangle<RectangleContext, Rectangle> 
for RectangleContext {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> RectangleContext {
        RectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(rect),
        }
    }
}

impl
AddColor<RectangleColorContext> 
for RectangleContext {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RectangleColorContext {
        RectangleColorContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            color: Value([r, g, b, a]),
            rect: Value(self.rect.get()),
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

impl
AddRound<RoundRectangleContext> 
for RectangleContext {
    #[inline(always)]
    fn round(
        &self, 
        radius: Radius
    ) -> RoundRectangleContext {
        RoundRectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            round_radius: Value(radius),
        }
    }
}

impl
AddBevel<BevelRectangleContext> 
for RectangleContext {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: Radius
    ) -> BevelRectangleContext {
        BevelRectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            bevel_radius: Value(radius),
        }
    }
}

impl<'b, I: ImageSize> 
AddImage<'b, ImageRectangleContext<'b, I>, I> 
for RectangleContext {
    fn image(
        &self, 
        image: &'b I
    ) -> ImageRectangleContext<'b, I> {
        let (w, h) = image.get_size();
        ImageRectangleContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            image: Value(image),
            source_rect: Value([0, 0, w as i32, h as i32]),
        }
    }
}

impl
AddBorder<RectangleBorderContext> 
for RectangleContext {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> RectangleBorderContext {
        RectangleBorderContext {
            view: Value(self.view.get()),
            transform: Value(self.transform.get()),
            rect: Value(self.rect.get()),
            border: Value(radius),
        }
    }
}

