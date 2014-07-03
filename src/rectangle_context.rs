
use {
    AddBevel,
    AddBorder,
    AddColor,
    AddImage,
    AddRound,
    BevelRectangleContext,
    ImageSize,
    ImageRectangleContext,
    RectangleBorderContext,
    RectangleColorContext,
    RoundRectangleContext,
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
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
}

impl
Clone 
for RectangleContext {
    #[inline(always)]
    fn clone(&self) -> RectangleContext {
        RectangleContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
        }
    }
}

impl
HasTransform<Matrix2d> 
for RectangleContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<RectangleContext, Matrix2d> 
for RectangleContext {
    #[inline(always)]
    fn transform(&self, value: Matrix2d) -> RectangleContext {
        RectangleContext {
            view: self.view,
            transform: value,
            rect: self.rect,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RectangleContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
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
            view: value,
            transform: self.transform,
            rect: self.rect,
        }
    }
}

impl
HasRectangle<Rectangle> 
for RectangleContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<RectangleContext, Rectangle> 
for RectangleContext {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> RectangleContext {
        RectangleContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
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
            view: self.view,
            transform: self.transform,
            color: [r, g, b, a],
            rect: self.rect,
        }
    }
}

#[test]
fn test_rgba() {
    use {Context, AddRectangle};

    let c = Context::new();
    let d = c.rect(0.0, 0.0, 100.0, 100.0);
    let e = d.rgba(1.0, 0.0, 0.0, 1.0);
    let color = e.color;
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
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            round_radius: radius,
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
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: radius,
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
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            image: image,
            source_rect: [0, 0, w as i32, h as i32],
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
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            border: radius,
        }
    }
}

