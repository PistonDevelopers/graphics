
use {
    AddBevel,
    AddColor,
    AddRound,
    BevelRectangleBorderContext,
    RectangleBorderColorContext,
    RoundRectangleBorderContext,
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
pub struct RectangleBorderContext {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current rectangle.
    pub rect: Rectangle,
    /// Current border.
    pub border: Radius,
}

impl
Clone 
for RectangleBorderContext {
    #[inline(always)]
    fn clone(&self) -> RectangleBorderContext {
        RectangleBorderContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            border: self.border,
        }
    }
}

impl
HasTransform<Matrix2d> 
for RectangleBorderContext {
    #[inline(always)]
    fn get_transform(&self) -> Matrix2d {
        self.transform
    }
}

impl
CanTransform<RectangleBorderContext, Matrix2d> 
for RectangleBorderContext {
    #[inline(always)]
    fn transform(
        &self, 
        value: Matrix2d
    ) -> RectangleBorderContext {
        RectangleBorderContext {
            view: self.view,
            transform: value,
            rect: self.rect,
            border: self.border,
        }
    }
}

impl
HasViewTransform<Matrix2d> 
for RectangleBorderContext {
    #[inline(always)]
    fn get_view_transform(&self) -> Matrix2d {
        self.view
    }
}

impl
CanViewTransform<RectangleBorderContext, Matrix2d> 
for RectangleBorderContext {
    #[inline(always)]
    fn view_transform(
        &self, 
        value: Matrix2d
    ) -> RectangleBorderContext {
        RectangleBorderContext {
            view: value,
            transform: self.transform,
            rect: self.rect,
            border: self.border,
        }
    }
}

impl
HasRectangle<Rectangle> 
for RectangleBorderContext {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        self.rect
    }
}

impl
CanRectangle<RectangleBorderContext, Rectangle> 
for RectangleBorderContext {
    #[inline(always)]
    fn rectangle(
        &self, 
        rect: Rectangle
    ) -> RectangleBorderContext {
        RectangleBorderContext {
            view: self.view,
            transform: self.transform,
            rect: rect,
            border: self.border,
        }
    }
}

impl
AddColor<RectangleBorderColorContext> 
for RectangleBorderContext {
    /// Creates a RectangleColorContext.
    #[inline(always)]
    fn rgba(
        &self, 
        r: ColorComponent, 
        g: ColorComponent, 
        b: ColorComponent, 
        a: ColorComponent
    ) -> RectangleBorderColorContext {
        RectangleBorderColorContext {
            view: self.view,
            transform: self.transform,
            color: [r, g, b, a],
            rect: self.rect,
            border: self.border,
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
AddRound<RoundRectangleBorderContext> 
for RectangleBorderContext {
    #[inline(always)]
    fn round(&self, radius: Radius) -> RoundRectangleBorderContext {
        RoundRectangleBorderContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            round_radius: radius,
            border: self.border,
        }
    }
}

impl
AddBevel<BevelRectangleBorderContext> 
for RectangleBorderContext {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: Radius
    ) -> BevelRectangleBorderContext {
        BevelRectangleBorderContext {
            view: self.view,
            transform: self.transform,
            rect: self.rect,
            bevel_radius: radius,
            border: self.border,
        }
    }
}

